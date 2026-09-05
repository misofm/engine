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
} from "../src/browser/pcm-ring.ts";
import { attachEngineFeed, PcmFeedError, prepareEngineFeed } from "../src/browser/pcm-feed.ts";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";

const context = { audioWorklet: { addModule: async () => {} } };

function controls(ring) { return new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4); }
function headers(ring, capacity) { return new Int32Array(ring, MSB1_HEADER_OFFSET, capacity * MSB1_SLOT_HEADER_BYTES / 4); }
function headers64(ring, capacity) { return new BigInt64Array(ring, MSB1_HEADER_OFFSET, capacity * MSB1_SLOT_HEADER_BYTES / 8); }

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
    assert.deepEqual([...c.slice(0, 8)], [0x4d534231, 1, 2, channels, 5, 256, expected[channels].pcm, sourceId.length]);
    assert.deepEqual([...c.slice(8, 28)], Array(20).fill(0).map((value, index) => index === 2 ? 1 : value));
    assert.deepEqual([...new Uint8Array(ring, MSB1_ID_OFFSET, sourceId.length)], [...new TextEncoder().encode(sourceId)]);

    const writer = new Msb1RingWriter(ring);
    writer.engage(7n);
    assert.equal(c[MSB1_CONTROL.WRITER_STATE], 1);
    const planes = writer.reserve(3);
    assert.equal(planes.length, channels);
    assert.ok(planes.every((plane) => plane.every((value) => Object.is(value, 0))));
    planes[0].set([1, 2, 3]);
    if (channels === 2) planes[1].set([4, 5, 6]);
    writer.commit({ generation: 7n, startFrame: 11n, frames: 3, endOfRegion: true });
    assert.deepEqual([...headers(ring, 2).slice(0, 4)], [0, 7, 3, 1]);
    assert.deepEqual([...headers64(ring, 2).slice(0, 4)], [7n << 32n, (1n << 32n) | 3n, 7n, 11n]);
    assert.equal(c[MSB1_CONTROL.WRITE_INDEX], 1);
    assert.equal(c[MSB1_CONTROL.WROTE], 1);
    assert.equal(writer.occupancy, 1);
    assert.deepEqual([...planes[0]], [1, 2, 3, 0, 0]);
    writer.seek(8n, 14n);
    assert.deepEqual([...new BigInt64Array(ring, 112, 2)], [8n, 14n]);
    assert.equal(c[MSB1_CONTROL.GENERATION_TAG], 8);
    assert.equal(c[MSB1_CONTROL.SEEK_EPOCH], 1);
    writer.release();
    assert.equal(c[MSB1_CONTROL.WRITER_STATE], 0);
  }

  const exactId = "é".repeat(64);
  assert.equal(new TextEncoder().encode(exactId).byteLength, MSB1_ID_CAPACITY);
  createMsb1Ring({ sourceId: exactId, channels: 1, frameCapacity: 4, capacity: 2 });
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
  assert.equal(writer.reserve(4), null);
  assert.equal(controls(ring)[MSB1_CONTROL.OVERFLOW], 1);
  assert.deepEqual([...headers(ring, 2)], before);
  Atomics.store(controls(ring), MSB1_CONTROL.READ_INDEX, 1);
  const reused = writer.reserve(4);
  assert.ok(reused); assert.ok(reused.every((plane) => plane.every((value) => Object.is(value, 0))));
  writer.commit({ generation: 1n, startFrame: 8n, frames: 4, endOfRegion: false });
  assert.equal(writer.occupancy, 2);

  assert.throws(() => new Msb1RingWriter(new ArrayBuffer(128)), /SharedArrayBuffer/);
  const malformed = createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 2 });
  new Int32Array(malformed)[MSB1_CONTROL.VERSION] = 2;
  assert.throws(() => new Msb1RingWriter(malformed), /MSB1/);
  assert.throws(() => createMsb1Ring({ sourceId: "bad", channels: 1, frameCapacity: 4, capacity: 3 }), /power of two/);
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
  const empty = attachEngineFeed({ context, sources: [], quantumFrames: 4, createNode: () => attachNode() });
  await empty.ready(); assert.equal(empty.state, "active");
  globalThis.AudioWorkletNode = oldNode;

  let waits = 0; const never = new Promise(() => {});
  let disconnects = 0; let detaches = 0;
  const blockedNode = attachNode({ onMessage: (message) => { if (message.op === "detach") detaches += 1; }, onDisconnect: () => { disconnects += 1; } });
  const blocked = attachEngineFeed({ context, sources: [{ sourceId: "blocked", channels: 1 }], quantumFrames: 4, createNode: () => blockedNode });
  new Msb1RingWriter(blocked.rings[0]).engage(1n);
  const first = blocked.ready({ wait: async () => { waits += 1; return never; } });
  const second = blocked.ready({ wait: async () => { waits += 1; return never; } });
  assert.equal(waits, 2);
  blocked.close(); blocked.close();
  const blockedResults = await Promise.allSettled([first, second]);
  assert.deepEqual(blockedResults.map((result) => result.status === "rejected" && result.reason.operation), ["closed", "closed"]);
  assert.equal(disconnects, 1); assert.equal(detaches, 1);
  assert.equal(controls(blocked.rings[0])[MSB1_CONTROL.WRITER_STATE], 0);

  let clock = 0; let timeoutDisconnects = 0; let timeoutDetaches = 0;
  const timed = attachEngineFeed({ context, sources: [{ sourceId: "timeout", channels: 1 }], quantumFrames: 4, createNode: () => attachNode({ onMessage: (message) => { if (message.op === "detach") timeoutDetaches += 1; }, onDisconnect: () => { timeoutDisconnects += 1; } }) });
  const tick = async () => { clock = 2; };
  const timing = timed.ready({ timeoutMs: 1, now: () => clock, wait: tick });
  const waiting = timed.ready({ timeoutMs: 1, now: () => clock, wait: tick });
  const timeoutResults = await Promise.allSettled([timing, waiting]);
  assert.deepEqual(timeoutResults.map((result) => result.status === "rejected" && result.reason.operation).sort(), ["closed", "readyTimeout"]);
  assert.equal(timeoutDisconnects, 1); assert.equal(timeoutDetaches, 1);
  assert.equal(controls(timed.rings[0])[MSB1_CONTROL.WRITER_STATE], 0);
});

