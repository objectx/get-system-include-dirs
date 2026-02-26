## Context

The project is a Rust CLI tool cross-compiled for multiple targets. macOS builds (`x86_64-apple-darwin`, `aarch64-apple-darwin`) use standard `cargo`; Linux uses `cross`; Windows must be built natively. There is no existing release automation — binaries are produced and distributed manually.

GitHub Actions provides macOS runners with Rust pre-installed, making it the natural fit for automating macOS builds without additional tooling.

## Goals / Non-Goals

**Goals:**
- Trigger on version tag pushes (`v*`) to produce a GitHub Release
- Build both macOS architectures (`x86_64` and `aarch64`) as separate jobs
- Upload the compiled binaries as assets on the GitHub Release
- Use `GITHUB_TOKEN` — no additional secrets needed for release asset upload

**Non-Goals:**
- Linux and Windows build jobs (future work)
- Code signing or notarization
- Publishing to Homebrew, crates.io, or other registries
- Running tests or linting in the release workflow

## Decisions

### Use a matrix strategy for macOS architectures

**Decision**: Use a single job with a `matrix` over `[x86_64-apple-darwin, aarch64-apple-darwin]` running on `macos-latest`.

**Rationale**: Both targets can be built on the same runner type (`macos-latest`, which is arm64). Cross-compiling `x86_64` from an arm64 runner is fully supported by `cargo` with the target added. A matrix avoids duplicating job steps and is easy to extend later.

**Alternative considered**: Two separate named jobs — rejected because it duplicates YAML and makes future additions (e.g., Linux) more verbose.

### Use `cargo` directly, not `just`

**Decision**: Invoke `cargo build --release --target <triple>` directly in the workflow, not `just build <target>`.

**Rationale**: `just` and `nushell` are not available on GitHub-hosted runners by default. Installing them adds latency and complexity. Since the build step is simple (`cargo build --release --target`), invoking `cargo` directly is simpler and more robust.

**Alternative considered**: Install `just` + `nushell` and run `just build` — rejected due to added complexity for no meaningful benefit in CI.

### Create the GitHub Release from the tag

**Decision**: Use `softprops/action-gh-release` to create and upload assets to the GitHub Release in one step.

**Rationale**: This action handles release creation (if it doesn't exist yet) and asset upload atomically, with minimal configuration. It reads the tag name automatically from `GITHUB_REF`.

**Alternative considered**: `gh release create` via the CLI — also viable but requires more shell scripting for idempotency.

### Add `x86_64-apple-darwin` target explicitly

**Decision**: Run `rustup target add ${{ matrix.target }}` before building, but only for `x86_64-apple-darwin` (the non-native cross-compile target on arm64 runners).

**Rationale**: `aarch64-apple-darwin` is the native target on `macos-latest` (arm64) and is already installed. `x86_64` requires explicit addition. Using `rustup target add` unconditionally is safe (no-op if already installed).

## Risks / Trade-offs

- **Runner architecture assumption** → `macos-latest` is currently arm64. If GitHub changes this, `aarch64-apple-darwin` would remain the native target and `x86_64` still cross-compiles fine. Low risk.
- **No test gate in release workflow** → A broken binary could be released if tests were not run separately. Mitigation: rely on a separate CI workflow (e.g., PR checks) to gate merges before a release tag is pushed.
- **`softprops/action-gh-release` is a third-party action** → Pin to a specific SHA for supply-chain safety. Trade-off: requires manual update to get new versions.
