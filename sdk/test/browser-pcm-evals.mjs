import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { createMsb1Ring, Msb1RingWriter, MSB1_CONTROL } from "../src/browser/pcm-ring.ts";
import { attachEngineFeed, PcmFeedError, prepareEngineFeed } from "../src/browser/pcm-feed.ts";
import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";

test("MSB1 layout and writer preserve mono/stereo sizes, zero fill, tail, seek and release", () => {
  for (const channels of [1, 2]) {
    const ring = createMsb1Ring({ sourceId: channels === 1 ? "mono" : "stereo", channels, frameCapacity: 5, capacity: 2 });
    const writer = new Msb1RingWriter(ring);
    writer.engage(7n);
    const planes = writer.reserve(3);
    assert.equal(planes?.length, channels);
    assert.ok(planes?.every((plane) => plane.every((value) => value === 0)));
    planes?.[0]?.set([1, 2, 3]);
    writer.commit({ generation: 7n, startFrame: 11n, frames: 3, endOfRegion: true });
    writer.seek(8n, 14n);
    writer.release();
    assert.equal(new Int32Array(ring)[MSB1_CONTROL.WRITER_STATE], 0);
  }
  assert.throws(() => createMsb1Ring({ sourceId: "x", channels: 1, frameCapacity: 4, capacity: 3 }), /power of two/);
});

test("feed lifecycle uses default URL, injected node, exact rings, cleanup, timeout and close", async () => {
  const calls = []; let disconnected = 0; let closedWait;
  const context = { audioWorklet: { addModule: async (url) => calls.push(url) } };
  await prepareEngineFeed(context);
  assert.equal(calls[0], String(BUNDLED_ENGINE_ASSETS.pcmFeedWorklet));
  const node = { port: { postMessage(message) { calls.push(message); if (message.op === "attach") closedWait?.(); } }, disconnect() { disconnected += 1; } };
  const feed = attachEngineFeed({ context, sources: [{ sourceId: "s", channels: 1 }], quantumFrames: 4, createNode: () => node });
  assert.equal(calls.at(-1).op, "attach");
  feed.close(); feed.close(); assert.equal(disconnected, 1); assert.equal(feed.state, "closed");
  await assert.rejects(feed.ready(), (error) => error instanceof PcmFeedError && error.operation === "closed");
  const timed = attachEngineFeed({ context, sources: [{ sourceId: "t", channels: 1 }], quantumFrames: 4, createNode: () => ({ port: { postMessage() {} }, disconnect() {} }) });
  let now = 0; await assert.rejects(timed.ready({ timeoutMs: 2, now: () => now++, wait: async () => {} }), (error) => error.operation === "readyTimeout");
  assert.equal(timed.state, "closed");
});

test("attach post failure releases rings and disconnects node", () => {
  let disconnected = 0; let ring;
  assert.throws(() => attachEngineFeed({ context: { audioWorklet: { addModule: async () => {} } }, sources: [{ sourceId: "s", channels: 1 }], quantumFrames: 4, createNode: () => ({ port: { postMessage() { throw new Error("blocked") } }, disconnect() { disconnected += 1 } }) }), (error) => error.operation === "attachPost");
  assert.equal(disconnected, 1); void ring;
});

test("moved prelude remains allocation-free in its drain source and is addressable", async () => {
  const source = await readFile(new URL("../src/browser-assets/miso-engine-v1-pcm-feed-worklet.js", import.meta.url), "utf8");
  const drain = source.slice(source.indexOf("drainSharedRing(ring)"), source.indexOf("/** The rings' way in"));
  assert.equal(/new Uint8Array\s*\(/u.test(drain), false); assert.equal(/\.subarray\s*\(/u.test(drain), false);
  assert.match(source, /miso_engine_web_v1_source_submit/); assert.match(source, /RESULT_BACKPRESSURE/);
});
