## Why

The project has no task runner. Building release artifacts for 4 platform targets requires remembering target triples, choosing the right build tool (`cargo` vs `cross`), and manually copying binaries to the right location. A Justfile standardizes these workflows into simple, memorable commands.

## What Changes

- Add a `Justfile` to the project root using NuShell as the script interpreter, with recipes for:
  - Building release artifacts for specific targets
  - Cross-compiling for x86_64-apple-darwin, aarch64-apple-darwin, and x86_64-unknown-linux-gnu (using `cross` for Linux)
  - Native Windows build for x86_64-pc-windows-msvc (run on a Windows machine)
  - Collecting artifacts into `dist/<target>/get-system-include-dirs`
  - Linting (`cargo clippy`), formatting (`cargo fmt`), testing (`cargo test`)
  - Cleaning build and dist artifacts
- Update `.gitignore` to exclude the `dist/` directory

## Capabilities

### New Capabilities

- `build-automation`: Justfile recipes for cross-platform release builds, artifact collection into `dist/`, and standard development workflows (check, fmt, test, clean)

### Modified Capabilities

(none)

## Non-goals

- CI/CD pipeline configuration (GitHub Actions, etc.)
- Debug build recipes (release only for platform targets)
- Packaging or archive creation (tar.gz, zip)
- Code signing or notarization
- Automatic toolchain/target installation

## Impact

- New file: `Justfile` at project root
- Modified file: `.gitignore` (add `dist/`)
- New dependency: `just` (task runner, not a Rust crate)
- Existing dependency: `nu` (NuShell, used as the Justfile script interpreter)
- Existing dependency: `cross` (already installed, used for Linux target)
