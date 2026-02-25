## MODIFIED Requirements

### Requirement: FR-2: VS Detection via vswhere
The system MUST use `vswhere.exe` to locate Visual Studio or Build Tools installations, using a two-attempt strategy to give VS IDE priority while supporting BuildTools-only environments.

**Acceptance Criteria**:
- Check for vswhere at: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`
- If not found, return error (no fallback)
- **Attempt 1**: Execute vswhere with `-format json -utf8` (no `-products` flag) to find VS IDE installations
- If attempt 1 returns results, use the first result and stop
- **Attempt 2** (only if attempt 1 returned empty results): Execute vswhere with `-products Microsoft.VisualStudio.Product.BuildTools -format json -utf8` to find BuildTools installations
- If attempt 2 returns results, use the first result
- If both attempts return empty results, return error indicating neither VS IDE nor Build Tools was found
- If vswhere execution itself fails (non-empty-result failure), propagate the error immediately without retrying

#### Scenario: VS IDE installed
- **WHEN** a VS IDE installation exists (Enterprise, Professional, or Community)
- **THEN** attempt 1 returns the IDE installation and attempt 2 is never executed

#### Scenario: BuildTools only installed
- **WHEN** no VS IDE installation exists but `Microsoft.VisualStudio.Product.BuildTools` is installed
- **THEN** attempt 1 returns empty results and attempt 2 returns the BuildTools installation

#### Scenario: Both VS IDE and BuildTools installed
- **WHEN** both a VS IDE installation and a BuildTools installation exist
- **THEN** attempt 1 returns the VS IDE installation; attempt 2 is never executed (VS IDE takes priority)

#### Scenario: Neither VS IDE nor BuildTools installed
- **WHEN** no VS IDE or BuildTools installation exists
- **THEN** both attempts return empty results and an error is returned

### Requirement: FR-6: Error Reporting
The system MUST provide detailed error messages showing what was attempted.

**Acceptance Criteria**:
- Show that INCLUDE was not set
- Show which step failed: vswhere location, vswhere execution, vsdevcmd execution
- Include relevant paths and error details
- Format: Multi-line with context
- When both vswhere attempts return empty results, the error MUST indicate that both VS IDE and Build Tools were checked

**Error Message Examples**:

vswhere not found:
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: vswhere.exe not found at standard location.
Expected: C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe
```

No VS IDE or BuildTools found (no version filter):
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: No Visual Studio or Build Tools installation found
```

No VS IDE or BuildTools found (with version filter):
```
Error: INCLUDE environment variable not set.
Tried to find Visual Studio: No Visual Studio or Build Tools installation found for version: 2022
```

vsdevcmd failed:
```
Error: INCLUDE environment variable not set.
Found VS at: C:\tools\MSVS\17\Professional
vsdevcmd.bat execution failed: <error details>
```

#### Scenario: Error when neither is found without version filter
- **WHEN** both vswhere attempts return empty results and no `--vs-version` was specified
- **THEN** error message reads: `"No Visual Studio or Build Tools installation found"`

#### Scenario: Error when neither is found with version filter
- **WHEN** both vswhere attempts return empty results and `--vs-version 2022` was specified
- **THEN** error message reads: `"No Visual Studio or Build Tools installation found for version: 2022"`
