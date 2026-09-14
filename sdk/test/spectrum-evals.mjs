import assert from "node:assert/strict";
import { test } from "node:test";

import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { ObservationSubscriptionOwner } from "../src/core/observation-subscriptions.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

const WINDOW_FRAMES = 2_048;
const CHANNELS = "both";

function eqParameters() {
  const definition = CATALOG.effects.find((row) => row.id === "miso.parametric-eq");
  assert.ok(definition, "the generated catalog must contain parametric EQ");
  return ["band-1-enabled", "band-1-kind", "band-1-frequency", "band-1-gain", "band-1-q"]
    .map((name) => {
      const parameter = definition.parameters.find((row) => row.name === name);
      assert.ok(parameter, `the generated EQ catalog must contain ${name}`);
      return {
        id: parameter.id,
        unit: parameter.unitName,
        value: name === "band-1-enabled" ? 1
          : name === "band-1-kind" ? 1
            : name === "band-1-frequency" ? 1_000
              : name === "band-1-gain" ? 6
                : parameter.default,
        channel: "both",
      };
    });
}

function spectrumDocument() {
  return sessionDocument({
    frames: 4_800,
    effects: {
      simd1: [effectEntry("eq", "miso.parametric-eq", eqParameters())],
      dynamic: [],
      simd2: [],
    },
  });
}

function queryFor(target, channels = CHANNELS) {
  return {
    target,
    channels,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  };
}

function feedAndRender(engine, block) {
  const shape = engine.shape();
  const source = shape.sources[0];
  assert.ok(source, "the spectrum fixture must have one source");
  const acknowledgement = engine.submitSource({
    sourceId: source.id,
    generation: 1n,
    startFrame: BigInt(block * shape.quantumFrames),
    planes: [
      ramp(shape.quantumFrames, 17 + block),
      ramp(shape.quantumFrames, 71 + block),
    ],
    endOfRegion: false,
  });
  assert.equal(acknowledgement.ok, true, `source block ${block} was refused`);
  return engine.render(shape.quantumFrames);
}

function assertSpectrumResult(result, query, expectedSample) {
  assert.deepEqual(result.target, query.target);
  assert.equal(result.channels, query.channels);
  assert.equal(result.sampleRateHz, 48_000);
  assert.equal(result.windowFrames, WINDOW_FRAMES);
  assert.equal(result.binCount, WINDOW_FRAMES / 2 + 1);
  assert.equal(result.capturedSample, BigInt(expectedSample));
  assert.equal(result.endSample, BigInt(expectedSample + WINDOW_FRAMES));
  assert.ok(result.snapshotToken > 0n);
  assert.equal(result.graphSourceUnderrun, false);
  assert.ok(result.resultBytes > 0n);
  assert.ok(Number.isFinite(result.floorDb));
  assert.ok(result.frequenciesHz instanceof Float32Array);
  assert.equal(result.frequenciesHz.length, result.binCount);
  assert.ok(result.frequenciesHz.every(Number.isFinite));
  if (query.channels === "both" || query.channels === "left") {
    assert.ok(result.leftDb instanceof Float32Array);
    assert.equal(result.leftDb.length, result.binCount);
    assert.ok(result.leftDb.every(Number.isFinite));
  } else {
    assert.equal(result.leftDb, undefined);
  }
  if (query.channels === "both" || query.channels === "right") {
    assert.ok(result.rightDb instanceof Float32Array);
    assert.equal(result.rightDb.length, result.binCount);
    assert.ok(result.rightDb.every(Number.isFinite));
  } else {
    assert.equal(result.rightDb, undefined);
  }
}

async function makeEngine(asset, query) {
  return createOfflineEngine(spectrumDocument(), {
    asset,
    spectrum: query,
  });
}

