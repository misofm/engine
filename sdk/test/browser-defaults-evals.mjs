import assert from "node:assert/strict";
import { test } from "node:test";
import { createEngine, scratchBootInWorker, scratchBootWithWorker, BrowserBootError, scratchBootOptions, workletBootOptions, toWebBootOptions } from "../src/browser/index.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { moduleBytes } from "./support.mjs";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";

const shape = { sampleRateHz: 48000, quantumFrames: 128 };
class FakeWorker {
  listeners = new Map(); history = new Map(); terminated = 0; requests = [];
  addEventListener(type, listener) { const set = this.listeners.get(type) ?? new Set(); set.add(listener); this.listeners.set(type, set); const historical = this.history.get(type) ?? []; historical.push(listener); this.history.set(type, historical); }
  removeEventListener(type, listener) { this.listeners.get(type)?.delete(listener); }
  emit(type, data) { for (const listener of [...(this.listeners.get(type) ?? [])]) listener(type === "message" ? { data } : data); }
  emitHistorical(type, data) { for (const listener of this.history.get(type) ?? []) listener(type === "message" ? { data } : data); }
  postMessage(request) { this.requests.push(request); this.onPost?.(request); }
  terminate() { this.terminated++; }
  assertClosed() { assert.equal(this.terminated, 1); assert.equal([...this.listeners.values()].reduce((n,s) => n+s.size,0), 0); }
}
function boot(worker, extra = {}) {
  return scratchBootWithWorker({ document: new Uint8Array([1]), options: {}, moduleUrl: "wasm", createWorker(url, options) {
    assert.equal(url.href, BUNDLED_ENGINE_ASSETS.scratchWorkerModule.href); assert.deepEqual(options, { type: "module" }); return worker;
  }, requestDeadlineMs: 20, ...extra });
}
const result = { type: "scratch-result", requestId: 1, ok: true, shape };
test("scratch succeeds once after ready, correlates and ignores late events", async () => {
  const worker = new FakeWorker(); const pending = boot(worker);
  worker.emit("message", result); assert.equal(worker.requests.length, 0);
  worker.emit("message", { type: "worker-ready" }); worker.emit("message", { type: "worker-ready" });
  assert.equal(worker.requests.length, 1); assert.equal(worker.requests[0].moduleUrl, "wasm");
  worker.emit("message", { ...result, requestId: 2 }); worker.emit("message", result);
  assert.equal(await pending, shape); worker.emitHistorical("message", result); worker.assertClosed();
});
for (const scratchStage of ["handshake", "request"]) {
  for (const fault of ["timeout", "abort", "error", "messageerror", "post", "reject"]) {
    if (scratchStage === "handshake" && ["post", "reject"].includes(fault)) continue;
    test(`scratch closes on ${scratchStage} ${fault}`, async () => {
      const worker = new FakeWorker(); const controller = new AbortController(); const reason = new Error("stop");
      const pending = boot(worker, { signal: controller.signal });
      if (fault === "post") worker.onPost = () => { throw reason; };
      if (scratchStage === "request") worker.emit("message", { type: "worker-ready" });
      if (fault === "abort") controller.abort(reason);
      if (fault === "error") worker.emit("error", { error: reason });
      if (fault === "messageerror") worker.emit("messageerror", {});
      if (fault === "reject") worker.emit("message", { ...result, ok: false, error: { name: "Failure", message: "refused" } });
      await assert.rejects(pending, error => fault === "abort" || fault === "post" ? error === reason : error instanceof Error);
      worker.emitHistorical("message", result); worker.assertClosed();
    });
  }
}
test("abort inside worker factory is observed before ready or request", async () => {
  const worker = new FakeWorker(); const controller = new AbortController(); const reason = new Error("cancel");
  await assert.rejects(boot(worker, { signal: controller.signal, createWorker() { controller.abort(reason); return worker; } }), error => error === reason);
  worker.assertClosed(); assert.equal(worker.requests.length, 0);
});

