## 1. Implement BuildTools Fallback in query_vswhere

- [x] 1.1 Extract the current vswhere result-parsing logic into a reusable helper (or inline the retry) within `query_vswhere` in `src/windows_vs.rs`
- [x] 1.2 After attempt 1 returns an empty result set, execute a second vswhere call with `-products Microsoft.VisualStudio.Product.BuildTools` and the same version flags
- [x] 1.3 Update the "no installation found" error messages to read "No Visual Studio or Build Tools installation found [for version: X]"

## 2. Verify Behavior

- [x] 2.1 Confirm on the CI BuildTools-only machine that the tool now returns include dirs successfully
- [x] 2.2 Confirm on a machine with VS IDE installed that behavior is unchanged (attempt 1 still wins, no extra vswhere call)
