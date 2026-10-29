import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import vm from "node:vm";

import {
  MSB1_CONTROL,
  MSB1_CONTROL_BYTES,
  MSB1_CONTROL_I64_OFFSET,
  MSB1_HEADER_OFFSET,
  MSB1_ID_CAPACITY,
  MSB1_ID_OFFSET,
  MSB1_SLOT_HEADER_BYTES,
  createMsb1Ring,
  msb1RingBytes,
  Msb1RingWriter,
  Msb1RingObserver,
  MSB1_WRAP,
} from "../src/browser/pcm-ring.ts";
import {
  attachEngineFeed,
  linkFeedEngine,
  PcmFeedError,
  PcmRunwayError,
  prepareEngineFeed,
  waitForPcmRunway,
} from "../src/browser/pcm-feed.ts";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { constantValue } from "../src/core/abi.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";
import { MSB1_CONSUMER_CASES, STALE_GENERATION_CODE } from "./msb1-consumer-cases.mjs";

const context = { audioWorklet: { addModule: async () => {} } };

// Issue #1496 D1/D3: the exports either AudioWorklet calls after construction are one table,
// `tools/realtime-policy/render-thread-exports.json`, which the realtime-policy tool also reads to
// prove each wrapped entry's body is a `render_locked` call. Every engine fake below that the PCM
// feed worklet drives is already constructed and ready, so each `miso_engine_web_v1_*` export the
// feed reaches through its `exports` is a post-construction call. `recordFeedCalls` wraps those
// exports; the last test of this file fails on a call the table does not list for the feed.
const renderThreadTable = JSON.parse(await readFile(new URL("../../tools/realtime-policy/render-thread-exports.json", import.meta.url), "utf8"));
assert.equal(renderThreadTable.table, "render-thread-exports");
const feedRenderThreadExports = new Set(renderThreadTable.exports.filter((entry) => entry.callers.some((caller) => caller.worklet === "feed")).map((entry) => entry.name));
const recordedFeedCalls = new Set();
function recordFeedCalls(exports) {
  const prefix = renderThreadTable.prefix;
  return new Proxy(exports, {
    get(target, property, receiver) {
      const value = Reflect.get(target, property, receiver);
      if (typeof property !== "string" || !property.startsWith(prefix)) return value;
      const name = property.slice(prefix.length);
      recordedFeedCalls.add(name);
      // An unlisted export that the fake does not define would stop the run with a bare TypeError
      // before the last test names it.
      if (value === undefined && !feedRenderThreadExports.has(name)) return () => 0;
      if (typeof value !== "function") return value;
      return (...args) => { recordedFeedCalls.add(name); return Reflect.apply(value, receiver, args); };
    },
  });
}

test("actual Wasm paused seek accepts target PCM before the first quantum with old queues full", async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const document = new TextEncoder().encode(sessionDocument({ frames: 480_000 }));
  const options = { sourceRingFrames: 512 };
  const engine = await WasmBoundary.boot(asset, document, options);
  const oracle = await WasmBoundary.boot(asset, document, options);
  try {
    const shape = engine.shape(), sourceId = shape.sources[0].id, quantum = shape.quantumFrames;
    const oldPlanes = [new Float32Array(quantum).fill(0.25), new Float32Array(quantum).fill(-0.25)];
    for (let block = 0; block < options.sourceRingFrames / quantum; block += 1) {
      assert.equal(engine.submitSource({ sourceId, generation: 1n, startFrame: BigInt(block * quantum), planes: oldPlanes, endOfRegion: false }).ok, true);
    }
    assert.equal(engine.submitSource({ sourceId, generation: 1n, startFrame: BigInt(options.sourceRingFrames), planes: oldPlanes, endOfRegion: false }).code, "backpressure");
    const ring = createMsb1Ring({ sourceId, channels: 2, frameCapacity: quantum, capacity: 64 });
    const writer = new Msb1RingWriter(ring);
    writer.engage(1n);
    for (let block = 0; block < writer.capacity; block += 1) {
      const planes = writer.reserve(quantum);
      planes[0].set(oldPlanes[0]); planes[1].set(oldPlanes[1]);
      writer.commit({ generation: 1n, startFrame: BigInt(options.sourceRingFrames + block * quantum), frames: quantum, endOfRegion: false });
    }
    writer.seek(2n, 10_000n);
    assert.equal(writer.occupancy, writer.capacity, "the SAB still holds a full stale generation");
    const before = engine.status();
    const seek = { sourceId, generation: 2n, sourceFrame: 10_000n };
    assert.equal(engine.seekSource(seek).ok, true);
    assert.equal(engine.status().nextAbsoluteSample, before.nextAbsoluteSample);
    assert.equal(engine.status().renderedQuanta, before.renderedQuanta);
    const target = {
      sourceId, generation: 2n, startFrame: 10_000n, endOfRegion: false,
      planes: [Float32Array.from({ length: quantum }, (_, i) => (i + 1) / 256), Float32Array.from({ length: quantum }, (_, i) => -(i + 1) / 512)],
    };
    // This is the Rust prerequisite, not a claim that the SDK control port already
    // releases SAB slots: direct admission must work even before the next render.
    const admitted = engine.submitSource(target);
    const first = engine.render(quantum);
    assert.equal(oracle.seekSource(seek).ok, true);
    assert.equal(oracle.submitSource(target).ok, true);
    const expected = oracle.render(quantum);
    assert.ok(expected.left.some((sample) => sample !== 0));
    assert.deepEqual(first, expected, `first target quantum; admission was ${admitted.code}`);
    assert.equal(admitted.ok, true);
    writer.release();
  } finally {
    engine.dispose(); oracle.dispose();
  }
});

function controls(ring) { return new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4); }
function headers(ring, capacity) { return new Int32Array(ring, MSB1_HEADER_OFFSET, capacity * MSB1_SLOT_HEADER_BYTES / 4); }
function headers64(ring, capacity) { return new BigInt64Array(ring, MSB1_HEADER_OFFSET, capacity * MSB1_SLOT_HEADER_BYTES / 8); }
function pcmSnapshot(ring, channels, frameCapacity, capacity) {
  const offset = controls(ring)[MSB1_CONTROL.PCM_OFFSET];
  return Array.from({ length: capacity }, (_, slot) => Array.from({ length: channels }, (_, channel) => [
    ...new Float32Array(ring, offset + (slot * channels + channel) * frameCapacity * 4, frameCapacity),
  ]));
}

async function settleWithin(promise, milliseconds = 50) {
  let timer;
  try {
    return await Promise.race([
      promise,
      new Promise((_, reject) => { timer = setTimeout(() => reject(new Error("observation exceeded bound")), milliseconds); }),
    ]);
  } finally {
    clearTimeout(timer);
  }
}

test("MSB1 layout and writer preserve frozen bytes, headers, counters and reuse", () => {
  const expected = {
    [1]: { bytes: 376, pcm: 336 },
    [2]: { bytes: 416, pcm: 336 },
  };
  for (const channels of [1, 2]) {
    const sourceId = channels === 1 ? "mono" : "stereo";
    const ring = createMsb1Ring({ sourceId, channels, frameCapacity: 5, capacity: 2 });
    assert.equal(ring.byteLength, expected[channels].bytes);
    assert.equal(msb1RingBytes(channels, 5, 2), expected[channels].bytes);
    const c = controls(ring);
    // Issue #1297b D2: the anchor i64 at 128 (-1: none), the ID at 144, the headers at 272.
    assert.equal(MSB1_CONTROL_BYTES, 144);
    assert.equal(MSB1_ID_OFFSET, 144);
    assert.equal(MSB1_ID_CAPACITY, 128);
    assert.equal(MSB1_HEADER_OFFSET, 272);
    assert.equal(MSB1_SLOT_HEADER_BYTES, 32);
    assert.deepEqual([...c.slice(0, 28)], [
      0x4d534231, 1, 2, channels, 5, 272, expected[channels].pcm, sourceId.length,
      0, 0, 1, ...Array(17).fill(0),
    ]);
    assert.deepEqual([...c.slice(28)], [0, 0, 0, 0, -1, -1, 0, 0]);
    assert.deepEqual([...new Uint8Array(ring, MSB1_ID_OFFSET, sourceId.length)], [...new TextEncoder().encode(sourceId)]);

    const writer = new Msb1RingWriter(ring);
    const generation = 0x1_0000_0007n;
    writer.engage(generation);
    assert.equal(c[MSB1_CONTROL.WRITER_STATE], 1);
    const planes = writer.reserve(3);
    assert.equal(planes.length, channels);
    assert.ok(planes.every((plane) => plane.every((value) => Object.is(value, 0))));
    planes[0].set([1, 2, 3]);
    if (channels === 2) planes[1].set([4, 5, 6]);
    writer.commit({ generation, startFrame: 11n, frames: 3, endOfRegion: true });
    assert.deepEqual([...headers(ring, 2).slice(0, 4)], [0, 7, 3, 1]);
    assert.deepEqual([...headers64(ring, 2).slice(0, 4)], [7n << 32n, (1n << 32n) | 3n, generation, 11n]);
    assert.equal(c[MSB1_CONTROL.GENERATION_TAG], 7);
    assert.equal(c[MSB1_CONTROL.WRITE_INDEX], 1);
    assert.equal(c[MSB1_CONTROL.WROTE], 1);
    assert.equal(writer.occupancy, 1);
    assert.deepEqual([...planes[0]], [1, 2, 3, 0, 0]);
    const nextGeneration = 0x2_0000_0008n;
    writer.seek(nextGeneration, 14n);
    assert.deepEqual([...new BigInt64Array(ring, 112, 3)], [nextGeneration, 14n, -1n]);
    assert.equal(c[MSB1_CONTROL.GENERATION_TAG], 8);
    assert.equal(c[MSB1_CONTROL.SEEK_EPOCH], 2, "a seek steps the parity epoch by 2 (#2184)");
    writer.seek(nextGeneration + 1n, 15n, 4_096n);
    assert.deepEqual([...new BigInt64Array(ring, 112, 3)], [nextGeneration + 1n, 15n, 4_096n]);
    assert.deepEqual([...new Uint8Array(ring, MSB1_ID_OFFSET, sourceId.length)], [...new TextEncoder().encode(sourceId)], "the anchor leaves the ID");
    assert.equal(c[MSB1_CONTROL.SEEK_EPOCH], 4);
    writer.release();
    assert.equal(c[MSB1_CONTROL.WRITER_STATE], 0);
  }

  const exactId = "é".repeat(64);
  const exactIdBytes = new TextEncoder().encode(exactId);
  assert.equal(exactIdBytes.byteLength, MSB1_ID_CAPACITY);
  const exactIdRing = createMsb1Ring({ sourceId: exactId, channels: 1, frameCapacity: 4, capacity: 2 });
  assert.deepEqual([...new Uint8Array(exactIdRing, MSB1_ID_OFFSET, MSB1_ID_CAPACITY)], [...exactIdBytes]);
  assert.equal(new TextDecoder().decode(new Uint8Array(exactIdRing, MSB1_ID_OFFSET, MSB1_ID_CAPACITY)), exactId);
  assert.throws(() => createMsb1Ring({ sourceId: `${exactId}x`, channels: 1, frameCapacity: 4, capacity: 2 }), /does not fit/);

  const ring = createMsb1Ring({ sourceId: "reuse", channels: 2, frameCapacity: 4, capacity: 2 });
  const writer = new Msb1RingWriter(ring);
  writer.engage(1n);
  for (let i = 0; i < 2; i += 1) {
    const planes = writer.reserve(4);
    planes[0].fill(i + 1); planes[1].fill(i + 2);
    writer.commit({ generation: 1n, startFrame: BigInt(i * 4), frames: 4, endOfRegion: false });
  }
  const before = [...headers(ring, 2)];
  const beforePcm = pcmSnapshot(ring, 2, 4, 2);
  const beforeWriteIndex = controls(ring)[MSB1_CONTROL.WRITE_INDEX];
  const beforeOverflow = controls(ring)[MSB1_CONTROL.OVERFLOW];
  assert.equal(writer.reserve(4), null);
  assert.equal(controls(ring)[MSB1_CONTROL.OVERFLOW], beforeOverflow + 1);
  assert.deepEqual([...headers(ring, 2)], before);
  assert.deepEqual(pcmSnapshot(ring, 2, 4, 2), beforePcm);
  assert.equal(controls(ring)[MSB1_CONTROL.WRITE_INDEX], beforeWriteIndex);
  Atomics.store(controls(ring), MSB1_CONTROL.READ_INDEX, 1);
  const reused = writer.reserve(4);
  assert.ok(reused); assert.ok(reused.every((plane) => plane.every((value) => Object.is(value, 0))));
  writer.commit({ generation: 1n, startFrame: 8n, frames: 4, endOfRegion: false });
  assert.equal(writer.occupancy, 2);
  assert.equal(controls(ring)[MSB1_CONTROL.WRITE_INDEX], 3);
  assert.equal(controls(ring)[MSB1_CONTROL.READ_INDEX], 1);
  assert.deepEqual([...headers(ring, 2).slice(0, 4)], [2, 1, 4, 0]);

  assert.throws(() => new Msb1RingWriter(new ArrayBuffer(128)), /SharedArrayBuffer/);
  const malformed = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 2 });
  new Int32Array(malformed)[MSB1_CONTROL.VERSION] = 2;
  assert.throws(() => new Msb1RingWriter(malformed), /MSB1/);
  const badMagic = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 2 });
  new Int32Array(badMagic)[MSB1_CONTROL.MAGIC] = 0;
  assert.throws(() => new Msb1RingWriter(badMagic), /MSB1/);
  const zeroCapacity = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 2 });
  new Int32Array(zeroCapacity)[MSB1_CONTROL.CAPACITY] = 0;
  assert.throws(() => new Msb1RingWriter(zeroCapacity), /header is invalid/);
  const nonPowerCapacity = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 4 });
  controls(nonPowerCapacity)[MSB1_CONTROL.CAPACITY] = 3;
  assert.throws(() => new Msb1RingWriter(nonPowerCapacity), /header is invalid/);
  const zeroFrameCapacity = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 2 });
  new Int32Array(zeroFrameCapacity)[MSB1_CONTROL.FRAME_CAPACITY] = 0;
  assert.throws(() => new Msb1RingWriter(zeroFrameCapacity), /header is invalid/);
  assert.throws(() => createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 3 }), /power of two/);

  const wrapped = createMsb1Ring({ sourceId: "wrapped", channels: 1, frameCapacity: 1, capacity: 2 });
  const wrappedWriter = new Msb1RingWriter(wrapped);
  const wrappedControls = controls(wrapped);
  wrappedWriter.engage(1n);
  Atomics.store(wrappedControls, MSB1_CONTROL.READ_INDEX, (1 << 30) - 1);
  Atomics.store(wrappedControls, MSB1_CONTROL.WRITE_INDEX, (1 << 30) - 1);
  wrappedWriter.reserve(1)[0].set([9]);
  wrappedWriter.commit({ generation: 1n, startFrame: 0n, frames: 1, endOfRegion: false });
  assert.equal(wrappedControls[MSB1_CONTROL.WRITE_INDEX], 0);
  assert.equal(wrappedWriter.occupancy, 1);
  Atomics.store(wrappedControls, MSB1_CONTROL.READ_INDEX, 0);
  wrappedWriter.reserve(1)[0].set([8]);
  wrappedWriter.commit({ generation: 1n, startFrame: 1n, frames: 1, endOfRegion: false });
  assert.equal(wrappedControls[MSB1_CONTROL.WRITE_INDEX], 1);
  assert.equal(wrappedWriter.occupancy, 1);
});

