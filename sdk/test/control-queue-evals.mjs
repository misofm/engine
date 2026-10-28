/**
 * #1563A gate 4: the per-engine transaction queue (D4) keeps one transaction in flight, sends each
 * with the revision the last reply reported, coalesces what is staged meanwhile by address, and
 * resolves reports in submit order.
 *
 * The core is a scripted fake that answers with the Rust codec's reply frames (the fixture of
 * `control-frame-evals.mjs`) patched to the request it received, so the queue is judged against the
 * real reply bytes and the real transaction frame it must send.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import {
  CONTROL_TRANSACTION_HEADER_BYTES,
  controlEditBytes,
  encodeControlTransaction,
  encodeRevisionProbe,
} from "../src/core/control-frame.ts";
import { ControlRequestIds, ControlTransactionQueue } from "../src/core/control-queue.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

const FIXTURE = JSON.parse(await readFile(
  new URL("../../crates/protocol/tests/fixtures/set-control-frames.json", import.meta.url),
  "utf8",
));

function fixtureReply(name) {
  return Uint8Array.from(Buffer.from(FIXTURE.replies.find((row) => row.name === name).hex, "hex"));
}

/** The fixture reply `name`, answering `requestId` and reporting `revision`. */
function reply(name, requestId, revision) {
  const bytes = fixtureReply(name);
  const view = new DataView(bytes.buffer);
  view.setBigUint64(24, requestId, true);
  view.setBigUint64(32, revision, true);
  return bytes;
}

function edit(owner, control, value, lane = "both", rampSamples) {
  return {
    address: { owner, section: { kind: "builtins" }, control, lane },
    value,
    ...(rampSamples === undefined ? {} : { rampSamples }),
  };
}

const vocal = { kind: "strip", id: "vocal" };
const drums = { kind: "strip", id: "drums" };

function hexOf(bytes) {
  return Buffer.from(bytes).toString("hex");
}

/**
 * A scripted core. Each `send` is recorded; a held send waits for `release()`, which answers it with
 * a committed reply at the next revision.
 */
function fakeCore({ hold = false, revisionStep = 1n } = {}) {
  const frames = [];
  const waiting = [];
  let revision = 0n;
  const send = (frame) => {
    frames.push(frame);
    const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
    const requestId = view.getBigUint64(24, true);
    const answer = () => {
      revision += revisionStep;
      return reply("applied_live", requestId, revision);
    };
    if (!hold) return answer();
    return new Promise((resolve) => waiting.push(() => resolve(answer())));
  };
  return {
    send,
    frames,
    release() {
      const next = waiting.shift();
      assert.ok(next, "a send is waiting");
      next();
    },
    get waiting() {
      return waiting.length;
    },
  };
}

const settled = () => new Promise((resolve) => setImmediate(resolve));

