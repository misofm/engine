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
import { encodeControlTransaction } from "../src/core/control-frame.ts";
import { ControlTransactionQueue } from "../src/core/control-queue.ts";
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
    const queue = new ControlTransactionQueue(core.send);
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
    const queue = new ControlTransactionQueue(core.send);
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

  test("edits of one submit go in one transaction, and an address staged again moves last", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send);
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
    const queue = new ControlTransactionQueue(core.send);
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
    const queue = new ControlTransactionQueue(core.send);
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
    });
    const refusal = await queue.submit([edit(vocal, 3, 30)]).then(
      () => assert.fail("a refused transaction resolved"),
      (error) => error,
    );
    assert.ok(refusal instanceof MisoControlRefusal);
    assert.equal(refusal.code, "control.domain");
    assert.equal(refusal.diagnosticCode, "control.domain");
    assert.equal(refusal.status, "validation_failed");
    assert.equal(refusal.editIndex, 2);
    assert.deepEqual({ ...refusal.report }, {
      ok: false, revision: 5n, code: "control.domain", editIndex: 2,
    });
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
    });
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
    });
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
    });
    await assert.rejects(queue.submit([edit(vocal, 3, 0)]), /no port/);
    await assert.rejects(queue.submit([edit(vocal, 3, 0)]), /port closed/);
    assert.equal(queue.inFlight, false);
    assert.equal((await queue.submit([edit(vocal, 3, 0)])).ok, true);
  });

  test("a reply for another request, or bytes that are not a reply", async () => {
    const wrong = new ControlTransactionQueue(() => reply("applied_live", 99n, 1n));
    await assert.rejects(wrong.submit([edit(vocal, 3, 0)]), MisoEngineError);
    const garbage = new ControlTransactionQueue(() => new Uint8Array(10));
    await assert.rejects(garbage.submit([edit(vocal, 3, 0)]), MisoEngineError);
    assert.equal(wrong.revision, 0n);
  });

  test("an edit the frame cannot hold is refused at stage and never sits staged", async () => {
    const core = fakeCore();
    const queue = new ControlTransactionQueue(core.send);
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
    const queue = new ControlTransactionQueue(core.send);
    const flying = queue.submit([edit(vocal, 3, -1)]);
    const staged = queue.submit([edit(vocal, 3, -2)]);
    const stagedOutcome = staged.then(() => "resolved", (error) => error);
    queue.reset();
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
    const queue = new ControlTransactionQueue(fakeCore().send);
    queue.close();
    queue.reset();
    assert.throws(() => queue.stage(edit(vocal, 3, 0)), MisoUsageError);
  });

  test("close rejects staged edits and refuses new ones; the transaction in flight settles", async () => {
    const core = fakeCore({ hold: true });
    const queue = new ControlTransactionQueue(core.send);
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

  test("loadSession starts the queue over at the booted revision", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const queue = engine.controlTransactions();
      queue.reset(9n);
      assert.equal(queue.revision, 9n);
      engine.loadSession(sessionDocument());
      assert.equal(engine.controlTransactions(), queue);
      assert.equal(queue.revision, 0n);
    } finally {
      engine.dispose();
    }
  });
});
