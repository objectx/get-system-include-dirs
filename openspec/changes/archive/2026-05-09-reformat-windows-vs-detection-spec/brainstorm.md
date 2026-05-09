## Design Summary

Reformat `openspec/specs/windows-vs-detection/spec.md` from the current `### Functional Requirements / #### FR-1: ... / **Priority**: MUST / **Acceptance Criteria**: ...` shape into OpenSpec's standard `### Requirement: <name>` + inline `SHALL/MUST` + `#### Scenario:` shape. Same semantics, different surface form. Closes the last `openspec validate --specs --strict` gap (currently 6/7; post-fix 7/7).

## Alternatives Considered

### Approach A: Full reformat (chosen)

- **Make**: Rewrite each FR-N / NFR-N as a top-level `### Requirement: <name>` block whose statement contains `SHALL` or `MUST` inline. Convert each "Acceptance Criteria" bullet into a `#### Scenario:` block (or fold into the requirement statement when the bullet is a constraint, not a behavior).
- **Pros**: Closes the strict-validation gap durably; aligns windows-vs-detection with the other six specs; the convention encoded in `spec-format-conventions` then describes 7/7 of the corpus.
- **Cons**: Most edit volume of the three approaches — six FRs × N acceptance criteria each. Cross-cutting reread of "Behavior Specifications" Scenarios 1-5 (bottom of file) to avoid duplicating coverage.
- **Why chosen**: Only approach that actually fixes the parser failure. The `**Priority**: MUST` line doesn't satisfy the strict checker because the parser sees `### Functional Requirements` / `### Non-Functional Requirements` as the two requirements, with the FR/NFR blocks parsed as scenarios — that's why exactly two errors fire.

### Approach B: Minimal patch — promote FR/NFR to `###`, leave "Acceptance Criteria" bullets in place

- **Make**: Demote `### Functional Requirements` / `### Non-Functional Requirements` to plain prose, promote `#### FR-N: ...` to `### Requirement: <name>`, and prefix each requirement statement with `The system SHALL ...`.
- **Pros**: Smaller diff. Preserves the FR-N numbering for traceability.
- **Cons**: Doesn't satisfy the second half of the convention — `#### Scenario:` blocks are still missing. Would still fail strict validation on `Each requirement MUST include at least one #### Scenario: block`. Forces a half-step that needs another pass later.
- **Why not**: Splits the work into two passes for no win. If we're going to touch every requirement, finish the conversion in one go.

### Approach C: Roll the convention encoding into `spec-format-conventions` only, defer the windows-vs-detection edit

- **Make**: Add a 6th requirement to `spec-format-conventions` ("Requirement statements include SHALL or MUST keyword inline"), but leave windows-vs-detection's surface form as-is for now.
- **Pros**: Captures the convention in one of the smallest possible deltas. Future drift is caught even if the existing violation lingers.
- **Cons**: Codifies a rule the corpus doesn't comply with — `--strict` still fires on windows-vs-detection. The convention then exists but is immediately violated by the spec it should describe. Smells.
- **Why not**: Encoding a convention you can't honor today is worse than no convention. Approach A makes the convention true everywhere.

## Agreed Approach

**Approach A** — full reformat. The strict-validation failure is real (two errors, exactly the two synthetic top-level requirements the parser sees), and fixing it requires both `### Requirement: <name>` blocks AND `#### Scenario:` blocks, so the half-step in B saves nothing. C is the wrong polarity for a convention.

The reformat is mechanical but not trivial: six FRs and three NFRs, each with a multi-bullet "Acceptance Criteria" list, plus a separate "Behavior Specifications" section near the bottom (Scenarios 1-5 in narrative form) that overlaps with the FR-level coverage. Rule for Scenarios 1-5: leave them alone unless a reformatted requirement makes them strictly redundant. They're prose, not validator-relevant.

## Key Decisions

1. **Flatten FR/NFR distinction.** OpenSpec convention is a flat list of `### Requirement:` blocks. The functional vs. non-functional grouping is editorial; collapse it. Use the requirement name to convey the flavor where it matters (e.g., "x64 architecture only", "Platform: Windows-only").
2. **Acceptance Criteria bullets → Scenarios where they describe behavior, fold inline where they describe constraints.** Constraint bullets like "Always use `-arch=x64` flag with vsdevcmd.bat" belong inside the requirement statement. Behavior bullets like "When attempt 1 returns results, use the first result and stop" become a `#### Scenario:`.
3. **Scenario syntax.** Follow the convention used in `timing-output/spec.md` and `release-workflow-ubuntu/spec.md` (post-fix): `#### Scenario: <name>` then `- **WHEN** ...` / `- **THEN** ...` bullets.
4. **Preserve all current behavior.** No semantic changes. If a scenario in the new form would describe behavior not currently in the spec, that's a separate change — not this one.
5. **Leave "Interface Specifications", "Dependencies", "Behavior Specifications", "Testing Requirements", and "Future Considerations" sections at the bottom alone.** They are reference material, not validator-relevant.
6. **The change's `specs/windows-vs-detection/spec.md` uses `## MODIFIED Requirements` deltas** — six FRs and three NFRs all become MODIFIED. Apply phase copies the new content to baseline.
7. **Docs-only override applies** (per `CLAUDE.md → Documentation-only changes need a meta-conventions capability`): direct in-session edits, no worktree dispatch, no per-task subagents.

## Open Questions

None blocking. Two minor judgment calls left for the apply phase:

- Whether to fold or split each Acceptance Criteria bullet — decided per-bullet during writing, not upfront.
- Whether NFR-3 ("Performance") survives as a `### Requirement:` block. It uses `SHOULD`, which the strict validator accepts, but has no testable scenario. Likely keep as a single requirement with a "best-effort" framing scenario, or drop entirely (it's aspirational). Decide during apply.
