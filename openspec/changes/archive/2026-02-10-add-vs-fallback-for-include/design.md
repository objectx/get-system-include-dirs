# Design: Add Visual Studio Fallback for INCLUDE Environment Variable

## Overview

This design implements automatic Visual Studio detection on Windows when the `INCLUDE` environment variable is not set. The implementation uses Microsoft's official `vswhere.exe` tool to locate VS installations and `vsdevcmd.bat` to extract include directories.

## Architecture

### Module Structure

```
src/
├── main.rs              # CLI orchestration, delegates to platform-specific logic
└── windows_vs.rs        # VS detection and INCLUDE extraction (new)
```

### Data Flow

```
┌──────────────────────────────────────────────────────────┐
│  get_include_dirs(compiler, vs_version)                  │
└────────────────┬─────────────────────────────────────────┘
                 │
    ┌────────────┴─────────────┐
    ▼                          ▼
Windows +                   Other paths
no compiler                 (unchanged)
    │
    ▼
┌─────────────────────────────────────────────┐
│ windows_vs::get_windows_include_dirs_       │
│              with_fallback(vs_version)      │
└─────────────────┬───────────────────────────┘
                  │
        ┌─────────┴─────────┐
        ▼                   ▼
   $INCLUDE set?          NOT set
        │                   │
        ▼                   ▼
   Parse & return     Find VS & get INCLUDE
```

### Precedence Rules (Critical)

1. **Highest**: `$INCLUDE` environment variable (if set)
2. **Fallback**: vswhere + vsdevcmd.bat detection
3. **Error**: If vswhere not found or detection fails

**Important**: `$INCLUDE` is always checked first, regardless of CLI flags. This ensures compatibility with existing VS developer command prompts.

## Component Design

### 1. New Module: `src/windows_vs.rs`

#### Public API

```rust
/// Main entry point - handles precedence and fallback logic
pub fn get_windows_include_dirs_with_fallback(
    vs_version: Option<&str>
) -> Result<Vec<String>, String>
```

#### Internal Functions

```rust
// Environment parsing
fn parse_include_env(include_var: &str) -> Result<Vec<String>, String>

// VS detection pipeline
fn find_vs_and_get_include(vs_version: Option<&str>) -> Result<Vec<String>, String>
fn find_vswhere() -> Result<PathBuf, String>
fn query_vswhere(vswhere_path: &PathBuf, vs_version: Option<&str>) -> Result<String, String>
fn map_version_to_range(version: &str) -> Result<String, String>
fn run_vsdevcmd_and_capture_include(vsdevcmd_path: &str) -> Result<String, String>
```

#### Data Structures

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct VsInstance {
    #[serde(rename = "installationPath")]
    installation_path: String,

    #[serde(rename = "installationVersion")]
    installation_version: String,
}
```

### 2. CLI Changes

#### New Argument

```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    compiler: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<String>,

    /// Visual Studio version (Windows only, ignored when compiler specified)
    #[cfg(windows)]
    #[arg(long)]
    vs_version: Option<String>,
}
```

#### Version String Mapping

| User Input | vswhere Range | VS Version |
|------------|---------------|------------|
| `"2026"` or `"18"` | `[18.0,19.0)` | VS 2026 |
| `"2022"` or `"17"` | `[17.0,18.0)` | VS 2022 |
| `"2019"` or `"16"` | `[16.0,17.0)` | VS 2019 |
| `"2017"` or `"15"` | `[15.0,16.0)` | VS 2017 |
| `"[17.0,18.0)"` | `[17.0,18.0)` | Custom range |
| None | `-latest` flag | Latest installed |

### 3. VS Detection Algorithm

#### Step 1: Locate vswhere.exe

- **Path**: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`
- **Error if missing**: No fallback to manual detection

#### Step 2: Query VS Installation

