---
name: auto-design-quality
description: "Impeccable design guidance and automatic command routing for new interfaces, components, inputs, and changes to existing visual designs. Use for UI planning, design, refinement, accessibility, responsiveness, typography, layout, motion, and UX copy. Skip unchanged reuse of an established component."
---

# Automatic design quality

Apply the preserved [upstream skill](upstream/skill/IMPECCABLE.md) and the relevant playbook. This entrypoint changes orchestration and routine interaction only; the original design guidance, commands, references, agents, and launcher remain in [upstream](upstream/skill/). [UPSTREAM.md](UPSTREAM.md) records the exact source and license. Upstream references to its `SKILL.md` mean the renamed `upstream/skill/IMPECCABLE.md`; its bytes are unchanged and the filename prevents duplicate skill discovery.

## Automatic entry

Summon, Dominion, and their workers load this skill before making a new design decision or editing an existing design. A new input, component, screen, responsive treatment, or visual change qualifies. Placing an unchanged, established component using its existing documented patterns does not need a new design pass. Once its styling, interaction, or visual contract changes, load Impeccable.

The caller retains its task, scope, and authorization. Read the upstream skill once and only the playbooks needed now. Apply these integration rules when the upstream workflow assumes a manually invoked command:

1. Infer the target, product, audience, platform, stack, and established visual direction from the user's request, accepted plan, project instructions, existing components, tokens, screenshots, PRODUCT.md, and DESIGN.md. Also read a legacy `.design/system.md` when present. If it names an `active_set`, read the matching file from [references/legacy-design-sets](references/legacy-design-sets/) when available and preserve its tokens/customizations. Read `design-system/MASTER.md` when present. These are existing design constraints, not permission to replace the system. Do not offer the legacy sets as a new setup menu.
2. Select the command from the table below. An automatic load with an active task never shows a command menu or asks the user to name a subcommand. When two playbooks fit, use the narrowest one first and add the second only if needed.
3. Reuse context and decisions already obtained by Summon or Dominion. Do not run an extra onboarding interview, question-tool probe, stack question, approval round, live-browser picker, or setup menu. Label material inferences; leave unsupported product claims unknown. Existing project context remains authoritative. Create or update PRODUCT.md only when useful to the requested design work, preserving existing content and distinguishing inferred facts from user-confirmed facts.
4. For automatic new work, use the upstream **code-led** build path. Choose a direction from the brief and incumbent design evidence, record the decision as agent-selected, and proceed. Do not launch image-comp selection, component-review servers, or live interaction loops that require the user to operate them. Keep the upstream craft floor, authored assets when called for, accessibility, bounded visual verification, and independent finish review. Never claim that an agent-selected direction was user-approved.
5. Load `auto-writing-quality` when writing human-facing copy and `auto-code-quality` for implementation. These complement the chosen design playbook. Preserve factual product copy and user commitments unless the request authorizes their change.

These adaptations supersede upstream requirements for routine interviews, menus, choice/approval probes, mandatory init before ordinary work, and comp-first routing in automatic calls. They do not authorize publishing, paid services, unrequested product claims, or destructive changes. A genuinely blocking product decision goes back to the calling workflow; do not add a separate skill setup conversation.

## Pick the playbook

Paths below are relative to `upstream/skill/reference/`.

| Task | Read and apply |
|---|---|
| New screen/component, new visual world, or requested redesign | `new-work.md`; `shape.md` when planning UX; `operate.md` for task or reading interfaces |
| Small edit to an existing design, alignment, shipping polish | `polish.md`; preserve incumbent identity and scope |
| Design or usability review | `critique.md` |
| Accessibility, responsive, or technical UI review | `audit.md`; use `audit.native.md` on native platforms |
| Layout/spacing or typography | `layout.md` or `typeset.md` |
| Labels, instructions, errors, onboarding/empty states | `clarify.md` or `onboard.md`, plus `auto-writing-quality` |
| Error states, overflow, locale or other UI edge cases | `harden.md` |
| Device adaptation or UI performance | `adapt.md` (native variant when applicable) or `optimize.md` |
| Simplification, stronger/quieter design, or color | `distill.md`, `bolder.md`, `quieter.md`, or `colorize.md` |
| Motion, personality, or explicitly ambitious effects | `animate.md`, `delight.md`, or `overdrive.md` |
| Documenting a system or extracting reusable components/tokens | `document.md` or `extract.md` |
| Missing durable product context | Relevant portions of `init.md`, using evidence and the automatic-entry rules |
| Explicit live variants, named commands, hooks, or diagnostics | The upstream Commands table and corresponding `live.md`, `generate.md`, `hooks.md`, or `doctor.md` |

