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
// Three documents ride along (#1085, #1228): the standing sixty-four-track console, the app shape
// and the bus-and-send console, the native rows `sixty_four_track_console`,
// `sixty_four_track_app_shape` and `sixty_four_track_console_sends`, listed with their facts in the
// same table. Each is booted from its checked-in fixture as written (its one source stretched) the
// way every document boots, through host-core with live controls, so every route into a submix of
// the sends document is a live route; each is checked to carry the layout and the bypass the table
// states, fed the same streamed tone, and rendered with no control traffic. After the three arms
// are timed, the three documents are timed alternated per observation,
// `miso_engine_web_v1_render` alone inside the clock, and the record states each one's
// percentiles and digest under `documents`. They are the console strip's browser baseline; no
// per-N browser rows exist, because that would be a second framework.
//
// usage: node --no-liftoff web-mixing-automation-benchmark.mjs preflight MODULE.wasm CONTROLS.json
//        node --no-liftoff web-mixing-automation-benchmark.mjs run MODULE.wasm CONTROLS.json ROUND
//
// A second mode (#1289, slice B1 of #1269) times a session boot rather than a render; its
// `rebuild-*` commands need no control table and are described where they are defined:
//        node --no-liftoff web-mixing-automation-benchmark.mjs rebuild-preflight MODULE.wasm
//        node --no-liftoff web-mixing-automation-benchmark.mjs rebuild-round MODULE.wasm warmup|1|2
//        node web-mixing-automation-benchmark.mjs rebuild-run WORKDIR OUTDIR CPU CONTROL
//        node web-mixing-automation-benchmark.mjs rebuild-validate RECORDS.jsonl
//
// `preflight` is untimed: seven arms over the pre-roll and the preflight blocks, and the row's
// premises asserted on their digests; then the three documents over the same blocks, each rendering
// audible bits of its own. `run` asserts the same premises first, then times the three arms
// alternated per observation, then the three documents alternated per observation, and prints one
// JSON record for ROUND (`warmup`, `1` or `2`; the runner launches one process per round, as the
// console runner does). Every assertion throws, so a broken premise exits non-zero before any
// number is printed.
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
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
// The rebuild-cost mode (#1289, slice B1 of #1269): how long a session boot blocks the audio
// thread. It needs no control table and returns before anything below reads one.
//
// The browser engine's Wasm instance lives inside the `AudioWorkletProcessor`, so a replacement
// session will be prepared on the rendering thread between two `process()` calls (#1269 P12). A
// boot runs the whole pipeline a replacement runs -- document parse, session compile, host-core
// preparation, graph compile, PDC, bind -- so `miso_engine_web_v1_boot` on the shipped module is
// the proxy this measures (decision D1) for four documents: the nine-track EQ session and the three
// sixty-four-track console documents. Each timed boot is on a fresh instance of the shipped module
// (its linear memory grows inside the clock, as a first boot in the worklet does), with the
// browser's live-control boot options: the SDK's default command queue and no meters, observation
// taps, master or spectrum. Only the boot export is inside the clock; the dispose that follows is
// timed on its own and stated beside it, and after every timed boot the session is fed and must
// render an audible block, outside the clock.
//
// `rebuild-preflight` is untimed: it boots every document once and asserts an audible block.
// `rebuild-round` asserts the same premise, then times the four documents alternated per
// observation and prints one JSON record. `rebuild-run` is the one timed invocation (launched by
// `run-web-mixing-automation-benchmark.sh rebuild-run`, which chooses the CPU and the control
// statement): before launching anything it refuses an existing output, modified tracked files and a
// module not prepared at HEAD; then it launches one warmup round, whose record is discarded, and two
// measured rounds, one process each, and writes the records, the validator's verdict and a short
// report under OUTDIR. A refused run keeps its records under a name no reader accepts.
// `rebuild-validate` applies the validator to a records file.
// ---------------------------------------------------------------------------------------------

const REBUILD_ROOT = fileURLToPath(ROOT);
const REBUILD_SCRIPT = fileURLToPath(import.meta.url);
// The frozen workload (D1).
const REBUILD_DOCUMENTS = [
  { kind: "nine_track_eq", fixture_id: "fixtures/session/v1/parametric-eq-nine-track.json", tracks: 9 },
  { kind: "sixty_four_track_console", fixture_id: "fixtures/session/v1/console-sixty-four-track.json", tracks: 64 },
  { kind: "sixty_four_track_app_shape", fixture_id: "fixtures/session/v1/console-sixty-four-track-app.json", tracks: 64 },
  { kind: "sixty_four_track_console_sends", fixture_id: "fixtures/session/v1/console-sixty-four-track-sends.json", tracks: 64 },
];
const REBUILD_OBSERVATIONS = 25;
// One quantum's budget: 128 / 48000 s.
const REBUILD_BUDGET_NS = (Q * 1e9) / RATE;
const REBUILD_BLOCKS = 8;
const REBUILD_ROUNDS = { warmup: 0, 1: 1, 2: 2 };
const REBUILD_RECORD = "web-rebuild-cost.jsonl";
const REBUILD_REFUSED = "web-rebuild-cost.refused.jsonl";
const REBUILD_VERDICT = "validator.json";
const REBUILD_REPORT = "report.md";
const REBUILD_STDERR = "web-rebuild-cost.stderr.log";
const REBUILD_OUTPUTS = [REBUILD_RECORD, REBUILD_REFUSED, REBUILD_VERDICT, REBUILD_REPORT, REBUILD_STDERR];


