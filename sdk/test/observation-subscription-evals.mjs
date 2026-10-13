/** Issue #783: one bounded managed resident-observation lifetime. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
import { createEngine } from "../src/browser/engine.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
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

const FAKE_DESCRIPTOR = Object.freeze({
  id: 1, name: "Gain Reduction", displayUnit: "dB", unitName: "dB", subscribable: true,
});

function injectedOwner() {
  let armed = false;
  let sequence = 1n;
  let timer;
  let submitHook;
  let readHook;
  let liveControlHook;
  let readCount = 0;
  let submitCount = 0;
  const map = {
    bindings: [{
      trackId: "t", rack: "inserts", effectSlotId: "comp", effectIndex: 0,
      nativeEffectId: "miso.compressor", tapIds: [1],
    }],
  };
  const liveControls = {
    edit: {
      strip: () => ({
        effect: () => ({
          observe: (_tap, on) => ({ kind: on ? "observeSubscribe" : "observeUnsubscribe" }),
        }),
      }),
    },
    async submit(...edits) {
      submitCount += 1;
      if (submitHook) await submitHook();
      armed = edits.at(-1)?.kind === "observeSubscribe";
      return {
        ok: true, result: 0, code: "ok", reason: 0, reasonName: "none", rejectedIndex: 0,
        admitted: edits.length, appliedAtSample: 0n,
      };
    },
  };
  const owner = new ObservationSubscriptionOwner({
    observationMap: () => map,
    liveControls: () => liveControlHook === undefined ? liveControls : liveControlHook(),
    readObservations: async (selections) => {
      readCount += 1;
      if (readHook) await readHook();
      return selections.map((selection) => ({
        ...selection,
        nativeEffectId: "miso.compressor",
        descriptor: FAKE_DESCRIPTOR,
        sampleRateHz: 48_000,
        status: armed && selection.effectSlotId !== "gate" ? "ready" : "unarmed",
        ...(armed && selection.effectSlotId !== "gate" ? {
          left: 1, right: 2,
          window: { firstSample: 0n, endSample: 128n, sequence, blocks: 1 },
        } : {}),
      }));
    },
    scheduler: {
      setInterval: (callback) => { timer = callback; return 1; },
      clearInterval: () => { timer = undefined; },
    },
  });
  return {
    owner,
    map,
    liveControls,
    fire: () => timer?.(),
    setSequence: (value) => { sequence = value; },
    setSubmit: (hook) => { submitHook = hook; },
    setRead: (hook) => { readHook = hook; },
    setLiveControls: (hook) => { liveControlHook = hook; },
    readCount: () => readCount,
    submitCount: () => submitCount,
  };
}

const injectedSelection = selection("comp");

describe("issue 783 -- managed resident observation subscriptions", () => {
  test("shares bindings, preserves owned rows, guards manual edits, and closes last owner", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64, observationTaps: 4 },
    });
    const compLeft = selection("comp", "left");
    const gateRight = selection("gate", "right");
    try {
      let notifications = 0;
      const first = await engine.subscribeObservations({
        selections: [compLeft, gateRight],
        windowBlocks: 2,
        onUpdate: () => { notifications += 1; },
      });
      assert.equal(first.configuration.windowBlocks, 2);
      assert.deepEqual(first.readLatest().map((row) => row.status), ["pending", "pending"]);
      const liveControls = engine.liveControls();
      await assert.rejects(
        () => liveControls.submit(liveControls.edit.track("t").effect("inserts", 0, "miso.compressor")
          .observe("Gain Reduction", false, 2)),
        /conflict/,
      );

      feed(engine, 0); engine.render(); await first.pump();
      feed(engine, 1); engine.render(); await first.pump();
      const saved = structuredClone(first.readLatest());
      assert.ok(notifications >= 1);
      assert.deepEqual(saved.map((row) => [row.status, row.channels]), [
        ["ready", "left"], ["ready", "right"],
      ]);
      assert.ok(saved[0].window && saved[0].window.sequence > 0n);
      const second = await engine.subscribeObservations({
        selections: [selection("comp", "right"), selection("gate", "left")],
        windowBlocks: 2,
      });
      assert.equal(second.readLatest().length, 2);
      assert.equal(second.readLatest()[0].channels, "right");

      await assert.rejects(
        () => engine.subscribeObservations({ selections: [compLeft], windowBlocks: 3 }),
        /conflicts/,
      );
      await assert.rejects(
        () => first.update({ selections: [selection("missing")], windowBlocks: 2 }),
        /unavailable|unsupported/,
      );
      assert.equal(first.configuration.selections[0].effectSlotId, "comp");
      assert.equal(first.readLatest()[0].window.sequence, saved[0].window.sequence);

      await first.close();
      assert.notEqual(engine.readObservations([compLeft])[0].status, "unarmed");
      await second.close();
      assert.equal(engine.readObservations([compLeft])[0].status, "unarmed");
      await second.close();
    } finally {
      engine.dispose();
    }
  });

  // #1345 gate 6. The effect's observation queue has one slot per declared tap and an `Observe`
  // record overwrites its tap's whole state, so a submission names each tap once.
  test("a window change on a one-tap effect is one re-arm record and lands", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64, observationTaps: 4 },
    });
    const submitted = [];
    const submitCommands = engine.submitCommands.bind(engine);
    engine.submitCommands = (bytes, count) => {
      submitted.push(count);
      return submitCommands(bytes, count);
    };
    try {
      const handle = await engine.subscribeObservations({
        selections: [selection("comp", "both")], windowBlocks: 2,
      });
      for (let block = 0; block < 2; block += 1) { feed(engine, block); engine.render(); }
      await handle.pump();
      assert.equal(handle.readLatest()[0].status, "ready");
      assert.equal(handle.readLatest()[0].window.blocks, 2);
      const submitsBefore = submitted.length;

      const receipt = await handle.update({ selections: [selection("comp", "both")], windowBlocks: 4 });
      assert.deepEqual(submitted.slice(submitsBefore), [1], "the update is one submission of one Observe edit");
      assert.equal(receipt.configuration.windowBlocks, 4);
      for (let block = 2; block < 6; block += 1) { feed(engine, block); engine.render(); }
      await handle.pump();
      const reading = handle.readLatest()[0];
      assert.equal(reading.status, "ready");
      assert.equal(reading.window.blocks, 4);
      assert.ok(reading.window.firstSample >= receipt.appliedAtSample);
      await handle.close();
    } finally {
      engine.dispose();
    }
  });

  test("a submission naming one observation tap twice is refused whole as malformed", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64, observationTaps: 4 },
    });
    try {
      const liveControls = engine.liveControls();
      const edit = (on, window) => liveControls.edit.track("t").effect("inserts", 0, "miso.compressor")
        .observe("Gain Reduction", on, window);
      assert.equal((await liveControls.submit(edit(true, 4))).ok, true);
      feed(engine, 0); engine.render();
      const refused = await liveControls.submit(edit(false, 2), edit(true, 4));
      assert.equal(refused.ok, false);
      assert.equal(refused.reason, 1);
      assert.equal(refused.reasonName, "malformed");
      assert.equal(refused.rejectedIndex, 1, "the second record of the tap is the one refused");
      assert.equal(refused.admitted, 0);
      for (let block = 1; block < 5; block += 1) { feed(engine, block); engine.render(); }
      const reading = engine.readObservations([selection("comp", "both")])[0];
      assert.equal(reading.status, "ready");
      assert.equal(reading.window.blocks, 4, "nothing of the refused submission applied");
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
      liveControls: { commandQueueRecords: 64, observationTaps: 4 },
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
      liveControls: { commandQueueRecords: 64, observationTaps: 4 },
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
      liveControls: { commandQueueRecords: 64, observationTaps: 2 },
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
    harness.setSubmit(() => new Promise((resolve) => { releaseSubmit = resolve; }));
    const pending = harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.throws(
      () => harness.owner.beforeLiveControlSubmit([{ kind: "observeUnsubscribe" }]),
      /conflict/,
    );
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
      trackId: "t", rack: "inserts", effectSlotId: "gate", effectIndex: 1,
      nativeEffectId: "miso.compressor", tapIds: [1],
    });
    const managed = (await mutationHarness.owner.subscribe({
      selections: [injectedSelection], windowBlocks: 1,
    })).handle;
    let releaseRead;
    mutationHarness.setRead(() => new Promise((resolve) => { releaseRead = resolve; }));
    const updating = managed.update({ selections: [injectedSelection, selection("gate")], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    const readsDuringPreflight = mutationHarness.readCount();
    mutationHarness.fire();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(mutationHarness.readCount(), readsDuringPreflight);
    releaseRead();
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
    let releaseLiveControls;
    epochHarness.setLiveControls(() => new Promise((resolve) => { releaseLiveControls = resolve; }));
    const stale = epochHarness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    epochHarness.owner.invalidate();
    releaseLiveControls(epochHarness.liveControls);
    await assert.rejects(stale, /owner changed/);
    assert.equal(epochHarness.submitCount(), 0);
  });

  test("retries a refused close and bounds the readable union before admission", async () => {
    const harness = injectedOwner();
    const sub = (await harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 })).handle;
    harness.setSubmit(() => { throw new Error("temporary refusal"); });
    await assert.rejects(() => sub.close(), /temporary refusal/);
    harness.setSubmit(undefined);
    await sub.close();
    assert.equal(harness.submitCount(), 3);

    assert.throws(
      () => new ObservationSubscriptionOwner({
        observationMap: () => harness.map,
        readObservations: () => [],
        liveControls: () => harness.liveControls,
      }, { maximumBindings: 257 }),
      /maximumBindings.*256/,
    );
  });
});

// #1345 gate 6 bullet 3, browser. No browser runs here, so the engine is the real browser entry
// (`createEngine`) over a scripted host that models the effect's one-slot observation queue: a
// command is admitted while the slot is free, refused with backpressure while an earlier command
// still holds it, and a render (`renderedQuanta` moving) frees it. The context renders on its own
// clock, which the test controls. The CI browser legs do not run this case against the real
// worklet yet; that is #1530's work.
describe("issue 1345 -- the browser engine waits for a render, within a bound", () => {
  const SHAPE = Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128, tracks: ["t"], submixes: [] });

  function slotHost() {
    const log = { commands: [], statusCalls: 0 };
    let rendered = 0n;
    let slotFull = false;
    let nextMalformed = false;
    let sticky = false;
    const ack = (result, reason, admitted) => ({
      tag: "miso.ack.v1", requestId: 1, result, reason, rejectedIndex: 0, admitted,
      appliedAtSample: rendered * 128n, records: new Uint8Array(0),
    });
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1", requestId: 1, result: 0, tracks: ["t"],
          sources: [], metersAttached: false, submixes: [], routes: [], vcas: [],
        };
      },
      async command(request) {
        log.commands.push({ at: rendered, kinds: request.commands.map((command) => command.kind) });
        if (nextMalformed) { nextMalformed = false; return ack(1, 1, 0); }
        if (slotFull || sticky) return ack(6, 8, 0);
        slotFull = true;
        return ack(0, 0, request.commands.length);
      },
      async observationMap() {
        return {
          tag: "miso.observationmap.v1", requestId: 1, result: 0,
          bindings: [{
            trackIndex: 0, rack: 1, effectIndex: 0, effectSlotId: "comp",
            nativeEffectId: "miso.compressor", tapIds: [1],
          }],
        };
      },
      async readObservations(request) {
        return {
          tag: "miso.observationread.v1", requestId: 1, result: 0,
          rows: request.selections.map((address) => ({
            ...address, status: 2, sampleRateHz: 48_000, firstSample: 0n, endSample: 0n,
            sequence: 0n, blocks: 0, leftPresent: 0, rightPresent: 0, left: 0, right: 0,
          })),
        };
      },
      async status() {
        log.statusCalls += 1;
        return { tag: "miso.status.v1", result: 0, state: 2, renderedQuanta: rendered };
      },
      async dispose() {},
    };
    return {
      host,
      log,
      render() { rendered += 1n; slotFull = false; },
      refuseNextAsMalformed() { nextMalformed = true; },
      // Another writer takes the slot again after every render, so no render ever frees it.
      keepRefusing() { sticky = true; },
    };
  }

  async function browserEngine(rig, extra = {}) {
    return createEngine({
      document: "opaque",
      scratchBoot: async () => SHAPE,
      createContext: () => ({
        sampleRate: 48_000, renderQuantumSize: 128, state: "suspended",
        audioWorklet: { async addModule() {} }, async close() {},
      }),
      createHost: async () => rig.host,
      policy: { liveControls: { commandQueueRecords: 64, meterBlocks: 0, observationTaps: 4, masterTrackPlusOne: 0 } },
      ...extra,
    });
  }

  const request = { selections: [selection("comp", "both")], windowBlocks: 2 };

  test("a close whose slot is taken waits for the context to render and goes through", async () => {
    const rig = slotHost();
    const engine = await browserEngine(rig);
    try {
      const handle = await engine.subscribeObservations(request);
      const closing = handle.close();
      setTimeout(() => rig.render(), 30);
      await closing;
      // Subscribe, the refused close, and the same close sent again after the render.
      assert.deepEqual(rig.log.commands.map((entry) => entry.at), [0n, 0n, 1n]);
      assert.equal(rig.log.commands[1].kinds[0], rig.log.commands[2].kinds[0]);
    } finally {
      await engine.close();
    }
  });

  test("a suspended context ends the wait in typed backpressure at the deadline, and the polling stops", async () => {
    const rig = slotHost();
    const engine = await browserEngine(rig, { requestDeadlineMs: 80 });
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
      const calls = rig.log.statusCalls;
      await new Promise((resolve) => setTimeout(resolve, 60));
      assert.equal(rig.log.statusCalls, calls);
      // Nothing was admitted by the refusal; once the context renders the same close goes through.
      rig.render();
      await handle.close();
    } finally {
      await engine.close();
    }
  });

  test("renders that never free the slot end the wait at one deadline from the first refusal", async () => {
    // A render lands every 60 ms and each one is followed by another refusal. A deadline fixed per
    // resend would keep the wait going to the cap of 8 resends, about 480 ms here; one deadline
    // from the first refusal ends it at 150 ms.
    const rig = slotHost();
    const engine = await browserEngine(rig, { requestDeadlineMs: 150 });
    try {
      const handle = await engine.subscribeObservations(request);
      rig.keepRefusing();
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
    const rig = slotHost();
    const engine = await browserEngine(rig, { requestDeadlineMs: 3_000 });
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

  test("a refusal that is not backpressure rejects at once and is never sent again", async () => {
    const rig = slotHost();
    const engine = await browserEngine(rig, { requestDeadlineMs: 2_000 });
    try {
      const handle = await engine.subscribeObservations(request);
      rig.refuseNextAsMalformed();
      const started = Date.now();
      await assert.rejects(() => handle.close(), (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "invalidArgument");
        return true;
      });
      assert.ok(Date.now() - started < 500, "a malformed refusal does not wait for a render");
      assert.equal(rig.log.commands.length, 2, "the subscribe and the one refused close, no resend");
    } finally {
      await engine.close();
    }
  });
});
