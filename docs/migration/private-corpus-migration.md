# Private Corpus Migration Plan

The old private repository may contain useful breakdown data, but none of that data belongs in this public repository.

## Migration Target

Private source data is imported into the user's local runtime library:

```text
library/
  sources/
  analyses/
  works/
  runs/
  recipes/
  indexes/
```

The runtime library is ignored by Git.

## Migration Flow

1. User selects a local private source directory.
2. The app performs a dry-run scan.
3. The app reports source paths, target record types, hashes, coverage, and unsupported files.
4. The user approves selected items.
5. The app writes migrated records into the local library.
6. The app writes a migration report.

## 9.3 Dry-Run Contract

The 9.3 implementation exposes a dry-run only path:

- Rust core: `dry_run_private_corpus_migration`
- Tauri command: `dry_run_private_corpus_migration_command`
- TypeScript contract module: `src/features/migration`
- JSON Schema: `schemas/migration.schema.json`

`MigrationDryRun` returns only metadata:

- `MigrationSource`: selected local root and optional relative include paths.
- `MigrationTarget`: selected local library root.
- `MigrationItem`: relative source path, target record kind, stable content hash, scope, status, and blocked reason.
- `MigrationReport`: counts and a stable source tree hash.

The dry-run result does not include file contents. It also does not create target library directories or migrated records.

## Path And Hash Rules

- Include paths must be relative to the selected source root.
- Absolute include paths and parent-directory traversal are rejected before scanning.
- Each scanned path is checked with its canonical path before it is reported.
- Hashes are stable across source root locations because they are based on relative paths and bytes, not absolute local paths.

## Safety Rules

- Never commit imported private data.
- Never copy source-book full text into public fixtures.
- Never preserve old repo history in this repo.
- Use synthetic toy fixtures for tests.
- Migration must be path-safe and hash-stable.