test("candidate Wasm spectrum query captures all graph targets through explicit renders", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const queries = [
    queryFor({ kind: "trackPostInputBuiltins", trackId: "t" }),
    queryFor({ kind: "trackPostMatrix", trackId: "t" }),
    queryFor({ kind: "output", outputId: "out" }),
  ];
  const results = [];
  for (const query of queries) {
    const engine = await makeEngine(asset, query);
    try {
      assert.throws(() => engine.readSpectrum(), /spectrum capture read|wrongState/,
        "a spectrum read before arming must be refused explicitly");
      const armed = engine.armSpectrum();
      assert.equal(armed.ok, true);
      const busy = engine.armSpectrum();
      assert.equal(busy.ok, false, "a second arm must be refused while the window is pending");
      assert.equal(busy.code, "backpressure");
      for (let block = 0; block < WINDOW_FRAMES / engine.shape().quantumFrames; block += 1) {
        feedAndRender(engine, block);
        if (block + 1 < WINDOW_FRAMES / engine.shape().quantumFrames) {
          assert.equal(engine.readSpectrum(), undefined,
            "the result must remain pending until the complete window is rendered");
        }
      }
      const first = engine.readSpectrum();
      assert.ok(first);
      assertSpectrumResult(first, query, 0);
      assert.notEqual(first.frequenciesHz, first.leftDb);
      assert.notEqual(first.leftDb, first.rightDb);
      const firstFrequencyValues = first.frequenciesHz.slice();
      const firstLeftValues = first.leftDb?.slice();
      const firstRightValues = first.rightDb?.slice();

      assert.equal(engine.armSpectrum().ok, true);
      for (let block = WINDOW_FRAMES / engine.shape().quantumFrames;
        block < 2 * WINDOW_FRAMES / engine.shape().quantumFrames;
        block += 1) {
        feedAndRender(engine, block);
      }
      const second = engine.readSpectrum();
      assert.ok(second);
      assertSpectrumResult(second, query, WINDOW_FRAMES);
      assert.notEqual(second.frequenciesHz, first.frequenciesHz);
      assert.notEqual(second.leftDb, first.leftDb);
      assert.deepEqual(first.frequenciesHz, firstFrequencyValues,
        "the first frequency axis must remain owned after a later query");
      assert.deepEqual(first.leftDb, firstLeftValues,
        "the first left spectrum must remain owned after a later query");
      assert.deepEqual(first.rightDb, firstRightValues,
        "the first right spectrum must remain owned after a later query");
      results.push({ query, first, second });
    } finally {
      engine.dispose();
    }
  }

  assert.deepEqual(results.map(({ first }) => first.target), queries.map(({ target }) => target));
  assert.ok(results[0].first.leftDb.some((value) => value > results[0].first.floorDb),
    "the captured source must produce a measurable spectrum");
  assert.notDeepEqual(results[0].first.leftDb, results[1].first.leftDb,
    "the post-input and post-matrix taps must observe the EQ in the prepared graph");
});

test("candidate Wasm spectrum arm, cancel, and dispose preserve lifecycle refusals", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "trackPostMatrix", trackId: "t" });
  const engine = await makeEngine(asset, query);
  assert.equal(engine.armSpectrum().ok, true);
  const cancelled = engine.cancelSpectrum();
  assert.equal(cancelled.ok, true);
  assert.equal(cancelled.code, "ok");
  assert.throws(() => engine.readSpectrum(), /spectrum capture read|wrongState/);
  assert.equal(engine.armSpectrum().ok, true, "a cancelled capture must be re-armable");
  engine.dispose();
  assert.equal(engine.armSpectrum().ok, false);
  assert.equal(engine.armSpectrum().code, "unsupported");
  assert.throws(() => engine.readSpectrum(), /no prepared spectrum boundary/);
  engine.dispose();
});

test("candidate Wasm spectrum honors a selected channel and explicit capture limit", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "trackPostMatrix", trackId: "t" }, "left");
  const engine = await makeEngine(asset, query);
  try {
    assert.equal(engine.armSpectrum().ok, true);
    for (let block = 0; block < WINDOW_FRAMES / engine.shape().quantumFrames; block += 1) {
      feedAndRender(engine, block);
    }
    const result = engine.readSpectrum();
    assert.ok(result);
    assertSpectrumResult(result, query, 0);
    assert.ok(result.leftDb);
    assert.equal(result.rightDb, undefined);
  } finally {
    engine.dispose();
  }
});

