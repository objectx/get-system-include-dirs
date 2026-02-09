# Tasks: Add Visual Studio Fallback for INCLUDE

## Overview

Implementation tasks for automatic Visual Studio detection when INCLUDE environment variable is not set on Windows.

## Task List

### Task 1: Update Cargo.toml Dependencies
**Status**: pending
**Estimate**: 5 minutes

Add serde and serde_json dependencies.

**Changes**:
- File: [Cargo.toml](../../../Cargo.toml)
- Add to `[dependencies]` section:
  ```toml
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  ```

**Verification**:
```bash
cargo build
```

---

### Task 2: Create src/windows_vs.rs Module
**Status**: pending
**Estimate**: 45 minutes

Create new module with VS detection and INCLUDE extraction logic.

**Changes**:
- File: [src/windows_vs.rs](../../../src/windows_vs.rs) (new file)

**Implementation**:

1. **Add module header and imports**:
```rust
// SPDX-License-Identifier: WTFPL
//! Windows Visual Studio detection and INCLUDE environment variable extraction.

use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::process::Command;
```

2. **Define VsInstance struct**:
```rust
#[derive(Deserialize)]
struct VsInstance {
    #[serde(rename = "installationPath")]
    installation_path: String,

    #[serde(rename = "installationVersion")]
    installation_version: String,
}
```

3. **Implement public API**:
```rust
pub fn get_windows_include_dirs_with_fallback(
    vs_version: Option<&str>
) -> Result<Vec<String>, String>
```
- Check `INCLUDE` env var first (highest priority)
- If not set, call `find_vs_and_get_include()`

4. **Implement helper functions**:
- `parse_include_env()` - Parse semicolon-separated paths
- `find_vs_and_get_include()` - Orchestrate VS detection
- `find_vswhere()` - Locate vswhere.exe
- `query_vswhere()` - Execute vswhere and parse JSON
- `map_version_to_range()` - Convert version strings to ranges
- `run_vsdevcmd_and_capture_include()` - Execute vsdevcmd.bat

**Verification**:
```bash
cargo build
# Should compile without errors
```

**Reference**: See [design.md](design.md) Section "Component Design" for detailed function signatures.

---

### Task 3: Update main.rs - Add Module Import
**Status**: pending
**Estimate**: 5 minutes

Add conditional compilation for windows_vs module.

**Changes**:
- File: [src/main.rs](../../../src/main.rs)
- Add after existing imports:
```rust
#[cfg(windows)]
mod windows_vs;
```

**Verification**:
```bash
cargo build
```

---

### Task 4: Update main.rs - Add --vs-version Flag
**Status**: pending
**Estimate**: 10 minutes

Add new CLI argument for VS version filtering.

**Changes**:
- File: [src/main.rs](../../../src/main.rs)
- Modify `Args` struct (around line 21-32):

```rust
#[derive(Parser, Debug)]
#[command(name = "get-system-include-dirs")]
#[command(about = "Extract system include directories from C++ compiler", long_about = None)]
struct Args {
    /// Path to the C++ compiler to query
    #[arg(short, long)]
    compiler: Option<PathBuf>,

    /// Output file path (use '-' for stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Visual Studio version (e.g., "2022", "2026", "17", "18")
    /// Only used on Windows when no compiler is specified
    #[cfg(windows)]
    #[arg(long)]
    vs_version: Option<String>,
}
```

**Verification**:
```bash
cargo build
get-system-include-dirs --help
# Should show --vs-version flag on Windows
```

---

### Task 5: Update main.rs - Modify get_include_dirs Signature
**Status**: pending
**Estimate**: 10 minutes

Add vs_version parameter to get_include_dirs function.

**Changes**:
- File: [src/main.rs](../../../src/main.rs)
- Update function signature (around line 96):

```rust
fn get_include_dirs(
    compiler: Option<PathBuf>,
    #[cfg(windows)] vs_version: Option<String>
) -> Result<Vec<String>, String>
```

**Verification**:
```bash
cargo build
# Will show compilation errors until Task 6 is complete (expected)
```

---

### Task 6: Update main.rs - Integrate Windows VS Detection
**Status**: pending
**Estimate**: 15 minutes

