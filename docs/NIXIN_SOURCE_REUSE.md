# Nixin Source Reuse Record

Reference repository: `dexter-cnx/nixin`.

The new project preserves approved semantics but does not blindly port Flutter architecture. Reuse is categorized as copied, adapted, rewritten or spec-only.

| Nixin source | Ref | New destination | Mode | Decision |
|---|---|---|---|---|
| `experiments/gpui-desktop/Cargo.toml` | `agent/gpui-desktop-spike` | root `Cargo.toml` | adapted | Retain the physically validated GPUI/Zed pin `fd90c0af7f021d89e511dd9a5f92d4f04ec29314` and `gpui_platform` `font-kit`; do not copy spike package topology. |
| `experiments/gpui-desktop/src/main.rs` | `agent/gpui-desktop-spike` | `crates/ui-gpui/src/main.rs` | rewritten from proven API shape | Keep native window/system-text GPUI invocation style; discard the monolithic spike state and synthetic S2/S3 implementation. |
| `docs/GPUI_ARCHITECTURE_REVIEW.md` | `agent/gpui-desktop-spike` | `docs/ARCHITECTURE.md`, handoff | spec-only | Preserve proven conclusions: direct Rust engine path, bounded large-catalog UI, framework-neutral catalog contract, persistence deferred. |
| `rust/` (`raw-engine` v8.0.0) | `main` | future `crates/raw-engine/` or workspace dependency | future adapted/reused | M1 will inspect and reuse safe Rust-native APIs. Do not introduce Flutter FFI wrappers into the desktop hot path. |
| Workplace/catalog/import implementations and tests | `main` | `domain`, `app`, future catalog/import crates | rewritten from semantics | Port stable identity, Linked/Managed behavior, relink, missing, recovery, duplicate prevention, catalog-only removal and recent-import rules, not Dart/Hive/Riverpod wiring. |
| Flutter widgets / Riverpod / go_router / Hive adapters | `main` | none | not reused | Implementation-specific to the old application and intentionally excluded from the new architecture. |

## GPUI spike evidence retained

The Nixin spike demonstrated on macOS that:

- GPUI can launch natively with the required system-font feature;
- direct `raw_engine` Rust-to-Rust preview is viable;
- a 5,000-asset Filmstrip can be rendered using visible-range virtualization;
- thumbnail work can be bounded (the spike used max four in flight and a 128-entry completed cache);
- stable catalog identity and repository contracts can remain outside GPUI.

These are architecture inputs, not production code guarantees.

## M1 raw-engine rule

Before copying engine code, inspect the safe Rust API surface and extract/reuse only what is required for current Dextryx Images behavior: raster loading, EXIF orientation normalization, current embedded JPEG RAW preview, existing DevelopSettings where suitable, and existing agreed mask/LUT/export behavior. Real RAW sensor demosaic remains out of scope.

## Test migration rule

Behavioral tests from Nixin are specifications. Translate important cases into Rust tests rather than deleting them, especially stable identity, duplicate prevention, storage-mode paths, relink, missing state, managed-copy safety, interrupted import recovery, thumbnail invalidation/corruption, RAW/raster separation and large-catalog behavior.
