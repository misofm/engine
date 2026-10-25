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
import { trackResponseResultOf } from "../src/core/live-response.ts";
import {
  decodeResponseReady,
  decodeResponseTagReply,
  encodeStripResponseRequest,
} from "../src/core/response-frame.ts";
import { readFileSync } from "node:fs";
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
  // `4` is the owner record's console code (S1c); `1` and `3` are retired.
  setU32(ownerLayout, owner, "rack", 4);
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
  // Decision 12 retired the owner record's `1` and `3` (simd1, simd2): never reinterpreted, and a
  // code nobody allocated is refused the same way. Red mutation: decode the owner rack through the
  // retired positional table -> `1` and `3` parse as racks and this goes red.
  for (const code of [0, 2, 4]) {
    const recoded = valid.slice();
    new DataView(recoded.buffer).setUint32(resultLayout.bytes + fieldOffset(ownerLayout, "rack"), code, true);
    assert.doesNotThrow(() => module.querySnapshotIfChanged(query, recoded, parseTrackResponseState(recoded)));
  }
  for (const retired of [1, 3, 5, 255]) {
    const recoded = valid.slice();
    new DataView(recoded.buffer).setUint32(resultLayout.bytes + fieldOffset(ownerLayout, "rack"), retired, true);
    assert.throws(
      () => module.querySnapshotIfChanged(query, recoded, parseTrackResponseState(recoded)),
      /unknown live response rack/,
      `owner rack ${retired} must be refused`,
    );
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

// The C ABI's strip request and the `RESPONSE_READY` it answered, for the same document
// (`crates/capi/tests/response_strip.rs` writes it).
const FIXTURE = JSON.parse(readFileSync(new URL("./fixtures/response-ready.json", import.meta.url), "utf8"));

function hexOf(bytes) {
  return Buffer.from(bytes).toString("hex");
}

function bitsOf(values) {
  if (values === undefined) return null;
  return [...new Uint32Array(values.buffer, values.byteOffset, values.length)]
    .map((word) => word.toString(16).padStart(8, "0"));
}

/** What D3 says the SDK result of the fixture's event is, built from the protocol codec's values. */
function expectedResult(expect) {
  const racks = new Map([[4, "input"], [5, "console"], [2, "inserts"]]);
  const kinds = new Map([[1, "parametricEq"], [2, "inputFilters"], [3, "unavailable"]]);
  const bits = (mask, count) => Array.from({ length: count }, (_unused, bit) => ((mask >>> bit) & 1) === 1);
  return {
    trackId: expect.stripId,
    sampleRateHz: expect.sampleRateHz,
    floorDb: bitsOf(new Float32Array(new Uint32Array([Number.parseInt(expect.floorDb, 16)]).buffer))[0],
    excludedMemberCount: expect.excludedCount,
    frequenciesHz: expect.frequenciesHz,
    leftDb: expect.leftDb,
    rightDb: expect.rightDb,
    members: expect.members.map((member, slot) => ({
      trackId: expect.stripId,
      nativeId: member.effect,
      stableId: member.slotId ?? "input-filters",
      rack: racks.get(member.section),
      rackValue: member.section,
      slot,
      kind: kinds.get(member.kind),
      kindValue: member.kind,
      bypassed: member.bypassed,
      available: member.kind !== 3,
      excludedReason: member.kind === 3 ? "linearResponseUnavailable" : undefined,
      enabledLeft: bits(member.enabledLeft, member.sectionCount),
      enabledRight: bits(member.enabledRight, member.sectionCount),
    })),
  };
}

/** The fields of a result that the core decides, curves by bits. */
function comparable(result) {
  return {
    trackId: result.trackId,
    sampleRateHz: result.sampleRateHz,
    floorDb: bitsOf(Float32Array.of(result.floorDb))[0],
    excludedMemberCount: result.excludedMemberCount,
    frequenciesHz: bitsOf(result.frequenciesHz),
    leftDb: bitsOf(result.leftDb),
    rightDb: bitsOf(result.rightDb),
    members: result.members.map((member) => ({ ...member, excludedReason: member.excludedReason })),
  };
}

/** Render blocks until `promise` settles; at most `blocks`. */
async function renderUntil(engine, promise, blocks = 8) {
  let settled = false;
  const observed = promise.finally(() => { settled = true; });
  observed.catch(() => {});
  for (let block = 0; block < blocks && !settled; block += 1) {
    await new Promise((resolve) => setImmediate(resolve));
    if (!settled) engine.render();
  }
  return observed;
}

// #1579a gate 1, the codec half. Test value: red if the SDK spells the strip request differently
// from the C ABI, or maps a member or curve field of `RESPONSE_READY` differently from the core's
// event (a section code, a slot ID, a mask bit, a curve word, the tag or the frame length).
test("the strip request and RESPONSE_READY codec match the C ABI's frames", () => {
  const request = FIXTURE.request;
  const encoded = encodeStripResponseRequest({
    requestId: BigInt(request.requestId),
    stripId: request.trackId,
    grid: request.grid.kind,
    points: request.grid.points,
    minimumHz: request.grid.minimumHz,
    maximumHz: request.grid.maximumHz,
    channels: request.channels,
  });
  assert.equal(hexOf(encoded), FIXTURE.requestHex);
  const frame = Uint8Array.from(Buffer.from(FIXTURE.eventHex, "hex"));
  const event = decodeResponseReady(frame);
  assert.ok(event);
  assert.equal(event.requestTag, BigInt(FIXTURE.expect.requestTag));
  const result = trackResponseResultOf(event);
  assert.deepEqual(comparable(result), expectedResult(FIXTURE.expect));
  assert.equal(result.mode, "target");
  assert.equal(result.meaning, "eqFilterSubtotal");
  assert.equal(result.capturedSample, BigInt(FIXTURE.expect.capturedSample));
  assert.equal(result.snapshotToken, BigInt(FIXTURE.expect.requestTag));
  assert.equal(result.resultBytes, BigInt(frame.length));
});

// #1579a gate 1, the headless leg. Test value: red if the headless engine answers the fixture's
// question with other members or other curve bits than the C ABI did for the same document, or if
// `onEvent` hands the answering `RESPONSE_READY` to the protocol event decoder (which refuses it).
test("candidate Wasm answers the C ABI fixture's question with the C ABI's result", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const engine = await createOfflineEngine(FIXTURE.document, { asset });
  try {
    const kinds = [];
    engine.onEvent((event) => kinds.push(event.kind));
    const result = await renderUntil(engine, engine.queryTrackResponse(FIXTURE.request));
    assert.deepEqual(comparable(result), expectedResult(FIXTURE.expect));
    assert.equal(engine.controlListenerFailures, 0);
  } finally {
    engine.dispose();
  }
});

