# Add Timing Output Implementation Plan

> **For agentic workers:** Use superpowers:subagent-driven-development
> to implement this plan task-by-task.

**Goal:** Add an opt-in `--timing` flag that emits one JSON timing line on stderr describing per-phase wall-clock duration uniformly across all three execution paths.

**Architecture:** A new `src/timing.rs` module hosts `Timings` (Serialize) and `PhaseTimer` helpers. Existing phase functions (`get_compiler_include_dirs`, `windows_vs::get_windows_include_dirs_with_fallback`) are refactored to return `(Vec<String>, Timings)` on success and `(Timings, String)` on failure, so partial timings are recoverable for the error path. `main` aggregates timings, computes `elapsed_ms`, and emits a `{"timing":{...}}` JSON line via `eprintln!` only when `--timing` is set.

**Tech Stack:** Rust 2024, `clap` (existing), `serde` + `serde_json` (existing), `std::time::Instant`. No new dependencies.

---

## Task 1: Timing module foundation

- [ ] **Step 1:** Create `src/timing.rs`. Add `use serde::Serialize;` and `use std::time::Instant;`.
- [ ] **Step 2:** Define `pub struct Timings` with fields `discover_ms: Option<u128>`, `parse_ms: Option<u128>`, `write_ms: Option<u128>`, `elapsed_ms: u128`, `error: Option<String>`. Add `#[derive(Default, Serialize)]`. Apply `#[serde(skip_serializing_if = "Option::is_none")]` to `discover_ms`, `parse_ms`, `write_ms`, and `error`.
- [ ] **Step 3:** Define `pub struct PhaseTimer { start: Instant }` with `pub fn start() -> Self { Self { start: Instant::now() } }` and `pub fn stop(self) -> u128 { self.start.elapsed().as_millis() }`.
- [ ] **Step 4:** Define `#[derive(Serialize)] pub struct TimingEnvelope { pub timing: Timings }` so the emitted JSON is shaped `{"timing": {...}}`.
- [ ] **Step 5:** Add `mod timing;` to the top of `src/main.rs` (under the existing `#[cfg(windows)] mod windows_vs;`).
- [ ] **Step 6:** Run `cargo build` to confirm the module compiles. Commit: `feat(timing): add Timings struct and PhaseTimer helper`.

## Task 2: Timing serialization tests (TDD before instrumentation)

- [ ] **Step 1:** In `src/timing.rs`, add a `#[cfg(test)] mod tests` block.
- [ ] **Step 2:** Write `test_serialize_full_success` — construct `TimingEnvelope { timing: Timings { discover_ms: Some(10), parse_ms: Some(2), write_ms: Some(1), elapsed_ms: 13, error: None } }`, assert `serde_json::to_string` returns a string equal to `{"timing":{"discover_ms":10,"parse_ms":2,"write_ms":1,"elapsed_ms":13}}` (no `error` key).
- [ ] **Step 3:** Write `test_serialize_partial_failure` — construct `TimingEnvelope { timing: Timings { discover_ms: Some(10), parse_ms: None, write_ms: None, elapsed_ms: 11, error: Some("boom".into()) } }`, assert resulting JSON contains exactly `discover_ms`, `elapsed_ms`, `error`.
- [ ] **Step 4:** Write `test_serialize_escapes_special_chars` — error message `"a\"b\\c\nd"`. Assert `serde_json::to_string` produces a string that 1) is one line (no raw newline) and 2) round-trips: `serde_json::from_str::<serde_json::Value>(&s).unwrap()`.
- [ ] **Step 5:** Run `cargo test` — these three tests SHALL pass. Commit: `test(timing): cover Timings JSON serialization`.

## Task 3: Add --timing CLI flag

- [ ] **Step 1:** In `src/main.rs`, add `#[arg(long)] timing: bool,` to the `Args` struct, just after the `output` field.
- [ ] **Step 2:** Run `cargo build`; check `cargo run -- --help` shows `--timing` in the help output.
- [ ] **Step 3:** Verify baseline behavior with `cargo run -- 2>&1 1>/dev/null` (no `--timing`) — stderr SHALL be empty (matches pre-change behavior). Commit: `feat(cli): add --timing flag`.

## Task 4: Instrument gcc-like path

