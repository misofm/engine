/**
 * The browser engine's spectrum calls over the three frames (#1596 D1-D7). The host stand-in's
 * control plane is a real core (a `WasmBoundary` on the shipped module): `control(frame)` runs the
 * frame through the entry the worklet's message handler calls, and `render()` feeds and renders
 * blocks as the audio thread does. The host has no spectrum method of its own, so an SDK path that
 * still called one fails here. The three-browser rows are the qualification's (#1596 gate 5).
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { onEndpointIds } from "./control-support.mjs";
import { moduleBytes, ramp, sessionDocument } from "./support.mjs";

const SKIP = { skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX };
const QUANTUM = 128;
const WINDOW_BLOCKS = 2_048 / QUANTUM;
const OUTPUT = Object.freeze({ target: Object.freeze({ kind: "output", outputId: "out" }), channels: "both" });
const TRACK = Object.freeze({ target: Object.freeze({ kind: "trackPostPan", trackId: "t" }), channels: "both" });
const BOTH = Object.freeze({
  entries: Object.freeze([OUTPUT, TRACK]),
  maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes * 2,
});

let assetPromise;
const candidateAsset = () => assetPromise ??= moduleBytes().then((bytes) => MisoEngineAsset.load(bytes));

const flush = () => new Promise((resolve) => setImmediate(resolve));

/**
 * A browser engine over a served core that staged the boot's taps, as the worklet boot does.
 * `harness.frames` counts the frames the page sent; `harness.hold`, when set, holds the reply of the
 * next frame (the core has served it) until that promise settles.
 */
async function served({ spectrum, spectrumCollection, policy, afterCall } = {}) {
  const document = sessionDocument({ frames: 64 * 2_048 });
  const core = await WasmBoundary.boot(await candidateAsset(), new TextEncoder().encode(document), {
    ...(spectrum === undefined ? {} : { spectrum }),
    ...(spectrumCollection === undefined ? {} : { spectrumCollection }),
  });
  const forward = onEndpointIds(core.controlRequestIds, (frame) => core.controlFrame(frame));
  const harness = { frames: 0, hold: undefined, hostOptions: undefined };
  let block = 0;
  const host = {
    async control(frame) {
      harness.frames += 1;
      const response = forward(frame);
      const hold = harness.hold;
      harness.hold = undefined;
      if (hold !== undefined) await hold;
      return { tag: "miso.control.v1", requestId: harness.frames, result: 0, response, revision: core.controlRevision() };
    },
    onControlEvent: () => () => {},
    async dispose() { core.dispose(); },
  };
  const render = (blocks = 1) => {
    for (let index = 0; index < blocks; index += 1) {
      const ack = core.submitSource({
        sourceId: "s",
        generation: 1n,
        startFrame: BigInt(block * QUANTUM),
        planes: [ramp(QUANTUM, 17 + block), ramp(QUANTUM, 71 + block)],
        endOfRegion: false,
      });
      assert.equal(ack.ok, true, `source block ${block} was refused`);
      core.render(QUANTUM);
      block += 1;
    }
  };
  const pending = createEngine({
    document,
    ...(spectrum === undefined ? {} : { spectrum }),
    ...(spectrumCollection === undefined ? {} : { spectrumCollection }),
    ...(policy === undefined ? {} : { policy }),
    scratchBoot: async () => ({ sampleRateHz: 48_000, quantumFrames: QUANTUM, tracks: ["t"], submixes: [] }),
    createContext: () => ({
      sampleRate: 48_000,
      renderQuantumSize: QUANTUM,
      state: "running",
      audioWorklet: { async addModule() {} },
      async close() {},
    }),
    createHost: async (request) => {
      harness.hostOptions = request.options;
      return host;
    },
  });
  afterCall?.();
  return { engine: await pending, render, harness };
}

/** Render while `promise` is pending, one block per turn, and return what it settles to. */
async function rendering(promise, render, blocks = 8 * WINDOW_BLOCKS) {
  let done = false;
  promise.then(() => { done = true; }, () => { done = true; });
  for (let index = 0; index < blocks && !done; index += 1) {
    render();
    await new Promise((resolve) => setTimeout(resolve, 1));
  }
  return promise;
}

/** Render a window and pump until a ready notification; every notification goes to `seen`. */
async function nextReady(subscription, render, seen = []) {
  for (let attempt = 0; attempt < 8; attempt += 1) {
    render(WINDOW_BLOCKS);
    const notification = await subscription.pump();
    if (notification === undefined) continue;
    seen.push(notification);
    if (notification.status === "ready") return notification;
  }
  throw new Error(`the stream published no ready window: ${seen.map((row) => `${row.status}/${row.metadata?.captureEpoch}/${row.metadata?.sequence}`).join(",")}`);
}

test("the browser boot sends the policy's hop in its SPECTRUM_CONFIGURE, not in a boot word", SKIP, async (t) => {
  // Red if the hop reaches neither the boot configure nor the stream (the boot words no longer
  // carry it), if a boot word still carries it (the worklet would call the hop boot export), if a
  // policy changed after `createEngine` was called changes it, or if a published metadata's nested
  // target can be changed.
  t.mock.timers.enable({ apis: ["setInterval"] });
  const policy = { spectrumHopFrames: 256 };
  const { engine, render, harness } = await served({
    spectrum: OUTPUT,
    policy,
    afterCall: () => { policy.spectrumHopFrames = 1_024; },
  });
  try {
    assert.equal(harness.hostOptions.spectrumHopFrames, undefined, "the boot words carry no hop");
    const subscription = await engine.subscribeSpectrum({ ...OUTPUT, smoothingMs: 0, cadenceMs: 1 });
    assert.equal(subscription.bounds.hopFrames, 256);
    const ready = await nextReady(subscription, render);
    assert.equal(ready.metadata.hopFrames, 256);
    assert.throws(() => { ready.metadata.target.outputId = "changed"; }, TypeError);
    await subscription.close();
  } finally {
    await engine.close();
  }
});

