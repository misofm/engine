import assert from "node:assert/strict";
import { test } from "node:test";
import { BrowserResponsePreview } from "../src/browser/response.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError, MisoUsageError, resultName } from "../src/core/errors.ts";
import { RESPONSE_CAPABILITIES, ResponsePreviewModule } from "../src/core/response.ts";
import { createResponsePreview as createHeadlessResponsePreview } from "../src/headless/response.ts";
import { decodeResponseReply, encodeResponseRequest } from "../src/core/response-frame.ts";
import { effect } from "../src/core/session.ts";
import { moduleBytes } from "./support.mjs";

class FakeWorker {
  listeners = new Map();
  requests = [];
  terminated = 0;
  pending = new Map();

  addEventListener(type, listener) {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(listener);
    this.listeners.set(type, listeners);
  }

  removeEventListener(type, listener) {
    this.listeners.get(type)?.delete(listener);
  }

  postMessage(message) {
    this.requests.push(message);
    if (message.type === "response-init") queueMicrotask(() => this.emit("message", { type: "response-ready" }));
    if (message.type === "response-query") this.pending.set(message.requestId, message);
  }

  reply(requestId) {
    const request = this.pending.get(requestId);
    assert.ok(request);
    this.pending.delete(requestId);
    const frequenciesHz = new Float32Array([20, 24000]);
    const totalLeftDb = new Float32Array([-1, -2]);
    const totalRightDb = new Float32Array([-3, -4]);
    this.emit("message", {
      type: "response-result",
      requestId,
      result: {
        requestedConfiguration: request.query.configuration,
        configurationId: request.query.configurationId,
        sampleRateHz: request.query.sampleRateHz,
        mode: "requestedConfiguration",
        frequenciesHz,
        totalLeftDb,
        totalRightDb,
        sections: [],
        floorDb: -120,
        bypass: false,
        enabledLeft: [],
        enabledRight: [],
        retainedBytes: 64n,
        resultBytes: 40n,
      },
    });
  }

  emit(type, data) {
    for (const listener of this.listeners.get(type) ?? []) {
      listener(type === "message" ? { data } : data);
    }
  }

  terminate() { this.terminated += 1; }
}

const asset = { module: {} };
const query = {
  configurationId: 9_007_199_254_740_993n,
  sampleRateHz: 48_000,
  quantumFrames: 128,
  configuration: { kind: "inputFilters", left: { hpfHz: 80 }, right: { lpfHz: 12_000 } },
  grid: { kind: "linear", points: 2, minimumHz: 20, maximumHz: 24_000 },
  channels: "both",
  fields: "total",
};

test("response capabilities are generated for EQ and input filters", () => {
  assert.ok(RESPONSE_CAPABILITIES.some((row) => row.owner === "effect" && row.target === "miso.parametric-eq"));
  assert.ok(RESPONSE_CAPABILITIES.some((row) => row.owner === "builtins" && row.target === "inputFilters"));
});

test("browser response Worker enforces one request, preserves bigint, and closes idempotently", async () => {
  const worker = new FakeWorker();
  const preview = await BrowserResponsePreview.create({
    asset,
    responseLimits: { requestDeadlineMs: 100 },
    createWorker: () => worker,
  });
  assert.equal(worker.requests[0].type, "response-init");
  const first = preview.query(query);
  const second = preview.query({ ...query, configurationId: 8n });
  await assert.rejects(second, /already in flight/);
  const request = worker.requests.find((entry) => entry.type === "response-query");
  worker.reply(request.requestId);
  const result = await first;
  assert.equal(result.configurationId, query.configurationId);
  assert.deepEqual([...result.frequenciesHz], [20, 24_000]);
  assert.notEqual(result.frequenciesHz, result.totalLeftDb);
  await preview.close();
  await preview.close();
  assert.equal(worker.terminated, 1);
  await assert.rejects(preview.query(query), /closed/);
});

test("browser response Worker timeout is terminal and admits no second job", async () => {
  const worker = new FakeWorker();
  const preview = await BrowserResponsePreview.create({
    asset,
    responseLimits: { requestDeadlineMs: 10 },
    createWorker: () => worker,
  });
  await assert.rejects(preview.query(query), /deadline/);
  assert.equal(worker.requests.filter((entry) => entry.type === "response-query").length, 1);
  assert.equal(worker.terminated, 1);
  await assert.rejects(preview.query(query), /closed/);
  await preview.close();
  assert.equal(worker.terminated, 1);
});

