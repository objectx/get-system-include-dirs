## Why

On CI machines with Visual Studio Build Tools installed (but no full VS IDE), the tool fails with "No Visual Studio installation found" because `vswhere.exe` by default only searches IDE product types, silently skipping `Microsoft.VisualStudio.Product.BuildTools`.

## What Changes

- `query_vswhere` gains a two-attempt strategy: first query uses the current default (IDE products only); if that returns no results, a second query retries with `-products Microsoft.VisualStudio.Product.BuildTools`
- Error message updated to mention both VS IDE and Build Tools were checked when both attempts fail

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `windows-vs-detection`: `query_vswhere` now falls back to a BuildTools-targeted query when the standard query returns no installations

## Impact

- `src/windows_vs.rs` — `query_vswhere` function only
- No CLI changes, no new flags, no breaking changes
- Full VS IDE still takes priority; BuildTools is only used when no IDE installation is found

## Non-goals

- Support for `-products *` (wildcard) — using the explicit product ID instead
- Preference policy when both VS IDE and BuildTools exist at the same version (first attempt wins = IDE wins)
- Registry-based detection, x86/ARM64 architecture support, workload filtering
