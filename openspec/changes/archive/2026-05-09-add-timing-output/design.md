## Context

`get-system-include-dirs` is a small Rust CLI (~250 lines across `src/main.rs` and `src/windows_vs.rs`) that queries a C++ compiler or a Windows VS installation for system include directories. There is currently no observability surface: a run either prints include directories to stdout (or a file) and exits 0, or prints an error to stderr and exits non-zero. CI consumers have no way to attribute build time to this step or to investigate slow runs.

Three code paths produce results:

1. **gcc-like compiler**: spawns `<compiler> -v -E -x c++ [extra_args] -` and parses the verbose stderr.
2. **Windows `$INCLUDE`**: reads the `INCLUDE` env var and splits on `;`.
3. **Windows VS auto-detect**: invokes `vswhere.exe`, then `vsdevcmd.bat -arch=x64`, captures the resulting `INCLUDE` value, and parses it.

Each path has natural sub-stages (a "discover" stage that does the I/O and a "parse" stage that turns the raw output into a `Vec<String>`), which makes a uniform high-level phase model feasible.

## Goals / Non-Goals

**Goals:**
- Provide an opt-in CLI flag that emits one JSON timing line on stderr per run.
- Use a uniform JSON schema across all three execution paths so log scrapers can treat them identically.
- Emit timing on both success and failure runs (with whichever phases completed) so failed runs can be attributed.
- Add zero new dependencies. Add minimal architectural surface.
- Preserve default behavior exactly when the flag is absent.

**Non-Goals:**
- Building a generic logging or tracing framework.
- Sub-phase telemetry (e.g., separating `vswhere` from `vsdevcmd`).
- Multiple output formats (key=value, plain text, etc.).
- Counting time spent before `main` (process spawn, dynamic loader).
- Replacing or complementing `time(1)` for end-to-end measurement.

## Decisions

### D1. Opt-in flag rather than always-on

`--timing` defaults to `false`. Justification: the tool is often called from build scripts that capture stdout into another tool; emitting unsolicited stderr output, even on stderr, risks polluting build logs in projects that don't ask for it. Backward compatibility is the simpler default.

### D2. Output channel: stderr, regardless of `-o/--output`

The existing `-o`/`--output` flag controls where the include-dirs payload is written (stdout or file). Timing is meta-information about the run, never part of the payload. Keeping it on stderr means it never collides with consumers piping stdout into `cmake -DCMAKE_INCLUDE_PATH=...` style invocations.

### D3. Single-line JSON format

Justification: stable for CI log scrapers (`grep timing | jq`), structured for dashboards, no escaping ambiguity for the `error` field. `serde_json` is already present transitively, so the cost is zero. Considered key=value and aligned text; rejected because manual escaping of error strings is error-prone and aligned text is unstable across alignment changes.

### D4. Uniform schema across all paths: `discover_ms`, `parse_ms`, `write_ms`, `elapsed_ms`

Justification: log consumers should not need to know which platform path was taken. The two natural sub-stages of every path map cleanly:
- "discover" = whatever I/O produces the raw include-dirs material (subprocess, env var, batch file)
- "parse" = whatever turns that material into `Vec<String>`
- "write" = output writer
- "elapsed" = outer wall-clock from `main` start to the JSON-emit point

Considered path-tagged keys (`compiler_ms`, `vswhere_ms`, etc.) and a generic `phases` array; rejected because they push platform-awareness into the consumer for marginal information value.

### D5. Always-measure, conditionally-emit

`Instant::now()` calls happen unconditionally at every phase boundary. The `Timings` struct is built every run; `main` only serializes and prints it when `--timing` is set. Justification: `Instant::now()` is ~20 ns; threading a `bool` flag into every phase boundary is more code with no measurable benefit.

### D6. Error-path emission

When a phase fails, the timing line is emitted immediately before the existing `Error: ...` line, and the process still exits non-zero. The JSON omits the `Option`s for phases that did not complete and includes an `error` string. Justification: failed runs are exactly when CI dashboards most want to see timing — a slow `vsdevcmd.bat` that ultimately failed is a debuggable signal.

### D7. New module `src/timing.rs`

A two-file project benefits from a small dedicated module for `Timings` and `PhaseTimer`. Justification: keeps `main.rs` focused on CLI plumbing and `windows_vs.rs` focused on VS-specific logic; both depend on a shared timing primitive.

### D8. Phase functions return `(payload, Timings)`

`get_compiler_include_dirs` and `windows_vs::get_windows_include_dirs_with_fallback` change return type from `Result<Vec<String>, String>` to `Result<(Vec<String>, Timings), (Timings, String)>` (or an equivalent shape — exact ergonomics decided in implementation). Justification: phase boundaries live inside these functions; pushing timing capture out to `main` would either require duplicating the platform dispatch logic or fall back to wall-clock-only timing. Returning the partial `Timings` even on failure also enables D6 cleanly.

## Risks / Trade-offs

- **Risk**: signature changes to `get_compiler_include_dirs` and `get_windows_include_dirs_with_fallback` are minor breaking changes for anyone embedding this crate. → Mitigation: this is a binary CLI, not a published library; only `main.rs` and the Windows module call these functions internally. No external consumers.
- **Risk**: `discover_ms` collapses meaningfully different operations (a 10ms env var read vs a 4-second `vsdevcmd.bat` invocation). → Mitigation: this is the explicit design choice (D4). The tradeoff is uniformity over detail; users who need finer breakdown can shell-time the call.
- **Risk**: error messages from compiler stderr may contain control characters or non-UTF-8 bytes. → Mitigation: errors stored in `Timings.error` are `String` (already lossy-decoded in the gcc-like path) and serialized via `serde_json`, which handles JSON escaping correctly.
- **Trade-off**: `Instant` measurement runs even when `--timing` is off (D5). The cost is negligible (~20 ns × 4 boundaries) but is not zero. Acceptable.
- **Trade-off**: `elapsed_ms` is captured at the JSON-emit point, not at process exit. Time spent printing the JSON itself is excluded. Acceptable — the omission is constant and tiny.

## Migration Plan

No migration needed. `--timing` is purely additive; all existing invocations behave unchanged. No rollback strategy required beyond reverting the change.

## Open Questions

None blocking. Field ordering inside the JSON object follows `serde` derive order; if a specific key order is desired later it can be enforced via `#[serde(...)]` attributes without a schema change.
