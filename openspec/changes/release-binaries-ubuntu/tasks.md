## 1. Add Ubuntu Build Job

- [x] 1.1 Add `build-ubuntu` job to `.github/workflows/release.yml` targeting `ubuntu-latest` with `permissions: contents: write`
- [x] 1.2 Add `rustup target add x86_64-unknown-linux-gnu` step
- [x] 1.3 Add `cargo build --release --target x86_64-unknown-linux-gnu` step
- [x] 1.4 Add step to copy binary as `get-system-include-dirs-x86_64-unknown-linux-gnu`
- [x] 1.5 Add `softprops/action-gh-release` upload step (same pinned SHA `a06a81a` as macOS job)

## 2. Verification

- [ ] 2.1 Push a test tag and verify the `build-ubuntu` job succeeds and the Linux binary appears as a release asset
- [ ] 2.2 Delete the test tag and release after verification
