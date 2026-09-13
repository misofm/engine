/** Issue #777: selected resident observation reads keep owner data and lifecycle boundaries. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { decodeObservationRows, enrichObservationMap } from "../src/core/observation.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

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
      dynamic: [
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

function selection(effectSlotId, channels = "both") {
  return { trackId: "t", rack: "dynamic", effectSlotId, tapId: 1, channels };
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
  test("keeps wide window clocks exact and admits more owners than one read batch", () => {
    const rawBindings = Array.from({ length: 257 }, (_unused, index) => ({
      trackIndex: 0,
      rack: 1,
      effectIndex: index,
      effectSlotId: index === 0 ? "comp" : `effect-${index}`,
      nativeEffectId: "miso.compressor",
      tapIds: [1],
    }));
    const map = enrichObservationMap(["t"], rawBindings);
    assert.equal(map.bindings.length, 257);

    const firstSample = 9_007_199_254_740_993n;
    const row = {
      trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, channels: 3,
      status: 3, sampleRateHz: 48_000, firstSample, endSample: firstSample + 128n,
      sequence: 9_007_199_254_740_995n, blocks: 1,
      leftPresent: 1, rightPresent: 1, left: 4.5, right: -2.25,
    };
    const [decoded] = decodeObservationRows(
      map,
      ["t"],
      [selection("comp")],
      [{ trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, channels: 3 }],
      [row],
      48_000,
    );
    assert.ok(decoded.window);
    assert.equal(decoded.window.firstSample, firstSample);
    assert.equal(decoded.window.endSample, firstSample + 128n);
    assert.equal(decoded.window.sequence, 9_007_199_254_740_995n);
  });

  test("headless reads two effects, preserves exact windows, and suppresses a re-arm", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      console: { commandQueueRecords: 64, observationTaps: 4 },
    });
    try {
      const comp = selection("comp");
      const gate = selection("gate", "left");
      assert.equal(engine.readObservations([comp])[0].status, "unarmed");

      const console = engine.console();
      const armed = await console.submit(
        console.edit.track("t").effect("dynamic", 0, "miso.compressor")
          .observe("Gain Reduction", true, 2),
        console.edit.track("t").effect("dynamic", 1, "miso.gate-expander")
          .observe("Gain Reduction", true, 2),
      );
      assert.equal(armed.ok, true);
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

      // The prior resident cell survives a re-arm, but is historical for that new arm.
      await console.submit(
        console.edit.track("t").effect("dynamic", 0, "miso.compressor")
          .observe("Gain Reduction", false, 2),
        console.edit.track("t").effect("dynamic", 0, "miso.compressor")
          .observe("Gain Reduction", true, 2),
      );
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
