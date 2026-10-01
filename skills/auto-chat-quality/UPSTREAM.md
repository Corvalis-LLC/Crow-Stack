# Upstream provenance

- Source: [ayghri/i-have-adhd](https://github.com/ayghri/i-have-adhd).
- Pinned commit: [`839872f9d1cd634fed642b4589ce7226199cc15f`](https://github.com/ayghri/i-have-adhd/tree/839872f9d1cd634fed642b4589ce7226199cc15f).
- License: MIT, copyright 2026 Ayoub Ghriss. The original notice is retained in [LICENSE](../../LICENSE).
- Original skill: `skills/i-have-adhd/SKILL.md`, retained byte-for-byte as [references/upstream/i-have-adhd.md](references/upstream/i-have-adhd.md).
- Integrity: [UPSTREAM.json](UPSTREAM.json) records the source path and SHA-256 of every vendored file.

The local entrypoint renames the skill, enables implicit selection, and connects it to summon and dominion. It translates agent-executable user handoffs into direct action, preserves existing routing and authorization, and avoids inventing next steps. It does not change the upstream communication rules. The source's explicit-only frontmatter is archival; the local `SKILL.md` and `agents/openai.yaml` control discovery.

The upstream installation wrappers and Gemini command are not part of this Claude/Codex skill integration. Automatic loading comes from the local workflow, not an additional plugin, hook, or manual invocation. The full upstream repository remains available at the pinned commit above.

The repository [LICENSE](../../LICENSE) contains the upstream notices in full. This skill has no separate license file. `UPSTREAM.json` records the original notice hashes and their marked sections in that root file.
