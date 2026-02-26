## Why

Release binaries are currently built manually and distributed ad-hoc. Automating this with GitHub Actions ensures reproducible, consistent release artifacts are published directly to GitHub Releases on every version tag, removing manual steps and reducing the chance of shipping incorrect binaries.

## What Changes

- Add a GitHub Actions workflow that triggers on version tag pushes (`v*`)
- Build macOS release binaries for both `x86_64-apple-darwin` and `aarch64-apple-darwin`
- Upload the binaries as assets to the corresponding GitHub Release

## Non-goals

- Linux builds (planned for future expansion)
- Windows builds (must be built natively; planned for future expansion)
- Code signing or notarization
- Publishing to package registries (Homebrew, crates.io, etc.)

## Capabilities

### New Capabilities

- `release-workflow-macos`: GitHub Actions workflow that builds and uploads macOS release binaries (`x86_64` and `aarch64`) on version tag pushes.

### Modified Capabilities

<!-- None -->

## Impact

- New file: `.github/workflows/release.yml`
- No changes to existing source code, `Justfile`, or CI workflows
- Requires GitHub repository permissions to create releases and upload assets (via `GITHUB_TOKEN`)
