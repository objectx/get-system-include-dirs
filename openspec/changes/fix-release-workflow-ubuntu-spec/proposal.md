## Why

`openspec/specs/release-workflow-ubuntu/spec.md` opens with `## ADDED Requirements` — a delta-operation marker that belongs only in change-spec deltas at `openspec/changes/<name>/specs/<capability>/spec.md`, not in baseline specs. The leak is a residue of an old archive cycle that didn't promote the delta header to the baseline `## Requirements` form. `openspec validate release-workflow-ubuntu --type spec --strict` rejects it with "Spec must have a Requirements section."

This was flagged as an out-of-scope follow-up in the `backfill-spec-purposes` proposal and verified there as pre-existing. Now we close it. The fix is one line, but it's framed as a conventions-strengthening change so that future leaks of the same kind get caught by the newly-codified requirement, not merely papered over.

## What Changes

**`spec-format-conventions` capability**
- From: four requirements (Title header format; Purpose section presence and position; Purpose content; Treatment of auto-generated Purpose placeholders)
- To: five requirements — adds "Requirements section header" forbidding `## ADDED / MODIFIED / REMOVED / RENAMED Requirements` markers in baseline specs and mandating `## Requirements`
- Reason: codify the convention so the same kind of leak can be caught by `openspec validate --strict` against the conventions, not merely by the OpenSpec tool's built-in spec-shape check
- Impact: documentation-only; non-breaking. New requirement against a brand-new (last cycle) capability.

**`release-workflow-ubuntu/spec.md` (implementation)**
- From: `## ADDED Requirements` as the header for the requirements section
- To: `## Requirements`
- Reason: bring the spec into compliance with both OpenSpec's built-in spec shape and the new convention requirement above
- Impact: documentation-only; non-breaking. The `### Requirement:` blocks below the header are unchanged.

**Out of scope**
- `windows-vs-detection`'s FR-N "Acceptance Criteria" format remains unaddressed. That is a substantial reformat (six requirements each with multiple sub-sections) and deserves its own cycle. It was carried in the `backfill-spec-purposes` follow-up list and remains there.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `spec-format-conventions`: ADDED Requirement "Requirements section header" (with three scenarios). No requirements removed or modified.

## Impact

- **Files touched**: 1 baseline spec (`release-workflow-ubuntu/spec.md`) + the change folder. No source code, no Justfile, no CI workflow.
- **Strict-validation outcome**: 5/7 → 6/7 specs pass. Only `windows-vs-detection` remains failing (deferred).
- **Cross-cycle consistency**: second consecutive change to use the "encode docs fix as convention requirement" pattern from the `backfill-spec-purposes` retrospective. Pattern is becoming routine, which is the desired outcome.
- **Risk**: low. Single-line markdown edit + one new requirement against a brand-new capability.
