---
name: plan-validate
description: "Validate a multi-stream implementation plan before /stream or /dominion execution. Checks that stream structure, dependencies, file ownership, required skills, verification strategy, and final validation mode are present and coherent. Use when reviewing a plan file, before execution handoff, or when plans may have been hand-edited."
---

# Plan Validation

## Overview

This skill validates the plan itself before implementation starts.

It exists because the summon/stream/dominion workflow increasingly depends on plan metadata being correct. A good-looking plan that is structurally incomplete can cause subtle downstream failures:
- streams that cannot be claimed correctly
- false dependencies that kill parallelism
- overlapping file ownership
- final validation mode missing
- no clear verification story

## When to Use

- Before `/stream` or `/dominion` on any multi-stream plan
- When a plan was edited manually after generation
- When a plan was written outside the normal `/summon` workflow
- When the user says "validate the plan", "check this plan", or "is this plan executable?"

## What to Validate

Load `auto-chat-quality` and `auto-writing-quality` before producing findings; load `auto-code-quality` when reviewing referenced code. Resolve the installed skill on Codex and read `SKILL.md`, or use Skill on Claude. Apply the installed `auto-workflow` skill's `references/quality-routing.md`. In auto mode, repair supported metadata gaps directly within the authorized plan scope, report amendments, and continue; do not add routine approval questions. Explicit review-only/manual requests remain controlling.

### 1. Stream Structure

Check that:
- stream headers parse cleanly
- stream IDs are unique
- sub-streams are named consistently
- each stream has a clear title and task scope

### 2. Dependencies

Check that:
- every dependency refers to an existing stream
- no dependency cycles exist
- dependencies are real, not just conceptual ordering
- the critical path is not obviously over-constrained

When a dependency looks fake, ask:
"What file, type, endpoint, schema, or migration does this stream actually need from the other one?"

If you cannot name the artifact, flag it.

### 3. File Ownership

Check that:
- each stream has owned files listed
- ownership is concrete enough to be actionable
- shared files are either additive-only or explicitly sequenced
- no two parallel streams mutate the same file without coordination

### 4. Required Skills

Check that:
- `## Required Skills` exists for multi-stream plans
- baseline skills are present
- per-stream skills look plausible for the work described
- skills are not obviously missing for security, data, API, UI, or test work
- the automatic quality floor applies even if the plan predates the new skills: `auto-chat-quality`, `auto-code-quality` for code/review, `auto-writing-quality` for human prose, and `auto-design-quality` for new/changed design beyond unchanged established reuse
- worker prompts require their own actual skill loads and evidence; parent excerpts are insufficient

### 5. Verification Story

Each stream should have an obvious verification surface:
- tests to add or update
- type/build/lint expectations
- smoke-test expectations for routes/pages/endpoints

If a stream has no clear verification story, flag it.

### 6. Final Validation Mode

Check that the plan records:

```markdown
## Final Validation Mode
Mode: codex
```

or

```markdown
## Final Validation Mode
Mode: review
```

If missing, use the existing `review` default and record it before execution, unless the user has explicitly selected `codex`.

### 7. Final Review and Security Siblings

Confirm that the execution contract injects reserved `final` and `final-security` IDs. Plan authors need not add stream headers for them. Both depend on every implementation stream, excluding both final IDs; neither depends on the other. They start only after implementation verification/remediation settles, not on primary completion alone.

Use the installed `stream` skill's `references/status-schema.md` as the canonical contract. Require a stable pre-implementation baseline; a frozen snapshot including committed, staged, unstaged, and untracked source changes; two concurrent read-only reviewers with separate reports; a single coordinator for status; and one remediation owner after joining the reports. Both passes need evidence for the current snapshot before review-mode commit/push/cleanup or Codex handoff. `auto-security-quality` is mandatory for the security sibling. Check that legacy status migration is idempotent and preserves progress while adding the missing gate.

Flag same-file parallel mutations, reviewers committing/cleaning independently, bare `git diff HEAD` as whole-plan coverage, a final-only skill override that bypasses quality routing, and missing re-review after fixes as real execution gaps.

## Output Format

Report in three buckets:

### Executable
- Things that are structurally correct

### Gaps
- Missing sections, invalid dependencies, ambiguous ownership, missing verification

### Recommended Amendments
- Concrete edits to the plan before execution begins

If the plan is execution-ready, say so explicitly.

## Rules

- Be strict about structure, not verbose about theory
- Prefer concrete amendments over abstract criticism
- Flag false dependencies aggressively
- Treat missing verification and missing final validation mode as real issues
