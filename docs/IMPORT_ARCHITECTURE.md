# Import Architecture

## UX

Primary flow:

```text
Import → native file picker → multi-select files
```

Secondary flow: `Import Folder`.

Storage mode is an option within the flow: Linked (reference originals) or Managed (copy into managed storage).

## Correctness boundary

Import orchestration belongs in application/core services, not GPUI widgets.

Managed import sequence:

```text
validate managed root
→ choose collision-safe destination
→ copy to temporary/partial path
→ atomic/best-effort commit/rename
→ persist asset
→ mark batch item completed
```

## Safety rules

- never silently overwrite originals
- do not recreate a disappeared external managed mount merely because a pathname can be constructed
- if copy succeeds but catalog persistence fails, clean up the uncommitted managed copy when safe
- persist enough ImportBatch state for restart recovery/retry
- prevent duplicate imports according to stable identity/content/path rules established in the domain milestone
- catalog-only removal never deletes the physical original

M5 implements these flows; M0 only establishes the boundaries.
