import assert from "node:assert/strict";
import { test } from "node:test";

import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

const WINDOW_FRAMES = 2_048;
const CHANNELS = "both";

function eqParameters() {
  const definition = CATALOG.effects.find((row) => row.id === "miso.parametric-eq");
  assert.ok(definition, "the generated catalog must contain parametric EQ");
  return ["band-1-enabled", "band-1-kind", "band-1-frequency", "band-1-gain", "band-1-q"]
    .map((name) => {
      const parameter = definition.parameters.find((row) => row.name === name);
      assert.ok(parameter, `the generated EQ catalog must contain ${name}`);
      return {
        id: parameter.id,
        unit: parameter.unitName,
        value: name === "band-1-enabled" ? 1
          : name === "band-1-kind" ? 1
            : name === "band-1-frequency" ? 1_000
              : name === "band-1-gain" ? 6
                : parameter.default,
        channel: "both",
      };
    });
}

function spectrumDocument() {
  return sessionDocument({
    frames: 4_800,
    effects: {
      simd1: [effectEntry("eq", "miso.parametric-eq", eqParameters())],
      dynamic: [],
      simd2: [],
    },
  });
}

function queryFor(target, channels = CHANNELS) {
  return {
    target,
    channels,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  };
}

function feedAndRender(engine, block) {
  const shape = engine.shape();
  const source = shape.sources[0];
  assert.ok(source, "the spectrum fixture must have one source");
  const acknowledgement = engine.submitSource({
    sourceId: source.id,
    generation: 1n,
    startFrame: BigInt(block * shape.quantumFrames),
    planes: [
      ramp(shape.quantumFrames, 17 + block),
      ramp(shape.quantumFrames, 71 + block),
    ],
    endOfRegion: false,
  });
  assert.equal(acknowledgement.ok, true, `source block ${block} was refused`);
  return engine.render(shape.quantumFrames);
}

function assertSpectrumResult(result, query, expectedSample) {
  assert.deepEqual(result.target, query.target);
  assert.equal(result.channels, query.channels);
  assert.equal(result.sampleRateHz, 48_000);
  assert.equal(result.windowFrames, WINDOW_FRAMES);
  assert.equal(result.binCount, WINDOW_FRAMES / 2 + 1);
  assert.equal(result.capturedSample, BigInt(expectedSample));
  assert.equal(result.endSample, BigInt(expectedSample + WINDOW_FRAMES));
  assert.ok(result.snapshotToken > 0n);
  assert.equal(result.graphSourceUnderrun, false);
  assert.ok(result.resultBytes > 0n);
  assert.ok(Number.isFinite(result.floorDb));
  assert.ok(result.frequenciesHz instanceof Float32Array);
  assert.equal(result.frequenciesHz.length, result.binCount);
  assert.ok(result.frequenciesHz.every(Number.isFinite));
  if (query.channels === "both" || query.channels === "left") {
    assert.ok(result.leftDb instanceof Float32Array);
    assert.equal(result.leftDb.length, result.binCount);
    assert.ok(result.leftDb.every(Number.isFinite));
  } else {
    assert.equal(result.leftDb, undefined);
  }
  if (query.channels === "both" || query.channels === "right") {
    assert.ok(result.rightDb instanceof Float32Array);
    assert.equal(result.rightDb.length, result.binCount);
    assert.ok(result.rightDb.every(Number.isFinite));
  } else {
    assert.equal(result.rightDb, undefined);
  }
}

async function makeEngine(asset, query) {
  return createOfflineEngine(spectrumDocument(), {
    asset,
    spectrum: query,
  });
}

