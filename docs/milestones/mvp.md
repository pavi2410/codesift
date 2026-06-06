# MVP Milestone (Phase 1)

**Status:** planned

Structural indexing for Rust with CLI. No semantic search or MCP.

## Scope

| In scope | Out of scope |
|----------|--------------|
| Index Rust workspace | Non-Rust languages |
| Symbol definitions | Semantic / vector search |
| References (syntax-level) | MCP server |
| Call graph (best-effort) | Watch / incremental mode |
| CLI commands below | tantivy, usearch, fastembed |

## Parser

- `ra_ap_syntax` for all `.rs` files
- Edition detection from `Cargo.toml` or default 2024

## Storage

- fjall keyspaces: `symbols`, `refs`, `edges`, `file_state`
- postcard serialization
- `.codesift/` on disk

## CLI commands

| Command | Required |
|---------|----------|
| `codesift index [path]` | yes |
| `codesift query <query>` | yes |
| `codesift symbol <id or --name>` | yes |
| `codesift refs <id>` | yes |
| `codesift status` | yes |
| `codesift export --format jsonl` | yes |

## Query support

| Query | Required |
|-------|----------|
| `symbol:name=` | yes |
| `kind=` | yes |
| `path=` | yes |
| `refs:to=` | yes |
| `callers:of=` | yes |

## Acceptance criteria

1. **Index** — `codesift index .` on a medium Rust crate (e.g. 500+ files) completes without panic
2. **Symbols** — `symbol:name=main` returns expected `main` function
3. **Refs** — `refs:to=` returns reference sites for a known symbol
4. **Callers** — `callers:of=` returns at least direct callers for a leaf function
5. **Persistence** — Second `codesift query` after restart returns same results without re-index (hash-based skip)
6. **Output** — `--format json` produces valid JSON per [query-language.md](../specs/query-language.md)
7. **Disk** — `.codesift/meta.json` exists with `symbol_count > 0`

## Test corpus

- codesift repo itself (once code exists)
- Public Rust crate: `ripgrep` or `tokio` subset (TBD)

## Success metrics (informal)

| Metric | Target |
|--------|--------|
| Index time | TBD — benchmark on ripgrep |
| Query latency | < 100ms for name lookup on medium repo |
| Memory | TBD |

## See also

- [roadmap.md](../roadmap.md)
- [phase-2-structural-depth.md](phase-2-structural-depth.md)
