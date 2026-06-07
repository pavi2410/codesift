# VFS and Workspace

**Status:** accepted

Virtual file system, workspace roots, and file identity.

**MVP scope:** Single root, `.rs` files only, hash-based skip. File watcher and git integration deferred.

## Workspace

A workspace is a directory tree to index.

```json
{
  "root": "/path/to/repo",
  "roots": ["/path/to/repo"],
  "index_path": ".codesift",
  "workspace_rev": 42
}
```

Multi-root workspaces (monorepo): `roots` array — **future**; MVP single root.

## File discovery

Uses `ignore` crate (ripgrep ecosystem):

1. Walk from workspace root
2. Apply `.gitignore`, `.ignore`, `.codesiftignore`
3. Include extensions from language config
4. Skip binary files (NUL byte detection or extension denylist)

### Default included extensions (MVP)

| Extension | Language |
|-----------|----------|
| `.rs` | rust |
| `.md`, `.mdx` | markdown |

`multi-language` capability adds `.ts`, `.py`, `.go`, etc.

## File identity

| Field | Description |
|-------|-------------|
| `path` | Repo-relative, `/` separators, normalized |
| `content_hash` | blake3 of file bytes |
| `language` | Detected language id |
| `mtime` | Optional; not used for correctness |

Correctness uses **content_hash**, not mtime.

## Ignore files

### `.codesiftignore`

Gitignore syntax. Additional excludes beyond `.gitignore`:

```
target/
*.generated.rs
vendor/
```

### Defaults always ignored

```
.codesift/
.git/
```

## File watcher (`watch` mode)

- `notify` RecursiveMode on workspace root
- Debounce: 300ms coalesce rapid saves
- Events: Create, Modify, Remove, Rename
- Rename: treat as remove old + add new path

## Git integration (future)

| Feature | Phase |
|---------|-------|
| Index at `git rev-parse HEAD` | 3 |
| Dirty overlay (uncommitted changes) | 3 |
| Index per branch | future |

`meta.json` may record `git_commit` when run inside git repo.

## See also

- [incremental-indexing.md](incremental-indexing.md)
- [storage.md](storage.md)
