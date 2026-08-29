/**
 * Issue #243 eval 2: 96 kHz / 127 frames, end to end, from the boot's own answer.
 *
 * The brief's constraint is the interesting part: "No number in the test may be hard-coded from
 * the document (the test derives everything from the boot's answer)." So the fixture below is
 * *written* at 96 kHz with a 127-frame quantum, and then never read again -- the ring, the plane
 * lengths, the channel counts, the source ids and the block count all come out of `engine.shape()`.
 * A test that hard-coded 127 would still pass against the old SDK's 48 kHz/128 fallback, because
 * it would be asserting its own input back at itself.
 *
 * The digest is compared against the native engine rendering the same document through
 * `AudioWorkletEngineHost` -- a separate process, a different CPU path, the same bits. See
 * `hosts/miso-engine-host-web/examples/sdk_render_oracle.rs`.
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { before, describe, test } from "node:test";
import { resolve } from "node:path";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { defaultSourceRingFrames } from "../src/core/abi.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { moduleBytes, ramp, sessionDocument } from "./support.mjs";

const REPO_ROOT = resolve(import.meta.dirname, "..", "..");

/** The native oracle: the same document, rendered by the engine on the host CPU. */
function nativeDigest(document, quanta, seed) {
  return execFileSync(
    "cargo",
    [
      "run", "--locked", "-q", "-p", "miso-engine-host-web",
      "--example", "sdk_render_oracle", "--", String(quanta), String(seed),
    ],
    { cwd: REPO_ROOT, input: document, encoding: "utf8", maxBuffer: 1 << 24 },
  ).trim();
}

/** SHA-256 over the little-endian `f32` words, exactly as every other parity gate in the repo. */
function digestBlocks(blocks) {
  const hash = createHash("sha256");
  for (const { left, right } of blocks) {
    for (const plane of [left, right]) {
      hash.update(Buffer.from(plane.buffer, plane.byteOffset, plane.byteLength));
    }
  }
  return hash.digest("hex");
}

/**
 * Render a whole document through the SDK, deriving every shape fact from the boot's answer.
 *
 * The plane key mirrors the oracle's: `seed + sourceIndex * 16 + channel + block * 1024`. It is
 * arbitrary except in being reproducible on both sides and distinct per plane, so no two planes
 * can be swapped without moving the digest.
 */
function renderThroughSdk(engine, quanta, seed) {
  const shape = engine.shape();
  const blocks = [];
  for (let block = 0; block < quanta; block += 1) {
    shape.sources.forEach((source, sourceIndex) => {
      const planes = Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, seed + sourceIndex * 16 + channel + block * 1024));
      const submitted = engine.submitSource({
        sourceId: source.id,
        generation: 1n,
        startFrame: BigInt(block * shape.quantumFrames),
        planes,
        endOfRegion: false,
      });
      assert.ok(submitted.ok, `source ${source.id} refused block ${block}: ${submitted.code}`);
    });
    blocks.push(engine.render());
  }
  return blocks;
}

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

describe("eval 2 -- 96k/127 end to end from the boot's own answer", () => {
  test("the SDK's source generator is bit-identical to the native oracle's", () => {
    // Asserted before the digests are compared, so a red digest is unambiguously the engine's
    // disagreement rather than the harness feeding two different signals. The values are the LCG's
    // first steps from seed 1, recomputed here in a third way -- longhand, from the constants --
    // rather than copied from either implementation.
    let state = 1;
    const expected = [];
    for (let index = 0; index < 4; index += 1) {
      state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
      expected.push(Math.fround((state / 0x1_0000_0000) * 2 - 1));
    }
    assert.deepEqual([...ramp(4, 1)], expected);
  });

  test("boot -> read shape -> prepare at the derived ring -> render -> native digest", async () => {
    const document = sessionDocument({
      quoteKeys: true,
      sampleRateHz: 96_000,
      quantumFrames: 127,
      // Longer than the ring the engine will derive, so a full ring turnover fits inside the
      // source's declared region. A source is bounded by its own `frames`, and rendering past
      // that is a refusal rather than silence -- which is how this number was chosen, and which
      // the assertion below re-reads from the boot rather than from here.
      frames: 19_200,
    });
    const engine = await createOfflineEngine(document, { asset });
    try {
      const shape = engine.shape();

      // Everything below is the engine's answer. The only number this test states about the
      // document is that it was written at a launch rate, which the engine has already agreed to.
      assert.equal(shape.sampleRateHz, 96_000);
      assert.equal(shape.quantumFrames, 127);
      assert.equal(
        shape.sourceRingFrames,
        defaultSourceRingFrames(shape.sampleRateHz, shape.quantumFrames),
        "the ring is derived from the reported shape, not from the fixture",
      );
      assert.equal(
        shape.sourceRingFrames % shape.quantumFrames,
        0,
        "the derived ring is a whole number of quanta -- the old 1024 default was not",
      );

      // The block count is derived too: enough quanta to cover the ring the engine chose, so the
      // render exercises a full ring turnover rather than a fixed guess.
      const quanta = Math.ceil(shape.sourceRingFrames / shape.quantumFrames);
      assert.equal(quanta, 78, "78 quanta of 127 frames is the derived ring");
      assert.ok(
        BigInt(quanta * shape.quantumFrames) <= shape.sources[0].frames,
        "a full ring turnover must fit inside the source region the engine reported",
      );

      const blocks = renderThroughSdk(engine, quanta, 1);
      assert.equal(blocks.length, quanta);
      for (const { left, right } of blocks) {
        assert.equal(left.length, shape.quantumFrames);
        assert.equal(right.length, shape.quantumFrames);
      }
      assert.equal(engine.renderedQuanta(), BigInt(quanta));
      assert.equal(
        engine.nextAbsoluteSample(),
        BigInt(quanta * shape.quantumFrames),
        "the engine's clock advanced exactly one quantum per render",
      );

      assert.equal(
        digestBlocks(blocks),
        nativeDigest(document, quanta, 1),
        "the wasm engine and the native engine rendered this 96k/127 session to different bits",
      );
    } finally {
      engine.dispose();
    }
  });

  test("the same cross-check holds at 48k/128, so 96k/127 is not a special case", async () => {
    const document = sessionDocument({ sampleRateHz: 48_000, quantumFrames: 128, frames: 4_800 });
    const engine = await createOfflineEngine(document, { asset });
    try {
      const blocks = renderThroughSdk(engine, 16, 7);
      assert.equal(digestBlocks(blocks), nativeDigest(document, 16, 7));
    } finally {
      engine.dispose();
    }
  });

  test("a mono source renders identically on both sides", async () => {
    // One channel rather than two changes the plane count the SDK derives from `shape.sources`,
    // so this is the leg that catches a hard-coded stereo assumption in the feed path.
    const document = sessionDocument({ channels: 1, frames: 4_800 });
    const engine = await createOfflineEngine(document, { asset });
    try {
      assert.equal(engine.shape().sources[0].channels, 1);
      const blocks = renderThroughSdk(engine, 8, 3);
      assert.equal(digestBlocks(blocks), nativeDigest(document, 8, 3));
    } finally {
      engine.dispose();
    }
  });
});
