/**
 * #1389: the headless engine edits its session through the shared core (`core/session-edit.ts`):
 * `apply`, `replaceSession`, `revision`, `watermark()` and `applied(revision)`, against the real
 * module. Every comparison is between two real engines fed the same PCM; nothing is a digest.
 */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { effect, session } from "../src/core/session.ts";
import { transaction } from "../src/core/session-transaction.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { moduleBytes, ramp } from "./support.mjs";

const QUANTUM = 128;
const FRAMES = 48_000;
const SOURCE = (seed) => ({ channels: 2, bitDepth: 24, frames: FRAMES, content: `blake3:${seed.repeat(64)}` });
// A stateful insert on every original track: a successor that restarted an unchanged node's state
// would render other bits than a fresh engine whose state ran from frame 0.
const EQ = () => effect("miso.parametric-eq", { "band-1-gain": { left: 6, right: -4 } });

const route = (trackId) => ({
  id: `${trackId}.main`,
  source: { kind: "track", trackId, tap: "post_pan" },
  destination: { kind: "output_input", outputId: "out" },
});

/** Four tracks on two sources, each with an EQ insert, plus `extra` (id, spec) tracks. */
function declare(extra = []) {
  let builder = session({ id: "headless.apply", sampleRateHz: 48_000, quantumFrames: QUANTUM })
    .source("a", SOURCE("a"))
    .source("b", SOURCE("b"));
  const tracks = [
    ["t1", { source: "a", inserts: [EQ()] }],
    ["t2", { source: "a", inserts: [EQ()] }],
    ["t3", { source: "b", inserts: [EQ()] }],
    ["t4", { source: "b", inserts: [EQ()] }],
    ...extra,
  ];
  for (const [id, spec] of tracks) builder = builder.track(id, spec);
  builder = builder.output("out");
  for (const [id] of tracks) builder = builder.route(route(id));
  return builder;
}

const base = () => declare();
/** The base session plus a fifth, muted track on an existing source with an EQ insert. */
const withFifth = () => declare([["t5", { source: "a", inserts: [EQ()], fader: { leftMute: true, rightMute: true } }]]);

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

const boot = (document) => createOfflineEngine(document, { asset });

/** Feed every source its PCM for `block` (the same bytes on every engine), then render it. */
function step(engine, block) {
  for (const source of engine.shape().sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * QUANTUM),
      planes: [0, 1].map((channel) => ramp(QUANTUM, 7 + block * 8 + channel + (source.id === "b" ? 3 : 0))),
      endOfRegion: false,
    });
  }
  return engine.render();
}

function run(engine, from, count) {
  return Array.from({ length: count }, (_unused, index) => step(engine, from + index));
}

function assertSameBlocks(actual, expected, what) {
  assert.equal(actual.length, expected.length);
  actual.forEach((block, index) => {
    assert.deepEqual(block.left, expected[index].left, `${what}: block ${index} left`);
    assert.deepEqual(block.right, expected[index].right, `${what}: block ${index} right`);
  });
}

/** Whether `promise` has settled, after the microtasks already queued run. */
async function settled(promise) {
  let done = false;
  promise.then(() => { done = true; }, () => { done = true; });
  await new Promise((resolve) => setImmediate(resolve));
  return done;
}