Replace existing Windows $INCLUDE parsing with new fallback logic.

**Changes**:
- File: [src/main.rs](../../../src/main.rs)
- Modify `get_include_dirs()` function (around lines 96-117):

**Replace**:
```rust
if cfg!(windows) && compiler.is_none() {
    // On Windows without a specified compiler, parse $INCLUDE
    return get_windows_include_dirs();
}
```

**With**:
```rust
#[cfg(windows)]
if compiler.is_none() {
    // On Windows without a specified compiler, use $INCLUDE or auto-detect VS
    return windows_vs::get_windows_include_dirs_with_fallback(
        vs_version.as_deref()
    );
}
```

**Also remove**:
- Function `get_windows_include_dirs()` (around lines 149-161) - no longer needed
- The check for MSVC-like compiler (around lines 111-114) - handled by new module

**Verification**:
```bash
cargo build
# Should compile successfully
```

---

### Task 7: Update main.rs - Thread vs_version Through main()
**Status**: pending
**Estimate**: 5 minutes

Pass vs_version argument from main() to get_include_dirs().

**Changes**:
- File: [src/main.rs](../../../src/main.rs)
- Update `main()` function (around line 37):

```rust
match get_include_dirs(
    args.compiler,
    #[cfg(windows)] args.vs_version
) {
    Ok(dirs) => {
        // ... existing code
    }
    Err(e) => {
        // ... existing code
    }
}
```

**Verification**:
```bash
cargo build
# Should compile successfully without warnings
```

---

### Task 8: Manual Testing - Basic Functionality
**Status**: pending
**Estimate**: 15 minutes

Test basic VS detection scenarios.

**Test Cases**:

1. **TC-1**: INCLUDE already set
```bash
# Set INCLUDE manually
set INCLUDE=C:\test\path
get-system-include-dirs
# Expected: C:/test/path
```

2. **TC-2**: INCLUDE not set, auto-detect latest
```bash
# Unset INCLUDE
set INCLUDE=
get-system-include-dirs
# Expected: List of include directories from latest VS
```

3. **TC-3**: Specific VS version
```bash
set INCLUDE=
get-system-include-dirs --vs-version 2022
# Expected: List of include directories from VS 2022
```

**Verification**:
- All test cases produce expected output
- No panics or crashes
- Error messages are clear and helpful

---

### Task 9: Manual Testing - Error Scenarios
**Status**: pending
**Estimate**: 10 minutes

Test error handling and edge cases.

**Test Cases**:

1. **TC-4**: Invalid version
```bash
set INCLUDE=
get-system-include-dirs --vs-version 9999
# Expected: Error message about version not found
```

2. **TC-5**: Output to file
```bash
set INCLUDE=
get-system-include-dirs --output includes.txt
# Expected: File created with include directories
```

3. **TC-6**: Help message
```bash
get-system-include-dirs --help
# Expected: Shows --vs-version flag (Windows only)
```

**Verification**:
- Error messages are detailed and helpful
- File output works correctly
- Help text is clear

---

### Task 10: Manual Testing - Precedence Verification
**Status**: pending
**Estimate**: 5 minutes

Verify INCLUDE environment variable takes precedence.

**Test Cases**:

1. **TC-7**: INCLUDE set with --vs-version flag
```bash
set INCLUDE=C:\custom\path
get-system-include-dirs --vs-version 2022
# Expected: C:/custom/path (INCLUDE takes precedence)
```

**Verification**:
- INCLUDE is always used when set, regardless of CLI flags

---

## Implementation Order

Tasks should be implemented in numerical order:
1. Dependencies (Task 1)
2. Core module (Task 2)
3. Integration (Tasks 3-7)
4. Testing (Tasks 8-10)

## Notes

- All code should follow existing style and conventions in the project
- Use SPDX license identifier: `// SPDX-License-Identifier: WTFPL`
- Maintain existing documentation quality (doc comments)
- Error messages should be detailed but concise
- Platform-specific code should use `#[cfg(windows)]` guards

## Success Criteria

✅ All tasks completed
✅ Code compiles without warnings
✅ All manual test cases pass
✅ Error messages are clear and actionable
✅ No regression in existing functionality (Unix/compiler paths)
✅ Windows-only features don't affect other platforms
