# GPUI Notes

## Pinned revision

Dextryx Images pins both `gpui` and `gpui_platform` to Zed commit:

`fd90c0af7f021d89e511dd9a5f92d4f04ec29314`

`gpui_platform` enables `font-kit` for the system-text path validated by the Nixin GPUI spike.

The executable workspace also commits `Cargo.lock`; the source pin and lockfile together define the reproducible dependency baseline.

## Why this pin

The Nixin `agent/gpui-desktop-spike` branch physically validated this exact revision on macOS for native window launch, system text, direct Rust image-engine integration and large Filmstrip experiments. Feature milestones deliberately reuse the known-good revision rather than mixing a GPUI upgrade into product work.

## M1 viewport path

The Nixin spike proved the following native viewport flow on macOS and M1 adapts it into production boundaries:

```text
PathBuf
  ↓
raw_engine::develop_preview
  ↓
owned RGBA pixel buffer
  ↓
GPUI-side channel adaptation
  ↓
RenderImage
  ↓
viewport
```

The raw engine remains UI-neutral. GPUI-specific image construction and input handling stay in `ui-gpui`.

M1 also adapts the spike's Fit, 1:1, bounded zoom, mouse/middle-button pan, trackpad pan, pinch zoom and Cmd/Ctrl-scroll zoom behavior.

This is **not** a claim of GPU zero-copy. The current path owns and adapts a CPU pixel buffer before `RenderImage` construction.

## Dependency policy

GPUI is pre-1.0. Never track upstream `main` implicitly.

A GPUI/Zed upgrade must be a focused PR containing:

- old/new revision;
- compile/test evidence;
- required source churn;
- platform behavior regression notes;
- updated `Cargo.lock`;
- macOS launch/render/input validation;
- Windows/Linux implications where relevant.

Do not continuously chase upstream during feature work. Do not combine a GPUI upgrade with storage migration, domain redesign or unrelated product features.
