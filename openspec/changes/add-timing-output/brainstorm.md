## Design Summary

Add an opt-in `--timing` flag that emits a single JSON line on stderr describing how long the run took, broken into uniform high-level phases (`discover_ms`, `parse_ms`, `write_ms`, `elapsed_ms`). The schema is identical across all three execution paths (gcc-like compiler, Windows `$INCLUDE`, Windows VS auto-detect) so log scrapers in CI can rely on a stable shape. Timing is also emitted on failure (with whichever phases completed plus an `error` field) so failed runs can be attributed in build dashboards.

## Alternatives Considered

### Option A: Inline `Instant::now()` timers at phase boundaries with a `Timings` struct serialized via `serde_json`

- **Approach**: Capture `Instant` at the start of each phase boundary inside `get_compiler_include_dirs`, `windows_vs::get_windows_include_dirs_with_fallback`, and around `write_output`. Phase functions return `(payload, Timings)`. `main` aggregates and serializes to stderr.
- **Pros**: Zero new dependencies (`serde`/`serde_json` already in `Cargo.toml`). Minimal change footprint — three call sites and one new struct. Works identically across all three platform paths. Always-on measurement is ~20ns and avoids per-phase flag branching.
- **Cons**: Touches the return signatures of two existing functions. Error-path timing requires a small amount of bookkeeping in `main`.
- **Why chosen**: Simplest change that preserves the current style of the codebase. No abstraction overhead.

### Option B: `tracing` crate with spans + a custom subscriber that emits JSON

- **Approach**: Wrap each phase in `tracing::info_span!`. Install a subscriber under `--timing` that aggregates span durations and emits one JSON line on shutdown.
- **Pros**: Future-proof for richer observability (nested spans, multiple sinks). Idiomatic for larger Rust services.
- **Cons**: Adds two new dependencies (`tracing`, `tracing-subscriber`) plus their transitive surface. Significant binary-size and compile-time impact for a tool with `opt-level = "z"` and `lto = true`. Overkill for one timer per phase.
- **Why not chosen**: Architecture cost dwarfs the value for a ~250-line CLI. YAGNI.

### Option C: Hand-formatted JSON string concatenation

- **Approach**: Avoid `serde_json` and assemble the JSON line with `format!` directly.
- **Pros**: Marginally smaller compile footprint if `serde_json` were not already a dep.
- **Cons**: Manual escaping of the `error` field is error-prone (quotes, backslashes, control characters in compiler stderr). `serde_json` is already a transitive cost via `serde`.
- **Why not chosen**: No real saving when `serde_json` is already in the dependency tree.

## Agreed Approach

**Option A** — inline `Instant` timers and a `Timings` struct serialized with `serde_json`. Justification: smallest-impact change, no new dependencies, uniform schema across all three code paths, and trivial to test with the project's existing fixture-driven style.

## Key Decisions

- **Activation**: opt-in `--timing` flag (no short form). Default off → backward-compatible.
- **Output channel**: stderr, regardless of `-o/--output` (which controls the include-dirs payload only).
- **Format**: single-line JSON `{"timing":{...}}` for stable CI log scraping.
- **Schema (uniform across all paths)**: `discover_ms`, `parse_ms`, `write_ms`, `elapsed_ms`. On failure: only completed phases plus `elapsed_ms` and `error`.
- **Phase mapping**:
  - gcc-like: subprocess call → `discover_ms`; stderr decode + `parse_include_dirs` → `parse_ms`; output writer → `write_ms`.
  - Windows `$INCLUDE`: env read → `discover_ms`; semicolon split → `parse_ms`; output writer → `write_ms`.
  - Windows VS auto-detect: `vswhere` + `vsdevcmd` → `discover_ms` (combined); INCLUDE extraction → `parse_ms`; output writer → `write_ms`.
- **Always-measure policy**: `Instant` is captured unconditionally; the `Timings` struct is discarded in `main` when `--timing` is off. Avoids per-phase flag branching.
- **Implementation locus**: a new `src/timing.rs` module hosting `Timings` and a small `PhaseTimer` helper. Existing functions return `(Vec<String>, Timings)`.

## Open Questions

None blocking. Cosmetic decisions (e.g., field ordering in JSON output) follow `serde` defaults.
