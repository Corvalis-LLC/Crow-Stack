const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const { mkdtempSync, mkdirSync, writeFileSync, readFileSync, readdirSync, lstatSync, realpathSync, readlinkSync, symlinkSync, copyFileSync, rmSync } = require('node:fs');
const { tmpdir } = require('node:os');
const path = require('node:path');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const retired = ['design', 'ui-ux-pro-max', 'auto-coding', 'auto-sanity', 'auto-layout', 'auto-refactor'];
const skills = readdirSync(path.join(root, 'skills')).filter(name => {
  try { return lstatSync(path.join(root, 'skills', name, 'SKILL.md')).isFile(); }
  catch (error) { if (error.code === 'ENOENT' || error.code === 'ENOTDIR') return false; throw error; }
});

for (const installer of ['install.sh', 'install-codex.sh']) {
  test(`${installer} rejects its own source as a destination, including aliases`, t => {
    const scratch = mkdtempSync(path.join(tmpdir(), 'corvalis source guard '));
    t.after(() => rmSync(scratch, { recursive: true, force: true }));
    mkdirSync(path.join(scratch, 'tools/skills'), { recursive: true });
    mkdirSync(path.join(scratch, 'skills/example'), { recursive: true });
    writeFileSync(path.join(scratch, 'skills/example/SKILL.md'), 'original');
    copyFileSync(path.join(root, installer), path.join(scratch, installer));
    copyFileSync(path.join(root, 'tools/skills/install-common.sh'), path.join(scratch, 'tools/skills/install-common.sh'));
    symlinkSync(path.join(scratch, 'skills'), path.join(scratch, 'alias'));
    for (const destination of ['skills', 'alias']) {
      const result = spawnSync('bash', [path.join(scratch, installer), '--skills-only', '--skills-dir', path.join(scratch, destination)], { encoding: 'utf8' });
      assert.notEqual(result.status, 0, 'source installation must fail');
      assert.match(result.stderr, /source.*destination|destination.*source/i);
      assert.equal(readFileSync(path.join(scratch, 'skills/example/SKILL.md'), 'utf8'), 'original');
      assert.ok(!readdirSync(scratch).some(name => name.startsWith('skills-backup-')));
    }
  });

  test(`${installer} installs the full ecosystem, preserves conflicts, and is idempotent`, t => {
    const scratch = mkdtempSync(path.join(tmpdir(), 'corvalis install '));
    t.after(() => rmSync(scratch, { recursive: true, force: true }));
    const destination = path.join(scratch, 'skills');
    mkdirSync(path.join(destination, 'summon'), { recursive: true });
    writeFileSync(path.join(destination, 'summon', 'local.txt'), 'local customization');
    mkdirSync(path.join(destination, '.system'));
    writeFileSync(path.join(destination, '.system', 'keep'), 'system');
    symlinkSync('/missing/old-skill', path.join(destination, 'dominion'));
    symlinkSync(`${root}/skills/design`, path.join(destination, 'design'));
    for (const name of retired.filter(name => name !== 'design')) {
      mkdirSync(path.join(destination, name));
      writeFileSync(path.join(destination, name, 'local.txt'), `retired ${name} customization`);
    }
    symlinkSync(`${root}/skills/future/`, path.join(destination, 'future'));
    symlinkSync(`${root}/skills/auto-errors/`, path.join(destination, 'auto-errors'));
    const run = () => spawnSync('bash', [path.join(root, installer), '--skills-only', '--skills-dir', destination], { encoding: 'utf8' });
    const first = run();
    assert.equal(first.status, 0, first.stderr + first.stdout);
    for (const name of skills) {
      assert.equal(realpathSync(path.join(destination, name)), realpathSync(path.join(root, 'skills', name)), name);
    }
    assert.equal(readFileSync(path.join(destination, '.system', 'keep'), 'utf8'), 'system');
    assert.ok(!readdirSync(destination).includes('future'), 'non-skill grouping directories must not be installed');
    for (const name of retired) assert.ok(!readdirSync(destination).includes(name), `retired ${name} must not be discoverable`);
    const backups = readdirSync(scratch).filter(name => name.startsWith('skills-backup-'));
    assert.equal(backups.length, 1);
    const backup = path.join(scratch, backups[0]);
    assert.equal(readFileSync(path.join(backup, 'summon', 'local.txt'), 'utf8'), 'local customization');
    assert.equal(readlinkSync(path.join(backup, 'dominion')), '/missing/old-skill');
    for (const name of retired.filter(name => name !== 'design')) {
      assert.equal(readFileSync(path.join(backup, name, 'local.txt'), 'utf8'), `retired ${name} customization`);
    }
    assert.deepEqual(readdirSync(backup).sort(), [...retired, 'dominion', 'summon', 'future'].sort());
    const second = run();
    assert.equal(second.status, 0, second.stderr + second.stdout);
    assert.deepEqual(readdirSync(scratch).filter(name => name.startsWith('skills-backup-')), backups);
  });
}
