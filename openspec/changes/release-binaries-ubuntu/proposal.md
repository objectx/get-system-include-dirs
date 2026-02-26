## Why

The existing release workflow only builds macOS binaries. Linux users currently have no prebuilt binary to download. Adding an Ubuntu job covers the most common Linux deployment target (`x86_64-unknown-linux-gnu`) and completes the planned multi-platform expansion.

## What Changes

- Add a `build-ubuntu` job to `.github/workflows/release.yml` that builds `x86_64-unknown-linux-gnu` on `ubuntu-latest` and uploads the binary as a release asset

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

- Modified file: `.github/workflows/release.yml` (new `build-ubuntu` job added alongside `build-macos`)
- No source code, Justfile, or other workflow changes
