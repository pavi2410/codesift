# Structural Index (Rust)

**Capability:** `structural-index-rust`, `cli-structural`  
**Status:** verified

Rust workspace structural indexing with CLI. Independent of semantic search and MCP — those are separate capabilities in [capabilities.md](../capabilities.md).

## Covered

| Item | Detail |
|------|--------|
| Parser | `ra_ap_syntax` for all `.rs` files |
| Edition | From `Cargo.toml` or default 2024 |
| Storage | fjall keyspaces: `symbols`, `refs`, `edges`, `file_state`; postcard; `.codesift/` |
| Symbols | Definitions: modules, functions, types, fields, macros, imports |
| References | Syntax-level refs and call sites |
| Call graph | Best-effort direct callers |

## CLI commands

| Command | Capability |
|---------|------------|
| `codesift index [path]` | `structural-index-rust` |
| `codesift query <query>` | `cli-structural` |
| `codesift symbol <id or --name>` | `cli-structural` |
| `codesift refs <id>` | `cli-structural` |
| `codesift status` | `cli-structural` |
| `codesift export --format jsonl` | `cli-structural` |

## Query support

| Query | Required |
|-------|----------|
| `symbol:name=` | yes |
| `kind=` | yes |
| `path=` | yes (glob when `*` or `?`) |
| `refs:to=` | yes (per-edge call sites) |
| `callers:of=` | yes (per-edge, `depth=` supported) |
| `depth=` | yes (caller graph BFS) |

Query filter syntax: [ADR 0009](../adr/0009-query-filter-syntax.md). Examples use `{filter_key}={value}` (first `=` only).

## Dogfooding acceptance

Exercises [DF-001](../dogfooding/exercises/001-parse-query-usages.md)–[DF-003](../dogfooding/exercises/003-symbol-by-kind-path.md) encode acceptance scenarios. CI: `cargo test -p codesift-index --test dogfood_exercises`.

## Acceptance criteria

1. **Index** — `codesift index .` on a medium Rust crate (500+ files) completes without panic
2. **Symbols** — `symbol:name=main` returns expected `main` function
3. **Refs** — `refs:to=` returns reference sites for a known symbol
4. **Callers** — `callers:of=` returns at least direct callers for a leaf function
5. **Persistence** — Second `codesift query` after restart returns same results without re-index (hash-based skip)
6. **Output** — `--format json` produces valid JSON per [query-language.md](../specs/query-language.md)
7. **Disk** — `.codesift/meta.json` exists with `symbol_count > 0`

## Test corpus

- codesift repo — [dogfooding exercises](../dogfooding/README.md)
- Public Rust crate: `ripgrep` or `tokio` subset (TBD)

## Success metrics (informal)

| Metric | Target |
|--------|--------|
| Index time | TBD — benchmark on ripgrep |
| Query latency | < 100ms for name lookup on medium repo |
| Memory | TBD |

## See also

- [capabilities.md](../capabilities.md)
- [structural-depth.md](structural-depth.md)
