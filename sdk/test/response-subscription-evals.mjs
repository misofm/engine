/** Issue #785: managed live-response sharing and captured-state suppression. */

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { parseTrackResponseState, sameTrackResponseState } from "../src/core/live-response.ts";
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

function parserCapture() {
  const resultLayout = ABI_LAYOUT.structures.liveResponseResult;
  const ownerLayout = ABI_LAYOUT.structures.liveResponseOwner;
  const sectionLayout = ABI_LAYOUT.structures.liveResponseSection;
  const fieldOffset = (layout, name) => layout.fields.find((field) => field.name === name).offset;
  const resultBytes = resultLayout.bytes + ownerLayout.bytes + sectionLayout.bytes + 3 + 64;
  const capture = new Uint8Array(resultBytes);
  const view = new DataView(capture.buffer);
  const setU32 = (layout, base, name, value) => view.setUint32(base + fieldOffset(layout, name), value, true);
  const setU64 = (layout, base, name, value) => view.setBigUint64(base + fieldOffset(layout, name), value, true);
  setU32(resultLayout, 0, "structSize", resultLayout.bytes);
  setU32(resultLayout, 0, "abiVersion", ABI_LAYOUT.abiVersion);
  setU32(resultLayout, 0, "result", 0);
  setU32(resultLayout, 0, "mode", 1);
  setU32(resultLayout, 0, "meaning", 1);
  setU32(resultLayout, 0, "channels", 3);
  setU32(resultLayout, 0, "points", 0);
  setU32(resultLayout, 0, "ownerCount", 1);
  setU32(resultLayout, 0, "excludedCount", 0);
  setU32(resultLayout, 0, "sampleRateHz", 48_000);
  setU64(resultLayout, 0, "capturedSample", (1n << 60n) + 7n);
  setU64(resultLayout, 0, "snapshotToken", (1n << 61n) + 9n);
  setU64(resultLayout, 0, "resultBytes", BigInt(resultBytes));
  const owner = resultLayout.bytes;
  setU32(resultLayout, 0, "ownersOffset", owner);
  setU32(resultLayout, 0, "ownerRecordBytes", ownerLayout.bytes);
  setU32(resultLayout, 0, "sectionRecordBytes", sectionLayout.bytes);
  const section = owner + ownerLayout.bytes;
  const strings = section + sectionLayout.bytes;
  setU32(ownerLayout, owner, "trackIdOffset", strings); setU32(ownerLayout, owner, "trackIdBytes", 1);
  setU32(ownerLayout, owner, "nativeIdOffset", strings + 1); setU32(ownerLayout, owner, "nativeIdBytes", 1);
  setU32(ownerLayout, owner, "stableIdOffset", strings + 2); setU32(ownerLayout, owner, "stableIdBytes", 1);
  setU32(ownerLayout, owner, "rack", 1); setU32(ownerLayout, owner, "slot", 2); setU32(ownerLayout, owner, "kind", 1);
  setU32(ownerLayout, owner, "bypassed", 0); setU32(ownerLayout, owner, "availability", 1);
  setU32(ownerLayout, owner, "leftOffset", section); setU32(ownerLayout, owner, "leftCount", 1);
  setU32(ownerLayout, owner, "rightOffset", section); setU32(ownerLayout, owner, "rightCount", 1);
  setU32(sectionLayout, section, "id", 7); setU32(sectionLayout, section, "kind", 1);
  setU32(sectionLayout, section, "enabled", 1); setU32(sectionLayout, section, "wordCount", 2);
  const wordsOffset = section + fieldOffset(sectionLayout, "words");
  view.setUint32(wordsOffset, 0x1234_5678, true); view.setUint32(wordsOffset + 4, 0x9abc_def0, true);
  capture.set(new TextEncoder().encode("tns"), strings);
  capture.fill(0xa5, strings + 3);
  return capture;
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
            rightDb: new Float32Array([-1, -2]), members: [], capturedSample: (1n << 54n) + 7n,
            snapshotToken: (1n << 55n) + BigInt(responseReads),
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

test("captured-state keys ignore unused storage and metadata while owning active words", () => {
  const capture = parserCapture();
  const baseline = parseTrackResponseState(capture);
  assert.equal(Object.isFrozen(baseline.words), true);

  const metadata = capture.slice();
  const resultLayout = ABI_LAYOUT.structures.liveResponseResult;
  const sectionLayout = ABI_LAYOUT.structures.liveResponseSection;
  const fieldOffset = (layout, name) => layout.fields.find((field) => field.name === name).offset;
  const setU32 = (view, layout, base, name, value) =>
    view.setUint32(base + fieldOffset(layout, name), value, true);
  const setU64 = (view, layout, base, name, value) =>
    view.setBigUint64(base + fieldOffset(layout, name), value, true);
  setU64(new DataView(metadata.buffer), resultLayout, 0, "capturedSample", (1n << 62n) + 1n);
  assert.equal(sameTrackResponseState(baseline, parseTrackResponseState(metadata)), true);

  const unused = capture.slice();
  unused[unused.length - 1] ^= 0xff;
  assert.equal(sameTrackResponseState(baseline, parseTrackResponseState(unused)), true);

  const active = capture.slice();
  setU32(new DataView(active.buffer), sectionLayout, resultLayout.bytes + ABI_LAYOUT.structures.liveResponseOwner.bytes,
    "words", 0xfeed_beef);
  assert.equal(sameTrackResponseState(baseline, parseTrackResponseState(active)), false);

  const ownedWords = [...baseline.words];
  structuredClone(capture, { transfer: [capture.buffer] });
  assert.deepEqual([...baseline.words], ownedWords);
});

test("response delivery admission is aggregate, updates are atomic, and polling is per-job", async () => {
  let now = 1_000;
  const realNow = Date.now;
  Date.now = () => now;
  let timer;
  const reads = [];
  const owner = new ObservationSubscriptionOwner({
    responseRead: (request, previousState) => {
      reads.push(request.trackId);
      const stateWord = request.trackId === "fast" ? 1 : 2;
      const changed = previousState === undefined;
      return {
        changed,
        state: { words: [stateWord] },
        ...(changed ? { result: responseResult(request.trackId) } : {}),
      };
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
