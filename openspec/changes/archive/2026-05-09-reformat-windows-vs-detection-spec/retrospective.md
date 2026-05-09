# Retrospective: reformat-windows-vs-detection-spec

> Written: 2026-05-10 (after verify passed)
> Commit range: pre-commit (apply phase done in worktree, commit happens after retro per docs-only flow)
> Worktree: main (no isolation; docs-only override)

---

## 0. Evidence

- **Commit range**: `73370a4..(unstaged)` — single commit will be created at archive step. Base is the user's `chore: enable /opsx:{new, continue, sync, verify}` commit that landed mid-cycle (between cycle-2 archive and cycle-3 start).
- **Diff size**: +170 / −109 lines on `openspec/specs/windows-vs-detection/spec.md` (only file modified). Plus 493 new lines across the change directory: brainstorm (49), proposal (28), specs delta (222), tasks (18), plan (71), verify (105), retrospective (this file).
- **Tasks done**: 8/8 (`grep -cE '^\s*- \[x\]' openspec/changes/reformat-windows-vs-detection-spec/tasks.md` → 8; 0 open).
- **Active hours**: ~25 min wall-clock from user's "Restart cycles" message to retro write. Significantly faster than cycle 1 (~45 min) and roughly on par with cycle 2 (~10 min) — the upfront-validate rule + docs-only override pattern continues to compound.
- **Subagent dispatches**: 0. Per docs-only override, no worktree dispatch / per-task subagent / TDD / per-task code review.
- **New external dependencies**: none.
- **Bugs encountered post-merge**: none yet (not merged).
- **OpenSpec validate state at archive**: PASS (`openspec validate --all --json` → 8/8 valid; 4 INFO-level long-text findings on `windows-vs-detection`, non-blocking).
- **Test coverage signal**: n/a (docs-only).

Commit chain (will be):

```
73370a4 chore: enable /opsx:{new, continue, sync, verify}  (pre-cycle, by user)
<TBD>   docs(opsx): reformat windows-vs-detection to standard requirement shape
<TBD>   chore(opsx): archive reformat-windows-vs-detection-spec
```

---

## 1. Wins

