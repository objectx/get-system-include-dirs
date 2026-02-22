## Why

When querying include directories for cross-compilation or SDK-targeting scenarios (e.g., Android NDK, custom sysroots, macOS SDK paths), the compiler must be invoked with additional flags like `--target`, `--sysroot`, or `-isysroot`. Currently these flags cannot be passed, making the tool unusable for cross-compilation workflows.

## What Changes

- Add `-- <COMPILER_ARGS>...` passthrough support to the CLI: all arguments after `--` are forwarded verbatim to the compiler invocation
- Extra args are appended after the fixed `-v -E -x c++` flags and before the stdin sentinel `-`
- Extra args are only applied when `--compiler` is explicitly specified
- Warn (to stderr) if extra args are provided without `--compiler`, or with an MSVC-like compiler on Windows

## Capabilities

### New Capabilities

- `compiler-extra-args`: CLI passthrough of arbitrary arguments to the gcc-like compiler invocation via `--`

### Modified Capabilities

<!-- No existing spec-level requirements are changing -->

## Impact

- `src/main.rs`: `Args` struct, `get_include_dirs`, `get_compiler_include_dirs` signatures
- No new dependencies
- No breaking changes to existing CLI flags

## Non-goals

- Passing extra args to MSVC / VS detection path (`cl.exe`, `clang-cl.exe`)
- Interpreting or validating the extra args in any way
- Supporting extra args when no `--compiler` is given (default compiler path)