const rebuildSha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

// ---------------------------------------------------------------------------------------------
// The validator (gate 2): a record set the report may be written from.
// ---------------------------------------------------------------------------------------------

// Every reason the records are refused; an empty list accepts them.
function rebuildRefusalReasons(records) {
  const reasons = [];
  if (!Array.isArray(records) || records.length !== 2) {
    return [`expected two measured records, found ${Array.isArray(records) ? records.length : "none"}`];
  }
  const rounds = records.map((record) => record?.round);
  if (rounds[0] !== 1 || rounds[1] !== 2) reasons.push(`rounds are ${JSON.stringify(rounds)}, not [1,2]`);
  records.forEach((record, index) => {
    const at = `record ${index + 1}`;
    if (record?.record !== "web_rebuild_cost" || record?.issue !== 1289 || record?.schema_version !== 1) {
      reasons.push(`${at}: not a web_rebuild_cost schema 1 record`);
      return;
    }
    if (record.sample_rate_hz !== RATE || record.quantum_frames !== Q
        || record.quantum_budget_ns !== REBUILD_BUDGET_NS) {
      reasons.push(`${at}: the rate, quantum or budget is not the frozen one`);
    }
    const docs = Array.isArray(record.documents) ? record.documents : [];
    const kinds = docs.map((doc) => doc?.workload_kind);
    if (JSON.stringify(kinds) !== JSON.stringify(REBUILD_DOCUMENTS.map((doc) => doc.kind))) {
      reasons.push(`${at}: documents are ${JSON.stringify(kinds)}, not the four frozen ones`);
    }
    for (const doc of docs) {
      const name = `${at} ${doc?.workload_kind}`;
      const samples = doc?.boot_ns;
      if (doc?.observations !== REBUILD_OBSERVATIONS || !Array.isArray(samples) || samples.length !== REBUILD_OBSERVATIONS) {
        reasons.push(`${name}: not ${REBUILD_OBSERVATIONS} observations`);
        continue;
      }
      if (!samples.every((value) => Number.isSafeInteger(value) && value > 0)) {
        reasons.push(`${name}: a boot time is not a positive integer`);
      }
      // A refused boot returns early and fast; every timed boot must have prepared a session that
      // renders audible bits.
      if (doc.failed_boots !== 0 || doc.booted !== REBUILD_OBSERVATIONS || doc.audible_after_boot !== REBUILD_OBSERVATIONS) {
        reasons.push(`${name}: a boot failed (failed ${doc.failed_boots}, booted ${doc.booted}, `
          + `audible ${doc.audible_after_boot})`);
      }
      if (doc.disposed !== REBUILD_OBSERVATIONS) reasons.push(`${name}: a dispose failed`);
      if (!(Number.isSafeInteger(doc.peak_memory_bytes) && doc.peak_memory_bytes > 0)) {
        reasons.push(`${name}: no peak memory`);
      }
    }
  });
  if (reasons.length === 0) {
    for (const key of ["module_sha256", "candidate_commit", "prepared_commit"]) {
      if (records[0][key] !== records[1][key]) reasons.push(`the rounds disagree on ${key}`);
    }
  }
  return reasons;
}

// ---------------------------------------------------------------------------------------------
// One boot on a fresh instance of the shipped module.
// ---------------------------------------------------------------------------------------------

function rebuildDocument(doc) {
  const text = readFileSync(path.join(REBUILD_ROOT, doc.fixture_id), "utf8");
  const parsed = JSON.parse(text);
  assert.equal(parsed.tracks.length, doc.tracks, `${doc.fixture_id}: track count`);
  assert.equal(parsed.sample_rate_hz, RATE, `${doc.fixture_id}: rate`);
  assert.equal(parsed.sources.length, 1, `${doc.fixture_id}: one source`);
  assert.equal(parsed.sources[0].channels, 2, `${doc.fixture_id}: a stereo source`);
  return {
    ...doc,
    bytes: new TextEncoder().encode(text),
    sourceId: new TextEncoder().encode(parsed.sources[0].id),
  };
}