test("candidate Wasm reads the applied state at a render, one capture at a time", {
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
      preInsert: [effectEntry("eq", "miso.parametric-eq", params)],
      inserts: [effectEntry("comp", "miso.compressor")],
      postInsert: [],
    },
  }), { asset });
  const request = {
    trackId: "t",
    grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
  };
  const shape = engine.shape();
  const feed = (block) => {
    for (const source of shape.sources) {
      engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: BigInt(block * shape.quantumFrames),
        planes: Array.from({ length: source.channels }, (_unused, channel) => ramp(shape.quantumFrames, 13 + channel)),
        endOfRegion: false,
      });
    }
  };
  try {
    feed(0);
    const first = await renderUntil(engine, engine.queryTrackResponse(request));
    assert.equal(first.mode, "target");
    assert.equal(first.meaning, "eqFilterSubtotal");
    assert.equal(first.sampleRateHz, 48_000);
    assert.equal(first.frequenciesHz.length, 16);
    assert.equal(first.leftDb.length, 16);
    assert.equal(first.rightDb.length, 16);
    assert.deepEqual(first.members.map((member) => [member.rack, member.stableId, member.available]), [
      ["input", "input-filters", true],
      ["console", "eq", true],
      ["inserts", "comp", false],
    ]);
    // The decision-17 section codes: `4` input filters, `5` console, `2` inserts.
    assert.deepEqual(first.members.map((member) => member.rackValue), [4, 5, 2]);
    assert.deepEqual(first.members[1].enabledLeft, [true, false, false, false, false, false]);
    assert.equal(first.members[2].excludedReason, "linearResponseUnavailable");
    const firstLeft = first.leftDb.slice();

    // #1579a gate 2. Test value: red if the SDK reads a stale curve (the one before the edit), or
    // if the headless query settles without a render.
    const liveControls = engine.liveControls();
    await liveControls.submit(
      liveControls.edit.track("t").console("eq", "miso.parametric-eq").parameter("band-1-gain", -3),
    );
    let settled = false;
    const afterEdit = engine.queryTrackResponse(request).finally(() => { settled = true; });
    for (let tick = 0; tick < 4; tick += 1) await new Promise((resolve) => setImmediate(resolve));
    assert.equal(settled, false, "the query waits for a render");
    feed(1);
    const edited = await renderUntil(engine, afterEdit);
    assert.ok(edited.capturedSample > first.capturedSample);
    assert.ok(edited.snapshotToken > first.snapshotToken);
    assert.notDeepEqual([...edited.leftDb], [...firstLeft]);
    assert.deepEqual([...first.leftDb], [...firstLeft]);

    // #1579a gate 3. Test value: red if two captures can run at once on one engine: the SDK's
    // queries would refuse each other, or a raw second request would be armed.
    const one = engine.queryTrackResponse(request);
    const two = engine.queryTrackResponse(request);
    for (let tick = 0; tick < 4; tick += 1) await new Promise((resolve) => setImmediate(resolve));
    const raw = decodeResponseTagReply(engine.controlFrame(encodeStripResponseRequest({
      requestId: engine.controlTransactions().requestIds.next(),
      stripId: "t",
      grid: "logarithmic",
      points: 16,
      minimumHz: 20,
      maximumHz: 20_000,
      channels: "both",
    })));
    assert.equal(raw.ok, false);
    assert.equal(raw.status, "backpressure");
    assert.equal(raw.diagnostics[0].code, "response.capture.pending");
    const [oneResult, twoResult] = await renderUntil(engine, Promise.all([one, two]));
    assert.ok(twoResult.snapshotToken > oneResult.snapshotToken);
    assert.ok(twoResult.capturedSample > oneResult.capturedSample);

    await assert.rejects(
      renderUntil(engine, engine.queryTrackResponse({ ...request, responseLimits: { maximumResultBytes: 104 } })),
      (error) => error instanceof MisoEngineError && error.code === "response.reply_too_large"
        && error.result === ABI_LAYOUT.constants.resultCodes.find((row) => row.name === "refusedBudget").value,
    );
    // Dispose rejects a pending query at once, with the error a call on a disposed engine gets.
    // Test value: red if a pending query waits for its deadline after the engine is gone.
    const pending = engine.queryTrackResponse(request);
    for (let tick = 0; tick < 4; tick += 1) await new Promise((resolve) => setImmediate(resolve));
    const disposedAt = Date.now();
    engine.dispose();
    await assert.rejects(pending, (error) => error instanceof MisoUsageError && /was disposed/.test(error.message));
    assert.ok(Date.now() - disposedAt < 1_000, "rejected at dispose, not at the deadline");
    await assert.rejects(engine.queryTrackResponse(request), (error) => error instanceof MisoUsageError && /was disposed/.test(error.message));
  } finally {
    if (engine.state() !== "disposed") engine.dispose();
  }
});
