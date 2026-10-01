const assert = require('node:assert/strict');
const { createHash } = require('node:crypto');
const { existsSync, readFileSync, readdirSync } = require('node:fs');
const path = require('node:path');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const bundles = ['auto-chat-quality', 'auto-code-quality', 'auto-writing-quality', 'auto-design-quality', 'auto-security-quality'];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');

for (const name of bundles) {
  const base = path.join(root, 'skills', name);
  test(`${name} preserves pinned upstream content and consolidated notices`, () => {
    const manifest = JSON.parse(readFileSync(path.join(base, 'UPSTREAM.json'), 'utf8'));
    assert.match(manifest.repository, /^https:\/\/github\.com\/[\w-]+\/[\w-]+$/);
    assert.match(manifest.commit, /^[a-f0-9]{40}$/);
    assert.ok(manifest.files.length > 0);
    assert.equal(new Set(manifest.files.map(file => file.path)).size, manifest.files.length);
    for (const file of manifest.files) {
      const absolute = path.resolve(base, file.path);
      assert.ok(absolute.startsWith(`${base}${path.sep}`), `bundle path escapes: ${file.path}`);
      assert.equal(digest(readFileSync(absolute)), file.sha256, file.path);
    }
    for (const notice of manifest.notices) {
      const text = readFileSync(path.resolve(base, notice.path), 'utf8');
      const start = `<!-- upstream-notice-start ${notice.section} -->\n`;
      const end = `<!-- upstream-notice-end ${notice.section} -->`;
      assert.equal(text.split(start).length, 2, `missing or duplicate notice ${notice.section}`);
      const content = text.split(start)[1].split(end);
      assert.equal(content.length, 2, `missing notice terminator ${notice.section}`);
      assert.equal(digest(content[0]), notice.sha256, notice.section);
    }
  });

  test(`${name} has one automatic entrypoint and resolvable wrapper references`, () => {
    const all = readdirSync(base, { recursive: true });
    assert.deepEqual(all.filter(file => path.basename(file) === 'SKILL.md'), ['SKILL.md']);
    const skill = readFileSync(path.join(base, 'SKILL.md'), 'utf8');
    assert.match(skill, new RegExp(`^---\\nname: ${name}\\n`, 'u'));
    assert.doesNotMatch(skill.split('---')[1], /disable-model-invocation:\s*true/);
    const metadata = path.join(base, 'agents/openai.yaml');
    if (existsSync(metadata)) assert.doesNotMatch(readFileSync(metadata, 'utf8'), /allow_implicit_invocation:\s*false/);
    for (const file of ['SKILL.md', 'UPSTREAM.md']) {
      for (const match of readFileSync(path.join(base, file), 'utf8').matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
        const target = match[1].split('#')[0];
        if (!target || /^[a-z]+:\/\//i.test(target)) continue;
        assert.ok(existsSync(path.resolve(base, target)), `${file}: missing ${target}`);
      }
    }
    assert.ok(!all.some(file => /^LICENSE(?:\.|$)|^NOTICE(?:\.|$)/.test(path.basename(file))), 'notices belong in the root LICENSE');
  });
}
