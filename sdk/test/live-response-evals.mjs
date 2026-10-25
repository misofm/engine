import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
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
import {
  WIRE_BOOL, WIRE_F32, WIRE_MESSAGE, WIRE_PACKED_F32, WIRE_U16, WIRE_U32, WIRE_U64, WIRE_U8, WIRE_UTF8,
  concat, field, messageBody,
} from "../src/core/control-frame.ts";
import { TrackResponseRequester } from "../src/core/live-response.ts";
import { effectEntry, engineHost, moduleBytes, ramp, sessionDocument } from "./support.mjs";

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

const FIXTURE_EVENT = Uint8Array.from(Buffer.from(FIXTURE.eventHex, "hex"));

/**
 * A `RESPONSE_READY` frame of `event` (a decoded event), with the fixture frame's header. The
 * corrupt-frame rows change one value of a decoded event and encode it again.
 */
function readyFrame(event) {
  const u64 = (value) => { const out = Buffer.alloc(8); out.writeBigUInt64LE(value); return out; };
  const u32 = (value) => { const out = Buffer.alloc(4); out.writeUInt32LE(value); return out; };
  const u16 = (value) => { const out = Buffer.alloc(2); out.writeUInt16LE(value); return out; };
  const f32 = (value) => { const out = Buffer.alloc(4); out.writeFloatLE(value); return out; };
  const utf8 = (value) => new TextEncoder().encode(value);
  const floats = (values) => new Uint8Array(Float32Array.from(values).buffer);
  const fields = [
    field(1, WIRE_U64, true, u64(event.sequence)),
    field(2, WIRE_U64, true, u64(event.requestTag)),
    field(3, WIRE_UTF8, true, utf8(event.stripId)),
    field(4, WIRE_U16, true, u16(0)),
  ];
  const { capture } = event;
  fields.push(
    field(6, WIRE_U64, false, u64(capture.capturedSample)),
    field(7, WIRE_U32, false, u32(capture.sampleRateHz)),
    field(8, WIRE_F32, false, f32(capture.floorDb)),
    field(9, WIRE_PACKED_F32, false, floats(capture.frequenciesHz)),
  );
  if (capture.leftDb !== undefined) fields.push(field(10, WIRE_PACKED_F32, false, floats(capture.leftDb)));
  if (capture.rightDb !== undefined) fields.push(field(11, WIRE_PACKED_F32, false, floats(capture.rightDb)));
  fields.push(field(12, WIRE_U32, false, u32(capture.excludedCount)));
  for (const member of capture.members) {
    const inner = [field(1, WIRE_U8, true, Uint8Array.of(member.section))];
    if (member.slotId !== undefined) inner.push(field(2, WIRE_UTF8, false, utf8(member.slotId)));
    inner.push(
      field(3, WIRE_UTF8, true, utf8(member.effect)),
      field(4, WIRE_U8, true, Uint8Array.of(member.kind)),
      field(5, WIRE_BOOL, true, Uint8Array.of(member.bypassed ? 1 : 0)),
      field(6, WIRE_U32, true, u32(member.enabledLeft)),
      field(7, WIRE_U32, true, u32(member.enabledRight)),
      field(8, WIRE_U8, true, Uint8Array.of(member.sectionCount)),
    );
    fields.push(field(13, WIRE_MESSAGE, false, messageBody(inner)));
  }
  const payload = concat(fields);
  const head = Buffer.from(FIXTURE_EVENT.subarray(0, 48));
  head.writeUInt32LE(payload.length, 20);
  head.writeUInt32LE(fields.length, 40);
  return Uint8Array.from(Buffer.concat([head, payload]));
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
  const frame = FIXTURE_EVENT;
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

  // The decoder refuses what the core's codec refuses: each row breaks one rule of
  // `response_ready_is_valid` or of the frame header, and is a typed refusal, not a result. Test
  // value: red if the decoder hands back a capture the core would never have encoded.
  assert.equal(hexOf(readyFrame(event)), FIXTURE.eventHex, "the eval's encoder writes the core's bytes");
  const member = (index, change) => ({
    ...event,
    capture: { ...event.capture, members: event.capture.members.map((row, at) => (at === index ? { ...row, ...change } : row)) },
  });
  const capture = (change) => ({ ...event, capture: { ...event.capture, ...change } });
  const header = (offset, write) => {
    const bytes = Buffer.from(frame);
    write(bytes, offset);
    return Uint8Array.from(bytes);
  };
  const rows = [
    ["header magic", header(0, (bytes) => { bytes[0] ^= 1; })],
    ["header version", header(8, (bytes, at) => bytes.writeUInt16LE(2, at))],
    ["header length", header(12, (bytes, at) => bytes.writeUInt16LE(40, at))],
    ["event flags", header(15, (bytes, at) => { bytes[at] = 1; })],
    ["request ID", header(24, (bytes, at) => bytes.writeBigUInt64LE(1n, at))],
    ["mask bit past the section count", readyFrame(member(0, { enabledLeft: 0b111 }))],
    ["non-finite floor", readyFrame(capture({ floorDb: Number.POSITIVE_INFINITY }))],
    ["side length", readyFrame(capture({ leftDb: event.capture.leftDb.subarray(1) }))],
    ["text length", readyFrame(member(1, { effect: `miso.${"x".repeat(123)}` }))],
    ["stable ID", readyFrame({ ...event, stripId: "Eq0" })],
  ];
  for (const [rule, corrupt] of rows) {
    assert.throws(() => decodeResponseReady(corrupt),
      (error) => error instanceof MisoEngineError && error.code === "internal", rule);
  }
});