test("browser response Worker failure messages are terminal", async () => {
  for (const [type, event] of [
    ["error", { message: "worker failed" }],
    ["messageerror", {}],
  ]) {
    const worker = new FakeWorker();
    const preview = await BrowserResponsePreview.create({
      asset,
      responseLimits: { requestDeadlineMs: 100 },
      createWorker: () => worker,
    });
    const pending = preview.query(query);
    worker.emit(type, event);
    await assert.rejects(pending);
    assert.equal(worker.requests.filter((entry) => entry.type === "response-query").length, 1);
    assert.equal(worker.terminated, 1);
    await assert.rejects(preview.query(query), /closed/);
    await preview.close();
  }
});

test("candidate Wasm answers EQ and input-filter previews headlessly", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const preview = await createHeadlessResponsePreview({ asset });
  try {
    assert.ok(preview.capabilities.some((row) => row.target === "miso.parametric-eq"));
    const eq = await preview.query({
      configurationId: 9_007_199_254_740_993n,
      sampleRateHz: 48_000,
      quantumFrames: 128,
      configuration: effect("miso.parametric-eq", {
        "band-1-enabled": true,
        "band-1-kind": "bell",
        "band-1-frequency": 1_000,
        "band-1-gain": 3,
        "band-1-q": 1,
      }),
      grid: { kind: "logarithmic", points: 16, minimumHz: 20, maximumHz: 20_000 },
      channels: "both",
      fields: "totalAndSections",
    });
    assert.equal(eq.configurationId, 9_007_199_254_740_993n);
    assert.deepEqual(eq.sections.map((section) => section.id), [1, 2, 3, 4, 5, 6]);
    assert.deepEqual(eq.sections.map((section) => section.name), ["Band 1", "Band 2", "Band 3", "Band 4", "HPF", "LPF"]);
    assert.deepEqual(eq.enabledLeft, [true, false, false, false, false, false]);
    assert.deepEqual(eq.enabledRight, [true, false, false, false, false, false]);
    assert.equal(eq.frequenciesHz.length, 16);
    assert.ok(eq.totalLeftDb?.some((value) => value > 0));
    const filters = await preview.query({
      ...query,
      configurationId: 9_007_199_254_740_994n,
      configuration: { kind: "inputFilters", left: { hpfHz: 80 }, right: { lpfHz: 12_000 } },
      grid: { kind: "linear", points: 8, minimumHz: 0, maximumHz: 24_000 },
      fields: "totalAndSections",
    });
    assert.equal(filters.sections.length, 2);
    assert.equal(filters.frequenciesHz[0], 0);
    assert.ok(filters.totalLeftDb?.[0] !== filters.totalLeftDb?.[filters.totalLeftDb.length - 1]);

    for (const [name, invalid] of [
      ["channels", { ...query, channels: "bogus" }],
      ["fields", { ...query, fields: "bogus" }],
      ["grid", { ...query, grid: { ...query.grid, kind: "bogus" } }],
    ]) {
      assert.throws(() => preview.query(invalid), MisoUsageError, `${name} selector must refuse`);
    }

    const boundedInstance = await asset.instantiate();
    const bounded = new ResponsePreviewModule(boundedInstance, { maximumResultBytes: 16 });
    const memoryBefore = boundedInstance.exports.memory.buffer.byteLength;
    assert.throws(
      () => bounded.query({
        ...query,
        grid: { ...query.grid, points: 2 },
      }),
      (error) => error instanceof MisoEngineError && error.code === "response.reply_too_large"
        && resultName(error.result, "call") === "refusedBudget",
    );
    const memoryAfterTiny = boundedInstance.exports.memory.buffer.byteLength;
    assert.throws(
      () => bounded.query({
        ...query,
        grid: { ...query.grid, points: 250_000 },
      }),
      (error) => error instanceof MisoEngineError && error.code === "response.points"
        && resultName(error.result, "call") === "invalidArgument",
    );
    assert.ok(memoryAfterTiny >= memoryBefore);
    assert.equal(boundedInstance.exports.memory.buffer.byteLength, memoryAfterTiny);
    bounded.close();
  } finally {
    await preview.close();
  }
});

