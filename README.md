# InfiniteTypewriter

InfiniteTypewriter is a local-first novel creation workbench for evidence-based source breakdown, original planning, draft generation, revision tracking, and provider-orchestrated AI jobs.

This public repository is intentionally clean: it does not include private source books, private breakdown data, local user libraries, provider secrets, or generated release artifacts.

## Public Repository Guardrails

This repository must remain publishable after every change. Do not commit private corpus directories, local user libraries, environment files, database files, logs, run outputs, caches, or release artifacts.

Run the boundary check before claiming repository guardrail work is complete:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File tests/repository-boundary.ps1
```

## Current Route

- Build for public release from the start.
- Target architecture: Tauri v2 + React/TypeScript + Rust core + SQLite + provider adapters.
- User data lives in a local `library/` directory, not in the Git repository.
- Existing private breakdown directories can be imported by a migration wizard, but are never committed here.

## Provider Policy

AI providers are explicit task executors. Local edits do not call AI. Provider jobs must be auditable and fail-closed by default.

Planned provider families:

- API providers: OpenAI-compatible, Anthropic, Gemini, Ollama.
- CLI agent providers: Codex, Claude Code.
- Fake/test provider for CI and demos.

## Start Here

Read `AGENTS.md`, then `docs/WORKING_GUIDE.md`.
