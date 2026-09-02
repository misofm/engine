/** Issue #320: prove the packed SDK is self-contained and boots its embedded Wasm. */

import assert from "node:assert/strict";
import { mkdir, readFile, readdir, symlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";

import { sessionDocument } from "./support.mjs";

if (process.argv.length !== 3) throw new Error("usage: node package-tarball-smoke.mjs PACKAGE_ROOT");
const packageRoot = resolve(process.argv[2]);
const manifest = JSON.parse(await readFile(resolve(packageRoot, "package.json"), "utf8"));
assert.equal(manifest.name, "@misofm/engine");
assert.equal(manifest.private, undefined);
assert.deepEqual(Object.keys(manifest.exports), [
  ".", "./headless", "./browser", "./assets", "./package.json",
]);
assert.equal(manifest.dependencies, undefined);
assert.equal(manifest.peerDependencies, undefined);
assert.equal(manifest.peerDependenciesMeta, undefined);
assert.equal(manifest.devDependencies?.effect, undefined);

const imported = {};
for (const subpath of [".", "./headless", "./browser", "./assets"]) {
  const target = manifest.exports[subpath];
  assert.equal(typeof target.import, "string", `${subpath} has an ESM target`);
  assert.equal(typeof target.types, "string", `${subpath} has a declaration target`);
  await readFile(resolve(packageRoot, target.types));
  imported[subpath] = await import(pathToFileURL(resolve(packageRoot, target.import)).href);
}
assert.equal(typeof imported["."].session, "function");
assert.equal(typeof imported["./headless"].createOfflineEngine, "function");
assert.equal(typeof imported["./browser"].createEngine, "function");
assert.ok(imported["./assets"].BUNDLED_ENGINE_ASSETS.wasm instanceof URL);
for (const assetUrl of Object.values(imported["./assets"].BUNDLED_ENGINE_ASSETS)) {
  await readFile(assetUrl);
}

const files = await readdir(packageRoot, { recursive: true });
assert.ok(files.includes("dist/assets/miso-engine-v1-audio-worklet.simd128.wasm"));
assert.ok(files.includes("dist/LICENSE"));
assert.equal(files.some((name) => name.includes("node_modules")), false);
assert.equal(files.some((name) => name.startsWith("test") || name.includes("/test/")), false);
assert.equal(files.includes("dist/effect.js"), false);
assert.equal(files.includes("dist/effect.d.ts"), false);

// Resolve the declaration surface exactly as a consumer does. Reading `.d.ts` files is not enough:
// a declaration can exist while importing a sibling TypeScript never emitted (the old failure).
const consumerRoot = resolve(packageRoot, "..", "consumer");
const scopeRoot = resolve(consumerRoot, "node_modules", "@misofm");
await mkdir(scopeRoot, { recursive: true });
await symlink(packageRoot, resolve(scopeRoot, "engine"), "dir");
const consumer = resolve(consumerRoot, "index.ts");
await writeFile(consumer, `
import { CATALOG, session } from "@misofm/engine";
import { createOfflineEngine, loadBundledEngineAsset } from "@misofm/engine/headless";
import { createEngine } from "@misofm/engine/browser";
import { BUNDLED_ENGINE_ASSETS } from "@misofm/engine/assets";
void [CATALOG, session, createOfflineEngine, loadBundledEngineAsset, createEngine,
  BUNDLED_ENGINE_ASSETS];
`, "utf8");
const program = ts.createProgram([consumer], {
  module: ts.ModuleKind.NodeNext,
  moduleResolution: ts.ModuleResolutionKind.NodeNext,
  noEmit: true,
  skipLibCheck: false,
  strict: true,
  target: ts.ScriptTarget.ES2022,
  types: [],
  lib: ["lib.es2022.d.ts", "lib.dom.d.ts"],
});
const diagnostics = ts.getPreEmitDiagnostics(program);
assert.deepEqual(
  diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")),
  [],
  "a fresh strict TypeScript consumer resolves every declaration dependency",
);

const engine = await imported["./headless"].createOfflineEngine(sessionDocument());
const sibling = await imported["./headless"].createOfflineEngine(sessionDocument({ sessionId: "sibling" }));
try {
  assert.equal(engine.asset.sha256?.length, 64);
  assert.equal(engine.asset, sibling.asset, "default engines share one verified compilation");
  assert.equal(engine.shape().sampleRateHz, 48_000);
  const block = engine.render();
  assert.equal(block.left.length, 128);
  assert.equal(block.right.length, 128);
} finally {
  engine.dispose();
  sibling.dispose();
}

// Red mutation: the package manifest still names the original digest, so one changed byte must be
// rejected before WebAssembly.compile can see it.
const wasmUrl = imported["./assets"].BUNDLED_ENGINE_ASSETS.wasm;
const original = await readFile(wasmUrl);
const changed = Buffer.from(original);
changed[changed.length - 1] ^= 1;
await writeFile(wasmUrl, changed);
try {
  await assert.rejects(
    imported["./headless"].loadBundledEngineAsset(),
    (error) => error?.name === "MisoEngineError" && error?.diagnosticCode === "sdk.asset.digest",
  );
} finally {
  await writeFile(wasmUrl, original);
}