test("candidate Wasm managed spectrum subscribes and pumps one owned window", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "output", outputId: "out" });
  const engine = await makeEngine(asset, query);
  let subscription;
  try {
    subscription = await engine.subscribeSpectrum({ ...query, smoothingMs: 0, cadenceMs: 1 });
    const manualCancel = engine.cancelSpectrum();
    assert.equal(manualCancel.ok, false, "manual cancellation must not steal a managed capture");
    assert.equal(manualCancel.code, "wrongState");
    assert.equal(subscription.readLatest(), undefined);
    for (let block = 0; block < WINDOW_FRAMES / engine.shape().quantumFrames; block += 1) {
      feedAndRender(engine, block);
    }
    const notification = await subscription.pump();
    assert.ok(notification);
    assert.equal(notification.status, "ready");
    assert.equal(notification.available, true);
    assert.equal(notification.metadata.capturedSample, 0n);
    const result = subscription.readLatest();
    assert.ok(result);
    assertSpectrumResult(result, query, 0);
    assert.notEqual(result.frequenciesHz, result.leftDb);
    assert.notEqual(result.leftDb, result.rightDb);
  } finally {
    await subscription?.close();
    engine.dispose();
  }
});

test("managed spectrum anchors asynchronous reads to native cadence", async (t) => {
  const target = { kind: "output", outputId: "out" };
  const query = queryFor(target);
  const scenarios = [
    { sampleRateHz: 44_100, sharedTimer: false },
    { sampleRateHz: 48_000, sharedTimer: true },
    { sampleRateHz: 88_200, sharedTimer: false },
    { sampleRateHz: 96_000, sharedTimer: false },
  ];
  let now = 0;
  t.mock.method(Date, "now", () => now);

  for (const scenario of scenarios) {
    const { sampleRateHz, sharedTimer } = scenario;
    let timer;
    let reads = 0;
    let observationArmed = false;
    const readReleases = [];
    const notifications = [];
    const observationSelection = {
      trackId: "t", rack: "dynamic", effectSlotId: "comp", tapId: 1, channels: CHANNELS,
    };
    const observationMap = {
      bindings: [{
        trackId: "t", rack: "dynamic", effectSlotId: "comp", effectIndex: 0,
        nativeEffectId: "miso.compressor", tapIds: [1],
      }],
    };
    const observationDescriptor = {
      id: 1, name: "Gain Reduction", displayUnit: "dB", unitName: "dB", subscribable: true,
    };
    const nativeCadenceMs = WINDOW_FRAMES * 1_000 / sampleRateHz;
    const timerCadenceMs = sharedTimer ? 1 : Math.ceil(nativeCadenceMs);
    const metadata = (status, sequence = 0n) => Object.freeze({
      result: 0,
      status,
      target,
      channels: CHANNELS,
      sampleRateHz,
      quantumFrames: 128,
      hopFrames: WINDOW_FRAMES,
      sourceUnderrun: false,
      captureEpoch: 1n,
      sequence,
      droppedCaptures: 0n,
      windows: sequence,
      capturedSample: sequence * BigInt(WINDOW_FRAMES),
      endSample: (sequence + 1n) * BigInt(WINDOW_FRAMES),
      analysisEpoch: 1n,
      historyStartSample: 0n,
      smoothingMs: 0,
    });
    const result = (sequence) => ({
      target,
      channels: CHANNELS,
      sampleRateHz,
      windowFrames: WINDOW_FRAMES,
      binCount: 1,
      floorDb: -120,
      frequenciesHz: new Float32Array([0]),
      leftDb: new Float32Array([0]),
      rightDb: new Float32Array([0]),
      capturedSample: sequence * BigInt(WINDOW_FRAMES),
      endSample: (sequence + 1n) * BigInt(WINDOW_FRAMES),
      snapshotToken: sequence,
      graphSourceUnderrun: false,
      resultBytes: 37n,
    });
    const owner = new ObservationSubscriptionOwner({
      observationMap: () => observationMap,
      readObservations: (selections) => selections.map((selection) => ({
        ...selection,
        nativeEffectId: "miso.compressor",
        descriptor: observationDescriptor,
        sampleRateHz,
        status: observationArmed ? "ready" : "unarmed",
        ...(observationArmed ? {
          left: 1, right: 1,
          window: { firstSample: 0n, endSample: 128n, sequence: 1n, blocks: 1 },
        } : {}),
      })),
      console: () => ({
        edit: {
          track: () => ({
            effect: () => ({ observe: (_tap, armed) => ({ kind: armed ? "observeSubscribe" : "observeUnsubscribe" }) }),
          }),
        },
        async submit(...edits) {
          observationArmed = edits.at(-1)?.kind === "observeSubscribe";
          return {
            ok: true, result: 0, code: "ok", reason: 0, reasonName: "none", rejectedIndex: 0,
            admitted: edits.length, appliedAtSample: 0n,
          };
        },
      }),
      spectrumPrepared: () => query,
      spectrumStart: async () => ({
        ok: true, result: 0, code: "ok", metadata: metadata("warming"),
      }),
      spectrumRead: async () => {
        reads += 1;
        await new Promise((resolve) => readReleases.push(resolve));
        const sequence = BigInt(reads);
        return { metadata: metadata("ready", sequence), result: result(sequence) };
      },
      spectrumStop: async () => ({ ok: true, result: 0, code: "ok" }),
      scheduler: {
        setInterval: (callback, milliseconds) => {
          timer = callback;
          assert.equal(milliseconds, timerCadenceMs);
          return 1;
        },
        clearInterval: () => { timer = undefined; },
      },
    }, undefined, undefined, { maximumDeliveredBytesPerSecond: 64 * 1024 * 1024 });
    const observation = sharedTimer ? (await owner.subscribe({
      selections: [observationSelection], windowBlocks: 1, cadenceMs: 1,
    })).handle : undefined;
    const spectrum = (await owner.subscribeSpectrum({
      ...query,
      cadenceMs: 1,
      onUpdate: (notification) => notifications.push(notification),
    })).handle;
    try {
      const firstTimerTick = timerCadenceMs;
      now = firstTimerTick;
      timer();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(reads, 1, `${sampleRateHz} Hz starts one bounded read`);

      now = firstTimerTick + 1;
      timer();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(reads, 1, "a second shared-timer tick cannot overlap the read");

      now = firstTimerTick + 5;
      readReleases.shift()();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(notifications.length, 1);
      assert.equal(notifications[0].available, true);
      assert.equal(notifications[0].nativeMissedWindows, 0n);
      assert.equal(notifications[0].skippedPublications, 0n);

      const firstDueTick = sharedTimer
        ? Math.ceil(firstTimerTick + nativeCadenceMs)
        : firstTimerTick * 2;
      now = firstDueTick - 1;
      timer();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(reads, 1, "polling remains bounded until the native cadence deadline");
      now = firstDueTick;
      timer();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(reads, 2, `${sampleRateHz} Hz does not lose the next fractional-cadence read`);

      now += 5;
      readReleases.shift()();
      await new Promise((resolve) => setImmediate(resolve));
      assert.equal(notifications.length, 2);
      assert.equal(notifications[1].available, true);
      assert.equal(notifications[1].nativeMissedWindows, 0n);
      assert.equal(notifications[1].skippedPublications, 0n);
      assert.ok(spectrum.readLatest());
    } finally {
      await spectrum.close();
      await observation?.close();
    }
  }
});

