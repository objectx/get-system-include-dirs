## Why

`openspec validate windows-vs-detection --type spec --strict` reports two `Requirement must contain SHALL or MUST keyword` errors. The cause: the spec uses an `### Functional Requirements / #### FR-N: <name> / **Priority**: MUST / **Acceptance Criteria**: ...` shape, so the parser sees only two top-level requirements (`### Functional Requirements`, `### Non-Functional Requirements`) — both lacking inline SHALL/MUST and lacking `#### Scenario:` blocks. The repo has been at 6/7 strict-pass since the `fix-release-workflow-ubuntu-spec` cycle; this change closes the final gap to 7/7. The convention now encoded in `spec-format-conventions` then describes the entire baseline corpus instead of describing 6/7 of it.

## What Changes

**`openspec/specs/windows-vs-detection/spec.md` requirement structure**

- From: `### Functional Requirements` group with `#### FR-1` through `#### FR-6` blocks, each using `**Priority**: <level>` + `**Acceptance Criteria**:` bullets; `### Non-Functional Requirements` group with `#### NFR-1` through `#### NFR-3` blocks in the same shape.
- To: A flat list of `### Requirement: <name>` blocks at H3, each whose statement contains `SHALL` (or `MUST`) inline, each followed by one or more `#### Scenario: <name>` blocks using `- **WHEN** ...` / `- **THEN** ...` bullets.
- Reason: Strict-validation compliance and corpus consistency.
- Impact: Non-breaking. Surface form only. Same semantics. The "Interface Specifications", "Dependencies", "Behavior Specifications", "Testing Requirements", and "Future Considerations" sections are unchanged.

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `windows-vs-detection`: All nine requirements (six FRs + three NFRs) reformatted to OpenSpec's standard shape. No semantic deltas — surface form only.

## Impact

- **Specs**: `openspec/specs/windows-vs-detection/spec.md` rewritten in the standard shape.
- **Code**: None.
- **Tests**: None — no behavior change.
- **CI**: None.
- **Validation gate**: `openspec validate --specs --strict` will pass for `windows-vs-detection` after apply, bringing the repo to 7/7.
