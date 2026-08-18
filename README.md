# Dextryx Images

`dxtr_imgs` is the Rust-native desktop implementation of **Dextryx Images** (`Dxtr Imgs`).

The project is a new desktop codebase, not a Flutter migration branch. It preserves approved product semantics from [`dexter-cnx/nixin`](https://github.com/dexter-cnx/nixin) while rebuilding the desktop application around a clean Rust-native architecture.

## Direction

- Rust-native
- GPUI desktop UI
- macOS-first production target
- local-first catalog/workplace model
- direct Rust image-engine integration
- framework-neutral domain and storage contracts
- Windows/Linux kept architecturally additive, not initial runtime targets

## Current milestone

**M1 — Direct raw-engine viewport**

The desktop shell now has a direct Rust image path: native file dialog → `raw-engine` → owned RGBA buffer → GPUI `RenderImage`. M1 supports common raster previews plus the existing embedded-JPEG RAW-preview behavior, with Fit, 1:1, bounded zoom, mouse/trackpad pan and pinch/Cmd-scroll zoom.

This milestone intentionally does **not** add real sensor RAW demosaic/debayer or expand the processing feature set.

## Commands

```bash
make setup
make fmt
make check
make test
make clippy
make run
```

`make run` launches the GPUI desktop shell on macOS.

## M1 validation

After CI passes, physical macOS validation should cover:

- native application launch;
- Open Image native dialog;
- raster preview and orientation;
- an existing supported RAW file through embedded preview;
- Fit / 1:1 / zoom / pan / pinch / Cmd-scroll behavior.

## Documentation

Start with:

- `docs/PROJECT_HANDOFF.md` — canonical project status and execution plan
- `docs/PRODUCT_SPEC.md` — approved product semantics
- `docs/ARCHITECTURE.md` — target architecture and crate boundaries
- `docs/CODE_WALKTHROUGH.md` — current code/data flow
- `docs/NIXIN_SOURCE_REUSE.md` — source/behavior reuse record
- `docs/GPUI_NOTES.md` — GPUI pin and upgrade policy

## Scope guard

Initial milestones do **not** expand RAW processing, add cloud/mobile features, duplicate PixelCraft processing work, or preserve Flutter/Hive as architectural dependencies.
