---
name: stream
description: "Multi-stream plan execution coordinator. Tracks stream progress across sessions, manages dependencies, auto-loads relevant skills, and enforces verification gates. User-invocable via /stream command. Triggers: stream, next stream, stream status, claim stream, continue plan."
---

# /stream — Multi-Stream Plan Executor

Coordinates multi-stream plan execution across sessions. Each session claims one stream, loads the right skills, executes with verification, and marks completion.

## Automatic Quality Routing

Immediately load `auto-chat-quality`, `auto-workflow` and its `references/quality-routing.md`. Load `auto-code-quality` before reviewing or changing code, and `auto-writing-quality` before responses, reports, docs, or UI copy. Route new/changed design to `auto-design-quality`; plain unchanged reuse of established components is exempt. These rules survive compaction, resumed work, empty/legacy skill lists, and final-only overrides.

On Claude, invoke skills through Skill; on Codex, resolve and read installed `SKILL.md` files. Resolve references relative to the skill directory. Every delegated worker must actually load its applicable quality skills in its own context before work, record load evidence, and re-audit if a missed load is discovered. Parent excerpts alone are insufficient. Follow default auto mode: choose routine next steps, run available checks/start local dev tools yourself, and fix supported in-scope issues without extra setup/approval prompts. Honor explicit manual or review-only scope and genuine permission boundaries.

## When to Use

- Any plan with `## Stream` headers (1 or more streams)
- Work that benefits from fresh context between streams
- Streams with dependency relationships or file ownership boundaries

## When NOT to Use

- Plans without `## Stream` headers → not a stream-based plan, use `/summon` no-plan path
- Mid-stream work → this skill is for session start/end, not mid-implementation

## Invocation

```
/stream docs/plans/slug.md     # Explicit plan file
/stream                         # Smart auto-detect
/stream --status                # Show progress dashboard
```

---

## Phase 1: Resolve Plan File and Ensure Status File

### Step 1: Resolve the plan

**Explicit path provided:** Read the file.

**Smart auto-detect (no args):** Follow this cascade:

1. **Check current plan/status companions read-only:** Glob `docs/plans/*.status.json`. A current, unarchived plan is active when any stream is incomplete **or** its final pair needs migration/missing/stale evidence, even if a legacy `final` says `completed`. An explicit continuation/current task context selects its matching companion first; otherwise select a unique active companion. A recorded completed finalization/archive is not reactivated merely because its schema is old. If multiple genuinely active plans remain ambiguous, ask which plan. Assess without writing, then normalize only the selected companion after checking ownership.

2. **Check existing plan files, then git:** If no current active companion was selected (not merely when no status files exist), list `docs/plans/*.md`, including untracked plans just written by summon. Prefer the path named in current task context, then a unique unfinished plan. Git history is a supplemental hint, not a requirement that a new plan be committed. For recent tracked plans, run:
   ```bash
   git log --diff-filter=A --name-only --pretty="" -5 -- "docs/plans/*.md"
   ```
   Use `git diff --name-only HEAD~5 -- "docs/plans/*.md"` only if that ancestor resolves; a new or shallow repository may lack it. Combine existing filesystem candidates and history, deduplicate, and discard deleted/archived plan paths.

3. **Single current plan with 1+ streams:** Read the file, check for `## Stream` headers. If it qualifies → initialize.

4. **Multiple recent plans:** List each with title and date, ask user to pick.

5. **No plans found:** `"No multi-stream plans found. Run /summon to create one."`

**`--status` flag:** Read all `.status.json` files in `docs/plans/` and display a dashboard showing each stream's status, blocked/eligible state, and overall progress.

### Step 2: Ensure status file exists

Once a plan is resolved, immediately check for its companion `.status.json`. If it exists, load it and apply the idempotent migration in `references/status-schema.md`. If not, create it now:

