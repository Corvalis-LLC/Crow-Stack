---
name: summon
description: "Session bootstrap for every new conversation. Offers three paths: plan, no-plan, or talk-it-out. Planning path writes a plan to docs/plans/, validates against quality standards, optionally runs triumvirate, then recommends clearing context and spinning up implementation sessions. User-invocable via /summon command. No prompts or props required."
---

# Summon — Session Bootstrap

`/summon` is the **entry point for every new conversation** (the other entry point is `/design` for UI-focused work).

## Phase 1: Foundation

Load immediately — no analysis needed:

1. `auto-workflow`
2. `auto-coding`

Then ask the user which path they want:

> **What would you like to do?**
> 1. **Plan** — brainstorm, write a plan, validate it against standards
> 2. **No plan** — tell me what to do and I'll load the right skills and get to work
> 3. **Talk about it** — not sure yet, let's discuss and figure out the right approach

---

## Path A: Planning

### A1. Brainstorm & Write the Plan

Follow the brainstorming and planning workflows from `auto-workflow` and collaborate with the user:

1. Ask what they're working on (if not already stated)
2. Brainstorm the approach collaboratively
3. Produce a plan using the writing-plans workflow
4. **Write the plan as a markdown file to `docs/plans/YYYY-MM-DD-<slug>.md`** — this is NON-NEGOTIABLE. The plan MUST be written to this path before proceeding. Do NOT skip this step, do NOT only show the plan in chat. Use the Write tool to save the file.
5. Get user approval on the plan

**Keep the plan at the right level of abstraction.** This phase is about *what* to build and *why*. Standards compliance comes next.

**Plan file location is mandatory.** If `docs/plans/` doesn't exist, create it. Every plan goes to `docs/plans/YYYY-MM-DD-<slug>.md` — no exceptions.

### A2. Standards Gate (NON-NEGOTIABLE)

**This step MUST happen after every plan is written. Never skip it.** Even if the plan seems simple or the user is eager to start implementing. The standards gate catches gaps that cost hours to fix later.

After the plan is written to `docs/plans/` and approved, load the **relevant** auto-* skills and check the plan against them.

#### Determine Relevant Skills

Analyze the plan and load only the auto-* skills it touches:

| Skill | Load When Plan Involves... |
|-------|---------------------------|
| `auto-typescript` | Any TypeScript code (almost always) |
| `auto-layout` | Any UI work — components, pages, layouts, CSS, styling |
| `auto-svelte` | Components, pages, layouts, reactivity |
| `auto-security` | Auth, user input, sessions, permissions |
| `auto-compliance` | PII, audit logging, data retention |
| `auto-accessibility` | UI components, forms, interactive elements |
| `auto-errors` | Error handling, Result types, API responses, validation |
| `auto-naming` | New types, functions, modules, or domain concepts |
| `auto-comments` | New modules, complex logic, architectural decisions |
| `auto-logging` | Observability, tracing, log output, background jobs |
| `auto-edge-cases` | Functions accepting user input, collections, pagination |
| `auto-resource-lifecycle` | Files, DB connections, HTTP clients, event listeners, spawned tasks |
| `auto-concurrency` | Async code, shared state, mutexes, spawned tasks, queues |
| `auto-test-quality` | Writing or reviewing tests |
| `auto-silent-defaults` | Config loading, fallbacks, missing data handling |
| `auto-hardcoding` | Service URLs, timeouts, pool sizes, magic numbers, credentials |
| `auto-resilience` | HTTP calls, external APIs, webhooks, partial failure scenarios |
| `auto-api-design` | REST endpoints, response formats, pagination, error responses |
| `auto-database` | SQL queries, ORM usage, pagination, bulk operations, indexes |
| `auto-evolution` | Schema changes, API contract changes, config renames, env var migration |
| `auto-serialization` | Serializable types, API payloads, job queue messages, JSON columns |
| `auto-caching` | Caching API calls, DB queries, computed results, external responses |
| `auto-job-queue` | Job queues, background workers, task processors, message consumers |
| `auto-observability` | Monitoring, health endpoints, metrics, tracing spans, alerting |
| `auto-file-io` | File reading/writing, uploads, temp files, filesystem paths |
| `auto-state-machines` | Workflows, order lifecycles, job statuses, entities with distinct phases |
| `auto-i18n` | Multi-locale support, translated strings, pluralization, number/date formatting, RTL |

