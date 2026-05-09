# Retrospective: fix-release-workflow-ubuntu-spec

> Written: 2026-05-10 (after verify passed with warnings)
> Commit range: `(uncommitted at write time — see §0)`
> Worktree: main checkout (no isolated worktree per project CLAUDE.md docs-only override)

---

## 0. Evidence

- **Commit range**: `(none at write time)`. Implementation applied directly in-session per project CLAUDE.md "Documentation-only changes" override. The single forthcoming commit lands the spec edit + the entire `openspec/changes/fix-release-workflow-ubuntu-spec/` directory.
- **Diff size**: +1 / −1 line in 1 baseline spec file (`release-workflow-ubuntu/spec.md`); +151 lines across 5 new files in the change directory (artifacts: brainstorm 44, proposal 39, plan 37, tasks 9, spec-format-conventions delta 22).
- **Tasks done**: 4/4 (`grep -c '^- \[x\]' openspec/changes/fix-release-workflow-ubuntu-spec/tasks.md` → 4)
- **Active hours**: ~10 min, single session (`/opsx:propose` → empty-scaffold validate → drafting → `/opsx:apply` → verify + retro). About 4× faster than the prior cycle.
- **Subagent dispatches**: 0 (project CLAUDE.md docs-only override applied; no per-cycle negotiation needed)
- **New external dependencies**: none
- **Bugs encountered post-merge**: n/a (not yet committed/merged)
- **OpenSpec validate state at archive**: change valid; specs 6/7 valid in `--strict` (was 5/7 pre-change). The 1 remaining failure (`windows-vs-detection`) is the last open follow-up from `backfill-spec-purposes`.
- **Test coverage signal**: n/a (no source-code changes)

Commit chain (chronological):

```
851a091 docs(claude): promote OpenSpec workflow lessons from backfill-spec-purposes retro  (HEAD before this change)
(pending) docs(opsx): forbid delta markers in baseline specs and fix release-workflow-ubuntu
(pending) chore(opsx): archive fix-release-workflow-ubuntu-spec
```

---

## 1. Wins

- **Both new CLAUDE.md rules from the prior retro fired naturally on the very next cycle** [evidence: bash log of `openspec validate fix-release-workflow-ubuntu-spec --type change` immediately after `openspec new change` returning the no-delta error; brainstorm.md "Approach A" framing as "ADDED Requirement to spec-format-conventions"]. The "validate empty scaffold first" rule caught the constraint at zero cost — no re-design pass needed. The "encode docs fix as convention requirement" rule made the design choice obvious and mechanical. This is direct evidence that retrospective candidates promoted to CLAUDE.md actually shape behavior.
- **Cycle wall time ~4× faster than prior** [evidence: §0 active hours]. ~10 min vs ~45 min. The improvement is almost entirely attributable to the two CLAUDE.md rules eliminating mid-flight design pivots.
- **Zero filler tasks** [evidence: tasks.md has only one implementation task and three verification tasks; no "confirm archive will create file X" meta-tasks]. The prior retro's 📌 "avoid filler tasks" candidate was followed without being explicitly invoked.
- **Predicted strict-pass outcome was exact** [evidence: plan.md predicted "5/7 → 6/7"; verify §1 confirms 6/7]. Knowing exactly what the validator will say after each step is a sign the spec model and the change scope are well-aligned.
- **The pattern is becoming routine** [evidence: this is the second consecutive cycle using the "encode docs fix as convention" pattern; the brainstorm.md explicitly notes it]. Routinization is the desired outcome — patterns that fire naturally don't need re-derivation each cycle.

## 2. Misses

- 📌 **[nit | evidence: plan.md "Practical override for this cycle" paragraph]** The override note in plan.md repeats the same boilerplate as last cycle's plan.md. As routinization continues, this will accumulate. Consider folding the override into the project CLAUDE.md so plan.md doesn't need to state it per-cycle. (See §6 candidate.)
- 📌 **[nit | evidence: brainstorm.md "Alternatives Considered" Approach C]** I dutifully wrote out three alternatives even though Approach A was foregone given the prior cycle's pattern. Brainstorming for routine pattern reuse adds little value. The schema's `superpowers:brainstorming` skill is the source of this expectation; it doesn't have a graceful "this is a routine reuse, skip alternatives enumeration" mode.

