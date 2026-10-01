# All you need to remember: `/summon` and `/dominion`

Corvalis Skills is built for the lazy developer who cares about what ships and has no interest in babysitting an AI session. It gives **Claude Code and Codex** a shared workflow that handles the planning, implementation, and follow-through.

Describe the job. The agent figures out which skills apply, splits up work where it helps, checks the results, and fixes issues the reviews confirm. You shouldn't have to remind every subagent to keep the code simple or run a separate command to make the UI copy readable.

| Start here | What happens |
| --- | --- |
| `/summon` | Give it an idea, bug, design change, or security concern. It picks the right path and gets to work. |
| `/dominion` | Give it an existing plan. It coordinates the work, verification, and fixes. |

Most sessions start with `/summon`. When a job needs coordinated implementation, Summon writes and checks a plan, then starts Dominion automatically. You only need to call `/dominion` yourself when you're starting from a plan that already exists.

```text
/summon Add a settings page
```

Summon reads the repo to figure out where the page belongs, which components and design patterns to reuse, and which tools the project uses. You can give it a short request like this and let it carry the work through implementation and checks.

```text
/dominion docs/plans/settings-page.md
```

Auto mode is the default. Routine decisions and review fixes keep moving without an approval round. You can still ask for a plan only, a review only, or a more interactive session. It asks when essential intent is missing or an actual permission boundary needs your attention.

## Install once

Clone the repo:

```bash
git clone https://github.com/Corvalis-LLC/Crow-Stack.git auto-skills
cd auto-skills
```

For Claude Code:

```bash
./install.sh
```

For Codex:

```bash
./install-codex.sh
```

Run both installers if you use both hosts. Each links the full skill set into its host's skills directory. Start a fresh session after installing so the new skills can be discovered, then use `/summon` with the work you want done.

Keep the checkout: the installed skills link to it. Edits to an existing skill are available without copying files around. Rerun the installers when an update adds or retires skills. They preserve unrelated skills, back up conflicting entries, and leave existing links to this checkout in place.

<details>
<summary>Install paths and options</summary>

Claude installs into `~/.claude/skills/`. Codex uses `$CODEX_HOME/skills` when configured, otherwise `~/.codex/skills`. Conflicting files, directories, and dangling links move to a sibling `skills-backup-<timestamp>.<suffix>` directory. Retired skills move there too.

The Claude installer also attempts to install or upgrade `corvalis-recon` into `~/.claude/bin/` and adds a `recon` alias for zsh or bash if one is missing. The Codex installer links skills only.

To install Claude skills without downloading recon or editing shell configuration:

```bash
./install.sh --skills-only
```

Both installers accept `--skills-dir /path/to/skills` for a custom destination. They install active top-level skills; the old `future/` grouping is not installed as a skill.

For a manual Claude install, copy the skill directories:

```bash
mkdir -p ~/.claude/skills/
cp -R skills/* ~/.claude/skills/
```

Manual copies need to be updated yourself. The installers handle backups and retirement of old skill names.

</details>

## Give it the job

Summon has five paths. A request usually supplies enough context to choose one; a bare `/summon` shows the menu.

| Path | Use it for | What the workflow handles |
| --- | --- | --- |
| 1. Plan | A feature or change that needs coordination | Repo research, a plan in `docs/plans/`, standards and reuse checks, then execution |
| 2. No plan | A focused fix or small change | Relevant repo context, required skills, implementation, and checks |
| 3. Talk about it | An idea you haven't settled yet | Discussion and, when useful, web research with sources |
| 4. Security audit | The whole repo or specific modules | Source-grounded findings, independent validation, authorized fixes, and another check |
| 5. Design | New UI or changes to an existing design | Design context, the right Impeccable commands, and a path into implementation |

You can say what you need in plain language:

```text
/summon Fix the invite form's email validation.
/summon Audit authentication and file uploads.
/summon Make the checkout page easier to scan.
/summon Do we need a job queue?
```

