## Why

The existing release workflow only builds macOS binaries. Linux users currently have no prebuilt binary to download. Adding an Ubuntu job covers the most common Linux deployment target (`x86_64-unknown-linux-gnu`) and completes the planned multi-platform expansion.

## What Changes

- Add a `create-release` job to `.github/workflows/release.yml` that creates the GitHub Release first (via `gh release create`), eliminating the race condition where multiple build jobs simultaneously finalized the same release
- Add a `build-ubuntu` job that builds `x86_64-unknown-linux-gnu` on `ubuntu-24.04`, waits for `create-release`, and uploads the binary via `gh release upload`
- Update `build-macos` to also wait for `create-release` and upload via `gh release upload`
- Remove `softprops/action-gh-release` from all jobs

## Non-goals

- ARM Linux targets
- Static musl builds
- Windows builds (separate effort, requires native runner)
- Any changes to the existing macOS jobs

## Capabilities

### New Capabilities

- `release-workflow-ubuntu`: GitHub Actions job that builds and uploads an `x86_64-unknown-linux-gnu` release binary on `ubuntu-latest` on version tag pushes.

### Modified Capabilities

<!-- None — the trigger, auth, and asset naming requirements from release-workflow-macos apply unchanged -->

## Impact

- Modified file: `.github/workflows/release.yml` (new `create-release` job; `build-ubuntu` job added; `build-macos` updated to depend on `create-release`)
- `softprops/action-gh-release` removed from all jobs; replaced with `gh` CLI (pre-installed on all GitHub-hosted runners)
- No source code, Justfile, or other workflow changes
