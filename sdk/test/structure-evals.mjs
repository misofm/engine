/**
 * #1562b-K: the SDK follows a rebuild.
 *
 * The browser accepts a rebuild like the C ABI does, so the strips, the effect an insert runs and
 * the taps a subscription holds can all change under a running engine. The SDK maps and reports; the
 * core decides. These evals send the structural frames the Rust codec wrote (`control-support.mjs`)
 * to a real core, on the headless engine and through the browser transport over the same core, and
 * hold what the SDK names -- the strips of a meter frame, the effect an effect-row edit is held to, the
 * index an observation record carries, the end of a subscription -- to what the core now says.
 */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { createBrowserLiveControls } from "../src/browser/live-controls.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { decodeControlReply } from "../src/core/control-frame.ts";
import { ObservationSubscriptionOwner, observationFrameTransport } from "../src/core/observation-subscriptions.ts";
import { decodeObservationEnded } from "../src/core/observation-ended.ts";
import { MisoControlRefusal } from "../src/core/errors.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { structuralFrame } from "./control-support.mjs";
import { engineHost, moduleBytes, ramp } from "./support.mjs";

const FIXTURE = JSON.parse(readFileSync(
  new URL("../../hosts/host-web/tests/fixtures/control-list.json", import.meta.url),
  "utf8",
));
const OPTIONS = { liveControls: { meterBlocks: 1, observationTaps: 8 } };
const STRIP = (id) => ({ kind: "strip", id });

let asset;
/**
 * The blocks a rebuild that restarts a strip may take before its successor is adopted and fades
 * in (a duck-swap, #1324a), plus one: `S` waits `q + N + C` after the commit, rounded up to a
 * block, and the fade fires `D` after `S`. `N` is the session's mute ramp; `C` (a route line) and
 * `D` (a strip's latency before its fader) are at most the plan's latency `L`, which the boot plan
 * delays an impulse by: PDC aligns every path to the output.
 */
let duckBlocks;

/** The plan latency `L`: the first output sample an impulse on every source at frame 0 reaches. */
async function measuredLatency() {
  const engine = await boot();
  try {
    const shape = engine.shape();
    const q = shape.quantumFrames;
    for (let block = 0; block < 64; block += 1) {
      for (const source of shape.sources) {
        engine.submitSource({
          sourceId: source.id,
          generation: 1n,
          startFrame: BigInt(block * q),
          planes: Array.from({ length: source.channels }, () => {
            const plane = new Float32Array(q);
            if (block === 0) plane[0] = 1;
            return plane;
          }),
          endOfRegion: false,
        });
      }
      const { left, right } = engine.render();
      const first = [...left].findIndex((sample, index) => sample !== 0 || right[index] !== 0);
      if (first >= 0) return block * q + first;
    }
    throw new Error("the impulse reaches the output within 64 blocks");
  } finally {
    engine.dispose();
  }
}

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
  const session = JSON.parse(FIXTURE.session);
  const smoothing = session.control_smoothing;
  const muteMs = smoothing?.kind === "default" || smoothing === undefined ? 10 : smoothing.mute_ms;
  const n = Math.round((muteMs * session.sample_rate_hz) / 1000);
  const q = session.quantum_frames;
  const latency = await measuredLatency();
  duckBlocks = Math.ceil((q + n + latency) / q) + Math.ceil(latency / q) + 1;
});

const boot = () => createOfflineEngine(FIXTURE.session, { asset, ...OPTIONS });

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 5 + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

/** Blocks from `from` to `from + count`, each fed and rendered; every meter frame completed on the way. */
function run(engine, from, count) {
  const frames = [];
  for (let block = from; block < from + count; block += 1) {
    feed(engine, block);
    engine.render();
    const frame = engine.pollMeters();
    if (frame !== undefined) frames.push(frame);
  }
  return frames;
}

/**
 * Blocks from `from`, as `run` renders them, until `reached()` holds after one: a rebuild that
 * restarts a strip is adopted at its duck-swap's `S`, not at a fixed block (#1324a). Holds within
 * `duckBlocks + extra` blocks or fails; the next block to render.
 */
async function runUntil(engine, from, extra, reached, what) {
  for (let block = from; block < from + duckBlocks + extra; block += 1) {
    run(engine, block, 1);
    if (await reached()) return block + 1;
  }
  assert.fail(`${what} within ${duckBlocks + extra} blocks`);
}

