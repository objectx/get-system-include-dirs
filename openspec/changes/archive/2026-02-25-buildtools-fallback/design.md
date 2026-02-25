## Context

`query_vswhere` in `src/windows_vs.rs` currently runs:

```
vswhere.exe -format json -utf8 [-latest | -version <range>]
```

vswhere's default product scope excludes `Microsoft.VisualStudio.Product.BuildTools`. On a BuildTools-only machine (common in CI), the query returns an empty array and the tool errors — even though a fully functional compiler is installed.

## Goals / Non-Goals

**Goals:**
- `query_vswhere` finds BuildTools installations when no VS IDE is present
- VS IDE retains priority when both are installed
- Single function change, no API changes, no new flags

**Non-Goals:**
- `-products *` wildcard (too broad; explicit product ID preferred)
- Supporting other non-standard VS product types
- Changing priority when both VS IDE and BuildTools are installed at the same version
- Any change outside `query_vswhere`

## Decisions

### Two-attempt fallback vs. single query with `-products Microsoft.VisualStudio.Product.BuildTools`

**Rejected**: Single query with only `-products Microsoft.VisualStudio.Product.BuildTools` would flip priority — full VS IDE would stop being returned unless it also matches.

**Rejected**: Single query with `-products *` finds all product types, but is implicit about intent and may accidentally surface non-compiler VS products in future.

**Chosen**: Two-attempt strategy:
1. Standard query (current behavior, no `-products` flag) → returns VS IDE installations
2. If result is empty: retry with `-products Microsoft.VisualStudio.Product.BuildTools` → returns BuildTools installations
3. If still empty: error, mentioning that both VS IDE and Build Tools were checked

This preserves VS IDE priority with zero behavior change for existing installations, and adds BuildTools support as a transparent fallback.

### Error on vswhere exec failure vs. fall through to attempt 2

If vswhere itself fails to execute (e.g., access denied, corrupt binary), propagate the error immediately — do not fall through to attempt 2. Both attempts use the same binary; retrying would give the same error.

Only an **empty result set** (vswhere ran successfully but found nothing) triggers the fallback.

## Risks / Trade-offs

- **Extra process spawn on BuildTools machines**: A second `vswhere.exe` call adds ~100–200ms on CI. Acceptable — this path only triggers when no VS IDE is installed.
- **vswhere not present at all**: BuildTools ships with the VS Installer which includes vswhere, so this path shouldn't occur. Existing `find_vswhere` error still applies.
- **Same version, both IDE and BuildTools installed**: Attempt 1 returns the IDE. Attempt 2 is never reached. No ambiguity.

## Open Questions

None — approach confirmed during exploration.
