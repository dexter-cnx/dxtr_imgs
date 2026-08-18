use std::fs;
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageDecoder, ImageReader, RgbaImage};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DevelopSettings {
    pub exposure: f32,
    pub temperature: f32,
    pub contrast: f32,
}

impl Default for DevelopSettings {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            temperature: 0.0,
            contrast: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelopedImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl DevelopedImage {
    pub fn expected_rgba_len(&self) -> Option<usize> {
        (self.width as usize)
            .checked_mul(self.height as usize)?
            .checked_mul(4)
    }

    pub fn has_valid_rgba_len(&self) -> bool {
        self.expected_rgba_len() == Some(self.data.len())
    }
}

#[derive(Debug, Error)]
pub enum ImageEngineError {
    #[error("failed to open image: {0}")]
    Open(#[source] std::io::Error),
    #[error("failed to read image: {0}")]
    Read(#[source] std::io::Error),
    #[error("unsupported image or RAW preview: {0}")]
    UnsupportedPreview(String),
    #[error("develop settings must be finite")]
    NonFiniteSettings,
}

/// Develops a raster image or the existing embedded-JPEG RAW preview into an
/// owned RGBA8 buffer for direct Rust callers.
///
/// This intentionally preserves the current Nixin behavior: RAW containers are
/// previewed from an embedded JPEG and are not sensor-demosaiced/debayered.
pub fn develop_image(
    path: impl AsRef<Path>,
    settings: DevelopSettings,
) -> Result<DevelopedImage, ImageEngineError> {
    let mut rgba = load_embedded_preview(path.as_ref())?.to_rgba8();
    apply_settings_to_rgba(&mut rgba, settings)?;
    Ok(from_rgba(rgba))
}

/// Native Rust convenience entry point using neutral develop settings.
pub fn develop_preview(path: impl AsRef<Path>) -> Result<DevelopedImage, ImageEngineError> {
    develop_image(path, DevelopSettings::default())
}

/// Loads a standard raster image and normalizes its orientation before it
/// enters the editing pipeline. If ordinary decoding fails, the source is
/// treated as a possible RAW container and scanned for its largest decodable
/// embedded JPEG preview.
pub fn load_embedded_preview(path: &Path) -> Result<DynamicImage, ImageEngineError> {
    match ImageReader::open(path) {
        Ok(reader) => {
            if let Ok(reader) = reader.with_guessed_format() {
                if let Ok(mut decoder) = reader.into_decoder() {
                    let orientation = decoder
                        .orientation()
                        .unwrap_or(image::metadata::Orientation::NoTransforms);
                    if let Ok(mut image) = DynamicImage::from_decoder(decoder) {
                        image.apply_orientation(orientation);
                        return Ok(image);
                    }
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(ImageEngineError::Open(error));
        }
        Err(_) => {}
    }

    let bytes = fs::read(path).map_err(ImageEngineError::Read)?;
    extract_best_embedded_jpeg(&bytes).map_err(ImageEngineError::UnsupportedPreview)
}

fn extract_best_embedded_jpeg(bytes: &[u8]) -> Result<DynamicImage, String> {
    let mut starts = Vec::new();
    let mut index = 0usize;

    while index + 1 < bytes.len() {
        if bytes[index] == 0xff && bytes[index + 1] == 0xd8 {
            starts.push(index);
        }
        index += 1;
    }

    if starts.is_empty() {
        return Err("no embedded JPEG start marker found".to_owned());
    }

    let mut best: Option<(u64, DynamicImage)> = None;
    for start in starts {
        let mut end = start + 2;
        while end + 1 < bytes.len() {
            if bytes[end] == 0xff && bytes[end + 1] == 0xd9 {
                let candidate = &bytes[start..=end + 1];
                if let Ok(image) =
                    image::load_from_memory_with_format(candidate, image::ImageFormat::Jpeg)
                {
                    let (width, height) = image.dimensions();
                    let area = u64::from(width) * u64::from(height);
                    if best
                        .as_ref()
                        .map(|(best_area, _)| area > *best_area)
                        .unwrap_or(true)
                    {
                        best = Some((area, image));
                    }
                    break;
                }
            }
            end += 1;
        }
    }

    best.map(|(_, image)| image)
        .ok_or_else(|| "embedded JPEG markers found, but none decoded".to_owned())
}

fn apply_settings_to_rgba(
    image: &mut RgbaImage,
    settings: DevelopSettings,
) -> Result<(), ImageEngineError> {
    if !settings.exposure.is_finite()
        || !settings.temperature.is_finite()
        || !settings.contrast.is_finite()
    {
        return Err(ImageEngineError::NonFiniteSettings);
    }

    let exposure_gain = 2.0_f32.powf(settings.exposure.clamp(-8.0, 8.0));
    let temperature = settings.temperature.clamp(-1.0, 1.0);
    let red_scale = 1.0 + 0.20 * temperature;
    let blue_scale = 1.0 - 0.20 * temperature;
    let contrast = settings.contrast.clamp(0.0, 4.0);

    for pixel in image.pixels_mut() {
        let mut rgb = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];
        rgb[0] *= exposure_gain * red_scale;
        rgb[1] *= exposure_gain;
        rgb[2] *= exposure_gain * blue_scale;

        for channel in &mut rgb {
            *channel = ((*channel - 128.0) * contrast + 128.0).clamp(0.0, 255.0);
        }

        pixel[0] = rgb[0].round() as u8;
        pixel[1] = rgb[1].round() as u8;
        pixel[2] = rgb[2].round() as u8;
    }

    Ok(())
}

fn from_rgba(image: RgbaImage) -> DevelopedImage {
    let (width, height) = image.dimensions();
    DevelopedImage {
        width,
        height,
        data: image.into_raw(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use image::codecs::jpeg::JpegEncoder;
    use image::{ExtendedColorType, ImageEncoder, Rgb, RgbImage, Rgba, RgbaImage};

    use super::*;

    fn temp_path(extension: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "dxtr-imgs-raw-engine-{}-{nonce}.{extension}",
            std::process::id()
        ))
    }

    fn encode_test_jpeg(source: &RgbImage) -> Vec<u8> {
        let mut jpeg = Vec::new();
        JpegEncoder::new_with_quality(&mut jpeg, 95)
            .write_image(
                source.as_raw(),
                source.width(),
                source.height(),
                ExtendedColorType::Rgb8,
            )
            .expect("test JPEG should encode");
        jpeg
    }

    #[test]
    fn raster_preview_returns_owned_rgba() {
        let path = temp_path("png");
        let mut source = RgbaImage::new(2, 1);
        source.put_pixel(0, 0, Rgba([10, 20, 30, 255]));
        source.put_pixel(1, 0, Rgba([40, 50, 60, 255]));
        source
            .save_with_format(&path, image::ImageFormat::Png)
            .expect("test PNG should save");

        let developed = develop_preview(&path).expect("PNG should decode");
        fs::remove_file(&path).expect("test file should be removable");

        assert_eq!((developed.width, developed.height), (2, 1));
        assert!(developed.has_valid_rgba_len());
        assert_eq!(&developed.data[0..4], &[10, 20, 30, 255]);
    }

    #[test]
    fn raw_container_falls_back_to_embedded_jpeg() {
        let path = temp_path("raw");
        let source = RgbImage::from_pixel(3, 2, Rgb([120, 90, 60]));
        let jpeg = encode_test_jpeg(&source);

        let mut container = b"synthetic-raw-prefix".to_vec();
        container.extend_from_slice(&jpeg);
        container.extend_from_slice(b"synthetic-raw-suffix");
        fs::write(&path, container).expect("synthetic RAW should save");

        let developed = develop_preview(&path).expect("embedded JPEG should decode");
        fs::remove_file(&path).expect("test file should be removable");

        assert_eq!((developed.width, developed.height), (3, 2));
        assert!(developed.has_valid_rgba_len());
    }

    #[test]
    fn embedded_jpeg_scanner_skips_nested_thumbnail_eoi() {
        let outer = RgbImage::from_pixel(4, 3, Rgb([120, 90, 60]));
        let outer_jpeg = encode_test_jpeg(&outer);
        let thumbnail = RgbImage::from_pixel(1, 1, Rgb([10, 20, 30]));
        let thumbnail_jpeg = encode_test_jpeg(&thumbnail);

        let comment_length = u16::try_from(thumbnail_jpeg.len() + 2)
            .expect("thumbnail JPEG should fit in a JPEG comment segment");
        let mut nested = Vec::new();
        nested.extend_from_slice(&outer_jpeg[..2]);
        nested.extend_from_slice(&[0xff, 0xfe]);
        nested.extend_from_slice(&comment_length.to_be_bytes());
        nested.extend_from_slice(&thumbnail_jpeg);
        nested.extend_from_slice(&outer_jpeg[2..]);

        let preview = extract_best_embedded_jpeg(&nested).expect("outer JPEG should decode");
        assert_eq!(preview.dimensions(), (4, 3));
    }

    #[test]
    fn non_finite_settings_are_rejected() {
        let mut image = RgbaImage::from_pixel(1, 1, Rgba([10, 20, 30, 255]));
        let result = apply_settings_to_rgba(
            &mut image,
            DevelopSettings {
                exposure: f32::NAN,
                ..DevelopSettings::default()
            },
        );

        assert!(matches!(result, Err(ImageEngineError::NonFiniteSettings)));
    }
}