/** Send a structural frame and require the core to take it as a rebuild. */
function rebuild(engine, name) {
  const reply = decodeControlReply(engine.controlFrame(structuralFrame(name, engine)));
  assert.equal(reply.status, "ok", `${name} is committed`);
  assert.equal(reply.applied.editPath, "rebuild", `${name} is a rebuild`);
  return reply.revision;
}

const row = (list, predicate, what) => {
  const found = list.controls.find(predicate);
  assert.ok(found !== undefined, `the list has ${what}`);
  return found;
};
const insertRow = (list, strip, insert, name) => row(
  list,
  (entry) => entry.address.owner.kind === "strip" && entry.address.owner.id === strip
    && entry.address.section.kind === "inserts" && entry.address.section.effect === insert
    && entry.name === name,
  `${strip}/${insert}/${name}`,
);

const tap = (trackId, rack, effectSlotId, channels = "both") => ({ trackId, rack, effectSlotId, tapId: 1, channels });

describe("#1562b-K -- a meter frame names its own strips", () => {
  test("a rebuild that adds a strip: frames before name the old strips, frames after the new", async () => {
    const engine = await boot();
    try {
      assert.equal(engine.meters(true).ok, true);
      const before = run(engine, 0, 12);
      assert.ok(before.length > 0, "frames complete before the rebuild");
      const epoch = before.at(-1).structureEpoch;
      for (const frame of before) {
        assert.deepEqual([...frame.trackIds], ["t1", "t2", "t3"]);
        assert.equal(frame.structureEpoch, epoch);
      }
      rebuild(engine, "add_first_strip");
      const after = run(engine, 12, 16);
      const last = after.at(-1);
      assert.ok(last.structureEpoch > epoch, "the structure epoch moved");
      assert.deepEqual([...last.trackIds], ["a0", "t1", "t2", "t3"]);
      assert.equal(last.trackCount, 4);
      assert.deepEqual([...last.submixIds], ["b"]);
      for (const frame of after) {
        // A frame is named by the list of the plan that rendered it, never by a newer list.
        assert.deepEqual([...frame.trackIds], frame.structureEpoch === epoch
          ? ["t1", "t2", "t3"]
          : ["a0", "t1", "t2", "t3"]);
        assert.equal(frame.trackIds.length, frame.trackCount);
      }
      // The muted new strip is silent, so its reading is the first pair, and t1's is the second.
      assert.equal(last.peaks[0], 0);
      assert.equal(last.peaks[1], 0);
      assert.ok(last.peaks[2] > 0 && last.peaks[3] > 0, "t1 is where the list now says");
      assert.equal(last.structureEpoch, (await engine.controls()).structureEpoch);
    } finally {
      engine.dispose();
    }
  });

  test("a window the old plan completed is named by the old list even after the new list was read", async () => {
    const engine = await boot();
    try {
      assert.equal(engine.meters(true).ok, true);
      const frames = run(engine, 0, 12);
      const epoch = frames.at(-1).structureEpoch;
      // One block rendered by the plan that is about to be replaced, its window not yet polled.
      feed(engine, 12);
      engine.render();
      rebuild(engine, "replace_last_strip");
      assert.deepEqual([...engine.shape().tracks], ["t1", "t2", "t4"], "the newest list is read");
      const late = engine.pollMeters();
      assert.equal(late.structureEpoch, epoch);
      assert.deepEqual([...late.trackIds], ["t1", "t2", "t3"], "the window is the old plan's, so the old strips'");
      const next = run(engine, 13, 6).at(-1);
      assert.deepEqual([...next.trackIds], ["t1", "t2", "t4"]);
    } finally {
      engine.dispose();
    }
  });

  test("a rebuild that keeps the strip count and changes the strips is named by the new list", async () => {
    const engine = await boot();
    try {
      assert.equal(engine.meters(true).ok, true);
      const before = run(engine, 0, 12);
      const epoch = before.at(-1).structureEpoch;
      assert.deepEqual([...before.at(-1).trackIds], ["t1", "t2", "t3"]);
      rebuild(engine, "replace_last_strip");
      const after = run(engine, 12, 16);
      const last = after.at(-1);
      assert.equal(last.trackCount, 3, "the count did not change");
      assert.ok(last.structureEpoch > epoch);
      assert.deepEqual([...last.trackIds], ["t1", "t2", "t4"]);
      for (const frame of after.filter((candidate) => candidate.structureEpoch === epoch)) {
        assert.deepEqual([...frame.trackIds], ["t1", "t2", "t3"]);
      }
      assert.deepEqual([...engine.shape().tracks], ["t1", "t2", "t4"]);
    } finally {
      engine.dispose();
    }
  });
});