test("candidate Wasm: a per-lane value given as both equals the same value on each lane", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const preview = await createHeadlessResponsePreview({ asset });
  try {
    const curves = (gain) => preview.query({
      ...query,
      configuration: effect("miso.parametric-eq", {
        "band-1-enabled": true,
        "band-1-kind": "bell",
        "band-1-frequency": 1_000,
        "band-1-gain": gain,
        "band-1-q": 1,
      }),
      grid: { kind: "logarithmic", points: 32, minimumHz: 20, maximumHz: 20_000 },
      fields: "totalAndSections",
    });
    const both = curves(4);
    const lanes = curves([4, 4]);
    assert.ok(both.totalLeftDb.some((value) => value > 0), "the gain shows in the curve");
    assert.deepEqual(both.totalLeftDb, lanes.totalLeftDb);
    assert.deepEqual(both.totalRightDb, lanes.totalRightDb);
    assert.deepEqual(both.totalLeftDb, both.totalRightDb);
  } finally {
    await preview.close();
  }
});

test("candidate Wasm: the engine refuses 4097 points with its own code", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const preview = await createHeadlessResponsePreview({ asset });
  try {
    assert.throws(
      () => preview.query({ ...query, grid: { ...query.grid, points: 4097 } }),
      (error) => error instanceof MisoEngineError && error.code === "response.points"
        && resultName(error.result, "call") === "invalidArgument",
    );
    const accepted = preview.query({ ...query, grid: { ...query.grid, points: 4096 } });
    assert.equal(accepted.frequenciesHz.length, 4096);
  } finally {
    await preview.close();
  }
});

/** A configuration the way a caller who skips `effect()` writes it: no catalog check has run. */
function forged(effectId, parameters, options = {}) {
  return {
    effectId,
    parameters,
    options: { bypass: false, quality: "normal", linkMode: "dual_mono", channel: "both", ...options },
  };
}

/**
 * Send one raw request frame through the module's session-less entry, as a client with no codec of
 * the core's would, and read the reply.
 */
function submitFrame(instance, frame) {
  const exports = instance.exports;
  const pointer = exports.miso_engine_web_v1_response_frame_ptr(frame.length);
  assert.notEqual(pointer, 0);
  new Uint8Array(exports.memory.buffer, pointer, frame.length).set(frame);
  assert.equal(exports.miso_engine_web_v1_response_frame_submit(frame.length), 0);
  const replyPointer = exports.miso_engine_web_v1_response_frame_reply_ptr();
  const replyBytes = exports.miso_engine_web_v1_response_frame_reply_bytes();
  return decodeResponseReply(new Uint8Array(exports.memory.buffer.slice(replyPointer, replyPointer + replyBytes)));
}

/**
 * #1623 gate 1: the SDK checks nothing the core checks. Each row is a request the SDK used to
 * refuse itself with a `MisoUsageError`; it now reaches the core, and the refusal is the core's
 * own, with the dotted code and the result class the C ABI gives the same request
 * (`crates/capi/tests/response_get.rs` and `crates/host-core/tests/response_frame.rs` hold those
 * rows to the same codes and statuses).
 */
