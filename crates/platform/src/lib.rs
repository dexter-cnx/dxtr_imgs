use std::path::PathBuf;

use rfd::FileDialog;
use thiserror::Error;

pub const IMAGE_AND_RAW_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tif", "tiff", "webp", "arw", "cr2", "cr3", "nef", "dng", "raf",
    "orf",
];

pub trait PlatformFileDialog: Send + Sync {
    fn pick_image_file(&self) -> Result<Option<PathBuf>, PlatformError>;
    fn pick_files(&self) -> Result<Vec<PathBuf>, PlatformError>;
    fn pick_folder(&self) -> Result<Option<PathBuf>, PlatformError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DesktopFileDialog;

impl PlatformFileDialog for DesktopFileDialog {
    fn pick_image_file(&self) -> Result<Option<PathBuf>, PlatformError> {
        Ok(FileDialog::new()
            .set_title("Open image or RAW preview")
            .add_filter("Images / RAW previews", IMAGE_AND_RAW_EXTENSIONS)
            .pick_file())
    }

    fn pick_files(&self) -> Result<Vec<PathBuf>, PlatformError> {
        Ok(FileDialog::new()
            .set_title("Import")
            .add_filter("Images / RAW", IMAGE_AND_RAW_EXTENSIONS)
            .pick_files()
            .unwrap_or_default())
    }

    fn pick_folder(&self) -> Result<Option<PathBuf>, PlatformError> {
        Ok(FileDialog::new().set_title("Import Folder").pick_folder())
    }
}

pub trait PlatformPaths: Send + Sync {
    fn app_support_dir(&self) -> Result<PathBuf, PlatformError>;
    fn cache_dir(&self) -> Result<PathBuf, PlatformError>;
}

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("platform operation is not available")]
    Unavailable,
    #[error("platform operation failed: {0}")]
    Operation(&'static str),
}
