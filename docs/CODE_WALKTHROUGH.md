# Code Walkthrough

## Root

`Cargo.toml` defines the Rust workspace and pins GPUI/Zed to the known-good revision used by the Nixin spike. `Cargo.lock` fixes the executable workspace dependency graph. `Makefile` exposes the local quality gates expected before each push.

## `crates/domain`

Framework-neutral product invariants. M0 introduces Workplace/Asset identity, persisted-ID round trips, `StorageMode`, `MediaType`, effective-path semantics and relink behavior. This crate must remain independent from GPUI and concrete storage.

## `crates/app`

Application-facing contracts. M0 defines Workplace/Catalog repository traits plus placeholder ImportBatch/Settings contracts and a lightweight localization boundary (`MessageKey`, `Translator`).

## `crates/platform`

Narrow OS-varying contracts. `PlatformFileDialog` is the UI-facing boundary for file/folder selection. M1 adds `DesktopFileDialog`, backed by `rfd`, including the single-image/RAW picker used by the viewport. The UI does not instantiate `rfd::FileDialog` directly.

## `crates/raw-engine`

Direct Rust image-engine boundary introduced in M1.

Public M1 API:

- `DevelopSettings`
- `DevelopedImage`
- `ImageEngineError`
- `develop_image(path, settings)`
- `develop_preview(path)`
- `load_embedded_preview(path)`

The engine first attempts ordinary raster decoding and applies decoder orientation normalization. If that path fails, it reads the source as a possible RAW container and selects the largest decodable embedded JPEG. This intentionally preserves the current preview-only RAW behavior; no sensor demosaic/debayer occurs.

The returned `DevelopedImage` owns its RGBA8 pixel vector. The desktop application calls this API directly—there is no Dart or C ABI in the desktop viewport path.

M1 tests cover:

- raster → owned RGBA preview;
- synthetic RAW container → embedded JPEG fallback;
- RGBA length invariants;
- rejection of non-finite develop settings.

## `crates/ui-gpui`

Native GPUI executable. M1 upgrades the central placeholder into a real image viewport while preserving the left Workplace/Catalog shell and right Develop shell.

The viewport flow is:

```text
DesktopFileDialog
      ↓ PathBuf
raw_engine::develop_preview
      ↓ DevelopedImage { width, height, RGBA }
validated owned pixel buffer
      ↓
GPUI RenderImage
      ↓
viewport renderer
```

The proven GPUI renderer path requires the channel swap used by the Nixin spike before constructing the `image::RgbaImage` adopted by `RenderImage`; the image engine itself remains RGBA-oriented and UI-agnostic.

Viewport state owns only presentation concerns:

- selected image path;
- dimensions / render image;
- zoom;
- pan offset / drag state;
- status text.

M1 interactions:

- Open Image via native dialog;
- Fit;
- 1:1;
- bounded 0.1×–8× zoom;
- zoom buttons;
- left/middle-button drag pan;
- trackpad/scroll pan;
- pinch zoom;
- Cmd/Ctrl + scroll zoom.

Catalog persistence, import correctness and processing rules remain outside GPUI.

## Evolution

M2 adds the real Workplace/catalog domain and repository contract behavior. M3+ adds virtualized Grid/Filmstrip and bounded thumbnail work. Processing scope must not expand merely because the engine crate now exists.
