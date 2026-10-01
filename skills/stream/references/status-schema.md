# Status File Schema

> Reference for `/stream` skill. See SKILL.md for overview.

## Location

Status files are companions to plan files:

```
docs/plans/2026-02-13-vendmaster-feature-parity.md           # Plan (read-only)
docs/plans/2026-02-13-vendmaster-feature-parity.status.json  # Status (managed by /stream)
```

The slug is derived from the plan filename by removing the `.md` extension.

## Schema

```json
{
  "schemaVersion": 2,
  "planFile": "docs/plans/2026-02-13-vendmaster-feature-parity.md",
  "finalValidationMode": "codex",
  "createdAt": "2026-02-13T10:00:00Z",
  "updatedAt": "2026-02-16T14:30:00Z",
  "streams": {
    "1": {
      "name": "Foundation — Universal Notes + Activity Log UI",
      "status": "completed",
      "dependencies": [],
      "baselineSkills": ["auto-typescript", "auto-database", "auto-evolution"],
      "claimedAt": "2026-02-13T10:05:00Z",
      "completedAt": "2026-02-13T12:30:00Z",
      "verification": {
        "typeCheck": true,
        "tests": true,
        "build": true,
        "timestamp": "2026-02-13T12:28:00Z"
      }
    },
    "2": {
      "name": "Financial Operations — Collections + Owners",
      "status": "in_progress",
      "dependencies": ["1"],
      "baselineSkills": ["auto-typescript", "auto-compliance", "auto-serialization", "auto-security"],
      "claimedAt": "2026-02-14T09:00:00Z",
      "completedAt": null,
      "verification": null
    },
    "3": {
      "name": "Contract & Sales Workflows",
      "status": "pending",
      "dependencies": ["1"],
      "baselineSkills": ["auto-typescript", "auto-api-design", "auto-resilience", "auto-state-machines"],
      "claimedAt": null,
      "completedAt": null,
      "verification": null
    }
  }
}
```

## Field Reference

### Top-level

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | number | `2`; missing versions are migrated as described below |
| `reviewBaseline` | object | Stable starting commit, initial dirty-file manifest, and scope/coverage notes; recorded before implementation |
| `finalGate` | object | Shared final-review snapshot, coordinator, report paths, and join state |
| `planFile` | string | Relative path to the plan markdown file |
| `finalValidationMode` | enum | `codex` or `review` — selects the review/cleanup sibling; the security sibling always runs |
| `createdAt` | ISO 8601 | When the status file was first created |
| `updatedAt` | ISO 8601 | Last modification timestamp |
| `streams` | object | Map of stream ID → stream status object |

### Stream object

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Stream title from plan header |
| `status` | enum | `pending` \| `in_progress` \| `completed` |
| `dependencies` | string[] | Stream IDs this stream depends on |
| `baselineSkills` | string[] | Declared domain skills plus automatic quality floor; absence/empty does not disable routing |
| `skillLoads` | object[] | Per-worker actual skill load evidence: skill name, resolved path, method (`Skill` or `read`), role, and loaded references |
| `reviewEvidence` | object \| null | Final sibling report path, snapshot ID, result (`pass`, `findings`, `incomplete`), coverage, and open findings |
| `claimedBy` | string \| null | Session/worker identity recorded by the coordinator or lock owner |
| `settledAt` | ISO 8601 \| null | Independent verification/remediation/late-delta work finished; required for implementation dependencies of final siblings |
| `claimedAt` | ISO 8601 \| null | When this stream was claimed by a session |
| `completedAt` | ISO 8601 \| null | When this stream was marked complete |
| `verification` | object \| null | Verification results (see below) |

### Verification object

| Field | Type | Description |
|-------|------|-------------|
| `typeCheck` | boolean | `npm run check` passed |
| `tests` | boolean | Relevant tests passed |
| `build` | boolean | `npm run build` passed |
| `timestamp` | ISO 8601 | When verification was run |

### Legion object (optional)

Present only on streams with `**Legion:** Yes` annotation in the plan.

