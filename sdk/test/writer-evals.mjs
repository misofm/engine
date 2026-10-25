/**
 * The writer contract, at harness level (adopted ruling 5462139867 finding 5).
 *
 * The audit split app#64's writer evals along the repo seam: #243 proves the CONTRACT here,
 * against the real engine, and #246 owns the browser drag matrices end to end. "No install-error"
 * reads at this boundary as: **a flow-control refusal is never an error and never terminal**.
 *
 * Since #1312, #1346, #1347 and #1345 every live value (fader, mute, pan, input filter, route, and
 * every effect parameter, bypass and EQ target) is held in a latest-target cell, which cannot
 * refuse for room, and since #1589c the writer sends value edits only. The refusal these evals drive
 * is the core's `backpressure` status for a full control queue, from a queue that answers it until
 * the transport moves. The cell contract has its own eval: a paused host takes any number of
 * effect edits, none is refused, and the last value of each lands.
 */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { LiveControlWriter } from "../src/core/writer.ts";
import { CONTROL_TRANSACTION_HEADER_BYTES, controlEditBytes } from "../src/core/control-frame.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";


const EQ_PARAMS = [
  { id: 1, unit: "linear", value: 1.0 },
  { id: 2, unit: "linear", value: 2.0 },
  { id: 3, unit: "hz", value: 1000.0 },
  { id: 4, unit: "db", value: 0.0 },
];

/**
 * One track carrying `instances` parametric EQs, so `effectParam` has real addresses on both lanes.
 *
 * The instance count matters for one test only: the ">32 both-lane batch" case needs more than
 * thirty-two DISTINCT addresses, or the writer's own coalescing would collapse the gesture before
 * the queue ever saw it. One EQ publishes twenty-two live per-lane rows, so four instances give
 * sixty-four addresses -- comfortably past the boundary being probed.
 */
function liveControlDocument(instances = 1) {
  return sessionDocument({
    effects: {
      preInsert: Array.from({ length: instances }, (_unused, index) =>
        effectEntry(`eq${index}`, "miso.parametric-eq", EQ_PARAMS)),
    },
  });
}

/**
 * The live band-gain parameter ids of `miso.parametric-eq`, read from the generated catalog.
 *
 * Four bands, so four distinct addressable controls on one effect instance -- which is exactly
 * what the paused repro needs: four records per flush that cannot coalesce into each other.
 */
const GAIN_IDS = CATALOG.effects
  .find((effect) => effect.id === "miso.parametric-eq")
  .parameters
  .filter((row) => row.name.endsWith("-gain") && row.liveUpdatable)
  .map((row) => row.id);

const LANES = ["left", "right", "both"];

/** A band-gain edit on one lane (`0` left, `1` right) of console slot `eq0`. */
function gainEdit(parameterId, channel, value, slot = "eq0") {
  return {
    address: {
      owner: { kind: "strip", id: "t" },
      section: { kind: "console", slot },
      control: parameterId,
      lane: LANES[channel],
    },
    value,
  };
}

/** A fader edit, which addresses the strip rather than an effect. */
function faderEdit(channel, db) {
  return {
    address: {
      owner: { kind: "strip", id: "t" },
      section: { kind: "builtins" },
      control: 5,
      lane: LANES[channel],
    },
    value: db,
  };
}

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

async function pausedEngine(instances = 1) {
  return createOfflineEngine(liveControlDocument(instances), { asset });
}

/** The core's refusal of a full control queue: flow control, never an error. */
function backpressure() {
  return new MisoControlRefusal({
    code: "protocol.backpressure", status: "backpressure", revision: 3n,
    editIndex: undefined, diagnostics: [],
  });
}

/**
 * A control queue that answers `backpressure` while `full()` says so, as the core's does until a
 * render drains it, and records the values of every batch it was sent.
 */
function gatedQueue(full) {
  const sent = [];
  return {
    sent,
    async submit(edits) {
      sent.push(edits.map((edit) => edit.value));
      if (full()) throw backpressure();
      return { ok: true, revision: 4n, path: "live", appliedOperations: edits.length };
    },
  };
}

