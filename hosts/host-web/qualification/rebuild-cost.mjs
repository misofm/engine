// Issue #1289 (slice B1 of #1269), decision D2: one Chromium point for the browser rebuild cost.
//
// `scripts/web-mixing-automation-benchmark.mjs rebuild-*` times `miso_engine_web_v1_boot` under
// Node's V8. This checks that proxy against the real place a replacement will run: an
// `AudioWorkletProcessor` on Chromium's rendering thread. It is harness code only. A test
// processor (not the shipped one, whose boot carries no timer) receives the shipped module, compiled
// on the page, and the sixty-four-track app-shape document through `processorOptions`, and inside
// two consecutive `process()` calls boots it on a fresh instance each time, with the browser's
// live-control boot options (the SDK's default command queue, nothing else attached). The first boot
// is the first run of the module's code in this worklet; the second is not. `AudioWorkletGlobalScope`
// exposes no `performance` in Chromium, so the clock is `Date.now()` and its resolution is 1 ms.
// After each timed boot, outside the clock, the processor feeds the source, renders a block and
// requires it audible, then disposes the session.
//
// usage: node rebuild-cost.mjs MODULE.wasm OUT.json
//
// The arguments, the documents and the output are checked before the browser is launched, and the
// output is created exclusively: an existing OUT.json is refused, never overwritten.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, "../../..");
const FIXTURE_ID = "fixtures/session/v1/console-sixty-four-track-app.json";
const RATE = 48000;
const Q = 128;
const QUANTUM_BUDGET_NS = (Q * 1e9) / RATE;
const BOOTS = 2;

if (process.argv.length !== 4) {
  process.stderr.write("usage: rebuild-cost.mjs MODULE.wasm OUT.json\n");
  process.exit(2);
}
const [modulePath, outPath] = process.argv.slice(2).map((argument) => path.resolve(argument));
let present = true;
try { lstatSync(outPath); } catch { present = false; }
if (present) {
  process.stderr.write(`refusing to overwrite ${outPath}\n`);
  process.exit(1);
}
lstatSync(path.dirname(outPath));

const ABI = JSON.parse(readFileSync(path.join(ROOT, "sdk/assets/miso-engine-v1-abi-layout.json")));
const moduleBytes = readFileSync(modulePath);
const documentBytes = readFileSync(path.join(ROOT, FIXTURE_ID));
const document = JSON.parse(documentBytes);
assert.equal(document.tracks.length, 64);
assert.equal(document.sources.length, 1);
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const bootField = (name) => ABI.structures.bootOptions.fields.find((row) => row.name === name).offset;
const constant = (group, name) => ABI.constants[group].find((item) => item.name === name).value;
const { stallToleranceMs, reserveQuanta } = ABI.constants.sourceRing;
const layout = {
  abiVersion: ABI.abiVersion,
  bootOptionsBytes: ABI.structures.bootOptions.bytes,
  offsets: Object.fromEntries(["structSize", "abiVersion", "requireSampleRateHz", "requireQuantumFrames",
    "sourceRingFrames", "reserved0", "maximumMemoryBytes", "liveControlCommandQueueRecords",
    "liveControlMeterBlocks", "liveControlObservationTaps", "liveControlMasterTrackPlusOne",
  ].map((name) => [name, bootField(name)])),
  sourceRingFrames: (Math.ceil(Math.floor((RATE * stallToleranceMs) / 1000) / Q) + reserveQuanta) * Q,
  commandQueueRecords: ABI.constants.defaultCommandQueueRecords,
  ok: constant("resultCodes", "ok"),
  bufferSourceId: constant("bufferKinds", "sourceId"),
  bufferSourcePcm: constant("bufferKinds", "sourcePcm"),
  bufferOutput: constant("bufferKinds", "outputPcm"),
  // `AudioWorkletGlobalScope` has no `TextEncoder`, so the id travels as bytes.
  sourceId: [...new TextEncoder().encode(document.sources[0].id)],
  rate: RATE,
  quantum: Q,
  boots: BOOTS,
};

