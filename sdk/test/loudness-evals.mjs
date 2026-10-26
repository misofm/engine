/**
 * #2115 S3: `subscribeLoudness()`, the managed handle on the session's output loudness meter,
 * against the shipped module (headless). The meter's DSP is `crates/loudness`'s and its command is
 * `crates/control-plane/tests/loudness_monitor.rs`'s; these evals hold the SDK's half: the frame
 * codec over real core replies, the core's refusals reaching the caller typed, and close.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import process from "node:process";
import { test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { LoudnessOwner, loudnessReadingOf } from "../src/core/loudness.ts";
import { decodeLoudnessReply, encodeLoudnessCommand } from "../src/core/loudness-frame.ts";
import { concat, field, readFields } from "../src/core/control-frame.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

const SKIP = { skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX };
/**
 * The Rust codec's `LOUDNESS` frames (`crates/protocol/tests/sdk_message_fixture.rs`). Read before
 * the first `test()`: a top-level await after one would let the runner finish without the tests
 * registered behind it.
 */
const FIXTURE = JSON.parse(await readFile(new URL("./fixtures/protocol-messages.json", import.meta.url), "utf8")).loudness;
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

const fixtureRow = (name) => FIXTURE.find((row) => row.name === name);
const bytesOf = (hex) => Uint8Array.from(Buffer.from(hex, "hex"));
/** A value in the fixture's spelling: bigints as decimal strings. */
const plain = (value) => JSON.parse(JSON.stringify(value, (_key, item) => (typeof item === "bigint" ? item.toString() : item)));

test("the LOUDNESS codec is the Rust codec's: every operation's bytes and every reply field", () => {
  for (const name of ["start", "read", "reset", "stop"]) {
    const row = fixtureRow(name);
    assert.equal(Buffer.from(encodeLoudnessCommand(BigInt(row.request.requestId), row.request.op)).toString("hex"), row.hex, name);
  }
  for (const name of ["reply_full", "reply_empty"]) {
    const row = fixtureRow(name);
    const reply = decodeLoudnessReply(bytesOf(row.hex));
    assert.equal(reply.ok, true, name);
    assert.deepEqual(
      plain({ ok: reply.ok, requestId: reply.requestId, revision: reply.revision, value: loudnessReadingOf(reply.value) }),
      row.expect,
      name,
    );
  }
});

/** `reply_full` with its fields edited by `edit` (a map from field ID to new bytes; `null` drops it). */
function editedReply(edit) {
  const bytes = bytesOf(fixtureRow("reply_full").hex);
  const view = new DataView(bytes.buffer);
  const fields = readFields(bytes.subarray(48), view.getUint32(40, true))
    .flatMap((row) => {
      if (!(row.id in edit)) return [row];
      return edit[row.id] === null ? [] : [{ ...row, value: edit[row.id] }];
    });
  // An ID the frame lacks is appended as an F32 (wire 6); every such ID here is above the last.
  for (const [id, value] of Object.entries(edit)) {
    if (value !== null && !fields.some((row) => row.id === Number(id))) fields.push({ id: Number(id), wire: 6, value });
  }
  const body = concat(fields.map((row) => field(row.id, row.wire, false, row.value)));
  const out = new Uint8Array(48 + body.length);
  out.set(bytes.subarray(0, 48));
  out.set(body, 48);
  const header = new DataView(out.buffer);
  header.setUint32(20, body.length, true);
  header.setUint32(40, fields.length, true);
  return out;
}

const f32 = (value) => {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setFloat32(0, value, true);
  return out;
};

