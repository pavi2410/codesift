# Roadmap

Phased delivery plan for codesift. **Current phase: 0 complete → Phase 1 (MVP structural) next.**

## Timeline

| Phase | Focus | Outcome | Status |
|-------|-------|---------|--------|
| 0 | Docs bootstrap | Specs, ADRs, architecture complete | **complete** |
| 1 | MVP structural (Rust) | index, symbol, refs, callers; CLI | planned |
| 2 | Semantic + MCP | chunking, embeddings, hybrid search, MCP tools | planned |
| 3 | Incremental + watch | sub-second updates on save | planned |
| 4 | Multi-language | tree-sitter for TS, Python, Go | planned |
| 5 | IDE/LSP + analyzers | plugin protocol, SAST hooks | planned |

## Phase 0: Documentation (current)

- [x] Vision, goals, use cases, glossary
- [x] Architecture and technology stack
- [x] Specs (symbol model, query language, CLI, MCP, storage)
- [x] ADRs 0001–0008
- [x] Techniques and references
- [ ] Community review and spec stabilization (ongoing)

**Exit criteria:** Specs marked `accepted`; MVP milestone agreed.

## Phase 1: MVP structural

See [milestones/mvp.md](milestones/mvp.md).

- Rust parsing via `ra_ap_syntax`
- fjall symbol/ref/call indexes
- CLI: `index`, `query`, `symbol`, `refs`, `status`
- JSON output

**No semantic search or MCP in Phase 1.**

## Phase 2: Semantic + MCP

- Chunking and fastembed embeddings
- usearch + tantivy hybrid search
- CLI `search` command
- MCP server with full tool catalog
- ADR-0007 benchmark → accepted

## Phase 3: Incremental + watch

- `notify` watch mode
- Incremental invalidation graph
- Sub-second update target (benchmark TBD)
- Git commit metadata in index

## Phase 4: Multi-language

- tree-sitter backends for TypeScript, Python, Go
- Per-language PSI providers
- Capability matrix per [multi-language-strategy.md](techniques/multi-language-strategy.md)

## Phase 5: IDE/LSP + analyzers

- LSP adapter for symbol search
- Pluggable analyzer API over PSI
- SAST/lint integration via JSONL export and hooks

## See also

- [milestones/mvp.md](milestones/mvp.md)
- [goals-and-non-goals.md](goals-and-non-goals.md)
