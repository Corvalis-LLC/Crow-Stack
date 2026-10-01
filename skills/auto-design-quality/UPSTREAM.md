# Upstream provenance

- Repository: https://github.com/pbakaus/impeccable
- Commit: `c74755d920985f7a92cef691ca970ba95f90126e`
- Retrieved: 2026-10-01
- Upstream skill version: 4.4.0; launcher engine version: 0.1.9
- License: [Apache-2.0](../../LICENSE); [third-party notices](../../LICENSE)

Vendored files are byte-for-byte copies, with these path mappings:

| Upstream path | Local path |
|---|---|
| `plugin/skills/impeccable/` | `upstream/skill/`, with `SKILL.md` renamed to `IMPECCABLE.md` |
| `skill/agents/` | `upstream/skill/agents/` |
| `tests/skill-reference.test.mjs` | `upstream/tests/skill-reference.test.mjs` |
| `tests/launcher-download.test.mjs` | `upstream/tests/launcher-download.test.mjs` |
| `LICENSE`, `NOTICE.md` | repository `LICENSE`, under Consolidated Upstream Notices |

The generated plugin distribution includes all command references, portable launcher scripts, runtime support assets, and degraded agent references. The source agent definitions are included for platforms without registered named agents. No engine binary, provider hook, or global configuration is installed by this package. The root SKILL.md is the Corvalis adaptation; vendored instructions are unchanged. Renaming the upstream entrypoint prevents recursive skill discovery from bypassing the adapter; references to its original filename resolve through the wrapper. [UPSTREAM.json](UPSTREAM.json) maps every preserved file to its source and SHA-256 digest.

Run the preserved checks from this skill directory:

```sh
node --test upstream/tests/skill-reference.test.mjs upstream/tests/launcher-download.test.mjs
```

The launcher suite uses a local fixture server and temporary fake engine, including checksum-failure cases; it does not fetch or execute a released engine. These checks cover the vendored reference/launcher contracts, not the upstream web application, Rust engine, or live browser integration.

The repository [LICENSE](../../LICENSE) contains the upstream notices in full. This skill has no separate license file. `UPSTREAM.json` records the original notice hashes and their marked sections in that root file.