test("the LOUDNESS decoder refuses every reply the Rust codec refuses", () => {
  // The unedited frame decodes, so each row below fails on its own rule.
  assert.equal(decodeLoudnessReply(editedReply({})).ok, true);
  assert.equal(decodeLoudnessReply(editedReply({ 7: f32(-120) })).ok, true, "the -120 floor itself is a level");
  const rules = {
    "a non-finite level": { 7: f32(Number.NaN) },
    "a level below the -120 floor": { 9: f32(-120.5) },
    "a negative loudness range": { 10: f32(-0.25) },
    "half a true-peak pair": { 14: null },
    "half a last-hop true-peak pair": { 15: null },
    "a true peak without integrating-since": { 3: null },
    "integrating-since without a true peak": { 13: null, 14: null },
    "a field the schema does not hold": { 17: f32(0) },
  };
  for (const [rule, edit] of Object.entries(rules)) {
    assert.throws(() => decodeLoudnessReply(editedReply(edit)),
      (error) => error instanceof MisoEngineError && error.code === "internal", rule);
  }
});

test("a managed loudness handle reads on its scheduler's tick and tells onUpdate", async () => {
  const reply = decodeLoudnessReply(bytesOf(fixtureRow("reply_full").hex));
  const sent = [];
  let tick;
  const owner = new LoudnessOwner({
    send: async (operation) => { sent.push(operation); return reply; },
    capabilityFlags: async () => 1n << 17n,
    scheduler: {
      setInterval: (callback, milliseconds) => { assert.equal(milliseconds, 100); tick = callback; return 1; },
      clearInterval: () => { tick = undefined; },
    },
  });
  const updates = [];
  const handle = await owner.subscribe({ onUpdate: (notification) => updates.push(notification) });
  assert.deepEqual(sent, ["start"]);
  assert.equal(handle.readLatest(), undefined);
  assert.equal(typeof tick, "function", "the subscribe started the timer");
  tick();
  await new Promise((resolve) => setImmediate(resolve));
  assert.deepEqual(sent, ["start", "read"]);
  assert.equal(updates.length, 1);
  assert.equal(updates[0].handle, handle);
  assert.deepEqual(updates[0].reading, loudnessReadingOf(reply.value));
  assert.equal(handle.readLatest(), updates[0].reading);
  await handle.close();
  assert.equal(tick, undefined, "close stopped the timer");
});

const REFUSED_ACTIVE = Object.freeze({
  ok: false,
  requestId: 1n,
  revision: 0n,
  status: "validation_failed",
  diagnostics: [{ code: "loudness.active", severity: "error", path: [] }],
  omittedDiagnostics: 0,
  backpressure: undefined,
});
const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

/**
 * A fake core behind a fake port: frames are served one at a time in send order, the first start
 * takes `startMs`, any other frame `frameMs` (a read never answers with `stuckReads`), and the
 * sender gives up on a frame after `deadlineMs` while the core still serves it, as the browser's
 * deadline does.
 */
function fakeCore({ startMs = 0, frameMs = 0, deadlineMs = 1_000, stuckReads = false, failStops = 0 } = {}) {
  const sent = [];
  let armed = false;
  let serving = Promise.resolve();
  let stopsToFail = failStops;
  let firstStart = true;
  const state = () => ({
    observedSample: 0n, armed, integratingSince: undefined, hopsDropped: 0n, sanitizedSamples: 0n, gatedBlockCount: 0n,
    momentaryLufs: undefined, shortTermLufs: undefined, integratedLufs: undefined, loudnessRangeLu: undefined,
    momentaryMaxLufs: undefined, shortTermMaxLufs: undefined, truePeakDbtp: undefined, hopTruePeakDbtp: undefined,
  });
  const send = (operation) => {
    sent.push(operation);
    if (operation === "read" && stuckReads) return new Promise((_resolve, reject) => setTimeout(() => reject(new Error("deadline")), deadlineMs));
    if (operation === "stop" && stopsToFail > 0) {
      stopsToFail -= 1;
      return Promise.reject(new Error("the port failed"));
    }
    const slow = operation === "start" && firstStart;
    if (operation === "start") firstStart = false;
    const served = serving.then(() => sleep(slow ? startMs : frameMs)).then(() => {
      if (operation === "start") {
        if (armed) return REFUSED_ACTIVE;
        armed = true;
      }
      if (operation === "stop") armed = false;
      return { ok: true, requestId: 1n, revision: 0n, value: Object.freeze(state()) };
    });
    serving = served;
    return Promise.race([served, sleep(deadlineMs).then(() => { throw new Error("deadline"); })]);
  };
  return { send, sent, armed: () => armed };
}

