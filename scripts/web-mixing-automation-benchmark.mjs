// Issue #1003: the browser arm of `console_mixing_automation`.
//
// The native row renders the mono console through a `PreparedRenderPlan` in-process. This arm
// renders the same session through the product's browser path and nothing else:
//
// * the shipped `host_web.wasm`, built by `scripts/build-web-audioworklet.sh --module-only` (the
//   runner builds it and states its digest beside the release pin, the digest of the module the
//   last release shipped, issue #1061);
// * booted from the checked-in mono fixture, its one source stretched so the tone never ends;
// * every block's control traffic submitted as one command batch through `prepared-control.js`
//   and `miso_engine_web_v1_command_submit` / `miso_engine_web_v1_prepared_command_submit`,
//   exactly as the SDK's headless boundary does (`sdk/src/core/boundary.ts`);
// * `channel = 2` on every record. The SDK lowers the EQ's to one prepared `Both` target and the
//   host lowers the compressor's and the limiter's to a Left and a Right record;
// * only `miso_engine_web_v1_render` inside the clock.
//
// The controls, their held bases and their per-block values are the native row's, read from the
// `mixing_automation_controls` example rather than transcribed, and each base is checked against
// the document this arm boots.
//
// The input is not the native row's (#1011). The native row binds each track to one frozen
// 128-frame block of the tone, phase-offset by track and repeated every block
// (`console_workload::source_block`). This arm streams the fixture's one source, so every track
// reads the same tone, in phase and continuous across blocks. The rate and the amplitude are the
// native tone's, from the same table. Both feeds are stated in the record, because how a limiter
// engages depends on them.
//
// usage: node --no-liftoff web-mixing-automation-benchmark.mjs preflight MODULE.wasm CONTROLS.json
//        node --no-liftoff web-mixing-automation-benchmark.mjs run MODULE.wasm CONTROLS.json ROUND
//
// `preflight` is untimed: seven arms over the pre-roll and the preflight blocks, and the row's
// premises asserted on their digests. `run` asserts the same premises first, then times the three
// arms alternated per observation and prints one JSON record for ROUND (`warmup`, `1` or `2`; the
// runner launches one process per round, as the console runner does). Every assertion throws, so
// a broken premise exits non-zero before any number is printed.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import process from "node:process";
import { createPreparedControl } from "../hosts/host-web/web/prepared-control.js";

const ROOT = new URL("../", import.meta.url);
const ABI_LAYOUT = JSON.parse(readFileSync(new URL("sdk/assets/miso-engine-v1-abi-layout.json", ROOT)));
const FIXTURE_ID = "fixtures/session/v1/console-sixty-four-track-mono.json";
// The release pin (#1061): `module_matches_pin` says whether the measured module is the one the
// last release shipped, not whether the tree was re-pinned.
const PIN_FILE = "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256";

const OBSERVATIONS = 1000;
const RATE = 48000;
const Q = 128;
// Blocks of PCM kept queued ahead of the render position.
const LEAD_BLOCKS = 4;
// The fixture's source is one second long; the arms render about 1,100 blocks each.
const SOURCE_FRAMES = "48000000";
const ARMS = ["quiet", "restated", "automated"];
const PREFLIGHT_ARMS = [
  ["quiet", null], ["restated", null], ["automated", null],
  ["automated", "miso.parametric-eq"], ["automated", "miso.compressor"],
  ["automated", "miso.true-peak-limiter"], ["restated", "miso.parametric-eq"],
];
const EFFECT_NAMES = {
  "miso.parametric-eq": "eq", "miso.compressor": "compressor", "miso.true-peak-limiter": "limiter",
};