test("candidate Wasm: a grid, a rate, a quality and a value the SDK used to refuse are the core's refusals", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const preview = await createHeadlessResponsePreview({ asset });
  const eq = (parameters = {}, options = {}) => forged("miso.parametric-eq", parameters, options);
  const grid = (override) => ({ ...query.grid, kind: "logarithmic", points: 8, minimumHz: 20, maximumHz: 20_000, ...override });
  const rows = [
    ["a logarithmic axis from zero", { grid: grid({ minimumHz: 0 }) }, "response.grid", "invalidArgument"],
    ["a negative logarithmic minimum", { grid: grid({ minimumHz: -5 }) }, "response.grid", "invalidArgument"],
    ["a NaN endpoint", { grid: grid({ maximumHz: Number.NaN }) }, "protocol.failure", "invalidArgument"],
    ["an infinite endpoint", { grid: grid({ kind: "linear", maximumHz: Number.POSITIVE_INFINITY }) }, "protocol.failure", "invalidArgument"],
    ["one point", { grid: grid({ points: 1 }) }, "response.points", "invalidArgument"],
    ["a sample rate that is no launch rate", { sampleRateHz: 12_345 }, "response.rate", "invalidArgument"],
    ["a zero quantum", { quantumFrames: 0 }, "response.rate", "invalidArgument"],
    ["draft quality", { configuration: eq({}, { quality: "draft" }) }, "effect.quality.unsupported", "invalidArgument"],
    ["high quality", { configuration: eq({}, { quality: "high" }) }, "effect.quality.unsupported", "invalidArgument"],
    ["a non-finite value", { configuration: eq({ "band-1-gain": Number.NaN }) }, "protocol.failure", "invalidArgument"],
    ["a gain outside its domain", { configuration: eq({ "band-1-gain": 500 }) }, "effect.parameter.initial", "invalidArgument"],
    ["an effect the engine does not know", { configuration: forged("miso.no-such-effect", {}) }, "response.effect.unknown", "invalidArgument"],
    ["an effect with no response", { configuration: forged("miso.compressor", {}) }, "response.effect.unsupported", "unsupported"],
    [
      "a shared parameter on one lane, of an effect with no response",
      { configuration: forged("miso.compressor", { threshold: -20 }, { channel: "left" }) },
      "response.effect.unsupported",
      "unsupported",
    ],
  ];
  try {
    for (const [name, change, code, result] of rows) {
      assert.throws(
        () => preview.query({ ...query, ...change }),
        (error) => error instanceof MisoEngineError && !(error instanceof MisoUsageError)
          && error.code === code && resultName(error.result, "call") === result,
        name,
      );
    }
    // The same request with a left-only scalar lane is accepted: the core, not the SDK, says
    // which lane an effect takes.
    const left = preview.query({
      ...query,
      configuration: eq({ "band-1-gain": 3 }, { channel: "left" }),
      grid: grid({}),
    });
    assert.equal(left.frequenciesHz.length, 8);
  } finally {
    await preview.close();
  }
});

test("candidate Wasm: a lane given twice is refused by the core with the C ABI's code", {
  skip: !process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX,
}, async () => {
  const asset = await MisoEngineAsset.load(await moduleBytes());
  const instance = await asset.instantiate();
  const frame = (overrides) => encodeResponseRequest({
    requestId: 1n,
    target: { kind: "effect", effect: "miso.parametric-eq", quality: "normal", linkMode: "dual_mono" },
    grid: "linear",
    points: 8,
    minimumHz: 20,
    maximumHz: 20_000,
    channels: "both",
    sections: false,
    overrides,
    configurationId: 1n,
    sampleRateHz: 48_000,
    quantumFrames: 128,
  });
  for (const overrides of [
    [{ control: 4, lane: "left", value: 1 }, { control: 4, lane: "left", value: 2 }],
    [{ control: 4, lane: "both", value: 1 }, { control: 4, lane: "left", value: 2 }],
  ]) {
    const reply = submitFrame(instance, frame(overrides));
    assert.equal(reply.ok, false);
    assert.equal(reply.status, "invalid_field");
    assert.deepEqual(reply.diagnostics.map((row) => row.code), ["response.override.duplicate"]);
  }
  // A limit left out of the request is a field left out of the frame, for the core's default.
  const accepted = submitFrame(instance, frame([{ control: 4, lane: "both", value: 3 }]));
  assert.equal(accepted.ok, true);
  assert.equal(accepted.curves.frequenciesHz.length, 8);
});

test("a name or a number with no wire spelling stays a usage error before any byte is made", () => {
  const request = {
    requestId: 1n,
    target: { kind: "inputFilters" },
    grid: "linear",
    points: 8,
    minimumHz: 20,
    maximumHz: 20_000,
    channels: "both",
    sections: false,
    overrides: [],
    configurationId: 1n,
    sampleRateHz: 48_000,
    quantumFrames: 128,
  };
  assert.ok(encodeResponseRequest(request) instanceof Uint8Array);
  for (const [name, change] of [
    ["a grid with no code", { grid: "bogus" }],
    ["a lane with no code", { channels: "middle" }],
    ["an override lane with no code", { overrides: [{ control: 1, lane: "middle", value: 0 }] }],
    ["a quality with no code", { target: { kind: "effect", effect: "e", quality: "ultra", linkMode: "dual_mono" } }],
    ["a link mode with no code", { target: { kind: "effect", effect: "e", quality: "normal", linkMode: "sum" } }],
    ["a point count that is no u32", { points: 2 ** 32 }],
    ["a fractional rate", { sampleRateHz: 48_000.5 }],
    ["a configuration ID that is not a bigint", { configurationId: 1 }],
  ]) {
    assert.throws(() => encodeResponseRequest({ ...request, ...change }), MisoUsageError, name);
  }
});