describe("#1678 -- the core holds an effect-row edit to the effect it was written for", () => {
  const named = (controls, name, value) => controls.edit.set({
    owner: STRIP("t1"), section: { kind: "inserts", effect: "comp" }, control: name, lane: "both",
  }, value);
  const refusalOf = async (controls, set) => {
    const refusal = await controls.submit(set).then(() => undefined, (error) => error);
    assert.ok(refusal instanceof MisoControlRefusal, "the core refused");
    return refusal;
  };

  test("the builders send the effect an edit was written for", async () => {
    const engine = await boot();
    try {
      const controls = engine.liveControls();
      const insert = controls.edit.track("t1").insert("comp", "miso.compressor").parameter("mix", 0.25);
      assert.equal(insert.edits[0].address.expectedEffect, "miso.compressor");
      assert.equal(named(controls, "mix", 0.25).edits[0].address.expectedEffect, "miso.compressor",
        "a row named by its section carries the effect the section runs");
      const numeric = controls.edit.set({
        owner: STRIP("t1"), section: { kind: "inserts", effect: "comp" }, control: 7, lane: "both",
      }, 0.25);
      assert.equal("expectedEffect" in numeric.edits[0].address, false, "a bare numeric address names no effect");
      assert.equal("expectedEffect" in controls.edit.track("t1").faderDb(-3).edits[0].address, false,
        "a row outside an effect section names none");
    } finally {
      engine.dispose();
    }
  });

  for (const path of ["headless", "browser"]) {
    test(`${path}: an edit written for the old effect is refused by the core after a rebuild`, async () => {
      const engine = await boot();
      try {
        run(engine, 0, 2);
        const controls = path === "headless"
          ? engine.liveControls()
          : await createBrowserLiveControls(engineHost(engine));
        // The controls are made before the rebuild and keep their view of the strips and effects.
        const before = named(controls, "mix", 0.25);
        assert.equal(before.edits[0].address.control, 7, "the compressor's mix row");
        assert.equal((await controls.submit(before)).ok, true);
        assert.equal(insertRow(await engine.controls(), "t1", "comp", "mix").value, 0.25);

        rebuild(engine, "swap_comp_for_soft_clip");
        // The structural frame did not go through this queue (the SDK has no builder for one yet),
        // so the queue takes the committed revision from the core, as after any replaced session.
        controls.queue.reset(engine.controlRevision());
        run(engine, 2, 4);
        const list = await engine.controls();
        assert.equal(insertRow(list, "t1", "comp", "mix").address.control, 3, "the soft clip's mix row");
        const untouched = insertRow(list, "t1", "comp", "mix").value;
        const revision = engine.controlRevision();

        // Built from the old view: row 7 of the compressor. The core refuses it with its own code.
        for (const stale of [
          named(controls, "mix", 0.5),
          named(controls, "threshold_db", -20),
          controls.edit.track("t1").insert("comp", "miso.compressor").parameter("mix", 0.75),
        ]) {
          const refusal = await refusalOf(controls, stale);
          assert.equal(refusal.code, "control.effect.changed");
          assert.equal(refusal.editIndex, 0);
        }
        assert.equal(insertRow(await engine.controls(), "t1", "comp", "mix").value, untouched, "nothing changed");
        assert.equal(engine.controlRevision(), revision, "nothing committed");

        // The same insert typed for the effect it runs now goes through.
        const typedNow = controls.edit.track("t1").insert("comp", "miso.soft-clip").parameter("mix", 0.75);
        assert.equal((await controls.submit(typedNow)).ok, true);
        assert.equal(insertRow(await engine.controls(), "t1", "comp", "mix").value, 0.75);

        // A numeric address names no effect, so it lands on whatever runs there: as before.
        const bare = controls.edit.set({
          owner: STRIP("t1"), section: { kind: "inserts", effect: "comp" }, control: 3, lane: "both",
        }, 0.5);
        assert.equal((await controls.submit(bare)).ok, true);
        assert.equal(insertRow(await engine.controls(), "t1", "comp", "mix").value, 0.5);
      } finally {
        engine.dispose();
      }
    });
  }

  test("an insert the list never had is the core's to refuse, not the SDK's to resolve", async () => {
    const engine = await boot();
    try {
      const controls = engine.liveControls();
      assert.throws(
        () => controls.edit.set({ owner: STRIP("t1"), section: { kind: "inserts", effect: "ghost" }, control: "mix", lane: "both" }, 1),
        /numeric/,
      );
    } finally {
      engine.dispose();
    }
  });
});

