/**
 * #1563B: shared semantic live controls over both transports. Every value edit is a `SET_CONTROL`
 * transaction sent through the engine's one queue; the builders check no value, and the core
 * refuses the same way on the headless engine and through the browser host.
 */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { createBrowserControlQueue, createBrowserLiveControls } from "../src/browser/live-controls.ts";
import { MisoControlRefusal, MisoUsageError } from "../src/core/errors.ts";
import { EngineLiveControls, LiveControlEdits } from "../src/core/live-controls.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { effect, session } from "../src/core/session.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, engineHost, moduleBytes, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function compressorDocument(frames = 4_800) {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  return sessionDocument({
    frames,
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


const OPTIONS = {};

async function boot(document, extra = {}) {
  return createOfflineEngine(document, { asset, ...OPTIONS, ...extra });
}

/** A document with `mutate` applied to its parsed JSON. */
function variant(document, mutate) {
  const parsed = JSON.parse(document);
  mutate(parsed);
  return JSON.stringify(parsed);
}

/** The edit a builder made, as the plain address/value the frame carries. */
function plain(set) {
  return set.edits.map((edit) => ({
    owner: edit.address.owner,
    section: edit.address.section,
    control: edit.address.control,
    lane: edit.address.lane,
    value: edit.value,
    ...(edit.rampSamples === undefined ? {} : { rampSamples: edit.rampSamples }),
  }));
}

const MAP = {
  tracks: ["t"], sources: [], metersAttached: false, submixes: ["s"],
};

describe("#1563B -- a builder is a set of SET_CONTROL edits at the core's address", () => {
  // Gate 3's other half: the rows, sections and lanes are the core's, spelled here as numbers so a
  // builder that moved to another row turns red even if it still read a table.
  test("every strip, send, VCA and effect builder names its owner, section, row and lane", () => {
    const edits = new LiveControlEdits(MAP);
    const track = edits.track("t");
    const strip = { kind: "strip", id: "t" };
    const builtins = { kind: "builtins" };
    const own = { kind: "own" };
    const row = (control, value, lane = "both", extra = {}) =>
      ({ owner: strip, section: builtins, control, lane, value, ...extra });

    assert.deepEqual(plain(track.faderDb(-6, { channel: "left", smoothingSamples: 32 })),
      [row(5, -6, "left", { rampSamples: 32 })]);
    assert.deepEqual(plain(track.mute(true, { channel: "right" })), [row(6, 1, "right")]);
    assert.deepEqual(plain(track.trimDb(1.5)), [row(2, 1.5)]);
    assert.deepEqual(plain(track.polarityInvert(true, { smoothingSamples: 0 })),
      [row(1, 1, "both", { rampSamples: 0 })]);
    assert.deepEqual(plain(track.hpfHz(80, { channel: "left" })), [row(3, 80, "left")]);
    assert.deepEqual(plain(track.lpfHz(12_000)), [row(4, 12_000)]);
    assert.deepEqual(plain(track.inputFilters({ hpfHz: 80, lpfHz: 12_000 }, { channel: "right" })),
      [row(3, 80, "right"), row(4, 12_000, "right")]);
    assert.deepEqual(plain(track.pan(-0.5, 0.25, { smoothingSamples: 16 })),
      [row(12, -0.5, "left", { rampSamples: 16 }), row(12, 0.25, "right", { rampSamples: 16 })]);
    assert.deepEqual(plain(track.matrix({ ll: 1, lr: 0.5, rl: -0.5, rr: 0.25 })),
      [row(7, 1), row(8, 0.5), row(9, -0.5), row(10, 0.25)]);
    assert.deepEqual(plain(track.solo(true)),
      [{ owner: strip, section: own, control: 2, lane: "both", value: 1 }]);

    const submix = edits.submix("s");
    assert.deepEqual(plain(submix.faderDb(-3)),
      [{ owner: { kind: "strip", id: "s" }, section: builtins, control: 5, lane: "both", value: -3 }]);

    const send = edits.route("r");
    const route = (control, value, extra = {}) =>
      ({ owner: { kind: "route", id: "r" }, section: own, control, lane: "both", value, ...extra });
    assert.deepEqual(plain(send.gainDb(-12, { smoothingSamples: 64 })),
      [route(1, -12, { rampSamples: 64 })]);
    assert.deepEqual(plain(send.mute(true)), [route(2, 1)]);
    assert.deepEqual(plain(send.matrix({ ll: 0.5, lr: 0.25, rl: -0.75, rr: 0.125 })),
      [route(3, 0.5), route(4, 0.25), route(5, -0.75), route(6, 0.125)]);
    assert.deepEqual(plain(send.followsMute(true)), [route(7, 1)]);

    const vca = edits.vca("v");
    const vcaRow = (control, value, lane, extra = {}) =>
      ({ owner: { kind: "vca", id: "v" }, section: own, control, lane, value, ...extra });
    assert.deepEqual(plain(vca.faderDb(1.5, { channel: "right", smoothingSamples: 64 })),
      [vcaRow(1, 1.5, "right", { rampSamples: 64 })]);
    assert.deepEqual(plain(vca.mute(true, { channel: "left" })), [vcaRow(2, 1, "left")]);

    const compressor = track.console("slot", "miso.compressor");
    const threshold = CATALOG.effects.find((e) => e.id === "miso.compressor")
      .parameters.find((p) => p.name === "threshold");
    assert.deepEqual(plain(compressor.parameter("threshold", -24, { smoothingSamples: 8 })),
      [{ owner: strip, section: { kind: "console", slot: "slot" }, control: threshold.id, lane: "both", value: -24, rampSamples: 8 }]);
    assert.deepEqual(plain(track.insert("ins", "miso.compressor").bypass(true)),
      [{ owner: strip, section: { kind: "inserts", effect: "ins" }, control: 0, lane: "both", value: 1 }]);
  });

  test("an absent ramp stays absent, an explicit one is carried, 0 included", () => {
    const track = new LiveControlEdits(MAP).track("t");
    assert.equal("rampSamples" in track.faderDb(-3).edits[0], false);
    assert.equal(track.faderDb(-3, { smoothingSamples: 0 }).edits[0].rampSamples, 0);
    assert.equal(track.faderDb(-3, { smoothingSamples: 4_294_967_295 }).edits[0].rampSamples, 4_294_967_295);
  });

  test("the builders refuse only what the frame cannot hold", () => {
    const edits = new LiveControlEdits(MAP);
    const track = edits.track("t");
    // A value is a number, a ramp a u32, a lane one of three names, an ID a string.
    for (const bad of [-1, 1.5, 4_294_967_296, Number.NaN]) {
      assert.throws(() => track.faderDb(0, { smoothingSamples: bad }), /smoothingSamples must be a u32/);
    }
    assert.throws(() => track.faderDb(0, { channel: "diagonal" }), /channel must be left, right, or both/);
    assert.throws(() => edits.track(1), (error) => error instanceof MisoUsageError);
    assert.throws(() => edits.route(null), MisoUsageError);
    assert.throws(() => edits.vca(undefined), MisoUsageError);
    // Nothing else: a value outside any domain, a lane a row does not allow, a bad ID are built and
    // left to the core.
    assert.equal(track.faderDb(30).edits[0].value, 30);
    assert.equal(track.faderDb(Number.NaN).edits[0].value, Number.NaN);
    assert.equal(track.pan(-2, 2).edits.length, 2);
    assert.equal(edits.track("nope").faderDb(0).edits[0].address.owner.id, "nope");
    assert.equal(edits.route("drums-out").mute(true).edits[0].address.owner.id, "drums-out");
    const compressor = track.console("slot", "miso.compressor");
    assert.equal(compressor.parameter("threshold", 100).edits[0].value, 100);
    assert.equal(compressor.parameter("threshold", -18, { channel: "left" }).edits[0].address.lane, "left");
    assert.throws(() => compressor.parameter({ key: "threshold", value: true }), /must be numeric/);
    assert.throws(() => compressor.parameter({ key: "threshold", value: -18, unit: "db" }), /unknown field 'unit'/);
    assert.throws(() => compressor.parameter({ key: "no-such", value: 1 }), /has no parameter 'no-such'/);
  });

  test("a value edit needs the instance's stable ID, which an index resolves only with a session", () => {
    const edits = new LiveControlEdits(MAP);
    const byIndex = edits.track("t").effect("console", 0, "miso.compressor");
    assert.throws(() => byIndex.parameter("threshold", -18), /stable ID/);
    assert.throws(() => byIndex.bypass(true), /stable ID/);
    assert.throws(() => edits.track("t").insert(0, "miso.compressor"), /nothing to resolve an index against/);
  });
});

describe("#1563B gate 2 -- one refusal on both engines", () => {
  const CASES = [
    ["a fader of 30 dB", (edit) => edit.track("t").faderDb(30), "control.domain"],
    ["a pan of 2.0", (edit) => edit.track("t").pan(2, 0), "control.domain"],
    ["a trim of NaN", (edit) => edit.track("t").trimDb(Number.NaN), "control.domain"],
    ["a fader on an unknown strip ID", (edit) => edit.track("nope").faderDb(-6), "control.address.unknown"],
  ];

  async function refuseAndCompare(controls, engine, untouched, set) {
    const before = await controls.queue.flush();
    let refusal;
    try {
      await controls.submit(set);
    } catch (error) {
      refusal = error;
    }
    assert.ok(refusal instanceof MisoControlRefusal, "the core refused");
    for (let block = 0; block < 4; block += 1) {
      feed(engine, block);
      feed(untouched, block);
      const [a, b] = [engine.render(), untouched.render()];
      assert.deepEqual([...a.left], [...b.left], `left block ${block}`);
      assert.deepEqual([...a.right], [...b.right], `right block ${block}`);
    }
    assert.equal(refusal.revision, before.revision, "a refusal commits no revision");
    return refusal;
  }

  for (const transport of ["headless", "browser"]) {
    test(`${transport}: each refusal carries the core's code, once, and changes nothing`, async () => {
      for (const [what, build, code] of CASES) {
        const engine = await boot(compressorDocument());
        const untouched = await boot(compressorDocument());
        try {
          const controls = transport === "headless"
            ? engine.liveControls()
            : await createBrowserLiveControls(engineHost(engine));
          const refusal = await refuseAndCompare(controls, engine, untouched, build(controls.edit));
          assert.equal(refusal.code, code, what);
          assert.equal(refusal.status, "validation_failed", what);
          assert.equal(refusal.editIndex, 0, what);
        } finally {
          engine.dispose();
          untouched.dispose();
        }
      }
    });
  }
});

/** Feed one block of a distinct deterministic signal per source, so a gain moves only its own sum. */
function renderBlocks(engine, from, count) {
  const out = [];
  for (let block = from; block < from + count; block += 1) {
    feedDistinct(engine, block);
    out.push(engine.render());
  }
  return out;
}

/**
 * Gate 3, through the shipped module: send `edit` to a live engine, and after the edit has settled
 * its output is that of an engine booted from the document that holds the value. `exact` rows are
 * memoryless, so the blocks are equal bit for bit; a row with filter or detector state is held to a
 * tolerance, because the booted engine's state has a different history.
 */
async function liveEqualsBooted({ base, booted, edit, path = "live", exact = true, settle = 24 }) {
  const live = await boot(base);
  const reference = await boot(booted);
  try {
    const before = [renderBlocks(live, 0, 4), renderBlocks(reference, 0, 4)];
    assert.ok(
      before[0].some((block, index) => block.left.some((sample, at) => sample !== before[1][index].left[at])
        || block.right.some((sample, at) => sample !== before[1][index].right[at])),
      "the edit must be audible, or the comparison proves nothing",
    );
    const controls = live.liveControls();
    const report = await controls.submit(edit(controls.edit));
    assert.equal(report.ok, true);
    assert.equal(report.path, path);
    renderBlocks(live, 4, settle);
    renderBlocks(reference, 4, settle);
    const [a, b] = [renderBlocks(live, 4 + settle, 4), renderBlocks(reference, 4 + settle, 4)];
    let peak = 0;
    let worst = 0;
    a.forEach((block, index) => {
      for (const side of ["left", "right"]) {
        block[side].forEach((sample, at) => {
          peak = Math.max(peak, Math.abs(b[index][side][at]));
          worst = Math.max(worst, Math.abs(sample - b[index][side][at]));
          if (exact) assert.equal(sample, b[index][side][at], `${side} block ${index} sample ${at}`);
        });
      }
    });
    assert.ok(peak > 0, "the mix carries signal");
    if (!exact) assert.ok(worst <= 1e-2 * peak, `settled to the booted value (error ${worst} of peak ${peak})`);
  } finally {
    live.dispose();
    reference.dispose();
  }
}

describe("#1563B gate 3 -- every builder renders as the document that holds its value", () => {
  const frames = 128 * 64;
  const doc = () => sessionDocument({ frames });
  const track = (mutate) => (document) => { mutate(document.tracks[0]); };
  const both = (track, section, key, value) => {
    track[section].left[key] = value;
    track[section].right[key] = value;
  };

  const ROWS = [
    ["track fader, both lanes", doc(), track((t) => { t.fader.left_db = -6; t.fader.right_db = -6; }),
      (e) => e.track("t").faderDb(-6, { smoothingSamples: 0 })],
    ["track fader, one lane", doc(), track((t) => { t.fader.left_db = -9; }),
      (e) => e.track("t").faderDb(-9, { channel: "left", smoothingSamples: 0 })],
    ["track mute, one lane", doc(), track((t) => { t.fader.left_mute = true; }),
      (e) => e.track("t").mute(true, { channel: "left", smoothingSamples: 0 })],
    ["track pan", doc(), track((t) => { t.pan.left = -0.25; t.pan.right = 0.75; }),
      (e) => e.track("t").pan(-0.25, 0.75, { smoothingSamples: 0 })],
    ["track trim, one lane", doc(), track((t) => { t.builtins.right.trim_db = 3; }),
      (e) => e.track("t").trimDb(3, { channel: "right", smoothingSamples: 0 })],
    ["track polarity", doc(), track((t) => both(t, "builtins", "polarity_invert", true)),
      (e) => e.track("t").polarityInvert(true, { smoothingSamples: 0 })],
  ];
  for (const [name, base, mutate, edit] of ROWS) {
    test(name, () => liveEqualsBooted({ base, booted: variant(base, mutate), edit }));
  }

  test("track matrix, on a strip in matrix mode", async () => {
    const matrix = { ll: 0.5, lr: 0.25, rl: -0.25, rr: 0.75, smoothing_samples: 0 };
    const base = variant(doc(), (d) => {
      delete d.tracks[0].pan;
      d.tracks[0].matrix = { ll: 1, lr: 0, rl: 0, rr: 1, smoothing_samples: 0 };
    });
    await liveEqualsBooted({
      base,
      booted: variant(base, (d) => { d.tracks[0].matrix = matrix; }),
      edit: (e) => e.track("t").matrix({ ll: 0.5, lr: 0.25, rl: -0.25, rr: 0.75 }, { smoothingSamples: 0 }),
    });
  });

  test("a strip in the other shape is refused by the core as inactive", async () => {
    const engine = await boot(doc());
    try {
      const controls = engine.liveControls();
      const refusal = await controls.submit(
        controls.edit.track("t").matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }),
      ).catch((error) => error);
      assert.ok(refusal instanceof MisoControlRefusal);
      assert.equal(refusal.code, "control.inactive");
    } finally {
      engine.dispose();
    }
  });

  const STATEFUL = [
    ["input HPF", track((t) => both(t, "builtins", "hpf_hz", 400)), (e) => e.track("t").hpfHz(400)],
    ["input LPF", track((t) => both(t, "builtins", "lpf_hz", 3000)), (e) => e.track("t").lpfHz(3000)],
    ["both input filters, one lane", track((t) => { t.builtins.left.hpf_hz = 300; t.builtins.left.lpf_hz = 5000; }),
      (e) => e.track("t").inputFilters({ hpfHz: 300, lpfHz: 5000 }, { channel: "left" })],
  ];
  for (const [name, mutate, edit] of STATEFUL) {
    test(name, () => liveEqualsBooted({ base: doc(), booted: variant(doc(), mutate), edit, exact: false }));
  }

  const thresholdId = CATALOG.effects.find((row) => row.id === "miso.compressor")
    .parameters.find((row) => row.name === "threshold").id;
  const compressor = () => compressorDocument(frames);

  test("effect parameter on a console slot", () => liveEqualsBooted({
    base: compressor(),
    booted: variant(compressor(), (d) => {
      d.tracks[0].console[0].params.find((p) => p.parameter_id === thresholdId).value = -40;
    }),
    edit: (e) => e.track("t").console("compressor", "miso.compressor").parameter("threshold", -40),
    exact: false,
    settle: 40,
  }));

  // A console slot always banks: the output settles to the render of the document that holds the
  // bypass. The crossfade itself (the bank renders what the node renders, #1341a2) is gated below.
  test("effect bypass on a console slot", () => liveEqualsBooted({
    base: compressor(),
    booted: variant(compressor(), (d) => { d.tracks[0].console[0].bypass = true; }),
    edit: (e) => e.track("t").console("compressor", "miso.compressor").bypass(true),
    exact: false,
  }));

  test("solo reports the monitor path and leaves the mix alone", async () => {
    const engine = await boot(doc());
    try {
      const controls = engine.liveControls();
      const report = await controls.submit(controls.edit.track("t").solo(true));
      assert.equal(report.ok, true);
      assert.equal(report.path, "monitor");
    } finally {
      engine.dispose();
    }
  });

  test("submix fader", async () => {
    const base = submixDocument();
    await liveEqualsBooted({
      base,
      booted: submixDocument({ drumsFaderDb: -6 }),
      edit: (e) => e.submix("drums").faderDb(-6, { smoothingSamples: 0 }),
    });
  });

  test("send gain, mute and matrix", async () => {
    const base = sendSession().toJson();
    const withRoute = (fields) => variant(base, (d) => {
      Object.assign(d.routes.find((r) => r.id === "kick-verb"), fields);
    });
    await liveEqualsBooted({
      base, booted: withRoute({ gain_db: -12 }),
      edit: (e) => e.route("kick-verb").gainDb(-12, { smoothingSamples: 0 }),
    });
    await liveEqualsBooted({
      base, booted: withRoute({ mute: true }),
      edit: (e) => e.route("kick-verb").mute(true, { smoothingSamples: 0 }),
    });
    await liveEqualsBooted({
      base, booted: withRoute({ channel_matrix: { ll: 0.5, lr: 0.25, rl: 0, rr: 0.75 } }),
      edit: (e) => e.route("kick-verb").matrix({ ll: 0.5, lr: 0.25, rl: 0, rr: 0.75 }, { smoothingSamples: 0 }),
    });
  });

  test("VCA fader and a one-lane mute", async () => {
    await liveEqualsBooted({
      base: vcaSession().toJson(),
      booted: vcaSession({ drums: { leftDb: -6, rightDb: -6 } }).toJson(),
      edit: (e) => e.vca("drums").faderDb(-6, { smoothingSamples: 0 }),
    });
    await liveEqualsBooted({
      base: vcaSession().toJson(),
      booted: vcaSession({ fx: { leftMute: true } }).toJson(),
      edit: (e) => e.vca("fx").mute(true, { channel: "left", smoothingSamples: 0 }),
    });
  });
});

/**
 * #1341a1 and #1341a2, the amendment gate taken from #1563B gate 3: the SDK's `bypass` builder (a
 * `SET_CONTROL` on control `0` of an effect section) crossfades an insert and a console slot over
 * the session's `mute_ms` ramp, on the headless engine and through the browser's core path, with
 * the same bits in every block. A console slot always banks and a lone insert renders per node, so
 * the two cases are the banked kernel and the per-node one (a bank renders exactly what the node
 * renders, #1341a2).
 *
 * Four engines render one input: W never bypassed, D booted bypassed, H bypassed live through the
 * headless controls, B bypassed live through the browser transport (`SET_CONTROL` frames into the
 * core). The C ABI leg of the same law is `crates/capi` `live_tests.rs`
 * (`a_live_bypass_crossfades_over_the_session_mute_ramp_and_lands_on_the_bypassed_control`), and
 * the C ABI against the browser host is `hosts/host-web/tests/control_parity.rs`.
 */
describe("#1341a1, #1341a2 -- the SDK bypass builder crossfades an insert and a console slot over the session ramp", () => {
  const frames = 128 * 64;
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  const threshold = compressor.parameters.find((row) => row.name === "threshold");
  const entry = (id) => effectEntry(
    id,
    compressor.id,
    compressor.parameters.map((row) => ({
      id: row.id,
      unit: row.unitName,
      value: row.id === threshold.id ? -30 : row.default,
      channel: "both",
    })),
  );
  /** The default session `mute_ms` is 10 ms. */
  const rampSamples = 480;
  const quantum = 128;

  const crossfadeCase = async ({ document, bypassed, edit }) => {
    const [wet, dry, headless, browser] = await Promise.all([
      boot(document()),
      boot(variant(document(), bypassed)),
      boot(document()),
      boot(document()),
    ]);
    try {
      const before = 4;
      const total = 16;
      const submit = async () => {
        const headlessControls = headless.liveControls();
        const browserControls = await createBrowserLiveControls(engineHost(browser));
        const reports = [
          await headlessControls.submit(edit(headlessControls.edit)),
          await browserControls.submit(edit(browserControls.edit)),
        ];
        for (const report of reports) {
          assert.equal(report.ok, true);
          assert.equal(report.path, "live");
        }
        assert.equal(reports[0].revision, reports[1].revision, "one revision on both transports");
      };
      const blocks = { wet: [], dry: [], headless: [], browser: [] };
      for (let block = 0; block < total; block += 1) {
        if (block === before) await submit();
        for (const [name, engine] of Object.entries({ wet, dry, headless, browser })) {
          feedDistinct(engine, block);
          blocks[name].push(engine.render());
        }
      }
      const same = (a, b) => a.left.every((x, i) => Object.is(x, b.left[i]))
        && a.right.every((x, i) => Object.is(x, b.right[i]));
      // Bit for bit per block, on both transports.
      blocks.headless.forEach((block, index) => {
        assert.ok(same(block, blocks.browser[index]), `block ${index}: headless equals browser`);
      });
      // Until the submit the live engines are the never-bypassed one.
      for (let block = 0; block < before; block += 1) {
        assert.ok(same(blocks.headless[block], blocks.wet[block]), `block ${block} precedes the edit`);
      }
      assert.ok(!same(blocks.wet[total - 1], blocks.dry[total - 1]), "the bypass is audible");
      // The ramp reaches the output after the chain's own delay; the blocks that sit between the
      // controls are the crossfade: a step has at most one, a ramp over `rampSamples` spans
      // `ceil(rampSamples / quantum)` or one more.
      const between = blocks.headless.map((block, index) =>
        !same(block, blocks.wet[index]) && !same(block, blocks.dry[index]));
      const rampBlocks = Math.ceil(rampSamples / quantum);
      const count = between.filter(Boolean).length;
      assert.ok(count >= rampBlocks && count <= rampBlocks + 1,
        `${count} blocks sit between wet and dry; the 10 ms ramp spans ${rampBlocks} or ${rampBlocks + 1}`);
      const last = between.lastIndexOf(true);
      for (let block = last + 1; block < total; block += 1) {
        assert.ok(same(blocks.headless[block], blocks.dry[block]), `block ${block} equals the booted bypass`);
      }
    } finally {
      for (const engine of [wet, dry, headless, browser]) engine.dispose();
    }
  };

  test("an insert: headless and browser paths render the same crossfade, and it settles on the bypassed document", () => crossfadeCase({
    document: () => sessionDocument({ frames, effects: { inserts: [entry("ins")] } }),
    bypassed: (d) => { d.tracks[0].inserts.effects[0].bypass = true; },
    edit: (e) => e.track("t").insert("ins", "miso.compressor").bypass(true),
  }));

  test("a console slot, which always banks: the same crossfade, and it settles on the bypassed document", () => crossfadeCase({
    document: () => sessionDocument({ frames, effects: { preInsert: [entry("compressor")] } }),
    bypassed: (d) => { d.tracks[0].console[0].bypass = true; },
    edit: (e) => e.track("t").console("compressor", "miso.compressor").bypass(true),
  }));
});

describe("#1563B -- the browser transport posts the same frames and decodes the core's replies", () => {
  test("a transaction goes out as a control frame at the core's revision, and the reply is the report", async () => {
    const engine = await boot(compressorDocument());
    try {
      const frames = [];
      const controls = await createBrowserLiveControls(engineHost(engine, { frames }));
      // The queue starts at the core's committed revision (the document's), not at 0, and the probe
      // that read it drew request ID 1.
      assert.equal(controls.queue.revision, engine.controlRevision());
      assert.ok(controls.queue.revision > 0n);
      const report = await controls.submit(
        controls.edit.track("t").faderDb(-6, { channel: "left", smoothingSamples: 32 }),
        controls.edit.track("t").trimDb(2),
      );
      assert.equal(report.ok, true);
      assert.equal(report.path, "live");
      assert.equal(report.appliedOperations, 2);
      assert.equal(report.revision, engine.controlRevision());
      const ids = frames.map((frame) => new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true));
      // The probe, the strip-order reads of the control list (#1564 D5) and the transaction draw
      // from one allocator: the IDs are 1, 2, 3.. with no repeat and no gap.
      assert.deepEqual(ids, ids.map((_, index) => BigInt(index + 1)), "one allocator serves every frame");
      assert.ok(ids.length >= 3, "a probe, a strip-order read and a transaction");
      assert.equal(controls.queue.requestIds.next(), BigInt(ids.length + 1), "a catalog read would draw the next ID");
    } finally {
      engine.dispose();
    }
  });

  test("one host has one queue and one allocator, and a failed probe is not remembered", async () => {
    // Plausible defect: two queues over one host draw request IDs from two allocators, and the
    // engine's replay cache refuses the second queue's first ID; or a probe that failed once is
    // the answer for good.
    const engine = await boot(compressorDocument());
    try {
      const host = engineHost(engine);
      const control = host.control.bind(host);
      let failures = 1;
      host.control = async (frame) => {
        if (failures > 0) {
          failures -= 1;
          throw new Error("worker gone");
        }
        return control(frame);
      };
      await assert.rejects(createBrowserControlQueue(host), /worker gone/);
      const [queue, again] = await Promise.all([createBrowserControlQueue(host), createBrowserControlQueue(host)]);
      assert.equal(queue, again);
      const controls = await createBrowserLiveControls(host);
      assert.equal(controls.queue, queue);
      assert.equal(queue.maximumFrameBytes, 256 << 10, "the limit the core reported to the probe");
      assert.equal((await controls.submit(controls.edit.track("t").faderDb(-6))).ok, true);
      assert.equal((await createBrowserLiveControls(host)).queue.requestIds, queue.requestIds);
    } finally {
      engine.dispose();
    }
  });

  test("a call the host refuses outright is the engine's error with its diagnostics", async () => {
    const engine = await boot(compressorDocument());
    try {
      const host = engineHost(engine);
      const controls = await createBrowserLiveControls(host);
      host.control = async () => ({
        tag: "miso.control.v1", requestId: 9, result: 10,
        response: new TextEncoder().encode("web.apply.rebuild_pending\t\n"), revision: 1n,
      });
      await assert.rejects(controls.submit(controls.edit.track("t").faderDb(-6)), (error) =>
        error.result === 10 && error.diagnostics[0].code === "web.apply.rebuild_pending");
    } finally {
      engine.dispose();
    }
  });

});

