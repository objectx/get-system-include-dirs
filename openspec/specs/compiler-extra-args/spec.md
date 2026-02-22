### Requirement: Compiler extra args passthrough
The tool SHALL accept arbitrary arguments after a `--` separator on the CLI and forward them verbatim to the gcc-like compiler invocation.

#### Scenario: Extra args forwarded to compiler
- **WHEN** the user invokes `get-system-include-dirs --compiler <path> -- <args...>`
- **THEN** the compiler SHALL be invoked as `<path> -v -E -x c++ <args...> -`

#### Scenario: No extra args (baseline)
- **WHEN** the user invokes `get-system-include-dirs --compiler <path>` with no `--` separator
- **THEN** the compiler SHALL be invoked as `<path> -v -E -x c++ -` unchanged

### Requirement: Extra args require explicit compiler
Extra args SHALL only be applied when `--compiler` is explicitly specified and the compiler is non-MSVC-like.

#### Scenario: Extra args without --compiler
- **WHEN** extra args are provided but no `--compiler` flag is given
- **THEN** the tool SHALL emit a warning to stderr: "compiler args ignored — no --compiler specified"
- **THEN** the tool SHALL continue and return results using the default compiler path

#### Scenario: Extra args with MSVC-like compiler on Windows
- **WHEN** extra args are provided and `--compiler` names an MSVC-like compiler (`cl`, `cl.exe`, `clang-cl`, `clang-cl.exe`)
- **THEN** the tool SHALL emit a warning to stderr: "compiler args ignored for MSVC-like compilers"
- **THEN** the tool SHALL continue and return results via the VS detection path

### Requirement: Extra args position in command
Extra args SHALL be appended after the fixed preprocessing flags and before the stdin sentinel.

#### Scenario: Correct arg ordering
- **WHEN** extra args `["--target", "aarch64-linux-android21"]` are provided
- **THEN** the full invocation SHALL be `compiler -v -E -x c++ --target aarch64-linux-android21 -`