test("managed spectrum collection updates target and smoothing atomically", async () => {
  const firstQuery = queryFor({ kind: "trackPostMatrix", trackId: "t" });
  const secondQuery = queryFor({ kind: "output", outputId: "out" });
  const collection = {
    entries: [
      { target: firstQuery.target, channels: "both" },
      { target: secondQuery.target, channels: "both" },
    ],
    maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes,
  };
  let activeQuery = firstQuery;
  let activeSmoothing = 0;
  let streamSelects = 0;
  let starts = 0;
  let stops = 0;
  let sequence = 0n;
  const metadata = (query, status = "warming") => Object.freeze({
    result: 0,
    status,
    target: query.target,
    channels: query.channels,
    sampleRateHz: 48_000,
    quantumFrames: 128,
    hopFrames: 2_048,
    sourceUnderrun: false,
    captureEpoch: activeQuery === firstQuery ? 1n : 2n,
    sequence,
    droppedCaptures: 0n,
    windows: sequence,
    capturedSample: sequence * 2_048n,
    endSample: (sequence + 1n) * 2_048n,
    analysisEpoch: activeQuery === firstQuery ? 1n : 2n,
    historyStartSample: 0n,
    smoothingMs: activeSmoothing,
  });
  const spectrumResult = (query) => ({
    target: query.target,
    channels: query.channels,
    sampleRateHz: 48_000,
    windowFrames: 2_048,
    binCount: 1,
    floorDb: -120,
    frequenciesHz: new Float32Array([100]),
    leftDb: new Float32Array([sequence === 0n ? -3 : -6]),
    rightDb: new Float32Array([sequence === 0n ? -4 : -7]),
    capturedSample: sequence * 2_048n,
    endSample: (sequence + 1n) * 2_048n,
    snapshotToken: sequence + 1n,
    graphSourceUnderrun: false,
    resultBytes: 37n,
  });
  const owner = new ObservationSubscriptionOwner({
    observationMap: () => ({ bindings: [] }),
    readObservations: () => [],
    console: () => { throw new Error("unused"); },
    spectrumPreparedCollection: () => collection,
    spectrumSelect: async () => ({ ok: true, result: 0, code: "ok" }),
    spectrumStreamSelect: async (query, smoothingMs) => {
      streamSelects += 1;
      activeQuery = query;
      activeSmoothing = smoothingMs;
      return { ok: true, result: 0, code: "ok", metadata: metadata(query) };
    },
    spectrumStart: async (smoothingMs, query) => {
      starts += 1;
      activeQuery = query;
      activeSmoothing = smoothingMs;
      return { ok: true, result: 0, code: "ok", metadata: metadata(query) };
    },
    spectrumRead: async (query) => {
      sequence += 1n;
      return { metadata: metadata(query, "ready"), result: spectrumResult(query) };
    },
    spectrumStop: async () => {
      stops += 1;
      return { ok: true, result: 0, code: "ok" };
    },
  }, undefined, undefined, { maximumDeliveredBytesPerSecond: 64 * 1024 * 1024 });

  const first = (await owner.subscribeSpectrum({ ...firstQuery, smoothingMs: 0, cadenceMs: 1 })).handle;
  const shared = (await owner.subscribeSpectrum({ ...firstQuery, smoothingMs: 0, cadenceMs: 1 })).handle;
  await assert.rejects(
    shared.update({ ...secondQuery, smoothingMs: 250.5, cadenceMs: 1 }),
    /shared managed subscriber/,
  );
  assert.equal(streamSelects, 0, "a shared refusal must not call native selection");
  assert.equal(starts, 1);
  await shared.close();

  const firstNotification = await first.pump();
  assert.equal(firstNotification.available, true);
  const oldResult = first.readLatest();
  const oldLeft = oldResult.leftDb.slice();
  const update = await first.update({ ...secondQuery, smoothingMs: 250.5, cadenceMs: 1 });
  assert.equal(streamSelects, 1);
  assert.equal(stops, 0, "a target+smoothing update must not stop the native stream");
  assert.deepEqual(update.configuration.target, secondQuery.target);
  assert.equal(update.configuration.smoothingMs, 250.5);
  assert.equal(first.readLatest(), undefined, "a changed selection starts with no old result");

  const secondNotification = await first.pump();
  assert.equal(secondNotification.available, true);
  assert.deepEqual(secondNotification.metadata.target, secondQuery.target);
  assert.equal(secondNotification.metadata.smoothingMs, 250.5);
  assert.deepEqual(first.readLatest().target, secondQuery.target);
  assert.deepEqual(oldResult.leftDb, oldLeft, "the old owned result remains unchanged after switching");
  assert.equal(stops, 0);

  await first.update({ ...secondQuery, smoothingMs: 250.5, cadenceMs: 1 });
  assert.equal(streamSelects, 1, "an exact effective no-op must preserve the active stream");
  await first.close();
  assert.equal(stops, 1);
});