function feedAndRender(engine) {
  const shape = engine.shape();
  for (const [index, source] of shape.sources.entries()) {
    const planes = Array.from({ length: source.channels }, (_unused, channel) =>
      ramp(shape.quantumFrames, 11 + index * 16 + channel));
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: engine.nextAbsoluteSample(),
      planes,
      endOfRegion: false,
    });
  }
  return engine.render();
}

describe("the writer contract -- paused", () => {
  test("a paused host takes any number of effect edits, refuses none, and the last of each lands", async () => {
    // The cell contract (#1345): effect parameters are latest-target cells, so a paused transport
    // has no queue to fill. Two hundred flushes of eight non-coalescing addresses (four bands, both
    // lanes) are all admitted, and one block renders as the twin that was sent only the last values.
    // The transaction queue reads the engine's events after every frame, which is what keeps the
    // core's two-event reliable queue from refusing the third commit.
    const many = await pausedEngine();
    const last = await pausedEngine();
    try {
      const final = 200;
      const writer = new LiveControlWriter({ queue: many.controlTransactions(), maximumBatch: 8 });
      for (let flush = 1; flush <= final; flush += 1) {
        for (const parameterId of GAIN_IDS) {
          writer.stage(gainEdit(parameterId, 0, -0.01 * flush));
          writer.stage(gainEdit(parameterId, 1, 0.01 * flush));
        }
        const outcome = await writer.flush();
        assert.equal(outcome.refused, false, `flush ${flush} was refused on a paused host`);
      }
      assert.equal(writer.stats.refusals, 0);
      assert.equal(writer.stats.escalations, 0);
      assert.equal(writer.pending, 0, "every flush took all eight edits");
      assert.equal(writer.stats.admitted, final * GAIN_IDS.length * 2);

      const twin = new LiveControlWriter({ queue: last.controlTransactions(), maximumBatch: 8 });
      for (const parameterId of GAIN_IDS) {
        twin.stage(gainEdit(parameterId, 0, -0.01 * final));
        twin.stage(gainEdit(parameterId, 1, 0.01 * final));
      }
      await twin.drain();

      const heard = feedAndRender(many);
      const expected = feedAndRender(last);
      assert.deepEqual(
        [...heard.left, ...heard.right].map((value) => Math.fround(value)),
        [...expected.left, ...expected.right].map((value) => Math.fround(value)),
        "the block applies the last value of each address, as the twin that was sent only those",
      );
      assert.ok(
        [...heard.left].some((value) => value !== 0),
        "the comparison is over audible output, not silence",
      );
    } finally {
      many.dispose();
      last.dispose();
    }
  });

  test("a refusal is not terminal: once the queue drains, the FINAL value lands", async () => {
    // Plausible defect: a refused edit is kept beside the newer one for its address, so the backlog
    // grows with the gesture and a stale value lands after the drain.
    let full = true;
    const queue = gatedQueue(() => full);
    const writer = new LiveControlWriter({ queue, maximumBatch: 1 });
    // The queue is full: every attempt is refused while the hand keeps moving.
    for (const db of [-1, -2, -3, -4]) {
      writer.stage(faderEdit(2, db));
      assert.equal((await writer.flush()).refused, true);
    }
    assert.equal(
      writer.pending,
      1,
      "latest-wins coalescing means a full queue never builds a backlog: one address, "
      + "one pending edit, however long the gesture lasted",
    );
    assert.equal(writer.stats.escalations, 0);

    // The transport moves: a render drains the queue.
    full = false;
    const outcome = await writer.drain();
    assert.equal(outcome.refused, false, "after the drain the pending edit fits");
    assert.equal(writer.pending, 0, "the final edit landed");
    assert.deepEqual(queue.sent.at(-1), [-4], "the value the hand reached is the one that lands");
    assert.equal(writer.stats.escalations, 0, "nothing escalated across the whole episode");
  });
});

