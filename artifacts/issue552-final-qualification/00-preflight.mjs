import { readdir, stat, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { join } from 'node:path';
import { execFileSync } from 'node:child_process';

const repo = '/home/bl/misofm/engine-js-hex-recovery';
const out = '/tmp/issue552-final-qualification-luna';
const artifact = '/tmp/issue555-postpin-artifact';
const names = await readdir(artifact);
const regular = [];
for (const name of names) {
  const s = await stat(join(artifact, name));
  if (s.isFile()) regular.push(name);
}
regular.sort();
const hashes = {};
for (const name of regular) hashes[name] = createHash('sha256').update(await readFile(join(artifact, name))).digest('hex');
const files = [
  'sdk/src/core/hex.ts', 'sdk/src/core/asset.ts', 'sdk/test/boot-evals.mjs',
  'hosts/host-web/web/hex-lower.js', 'hosts/host-web/web/stem-store/incremental-sha256.js',
  'hosts/host-web/qualification/qualification.js', 'hosts/host-web/qualification/server.mjs',
  'hosts/host-web/tests/stem-store-hash-v1.mjs',
  'hosts/host-web/web/stem-store/incremental-sha256.provenance.json', 'scripts/check-stem-store-v1.mjs',
];
const sourceHashes = {};
for (const file of files) sourceHashes[file] = createHash('sha256').update(await readFile(join(repo, file))).digest('hex');
const command = (args) => execFileSync(args[0], args.slice(1), { cwd: repo, encoding: 'utf8' });
const head = command(['git', 'rev-parse', 'HEAD']).trim();
const status = command(['git', 'status', '--short', '--branch']);
const report = [
  `repo=${repo}`,
  `artifact=${artifact}`,
  `artifact_regular_count=${regular.length}`,
  `artifact_regular_names=${JSON.stringify(regular)}`,
  `artifact_hashes=${JSON.stringify(hashes, null, 2)}`,
  `expected_wasm=6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`,
  `pre_command_head=${head}`,
  'pre_command_status_begin', status.trimEnd(), 'pre_command_status_end',
  'relevant_source_sha256_begin', JSON.stringify(sourceHashes, null, 2), 'relevant_source_sha256_end',
  `original_base=e684275cb349e49a66e73f7f030da6520dd63607`,
  `current_base=origin/main (${command(['git', 'rev-parse', 'origin/main']).trim()})`,
].join('\n') + '\n';
await writeFile(join(out, '00-preflight.txt'), report);
if (regular.length !== 6 || hashes['miso-engine-v1-audio-worklet.simd128.wasm'] !== '6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c') process.exit(2);