function context() { return { sampleRate: 48000, renderQuantumSize: 128, state: "suspended", audioWorklet: { async addModule() {} }, closed: 0, async close() { this.closed++; } }; }
function hostUrl(body) { return `data:text/javascript,${encodeURIComponent(body)}`; }
test("default context + host preserve exact context, URLs and mapped policy", async () => {
  const previous = globalThis.AudioContext;
  const candidate = context(); let constructed;
  globalThis.AudioContext = class { constructor(options) { constructed = options; return candidate; } };
  globalThis.__sdkBootRequests = [];
  const url = hostUrl('export async function createMisoAudioWorkletHost(request) { globalThis.__sdkBootRequests.push(request); return { async dispose() { globalThis.__sdkBootRequests.push("dispose"); } }; }');
  try {
    const engine = await createEngine({ document: "raw", scratchBoot: async () => shape, hostModuleUrl: url, simd128ModuleUrl: "chosen-wasm", workletModuleUrl: "chosen-worklet" });
    assert.equal(engine.context, candidate); assert.deepEqual(constructed, { sampleRate: 48000, renderSizeHint: 128 });
    const request = globalThis.__sdkBootRequests[0]; assert.equal(request.context, candidate);
    assert.deepEqual(request.options, toWebBootOptions(workletBootOptions({}, shape)));
    assert.equal(request.simd128ModuleUrl, "chosen-wasm"); assert.equal(request.workletModuleUrl, "chosen-worklet");
    const close = engine.close(); assert.equal(engine.close(), close); await close;
    assert.deepEqual(globalThis.__sdkBootRequests.slice(1), ["dispose"]); assert.equal(candidate.closed, 1);
  } finally { if (previous === undefined) delete globalThis.AudioContext; else globalThis.AudioContext = previous; delete globalThis.__sdkBootRequests; }
});
test("injected context and host independently retain default scratch worker", async () => {
  const worker = new FakeWorker(); const candidate = context(); let request;
  worker.onPost = value => { request = value; worker.emit("message", result); };
  const pending = createEngine({ document: "opaque", createContext: () => candidate, createHost: async () => ({ async dispose() {} }), simd128ModuleUrl: "override", createWorker: () => worker });
  worker.emit("message", { type: "worker-ready" }); const engine = await pending;
  assert.equal(request.moduleUrl, "override"); assert.deepEqual(request.options, scratchBootOptions({})); worker.assertClosed(); await engine.close();
});
test("default host failures close accepted context and preserve factory refusal", async () => {
  const refusalUrl = hostUrl('export async function createMisoAudioWorkletHost() { throw { tag: "miso.error.v1", result: 1 }; }');
  for (const url of ["data:text/javascript,throw new Error('load')", refusalUrl]) {
    const candidate = context();
    await assert.rejects(createEngine({ document: "opaque", scratchBoot: async () => shape, createContext: () => candidate, hostModuleUrl: url }), error => url === refusalUrl ? error.tag === "miso.error.v1" : error instanceof BrowserBootError && error.operation === "host-import");
    assert.equal(candidate.closed, 1);
  }
});

test("default host suspends a running native context before construction", async () => {
  const candidate = context(); candidate.state = "running";
  const calls = [];
  candidate.suspend = async () => { calls.push("suspend"); candidate.state = "suspended"; };
  const url = hostUrl('export async function createMisoAudioWorkletHost(request) { if (request.context.state !== "suspended") throw new Error("running"); return { async dispose() {} }; }');
  const engine = await createEngine({ document: "opaque", scratchBoot: async () => shape, createContext: () => candidate, hostModuleUrl: url });
  assert.deepEqual(calls, ["suspend"]); await engine.close(); assert.equal(candidate.closed, 1);
});
test("suspension failure closes the accepted context and retains the error", async () => {
  const candidate = context(); candidate.state = "running";
  const error = new Error("suspend failed");
  candidate.suspend = async () => { throw error; };
  await assert.rejects(createEngine({ document: "opaque", scratchBoot: async () => shape, createContext: () => candidate, hostModuleUrl: "not-an-importable-url" }), failure => failure === error);
  assert.equal(candidate.closed, 1);
});


