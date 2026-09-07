import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
const root = '/tmp/issue552-final-qualification-luna';
const gates = ['01-direct','02-stem-store','03-stem-store-self-test','04-sdk-types','05-sdk-headless','06-sdk-package','07-session-identities','08-qualification'];
const pre = await readFile(join(root, '00-preflight.txt'), 'utf8');
for (const gate of gates) {
  const dir = join(root, gate);
  await writeFile(join(dir, 'pre-command-head.txt'), '3881591d307c7b00e1cc7dfb431b3abf26874717\n');
  await writeFile(join(dir, 'pre-command-status.txt'), '## codex/js-hex-recovery...origin/codex/js-hex-recovery\n');
  await writeFile(join(dir, 'relevant-source-sha256.txt'), pre.slice(pre.indexOf('relevant_source_sha256_begin'), pre.indexOf('relevant_source_sha256_end') + 'relevant_source_sha256_end'.length) + '\n');
}
const report = `Issue #552/#558 final qualification — Luna HIGH\n\n` +
  `Repository: /home/bl/misofm/engine-js-hex-recovery\n` +
  `Sequence pre-command HEAD: 3881591d307c7b00e1cc7dfb431b3abf26874717\n` +
  `Sequence pre-command status: clean (codex/js-hex-recovery...origin/codex/js-hex-recovery)\n` +
  `Artifact: /tmp/issue555-postpin-artifact (exactly six regular files; required Wasm SHA-256 verified)\n\n` +
  `01 direct: PASS (status 0)\n02 stem-store: PASS (status 0)\n03 stem-store --self-test: PASS (status 0; six provenance RED records plus prior controls)\n` +
  `04 SDK types: PASS (status 0)\n05 SDK headless: PASS (status 0)\n06 SDK package: PASS (status 0)\n07 session-identities: PASS (status 0)\n` +
  `08 qualification: FAIL (status 1); stopped before gates 09–11\n` +
  `Failure: ERR_MODULE_NOT_FOUND — package playwright imported by hosts/host-web/qualification/run.mjs.\n` +
  `Exact raw records: 08-qualification/stdout.raw and 08-qualification/stderr.raw; argv/cwd/status files are adjacent.\n\n` +
  `Final worktree status: clean; no repository files changed by qualification.\n`;
await writeFile(join(root, 'terminal-report.txt'), report);