/** A tag reply to the `RESPONSE_GET` of `requestId`, as the core answers a strip request. */
function tagReply(requestId, tag) {
  const value = Buffer.alloc(8);
  value.writeBigUInt64LE(tag);
  const payload = field(1, WIRE_U64, false, value);
  const head = Buffer.alloc(48);
  Buffer.from(FIXTURE_EVENT.subarray(0, 8)).copy(head);
  head.writeUInt16LE(1, 8);
  head.writeUInt16LE(48, 12);
  head[14] = 2;
  head.writeUInt16LE(14, 16);
  head.writeUInt32LE(payload.length, 20);
  head.writeBigUInt64LE(requestId, 24);
  head.writeUInt32LE(1, 40);
  return Uint8Array.from(Buffer.concat([head, payload]));
}

// The late tag reply (the browser's transport is asynchronous; the headless send returns before a
// deadline can pass). Test value: red if a tag that arrives after its query's deadline is not held,
// so the next query sends while that capture is still armed in the core, or if the requester stops
// listening before the held tag's event is read.
test("a tag reply after the deadline is held, and the next query waits for its event", async () => {
  const base = decodeResponseReady(FIXTURE_EVENT);
  const listeners = new Set();
  const replies = [];
  let nextId = 0n;
  const requester = new TrackResponseRequester({
    nextRequestId: () => (nextId += 1n),
    onControlEvent(listener) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    send(frame) {
      const requestId = Buffer.from(frame).readBigUInt64LE(24);
      return new Promise((resolve) => replies.push({ requestId, resolve }));
    },
  });
  const emit = (tag) => {
    const frame = readyFrame({ ...base, requestTag: tag });
    for (const listener of [...listeners]) listener({ lane: 0, frame });
  };
  const ticks = async () => {
    for (let tick = 0; tick < 8; tick += 1) await new Promise((resolve) => setImmediate(resolve));
  };
  const request = { ...FIXTURE.request, responseLimits: { requestDeadlineMs: 20 } };
  const first = requester.query(request);
  const second = requester.query({ ...request, responseLimits: { requestDeadlineMs: 2_000 } });
  await assert.rejects(first, /exceeded its deadline/);
  await ticks();
  assert.equal(replies.length, 1, "the next query waits while the late reply is outstanding");
  replies[0].resolve(tagReply(replies[0].requestId, 7n));
  await ticks();
  assert.equal(replies.length, 1, "the next query waits for the held tag's event");
  assert.ok(listeners.size > 0, "the requester listens while it holds a tag");
  emit(7n);
  await ticks();
  assert.equal(replies.length, 2);
  replies[1].resolve(tagReply(replies[1].requestId, 8n));
  await ticks();
  emit(8n);
  assert.equal((await second).snapshotToken, 8n);
  await ticks();
  assert.equal(listeners.size, 0, "idle again: no listener left");
});

