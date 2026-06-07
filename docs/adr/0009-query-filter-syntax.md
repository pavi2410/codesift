# 0009. Query Filter Syntax

**Status:** accepted

**Date:** 2026-06-07

## Context

Structural queries are whitespace-separated filters combined with AND semantics. Filter keys include compound namespaces (`symbol:name`, `refs:to`, `callers:of`) while values include symbol IDs with internal colons (`sym://1/.../parse_query@316:1401`) and path globs. A single delimiter rule must keep parsing unambiguous without a prefix registry or expression parser.

The MVP parser had drifted: most filters used `key=value` but `path:` used a colon with no equals, and documentation mixed both forms.

## Decision

**Filter syntax:** `{filter_key}={value}` — split each token on the **first `=`** only.

- **Keys** may contain `:` (namespace within the key): `symbol:name`, `refs:to`, `callers:of`.
- **Values** may contain `:` freely (symbol IDs, Windows paths).
- Whitespace separates filters; all filters are ANDed.

Supported MVP filters:

| Filter key | Example value |
|------------|---------------|
| `symbol:name` | `parse_query`, `*Error*` |
| `kind` | `function`, `type` |
| `path` | `crates/**/parser.rs` |
| `lang` | `rust` |
| `refs:to` | `sym://1/.../parse_query@316:1401` |
| `callers:of` | same symbol ID form |
| `depth` | `3` (graph traversal for callers) |

Examples:

```
symbol:name=parse_query kind=function path=crates/**/parser.rs
refs:to=sym://1/crates/codesift-query/src/parser.rs#function:parse_query@316:1401
callers:of=sym://1/crates/codesift-query/src/parser.rs#function:parse_query@316:1401 depth=3
```

Migrate `path:` → `path=` in parser and all documentation.

## Consequences

### Positive

- One parsing rule for all filters; no special case for paths or symbol IDs
- Agents can compose queries without memorizing mixed delimiters
- Extending with new filters (`depth=`, `lang=`) follows the same pattern

### Negative

- Breaking change for any client still sending `path:glob` (pre-release MVP)
- Keys and values cannot contain unescaped `=` (acceptable for MVP scope)

## Alternatives considered

Colon-separated values (`refs:to:sym://…`) require a registered prefix table so values containing `:` are not truncated — more parser state for no gain. Mixed delimiters (`kind:function` + `symbol:name=foo`) were the accidental status quo. Dot keys, quoted values, function-call syntax, CLI flags-only, s-expressions, and JSON query bodies were evaluated for agent ergonomics; none beat the simplicity of first-equals for whitespace-tokenized AND filters at MVP scope.

## References

- [Query language spec](../specs/query-language.md)
- [CLI spec](../specs/cli.md)
