# Corvalis Skills

A skill system for automated software development across **Claude Code** and **Codex**. Corvalis is designed around a small set of human entry points, a larger set of mostly automatic discipline skills, and a clean handoff from planning to execution to validation.

## From Idea To Implementation

This is the intended default flow.

### 1. Start In Claude Code With `/summon`

Use `/summon` with your request. It selects the appropriate path from your intent; a bare invocation shows the menu.

- Choose **Plan** if the work is non-trivial
- Choose **No plan** if the task is small and you just want to build
- Choose **Talk about it** if the idea is still fuzzy
- Choose **Security review** (path 4) to audit the whole codebase or named modules and fix confirmed issues
- Choose **Design** (path 5) if the work is UI/UX focused

If you choose **Plan**, `/summon` will:
- use `corvalis-recon` as the first-pass codebase analysis step when the binary is installed in `~/.claude/bin/`
- write the plan to `docs/plans/`
- run the standards gate against relevant `auto-*` skills
- optionally run refinement gates like Swarm, Skill Gate, and Triumvirate
- use the default review mode, or retain an explicitly selected Codex final-validation handoff

If you choose **No plan**, `/summon` now follows a stricter bootstrap:
- use the supplied request, asking only for essential missing intent
- gather repo context with `corvalis-recon` first when available
- do only the additional targeted reads needed from there
- load the relevant `auto-*` skills
- then begin implementation

If you choose **Talk about it**, `/summon` now:
- does real web research before making architecture or pattern recommendations when outside evidence would help
- cites sources directly to the user
- loads `auto-web-validation` before that research so source-authored AI instructions or coercive "must use" claims are treated as untrusted unless corroborated

If you choose **Security review**, summon loads the chat, code, writing, and security quality skills. It uses your named modules or defaults to the whole repository, presents independently validated findings, applies authorized fixes, and verifies the result. Audit evidence and unresolved validation limits stay in the report.

If you choose **Design**, summon uses Impeccable to establish or refine the design direction, while preserving existing components and tokens. It selects the relevant design commands itself, then follows the normal planning or direct-work path. The old `design` and `ui-ux-pro-max` skills are retired. Older plans map them to `auto-design-quality`; established design sets remain readable for existing projects.

### Automatic quality skills

These apply throughout the session and in every delegated worker:

