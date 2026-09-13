/** Issue #785: managed live-response sharing and captured-state suppression. */

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  if (process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX) asset = await MisoEngineAsset.load(await moduleBytes());
});

test("managed live responses share jobs, suppress unchanged captures, and own latest arrays", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const eq = CATALOG.effects.find((row) => row.id === "miso.parametric-eq");
  assert.ok(eq);
  const params = ["band-1-enabled", "band-1-kind", "band-1-frequency", "band-1-gain", "band-1-q"]
    .map((name) => {
      const row = eq.parameters.find((candidate) => candidate.name === name);
      assert.ok(row);
      return {
        id: row.id,
        unit: row.unitName,
        value: name === "band-1-enabled" ? 1 : name === "band-1-gain" ? 3 : row.default,
        channel: "both",
      };
    });
  const engine = await createOfflineEngine(sessionDocument({
    effects: { simd1: [effectEntry("eq", "miso.parametric-eq", params)] },
  }), { asset, console: { commandQueueRecords: 64 } });
  const request = {
    trackId: "t",
    grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
    channels: "both",
    cadenceMs: 1,
  };
  try {
    const first = await engine.subscribeTrackResponse(request);
    const second = await engine.subscribeTrackResponse(request);
    assert.equal(first.job, second.job);
    assert.equal(first.revision, 1n);
    const firstResult = first.readLatest();
    const secondResult = second.readLatest();
    assert.ok(firstResult && secondResult);
    const secondValue = secondResult.leftDb[0];
    firstResult.leftDb[0] = Number.NaN;
    assert.equal(secondResult.leftDb[0], secondValue);
    assert.equal(await first.pump(), undefined);

    const semanticConsole = engine.console();
    await semanticConsole.submit(
      semanticConsole.edit.track("t").effect("simd1", 0, "miso.parametric-eq")
        .parameter("band-1-gain", -3),
    );
    assert.equal(await first.pump(), undefined);
    const shape = engine.shape();
    for (const source of shape.sources) {
      engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: 0n,
        planes: Array.from({ length: source.channels }, () => new Float32Array(shape.quantumFrames)),
        endOfRegion: false,
      });
    }
    engine.render();
    const notification = await first.pump();
    assert.ok(notification);
    assert.equal(notification.revision, 2n);
    assert.equal(notification.skippedPublications, 0n);

    await first.close();
    assert.ok(second.readLatest());
    await second.close();
  } finally {
    if (engine.state() !== "disposed") engine.dispose();
  }
});
