# Tasks

## 1. Rewrite the baseline windows-vs-detection spec

- [x] **1.1** Replace the `## Requirements` body of `openspec/specs/windows-vs-detection/spec.md` with the nine `### Requirement: <name>` blocks defined in `specs/windows-vs-detection/spec.md` ADDED Requirements. Drop the `### Functional Requirements` and `### Non-Functional Requirements` subheadings entirely; the new structure is a flat list under `## Requirements`.
- [x] **1.2** Preserve the title (`# Spec: Windows Visual Studio Detection`), the `## Purpose` section, and every section after the requirements block (`## Interface Specifications`, `## Dependencies`, `## Behavior Specifications`, `## Testing Requirements`, `## Future Considerations (Out of Scope)`) verbatim.
- [x] **1.3** Within each new `### Requirement:` block, ensure the requirement statement contains `SHALL` or `MUST` inline (per the strict validator) and ensure at least one `#### Scenario:` block follows it.

## 2. Validate

- [x] **2.1** Run `openspec validate windows-vs-detection --type spec --strict` and confirm zero errors. → **PASS** (zero errors).
- [x] **2.2** Run `openspec validate --specs --strict` and confirm 7/7 specs pass (was 6/7 before). → **PASS** (7/7).
- [x] **2.3** Run `openspec validate reformat-windows-vs-detection-spec --type change --strict` and confirm the change still validates after the baseline edit. → **PASS**.

## 3. Sanity-check the rewrite

- [x] **3.1** Diff the rewritten baseline spec against the pre-rewrite version: confirm no semantic changes (every FR-N / NFR-N requirement is represented; behavior unchanged; only surface form differs). → All nine requirements present (`grep -c '^### Requirement:' = 9`); FR-1 → `$INCLUDE precedence`, FR-2 → `vswhere two-attempt detection`, FR-3 → `Version filtering via --vs-version`, FR-4 → `vsdevcmd INCLUDE capture`, FR-5 → `INCLUDE value parsing`, FR-6 → `Detailed VS-detection errors`, NFR-1 → `Platform: Windows-only`, NFR-2 → `Architecture: x64 only`, NFR-3 → `Performance budget`. Acceptance Criteria bullets either folded into requirement statements (constraints) or promoted to `#### Scenario:` blocks (behaviors). No semantic change.
- [x] **3.2** Spot-check that the `## Behavior Specifications` Scenarios 1-5 (narrative form) at the bottom of the file are still present and unmodified. → All five scenarios preserved at the bottom of the file in their original `Given/When/Then/And` prose form.
