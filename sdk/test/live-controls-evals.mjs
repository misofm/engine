/** Issue #322: shared semantic live controls and one whole-batch contract over both transports. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { createBrowserLiveControls } from "../src/browser/live-controls.ts";
import { EngineLiveControls, LiveControlEdits } from "../src/core/live-controls.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { effect, session } from "../src/core/session.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function compressorDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
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

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 71 + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

function encodeBrowserCommands(commands) {
  const records = new Uint8Array(commands.length * ABI_LAYOUT.commandRecord.bytes);
  const view = new DataView(records.buffer);
  const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
  for (const [index, command] of commands.entries()) {
    const base = index * ABI_LAYOUT.commandRecord.bytes;
    view.setUint8(base + at("kind"), command.kind);
    view.setUint8(base + at("rack"), command.rack);
    view.setUint8(base + at("channel"), command.channel);
    view.setUint32(base + at("trackIndex"), command.trackIndex, true);
    view.setUint32(base + at("effectIndex"), command.effectIndex, true);
    view.setUint32(base + at("parameterId"), command.parameterId, true);
    view.setUint32(base + at("smoothingSamples"), command.smoothingSamples, true);
    command.values.forEach((value, valueIndex) => {
      view.setFloat32(base + at("values") + valueIndex * 4, value, true);
    });
  }
  return records;
}

describe("issue 322 -- shared semantic live controls", () => {
  test("all twelve command kinds are built by name and admitted by live Wasm", async () => {
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64, meterBlocks: 2, observationTaps: 1 },
    });
    try {
      const liveControls = engine.liveControls();
      const track = liveControls.edit.track("t");
      const compressor = track.effect("console", 0, "miso.compressor");
      const first = await liveControls.submit(
        track.pan(-0.5, 0.5, { smoothingSamples: 16 }),
        track.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }),
        track.faderDb(-3, { channel: "both" }),
        track.mute(false, { channel: "left" }),
        track.solo(true),
        track.trimDb(1.5, { channel: "right" }),
        track.polarityInvert(false, { channel: "both" }),
        compressor.parameter("threshold", -24, { channel: "both" }),
        compressor.bypass(false),
        compressor.observe("Gain Reduction", true, 2),
        track.inputFilters({ hpfHz: 80, lpfHz: 12_000 }),
      );
      assert.equal(first.ok, true);
      assert.equal(first.admitted, 11);
      assert.equal(first.reasonName, "none");
      assert.equal(first.appliedAtSample, 0n);

      feed(engine, 0);
      engine.render();
      const unsubscribe = await liveControls.submit(
        compressor.observe("Gain Reduction", false, 2),
      );
      assert.equal(unsubscribe.ok, true);
      assert.equal(unsubscribe.admitted, 1);
      assert.equal(unsubscribe.appliedAtSample, 128n);

      const kindNames = [
        "pan", "matrix", "faderDb", "mute", "effectParam", "effectBypass",
        "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert", "inputFilters",
      ];
      // Issue #1222 added the three send kinds to the wire; the SDK builds them by name from
      // issue #1223, which moves them into `kindNames`. Until then they are named here, so any
      // other kind without a semantic method still turns this red.
      const kindsAwaitingSdk = ["routeGainDb", "routeMute", "routeMatrix"];
      assert.deepEqual(
        [...ABI_LAYOUT.constants.wireCommandKinds.map((row) => row.name)].sort(),
        [...kindNames, ...kindsAwaitingSdk].sort(),
        "the semantic methods cover the generated command vocabulary exactly",
      );

      feed(engine, 1);
      engine.render();
      const refused = await liveControls.submit(
        liveControls.edit.track("t").effect("console", 99, "miso.compressor").bypass(true),
      );
      assert.equal(refused.ok, false);
      assert.equal(refused.admitted, 0);
      assert.equal(refused.reasonName, "unknownEffect");

      const recovery = await liveControls.submit(track.faderDb(-6));
      assert.equal(recovery.ok, true, "a typed refusal is per request, not terminal");
    } finally {
      engine.dispose();
    }
  });

  test("unknown tracks and numeric domains refuse before transport", async () => {
    let calls = 0;
    const liveControls = new EngineLiveControls(
      { tracks: ["t"], sources: [], metersAttached: false, submixes: [] },
      () => {
        calls += 1;
        throw new Error("must not be called");
      },
    );
    assert.throws(() => liveControls.edit.track("missing"), MisoUsageError);
    assert.throws(() => liveControls.edit.track("t").faderDb(Number.NaN), /finite/);
    const hpf = liveControls.edit.track("t").hpfHz(80, { channel: "left" });
    const pair = liveControls.edit.track("t").inputFilters({ hpfHz: 80, lpfHz: 12_000 });
    assert.equal(hpf.kind, "inputFilters");
    assert.equal(hpf.parameterId, 3);
    assert.equal(pair.kind, "inputFilters");
    assert.deepEqual(pair.values, [80, 12_000, 0, 0]);
    assert.equal(calls, 0, "authoring a filter edit does not submit it");
    assert.throws(() => liveControls.edit.track("t").pan(-2, 0), /at least -1/);
    assert.throws(
      () => liveControls.edit.track("t").effect("console", 0, "miso.compressor")
        .parameter("threshold", 100),
      /at most/,
    );
    const compressor = liveControls.edit.track("t").effect("console", 0, "miso.compressor");
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, unit: "db" }),
      /unknown field 'unit'/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: true }),
      /must be numeric/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: Number.NaN }),
      /finite/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: 100 }),
      /at most/,
    );
    assert.throws(
      () => compressor.parameter({ key: "lookahead", value: 1 }),
      /not live-updatable/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, channel: "diagonal" }),
      /channel must be left, right, or both/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, smoothingSamples: 1.5 }),
      /smoothingSamples must be a u32/,
    );
    const delay = liveControls.edit.track("t").effect("inserts", 0, "miso.delay");
    assert.throws(
      () => delay.parameter({ key: "cross feedback", value: 0.5, channel: "left" }),
      /shared and must address both lanes/,
    );
    assert.equal(calls, 0);
  });

  test("object and positional effect edits share records and change rendered PCM", async () => {
    async function renderAfter(edit) {
      const engine = await createOfflineEngine(compressorDocument(), {
        asset,
        liveControls: { commandQueueRecords: 64 },
      });
      try {
        const shape = engine.shape();
        feed(engine, 0);
        engine.render();
        const liveControls = engine.liveControls();
        const parameter = liveControls.edit.track("t").effect("console", 0, "miso.compressor");
        const report = edit === undefined ? undefined : await liveControls.submit(edit(parameter));
        const output = [];
        for (let block = 1; block < 4; block += 1) {
          feed(engine, block);
          output.push(engine.render());
        }
        return { report, output, quantumFrames: shape.quantumFrames };
      } finally {
        engine.dispose();
      }
    }

    const baseline = await renderAfter();
    const object = await renderAfter((parameter) => parameter.parameter({
      key: "threshold",
      value: -80,
      channel: "both",
      smoothingSamples: 0,
    }));
    const positional = await renderAfter((parameter) =>
      parameter.parameter("threshold", -80, { channel: "both", smoothingSamples: 0 }));

    assert.equal(object.report?.ok, true);
    assert.equal(object.report?.admitted, 1);
    assert.equal(object.report?.appliedAtSample, BigInt(object.quantumFrames));
    for (const [index, block] of object.output.entries()) {
      assert.deepEqual([...block.left], [...positional.output[index].left], `left block ${index}`);
      assert.deepEqual([...block.right], [...positional.output[index].right], `right block ${index}`);
    }
    assert.notDeepEqual(
      [...object.output[0].left],
      [...baseline.output[0].left],
      "the live object edit must affect the next rendered block",
    );
    assert.notDeepEqual(
      [...object.output.at(-1).left],
      [...baseline.output.at(-1).left],
      "the live object edit must remain audible after the compressor settles",
    );
  });

  test("browser transport maps the same semantic record and acknowledgement", async () => {
    let request;
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1",
          requestId: 1,
          result: 0,
          tracks: ["t"],
          sources: [{ id: "s", channels: 2, frames: 4_800n }],
          metersAttached: false,
          submixes: [],
        };
      },
      async command(value) {
        request = value;
        return {
          tag: "miso.ack.v1",
          requestId: 2,
          result: 0,
          reason: 0,
          rejectedIndex: 0,
          admitted: value.commands.length,
          appliedAtSample: 256n,
          records: new Uint8Array(value.commands.length * 48),
        };
      },
    };
    const liveControls = await createBrowserLiveControls(host);
    const report = await liveControls.submit(
      liveControls.edit.track("t").faderDb(-6, { channel: "left", smoothingSamples: 32 }),
    );
    assert.equal(report.ok, true);
    assert.equal(report.reasonName, "none");
    assert.equal(report.appliedAtSample, 256n);
    assert.deepEqual(request, {
      commands: [{
        kind: 3,
        rack: 255,
        channel: 0,
        trackIndex: 0,
        effectIndex: 0,
        parameterId: 0,
        smoothingSamples: 32,
        values: [-6, 0, 0, 0],
      }],
    });
  });

  test("browser transport carries the object edit record and actual report", async () => {
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    let request;
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1",
          requestId: 1,
          result: 0,
          ...engine.sessionMap(),
        };
      },
      async command(value) {
        request = value;
        const records = encodeBrowserCommands(value.commands);
        const report = engine.submitCommands(records, value.commands.length);
        return {
          tag: "miso.ack.v1",
          requestId: 2,
          result: report.result,
          reason: report.reason,
          rejectedIndex: report.rejectedIndex,
          admitted: report.admitted,
          appliedAtSample: report.appliedAtSample,
          records,
        };
      },
    };
    try {
      const liveControls = await createBrowserLiveControls(host);
      const parameter = liveControls.edit.track("t").effect("console", 0, "miso.compressor");
      const objectEdit = parameter.parameter({
        key: "threshold",
        value: -18,
        channel: "both",
        smoothingSamples: 64,
      });
      const positionalEdit = parameter.parameter("threshold", -18, {
        channel: "both",
        smoothingSamples: 64,
      });
      assert.deepEqual(objectEdit, positionalEdit, "the overloads normalize to one LaneEdit");
      const report = await liveControls.submit(objectEdit);
      assert.equal(report.ok, true);
      assert.equal(report.reasonName, "none");
      assert.equal(report.appliedAtSample, 0n);
      assert.deepEqual(request, {
        commands: [{
          kind: 5,
          rack: 3, // console slot 0, the record's appended console code (S1c)
          channel: 2,
          trackIndex: 0,
          effectIndex: 0,
          parameterId: 1,
          smoothingSamples: 64,
          values: [-18, 0, 0, 0],
        }],
      });
    } finally {
      engine.dispose();
    }
  });

  test("a torn acknowledgement is rejected after, never before, transport answers", async () => {
    let answered = false;
    const liveControls = new EngineLiveControls(
      { tracks: ["t"], sources: [], metersAttached: false, submixes: [] },
      async () => {
        answered = true;
        return {
          ok: true,
          result: 0,
          code: "ok",
          reason: 0,
          reasonName: "none",
          rejectedIndex: 0,
          admitted: 0,
          appliedAtSample: 0n,
        };
      },
    );
    await assert.rejects(
      () => liveControls.submit(liveControls.edit.track("t").faderDb(-6)),
      /violated whole-batch admission/,
    );
    assert.equal(answered, true, "the SDK inspected an acknowledgement only after transport settled");
  });
});

const RACK = Object.fromEntries(ABI_LAYOUT.constants.racks.map((row) => [row.name, row.value]));

/**
 * Issue #1214 gates 1-2: tracks `kick`, `snare` and `vox`, each on its own source; `kick` and
 * `snare` feed `drums`, `vox` feeds `verb`, and both buses feed the output. Each bus has no console
 * slots, no inserts and an identity input section.
 */