test("scratch settlement clears timers and abort listeners; removed callbacks remain inert", async (t) => {
  const timers = new Map(); const historicalTimers = [];
  t.mock.method(globalThis, "setTimeout", (callback) => { const handle = {}; timers.set(handle, callback); historicalTimers.push(callback); return handle; });
  t.mock.method(globalThis, "clearTimeout", handle => timers.delete(handle));
  for (const outcome of ["success", "abort", "error"]) {
    const worker = new FakeWorker(); const controller = new AbortController();
    const abortListeners = new Set(); const historicalAbort = [];
    const add = controller.signal.addEventListener.bind(controller.signal);
    const remove = controller.signal.removeEventListener.bind(controller.signal);
    t.mock.method(controller.signal, "addEventListener", (type, listener, options) => {
      if (type === "abort") { abortListeners.add(listener); historicalAbort.push(listener); } add(type, listener, options);
    });
    t.mock.method(controller.signal, "removeEventListener", (type, listener) => {
      if (type === "abort") abortListeners.delete(listener); remove(type, listener);
    });
    let settlements = 0;
    const pending = boot(worker, { signal: controller.signal }).then(() => { settlements++; }, () => { settlements++; });
    assert.equal(timers.size, 1); assert.equal(abortListeners.size, 1);
    worker.emit("message", { type: "worker-ready" }); assert.equal(timers.size, 1);
    if (outcome === "success") worker.emit("message", result);
    else if (outcome === "abort") controller.abort(new Error("stop"));
    else worker.emit("error", { error: new Error("failed") });
    await pending;
    assert.equal(timers.size, 0, "no scratch deadline survives settlement");
    assert.equal(abortListeners.size, 0, "no abort listener survives settlement");
    worker.emitHistorical("message", { type: "worker-ready" });
    worker.emitHistorical("message", result);
    worker.emitHistorical("error", { error: new Error("late") });
    worker.emitHistorical("messageerror", {});
    for (const callback of historicalAbort) callback();
    for (const callback of historicalTimers) callback();
    await Promise.resolve();
    assert.equal(settlements, 1); assert.equal(worker.requests.length, 1);
    assert.equal(timers.size, 0); assert.equal(abortListeners.size, 0); worker.assertClosed();
  }
});

test("actual scratch entry and client retain real Wasm refusal and usage error types", async () => {
  const bytes = await moduleBytes();
  const document = new TextEncoder().encode("{}");
  const options = scratchBootOptions({});
  let direct;
  await assert.rejects(scratchBootInWorker({ moduleBytes: bytes, document, options }), error => {
    assert.ok(error instanceof MisoEngineError); direct = error; return true;
  });
  const oldSelf = globalThis.self; const oldFetch = globalThis.fetch;
  let worker;
  const scope = { onmessage: null, postMessage(reply) { worker?.emit("message", structuredClone(reply)); } };
  globalThis.self = scope;
  globalThis.fetch = async () => new Response(bytes);
  try {
    await import("../src/browser/scratch-worker.ts");
    async function refused() {
      worker = new FakeWorker();
      worker.onPost = request => scope.onmessage({ data: structuredClone(request) });
      const pending = scratchBootWithWorker({ document, options, moduleUrl: "wasm", createWorker: () => worker });
      worker.emit("message", { type: "worker-ready" });
      let refusal;
      await assert.rejects(pending, error => { refusal = error; return true; });
      worker.assertClosed(); return refusal;
    }
    const transported = await refused();
    assert.ok(transported instanceof MisoEngineError);
    for (const key of ["name", "message", "phase", "code", "result", "diagnostics", "diagnosticCode", "diagnosticPath"]) {
      assert.deepEqual(transported[key], direct[key], key);
    }
    assert.ok(Object.isFrozen(transported.diagnostics));
    const usage = new MisoUsageError("Worker-side SDK usage refusal");
    globalThis.fetch = async () => { throw usage; };
    const transportedUsage = await refused();
    assert.ok(transportedUsage instanceof MisoUsageError);
    assert.equal(transportedUsage.name, usage.name); assert.equal(transportedUsage.message, usage.message);
  } finally {
    if (oldSelf === undefined) delete globalThis.self; else globalThis.self = oldSelf;
    globalThis.fetch = oldFetch;
  }
});