| Field | Type | Description |
|-------|------|-------------|
| `enabled` | boolean | Whether legion mode is active (false = fell back to solo) |
| `fallbackReason` | string \| null | Why legion was disabled, if applicable |
| `currentWave` | string \| null | Active wave type: `"T"`, `"I"`, `"D"`, `"R"`, or null |
| `waves` | object | Map of wave type → wave status object |

### Wave status object

| Field | Type | Description |
|-------|------|-------------|
| `status` | enum | `pending` \| `in_progress` \| `completed` |
| `agents` | array | List of agent task objects |
| `verification` | object \| null | Post-wave verification results |

### Agent task object

| Field | Type | Description |
|-------|------|-------------|
| `task` | string | Short description of what this agent does |
| `files` | string[] | File paths this agent owns |
| `status` | enum | `pending` \| `in_progress` \| `completed` \| `failed` |
| `retries` | number | Number of retry attempts (max 2) |

Example:

```json
"legion": {
  "enabled": true,
  "fallbackReason": null,
  "currentWave": "I",
  "waves": {
    "T": {
      "status": "completed",
      "agents": [
        { "task": "Write capacity service tests", "files": ["src/lib/server/services/capacity.test.ts"], "status": "completed", "retries": 0 },
        { "task": "Write registration API tests", "files": ["src/routes/api/registrations/+server.test.ts"], "status": "completed", "retries": 0 }
      ],
      "verification": { "typeCheck": true, "tests": false, "build": true, "timestamp": "2026-03-31T..." }
    },
    "I": {
      "status": "in_progress",
      "agents": [
        { "task": "Implement capacity service", "files": ["src/lib/server/services/capacity.ts"], "status": "completed", "retries": 0 },
        { "task": "Implement registration endpoint", "files": ["src/routes/api/registrations/+server.ts"], "status": "in_progress", "retries": 0 }
      ],
      "verification": null
    }
  }
}
```

## Status Transitions

```
pending → in_progress    (stream claimed by a session)
in_progress → completed  (verification gate passed)
in_progress → pending    (session abandoned, stream unclaimed)
```

## Stream ID Format

Stream IDs match what appears in the plan headers:
- Simple: `"1"`, `"2"`, `"3"`
- Sub-streams: `"4A"`, `"4B"`, `"5A"`
- **Special:** `"final"` — auto-injected Final Validation (`review`) or Final Cleanup (`codex`)
- **Special:** `"final-security"` — auto-injected Final Security Audit using `auto-security-quality`

Sub-streams are tracked as independent entries. The parent stream (e.g., "4") is not tracked separately — only the sub-streams appear in the status file.

### Two Final Sibling Streams

Both `final` and `final-security` are injected automatically; neither needs a plan header. Their dependency arrays contain **all implementation stream IDs, excluding both reserved final IDs**. Neither final depends on its sibling. Both are eligible only after implementation primaries, verification, remediation, and late cross-stream deltas have fully settled; primary completion alone is insufficient.

`final` uses the existing review/cleanup mode. `final-security` uses `auto-security-quality` to audit the changes and their surrounding trust boundaries. Neither reviewer may edit source, update shared status, remediate findings, commit, push, delete artifacts, or hand off early. They run concurrently against one frozen snapshot and write to exclusive report/artifact directories. The coordinator joins their results and owns remediation and finalization.

Example (inside `streams`):

```json
{
  "final": {
    "name": "Final Validation",
    "status": "pending",
    "dependencies": ["1", "2", "3"],
    "baselineSkills": ["auto-chat-quality", "auto-code-quality", "auto-writing-quality", "review"],
    "claimedAt": null,
    "completedAt": null,
    "verification": null,
    "reviewEvidence": null
  },
  "final-security": {
    "name": "Final Security Audit",
    "status": "pending",
    "dependencies": ["1", "2", "3"],
    "baselineSkills": ["auto-chat-quality", "auto-code-quality", "auto-writing-quality", "auto-security-quality"],
    "claimedAt": null,
    "completedAt": null,
    "verification": null,
    "reviewEvidence": null
  }
}
```

### Baseline and Frozen Review Snapshot