function attachNode({ onMessage = () => {}, onDisconnect = () => {} } = {}) {
  return { port: { postMessage: onMessage }, disconnect: onDisconnect };
}

test("feed lifecycle preserves URL, typed seams, identity and terminal cleanup", async () => {
  const urls = [];
  await prepareEngineFeed({ audioWorklet: { addModule: async (url) => urls.push(url) } });
  await prepareEngineFeed({ audioWorklet: { addModule: async (url) => urls.push(url) } }, "https://example.test/feed.js");
  assert.equal(urls[0], String(BUNDLED_ENGINE_ASSETS.pcmFeedWorklet));
  assert.equal(urls[1], "https://example.test/feed.js");
  await assert.rejects(prepareEngineFeed({ audioWorklet: { addModule: async () => { throw new Error("no module"); } } }), (error) => error instanceof PcmFeedError && error.operation === "moduleLoad");

  let defaultConstructed = false;
  const oldNode = globalThis.AudioWorkletNode;
  globalThis.AudioWorkletNode = class { constructor() { defaultConstructed = true; } };
  const posted = [];
  let factoryContext;
  let factoryOptions;
  const node = attachNode({ onMessage: (message) => posted.push(message) });
  const sources = [{ sourceId: "mono", channels: 1 }, { sourceId: "stereo", channels: 2 }, { sourceId: "tail", channels: 1 }];
  const feed = attachEngineFeed({ context, sources, quantumFrames: 4, createNode: (received, name, options) => { factoryContext = received; factoryOptions = { name, options }; return node; } });
  assert.equal(defaultConstructed, false);
  assert.equal(factoryContext, context);
  assert.deepEqual(factoryOptions, { name: "miso-sab-feed-attach", options: { numberOfInputs: 0, numberOfOutputs: 1 } });
  assert.equal(posted[0].op, "attach");
  assert.deepEqual(posted[0].rings, feed.rings);
  assert.deepEqual(feed.rings.map((ring) => controls(ring)[MSB1_CONTROL.FRAME_CAPACITY]), [4, 4, 4]);
  assert.deepEqual(feed.rings.map((ring) => controls(ring)[MSB1_CONTROL.CAPACITY]), [64, 64, 64]);
  for (const ring of feed.rings) Atomics.store(controls(ring), MSB1_CONTROL.ATTACHED, 1);
  await feed.ready();
  assert.equal(feed.state, "active");
  assert.throws(() => attachEngineFeed({
    context,
    sources: [{ sourceId: "node-failure", channels: 1 }],
    quantumFrames: 4,
    createNode: () => { throw new Error("node unavailable"); },
  }), (error) => error instanceof PcmFeedError && error.operation === "nodeCreate");
  const empty = attachEngineFeed({ context, sources: [], quantumFrames: 4, createNode: () => attachNode() });
  await empty.ready(); assert.equal(empty.state, "active");
  globalThis.AudioWorkletNode = oldNode;

  let waits = 0; let enteredResolve;
  const entered = new Promise((resolve) => { enteredResolve = resolve; });
  const never = new Promise(() => {});
  let disconnects = 0; let detaches = 0;
  const blockedNode = attachNode({ onMessage: (message) => { if (message.op === "detach") detaches += 1; }, onDisconnect: () => { disconnects += 1; } });
  const blocked = attachEngineFeed({ context, sources: [{ sourceId: "blocked", channels: 1 }], quantumFrames: 4, createNode: () => blockedNode });
  new Msb1RingWriter(blocked.rings[0]).engage(1n);
  const blockedWait = async () => {
    waits += 1;
    if (waits === 2) enteredResolve();
    return never;
  };
  const first = blocked.ready({ wait: blockedWait });
  const second = blocked.ready({ wait: blockedWait });
  await settleWithin(entered);
  blocked.close(); blocked.close();
  const blockedResults = await settleWithin(Promise.allSettled([first, second]));
  assert.deepEqual(blockedResults.map((result) => result.status === "rejected" && result.reason.operation), ["closed", "closed"]);
  assert.equal(disconnects, 1); assert.equal(detaches, 1);
  assert.equal(controls(blocked.rings[0])[MSB1_CONTROL.WRITER_STATE], 0);

  let clock = 0; let timeoutDisconnects = 0; let timeoutDetaches = 0;
  const timed = attachEngineFeed({ context, sources: [{ sourceId: "timeout", channels: 1 }], quantumFrames: 4, createNode: () => attachNode({ onMessage: (message) => { if (message.op === "detach") timeoutDetaches += 1; }, onDisconnect: () => { timeoutDisconnects += 1; } }) });
  new Msb1RingWriter(timed.rings[0]).engage(1n);
  let ownerEnteredResolve; let otherEnteredResolve;
  const ownerEntered = new Promise((resolve) => { ownerEnteredResolve = resolve; });
  const otherEntered = new Promise((resolve) => { otherEnteredResolve = resolve; });
  let releaseOwnerWait;
  const ownerWait = new Promise((resolve) => { releaseOwnerWait = resolve; });
  const timing = timed.ready({ timeoutMs: 1, now: () => clock, wait: async () => { ownerEnteredResolve(); await ownerWait; } });
  await settleWithin(ownerEntered);
  const waiting = timed.ready({ timeoutMs: 1, now: () => clock, wait: async () => { otherEnteredResolve(); return never; } });
  await settleWithin(otherEntered);
  clock = 2;
  releaseOwnerWait();
  const timeoutResults = await settleWithin(Promise.allSettled([timing, waiting]));
  assert.deepEqual(timeoutResults.map((result) => result.status === "rejected" && result.reason.operation), ["readyTimeout", "closed"]);
  assert.equal(timeoutDisconnects, 1); assert.equal(timeoutDetaches, 1);
  assert.equal(controls(timed.rings[0])[MSB1_CONTROL.WRITER_STATE], 0);
  timed.close();
  assert.equal(timeoutDisconnects, 1); assert.equal(timeoutDetaches, 1);
});

test("attach-post cleanup releases engaged rings before throwing cleanup failures", () => {
  let messageRings = [];
  let disconnectStates = [];
  let disconnects = 0;
  let detaches = 0;
  assert.throws(() => attachEngineFeed({
    context,
    sources: [{ sourceId: "a", channels: 1 }, { sourceId: "b", channels: 2 }, { sourceId: "c", channels: 1 }],
    quantumFrames: 4,
    createNode: () => attachNode({
      onMessage: (message) => {
        if (message.op === "attach") {
          messageRings = message.rings;
          for (const ring of message.rings) Atomics.store(controls(ring), MSB1_CONTROL.WRITER_STATE, 1);
          throw new Error("attach blocked");
        }
        if (message.op === "detach") detaches += 1;
        throw new Error("detach blocked");
      },
      onDisconnect: () => { disconnects += 1; disconnectStates = messageRings.map((ring) => controls(ring)[MSB1_CONTROL.WRITER_STATE]); throw new Error("disconnect blocked"); },
    }),
  }), (error) => error instanceof PcmFeedError && error.operation === "attachPost");
  assert.equal(disconnects, 1);
  assert.equal(detaches, 1);
  assert.ok(messageRings.length === 3);
  assert.deepEqual(disconnectStates, [0, 0, 0]);
  assert.deepEqual(messageRings.map((ring) => controls(ring)[MSB1_CONTROL.WRITER_STATE]), [0, 0, 0]);
});

function preludeHarness(source, { tracking = false } = {}) {
  const registrations = new Map();
  const allocations = [];
  let armed = false;
  class AudioWorkletProcessorFake { constructor() { this.port = { onmessage: null }; } }
  const sandbox = { SharedArrayBuffer, Int32Array, BigInt64Array, Uint8Array, Float32Array, Atomics, TextDecoder, AudioWorkletProcessor: AudioWorkletProcessorFake, registerProcessor(name, constructor) { registrations.set(name, constructor); } };
  for (const name of ["Int8Array", "Uint8Array", "Uint8ClampedArray", "Int16Array", "Uint16Array", "Int32Array", "Uint32Array", "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array"]) {
    const Base = globalThis[name];
    sandbox[name] = class extends Base {
      constructor(...args) { super(...args); if (tracking && armed) allocations.push(name); }
      subarray(...args) { if (tracking && armed) allocations.push(`${name}.subarray`); return super.subarray(...args); }
      slice(...args) { if (tracking && armed) allocations.push(`${name}.slice`); return super.slice(...args); }
    };
  }
  sandbox.globalThis = sandbox;
  vm.runInNewContext(source, sandbox);
  return { sandbox, registrations, allocations, arm: () => { armed = true; } };
}

test("actual Wasm control-port preparation releases full stale SAB before exact first target render", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const instance = await asset.instantiate();
  let handle;
  const exports = { ...instance.exports, miso_engine_web_v1_boot(...args) { handle = instance.exports.miso_engine_web_v1_boot(...args); return handle; } };
  const document = new TextEncoder().encode(sessionDocument({ frames: 480_000 }));
  const boundary = await WasmBoundary.boot({ instantiate: async () => ({ exports }) }, document, { sourceRingFrames: 512 });
  const { sandbox, registrations } = preludeHarness(source);
  let first;
  class Engine {
    constructor() {
      this.quantumFrames = 128; this.maximumSourceChannels = 2;
      this.exports = recordFeedCalls(exports); this.handle = handle; this.memoryBuffer = exports.memory.buffer;
      this.sourceIdPointer = exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", "sourceId"));
      this.sourceIdCapacity = exports.miso_engine_web_v1_buffer_capacity(handle, constantValue("bufferKinds", "sourceId"));
      this.sourcePcm = new Float32Array(this.memoryBuffer, exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", "sourcePcm")), 256);
      this.ready = true; this.disposed = false; this.stickyResult = 0;
    }
    rebindMemoryViews() { this.memoryBuffer = this.exports.memory.buffer; }
    process() { first = boundary.render(128); return true; }
  }
  sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
  const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
  const attach = new (registrations.get("miso-sab-feed-attach"))();
  const port = { onmessage: null, postMessage(data) { const cloned = structuredClone(data); queueMicrotask(() => attach.port.onmessage({ data: cloned })); } };
  attach.port.postMessage = (data) => { const cloned = structuredClone(data); queueMicrotask(() => port.onmessage?.({ data: cloned })); };
  const suspended = { ...context, state: "suspended" };
  const feed = attachEngineFeed({ context: suspended, sources: [{ sourceId: "s", channels: 2 }], quantumFrames: 128, createNode: () => ({ port, disconnect() {} }) });
  try {
    await feed.ready();
    const writer = new Msb1RingWriter(feed.rings[0]); writer.engage(1n);
    const old = [new Float32Array(128).fill(0.25), new Float32Array(128).fill(-0.25)];
    for (let index = 0; index < 4; index++) assert.equal(boundary.submitSource({ sourceId: "s", generation: 1n, startFrame: BigInt(index * 128), planes: old, endOfRegion: false }).ok, true);
    assert.equal(boundary.submitSource({ sourceId: "s", generation: 1n, startFrame: 512n, planes: old, endOfRegion: false }).code, "backpressure");
    for (let index = 0; index < writer.capacity; index++) {
      const planes = writer.reserve(128); planes[0].set(old[0]); planes[1].set(old[1]);
      writer.commit({ generation: 1n, startFrame: BigInt(512 + index * 128), frames: 128, endOfRegion: false });
    }
    writer.seek(2n, 10_000n);
    await feed.prepareSeek();
    assert.equal(suspended.state, "suspended");
    assert.equal(writer.occupancy, 0);
    assert.equal(boundary.status().nextAbsoluteSample, 0n);
    const c = controls(feed.rings[0]);
    assert.equal(c[MSB1_CONTROL.STALE], 64);
    assert.equal(c[MSB1_CONTROL.DRAIN_BLOCKS], 0);
    assert.equal(c[MSB1_CONTROL.UNDERRUNS], 0);
    assert.equal(c[MSB1_CONTROL.SEEKS_APPLIED], 1);
    const target = [Float32Array.from({ length: 128 }, (_, i) => (i + 1) / 256), Float32Array.from({ length: 128 }, (_, i) => -(i + 1) / 512)];
    for (let index = 0; index < 4; index++) {
      const planes = writer.reserve(128); planes[0].set(target[0]); planes[1].set(target[1]);
      writer.commit({ generation: 2n, startFrame: BigInt(10_000 + index * 128), frames: 128, endOfRegion: false });
    }
    await feed.prepareSeek(); // Same proof retains all fresh queued work.
    assert.equal(writer.occupancy, 4);
    engine.process([], []);
    assert.deepEqual(first.left, target[0]); assert.deepEqual(first.right, target[1]);
    assert.equal(c[MSB1_CONTROL.UNDERRUNS], 0);
    assert.equal(c[MSB1_CONTROL.SUBMITTED_GENERATION_TAG], 2);
    assert.equal(c[MSB1_CONTROL.SEEKS_APPLIED], 1);
  } finally { feed.close(); boundary.dispose(); }
});