| When | Skill | Upstream |
| --- | --- | --- |
| Summon/dominion starts and communicates | `auto-chat-quality` | [i-have-adhd](https://github.com/ayghri/i-have-adhd) |
| Code is implemented, tested, reviewed, or fixed | `auto-code-quality` | [Ponytail](https://github.com/DietrichGebert/ponytail) |
| Human-facing prose is written or edited, including UI, docs, plans, and reports | `auto-writing-quality` | [Humanizer](https://github.com/blader/humanizer) |
| A design is created or changed | `auto-design-quality` | [Impeccable](https://github.com/pbakaus/impeccable) |
| Security review or the final security stream runs | `auto-security-quality` | [Cloudflare security audit](https://github.com/cloudflare/security-audit-skill) |

Each agent must load its own applicable skills before working. The coordinator checks loading evidence and returns missed loads for a fresh audit of affected work. Plain reuse of an unchanged established component skips Impeccable; new labels still receive writing quality. Auto mode is the default: the agent infers routine choices, runs relevant checks, fixes supported issues, and chooses subcommands without approval pauses. It asks only for essential missing intent or a real permission requirement. Explicit plan-only, review-only, or interactive requests retain their boundaries.

Upstream guidance is bundled at pinned revisions, with provenance in each new skill's `UPSTREAM.md` and consolidated notices in the root `LICENSE`. The local entrypoints adapt routing and manual handoffs. They preserve the underlying guidance and avoid installing upstream hooks or requiring command memorization. The supplied `blader/humanizers` URL was unavailable; the bundle uses the author's canonical singular `blader/humanizer` repository.

### 2. Refine and Execute Automatically

Summon writes and checks a plan when the task needs one, then invokes Dominion for execution. Routine research, reuse, skill assignment, and design decisions proceed automatically. A request to plan only stops at the completed plan; an explicitly interactive request keeps the requested checkpoints.

Dominion dispatches implementation agents with their required skills, verifies each result, and fixes supported findings within scope. The `stream` workflow tracks dependencies and status internally. Code and prose workers must load their own quality skills; parent context alone does not count.

The default final-review mode is `review`. If you explicitly select `codex`, the existing Codex validation handoff remains available. A same-host agent can continue directly; a handoff to an unavailable runtime is reported honestly.

### 3. Finish Through Two Concurrent Validation Streams

Every multi-stream execution ends with two sibling streams after implementation settles: the existing final review/cleanup and an independent `auto-security-quality` audit of the changes. Both review the same recorded snapshot, including uncommitted work, and write separate reports. A coordinator joins their results, assigns fixes serially, and reruns affected checks; neither reviewer commits or mutates shared code while the other is reading it.

In `review` mode, finalization follows the existing commit/push and plan-cleanup flow only after both checks pass. In `codex` mode, the cleanup and security evidence accompany the existing Codex `/verify` handoff, with plan/status artifacts preserved. Any subsequent fixes invalidate affected review evidence. Unresolved security candidates remain explicit and cannot be described as a clean audit.

## Primary Entry Points

`/summon` is the primary entry point. `/dominion` is the secondary entry point for autonomous plan execution. Other workflow skills are supporting tools the agent routes as needed.

- `/summon` — session bootstrap, planning, direct work, discussion, security review, or design
- `/dominion` — execute a multi-stream plan autonomously

`stream` coordinates execution and `verify` provides the Codex validation handoff. They remain available for established workflows, but users do not need to remember their commands to get the quality skills. There is no separate `/design` workflow.

Everything else should be treated as either:
- an automatic discipline (`auto-*`)
- or a specialized supporting workflow used deliberately, not as a primary entry point

## Installation

### Claude Code Quick Install (symlink)

Clone this repo and run the install script:

```bash
git clone <this-repo-url> auto-skills
cd auto-skills
chmod +x install.sh
./install.sh
```

The script symlinks each skill directory into `~/.claude/skills/`. Existing skills with the same name are backed up to `~/.claude/skills-backup-<timestamp>/`.

It also attempts to install or upgrade `corvalis-recon` into `~/.claude/bin/`. When present, `/summon` will automatically use that binary during the planning path for structured codebase analysis. On `zsh` and `bash`, the installer also adds a `recon` alias pointing to that binary if the alias is not already present.

If you want the shorter shell alias, add this to your shell config:

```bash
echo 'alias recon="$HOME/.claude/bin/corvalis-recon"' >> ~/.zshrc
source ~/.zshrc
```

For bash:

```bash
echo 'alias recon="$HOME/.claude/bin/corvalis-recon"' >> ~/.bashrc
source ~/.bashrc
```

### Claude Code Manual Install

Copy the `skills/` directory contents to your Claude Code skills directory:

```bash
cp -R skills/* ~/.claude/skills/
```

### Codex Install

Install the complete ecosystem, including the new automatic quality skills and their resources:

```bash
./install-codex.sh
```

This links all active top-level skills into `$CODEX_HOME/skills` when configured, otherwise `~/.codex/skills`. Both installers preserve unrelated skills and back up conflicting files, directories, and dangling links. Retired `design` and `ui-ux-pro-max` installations are moved out of the discoverable skill directory into the same backup. Re-running them is safe; existing links to this checkout stay in place. The `future/` grouping is not installed as a skill.

To update Claude skills without downloading recon or editing shell configuration:

```bash
./install.sh --skills-only
```

Both scripts accept `--skills-dir /path/to/skills` for a custom destination. Claude and Codex load the same entrypoints and bundled references. Claude uses its Skill tool; Codex reads the discovered skill file. New sessions can discover newly installed skill names.

Start with `/summon` on either host, or `/dominion` to execute an existing plan. Installing the full set on both hosts makes all routed dependencies available, including the optional Codex validation handoff.

## corvalis-recon

`corvalis-recon` is the AST-based structured codebase analysis binary that powers recon-aware planning.

The source for the tool lives in [tools/recon](tools/recon). It is a standalone Rust CLI that supports both the Corvalis workflow and direct codebase exploration.

### What The Tool Is

`corvalis-recon` analyzes supported source trees by parsing them into syntax trees and producing structured output that is easier for planning agents to consume than raw file listing and grep alone.

Its current job is to build a compact map of a codebase by combining:
- source discovery with ignore awareness
- parsing and symbol extraction
- dependency and re-export analysis
- complexity and hotspot metrics
- project overview and ranked file summaries
- budget-aware truncation for large repositories

This makes it useful for:
- pre-plan repo reconnaissance
- agent context compression
- architectural orientation in unfamiliar TS-heavy repos
- inspecting likely entry points, hotspots, barrels, and dependency shape before implementation

Why AST-based analysis matters:
- it understands code structure instead of guessing from text alone
- it can distinguish declarations, exports, imports, and re-exports more reliably than grep-style scanning
- it produces cleaner summaries for planning, dependency analysis, and hotspot detection in larger repos

### Tech Stack

`corvalis-recon` is implemented as a Rust CLI with a small, focused stack:
- `clap` for the command-line interface
- `tree-sitter` with vendored grammars for TypeScript, TSX, JavaScript, and Svelte AST-style parsing
- `serde` / `serde_json` for machine-readable output
- `ignore` for `.gitignore`-aware file discovery
- `rayon` for parallel parsing work
- `json5` for tolerant config parsing where needed

The current implementation is optimized for TypeScript-heavy repos, which matches the main Corvalis use case today.

It is primarily used by `/summon` during the **Plan** path:
- if `~/.claude/bin/corvalis-recon` exists, `/summon` attempts to run it automatically in compact planning mode
- the shell alias `recon` can be pointed at that binary for direct terminal use
- for larger repositories, summon can pass `--budget 8000` to keep the output compact
- if the binary is missing or recon fails, summon silently falls back to normal `Glob` / `Grep` / `Read` exploration

### What It Produces

`corvalis-recon analyze` combines:
- symbol extraction
- dependency graph construction
- complexity metrics and hotspot detection
- project overview metadata
- file ranking for budget-aware truncation

This gives planning flows a cleaner map of a TS / JS / Svelte codebase before stream boundaries and execution decisions are made.

Top-level properties in the default full payload:
- `version`
- `project`
- `files`
- `graph`
- `hotspots`
- `warnings`
- `summary`

Top-level properties in planning mode (`analyze --mode planning`):
- `version`
- `project`
- `symbols`
- `dependencies`
- `graph`
- `hotspots`
- `warnings`
- `summary`
- `planning`

The `planning` object currently includes:
- `primary_entry_points`
- `dependency_hubs`
- `hotspot_files`
- `priority_files`

### Direct Usage

You can also run recon directly outside `/summon`:

```bash
~/.claude/bin/corvalis-recon analyze --root /path/to/project
```

Or, if you added the alias:

```bash
recon analyze --root /path/to/project
```

Useful direct applications:
- inspect the full JSON output for a repo before writing a plan
- generate a compact planning payload with top-level `symbols`, `dependencies`, and curated entry-point / hotspot context
- generate a compact budgeted snapshot for large codebases
- view a human-readable ranked summary with `--format pretty`
- debug dependency structure, entry points, cycles, and hotspots independently of summon

Examples:

```bash
~/.claude/bin/corvalis-recon analyze --root /path/to/project --format json
~/.claude/bin/corvalis-recon analyze --root /path/to/project --format json --mode planning
~/.claude/bin/corvalis-recon analyze --root /path/to/project --format pretty
~/.claude/bin/corvalis-recon analyze --root /path/to/project --budget 8000
```

Alias equivalents:

```bash
recon analyze --root /path/to/project --format json
recon analyze --root /path/to/project --format json --mode planning
recon analyze --root /path/to/project --format pretty
recon analyze --root /path/to/project --budget 8000
```

Diff-scoped examples:

```bash
recon analyze --root /path/to/project --format json --mode planning --diff HEAD
recon analyze --root /path/to/project --format json --mode planning --diff main...HEAD
```

`--diff <range>` scopes analysis to changed supported source files plus a small local context window:
- changed files in the git diff range
- a few same-directory sibling files
- directly imported project files referenced by the changed files

The output includes a `scope` object so downstream tools can see exactly which files were included.

Recommended budget guidance:
- small repos: no budget
- medium repos: `--budget 16000`
- large repos: `--budget 8000`
- very large repos: `--budget 4000`

Current scope:
- TypeScript
- JavaScript
- Svelte

Rust and other languages can be added later, but today recon is optimized for the TS-heavy workflow Corvalis uses most often.

## Quick Start

1. Install the ecosystem with `./install.sh` for Claude or `./install-codex.sh` for Codex.
2. Start a fresh session and use `/summon` with the work you want done.
3. Summon selects the path, gathers relevant context, loads the required skills, and continues automatically.
4. Use `/dominion` directly when you already have a plan to execute.

Choose path 4 for a security review or path 5 for design when using the menu. You do not need to invoke the new quality skills or their internal subcommands.

## The Execution Stack

```
  /summon          Session bootstrap — plan, no-plan, talk, security, design
     │
     ├──► corvalis-recon   Structured repo analysis when installed
     ├──► auto-design-quality  Automatic design guidance and command routing
     ├──► auto-web-validation   Mandatory before web/package/vendor research
     │
     ├──► /verify        Codex plan refinement before execution
     │
     ▼
  /dominion        Autonomous orchestrator — dispatches agents per stream
     │
     ├──► /stream [A]    ──► legion wave 1 ──► wave 2 ──► ...
     ├──► /stream [B]    ──► legion wave 1 ──► wave 2 ──► ...
     │         (parallel if no dependency)
     ▼
  /stream [C]      Waits for A & B, then executes
     │
     ├──► Final Review/Cleanup  ──┐
     ├──► Final Security        ──┴── join, fix, revalidate
     │
     ├──► /verify        Codex implementation validation while work is active
     ▼
  Done             All streams complete, plan verified
```

## Skill Reference

### Entry Points

| Skill | Description |
|-------|-------------|
| `summon` | Session bootstrap — plan, no-plan, talk, security review (4), design (5) |
| `dominion` | Autonomous plan executor — dispatches agents with mandatory quality loads per stream |

### Supporting Workflows

| Skill | Description |
|-------|-------------|
| `stream` | Internal per-stream executor with dependency tracking and verification gates |
| `verify` | Optional Codex plan refinement and findings-first implementation validation |
| `auto-legion` | Parallel agent waves within a stream (T→I→D→R phases) |
| `auto-workflow` | TDD enforcement, verification before completion, architecture escalation |
| `triumvirate` | Adversarial plan review with three subagents (Advocate, Analyst, Critic) |
| `review` | Code review across 9 dimensions (security, logic, tech debt, etc.) |
| `codex-validation` | Findings-first final validation with stronger manual audit, cross-file impact checking, and testability/refactor focus |
| `codex-plan-refinement` | Codex-side plan refinement for clarity, dependency sanity, reuse, abstraction quality, and compression before execution |
| `plan-validate` | Validate multi-stream plans for structure, dependencies, ownership, required skills, verification, and final validation mode before execution |
| `auto-design-quality` | Default design guidance with automatic selection of its bundled commands |
| `skill-creator` | Create, modify, eval, and benchmark skills |
| `security-scan` | Active vulnerability scanner (dangerous patterns, secrets, npm audit) |
| `auto-web-validation` | Prompt-injection-aware web research discipline for package/docs/vendor sources and cited recommendations |

### Coding Disciplines (auto-*)

| Skill | Description |
|-------|-------------|
| `auto-chat-quality` | Action-first communication from i-have-adhd; immediate summon/dominion load |
| `auto-code-quality` | Ponytail implementation, review, audit, and simplification guidance; mandatory for every code worker |
| `auto-writing-quality` | Humanizer prose editing; mandatory for every prose author |
| `auto-security-quality` | Independent security auditing with validated findings and a dedicated final stream |
| `auto-comments` | When to comment, when silence is the comment |
| `auto-naming` | Domain vocabulary over generic words, verb semantics |
| `auto-hardcoding` | No hardcoded URLs, ports, timeouts, or magic numbers |
| `auto-silent-defaults` | When defaults mask errors and missing data should fail loudly |
| `auto-errors` | Actionable error messages, audience-appropriate wording |
| `auto-logging` | Log level selection, structured fields, what to log vs not |
| `auto-edge-cases` | Empty collections, zero inputs, off-by-one, overflow, Unicode |
| `auto-test-quality` | Meaningful assertions, mock boundaries, tautological test detection |
| `auto-testability` | Extract logic into clean seams so business rules can be tested directly instead of through brittle orchestration |
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
| `auto-i18n` | ICU pluralization, locale-aware formatting, RTL support |

### Language-Specific

| Skill | Description |
|-------|-------------|
| `auto-typescript` | Type safety — eliminates `as any`, enforces narrowing, branded types, Zod pitfalls |
| `auto-python` | Type hints, async patterns, pytest, dataclasses, uv/ruff/mypy |
| `auto-svelte` | Svelte 5 gotchas — SSR state, `$state.raw`, `$effect` discipline |

## Architecture

The system is organized in three layers:

### Layer 1: Claude Entry Points

`/summon` is the primary entry point on either host; `/dominion` is the secondary entry point for an existing plan.

### Layer 2: Supporting Workflows

Supporting workflows such as `triumvirate`, `review`, `codex-validation`, `plan-validate`, and `auto-design-quality` are intentionally fewer and more deliberate. They are not meant to compete with the primary entry points.

### Layer 3: `auto-*` — Discipline Skills

Auto-triggered coding standards activate based on what you're doing. Writing a database query? `auto-database` loads. Growing a route into a monolith? `auto-testability` and `auto-code-quality` should push extraction and cleanup. These skills encode the patterns the model knows but applies inconsistently, making the quality floor more reliable.

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

During planning, `/summon` assigns a concrete list of auto-* skills to each stream. The execution baseline includes `auto-workflow`, `auto-code-quality`, `auto-errors`, `auto-naming`, and `auto-edge-cases`. Mandatory chat/code/writing/design quality loads apply to the actual work even when an older plan omits them. Additional skills are assigned per-stream based on what that stream touches — the agent assigns and checks them automatically. These are written into the plan's `## Required Skills` section and flow into the status file's `baselineSkills` field, so `/stream` loads exactly the right skills without heuristic guessing.

### The Parallelization Gate

Before `/dominion` spawns parallel streams, it validates that file ownership boundaries don't overlap. If two streams touch the same file, they cannot run in parallel regardless of their declared dependencies. This prevents merge conflicts and race conditions in the codebase.

## Verifying This Ecosystem

Run the installer checks in isolated temporary destinations:

```bash
node --test tools/skills/*.test.cjs
node --test skills/auto-security-quality/upstream/*.test.cjs
node --test skills/auto-design-quality/upstream/tests/*.test.mjs
bash -n install.sh install-codex.sh tools/skills/install-common.sh
```

The security bundle also includes its upstream validators and tests. Skill routing is an instruction contract, so behavioral checks should cover a direct code change, a prose-only edit, an unchanged component reuse, a design change, a security audit, and both final validation modes. Check each delegated worker's actual loads and ensure final review evidence is refreshed after fixes. A skill name in a result checklist alone does not prove execution.

## Retired Skills

The installers back up these old entries outside the discoverable skill directory. Older plan/status assignments map to their replacements automatically.

| Retired | Replacement |
| --- | --- |
| `design`, `ui-ux-pro-max`, `auto-layout` | `auto-design-quality`; distinct layout rules and existing design-set references are retained there |
| `auto-coding`, `auto-sanity` | `auto-code-quality`, with `auto-testability` for structural review |
| Experimental `auto-refactor` | `auto-code-quality` and `auto-testability` |

Naming, comments, testability, accessibility, language, security, and other domain disciplines remain because they provide rules the new general skills do not cover. Reduced-motion, test-count, and localization guidance were reconciled with the new design and coding workflows.
