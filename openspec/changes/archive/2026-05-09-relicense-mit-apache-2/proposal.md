## Why

The repository is currently licensed under WTFPL Version 2, recorded in `COPYING` and reflected in three source-file SPDX headers (`src/main.rs`, `src/windows_vs.rs`, `src/timing.rs`). WTFPL is uncommon in the Rust ecosystem: `Cargo.toml` has no `license` field set, crates.io and downstream license scanners don't reliably recognize WTFPL, and the dual `MIT OR Apache-2.0` choice is the de facto Rust standard (rust-lang/rust, tokio, serde, clap, regex, etc.). Aligning with that standard removes friction for downstream consumers, packagers, and license-audit tooling.

The repo also has no `license-conventions` capability today, so even after the relicense there is nothing preventing future drift — a stale SPDX header, a re-introduced `COPYING`, a missing `Cargo.toml` `license` field. Encoding the convention as a capability with `openspec validate --strict`-checkable requirements addresses that gap durably.

## What Changes

**New capability**: `license-conventions` — encodes the project's dual-license convention as four requirements (file layout, Cargo.toml license field, source-file SPDX header, canonical license texts), each with `#### Scenario:` blocks for strict-validation compliance.

**Repo state brought into compliance with the new capability:**

- `LICENSE-MIT` added at repo root (SPDX MIT template body with `Copyright (c) 2025–2026 Masashi Fujita <objectxtreme@gmail.com>`).
- `LICENSE-APACHE` added at repo root (SPDX canonical Apache-2.0 text, byte-equivalent, no project-specific body edits).
- `COPYING` (WTFPL Version 2 text) deleted.
- `Cargo.toml` `[package]` table gains `license = "MIT OR Apache-2.0"`.
- SPDX headers in `src/main.rs`, `src/windows_vs.rs`, `src/timing.rs` change from `// SPDX-License-Identifier: WTFPL` to `// SPDX-License-Identifier: MIT OR Apache-2.0`.

**Out of scope**: adding a `README.md` or `CONTRIBUTING.md` (project has none today; introducing one is a separate cycle), any source-code behavior change, any change to the build system or release pipeline.

## Capabilities

### New Capabilities

- `license-conventions`: Encodes the project's dual `MIT OR Apache-2.0` licensing convention as four grep-testable requirements (file layout, Cargo.toml manifest field, per-file SPDX header, canonical license-text bodies). The convention is durable across future cycles — drift is caught by `openspec validate license-conventions --type spec --strict` and by the requirement-level scenarios.

### Modified Capabilities

<!-- none -->

## Impact

- **Specs**: Adds `openspec/specs/license-conventions/spec.md` (after sync). No other spec is touched.
- **License files**: `COPYING` removed; `LICENSE-MIT` and `LICENSE-APACHE` added.
- **Manifest**: `Cargo.toml` gains a `license` field.
- **Code**: Three SPDX header lines updated; no executable behavior change.
- **Tests**: None added; existing `cargo test` is run as a sanity check post-apply (expected: unchanged pass/fail status, since no behavior changes).
- **CI**: None.
- **Validation gate**: After apply, `openspec validate --specs --strict` SHALL pass for the new `license-conventions` capability (alongside the existing 7/7), and `openspec validate relicense-mit-apache-2 --type change --strict` SHALL pass throughout authoring and post-apply.
- **Downstream**: Anyone consuming the project under WTFPL is unaffected — both MIT and Apache-2.0 grant strictly broader permissions than WTFPL withholds, so no existing downstream use is disrupted.
