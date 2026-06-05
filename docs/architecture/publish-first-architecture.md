# Publish-First Architecture Decision

InfiniteTypewriter is built for public release from the start. The project does not implement a throwaway local-only v1 first.

## Decision

Use Tauri v2 + React/TypeScript + Rust core + SQLite + provider adapters.

## Rationale

- Public release needs clean separation between application code, user data, and private migration inputs.
- Rust core gives a clear place for filesystem safety, schema validation, migrations, provider command construction, and redaction.
- React/TypeScript supports a rich workspace UI.
- SQLite fits local-first indexing, manifests, job history, and state ledgers.
- Provider adapters let API providers, CLI agents, and fake/test providers share one job lifecycle.

## Non-Goals

- Do not port the old private Electron prototype as-is.
- Do not publish private corpus data.
- Do not make AI a background watcher.
- Do not make provider credentials part of repository state.