/** The OBSERVATION_ENDED events of an engine's reliable lane, decoded. */
function endedEvents(engine) {
  const events = [];
  const stop = engine.onControlEvent((event) => {
    const ended = event.lane === 0 ? decodeObservationEnded(event.frame) : undefined;
    if (ended !== undefined) events.push(ended);
  });
  return { events, stop };
}

describe("#1562b-K -- a managed observation subscription follows a rebuild", () => {
  test("the end of a tap is reported once, with the rest of the subscription kept (headless)", async () => {
    const engine = await boot();
    const wire = endedEvents(engine);
    try {
      const ends = [];
      const handle = await engine.subscribeObservations({
        selections: [tap("t1", "inserts", "comp", "left"), tap("t2", "inserts", "gate", "left")],
        windowBlocks: 2,
        onEnd: (end) => ends.push(end),
      });
      run(engine, 0, 4);
      await handle.pump();
      assert.deepEqual(handle.readLatest().map((rowRead) => rowRead.status), ["ready", "ready"]);

      const revision = rebuild(engine, "swap_comp_for_soft_clip");
      assert.equal(ends.length, 1, "one report for the commit");
      assert.equal(ends[0].revision, revision);
      assert.equal(ends[0].handle, handle);
      assert.deepEqual(ends[0].ended.map((selection) => selection.effectSlotId), ["comp"]);
      assert.deepEqual(ends[0].remaining.map((selection) => selection.effectSlotId), ["gate"]);
      assert.deepEqual(handle.ended.map((selection) => selection.effectSlotId), ["comp"]);
      assert.equal(wire.events.length, 1);
      assert.equal(wire.events[0].keys.length, 1);

      run(engine, 4, 6);
      await handle.pump();
      assert.deepEqual(handle.readLatest().map((rowRead) => [rowRead.effectSlotId, rowRead.status]), [["gate", "ready"]]);
      await handle.close();
      assert.equal(engine.readObservations([tap("t2", "inserts", "gate")])[0].status, "unarmed");
      assert.equal(ends.length, 1);
    } finally {
      wire.stop();
      engine.dispose();
    }
  });

  test("one commit that ends every tap of a subscription closes it and says so once", async () => {
    const engine = await boot();
    try {
      const ends = [];
      const handle = await engine.subscribeObservations({
        selections: [tap("t3", "inserts", "multi"), tap("t3", "inserts", "limiter")],
        windowBlocks: 2,
        onEnd: (end) => ends.push(end),
      });
      run(engine, 0, 4);
      const kept = await engine.subscribeObservations({
        selections: [tap("t2", "inserts", "gate")],
        windowBlocks: 2,
      });
      rebuild(engine, "replace_last_strip");
      assert.equal(ends.length, 1, "the one event of the commit is one report");
      assert.deepEqual(ends[0].ended.map((selection) => selection.effectSlotId), ["multi", "limiter"]);
      assert.deepEqual(ends[0].remaining, []);
      assert.throws(() => handle.readLatest(), /ended/);
      await handle.close();
      assert.equal(kept.ended.length, 0, "a subscription on a surviving tap is untouched");
      run(engine, 4, 6);
      await kept.pump();
      assert.equal(kept.readLatest()[0].status, "ready");
      await kept.close();
    } finally {
      engine.dispose();
    }
  });

  test("removing the insert ends its tap the same way", async () => {
    const engine = await boot();
    try {
      const ends = [];
      const handle = await engine.subscribeObservations({
        selections: [tap("t1", "inserts", "comp")],
        windowBlocks: 2,
        onEnd: (end) => ends.push(end),
      });
      run(engine, 0, 4);
      rebuild(engine, "remove_comp");
      // The removal restarts t1: the plan without `comp` renders from the duck-swap's `S`.
      await runUntil(
        engine,
        4,
        0,
        () => {
          try {
            engine.readObservations([tap("t1", "inserts", "comp")]);
            return false;
          } catch (error) {
            return /unavailable/.test(error.message);
          }
        },
        "the plan without comp renders",
      );
      assert.equal(ends.length, 1);
      assert.deepEqual(handle.ended.map((selection) => selection.effectSlotId), ["comp"]);
      // The subscription is ended; asking for the tap again is the core's unavailable.
      await assert.rejects(
        () => engine.subscribeObservations({ selections: [tap("t1", "inserts", "comp")], windowBlocks: 2 }),
        /unavailable|unsupported/,
      );
    } finally {
      engine.dispose();
    }
  });

  test("a tap that survives a rebuild keeps publishing and is disarmed at the strip's new index", async () => {
    const engine = await boot();
    try {
      const handle = await engine.subscribeObservations({
        selections: [tap("t1", "inserts", "comp")],
        windowBlocks: 2,
      });
      run(engine, 0, 4);
      rebuild(engine, "add_first_strip");
      run(engine, 4, 6);
      await handle.pump();
      assert.equal(handle.readLatest()[0].status, "ready");
      assert.equal(handle.ended.length, 0);
      await handle.close();
      // t1 moved from strip 0 to strip 1: the disarm went to t1, and a0 (strip 0, a copy of t1)
      // was never armed.
      assert.equal(engine.readObservations([tap("t1", "inserts", "comp")])[0].status, "unarmed");
      assert.equal(engine.readObservations([tap("a0", "inserts", "comp")])[0].status, "unarmed");
    } finally {
      engine.dispose();
    }
  });

  test("an effect that moved to another index is disarmed where it is now", async () => {
    const engine = await boot();
    try {
      const handle = await engine.subscribeObservations({
        selections: [tap("t1", "inserts", "comp")],
        windowBlocks: 2,
      });
      run(engine, 0, 4);
      assert.equal(engine.observationMap().bindings.find((b) => b.effectSlotId === "comp" && b.trackId === "t1").effectIndex, 1);
      rebuild(engine, "remove_eq");
      // The removal restarts t1: a duck-swap, adopted at its `S` (#1324a).
      const index = () => engine.observationMap().bindings
        .find((b) => b.effectSlotId === "comp" && b.trackId === "t1").effectIndex;
      const adopted = await runUntil(engine, 4, 0, () => index() === 0, "the plan that moved comp renders");
      // The restarted strip's effects start at rest (#1324 D2): the tap opens a fresh window.
      const next = await runUntil(engine, adopted, 2, async () => {
        await handle.pump();
        return handle.readLatest()[0].status === "ready";
      }, "the moved effect's tap is ready");
      assert.equal(handle.readLatest()[0].status, "ready", "the tap is carried to the moved effect");
      assert.equal(handle.ended.length, 0);
      await handle.close();
      run(engine, next, 2);
      assert.equal(engine.readObservations([tap("t1", "inserts", "comp")])[0].status, "unarmed");
    } finally {
      engine.dispose();
    }
  });
});