test("candidate Wasm spectrum query captures all graph targets through explicit renders", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const queries = [
    queryFor({ kind: "trackPostInputBuiltins", trackId: "t" }),
    queryFor({ kind: "trackPostMatrix", trackId: "t" }),
    queryFor({ kind: "output", outputId: "out" }),
  ];
  const results = [];
  for (const query of queries) {
    const engine = await makeEngine(asset, query);
    try {
      assert.throws(() => engine.readSpectrum(), /spectrum capture read|wrongState/,
        "a spectrum read before arming must be refused explicitly");
      const armed = engine.armSpectrum();
      assert.equal(armed.ok, true);
      const busy = engine.armSpectrum();
      assert.equal(busy.ok, false, "a second arm must be refused while the window is pending");
      assert.equal(busy.code, "backpressure");
      for (let block = 0; block < WINDOW_FRAMES / engine.shape().quantumFrames; block += 1) {
        feedAndRender(engine, block);
        if (block + 1 < WINDOW_FRAMES / engine.shape().quantumFrames) {
          assert.equal(engine.readSpectrum(), undefined,
            "the result must remain pending until the complete window is rendered");
        }
      }
      const first = engine.readSpectrum();
      assert.ok(first);
      assertSpectrumResult(first, query, 0);
      assert.notEqual(first.frequenciesHz, first.leftDb);
      assert.notEqual(first.leftDb, first.rightDb);
      const firstFrequencyValues = first.frequenciesHz.slice();
      const firstLeftValues = first.leftDb?.slice();
      const firstRightValues = first.rightDb?.slice();

      assert.equal(engine.armSpectrum().ok, true);
      for (let block = WINDOW_FRAMES / engine.shape().quantumFrames;
        block < 2 * WINDOW_FRAMES / engine.shape().quantumFrames;
        block += 1) {
        feedAndRender(engine, block);
      }
      const second = engine.readSpectrum();
      assert.ok(second);
      assertSpectrumResult(second, query, WINDOW_FRAMES);
      assert.notEqual(second.frequenciesHz, first.frequenciesHz);
      assert.notEqual(second.leftDb, first.leftDb);
      assert.deepEqual(first.frequenciesHz, firstFrequencyValues,
        "the first frequency axis must remain owned after a later query");
      assert.deepEqual(first.leftDb, firstLeftValues,
        "the first left spectrum must remain owned after a later query");
      assert.deepEqual(first.rightDb, firstRightValues,
        "the first right spectrum must remain owned after a later query");
      results.push({ query, first, second });
    } finally {
      engine.dispose();
    }
  }

  assert.deepEqual(results.map(({ first }) => first.target), queries.map(({ target }) => target));
  assert.ok(results[0].first.leftDb.some((value) => value > results[0].first.floorDb),
    "the captured source must produce a measurable spectrum");
  assert.notDeepEqual(results[0].first.leftDb, results[1].first.leftDb,
    "the post-input and post-matrix taps must observe the EQ in the prepared graph");
});

test("candidate Wasm spectrum arm, cancel, and dispose preserve lifecycle refusals", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "trackPostMatrix", trackId: "t" });
  const engine = await makeEngine(asset, query);
  assert.equal(engine.armSpectrum().ok, true);
  const cancelled = engine.cancelSpectrum();
  assert.equal(cancelled.ok, true);
  assert.equal(cancelled.code, "ok");
  assert.throws(() => engine.readSpectrum(), /spectrum capture read|wrongState/);
  assert.equal(engine.armSpectrum().ok, true, "a cancelled capture must be re-armable");
  engine.dispose();
  assert.equal(engine.armSpectrum().ok, false);
  assert.equal(engine.armSpectrum().code, "unsupported");
  assert.throws(() => engine.readSpectrum(), /no prepared spectrum boundary/);
  engine.dispose();
});

test("candidate Wasm spectrum honors a selected channel and explicit capture limit", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const query = queryFor({ kind: "trackPostMatrix", trackId: "t" }, "left");
  const engine = await makeEngine(asset, query);
  try {
    assert.equal(engine.armSpectrum().ok, true);
    for (let block = 0; block < WINDOW_FRAMES / engine.shape().quantumFrames; block += 1) {
      feedAndRender(engine, block);
    }
    const result = engine.readSpectrum();
    assert.ok(result);
    assertSpectrumResult(result, query, 0);
    assert.ok(result.leftDb);
    assert.equal(result.rightDb, undefined);
  } finally {
    engine.dispose();
  }
});
