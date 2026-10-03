/** Issue #321: direct-Wasm source seek, status/session map, and meter capability parity. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { LiveControlWriter } from "../src/core/writer.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function feed(engine, generation, startFrame, seed = 1) {
  const shape = engine.shape();
  for (const [sourceIndex, source] of shape.sources.entries()) {
    const planes = Array.from({ length: source.channels }, (_unused, channel) =>
      ramp(shape.quantumFrames, seed + sourceIndex * 16 + channel));
    const result = engine.submitSource({
      sourceId: source.id,
      generation,
      startFrame,
      planes,
      endOfRegion: false,
    });
    assert.equal(result.ok, true, result.code);
  }
}

function compressorDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  assert.ok(compressor);
  return sessionDocument({
    effects: {
      preInsert: [effectEntry(
        "compressor",
        compressor.id,
        compressor.parameters.map((row) => ({
          id: row.id,
          unit: row.unitName,
          value: row.default,
          channel: "both",
        })),
      )],
    },
  });
}

/** Issue #1209 gate 2: tracks `t0`, `t1`, `t2`; `t0` and `t1` feed `bus-a`, `t2` feeds `bus-b`. */
function busDocument() {
  const document = JSON.parse(sessionDocument());
  const [track] = document.tracks;
  const [route] = document.routes;
  document.tracks = ["t0", "t1", "t2"].map((id) => ({ ...structuredClone(track), id }));
  document.submixes = ["bus-a", "bus-b"].map((id) => ({
    id,
    builtins: structuredClone(track.builtins),
    console: [],
    inserts: { effects: [] },
    fader: structuredClone(track.fader),
    pan: structuredClone(track.pan),
  }));
  const feed = (id, trackId, submixId) => ({
    ...structuredClone(route),
    id,
    source: { kind: "track", track_id: trackId, tap: "post_pan" },
    destination: { kind: "submix_input", submix_id: submixId },
  });
  const out = (id, submixId) => ({
    ...structuredClone(route),
    id,
    source: { kind: "submix", submix_id: submixId, tap: "post_pan" },
  });
  document.routes = [
    feed("t0-a", "t0", "bus-a"), feed("t1-a", "t1", "bus-a"), feed("t2-b", "t2", "bus-b"),
    out("a-main", "bus-a"), out("b-main", "bus-b"),
  ];
  return `${JSON.stringify(document, null, 2)}\n`;
}

