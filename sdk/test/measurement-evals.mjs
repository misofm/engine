import assert from "node:assert/strict";
import test from "node:test";

import { createEngine } from "../src/browser/engine.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createMeasurementFeeds } from "../src/browser/measurement.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";

const meterFrame = () => ({
  tag: "miso.meter.v1",
  sequence: 7,
  generation: 3n,
  validity: 0xb,
  lossCount: 2,
  windows: 3,
  trackCount: 2,
  peaks: new Float32Array([0.1, 0.2, 0.3, 0.4, 0.9, 0.8]),
  trackGrDb: new Float32Array([1.5, 2]),
  masterGrDb: null,
  firstSample: 128n,
  endSample: 512n,
});

const telemetryFrame = {
  tag: "miso.telemetry.v1",
  sequence: 8,
  blocks: 3,
  cpuPercent: 20,
  peakBlockMs: 0.7,
  meanBlockMs: 0.5,
  budgetMs: 2.6,
  deadlineMisses: 1,
  resolutionMs: 0.1,
  belowResolution: false,
};

function fakeHost() {
  const leases = [];
  let meterListener = null;
  let telemetryListener = null;
  let meterResult = 0;
  let telemetryResult = 0;
  let disposed = false;
  return {
    leases,
    get meterListener() { return meterListener; },
    get telemetryListener() { return telemetryListener; },
    get disposed() { return disposed; },
    set meterResult(value) { meterResult = value; },
    async meters(request) {
      leases.push(["meters", request.enabled]);
      meterListener = request.onFrame;
      return { result: meterResult };
    },
    async telemetry(request) {
      leases.push(["telemetry", request.enabled]);
      telemetryListener = request.onFrame;
      return { result: telemetryResult };
    },
    emitMeter(frame) { meterListener?.(frame); },
    emitTelemetry(frame) { telemetryListener?.(frame); },
    async dispose() { disposed = true; },
  };
}

function browserContext(onClose = () => {}) {
  return {
    sampleRate: 48_000,
    renderQuantumSize: 128,
    state: "suspended",
    audioWorklet: { async addModule() {} },
    async close() { onClose(); },
  };
}

function openBrowser(host, policy = { console: { commandQueueRecords: 8, meterBlocks: 1 } }) {
  return createEngine({
    document: "opaque",
    policy,
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: 512,
      backend: "simd128",
      sources: [],
      tracks: ["t"],
    }),
    createContext: () => browserContext(),
    createHost: async () => host,
  });
}

test("SDK measurement feeds use one shared lease and preserve copied projections", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, ["snare", "kick"], true);
  const firstUpdates = [];
  const secondUpdates = [];
  const stopFirst = await feeds.meters((update) => firstUpdates.push(update));
  const stopSecond = await feeds.meters((update) => secondUpdates.push(update));
  assert.deepEqual(host.leases, [["meters", true]]);

  const source = meterFrame();
  host.emitMeter(source);
  assert.equal(firstUpdates.length, 1);
  assert.equal(secondUpdates.length, 1);
  const update = firstUpdates[0];
  assert.equal(update.sequence, 7n);
  assert.equal(update.generation, 3n);
  assert.equal(update.validity, 0xb);
  assert.equal(update.lossCount, 2);
  assert.equal(update.windows, 3);
  assert.equal(update.firstSample, 128n);
  assert.equal(update.endSample, 512n);
  assert.deepEqual([...update.tracks].map(([id, meter]) => [id, {
    ...meter,
    peakLeft: Number(meter.peakLeft.toFixed(6)),
    peakRight: Number(meter.peakRight.toFixed(6)),
  }]), [
    ["snare", { peakLeft: 0.1, peakRight: 0.2, gainReductionDb: 1.5 }],
    ["kick", { peakLeft: 0.3, peakRight: 0.4, gainReductionDb: 2 }],
  ]);
  assert.deepEqual({ ...update.master,
    peakLeft: Number(update.master.peakLeft.toFixed(6)),
    peakRight: Number(update.master.peakRight.toFixed(6)),
  }, { peakLeft: 0.9, peakRight: 0.8, gainReductionDb: null });

  source.peaks[0] = 0.99;
  source.trackGrDb[0] = 99;
  host.emitMeter({ ...meterFrame(), sequence: 8, peaks: new Float32Array([0.5, 0.6, 0.7, 0.8, 0.1, 0.2]) });
  assert.equal(Number(update.tracks.get("snare")?.peakLeft.toFixed(6)), 0.1, "prior projection is independent of later host frames");
  assert.equal(update.tracks.get("snare")?.gainReductionDb, 1.5);
  assert.equal(firstUpdates[1]?.tracks.get("snare")?.peakLeft, 0.5);

  stopFirst(); stopFirst();
  assert.deepEqual(host.leases, [["meters", true]]);
  stopSecond(); stopSecond();
  await Promise.resolve();
  assert.deepEqual(host.leases, [["meters", true], ["meters", false]]);
  feeds.close();
});

