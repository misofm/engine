/** Issue #783: one bounded managed resident-observation lifetime. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
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

describe("issue 783 -- managed resident observation subscriptions", () => {
  test("shares bindings, preserves owned rows, guards manual edits, and closes last owner", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      console: { commandQueueRecords: 64, observationTaps: 4 },
    });
    const compLeft = selection("comp", "left");
    const gateRight = selection("gate", "right");
    try {
      let notifications = 0;
      const first = await engine.subscribeObservations({
        selections: [compLeft, gateRight],
        windowBlocks: 2,
        onUpdate: () => { notifications += 1; },
      });
      assert.equal(first.configuration.windowBlocks, 2);
      assert.deepEqual(first.readLatest().map((row) => row.status), ["pending", "pending"]);
      const console = engine.console();
      await assert.rejects(
        () => console.submit(console.edit.track("t").effect("dynamic", 0, "miso.compressor")
          .observe("Gain Reduction", false, 2)),
        /conflict/,
      );

      feed(engine, 0); engine.render(); await first.pump();
      feed(engine, 1); engine.render(); await first.pump();
      const saved = structuredClone(first.readLatest());
      assert.ok(notifications >= 1);
      assert.deepEqual(saved.map((row) => [row.status, row.channels]), [
        ["ready", "left"], ["ready", "right"],
      ]);
      assert.ok(saved[0].window && saved[0].window.sequence > 0n);
      const second = await engine.subscribeObservations({
        selections: [selection("comp", "right"), selection("gate", "left")],
        windowBlocks: 2,
      });
      assert.equal(second.readLatest().length, 2);
      assert.equal(second.readLatest()[0].channels, "right");

      await assert.rejects(
        () => engine.subscribeObservations({ selections: [compLeft], windowBlocks: 3 }),
        /conflicts/,
      );
      await assert.rejects(
        () => first.update({ selections: [selection("missing")], windowBlocks: 2 }),
        /unavailable|unsupported/,
      );
      assert.equal(first.configuration.selections[0].effectSlotId, "comp");
      assert.equal(first.readLatest()[0].window.sequence, saved[0].window.sequence);

      await first.close();
      assert.notEqual(engine.readObservations([compLeft])[0].status, "unarmed");
      await second.close();
      assert.equal(engine.readObservations([compLeft])[0].status, "unarmed");
      await second.close();
    } finally {
      engine.dispose();
    }
  });

  test("invalidates a handle at headless replacement", async () => {
    const engine = await createOfflineEngine(observationDocument(), {
      asset,
      console: { commandQueueRecords: 64, observationTaps: 2 },
    });
    const sub = await engine.subscribeObservations({ selections: [selection("comp")], windowBlocks: 1 });
    try {
      assert.throws(() => engine.loadSession("{}"));
      assert.throws(() => sub.readLatest(), /stale|closed/);
      await assert.rejects(() => sub.pump(), /stale|closed/);
    } finally {
      engine.dispose();
    }
  });
});