describe("issue 321 -- complete headless ABI capability parity", () => {
  test("status and sessionMap expose the compiled addressing authority", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const status = engine.status();
      assert.equal(status.stateName, "ready");
      assert.equal(status.lastResultName, "ok");
      assert.equal(status.backendName, "simd128");
      assert.equal(status.sampleRateHz, engine.shape().sampleRateHz);
      assert.equal(status.quantumFrames, engine.shape().quantumFrames);
      assert.equal(status.nextAbsoluteSample, 0n);
      assert.equal(status.renderedQuanta, 0n);
      assert.ok(status.memoryBytes > 0);

      const map = engine.sessionMap();
      assert.deepEqual(map.tracks, ["t"]);
      assert.deepEqual(map.sources, [{ id: "s", channels: 2, frames: 4_800n }]);
      assert.equal(map.metersAttached, false);
    } finally {
      engine.dispose();
    }
  });

  test("seek changes generation/frame and the source remains renderable", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      assert.throws(
        () => engine.seekSource({ sourceId: "s", generation: 0n, sourceFrame: 0n }),
        (error) => error instanceof MisoUsageError && /positive bigint/.test(error.message),
      );
      assert.throws(
        () => engine.seekSource({ sourceId: "s", generation: 2n, sourceFrame: -1n }),
        (error) => error instanceof MisoUsageError && /non-negative bigint/.test(error.message),
      );

      const seek = engine.seekSource({ sourceId: "s", generation: 2n, sourceFrame: 256n });
      assert.deepEqual(seek, { ok: true, result: 0, code: "ok" });
      feed(engine, 2n, 256n, 17);
      const block = engine.render();
      assert.equal(block.left.length, engine.shape().quantumFrames);
      assert.equal(engine.renderedQuanta(), 1n);
    } finally {
      engine.dispose();
    }
  });

  test("meter lease and poll return one copied, ABI-shaped sample window", async () => {
    const meterBlocks = 2;
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      liveControls: {
        commandQueueRecords: ABI_LAYOUT.constants.defaultCommandQueueRecords,
        meterBlocks,
        observationTaps: 1,
        masterTrackPlusOne: 1,
      },
    });
    try {
      assert.equal(engine.sessionMap().metersAttached, true);
      assert.deepEqual(engine.meters(true), { ok: true, result: 0, code: "ok" });

      const writer = new LiveControlWriter({
        submit: (records, count) => engine.submitCommands(records, count),
      });
      writer.stage({
        kind: "observeSubscribe",
        trackIndex: 0,
        rack: 3, // console slot 0
        channel: 255,
        effectIndex: 0,
        parameterId: 1,
        smoothingSamples: meterBlocks,
        values: [0, 0, 0, 0],
      });
      assert.equal((await writer.flush()).admitted, 1);

      for (let block = 0; block < meterBlocks; block += 1) {
        feed(engine, 1n, BigInt(block * engine.shape().quantumFrames), 31 + block);
        engine.render();
      }
      const frame = engine.pollMeters();
      assert.ok(frame);
      assert.equal(frame.tag, "miso.meter.v1");
      assert.equal(frame.sequence, 1n);
      assert.equal(frame.windows, 1);
      assert.equal(frame.trackCount, 1);
      assert.equal(frame.peaks.length, 4, "2T + 2 peak words");
      assert.equal(frame.trackGrDb.length, 1, "T gain-reduction words");
      assert.equal(typeof frame.masterGrDb, "number");
      assert.equal(frame.firstSample, 0n);
      assert.equal(frame.endSample, BigInt(meterBlocks * engine.shape().quantumFrames));
      assert.ok(frame.peaks.every(Number.isFinite));
      assert.ok(frame.trackGrDb.every((value) => Number.isFinite(value) && value >= 0));

      const historicalPeaks = frame.peaks.slice();
      for (let block = 0; block < meterBlocks; block += 1) {
        const start = BigInt((meterBlocks + block) * engine.shape().quantumFrames);
        feed(engine, 1n, start, 97 + block);
        engine.render();
      }
      assert.ok(engine.pollMeters());
      assert.deepEqual(frame.peaks, historicalPeaks, "a returned frame is detached from Wasm");

      assert.deepEqual(engine.meters(false), { ok: true, result: 0, code: "ok" });
      assert.equal(engine.pollMeters(), undefined);
    } finally {
      engine.dispose();
    }
  });

  test("a frame with submixes keeps the track shape and carries every bus", async () => {
    // Issue #1209 gate 2: `T = 3`, `S = 2` through the shipped module. Red if the headless reader
    // keeps the `3T + 3` shape check and refuses a frame with buses, or drops its submix sections.
    const meterBlocks = 2;
    const engine = await createOfflineEngine(busDocument(), {
      asset,
      liveControls: {
        commandQueueRecords: ABI_LAYOUT.constants.defaultCommandQueueRecords,
        meterBlocks,
        observationTaps: 0,
        masterTrackPlusOne: 0,
      },
    });
    try {
      assert.deepEqual(engine.meters(true), { ok: true, result: 0, code: "ok" });
      for (let block = 0; block < meterBlocks; block += 1) {
        feed(engine, 1n, BigInt(block * engine.shape().quantumFrames), 31 + block);
        engine.render();
      }
      const frame = engine.pollMeters();
      assert.ok(frame);
      const trackCount = engine.shape().tracks.length;
      assert.equal(frame.trackCount, trackCount);
      assert.equal(trackCount, 3);
      assert.equal(frame.peaks.length, trackCount * 2 + 2, "2T + 2 peak words");
      assert.equal(frame.trackGrDb.length, trackCount, "T gain-reduction words");
      assert.equal(frame.submixCount, 2);
      assert.equal(frame.submixPeaks.length, frame.submixCount * 2, "2S bus peak words");
      assert.equal(frame.submixGrDb.length, frame.submixCount, "S bus gain-reduction words");
      for (const values of [frame.peaks, frame.trackGrDb, frame.submixPeaks, frame.submixGrDb]) {
        assert.ok(values.every((value) => Number.isFinite(value) && value >= 0), String(values));
      }
      assert.ok(frame.submixPeaks.every((value) => value > 0), "every bus lane is metered");
      assert.ok(frame.peaks.every((value) => value > 0), "every track lane and the master");
    } finally {
      engine.dispose();
    }
  });

  test("a session prepared without meters refuses the lease as unsupported", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const result = engine.meters(true);
      assert.equal(result.ok, false);
      assert.equal(result.code, "unsupported");
      assert.equal(engine.pollMeters(), undefined);
    } finally {
      engine.dispose();
    }
  });
});
