# Plan retention and Dominion cleanup

Shared contract for summon, dominion, stream, and verify. Read before inspecting `docs/plans/`, creating a plan, or finalizing execution. Run housekeeping automatically within the workflow; do not add an approval question. An explicit read-only request or `--dry-run` reports eligible cleanup without deleting anything.

## Delete plans older than three days

Before discovering candidates or writing a new plan, prune expired plans from `docs/plans/`:

1. Identify top-level plan Markdown files by their generated date-prefixed name, matching status companion, or actual plan structure. Leave README/index/reference documents and nested artifact directories alone. A plan expires when its filesystem modification time is **strictly more than 72 hours** before the current time. Use the plan's mtime, not its filename date or a recent status write.
2. Protect a plan explicitly selected by the current request/continuation, and plans with confirmed live workers or a live owner/lock. A stale `in_progress` status alone is not evidence of live activity. Check recorded worker IDs/locks against the runtime; no owner and no live runtime evidence permits retirement, but an existing owner/lock that cannot be checked requires deferral. Do not prune while another session's frozen-review barrier covers these files.
3. Under the existing repository coordination lock, recheck age and ownership. Preserve/migrate final evidence and references using the artifact rules below while the plan/status still exist, and write an `expired` receipt with cleanup pending. Remove and verify that plan's attributable temporary logs, then delete the eligible plan and its exact same-slug `.status.json` companion last. Record remaining ambiguous paths separately; they do not prevent retiring the identified plan. This also retires abandoned/incomplete plans; it does not mark their work complete or their reviews passed. Do not commit or push merely to prune plans.
4. Discover plans from the remaining files. Git history is only a hint and must not resurrect a pruned path. Committed versions remain in Git history; do not claim Git can recover untracked or uncommitted content. Report pruned plans and any deferred paths briefly.

Use exact, contained paths and never follow symlinked plan/artifact directories outside the repository. An absent plan directory needs no work. Repeating pruning must be harmless. A plan currently protected by an explicit handoff can be pruned on a later discovery once that protection no longer applies.

## Separate temporary logs from final evidence

- Write new briefing packets, worker returns/retries, and fallback process logs under `docs/plans/.dominion-logs/{slug}/`. Record this plan-owned directory in status so another plan's files cannot be mistaken for cleanup targets.
- Keep final reports, snapshot manifests, baseline evidence, and `finalization.json` under `docs/plans/.dominion-audit/{slug}/`. Use `{snapshotId}/` for snapshot-specific reports. Keep the security audit's external run directory intact; the retained audit directory stores its returned paths and linking summary.
- For a Codex handoff, record `codexValidation: pending` in that receipt and change it to `completed` only after the actual Codex pass. An older Codex receipt without this evidence remains pending. Discovery treats pending validation/finalization/cleanup as active work even when both final siblings say `completed`; an expired or fully finalized receipt does not reactivate its plan.
- If interruption removed plan/status before cleanup was recorded complete, resume directly from the retained receipt's exact pending paths once it proves required validation and Git finalization already finished. Do not recreate the plan or replay those Git operations. An expiry receipt resumes retirement cleanup without claiming successful implementation.
- Before deleting legacy logs, copy any final reports/manifests/baseline/receipt they contain into the retained audit directory, verify the copies, and update status/report/receipt references. Never overwrite different evidence or delete the only copy. Preserve paths referenced by the retained evidence, or relocate those dependencies and update their links too.
- Old flat `briefing-stream-*`, `return-stream-*`, and `stream-*.log` files may be removed only when their contents or recorded run metadata identify the retiring plan. Stream numbers alone are insufficient. Leave ambiguous files and other plans' directories in place and report them; never delete the whole shared log root. Remove that root only if empty.

## Final cleanup belongs to `final`

The final review/cleanup stream inventories the plan's temporary artifacts and returns the exact cleanup paths with its report. Its concurrent review phase stays read-only. After both final reviewers and all other workers have stopped, the coordinator performs or resumes `final` as the **sole finalization owner** to execute this cleanup:

1. Require both final reports to pass on the same current snapshot and all required project checks to pass. In `review` mode, finish the existing authorized commit/push steps first. In `codex` mode, preserve logs and handoff evidence until the actual Codex validation and authorized finalization finish; an initial Claude cleanup pass is not completion. A failed check, unresolved finding, active writer, failed push, or pending handoff keeps the logs available.
2. Preserve final evidence and consult the retained `finalization.json` receipt. Record the cleanup paths and pending cleanup state before deleting anything. The receipt retains the existing owner, snapshot, commit, and push state. Do not repeat a recorded commit/push when resuming interrupted cleanup.
3. Delete only this plan's temporary briefing packets, worker returns/retries, process logs, and empty temporary directories. Keep final evidence and external security output. Apply the existing mode-specific plan/status cleanup only at its authorized completion point; remove status last so an interrupted run can still locate its receipt.
4. Check that every selected temporary path is absent, then mark cleanup completed in the retained receipt and report the result. Already-missing paths count as removed. On failure, keep cleanup pending, record the exact remaining paths, and retry only those paths on resume; do not claim successful cleanup.

Record plan/status and generated artifact paths as housekeeping exclusions **before** freezing the review snapshot. Their expected removal does not change reviewed source. A source change still invalidates review evidence. Age-based retirement of an abandoned plan uses the same path ownership and evidence-preservation rules, but records `expired`, never a successful finalization.
