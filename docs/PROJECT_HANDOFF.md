# Dextryx Images — Project Handoff

Canonical status document for `dexter-cnx/dxtr_imgs`.

## Product identity

- Product: **Dextryx Images**
- Compact label: **Dxtr Imgs**
- Repository: `dexter-cnx/dxtr_imgs`
- Initial production platform: macOS
- UI runtime: GPUI
- Core language: Rust

## Current milestone

### M0 — Repository foundation

Status: **in progress on `agent/m0-rust-gpui-foundation`**.

Implemented foundation:

- Cargo workspace
- pinned GPUI/Zed revision
- framework-neutral `domain` crate
- application/repository/localization boundary in `app`
- platform service boundary in `platform`
- native GPUI desktop shell in `ui-gpui`
- default Workplace semantics (`My workplace`)
- structured domain/repository/platform errors
- tracing setup
- Makefile development gates
- architecture/product/source-reuse documentation

Validation still required on a physical macOS development environment:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo run -p dxtr-imgs-ui-gpui`

The connected execution environment used to bootstrap the repository cannot resolve GitHub hosts or run GitHub CLI, so dependency resolution/runtime validation is intentionally left as a gate rather than falsely claimed as complete.

## Architecture decisions already accepted

1. GPUI is presentation/application-shell code, not the owner of catalog/storage/import/image-processing rules.
2. Domain and repository contracts remain plain Rust.
3. macOS is the only initial production target; Windows/Linux remain architectural future targets.
4. One authoritative durable catalog will exist; production persistence is deferred until M6 or until justified earlier.
5. Nixin is a behavior/specification/source donor, not an architecture to copy blindly.
6. Flutter, Riverpod, go_router and Hive are not dependencies of the new architecture.
7. `raw-engine` will be integrated directly as Rust in M1; no Dart/C-FFI hot path.
8. Real RAW demosaic/debayer remains out of scope.
9. GPUI is pinned to the Zed revision already validated by the Nixin spike: `fd90c0af7f021d89e511dd9a5f92d4f04ec29314`.

## Workspace layout

```text
.
├── Cargo.toml
├── crates/
│   ├── domain/       # framework-neutral product invariants
│   ├── app/          # commands/use-cases/repository + localization contracts
│   ├── platform/     # OS-varying service contracts
│   └── ui-gpui/      # GPUI desktop presentation shell
├── assets/
├── docs/
├── tool/
└── Makefile
```

Crates for catalog/import/thumbnail/storage/raw-engine should be introduced only when the responsibility is substantial enough to justify a separate crate. Avoid aesthetic micro-crates.

## Milestone plan

- **M0** repository foundation + GPUI shell + docs
- **M1** direct `raw-engine` integration, raster/current RAW embedded preview, Fit/1:1/pan/zoom
- **M2** Workplace/catalog domain + repository contract tests
- **M3** virtualized Grid + Filmstrip + shared selection + 5,000 fixture
- **M4** bounded thumbnail memory/disk cache
- **M5** Import multi-select/folder + linked/managed safety + recovery state
- **M6** authoritative durable storage adapter; evaluate Rust-native Dxtr_Box if mature
- **M7** missing/relink/recovery/catalog-only removal
- **M8** current Develop shell using existing engine capability only
- **M9** macOS `.app` productization, resources, release/signing plan and physical validation

## PR policy

Use branch → focused PR → CI → merge → delete branch. Before push/merge run format, compile, tests and clippy locally. Do not mix GPUI upgrades, storage migrations, domain refactors and feature work unless technically inseparable.

## Next action after M0 validation

Fix only genuine M0 compile/format issues, merge M0, delete the branch, then start M1 as a separate PR. Do not expand the processing roadmap while integrating `raw-engine`.