describe("the writer contract -- coalescing and escalation", () => {
  test("a hundred positions of one control collapse to one pending record", () => {
    const writer = new LiveControlWriter({
      queue: { submit: () => { throw new Error("not reached"); } },
      maximumBatch: 4,
    });
    for (let position = 0; position < 100; position += 1) {
      writer.stage(gainEdit(GAIN_IDS[0], 2, -0.1 * position));
    }
    assert.equal(writer.pending, 1, "one address, one pending record");
    assert.equal(writer.stats.coalesced, 99);
  });

  test("distinct addresses do not coalesce into each other", () => {
    const writer = new LiveControlWriter({
      queue: { submit: () => { throw new Error("not reached"); } },
      maximumBatch: 4,
    });
    // Same parameter, different lanes, and a different kind entirely: three addresses.
    writer.stage(gainEdit(GAIN_IDS[0], 0, -1));
    writer.stage(gainEdit(GAIN_IDS[0], 1, -1));
    writer.stage(faderEdit(2, -1));
    assert.equal(writer.pending, 3);
    assert.equal(writer.stats.coalesced, 0);
  });

  test("a refusal that is NOT flow control throws rather than retrying forever", async () => {
    // Backpressure will succeed once the render thread drains. An unknown address never will, so
    // retrying it silently would be an infinite loop wearing the costume of resilience.
    const engine = await pausedEngine();
    try {
      const writer = new LiveControlWriter({ queue: engine.controlTransactions(), maximumBatch: 4 });
      const edit = gainEdit(GAIN_IDS[0], 2, -1);
      writer.stage({ ...edit, address: { ...edit.address, owner: { kind: "strip", id: "ghost" } } });
      const refusal = await writer.flush().catch((error) => error);
      assert.ok(refusal instanceof MisoControlRefusal);
      assert.equal(refusal.code, "control.address.unknown");
      assert.equal(writer.stats.escalations, 1);
      assert.equal(writer.stats.refusals, 0, "an unknown strip is not flow control");
    } finally {
      engine.dispose();
    }
  });

  test("backpressure from the core is flow control: nothing is dropped and nothing escalates", async () => {
    // The core answers a full control queue with the `backpressure` status. The writer halves its
    // batch and keeps every edit pending.
    let calls = 0;
    const full = gatedQueue(() => (calls += 1) === 1);
    const writer = new LiveControlWriter({ queue: full, maximumBatch: 8 });
    for (const parameterId of GAIN_IDS) writer.stage(gainEdit(parameterId, 0, -1));
    const refused = await writer.flush();
    assert.equal(refused.refused, true);
    assert.equal(refused.reason, "backpressure");
    assert.equal(refused.pending, GAIN_IDS.length, "a refusal drops nothing");
    assert.equal(refused.nextBatch, 2, "the four that were submitted halve to two");
    assert.equal(writer.stats.escalations, 0);
    const outcome = await writer.drain();
    assert.equal(outcome.refused, false);
    assert.equal(writer.pending, 0);
  });

  test("an edit the frame cannot hold is refused at stage, and nothing is staged", () => {
    const control = new LiveControlWriter({ queue: { submit: () => assert.fail("not reached") } });
    // The retired observation record (#1589c) has no control address.
    assert.throws(() => control.stage({ kind: "observeSubscribe", rack: 1, stripId: "t", effectId: "comp", tapId: 1, windowBlocks: 0 }), MisoUsageError);
    assert.throws(() => control.stage({ ...faderEdit(0, -1), rampSamples: 1.5 }), MisoUsageError);
    assert.equal(control.pending, 0);
  });
});

