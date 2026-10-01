---
name: auto-security-quality
description: "Cloudflare's source-grounded security audit with independent finding validation. Use for Summon's security review path, requested full or module audits, and Dominion's concurrent final security review of frozen changes. Supports focused security guidance without starting a full audit."
---

# Automatic Security Quality

Read the preserved [upstream security audit](upstream/SECURITY-AUDIT.md) and its phase references. This wrapper supplies scope, output, and workflow coordination; it preserves the upstream evidence bar, execution safeguards, coverage ledger, independent verification, schema, and reporting. [UPSTREAM.md](UPSTREAM.md) records the source and license. Upstream references to its `SKILL.md` mean the renamed `upstream/SECURITY-AUDIT.md`; its bytes are unchanged and the filename prevents duplicate skill discovery.

## Resolve the invocation automatically

| Caller/request | Audit mode and scope |
|---|---|
| Summon's security-review path, entire codebase | Full audit workflow; entire repository |
| Summon's security-review path, named modules or paths | Full audit workflow; named scope plus source paths needed to trace its boundaries; remaining target explicitly out of scope |
| Dominion final security stream | Full audit workflow scoped to the frozen implementation diff, its affected boundaries, callers, and configuration |
| Explicit full audit or requested report artifacts | Full audit workflow using the requested scope |
| A security question, individual finding, or ordinary focused code review | Upstream guidance mode; no automatic report directory or six-phase audit |

Summon/Dominion's explicit dispatch is sufficient to select the corresponding audit mode. Do not ask the user to select a mode, output path, profile, companion files, subcommands, or confirm that the authorized review should begin. Use the request and plan; `standard` is the default profile. Select `quick` only for a requested bounded pass or small scoped recheck and disclose partial coverage. Respect an explicit `deep` profile or agent budget. Never invent a budget that creates an unnecessary permission checkpoint.

Use the current repository as the target, local Git/directory metadata as its name, and the upstream default external output root `~/security-audit-skill/<repo-name>/run-<N>` unless the caller supplied another permitted path. Select an unused run directory and create it exclusively; retry the next integer on a collision. Output remains outside the target unless an explicitly selected in-target directory meets upstream ignore checks. A failed default path is a concrete blocker for the caller, not permission to use an unsafe fallback. Resolve every upstream `<skill-dir>` to this skill's absolute `upstream` directory.

Select attack-class companions from the observed stack and trust boundaries. Reuse prior compatible evidence exactly as upstream requires. Missing live deployment facts remain `needs_validation`; do not ask broad setup questions or guess them. A strict budget that cannot fund mandatory independent checks yields an incomplete report under the upstream budget rules; the caller presents the exact limitation. Never silently lower the evidence bar.

## Review source; separate fixes

The audit and its hunters/verifiers read target source only. They may write only their assigned run artifacts under upstream write isolation. The audit coordinator alone writes shared run files; each delegated worker gets a distinct scratch directory. Use real fresh agents for candidate and final verification, scheduling bounded batches or sequential independent agents when concurrent slots are limited. Never relabel the hunter's self-review as independent verification. If independent verification is unavailable, retain an unresolved candidate and mark the run incomplete as upstream requires.

Every reconnaissance, hunter, critic, verifier, and remediation prompt includes resolved required-skill paths and an explicit first step: load `auto-code-quality` before reading, reviewing, or editing code, and load `auto-writing-quality` before writing prose, including findings and reports. Include this wrapper plus the relevant upstream phase/companion references. Workers return the loaded entrypoints/references and the work they governed; the coordinator checks actual read/tool evidence where available. Parent loading is insufficient. Apply the shared [quality-routing contract](../auto-workflow/references/quality-routing.md), returning missed loads for a fresh affected-work check before accepting results.

Source inspection may proceed without target execution. Target-controlled tests, builds, fixture processing, browsers, or fuzzers require **all** upstream OS-enforced sandbox controls: no external network, an allowlisted empty environment, read-only source/toolchain, scratch-only writes, and resource/time limits. Ordinary tool permissions, a worktree, or a promise to be careful are not that sandbox. If those controls are unavailable, perform source review, record the exact local-validation blocker, and give a safe validation plan. Never probe deployed endpoints or fetch target dependencies during the audit.

Keep each finding source-grounded: lower-trust principal, real boundary, reachable path, affected principal/resource, and bounded result. Best-practice concerns alone are hardening notes. Only independently validated `confirmed` records get severity; `needs_validation` never does. A clean report means no confirmed findings within stated coverage, not a secure-codebase guarantee.

The calling Summon/Dominion agent owns human-facing presentation and authorized remediation. It loads `auto-chat-quality`, `auto-code-quality`, and `auto-writing-quality` as appropriate, presents verified findings, and applies narrow in-scope fixes when the user requested review-and-fix or the implementation plan already authorizes them. This is a separate implementation step after the audit releases its source snapshot. Review-only requests remain review-only. Revalidate each repaired boundary independently, preserve pre-fix evidence, and report remaining blockers without calling unverified repairs complete.

## Concurrent final stream contract

Run Dominion's security stream beside final functional/quality verification after implementation streams finish. Both reviewers receive the **same frozen source state** and separate artifact directories:

1. The coordinator supplies the planned base ref, reviewed commit or snapshot identity, exact changed-path inventory (including relevant untracked/generated files), and a content fingerprint of the frozen source. An immutable read-only copy or a coordinator-enforced freeze is acceptable for source review; target execution still needs the OS sandbox above. Do not guess that `HEAD^` represents the plan's changes.
2. Record this identity, `scope_paths`, and the base ref in run metadata. Trace security effects beyond changed lines where needed; unchanged surrounding code can prevent or complete a boundary path. Clearly exclude the rest of the repository from coverage claims.
3. Both final streams are read-only against that source. Functional test output also belongs in isolated output/scratch. Neither stream applies fixes, reformats files, installs dependencies, or updates shared plan state. The audit writes only its separate report directory and returns its paths and findings to Dominion.
4. Before accepting either result, compare the reviewed content fingerprint with the current source. If anything relevant changed during review, invalidate affected evidence and review the new frozen state; do not report the old result as current.
5. Dominion collects both results, assigns fixes to the implementation owner, and serializes overlapping changes. After fixes, freeze the new state and rerun the affected functional checks and independent security validation. Preserve the previous report as evidence; update current findings and source identity in a new run. Completion requires both streams' current results or an explicit incomplete/blocker report.

## Validate and hand off

Run both preserved validators after writing final structured output:

```sh
node "<absolute-skill-dir>/upstream/validate-findings.cjs" "<output-dir>/findings.json"
node "<absolute-skill-dir>/upstream/validate-coverage-ledger.cjs" "<output-dir>/coverage-ledger.json"
```

Complete all six upstream phases or record the exact incomplete terminal state. Return the reviewed source identity, profile/scope, independently validated findings, required report paths (`REPORT.md`, `FINDINGS-DETAIL.md`, `NEEDS-VALIDATION.md`, `findings.json`, `coverage-ledger.json`, `run-metadata.json`), validator results, and unresolved candidates/coverage limits. `auto-writing-quality` may improve report prose but must preserve every structured verdict, severity, evidence claim, and uncertainty.
