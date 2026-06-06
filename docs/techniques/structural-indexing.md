# Structural Indexing

Algorithms for building symbol, reference, and call graph indexes.

## Definition indexing

For each parsed file:

1. Walk PSI / CST for definable nodes (functions, types, modules, ...)
2. Assign symbol ID (path + kind + name + span)
3. Write `SymbolRecord` to fjall `symbols`
4. Update secondary indexes: `name:`, `path:`, `kind:`
5. Record symbol IDs in `file_state`

### Qualified names

Best-effort `qualified_name` from module path + name. Full cross-crate resolve deferred.

## Reference indexing

Syntax-level references (not type-resolved):

1. Visit reference sites: type names, path segments, use statements
2. Resolve target by local scope rules (same file, imports) — MVP: name match + import graph
3. Write `RefRecord` to `refs` keyspace
4. If target unknown, store unresolved ref with `resolved: false`

## Call graph extraction

1. Visit call expressions in function bodies
2. Extract callee identifier or path
3. Match to symbol in scope (local → import → global name index)
4. Write `calls` edge to `edges` + reverse index `rev:calls:{to_id}`

### Limitations (MVP)

- No dispatch through traits (syntax-level name only)
- Macro-generated calls may be missed
- Dynamic calls (`fn_ptr()()`) not tracked

## Import graph

1. Extract `use` and `mod` statements
2. Create `imports` edges: module → target module/symbol
3. Used for cross-file invalidation and Phase 2 scope resolution

## Indexing order (full build)

```
for each file in parallel (bounded):
  parse → extract symbols → extract refs → extract calls
batch write to fjall (per-file transaction)
build tantivy docs from symbols/chunks
build usearch vectors from chunks (semantic pass)
```

## Query execution

| Query | Index used |
|-------|------------|
| `symbol:name=` | `name:` secondary index |
| `refs:to=` | forward `refs` or scan by `to_id` |
| `callers:of=` | `rev:calls:{to_id}` |
| `path:` | `path:` secondary index |

## See also

- [../specs/symbol-model.md](../specs/symbol-model.md)
- [../specs/index-schema.md](../specs/index-schema.md)
- [incremental-invalidation.md](incremental-invalidation.md)
