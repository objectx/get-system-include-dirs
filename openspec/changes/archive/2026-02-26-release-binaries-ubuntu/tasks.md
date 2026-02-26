## 1. Add Ubuntu Build Job

- [x] 1.1 Add `build-ubuntu` job to `.github/workflows/release.yml` targeting `ubuntu-24.04` with `permissions: contents: write`
- [x] 1.2 Add `rustup target add x86_64-unknown-linux-gnu` step
- [x] 1.3 Add `cargo build --release --target x86_64-unknown-linux-gnu` step
- [x] 1.4 Add step to copy binary as `get-system-include-dirs-x86_64-unknown-linux-gnu`
- [x] 1.5 Add `gh release upload` step to upload the asset

## 2. Fix Release Race Condition (e456aeb)

- [x] 2.1 Add `create-release` job that runs first via `gh release create --generate-notes`
- [x] 2.2 Add `needs: [create-release]` to `build-macos` and `build-ubuntu`
- [x] 2.3 Replace `softprops/action-gh-release` in `build-macos` and `build-ubuntu` with `gh release upload`

## 3. Verification

- [x] 3.1 Push a test tag and verify `create-release` runs first, then all three build jobs complete and attach their assets
- [x] 3.2 Delete the test tag and release after verification
