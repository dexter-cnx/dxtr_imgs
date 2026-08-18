# Architecture

## Target flow

```text
GPUI Desktop UI
    ↓
Application / Commands
    ↓
Workplace / Catalog / Import / Selection / Develop orchestration
    ↓
Shared Rust Core
    ↓
Storage adapter / Thumbnail service / Metadata service
    ↓
Authoritative local database

M1 image viewport path:
PathBuf
  ↓
raw-engine (direct Rust dependency)
  ↓
owned RGBA8 DevelopedImage
  ↓
GPUI RenderImage adapter
  ↓
viewport renderer
```

## Rules

- GPUI owns presentation, window/input behavior and application-shell state.
- Domain invariants must compile and test without GPUI.
- Application services depend on repository/platform traits, not concrete databases or AppKit.
- Platform-specific behavior sits behind narrow interfaces only where OS behavior genuinely differs.
- Storage has one authoritative durable catalog.
- Path semantics use `Path`/`PathBuf`.
- Image processing stays outside widgets.
- The desktop image hot path must not reintroduce Dart or C FFI.

## Current crates

### `dxtr-imgs-domain`
Framework-neutral identity and invariants: Workplace, Asset, persisted-ID round trips, storage mode, media type, effective path and relink semantics.

### `dxtr-imgs-app`
Use-case/repository contracts and localization boundary. This layer coordinates product behavior but does not know GPUI or a concrete database.

### `dxtr-imgs-platform`
Contracts for file dialogs, app paths and later OS-varying capabilities. M1 adds `DesktopFileDialog` behind `PlatformFileDialog` so GPUI does not construct `rfd` dialogs directly.

### `dxtr-imgs-raw-engine`
Direct Rust image-engine boundary introduced in M1. It owns raster/current RAW-preview decode/develop semantics and returns an owned UI-neutral RGBA8 `DevelopedImage`.

M1 deliberately exposes a small safe Rust API:

- `DevelopSettings`
- `DevelopedImage`
- `ImageEngineError`
- `develop_image`
- `develop_preview`
- `load_embedded_preview`

Ordinary raster decode attempts orientation normalization. If ordinary decoding fails, the engine may scan the source as a RAW container and select the largest decodable embedded JPEG. This is the existing preview behavior, not sensor demosaic/debayer.

The engine has no GPUI dependency. UI-specific channel adaptation and `RenderImage` construction remain in `ui-gpui`.

### `dxtr-imgs-ui-gpui`
Native desktop presentation shell. M1 owns a dedicated `ViewportState` for image path/dimensions, render image, zoom, pan/drag state and status. It calls the raw engine directly and renders the owned buffer through GPUI.

The viewport currently supports Open Image, Fit, 1:1, bounded zoom, left/middle-button pan, trackpad/scroll pan, pinch zoom and Cmd/Ctrl-scroll zoom.

## State direction

Expected GPUI state separation as features arrive:

- `Entity<AppState>`
- `Entity<WorkplaceState>`
- `Entity<CatalogState>`
- `Entity<SelectionState>`
- `Entity<ViewportState>`
- `Entity<ImportState>`
- `Entity<DevelopState>`

Do not collapse these into one giant mutable state object. Plain Rust services remain plain Rust types when GPUI lifecycle is unnecessary.

M1 uses a dedicated viewport state structure inside the current shell rather than mixing image interaction state into catalog/domain types. A later GPUI Entity boundary may replace the ownership container when cross-component observation is required.

## RAW scope boundary

M1 preserves the current Nixin embedded-JPEG RAW-preview behavior only. Real RAW sensor demosaic/debayer is explicitly out of scope. Masks/LUT/export behavior is also not pulled into M1 merely because the raw-engine crate now exists.

## Large catalog policy

5,000 assets is an initial regression fixture. Grid/Filmstrip must virtualize from the beginning; thumbnail work must be bounded, lazy and outside render-path filesystem/decode work.

## Storage decision

The durable adapter is intentionally deferred. A future Rust-native Dxtr_Box adapter is eligible only behind repository contracts and only after maturity is demonstrated. GPUI must not depend on Dxtr_Box-specific concepts.

## Dependency reproducibility

GPUI/Zed remains pinned to `fd90c0af7f021d89e511dd9a5f92d4f04ec29314`. The executable workspace also commits `Cargo.lock`; adding M1 dependencies requires refreshing that lockfile rather than relying on CI to silently re-resolve compatible versions.
