/** Issue #777: selected resident observation reads keep owner data and lifecycle boundaries. */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, insertTap, moduleBytes, observeTaps, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function observationDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  const gate = CATALOG.effects.find((row) => row.id === "miso.gate-expander");
  assert.ok(compressor && gate);
  return sessionDocument({
    effects: {
      inserts: [
        effectEntry("comp", compressor.id, compressor.parameters.map((row) => ({
          id: row.id, unit: row.unitName, value: row.default, channel: "both",
        }))),
        effectEntry("gate", gate.id, gate.parameters.map((row) => ({
          id: row.id, unit: row.unitName, value: row.default, channel: "both",
        }))),
      ],
    },
  });
}

function largeObservationDocument() {
  const document = JSON.parse(observationDocument());
  const trackTemplate = document.tracks[0];
  const routeTemplate = document.routes[0];
  const tracks = Array.from({ length: 257 }, (_unused, index) => {
    const track = structuredClone(trackTemplate);
    track.id = `t${index}`;
    track.inserts.effects = [structuredClone(trackTemplate.inserts.effects[0])];
    return track;
  });
  document.tracks = tracks;
  document.routes = tracks.map((track, index) => {
    const route = structuredClone(routeTemplate);
    route.id = `main-${index}`;
    route.source.track_id = track.id;
    return route;
  });
  return JSON.stringify(document);
}

function selection(effectSlotId, channels = "both") {
  return { trackId: "t", rack: "inserts", effectSlotId, tapId: 1, channels };
}

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    const planes = Array.from({ length: source.channels }, (_unused, channel) =>
      ramp(shape.quantumFrames, 17 + block * 11 + channel));
    assert.equal(engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes,
      endOfRegion: false,
    }).ok, true);
  }
}

describe("issue 777 -- selected resident observations", () => {
  // #1590 gate 3 (R2): the map from the core's control list (section headers with the effect
  // identity) and the generated catalog's taps equals the map the browser's map exports gave,
  // recorded from them before #1590 deleted them, in strip, section and tap content, by stable
  // IDs only. The control-list fixture adds console slots, a submix and a four-tap effect. Red if
  // the list and the catalog give another set of taps than the exports did.
  test("the observation map from the control list and the catalog equals the exports' map", async () => {
    const binding = ([trackId, rack, effectSlotId, nativeEffectId, tapIds]) =>
      ({ trackId, rack, effectSlotId, nativeEffectId, tapIds });
    const cases = [
      [
        readFileSync(new URL("../../hosts/host-web/qualification/observation-session.json", import.meta.url), "utf8"),
        [["track", "inserts", "comp", "miso.compressor", [1]]],
      ],
      [
        JSON.parse(readFileSync(new URL("../../hosts/host-web/tests/fixtures/control-list.json", import.meta.url), "utf8")).session,
        [
          ["t1", "console", "pre-comp", "miso.compressor", [1]],
          ["t1", "inserts", "comp", "miso.compressor", [1]],
          ["t2", "console", "pre-comp", "miso.compressor", [1]],
          ["t2", "inserts", "gate", "miso.gate-expander", [1]],
          ["t3", "console", "pre-comp", "miso.compressor", [1]],
          ["t3", "inserts", "multi", "miso.multiband-compressor", [1]],
          ["t3", "inserts", "limiter", "miso.true-peak-limiter", [1]],
          ["t3", "inserts", "deess", "miso.de-esser", [1]],
          ["t3", "inserts", "dyneq", "miso.dynamic-eq", [1, 2, 3, 4]],
          ["b", "console", "pre-comp", "miso.compressor", [1]],
        ],
      ],
    ];
    for (const [document, expected] of cases) {
      const engine = await createOfflineEngine(document, { asset });
      try {
        assert.deepEqual(engine.observationMap(), { bindings: expected.map(binding) });
      } finally {
        engine.dispose();
      }
    }
  });

  test("reads an unarmed owner through the actual Wasm map beyond one read batch", async () => {
    const engine = await createOfflineEngine(largeObservationDocument(), {
      asset,
      maximumMemoryBytes: 64n << 20n,
    });
    try {
      const map = engine.observationMap();
      assert.equal(map.bindings.length, 257);
      const [binding] = map.bindings;
      assert.ok(binding);
      const [row] = engine.readObservations([{
        trackId: binding.trackId,
        rack: binding.rack,
        effectSlotId: binding.effectSlotId,
        tapId: binding.tapIds[0],
        channels: "both",
      }]);
      assert.equal(row.status, "unarmed");
    } finally {
      engine.dispose();
    }
  });

  test("headless reads two effects, preserves exact windows, and suppresses a re-arm", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
    });
    try {
      const comp = selection("comp");
      const gate = selection("gate", "left");
      assert.equal(engine.readObservations([comp])[0].status, "unarmed");

      const armed = observeTaps(engine, [insertTap("t", "comp", true, 2), insertTap("t", "gate", true, 2)]);
      assert.equal(armed.status, "ok");
      assert.equal(engine.readObservations([comp, gate])[0].status, "pending");

      feed(engine, 0); engine.render();
      feed(engine, 1); engine.render();
      const first = engine.readObservations([comp, gate]);
      assert.deepEqual(first.map((row) => [row.status, row.channels]), [
        ["ready", "both"], ["ready", "left"],
      ]);
      assert.equal(first[0].descriptor.displayUnit, "dB");
      assert.ok(Number.isFinite(first[0].left) && Number.isFinite(first[0].right));
      assert.notEqual(first[0].left, first[0].right, "the two compressor lanes remain independent");
      assert.equal(first[1].right, undefined);
      assert.ok(first[0].window && first[1].window);
      assert.equal(first[0].window.blocks, 2);
      assert.equal(first[1].window.blocks, 2);
      const firstSnapshot = structuredClone(first);
      const repeated = engine.readObservations([comp, gate]);
      assert.deepEqual(repeated, first);

      // The prior resident cell survives a re-arm, but is historical for that new arm. The disarm of
      // the old window and the arm of a new one in one transaction is the core's one re-arm: it
      // restarts the window.
      const rearmed = observeTaps(engine, [
        insertTap("t", "comp", false, 2, 1, armed.applied.observeArmTags[0]),
        insertTap("t", "comp", true, 1),
      ]);
      assert.equal(rearmed.status, "ok");
      assert.equal(engine.readObservations([comp])[0].status, "pending");
      feed(engine, 2); engine.render();
      feed(engine, 3); engine.render();
      const fresh = engine.readObservations([comp])[0];
      assert.equal(fresh.status, "ready");
      assert.ok(fresh.window && first[0].window);
      assert.ok(fresh.window.firstSample > first[0].window.firstSample);
      assert.deepEqual(first, firstSnapshot, "returned values are owned after later renders");

      assert.throws(
        () => engine.readObservations([selection("missing")]),
        (error) => error instanceof MisoEngineError && error.code === "unsupported",
      );
      const map = engine.observationMap();
      assert.deepEqual(map.bindings.map((binding) => binding.effectSlotId), ["comp", "gate"]);

      assert.throws(
        () => engine.loadSession("{}"),
        MisoEngineError,
      );
      assert.throws(() => engine.observationMap(), /disposed/);
      assert.throws(() => engine.readObservations([comp]), /disposed/);
    } finally {
      engine.dispose();
    }
  });
});