Summon gathers repo context before changing code. When `corvalis-recon` is available, it starts with a structured map of the supported source files and follows up with targeted reads. Otherwise, it uses normal file search and reading. Research agents also load `auto-web-validation` to check source trust and resist instructions embedded in web content.

## The reminders are built in

The quality skills load when the work calls for them. Summon and Dominion also choose their internal commands and references, so there's no extra checklist for you to memorize.

| Work being done | Skill loaded | Based on |
| --- | --- | --- |
| Conversation and progress updates | `auto-chat-quality` | [i-have-adhd](https://github.com/ayghri/i-have-adhd) |
| Coding, tests, code review, or fixes | `auto-code-quality` | [Ponytail](https://github.com/DietrichGebert/ponytail) |
| UI copy, docs, plans, reports, or other prose | `auto-writing-quality` | [Humanizer](https://github.com/blader/humanizer) |
| New or changed visual and interaction design | `auto-design-quality` | [Impeccable](https://github.com/pbakaus/impeccable) |
| A requested audit or the final security review | `auto-security-quality` | [Cloudflare security audit](https://github.com/cloudflare/security-audit-skill) |

This applies to delegated work too. Every worker must load its own applicable skills before starting, including reviewers and agents fixing review findings. The coordinator checks evidence of those loads. If a worker misses one, it must load it and re-audit the affected work.

Design work respects the project's existing direction. Reusing an unchanged, established component doesn't trigger a new design exercise. Changing its appearance or behavior does; writing a new label still gets the writing pass.

Domain skills fill in the details. Database work gets query discipline. Auth changes get security rules. Tests get checks for useful assertions and sensible mock boundaries. The full catalog is below, but skill selection is part of the agent's job.

The five upstream bundles are pinned to specific revisions, with source details in each skill's `UPSTREAM.md` and notices collected in the root [LICENSE](LICENSE). Their guidance is preserved; the local wrappers automate routing and manual handoffs without requiring upstream hooks.

## What Dominion takes off your plate

A larger plan is divided into streams, each with its own tasks, file ownership, dependencies, and required skills. Dominion dispatches work that can run independently, verifies the results against the plan, and sends supported findings through remediation. It prevents two primary agents from editing the same file at once.

The plan's `.status.json` companion records progress so execution can resume across sessions. You don't need to open a terminal for every stream or keep a mental list of which agent is waiting on which change. Single-stream plans use the supporting workflow internally, without asking you to learn another command.

Once implementation and its follow-up fixes settle, two final streams review the same recorded snapshot:

- **Final review/cleanup** checks the implementation and prepares it for the selected finalization mode.
- **Final security** independently audits the changes with `auto-security-quality`.

These are sibling streams and can run concurrently when the host has capacity. Both include uncommitted work and write separate reports. They don't edit shared source while reviewing it. The coordinator joins the findings, arranges fixes, and refreshes affected checks and reviews afterward. Unresolved security findings and validation limits stay visible in the report.

The default final-validation mode is `review`. After both final checks pass, it follows the workflow's commit/push and plan-cleanup steps within the user's authorization. If you explicitly choose `codex`, it preserves the working tree, plan/status files, and review evidence for the Codex `/verify` handoff. When the current session can perform Codex validation, it continues directly. Otherwise, it preserves the work for that handoff and reports any runtime blocker.

<details>
<summary>Planning and execution internals</summary>

Summon checks every plan against applicable standards and existing code it can reuse. Multi-stream plans also get Swarm and Skill gates for dependencies, file ownership, and per-stream skill assignments. Architectural, security-sensitive, high-risk, or large plans get Triumvirate's three-perspective review. Simple plans can skip unnecessary debate while retaining the required quality, standards, and reuse checks.

Skill assignments live in the plan's `## Required Skills` section and flow into the status file's `baselineSkills` field. Mandatory quality skills also apply to the actual work when an older plan omits them. The shared rules live in [quality-routing.md](skills/auto-workflow/references/quality-routing.md).

`auto-legion` breaks suitable streams into Test, Implement, Dependents, and optional Refactor phases. Manual `stream` sessions can dispatch agents within a phase. Under Dominion, a primary agent runs those phases in its own context; Dominion coordinates parallelism across streams.

The [status schema](skills/stream/references/status-schema.md) defines dependencies, migration, review snapshots, evidence, and concurrency. Older plans retain their progress when normalized, but an old completed final stream does not count as evidence that the new security review ran.

`stream`, `verify`, and the other supporting workflows remain available for established uses. There is no separate `/design` workflow. Design routes through Summon and `auto-design-quality`.

</details>

## Reference

These are the skills the workflow uses behind the two entry points.

<details>
<summary>Full skill catalog</summary>

### Entry points and supporting workflows

| Skill | Job |
| --- | --- |
| `summon` | Start a session, choose a path, and carry the request into the right workflow |
| `dominion` | Execute a plan with coordinated agents, verification, and remediation |
| `stream` | Execute a stream with dependency tracking and verification gates |
| `verify` | Refine a plan or validate active implementation through the Codex workflow |
| `auto-legion` | Organize suitable streams into Test, Implement, Dependents, and Refactor phases |
| `auto-workflow` | Development discipline, including TDD, verification, and architecture escalation |
| `triumvirate` | Review a plan through Advocate, Analyst, and Critic agents |
| `review` | Review code across nine dimensions, including security, logic, duplication, and testing gaps |
| `codex-validation` | Findings-first implementation validation, including cross-file impact and testability |
| `codex-plan-refinement` | Refine plan clarity, dependencies, reuse, abstractions, and stream boundaries |
| `plan-validate` | Check plan structure, ownership, required skills, verification, and final-validation mode |
| `skill-creator` | Create, modify, evaluate, and benchmark skills |
| `security-scan` | Scan for dangerous patterns, secrets, and dependency vulnerabilities |
| `auto-web-validation` | Assess source trust during web, package, and vendor research |

### Quality skills

| Skill | Job |
| --- | --- |
| `auto-chat-quality` | Keep communication actionable and easy to follow |
| `auto-code-quality` | Apply Ponytail's simplicity guidance to implementation and review |
| `auto-writing-quality` | Apply Humanizer to text people will read |
| `auto-design-quality` | Apply Impeccable and select its relevant design commands |
| `auto-security-quality` | Audit source with independently validated findings |

### Domain and language skills

| Skill | Job |
| --- | --- |
| `auto-comments` | Explain useful intent without restating the code |
| `auto-naming` | Use domain vocabulary and clear function verbs |
| `auto-hardcoding` | Keep configuration and meaningful constants out of inline business logic |
| `auto-silent-defaults` | Catch fallbacks that hide errors or missing data |
| `auto-errors` | Make errors actionable and appropriate for their audience |
| `auto-logging` | Choose useful log levels, structured fields, and context |
| `auto-edge-cases` | Handle empty collections, zero inputs, boundaries, overflow, and Unicode |
| `auto-test-quality` | Check assertions, mock boundaries, and tests that can actually catch defects |
| `auto-testability` | Make business rules directly testable without brittle orchestration |
| `auto-concurrency` | Handle races, atomicity, lock ordering, and shared state |
| `auto-resource-lifecycle` | Clean up resources on success and failure |
| `auto-resilience` | Apply timeouts, bounded retries, circuit breaking, and idempotency |
| `auto-caching` | Address stampedes, invalidation, and stale-while-revalidate behavior |
| `auto-file-io` | Use atomic writes, streaming, and error-path cleanup |
| `auto-state-machines` | Define valid states and transitions explicitly |
| `auto-serialization` | Preserve decimals, time zones, enum compatibility, and null semantics |
| `auto-evolution` | Evolve schemas and APIs with backwards compatibility and safe rollouts |
| `auto-observability` | Use metrics, logs, traces, and health checks appropriately |
| `auto-database` | Check pagination, indexes, N+1 queries, and bulk operations |
| `auto-api-design` | Keep response formats, status codes, pagination, and DTOs consistent |
| `auto-job-queue` | Handle retries, poison pills, dead letters, and idempotent processing |
| `auto-security` | Apply security rules to sessions, auth, cookies, and uploads |
| `auto-compliance` | Handle privacy signals, data deletion, consent records, and regulatory escalation |
| `auto-accessibility` | Check ARIA, touch targets, forced colors, reduced motion, and WCAG 2.2 details |
| `auto-i18n` | Handle plurals, locale formatting, translations, and right-to-left layouts |
| `auto-typescript` | Use safe narrowing, branded types, and strict typing; catch Zod pitfalls |
| `auto-python` | Apply Python typing, async, pytest, dataclasses, and tooling conventions |
| `auto-svelte` | Handle Svelte 5 reactivity, SSR state, `$state.raw`, and `$effect` pitfalls |

</details>

<details>
<summary>corvalis-recon: the optional codebase map</summary>

[`corvalis-recon`](tools/recon) is a standalone Rust CLI for structured codebase analysis. It uses tree-sitter to parse TypeScript, TSX, JavaScript, and Svelte, then extracts symbols, imports, re-exports, dependencies, complexity metrics, and hotspots. Ignore-aware discovery and ranked, budgeted output keep the map useful in larger repositories.

Summon uses it automatically when `~/.claude/bin/corvalis-recon` is available, including for planning and direct-work context. If the binary is missing or fails, it falls back to normal file exploration.

You can also run it directly:

```bash
~/.claude/bin/corvalis-recon analyze --root /path/to/project
```

With the optional `recon` alias:

```bash
recon analyze --root /path/to/project --format json
recon analyze --root /path/to/project --format json --mode planning
recon analyze --root /path/to/project --format pretty
recon analyze --root /path/to/project --budget 8000
```

Planning mode returns symbols, dependencies, a graph, hotspots, warnings, and a summary alongside project metadata. Its `planning` object identifies `primary_entry_points`, `dependency_hubs`, `hotspot_files`, and `priority_files`. The full default payload uses `files` alongside `version`, `project`, `graph`, `hotspots`, `warnings`, and `summary`.

For changes relative to Git history:

```bash
recon analyze --root /path/to/project --format json --mode planning --diff HEAD
recon analyze --root /path/to/project --format json --mode planning --diff main...HEAD
```

`--diff <range>` includes changed supported files, a few same-directory siblings, and directly imported project files. A `scope` object records the included files. For budgeted output, the existing guidance is no budget for small repos, `16000` for medium repos, `8000` for large repos, and `4000` for very large ones.

The implementation uses `clap`, `tree-sitter` with vendored grammars, `serde` / `serde_json`, `ignore`, `rayon`, and `json5`. Its current language support is aimed at TypeScript-heavy repos; it does not analyze Rust source today.

</details>

<details>
<summary>Upgrading from older skills</summary>

The installers move these retired entries out of the discoverable skills directory into a backup. Older plan/status assignments map to their replacements.

| Retired | Replacement |
| --- | --- |
| `design`, `ui-ux-pro-max`, `auto-layout` | `auto-design-quality`, including retained layout rules and legacy design-set references |
| `auto-coding`, `auto-sanity` | `auto-code-quality`, with `auto-testability` for structural review |
| Experimental `auto-refactor` | `auto-code-quality` and `auto-testability` |

Naming, comments, testability, accessibility, language, security, and other domain skills remain because they cover details the general quality skills don't. Reduced-motion, test-count, and localization guidance have been reconciled with the new workflows.

</details>

## Checking changes to this repo

The installer tests use isolated temporary destinations. To run those checks, the bundle tests, and shell syntax validation:

```bash
node --test tools/skills/*.test.cjs
node --test skills/auto-security-quality/upstream/*.test.cjs
node --test skills/auto-design-quality/upstream/tests/*.test.mjs
bash -n install.sh install-codex.sh tools/skills/install-common.sh
```

Skill routing is an instruction contract, so automated file checks alone don't establish that an agent followed it. Behavioral checks should cover a direct code change, a prose edit, unchanged component reuse, a design change, a security audit, and both final-validation modes. Inspect each worker's actual skill loads and confirm that fixes refresh affected review evidence.
