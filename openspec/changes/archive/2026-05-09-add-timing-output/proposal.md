## Why

CI pipelines that invoke this tool as a build step want to attribute time to it without grafting external timing wrappers. Today there is no way to know how long the run took, let alone which phase (compiler subprocess, parsing, output write) dominated. Adding a structured, opt-in timing line lets dashboards scrape duration from logs uniformly across gcc-like and Windows VS code paths, and supports debugging slow runs (e.g., a stale `vsdevcmd.bat` taking seconds) without changing default output behavior.

## What Changes

**CLI surface**
- From: `get-system-include-dirs [--compiler ...] [--output ...] [-- <args>]`
- To: `get-system-include-dirs [--compiler ...] [--output ...] [--timing] [-- <args>]`
- Reason: enable opt-in timing output for CI observability
- Impact: non-breaking — existing invocations without `--timing` behave identically

**Output behavior under `--timing`**
- A single JSON line `{"timing":{...}}` SHALL be written to stderr after the run, regardless of the include-dirs payload destination set by `-o/--output`
- On success: keys `discover_ms`, `parse_ms`, `write_ms`, `elapsed_ms` are all present
- On failure: only the phases that completed are present, plus `elapsed_ms` and an `error` string; the JSON line is written before the existing `Error: ...` line; the process still exits non-zero

**Schema is uniform across all three execution paths**
- gcc-like: `discover_ms` covers the compiler subprocess; `parse_ms` covers stderr decode + `parse_include_dirs`
- Windows `$INCLUDE`: `discover_ms` covers env read; `parse_ms` covers semicolon split
- Windows VS auto-detect: `discover_ms` covers `vswhere` + `vsdevcmd` combined; `parse_ms` covers extracting INCLUDE
- Reason: stable schema lets log scrapers treat all paths identically

## Capabilities

### New Capabilities

- `timing-output`: opt-in CLI flag that emits a JSON line on stderr describing run duration broken into uniform high-level phases, on both success and failure paths

### Modified Capabilities

<!-- No existing spec-level requirements are changing -->

## Impact

- `src/main.rs`: `Args` adds `--timing`; `main` orchestrates timing capture and stderr emission; `get_include_dirs` / `get_compiler_include_dirs` signatures change to return phase timings
- `src/windows_vs.rs`: `get_windows_include_dirs_with_fallback` signature changes to return phase timings
- `src/timing.rs` (new): `Timings` struct (Serialize) and a small `PhaseTimer` helper
- No new dependencies (`serde`, `serde_json` already in `Cargo.toml`)
- No breaking changes to default CLI behavior or stdout payload format

## Non-goals

- A generic `--verbose` framework or multi-level diagnostics
- Sub-phase breakdowns (e.g., separating `vswhere` from `vsdevcmd` on Windows)
- Machine-readable timing in formats other than JSON
- Wall-clock timing of pre-CLI work (process spawn, dynamic loader, etc.)
