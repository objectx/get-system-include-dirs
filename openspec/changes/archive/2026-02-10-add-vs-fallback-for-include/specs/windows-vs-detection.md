# Spec: Windows Visual Studio Detection

**Status**: Delta (new capability)

## Overview

Automatic Visual Studio detection and INCLUDE extraction for Windows when no compiler is specified and the INCLUDE environment variable is not set.

## Requirements

### Functional Requirements

#### FR-1: Environment Variable Precedence
**Priority**: MUST

The system MUST check the `INCLUDE` environment variable first before attempting VS detection.

**Acceptance Criteria**:
- When `INCLUDE` is set, use its value immediately
- When `INCLUDE` is not set, proceed to VS detection
- This precedence applies regardless of CLI flags like `--vs-version`

#### FR-2: VS Detection via vswhere
**Priority**: MUST

The system MUST use `vswhere.exe` to locate Visual Studio installations.

**Acceptance Criteria**:
- Check for vswhere at: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`
- If not found, return error (no fallback)
- Execute vswhere with `-format json -utf8` flags
- Parse JSON output to extract `installationPath`

#### FR-3: Version Filtering
**Priority**: MUST

The system MUST support filtering VS installations by version via `--vs-version` flag.

**Acceptance Criteria**:
- Accept friendly names: "2017", "2019", "2022", "2026"
- Accept version numbers: "15", "16", "17", "18"
- Accept custom ranges: "[17.0,18.0)"
- Map to vswhere version ranges:
  - "2026"/"18" → "[18.0,19.0)"
  - "2022"/"17" → "[17.0,18.0)"
  - "2019"/"16" → "[16.0,17.0)"
  - "2017"/"15" → "[15.0,16.0)"
- When no version specified, use `-latest` flag in vswhere

#### FR-4: vsdevcmd Execution
**Priority**: MUST

The system MUST execute `vsdevcmd.bat` to capture the INCLUDE environment variable.

**Acceptance Criteria**:
- Locate vsdevcmd.bat at: `<installationPath>\Common7\Tools\vsdevcmd.bat`
- Execute with `-arch=x64` flag (only x64 supported)
- Command format: `cmd /c "\"<path>\" -arch=x64 >nul 2>&1 && set INCLUDE"`
- Parse `INCLUDE=...` line from stdout
- Extract value after `INCLUDE=` prefix

#### FR-5: Include Path Parsing
**Priority**: MUST

The system MUST parse the INCLUDE value into individual directory paths.

**Acceptance Criteria**:
- Split by semicolon (`;`) separator
- Filter out empty entries
- Normalize backslashes to forward slashes
- Return as vector of strings

#### FR-6: Error Reporting
**Priority**: MUST

The system MUST provide detailed error messages showing what was attempted.

**Acceptance Criteria**:
- Show that INCLUDE was not set
- Show which step failed: vswhere location, vswhere execution, vsdevcmd execution
- Include relevant paths and error details
- Format: Multi-line with context

**Error Message Examples**:

vswhere not found:
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: vswhere.exe not found at standard location.
Expected: C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe
```

No VS found for version:
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: No Visual Studio installation found for version: 2022
```

vsdevcmd failed:
```
Error: INCLUDE environment variable not set.
Found VS at: C:\tools\MSVS\17\Professional
vsdevcmd.bat execution failed: <error details>
```

### Non-Functional Requirements

#### NFR-1: Platform Specificity
**Priority**: MUST

The VS detection functionality MUST only compile and be available on Windows.

**Acceptance Criteria**:
- Module `windows_vs.rs` guarded by `#[cfg(windows)]`
- `--vs-version` flag only available on Windows builds
- No impact on Unix-like platform compilation

#### NFR-2: Architecture Support
**Priority**: MUST

The system MUST only support x64 architecture.

**Acceptance Criteria**:
- Always use `-arch=x64` flag with vsdevcmd.bat
- No support for x86, ARM64, or other architectures

#### NFR-3: Performance
**Priority**: SHOULD

VS detection SHOULD complete within 5 seconds under normal conditions.

**Context**:
- Environment variable check: < 1ms
- vswhere execution: ~100-200ms
- vsdevcmd.bat execution: ~1-2 seconds
- Total expected: ~1.5-3 seconds

