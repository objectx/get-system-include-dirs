# Spec: Windows Visual Studio Detection

## Purpose

Automatic Visual Studio detection and INCLUDE extraction for Windows when no compiler is specified and the INCLUDE environment variable is not set.

## Requirements

### Requirement: $INCLUDE precedence

The system SHALL check the `INCLUDE` environment variable before attempting any Visual Studio detection. When `INCLUDE` is set, the system SHALL use its value immediately and SHALL NOT invoke `vswhere.exe` or `vsdevcmd.bat`. This precedence SHALL apply regardless of whether `--vs-version` was passed on the command line.

#### Scenario: INCLUDE is set, no flag

- **WHEN** `INCLUDE` is set in the environment and no `--vs-version` flag is given
- **THEN** the system SHALL parse and use the `INCLUDE` value
- **THEN** the system SHALL NOT invoke `vswhere.exe`
- **THEN** the system SHALL NOT invoke `vsdevcmd.bat`

#### Scenario: INCLUDE is set, --vs-version also provided

- **WHEN** `INCLUDE` is set in the environment and `--vs-version 2022` is given
- **THEN** the system SHALL parse and use the `INCLUDE` value
- **THEN** the system SHALL ignore the `--vs-version` flag

#### Scenario: INCLUDE is not set

- **WHEN** `INCLUDE` is not set in the environment
- **THEN** the system SHALL proceed to Visual Studio detection via `vswhere.exe`

---

### Requirement: vswhere two-attempt detection

The system SHALL locate Visual Studio or Build Tools installations via `vswhere.exe` using a two-attempt strategy that gives VS IDE installations priority while supporting BuildTools-only environments. The system SHALL look for `vswhere.exe` at `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`. If `vswhere.exe` is absent the system SHALL return an error and SHALL NOT fall back. Attempt 1 SHALL invoke `vswhere.exe` with `-format json -utf8` (no `-products` flag); on a non-empty result the system SHALL use the first installation and SHALL NOT execute attempt 2. Attempt 2 SHALL run only when attempt 1 returns an empty result and SHALL invoke `vswhere.exe` with `-products Microsoft.VisualStudio.Product.BuildTools -format json -utf8`. If both attempts return empty results the system SHALL return an error indicating that neither VS IDE nor Build Tools was found. If `vswhere.exe` execution itself fails (a non-empty-result failure), the system SHALL propagate the error immediately without retrying.

#### Scenario: VS IDE installed

- **WHEN** at least one VS IDE installation exists and `vswhere.exe` is present
- **THEN** attempt 1 SHALL return the IDE installation
- **THEN** the system SHALL use the first IDE installation in the result list
- **THEN** attempt 2 SHALL NOT be executed

#### Scenario: BuildTools only installed

- **WHEN** no VS IDE is installed but Build Tools are installed and `vswhere.exe` is present
- **THEN** attempt 1 SHALL return an empty result
- **THEN** attempt 2 SHALL be executed and SHALL return the BuildTools installation
- **THEN** the system SHALL use the first BuildTools installation in the result list

#### Scenario: VS IDE and BuildTools both installed

- **WHEN** both a VS IDE installation and a Build Tools installation are present
- **THEN** attempt 1 SHALL return the IDE installation (giving IDE priority)
- **THEN** attempt 2 SHALL NOT be executed

#### Scenario: Neither VS IDE nor BuildTools installed

- **WHEN** `vswhere.exe` is present but no Visual Studio or Build Tools installation can be found
- **THEN** both attempt 1 and attempt 2 SHALL return empty results
- **THEN** the system SHALL return an error indicating that both VS IDE and Build Tools were checked

#### Scenario: vswhere.exe missing

- **WHEN** `vswhere.exe` is not present at `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`
- **THEN** the system SHALL return an error naming the expected path
- **THEN** the system SHALL NOT fall back to any other detection mechanism

#### Scenario: vswhere.exe execution fails