Read [craft-floor.md](upstream/skill/reference/craft-floor.md) immediately before UI edits. Ordinary reuse never grants permission to redesign long-lived components. A requested redesign keeps product truth and function while replacing the visual world within scope.

For CSS or native layout edits, also read [layout implementation guardrails](references/layout-guardrails.md). This contains the distinct implementation rules retained from the retired `auto-layout` skill.

## Portable tools and agents

The upstream runtime is optional for applying the design language. Resolve `<skill-base-dir>` and upstream `${CLAUDE_SKILL_DIR}` references to this skill's absolute `upstream/skill` directory, and `{{scripts_path}}` in source agent definitions to its `scripts` directory. Never assume Claude-specific variables exist in Codex.

Use a locally installed engine if available. The vendored launcher's no-download mode is:

```sh
IMPECCABLE_LAUNCHER_PROBE=1 "<absolute-skill-dir>/upstream/skill/scripts/impeccable" context --target <target-path>
```

The probe guard permits an existing configured, sibling, or version-cached engine and prevents first-run downloads. Apply `IMPECCABLE_LAUNCHER_PROBE=1` to **every** automatic launcher invocation, including delegated detector, storage, and asset helpers; the context command above is one example. Do not install a runtime, hook, plugin, browser bridge, or dependency merely because a design task loaded this skill. If no engine is available, state that context loading did not run, read the existing project context directly, and perform the applicable playbook through available tools. Skip engine-only mechanics truthfully; record equivalent design reasoning and verification without fabricating engine artifacts or scores. The runtime may be installed when the user explicitly requests its engine features or installation.

For an upstream named agent, use the platform's existing agent if installed; otherwise supply the relevant file from [upstream/skill/agents](upstream/skill/agents/) to a generic delegated agent with its bounded inputs. This replaces the upstream assumption that named agents are automatically registered. Resolve template/tool names to actual available capabilities. A finish reviewer receives fresh context (Codex `fork_turns: "none"`) and the required screenshots; the builder applies its findings. Preserve read/write ownership and the upstream review limits. When delegation or visual evidence is unavailable, disclose the narrower check and use the documented degraded reference where applicable.

Every delegated agent prompt includes the resolved required-skill paths and an explicit first step: load `auto-code-quality` before reading, reviewing, or editing code, and load `auto-writing-quality` before writing prose, including reports or design documentation. Include this wrapper's orchestration rules for every design delegate, including detector, asset, and documentation workers; load the selected playbook when applicable. The result records loaded entrypoints/references and the work they governed; the coordinator checks actual read/tool evidence where available. Parent loading is insufficient. Apply the shared [quality-routing contract](../auto-workflow/references/quality-routing.md), and return a missed load for a fresh affected-work check before accepting the result.

### Capacity and review ownership

Before launching parallel builders that require an independent finish review, the calling coordinator reserves at least one available helper slot or brokers their child work itself. Do not fill every slot with workers that will each wait for a nested reviewer. A delegated builder requests the required review from its coordinator when it has no capacity; the coordinator queues independent work and frees or reserves capacity before asking the builder to wait. If needed, return a checkpoint with the bounded review inputs so the outer worker releases its slot and can resume after the review. Never spin on failed spawn calls.

Upstream critique's assessments A and B may run sequentially as separate fresh agents when capacity prevents parallel execution. Neither sees the other's output, and synthesis still waits for both. This adapter overrides the upstream requirement to launch those agents simultaneously, while preserving their independence. Keep the builder paused against reviewed source until its reviewer finishes. Insufficient capacity does not turn self-review into independent evidence; if the coordinator cannot arrange an independent check, report that check as incomplete.

When the caller assigns a read-only final review, preserve its frozen snapshot and write boundary. Return findings to the caller for joined remediation. Use only the assigned artifact/scratch root for screenshots or reports; skip upstream persistence to `.impeccable/critique/`, PRODUCT.md, DESIGN.md, or project configuration. Do not run helpers that would write outside the assigned root. Read existing context and captures directly when the runtime cannot respect that boundary, and disclose unavailable detector or visual evidence. Documentation and implementation follow-ups wait until the caller releases the source barrier and assigns an implementation owner.

Explicit user requests for the upstream comp-led or live engine workflow retain its real approval gates. Never forge a receipt, submit a user decision, or bypass a gate. Automatic work avoids those workflows; explicit engine work can require the user action inherent in that requested feature.
