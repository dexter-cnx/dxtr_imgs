# Thumbnail Architecture

## Boundary

```text
Catalog UI
→ ThumbnailService
→ bounded scheduler
→ memory cache
→ disk cache
→ source asset
```

Thumbnail generation is independent from GPUI widgets.

## Requirements

- lazy/background execution
- bounded concurrency
- identical in-flight request deduplication
- bounded memory and disk caches
- recoverable corrupt cache entries
- atomic/best-effort writes
- cache invalidation using stable asset identity plus source version/file metadata
- thumbnail failure must not make the catalog unavailable
- no synchronous source stat/decode from GPUI render paths

## Media policy

Raster thumbnails are first. RAW files must not enter the ordinary raster decoder merely because import accepts their extension. RAW preview remains owned by the image-engine/raw-preview boundary.

## Large catalog

5,000 assets is the initial regression fixture. Filmstrip/Grid schedule only visible range plus bounded overscan and must not eagerly create or decode all catalog entries.

The prior GPUI spike demonstrated a viable bounded shape (four thumbnail jobs in flight and a 128-entry completed cache); production values may change only with measurement.
