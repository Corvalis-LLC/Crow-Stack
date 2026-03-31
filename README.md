# Corvalis Skills

A Claude Code skill system for automated software development. Corvalis provides a layered execution pipeline — from session bootstrap through autonomous multi-stream orchestration — plus a comprehensive set of coding discipline skills that enforce quality standards across every line Claude writes. Install the skills, run `/summon`, and let the system handle planning, parallelization, and standards enforcement.

## The Execution Stack

```
  /summon          Session bootstrap — brainstorm, plan, validate
     │
     ▼
  /dominion        Autonomous orchestrator — spawns headless Claude instances
     │
     ├──► /stream [A]    ──► legion wave 1 ──► wave 2 ──► ...
     ├──► /stream [B]    ──► legion wave 1 ──► wave 2 ──► ...
     │         (parallel if no dependency)
     ▼
  /stream [C]      Waits for A & B, then executes
     │
     ▼
  Done             All streams complete, plan verified
```

1. **`/summon`** bootstraps a new session. You choose: plan, no-plan, or talk-it-out. The planning path writes a structured plan to `docs/plans/`, validates it against quality standards, and optionally runs `/triumvirate` for adversarial review.
2. **`/dominion`** reads the plan and autonomously executes every stream — spawning parallel headless Claude instances where dependencies allow. This is the hands-off mode.
3. **`/stream`** executes a single stream per session. It loads the relevant skills, enforces verification gates, and marks completion so the next session knows where to pick up. Use this instead of `/dominion` when you want hands-on control.
4. **Legion** (loaded automatically by `/stream` when annotated) decomposes a stream's work into TDD-phased waves of parallel background agents, each given minimal context for maximum efficiency.

## Installation

### Quick Install (symlink)

Clone this repo and run the install script:

```bash
git clone <this-repo-url> auto-skills
cd auto-skills
chmod +x install.sh
./install.sh
```

The script symlinks each skill directory into `~/.claude/skills/`. Existing skills with the same name are backed up to `~/.claude/skills-backup-<timestamp>/`.

### Manual Install

Copy the `skills/` directory contents to your Claude Code skills directory:

```bash
cp -R skills/* ~/.claude/skills/
```

## Quick Start

1. Install the skills (see above)
2. Start a new Claude Code session
3. Run `/summon`
4. Choose **Plan** — describe what you're building
5. The system writes a plan, validates it, and recommends next steps
6. Run `/dominion` to execute the full plan autonomously, or `/stream` to execute one stream at a time

## Skill Reference

### Workflow & Orchestration

| Skill | Description |
|-------|-------------|
| `summon` | Session bootstrap — offers plan, no-plan, or talk-it-out paths |
| `dominion` | Autonomous plan executor — spawns headless Claude instances per stream |
| `stream` | Per-stream executor with dependency tracking and verification gates |
| `auto-legion` | Parallel agent waves within a stream (T→I→D→R phases) |
| `auto-workflow` | TDD enforcement, verification before completion, architecture escalation |
| `triumvirate` | Adversarial plan review with three subagents (Advocate, Analyst, Critic) |

### Tools

| Skill | Description |
|-------|-------------|
| `review` | Code review across 9 dimensions (security, logic, tech debt, etc.) |
| `design` | UI/UX design system with auditing, generation, and style migration |
| `skill-creator` | Create, modify, eval, and benchmark skills |
| `security-scan` | Active vulnerability scanner (dangerous patterns, secrets, npm audit) |

### Coding Disciplines (auto-*)

| Skill | Description |
|-------|-------------|
| `auto-coding` | Language-agnostic code quality, clarity, anti-over-engineering |
| `auto-comments` | When to comment, when silence is the comment |
| `auto-naming` | Domain vocabulary over generic words, verb semantics |
| `auto-hardcoding` | No hardcoded URLs, ports, timeouts, or magic numbers |
| `auto-silent-defaults` | When defaults mask errors and missing data should fail loudly |
| `auto-errors` | Actionable error messages, audience-appropriate wording |
| `auto-logging` | Log level selection, structured fields, what to log vs not |
| `auto-edge-cases` | Empty collections, zero inputs, off-by-one, overflow, Unicode |
| `auto-test-quality` | Meaningful assertions, mock boundaries, tautological test detection |
| `auto-concurrency` | Race conditions, atomicity, lock ordering, TOCTOU bugs |
| `auto-resource-lifecycle` | Guaranteed cleanup on all paths, RAII/context managers |
| `auto-resilience` | Timeouts, retries with backoff, circuit breaking, idempotency |
| `auto-caching` | Stampede protection, invalidation strategy, stale-while-revalidate |
| `auto-file-io` | Atomic writes, streaming large files, error path cleanup |
| `auto-state-machines` | Explicit state enums, transition validation, impossible state elimination |
| `auto-serialization` | Decimal precision, timezone-aware datetimes, forwards-compatible enums |
| `auto-evolution` | Backwards-compatible schema/API changes, rolling deploy safety |
| `auto-observability` | Metrics vs logs vs traces, health check depth, SLO-oriented measurement |
| `auto-database` | Cursor pagination, index awareness, N+1 prevention, bulk operations |
| `auto-api-design` | Response envelopes, HTTP status codes, cursor pagination, DTOs |
| `auto-job-queue` | Idempotent processing, poison pill protection, dead letter handling |
| `auto-security` | Session token hashing, auth hiding, timing-safe flows, cookie hardening |
| `auto-compliance` | GPC headers, data deletion gates, consent proof, regulatory escalation |
| `auto-accessibility` | ARIA completeness, touch targets, forced-colors/reduced-motion, WCAG 2.2 |
| `auto-layout` | Card restraint, grid over flex for 2D, design tokens, z-index management |
| `auto-i18n` | ICU pluralization, locale-aware formatting, RTL support |

