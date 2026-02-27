## 1. Add Windows Build Job

- [x] 1.1 Add `build-windows` job to `.github/workflows/release.yml` with `runs-on: windows-2025`, `needs: [create-release]`, and `permissions: contents: write`
- [x] 1.2 Add checkout, `rustup target add x86_64-pc-windows-msvc`, and `cargo build --release --target x86_64-pc-windows-msvc` steps (using bash shell)
- [x] 1.3 Add smoke test step using PowerShell: invoke the built binary with no args, assert exit code 0 and non-empty output
- [x] 1.4 Add rename step (bash): copy binary to `get-system-include-dirs-x86_64-pc-windows-msvc.exe`
- [x] 1.5 Add upload step (bash): `gh release upload` the renamed binary

## 2. Verification

- [x] 2.1 Review the complete workflow file for correctness (shell directives, path separators, `.exe` suffix handling)
