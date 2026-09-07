import { readdir, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { join, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
const root = '/tmp/issue552-final-qualification-luna-attempt2';
const walk = async (dir) => {
  const out = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...await walk(path)); else out.push(path);
  }
  return out;
};
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: '/home/bl/misofm/engine-js-hex-recovery', encoding: 'utf8' });
if (git.status !== 0) throw new Error(`git rev-parse failed: ${git.stderr}`);
const head = git.stdout.trim();
const entries = [];
for (const path of await walk(root)) {
  if (path.endsWith('manifest.json') || path.endsWith('manifest.sha256')) continue;
  const bytes = await readFile(path);
  entries.push({ path: relative(root, path), bytes: bytes.length, sha256: digest(bytes) });
}
entries.sort((a, b) => a.path.localeCompare(b.path));
await writeFile(join(root, 'manifest.json'), JSON.stringify({ schema: 1, git_head: head, files: entries }, null, 2) + '\n');
const manifest = await readFile(join(root, 'manifest.json'));
await writeFile(join(root, 'manifest.sha256'), `${digest(manifest)}  manifest.json\n`);
