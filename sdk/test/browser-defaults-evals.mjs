import assert from "node:assert/strict";
import { test } from "node:test";
import { createEngine, scratchBootWithWorker, BrowserBootError, scratchBootOptions, workletBootOptions, toWebBootOptions } from "../src/browser/index.ts";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";

const shape = { sampleRateHz: 48000, quantumFrames: 128 };
class FakeWorker {
  listeners = new Map(); terminated = 0; requests = [];
  addEventListener(type, listener) { const set = this.listeners.get(type) ?? new Set(); set.add(listener); this.listeners.set(type, set); }
  removeEventListener(type, listener) { this.listeners.get(type)?.delete(listener); }
  emit(type, data) { for (const listener of [...(this.listeners.get(type) ?? [])]) listener(type === "message" ? { data } : data); }
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
  assert.equal(await pending, shape); worker.emit("message", result); worker.assertClosed();
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
      worker.emit("message", result); worker.assertClosed();
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