## 3. Plan deviations

| Plan task | What changed | Why |
|---|---|---|
| (artifact: design.md) | Skipped (same as prior cycle) | Not in `applyRequires`; rationale captured in brainstorm.md + proposal.md |
| (apply phase) | Direct in-session edits, no worktree, no subagent | Project CLAUDE.md docs-only override applied without per-cycle negotiation this time (vs the prior cycle which negotiated explicitly) |
| (apply phase) | No `superpowers:writing-plans` invocation | Same as prior cycle; one-line edit doesn't benefit from formal plan decomposition |
| Plan §3 (commit) | Bundled into same turn as verify + retro per user's "roll through" request | User-directed |

## 4. Skill / workflow compliance

| Skill | Used |
|---|---|
| superpowers:brainstorming | ✓ (truncated; alternatives enumeration was perfunctory — see §2) |
| superpowers:writing-plans | ✗ |
| superpowers:using-git-worktrees | ✗ |
| superpowers:subagent-driven-development | ✗ |
| (transitive) superpowers:test-driven-development | ✗ (n/a — no code) |
| (transitive) superpowers:requesting-code-review | ✗ |
| superpowers:finishing-a-development-branch | ⏳ pending (after archive) |

> **Default expectation**: every row ✓. This cycle has the same five ✗ as the prior one, all caused by the same root condition (documentation-only change against a source-code-oriented schema). The prior cycle's §6 schema PR motivator candidate continues to accumulate evidence (see §6 carry-forward).

### Deliberately Skipped Skills

The five active skips (`writing-plans`, `using-git-worktrees`, `subagent-driven-development`, transitive `test-driven-development`, transitive `requesting-code-review`) are skipped for **the exact same reasons as the prior cycle** (`backfill-spec-purposes` retro §4). Per the carry-forward mechanism, rather than re-paste the same three answers (What / Why / How to prevent) for each skill, this section refers to:

- `openspec/changes/archive/2026-05-09-backfill-spec-purposes/retrospective.md` §4 "Deliberately Skipped Skills" — full per-skill rationale.

What's *new* this cycle:

- The skips happened **without per-cycle negotiation** because the project CLAUDE.md docs-only override is now in place. This is a measurable improvement: zero AskUserQuestion calls about apply mode, zero back-and-forth.
- The accumulating evidence (now 2 cycles, same skip set, same root cause) reinforces the prior cycle's §6 schema PR motivator candidate. Carrying it forward (see §6) with the strengthened evidence claim.

## 5. Surprises

- **The prior cycle's §6 candidates didn't just inform behavior — they prevented redundant work.** I expected the new CLAUDE.md rules to be useful, but I underestimated how much friction the prior cycle's design pivot had cost. Without the empty-scaffold-validate rule, this cycle would have repeated the same mid-flight pivot.
- **Cycle wall time dropped 4×.** I expected maybe 2× speedup from the rules; actual was ~4×. The compounding factor: the rules eliminated not just the pivot but also the AskUserQuestion negotiation about apply mode (worktree/subagent vs direct).
- **`brainstorm.md`'s "Alternatives Considered" felt like ceremony.** When the next cycle uses the exact same pattern as this one, listing alternatives becomes performative. Surprise: routinization stress-tests the brainstorming skill in a way I didn't anticipate.

## 6. Promote candidates → long-term learning

### Carry-forward from prior retros (re-evaluated this cycle)

