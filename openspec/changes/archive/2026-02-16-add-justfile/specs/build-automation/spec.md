## ADDED Requirements

### Requirement: Build release artifact for a specific target
The system SHALL provide a `build` recipe that accepts a target triple and produces a release binary at `dist/<target>/get-system-include-dirs`.

#### Scenario: Build for macOS x86_64
- **WHEN** user runs `just build x86_64-apple-darwin`
- **THEN** the system builds a release binary using `cargo build --release --target x86_64-apple-darwin`
- **AND** copies the binary to `dist/x86_64-apple-darwin/get-system-include-dirs`

#### Scenario: Build for macOS arm64
- **WHEN** user runs `just build aarch64-apple-darwin`
- **THEN** the system builds a release binary using `cargo build --release --target aarch64-apple-darwin`
- **AND** copies the binary to `dist/aarch64-apple-darwin/get-system-include-dirs`

#### Scenario: Build for Linux using cross
- **WHEN** user runs `just build x86_64-unknown-linux-gnu`
- **THEN** the system builds a release binary using `cross build --release --target x86_64-unknown-linux-gnu`
- **AND** copies the binary to `dist/x86_64-unknown-linux-gnu/get-system-include-dirs`

#### Scenario: Build for Windows
- **WHEN** user runs `just build x86_64-pc-windows-msvc` on a Windows machine
- **THEN** the system builds a release binary using `cargo build --release --target x86_64-pc-windows-msvc`
- **AND** copies the binary to `dist/x86_64-pc-windows-msvc/get-system-include-dirs.exe`

### Requirement: Tool selection based on target
The system SHALL use `cross` as the build tool when the target triple contains `linux`, and `cargo` for all other targets.

#### Scenario: Linux target uses cross
- **WHEN** the target triple contains `linux`
- **THEN** the build tool SHALL be `cross`

#### Scenario: Non-Linux target uses cargo
- **WHEN** the target triple does not contain `linux`
- **THEN** the build tool SHALL be `cargo`

### Requirement: Windows binary suffix
The system SHALL append `.exe` to the binary name when the target triple contains `windows`.

#### Scenario: Windows target gets .exe suffix
- **WHEN** the target triple contains `windows`
- **THEN** the artifact filename SHALL be `get-system-include-dirs.exe`

#### Scenario: Non-Windows target has no suffix
- **WHEN** the target triple does not contain `windows`
- **THEN** the artifact filename SHALL be `get-system-include-dirs`

### Requirement: Build all cross-compilable targets
The system SHALL provide a `build-all` recipe that builds release artifacts for x86_64-apple-darwin, aarch64-apple-darwin, and x86_64-unknown-linux-gnu.

#### Scenario: Build all targets
- **WHEN** user runs `just build-all`
- **THEN** the system builds release artifacts for all three cross-compilable targets
- **AND** places each artifact in its respective `dist/<target>/` directory

#### Scenario: build-all excludes Windows
- **WHEN** user runs `just build-all`
- **THEN** the system SHALL NOT attempt to build for x86_64-pc-windows-msvc

### Requirement: Windows build convenience recipe
The system SHALL provide a `build-windows` recipe as a shorthand for building the Windows MSVC target.

#### Scenario: Build Windows target
- **WHEN** user runs `just build-windows`
- **THEN** the system builds for `x86_64-pc-windows-msvc` using `cargo`

### Requirement: Binary name from Cargo.toml
The system SHALL derive the binary name from the project's Cargo.toml rather than hardcoding it.

#### Scenario: Binary name matches package name
- **WHEN** any build recipe runs
- **THEN** the artifact binary name SHALL match the `name` field in Cargo.toml

### Requirement: Lint check
The system SHALL provide a `check` recipe that runs `cargo clippy` with default lints.

#### Scenario: Run clippy
- **WHEN** user runs `just check`
- **THEN** the system executes `cargo clippy`

### Requirement: Format code
The system SHALL provide a `fmt` recipe that runs `cargo fmt`.

#### Scenario: Run formatter
- **WHEN** user runs `just fmt`
- **THEN** the system executes `cargo fmt`

### Requirement: Run tests
The system SHALL provide a `test` recipe that runs `cargo test`.

#### Scenario: Run tests
- **WHEN** user runs `just test`
- **THEN** the system executes `cargo test`

### Requirement: Clean artifacts
The system SHALL provide a `clean` recipe that removes both the Cargo build directory and the `dist/` directory.

#### Scenario: Clean all build artifacts
- **WHEN** user runs `just clean`
- **THEN** the system executes `cargo clean`
- **AND** removes the `dist/` directory

### Requirement: Clean artifacts for a specific target
The system SHALL provide a hidden `_clean` recipe that removes only the build and dist artifacts for a single target, without affecting other targets.

#### Scenario: Clean a specific target
- **WHEN** `_clean` is invoked with a target triple
- **THEN** the system removes `target/<target>/release/` and `dist/<target>/`
- **AND** does not affect artifacts for other targets

#### Scenario: Hidden from recipe list
- **WHEN** user runs `just --list`
- **THEN** the `_clean` recipe SHALL NOT appear in the output

### Requirement: Rebuild a specific target
The system SHALL provide a `rebuild` recipe that cleans and rebuilds a single target without affecting other targets.

#### Scenario: Rebuild one target
- **WHEN** user runs `just rebuild x86_64-apple-darwin`
- **THEN** the system removes only `x86_64-apple-darwin` build and dist artifacts
- **AND** builds a fresh release binary for `x86_64-apple-darwin`

### Requirement: Rebuild all cross-compilable targets
The system SHALL provide a `rebuild-all` recipe that fully cleans and rebuilds all cross-compilable targets.

#### Scenario: Rebuild all targets
- **WHEN** user runs `just rebuild-all`
- **THEN** the system runs a full `clean` (all targets and dist)
- **AND** builds release artifacts for all three cross-compilable targets

### Requirement: Gitignore dist directory
The `.gitignore` file SHALL include an entry to exclude the `dist/` directory from version control.

#### Scenario: dist directory is ignored
- **WHEN** build artifacts are placed in `dist/`
- **THEN** git SHALL not track the `dist/` directory