## Interface Specifications

### CLI Interface

#### New Flag: --vs-version

```
--vs-version <VERSION>

Visual Studio version filter (Windows only)

ARGUMENTS:
  <VERSION>    VS version: "2017", "2019", "2022", "2026", "15", "16", "17", "18",
               or custom range like "[17.0,18.0)"

NOTES:
  - Only available on Windows builds
  - Ignored if --compiler is specified
  - If INCLUDE env var is set, this flag is ignored (INCLUDE takes precedence)
  - If not specified, uses latest installed VS version
```

### Module API

#### Public Function

```rust
/// Gets Windows include directories with automatic VS detection fallback.
///
/// Precedence:
/// 1. Uses $INCLUDE if already set (highest priority)
/// 2. Finds VS with vswhere and runs vsdevcmd.bat to get INCLUDE
/// 3. Errors if vswhere not found or VS detection fails
///
/// # Arguments
/// * `vs_version` - Optional VS version filter ("2022", "2026", "17", "18")
///
/// # Returns
/// * `Ok(Vec<String>)` - Include directory paths (forward slashes)
/// * `Err(String)` - Detailed error message
pub fn get_windows_include_dirs_with_fallback(
    vs_version: Option<&str>
) -> Result<Vec<String>, String>
```

## Dependencies

### New Rust Crates

- `serde` (v1.0) with `derive` feature - For deserializing vswhere JSON
- `serde_json` (v1.0) - For parsing vswhere JSON output

### External Tools

- `vswhere.exe` - Must be present at standard location (ships with VS 2017+)
- `vsdevcmd.bat` - Must exist in VS installation (standard component)
- `cmd.exe` - Windows command processor (always available on Windows)

## Behavior Specifications

### Scenario 1: INCLUDE Already Set

```
Given: INCLUDE environment variable is set
When: User runs get-system-include-dirs
Then: Parse and return INCLUDE value
And: Do not invoke vswhere or vsdevcmd
And: Ignore --vs-version flag if provided
```

### Scenario 2: Auto-detect Latest VS

```
Given: INCLUDE environment variable is not set
And: --vs-version flag not provided
When: User runs get-system-include-dirs
Then: Execute vswhere with -latest flag
And: Run vsdevcmd.bat from found installation
And: Capture and parse INCLUDE value
And: Return include directories
```

### Scenario 3: Specific VS Version

```
Given: INCLUDE environment variable is not set
And: User provides --vs-version 2022
When: User runs get-system-include-dirs --vs-version 2022
Then: Execute vswhere with -version "[17.0,18.0)"
And: Run vsdevcmd.bat from VS 2022 installation
And: Capture and parse INCLUDE value
And: Return include directories
```

### Scenario 4: VS Not Found

```
Given: INCLUDE environment variable is not set
And: User provides --vs-version 2022
And: No VS 2022 installation exists
When: User runs get-system-include-dirs --vs-version 2022
Then: Return error with message:
      "INCLUDE environment variable not set.
       Tried to find Visual Studio: No Visual Studio installation found for version: 2022"
```

### Scenario 5: vswhere Missing

```
Given: INCLUDE environment variable is not set
And: vswhere.exe does not exist at standard location
When: User runs get-system-include-dirs
Then: Return error with message showing expected vswhere path
```

## Testing Requirements

### Manual Test Cases

1. **TC-1**: INCLUDE set → Should use INCLUDE value
2. **TC-2**: INCLUDE not set, no version flag → Should use latest VS
3. **TC-3**: INCLUDE not set, --vs-version 2022 → Should use VS 2022
4. **TC-4**: INCLUDE not set, --vs-version 17 → Should use VS 2022
5. **TC-5**: INCLUDE not set, invalid version → Should error with helpful message
6. **TC-6**: INCLUDE not set, no VS installed → Should error with helpful message
7. **TC-7**: Multiple VS versions installed → Should respect version filter

## Future Considerations (Out of Scope)

- Support for x86, ARM64 architectures
- Workload/component filtering (e.g., require C++ tools)
- Caching of detected VS paths
- Registry-based detection for VS 2015 and older
- Support for VS BuildTools-only installations
