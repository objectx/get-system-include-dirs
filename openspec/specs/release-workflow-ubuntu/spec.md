## ADDED Requirements

### Requirement: Build Ubuntu binary for x86_64
The release workflow SHALL include a job that builds a release binary for `x86_64-unknown-linux-gnu` on an `ubuntu-latest` runner using `cargo build --release --target`.

#### Scenario: Ubuntu build job runs on tag push
- **WHEN** a tag matching `v*` is pushed to the repository
- **THEN** the `build-ubuntu` job runs on `ubuntu-latest` alongside the macOS jobs

#### Scenario: Binary is produced for x86_64-unknown-linux-gnu
- **WHEN** the Ubuntu build job completes successfully
- **THEN** a release binary exists at `target/x86_64-unknown-linux-gnu/release/get-system-include-dirs`

---

### Requirement: Release is created before any build job uploads
A dedicated `create-release` job SHALL create the GitHub Release (via `gh release create`) before any build job runs. Build jobs SHALL declare `needs: [create-release]` to enforce ordering.

#### Scenario: Release exists before uploads begin
- **WHEN** a tag matching `v*` is pushed
- **THEN** the `create-release` job runs first and the build jobs wait for it to complete before uploading

---

### Requirement: Ubuntu binary is uploaded as a GitHub Release asset
The Ubuntu build job SHALL upload the compiled binary as a named release asset via `gh release upload`. The binary SHALL be named `get-system-include-dirs-x86_64-unknown-linux-gnu` (no extension).

#### Scenario: Ubuntu asset is attached to the release
- **WHEN** the Ubuntu build job completes
- **THEN** the binary is uploaded to the GitHub Release corresponding to the pushed tag

#### Scenario: Ubuntu asset is distinguishable by name
- **WHEN** viewing the GitHub Release assets
- **THEN** the Ubuntu binary is identifiable by `x86_64-unknown-linux-gnu` in its filename

---

### Requirement: Build jobs use only GITHUB_TOKEN and gh CLI
All build jobs and the `create-release` job SHALL use the pre-installed `gh` CLI with `GITHUB_TOKEN` and `permissions: contents: write`. No third-party upload actions SHALL be used.

#### Scenario: Release upload succeeds without extra secrets
- **WHEN** the workflow runs in a repository with default Actions permissions
- **THEN** the release is created and all assets are uploaded using only `GITHUB_TOKEN`
