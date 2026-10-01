---
name: dominion
description: "Autonomous plan executor. Reads a multi-stream plan, orchestrates the entire execution by dispatching background Agent-tool instances — one primary agent per eligible stream, plus verification and remediation agents per stream. Monitors status files, spawns next waves when streams complete, and reports final results. User-invocable via /dominion command. The user's alternative to manually running /stream in separate terminals. Triggers: dominion, auto-execute, run all streams, execute plan."
---

# /dominion — Autonomous Plan Orchestrator

`/dominion` is the hands-off execution layer. Where `/stream` executes one stream per session (manual), `/dominion` runs the **entire plan** autonomously by dispatching background Agent-tool instances and cascading continuously as each dependency's primary lands (pipelined, not phase-barriered — see 2.1).

```
/summon   → creates and validates the plan
/dominion → executes the entire plan autonomously
/stream   → executes one stream at a time (manual alternative to /dominion)
```

Use `/dominion` for full autonomy and `/stream` for hands-on control.

## Design Spine — Adversarial Handoffs Against the Plan Contract

The unifying principle holding the pipeline together: **every handoff in dominion is an adversarial review against the original plan contract, not against the prior agent's self-report.** The baton never carries "this is done"; it carries "does this still match the plan's original requirement?"

| Handoff                           | Receiver's job                                                                                                              |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Stream agent → verification agent | Read the shipped code. Does it match the plan's stream section? Not: does it match what the stream said it did?             |
| Verification → remediation        | Treat findings as an attack surface. The remedial agent re-derives fixes from plan + skill rules, not from trust.           |
| Remediation → re-gate             | Re-run the gate adversarially against the same original contract. Fixes don't get a pass because remediation made them.    |
| Dominion → Codex `/verify`        | Final independent adversarial pass before anything merges.                                                                  |

No stage inherits trust from the prior stage. Every stage plays Critic against the plan. The mechanisms below — Agent-mode execution, three-input remediation, the briefing packet model, phase gates — all exist to make this concrete.

## Automatic Quality Routing and Auto Mode

Immediately load `auto-chat-quality`, `auto-code-quality`, `auto-writing-quality`, and `auto-workflow` with its `references/quality-routing.md`. On Claude use Skill; on Codex resolve/read the installed `SKILL.md`. Resolve references from their skill directory. Route new/changed design to `auto-design-quality` on actual work, except plain unchanged reuse of established components. These adapters remain active through later turns, compaction, resume, direct orchestrator edits, and every worker role.

Default to auto mode: resolve routine choices, show the execution preview, and proceed without a new approval question. Run missing plan/refinement checks yourself; keep `review` as the default final-validation mode unless `codex` is explicitly selected. Fix supported in-scope issues and choose upstream subcommands yourself. Honor explicit manual/review-only limits and actual permission boundaries. Ask only for material missing intent, ambiguous ownership/active takeover, or a genuine blocker that cannot be resolved safely from context.

**Every subagent must load its applicable quality skills in its own context before work.** This includes primary, verification (code reading/review counts), remediation, surgical follow-up, research/report writers, both final reviewers, and any delegated fix owner. Chat quality applies to all workers; code quality applies to all code work/review; writing quality applies to every human-facing response, report, doc, or UI string; Impeccable applies to design creation/review/editing. The final security worker additionally loads `auto-security-quality`. Pass resolved skill paths and the actual assignment manifest in each prompt. A parent's loaded skill names or short rule excerpts do not satisfy this requirement. Require returned load evidence and re-audit affected output after any missed load.

## Orchestrator-Bears-the-Skills — Briefing Packet Model

Dominion owns exactly one context. N agents don't multiply that work; they inherit it. Agents receive pre-digested domain briefing packets plus mandatory first-step quality skill loads. Domain excerpts reduce repeated context work; they do not replace the quality adapters in each worker context.

### Dominion's one-time work per plan

- Load `CLAUDE.md` + baseline skills into its own context
- Read the plan file end-to-end
- For each stream, precompute a **briefing packet**:
  - Stream section text (verbatim or extracted key passages)
  - **Skill-rule excerpts** — just the enforceable rules from each declared skill, not the full skill doc (~300 tokens per skill vs. ~2–3k for the full doc)
  - Line-range excerpts from reference files the stream will touch (e.g., `sessions.ts:40-95` + `:130-200`, not the whole file)
  - Cross-stream intake items from prior streams' verification

### What goes in each agent prompt

Primary, verification, and remediation agents all receive the relevant briefing packet inline + the specific work for their role. The domain context is handed to them. Each worker still resolves and loads its assigned quality skills and selected references before acting.

### Why this is canonical, not an optimization

1. **Adversarial handoff gets sharper.** Dominion, which owns the original plan contract, curates each receiver's briefing against that contract. No stage slips in its own interpretation of "what matters."
2. **Curation needs pipeline history.** Dominion has seen the plan AND earlier stages' findings. It can hand the verifier exactly which skill rules to check against which files. It can hand the remedial agent exactly which rules to apply to which finding. A fresh agent loading skills on its own has no way to make that call.
3. **Skill updates propagate cleanly.** Dominion re-reads skills on each run; every new briefing picks up the latest rules. No agent-side version drift.
4. **Token efficiency compounds** — domain curation is performed once by dominion and reused; mandatory quality skill loads still occur in each worker context.

### Fallback: agent loads its own skills

- Manual `/stream` (no orchestrator exists): stream loads its own skills — unchanged from today
- Rare agent that hits an unknown mid-work: it can still call the Skill tool for a skill dominion didn't anticipate
- 1–2 stream plans where orchestrator overhead isn't worth it

## When to Use

- Plans with 2+ streams where the user wants to walk away
- Plans with parallel-eligible streams (maximum time savings)
- After `/summon` has finalized a plan with the parallelization section

## When NOT to Use

- For a single implementation stream, use the supporting `stream` workflow internally and preserve its two final review siblings; do not ask the user to invoke another command
- When the user wants to review between streams
- Plans involving risky operations that need human judgment between streams (destructive migrations, external API changes, production deployments)

## Invocation

```
/dominion                              # Auto-detect plan
/dominion docs/plans/slug.md           # Explicit plan file
/dominion --status                     # Show live progress
/dominion --dry-run                    # Show execution schedule without spawning
```

---

## Phase 1: Resolve Plan, Ensure Status File, and Validate

### Step 1: Resolve the plan