describe("one transaction in flight, the committed revision in every frame", () => {
  test("an idle submit sends at once, with revision 0 and request ID 1, and reports the reply", async () => {
    const core = fakeCore();
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const e = edit(vocal, 3, -6);
    const report = await queue.submit([e]);
    assert.equal(core.frames.length, 1);
    assert.equal(
      hexOf(core.frames[0]),
      hexOf(encodeControlTransaction({ requestId: 1n, expectedRevision: 0n, edits: [e] })),
    );
    assert.deepEqual({ ...report }, { ok: true, revision: 1n, path: "live", appliedOperations: 3 });
    assert.equal(queue.revision, 1n);
  });

  test("the next transaction carries the revision the last reply reported", async () => {
    const core = fakeCore({ revisionStep: 7n });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 4n });
    await queue.submit([edit(vocal, 3, -1)]);
    await queue.submit([edit(vocal, 3, -2)]);
    await queue.submit([edit(vocal, 3, -3)]);
    const expected = core.frames.map((frame) => new DataView(frame.buffer).getBigUint64(32, true));
    const requests = core.frames.map((frame) => new DataView(frame.buffer).getBigUint64(24, true));
    // The first frame carries the revision the queue was given; each later one the core's last.
    assert.deepEqual(expected, [4n, 7n, 14n]);
    assert.deepEqual(requests, [1n, 2n, 3n]);
  });

  test("edits staged while one is in flight coalesce by address into one next transaction", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const order = [];
    const track = (label, promise) => promise.then((report) => { order.push([label, report.revision]); });

    const first = track("A", queue.submit([edit(vocal, 3, -1)]));
    assert.equal(core.frames.length, 1);
    assert.ok(queue.inFlight);

    // Three values on one address and one on another, staged while A is in flight.
    const staged = [
      track("B1", queue.submit([edit(vocal, 3, -10, "both", 480)])),
      track("B2", queue.submit([edit(vocal, 3, -20)])),
      track("B3", queue.submit([edit(vocal, 3, -30, "both", 96)])),
      track("C", queue.submit([edit(drums, 4, 5)])),
    ];
    await settled();
    assert.equal(core.frames.length, 1, "no second transaction while one is in flight");
    assert.equal(queue.pending, 2);

    core.release();
    await first;
    await settled();
    assert.equal(core.frames.length, 2, "exactly one next transaction");
    assert.equal(
      hexOf(core.frames[1]),
      hexOf(encodeControlTransaction({
        requestId: 2n,
        expectedRevision: 1n,
        edits: [edit(vocal, 3, -30, "both", 96), edit(drums, 4, 5)],
      })),
      "two edits, the latest value and ramp of each address",
    );
    core.release();
    await Promise.all(staged);
    assert.deepEqual(order, [["A", 1n], ["B1", 2n], ["B2", 2n], ["B3", 2n], ["C", 2n]]);
    assert.equal(core.waiting, 0);
  });

  test("two edits of one row for different expected effects do not coalesce", async () => {
    // Plausible defect: the coalescing key ignores `expectedEffect`, so a stale edit (written for an
    // effect a rebuild replaced) and a fresh one on the same numeric row collapse into one, and the
    // stale submit resolves `ok` as superseded, or the fresh value is dropped behind the stale one.
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const insert = (expectedEffect, value) => ({
      address: { owner: vocal, section: { kind: "inserts", effect: "comp" }, control: 2, lane: "both", expectedEffect },
      value,
    });
    const first = queue.submit([edit(vocal, 1, 0)]);
    const stale = insert("miso.compressor", -10);
    const fresh = insert("miso.gate", -20);
    const staged = [queue.submit([stale]), queue.submit([fresh])];
    core.release();
    await first;
    await settled();
    assert.equal(
      hexOf(core.frames[1]),
      hexOf(encodeControlTransaction({ requestId: 2n, expectedRevision: 1n, edits: [stale, fresh] })),
      "both edits are sent, in staged order",
    );
    core.release();
    await Promise.all(staged);
    assert.deepEqual({ ...queue.stats }, { transactions: 2, coalescedEdits: 0, replacedSubmits: 0 });
  });

  test("edits of one submit go in one transaction, and an address staged again moves last", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const first = queue.submit([edit(vocal, 1, 0)]);
    const pair = [edit(vocal, 5, 0.25, "left"), edit(vocal, 5, 0.75, "right")];
    const second = queue.submit(pair);
    // `both` first, then `left` on the same row, then `both` again: the second `both` is the latest.
    queue.stage(edit(vocal, 6, 1, "both"));
    queue.stage(edit(vocal, 6, 2, "left"));
    queue.stage(edit(vocal, 6, 3, "both"));
    core.release();
    await first;
    await settled();
    assert.equal(
      hexOf(core.frames[1]),
      hexOf(encodeControlTransaction({
        requestId: 2n,
        expectedRevision: 1n,
        edits: [...pair, edit(vocal, 6, 2, "left"), edit(vocal, 6, 3, "both")],
      })),
    );
    core.release();
    await second;
  });

  test("a stale value is never sent: a later value staged during flight goes in the next transaction", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const first = queue.submit([edit(vocal, 3, -1)]);
    const second = queue.submit([edit(vocal, 3, -2)]);
    core.release();
    await first;
    await settled();
    // A third value arrives while the second is in flight: it must not rewrite what was sent.
    const third = queue.submit([edit(vocal, 3, -3)]);
    assert.equal(core.frames.length, 2);
    assert.equal(
      hexOf(core.frames[1]),
      hexOf(encodeControlTransaction({ requestId: 2n, expectedRevision: 1n, edits: [edit(vocal, 3, -2)] })),
    );
    core.release();
    await second;
    await settled();
    assert.equal(
      hexOf(core.frames[2]),
      hexOf(encodeControlTransaction({ requestId: 3n, expectedRevision: 2n, edits: [edit(vocal, 3, -3)] })),
    );
    core.release();
    await third;
  });

  test("flush with nothing staged waits for the transaction in flight, or resolves empty", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    assert.deepEqual({ ...(await queue.flush()) }, {
      ok: true, revision: 0n, path: "model_only", appliedOperations: 0,
    });
    assert.equal(core.frames.length, 0);
    const submitted = queue.submit([edit(vocal, 3, -1)]);
    let flushed = false;
    const flush = queue.flush().then((report) => { flushed = true; return report; });
    await settled();
    assert.equal(flushed, false);
    core.release();
    assert.equal((await flush).revision, (await submitted).revision);
  });
});

