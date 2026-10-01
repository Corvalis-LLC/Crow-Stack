---
name: auto-writing-quality
description: "Apply Humanizer whenever writing or editing text people will read: website prose, UI labels and messages, help guides, repository documentation, plans, reports, comments, and user-facing explanations. Preserve facts, voice, and functional syntax while removing formulaic AI writing. Activate automatically during summon or dominion work that creates human-facing copy."
---

# Auto writing quality

Read and apply the complete [Humanizer skill](references/upstream/humanizer.md), including its editing process, voice guidance, 26 patterns, exceptions, and factual preservation checks. The source is preserved verbatim. Apply these integration rules to its invocation and manual handoffs:

- Load before drafting or editing human-facing text, including small UI strings and text embedded in code. Run Humanizer's draft, review, and final rewrite as part of producing the requested artifact. Do not ask the user to paste a draft the agent can read, select an editing mode, or perform the editing pass.
- Use **embedded mode** for copy created within another task: return or write only the final copy. Use **file mode** when editing a specified file and apply the final prose directly. Keep intermediate drafts and the list of detected patterns internal unless the user requests a writing audit or the upstream standalone rewrite format.
- Infer audience, tone, and terminology from the request, surrounding text, existing product, and repository conventions. Follow a supplied writing sample. When a detail is unsupported, use a simpler accurate sentence or retain the existing uncertainty. Ask only if an essential fact cannot be obtained from the available context and the task cannot be completed accurately without it.
- Preserve facts, numbers, names, dates, citations, links, meaningful uncertainty, and the writer's deliberate voice. The skill improves prose; it is not evidence that a text was written by AI.
- In documentation, preserve code blocks, inline code, commands, paths, metadata, and link targets. In UI source, edit only intended human-readable string values; preserve interpolation placeholders, escaping, message keys, accessibility meaning, and executable behavior. Keep quotations, legal notices, and required terminology intact unless changing them is part of the request.

Apply the checks to new text before delivery and to existing text within the authorized edit scope. Do not expand a scoped copy change into a repository-wide rewrite. Auto-chat-quality still shapes conversation updates; this skill checks the quality and accuracy of the words themselves.

The user-provided `blader/humanizers` repository does not exist; this integration uses [blader/humanizer](https://github.com/blader/humanizer). Source, license, and adaptation details: [UPSTREAM.md](UPSTREAM.md). File hashes: [UPSTREAM.json](UPSTREAM.json).