test("managed spectrum capture limits refuse before native activation and leave one-shot usable", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "output", outputId: "out" });
  const engine = await makeEngine(asset, query);
  try {
    await assert.rejects(
      engine.subscribeSpectrum({ ...query, spectrumLimits: { maximumCaptureBytes: 1 } }),
      /serialized window|capture limit|spectrum capture/,
    );
    assert.equal(engine.armSpectrum().ok, true, "a refused managed admission must not orphan native capture state");
    assert.equal(engine.cancelSpectrum().ok, true);
  } finally {
    engine.dispose();
  }
});

test("managed spectrum loss baselines stay monotonic within an epoch and reset on failure", async () => {
  const prepared = queryFor({ kind: "output", outputId: "out" });
  const metadata = (status, sequence, droppedCaptures, captureEpoch = 1n) => Object.freeze({
    result: 0,
    status,
    target: prepared.target,
    channels: "both",
    sampleRateHz: 48_000,
    quantumFrames: 128,
    hopFrames: 2_048,
    sourceUnderrun: false,
    captureEpoch,
    sequence: BigInt(sequence),
    droppedCaptures,
    windows: BigInt(sequence + 1),
    capturedSample: BigInt(sequence * 2_048),
    endSample: BigInt((sequence + 1) * 2_048),
    analysisEpoch: 1n,
    historyStartSample: 0n,
    smoothingMs: 0,
  });
  const result = (sequence) => ({
    target: prepared.target,
    channels: "both",
    sampleRateHz: 48_000,
    windowFrames: 2_048,
    binCount: 1,
    floorDb: -120,
    frequenciesHz: new Float32Array([0]),
    leftDb: new Float32Array([0]),
    rightDb: new Float32Array([0]),
    capturedSample: BigInt(sequence * 2_048),
    endSample: BigInt((sequence + 1) * 2_048),
    snapshotToken: BigInt(sequence + 1),
    graphSourceUnderrun: false,
    resultBytes: 37n,
  });
  const reads = [
    { metadata: metadata("gap", 5, 1n), result: undefined },
    { metadata: metadata("ready", 4, 0n), result: result(4) },
    { metadata: metadata("ready", 6, 1n), result: result(6) },
    { metadata: metadata("pending", 6, 0n), result: undefined },
    { metadata: metadata("failed", 0, 0n, 2n), result: undefined },
    { metadata: metadata("ready", 1, 1n, 2n), result: result(1) },
  ];
  const owner = new ObservationSubscriptionOwner({
    observationMap: () => ({ bindings: [] }),
    readObservations: () => [],
    console: () => { throw new Error("unused"); },
    spectrumPrepared: () => prepared,
    spectrumStart: async () => ({ ok: true, result: 0, code: "ok", metadata: metadata("warming", 0, 0n) }),
    spectrumRead: async () => reads.shift() ?? { metadata: metadata("pending", 1, 0n), result: undefined },
    spectrumStop: async () => ({ ok: true, result: 0, code: "ok" }),
  }, undefined, undefined, {});
  const subscription = (await owner.subscribeSpectrum({ ...prepared, cadenceMs: 1 })).handle;
  const first = await subscription.pump();
  assert.equal(first.nativeMissedWindows, 1n);
  const historical = await subscription.pump();
  assert.equal(historical.nativeMissedWindows, 0n);
  const job = subscription.job;
  const revision = subscription.revision;
  const updated = await subscription.update({ ...prepared, cadenceMs: 101 });
  assert.equal(updated.job, job);
  assert.equal(updated.revision, revision);
  assert.equal(updated.configuration.cadenceMs, 101);
  const recovery = await subscription.pump();
  assert.equal(recovery.nativeMissedWindows, 0n);
  assert.equal(await subscription.pump(), undefined);
  assert.equal((await subscription.pump()).nativeMissedWindows, 0n);
  assert.equal((await subscription.pump()).nativeMissedWindows, 1n);
  await subscription.close();
});

