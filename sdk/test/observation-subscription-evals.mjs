/** Issue #783: one bounded managed resident-observation lifetime; #1589b: on the core's frames. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { constantValue } from "../src/core/abi.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { decodeObservationReadReply, encodeObservationRead } from "../src/core/control-frame.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
import { effect, session } from "../src/core/session.ts";
import { encodeSessionTransaction, transaction } from "../src/core/session-transaction.ts";
import { createEngine } from "../src/browser/engine.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { onEndpointIds } from "./control-support.mjs";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function observationDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  const gate = CATALOG.effects.find((row) => row.id === "miso.gate-expander");
  assert.ok(compressor && gate);
  return sessionDocument({
    effects: {
      inserts: [
        effectEntry("comp", compressor.id, compressor.parameters.map((row) => ({
          id: row.id, unit: row.unitName, value: row.default, channel: "both",
        }))),
        effectEntry("gate", gate.id, gate.parameters.map((row) => ({
          id: row.id, unit: row.unitName, value: row.default, channel: "both",
        }))),
      ],
    },
  });
}

function selection(effectSlotId, channels = "both") {
  return { trackId: "t", rack: "inserts", effectSlotId, tapId: 1, channels };
}

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    const planes = Array.from({ length: source.channels }, (_unused, channel) =>
      ramp(shape.quantumFrames, 17 + block * 11 + channel));
    assert.equal(engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes,
      endOfRegion: false,
    }).ok, true);
  }
}

/**
 * An owner over a scripted frame transport that shares arms as the core does (#1584 D3): an arm of
 * an unarmed tap issues a fresh arm tag, an arm of an armed tap adds a subscriber and reports its
 * tag, and a disarm removes one and drops the tap with the last; `read` reports a ready window with
 * the tap's tag while it is armed.
 */
function injectedOwner() {
  let sequence = 1n;
  let timer;
  let applyHook;
  let readHook;
  let mapHook;
  let rowsHook;
  let readCount = 0;
  let applyCount = 0;
  let nextTag = 0n;
  const tags = new Map();
  const subscribers = new Map();
  const map = {
    bindings: [{
      trackId: "t", rack: "inserts", effectSlotId: "comp",
      nativeEffectId: "miso.compressor", tapIds: [1],
    }],
  };
  const owner = new ObservationSubscriptionOwner({
    observationMap: () => mapHook === undefined ? map : mapHook(),
    async apply(edits) {
      applyCount += 1;
      if (applyHook) await applyHook();
      const issued = edits.map((edit) => {
        const held = tags.get(edit.effectId);
        if (edit.armed) {
          if (held !== undefined) {
            subscribers.set(edit.effectId, subscribers.get(edit.effectId) + 1);
            return held;
          }
          nextTag += 1n;
          tags.set(edit.effectId, nextTag);
          subscribers.set(edit.effectId, 1);
          return nextTag;
        }
        const left = (subscribers.get(edit.effectId) ?? 1) - 1;
        if (left > 0) {
          subscribers.set(edit.effectId, left);
          return held;
        }
        nextTag += 1n;
        tags.delete(edit.effectId);
        subscribers.delete(edit.effectId);
        return nextTag;
      });
      return { revision: 1n, observeArmTags: issued, removedArmTags: [] };
    },
    async read(selections) {
      readCount += 1;
      if (readHook) await readHook();
      if (rowsHook !== undefined) return { observedSample: 128n, rows: rowsHook(selections) };
      return {
        observedSample: 128n,
        rows: selections.map((selection) => {
          const tag = tags.get(selection.effectId);
          return tag === undefined || selection.effectId === "gate"
            ? { status: "unarmed", armed: false, armSample: 0n, armTag: 0n, window: undefined, left: undefined, right: undefined, unit: 1 }
            : {
              status: "ready", armed: true, armSample: 0n, armTag: tag,
              window: { firstSample: 0n, endSample: 128n, sequence, blocks: 1 }, left: 1, right: 2, unit: 1,
            };
        }),
      };
    },
    sampleRateHz: () => 48_000,
    scheduler: {
      setInterval: (callback) => { timer = callback; return 1; },
      clearInterval: () => { timer = undefined; },
    },
  });
  return {
    owner,
    map,
    fire: () => timer?.(),
    setSequence: (value) => { sequence = value; },
    setApply: (hook) => { applyHook = hook; },
    setRead: (hook) => { readHook = hook; },
    setMap: (hook) => { mapHook = hook; },
    setRows: (hook) => { rowsHook = hook; },
    readCount: () => readCount,
    applyCount: () => applyCount,
  };
}

const injectedSelection = selection("comp");

