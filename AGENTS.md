# InfiniteTypewriter Agent Instructions

## Highest Priority

This repository is the public release repository. Do not copy private corpus data, old repository history, source-book text, local analysis outputs, provider secrets, or generated user libraries into this repo.

Forbidden paths/content:

- `Textbook/`
- `analysis/`
- `library/`
- `.env` or provider credentials
- local DB files
- source-book excerpts from private corpora
- generated logs, runs, release artifacts, caches

## Workflow

- Read `docs/WORKING_GUIDE.md` before planning or implementation.
- Work from the checklist in order unless the user explicitly redirects.
- Do not create a git worktree without explicit user approval.
- Keep this repository clean and publishable after every change.
- Do not use the old `<private-old-repo>` repo as an implementation target. It is only a private migration/reference source.

## AI Provider Rules

- AI is an explicit task executor, not a background watcher.
- Local saves must not trigger provider calls.
- Provider jobs must record provider, model, permissions, context hash, prompt template version, output schema, stdout/stderr or API response summary, errors, and usage.
- Default provider behavior must be fail-closed.

## Verification

Before claiming completion, run the relevant tests and a repository boundary scan. At minimum, for documentation-only bootstrap changes run:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File tests/repository-boundary.ps1
git status --short --branch
rg -n "Textbook|analysis/|<private-old-repo>|API_KEY|SECRET|TOKEN" .
```

Matches inside `AGENTS.md` or `docs/WORKING_GUIDE.md` that describe forbidden paths are acceptable; matches in app source, fixtures, build output, or release artifacts are not.


