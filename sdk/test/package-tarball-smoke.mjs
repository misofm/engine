/** Issue #320: prove the packed SDK is self-contained and boots its embedded Wasm. */

import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { access, mkdir, readFile, readdir, symlink, writeFile } from "node:fs/promises";
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
assert.equal(typeof imported["./browser"].scratchBootWithWorker, "function");
assert.equal(typeof imported["./browser"].createDefaultHost, "function");
const workerText = await readFile(imported["./assets"].BUNDLED_ENGINE_ASSETS.scratchWorkerModule, "utf8");
const workerAst = ts.createSourceFile("scratch-worker.js", workerText, ts.ScriptTarget.ES2022, true, ts.ScriptKind.JS);
assert.equal(workerAst.statements.some(statement => ts.isImportDeclaration(statement)
  || (ts.isExportDeclaration(statement) && statement.moduleSpecifier !== undefined)), false,
"the exported Worker URL is import-complete when a consumer bundler copies it");
assert.equal(typeof imported["./browser"].attachEngineFeed, "function");
assert.equal(typeof imported["./browser"].prepareEngineFeed, "function");
assert.ok(imported["./assets"].BUNDLED_ENGINE_ASSETS.wasm instanceof URL);
assert.ok(imported["./assets"].BUNDLED_ENGINE_ASSETS.pcmFeedWorklet instanceof URL);
const shippedNotice = await readFile(resolve(packageRoot, "dist/NOTICE"), "utf8");
assert.match(shippedNotice, /engine-web-adapter/);
assert.match(shippedNotice, /bd7f330a9773ce43bb077f0e6d5c8fc30fe9e27c/);
assert.match(shippedNotice, /7485693e9bbcf2f65a91a4e5950e22d678d99062/);
assert.match(shippedNotice, /63b4ee6212287000ff85e1cfa969d385f6246d2d/);
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
const assetManifest = JSON.parse(await readFile(imported["./assets"].BUNDLED_ENGINE_ASSETS.manifest, "utf8"));
assert.deepEqual(
  Object.keys(assetManifest.artifacts).sort(),
  [
    "miso-engine-v1-abi-layout.json",
    "miso-engine-v1-audio-worklet-host.d.ts",
    "miso-engine-v1-audio-worklet-host.js",
    "miso-engine-v1-audio-worklet.js",
    "miso-engine-v1-audio-worklet.simd128.wasm",
    "miso-engine-v1-parameter-metadata.json",
  ],
  "the package manifest declares exactly the Engine artifact closure",
);

const files = await readdir(packageRoot, { recursive: true });
assert.ok(files.includes("dist/assets/miso-engine-v1-audio-worklet.simd128.wasm"));
assert.equal(files.some((name) => /flac|decoder|cli\/stems/i.test(name)), false, "the archive has no retired delivery payload");
assert.ok(files.includes("dist/LICENSE"));
assert.ok(files.includes("dist/NOTICE"));
assert.ok(files.includes("dist/assets/miso-engine-v1-pcm-feed-worklet.js"));
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
import { createEngine, prepareEngineFeed, attachEngineFeed, Msb1RingWriter, Msb1RingObserver } from "@misofm/engine/browser";
import type { BrowserEngine, PcmSourceChunk } from "@misofm/engine/browser";
import { BUNDLED_ENGINE_ASSETS } from "@misofm/engine/assets";
// @ts-expect-error arbitrary-model canonical serialization is intentionally not public
import { canonicalSessionJson } from "@misofm/engine";
void [CATALOG, session, createOfflineEngine, loadBundledEngineAsset, createEngine,
  prepareEngineFeed, attachEngineFeed, BUNDLED_ENGINE_ASSETS, canonicalSessionJson];
declare const domContext: BaseAudioContext;
const domFactory = (context: BaseAudioContext, name: string, options: AudioWorkletNodeOptions): AudioWorkletNode =>
  new AudioWorkletNode(context, name, options);