const field = (structure, name) => {
  const row = ABI_LAYOUT.structures[structure].fields.find((item) => item.name === name);
  assert.ok(row, `missing ABI field ${structure}.${name}`);
  return row.offset;
};
const commandField = (name) => ABI_LAYOUT.commandRecord.fields.find((item) => item.name === name).offset;
const constant = (group, name) => ABI_LAYOUT.constants[group].find((item) => item.name === name).value;
const RESULT_OK = constant("resultCodes", "ok");
const COMMAND_EFFECT_PARAM = constant("wireCommandKinds", "effectParam");
const BUFFER_SOURCE_ID = constant("bufferKinds", "sourceId");
const BUFFER_SOURCE_PCM = constant("bufferKinds", "sourcePcm");
const BUFFER_OUTPUT = constant("bufferKinds", "outputPcm");
const BUFFER_COMMAND = constant("bufferKinds", "command");
const COMMAND_RECORD_BYTES = ABI_LAYOUT.commandRecord.bytes;
const COMMAND_QUEUE_RECORDS = ABI_LAYOUT.constants.defaultCommandQueueRecords;
// The SDK's default ring (`defaultSourceRingFrames`): the stall tolerance in quanta plus the
// reserve.
const SOURCE_RING_FRAMES = (() => {
  const { stallToleranceMs, reserveQuanta } = ABI_LAYOUT.constants.sourceRing;
  return (Math.ceil(Math.floor((RATE * stallToleranceMs) / 1000) / Q) + reserveQuanta) * Q;
})();

// ---------------------------------------------------------------------------------------------
// Inputs: the module, the native row's control table, and the session document.
// ---------------------------------------------------------------------------------------------

const [mode, modulePath, controlsPath, roundArgument] = process.argv.slice(2);
const ROUNDS = { warmup: 0, 1: 1, 2: 2 };
if (!((mode === "preflight" && process.argv.length === 5)
      || (mode === "run" && process.argv.length === 6 && Object.hasOwn(ROUNDS, roundArgument)))) {
  process.stderr.write("usage: web-mixing-automation-benchmark.mjs preflight MODULE.wasm CONTROLS.json | run MODULE.wasm CONTROLS.json warmup|1|2\n");
  process.exit(2);
}
const ROUND = mode === "run" ? ROUNDS[roundArgument] : null;
const moduleBytes = readFileSync(modulePath);
const moduleSha256 = createHash("sha256").update(moduleBytes).digest("hex");
const pinnedSha256 = readFileSync(new URL(PIN_FILE, ROOT), "utf8").trim();
const table = JSON.parse(readFileSync(controlsPath, "utf8"));
assert.equal(table.fixture_id, FIXTURE_ID, "the control table names another fixture");
assert.equal(table.controls.length, 8, "the row automates eight controls");
const PREROLL = table.preroll_blocks;
const PREFLIGHT_BLOCKS = table.preflight_blocks;
// The native tone's rate and amplitude, streamed continuously and in phase on every track.
const NATIVE_FEED = table.native_input_feed;
assert.equal(NATIVE_FEED.waveform, "sine", "the native feed is a sine");
assert.equal(NATIVE_FEED.continuous_across_blocks, false, "the native feed is a frozen block");
const TONE_RADIANS_PER_FRAME = NATIVE_FEED.radians_per_frame;
const TONE_AMPLITUDE = NATIVE_FEED.amplitude;
const INPUT_FEED = {
  waveform: "sine",
  radians_per_frame: TONE_RADIANS_PER_FRAME,
  amplitude: TONE_AMPLITUDE,
  track_phase_radians: 0,
  delivery: "streamed_source",
  block_frames: Q,
  continuous_across_blocks: true,
};

const fixtureText = readFileSync(new URL(FIXTURE_ID, ROOT), "utf8");
const fixture = JSON.parse(fixtureText);
assert.equal(fixture.sources.length, 1, "the mono fixture carries one source");
assert.deepEqual(fixture.automation, [], "the mono fixture declares no automation");
const framesText = `"frames": "${fixture.sources[0].frames}"`;
assert.equal(fixtureText.split(framesText).length, 2, "exactly one source length to stretch");
const documentBytes = new TextEncoder().encode(
  fixtureText.replace(framesText, `"frames": "${SOURCE_FRAMES}"`),
);
const sourceId = new TextEncoder().encode(fixture.sources[0].id);

