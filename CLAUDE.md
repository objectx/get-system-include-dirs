# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`get-system-include-dirs` is a small Rust CLI tool that queries C++ compilers (or Windows Visual Studio installations) to discover their default system include directories.

## Common Commands

All recipes are defined in `Justfile` and use [Just](https://just.systems/) with [Nushell](https://www.nushell.sh/) as the shell.

```sh
cargo test                    # Run tests
cargo clippy                  # Lint
cargo fmt                     # Format code
just build aarch64-apple-darwin  # Release build for a specific target
just build-all                # Build for macOS x86, macOS arm, Linux (uses `cross` for Linux)
just check                    # Alias for cargo clippy
just clean                    # Clean build + dist artifacts
```

Run a single test by name:
```sh
cargo test <test_name>
```

## Architecture

The tool has two source files:

- **`src/main.rs`** — CLI entry point, argument parsing (via `clap`), and the gcc-like compiler path. Invokes the compiler with `-v -E -x c++ [extra_args] -` and parses stderr output between `#include <...> search starts here:` and `End of search list.` Strips macOS `(framework directory)` annotations and normalizes path separators to `/`.

- **`src/windows_vs.rs`** — Windows-only module (compiled with `#[cfg(windows)]`). Handles MSVC-like compilers (`cl`, `cl.exe`, `clang-cl`, `clang-cl.exe`) and the no-compiler case. Priority: `$INCLUDE` env var → auto-detect via `vswhere.exe` → run `vsdevcmd.bat -arch=x64` to capture INCLUDE. Falls back from VS IDE editions to BuildTools product when no IDE is found.

### Decision flow in `get_include_dirs`

1. On Windows, if no compiler is given or the compiler is MSVC-like → delegate to `windows_vs::get_windows_include_dirs_with_fallback`.
2. Otherwise (gcc-like compiler, or Unix with no compiler) → default to `/usr/bin/c++` on Unix, then call `get_compiler_include_dirs`.
3. `compiler_args` (passed via `--`) are forwarded to gcc-like compilers only; a warning is emitted if they cannot be applied.

## Conventions

- Commit messages follow **Conventional Commits**.
- Releases are cross-compiled: macOS targets use `cargo`, Linux uses `cross`, Windows must be built natively.
- Output binaries go to `dist/<target>/`.

## OpenSpec Workflow

Design specs live in `openspec/specs/<change-name>/spec.md`. Active changes are worked from that directory; completed changes are archived under `openspec/changes/archive/`. The `openspec/config.yaml` records project context and per-artifact rules used when proposing or applying changes via the `opsx:*` skills.
