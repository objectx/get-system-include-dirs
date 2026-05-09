## 1. Implement the convention compliance

- [x] 1.1 `openspec/specs/release-workflow-ubuntu/spec.md`: rename the line `## ADDED Requirements` to `## Requirements`. Leave every `### Requirement:` block and `#### Scenario:` block below the header untouched.

## 2. Verify

- [x] 2.1 Run `openspec validate release-workflow-ubuntu --type spec --strict` and confirm it reports the spec as valid (was failing pre-change with "Spec must have a Requirements section").
- [x] 2.2 Run `openspec validate fix-release-workflow-ubuntu-spec --type change` and confirm the change reports as valid.
- [x] 2.3 Run `openspec validate --specs --strict` and confirm 6 of 7 baseline specs now pass strict (was 5/7 pre-change). The single remaining failure SHALL be `windows-vs-detection`, which is the deferred FR-N reformat follow-up.
