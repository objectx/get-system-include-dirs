## Why

The six baseline specs in `openspec/specs/` use three different conventions for the prose blurb that introduces a capability: two use `## Overview`, one uses a TBD placeholder under `## Purpose` (left over from `openspec archive`), and three have no introductory section at all. The top-level header is similarly inconsistent: two specs use `# Spec: <Title>`, one uses `# <name> Specification`, and three have no top header. New readers (human and tooling) cannot tell at a glance which capability a spec covers or whether the file is complete. `openspec archive` will keep generating `## Purpose` placeholders, so the divergence will recur unless we converge now on the tool's own vocabulary.

## What Changes

**Top-level header**
- From: heterogeneous (`# Spec: <Title>`, `# <name> Specification`, or missing)
- To: `# Spec: <Title Case Capability>` for every baseline spec
- Reason: one canonical title format; matches the most common existing style
- Impact: documentation only; non-breaking

**Introductory section**
- From: `## Overview` in two specs, `## Purpose` (TBD placeholder) in one, absent in three
- To: `## Purpose` in every spec, populated with a 1–3 sentence description distilled from the spec's own requirements (and the matching archived change, when one exists)
- Reason: aligns with the placeholder `openspec archive` already inserts; resolves the TBD; gives every spec the same shape
- Impact: documentation only; non-breaking

**Stale annotation in `windows-vs-detection`**
- From: `**Status**: Delta (new capability)` line beneath the title
- To: removed (the capability was added long ago; the line is no longer accurate and would read awkwardly under `## Purpose`)
- Reason: avoid stale metadata
- Impact: documentation only; non-breaking

**Out of scope (recorded so they aren't forgotten)**
- `release-workflow-ubuntu/spec.md` opens with `## ADDED Requirements` — a delta marker that has leaked into a baseline spec. Fix is a separate change (semantic, not cosmetic).
- `windows-vs-detection/spec.md` uses an FR-N / "Acceptance Criteria" requirement format instead of OpenSpec's `### Requirement:` + `#### Scenario:` shape. A reformat is a separate, larger change.

## Capabilities

### New Capabilities

- `spec-format-conventions`: codifies the structural conventions every baseline spec at `openspec/specs/<capability>/spec.md` MUST follow — title header format, Purpose section presence and position, and Purpose content rules. The backfill of the six existing specs is the implementation work that brings the codebase into compliance with this capability.

### Modified Capabilities

None at the requirement level. Six existing baseline specs are touched as part of implementing the new `spec-format-conventions` capability, but their requirements are unchanged. Their edits are recorded in `tasks.md`, not as spec deltas:

- `build-automation`: rename `## Overview` → `## Purpose`; keep prose
- `compiler-extra-args`: prepend `# Spec:` header + `## Purpose`
- `release-workflow-macos`: prepend `# Spec:` header + `## Purpose`
- `release-workflow-ubuntu`: prepend `# Spec:` header + `## Purpose` (above the existing `## ADDED Requirements` line, which is left untouched in this change)
- `timing-output`: rewrite top header to `# Spec: Timing Output`; replace TBD content under `## Purpose`
- `windows-vs-detection`: rename `## Overview` → `## Purpose`; drop stale `**Status**: Delta (new capability)` line

## Impact

- **Files touched**: six `openspec/specs/<capability>/spec.md` files. No source code, no Justfile, no CI workflow, no Cargo.toml.
- **Tooling**: future runs of `openspec archive` will continue to insert `## Purpose` placeholders; this change is the precedent that future authors fill them in immediately.
- **Risk**: low. Documentation-only edits with a clear visual diff per file. Verification is `openspec status` plus visual inspection.
