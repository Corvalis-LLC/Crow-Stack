# Upstream provenance

- Source: [DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail).
- Pinned commit: [`e3ba2aa6f1e6f0bc4d69eb09c9f0d0a93af56156`](https://github.com/DietrichGebert/ponytail/tree/e3ba2aa6f1e6f0bc4d69eb09c9f0d0a93af56156).
- License: MIT, copyright 2026 DietrichGebert. The original notice is retained in [LICENSE](../../LICENSE).
- All six upstream skill files under `skills/` are retained byte-for-byte in [references/upstream](references/upstream), with descriptive `.md` names so they are not discovered as separately installed skills.
- Integrity: [UPSTREAM.json](UPSTREAM.json) records source paths and SHA-256 hashes.

The local entrypoint renames the skill, activates full mode for direct work, routes subcommands internally, and applies fixes when the enclosing workflow already authorizes them. Existing repository verification, stream ownership, and host communication rules take precedence over standalone examples. The original `ponytail:` marker remains unchanged. No upstream behavioral reference is rewritten.

The upstream plugin hooks, installation wrappers, host configuration, and slash-command files are not installed. They duplicate activation supplied by summon and dominion and would add unrelated global behavior. All skill modes are accessible through the local entrypoint. The original repository and supporting benchmark resources remain available at the pinned commit.

## Benchmark caveat

The retained `ponytail-gain` reference contains the older five-task, single-shot benchmark card. At the same commit, the upstream [README](https://github.com/DietrichGebert/ponytail/blob/e3ba2aa6f1e6f0bc4d69eb09c9f0d0a93af56156/README.md) explains that those figures overstate typical agentic savings. Its [June 18 agentic results](https://github.com/DietrichGebert/ponytail/blob/e3ba2aa6f1e6f0bc4d69eb09c9f0d0a93af56156/benchmarks/results/2026-06-18-agentic.md) report a different experiment. If asked for measured savings, identify the benchmark and its limitations, consult those pinned sources, and distinguish historical figures from current-project measurements. Do not present the older card as a general guarantee or measured savings for the user's repository.

The repository [LICENSE](../../LICENSE) contains the upstream notices in full. This skill has no separate license file. `UPSTREAM.json` records the original notice hashes and their marked sections in that root file.