describe("a refusal is the core's, once", () => {
  test("it rejects every submit of the transaction with the core's code and is not retried", async () => {
    const frames = [];
    const queue = new ControlTransactionQueue((frame) => {
      frames.push(frame);
      const requestId = new DataView(frame.buffer).getBigUint64(24, true);
      return reply("control_domain", requestId, 5n);
    }, { initialRevision: 0n });
    const refusal = await queue.submit([edit(vocal, 3, 30)]).then(
      () => assert.fail("a refused transaction resolved"),
      (error) => error,
    );
    assert.ok(refusal instanceof MisoControlRefusal);
    assert.equal(refusal.code, "control.domain");
    assert.equal(refusal.diagnosticCode, "control.domain");
    assert.equal(refusal.status, "validation_failed");
    // The fixture's diagnostic names edit 2 of the transaction; the submit holds one edit, so it
    // names none of the submit's own and the refusal is the whole transaction's, without an index.
    assert.equal(refusal.editIndex, undefined);
    assert.deepEqual({ ...refusal.report }, { ok: false, revision: 5n, code: "control.domain" });
    await settledAll();
    assert.equal(frames.length, 1, "not retried");
    // The reply's revision is the core's current one: the next transaction expects it.
    await queue.submit([edit(vocal, 3, 0)]).catch(() => undefined);
    assert.equal(new DataView(frames[1].buffer).getBigUint64(32, true), 5n);
  });

  test("a refusal with no diagnostic carries the status name as its code", async () => {
    const queue = new ControlTransactionQueue((frame) => {
      const requestId = new DataView(frame.buffer).getBigUint64(24, true);
      return reply("revision_conflict", requestId, 9n);
    }, { initialRevision: 0n });
    const refusal = await queue.submit([edit(vocal, 3, 0)]).catch((error) => error);
    assert.ok(refusal instanceof MisoControlRefusal);
    assert.equal(refusal.code, "revision_conflict");
    assert.equal(refusal.editIndex, undefined);
    assert.equal(queue.revision, 9n);
  });

  test("a refused batch does not stop the batch behind it", async () => {
    const core = fakeCore({ hold: true });
    let refuse = true;
    const queue = new ControlTransactionQueue((frame) => {
      if (!refuse) return core.send(frame);
      refuse = false;
      const requestId = new DataView(frame.buffer).getBigUint64(24, true);
      return reply("control_domain", requestId, 3n);
    }, { initialRevision: 0n });
    const refused = queue.submit([edit(vocal, 3, 30)]).catch((error) => error);
    const next = queue.submit([edit(vocal, 3, -3)]);
    assert.ok((await refused) instanceof MisoControlRefusal);
    await settled();
    assert.equal(core.frames.length, 1);
    core.release();
    assert.equal((await next).ok, true);
  });
});

async function settledAll() {
  await settled();
  await settled();
}

describe("transport and reply faults reject the submit and leave the queue usable", () => {
  test("a send that throws or rejects", async () => {
    let calls = 0;
    const queue = new ControlTransactionQueue((frame) => {
      calls += 1;
      if (calls === 1) throw new Error("no port");
      if (calls === 2) return Promise.reject(new Error("port closed"));
      return reply("applied_live", new DataView(frame.buffer).getBigUint64(24, true), 1n);
    }, { initialRevision: 0n });
    await assert.rejects(queue.submit([edit(vocal, 3, 0)]), /no port/);
    await assert.rejects(queue.submit([edit(vocal, 3, 0)]), /port closed/);
    assert.equal(queue.inFlight, false);
    assert.equal((await queue.submit([edit(vocal, 3, 0)])).ok, true);
  });

  test("a reply for another request, or bytes that are not a reply", async () => {
    const wrong = new ControlTransactionQueue(() => reply("applied_live", 99n, 1n), { initialRevision: 0n });
    await assert.rejects(wrong.submit([edit(vocal, 3, 0)]), MisoEngineError);
    const garbage = new ControlTransactionQueue(() => new Uint8Array(10), { initialRevision: 0n });
    await assert.rejects(garbage.submit([edit(vocal, 3, 0)]), MisoEngineError);
    assert.equal(wrong.revision, 0n);
  });

  test("an edit the frame cannot hold is refused at stage and never sits staged", async () => {
    const core = fakeCore();
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    assert.throws(() => queue.stage({ ...edit(vocal, 3, 0), rampSamples: 1.5 }), MisoUsageError);
    assert.throws(() => queue.submit([edit(vocal, 3, 0), { ...edit(vocal, 4, 0), value: "x" }]), MisoUsageError);
    assert.equal(queue.pending, 0);
    assert.equal(core.frames.length, 0);
    assert.equal((await queue.submit([edit(vocal, 3, 0)])).ok, true);
  });
});