**Minimum load:** `auto-typescript` applies to virtually every task. `auto-coding`, `auto-errors`, `auto-naming`, and `auto-edge-cases` apply to most implementation work.

#### Amend the Plan

Review the plan against each loaded skill's standards and call out gaps:

- "This plan adds a form but doesn't mention validation or accessibility."
- "This plan creates a new endpoint but doesn't account for rate limiting."
- "This plan modifies user data but doesn't include audit logging."
- "This plan adds a database column but doesn't address migration safety."

Present amendments as a short list. If the plan already satisfies all relevant standards, say so — don't invent issues.

Update the plan file in `docs/plans/` with the amendments and get user sign-off.

### A3. Optional Gates

After the standards gate, present the user with optional refinement gates. These run **in order** when selected — the order matters because each gate builds on the previous one's output.

> **Optional refinement gates (combine numbers, e.g. "12", "123", "3"):**
> 1. **Swarm Gate** — optimize dependencies for parallel execution, annotate legion viability
> 2. **Skill Gate** — explicitly assign auto-* skills per stream
> 3. **Triumvirate** — adversarial plan review (three subagents debate the plan)
> 0. **Skip all** — proceed straight to handoff
>
> Recommended: `123` for large multi-stream plans, `12` for medium plans, `0` for simple ones.

**Execution order is always: Swarm → Skill → Triumvirate** (regardless of which subset the user picks). Each gate reads the plan as modified by the previous gate.

---

#### Swarm Gate

Two jobs: (1) **flatten the dependency chain** so streams run in parallel wherever possible, and (2) **annotate legion viability** per stream.

**Step 1: Dependency Optimization (THE HARD PART)**

Plans naturally drift toward sequential chains: 1 → 2 → 3 → 4 → 5 → 6 → 7. This is almost always over-constrained. A stream should depend on another **only** if it reads or mutates a specific artifact that the other stream produces. "It feels like it comes after" is not a dependency.

**The optimization algorithm:**

**1a. Build the true dependency graph.**

For each stream, identify what it **actually needs from other streams** — not what the author assumed. A dependency is real only when:

| Real Dependency | NOT a Real Dependency |
|----------------|----------------------|
| Stream 3 imports a type that Stream 1 creates | Stream 3 "conceptually builds on" Stream 1 |
| Stream 4 adds a column that Stream 2's migration creates | Stream 4 is "the next logical step" after Stream 2 |
| Stream 5 calls an API endpoint that Stream 3 implements | Stream 5 was written after Stream 3 in the plan |
| Stream 6 renders data from a service Stream 4 builds | Stream 6 is in the same feature area as Stream 4 |

For each declared dependency, ask: **"What specific file, type, table, or endpoint does this stream need that doesn't exist yet?"** If you can't name it, the dependency is false.

**1b. Apply common optimizations.**

| Pattern | Before | After | Why It Works |
|---------|--------|-------|-------------|
| **False chain** | 1 → 2 → 3 → 4 | 1 → {2, 3, 4} | Streams 2-4 only actually need Stream 1's foundation |
| **Diamond collapse** | 1 → 2 → 4, 1 → 3 → 4 | 1 → {2, 3} → 4 | Streams 2 and 3 are independent of each other |
| **Interface-first unlock** | 1 → 2(types+impl) → 3(uses impl) | 1 → 2a(types) → {2b(impl), 3(uses types)} | Stream 3 only needs the type definitions, not the full implementation |
| **Migration batching** | 1(migration) → 2(migration) → 3(code) | 1(both migrations) → {2, 3}(code) | Combine sequential migrations into one stream to unblock others |
| **Stub unlocking** | 1(service) → 2(frontend uses service) | {1(service), 2(frontend with stub)} | Frontend can build against a type stub while service is implemented |

**1c. Calculate the critical path.**

After optimization, compute the longest dependency chain. This is the minimum number of sequential phases. Report the improvement:

