/**
 * The writer contract, at harness level (adopted ruling 5462139867 finding 5).
 *
 * The audit split app#64's writer evals along the repo seam: #243 proves the CONTRACT here,
 * against the real engine's real bounded queue, and #246 owns the browser drag matrices end to
 * end. "No install-error" reads at this boundary as: **a flow-control refusal is never an error
 * and never terminal**.
 *
 * Every number below is derived from the engine rather than asserted at it. The queue depth comes
 * from the generated `defaultCommandQueueRecords`; the flush counts and batch sizes are measured
 * against a live paused session and then checked against the arithmetic they ought to follow, so a
 * change in the engine's lowering rule shows up as a disagreement rather than as a silent pass.
 */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { ConsoleWriter } from "../src/core/writer.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

const QUEUE_RECORDS = ABI_LAYOUT.constants.defaultCommandQueueRecords;

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
 * the queue ever saw it. One EQ publishes sixteen live per-lane rows, so four instances give
 * sixty-four addresses -- comfortably past the boundary being probed.
 */
function consoleDocument(instances = 1) {
  return sessionDocument({
    effects: {
      simd1: Array.from({ length: instances }, (_unused, index) =>
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

/** Every live per-lane row of the same effect, for the batches that need more than four addresses. */
const LIVE_IDS = CATALOG.effects
  .find((effect) => effect.id === "miso.parametric-eq")
  .parameters
  .filter((row) => row.liveUpdatable)
  .map((row) => row.id);

/** A band-gain edit on one lane. `parameterId` varies so edits address distinct controls. */
function gainEdit(parameterId, channel, value) {
  return {
    kind: "effectParam",
    trackIndex: 0,
    rack: 0,
    channel,
    effectIndex: 0,
    parameterId,
    smoothingSamples: 0,
    values: [value, 0, 0, 0],
  };
}

/** A fader edit, which addresses the strip rather than an effect. */
function faderEdit(channel, db) {
  return {
    kind: "faderDb",
    trackIndex: 0,
    rack: 255,
    channel,
    effectIndex: 0,
    parameterId: 0,
    smoothingSamples: 0,
    values: [db, 0, 0, 0],
  };
}

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

async function pausedEngine(instances = 1) {
  return createOfflineEngine(consoleDocument(instances), {
    asset,
    console: { commandQueueRecords: QUEUE_RECORDS },
  });
}

/**
 * Distinct both-lane addresses, each carrying its own row's declared default.
 *
 * The VALUE is irrelevant to flow control but not to admission: a frequency row will refuse a
 * decibel value on `domain` grounds, which is a caller error rather than backpressure. Using each
 * row's own default keeps every record lawful, so the only thing the engine can object to is that
 * the queue is full -- which is the whole point of the probe.
 */
function bothLaneAddresses(instances) {
  const rows = CATALOG.effects
    .find((effect) => effect.id === "miso.parametric-eq")
    .parameters
    .filter((row) => row.liveUpdatable);
  const edits = [];
  for (let effectIndex = 0; effectIndex < instances; effectIndex += 1) {
    for (const row of rows) {
      edits.push({
        kind: "effectParam",
        trackIndex: 0,
        rack: 0,
        channel: 2,
        effectIndex,
        parameterId: row.id,
        smoothingSamples: 0,
        values: [row.default, 0, 0, 0],
      });
    }
  }
  return edits;
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
  engine.render();
}

describe("the writer contract -- paused", () => {
  test("the 17th flush of 4 single-lane records surfaces typed reason-8", async () => {
    // The repro, exactly as the audit specifies it: queue 64, four lane-records per flush, nothing
    // rendering. 64 / 4 = 16 flushes fit; the 17th has nowhere to go.
    //
    // Nothing about that is asserted from the arithmetic alone -- the engine is asked, and the
    // arithmetic is then checked against its answer.
    assert.equal(QUEUE_RECORDS, 64, "the generated queue depth is the one this repro is written for");
    const engine = await pausedEngine();
    try {
      const submit = (records, count) => engine.submitCommands(records, count);
      const writer = new ConsoleWriter({ submit, maximumBatch: 4 });

      assert.equal(GAIN_IDS.length, 4, "four bands give four non-coalescing addresses");
      let admittedFlushes = 0;
      let refusal;
      for (let flush = 1; flush <= 20; flush += 1) {
        // Four distinct single-lane controls per flush, so nothing coalesces away and every flush
        // costs the queue exactly four lane records.
        for (const parameterId of GAIN_IDS) {
          writer.stage(gainEdit(parameterId, 0, -0.1 * flush));
        }
        const outcome = await writer.flush();
        if (outcome.refused) { refusal = { flush, outcome }; break; }
        admittedFlushes += 1;
      }

      assert.ok(refusal, "a paused queue must eventually refuse");
      assert.equal(refusal.flush, 17, "the 17th flush is the one with nowhere to go");
      assert.equal(admittedFlushes, 16, "sixteen flushes of four records fill a 64-record queue");
      assert.equal(admittedFlushes * 4, QUEUE_RECORDS, "16 x 4 = 64, the queue depth exactly");
      assert.equal(refusal.outcome.reason, "backpressure");
      assert.equal(
        ABI_LAYOUT.constants.commandReasons.find((row) => row.name === "backpressure").value,
        8,
        "backpressure is reason 8",
      );

      // Never an error, never terminal.
      assert.equal(writer.stats.escalations, 0, "flow control must not escalate");
      assert.equal(writer.stats.refusals, 1);
      assert.ok(writer.pending > 0, "a refused batch stays pending; it is not dropped");
    } finally {
      engine.dispose();
    }
  });

  test("a refusal is not terminal: after the first drain, the FINAL values land", async () => {
    const engine = await pausedEngine();
    try {
      const submit = (records, count) => engine.submitCommands(records, count);
      const writer = new ConsoleWriter({ submit, maximumBatch: 4 });

      // Fill the queue while paused.
      for (let flush = 0; flush < 24; flush += 1) {
        for (const parameterId of GAIN_IDS) {
          writer.stage(gainEdit(parameterId, 0, -0.1 * flush));
        }
        await writer.flush();
      }
      assert.ok(writer.stats.refusals > 0, "the queue filled");
      assert.equal(writer.stats.escalations, 0);

      // The hand keeps moving while the queue is full. These supersede everything pending.
      for (const parameterId of GAIN_IDS) writer.stage(gainEdit(parameterId, 0, -9.9));
      assert.equal(
        writer.pending,
        GAIN_IDS.length,
        "latest-wins coalescing means a full queue never builds a backlog: four addresses, "
        + "four pending records, however long the drag lasted",
      );

      // The transport moves: one render drains every control queue.
      feedAndRender(engine);

      const outcome = await writer.drain();
      assert.equal(outcome.refused, false, "after a drain the pending records fit");
      assert.equal(writer.pending, 0, "the final ramps landed");
      assert.equal(writer.stats.escalations, 0, "nothing escalated across the whole episode");
    } finally {
      engine.dispose();
    }
  });

  test("one both-lane record costs two queue slots: 32 fit, 33 do not", async () => {
    // The engine fact the ">32 both-lane" case rests on, measured rather than assumed.
    //
    // The bound is per DESTINATION QUEUE -- one per addressed effect instance -- so this probe
    // deliberately aims every record at a single instance. Records are built directly rather than
    // through the writer, because what is being measured here is the engine, not the writer: the
    // writer's own coalescing would collapse a repeated address before the queue ever saw it.
    const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
    const kind = ABI_LAYOUT.constants.wireCommandKinds
      .find((row) => row.name === "effectParam").value;
    const rawBatch = (count) => {
      const records = new Uint8Array(count * ABI_LAYOUT.commandRecord.bytes);
      const view = new DataView(records.buffer);
      for (let index = 0; index < count; index += 1) {
        const base = index * ABI_LAYOUT.commandRecord.bytes;
        view.setUint8(base + at("kind"), kind);
        view.setUint8(base + at("rack"), 0);
        view.setUint8(base + at("channel"), 2);
        view.setUint32(base + at("parameterId"), GAIN_IDS[0], true);
        view.setFloat32(base + at("values"), -0.1 * index, true);
      }
      return records;
    };

    let largest = 0;
    let firstRefusal;
    for (let count = 30; count <= 34; count += 1) {
      const engine = await pausedEngine();
      try {
        const report = engine.submitCommands(rawBatch(count), count);
        if (report.ok) largest = count;
        else if (!firstRefusal) firstRefusal = { count, reason: report.reasonName };
      } finally {
        engine.dispose();
      }
    }
    assert.equal(largest, 32, "32 both-lane records fill one 64-record destination queue");
    assert.equal(largest * 2, QUEUE_RECORDS, "each both-lane record lowers to two lane records");
    assert.deepEqual(firstRefusal, { count: 33, reason: "backpressure" });
  });

  test("a single oversized both-lane gesture from idle splits and coalesces under the same contract", async () => {
    // The contract: one gesture, larger than the queue will take, staged from an idle transport.
    //
    // # Why this queue is 16 rather than 64
    //
    // The adopted ruling states this case as "a single >32 both-lane-param batch from idle". That
    // sentence comes from the app, where a strip's controls are counted across a whole track. On
    // this engine the bound is per DESTINATION QUEUE -- one per addressed effect instance -- and
    // the largest launch effect, the parametric EQ, publishes sixteen live per-lane rows. So on a
    // 64-record queue, thirty-two distinct both-lane addresses aimed at one instance do not exist:
    // sixteen do, they cost thirty-two lane slots, and they fit. Spreading them over four
    // instances spreads them over four queues and they fit again.
    //
    // The number 32 is not the contract; being oversized is. It is pinned exactly, at queue 64, by
    // the test above. Here the queue is set to sixteen records -- `commandQueueRecords` is a boot
    // option the caller chooses, and a smaller queue is a legitimate configuration -- so that one
    // instance's sixteen both-lane rows are twice what the queue can hold, and the split behaviour
    // has something real to do.
    const smallQueue = 16;
    const addresses = bothLaneAddresses(1);
    assert.equal(addresses.length, 16, "one EQ publishes sixteen live per-lane rows");
    assert.equal(
      addresses.length * 2,
      smallQueue * 2,
      "sixteen both-lane records cost thirty-two lane slots: twice this queue",
    );

    const engine = await createOfflineEngine(consoleDocument(1), {
      asset,
      console: { commandQueueRecords: smallQueue },
    });
    try {
      const writer = new ConsoleWriter({
        submit: (records, count) => engine.submitCommands(records, count),
        maximumBatch: addresses.length,
      });
      for (const edit of addresses) writer.stage(edit);
      assert.equal(writer.pending, 16, "sixteen distinct addresses do not coalesce");

      const first = await writer.flush();
      assert.equal(first.refused, true, "an oversized gesture does not fit from idle");
      assert.equal(first.reason, "backpressure");
      assert.equal(first.pending, 16, "a refusal admits nothing and drops nothing");
      assert.ok(first.nextBatch < 16, "the writer narrows its next attempt");

      // It splits: the following attempts admit what fits. The writer was never told the
      // boundary; it halved until the engine said yes.
      await writer.drain();
      assert.ok(writer.stats.admitted > 0, "the split admitted part of the gesture");
      assert.ok(writer.stats.refusals >= 1);
      assert.equal(writer.stats.escalations, 0, "flow control never escalates");
      const stillPending = writer.pending;
      assert.ok(stillPending > 0, "a paused transport cannot take the whole gesture");

      // And the rest lands once the transport moves; each render drains every control queue.
      for (let block = 0; block < 8 && writer.pending > 0; block += 1) {
        feedAndRender(engine);
        await writer.drain();
      }
      assert.equal(writer.pending, 0, `${stillPending} pending records landed after the drain`);
      assert.equal(writer.stats.escalations, 0);
    } finally {
      engine.dispose();
    }
  });
});

describe("the writer contract -- playing", () => {
  test("a steady cadence against a rendering transport never refuses", async () => {
    // The playing case is the one that must be boring. Each render drains every control queue, so
    // a cadence that stages a handful of edits per block and flushes once per block has a fresh
    // queue every time.
    const engine = await pausedEngine();
    try {
      const submit = (records, count) => engine.submitCommands(records, count);
      const writer = new ConsoleWriter({ submit, maximumBatch: 8 });

      for (let block = 0; block < 64; block += 1) {
        writer.stage(gainEdit(GAIN_IDS[0], 2, -0.05 * block));
        writer.stage(faderEdit(2, -0.1 * block));
        const outcome = await writer.flush();
        assert.equal(outcome.refused, false, `block ${block} refused under a playing cadence`);
        feedAndRender(engine);
      }

      assert.equal(writer.stats.refusals, 0, "a playing cadence is zero-refusal");
      assert.equal(writer.stats.escalations, 0);
      assert.equal(writer.pending, 0);
      assert.equal(writer.stats.flushes, 64);
    } finally {
      engine.dispose();
    }
  });
});

describe("the writer contract -- coalescing and escalation", () => {
  test("a hundred positions of one control collapse to one pending record", () => {
    const writer = new ConsoleWriter({
      submit: () => { throw new Error("not reached"); },
      maximumBatch: 4,
    });
    for (let position = 0; position < 100; position += 1) {
      writer.stage(gainEdit(GAIN_IDS[0], 2, -0.1 * position));
    }
    assert.equal(writer.pending, 1, "one address, one pending record");
    assert.equal(writer.stats.coalesced, 99);
  });

  test("distinct addresses do not coalesce into each other", () => {
    const writer = new ConsoleWriter({
      submit: () => { throw new Error("not reached"); },
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
      const writer = new ConsoleWriter({
        submit: (records, count) => engine.submitCommands(records, count),
        maximumBatch: 4,
      });
      writer.stage({ ...gainEdit(GAIN_IDS[0], 2, -1), trackIndex: 99 });
      await assert.rejects(() => writer.flush(), MisoUsageError);
      assert.equal(writer.stats.escalations, 1);
      assert.equal(writer.stats.refusals, 0, "an unknown track is not flow control");
    } finally {
      engine.dispose();
    }
  });
});

describe("the writer contract -- the async submit boundary", () => {
  /**
   * The same episode, recorded flush by flush.
   *
   * `wrap` is the only thing that varies: it is what turns the engine's synchronous
   * `submitCommands` into the shape a caller actually has. The app reaches the engine over a
   * worklet port, so its report is a promise by construction (issue #246); the writer's contract
   * is supposed to be identical either way, and "identical" is only a claim until the two
   * transcripts are compared element for element.
   *
   * The episode deliberately covers all three paths: admitted flushes, the refusal that fills a
   * paused queue, and the drain that lands the rest once the transport moves.
   */
  async function episode(wrap, semantic = false) {
    const engine = await pausedEngine();
    try {
      const writer = new ConsoleWriter({
        ...(semantic
          ? { submitEdits: edits => engine.console().submit(...edits) }
          : { submit: wrap((records, count) => engine.submitCommands(records, count)) }),
        maximumBatch: 4,
      });
      const outcomes = [];
      const stats = [];
      for (let flush = 1; flush <= 20; flush += 1) {
        for (const parameterId of GAIN_IDS) {
          writer.stage(gainEdit(parameterId, 0, -0.1 * flush));
        }
        outcomes.push(await writer.flush());
        stats.push(writer.stats);
      }
      feedAndRender(engine);
      outcomes.push(await writer.drain());
      stats.push(writer.stats);
      return { outcomes, stats, pending: writer.pending };
    } finally {
      engine.dispose();
    }
  }

  /** The report as the engine hands it back, in-process. */
  const immediately = (submit) => submit;

  /** The report one microtask later, which is the cheapest honest stand-in for a port hop. */
  const nextMicrotask = (submit) => async (records, count) => {
    await Promise.resolve();
    return submit(records, count);
  };

  test("an async submit produces the same outcomes and the same stats as a sync one", async () => {
    const sync = await episode(immediately);
    const async_ = await episode(nextMicrotask);
    const semantic = await episode(undefined, true);

    // Guard the fixture itself: a transcript that never refused and never admitted would compare
    // equal for the wrong reason.
    assert.ok(sync.outcomes.some((outcome) => outcome.refused), "the episode reached backpressure");
    assert.ok(sync.stats.at(-1).admitted > 0, "the episode admitted records");
    assert.equal(sync.stats.at(-1).escalations, 0);

    assert.deepEqual(async_.outcomes, sync.outcomes, "outcome sequences must not differ by timing");
    assert.deepEqual(async_.stats, sync.stats, "stat sequences must not differ by timing");
    assert.equal(async_.pending, sync.pending);
    assert.deepEqual(semantic, sync, "semantic transport preserves actual admission, backpressure and coalescing");
  });

  test("two flushes entered without awaiting the first serialize into two disjoint batches", async () => {
    // The failure this guards is a torn batch: the second flush picking its keys out of the
    // pending map while the first is still awaiting its report, so both submit the SAME edits --
    // admitted twice in the stats, and still pending afterwards.
    const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
    const decode = (records, count) => {
      const view = new DataView(records.buffer, records.byteOffset, records.byteLength);
      return Array.from({ length: count }, (_unused, index) => {
        const base = index * ABI_LAYOUT.commandRecord.bytes;
        return `${view.getUint8(base + at("channel"))}/${view.getUint32(base + at("parameterId"), true)}`;
      });
    };

    const engine = await pausedEngine();
    try {
      const submitted = [];
      let inFlight = 0;
      let concurrent = 0;
      const writer = new ConsoleWriter({
        submit: async (records, count) => {
          inFlight += 1;
          concurrent = Math.max(concurrent, inFlight);
          submitted.push(decode(records, count));
          // Two hops, so a second flush entered synchronously after the first has ample room to
          // race in if nothing is serializing them.
          await Promise.resolve();
          await Promise.resolve();
          const report = engine.submitCommands(records, count);
          inFlight -= 1;
          return report;
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
  const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;

  /** The addressed control and its first value, read back out of the bytes actually submitted. */
  const decode = (records, count) => {
    const view = new DataView(records.buffer, records.byteOffset, records.byteLength);
    return Array.from({ length: count }, (_unused, index) => {
      const base = index * ABI_LAYOUT.commandRecord.bytes;
      return {
        address: `${view.getUint8(base + at("channel"))}/${view.getUint32(base + at("parameterId"), true)}`,
        value: view.getFloat32(base + at("values"), true),
      };
    });
  };

  for (const semantic of [false, true]) test(`an edit staged while its own batch is in flight is not deleted by that batch's success (${semantic ? "semantic" : "encoded"})`, async () => {
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

      const writer = new ConsoleWriter({
        ...(semantic ? { submitEdits: async edits => {
          submissions.push(edits.map(edit => ({ address: `${edit.channel}/${edit.parameterId}`, value: edit.values[0] })));
          if (!held) { held = true; announceEntry(); await gate; }
          return engine.console().submit(...edits);
        } } : { submit: async (records, count) => {
          submissions.push(decode(records, count));
          if (!held) { held = true; announceEntry(); await gate; }
          return engine.submitCommands(records, count);
        } }),
        maximumBatch: 4,
      });

      const address = GAIN_IDS[0];
      writer.stage(gainEdit(address, 0, -1));
      const inFlight = writer.flush();
      await entered;

      // The hand keeps moving while the batch is out.
      writer.stage(gainEdit(address, 0, -9.9));
      const queued = semantic ? writer.flush() : undefined;
      assert.equal(submissions.length, 1, "a second flush cannot submit before the first report");
      openGate();

      const first = await inFlight;
      assert.equal(first.admitted, 1, "the batch that was submitted was admitted");
      assert.deepEqual(submissions[0], [{ address: `0/${address}`, value: -1 }]);
      assert.equal(
        first.pending,
        1,
        "the newer value is still staged: a success clears the edits it SUBMITTED, not the address",
      );
      assert.equal(writer.pending, 1);

      const second = await (queued ?? writer.flush());
      assert.deepEqual(
        submissions[1],
        [{ address: `0/${address}`, value: semantic ? -9.9 : Math.fround(-9.9) }],
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

  for (const semantic of [false, true]) test(`an escalation rejects its own caller and does not poison later flushes (${semantic ? "semantic" : "encoded"})`, async () => {
    // The chain that serializes flushes is a promise, and a promise that is left rejected
    // propagates to everything chained behind it. An escalation must reject the call that caused
    // it and nothing else: the writer is as usable afterwards as a synchronous one, which throws
    // to one caller and leaves the next alone.
    //
    // The escalation is real rather than stubbed: the stub corrupts the bytes on the way past, so
    // the ENGINE refuses an unknown track, while the writer's staged edit stays lawful and can be
    // resubmitted unmodified.
    const engine = await pausedEngine();
    try {
      let calls = 0;
      const corrupt = new Set([1, 3]);
      const writer = new ConsoleWriter({
        ...(semantic ? { submitEdits: async edits => {
          await Promise.resolve();
          calls += 1;
          const submitted = corrupt.has(calls)
            ? edits.map((edit, index) => index === 0 ? { ...edit, trackIndex: 99 } : edit)
            : edits;
          return engine.console().submit(...submitted);
        } } : { submit: async (records, count) => {
          await Promise.resolve();
          calls += 1;
          if (corrupt.has(calls)) {
            new DataView(records.buffer, records.byteOffset, records.byteLength)
              .setUint32(at("trackIndex"), 99, true);
          }
          return engine.submitCommands(records, count);
        } }),
        maximumBatch: 1,
      });

      writer.stage(gainEdit(GAIN_IDS[0], 0, -1));
      await assert.rejects(() => writer.flush(), MisoUsageError);
      assert.equal(writer.stats.escalations, 1);
      assert.equal(writer.pending, 1, "an escalated batch admits nothing and drops nothing");

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
      await assert.rejects(() => escalating, MisoUsageError);
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
    // ceiling allows, which the synchronous writer's tests never staged.
    const engine = await pausedEngine();
    try {
      // Fill the queue through a writer of its own, so the probe writer's batch is still its
      // untouched ceiling of eight when it makes its one attempt.
      const filler = new ConsoleWriter({
        submit: (records, count) => engine.submitCommands(records, count),
        maximumBatch: 4,
      });
      for (let flush = 0; flush < QUEUE_RECORDS / 4; flush += 1) {
        for (const parameterId of GAIN_IDS) filler.stage(gainEdit(parameterId, 0, -0.1 * flush));
        const outcome = await filler.flush();
        assert.equal(outcome.refused, false, `filler flush ${flush} should still fit`);
      }
      assert.equal(filler.stats.admitted, QUEUE_RECORDS, "the queue is full to the record");

      const writer = new ConsoleWriter({
        submit: async (records, count) => {
          await Promise.resolve();
          return engine.submitCommands(records, count);
        },
        maximumBatch: 8,
      });
      for (const parameterId of GAIN_IDS.slice(0, 3)) writer.stage(gainEdit(parameterId, 1, -1));
      assert.equal(writer.pending, 3, "three staged against a ceiling of eight");

      const outcome = await writer.flush();
      assert.equal(outcome.refused, true, "the filled queue has no room");
      assert.equal(outcome.reason, "backpressure");
      assert.equal(
        outcome.nextBatch,
        1,
        "the three that were submitted halve to one; the ceiling of eight would halve to four",
      );
      assert.equal(outcome.pending, 3, "a refusal drops nothing");
      assert.equal(writer.stats.refusals, 1);
      assert.equal(writer.stats.escalations, 0);
    } finally {
      engine.dispose();
    }
  });
});


test("writer construction requires exactly one callable submission path", () => {
  const submit = () => { throw new Error("constructor must not submit"); };
  for (const options of [{}, { submit, submitEdits: submit }, { submit: 1 }, { submitEdits: null }]) {
    assert.throws(() => new ConsoleWriter(options), MisoUsageError);
  }
  assert.doesNotThrow(() => new ConsoleWriter({ submit }));
  assert.doesNotThrow(() => new ConsoleWriter({ submitEdits: submit }));
});