describe("a replaced session starts the queue over", () => {
  test("reset rejects staged edits, sets the revision, and ignores a late reply's revision", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const flying = queue.submit([edit(vocal, 3, -1)]);
    const staged = queue.submit([edit(vocal, 3, -2)]);
    const stagedOutcome = staged.then(() => "resolved", (error) => error);
    queue.reset(0n);
    assert.ok((await stagedOutcome) instanceof MisoUsageError);
    core.release();
    await flying;
    assert.equal(queue.revision, 0n, "the old session's reply sets nothing");
    const after = queue.submit([edit(vocal, 3, -3)]);
    core.release();
    await after;
    const requests = core.frames.map((frame) => new DataView(frame.buffer).getBigUint64(24, true));
    assert.deepEqual(requests, [1n, 2n], "request IDs keep counting across a reset");
  });

  test("reset keeps a closed queue closed", () => {
    const queue = new ControlTransactionQueue(fakeCore().send, { initialRevision: 0n });
    queue.close();
    queue.reset(0n);
    assert.throws(() => queue.stage(edit(vocal, 3, 0)), MisoUsageError);
  });

  test("close rejects staged edits and refuses new ones; the transaction in flight settles", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const flying = queue.submit([edit(vocal, 3, -1)]);
    const staged = queue.submit([edit(vocal, 3, -2)]).catch((error) => error);
    queue.close();
    assert.ok((await staged) instanceof MisoUsageError);
    assert.throws(() => queue.submit([edit(vocal, 3, 0)]), MisoUsageError);
    core.release();
    assert.equal((await flying).ok, true);
  });
});

describe("the headless engine owns one queue", () => {
  let asset;
  before(async () => {
    asset = await MisoEngineAsset.load(await moduleBytes());
  });

  test("the same queue every call; closed by dispose", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    const queue = engine.controlTransactions();
    assert.equal(engine.controlTransactions(), queue);
    engine.dispose();
    assert.throws(() => queue.stage(edit(vocal, 3, 0)), MisoUsageError);
  });

  test("the queue starts at the core's committed revision, and loadSession starts it over at the new session's", async () => {
    // Plausible defect: a queue that assumes a booted session is at revision 0 sends its first
    // transaction with a stale expectation, and the core refuses it as a revision conflict. The
    // document's own revision is 1, and a reloaded one can be any.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const queue = engine.controlTransactions();
      assert.equal(queue.revision, 1n, "the document's revision, not 0");
      assert.equal(queue.revision, engine.controlRevision());
      const edit = {
        address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
        value: -3,
      };
      assert.equal((await queue.submit([edit])).revision, 2n);

      const reloaded = JSON.parse(sessionDocument());
      reloaded.revision = "7";
      engine.loadSession(JSON.stringify(reloaded));
      assert.equal(engine.controlTransactions(), queue);
      assert.equal(queue.revision, 7n);
      assert.equal(engine.controlRevision(), 7n);
      assert.equal((await queue.submit([{ ...edit, value: -4 }])).revision, 8n, "accepted at the seeded revision");

      // A refused reboot leaves the queue dropped, never stuck at the old session's revision.
      assert.throws(() => engine.loadSession("{}"));
      await assert.rejects(queue.submit([edit]));
    } finally {
      engine.dispose();
    }
  });

  test("a frame sent through engine.controlFrame shares the queue's request IDs", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const queue = engine.controlTransactions();
      const probe = engine.controlFrame(encodeRevisionProbe(queue.requestIds.next()));
      assert.equal(new DataView(probe.buffer, probe.byteOffset).getBigUint64(32, true), engine.controlRevision());
      const edit = {
        address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
        value: -3,
      };
      await queue.submit([edit]);
      // The queue's own capabilities read drew ID 1, the probe 2 and the transaction 3: the core's
      // replay cache saw no reuse.
      assert.equal(queue.requestIds.next(), 4n);
    } finally {
      engine.dispose();
    }
  });

  test("events the core queues are read after every frame, so the next commit is never refused for them", async () => {
    // The reliable event lane holds two events; unread, the third commit is refused with
    // backpressure. Plausible defect: a headless engine that never reads the lane.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const seen = [];
      const stop = engine.onControlEvent((event) => seen.push(event));
      const queue = engine.controlTransactions();
      for (let step = 0; step < 6; step += 1) {
        const report = await queue.submit([{
          address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
          value: -step - 1,
        }]);
        assert.equal(report.ok, true);
      }
      assert.ok(seen.length >= 6, "each commit queued an event");
      assert.ok(seen.every((event) => event.lane === 0 || event.lane === 1));
      stop();
      await queue.submit([{
        address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
        value: -9,
      }]);
      assert.ok(engine.discardedControlEvents >= 1, "an event with no listener is counted, not queued");
    } finally {
      engine.dispose();
    }
  });
});