function rebuildInstance(wasmModule) {
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
  // The browser's live controls: the SDK's default command queue, nothing else attached.
  u64("liveControlCommandQueueRecords", BigInt(COMMAND_QUEUE_RECORDS));
  u64("liveControlMeterBlocks", 0n);
  u64("liveControlObservationTaps", 0n);
  u64("liveControlMasterTrackPlusOne", 0n);
  return e;
}

function rebuildStage(e, document) {
  const pointer = e.miso_engine_web_v1_document_ptr(document.bytes.byteLength);
  assert.notEqual(pointer, 0, `${document.kind}: document staging`);
  new Uint8Array(e.memory.buffer, pointer, document.bytes.byteLength).set(document.bytes);
}

// Feeds a few blocks of a sine on the one source and renders; true when a block was audible.
// Outside every clock.
function rebuildRendersAudibly(e, handle, document) {
  let frame = 0n;
  const feed = () => {
    const idPointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
    const pcmPointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM);
    new Uint8Array(e.memory.buffer, idPointer, document.sourceId.length).set(document.sourceId);
    const pcm = new Float32Array(e.memory.buffer, pcmPointer, 2 * Q);
    for (let i = 0; i < Q; i++) {
      const t = (Number(frame) + i) * 0.0575;
      pcm[i] = Math.sin(t) * 0.25;
      pcm[Q + i] = -Math.sin(t) * 0.2;
    }
    const result = e.miso_engine_web_v1_source_submit(
      handle, document.sourceId.length, 1n, frame, 2, Q, 0);
    assert.equal(result, RESULT_OK, `${document.kind}: source submit`);
    frame += BigInt(Q);
  };
  for (let i = 0; i < 4; i++) feed();
  let audible = false;
  for (let block = 0; block < REBUILD_BLOCKS; block++) {
    feed();
    assert.equal(e.miso_engine_web_v1_render(handle, Q), RESULT_OK, `${document.kind}: render`);
    const outputPointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT);
    const output = new Float32Array(e.memory.buffer, outputPointer, 2 * Q);
    audible ||= output.some((word) => word !== 0 && Number.isFinite(word));
  }
  return audible;
}

// One observation: a fresh instance, the document staged, `miso_engine_web_v1_boot` alone in the
// clock, the result checked and rendered outside it, then the dispose timed on its own.
function rebuildObserve(wasmModule, document) {
  const e = rebuildInstance(wasmModule);
  rebuildStage(e, document);
  const start = process.hrtime.bigint();
  const handle = e.miso_engine_web_v1_boot(document.bytes.byteLength);
  const bootNs = Number(process.hrtime.bigint() - start);
  const bootResult = e.miso_engine_web_v1_boot_result();
  const memoryBytes = e.memory.buffer.byteLength;
  if (handle === 0 || bootResult !== RESULT_OK) {
    return { bootNs, booted: false, audible: false, disposed: false, disposeNs: 0, memoryBytes, bootResult };
  }
  const audible = rebuildRendersAudibly(e, handle, document);
  const disposeStart = process.hrtime.bigint();
  const disposeResult = e.miso_engine_web_v1_dispose(handle);
  const disposeNs = Number(process.hrtime.bigint() - disposeStart);
  return { bootNs, booted: true, audible, disposed: disposeResult === RESULT_OK, disposeNs, memoryBytes, bootResult };
}

// The premise, untimed: every document boots once and renders an audible block (gate 1).
function rebuildPreflight(wasmModule, documents) {
  return Object.fromEntries(documents.map((document) => {
    const result = rebuildObserve(wasmModule, document);
    assert.ok(result.booted, `preflight ${document.kind}: boot refused with result ${result.bootResult}`);
    assert.ok(result.audible, `preflight ${document.kind}: rendered no audible block`);
    assert.ok(result.disposed, `preflight ${document.kind}: dispose failed`);
    return [document.kind, { audible: true, memory_bytes: result.memoryBytes }];
  }));
}

