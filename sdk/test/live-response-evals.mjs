import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { BrowserTrackResponse } from "../src/browser/response.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

class FakeWorker {
  listeners = new Map();
  requests = [];
  pending = new Map();
  terminated = 0;

  addEventListener(type, listener) {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(listener);
    this.listeners.set(type, listeners);
  }

  removeEventListener(type, listener) {
    this.listeners.get(type)?.delete(listener);
  }

  postMessage(message, transfer = []) {
    this.requests.push({ message, transfer });
    if (message.type === "track-response-init") {
      queueMicrotask(() => this.emit("message", { type: "track-response-ready" }));
    } else if (message.type === "track-response-query") {
      this.pending.set(message.requestId, message);
    }
  }

  reply(requestId) {
    assert.ok(this.pending.has(requestId));
    this.pending.delete(requestId);
    this.emit("message", {
      type: "track-response-result",
      requestId,
      result: {
        trackId: "t",
        mode: "target",
        meaning: "eqFilterSubtotal",
        sampleRateHz: 48_000,
        floorDb: -120,
        frequenciesHz: new Float32Array([20, 20_000]),
        leftDb: new Float32Array([-1, -2]),
        rightDb: new Float32Array([-3, -4]),
        members: [],
        capturedSample: 0n,
        snapshotToken: 1n,
        excludedMemberCount: 0,
        resultBytes: 200n,
      },
    });
  }

  emit(type, data) {
    for (const listener of this.listeners.get(type) ?? []) {
      listener(type === "message" ? { data } : data);
    }
  }

  terminate() {
    this.terminated += 1;
  }
}

const browserQuery = {
  trackId: "t",
  grid: { kind: "logarithmic", points: 2, minimumHz: 20, maximumHz: 20_000 },
  channels: "both",
};

test("browser live response copies the snapshot, owns arrays, serializes one query, and closes pending work", async () => {
  const worker = new FakeWorker();
  const client = await BrowserTrackResponse.create({
    module: {},
    responseLimits: { requestDeadlineMs: 100 },
    createWorker: () => worker,
  });
  const source = new Uint8Array([1, 2, 3]);
  const first = client.query(browserQuery, { result: 0, snapshot: source });
  await assert.rejects(client.query(browserQuery, { result: 0, snapshot: source }), /already in flight/);
  const request = worker.requests.find(({ message }) => message.type === "track-response-query");
  assert.ok(request);
  assert.notEqual(request.message.snapshot, source);
  assert.deepEqual([...request.message.snapshot], [1, 2, 3]);
  assert.deepEqual(request.transfer, [request.message.snapshot.buffer]);
  worker.reply(request.message.requestId);
  const result = await first;
  assert.equal(result.snapshotToken, 1n);
  assert.deepEqual([...result.leftDb], [-1, -2]);
  assert.notEqual(result.frequenciesHz, result.leftDb);

  const pending = client.query(browserQuery, { result: 0, snapshot: source });
  await client.close();
  await assert.rejects(pending, /closed/);
  assert.equal(worker.terminated, 1);
  await assert.rejects(client.query(browserQuery, { result: 0, snapshot: source }), /closed/);
  await client.close();
  assert.equal(worker.terminated, 1);
});

test("browser engine close invalidates a capture that resolves before its Worker starts", async () => {
  const worker = new FakeWorker();
  const events = [];
  const context = {
    sampleRate: 48_000,
    renderQuantumSize: 128,
    state: "suspended",
    audioWorklet: { async addModule() {} },
    async close() { events.push("context"); },
  };
  const engine = await createEngine({
    document: "opaque",
    scratchBoot: async () => ({ sampleRateHz: 48_000, quantumFrames: 128 }),
    createContext: () => context,
    createHost: async () => ({
      async captureTrackResponse() {
        return { result: 0, snapshot: new Uint8Array([1, 2, 3]) };
      },
      async dispose() { events.push("host"); },
    }),
    createResponseWorker: () => worker,
  });
  const pending = engine.queryTrackResponse(browserQuery);
  await engine.close();
  await assert.rejects(pending, /closed/);
  assert.deepEqual(events, ["host", "context"]);
  assert.equal(worker.requests.length, 0);
});

