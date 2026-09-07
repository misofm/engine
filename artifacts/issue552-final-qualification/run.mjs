import { mkdir, writeFile, readFile, readdir, stat } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import { join, resolve } from 'node:path';

const repo = '/home/bl/misofm/engine-js-hex-recovery';
const evidence = '/tmp/issue552-final-qualification-luna';
const artifact = '/tmp/issue555-postpin-artifact';
await mkdir(evidence, { recursive: true });

function run(argv, cwd, dir) {
  return new Promise((resolveRun) => {
    const started = new Date().toISOString();
    const child = spawn(argv[0], argv.slice(1), { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    const out = [];
    const err = [];
    child.stdout.on('data', (b) => out.push(b));
    child.stderr.on('data', (b) => err.push(b));
    child.on('close', async (code, signal) => {
      await writeFile(join(dir, 'stdout.raw'), Buffer.concat(out));
      await writeFile(join(dir, 'stderr.raw'), Buffer.concat(err));
      await writeFile(join(dir, 'status.txt'), `${code ?? 'null'}${signal ? ` signal=${signal}` : ''}\n`);
      await writeFile(join(dir, 'argv.txt'), `${JSON.stringify(argv)}\n`);
      await writeFile(join(dir, 'cwd.txt'), `${cwd}\n`);
      await writeFile(join(dir, 'started.txt'), `${started}\n`);
      resolveRun(code ?? 1);
    });
  });
}

const gates = [
  { name: '01-direct', argv: ['node', '--experimental-strip-types', '/tmp/issue552/attempt3-direct.mjs'], cwd: repo },
  { name: '02-stem-store', argv: ['node', 'scripts/check-stem-store-v1.mjs'], cwd: repo },
  { name: '03-stem-store-self-test', argv: ['node', 'scripts/check-stem-store-v1.mjs', '--self-test'], cwd: repo },
  { name: '04-sdk-types', argv: ['bash', 'scripts/check-sdk-types.sh'], cwd: repo },
  { name: '05-sdk-headless', argv: ['bash', 'scripts/check-sdk-headless.sh', artifact], cwd: repo },
  { name: '06-sdk-package', argv: ['bash', 'scripts/sdk-package.sh', 'check', artifact], cwd: repo },
  { name: '07-session-identities', argv: ['npm', 'run', 'session-identities'], cwd: join(repo, 'hosts/host-web/qualification') },
  { name: '08-qualification', argv: ['npm', 'run', 'qualify', '--', '--artifacts', artifact, '--browser', 'chromium', '--check-matrix', '--self-test-mutations'], cwd: join(repo, 'hosts/host-web/qualification') },
  { name: '09-workspace-policy', argv: ['bash', 'scripts/check-workspace-policy.sh'], cwd: repo },
];

for (const gate of gates) {
  const dir = join(evidence, gate.name);
  await mkdir(dir, { recursive: true });
  const code = await run(gate.argv, gate.cwd, dir);
  await writeFile(join(dir, 'result.json'), JSON.stringify({ name: gate.name, argv: gate.argv, cwd: gate.cwd, status: code }, null, 2) + '\n');
  if (code !== 0) {
    await writeFile(join(evidence, 'terminal-report.txt'), `STOPPED at ${gate.name}; status ${code}.\n`);
    process.exitCode = code;
    process.exit();
  }
}

await writeFile(join(evidence, 'terminal-report.txt'), 'Gates 01-09 passed; gates 10-11 require the exact audit/census command records.\n');