// Each base is the value the booted document holds, as the native row reads it from the model.
const RACKS = ["simd1", "dynamic", "simd2"];
for (const control of table.controls) {
  const track = fixture.tracks[control.track_index];
  assert.equal(track.id, control.track_id, `${control.track_id}: track index`);
  const slot = track[RACKS[control.rack]].effects[control.effect_index];
  assert.equal(slot.id, control.slot_id, `${control.track_id}: slot`);
  assert.equal(slot.identity.effect_id, control.effect, `${control.track_id}: effect`);
  const held = slot.params.filter((param) => param.parameter_id === control.parameter_id);
  assert.ok(held.length > 0, `${control.track_id}: the document holds the parameter`);
  for (const param of held) {
    assert.equal(Math.fround(param.value), Math.fround(control.base), `${control.track_id}: held base`);
  }
}

// ---------------------------------------------------------------------------------------------
// One booted engine, and the SDK's prepared-control owner on it.
// ---------------------------------------------------------------------------------------------

// Every pointer is read before `memory.buffer`: an export that grows memory detaches the buffer a
// view was taken on.
function commandReport(e, handle, result, records) {
  const pointer = e.miso_engine_web_v1_command_report_ptr(handle);
  const report = new DataView(e.memory.buffer, pointer, ABI_LAYOUT.structures.commandReport.bytes);
  return {
    result,
    reason: report.getUint32(field("commandReport", "reason"), true),
    rejectedIndex: report.getUint32(field("commandReport", "rejectedIndex"), true),
    admitted: report.getUint32(field("commandReport", "admitted"), true),
    appliedAtSample: report.getBigUint64(field("commandReport", "appliedAtSample"), true),
    records: records.slice(),
  };
}

// The same five callbacks `boundary.ts` hands `createPreparedControl`.
function preparedControl(e, handle) {
  const configBytes = ABI_LAYOUT.structures.eqTargetConfig.bytes;
  const stage = (records) => {
    const pointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_COMMAND);
    new Uint8Array(e.memory.buffer, pointer, records.byteLength).set(records);
  };
  return createPreparedControl({
    instance: e,
    abiLayout: ABI_LAYOUT,
    sampleRateHz: RATE,
    configCopySync: (address) => {
      const result = e.miso_engine_web_v1_eq_target_config_copy(
        handle, address.trackIndex, address.rack, address.effectIndex,
      );
      const none = constant("commandReasons", "none");
      if (result !== RESULT_OK) return { result, reason: none, config: new Uint8Array(0) };
      const pointer = e.miso_engine_web_v1_eq_target_config_ptr(handle);
      return { result, reason: none, config: new Uint8Array(e.memory.buffer, pointer, configBytes).slice() };
    },
    ordinarySubmitSync: (records, count) => {
      stage(records);
      return commandReport(e, handle, e.miso_engine_web_v1_command_submit(handle, count), records);
    },
    preparedSubmitSync: (records, companion, count) => {
      stage(records);
      const pointer = e.miso_engine_web_v1_prepared_companion_ptr(handle);
      new Uint8Array(e.memory.buffer, pointer, companion.byteLength).set(companion);
      return commandReport(e, handle,
        e.miso_engine_web_v1_prepared_command_submit(handle, count, companion.byteLength), records);
    },
    preparedRefusalSync: (records, _count, reason, rejectedIndex, result) => ({
      result, reason, rejectedIndex: rejectedIndex ?? 0, admitted: 0, appliedAtSample: 0n,
      records: records.slice(),
    }),
  });
}

const wasmModule = new WebAssembly.Module(moduleBytes);

