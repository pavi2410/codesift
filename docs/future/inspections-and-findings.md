# Inspections and Findings

> **Type:** future · **Audience:** contributor · **Read when:** exploring analysis-layer MCP tools or CI gates

**Status:** `exploring`

Sketch for Layer 2–3 and Layer 6 capabilities in [big-picture.md](../product/big-picture.md). Promote to `docs/specs/` when ready to implement.

## Why agents need more than search

| Failure mode | Root cause | Intelligence that prevents it |
|--------------|------------|-------------------------------|
| Wrong file or symbol | Guessing from names | Structural index + resolve |
| Breaking callers | No impact graph | `find_references`, `get_callers`, blast radius |
| Duplicate logic | No similarity awareness | Clone / near-duplicate detection |
| Dead or unreachable code | No reachability | Usage + CFG-backed heuristics |
| Unsafe refactor | No data-flow context | Flow summaries, taint paths |
| "Looks fine" patches | No inspections | Rule findings with evidence and severity |
| Vibe-coded slop | No pre/post change gates | CI + MCP inspection workflows |

## Inspection plugin model

```rust
// Conceptual — TBD at implementation
trait Inspection {
    fn id(&self) -> &str;
    fn run(&self, ctx: &AnalysisCtx) -> Vec<Finding>;
}
```

**Integrate** existing engines via adapters (Clippy, Ruff, Semgrep, ast-grep rules) rather than rewriting every lint. codesift **unifies** findings onto `sym://`, severity, category, and optional fix hints.

## Planned analysis surfaces

| Tool / command | Purpose |
|----------------|---------|
| `run_inspections` | Run rule set; return findings with `sym://` |
| `find_duplicates` | Clone and near-duplicate clusters |
| `dead_code_candidates` | Unreferenced / unreachable symbols |
| `resolve_symbol` | Bind ref site to definition (per language) |
| `preview_rename` | Impact before rename |
| `dataflow_summary` | Flow in/out of a function (lite) |
| `get_complexity` | Cyclomatic/cognitive scores for a symbol |
| `search_library_docs` | Semantic search over PURL-indexed dependency docs |
| `resolve_libraries` | Lockfile-resolved PURLs for workspace deps |
| `codesift check` | CI gate on severity threshold |

## Finding record (conceptual)

```json
{
  "id": "finding://42/inspection/unused_symbol/7f3a",
  "rule_id": "unused_symbol",
  "severity": "warning",
  "message": "Function `old_helper` is never referenced",
  "symbol_id": "sym://42/src/util.rs#function:old_helper@100:200",
  "path": "src/util.rs",
  "location": { "start_line": 10, "end_line": 25 },
  "category": "dead_code"
}
```

## Agent workflow with intelligence gates

Recommended loop for agentic changes (policy via MCP tool descriptions, Cursor rules, or skills):

```
1. index_status                    → index fresh?
2. search_semantic / query_structural → locate area
3. find_references + get_callers    → impact before edit
4. run_inspections (scope)          → existing smells (planned)
5. find_duplicates (near change)      → don't duplicate (planned)
6. agent proposes edit
7. run_inspections (post-edit)      → incremental re-check (planned)
8. CI: codesift check --min-severity=warning (planned)
```

## Entry discovery (sketch)

Agents must not assume conventions like `fn main`. Exploration combines build manifests, structural query, semantic search, and graph traversal. Planned conveniences: `entrypoint:` query, `bootstrap` MCP tool (spec TBD).

## See also

- [big-picture.md](../product/big-picture.md) — analysis pillar strategy
- [use-cases.md](../product/use-cases.md) — consumer workflows
- [complexity-metrics.md](complexity-metrics.md) — CFG-backed complexity
- [capabilities.md](../capabilities.md) — `inspections-unified`, `quality-signals`, `refactor-preview`
