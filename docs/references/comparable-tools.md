# Comparable Tools

How codesift relates to existing tools. codesift is **complementary**, not a drop-in replacement for most.

## Positioning matrix

| Tool | Structural | Semantic | Persistent index | Agent (MCP) | Offline |
|------|------------|----------|------------------|-------------|---------|
| **codesift** | yes | yes | yes | yes | yes |
| rust-analyzer | yes (deep) | no | no (in-memory) | no | yes |
| ripgrep / grep | no | no | no | via tools | yes |
| ctags / universal-ctags | shallow | no | optional | no | yes |
| Zoekt / Sourcegraph | yes (text) | limited | server | no | server |
| Embedding-only RAG | no | yes | varies | varies | varies |
| GitHub code search | shallow | no | cloud | no | no |

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

## See also

- [vision.md](../vision.md)
- [goals-and-non-goals.md](../goals-and-non-goals.md)