describe("callers are isolated in one queue (#1563B)", () => {
  test("a refusal names one submit, which alone is rejected; the others go again at the refusal's revision", async () => {
    // Plausible defect: one caller's out-of-domain edit refuses the transaction it was coalesced
    // into and drops two other callers' valid edits, or blames the wrong submit, or reports an edit
    // index of the sent transaction instead of the submit's own.
    const frames = [];
    const answers = [];
    const queue = new ControlTransactionQueue((frame) => {
      frames.push(frame);
      const requestId = new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true);
      // The fixture's refusal names edit 2 of the transaction it answers.
      const answer = frames.length === 2
        ? reply("control_domain", requestId, 5n)
        : reply("applied_live", requestId, BigInt(frames.length));
      answers.push(answer);
      return frames.length === 1 ? new Promise((resolve) => { release = () => resolve(answer); }) : answer;
    }, { initialRevision: 0n });
    let release;
    const first = queue.submit([edit(vocal, 1, 0)]);
    const [e0, e1, e2, e3] = [edit(vocal, 2, 0), edit(vocal, 3, 0), edit(vocal, 4, 99), edit(vocal, 5, 0)];
    const one = queue.submit([e0]);
    const two = queue.submit([e1, e2]);
    const three = queue.submit([e3]);
    release();
    const outcomes = await Promise.allSettled([first, one, two, three]);
    assert.deepEqual(outcomes.map((o) => o.status), ["fulfilled", "fulfilled", "rejected", "fulfilled"]);
    const refusal = outcomes[2].reason;
    assert.ok(refusal instanceof MisoControlRefusal);
    assert.equal(refusal.code, "control.domain");
    assert.equal(refusal.editIndex, 1, "e2 is the second edit of its own submit");
    assert.equal(frames.length, 3, "the first, the four-edit transaction, and the resend");
    assert.equal(
      hexOf(frames[1]),
      hexOf(encodeControlTransaction({ requestId: 2n, expectedRevision: 1n, edits: [e0, e1, e2, e3] })),
    );
    assert.equal(
      hexOf(frames[2]),
      hexOf(encodeControlTransaction({ requestId: 3n, expectedRevision: 5n, edits: [e0, e3] })),
      "the resend leaves out the refused submit whole, and carries the refusal's revision",
    );
    assert.equal(outcomes[1].value.revision, 3n);
    assert.equal(queue.revision, 3n);
    assert.equal(queue.inFlight, false);
  });

  test("a submit that is superseded by a refused one still lands its own value", async () => {
    // Two submits write one address; the later is refused. The earlier one's value was coalesced
    // away from the first transaction, and it must go in the resend.
    let calls = 0;
    const frames = [];
    let release;
    const queue = new ControlTransactionQueue((frame) => {
      frames.push(frame);
      calls += 1;
      const requestId = new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true);
      if (calls === 1) return new Promise((resolve) => { release = () => resolve(reply("applied_live", requestId, 2n)); });
      return calls === 2
        ? reply("control_domain", requestId, 2n) // names edit 2 of the transaction it answers
        : reply("applied_live", requestId, 3n);
    }, { initialRevision: 0n });
    const first = queue.submit([edit(vocal, 1, 0)]);
    // Three edits so that the fixture's edit index 2 is the later submit's.
    const early = queue.submit([edit(vocal, 2, -5), edit(vocal, 3, -5)]);
    const middle = queue.submit([edit(vocal, 4, 0)]);
    const late = queue.submit([edit(vocal, 2, 99)]);
    release();
    await first;
    const outcomes = await Promise.allSettled([early, middle, late]);
    assert.deepEqual(outcomes.map((o) => o.status), ["fulfilled", "fulfilled", "rejected"]);
    assert.equal(outcomes[2].reason.editIndex, 0);
    // The first transaction held the later value of address 2 last, which is the third edit.
    assert.equal(
      hexOf(frames[1]),
      hexOf(encodeControlTransaction({
        requestId: 2n, expectedRevision: 2n,
        edits: [edit(vocal, 3, -5), edit(vocal, 4, 0), edit(vocal, 2, 99)],
      })),
    );
    assert.equal(
      hexOf(frames[2]),
      hexOf(encodeControlTransaction({
        requestId: 3n, expectedRevision: 2n,
        edits: [edit(vocal, 2, -5), edit(vocal, 3, -5), edit(vocal, 4, 0)],
      })),
      "the earlier submit's value of address 2, coalesced away before, goes in the resend",
    );
  });

  test("a refusal that names no edit rejects every submit it carried, once", async () => {
    let calls = 0;
    const queue = new ControlTransactionQueue((frame) => {
      calls += 1;
      const requestId = new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true);
      return calls === 1
        ? new Promise((resolve) => { release = () => resolve(reply("applied_live", requestId, 1n)); })
        : reply("revision_conflict", requestId, 9n);
    }, { initialRevision: 0n });
    let release;
    const first = queue.submit([edit(vocal, 1, 0)]);
    const a = queue.submit([edit(vocal, 2, 0)]);
    const b = queue.submit([edit(vocal, 3, 0)]);
    release();
    await first;
    const outcomes = await Promise.allSettled([a, b]);
    assert.deepEqual(outcomes.map((o) => o.status), ["rejected", "rejected"]);
    assert.equal(outcomes[0].reason.code, "revision_conflict");
    assert.equal(calls, 2, "not retried");
    assert.equal(queue.revision, 9n);
  });

  test("flush rejects for a staged edit the core refused, not for another caller's", async () => {
    let calls = 0;
    const queue = new ControlTransactionQueue((frame) => {
      calls += 1;
      const requestId = new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true);
      // Edit 2 of the first (three-edit) transaction is refused, then everything applies.
      return calls === 1 ? reply("control_domain", requestId, 1n) : reply("applied_live", requestId, 2n);
    }, { initialRevision: 0n });
    queue.stage(edit(vocal, 1, 0));
    queue.stage(edit(vocal, 2, 0));
    queue.stage(edit(vocal, 3, 99));
    await assert.rejects(queue.flush(), MisoControlRefusal);

    calls = 0;
    const foreign = queue.submit([edit(vocal, 4, 0), edit(vocal, 5, 0), edit(vocal, 6, 99)]);
    queue.stage(edit(vocal, 7, 0));
    const flushed = queue.flush();
    const outcome = await Promise.allSettled([foreign, flushed]);
    assert.deepEqual(outcome.map((o) => o.status), ["rejected", "fulfilled"], "the foreign refusal is the foreign submit's");
  });
});

