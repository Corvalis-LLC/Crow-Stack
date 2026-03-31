---
name: review
description: "Code review workflow inspired by CodeRabbit. Analyzes git changes across 9 dimensions including security, logic, tech debt, duplication, cross-file impact, and testing gaps. Produces a walkthrough, changes table, categorized findings, and orchestrates fixes via subagents. User-invocable via /review command."
---

# Code Review Workflow

<command-name>review</command-name>

## Overview

Comprehensive code review on git changes. Produces a structured report (walkthrough, changes table, categorized findings), then fixes approved issues via subagents.

```
1. Detect Changes → 2. Load Skills → 3. Analyze (9 categories) → 4. Report → 5. Fix → 6. Verify → 7. Commit
```

## Step 1: Detect Changes

Determine what to review based on arguments:

| Invocation | Git Command |
|------------|-------------|
| `/review` (default) | `git diff HEAD` — all uncommitted changes |
| `/review --staged` | `git diff --cached` — staged only |
| `/review --branch main` | `git diff main...HEAD` — branch comparison |

Get the list of changed files: `git diff HEAD --name-only`

## Step 2: Load Relevant Skills

Based on changed files, load skills automatically:

| File Pattern | Skills |
|--------------|--------|
| `*.svelte` | auto-svelte, auto-accessibility, auto-layout |
| `*.css`, `*.scss`, Tailwind classes, StyleSheet | auto-layout |
| `*.ts` (non-test) | auto-typescript |
| `+server.ts`, `api/**` | auto-security, auto-errors |
| Auth/login/session files | auto-security, auto-compliance |
| User data/PII handling | auto-compliance |
| Any code changes | auto-naming, auto-comments, auto-edge-cases |
| `*.rs`, Rust code | auto-errors, auto-logging, auto-concurrency |
| Logging/tracing changes | auto-logging |
| Test files (`*.test.*`, `*_test.*`) | auto-test-quality |
| Async/concurrent code | auto-concurrency |
| Config/env loading | auto-silent-defaults |
| Resource management (files, connections, listeners) | auto-resource-lifecycle |
| Service URLs, timeouts, magic numbers, env vars | auto-hardcoding |
| HTTP calls, external APIs, webhooks, retries | auto-resilience |
| REST endpoints, response shapes, pagination | auto-api-design |
| SQL queries, ORM code, database operations | auto-database |
| Schema migrations, column renames, breaking changes | auto-evolution |
| Serializable types, JSON payloads, serde/zod/pydantic | auto-serialization |
| Cache implementations, TTL, invalidation | auto-caching |
| Job queues, background workers, retry logic | auto-job-queue |
| Health checks, metrics, tracing, observability | auto-observability |
| File I/O, temp files, atomic writes, streaming | auto-file-io |
| State enums, status fields, transition logic | auto-state-machines |
| Translation strings, locale formatting, pluralization, RTL | auto-i18n |

## Step 3: Analyze Across 9 Categories

Each finding gets a **category** (domain) and a **comment type** (action level):

### Comment Types
| Type | Action |
|------|--------|
| **Issue** | Will or could cause bugs/failures — must fix |
| **Suggestion** | Quality improvement — should fix |
| **Nitpick** | Minor style/convention — collapsed separately |

### Categories
1. **Security** — Injection, auth gaps, secrets, rate limiting
2. **Logic & Correctness** — Business logic errors, race conditions, edge cases
3. **Duplicated Code & Tech Debt** — Code clones, dead code, TODOs, magic numbers, growing functions
4. **Type Safety** — `any` usage, unsafe assertions, missing null checks
5. **API Design** — Response format, validation, error handling (see auto-errors)
6. **Accessibility** — ARIA, keyboard nav, contrast, focus management
7. **Performance** — N+1 queries, missing pagination, sequential fetches
8. **Cross-File Impact** — Broken imports, interface changes affecting callers, missing co-changes
9. **Testing Gaps** — New code without tests, modified code with stale tests

For detailed analysis criteria per category, see **[analysis-categories.md](references/analysis-categories.md)**.

## Step 4: Generate Report

The report follows this structure:

1. **Walkthrough** — Narrative paragraph explaining what the changes do and why
2. **Changes Table** — File-by-file summary table
3. **Findings** — Grouped by severity: Critical → Issues → Suggestions → Nitpicks (collapsed)
4. **Tech Debt Summary** — Table of duplication, dead code, TODOs (only if found)
5. **Passed Checks** — Collapsed section of what looks good

For full report templates and examples, see **[report-templates.md](references/report-templates.md)**.

## Step 5: Present and Await Response

Present findings directly — do NOT use AskUserQuestion.

```
## Code Review Complete

Found [N] issues in your changes:
- [X] Critical / [Y] Issues / [Z] Suggestions / [W] Nitpicks

[Full report above]

Reply with what you'd like me to do (e.g., "fix all", "fix critical only", "skip and commit").
```

## Step 6: Fix Issues with Subagents

For approved fixes, launch parallel subagents grouped by file. Each subagent:
- Gets the file path, list of fixes, and relevant skill context
- Applies fixes only — no other changes, no explanatory comments
- Uses `model='sonnet'` for efficiency

After fixes:
1. `git diff` to verify changes
2. Check for new issues introduced
3. Present final summary

## Step 7: Commit

When user approves, stage specific files (follow auto-git — **never** `git add -A` or `git add .`):

```bash
git add <specific-fixed-files>
git commit -m "fix: address code review findings
..."
```

## Skill Dependencies

This skill orchestrates: auto-security, auto-typescript, auto-accessibility, auto-svelte, auto-compliance, auto-coding, auto-errors, auto-naming, auto-comments, auto-logging, auto-edge-cases, auto-resource-lifecycle, auto-concurrency, auto-test-quality, auto-silent-defaults, auto-hardcoding, auto-resilience, auto-api-design, auto-database, auto-evolution, auto-serialization, auto-caching, auto-job-queue, auto-observability, auto-file-io, auto-state-machines, auto-layout, auto-i18n

## Reference Files

- **[references/analysis-categories.md](references/analysis-categories.md)** — Detailed criteria for all 9 analysis categories including tech debt detection, cross-file impact, and duplication patterns
- **[references/report-templates.md](references/report-templates.md)** — Walkthrough, changes table, findings format, tech debt summary, commit message template, fix dispatch pattern