### Language-Specific

| Skill | Description |
|-------|-------------|
| `auto-typescript` | Type safety — eliminates `as any`, enforces narrowing, branded types, Zod pitfalls |
| `auto-python` | Type hints, async patterns, pytest, dataclasses, uv/ruff/mypy |
| `auto-svelte` | Svelte 5 gotchas — SSR state, `$state.raw`, `$effect` discipline |

## Architecture

The system is organized in three layers:

### Layer 1: `/dominion` — Autonomous Orchestrator

Reads a multi-stream plan and executes the entire thing without human intervention. Spawns one headless Claude instance per eligible stream, runs them in parallel where dependencies allow, monitors their status files, and cascades to the next wave when streams complete. Choose `/dominion` when you want to walk away.

### Layer 2: `/stream` — Per-Stream Executor

Executes a single stream within a plan. Loads the skills relevant to that stream's work, tracks progress in a `.status.json` companion file, enforces verification gates (tests must pass before marking complete), and coordinates with other streams via dependency checks. When a stream is annotated for legion mode, `/stream` loads `auto-legion` to parallelize the work further.

### Layer 3: `auto-*` — Discipline Skills

Auto-triggered coding standards that activate based on what you're doing. Writing a database query? `auto-database` loads. Adding error handling? `auto-errors` activates. These skills encode the patterns Claude knows but inconsistently applies — they make the quality floor consistent.

## Key Concepts

### Streams

A stream is an independent unit of work within a plan. Each stream has file ownership boundaries (no two streams edit the same file), explicit dependencies on other streams, and a set of tasks. Streams can execute in parallel when they have no dependency relationship.

### Legion Waves (T→I→D→R)

Legion decomposes a stream into phased waves following TDD progression:

- **T (Test)** — Write tests first, in parallel per module
- **I (Implement)** — Implement against the tests, in parallel per module
- **D (Debug)** — Fix any failing tests
- **R (Refine)** — Polish, optimize, clean up

Each wave dispatches multiple background agents with minimal, surgical context. The orchestrator verifies between waves before proceeding.

### Dependency Optimization

Plans declare stream dependencies explicitly. `/dominion` builds a DAG and identifies the maximum parallelism — streams with no shared dependencies run simultaneously. The parallelization section of a plan shows which streams can overlap.

### The Status File

Each plan gets a `.status.json` companion file that tracks which streams are complete, in-progress, or blocked. This allows `/stream` to resume across sessions and `/dominion` to monitor progress across its spawned instances.

### Zero-Tolerance Helper Mode

Corvalis discipline skills don't suggest improvements — they enforce them. When a skill detects a violation (e.g., `SELECT *` in a query, a bare network call without timeout), it corrects the code directly rather than leaving a comment. The quality floor is non-negotiable.

### The Skill Gate

During planning, `/summon` assigns a concrete list of auto-* skills to each stream. A baseline set (`auto-workflow`, `auto-coding`, `auto-errors`, `auto-naming`, `auto-edge-cases`) loads unconditionally for every stream. Additional skills are assigned per-stream based on what that stream touches — the user reviews and approves the assignments. These are written into the plan's `## Required Skills` section and flow into the status file's `baselineSkills` field, so `/stream` loads exactly the right skills without heuristic guessing.

### The Parallelization Gate

Before `/dominion` spawns parallel streams, it validates that file ownership boundaries don't overlap. If two streams touch the same file, they cannot run in parallel regardless of their declared dependencies. This prevents merge conflicts and race conditions in the codebase.