// The test processor. Each of the first BOOTS `process()` calls performs one timed boot.
const PROCESSOR = `
registerProcessor("rebuild-cost", class extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.o = options.processorOptions;
    this.results = [];
  }
  bootOnce() {
    const o = this.o;
    const L = o.layout;
    const e = new WebAssembly.Instance(o.module, {}).exports;
    // Every pointer is read before \`memory.buffer\`: an export that grows memory detaches it.
    const optionsPointer = e.miso_engine_web_v1_boot_options_ptr();
    const view = new DataView(e.memory.buffer, optionsPointer, L.bootOptionsBytes);
    const u32 = (name, value) => view.setUint32(L.offsets[name], value, true);
    const u64 = (name, value) => view.setBigUint64(L.offsets[name], value, true);
    u32("structSize", L.bootOptionsBytes); u32("abiVersion", L.abiVersion);
    u32("requireSampleRateHz", L.rate); u32("requireQuantumFrames", L.quantum);
    u32("sourceRingFrames", L.sourceRingFrames); u32("reserved0", 0);
    u64("maximumMemoryBytes", 0n);
    u64("liveControlCommandQueueRecords", BigInt(L.commandQueueRecords));
    u64("liveControlMeterBlocks", 0n); u64("liveControlObservationTaps", 0n);
    u64("liveControlMasterTrackPlusOne", 0n);
    const doc = o.document;
    const pointer = e.miso_engine_web_v1_document_ptr(doc.byteLength);
    new Uint8Array(e.memory.buffer, pointer, doc.byteLength).set(doc);
    const start = Date.now();
    const handle = e.miso_engine_web_v1_boot(doc.byteLength);
    const bootMs = Date.now() - start;
    const result = { boot_ms: bootMs, boot_result: e.miso_engine_web_v1_boot_result(), booted: handle !== 0,
      audible: false, disposed: false, memory_bytes: e.memory.buffer.byteLength };
    if (handle === 0) return result;
    const id = Uint8Array.from(L.sourceId);
    let frame = 0n;
    for (let block = 0; block < 12; block++) {
      const idPointer = e.miso_engine_web_v1_buffer_ptr(handle, L.bufferSourceId);
      const pcmPointer = e.miso_engine_web_v1_buffer_ptr(handle, L.bufferSourcePcm);
      new Uint8Array(e.memory.buffer, idPointer, id.length).set(id);
      const pcm = new Float32Array(e.memory.buffer, pcmPointer, 2 * L.quantum);
      for (let i = 0; i < L.quantum; i++) {
        const t = (Number(frame) + i) * 0.0575;
        pcm[i] = Math.sin(t) * 0.25;
        pcm[L.quantum + i] = -Math.sin(t) * 0.2;
      }
      if (e.miso_engine_web_v1_source_submit(handle, id.length, 1n, frame, 2, L.quantum, 0) !== L.ok) return result;
      frame += BigInt(L.quantum);
      if (block < 4) continue;
      if (e.miso_engine_web_v1_render(handle, L.quantum) !== L.ok) return result;
      const outPointer = e.miso_engine_web_v1_buffer_ptr(handle, L.bufferOutput);
      const out = new Float32Array(e.memory.buffer, outPointer, 2 * L.quantum);
      result.audible ||= out.some((word) => word !== 0 && Number.isFinite(word));
    }
    result.disposed = e.miso_engine_web_v1_dispose(handle) === L.ok;
    return result;
  }
  process() {
    if (this.results.length < this.o.layout.boots) {
      try {
        this.results.push(this.bootOnce());
      } catch (error) {
        this.port.postMessage({ error: String(error) });
        return false;
      }
      if (this.results.length === this.o.layout.boots) this.port.postMessage({ results: this.results });
    }
    return true;
  }
});
`;

const server = createServer((request, response) => {
  if (request.url === "/") {
    response.setHeader("content-type", "text/html; charset=utf-8");
    response.end("<!doctype html><title>rebuild cost</title>");
  } else if (request.url === "/module.wasm") {
    response.setHeader("content-type", "application/wasm");
    response.end(moduleBytes);
  } else if (request.url === "/document.json") {
    response.setHeader("content-type", "application/json");
    response.end(documentBytes);
  } else if (request.url === "/processor.js") {
    response.setHeader("content-type", "text/javascript; charset=utf-8");
    response.end(PROCESSOR);
  } else {
    response.statusCode = 404;
    response.end();
  }
}).listen(0, "127.0.0.1");
await new Promise((resolve) => server.once("listening", resolve));

const browser = await chromium.launch({ args: ["--autoplay-policy=no-user-gesture-required", "--disable-dev-shm-usage"] });
let outcome;
try {
  const page = await browser.newPage();
  await page.goto(`http://localhost:${server.address().port}/`);
  outcome = await page.evaluate(async (pageLayout) => {
    const module = await WebAssembly.compile(await (await fetch("/module.wasm")).arrayBuffer());
    const documentArray = new Uint8Array(await (await fetch("/document.json")).arrayBuffer());
    const context = new AudioContext({ sampleRate: pageLayout.rate });
    await context.audioWorklet.addModule("/processor.js");
    const node = new AudioWorkletNode(context, "rebuild-cost", {
      numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2],
      processorOptions: { module, document: documentArray, layout: pageLayout },
    });
    node.connect(context.destination);
    await context.resume();
    const reply = await new Promise((resolve, reject) => {
      node.port.onmessage = (event) => resolve(event.data);
      setTimeout(() => reject(new Error("the worklet did not answer in 60 s")), 60000);
    });
    const state = { reply, sampleRate: context.sampleRate, baseLatency: context.baseLatency,
      outputLatency: context.outputLatency, userAgent: navigator.userAgent };
    await context.close();
    return state;
  }, layout);
} finally {
  await browser.close();
  server.close();
}

const { reply } = outcome;
if (reply.error) throw new Error(`worklet: ${reply.error}`);
assert.equal(reply.results.length, BOOTS);
for (const [index, row] of reply.results.entries()) {
  assert.ok(row.booted, `boot ${index + 1} refused with result ${row.boot_result}`);
  assert.ok(row.audible, `boot ${index + 1} rendered no audible block`);
  assert.ok(row.disposed, `boot ${index + 1}: dispose failed`);
}
const record = {
  schema_version: 1,
  issue: 1289,
  record: "web_rebuild_cost_chromium",
  workload_kind: "sixty_four_track_app_shape",
  fixture_id: FIXTURE_ID,
  document_sha256: sha256(documentBytes),
  module_sha256: sha256(moduleBytes),
  browser: "chromium",
  user_agent: outcome.userAgent,
  context_sample_rate_hz: outcome.sampleRate,
  context_base_latency_s: outcome.baseLatency,
  context_output_latency_s: outcome.outputLatency,
  quantum_frames: Q,
  quantum_budget_ns: QUANTUM_BUDGET_NS,
  clock: "Date.now() inside AudioWorkletGlobalScope, 1 ms resolution",
  where: "a test AudioWorkletProcessor's process(), Chromium rendering thread, a fresh instance per boot",
  boots: reply.results.map((row, index) => ({
    boot: index + 1,
    first_run_of_module_code: index === 0,
    ...row,
  })),
  descriptive_only: true,
};
writeFileSync(outPath, `${JSON.stringify(record)}\n`, { flag: "wx" });
process.stdout.write(`${outPath}\n`);
