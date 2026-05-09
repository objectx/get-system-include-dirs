# Retrospective: relicense-mit-apache-2

> Written: 2026-05-10 (after verify passed; pre-archive, pre-commit)
> Worktree: main (no isolation; explicit non-docs-only ceremony waiver per plan.md)

---

## 0. Evidence

- **Commit range**: `22c6afb..(unstaged)`. Single Conventional Commit will be created at archive step. Base is `chore(claude): enable sandbox for project sessions`.
- **Diff size**: 6 files changed in working tree (`COPYING` deleted, `Cargo.toml` edited, 3 SPDX headers in `src/*.rs` flipped, plus `.claude/settings.json` from a prior session that won't be staged). Plus 639 new lines across the change directory and the synced baseline + license-text files.
- **Tasks done**: 18/18 will be ticked at commit time (16/18 at archive entry; the remaining two are 8.4 archive — running this step — and 9.1 commit — the very next step).
- **Active hours**: ~25 min wall-clock from `/superpowers:brainstorming` invocation to retro write. In the same band as the recent docs-only cycles, slightly longer because of (a) the canonical Apache-2.0 fetch and byte-equivalence verification and (b) the four-requirement spec authoring (vs the docs-only cycles' typical one-or-two-requirement deltas).
- **Subagent dispatches**: 0. Per `plan.md → Apply mode override`, no worktree dispatch / per-task subagent / TDD / per-task code review.
- **OpenSpec validate state at archive entry**: PASS (`openspec validate --all --json` → 9/9 valid; pre-existing 4 INFO-level long-text findings on `windows-vs-detection`, inherited from a prior cycle, non-blocking, out of scope).
- **Test coverage signal**: `cargo test` 3 passed (1 suite, 0.00s) — same outcome as pre-change. No behavior changes were intended; none observed.
- **External fetches**: 1 (`raw.githubusercontent.com/spdx/license-list-data/main/text/{MIT,Apache-2.0}.txt`) for canonical license text sourcing. `LICENSE-APACHE` byte-equivalent to upstream verified via `diff -q`.

Commit chain (will be):

```
22c6afb chore(claude): enable sandbox for project sessions  (pre-cycle, by user)
<TBD>   chore(license): relicense from WTFPL to MIT OR Apache-2.0
<TBD>   chore(opsx): archive relicense-mit-apache-2  (or folded into the prior commit)
```

---

## 1. Wins

- **First non-docs-only cycle to use the meta-conventions capability pattern.** The pattern (introduce a capability whose requirements encode a convention; the implementation is bringing the repo into compliance) was established by `spec-format-conventions` over three docs-only cycles. This cycle proves it generalizes to mixed scopes — the change touched `Cargo.toml`, two new license-text files, three `*.rs` SPDX header edits, and one delete, none of which qualify as "docs-only". The capability still works as a durable convention encoder; the apply ceremony was just waived explicitly via `plan.md` rather than implicitly via the docs-only override.
- **Explicit ceremony waiver is the right shape for borderline changes.** The plan-level "Apply mode override" paragraph was *not* boilerplate this cycle (unlike its three docs-only-cycle predecessors). It carried weight: this change isn't docs-only (the docs-only implicit override doesn't auto-apply), but the scope is too narrow to repay the schema's heavy `applyRequires` chain. The paragraph explicitly named *why* the waiver applies (seven file-level operations, zero behavioral risk, no test surface) — making it a real argument rather than a copy-pasted disclaimer. This is the shape future small-but-not-strictly-docs-only cycles should reuse.
- **Validate-the-empty-scaffold rule fired naturally.** The CLAUDE.md rule "After running `openspec new change <name>`, run `openspec validate <name> --type change` against the empty scaffold *before* drafting any artifacts" surfaced the no-delta constraint at zero cost (one Bash call). The error message was specific (`Change must have at least one delta`); the response was authoring the spec delta first, which is the right shape for a change that introduces a brand-new capability.
- **Year-in-copyright spec carve-out.** The `Canonical license texts` requirement explicitly states that year and holder values are *not* normative — only the *presence* of a copyright line is. This avoids a calendar-year drift problem (the convention spec doesn't need a yearly edit) without weakening the convention. Pattern worth re-using for any spec whose subject matter has time-varying free-form fields.
- **Byte-equivalent canonical Apache-2.0 sourcing.** Pulled the SPDX canonical text via `raw.githubusercontent.com/spdx/license-list-data` (an SPDX-maintained source-of-truth repo) and copied it byte-for-byte. `diff -q` confirms identity. This is the right way to satisfy the `Apache body is byte-equivalent to canonical` scenario — the spec encodes a structural requirement that any future `LICENSE-APACHE` re-write must respect.
- **The verbal-brainstorm → opsx promotion routing in CLAUDE.md held up.** User invoked `/superpowers:brainstorming`; the conversation converged through file-layout, scope, routing, and approach forks; promotion to opsx (with a new `license-conventions` capability) was decided *during* the brainstorm rather than auto-triggering. The five promotion criteria all held by the time the design was presented in three sections. No anti-pattern (`docs/superpowers/specs/` write) was triggered.

## 2. Misses

- 🟡 [minor] **The default opsx flow has separate sync, verify, archive skills, but `openspec` CLI's `archive` command claims to do "Archive a completed change and update main specs" — i.e., it folds in sync.** I followed the plan's three-step `opsx:sync` → `opsx:verify` → `opsx:archive` flow because that's what the plan stated and because it gives a checkpoint between sync and archive. But `openspec archive` may have done the sync internally; I don't know without testing. This isn't wrong — running sync explicitly ensures the baseline is observable before archive — but the redundancy is worth understanding. Recorded as a §6 candidate to investigate.
- 📌 [nit] **`design.md` artifact left as `status: ready` (not authored).** The schema lists `design` as an artifact (output `design.md`); it's not in `applyRequires`, but its `ready` status caused a warning at archive time. `plan.md` is comprehensive and covers everything `design.md` would have. The right fix is either (a) author a one-line `design.md` that just says "see plan.md", (b) treat `design.md` as truly optional in this schema (status should not warn), or (c) confirm via the schema repo whether `design.md` and `plan.md` overlap is intended. Recorded for §6.
- 📌 [nit] **The retrospective is heavyweight relative to the cycle.** The change is a license relicense — small, well-scoped, low-risk. The retrospective convention here was established by larger architectural cycles (`reformat-windows-vs-detection-spec`, `backfill-spec-purposes`) and feels disproportionate for a 6-file diff. Two options: (a) accept that retrospectives normalize to scale (skip §-headers that have nothing to say), or (b) introduce a "retrospective-lite" convention for cycles below some scope threshold. Going with (a) here — sections that genuinely had nothing to say (Plan deviations) are short.

## 3. Plan deviations

| Plan task | What changed | Why |
|-----------|--------------|-----|
| (none) | Plan executed as written | Tight scope (7 file operations + sync + archive + commit) gave little room for deviation. The "archive auto-apply conflict" risk in `plan.md` is the only thing that might cause a post-archive deviation; mitigation step (`grep -c '^### Requirement:'` after archive) is queued. |

## 4. Skill / workflow compliance

| Skill                                            | Used |
|--------------------------------------------------|------|
| superpowers:brainstorming                        | ✓    |
| superpowers:writing-plans                        | ✗    |
| superpowers:using-git-worktrees                  | ✗    |
| superpowers:subagent-driven-development          | ✗    |
| (transitive) superpowers:test-driven-development | ✗    |
| (transitive) superpowers:requesting-code-review  | ✗    |
| superpowers:finishing-a-development-branch       | ✗    |
| opsx:sync                                        | ✓    |
| opsx:verify                                      | ✓    |
| opsx:archive                                     | ✓ (in flight) |

### Skills used

- **`superpowers:brainstorming`** — Invoked by the user. Drove the design discussion through file-layout, scope, routing, and approach forks. Output (the verbal design + the agreed `license-conventions` capability shape) was authored directly into the change folder per CLAUDE.md routing (`docs/superpowers/specs/` is anti-pattern; promotion to opsx is the right path). Five-criteria promotion check held by the end of section 3 of the design.
- **`opsx:sync`** — Invoked after apply (Tasks 3-7) completed. Promoted the delta to baseline `openspec/specs/license-conventions/spec.md`. Capability didn't exist before, so the sync was a "create new spec file" path; the auto-generated `## Purpose` TBD placeholder didn't appear because I authored the baseline directly with compliant Purpose content. `openspec validate --specs --strict` post-sync: 8/8.
- **`opsx:verify`** — Produced this cycle's `verify.md` (170 lines). All three dimensions (Completeness, Correctness, Coherence) clean. 0 CRITICAL, 0 WARNING, 0 SUGGESTION.
- **`opsx:archive`** — Running this step.

### Deliberately Skipped Skills

The seven apply-phase skills (writing-plans, using-git-worktrees, subagent-driven-development, TDD, requesting-code-review, finishing-a-development-branch) all skipped under the explicit ceremony waiver in `plan.md → Apply mode override`. Reason consistent with prior docs-only cycles: scope (seven file operations, zero behavioral risk) is too narrow to repay the per-task subagent / per-task review / worktree-isolation pattern.

The pattern this cycle establishes: when a change is *not* docs-only but the apply scope is still too small to repay full apply ceremony, write the waiver paragraph as an *explicit* argument in `plan.md`, naming the file count, the behavioral risk level, and the absence of test surface. Don't rely on implicit overrides for non-docs-only cases.

## 5. Carry-forward (open §6 candidates from prior cycles)

| Cycle | Candidate | Status |
|-------|-----------|--------|
| `backfill-spec-purposes`, `fix-release-workflow-ubuntu-spec`, `reformat-windows-vs-detection-spec` | "Docs-only apply override should be implicit, not stated per-plan" | **Promoted** in CLAUDE.md (commit `5ff04d8`). This cycle was the first non-docs-only cycle since promotion; the promoted text correctly did not auto-apply here, and the explicit waiver was used as designed. |

## 6. New §6 candidates (for future schema/CLAUDE.md fixes)

1. **Investigate `openspec archive` vs `opsx:sync` overlap.** Does `openspec archive` already run the delta-to-baseline sync? If yes, `opsx:sync` becomes optional (advisory before archive); if no, the three-step sync→verify→archive flow is correct as documented. Need to test in a future cycle. Cost of leaving unresolved: occasional double-sync (idempotent, low cost) or potentially unsynced archives if `archive` doesn't actually sync.
2. **Clarify `design.md` vs `plan.md` artifact roles in the superpowers-bridge schema.** The schema lists both as artifacts; `applyRequires` only includes `plan`. In our usage `plan.md` covered the whole design + plan space, and `design.md` was left as `status: ready`. Either the schema should mark `design` as truly optional (no warning when unauthored), or the convention should mandate `design.md` even when it just points to `plan.md`. Track upstream at `JiangWay/openspec-schemas`.
3. **Pattern: explicit "scope too narrow" ceremony waivers for non-docs-only changes.** This cycle established the pattern (named in §1 as a Win). It's the right shape for changes like dependency bumps, single-line code fixes, version bumps, and other small-scope but non-docs-only work. Should it be promoted into CLAUDE.md as a rule alongside the docs-only override? Probably not yet — one cycle of evidence isn't enough; revisit after 2-3 more cycles use this shape.
4. **Pattern: capability requirements with non-normative free-form fields.** The `Canonical license texts → MIT body matches SPDX template` scenario explicitly carves out year-and-holder values from normativity. This is a useful pattern for any spec whose subject has time-varying or instance-specific fields (e.g., contributor lists, version-tied URLs, dated cutoff thresholds). Worth recording somewhere durable; possibly as a `## Purpose`-section convention in `spec-format-conventions` ("requirements may explicitly mark free-form fields non-normative when the structural rule is what matters").

## 7. Forward-looking note

The repo is now at 8/8 specs on `openspec validate --specs --strict` and has an explicit licensing convention. No follow-up cycle is required from this work; the convention is durable and the implementation is in compliance. If a future contributor adds a `*.rs` file to `src/` without an SPDX header, or sets `Cargo.toml` `license-file`, or re-introduces `COPYING`, the spec's scenarios catch it.
