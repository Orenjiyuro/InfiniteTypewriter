# Evidence-Based Breakdown Methodology

This public methodology describes how InfiniteTypewriter should represent source breakdowns without copying source-book events, proprietary objects, dialogue, or relationship chains into generated works.

## Core Principle

Breakdown is evidence-based analysis, not summary. Each reusable mechanism must be traceable to evidence and must describe what can be transferred and what must not be copied.

## Three Layers

| Layer | Purpose | Use |
|---|---|---|
| Source evidence | Location, scene, quote pointer, observed behavior | Audit and verification only |
| Editorial understanding | What the technique does in the source | Human-readable planning |
| Transfer mechanism | Abstracted mechanism, variables, constraints, forbidden copies | Original project generation |

## Evidence Card

Minimum public contract fields:

- `source`: source id, public-safe title, and locator.
- `evidence`: one or more locator records with short public-safe evidence summaries.
- `observedTechnique`: the writing move visible at the evidence location.
- `sourceFunction`: what that move does inside the source.
- `transferableMechanism`: the abstract mechanism that can be reused.
- `forbiddenCopy`: source events, names, dialogue, relationship chains, or proprietary objects that must not be copied.
- `coverage`: scope and notes for what the card actually covers.
- `confidence`: low, medium, or high, with rationale.

An Evidence Card without `evidence`, `source`, `coverage`, or `confidence` is invalid. If a claim is not evidenced, it must stay out of the Evidence Card or be represented as an uncovered hypothesis elsewhere.

## Reader Contract / Promise Progress Payoff

Record genre promise, tone promise, emotional promise, promise progress, delay, and payoff timing. A promise can be reused only as an abstract expectation pattern, not as the original event.

The public schema records `promise`, `progress`, `payoff`, `tone`, `emotionalTarget`, and links to supporting Evidence Cards.

## Scene Card

Record POV, time/place, character goals, conflict agenda, information revealed/hidden, state change, turn, and which reader promise the scene advances.

The public schema keeps scene planning abstract: goals, conflict agenda, reveal/hide lists, state change, and turn. It references Knowledge Boundary, Setting Pressure, and Reader Contract records by id instead of embedding private source material.

## POV / Knowledge Boundary

Record who narrates, who knows what, what the reader knows, what the author knows but cannot reveal yet, and whether narration is unreliable.

The public schema names this object `KnowledgeBoundary` and records POV, narrator knowledge, reader knowledge, withheld information, and unreliability.

## Setting as Pressure

A setting item is useful only if it restricts someone, grants resources to someone, imposes a cost, or changes a character choice. Encyclopedia-like setting notes are not enough.

The public schema names this object `SettingPressure` and requires a setting element, restriction, resource, cost, and choice impact.

## Layered Revision Workflow

Revisions are separated into structure, character, scene, prose style, and continuity. A temporary breakthrough is not automatically a permanent character change.

`DraftReviewItem` records review findings against a target id with severity, category, recommendation, and supporting Evidence Card ids. `ChangeSet` records accepted changes, affected ids, and a short summary so later work can audit what changed without copying source text.

## Context Pack Contract

`ContextPack` is the 9.2 root methodology schema. It groups Evidence Cards, Reader Contracts, Scene Cards, Knowledge Boundaries, Setting Pressures, Reference Mechanisms, Draft Review Items, and Change Sets into a public-safe planning packet. This contract defines shape only; context selection, hashing, provider execution, and migration behavior are later checklist items.
