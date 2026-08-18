# Platform Support

## Initial production target

macOS only.

M0 must establish a native GPUI window and system text. M9 will productize a normal `.app` bundle, release resources, configurable bundle identifier, signing/notarization plan and physical validation.

## Architecture for future platforms

Windows and Linux are future additive targets. Core/domain/storage formats must not assume macOS semantics. OS-varying behavior should live behind explicit contracts such as file dialogs, app paths, lifecycle, external-open, clipboard and filesystem capabilities.

`Path`/`PathBuf` are authoritative for paths; do not build cross-platform semantics from string concatenation.

## Support claims

Do not claim Windows/Linux runtime support until dedicated compile/launch validation exists. Architecture compatibility is not the same as production support.
