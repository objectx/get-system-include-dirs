## Design Summary

Backfill a consistent `## Purpose` section into every baseline spec at `openspec/specs/<capability>/spec.md`, and standardize the top-level header to `# Spec: <Title Case Capability>`. Purpose for each spec is a 1–3 sentence "what is this capability and why does it exist", distilled from the spec's own requirements and (when one exists) the archived change folder for that capability.

The change is documentation-only. No Rust source, no Justfile recipe, and no requirement/scenario text is touched.

Affected specs (six in total):

| Spec | Current state | Action |
|------|---------------|--------|
| `build-automation` | `# Spec: Build Automation`, `## Overview` | Rename `## Overview` → `## Purpose`; keep prose. |
| `windows-vs-detection` | `# Spec: Windows Visual Studio Detection`, `## Overview` | Rename `## Overview` → `## Purpose`; keep prose; drop the now-stale `**Status**: Delta (new capability)` line. |
| `timing-output` | `# timing-output Specification`, `## Purpose` (TBD placeholder) | Rewrite top header to `# Spec: Timing Output`; replace TBD with a real Purpose. |
| `compiler-extra-args` | No header, no Purpose, dives into `### Requirement:` | Prepend `# Spec: Compiler Extra Args` + `## Purpose`. |
| `release-workflow-macos` | No header, no Purpose | Prepend `# Spec: Release Workflow (macOS)` + `## Purpose`. |
| `release-workflow-ubuntu` | No header, opens with `## ADDED Requirements` (delta marker leaked into baseline) | Prepend `# Spec: Release Workflow (Ubuntu)` + `## Purpose`. The bogus `## ADDED Requirements` heading is flagged as a follow-up; not fixed here. |

## Alternatives Considered

### Approach A: Standardize on `## Purpose` (selected)
- **Approach**: Use `## Purpose` everywhere. Rename existing `## Overview` blocks. Replace TBD content with a real Purpose. Standardize the top-level header to `# Spec: <Title Case>`.
- **Pros**: Matches what `openspec archive` auto-generates; aligns with the change name; converges the two heterogeneous styles into one.
- **Cons**: Touches existing specs that are not broken (cosmetic rename).
- **Why chosen**: One vocabulary for one concept. Future archives will keep adding `## Purpose` placeholders, so picking that name minimizes future drift.

### Approach B: Standardize on `## Overview`
- **Approach**: Keep what two of the existing specs already use. Rewrite the timing-output TBD as `## Overview`. Each future auto-generated `## Purpose` placeholder gets renamed during backfill.
- **Pros**: Touches fewer existing files.
- **Cons**: Permanently fights the OpenSpec tool — every archive adds a `## Purpose` that must be renamed. The change name and the placeholder text both speak "Purpose."
- **Why not**: Asymmetry with the tool guarantees recurring rework.

### Approach C: Minimal touch — fill in TBDs only, leave headers heterogeneous
- **Approach**: Only replace the `TBD` text in `timing-output` and prepend a Purpose to the four specs that have none. Do not rename existing `## Overview` blocks. Do not touch top-level headers.
- **Pros**: Smallest possible diff.
- **Cons**: Leaves the codebase with two names for the same section indefinitely. Future readers can't tell which is canonical.
- **Why not**: The point of a "backfill" change is to converge — leaving the divergence defeats the purpose.

## Agreed Approach

**Approach A — standardize on `## Purpose`.** Header order in every spec is fixed as: `# Spec: <Title Case>` → `## Purpose` → `## Requirements` (or directly into `### Requirement:` blocks where there is no wrapper, matching current style of each spec).

Purpose text rules:
- 1–3 sentences. Answers: *what capability is this, and why does it exist*.
- Derived from the spec's own requirements and, where one exists, the matching archived change folder.
- No implementation detail leakage; that lives in the requirement text.

## Key Decisions

1. **Section name is `## Purpose`**, not `## Overview`. (User confirmed.)
2. **Top-level header is `# Spec: <Title Case Capability>`** — matches `build-automation` and `windows-vs-detection`. Replaces `# timing-output Specification` and the missing top header in three other specs.
3. **No requirement or scenario text changes.** Renames and prepends only.
4. **`release-workflow-ubuntu`'s `## ADDED Requirements` heading is left in place.** Removing it is a separate semantic fix (delta marker → baseline), beyond the scope of "backfill purposes." Flagged in proposal as a follow-up.
5. **`windows-vs-detection`'s `**Status**: Delta (new capability)` line is dropped** when renaming its `## Overview` → `## Purpose`. The capability has long since been added; the line is stale and would be confusing to leave under `## Purpose`.

## Scope Adjustment (Post-Validation)

After scaffolding the change, `openspec validate backfill-spec-purposes --type change` rejected the artifact with `Change must have at least one delta`. The OpenSpec model treats a change without requirement deltas as ill-formed. Pure documentation backfills do not naturally fit.

Resolution: introduce a new baseline capability `spec-format-conventions` whose requirements codify the conventions this change is converging on (title header format, Purpose section presence and position, Purpose content rules). The "backfill" of the six existing specs becomes the implementation work that brings the codebase into compliance with the new conventions capability.

This is a strict improvement over the original framing: instead of a one-shot edit that drifts again next quarter, the conventions are now a durable, testable specification that future specs are checked against.

## Open Questions

None blocking. The two known follow-ups (release-workflow-ubuntu's leaked delta marker; windows-vs-detection's non-OpenSpec FR-N requirement format) are explicitly out of scope and recorded in the proposal so they aren't lost.
