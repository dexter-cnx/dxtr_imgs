# GPUI Notes

## Pinned revision

M0 pins both `gpui` and `gpui_platform` to Zed commit:

`fd90c0af7f021d89e511dd9a5f92d4f04ec29314`

`gpui_platform` enables `font-kit` for the system-text path validated by the Nixin GPUI spike.

## Why this pin

The Nixin `agent/gpui-desktop-spike` branch physically validated this exact revision on macOS for native window launch, system text, direct Rust image-engine integration and large Filmstrip experiments. M0 deliberately reuses the known-good revision rather than mixing a GPUI upgrade into repository bootstrap.

## Dependency policy

GPUI is pre-1.0. Never track upstream `main` implicitly.

A GPUI/Zed upgrade must be a focused PR containing:

- old/new revision
- compile/test evidence
- required source churn
- platform behavior regression notes
- macOS launch validation
- Windows/Linux implications where relevant

Do not continuously chase upstream during feature work.

## M0 API usage

The native application entry follows the proven spike shape: `gpui_platform::application()`, `App::open_window`, a GPUI `Render` view, `WindowOptions`, and system-font support from `gpui_platform`.
