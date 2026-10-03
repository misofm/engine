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
 *    They resolve IDs only against the session the engine booted (`withSession()` holds a builder
 *    to the booted bytes), and they refuse a live lift the engine would acknowledge and ignore (a
 *    session-bypassed delay or multiband keeps its prepared bypass).
 * 5. The SDK's copies of two engine tables the metadata does not publish -- each effect's link
 *    modes, and the effects that keep a prepared bypass -- are the engine's, effect by effect.
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
import { EngineLiveControls, PREPARED_BYPASS_EFFECTS } from "../src/core/live-controls.ts";
import { CONSOLE_ELIGIBLE_EFFECTS, EFFECT_LINK_MODES, effect, session } from "../src/core/session.ts";
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

/** A hand-written insert record, as the engine's schema spells it, with `fields` overriding. */
function nativeInsert(effectId, fields = {}) {
  return {
    id: "x", identity: { kind: "native", effect_id: effectId }, quality: "normal", bypass: false,
    link_mode: "dual_mono", params: [], sidechain: { kind: "none" }, ...fields,
  };
}

/** The valid strip with its automation retargeted: `target` overrides the ride on the console EQ. */
function ride(target) {
  return stripBase().track("t", { source: "stem", console: stripEntries() }).automation({
    id: "ride",
    target: { trackId: "t", rack: "console", slotId: "eq", parameter: "band-1-gain", channel: "both", ...target },
    segments: [{ shape: "step", startSample: 0n, endSample: 480n, startValue: 0, endValue: 0 }],
  });
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
      name: "a slot quality no launch effect publishes",
      code: "effect.quality.unsupported",
      build: () => stripBase({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq", quality: "draft" }] }),
      edit: (model) => { model.console.pre_insert[0].quality = "draft"; },
    },
    {
      name: "a slot quality outside the grammar",
      code: "schema.invalid_enum",
      build: () => stripBase({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq", quality: "ultra" }] }),
      edit: (model) => { model.console.pre_insert[0].quality = "ultra"; },
    },
    {
      name: "an insert quality no launch effect publishes",
      code: "effect.quality.unsupported",
      build: () => effect("miso.delay", {}, { quality: "high" }),
      edit: (model) => { model.tracks[0].inserts.effects = [nativeInsert("miso.delay", { quality: "high" })]; },
    },
    {
      name: "a link mode the effect does not support, on a slot (the EQ, maximum)",
      code: "effect.link_mode.unsupported",
      build: () => stripBase({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq", linkMode: "maximum" }] }),
      edit: (model) => { model.console.pre_insert[0].link_mode = "maximum"; },
    },
    {
      name: "a link mode the effect does not support, on an insert (the limiter, average)",
      code: "effect.link_mode.unsupported",
      build: () => effect("miso.true-peak-limiter", {}, { linkMode: "average" }),
      edit: (model) => {
        model.tracks[0].inserts.effects = [nativeInsert("miso.true-peak-limiter", { link_mode: "average" })];
      },
    },
    {
      name: "a console entry bypass that is not a boolean",
      code: "schema.wrong_type",
      build: () => stripBase().track("t", {
        source: "stem",
        console: [{ slot: "eq", bypass: 1 }, { slot: "comp" }, { slot: "limiter" }],
      }),
      edit: (model) => { model.tracks[0].console[0].bypass = 1; },
    },
    {
      name: "an insert bypass that is not a boolean",
      code: "schema.wrong_type",
      build: () => effect("miso.delay", {}, { bypass: "yes" }),
      edit: (model) => { model.tracks[0].inserts.effects = [nativeInsert("miso.delay", { bypass: "yes" })]; },
    },
    {
      name: "a console automation target naming no slot",
      code: "reference.missing_entity",
      build: () => ride({ rack: "console", slotId: "ghost", parameter: "band-1-gain" }),
      edit: (model) => { model.automation[0].target.effect_id = "ghost"; },
    },
    {
      name: "an insert automation target naming no insert",
      code: "reference.missing_entity",
      build: () => ride({ rack: "inserts", slotId: "ghost", parameter: "band-1-gain" }),
      edit: (model) => { model.automation[0].target.rack = "inserts"; model.automation[0].target.effect_id = "ghost"; },
    },
    {
      name: "an automation target on a parameter its instance does not declare",
      code: "reference.missing_entity",
      build: () => ride({ rack: "console", slotId: "eq", parameter: "band-1-frequency" }),
      edit: (model) => { model.automation[0].target.parameter_id = 3; },
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

describe("issue #1097 -- the SDK's link-mode table is the engine's", () => {
  test("every catalog effect takes exactly the link modes the engine admits, as an insert and as a slot", async () => {
    // The metadata does not publish `supported_link_modes`, so the SDK holds EFFECT_LINK_MODES;
    // this is what keeps it the engine's (#1097 verdict L1). Red mutation: admit `maximum` for the
    // EQ, or drop `maximum` from the limiter -> the builder and the engine disagree on that cell.
    assert.deepEqual(Object.keys(EFFECT_LINK_MODES).sort(), CATALOG.effects.map((row) => row.id).sort());
    for (const { id } of CATALOG.effects) {
      for (const mode of ["dual_mono", "maximum", "average"]) {
        const supported = EFFECT_LINK_MODES[id].includes(mode);
        const where = `${id} at ${mode}`;
        const asInsert = () => effect(id, {}, { linkMode: mode });
        if (supported) assert.doesNotThrow(asInsert, where);
        else assert.equal(builderCode(asInsert), "effect.link_mode.unsupported", where);

        const insertModel = strippedModel();
        insertModel.tracks[0].inserts.effects = [nativeInsert(id, { link_mode: mode })];
        const inserted = await validate(JSON.stringify(insertModel), { asset });
        assert.equal(inserted.ok, supported, `${where} as an insert: ${JSON.stringify(inserted.diagnostics)}`);
        if (!supported) assert.equal(inserted.diagnostics[0]?.code, "effect.link_mode.unsupported", where);

        if (!CONSOLE_ELIGIBLE_EFFECTS.includes(id)) continue;
        const asSlot = () => session({ id: "link", sampleRateHz: 48_000 })
          .console({ preInsert: [{ slot: "x", effectId: id, linkMode: mode }] });
        if (supported) assert.doesNotThrow(asSlot, where);
        else assert.equal(builderCode(asSlot), "effect.link_mode.unsupported", where);

        const slotModel = strippedModel();
        slotModel.console.pre_insert = [{
          slot: "x", identity: { kind: "native", effect_id: id }, quality: "normal", link_mode: mode,
        }];
        slotModel.console.post_insert = [];
        slotModel.tracks[0].console = [{ slot: "x", bypass: false, params: [] }];
        slotModel.automation = [];
        const slotted = await validate(JSON.stringify(slotModel), { asset });
        assert.equal(slotted.ok, supported, `${where} as a slot: ${JSON.stringify(slotted.diagnostics)}`);
        if (!supported) assert.equal(slotted.diagnostics[0]?.code, "effect.link_mode.unsupported", where);
      }
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
    const lane = (row) => ({
      polarityInvert: row.polarity_invert, trimDb: row.trim_db, hpfHz: row.hpf_hz, lpfHz: row.lpf_hz,
      delaySamples: row.delay_samples,
    });
    // A track's strip and a submix's are the same keys: a submix is passed as a `SubmixSpec`.
    const strip = (row) => {
      assert.deepEqual(row.inserts.effects, [], "the console fixtures carry no inserts");
      return {
        builtins: { left: lane(row.builtins.left), right: lane(row.builtins.right) },
        console: row.console.map((entry) => ({
          slot: entry.slot,
          bypass: entry.bypass,
          parameters: parameters(slotEffects.get(entry.slot), entry.params),
        })),
        fader: {
          leftDb: row.fader.left_db, rightDb: row.fader.right_db,
          leftMute: row.fader.left_mute, rightMute: row.fader.right_mute,
        },
        pan: row.pan === undefined
          ? { matrix: row.matrix, smoothingSamples: row.matrix.smoothing_samples }
          : { left: row.pan.left, right: row.pan.right, smoothingSamples: row.pan.smoothing_samples },
      };
    };
    for (const submix of model.submixes) built = built.submix(submix.id, strip(submix));
    for (const track of model.tracks) {
      built = built.track(track.id, {
        source: { id: track.source_id, left: track.left_source_channel, right: track.right_source_channel },
        ...strip(track),
      });
    }
    const source = (row) => row.kind === "track"
      ? { kind: "track", trackId: row.track_id, tap: row.tap }
      : { kind: "submix", submixId: row.submix_id, tap: row.tap };
    for (const route of model.routes) {
      built = built.route({
        id: route.id,
        source: source(route.source),
        destination: route.destination.kind === "output_input"
          ? { kind: "output_input", outputId: route.destination.output_id }
          : { kind: "submix_input", submixId: route.destination.submix_id },
        matrix: route.channel_matrix,
        gainDb: route.gain_db,
        mute: route.mute,
        followsMute: route.follows_mute,
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

  test("the builder rebuilds an engine-written submix strip and bus tap byte for byte", async () => {
    // Issue #1205: `rebuild()` passes each submix's strip as a `SubmixSpec` and each bus source
    // with its tap, the way an author would. Red mutation: drop the strip in `normalizeSubmix` for
    // the spec'd form, or the tap from the submix source -> the rebuilt text differs.
    const written = stripBase()
      .track("t", { source: "stem", console: stripEntries() })
      .submix("bus", {
        builtins: { left: { trimDb: 1.5 }, right: { lpfHz: 9_000 } },
        console: [
          { slot: "eq", parameters: { "band-1-enabled": true, "band-1-gain": 2 } },
          { slot: "comp", bypass: true },
          { slot: "limiter" },
        ],
        fader: { leftDb: -2, rightMute: true },
        pan: { left: 0.2, right: 0.4 },
      })
      .output("out")
      .route({
        id: "bus-out",
        source: { kind: "submix", submixId: "bus", tap: "insert_return" },
        destination: { kind: "output_input", outputId: "out" },
      })
      .route({
        id: "t-bus",
        source: { kind: "track", trackId: "t", tap: "insert_send" },
        destination: { kind: "submix_input", submixId: "bus" },
        // #1218 (VERIFY-2 M8): the non-default value, so a rebuild that drops `followsMute`
        // writes the submix default `true` and differs.
        followsMute: false,
      });
    const expected = await engineCanonical("bus-rebuild", written.toJson());
    const model = JSON.parse(expected);
    assert.equal(model.submixes[0].fader.right_mute, true, "the strip must reach the engine's text");
    assert.deepEqual(model.routes[0].source, { kind: "submix", submix_id: "bus", tap: "insert_return" });
    assert.equal(rebuild(expected).toJson(), expected);
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
 * different, so a bypass on any one instance of track `b` is heard and told apart. The two tracks'
 * insert chains differ in order, so an insert ID resolved against the wrong track's chain lands on
 * another index.
 *
 * `variant` makes a session that is *not* this one, for `withSession()` to refuse: its console
 * reordered, a slot renamed, or track `b`'s insert chain changed.
 */
function addressedStrip(bypassed = undefined, variant = undefined) {
  const eq = (gain, frequency) => ({ "band-1-enabled": true, "band-1-gain": gain, "band-1-frequency": frequency });
  const preEq = variant === "renamed slot" ? "pre-eq-2" : "pre-eq";
  const preInsert = [
    { slot: preEq, effectId: "miso.parametric-eq" },
    { slot: "pre-comp", effectId: "miso.compressor" },
  ];
  if (variant === "reordered console") preInsert.reverse();
  let built = session({ id: "console.live", sampleRateHz: 48_000, revision: 1 })
    .source("sa", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .source("sb", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .console({ preInsert, postInsert: [{ slot: "post-eq", effectId: "miso.parametric-eq" }] })
    .output("out");
  for (const id of ["a", "b"]) {
    const off = (name) => id === "b" && bypassed === name;
    const entries = {
      [preEq]: { slot: preEq, bypass: off("pre-eq"), parameters: eq(9, 400) },
      "pre-comp": { slot: "pre-comp", bypass: off("pre-comp"), parameters: { threshold: -30, ratio: 4 } },
    };
    const inserts = [
      effect("miso.parametric-eq", eq(6, 1_200), { slotId: "ins-eq", bypass: off("ins-eq") }),
      effect("miso.delay", { "delay time": 3 }, { slotId: "echo" }),
    ];
    // Track `a` runs its chain the other way round, so `ins-eq` is insert 1 on `a` and 0 on `b`.
    if (id === "a" || (id === "b" && variant === "another insert chain")) inserts.reverse();
    built = built
      .track(id, {
        source: `s${id}`,
        console: [
          ...preInsert.map((slot) => entries[slot.slot]),
          { slot: "post-eq", bypass: off("post-eq"), parameters: eq(-9, 3_000) },
        ],
        inserts,
      })
      .route({
        id: `${id}-out`,
        source: { kind: "track", trackId: id, tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
  }
  return built;
}

/** The refusal `withSession()` makes for a session that did not write the booted document. */
const NOT_BOOTED = (error) => error instanceof MisoUsageError
  && /not the document this engine booted/.test(error.message);

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
      // Each track's inserts resolve against its own chain (#1097 verdict L3). Red mutation:
      // resolve insert() against the first track's chain -> `b`'s `ins-eq` is written at index 1.
      const a = engine.liveControls().edit.track("a");
      assert.deepEqual(address(a.insert("ins-eq", "miso.parametric-eq").bypass(true)), [0, RACK.inserts, 1]);
      assert.deepEqual(address(a.insert("echo", "miso.delay").bypass(true)), [0, RACK.inserts, 0]);
      assert.throws(() => a.insert(0, "miso.parametric-eq"), /insert 'echo' is miso\.delay, not miso\.parametric-eq/);

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
      // A different builder object that writes the same document is the same session.
      const again = controls.withSession(addressedStrip()).edit.track("b").insert("ins-eq", "miso.parametric-eq");
      assert.deepEqual([again.bypass(true).rack, again.bypass(true).effectIndex], [RACK.inserts, 0]);
      const other = session({ id: "other", sampleRateHz: 48_000 })
        .source("s", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT })
        .track("z", { source: "s" });
      assert.throws(() => controls.withSession(other), NOT_BOOTED);
    } finally {
      engine.dispose();
    }
  });

  test("withSession() refuses any session but the one the engine booted, before an edit is built", async () => {
    // #1097 verdict M1. Each variant keeps the booted tracks, so a track-set check passes all
    // three, and each would resolve an ID to an address that names another instance: with the
    // console reordered, `pre-comp` resolves to slot 0 (the EQ); with the slot renamed, nothing
    // resolves `pre-eq`; with `b`'s chain reversed, `ins-eq` resolves to insert 1 (the delay).
    // Red mutation: drop assertBootedSession from withSession() -> every variant is accepted.
    const engine = await createOfflineEngine(addressedStrip().toJson(), { asset, liveControls: { commandQueueRecords: 8 } });
    try {
      const controls = engine.liveControls();
      for (const variant of ["reordered console", "renamed slot", "another insert chain"]) {
        const builder = addressedStrip(undefined, variant);
        assert.notEqual(builder.toJson(), addressedStrip().toJson(), `${variant} is another session`);
        assert.throws(() => controls.withSession(builder), NOT_BOOTED, variant);
      }
      // The booted text is held byte for byte, so the same session booted from non-canonical text
      // cannot be matched without a parser, and is refused rather than trusted.
      const spaced = await createOfflineEngine(`${addressedStrip().toJson().trimEnd()} \n`, {
        asset, liveControls: { commandQueueRecords: 8 },
      });
      try {
        assert.throws(() => spaced.liveControls().withSession(addressedStrip()), NOT_BOOTED);
      } finally {
        spaced.dispose();
      }
    } finally {
      engine.dispose();
    }
  });

  test("loadSession() moves the document withSession() holds a builder to", async () => {
    // Red mutation: keep the first boot's bytes across loadSession() -> the new session's builder
    // is refused and the replaced one is accepted.
    const engine = await createOfflineEngine(addressedStrip(), { asset, liveControls: { commandQueueRecords: 8 } });
    try {
      const next = addressedStrip(undefined, "another insert chain");
      engine.loadSession(next.toJson(), { liveControls: { commandQueueRecords: 8 } });
      const controls = engine.liveControls();
      assert.throws(() => controls.withSession(addressedStrip()), NOT_BOOTED);
      const edit = controls.withSession(next).edit.track("b").insert("ins-eq", "miso.parametric-eq").bypass(true);
      assert.deepEqual([edit.rack, edit.effectIndex], [RACK.inserts, 1]);
    } finally {
      engine.dispose();
    }
  });

  test("withSession() on live controls built without the booted document refuses rather than trusts", () => {
    // Red mutation: skip the check when the booted document is absent -> any builder is accepted.
    const controls = new EngineLiveControls(
      { tracks: ["a", "b"], sources: [], metersAttached: false, submixes: [] },
      () => assert.fail("nothing is submitted"),
    );
    assert.throws(() => controls.withSession(addressedStrip()), /constructed without it/);
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

/** Parameters that make each catalog effect audible on `feed()`'s noise, so bypass is heard. */
const AUDIBLE = Object.freeze({
  "miso.compressor": { threshold: -40, ratio: 8 },
  "miso.delay": { "delay time": 2, mix: 0.5 },
  "miso.gate-expander": { threshold: -3, ratio: 20, hold: 0, hysteresis: 0, release: 5 },
  "miso.multiband-compressor": { low_threshold: -40, low_ratio: 8, high_threshold: -40, high_ratio: 8 },
  "miso.parametric-eq": { "band-1-enabled": true, "band-1-gain": 9, "band-1-frequency": 1_000 },
  "miso.soft-clip": { drive: 24 },
  "miso.transient-shaper": { "attack amount": 1, "sustain amount": -1 },
  "miso.true-peak-limiter": { ceiling: -12 },
});

/** One track whose one insert, `fx`, is `effectId`, audible, with its session bypass `bypassed`. */
function oneInsert(effectId, bypassed) {
  return session({ id: "console.prepared-bypass", sampleRateHz: 48_000, revision: 1 })
    .source("s", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .track("t", { source: "s", inserts: [effect(effectId, AUDIBLE[effectId], { slotId: "fx", bypass: bypassed })] })
    .output("out")
    .route({
      id: "main",
      source: { kind: "track", trackId: "t", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
}

/** The refusal `bypass(false)` makes for a session-bypassed effect that keeps its prepared bypass. */
const KEEPS_PREPARED_BYPASS = (error) => error instanceof MisoUsageError
  && /keeps its prepared bypass/.test(error.message);

describe("issue #1097 -- a live lift the engine would acknowledge and ignore is refused before it is sent", () => {
  test("a session-bypassed delay or multiband refuses bypass(false), by ID, by index and through withSession()", async () => {
    // #1097 verdict M2: the engine admits the lift, acks `ok`, and renders nothing different, so an
    // ack would precede a no-op. Red mutation: drop the prepared-bypass refusal in
    // EffectEdits.bypass() -> every `throws` below goes red.
    const built = session({ id: "console.lift", sampleRateHz: 48_000, revision: 1 })
      .source("s", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
      .track("t", {
        source: "s",
        inserts: [
          effect("miso.delay", {}, { slotId: "echo", bypass: true }),
          effect("miso.multiband-compressor", {}, { slotId: "mb", bypass: true }),
          effect("miso.parametric-eq", {}, { slotId: "ins-eq", bypass: true }),
          effect("miso.delay", {}, { slotId: "echo-live" }),
        ],
      })
      .output("out")
      .route({
        id: "main",
        source: { kind: "track", trackId: "t", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
    for (const [how, boot, bind] of [
      ["booted from the builder", built, (controls) => controls],
      ["booted from text, then withSession()", built.toJson(), (controls) => controls.withSession(built)],
    ]) {
      const engine = await createOfflineEngine(boot, { asset, liveControls: { commandQueueRecords: 8 } });
      try {
        const track = bind(engine.liveControls()).edit.track("t");
        assert.throws(() => track.insert("echo", "miso.delay").bypass(false), KEEPS_PREPARED_BYPASS, how);
        assert.throws(() => track.insert("mb", "miso.multiband-compressor").bypass(false), KEEPS_PREPARED_BYPASS, how);
        assert.throws(() => track.insert(0, "miso.delay").bypass(false), KEEPS_PREPARED_BYPASS, how);
        assert.throws(() => track.effect("inserts", 1, "miso.multiband-compressor").bypass(false), KEEPS_PREPARED_BYPASS, how);
        // The instance at the address governs, not the effect the caller claims for it.
        assert.throws(() => track.effect("inserts", 0, "miso.parametric-eq").bypass(false), KEEPS_PREPARED_BYPASS, how);
        // Setting a bypass, lifting one on any other effect, and lifting a delay the session did not
        // bypass all still go through.
        assert.equal(track.insert("echo", "miso.delay").bypass(true).kind, "effectBypass", how);
        assert.equal(track.insert("ins-eq", "miso.parametric-eq").bypass(false).kind, "effectBypass", how);
        assert.equal(track.insert("echo-live", "miso.delay").bypass(false).kind, "effectBypass", how);
      } finally {
        engine.dispose();
      }
    }
  });

  test("every catalog effect's live lift does what the SDK's PREPARED_BYPASS_EFFECTS says the engine does", async () => {
    // The metadata does not publish which effects keep a prepared bypass, so the SDK holds a copy;
    // this holds it to the engine. For each effect, authored bypassed: the engine's own answer to a
    // raw lift (a text boot, addressed by index, so the SDK cannot refuse it) must leave the
    // bypassed render exactly on the listed effects, and on every other effect the SDK must send
    // the lift and it must render the authored-unbypassed session bit for bit.
    // Red mutation: drop the multiband from PREPARED_BYPASS_EFFECTS, or add the soft-clip -> the
    // SDK's answer and the engine's disagree on that effect.
    assert.deepEqual(Object.keys(AUDIBLE).sort(), CATALOG.effects.map((row) => row.id).sort());
    for (const { id } of CATALOG.effects) {
      const keeps = PREPARED_BYPASS_EFFECTS.includes(id);
      const bypassed = await renderBlocks(oneInsert(id, true));
      const enabled = await renderBlocks(oneInsert(id, false));
      assert.notDeepEqual(bypassed, enabled, `${id}: the fixture must hear the bypass`);
      const raw = await renderBlocks(
        oneInsert(id, true).toJson(),
        (controls) => controls.edit.track("t").effect("inserts", 0, id).bypass(false),
      );
      assert.deepEqual(raw, keeps ? bypassed : enabled, `${id}: the engine's live lift`);

      const engine = await createOfflineEngine(oneInsert(id, true), { asset, liveControls: { commandQueueRecords: 8 } });
      try {
        const lift = () => engine.liveControls().edit.track("t").insert("fx", id).bypass(false);
        if (keeps) assert.throws(lift, KEEPS_PREPARED_BYPASS, id);
        else assert.doesNotThrow(lift, id);
      } finally {
        engine.dispose();
      }
      if (!keeps) {
        const lifted = await renderBlocks(
          oneInsert(id, true),
          (controls) => controls.edit.track("t").insert("fx", id).bypass(false),
        );
        assert.deepEqual(lifted, enabled, `${id}: the SDK's live lift`);
      }
    }
  });
});

/**
 * Issue #1205: submix strips and bus taps, authored through the SDK and held to the engine.
 *
 * `busSession()` is gate 1's session: console slots in both sections, two tracks, a `drums` bus
 * with every strip key off its default (a matrix, a linked glue compressor insert), a `verb` bus
 * with a `miso.delay` insert and the default pan, routes from a track's `pre_fader` tap and a bus's
 * `post_fader` tap, and a compressor on `vox` keyed from the drum bus's `pre_fader` tap.
 */
function busSession() {
  const duck = effect("miso.compressor", { threshold: -24, ratio: 4 }, {
    slotId: "duck",
    linkMode: "maximum",
    sidechain: { source: { kind: "submix", submixId: "drums", tap: "pre_fader" }, portId: "sidechain-in" },
  });
  const bypassedEntries = [{ slot: "eq", bypass: true }, { slot: "glue", bypass: true }];
  return session({ id: "console.buses", sampleRateHz: 48_000, revision: 2 })
    .source("kit", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
    .source("voice", { channels: 1, bitDepth: 24, frames: 48_000, content: CONTENT })
    .console({
      preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }],
      postInsert: [{ slot: "glue", effectId: "miso.compressor", linkMode: "dual_mono" }],
    })
    .output("main")
    .submix("drums", {
      builtins: { left: { trimDb: -2, hpfHz: 30 }, right: { trimDb: -2.5, hpfHz: 30, polarityInvert: true } },
      console: [
        { slot: "eq", parameters: { "band-1-enabled": true, "band-1-gain": 3, "band-1-frequency": 90 } },
        { slot: "glue", bypass: true },
      ],
      inserts: [effect("miso.compressor", { threshold: -14, ratio: 2 }, { slotId: "bus-glue", linkMode: "average" })],
      fader: { leftDb: -1.5, rightDb: -1.5, rightMute: false },
      pan: { matrix: { ll: 0.9, lr: 0.1, rl: 0.1, rr: 0.9 }, smoothingSamples: 32 },
    })
    .submix("verb", {
      console: bypassedEntries,
      inserts: [effect("miso.delay", { "delay time": 80 }, { slotId: "space" })],
    })
    .track("kick", { source: "kit", console: [{ slot: "eq" }, { slot: "glue" }] })
    .track("vox", { source: "voice", console: bypassedEntries, inserts: [duck] })
    .route({
      id: "kick-drums",
      source: { kind: "track", trackId: "kick", tap: "pre_fader" },
      destination: { kind: "submix_input", submixId: "drums" },
    })
    .route({
      id: "drums-main",
      source: { kind: "submix", submixId: "drums", tap: "post_fader" },
      destination: { kind: "output_input", outputId: "main" },
    })
    .route({
      id: "vox-verb",
      source: { kind: "track", trackId: "vox", tap: "post_fader" },
      destination: { kind: "submix_input", submixId: "verb" },
      gainDb: -6,
    })
    .route({
      id: "verb-main",
      source: { kind: "submix", submixId: "verb", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main" },
    })
    .route({
      id: "vox-main",
      source: { kind: "track", trackId: "vox", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main" },
    });
}

/** Render `blocks` blocks of `document` from finite source PCM that holds no `-0.0` sample. */
async function renderFrom(document, blocks = 8) {
  const engine = await createOfflineEngine(document, { asset });
  try {
    const shape = engine.shape();
    const output = [];
    for (let block = 0; block < blocks; block += 1) {
      shape.sources.forEach((source, sourceIndex) => {
        const planes = Array.from({ length: source.channels }, (_unused, channel) =>
          ramp(shape.quantumFrames, 7 + sourceIndex * 16 + channel + block * 1024));
        for (const plane of planes) {
          assert.ok(plane.every((sample) => Number.isFinite(sample) && !Object.is(sample, -0)));
        }
        assert.equal(engine.submitSource({
          sourceId: source.id, generation: 1n, startFrame: BigInt(block * shape.quantumFrames), planes,
          endOfRegion: false,
        }).ok, true);
      });
      const rendered = engine.render();
      output.push([...rendered.left], [...rendered.right]);
    }
    return output;
  } finally {
    engine.dispose();
  }
}

describe("issue #1205 -- submix strips and bus taps through the SDK", () => {
  test("a bus session is the engine's canonical JSON, byte for byte, and boots and renders headless", async () => {
    // Gates 1 and 4. Red mutations: write a submix's `console` after `inserts`, or its `tap`
    // before `submix_id`, in `session-json.ts` -> the engine's re-serialization differs; drop the
    // tap from `normalizeRouteSource`'s submix branch -> the engine refuses the document.
    const built = busSession();
    const text = built.toJson();
    assert.equal(await engineCanonical("bus-session", text), text);
    // The session is the shape it is named for, so the round trip above covers each path.
    const model = built.toJSON();
    const drums = model.submixes.find((row) => row.id === "drums");
    const verb = model.submixes.find((row) => row.id === "verb");
    assert.deepEqual(Object.keys(drums), ["id", "builtins", "console", "inserts", "fader", "matrix"]);
    assert.deepEqual(Object.keys(verb), ["id", "builtins", "console", "inserts", "fader", "pan"]);
    assert.equal(drums.builtins.right.polarity_invert, true);
    assert.deepEqual(verb.inserts.effects.map((row) => row.identity.effect_id), ["miso.delay"]);
    assert.deepEqual(
      model.tracks.find((row) => row.id === "vox").inserts.effects[0].sidechain.source,
      { kind: "submix", submix_id: "drums", tap: "pre_fader" },
    );
    assert.deepEqual(
      Object.fromEntries(model.routes.map((row) => [row.id, [row.source.kind, row.source.tap]])),
      {
        "drums-main": ["submix", "post_fader"],
        "kick-drums": ["track", "pre_fader"],
        "verb-main": ["submix", "post_pan"],
        "vox-main": ["track", "post_pan"],
        "vox-verb": ["track", "post_fader"],
      },
    );

    const output = await renderFrom(text, 1);
    assert.equal(output[0].length, 128);
    assert.ok(output.flat().every(Number.isFinite));
    assert.ok(output.flat().some((sample) => sample !== 0), "the buses must carry signal to the output");
  });

  test("a spec-less submix is the transparent strip: every slot bypassed, and it renders as no bus", async () => {
    // Gate 3. Every console slot declares zero latency (one parametric EQ), so a transparent bus
    // must render bit-identically to the track routed straight to the output. Red mutations: write
    // a spec-less submix's entries with `bypass: false`, give it the default `pan` instead of the
    // identity matrix, or a fader or trim off unity -> the document or the render differs.
    const base = () => session({ id: "console.transparent", sampleRateHz: 48_000, revision: 1 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
      .console({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }] })
      .track("t", {
        source: "stem",
        console: [{ slot: "eq", parameters: { "band-1-enabled": true, "band-1-gain": 6, "band-1-frequency": 900 } }],
        fader: { leftDb: -3, rightDb: -4.5 },
        pan: { left: -0.4, right: 0.7 },
      })
      .output("out");
    const viaBus = base()
      .submix("bus")
      .route({
        id: "t-bus",
        source: { kind: "track", trackId: "t", tap: "post_pan" },
        destination: { kind: "submix_input", submixId: "bus" },
      })
      .route({
        id: "bus-out",
        source: { kind: "submix", submixId: "bus", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
    const direct = base().route({
      id: "t-out",
      source: { kind: "track", trackId: "t", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "out" },
    });
    assert.deepEqual(viaBus.toJSON().submixes[0].console, [{ slot: "eq", bypass: true, params: [] }]);
    const text = viaBus.toJson();
    assert.equal(await engineCanonical("transparent-bus", text), text);
    const bus = await renderFrom(viaBus);
    const straight = await renderFrom(direct);
    assert.ok(straight.flat().some((sample) => sample !== 0), "the fixture must carry signal");
    assert.deepEqual(bus, straight);
  });
});

describe("issue #1216 -- a muted send through the SDK", () => {
  test("a session with one muted send is the engine's canonical JSON, byte for byte", async () => {
    // Gate 7. Red mutations: write `mute` before `gain_db` in `session-json.ts`'s route key
    // order, or drop it from `normalizeSession`'s route record -> the engine re-serializes the
    // document differently, or refuses it (`schema.missing_field`) and prints nothing.
    const built = session({ id: "console.muted-send", sampleRateHz: 48_000, revision: 1 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
      .track("t", { source: "stem" })
      .submix("verb")
      .output("out")
      .route({
        id: "t-out",
        source: { kind: "track", trackId: "t", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      })
      .route({
        id: "t-verb",
        source: { kind: "track", trackId: "t", tap: "post_fader" },
        destination: { kind: "submix_input", submixId: "verb" },
        gainDb: -6,
        mute: true,
      })
      .route({
        id: "verb-out",
        source: { kind: "submix", submixId: "verb", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
    const text = built.toJson();
    assert.equal(await engineCanonical("muted-send", text), text);
    assert.deepEqual(
      built.toJSON().routes.map((row) => [row.id, row.mute]),
      [["t-out", false], ["t-verb", true], ["verb-out", false]],
    );
  });
});

describe("issue #1218 -- a send that follows its source's mute, through the SDK", () => {
  test("follow sends from a muted track and a muted bus are the engine's canonical JSON, byte for byte", async () => {
    // Gate 7. Red mutations: write `follows_mute` before `mute` in `session-json.ts`'s route key
    // order, drop it from `normalizeSession`'s route record, or default it to `true` on a route
    // into the output -> the engine re-serializes the document differently, or refuses it
    // (`schema.missing_field`, `schema.invalid_enum`) and prints nothing.
    const built = session({ id: "console.follow-send", sampleRateHz: 48_000, revision: 1 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 48_000, content: CONTENT })
      .track("t", { source: "stem", fader: { leftMute: true } })
      .submix("verb", { fader: { rightMute: true } })
      .submix("echo")
      .output("out")
      .route({
        id: "echo-out",
        source: { kind: "submix", submixId: "echo", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      })
      .route({
        id: "t-echo",
        source: { kind: "track", trackId: "t", tap: "pre_fader" },
        destination: { kind: "submix_input", submixId: "echo" },
        followsMute: false,
      })
      .route({
        id: "t-out",
        source: { kind: "track", trackId: "t", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      })
      .route({
        id: "t-verb",
        source: { kind: "track", trackId: "t", tap: "pre_fader" },
        destination: { kind: "submix_input", submixId: "verb" },
        gainDb: -6,
      })
      .route({
        id: "verb-echo",
        source: { kind: "submix", submixId: "verb", tap: "pre_fader" },
        destination: { kind: "submix_input", submixId: "echo" },
      })
      .route({
        id: "verb-out",
        source: { kind: "submix", submixId: "verb", tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
    const text = built.toJson();
    assert.equal(await engineCanonical("follow-send", text), text);
    assert.deepEqual(
      built.toJSON().routes.map((row) => [row.id, row.follows_mute]),
      [
        ["echo-out", false],
        ["t-echo", false],
        ["t-out", false],
        ["t-verb", true],
        ["verb-echo", true],
        ["verb-out", false],
      ],
    );
  });
});