// #1677 (the successor of batch 12b review M1). A rebuild's commit moves the control list at once,
// and the plan that renders keeps its own positions until the block that adopts the replacement. An
// `Observe` edit names its tap by stable key (the strip, the effect and the tap) and the core
// resolves it on the newest committed plan, so an edit sent in that window arms the tap it names
// and no client keeps an index space to get wrong. `swap_t3_multi_and_limiter` puts two effects with
// the same tap (`Gain Reduction`, ID 1) at each other's index: a record that named `multi` by the
// rendering plan's index 0 would arm `limiter`, which is index 0 in the newest plan.
describe("#1677 -- an Observe edit names its tap by stable key", () => {
  const armed = (engine, effectSlotId) => engine.readObservations([tap("t3", "inserts", effectSlotId)])[0].status;

  /** An engine whose rebuild is committed and not yet adopted. */
  async function committedNotAdopted() {
    const engine = await boot();
    run(engine, 0, 4);
    rebuild(engine, "swap_t3_multi_and_limiter");
    return engine;
  }

  /**
   * The subscriptions through the headless engine. Both engines send the same `Observe` frames to
   * the same core entry since #1589b (no index of either plan is on the wire), so a hand-built
   * browser transport would test only itself.
   */
  const transports = [
    ["headless", (engine) => engine],
  ];

  for (const [transport, open] of transports) {
    test(`${transport}: a subscribe in the window is accepted and arms the named tap, not its neighbour`, async () => {
      const engine = await committedNotAdopted();
      try {
        const handle = await open(engine).subscribeObservations({
          selections: [tap("t3", "inserts", "multi")],
          windowBlocks: 2,
        });
        assert.equal(armed(engine, "limiter"), "unarmed", "the other effect's tap is not armed in its place");
        // The swap restarts t3: a duck-swap, adopted at its `S` (#1324a).
        await runUntil(engine, 4, 2, async () => {
          await handle.pump();
          return handle.readLatest()[0].status === "ready";
        }, "multi's tap is ready");
        assert.equal(handle.readLatest()[0].status, "ready");
        assert.notEqual(armed(engine, "multi"), "unarmed", "multi's own tap is armed");
        assert.equal(armed(engine, "limiter"), "unarmed", "limiter's tap is not");
        await handle.close();
      } finally {
        engine.dispose();
      }
    });

    test(`${transport}: a close in the window disarms the named tap, not its neighbour`, async () => {
      const engine = await boot();
      try {
        const handle = await open(engine).subscribeObservations({
          selections: [tap("t3", "inserts", "multi")],
          windowBlocks: 2,
        });
        run(engine, 0, 4);
        rebuild(engine, "swap_t3_multi_and_limiter");
        await handle.close();
        run(engine, 4, 4);
        assert.equal(armed(engine, "multi"), "unarmed", "multi is disarmed where it is now");
        assert.equal(armed(engine, "limiter"), "unarmed");
      } finally {
        engine.dispose();
      }
    });

    test(`${transport}: an update in the window moves the arm to the named tap`, async () => {
      const engine = await boot();
      try {
        const handle = await open(engine).subscribeObservations({
          selections: [tap("t3", "inserts", "limiter")],
          windowBlocks: 2,
        });
        run(engine, 0, 4);
        rebuild(engine, "swap_t3_multi_and_limiter");
        await handle.update({ selections: [tap("t3", "inserts", "multi")], windowBlocks: 2 });
        // The swap restarts t3: a duck-swap, adopted at its `S` (#1324a).
        await runUntil(engine, 4, 2, async () => {
          await handle.pump();
          return handle.readLatest()[0].status === "ready";
        }, "multi's tap is ready");
        assert.equal(handle.readLatest()[0].status, "ready");
        assert.notEqual(armed(engine, "multi"), "unarmed");
        assert.equal(armed(engine, "limiter"), "unarmed", "limiter was released where it is now");
        await handle.close();
      } finally {
        engine.dispose();
      }
    });
  }
});

