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

Image viewport/develop path:
GPUI → direct Rust raw-engine dependency
```

## Rules

- GPUI owns presentation, window/input behavior and application-shell state.
- Domain invariants must compile and test without GPUI.
- Application services depend on repository/platform traits, not concrete databases or AppKit.
- Platform-specific behavior sits behind narrow interfaces only where OS behavior genuinely differs.
- Storage has one authoritative durable catalog.
- Path semantics use `Path`/`PathBuf`.
- Image processing stays outside widgets.

## M0 crates

### `dxtr-imgs-domain`
Framework-neutral identity and invariants: Workplace, Asset, storage mode, media type, effective path and relink semantics.

### `dxtr-imgs-app`
Use-case/repository contracts and localization boundary. This layer coordinates product behavior but does not know GPUI or a concrete database.

### `dxtr-imgs-platform`
Contracts for file dialogs, app paths and later OS-varying capabilities.

### `dxtr-imgs-ui-gpui`
Native desktop presentation shell. It may own GPUI `Entity<T>` state in later milestones, but must not become the catalog/storage rule engine.

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

## Large catalog policy

5,000 assets is an initial regression fixture. Grid/Filmstrip must virtualize from the beginning; thumbnail work must be bounded, lazy and outside render-path filesystem/decode work.

## Storage decision

The durable adapter is intentionally deferred. A future Rust-native Dxtr_Box adapter is eligible only behind repository contracts and only after maturity is demonstrated. GPUI must not depend on Dxtr_Box-specific concepts.