test("automatic spectrum drains once after a gap and preserves coalesced losses", async (t) => {
  let now = 0;
  t.mock.method(Date, "now", () => now);
  const prepared = queryFor({ kind: "output", outputId: "out" });
  const metadata = (status, sequence, droppedCaptures, captureEpoch = 1n) => ({
    result: 0, status, target: prepared.target, channels: "both", sampleRateHz: 48_000,
    quantumFrames: 128, hopFrames: 2_048, sourceUnderrun: false, captureEpoch,
    sequence: BigInt(sequence), droppedCaptures, windows: BigInt(sequence),
    capturedSample: BigInt(sequence * 2_048), endSample: BigInt((sequence + 1) * 2_048),
    analysisEpoch: 1n, historyStartSample: 0n, smoothingMs: 0,
  });
  const read = (status, sequence, drops, epoch = 1n) => ({
    metadata: metadata(status, sequence, drops, epoch),
    ...(status === "ready" ? { result: {
      target: prepared.target, channels: "both", sampleRateHz: 48_000,
      windowFrames: 2_048, binCount: 1, floorDb: -120,
      frequenciesHz: new Float32Array([0]), leftDb: new Float32Array([0]), rightDb: new Float32Array([0]),
      capturedSample: BigInt(sequence * 2_048), endSample: BigInt((sequence + 1) * 2_048),
      snapshotToken: BigInt(sequence), graphSourceUnderrun: false, resultBytes: 37n,
    } } : {}),
  });
  const queued = [];
  const fast = [];
  const slow = [];
  let timer;
  let calls = 0;
  let stopped = false;
  const owner = new ObservationSubscriptionOwner({
    observationMap: () => ({ bindings: [] }), readObservations: () => [],
    console: () => { throw new Error("unused"); }, spectrumPrepared: () => prepared,
    spectrumStart: async () => ({ ok: true, result: 0, code: "ok", metadata: metadata("warming", 0, 0n) }),
    spectrumRead: async () => { calls++; return await queued.shift(); },
    spectrumStop: async () => { stopped = true; return { ok: true, result: 0, code: "ok" }; },
    scheduler: { setInterval: (callback) => { timer = callback; return 1; }, clearInterval: () => {} },
  });
  const fastHandle = (await owner.subscribeSpectrum({ ...prepared, cadenceMs: 1, onUpdate: (n) => fast.push(n) })).handle;
  const slowHandle = (await owner.subscribeSpectrum({ ...prepared, cadenceMs: 100, onUpdate: (n) => slow.push(n) })).handle;
  const tick = async (time) => { now = time; timer(); await new Promise((resolve) => setImmediate(resolve)); };
  queued.push(read("ready", 1, 0n));
  await tick(43);
  assert.equal(slow.length, 1);
  // Native gap reporting leaves its full queue intact; the second read must pop its older window.
  queued.push(read("gap", 1, 3n), read("ready", 2, 0n));
  await tick(86);
  assert.equal(calls, 3);
  assert.equal(fast.at(-1).status, "gap");
  assert.equal(fast.at(-1).nativeMissedWindows, 3n);
  assert.equal(slow.length, 1, "recovery must respect the slower delivery cadence");
  assert.ok(fastHandle.readLatest(), "the immediate drain recovers an owned window");
  queued.push(read("pending", 2, 0n));
  await tick(150);
  assert.equal(slow.at(-1).nativeMissedWindows, 3n, "older queued metadata cannot erase coalesced loss");
  assert.equal(slow.at(-1).skippedPublications, 1n);
  // A second gap in the recovery slot is published, but never recursively drained.
  queued.push(read("gap", 2, 4n), read("gap", 2, 5n));
  await tick(193);
  assert.equal(calls, 6, "one timer poll performs at most two sequential reads");
  queued.push(read("failed", 0, 0n, 2n));
  await tick(260);
  assert.equal(slow.at(-1).nativeMissedWindows, 0n, "a new capture epoch resets undelivered loss");
  await slowHandle.close();
  // The owner retains its one-in-flight guard, including while the recovery read is pending.
  let release;
  queued.push(read("gap", 0, 1n, 2n), new Promise((resolve) => { release = resolve; }));
  await tick(303);
  const pendingCalls = calls;
  await tick(346);
  assert.equal(calls, pendingCalls);
  const closing = fastHandle.close();
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(stopped, false, "close must await the recovery read");
  release(read("ready", 1, 0n, 2n));
  await closing;
  assert.equal(stopped, true);
});

