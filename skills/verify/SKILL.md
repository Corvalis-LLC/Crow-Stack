---
name: verify
description: "Plan-aware Codex verification and refinement. If an active plan has a status file, perform findings-first implementation validation. If a plan exists without a status file, treat /verify as a Codex plan-refinement pass before execution. User-invocable via /verify command."
---

# /verify — Plan-Aware Codex Validation

<command-name>verify</command-name>

## Overview

`/verify` is the Codex-side validation pass the user can invoke directly.

It has two modes:

- **Plan refinement mode**: when a plan file exists but no status file exists yet
- **Implementation validation mode**: when an active plan has a status file, or when no plan exists and review falls back to the current working tree

Use this when the user wants the stronger manual validation style rather than a lightweight diff review.

## Behavior

Default behavior: **begin review immediately**. Do not ask setup questions unless multiple active plans make auto-resolution ambiguous.

Immediately load `auto-chat-quality`, `auto-writing-quality`, and `auto-workflow` with its `references/quality-routing.md`; load `auto-code-quality` before any code inspection/review/fix. Use Skill on Claude or resolve/read the installed `SKILL.md` on Codex, with relative references resolved from that skill directory. New/changed design loads `auto-design-quality`; plain unchanged established reuse is exempt. Each delegated reviewer/fixer must actually load applicable skills in its own context and return evidence. Missing loads require a fresh audit before accepting output.

In auto mode, fix supported in-scope findings without asking for routine confirmation; honor explicit review-only/manual scope and actual permission boundaries. Skill subcommands and available checks are executed by the agent, not assigned back to the user.

## Step 1: Resolve Context

### Prefer the active plan when one exists

Resolve in this order:

1. Current, unarchived `docs/plans/*.status.json` companions with incomplete streams or migration-needed/missing/stale final-pair evidence, including legacy all-completed status; honor explicit continuation context and do not reactivate a recorded finalized/archive plan merely because its schema is old
2. Most recent `docs/plans/*.md`
3. If no plan exists, review the current git working tree

Assess candidates read-only before selecting; normalize only the selected safe companion, never all discovered archived plans.

If exactly one active plan exists:
- read the plan
- read the status file
- identify completed, in-progress, and final-validation state
- normalize legacy status using the installed `stream` skill's `references/status-schema.md`; preserve implementation progress, stable baseline, and user-owned work
- enter **implementation validation mode**

If no status file exists but exactly one recent plan exists:
- read the plan
- assume the user wants **Codex plan refinement before execution**
- do not default to code review yet

If multiple active plans exist:
- show the candidates and ask the user which one to verify

If no active plan exists:
- fall back to git-based review of current changes

## Step 2: Gather Structured Repo Context First

Before deep review, attempt to gather structured codebase context via `corvalis-recon`.

Use the same binary path summon relies on:
- macOS/Linux: `~/.claude/bin/corvalis-recon`
- Windows: `%USERPROFILE%\.claude\bin\corvalis-recon.exe`

The human-facing shell alias `recon` may also exist, but verify should prefer the explicit binary path when checking availability.

Procedure:
1. Check whether the recon binary exists
2. If present, run it before broad `Glob`/`Grep`/`Read` exploration:
   - plan refinement mode: `~/.claude/bin/corvalis-recon analyze --root <project_root> --format json --mode planning`
   - implementation validation mode with an active diff: prefer `~/.claude/bin/corvalis-recon analyze --root <project_root> --format json --mode planning --diff <git_range>`
     - examples: `--diff HEAD` for the working tree vs current commit, `--diff main...HEAD` for a branch comparison
   - for larger repositories, add `--budget 8000`
3. Validate that the JSON parses and includes `version`, `planning`, `dependencies`, and `summary`
4. Use recon output to orient the verification pass:
   - entry points and dependency hubs for architectural context
   - hotspots for likely bug concentration
   - symbols and dependencies for cross-file impact review
   - project overview for language and surface-area awareness
5. If recon is unavailable or invalid, continue with normal repository inspection

Hard rule: do not make negative existence claims about files, subsystems, or code paths until recon has been checked when available, and direct repository search has been used when recon is unavailable or inconclusive.

Recon is for context gathering, not verdict replacement. Still inspect the real changed surface, adjacent consumers, and tests directly.

## Step 3A: Plan Refinement Mode

If a plan exists without a status file, treat `/verify` as a pre-execution Codex refinement pass.

Load and follow `codex-plan-refinement`.

Review the plan using the refinement angles from `codex-plan-refinement`.

Output format in plan refinement mode:

1. **Refinement Findings**
2. **Recommended Plan Amendments**
3. **Compression Opportunities**
3. **Short Handoff Readiness Summary**

