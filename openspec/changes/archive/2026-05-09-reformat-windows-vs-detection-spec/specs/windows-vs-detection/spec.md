## REMOVED Requirements

### Requirement: Functional Requirements

**Reason**: Parser-visible artifact of the FR-N grouping. The baseline grouped six functional requirements under a single `### Functional Requirements` heading, which OpenSpec's strict validator parsed as a single top-level requirement whose statement contained no inline `SHALL` / `MUST` keyword. The grouping is replaced by nine flat `### Requirement: <name>` blocks (six functional + three non-functional) at H3, each with its statement containing `SHALL` or `MUST` inline.

**Migration**: The six functional requirements that were nested under this heading become top-level requirements (see ADDED Requirements below): `$INCLUDE precedence`, `vswhere two-attempt detection`, `Version filtering via --vs-version`, `vsdevcmd INCLUDE capture`, `INCLUDE value parsing`, `Detailed VS-detection errors`. No semantic change.

### Requirement: Non-Functional Requirements

**Reason**: Same as `### Requirement: Functional Requirements` — parser-visible artifact of the NFR-N grouping. Replaced by flat `### Requirement: <name>` blocks alongside the functional ones.

**Migration**: The three non-functional requirements that were nested under this heading become top-level requirements (see ADDED Requirements below): `Platform: Windows-only`, `Architecture: x64 only`, `Performance budget`. No semantic change.

---

## ADDED Requirements

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