/** The `SESSION_TRANSACTION_APPLY` frames a headless engine sends, by their edit count. */
function transactionsOf(engine) {
  const sent = [];
  const controlFrame = engine.controlFrame.bind(engine);
  engine.controlFrame = (frame) => {
    const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
    if (view.getUint16(16, true) === 0x0003) sent.push(view.getUint32(40, true));
    return controlFrame(frame);
  };
  return sent;
}

describe("issue 783 -- managed resident observation subscriptions", () => {
  // #1590 gate 1: the SDK sends one arm per selection of each handle and keeps no reference count;
  // the core shares the arm. Red if the SDK still counts references (the second subscribe sends no
  // edit, or the first close sends none) or if the core does not share the arm (the second arm
  // gives the tap another tag, or the first close silences the tap the second handle holds).
  test("two handles on one tap and window share the core's one arm; the last close disarms", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    const sent = transactionsOf(engine);
    const compLeft = selection("comp", "left");
    const gateRight = selection("gate", "right");
    try {
      let notifications = 0;
      const first = await engine.subscribeObservations({
        selections: [compLeft, gateRight],
        windowBlocks: 2,
        onUpdate: () => { notifications += 1; },
      });
      assert.deepEqual(sent, [2], "one arm per selection");
      assert.equal(first.configuration.windowBlocks, 2);
      assert.deepEqual(first.readLatest().map((row) => row.status), ["pending", "pending"]);

      feed(engine, 0); engine.render(); await first.pump();
      feed(engine, 1); engine.render(); await first.pump();
      const saved = structuredClone(first.readLatest());
      assert.ok(notifications >= 1);
      assert.deepEqual(saved.map((row) => [row.status, row.channels]), [
        ["ready", "left"], ["ready", "right"],
      ]);
      assert.ok(saved[0].window && saved[0].window.sequence > 0n);
      const tag = coreRow(engine, "comp").armTag;
      const second = await engine.subscribeObservations({
        selections: [selection("comp", "right"), selection("gate", "left")],
        windowBlocks: 2,
      });
      assert.deepEqual(sent, [2, 2], "the second handle sends its own arms");
      assert.equal(coreRow(engine, "comp").armTag, tag, "the core holds one arm: one tag");
      assert.equal(second.readLatest().length, 2);
      assert.equal(second.readLatest()[0].channels, "right");

      await assert.rejects(
        () => engine.subscribeObservations({ selections: [compLeft], windowBlocks: 3 }),
        (error) => error instanceof MisoUsageError
          && /^observation window conflicts with a managed subscriber/.test(error.message)
          && error.diagnosticCode === "observation.window_conflict",
      );
      await assert.rejects(
        () => first.update({ selections: [selection("missing")], windowBlocks: 2 }),
        /unavailable|unsupported/,
      );
      assert.equal(first.configuration.selections[0].effectSlotId, "comp");
      assert.equal(first.readLatest()[0].window.sequence, saved[0].window.sequence);

      await first.close();
      assert.deepEqual(sent.slice(-1), [2], "the first close sends one disarm per selection");
      feed(engine, 2); engine.render();
      feed(engine, 3); engine.render();
      await second.pump();
      const kept = second.readLatest();
      assert.equal(kept[0].status, "ready");
      assert.ok(kept[0].window.sequence > saved[0].window.sequence, "the other handle's rows keep flowing");
      assert.equal(coreRow(engine, "comp").armTag, tag);
      await second.close();
      feed(engine, 4); engine.render();
      assert.equal(engine.readObservations([compLeft])[0].status, "unarmed");
      await second.close();
    } finally {
      engine.dispose();
    }
  });

  // #1345 gate 6 and #1590 gate 2, on the core's frames: a window change on a tap one subscriber
  // holds is the disarm of the old window and the arm of the new one in one transaction, which the
  // core applies as one re-arm. Red if the update sends the arm alone (the core refuses
  // `observation.window_conflict`), sends two transactions or two arms, or sends a disarm that
  // silences the tap.
  test("a window change on a one-tap effect is one re-arm transaction and lands", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    const sent = transactionsOf(engine);
    try {
      const handle = await engine.subscribeObservations({
        selections: [selection("comp", "both")], windowBlocks: 4,
      });
      for (let block = 0; block < 4; block += 1) { feed(engine, block); engine.render(); }
      await handle.pump();
      assert.equal(handle.readLatest()[0].status, "ready");
      assert.equal(handle.readLatest()[0].window.blocks, 4);
      const before = sent.length;
      const oldTag = coreRow(engine, "comp").armTag;

      const receipt = await handle.update({ selections: [selection("comp", "both")], windowBlocks: 8 });
      assert.deepEqual(sent.slice(before), [2], "the update is one transaction of a disarm and an arm");
      assert.equal(receipt.configuration.windowBlocks, 8);
      feed(engine, 4); engine.render();
      const rearmed = coreRow(engine, "comp");
      assert.equal(rearmed.armed, true, "the re-arm leaves the tap armed");
      assert.equal(rearmed.armTag, oldTag + 1n, "one new tag: one re-arm record");
      for (let block = 5; block < 13; block += 1) { feed(engine, block); engine.render(); }
      await handle.pump();
      const reading = handle.readLatest()[0];
      assert.equal(reading.status, "ready");
      assert.equal(reading.window.blocks, 8);
      assert.ok(reading.window.firstSample >= await receipt.armed());
      await handle.close();
    } finally {
      engine.dispose();
    }
  });

  // #1345 gate 6 bullet 3, headless. The engine renders only when the caller does, so a managed
  // submission whose tap slot an earlier submission still holds is refused at once with the typed,
  // retryable `backpressure` error. It does not wait for a render that only the caller can make.
  async function refusedAtOnce(operation) {
    const outcome = await Promise.race([
      operation().then(() => ({ settled: "resolved" }), (error) => ({ settled: "rejected", error })),
      new Promise((resolve) => setTimeout(() => resolve({ settled: "pending" }), 1_000)),
    ]);
    assert.equal(outcome.settled, "rejected", "the refusal is immediate, not a wait for a render");
    assert.ok(outcome.error instanceof MisoEngineError);
    assert.equal(outcome.error.code, "backpressure");
    return outcome.error;
  }

  test("an awaited close before any render is refused with typed backpressure; a render and a retry succeed", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    try {
      const handle = await engine.subscribeObservations({
        selections: [selection("comp", "both")], windowBlocks: 2,
      });
      // The exact sequence that hung: `await handle.close()` before the caller renders.
      await refusedAtOnce(() => handle.close());
      // The refusal admitted nothing and released the owner: the handle is open and another
      // owner operation is not "already in flight".
      await handle.pump();
      feed(engine, 0); engine.render();
      assert.notEqual(engine.readObservations([selection("comp", "both")])[0].status, "unarmed",
        "the refused close did not disarm the tap");
      await handle.close();
      feed(engine, 1); engine.render();
      assert.equal(engine.readObservations([selection("comp", "both")])[0].status, "unarmed");
    } finally {
      engine.dispose();
    }
  });

  test("a subscribe or update before the slot is drained is refused with typed backpressure and changes nothing", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    const comp = selection("comp", "both");
    try {
      const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
      feed(engine, 0); engine.render();
      // The update's record takes the slot; a second operation before a render is refused.
      await handle.update({ selections: [comp], windowBlocks: 4 });
      await refusedAtOnce(() => handle.update({ selections: [comp], windowBlocks: 6 }));
      assert.equal(handle.configuration.windowBlocks, 4, "a refused update leaves the handle as it was");
      await refusedAtOnce(() => handle.close());
      feed(engine, 1); engine.render();
      // Retried after a render, the same update goes through.
      const receipt = await handle.update({ selections: [comp], windowBlocks: 6 });
      assert.equal(receipt.configuration.windowBlocks, 6);
      feed(engine, 2); engine.render();
      await handle.close();
      // The close's disarm now holds the slot, so a subscribe before the next render is refused too.
      await refusedAtOnce(() => engine.subscribeObservations({ selections: [comp], windowBlocks: 2 }));
      feed(engine, 3); engine.render();
      assert.equal(engine.readObservations([comp])[0].status, "unarmed");
      const again = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
      feed(engine, 4); engine.render();
      assert.notEqual(engine.readObservations([comp])[0].status, "unarmed");
      assert.equal(again.configuration.windowBlocks, 2);
    } finally {
      engine.dispose();
    }
  });

  test("invalidates a handle at headless replacement", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    const sub = await engine.subscribeObservations({ selections: [selection("comp")], windowBlocks: 1 });
    try {
      assert.throws(() => engine.loadSession("{}"));
      assert.throws(() => sub.readLatest(), /stale|closed/);
      await assert.rejects(() => sub.pump(), /stale|closed/);
    } finally {
      engine.dispose();
    }
  });

  test("serializes internal edits and reads, honors cadence, and baselines joining loss", async (t) => {
    const harness = injectedOwner();
    let releaseSubmit;
    harness.setApply(() => new Promise((resolve) => { releaseSubmit = resolve; }));
    const pending = harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    releaseSubmit();
    await pending;

    const overlapHarness = injectedOwner();
    const overlap = (await overlapHarness.owner.subscribe({
      selections: [injectedSelection], windowBlocks: 1,
    })).handle;
    const readReleases = [];
    overlapHarness.setRead(() => new Promise((resolve) => { readReleases.push(resolve); }));
    const pumping = overlap.pump();
    await new Promise((resolve) => setTimeout(resolve, 0));
    overlapHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(readReleases.length, 1);
    readReleases.forEach((resolve) => resolve());
    await pumping;
    await overlap.close();

    const mutationHarness = injectedOwner();
    mutationHarness.map.bindings.push({
      trackId: "t", rack: "inserts", effectSlotId: "gate",
      nativeEffectId: "miso.compressor", tapIds: [1],
    });
    const managed = (await mutationHarness.owner.subscribe({
      selections: [injectedSelection], windowBlocks: 1,
    })).handle;
    // A timer tick while the update's transaction is in flight reads nothing.
    let releaseApply;
    mutationHarness.setApply(() => new Promise((resolve) => { releaseApply = resolve; }));
    const updating = managed.update({ selections: [injectedSelection, selection("gate")], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    const readsDuringApply = mutationHarness.readCount();
    mutationHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(mutationHarness.readCount(), readsDuringApply);
    mutationHarness.setApply(undefined);
    releaseApply();
    await updating;
    await managed.close();

    // The scheduler is manual; its cadence clock must also be controlled rather
    // than assuming three setTimeout(0) turns finish within ten milliseconds.
    let now = 0;
    const cadenceClock = t.mock.method(Date, "now", () => now);
    const fastHarness = injectedOwner();
    let fast = 0;
    let slow = 0;
    await fastHarness.owner.subscribe({
      selections: [injectedSelection], windowBlocks: 1, cadenceMs: 10, onUpdate: () => { fast += 1; },
    });
    await fastHarness.owner.subscribe({
      selections: [injectedSelection], windowBlocks: 1, cadenceMs: 1_000, onUpdate: () => { slow += 1; },
    });
    for (let index = 1; index <= 3; index += 1) {
      fastHarness.setSequence(BigInt(index));
      fastHarness.fire();
      await new Promise((resolve) => setTimeout(resolve, 0));
    }
    assert.equal(fast, 1);
    assert.equal(slow, 1);
    now = 9;
    fastHarness.setSequence(4n);
    fastHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(fast, 1);
    assert.equal(slow, 1);
    now = 10;
    fastHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(fast, 2);
    assert.equal(slow, 1);
    now = 1_000;
    fastHarness.setSequence(5n);
    fastHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(fast, 3);
    assert.equal(slow, 2);
    cadenceClock.mock.restore();

    const lossHarness = injectedOwner();
    const first = (await lossHarness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle;
    await first.pump();
    lossHarness.setSequence(10n);
    await first.pump();
    lossHarness.setSequence(11n);
    const next = await first.pump();
    assert.equal(next.nativeMissedWindows, 0n);
    const joining = (await lossHarness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle;
    const notification = await joining.pump();
    assert.equal(notification.nativeMissedWindows, 0n);
    await first.close();
    await joining.close();

    const epochHarness = injectedOwner();
    let releaseMap;
    epochHarness.setMap(() => new Promise((resolve) => { releaseMap = resolve; }));
    const stale = epochHarness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    epochHarness.owner.invalidate();
    releaseMap(epochHarness.map);
    await assert.rejects(stale, /owner changed/);
    assert.equal(epochHarness.applyCount(), 0);
  });

  test("retries a refused close", async () => {
    const harness = injectedOwner();
    const sub = (await harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle;
    harness.setApply(() => { throw new Error("temporary refusal"); });
    await assert.rejects(() => sub.close(), /temporary refusal/);
    harness.setApply(undefined);
    await sub.close();
    assert.equal(harness.applyCount(), 3);
  });
});

/** Whether `promise` has settled, after every queued reaction ran. */
async function settled(promise) {
  let done = false;
  promise.then(() => { done = true; }, () => { done = true; });
  await new Promise((resolve) => setImmediate(resolve));
  return done;
}

const QUANTUM = 128;
const SOURCE = { channels: 2, bitDepth: 24, frames: 48_000, content: `blake3:${"a".repeat(64)}` };
const compressor = () => effect("miso.compressor", {}, { slotId: "comp" });

/** Tracks on one source, each with a compressor insert `comp` and a route to the output. */
function compSession(tracks = ["t"]) {
  let builder = session({ id: "observe.frames", sampleRateHz: 48_000, quantumFrames: QUANTUM }).source("a", SOURCE);
  for (const id of tracks) builder = builder.track(id, { source: "a", inserts: [compressor()] });
  builder = builder.output("out");
  for (const id of tracks) {
    builder = builder.route({
      id: `${id}.main`,
      source: { kind: "track", trackId: id, tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
  }
  return builder;
}

/** One `OBSERVATION_READ` of the tap `effectId`/1 on track `t`, sent through the engine's own entry. */
function coreRow(engine, effectId) {
  const reply = decodeObservationReadReply(engine.controlFrame(encodeObservationRead(
    engine.controlTransactions().requestIds.next(),
    [{ stripId: "t", rack: "inserts", effectId, tapId: 1, channels: "both" }],
  )));
  assert.equal(reply.ok, true);
  return reply.value.rows[0];
}

function hexOf(bytes) {
  return Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength).toString("hex");
}

/**
 * The browser engine (`createEngine`) over a host whose `control` message runs the real core in
 * process: the core's own `Observe`, backpressure and `OBSERVATION_READ` answer every frame, and the
 * context renders when the test calls `render()`. No browser runs here; the real worklet legs are
 * #1530's and #1535's qualification rows. `inject` may answer an `Observe` transaction instead of
 * the core, and `log` records each transaction the browser sent: the render count when it was sent,
 * its call result, and its payload (the edits, without the header's request ID and revision).
 */
async function coreRig({ document = compSession() } = {}) {
  const text = typeof document === "string" ? document : document.toJson();
  const core = await WasmBoundary.boot(asset, new TextEncoder().encode(text), {
    liveControls: { meterBlocks: 0 },
  });
  const log = [];
  const statusCalls = { count: 0 };
  const forward = onEndpointIds(core.controlRequestIds, (frame) => core.controlFrame(frame));
  const call = (work) => {
    try {
      return { result: 0, response: work() };
    } catch (error) {
      if (typeof error?.result !== "number") throw error;
      const lines = error.diagnostics.map((row) => `${row.code}\t${row.path}\n`).join("");
      return { result: error.result, response: new TextEncoder().encode(lines) };
    }
  };
  const rig = {
    core,
    log,
    statusCalls,
    inject: undefined,
    render() { core.render(QUANTUM); },
  };
  const host = {
    async control(frame) {
      const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
      const transaction = view.getUint16(16, true) === 0x0003;
      const at = core.renderedQuanta();
      const answer = (transaction ? rig.inject?.(frame) : undefined) ?? call(() => forward(frame));
      if (transaction) log.push({ at, result: answer.result, payload: hexOf(frame.subarray(48)) });
      return { tag: "miso.control.v1", requestId: 1, ...answer, revision: core.controlRevision() };
    },
    async sessionMap() {
      const map = core.sessionMap();
      return { tag: "miso.sessionmap.v1", requestId: 1, result: 0, sources: map.sources, metersAttached: map.metersAttached };
    },
    async status() {
      statusCalls.count += 1;
      return { tag: "miso.status.v1", result: 0, state: 2, renderedQuanta: core.renderedQuanta() };
    },
    onControlEvent: (listener) => core.onControlEvent(listener),
    async dispose() { core.dispose(); },
  };
  rig.browser = (extra = {}) => createEngine({
    document: text,
    scratchBoot: async () => core.shape(),
    createContext: () => ({
      sampleRate: 48_000, renderQuantumSize: QUANTUM, state: "suspended",
      audioWorklet: { async addModule() {} }, async close() {},
    }),
    createHost: async () => host,
    policy: { liveControls: { meterBlocks: 0 } },
    ...extra,
  });
  return rig;
}

const BACKPRESSURE = constantValue("resultCodes", "backpressure");

/** A failed call as the core answers a full tap queue: the backpressure result and its diagnostic. */
const backpressureCall = () => ({
  result: BACKPRESSURE,
  response: new TextEncoder().encode("observation.backpressure\t\n"),
});

// #1345 gate 6 bullet 3, browser, on the core's frames (#1589b). An `Observe` of a tap whose queue
// slot an earlier `Observe` holds is refused with backpressure until a render drains it.
describe("issue 1345 -- the browser engine waits for a render, within a bound", () => {
  const request = { selections: [{ trackId: "t", rack: "inserts", effectSlotId: "comp", tapId: 1, channels: "both" }], windowBlocks: 2 };

  test("a close whose slot is taken waits for the context to render and goes through", async () => {
    const rig = await coreRig();
    const engine = await rig.browser();
    try {
      const handle = await engine.subscribeObservations(request);
      const closing = handle.close();
      setTimeout(() => rig.render(), 30);
      await closing;
      // Subscribe, the refused close, and the same close sent again after the render.
      assert.deepEqual(rig.log.map((entry) => [entry.at, entry.result]), [[0n, 0], [0n, BACKPRESSURE], [1n, 0]]);
      assert.equal(rig.log[1].payload, rig.log[2].payload, "the resend carries the same edits");
    } finally {
      await engine.close();
    }
  });

  // Red if the full frame of armed taps (the core's resource limit) loses its named diagnostic and
  // the advice to unsubscribe another tap, or is resent as if a render made room.
  test("a full frame of armed taps is refused observationLimit at once and never resent", async () => {
    const rig = await coreRig();
    const engine = await rig.browser({ requestDeadlineMs: 2_000 });
    try {
      rig.inject = () => ({
        result: constantValue("resultCodes", "compileRejected"),
        response: new TextEncoder().encode("capi.resource.limit\t$\n"),
      });
      await assert.rejects(engine.subscribeObservations(request), (error) =>
        error instanceof MisoEngineError && error.diagnosticCode === "observationLimit"
        && /unsubscribe another tap/.test(error.message));
      assert.equal(rig.log.length, 1, "the one refused subscription, no resend");
    } finally {
      await engine.close();
    }
  });

  test("a suspended context ends the wait in typed backpressure at the deadline, and the polling stops", async () => {
    const rig = await coreRig();
    const engine = await rig.browser({ requestDeadlineMs: 80 });
    try {
      const handle = await engine.subscribeObservations(request);
      const started = Date.now();
      const outcome = await Promise.race([
        handle.close().then(() => "resolved", (error) => error),
        new Promise((resolve) => setTimeout(() => resolve("still waiting"), 2_000)),
      ]);
      assert.ok(outcome instanceof MisoEngineError, `ended with ${String(outcome)}`);
      assert.equal(outcome.code, "backpressure");
      assert.ok(Date.now() - started >= 70, "it waited for the context before it gave up");
      // The wait left nothing polling the host.
      const calls = rig.statusCalls.count;
      await new Promise((resolve) => setTimeout(resolve, 60));
      assert.equal(rig.statusCalls.count, calls);
      // Nothing was admitted by the refusal; once the context renders the same close goes through.
      rig.render();
      await handle.close();
    } finally {
      await engine.close();
    }
  });

  test("renders that never free the slot end the wait at one deadline from the first refusal", async () => {
    // A render lands every 60 ms and another writer takes the slot again after each one. A deadline
    // fixed per resend would keep the wait going to the cap of 8 resends, about 480 ms here; one
    // deadline from the first refusal ends it at 150 ms.
    const rig = await coreRig();
    const engine = await rig.browser({ requestDeadlineMs: 150 });
    try {
      const handle = await engine.subscribeObservations(request);
      rig.inject = backpressureCall;
      const ticker = setInterval(() => rig.render(), 60);
      const started = Date.now();
      try {
        await assert.rejects(() => handle.close(), (error) => {
          assert.ok(error instanceof MisoEngineError);
          assert.equal(error.code, "backpressure");
          return true;
        });
      } finally {
        clearInterval(ticker);
      }
      const elapsed = Date.now() - started;
      assert.ok(elapsed >= 140, `it waited for the deadline (${elapsed} ms)`);
      assert.ok(elapsed < 300, `one deadline spans every resend (${elapsed} ms)`);
    } finally {
      await engine.close();
    }
  });

  test("closing the engine ends a pending wait at once with the stale-owner error", async () => {
    const rig = await coreRig();
    const engine = await rig.browser({ requestDeadlineMs: 3_000 });
    const handle = await engine.subscribeObservations(request);
    const closing = handle.close();
    const started = Date.now();
    setTimeout(() => { void engine.close(); }, 40);
    const outcome = await Promise.race([
      closing.then(() => "resolved", (error) => error),
      new Promise((resolve) => setTimeout(() => resolve("still waiting"), 1_500)),
    ]);
    assert.ok(outcome instanceof Error, `ended with ${String(outcome)}`);
    assert.match(outcome.message, /owner changed/);
    assert.ok(Date.now() - started < 1_000, "the wait did not run to its deadline");
    await engine.close();
  });

  // #1590 D1: the core's `observation.not_subscribed` reaches the caller as `MisoUsageError` with
  // the core's code. Red if a refusal that is not backpressure waits for a render or is resent, or
  // if the subscriber refusal reaches the caller as an engine failure.
  test("a refusal that is not backpressure rejects at once with the core's code and is never sent again", async () => {
    const rig = await coreRig();
    const engine = await rig.browser({ requestDeadlineMs: 2_000 });
    try {
      const handle = await engine.subscribeObservations(request);
      rig.inject = () => {
        rig.inject = undefined;
        return {
          result: constantValue("resultCodes", "invalidArgument"),
          response: new TextEncoder().encode("observation.not_subscribed\tedits[0]\n"),
        };
      };
      const started = Date.now();
      await assert.rejects(() => handle.close(), (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.equal(error.diagnosticCode, "observation.not_subscribed");
        return true;
      });
      assert.ok(Date.now() - started < 500, "a refusal that is not backpressure does not wait for a render");
      assert.equal(rig.log.length, 2, "the subscribe and the one refused close, no resend");
    } finally {
      await engine.close();
    }
  });
});

describe("#1589 gate 2 -- armed() settles on the arm render applied", () => {
  const comp = { trackId: "t", rack: "inserts", effectSlotId: "comp", tapId: 1, channels: "both" };

  // Red if `armed()` resolves at the reply (before any render applied the arm), or with another
  // sample than the arm sample the core's own read reports.
  test("armed() stays pending until a render applies the arm and resolves with the read's arm sample", async () => {
    const engine = await createOfflineEngine(compSession(), { asset });
    try {
      const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
      assert.equal(engine.renderedQuanta(), 0n, "subscribe returned before any render");
      const armed = handle.armed();
      await handle.pump();
      assert.equal(await settled(armed), false, "a read before the render applied the arm leaves it pending");
      engine.render();
      await handle.pump();
      const row = coreRow(engine, "comp");
      assert.equal(row.armed, true);
      assert.equal(await armed, row.armSample);
    } finally {
      engine.dispose();
    }
  });

  // Red if `armed()` takes any armed row (another arm's tag) as its own, if a window of another arm
  // is given to the subscription, or if a newer tag on a pending binding is ignored.
  test("a row of another arm's tag never resolves armed() or reaches readLatest(), and a newer tag rejects it superseded", async () => {
    const harness = injectedOwner();
    // An earlier subscription of the tap takes tags 1 (its arm) and 2 (its close's disarm).
    await (await harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle.close();
    const handle = (await harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle;
    const armed = handle.armed();
    let reported = 1n;
    harness.setRows((selections) => selections.map(() => ({
      status: "ready", armed: true, armSample: 512n, armTag: reported,
      window: { firstSample: 512n, endSample: 640n, sequence: 9n, blocks: 1 }, left: 1, right: 2, unit: 1,
    })));
    await handle.pump();
    assert.equal(await settled(armed), false, "an armed row of another tag is not this arm");
    assert.equal(handle.readLatest()[0].status, "pending", "a window of an earlier arm is never given to it");
    assert.equal(handle.readLatest()[0].window, undefined);
    reported = 4n;
    await handle.pump();
    await assert.rejects(armed, (error) => {
      assert.ok(error instanceof MisoEngineError);
      assert.equal(error.code, "internal");
      assert.equal(error.diagnosticCode, "sdk.observation.superseded");
      return true;
    });
  });

  // Red if a close before the arm applied leaves `armed()` pending for ever (or resolves it).
  test("closing before the arm applied rejects armed() with sdk.observation.closed", async () => {
    const engine = await createOfflineEngine(compSession(), { asset });
    try {
      const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
      const armed = handle.armed();
      engine.render();
      await handle.close();
      await assert.rejects(armed, (error) => error instanceof MisoEngineError && error.code === "wrongState"
        && error.diagnosticCode === "sdk.observation.closed");
    } finally {
      engine.dispose();
    }
  });

  // Red if a disposal leaves a pending `armed()` pending, or names another cause.
  test("disposing the engine rejects a pending armed() with sdk.engine.disposed", async () => {
    const engine = await createOfflineEngine(compSession(), { asset });
    const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
    const armed = handle.armed();
    engine.dispose();
    await assert.rejects(armed, (error) => error instanceof MisoEngineError && error.code === "wrongState"
      && error.diagnosticCode === "sdk.engine.disposed");
  });

  // Red if a commit that removes the effect does not settle a pending `armed()`, or settles it with
  // another cause.
  test("removing the effect in a transaction rejects armed() with sdk.observation.owner_removed", async () => {
    const engine = await createOfflineEngine(compSession(), { asset });
    try {
      const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
      const armed = handle.armed();
      engine.render();
      await engine.apply(transaction(compSession()).removeTrackEffect("t", "comp").build());
      await assert.rejects(armed, (error) => error instanceof MisoEngineError && error.code === "wrongState"
        && error.diagnosticCode === "sdk.observation.owner_removed");
      assert.deepEqual(handle.ended.map((row) => row.effectSlotId), ["comp"]);
    } finally {
      engine.dispose();
    }
  });
});

describe("#1589 gate 3 -- a subscription keeps reporting across a rebuild", () => {
  const comp = { trackId: "t", rack: "inserts", effectSlotId: "comp", tapId: 1, channels: "both" };
  const addTrack = () => transaction(compSession()).upsertTrack("t2", { source: "a", inserts: [compressor()] }).build();

  /** Subscribe, read a window, add a track (a rebuild), render on, and read again. */
  async function acrossRebuild(engine, render) {
    const handle = await engine.subscribeObservations({ selections: [comp], windowBlocks: 2 });
    for (let block = 0; block < 4; block += 1) render();
    await handle.pump();
    const before = handle.readLatest()[0];
    assert.equal(before.status, "ready");
    const outcome = await engine.apply(addTrack());
    assert.equal(outcome.path, "rebuild");
    for (let block = 0; block < 6; block += 1) render();
    await handle.pump();
    const after = handle.readLatest()[0];
    assert.equal(after.status, "ready", "the tap still reports after the rebuild");
    assert.ok(after.window.sequence > before.window.sequence, "its windows continue");
    assert.ok(after.window.firstSample > before.window.firstSample);
    assert.deepEqual(handle.ended, []);
    await handle.close();
  }

  // Red if the headless engine's subscription is dropped or restarted by a rebuild.
  test("headless", async () => {
    const engine = await createOfflineEngine(compSession(), { asset });
    try {
      await acrossRebuild(engine, () => engine.render());
    } finally {
      engine.dispose();
    }
  });

  // Red if the browser engine's subscription is dropped at a rebuild (the browser's own observation
  // records were never carried to a replacement plan).
  test("browser", async () => {
    const rig = await coreRig();
    const engine = await rig.browser();
    try {
      await acrossRebuild(engine, () => rig.render());
    } finally {
      await engine.close();
    }
  });
});

describe("#1589b -- the frame transport's resend and frame rules", () => {
  const tapOf = (trackId) => ({ trackId, rack: "inserts", effectSlotId: "comp", tapId: 1, channels: "both" });

  // Red if an `Observe` refused `revision_conflict` (another sender committed after the browser
  // queue last saw the revision) is thrown instead of sent again at the revision the refusal names.
  test("an Observe that another sender's commit made stale is sent again at the newer revision", async () => {
    const rig = await coreRig();
    const engine = await rig.browser();
    try {
      // The first subscription opens the browser's queue at the current revision.
      const first = await engine.subscribeObservations({ selections: [tapOf("t")], windowBlocks: 2 });
      rig.render();
      await first.close();
      rig.render();
      // Another sender commits between the browser's revision read and its send: the core sees a
      // structural edit first, then the browser's `Observe` at the revision before it.
      rig.inject = () => {
        rig.inject = undefined;
        rig.core.controlFrame(encodeSessionTransaction(
          transaction(compSession()).upsertTrack("t2", { source: "a", inserts: [compressor()] }).build(),
          { requestId: rig.core.controlRequestIds.next(), expectedRevision: rig.core.controlRevision() },
        ));
        return undefined;
      };
      const before = rig.log.length;
      const again = await engine.subscribeObservations({ selections: [tapOf("t")], windowBlocks: 2 });
      const sent = rig.log.slice(before);
      assert.equal(sent.length, 2, "the stale send and its resend");
      assert.equal(sent[0].payload, sent[1].payload, "the resend carries the same edits");
      for (let block = 0; block < 4; block += 1) rig.render();
      await again.pump();
      assert.ok(await again.armed() > 0n, "the resent arm is in effect");
      await again.close();
    } finally {
      await engine.close();
    }
  });

  // Red if a read of more bindings than one reply frame can carry (`64 + 192 n` bytes) is sent as
  // one read (the core refuses `observation.frame`), or if an `Observe` transaction over the frame
  // limit is sent instead of refused before it leaves the SDK.
  test("a read past one reply frame is split, and a transaction past the frame is refused before it is sent", async () => {
    const tracks = Array.from({ length: 100 }, (_unused, index) => `t${index}`);
    // The 16 KiB frame floor (#2191), so 100 taps outgrow one reply frame.
    const engine = await createOfflineEngine(compSession(tracks), { asset, maximumControlFrameBytes: 16n << 10n });
    const sent = transactionsOf(engine);
    try {
      const limit = engine.controlTransactions().maximumFrameBytes;
      assert.ok(limit !== undefined && Math.floor((limit - 64) / 192) < tracks.length, `the frame (${limit} bytes) holds fewer rows than taps`);
      await assert.rejects(
        engine.subscribeObservations({ selections: tracks.map(tapOf), windowBlocks: 2 }),
        (error) => error instanceof MisoUsageError && /one control frame holds/.test(error.message),
      );
      assert.deepEqual(sent, [], "nothing was sent");
      const handles = [];
      for (let start = 0; start < tracks.length; start += 50) {
        handles.push(await engine.subscribeObservations({
          selections: tracks.slice(start, start + 50).map(tapOf), windowBlocks: 2,
        }));
      }
      for (let block = 0; block < 4; block += 1) engine.render();
      await handles[0].pump();
      for (const handle of handles) {
        assert.ok(handle.readLatest().every((row) => row.status === "ready"));
      }
    } finally {
      engine.dispose();
    }
  });
});
