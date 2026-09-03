import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";

const ABI_VERSION = 0x00010000;
const BOOT_OPTIONS_BYTES = 64;
const QUANTUM = 128;
const SAMPLE_RATE = 48000;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_OUTPUT_PCM = 5;
// Issue #137 D1: the live-console command path.
const BUFFER_COMMAND = 6;
const COMMAND_RECORD_BYTES = 48;
const COMMAND_QUEUE_RECORDS = 4;
const COMMAND_PAN = 1;
const COMMAND_MATRIX = 2;
// Issue #140: every declared kind is live now.
const COMMAND_FADER_DB = 3;
const COMMAND_MUTE = 4;
const COMMAND_EFFECT_PARAM = 5;
const COMMAND_EFFECT_BYPASS = 6;
// Issue #143: the observation subscribe/unsubscribe kinds and the meter frame's shape.
const COMMAND_OBSERVE_SUBSCRIBE = 7;
const COMMAND_OBSERVE_UNSUBSCRIBE = 8;
const BUFFER_METER_FRAME = 7;
const METER_HEADER_BYTES = 64;
const OBSERVATION_WINDOW_BLOCKS = 2;
const RESOURCE_NAMES = [
  "optionsBytes", "statusBytes", "sessionDocumentBytes", "diagnosticBytes", "sourceIdBytes",
  "sourcePcmStagingBytes", "outputPcmBytes", "bridgeMetadataBytes", "bridgeRetainedBytes",
  "largestBridgeAllocationBytes", "sourceTotalBytes", "sourceOverheadBytes",
  "effectScalarStateBytes", "effectScalarScratchBytes", "builtinRetainedBytes",
  "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes", "graphMetadataBytes",
  "graphDelayBytes", "largestNamedAllocationBytes",
  // Issue #143: carved from the report's first reserved word. Zero on this transcript, which
  // asks for no observation capacity at all.
  "observationRetainedBytes",
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
  // Issue #143 carved `observationRetainedBytes` from the first of the report's four reserved
  // words; the other three are still required zero and the 224-byte layout is unchanged.
  for (const offset of [200, 208, 216]) assert.equal(view.getBigUint64(offset, true), 0n);
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

function boot(exports, document, consoleWords = [0n, 0n, 0n, 0n]) {
  const pointer = exports.miso_engine_web_v1_boot_options_ptr();
  assert.notEqual(pointer, 0);
  const view = new DataView(exports.memory.buffer, pointer, BOOT_OPTIONS_BYTES);
  view.setUint32(0, BOOT_OPTIONS_BYTES, true);
  view.setUint32(4, ABI_VERSION, true);
  view.setUint32(8, SAMPLE_RATE, true);
  view.setUint32(12, QUANTUM, true);
  // The direct oracle deliberately pins one quantum so its backpressure transcript remains the
  // byte-identical boot-equivalence instrument specified by #240 S6.
  view.setUint32(16, QUANTUM, true);
  view.setUint32(20, 0, true);
  view.setBigUint64(24, 0n, true);
  consoleWords.forEach((value, index) => view.setBigUint64(32 + index * 8, value, true));
  const documentPointer = exports.miso_engine_web_v1_document_ptr(document.byteLength);
  assert.notEqual(documentPointer, 0);
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);
  const handle = exports.miso_engine_web_v1_boot(document.byteLength);
  assert.notEqual(handle, 0, `boot refused with ${exports.miso_engine_web_v1_boot_result()}`);
  return handle;
}

/// Stage one 48-byte command record and submit the batch, returning the typed report.
function submitCommands(exports, handle, records) {
  const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_COMMAND);
  assert.notEqual(pointer, 0);
  const staging = new DataView(exports.memory.buffer, pointer, records.length * COMMAND_RECORD_BYTES);
  for (let index = 0; index < records.length; index += 1) {
    const record = records[index];
    const offset = index * COMMAND_RECORD_BYTES;
    for (let byte = 0; byte < COMMAND_RECORD_BYTES; byte += 1) staging.setUint8(offset + byte, 0);
    staging.setUint8(offset, record.kind);
    staging.setUint8(offset + 1, record.rack ?? 255);
    staging.setUint8(offset + 2, record.channel ?? 255);
    staging.setUint32(offset + 4, record.trackIndex, true);
    staging.setUint32(offset + 8, record.effectIndex ?? 0, true);
    staging.setUint32(offset + 12, record.parameterId ?? 0, true);
    staging.setUint32(offset + 16, record.smoothingSamples ?? 0, true);
    for (let slot = 0; slot < 4; slot += 1) {
      staging.setFloat32(offset + 24 + slot * 4, record.values[slot], true);
    }
  }
  const result = exports.miso_engine_web_v1_command_submit(handle, records.length);
  const report = new DataView(
    exports.memory.buffer,
    exports.miso_engine_web_v1_command_report_ptr(handle),
    48,
  );
  assert.equal(report.getUint32(0, true), 48);
  assert.equal(report.getUint32(4, true), ABI_VERSION);
  for (const offset of [32, 40]) assert.equal(report.getBigUint64(offset, true), 0n);
  return {
    result,
    reason: report.getUint32(12, true),
    rejectedIndex: report.getUint32(16, true),
    admitted: report.getUint32(20, true),
    appliedAtSample: report.getBigUint64(24, true).toString(),
  };
}

