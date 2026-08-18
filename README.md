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

**M0 — Repository foundation**

The initial foundation establishes the Cargo workspace, pinned GPUI dependency, native macOS window, application/domain/platform boundaries, structured errors/logging, localization boundary, Makefile tooling, and canonical architecture documentation.

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

## Documentation

Start with:

- `docs/PROJECT_HANDOFF.md` — canonical project status and execution plan
- `docs/PRODUCT_SPEC.md` — approved product semantics
- `docs/ARCHITECTURE.md` — target architecture and crate boundaries
- `docs/NIXIN_SOURCE_REUSE.md` — source/behavior reuse record
- `docs/GPUI_NOTES.md` — GPUI pin and upgrade policy

## Scope guard

Initial milestones do **not** expand RAW processing, add cloud/mobile features, duplicate PixelCraft processing work, or preserve Flutter/Hive as architectural dependencies.