1. Before first implementation, record `reviewBaseline`: the resolved starting commit (an explicitly supplied base takes priority; otherwise current HEAD), timestamp, initial tracked/index/worktree changes, and user-owned untracked files, including content hashes/patch evidence needed to distinguish later edits. Preserve this baseline through resumes; do not silently reset it to the new HEAD. An unborn repository uses an explicit empty-tree baseline.
2. After all implementation work settles, stop writers and create a snapshot manifest covering the complete resulting review scope: baseline-to-current committed changes, staged changes, unstaged changes, new/untracked source, deletions, renames, file modes, and relevant adjacent code. Record the current HEAD as metadata and a content fingerprint of repository-relative paths and bytes, including symlink targets and submodule commits/dirty state when present. Exclude only recorded generated/tool-output paths such as the plan status and final report directory; do not omit untracked source merely because `git diff` does.
3. Store `finalGate` with `phase` (`pending`, `reviewing`, `remediating`, `passed`, or `incomplete`), coordinator identity, baseline, snapshot ID, manifest path, and distinct artifact roots/report paths. The functional report may be `docs/plans/.dominion-logs/{slug}/final/{snapshotId}/review.md`. The security worker uses `auto-security-quality`'s permitted external run directory by default (resolved from `~/security-audit-skill/<repo>/run-<N>`) and returns its `REPORT.md`, structured findings, coverage ledger, metadata, and other required paths. A requested in-target security directory is allowed only when the upstream ignore/output rules permit it; never silently force tracked target output. The coordinator can write a linking summary in its log directory after the join. Reports include their snapshot ID, scope, actual skill loads, checks, findings, and coverage limits.
4. Define the snapshot ID from baseline plus reviewed content/modes, not mutable status/report output. A recorded finalization commit changes HEAD/index bookkeeping but does not invalidate passing content evidence when the exact reviewed source bytes and baseline coverage remain unchanged. Both readers use a frozen checkout/copy including the captured working tree, or the shared checkout under a no-source-writes barrier. Run any test/build command that may mutate tracked source only before the snapshot or in an isolated copy. Independent readers may write only inside their assigned artifact roots; nested audit workers get isolated scratch paths under that root. Check the manifest before accepting results; any source drift invalidates both reports, requires a new snapshot, and is never a clean pass.
5. Compare against the stable baseline for change coverage; do not use only `git diff HEAD` or only the latest commit. Preserve initial user changes separately for ownership/staging decisions. Review coverage does not authorize overwriting or committing unrelated user work.

### Join, Remediation, and Completion

Respect the host's real agent capacity. Reserve a slot for mandatory fresh design/security reviewers, or have a worker return a bounded child assignment and release its slot so the coordinator can schedule that independent reviewer and resume the owner later. Do not occupy every slot with workers waiting for children. Security candidate/final independent validation and Impeccable finish review are explicit bounded exceptions to the implementation-primary no-nesting rule; use sequential independent phases if concurrency cannot fit. Never substitute the author's self-review for an unavailable independent agent.

The coordinator waits for **both** complete reports and required nested verification, then presents the findings together. One designated owner applies fixes within the already authorized scope using the relevant quality skills. Shared source is never patched concurrently by the two reviewers. For distinct-file delegated fixes, ownership must be exclusive and all writers must rejoin before recapturing the snapshot.

After remediation, re-run affected project checks, create a new snapshot, and re-review changed paths plus their dependent/security surfaces. Both reviewers must issue fresh evidence bound to the new snapshot, covering fixes and checking for regressions; unchanged areas may reuse previous analysis only with an explicit explanation of unchanged inputs. Missing tools, missing reports, unresolved required findings, or incomplete coverage are recorded as `incomplete`/`findings`, never coerced to `pass`. Existing escalation/authorization rules still govern genuinely blocked or out-of-scope work. Record implementation `settledAt` only after all its verification/remediation/late deltas finish.

