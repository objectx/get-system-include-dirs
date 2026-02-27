## Why

The release workflow currently builds macOS (x86_64/arm64) and Linux (x86_64) binaries but not Windows, even though the tool has significant Windows-specific functionality (VS auto-detection via vswhere, vsdevcmd.bat integration, INCLUDE parsing). Adding a Windows build to the release workflow completes platform coverage and lets users download a pre-built `.exe` from GitHub Releases.

## What Changes

- Add a `build-windows` job to `.github/workflows/release.yml` that builds `x86_64-pc-windows-msvc` on `windows-2025`
- Include a smoke test that runs the built binary to exercise the full VS auto-detection code path
- Upload the binary as `get-system-include-dirs-x86_64-pc-windows-msvc.exe` to the GitHub Release

## Non-goals

- ARM64 Windows (`aarch64-pc-windows-msvc`)
- Code signing
- Comprehensive test suite in CI (the smoke test is a basic integration check, not full coverage)

## Capabilities

### New Capabilities

- `release-workflow-windows`: GitHub Actions job that builds, smoke-tests, and uploads a Windows x86_64 release binary on version tag pushes.

### Modified Capabilities

<!-- None -->

## Impact

- Modified file: `.github/workflows/release.yml` (new `build-windows` job added)
- No changes to source code, Justfile, or other workflows
- Uses existing `GITHUB_TOKEN` permissions (contents: write)
