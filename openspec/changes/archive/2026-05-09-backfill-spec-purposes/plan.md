# backfill-spec-purposes Implementation Plan

> **For agentic workers:** Use superpowers:subagent-driven-development
> to implement this plan task-by-task.

**Goal:** Bring all six baseline specs in `openspec/specs/` into compliance with the new `spec-format-conventions` capability — every spec begins with `# Spec: <Title Case>` followed by a populated `## Purpose` section.

**Architecture:** Pure documentation edits to six markdown files. No source code, no build configuration, no CI workflow. The new conventions capability is materialized as a baseline spec by `openspec archive` after this change is verified.

**Tech Stack:** Markdown only. Verification via `openspec validate` (CLI v1.3.1) and visual diff.

---

## Task 1: Backfill `build-automation/spec.md`

- [ ] **Step 1:** Open `openspec/specs/build-automation/spec.md`
- [ ] **Step 2:** Replace the line `## Overview` with `## Purpose`. Leave the prose body that follows it untouched.
- [ ] **Step 3:** Confirm the file now reads: line 1 `# Spec: Build Automation`, blank line, `**Status**: Active`, blank line, `## Purpose`, blank line, existing prose.
- [ ] **Step 4:** Visual diff: only the `## Overview` → `## Purpose` change should appear.

## Task 2: Backfill `windows-vs-detection/spec.md`

- [ ] **Step 1:** Open `openspec/specs/windows-vs-detection/spec.md`
- [ ] **Step 2:** Delete the line `**Status**: Delta (new capability)` (and the surrounding blank line so spacing stays clean).
- [ ] **Step 3:** Replace the line `## Overview` with `## Purpose`. Leave the prose body untouched.
- [ ] **Step 4:** Visual diff: two changes only — the deleted Status line and the renamed heading.

## Task 3: Backfill `timing-output/spec.md`

- [ ] **Step 1:** Open `openspec/specs/timing-output/spec.md`
- [ ] **Step 2:** Replace the title `# timing-output Specification` with `# Spec: Timing Output`.
- [ ] **Step 3:** Under `## Purpose`, replace the placeholder line `TBD - created by archiving change add-timing-output. Update Purpose after archive.` with a 1–3 sentence Purpose distilled from the spec's requirements and the archived change at `openspec/changes/archive/2026-05-09-add-timing-output/`. Suggested content: describe the opt-in `--timing` flag, the per-phase millisecond breakdown emitted to stderr, and that the spec covers both success and failure timing emission.
- [ ] **Step 4:** Confirm the new Purpose contains no `SHALL`/`MUST`, no `TBD`/`TODO`, and is between 1 and 3 sentences.

## Task 4: Backfill `compiler-extra-args/spec.md`

- [ ] **Step 1:** Open `openspec/specs/compiler-extra-args/spec.md`
- [ ] **Step 2:** Prepend two new sections at the top of the file (above the existing `### Requirement:` blocks):
  - Line 1: `# Spec: Compiler Extra Args`
  - Followed by `## Purpose` and a 1–3 sentence body. Suggested content: describe that the spec covers the `--` separator on the CLI for forwarding arbitrary args to a gcc-like compiler invocation, and that it defines when extra args are silently ignored versus warned about.
- [ ] **Step 3:** Verify the existing `### Requirement: Compiler extra args passthrough` and subsequent blocks remain unchanged below the new sections.

## Task 5: Backfill `release-workflow-macos/spec.md`

- [ ] **Step 1:** Open `openspec/specs/release-workflow-macos/spec.md`
- [ ] **Step 2:** Prepend at the top of the file:
  - `# Spec: Release Workflow (macOS)`
  - `## Purpose` and a 1–3 sentence body. Suggested content: describe that the spec covers the GitHub Actions workflow that builds and uploads `x86_64-apple-darwin` and `aarch64-apple-darwin` release binaries on tag pushes matching `v*`.
- [ ] **Step 3:** Verify the existing `### Requirement:` blocks remain unchanged below.

## Task 6: Backfill `release-workflow-ubuntu/spec.md`

- [ ] **Step 1:** Open `openspec/specs/release-workflow-ubuntu/spec.md`
- [ ] **Step 2:** Prepend at the top of the file:
  - `# Spec: Release Workflow (Ubuntu)`
  - `## Purpose` and a 1–3 sentence body. Suggested content: describe that the spec covers the GitHub Actions job that builds the `x86_64-unknown-linux-gnu` release binary and uploads it as a release asset alongside the macOS artifacts, plus the shared `create-release` job ordering.
- [ ] **Step 3:** Leave the existing `## ADDED Requirements` heading and everything below it exactly as-is. The leaked delta marker is a known follow-up and is explicitly out of scope here.

## Task 7: Verify `spec-format-conventions` compliance for all six edited specs

- [ ] **Step 1:** For each of the six files in tasks 1–6, confirm the four checks listed in `tasks.md` section 2 (title format, Purpose first, Purpose content rules, no `## Overview`).
- [ ] **Step 2:** A quick scriptable sanity check:
  ```sh
  for f in openspec/specs/*/spec.md; do
    head -1 "$f"
    grep -nE '^## (Purpose|Overview)' "$f"
    echo "---"
  done
  ```
  Every file's first line should match `# Spec: ...`. The grep should show a `## Purpose` line and no `## Overview` line.

## Task 8: Validate the change

- [ ] **Step 1:** Run `openspec validate backfill-spec-purposes --type change`. Expect: `Change 'backfill-spec-purposes' is valid`.
- [ ] **Step 2:** Run `openspec validate --specs --strict`. Expect: all specs report valid (or, if a baseline spec is found non-compliant in a way unrelated to this change — e.g., the FR-N format in `windows-vs-detection` or the `## ADDED Requirements` in `release-workflow-ubuntu` — surface the failure to the user; do not attempt to fix it in this change).
- [ ] **Step 3:** Run `openspec status --change backfill-spec-purposes`. Confirm every artifact in `applyRequires` is `done`.

## Task 9: Commit

- [ ] **Step 1:** Stage only the six touched baseline spec files plus the entire `openspec/changes/backfill-spec-purposes/` directory.
- [ ] **Step 2:** Commit using a Conventional Commit message: `docs(specs): backfill Purpose sections and standardize titles`.
- [ ] **Step 3:** Do NOT push. Do NOT archive yet — that happens in a separate `/opsx:verify` and `/opsx:archive` step after the user reviews.