function boot() {
  const e = new WebAssembly.Instance(wasmModule, {}).exports;
  assert.equal(e.miso_engine_web_v1_abi_version(), ABI_LAYOUT.abiVersion);
  const optionsPointer = e.miso_engine_web_v1_boot_options_ptr();
  const options = new DataView(e.memory.buffer, optionsPointer, ABI_LAYOUT.structures.bootOptions.bytes);
  const u32 = (name, value) => options.setUint32(field("bootOptions", name), value, true);
  const u64 = (name, value) => options.setBigUint64(field("bootOptions", name), value, true);
  u32("structSize", ABI_LAYOUT.structures.bootOptions.bytes);
  u32("abiVersion", ABI_LAYOUT.abiVersion);
  u32("requireSampleRateHz", RATE);
  u32("requireQuantumFrames", Q);
  u32("sourceRingFrames", SOURCE_RING_FRAMES);
  u32("reserved0", 0);
  u64("maximumMemoryBytes", 0n);
  // The live console with the default command queue, and no meters, observation or master, as
  // the native row prepares its plan (`control: true`, nothing else).
  u64("consoleCommandQueueRecords", BigInt(COMMAND_QUEUE_RECORDS));
  u64("consoleMeterBlocks", 0n);
  u64("consoleObservationTaps", 0n);
  u64("consoleMasterTrackPlusOne", 0n);
  const pointer = e.miso_engine_web_v1_document_ptr(documentBytes.byteLength);
  new Uint8Array(e.memory.buffer, pointer, documentBytes.byteLength).set(documentBytes);
  const handle = e.miso_engine_web_v1_boot(documentBytes.byteLength);
  if (handle === 0) {
    const bytes = e.miso_engine_web_v1_boot_diagnostic_bytes();
    throw new Error(`boot ${e.miso_engine_web_v1_boot_result()}: ${new TextDecoder().decode(
      new Uint8Array(e.memory.buffer, pointer, bytes))}`);
  }
  // The wire's track index is the booted console's track order; check the table addresses it.
  for (const control of table.controls) {
    const length = e.miso_engine_web_v1_console_track_id(handle, control.track_index);
    const idPointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
    const id = new TextDecoder().decode(new Uint8Array(e.memory.buffer, idPointer, length));
    assert.equal(id, control.track_id, `${control.track_id}: console track index`);
  }
  return {
    e, handle, control: preparedControl(e, handle), frame: 0n, block: 0,
    outputPointer: e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT),
    left: new Float32Array(Q), right: new Float32Array(Q),
  };
}

// The tone: one continuous sine on the fixture's one source. The mono fixture maps source channel
// 0 onto both channels of every track, so the right plane is carried and never read.
function feed(engine) {
  const block = Number(engine.frame / BigInt(Q));
  for (let i = 0; i < Q; i++) {
    const t = (block * Q + i) * TONE_RADIANS_PER_FRAME;
    engine.left[i] = Math.sin(t) * TONE_AMPLITUDE;
    engine.right[i] = -Math.sin(t) * 0.45;
  }
  const idPointer = engine.e.miso_engine_web_v1_buffer_ptr(engine.handle, BUFFER_SOURCE_ID);
  const pcmPointer = engine.e.miso_engine_web_v1_buffer_ptr(engine.handle, BUFFER_SOURCE_PCM);
  const memory = engine.e.memory.buffer;
  new Uint8Array(memory, idPointer, sourceId.length).set(sourceId);
  const pcm = new Float32Array(memory, pcmPointer, 2 * Q);
  pcm.set(engine.left, 0);
  pcm.set(engine.right, Q);
  const result = engine.e.miso_engine_web_v1_source_submit(
    engine.handle, sourceId.length, 1n, engine.frame, 2, Q, 0);
  assert.equal(result, RESULT_OK, "source submit");
  engine.frame += BigInt(Q);
}

function encode(records) {
  const bytes = new Uint8Array(records.length * COMMAND_RECORD_BYTES);
  const view = new DataView(bytes.buffer);
  records.forEach((record, index) => {
    const o = index * COMMAND_RECORD_BYTES;
    view.setUint8(o + commandField("kind"), COMMAND_EFFECT_PARAM);
    view.setUint8(o + commandField("rack"), record.rack);
    view.setUint8(o + commandField("channel"), 2);
    view.setUint32(o + commandField("trackIndex"), record.track_index, true);
    view.setUint32(o + commandField("effectIndex"), record.effect_index, true);
    view.setUint32(o + commandField("parameterId"), record.parameter_id, true);
    view.setUint32(o + commandField("smoothingSamples"), 0, true);
    view.setFloat32(o + commandField("values"), record.value, true);
  });
  return bytes;
}

// ---------------------------------------------------------------------------------------------
// One arm: an engine, the traffic it delivers, and what it was acknowledged.
// ---------------------------------------------------------------------------------------------

function value(control, arm, block) {
  if (arm === "restated") return control.base;
  return block % 2 === 0 ? control.even_value : control.odd_value;
}

