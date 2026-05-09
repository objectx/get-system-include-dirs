## ADDED Requirements

### Requirement: Requirements section header

Every baseline spec at `openspec/specs/<capability>/spec.md` SHALL have its requirements section headed exactly `## Requirements`. The section SHALL NOT use any of the OpenSpec delta-operation markers (`## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, `## RENAMED Requirements`); those markers are reserved for delta specs at `openspec/changes/<name>/specs/<capability>/spec.md`.

#### Scenario: Baseline spec uses `## Requirements`

- **WHEN** a baseline spec is read
- **THEN** the level-2 heading immediately preceding the first `### Requirement:` block SHALL be exactly `## Requirements`

#### Scenario: Delta marker leaked into a baseline spec is non-compliant

- **WHEN** a baseline spec at `openspec/specs/<capability>/spec.md` contains `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, or `## RENAMED Requirements` as its requirements section header
- **THEN** the spec SHALL be considered non-compliant with these conventions
- **THEN** the leaked delta marker SHALL be replaced with `## Requirements`, with the requirement blocks below the header preserved unchanged

#### Scenario: openspec validate flags the leak

- **WHEN** `openspec validate <capability> --type spec --strict` runs against a baseline spec whose requirements section is headed by a delta marker
- **THEN** the validator SHALL report `Spec must have a Requirements section`
- **THEN** the spec SHALL be brought into compliance by replacing the delta marker with `## Requirements`
