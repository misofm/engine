/** Issue #785: managed live-response sharing and captured-state suppression. */

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  if (process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX) asset = await MisoEngineAsset.load(await moduleBytes());
});

test("managed live responses share jobs, suppress unchanged captures, and own latest arrays", {
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
  const engine = await createOfflineEngine(sessionDocument({
    effects: { simd1: [effectEntry("eq", "miso.parametric-eq", params)] },
  }), { asset, console: { commandQueueRecords: 64 } });
  const request = {
    trackId: "t",
    grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
    cadenceMs: 1,
  };
  try {
    const first = await engine.subscribeTrackResponse(request);
    const second = await engine.subscribeTrackResponse(request);
    assert.equal(first.job, second.job);
    assert.equal(first.revision, 1n);
    const firstResult = first.readLatest();
    const secondResult = second.readLatest();
    assert.ok(firstResult && secondResult);
    const secondValue = secondResult.leftDb[0];
    firstResult.leftDb[0] = Number.NaN;
    assert.equal(secondResult.leftDb[0], secondValue);
    assert.equal(await first.pump(), undefined);

    const semanticConsole = engine.console();
    await semanticConsole.submit(
      semanticConsole.edit.track("t").effect("simd1", 0, "miso.parametric-eq")
        .parameter("band-1-gain", -3),
    );
    assert.equal(await first.pump(), undefined);
    const shape = engine.shape();
    for (const source of shape.sources) {
      engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: 0n,
        planes: Array.from({ length: source.channels }, () => new Float32Array(shape.quantumFrames)),
        endOfRegion: false,
      });
    }
    engine.render();
    const notification = await first.pump();
    assert.ok(notification);
    assert.equal(notification.revision, 2n);
    assert.equal(notification.skippedPublications, 0n);

    await first.close();
    assert.ok(second.readLatest());
    await second.close();
  } finally {
    if (engine.state() !== "disposed") engine.dispose();
  }
});

test("resident and response handles share one owner poll, timer, and epoch", async () => {
  let timer;
  let timerStarts = 0;
  let releaseResponse;
  let responseReads = 0;
  let responseWords = [1];
  const observationMap = {
    bindings: [{
      trackId: "t", rack: "dynamic", effectSlotId: "comp", effectIndex: 0,
      nativeEffectId: "miso.compressor", tapIds: [1],
    }],
  };
  const observationConsole = {
    edit: {
      track: () => ({
        effect: () => ({ observe: (_tap, on) => ({ kind: on ? "observeSubscribe" : "observeUnsubscribe" }) }),
      }),
    },
    async submit(...edits) {
      return {
        ok: true, result: 0, code: "ok", reason: 0, reasonName: "none", rejectedIndex: 0,
        admitted: edits.length, appliedAtSample: 0n,
      };
    },
  };
  const transport = {
    observationMap: () => observationMap,
    readObservations: (selections) => selections.map((selection) => ({
      ...selection,
      nativeEffectId: "miso.compressor",
      descriptor: { id: 1, name: "Gain Reduction", displayUnit: "dB", unitName: "dB", subscribable: true },
      sampleRateHz: 48_000,
      status: "unarmed",
    })),
    console: () => observationConsole,
    responseRead: async (_request, previousState) => {
      responseReads += 1;
      if (previousState !== undefined) {
        await new Promise((resolve) => { releaseResponse = resolve; });
      }
      return {
        changed: previousState === undefined || previousState.words[0] !== responseWords[0],
        state: { words: [...responseWords] },
        ...(previousState === undefined || previousState.words[0] !== responseWords[0] ? {
          result: {
            trackId: "t", mode: "target", meaning: "eqFilterSubtotal", sampleRateHz: 48_000, floorDb: -120,
            frequenciesHz: new Float32Array([20, 20_000]), leftDb: new Float32Array([-1, -2]),
            rightDb: new Float32Array([-1, -2]), members: [], capturedSample: 0n, snapshotToken: BigInt(responseReads),
            excludedMemberCount: 0, resultBytes: 200n,
          },
        } : {}),
      };
    },
    scheduler: {
      setInterval: (callback) => { timer = callback; timerStarts += 1; return 1; },
      clearInterval: () => { timer = undefined; },
    },
  };
  const owner = new ObservationSubscriptionOwner(transport, undefined, {
    maximumCadenceMs: 10,
    maximumCaptureAttempts: 2,
  });
  const resident = await owner.subscribe({
    selections: [{ trackId: "t", rack: "dynamic", effectSlotId: "comp", tapId: 1, channels: "both" }],
    windowBlocks: 1,
    cadenceMs: 10,
  });
  const response = await owner.subscribeTrackResponse({
    trackId: "t", grid: { kind: "linear", points: 2, minimumHz: 20, maximumHz: 20_000 },
    channels: "both", cadenceMs: 10,
  });
  assert.equal(resident.owner, response.owner);
  assert.equal(resident.epoch, response.epoch);
  assert.equal(timerStarts, 1);
  const polling = response.handle.pump();
  await new Promise((resolve) => setTimeout(resolve, 0));
  timer();
  assert.equal(responseReads, 2);
  releaseResponse();
  await polling;
  owner.invalidate();
  assert.throws(() => resident.handle.readLatest(), /stale|closed/);
  await assert.rejects(() => response.handle.pump(), /stale|closed/);
});
