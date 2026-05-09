# Design

The full design — architecture, key decisions, apply-mode override, risks, and per-task implementation steps — is captured in `plan.md`. This file exists as a pointer so the schema's `design` artifact does not flag as unauthored.

For the verbal-brainstorm record (alternatives considered, agreed approach, key decisions), see `brainstorm.md`.

## Quick links

- [`brainstorm.md`](brainstorm.md) — design summary, three-approach trade-off (fine-grained / coarse / direct PR), agreed approach (fine-grained `license-conventions` capability), seven key decisions.
- [`plan.md`](plan.md) — implementation plan with apply-mode override paragraph, nine tasks (1: delta spec, 2: validate, 3-6: apply, 7: verify, 8: sync+archive, 9: commit), and two risk sections (archive auto-apply conflict, Apache-2.0 byte-equivalence drift).
- [`specs/license-conventions/spec.md`](specs/license-conventions/spec.md) — the delta with `## ADDED Requirements` containing the four convention requirements.
