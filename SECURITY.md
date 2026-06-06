# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in codesift, please report it responsibly.

**Do not** open a public GitHub issue for security-sensitive reports.

Instead, email the maintainers (contact address TBD at first public release) with:

- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested fix (optional)

We aim to acknowledge reports within 5 business days and provide a status update within 14 days.

## Scope

This policy covers the codesift CLI, MCP server, and library crates once implementation begins.

## Sensitive data in indexes

codesift indexes **local repository content**. A `.codesift/` index directory may contain:

- Source code snippets and embeddings derived from your codebase
- Symbol names, paths, and structural metadata
- Potentially sensitive strings from indexed files

**Recommendations:**

- Add `.codesift/` to `.gitignore` (already documented in the project `.gitignore`)
- Do not commit index directories to version control
- Treat index exports (JSONL) as sensitive if the source repo is sensitive
- Run indexing in CI only on code you are permitted to process

## Dependency security

Once Rust code lands, we will use `cargo audit` (or equivalent) in CI to track known vulnerabilities in dependencies.
