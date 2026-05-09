# Reformat windows-vs-detection Spec — Implementation Plan

**Goal:** Bring `openspec/specs/windows-vs-detection/spec.md` into compliance with the OpenSpec strict-validation requirement shape (`### Requirement: <name>` + inline `SHALL`/`MUST` + `#### Scenario:` blocks), without changing any documented behavior.

**Architecture:** Documentation-only change. The single edited file is the baseline spec under `openspec/specs/`. No code, no tests, no CI changes.

**Tech Stack:** Markdown + OpenSpec validator (`openspec validate ... --type spec --strict`).

---

## Apply mode override

Per `CLAUDE.md → OpenSpec Workflow → Documentation-only changes need a meta-conventions capability` (which has now governed three docs-only cycles: `backfill-spec-purposes`, `fix-release-workflow-ubuntu-spec`, and this one), this change uses **direct in-session edits** instead of the schema's heavyweight worktree + per-task subagent + TDD + per-task code-review prescription. The implementation reduces to a single Markdown edit; the schema's apply prescription is built for code changes and is mismatched here.

This carries §6 candidate "For docs-only changes, the apply override should be implicit, not stated per-plan" forward — three cycles of evidence now supports promoting it. (Tracked in `tasks/todo.md`; promotion deferred to keep this cycle scoped.)

---

## Task 1: Rewrite the requirements section

- [ ] **Step 1:** Read `openspec/specs/windows-vs-detection/spec.md` to confirm the exact byte-range to replace (everything between the `## Requirements` header and the `## Interface Specifications` header — exclusive on both ends, but the `## Requirements` line itself stays).
- [ ] **Step 2:** Read `openspec/changes/reformat-windows-vs-detection-spec/specs/windows-vs-detection/spec.md` to extract the nine `### Requirement: <name>` blocks under `## ADDED Requirements` (lines 17 onward in the change spec). These are the canonical replacement content.
- [ ] **Step 3:** Replace the requirements body in the baseline. The new structure is exactly:
  ```
  ## Requirements

  ### Requirement: $INCLUDE precedence
  ...
  ### Requirement: vswhere two-attempt detection
  ...
  ### Requirement: Version filtering via --vs-version
  ...
  ### Requirement: vsdevcmd INCLUDE capture
  ...
  ### Requirement: INCLUDE value parsing
  ...
  ### Requirement: Detailed VS-detection errors
  ...
  ### Requirement: Platform: Windows-only
  ...
  ### Requirement: Architecture: x64 only
  ...
  ### Requirement: Performance budget
  ...
  ```
  No `### Functional Requirements` / `### Non-Functional Requirements` subheadings. Each requirement statement contains `SHALL` or `MUST` inline; each is followed by one or more `#### Scenario:` blocks. Use `---` separators between requirements (mirroring the convention used in `release-workflow-ubuntu/spec.md` and `timing-output/spec.md`).
- [ ] **Step 4:** Leave everything below `## Requirements` untouched: `## Interface Specifications`, `## Dependencies`, `## Behavior Specifications` (Scenarios 1-5 in narrative form), `## Testing Requirements`, `## Future Considerations (Out of Scope)`.
- [ ] **Step 5:** Mark tasks 1.1, 1.2, 1.3 in `tasks.md` complete.

## Task 2: Validate

- [ ] **Step 1:** Run `openspec validate windows-vs-detection --type spec --strict`. Expect zero errors. If any error reports `Requirement must contain SHALL or MUST keyword` or `Each requirement MUST include at least one #### Scenario: block`, fix the offending requirement before proceeding.
- [ ] **Step 2:** Run `openspec validate --specs --strict`. Expect 7/7 specs to pass (was 6/7 before this change).
- [ ] **Step 3:** Run `openspec validate reformat-windows-vs-detection-spec --type change --strict`. Expect the change to still validate after the baseline edit.
- [ ] **Step 4:** Mark tasks 2.1, 2.2, 2.3 in `tasks.md` complete.

## Task 3: Sanity-check the rewrite

- [ ] **Step 1:** `git diff openspec/specs/windows-vs-detection/spec.md` and walk every old FR-N / NFR-N to confirm there's a corresponding new `### Requirement:` block with equivalent semantic content. The old "Acceptance Criteria" bullets MUST be present either as scenarios or folded into the requirement statement — none should be silently dropped.
- [ ] **Step 2:** Confirm `## Behavior Specifications` Scenarios 1-5 (narrative form, near the bottom of the file) are still present and unmodified.
- [ ] **Step 3:** Mark tasks 3.1, 3.2 in `tasks.md` complete.

---

## Risk: archive auto-apply conflict

**What:** `openspec archive` may attempt to auto-apply the REMOVED + ADDED deltas in `specs/windows-vs-detection/spec.md` to the baseline file *after* the apply phase has already written the same content there. This could produce duplicate requirements, unfound REMOVED targets, or other drift.

**Mitigation:** After `openspec archive -y`, immediately re-run `openspec validate windows-vs-detection --type spec --strict` and `grep -c '^### Requirement:' openspec/specs/windows-vs-detection/spec.md` — expect zero errors and exactly nine `### Requirement:` blocks. If counts diverge, the archive auto-apply has done something unexpected; reconcile by hand before committing.

**Precedent:** Cycles 1 and 2 successfully archived after manual baseline edits with `## ADDED Requirements` deltas (cycle 1 created a brand-new spec; cycle 2 added a 5th requirement to that spec). The wholesale REMOVED + ADDED restructure in this cycle is a less-tested archive path; hence the explicit verification step.