test("attach-post cleanup releases engaged rings before throwing cleanup failures", () => {
  let messageRings = [];
  let disconnects = 0;
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
        throw new Error("detach blocked");
      },
      onDisconnect: () => { disconnects += 1; throw new Error("disconnect blocked"); },
    }),
  }), (error) => error instanceof PcmFeedError && error.operation === "attachPost");
  assert.equal(disconnects, 1);
  assert.ok(messageRings.length === 3);
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
  const mutated = mutate ? source.replace("const control = ring.control", "new Float32Array(4); const control = ring.control") : source;
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
  if (tracking) arm();
  const process = () => engine.process([], []);
  if (tracking) { process(); process(); } else process();
  return { sandbox, allocations, engine, attach, rings, ringControls, writers, submissions, seeks, process, setSubmitResult: (value) => { submitResult = value; }, setSeekResult: (value) => { seekResult = value; } };
}

test("moved prelude drains odd mono/stereo rings and allocation mutation turns red", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const run = runPrelude(source, { tracking: true });
  assert.deepEqual(run.submissions.map(({ channels, frames, end }) => ({ channels, frames, end })), [
    { channels: 1, frames: 3, end: 1 }, { channels: 2, frames: 3, end: 1 }, { channels: 1, frames: 3, end: 1 },
  ]);
  assert.deepEqual(run.submissions[1].pcm, [2, 2, 3, 0, 4, 5, 6, 0]);
  assert.deepEqual(run.allocations, [], "first and steady drains must allocate no typed arrays or views");
  assert.ok(run.ringControls.every((control) => control[MSB1_CONTROL.SUBMITTED] === 1));
  run.setSubmitResult(6); const writer = run.writers[0]; const plane = writer.reserve(4); plane[0].set([9, 8, 7, 6]); writer.commit({ generation: 1n, startFrame: 3n, frames: 4, endOfRegion: false });
  run.process(); assert.equal(writer.occupancy, 1); assert.equal(run.ringControls[0][MSB1_CONTROL.REFUSED], 0);
  run.setSubmitResult(0); run.process(); assert.equal(writer.occupancy, 0);
  writer.reserve(4)[0].fill(5); writer.commit({ generation: 1n, startFrame: 7n, frames: 4, endOfRegion: false });
  writer.seek(2n, 12n); run.setSeekResult(6); run.process(); assert.equal(run.ringControls[0][MSB1_CONTROL.SEEKS_APPLIED], 0); assert.equal(writer.occupancy, 1);
  run.setSeekResult(0); run.process(); assert.equal(run.ringControls[0][MSB1_CONTROL.SEEKS_APPLIED], 1); assert.equal(run.ringControls[0][MSB1_CONTROL.STALE], 1); assert.equal(writer.occupancy, 0);
  run.process(); assert.ok(run.ringControls[0][MSB1_CONTROL.UNDERRUNS] > 0);
  run.attach.port.onmessage({ data: { op: "detach" } }); assert.ok(run.ringControls.every((control) => control[MSB1_CONTROL.ATTACHED] === 0));
  const mutant = runPrelude(source, { tracking: true, mutate: true });
  assert.ok(mutant.allocations.includes("Float32Array"), "runtime mutation must be caught by constructor instrumentation");
});
