## Design Summary

Relicense `get-system-include-dirs` from WTFPL Version 2 to the Rust ecosystem standard dual license `MIT OR Apache-2.0`. The change introduces a brand-new `license-conventions` capability whose requirements encode the convention (file layout, manifest field, per-file SPDX marker, canonical license texts), and the relicensing work is the implementation that brings the repo into compliance with that capability.

The project is single-author (sole copyright holder: Masashi Fujita), so unilateral relicensing is legally clean — no contributor consent process is required. WTFPL is permissive enough that downstream users licensed under it are not harmed by the move to MIT/Apache-2.0 (both are at least as permissive).

## Alternatives Considered

### Approach A: Fine-grained capability (chosen)

- **Make**: A `license-conventions` capability with four requirements: license file layout, Cargo.toml license field, SPDX header in source files, canonical license texts. Each requirement carries one or more `#### Scenario:` blocks. The relicensing implementation populates `LICENSE-MIT`, `LICENSE-APACHE`, deletes `COPYING`, sets `Cargo.toml`'s `license` field, and updates the three `*.rs` SPDX headers.
- **Pros**: Each requirement is grep-testable. Future drift (e.g., someone re-introduces `COPYING`, or pastes a `WTFPL` SPDX header from an old commit) is caught by `openspec validate license-conventions --type spec --strict` and by the requirement-level scenarios. Mirrors the precedent set by `spec-format-conventions`.
- **Cons**: More requirement blocks to author and maintain than a single coarse rule. Not a real cost — the requirements are short and the convention is genuinely four-part (file layout / manifest / source headers / canonical text).
- **Why chosen**: The granularity matches what the convention actually consists of. A single coarse requirement would obscure which part is non-compliant when validation fires.

### Approach B: Coarse capability — one big requirement

- **Make**: A single `### Requirement: Project is dual-licensed under MIT OR Apache-2.0` that asserts file layout + manifest field + SPDX header + canonical texts in one statement.
- **Pros**: Shorter spec.
- **Cons**: Validation failure messages don't localize to which sub-rule broke. Reads less like a checkable convention and more like a slogan.
- **Why not**: For a meta-conventions capability the whole point is grep-testability per sub-rule.

### Approach C: Direct PR — no opsx change, no capability

- **Make**: Treat the relicense as a docs/config tweak per `CLAUDE.md → Workflow routing`. One commit: replace COPYING with LICENSE-MIT/LICENSE-APACHE, set `Cargo.toml` `license` field, update three SPDX headers. No spec, no capability.
- **Pros**: Smallest possible diff and process surface. License-text-only changes are arguably docs-tier.
- **Cons**: License is an external contract. Without a capability, nothing prevents future drift (someone re-adding COPYING, mismatching the SPDX expression in different places, etc.). The cost of the capability is small enough that the durability is a clear net positive.
- **Why not**: User explicitly chose the opsx-with-new-capability route after weighing this. The `spec-format-conventions` precedent is the right pattern for repo-wide conventions.

## Agreed Approach

**Approach A** — fine-grained `license-conventions` capability with four requirements, each with `#### Scenario:` blocks. The relicensing work (Phase 3 of the implementation plan) is the apply-phase that brings the repo into compliance with the capability the change introduces. This is the same shape used by `spec-format-conventions`.

Scope is explicitly minimum: license files + Cargo.toml + 3 source-file SPDX headers + delete COPYING. **Not** in scope: README.md (project has none and adding one is a separate cycle), CONTRIBUTING.md (no contributor process needed for a single-author project), any source-code behavior change. The capability requirements describe *only* what the relicense scope touches.

## Key Decisions

1. **Capability granularity is fine, not coarse.** Four requirements (file layout, Cargo.toml field, SPDX header, canonical texts) — each grep-testable independently.
2. **Copyright year details are non-normative.** The capability requires *that* a `Copyright (c) <years> <holder>` line exist in `LICENSE-MIT`; it does not pin the year string. This avoids needing a spec edit every January and keeps the capability stable across years.
3. **Apache-2.0 body is byte-equivalent to canonical SPDX text.** No project-specific edits to the Apache body. Project copyright lives in `LICENSE-MIT`'s copyright line and in source-file SPDX headers, not in the Apache body.
4. **MIT body uses the SPDX template with one filled-in copyright line.** Year value chosen by implementation: `2025–2026` (en-dash range — preserves the original 2025 from `COPYING` and reflects current 2026 work). This year string is implementation detail, not normative.
5. **SPDX expression is exactly `MIT OR Apache-2.0`.** Same string in `Cargo.toml`'s `license` field and in every source file's `// SPDX-License-Identifier:` line. Consistency caught by the SPDX-header requirement's "no other identifier" scenario.
6. **Apply ceremony is waived.** The change touches 7 files (2 added license files, 1 deleted, 1 manifest edit, 3 SPDX header edits). Worktree dispatch + per-task subagent + TDD + per-task code review is mismatched for this scope. Direct in-session edits, single commit. The waiver is recorded explicitly in `plan.md` rather than relying on the docs-only implicit override (this change is not docs-only — it touches `Cargo.toml` and `*.rs` files — so the docs-only convention does not auto-apply).
7. **Change folder name**: `relicense-mit-apache-2`. Imperative-verb noun phrase, matches existing convention (`backfill-spec-purposes`, `reformat-windows-vs-detection-spec`).
