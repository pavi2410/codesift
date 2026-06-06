# Use Cases

Each consumer interacts with the same underlying index through CLI or MCP. Examples below use planned query syntax; see [specs/query-language.md](specs/query-language.md).

## AI coding agents (MCP)

Agents need structured, verifiable answers — not hallucinated file paths.

| Task | MCP tool / query | Example |
|------|------------------|---------|
| Find callers | `get_callers` | "Who calls `retry_with_backoff`?" |
| Semantic similarity | `search_semantic` | "Code that handles exponential backoff on network errors" |
| Read exact source | `read_chunk` | Return byte range for a matched chunk |
| Structural filter | `query_structural` | `symbol:name=UserService kind=struct` |
| Index freshness | `index_status` | Report last index time, file count |

**Agent workflow:**

```
1. index_status          → confirm workspace is indexed
2. search_semantic       → find candidate chunks
3. get_symbol            → resolve stable ID
4. find_references       → impact analysis
5. read_chunk            → fetch source for context window
```

## IDE and LSP plugins (future)

| Task | Query | Notes |
|------|-------|-------|
| Go to definition | `symbol:name=foo path:src/**` | Phase 5 LSP adapter |
| Find references | `refs:to=sym://...` | Same index as CLI |
| Call hierarchy | `callers:of=sym://... depth=3` | Structural graph |
| Symbol search | `symbol:name=*Auth* kind=function` | Fuzzy via tantivy |

codesift complements language servers: LSP provides types and diagnostics; codesift provides persistent cross-session search and agent-oriented retrieval.

## Code review and CI

| Task | Query / export | Value |
|------|----------------|-------|
| Blast radius | `refs:to=sym://...` on changed symbols | Reviewers see impact |
| Changed files → symbols | Diff + index | Map git diff to symbol set |
| Export for tools | `codesift export --format jsonl` | Feed SAST pipelines |

**Example CI step:**

```bash
codesift index .
codesift query "refs:to=sym://abc123" --format json > impact.json
```

## SAST and linting

codesift provides a **substrate**, not a rule engine:

- PSI/AST per file for pluggable analyzers
- Symbol graph for inter-procedural rules (Phase 2+)
- JSONL export for external tools

Analyzers register over the index API (future) rather than re-parsing the repo.

## General development

| Task | Interface | Example |
|------|-----------|---------|
| Cross-crate symbol search | CLI | `codesift query 'symbol:name=Deserialize kind=trait'` |
| Doc + code unified search | CLI `search` | "how do we configure logging?" |
| Explore unfamiliar repo | MCP `search_semantic` | Onboarding |
| Local offline search | CLI | No network required |

## See also

- [specs/mcp.md](specs/mcp.md) — MCP tool catalog
- [specs/cli.md](specs/cli.md) — CLI commands
- [specs/query-language.md](specs/query-language.md) — query grammar