test("throwing one SDK listener does not suppress another listener", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, ["track"], true);
  let delivered = 0;
  await feeds.meters(() => { throw new Error("consumer callback"); });
  await feeds.meters(() => { delivered += 1; });
  host.emitMeter({ ...meterFrame(), trackCount: 1, peaks: new Float32Array([0.1, 0.2, 0.9, 0.8]), trackGrDb: new Float32Array([1]) });
  assert.equal(delivered, 1);
  feeds.close();
});

test("identical listener functions retain independent subscriptions", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, ["track"], true);
  let delivered = 0;
  const listener = () => { delivered += 1; };
  const stopFirst = await feeds.meters(listener);
  const stopSecond = await feeds.meters(listener);
  stopFirst(); await Promise.resolve();
  assert.deepEqual(host.leases, [["meters", true]]);
  host.emitMeter({ ...meterFrame(), trackCount: 1, peaks: new Float32Array([0.1, 0.2, 0.9, 0.8]), trackGrDb: new Float32Array([1]) });
  assert.equal(delivered, 1, "the second identical registration remains live");
  stopSecond(); await Promise.resolve();
  assert.deepEqual(host.leases, [["meters", true], ["meters", false]]);
  feeds.close();
});

test("telemetry preserves every host measurement and shares its lease", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, ["track"], true);
  const updates = [];
  const stop = await feeds.telemetry((update) => updates.push(update));
  host.emitTelemetry(telemetryFrame);
  const { tag: _tag, ...telemetryResult } = telemetryFrame;
  assert.deepEqual(updates[0], { ...telemetryResult, sequence: 8n });
  stop(); await Promise.resolve();
  assert.deepEqual(host.leases, [["telemetry", true], ["telemetry", false]]);
  feeds.close();
});

test("a lease refusal is typed and does not reserve a later subscriber", async () => {
  const host = fakeHost();
  host.meterResult = 7;
  const feeds = createMeasurementFeeds(host, ["track"], true);
  await assert.rejects(
    feeds.meters(() => undefined),
    (error) => error instanceof MisoEngineError && error.code === "unsupported" && error.result === 7,
  );
  host.meterResult = 0;
  const stop = await feeds.meters(() => undefined);
  assert.deepEqual(host.leases, [["meters", true], ["meters", true]]);
  stop(); await Promise.resolve();
  assert.deepEqual(host.leases, [["meters", true], ["meters", true], ["meters", false]]);
  feeds.close();
});

test("a rejected numeric lease result is typed and recoverable", async () => {
  const host = fakeHost();
  const originalMeters = host.meters;
  let rejectOnce = true;
  host.meters = async (request) => {
    if (request.enabled && rejectOnce) {
      rejectOnce = false;
      throw Object.assign(new Error("backpressure"), { result: 6 });
    }
    return originalMeters(request);
  };
  const feeds = createMeasurementFeeds(host, ["track"], true);
  await assert.rejects(
    feeds.meters(() => undefined),
    (error) => error instanceof MisoEngineError && error.code === "backpressure" && error.result === 6,
  );
  const stop = await feeds.meters(() => undefined);
  assert.deepEqual(host.leases, [["meters", true]]);
  stop(); await Promise.resolve();
  assert.deepEqual(host.leases, [["meters", true], ["meters", false]]);
  feeds.close();
});

test("measurement admission without a console is a typed SDK refusal", async () => {
  const feeds = createMeasurementFeeds(fakeHost(), ["track"], false);
  await assert.rejects(feeds.meters(() => undefined), (error) => error instanceof MisoUsageError);
  await assert.rejects(feeds.telemetry(() => undefined), (error) => error instanceof MisoUsageError);
  feeds.close();
});

test("closing an already-armed feed rejects same-turn admission", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, ["track"], true);
  await feeds.meters(() => undefined);
  const admission = feeds.meters(() => undefined);
  feeds.close();
  await assert.rejects(admission, (error) => error instanceof MisoUsageError && /closed/.test(error.message));
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.deepEqual(host.leases, [["meters", true], ["meters", false]]);
});

