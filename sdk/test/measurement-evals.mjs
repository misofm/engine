import assert from "node:assert/strict";
import test from "node:test";

import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createMeasurementFeeds } from "../src/browser/measurement.ts";

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
  return {
    leases,
    get meterListener() { return meterListener; },
    get telemetryListener() { return telemetryListener; },
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
  };
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

test("measurement admission without a console is a typed SDK refusal", async () => {
  const feeds = createMeasurementFeeds(fakeHost(), ["track"], false);
  await assert.rejects(feeds.meters(() => undefined), (error) => error instanceof MisoUsageError);
  await assert.rejects(feeds.telemetry(() => undefined), (error) => error instanceof MisoUsageError);
  feeds.close();
});