function realScheduler(milliseconds) {
  return {
    setInterval: (callback) => setInterval(callback, milliseconds),
    clearInterval: (handle) => clearInterval(handle),
  };
}

test("a stuck host holds at most one timed read, and close sends no read and settles within one deadline", { timeout: 5_000 }, async () => {
  const DEADLINE = 50;
  const core = fakeCore({ deadlineMs: DEADLINE, stuckReads: true });
  const owner = new LoudnessOwner({ send: core.send, capabilityFlags: async () => 1n << 17n, scheduler: realScheduler(5) });
  const errors = [];
  const handle = await owner.subscribe({ onError: (error) => errors.push(error) });
  await sleep(300);
  const reads = core.sent.filter((operation) => operation === "read").length;
  assert.ok(reads <= Math.ceil(300 / DEADLINE) + 1, `${reads} timed reads were sent against a host that answers none`);
  // A pump waits behind the one timed read in flight at most, not behind every tick since.
  const pumped = Date.now();
  await assert.rejects(handle.pump(), /deadline/);
  assert.ok(Date.now() - pumped <= 2 * DEADLINE + 40, `a pump waited ${Date.now() - pumped} ms`);
  await sleep(20);
  const before = core.sent.length;
  // A pump queued when close is called is not sent: the stop is the next frame.
  const late = handle.pump();
  const started = Date.now();
  await handle.close();
  const took = Date.now() - started;
  await assert.rejects(late, MisoUsageError);
  assert.deepEqual(core.sent.slice(before), ["stop"], "close sends its stop and nothing else");
  assert.ok(took <= DEADLINE + 40, `close took ${took} ms`);
  await sleep(30);
  assert.deepEqual(core.sent.slice(before), ["stop"], "no read after close");
  // A failed timed read is reported, not swallowed; the readings stop advancing meanwhile.
  assert.ok(errors.length >= 1 && errors.every((error) => /deadline/.test(error.message)), "onError saw the failures");
});

test("a start the sender gave up on is stopped behind it, so the next subscribe is not refused loudness.active", { timeout: 5_000 }, async () => {
  const core = fakeCore({ startMs: 40, deadlineMs: 10 });
  const owner = new LoudnessOwner({ send: core.send, capabilityFlags: async () => 1n << 17n });
  await assert.rejects(owner.subscribe(), /deadline/);
  await sleep(60);
  const handle = await owner.subscribe();
  assert.deepEqual(core.sent, ["start", "stop", "start"]);
  await handle.close();
});

test("a close whose stop fails keeps the handle live: the timer reads again and a later close stops", { timeout: 5_000 }, async () => {
  const core = fakeCore({ failStops: 1 });
  let tick;
  const owner = new LoudnessOwner({
    send: core.send,
    capabilityFlags: async () => 1n << 17n,
    scheduler: { setInterval: (callback) => { tick = callback; return 1; }, clearInterval: () => { tick = undefined; } },
  });
  const handle = await owner.subscribe();
  await assert.rejects(handle.close(), /the port failed/);
  assert.equal(typeof tick, "function", "the failed close restarted the timer");
  tick();
  await sleep(5);
  assert.deepEqual(core.sent, ["start", "stop", "read"]);
  await handle.close();
  assert.equal(core.armed(), false);
  assert.equal(tick, undefined);
});

test("an engine without capability bit 17 is refused before any LOUDNESS frame", { timeout: 5_000 }, async () => {
  const core = fakeCore();
  const owner = new LoudnessOwner({ send: core.send, capabilityFlags: async () => (1n << 16n) | (1n << 18n) });
  await assert.rejects(owner.subscribe(), (error) => {
    assert.ok(error instanceof MisoEngineError, String(error));
    assert.equal(error.code, "unsupported");
    assert.equal(error.diagnosticCode, "sdk.asset.capability");
    return true;
  });
  assert.deepEqual(core.sent, []);
});
