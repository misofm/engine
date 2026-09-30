/**
 * Issue #1097 (console strip S1d): the session console and per-track inserts through the SDK.
 *
 * Four claims, each held against the engine rather than against the SDK's own expectations:
 *
 * 1. Every builder refusal is one the engine makes, with the engine's own diagnostic code: the
 *    builder refuses a document before it exists, and the engine refuses the same defect written
 *    by hand, and both name the same code.
 * 2. The SDK's copy of the console eligibility list is the engine's, effect by effect.
 * 3. What the builder writes round-trips byte for byte through the engine's canonical writer, for
 *    an empty console, both sections, empty inserts and the app shape; and the builder rebuilds the
 *    engine-written console fixtures byte for byte.
 * 4. Live controls address a console slot by its slot ID and an insert by its ID or index, and a
 *    live bypass on each lands on exactly the instance the same bypass authored in the session does.
 *
 * Each test names the red mutation that turns it red.
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { after, before, describe, test } from "node:test";

import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { CONSOLE_ELIGIBLE_EFFECTS, effect, session } from "../src/core/session.ts";
import { createOfflineEngine, validate } from "../src/headless/engine.ts";
import { moduleBytes, ramp } from "./support.mjs";

const REPO_ROOT = resolve(import.meta.dirname, "..", "..");
const CONTENT = `blake3:${"0".repeat(64)}`;
const RACK = Object.fromEntries(ABI_LAYOUT.constants.racks.map((row) => [row.name, row.value]));

let asset;
let scratch;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
  scratch = await mkdtemp(join(tmpdir(), "miso-sdk-console-"));
});

after(async () => {
  if (scratch !== undefined) await rm(scratch, { recursive: true, force: true });
});

/** A strip: EQ -> compressor before the inserts, a limiter after them. */
const STRIP = Object.freeze({
  preInsert: [
    { slot: "eq", effectId: "miso.parametric-eq" },
    { slot: "comp", effectId: "miso.compressor" },
  ],
  postInsert: [{ slot: "limiter", effectId: "miso.true-peak-limiter", linkMode: "maximum" }],
});

/** One track's complete, valid entries for `STRIP`, with a ridden EQ gain. */
function stripEntries() {
  return [
    { slot: "eq", parameters: { "band-1-enabled": true, "band-1-gain": -2 } },
    { slot: "comp" },
    { slot: "limiter" },
  ];
}

function stripBase(consoleSpec = STRIP) {
  return session({ id: "console.eval", sampleRateHz: 48_000, revision: 1 })
    .source("stem", { channels: 2, bitDepth: 24, frames: 4_800, content: CONTENT })
    .console(consoleSpec);
}

/** Finish a one-track strip session: its output, its route, and a ride on the console EQ. */
function finish(builder, tap = "post_pan") {
  return builder
    .output("out")
    .route({
      id: "main",
      source: { kind: "track", trackId: "t", tap },
      destination: { kind: "output_input", outputId: "out" },
    })
    .automation({
      id: "ride",
      target: { trackId: "t", rack: "console", slotId: "eq", parameter: "band-1-gain", channel: "both" },
      segments: [{ shape: "linear", startSample: 0n, endSample: 480n, startValue: -2, endValue: 0 }],
    });
}

function validStrip() {
  return finish(stripBase().track("t", { source: "stem", console: stripEntries() }));
}

/** The valid strip's normalized model, as a mutable JSON value to break by hand. */
function strippedModel() {
  return JSON.parse(JSON.stringify(validStrip().toJSON()));
}

/** Boot a hand-written document and return the engine's first diagnostic code. */
async function engineCode(model) {
  const outcome = await validate(JSON.stringify(model, null, 1), { asset });
  assert.equal(outcome.ok, false, "the engine must refuse the hand-written defect");
  return outcome.diagnostics[0]?.code;
}

function builderCode(build) {
  try {
    build();
  } catch (error) {
    assert.ok(error instanceof MisoUsageError, `expected a MisoUsageError, got ${error}`);
    return error.diagnosticCode;
  }
  assert.fail("the builder accepted the defect");
}

