---
name: auto-code-quality
description: "Apply Ponytail's simplicity ladder to direct coding, implementation, fixes, refactoring, dependency choices, and code reviews. Load for summon Path 2, dominion, and whenever either workflow starts direct code work. Route complexity review, audit, and debt subcommands automatically from the task."
---

# Auto code quality

Read and apply the complete [Ponytail skill](references/upstream/ponytail.md) before direct code work. The upstream rules, ladder, examples, intensity levels, and safeguards remain intact. Apply these integration rules to its invocation and manual handoffs:

- Activate in **full** mode automatically for summon Path 2, dominion, and any transition from planning, discussion, design, or audit into direct code work. Honor a user-selected intensity or explicit opt-out for the session. Do not ask the user to select a mode or run a command.
- Inspect the actual flow, existing components, and callers before choosing the simplest implementation that fulfills the request. An explicitly requested behavior remains a requirement. Preserve validation, error handling, security, accessibility, and the repository's required verification.
- During scoped refactoring, preserve public APIs and observable behavior unless the requested change requires altering them. Keep changes within scope and verify before and after. Prefer readable expressions and useful responsibility boundaries over dense one-liners or splitting files/functions solely to meet a line-count quota. Use `auto-testability` when extracting domain logic or verification seams.
- Use the existing test framework and checks when present. Upstream's minimal standalone-check suggestion does not replace repository test conventions or required gates. Do not add tests that merely repeat a reversible text edit or trivial implementation.
- Let auto-chat-quality and the host govern progress and final communication. Ponytail's terse code-first examples do not suppress required findings, evidence, requested explanations, or workflow updates.
- Keep the existing `ponytail:` marker for deliberate limitations so the upstream debt scan still finds them. Do not add markers to ordinary changes that have no real limitation to record.

## Internal command routing

Choose and read the matching reference yourself. These are local modes within this skill; the user does not need to install the upstream plugin or invoke another slash command.

| Current work | Reference and action |
|---|---|
| Implement or fix code | [ponytail](references/upstream/ponytail.md): use the ladder in full mode unless the user chose otherwise. |
| Review a diff for avoidable complexity | [ponytail-review](references/upstream/ponytail-review.md): inspect the current scoped diff and report grounded simplifications. |
| Audit a codebase or selected modules for complexity | [ponytail-audit](references/upstream/ponytail-audit.md): apply the same hunt to the user-selected scope; absent a scope, use the current project. |
| Inspect deliberate shortcuts or produce a debt ledger | [ponytail-debt](references/upstream/ponytail-debt.md): search the project using the available search tool and preserve file/line evidence. Write a requested or plan-required ledger directly. |
| Explain modes or commands | [ponytail-help](references/upstream/ponytail-help.md): describe this integration's local modes; upstream plugin installation, hooks, environment configuration, and update steps are not used here. |
| User explicitly asks about upstream measured savings | [ponytail-gain](references/upstream/ponytail-gain.md): observe its honesty boundary and the benchmark caveat in [UPSTREAM.md](UPSTREAM.md). Never invent savings for the current project. |

Upstream review and audit modes produce findings without applying them. When the enclosing task already authorizes fixes (including summon's audit-and-fix path or dominion's planned remediation), hand actionable findings back to the coordinating agent or perform the authorized fixes directly, then verify them. Respect review-only requests and stream file ownership. Do not stop to ask whether to fix issues already within the task's authorized scope. A security issue goes to auto-security-quality and the existing verification workflow; a short diff is not evidence of security.

Source and license details: [UPSTREAM.md](UPSTREAM.md). File hashes: [UPSTREAM.json](UPSTREAM.json).
