# Agent capacity and dispatch

Apply before delegation in summon, dominion, stream, and auto-legion. Fill available worker slots with useful independent work automatically, including Summon Path 2. Capacity limits throughput; they do not reduce the required quality skills, verification, or independent reviews.

## Resolve the current runtime's capacity

Use the actual agent tools and their counting rules, not the model name. Prefer session-injected limits and native tool feedback, then effective runtime configuration, then a documented default that applies to the installed version. Keep the source of the limit in the execution preview or internal dispatch record.

| Runtime | Capacity evidence and counting |
| --- | --- |
| Claude Code Agent tool | Current documented default: **20 running subagents per session**, excluding the main conversation, in v2.1.217+. Honor the effective positive-integer `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` override and any session restrictions. Nested/running resumed subagents also occupy capacity. |
| Codex native agent threads | Honor the session limit and effective `agents.max_concurrent_threads_per_session`; `agents.max_threads` is its legacy alias. This counts **open spawned threads**, excluding the primary. When unset, Codex chooses its default; do not assume every installation has a three-worker cap. |
| Hosted collaboration tools | Follow the injected total-slot or active-subagent limit across the whole tree. If it says four total agents including the root, that leaves **three active worker slots** shared by all descendants. The Responses API default is three active subagents, excluding the root, but it is configurable. |

Normalize to a worker limit once. Subtract the coordinator only when the supplied limit includes it. Account for other workers already occupying the same pool. If multiple applicable limits constrain different resources, honor all of them and admit only what fits each remaining budget. Do not confuse a per-stream total-role/retry budget, tool-call concurrency, spawn depth, or model usage limit with the number of concurrent workers.

Use native background Agent calls on Claude and the available collaboration tools on Codex. A completed turn frees an active-turn slot; it may still occupy an open-thread slot. For hosts that count open threads, collect the result and close/release the finished agent with the available native tool before replacing it. Waiting or interrupting is not proof of release. If no release tool exists, reuse an idle worker for compatible work that does not require fresh-context independence. Do not invent a close tool when the host only counts active turns.

Use Claude's completion notifications or Codex's available native wait/status tools to receive results. On Codex, do not depend on a Claude-specific background notification. Prefer event-driven waits, keep progress updates flowing, and avoid shell polling or scheduled wakeups. Receiving one completion is enough to reconsider admissions; it need not wait for every worker.

## Keep available slots working

1. Identify ready tasks with bounded scope, exclusive write ownership, and enough inputs to start. Use a small in-memory queue for direct work; a written plan is not required. Keep a tiny indivisible edit local.
2. Dispatch `min(ready tasks, free worker slots)` immediately through native calls. Queue the remainder. When the host permits parallel calls, launch the fitting set together; otherwise issue the calls without waiting for each worker's result.
3. Whenever a worker finishes, collect its evidence, release its capacity as required by that host, and refill the available slots. Do not wait for the whole batch when another eligible task can start. Include verification and remediation in this scheduling, with priority for work that unblocks dependencies or a required review.
4. Keep the coordinator useful on planning, integration, or other unowned work while workers run. Give workers distilled task context and actual required skill paths; each worker still loads its applicable quality skills itself.

If too few streams are ready but a stream contains independent tasks, its primary can return bounded task packets for the coordinator to dispatch into free slots. Each packet includes inputs, exact file ownership, required skills, and acceptance checks. Transfer ownership before dispatch; the primary must not edit delegated files and releases its own capacity if only waiting. Resume it for integration after the packets join. A checkpoint or helper return is not stream completion and cannot unlock downstream work. Preserve the existing bounded retries; decomposition is not permission to start an unbounded remediation loop.

Use the full discovered capacity when enough independent work exists: 20 ready Claude workers can use a default 20-worker pool; a Codex session with three worker slots can run three and refill them continuously. A higher configured Codex capacity should be used too. Do not force Codex into serial execution, route its work to Claude, or hold slots idle merely because it has a smaller pool. Preserve file ownership and the final frozen-snapshot barrier.

## Independent reviews without idle slots or deadlocks

The coordinator brokers required fresh reviewers. Prefer having a worker return its bounded independent-review assignment and artifacts, then finish its turn. Confirm capacity was released using the host's rules before scheduling the fresh reviewer, then resume the owner with the findings when capacity permits. This lets implementation use the full pool without every worker waiting for a child.

Reserve capacity only for a concrete requirement: an active review needs a resident parent, or the host counts open threads but offers no release tool. In the latter case, account for already-known fresh-review roles before filling the pool; if they cannot fit, report that capability limit instead of consuming the last usable fresh contexts. Release reservations when their need ends. If nesting is unavailable or depth-limited, use a fresh sibling reviewer scheduled by the coordinator. With a small pool, run independent passes sequentially while preserving independent contexts and the joined final gate. Never substitute the author's self-review for a required independent review.

When a capacity rejection occurs, preserve already-started work, refresh the actual occupancy, and queue the rejected task until capacity is released. Do not retry repeatedly, launch dummy probes, alter user configuration, or use resumes, subprocesses, separate sessions, or agent teams to bypass a native limit. If no native delegation is available, do the feasible local work and report the independent-review limitation honestly.

## Source notes

Checked 2026-10-01: [Claude Code concurrent subagent limit](https://code.claude.com/docs/en/sub-agents#concurrent-subagent-limit), [Codex subagent settings](https://learn.chatgpt.com/docs/agent-configuration/subagents#global-settings), and [Responses API multi-agent counting](https://developers.openai.com/api/docs/guides/responses-multi-agent). These document different mechanisms; live session constraints take precedence over these defaults.
