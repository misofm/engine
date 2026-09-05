import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import vm from "node:vm";

import {
  MSB1_CONTROL,
  MSB1_CONTROL_BYTES,
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
import { attachEngineFeed, PcmFeedError, prepareEngineFeed } from "../src/browser/pcm-feed.ts";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";

const context = { audioWorklet: { addModule: async () => {} } };

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
    [1]: { bytes: 360, pcm: 320 },
    [2]: { bytes: 400, pcm: 320 },
  };
  for (const channels of [1, 2]) {
    const sourceId = channels === 1 ? "mono" : "stereo";
    const ring = createMsb1Ring({ sourceId, channels, frameCapacity: 5, capacity: 2 });
    assert.equal(ring.byteLength, expected[channels].bytes);
    assert.equal(msb1RingBytes(channels, 5, 2), expected[channels].bytes);
    const c = controls(ring);
    assert.equal(MSB1_CONTROL_BYTES, 128);
    assert.equal(MSB1_ID_OFFSET, 128);
    assert.equal(MSB1_ID_CAPACITY, 128);
    assert.equal(MSB1_HEADER_OFFSET, 256);
    assert.equal(MSB1_SLOT_HEADER_BYTES, 32);
    assert.deepEqual([...c], [
      0x4d534231, 1, 2, channels, 5, 256, expected[channels].pcm, sourceId.length,
      0, 0, 1, ...Array(21).fill(0),
    ]);
    assert.deepEqual([...c.slice(28)], [0, 0, 0, 0]);
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
    assert.deepEqual([...new BigInt64Array(ring, 112, 2)], [nextGeneration, 14n]);
    assert.equal(c[MSB1_CONTROL.GENERATION_TAG], 8);
    assert.equal(c[MSB1_CONTROL.SEEK_EPOCH], 1);
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

function runPrelude(source, { tracking = false, mutate = false } = {}) {
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
      this.exports = { memory: { buffer: this.memoryBuffer }, miso_engine_web_v1_source_seek: (_h, _id, generation, frame) => { seeks.push([generation, frame]); return seekResult; }, miso_engine_web_v1_source_submit: (_h, _id, generation, start, channels, frames, end) => { submissions.push({ generation, start, channels, frames, end, pcm: [...this.sourcePcm] }); return submitResult; } };
    }
    process() { return true; }
  }
  sandbox.registerProcessor("miso-engine-v1-audio-worklet", Engine);
  const engine = new (registrations.get("miso-engine-v1-audio-worklet"))();
  const attach = new (registrations.get("miso-sab-feed-attach"))();
  const rings = [1, 2, 1].map((channels, index) => createMsb1Ring({ sourceId: `source-${index}`, channels, frameCapacity: 4, capacity: 2 }));
  const ringControls = rings.map(controls);
  attach.port.onmessage({ data: { op: "attach", rings } });
  const writers = rings.map((ring) => new Msb1RingWriter(ring));
  for (const [index, writer] of writers.entries()) {
    writer.engage(1n);
    const planes = writer.reserve(3);
    planes[0].set([index + 1, 2, 3]);
    if (writer.channels === 2) planes[1].set([4, 5, 6]);
    writer.commit({ generation: 1n, startFrame: 0n, frames: 3, endOfRegion: true });
  }
  const process = () => engine.process([], []);
  if (tracking) arm();
  return { sandbox, allocations, engine, attach, rings, ringControls, writers, submissions, seeks, process, setSubmitResult: (value) => { submitResult = value; }, setSeekResult: (value) => { seekResult = value; } };
}

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