test("browser host failure or disposal while the Worker is pending invalidates the result", async () => {
  for (const lifecycle of ["failed", "disposed"]) {
    const worker = new FakeWorker();
    let hostLifecycle = "ready";
    const context = {
      sampleRate: 48_000,
      renderQuantumSize: 128,
      state: "suspended",
      audioWorklet: { async addModule() {} },
      async close() {},
    };
    const host = {
      async captureTrackResponse() {
        return { result: 0, snapshot: new Uint8Array([1, 2, 3]) };
      },
      async status() {
        if (hostLifecycle === "disposed") {
          throw new MisoEngineError("host disposed", { phase: "lifecycle", code: "wrongState", result: 3 });
        }
        return hostLifecycle === "failed"
          ? { result: 0, state: 3, lastResult: 0 }
          : { result: 0, state: 2, lastResult: 0 };
      },
      async dispose() { hostLifecycle = "disposed"; },
    };
    const engine = await createEngine({
      document: "opaque",
      scratchBoot: async () => ({ sampleRateHz: 48_000, quantumFrames: 128 }),
      createContext: () => context,
      createHost: async () => host,
      createResponseWorker: () => worker,
    });
    try {
      const pending = engine.queryTrackResponse(browserQuery);
      for (let attempt = 0; attempt < 10 && worker.pending.size === 0; attempt += 1) await Promise.resolve();
      assert.equal(worker.pending.size, 1);
      hostLifecycle = lifecycle;
      if (lifecycle === "disposed") await host.dispose();
      const request = [...worker.pending.values()][0];
      worker.reply(request.requestId);
      await assert.rejects(pending, error => error instanceof MisoEngineError && error.phase === "lifecycle");
    } finally {
      await engine.close();
    }
  }
});

test("candidate Wasm captures the live boundary, actual owners, edit drain, bounds, and lifecycle", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const eq = CATALOG.effects.find((row) => row.id === "miso.parametric-eq");
  assert.ok(eq);
  const params = ["band-1-enabled", "band-1-kind", "band-1-frequency", "band-1-gain", "band-1-q"]
    .map((name) => {
      const row = eq.parameters.find((candidate) => candidate.name === name);
      assert.ok(row);
      return {
        id: row.id,
        unit: row.unitName,
        value: name === "band-1-enabled" ? 1 : name === "band-1-gain" ? 3 : row.default,
        channel: "both",
      };
    });
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const engine = await createOfflineEngine(sessionDocument({
    effects: {
      simd1: [effectEntry("eq", "miso.parametric-eq", params)],
      dynamic: [effectEntry("comp", "miso.compressor")],
      simd2: [],
    },
  }), { asset, console: { commandQueueRecords: 64 } });
  const request = {
    trackId: "t",
    grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
  };
  try {
    const first = engine.queryTrackResponse(request);
    assert.equal(first.mode, "target");
    assert.equal(first.meaning, "eqFilterSubtotal");
    assert.equal(first.sampleRateHz, 48_000);
    assert.equal(first.capturedSample, 0n);
    assert.equal(first.snapshotToken, 1n);
    assert.equal(first.frequenciesHz.length, 16);
    assert.equal(first.leftDb.length, 16);
    assert.equal(first.rightDb.length, 16);
    assert.deepEqual(first.members.map((member) => [member.rack, member.stableId, member.available]), [
      ["input", "input-filters", true],
      ["simd1", "eq", true],
      ["dynamic", "comp", false],
    ]);
    assert.deepEqual(first.members[1].enabledLeft, [true, false, false, false]);
    assert.equal(first.members[2].excludedReason, "linearResponseUnavailable");
    const firstFrequencies = first.frequenciesHz.slice();
    const firstLeft = first.leftDb.slice();

    const console = engine.console();
    await console.submit(
      console.edit.track("t").effect("simd1", 0, "miso.parametric-eq").parameter("band-1-gain", -3),
    );
    const beforeDrain = engine.queryTrackResponse(request);
    assert.equal(beforeDrain.capturedSample, 0n);
    assert.deepEqual([...beforeDrain.leftDb], [...firstLeft]);

    const shape = engine.shape();
    for (const source of shape.sources) {
      engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: 0n,
        planes: Array.from({ length: source.channels }, (_unused, channel) => ramp(shape.quantumFrames, 13 + channel)),
        endOfRegion: false,
      });
    }
    engine.render();
    const afterDrain = engine.queryTrackResponse(request);
    assert.equal(afterDrain.capturedSample, 128n);
    assert.equal(afterDrain.snapshotToken, 3n);
    assert.notEqual(afterDrain.frequenciesHz, first.frequenciesHz);
    assert.notEqual(afterDrain.leftDb, first.leftDb);
    assert.deepEqual([...first.frequenciesHz], [...firstFrequencies]);
    assert.deepEqual([...first.leftDb], [...firstLeft]);
    assert.notDeepEqual([...afterDrain.leftDb], [...firstLeft]);

    assert.throws(
      () => engine.queryTrackResponse({ ...request, responseLimits: { maximumResultBytes: 104 } }),
      (error) => error instanceof MisoEngineError && error.code === "refusedBudget",
    );
    engine.dispose();
    assert.throws(() => engine.queryTrackResponse(request), MisoUsageError);
  } finally {
    if (engine.state() !== "disposed") engine.dispose();
  }
});
