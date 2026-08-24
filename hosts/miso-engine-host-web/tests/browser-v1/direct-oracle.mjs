import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";

const ABI_VERSION = 0x00010000;
const CONFIG_BYTES = 192;
const QUANTUM = 128;
const SAMPLE_RATE = 48000;
const BUFFER_SESSION_TOML = 1;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_OUTPUT_PCM = 5;
const RESOURCE_NAMES = [
  "configBytes", "statusBytes", "sessionTomlBytes", "diagnosticBytes", "sourceIdBytes",
  "sourcePcmStagingBytes", "outputPcmBytes", "bridgeMetadataBytes", "bridgeRetainedBytes",
  "largestBridgeAllocationBytes", "sourceTotalBytes", "sourceOverheadBytes",
  "effectScalarStateBytes", "effectScalarScratchBytes", "builtinRetainedBytes",
  "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes", "graphMetadataBytes",
  "graphDelayBytes", "largestNamedAllocationBytes",
];
const LIMITS32 = [
  CONFIG_BYTES, ABI_VERSION, SAMPLE_RATE, QUANTUM, 1 << 20, 1 << 14, 1 << 10, 8, QUANTUM, 256,
];
const LIMITS64 = [
  1024n, 1024n, 4096n, 8192n, 64n << 20n, 64n << 20n, 16n << 20n,
  16n << 20n, 16n << 20n, 64n << 20n, 16n << 20n, 64n << 20n,
  1024n, 1n << 20n, 16n << 20n,
];

function exactKeys(value, keys, label) {
  assert.deepEqual(Object.keys(value).sort(), [...keys].sort(), label);
}

function status(exports, handle) {
  const pointer = exports.miso_engine_web_v1_status_ptr(handle);
  const view = new DataView(exports.memory.buffer, pointer, 80);
  assert.equal(view.getUint32(0, true), 80);
  assert.equal(view.getUint32(4, true), ABI_VERSION);
  assert.equal(view.getUint32(28, true), 0);
  for (const offset of [48, 56, 64, 72]) assert.equal(view.getBigUint64(offset, true), 0n);
  return {
    state: view.getUint32(8, true),
    lastResult: view.getUint32(12, true),
    backend: view.getUint32(16, true),
    sampleRateHz: view.getUint32(20, true),
    quantumFrames: view.getUint32(24, true),
    nextAbsoluteSample: view.getBigUint64(32, true).toString(),
    renderedQuanta: view.getBigUint64(40, true).toString(),
    memoryBytes: exports.memory.buffer.byteLength,
  };
}

function resources(exports, handle) {
  const pointer = exports.miso_engine_web_v1_resource_ptr(handle);
  const view = new DataView(exports.memory.buffer, pointer, 224);
  assert.equal(view.getUint32(0, true), 224);
  assert.equal(view.getUint32(4, true), ABI_VERSION);
  for (const offset of [20, 24, 28]) assert.equal(view.getUint32(offset, true), 0);
  for (const offset of [192, 200, 208, 216]) assert.equal(view.getBigUint64(offset, true), 0n);
  const report = {
    sampleRateHz: view.getUint32(8, true),
    quantumFrames: view.getUint32(12, true),
    backend: view.getUint32(16, true),
  };
  RESOURCE_NAMES.forEach((name, index) => {
    report[name] = view.getBigUint64(32 + index * 8, true).toString();
  });
  return report;
}

function writeConfig(exports, handle) {
  const pointer = exports.miso_engine_web_v1_config_ptr(handle);
  assert.notEqual(pointer, 0);
  const view = new DataView(exports.memory.buffer, pointer, CONFIG_BYTES);
  LIMITS32.forEach((value, index) => view.setUint32(index * 4, value, true));
  LIMITS64.forEach((value, index) => view.setBigUint64(40 + index * 8, value, true));
  for (let index = 0; index < 4; index += 1) view.setBigUint64(160 + index * 8, 0n, true);
}

function blockPlanes(description) {
  const left = new Float32Array(QUANTUM);
  const right = new Float32Array(QUANTUM);
  for (let index = 0; index < QUANTUM; index += 1) {
    left[index] = description.leftBase + description.leftStep * index;
  }
  return [left, right];
}

function writeSource(exports, handle, sourceId, description) {
  const encoder = new TextEncoder();
  const id = encoder.encode(sourceId);
  const idPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
  const idCapacity = exports.miso_engine_web_v1_buffer_capacity(handle, BUFFER_SOURCE_ID);
  assert.ok(id.byteLength <= idCapacity);
  new Uint8Array(exports.memory.buffer, idPointer, id.byteLength).set(id);
  const pcmPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM);
  const pcm = new Float32Array(exports.memory.buffer, pcmPointer, QUANTUM * 2);
  const planes = blockPlanes(description);
  pcm.set(planes[0], 0);
  pcm.set(planes[1], QUANTUM);
  return exports.miso_engine_web_v1_source_submit(
    handle,
    id.byteLength,
    BigInt(description.generation),
    BigInt(description.startFrame),
    2,
    description.frames,
    description.final ? 1 : 0,
  );
}

function seek(exports, handle, sourceId, generation, sourceFrame) {
  const id = new TextEncoder().encode(sourceId);
  const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
  new Uint8Array(exports.memory.buffer, pointer, id.byteLength).set(id);
  return exports.miso_engine_web_v1_source_seek(
    handle, id.byteLength, BigInt(generation), BigInt(sourceFrame),
  );
}

function render(exports, handle) {
  assert.equal(exports.miso_engine_web_v1_render(handle, QUANTUM), 0);
  const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT_PCM);
  return [
    Array.from(new Float32Array(exports.memory.buffer, pointer, QUANTUM)),
    Array.from(new Float32Array(exports.memory.buffer, pointer + QUANTUM * 4, QUANTUM)),
  ];
}