test("prepareSeek bounds requests and rejects supersession, refusal, close and timeout", async () => {
  const requests = [];
  const port = { onmessage: null, postMessage(message) {
    if (message.op === "attach") for (const ring of message.rings) Atomics.store(controls(ring), MSB1_CONTROL.ATTACHED, 1);
    else if (message.op === "prepare-seek") requests.push(message);
  } };
  const suspended = { ...context, state: "suspended" };
  const feed = attachEngineFeed({ context: suspended, sources: [{ sourceId: "s", channels: 1 }], quantumFrames: 4, createNode: () => ({ port, disconnect() {} }) });
  const writer = new Msb1RingWriter(feed.rings[0]); writer.engage(1n); writer.seek(2n, 4n);
  const reply = (message, changes = {}) => port.onmessage?.({ data: { op: "seek-prepared", requestId: message.requestId, kind: "confirmed", seeks: message.seeks, ...changes } });
  const first = feed.prepareSeek();
  await assert.rejects(feed.prepareSeek(), (error) => error.operation === "prepareBusy");
  assert.equal(requests.length, 1);
  writer.seek(0x1_0000_0002n, 8n); // Same low-word tag still supersedes the full identity.
  reply(requests[0]);
  await assert.rejects(first, (error) => error.operation === "prepareSuperseded");
  const refused = feed.prepareSeek(); reply(requests[1], { kind: "refused", result: 6 });
  await assert.rejects(refused, (error) => error instanceof PcmFeedError && error.operation === "prepareRefused" && error.result === 6);
  const pending = feed.prepareSeek(); feed.close();
  await assert.rejects(pending, (error) => error.operation === "closed");
  reply(requests[2]);
  assert.equal(feed.state, "closed");
  const timed = attachEngineFeed({ context: suspended, sources: [{ sourceId: "s", channels: 1 }], quantumFrames: 4, createNode: () => ({ port, disconnect() {} }) });
  new Msb1RingWriter(timed.rings[0]).seek(2n, 4n);
  await assert.rejects(timed.prepareSeek({ timeoutMs: 1 }), (error) => error.operation === "prepareTimeout");
  assert.equal(timed.state, "closed");
});

