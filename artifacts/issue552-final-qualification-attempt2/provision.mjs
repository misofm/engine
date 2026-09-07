import { readFile, writeFile, lstat, symlink } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const repo = '/home/bl/misofm/engine-js-hex-recovery';
const source = '/home/bl/misofm/engine/hosts/host-web/qualification/node_modules';
const target = `${repo}/hosts/host-web/qualification/node_modules`;
const sha = async (path) => createHash('sha256').update(await readFile(path)).digest('hex');
const manifest = async (base) => ({ package_json: await sha(`${base}/package.json`), package_lock_json: await sha(`${base}/package-lock.json`) });
try { await lstat(target); throw new Error(`target already exists: ${target}`); } catch (error) { if (error.code !== 'ENOENT') throw error; }
const targetManifest = await manifest(`${repo}/hosts/host-web/qualification`);
const sourceManifest = await manifest('/home/bl/misofm/engine/hosts/host-web/qualification');
if (JSON.stringify(targetManifest) !== JSON.stringify(sourceManifest)) throw new Error('package manifest/lock hashes differ');
const playwright = `${source}/playwright/package.json`;
const playwrightCore = `${source}/playwright-core/package.json`;
const playwrightVersion = JSON.parse(await readFile(playwright, 'utf8')).version;
const playwrightCoreVersion = JSON.parse(await readFile(playwrightCore, 'utf8')).version;
if (playwrightVersion !== '1.62.1' || playwrightCoreVersion !== '1.62.1') throw new Error('Playwright version mismatch');
const details = {
  decision_owner: 'root', decision: 'Use one temporary symlink to existing byte-identical pinned dependencies; no installation or update.',
  source_path: source, target_path: target, target_manifest: targetManifest, source_manifest: sourceManifest,
  playwright: { version: playwrightVersion, package_json_sha256: await sha(playwright) },
  playwright_core: { version: playwrightCoreVersion, package_json_sha256: await sha(playwrightCore) },
};
await writeFile('/tmp/issue552-final-qualification-luna-attempt2/provisioning-details.json', JSON.stringify(details, null, 2) + '\n');
await symlink(source, target);
process.stdout.write(JSON.stringify(details) + '\n');