function submixDocument({ drumsFaderDb = 0 } = {}) {
  const document = JSON.parse(sessionDocument());
  const [source] = document.sources;
  const [track] = document.tracks;
  const [route] = document.routes;
  const ids = ["kick", "snare", "vox"];
  document.sources = ids.map((id) => ({ ...structuredClone(source), id: `s-${id}` }));
  document.tracks = ids.map((id) => ({ ...structuredClone(track), id, source_id: `s-${id}` }));
  const bus = (id, faderDb) => ({
    id,
    builtins: structuredClone(track.builtins),
    console: [],
    inserts: { effects: [] },
    fader: { ...structuredClone(track.fader), left_db: faderDb, right_db: faderDb },
    pan: structuredClone(track.pan),
  });
  document.submixes = [bus("drums", drumsFaderDb), bus("verb", 0)];
  const feedBus = (trackId, submixId) => ({
    ...structuredClone(route),
    id: `${trackId}-${submixId}`,
    source: { kind: "track", track_id: trackId, tap: "post_pan" },
    destination: { kind: "submix_input", submix_id: submixId },
  });
  const busOut = (submixId) => ({
    ...structuredClone(route),
    id: `${submixId}-out`,
    source: { kind: "submix", submix_id: submixId, tap: "post_pan" },
  });
  document.routes = [
    feedBus("kick", "drums"), feedBus("snare", "drums"), feedBus("vox", "verb"),
    busOut("drums"), busOut("verb"),
  ];
  return `${JSON.stringify(document, null, 2)}\n`;
}