describe("request IDs come from one allocator", () => {
  test("a queue draws its transaction IDs from the allocator it was given, and keeps counting", async () => {
    const ids = new ControlRequestIds();
    const taken = [ids.next(), ids.next()];
    assert.deepEqual(taken, [1n, 2n]);
    const core = fakeCore();
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n, requestIds: ids });
    assert.equal(queue.requestIds, ids);
    await queue.submit([edit(vocal, 3, 0)]);
    assert.equal(ids.next(), 4n, "the transaction took 3");
    const frame = core.frames[0];
    assert.equal(new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true), 3n);
  });
});

const frameBytes = (frame) => frame.byteLength;
const frameEdits = (frame) => new DataView(frame.buffer, frame.byteOffset).getUint32(40, true);
const requestIdOf = (frame) => new DataView(frame.buffer, frame.byteOffset).getBigUint64(24, true);

describe("a transaction stays under the host's frame ceiling (batch 7 M4)", () => {
  const oneEdit = controlEditBytes(edit(vocal, 3, 0));
  const ceilingFor = (edits) => CONTROL_TRANSACTION_HEADER_BYTES + edits * oneEdit;

  test("staged submits are cut into transactions that each fit, whole and in the order staged", async () => {
    // Plausible defect: coalescing merges every staged submit into one frame, which the transport
    // then refuses (browser 16 KiB, C ABI 4 KiB), and a caller resends the same frame forever.
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, {
      initialRevision: 0n,
      limits: { maximumFrameBytes: ceilingFor(3), maximumTransactionEdits: 1000 },
    });
    const first = queue.submit([edit(vocal, 100, 0)]);
    // While it is in flight: a two-edit submit, then eight single edits, on distinct addresses.
    const pair = queue.submit([edit(vocal, 1, 0), edit(vocal, 2, 0)]);
    const singles = Array.from({ length: 8 }, (_unused, at) => queue.submit([edit(vocal, 10 + at, 0)]));
    core.release();
    for (;;) {
      await settled();
      if (core.waiting === 0) break;
      core.release();
    }
    const reports = await Promise.all([first, pair, ...singles]);
    assert.ok(reports.every((report) => report.ok));
    for (const frame of core.frames) {
      assert.ok(frameBytes(frame) <= ceilingFor(3), `a ${frameBytes(frame)} byte frame is over the ceiling`);
    }
    // 1 + 10 edits: the pair is never split, so it shares a frame with one single (3), and the
    // other seven go three, three and one.
    assert.deepEqual(core.frames.map(frameEdits), [1, 3, 3, 3, 1]);
    assert.deepEqual(
      core.frames.map(requestIdOf),
      [1n, 2n, 3n, 4n, 5n],
      "each transaction drew its own request ID",
    );
  });

  test("a submit that no transaction can carry is refused up front, naming the limit, and stages nothing", async () => {
    // Plausible defect: an oversized submit is staged, fails the transport on every send, and
    // blocks the writer that owns it for good.
    const core = fakeCore();
    const queue = new ControlTransactionQueue(core.send, {
      initialRevision: 0n,
      limits: { maximumFrameBytes: ceilingFor(2), maximumTransactionEdits: 1000 },
    });
    const big = [edit(vocal, 1, 0), edit(vocal, 2, 0), edit(vocal, 3, 0)];
    const thrown = (() => {
      try {
        queue.submit(big);
      } catch (error) {
        return error;
      }
      return undefined;
    })();
    assert.ok(thrown instanceof MisoUsageError);
    assert.match(thrown.message, new RegExp(`limit of ${ceilingFor(2)} bytes`));
    assert.throws(() => queue.stage(edit({ kind: "strip", id: "x".repeat(ceilingFor(2)) }, 1, 0)), MisoUsageError);
    assert.equal(queue.pending, 0);
    assert.equal(core.frames.length, 0);
    // The same address three times is one edit once coalesced, and fits.
    await queue.submit([edit(vocal, 1, 0), edit(vocal, 1, 1), edit(vocal, 1, 2)]);
    assert.equal(frameEdits(core.frames[0]), 1);
    // An edit-count ceiling is held the same way.
    const counted = new ControlTransactionQueue(core.send, {
      initialRevision: 0n,
      limits: { maximumFrameBytes: 1 << 20, maximumTransactionEdits: 2 },
    });
    assert.throws(() => counted.submit(big), /2 edits/);
  });

  test("the real core's limits bound the headless queue", async () => {
    const asset = await MisoEngineAsset.load(await moduleBytes());
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const queue = engine.controlTransactions();
      assert.equal(queue.maximumFrameBytes, 128 << 10, "the web host's default control frame (#2191)");
      assert.ok(queue.maximumTransactionEdits > 0);
      // A submit past the frame is refused here, before the core's staging buffer could be.
      const wide = Array.from({ length: 8000 }, (_unused, at) => edit(vocal, at + 1, 0));
      assert.throws(() => queue.submit(wide), MisoUsageError);
    } finally {
      engine.dispose();
    }
  });
});