import { prepareBrowserSessionWithWorker, prepareBrowserSessionInWorker } from "../src/browser/index.ts";
import { sessionDocument, effectEntry, ramp } from "./support.mjs";
import { WasmBoundary } from "../src/core/boundary.ts";
import { CATALOG } from "../src/generated/catalog.ts";

test("prepared worker snapshots inputs and retains the exact module after termination", async () => {
  const worker = new FakeWorker(); const document = new Uint8Array([4]); const options = { console: { meterBlocks: 2 } };
  const module = await WebAssembly.compile(new Uint8Array([0,97,115,109,1,0,0,0]));
  const pending = prepareBrowserSessionWithWorker({ document, options, moduleUrl: "wasm", createWorker: () => worker });
  document[0] = 9; options.console.meterBlocks = 99;
  worker.emit("message", { type: "worker-ready" });
  assert.equal(worker.requests[0].type, "prepare"); assert.equal(worker.requests[0].document[0], 4);
  assert.equal(worker.requests[0].options.console.meterBlocks, 2);
  worker.emit("message", { ...result, module });
  assert.equal((await pending).module, module); worker.assertClosed();
});

for (const fault of ["post", "missing-module", "abort-before-reply", "reply-before-abort", "messageerror"]) {
  test(`prepared worker lifecycle ${fault}`, async () => {
    const worker = new FakeWorker(); const controller = new AbortController();
    const module = await WebAssembly.compile(new Uint8Array([0,97,115,109,1,0,0,0]));
    const pending = prepareBrowserSessionWithWorker({ document: new Uint8Array(), options: {}, moduleUrl: "wasm", createWorker: () => worker, signal: controller.signal });
    if (fault === "post") worker.onPost = () => { throw new DOMException("clone", "DataCloneError"); };
    worker.emit("message", { type: "worker-ready" });
    if (fault === "abort-before-reply") controller.abort();
    if (fault === "messageerror") worker.emit("messageerror", {});
    worker.emit("message", fault === "missing-module" ? result : { ...result, module });
    if (fault === "reply-before-abort") { controller.abort(); assert.equal((await pending).module, module); }
    else await assert.rejects(pending);
    worker.emitHistorical("message", { ...result, module }); worker.assertClosed();
  });
}

test("createEngine snapshots document and nested policy before scratch awaits and forwards prepared identity", async () => {
  const document = new Uint8Array([3]); const policy = { console: { meterBlocks: 2 } };
  const module = await WebAssembly.compile(new Uint8Array([0,97,115,109,1,0,0,0]));
  const engine = await createEngine({ document, policy, preparedModule: module, createContext: context,
    scratchBoot: async request => { document[0] = 8; policy.console.meterBlocks = 9; assert.equal(request.document[0], 3); return shape; },
    createHost: async request => { assert.equal(request.document[0], 3); assert.equal(request.options.console.meterBlocks, 2); assert.equal(request.preparedModule, module); return { async dispose() {} }; },
  });
  await engine.close();
});