describe("the writer contract -- the async submit boundary", () => {
  test("two flushes entered without awaiting the first serialize into two disjoint batches", async () => {
    // The failure this guards is a torn batch: the second flush picking its keys out of the
    // pending map while the first is still awaiting its report, so both submit the SAME edits --
    // admitted twice in the stats, and still pending afterwards.
    const decode = (edits) => edits.map((edit) => `${edit.address.lane}/${edit.address.control}`);

    const engine = await pausedEngine();
    try {
      const submitted = [];
      let inFlight = 0;
      let concurrent = 0;
      const writer = new LiveControlWriter({
        queue: {
          submit: async (edits) => {
            inFlight += 1;
            concurrent = Math.max(concurrent, inFlight);
            submitted.push(decode(edits));
            // Two hops, so a second flush entered synchronously after the first has ample room to
            // race in if nothing is serializing them.
            await Promise.resolve();
            await Promise.resolve();
            const report = await engine.controlTransactions().submit(edits);
            inFlight -= 1;
            return report;
          },
        },
        maximumBatch: 4,
      });

      // Eight distinct addresses -- four bands on each of two lanes -- so two batches of four.
      for (const channel of [0, 1]) {
        for (const parameterId of GAIN_IDS) writer.stage(gainEdit(parameterId, channel, -1));
      }
      assert.equal(writer.pending, 8, "eight distinct addresses do not coalesce");

      const first = writer.flush();
      const second = writer.flush();
      const [a, b] = await Promise.all([first, second]);

      assert.equal(concurrent, 1, "a flush must not start while a prior submit is outstanding");
      assert.equal(submitted.length, 2, "two calls are two attempts; the chain orders, never merges");
      assert.deepEqual(
        [...submitted[0], ...submitted[1]].sort(),
        [...new Set([...submitted[0], ...submitted[1]])].sort(),
        "the two batches are disjoint: no edit was submitted twice",
      );
      assert.equal(submitted[0].length + submitted[1].length, 8, "between them they cover the eight");

      assert.equal(a.admitted, 4);
      assert.equal(a.pending, 4, "the first flush leaves the other four staged");
      assert.equal(b.admitted, 4);
      assert.equal(b.pending, 0, "the second flush takes the four the first did not");
      assert.equal(writer.pending, 0, "a torn batch would leave the duplicated edits pending");

      const stats = writer.stats;
      assert.equal(stats.flushes, 2);
      assert.equal(stats.admitted, 8, "admitted counts records, and no record was counted twice");
      assert.equal(stats.refusals, 0);
      assert.equal(stats.escalations, 0);
      assert.equal(stats.coalesced, 0);
    } finally {
      engine.dispose();
    }
  });
});

/**
 * The windows an async submit opens that a synchronous one could not.
 *
 * A synchronous submit is atomic against the writer's own state: nothing can be staged, and no
 * second flush can be entered, between picking a batch and applying its report. Over a port that
 * gap is a round trip, and each of these probes lives inside it.
 */