```
Dependency optimization:
  Before: 7 sequential streams (critical path = 7)
  After:  3 phases (critical path = 3)
    Phase 1: Stream 1 (Foundation + migrations)
    Phase 2: Streams 2, 3, 4 (parallel — independent feature slices)
    Phase 3: Streams 5, 6, 7 (parallel — integration + frontend)
  
  Improvement: 7 sequential → 3 phases (57% reduction in wall-clock stream time)
```

**1d. Restructure the plan if needed.**

If optimization changes dependencies, update the stream headers in the plan file:
- Move the `**Dependencies:**` lines to reflect true dependencies
- If streams were split (e.g., extracting types into a separate sub-stream), add the new stream header
- If migrations were combined, merge those stream sections
- Present changes to the user for approval before writing

**Never silently change stream structure.** Show the before/after dependency graph and explain each change.

**Step 2: File Ownership Matrix**

After dependency optimization, build the file-ownership matrix:

1. For each stream, list all files it will create or modify
2. Identify **shared files** — files touched by multiple streams
3. Classify shared files:
   - **Additive-only**: barrel exports (`index.ts`), route registrations, migration directories, Zod schema barrel files (safe for parallel — append-only, last write wins or trivial merge)
   - **Mutating**: editing existing logic in the same function/component (NOT safe for parallel — enforce ordering or split ownership)
4. For mutating shared files, either:
   - Assign exclusive ownership to one stream and make the other depend on it
   - Split the file into separate concerns that each stream owns independently

**Step 3: Per-Stream Legion Analysis**

For each stream with 3+ tasks, evaluate legion viability:

| Factor | Legion YES | Legion NO |
|--------|-----------|-----------|
| Task count | 3+ parallelizable tasks | ≤ 2 tasks |
| File independence | Tasks touch different files | All tasks modify same file |
| TDD phases | Clear test → implement → dependent phases | Purely sequential dependencies |
| Complexity | Well-defined patterns (CRUD, batch ops) | Deep algorithmic work needing full context |

For each legion-viable stream, annotate with a suggested wave structure:

```markdown
**Legion:** Yes
- Wave T: Write tests for [service A, service B, API route] (3 agents)
- Wave I: Implement [service A, service B, API route] (3 agents)
- Wave D: Build [component X, page Y] consuming the API (2 agents)
```

For non-legion streams:
```markdown
**Legion:** No — single complex migration requiring sequential steps
```

**Step 4: Write to Plan**

Add a `## Parallelization` section to the plan file:

```markdown
## Parallelization

### Dependency Optimization
Original critical path: 7 sequential streams
Optimized critical path: 3 phases

Changes made:
- Streams 2, 3, 4: removed false dependency chain (2→3→4). All only need Stream 1.
- Stream 1: absorbed migration from Stream 2 (unblocks parallel execution)
- Stream 5: split into 5a (types) and 5b (implementation) to unlock Stream 6 earlier

### Execution Schedule
- **Phase 1:** Stream 1 (Foundation + migrations) — solo
- **Phase 2:** Streams 2, 3, 4, 5a — parallel (no shared mutable files)
- **Phase 3:** Streams 5b, 6, 7 — parallel (5b depends on 5a; 6,7 depend on Phase 2)
- Shared files: `src/lib/index.ts` (additive barrel export — safe)

### Per-Stream Legion
- Stream 1: No (2 tasks, sequential migration)
- Stream 2: Yes — T(2 agents) → I(2 agents) → D(1 agent)
- Stream 3: Yes — T(3 agents) → I(3 agents)
- Stream 4: No (single complex file)
- Stream 5a: No (types-only, 1 task)
- Stream 5b: Yes — T(2 agents) → I(2 agents)
- Stream 6: Yes — T(2 agents) → I(2 agents) → D(2 agents)
- Stream 7: No (2 tasks)
```

This section is consumed by `/stream` to decide execution mode per stream.

**Rationalization Prevention for Dependencies**

| You're thinking... | Reality |
|---|---|
| "Stream 3 obviously needs Stream 2" | Does it import a file Stream 2 creates? Name the file or it's a false dep. |
| "These should run in order because that's how I'd build it" | Human intuition ≠ machine constraint. Check actual file/type dependencies. |
| "It's safer to keep them sequential" | Sequential = slower. If the files don't overlap, parallel is both safe AND fast. |
| "Splitting this stream is too complicated" | A 20-minute split saves hours of sequential waiting. Do the split. |
| "The user wrote the deps this way" | The user wrote the high-level intent. You optimize the execution. Show your work. |