test("preparation compiles once, disposes scratch, and yields fresh stateful live DSP and meter origins", async () => {
  const effects = ["miso.parametric-eq", "miso.compressor", "miso.delay"].map((id, index) => {
    const row = CATALOG.effects.find(effect => effect.id === id);
    const values = id === "miso.parametric-eq" ? { 1: 1, 3: 1000, 4: 6 }
      : id === "miso.delay" ? { 1: 5, 2: 0.5, 4: 0.5 } : {};
    return effectEntry(`fx${index}`, id, row.parameters.map(parameter => ({ id: parameter.id, unit: parameter.unitName, value: values[parameter.id] ?? parameter.default })));
  });
  const bytes = await moduleBytes();
  const document = new TextEncoder().encode(sessionDocument({ effects: { simd1: effects }, frames: 16384 }));
  const options = { console: { commandQueueRecords: 64, meterBlocks: 2, observationTaps: 1 } };
  let compiles = 0, disposals = 0; const compile = WebAssembly.compile; const dispose = WasmBoundary.prototype.dispose;
  WebAssembly.compile = async (...args) => { compiles++; return compile(...args); };
  WasmBoundary.prototype.dispose = function () { disposals++; return dispose.call(this); };
  let prepared;
  try { prepared = await prepareBrowserSessionInWorker({ moduleBytes: bytes, document, options }); }
  finally { WebAssembly.compile = compile; WasmBoundary.prototype.dispose = dispose; }
  assert.equal(compiles, 1); assert.equal(disposals, 1);
  const referenceModule = await compile(bytes);
  const boot = module => WasmBoundary.boot({ instantiate: () => WebAssembly.instantiate(module, {}) }, document, options);
  const live = await boot(prepared.module), reference = await boot(referenceModule);
  try {
    assert.equal(live.renderedQuanta(), 0n); assert.deepEqual(live.shape(), prepared.shape);
    live.meterLease(true); reference.meterLease(true);
    let meterWindows = 0, delayedEnergy = 0;
    for (let block = 0; block < 8; block++) {
      // Excite the effects once, then observe their history while the source is silent.
      const pcm = block === 0 ? ramp(128, 1) : new Float32Array(128);
      for (const boundary of [live, reference]) assert.equal(boundary.submitSource({ sourceId: "s", generation: 1n, startFrame: BigInt(block * 128), planes: [pcm, pcm], endOfRegion: false }).ok, true);
      const output = live.render(128);
      assert.deepEqual(output, reference.render(128));
      if (block === 0) assert.notDeepEqual(output.left, pcm, "active processing changes unaffected input");
      if (block >= 2) for (const value of output.left) delayedEnergy += value * value;
      const meter = live.pollMeters();
      assert.deepEqual(meter, reference.pollMeters());
      if (meter !== undefined) {
        assert.equal(meter.firstSample, BigInt(meterWindows * 256));
        assert.equal(meter.endSample, BigInt((meterWindows + 1) * 256));
        meterWindows++;
      }
    }
    assert.equal(meterWindows, 4, "live meter windows originate at sample zero");
    assert.ok(delayedEnergy > 0.001, "short wet feedback delay produces a nontrivial tail within eight quanta");
  } finally { live.dispose(); reference.dispose(); }
});

for (const frames of [0, 1, 129]) for (const channels of [1, 2]) {
  test(`preparation handles ${frames} frames and ${channels} channels without console`, async () => {
    const pending = prepareBrowserSessionInWorker({ moduleBytes: await moduleBytes(), document: new TextEncoder().encode(sessionDocument({ frames, channels })), options: {} });
    if (frames === 0) { await assert.rejects(pending, error => error instanceof MisoEngineError && error.diagnosticCode === "capacity.zero"); return; }
    const prepared = await pending;
    assert.equal(prepared.shape.sources[0].frames, BigInt(frames));
  });
}

