# Verification Report

**Change**: `backfill-spec-purposes`
**Verified at**: `2026-05-10 01:16`
**Verifier**: Claude (Opus 4.7, in-session direct apply)

---

## 1. Structural Validation (`openspec validate --all --json`)

- [x] Change is valid
- [ ] All baseline specs valid

**Result**: 5 of 7 items valid (1 change + 4 specs pass; 2 specs fail with pre-existing structural defects unrelated to this change).

| Item | Type | Issues |
|---|---|---|
| `backfill-spec-purposes` | change | — (valid) |
| `build-automation` | spec | — (valid) |
| `compiler-extra-args` | spec | — (valid) |
| `release-workflow-macos` | spec | — (valid) |
| `release-workflow-ubuntu` | spec | `Spec must have a Requirements section` — caused by the leaked `## ADDED Requirements` baseline marker. **Pre-existing**. Recorded as out-of-scope follow-up in `proposal.md`. |
| `timing-output` | spec | — (valid) |
| `windows-vs-detection` | spec | `Requirement must contain SHALL or MUST keyword` (×2) — caused by the FR-N "Acceptance Criteria" format that predates the OpenSpec convention. **Pre-existing**. Recorded as out-of-scope follow-up in `proposal.md`. |

**Pre-existence proof**: ran `git stash && openspec validate --specs --strict` against the un-edited tree — 5 of 6 specs failed strict before this change. After this change, only the same 2 long-standing structural defects remain. This change improved the strict-pass count from 1/6 → 4/6.

---

## 2. Task Completion (`tasks.md`)

- [x] All `- [ ]` are `- [x]` (15 of 15)

**Unfinished tasks**: none.

| Task | 未完成原因 | 是否阻塞 archive |
|---|---|---|
| — | — | — |

---

## 3. Delta Spec Sync State

| Capability | Sync 狀態 | 備註 |
|---|---|---|
| `spec-format-conventions` | ✗ 待 sync | New capability. Will be materialized at `openspec/specs/spec-format-conventions/spec.md` by `openspec archive`. No manual sync required. |

---

## 4. Design / Specs Coherence Spot Check

`design.md` was intentionally skipped (it is not in `applyRequires`; the rationale is captured in `brainstorm.md` and `proposal.md`). Coherence is checked against `brainstorm.md` instead.

| 抽樣項 | brainstorm.md / proposal.md 描述 | specs 對應 | 差距 |
|---|---|---|---|
| Title header format | "Standardize top-level header to `# Spec: <Title Case Capability>` for every baseline spec" | `spec-format-conventions` Requirement: Title header format, with three Title Case scenarios (single-word, parenthesized platform qualifier, proper-noun) | None |
| Purpose section presence and position | "`## Purpose` is the first section after the title" | `spec-format-conventions` Requirement: Purpose section presence and position | None |
| Purpose content rules | "1–3 sentences; describes what + why; no requirement text" | `spec-format-conventions` Requirement: Purpose content (concise, descriptive-not-prescriptive, no placeholder) | None |
| Treatment of TBD placeholders | Brainstorm noted that `openspec archive` keeps generating `## Purpose` placeholders | `spec-format-conventions` Requirement: Treatment of auto-generated Purpose placeholders | None |
| Backfill of six existing specs | Brainstorm mapped each of the six specs to a specific edit | `tasks.md` §1 lists tasks 1.1–1.6 with the exact edits | None |

**漂移警告**（非阻塞）: 無.

---

## 5. Implementation Signal

- [ ] Worktree 內無未 staged 的檔案
- [ ] 所有相關 commit 已推送

**Worktree state**: 6 modified baseline spec files + 1 untracked change directory (`openspec/changes/backfill-spec-purposes/`).

```
 M openspec/specs/build-automation/spec.md
 M openspec/specs/compiler-extra-args/spec.md
 M openspec/specs/release-workflow-macos/spec.md
 M openspec/specs/release-workflow-ubuntu/spec.md
 M openspec/specs/timing-output/spec.md
 M openspec/specs/windows-vs-detection/spec.md
?? openspec/changes/backfill-spec-purposes/
```

**Commit 範圍**: none yet. Implementation was applied directly in-session (per user choice over the heavyweight worktree+subagent flow). The user has not yet been asked to commit; CLAUDE.md prohibits committing without explicit instruction.

**Blocker for archive**: yes — `openspec archive` requires the change to be committed. This is the only outstanding step before archive.

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
- [x] ⚠️ PASS WITH WARNINGS — implementation is verified against specs and conventions, but two pre-existing strict-validation failures persist and the implementation is not yet committed.
- [ ] ❌ FAIL

**Warnings**:

1. **Pre-existing strict-validation failures persist** in `release-workflow-ubuntu` (leaked `## ADDED Requirements` marker) and `windows-vs-detection` (FR-N format without SHALL/MUST). Both are explicitly out-of-scope for this change and are recorded as follow-ups in `proposal.md`. This change improves the strict-pass count (1/6 → 4/6) but does not address them. **Recommend opening two separate changes** to fix these.
2. **Implementation is uncommitted.** Six modified baseline spec files and the change directory are in the working tree but not staged or committed. Archive cannot proceed until commit.

**下一步**:

1. Review the working-tree diff (`git diff openspec/specs/`).
2. Commit. Suggested message (Conventional Commits):
   ```
   docs(specs): backfill Purpose sections and standardize titles
   ```
   Stage the six modified spec files and the entire `openspec/changes/backfill-spec-purposes/` directory in the same commit.
3. Run `/opsx:archive` (or `openspec archive backfill-spec-purposes -y`) to materialize `openspec/specs/spec-format-conventions/spec.md` and move the change folder to `openspec/changes/archive/2026-05-10-backfill-spec-purposes/`.
4. (Optional, but recommended) open follow-up changes for the two pre-existing strict failures.

The retrospective artifact is unlocked and can be written next; per the schema it should land in the same commit/PR as the implementation, before archive.