---

#### Skill Gate

Assigns a concrete list of auto-* skills to each stream. Without this gate, `/stream` falls back to heuristic keyword matching — which works but can miss things.

**Step 1: Build the Baseline**

Every stream gets these skills unconditionally:

```
auto-workflow, auto-coding, auto-errors, auto-naming, auto-edge-cases
```

This is the floor. No stream runs without them.

**Step 2: Assign Per-Stream Skills**

For each stream, analyze its tasks, files, and domain and assign the additional auto-* skills it requires. Use the same mapping table from the Standards Gate (A2), but now you're assigning to specific streams rather than loading globally.

Present the assignments as a table:

```markdown
## Required Skills

### Baseline (all streams)
auto-workflow, auto-coding, auto-errors, auto-naming, auto-edge-cases

### Per-Stream
| Stream | Additional Skills |
|--------|------------------|
| 1 — Foundation | auto-typescript, auto-database, auto-evolution |
| 2 — Financial Ops | auto-typescript, auto-compliance, auto-serialization, auto-security |
| 3 — API Layer | auto-typescript, auto-api-design, auto-resilience, auto-hardcoding |
| 4 — Frontend | auto-typescript, auto-svelte, auto-accessibility, auto-layout, auto-i18n |
```

**Step 3: User Review**

Present the skill assignments for approval. The user may:
- Add skills you missed ("Stream 2 also needs `auto-caching`")
- Remove skills that don't apply ("Stream 1 doesn't need `auto-evolution`, it's a fresh schema")
- Move skills between streams

**Step 4: Write to Plan**

Add the `## Required Skills` section to the plan file. This section is consumed by `/stream` during initialization and written into the status file's `baselineSkills` field per stream.

**The plan is the source of truth for skill assignments.** `/stream` reads this section and loads exactly these skills — it does not fall back to heuristic matching when this section exists.

---

#### Triumvirate

Invoke `/triumvirate` which runs three adversarial subagents (Advocate, Analyst, Critic) to stress-test the plan from different angles. Update the plan file with any changes that come out of the debate.

Recommended for: architectural decisions, high-risk changes, large features.
Skip for: small features, bug fixes, straightforward additions.

---

### A4. Handoff — Clear Context & Use /stream

After the plan is finalized, **recommend clearing context**. The planning session has done its job — it's heavy with brainstorming, standards checking, and potentially triumvirate debate. Implementation sessions should start clean.

#### For plans with stream headers (`## Stream N:`)

Analyze the plan's stream structure and show the dependency graph, then recommend `/stream`:

```
Plan finalized: docs/plans/YYYY-MM-DD-<slug>.md

Optimized from 5 sequential streams → 3 phases (40% reduction):

  Phase 1: Stream 1 (Foundation) — solo
  Phase 2: Streams 2, 3, 4 — parallel
    Stream 2: legion (T:2 → I:2 → D:1)
    Stream 3: legion (T:3 → I:3)
    Stream 4: solo (2 tasks)
  Phase 3: Stream 5 (Integration) — legion (T:2 → I:2 → D:2)

I recommend clearing context now. Two execution options:

  /dominion  — autonomous: spawns headless instances, runs all streams
              in parallel where possible, monitors progress, cascades
              automatically. Walk away and come back to a commit.

  /stream    — manual: you run one stream at a time, clear context
              between each, control the pace yourself.

Recommendation: /dominion for plans with 3+ streams or parallel phases.
               /stream for small plans or when you want hands-on control.
```

This applies to ALL plans with stream headers — even single-stream plans. Cleared context is always a win. The `/stream` skill handles everything: tracking progress, loading relevant skills, managing dependencies, and prompting for the next session.

**Key rules:**
- The plan file is read-only for implementation sessions — they don't modify it
- `/stream` generates a companion `.status.json` file to track progress across sessions
- Each stream has file ownership boundaries enforced by `/stream`
- For parallel-eligible streams, the user can open multiple terminals and run `/stream` in each
- `/stream` **automatically appends a Final Review stream** that depends on all other streams. This final stream verifies everything, runs `/review`, commits/pushes via `auto-git`, then deletes both the plan and status files. Plan authors do NOT need to include this stream — it's injected automatically.

