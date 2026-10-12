/** Issue #322: shared semantic live controls and one whole-batch contract over both transports. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { createBrowserLiveControls } from "../src/browser/live-controls.ts";
import { EngineLiveControls, LiveControlEdits } from "../src/core/live-controls.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { encodeLaneEdits, LiveControlWriter } from "../src/core/writer.ts";
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
  test("every command kind is built by name, and the strip and effect kinds are admitted by live Wasm", async () => {
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

      // The send kinds are built by `RouteEdits` and admitted by live Wasm in the issue 1223
      // evals below, and the VCA kinds by `VcaEdits` in the issue 1246 evals; this session has
      // neither a send nor a VCA.
      const kindNames = [
        "pan", "matrix", "faderDb", "mute", "effectParam", "effectBypass",
        "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert", "inputFilters",
        "routeGainDb", "routeMute", "routeMatrix", "vcaFaderDb", "vcaMute",
      ];
      assert.deepEqual(
        [...ABI_LAYOUT.constants.wireCommandKinds.map((row) => row.name)].sort(),
        [...kindNames].sort(),
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
      { tracks: ["t"], sources: [], metersAttached: false, submixes: [], routes: [], vcas: [] },
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
          routes: [],
          vcas: [],
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
      { tracks: ["t"], sources: [], metersAttached: false, submixes: [], routes: [], vcas: [] },
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
        () => new LiveControlEdits({ tracks: ["kick"], sources: [], metersAttached: false, submixes: [], routes: [], vcas: [] }, built),
        /declares tracks kick and submixes drums, verb, but the engine compiled tracks kick and submixes none/,
      );
    } finally {
      engine.dispose();
    }
  });
});

/**
 * Issue #1223: tracks `kick`, `snare` and `vox`, each on its own source; unity buses `drums` and
 * `verb` (no console slot, no insert, an identity input section); four sends declared out of
 * route-ID order -- `snare-drums`, `vox-verb`, `kick-verb`, `kick-drums` -- and both buses to the
 * output. The live routes are therefore `kick-drums`, `kick-verb`, `snare-drums`, `vox-verb`,
 * while the declaration puts `kick-verb` at 2 and every route in ID order puts it at 2 too (behind
 * `drums-out`).
 */
