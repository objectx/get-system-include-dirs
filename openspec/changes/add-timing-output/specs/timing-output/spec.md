## ADDED Requirements

### Requirement: Opt-in `--timing` flag

The CLI SHALL accept an opt-in `--timing` flag (long form only, no short alias). When the flag is absent, no timing information SHALL be emitted and the tool SHALL behave identically to prior versions.

#### Scenario: --timing flag absent (baseline)

- **WHEN** the user invokes `get-system-include-dirs` without `--timing`
- **THEN** stderr SHALL NOT contain any timing JSON line
- **THEN** the tool SHALL produce its include-dirs output and exit code unchanged from prior behavior

#### Scenario: --timing flag present on success

- **WHEN** the user invokes `get-system-include-dirs --timing` and the run succeeds
- **THEN** exactly one JSON line of the form `{"timing":{...}}` SHALL be written to stderr
- **THEN** the include-dirs payload SHALL be written to its configured destination (stdout or `-o`-specified file) unchanged
- **THEN** the tool SHALL exit 0

### Requirement: Timing output channel

The timing JSON line SHALL always be written to stderr, regardless of the value of `-o/--output`. The `-o/--output` flag SHALL continue to control only the destination of the include-dirs payload.

#### Scenario: --timing combined with --output to a file

- **WHEN** the user invokes `get-system-include-dirs --timing --output result.txt`
- **THEN** the include-dirs payload SHALL be written to `result.txt`
- **THEN** the timing JSON line SHALL be written to stderr (not to `result.txt`)

#### Scenario: --timing combined with --output -

- **WHEN** the user invokes `get-system-include-dirs --timing --output -`
- **THEN** the include-dirs payload SHALL be written to stdout
- **THEN** the timing JSON line SHALL be written to stderr

### Requirement: Uniform timing schema across execution paths

The timing JSON object SHALL use a uniform schema across all three execution paths (gcc-like compiler, Windows `$INCLUDE`, Windows VS auto-detect). On success, the object SHALL contain exactly the keys `discover_ms`, `parse_ms`, `write_ms`, and `elapsed_ms`, each holding a non-negative integer count of milliseconds.

#### Scenario: Successful gcc-like run

- **WHEN** `--timing` is set and the gcc-like compiler path produces include directories successfully
- **THEN** stderr SHALL contain a JSON line `{"timing":{"discover_ms":<n>,"parse_ms":<n>,"write_ms":<n>,"elapsed_ms":<n>}}`
- **THEN** `discover_ms` SHALL reflect the wall-clock duration of the compiler subprocess invocation
- **THEN** `parse_ms` SHALL reflect the wall-clock duration of decoding stderr and parsing include directories from it

#### Scenario: Successful Windows `$INCLUDE` run

- **WHEN** `--timing` is set on Windows, `INCLUDE` is set in the environment, and the run succeeds
- **THEN** stderr SHALL contain a JSON line `{"timing":{"discover_ms":<n>,"parse_ms":<n>,"write_ms":<n>,"elapsed_ms":<n>}}`
- **THEN** `discover_ms` SHALL reflect the wall-clock duration of reading the `INCLUDE` environment variable
- **THEN** `parse_ms` SHALL reflect the wall-clock duration of splitting and normalizing the semicolon-separated paths

#### Scenario: Successful Windows VS auto-detect run

- **WHEN** `--timing` is set on Windows, `INCLUDE` is not set, and VS auto-detection succeeds via `vswhere.exe` and `vsdevcmd.bat`
- **THEN** stderr SHALL contain a JSON line `{"timing":{"discover_ms":<n>,"parse_ms":<n>,"write_ms":<n>,"elapsed_ms":<n>}}`
- **THEN** `discover_ms` SHALL reflect the combined wall-clock duration of `vswhere.exe` plus `vsdevcmd.bat`
- **THEN** `parse_ms` SHALL reflect the wall-clock duration of extracting `INCLUDE=` from `vsdevcmd.bat` output and splitting/normalizing it

### Requirement: Timing emission on failure

When `--timing` is set and the run fails (non-zero exit), the tool SHALL emit a partial timing JSON line on stderr **before** the existing `Error: ...` line. The partial object SHALL include `elapsed_ms`, an `error` string, and any phase keys whose phases were entered (i.e., the phase timer started). A phase that ran to a failure result counts as entered and SHALL have its key present, recording the time spent attempting it. Phase keys for phases that were never entered SHALL be omitted.

#### Scenario: Compiler subprocess fails

- **WHEN** `--timing` is set and the gcc-like compiler subprocess returns non-zero or fails to spawn
- **THEN** stderr SHALL contain a JSON line of the form `{"timing":{...,"elapsed_ms":<n>,"error":"<message>"}}` written before the `Error: ...` line
- **THEN** the `discover_ms` key MAY be present (if the subprocess started); the `parse_ms` and `write_ms` keys SHALL be absent
- **THEN** the tool SHALL exit non-zero

#### Scenario: Parse stage fails (no include directives found)

- **WHEN** `--timing` is set and the compiler ran successfully but its output contained no recognizable include directives
- **THEN** stderr SHALL contain a JSON line containing `discover_ms`, `parse_ms` (the time spent attempting to parse), `elapsed_ms`, and `error`
- **THEN** the `write_ms` key SHALL be absent
- **THEN** the tool SHALL exit non-zero

#### Scenario: Windows VS auto-detection fails

- **WHEN** `--timing` is set on Windows, `INCLUDE` is not set, and `vswhere.exe` is missing or VS detection fails
- **THEN** stderr SHALL contain a JSON line containing `discover_ms` (time spent attempting detection), `elapsed_ms`, and `error`
- **THEN** the tool SHALL exit non-zero

#### Scenario: Output write fails

- **WHEN** `--timing` is set, the include-dirs payload was produced successfully, but writing the output (to `stdout` or to `--output <file>`) fails
- **THEN** stderr SHALL contain a JSON line containing `discover_ms`, `parse_ms`, `write_ms` (the time spent attempting to write), `elapsed_ms`, and `error`
- **THEN** the tool SHALL exit non-zero

### Requirement: Timing JSON format

The timing line SHALL be a single line of valid JSON with the top-level shape `{"timing": <object>}`. Numeric values SHALL be JSON numbers (no quotes, no trailing units). The `error` value, when present, SHALL be a JSON-escaped string suitable for any UTF-8 message including embedded quotes, backslashes, or newlines.

#### Scenario: Single-line output

- **WHEN** `--timing` is set
- **THEN** the timing output SHALL occupy exactly one line on stderr (terminated by a single `\n`)
- **THEN** the line SHALL be parseable by a standard JSON parser without modification

#### Scenario: Error string with special characters

- **WHEN** `--timing` is set, the run fails, and the error message contains characters such as `"`, `\`, or newline
- **THEN** the `error` value in the JSON SHALL be properly JSON-escaped so the line remains valid JSON