// #1579 gate 1, the headless and browser legs. Test value: red if the headless engine or the
// browser engine answers the fixture's question with other members or other curve bits than the
// C ABI did for the same document (the browser keeping a path of its own, or reading other
// frames), if `onEvent` hands the answering `RESPONSE_READY` to the protocol event decoder (which
// refuses it), or if a browser engine's close, or its host's failure, leaves a pending query to its
// deadline.
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

  // The browser engine sends the same frame through its host's `control` message and reads the
  // event from the host's event lane: here a stand-in over another core of the same document.
  const served = await createOfflineEngine(FIXTURE.document, { asset });
  const failingCore = await createOfflineEngine(FIXTURE.document, { asset });
  // `2` ready, `3` failed: the shipped host's status states.
  let hostState = 2;
  const boot = (core) => createEngine({
    document: new Uint8Array([1]),
    scratchBoot: async () => core.shape(),
    createContext: () => ({
      sampleRate: 48_000, renderQuantumSize: 128, state: "running",
      audioWorklet: { async addModule() {} }, async close() {},
    }),
    createHost: async () => ({
      ...engineHost(core),
      async status() { return { result: 0, state: hostState, lastResult: 0 }; },
      async dispose() {},
    }),
  });
  const ticks = async () => {
    for (let tick = 0; tick < 8; tick += 1) await new Promise((resolve) => setImmediate(resolve));
  };
  const browser = await boot(served);
  const failing = await boot(failingCore);
  try {
    const result = await renderUntil(served, browser.queryTrackResponse(FIXTURE.request), 32);
    assert.deepEqual(comparable(result), expectedResult(FIXTURE.expect));
    const pending = browser.queryTrackResponse(FIXTURE.request);
    await ticks();
    const closedAt = Date.now();
    await browser.close();
    await assert.rejects(pending, (error) => error instanceof MisoUsageError && /closed/.test(error.message));
    assert.ok(Date.now() - closedAt < 1_000, "rejected at close, not at the deadline");

    // A host that fails while a query waits rejects it at once with a lifecycle error.
    const lost = failing.queryTrackResponse(FIXTURE.request);
    await ticks();
    const failedAt = Date.now();
    hostState = 3;
    await assert.rejects(lost, (error) => error instanceof MisoEngineError && error.phase === "lifecycle");
    assert.ok(Date.now() - failedAt < 1_000, "rejected at the host failure, not at the deadline");
  } finally {
    await browser.close();
    await failing.close();
    served.dispose();
    failingCore.dispose();
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

    // An idle engine sends the request before `queryTrackResponse` returns, so the render right
    // after the call answers it. Test value: red if the send waits for a later turn and misses the
    // render the app makes at once.
    feed(2);
    const sync = engine.queryTrackResponse({ ...request, responseLimits: { requestDeadlineMs: 2_000 } });
    engine.render();
    assert.ok((await sync).capturedSample > edited.capturedSample);

    // A query that timed out keeps its capture in the core until a render answers it; the next
    // query waits for that answer (its own deadline covers the wait) instead of being refused.
    // Test value: red if a query after a timeout is refused `response.capture.pending`.
    await assert.rejects(
      engine.queryTrackResponse({ ...request, responseLimits: { requestDeadlineMs: 20 } }),
      /exceeded its deadline/,
    );
    const afterTimeout = await renderUntil(engine, engine.queryTrackResponse(request));
    assert.equal(afterTimeout.members.length, 3);

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