describe("the writer contract -- races the async boundary opens", () => {
  test("an edit staged while its own batch is in flight is not deleted by that batch's success", async () => {
    // The drag-during-round-trip case, which is the case the widening exists for. The hand does
    // not stop moving while a batch crosses the port: a newer value for an address already in
    // flight is staged before the report comes back.
    //
    // Clearing the batch by ADDRESS would delete that newer edit -- `pending` would read zero
    // while only the stale value ever reached the engine, which is the stale-landing bug
    // latest-wins exists to prevent, reintroduced by the async boundary. The batch is cleared by
    // edit identity instead, so the superseding edit survives and goes out next.
    const engine = await pausedEngine();
    try {
      const submissions = [];
      let openGate;
      let announceEntry;
      const gate = new Promise((resolve) => { openGate = resolve; });
      const entered = new Promise((resolve) => { announceEntry = resolve; });
      let held = false;

      const writer = new LiveControlWriter({
        queue: {
          submit: async (edits) => {
            submissions.push(edits.map((edit) => ({ address: `${edit.address.lane}/${edit.address.control}`, value: edit.value })));
            if (!held) { held = true; announceEntry(); await gate; }
            return engine.controlTransactions().submit(edits);
          },
        },
        maximumBatch: 4,
      });

      const address = GAIN_IDS[0];
      writer.stage(gainEdit(address, 0, -1));
      const inFlight = writer.flush();
      await entered;

      // The hand keeps moving while the batch is out.
      writer.stage(gainEdit(address, 0, -9.9));
      const queued = writer.flush();
      assert.equal(submissions.length, 1, "a second flush cannot submit before the first report");
      openGate();

      const first = await inFlight;
      assert.equal(first.admitted, 1, "the batch that was submitted was admitted");
      assert.deepEqual(submissions[0], [{ address: `left/${address}`, value: -1 }]);
      assert.equal(
        first.pending,
        1,
        "the newer value is still staged: a success clears the edits it SUBMITTED, not the address",
      );
      assert.equal(writer.pending, 1);

      const second = await queued;
      assert.deepEqual(
        submissions[1],
        [{ address: `left/${address}`, value: -9.9 }],
        "the value the hand actually reached goes out on the next flush",
      );
      assert.equal(second.admitted, 1);
      assert.equal(second.pending, 0);
      assert.equal(writer.pending, 0);

      const stats = writer.stats;
      assert.equal(stats.flushes, 2);
      assert.equal(stats.admitted, 2);
      assert.equal(stats.refusals, 0);
      assert.equal(stats.escalations, 0);
      // The in-flight edit was in the pending map when the newer one arrived, and by the map's own
      // rule it was superseded there. It was also submitted -- both are true of the same edit, and
      // the counter reports the map's view.
      assert.equal(stats.coalesced, 1);
    } finally {
      engine.dispose();
    }
  });

  test("an escalation rejects its own caller and does not poison later flushes", async () => {
    // The chain that serializes flushes is a promise, and a promise that is left rejected
    // propagates to everything chained behind it. An escalation must reject the call that caused
    // it and nothing else: the writer is as usable afterwards as a synchronous one, which throws
    // to one caller and leaves the next alone.
    //
    // The escalation is real rather than stubbed: the stub renames the strip on the way past, so
    // the CORE refuses an unknown strip, while the writer's staged edit stays lawful and can be
    // resubmitted unmodified.
    const engine = await pausedEngine();
    try {
      let calls = 0;
      const corrupt = new Set([1, 3]);
      const writer = new LiveControlWriter({
        queue: {
          submit: async (edits) => {
            await Promise.resolve();
            calls += 1;
            const submitted = corrupt.has(calls)
              ? edits.map((edit, index) => index === 0
                ? { ...edit, address: { ...edit.address, owner: { kind: "strip", id: "ghost" } } }
                : edit)
              : edits;
            return engine.controlTransactions().submit(submitted);
          },
        },
        maximumBatch: 1,
      });

      writer.stage(gainEdit(GAIN_IDS[0], 0, -1));
      await assert.rejects(() => writer.flush(), MisoControlRefusal);
      assert.equal(writer.stats.escalations, 1);
      assert.equal(writer.pending, 0, "the refused edit is removed, so it is never resent");

      writer.stage(gainEdit(GAIN_IDS[0], 0, -1));
      const recovered = await writer.flush();
      assert.equal(recovered.refused, false, "the next flush is a fresh attempt, not an inheritance");
      assert.equal(recovered.admitted, 1);
      assert.equal(writer.stats.escalations, 1, "the second flush did not escalate");

      // And the same when the second flush is entered before the first has rejected, which is when
      // it is actually chained behind it.
      writer.stage(gainEdit(GAIN_IDS[1], 0, -2));
      writer.stage(gainEdit(GAIN_IDS[2], 0, -3));
      const escalating = writer.flush();
      const behind = writer.flush();
      await assert.rejects(() => escalating, MisoControlRefusal);
      const survived = await behind;
      assert.equal(survived.refused, false, "a flush chained behind a rejected one must still run");
      assert.equal(survived.admitted, 1);
      assert.equal(writer.stats.escalations, 2);
    } finally {
      engine.dispose();
    }
  });

  test("a refusal halves the count that was submitted, not the ceiling", async () => {
    // `take` is min(batch, pending), and it is captured before the submit. Halving the CEILING
    // instead would leave the next attempt larger than the batch that was just refused -- the
    // opposite of narrowing -- and the gap is only visible when fewer edits are pending than the
    // ceiling allows.
    let calls = 0;
    const writer = new LiveControlWriter({ queue: gatedQueue(() => (calls += 1) === 1), maximumBatch: 8 });
    for (const parameterId of GAIN_IDS.slice(0, 3)) writer.stage(gainEdit(parameterId, 0, -1));
    assert.equal(writer.pending, 3, "three staged against a ceiling of eight");

    const refused = await writer.flush();
    assert.equal(refused.refused, true, "the full queue has no room");
    assert.equal(refused.reason, "backpressure");
    assert.equal(
      refused.nextBatch,
      1,
      "the three that were submitted halve to one; the ceiling of eight would halve to four",
    );
    assert.equal(refused.pending, 3, "a refusal drops nothing");
    assert.equal(writer.stats.refusals, 1);
    assert.equal(writer.stats.escalations, 0);
  });
});