describe("#1563B -- callers are isolated in one transaction queue", () => {
  test("a refused out-of-domain edit does not drop another caller's valid edits", async () => {
    // Plausible defect: coalescing three callers into one atomic transaction refuses the two valid
    // submits with the invalid one, or blames the wrong submit.
    const engine = await boot(sessionDocument({ frames: 128 * 16 }));
    const untouched = await boot(sessionDocument({ frames: 128 * 16 }));
    try {
      const controls = engine.liveControls();
      const edit = controls.edit.track("t");
      // The first goes out alone; the next three are staged while it is in flight and share one
      // transaction.
      const first = controls.submit(edit.faderDb(-3, { smoothingSamples: 0 }));
      const bad = controls.submit(edit.trimDb(99));
      const good = controls.submit(edit.polarityInvert(true, { smoothingSamples: 0 }),
        edit.trimDb(2, { channel: "left", smoothingSamples: 0 }));
      const alsoGood = controls.submit(edit.hpfHz(100));
      const outcomes = await Promise.allSettled([first, bad, good, alsoGood]);
      assert.deepEqual(outcomes.map((o) => o.status), ["fulfilled", "rejected", "fulfilled", "fulfilled"]);
      const refusal = outcomes[1].reason;
      assert.ok(refusal instanceof MisoControlRefusal);
      assert.equal(refusal.code, "control.domain");
      assert.equal(refusal.editIndex, 0, "the index is the submit's own edit");
      assert.equal(outcomes[2].value.appliedOperations, 3, "the valid callers went in one transaction");

      // The valid edits landed: an engine given only them renders the same.
      const lone = untouched.liveControls();
      await lone.submit(lone.edit.track("t").faderDb(-3, { smoothingSamples: 0 }));
      await lone.submit(
        lone.edit.track("t").polarityInvert(true, { smoothingSamples: 0 }),
        lone.edit.track("t").trimDb(2, { channel: "left", smoothingSamples: 0 }),
        lone.edit.track("t").hpfHz(100),
      );
      for (let block = 0; block < 6; block += 1) {
        feed(engine, block);
        feed(untouched, block);
        const [a, b] = [engine.render(), untouched.render()];
        assert.deepEqual([...a.left], [...b.left], `left block ${block}`);
        assert.deepEqual([...a.right], [...b.right], `right block ${block}`);
      }
    } finally {
      engine.dispose();
      untouched.dispose();
    }
  });

  test("an edit that nobody can place is the submit's own refusal, in its own coordinates", async () => {
    const engine = await boot(sessionDocument());
    try {
      const controls = engine.liveControls();
      const edit = controls.edit.track("t");
      const first = controls.submit(edit.faderDb(-3));
      const second = controls.submit(edit.trimDb(1), edit.pan(0, 2));
      const refusal = await second.catch((error) => error);
      await first;
      assert.ok(refusal instanceof MisoControlRefusal);
      assert.equal(refusal.code, "control.domain");
      assert.equal(refusal.editIndex, 2, "trim is edit 0, the left pan edit 1, the right pan edit 2");
    } finally {
      engine.dispose();
    }
  });
});

