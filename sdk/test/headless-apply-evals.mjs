/**
 * #1389: the headless engine edits its session through the shared core (`core/session-edit.ts`):
 * `apply`, `replaceSession`, `revision`, `watermark()` and `applied(revision)`, against the real
 * module. Every comparison is between two real engines fed the same PCM; nothing is a digest.
 */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { ControlTransactionQueue } from "../src/core/control-queue.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { effect, session } from "../src/core/session.ts";
import { encodeSessionReplaceRequest } from "../src/core/session-edit.ts";
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
function declare(extra = [], sampleRateHz = 48_000) {
  let builder = session({ id: "headless.apply", sampleRateHz, quantumFrames: QUANTUM })
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

const FIXTURE = JSON.parse(readFileSync(
  new URL("../../crates/protocol/tests/fixtures/set-control-frames.json", import.meta.url),
  "utf8",
));

test("the SESSION_REPLACE encoder writes the protocol codec's bytes", () => {
  // Red if the encoder drifts from the codec by any byte (a field ID, wire type, mandatory flag,
  // padding or header word) that the core would still accept, or would refuse only with its own
  // digest check.
  const row = FIXTURE.replace;
  const frame = encodeSessionReplaceRequest({
    requestId: BigInt(row.requestId),
    expectedRevision: BigInt(row.expectedRevision),
    documentBytes: row.documentBytes,
    documentSha256: Buffer.from(row.documentSha256, "hex"),
  });
  assert.equal(Buffer.from(frame).toString("hex"), row.hex);
});

test("a commit noted while a queued transaction flies outlives that transaction's older reply", async () => {
  // Red if the queue's reply overwrites a revision `noteCommit` raised while its transaction was in
  // flight: the queue's next transaction then expects the older revision and is refused.
  const hexReply = (requestId, revision) => {
    const bytes = Buffer.from(FIXTURE.replies.find((row) => row.name === "applied_live").hex, "hex");
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    view.setBigUint64(24, requestId, true);
    view.setBigUint64(32, revision, true);
    return new Uint8Array(bytes);
  };
  const expected = [];
  let release;
  const queue = new ControlTransactionQueue((frame) => {
    const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
    expected.push(view.getBigUint64(32, true));
    const answer = hexReply(view.getBigUint64(24, true), view.getBigUint64(32, true) + 1n);
    return expected.length === 1 ? new Promise((resolve) => { release = () => resolve(answer); }) : answer;
  }, { initialRevision: 0n });
  const address = { owner: { kind: "strip", id: "t1" }, section: { kind: "builtins" }, control: 3, lane: "both" };
  const first = queue.submit([{ address, value: -1 }]);
  await new Promise((resolve) => setImmediate(resolve));
  queue.noteCommit(4n); // An apply committed revision 4 while the first transaction flew.
  release();
  await first;
  await queue.submit([{ address, value: -2 }]);
  assert.deepEqual(expected, [0n, 4n]);
});

describe("headless session edits", () => {
  test("gate 1: a structural replace keeps every source and node playing", async () => {
    // Red if replaceSession reboots instead of editing (the sources restart at frame 0), or if the
    // successor plan restarts an unchanged node's state: the blocks then differ from a fresh engine
    // booted from the new session and fed the same PCM from frame 0. Red too if the SDK cannot read
    // the `unchanged` path of a replace by the committed document, if `withSession()` stays held to
    // the booted document after a replace, or if a refused replace rebinds it.
    const engine = await boot(base());
    const reference = await boot(withFifth());
    try {
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
      // D4: live controls now hold a session to the replacement, not to the booted document.
      assert.doesNotThrow(() => engine.liveControls().withSession(withFifth()));
      // A refused replace (a new rate needs a new engine) commits nothing and rebinds nothing.
      const refused = declare([["t5", { source: "a", inserts: [EQ()], fader: { leftMute: true, rightMute: true } }]], 96_000);
      await assert.rejects(engine.replaceSession(refused));
      assert.equal(engine.revision, outcome.revision);
      assert.throws(() => engine.liveControls().withSession(refused), MisoUsageError);
      assert.doesNotThrow(() => engine.liveControls().withSession(withFifth()));
    } finally {
      engine.dispose();
      reference.dispose();
    }
  });

  test("gate 2: a live apply is applied at the watermark of the next block, which ramps", async () => {
    // Red if applied() resolves at the commit instead of the watermark, if the engine never reads
    // the watermark after a render (it never resolves), or if the live value starts later than the
    // first block after the call. Gate 4's second half: red if the engine does not tell its live
    // controls' queue the revision an apply committed beside it (the next submit is then refused
    // `revision_conflict`).
    const engine = await boot(base());
    const reference = await boot(base());
    try {
      const earlier = engine.liveControls();
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
      // Gate 4: a live reply rebinds nothing, and the queue made before the apply sends its next
      // transaction at the revision the apply committed, first try.
      assert.deepEqual(await earlier.submit(earlier.edit.track("t2").faderDb(-3)),
        { ok: true, revision: 2n, path: "live", appliedOperations: 1 });
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

  test("#2194: a replace whose parse cannot fit beside the live session is refusedBudget", async () => {
    // Red if the SDK reports the staging refusal as its own `bufferTooSmall` instead of reading the
    // host's `host.budget.parse_projection` diagnostic. The boot's document limit leaves the budget
    // less the control frame's rows for the parse, so a document at that limit cannot be parsed
    // beside the live session's retained bytes. The limit is read from a fresh instance's refusal.
    const probe = (await asset.instantiate()).exports;
    assert.equal(probe.miso_engine_web_v1_document_ptr(0xffff_ffff), 0);
    const refusal = new TextDecoder().decode(new Uint8Array(
      probe.memory.buffer,
      Number(probe.miso_engine_web_v1_boot_diagnostic_ptr()),
      Number(probe.miso_engine_web_v1_boot_diagnostic_bytes()),
    ).slice());
    const field = (name) => Number(new RegExp(`${name}=(\\d+)`).exec(refusal)?.[1]);
    // The parse transient's multiplier, from the same refusal's projection of its own length.
    const rows = field("control_frame_rows_bytes");
    const multiplier = (field("projected_bytes") - rows) / field("document_bytes");
    const limit = Math.floor((field("budget_bytes") - rows) / multiplier);
    assert.ok(limit > 0, refusal);
    const engine = await boot(base());
    try {
      await assert.rejects(engine.replaceSession(new Uint8Array(limit).fill(0x20)), (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "refusedBudget");
        assert.equal(error.diagnostics[0]?.code, "host.budget.parse_projection");
        assert.match(error.diagnostics[0].path, new RegExp(`document_bytes=${limit},live_retained_bytes=`));
        return true;
      });
      assert.equal(engine.revision, 0n);
    } finally {
      engine.dispose();
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