First read the installed `stream` skill's `references/plan-lifecycle.md` and apply its automatic three-day plan pruning before discovery. Protect explicitly selected/current work and confirmed live owners; `--dry-run` and explicit read-only requests only report eligible cleanup. Then follow `/stream` Phase 1: check remaining active status files, recent existing plans, and ask only if still ambiguous. Check retained finalization receipts for interrupted cleanup before announcing an already-completed plan is done.

### Step 2: Ensure status file exists

Once a plan is resolved, immediately check for its companion `.status.json`. If it exists, load and normalize it using the installed `stream` skill's `references/status-schema.md`. If not, create it:

1. Parse all stream headers matching `## Stream N:` or `## Stream N —`
2. For each stream, extract: name, dependencies, files owned, sub-streams
3. Parse the `## Required Skills` section (if present):
   - Extract **Baseline** skills (apply to all streams)
   - Extract **Per-Stream** skill assignments from the table
   - Combine baseline + per-stream into a `baselineSkills` array for each stream
   - If no `## Required Skills` section exists, set `baselineSkills` to `[]`
4. Parse the `## Final Validation Mode` section (if present):
   - `Mode: codex` → final validation uses `codex-validation`
   - `Mode: review` → final validation uses classic `review`
   - If absent, default to `review`
5. Write `docs/plans/{slug}.status.json` with all streams set to `pending`, including `baselineSkills` per stream and the selected `finalValidationMode`
6. Inject the two reserved sibling IDs `final` (Final Validation for review mode, Final Cleanup for codex mode) and `final-security` (Final Security Audit). Both depend on every implementation ID and neither depends on the other. Initialize the stable `reviewBaseline` before implementation. Use the installed `stream` skill's `references/status-schema.md` as the canonical migration, snapshot, evidence, and concurrency contract. Normalize legacy status idempotently without resetting progress or silently bypassing security on an old completed final stream.

### Step 3: Pre-compute briefing packets (NEW)

Before spawning anything, pre-digest each stream's briefing packet so primary/verification/remediation agents can be dispatched cheaply:

1. **Load declared skills and the automatic quality floor into dominion's own context.** For each unique skill across all streams, load it once. Extract the enforceable rules (the "must/must not" lines and concrete anti-patterns) into a compact rule-excerpt block per skill.
2. **Read reference files.** For each file a stream will touch, identify the relevant line ranges (interfaces, call sites, schemas the stream must respect). Store these as excerpts.
3. **Build per-stream packets** — one dict per stream containing:
   - `stream_section`: verbatim markdown of that stream's section in the plan
   - `skill_rules`: map of `{skill_name: rule_excerpt}` for each declared and automatically routed skill
   - `quality_skill_manifest`: actual assigned skill names, absolute installed paths, required references/subcommands, and first-step load instructions for this worker role
   - `quality_load_evidence`: returned per-worker skill name/path, invocation or read method, loaded references, and evidence of the resulting audit (initially empty)
   - `reference_excerpts`: map of `{file_path: {line_range: text}}`
   - `cross_stream_intake`: items from upstream streams' verification findings (populated as earlier phases complete)

Cache these in memory for the dominion run. They are reused by primary, verification, and remediation agents — digest once, hand out N times.

### Step 4: Pre-flight validation

Before spawning anything, verify:

1. **Plan has a `## Parallelization` section** — if not, derive and validate the execution schedule using the relevant gate automatically
2. **Plan has at least one implementation stream** — execute a single-stream plan through the supporting `stream` workflow internally; use full scheduling for multiple streams
3. **Execution schedule is parseable** — the `### Execution Schedule` from the parallelization section defines the phases
4. **No streams are currently `in_progress`** — if any are, another dominion/stream session may be active. Ask the user before proceeding.

### Step 5: Show execution preview

```
Plan: docs/plans/2026-04-22-feature-overhaul.md
Execution schedule (from parallelization section):

  Phase 1: Stream 1 (Foundation)              — 1 primary + 1 verifier + ≤1 remediator
  Phase 2: Streams 2, 3, 4 (parallel)         — scheduled within host capacity and reviewer reservation
  Phase 3: Streams 5, 6 (parallel)            — scheduled within host capacity
  Phase 4: Final review/cleanup + security    — 2 concurrent read-only reviewers, then joined remediation

Per-stream lifecycle roles: primary + verification + remediation
  At most ONE surgical follow-up after remediation fails; then handle inline or resolve the blocker.
  Bounded task helpers and required independent reviewers use the shared host pool separately.
Concurrent worker limit: {effective capacity and runtime/config source}

Estimated: ~4-5 phases
Starting the validated execution schedule.
```

Proceed after the preview in auto mode. Pause here only when the user explicitly requested an execution approval checkpoint.

---

## Runtime Capacity and Independent Reviewers

Read the installed `auto-workflow` skill's `references/agent-capacity.md` before dispatch. Resolve the live capacity and its counting unit: Claude's current default is 20 running subagents; Codex uses its exposed/configured worker pool, which may count open threads or active turns. Count the orchestrator only when the host's limit includes it. Report the effective worker limit and source in the execution preview. The normal 3/4 implementation roles per stream are a total lifecycle budget, not a global concurrency cap. Fill every available slot with eligible work, queue excess work, and refill immediately on completion. Codex runs the same full workflow at its available capacity.

An implementation primary normally runs its own Legion phases without nesting; when a phase has independent tasks and spare capacity, it returns bounded task packets for Dominion to dispatch under the shared capacity contract. Impeccable finish review and security candidate/final validation remain required independent roles outside the generic implementation-role budget. Prefer coordinator-brokered reviews: a worker returns the exact review assignment and artifacts, releases capacity under the host's counting rules, and Dominion schedules a fresh reviewer before resuming the owner. Reserve capacity only for resident-parent checks or known fresh reviewers in an open-thread pool with no release tool, as defined in the shared contract. Do not reserve a hypothetical reviewer slot throughout ordinary implementation or fill the pool with parents waiting for children. In a four-total-slot runtime, use all three worker slots for independent implementation; the two final readers can use two slots with the third available for their required child review. Smaller pools use sequential independent passes under the same final barrier.

## Phase 2: Execute Phases

For each phase in the execution schedule:

### 2.1 Identify Eligible Streams (pipelined, not phase-barriered)

**This pipeline applies only to implementation streams. Reserved `final` and `final-security` are excluded; both wait for every implementation verification/remediation/late delta to settle and use Phase 3.**

