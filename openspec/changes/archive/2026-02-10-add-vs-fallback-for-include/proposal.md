# Proposal: Add Visual Studio Fallback for INCLUDE Environment Variable

## Problem

On Windows, when no compiler is specified, the tool currently relies on the `INCLUDE` environment variable to find system include directories. If this variable is not set, the tool fails with an error. This is problematic because:

1. Users who haven't run Visual Studio's developer command prompt won't have `INCLUDE` set
2. The tool should be able to find Visual Studio automatically and extract include directories
3. Visual Studio provides `vswhere.exe` and `vsdevcmd.bat` to discover and configure development environments

## Solution

Implement automatic Visual Studio detection when `INCLUDE` is not set on Windows:

1. Check if `INCLUDE` environment variable exists (highest priority - existing behavior)
2. If not set, use `vswhere.exe` to locate Visual Studio installation
3. Run `vsdevcmd.bat -arch=x64` to initialize the environment and capture `INCLUDE` value
4. Parse and return the include directories
5. Provide detailed error messages if any step fails

## User Experience

### Before
```bash
$ get-system-include-dirs
Error: INCLUDE environment variable not set
```

### After
```bash
# Automatic fallback works
$ get-system-include-dirs
C:/tools/MSVS/17/Professional/VC/Tools/MSVC/14.44.35207/include
...

# Can specify VS version
$ get-system-include-dirs --vs-version 2022
...

# INCLUDE still takes precedence if set
$ set INCLUDE=C:\custom\path
$ get-system-include-dirs
C:/custom/path
```

## Scope

### In Scope
- Auto-detection via `vswhere.exe` (VS 2017+)
- Support for x64 architecture only
- Version filtering via `--vs-version` flag (e.g., "2022", "2026", "17", "18")
- Detailed error messages showing what was tried
- `INCLUDE` environment variable maintains highest precedence

### Out of Scope
- Support for x86, ARM, or other architectures
- Manual VS detection without vswhere (pre-VS 2017)
- Caching of detected VS paths
- Support for BuildTools-only installations (no workload filtering)

## Implementation Notes

- New module: `src/windows_vs.rs` for VS-specific logic
- New dependency: `serde` + `serde_json` for parsing vswhere JSON output
- CLI flag: `--vs-version <VERSION>` (Windows-only, ignored when compiler specified)

## Non-Goals

- Supporting Visual Studio versions older than 2017
- Cross-compilation scenarios (host ≠ target architecture)
- Performance optimization via caching