test("a run-time configure that adds a tap keeps the stream on the kept tap (#1596 gate 4)", SKIP, async (t) => {
  // Red if `configureSpectrum` restarts or ends a stream on a tap the new configuration keeps:
  // the stream would answer `warming`, `gap` or `failed`, or a new capture epoch, after it.
  t.mock.timers.enable({ apis: ["setInterval"] });
  // The core keeps a stream across a configure within the collection form (#1594 D6); a change
  // between the single form and a collection ends it.
  const { engine, render } = await served({
    spectrumCollection: { entries: [OUTPUT], maximumCaptureBytes: BOTH.maximumCaptureBytes },
  });
  try {
    const subscription = await engine.subscribeSpectrum({ ...OUTPUT, smoothingMs: 0, cadenceMs: 1 });
    const before = await nextReady(subscription, render);
    const configured = await engine.configureSpectrum(BOTH);
    assert.equal(configured.rebuild, true);
    assert.ok(configured.configuredEpoch > configured.adoptedEpoch);
    const seen = [];
    const after = await nextReady(subscription, render, seen);
    assert.deepEqual(seen.map((row) => row.status), ["ready"], "the kept stream went on publishing");
    assert.equal(after.metadata.captureEpoch, before.metadata.captureEpoch);
    assert.ok(after.metadata.sequence > before.metadata.sequence);
    await subscription.close();
  } finally {
    await engine.close();
  }
});

test("a one-shot query on a collection selects its entry first", SKIP, async () => {
  // Red if `querySpectrum` arms without selecting the queried entry: the window would be the other
  // tap's, or the core would refuse the arm with no entry selected.
  const { engine, render } = await served({ spectrumCollection: BOTH });
  try {
    for (const query of [TRACK, OUTPUT]) {
      const result = await rendering(engine.querySpectrum(query), render);
      assert.deepEqual(result.target, query.target);
      assert.equal(result.windowFrames, 2_048);
    }
  } finally {
    await engine.close();
  }
});

test("a one-shot query past its deadline cancels its arm behind it, so the next query arms", SKIP, async () => {
  // Red if a query that stopped waiting leaves its arm standing: the next arm is then refused
  // `spectrum.busy`. Red too if a single-form query of another target is armed: an arm names no
  // target, so its window would be the configured tap's under the other target's name.
  const { engine, render, harness } = await served({ spectrum: OUTPUT });
  try {
    await assert.rejects(engine.querySpectrum(TRACK), /not the configured tap/);
    let release;
    harness.hold = new Promise((resolve) => { release = resolve; });
    await assert.rejects(engine.querySpectrum({ ...OUTPUT, spectrumLimits: { requestDeadlineMs: 10 } }), /deadline/);
    release();
    const result = await rendering(engine.querySpectrum(OUTPUT), render);
    assert.deepEqual(result.target, OUTPUT.target);
  } finally {
    await engine.close();
  }
});

test("a stream read past its deadline stops the stream behind it and ends the subscription", SKIP, async (t) => {
  // Red if a stream frame that timed out leaves the stream running (the next start is refused) or
  // leaves the subscription live.
  t.mock.timers.enable({ apis: ["setInterval"] });
  const { engine, render, harness } = await served({ spectrum: OUTPUT });
  const request = { ...OUTPUT, smoothingMs: 0, cadenceMs: 1, spectrumLimits: { requestDeadlineMs: 20 } };
  try {
    const failed = await engine.subscribeSpectrum(request);
    let release;
    harness.hold = new Promise((resolve) => { release = resolve; });
    await assert.rejects(failed.pump(), /deadline/);
    release();
    await assert.rejects(failed.pump(), /closed|stale/);
    const replacement = await engine.subscribeSpectrum(request);
    assert.equal((await nextReady(replacement, render)).status, "ready");
    await replacement.close();
  } finally {
    await engine.close();
  }
});

test("closing the last spectrum subscription waits for the owner's automatic read before it stops", SKIP, async (t) => {
  // Red if `close()` sends its stop while the owner's automatic read of the stream is in flight.
  t.mock.timers.enable({ apis: ["setInterval"] });
  const { engine, harness } = await served({ spectrum: OUTPUT });
  try {
    const handle = await engine.subscribeSpectrum({ ...OUTPUT, smoothingMs: 0, cadenceMs: 60_000 });
    let release;
    harness.hold = new Promise((resolve) => { release = resolve; });
    // The owner's timer runs at the stream's capture cadence (2048 frames at 48 kHz): one tick is
    // one automatic read, whose reply is held.
    const before = harness.frames;
    t.mock.timers.tick(43);
    await flush();
    assert.equal(harness.frames, before + 1, "the automatic read was sent");
    const sent = harness.frames;
    let closed = false;
    const closing = handle.close().then(() => { closed = true; });
    await flush();
    assert.equal(harness.frames, sent, "no frame while the read is in flight");
    assert.equal(closed, false);
    release();
    await closing;
    assert.equal(harness.frames, sent + 1, "one stop after the read");
    await assert.rejects(handle.pump(), /closed|stale/);
  } finally {
    await engine.close();
  }
});
