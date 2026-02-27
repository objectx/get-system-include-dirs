## Architecture

The new `build-windows` job follows the same pattern as `build-ubuntu` — a standalone job that `needs: [create-release]` and runs in parallel with the other build jobs.

```
release.yml triggers on v* tags
         │
         ▼
┌─────────────────┐
│  create-release  │
└────────┬────────┘
         │
    ┌────┼─────────────────┬──────────────────┐
    ▼    ▼                 ▼                  ▼
┌──────┐ ┌──────────┐ ┌──────────┐ ┌───────────┐
│macOS │ │  Ubuntu  │ │ Windows  │ │  (future) │
│matrix│ │ x86_64   │ │ x86_64   │ │           │
└──────┘ └──────────┘ └──────────┘ └───────────┘
```

## Shell Strategy

- **Bash** for build, rename, and upload steps (consistent with macOS/Ubuntu jobs)
- **PowerShell** for the smoke test only (natural fit for invoking a `.exe` and checking results on Windows)

## Smoke Test Design

The smoke test invokes the built binary with no arguments on a `windows-2025` runner, which has Visual Studio pre-installed. This exercises the full VS auto-detection path:

1. `$INCLUDE` is not set in a plain shell → falls through to vswhere
2. `find_vswhere()` locates `vswhere.exe` at the standard path
3. `query_vswhere()` finds the runner's VS installation (IDE first, then BuildTools fallback)
4. `run_vsdevcmd_and_capture_include()` runs `vsdevcmd.bat -arch=x64`
5. INCLUDE is parsed and printed to stdout

The test asserts:
- Exit code is 0
- Output is non-empty (at least one include dir was found)

Expected overhead: ~10-20 seconds for `vsdevcmd.bat` execution, acceptable for release CI.

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Runner | `windows-2025` | User preference; current-gen Windows runner |
| Target | `x86_64-pc-windows-msvc` only | ARM64 deferred |
| Job structure | Standalone (not matrix) | Matches `build-ubuntu` pattern, keeps per-platform differences clear |
| Asset naming | `...-x86_64-pc-windows-msvc.exe` | Preserves `.exe` so the download is directly executable |
| Shell | Bash default, PowerShell for smoke test | Bash for consistency with other jobs; PowerShell is natural for Windows-specific testing |
