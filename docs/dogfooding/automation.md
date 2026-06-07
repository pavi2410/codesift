# Dogfooding Automation (design stub)

**Status:** exploring — not implemented in structural MVP.

## Goal

Automated runner comparing codesift vs ripgrep/grep on quality, latency, and agent-efficiency metrics defined in [README.md](README.md).

## Planned layout

```
benchmarks/dogfood/
  runner.rs          # load exercise YAML/JSON, run codesift + rg, score
  exercises/         # machine-readable copies of DF-* ground truth
  fixtures/          # minimal repos for isolated tests
  reports/           # JSON results per commit (gitignored or CI artifact)
```

## Runner behavior

1. Load exercise definition (prompt, ground truth, allowed tools)
2. For **agent** benchmarks: spawn isolated subagent/run with prompt only (same clean-context rules as manual protocol); scripted CLI runs may skip the agent
3. Run **codesift** command sequence; capture stdout, `took_ms`, hit JSON
4. Run **ripgrep** / **grep** baseline commands
5. Score quality (recall/precision vs ground truth), latency, command count
6. Emit comparison report (markdown + JSON for CI trend graphs)

## Agent runners

Manual exercises use Cursor subagents today. For repeatable CI benchmarks, prefer headless agents (e.g. OpenCode `run`, scripted SDK agents) with the same prompt-only isolation rules.

## Competing tools

| Tool | Role |
|------|------|
| **ripgrep** | Text search baseline |
| **grep** | Portable baseline |
| **ast-grep** | Structural pattern baseline (when rules exist) |
| **rust-analyzer / IDE** | Gold standard for resolve (manual or LSP, future) |
| **codesift** | Indexed structural queries |

Link from [capabilities.md](../capabilities.md) when capability `dogfood-benchmark` is added.

## CI today

[`crates/codesift-index/tests/dogfood_exercises.rs`](../../crates/codesift-index/tests/dogfood_exercises.rs) encodes DF-001–003 without an agent layer.
