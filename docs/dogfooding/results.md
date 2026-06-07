# Dogfooding Results

Changelog of benchmark runs. Orchestrator fills rows after scoring offline.

| Date | Exercise | Commit | Tool calls | rg runs | Completeness | Latency | Notes |
|------|----------|--------|------------|---------|--------------|---------|-------|
| 2026-06-07 | DF-001 (CLI) | pre-fix | ~45–50 CLI | 1 | partial | ~8–12 min | parent session, not isolated subagent |
| 2026-06-07 | DF-001 (CLI) | 05dc01c | 12 CLI | 0 | index-complete (11 refs) | ~3 min | shell subagent; missed callers:of= |
| 2026-06-07 | DF-001-MCP | 05dc01c | **3 MCP** | 0 | 11 refs + def; stack partial | **~3–5 s** | generalPurpose subagent; validate→parse_query chain |
