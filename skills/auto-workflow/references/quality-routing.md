# Automatic quality routing

Apply this contract throughout `/summon`, `/dominion`, `/stream`, and their delegated work. Re-evaluate it when a task changes from discussion into edits, when new files enter scope, and after context compaction. These loads supplement existing discipline skills; they add no setup interview, mode-selection menu, or routine approval gate.

**Auto mode is the default.** Within the requested task, resolve routine choices from evidence, run applicable quality/refinement gates, amend plans, fix supported findings, and verify without asking whether to continue. Instructions to ask before those routine steps become an agent decision followed by action. Keep explicit plan-only or review-only boundaries. Ask only for missing intent that materially blocks correct work, or a real authorization/tool-permission requirement. A bare `/summon` can still ask what work the user wants; a supplied task must not trigger a redundant interview. Honor an explicit request for interactive checkpoints.

| Work being performed | Required load before work |
| --- | --- |
| Summon or dominion entry, progress, handoffs, user replies | `auto-chat-quality` immediately |
| Code implementation, tests, refactoring, code review, verification, or remediation | `auto-code-quality` (Ponytail), including direct work after any summon path |
| Human-facing prose: UI strings, website copy, help, repository docs, plans, reports, comments explaining code, or substantive written responses | `auto-writing-quality` (Humanizer) |
| New visual/interaction design or edits to an existing design | `auto-design-quality`, plus `auto-accessibility` |
| Summon security path or final security sibling stream | `auto-security-quality`, plus code/chat/writing skills appropriate to the work |

Plain reuse of an established component with unchanged appearance and behavior does not require Impeccable. A changed layout, new input interaction, style, or design decision does. Writing its label still requires writing quality. Apply this by the work actually performed, not by whether a plan happened to list the skill.

## Loading on both hosts

On Claude, invoke the available Skill tool. On Codex or hosts without that tool, read the named skill's `SKILL.md` from its discovered installation location. Resolve bundled paths relative to that actual skill directory; do not assume the working project has a `skills/` directory. Prefer the installed skill catalog, then the known Claude/Codex skill root. Missing required skill content is a concrete setup failure: report it and do not claim that the quality pass happened. Never ask the user to type a subcommand that the agent can select and run itself.

Read the wrapper first and follow its routed upstream references. Do not independently activate a vendored original skill name. Load once per agent context, read conditional references only when needed, and preserve the loaded-skill manifest in handoffs. Re-load missing instructions after compaction before continuing work governed by them.

## Delegation is explicit

The coordinating agent reads [agent capacity and dispatch](agent-capacity.md) before spawning workers. Resolve the actual host limit and fill available slots with eligible work; a per-stream role budget is not a global concurrency cap. Pass the remaining capacity and any child-review assignment back through the coordinator rather than letting descendants overbook the shared pool.

Every primary, test, implementation, integration, verification, review, security-validation, and remediation agent receives the applicable required skill names and resolved paths in its prompt. Parent loading does not load a child's context. Agents that read or edit code **must load `auto-code-quality` themselves**; agents that write prose **must load `auto-writing-quality` themselves**. Include applicable Impeccable commands and security instructions too.

The worker's first step is to read/invoke the required skills. Its result records which entrypoints and routed references were loaded, what work they governed, and verification evidence. The coordinator checks the tool/read evidence when available; a name in an output checklist is not proof of loading. If a required load was missed, return the task for loading and a fresh affected-work audit before accepting completion. Do not rerun unrelated work.

## Preserve intent and autonomy

Infer scope, audience, design context, and suitable subcommands from the user's ask and repository before asking anything new. Never repeat an answered question. Select the useful Ponytail or Impeccable operations internally. Humanizer edits the actual deliverable and checks it again; it does not turn ordinary writing into a critique-and-approval loop.

Chat quality controls how work is communicated; writing quality controls the words; code/design/security skills control their respective work. Preserve exact API identifiers, quotations, required formats, facts, project conventions, and the user's chosen design direction. The upstream skills' substantive guidance remains in their bundled references. Automatic routing does not expand the authorized task or bypass real tool permissions. During concurrent final review, reviewers only report; the designated coordinator performs or assigns fixes after both reviews join.
