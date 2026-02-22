# Design: Fix MSVC-like Compiler Dispatch

## Overview

Two targeted changes to `src/main.rs`:

1. Add `is_msvc_like_compiler()` back (exact filename match, no regex)
2. Extend the Windows guard in `get_include_dirs()` to also catch MSVC-like compilers
3. Remove orphaned doc comment (cleanup)

No new files. No new dependencies.

## Updated Dispatch Flow

```
get_include_dirs(compiler, vs_version)   [Windows build only]
         │
    ┌────┴──────────────────────────────────────────────────┐
    │  compiler.is_none()                                    │
    │  || is_msvc_like_compiler(compiler)                    │
    └──────────────────┬────────────────────────────────────┘
                       │                         │
                      YES                        NO
                       │                         │
    windows_vs::get_windows_include_dirs_    get_compiler_include_dirs()
    with_fallback(vs_version.as_deref())     (gcc/clang -v -E -x c++)
```

## Function: `is_msvc_like_compiler`

Location: `src/main.rs` (Windows-only, gated with `#[cfg(windows)]`)

```rust
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

Key decisions:
- **Filename only** — ignores the directory prefix (path-agnostic)
- **Case-insensitive** — Windows filesystem is case-insensitive; `CL.EXE` should match
- **Exact match** — no regex, no partial matching; avoids false positives

## Change to `get_include_dirs`

Replace the current Windows guard (compiler.is_none() only) with an OR condition:

```rust
// Before
#[cfg(windows)]
if compiler.is_none() {
    return windows_vs::get_windows_include_dirs_with_fallback(vs_version.as_deref());
}

// After
#[cfg(windows)]
if compiler.as_ref().map_or(true, is_msvc_like_compiler) {
    return windows_vs::get_windows_include_dirs_with_fallback(vs_version.as_deref());
}
```

`Option::map_or(true, f)` returns `true` when `None` (no compiler → Windows path)
and applies `is_msvc_like_compiler` when `Some` (compiler supplied).

## Orphaned Comment Cleanup

Lines 118–136 in `main.rs` contain a leftover doc comment block from the
deleted `is_msvc_like_compiler()` function. It must be removed entirely.

## Behaviour Table

| Invocation | Before | After |
|---|---|---|
| (no flags, Windows) | ✓ Windows INCLUDE path | ✓ Windows INCLUDE path |
| `--compiler gcc` | ✓ gcc-style `-v` | ✓ gcc-style `-v` |
| `--compiler clang` | ✓ gcc-style `-v` | ✓ gcc-style `-v` |
| `--compiler cl.exe` | ✗ gcc-style → error | ✓ Windows INCLUDE path |
| `--compiler CL.EXE` | ✗ gcc-style → error | ✓ Windows INCLUDE path |
| `--compiler clang-cl` | ✗ gcc-style → error | ✓ Windows INCLUDE path |
| `--compiler clang-cl.exe` | ✗ gcc-style → error | ✓ Windows INCLUDE path |
| (no flags, Unix) | ✓ `/usr/bin/c++` | ✓ `/usr/bin/c++` (unchanged) |
