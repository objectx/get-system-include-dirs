# Tasks

## 1. Author the `license-conventions` delta spec

- [x] **1.1** Write `openspec/changes/relicense-mit-apache-2/specs/license-conventions/spec.md` with a `## ADDED Requirements` block containing four `### Requirement:` blocks (License file layout, Cargo manifest license field, SPDX header in source files, Canonical license texts). Each requirement statement uses `SHALL` (and `SHALL NOT` where applicable) inline; each is followed by one or more `#### Scenario:` blocks.
- [x] **1.2** Confirm the spec does not pin year or holder values for `LICENSE-MIT`'s copyright line — only its presence is normative.

## 2. Validate the change pre-apply

- [x] **2.1** Run `openspec validate relicense-mit-apache-2 --type change --strict`. Expect zero errors. → **PASS** (zero errors).

## 3. Apply — author license-text files

- [x] **3.1** Write `LICENSE-MIT` at repo root: SPDX canonical MIT template body with `Copyright (c) 2025–2026 Masashi Fujita <objectxtreme@gmail.com>` (en-dash U+2013). → File at root, 1103 bytes; `xxd` confirms en-dash bytes `e2 80 93`; body diff against `<year>/<copyright holders>` SPDX template is empty.
- [x] **3.2** Write `LICENSE-APACHE` at repo root: SPDX canonical Apache-2.0 text, byte-equivalent, no project-specific body edits. → File at root, 10280 bytes; `diff -q` against canonical SPDX text → identical.

## 4. Apply — remove the WTFPL `COPYING`

- [x] **4.1** Delete `COPYING` at repo root. Confirm `LICENSE-MIT` and `LICENSE-APACHE` are the only license-named files. → `COPYING` removed; `ls LICENSE LICENSE.md LICENCE LICENCE.md` reports no such files; only `LICENSE-MIT` and `LICENSE-APACHE` remain.

## 5. Apply — set `Cargo.toml` `license` field

- [x] **5.1** Add `license = "MIT OR Apache-2.0"` to `Cargo.toml`'s `[package]` table after `edition = "2024"`. Confirm `license-file` is absent. → `grep -E '^license = "MIT OR Apache-2\.0"$' Cargo.toml` matches exactly once; no `license-file` key present.

## 6. Apply — update SPDX headers in `src/*.rs`

- [x] **6.1** Edit `src/main.rs`: replace `// SPDX-License-Identifier: WTFPL` with `// SPDX-License-Identifier: MIT OR Apache-2.0`. → `grep -c 'SPDX-License-Identifier: MIT OR Apache-2.0' src/main.rs` = 1.
- [x] **6.2** Edit `src/windows_vs.rs`: same replacement. → count = 1.
- [x] **6.3** Edit `src/timing.rs`: same replacement. → count = 1.

## 7. Verify

- [x] **7.1** Run `cargo build --quiet` — confirms `Cargo.toml` syntax. → **PASS** (no diagnostics).
- [x] **7.2** Run `cargo test --quiet` — sanity check, expect same outcome as pre-change. → **PASS** (3 passed, 1 suite, same as pre-change).
- [x] **7.3** Grep audit: no `WTFPL` anywhere; no `COPYING`; SPDX line present in each `src/*.rs`; `Cargo.toml` `license` field present and exact. → All checks PASS.
- [x] **7.4** Run `openspec validate relicense-mit-apache-2 --type change --strict`. Expect zero errors. → **PASS**.
- [x] **7.5** Sanity check `wc -c LICENSE-APACHE` against the known canonical byte count. → 10280 bytes, byte-equivalent to canonical SPDX text via `diff -q`.

## 8. Sync delta to baseline + archive

- [x] **8.1** Run `opsx:sync` to promote the delta to baseline `openspec/specs/license-conventions/spec.md`. Replace any auto-generated `## Purpose` TBD placeholder with compliant Purpose content per `spec-format-conventions`. → Baseline created with compliant Purpose (no TBD); 4 `### Requirement:` blocks under `## Requirements`.
- [x] **8.2** Run `openspec validate --specs --strict`. Expect 8/8 specs pass. → **PASS** (8/8: build-automation, compiler-extra-args, license-conventions, release-workflow-macos, release-workflow-ubuntu, spec-format-conventions, timing-output, windows-vs-detection).
- [x] **8.3** Run `opsx:verify` to produce `verify.md`. → This task; verify.md authored alongside this checkbox flip.
- [ ] **8.4** Run `opsx:archive` to finalize. Re-run `openspec validate --specs --strict` post-archive; verify exactly four `### Requirement:` blocks in the baseline `license-conventions/spec.md`.

## 9. Commit

- [ ] **9.1** Conventional Commit: `chore(license): relicense from WTFPL to MIT OR Apache-2.0`. Body explains the motivation and the new `license-conventions` capability. Do not push (user-driven).
