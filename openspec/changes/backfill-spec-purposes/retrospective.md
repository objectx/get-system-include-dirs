# Retrospective: backfill-spec-purposes

> Written: 2026-05-10 (after verify passed with warnings)
> Commit range: `(uncommitted at write time — see §0)`
> Worktree: main checkout (no isolated worktree per user choice)

---

## 0. Evidence

> Quantitative front-matter; analysis sections below cite this rather than re-stating numbers per bullet.

- **Commit range**: `(none at write time)`. Implementation was applied directly in-session per user choice; the schema's "write retrospective while context is hot, before PR" rule is honored by writing now, ahead of the single forthcoming commit. The forthcoming commit will land six modified spec files plus the entire `openspec/changes/backfill-spec-purposes/` directory in one Conventional Commit (`docs(specs): backfill Purpose sections and standardize titles`).
- **Diff size**: +28 / −6 lines across 6 baseline spec files (existing files); +436 lines across 6 new files in the change directory (artifacts: brainstorm 65, proposal 50, plan 82, tasks 28, verify 136, spec-format-conventions/spec.md 75).
- **Tasks done**: 15/15 (`grep -c '^- \[x\]' openspec/changes/backfill-spec-purposes/tasks.md` → 15)
- **Active hours**: ~45 min, single session (`/opsx:propose` → `/opsx:apply` → `/opsx:verify` → retrospective)
- **Subagent dispatches**: 0 (user-confirmed direct in-session execution)
- **New external dependencies**: none (markdown-only edits)
- **Bugs encountered post-merge**: n/a (not yet committed/merged)
- **OpenSpec validate state at archive**: change is valid; specs are 4/6 valid in `--strict` (was 1/6 pre-change). The two remaining strict-failures (`release-workflow-ubuntu`, `windows-vs-detection`) are pre-existing structural defects explicitly recorded as out-of-scope follow-ups in `proposal.md`. **Pre-existence proof** captured in verify.md §1 via `git stash && openspec validate --specs --strict`.
- **Test coverage signal**: n/a (no source-code changes; verification is `openspec validate` + visual diff + a sanity grep over all six edited specs)

Commit chain (chronological):

```
0a95908 chore: bump to v1.1.0                                       (HEAD before this change)
(pending) docs(specs): backfill Purpose sections and standardize titles  (this change, uncommitted)
```

---

## 1. Wins

- **Honest resolution of the validator-vs-doc-only tension** [evidence: validator output captured in conversation; `specs/spec-format-conventions/spec.md`]. When `openspec validate` rejected the change for "Change must have at least one delta," the lazy fix (a fictitious stub MODIFIED delta) was offered and explicitly rejected in favor of introducing a real `spec-format-conventions` capability. This converts a one-shot backfill into durable normative requirements that future specs are checked against — the validator constraint pushed us toward a strictly better design.
- **Strict-validation pass count moved 1/6 → 4/6** [evidence: verify.md §1; `git stash` reproduction logged in conversation]. The two remaining failures are documented as separate follow-ups, not silently absorbed.
- **Out-of-scope follow-ups recorded at three places** [evidence: `proposal.md` "Out of scope" subsection; `tasks.md` 1.6 explicit do-not-touch note; verify.md §1 issues table]. Lowers the chance these get forgotten.
- **Pre-existence of remaining failures was proven, not assumed** [evidence: `git stash && openspec validate --specs --strict` then `git stash pop`]. Took ~10 seconds and removed the ambiguity around "did my edits break this?"
- **Heavyweight schema flow was negotiated transparently with the user** [evidence: AskUserQuestion in apply phase]. The schema prescribes git-worktree + per-task subagent + TDD + per-task code review. For six markdown header renames, this was disproportionate; the trade-off was surfaced explicitly rather than silently bypassed.

## 2. Misses

- 🟡 **[painful | evidence: validator response after writing brainstorm/proposal/specs/]** Did not run `openspec validate` against the empty change scaffold immediately after `openspec new change`. The "must have at least one delta" constraint was discovered only after writing brainstorm.md, proposal.md, and the empty `specs/` directory. Cost a re-design pass to add `spec-format-conventions`. Brainstorm.md and proposal.md both had to be edited mid-flight to record the scope expansion.
- 📌 **[nit | evidence: tasks.md §4]** Tasks 4.1 and 4.2 are meta-confirmations ("archive will materialize the new spec"; "no edit to CLAUDE.md required") rather than actionable work. They make the section feel padded. Should have been rolled into the proposal's Impact section instead.
- 📌 **[nit | evidence: schema instruction text vs cycle reality]** The superpowers-bridge schema's apply phase prescription does not degrade gracefully for documentation-only changes. The "or explicitly opt into the manual fallback path described at the end of this instruction" clause does not actually describe a fallback path — only the heavyweight path is described. A lightweight branch in the schema (or a clearer fallback section) would let docs-only cycles land without per-cycle negotiation.

## 3. Plan deviations