describe("headless session edits", () => {
  test("gate 1: a structural replace keeps every source and node playing", async () => {
    // Red if replaceSession reboots instead of editing (the sources restart at frame 0), or if the
    // successor plan restarts an unchanged node's state: the blocks then differ from a fresh engine
    // booted from the new session and fed the same PCM from frame 0. Red too if the SDK cannot read
    // the `unchanged` path of a replace by the committed document. Gate 4 rides here: red if the
    // engine keeps binding live controls to the booted session after a rebuild.
    const engine = await boot(base());
    const reference = await boot(withFifth());
    try {
      const listBefore = await engine.controls();
      const blocks = run(engine, 0, 6);
      const revision = engine.revision;
      const outcome = await engine.replaceSession(withFifth());
      assert.deepEqual(outcome, { revision: revision + 1n, path: "rebuild" });
      assert.equal(engine.revision, revision + 1n);
      const applied = engine.applied(outcome.revision);
      let block = 6;
      while (!(await settled(applied))) {
        assert.ok(block < 12, "the rebuild applies within six blocks");
        blocks.push(step(engine, block));
        block += 1;
      }
      blocks.push(...run(engine, block, 6));
      assertSameBlocks(blocks, run(reference, 0, block + 6), "replace versus a fresh engine");
      // The committed document again: nothing to commit, and the revision stays.
      assert.deepEqual(await engine.replaceSession(withFifth()), { revision: outcome.revision, path: "unchanged" });

      // Gate 4: a list read before the rebuild is stale; the added track is addressable at once.
      await assert.rejects(engine.controlState(listBefore), { code: "control.list.stale" });
      const controls = engine.liveControls();
      assert.equal((await controls.submit(controls.edit.track("t5").faderDb(-3))).ok, true);
    } finally {
      engine.dispose();
      reference.dispose();
    }
  });

  test("gate 2: a live apply is applied at the watermark of the next block, which ramps", async () => {
    // Red if applied() resolves at the commit instead of the watermark, if the engine never reads
    // the watermark after a render (it never resolves), or if the live value starts later than the
    // first block after the call.
    const engine = await boot(base());
    const reference = await boot(base());
    try {
      run(engine, 0, 3);
      const plain = run(reference, 0, 4);
      const tx = transaction(base()).upsertTrack("t1", { source: "a", inserts: [EQ()], fader: { leftDb: -12, rightDb: -12 } });
      const outcome = await engine.apply(tx.build());
      assert.deepEqual(outcome, { revision: 1n, path: "live" });
      const applied = engine.applied(outcome.revision);
      assert.equal(await settled(applied), false, "pending until a block renders");
      const first = engine.nextAbsoluteSample();
      const edited = step(engine, 3);
      const snapshot = await applied;
      assert.deepEqual(snapshot, {
        revision: 1n,
        sample: first,
        outcome: { exact: true, transitionFallback: false, superseded: false },
      });
      assert.notDeepEqual(edited.left, plain[3].left, "the ramp has begun in that block");
    } finally {
      engine.dispose();
      reference.dispose();
    }
  });

  test("gate 3: a refused transaction commits nothing and leaves the plan alone", async () => {
    // Red if a refusal advances `revision` or touches the running plan.
    const engine = await boot(base());
    const reference = await boot(base());
    try {
      run(engine, 0, 2);
      run(reference, 0, 2);
      const before = engine.revision;
      // A new rate needs a new engine: the core refuses before it commits, with its typed result.
      await assert.rejects(engine.apply(transaction(base()).setSampleRateHz(96_000).build()), (error) =>
        error instanceof MisoEngineError && error.code === "reprepareRequired"
          && error.diagnostics[0]?.code === "control.apply.reprepare_required");
      assert.equal(engine.revision, before);
      assertSameBlocks(run(engine, 2, 4), run(reference, 2, 4), "after a refusal");
    } finally {
      engine.dispose();
      reference.dispose();
    }
  });

  test("gate 5: loadSession rejects a pending applied and restarts the revision", async () => {
    // Red if a reboot leaves applied() pending forever or keeps the old revision.
    const engine = await boot(base());
    try {
      const { revision } = await engine.apply(
        transaction(base()).upsertTrack("t1", { source: "a", inserts: [EQ()], fader: { leftDb: -6 } }).build(),
      );
      const pending = engine.applied(revision);
      engine.loadSession(base());
      await assert.rejects(pending, MisoUsageError);
      assert.equal(engine.revision, 0n);
    } finally {
      engine.dispose();
    }
  });
});
