# Code Walkthrough

## Root

`Cargo.toml` defines the Rust workspace and pins GPUI/Zed to the known-good revision used by the Nixin spike. `Makefile` exposes the local quality gates expected before each push.

## `crates/domain`

Framework-neutral product invariants. M0 introduces Workplace/Asset identity, `StorageMode`, `MediaType`, effective-path semantics and relink behavior. This crate must remain independent from GPUI and concrete storage.

## `crates/app`

Application-facing contracts. M0 defines Workplace/Catalog repository traits plus placeholder ImportBatch/Settings contracts and a lightweight localization boundary (`MessageKey`, `Translator`).

## `crates/platform`

Narrow OS-varying contracts. M0 defines native file/folder picker and app/cache path boundaries without wrapping ordinary `std` filesystem behavior unnecessarily.

## `crates/ui-gpui`

Native GPUI executable. The M0 shell launches a macOS window with left Workplace/Catalog navigation, central catalog placeholder and right Develop placeholder. It initializes tracing and uses the application localization boundary.

The UI does not implement catalog persistence, import correctness or image processing.

## Evolution

M1 adds direct Rust image-engine integration. M2+ grows catalog/application responsibilities behind plain Rust contracts. New crates should be introduced only when their responsibility becomes independently meaningful.