If the plan is already strong, say so explicitly.

## Step 3B: Implementation Validation Mode

For an active implementation plan, validate the two sibling final reports before treating prior completion as current. If implementation is still running, review only the requested settled scope and do not claim final completion. Once all implementation verification/remediation has settled, resume the joined final protocol from the installed `stream` skill's `references/status-schema.md` if either report is absent, stale, incomplete, or from another snapshot. Run the code/cleanup and `auto-security-quality` passes against the same frozen full-change snapshot, with separate reports and no parallel source mutations. This is automatic, including for legacy status files with only `final`.

For a Codex handoff, the earlier Claude Cleanup and security reports are inputs, not a substitute for this Codex review. If Codex review causes fixes, refresh the affected project checks and both snapshot-bound final reports before completion or any authorized commit/push/cleanup.

Run the project verification suite before producing review findings.

Minimum:
- type check / lint
- relevant tests
- build when applicable

If checks fail:
- note the failures
- continue the review, but treat verification failures as top-priority findings

## Step 4: Read the Real Change Surface

Skip this step in plan refinement mode unless the plan references existing code that must be inspected for feasibility.

In implementation validation mode, inspect:
- the stable baseline-to-current change set, including committed, staged, unstaged, and untracked source changes; `git diff HEAD` alone is insufficient for an active plan
- changed file list
- the plan's owned files for the current or final stream when available
- adjacent consumers, shared types, tests, and related services
- the user-facing surfaces that expose the changed behavior: routes, pages, components, menus, forms, settings, buttons, empty states, and success/error states

Do not review the diff in isolation.

Always look for:
- missing co-changes
- duplicated logic that should already be extracted
- pure logic that should have direct tests
- route/page files doing domain work that belongs in helpers or services
- implemented functionality that is not actually reachable, visible, or understandable in the UI/UX

## Step 5: Review Using Senior Standards

Prioritize these categories:

1. Correctness and business logic
2. Cross-file impact
3. Edge cases and error handling
4. Test gaps and testability seams
5. Duplication and extraction opportunities
6. UI/UX completeness for the implemented behavior
7. Maintainability and architectural drift

Specific patterns to reward:
- thin routes/pages, richer helpers/services
- reusable factories for repeated validation/payload/workflow logic
- pure pricing/validation/domain helpers with focused tests
- functionality that is actually surfaced through the necessary UI/UX path, with affordances users can discover and understand
- findings-first review instead of “looks good” summaries
- explicit verification after changes

Specific patterns to flag:
- repeated inline `if` ladders that should be rule-driven
- large handlers/pages accumulating domain logic
- cross-file contract changes without dependent updates
- extracted helpers without direct tests
- backend or domain functionality that ships without the UI controls, page wiring, navigation entry points, loading states, empty states, or result messaging needed for a user to use it
- “works but is calcifying” structure in growing files

UI/UX completeness check:
- If functionality was added or changed, verify that the user can reach it from the intended product surface
- Verify that the surface communicates state clearly: loading, empty, success, error, disabled, and permission states when applicable
- Verify that labels, affordances, and placement make the feature discoverable enough for its intended audience
- If the implementation obviously requires a small UI/UX layer to expose existing functionality and that surface is missing, treat that as a real validation gap rather than an optional polish note

## Step 6: Report Format

Report in this order:

1. **Findings** — ordered by severity with file references
2. **Open questions / assumptions**
3. **Short summary**

If no findings exist, say so explicitly and mention residual risk if any.

## Step 7: Fix Within the Authorized Scope

In auto mode or when fixes have already been requested:
- fix supported in-scope issues directly; preserve explicit review-only limits
- prefer extracting reusable factories/helpers/workflows instead of patching around duplication
- add focused tests for pure logic that was extracted or materially changed
- if the missing piece is the UI/UX needed to expose already-implemented functionality, implement that surface as part of the fix when the intended behavior is reasonably inferable from the plan, surrounding code, or product structure
- re-run verification

## Step 8: Close Cleanly

Before closing:
- re-run checks after fixes
- for an active plan, confirm both final reports pass on the same current snapshot; stale or incomplete security evidence blocks final completion
- confirm whether the tree is verification-clean
- summarize what was fixed and what remains, if anything

## Rules

- Start reviewing immediately when invoked
- Prefer active-plan awareness over blind diff review
- If a plan exists without a status file, default to plan refinement mode
- Verify before opinion whenever possible
- Findings first, summary second
- Review surrounding context, not only changed lines
- Treat testability as part of code quality, not an optional add-on
- Treat missing UI/UX exposure for implemented functionality as a product correctness issue, not mere polish
- Prefer extraction over repeated review comments when the same pattern appears in 3+ places