// One round's record. `rebuild-preflight` runs it with no timed observation, so everything a timed
// round reads and assembles is exercised before any timed launch.
function rebuildRound(modulePath, roundName, count) {
  const moduleBytes = readFileSync(modulePath);
  const wasmModule = new WebAssembly.Module(moduleBytes);
  const documents = REBUILD_DOCUMENTS.map(rebuildDocument);
  const premise = rebuildPreflight(wasmModule, documents);
  const loadStart = readFileSync("/proc/loadavg", "utf8").trim();
  const observations = documents.map(() => []);
  for (let observation = 0; observation < count; observation++) {
    documents.forEach((document, index) => observations[index].push(rebuildObserve(wasmModule, document)));
  }
  const loadEnd = readFileSync("/proc/loadavg", "utf8").trim();
  return {
    schema_version: 1,
    issue: 1289,
    record: "web_rebuild_cost",
    round: REBUILD_ROUNDS[roundName],
    module_sha256: rebuildSha256(moduleBytes),
    // Whether the measured module is the one the last release shipped (#1061), not a re-pin.
    module_matches_pin: rebuildSha256(moduleBytes) === readFileSync(path.join(REBUILD_ROOT, PIN_FILE), "utf8").trim(),
    node_version: process.version,
    v8_version: process.versions.v8,
    node_flags: process.execArgv,
    sample_rate_hz: RATE,
    quantum_frames: Q,
    quantum_budget_ns: REBUILD_BUDGET_NS,
    live_control_command_queue_records: COMMAND_QUEUE_RECORDS,
    source_ring_frames: SOURCE_RING_FRAMES,
    observations_per_document: count,
    preflight: premise,
    pairing: "four documents alternated per observation; a fresh instance per boot",
    units: "ns",
    percentile_method: "nearest_rank",
    documents: documents.map((document, index) => {
      const rows = observations[index];
      const boot = rows.map((row) => row.bootNs);
      const dispose = rows.filter((row) => row.disposed).map((row) => row.disposeNs);
      return {
        workload_kind: document.kind,
        fixture_id: document.fixture_id,
        document_sha256: rebuildSha256(document.bytes),
        document_bytes: document.bytes.byteLength,
        tracks: document.tracks,
        observations: rows.length,
        booted: rows.filter((row) => row.booted).length,
        failed_boots: rows.filter((row) => !row.booted).length,
        audible_after_boot: rows.filter((row) => row.audible).length,
        disposed: dispose.length,
        boot_ns: boot,
        boot_p50_ns: nearestRank(boot, 50),
        boot_max_ns: Math.max(...boot),
        boot_min_ns: Math.min(...boot),
        dispose_p50_ns: dispose.length ? nearestRank(dispose, 50) : null,
        dispose_max_ns: dispose.length ? Math.max(...dispose) : null,
        peak_memory_bytes: Math.max(...rows.map((row) => row.memoryBytes)),
      };
    }),
    loadavg_start: loadStart,
    loadavg_end: loadEnd,
    descriptive_only: true,
  };
}

// ---------------------------------------------------------------------------------------------
// The one timed invocation.
// ---------------------------------------------------------------------------------------------

const rebuildGit = (...args) => execFileSync("git", ["-C", REBUILD_ROOT, ...args], { encoding: "utf8" }).trim();
const rebuildTrackedClean = () => rebuildGit("status", "--porcelain=v1", "--untracked-files=no") === "";

function rebuildReport(records) {
  const ms = (ns) => (ns / 1e6).toFixed(3);
  const ratio = (ns) => (ns / REBUILD_BUDGET_NS).toFixed(1);
  const mib = (bytes) => (bytes / 1048576).toFixed(1);
  const lines = [
    "# Browser session rebuild cost (#1289, B1 of #1269)",
    "",
    `Module \`${records[0].module_sha256}\` at \`${records[0].candidate_commit}\`, Node ${records[0].node_version} `
      + `(V8 ${records[0].v8_version}, ${records[0].node_flags.join(" ")}), `
      + `${records[0].measurement_control}.`,
    "",
    `Time of \`miso_engine_web_v1_boot\` on a fresh instance, ${REBUILD_OBSERVATIONS} boots per document per round; `
      + `one quantum's budget is ${Q} / ${RATE} s = ${ms(REBUILD_BUDGET_NS)} ms. Descriptive only.`,
    "",
    "| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |",
    "|---|---|---|---|---|---|---|---|---|",
  ];
  for (const record of records) {
    for (const doc of record.documents) {
      lines.push(`| ${doc.workload_kind} | ${doc.tracks} | ${record.round} | ${ms(doc.boot_p50_ns)} | `
        + `${ms(doc.boot_max_ns)} | ${ratio(doc.boot_p50_ns)} | ${ratio(doc.boot_max_ns)} | `
        + `${ms(doc.dispose_p50_ns)} | ${mib(doc.peak_memory_bytes)} |`);
    }
  }
  lines.push("");
  return `${lines.join("\n")}`;
}

