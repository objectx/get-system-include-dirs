## 1. Backfill `## Purpose` in existing baseline specs

- [x] 1.1 `openspec/specs/build-automation/spec.md`: rename `## Overview` → `## Purpose`; keep prose
- [x] 1.2 `openspec/specs/windows-vs-detection/spec.md`: rename `## Overview` → `## Purpose`; drop the `**Status**: Delta (new capability)` line; keep prose
- [x] 1.3 `openspec/specs/timing-output/spec.md`: rewrite top header from `# timing-output Specification` to `# Spec: Timing Output`; replace TBD content under `## Purpose` with a 1–3 sentence description distilled from the spec's requirements
- [x] 1.4 `openspec/specs/compiler-extra-args/spec.md`: prepend `# Spec: Compiler Extra Args` and `## Purpose` (1–3 sentences) above the existing `### Requirement:` blocks
- [x] 1.5 `openspec/specs/release-workflow-macos/spec.md`: prepend `# Spec: Release Workflow (macOS)` and `## Purpose` (1–3 sentences) above the existing `### Requirement:` blocks
- [x] 1.6 `openspec/specs/release-workflow-ubuntu/spec.md`: prepend `# Spec: Release Workflow (Ubuntu)` and `## Purpose` (1–3 sentences) above the existing `## ADDED Requirements` heading; do NOT remove or modify that heading in this change

## 2. Verify each Purpose meets `spec-format-conventions`

For each of the six specs touched in section 1, confirm visually:

- [x] 2.1 First line of file is `# Spec: <Title Case Name>`
- [x] 2.2 First `##` heading after the title is `## Purpose`
- [x] 2.3 Purpose body is 1–3 sentences, contains no `SHALL` or `MUST`, contains no `TBD`/`TODO`/`FIXME`
- [x] 2.4 No `## Overview` heading remains anywhere in the file

## 3. Validate the change as a whole

- [x] 3.1 Run `openspec validate backfill-spec-purposes --type change` and confirm it reports the change as valid
- [x] 3.2 Run `openspec validate --specs --strict` and confirm all baseline specs (including the touched six) are valid — **partial**: 4/6 pass strict (was 1/6 pre-change). The 2 remaining failures (`release-workflow-ubuntu`, `windows-vs-detection`) are pre-existing structural issues explicitly recorded as out-of-scope follow-ups in `proposal.md`. Confirmed pre-existence by running the same validate against the un-edited tree (`git stash`): 5/6 failed before, 2/6 fail after.
- [x] 3.3 Run `openspec status --change backfill-spec-purposes` and confirm `applyRequires` artifacts are all `done`

## 4. Capture the new convention in the project's spec index

- [x] 4.1 Confirm that after archive, `openspec/specs/spec-format-conventions/spec.md` will exist with the requirements from this change's `specs/spec-format-conventions/spec.md` delta (no manual file creation needed — `openspec archive` materializes it)
- [x] 4.2 No edit to `CLAUDE.md` or `openspec/config.yaml` in this change; the conventions are discoverable via the new spec itself