function makeArm(arm, effect) {
  const engine = boot();
  const controls = table.controls.filter((control) => effect === null || control.effect === effect);
  const state = { arm, effect, engine, controls, submitted: 0, admitted: 0, refusals: 0 };
  for (let i = 0; i < LEAD_BLOCKS; i++) feed(engine);
  if (arm !== "quiet") submit(state, "restated");
  for (let i = 0; i < PREROLL; i++) {
    feed(engine);
    render(state);
  }
  // The settling write is not the arm's traffic.
  state.submitted = 0;
  state.admitted = 0;
  return state;
}

// One block's traffic, as one batch: what an app hands the SDK for a mixing gesture.
function submit(state, arm = state.arm) {
  if (arm === "quiet" || state.controls.length === 0) return;
  const block = state.engine.block;
  const records = state.controls.map((control) => ({ ...control, value: Math.fround(value(control, arm, block)) }));
  // A restatement pushes exactly the held value (#1011): the digests cannot say so for a limiter,
  // which does not engage near its held ceiling.
  if (arm === "restated") {
    for (const record of records) {
      assert.equal(record.value, Math.fround(record.base), `${record.track_id}: restated value`);
    }
  }
  const reply = state.engine.control.submitSync(encode(records), records.length);
  state.submitted += records.length;
  if (reply.result === RESULT_OK) state.admitted += reply.admitted;
  else state.refusals += 1;
}

function render(state) {
  const result = state.engine.e.miso_engine_web_v1_render(state.engine.handle, Q);
  assert.equal(result, RESULT_OK, "render");
  state.engine.block += 1;
}

function absorb(state, hash) {
  hash.update(new Uint8Array(state.engine.e.memory.buffer, state.engine.outputPointer, 2 * Q * 4));
}

function armName(arm, effect) {
  return effect === null ? arm : `${arm}_${EFFECT_NAMES[effect]}_only`;
}

// ---------------------------------------------------------------------------------------------
// The premises, asserted before any number is taken.
// ---------------------------------------------------------------------------------------------

function preflight() {
  const arms = PREFLIGHT_ARMS.map(([arm, effect]) => makeArm(arm, effect));
  const hashes = arms.map(() => createHash("sha256"));
  for (let block = 0; block < PREFLIGHT_BLOCKS; block++) {
    arms.forEach((state, index) => {
      feed(state.engine);
      submit(state);
      render(state);
      absorb(state, hashes[index]);
    });
  }
  const digests = Object.fromEntries(arms.map((state, index) => [
    armName(state.arm, state.effect), hashes[index].digest("hex"),
  ]));
  for (const state of arms) {
    assert.equal(state.refusals, 0, `preflight ${armName(state.arm, state.effect)}: a batch was refused`);
    assert.equal(state.admitted, state.submitted,
      `preflight ${armName(state.arm, state.effect)}: a record was not admitted`);
  }
  assert.equal(digests.quiet, digests.restated, "preflight: restating the held values moved a rendered bit");
  assert.notEqual(digests.automated, digests.restated, "preflight: the automated arm rendered the restated arm's bits");
  for (const effect of Object.values(EFFECT_NAMES)) {
    assert.notEqual(digests[`automated_${effect}_only`], digests.restated,
      `preflight: automating the ${effect} alone moved no rendered bit`);
  }
  assert.equal(digests.restated_eq_only, digests.quiet, "preflight: restating the EQ moved a rendered bit");
  return digests;
}

// ---------------------------------------------------------------------------------------------
// The timed run: three arms alternated per observation, the render call alone inside the clock.
// ---------------------------------------------------------------------------------------------

function nearestRank(samples, percentile) {
  const sorted = [...samples].sort((a, b) => a - b);
  // Integer arithmetic, as `bench_support::stats::nearest_rank` does it.
  const rank = Math.ceil((sorted.length * percentile) / 100);
  return sorted[Math.max(rank, 1) - 1];
}

function pairedMedian(left, right) {
  const paired = left.map((value, index) => value - right[index]).sort((a, b) => a - b);
  return paired[paired.length >> 1];
}

const loadStart = readFileSync("/proc/loadavg", "utf8").trim();
const preflightDigests = preflight();
if (mode === "preflight") {
  process.stdout.write(`${JSON.stringify({ mode, module_sha256: moduleSha256, digests: preflightDigests })}\n`);
  process.exit(0);
}