describe("a call that failed without a reply (batch 7 minors)", () => {
  test("it resends each submit alone, rejects only the one at fault, and reads the revision again", async () => {
    // Plausible defect: a call-level failure (rebuild pending, plan backpressure) rejects every
    // caller that was coalesced with the one at fault, or leaves the queue on a revision a lost
    // reply made stale.
    const frames = [];
    const reads = [];
    let calls = 0;
    let release;
    const queue = new ControlTransactionQueue((frame) => {
      frames.push(frame);
      calls += 1;
      if (calls === 1) return new Promise((resolve) => { release = () => resolve(reply("applied_live", requestIdOf(frame), 1n)); });
      // The transaction holding all three is refused as a call, then each goes alone: the middle one fails.
      if (calls === 2) throw new MisoEngineError("rebuild", { phase: "lifecycle", code: "compileRejected" });
      if (frameEdits(frame) === 1 && calls === 4) {
        throw new MisoEngineError("rebuild", { phase: "lifecycle", code: "compileRejected" });
      }
      return reply("applied_live", requestIdOf(frame), BigInt(calls));
    }, {
      initialRevision: 0n,
      readRevision: () => {
        reads.push(calls);
        return 40n;
      },
    });
    const first = queue.submit([edit(vocal, 1, 0)]);
    const a = queue.submit([edit(vocal, 2, 0)]);
    const b = queue.submit([edit(vocal, 3, 0)]);
    const c = queue.submit([edit(vocal, 4, 0)]);
    release();
    await first;
    const outcomes = await Promise.allSettled([a, b, c]);
    assert.deepEqual(outcomes.map((row) => row.status), ["fulfilled", "rejected", "fulfilled"]);
    assert.equal(outcomes[1].reason.code, "compileRejected");
    // One frame of three, then three alone: bounded, not retried again.
    assert.equal(calls, 5);
    assert.deepEqual(frames.slice(1).map(frameEdits), [3, 1, 1, 1]);
    // The revision was read after each failure and the next frame carried it.
    assert.deepEqual(reads, [2, 4]);
    assert.equal(new DataView(frames[2].buffer, frames[2].byteOffset).getBigUint64(32, true), 40n);
    assert.equal(queue.inFlight, false);
  });

  test("a lost reply leaves no stale revision: the core is read again", async () => {
    // Plausible defect: the transport threw after the core committed, the queue kept the old
    // revision, and every later transaction was refused as a revision conflict.
    let calls = 0;
    const frames = [];
    const queue = new ControlTransactionQueue((frame) => {
      frames.push(frame);
      calls += 1;
      if (calls === 1) throw new Error("port closed");
      return reply("applied_live", requestIdOf(frame), 9n);
    }, { initialRevision: 3n, readRevision: async () => 8n });
    await assert.rejects(queue.submit([edit(vocal, 1, 0)]), /port closed/);
    assert.equal(queue.revision, 8n);
    await queue.submit([edit(vocal, 1, 0)]);
    assert.equal(new DataView(frames[1].buffer, frames[1].byteOffset).getBigUint64(32, true), 8n);
  });

  test("a request-ID allocator that throws settles the batch and frees the queue", async () => {
    // Plausible defect: a fault inside the send loop leaves the queue "in flight" for good, and
    // every caller waits on a transaction that never settles.
    class Broken extends ControlRequestIds {
      next() {
        throw new Error("ids exhausted");
      }
    }
    const queue = new ControlTransactionQueue(fakeCore().send, { initialRevision: 0n, requestIds: new Broken() });
    await assert.rejects(queue.submit([edit(vocal, 1, 0)]), /ids exhausted/);
    assert.equal(queue.inFlight, false);
  });
});