const packedFeed = attachEngineFeed({
  context: domContext,
  sources: [{ sourceId: "packed-source", channels: 2 }],
  quantumFrames: 128,
  createNode: domFactory,
});
void packedFeed.rings;
const packedWriter = new Msb1RingWriter(packedFeed.rings[0]);
packedWriter.engage(1n);
void packedWriter;
const packedObserver = new Msb1RingObserver(packedFeed.rings[0]);
packedObserver.pull((chunk: PcmSourceChunk) => {
  const generation: bigint = chunk.generation;
  const planes: readonly Float32Array[] = chunk.planes;
  const frames: number = chunk.frames;
  // @ts-expect-error Borrowed metadata is read-only.
  chunk.frames = 3;
  void [generation, planes, frames];
}, 1);
const observedCounters: number[] = [packedObserver.counters().underruns, packedObserver.counters().drainBlocks, packedObserver.counters().depth];
void observedCounters;
packedObserver.close();
void prepareEngineFeed(domContext, BUNDLED_ENGINE_ASSETS.pcmFeedWorklet);
async function defaultBrowserContext() {
  const engine = await createEngine({ document: "opaque" });
  await engine.context.resume();
  await engine.context.suspend();
  engine.host.node.connect(engine.context.destination);
  const nativeContext: AudioContext = engine.context;
  const forwarded = await createEngine({ document: "opaque", createWorker: (url, options) => new Worker(url, options) });
  await forwarded.context.resume();
  const thin = await createEngine({ document: "opaque", createContext: () => ({
    sampleRate: 48000, state: "suspended", close: async () => {},
    audioWorklet: { addModule: async (_url: string) => {} }, marker: "thin" as const,
  }) });
  const marker: "thin" = thin.context.marker;
  // @ts-expect-error Thin injected contexts never acquire DOM capabilities.
  thin.context.resume();
  void [nativeContext, marker];
}
void defaultBrowserContext;
declare const browser: BrowserEngine;
const host = browser.host;
void host.command({ commands: [] });
void host.observe({ subscriptions: [] });
void host.submitSource({
  sourceId: "s", generation: 1n, startFrame: 0n, sampleRateHz: 48_000,
  planes: [new Float32Array()], frames: 0, endOfRegion: true,
});
void host.seekSource({ sourceId: "s", generation: 1n, sourceFrame: 0n });
void host.meters({ enabled: false, onFrame: null });
void host.telemetry({ enabled: false, onFrame: null });
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

const feedModuleUrls = [];
const packedContext = { audioWorklet: { addModule: async (url) => feedModuleUrls.push(String(url)) } };
await imported["./browser"].prepareEngineFeed(packedContext);
await imported["./browser"].prepareEngineFeed(packedContext, "https://example.test/explicit-feed.js");
assert.equal(feedModuleUrls[0], String(imported["./assets"].BUNDLED_ENGINE_ASSETS.pcmFeedWorklet));
assert.equal(feedModuleUrls[1], "https://example.test/explicit-feed.js");
let packedAttach;
let packedDisconnects = 0;
const packedNode = {
  port: { postMessage(message) { if (message.op === "attach") packedAttach = message; } },
  disconnect() { packedDisconnects += 1; },
};
const packedFeedRuntime = imported["./browser"].attachEngineFeed({
  context: packedContext,
  sources: [{ sourceId: "packed-mono", channels: 1 }, { sourceId: "packed-stereo", channels: 2 }],
  quantumFrames: 4,
  createNode: () => packedNode,
});
assert.deepEqual(packedAttach.rings, packedFeedRuntime.rings);
assert.deepEqual(packedFeedRuntime.rings.map((ring) => new Int32Array(ring)[2]), [64, 64]);
for (const ring of packedFeedRuntime.rings) Atomics.store(new Int32Array(ring), 13, 1);
await packedFeedRuntime.ready();
const publicWriter = new imported["./browser"].Msb1RingWriter(packedFeedRuntime.rings[0]);
publicWriter.engage(1n);
publicWriter.reserve(3)[0].set([1, 2, 3]);
publicWriter.commit({ generation: 1n, startFrame: 11n, frames: 3, endOfRegion: true });
const observationBefore = Buffer.from(new Uint8Array(packedFeedRuntime.rings[0]));
const publicObserver = new imported["./browser"].Msb1RingObserver(packedFeedRuntime.rings[0]);
assert.equal(publicObserver.pull((chunk) => {
  assert.equal(chunk.generation, 1n);
  assert.equal(chunk.startFrame, 11n);
  assert.equal(chunk.frames, 3);
  assert.equal(chunk.endOfRegion, true);
  assert.deepEqual([...chunk.planes[0]], [1, 2, 3, 0]);
}), 1);
assert.deepEqual([publicObserver.counters().underruns, publicObserver.counters().drainBlocks, publicObserver.counters().depth], [0, 0, 0]);
publicObserver.close();
assert.equal(publicObserver.pull(() => assert.fail("closed observer")), 0);
assert.deepEqual(Buffer.from(new Uint8Array(packedFeedRuntime.rings[0])), observationBefore);
packedFeedRuntime.close();
packedFeedRuntime.close();
assert.equal(packedDisconnects, 1);
assert.ok(files.includes("dist/assets/miso-engine-v1-pcm-feed-worklet.js"));

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

