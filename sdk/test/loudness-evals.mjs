/**
 * #2115 S3: `subscribeLoudness()`, the managed handle on the session's output loudness meter,
 * against the shipped module (headless). The meter's DSP is `crates/loudness`'s and its command is
 * `crates/control-plane/tests/loudness_monitor.rs`'s; these evals hold the SDK's half: the frame
 * codec over real core replies, the core's refusals reaching the caller typed, and close.
 */

import assert from "node:assert/strict";
import process from "node:process";
import { test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

const SKIP = { skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX };
const RATE = 48_000;
const QUANTUM = 128;
/** About 3.5 s of tone, then 1 s of silence. */
const TONE_BLOCKS = 1_313;
const SILENCE_BLOCKS = 375;
/** -20 dBFS on the left only: BS.1770 reads it -23.01 LUFS (half the energy of both channels). */
const LEFT_AMPLITUDE = 10 ** (-20 / 20);

let assetPromise;
function asset() {
  assetPromise ??= moduleBytes().then((bytes) => MisoEngineAsset.load(bytes));
  return assetPromise;
}

async function engineFor(blocks) {
  return createOfflineEngine(sessionDocument({ frames: (blocks + 1) * QUANTUM, sessionId: "sdk-loudness" }), {
    asset: await asset(),
  });
}

/** Submit and render block `block`: a 1 kHz sine on the left while `tone`, silence otherwise. */
function renderBlock(engine, block, tone) {
  const left = new Float32Array(QUANTUM);
  if (tone) {
    for (let frame = 0; frame < QUANTUM; frame += 1) {
      left[frame] = LEFT_AMPLITUDE * Math.sin((2 * Math.PI * 1_000 * (block * QUANTUM + frame)) / RATE);
    }
  }
  const acknowledgement = engine.submitSource({
    sourceId: "s",
    generation: 1n,
    startFrame: BigInt(block * QUANTUM),
    planes: [left, new Float32Array(QUANTUM)],
    endOfRegion: false,
  });
  assert.equal(acknowledgement.ok, true, `source block ${block} was refused`);
  engine.render(QUANTUM);
}

function near(actual, expected, tolerance, what) {
  assert.ok(actual !== undefined && Math.abs(actual - expected) <= tolerance,
    `${what} is ${actual}, not within ${tolerance} of ${expected}`);
}

test("subscribeLoudness reads BS.1770 levels and true peak of the master through the core's frames", SKIP, async () => {
  const engine = await engineFor(TONE_BLOCKS + SILENCE_BLOCKS);
  try {
    const subscription = await engine.subscribeLoudness();
    assert.equal(subscription.readLatest(), undefined, "a reading before the first read");
    for (let block = 0; block < TONE_BLOCKS; block += 1) renderBlock(engine, block, true);
    const tone = await subscription.pump();
    assert.equal(subscription.readLatest(), tone);
    assert.equal(tone.armed, true);
    assert.equal(tone.hopsDropped, 0n);
    assert.equal(typeof tone.integratingSince, "bigint");
    assert.ok(tone.observedSample > 3n * BigInt(RATE), `observed sample ${tone.observedSample}`);
    near(tone.integrated, -23, 0.1, "integrated after the tone");
    near(tone.momentary, -23, 0.1, "momentary after the tone");
    near(tone.shortTerm, -23, 0.1, "short-term after the tone");
    near(tone.truePeak?.[0], -20, 0.1, "left true peak");
    assert.equal(tone.truePeak?.[1], -120, "right true peak of a silent channel");
    near(tone.truePeakLastHop?.[0], -20, 0.1, "left true peak of the last hop");

    // One second of silence: the momentary window is silent, the short-term window holds two
    // seconds of the tone in three, and the gates keep the silence out of the integrated value
    // (the blocks that straddle the tone's end pass the relative gate and pull it about 0.2 LU).
    for (let block = TONE_BLOCKS; block < TONE_BLOCKS + SILENCE_BLOCKS; block += 1) renderBlock(engine, block, false);
    const after = await subscription.pump();
    assert.equal(after.momentary, -120, "momentary of silence");
    near(after.shortTerm, -23.01 + 10 * Math.log10(2 / 3), 0.1, "short-term over two seconds of tone in three");
    near(after.integrated, -23.2, 0.1, "integrated after the silence");
    near(after.maxMomentary, -23, 0.1, "the momentary maximum");
    near(after.maxShortTerm, -23, 0.1, "the short-term maximum");
    // The tone stops mid-cycle, and the 4x interpolator rings on the step: the maximum since reset
    // rises a little above the sine's own peak and never falls.
    assert.ok(after.truePeak[0] >= tone.truePeak[0] && after.truePeak[0] < -19, `left true peak ${after.truePeak[0]}`);
    assert.equal(after.truePeak[1], -120, "right true peak since reset");
    assert.deepEqual(after.truePeakLastHop, [-120, -120], "the last hop is silent");
    assert.ok(after.range !== undefined && after.range >= 0 && after.range < 3, `range ${after.range}`);
    await subscription.close();
  } finally {
    engine.dispose();
  }
});

test("a second subscribeLoudness while one is open is the core's typed loudness.active refusal", SKIP, async () => {
  const engine = await engineFor(8);
  try {
    const first = await engine.subscribeLoudness();
    await assert.rejects(engine.subscribeLoudness(), (error) => {
      assert.ok(error instanceof MisoEngineError, String(error));
      assert.equal(error.code, "invalidArgument");
      assert.equal(error.diagnosticCode, "loudness.active");
      return true;
    });
    await first.close();
  } finally {
    engine.dispose();
  }
});

test("close stops the meter: a fresh subscribe succeeds and the closed handle refuses reset", SKIP, async () => {
  const engine = await engineFor(8);
  try {
    const first = await engine.subscribeLoudness();
    renderBlock(engine, 0, true);
    await first.close();
    const closed = first.readLatest();
    assert.equal(closed?.armed, false, "the stop's reading is kept after close");
    await first.close();
    await assert.rejects(first.reset(), MisoUsageError);
    const second = await engine.subscribeLoudness();
    assert.equal(second.readLatest(), undefined);
    await second.close();
  } finally {
    engine.dispose();
  }
});