const arms = ARMS.map((arm) => makeArm(arm, null));
const samples = arms.map(() => new Array(OBSERVATIONS));
const hashes = arms.map(() => createHash("sha256"));
for (let observation = 0; observation < OBSERVATIONS; observation++) {
  arms.forEach((state, index) => {
    feed(state.engine);
    submit(state);
    const start = process.hrtime.bigint();
    const result = state.engine.e.miso_engine_web_v1_render(state.engine.handle, Q);
    const elapsed = process.hrtime.bigint() - start;
    assert.equal(result, RESULT_OK, "render");
    state.engine.block += 1;
    samples[index][observation] = Number(elapsed);
  });
  arms.forEach((state, index) => absorb(state, hashes[index]));
}
const loadEnd = readFileSync("/proc/loadavg", "utf8").trim();
const digests = hashes.map((hash) => hash.digest("hex"));
assert.equal(digests[0], digests[1], "run: restating the held values moved a rendered bit");
assert.notEqual(digests[1], digests[2], "run: the automated arm rendered the restated arm's bits");
for (const state of arms.slice(1)) {
  assert.equal(state.refusals, 0, `run ${state.arm}: a batch was refused`);
  assert.equal(state.admitted, OBSERVATIONS * table.controls.length, `run ${state.arm}: a record was not admitted`);
}

const record = {
  schema_version: 1,
  issue: 1003,
  record: "web_mixing_automation",
  round: ROUND,
  workload_kind: "sixty_four_track_console_mono_mixing_automation",
  fixture_id: FIXTURE_ID,
  source_frames: SOURCE_FRAMES,
  tracks: fixture.tracks.length,
  input_signal: "tone",
  input_feed: INPUT_FEED,
  native_input_feed: NATIVE_FEED,
  sample_rate_hz: RATE,
  quantum_frames: Q,
  module_sha256: moduleSha256,
  module_matches_pin: moduleSha256 === pinnedSha256,
  pinned_sha256: pinnedSha256,
  node_version: process.version,
  v8_version: process.versions.v8,
  node_flags: process.execArgv,
  console_command_queue_records: COMMAND_QUEUE_RECORDS,
  source_ring_frames: SOURCE_RING_FRAMES,
  observations: OBSERVATIONS,
  preroll_blocks: PREROLL,
  pairing: "alternating_per_observation",
  arms: ARMS,
  controls: table.controls.map((control) => ({
    track_id: control.track_id, slot_id: control.slot_id, effect: control.effect,
    parameter: control.parameter, parameter_index: control.parameter_index,
    parameter_id: control.parameter_id, lowering: control.lowering, base: control.base,
    step: control.step, even_value: control.even_value, odd_value: control.odd_value,
  })),
  command_records_per_block: table.controls.length,
  records_admitted: Object.fromEntries(arms.map((state) => [state.arm, state.admitted])),
  units: "ns_per_block",
  percentile_method: "nearest_rank",
  ...Object.fromEntries(arms.flatMap((state, index) => [50, 95, 99].map((p) => [
    `${state.arm}_p${p}_ns`, nearestRank(samples[index], p),
  ]))),
  paired_ramp_delta_median_ns: pairedMedian(samples[2], samples[1]),
  paired_collapse_delta_median_ns: pairedMedian(samples[1], samples[0]),
  quiet_output_sha256: digests[0],
  restated_output_sha256: digests[1],
  automated_output_sha256: digests[2],
  preflight_output_sha256: preflightDigests,
  bit_identity: "quiet == restated, asserted in-run",
  // host_web.wasm exports no collapse counter; the native row states them.
  bank_collapse_counters_exported: false,
  loadavg_start: loadStart,
  loadavg_end: loadEnd,
  descriptive_only: true,
  statistical_method: "three arms alternated per observation; one warmup launch and two measured launches; nearest-rank percentiles over per-block nanoseconds of miso_engine_web_v1_render alone; ramp delta is automated minus restated and collapse delta is restated minus quiet, per observation; descriptive only; no threshold",
};
process.stdout.write(`${JSON.stringify(record)}\n`);
