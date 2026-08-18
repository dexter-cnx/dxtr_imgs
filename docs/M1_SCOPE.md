# M1 Scope

## In scope

- direct Rust `raw-engine` crate
- raster preview with orientation normalization
- current embedded-JPEG RAW preview fallback
- owned RGBA image buffer
- native desktop file dialog through platform boundary
- GPUI `RenderImage` viewport
- Fit / 1:1 / bounded zoom
- mouse/middle-button pan
- trackpad/scroll pan
- pinch and Cmd/Ctrl-scroll zoom
- automated engine tests
- physical macOS validation checklist

## Out of scope

- sensor RAW demosaic/debayer
- new Develop controls or processing algorithms
- masks/LUT/export expansion
- catalog persistence
- Grid/Filmstrip implementation
- thumbnail scheduling/cache
- import orchestration
- Windows/Linux production validation
