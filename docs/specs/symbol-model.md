# Symbol Model

**Status:** draft

Defines symbols, stable IDs, locations, and relations. See ADR-0004 for storage.

## Symbol kinds

| Kind | Description | Example |
|------|-------------|---------|
| `module` | File or inline module | `src/lib.rs` |
| `function` | Function or method | `fn parse_query` |
| `type` | struct, enum, union, type alias | `struct User` |
| `field` | Struct/enum field | `user.name` |
| `const` | const / static item | `const MAX: u32` |
| `macro` | macro_rules! or proc macro | `macro_rules! vec` |
| `import` | use / mod import | `use std::io` |
| `trait` | trait definition | `trait Display` |
| `impl` | impl block (as container) | `impl User` |
| `doc_section` | Markdown heading section | `## Installation` |
| `parameter` | Function parameter (optional) | `name: String` |

## Symbol ID format

```
sym://{workspace_rev}/{path}#{kind}:{name}@{start_byte}:{end_byte}
```

| Component | Description |
|-----------|-------------|
| `workspace_rev` | Unsigned integer; bumped on material index changes |
| `path` | Repo-relative path, URL-encoded (`/` as `/`) |
| `kind` | Symbol kind from table above |
| `name` | Display name; nested names use `.` (e.g. `User.name`) |
| `start_byte` / `end_byte` | UTF-8 byte offsets in file |

**Example:**

```
sym://42/src/query/parser.rs#function:parse_query@1204:1890
```

### Stability rules

- IDs are stable within a `workspace_rev` if the symbol's path, kind, name, and span are unchanged
- Renames or moves produce new IDs; old IDs are tombstoned
- Cross-revision lookup uses name/path queries, not ID alone

## Location

```json
{
  "path": "src/query/parser.rs",
  "language": "rust",
  "start_byte": 1204,
  "end_byte": 1890,
  "start_line": 45,
  "start_column": 0,
  "end_line": 72,
  "end_column": 1
}
```

Line/column are 0-based. `language` uses lowercase identifiers (`rust`, `typescript`, `markdown`).

## Symbol record

```json
{
  "id": "sym://42/src/query/parser.rs#function:parse_query@1204:1890",
  "kind": "function",
  "name": "parse_query",
  "qualified_name": "codesift_query::parse_query",
  "path": "src/query/parser.rs",
  "language": "rust",
  "location": {
    "start_byte": 1204,
    "end_byte": 1890,
    "start_line": 45,
    "end_line": 72
  },
  "visibility": "pub",
  "signature": "pub fn parse_query(input: &str) -> Result<QueryAst, ParseError>",
  "doc_comment": "Parse a structural query string into an AST.",
  "parent_id": "sym://42/src/query/parser.rs#module:parser@0:5000",
  "workspace_rev": 42,
  "record_version": 1
}
```

| Field | Required | Notes |
|-------|----------|-------|
| `qualified_name` | no | Best-effort; full resolve deferred |
| `visibility` | no | `pub`, `pub(crate)`, `private`, etc. |
| `signature` | no | Display string for search/display |
| `doc_comment` | no | Attached doc comment text |
| `parent_id` | no | Enclosing module/impl/type |

## Relations

| Relation | From | To | Example |
|----------|------|-----|---------|
| `defines` | module | symbol | module contains function |
| `contains` | type/impl | field/method | struct contains field |
| `references` | symbol | symbol | type usage (syntax-level) |
| `calls` | function | function | call expression |
| `imports` | module | symbol | `use` target |
| `implements` | impl | trait | `impl Trait for Type` |

### Edge record

```json
{
  "id": "edge://42/calls/7f3a",
  "relation": "calls",
  "from_id": "sym://42/src/main.rs#function:main@100:500",
  "to_id": "sym://42/src/query/parser.rs#function:parse_query@1204:1890",
  "call_site": {
    "path": "src/main.rs",
    "start_byte": 320,
    "end_byte": 340
  },
  "workspace_rev": 42,
  "record_version": 1
}
```

`to_id` may be unresolved (best-effort name match) when target is in another crate without resolve — marked `resolved: false` (TBD at implementation).

## See also

- [index-schema.md](index-schema.md)
- [query-language.md](query-language.md)
- [../techniques/structural-indexing.md](../techniques/structural-indexing.md)
