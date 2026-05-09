# Verification Report

**Change**: `fix-release-workflow-ubuntu-spec`
**Verified at**: `2026-05-10 01:30`
**Verifier**: Claude (Opus 4.7, in-session direct apply)

---

## 1. Structural Validation (`openspec validate --all --json`)

- [x] Change is valid
- [x] All baseline specs touched by this change are valid
- [ ] All baseline specs in repo are valid (one pre-existing follow-up remains)

**Result**: 7 of 8 items valid (1 change + 6 specs pass; 1 spec fails with a pre-existing structural defect explicitly out of scope for this change).

| Item | Type | Issues |
|---|---|---|
| `fix-release-workflow-ubuntu-spec` | change | — (valid) |
| `build-automation` | spec | — (valid) |
| `compiler-extra-args` | spec | — (valid) |
| `release-workflow-macos` | spec | — (valid) |
| `release-workflow-ubuntu` | spec | — (valid; **was failing pre-change** with `Spec must have a Requirements section`. The fix from this change makes it pass.) |
| `spec-format-conventions` | spec | — (valid) |
| `timing-output` | spec | — (valid) |
| `windows-vs-detection` | spec | `Requirement must contain SHALL or MUST keyword` (×2). **Pre-existing**, FR-N "Acceptance Criteria" format. Last open follow-up from `backfill-spec-purposes` proposal; explicitly out of scope here. |

Strict-validation pass count: **5/7 → 6/7** as a direct result of this change. Only `windows-vs-detection` remains.

---

## 2. Task Completion (`tasks.md`)

- [x] All `- [ ]` are `- [x]` (4 of 4)

**Unfinished tasks**: none.

| Task | 未完成原因 | 是否阻塞 archive |
|---|---|---|
| — | — | — |

---

## 3. Delta Spec Sync State

| Capability | Sync 狀態 | 備註 |
|---|---|---|
| `spec-format-conventions` | ✗ 待 sync | This change adds one new requirement (`Requirements section header`) to the existing `spec-format-conventions` capability via an `## ADDED Requirements` delta. `openspec archive` will merge it into the baseline at `openspec/specs/spec-format-conventions/spec.md`. |

---

## 4. Design / Specs Coherence Spot Check

`design.md` was intentionally skipped (not in `applyRequires`; the rationale is captured in `brainstorm.md` and `proposal.md`, both of which converge on the same single design). Coherence is checked against `brainstorm.md`.

| 抽樣項 | brainstorm / proposal 描述 | specs 對應 | 差距 |
|---|---|---|---|
| New requirement codifies the fix | "Add one new requirement … forbids `## ADDED / MODIFIED / REMOVED / RENAMED Requirements` markers in baseline specs and mandates `## Requirements`" | `spec-format-conventions` ADDED Requirement: "Requirements section header" with three scenarios (baseline uses `## Requirements`; delta-marker leak is non-compliant; openspec validate flags the leak) | None |
| Implementation scope | "Single one-line edit to `release-workflow-ubuntu/spec.md`" | tasks.md §1.1 — header rename only, all `### Requirement:` blocks below preserved | None |
| Out-of-scope follow-up explicit | "windows-vs-detection FR-N reformat remains unaddressed" | proposal.md "Out of scope" subsection | None |

**漂移警告**（非阻塞）: 無.

---

## 5. Implementation Signal

- [ ] Worktree 內無未 staged 的檔案
- [ ] 所有相關 commit 已推送

**Worktree state**: 1 modified baseline spec file + 1 untracked change directory.

```
 M openspec/specs/release-workflow-ubuntu/spec.md
?? openspec/changes/fix-release-workflow-ubuntu-spec/
```

**Commit 範圍**: none yet. Same posture as the prior cycle — implementation is real and reviewable, but uncommitted because CLAUDE.md prohibits committing without explicit user instruction. The user has now (this turn) asked to commit + archive in one pass; commit will follow this verify file.

**Blocker for archive**: yes — `openspec archive` requires the change to be committed. Will be resolved in the same turn.

---

## 6. Front-Door Routing Leak Detector (warning, 非阻塞)

- [x] 無檔案

```bash
$ ls docs/superpowers/specs/*.md 2>/dev/null
(no matches)
```

**洩漏清單**: 無.

---

## 7. Deferred Manual Dogfood vs Automated Test Equivalence

`plan.md` contains zero `[~]` deferred rows.

| Deferred dogfood (plan §) | Equivalent automated test | Coverage assessment | 真正 gap? |
|---|---|---|---|
| — (no `[~]` rows in plan) | — | — | — |

Section is intentionally empty per the schema's "整節空白" rule.

---

## Overall Decision

- [ ] ✅ PASS
- [x] ⚠️ PASS WITH WARNINGS — implementation is verified, but uncommitted at write time (will be committed in the same turn as part of the user's roll-through request); one pre-existing strict-validation failure remains (`windows-vs-detection`, deferred to its own cycle).
- [ ] ❌ FAIL

**Warnings**:

1. **One pre-existing strict-validation failure persists** in `windows-vs-detection` (FR-N format without inline SHALL/MUST). This is the last remaining out-of-scope follow-up from `backfill-spec-purposes`. Recommend opening it as a separate cycle once the current commit/archive lands.
2. **Implementation is uncommitted at verify write time.** This is a process artifact of writing verify before commit per the schema's "write while context is hot" rule; the commit follows immediately in this turn.

**下一步**:

1. Write `retrospective.md` (next, in this turn).
2. Single commit covering the spec edit + the change directory + verify.md + retrospective.md. Suggested message: `docs(opsx): forbid delta markers in baseline specs and fix release-workflow-ubuntu`.
3. `openspec archive fix-release-workflow-ubuntu-spec -y`.
4. Second commit covering the archive movement + sync into `openspec/specs/spec-format-conventions/spec.md`.
