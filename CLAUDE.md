# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`get-system-include-dirs` is a small Rust CLI tool that queries C++ compilers (or Windows Visual Studio installations) to discover their default system include directories.

## Common Commands

All recipes are defined in `Justfile` and use [Just](https://just.systems/) with [Nushell](https://www.nushell.sh/) as the shell.

```sh
cargo test                    # Run tests
cargo clippy                  # Lint
cargo fmt                     # Format code
just build aarch64-apple-darwin  # Release build for a specific target
just build-all                # Build for macOS x86, macOS arm, Linux (uses `cross` for Linux)
just check                    # Alias for cargo clippy
just clean                    # Clean build + dist artifacts
```

Run a single test by name:
```sh
cargo test <test_name>
```

## Architecture

The tool has two source files:

- **`src/main.rs`** — CLI entry point, argument parsing (via `clap`), and the gcc-like compiler path. Invokes the compiler with `-v -E -x c++ [extra_args] -` and parses stderr output between `#include <...> search starts here:` and `End of search list.` Strips macOS `(framework directory)` annotations and normalizes path separators to `/`.

- **`src/windows_vs.rs`** — Windows-only module (compiled with `#[cfg(windows)]`). Handles MSVC-like compilers (`cl`, `cl.exe`, `clang-cl`, `clang-cl.exe`) and the no-compiler case. Priority: `$INCLUDE` env var → auto-detect via `vswhere.exe` → run `vsdevcmd.bat -arch=x64` to capture INCLUDE. Falls back from VS IDE editions to BuildTools product when no IDE is found.

### Decision flow in `get_include_dirs`

1. On Windows, if no compiler is given or the compiler is MSVC-like → delegate to `windows_vs::get_windows_include_dirs_with_fallback`.
2. Otherwise (gcc-like compiler, or Unix with no compiler) → default to `/usr/bin/c++` on Unix, then call `get_compiler_include_dirs`.
3. `compiler_args` (passed via `--`) are forwarded to gcc-like compilers only; a warning is emitted if they cannot be applied.

## Conventions

- Commit messages follow **Conventional Commits**.
- Releases are cross-compiled: macOS targets use `cargo`, Linux uses `cross`, Windows must be built natively.
- Output binaries go to `dist/<target>/`.

## OpenSpec Workflow

Design specs live in `openspec/specs/<change-name>/spec.md`. Active changes are worked from that directory; completed changes are archived under `openspec/changes/archive/`. The `openspec/config.yaml` records project context and per-artifact rules used when proposing or applying changes via the `opsx:*` skills.

### Validate the empty change scaffold first

After running `openspec new change <name>`, run `openspec validate <name> --type change` against the empty scaffold **before drafting any artifacts**. This surfaces schema-required constraints (e.g. "Change must have at least one delta", `applyRequires` chains) at zero cost. Discovering those constraints mid-flight (after brainstorm/proposal/specs are written) forces a re-design pass — see the `2026-05-09-backfill-spec-purposes` retrospective §2 for the cycle that motivated this rule.

### Documentation-only changes need a meta-conventions capability

When a planned change touches only `openspec/specs/**/*.md` (or other docs paths) and has no requirement deltas, `openspec validate` will reject it with `Change must have at least one delta`. The lazy fix — fabricating a no-op `MODIFIED Requirements` stub — lies about what is changing and adds no future value.

The honest fix is to ask "what convention am I converging the docs onto?" and introduce a new capability whose **requirements encode that convention**. The docs edits then become the implementation work that brings existing files into compliance with the new requirements. This is strictly better: the convention is testable (`openspec validate --strict`), durable (future drift is caught), and the change is no longer fighting the validator. See `openspec/specs/spec-format-conventions/spec.md` for the worked example.

Smell to recognize: "I'm trying to skip deltas because nothing 'really' changes" → reframe as "I'm encoding a convention as deltas."

### Docs-only apply override is implicit

When a change's only modified files match `openspec/specs/**/*.md` (or other docs paths), the apply phase **defaults to direct in-session edits** — no worktree dispatch, no per-task subagent, no TDD/code-review ceremony. Plan.md may omit the override paragraph; reviewers infer it from the file scope.

This is the schema's `apply.requires` boilerplate (worktree + subagent-driven-development + TDD + per-task code review) being incorrectly calibrated for code changes. Three docs-only cycles (`backfill-spec-purposes`, `fix-release-workflow-ubuntu-spec`, `reformat-windows-vs-detection-spec`) wrote the same override paragraph identically; promoting the rule here removes the boilerplate. The upstream fix is a docs-only branch in the schema graph (tracked as a §6 candidate in those retros' carry-forward sections).

## Workflow routing (read on session start)

This repo uses [`superpowers-bridge`](https://github.com/JiangWay/openspec-schemas/tree/main/superpowers-bridge) to bridge OpenSpec and Superpowers. Integration rules (language, artifact paths, PRECHECK) follow that bridge's README; this section is the routing guidance for Claude.

### Entry routing

| Trigger you observe | What to do |
|---|---|
| User starts a narrative "design discussion / let's brainstorm" | Run verbal `superpowers:brainstorming`, but **do NOT** write to `docs/superpowers/specs/`. Once the conversation converges per the 5 criteria below, promote to `/opsx:propose` |
| User invokes `/opsx:new` / `/opsx:ff` / `/opsx:propose` directly | Follow the schema's flow; artifact instructions inject at each step |
| User explicitly says bug fix / typo / config tweak / doc update | Direct PR — **do NOT** open a change (see skip rules below) |
| User is mid-change | Advance with `/opsx:continue`, `/opsx:apply`, `/opsx:verify`, or `/opsx:archive` |

### When NOT to use opsx (direct PR)

| Scenario | Direct PR? |
|---|---|
| New feature / new capability / architectural change / breaking change | ❌ Use opsx |
| Bug fix (no contract change) / test backfill / linter tweak / non-breaking upgrade / typo / docs / config value tweak | ✅ Direct PR |

Principle: **process ceremony scales with risk**. External contracts / schema / cross-system integration / compliance → opsx. Otherwise → direct PR.

### Verbal brainstorm → opsx promotion criteria

All 5 must hold before promoting (any missing → keep brainstorming, **never** write to `docs/superpowers/specs/`):

1. **Scope locked** — one sentence describes what's in / out
2. **Major design forks resolved** — alternatives weighed; remaining TBDs have an owner and impact-scope statement
3. **Cross-system dependencies mapped** — ready / mockable / genuinely unknown — pick one per dep
4. **Acceptance criteria stateable** — concrete pass conditions (e.g., `./mvnw clean verify` passes + N deliverables)
5. **Conversation converging** — recent turns are confirmations, not new alternatives

When all 5 hold → proactively suggest "ready to `/opsx:propose`?" — wait for user ack. Never auto-trigger.

### Front-door anti-patterns (don't do)

- Letting brainstorming write to `docs/superpowers/specs/`
- Letting writing-plans write to `docs/superpowers/plans/`
- Promoting to opsx with unresolved blocking TBDs
- Opening a change for bug fix / typo

Full detail: [superpowers-bridge README §Entry & exit gates](https://github.com/JiangWay/openspec-schemas/blob/main/superpowers-bridge/README.md#entry--exit-gates).
