# Upstream provenance

- Source: [blader/humanizer](https://github.com/blader/humanizer).
- Pinned commit: [`225a6f39ac85f76ee48dbad772ea4abe4ed6c9d8`](https://github.com/blader/humanizer/tree/225a6f39ac85f76ee48dbad772ea4abe4ed6c9d8).
- Upstream skill version: 3.1.0.
- License: MIT, copyright 2025 Siqi Chen. The original notice is retained in [LICENSE](../../LICENSE).
- Original `SKILL.md` is retained byte-for-byte as [references/upstream/humanizer.md](references/upstream/humanizer.md), including all 26 patterns and the source attribution.
- Integrity: [UPSTREAM.json](UPSTREAM.json) records source paths and SHA-256 hashes.

The supplied `https://github.com/blader/humanizers` URL returned “Repository not found.” The singular `blader/humanizer` repository is the available source matching the requested Humanizer skill; this is a documented URL correction, not a GitHub redirect.

The local entrypoint renames the skill and broadens activation to newly authored human-facing copy as requested. It selects the upstream embedded or file mode automatically, applies final text directly, and uses available context before requesting information. UI-string handling adds protection for placeholders and executable syntax. The editing process, voice rules, examples, and patterns are unchanged.

The upstream package validator checks the original repository's plugin manifests, README, and changelog; it is not a runtime resource of the editing skill and is not installed. The upstream plugin packaging and host metadata are replaced by the local skill entrypoint and `agents/openai.yaml`. The full original package is available at the pinned commit above.

The repository [LICENSE](../../LICENSE) contains the upstream notices in full. This skill has no separate license file. `UPSTREAM.json` records the original notice hashes and their marked sections in that root file.
