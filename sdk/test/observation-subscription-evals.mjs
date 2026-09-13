/** Issue #783: one bounded managed resident-observation lifetime. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
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
      dynamic: [
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
  return { trackId: "t", rack: "dynamic", effectSlotId, tapId: 1, channels };
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
  let consoleHook;
  let readCount = 0;
  let submitCount = 0;
  const map = {
    bindings: [{
      trackId: "t", rack: "dynamic", effectSlotId: "comp", effectIndex: 0,
      nativeEffectId: "miso.compressor", tapIds: [1],
    }],
  };
  const console = {
    edit: {
      track: () => ({
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
    console: () => consoleHook === undefined ? console : consoleHook(),
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
    console,
    fire: () => timer?.(),
    setSequence: (value) => { sequence = value; },
    setSubmit: (hook) => { submitHook = hook; },
    setRead: (hook) => { readHook = hook; },
    setConsole: (hook) => { consoleHook = hook; },
    readCount: () => readCount,
    submitCount: () => submitCount,
  };
}

const injectedSelection = selection("comp");

describe("issue 783 -- managed resident observation subscriptions", () => {
  test("shares bindings, preserves owned rows, guards manual edits, and closes last owner", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      console: { commandQueueRecords: 64, observationTaps: 4 },
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
      const console = engine.console();
      await assert.rejects(
        () => console.submit(console.edit.track("t").effect("dynamic", 0, "miso.compressor")
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

  test("invalidates a handle at headless replacement", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      console: { commandQueueRecords: 64, observationTaps: 2 },
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

  test("serializes internal edits and reads, honors cadence, and baselines joining loss", async () => {
    const harness = injectedOwner();
    let releaseSubmit;
    harness.setSubmit(() => new Promise((resolve) => { releaseSubmit = resolve; }));
    const pending = harness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.throws(
      () => harness.owner.beforeConsoleSubmit([{ kind: "observeUnsubscribe" }]),
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
      trackId: "t", rack: "dynamic", effectSlotId: "gate", effectIndex: 1,
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
    let releaseConsole;
    epochHarness.setConsole(() => new Promise((resolve) => { releaseConsole = resolve; }));
    const stale = epochHarness.owner.subscribe({ selections: [injectedSelection], windowBlocks: 1 });
    await new Promise((resolve) => setTimeout(resolve, 0));
    epochHarness.owner.invalidate();
    releaseConsole(epochHarness.console);
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
        console: () => harness.console,
      }, { maximumBindings: 257 }),
      /maximumBindings.*256/,
    );
  });
});
