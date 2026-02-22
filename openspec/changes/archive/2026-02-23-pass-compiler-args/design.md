## Context

`get_compiler_include_dirs` hard-codes the compiler invocation as:

```
compiler  -v  -E  -x c++  -
```

There is no mechanism to inject flags like `--target`, `--sysroot`, or `-isysroot` that cross-compilation and SDK-targeting scenarios require. The fix is a thin passthrough: collect trailing CLI args after `--` and append them to the command before the stdin sentinel.

## Goals / Non-Goals

**Goals:**
- Accept arbitrary flags after `--` and forward them verbatim to the gcc-like compiler
- Apply extra args only when `--compiler` is explicitly given and is non-MSVC-like
- Warn clearly when extra args are provided but cannot be applied

**Non-Goals:**
- Applying extra args to the MSVC / VS detection path
- Validating or interpreting the extra args
- Supporting extra args with the default compiler (no `--compiler` flag)

## Decisions

### D1 — `last = true` positional args (vs. repeatable flag or string)

**Chosen:** `#[arg(last = true)] compiler_args: Vec<String>`

Collects everything after `--` as positional args. Zero parsing ambiguity — clap handles the `--` sentinel natively.

**Alternatives considered:**
- Repeatable `--compiler-arg <val>`: verbose for multi-token flags (e.g. `--compiler-arg --target --compiler-arg arm`); rejected.
- `--compiler-args "<str>"`: space-split parsing breaks on paths with spaces; rejected.

### D2 — Insertion after fixed flags, before `-`

**Chosen:**
```
compiler  -v  -E  -x c++  <extra_args>  -
```

Reads as "standard invocation, then user augmentation, then stdin". GCC/clang treat these flags as position-independent, so functional behaviour is identical regardless of position.

**Alternatives considered:**
- Before fixed flags: also valid, but less readable — configuration before operation is less intuitive here.

### D3 — Warn-not-error for inapplicable extra args

When extra args are supplied but cannot be applied (no `--compiler`, or MSVC-like compiler), emit a warning to stderr and continue rather than hard-erroring.

**Rationale:** A warning is sufficient — the user still gets output, just without their extra flags applied. An error would break scripts that conditionally set `--` args.

## Risks / Trade-offs

- **Shell quoting complexity** — users must quote args with spaces correctly. No mitigation beyond documentation; this is standard CLI behaviour.
- **Silently wrong results** — if a user passes a flag that changes which headers are found but doesn't notice the warning about MSVC compilers, they could get unexpected output. Mitigation: the warning is explicit.

## Migration Plan

No migration needed. Purely additive — existing invocations are unaffected. The `--` separator only activates when the user explicitly uses it.

## Open Questions

None.