function blockPlanes(description) {
  const left = new Float32Array(QUANTUM);
  const right = new Float32Array(QUANTUM);
  for (let index = 0; index < QUANTUM; index += 1) {
    left[index] = description.leftBase + description.leftStep * index;
  }
  return [left, right];
}

/// Issue #207: read one canonical source ID back out of the staging buffer the ABI copies it into.
///
/// The ID is `[a-z][a-z0-9._-]{0,126}` by the session schema, so every byte is ASCII and the
/// decode is a loop rather than a `TextDecoder` -- the same reasoning the worklet's own reader
/// carries, held here so both sides decode identically.
function readSourceId(exports, handle, index) {
  const length = exports.miso_engine_web_v1_source_id(handle, index);
  assert.ok(length > 0, "a declared source has a nonempty ID");
  const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
  const bytes = new Uint8Array(exports.memory.buffer, pointer, length);
  let id = "";
  for (let byte = 0; byte < length; byte += 1) {
    assert.ok(bytes[byte] <= 0x7f, "source IDs are ASCII by the session schema");
    id += String.fromCharCode(bytes[byte]);
  }
  return id;
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

async function runBackend(modulePath, expectedBackend, sessionDocument, source) {
  const { instance } = await WebAssembly.instantiate(await readFile(modulePath), {});
  const exports = instance.exports;
  assert.equal(exports.miso_engine_web_v1_abi_version(), ABI_VERSION);
  const handle = boot(exports, sessionDocument);
  const memoryBuffer = exports.memory.buffer;
  const initialStatus = status(exports, handle);
  const resourceReport = resources(exports, handle);
  assert.equal(initialStatus.backend, expectedBackend);
  assert.equal(resourceReport.backend, expectedBackend);

  // Issue #207: source introspection, against the real module and the real compiled session -- the
  // leg the JS suite's fake exports cannot supply. `session.json` declares one source, and the
  // transcript this oracle replays addresses it by the *same* ID the engine reports here: the
  // assertion is that a driver holding only the compiled session can find the source it must feed.
  assert.equal(exports.miso_engine_web_v1_source_count(handle), 1);
  assert.equal(readSourceId(exports, handle, 0), source.sourceId);
  assert.equal(exports.miso_engine_web_v1_source_channels(handle, 0), 2);
  assert.equal(exports.miso_engine_web_v1_source_frames(handle, 0), 256n);
  // Out of range answers the zero sentinel everywhere.
  assert.equal(exports.miso_engine_web_v1_source_id(handle, 1), 0);
  assert.equal(exports.miso_engine_web_v1_source_channels(handle, 1), 0);
  assert.equal(exports.miso_engine_web_v1_source_frames(handle, 1), 0n);

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

/// #137 E2, extended by #140 C: the same session, source feed and command timeline the native
/// twin runs, rendered through the shipped artifact.
///
/// Its digest is asserted equal to the native twin's before anything is printed, so the pin can
/// only ever be the value two independent implementations already agree on.
///
/// The fixture is identity apart from one dynamic-rack parametric EQ whose band 1 is a low shelf,
/// so a constant input renders to that same constant until a command moves it. Ten blocks; one
/// matrix retarget, three refusals, a smoothed pan, a windowed fader move, a windowed mute, an
/// effect parameter on both lanes, a live bypass, and one batch that releases the mute and the
/// bypass together across two different destination queues. The digest is a statement about
/// *when* each of those took effect, not merely that they did.
async function runCommandTimeline(modulePath, sessionDocument, sourceId) {
  const { instance } = await WebAssembly.instantiate(await readFile(modulePath), {});
  const exports = instance.exports;
  const handle = boot(
    exports,
    sessionDocument,
    [BigInt(COMMAND_QUEUE_RECORDS), 0n, 0n, 0n],
  );
  assert.equal(exports.miso_engine_web_v1_console_track_count(handle), 1);

  const memoryBuffer = exports.memory.buffer;
  const feed = (block) => {
    const id = new TextEncoder().encode(sourceId);
    const idPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
    new Uint8Array(exports.memory.buffer, idPointer, id.byteLength).set(id);
    const pcmPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM);
    const pcm = new Float32Array(exports.memory.buffer, pcmPointer, QUANTUM * 2);
    pcm.fill(0.25);
    return exports.miso_engine_web_v1_source_submit(
      handle, id.byteLength, 1n, BigInt(block * QUANTUM), 2, QUANTUM, 0,
    );
  };
  const matrix = { kind: COMMAND_MATRIX, trackIndex: 0, values: [0.5, 0, 0, 1] };
  const pan = { kind: COMMAND_PAN, trackIndex: 0, smoothingSamples: QUANTUM, values: [-1, 1, 0, 0] };
  const fader = {
    kind: COMMAND_FADER_DB, rack: 255, channel: 2, trackIndex: 0,
    smoothingSamples: QUANTUM, values: [-6, 0, 0, 0],
  };
  const muteOn = {
    kind: COMMAND_MUTE, rack: 255, channel: 2, trackIndex: 0,
    smoothingSamples: QUANTUM, values: [1, 0, 0, 0],
  };
  const muteOff = {
    kind: COMMAND_MUTE, rack: 255, channel: 2, trackIndex: 0, values: [0, 0, 0, 0],
  };
  const bandGain = {
    kind: COMMAND_EFFECT_PARAM, rack: 1, channel: 2, trackIndex: 0,
    effectIndex: 0, parameterId: 4, values: [-12, 0, 0, 0],
  };
  const unknownParameter = { ...bandGain, parameterId: 4242, values: [0, 0, 0, 0] };
  const bypassOn = {
    kind: COMMAND_EFFECT_BYPASS, rack: 1, channel: 255, trackIndex: 0,
    effectIndex: 0, values: [1, 0, 0, 0],
  };
  const bypassOff = { ...bypassOn, values: [0, 0, 0, 0] };
  const reports = {};
  const blocks = [];
  const step = (block) => {
    assert.equal(feed(block), 0);
    blocks.push(render(exports, handle));
  };

  step(0);
  reports.matrix = submitCommands(exports, handle, [matrix]);
  step(1);
  reports.unknownTrack = submitCommands(
    exports, handle, [{ ...matrix, trackIndex: 5 }],
  );
  reports.flood = submitCommands(
    exports, handle, Array.from({ length: COMMAND_QUEUE_RECORDS + 1 }, () => matrix),
  );
  reports.unknownParameter = submitCommands(exports, handle, [unknownParameter]);
  step(2);
  reports.pan = submitCommands(exports, handle, [pan]);
  step(3);
  reports.fader = submitCommands(exports, handle, [fader]);
  step(4);
  reports.mute = submitCommands(exports, handle, [muteOn]);
  step(5);
  reports.effectParam = submitCommands(exports, handle, [bandGain]);
  step(6);
  reports.effectBypass = submitCommands(exports, handle, [bypassOn]);
  step(7);
  reports.mixedBatch = submitCommands(exports, handle, [muteOff, bypassOff]);
  step(8);
  step(9);
  const beforeDisposeStatus = status(exports, handle);
  const pcm = [blocks.flatMap((block) => block[0]), blocks.flatMap((block) => block[1])];
  assert.equal(exports.memory.buffer, memoryBuffer, "a command timeline never grows memory");
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
  return { reports, beforeDisposeStatus, pcmF32leSha256: pcmSha256(pcm) };
}

/// Issue #143 E1/E4/E12: the observation timeline, through the shipped artifact.
///
/// One track, one compressor, everything else identity. The timeline is deliberately the same
/// eleven blocks whether or not a tap is armed, so the two legs' PCM digests must be identical --
/// that is E1 on the browser, and it is checked here rather than asserted about.
///
/// `taps` of `0n` is the level-1 zero leg: no lane, no accumulator, no conflating cell, and
/// `observationRetainedBytes` is `0`. The subscription it still sends is refused with the typed
/// `observationUnbound` reason rather than silently ignored.
async function runObservationTimeline(modulePath, sessionDocument, sourceId, taps) {
  const { instance } = await WebAssembly.instantiate(await readFile(modulePath), {});
  const exports = instance.exports;
  const handle = boot(exports, sessionDocument, [
    BigInt(COMMAND_QUEUE_RECORDS), BigInt(OBSERVATION_WINDOW_BLOCKS), taps, taps === 0n ? 0n : 1n,
  ]);
  assert.equal(exports.miso_engine_web_v1_console_track_count(handle), 1);
  const resourceReport = resources(exports, handle);
  assert.equal(
    resourceReport.observationRetainedBytes === "0",
    taps === 0n,
    "a session that asked for no observation retains none, and one that asked retains some",
  );

  const framePointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_METER_FRAME);
  // `3T + 3` for one track: two peaks, the master pair, one gain-reduction slot and the master's.
  assert.equal(exports.miso_engine_web_v1_buffer_capacity(handle, BUFFER_METER_FRAME), 6 * 4);
  const headerPointer = exports.miso_engine_web_v1_meter_header_ptr(handle);
  assert.notEqual(headerPointer, 0);
  const memoryBuffer = exports.memory.buffer;
  const header = new DataView(memoryBuffer, headerPointer, METER_HEADER_BYTES);
  assert.equal(header.getUint32(0, true), METER_HEADER_BYTES);
  assert.equal(header.getUint32(4, true), ABI_VERSION);
  assert.equal(header.getUint32(8, true), 1);
  assert.equal(header.getUint32(40, true), taps === 0n ? 0 : 1);
  const frame = new Float32Array(memoryBuffer, framePointer, 6);
  assert.equal(exports.miso_engine_web_v1_meter_lease(handle, 1), 0);

  const feed = (block) => {
    const id = new TextEncoder().encode(sourceId);
    const idPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
    new Uint8Array(exports.memory.buffer, idPointer, id.byteLength).set(id);
    const pcmPointer = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM);
    new Float32Array(exports.memory.buffer, pcmPointer, QUANTUM * 2).fill(0.5);
    return exports.miso_engine_web_v1_source_submit(
      handle, id.byteLength, 1n, BigInt(block * QUANTUM), 2, QUANTUM, 0,
    );
  };
  const subscribe = {
    kind: COMMAND_OBSERVE_SUBSCRIBE, rack: 1, channel: 255, trackIndex: 0,
    effectIndex: 0, parameterId: 1, smoothingSamples: OBSERVATION_WINDOW_BLOCKS,
    values: [0, 0, 0, 0],
  };
  const unsubscribe = { ...subscribe, kind: COMMAND_OBSERVE_UNSUBSCRIBE, smoothingSamples: 0 };
  const blocks = [];
  const step = (block) => {
    assert.equal(feed(block), 0);
    blocks.push(render(exports, handle));
    exports.miso_engine_web_v1_meter_poll(handle);
  };

  const reports = {};
  reports.unknownTap = submitCommands(exports, handle, [{ ...subscribe, parameterId: 9 }]);
  reports.subscribe = submitCommands(exports, handle, [subscribe]);
  for (let block = 0; block < 8; block += 1) step(block);
  const armed = {
    trackGrDb: frame[4],
    masterGrDb: header.getUint32(44, true) === 1 ? frame[5] : null,
    windowSamples: (header.getBigUint64(24, true) - header.getBigUint64(16, true)).toString(),
  };
  reports.unsubscribe = submitCommands(exports, handle, [unsubscribe]);
  for (let block = 8; block < 12; block += 1) step(block);
  const disarmed = {
    trackGrDb: frame[4],
    masterGrDb: header.getUint32(44, true) === 1 ? frame[5] : null,
  };
  const pcm = [blocks.flatMap((block) => block[0]), blocks.flatMap((block) => block[1])];
  assert.equal(exports.memory.buffer, memoryBuffer, "an observation timeline never grows memory");
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
  return {
    observationRetainedBytesIsZero: resourceReport.observationRetainedBytes === "0",
    reports,
    armed,
    disarmed,
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
  const sessionDocument = await readFile(path.join(fixtureDirectory, "session.json"));
  // W4-D1: one shipped artifact, so the direct oracle drives it alone. The cross-backend proof
  // moved to two independent places: #83's G5 corpus runs the same kernels natively at
  // Scalar/Simd4/Simd8 and under wasmtime with and without simd128, and `nativePcmF32leSha256`
  // below is this exact session rendered through the native `AudioWorkletEngineHost`. Equality is
  // `to_bits` (SHA-256 over little-endian f32 words), never a tolerance.
  const actual = {
    schema: "miso.web.browser.direct-oracle.v1",
    simd128: await runBackend(
      path.join(artifactDirectory, "miso-engine-v1-audio-worklet.simd128.wasm"),
      1,
      sessionDocument,
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
  // #137 E2: the command-timeline leg, pinned the same way and asserted before any print.
  actual.commandTimeline = await runCommandTimeline(
    path.join(artifactDirectory, "miso-engine-v1-audio-worklet.simd128.wasm"),
    await readFile(path.join(fixtureDirectory, "command-session.json")),
    source.sourceId,
  );
  const nativeTimeline = expected.directOracle?.nativeCommandTimelinePcmF32leSha256;
  assert.equal(
    typeof nativeTimeline,
    "string",
    "expected.json must carry the native command-timeline digest pin",
  );
  actual.nativeCommandTimelinePcmF32leSha256 = nativeTimeline;
  assert.equal(
    actual.commandTimeline.pcmF32leSha256,
    nativeTimeline,
    "native and simd128 must render this command timeline to identical bits",
  );
  // Issue #143 E1/E4/E12: the observation timeline. Both legs render the *same* eleven blocks;
  // only the observation capacity and the subscription differ. Their PCM digests are compared to
  // each other and to the native pin before anything is printed, so the pin can only ever be the
  // value three independent runs already agree on.
  const observationSession = await readFile(
    path.join(fixtureDirectory, "observation-session.json"),
  );
  const artifact = path.join(artifactDirectory, "miso-engine-v1-audio-worklet.simd128.wasm");
  const observed = await runObservationTimeline(artifact, observationSession, source.sourceId, 4n);
  const unobserved = await runObservationTimeline(
    artifact, observationSession, source.sourceId, 0n,
  );
  assert.equal(
    observed.pcmF32leSha256,
    unobserved.pcmF32leSha256,
    "arming every declared tap renders the identical bits",
  );
  assert.equal(observed.observationRetainedBytesIsZero, false);
  assert.equal(unobserved.observationRetainedBytesIsZero, true);
  // A tap the effect does not declare is `unknownTap` (10), never `unknownParameter` (5); a
  // subscription against a session with no capacity is `observationUnbound` (11), never silence.
  assert.equal(observed.reports.unknownTap.reason, 10);
  assert.equal(observed.reports.subscribe.result, 0);
  assert.equal(unobserved.reports.subscribe.result, 7);
  assert.equal(unobserved.reports.subscribe.reason, 11);
  assert.ok(observed.armed.trackGrDb > 0, "an armed tap publishes a positive magnitude");
  assert.equal(observed.armed.masterGrDb, observed.armed.trackGrDb);
  assert.equal(observed.disarmed.trackGrDb, 0, "an unsubscribed tap publishes nothing");
  assert.equal(unobserved.armed.trackGrDb, 0);
  assert.equal(unobserved.armed.masterGrDb, null);
  actual.observationTimeline = {
    armedTrackGrDbPositive: observed.armed.trackGrDb > 0,
    armedMasterEqualsTrack: observed.armed.masterGrDb === observed.armed.trackGrDb,
    armedWindowSamples: observed.armed.windowSamples,
    disarmedTrackGrDb: observed.disarmed.trackGrDb,
    unobservedMasterGrDb: unobserved.armed.masterGrDb,
    subscribeReason: observed.reports.subscribe.reason,
    unknownTapReason: observed.reports.unknownTap.reason,
    unboundReason: unobserved.reports.subscribe.reason,
    pcmF32leSha256: observed.pcmF32leSha256,
  };
  const nativeObservation = expected.directOracle?.nativeObservationPcmF32leSha256;
  assert.equal(
    typeof nativeObservation,
    "string",
    "expected.json must carry the native observation-timeline digest pin",
  );
  actual.nativeObservationPcmF32leSha256 = nativeObservation;
  assert.equal(
    observed.pcmF32leSha256,
    nativeObservation,
    "native and simd128 must render this observation timeline to identical bits",
  );
  if (process.env.MISO_ENGINE_WEB_ORACLE_PRINT === "1") {
    process.stdout.write(`${JSON.stringify(actual, null, 2)}\n`);
    return;
  }
  assert.deepEqual(actual, expected.directOracle, "checked raw-Wasm oracle drift");
  console.log("web AudioWorklet independent raw-Wasm oracle passed");
}

await main();
