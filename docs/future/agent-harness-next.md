# Agent harness & MCP — next changes

**Status:** exploring — planning doc after MCP v1 + mise wiring.

## Done (this milestone)

- `mise run mcp` — canonical stdio launcher
- `.cursor/mcp.json` — Cursor project MCP
- `opencode.json` — OpenCode project MCP

## Immediate next steps

### 1. First MCP dogfood baselines

| Exercise | Client | Protocol |
|----------|--------|----------|
| DF-001-MCP | Cursor Agent (fresh chat, MCP on) | Prompt only, no ground-truth docs |
| DF-001-MCP | OpenCode | Same prompt — **cross-client baseline** |

Record in [dogfooding/results.md](../dogfooding/results.md): client, model, commit, MCP tool-call count, wall time.

**Do not** use shell/Task subagents for MCP exercises — they have no MCP tools.

### 2. Call stack / ancestry (flat wire format)

Replace manual `refs --name` walking with structured MCP output:

| Tool | Output |
|------|--------|
| `get_call_stack` (new) or extend `get_callers` | Flat ordered `frames[]` — one ancestry path to target |
| `get_callers` | Flat `frames[]` with `depth` + `parent_id` (not nested JSON) |

ASCII tree stays in CLI `--format ascii` only.

**Unblocks:** DF-001 callstack in ≤2 MCP calls (`find_references` + `get_call_stack`).

### 3. Store locking — read path

Fjall: one process per DB open. Parallel `cargo run` / parallel CLI = `Locked`.

| Priority | Change |
|----------|--------|
| P0 | Document: serialize codesift CLI; use `mise run cli` not parallel `cargo run` |
| P1 | **Index snapshot** at end of `index` — queries load snapshot, never open fjall from N processes |
| P2 | Long-lived query reader (MCP already one process — OK) |

### 4. `explore_symbol` composite (v2 MCP)

Single tool: definition + refs + call stack for a name. Collapses DF-001 to **1 call**.

Deferred until `get_call_stack` proven.

### 5. Deterministic harness (CI, no LLM)

```
benchmarks/dogfood/
  mcp_stdio.rs   # spawn `mise run mcp`, JSON-RPC tool calls, score vs ground truth
```

Complements LLM dogfood; no client bias.

## Suggested PR order

| PR | Content |
|----|---------|
| **PR1** | mise MCP + Cursor/OpenCode config (this change) |
| **PR2** | `get_call_stack` flat frames + MCP tool |
| **PR3** | Index snapshot read path (fix multi-process lock) |
| **PR4** | OpenCode DF-001-MCP baseline + results.md row |
| **PR5** | `explore_symbol` or stdio dogfood harness |

## Testing pyramid

```
L0  cargo test (mcp_tools.rs, dogfood_exercises.rs)
L1  MCP Inspector → mise run mcp
L2  Cursor Agent smoke (dev — biased)
L3  OpenCode dogfood (cross-client)
L4  CI stdio harness (deterministic)
```

## See also

- [dogfooding/README.md](../dogfooding/README.md)
- [adr/0010-mcp-agent-frontend-cli-ops.md](../adr/0010-mcp-agent-frontend-cli-ops.md)
- [mcp.md](../specs/mcp.md)
