# M1 Desktop Validation

M1 is not complete until repository CI passes and the direct Rust viewport is validated on a physical macOS development environment.

## Automated gates

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- raw-engine raster preview test
- raw-engine synthetic embedded-JPEG RAW fallback test
- raw-engine RGBA length invariant
- non-finite DevelopSettings rejection

## Physical macOS gate

Run:

```bash
make run
```

Validate:

1. Dextryx Images opens as a native GPUI window.
2. **Open Image** presents the native desktop file picker.
3. Open a JPEG/PNG and verify expected orientation and color.
4. Open at least one supported camera RAW (`ARW`, `CR2`, `CR3`, `NEF`, `DNG`, `RAF`, or `ORF`) that contains an embedded JPEG preview.
5. Verify the RAW opens through the current preview path; do not interpret this as sensor demosaic/debayer support.
6. **Fit** returns the image to a centered fit view.
7. **1:1** produces 100% zoom.
8. Zoom buttons remain bounded between 0.1× and 8×.
9. Left-button drag pans the image.
10. Middle-button drag pans the image.
11. Trackpad/scroll input pans without the platform modifier.
12. Pinch zoom changes zoom smoothly.
13. Cmd + scroll zooms on macOS.
14. Repeated open/zoom/pan operations do not crash or visibly corrupt the image.

## Scope guard

A pass here validates the M1 desktop viewport architecture only. It does not validate real RAW sensor development, catalog scale, thumbnail scheduling, import correctness, durable storage, masks, LUTs, or export behavior.