function rebuildRun(workdir, outdir, cpu, control) {
  const outputs = Object.fromEntries(REBUILD_OUTPUTS.map((name) => [name, path.join(outdir, name)]));
  // Overwrite refusal and persistence, before anything is launched.
  for (const file of Object.values(outputs)) {
    // `lstat`, so a dangling symlink counts as present.
    let present = true;
    try { lstatSync(file); } catch { present = false; }
    if (present) throw new Error(`refusing to overwrite ${file}`);
  }
  if (!rebuildTrackedClean()) throw new Error("the timed run requires unmodified tracked files");
  const provenance = JSON.parse(readFileSync(path.join(workdir, "provenance.json"), "utf8"));
  const modulePath = path.join(workdir, "host_web.wasm");
  const commit = rebuildGit("rev-parse", "--verify", "HEAD");
  if (provenance.commit !== commit) {
    throw new Error(`the module was prepared at ${provenance.commit}, not at HEAD ${commit}; run prepare again`);
  }
  if (rebuildSha256(readFileSync(modulePath)) !== provenance.module_sha256) {
    throw new Error("host_web.wasm changed after prepare recorded it");
  }
  if (!/^[0-9]+$/.test(cpu) || typeof control !== "string" || control === "") throw new Error("rebuild-run: CPU must be a number and CONTROL nonempty");
  mkdirSync(outdir, { recursive: true, mode: 0o755 });
  writeFileSync(outputs[REBUILD_STDERR], "", { flag: "wx" });

  const records = [];
  let failure = null;
  for (const name of ["warmup", "1", "2"]) {
    const child = spawnSync("taskset", ["-c", cpu, process.execPath, "--no-liftoff", REBUILD_SCRIPT, "rebuild-round", modulePath, name],
      { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
    writeFileSync(outputs[REBUILD_STDERR], child.stderr ?? "", { flag: "a" });
    if (child.status !== 0) {
      failure = `the ${name} launch failed (status ${child.status}); see ${REBUILD_STDERR}`;
      break;
    }
    if (name === "warmup") continue;
    const record = JSON.parse(child.stdout);
    records.push({
      ...record, candidate_commit: commit, prepared_commit: provenance.commit,
      measurement_control: control, cpu_affinity: cpu,
    });
  }
  if (failure === null && !rebuildTrackedClean()) failure = "a tracked file changed while the rounds ran";
  if (failure === null && rebuildGit("rev-parse", "--verify", "HEAD") !== commit) failure = "HEAD moved while the rounds ran";
  if (failure === null && rebuildSha256(readFileSync(modulePath)) !== provenance.module_sha256) {
    failure = "host_web.wasm changed while the rounds ran";
  }
  const reasons = failure === null ? rebuildRefusalReasons(records) : [failure];
  const jsonl = records.map((record) => `${JSON.stringify(record)}\n`).join("");
  writeFileSync(outputs[REBUILD_VERDICT], `${JSON.stringify({ accepted: reasons.length === 0, reasons })}\n`, { flag: "wx" });
  if (reasons.length !== 0) {
    if (jsonl !== "") writeFileSync(outputs[REBUILD_REFUSED], jsonl, { flag: "wx" });
    throw new Error(`the run is refused: ${reasons.join("; ")}`);
  }
  writeFileSync(outputs[REBUILD_RECORD], jsonl, { flag: "wx" });
  writeFileSync(outputs[REBUILD_REPORT], rebuildReport(records), { flag: "wx" });
  process.stdout.write(`${outputs[REBUILD_RECORD]}\n`);
}

// ---------------------------------------------------------------------------------------------

function rebuildMain([command, ...rest]) {
  if (command === "rebuild-preflight" && rest.length === 1) {
    const record = rebuildRound(rest[0], "warmup", 0);
    JSON.parse(JSON.stringify(record));
    process.stdout.write(`${JSON.stringify({ mode: command, documents: record.preflight })}\n`);
    return 0;
  }
  if (command === "rebuild-round" && rest.length === 2 && Object.hasOwn(REBUILD_ROUNDS, rest[1])) {
    process.stdout.write(`${JSON.stringify(rebuildRound(rest[0], rest[1], REBUILD_OBSERVATIONS))}\n`);
    return 0;
  }
  if (command === "rebuild-run" && rest.length === 4) {
    rebuildRun(path.resolve(rest[0]), path.resolve(rest[1]), rest[2], rest[3]);
    return 0;
  }
  if (command === "rebuild-validate" && rest.length === 1) {
    const records = readFileSync(rest[0], "utf8").split("\n").filter((line) => line !== "")
      .map((line) => JSON.parse(line));
    const reasons = rebuildRefusalReasons(records);
    process.stdout.write(`${JSON.stringify({ accepted: reasons.length === 0, reasons })}\n`);
    return reasons.length === 0 ? 0 : 1;
  }
  process.stderr.write("usage: web-mixing-automation-benchmark.mjs rebuild-preflight MODULE.wasm "
    + "| rebuild-round MODULE.wasm warmup|1|2 | rebuild-run WORKDIR OUTDIR CPU CONTROL "
    + "| rebuild-validate RECORDS.jsonl\n");
  return 2;
}
if (process.argv[2]?.startsWith("rebuild-")) process.exit(rebuildMain(process.argv.slice(2)));

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

// A checked-in fixture as the browser boots it: its one source stretched so the tone never ends,
// and nothing else changed.
function loadDocument(fixtureId) {
  const text = readFileSync(new URL(fixtureId, ROOT), "utf8");
  const parsed = JSON.parse(text);
  assert.equal(parsed.sources.length, 1, `${fixtureId}: one source`);
  assert.deepEqual(parsed.automation, [], `${fixtureId}: no automation`);
  const frames = `"frames": "${parsed.sources[0].frames}"`;
  assert.equal(text.split(frames).length, 2, `${fixtureId}: exactly one source length to stretch`);
  return {
    fixture: parsed,
    bytes: new TextEncoder().encode(text.replace(frames, `"frames": "${SOURCE_FRAMES}"`)),
    sourceId: new TextEncoder().encode(parsed.sources[0].id),
  };
}

const mixingDocument = loadDocument(FIXTURE_ID);
const fixture = mixingDocument.fixture;

// Each base is the value the booted document holds, as the native row reads it from the model.
// Decision 12 (#1096): the control table addresses the record's live racks -- `3` a console slot by
// its index in the session's slot order (`pre_insert`, then `post_insert`), which is its index in
// the track's `console` entries, and `1` an insert by its index in the track's `inserts`. A console
// slot's effect is declared once on the session and its knobs are the track's entry.
const RACK_INSERTS = constant("racks", "inserts");
const RACK_CONSOLE = constant("racks", "console");
function liveSlot(document, track, rack, index) {
  if (rack === RACK_INSERTS) {
    const effect = track.inserts.effects[index];
    return { id: effect.id, effect_id: effect.identity.effect_id, params: effect.params };
  }
  const slot = [...document.console.pre_insert, ...document.console.post_insert][index];
  const entry = track.console[index];
  assert.equal(entry.slot, slot.slot, `${track.id}: console entries follow the slot order`);
  return { id: slot.slot, effect_id: slot.identity.effect_id, params: entry.params };
}
for (const control of table.controls) {
  const track = fixture.tracks[control.track_index];
  assert.equal(track.id, control.track_id, `${control.track_id}: track index`);
  assert.ok([RACK_INSERTS, RACK_CONSOLE].includes(control.rack),
    `${control.track_id}: a live rack`);
  const slot = liveSlot(fixture, track, control.rack, control.effect_index);
  assert.equal(slot.id, control.slot_id, `${control.track_id}: slot`);
  assert.equal(slot.effect_id, control.effect, `${control.track_id}: effect`);
  const held = slot.params.filter((param) => param.parameter_id === control.parameter_id);
  assert.ok(held.length > 0, `${control.track_id}: the document holds the parameter`);
  for (const param of held) {
    assert.equal(Math.fround(param.value), Math.fround(control.base), `${control.track_id}: held base`);
  }
}

// The three console-strip documents (#1085, #1228), as the native rows state them. Each fixture is checked
// against the table's facts from its own JSON: every track's effects, section by section in strip
// order, in the console vocabulary of `console_workload::Workload::strip_layout` (decision 12,
// #1093: the session's `pre_insert` slots, the track's `inserts`, then the `post_insert` slots),
// and which tracks bypass them.
const DOCUMENT_KINDS = [
  "sixty_four_track_console", "sixty_four_track_app_shape", "sixty_four_track_console_sends",
];
const SHORT_NAMES = {
  "miso.parametric-eq": "eq", "miso.compressor": "compressor", "miso.true-peak-limiter": "limiter",
};
const shortName = (effect) => SHORT_NAMES[effect.effect_id] ?? "other";
// A track's strip as `[section, effects]` in strip order, each effect `{ effect_id, bypass }`. A
// console slot's effect is declared once on the session; its bypass is the track's entry.
function trackSections(document, track) {
  const { pre_insert: pre, post_insert: post } = document.console;
  const declared = new Map([...pre, ...post].map((slot) => [slot.slot, slot.identity.effect_id]));
  const entries = (slots) => {
    const names = new Set(slots.map((slot) => slot.slot));
    return track.console.filter((entry) => names.has(entry.slot))
      .map((entry) => ({ effect_id: declared.get(entry.slot), bypass: entry.bypass }));
  };
  const inserts = track.inserts.effects
    .map((effect) => ({ effect_id: effect.identity.effect_id, bypass: effect.bypass }));
  return [["pre_insert", entries(pre)], ["inserts", inserts], ["post_insert", entries(post)]];
}
const trackEffects = (document, track) => trackSections(document, track).flatMap(([, effects]) => effects);
function trackLayout(document, track) {
  const sections = trackSections(document, track).filter(([, effects]) => effects.length > 0)
    .map(([name, effects]) => `${name}:${effects.map(shortName).join("+")}`);
  return sections.length === 0 ? "builtins" : sections.join(",");
}
function bypassCensus(document) {
  let any = false;
  let exact = true;
  let bypassed = 0;
  document.tracks.forEach((track, index) => {
    const effects = trackEffects(document, track);
    const some = effects.some((effect) => effect.bypass);
    const every = effects.length > 0 && effects.every((effect) => effect.bypass);
    any ||= some;
    bypassed += every ? 1 : 0;
    exact &&= every === (index % 3 === 2) && some === every;
  });
  return { pattern: !any ? "none" : exact ? "index_mod_3_is_2" : "other", bypassed };
}
assert.deepEqual((table.documents ?? []).map((doc) => doc.workload_kind), DOCUMENT_KINDS,
  "the table lists the three console-strip documents");
const DOCUMENTS = table.documents.map((doc) => {
  const loaded = loadDocument(doc.fixture_id);
  const document = loaded.fixture;
  assert.equal(document.tracks.length, doc.tracks, `${doc.workload_kind}: track count`);
  for (const track of document.tracks) {
    assert.equal(trackLayout(document, track), doc.strip_layout, `${doc.workload_kind} ${track.id}: layout`);
    const content = trackEffects(document, track).map(shortName).join("+");
    assert.equal(content, doc.strip_content, `${doc.workload_kind} ${track.id}: content`);
  }
  const census = bypassCensus(document);
  assert.equal(census.pattern, doc.bypass_pattern, `${doc.workload_kind}: bypass pattern`);
  assert.equal(census.bypassed, doc.bypassed_tracks, `${doc.workload_kind}: bypassed tracks`);
  return { doc, loaded };
});

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

function boot(document = mixingDocument) {
  const documentBytes = document.bytes;
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
  // The live controls with the default command queue, and no meters, observation or master, as
  // the native row prepares its plan (`control: true`, nothing else).
  u64("liveControlCommandQueueRecords", BigInt(COMMAND_QUEUE_RECORDS));
  u64("liveControlMeterBlocks", 0n);
  u64("liveControlObservationTaps", 0n);
  u64("liveControlMasterTrackPlusOne", 0n);
  const pointer = e.miso_engine_web_v1_document_ptr(documentBytes.byteLength);
  new Uint8Array(e.memory.buffer, pointer, documentBytes.byteLength).set(documentBytes);
  const handle = e.miso_engine_web_v1_boot(documentBytes.byteLength);
  if (handle === 0) {
    const bytes = e.miso_engine_web_v1_boot_diagnostic_bytes();
    throw new Error(`boot ${e.miso_engine_web_v1_boot_result()}: ${new TextDecoder().decode(
      new Uint8Array(e.memory.buffer, pointer, bytes))}`);
  }
  // The wire's track index is the booted live controls' track order; check the table addresses it.
  // The console-strip documents carry no control traffic, so only the mixing document is checked.
  for (const control of document === mixingDocument ? table.controls : []) {
    const length = e.miso_engine_web_v1_live_control_track_id(handle, control.track_index);
    const idPointer = e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID);
    const id = new TextDecoder().decode(new Uint8Array(e.memory.buffer, idPointer, length));
    assert.equal(id, control.track_id, `${control.track_id}: live-control track index`);
  }
  return {
    e, handle, control: preparedControl(e, handle), frame: 0n, block: 0, sourceId: document.sourceId,
    outputPointer: e.miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT),
    left: new Float32Array(Q), right: new Float32Array(Q),
  };
}

