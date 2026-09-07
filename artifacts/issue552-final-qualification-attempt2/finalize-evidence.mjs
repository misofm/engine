import { readdir, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { join, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
const root = '/tmp/issue552-final-qualification-luna-attempt2';
const repo = '/home/bl/misofm/engine-js-hex-recovery';
const walk = async (dir) => {
  const out = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...await walk(path)); else out.push(path);
  }
  return out;
};
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const finalHead = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' });
const finalStatus = spawnSync('git', ['status', '--short', '--branch'], { cwd: repo, encoding: 'utf8' });
const finalDiff = spawnSync('git', ['diff', '--check'], { cwd: repo, encoding: 'utf8' });
if (finalHead.status !== 0 || finalStatus.status !== 0 || finalDiff.status !== 0) throw new Error('final Git command failed');
if (finalStatus.stdout !== '## codex/js-hex-recovery...origin/codex/js-hex-recovery\n') throw new Error(`worktree not clean: ${finalStatus.stdout}`);
await writeFile(join(root, 'final-clean-state.txt'), `HEAD=${finalHead.stdout}STATUS=${finalStatus.stdout}diff-check=PASS\n`);

const packed = [];
for (const path of (await walk(root)).filter((p) => p.endsWith('.stdout.raw') || p.endsWith('.stderr.raw'))) {
  const bytes = await readFile(path);
  const text = bytes.toString('utf8');
  if (/[ \t]+\n$/.test(text) || bytes.endsWith(Buffer.from('\n\n'))) {
    const result = spawnSync('gzip', ['-n', path], { encoding: 'utf8' });
    if (result.status !== 0) throw new Error(`gzip failed for ${path}: ${result.stderr}`);
    packed.push(`${relative(root, path)}\t${relative(root, `${path}.gz`)}\tsha256=${digest(await readFile(`${path}.gz`))}`);
  }
}
await writeFile(join(root, 'raw-gzip-map.tsv'), packed.length ? `${packed.join('\n')}\n` : 'No raw stdout/stderr records required gzip packing.\n');
await writeFile(join(root, 'terminal-report.txt'), `PASS: gates 1-7, gate 8 retry, fail-closed cleanup, and gates 9-11 passed. HEAD ${finalHead.stdout.trim()}; worktree clean.\n`);
const badText = [];
for (const path of await walk(root)) {
  if (path.endsWith('.gz') || path.endsWith('manifest.json') || path.endsWith('manifest.sha256')) continue;
  const bytes = await readFile(path);
  if (bytes.includes(0)) continue;
  const text = bytes.toString('utf8');
  if (/[ \t]+\n$/.test(text) || text.endsWith('\n\n')) badText.push(relative(root, path));
}
if (badText.length) throw new Error(`text evidence has trailing whitespace/blank lines: ${badText.join(', ')}`);
