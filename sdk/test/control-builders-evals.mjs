/**
 * #1564B gate 3: sugar is `set`. The typed builders are generated from the core's control tables
 * (`src/generated/control-builders.ts`), and each one must stage exactly the `ControlFrameEdit`s
 * that `edit.set` stages for the same address and value. The eval walks the generated manifest and
 * every row of the tables, so a row without a builder, or a builder that names another row, lane or
 * ramp than its row does, turns it red without anyone writing a case for it.
 */

import assert from "node:assert/strict";
import { describe, test } from "node:test";

import { LiveControlEdits } from "../src/core/live-controls.ts";
import { CONTROLS } from "../src/generated/controls.ts";
import {
  CONTROL_BUILDERS,
  EFFECT_LINK_MODE_ROWS,
  EFFECT_PARAMETER_ROWS,
} from "../src/generated/control-builders.ts";

const MAP = { tracks: ["t"], sources: [], metersAttached: false, submixes: ["bus"] };
const edit = new LiveControlEdits(MAP);

const TABLES = {
  "strip/own": CONTROLS.stripOwn,
  "strip/builtins": CONTROLS.builtins,
  "route/own": CONTROLS.routeOwn,
  "vca/own": CONTROLS.vcaOwn,
  "session/own": CONTROLS.sessionOwn,
};

function derived(row) {
  const classes = row.class !== undefined ? [row.class] : [row.intoSubmix.class, row.intoOutput.class];
  return classes.every((rowClass) => rowClass === "derived");
}

/** The builder objects of an owner kind, one per kind of strip. */
function ownersOf(entry) {
  switch (entry.owner) {
    case "strip":
      return entry.only === "track"
        ? [["track", edit.track("t"), { kind: "strip", id: "t" }]]
        : [
          ["track", edit.track("t"), { kind: "strip", id: "t" }],
          ["submix", edit.submix("bus"), { kind: "strip", id: "bus" }],
        ];
    case "route":
      return [["route", edit.route("t-bus"), { kind: "route", id: "t-bus" }]];
    case "vca":
      return [["vca", edit.vca("drums"), { kind: "vca", id: "drums" }]];
    case "session":
      return [["session", edit.session(), { kind: "session" }]];
    default:
      throw new Error(`unknown owner ${entry.owner}`);
  }
}

/** The option bags a builder of this options shape takes, with the lane and ramp each one means. */
function optionCases(shape) {
  const ramps = shape === "lane" || shape === "smoothing" ? [undefined, 0, 480] : [undefined];
  const channels = shape === "lane" || shape === "channel" ? [undefined, "left", "right", "both"] : [undefined];
  const cases = [];
  for (const smoothingSamples of ramps) {
    for (const channel of channels) {
      cases.push({
        options: {
          ...(smoothingSamples === undefined ? {} : { smoothingSamples }),
          ...(channel === undefined ? {} : { channel }),
        },
        rampSamples: smoothingSamples,
        channel: channel ?? "both",
      });
    }
  }
  return cases;
}

/** What `set` stages for one row, with the ramp absent when none was given. */
function viaSet(owner, section, control, value, lane, rampSamples) {
  return edit.set(
    { owner, section: { kind: section }, control, lane },
    value,
    rampSamples === undefined ? {} : { rampSamples },
  ).edits;
}

function valuesOf(entry) {
  if (entry.valueKind === "boolean") return [[true, 1], [false, 0]];
  if (entry.valueKind === "enumeration") return entry.choices.map(([label, value]) => [label, value]);
  return [[0.25, 0.25]];
}

