import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";

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
    } else if (message.type === "spectrum-stream-query") {
      const bytes = encodedSpectrumResult(message.buffer);
      queueMicrotask(() => this.#emit({
        type: "spectrum-stream-result",
        requestId: message.requestId,
        resultByteLength: bytes,
        buffer: message.buffer,
        metadata: message.metadata,
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

function streamMetadata(status, sequence = 0n) {
  return {
    structSize: ABI_LAYOUT.constants.spectrumStreamMetadataBytes,
    abiVersion: ABI_LAYOUT.abiVersion,
    result: 0,
    status,
    target: 3,
    channels: 3,
    sampleRateHz: 48_000,
    quantumFrames: 128,
    hopFrames: 2_048,
    sourceUnderrun: 0,
    captureEpoch: 1n,
    sequence,
    droppedCaptures: 0n,
    windows: sequence,
    capturedSample: sequence * 2_048n,
    endSample: (sequence + 1n) * 2_048n,
    analysisEpoch: 1n,
    historyStartSample: 0n,
    smoothingMs: 100,
  };
}

function encodedSpectrumResult(buffer) {
  const count = ABI_LAYOUT.constants.spectrumBinCount;
  const header = ABI_LAYOUT.constants.spectrumResultHeaderBytes;
  const frequenciesOffset = header;
  const leftOffset = frequenciesOffset + count * 4;
  const rightOffset = leftOffset + count * 4;
  const resultBytes = rightOffset + count * 4;
  const view = new DataView(buffer, 0, resultBytes);
  const field = (name) => ABI_LAYOUT.structures.spectrumResult.fields.find((candidate) => candidate.name === name).offset;
  view.setUint32(field("structSize"), header, true);
  view.setUint32(field("abiVersion"), ABI_LAYOUT.abiVersion, true);
  view.setUint32(field("result"), 0, true);
  view.setUint32(field("target"), 3, true);
  view.setUint32(field("channels"), 3, true);
  view.setUint32(field("sampleRateHz"), 48_000, true);
  view.setUint32(field("windowFrames"), 2_048, true);
  view.setUint32(field("binCount"), count, true);
  view.setUint32(field("sourceUnderrun"), 0, true);
  view.setFloat32(field("floorDb"), -120, true);
  view.setBigUint64(field("capturedSample"), 0n, true);
  view.setBigUint64(field("endSample"), 2_048n, true);
  view.setBigUint64(field("snapshotToken"), 1n, true);
  view.setBigUint64(field("resultBytes"), BigInt(resultBytes), true);
  view.setUint32(field("frequenciesOffset"), frequenciesOffset, true);
  view.setUint32(field("leftOffset"), leftOffset, true);
  view.setUint32(field("rightOffset"), rightOffset, true);
  view.setUint32(field("reserved0"), 0, true);
  return resultBytes;
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

test("browser managed spectrum returns and reuses one transfer buffer", async () => {
  const worker = new SpectrumWorker();
  const buffers = [];
  let sequence = 0n;
  const host = hostWithCapture({
    async startSpectrumStream() {
      return { result: 0, metadata: streamMetadata(1) };
    },
    async readSpectrumStream(buffer) {
      buffers.push(buffer);
      sequence += 1n;
      return {
        result: 0,
        byteLength: 8_256,
        buffer,
        metadata: streamMetadata(6, sequence),
      };
    },
    async stopSpectrumStream() {
      return { result: 0, metadata: streamMetadata(5, sequence) };
    },
  });
  const engine = await browserEngine(host, worker);
  try {
    const subscription = await engine.subscribeSpectrum({
      target: { kind: "output", outputId: "out" },
      channels: "both",
      cadenceMs: 1,
    });
    const first = await subscription.pump();
    assert.equal(first.available, true);
    assert.equal(subscription.readLatest().leftDb.length, ABI_LAYOUT.constants.spectrumBinCount);
    const streamMessages = worker.messages.filter((message) => message.type === "spectrum-stream-query");
    assert.equal(streamMessages.length, 1);
    assert.equal("result" in streamMessages[0], false);
    assert.equal(streamMessages[0].buffer instanceof ArrayBuffer, true);
    const owned = subscription.readLatest().leftDb;
    owned[0] = 9;
    assert.equal(subscription.readLatest().leftDb[0], 0);

    const second = await subscription.pump();
    assert.equal(second.available, true);
    assert.equal(buffers.length, 2);
    assert.equal(buffers[0], buffers[1]);
    assert.equal(worker.messages.filter((message) => message.type === "spectrum-stream-query").length, 2);
    await subscription.close();
  } finally {
    await engine.close();
  }
});

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
