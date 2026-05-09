## Design Summary

Fix the structural defect in `openspec/specs/release-workflow-ubuntu/spec.md` where its requirements section is headed `## ADDED Requirements` (a delta marker that leaked into the baseline spec) instead of `## Requirements`. `openspec validate --strict` rejects the file with "Spec must have a Requirements section."

The fix is a single one-line edit (rename `## ADDED Requirements` → `## Requirements`). Per the rule established in the prior cycle's retrospective and now codified in `CLAUDE.md`, a single docs-only edit cannot stand alone as an OpenSpec change; the validator hard-requires a delta. So the change is framed as **adding a new requirement to `spec-format-conventions`** that explicitly forbids delta markers in baseline specs. The release-workflow-ubuntu rename is the implementation that brings the codebase into compliance with the new requirement.

This is the second time we've used the "encode a docs fix as a convention requirement" pattern. After this cycle, the `spec-format-conventions` capability gains a fourth (well, fifth) requirement and another previously-failing spec joins the strict-validation pass list.

## Alternatives Considered

### Approach A: Add a `Requirements section header` requirement to `spec-format-conventions` (selected)
- **Approach**: ADDED Requirements delta against `spec-format-conventions` introducing one new requirement (with three scenarios) that mandates `## Requirements` and forbids the four delta markers in baseline specs. Implementation task renames the leaked header in `release-workflow-ubuntu/spec.md`.
- **Pros**: Honest delta (the convention is being strengthened); validator passes; future leaked markers are caught by the new convention; consistent with the last cycle's pattern.
- **Cons**: Slightly heavier than a one-line fix would suggest. The convention requirement is so narrow it almost reads as "validator output, in spec form."
- **Why chosen**: Same reasoning as the prior cycle — encoding the convention as deltas is durable; fabricating no-op stubs is not.

### Approach B: Add the requirement to `spec-format-conventions` AND fix the windows-vs-detection FR-N format in the same change
- **Approach**: Bundle both pre-existing strict failures into one cycle.
- **Pros**: One PR clears both follow-ups.
- **Cons**: The two failures have nothing in common. The Ubuntu fix is a one-line header rename. The windows-vs-detection fix is a substantial reformat (FR-1 through FR-6 each with Acceptance Criteria → `### Requirement:` + `#### Scenario:` shape). Bundling them violates the "scope = one cohesive idea" principle and makes review harder.
- **Why not**: Two distinct defects deserve two distinct changes. This cycle handles only Ubuntu.

### Approach C: Just rename the header, file as a `chore:` direct PR, skip OpenSpec
- **Approach**: One-line edit, no change folder.
- **Pros**: Smallest possible footprint.
- **Cons**: The repo's `CLAUDE.md` "Workflow routing" section says external contracts / schema / cross-system integration → opsx; bug fixes → direct PR. This is arguably a "bug fix" (validator failure) and not a contract change. So Approach C is permissible.
- **Why not**: Even though direct-PR is permissible, going through opsx lets us strengthen the conventions capability at the same time, which is more valuable than the saved process overhead. The strengthened convention prevents recurrence; a direct PR fixes one instance.

## Agreed Approach

**Approach A.** Add one new requirement to `spec-format-conventions` (ADDED delta), then implement by renaming the leaked header in `release-workflow-ubuntu/spec.md`.

After archive: `spec-format-conventions` will have a new "Requirements section header" requirement; release-workflow-ubuntu will join the strict-validation pass list (4/7 → 5/7 unrelated to this one fix wait — let me recount: post-prior-cycle 5/7 specs pass, 2 fail. After this cycle: 6/7 pass, 1 fails. Then the windows-vs-detection follow-up cycle will close the last one).

## Key Decisions

1. **The new requirement is named "Requirements section header"** — describes what the convention is.
2. **The requirement forbids all four delta markers** (`## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, `## RENAMED Requirements`), not just `## ADDED Requirements`. The leak we're fixing is one instance of a class.
3. **The fix is in-scope**; the windows-vs-detection FR-N reformat is explicitly NOT included (separate change, separate cycle).
4. **No edits to the `release-workflow-ubuntu` requirements themselves** — only the section header is renamed. Requirement text is untouched.

## Open Questions

None blocking. The validate-empty-scaffold rule (added to CLAUDE.md from the prior cycle) was applied immediately and confirmed the no-delta constraint before any artifact drafting.
