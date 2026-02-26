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

### Requirement: Ubuntu binary is uploaded as a GitHub Release asset
The Ubuntu build job SHALL upload the compiled binary as a named release asset. The binary SHALL be named `get-system-include-dirs-x86_64-unknown-linux-gnu` (no extension).

#### Scenario: Ubuntu asset is attached to the release
- **WHEN** the Ubuntu build job completes
- **THEN** the binary is uploaded to the GitHub Release corresponding to the pushed tag

#### Scenario: Ubuntu asset is distinguishable by name
- **WHEN** viewing the GitHub Release assets
- **THEN** the Ubuntu binary is identifiable by `x86_64-unknown-linux-gnu` in its filename

---

### Requirement: Ubuntu job uses only GITHUB_TOKEN
The Ubuntu build job SHALL use the built-in `GITHUB_TOKEN` with `permissions: contents: write` to upload release assets. No additional secrets SHALL be required.

#### Scenario: Ubuntu release upload succeeds without extra secrets
- **WHEN** the Ubuntu build job runs in a repository with default Actions permissions
- **THEN** the binary is uploaded to the GitHub Release using only `GITHUB_TOKEN`