test("actual Wasm: a ring seek is consumed only when the engine refuses it as already at the session seek's generation", async () => {
  // Issue #1644 gate 1 (R4 amended). After a session seek at 2 a moved source refuses its own seek
  // at 2 as stale, and without the rule that ring never drains again. A source the session seek
  // did not move (one a later plan added, still at 1) must still be seeked, or every chunk at 2 is
  // refused for good; any other refusal at that generation is counted as before.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const document = new TextEncoder().encode(sessionDocument({ frames: 4_800 }));
  const feedOver = async ({ sessionSeek }) => {
    const instance = await asset.instantiate();
    let handle;
    const exports = { ...instance.exports, miso_engine_web_v1_boot(...args) { handle = instance.exports.miso_engine_web_v1_boot(...args); return handle; } };
    const boundary = await WasmBoundary.boot({ instantiate: async () => ({ exports }) }, document, { sourceRingFrames: 512 });
    if (sessionSeek) assert.equal(boundary.sessionSeek({ generation: 2n, frame: 256n }).ok, true);
    const { sandbox, registrations } = preludeHarness(source);
    class Engine {
      constructor() {
        this.quantumFrames = 128; this.maximumSourceChannels = 2;
        this.exports = recordFeedCalls(exports); this.handle = handle; this.memoryBuffer = exports.memory.buffer;
        this.sourceIdPointer = exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", "sourceId"));
        this.sourceIdCapacity = exports.miso_engine_web_v1_buffer_capacity(handle, constantValue("bufferKinds", "sourceId"));
        this.sourcePcm = new Float32Array(this.memoryBuffer, exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", "sourcePcm")), 256);
        // The engine worklet's boot binds this view (the hermetic suite pins it).
        this.diagnosticView = new Uint8Array(this.memoryBuffer, exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", "diagnostic")), exports.miso_engine_web_v1_buffer_capacity(handle, constantValue("bufferKinds", "diagnostic")));
        this.ready = true; this.disposed = false; this.stickyResult = 0;
        this.lastSessionSeekGeneration = 2n; // what the engine handler records after an OK
      }
      rebindMemoryViews() { this.memoryBuffer = this.exports.memory.buffer; }
      process() { return true; }
    }
    sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
    const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
    const attach = new (registrations.get("miso-sab-feed-attach"))();
    const ring = createMsb1Ring({ sourceId: "s", channels: 2, frameCapacity: 128, capacity: 4 });
    attach.port.onmessage({ data: { op: "attach", rings: [ring] } });
    const writer = new Msb1RingWriter(ring); writer.engage(1n);
    const seekAndWrite = (frame) => {
      writer.seek(2n, frame);
      writer.reserve(128);
      writer.commit({ generation: 2n, startFrame: frame, frames: 128, endOfRegion: false });
      engine.process([], []);
      return controls(ring);
    };
    return { boundary, seekAndWrite, exports, handle };
  };
  const counts = (c) => [c[MSB1_CONTROL.SEEKS_APPLIED], c[MSB1_CONTROL.REFUSED], c[MSB1_CONTROL.SUBMITTED]];

  // Issue #1387 (MINOR 2): the code both consumers' fakes write is the one the built engine writes.
  const probe = await feedOver({ sessionSeek: true });
  try {
    const { exports, handle } = probe;
    const staged = (kind) => exports.miso_engine_web_v1_buffer_ptr(handle, constantValue("bufferKinds", kind));
    new Uint8Array(exports.memory.buffer, staged("sourceId"), 1).set([0x73]); // "s"
    assert.equal(exports.miso_engine_web_v1_source_seek(handle, 1, 2n, 0n), 1, "a seek at the session seek's generation is refused");
    const code = new TextDecoder().decode(new Uint8Array(exports.memory.buffer, staged("diagnostic"), STALE_GENERATION_CODE.length).slice());
    assert.equal(code, STALE_GENERATION_CODE, "the engine's refusal code is the consumers'");
  } finally { probe.boundary.dispose(); }
  const moved = await feedOver({ sessionSeek: true });
  try {
    assert.deepEqual(counts(moved.seekAndWrite(256n)), [1, 0, 1], "the moved source's stale refusal is the seek applied, not a refusal");
  } finally { moved.boundary.dispose(); }
  const fresh = await feedOver({ sessionSeek: false });
  try {
    assert.deepEqual(counts(fresh.seekAndWrite(0n)), [1, 0, 1], "a source the session seek did not move is seeked, so its PCM at 2 is accepted");
  } finally { fresh.boundary.dispose(); }
  const outside = await feedOver({ sessionSeek: false });
  try {
    const c = outside.seekAndWrite(1_000_000n);
    assert.deepEqual(counts(c), [0, 1, 0], "another refusal at the session generation is counted, and the epoch stays unseen");
    assert.equal(c[MSB1_CONTROL.LAST_RESULT], 1);
  } finally { outside.boundary.dispose(); }
});

test("the pump seeks every ring under the session's generation and refuses one not above its own", async () => {
  // Issue #1644 gate 4: a pump that writes its own next generation after a session seek feeds a
  // generation the engine never applied.
  const { CanonicalPcmPump } = await import("../../hosts/host-web/web/stem-store/pcm-pump.js");
  const ringSeeks = [];
  const ring = (channels) => ({ channels, frameCapacity: 4, engage() {}, seek(generation, frame) { ringSeeks.push([generation, frame]); }, release() {} });
  const pump = new CanonicalPcmPump({
    lease: { read: async () => new Blob([]) },
    windowFrames: 4,
    sources: [
      { sourceId: "short", identity: "a", channels: 1, bitDepth: 16, frames: 10, ring: ring(1) },
      { sourceId: "long", identity: "b", channels: 2, bitDepth: 16, frames: 100, ring: ring(2) },
    ],
  });
  await pump.seek(50, 5n);
  assert.deepEqual(ringSeeks.splice(0), [[5n, 10n], [5n, 50n]], "each stem at its clamped frame under the given generation");
  await assert.rejects(pump.seek(0, 5n), RangeError);
  assert.deepEqual(ringSeeks, [], "a generation not above the pump's own writes nothing");
  await pump.seek(0);
  assert.deepEqual(ringSeeks, [[6n, 0n], [6n, 0n]], "without one the pump continues from the session's");
});

function runPrelude(source, { tracking = false, mutate = false, capacity = 2, initial = true } = {}) {
  const mutated = mutate ? source.replace(
    "const staging = this.sourcePcm",
    "if (control[CONTROL_WROTE] > 1) new Float32Array(4); const staging = this.sourcePcm",
  ) : source;
  const { sandbox, registrations, allocations, arm } = preludeHarness(mutated, { tracking });
  const submissions = []; const seeks = []; let submitResult = 0; let seekResult = 0;
  class Engine {
    constructor() {
      this.quantumFrames = 4; this.maximumSourceChannels = 2; this.memoryBuffer = new ArrayBuffer(65_536);
      this.sourceIdPointer = 0; this.sourceIdCapacity = 128; this.sourcePcm = new sandbox.Float32Array(this.memoryBuffer, 1024, 8);
      this.handle = 1; this.ready = true; this.disposed = false; this.stickyResult = 0;
      this.exports = recordFeedCalls({ memory: { buffer: this.memoryBuffer }, miso_engine_web_v1_source_seek: (_h, _id, generation, frame) => { seeks.push([generation, frame]); return seekResult; }, miso_engine_web_v1_source_submit: (_h, _id, generation, start, channels, frames, end) => { submissions.push({ generation, start, channels, frames, end, pcm: [...this.sourcePcm] }); return typeof submitResult === "function" ? submitResult() : submitResult; } });
    }
    rebindMemoryViews() { this.memoryBuffer = this.exports.memory.buffer; }
    process() { return true; }
  }
  sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
  const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
  const attach = new (registrations.get("miso-sab-feed-attach"))();
  const rings = [1, 2, 1].map((channels, index) => createMsb1Ring({ sourceId: `source-${index}`, channels, frameCapacity: 4, capacity }));
  const ringControls = rings.map(controls);
  attach.port.onmessage({ data: { op: "attach", rings } });
  const writers = rings.map((ring) => new Msb1RingWriter(ring));
  for (const [index, writer] of writers.entries()) {
    writer.engage(1n);
    if (!initial) continue;
    const planes = writer.reserve(3);
    planes[0].set([index + 1, 2, 3]);
    if (writer.channels === 2) planes[1].set([4, 5, 6]);
    writer.commit({ generation: 1n, startFrame: 0n, frames: 3, endOfRegion: true });
  }
  const process = () => engine.process([], []);
  if (tracking) arm();
  return { sandbox, allocations, engine, attach, rings, ringControls, writers, submissions, seeks, process, setSubmitResult: (value) => { submitResult = value; }, setSeekResult: (value) => { seekResult = value; } };
}

/**
 * Issue #1297b: the feed prelude over a stand-in engine processor that has the engine handler's
 * `receiveSeekAt` shape (validate, call `source_seek_at` once, `acknowledge` the result). The
 * stand-in calls its own recorder, never `exports`, so a `source_seek_at` the drain makes is the
 * only one the recording proxy (and the last test) can see. `node` is an SDK feed node wired to
 * the attach processor's port in both directions.
 */
function anchoredPrelude(source) {
  const { sandbox, registrations } = preludeHarness(source);
  const submissions = []; const seeks = []; const seekAts = []; let seekAtResult = 0;
  class Engine {
    constructor() {
      this.quantumFrames = 4; this.maximumSourceChannels = 2; this.memoryBuffer = new ArrayBuffer(65_536);
      this.sourceIdPointer = 0; this.sourceIdCapacity = 128; this.sourcePcm = new sandbox.Float32Array(this.memoryBuffer, 1024, 8);
      this.handle = 1; this.ready = true; this.disposed = false; this.stickyResult = 0;
      this.exports = recordFeedCalls({ memory: { buffer: this.memoryBuffer }, miso_engine_web_v1_source_seek: (_h, _id, generation, frame) => { seeks.push([generation, frame]); return 0; }, miso_engine_web_v1_source_submit: (_h, idLength, generation, start, channels, frames) => { submissions.push({ id: new TextDecoder().decode(new Uint8Array(this.memoryBuffer, 0, idLength)), generation, start, channels, frames }); return 0; } });
    }
    receiveSeekAt(message) {
      seekAts.push([message.sourceId, message.generation, message.sourceFrame, message.anchorSample]);
      this.acknowledge(message.requestId, seekAtResult);
    }
    acknowledge(requestId, result) { this.lastAck = { requestId, result }; }
    rebindMemoryViews() { this.memoryBuffer = this.exports.memory.buffer; }
    process() { return true; }
  }
  sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
  const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
  const attach = new (registrations.get("miso-sab-feed-attach"))();
  // Each direction clones its message as a browser port does: a SharedArrayBuffer arrives as a new
  // object over the same memory, so the worklet can never find a ring by the SDK's object identity.
  const node = { port: { onmessage: null, postMessage: (data) => attach.port.onmessage({ data: structuredClone(data) }) }, disconnect() {} };
  attach.port.postMessage = (data) => node.port.onmessage?.({ data: structuredClone(data) });
  return { engine, attach, node, submissions, seeks, seekAts, process: () => engine.process([], []), setSeekAtResult: (value) => { seekAtResult = value; } };
}

/**
 * Issue #1387 (MAJOR-2): the shared MSB1 consumer case table (`msb1-consumer-cases.mjs`) over the
 * prelude: a stand-in engine with the engine handler's shapes, its exports recorded, and the
 * attach node's port cloning each message as a browser port does. `pass()` is one `process()`.
 */
function preludeConsumer(source) {
  const { sandbox, registrations } = preludeHarness(source);
  const adapter = {
    submissions: [], seeks: [], submitResult: 0, seekResult: 0, seekAtResult: 0, seekStale: false, onSeek: null, onApply: null, onSnapshot: null,
  };
  const stale = new TextEncoder().encode(STALE_GENERATION_CODE);
  class Engine {
    constructor() {
      this.quantumFrames = 4; this.maximumSourceChannels = 2; this.memoryBuffer = new ArrayBuffer(65_536);
      this.sourceIdPointer = 0; this.sourceIdCapacity = 128; this.sourcePcm = new sandbox.Float32Array(this.memoryBuffer, 1024, 8);
      this.diagnosticView = new sandbox.Uint8Array(this.memoryBuffer, 4096, 256);
      this.sampleRateHz = 48_000;
      this.handle = 1; this.ready = true; this.disposed = false; this.stickyResult = 0;
      this.exports = recordFeedCalls({
        memory: { buffer: this.memoryBuffer },
        miso_engine_web_v1_source_seek: (_h, _id, generation, frame) => {
          adapter.seeks.push([generation, frame]);
          adapter.onSeek?.(generation, frame);
          this.diagnosticView.fill(0);
          if (adapter.seekResult !== 0 && adapter.seekStale) this.diagnosticView.set(stale);
          return adapter.seekResult;
        },
        miso_engine_web_v1_source_submit: (_h, idLength, generation, start, channels, frames, end) => {
          adapter.submissions.push({ id: new TextDecoder().decode(new Uint8Array(this.memoryBuffer, 0, idLength)), generation, start, channels, frames, end, pcm: [...this.sourcePcm] });
          return adapter.submitResult;
        },
      });
    }
    receiveSeekAt(message) { this.acknowledge(message.requestId, adapter.seekAtResult); }
    acknowledge(requestId, result) { this.lastAck = { requestId, result }; }
    rebindMemoryViews() { this.memoryBuffer = this.exports.memory.buffer; }
    process() { return true; }
  }
  sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
  const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
  const snapshot = engine.readSharedSeek.bind(engine);
  engine.readSharedSeek = (ring) => {
    const read = snapshot(ring);
    adapter.onSnapshot?.();
    return read;
  };
  const apply = engine.applySharedSeek.bind(engine);
  engine.applySharedSeek = (ring, ...identity) => {
    if (identity.length !== 0) adapter.onApply?.();
    return apply(ring, ...identity);
  };
  const attach = new (registrations.get("miso-sab-feed-attach"))();
  const replies = [];
  attach.port.postMessage = (data) => replies.push(structuredClone(data));
  const send = (data) => { attach.port.onmessage({ data: structuredClone(data) }); return replies.pop(); };
  let requestId = 0;
  Object.defineProperty(adapter, "seekBound", { get: () => engine.seekDeferralBlocks });
  return Object.assign(adapter, {
    attach: async (rings) => { send({ op: "attach", rings }); },
    pass: async () => { engine.process([], []); },
    seekAt: async (message) => { engine.receiveSeekAt({ tag: "miso.seek-at.v1", requestId: (requestId += 1), ...message }); },
    sessionSeek: async (generation) => { engine.lastSessionSeekGeneration = generation; },
    prepareSeek: async (seeks) => send({ op: "prepare-seek", requestId: (requestId += 1), seeks }),
    detachSource: async (index) => send({ op: "detach-source", requestId: (requestId += 1), index }),
  });
}

for (const { name, run } of MSB1_CONSUMER_CASES) {
  test(`MSB1 consumer (worklet prelude): ${name}`, async () => {
    const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
    await run(preludeConsumer(source));
  });
}

test("issue 1387: in worker mode the prelude binds no ring, process() submits nothing, and the feed is refused at once", async () => {
  // Red if the prelude binds rings in worker mode (process() would then call source_submit on a
  // render-only instance), or if the refusal does not reach the feed, which would wait out its
  // ready() bound instead.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = anchoredPrelude(source);
  run.engine.workerMode = true;
  const replies = [];
  run.attach.port.postMessage = (data) => replies.push(data);
  const ring = createMsb1Ring({ sourceId: "worker", channels: 1, frameCapacity: 4, capacity: 2 });
  const writer = new Msb1RingWriter(ring);
  writer.engage(1n);
  writer.reserve(4);
  writer.commit({ generation: 1n, startFrame: 0n, frames: 4, endOfRegion: false });
  run.attach.port.onmessage({ data: { op: "attach", rings: [ring] } });
  run.process();
  assert.deepEqual(run.submissions, [], "process() submits nothing in worker mode");
  assert.equal(controls(ring)[MSB1_CONTROL.ATTACHED], 0, "the ring is not bound");
  assert.deepEqual(replies.map((reply) => reply.op), ["attach-refused"]);
  // Rings that arrive before the engine processor exists are refused when it is built.
  const early = preludeHarness(source);
  const earlyAttach = new (early.registrations.get("miso-sab-feed-attach"))();
  const earlyReplies = [];
  earlyAttach.port.postMessage = (data) => earlyReplies.push(data.op);
  earlyAttach.port.onmessage({ data: { op: "attach", rings: [createMsb1Ring({ sourceId: "early", channels: 1, frameCapacity: 4, capacity: 2 })] } });
  assert.deepEqual(earlyReplies, []);
  early.sandbox.registerProcessor("miso-engine-v1-audio-worklet", class { constructor() { this.workerMode = true; } rebindMemoryViews() {} });
  new (early.registrations.get("miso-engine-v1-audio-worklet"))();
  assert.deepEqual(earlyReplies, ["attach-refused"], "the pending rings are refused too");
  const refusing = { port: { onmessage: null, postMessage(data) { if (data.op === "attach") queueMicrotask(() => this.onmessage?.({ data: { op: "attach-refused" } })); } }, disconnect() {} };
  const feed = attachEngineFeed({ context, quantumFrames: 4, sources: [{ sourceId: "worker", channels: 1 }], createNode: () => refusing });
  await assert.rejects(feed.ready({ timeoutMs: 60_000 }), (error) => error instanceof PcmFeedError && error.operation === "attachRefused");
  assert.equal(feed.state, "closed");
});

test("issue 1387: a feed of a worker-mode engine attaches through the host's feed port, a single-mode one through the attach node", async () => {
  // Red if the feed ignores the engine's instanceMode in either direction.
  const feedFor = (instanceMode) => {
    const posted = { port: [], node: [] };
    const host = {
      instanceMode,
      openFeedPort: () => ({ onmessage: null, postMessage: (data) => posted.port.push(data.op) }),
      async status() { return { result: 0, nextAbsoluteSample: 0n }; },
      async seekSourceAt() { return { result: 0 }; },
      async seekReport() { return { result: 0, report: null }; },
    };
    linkFeedEngine(host, () => 1n);
    const feed = attachEngineFeed({
      engine: { host }, context, quantumFrames: 4, sources: [{ sourceId: "routed", channels: 1 }],
      createNode: () => attachNode({ onMessage: (data) => posted.node.push(data.op) }),
    });
    feed.close();
    return posted;
  };
  assert.deepEqual(feedFor("worker"), { port: ["attach", "detach"], node: [] });
  assert.deepEqual(feedFor("single"), { port: [], node: ["attach", "detach"] });
});

test("an anchored start is applied once by the engine handler, and the drain reads none of its ring before", async () => {
  // #1297b gates 1 and 2. Red if the SDK starts the stem with a plain seek or lets the drain apply
  // it (`source_seek_at` from process() runs the service step), if the drain feeds the ring's
  // PCM before the handler applied its seek, if the SDK recomputes the anchor or the frame after
  // the one status read, or if `A` is not next + the ring's frames, rounded up to a quantum.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = anchoredPrelude(source);
  let next = 1_001n; let statusReads = 0; let requestId = 0; let generation = 4n;
  const host = {
    async status() { statusReads += 1; const now = next; next += 512n; return { result: 0, nextAbsoluteSample: now }; },
    async seekSourceAt(request) {
      run.engine.receiveSeekAt({ tag: "miso.seek-at.v1", requestId: (requestId += 1), ...request });
      return { tag: "miso.ack.v1", requestId, result: run.engine.lastAck.result };
    },
  };
  linkFeedEngine(host, () => (generation += 1n));
  const asked = []; const primed = [];
  const start = {
    frameAt(anchorSample) { asked.push(anchorSample); return anchorSample - 1_000n + 5n; },
    prime(seek) {
      primed.push(seek);
      const writer = new Msb1RingWriter(feed.rings[0]);
      for (let chunk = 0; chunk < 2; chunk += 1) {
        writer.reserve(4)[0].fill(chunk + 1);
        writer.commit({ generation: seek.generation, startFrame: seek.frame + BigInt(chunk * 4), frames: 4, endOfRegion: false });
      }
      run.process(); // a block while the seek is published in the ring but not yet applied
      assert.deepEqual([run.submissions.length, run.seeks.length, controls(feed.rings[0])[MSB1_CONTROL.SEEKS_APPLIED]], [0, 0, 0],
        "the drain reads no slot and seeks nothing before the handler applied the anchored seek");
    },
  };
  const feed = attachEngineFeed({ context, sources: [{ sourceId: "added", channels: 1, start }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
  new Msb1RingWriter(feed.rings[0]).engage(1n);
  await feed.ready();
  // A = 1001 + 2 chunks * 4 frames = 1009, rounded up to 1012; the frame is the app's for it.
  assert.deepEqual([statusReads, asked], [1, [1_012n]]);
  assert.deepEqual(primed, [{ sourceId: "added", generation: 5n, frame: 17n, anchorSample: 1_012n }]);
  assert.deepEqual(run.seekAts, [["added", 5n, 17n, 1_012n]], "the handler applied the seek once");
  assert.deepEqual([...new BigInt64Array(feed.rings[0], MSB1_CONTROL_I64_OFFSET, 3)], [5n, 17n, 1_012n], "the ring holds the same seek");
  run.process();
  run.process();
  assert.deepEqual(run.seeks, [], "never a plain seek for the started source");
  assert.deepEqual(run.submissions.map(({ id, generation: g, start: frame }) => [id, g, frame]), [["added", 5n, 17n], ["added", 5n, 21n]]);
  assert.equal(controls(feed.rings[0])[MSB1_CONTROL.SEEKS_APPLIED], 1);

  // Acked batch: a refused `seek-at` rejects the start with its result, and its epoch is never consumed.
  run.setSeekAtResult(1);
  const refused = attachEngineFeed({ context, sources: [{ sourceId: "refused", channels: 1, start: { frameAt: () => 0n, prime() {} } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
  await assert.rejects(refused.ready(), (error) => error instanceof MisoEngineError && error.result === 1);
  assert.equal(refused.state, "closed");
});

/**
 * Issue 1297c: an engine host whose seek reports for `alignTo` come from `replies` (a reply may be
 * a function that returns one), cloned as a port does. `next` is the status's next sample, or a
 * function that returns it (or a promise of it).
 */
function alignHost(run, next, replies) {
  let requestId = 0;
  const reads = [];
  const host = {
    async status() { return { result: 0, nextAbsoluteSample: typeof next === "function" ? await next() : next }; },
    async seekSourceAt(request) {
      run.engine.receiveSeekAt({ tag: "miso.seek-at.v1", requestId: (requestId += 1), ...request });
      return structuredClone({ tag: "miso.ack.v1", requestId, result: run.engine.lastAck.result });
    },
    async seekReport(sourceId) {
      reads.push(sourceId);
      const reply = replies.shift();
      const { result, report = null } = typeof reply === "function" ? reply() : reply;
      return structuredClone({ tag: "miso.seek-report.v1", requestId: (requestId += 1), result, report });
    },
  };
  let generation = 4n;
  linkFeedEngine(host, () => (generation += 1n));
  return { host, reads };
}
const referenceReport = { generation: 3n, firstSample: 1_024n, sourceFrame: 48_000n, cumulativeUnderrunFrames: 7n, cumulativeHeldFrames: 2n };

test("alignTo starts the stem at the reference's frame at the anchor, read from its seek report", async () => {
  // #1297c gate 1a. Red if the SDK adds A to the frame without subtracting first_sample, reads the
  // frame from anything but the reference's report, gives up on the first busy read, or refuses a
  // reference that underran (its frame stays exact).
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = anchoredPrelude(source);
  // A = 4088 + 2 chunks * 4 frames = 4096, a quantum multiple.
  // The consumer advances its frame on an underrun, so an underrun between the reads keeps the frame exact.
  const { host, reads } = alignHost(run, 4_088n, [{ result: 6 }, { result: 0, report: referenceReport }, { result: 0, report: { ...referenceReport, cumulativeUnderrunFrames: 11n } }]);
  const primed = [];
  const feed = attachEngineFeed({ context, sources: [{ sourceId: "added", channels: 1, start: { alignTo: "X", prime: (seek) => { primed.push(seek); } } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
  await feed.ready();
  assert.deepEqual(reads, ["X", "X", "X"], "one busy read is retried, then the report is confirmed once");
  assert.deepEqual(primed, [{ sourceId: "added", generation: 5n, frame: 51_072n, anchorSample: 4_096n }]);
  assert.deepEqual([...new BigInt64Array(feed.rings[0], MSB1_CONTROL_I64_OFFSET, 3)], [5n, 51_072n, 4_096n]);
  assert.deepEqual(run.seekAts, [["added", 5n, 51_072n, 4_096n]]);
});

test("alignTo refuses an unrendered, busy, reseeked, held or pending-seek reference and publishes nothing", async () => {
  // #1297c gate 1a. Red if the SDK aligns to a reference that has not rendered (generation 0),
  // retries a busy report without bound or not at all, aligns across a seek or held frames the
  // reference counted since the report it computed from, or aligns to a fed reference whose ring
  // publishes a seek the engine has not rendered (the report is the old generation's).
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const ok = { result: 0, report: referenceReport };
  const cases = [
    ["feed.align.unrendered", [{ result: 0, report: { ...referenceReport, generation: 0n } }], 1],
    ["feed.align.busy", [{ result: 6 }, { result: 6 }, { result: 6 }, { result: 6 }, ok, ok], 4],
    ["feed.align.reseeked", [ok, { result: 0, report: { ...referenceReport, generation: 4n } }], 2],
    ["feed.align.held", [ok, { result: 0, report: { ...referenceReport, cumulativeHeldFrames: 6n } }], 2],
  ];
  for (const [code, replies, readCount] of cases) {
    const run = anchoredPrelude(source);
    const { host, reads } = alignHost(run, 4_088n, replies);
    const primed = [];
    const feed = attachEngineFeed({ context, sources: [{ sourceId: "added", channels: 1, start: { alignTo: "X", prime: (seek) => { primed.push(seek); } } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
    await assert.rejects(feed.ready(), (error) => error instanceof MisoUsageError && error.diagnosticCode === code, code);
    assert.deepEqual([reads.length, primed, run.seekAts, controls(feed.rings[0])[MSB1_CONTROL.SEEK_EPOCH], feed.state], [readCount, [], [], 0, "closed"], code);
  }
  {
    // A fed reference with a plain ring seek above its report's generation.
    const run = anchoredPrelude(source);
    const { host, reads } = alignHost(run, 4_088n, [ok, ok]);
    const reference = attachEngineFeed({ context, sources: [{ sourceId: "X", channels: 1 }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
    new Msb1RingWriter(reference.rings[0]).seek(referenceReport.generation + 1n, 0n);
    const feed = attachEngineFeed({ context, sources: [{ sourceId: "added", channels: 1, start: { alignTo: "X", prime() {} } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
    await assert.rejects(feed.ready(), (error) => error instanceof MisoUsageError && error.diagnosticCode === "feed.align.held");
    assert.deepEqual([reads.length, run.seekAts, controls(feed.rings[0])[MSB1_CONTROL.SEEK_EPOCH]], [1, [], 0]);
  }
  const run = anchoredPrelude(source);
  const { host } = alignHost(run, 0n, []);
  assert.throws(() => attachEngineFeed({ context, sources: [{ sourceId: "both", channels: 1, start: { alignTo: "X", frameAt: () => 0n, prime() {} } }], quantumFrames: 4, engine: { host }, createNode: () => run.node }), MisoUsageError);
  // A self-alignment and an alignTo cycle within one feed are refused at attach.
  for (const sources of [
    [{ sourceId: "self", channels: 1, start: { alignTo: "self", prime() {} } }],
    [{ sourceId: "a", channels: 1, start: { alignTo: "c", prime() {} } }, { sourceId: "b", channels: 1, start: { alignTo: "a", prime() {} } }, { sourceId: "c", channels: 1, start: { alignTo: "b", prime() {} } }],
  ]) {
    assert.throws(() => attachEngineFeed({ context, sources, quantumFrames: 4, engine: { host }, createNode: () => run.node }), (error) => error instanceof MisoUsageError && /alignTo cycle/.test(error.message), sources[0].sourceId);
  }
});

test("alignTo a source whose own anchored start is not rendered yet takes that start's frame", async () => {
  // #1297c (several stems added together). Red if the SDK aligns to the reference's report while
  // the reference's own start is pending (the report still shows generation 1 at frame 0), or
  // starts a feed's sources in list order when an aligned source is listed before its reference.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const x = { sourceId: "X", channels: 1, start: { frameAt: (anchor) => anchor + 50_000n, prime() {} } };
  const y = { sourceId: "Y", channels: 1, start: { alignTo: "X", prime() {} } };
  for (const sources of [[x, y], [y, x]]) {
    const run = anchoredPrelude(source);
    const initial = { result: 0, report: { generation: 1n, firstSample: 0n, sourceFrame: 0n, cumulativeUnderrunFrames: 0n, cumulativeHeldFrames: 0n } };
    let next = 1_000n;
    const { host } = alignHost(run, () => { const now = next; next += 100n; return now; }, [initial, initial]);
    const feed = attachEngineFeed({ context, sources, quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
    await feed.ready();
    // A_X = 1000 + 8 = 1008, F_X = 51008; A_Y = 1100 + 8 = 1108, so F_Y = 51008 + 100.
    assert.deepEqual(run.seekAts, [["X", 5n, 51_008n, 1_008n], ["Y", 6n, 51_108n, 1_108n]], sources[0].sourceId);
    assert.deepEqual([...new BigInt64Array(feed.rings[sources.indexOf(y)], MSB1_CONTROL_I64_OFFSET, 3)], [6n, 51_108n, 1_108n]);
  }
});

test("alignTo refuses a reference another feed has not started yet, and one restarted while the anchor was computed", async () => {
  // #1297c. Red if Y aligns to a reference whose start a linked feed has not published (Y would
  // take the reference's old report), keeps a start record that a newer start of the reference
  // replaced while Y's anchor was computed, or still counts a detached reference's start as pending.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const initial = { result: 0, report: { generation: 1n, firstSample: 0n, sourceFrame: 0n, cumulativeUnderrunFrames: 0n, cumulativeHeldFrames: 0n } };
  const options = (run, host, sources) => ({ context, sources, quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
  {
    const run = anchoredPrelude(source);
    const { host } = alignHost(run, 1_000n, [initial]);
    const reference = attachEngineFeed(options(run, host, [{ sourceId: "X", channels: 1, start: { frameAt: () => 7n, prime() {} } }]));
    const feed = attachEngineFeed(options(run, host, [{ sourceId: "Y", channels: 1, start: { alignTo: "X", prime() {} } }]));
    await assert.rejects(feed.ready(), (error) => error instanceof MisoUsageError && error.diagnosticCode === "feed.align.unstarted");
    assert.deepEqual([run.seekAts, controls(feed.rings[0])[MSB1_CONTROL.SEEK_EPOCH]], [[], 0]);
    await reference.ready();
    assert.deepEqual(run.seekAts.map(([id]) => id), ["X"], "the reference still starts");
  }
  {
    // A reference detached before its start ran is no longer pending: Y reads its report.
    const run = anchoredPrelude(source);
    const { host } = alignHost(run, 1_000n, [{ result: 0, report: { ...initial.report, generation: 0n } }]);
    const reference = attachEngineFeed(options(run, host, [{ sourceId: "X", channels: 1, start: { frameAt: () => 7n, prime() {} } }]));
    await reference.detachSource("X");
    const feed = attachEngineFeed(options(run, host, [{ sourceId: "Y", channels: 1, start: { alignTo: "X", prime() {} } }]));
    await assert.rejects(feed.ready(), (error) => error instanceof MisoUsageError && error.diagnosticCode === "feed.align.unrendered");
  }
  {
    const run = anchoredPrelude(source);
    let restart;
    const { host } = alignHost(run, async () => { const hook = restart; restart = undefined; await hook?.(); return 1_000n; }, [initial]);
    const first = attachEngineFeed(options(run, host, [{ sourceId: "X", channels: 1, start: { frameAt: () => 7n, prime() {} } }]));
    await first.ready();
    // During Y's one status read, another feed starts X again at a new generation and frame.
    restart = () => attachEngineFeed(options(run, host, [{ sourceId: "X", channels: 1, start: { frameAt: () => 900n, prime() {} } }])).ready();
    const feed = attachEngineFeed(options(run, host, [{ sourceId: "Y", channels: 1, start: { alignTo: "X", prime() {} } }]));
    await assert.rejects(feed.ready(), (error) => error instanceof MisoUsageError && error.diagnosticCode === "feed.align.reseeked");
    assert.deepEqual([run.seekAts.map(([id, , frame]) => [id, frame]), controls(feed.rings[0])[MSB1_CONTROL.SEEK_EPOCH]], [[["X", 7n], ["X", 900n]], 0]);
  }
});

test("two feeds that align to one reference at once both start: a locally refused read is retried after a macrotask", async () => {
  // #1297c. Red if the SDK retries a read the wrapper refused locally (one unsettled read per
  // source) without yielding, or does not retry it: the second feed then rejects busy.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = anchoredPrelude(source);
  const { host } = alignHost(run, 4_088n, []);
  const inFlight = new Set();
  host.seekReport = (sourceId) => {
    if (inFlight.has(sourceId)) return Promise.reject(Object.assign(new Error("in flight"), { result: 6 }));
    inFlight.add(sourceId);
    return new Promise((resolve) => setTimeout(() => {
      inFlight.delete(sourceId);
      resolve(structuredClone({ tag: "miso.seek-report.v1", requestId: 0, result: 0, report: referenceReport }));
    }, 0));
  };
  const feeds = ["one", "two"].map((sourceId) => attachEngineFeed({ context, sources: [{ sourceId, channels: 1, start: { alignTo: "X", prime() {} } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node }));
  await Promise.all(feeds.map((feed) => feed.ready()));
  assert.deepEqual(run.seekAts.map(([id, , frame]) => [id, frame]).sort(), [["one", 51_072n], ["two", 51_072n]]);
});

test("alignTo publishes nothing for a feed closed or a source detached while the reports were read", async () => {
  // #1297c. Red if a start whose feed closed, or whose source was detached, during its report
  // reads still reserves a generation, publishes the ring seek or primes.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const ok = { result: 0, report: referenceReport };
  for (const action of ["close", "detach"]) {
    const run = anchoredPrelude(source);
    let feed;
    const { host } = alignHost(run, 4_088n, [ok, () => { if (action === "close") feed.close(); else void feed.detachSource("added"); return ok; }]);
    const primed = [];
    feed = attachEngineFeed({ context, sources: [{ sourceId: "added", channels: 1, start: { alignTo: "X", prime: (seek) => { primed.push(seek); } } }], quantumFrames: 4, capacityChunks: 2, engine: { host }, createNode: () => run.node });
    const ring = feed.rings[0];
    if (action === "close") await assert.rejects(feed.ready(), PcmFeedError); else await feed.ready();
    assert.deepEqual([primed, run.seekAts, controls(ring)[MSB1_CONTROL.SEEK_EPOCH]], [[], [], 0], action);
  }
});

test("the prelude over the real engine worklet class: one miso.seek-at.v1, and the drain consumes the epoch", async () => {
  // #1297b: red if the prelude's hook does not compose with the engine handler it wraps (a renamed
  // `receiveSeekAt` or `acknowledge`, a reply shape it does not read): the ring would then wait
  // forever on a seek the engine applied. The class is the shipped worklet's; its exports are stubs.
  const feedSource = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const engineSource = await readFile(new URL("../../hosts/host-web/web/miso-engine-v1-audio-worklet.js", import.meta.url), "utf8");
  const { sandbox, registrations } = preludeHarness(feedSource);
  // Each worklet module has its own scope in a browser; a script in one vm context shares one.
  vm.runInContext(`(() => {\n${engineSource}\n})();`, sandbox);
  const Wrapped = registrations.get("miso-engine-v1-audio-worklet");
  // Not constructed (that boots a module): the fields the handler and the drain read, set directly.
  const engine = Object.create(Wrapped.prototype);
  const memory = { buffer: new ArrayBuffer(65_536) };
  const seekAts = []; const submissions = []; const posts = [];
  Object.assign(engine, {
    exports: {
      memory,
      miso_engine_web_v1_source_seek_at: (...args) => { seekAts.push(args); return 0; },
      miso_engine_web_v1_source_submit: (_h, _id, generation, start) => { submissions.push([generation, start]); return 0; },
    },
    memoryBuffer: memory.buffer, handle: 7, sourceIdPointer: 0, sourceIdCapacity: 128,
    sourcePcm: new sandbox.Float32Array(memory.buffer, 1024, 4), quantumFrames: 4, maximumSourceChannels: 2,
    ready: true, disposed: false, stickyResult: 0, lastRequestId: 0, sabRings: [], pendingSeekAt: null,
    port: { postMessage: (message) => posts.push(message) },
  });
  const ring = createMsb1Ring({ sourceId: "café", channels: 1, frameCapacity: 4, capacity: 2 });
  engine.attachSharedRings([ring]);
  const writer = new Msb1RingWriter(ring); writer.engage(1n);
  writer.seek(3n, 40n, 1_024n);
  writer.reserve(4); writer.commit({ generation: 3n, startFrame: 40n, frames: 4, endOfRegion: false });
  engine.drainSharedRing(engine.sabRings[0]);
  assert.deepEqual(submissions, [], "nothing before the handler");
  engine.receive({ tag: "miso.seek-at.v1", requestId: 1, sourceId: "café", generation: 3n, sourceFrame: 40n, anchorSample: 1_024n });
  assert.deepEqual(seekAts, [[7, 5, 3n, 40n, 1_024n]]);
  assert.deepEqual(posts.map(({ tag, result }) => [tag, result]), [["miso.ack.v1", 0]]);
  engine.drainSharedRing(engine.sabRings[0]);
  assert.deepEqual(submissions, [[3n, 40n]]);
  assert.equal(controls(ring)[MSB1_CONTROL.SEEKS_APPLIED], 1);
});

test("detachSource withdraws one ring: the drain never reads it after it resolves, and the others keep draining", async () => {
  // #1297b gate 4 (the feed half; the engine's timing is browser-evals'). Red if detachSource
  // resolves before the worklet withdrew the ring, names the ring by an identity a cloned message
  // loses (the worklet then withdraws nothing and still confirms), compacts the attach list (a
  // later index then names another ring, or none), lets a withdrawn slot reach prepare-seek,
  // withdraws another node's or every ring, or leaves the ring's writer engaged.
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = anchoredPrelude(source);
  const suspended = { ...context, state: "suspended" };
  const feed = attachEngineFeed({ context: suspended, sources: ["first", "middle", "last"].map((sourceId) => ({ sourceId, channels: 1 })), quantumFrames: 4, capacityChunks: 4, createNode: () => run.node });
  const writers = feed.rings.map((ring) => new Msb1RingWriter(ring));
  const write = (writer, frame) => { writer.reserve(4); writer.commit({ generation: 1n, startFrame: frame, frames: 4, endOfRegion: false }); };
  for (const writer of writers) writer.engage(1n);
  await feed.ready();
  const [firstRing, middleRing, lastRing] = feed.rings;
  await feed.detachSource("first");
  await feed.detachSource("last");
  assert.deepEqual(feed.rings, [middleRing]);
  assert.deepEqual([firstRing, lastRing].map((ring) => controls(ring)[MSB1_CONTROL.WRITER_STATE]), [0, 0], "released after the confirmation");
  for (const writer of writers) write(writer, 0n);
  run.process();
  assert.deepEqual(run.submissions.map(({ id }) => id), ["middle"]);
  assert.deepEqual([firstRing, lastRing].map((ring) => controls(ring)[MSB1_CONTROL.READ_INDEX]), [0, 0], "the withdrawn rings are never read");
  // The withdrawn slots stay out of a later prepare-seek over the remaining ring.
  writers[1].seek(2n, 0n);
  await feed.prepareSeek();
  assert.deepEqual(run.seeks, [[2n, 0n]]);
  await assert.rejects(feed.detachSource("last"), /no source last/);
  // A worklet that withdrew nothing says so; the SDK rejects and closes rather than leak the ring.
  const unconfirmed = { port: { onmessage: null, postMessage: (data) => { if (data.op === "detach-source") unconfirmed.port.onmessage?.({ data: { op: "source-detached", requestId: data.requestId, detached: false } }); } }, disconnect() {} };
  const refused = attachEngineFeed({ context, sources: [{ sourceId: "lost", channels: 1 }], quantumFrames: 4, createNode: () => unconfirmed });
  await assert.rejects(refused.detachSource("lost"), (error) => error instanceof PcmFeedError && error.operation === "detachRefused");
  assert.equal(refused.state, "closed");
  // A source detached while ready() is starting an earlier one is never started.
  const asked = [];
  const linkedHost = { status: async () => ({ result: 0, nextAbsoluteSample: 0n }), seekSourceAt: async (request) => { asked.push(request.sourceId); return { result: 0 }; } };
  linkFeedEngine(linkedHost, () => 2n);
  const starting = attachEngineFeed({
    context, quantumFrames: 4, createNode: () => anchoredPrelude(source).node, engine: { host: linkedHost },
    sources: [
      { sourceId: "one", channels: 1, start: { frameAt: () => 0n, prime: () => starting.detachSource("two") } },
      { sourceId: "two", channels: 1, start: { frameAt: () => 0n, prime() {} } },
    ],
  });
  await starting.ready();
  assert.deepEqual(asked, ["one"]);
  // A prime that detaches its own source sends no seek-at for it.
  const self = attachEngineFeed({
    context, quantumFrames: 4, createNode: () => anchoredPrelude(source).node, engine: { host: linkedHost },
    sources: [{ sourceId: "self", channels: 1, start: { frameAt: () => 0n, prime: () => self.detachSource("self") } }],
  });
  await self.ready();
  assert.deepEqual(asked, ["one"]);
  // One ring per source ID: a second ring of one ID would be detached by its first's index.
  assert.throws(() => attachEngineFeed({ context, sources: [{ sourceId: "twice", channels: 1 }, { sourceId: "twice", channels: 2 }], quantumFrames: 4, createNode: () => attachNode() }), /unique: twice/);
});

test("the pump's writer publishes a plain seek in the SDK's ring layout, and both writers store it under an odd epoch", async (t) => {
  // #1297b hazard: a pump that keeps the old two-word seek leaves an earlier anchor in the ring,
  // and the drain then waits for a handler seek that never comes. #2184: a writer that stores the
  // tuple or the tag while the epoch is even lets a reader take a torn seek for a whole one.
  const { Msb1RingWriter: PumpWriter } = await import("../../hosts/host-web/web/stem-store/pcm-pump.js");
  const ring = createMsb1Ring({ sourceId: "stem", channels: 1, frameCapacity: 4, capacity: 2 });
  const control = controls(ring);
  const store = Atomics.store;
  const parities = [];
  t.mock.method(Atomics, "store", (array, index, value) => {
    if (array.buffer === ring) parities.push(Atomics.load(control, MSB1_CONTROL.SEEK_EPOCH) & 1);
    return store(array, index, value);
  });
  new Msb1RingWriter(ring).seek(2n, 0n, 4_096n);
  new PumpWriter(ring).seek(3n, 8n);
  t.mock.restoreAll();
  assert.deepEqual(parities, [1, 1, 1, 1, 1, 1, 1, 1], "the generation, frame, anchor and tag of each seek are stored at an odd epoch");
  assert.deepEqual([...new BigInt64Array(ring, MSB1_CONTROL_I64_OFFSET, 3)], [3n, 8n, -1n]);
  assert.equal(new TextDecoder().decode(new Uint8Array(ring, MSB1_ID_OFFSET, 4)), "stem");
});

test("full shared runways fill each internal queue gradually within the per-source callback budget", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = runPrelude(source, { capacity: 8, initial: false });
  const queued = [0, 0, 0]; const accepted = [0, 0, 0]; const positions = [0n, 0n, 0n];
  let pressure = 0;
  run.setSubmitResult(() => {
    const index = new Uint8Array(run.engine.memoryBuffer, 0, 8)[7] - 48;
    if (queued[index] === 8) { pressure++; return 6; }
    queued[index]++; accepted[index]++; return 0;
  });
  const refill = () => run.writers.forEach((writer, index) => {
    while (writer.occupancy < writer.capacity) {
      for (const plane of writer.reserve(4)) plane.fill(index + 0.25);
      writer.commit({ generation: 1n, startFrame: positions[index], frames: 4, endOfRegion: false });
      positions[index] += 4n;
    }
  });
  for (let block = 0; block < 12; block++) {
    refill(); const before = [...accepted]; run.process();
    assert.ok(accepted.every((count, index) => count - before[index] <= 2));
    assert.ok(queued.every((count) => count > 0), "each source supplies the upcoming render");
    for (let index = 0; index < queued.length; index++) queued[index]--;
    assert.deepEqual(run.ringControls.map((control) => control[MSB1_CONTROL.DEPTH]), queued);
  }
  assert.deepEqual(queued, [7, 7, 7], "the original internal capacity remains reachable");
  assert.ok(pressure > 0, "real capacity backpressure retains pending shared PCM");
  assert.ok(run.writers.every((writer) => writer.occupancy > 0));
  assert.ok(run.ringControls.every((control) => control[MSB1_CONTROL.UNDERRUNS] === 0 && control[MSB1_CONTROL.REFUSED] === 0));
});

test("a memory growth is re-bound between blocks: the rings' id views follow and the drain resumes", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = runPrelude(source);
  const memory = run.engine.exports.memory;
  const grown = new ArrayBuffer(memory.buffer.byteLength * 2);
  new Uint8Array(grown).set(new Uint8Array(memory.buffer));
  memory.buffer = grown; // a control call grew the module's memory
  run.process();
  assert.equal(run.submissions.length, 0, "a drain on a detached view submits nothing (and never re-binds in process())");
  assert.ok(run.engine.sabRings.every((ring) => ring.idTargetBuffer !== grown), "process() cut no view");
  run.engine.rebindMemoryViews(); // what the message handler does at its end
  assert.ok(run.engine.sabRings.every((ring) => ring.idTargetBuffer === grown && ring.idTarget.buffer === grown), "every ring's id view is re-cut against the grown memory");
  run.process();
  assert.equal(run.submissions.length, 3, "the drain resumes through the re-bound views");
});

test("moved prelude drains odd mono/stereo rings and allocation mutation turns red", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = runPrelude(source, { tracking: true });
  run.process();
  run.process();
  assert.deepEqual(run.submissions.map(({ channels, frames, end }) => ({ channels, frames, end })), [
    { channels: 1, frames: 3, end: 1 }, { channels: 2, frames: 3, end: 1 }, { channels: 1, frames: 3, end: 1 },
  ]);
  assert.deepEqual(run.submissions[1].pcm, [2, 2, 3, 0, 4, 5, 6, 0]);
  assert.ok(Object.is(run.submissions[1].pcm[3], 0));
  assert.ok(Object.is(run.submissions[1].pcm[7], 0));
  assert.deepEqual(run.submissions.map(({ generation, start, channels, frames, end }) => ({ generation, start, channels, frames, end })), [
    { generation: 1n, start: 0n, channels: 1, frames: 3, end: 1 },
    { generation: 1n, start: 0n, channels: 2, frames: 3, end: 1 },
    { generation: 1n, start: 0n, channels: 1, frames: 3, end: 1 },
  ]);
  assert.ok(run.ringControls.every((control) => control[MSB1_CONTROL.SUBMITTED] === 1));
  run.setSubmitResult(6); const writer = run.writers[0]; const plane = writer.reserve(4); plane[0].set([9, 8, 7, 6]); writer.commit({ generation: 1n, startFrame: 3n, frames: 4, endOfRegion: false });
  run.process();
  assert.equal(writer.occupancy, 1); assert.equal(run.ringControls[0][MSB1_CONTROL.REFUSED], 0);
  assert.deepEqual(run.submissions.at(-1), { generation: 1n, start: 3n, channels: 1, frames: 4, end: 0, pcm: [9, 8, 7, 6, 4, 5, 6, 0] });
  assert.equal(run.submissions.length, 4, "backpressure must capture the refused submit attempt");
  run.setSubmitResult(0); run.process(); assert.equal(writer.occupancy, 0);
  assert.equal(run.submissions.length, 5, "draining the retained slot must invoke source_submit again");
  assert.deepEqual(run.submissions.at(-1), { generation: 1n, start: 3n, channels: 1, frames: 4, end: 0, pcm: [9, 8, 7, 6, 4, 5, 6, 0] });
  writer.reserve(4)[0].fill(5); writer.commit({ generation: 1n, startFrame: 7n, frames: 4, endOfRegion: false });
  writer.seek(2n, 12n); run.setSeekResult(6); run.process(); assert.equal(run.ringControls[0][MSB1_CONTROL.SEEKS_APPLIED], 0); assert.equal(writer.occupancy, 1);
  assert.deepEqual(run.seeks, [[2n, 12n]]);
  run.setSeekResult(0); run.process(); assert.equal(run.ringControls[0][MSB1_CONTROL.SEEKS_APPLIED], 1); assert.equal(run.ringControls[0][MSB1_CONTROL.STALE], 1); assert.equal(writer.occupancy, 0);
  assert.deepEqual(run.seeks, [[2n, 12n], [2n, 12n]]);
  assert.equal(run.submissions.some(({ start }) => start === 7n), false);
  run.process(); assert.ok(run.ringControls[0][MSB1_CONTROL.UNDERRUNS] > 0);
  run.attach.port.onmessage({ data: { op: "detach" } }); assert.ok(run.ringControls.every((control) => control[MSB1_CONTROL.ATTACHED] === 0));
  assert.deepEqual(run.allocations, [], "first, later, partial and retry drains must allocate no typed arrays or views");
  const mutant = runPrelude(source, { tracking: true, mutate: true });
  mutant.process();
  mutant.process();
  const mutantWriter = mutant.writers[0];
  mutant.setSubmitResult(0);
  mutantWriter.reserve(4)[0].fill(1);
  mutantWriter.commit({ generation: 1n, startFrame: 4n, frames: 4, endOfRegion: false });
  mutant.process();
  assert.ok(mutant.allocations.includes("Float32Array"), "runtime mutation must be caught by constructor instrumentation");
});

function observedFixture(channels = 2, capacity = 4, frameCapacity = 4) {
  const ring = createMsb1Ring({ sourceId: "observation", channels, capacity, frameCapacity });
  const writer = new Msb1RingWriter(ring);
  writer.engage(1n);
  return { ring, writer, c: controls(ring), write(startFrame, frames = frameCapacity, generation = 1n) {
    const planes = writer.reserve(frames);
    assert.ok(planes);
    for (let ch = 0; ch < channels; ch++) for (let f = 0; f < frames; f++) planes[ch][f] = Number(startFrame) + ch * 100 + f;
    writer.commit({ startFrame: BigInt(startFrame), frames, generation, endOfRegion: frames < frameCapacity });
  } };
}

function runwayFixture(sourceId, { frames = 32n, capacity = 4, frameCapacity = 4 } = {}) {
  const ring = createMsb1Ring({ sourceId, channels: 1, capacity, frameCapacity });
  const writer = new Msb1RingWriter(ring);
  writer.engage(1n);
  return { sourceId, frames: BigInt(frames), ring, writer };
}

function runwayWrite(fixture, startFrame, frames = fixture.writer.frameCapacity, generation = 1n, endOfRegion = false) {
  const planes = fixture.writer.reserve(frames);
  assert.ok(planes);
  planes[0].fill(Number(startFrame));
  fixture.writer.commit({ generation, startFrame: BigInt(startFrame), frames, endOfRegion });
}

function runwayOptions(fixture, overrides = {}) {
  return {
    sources: [{ sourceId: fixture.sourceId, frames: fixture.frames, ring: fixture.ring }],
    targetFrame: 0n,
    generation: 1n,
    timeoutMs: 100,
    ...overrides,
  };
}

// Provenance for these moved proof cases: engine-web-adapter
// f833303f146de7cbe1705fe88ae68a6d6e0d4e45, src/session.ts::waitForRunway and the
// `initial readiness waits for every source's full runway` / `seek runway clips to the exact
// source tail` cases in tests/session.test.ts. Adapter lifecycle remains out of this SDK test.
test("PCM runway proves default and explicit bounded readiness without consuming rings", async () => {
  const full = runwayFixture("full", { frames: 16n, capacity: 4, frameCapacity: 4 });
  for (let start = 0; start < 16; start += 4) runwayWrite(full, start);
  const before = {
    read: controls(full.ring)[MSB1_CONTROL.READ_INDEX],
    seekEpoch: controls(full.ring)[MSB1_CONTROL.SEEK_EPOCH],
    generation: [...new BigInt64Array(full.ring, MSB1_CONTROL_I64_OFFSET, 2)],
    writerState: controls(full.ring)[MSB1_CONTROL.WRITER_STATE],
  };
  await waitForPcmRunway(runwayOptions(full));
  assert.equal(controls(full.ring)[MSB1_CONTROL.READ_INDEX], before.read);
  assert.equal(controls(full.ring)[MSB1_CONTROL.SEEK_EPOCH], before.seekEpoch);
  assert.deepEqual([...new BigInt64Array(full.ring, MSB1_CONTROL_I64_OFFSET, 2)], before.generation);
  assert.equal(controls(full.ring)[MSB1_CONTROL.WRITER_STATE], before.writerState);

  const small = runwayFixture("small", { frames: 20n, capacity: 4, frameCapacity: 4 });
  runwayWrite(small, 0);
  await waitForPcmRunway(runwayOptions(small, { minimumFrames: 3n }));
  assert.equal(small.writer.occupancy, 1, "an explicit minimum must not require the whole ring");

  const short = runwayFixture("short", { frames: 6n, capacity: 4, frameCapacity: 4 });
  runwayWrite(short, 0, 4);
  runwayWrite(short, 4, 2, 1n, true);
  await waitForPcmRunway(runwayOptions(short));
  const past = runwayFixture("past", { frames: 0n });
  await waitForPcmRunway(runwayOptions(past));
});

test("PCM runway waits for every source and snapshots source options", async () => {
  const first = runwayFixture("first", { frames: 8n });
  const second = runwayFixture("second", { frames: 8n });
  runwayWrite(first, 0);
  const sourceList = [
    { sourceId: first.sourceId, frames: first.frames, ring: first.ring },
    { sourceId: second.sourceId, frames: second.frames, ring: second.ring },
  ];
  const pending = waitForPcmRunway({ sources: sourceList, targetFrame: 0n, generation: 1n, timeoutMs: 200, minimumFrames: 4n });
  sourceList.length = 0;
  runwayWrite(second, 0);
  await pending;
});

test("PCM runway reports full-generation, contiguity and source-bound mismatches", async () => {
  const wrongGeneration = runwayFixture("wrong-generation", { frames: 4n });
  wrongGeneration.writer.engage(0x1_0000_0001n); // Low tag 1, full generation differs from the request.
  runwayWrite(wrongGeneration, 0, 4, 1n);
  await assert.rejects(
    waitForPcmRunway(runwayOptions(wrongGeneration, { generation: 0x1_0000_0001n })),
    (error) => error instanceof PcmRunwayError && error.reason === "mismatch" && error.sourceId === "wrong-generation",
  );

  const noncontiguous = runwayFixture("noncontiguous", { frames: 8n });
  runwayWrite(noncontiguous, 4);
  await assert.rejects(
    waitForPcmRunway(runwayOptions(noncontiguous)),
    (error) => error instanceof PcmRunwayError && error.reason === "mismatch" && error.sourceId === "noncontiguous",
  );

  const outOfRange = runwayFixture("out-of-range", { frames: 3n });
  runwayWrite(outOfRange, 0, 4);
  await assert.rejects(
    waitForPcmRunway(runwayOptions(outOfRange)),
    (error) => error instanceof PcmRunwayError && error.reason === "mismatch" && error.sourceId === "out-of-range",
  );
});

test("PCM runway timeout and abort release observers, preserving cancellation reason", async () => {
  const timed = runwayFixture("timed", { frames: 8n });
  await assert.rejects(
    waitForPcmRunway(runwayOptions(timed, { timeoutMs: 2 })),
    (error) => error instanceof PcmRunwayError && error.reason === "timeout" && error.sourceId === undefined,
  );

  const cancelled = runwayFixture("cancelled", { frames: 4n });
  const controller = new AbortController();
  const reason = new Error("caller cancelled");
  const readBefore = controls(cancelled.ring)[MSB1_CONTROL.READ_INDEX];
  const wait = waitForPcmRunway(runwayOptions(cancelled, { timeoutMs: 1_000, signal: controller.signal }));
  controller.abort(reason);
  await assert.rejects(wait, (error) => error === reason);
  assert.equal(controls(cancelled.ring)[MSB1_CONTROL.READ_INDEX], readBefore);
  // null is a legal explicit abort reason; retain it both during a wait and before entry.
  const nullController = new AbortController();
  const nullWait = waitForPcmRunway(runwayOptions(cancelled, { signal: nullController.signal }));
  nullController.abort(null);
  await assert.rejects(nullWait, (error) => error === null);
  await assert.rejects(
    waitForPcmRunway(runwayOptions(cancelled, { signal: nullController.signal })),
    (error) => error === null,
  );
  runwayWrite(cancelled, 0);
  await waitForPcmRunway(runwayOptions(cancelled));
});

test("PCM runway refuses invalid requests before waiting", async () => {
  const fixture = runwayFixture("valid", { frames: 8n, capacity: 2, frameCapacity: 4 });
  const valid = runwayOptions(fixture);
  const invalid = [
    { ...valid, sources: [{ ...valid.sources[0], sourceId: "" }] },
    { ...valid, sources: [valid.sources[0], valid.sources[0]] },
    { ...valid, targetFrame: -1n },
    { ...valid, generation: 0n },
    { ...valid, minimumFrames: 0n },
    { ...valid, minimumFrames: 9n },
    { ...valid, timeoutMs: 0 },
    { ...valid, timeoutMs: Infinity },
    { ...valid, sources: [{ ...valid.sources[0], frames: -1n }] },
    { ...valid, sources: [{ ...valid.sources[0], ring: new ArrayBuffer(128) }] },
  ];
  for (const options of invalid) await assert.rejects(waitForPcmRunway(options), (error) => error?.name === "MisoUsageError");
});

test("observer borrows reusable mono/stereo scratch without altering any shared byte", (t) => {
  for (const channels of [1, 2]) {
    const f = observedFixture(channels);
    f.write(10); f.write(20, 2);
    for (const [word, value] of [[MSB1_CONTROL.UNDERRUNS, 7], [MSB1_CONTROL.DRAIN_BLOCKS, 8], [MSB1_CONTROL.DEPTH, 9]]) Atomics.store(f.c, word, value);
    const before = Buffer.from(new Uint8Array(f.ring));
    const observer = new Msb1RingObserver(f.ring), independent = new Msb1RingObserver(f.ring);
    assert.equal(observer.channels, channels); assert.equal(observer.frameCapacity, 4);
    let metadata, planes, calls = 0;
    const native = globalThis.Float32Array;
    const constructor = t.mock.method(globalThis, "Float32Array", new Proxy(native, { construct() { throw new Error("pull allocated PCM"); } }));
    const subarray = t.mock.method(native.prototype, "subarray", () => { throw new Error("pull allocated a view"); });
    const slice = t.mock.method(native.prototype, "slice", () => { throw new Error("pull allocated PCM"); });
    try {
      assert.equal(observer.pull((chunk) => {
        if (!metadata) { metadata = chunk; planes = [...chunk.planes]; }
        else assert.equal(chunk, metadata);
        const start = calls++ === 0 ? 10 : 20;
        assert.equal(chunk.generation, 1n); assert.equal(chunk.startFrame, BigInt(start));
        assert.equal(chunk.frames, start === 10 ? 4 : 2); assert.equal(chunk.endOfRegion, start === 20);
        chunk.planes.forEach((plane, ch) => {
          assert.equal(plane, planes[ch]); assert.notEqual(plane.buffer, f.ring);
          assert.deepEqual([...plane], start === 10 ? [10 + ch * 100, 11 + ch * 100, 12 + ch * 100, 13 + ch * 100] : [20 + ch * 100, 21 + ch * 100, 0, 0]);
        });
      }, 1), 1);
      assert.equal(observer.pull((chunk) => {
        assert.equal(chunk, metadata); assert.equal(chunk.planes[0], planes[0]);
        assert.equal(chunk.frames, 2); assert.equal(chunk.endOfRegion, true);
        assert.deepEqual([...chunk.planes[0]], [20, 21, 0, 0]);
      }), 1);
    } finally { constructor.mock.restore(); subarray.mock.restore(); slice.mock.restore(); }
    assert.equal(independent.pull(() => {}), 2);
    assert.equal(observer.pull(() => assert.fail("already observed")), 0);
    assert.deepEqual(observer.counters(), { wrote: 2, overflow: 0, submitted: 0, stale: 0, refused: 0, lastResult: 0, underruns: 7, drainBlocks: 8, depth: 9, seeksApplied: 0, torn: 0, errors: 0, occupancy: 2, generationTag: 1, submittedGenerationTag: 0 });
    const finalCounters = observer.counters();
    observer.close(); observer.close(); independent.close();
    assert.equal(observer.pull(() => assert.fail("closed")), 0);
    assert.deepEqual(observer.counters(), finalCounters);
    assert.deepEqual(Buffer.from(new Uint8Array(f.ring)), before);
  }
  assert.throws(() => new Msb1RingObserver(new ArrayBuffer(128)), /SharedArrayBuffer/);
  const bad = createMsb1Ring({ sourceId: "bad", channels: 1, capacity: 2, frameCapacity: 4 });
  controls(bad)[MSB1_CONTROL.MAGIC] = 0;
  assert.throws(() => new Msb1RingObserver(bad), /MSB1/);
});

test("observer bounds candidate work, catches up, wraps and remains independent of audio consumption", () => {
  const f = observedFixture(1, 64, 1);
  for (let i = 0; i < 40; i++) f.write(i);
  const observer = new Msb1RingObserver(f.ring);
  for (const invalid of [0, -1, 33, 1.5, NaN, Infinity]) assert.throws(() => observer.pull(() => {}, invalid), /maximumChunks/);
  const starts = [];
  assert.equal(observer.pull((chunk) => starts.push(chunk.startFrame)), 32);
  assert.equal(observer.pull((chunk) => starts.push(chunk.startFrame)), 8);
  assert.deepEqual(starts, Array.from({ length: 40 }, (_, i) => BigInt(i)));
  assert.equal(f.c[MSB1_CONTROL.READ_INDEX], 0);
  const wrapped = observedFixture(1, 2, 1);
  Atomics.store(wrapped.c, MSB1_CONTROL.READ_INDEX, MSB1_WRAP - 1);
  Atomics.store(wrapped.c, MSB1_CONTROL.WRITE_INDEX, MSB1_WRAP - 1);
  wrapped.write(1); wrapped.write(2);
  const slow = new Msb1RingObserver(wrapped.ring);
  assert.equal(slow.pull((chunk) => assert.equal(chunk.startFrame, 1n), 1), 1);
  Atomics.store(wrapped.c, MSB1_CONTROL.READ_INDEX, 1);
  wrapped.write(3); wrapped.write(4);
  const caught = [];
  assert.equal(slow.pull((chunk) => caught.push(chunk.startFrame)), 2);
  assert.deepEqual(caught, [3n, 4n]);
  const fresh = new Msb1RingObserver(wrapped.ring);
  assert.equal(fresh.pull((chunk) => { assert.equal(chunk.startFrame, 3n); fresh.close(); }), 1);
  assert.equal(fresh.pull(() => assert.fail("closed in callback")), 0);
  const bounded = new Msb1RingObserver(f.ring);
  headers(f.ring, 64)[2] = 0; // Invalid first candidate still consumes the explicit budget.
  assert.equal(bounded.pull(() => assert.fail("invalid candidate"), 1), 0);
  assert.equal(bounded.pull((chunk) => assert.equal(chunk.startFrame, 1n), 1), 1);
});

test("observer rejects stale, invalid and torn slots and follows full seek generations", () => {
  const f = observedFixture(1, 8, 1);
  for (let i = 0; i < 5; i++) f.write(i);
  const h = headers(f.ring, 8);
  h[1] = 2; // Stale tag.
  h[8] = 0; // Torn sequence.
  h[18] = 2; // Frames exceed capacity.
  headers64(f.ring, 8)[14] = 2n; // Full generation disagrees with tag.
  const observer = new Msb1RingObserver(f.ring);
  assert.equal(observer.pull((chunk) => assert.equal(chunk.startFrame, 4n)), 1);
  const generation = (1n << 32n) + 1n; // Same low tag, new seek epoch.
  f.writer.seek(generation, 5n);
  f.write(5, 1, generation);
  assert.equal(observer.pull((chunk) => { assert.equal(chunk.generation, generation); assert.equal(chunk.startFrame, 5n); }), 1);
  const signed = 0x80000001n;
  f.writer.seek(signed, 6n); f.write(6, 1, signed);
  assert.equal(observer.pull((chunk) => { assert.equal(chunk.generation, signed); assert.equal(chunk.startFrame, 6n); }), 1);
  assert.throws(() => new Msb1RingObserver(f.ring).pull(() => { observer.pull(() => {}); throw new Error("consumer failure"); }), /consumer failure/);
  const reentrant = new Msb1RingObserver(f.ring);
  assert.equal(reentrant.pull(() => assert.throws(() => reentrant.pull(() => {}), /reentered/)), 1);
});

test("observer drops PCM reused during copy before sequence publication, and seeks during copy", (t) => {
  for (const mode of ["reuse", "seek"]) {
    const f = observedFixture(2, 1, 4);
    f.write(10);
    const observer = new Msb1RingObserver(f.ring);
    const originalSet = Float32Array.prototype.set;
    let mutated = false, afterMutation;
    const hook = t.mock.method(Float32Array.prototype, "set", function (source, offset) {
      originalSet.call(this, source, offset);
      if (!mutated && source.buffer === f.ring && this.buffer !== f.ring) {
        mutated = true;
        if (mode === "reuse") {
          Atomics.store(f.c, MSB1_CONTROL.READ_INDEX, 1);
          assert.ok(f.writer.reserve(4)); // Zeroes both shared planes WITHOUT publishing a new sequence.
          assert.equal(headers(f.ring, 1)[0], 0);
        } else f.writer.seek(2n, 20n);
        afterMutation = Buffer.from(new Uint8Array(f.ring));
      }
    });
    try { assert.equal(observer.pull(() => assert.fail("delivered raced PCM")), 0); }
    finally { hook.mock.restore(); }
    assert.equal(mutated, true);
    assert.deepEqual(Buffer.from(new Uint8Array(f.ring)), afterMutation);
    if (mode === "reuse") {
      f.writer.commit({ generation: 1n, startFrame: 20n, frames: 4, endOfRegion: false });
      assert.equal(observer.pull((chunk) => { assert.equal(chunk.startFrame, 20n); assert.deepEqual([...chunk.planes[0]], [0, 0, 0, 0]); }), 1);
    }
  }
});

// Issue #2102: the browser entry's canonical PCM layer, against the engine's frozen stem-identity
// corpus (docs/STEM_IDENTITY_V1.md).
const STEM_CORPUS = new URL("../../fixtures/stem-identity/v1/", import.meta.url);
async function stemVector(name) {
  const [, header, ...rows] = (await readFile(new URL("VECTORS.tsv", STEM_CORPUS), "utf8")).trim().split("\n").map((line) => line.split("\t"));
  const row = Object.fromEntries(header.map((key, index) => [key, rows.find((cells) => cells[0] === name)[index]]));
  return { ...row, channels: Number(row.channels), frames: Number(row.frames), pcm: await readFile(new URL(row.pcm_file, STEM_CORPUS)),
    samples: row.samples_by_frame.split("|").map((frame) => frame.split(",").map(Number)) };
}

test("the package's CanonicalPcmPump writes canonical 16-bit mono and 24-bit stereo into Msb1RingWriter rings as exact f32", async () => {
  // The published barrel's pump writes into the SDK's own Msb1RingWriter, and the SDK's
  // Msb1RingObserver reads back every boundary sample exactly.
  const stems = await import("../src/browser/index.ts");
  const vectors = [await stemVector("pcm16-mono-boundaries"), await stemVector("pcm24-stereo-boundaries")];
  const rings = vectors.map((v) => createMsb1Ring({ sourceId: v.name, channels: v.channels, frameCapacity: 2, capacity: 4 }));
  const pump = new stems.CanonicalPcmPump({
    lease: { read: async (identity) => new Blob([vectors.find((v) => v.identity === identity).pcm]) },
    sources: vectors.map((v, index) => ({ sourceId: v.name, identity: v.identity, channels: v.channels,
      bitDepth: Number(v.bit_depth), frames: v.frames, ring: new Msb1RingWriter(rings[index]) })),
  });
  assert.deepEqual(await pump.pumpUntilFull(), { chunks: 5, frames: 8, finished: true });
  vectors.forEach((v, index) => {
    const planes = Array.from({ length: v.channels }, () => []); const starts = []; let ended = false;
    new Msb1RingObserver(rings[index]).pull((chunk) => {
      starts.push(chunk.startFrame); ended = chunk.endOfRegion;
      chunk.planes.forEach((plane, channel) => planes[channel].push(...plane.subarray(0, chunk.frames)));
    });
    const scale = 2 ** (Number(v.bit_depth) - 1);
    assert.deepEqual(starts, [0n, 2n, 4n].slice(0, Math.ceil(v.frames / 2)), v.name);
    assert.equal(ended, true, `${v.name} ends its region`);
    assert.deepEqual(planes, Array.from({ length: v.channels }, (_, channel) => v.samples.map((frame) => frame[channel] / scale)), v.name);
  });
});

function riffWave(chunks, { riff = "RIFF" } = {}) {
  const parts = chunks.flatMap(([id, body, size = body.byteLength]) => {
    const header = Buffer.alloc(8); header.write(id, 0, "latin1"); header.writeUInt32LE(size, 4);
    return body.byteLength % 2 === 1 ? [header, body, Buffer.alloc(1)] : [header, body];
  });
  const head = Buffer.alloc(12); head.write(riff, 0, "latin1"); head.write("WAVE", 8, "latin1");
  head.writeUInt32LE(4 + parts.reduce((sum, part) => sum + part.byteLength, 0), 4);
  return Buffer.concat([head, ...parts]);
}
function waveFormat({ tag = 1, channels = 2, rate = 48_000, bits = 24, extensible = false, guidTail = "000000001000800000aa00389b71" }) {
  const fmt = Buffer.alloc(extensible ? 40 : 16);
  fmt.writeUInt16LE(extensible ? 0xfffe : tag, 0); fmt.writeUInt16LE(channels, 2); fmt.writeUInt32LE(rate, 4);
  fmt.writeUInt32LE(rate * channels * (bits / 8), 8); fmt.writeUInt16LE(channels * (bits / 8), 12); fmt.writeUInt16LE(bits, 14);
  if (extensible) {
    fmt.writeUInt16LE(22, 16); fmt.writeUInt16LE(bits, 18); fmt.writeUInt32LE(3, 20); fmt.writeUInt16LE(tag, 24);
    Buffer.from(guidTail, "hex").copy(fmt, 26);
  }
  return fmt;
}

test("readWavHeader strips plain and extensible WAVs to the canonical bytes of the engine's blake3 identity", async () => {
  // The two header layouts a real stem has (plain and WAVE_FORMAT_EXTENSIBLE), an odd chunk's pad
  // byte and an unpatched data size each move dataOffset or canonicalBytes, and the identity of
  // the slice then misses the corpus; a reader that probes ever larger prefixes reads too much.
  const { readWavHeader, blake3Stream, WavFormatError } = await import("../src/browser/index.ts");
  const identity = async (blob, info) => `blake3:${(await blake3Stream(blob.slice(info.dataOffset, info.dataOffset + info.canonicalBytes).stream())).hex}`;
  for (const name of ["pcm16-stereo-boundaries", "pcm24-stereo-boundaries"]) {
    const v = await stemVector(name), blob = new Blob([await readFile(new URL(v.wave_file, STEM_CORPUS))]);
    const info = await readWavHeader(blob);
    assert.deepEqual(info, { sampleRateHz: 48_000, channels: 2, bitDepth: Number(v.bit_depth), frames: 3, dataOffset: 44, canonicalBytes: v.pcm.byteLength }, name);
    assert.equal(await identity(blob, info), v.identity, name);
  }
  const v = await stemVector("pcm24-stereo-boundaries");
  const odd = Buffer.from("abc", "latin1");
  const reads = [];
  const bytes = riffWave([["fmt ", waveFormat({ extensible: true })], ["LIST", odd], ["data", v.pcm, 0xffff_ffff]]);
  const real = new Blob([bytes]);
  const recorded = { size: real.size, slice(start, end) { reads.push(end - start); return real.slice(start, end); } };
  const info = await readWavHeader(recorded);
  assert.deepEqual(info, { sampleRateHz: 48_000, channels: 2, bitDepth: 24, frames: 3, dataOffset: 12 + 8 + 40 + 8 + 4 + 8, canonicalBytes: 18 });
  assert.equal(await identity(real, info), v.identity);
  assert.deepEqual(reads, [12, 8, 40, 8, 8], "one 12-byte RIFF read, an 8-byte read per chunk header and one fmt body read");
  for (const rate of [44_100, 88_200, 96_000]) {
    assert.equal((await readWavHeader(new Blob([riffWave([["fmt ", waveFormat({ rate, bits: 16, channels: 1 })], ["data", v.pcm.subarray(0, 4)]])]))).sampleRateHz, rate);
  }
  // RF64/BW64: the data size comes from ds64. The trailing chunk makes a reader that ignores ds64
  // and reads to the file's end take its bytes as frames.
  const ds64 = (dataSize) => { const body = Buffer.alloc(28); body.writeBigUInt64LE(BigInt(dataSize), 8); body.writeBigUInt64LE(3n, 16); return body; };
  const tail = ["LIST", Buffer.from("tail", "latin1")];
  const twin = new Blob([riffWave([["JUNK", Buffer.alloc(28)], ["fmt ", waveFormat({})], ["data", v.pcm], tail])]);
  const twinInfo = await readWavHeader(twin);
  for (const magic of ["RF64", "BW64"]) {
    const wide = new Blob([riffWave([["ds64", ds64(v.pcm.byteLength)], ["fmt ", waveFormat({})], ["data", v.pcm, 0xffff_ffff], tail], { riff: magic })]);
    const wideInfo = await readWavHeader(wide);
    assert.deepEqual(wideInfo, twinInfo, `${magic} reads like its RIFF twin`);
    assert.equal(await identity(wide, wideInfo), v.identity, magic);
  }
  const refusal = async (blob, code, message) => {
    await assert.rejects(readWavHeader(blob), (error) => error instanceof WavFormatError && error.code === code && message.test(error.message));
  };
  await refusal(new Blob([await readFile(new URL("f32-stereo-edge-bits.wav", STEM_CORPUS))]), "wav.float_unsupported", /32-bit float.*export 16- or 24-bit/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({ tag: 3, bits: 32, extensible: true })], ["data", v.pcm]])]), "wav.float_unsupported", /export 16- or 24-bit/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({ extensible: true, guidTail: "000000001000800000aa00389b72" })], ["data", v.pcm]])]), "wav.format_unsupported", /not a WAVE format/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({ channels: 3 })], ["data", v.pcm]])]), "wav.channels", /mono or stereo/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({ rate: 192_000 })], ["data", v.pcm]])]), "wav.sample_rate", /44\.1, 48, 88\.2 or 96 kHz/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({})], ["data", v.pcm, 0xffff_ffff]], { riff: "RF64" })]), "wav.ds64_missing", /needs a ds64 chunk before it/);
  await refusal(new Blob([riffWave([["fmt ", waveFormat({})], ["data", v.pcm, 0xffff_ffff], ["ds64", ds64(v.pcm.byteLength)]], { riff: "RF64" })]), "wav.ds64_missing", /needs a ds64 chunk before it/);
  await refusal(new Blob([riffWave([["ds64", Buffer.alloc(8)], ["fmt ", waveFormat({})], ["data", v.pcm, 0xffff_ffff]], { riff: "RF64" })]), "wav.ds64_short", /too short/);
});

test("the PCM feed worklet calls only exports that the render-thread table lists for it", () => {
  // The actual-Wasm and prelude tests above drive the feed's process drain and both seek paths,
  // so the recording is not empty; an empty one would mean the proxy lost its hook.
  for (const name of ["source_submit", "source_seek"]) assert.ok(recordedFeedCalls.has(name), `the recording proxy saw the feed worklet call ${name}`);
  const unlisted = [...recordedFeedCalls].filter((name) => !feedRenderThreadExports.has(name));
  assert.deepEqual(unlisted, [], `the feed worklet calls these exports after construction, and tools/realtime-policy/render-thread-exports.json does not list them for it: ${unlisted.map((name) => renderThreadTable.prefix + name).join(", ")}`);
});