| Plan task | What changed | Why |
|---|---|---|
| (artifact: design.md) | Skipped entirely | `design.md` is not in `applyRequires` (only `plan` is). For this small change the design is already captured in `brainstorm.md` and `proposal.md`; a separate design.md would have been duplication. Schema-permitted. |
| (apply phase) | No git worktree, no subagent dispatch | User-chosen "Direct in-session edits" over the schema's prescribed worktree + subagent flow. Trade-off was surfaced via AskUserQuestion before deviation. |
| (apply phase) | No `superpowers:writing-plans` invocation for plan.md | plan.md was authored directly from the OpenSpec template. The micro-tasks (rename one heading, prepend two sections) did not benefit from formal plan decomposition. |
| Plan §9 (commit) | Not yet executed at retrospective write time | CLAUDE.md prohibits committing without explicit user instruction. Retrospective is being written before commit per the schema's "write while context is hot" rule. |

## 4. Skill / workflow compliance

| Skill | Used |
|---|---|
| superpowers:brainstorming | ✓ (truncated form: invoked, but converged in one round given the small clear scope) |
| superpowers:writing-plans | ✗ |
| superpowers:using-git-worktrees | ✗ |
| superpowers:subagent-driven-development | ✗ |
| (transitive) superpowers:test-driven-development | ✗ (n/a — no code) |
| (transitive) superpowers:requesting-code-review | ✗ |
| superpowers:finishing-a-development-branch | ⏳ pending (will run after commit + archive) |

> **Default expectation**: every row ✓. This cycle has five ✗ rows, all related to the same root condition: a documentation-only change against a schema designed for source-code changes. The §6 Promote candidates address this pattern; the per-skill rationale is below.

### Deliberately Skipped Skills

- **`superpowers:writing-plans`**
  - **What was skipped**: the entire skill. plan.md was authored from the OpenSpec template inline.
  - **Why this cycle**: every plan micro-task was a literal one-line edit to one markdown file (`Edit "## Overview" → "## Purpose"`). Plan decomposition into 2-5 minute TDD micro-steps was not applicable — there is no implementation code to write tests for.
  - **How to prevent recurrence**: `schema graph fix` — `plan.requires` and `plan.instruction` could conditionally bypass `superpowers:writing-plans` when the change's spec deltas affect zero source-code paths. Concretely: the `plan` artifact's instruction could begin with "If `tasks.md` contains only edits to `**/*.md` files, template-direct authorship is acceptable." This avoids per-cycle negotiation for docs changes.

- **`superpowers:using-git-worktrees`**
  - **What was skipped**: worktree creation. Edits were applied directly to the main checkout.
  - **Why this cycle**: user explicitly chose "Direct in-session edits" when offered the three options (direct / worktree-only / full superpowers). Reason given: the six edits are trivial header renames; isolation overhead exceeds the change's blast radius.
  - **How to prevent recurrence**: `scope-judgment rule` — for changes touching only `openspec/specs/**/*.md` (or any glob configurable as "documentation paths"), the apply phase should default to direct edits without offering the worktree option at all. Surface the worktree option only when the change is expected to touch source code (any non-docs glob). This both reduces friction and surfaces the question only when the answer is meaningful.

- **`superpowers:subagent-driven-development`**
  - **What was skipped**: the entire skill (no per-task subagent dispatch).
  - **Why this cycle**: same root cause as worktrees skip — six trivial markdown edits. Spawning six subagents to rename a heading would have inflated cost and latency without changing the outcome.
  - **How to prevent recurrence**: `scope-judgment rule`, same as above. Tying subagent dispatch to "source-code-touching change" rather than "every change" makes the schema honest about when isolation buys safety.

- **`superpowers:test-driven-development`** (transitive)
  - **What was skipped**: writing failing tests before implementation.
  - **Why this cycle**: there is no implementation code in this change. Validation is `openspec validate --strict` plus the new `spec-format-conventions` requirements being satisfied by the edited specs.
  - **How to prevent recurrence**: `schema graph fix` — TDD transitive activation should be conditional on the change touching source files (mirror of the writing-plans gate). An "asserts via spec-validator + spec-defined invariants" mode is a legitimate verification path for documentation changes.

- **`superpowers:requesting-code-review`** (transitive)
  - **What was skipped**: dispatching a code-reviewer subagent after each task and at the end.
  - **Why this cycle**: change is markdown only and the diff per file is 1-8 lines. The OpenSpec validator (which checks the new `spec-format-conventions` requirements against each edited spec) plus the sanity grep in tasks.md §2 plus the user's own diff review serve the same function for documentation. Code review's value comes from catching subtle behavior bugs in source code; it adds little to a diff that renames `## Overview` → `## Purpose`.
  - **How to prevent recurrence**: `scope-judgment rule` — for documentation-only changes, the reviewer pass can be substituted by `openspec validate --strict --all` plus a single coherence check against the new convention requirements. Schema could allow this substitution when the spec deltas affect no source paths.

