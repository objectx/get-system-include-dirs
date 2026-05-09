# spec-format-conventions Specification

## Purpose
TBD - created by archiving change backfill-spec-purposes. Update Purpose after archive.
## Requirements
### Requirement: Title header format

Every baseline spec at `openspec/specs/<capability>/spec.md` SHALL begin with a level-1 markdown header of the form `# Spec: <Title Case Capability Name>`. The title SHALL be the human-readable, title-cased rendering of the capability folder name.

#### Scenario: Single-word capability has a title-cased Spec header

- **WHEN** a baseline spec exists at `openspec/specs/build-automation/spec.md`
- **THEN** the first line of the file SHALL be `# Spec: Build Automation`

#### Scenario: Multi-word capability with platform qualifier renders parenthesized

- **WHEN** a baseline spec exists at `openspec/specs/release-workflow-ubuntu/spec.md`
- **THEN** the first line of the file SHALL be `# Spec: Release Workflow (Ubuntu)`

#### Scenario: Capability with proper-noun token preserves casing

- **WHEN** a baseline spec exists at `openspec/specs/windows-vs-detection/spec.md`
- **THEN** the first line of the file SHALL be `# Spec: Windows Visual Studio Detection`

### Requirement: Purpose section presence and position

Every baseline spec at `openspec/specs/<capability>/spec.md` SHALL contain a `## Purpose` section, and that section SHALL be the first level-2 heading in the file (immediately following the title header, with no other level-2 sections preceding it).

#### Scenario: Purpose is the first section in a fresh spec

- **WHEN** a baseline spec is opened
- **THEN** the first `##` heading encountered SHALL be `## Purpose`

#### Scenario: Spec without a Purpose section is non-compliant

- **WHEN** a baseline spec at `openspec/specs/<capability>/spec.md` contains no `## Purpose` heading
- **THEN** the spec SHALL be considered non-compliant with these conventions and SHALL be backfilled before the next change against that capability is archived

#### Scenario: Other introductory section names are not used

- **WHEN** a baseline spec is authored or backfilled
- **THEN** the introductory section SHALL be named `## Purpose`
- **THEN** the section SHALL NOT be named `## Overview`, `## Summary`, or any other synonym

### Requirement: Purpose content

The `## Purpose` section SHALL contain a 1–3 sentence description of what capability the spec covers and why it exists. The description SHALL be descriptive prose, not normative requirement text, and SHALL NOT contain placeholder markers.

#### Scenario: Purpose is concise

- **WHEN** the `## Purpose` section is read
- **THEN** its body SHALL contain between 1 and 3 sentences

#### Scenario: Purpose is descriptive, not prescriptive

- **WHEN** the `## Purpose` section is read
- **THEN** its body SHALL NOT contain the words `SHALL` or `MUST` (which belong in `### Requirement:` blocks, not in the introductory description)

#### Scenario: Purpose has no placeholder text

- **WHEN** the `## Purpose` section is read
- **THEN** its body SHALL NOT contain `TBD`, `TODO`, `FIXME`, or any text indicating the section is unfinished

### Requirement: Treatment of auto-generated Purpose placeholders

When `openspec archive` creates a baseline spec from a change, the resulting `## Purpose` section may contain a placeholder of the form `TBD - created by archiving change <name>. Update Purpose after archive.`. The author of the next change to touch that capability SHALL replace the placeholder with compliant Purpose content (per the previous requirement) before that next change is archived.

#### Scenario: Placeholder Purpose is replaced on next change

- **WHEN** a baseline spec contains an auto-generated TBD `## Purpose` placeholder
- **AND** a new change is being authored that touches the same capability
- **THEN** the change SHALL include a task to replace the placeholder with compliant Purpose content

#### Scenario: Placeholder is not allowed to persist after archive

- **WHEN** a change is being archived
- **AND** that change touched a capability whose `## Purpose` was a TBD placeholder before the change
- **THEN** the archive SHALL NOT proceed until the placeholder has been replaced with compliant Purpose content