Dominion does NOT wait for a whole implementation phase to settle before starting the next. It schedules **continuously** on an eligibility check. Read the status file; a `pending` stream becomes eligible to DISPATCH ITS PRIMARY as soon as:

- Every dependency's **primary** has reached `completed` (its artifacts are on disk).

That's the whole gate. Eligibility keys on the dependency's **primary completion**, NOT on the dependency having passed verification or remediation. The bet — validated in practice — is that most stream work is correct on first implementation, so a downstream can safely build on an upstream's shipped code while that upstream is being verified/remediated in parallel. This collapses the critical path from "sum of phases" toward "the slowest primary at each dependency link."

**Do NOT gate a downstream on an upstream's full settle — even when they share a file.** Mild overlap during the upstream's verification/remediation is acceptable and expected; waiting for full settle throws away most of the pipelining win. What keeps the overlap safe is not serialization but two coordination facts:

1. **Verification agents are READ-ONLY.** A downstream primary editing file F while the upstream's *verifier* reads F is zero conflict — the verifier never writes. So a shared file with a still-*verifying* upstream is never a reason to wait.
2. **Ownership passes forward on dispatch.** The instant downstream D is dispatched, D owns every file it will edit — including files it shares with an upstream U. If U's *remediation* later finds an issue in one of those handed-off files, U's remediator does NOT edit it; it reports the issue as a cross-stream finding routed into D's remediation (2.5 late-delta). Concurrent blind writes to one file are prevented by this hand-off, not by making D wait.

The only thing that still hard-serializes is **two PRIMARY agents editing the same file in the same wave** (e.g. two sibling streams that both own one 2000-line admin page). The plan's file-ownership matrix already sequences those (e.g. a `3 → 7 → 4` sub-sequence). That is a primary-vs-primary constraint — NOT a reason to wait on any upstream's verification or remediation.

The `## Parallelization` "phases" remain the mental model for the preview and the final report, but execution is eligibility-driven: as each primary lands, re-scan for anything newly unblocked and dispatch it immediately.

### 2.2 Dispatch Primary Stream Agents (Agent Tool, Background)

For each eligible stream that fits the available worker pool, dispatch a **background Agent-tool agent** with the pre-computed briefing packet. Honor only currently necessary reviewer reservations. No subprocess, no stdio plumbing, no log-tail parsing.

**Mechanism:** use Claude's Agent tool with `subagent_type: "general-purpose"` and `run_in_background: true`, or Codex's native collaboration tools. Preserve ownership boundaries. Dispatch every eligible assignment that fits now; queue the remainder and refill slots as each worker finishes, without waiting for the whole wave. Collect evidence and close/release completed agents when the host counts open threads; hosts that count only active turns free capacity when those turns finish. Capacity errors queue work until a slot is released, without repeated spawn attempts or bypasses.

**Prompt template — Primary Stream Agent:**

```
You are the primary agent for Stream {id} of plan `{plan_path}`.

## Required first step: load your quality skills

{quality_skill_manifest_with_resolved_paths}

Before reviewing/editing code, load `auto-code-quality` yourself. Before any human-facing prose/report, load `auto-writing-quality` yourself. Load `auto-chat-quality` immediately and `auto-design-quality` for design work/review beyond unchanged established reuse. Use Skill on Claude or read the installed SKILL.md on Codex, then selected references relative to that directory. Follow the shared quality-routing contract and upstream adapters; no extra user setup prompts. Return actual load evidence. Parent excerpts are not a substitute. If you discover a missing load, load it and re-audit prior affected work before returning.

## Briefing Packet (read first; do not edit anything yet)

### Stream section
{stream_section_verbatim}

### Skill rules (apply these to every decision and every file you touch)
{per_skill_rule_excerpts}

### Reference file excerpts (what you need from other files)
{line_range_excerpts}

### Cross-stream intake (reconciliations from upstream streams)
{cross_stream_intake}

## Coordination protocol

1. Read `{status_path}` and confirm dominion assigned Stream {id} to you; abort on conflicting ownership.
2. Dominion is the sole status writer. Do not edit shared status; it has recorded your claim before dispatch.
3. Return completion time, verification results, actual skill-load evidence, and deferrals. Dominion verifies the evidence and records completion serially.

## File ownership

You own:
{file_list_from_plan}

Do NOT edit files outside this list unless the plan's cross-stream intake explicitly directs you to. If you find a gap in another stream's file that blocks your work, record it in your `deferrals` return field — do not patch it yourself.

## Execution

Implement the stream's sub-tasks using TDD where applicable. With `Legion: Yes`, preserve Test, Implement, and Dependents phase order. When a phase has bounded independent tasks, return their task packets to Dominion for dispatch into spare slots under `auto-workflow/references/agent-capacity.md`; do not independently overbook the pool. Transfer exact file ownership and stop editing delegated files. Return a checkpoint and release your capacity if only waiting, then resume for integration. Otherwise perform the work locally. Required independent design/security reviews use the Runtime Capacity and Independent Reviewers protocol.

After each cluster of file edits, run `pnpm exec vitest run <touched files>` and the type checker. Fix issues before the next cluster.

## Self-audit before marking completed (mandatory)

Before returning completion for dominion to record, walk the files you touched and re-check each declared skill's rules against your diff. Fix obvious violations now — you're the cheapest place to catch them.

## Verification gate (must pass before marking completed)

Run these commands directly; capture exit codes. Do NOT pipe `pnpm test` to `head`/`tail`/`grep` — the `| tail -N` pattern hangs if any new test file leaves a handle open (a fresh promise, an unreleased timer, a redis reconnect loop). Use `pnpm exec vitest run` directly.

1. `pnpm check`                           → 0 errors required
2. `pnpm lint`                            → 0 errors required
3. `pnpm exec vitest run {test_glob}`     → all tests pass

Stream-specific smoke checks, if the plan defines any, go last.

## Return format (structured, < 300 words)

SUMMARY
- files: <git diff --stat, abbreviated>
- gates: { check: pass|fail, lint: pass|fail, tests: N/M passed }
- deferrals: [
    { "item": "<what was deferred>",
      "reason": "<why>",
      "owner_suggested": "<which stream/role should handle>" }
  ]
- skillLoads: [{ skill, resolvedPath, method: Skill|read, role, references }]
- notes: <anything Phase 2.3.5 verification should know>

No narration beyond the structured fields.
```

