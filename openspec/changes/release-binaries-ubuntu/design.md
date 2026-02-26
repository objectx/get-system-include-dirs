## Context

The release workflow (`release.yml`) already has a `build-macos` job covering `x86_64-apple-darwin` and `aarch64-apple-darwin`. The Justfile uses `cross` for Linux builds locally, but in GitHub Actions a dedicated `ubuntu-latest` runner natively targets `x86_64-unknown-linux-gnu`, making `cross` unnecessary.

## Goals / Non-Goals

**Goals:**
- Add a `build-ubuntu` job to the existing `release.yml` that builds `x86_64-unknown-linux-gnu` and uploads the binary to the GitHub Release
- Reuse the same `softprops/action-gh-release` action (same pinned SHA) and `GITHUB_TOKEN` pattern established by the macOS job

**Non-Goals:**
- ARM Linux, musl, or any other Linux variants
- Changing or refactoring the existing `build-macos` job
- Using `cross` in CI (unnecessary for native target)

## Decisions

### Use `cargo` directly, not `cross`

**Decision**: Use `cargo build --release --target x86_64-unknown-linux-gnu` on `ubuntu-latest`.

**Rationale**: `ubuntu-latest` runners are `x86_64` Linux, so `x86_64-unknown-linux-gnu` is the native target. `cargo` builds it natively without Docker or `cross`. The Justfile uses `cross` only to support cross-compilation from macOS; that constraint doesn't apply in CI.

**Alternative considered**: Install and use `cross` — rejected as unnecessary overhead that adds Docker dependency and longer build times.

### Add a standalone `build-ubuntu` job, not extend the matrix

**Decision**: Add a new top-level job `build-ubuntu` rather than merging Linux into the `build-macos` matrix.

**Rationale**: macOS and Ubuntu jobs run on different runner types (`macos-latest` vs `ubuntu-24.04`). A shared matrix would require a `runs-on` lookup per target, making the YAML harder to read and extend. Separate jobs are clearer and easier to extend independently (e.g., adding ARM Linux later).

**Alternative considered**: Single matrix job with `runs-on: ${{ matrix.os }}` — rejected for readability and because it conflates two distinct platform strategies.

### Reuse the same pinned `softprops/action-gh-release` SHA

**Decision**: Use the same `softprops/action-gh-release@a06a81a` (v2.5.0) as the macOS job.

**Rationale**: Consistency and avoiding drift between jobs. Both jobs upload to the same GitHub Release; using the same action version ensures identical behavior.

## Risks / Trade-offs

- **glibc version dependency** → The binary will link against the glibc version on `ubuntu-latest`. Users on older distros may get a glibc compatibility error. Mitigation: document the minimum glibc requirement; a musl build can be added later if needed.
- **Pinned runner version** → Using `ubuntu-24.04` explicitly avoids surprise glibc bumps if GitHub later updates `ubuntu-latest`. Trade-off: requires a manual update when moving to a newer Ubuntu LTS.
