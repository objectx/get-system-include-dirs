### Requirement: Workflow triggers on version tags
The release workflow SHALL trigger on pushes to tags matching the pattern `v*` (e.g., `v1.0.0`, `v0.2.1`). It SHALL NOT trigger on branch pushes or pull requests.

#### Scenario: Tag push triggers workflow
- **WHEN** a tag matching `v*` is pushed to the repository
- **THEN** the release workflow is started

#### Scenario: Branch push does not trigger workflow
- **WHEN** a commit is pushed to any branch (including `main`)
- **THEN** the release workflow is NOT triggered

---

### Requirement: Build macOS binaries for both architectures
The workflow SHALL build a release binary for `x86_64-apple-darwin` and `aarch64-apple-darwin` using a matrix strategy on a `macos-latest` runner.

#### Scenario: Matrix produces two build jobs
- **WHEN** the workflow runs
- **THEN** two parallel jobs are created — one for each target triple

#### Scenario: Binary is produced for each target
- **WHEN** a build job completes successfully
- **THEN** a release binary exists at `target/<triple>/release/<bin-name>`

---

### Requirement: Required Rust target is installed before build
The workflow SHALL run `rustup target add <target>` before invoking `cargo build` to ensure the cross-compilation target is available on the runner.

#### Scenario: Target installation succeeds
- **WHEN** `rustup target add <triple>` is executed
- **THEN** the target is available for `cargo build --target`

---

### Requirement: Binary is uploaded as a GitHub Release asset
The workflow SHALL create (or update) a GitHub Release for the triggering tag and upload each platform's binary as a named release asset. The binary SHALL be named `<bin-name>-<target>` (no extension for macOS).

#### Scenario: Release asset is attached to the tag
- **WHEN** the build job completes
- **THEN** the binary is uploaded to the GitHub Release corresponding to the pushed tag

#### Scenario: Asset name includes target triple
- **WHEN** viewing the GitHub Release assets
- **THEN** each asset is identifiable by its target triple in the filename

---

### Requirement: Workflow uses only GITHUB_TOKEN for authentication
The workflow SHALL use the built-in `GITHUB_TOKEN` secret for creating releases and uploading assets. No additional secrets or credentials SHALL be required.

#### Scenario: Release created without extra secrets
- **WHEN** the workflow runs in a repository with default Actions permissions
- **THEN** the release is created and assets are uploaded without any manually configured secrets