#### For plans without stream headers

If the plan is a simple task list without `## Stream` headers, fall back to the paste-ready prompt:

```
Paste this into a new Claude terminal:
─────────────────────────────────────
/summon
Skip planning — implement the plan at docs/plans/YYYY-MM-DD-<slug>.md
─────────────────────────────────────
```

---

## Path B: No Plan

The user knows what they want. Get to work:

1. Ask what they'd like to do (if not already stated)
2. Load all relevant auto-* skills based on the task
3. Load `auto-workflow` (TDD + verification superpowers apply)
4. Start implementing

---

## Path C: Talk About It

The user isn't sure yet. Help them figure it out:

1. Ask open-ended questions about what they're thinking
2. Explore the problem space collaboratively
3. Once clarity emerges, transition to Path A (plan) or Path B (no plan) based on the task's complexity

---

## Handling "Skip Planning" From Implementation Sessions

When a user pastes a handoff prompt like "Skip planning — implement the plan at docs/plans/...", this is an implementation session spawned from a planning session. Handle it as:

1. Read the plan file
2. Load all auto-* skills relevant to the assigned section
3. Load `auto-workflow` (TDD + verification superpowers apply)
4. Begin implementing the assigned tasks — respect the "Focus on" and "Do NOT touch" boundaries

---

## Situational Skills (User-Invocable Only)

These are **not auto-loaded** — invoke them manually when needed:

| Skill | When to Invoke |
|-------|---------------|
| `/design` | UI component building, design audits |
| `/review` | Code review before committing |
| `/triumvirate` | Adversarial plan review (offered in A3, can also invoke standalone) |

| `/security-scan` | Active vulnerability scanning |

## Output Format

After Phase 1:

```
Foundation loaded. What would you like to do?
1. Plan — brainstorm and write a validated plan
2. No plan — tell me what to build
3. Talk about it — let's figure out the approach
```

After planning + standards gate:

```
Plan written: docs/plans/YYYY-MM-DD-<slug>.md
Standards checked against: [list of loaded skills]
Amendments: [list or "None"]

Optional refinement gates (combine numbers, e.g. "12", "123", "3"):
1. Swarm Gate — optimize dependencies, annotate legion viability
2. Skill Gate — assign auto-* skills per stream
3. Triumvirate — adversarial plan review
0. Skip all — proceed to handoff
Recommended: 123 for large plans, 12 for medium, 0 for simple.
```

After finalization:

```
Plan finalized. Recommend clearing context and starting [N] implementation session(s).
[Paste-ready prompts for each session]
```

## Rules

- **No prompts, no props** — fully automatic after invocation
- **Always offer the three paths** — plan, no plan, talk about it
- **Plans MUST be written to `docs/plans/`** — always, with `YYYY-MM-DD-<slug>.md` naming. Use the Write tool. Never only display the plan in chat without saving to file. If `docs/plans/` doesn't exist, create it first.
- **Standards gate is MANDATORY for all plans** — never skip it, even if it finds no issues, even if the plan seems simple, even if the user is eager to start. Load the relevant auto-* skills and check the plan against them BEFORE proceeding to A3 or A4.
- **Triumvirate is optional** — offer it, recommend based on complexity, but don't force it
- **Recommend clearing context after planning** — the planning session's job is done
- **Multi-session handoffs must have clear file ownership** — prevent merge conflicts
- **Do NOT auto-load** situational skills (design, review, triumvirate, security-scan, evolve, instinct-*)

## Rationalization Prevention

These are NOT valid reasons to skip the standards gate or plan file:
| Rationalization | Reality |
|----------------|---------|
| "The plan is simple enough" | Simple plans still benefit from standards review |
| "The user wants to start implementing" | Standards gate takes 2 minutes, saves hours |
| "I'll check standards during implementation" | Implementation sessions don't have planning context |
| "I already know what standards apply" | Load and check anyway — you'll miss something |
| "I'll just show the plan in chat" | Write it to `docs/plans/` — chat doesn't persist across sessions |