- **WHEN** `vswhere.exe` is invoked and exits non-zero (or fails to spawn) for a reason other than returning an empty result
- **THEN** the system SHALL propagate the underlying error immediately
- **THEN** the system SHALL NOT retry with attempt 2

---

### Requirement: Version filtering via --vs-version

The system SHALL accept an optional `--vs-version <VERSION>` flag for filtering Visual Studio installations by version. The system SHALL accept friendly version names (`2017`, `2019`, `2022`, `2026`), numeric version names (`15`, `16`, `17`, `18`), and explicit vswhere version ranges (e.g. `[17.0,18.0)`). When a friendly or numeric name is given the system SHALL map it to the corresponding vswhere range as follows: `2017` / `15` → `[15.0,16.0)`; `2019` / `16` → `[16.0,17.0)`; `2022` / `17` → `[17.0,18.0)`; `2026` / `18` → `[18.0,19.0)`. When the flag is omitted the system SHALL pass `-latest` to `vswhere.exe`.

#### Scenario: Friendly version name

- **WHEN** the user invokes the tool with `--vs-version 2022`
- **THEN** the system SHALL invoke `vswhere.exe` with `-version "[17.0,18.0)"`

#### Scenario: Numeric version name

- **WHEN** the user invokes the tool with `--vs-version 17`
- **THEN** the system SHALL invoke `vswhere.exe` with `-version "[17.0,18.0)"`

#### Scenario: Explicit version range

- **WHEN** the user invokes the tool with `--vs-version "[17.0,18.0)"`
- **THEN** the system SHALL invoke `vswhere.exe` with `-version "[17.0,18.0)"` unchanged

#### Scenario: No version flag given

- **WHEN** `--vs-version` is omitted
- **THEN** the system SHALL invoke `vswhere.exe` with the `-latest` flag instead of a version range

---

### Requirement: vsdevcmd INCLUDE capture

The system SHALL invoke `vsdevcmd.bat` from `<installationPath>\Common7\Tools\vsdevcmd.bat` to capture the `INCLUDE` environment variable. The invocation SHALL use the `-arch=x64` flag (only x64 is supported). The system SHALL spawn the batch file via `cmd /c "\"<vsdevcmd-path>\" -arch=x64 >nul 2>&1 && set INCLUDE"` so that the batch file's own diagnostic output is suppressed and only the resulting `INCLUDE=...` line is captured. The system SHALL parse the `INCLUDE=...` line from stdout and SHALL extract the value following the `INCLUDE=` prefix.

#### Scenario: vsdevcmd present and succeeds

- **WHEN** `vsdevcmd.bat` exists at `<installationPath>\Common7\Tools\vsdevcmd.bat` and runs successfully under `cmd.exe`
- **THEN** the system SHALL emit `INCLUDE=<value>` on stdout via `set INCLUDE`
- **THEN** the system SHALL extract `<value>` as the captured INCLUDE string

#### Scenario: vsdevcmd execution fails

- **WHEN** `vsdevcmd.bat` exits non-zero or `cmd.exe` cannot spawn it
- **THEN** the system SHALL return an error that names the discovered VS installation path and the underlying execution error
- **THEN** the system SHALL NOT attempt to fall back to any other detection mechanism

---

### Requirement: INCLUDE value parsing