**Key details:**
- The Agent tool returns the agent's final message directly to dominion's context — no log parsing, no `while ps -p` polling
- Structured `deferrals` field feeds Phase 2.4 remediation
- The self-audit instruction turns declared skills into Layer 2 enforcement (see Layered Enforcement below)

### 2.3 Await Agent Completion

Claude background Agent calls notify Dominion on completion. On Codex, use the available native wait/status tools and returned notifications; do not wait for a Claude-specific event that host does not emit. Avoid shell polling or scheduled wakeups. After each completion, collect the result, release capacity according to the host's counting rules, and dispatch newly eligible work immediately.

When a result arrives:

1. Read the agent's returned SUMMARY. A checkpoint requesting task helpers or an independent reviewer queues those assignments and keeps the stream in progress; it does not unlock downstream streams. Helper completion returns to the stream owner for integration. Continue the completion steps below only after the primary returns its final integrated result
2. Check the returned evidence and actual artifacts, then record the stream's completion in `docs/plans/{slug}.status.json` as the single writer; leave `settledAt: null` until verification/remediation finishes
3. Run `git diff --stat` to see what files actually changed
4. Record the structured deferrals from the agent's return (they feed 2.4)
5. If the completed agent was a **primary**: immediately enqueue its verification agent (2.3.5) and every downstream primary newly eligible under 2.1. Dispatch as many as fit the currently available capacity, prioritizing required verification; queue the rest. Verification/remediation and downstream primaries can overlap when slots permit. Do not impose a phase-settle barrier or wait for upstream verification merely to make a downstream eligible.

```
[03:45:12] Phase 2 — 3 primary agents dispatched
[03:52:30] Stream 4 primary: completed (8m, 4 files, 0 deferrals)
[03:52:31] Stream 4 → dispatching verification agent
[03:54:15] Stream 2 primary: completed (11m, 7 files, 2 deferrals)
[03:54:16] Stream 2 → dispatching verification agent
[03:56:40] Stream 3 primary: completed (12m, 3 files, 1 deferral)
[03:56:41] Stream 3 → dispatching verification agent
```

### 2.3.5 Verification Agent (Independent Adversarial Pass — MANDATORY)

**Run this for EVERY stream between primary exit and remediation. Never skip for "trusted" streams — there are no trusted streams.**

Stream self-reports are unreliable. A real run across 7 parallel Sonnet streams produced: every stream reported "completed" with clean gate results, but 6 of 7 had material deviations from the plan's post-review amendments — including Stripe-impossible coupon codes, test files in `src/routes/` that broke SvelteKit, migration number collisions, and silent-fallback patterns the amendments explicitly forbade.