1. Parse all stream headers matching `## Stream N:` or `## Stream N —`
2. For each stream, extract:
   - **Name:** text after the stream number
   - **Dependencies:** from `**Dependencies:**` line
   - **Files owned:** from `**Files owned:**` line
   - **Sub-streams:** from `### NA.` patterns
3. Parse the `## Required Skills` section (if present):
   - Extract **Baseline** skills (apply to all streams)
   - Extract **Per-Stream** skill assignments from the table
   - Combine baseline + per-stream into a `baselineSkills` array for each stream
   - If no `## Required Skills` section exists, set `baselineSkills` to `[]` (triggers conditional loading fallback in Phase 3)
4. Parse the `## Final Validation Mode` section (if present):
   - `Mode: codex` → final validation uses `codex-validation`
   - `Mode: review` → final validation uses classic `review`
   - If absent, default to `review`
5. Write `docs/plans/{slug}.status.json` with all streams set to `pending`, including `baselineSkills` per stream and the selected `finalValidationMode`
6. **Auto-inject two final siblings:** add reserved IDs `final` (Final Validation in review mode; Final Cleanup in codex mode) and `final-security` (Final Security Audit with `auto-security-quality`). Each depends on all implementation IDs, excluding both final IDs. Neither depends on its sibling. Initialize the stable `reviewBaseline` before implementation and preserve it across resumes. Follow `references/status-schema.md` for normalization, snapshots, reports, and the joined finalization barrier; no final worker commits, pushes, cleans up, or hands off independently.
7. Display the dependency graph

See `references/status-schema.md` for the full JSON schema.

---

## Phase 2: Claim a Stream

### Auto-selection

1. Find all streams where `status` is `pending`
2. Filter to streams whose dependencies are ALL `completed`; the two final siblings also require every implementation `settledAt` and all late deltas to be settled
3. From eligible set, pick the **lowest-numbered** stream
4. If no eligible streams:
   - All completed → announce plan completion (Phase 6)
   - Some `in_progress` → **assume another session is actively working on them** (see below)
   - Dependencies unmet → enter **Dependency Wait** (see below)

### In-Progress Streams (Another Session Is Working)

When streams show `in_progress`, **assume another Claude session is actively working on them.** Do NOT attempt to resume or take over. Report the situation and let the user decide.

**Only resume an `in_progress` stream when the user explicitly says** one of:
- "take over stream N"
- "resume stream N"
- "the other session is done/dead/crashed"

### Multiple eligible streams

**If the user's prompt specifies a stream** (e.g. "claim stream 3", "execute stream 5", or the prompt ends with "— claim and execute Stream N"): claim that specific stream immediately. Do NOT list options or ask — just claim and execute.

**Otherwise** (no stream specified): select the lowest-numbered eligible implementation stream automatically. In explicit manual mode, list options if the user wants to choose. Once only the two final siblings remain, coordinate both under Phase 4F without asking for a second session or command.

### Dependency Wait

When the next logical stream has unmet dependencies, choose another eligible stream or wait for active owners to finish. Do not treat user confirmation as evidence that dependencies passed; verify them before continuing.

### Override Verification

When the user says a dependency is done:

1. Read the dependency stream's **Files owned** from the plan
2. Check file existence via Glob
3. Run the project's type checker and relevant tests
4. If all pass → mark dependency as `completed` and proceed
5. If any fail → report failures, do NOT proceed

### Claiming

Once a stream is selected and dependencies are met:

1. Under the schema's exclusive transaction lock (or dominion single writer), re-read status, validate ownership, set `status: "in_progress"`, and record `claimedBy` plus `claimedAt`
2. Announce the claimed stream, dependencies status, and files owned

---

## Phase 3: Load Skills

This happens BEFORE any implementation.

### Baseline skills from status file (preferred path)

If the status file contains a non-empty declared `baselineSkills` array for this stream, load those domain skills plus the always-load set and actual-work quality routing. Do NOT replace explicit domain assignments with keyword matching; do add required quality adapters even for legacy or empty arrays.

