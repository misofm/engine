/** Issue #321: direct-Wasm source seek, status/session map, and meter capability parity. */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoControlRefusal, MisoUsageError } from "../src/core/errors.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { decodeControlReply } from "../src/core/control-frame.ts";
import { structuralFrame } from "./control-support.mjs";
import { effectEntry, moduleBytes, observeTaps, ramp, sessionDocument } from "./support.mjs";

/** The parity session: three tracks, t3 behind a multiband compressor, a true-peak limiter and a delay. */
const PARITY = JSON.parse(readFileSync(
  new URL("../../hosts/host-web/tests/fixtures/control-list.json", import.meta.url),
  "utf8",
));

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

/**
 * Issue #1210 gate 2(a): tracks `t0`, `t1`, `t2`; the submixes are declared out of canonical order.
 * `zz-bus` (declared first) sums `t0` and `t1` behind a -40 dB fader; `aa-bus` carries `t2` alone
 * at unity, so its lanes peak well above `zz-bus`'s whatever the source block holds.
 */
function namedBusDocument() {
  const document = JSON.parse(busDocument());
  const [quiet, loud] = document.submixes;
  quiet.id = "zz-bus";
  quiet.fader = { ...quiet.fader, left_db: -40.0, right_db: -40.0 };
  loud.id = "aa-bus";
  document.submixes = [quiet, loud];
  for (const route of document.routes) {
    for (const end of [route.source, route.destination]) {
      if (end.submix_id === "bus-a") end.submix_id = "zz-bus";
      else if (end.submix_id === "bus-b") end.submix_id = "aa-bus";
    }
  }
  return `${JSON.stringify(document, null, 2)}\n`;
}

/**
 * Issue #1210 gate 4: track `t` (compressor insert `comp`) feeds `bus`, which carries its own
 * compressor insert `bus-comp` and feeds the output.
 */
function busObservationDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  assert.ok(compressor);
  const insert = (id) => effectEntry(id, compressor.id, compressor.parameters.map((row) => ({
    id: row.id, unit: row.unitName, value: row.default, channel: "both",
  })));
  const document = JSON.parse(sessionDocument({ effects: { inserts: [insert("comp")] } }));
  const [track] = document.tracks;
  const [route] = document.routes;
  document.submixes = [{
    id: "bus",
    builtins: structuredClone(track.builtins),
    console: [],
    inserts: { effects: [insert("bus-comp")] },
    fader: structuredClone(track.fader),
    pan: structuredClone(track.pan),
  }];
  document.routes = [
    { ...structuredClone(route), id: "t-bus",
      destination: { kind: "submix_input", submix_id: "bus" } },
    { ...structuredClone(route), id: "bus-main",
      source: { kind: "submix", submix_id: "bus", tap: "post_pan" } },
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
      assert.deepEqual(map.submixes, [], "issue #1210: a session without submixes");
    } finally {
      engine.dispose();
    }
  });

  test("status carries the session's count of superseded live values", async () => {
    // Red if the headless status drops `liveValuesSuperseded`: an agent on the headless SDK then
    // cannot read the count that the browser host and the C ABI report (#1399 verdict m1).
    const engine = await createOfflineEngine(sessionDocument(), {
      asset,
      liveControls: { meterBlocks: 0 },
    });
    try {
      assert.equal(engine.status().liveValuesSuperseded, 0n);
      const liveControls = engine.liveControls();
      const track = liveControls.edit.track("t");
      for (const db of [-3, -6]) {
        const report = await liveControls.submit(track.faderDb(db, { channel: "left" }));
        assert.equal(report.ok, true);
      }
      assert.equal(
        engine.status().liveValuesSuperseded,
        0n,
        "render has not read either write: nothing is superseded yet",
      );
      // Two blocks, so the count (1) cannot be mistaken for the rendered-quanta word (2).
      for (const block of [0n, 1n]) {
        feed(engine, 1n, block * BigInt(engine.shape().quantumFrames));
        engine.render();
      }
      assert.equal(engine.status().renderedQuanta, 2n);
      assert.equal(
        engine.status().liveValuesSuperseded,
        1n,
        "the left fader cell was written twice before the block read it: one value replaced",
      );
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

  test("session seek calls the at-once session seek export alone and lands in the next block", async () => {
    // Issue #1643 gate 2, #2205. Red if `seek` does not call the at-once session seek export: the
    // live form keeps the fed source's old generation playing for its runway (the headless engine
    // has no live audio to keep), so the next block is not the twin's target; if it calls no
    // session seek (the generation-2 PCM is then refused); if it declares the discontinuity
    // itself before the seek (root ruling B16-R9 retired B15-R7's order: the plain session seek
    // declares in the same call, so a separate declaration first would reset the floors even
    // when the seek is refused); or if it picks a stale generation.
    const calls = [];
    const instantiate = asset.instantiate.bind(asset);
    asset.instantiate = async () => {
      const instance = await instantiate();
      const exports = {};
      for (const [name, value] of Object.entries(instance.exports)) {
        exports[name] = typeof value === "function" && /declare_discontinuity$|session_seek$|session_prepare_seek$/.test(name)
          ? (...args) => { calls.push(name.replace("miso_engine_web_v1_", "")); return value(...args); }
          : value;
      }
      return { exports };
    };
    let engine;
    let twin;
    try {
      engine = await createOfflineEngine(sessionDocument(), { asset });
      twin = await createOfflineEngine(sessionDocument(), { asset });
    } finally {
      delete asset.instantiate;
    }
    try {
      assert.throws(
        () => engine.seek(-1n),
        (error) => error instanceof MisoUsageError && /non-negative bigint/.test(error.message),
      );
      assert.throws(
        () => engine.seek(0n, { generation: 0n }),
        (error) => error instanceof MisoUsageError && /positive bigint/.test(error.message),
      );
      assert.deepEqual(calls, [], "a usage error reaches no export");

      // Both play generation 1 first, with old PCM still queued at the seek (#2205).
      for (const subject of [engine, twin]) {
        const q = BigInt(subject.shape().quantumFrames);
        for (let block = 0n; block < 4n; block += 1n) feed(subject, 1n, block * q, 5);
        subject.render();
      }
      const seek = engine.seek(4_096n);
      assert.deepEqual(seek, { generation: 2n, ok: true, result: 0, code: "ok" });
      assert.deepEqual(calls, ["session_prepare_seek"], "the at-once form; no separate declaration");
      feed(engine, 2n, 4_096n, 17);
      const block = engine.render();

      // The per-source seek reads the same frame in its first block, so the two outputs match.
      assert.deepEqual(twin.seekSource({ sourceId: "s", generation: 2n, sourceFrame: 4_096n }),
        { ok: true, result: 0, code: "ok" });
      feed(twin, 2n, 4_096n, 17);
      const expected = twin.render();
      assert.ok(block.left.some((sample) => sample !== 0), "the sought frame renders audio");
      assert.deepEqual(block.left, expected.left, "the next block is the target, not the old generation");
      assert.deepEqual(block.right, expected.right);

      // The default generation is one above the largest passed, and a stale one changes nothing.
      // One block between seeks drains each consumer's command slot.
      const settle = (generation, frame) => {
        feed(engine, generation, frame);
        engine.render();
      };
      assert.equal(engine.seek(0n).generation, 3n);
      settle(3n, 0n);
      const stale = engine.seek(0n, { generation: 3n });
      assert.equal(stale.generation, 3n);
      assert.equal(stale.ok, false);
      assert.equal(stale.code, "invalidArgument");
      assert.equal(engine.seek(0n, { generation: 9n }).ok, true);
      settle(9n, 0n);
      assert.equal(engine.seek(0n).generation, 10n, "an explicit generation moves the counter");
      settle(10n, 0n);
      assert.equal(engine.seekSource({ sourceId: "s", generation: 12n, sourceFrame: 0n }).ok, true);
      settle(12n, 0n);
      assert.equal(engine.seek(0n).generation, 13n, "seekSource moves the counter");
      settle(13n, 0n);
      assert.equal(engine.seek(0n).generation, 14n);
    } finally {
      engine?.dispose();
      twin?.dispose();
    }
  });

  test("a plain session seek declares the discontinuity itself, and a refused one declares nothing", async () => {
    // Root ruling B16-R9 gate 4, on the headless path (which has no anchored session seek). The
    // parity session holds a true-peak limiter on t3; removing it rebuilds the plan, and #1285
    // keeps the old latency as floors. Red if the core's plain seek does not
    // declare (the latency stays floored after an accepted seek), if it declares before its check
    // (the refused seek resets the floors), or if the SDK declares separately (the export calls).
    const calls = [];
    let exports;
    let handle;
    const instantiate = asset.instantiate.bind(asset);
    asset.instantiate = async () => {
      const instance = await instantiate();
      exports = {};
      for (const [name, value] of Object.entries(instance.exports)) {
        exports[name] = typeof value === "function" && /declare_discontinuity$|session_seek$|session_prepare_seek$|_render$/.test(name)
          ? (...args) => {
            handle = args[0];
            if (!name.endsWith("_render")) calls.push(name.replace("miso_engine_web_v1_", ""));
            return value(...args);
          }
          : value;
      }
      return { exports };
    };
    let engine;
    try {
      engine = await createOfflineEngine(PARITY.session, {
        asset,
        liveControls: { meterBlocks: 1 },
      });
    } finally {
      delete asset.instantiate;
    }
    const quantum = BigInt(engine.shape().quantumFrames);
    const latency = () => {
      assert.equal(Number(exports.miso_engine_web_v1_plan_latency_read(handle)), 0);
      const record = Number(exports.miso_engine_web_v1_plan_latency_ptr(handle));
      return new DataView(exports.memory.buffer).getBigUint64(record + 8, true);
    };
    const play = (generation, from, blocks, start) => {
      for (let block = 0n; block < blocks; block += 1n) {
        feed(engine, generation, start + block * quantum, 3);
        engine.render();
      }
      return from + blocks;
    };
    try {
      let block = play(1n, 0n, 4n, 0n);
      const boot = latency();
      assert.ok(boot > 0n, "the limiter delays the plan");
      const reply = decodeControlReply(engine.controlFrame(structuralFrame("remove_t3_limiter", engine)));
      assert.equal(reply.status, "ok");
      assert.equal(reply.applied.editPath, "rebuild", "removing the limiter rebuilds the plan");
      block = play(1n, block, 2n, block * quantum);
      assert.equal(latency(), boot, "the floors keep the old latency (#1285)");

      const refused = engine.seek(0n, { generation: 1n });
      assert.equal(refused.code, "invalidArgument", "generation 1 is stale");
      block = play(1n, block, 2n, block * quantum);
      assert.equal(latency(), boot, "a refused plain seek declares nothing");

      const accepted = engine.seek(0n, { generation: 2n });
      assert.equal(accepted.ok, true, accepted.code);
      play(2n, block, 2n, 0n);
      assert.ok(latency() < boot, "an accepted plain seek declares: the floors reset");
      assert.deepEqual(calls, ["session_prepare_seek", "session_prepare_seek"], "the SDK makes no separate declaration");
    } finally {
      engine.dispose();
    }
  });

  test("meter lease and poll return one copied, ABI-shaped sample window", async () => {
    const meterBlocks = 2;
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      liveControls: {
        meterBlocks,
      },
    });
    try {
      // #1689 D6: the master is the strip's `master` row, written after boot.
      const controls = engine.liveControls();
      await controls.submit(controls.edit.track(engine.sessionMap().tracks[0]).master(true));
      assert.equal(engine.sessionMap().metersAttached, true);
      assert.deepEqual(engine.meters(true), { ok: true, result: 0, code: "ok" });

      const arm = observeTaps(engine, [{
        stripId: engine.sessionMap().tracks[0],
        rack: "console",
        effectId: "compressor",
        tapId: 1,
        armed: true,
        windowBlocks: meterBlocks,
      }]);
      assert.equal(arm.status, "ok");

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
        meterBlocks,
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
      // #1209 MINOR-1: values, not only shapes. The three tracks are identical copies of one
      // source and nothing is armed, so bus-a (two tracks) peaks at twice bus-b (one), the master
      // at bus-a + bus-b, and every gain-reduction word is 0. Red if the reader takes the master
      // pair from a bus slot, or reads `trackGrDb` from the old `2T + 2` base (bus peaks as dB).
      const close = (actual, expected) => Math.abs(actual - expected) <= Math.abs(expected) * 1e-5;
      const master = trackCount * 2;
      for (const lane of [0, 1]) {
        assert.ok(close(frame.submixPeaks[lane], 2 * frame.submixPeaks[2 + lane]), `bus-a lane ${lane}`);
        assert.ok(close(frame.peaks[master + lane],
          frame.submixPeaks[lane] + frame.submixPeaks[2 + lane]), `master lane ${lane}`);
      }
      assert.ok(frame.trackGrDb.every((value) => value === 0), "no track effect is armed");
      assert.ok(frame.submixGrDb.every((value) => value === 0), "no bus effect is armed");
    } finally {
      engine.dispose();
    }
  });

  test("the session map names every submix in the frame's bus order", async () => {
    // Issue #1210 gate 2(a). Red if the shipped enumeration orders submixes other than the frame
    // does (the quiet bus's pair would sit where the map says the loud bus is), or if `shape()`
    // does not read the submix exports at all.
    const meterBlocks = 2;
    const engine = await createOfflineEngine(namedBusDocument(), {
      asset,
      liveControls: {
        meterBlocks,
      },
    });
    try {
      const map = engine.sessionMap();
      assert.deepEqual(map.submixes, ["aa-bus", "zz-bus"], "canonical order, not declaration order");
      assert.deepEqual(engine.shape().submixes, map.submixes);
      assert.deepEqual(map.tracks, ["t0", "t1", "t2"]);
      assert.deepEqual(engine.meters(true), { ok: true, result: 0, code: "ok" });
      for (let block = 0; block < meterBlocks; block += 1) {
        feed(engine, 1n, BigInt(block * engine.shape().quantumFrames), 31 + block);
        engine.render();
      }
      const frame = engine.pollMeters();
      assert.ok(frame);
      assert.equal(frame.submixCount, map.submixes.length);
      const pair = (id) => {
        const index = map.submixes.indexOf(id);
        return [frame.submixPeaks[index * 2], frame.submixPeaks[index * 2 + 1]];
      };
      const [loud, quiet] = [pair("aa-bus"), pair("zz-bus")];
      for (const lane of [0, 1]) {
        assert.ok(quiet[lane] > 0, "the quiet bus is metered");
        assert.ok(loud[lane] > 10 * quiet[lane], `lane ${lane}: ${loud} vs ${quiet}`);
      }
    } finally {
      engine.dispose();
    }
  });

  test("a bus effect's observation names its submix and a track tap still reads", async () => {
    // Issue #1210 gate 4. Red if the SDK enriches or resolves observation bindings against the
    // tracks only: the bus binding's strip index is past the track list, so the map throws
    // `sdk.observation.map` and every read, the track's own included, fails with it.
    const engine = await createOfflineEngine(busObservationDocument(), {
      asset,
    });
    try {
      const map = engine.observationMap();
      const owners = map.bindings.map((binding) => [binding.trackId, binding.effectSlotId]);
      assert.deepEqual(owners.filter(([trackId]) => trackId === "bus"), [["bus", "bus-comp"]]);
      assert.deepEqual(owners.filter(([trackId]) => trackId === "t"), [["t", "comp"]]);
      const read = (trackId, effectSlotId) => {
        const binding = map.bindings.find((row) => row.trackId === trackId
          && row.effectSlotId === effectSlotId);
        assert.ok(binding);
        return engine.readObservations([{
          trackId, rack: binding.rack, effectSlotId, tapId: binding.tapIds[0], channels: "both",
        }]);
      };
      const [track] = read("t", "comp");
      assert.equal(track.trackId, "t");
      assert.equal(track.status, "unarmed");
    } finally {
      engine.dispose();
    }
  });

  test("a selected read of a bus tap names the bus, unarmed and then armed by its stable key", async () => {
    // Issue #1213 (K2 verdict requirement (a)). The SDK resolves a bus tap to its strip index
    // `T + j`, and host-web must resolve that index per strip -- tracks, then submixes. Red if the
    // host looks the index up among the tracks only: the read then fails with `invalidArgument`.
    // The arm is kind 7 at the bus's strip index through the shipped module.
    const engine = await createOfflineEngine(busObservationDocument(), {
      asset,
    });
    try {
      const map = engine.sessionMap();
      const busIndex = map.tracks.length + map.submixes.indexOf("bus");
      assert.equal(busIndex, 1, "strip order: the track, then the bus");
      const binding = engine.observationMap().bindings.find((row) => row.trackId === "bus");
      assert.ok(binding);
      const selection = [{
        trackId: "bus", rack: binding.rack, effectSlotId: "bus-comp", tapId: binding.tapIds[0],
        channels: "both",
      }];
      const [unarmed] = engine.readObservations(selection);
      assert.equal(unarmed.trackId, "bus");
      assert.equal(unarmed.effectSlotId, "bus-comp");
      assert.equal(unarmed.status, "unarmed");

      const arm = observeTaps(engine, [{
        stripId: "bus",
        rack: binding.rack,
        effectId: "bus-comp",
        tapId: binding.tapIds[0],
        armed: true,
        windowBlocks: 1,
      }]);
      assert.equal(arm.status, "ok");
      for (let block = 0; block < 4; block += 1) {
        feed(engine, 1n, BigInt(block * engine.shape().quantumFrames), 41 + block);
        engine.render();
      }
      const [armed] = engine.readObservations(selection);
      assert.equal(armed.trackId, "bus");
      assert.notEqual(armed.status, "unarmed", "kind 7 naming the bus's strip armed its tap");
    } finally {
      engine.dispose();
    }
  });

  test("a managed subscription to a bus tap arms it at the bus's strip index and reads it ready", async () => {
    // #1214 MINOR-1. The managed owner arms and disarms through `edit.strip(trackId)`. Red if it
    // builds the edit with `edit.track()`: a bus is not a track, so the subscription throws
    // "no track 'bus'" before anything is submitted.
    const engine = await createOfflineEngine(busObservationDocument(), {
      asset,
    });
    try {
      const binding = engine.observationMap().bindings.find((row) => row.trackId === "bus");
      assert.ok(binding);
      const selection = {
        trackId: "bus", rack: binding.rack, effectSlotId: "bus-comp", tapId: binding.tapIds[0],
        channels: "both",
      };
      const subscription = await engine.subscribeObservations({ selections: [selection], windowBlocks: 1 });
      for (let block = 0; block < 2; block += 1) {
        feed(engine, 1n, BigInt(block * engine.shape().quantumFrames), 61 + block);
        engine.render();
        await subscription.pump();
      }
      const [row] = subscription.readLatest();
      assert.equal(row.trackId, "bus");
      assert.equal(row.effectSlotId, "bus-comp");
      assert.equal(row.status, "ready");
      await subscription.close();
      assert.equal(engine.readObservations([selection])[0].status, "unarmed", "close disarms the bus tap");
    } finally {
      engine.dispose();
    }
  });

  test("a bad insert on a bus and an unknown strip are the core's one address refusal", async () => {
    // Issue #1213 D5a, as #1563B restates it: the address names a strip by ID, so a submix is a
    // strip like a track. Red if the core's address lookup skips submix strips (the bus's own
    // fader would then be unknown), or answers a missing insert on the bus with another code.
    const engine = await createOfflineEngine(busObservationDocument(), { asset });
    try {
      const controls = engine.liveControls();
      const code = (set) => controls.submit(set).then(() => "applied", (error) => error.code);
      assert.equal(await code(controls.edit.submix("bus").faderDb(-6)), "applied", "the bus is a strip");
      assert.equal(
        await code(controls.edit.submix("bus").insert("ghost", "miso.compressor").parameter("threshold", -20)),
        "control.address.unknown",
        "the bus has no such insert",
      );
      assert.equal(await code(controls.edit.submix("nope").faderDb(-6)), "control.address.unknown");
    } finally {
      engine.dispose();
    }
  });

  test("a solo of a bus is refused by the core", async () => {
    // Issue #1213 D4 (and #1212 NIT-2), as #1563B restates it: a bus is solo-safe, and the core
    // refuses a solo at a submix from the real address table rather than a mocked reply. Red if a
    // bus can be soloed.
    const engine = await createOfflineEngine(busObservationDocument(), { asset });
    try {
      const controls = engine.liveControls();
      // `track()` does not check the ID, so it can name the bus; `submix()` has no `solo` at all.
      const refusal = await controls.submit(controls.edit.track("bus").solo(true)).catch((error) => error);
      assert.ok(refusal instanceof MisoControlRefusal);
      assert.equal(refusal.code, "control.address.unknown");
      assert.equal(refusal.editIndex, 0);
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
