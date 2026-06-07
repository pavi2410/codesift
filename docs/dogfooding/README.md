# Dogfooding Exercises

Reproducible agent tasks on the codesift repo (or fixtures) with ground-truth answers, recorded baselines, and future automation input.

**Not** a limitations manifest — exercises define what good looks like.

## Exercises

| ID | Slug | Description |
|----|------|-------------|
| [DF-001](exercises/001-parse-query-usages.md) | `parse-query-usages` | Find callstack and usages of `parse_query` (codesift CLI only) |
| [DF-002](exercises/002-indexer-callers.md) | `indexer-callers` | Find callers of `index` function |
| [DF-003](exercises/003-symbol-by-kind-path.md) | `symbol-by-kind-path` | Path glob + kind filter on query crate |

See [automation.md](automation.md) for the planned benchmark harness. Aggregate run history: [results.md](results.md).

## Running exercises

Dogfooding measures **tool + agent behavior under constraints**, not whether the same chat already learned workarounds.

**Rule:** Never run the exercise **Prompt** in the chat that authored the exercise, debugged codesift, or read ground-truth tables. Use a **fresh subagent** per attempt.

| Role | Responsibility |
|------|----------------|
| **Orchestrator** (parent session) | Pick exercise; record commit SHA, codesift version, date; ensure prerequisite index; launch subagent; collect metrics; score against ground truth **after** the run |
| **Exercise subagent** | Receives **only** the exercise prompt, tool constraints, and repo workspace — no ground truth, no prior dogfooding summaries, no hints about sym IDs or workarounds |

### Launch checklist (orchestrator)

1. New subagent (e.g. Cursor Task) — do not `resume` a prior exercise agent unless explicitly benchmarking continuation behavior (DF-010).
2. Subagent prompt = exercise **Prompt** field verbatim + allowed/forbidden tools (e.g. "codesift CLI only; MUST NOT use grep/rg").
3. Optional context: workspace path, `mise run cli` invocation pattern — **not** expected answers or example sym URLs from a previous session.
4. Let subagent run to completion; do not mid-run correct misses or paste ground-truth tables.
5. After completion, orchestrator fills **Baseline run** metrics and compares to **Ground truth** offline.

### Metrics dimensions

| Dimension | What to measure | codesift | ripgrep/grep |
|-----------|-------------------|----------|--------------|
| Quality — completeness | All true usages/callers found? | hit count vs ground truth | match count (may include noise) |
| Quality — precision | False positives? | spurious hits | doc strings, comments |
| Quality — structure | Callstack, kind, signature? | yes / partial | no (text only) |
| Perf — query latency | Time per lookup (ms) | `took_ms` in response | `time rg …` |
| Perf — cold start | Index + first query | `index` duration + query | N/A |
| Efficiency — agent cost | Shell invocations, output tokens | count CLI runs | count rg runs |

## CI encoding

Integration tests in [`crates/codesift-index/tests/dogfood_exercises.rs`](../../crates/codesift-index/tests/dogfood_exercises.rs) encode DF-001–003 ground truth.
