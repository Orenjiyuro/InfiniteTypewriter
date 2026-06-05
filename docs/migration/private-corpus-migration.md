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

## Safety Rules

- Never commit imported private data.
- Never copy source-book full text into public fixtures.
- Never preserve old repo history in this repo.
- Use synthetic toy fixtures for tests.
- Migration must be path-safe and hash-stable.
