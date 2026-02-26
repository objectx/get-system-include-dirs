## 1. Create Release Workflow

- [x] 1.1 Create `.github/workflows/release.yml` with trigger on `v*` tag pushes
- [x] 1.2 Add a matrix build job for `x86_64-apple-darwin` and `aarch64-apple-darwin` on `macos-latest`
- [x] 1.3 Add `rustup target add` step before `cargo build --release --target`
- [x] 1.4 Add step to rename the binary as `<bin-name>-<target>` for upload

## 2. Release Asset Upload

- [x] 2.1 Add `softprops/action-gh-release` step (pinned to a specific SHA) to create the GitHub Release and upload the renamed binary
- [x] 2.2 Configure the step to use `GITHUB_TOKEN` and set `permissions: contents: write` on the job

## 3. Verification

- [x] 3.1 Push a test tag (e.g., `v0.0.0-test`) to a fork or test branch and verify the workflow triggers, both matrix jobs succeed, and assets appear on the GitHub Release
- [x] 3.2 Delete the test tag and release after verification