// Opt-in real-browser qualification reuses an existing Vite/Playwright installation; neither is
// a package dependency. Set MISO_ENGINE_SDK_BROWSER_TOOLS to that installation's node_modules.
// This deliberately tests a production bundle: a raw new-URL Worker asset must survive copying,
// not just Vite's special handling of the default literal Worker constructor.
if (process.env.MISO_ENGINE_SDK_BROWSER_TOOLS) {
  const toolsRoot = resolve(process.env.MISO_ENGINE_SDK_BROWSER_TOOLS);
  const [{ build }, { chromium }, { createServer }] = await Promise.all([
    import(pathToFileURL(resolve(toolsRoot, "vite/dist/node/index.js")).href),
    import(pathToFileURL(resolve(toolsRoot, "playwright/index.mjs")).href),
    import("node:http"),
  ]);
  const browserRoot = resolve(consumerRoot, "browser");
  await mkdir(browserRoot);
  await writeFile(resolve(browserRoot, "index.html"), '<!doctype html><button id="default">Default boot</button><button id="forward">Forwarding factory</button><script type="module" src="/main.js"></script>');
  await writeFile(resolve(browserRoot, "main.js"), `
import { createEngine } from '@misofm/engine/browser';
import { BUNDLED_ENGINE_ASSETS } from '@misofm/engine/assets';
const sessionDocument = ${JSON.stringify(builtDocument)};
window.proof = [];
for (const id of ['default', 'forward']) document.querySelector('#' + id).onclick = async () => {
  try {
    const calls = [];
    const engine = await createEngine({ document: sessionDocument, ...(id === 'forward' ? {
      createWorker(url, options) {
        calls.push({ url: String(url), expected: String(BUNDLED_ENGINE_ASSETS.scratchWorkerModule), type: options.type });
        return new Worker(url, options);
      },
    } : {}) });
    engine.host.node.connect(engine.context.destination);
    await engine.context.resume();
    const status = await engine.host.status();
    await engine.context.suspend();
    await engine.close();
    await engine.close();
    window.proof.push({ id, calls, rate: engine.shape.sampleRateHz, quantum: engine.shape.quantumFrames, result: status.result, state: engine.context.state });
  } catch (error) { window.bootError = String(error?.stack ?? JSON.stringify(error)); }
};
`);
  await build({ root: browserRoot, configFile: false, logLevel: "warn" });
  const network = [];
  const faults = [];
  const server = createServer(async (request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    try {
      const bytes = await readFile(resolve(browserRoot, "dist", `.${pathname === "/" ? "/index.html" : pathname}`));
      const mime = pathname.endsWith(".js") ? "text/javascript" : pathname.endsWith(".wasm") ? "application/wasm" : "text/html";
      response.writeHead(200, { "content-type": mime }); response.end(bytes);
    } catch { response.writeHead(404); response.end(); }
  });
  await new Promise((accept, reject) => { server.once("error", reject); server.listen(0, "127.0.0.1", accept); });
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("pageerror", error => faults.push(error.message));
    page.on("requestfailed", request => faults.push(`${request.url()}: ${request.failure()?.errorText}`));
    page.on("response", response => network.push({ url: response.url(), status: response.status() }));
    await page.goto(`http://127.0.0.1:${server.address().port}`);
    await page.waitForLoadState("networkidle");
    for (const [id, count] of [["default", 1], ["forward", 2]]) {
      await page.locator(`#${id}`).click();
      await page.waitForFunction(count => window.proof.length === count || window.bootError, count, { timeout: 20000 });
      assert.equal(await page.evaluate(() => window.bootError), undefined);
    }
    const results = await page.evaluate(() => window.proof);
    for (const result of results) {
      assert.equal(result.rate, 48000); assert.equal(result.quantum, 128);
      assert.equal(result.result, 0); assert.equal(result.state, "closed");
    }
    assert.deepEqual(results[0].calls, []);
    assert.equal(results[1].calls.length, 1);
    assert.equal(results[1].calls[0].url, results[1].calls[0].expected);
    assert.equal(results[1].calls[0].type, "module");
    assert.deepEqual(faults, []);
    assert.equal(network.some(response => response.status >= 400), false);
    console.log(`packed Vite/Chromium browser boot passed: ${JSON.stringify({ results, network })}`);
  } finally {
    await browser?.close();
    await new Promise(accept => server.close(accept));
  }
}