// Batch 12b review, m3. A deferred `OBSERVATION_ENDED` event of an earlier commit names a tap by its
// stable key only. The tap may have been armed again since (removed at R1, added back, subscribed):
// the event ended the earlier arm, and the new subscription is the core's.
describe("batch 12b review m3 -- an ended event older than the arm is ignored", () => {
  test("an event of a commit at or before the arm does not close the subscription; a later one does", async () => {
    const engine = await boot();
    let deliver;
    const owner = new ObservationSubscriptionOwner({
      observationMap: () => engine.observationMap(),
      ...observationFrameTransport({
        send: (frame) => engine.controlFrame(frame),
        requestIds: engine.controlTransactions().requestIds,
        committedRevision: () => engine.controlRevision(),
        maximumFrameBytes: () => undefined,
      }),
      sampleRateHz: () => engine.shape().sampleRateHz,
      observationEnded: (listener) => { deliver = listener; return () => {}; },
    });
    try {
      run(engine, 0, 2);
      const ends = [];
      const { handle } = await owner.subscribe({
        selections: [tap("t1", "inserts", "comp")],
        windowBlocks: 2,
        onEnd: (end) => ends.push(end),
      });
      const armedAt = engine.controlRevision();
      const key = { stripId: "t1", rack: "inserts", effectSlotId: "comp", tapId: 1 };
      deliver({ sequence: 1n, revision: armedAt - 1n, keys: [key] });
      deliver({ sequence: 2n, revision: armedAt, keys: [key] });
      assert.equal(ends.length, 0, "the events belong to earlier arms of the tap");
      run(engine, 2, 4);
      await handle.pump();
      assert.equal(handle.readLatest()[0].status, "ready", "the tap is still the subscription's");
      deliver({ sequence: 3n, revision: armedAt + 1n, keys: [key] });
      assert.equal(ends.length, 1, "a commit after the arm ended it");
      assert.deepEqual(ends[0].ended.map((selection) => selection.effectSlotId), ["comp"]);
    } finally {
      owner.invalidate(true);
      engine.dispose();
    }
  });
});
