# Product Spec

## Product

Dextryx Images is a professional local desktop photo-management/photo-editing shell. The compact UI label is `Dxtr Imgs`.

## Initial platform

macOS-first and desktop-first. Windows/Linux are future additive targets; mobile is not part of this codebase's initial scope.

## Core concepts

### Workplace

- first launch resolves or creates `My workplace`
- active Workplace is deterministic after successful bootstrap
- multiple Workplaces may be added later

### Catalog

Initial views:

- All photos
- Missing
- Recent imports

Views filter/reorder the same stable asset identities; they do not duplicate assets.

### Asset/storage

- stable `AssetId`
- `Linked`: original remains at `source_path`
- `Managed`: application-owned copy is at `managed_path`
- effective path is managed path for Managed, source path for Linked
- relink preserves identity and updates the path corresponding to storage mode
- catalog removal never deletes the original file

### Import UX

Primary: **Import** → native picker → multi-select files.

Secondary: **Import Folder**.

Storage option is selected as a mode: reference originals (Linked) or copy into managed storage (Managed). Do not present three competing primary import actions.

### Grid + Filmstrip

Both share one selected asset ID. Catalog scale is designed around virtualization; 5,000 assets is the first representative regression fixture.

### Viewport/develop

The desktop application calls the Rust image engine directly. Initial image-engine parity includes existing raster/current embedded-RAW-preview behavior and current Develop controls where appropriate. Real sensor RAW demosaic/debayer is explicitly out of scope.

## Processing boundary

Dextryx Images owns catalog/workplace/import/metadata/selection/thumbnails/browsing/editor orchestration. PixelCraft/Dextryx Pixels remains the main processing-product authority; this project must not absorb that roadmap casually.

## Non-goals

No cloud sync, collaboration, CRDT, server backend, mobile UI, plugin marketplace, AI-processing expansion, permanent Flutter embedding, or automatic migration of old Nixin databases before the storage format is explicitly designed.