/** A distinct deterministic signal per source and lane, so a gain on one strip moves only its sum. */
function feedDistinct(engine, block) {
  const shape = engine.shape();
  for (const [sourceIndex, source] of shape.sources.entries()) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 1_009 * (sourceIndex + 1) + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

/**
 * Issue #1214 gate 3: console slots `eq` then `comp`, so `comp` is slot 1; `drums` runs inserts
 * `tone` then `glue`, so `glue` is its insert 1, while `verb` runs `glue` alone at insert 0 and
 * track `kick` carries no insert at all.
 */
function builtSubmixSession() {
  const compressor = (slotId) => effect("miso.compressor", { threshold: -12 }, { slotId });
  let built = session({ id: "submix.live", sampleRateHz: 48_000, revision: 1 })
    .source("s", { channels: 2, bitDepth: 24, frames: 4_800, content: `blake3:${"0".repeat(64)}` })
    .console({
      preInsert: [
        { slot: "eq", effectId: "miso.parametric-eq" },
        { slot: "comp", effectId: "miso.compressor" },
      ],
    });
  const entries = [{ slot: "eq" }, { slot: "comp" }];
  built = built
    .track("kick", { source: "s", console: entries })
    .submix("drums", {
      console: entries,
      inserts: [effect("miso.parametric-eq", {}, { slotId: "tone" }), compressor("glue")],
    })
    .submix("verb", { console: entries, inserts: [compressor("glue")] })
    .output("out")
    .route({
      id: "kick-drums",
      source: { kind: "track", trackId: "kick", tap: "post_pan" },
      destination: { kind: "submix_input", submixId: "drums" },
    })
    .route({
      id: "kick-verb",
      source: { kind: "track", trackId: "kick", tap: "post_fader" },
      destination: { kind: "submix_input", submixId: "verb" },
    });
  for (const bus of ["drums", "verb"]) {
    built = built.route({
      id: `${bus}-out`,
      source: { kind: "submix", submixId: bus, tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
  }
  return built;
}

describe("issue 1214 -- live controls drive submix strips", () => {
  test("a submix encodes its strip index T + j and IDs resolve only in their own list", async () => {
    // Gate 1. Red if the SDK indexes submixes from 0, resolves them against tracks, or lets a
    // submix ID through track(); the batch below also proves the shipped module admits every
    // builtin edit the SDK builds at a bus index.
    const engine = await createOfflineEngine(submixDocument(), {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    try {
      const controls = engine.liveControls();
      assert.deepEqual(engine.shape().submixes, ["drums", "verb"]);
      assert.equal(controls.edit.submix("verb").faderDb(-6).trackIndex, 4);
      assert.equal(controls.edit.submix("drums").pan(-1, 1).trackIndex, 3);
      assert.equal(controls.edit.track("vox").faderDb(-6).trackIndex, 2);
      assert.throws(() => controls.edit.submix("kick"), (error) =>
        error instanceof MisoUsageError && /no submix 'kick'; expected one of drums, verb/.test(error.message));
      assert.throws(() => controls.edit.submix("nope"), MisoUsageError);
      assert.throws(() => controls.edit.track("drums"), MisoUsageError);
      assert.equal("solo" in controls.edit.submix("drums"), false, "a bus has no solo edit");

      const verb = controls.edit.submix("verb");
      const edits = [
        verb.pan(-0.5, 0.5),
        verb.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }),
        verb.faderDb(-3, { channel: "left" }),
        verb.mute(false, { channel: "right" }),
        verb.trimDb(1.5),
        verb.polarityInvert(false),
        verb.hpfHz(40, { channel: "left" }),
        verb.lpfHz(16_000, { channel: "right" }),
        verb.inputFilters({ hpfHz: 30, lpfHz: 18_000 }),
      ];
      assert.deepEqual(edits.map((edit) => edit.trackIndex), edits.map(() => 4));
      const report = await controls.submit(...edits);
      assert.equal(report.ok, true, report.reasonName);
      assert.equal(report.admitted, edits.length);
    } finally {
      engine.dispose();
    }
  });

  test("a live bus fader equals the bus booted at that fader from the edit's block on", async () => {
    // Gate 2, through the shipped module. Red if the SDK's record reaches the wrong strip (a track
    // inside the bus, the other bus) or the wrong band of the right strip.
    const options = { asset, liveControls: { commandQueueRecords: 64 } };
    const live = await createOfflineEngine(submixDocument(), options);
    const booted = await createOfflineEngine(submixDocument({ drumsFaderDb: -6 }), options);
    try {
      const boundary = 4;
      const quantum = live.shape().quantumFrames;
      let differedBefore = false;
      for (let block = 0; block < boundary; block += 1) {
        feedDistinct(live, block);
        feedDistinct(booted, block);
        const [a, b] = [live.render(), booted.render()];
        differedBefore ||= a.left.some((sample, index) => sample !== b.left[index]);
      }
      assert.ok(differedBefore, "the bus fader must be audible, or the comparison proves nothing");

      const controls = live.liveControls();
      const report = await controls.submit(
        controls.edit.submix("drums").faderDb(-6, { smoothingSamples: 0 }),
      );
      assert.equal(report.ok, true, report.reasonName);
      assert.equal(report.appliedAtSample, BigInt(boundary * quantum));
      for (let block = boundary; block < boundary + 4; block += 1) {
        feedDistinct(live, block);
        feedDistinct(booted, block);
        const [a, b] = [live.render(), booted.render()];
        assert.ok(a.left.some((sample) => sample !== 0), "the mix carries signal");
        assert.deepEqual([...a.left], [...b.left], `left block ${block}`);
        assert.deepEqual([...a.right], [...b.right], `right block ${block}`);
      }
    } finally {
      live.dispose();
      booted.dispose();
    }
  });

  test("console slots and inserts resolve by ID on a bus of an SDK-built session", async () => {
    // Gate 3. Red if layoutOf walks tracks only (no 'glue' on drums), refuses a session with
    // submixes as mismatched, or resolves a bus insert against another strip's chain.
    const built = builtSubmixSession();
    const engine = await createOfflineEngine(built, {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    try {
      const controls = engine.liveControls();
      const address = (edit) => [edit.trackIndex, edit.rack, edit.effectIndex];
      const glue = controls.edit.submix("drums").insert("glue", "miso.compressor")
        .parameter("threshold", -18);
      const comp = controls.edit.submix("drums").console("comp", "miso.compressor").bypass(true);
      assert.deepEqual(address(glue), [1, RACK.inserts, 1]);
      assert.equal(glue.kind, "effectParam");
      assert.deepEqual(address(comp), [1, RACK.console, 1]);
      assert.deepEqual(
        address(controls.edit.submix("verb").insert("glue", "miso.compressor").bypass(true)),
        [2, RACK.inserts, 0],
      );
      assert.throws(() => controls.edit.submix("drums").insert("ghost", "miso.compressor"),
        /submix 'drums' has no insert 'ghost'; its inserts are tone, glue/);
      assert.throws(() => controls.edit.track("kick").insert("glue", "miso.compressor"),
        /track 'kick' has no insert 'glue'/);
      const report = await controls.submit(glue, comp);
      assert.equal(report.ok, true, report.reasonName);
      assert.equal(report.admitted, 2);

      // The layout is held to the compiled strips: a map that lacks the session's submixes is
      // refused rather than resolving bus IDs the engine never compiled.
      assert.throws(
        () => new LiveControlEdits({ tracks: ["kick"], sources: [], metersAttached: false, submixes: [] }, built),
        /declares tracks kick and submixes drums, verb, but the engine compiled tracks kick and submixes none/,
      );
    } finally {
      engine.dispose();
    }
  });
});
