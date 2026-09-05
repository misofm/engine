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
for (const phase of ["handshake", "request"]) {
  for (const fault of ["timeout", "abort", "error", "messageerror", "post", "reject"]) {
    if (phase === "handshake" && ["post", "reject"].includes(fault)) continue;
    test(`scratch closes on ${phase} ${fault}`, async () => {
      const worker = new FakeWorker(); const controller = new AbortController(); const reason = new Error("stop");
      const pending = boot(worker, { signal: controller.signal });
      if (fault === "post") worker.onPost = () => { throw reason; };
      if (phase === "request") worker.emit("message", { type: "worker-ready" });
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