describe("issue #1097 -- every builder refusal is the engine's, with the engine's code", () => {
  test("the strip used by every refusal below is itself valid, and boots", async () => {
    // Without this, every refusal below could be the engine refusing the base document.
    const outcome = await validate(validStrip(), { asset });
    assert.equal(outcome.ok, true, JSON.stringify(outcome.diagnostics));
  });

  /**
   * Each row is one defect: the builder call that makes it, the hand edit that writes it into an
   * otherwise valid document, and the one code both sides must name. Red mutation: give any
   * builder refusal its own code (or none) -> its row goes red on the builder side; relax the
   * matching builder check -> "the builder accepted the defect".
   */
  const cases = [
    {
      name: "a missing console entry",
      code: "console.entry_missing",
      build: () => stripBase().track("t", { source: "stem", console: stripEntries().slice(0, 2) }),
      edit: (model) => model.tracks[0].console.pop(),
    },
    {
      name: "a track with no console entries while the session declares slots",
      code: "console.entry_missing",
      build: () => stripBase().track("t", { source: "stem" }),
      // The automation rides the EQ entry this removes, so it goes too: it would otherwise be the
      // first thing refused, as a target with no instance.
      edit: (model) => { model.tracks[0].console = []; model.automation = []; },
    },
    {
      name: "a repeated console entry",
      code: "id.duplicate",
      build: () => stripBase().track("t", {
        source: "stem",
        console: [{ slot: "eq" }, { slot: "eq" }, { slot: "comp" }, { slot: "limiter" }],
      }),
      edit: (model) => model.tracks[0].console.splice(1, 0, structuredClone(model.tracks[0].console[0])),
    },
    {
      name: "misordered console entries",
      code: "console.entry_order",
      build: () => stripBase().track("t", {
        source: "stem",
        console: [{ slot: "comp" }, { slot: "eq" }, { slot: "limiter" }],
      }),
      edit: (model) => model.tracks[0].console.reverse(),
    },
    {
      name: "an entry naming no declared slot",
      code: "reference.missing_entity",
      build: () => stripBase().track("t", {
        source: "stem",
        console: [...stripEntries(), { slot: "ghost" }],
      }),
      edit: (model) => model.tracks[0].console.push({ slot: "ghost", bypass: false, params: [] }),
    },
    {
      name: "an effect field on a console entry",
      code: "schema.unknown_field",
      build: () => stripBase().track("t", {
        source: "stem",
        console: [{ slot: "eq", quality: "normal" }, { slot: "comp" }, { slot: "limiter" }],
      }),
      edit: (model) => { model.tracks[0].console[0].quality = "normal"; },
    },
    {
      name: "a console sidechain",
      code: "schema.unknown_field",
      build: () => stripBase({
        preInsert: [{
          slot: "comp",
          effectId: "miso.compressor",
          sidechain: { source: { kind: "track", trackId: "t", tap: "post_fader" }, portId: "sidechain-in" },
        }],
      }),
      edit: (model) => { model.console.pre_insert[1].sidechain = { kind: "none" }; },
    },
    {
      name: "a per-track knob on a slot declaration",
      code: "schema.unknown_field",
      build: () => stripBase({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq", bypass: true }] }),
      edit: (model) => { model.console.pre_insert[0].bypass = false; },
    },
    {
      name: "an ineligible console effect (the delay)",
      code: "console.slot.ineligible_effect",
      build: () => stripBase({ postInsert: [{ slot: "limiter", effectId: "miso.delay" }] }),
      edit: (model) => { model.console.post_insert[0].identity.effect_id = "miso.delay"; },
    },
    {
      name: "an ineligible console effect (the multiband, until #1069)",
      code: "console.slot.ineligible_effect",
      build: () => stripBase({ postInsert: [{ slot: "limiter", effectId: "miso.multiband-compressor" }] }),
      edit: (model) => { model.console.post_insert[0].identity.effect_id = "miso.multiband-compressor"; },
    },
    {
      name: "a slot ID repeated across the two sections",
      code: "id.duplicate",
      build: () => stripBase({
        preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }],
        postInsert: [{ slot: "eq", effectId: "miso.true-peak-limiter" }],
      }),
      edit: (model) => { model.console.post_insert[0].slot = "eq"; },
    },
    {
      name: "a slot ID that is not a stable ID",
      code: "id.invalid",
      build: () => stripBase({ preInsert: [{ slot: "EQ", effectId: "miso.parametric-eq" }] }),
      edit: (model) => { model.console.pre_insert[0].slot = "EQ"; },
    },
    {
      name: "an unknown link mode",
      code: "schema.invalid_enum",
      build: () => stripBase({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq", linkMode: "stereo" }] }),
      edit: (model) => { model.console.pre_insert[0].link_mode = "stereo"; },
    },
    {
      name: "an insert ID repeated on one track",
      code: "id.duplicate",
      build: () => stripBase().track("t", {
        source: "stem",
        console: stripEntries(),
        inserts: [
          effect("miso.parametric-eq", {}, { slotId: "x" }),
          effect("miso.delay", {}, { slotId: "x" }),
        ],
      }),
      edit: (model) => {
        const insert = (effectId) => ({
          id: "x", identity: { kind: "native", effect_id: effectId }, quality: "normal", bypass: false,
          link_mode: "dual_mono", params: [], sidechain: { kind: "none" },
        });
        model.tracks[0].inserts.effects = [insert("miso.parametric-eq"), insert("miso.delay")];
      },
    },
    {
      name: "a retired per-track rack key",
      code: "schema.unknown_field",
      build: () => stripBase().track("t", { source: "stem", console: stripEntries(), dynamic: [] }),
      edit: (model) => { model.tracks[0].dynamic = { effects: [] }; },
    },
    {
      name: "a retired tap spelling",
      code: "schema.invalid_enum",
      build: () => finish(stripBase().track("t", { source: "stem", console: stripEntries() }), "post_matrix"),
      edit: (model) => { model.routes[0].source.tap = "post_matrix"; },
    },
    {
      name: "a retired automation rack",
      code: "schema.invalid_enum",
      build: () => stripBase().track("t", { source: "stem", console: stripEntries() }).automation({
        id: "ride",
        target: { trackId: "t", rack: "simd1", slotId: "eq", parameter: "band-1-gain", channel: "both" },
        segments: [{ shape: "step", startSample: 0n, endSample: 480n, startValue: 0, endValue: 0 }],
      }),
      edit: (model) => { model.automation[0].target.rack = "simd1"; },
    },
  ];

  for (const row of cases) {
    test(`${row.name}: ${row.code}, from the builder and from the engine`, async () => {
      assert.equal(builderCode(row.build), row.code, "the builder's code");
      const model = strippedModel();
      row.edit(model);
      assert.equal(await engineCode(model), row.code, "the engine's first diagnostic");
    });
  }

  test("the effect fields a slot owns are refused on an entry, one by one", () => {
    // An entry carries only the track's knobs. Red mutation: admit `channel`'s neighbours in
    // CONSOLE_ENTRY_KEYS -> one of these is silently dropped instead of refused.
    for (const field of ["effectId", "quality", "linkMode", "sidechain", "id", "identity", "params"]) {
      const code = builderCode(() => stripBase().track("t", {
        source: "stem",
        console: [{ slot: "eq", [field]: "x" }, { slot: "comp" }, { slot: "limiter" }],
      }));
      assert.equal(code, "schema.unknown_field", field);
    }
  });

  test("the console is declared once, before the first track", () => {
    // Not a document defect -- an API one -- so it names no engine code.
    const withTrack = session({ id: "late", sampleRateHz: 48_000 })
      .source("stem", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT })
      .track("t", { source: "stem" });
    for (const [build, message] of [
      [() => withTrack.console(STRIP), /before the first track/],
      [() => stripBase().console(STRIP), /declared once/],
    ]) {
      assert.throws(build, message);
      assert.equal(builderCode(build), undefined, String(message));
    }
  });
});

describe("issue #1097 -- the SDK's eligibility list is the engine's", () => {
  test("every catalog effect is a console slot exactly when the engine admits it as one", async () => {
    // The metadata does not publish the list, so the SDK holds a copy; this is what keeps it the
    // engine's. Red mutation: add `miso.delay` to CONSOLE_ELIGIBLE_EFFECTS, or drop the limiter ->
    // the builder and the engine disagree on that effect.
    assert.deepEqual(
      [...CONSOLE_ELIGIBLE_EFFECTS].sort(),
      CATALOG.effects.map((row) => row.id).filter((id) => CONSOLE_ELIGIBLE_EFFECTS.includes(id)).sort(),
      "every listed effect is a catalog effect",
    );
    for (const { id } of CATALOG.effects) {
      const eligible = CONSOLE_ELIGIBLE_EFFECTS.includes(id);
      const built = () => session({ id: "eligibility", sampleRateHz: 48_000 })
        .console({ preInsert: [{ slot: "x", effectId: id }] });
      if (eligible) assert.doesNotThrow(built, id);
      else assert.equal(builderCode(built), "console.slot.ineligible_effect", id);

      const model = strippedModel();
      model.console.pre_insert = [{
        slot: "x", identity: { kind: "native", effect_id: id }, quality: "normal", link_mode: "dual_mono",
      }];
      model.console.post_insert = [];
      model.tracks[0].console = [{ slot: "x", bypass: false, params: [] }];
      model.automation = [];
      const outcome = await validate(JSON.stringify(model), { asset });
      assert.equal(outcome.ok, eligible, `${id}: ${JSON.stringify(outcome.diagnostics)}`);
      if (!eligible) assert.equal(outcome.diagnostics[0]?.code, "console.slot.ineligible_effect", id);
    }
  });
});

/** The engine's canonical re-serialization of a document, through the session validator. */
async function engineCanonical(name, text) {
  const path = join(scratch, `${name}.json`);
  await writeFile(path, text);
  return execFileSync(
    "cargo",
    ["run", "--locked", "-q", "-p", "session-validator", "--", "validate", "--canonical", path],
    { cwd: REPO_ROOT, encoding: "utf8", maxBuffer: 1 << 26, stdio: ["ignore", "pipe", "pipe"] },
  );
}

/** The app's mix: EQ -> compressor as two `pre_insert` slots, unselected tracks bypassed. */
function appShape(tracks = 9) {
  let built = session({ id: "console.app", sampleRateHz: 48_000, revision: 3 })
    .source("mix", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .console({
      preInsert: [
        { slot: "eq", effectId: "miso.parametric-eq" },
        { slot: "compressor", effectId: "miso.compressor" },
      ],
    })
    .output("main-out");
  for (let index = 0; index < tracks; index += 1) {
    const id = `track-${String(index).padStart(3, "0")}`;
    const unselected = index % 3 === 2;
    built = built
      .track(id, {
        source: "mix",
        console: [
          {
            slot: "eq",
            bypass: unselected,
            parameters: { "band-1-enabled": true, "band-1-gain": index - 4, "band-1-frequency": 200 + index * 50 },
          },
          { slot: "compressor", bypass: unselected, parameters: { threshold: -12 - index, ratio: 3 } },
        ],
        inserts: [],
        fader: { leftDb: -index / 2, rightDb: -index / 2 },
      })
      .route({
        id: `${id}-main`,
        source: { kind: "track", trackId: id, tap: "post_pan" },
        destination: { kind: "output_input", outputId: "main-out" },
      });
  }
  return built;
}

describe("issue #1097 -- what the builder writes is what the engine's canonical writer writes", () => {
  const shapes = {
    "empty console": () => session({ id: "console.empty", sampleRateHz: 48_000 })
      .source("stem", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT })
      .track("t", { source: "stem", inserts: [effect("miso.delay", { "delay time": 20 })] })
      .output("out")
      .route({
        id: "main",
        source: { kind: "track", trackId: "t", tap: "insert_return" },
        destination: { kind: "output_input", outputId: "out" },
      }),
    "both sections": () => finish(stripBase().track("t", {
      source: "stem",
      console: stripEntries(),
      inserts: [
        effect("miso.delay", { "delay time": 12 }, { slotId: "echo" }),
        effect("miso.multiband-compressor", {}, { bypass: true }),
      ],
    })),
    "empty inserts": () => finish(stripBase().track("t", {
      source: "stem",
      console: [
        { slot: "eq", bypass: true, parameters: { "band-1-enabled": true, "band-1-gain": -2 } },
        { slot: "comp", parameters: { ratio: { left: 2, right: 5 } } },
        { slot: "limiter" },
      ],
      inserts: [],
    }), "pre_fader"),
    "app shape": () => appShape(),
  };

  test("four shapes round-trip byte for byte through session_validator --canonical", async () => {
    // Gate 2 of #1097, against the engine's own canonical writer rather than a transcription of it.
    // Red mutation: swap `console` and `inserts` in the writer's track order, or emit `bypass`
    // before `slot` in an entry -> the engine's re-serialization differs from the builder's text.
    for (const [name, build] of Object.entries(shapes)) {
      const text = build().toJson();
      assert.equal(await engineCanonical(name.replaceAll(" ", "-"), text), text, name);
      const outcome = await validate(text, { asset });
      assert.equal(outcome.ok, true, `${name}: ${JSON.stringify(outcome.diagnostics)}`);
    }
  });

  test("the shapes are the shapes they are named for", () => {
    // The round trip above proves nothing about a shape that silently degenerated.
    const empty = shapes["empty console"]().toJSON();
    assert.deepEqual(empty.console, { pre_insert: [], post_insert: [] });
    assert.deepEqual(empty.tracks[0].console, []);
    const both = shapes["both sections"]().toJSON();
    assert.deepEqual(both.console.pre_insert.map((slot) => slot.slot), ["eq", "comp"]);
    assert.deepEqual(both.console.post_insert.map((slot) => slot.slot), ["limiter"]);
    assert.deepEqual(both.tracks[0].inserts.effects.map((row) => row.id), ["echo", "insert-2"]);
    assert.deepEqual(shapes["empty inserts"]().toJSON().tracks[0].inserts, { effects: [] });
    const app = shapes["app shape"]().toJSON();
    assert.deepEqual(app.console.pre_insert.map((slot) => slot.identity.effect_id), [
      "miso.parametric-eq", "miso.compressor",
    ]);
    assert.deepEqual(app.console.post_insert, []);
    for (const [index, track] of app.tracks.entries()) {
      assert.deepEqual(track.console.map((entry) => entry.slot), ["eq", "compressor"]);
      assert.deepEqual(track.console.map((entry) => entry.bypass), [index % 3 === 2, index % 3 === 2]);
    }
  });

  /**
   * Rebuild an engine-written document through the builder, reading it only as the author would
   * have written it: slots, entries by parameter NAME and display value, sources, strips, routes.
   */
  function rebuild(document) {
    const model = JSON.parse(document);
    const byId = new Map(CATALOG.effects.map((row) => [row.id, row]));
    const slotEffects = new Map([...model.console.pre_insert, ...model.console.post_insert]
      .map((slot) => [slot.slot, slot.identity.effect_id]));
    const parameters = (effectId, rows) => {
      const descriptor = byId.get(effectId);
      const values = {};
      for (const row of rows) {
        const parameter = descriptor.parameters.find((candidate) => candidate.id === row.parameter_id);
        const value = parameter.domainName === "boolean" ? row.value !== 0
          : parameter.domainName === "enumeration"
            ? parameter.enumChoices.find((choice) => choice.value === row.value).label
            : row.value;
        if (row.channel === "both") values[parameter.name] = value;
        else values[parameter.name] = { ...values[parameter.name], [row.channel]: value };
      }
      return values;
    };
    const slot = (row) => ({
      slot: row.slot, effectId: row.identity.effect_id, quality: row.quality, linkMode: row.link_mode,
    });
    let built = session({
      id: model.session_id,
      sampleRateHz: model.sample_rate_hz,
      revision: BigInt(model.revision),
      quantumFrames: model.quantum_frames,
    });
    for (const source of model.sources) {
      built = built.source(source.id, {
        channels: source.channels, bitDepth: source.bit_depth, frames: BigInt(source.frames), content: source.content,
      });
    }
    built = built.console({
      preInsert: model.console.pre_insert.map(slot),
      postInsert: model.console.post_insert.map(slot),
    });
    for (const id of model.outputs.map((row) => row.id)) built = built.output(id);
    for (const id of model.submixes.map((row) => row.id)) built = built.submix(id);
    const lane = (row) => ({
      polarityInvert: row.polarity_invert, trimDb: row.trim_db, hpfHz: row.hpf_hz, lpfHz: row.lpf_hz,
      delaySamples: row.delay_samples,
    });
    for (const track of model.tracks) {
      assert.deepEqual(track.inserts.effects, [], "the console fixtures carry no inserts");
      built = built.track(track.id, {
        source: { id: track.source_id, left: track.left_source_channel, right: track.right_source_channel },
        builtins: { left: lane(track.builtins.left), right: lane(track.builtins.right) },
        console: track.console.map((entry) => ({
          slot: entry.slot,
          bypass: entry.bypass,
          parameters: parameters(slotEffects.get(entry.slot), entry.params),
        })),
        fader: {
          leftDb: track.fader.left_db, rightDb: track.fader.right_db,
          leftMute: track.fader.left_mute, rightMute: track.fader.right_mute,
        },
        pan: track.pan === undefined
          ? { matrix: track.matrix, smoothingSamples: track.matrix.smoothing_samples }
          : { left: track.pan.left, right: track.pan.right, smoothingSamples: track.pan.smoothing_samples },
      });
    }
    for (const route of model.routes) {
      built = built.route({
        id: route.id,
        source: route.source.kind === "track"
          ? { kind: "track", trackId: route.source.track_id, tap: route.source.tap }
          : { kind: "submix_output", submixId: route.source.submix_id },
        destination: route.destination.kind === "output_input"
          ? { kind: "output_input", outputId: route.destination.output_id }
          : { kind: "submix_input", submixId: route.destination.submix_id },
        matrix: route.channel_matrix,
        gainDb: route.gain_db,
      });
    }
    assert.deepEqual(model.automation, []);
    return built;
  }

  test("the author-session skill's worked session is canonical and boots", async () => {
    // #1097 gate 5: the skill teaches the console shape by example, so its example must stay a
    // document the engine takes. `session_validator --canonical` runs all five stages (grammar,
    // typed model, compile, builtins, effects) and prints nothing on a refusal; the wasm boot then
    // compiles the graph. Red mutation: give the worked session's bass track a `dynamic` key or
    // move its keyed compressor into the console -> the validator refuses and this goes red.
    const path = join(REPO_ROOT, ".claude", "skills", "author-session", "worked-session.json");
    const text = await readFile(path, "utf8");
    assert.equal(await engineCanonical("worked-session", text), text);
    const outcome = await validate(text, { asset });
    assert.equal(outcome.ok, true, JSON.stringify(outcome.diagnostics));
    const model = JSON.parse(text);
    assert.ok(model.console.pre_insert.length > 0 && model.console.post_insert.length > 0);
    assert.ok(model.tracks.some((track) => track.inserts.effects.some((row) => row.sidechain.kind === "routed")));
  });

  for (const [fixture, what] of [
    ["console-sixty-four-track-app.json", "the app shape: EQ -> compressor pre_insert, 2-mod-3 bypass"],
    ["console-sixty-four-track-intended.json", "both sections: EQ -> compressor, then the limiter"],
  ]) {
    test(`the builder rebuilds ${fixture} byte for byte (${what})`, async () => {
      // These were written by the engine's canonical writer and are pinned by
      // check-console-fixtures.sh. Red mutation: emit the entry's `params` before `bypass`, or the
      // root `console` after `tracks` -> the rebuilt text differs.
      const expected = await readFile(join(REPO_ROOT, "fixtures", "session", "v1", fixture), "utf8");
      assert.equal(rebuild(expected).toJson(), expected);
    });
  }
});

/** Deterministic PCM for every source, block by block. */
function feed(engine, block) {
  const shape = engine.shape();
  shape.sources.forEach((source, sourceIndex) => {
    const planes = Array.from({ length: source.channels }, (_unused, channel) =>
      ramp(shape.quantumFrames, 101 + sourceIndex * 16 + channel + block * 1024));
    assert.equal(engine.submitSource({
      sourceId: source.id, generation: 1n, startFrame: BigInt(block * shape.quantumFrames), planes,
      endOfRegion: false,
    }).ok, true);
  });
}

async function renderBlocks(document, edit, blocks = 6) {
  const engine = await createOfflineEngine(document, { asset, liveControls: { commandQueueRecords: 64 } });
  try {
    if (edit !== undefined) {
      const controls = engine.liveControls();
      const report = await controls.submit(edit(controls));
      assert.equal(report.ok, true, report.reasonName);
    }
    const output = [];
    for (let block = 0; block < blocks; block += 1) {
      feed(engine, block);
      const rendered = engine.render();
      output.push([...rendered.left], [...rendered.right]);
    }
    return output;
  } finally {
    engine.dispose();
  }
}

/**
 * Two tracks, each with a `pre_insert` EQ, an insert EQ and a `post_insert` EQ, all audibly
 * different, so a bypass on any one instance of track `b` is heard and told apart.
 */
function addressedStrip(bypassed = undefined) {
  const eq = (gain, frequency) => ({ "band-1-enabled": true, "band-1-gain": gain, "band-1-frequency": frequency });
  let built = session({ id: "console.live", sampleRateHz: 48_000, revision: 1 })
    .source("sa", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .source("sb", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .console({
      preInsert: [
        { slot: "pre-eq", effectId: "miso.parametric-eq" },
        { slot: "pre-comp", effectId: "miso.compressor" },
      ],
      postInsert: [{ slot: "post-eq", effectId: "miso.parametric-eq" }],
    })
    .output("out");
  for (const id of ["a", "b"]) {
    const off = (name) => id === "b" && bypassed === name;
    built = built
      .track(id, {
        source: `s${id}`,
        console: [
          { slot: "pre-eq", bypass: off("pre-eq"), parameters: eq(9, 400) },
          { slot: "pre-comp", bypass: off("pre-comp"), parameters: { threshold: -30, ratio: 4 } },
          { slot: "post-eq", bypass: off("post-eq"), parameters: eq(-9, 3_000) },
        ],
        inserts: [
          effect("miso.parametric-eq", eq(6, 1_200), { slotId: "ins-eq", bypass: off("ins-eq") }),
          effect("miso.delay", { "delay time": 3 }, { slotId: "echo" }),
        ],
      })
      .route({
        id: `${id}-out`,
        source: { kind: "track", trackId: id, tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
  }
  return built;
}

describe("issue #1097 -- live controls address console slots by slot ID and inserts by ID or index", () => {
  test("stable IDs resolve to S1c's live addresses and rack codes", async () => {
    // Console slot index is the slot's position in pre_insert-then-post_insert order. Red
    // mutation: resolve post_insert slots from zero (the section's own index) -> `post-eq` is
    // written as console slot 0, which is `pre-eq`.
    const engine = await createOfflineEngine(addressedStrip(), { asset, liveControls: { commandQueueRecords: 8 } });
    try {
      const track = engine.liveControls().edit.track("b");
      const address = (edit) => [edit.trackIndex, edit.rack, edit.effectIndex];
      assert.deepEqual(address(track.console("pre-eq", "miso.parametric-eq").bypass(true)), [1, RACK.console, 0]);
      assert.deepEqual(address(track.console("pre-comp", "miso.compressor").bypass(true)), [1, RACK.console, 1]);
      assert.deepEqual(address(track.console("post-eq", "miso.parametric-eq").bypass(true)), [1, RACK.console, 2]);
      assert.deepEqual(address(track.insert("ins-eq", "miso.parametric-eq").bypass(true)), [1, RACK.inserts, 0]);
      assert.deepEqual(address(track.insert("echo", "miso.delay").bypass(true)), [1, RACK.inserts, 1]);
      assert.deepEqual(address(track.insert(1, "miso.delay").bypass(true)), [1, RACK.inserts, 1]);
      assert.deepEqual(address(track.effect("console", 2, "miso.parametric-eq").bypass(true)), [1, 3, 2]);
      assert.deepEqual(address(track.effect("inserts", 0, "miso.parametric-eq").bypass(true)), [1, 1, 0]);
      assert.equal(RACK.console, 3);
      assert.equal(RACK.inserts, 1);

      assert.throws(() => track.console("ghost", "miso.parametric-eq"), /no console slot 'ghost'; its slots are pre-eq, pre-comp, post-eq/);
      assert.throws(() => track.console("pre-eq", "miso.compressor"), /'pre-eq' is miso\.parametric-eq, not miso\.compressor/);
      assert.throws(() => track.insert("ghost", "miso.delay"), /no insert 'ghost'; its inserts are ins-eq, echo/);
      assert.throws(() => track.insert(2, "miso.delay"), /no insert at index 2/);
      for (const retired of ["simd1", "dynamic", "simd2"]) {
        assert.throws(() => track.effect(retired, 0, "miso.parametric-eq"), /simd1, dynamic and simd2 are retired/);
      }
    } finally {
      engine.dispose();
    }
  });

  test("a document booted from text resolves IDs only through withSession()", async () => {
    // The SDK never parses a document, so text has no layout. Red mutation: fall back to an empty
    // layout -> `console('pre-eq')` answers "no console slot" instead of naming withSession().
    const builder = addressedStrip();
    const engine = await createOfflineEngine(builder.toJson(), { asset, liveControls: { commandQueueRecords: 8 } });
    try {
      const controls = engine.liveControls();
      assert.throws(() => controls.edit.track("b").console("pre-eq", "miso.parametric-eq"), /withSession/);
      assert.throws(() => controls.edit.track("b").insert("ins-eq", "miso.parametric-eq"), /withSession/);
      const edit = controls.withSession(builder).edit.track("b").console("post-eq", "miso.parametric-eq").bypass(true);
      assert.deepEqual([edit.rack, edit.effectIndex], [RACK.console, 2]);
      const other = session({ id: "other", sampleRateHz: 48_000 })
        .source("s", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT })
        .track("z", { source: "s" });
      assert.throws(() => controls.withSession(other), /the engine compiled a, b/);
    } finally {
      engine.dispose();
    }
  });

  test("a live bypass on a console slot or an insert renders the session bypass of exactly that instance", async () => {
    // The native twin of the browser gate. Each live bypass is applied before the first quantum,
    // so it must equal, bit for bit, the document authored with that one instance bypassed; it
    // must be audible; and the four addresses must be heard as four different instances.
    // Red mutation: address post_insert slots by their index within the section (post-eq -> 0),
    // or inserts through the console code -> the live render equals another instance's bypass.
    const base = await renderBlocks(addressedStrip());
    const renders = new Map();
    for (const [name, edit] of [
      ["pre-eq", (controls) => controls.edit.track("b").console("pre-eq", "miso.parametric-eq").bypass(true)],
      ["pre-comp", (controls) => controls.edit.track("b").console("pre-comp", "miso.compressor").bypass(true)],
      ["post-eq", (controls) => controls.edit.track("b").console("post-eq", "miso.parametric-eq").bypass(true)],
      ["ins-eq", (controls) => controls.edit.track("b").insert("ins-eq", "miso.parametric-eq").bypass(true)],
    ]) {
      const live = await renderBlocks(addressedStrip(), edit);
      const authored = await renderBlocks(addressedStrip(name));
      assert.deepEqual(live, authored, `${name}: the live bypass must equal the authored bypass`);
      assert.notDeepEqual(live, base, `${name}: the bypass must be audible`);
      renders.set(name, JSON.stringify(live));
    }
    assert.equal(new Set(renders.values()).size, 4, "each address reaches a different instance");
  });
});