test("managed spectrum admission refuses before start and preserves a working stream on update refusal", async () => {
  const prepared = queryFor({ kind: "output", outputId: "out" });
  const metadata = (target = prepared.target, status = "warming") => Object.freeze({
    result: 0,
    status,
    target,
    channels: "both",
    sampleRateHz: 48_000,
    quantumFrames: 128,
    hopFrames: 1_024,
    sourceUnderrun: false,
    captureEpoch: 1n,
    sequence: 0n,
    droppedCaptures: 0n,
    windows: 0n,
    capturedSample: 0n,
    endSample: 0n,
    analysisEpoch: 0n,
    historyStartSample: 0n,
    smoothingMs: 0,
  });
  const makeOwner = (spectrumSubscriptionLimits, startMetadata = metadata()) => {
    let starts = 0;
    let stops = 0;
    let reads = 0;
    const owner = new ObservationSubscriptionOwner({
      observationMap: () => ({ bindings: [] }),
      readObservations: () => [],
      console: () => { throw new Error("unused"); },
      spectrumPrepared: () => prepared,
      spectrumStart: async () => {
        starts += 1;
        return { ok: true, result: 0, code: "ok", metadata: startMetadata };
      },
      spectrumRead: async () => {
        reads += 1;
        return { metadata: metadata(prepared.target, "pending") };
      },
      spectrumStop: async () => {
        stops += 1;
        return { ok: true, result: 0, code: "ok" };
      },
    }, undefined, undefined, spectrumSubscriptionLimits);
    return { owner, counts: () => ({ starts, stops, reads }) };
  };

  const refused = makeOwner({ maximumDeliveredBytesPerSecond: 1 });
  await assert.rejects(
    refused.owner.subscribeSpectrum({ ...prepared, cadenceMs: 1 }),
    /delivery bound/,
  );
  assert.deepEqual(refused.counts(), { starts: 0, stops: 0, reads: 0 });

  const malformed = makeOwner(undefined, metadata({ kind: "trackPostMatrix", trackId: "other" }));
  await assert.rejects(
    malformed.owner.subscribeSpectrum({ ...prepared, cadenceMs: 1 }),
    /target differs|not prepared/,
  );
  assert.deepEqual(malformed.counts(), { starts: 1, stops: 1, reads: 0 });

  const working = makeOwner();
  const subscription = (await working.owner.subscribeSpectrum({ ...prepared, cadenceMs: 1 })).handle;
  await assert.rejects(
    subscription.update({ ...prepared, target: { kind: "output", outputId: "missing" }, cadenceMs: 1 }),
    /not prepared/,
  );
  assert.deepEqual(working.counts(), { starts: 1, stops: 0, reads: 0 });
  await subscription.pump();
  assert.deepEqual(working.counts(), { starts: 1, stops: 0, reads: 1 });
  await subscription.close();
  assert.deepEqual(working.counts(), { starts: 1, stops: 1, reads: 1 });
});