The system SHALL split the captured `INCLUDE` value on the `;` separator, filter out empty entries, normalize backslashes (`\`) to forward slashes (`/`), and return the result as an ordered vector of strings.

#### Scenario: Multiple paths separated by semicolons

- **WHEN** the captured `INCLUDE` value is `C:\foo;C:\bar\baz;C:\qux`
- **THEN** the system SHALL return `["C:/foo", "C:/bar/baz", "C:/qux"]`

#### Scenario: Trailing or empty entries are filtered

- **WHEN** the captured `INCLUDE` value contains adjacent or trailing `;` (e.g. `C:\foo;;C:\bar;`)
- **THEN** the system SHALL drop the empty entries and return `["C:/foo", "C:/bar"]`

#### Scenario: Backslashes normalized to forward slashes

- **WHEN** any returned path contains a backslash separator
- **THEN** the system SHALL replace every `\` with `/` in the returned string

---

### Requirement: Detailed VS-detection errors

On any Visual Studio detection failure, the system SHALL emit a multi-line error message that begins with `Error: INCLUDE environment variable not set.` and that names the step that failed (vswhere location, vswhere execution, or vsdevcmd execution) along with the relevant paths or underlying error details. When both `vswhere.exe` attempts return empty results the error SHALL indicate that both VS IDE and Build Tools were checked. When a `--vs-version` filter was active the error SHALL include the requested version in the message.

#### Scenario: vswhere.exe not found

- **WHEN** `vswhere.exe` is missing at the expected path
- **THEN** the error SHALL begin with `Error: INCLUDE environment variable not set.`
- **THEN** the error SHALL include `vswhere.exe not found at standard location.`
- **THEN** the error SHALL include the expected path (`C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`)

#### Scenario: No VS IDE or BuildTools found, no version filter

- **WHEN** both vswhere attempts return empty results and `--vs-version` was not specified
- **THEN** the error SHALL include `No Visual Studio or Build Tools installation found`

#### Scenario: No VS IDE or BuildTools found, with version filter

- **WHEN** both vswhere attempts return empty results and `--vs-version 2022` was specified
- **THEN** the error SHALL include `No Visual Studio or Build Tools installation found for version: 2022`

#### Scenario: vsdevcmd execution fails

- **WHEN** a VS installation was discovered but `vsdevcmd.bat` execution failed
- **THEN** the error SHALL name the discovered installation path
- **THEN** the error SHALL include `vsdevcmd.bat execution failed:` followed by the underlying error details

---

### Requirement: Platform: Windows-only

The Visual Studio detection module and the `--vs-version` flag SHALL only be compiled and exposed on Windows builds. The `windows_vs.rs` module SHALL be guarded by `#[cfg(windows)]`. Building or running on Unix-like platforms SHALL be unaffected.

#### Scenario: Windows build

- **WHEN** the crate is built with a Windows target
- **THEN** the `windows_vs` module SHALL be compiled
- **THEN** the `--vs-version` flag SHALL be present in the CLI

#### Scenario: Unix-like build

- **WHEN** the crate is built with a non-Windows target (Linux, macOS, etc.)
- **THEN** the `windows_vs` module SHALL NOT be compiled
- **THEN** the `--vs-version` flag SHALL NOT be present in the CLI

---

### Requirement: Architecture: x64 only

The system SHALL only support the x64 architecture for Visual Studio detection. Every invocation of `vsdevcmd.bat` SHALL pass `-arch=x64`. The system SHALL NOT support `x86`, `ARM64`, or any other target architecture for VS detection.

#### Scenario: vsdevcmd invocation uses -arch=x64

- **WHEN** the system invokes `vsdevcmd.bat`
- **THEN** the command line SHALL include `-arch=x64`

---

### Requirement: Performance budget

VS detection SHOULD complete within 5 seconds under normal conditions. Expected breakdown: environment-variable check < 1 ms; `vswhere.exe` execution ~100–200 ms; `vsdevcmd.bat` execution ~1–2 s. This is a non-binding budget for documentation purposes; the system SHALL NOT abort solely because the budget was exceeded.

#### Scenario: Normal-condition detection completes within budget

- **WHEN** `INCLUDE` is unset, `vswhere.exe` is present, and a VS installation is reachable
- **THEN** detection SHOULD complete within ~5 seconds wall-clock under normal conditions

#### Scenario: Slow environment exceeds budget

- **WHEN** `vswhere.exe` or `vsdevcmd.bat` runs slower than the budget for any reason
- **THEN** the system SHALL still complete the detection rather than aborting on the budget alone

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