describe("what a refusal names, and what a replaced submit costs (batch 7 minors)", () => {
  test("a whole-transaction refusal names an edit only when it is in that submit's own edits", async () => {
    // Plausible defect: the transaction-level index of the fixture's diagnostic (edit 2) is
    // reported on a refusal of a submit that holds one edit.
    let calls = 0;
    let release;
    const queue = new ControlTransactionQueue((frame) => {
      calls += 1;
      if (calls === 1) return new Promise((resolve) => { release = () => resolve(reply("applied_live", requestIdOf(frame), 1n)); });
      return reply("control_domain", requestIdOf(frame), 5n);
    }, { initialRevision: 0n });
    const first = queue.submit([edit(vocal, 1, 0)]);
    const a = queue.submit([edit(vocal, 2, 0)]);
    const b = queue.submit([edit(vocal, 3, 0)]);
    release();
    await first;
    const [ra, rb] = await Promise.all([a, b].map((promise) => promise.catch((error) => error)));
    // The diagnostic names edit 2, and the transaction holds two: no submit owns it.
    assert.equal(ra.editIndex, undefined);
    assert.equal(rb.editIndex, undefined);
    assert.equal(ra.report.editIndex, undefined);
  });

  test("a submit whose addresses were all staged again resolves ok, and the queue counts it", async () => {
    // Plausible defect: the replacement is silent, so a caller that reads `ok` believes the core
    // saw its value.
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send, { initialRevision: 0n });
    const first = queue.submit([edit(vocal, 1, 0)]);
    const early = queue.submit([edit(vocal, 2, -1)]);
    const late = queue.submit([edit(vocal, 2, -9)]);
    core.release();
    await first;
    core.release();
    const [earlyReport, lateReport] = await Promise.all([early, late]);
    assert.equal(earlyReport.ok, true);
    assert.equal(earlyReport, lateReport, "the replaced submit gets the later values' report");
    assert.equal(core.frames.length, 2, "its value never reached the core");
    assert.deepEqual({ ...queue.stats }, { transactions: 2, coalescedEdits: 1, replacedSubmits: 1 });
  });

  test("the queue has no assumed revision", () => {
    // Plausible defect: a queue made without the core's revision starts at 0 and meets a revision
    // conflict on its first transaction.
    const send = fakeCore().send;
    assert.throws(() => new ControlTransactionQueue(send), MisoUsageError);
    assert.throws(() => new ControlTransactionQueue(send, {}), MisoUsageError);
    assert.throws(() => new ControlTransactionQueue(send, { initialRevision: 0n }).reset(), MisoUsageError);
  });
});

describe("a control event listener that throws loses nothing (batch 7 M5)", () => {
  test("the committed reply still returns, the failure is counted and reported outside the call", async () => {
    // Plausible defect: the listener runs inside the control call with no guard, so its throw
    // replaces the reply of a transaction the core already committed, and the caller resends it.
    const asset = await MisoEngineAsset.load(await moduleBytes());
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    const reported = [];
    const original = globalThis.reportError;
    globalThis.reportError = (error) => { reported.push(error); };
    try {
      const seen = [];
      engine.onControlEvent(() => { throw new Error("listener fault"); });
      engine.onControlEvent((event) => seen.push(event));
      const queue = engine.controlTransactions();
      const revision = queue.revision;
      const report = await queue.submit([{
        address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
        value: -3,
      }]);
      assert.equal(report.ok, true);
      assert.equal(report.revision, revision + 1n, "the reply of the committed transaction");
      assert.ok(engine.controlListenerFailures >= 1);
      assert.ok(seen.length >= 1, "a later listener still hears the event");
      await settled();
      assert.ok(reported.length >= 1);
      assert.match(String(reported[0].message), /listener fault/);
      assert.equal(engine.discardedControlEvents, 0);
    } finally {
      if (original === undefined) delete globalThis.reportError;
      else globalThis.reportError = original;
      engine.dispose();
    }
  });
});