- [ ] 🟡 **superpowers-bridge schema's apply phase needs a documentation-only branch.** → **Promote to** schema (upstream PR motivator at JiangWay/openspec-schemas)
  > **Why**: Now **two consecutive cycles** with the same five-skill skip set, same root cause (documentation-only change against source-code-oriented schema). This cycle skipped without per-cycle negotiation only because the project CLAUDE.md docs-only override is in place — but other adopters of superpowers-bridge don't have that override.
  > **How to apply**: Same as prior — when a change's spec deltas would touch only `openspec/specs/**/*.md` (no source paths), the schema should auto-route to a lightweight apply branch. **New evidence this cycle**: the project-level override pattern works (zero negotiation cost); the upstream schema fix would generalize that to all adopters.
  > **Carry-forward note**: Prior retro had this candidate. Two-cycle evidence now justifies opening the upstream PR; not promoting locally because this is genuinely a schema-level fix.

### New candidates this cycle

- [ ] 🟡 **For documentation-only changes, the apply override should be implicit, not stated per-plan.** → **Promote to** project CLAUDE.md (OpenSpec workflow section, extends the existing "Documentation-only changes" rule)
  > **Why**: This cycle's plan.md repeated the same "direct in-session execution is appropriate per CLAUDE.md docs-only rule" override paragraph as the prior cycle. As the pattern routinizes, this becomes boilerplate. The CLAUDE.md docs-only rule already covers it; plan.md doesn't need to re-cite it per cycle.
  > **How to apply**: Extend the existing `## OpenSpec Workflow → Documentation-only changes need a meta-conventions capability` section in CLAUDE.md to also state: "When the implementation reduces to markdown edits within `openspec/specs/**/*.md`, the apply phase defaults to direct in-session edits without worktree/subagent dispatch — no per-plan override note needed."

- [ ] 📌 **The brainstorm step adds little value when a change reuses an established pattern.** → **One-off** (record only — pattern observation, not a hard rule)
  > **Why**: This cycle's brainstorm.md enumerated three alternatives mostly for ceremony; Approach A was a foregone conclusion given the prior cycle's pattern. The brainstorming skill doesn't have a "routine pattern reuse" branch.
  > **How to apply**: When the next cycle is "another instance of an already-established pattern," consider an abbreviated brainstorm (single paragraph: "this reuses the X pattern from change Y; no alternatives meaningfully differ") and document the elision in retro §3 plan deviations rather than fighting the skill's full alternatives flow.

### Already-promoted (from prior cycle, validated this cycle)

- [x] 🟡 **Validate the empty change scaffold immediately after `openspec new change`** — promoted to CLAUDE.md in commit `851a091`. **Validated this cycle**: rule fired immediately, caught the no-delta constraint at zero cost.
- [x] 🟡 **Documentation-only changes need a meta-conventions capability** — promoted to CLAUDE.md in commit `851a091`. **Validated this cycle**: the "Approach A" framing in brainstorm.md was a direct application of this rule.

### Carry-forward (still unchecked, not applicable this cycle)

- [ ] 🟡 (from `add-timing-output`) "When the spec describes failure scenarios, enumerate every phase explicitly OR state the phase-emission rule unambiguously" — N/A this cycle (no failure-scenario specs written). Carry forward.
- [ ] 🟡 (from `add-timing-output`) "/opsx:propose artifacts should be committed before /opsx:apply enters a worktree" — N/A this cycle (no worktree used). Carry forward; same root cause as schema-docs-only-branch candidate above.
- [ ] 📌 (from `backfill-spec-purposes`) "Avoid filler tasks that merely confirm schema behavior" — followed this cycle without being explicitly invoked (tasks.md has zero filler tasks). Treating as **practiced**, but leaving unchecked since it's a one-off observation about authoring style, not a promotable rule.
- [ ] 📌 (from `backfill-spec-purposes`) "`git stash` is a fast pre-existence proof" — Not used this cycle (validation was clear; no ambiguity to resolve). Carry forward as a useful technique-on-call.
- [ ] 📌 (from `add-timing-output`) Two `📌` one-offs about TDD test plan style and shell-pinned smoke tests — N/A this cycle (no source-code work). Carry forward.
