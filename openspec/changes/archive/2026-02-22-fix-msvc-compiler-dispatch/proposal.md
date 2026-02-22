# Proposal: Fix MSVC-like Compiler Dispatch

## Problem

When a user passes `--compiler cl.exe` (or `clang-cl`, `clang-cl.exe`, `cl`), the tool
falls through to `get_compiler_include_dirs()` which runs gcc-style flags
(`-v -E -x c++ -`). MSVC and clang-cl do not support this invocation, so the
tool always errors for these compilers.

Additionally, the orphaned doc comment from the deleted `is_msvc_like_compiler()`
function still exists in `main.rs` (lines 118–136), creating a misleading
code comment.

## Solution

1. Restore `is_msvc_like_compiler()` in `main.rs` using an exact filename match
   (no regex) against the four known MSVC-like compiler names.
2. In `get_include_dirs()`, on Windows, if a compiler is supplied and is
   MSVC-like, redirect to `windows_vs::get_windows_include_dirs_with_fallback()`
   — the same path used when no compiler is specified.
3. Remove the orphaned doc comment.

## Scope

### In Scope
- Detecting these four compiler filenames (case-insensitive on Windows): `cl`,
  `cl.exe`, `clang-cl`, `clang-cl.exe`
- Routing MSVC-like compilers to the existing Windows INCLUDE fallback path
- Cleaning up the orphaned doc comment in `main.rs`

### Out of Scope
- Path-based VS installation inference from compiler path
- clang-cl-specific gcc-style `-v` querying
- Any other compiler identification logic

## Non-Goals

- Supporting non-MSVC compilers on Windows differently than today
- Inferring VS version from a supplied compiler path