1. Load the always-load set (see below)
2. Load every skill listed in the stream's `baselineSkills` array
3. If the stream has `**Legion:** Yes`, also load `auto-legion`
4. Skip the conditional loading section entirely

### Always load

These load for every stream, regardless of `baselineSkills`:

- `auto-workflow` (execution, TDD, verification)
- `auto-chat-quality` (conversation and attention discipline)
- `auto-code-quality` (mandatory in each worker reviewing/changing code)
- `auto-writing-quality` (human-facing prose, copy, reports)
- `auto-errors` (error handling discipline)
- `auto-naming` (naming discipline)
- `auto-edge-cases` (boundary handling)

### Final stream override

If the claimed stream is `final` or `final-security`, retain all applicable quality routing and foundation skills. The review sibling additionally loads `review` in review mode; the security sibling loads `auto-security-quality`. Impeccable still loads for design review or remediation. This override only skips unrelated domain loading; it never disables chat, code, writing, or design quality. Proceed to Phase 4F and coordinate the pair.

### Legion loading

If the stream has a `**Legion:** Yes` annotation in the plan (from `/summon`'s legion gate), also load:
- `auto-legion` (orchestrator discipline, wave management)

### Conditional loading (fallback only)

Use this when the status file has no declared domain skills for the stream (missing/empty legacy `baselineSkills`, or only the automatically merged quality floor).

Analyze the stream section from the plan. Extract all file paths and keywords, then match:

| Pattern | Skills to Load |
|---------|---------------|
| `*.svelte`, `+page.svelte`, `+layout.svelte` | auto-svelte, auto-accessibility, auto-design-quality |
| `*.css`, `*.scss`, Tailwind classes, StyleSheet | auto-design-quality |
| `*.ts`, TypeScript code | auto-typescript |
| Keywords: auth, session, password, encrypt, permission, sensitive, PII | auto-security |
| Keywords: PII, audit, consent, retention, GDPR | auto-compliance |
| `*.py`, Python code | auto-python |
| Keywords: log, tracing, observability, span, instrument | auto-logging |
| Keywords: comment, docstring, documentation, complex algorithm | auto-comments |
| Keywords: async, spawn, mutex, concurrent, shared state, channel | auto-concurrency |
| Keywords: test, spec, assertion, mock, vitest, pytest, proptest | auto-test-quality |
| Keywords: config, env, fallback, default, optional | auto-silent-defaults |
| Keywords: file, connection, pool, listener, cleanup, shutdown | auto-resource-lifecycle |
| Keywords: url, port, timeout, config, env, secret, api key, localhost | auto-hardcoding |
| Keywords: fetch, request, webhook, retry, timeout, external API, delivery | auto-resilience |
| Keywords: endpoint, handler, route, REST, response, pagination, DTO | auto-api-design |
| Keywords: query, SQL, SELECT, INSERT, UPDATE, JOIN, ORM, sqlx, prisma | auto-database |
| Keywords: migration, rename, schema, breaking change, deprecate, column, evolution | auto-evolution |
| Keywords: serialize, deserialize, serde, json, payload, precision, decimal, timestamp | auto-serialization |
| Keywords: cache, caching, TTL, invalidate, stale, memoize, redis cache | auto-caching |
| Keywords: job, queue, worker, task, background, dequeue, enqueue, retry, dead letter | auto-job-queue |
| Keywords: metrics, health check, tracing, span, SLO, prometheus, opentelemetry, monitor | auto-observability |
| Keywords: file, write file, read file, atomic write, temp file, upload, streaming | auto-file-io |
| Keywords: state machine, state, status, transition, workflow, lifecycle, FSM | auto-state-machines |
| Keywords: i18n, locale, translation, plural, ICU, MessageFormat, Fluent, RTL, Intl, l10n | auto-i18n |
| New design or edits to UI, UX, components, layout, color, typography, style, animation, forms, charts, navigation | auto-design-quality, auto-accessibility |
| Human-facing documentation, help, labels, errors, website copy, reports | auto-writing-quality |

### Impeccable integration

For new design or edits, load `auto-design-quality`, preserve project tokens/components and existing `.design/system.md` / `design-system/MASTER.md`, and select applicable bundled commands/references yourself. Plain unchanged reuse does not require a new design direction. Return the design constraints, guidance applied, and actual validation evidence in the worker report. Do not ask the user to invoke commands or invent mandatory search logs.

### Load the skills

Invoke each skill through the runtime adapter above before acting on the implementation tasks. Each worker must load its own applicable quality skills and record `skillLoads` (name, resolved path, load method, role, references). A loaded-name list from the parent is not execution evidence.

Apply `auto-design-quality` before design work; select its relevant bundled command/reference yourself and record useful evidence in the existing report. No separate legacy search-log workflow is required.

Hard rule: if this stream session needs any web research, package/library search, vendor-doc lookup, or source-backed recommendation work, load `auto-web-validation` first before doing that research.

---

## Phase 4: Execute the Stream

**If either final sibling (`"final"` or `"final-security"`) is selected, skip to Phase 4F below and coordinate both.**

**If the stream has `**Legion:** Yes` in the plan, skip to Phase 4L below.**

Hand off to auto-workflow's executing-plans process with these stream-specific additions:

### 4.1 Scope Enforcement

Before starting, explicitly state which files you WILL and will NOT touch. Respect these boundaries throughout the session.

### 4.2 Incremental Verification

Track file edit count. After every **3 file edits** (complex) or **5 file edits** (simple), run the type checker / linter. If it reports errors: **stop and fix** before editing more files.

**Run verification commands directly.** Use `pnpm exec vitest run <files>`, NOT `pnpm test <files> | tail -N`. The `| tail -N` pattern hangs indefinitely if any new test file leaves a handle open (a fresh promise, an unreleased timer, a redis reconnect loop). `tail` buffers stdout until EOF; if node never exits, the pipeline never unblocks. This pattern has historically caused `/stream` sessions (especially headless ones) to hang at 0% CPU after shipping all files. Avoid it.

**Parallel stream awareness:** Before fixing any error, check whether the erroring file is owned by another `in_progress` stream. If so, **skip it** — that stream is responsible.

### 4.3 Sub-stream Sequencing

If the stream has sub-streams (e.g., 2A, 2B):
- Execute in alphabetical order (A before B)
- Run verification between sub-streams
- Update the status file checkpoint after each sub-stream completes

### 4.4 Follow TDD and Execution Standards

All implementation follows auto-workflow's executing-plans process:
- Batch execution (3 tasks default)
- TDD: failing test first, then implement
- Report between batches
- Stop on blockers

### 4.5 Smoke Tests

After implementing each API endpoint or page, run a quick smoke test against the live dev server to verify it works end-to-end. If the dev server is not running, use the project's documented local start command and manage the process yourself; report a genuine environment blocker instead of assigning a routine start command to the user.

---

## Phase 4L: Legion Execution Protocol

This phase executes only for streams annotated with `**Legion:** Yes`. You are the **orchestrator** — follow `auto-legion`.

### 4L.1 Context Gathering

Read ALL files in the stream's scope. Build a mental model of:
- Interfaces and types the stream's code must satisfy
- Existing code patterns in the project
- Dependencies between tasks in the stream

This is the only time you read files. After this, craft prompts from what you learned.

### 4L.2 Decompose Into Waves

Follow `auto-legion`'s decomposition algorithm. Using the stream task list and the suggested wave structure, produce the wave plan:

1. Parse the plan's suggested waves (e.g., `Wave T: 3 agents → Wave I: 3 agents → Wave D: 2 agents`)
2. Refine based on what you learned in 5L.1 — the plan's suggestion is a starting point, not gospel
3. Assign specific files and tasks to each agent in each wave
4. Identify interfaces/types to paste into agent prompts

Update the status file with the legion wave structure.

### 4L.3 Execute Waves

For each wave, in order (T → I → D → R):

**Dispatch:** Craft focused prompts and dispatch ALL agents in the wave simultaneously using Claude Agent with `run_in_background: true` or Codex collaboration agents. Respect the runtime's available concurrency capacity. All concurrent dispatch calls MUST be in a single message. Include a mandatory first-step load of `auto-chat-quality`, `auto-code-quality` for code/review, `auto-writing-quality` for prose, and `auto-design-quality` for design changes/review. Workers resolve/read the actual skill files and references, then return load evidence with their results.

**Wait:** Agents complete in background. You are notified when each finishes.

**Collect:** Read each agent's output. Note successes, failures, and any reported issues.

**Verify:** Run project-wide verification:
```bash
npx tsc --noEmit                    # Type check
npx vitest run <stream file paths>  # Tests for this stream
```

**For Wave T (tests):** Verification means tests exist and FAIL (red phase). Type errors are blockers; test failures are expected.

**For Wave I (implementation):** Verification means tests PASS (green phase) and type check is clean.

**For Wave D (dependents):** Full verification — types, tests, and build.

**Fix:** If verification fails:
- Read the failing files
- If 1-2 small issues: fix them directly (orchestrator handles it)
- If an agent's entire output is wrong: re-dispatch that single agent with error context (max 2 retries)
- If systemic failure: fall back to solo execution for remaining tasks

Update the status file after each wave completes.

### 4L.4 Assembly Check

After all waves complete, run the full verification gate (same as Phase 5.1). The orchestrator reviews all agent-produced code as a whole:

- Do modules integrate correctly?
- Are imports consistent?
- Any naming conflicts between agent outputs?

Fix any integration issues directly — these are typically small (import paths, naming alignment).

### 4L.5 Solo Fallback

If legion execution cannot complete (2 consecutive wave failures, unresolvable agent conflicts):

1. Update status: `"legion": { "enabled": false, "fallbackReason": "..." }`
2. Load remaining tasks into context
3. Execute remaining tasks directly using standard Phase 4 process
4. This is not a failure — some tasks resist decomposition

---

## Phase 4F: Joined Final Validation Protocol

This phase coordinates both `final` and `final-security`. Read and follow the canonical protocol in `references/status-schema.md` (Two Final Sibling Streams through Concurrency). No generic Phase 5 completion shortcut may bypass this gate.

### 4F.1 Prepare and Freeze

Wait until every implementation stream and all its verification/remediation/late deltas are settled. One session claims final coordination, preserves the stable baseline, and runs the complete project verification suite: type/lint, tests, build, and applicable smoke checks. The existing zero-error/warning standard applies; fix supported issues before freezing. Use direct commands and capture results.

Capture the immutable snapshot including committed changes since baseline, staged/unstaged changes, untracked source, and relevant context. Stop source writers. Both readers receive identical baseline, snapshot ID/manifest, scope, skill manifest, and exclusive artifact roots. Record their report paths when returned. Do not run source-mutating checks against the frozen checkout.

### 4F.2 Dispatch the Siblings Concurrently

- **`final`:** Load the quality skills in the worker's own context. In `review` mode use `review` for the classic findings pass. In `codex` mode perform the existing broad Claude Cleanup Review for correctness, readability, maintainability, warnings, and testability; do not run `codex-validation` here. Report proposed fixes, never apply them while the sibling is reading.
- **`final-security`:** Load `auto-chat-quality`, `auto-code-quality`, `auto-writing-quality`, and `auto-security-quality` in the worker's own context. Follow the complete security audit workflow on the full captured change set and affected trust boundaries, reading surrounding code as needed. Preserve its independent fresh-agent validation, structured reports/validators, output isolation, and OS sandbox requirement for target execution. Without that sandbox, perform source review and record the execution limitation; do not treat ordinary worktree checks as sandboxed security validation. Record findings, evidence, coverage, and tool limitations.

Both workers are read-only against source and write only their exclusive report/artifact directories. The security audit uses its upstream-permitted external run directory by default and returns all artifact paths; do not force its six-phase output into a tracked target directory. Neither changes shared status or commits/pushes/deletes anything. Reserve capacity for mandated independent design/security reviewers, or return their bounded assignments to the coordinator and release the worker slot for phased scheduling. Never fill all available slots with workers waiting to spawn children; preserve reviewer independence when sequential scheduling is needed. Return actual skill load evidence. If a required load was missed, load it and repeat the affected audit before accepting the report. With no concurrency support, run the two independent reads sequentially under the same barrier automatically and report the limitation.

### 4F.3 Join and Remediate

Wait for both reports and validate their snapshot fingerprints. Present consolidated findings and apply authorized fixes through one owner after the readers finish. Classic review issues/suggestions and confirmed security findings must be resolved; nitpicks retain their existing optional status. Use code/writing/design quality routing for all fixes. Re-run affected checks, capture a new snapshot, and obtain fresh evidence from both reviewers covering fixes and regressions. Do not mark an incomplete audit as passing.

Only the coordinator marks both final streams completed and `finalGate.phase: passed` when both pass on the current snapshot and project verification passes. Source drift invalidates both reports. A prior pass on old bytes is insufficient.

### 4F.4 Finalize Once

In `review` mode, the finalization owner reviews git status/diff, stages only intended files, writes a conventional commit, and pushes under the existing workflow authorization. Preserve unrelated user changes. Only after successful authorized commit/push does it delete the plan and status files; keep review reports. Never repeat an already recorded finalization after resume.

In `codex` mode, do not commit, push, or delete plan/status. Preserve both reports, the working tree, and baseline, then perform the existing Codex `/verify` handoff. If already in Codex and validation is authorized, continue it directly; otherwise state the exact handoff needed. Do not claim Codex validation happened when it did not.

### 4F.5 Announce Completion

Report both review and security outcomes, snapshot/check evidence, and any remaining coverage limits. For `review`, include commit/push/cleanup results. For `codex`, report Final Cleanup and Final Security Audit completed, preserved artifacts, and the actual Codex validation/handoff state.

---

## Phase 5: Complete the Stream

When all tasks in the stream are implemented:

### 5.1 Verification Gate (non-negotiable)

Run all of these **project-wide**. **Use direct commands.** Do NOT pipe `pnpm test` output through `| tail` / `| head` / `| grep` when you need to see the full picture — the pipe hangs if node doesn't exit cleanly. Prefer `pnpm exec vitest run` over `pnpm test`.

| Check | Scope | Must |
|-------|-------|------|
| Type check / lint | Entire project | Exit 0 — OR all remaining errors belong to other active streams (see 6.2) |
| Tests | Entire project | All pass — OR all failures are in files owned by other active streams |
| Build | Entire project | Exit 0 (no exceptions) |
| Smoke tests | Stream's endpoints | All return expected responses |

### 5.2 Fix Pre-existing Errors (Parallel-Aware)

If verification surfaces errors **outside your stream's files**:

1. Read the status file — identify all `in_progress` streams (other than yours)
2. Read each active stream's **Files owned** from the plan
3. Classify each error:
   - **Owned by another active stream** → **SKIP.** That stream will fix its own errors.
   - **Not owned by any active stream** → **Fix it.**

If all remaining errors belong to other active streams, your stream may pass verification with a note listing the skipped errors.

### 5.3 Self-Audit Pass (mandatory)

Before marking the stream complete, walk the files you touched and re-check each declared skill's rules against your diff. You're the cheapest place to catch skill violations — a fresh verification agent (under `/dominion`) or a later reviewer will catch them otherwise, more expensively.

For each declared and automatically routed skill (including the quality floor):
- Re-read the skill's enforceable rules
- Walk your diff: does anything violate those rules?
- If a violation is obvious and safe to fix, fix it now
- If uncertain, note it in the `deferrals` field of your completion return

This is Layer 2 of the four-layer enforcement model (see `dominion` SKILL: Layered Skill Enforcement). Primary shapes design, self-audit catches rough edges, verification catches what self-audit missed, remediation fixes the rest. Skipping self-audit pushes more work onto the later layers.

### 5.4 Mark Complete

**Quality gate:** Confirm every worker actually loaded applicable code/writing/design quality skills and re-audited any work performed before a missing load was corrected. Include `skillLoads` and concrete verification evidence; a skill name in the plan is not proof it ran.

Update the status file through the coordinated writer: set `status: "completed"`, record `completedAt` timestamp, and set `settledAt` only after required verification/remediation finishes (manual solo completion includes its completed checks; dominion records settle after independent workers), and write a `verification` sub-object summarizing gate results. Include a `deferrals` array if any items are out-of-scope-but-noted for a later stream or the human to decide.

```json
"verification": {
  "gates": { "check": "pass", "lint": "pass", "tests": "42/42" },
  "deferrals": [
    { "item": "Sentry breadcrumbs on rate-limit fallback",
      "reason": "Sentry not wired at project level yet",
      "owner_suggested": "Foundation follow-up" }
  ]
}
```

Under `/dominion`, the verification and remediation agents read this sub-object — deferrals feed Input 1 of the remediation wave. Under manual `/stream`, the deferrals are notes for the human running the next session.

### 5.5 Announce and Prompt Next Session

Report verification results, remaining streams and their status, overall progress. In explicit manual mode, prompt for the next session. In auto mode, continue the next eligible stream internally or through Dominion; do not require a new command.

### 5.6 Failed Verification

If any check fails: report the failure, do NOT mark as completed, keep `status: "in_progress"`.

---

## Phase 6: Plan Complete

When ALL streams (including `final` and `final-security`) have `status: "completed"` and `finalGate.phase` is `passed` for the current snapshot:

If reached via Final Validation, the 4F.5 announcement is the primary output.

If final evidence is missing or stale, automatically resume Phase 4F; do not announce completion or hand routine validation work back to the user.

---

## Edge Cases

### In-progress stream from another session
Assume another session is working on it. Only resume on explicit user request.

### Parallel-eligible streams
If the prompt specifies which stream to claim, claim it immediately. Otherwise, select the lowest-numbered eligible stream. Use the schema's locking/single-writer protocol to prevent double-claiming.

### Single-stream plan
Full Phase 1-6 workflow applies. Phase 2 auto-selects the single stream.

### Plan file changed after status file created
Compare stream headers against status file. If new streams were added within the authorized plan, normalize the status file while preserving existing progress and invalidate stale final evidence. Escalate only genuinely conflicting scope/ownership.

---

## Rules

1. **NEVER** start implementing before loading relevant skills
2. **NEVER** skip incremental verification
3. **NEVER** mark a stream complete without fresh verification evidence
4. **NEVER** touch files owned by other `in_progress` streams
5. **NEVER** skip either final sibling or their joined completion gate
6. **ALWAYS** read the status file before claiming
7. **ALWAYS** verify dependencies before starting blocked streams
8. **ALWAYS** continue authorized work in auto mode; offer a next-session prompt only for explicit manual execution
9. **ALWAYS** load `auto-web-validation` before any web search, package search, or vendor/library research in `/stream`
10. The status file is the **single source of truth**
11. The plan file is **read-only**
12. Only the finalization owner deletes plan/status in review mode after both siblings pass and the authorized commit/push succeeds; codex mode preserves them
13. Require actual quality skill loads and checks on the resulting work, including design guidance/evidence when applicable

## Rationalization Prevention

| You're thinking... | Reality |
|---|---|
| "The parent loaded Ponytail/Humanizer, so the worker does not need them" | Every worker must load `auto-code-quality` before code work/review and `auto-writing-quality` before prose, then return evidence. |
| "The old plan has no security stream, so final review is enough" | Normalize the status and run both final siblings automatically on the same current snapshot. |
| "The security report passed before my fix, so I can commit now" | The fix changed the reviewed bytes. Refresh checks and both reports before finalization. |
