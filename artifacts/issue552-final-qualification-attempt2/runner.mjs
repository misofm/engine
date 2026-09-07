import { mkdir, writeFile, readFile, readdir, lstat } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { spawn } from 'node:child_process';
import { execFileSync } from 'node:child_process';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';

const repo = '/home/bl/misofm/engine-js-hex-recovery';
const evidence = '/tmp/issue552-final-qualification-luna-attempt2';
const artifact = '/tmp/issue555-postpin-artifact';
const sourcePaths = [
  'sdk/src/core/hex.ts', 'sdk/src/core/asset.ts', 'sdk/test/boot-evals.mjs',
  'hosts/host-web/web/hex-lower.js', 'hosts/host-web/web/stem-store/incremental-sha256.js',
  'hosts/host-web/qualification/qualification.js', 'hosts/host-web/qualification/server.mjs',
  'hosts/host-web/tests/stem-store-hash-v1.mjs',
  'hosts/host-web/web/stem-store/incremental-sha256.provenance.json', 'scripts/check-stem-store-v1.mjs',
];
const qualificationCwd = join(repo, 'hosts/host-web/qualification');
await mkdir(evidence, { recursive: true });

const hash = async (path) => createHash('sha256').update(await readFile(path)).digest('hex');
const artifactInventory = async () => {
  const names = (await readdir(artifact)).sort();
  const files = [];
  for (const name of names) {
    const path = join(artifact, name);
    const s = await lstat(path);
    if (s.isFile()) files.push({ name, bytes: s.size, sha256: await hash(path) });
  }
  return { directory: artifact, regular_file_count: files.length, files };
};
const preState = async (cwd) => {
  const actualCwd = execFileSync('pwd', { cwd, encoding: 'utf8' });
  const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd, encoding: 'utf8' });
  const status = execFileSync('git', ['status', '--short', '--branch'], { cwd, encoding: 'utf8' });
  const sources = {};
  for (const path of sourcePaths) sources[path] = await hash(join(repo, path));
  return { utc_timestamp: new Date().toISOString(), cwd: actualCwd, head, status, source_sha256: sources, artifact_inventory: await artifactInventory() };
};
const runStep = async (name, argv, cwd = repo) => {
  const dir = join(evidence, name);
  await mkdir(dir, { recursive: true });
  const pre = await preState(cwd);
  await writeFile(join(dir, 'argv.json'), JSON.stringify(argv) + '\n');
  await writeFile(join(dir, 'pre-command.json'), JSON.stringify(pre, null, 2) + '\n');
  const child = spawn(argv[0], argv.slice(1), { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
  const out = [], err = [];
  child.stdout.on('data', (b) => out.push(b));
  child.stderr.on('data', (b) => err.push(b));
  return await new Promise((resolve) => child.on('close', async (exitCode, signal) => {
    await writeFile(join(dir, 'stdout.raw'), Buffer.concat(out));
    await writeFile(join(dir, 'stderr.raw'), Buffer.concat(err));
    await writeFile(join(dir, 'status.json'), JSON.stringify({ exit_code: exitCode, signal }, null, 2) + '\n');
    resolve({ exitCode: exitCode ?? 1, signal });
  }));
};
const stop = async (label, status) => {
  await writeFile(join(evidence, 'terminal-report.txt'), `STOPPED at ${label}; status ${status}.\n`);
  process.exitCode = status || 1;
  process.exit();
};
const steps = [
  ['01-direct', ['node', '--experimental-strip-types', '/tmp/issue552/attempt3-direct.mjs'], repo],
  ['02-stem-store', ['node', 'scripts/check-stem-store-v1.mjs'], repo],
  ['03-stem-store-self-test', ['node', 'scripts/check-stem-store-v1.mjs', '--self-test'], repo],
  ['04-sdk-types', ['bash', 'scripts/check-sdk-types.sh'], repo],
  ['05-sdk-headless', ['bash', 'scripts/check-sdk-headless.sh', artifact], repo],
  ['06-sdk-package', ['bash', 'scripts/sdk-package.sh', 'check', artifact], repo],
  ['07-session-identities', ['npm', 'run', 'session-identities'], qualificationCwd],
];
for (const [name, argv, cwd] of steps) {
  const result = await runStep(name, argv, cwd);
  if (result.exitCode !== 0) await stop(name, result.exitCode);
}
let result = await runStep('provision-dependency', ['node', join(evidence, 'provision.mjs')], repo);
if (result.exitCode !== 0) await stop('provisioning', result.exitCode);
result = await runStep('08-qualification-retry', ['npm', 'run', 'qualify', '--', '--artifacts', artifact, '--browser', 'chromium', '--check-matrix', '--self-test-mutations'], qualificationCwd);
const cleanup = [
  ['cleanup-01-verify-exists', 'verify-exists'], ['cleanup-02-verify-target', 'verify-target'],
  ['cleanup-03-unlink', 'unlink'], ['cleanup-04-verify-absent', 'verify-absent'], ['cleanup-05-final-clean', 'final-clean'],
];
for (const [name, mode] of cleanup) {
  const cleanupResult = await runStep(name, ['node', join(evidence, 'cleanup-step.mjs'), mode], repo);
  if (cleanupResult.exitCode !== 0) await stop(name, cleanupResult.exitCode);
}
if (result.exitCode !== 0) await stop('08-qualification-retry', result.exitCode);
result = await runStep('09-workspace-policy', ['bash', 'scripts/check-workspace-policy.sh'], repo);
if (result.exitCode !== 0) await stop('gate 09', result.exitCode);
result = await runStep('10-diff-census', ['bash', join(evidence, 'gate10-diff-census.sh')], repo);
if (result.exitCode !== 0) await stop('gate 10', result.exitCode);
result = await runStep('11-semantic-census', ['bash', join(evidence, 'gate11-semantic-census.sh')], repo);
if (result.exitCode !== 0) await stop('gate 11', result.exitCode);
result = await runStep('finalize-evidence', ['node', join(evidence, 'finalize-evidence.mjs')], repo);
if (result.exitCode !== 0) await stop('evidence finalization', result.exitCode);
const manifestResult = spawnSync('node', [join(evidence, 'refresh-manifest.mjs')], { cwd: repo, encoding: 'utf8' });
if (manifestResult.status !== 0) await stop('manifest generation', manifestResult.status ?? 1);
