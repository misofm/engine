/** Issue #320: prove the packed SDK is self-contained and boots its embedded Wasm. */

import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { access, copyFile, mkdir, readFile, readdir, symlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";

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
assert.deepEqual(manifest.bin, { enginectl: "./dist/enginectl.js" });

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
for (const [subpath, module] of Object.entries(imported)) {
  assert.equal(
    "canonicalSessionJson" in module,
    false,
    `${subpath} must not expose arbitrary-model canonical serialization`,
  );
  assert.equal(
    "writeCanonicalSessionDocument" in module,
    false,
    `${subpath} must not expose the internal serializer test hook`,
  );
}
for (const assetUrl of Object.values(imported["./assets"].BUNDLED_ENGINE_ASSETS)) {
  await readFile(assetUrl);
}

const files = await readdir(packageRoot, { recursive: true });
assert.ok(files.includes("dist/assets/miso-engine-v1-audio-worklet.simd128.wasm"));
assert.ok(files.includes("dist/assets/flac-decoder.wasm"));
assert.ok(files.includes("dist/assets/flac-decoder.js"));
assert.ok(files.includes("dist/assets/flac-decoder.d.ts"));
assert.ok(files.includes("dist/assets/decoder-artifact.sha256"));
assert.ok(files.includes("dist/LICENSE"));
assert.equal(files.some((name) => name.includes("node_modules")), false);
assert.equal(files.some((name) => name.startsWith("test") || name.includes("/test/")), false);
assert.equal(files.includes("dist/effect.js"), false);
assert.equal(files.includes("dist/effect.d.ts"), false);
const shippedWorklet = await readFile(
  resolve(packageRoot, "dist/assets/miso-engine-v1-audio-worklet.js"),
  "utf8",
);
assert.match(shippedWorklet, /class MisoEngineAudioWorkletProcessor extends AudioWorkletProcessor/);
assert.doesNotMatch(
  shippedWorklet,
  /class MisoEngineV[0-9]+AudioWorkletProcessor/,
  "the shipped implementation class is private and therefore unversioned",
);

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
// @ts-expect-error arbitrary-model canonical serialization is intentionally not public
import { canonicalSessionJson } from "@misofm/engine";
void [CATALOG, session, createOfflineEngine, loadBundledEngineAsset, createEngine,
  BUNDLED_ENGINE_ASSETS, canonicalSessionJson];
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

const builtDocument = imported["."].session({ id: "tarball.boot", sampleRateHz: 48_000 })
  .source("stem", {
    channels: 2, bitDepth: "32f", frames: 480, content: `sha256:${"0".repeat(64)}`,
  })
  .track("track", { source: "stem" })
  .output("main")
  .route({
    id: "main",
    source: { kind: "track", trackId: "track", tap: "post_matrix" },
    destination: { kind: "output_input", outputId: "main" },
  })
  .toJson();
const engine = await imported["./headless"].createOfflineEngine(builtDocument);
const sibling = await imported["./headless"].createOfflineEngine(
  builtDocument.replace('"session_id": "tarball.boot"', '"session_id": "tarball.sibling"'),
);
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

// The packed executable uses only this extraction's embedded artifacts and emits raw JSON.
const cliRequest = JSON.stringify({
  schemaVersion: 1,
  session: { id: "tarball.cli", sampleRateHz: 48_000 },
  sources: [{
    id: "stem",
    spec: { channels: 2, bitDepth: "32f", frames: 480, content: `sha256:${"0".repeat(64)}` },
  }],
  tracks: [{ id: "track", spec: { source: "stem" } }],
  outputs: ["main"],
  routes: [{
    id: "main",
    source: { kind: "track", trackId: "track", tap: "post_matrix" },
    destination: { kind: "output_input", outputId: "main" },
  }],
});
const cli = spawn(
  process.execPath,
  [resolve(packageRoot, manifest.bin.enginectl), "session", "build", "--request", "-", "--output", "-"],
  {
    cwd: packageRoot,
    env: { PATH: "", HOME: resolve(packageRoot, "..", "no-home") },
    stdio: ["pipe", "pipe", "pipe"],
  },
);
const cliStdout = [];
const cliStderr = [];
cli.stdout.on("data", (chunk) => cliStdout.push(chunk));
cli.stderr.on("data", (chunk) => cliStderr.push(chunk));
cli.stdin.end(cliRequest);
const cliStatus = await new Promise((accept, reject) => {
  cli.once("error", reject);
  cli.once("close", accept);
});
assert.equal(cliStatus, 0, Buffer.concat(cliStderr).toString("utf8"));
assert.equal(Buffer.concat(cliStderr).byteLength, 0);
const cliDocument = Buffer.concat(cliStdout).toString("utf8");
assert.match(cliDocument, /^\{\n  "schema_version": 1,\n/);
assert.equal(cliDocument.endsWith("\n"), true, "the request-mode snapshot is canonical JSON");
const expectedCliDocument = imported["."].session({ id: "tarball.cli", sampleRateHz: 48_000 })
  .source("stem", {
    channels: 2, bitDepth: "32f", frames: 480, content: `sha256:${"0".repeat(64)}`,
  })
  .track("track", { source: "stem" })
  .output("main")
  .route({
    id: "main",
    source: { kind: "track", trackId: "track", tap: "post_matrix" },
    destination: { kind: "output_input", outputId: "main" },
  })
  .toJson();
assert.equal(cliDocument, expectedCliDocument, "request mode publishes the package writer's canonical snapshot");

async function bootSnapshotSubmitRender(document, sourceId, channels) {
  const snapshot = `${document}`;
  assert.match(snapshot, /^\{\n  "schema_version": 1,\n/);
  const offline = await imported["./headless"].createOfflineEngine(snapshot);
  try {
    const frames = offline.shape().quantumFrames;
    const submitted = offline.submitSource({
      sourceId,
      generation: 1n,
      startFrame: 0n,
      planes: Array.from({ length: channels }, (_unused, channel) =>
        Float32Array.from({ length: frames }, (_sample, frame) => (frame + channel + 1) / frames)),
      endOfRegion: false,
    });
    assert.equal(submitted.ok, true, `packed snapshot source refused: ${submitted.code}`);
    const rendered = offline.render();
    assert.equal(rendered.left.length, frames);
    assert.equal(rendered.right.length, frames);
    assert.equal(rendered.left.some((sample) => sample !== 0), true);
    return snapshot;
  } finally {
    offline.dispose();
  }
}

await bootSnapshotSubmitRender(cliDocument, "stem", 2);

// The extracted executable imports a FLAC leaf using only this package's decoder closure. The
// fixture is copied before launch; the command runs with the package as cwd and never discovers
// repository tools or files.
const stemsRoot = resolve(packageRoot, "..", "packed-stems");
await mkdir(stemsRoot);
await copyFile(
  resolve(import.meta.dirname, "../../fixtures/flac-delivery/v1/flac/pcm24-stereo-boundaries-b32.flac"),
  resolve(stemsRoot, "Packed Stem.flac"),
);
const stemsOutput = resolve(packageRoot, "..", "packed.session.json");
const stemsCli = spawn(
  process.execPath,
  [resolve(packageRoot, manifest.bin.enginectl), "session", "build", "--stems", stemsRoot, "--output", stemsOutput],
  { cwd: packageRoot, env: { PATH: "", HOME: resolve(packageRoot, "..", "no-home") }, stdio: ["ignore", "pipe", "pipe"] },
);
const stemsStdout = [];
const stemsStderr = [];
stemsCli.stdout.on("data", (chunk) => stemsStdout.push(chunk));
stemsCli.stderr.on("data", (chunk) => stemsStderr.push(chunk));
const stemsStatus = await new Promise((accept, reject) => {
  stemsCli.once("error", reject);
  stemsCli.once("close", accept);
});
assert.equal(stemsStatus, 0, Buffer.concat(stemsStderr).toString("utf8"));
assert.equal(Buffer.concat(stemsStderr).byteLength, 0);
const stemsReceipt = JSON.parse(Buffer.concat(stemsStdout).toString("utf8"));
assert.deepEqual(stemsReceipt.session, {
  id: "packed-stems", revision: 0, sampleRateHz: 48_000, quantumFrames: 128, sources: 1, tracks: 1,
});
assert.equal(stemsReceipt.stems[0].content, "sha256:f5868e05edf12c6032419ce0d7d786c9dc781989ac69ed17e8a4a374341f92f3");
const stemsDocument = await readFile(stemsOutput, "utf8");
assert.match(stemsDocument, /^\{\n  "schema_version": 1,\n/);
await bootSnapshotSubmitRender(stemsDocument, stemsReceipt.stems[0].sourceId, 2);

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

// The decoder is independently sealed. Mutation must fail internally before a generated session
// can be published, with no repository decoder or media subprocess available as a fallback.
const decoderUrl = imported["./assets"].BUNDLED_ENGINE_ASSETS.flacDecoderWasm;
const decoderOriginal = await readFile(decoderUrl);
const decoderChanged = Buffer.from(decoderOriginal);
decoderChanged[decoderChanged.length - 1] ^= 1;
await writeFile(decoderUrl, decoderChanged);
try {
  const rejectedOutput = resolve(packageRoot, "..", "decoder-mutation.session.json");
  const rejected = spawn(
    process.execPath,
    [resolve(packageRoot, manifest.bin.enginectl), "session", "build", "--stems", stemsRoot, "--output", rejectedOutput],
    {
      cwd: packageRoot,
      env: { PATH: "", HOME: resolve(packageRoot, "..", "no-home") },
      stdio: ["ignore", "pipe", "pipe"],
    },
  );
  const rejectedStdout = [];
  const rejectedStderr = [];
  rejected.stdout.on("data", (chunk) => rejectedStdout.push(chunk));
  rejected.stderr.on("data", (chunk) => rejectedStderr.push(chunk));
  const rejectedStatus = await new Promise((accept, reject) => {
    rejected.once("error", reject);
    rejected.once("close", accept);
  });
  assert.equal(rejectedStatus, 70);
  assert.equal(Buffer.concat(rejectedStdout).byteLength, 0);
  assert.match(Buffer.concat(rejectedStderr).toString("utf8"), /internal\.packaged_decoder/);
  await assert.rejects(access(rejectedOutput), (error) => error?.code === "ENOENT");
} finally {
  await writeFile(decoderUrl, decoderOriginal);
}
