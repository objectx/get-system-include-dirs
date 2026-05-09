# fix-release-workflow-ubuntu-spec Implementation Plan

> **For agentic workers:** Use superpowers:subagent-driven-development
> to implement this plan task-by-task.
>
> Practical override for this cycle: same as the prior `backfill-spec-purposes`
> cycle — single-line markdown edit, direct in-session execution is appropriate
> per the project's CLAUDE.md "Documentation-only changes" rule. Worktree +
> per-task subagent dispatch is over-prescription for this scope.

**Goal:** Bring `openspec/specs/release-workflow-ubuntu/spec.md` into compliance with the new `Requirements section header` requirement of `spec-format-conventions` by replacing its leaked `## ADDED Requirements` marker with `## Requirements`.

**Architecture:** One-line markdown edit. The new `spec-format-conventions` requirement is materialized at archive.

**Tech Stack:** Markdown. Verification via `openspec validate` (CLI v1.3.1).

---

## Task 1: Rename the leaked delta marker

- [ ] **Step 1:** Open `openspec/specs/release-workflow-ubuntu/spec.md`.
- [ ] **Step 2:** Locate the line `## ADDED Requirements` (it should be the second `##` heading in the file, immediately after `## Purpose`).
- [ ] **Step 3:** Replace `## ADDED Requirements` with `## Requirements`. Leave every `### Requirement:` and `#### Scenario:` block below the header completely unchanged.
- [ ] **Step 4:** Confirm the file structure is now: `# Spec: Release Workflow (Ubuntu)` → blank → `## Purpose` → blank → Purpose body → blank → `## Requirements` → blank → first `### Requirement:` block.

## Task 2: Validate

- [ ] **Step 1:** Run `openspec validate release-workflow-ubuntu --type spec --strict`. Expect: `Specification 'release-workflow-ubuntu' is valid` (was failing before with `Spec must have a Requirements section`).
- [ ] **Step 2:** Run `openspec validate fix-release-workflow-ubuntu-spec --type change`. Expect: `Change 'fix-release-workflow-ubuntu-spec' is valid`.
- [ ] **Step 3:** Run `openspec validate --specs --strict`. Expect: 6/7 specs pass; the only failure SHALL be `windows-vs-detection` (the deferred FR-N reformat follow-up).
- [ ] **Step 4:** Run `openspec status --change fix-release-workflow-ubuntu-spec`. Confirm `applyRequires` artifacts are all `done`.

## Task 3: Commit

- [ ] **Step 1:** Stage `openspec/specs/release-workflow-ubuntu/spec.md` and the entire `openspec/changes/fix-release-workflow-ubuntu-spec/` directory in one commit.
- [ ] **Step 2:** Conventional Commit message: `docs(opsx): forbid delta markers in baseline specs and fix release-workflow-ubuntu`.
- [ ] **Step 3:** Do NOT push. Do NOT archive yet — verify and retrospective come first.
