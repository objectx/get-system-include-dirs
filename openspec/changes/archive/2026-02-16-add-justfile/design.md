## Context

The project currently has no task runner. Building release artifacts for 4 platform targets (x86_64-apple-darwin, aarch64-apple-darwin, x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc) requires manual invocation of `cargo` or `cross` with the correct flags, followed by copying binaries into a distribution directory.

The developer's environment:
- macOS (arm64) as primary dev machine — cross-compiles to 3 targets
- Separate Windows machine for native MSVC builds
- `cross` already installed for Linux cross-compilation
- `just` as the chosen task runner
- NuShell (`nu`) as the script interpreter for Justfile recipes

## Goals / Non-Goals

**Goals:**
- Single-command release builds per target
- Single-command build for all cross-compilable targets
- Automatic artifact placement in `dist/<target>/`
- Correct tool selection: `cargo` for macOS/Windows, `cross` for Linux

**Non-Goals:**
- Debug builds via Just (use `cargo build` directly)
- Toolchain or target installation automation
- CI/CD integration
- Archive/packaging creation

## Decisions

### D1: Tool selection based on target triple

Use `cross` only for Linux targets, `cargo` for everything else.

**Rationale:** Both macOS targets build natively with `cargo` (just needs `rustup target add`). Windows builds happen on a native Windows machine, also using `cargo`. Only Linux requires a cross-compilation environment, which `cross` provides via Docker.

**Implementation:** A NuShell conditional in the `build` recipe that checks if the target contains `linux`, selecting `cross` or `cargo` accordingly.

**Alternative considered:** Use `cross` for all targets. Rejected because `cross` adds Docker overhead unnecessarily for native/macOS builds.

### D2: Recipe structure — parametric `build` recipe

Use a single `build <target>` recipe that accepts a target triple, rather than separate per-platform recipes.

```
just build x86_64-apple-darwin
just build-all          # builds all 3 cross-compilable targets
just build-windows      # alias for x86_64-pc-windows-msvc (Windows machine)
```

**Rationale:** Avoids recipe duplication. The logic is identical across targets except for the tool choice (D1) and the `.exe` suffix for Windows.

**Alternative considered:** Separate `build-macos-x86`, `build-macos-arm64`, `build-linux`, `build-windows` recipes. Rejected as redundant — the parametric approach is cleaner and Just supports it well.

### D3: Artifact output structure

Place binaries at `dist/<target>/get-system-include-dirs` (with `.exe` for Windows).

**Rationale:** Matches the user's stated preference. Using the full target triple as the directory name is unambiguous and aligns with Cargo's own `target/<triple>/release/` layout.

### D4: Binary name detection from Cargo.toml

Read the binary name from `Cargo.toml` rather than hardcoding it, using a Just backtick evaluated through NuShell (`cargo metadata ... | from json | get packages.0.name`).

**Rationale:** Avoids drift if the package name changes. NuShell's built-in JSON parsing (`from json | get`) replaces the need for external tools like `python3` or `jq`.

**Alternative considered:** Hardcode `get-system-include-dirs`. Simpler but brittle.

### D5: `build-all` scope

`build-all` builds only the 3 cross-compilable targets (both macOS + Linux). It does NOT include Windows.

**Rationale:** `build-all` runs on the macOS dev machine. Windows builds happen on a separate machine, so including it in `build-all` would always fail on macOS.

## Risks / Trade-offs

- **Docker required for Linux builds** → Acceptable since `cross` is already installed and the user expects this.
- **`cargo metadata` adds ~200ms to resolve the binary name** → Minor. Could fall back to hardcoding if this bothers anyone.
- **Windows recipe untestable on macOS** → Expected. The `build-windows` recipe is only meant to run on the Windows machine.