test("writer construction requires a queue with a callable submit", () => {
  const submit = () => { throw new Error("constructor must not submit"); };
  // #1589c: the record callbacks `submit` and `submitEdits` are gone; a queue is the one path.
  for (const options of [
    {}, { submit }, { submitEdits: submit }, { queue: { submit }, submit }, { queue: {} }, { queue: null },
    { queue: { submit }, maximumBatch: 0 },
  ]) {
    assert.throws(() => new LiveControlWriter(options), MisoUsageError);
  }
  assert.doesNotThrow(() => new LiveControlWriter({ queue: { submit } }));
});

describe("the writer does not get stuck on a refusal (batch 7 M3)", () => {
  const ghost = (edit) => ({ ...edit, address: { ...edit.address, owner: { kind: "strip", id: "ghost" } } });

  test("a refusal that names an edit removes that edit alone, whatever it was refused for", async () => {
    // Plausible defect: the refused edit stays staged, the batch is one atomic submit, and every
    // later flush resends it and is refused again: the writer is stuck for good.
    const engine = await pausedEngine();
    try {
      const writer = new LiveControlWriter({ queue: engine.controlTransactions() });
      const good = [gainEdit(GAIN_IDS[0], 2, -1), gainEdit(GAIN_IDS[1], 2, -2)];
      // An unknown ID, between two lawful edits.
      writer.stage(good[0]);
      writer.stage(ghost(gainEdit(GAIN_IDS[2], 2, -3)));
      writer.stage(good[1]);
      await assert.rejects(writer.flush(), (error) =>
        error instanceof MisoControlRefusal && error.code === "control.address.unknown" && error.editIndex === 1);
      assert.equal(writer.pending, 2, "only the edit the core named is gone");
      assert.equal((await writer.flush()).admitted, 2, "the others go out on the next flush");
      assert.equal(writer.pending, 0);

      // A value outside its domain, first in the batch.
      writer.stage(faderEdit(2, 1.0e9));
      writer.stage(gainEdit(GAIN_IDS[0], 2, -4));
      await assert.rejects(writer.flush(), (error) =>
        error instanceof MisoControlRefusal && error.editIndex === 0);
      assert.equal(writer.pending, 1);
      assert.equal((await writer.flush()).admitted, 1);
      assert.equal(writer.stats.escalations, 2);
    } finally {
      engine.dispose();
    }
  });

  test("a newer value staged while the refused batch was in flight stays", async () => {
    const engine = await pausedEngine();
    try {
      const queue = engine.controlTransactions();
      const writer = new LiveControlWriter({
        queue: {
          submit: async (edits) => {
            await Promise.resolve();
            // The hand moves while the refusal is on its way back.
            writer.stage(ghost(gainEdit(GAIN_IDS[0], 2, -9)));
            return queue.submit(edits);
          },
        },
      });
      writer.stage(ghost(gainEdit(GAIN_IDS[0], 2, -1)));
      await assert.rejects(writer.flush(), MisoControlRefusal);
      assert.equal(writer.pending, 1, "cleared by identity: the newer edit of that address is still staged");
    } finally {
      engine.dispose();
    }
  });

  test("a whole-batch refusal clears the batch; a call-level failure of the plan does too", async () => {
    // Plausible defect: a refusal or a call failure that names no edit (rebuild pending) keeps the
    // whole batch staged, and it is resent forever.
    const wholeBatch = [
      new MisoControlRefusal({
        code: "control.domain", status: "validation_failed", revision: 3n,
        editIndex: undefined, diagnostics: [],
      }),
      new MisoEngineError("the engine refused a control frame", {
        phase: "lifecycle",
        code: "compileRejected",
        diagnostics: [{ code: "web.apply.rebuild_pending", path: "$" }],
      }),
      new MisoUsageError("the control transaction queue was closed"),
    ];
    for (const failure of wholeBatch) {
      const writer = new LiveControlWriter({ queue: { submit: async () => { throw failure; } } });
      for (const parameterId of GAIN_IDS) writer.stage(gainEdit(parameterId, 0, -1));
      await assert.rejects(writer.flush(), (error) => error === failure);
      assert.equal(writer.pending, 0, failure.name);
      assert.equal(writer.stats.escalations, 1);
      assert.equal(writer.stats.refusals, 0);
    }
  });

  test("backpressure of the plan, and an error that is not the SDK's, are not terminal", async () => {
    // Plausible defect: plan backpressure (a MisoEngineError, not a refusal frame) is thrown as an
    // escalation and loses the hand's position, or a lost port empties the pending edits.
    const calls = [];
    const full = new MisoEngineError("the plan is full", { phase: "lifecycle", code: "backpressure" });
    const lost = new Error("port closed");
    const failures = [full, lost];
    const writer = new LiveControlWriter({
      queue: {
        submit: async (edits) => {
          calls.push(edits.length);
          const failure = failures.shift();
          if (failure !== undefined) throw failure;
          return { ok: true, revision: 1n, path: "live", appliedOperations: edits.length };
        },
      },
    });
    for (const parameterId of GAIN_IDS) writer.stage(gainEdit(parameterId, 0, -1));
    const refused = await writer.flush();
    assert.equal(refused.refused, true);
    assert.equal(refused.reason, "backpressure");
    assert.equal(writer.pending, GAIN_IDS.length);
    await assert.rejects(writer.flush(), (error) => error === lost);
    assert.equal(writer.pending, GAIN_IDS.length, "nothing says these edits are at fault");
    assert.equal(writer.stats.escalations, 0);
    assert.equal((await writer.drain()).refused, false);
    assert.equal(writer.pending, 0);
  });
});