- [ ] **Step 1:** Open `src/main.rs`. Change `get_compiler_include_dirs` signature from `Result<Vec<String>, String>` to `Result<(Vec<String>, Timings), (Timings, String)>`. Import `crate::timing::{PhaseTimer, Timings};`.
- [ ] **Step 2:** Inside the function, build `let mut t = Timings::default();`. Wrap the `Command::new(compiler)...output()` block: `let timer = PhaseTimer::start();` immediately before it; `t.discover_ms = Some(timer.stop());` immediately after. On `Err(e)` from the call, return `Err((t, format!("Failed to execute compiler: {}", e)))`.
- [ ] **Step 3:** Wrap the stderr decode + `parse_include_dirs(&stderr)` in a second `PhaseTimer`, recording into `t.parse_ms`. On `Err(e)` from the parser, set `t.parse_ms` (the timer has already stopped) and return `Err((t, e))`.
- [ ] **Step 4:** On success, return `Ok((dirs, t))`.
- [ ] **Step 5:** Update `get_include_dirs` signature analogously: `Result<(Vec<String>, Timings), (Timings, String)>`. Forward the inner result through.
- [ ] **Step 6:** `cargo build` to surface call-site mismatches. Commit: `refactor(timing): thread Timings through gcc-like path`.

## Task 5: Instrument Windows paths

- [ ] **Step 1:** In `src/windows_vs.rs`, change `get_windows_include_dirs_with_fallback` signature to `Result<(Vec<String>, Timings), (Timings, String)>`. Add `use crate::timing::{PhaseTimer, Timings};`.
- [ ] **Step 2:** For the `$INCLUDE` branch: build `let mut t = Timings::default();`, time `env::var("INCLUDE")` into `t.discover_ms`, then time `parse_include_env(&include_var)` into `t.parse_ms`. On parse error, return `Err((t, e))`.
- [ ] **Step 3:** For the auto-detect branch (`find_vs_and_get_include`): change its signature too, or inline the work into the wrapper. Time `find_vswhere` + `query_vswhere` + `run_vsdevcmd_and_capture_include` collectively into `t.discover_ms`. Time `parse_include_env(&include_value)` into `t.parse_ms`.
- [ ] **Step 4:** Ensure every `?`/`map_err` path that returns early in the autodetect branch returns `Err((t, message))` with `t` carrying whatever phases completed.
- [ ] **Step 5:** `cargo build` (use a Windows host or `cross` if needed for compile-check). Commit: `refactor(timing): thread Timings through Windows VS paths`.

## Task 6: Wire timing in `main`

- [ ] **Step 1:** In `main`, immediately after `let args = Args::parse();`, add `let outer = std::time::Instant::now();`.
- [ ] **Step 2:** Replace the existing `match get_include_dirs(...)` block with a version that handles the new tuple shapes. On `Ok((dirs, mut t))`: time the `write_output(&dirs, args.output)` call into `t.write_ms`, then `t.elapsed_ms = outer.elapsed().as_millis();`. If `args.timing`, emit `eprintln!("{}", serde_json::to_string(&TimingEnvelope { timing: t }).unwrap());`. Exit 0.
- [ ] **Step 3:** On `Err((mut t, msg))`: `t.elapsed_ms = outer.elapsed().as_millis(); t.error = Some(msg.clone());`. If `args.timing`, emit the JSON line on stderr **before** `eprintln!("Error: {}", msg);`. Exit 1.
- [ ] **Step 4:** `cargo run -- --timing 2>/tmp/t.log >/dev/null && cat /tmp/t.log` — confirm a single JSON line.
- [ ] **Step 5:** `cargo run -- --timing --compiler /no/such/compiler 2>/tmp/t.log; echo "exit=$?"; cat /tmp/t.log` — confirm JSON line with `error` field, exit code non-zero. Commit: `feat(main): emit JSON timing line on stderr under --timing`.

## Task 7: Verification

- [ ] **Step 1:** Run `cargo fmt --check`. If it complains, run `cargo fmt` and amend.
- [ ] **Step 2:** Run `cargo clippy -- -D warnings`. Resolve any warnings (likely around the new function signatures).
- [ ] **Step 3:** Run `cargo test`. All tests including the three new serialization tests SHALL pass.
- [ ] **Step 4:** Confirm baseline silence: `cargo run -- 2>&1 1>/dev/null | wc -c` should be `0`. (No timing line emitted when `--timing` is not set.)
- [ ] **Step 5:** Pipe success-path JSON through `jq`: `cargo run -- --timing 2>&1 1>/dev/null | jq .` should pretty-print without error.
- [ ] **Step 6:** Pipe failure-path JSON through `jq`: `cargo run -- --timing --compiler /no/such 2>&1 1>/dev/null | jq '.timing.error'` should return the error string. Final commit: `chore: verify timing output end-to-end`.
