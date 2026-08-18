# Nixin Source Reuse Record

Reference repository: `dexter-cnx/nixin`.

The new project preserves approved semantics but does not blindly port Flutter architecture. Reuse is categorized as copied, adapted, rewritten or spec-only.

| Nixin source | Ref | New destination | Mode | Decision |
|---|---|---|---|---|
| `experiments/gpui-desktop/Cargo.toml` | `agent/gpui-desktop-spike` | root `Cargo.toml` | adapted | Retain the physically validated GPUI/Zed pin `fd90c0af7f021d89e511dd9a5f92d4f04ec29314` and `gpui_platform` `font-kit`; do not copy spike package topology. |
| `experiments/gpui-desktop/src/main.rs` | `agent/gpui-desktop-spike` | `crates/ui-gpui/src/main.rs` | adapted/restructured | Reuse the proven direct `raw_engine::develop_preview` → owned pixel buffer → GPUI `RenderImage` path and Fit/1:1/pan/zoom interaction model. Production code keeps viewport state separate from catalog/domain semantics and discards the spike's synthetic S2/S3 catalog implementation. |
| `docs/GPUI_ARCHITECTURE_REVIEW.md` | `agent/gpui-desktop-spike` | `docs/ARCHITECTURE.md`, handoff | spec-only | Preserve proven conclusions: direct Rust engine path, bounded large-catalog UI, framework-neutral catalog contract, persistence deferred. |
| `rust/src/api.rs` preview/develop subset | `agent/gpui-desktop-spike` plus `main` behavior | `crates/raw-engine/src/lib.rs` | adapted | Preserve raster decode with orientation normalization, embedded-JPEG RAW fallback, `DevelopSettings`, owned RGBA output and current exposure/temperature/contrast math. Replace string-only authoritative errors with `ImageEngineError`; omit mask/LUT/export implementation from M1 because those are outside the viewport milestone. |
| `rust/src/lib.rs` native `develop_image` / `develop_preview` API | `agent/gpui-desktop-spike` | `crates/raw-engine/src/lib.rs` | adapted | Keep direct Rust entry points and owned `DevelopedImage`; intentionally omit C ABI/Flutter FFI wrappers from the desktop hot path. |
| Workplace/catalog/import implementations and tests | `main` | `domain`, `app`, future catalog/import crates | rewritten from semantics | Port stable identity, Linked/Managed behavior, relink, missing, recovery, duplicate prevention, catalog-only removal and recent-import rules, not Dart/Hive/Riverpod wiring. |
| Flutter widgets / Riverpod / go_router / Hive adapters | `main` | none | not reused | Implementation-specific to the old application and intentionally excluded from the new architecture. |

## GPUI spike evidence retained

The Nixin spike demonstrated on macOS that:

- GPUI can launch natively with the required system-font feature;
- direct `raw_engine` Rust-to-Rust preview is viable;
- the existing owned pixel buffer can become a GPUI `RenderImage` without Dart/C FFI;
- Fit / 1:1 / bounded zoom, mouse/middle-button pan, trackpad pan and pinch/platform-scroll zoom are viable;
- a 5,000-asset Filmstrip can be rendered using visible-range virtualization;
- thumbnail work can be bounded (the spike used max four in flight and a 128-entry completed cache);
- stable catalog identity and repository contracts can remain outside GPUI.

These are architecture inputs, not production code guarantees.

## M1 raw-engine extraction

M1 deliberately extracts only the image-engine responsibility required for the first real viewport vertical slice:

- raster loading;
- decoder orientation normalization where available;
- current embedded JPEG RAW-preview fallback;
- `DevelopSettings` with exposure/temperature/contrast defaults and existing processing math;
- owned RGBA8 `DevelopedImage`;
- direct Rust `develop_image` / `develop_preview` calls.

M1 does **not** port the old C ABI, Dart/Flutter bridge, masks, LUT, JPEG export, or any sensor RAW demosaic/debayer path. Those remain separate responsibilities/milestones and must be added only when product scope requires them.

## Test migration rule

Behavioral tests from Nixin are specifications. Translate important cases into Rust tests rather than deleting them. M1 adds direct tests for ordinary raster preview, synthetic RAW-container embedded-JPEG fallback, owned RGBA buffer invariants and invalid develop settings. Later milestones must cover stable identity, duplicate prevention, storage-mode paths, relink, missing state, managed-copy safety, interrupted import recovery, thumbnail invalidation/corruption, RAW/raster separation and large-catalog behavior.