```bash
# Without version filter (use latest)
vswhere.exe -format json -utf8 -latest

# With version filter
vswhere.exe -format json -utf8 -version "[17.0,18.0)"
```

Parse JSON response and extract `installationPath` from first result.

#### Step 3: Execute vsdevcmd.bat

```bash
cmd /c "\"<installationPath>\Common7\Tools\vsdevcmd.bat\" -arch=x64 >nul 2>&1 && set INCLUDE"
```

- Always use `-arch=x64` (only x64 supported)
- Redirect vsdevcmd output to nul (only want `set INCLUDE` output)
- Parse `INCLUDE=...` from stdout

#### Step 4: Parse INCLUDE Value

- Split by `;` separator
- Filter empty entries
- Normalize backslashes to forward slashes
- Return as `Vec<String>`

### 4. Error Handling Strategy

Detailed, multi-level error messages that show what was tried:

#### Scenario 1: vswhere not found
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: vswhere.exe not found at standard location.
Expected: C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe
```

#### Scenario 2: No VS installation found
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: No Visual Studio installation found for version: 2022
```

#### Scenario 3: vsdevcmd.bat execution failed
```
Error: INCLUDE environment variable not set.
Found VS at: C:\tools\MSVS\17\Professional
vsdevcmd.bat execution failed: <stderr output>
```

#### Scenario 4: INCLUDE variable not in output
```
Error: INCLUDE environment variable not set.
Found VS at: C:\tools\MSVS\17\Professional
Could not find INCLUDE variable in vsdevcmd.bat output
```

## Implementation Details

### Dependency Changes

**Cargo.toml**:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
regex = "1.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Integration Points

**main.rs modifications**:

1. Add conditional module import:
```rust
#[cfg(windows)]
mod windows_vs;
```

2. Update `Args` struct with `vs_version` field (Windows-only)

3. Modify `get_include_dirs()` signature:
```rust
fn get_include_dirs(
    compiler: Option<PathBuf>,
    #[cfg(windows)] vs_version: Option<String>
) -> Result<Vec<String>, String>
```

4. Update Windows path in `get_include_dirs()`:
```rust
#[cfg(windows)]
if compiler.is_none() {
    return windows_vs::get_windows_include_dirs_with_fallback(
        vs_version.as_deref()
    );
}
```

5. Thread `vs_version` through from `main()`:
```rust
match get_include_dirs(
    args.compiler,
    #[cfg(windows)] args.vs_version
) {
    // ... existing error handling
}
```

### Platform-Specific Compilation

- `windows_vs.rs` module only compiled on Windows targets
- `--vs-version` flag only available on Windows (via `#[cfg(windows)]`)
- No impact on Unix-like platform builds

## Testing Considerations

### Manual Testing Scenarios

1. **$INCLUDE already set**: Should use existing value (no vswhere call)
2. **$INCLUDE not set, latest VS**: Should find and use latest installation
3. **$INCLUDE not set, specific version**: Should find requested VS version
4. **No VS installed**: Should produce helpful error message
5. **vswhere missing**: Should produce helpful error message
6. **Multiple VS versions**: Should respect version filter

### Command Examples

```bash
# Fallback to latest VS
get-system-include-dirs

# Use specific VS version
get-system-include-dirs --vs-version 2022
get-system-include-dirs --vs-version 17

# INCLUDE takes precedence (even with --vs-version)
set INCLUDE=C:\custom\path
get-system-include-dirs --vs-version 2022
# Output: C:/custom/path
```

## Performance Considerations

- **Fast path**: `$INCLUDE` check is instant (environment variable read)
- **Slow path**: vsdevcmd.bat execution takes ~1-2 seconds
- **No caching**: Each invocation re-detects (acceptable for CLI tool)

## Future Extensions (Out of Scope)

- Support for additional architectures (x86, ARM64)
- VS BuildTools detection with workload filtering
- Caching of detected VS paths
- Registry-based detection for pre-2017 VS versions