Mark both final streams `completed` and `finalGate.phase: passed` only when both have passing evidence for the same current snapshot and required project checks pass. In `review` mode, only the coordinator/finalization owner then follows the existing commit/push/plan cleanup flow. In `codex` mode, retain plan, status, reports, and the working tree and perform the existing Codex `/verify` handoff. Persist a finalization receipt in the retained audit directory with owner, content snapshot, intended files, resulting commit, push state, and cleanup state; consult it on resume so an interrupted push/cleanup never creates a duplicate commit. The security pass does not replace Codex validation. Any later code or human-facing artifact edit invalidates final evidence for affected scope; rejoin and refresh both reports before finalization.

### Idempotent Migration of Existing Plans and Status

Apply this normalization whenever a status file is initialized or resumed, including by `/verify`. Do not rewrite the read-only implementation plan or erase progress:

1. Preserve stream IDs, existing domain skills, mode, claims, timestamps, findings, and implementation progress. Missing mode retains the historic `review` default.
2. Add `final` or `final-security` only if the reserved ID is absent. Recompute both dependency lists from implementation IDs, excluding both reserved IDs. Never append duplicates or introduce a sibling dependency cycle.
3. Merge the quality-routing floor into each stream's skills (deduplicated). Add `auto-design-quality` when actual design work requires it even if a legacy plan lacks it. Replace retired `design`, `ui-ux-pro-max`, and `auto-layout` assignments with `auto-design-quality`; map retired `auto-coding`, `auto-sanity`, and `auto-refactor` to `auto-code-quality` (also retain/add `auto-testability` for structural review); deduplicate, and discard their retired search/artifact gates; use the new skill's legacy set references only for established project constraints.
4. Preserve any existing stable baseline. If none exists, reconstruct it from recorded execution evidence when reliable. Otherwise mark baseline provenance unknown, review the entire current repository as the conservative security scope, and record that historical change attribution is unavailable. Do not silently invent a starting commit that excludes previous implementation commits.
5. Legacy `final: completed` is not proof of a security pass. Preserve its old evidence in history, add the missing sibling, and reopen final review eligibility if it lacks matching current snapshot evidence. Do not automatically replay a prior commit or push. For completed implementation streams lacking `settledAt`, recover it only from reliable verification/remediation evidence; otherwise run the missing settling checks without re-running their implementation primaries. A `passed` gate is reused only if its two reports and current manifest still match.
6. Do not steal an active claim. A running old final worker must finish or be stopped under the workflow's existing takeover policy before migration can establish the source-write barrier. Record that finalization is blocked until the pair gate is satisfied.
7. Persist version 2 with one atomic, coordinated write. Repeating normalization with unchanged inputs must produce no semantic changes. New implementation streams invalidate the previous final gate and extend both dependency lists automatically.

## Parsing Rules

Extract streams from plan markdown using these patterns:

```
## Stream N: Title        →  id: "N",  name: "Title"
## Stream N — Title       →  id: "N",  name: "Title"
### NA. Sub-title         →  id: "NA", name: "Sub-title" (under parent stream N)

**Dependencies:** Stream 1 (notes component)  →  dependencies: ["1"]
**Dependencies:** Streams 1 and 2              →  dependencies: ["1", "2"]
**Dependencies:** None                         →  dependencies: []
```

## Concurrency

Read → check → overwrite is **not** safe optimistic concurrency for a JSON file: two sessions writing different keys can still lose each other's changes. Use a single writer or an exclusive lock covering fresh read, claim validation, update, and atomic replacement.

- Under `/dominion`, the orchestrator owns status writes; workers return their claim/completion/verification data and write only in their exclusive artifact directories. The orchestrator serializes updates and confirms completion from independent evidence.
- Under manual `/stream`, use a repository-local lock for the whole read-modify-write transaction. Do not rely on `flock` being installed on macOS; use an available locking primitive or an atomic lock-directory operation with recorded owner identity. Do not steal a live lock or stale claim without the workflow's existing takeover authorization. Write a temporary file in the same directory and atomically rename it while holding the lock.
- One coordinator owns `finalGate`. Review workers never mutate it or one another's reports. When the final pair is ready in manual mode, the claiming session becomes coordinator and launches both reviewers automatically; no second user session or command is required. If the runtime cannot run workers concurrently, preserve the two independent passes and the same join barrier sequentially and report that execution limitation.
