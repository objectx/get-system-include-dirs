## 1. Timing primitives

- [x] 1.1 Create `src/timing.rs` with a `Timings` struct (fields: `discover_ms`, `parse_ms`, `write_ms` as `Option<u128>`; `elapsed_ms` as `u128`; `error` as `Option<String>`) and `#[derive(Default, Serialize)]` plus `#[serde(skip_serializing_if = "Option::is_none")]` on the optional fields
- [x] 1.2 Add a `PhaseTimer` helper to `src/timing.rs` wrapping `std::time::Instant` with a `stop(self) -> u128` method returning elapsed milliseconds
- [x] 1.3 Add a `Serialize` wrapper struct so the emitted JSON has shape `{"timing": <Timings>}` (e.g. `#[derive(Serialize)] struct TimingEnvelope { timing: Timings }`)
- [x] 1.4 Wire `mod timing;` from `src/main.rs`

## 2. CLI flag

- [x] 2.1 Add `#[arg(long)] timing: bool` field to the `Args` struct in `src/main.rs`
- [x] 2.2 Confirm the flag is accepted by `clap` and defaults to `false` when omitted

## 3. Phase instrumentation — gcc-like path

- [x] 3.1 Change `get_compiler_include_dirs(...)` to capture an `Instant` before `Command::new(compiler).output()`, populate `Timings.discover_ms` after the subprocess returns
- [x] 3.2 Move `String::from_utf8_lossy(&output.stderr)` and `parse_include_dirs(&stderr)` inside a second timed region, populating `Timings.parse_ms`
- [x] 3.3 Change the return type to carry `Timings`. On success, return `(Vec<String>, Timings)`. On failure, return the partially-populated `Timings` plus the error string.
- [x] 3.4 Update `get_include_dirs(...)` to thread the new return shape through to `main`

## 4. Phase instrumentation — Windows paths

- [x] 4.1 In `src/windows_vs.rs`, change `get_windows_include_dirs_with_fallback(...)` to populate and return a `Timings` value alongside the `Vec<String>` (or partial `Timings` plus error on failure)
- [x] 4.2 For the `$INCLUDE` branch: time the `env::var("INCLUDE")` read into `discover_ms`, time `parse_include_env(...)` into `parse_ms`
- [x] 4.3 For the auto-detect branch: time `find_vswhere` + `query_vswhere` + `run_vsdevcmd_and_capture_include` together into `discover_ms`, time the final `parse_include_env(&include_value)` into `parse_ms`
- [x] 4.4 Ensure partial `Timings` is returned on every failure path inside the module (vswhere missing, vsdevcmd failure, parse failure)

## 5. Output write timing and main wiring

- [x] 5.1 In `main`, capture the outer `Instant` immediately after `Args::parse()` (this anchors `elapsed_ms`)
- [x] 5.2 Call `get_include_dirs(...)`; on success, time the `write_output(...)` call into `Timings.write_ms`
- [x] 5.3 On success: compute `elapsed_ms` from the outer instant, then if `args.timing`, serialize `TimingEnvelope { timing }` with `serde_json::to_string` and `eprintln!` it
- [x] 5.4 On failure: receive partial `Timings`, set `error = Some(message)`, set `elapsed_ms`, and if `args.timing`, emit the JSON line on stderr **before** the existing `Error: ...` print

## 6. Tests

- [x] 6.1 Add unit test: `Timings { discover_ms: Some(10), parse_ms: Some(2), write_ms: Some(1), elapsed_ms: 13, error: None }` serializes to JSON containing all four `*_ms` keys and no `error` key
- [x] 6.2 Add unit test: `Timings { discover_ms: Some(10), parse_ms: None, write_ms: None, elapsed_ms: 11, error: Some("boom".into()) }` serializes to JSON containing `discover_ms`, `elapsed_ms`, `error` and omitting `parse_ms` and `write_ms`
- [x] 6.3 Add unit test: error string with embedded `"`, `\`, and newline serializes to a single-line valid JSON
- [ ] 6.4 Update existing tests for `get_compiler_include_dirs` / parsing to accommodate the new return signature; add at least one assertion that returned `Timings` has `discover_ms` and `parse_ms` set to `Some(_)`
- [ ] 6.5 Where Windows tests exist, add equivalent assertions for `get_windows_include_dirs_with_fallback`'s new return shape (skip if no test infrastructure exists for that module)

## 7. Verification

- [ ] 7.1 Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`; all SHALL pass
- [ ] 7.2 Manual smoke test: `cargo run -- --timing 2>/tmp/timing.log >/dev/null; cat /tmp/timing.log` shows a single JSON line parseable by `jq`
- [ ] 7.3 Manual smoke test (failure path): `cargo run -- --timing --compiler /no/such/compiler 2>/tmp/timing.log; cat /tmp/timing.log` shows a JSON line with `error` set, exit code non-zero
- [ ] 7.4 Confirm baseline behavior: `cargo run -- 2>/dev/null` produces identical output to a build before this change (no timing line on stderr)
