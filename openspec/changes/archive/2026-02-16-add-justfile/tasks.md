## 1. Justfile Core Setup

- [x] 1.1 Create `Justfile` at project root with binary name variable derived from Cargo.toml
- [x] 1.2 Define target triple variables for all 4 platforms
- [x] 1.3 Implement build tool selection logic: `cross` when target contains `linux`, `cargo` otherwise
- [x] 1.4 Implement `.exe` suffix logic: append when target contains `windows`

## 2. Build Recipes

- [x] 2.1 Implement parametric `build <target>` recipe: build release binary and copy to `dist/<target>/`
- [x] 2.2 Implement `build-all` recipe: invoke `build` for x86_64-apple-darwin, aarch64-apple-darwin, x86_64-unknown-linux-gnu
- [x] 2.3 Implement `build-windows` convenience recipe: invoke `build` for x86_64-pc-windows-msvc

## 3. Rebuild Recipes

- [x] 3.1 Implement hidden `_clean <target>` recipe: remove `target/<target>/release/` and `dist/<target>/`
- [x] 3.2 Implement `rebuild <target>` recipe: `_clean` then `build` for a single target
- [x] 3.3 Implement `rebuild-all` recipe: full `clean` then `build-all`

## 4. Development Recipes

- [x] 4.1 Implement `check` recipe: `cargo clippy`
- [x] 4.2 Implement `fmt` recipe: `cargo fmt`
- [x] 4.3 Implement `test` recipe: `cargo test`
- [x] 4.4 Implement `clean` recipe: `cargo clean` and remove `dist/`

## 5. Gitignore Update

- [x] 5.1 Add `/dist` entry to `.gitignore`
