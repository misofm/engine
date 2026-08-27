#!/usr/bin/env node
/** Focused Phase 1 core smoke: generated catalog access and ABI-driven command encoding. */

import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const sdkRoot = resolve(import.meta.dirname, "..");
const compiler = resolve(sdkRoot, "node_modules", ".bin", "tsc");

function compile(directory) {
  const result = spawnSync(compiler, [
    "--project", "tsconfig.json", "--noEmit", "false", "--rootDir", "src", "--outDir", directory,
  ], { cwd: sdkRoot, encoding: "utf8" });
  assert.equal(result.status, 0, `SDK core compilation failed:\n${result.stdout}${result.stderr}`);
}

async function withModule(run) {
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-core-"));
  try {
    compile(directory);
    await run(await import(pathToFileURL(resolve(directory, "index.js")).href));
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

async function check() {
  await withModule(async (sdk) => {
    const description = sdk.describe();
    assert.equal(description.schema, "miso.sdk.describe.v1");
    assert.equal(description.catalog, sdk.CATALOG);
    assert.equal(Object.isFrozen(description), true);
    assert.deepEqual(description.engine.sampleRates, [44_100, 48_000, 88_200, 96_000]);
    assert.equal(description.engine.quantumFrames, 128);
    assert.equal(description.engine.wasmBytes, 2_494_615);
    assert.equal(
      description.engine.assetHashes["miso-engine-v2-audio-worklet.simd128.wasm"],
      "99c08301577dc27799bee3c13fe74dfee87db36b0b54864d97c92935666368d6",
    );
    assert.equal(Object.isFrozen(description.engine.assetHashes), true);
    assert.equal(Object.isFrozen(sdk.COMMAND_REASON_NAMES), true);

    const batch = sdk.encodeCommandBatch([{
      kind: 5, rack: 1, channel: 2, trackIndex: 3, effectIndex: 4, parameterId: 5,
      smoothingSamples: 6, values: [0.25, 0, 0, 0],
    }]);
    assert.equal(batch.count, 1);
    assert.equal(batch.records.byteLength, sdk.ABI_LAYOUT.commandRecord.bytes);
    const view = new DataView(batch.records.buffer, batch.records.byteOffset, batch.records.byteLength);
    assert.equal(view.getUint8(0), 5);
    assert.equal(view.getUint32(4, true), 3);
    assert.equal(view.getUint32(8, true), 4);
    assert.equal(view.getUint32(12, true), 5);
    assert.equal(view.getUint32(16, true), 6);
    assert.equal(view.getFloat32(24, true), 0.25);
    for (const offset of [3, 20, 21, 22, 23, 40, 41, 42, 43, 44, 45, 46, 47]) {
      assert.equal(view.getUint8(offset), 0, `reserved byte ${offset}`);
    }
  });
}

async function selfTest() {
  await check();
  await withModule(async (sdk) => {
    assert.throws(
      () => sdk.encodeCommandBatch([{
        kind: 5, rack: 1, channel: 2, trackIndex: 0, effectIndex: 0, parameterId: 1,
        smoothingSamples: 0, values: [Number.NaN, 0, 0, 0],
      }]),
      (error) => error instanceof sdk.MisoCommandError && error.path === "$.commands[0].values[0]",
      "the deliberate non-finite command mutation must be rejected before encoding",
    );
    assert.throws(
      () => sdk.encodeCommandBatch([{
        kind: 5, rack: 1, channel: 2, trackIndex: 0, effectIndex: 0, parameterId: 1,
        smoothingSamples: 0, values: [Number.MAX_VALUE, 0, 0, 0],
      }]),
      (error) => error instanceof sdk.MisoCommandError && error.path === "$.commands[0].values[0]",
      "the deliberate finite-f64/out-of-f32-range mutation must be rejected before encoding",
    );
    for (const values of [[0], [0, 0, 0, 0, 1]]) {
      assert.throws(
        () => sdk.encodeCommandBatch([{
          kind: 5, rack: 1, channel: 2, trackIndex: 0, effectIndex: 0, parameterId: 1,
          smoothingSamples: 0, values,
        }]),
        (error) => error instanceof sdk.MisoCommandError && error.path === "$.commands[0].values",
        `the deliberate ${values.length}-value command mutation must be rejected before encoding`,
      );
    }
  });
}

if (process.argv.length === 2) {
  await check();
  console.log("SDK core catalog/command check passed");
} else if (process.argv.length === 3 && process.argv[2] === "--self-test") {
  await selfTest();
  console.log("SDK core self-test passed (non-finite and f32-overflow red mutations caught)");
} else {
  throw new Error("usage: node sdk/test/core.mjs [--self-test]");
}