function sendSession({ kickVerbDb = 0 } = {}) {
  let built = session({ id: "send.live", sampleRateHz: 48_000, revision: 1 });
  const ids = ["kick", "snare", "vox"];
  for (const id of ids) {
    built = built.source(`s-${id}`, {
      channels: 2, bitDepth: 24, frames: 4_800, content: `blake3:${"0".repeat(64)}`,
    });
  }
  for (const id of ids) built = built.track(id, { source: `s-${id}` });
  built = built.submix("drums").submix("verb").output("out");
  const sends = [
    ["snare-drums", "snare", "drums", 0],
    ["vox-verb", "vox", "verb", -3],
    ["kick-verb", "kick", "verb", kickVerbDb],
    ["kick-drums", "kick", "drums", 0],
  ];
  for (const [id, trackId, submixId, gainDb] of sends) {
    built = built.route({
      id,
      source: { kind: "track", trackId, tap: "post_pan" },
      destination: { kind: "submix_input", submixId },
      gainDb,
    });
  }
  for (const bus of ["drums", "verb"]) {
    built = built.route({
      id: `${bus}-out`,
      source: { kind: "submix", submixId: bus, tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
  }
  return built;
}

describe("issue 1223 -- live controls drive sends", () => {
  test("a send edit encodes its kind at the engine's live-route index, over both transports", async () => {
    // Gate 1. Red if the SDK indexes a send by its own order of the session (declaration, or every
    // route by ID), if the browser map drops the engine's route list, or if a send record's kind,
    // rack, channel or matrix words are written otherwise.
    const engine = await createOfflineEngine(sendSession(), {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    let request;
    const host = {
      async sessionMap() {
        return { tag: "miso.sessionmap.v1", requestId: 1, result: 0, ...engine.sessionMap() };
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
      const routes = engine.sessionMap().routes;
      assert.deepEqual(routes, ["kick-drums", "kick-verb", "snare-drums", "vox-verb"]);
      for (const controls of [engine.liveControls(), await createBrowserLiveControls(host)]) {
        const send = controls.edit.route("kick-verb");
        const edits = [
          send.gainDb(-12, { smoothingSamples: 64 }),
          send.mute(true, { smoothingSamples: 0 }),
          send.matrix({ ll: 0.5, lr: 0.25, rl: -0.75, rr: 0.125 }, { smoothingSamples: 32 }),
        ];
        assert.deepEqual(edits.map((edit) => edit.kind), ["routeGainDb", "routeMute", "routeMatrix"]);
        assert.deepEqual(edits.map((edit) => edit.trackIndex), edits.map(() => routes.indexOf("kick-verb")));
        assert.equal(controls.edit.route("vox-verb").gainDb(0).trackIndex, 3);
        const report = await controls.submit(...edits);
        assert.equal(report.ok, true, report.reasonName);
        assert.equal(report.admitted, 3);
      }
      const common = { rack: 255, channel: 255, trackIndex: 1, effectIndex: 0, parameterId: 0 };
      assert.deepEqual(request, {
        commands: [
          { kind: 13, ...common, smoothingSamples: 64, values: [-12, 0, 0, 0] },
          { kind: 14, ...common, smoothingSamples: 0, values: [1, 0, 0, 0] },
          { kind: 15, ...common, smoothingSamples: 32, values: [0.5, 0.25, -0.75, 0.125] },
        ],
      });
    } finally {
      engine.dispose();
    }
  });

  test("an unknown or output route ID refuses before any record is built", async () => {
    // Gate 2. Red if the SDK hands an ID it cannot place to some index (a track's, an output
    // route's, or index 0) instead of refusing, or loses the output-route reason with a session.
    const built = sendSession();
    const engine = await createOfflineEngine(built, {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    try {
      const controls = engine.liveControls();
      const unknownRoute = (pattern) => (error) =>
        error instanceof MisoUsageError && error.diagnosticCode === "unknownRoute"
          && pattern.test(error.message);
      const listed = /no live route '[^']+'; expected one of kick-drums, kick-verb, snare-drums, vox-verb/;
      assert.throws(() => controls.edit.route("nope"), unknownRoute(listed));
      assert.throws(() => controls.edit.route("kick"), unknownRoute(listed));
      assert.throws(() => controls.edit.route("drums-out"),
        unknownRoute(/route 'drums-out' goes to the output, and output routes are not live/));
      // Without the session the SDK cannot tell an output route from a typo, so it lists the
      // live routes instead.
      const bare = new LiveControlEdits(engine.sessionMap());
      assert.throws(() => bare.route("drums-out"), unknownRoute(listed));
      assert.equal(bare.route("snare-drums").mute(false).trackIndex, 2);
    } finally {
      engine.dispose();
    }
  });

  test("a live send gain equals the session booted at that gain from the edit's block on", async () => {
    // Gate 3, through the shipped module. Red if the route export, the session map or the
    // encoding disagrees with the engine about which send an index names, or about the gain word.
    const options = { asset, liveControls: { commandQueueRecords: 64 } };
    const live = await createOfflineEngine(sendSession(), options);
    const booted = await createOfflineEngine(sendSession({ kickVerbDb: -12 }), options);
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
      assert.ok(differedBefore, "the send gain must be audible, or the comparison proves nothing");

      const controls = live.liveControls();
      const report = await controls.submit(
        controls.edit.route("kick-verb").gainDb(-12, { smoothingSamples: 0 }),
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
});

/**
 * Issue #1246: tracks `kick`, `snare` and `vox`, each on its own source; `kick` and `snare` feed
 * the unity bus `bus` (no console slot, no insert, an identity input section), and `vox` and
 * `bus` feed the output. Three VCAs: `drums` over `kick` and `snare`, `fx` over `vox`, and `band`
 * over both VCAs. The builder writes them in canonical VCA-ID order, `band`, `drums`, `fx`.
 */
function vcaSession({ drums = {}, fx = {} } = {}) {
  let built = session({ id: "vca.live", sampleRateHz: 48_000, revision: 1 });
  const ids = ["kick", "snare", "vox"];
  for (const id of ids) {
    built = built.source(`s-${id}`, {
      channels: 2, bitDepth: 24, frames: 4_800, content: `blake3:${"0".repeat(64)}`,
    });
  }
  for (const id of ids) built = built.track(id, { source: `s-${id}` });
  built = built
    .submix("bus")
    .output("out")
    .vca("drums", { members: ["kick", "snare"], fader: drums })
    .vca("fx", { members: ["vox"], fader: fx })
    .vca("band", { members: ["drums", "fx"] });
  for (const id of ["kick", "snare"]) {
    built = built.route({
      id: `${id}-bus`,
      source: { kind: "track", trackId: id, tap: "post_pan" },
      destination: { kind: "submix_input", submixId: "bus" },
    });
  }
  built = built
    .route({
      id: "vox-out",
      source: { kind: "track", trackId: "vox", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    })
    .route({
      id: "bus-out",
      source: { kind: "submix", submixId: "bus", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
  return built;
}

/**
 * `vcaSession()`'s canonical document with its VCAs declared out of VCA-ID order -- `drums`, `fx`,
 * `band`, the order a member-first author writes them -- which puts `drums` at 0 where the
 * engine's canonical order puts it at 1, and `band` at 2 where the engine puts it at 0.
 */
function vcaDocument() {
  const model = JSON.parse(JSON.stringify(vcaSession().toJSON()));
  const byId = new Map(model.vcas.map((vca) => [vca.id, vca]));
  model.vcas = ["drums", "fx", "band"].map((id) => byId.get(id));
  return `${JSON.stringify(model, null, 2)}\n`;
}

describe("issue 1246 -- live controls drive VCA groups", () => {
  test("a VCA edit encodes its kind at the engine's VCA index, over both transports", async () => {
    // Gate 1. Red if the SDK indexes a VCA by its own order of the session (the declaration puts
    // `drums` at 0), if the engine's export or either session map loses or reorders the VCA list,
    // or if a VCA record's kind, rack, channel or value words are written otherwise.
    const document = vcaDocument();
    assert.deepEqual(JSON.parse(document).vcas.map((vca) => vca.id), ["drums", "fx", "band"],
      "the document declares the VCAs in an order the index must not follow");
    const engine = await createOfflineEngine(document, {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    let request;
    const host = {
      async sessionMap() {
        return { tag: "miso.sessionmap.v1", requestId: 1, result: 0, ...engine.sessionMap() };
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
      assert.deepEqual(engine.sessionMap().vcas, ["band", "drums", "fx"]);
      for (const controls of [engine.liveControls(), await createBrowserLiveControls(host)]) {
        const drums = controls.edit.vca("drums");
        const edits = [
          drums.faderDb(-6, { smoothingSamples: 0 }),
          drums.mute(true, { channel: "left", smoothingSamples: 32 }),
          controls.edit.vca("band").faderDb(1.5, { channel: "right", smoothingSamples: 64 }),
        ];
        assert.deepEqual(edits.map((edit) => edit.kind), ["vcaFaderDb", "vcaMute", "vcaFaderDb"]);
        assert.deepEqual(edits.map((edit) => edit.trackIndex), [1, 1, 0]);
        const report = await controls.submit(...edits);
        assert.equal(report.ok, true, report.reasonName);
        assert.equal(report.admitted, 3);
      }
      const common = { rack: 255, effectIndex: 0, parameterId: 0 };
      assert.deepEqual(request, {
        commands: [
          { kind: 16, ...common, channel: 2, trackIndex: 1, smoothingSamples: 0, values: [-6, 0, 0, 0] },
          { kind: 17, ...common, channel: 0, trackIndex: 1, smoothingSamples: 32, values: [1, 0, 0, 0] },
          { kind: 16, ...common, channel: 1, trackIndex: 0, smoothingSamples: 64, values: [1.5, 0, 0, 0] },
        ],
      });
    } finally {
      engine.dispose();
    }

    // Without VCAs, and without live controls, the engine enumerates none.
    for (const [document, liveControls] of [
      [sendSession(), { commandQueueRecords: 64 }],
      [vcaSession(), undefined],
    ]) {
      const other = await createOfflineEngine(document, { asset, liveControls });
      try {
        assert.deepEqual(other.sessionMap().vcas, []);
      } finally {
        other.dispose();
      }
    }
  });

  test("an unknown VCA ID refuses with unknownVca before any record is built", async () => {
    // Gate 1. Red if the SDK hands an ID it cannot place -- a typo, a member strip's ID -- to some
    // index instead of refusing, or refuses it under a reason other than the engine's.
    const engine = await createOfflineEngine(vcaSession(), {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    try {
      const unknownVca = (pattern) => (error) =>
        error instanceof MisoUsageError && error.diagnosticCode === "unknownVca"
          && pattern.test(error.message);
      const listed = /no VCA '[^']+'; expected one of band, drums, fx/;
      for (const edits of [engine.liveControls().edit, new LiveControlEdits(engine.sessionMap())]) {
        assert.throws(() => edits.vca("nope"), unknownVca(listed));
        assert.throws(() => edits.vca("kick"), unknownVca(listed));
        assert.throws(() => edits.vca("bus"), unknownVca(listed));
        assert.equal(edits.vca("fx").mute(false).trackIndex, 2);
      }
    } finally {
      engine.dispose();
    }
    const none = new LiveControlEdits({ tracks: [], sources: [], metersAttached: false, submixes: [], routes: [], vcas: [] });
    assert.throws(() => none.vca("drums"), (error) =>
      error instanceof MisoUsageError && error.diagnosticCode === "unknownVca"
        && /expected one of none/.test(error.message));
  });

  test("a live VCA fader and mute equal the session booted at those values from the edit's block on", async () => {
    // Gate 2, through the shipped module. Red if the VCA export, the session map or the encoding
    // disagrees with the engine about which VCA an index names, which kind a record is, which
    // lane it selects, or the offset word.
    const options = { asset, liveControls: { commandQueueRecords: 64 } };
    for (const [what, edit, edited] of [
      ["fader", (controls) => controls.edit.vca("drums").faderDb(-6, { smoothingSamples: 0 }),
        { drums: { leftDb: -6, rightDb: -6 } }],
      ["left-lane mute", (controls) => controls.edit.vca("fx").mute(true, { channel: "left", smoothingSamples: 0 }),
        { fx: { leftMute: true } }],
    ]) {
      const live = await createOfflineEngine(vcaSession(), options);
      const booted = await createOfflineEngine(vcaSession(edited), options);
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
        assert.ok(differedBefore, `${what}: the VCA must be audible, or the comparison proves nothing`);

        const controls = live.liveControls();
        const report = await controls.submit(edit(controls));
        assert.equal(report.ok, true, `${what}: ${report.reasonName}`);
        assert.equal(report.appliedAtSample, BigInt(boundary * quantum));
        for (let block = boundary; block < boundary + 4; block += 1) {
          feedDistinct(live, block);
          feedDistinct(booted, block);
          const [a, b] = [live.render(), booted.render()];
          assert.ok(a.left.some((sample) => sample !== 0), `${what}: the mix carries signal`);
          assert.ok(a.right.some((sample) => sample !== 0), `${what}: the right lane carries signal`);
          assert.deepEqual([...a.left], [...b.left], `${what}: left block ${block}`);
          assert.deepEqual([...a.right], [...b.right], `${what}: right block ${block}`);
        }
      } finally {
        live.dispose();
        booted.dispose();
      }
    }
  });
});

describe("issue 1364 -- an absent ramp length is the session default", () => {
  const MARKER = 0xffff_ffff;
  const map = {
    tracks: ["t"],
    sources: [],
    metersAttached: false,
    submixes: ["s"],
    routes: ["r"],
    vcas: ["v"],
  };

  /** Each ramped builder method, given the options to pass. */
  function rampedEdits(options) {
    const edits = new LiveControlEdits(map);
    const track = edits.track("t");
    const submix = edits.submix("s");
    const send = edits.route("r");
    const vca = edits.vca("v");
    return {
      pan: track.pan(-0.5, 0.5, options),
      matrix: track.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }, options),
      faderDb: track.faderDb(-3, options),
      mute: track.mute(true, options),
      trimDb: track.trimDb(-3, options),
      polarityInvert: track.polarityInvert(true, options),
      solo: track.solo(true, options),
      submixFaderDb: submix.faderDb(-3, options),
      routeGainDb: send.gainDb(-3, options),
      routeMute: send.mute(true, options),
      routeMatrix: send.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }, options),
      vcaFaderDb: vca.faderDb(-3, options),
      vcaMute: vca.mute(true, options),
    };
  }

  /** The `u32` at the record's `smoothingSamples` offset, as the SDK's record encoder writes it. */
  function wireWord(edit) {
    const layout = ABI_LAYOUT.commandRecord.fields.find((row) => row.name === "smoothingSamples");
    assert.equal(layout.offset, 16, "the ramp word sits at offset 16 of the 48-byte record");
    return new DataView(encodeLaneEdits([edit]).buffer).getUint32(layout.offset, true);
  }

  test("every ramped builder method writes the session-default marker when no length is given, and the given length otherwise", () => {
    // Red if any method's default is still 0 (a step), if the marker is not 0xFFFFFFFF, if a method
    // resolves the word itself instead of leaving it to the encoder, or if a
    // given length -- 0 included -- is replaced.
    const absent = rampedEdits({});
    assert.equal(Object.keys(absent).length, 13, "the twelve ramped kinds, and the submix strip fader");
    for (const [name, edit] of Object.entries(absent)) {
      assert.equal(edit.smoothingSamples, undefined, `${name} record leaves the length absent`);
      assert.equal(wireWord(edit), MARKER, `${name} wire`);
    }
    for (const given of [0, 37, MARKER - 1]) {
      for (const [name, edit] of Object.entries(rampedEdits({ smoothingSamples: given }))) {
        assert.equal(edit.smoothingSamples, given, `${name} record at ${given}`);
        assert.equal(wireWord(edit), given, `${name} wire at ${given}`);
      }
    }
  });

  test("the kinds that carry no ramp still write 0", () => {
    // Red if `trackEdit`'s default moved to the marker: the engine refuses a non-zero word on an
    // input filter or a bypass as malformed.
    const track = new LiveControlEdits(map).track("t");
    const compressor = track.effect("console", 0, "miso.compressor");
    for (const edit of [
      track.hpfHz(80),
      track.lpfHz(12_000),
      track.inputFilters({ hpfHz: 80, lpfHz: 12_000 }),
      compressor.bypass(true),
    ]) {
      assert.equal(wireWord(edit), 0, edit.kind);
    }
  });

  test("the marker is not a length: smoothingSamples 4294967295 throws on every ramped method", () => {
    // Red if the marker passes as an explicit length (the engine would then read it as "default"
    // and the author's `u32::MAX` would silently mean something else).
    const names = Object.keys(rampedEdits({}));
    const attempts = {
      pan: (o) => new LiveControlEdits(map).track("t").pan(0, 0, o),
      matrix: (o) => new LiveControlEdits(map).track("t").matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }, o),
      faderDb: (o) => new LiveControlEdits(map).track("t").faderDb(0, o),
      mute: (o) => new LiveControlEdits(map).track("t").mute(true, o),
      trimDb: (o) => new LiveControlEdits(map).track("t").trimDb(0, o),
      polarityInvert: (o) => new LiveControlEdits(map).track("t").polarityInvert(true, o),
      solo: (o) => new LiveControlEdits(map).track("t").solo(true, o),
      submixFaderDb: (o) => new LiveControlEdits(map).submix("s").faderDb(0, o),
      routeGainDb: (o) => new LiveControlEdits(map).route("r").gainDb(0, o),
      routeMute: (o) => new LiveControlEdits(map).route("r").mute(true, o),
      routeMatrix: (o) => new LiveControlEdits(map).route("r").matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }, o),
      vcaFaderDb: (o) => new LiveControlEdits(map).vca("v").faderDb(0, o),
      vcaMute: (o) => new LiveControlEdits(map).vca("v").mute(true, o),
    };
    assert.deepEqual(Object.keys(attempts), names);
    for (const [name, attempt] of Object.entries(attempts)) {
      assert.throws(
        () => attempt({ smoothingSamples: MARKER }),
        (error) => error instanceof MisoUsageError && /session-default marker/.test(error.message),
        name,
      );
      // The bounds that were already there still hold.
      for (const bad of [-1, 1.5, 4_294_967_296, Number.NaN]) {
        assert.throws(() => attempt({ smoothingSamples: bad }), /smoothingSamples must be a u32/, `${name} ${bad}`);
      }
    }
  });

  /** A literal `LaneEdit` (a plain object, as an app writes one), with no `smoothingSamples`. */
  function literal(kind, extra = {}) {
    return {
      kind,
      trackIndex: 0,
      rack: 255,
      channel: 2,
      values: [0, 0, 0, 0],
      ...extra,
    };
  }
  const RAMPED_KINDS = [
    "pan", "matrix", "faderDb", "mute", "solo", "trimDb", "polarityInvert",
    "routeGainDb", "routeMute", "routeMatrix", "vcaFaderDb", "vcaMute",
  ];
  const UNRAMPED_KINDS = ["effectBypass", "inputFilters", "observeSubscribe", "observeUnsubscribe"];

  test("a literal LaneEdit without a length encodes the marker through the writer's encoder, by kind", () => {
    // Red if `encodeLaneEdits` writes `smoothingSamples ?? 0` again (a step on a literal edit, the
    // path `LiveControlWriter` and the headless engine take): the builder evals above never reach it.
    const at = ABI_LAYOUT.commandRecord.fields.find((row) => row.name === "smoothingSamples").offset;
    const word = (edit) => new DataView(encodeLaneEdits([edit]).buffer).getUint32(at, true);
    for (const kind of RAMPED_KINDS) {
      assert.equal(word(literal(kind)), MARKER, `${kind} absent`);
      assert.equal(word(literal(kind, { smoothingSamples: 0 })), 0, `${kind} explicit 0 is a step`);
      assert.equal(word(literal(kind, { smoothingSamples: 37 })), 37, `${kind} explicit 37`);
      assert.throws(
        () => encodeLaneEdits([literal(kind, { smoothingSamples: MARKER })]),
        (error) => error instanceof MisoUsageError && /session-default marker/.test(error.message),
        `${kind} explicit marker`,
      );
    }
    for (const kind of UNRAMPED_KINDS) {
      assert.equal(word(literal(kind)), 0, `${kind} absent stays 0`);
    }
    // The writer refuses the marker when the edit is staged, so it never poisons a later flush.
    const writer = new LiveControlWriter({ submit: () => ({ ok: true }) });
    assert.throws(() => writer.stage(literal("faderDb", { smoothingSamples: MARKER })), MisoUsageError);
    assert.equal(writer.pending, 0);
  });

  test("a literal LaneEdit without a length reaches the worklet host with the marker, by kind", async () => {
    // Red if the browser transport writes `smoothingSamples ?? 0` again: a literal fader edit sent
    // to the worklet would then step under every session table.
    let request;
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1", requestId: 1, result: 0, tracks: ["t"], sources: [],
          metersAttached: false, submixes: [], routes: [], vcas: [],
        };
      },
      async command(value) {
        request = value;
        return {
          tag: "miso.ack.v1", requestId: 2, result: 0, reason: 0, rejectedIndex: 0,
          admitted: value.commands.length, appliedAtSample: 0n,
          records: new Uint8Array(value.commands.length * 48),
        };
      },
    };
    const controls = await createBrowserLiveControls(host);
    await controls.submit(controls.edit.track("t").faderDb(-3));
    assert.equal(request.commands[0].smoothingSamples, MARKER, "a builder edit agrees with a literal one");
    for (const kind of RAMPED_KINDS) {
      await controls.submit(literal(kind));
      assert.equal(request.commands[0].smoothingSamples, MARKER, `${kind} absent`);
      await controls.submit(literal(kind, { smoothingSamples: 0 }));
      assert.equal(request.commands[0].smoothingSamples, 0, `${kind} explicit 0`);
      await assert.rejects(
        controls.submit(literal(kind, { smoothingSamples: MARKER })),
        (error) => error instanceof MisoUsageError && /session-default marker/.test(error.message),
        `${kind} explicit marker`,
      );
    }
    for (const kind of UNRAMPED_KINDS) {
      await controls.submit(literal(kind));
      assert.equal(request.commands[0].smoothingSamples, 0, `${kind} absent stays 0`);
    }
  });

  /** The one-track document with `control_smoothing` and a constant fader of `faderDb`. */
  function documentWith(controlSmoothing, faderDb = 0) {
    const document = JSON.parse(sessionDocument({ frames: 128 * 24 }));
    document.control_smoothing = controlSmoothing;
    document.tracks[0].fader.left_db = faderDb;
    document.tracks[0].fader.right_db = faderDb;
    return JSON.stringify(document);
  }

  /**
   * Boot a live engine from `controlSmoothing`, move the fader to -6 dB at block 2 with `options`,
   * and return for each of blocks 2..13 whether the live output differs from an engine booted at
   * -6 dB.
   */
  async function differsFromBooted(controlSmoothing, options) {
    const live = await createOfflineEngine(documentWith(controlSmoothing), {
      asset,
      liveControls: { commandQueueRecords: 64 },
    });
    const booted = await createOfflineEngine(documentWith(controlSmoothing, -6), { asset });
    try {
      const differs = [];
      for (let block = 0; block < 14; block += 1) {
        if (block === 2) {
          const controls = live.liveControls();
          const report = await controls.submit(controls.edit.track("t").faderDb(-6, options));
          assert.equal(report.ok, true, report.reasonName);
          assert.equal(report.appliedAtSample, BigInt(2 * live.shape().quantumFrames));
        }
        feed(live, block);
        feed(booted, block);
        const [a, b] = [live.render(), booted.render()];
        assert.ok(a.left.some((sample) => sample !== 0), "the mix carries signal");
        if (block >= 2) {
          differs.push(a.left.some((sample, index) => sample !== b.left[index])
            || a.right.some((sample, index) => sample !== b.right[index]));
        }
      }
      return differs;
    } finally {
      live.dispose();
      booted.dispose();
    }
  }

  test("a fader move without a length ramps over the session's fader length and lands on the booted value", async () => {
    // Gate 2, through the shipped module. Red if the SDK still writes 0 (every block settled, the
    // first one included), if the engine reads the marker as a length, or if it ignores the
    // session's table: the default table's 20 ms is 960 samples, 7.5 blocks of 128; 2 ms is 96
    // samples, inside the first block; an explicit all-zero table is a step.
    const defaultTable = await differsFromBooted({ kind: "default" }, {});
    assert.deepEqual(defaultTable.slice(0, 7), Array(7).fill(true), "ramping through block 8");
    assert.deepEqual(defaultTable.slice(8), Array(4).fill(false), "settled from block 10");

    const twoMs = await differsFromBooted(
      { kind: "explicit", mute_ms: 10, fader_ms: 2, pan_ms: 20 },
      {},
    );
    assert.deepEqual(twoMs.slice(0, 2), [true, false], "a 96-sample ramp ends inside block 2");
    assert.deepEqual(twoMs.slice(2), Array(10).fill(false));

    const stepTable = await differsFromBooted(
      { kind: "explicit", mute_ms: 0, fader_ms: 0, pan_ms: 0 },
      {},
    );
    assert.deepEqual(stepTable, Array(12).fill(false), "an all-zero table steps from block 2");

    const explicitZero = await differsFromBooted({ kind: "default" }, { smoothingSamples: 0 });
    assert.deepEqual(explicitZero, Array(12).fill(false), "an explicit 0 steps under the default table");
  });
});
