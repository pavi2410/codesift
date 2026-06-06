# Comparable Tools

How codesift relates to existing tools. codesift is **complementary**, not a drop-in replacement for most.

## Positioning matrix

| Tool | Structural | Semantic | Persistent index | Agent (MCP) | Offline |
|------|------------|----------|------------------|-------------|---------|
| **codesift** | yes | yes | yes | yes | yes |
| [codedb](https://github.com/justrach/codedb) | yes | no | yes (in-memory + snapshots) | yes | mostly |
| [ast-grep](https://github.com/ast-grep/ast-grep) | yes (pattern) | no | no | via CLI | yes |
| rust-analyzer | yes (deep) | no | no (in-memory) | no | yes |
| ripgrep / grep | no | no | no | via tools | yes |
| ctags / universal-ctags | shallow | no | optional | no | yes |
| Zoekt / Sourcegraph | yes (text) | limited | server | no | server |
| Embedding-only RAG | no | yes | varies | varies | varies |
| GitHub code search | shallow | no | cloud | no | no |

## codedb

[codedb](https://github.com/justrach/codedb) is a Zig binary that indexes a codebase on startup and serves **structural queries over MCP** — outlines, identifier lookup, trigram search, reverse dependency graph, and a content cache. See [Why codedb feels instant for agents](https://codegraff.com/blog/codedb-code-intelligence) for the performance and token-budget story.

**Overlap with codesift:** Both target AI agents that today pay a high cost shelling out to `ripgrep` and reading whole files. Both offer persistent-ish indexes, structured MCP responses, and dependency-aware queries.

**Difference:**

| Aspect | codedb | codesift |
|--------|--------|----------|
| Implementation | Zig | Rust |
| Hot path | In-memory indexes; snapshots for restart | Disk-backed fjall + usearch + tantivy (`.codesift/`) |
| Semantic search | Not a focus in public material | Planned (embeddings + BM25 hybrid) |
| Analysis depth | Structural indexes | Path to inspections, complexity, architectural views |
| Remote repos | `codedb_remote` via cloud | Local-first; PURL/library docs (integrate-first, planned) |
| Ecosystem | Pairs with [muonry](https://codegraff.com) for edits + instant index sync | Index/analyze; refactor **preview** — apply via LSP / ast-grep |

**Relationship:** Direct competitor on **Phase 1–3 structural MCP** (symbols, deps, fast agent queries). codesift differentiates on **semantic retrieval**, **deeper IntelliJ-style analysis**, and **local dependency intelligence**. Benchmarks and token efficiency in codedb’s blog are a useful bar for codesift MCP design.

## ast-grep

[ast-grep](https://github.com/ast-grep/ast-grep) is a **structural search and rewrite** tool: AST-aware patterns (`$VAR`, `$$$`), codemods, and lint-style rules across many languages. Agents and CI use it to find and **apply** transformations, not to maintain a long-lived index.

**Overlap:** Both work above raw text — ast-grep matches syntax trees; codesift builds symbol graphs and PSI-like structure.

**Difference:** ast-grep is **on-demand**: each run parses and matches. codesift **indexes once** (incrementally) and answers graph queries (`callers:`, `refs:`) and semantic search across sessions. ast-grep **mutates** code; codesift **observes** changes and re-indexes (refactor preview only, per [big-picture.md](../big-picture.md)).

**Relationship:** **Complementary.** Typical split:

1. codesift — discover, impact analysis, inspections, “what exists / who calls whom”
2. ast-grep — execute codemods and structural rewrites from vetted rules

codesift may export rule hints or spans for ast-grep; it does not replace ast-grep for batch rewrite.

## rust-analyzer

**Overlap:** Rust parsing, symbols, references, types.

**Difference:** rust-analyzer is an LSP server with rich type inference, kept in memory per session. codesift is a **persistent, queryable index** with semantic search and MCP — designed for agents and cross-session use.

**Relationship:** Use `ra_ap_syntax` for Rust parsing; do not duplicate type checking. Future LSP plugin may query codesift for search while rust-analyzer handles types.

## ripgrep / grep

Fast literal search. codesift does not replace grep for regex over raw files. codesift adds structure (who calls X?) and semantics (code similar to Y).

## ctags / universal-ctags

Lightweight tag indexes (definitions). codesift adds references, call graphs, semantic chunks, and hybrid search. ctags is simpler and faster for basic jump-to-def in editors.

## Zoekt / Sourcegraph

Web-scale code search (trigram + regex). Requires server infrastructure. codesift targets **local embedded** indexes for CLI and agents. Zoekt excels at monorepo grep; codesift excels at agent-structured queries and local semantic search.

## Embedding-only RAG tools

Tools that chunk and embed without structural indexes. codesift combines both: agents get `sym://` IDs and `callers:` queries, not just similar text chunks.

## GitHub / GitLab code search

Cloud-hosted, shallow. codesift works on local/private repos offline.

## When to use codesift

- AI agents need structured symbol/ref/caller tools via MCP
- You want persistent local semantic + structural search
- CI needs blast-radius queries on symbol changes
- Building custom analyzers on a shared index

## When not to use codesift (yet)

- You need full Rust type inference → use rust-analyzer
- You need fast regex over files → use ripgrep
- You need enterprise-wide code hosting search → use Sourcegraph/Zoekt
- You need only sub-ms **structural** MCP today, shipped in Zig → evaluate [codedb](https://github.com/justrach/codedb)
- You need to **apply** AST-aware codemods now → use [ast-grep](https://github.com/ast-grep/ast-grep); use codesift for discovery and impact first

## See also

- [vision.md](../vision.md)
- [big-picture.md](../big-picture.md)
- [goals-and-non-goals.md](../goals-and-non-goals.md)
- [codedb blog — code intelligence for agents](https://codegraff.com/blog/codedb-code-intelligence)
- [ast-grep documentation](https://ast-grep.github.io/)
