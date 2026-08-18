# Storage Architecture

## Principle

Dextryx Images will have exactly one authoritative durable catalog. UI state, caches and derived views must never become competing sources of truth.

## Contracts

Application/domain code depends on repository traits, initially including:

- `WorkplaceRepository`
- `CatalogRepository`
- `ImportBatchRepository`
- `SettingsRepository`

Concrete persistence is not selected in M0.

## Deferred production adapter

Early vertical slices may use an in-memory adapter for contract testing. That is not production persistence.

At M6, select the smallest robust local Rust adapter that satisfies restart persistence, transaction/recovery behavior, schema evolution and catalog correctness. A mature Rust-native Dxtr_Box adapter may be evaluated then, but remains an implementation detail behind repository contracts.

## Prohibitions

- no direct GPUI-to-database dependency
- no Hive port
- no second durable catalog
- no macOS-specific storage format
- no automatic migration of Nixin databases before the new format is explicitly designed
