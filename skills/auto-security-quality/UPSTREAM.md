# Upstream provenance

- Repository: https://github.com/cloudflare/security-audit-skill
- Commit: `c1c8a8c1471069fb0e188eeaff69b8e8db6564a8`
- Retrieved: 2026-10-01
- License: [MIT](../../LICENSE), copyright 2025-2026 Cloudflare, Inc.

The complete upstream `skills/security-audit/` directory is copied byte-for-byte into `upstream/`, with `SKILL.md` renamed to `SECURITY-AUDIT.md` and the root upstream license consolidated in the repository `LICENSE`. This includes all attack-class/domain companions, reconnaissance and independent-validation instructions, report schema, both dependency-free Node.js validators, and their original test suites. The root SKILL.md is the Corvalis adapter; the upstream skill remains named `security-audit` within its preserved reference. The renamed file prevents recursive skill discovery from bypassing the adapter; upstream mentions of `SKILL.md` resolve through the wrapper. [UPSTREAM.json](UPSTREAM.json) maps every preserved file to its source and SHA-256 digest.

The adapter selects mode/scope/output from Summon or Dominion, adds the shared frozen-source contract for concurrent final review, and hands authorized fixes back to the calling implementation owner. It does not modify upstream evidence, sandbox, independence, validation, or severity requirements.

Run the preserved test suites from this skill directory:

```sh
node --test upstream/validate-findings.test.cjs upstream/validate-coverage-ledger.test.cjs
```

No third-party runtime dependencies are required. Passing these suites checks validator behavior, not the security posture of any repository audited later.

The repository [LICENSE](../../LICENSE) contains the upstream notices in full. This skill has no separate license file. `UPSTREAM.json` records the original notice hashes and their marked sections in that root file.