describe("#1563B -- a ramp the edit does not give is the core's", () => {
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
    const live = await boot(documentWith(controlSmoothing));
    const booted = await boot(documentWith(controlSmoothing, -6));
    try {
      const differs = [];
      for (let block = 0; block < 14; block += 1) {
        if (block === 2) {
          const controls = live.liveControls();
          const report = await controls.submit(controls.edit.track("t").faderDb(-6, options));
          assert.equal(report.ok, true);
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
    // Red if the SDK resolves a ramp of its own (every block settled, the first one included), or
    // if the core ignores the session's table: the default table's 20 ms is 960 samples, 7.5 blocks
    // of 128; 2 ms is 96 samples, inside the first block; an explicit all-zero table is a step.
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

describe("#1563B -- console slots and inserts resolve by ID on a bus of an SDK-built session", () => {
  test("the edit names the instance by its stable ID, whichever way it was addressed", async () => {
    const built = builtSubmixSession();
    const engine = await boot(built);
    try {
      const controls = engine.liveControls();
      const drums = controls.edit.submix("drums");
      const glue = drums.insert("glue", "miso.compressor").parameter("threshold", -18);
      const byIndex = drums.insert(1, "miso.compressor").parameter("threshold", -18);
      const comp = drums.console("comp", "miso.compressor").bypass(true);
      assert.deepEqual(plain(glue), plain(byIndex), "an index and its ID are the same address");
      assert.deepEqual(plain(glue)[0].section, { kind: "inserts", effect: "glue" });
      assert.deepEqual(plain(comp)[0].section, { kind: "console", slot: "comp" });
      assert.deepEqual(plain(controls.edit.submix("verb").insert("glue", "miso.compressor").bypass(true))[0].owner,
        { kind: "strip", id: "verb" });
      assert.throws(() => drums.insert("ghost", "miso.compressor"),
        /submix 'drums' has no insert 'ghost'; its inserts are tone, glue/);
      assert.throws(() => controls.edit.track("kick").insert("glue", "miso.compressor"),
        /track 'kick' has no insert 'glue'/);
      assert.throws(() => drums.insert("tone", "miso.compressor"), /is miso.parametric-eq, not miso.compressor/);
      const report = await controls.submit(glue, comp);
      assert.equal(report.ok, true);

      // The layout is held to the compiled strips.
      assert.throws(
        () => new LiveControlEdits({ tracks: ["kick"], sources: [], metersAttached: false, submixes: [] }, built),
        /declares tracks kick and submixes drums, verb, but the engine compiled tracks kick and submixes none/,
      );
    } finally {
      engine.dispose();
    }
  });
});
