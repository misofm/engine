/** Issue #785: managed live-response sharing and captured-state suppression. */

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, sessionDocument } from "./support.mjs";

let asset;

function responseResult(trackId) {
  return {
    trackId,
    mode: "target",
    meaning: "eqFilterSubtotal",
    sampleRateHz: 48_000,
    floorDb: -120,
    frequenciesHz: new Float32Array([20, 20_000]),
    leftDb: new Float32Array([-1, -2]),
    rightDb: new Float32Array([-3, -4]),
    members: [],
    capturedSample: 0n,
    snapshotToken: 1n,
    excludedMemberCount: 0,
    resultBytes: 200n,
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
    effects: {
      preInsert: [effectEntry("eq", "miso.parametric-eq", params)],
      inserts: [effectEntry("comp", "miso.compressor")],
    },
  }), { asset });
  const request = {
    trackId: "t",
    grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
    cadenceMs: 1,
  };
  try {
    // A headless read is a query, which settles after the app's next render (#1579a D2).
    const first = await renderUntil(engine, engine.subscribeTrackResponse(request));
    const second = await renderUntil(engine, engine.subscribeTrackResponse(request));
    assert.equal(first.job, second.job);
    assert.equal(first.revision, 1n);
    const firstResult = first.readLatest();
    const secondResult = second.readLatest();
    assert.ok(firstResult && secondResult);
    const secondValue = secondResult.leftDb[0];
    firstResult.leftDb[0] = Number.NaN;
    assert.equal(secondResult.leftDb[0], secondValue);
    // A later capture of the same values publishes nothing, although its sample and tag moved.
    await new Promise((resolve) => setTimeout(resolve, 2));
    assert.equal(await renderUntil(engine, first.pump()), undefined);

    const semanticLiveControls = engine.liveControls();
    await semanticLiveControls.submit(
      semanticLiveControls.edit.track("t").console("eq", "miso.parametric-eq")
        .parameter("band-1-gain", -3),
    );
    await new Promise((resolve) => setTimeout(resolve, 2));
    const notification = await renderUntil(engine, first.pump());
    assert.ok(notification);
    assert.equal(notification.revision, 2n);
    assert.equal(notification.skippedPublications, 0n);

    // A live bypass of the compressor (kind 3: no curve, so no curve bit moves) changes one member
    // field and publishes exactly once. Test value: red if the owner's comparison skips a member
    // field such as `bypassed`, so an owner change with an unchanged curve is never published.
    const editedLeft = [...first.readLatest().leftDb];
    await semanticLiveControls.submit(
      semanticLiveControls.edit.track("t").insert("comp", "miso.compressor").bypass(true),
    );
    await new Promise((resolve) => setTimeout(resolve, 2));
    const bypassed = await renderUntil(engine, first.pump());
    assert.ok(bypassed);
    assert.equal(bypassed.revision, 3n);
    const compressor = first.readLatest().members.find((member) => member.stableId === "comp");
    assert.equal(compressor.kindValue, 3);
    assert.equal(compressor.bypassed, true);
    assert.deepEqual([...first.readLatest().leftDb], editedLeft);
    await new Promise((resolve) => setTimeout(resolve, 2));
    assert.equal(await renderUntil(engine, first.pump()), undefined);

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
  const observationMap = {
    bindings: [{
      trackId: "t", rack: "inserts", effectSlotId: "comp", effectIndex: 0,
      nativeEffectId: "miso.compressor", tapIds: [1],
    }],
  };
  const transport = {
    observationMap: () => observationMap,
    // #1589b: the observation half on the core's frames; the tap never arms here.
    apply: async (edits) => ({ revision: 1n, observeArmTags: edits.map(() => 1n), removedArmTags: [] }),
    read: async (selections) => ({
      observedSample: 0n,
      rows: selections.map(() => ({
        status: "unarmed", armed: false, armSample: 0n, armTag: 0n,
        window: undefined, left: undefined, right: undefined, unit: 1,
      })),
    }),
    sampleRateHz: () => 48_000,
    responseRead: async () => {
      responseReads += 1;
      if (responseReads > 1) {
        await new Promise((resolve) => { releaseResponse = resolve; });
      }
      return {
        trackId: "t", mode: "target", meaning: "eqFilterSubtotal", sampleRateHz: 48_000, floorDb: -120,
        frequenciesHz: new Float32Array([20, 20_000]), leftDb: new Float32Array([-1, -2]),
        rightDb: new Float32Array([-1, -2]), members: [], capturedSample: (1n << 54n) + 7n,
        snapshotToken: (1n << 55n) + BigInt(responseReads),
        excludedMemberCount: 0, resultBytes: 200n,
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
    selections: [{ trackId: "t", rack: "inserts", effectSlotId: "comp", tapId: 1, channels: "both" }],
    windowBlocks: 1,
    cadenceMs: 10,
  });
  const response = await owner.subscribeTrackResponse({
    trackId: "t", grid: { kind: "linear", points: 2, minimumHz: 20, maximumHz: 20_000 },
    channels: "both", cadenceMs: 10,
  });
  const responseResultValue = response.handle.readLatest();
  assert.equal(responseResultValue.capturedSample, (1n << 54n) + 7n);
  assert.equal(responseResultValue.snapshotToken, (1n << 55n) + 1n);
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

test("response delivery admission is aggregate, updates are atomic, and polling is per-job", async () => {
  let now = 1_000;
  const realNow = Date.now;
  Date.now = () => now;
  let timer;
  const reads = [];
  const owner = new ObservationSubscriptionOwner({
    responseRead: (request) => {
      reads.push(request.trackId);
      return responseResult(request.trackId);
    },
    scheduler: {
      setInterval: (callback) => { timer = callback; return 1; },
      clearInterval: () => { timer = undefined; },
    },
  }, undefined, {
    maximumDeliveredBytesPerSecond: 4_000,
    maximumCadenceMs: 1_000,
  });
  const request = (trackId, cadenceMs, requestDeadlineMs) => ({
    trackId,
    grid: { kind: "linear", points: 2, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
    cadenceMs,
    ...(requestDeadlineMs === undefined ? {} : { responseLimits: { requestDeadlineMs } }),
  });
  let fast;
  let slow;
  let deadlineA;
  let deadlineB;
  try {
    fast = await owner.subscribeTrackResponse(request("fast", 10));
    slow = await owner.subscribeTrackResponse(request("slow", 100));
    assert.equal(reads.length, 2);
    await assert.rejects(() => fast.handle.update(request("fast", 1)), /delivery bound/);
    assert.equal(fast.configuration.cadenceMs, 10);

    deadlineA = await owner.subscribeTrackResponse(request("deadline", 100, 10));
    deadlineB = await owner.subscribeTrackResponse(request("deadline", 100, 20));
    assert.notEqual(deadlineA.job, deadlineB.job);

    now = 1_010;
    timer();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(reads.filter((trackId) => trackId === "fast").length, 2);
    assert.equal(reads.filter((trackId) => trackId === "slow").length, 1);
  } finally {
    if (deadlineB !== undefined) await deadlineB.handle.close();
    if (deadlineA !== undefined) await deadlineA.handle.close();
    if (slow !== undefined) await slow.handle.close();
    if (fast !== undefined) await fast.handle.close();
    Date.now = realNow;
  }
});
