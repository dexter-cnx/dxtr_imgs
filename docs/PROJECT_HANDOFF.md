# Dextryx Images — Project Handoff

Canonical status document for `dexter-cnx/dxtr_imgs`.

## Product identity

- Product: **Dextryx Images**
- Compact label: **Dxtr Imgs**
- Repository: `dexter-cnx/dxtr_imgs`
- Initial production platform: macOS
- UI runtime: GPUI
- Core language: Rust
- Declared Rust MSRV: **1.95**

## Current milestone

### M1 — Direct raw-engine viewport

Status: **implementation validated by hosted CI; physical macOS viewport validation remains**.

M0 and its PR #2 review-hardening follow-up are merged into `main`. Persisted-ID round trips, committed executable-workspace `Cargo.lock`, the GPUI-compatible Rust 1.95 MSRV, and ruleset-aligned required CI contexts are now baseline repository policy.

M1 implementation currently includes:

- dedicated `crates/raw-engine` boundary;
- direct Rust `develop_image` / `develop_preview` API;
- owned RGBA8 `DevelopedImage` output;
- ordinary raster decode with orientation normalization;
- current embedded-JPEG RAW preview fallback;
- existing neutral Exposure/Temperature/Contrast settings semantics without processing-scope expansion;
- structured `ImageEngineError`;
- native file dialog behind `PlatformFileDialog` / `DesktopFileDialog`;
- GPUI `RenderImage` adoption from the owned raw-engine buffer;
- Open Image;
- Fit / 1:1;
- bounded zoom;
- mouse/middle-button drag pan;
- trackpad/scroll pan;
- pinch zoom;
- Cmd/Ctrl + scroll zoom;
- raster and embedded-preview engine tests.

M1 explicitly does **not** include:

- sensor RAW demosaic/debayer;
- Flutter/Dart/C-FFI in the desktop image path;
- masks/LUT/export expansion;
- catalog persistence;
- thumbnail/cache work;
- Develop-panel feature expansion.

Repository required CI contexts are:

- `PR CI required`
  - `cargo fmt --all -- --check`
  - `cargo check --workspace`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo +1.95.0 check --workspace --locked`
- `Merge gate` — lightweight final context that depends on `PR CI required`

Physical macOS validation is required before M1 is marked complete:

- native app launches;
- native file picker opens;
- common raster image opens and renders with expected orientation/color;
- at least one supported RAW file opens through the existing embedded-preview path;
- Fit / 1:1 / zoom / mouse pan / trackpad pan / pinch / Cmd-scroll behave correctly;
- UI remains responsive during normal M1 usage.

## Architecture decisions already accepted

1. GPUI is presentation/application-shell code, not the owner of catalog/storage/import/image-processing rules.
2. Domain and repository contracts remain plain Rust.
3. macOS is the only initial production target; Windows/Linux remain architectural future targets.
4. One authoritative durable catalog will exist; production persistence is deferred until M6 or until justified earlier.
5. Nixin is a behavior/specification/source donor, not an architecture to copy blindly.
6. Flutter, Riverpod, go_router and Hive are not dependencies of the new architecture.
7. `raw-engine` is a direct Rust dependency in M1; no Dart/C-FFI hot path.
8. Real RAW demosaic/debayer remains out of scope.
9. GPUI is pinned to the Zed revision already validated by the Nixin spike: `fd90c0af7f021d89e511dd9a5f92d4f04ec29314`.
10. Executable dependency resolution is committed through `Cargo.lock`; GPUI pinning alone is not sufficient reproducibility.
11. The actual pinned GPUI graph/API surface establishes Rust 1.95 as the repository MSRV; CI validates that floor with a locked workspace check.
12. M1 keeps image-engine output UI-neutral as owned RGBA; GPUI-specific channel adaptation/render-image construction stays in `ui-gpui`.
13. Platform file dialogs stay behind the platform boundary; raw-engine does not own picker UX.

## Workspace layout

```text
.
├── Cargo.toml
├── Cargo.lock
├── crates/
│   ├── domain/       # framework-neutral product invariants
│   ├── app/          # commands/use-cases/repository + localization contracts
│   ├── platform/     # OS-varying service contracts/implementations
│   ├── raw-engine/   # direct Rust image preview/develop boundary
│   └── ui-gpui/      # GPUI desktop presentation shell + viewport
├── assets/
├── docs/
├── tool/
└── Makefile
```

Future catalog/import/thumbnail/storage crates should be introduced only when their responsibilities are substantial enough to justify a separate crate. Avoid aesthetic micro-crates.

## Milestone plan

- **M0** repository foundation + GPUI shell + docs — complete, including merged PR #2 review hardening
- **M1** direct `raw-engine` integration, raster/current RAW embedded preview, Fit/1:1/pan/zoom — hosted CI validated; physical macOS validation remains
- **M2** Workplace/catalog domain + repository contract tests
- **M3** virtualized Grid + Filmstrip + shared selection + 5,000 fixture
- **M4** bounded thumbnail memory/disk cache
- **M5** Import multi-select/folder + linked/managed safety + recovery state
- **M6** authoritative durable storage adapter; evaluate Rust-native Dxtr_Box if mature
- **M7** missing/relink/recovery/catalog-only removal
- **M8** current Develop shell using existing engine capability only
- **M9** macOS `.app` productization, resources, release/signing plan and physical validation

## PR policy

Use branch → focused PR → CI → merge → delete branch. Before push/merge run format, compile, tests and clippy locally. Required repository contexts must remain aligned with the ruleset (`PR CI required`, `Merge gate`). Do not mix GPUI upgrades, storage migrations, domain refactors and feature work unless technically inseparable.

## Next action

Complete physical macOS viewport validation for M1. Do not claim that manual runtime gate from hosted CI. Once validated, mark M1 complete and move to M2 Workplace/catalog domain work without expanding image-processing scope.