function pcmSha256(channels) {
  const bytes = Buffer.alloc(channels.length * channels[0].length * 4);
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let offset = 0;
  for (const channel of channels) {
    for (const sample of channel) {
      view.setFloat32(offset, sample, true);
      offset += 4;
    }
  }
  return createHash("sha256").update(bytes).digest("hex");
}

async function runBackend(modulePath, expectedBackend, sessionToml, source) {
  const { instance } = await WebAssembly.instantiate(await readFile(modulePath), {});
  const exports = instance.exports;
  assert.equal(exports.miso_engine_web_v1_abi_version(), ABI_VERSION);
  assert.equal(exports.miso_engine_web_v1_config_bytes(), CONFIG_BYTES);
  const handle = exports.miso_engine_web_v1_config_new();
  assert.notEqual(handle, 0);
  writeConfig(exports, handle);
  assert.equal(exports.miso_engine_web_v1_prepare(handle), 0);
  const tomlPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SESSION_TOML);
  const tomlCapacity = exports.miso_engine_web_v1_buffer_capacity(handle, BUFFER_SESSION_TOML);
  assert.ok(sessionToml.byteLength <= tomlCapacity);
  new Uint8Array(exports.memory.buffer, tomlPointer, sessionToml.byteLength).set(sessionToml);
  assert.equal(exports.miso_engine_web_v1_compile(handle, sessionToml.byteLength), 0);
  const memoryBuffer = exports.memory.buffer;
  const initialStatus = status(exports, handle);
  const resourceReport = resources(exports, handle);
  assert.equal(initialStatus.backend, expectedBackend);
  assert.equal(resourceReport.backend, expectedBackend);

  const first = { ...source.blocks[0], generation: 1 };
  const second = { ...source.blocks[1], generation: 1 };
  const results = {};
  results.first = writeSource(exports, handle, source.sourceId, first);
  results.initialBackpressure = writeSource(exports, handle, source.sourceId, second);
  const blocks = [render(exports, handle)];
  results.seek = seek(exports, handle, source.sourceId, 2, 0);
  results.repeat = writeSource(exports, handle, source.sourceId, { ...first, generation: 2 });
  results.repeatBackpressure = writeSource(
    exports, handle, source.sourceId, { ...second, generation: 2 },
  );
  blocks.push(render(exports, handle));
  results.final = writeSource(exports, handle, source.sourceId, { ...second, generation: 2 });
  blocks.push(render(exports, handle));
  const beforeDisposeStatus = status(exports, handle);
  const pcm = [
    blocks.flatMap((block) => block[0]),
    blocks.flatMap((block) => block[1]),
  ];
  assert.equal(exports.memory.buffer, memoryBuffer);
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
  return {
    initialStatus,
    beforeDisposeStatus,
    resources: resourceReport,
    results,
    pcmF32leSha256: pcmSha256(pcm),
  };
}

async function main() {
  if (process.argv.length !== 4) {
    throw new Error("usage: direct-oracle.mjs ARTIFACT_DIRECTORY EXPECTED_JSON");
  }
  const artifactDirectory = path.resolve(process.argv[2]);
  const fixtureDirectory = path.dirname(new URL(import.meta.url).pathname);
  const source = JSON.parse(await readFile(path.join(fixtureDirectory, "source.json"), "utf8"));
  const expected = JSON.parse(await readFile(process.argv[3], "utf8"));
  exactKeys(source, ["schema", "sourceId", "sampleRateHz", "quantumFrames", "blocks"], "source");
  const sessionToml = await readFile(path.join(fixtureDirectory, "session.toml"));
  // W4-D1: one shipped artifact, so the direct oracle drives it alone. The cross-backend proof
  // moved to two independent places: #83's G5 corpus runs the same kernels natively at
  // Scalar/Simd4/Simd8 and under wasmtime with and without simd128, and `nativePcmF32leSha256`
  // below is this exact session rendered through the native `AudioWorkletEngineHost`. Equality is
  // `to_bits` (SHA-256 over little-endian f32 words), never a tolerance.
  const actual = {
    schema: "miso.web.browser.direct-oracle.v2",
    simd128: await runBackend(
      path.join(artifactDirectory, "miso-engine-v2-audio-worklet.simd128.wasm"),
      1,
      sessionToml,
      source,
    ),
  };
  // #106 F4/E4, the parity gate. The native digest is pinned by
  // `tests::native_identity_session_digest_pins_the_wasm_parity`, which renders this same session
  // and transcript through `AudioWorkletEngineHost` on the host CPU. Equality is `to_bits`
  // (SHA-256 over little-endian f32 words), never a tolerance, and it is asserted *before* any
  // print so `MISO_ENGINE_WEB_ORACLE_PRINT=1` can never mint a pin from a non-identical pair.
  const native = expected.directOracle?.nativePcmF32leSha256;
  assert.equal(typeof native, "string", "expected.json must carry the native digest pin");
  actual.nativePcmF32leSha256 = native;
  assert.equal(
    actual.simd128.pcmF32leSha256,
    native,
    "native and simd128 must render this session to identical bits",
  );
  if (process.env.MISO_ENGINE_WEB_ORACLE_PRINT === "1") {
    process.stdout.write(`${JSON.stringify(actual, null, 2)}\n`);
    return;
  }
  assert.deepEqual(actual, expected.directOracle, "checked raw-Wasm oracle drift");
  console.log("web AudioWorklet independent raw-Wasm oracle passed");
}

await main();