describe("the writer sizes a batch by bytes (batch 7 M4)", () => {
  test("no submit is larger than the host's frame, and every edit still lands", async () => {
    // Plausible defect: a default batch of hundreds of edits (~160 bytes each) is one frame over the
    // host's 4 KiB or 16 KiB ceiling, which fails on every send and is resent forever.
    const edits = GAIN_IDS.flatMap((parameterId) => [0, 1, 2].map((lane) => gainEdit(parameterId, lane, -1)));
    const each = controlEditBytes(edits[0]);
    const ceiling = CONTROL_TRANSACTION_HEADER_BYTES + 4 * each + 1;
    const sent = [];
    const writer = new LiveControlWriter({
      queue: {
        maximumFrameBytes: ceiling,
        maximumTransactionEdits: 1000,
        submit: async (batch) => {
          const bytes = CONTROL_TRANSACTION_HEADER_BYTES
            + batch.reduce((total, edit) => total + controlEditBytes(edit), 0);
          sent.push(bytes);
          assert.ok(bytes <= ceiling, `a ${bytes} byte submit is over the ${ceiling} byte ceiling`);
          return { ok: true, revision: 1n, path: "live", appliedOperations: batch.length };
        },
      },
    });
    for (const edit of edits) writer.stage(edit);
    const outcome = await writer.drain(edits.length);
    assert.equal(writer.pending, 0);
    assert.equal(writer.stats.admitted, edits.length);
    assert.ok(sent.length >= Math.ceil(edits.length / 4), "cut into frames of four");
    assert.ok(outcome.admitted > 0);
  });
});
