import { lstat, readlink, unlink } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
const target = '/home/bl/misofm/engine-js-hex-recovery/hosts/host-web/qualification/node_modules';
const source = '/home/bl/misofm/engine/hosts/host-web/qualification/node_modules';
const mode = process.argv[2];
if (mode === 'verify-exists') {
  const s = await lstat(target); if (!s.isSymbolicLink()) throw new Error('node_modules is not a symlink'); process.stdout.write('symlink exists\n');
} else if (mode === 'verify-target') {
  const s = await lstat(target); if (!s.isSymbolicLink()) throw new Error('node_modules is not a symlink'); const value = await readlink(target); process.stdout.write(`${value}\n`); if (value !== source) throw new Error(`unexpected symlink target: ${value}`);
} else if (mode === 'unlink') {
  const s = await lstat(target); if (!s.isSymbolicLink()) throw new Error('node_modules is not a symlink'); const value = await readlink(target); if (value !== source) throw new Error(`unexpected symlink target: ${value}`); await unlink(target); process.stdout.write('symlink removed\n');
} else if (mode === 'verify-absent') {
  try { await lstat(target); throw new Error('node_modules still exists'); } catch (error) { if (error.code !== 'ENOENT') throw error; } process.stdout.write('symlink absent\n');
} else if (mode === 'final-clean') {
  const cwd = '/home/bl/misofm/engine-js-hex-recovery';
  const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd, encoding: 'utf8' });
  const status = execFileSync('git', ['status', '--short', '--branch'], { cwd, encoding: 'utf8' });
  execFileSync('git', ['diff', '--check'], { cwd, encoding: 'utf8' });
  if (status !== '## codex/js-hex-recovery...origin/codex/js-hex-recovery\n') throw new Error(`worktree not clean: ${status}`);
  process.stdout.write(`HEAD=${head}STATUS=${status}diff-check=PASS\n`);
} else throw new Error(`unknown cleanup mode: ${mode}`);
