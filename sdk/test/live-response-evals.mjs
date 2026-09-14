import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { BrowserTrackResponse } from "../src/browser/response.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { TrackResponseModule, parseTrackResponseState } from "../src/core/live-response.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
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

function responseCapture(sectionCount = 6) {
  const resultLayout = ABI_LAYOUT.structures.liveResponseResult;
  const ownerLayout = ABI_LAYOUT.structures.liveResponseOwner;
  const sectionLayout = ABI_LAYOUT.structures.liveResponseSection;
  const fieldOffset = (layout, name) => layout.fields.find((field) => field.name === name).offset;
  const resultBytes = resultLayout.bytes + ownerLayout.bytes + 3
    + sectionCount * sectionLayout.bytes * 2;
  const capture = new Uint8Array(resultBytes);
  const view = new DataView(capture.buffer);
  const setU32 = (layout, base, name, value) => view.setUint32(base + fieldOffset(layout, name), value, true);
  const setU64 = (layout, base, name, value) => view.setBigUint64(base + fieldOffset(layout, name), value, true);
  setU32(resultLayout, 0, "structSize", resultLayout.bytes);
  setU32(resultLayout, 0, "abiVersion", ABI_LAYOUT.abiVersion);
  setU32(resultLayout, 0, "result", 0);
  setU32(resultLayout, 0, "mode", 1);
  setU32(resultLayout, 0, "meaning", 1);
  setU32(resultLayout, 0, "channels", 3);
  setU32(resultLayout, 0, "points", 0);
  setU32(resultLayout, 0, "ownerCount", 1);
  setU32(resultLayout, 0, "excludedCount", 0);
  setU32(resultLayout, 0, "sampleRateHz", 48_000);
  setU64(resultLayout, 0, "capturedSample", 7n);
  setU64(resultLayout, 0, "snapshotToken", 9n);
  setU64(resultLayout, 0, "resultBytes", BigInt(resultBytes));
  const owner = resultLayout.bytes;
  const strings = owner + ownerLayout.bytes;
  const left = strings + 3;
  const right = left + sectionCount * sectionLayout.bytes;
  setU32(resultLayout, 0, "ownersOffset", owner);
  setU32(resultLayout, 0, "ownerRecordBytes", ownerLayout.bytes);
  setU32(resultLayout, 0, "sectionRecordBytes", sectionLayout.bytes);
  setU32(ownerLayout, owner, "trackIdOffset", strings);
  setU32(ownerLayout, owner, "trackIdBytes", 1);
  setU32(ownerLayout, owner, "nativeIdOffset", strings + 1);
  setU32(ownerLayout, owner, "nativeIdBytes", 1);
  setU32(ownerLayout, owner, "stableIdOffset", strings + 2);
  setU32(ownerLayout, owner, "stableIdBytes", 1);
  setU32(ownerLayout, owner, "rack", 1);
  setU32(ownerLayout, owner, "slot", 2);
  setU32(ownerLayout, owner, "kind", 1);
  setU32(ownerLayout, owner, "availability", 1);
  setU32(ownerLayout, owner, "leftOffset", left);
  setU32(ownerLayout, owner, "leftCount", sectionCount);
  setU32(ownerLayout, owner, "rightOffset", right);
  setU32(ownerLayout, owner, "rightCount", sectionCount);
  for (let index = 0; index < sectionCount; index += 1) {
    for (const [base, enabled] of [[left, index === 0], [right, index === 0]]) {
      const section = base + index * sectionLayout.bytes;
      setU32(sectionLayout, section, "id", index + 1);
      setU32(sectionLayout, section, "kind", 1);
      setU32(sectionLayout, section, "enabled", enabled ? 1 : 0);
      setU32(sectionLayout, section, "wordCount", 0);
    }
  }
  capture.set(new TextEncoder().encode("tns"), strings);
  return capture;
}

function responseParserModule() {
  const memory = new WebAssembly.Memory({ initial: 1 });
  return new TrackResponseModule({
    memory,
    miso_engine_web_v1_track_response_request_ptr: () => 64,
    miso_engine_web_v1_track_response_request_bytes: () => 48,
    miso_engine_web_v1_track_response_track_id_ptr: () => 128,
    miso_engine_web_v1_track_response_track_id_capacity: () => 127,
    miso_engine_web_v1_track_response_analysis: () => 0,
    miso_engine_web_v1_track_response_result_ptr: () => 256,
    miso_engine_web_v1_track_response_result_bytes: () => 0,
    miso_engine_web_v1_track_response_snapshot_ptr: () => 512,
    miso_engine_web_v1_track_response_snapshot_capacity: () => 1 << 20,
    miso_engine_web_v1_track_response_snapshot_set_bytes: () => 0,
  });
}

test("live response parsers accept six sections and reject seven or truncated records", () => {
  const valid = responseCapture();
  const state = parseTrackResponseState(valid);
  const module = responseParserModule();
  const query = {
    trackId: "t",
    grid: { kind: "linear", points: 2, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
  };
  assert.equal(module.querySnapshotIfChanged(query, valid, state).changed, false);

  const ownerLayout = ABI_LAYOUT.structures.liveResponseOwner;
  const resultLayout = ABI_LAYOUT.structures.liveResponseResult;
  const fieldOffset = (layout, name) => layout.fields.find((field) => field.name === name).offset;
  const seven = valid.slice();
  new DataView(seven.buffer).setUint32(
    resultLayout.bytes + fieldOffset(ownerLayout, "leftCount"),
    7,
    true,
  );
  const truncated = valid.slice(0, -1);
  new DataView(truncated.buffer).setBigUint64(
    fieldOffset(resultLayout, "resultBytes"),
    BigInt(truncated.byteLength),
    true,
  );
  for (const malformed of [seven, truncated]) {
    assert.throws(() => parseTrackResponseState(malformed), MisoEngineError);
    assert.throws(() => module.querySnapshotIfChanged(query, malformed, state), MisoEngineError);
  }
  module.close();
});

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
    assert.deepEqual(first.members[1].enabledLeft, [true, false, false, false, false, false]);
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