// The tone: one continuous sine on the fixture's one source. The mono fixture maps source channel
// 0 onto both channels of every track, so the right plane is carried and never read; the stereo
// documents read it, and it is the native tone's right channel, the left scaled by -0.75.
function feed(engine) {
  const { sourceId } = engine;
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

// One console-strip document (#1085): booted as written, the tone fed, the pre-roll rendered, and
// no control traffic, ever. The engine must report one live route per route into a submix, and the
// sends document must have some (#1228): live and static routes render the same bits, so no digest
// would show a document booted without live controls, the native row's static path.
const SENDS_KIND = "sixty_four_track_console_sends";
function makeDocument({ doc, loaded }) {
  const engine = boot(loaded);
  const submixRoutes = loaded.fixture.routes
    .filter((route) => route.destination.kind === "submix_input").length;
  assert.ok(doc.workload_kind !== SENDS_KIND || submixRoutes > 0,
    `${doc.workload_kind}: routes into a submix`);
  assert.equal(engine.e.miso_engine_web_v1_live_control_route_count(engine.handle), submixRoutes,
    `${doc.workload_kind}: one live route per route into a submix`);
  const state = { doc, engine, audible: false };
  for (let i = 0; i < LEAD_BLOCKS; i++) feed(engine);
  for (let i = 0; i < PREROLL; i++) {
    feed(engine);
    render(state);
  }
  return state;
}

// Folds a document's block into its digest and notes whether it was audible. Outside the clock.
function absorbDocument(state, hash) {
  absorb(state, hash);
  const output = new Float32Array(state.engine.e.memory.buffer, state.engine.outputPointer, 2 * Q);
  state.audible ||= output.some((word) => word !== 0);
}

// Asserts every pair of documents rendered different bits, or one session was booted twice; a
// failure names the pair.
function assertDocumentsDistinct(digests, phase) {
  DOCUMENT_KINDS.forEach((first, i) => DOCUMENT_KINDS.slice(i + 1).forEach((second) => {
    assert.notEqual(digests[first], digests[second],
      `${phase}: the documents ${first} and ${second} rendered the same bits`);
  }));
}

// The documents' premises, over the preflight blocks: each renders audible bits, and every two
// render different ones.
function documentPreflight() {
  const states = DOCUMENTS.map(makeDocument);
  const hashes = states.map(() => createHash("sha256"));
  for (let block = 0; block < PREFLIGHT_BLOCKS; block++) {
    states.forEach((state, index) => {
      feed(state.engine);
      render(state);
      absorbDocument(state, hashes[index]);
    });
  }
  const digests = Object.fromEntries(states.map((state, index) => [
    state.doc.workload_kind, hashes[index].digest("hex"),
  ]));
  for (const state of states) assert.ok(state.audible, `preflight ${state.doc.workload_kind}: silent`);
  assertDocumentsDistinct(digests, "preflight");
  return digests;
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
const documentPreflightDigests = documentPreflight();
if (mode === "preflight") {
  process.stdout.write(`${JSON.stringify({
    mode, module_sha256: moduleSha256, digests: preflightDigests, documents: documentPreflightDigests,
  })}\n`);
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
// The console-strip documents (#1085), booted once the arms are timed and timed the same way,
// alternated per observation, with the render call alone inside the clock.
const documents = DOCUMENTS.map(makeDocument);
const documentSamples = documents.map(() => new Array(OBSERVATIONS));
const documentHashes = documents.map(() => createHash("sha256"));
for (let observation = 0; observation < OBSERVATIONS; observation++) {
  documents.forEach((state, index) => {
    feed(state.engine);
    const start = process.hrtime.bigint();
    const result = state.engine.e.miso_engine_web_v1_render(state.engine.handle, Q);
    const elapsed = process.hrtime.bigint() - start;
    assert.equal(result, RESULT_OK, "render");
    state.engine.block += 1;
    documentSamples[index][observation] = Number(elapsed);
  });
  documents.forEach((state, index) => absorbDocument(state, documentHashes[index]));
}
const loadEnd = readFileSync("/proc/loadavg", "utf8").trim();
const documentDigests = documentHashes.map((hash) => hash.digest("hex"));
for (const state of documents) assert.ok(state.audible, `run ${state.doc.workload_kind}: silent`);
assertDocumentsDistinct(Object.fromEntries(documents.map((state, index) => [
  state.doc.workload_kind, documentDigests[index],
])), "run");
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
  documents: documents.map((state, index) => ({
    workload_kind: state.doc.workload_kind,
    fixture_id: state.doc.fixture_id,
    tracks: state.doc.tracks,
    strip_content: state.doc.strip_content,
    strip_layout: state.doc.strip_layout,
    input_signal: state.doc.input_signal,
    bypass_pattern: state.doc.bypass_pattern,
    bypassed_tracks: state.doc.bypassed_tracks,
    ...Object.fromEntries([50, 95, 99].map((p) => [`p${p}_ns`, nearestRank(documentSamples[index], p)])),
    output_sha256: documentDigests[index],
  })),
  bit_identity: "quiet == restated, asserted in-run",
  // host_web.wasm exports no collapse counter; the native row states them.
  bank_collapse_counters_exported: false,
  loadavg_start: loadStart,
  loadavg_end: loadEnd,
  descriptive_only: true,
  statistical_method: "three arms alternated per observation, then the three documents alternated per observation; one warmup launch and two measured launches; nearest-rank percentiles over per-block nanoseconds of miso_engine_web_v1_render alone; ramp delta is automated minus restated and collapse delta is restated minus quiet, per observation; descriptive only; no threshold",
};
process.stdout.write(`${JSON.stringify(record)}\n`);