test("engine close clears measurement callbacks and releases a late successful arm", async () => {
  const host = fakeHost();
  let settleArm;
  let lateListener;
  const originalMeters = host.meters;
  host.meters = async (request) => {
    if (request.enabled) {
      host.leases.push(["meters", true]);
      lateListener = request.onFrame;
      return new Promise((resolve) => { settleArm = () => resolve({ result: 0 }); });
    }
    return originalMeters(request);
  };
  const engine = await openBrowser(host);
  let delivered = 0;
  const admission = engine.subscribeMeters(() => { delivered += 1; });
  await Promise.resolve();
  const closing = engine.close();
  await assert.rejects(engine.subscribeMeters(() => undefined), (error) => error instanceof MisoUsageError);
  lateListener?.({ ...meterFrame(), trackCount: 1, peaks: new Float32Array([0.1, 0.2, 0.9, 0.8]), trackGrDb: new Float32Array([1]) });
  assert.equal(delivered, 0, "close clears listeners before a late arm can publish");
  await closing;
  assert.equal(host.disposed, true);
  await assert.rejects(admission, (error) => error instanceof MisoUsageError && /closed/.test(error.message));
  settleArm();
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.deepEqual(host.leases, [["meters", true], ["meters", false]]);
  lateListener?.({ ...meterFrame(), trackCount: 1, peaks: new Float32Array([0.1, 0.2, 0.9, 0.8]), trackGrDb: new Float32Array([1]) });
  assert.equal(delivered, 0, "a late successful lease cannot revive a closed owner");
  await engine.close();
});

test("browser console retains the managed observation conflict hook", async () => {
  const host = fakeHost();
  const commands = [];
  Object.assign(host, {
    async sessionMap() {
      return {
        tag: "miso.sessionmap.v1", result: 0, tracks: ["t"], sources: [], metersAttached: true,
      };
    },
    async command(request) {
      commands.push(request.commands);
      return {
        tag: "miso.ack.v1", result: 0, reason: 0, rejectedIndex: 0,
        admitted: request.commands.length, appliedAtSample: 0n,
      };
    },
    async observationMap() {
      return {
        tag: "miso.observationmap.v1", result: 0,
        bindings: [{ trackIndex: 0, rack: 1, effectIndex: 0, effectSlotId: "comp",
          nativeEffectId: "miso.compressor", tapIds: [1] }],
      };
    },
    async readObservations(request) {
      return {
        tag: "miso.observation.v1", result: 0,
        rows: request.selections.map((selection) => ({ ...selection, status: 2,
          sampleRateHz: 48_000, firstSample: 0n, endSample: 0n, sequence: 0n, blocks: 0,
          leftPresent: 0, rightPresent: 0, left: 0, right: 0 })),
      };
    },
  });
  const engine = await openBrowser(host, {
    console: { commandQueueRecords: 8, meterBlocks: 1, observationTaps: 1 },
  });
  try {
    const managed = await engine.subscribeObservations({
      selections: [{ trackId: "t", rack: "dynamic", effectSlotId: "comp", tapId: 1, channels: "both" }],
      windowBlocks: 1,
    });
    const console = await engine.console();
    const manual = console.edit.track("t").effect("dynamic", 0, "miso.compressor")
      .observe("Gain Reduction", false, 1);
    await assert.rejects(console.submit(manual), /conflict/);
    assert.equal(commands.length, 1, "the manual conflicting edit is stopped by the existing hook");
    await managed.close();
    assert.equal(commands.length, 2, "managed close still uses the owner bypass");
    assert.equal(commands[0][0].kind, ABI_LAYOUT.constants.wireCommandKinds.find((row) => row.name === "observeSubscribe").value);
    assert.equal(commands[1][0].kind, ABI_LAYOUT.constants.wireCommandKinds.find((row) => row.name === "observeUnsubscribe").value);
  } finally {
    await engine.close();
  }
});


test("telemetry shares one lease until its last subscription and isolates throwing callbacks", async () => {
  const host = fakeHost();
  const feeds = createMeasurementFeeds(host, [], true);
  let deliveries = 0;
  const first = await feeds.telemetry(() => { throw new Error("consumer"); });
  const second = await feeds.telemetry(() => { deliveries += 1; });
  assert.deepEqual(host.leases, [["telemetry", true]]);
  host.emitTelemetry(telemetryFrame);
  assert.equal(deliveries, 1);
  first(); first();
  host.emitTelemetry(telemetryFrame);
  assert.equal(deliveries, 2);
  assert.deepEqual(host.leases, [["telemetry", true]]);
  second(); second();
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.deepEqual(host.leases, [["telemetry", true], ["telemetry", false]]);
  feeds.close();
});

test("close inside a measurement callback suppresses remaining same-frame callbacks", async () => {
  const host = fakeHost();
  const engine = await openBrowser(host);
  let closing;
  await engine.subscribeTelemetry(() => { closing = engine.close(); });
  await engine.subscribeTelemetry(() => assert.fail("callback after close started"));
  // Callback exceptions are intentionally isolated, so assert delivery outside the callback.
  let afterClose = 0;
  await engine.subscribeTelemetry(() => { afterClose += 1; });
  host.emitTelemetry(telemetryFrame);
  assert.equal(afterClose, 0);
  await closing;
  assert.equal(host.disposed, true);
});