test("current shipped host sends prepared module to worklet without fetch or compile", async () => {
  const { createMisoAudioWorkletHost } = await import("../../hosts/host-web/web/miso-engine-v1-audio-worklet-host.js");
  const module = await WebAssembly.compile(new Uint8Array([0,97,115,109,1,0,0,0]));
  const previous = { fetch: globalThis.fetch, compile: WebAssembly.compile, node: globalThis.AudioWorkletNode };
  let sent = false, disconnected = false;
  globalThis.fetch = async () => { throw new Error("unexpected fetch"); };
  WebAssembly.compile = async () => { throw new Error("unexpected compile"); };
  globalThis.AudioWorkletNode = class {
    constructor(_context, _name, options) {
      assert.equal(options.processorOptions.module, module); sent = true;
      this.port = { close() {}, onmessage: null };
      queueMicrotask(() => this.port.onmessage({ data: { tag: "miso.error.v1", requestId: 0, result: 1 } }));
    }
    disconnect() { disconnected = true; }
  };
  const options = { context: context(), document: new Uint8Array([1]), options: toWebBootOptions({}), simd128ModuleUrl: "must-not-fetch", workletModuleUrl: "worklet", preparedModule: module };
  try {
    await assert.rejects(createMisoAudioWorkletHost(options), error => error.tag === "miso.error.v1" && error.result === 1);
    assert.equal(sent, true); assert.equal(disconnected, true);
    sent = false;
    await assert.rejects(createMisoAudioWorkletHost({ ...options, unexpected: true })); assert.equal(sent, false);
    await assert.rejects(createMisoAudioWorkletHost({ ...options, preparedModule: {} })); assert.equal(sent, false);
  } finally { globalThis.fetch = previous.fetch; WebAssembly.compile = previous.compile; globalThis.AudioWorkletNode = previous.node; }
});

for (const cloneFault of [false, true]) test(`actual prepared worker structured clone, clone fault=${cloneFault}`, async () => {
  const bytes = await moduleBytes(); const previous = { self: globalThis.self, fetch: globalThis.fetch };
  const worker = new FakeWorker(); let sentModule;
  const scope = { onmessage: null, postMessage(reply) {
    if (reply.module) { sentModule = reply.module; if (cloneFault) throw new DOMException("module clone failed", "DataCloneError"); }
    worker.emit("message", structuredClone(reply));
  } };
  globalThis.self = scope; globalThis.fetch = async () => new Response(bytes);
  try {
    await import(`../src/browser/scratch-worker.ts?prepare=${cloneFault}`);
    worker.onPost = request => scope.onmessage({ data: structuredClone(request) });
    const pending = prepareBrowserSessionWithWorker({ document: new TextEncoder().encode(sessionDocument()), options: {}, moduleUrl: "wasm", createWorker: () => worker });
    worker.emit("message", { type: "worker-ready" });
    if (cloneFault) await assert.rejects(pending, error => error.name === "DataCloneError");
    else { const prepared = await pending; assert.ok(prepared.module instanceof WebAssembly.Module); assert.notEqual(prepared.module, sentModule); await WebAssembly.instantiate(prepared.module, {}); }
    worker.assertClosed();
  } finally { globalThis.self = previous.self; globalThis.fetch = previous.fetch; }
});


test("preparation admits several independent mono and stereo sources", async () => {
  const document = JSON.parse(sessionDocument({ frames: 257 }));
  const source = document.sources[0], track = document.tracks[0], route = document.routes[0];
  document.sources = []; document.tracks = []; document.routes = [];
  for (const [index, channels] of [1, 2, 1, 2].entries()) {
    const sourceId = `s${index}`, trackId = `t${index}`;
    document.sources.push({ ...source, id: sourceId, channels, frames: String(257 + index) });
    document.tracks.push({ ...track, id: trackId, source_id: sourceId, right_source_channel: channels - 1 });
    document.routes.push({ ...route, id: `route${index}`, source: { ...route.source, track_id: trackId } });
  }
  const prepared = await prepareBrowserSessionInWorker({ moduleBytes: await moduleBytes(), document: new TextEncoder().encode(JSON.stringify(document)), options: { sourceRingFrames: 256 } });
  assert.deepEqual(prepared.shape.sources.map(source => [source.id, source.channels, source.frames]), [
    ["s0", 1, 257n], ["s1", 2, 258n], ["s2", 1, 259n], ["s3", 2, 260n],
  ]);
});
