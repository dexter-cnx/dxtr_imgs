# M1 Implementation Notes

M1 establishes the first production direct-Rust image viewport vertical slice.

## Boundaries

- `crates/raw-engine`: image decode/develop semantics and owned RGBA output.
- `crates/platform`: native file dialog service.
- `crates/ui-gpui`: GPUI image adaptation, RenderImage ownership and viewport input.

No Flutter/Dart/C-FFI layer participates in the desktop image path.

## Reused behavior

The image pipeline is adapted from Nixin's current raw-engine and its GPUI spike. M1 keeps ordinary raster decode/orientation behavior, embedded-JPEG RAW preview fallback, neutral DevelopSettings and the proven GPUI viewport interaction model.

## Intentionally deferred

- sensor RAW demosaic/debayer;
- masks;
- LUT application;
- export;
- catalog-driven viewport selection;
- async/coalesced Develop rendering;
- thumbnail/cache work.

Those responsibilities must arrive through their own milestones rather than expanding M1 opportunistically.
