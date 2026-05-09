# Open follow-ups

Tracked work that crosses cycle boundaries. Updated 2026-05-10 after the `reformat-windows-vs-detection-spec` cycle landed (repo at 7/7 strict-pass).

## Active OpenSpec changes

(none)

## Spec follow-ups

(none — `windows-vs-detection` reformat completed this cycle; `openspec validate --specs --strict` now passes 7/7.)

## Cross-cycle §6 candidates not yet promoted

- [ ] **🟡 `superpowers-bridge` schema needs a documentation-only branch** — upstream PR motivator at `JiangWay/openspec-schemas`. Three cycles of evidence now (`backfill-spec-purposes`, `fix-release-workflow-ubuntu-spec`, `reformat-windows-vs-detection-spec`); identical seven-skill skip set every cycle (brainstorming, writing-plans, using-git-worktrees, subagent-driven-development, TDD, requesting-code-review, finishing-a-development-branch). The implicit override now lives in project CLAUDE.md (promoted this cycle), so all this CLAUDE.md edit protects is this repo. Upstream branch in the schema graph would generalize to all adopters and make the schema describe the actual workflow rather than fight it.

- [ ] **🟡 Schema's brainstorm artifact assumes a fresh design problem** — upstream PR motivator at `JiangWay/openspec-schemas`. New from this cycle. Follow-up cycles whose scope is named-and-defined in a prior retro's §6 / open-follow-up section re-derive the alternatives from scratch when they should reduce to "confirm prior decision still holds". Likely combines with the docs-only branch PR above.

- [ ] **🟡 Three-cycles-cluster auto-hint for §4 Deliberately Skipped Skills** — meta-observation new this cycle. The schema's own retro instruction names the cluster pattern ("If multiple cycles skip the same skill with similar 'How to prevent' answers, that pattern is a §6 Promote candidate"), but doesn't give the next retro a tool to detect it. A simple `grep -A 8` over archived retros would surface clusters automatically. Promote when the next retro would benefit, or fold into the schema PR above.

- [ ] **📌 Long-text INFO findings on protocol-shaped requirements** — non-actionable, recorded for transparency. Multi-step protocols (e.g. windows-vs-detection's vswhere two-attempt strategy) inherently produce >500-char requirement statements after SHALL-conjunction normalization. The validator's INFO level is correct; no fix planned.

## Pointer: all carry-forward §6 candidates

The full set of unpromoted retro candidates from prior cycles lives in the archived retrospectives. To re-evaluate at the start of any new cycle:

```sh
grep -A 5 '^- \[ \]' openspec/changes/archive/*/retrospective.md
```

Per the schema's carry-forward mechanism, each retro re-evaluates these as either: carry-forward, promote-to-here, or mark-stale.

## Out of scope (explicitly not tracked)

- The `windows-vs-detection`'s "Behavior Specifications" prose section (Scenarios 1-5 in narrative form, near the bottom of the file) — separate stylistic question from the FR-N → `### Requirement:` reformat completed this cycle. Leave alone unless a future change has reason to touch them.

- 📌 one-offs from prior retros (technique notes, not actionable rules) — recorded in their respective archived retros; no action needed.
