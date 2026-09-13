import assert from "node:assert/strict";
import { test } from "node:test";
import { BrowserResponsePreview } from "../src/browser/response.ts";
import { RESPONSE_CAPABILITIES } from "../src/core/response.ts";

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
    limits: { requestDeadlineMs: 100 },
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