- **Strict-validation goal hit (7/7).** `openspec validate --specs --strict` now passes for every spec in the corpus (was 6/7 since cycle 2). The convention encoded in `spec-format-conventions/spec.md` now describes the entire baseline, not 6/7 of it. The §6 candidate from cycle 1 ("the convention should describe what's actually true") is now satisfied on the corpus side.
- **Two prior-cycle CLAUDE.md rules fired naturally without prompting.** "Validate the empty change scaffold first" caught the no-delta constraint at zero cost (one Bash call after `openspec status`); "Documentation-only changes need a meta-conventions capability" justified the REMOVED + ADDED delta shape and the direct-edit apply. Both rules are paying compound interest across cycles.
- **Proper REMOVED + ADDED delta encoding for a wholesale restructure.** Cycles 1 & 2 used pure ADDED deltas (new capability / additional requirement). This cycle is the first that needed REMOVED+ADDED to honestly describe a shape change. The honest framing — REMOVED with `Reason` + `Migration` for the two parser-visible groupings, ADDED for the nine new requirements — passes validation and reads as a real architectural change rather than a stub.
- **`/opsx:verify` slash command landed mid-cycle and was used.** User added it between cycle 2 and this cycle (commit `73370a4`); verify ran via the now-supported path (manual invocation per fallback clause, since the precheck guards uncommitted-but-applied state which the schema doesn't natively cover yet).

## 2. Misses

- 🟡 [painful] **Auto mode + apply-mode question for docs-only.** Docs-only override is now firing on its third cycle without me even asking the question (this cycle had auto mode active and I just proceeded with direct edits). The `## Apply mode override` paragraph in plan.md is becoming pure boilerplate. This was identified as a §6 candidate after cycle 2; three cycles of evidence now warrant the promotion (see §6 below). 🟡 because the boilerplate paragraph costs ~15 lines per cycle but doesn't break anything.
- 📌 [nit] **Long-text INFO findings on 4 of 9 requirements.** The vswhere two-attempt detection / Version filtering / vsdevcmd / VS-detection error requirements all crossed the 500-char threshold. They're long because each describes a multi-step protocol where breaking up the SHALL conjunction would fragment a single semantic rule. The INFO level is correct (advisory, not blocking). Could be addressed by introducing OpenSpec's "AND" continuation pattern if it exists, or by accepting that protocol-shaped requirements are inherently long. Recorded for future judgment, not actionable now.

## 3. Plan deviations

| Plan task | What changed | Why |
|-----------|--------------|-----|
| (none) | Plan executed as written | Tight scope (one file rewrite + validator runs) gave little room for deviation. The "archive auto-apply conflict" risk in plan §"Risk" is the only thing that might cause a post-archive deviation; mitigation step is queued. |

## 4. Skill / workflow compliance

| Skill                                            | Used |
|--------------------------------------------------|------|
| superpowers:brainstorming                        | ✗    |
| superpowers:writing-plans                        | ✗    |
| superpowers:using-git-worktrees                  | ✗    |
| superpowers:subagent-driven-development          | ✗    |
| (transitive) superpowers:test-driven-development | ✗    |
| (transitive) superpowers:requesting-code-review  | ✗    |
| superpowers:finishing-a-development-branch       | ✗    |

### Deliberately Skipped Skills

All seven apply-phase skills skipped — same skip set as cycles 1 and 2. Three cycles of identical "Why this cycle" + "How to prevent" answers means the pattern is real and the §6 promotion is overdue (see §6).

- **`superpowers:brainstorming`**
  - **What was skipped**: Skill invocation. Brainstorm artifact written by hand directly to `openspec/changes/reformat-windows-vs-detection-spec/brainstorm.md` per the schema's output redirection.
  - **Why this cycle**: Auto mode is active (per `<system-reminder>` in the user's first turn). User said "Restart cycles" — a directive to proceed, not to discuss. The design space (three approaches: A=full reformat, B=minimal patch, C=convention-only) was already locked because cycle 1's retro had named "the windows-vs-detection FR-N reformat" as the open follow-up; the alternatives weighed in brainstorm.md were re-derived but not new.
  - **How to prevent recurrence**: `schema graph fix` — the schema's brainstorm artifact assumes a fresh design problem. For follow-up cycles whose scope is named-and-defined in a prior retro's §6 / open-follow-up section, the brainstorm step should either (a) auto-skip or (b) reduce to a "confirm the prior decision still holds" stub. PR motivator at `JiangWay/openspec-schemas` (combining with the docs-only branch motivator from cycle 1).
- **`superpowers:writing-plans`**
  - **What was skipped**: Skill invocation. Plan written by hand from tasks.md directly.
  - **Why this cycle**: Same as cycle 1 + 2 — docs-only change. The plan reduces to: edit one file, run validator, read output. The micro-step decomposition the skill produces is calibrated for code; it would generate noise for a Markdown edit.
  - **How to prevent recurrence**: Same as brainstorming — `schema graph fix`. Docs-only branch in the schema would auto-route around writing-plans for changes whose plan trivially reduces to "edit Markdown + run validator".
- **`superpowers:using-git-worktrees`**
  - **What was skipped**: Skill invocation. Worked directly on `main`.
  - **Why this cycle**: Docs-only override per project CLAUDE.md. The override is now implicit on its third cycle (see §6).
  - **How to prevent recurrence**: `schema graph fix` (docs-only branch) — same root cause as the previous two skips.
- **`superpowers:subagent-driven-development`**
  - **What was skipped**: Skill invocation. Apply phase ran in the parent session.
  - **Why this cycle**: Same as cycle 1 + 2 — docs-only override. Single-file Markdown edit doesn't benefit from per-task subagent dispatch.
  - **How to prevent recurrence**: `schema graph fix` (docs-only branch).
- **`superpowers:test-driven-development`**
  - **What was skipped**: Skill invocation.
  - **Why this cycle**: No code changed. The OpenSpec validator is the test; it ran in tasks 2.1, 2.2, 2.3 of plan.md.
  - **How to prevent recurrence**: `schema graph fix` (docs-only branch). For docs-only changes the equivalent of TDD is "validator-driven development" — write the convention, run validator, fix, repeat. That's already what happens; the formal TDD skill just doesn't apply.
- **`superpowers:requesting-code-review`**
  - **What was skipped**: Skill invocation. No subagent code review per task.
  - **Why this cycle**: Same as cycle 1 + 2 — docs-only override. Single Markdown rewrite reviewed inline by the parent session via direct read-back + validator runs.
  - **How to prevent recurrence**: `schema graph fix` (docs-only branch).
- **`superpowers:finishing-a-development-branch`**
  - **What was skipped**: Skill invocation.
  - **Why this cycle**: Same as cycle 1 + 2 — docs-only override; no PR creation in scope. Single commit + archive replaces the full branch-finishing flow.
  - **How to prevent recurrence**: `schema graph fix` (docs-only branch). The "ship" step for a docs-only change is `commit + archive`, not the full PR ceremony.

## 5. Surprises

- **The `--strict` validator's `requirements[X] is very long` is INFO-level and shows up in `--all --json` even when items are `valid: true`.** I'd expected only ERROR-level findings. The four INFO findings on the long protocol-shaped requirements are correct but caught me off-guard during verify §1; I had to explicitly note them as advisory rather than blocking. Useful surprise — the validator is more granular than I'd internalized.
- **Mid-cycle commit `73370a4` enabled the very thing my prior message had said was pending.** The user installed `/opsx:verify` between cycles 2 and 3 (during my context window's compaction), so verify ran via the now-supported path on its first try this cycle. The summary I wrote at the end of cycle 2 ("`/opsx:verify` is now properly wired up — future verify steps will route through it") turned out true on the very next cycle. Pleasant surprise; not actionable.

## 6. Promote candidates → long-term learning

- [x] 🟡 **For docs-only changes, the apply override should be implicit, not stated per-plan** → **Promote to project CLAUDE.md** (`/Users/objectx/Workspace/GitHub/get-system-include-dirs/CLAUDE.md`, OpenSpec Workflow → Documentation-only changes need a meta-conventions capability section). **Promoted 2026-05-10**: this is the third cycle where the override fired identically; the per-plan paragraph is now boilerplate. CLAUDE.md edit happens at commit time, captured in the same commit as the archive.
  > **Why**: Three cycles (`backfill-spec-purposes`, `fix-release-workflow-ubuntu-spec`, `reformat-windows-vs-detection-spec`) all used the same override paragraph in plan.md; auto mode now triggers it without even asking. Ceremony that fires identically every time should move from per-plan to CLAUDE.md.
  > **How to apply**: When a change's only modified files match `openspec/specs/**/*.md` (or other docs paths), the apply phase auto-defaults to direct in-session edits without worktree dispatch — no per-plan override paragraph needed. Plan.md may omit the override section entirely; reviewers infer it from the file scope.

- [ ] 🟡 **Schema needs a docs-only branch in the apply phase** → **Promote to upstream PR motivator** (`JiangWay/openspec-schemas`, superpowers-bridge schema). Three cycles of evidence: identical 7-skill skip set every cycle, identical "Why this cycle" + "How to prevent" answers in §4. The cycle 1 §6 candidate (carried forward in cycle 2, restated in cycle 3) is now overdue for the upstream PR. Adding a branch in the schema graph that, when the change's specs/* directory contains only `.md` files, auto-skips brainstorming → writing-plans → using-git-worktrees → subagent-driven-development → TDD → code-review → finishing-a-branch and routes directly to a "validator-driven" minimal apply flow.
  > **Why**: Three cycles, same skip set, same root cause. The override pattern is real and should be encoded in the schema rather than per-project CLAUDE.md (which only protects this repo, not other adopters). The cycle-2 retro promoted the §6 candidate "tighten brainstorm scope" to upstream, but didn't open the PR; this is the strongly-related PR.
  > **How to apply**: When a docs-only cycle (4th, 5th, ...) starts, the next person reading the schema doesn't need to re-derive "should I skip these 7 skills" — the schema branch handles it. Carry-forward in the next retro: open the PR if the next cycle is docs-only and would benefit; otherwise mark stale.

- [ ] 🟡 **The schema's brainstorm artifact assumes a fresh design problem; follow-up cycles defined by a prior retro's open-follow-up should auto-skip or reduce brainstorming** → **Promote to upstream PR motivator** (`JiangWay/openspec-schemas`, superpowers-bridge schema brainstorm artifact). New this cycle. The brainstorm.md I wrote re-derived three approaches that cycle 1's open-follow-up section had already named in essence ("reformat to OpenSpec's standard requirement shape"). Useful framing exercise but not new design work.
  > **Why**: Repeated brainstorming over a prior cycle's named follow-up is dead weight. The schema doesn't currently distinguish "fresh design problem" from "implementation of a previously-decided follow-up". For the latter, brainstorm should reduce to "confirm the prior decision still holds" rather than asking the alternatives-and-tradeoffs questions from scratch.
  > **How to apply**: Carry-forward in the next retro. If the next cycle is also a follow-up of a prior retro's §6 / open-follow-up, the same noise will repeat — that's the third datapoint that justifies the upstream PR.

- [ ] 📌 **Long-text INFO findings on protocol-shaped requirements are inherent, not fixable inside the requirement** → **One-off** (record only, no promotion needed). Multi-step protocols (vswhere two-attempt strategy) need the SHALL-conjunction style and inherently produce >500-char requirement statements. The validator's INFO level is the right behavior; the requirement structure is the right structure; no action required.
  > **Why**: Splitting one protocol into multiple requirements would fragment behavior that is semantically one rule. The INFO-level finding is the validator hinting "consider it" — and the answer is "considered, no thanks".

- [ ] 🟡 **Three cycles of identical skipped-skill rationale should auto-cluster in the next retro** → **Promote to schema (or skill description tightening)** (deferred). New this cycle as a meta-observation: when §4's Deliberately Skipped Skills subsection has the same answer for the same skill across N cycles, the pattern itself is a §6 candidate per the schema's own rule ("If multiple cycles skip the same skill with similar 'How to prevent' answers, that pattern is a §6 Promote candidate"). Rather than each cycle re-listing the seven skills with three near-identical answers each, the retro instruction could surface a "this skip pattern has fired N times" hint to make the cluster visible.
  > **Why**: §4 of this retro is mostly identical to cycle 1 §4 and cycle 2 §4. The schema's own rule already names the cluster, but doesn't give the next retro a tool to detect it. A `grep` over archived retros' Deliberately Skipped Skills subsections would surface clusters automatically.
  > **How to apply**: Next retro could check `grep -A 8 '^- \*\*\`superpowers:brainstorming\`\*\*' openspec/changes/archive/*/retrospective.md | grep -c "How to prevent"` to surface the cluster count. If ≥ 3 with identical answers, fast-track to upstream PR.
