# Tasks: Fix MSVC-like Compiler Dispatch

## Task List

### Task 1: Remove orphaned doc comment from main.rs
**Status**: done

Remove the leftover doc comment block (lines ~118–136) that belonged to the
deleted `is_msvc_like_compiler()` function.

**What to remove** — the block starting with:
```
/// Checks if a compiler is MSVC-like based on its filename.
///
/// MSVC-like compilers include: cl, cl.exe, clang-cl, clang-cl.exe
...
/// `true` if the compiler filename matches the pattern `cl(?:\.exe)$`
```

**Verification**: `cargo build` — no warnings, no errors.

---

### Task 2: Add is_msvc_like_compiler() to main.rs
**Status**: done

Add the function (Windows-only) immediately before `get_compiler_include_dirs`:

```rust
/// Returns `true` if the compiler filename matches a known MSVC-like compiler.
///
/// Matches (case-insensitive): `cl`, `cl.exe`, `clang-cl`, `clang-cl.exe`
///
/// # Arguments
/// * `compiler` - Path to the compiler executable
#[cfg(windows)]
fn is_msvc_like_compiler(compiler: &PathBuf) -> bool {
    compiler
        .file_name()
        .and_then(|n| n.to_str())
        .map(|name| {
            let lower = name.to_ascii_lowercase();
            matches!(lower.as_str(), "cl" | "cl.exe" | "clang-cl" | "clang-cl.exe")
        })
        .unwrap_or(false)
}
```

**Verification**: `cargo build` — no warnings.

---

### Task 3: Update Windows guard in get_include_dirs()
**Status**: done

Replace the current guard:
```rust
#[cfg(windows)]
if compiler.is_none() {
    return windows_vs::get_windows_include_dirs_with_fallback(vs_version.as_deref());
}
```

With:
```rust
#[cfg(windows)]
if compiler.as_ref().map_or(true, is_msvc_like_compiler) {
    return windows_vs::get_windows_include_dirs_with_fallback(vs_version.as_deref());
}
```

**Verification**: `cargo build` — compiles clean.

---

### Task 4: Manual smoke test
**Status**: done

```bash
# Should route to Windows INCLUDE path (not gcc-style)
get-system-include-dirs --compiler cl.exe
get-system-include-dirs --compiler clang-cl.exe
get-system-include-dirs --compiler CL.EXE   # case-insensitive

# Should still use gcc-style -v
get-system-include-dirs --compiler clang
```

**Verification**: All produce include directories or a clear error; no
"Failed to execute compiler" from gcc-style invocation for MSVC compilers.

## Notes
- No new files, no new dependencies
- `is_msvc_like_compiler` only compiled on Windows (`#[cfg(windows)]`)
- Regex dependency (`regex`) is no longer used anywhere after this change —
  check if it can be removed from `Cargo.toml`
