import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";

const SHAPE = Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128, tracks: ["t"] });
const PREPARED = Object.freeze({
  target: Object.freeze({ kind: "output", outputId: "out" }),
  channels: "both",
});

function context() {
  return {
    sampleRate: 48_000,
    renderQuantumSize: 128,
    state: "suspended",
    audioWorklet: { async addModule() {} },
    async close() {},
  };
}

function spectrumResult(query) {
  return {
    target: query.target,
    channels: query.channels ?? "both",
    sampleRateHz: 48_000,
    windowFrames: 2_048,
    binCount: 1,
    floorDb: -120,
    frequenciesHz: new Float32Array([0]),
    leftDb: new Float32Array([0]),
    rightDb: new Float32Array([0]),
    capturedSample: 0n,
    endSample: 2_048n,
    snapshotToken: 1n,
    graphSourceUnderrun: false,
    resultBytes: 1n,
  };
}

class SpectrumWorker {
  #listeners = new Map();
  messages = [];
  silent;
  terminated = false;

  constructor({ silent = false } = {}) {
    this.silent = silent;
  }

  addEventListener(type, listener) {
    const listeners = this.#listeners.get(type) ?? new Set();
    listeners.add(listener);
    this.#listeners.set(type, listeners);
  }

  removeEventListener(type, listener) {
    this.#listeners.get(type)?.delete(listener);
  }

  postMessage(message) {
    this.messages.push(message);
    if (this.silent) return;
    if (message.type === "spectrum-init") {
      queueMicrotask(() => this.#emit({ type: "spectrum-ready" }));
    } else if (message.type === "spectrum-query") {
      queueMicrotask(() => this.#emit({
        type: "spectrum-result",
        requestId: message.requestId,
        result: spectrumResult(message.query),
      }));
    }
  }

  terminate() {
    this.terminated = true;
  }

  #emit(data) {
    for (const listener of this.#listeners.get("message") ?? []) listener({ data });
  }
}

function hostWithCapture(overrides = {}) {
  return {
    async armSpectrum() { return { result: 0 }; },
    async readSpectrum() { return { result: 0, snapshot: new Uint8Array([1]) }; },
    async status() { return { result: 0, state: 2, lastResult: 0 }; },
    async cancelSpectrum() { return { result: 0 }; },
    async dispose() {},
    ...overrides,
  };
}

async function browserEngine(host, worker) {
  return createEngine({
    document: "opaque",
    spectrum: PREPARED,
    scratchBoot: async () => SHAPE,
    createContext: context,
    createHost: async () => host,
    createResponseWorker: () => worker,
  });
}

test("browser spectrum admission copies mutable query metadata before Worker dispatch", async () => {
  const worker = new SpectrumWorker();
  const engine = await browserEngine(hostWithCapture(), worker);
  try {
    const request = { target: { kind: "output", outputId: "out" }, channels: "both" };
    const pending = engine.querySpectrum(request);
    request.target.outputId = "mutated";
    const result = await pending;
    assert.equal(result.target.outputId, "out");
    const message = worker.messages.find((candidate) => candidate.type === "spectrum-query");
    assert.equal(message.query.target.outputId, "out");
  } finally {
    await engine.close();
  }
});

test("browser spectrum deadline cancels a late arm and blocks inherited capture", async () => {
  let armed = false;
  let cancels = 0;
  const host = hostWithCapture({
    async armSpectrum() {
      armed = true;
      await new Promise((resolve) => setTimeout(resolve, 30));
      return { result: 0 };
    },
    async readSpectrum() {
      throw new Error("read must wait for the late arm");
    },
    async cancelSpectrum() {
      armed = false;
      cancels += 1;
      return { result: 0 };
    },
  });
  const engine = await browserEngine(host, new SpectrumWorker());
  try {
    const request = { target: { kind: "output", outputId: "out" }, channels: "both", spectrumLimits: { requestDeadlineMs: 5 } };
    await assert.rejects(engine.querySpectrum(request), /deadline/);
    await assert.rejects(engine.querySpectrum(request), /already in flight/);
    await new Promise((resolve) => setTimeout(resolve, 45));
    assert.equal(armed, false);
    assert.equal(cancels, 1);
    await assert.rejects(engine.querySpectrum(request), /spectrum Worker initialization|deadline|already in flight/);
  } finally {
    await engine.close();
  }
});

test("browser spectrum Worker initialization uses the total query deadline", async () => {
  const engine = await browserEngine(hostWithCapture(), new SpectrumWorker({ silent: true }));
  try {
    const started = Date.now();
    await assert.rejects(
      engine.querySpectrum({
        target: { kind: "output", outputId: "out" },
        channels: "both",
        spectrumLimits: { requestDeadlineMs: 10 },
      }),
      /deadline|initialization/,
    );
    assert.ok(Date.now() - started < 1_000);
  } finally {
    await engine.close();
  }
});