- **`superpowers:finishing-a-development-branch`** — Not skipped, just deferred. Will run after commit + archive per the schema's prescribed sequence.

> **Cross-cycle pattern observation**: four of the five active skips share the same root condition (documentation-only change against a source-code-oriented schema) and the same prevention answer (gate the skill on source-path touch). Per the §4 → §6 escalation rule, this pattern is promoted to §6 as a schema PR motivator.

## 5. Surprises

- **OpenSpec model rejects documentation-only changes for lacking deltas.** I assumed the change scaffold could be valid with zero deltas as long as the artifacts were complete. The validator's `Change must have at least one delta` is hard-required. The lazy workaround (stub MODIFIED) would have been a lie about what was changing; the honest workaround (introduce a real conventions capability) was a strict improvement. The validator pushed the design in the right direction.
- **Pre-edit strict-validation showed 5/6 specs failing.** I expected 2 of 6 to fail (the known follow-ups). Three additional specs failed because they lacked a `## Requirements` wrapper — the very thing the new `spec-format-conventions` capability requires. So the "1/6 → 4/6 improvement" headline is partly tautological: this change introduces the rule and brings the targets into compliance with the rule it introduces. That's the legitimate purpose of a meta-conventions capability, but worth stating clearly.
- **The schema's "manual fallback path" is referenced but not documented.** The apply instruction says "or explicitly opt into the manual fallback path described at the end of this instruction" — but the end of the instruction only describes the heavyweight path. This forced per-cycle negotiation rather than a clean documented opt-out.

## 6. Promote candidates → long-term learning

- [ ] 🟡 **Validate the empty change scaffold immediately after `openspec new change`, before drafting any artifacts.** → **Promote to** project CLAUDE.md (OpenSpec workflow section)
  > **Why**: This cycle discovered the validator's "must have at least one delta" hard-rule only after writing brainstorm + proposal + empty specs/. A 5-second `openspec validate <name> --type change` against the empty scaffold would have surfaced the constraint before any drafting, avoiding a mid-flight design pass.
  > **How to apply**: Add to `CLAUDE.md` under "OpenSpec Workflow": "After `openspec new change <name>`, immediately run `openspec validate <name> --type change` to surface schema-required constraints (deltas, applyRequires) before drafting artifacts."

- [ ] 🟡 **For documentation-only changes that don't fit the OpenSpec delta model, introduce a meta-conventions capability rather than fabricating stub deltas.** → **Promote to** project CLAUDE.md (OpenSpec workflow section)
  > **Why**: When `openspec validate` rejects a docs change for lacking deltas, the lazy fix is a no-op stub MODIFIED. The honest fix is to ask "what conventions am I converging on?" and introduce a new capability whose requirements the docs change is implementing. The latter is a strictly better artifact: testable, durable, prevents future drift.
  > **How to apply**: When a planned change touches only `openspec/specs/**/*.md` and has no requirement deltas, treat that as a signal to scope a meta-conventions capability instead. Recognize the smell: "I'm fighting the validator to skip deltas" → re-frame as "I'm encoding a convention as deltas."

- [ ] 🟡 **superpowers-bridge schema's apply phase needs a documentation-only branch.** → **Promote to** schema (upstream PR motivator at JiangWay/openspec-schemas)
  > **Why**: Five of the seven required apply-phase skills (writing-plans, using-git-worktrees, subagent-driven-development, transitive TDD, transitive code-review) were skipped this cycle because they don't apply to markdown edits. Per cycle the same negotiation will happen for any docs-only change. The current "manual fallback" is referenced but not described in the schema's apply instruction.
  > **How to apply**: When a change's spec deltas would touch only `openspec/specs/**/*.md` (no source paths), the schema should auto-route to a lightweight apply branch: direct edits permitted, validate-as-review, no worktree required. Alternative: explicitly document the "manual fallback path" referenced in the apply instruction so adopters can opt into it without per-cycle negotiation.

- [ ] 📌 **Avoid filler tasks that merely confirm schema behavior.** → **One-off** (record only, don't generalize)
  > **Why**: tasks.md §4 ("after archive, the new spec will exist") restates schema behavior that is true by definition. Such tasks pad the count without describing real work.
  > **How to apply**: When drafting tasks.md, ask per task "is this an action a human or agent has to take, or is it a fact about the schema?" If the latter, move to proposal.md's Impact section.

- [ ] 📌 **`git stash` is a fast pre-existence proof for "did I cause this validator failure?"** → **One-off** (technique, not a rule)
  > **Why**: When `openspec validate --strict` reported 2 spec failures after my edits, a quick `git stash && openspec validate --specs --strict && git stash pop` proved the failures pre-existed. Took 10 seconds, removed the ambiguity. Worth remembering when validator output is unclear about cause.
  > **How to apply**: Whenever a verification step fails after a multi-file edit and the cause is ambiguous, stash → re-verify → unstash to isolate. Faster than reading diffs trying to reason about which line might have triggered which check.