Dispatch a **verification agent** as a fresh Agent-tool call (fresh context, new conversation — it has NOT seen the primary's work). The agent's only job is adversarial verification of shipped code against the plan contract.

**Prompt template — Verification Agent:**

```
You are the verification agent for Stream {id}. Your job is adversarial: assume the stream shipped code that deviates from the plan, and find the deviations.

## Required first step: load your quality skills

{quality_skill_manifest_with_resolved_paths}

Before reviewing/editing code, load `auto-code-quality` yourself. Before any human-facing prose/report, load `auto-writing-quality` yourself. Load `auto-chat-quality` immediately and `auto-design-quality` for design work/review beyond unchanged established reuse. Use Skill on Claude or read the installed SKILL.md on Codex, then selected references relative to that directory. Follow the shared quality-routing contract and upstream adapters; no extra user setup prompts. Return actual load evidence. Parent excerpts are not a substitute. If you discover a missing load, load it and re-audit prior affected work before returning.

## Briefing Packet

### Stream section (the authoritative contract)
{stream_section_verbatim}

### Skill rules relevant to this stream's files
{per_skill_rule_excerpts}

### Files the stream claims to own
{file_list_from_plan}

### Primary agent's self-reported summary
{primary_agent_summary}

### Primary agent's self-reported deferrals
{primary_agent_deferrals}

## Your task

DO NOT trust the primary's self-report. Re-derive every requirement from the plan's stream section, then verify against shipped code.

For each requirement:
- Use grep/Read to verify it was implemented (e.g., amendment says "use X not Y" → confirm X present, Y absent)
- Check the primary's deferrals — are they legitimate (genuinely out of scope) or rationalizations (the primary punted on in-scope work)?
- For each declared skill, walk the diff and flag rule violations as findings

Verification grep patterns to use as appropriate:
- "Use X not Y" → grep for X (must be present), grep for Y (must be absent)
- "Delete Z" → grep for Z (zero hits)
- Column NOT NULL/default → read schema file, confirm constraint
- Env var gate → grep for var + presence of boot-time gate
- Function signature → read the function, count parameters
- Hardcoded value to remove → grep src/ and docs/ for it (zero hits)
- Test file location → ls the directories
- Cap/limit → read handler, confirm constant + slice/limit

## Return format (structured, < 500 words)

FINDINGS
- [ { "severity": "blocker" | "correctness" | "quality",
      "skill": "<declared skill name or 'plan-contract'>",
      "file": "<path>",
      "line": <number if applicable>,
      "evidence": "<what's there vs what plan requires>",
      "fix_scope": "<minimal description of fix>" },
    ... ]

DEFERRAL_ASSESSMENT
- { "id_or_item": "<from primary's deferrals>",
    "verdict": "legitimate" | "rationalization" | "partial",
    "reasoning": "<one sentence>" }

SKILL_LOADS
- [{ skill, resolvedPath, method: Skill|read, role, references }]

SUMMARY
- blockers: N, correctness: N, quality: N
- overall: proceed | remediate | halt
```

### Quality Skill and Design Verification

Check each worker's actual quality-skill load evidence and the output it shaped. Missing code-quality loading before code review/edits or writing-quality loading before prose is a blocking workflow gap: load the missed skill and re-audit the affected output before accepting completion. Impeccable design work must identify preserved project constraints, relevant loaded references/commands, and checks on the resulting surface; do not impose ui-ux search-log artifacts on auto-design-quality streams.

**Acting on findings:**

| Findings | Action |
|---|---|
| Empty | Still dispatch lightweight remediation with Input 3 only, then settle the stream |
| Quality items | Dispatch remediation |
| Blockers/correctness | Dispatch remediation; do not mark the stream settled until its re-gate passes |
| Impossible plan or contract conflict | Resolve within authorized scope if evidence supports a safe correction; otherwise report the concrete blocker and ask only for the material missing decision |

### 2.4 Remediation Agent (Three-Input Adversarial Wave)

Dispatch ONE remediation agent per stream. The agent takes **three inputs** and acts on all three — a mechanical fix-list applier is not enough.

**Input 1 — Primary's deferrals:** What the stream self-reported as out-of-scope or deferred.
**Input 2 — Verification findings:** Skill-tagged, file:line-anchored findings from 2.3.5.
**Input 3 — Free-form standards audit:** The remediator walks the stream's diff against its declared skills and reports any clear rule violations the primary's self-audit AND the verification agent both missed.

Input 3 is the difference between a fix-list applier and a real QA layer. Primary agents miss things; verification agents miss things; a fresh scan from a fresh agent loaded with just the stream's declared skills catches the third-order misses.

**Guardrails on the free-form scan:**

- **File scope:** only files the stream declared ownership of — never reach into other streams' files
- **Skill scope:** declared domain skills plus the automatic quality floor and actual-work design/writing routing; a legacy declaration cannot exclude required quality skills
- **Edit scope:** one violation = one minimal edit. No "while I'm here, let me refactor this function." Restraint is a feature.
- **Escalation rule:** if the free scan finds a blocker not already in 2.3.5's list (security hole, data-loss path, broken invariant), return its evidence and ownership needs to Dominion. Dominion routes supported in-scope fixes to the right owner automatically; ask only when a material decision, permission boundary, or genuinely unresolvable conflict remains.

**Prompt template — Remediation Agent:**

```
You are remediating Stream {id}. Three input lists follow; act on all three.

## Scope (hard constraints)

- Files: {stream_files}  (never edit outside this set)
- Skills: {stream_skills_plus_quality_floor} (domain assignments plus mandatory actual-work routing)

## Required first step: load your quality skills

{quality_skill_manifest_with_resolved_paths}

Before reviewing/editing code, load `auto-code-quality` yourself. Before any human-facing prose/report, load `auto-writing-quality` yourself. Load `auto-chat-quality` immediately and `auto-design-quality` for design work/review beyond unchanged established reuse. Use Skill on Claude or read the installed SKILL.md on Codex, then selected references relative to that directory. Follow the shared quality-routing contract and upstream adapters; no extra user setup prompts. Return actual load evidence. Parent excerpts are not a substitute. If you discover a missing load, load it and re-audit prior affected work before returning.

## Briefing Packet

### Stream section (the contract)
{stream_section_verbatim}

### Skill rules
{per_skill_rule_excerpts}

## Input 1 — Deferrals from primary

{primary_deferrals_list}

For each: if addressable with a minimal edit inside scope, fix it. If genuinely out-of-scope, repeat it unchanged in your return.

## Input 2 — Verification findings

{findings_list_with_skill_tags}

For each: fix with a minimal edit. If you believe the finding is wrong, argue back in your return — do not silently skip.

## Input 3 — Free-form standards audit

For each file in scope, walk the diff against each declared skill's rules. Report and fix any clear violations the primary and verifier missed. Minimal edits only.

## Re-run the gate after fixing

pnpm check
pnpm lint
pnpm exec vitest run {stream_test_glob}

(Direct commands — do NOT pipe to head/tail/grep if you need to see full output.)

## Return format (structured, < 400 words)

REMEDIATION_RESULT
- skillLoads: [{ skill, resolvedPath, method: Skill|read, role, references }]
- fixed: [ { "input": "1|2|3", "file": "<path>", "change": "<one-line>" }, ... ]
- skipped-with-reason: [ { "input": "1|2|3", "item": "<...>", "reason": "<...>" } ]
- new-blockers: [ { "file": "<path>", "issue": "<...>" } ]
- gate-result: { check: pass|fail, lint: pass|fail, tests: N/M passed }
- free-audit-summary: "<one sentence on what the free scan found>"
```

**After remediation returns:**

| Remediation return                                                    | Dominion action                                                              |
| --------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Gate: pass; no new blockers                                            | Set `settledAt` after checking the completed verification/remediation evidence. Proceed to the next eligibility scan.                     |
| Gate: fail; narrow scope (1–3 files, well-understood)                  | **Dominion handles inline** using its own Read/Edit/Bash. No additional agent. |
| Gate: fail; broad scope where inline burn would be costly              | Dispatch ONE more surgical agent with an explicit, narrow prompt. This is the single allowed follow-up role. |
| New blockers surfaced (plan-level or cross-stream)                     | Route the finding to its owner and resolve within the existing authorized scope and bounded agent budget; escalate with evidence only if a material unresolved decision remains.  |

The normal per-stream lifecycle is primary, verification, and remediation, with **one surgical follow-up at most** after a failed remediation gate. This bounds the retry loop; it is not a global concurrency limit or a ban on bounded implementation task packets and required independent reviewers. Count every live role/helper against host capacity and include them in actual dispatch totals. Helpers must not bypass the follow-up limit by relabeling repeated failed remediation as new tasks.

### 2.5 Continuous Scheduling & Late-Reconciliation

There is no hard phase barrier. Scheduling is continuous (2.1): every time any agent finishes, re-scan for newly-eligible streams and dispatch them. A "phase" is complete only in the reporting sense — when every stream assigned to it is fully settled. Two things still matter at each settle:

1. **Propagate cross-stream intake — including LATE deltas.** When an upstream's verification/remediation changes an artifact's shape AFTER a downstream already started (or finished) against the old shape, feed that delta into the downstream's remediation as an Input-2 finding ("upstream U changed X from shape A→B; reconcile"). This is the price of pipelining: the shared contract can shift late, and the mechanism that keeps it honest is routing upstream remediation deltas into downstream remediation. Do not silently drop them — a pipelined downstream that consumed a since-revised artifact is the one real failure mode this model introduces, and this step is its backstop.
2. **Final gate.** Proceed to Phase 3 only when ALL implementation streams (excluding both `final` and `final-security`) are `completed` AND all verification/remediation and late deltas have settled.

### 2.6 Failure Handling

**Primary agent returns with gate failure AND empty deferrals list (it tried but couldn't pass):**
- The primary is saying "I shipped what I could; the gate still fails and I don't know why"
- Dispatch verification and remediation as normal — they may catch what primary missed

**Primary agent returns with a crash or abort:**
- Read the partial diff on disk; read status.json
- If status is still `in_progress`, reset to `pending` and re-dispatch the primary ONCE
- Max 2 primary retries per stream before escalating to user

**Remediation agent returns with new blockers:**
- These indicate plan-level problems (contract conflict, impossible requirement, cross-stream gap)
- Investigate and route the issue to its owner within the existing scope and bounded retry policy. Ask only if a material missing decision or authorization prevents a correct fix; include the evidence and preserve completed work.

**Multiple streams fail in same phase:**
- If 2+ streams in the same phase fail verification AND remediation, investigate the shared cause in the orchestrator, fix supported in-scope issues under the existing budget, and ask only for a genuinely unresolved material decision
- Could indicate a systemic issue (broken dependency, bad plan)

---

## Layered Skill Enforcement (Four Depths)

Declared skills must shape each stream's output, not just get loaded and forgotten. Four enforcement depths, each catching what the previous missed:

1. **Primary work — skills loaded up front** (mandatory worker quality loads plus the domain briefing packet). Shapes decisions during implementation.
2. **Self-audit pass before marking completed** (part of the primary agent's contract, see template). Catches easy-to-see violations the primary introduced while focused on feature work.
3. **Verification agent (2.3.5)** — adversarial read against plan + skills. Catches what self-audit missed.
4. **Remediation agent (2.4) — three inputs** (deferrals + findings + free audit). Catches what verification missed AND fixes what was flagged.

Skills never sit inert. Each layer has a specific job: primary shapes design, self-audit catches rough edges, verification catches cross-stream gaps, remediation fixes what escaped.

---

## Phase 3: Two Concurrent Final Streams

This phase always runs, even when implementation needed no remediation. Read the installed `stream` skill's `references/status-schema.md` and use its canonical snapshot/join protocol; Phase 2 primary-completion pipelining and generic worker status rules do not apply to these reserved siblings.

1. **Settle and prepare.** Wait for all implementation verification, remediation, and late contract deltas. Run full project checks and perform any pre-snapshot fixes through one owner. Preserve the original `reviewBaseline`, capture committed/staged/unstaged/untracked changes and context in one immutable manifest, and stop source writers.
2. **Dispatch both readers in the same wave.** Claim `final` and `final-security` as siblings, each depending on all implementation streams. Give both the same baseline, snapshot ID, complete change manifest, and exclusive artifact roots and returned report paths. Each worker must first load its own `auto-chat-quality`, `auto-code-quality`, and `auto-writing-quality`, plus applicable design/domain skills. The security reader also loads `auto-security-quality`. Include the actual resolved skill manifest and demand load evidence in both prompts. Assign `final` the temporary-log cleanup inventory from `stream/references/plan-lifecycle.md`; it returns exact plan-owned paths for deletion after the joined finalization barrier.
3. **Review concurrently without source mutation.** `final` performs the existing classic `review` findings pass in review mode, or the broad Claude Cleanup Review in codex mode (no `codex-validation` yet). `final-security` audits the entire captured change set and adjacent trust boundaries with `auto-security-quality`. Workers can write only their assigned artifact roots (the security wrapper uses its permitted external run directory by default and returns all six-phase reports/validators); neither edits shared status, fixes code, commits, pushes, cleans up, or hands off. Functional source-mutating checks use isolated copies. Security target execution additionally requires the upstream OS-enforced sandbox; otherwise use source review and record that exact validation limit. Preserve security independent fresh-agent checks and use the capacity reservation/broker above. If parallel workers are unavailable, perform both independent passes sequentially under the same barrier and record that limitation.
4. **Join and fix.** Wait for both reports. Validate their skill loads, coverage, and snapshot fingerprints. Present combined findings, then assign one remediation owner to apply supported authorized fixes after readers have finished. No concurrent reviewer-fixer races. Preserve explicit review-only limits and escalate genuine blockers under the existing policy.
5. **Refresh both gates.** Re-run affected checks, capture a new snapshot, and obtain fresh evidence from both reviewers on the fixes and regression surface. Missing/incomplete audits or stale snapshots cannot pass. Dominion alone marks both streams completed and `finalGate.phase: passed` once both reports pass on the same current snapshot.
6. **Finalize once and clean the logs.** In `review` mode, the coordinator performs or resumes `final` as the sole finalization owner after both readers stop. Complete authorized commit/push, retain audit evidence outside `.dominion-logs`, then delete this plan's temporary logs and plan/status using `stream/references/plan-lifecycle.md`. Verify removal and record cleanup in the retained receipt; pending cleanup prevents a full-completion claim. Preserve unrelated user work and other plans' logs. In `codex` mode, preserve the working tree, plan, status, temporary logs, baseline, both reports, and cleanup inventory for the existing Codex `/verify` handoff; Codex completes cleanup after actual validation and authorized finalization. If already running in Codex with authority to validate, continue `/verify` directly. Otherwise report the handoff honestly. Never claim the Claude cleanup or security report replaces Codex validation.

A later mutation invalidates evidence for affected scope and requires fresh checks/reviews before commit, push, cleanup, or Codex handoff. Do not replay a recorded commit/push after resume. The final pair has two initial reviewers; remediation and re-review use the existing bounded follow-up policy, not a separate unbounded audit loop.

Example Codex handoff:

```
Implementation, Final Cleanup, and Final Security Audit are complete.
Both final reports pass on snapshot {snapshotId}; artifacts are preserved.
Final validation mode: codex. Codex validation has not run yet.
Next step when a Codex runtime is unavailable here: open Codex and run /verify.
```

---

## Phase 4: Report Results

### Success

```
/dominion complete ✓

Plan: docs/plans/2026-04-22-feature-overhaul.md
Streams: 8/8 completed (6 implementation + 2 final siblings)
Agents dispatched: 18 (avg 2.6/stream)
Duration: 34 minutes (vs ~2h sequential estimate)
Commit: abc1234
Branch: main

Phase breakdown:
  Phase 1 (Stream 1):          6 min   (primary + verification, no remediation needed)
  Phase 2 (Streams 2,3,4):    12 min   (parallel; Stream 2 needed remediation)
  Phase 3 (Streams 5,6):       9 min   (parallel; both clean)
  Phase 4 (Review + Security): 7 min (concurrent reads, joined remediation)

Plan, status, and plan-owned temporary logs cleaned up by the finalization owner.
Final reports and cleanup receipt: docs/plans/.dominion-audit/{slug}/
```

### Codex Handoff

```
/dominion implementation and cleanup phases complete ✓

Plan: docs/plans/2026-04-22-feature-overhaul.md
Implementation streams: 6/6 completed
Final Cleanup: completed
Final Security Audit: completed
Final gate: both pass on {snapshotId}
Final validation mode: codex

No Claude Codex-validation stream was spawned.

Next step:
  Open Codex and run `/verify`

Preserved for Codex:
  - docs/plans/2026-04-22-feature-overhaul.md
  - docs/plans/2026-04-22-feature-overhaul.status.json
  - docs/plans/.dominion-logs/{slug}/ (temporary; remove after Codex finalization)
  - docs/plans/.dominion-audit/{slug}/ (retained evidence and cleanup receipt)
```

### Failure

```
/dominion stopped — Stream 3 remediation surfaced new blockers

Completed: Streams 1, 2, 4 (3/7)
Failed: Stream 3 (see findings below)
Blocked: Streams 5, 6, Final Validation, Final Security Audit

Findings:
  - src/lib/server/auth/sessions.ts:142 — primary shipped plaintext
    session tokens; amendment required sha256 hashing
  - migrations/0042: collides with existing migration number 0042

Status file preserved: docs/plans/2026-04-22-feature-overhaul.status.json
Briefings + agent returns: docs/plans/.dominion-logs/{slug}/

Needed to continue: [specific missing decision or external-state change]
After it is resolved, /dominion resumes from the preserved status.
```

---

## Artifact Management

### Artifacts directory

```
docs/plans/.dominion-logs/{slug}/
  briefing-stream-1.json          # The briefing packet dominion built
  briefing-stream-2.json
  ...
  return-stream-1-primary.md      # What the primary agent returned
  return-stream-1-verify.md       # What the verification agent returned
  return-stream-1-remediate.md    # What the remediation agent returned (if dispatched)
  stream-1.log                  # fallback subprocess output, if used
docs/plans/.dominion-audit/{slug}/
  {snapshotId}/review.md         # retained independent review/cleanup report
  {snapshotId}/security.md       # retained links to external security run artifacts
  {snapshotId}/manifest.json     # shared immutable input manifest
  finalization.json             # retained commit/push/cleanup receipt
```

Create the plan-owned temporary directory at dominion start and record its path in status. Dominion writes briefing packets and collects agent returns there. Final evidence goes in the separate retained audit directory. Follow the installed `stream` skill's `references/plan-lifecycle.md` for ownership, legacy artifact migration, expiry, and cleanup.

### Retry handling

If a primary is retried, suffix the previous return file:
```
return-stream-3-primary.md → return-stream-3-primary.attempt-1.md
```

### Cleanup

The `final` stream owns temporary-log cleanup. It inventories paths while reviewing, then the sole finalization owner deletes them after both final gates and the selected mode's finalization succeed. Retain final reports, manifests, baseline evidence, the cleanup receipt, and external security outputs. Do not leave completed-run logs for the user to delete manually. Codex handoffs keep temporary logs until actual Codex finalization; failed or active runs keep them unless the separate three-day plan-retention policy retires an abandoned plan. Verify removal and report unresolved or ambiguous legacy paths.

---

## Status File as Coordination Layer

Dominion is the single status writer. It records claims before dispatch, checks actual worker artifacts and returned evidence, and serializes completion/verification/final-gate updates. Workers read status and return data; they never overwrite shared JSON. Exclusive artifact directories prevent sibling write collisions; security workers retain upstream external output and scratch isolation.

Use the installed `stream` skill's `references/status-schema.md` Concurrency rules for atomic updates and manual-session locks. Read/check/write without a lock is not safe even when sessions target different JSON keys. The finalization owner is unique and cannot act before both snapshot-bound reports pass.

---

## Resumability

`/dominion` is fully resumable. If the session closes or dominion is interrupted:

1. Status file preserves all progress
2. Running `/dominion` again reads the status file
3. Already-completed implementation streams are skipped; normalize legacy final state and revalidate both final reports against the current snapshot before trusting completion
4. `in_progress` streams are flagged (user decides: wait or take over)
5. Pending streams with met dependencies are re-dispatched
6. Dominion re-builds briefing packets on resume; retained plan/status and audit evidence remain the source of truth
7. A retained finalization receipt resumes pending cleanup without replaying a completed commit/push, even if plan/status removal was interrupted

Resumption is available while the plan is retained. Discovery prunes plans older than three days unless they are explicitly selected for continuation or have confirmed live owners.

---

## Dry Run Mode

`/dominion --dry-run` shows the full execution plan without dispatching anything:

```
Dry run for: docs/plans/2026-04-22-feature-overhaul.md

Phase 1 (sequential):
  → Stream 1: Foundation — 1 primary, 1 verifier

Phase 2 (parallel, after Stream 1):
  → Stream 2: Financial Ops — legion (T:2 → I:2 → D:1) — run sequentially within primary
  → Stream 3: Contract & Sales — legion (T:3 → I:3) — run sequentially within primary
  → Stream 4: Admin UI — solo

Phase 3 (parallel, after Streams 2-4):
  → Stream 5: Integration — legion (T:2 → I:2 → D:2) — run sequentially within primary
  → Stream 6: Polish — solo

Phase 4 (after all implementation has settled):
  → Final Validation / Cleanup + Final Security Audit — concurrent read-only passes
  → Join → one remediation owner → fresh passing reports → authorized finalization

Agent count: implementation primary/verify/remediate roles + 2 final readers
Follow-up budget: bounded remediation/re-review under the existing cap
Max concurrent: {effective worker limit} workers ({runtime/config source}); fill available slots, broker independent reviews
```

---

## Fallback: Headless Subprocess Mode (Deprecated, On-Demand Only)

The historic `claude -p` headless mechanism remains available for users who explicitly request it or in runtimes where the Agent tool is unavailable. Do NOT use by default.

```bash
cd {project_root} && claude -p "/stream {plan_file} --claim {stream_number}" \
  --model sonnet \
  --allowedTools "Bash,Read,Write,Edit,Glob,Grep,Skill,Agent" \
  < /dev/null > docs/plans/.dominion-logs/{slug}/stream-{id}.log 2>&1
```

**Known issue:** headless streams that run `pnpm test <files> | tail -N` as their verification gate have been observed to hang indefinitely when a new test file in the stream's own diff leaks a timer/promise/connection. `tail` buffers until EOF; if node never exits, the pipeline never unblocks. This was the original motivation for moving to Agent-tool bg-mode.

If forced to use headless, ensure the spawned `/stream` uses `pnpm exec vitest run` rather than `pnpm test | tail -N`.

---

## Rules

1. **ALWAYS** show the execution preview and proceed in auto mode; require a routine confirmation only when the user explicitly requested that checkpoint
2. **ALWAYS** pre-compute domain briefing packets and include mandatory first-step worker quality skill loads with resolved paths and returned evidence
3. **ALWAYS** use the runtime's native agent facility (Claude Agent with `run_in_background: true`; Codex collaboration agents). Never use headless `claude -p` by default
4. Fill actual host capacity with eligible work and refill on each completion; broker independent reviews and reserve only the concrete capacity required by the shared contract, including fresh-review slots in an unreleasable open-thread pool
5. **ALWAYS** run verification (2.3.5) for EVERY stream — no trusted streams
6. **ALWAYS** run remediation (2.4) if verification finds anything, even quality-only
7. **ALWAYS** run a lightweight remediation with Input 3 (free audit) even when verification finds nothing — it's Layer 4 insurance
8. **ALWAYS** serialize shared status writes in dominion; workers return data and write only inside exclusive artifact roots
9. Keep the primary/verification/remediation lifecycle and at most one surgical follow-up; bounded task helpers and required independent reviewers are separately accounted but share the host's concurrency pool
10. **NEVER** pipe `pnpm test` output through `| tail` / `| head` / `| grep` in agent prompts — the pipe hangs on leaky teardown
11. **ALWAYS** load `auto-web-validation` into dominion's own context before any web research or vendor/library lookup
12. In either final mode, run `final` and `final-security` together, join/fix/recheck, and require both to pass on the current snapshot. Codex mode then preserves artifacts for `/verify`
13. Use Claude completion notifications or Codex's native wait/status tools; collect results and refill available slots without shell polling or scheduled wakeups
14. `final` owns verified temporary-log deletion after joined finalization; retain final evidence and the receipt outside `.dominion-logs`. Apply automatic three-day plan retention during discovery
15. **PIPELINE implementation only on primary completion, not on phase settle.** Both reserved final streams wait for full implementation settle. The moment a primary lands, enqueue its verifier and all newly eligible downstream primaries, then fill available slots with priority for required verification. Lack of capacity queues work; upstream verification does not add a dependency barrier.
16. **Do NOT gate a downstream on an upstream's verification/remediation — even on a shared file.** Overlap is safe: verifiers are read-only, and file ownership passes to the downstream on dispatch (an upstream remediator that finds an issue in a handed-off file REPORTS it downstream via 2.5, it does not edit it). The only hard serialization is **two PRIMARY agents editing the same file in the same wave** — the plan's file-ownership matrix sequences those. Over-gating on "shared file + still settling" throws away the pipeline win; don't.
17. **Route late upstream remediation deltas into the affected downstream's remediation** (Input 2). Pipelining trades a possible late contract shift for wall-clock; this is how that shift gets reconciled instead of lost.

## Rationalization Prevention

| You're thinking...                                                                 | Reality                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "I'll just spawn `claude -p` like before, Agent tool is extra work"                | Agent tool is strictly simpler: no stdio plumbing, no log parsing, no `while ps -p` polling, no `| tail` hangs. The return value lands directly in your context. The only reason to use `claude -p` is fallback. |
| "The primary agent's summary looks clean, I can skip verification"                 | Self-reports are unreliable. 6/7 Sonnet streams shipped material deviations while reporting "completed." Always run 2.3.5.                                                                                    |
| "Verification found nothing, I can skip remediation"                               | Run the lightweight Input-3-only remediation anyway. It's cheap and catches what both the primary's self-audit and the verifier missed. Three layers, not two.                                                |
| "Remediation failed — I'll dispatch another remediation agent"                     | Don't. At that point dominion has strictly more information than a fresh remediator. Handle inline, dispatch ONE surgical follow-up, or escalate. Never dispatch a second generic remediation.                |
| "I should make each agent load its own skills for clean context separation"        | Domain context is precomputed, but every worker must actually load its applicable quality adapters and selected references. Parent excerpts alone do not apply Ponytail or Humanizer in the worker context.                      |
| "I'll poll the status file to track agent progress"                                | Don't. Agent-tool `run_in_background: true` notifies automatically. Polling wastes cycles and breaks cache efficiency.                                                                                        |
| "This stream is taking too long, I'll kill the agent"                              | Trust the agent. If it's still running, it's still working. Agent tool notifies on completion — you'll hear when it's done.                                                                                   |
| "I'll let the primary self-certify — verification is just overhead"                | Verification catches the things the primary can't see (it's too close to its own work). The independent fresh-context read is the point.                                                                     |
| "The primary said it deferred X because it's out of scope — I should trust that"   | Run Input 1 through the remediation agent. The deferral may be legitimate; it may also be the primary punting on in-scope work. The remediator assesses.                                                      |
| "I must wait for the whole phase (verify + remediate) to settle before starting the next stream" | No. Pipeline on the dependency's **primary** completion. Most work is correct first-pass, so a downstream can build on shipped code while the upstream verifies in parallel. Only a shared-file overlap forces waiting (rule 16). |
| "Downstream D shares a file with upstream U, and U is still in verification/remediation — D must wait for U to settle" | No — that's the over-gating trap. Verifiers are read-only; U's remediator hands off (reports, doesn't edit) files D now owns. Waiting on settle throws away the pipeline win. Only two **primary** agents editing one file in the same wave serialize — and the file-ownership matrix already sequences those. |
| "Two sibling PRIMARIES both own the one 2000-line admin page, but pipelining says go fast, so launch both" | That IS the one case that serializes — two concurrent primaries blind-writing one file clobber. Honor the matrix's `3 → 7 → 4` sub-sequence. This is primary-vs-primary, distinct from (and not softened by) the read-only-verifier / ownership-handoff rules. |
| "The downstream already finished against the upstream's code — if the upstream is later remediated, that's the upstream's problem" | It's YOUR problem. A late upstream shape-change orphans the downstream that consumed the old shape. Route the delta into the downstream's remediation as an Input-2 finding (2.5). That reconciliation is the price of pipelining. |