describe("#1564B gate 3 -- every generated builder stages what set stages", () => {
  test("every settable row of the control tables has a builder, and no builder names a missing row", () => {
    for (const [key, rows] of Object.entries(TABLES)) {
      const [owner, section] = key.split("/");
      const written = new Set(
        CONTROL_BUILDERS.filter((entry) => entry.owner === owner && entry.section === section)
          .flatMap((entry) => entry.rows),
      );
      const settable = rows.filter((row) => !derived(row)).map((row) => row.name);
      assert.deepEqual([...written].sort(), settable.sort(), `${key}: builders and settable rows differ`);
    }
    // A derived row has no builder: its value follows other controls and `set` refuses it.
    for (const [key, rows] of Object.entries(TABLES)) {
      const [owner, section] = key.split("/");
      for (const row of rows.filter(derived)) {
        assert.ok(
          !CONTROL_BUILDERS.some((entry) =>
            entry.owner === owner && entry.section === section && entry.rows.includes(row.name)),
          `${key}.${row.name} is derived and must have no builder`,
        );
      }
    }
  });

  test("a method name is unique among the builders of an owner", () => {
    const seen = new Set();
    for (const entry of CONTROL_BUILDERS) {
      const key = `${entry.owner}/${entry.only ?? "*"}/${entry.method}`;
      assert.ok(!seen.has(key), `${key} is generated twice`);
      seen.add(key);
    }
  });

  for (const entry of CONTROL_BUILDERS) {
    test(`${entry.owner} ${entry.method} (${entry.rows.join(", ")}) is set over its rows`, () => {
      let checked = 0;
      for (const [, builders, owner] of ownersOf(entry)) {
        assert.equal(typeof builders[entry.method], "function", `${entry.owner}.${entry.method}`);
        for (const { options, rampSamples, channel } of optionCases(entry.options)) {
          for (const [given, scalar] of valuesOf(entry)) {
            let staged;
            let expected;
            if (entry.shape === "row") {
              staged = builders[entry.method](given, ...(entry.options === "none" ? [] : [options]));
              expected = viaSet(owner, entry.section, entry.rows[0], scalar, channel, rampSamples);
            } else if (entry.shape === "lanePair") {
              staged = builders[entry.method](0.25, -0.5, options);
              expected = [
                ...viaSet(owner, entry.section, entry.rows[0], 0.25, "left", rampSamples),
                ...viaSet(owner, entry.section, entry.rows[0], -0.5, "right", rampSamples),
              ];
            } else {
              const values = Object.fromEntries(entry.fields.map((field, index) => [field, 0.1 * (index + 1)]));
              staged = builders[entry.method](values, ...(entry.options === "none" ? [] : [options]));
              expected = entry.rows.flatMap((row, index) =>
                viaSet(owner, entry.section, row, values[entry.fields[index]], channel, rampSamples));
            }
            assert.deepEqual(staged.edits, expected, `${entry.method} ${JSON.stringify(options)} ${given}`);
            assert.ok(Object.isFrozen(staged) && Object.isFrozen(staged.edits));
            checked += 1;
          }
        }
      }
      assert.ok(checked > 0);
    });
  }

  test("a track offers solo and a submix does not, and a strip offers every other strip builder", () => {
    const track = edit.track("t");
    const submix = edit.submix("bus");
    for (const entry of CONTROL_BUILDERS.filter((candidate) => candidate.owner === "strip")) {
      assert.equal(typeof track[entry.method], "function", entry.method);
      assert.equal(typeof submix[entry.method], entry.only === undefined ? "function" : "undefined", entry.method);
    }
    assert.equal(submix.solo, undefined);
    assert.equal(typeof track.solo, "function");
  });
});

describe("#1564B gate 3 -- every effect row", () => {
  const instances = [
    ["inserts", (strip, effectId) => strip.insert("i", effectId), { kind: "inserts", effect: "i" }],
    ["console", (strip, effectId) => strip.console("c", effectId), { kind: "console", slot: "c" }],
  ];

  function numeric(effectId, name) {
    const table = CONTROLS.effects[effectId];
    const rows = [table.bypass, ...table.parameters, ...(table.linkMode === null ? [] : [table.linkMode])];
    const row = rows.find((candidate) => candidate.name === name);
    assert.ok(row, `${effectId} has no control row ${name}`);
    return row.control;
  }

  test("every parameter, bypass and link mode row of every effect has a generated builder row", () => {
    for (const [effectId, table] of Object.entries(CONTROLS.effects)) {
      const named = Object.values(EFFECT_PARAMETER_ROWS[effectId]).map((row) => row.control).sort();
      assert.deepEqual(named, table.parameters.map((row) => row.name).sort(), `${effectId} parameters`);
      assert.equal(EFFECT_LINK_MODE_ROWS[effectId]?.control, table.linkMode?.name, `${effectId} link mode`);
    }
  });

  for (const [rack, instance, section] of instances) {
    test(`${rack}: parameter, bypass and link mode stage what set stages`, () => {
      const strip = edit.track("t");
      let checked = 0;
      for (const [effectId, rows] of Object.entries(EFFECT_PARAMETER_ROWS)) {
        const builders = instance(strip, effectId);
        const owner = { kind: "strip", id: "t" };
        for (const [name, row] of Object.entries(rows)) {
          const control = numeric(effectId, row.control);
          const values = row.kind === "boolean"
            ? [[true, 1], [false, 0]]
            : row.kind === "enumeration" ? row.choices : [[0.5, 0.5]];
          for (const [given, scalar] of values) {
            for (const [options, lane, rampSamples] of [
              [{}, "both", undefined],
              [{ channel: "left" }, "left", undefined],
              [{ channel: "right", smoothingSamples: 64 }, "right", 64],
              [{ smoothingSamples: 0 }, "both", 0],
            ]) {
              assert.deepEqual(
                builders.parameter(name, given, options).edits,
                edit.set({ owner, section, control, lane }, scalar, rampSamples === undefined ? {} : { rampSamples }).edits,
                `${effectId}.${name} ${JSON.stringify(options)}`,
              );
              assert.deepEqual(
                builders.parameter({ key: name, value: given, ...options }).edits,
                edit.set({ owner, section, control, lane }, scalar, rampSamples === undefined ? {} : { rampSamples }).edits,
              );
              checked += 1;
            }
          }
        }
        for (const [enabled, scalar] of [[true, 1], [false, 0]]) {
          assert.deepEqual(
            builders.bypass(enabled).edits,
            edit.set({ owner, section, control: numeric(effectId, "bypass"), lane: "both" }, scalar).edits,
            `${effectId} bypass`,
          );
        }
        const linkMode = EFFECT_LINK_MODE_ROWS[effectId];
        if (linkMode !== undefined) {
          for (const [label, scalar] of linkMode.choices) {
            assert.deepEqual(
              builders.linkMode(label).edits,
              edit.set({ owner, section, control: numeric(effectId, linkMode.control), lane: "both" }, scalar).edits,
              `${effectId} link mode ${label}`,
            );
          }
        }
      }
      assert.ok(checked > 100, `only ${checked} parameter edits were compared`);
    });
  }
});
