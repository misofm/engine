import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { cp, mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";
import { chromium, firefox, webkit } from "playwright";
import { renderMatrix } from "./generate-matrix.mjs";
import { checkSessionIdentities } from "./session-identities.mjs";
import { ARTIFACT_NAMES, exactArtifacts, startQualificationServer } from "./server.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const HOST_WEB = path.dirname(HERE);
const RESULTS_PATH = path.join(HERE, "results.json");
const MATRIX_PATH = path.join(HOST_WEB, "BROWSER_DEPLOYMENT_MATRIX.md");
const WASM_ARTIFACT = "miso-engine-v1-audio-worklet.simd128.wasm";
const CANONICAL_COMMIT = /^[0-9a-f]{40}$/;
const PLAYWRIGHT_VERSION = JSON.parse(
  await readFile(path.join(HERE, "node_modules", "playwright", "package.json"), "utf8"),
).version;
const ENGINES = { chromium, firefox, webkit };
const MUTATIONS = [
  "attestation", "boot", "native-corpus-digest", "main-thread-stall",
  // Issue #137 E8/E6: the live-control row and the live-control load carried across the stall.
  "control-path-applied", "control-path-meter", "control-path-command", "stall-live-controls-load",
  // Issue #143 E12: the observation row. `observation-armed` is the eval's named red mutation --
  // a run whose armed tap published nothing, which is exactly what a browser that lost the
  // transport would produce.
  "observation-armed", "observation-disarmed", "observation-identity", "observation-window",
  "sdk-observation-window-reversed", "sdk-observation-window-malformed",
  "sdk-response", "sdk-observation", "sdk-spectrum", "sdk-spectrum-hop-missing",
  "sdk-spectrum-hop-wrong", "sdk-spectrum-recovery-loss",
  // Issue #1106: a hop probe whose audio start was not delayed, one that never waited, one that
  // spun, and a never-publishing wait that outlived its deadline.
  "sdk-spectrum-hop-wait-fast-start", "sdk-spectrum-hop-wait-early", "sdk-spectrum-hop-wait-spin",
  "sdk-spectrum-hop-wait-never-unbounded",
  "sdk-spectrum-recovery-unavailable", "sdk-spectrum-recovery-identity",
  // Issue #1097 gate 4: a live bypass that reached no instance, or the wrong one.
  "sdk-live-bypass-console", "sdk-live-bypass-insert", "sdk-live-bypass-address",
  // Issues #1341a1 and #1341a2: a live bypass is a ramp over the session's mute length, so a step,
  // a ramp that never settles and a lift that never lands are red too.
  "sdk-live-bypass-step", "sdk-live-bypass-unsettled", "sdk-live-bypass-lift",
  // Issue #1333 D3: one render-locked allocator call on one instance, and a staging-read workload
  // whose spectrum read never completed a window.
  "render-allocations", "staging-reads",
  // Issue #1476 D5: one SDK instance's allocator call, and one SDK instance closed without its read.
  "sdk-render-allocations", "sdk-render-allocations-missing",
  // Batch-verdict 2, B2 (root 2026-10-12): a real AudioContext resumed outside a trusted click, and
  // a harness that clicked fewer or more controls than the page declared.
  "resume-clicks-direct", "resume-clicks-missing", "resume-clicks-extra",
  // Issue #1530 gate 4: the browser observation wait, one predicate each.
  "sdk-observation-wait-unbounded", "sdk-observation-wait-late", "sdk-observation-wait-early",
  "sdk-observation-wait-untyped", "sdk-observation-wait-rendered",
  "sdk-observation-wait-after-render", "sdk-observation-wait-resend",
];

// The total time the page may take, from the first evaluate to the result, clicks included. A page
// that is stuck (WebKit can wedge a page whose context stalls under full CPU load) fails by name here
// instead of waiting on a locator.
const PAGE_DEADLINE_MS = 300_000;

// Issue #1476 D4: every SDK engine `sdk-response-entry.ts` creates, in the order its count is
// read (with repeats). A new SDK instance adds its label here.
const SDK_RENDER_ALLOCATION_WORKLOADS = Object.freeze([
  "resident-observation",
  "observation-wait-deadline",
  "observation-wait-render",
  "track-response-subscription",
  "sdk-observation",
  "spectrum-query:trackPostInput:track",
  "spectrum-query:trackPostPan:track",
  "spectrum-query:output:main-out",
  "spectrum-query-closed",
  "spectrum-continuous-hop-256",
  "spectrum-collection",
  "spectrum-continuous-hop-1024",
  "live-bypass:authored-none",
  "live-bypass:authored-desk-hi",
  "live-bypass:authored-ins-mid",
  "live-bypass:authored-none:live-desk-hi-bypass",
  "live-bypass:authored-none:live-ins-mid-bypass",
  "live-bypass:authored-desk-hi:live-desk-hi-lift",
  "live-bypass:authored-ins-mid:live-ins-mid-lift",
]);

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

function gate(browserName, name, condition, detail) {
  if (!condition) throw new Error(`${browserName}: ${name}: ${detail}`);
}

const MAX_U64 = 18_446_744_073_709_551_615n;

function canonicalU64(value) {
  if (typeof value !== "string" || value.length > 20 || !/^(0|[1-9][0-9]*)$/.test(value)) {
    return undefined;
  }
  const parsed = BigInt(value);
  return parsed <= MAX_U64 ? parsed : undefined;
}

function validResidentWindow(window) {
  if (window === null || typeof window !== "object") return false;
  const firstSample = canonicalU64(window.firstSample);
  const endSample = canonicalU64(window.endSample);
  const sequence = canonicalU64(window.sequence);
  return firstSample !== undefined && endSample !== undefined && sequence !== undefined
    && endSample > firstSample && window.blocks === 2;
}

async function sha256(file) {
  return createHash("sha256").update(await readFile(file)).digest("hex");
}

async function buildSdkBundle(sdkRoot) {
  const esbuildPath = pathToFileURL(path.join(sdkRoot, "node_modules", "esbuild", "lib", "main.js")).href;
  const { build } = await import(esbuildPath);
  const bundle = await mkdtemp(path.join(os.tmpdir(), "miso-sdk-response-bundle-"));
  // A selected built distribution (including an unpacked npm archive) supplies every SDK runtime
  // import. The existing source-only CI mode remains available before package assembly.
  const sourceRoot = path.resolve(HERE, "../../../sdk/src");
  const distribution = path.join(sdkRoot, "dist");
  const useDistribution = await readdir(distribution).then((entries) => entries.includes("core"), () => false);
  const repositorySdk = sdkRoot === path.dirname(sourceRoot);
  if (!useDistribution && !repositorySdk) {
    throw new Error("selected SDK package has no built distribution");
  }
  // Say which SDK this run qualifies (#1097 verdict L5). A leftover `sdk/dist` -- from
  // `sdk-package.sh check`, say -- silently switches the repository's own SDK from CI's source
  // bundle to whatever that build left behind, so the evidence must name the bundle it used.
  process.stdout.write(!useDistribution
    ? `sdk bundle: the source at ${sourceRoot} (CI's mode)\n`
    : repositorySdk
      ? `sdk bundle: the built distribution at ${distribution}, not CI's source bundle; delete it `
        + "to qualify the source as CI does\n"
      : `sdk bundle: the selected package's built distribution at ${distribution}\n`);
  const shippedSdk = {
    name: "shipped-sdk",
    setup(builder) {
      builder.onResolve({ filter: /\.ts$/ }, (args) => {
        const resolved = path.resolve(args.resolveDir, args.path);
        if (!resolved.startsWith(`${sourceRoot}${path.sep}`)) return undefined;
        return { path: path.join(distribution, path.relative(sourceRoot, resolved).replace(/\.ts$/, ".js")) };
      });
    },
  };
  try {
    await build({
      entryPoints: [path.join(HERE, "sdk-response-entry.ts")],
      plugins: useDistribution ? [shippedSdk] : [],
      outfile: path.join(bundle, "sdk-response-client.js"),
      bundle: true,
      format: "esm",
      platform: "browser",
      target: "es2022",
      legalComments: "inline",
    });
    await build({
      entryPoints: [useDistribution
        ? path.join(distribution, "browser", "response-worker.js")
        : path.join(sourceRoot, "browser", "response-worker.ts")],
      outfile: path.join(bundle, "response-worker.js"),
      bundle: true,
      format: "esm",
      platform: "browser",
      target: "es2022",
      legalComments: "inline",
    });
    return bundle;
  } catch (error) {
    await rm(bundle, { recursive: true, force: true });
    throw error;
  }
}

function validate(browserName, result) {
  gate(browserName, "result-schema", result?.schema === "miso.web.qualification.result.v1",
    "unexpected browser result");
  gate(browserName, "secure-context", result.secureContext === true,
    "qualification did not run in a secure context");
  const supported = result.attestation?.probe === true;
  const typedUnsupported = result.attestation?.probe === false
    && result.attestation?.outcome === "miso.unsupported.v1"
    && result.attestation?.typedRefusal === true;
  gate(browserName, "attestation", supported || typedUnsupported,
    "expected simd128 support or exact miso.unsupported.v1 refusal");
  if (!supported) return "miso.unsupported.v1";
  gate(browserName, "attestation", result.attestation.outcome === "simd128",
    "simd128 probe and outcome disagree");
  gate(browserName, "AudioWorklet-boot", result.boot?.ready === true
    && result.boot?.backend === "simd128", "module did not instantiate and report ready");
  const corpus = result.corpus;
  gate(browserName, "native-corpus-digest", corpus?.nativeDigest === corpus?.shippedArtifactDigest
    && Array.isArray(corpus?.browserDigests)
    && corpus.browserDigests.length === 2
    && corpus.browserDigests.every((digest) => digest === corpus.nativeDigest)
    && corpus.freshContextIdentity === true, "browser PCM differs from the native corpus pin");
  // #137 E8: one parameter change reached the DSP and did exactly what it declared, and the
  // decimated meter stream produced frames of the declared width whose master peak is a real
  // observation of the rendered output rather than a zero.
  const live = result.liveControls;
  gate(browserName, "control-path", live?.commandOk === true && live?.commandPath === "live"
    && live?.commandApplied === 4
    && Array.isArray(live?.tracks) && live.tracks.length === 1,
  "the live-control SET_CONTROL transaction was not applied on the live path");
  gate(browserName, "control-path", live?.exactRetargetedOutput === true
    && live?.renderedDigest === live?.expectedDigest,
  "the applied parameter change did not produce the exact declared output");
  gate(browserName, "control-path", live?.metersAttached === true
    && live?.meterLeaseResult === 0 && live?.telemetryLeaseResult === 0
    && live?.meterFrames >= 1 && live?.meterFrameWidth === 4
    && live?.masterPeak > 0 && live?.masterPeak <= live?.inputPeak,
  "no usable meter frame arrived while the lease was held");
  gate(browserName, "control-path", live?.telemetryFrames >= 1,
    "no render-telemetry frame arrived over a full window");

  // Issue #143 E12: subscribe -> nonzero `trackGrDb`, an unarmed tap publishes none, with `firstSample`
  // monotonic and the windows tiling. The two runs render the *same* sixteen blocks, so their
  // audio must be identical: arming a declared tap may not move a sample.
  const observation = result.observation;
  gate(browserName, "observation", observation?.armed?.meterLeaseResult === 0
    && observation?.armed?.subscribeResult === 0 && observation?.armed?.subscribeReason === 0
    && observation?.armed?.bindings === 1 && observation?.armed?.frameSlot === 0
    && observation?.armed?.windowBlocks === 2,
  "the subscription was not acknowledged with a usable map");
  gate(browserName, "observation", observation?.armed?.frames >= 1
    && observation?.armed?.trackGrDbWidth === 1
    && observation?.armed?.peakWidth === 4
    && observation?.armed?.everyValueFinite === true,
  "no usable observation frame arrived while the tap was armed");
  gate(browserName, "observation-armed", observation?.armed?.maximumTrackGrDb > 0,
    "an armed tap published no reduction at all");
  gate(browserName, "observation", observation?.armed?.masterPresent === true
    && observation?.armed?.masterMatchesTrack === true,
  "the designated master did not report the track's own reading");
  gate(browserName, "observation-window", observation?.armed?.firstSampleMonotonic === true
    && observation?.armed?.windowsTile === true,
  "observation windows did not advance monotonically and tile");
  // A tap that was never armed publishes none. (That an unsubscribe stops a tap's reading is the
  // Rust test `a_withdrawn_tap_leaves_the_first_polled_frame_and_only_its_own_reading`'s, #1345
  // Amendment 7.)
  gate(browserName, "observation-disarmed", observation?.disarmed?.maximumTrackGrDb === 0,
    "a tap that was never armed published a reduction");
  gate(browserName, "observation-identity", observation?.identicalAudio === true,
    "arming a declared tap moved a rendered sample");
  // Every real `AudioContext.resume()` ran inside a trusted click (the page counts them, split by
  // `window.event`) and the harness clicked exactly the controls the page declared: no direct
  // `resume()`, no missing click, no extra one (batch-verdict 2, B2).
  const resumes = result.resumes;
  gate(browserName, "resume-clicks", resumes?.direct === 0
    && Number.isInteger(resumes?.declared)
    && resumes.declared === (result.sdkResponse === null ? 0 : 2)
    && resumes.inClick === resumes.declared
    && resumes.planned === resumes.declared
    && resumes.clicksPerformed === resumes.declared,
  `a real AudioContext was resumed outside a trusted click, the page did not declare the two SDK clicks (none without the SDK), or the harness did not click exactly the declared controls: ${JSON.stringify(resumes)}`);

  const stall = result.stall;
  // #137 E6: the frozen stall requirements are unchanged, and they are now met with the control
  // path and the meter fold both live across the fault.
  gate(browserName, "main-thread-stall", stall?.liveControlCommandOk === true
    && stall?.liveControlMeterLeaseResult === 0 && stall?.liveControlMeterFrames >= 1,
  "the stall did not carry a live command and meter load");
  gate(browserName, "main-thread-stall", stall?.minimumStallMs === 100
    && stall?.requestedStallMs >= 100
    && stall?.measuredStallMs >= 100
    && stall?.ringFrames === 5120
    && stall?.renderedFrames === 5120
    && stall?.nextAbsoluteSample === "5120"
    && stall?.renderedQuanta === "40"
    && stall?.noDropout === true
    && stall?.noDesync === true
    && stall?.renderedDigest === stall?.expectedDigest,
    "100 ms fault did not preserve exact 5,120-frame output");
  // Issue #1333 D3 / Amendment 1: the staging reads ran over real data -- a completed one-shot
  // window and a completed stream window from a collection's selected entry, an observation row
  // and a track response -- so the render-allocation gate below covers each of them.
  const oneShot = result.stagingReads?.oneShot;
  const streamRun = result.stagingReads?.stream;
  gate(browserName, "staging-reads", oneShot?.selectResult === 0
    && oneShot?.subscribeResult === 0
    && oneShot?.spectrumReadResult === 0 && oneShot?.spectrumReadBytes > 0
    && oneShot?.observationReadResult === 0 && oneShot?.observationRows === 1
    && oneShot?.trackResponseResult === 0 && oneShot?.trackResponseBytes > 0
    && oneShot?.streamStartResult === 0
    && (oneShot?.streamReadResult === 0 || oneShot?.streamReadResult === 6),
  `the one-shot staging reads did not all complete: ${JSON.stringify(oneShot)}`);
  gate(browserName, "staging-reads", streamRun?.selectResult === 0
    && streamRun?.streamStartResult === 0
    && streamRun?.streamReadResult === 0 && streamRun?.streamReadBytes > 0,
  `the stream staging read did not return a completed window: ${JSON.stringify(streamRun)}`);
  // Issue #1333 D3: every booted instance, read before its dispose, made no allocator call in any
  // export its processor calls on the render thread after boot -- including everything the plan
  // executor does behind its `call_indirect`, which the static gate cannot see.
  const allocations = result.renderAllocations;
  gate(browserName, "render-allocations", Array.isArray(allocations)
    && allocations.length === 10
    && allocations.every((row) => typeof row?.workload === "string" && row.count === 0),
  `a worklet instance allocated on its render thread: ${JSON.stringify(allocations)}`);
  if (result.sdkResponse !== null) validateSdkResponse(browserName, result.sdkResponse);
  return "simd128 supported";
}

/**
 * Issue #1097 gate 4, as #1563B restates it and #1341a1/#1341a2 move it: a live bypass toggle on a
 * console slot and on an insert, through the SDK.
 *
 * Each of the four edits is a `SET_CONTROL` transaction the core applies on its live path before
 * the first quantum, naming its instance by its stable ID: the console slot `desk-hi` and the insert
 * `ins-mid`, control `0` (bypass). A live bypass is a crossfade over the session's `mute_ms` ramp
 * (the fixture declares 10 ms), so the render is a ramp between two controls, both rendered by the
 * same browser: the starting one (never bypassed, or the document that authors the bypass) and the
 * target (the other). Per quantum, strictly: the render equals the starting control until the ramp
 * reaches the output; then it differs from BOTH controls for as many quanta as the ramp spans,
 * `ceil(ramp / quantum)` or one more (a step spans at most one, a no-op none); and from there it
 * equals the target bit for bit. The ramp's start is found, not assumed, because the chain's own
 * delay (PDC) shifts it.
 */
function validateSdkLiveBypass(browserName, live) {
  const digest = (value) => typeof value === "string" && /^[0-9a-f]{64}$/.test(value);
  const rows = ["base", "consoleOff", "insertOff", "consoleOffLive", "insertOffLive",
    "consoleOnLive", "insertOnLive"];
  const frames = live?.blocks?.base?.length;
  gate(browserName, "sdk-live-bypass", ["base", "consoleOff", "insertOff"].every((key) => digest(live?.[key]))
    && live.baseEnergy > 0
    && new Set([live.base, live.consoleOff, live.insertOff]).size === 3
    && Number.isInteger(live.quantumFrames) && live.quantumFrames > 0
    && Number.isInteger(live.muteRampSamples) && live.muteRampSamples > 0
    && Number.isInteger(frames) && frames > 0
    && rows.every((key) => Array.isArray(live.blocks[key]) && live.blocks[key].length === frames
      && live.blocks[key].every(digest)),
  "the console and insert bypasses were not both audible and distinct renders");
  const rampBlocks = Math.ceil(live.muteRampSamples / live.quantumFrames);
  const blocks = live.blocks;
  // Why `rows[index]` fails to be the crossfade from `from` to `to`, or null when it is one.
  const crossfade = (rendered, from, to) => {
    const same = (a, b, at) => a[at] === b[at];
    const start = rendered.findIndex((_, at) => !same(rendered, from, at));
    if (start < 0) return "it never left its starting control";
    let ramp = 0;
    while (start + ramp < rendered.length && !same(rendered, to, start + ramp)) ramp += 1;
    for (let at = start; at < start + ramp; at += 1) {
      if (same(rendered, from, at)) return `block ${at} fell back to its starting control inside the ramp`;
    }
    if (ramp < rampBlocks || ramp > rampBlocks + 1) {
      return `${ramp} blocks sit between the controls, the ${live.muteRampSamples}-sample ramp spans ${rampBlocks} or ${rampBlocks + 1}`;
    }
    for (let at = start + ramp; at < rendered.length; at += 1) {
      if (!same(rendered, to, at)) return `block ${at} left the target after the ramp`;
    }
    return null;
  };
  const rowGate = (key, from, to, detail) => {
    const problem = crossfade(blocks[key], blocks[from], blocks[to]);
    gate(browserName, "sdk-live-bypass", problem === null, `${detail} (${problem})`);
  };
  rowGate("consoleOffLive", "base", "consoleOff",
    "a live bypass toggle on the post_insert console slot did not crossfade onto its authored bypass state");
  rowGate("insertOffLive", "base", "insertOff",
    "a live bypass toggle on the insert did not crossfade onto its authored bypass state");
  rowGate("consoleOnLive", "consoleOff", "base",
    "lifting the authored bypass of the post_insert console slot did not crossfade onto the document with none");
  rowGate("insertOnLive", "insertOff", "base",
    "lifting the authored bypass of the insert did not crossfade onto the document with none");
  gate(browserName, "sdk-live-bypass", live.consoleAddress === "console:desk-hi:0"
    && live.insertAddress === "inserts:ins-mid:0"
    && JSON.stringify(live.liftAddresses) === JSON.stringify(["console:desk-hi:0", "inserts:ins-mid:0"])
    && JSON.stringify(live.paths) === JSON.stringify(["live", "live", "live", "live"]),
  "the live bypass edits did not name the console/insert instances or take the core's live path");
}

/**
 * Issue #1530 gates 1-3: the browser SDK's wait on observation backpressure, against the shipped
 * worklet. `ok` is command result 0; `bp` is result 6 with reason 8.
 */
function validateSdkObservationWait(browserName, wait) {
  const deadline = wait?.deadline;
  const render = wait?.render;
  const log = (entries) => JSON.stringify(Array.isArray(entries)
    ? entries.map((entry) => [entry?.result, entry?.reason]) : null);
  const OK = [0, 0];
  const BP = [6, 8];
  const expected = (...entries) => JSON.stringify(entries);
  gate(browserName, "sdk-observation-wait", deadline?.settled === true
    && deadline.typed === true
    && deadline.errorCode === "backpressure"
    && Number.isFinite(deadline.elapsedMs) && deadline.elapsedMs >= 225 && deadline.elapsedMs < 4_000
    && deadline.renderedQuantaBefore === "0"
    && deadline.renderedQuantaAfterRefusal === "0"
    && deadline.handleOpenAfterRefusal === true
    && log(deadline.commandsBeforeRender) === expected(OK, BP),
  `a close on a context that did not render did not end in the typed backpressure error at the 250 ms deadline: ${
    JSON.stringify({ ...deadline, commands: undefined, commandsBeforeRender: log(deadline?.commandsBeforeRender) })}`);
  gate(browserName, "sdk-observation-wait", deadline?.closeAfterRender?.resolved === true
    && deadline.renderedQuantaAfterRender === "16"
    && log(deadline.commands) === expected(OK, BP, OK),
  `the close after the context rendered did not resolve with the rendered blocks reported: ${
    JSON.stringify({ closeAfterRender: deadline?.closeAfterRender,
      renderedQuantaAfterRender: deadline?.renderedQuantaAfterRender, commands: log(deadline?.commands) })}`);
  gate(browserName, "sdk-observation-wait", render?.refusalSeenBeforeRender === true
    && render.settled === true
    && render.resolved === true
    && Number.isFinite(render.elapsedMs) && render.elapsedMs < 10_000
    && render.renderedQuantaBefore === "0"
    && render.renderedQuantaAfterRender === "16"
    && log(render.commands) === expected(OK, BP, OK)
    && render.resendIdentical === true,
  `a refused close did not resolve by itself with one identical resend when the context rendered: ${
    JSON.stringify({ ...render, commands: log(render?.commands) })}`);
}

/**
 * Issue #1476 D4: every SDK engine, read before its close, made no allocator call in any export
 * its processor calls on the render thread -- including paths no raw workload reaches. Every
 * instance the entry created has exactly one row, under its frozen label.
 */
function validateSdkRenderAllocations(browserName, response) {
  const rows = response?.renderAllocations;
  gate(browserName, "sdk-render-allocations", Array.isArray(rows)
    && Number.isSafeInteger(response?.renderAllocationInstances)
    && rows.length === response.renderAllocationInstances,
  `an SDK instance closed without its render allocation read: ${
    Array.isArray(rows) ? rows.length : "no"} rows for ${response?.renderAllocationInstances} instances`);
  gate(browserName, "sdk-render-allocations",
    JSON.stringify(rows.map((row) => row?.workload)) === JSON.stringify(SDK_RENDER_ALLOCATION_WORKLOADS),
  `the SDK render allocation rows are not the frozen workload list: ${
    JSON.stringify(rows.map((row) => row?.workload))}`);
  const allocating = rows.filter((row) => row?.count !== 0);
  gate(browserName, "sdk-render-allocations", allocating.length === 0,
    `an SDK worklet instance allocated on its render thread: ${allocating
      .map((row) => `${row?.workload}=${row?.count}`).join(" ")}`);
}

function validateSdkResponse(browserName, response) {
  validateSdkRenderAllocations(browserName, response);
  validateSdkLiveBypass(browserName, response?.liveBypass);
  gate(browserName, "sdk-response", response?.capabilities?.some(
    (row) => row.owner === "effect" && row.target === "miso.parametric-eq",
  ) === true, "SDK did not expose the generated EQ capability");
  gate(browserName, "sdk-response", response?.capabilities?.some(
    (row) => row.owner === "builtins" && row.target === "inputFilters",
  ) === true, "SDK did not expose the generated input-filter capability");
  gate(browserName, "sdk-response", response?.eq?.configurationId === "9007199254740993"
    && response.eq.mode === "requestedConfiguration"
    && response.eq.points === 32 && response.eq.sections === 6
    && response.eq.left.length === 32 && response.eq.right.length === 32
    && response.eq.frequencies.length === 32
    && response.eq.enabledLeft.length === 6 && response.eq.enabledRight.length === 6
    && response.eq.enabledLeft.slice(4).every((enabled) => enabled === false)
    && response.eq.enabledRight.slice(4).every((enabled) => enabled === false)
    && response.eq.enabledLeft.filter(Boolean).length === 1
    && response.eq.enabledRight.filter(Boolean).length === 1,
  "SDK EQ Worker result did not preserve the explicit configuration and sections");
  gate(browserName, "sdk-response", response?.filters?.configurationId === "9007199254740994"
    && response.filters.points === 16 && response.filters.sections === 2
    && response.filters.firstFrequency === 0 && response.filters.lastFrequency === 24000
    && response.filters.left.length === 16 && response.filters.right.length === 16,
  "SDK input-filter Worker result did not preserve independent arrays and endpoints");
  gate(browserName, "sdk-response", response?.ownedAfterSecondQuery === true,
    "SDK response arrays were not stable after a subsequent query");
  const observations = response?.observations;
  const live = observations?.liveResponse;
  const liveMembers = Array.isArray(live?.members) ? live.members : [];
  const liveInputFilters = liveMembers.find((member) =>
    member.nativeId === "miso.builtin.input-filters"
      && member.stableId === "input-filters"
      && member.rack === "input"
      && member.kind === "inputFilters"
      && member.available === true,
  );
  const liveCompressor = liveMembers.find((member) =>
    member.nativeId === "miso.compressor"
      && member.kind === "unavailable"
      && member.available === false,
  );
  const liveU64 = (value) => typeof value === "string"
    && /^[0-9]+$/.test(value) && BigInt(value) > 0n;
  const liveMemberOrder = liveMembers.map((member) => [
    member.nativeId, member.stableId, member.rack, member.kind, member.available, member.bypassed,
  ]);
  const expectedLiveMemberOrder = [
    ["miso.builtin.input-filters", "input-filters", "input", "inputFilters", true, false],
    ["miso.parametric-eq", "eq-simd1", "console", "parametricEq", true, false],
    ["miso.compressor", "comp", "inserts", "unavailable", false, false],
    ["miso.parametric-eq", "eq-simd2", "console", "parametricEq", true, true],
  ];
  gate(browserName, "sdk-live-response", live?.trackId === "track"
    && live.mode === "target" && live.meaning === "eqFilterSubtotal"
    && live.sampleRateHz === 48_000 && live.points === 5
    && Number.isFinite(live.firstFrequency) && Number.isFinite(live.lastFrequency)
    && Array.isArray(live.left) && live.left.length === 5
    && Array.isArray(live.right) && live.right.length === 5
    && live.finite === true && liveU64(live.capturedSample) && liveU64(live.snapshotToken)
    && liveU64(live.resultBytes) && live.ownedAfterSecondQuery === true
    && live.pendingRefused === true && live.closedRefused === true
    && liveInputFilters !== undefined && liveCompressor !== undefined
    && JSON.stringify(liveMemberOrder) === JSON.stringify(expectedLiveMemberOrder)
    && live.excludedMemberCount === 1,
  "browser live response did not return the target subtotal, owned channels, membership, boundary identity, or lifecycle refusals");
  const managedResponse = observations?.managedResponse;
  const managedInitial = managedResponse?.initial;
  const managedAfterRender = managedResponse?.afterRender;
  const managedUpdated = managedResponse?.updated;
  const managedLeftOnly = managedResponse?.leftOnly;
  const managedMemberOrder = managedInitial?.members?.map((member) => [
    member.nativeId, member.stableId, member.rack, member.kind, member.available, member.bypassed,
  ]);
  const expectedManagedMemberOrder = [
    ["miso.builtin.input-filters", "input-filters", "input", "inputFilters", true, false],
    ["miso.parametric-eq", "eq-simd1", "console", "parametricEq", true, false],
    ["miso.compressor", "comp", "inserts", "unavailable", false, false],
    ["miso.parametric-eq", "eq-simd2", "console", "parametricEq", true, true],
  ];
  const finiteArray = (values) => Array.isArray(values) && values.length > 0
    && values.every(Number.isFinite);
  gate(browserName, "sdk-track-response-subscription", managedResponse?.parityWithOneShot === true
    && JSON.stringify(managedMemberOrder) === JSON.stringify(expectedManagedMemberOrder)
    && managedInitial?.points === 5 && finiteArray(managedInitial?.frequencies)
    && finiteArray(managedInitial?.left) && finiteArray(managedInitial?.right)
    && managedInitial.frequencies.length === 5
    && managedInitial.left.length === 5 && managedInitial.right.length === 5
    && managedInitial.capturedSample === "0"
    && liveU64(managedInitial.snapshotToken),
  "managed track-response subscription did not match the one-shot owner membership and values");
  gate(browserName, "sdk-track-response-subscription", managedResponse?.unchangedBeforeRender === true
    && managedResponse.unchangedNotificationCount === 0
    && managedResponse.changedPublication === true
    && managedResponse.callbackCount >= 1
    && managedResponse.callbackOwner === managedResponse.owner
    && managedResponse.callbackEpoch === managedResponse.epoch
    && managedResponse.callbackJob === managedResponse.job
    && BigInt(managedResponse.callbackRevision) > 1n
    && managedResponse.callbackMatches === true
    && managedResponse.automaticDelivery === true
    && managedAfterRender?.points === 5
    && finiteArray(managedAfterRender?.left)
    && finiteArray(managedAfterRender?.right),
  "managed track-response subscription did not suppress unchanged captures or publish the captured edit");
  gate(browserName, "sdk-live-eq-cuts", managedResponse?.liveEq !== undefined
    && BigInt(managedResponse.liveEq.capturedSample) > BigInt(managedResponse.liveEq.appliedAtSample)
    && managedResponse.liveEq.postRampComparedFrames >= 128
    && Number.isFinite(managedResponse.liveEq.postRampMaximumDifference)
    && managedResponse.liveEq.postRampMaximumDifference <= 1e-6,
  "packed browser EQ cuts did not produce fresh target response and settled headless PCM parity");
  gate(browserName, "sdk-track-response-subscription", managedResponse?.workerInitializations === 1
    && managedResponse.workerQueries === 4
    && managedResponse.unchangedPollSuppressed === true
    && managedResponse.sharedJob === true
    && managedResponse.arraysIsolated === true
    && managedResponse.firstCloseKeepsJob === true
    && managedResponse.invalidUpdateRefused === true
    && managedResponse.invalidUpdatePreserved === true
    && managedResponse.gridUpdated === true
    && managedResponse.independentChannelJob === true
    && managedResponse.lastCloseStoppedCapture === true
    && managedResponse.staleReadRefused === true,
  "managed track-response subscription did not preserve bounded shared-job and close/update semantics");
  gate(browserName, "sdk-track-response-subscription", managedUpdated?.points === 7
    && finiteArray(managedUpdated?.frequencies)
    && finiteArray(managedUpdated?.left) && finiteArray(managedUpdated?.right)
    && managedUpdated.frequencies.length === 7
    && managedUpdated.left.length === 7 && managedUpdated.right.length === 7
    && managedLeftOnly?.points === 7
    && finiteArray(managedLeftOnly?.left)
    && managedLeftOnly?.right === undefined
    && managedLeftOnly.left.length === 7
    && liveU64(managedResponse.owner) && liveU64(managedResponse.epoch)
    && liveU64(managedResponse.job) && liveU64(managedResponse.revision)
    && managedResponse.bounds?.maximumHandles > 0
    && managedResponse.bounds?.maximumJobs > 0
    && managedResponse.bounds?.maximumRetainedBytes > 0
    && managedResponse.bounds?.maximumCaptureAttempts > 0
    && managedResponse.bounds?.maximumPoints > 0
    && managedResponse.bounds?.maximumCaptureBytes > 0,
  "managed track-response subscription did not expose updated grids, channel ownership, or bounds");
  gate(browserName, "sdk-observation", Array.isArray(observations?.mapBindings)
    && observations.mapBindings.includes("comp") && observations.subscribeResult === 0
    && observations.pendingBeforeArm?.every((status) => status === "unarmed")
    && observations.pendingAfterArm?.every((status) => status === "pending"),
  "SDK observation map or availability statuses were not current-owner answers");
  gate(browserName, "sdk-observation", observations.readyStatuses?.every((status) => status === "ready")
    && observations.readyChannels?.[0] === "both"
    && observations.projectedChannels?.[0] === "left"
    && observations.readyValuesFinite === true
    && observations.distinctChannelProjection === true
    && observations.ownedAfterSecondQuery === true,
  "SDK observation endpoint did not preserve exact owned data and channel projection");
  gate(browserName, "sdk-observation", Array.isArray(observations.windows)
    && observations.windows.length === 1
    && observations.windows[0]?.endSample > observations.windows[0]?.firstSample
    && observations.windows[0]?.blocks === 2,
  "SDK observation endpoint did not return an actual resident window");
  const resident = observations?.resident;
  gate(browserName, "sdk-observation-subscription", Array.isArray(resident?.mapBindings)
    && resident.mapBindings.includes("comp") && resident.mapBindings.includes("gate")
    && JSON.stringify(resident.selectionEffectSlots) === JSON.stringify(["comp", "gate"])
    && JSON.stringify(resident.pendingStatuses) === JSON.stringify(["pending", "pending"]),
  "resident observation subscription did not acquire the compressor and gate bindings");
  gate(browserName, "sdk-observation-subscription", JSON.stringify(resident.readyStatuses)
    === JSON.stringify(["ready", "ready"])
    && JSON.stringify(resident.readyChannels) === JSON.stringify(["both", "left"])
    && resident.readyValuesFinite === true
    && resident.ownedReadStable === true
    && Array.isArray(resident.windows) && resident.windows.length === 2
    && resident.windows.every(validResidentWindow),
  "resident observation subscription did not return stable owned two-effect windows");
  gate(browserName, "sdk-observation-subscription", resident.automaticDelivery === true
    && resident.callbackCount >= 1 && resident.callbackAvailable === true
    && resident.callbackOwner === resident.owner && resident.callbackEpoch === resident.epoch
    && resident.callbackRows?.some((rows) => Array.isArray(rows)
      && rows.every((row) => row.status === "ready")),
  "browser resident observation subscription did not deliver an automatic ready notification");
  gate(browserName, "sdk-observation-subscription", liveU64(resident.subscriptionId)
    && liveU64(resident.owner) && liveU64(resident.epoch)
    && typeof resident.appliedAtSample === "string"
    && resident.bounds?.maximumHandles > 0
    && resident.bounds?.maximumBindings > 0
    && resident.bounds?.maximumSelections > 0
    && resident.bounds?.maximumWindowBlocks > 0
    && resident.bounds?.maximumCadenceMs > 0,
  "resident observation subscription did not publish bounded owner identity and limits");
  gate(browserName, "sdk-observation-subscription", resident.sharedBinding === true
    && resident.firstCloseKeepsLive === true
    && resident.invalidUpdateRefused === true
    && resident.invalidUpdatePreserved === true
    && JSON.stringify(resident.updatedSelections) === JSON.stringify([
      { effectSlotId: "comp", channels: "right" },
    ])
    && resident.updatedRightOnly === true
    && resident.firstSnapshotRetained === true
    && resident.staleReadRefused === true,
  "resident observation subscription did not preserve shared, update, and close lifecycle semantics");
  validateSdkObservationWait(browserName, observations?.wait);
  const spectrum = response?.spectrum;
  const spectrumRows = Array.isArray(spectrum?.targets) ? spectrum.targets : [];
  const expectedSpectrumTargets = [
    "trackPostInput:track",
    "trackPostPan:track",
    "output:main-out",
  ];
  const spectrumU64 = (value) => typeof value === "string"
    && /^[0-9]+$/.test(value) && BigInt(value) > 0n;
  gate(browserName, "sdk-spectrum", spectrumRows.length === 3
    && JSON.stringify(spectrum?.targetKeys) === JSON.stringify(expectedSpectrumTargets)
    && spectrum?.allTargetsDistinct === true,
  "SDK spectrum query did not exercise the three prepared graph targets in order");
  gate(browserName, "sdk-spectrum", spectrumRows.every((row) => row.channels === "both"
    && row.sampleRateHz === 48_000 && row.windowFrames === 2_048 && row.binCount === 1_025
    && row.capturedSample === "0" && row.endSample === "2048"
    && spectrumU64(row.snapshotToken) && spectrumU64(row.resultBytes)
    && Number.isFinite(row.floorDb) && row.sourceUnderrun === false
    && row.finite === true && Array.isArray(row.frequencies) && row.frequencies.length === 1_025
    && Array.isArray(row.left) && row.left.length === 1_025
    && Array.isArray(row.right) && row.right.length === 1_025),
  "SDK spectrum Worker result did not preserve the complete bounded L/R window and sample span");
  gate(browserName, "sdk-spectrum", spectrum?.allFinite === true
    && spectrum?.ownedArrays === true && spectrum?.postChainDiffers === true
    && spectrum?.busyRefused === true && spectrum?.closedRefused === true,
  "SDK spectrum query did not prove owned arrays, graph-boundary distinction, or lifecycle refusals");
  const continuous = spectrum?.continuous;
  const continuousFirst = continuous?.first;
  const recoveryDelivery = continuous?.recoveryDelivery;
  const recoveryNativeMissedWindows = canonicalU64(recoveryDelivery?.nativeMissedWindows);
  const recoverySkippedPublications = canonicalU64(recoveryDelivery?.skippedPublications);
  const recoveryNotificationCapturedSample = canonicalU64(recoveryDelivery?.notificationCapturedSample);
  const recoveryNotificationEndSample = canonicalU64(recoveryDelivery?.notificationEndSample);
  const recoveryResultCapturedSample = canonicalU64(recoveryDelivery?.resultCapturedSample);
  const recoveryResultEndSample = canonicalU64(recoveryDelivery?.resultEndSample);
  // Each predicate is named so a failed run says which one was false; the pass condition is
  // their conjunction, unchanged.
  const continuousLifecycle = [
    ["pendingBeforeRender", continuous?.pendingBeforeRender === true],
    ["sharedJob", continuous?.sharedJob === true],
    ["automaticDelivery", continuous?.automaticDelivery === true],
    [`windows >= 1 (windows=${continuous?.windows})`, continuous?.windows >= 1],
    ["gap", continuous?.gap === true],
    // Issue #1480 D2: no read runs during the render, so all 17 windows of 6,144 frames at hop 256
    // meet the one-record native queue: one is queued, 16 are dropped, and the first read after
    // the render reports that gap (status 3).
    [`renderLoss (status=${continuous?.renderLoss?.status}, dropped=${continuous?.renderLoss?.droppedCaptures})`,
      continuous?.renderLoss?.status === 3 && continuous?.renderLoss?.droppedCaptures === "16"],
    ["ownedArrays", continuous?.ownedArrays === true],
    ["sharedAfterFirstClose", continuous?.sharedAfterFirstClose === true],
    ["staleReadRefused", continuous?.staleReadRefused === true],
  ];
  const falseContinuousLifecycle = continuousLifecycle
    .filter(([, holds]) => !holds).map(([name]) => name);
  gate(browserName, "sdk-spectrum-continuous", falseContinuousLifecycle.length === 0,
    "continuous spectrum did not prove warmup, shared ownership, capture loss, or close lifecycle;"
    + ` false: ${falseContinuousLifecycle.join(", ")}`);
  gate(browserName, "sdk-spectrum-continuous", continuous?.statuses?.includes("ready") === true
    && continuous?.hopFrames === 256
    && continuous?.nativeStart?.hopFrames === 256
    && continuous?.nativeStart?.smoothingMs === 0
    && Array.isArray(continuous?.nativeReadHopFrames)
    && continuous.nativeReadHopFrames.length > 0
    && continuous.nativeReadHopFrames.every((hop) => hop === 256)
    && Array.isArray(continuous?.publicationHopFrames)
    && continuous.publicationHopFrames.length > 0
    && continuous.publicationHopFrames.every((hop) => hop === 256)
    && Array.isArray(continuous?.publicationSmoothingMs)
    && continuous.publicationSmoothingMs.length > 0
    && continuous.publicationSmoothingMs.every((smoothing) => smoothing === 0)
    && continuousFirst?.sampleRateHz === 48_000
    && continuousFirst?.windowFrames === 2_048
    && continuousFirst?.binCount === 1_025
    && continuousFirst?.hopFrames === 256
    && continuousFirst?.smoothingMs === 0
    && continuousFirst?.peakBins?.[0] === 32 && continuousFirst?.peakBins?.[1] === 32
    && Math.abs(continuousFirst?.peakHz?.[0] - 750) < 0.01
    && Math.abs(continuousFirst?.peakHz?.[1] - 750) < 0.01
    && Math.abs(continuousFirst?.responseHz - 750) < 0.01
    && continuousFirst?.sourceUnderrun === false
    && Array.isArray(continuousFirst?.meterPeaks)
    && continuousFirst.meterPeaks.length >= 4
    && Math.abs(continuousFirst.meterPeaks[0] - continuousFirst.expectedLinearPeaks[0]) < 0.01
    && Math.abs(continuousFirst.meterPeaks[1] - continuousFirst.expectedLinearPeaks[1]) < 0.01
    && Math.abs(continuousFirst.meterPeaks[2] - continuousFirst.expectedLinearPeaks[0]) < 0.01
    && Math.abs(continuousFirst.meterPeaks[3] - continuousFirst.expectedLinearPeaks[1]) < 0.01
    && Math.abs(continuousFirst.firstPcmPeak - continuousFirst.expectedLinearPeaks[0]) < 0.01
    && Math.abs(continuousFirst.peakDbfs?.[0] - continuousFirst.expectedPeakDbfs?.[0]) < 0.02
    && Math.abs(continuousFirst.peakDbfs?.[1] - continuousFirst.expectedPeakDbfs?.[1]) < 0.02
    && Math.abs(continuousFirst.responseGainDb?.[0] - 6) < 0.02
    && Math.abs(continuousFirst.responseGainDb?.[1] - 6) < 0.02,
  "continuous spectrum did not align the 750 Hz peak with response, meter, and owned PCM");
  gate(browserName, "sdk-spectrum-recovery", recoveryDelivery?.status === "ready"
    && recoveryDelivery?.available === true
    && recoveryNativeMissedWindows !== undefined && recoveryNativeMissedWindows > 0n
    && recoverySkippedPublications !== undefined && recoverySkippedPublications > 0n
    && recoveryNotificationCapturedSample !== undefined
    && recoveryNotificationEndSample !== undefined
    && recoveryNotificationEndSample > recoveryNotificationCapturedSample
    && recoveryResultCapturedSample !== undefined
    && recoveryResultEndSample !== undefined
    && recoveryResultEndSample > recoveryResultCapturedSample
    && recoveryNotificationCapturedSample === recoveryResultCapturedSample
    && recoveryNotificationEndSample === recoveryResultEndSample,
  "continuous spectrum recovery did not preserve positive loss and exact notification/result identity");
  const collection = spectrum?.collection;
  const collectionFirstCaptured = canonicalU64(collection?.firstCapturedSample);
  const collectionFirstEnd = canonicalU64(collection?.firstEndSample);
  const collectionSecondCaptured = canonicalU64(collection?.secondCapturedSample);
  const collectionSecondEnd = canonicalU64(collection?.secondEndSample);
  const collectionCaptured = canonicalU64(collection?.capturedSample);
  const collectionEnd = canonicalU64(collection?.endSample);
  gate(browserName, "sdk-spectrum-collection", JSON.stringify(collection?.selections) === JSON.stringify([
      "trackPostPan:track-a", "trackPostPan:track-b", "trackPostPan:track-a",
    ])
    && JSON.stringify(collection?.masks) === JSON.stringify(["both", "left"])
    && collection?.toBOk === true && collection?.toAOks === true
    && collection?.toBTarget === "trackPostPan:track-b"
    && collection?.toBChannels === "left"
    && collection?.bCleared === true && collection?.aCleared === true
    && collection?.switchedWithoutRestart === true,
  "spectrum collection did not atomically switch the existing owner A-to-B-to-A with fresh results");
  gate(browserName, "sdk-spectrum-collection", collection?.firstTarget === "trackPostPan:track-a"
    && collection?.firstChannels === "both"
    && collectionFirstCaptured === 0n
    && collectionFirstEnd === 2_048n
    && collection?.secondTarget === "trackPostPan:track-b"
    && collection?.secondChannels === "left"
    && collectionSecondCaptured !== undefined && collectionSecondEnd !== undefined
    && collectionSecondEnd > collectionSecondCaptured
    && Math.abs(collection?.firstPeakHz - 750) < 0.01
    && Math.abs(collection?.secondPeakHz - 750) < 0.01
    && collection?.distinctSelectedSignal === true
    && collection?.resultTarget === "trackPostPan:track-a"
    && collection?.resultChannels === "both"
    && collectionCaptured !== undefined && collectionEnd !== undefined
    && collectionSecondEnd !== undefined && collectionCaptured > collectionSecondEnd
    && collectionEnd > collectionCaptured
    && collection?.peakBin === 32 && Math.abs(collection?.peakHz - 750) < 0.01
    && collection?.finite === true && collection?.owned === true
    && collection?.audioContinued === true,
  `spectrum collection did not return distinct owned A/B/A known-signal spans while audio continued: ${JSON.stringify(collection)}`);
  const configuredHop = spectrum?.configuredHop;
  const configuredPublications = Array.isArray(configuredHop?.publications)
    ? configuredHop.publications : [];
  gate(browserName, "sdk-spectrum-hop", configuredHop?.requestHopFrames === 1_024
    && configuredHop?.boundsHopFrames === 1_024
    && configuredHop?.nativeStart?.hopFrames === 1_024
    && configuredHop?.nativeStart?.sampleRateHz === 48_000
    && configuredHop?.nativeStart?.quantumFrames === 128
    && configuredHop?.nativeStart?.smoothingMs === 37.5
    && Array.isArray(configuredHop?.nativeRead)
    && configuredHop.nativeRead.length > 0
    && configuredHop.nativeRead.every((metadata) => metadata?.hopFrames === 1_024
      && metadata?.smoothingMs === 37.5)
    && configuredHop?.publicationCount >= 2
    && Array.isArray(configuredHop?.notificationHops)
    && configuredHop.notificationHops.length >= 2
    && configuredHop.notificationHops.every((hop) => hop === 1_024)
    && Array.isArray(configuredHop?.notificationSmoothingMs)
    && configuredHop.notificationSmoothingMs.length >= 2
    && configuredHop.notificationSmoothingMs.every((smoothing) => smoothing === 37.5)
    && configuredPublications.length === 2
    && configuredPublications.every((publication) => publication?.metadata?.hopFrames === 1_024
      && publication.metadata.smoothingMs === 37.5
      && publication.metadata.sampleRateHz === 48_000
      && publication.metadata.quantumFrames === 128
      && publication.metadata.endSample > publication.metadata.capturedSample
      && publication.result?.windowFrames === 2_048
      && publication.result?.binCount === 1_025
      && publication.result?.capturedSample === publication.metadata.capturedSample
      && publication.result?.endSample === publication.metadata.endSample
      && publication.result?.finite === true
      && publication.result?.ownedArrays === true)
    && configuredHop?.startDelta === "1024"
    && configuredHop?.firstSpan === "2048"
    && configuredHop?.secondSpan === "2048"
    && configuredHop?.resultHopFrames === 1_024
    && configuredHop?.resultSmoothingMs === 37.5
    && configuredHop?.ownedArrays === true,
  "actual browser H1024 did not preserve Worklet metadata, Worker results, overlap, smoothing, and ownership");
  // Issue #1106: the probe waits on its deadline, not a poll count. Its audio starts late by a
  // planted delay, so both windows arrive only after the audio start (a publication
  // before that means the delay was not in effect); a wait that can never be satisfied ends at
  // its own deadline and polls no faster than its timer.
  const hopWait = configuredHop?.wait;
  const neverPublishes = hopWait?.neverPublishes;
  gate(browserName, "sdk-spectrum-hop-wait", hopWait?.startDelayMs >= 500
    && hopWait?.deadlineMs === 10_000
    && hopWait?.pollMs > 0
    && hopWait?.firstPublicationAfterMs >= hopWait?.startDelayMs - 100
    && hopWait?.secondPublicationAfterMs >= hopWait.firstPublicationAfterMs
    && hopWait?.elapsedMs < hopWait.deadlineMs
    && hopWait?.polls <= 2 * Math.ceil(hopWait.elapsedMs / hopWait.pollMs) + 2
    && neverPublishes?.count === 0
    && neverPublishes?.elapsedMs >= neverPublishes.deadlineMs
    && neverPublishes?.elapsedMs < neverPublishes.deadlineMs + 2_000
    && neverPublishes?.polls >= 1
    && neverPublishes?.polls <= 2 * Math.ceil(neverPublishes.deadlineMs / hopWait.pollMs) + 2,
  `the hop probe's wait did not outlast a planted slow audio start or was not bounded by its deadline: ${JSON.stringify(hopWait)}`);
}

function mutate(result, mutation) {
  const copy = structuredClone(result);
  if (mutation === "attestation") copy.attestation.outcome = "miso.unsupported.v1";
  if (mutation === "boot") copy.boot.ready = false;
  if (mutation === "native-corpus-digest") copy.corpus.browserDigests[0] = "0".repeat(64);
  if (mutation === "main-thread-stall") copy.stall.measuredStallMs = 0;
  if (mutation === "control-path-applied") copy.liveControls.exactRetargetedOutput = false;
  if (mutation === "control-path-meter") copy.liveControls.masterPeak = 0;
  if (mutation === "control-path-command") copy.liveControls.commandApplied = 0;
  if (mutation === "stall-live-controls-load") copy.stall.liveControlMeterFrames = 0;
  // Issue #143 E12's named red mutation: `observationArmed = 0`.
  if (mutation === "observation-armed") copy.observation.armed.maximumTrackGrDb = 0;
  if (mutation === "observation-disarmed") copy.observation.disarmed.maximumTrackGrDb = 1;
  if (mutation === "observation-identity") copy.observation.identicalAudio = false;
  if (mutation === "resume-clicks-direct") copy.resumes.direct = 1;
  if (mutation === "resume-clicks-missing") copy.resumes.clicksPerformed -= 1;
  if (mutation === "resume-clicks-extra") copy.resumes.clicksPerformed += 1;
  if (mutation === "observation-window") copy.observation.armed.firstSampleMonotonic = false;
  if (mutation === "sdk-observation-window-reversed") {
    const window = copy.sdkResponse.observations.resident.windows[0];
    window.endSample = window.firstSample;
  }
  if (mutation === "sdk-observation-window-malformed") {
    copy.sdkResponse.observations.resident.windows[0].endSample = "1024x";
  }
  if (mutation === "sdk-response") copy.sdkResponse.eq.points = 0;
  if (mutation === "sdk-observation") copy.sdkResponse.observations.readyStatuses[0] = "pending";
  if (mutation === "sdk-spectrum") copy.sdkResponse.spectrum.targets[0].finite = false;
  if (mutation === "sdk-spectrum-hop-missing") {
    delete copy.sdkResponse.spectrum.configuredHop.nativeStart.hopFrames;
  }
  if (mutation === "sdk-spectrum-hop-wrong") {
    copy.sdkResponse.spectrum.continuous.first.hopFrames = 2_048;
  }
  if (mutation === "sdk-spectrum-hop-wait-fast-start") {
    copy.sdkResponse.spectrum.configuredHop.wait.startDelayMs = 0;
  }
  if (mutation === "sdk-spectrum-hop-wait-early") {
    copy.sdkResponse.spectrum.configuredHop.wait.firstPublicationAfterMs = 1;
  }
  if (mutation === "sdk-spectrum-hop-wait-spin") {
    copy.sdkResponse.spectrum.configuredHop.wait.polls = 100_000;
  }
  if (mutation === "sdk-spectrum-hop-wait-never-unbounded") {
    copy.sdkResponse.spectrum.configuredHop.wait.neverPublishes.elapsedMs = 100;
  }
  if (mutation === "sdk-spectrum-recovery-loss") {
    copy.sdkResponse.spectrum.continuous.recoveryDelivery.nativeMissedWindows = "0";
    copy.sdkResponse.spectrum.continuous.recoveryDelivery.skippedPublications = "0";
  }
  if (mutation === "sdk-spectrum-recovery-unavailable") {
    copy.sdkResponse.spectrum.continuous.recoveryDelivery.status = "gap";
    copy.sdkResponse.spectrum.continuous.recoveryDelivery.available = false;
  }
  if (mutation === "sdk-spectrum-recovery-identity") {
    const recovery = copy.sdkResponse.spectrum.continuous.recoveryDelivery;
    recovery.resultCapturedSample = (BigInt(recovery.notificationCapturedSample) + 1n).toString();
  }
  // A live console bypass that did nothing, an insert bypass that landed on another instance, and
  // a post_insert slot addressed by its index within its own section.
  if (mutation === "sdk-live-bypass-console") {
    copy.sdkResponse.liveBypass.blocks.consoleOffLive = copy.sdkResponse.liveBypass.blocks.base;
  }
  if (mutation === "sdk-live-bypass-insert") {
    copy.sdkResponse.liveBypass.blocks.insertOffLive = copy.sdkResponse.liveBypass.blocks.consoleOff;
  }
  // A stepped bypass: one quantum of the starting control, then the target at once.
  if (mutation === "sdk-live-bypass-step") {
    const blocks = copy.sdkResponse.liveBypass.blocks;
    blocks.insertOffLive = [blocks.base[0], ...blocks.insertOff.slice(1)];
  }
  // A ramp that never settles, and a lift that does not reach the document with none.
  if (mutation === "sdk-live-bypass-unsettled") {
    const blocks = copy.sdkResponse.liveBypass.blocks;
    blocks.consoleOffLive = [...blocks.consoleOffLive.slice(0, -1), blocks.base.at(-1)];
  }
  if (mutation === "sdk-live-bypass-lift") {
    copy.sdkResponse.liveBypass.blocks.consoleOnLive = copy.sdkResponse.liveBypass.blocks.consoleOff;
  }
  if (mutation === "sdk-live-bypass-address") copy.sdkResponse.liveBypass.consoleAddress = [3, 0];
  if (mutation === "render-allocations") copy.renderAllocations[0].count = 1;
  if (mutation === "staging-reads") copy.stagingReads.oneShot.spectrumReadResult = 6;
  const wait = copy.sdkResponse?.observations?.wait;
  if (mutation === "sdk-observation-wait-unbounded") wait.deadline.settled = false;
  if (mutation === "sdk-observation-wait-late") wait.deadline.elapsedMs = 5_000;
  if (mutation === "sdk-observation-wait-early") wait.deadline.elapsedMs = 0;
  if (mutation === "sdk-observation-wait-untyped") wait.deadline.errorCode = "invalidArgument";
  if (mutation === "sdk-observation-wait-rendered") wait.deadline.renderedQuantaAfterRefusal = "1";
  if (mutation === "sdk-observation-wait-after-render") wait.deadline.closeAfterRender.resolved = false;
  if (mutation === "sdk-observation-wait-resend") {
    wait.render.commands.splice(wait.render.commands.length - 1, 0,
      { result: 6, reason: 8, records: wait.render.commands[1].records });
  }
  if (mutation === "sdk-render-allocations") copy.sdkResponse.renderAllocations[0].count = 1;
  if (mutation === "sdk-render-allocations-missing") copy.sdkResponse.renderAllocations.splice(1, 1);
  return copy;
}

function mutationProofs(browserName, result) {
  const mutations = result.sdkResponse === null
    ? MUTATIONS.filter((mutation) => !mutation.startsWith("sdk-"))
    : MUTATIONS;
  for (const mutation of mutations) {
    assert.throws(
      () => validate(browserName, mutate(result, mutation)),
      (error) => error instanceof Error && error.message.startsWith(
        mutation.startsWith("resume-clicks-") ? `${browserName}: resume-clicks:`
          : mutation.startsWith("sdk-observation-wait-") ? `${browserName}: sdk-observation-wait:`
            : mutation.startsWith("sdk-spectrum-hop-wait-") ? `${browserName}: sdk-spectrum-hop-wait:`
            : `${browserName}:`,
      ),
      `${browserName}: ${mutation}: red mutation escaped its gate`,
    );
  }
  if (result.sdkResponse === null) return;
  const resident = result.sdkResponse.observations?.resident;
  const original = resident?.windows?.[0];
  if (original === undefined) return;
  const widthCrossing = structuredClone(result);
  widthCrossing.sdkResponse.observations.resident.windows[0] = {
    ...original,
    firstSample: "768",
    endSample: "1024",
  };
  assert.doesNotThrow(
    () => validate(browserName, widthCrossing),
    `${browserName}: sdk-observation-window-width: valid decimal-width-crossing span was refused`,
  );
}

// Issue #280: the served artifact set is *exact*, and both halves of that are proved here.
//
// The set drifted to five names when #243 added `miso-engine-v1-abi-layout.json`, so
// `npm run qualify` refused the very directory `scripts/build-web-audioworklet.sh` produces --
// before any browser started. Widening a pin can silently become loosening it, so this walks the
// real built directory: the shipped six are accepted, removing *any one* of them is refused
// (including the sixth, which is what proves the widening is not a five-name pin with a hole in
// it), one stray file is refused, and a directory wearing an artifact's name is refused as not a
// regular file. Every mutation is made on a copy under a temporary root; the built artifacts are
// never touched.
async function artifactSetProofs(artifacts) {
  const root = await mkdtemp(path.join(os.tmpdir(), "miso-qualification-artifact-set-"));
  const refusesSet = (directory, mutation) => assert.rejects(
    () => exactArtifacts(directory),
    (error) => error instanceof Error
      && error.message === "artifact directory must contain the exact shipped artifact set",
    `artifact-set: ${mutation}: red mutation escaped the artifact pin`,
  );
  try {
    const shipped = path.join(root, "shipped");
    await cp(artifacts, shipped, { recursive: true });
    const names = (await readdir(shipped)).sort();
    assert.deepEqual(names, [...ARTIFACT_NAMES].sort(),
      "artifact-set: the built directory is not the exact shipped set");
    // Green on the unmutated build, so the refusals below are refusals of the mutation and not of
    // something the directory was already failing.
    await exactArtifacts(shipped);
    for (const name of names) {
      const missing = path.join(root, `missing-${name}`);
      await cp(shipped, missing, { recursive: true });
      await rm(path.join(missing, name));
      await refusesSet(missing, `${name} removed`);
    }
    // The W4-D1 artifact this project deliberately stopped shipping: the exact spelling of a stray
    // file a stale build tree would leave behind.
    const STRAY = "miso-engine-v1-audio-worklet.scalar.wasm";
    const stray = path.join(root, "stray");
    await cp(shipped, stray, { recursive: true });
    await writeFile(path.join(stray, STRAY), "");
    await refusesSet(stray, "one stray file added");
    // Substitution keeps the file count unchanged, so only the name test can catch it. Without this row
    // the name test could be deleted and every other row would stay green.
    for (const name of names) {
      const substituted = path.join(root, `substituted-${name}`);
      await cp(shipped, substituted, { recursive: true });
      await rm(path.join(substituted, name));
      await writeFile(path.join(substituted, STRAY), "");
      await refusesSet(substituted, `${name} replaced by a stray of the same count`);
    }
    const directoryNamedLikeAnArtifact = path.join(root, "not-a-regular-file");
    await cp(shipped, directoryNamedLikeAnArtifact, { recursive: true });
    await rm(path.join(directoryNamedLikeAnArtifact, "miso-engine-v1-abi-layout.json"));
    await mkdir(path.join(directoryNamedLikeAnArtifact, "miso-engine-v1-abi-layout.json"));
    await assert.rejects(
      () => exactArtifacts(directoryNamedLikeAnArtifact),
      (error) => error instanceof Error
        && error.message === "artifact is not a regular file: miso-engine-v1-abi-layout.json",
      "artifact-set: directory named like an artifact: red mutation escaped the artifact pin",
    );
    return names.length;
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

function normalizedRow(browserName, browserVersion, outcome) {
  const passed = outcome === "simd128 supported";
  return {
    browser: browserName,
    versionFloor: browserVersion,
    outcome,
    gates: {
      attestation: "pass",
      audioWorkletBoot: passed ? "pass" : "not-applicable",
      nativeCorpusDigest: passed ? "pass" : "not-applicable",
      controlPath: passed ? "pass" : "not-applicable",
      observation: passed ? "pass" : "not-applicable",
      mainThreadStall: passed ? "pass" : "not-applicable",
    },
  };
}

async function qualifyBrowser(browserName, engine, origin, proveMutations, sdkEnabled) {
  const launchOptions = { headless: true };
  if (browserName === "chromium") {
    launchOptions.channel = "chromium";
    launchOptions.args = ["--autoplay-policy=no-user-gesture-required", "--disable-dev-shm-usage"];
  }
  const browser = await engine.launch(launchOptions);
  try {
    const page = await browser.newPage();
    const diagnostics = [];
    page.on("console", (message) => diagnostics.push(`console ${message.type()}: ${message.text()}`));
    page.on("pageerror", (error) => diagnostics.push(`pageerror: ${error.message}`));
    page.on("response", (response) => {
      if (!response.ok()) diagnostics.push(`HTTP ${response.status()}: ${response.url()}`);
    });
    page.setDefaultTimeout(120000);
    await page.goto(`${origin}/qualification/index.html`);
    const plan = await page.evaluate(
      async (enabled) => (await import(`/qualification/qualification.js${enabled ? "?sdk=1" : ""}`))
        .resumeClickPlan(enabled),
      sdkEnabled,
    );
    const execution = page.evaluate(async (enabled) => {
      try {
        const module = await import(`/qualification/qualification.js${enabled ? "?sdk=1" : ""}`);
        return await module.runQualification();
      } catch (error) {
        return {
          qualificationError: {
            message: error?.message ?? String(error),
            name: error?.name ?? typeof error,
            stack: error?.stack,
            value: error !== null && typeof error === "object" ? { ...error } : error,
          },
        };
      }
    }, sdkEnabled);
    // Resume each real AudioContext with a trusted gesture across browsers: WebKit starts or restarts
    // a context only inside a user activation, and leaves `resume()` pending without one. The page
    // declares the controls it needs clicked (`resumeClickPlan`), in order, and this loop clicks
    // exactly those, counting its clicks, under the one `PAGE_DEADLINE_MS` deadline for the whole
    // page. The count goes into the result: a missing or an extra click, or a `resume()` that was
    // not in a click, fails the `resume-clicks` gate by name. Racing completion keeps a preceding
    // failure from turning into a locator timeout that hides its diagnostic.
    const deadline = Date.now() + PAGE_DEADLINE_MS;
    const remaining = () => Math.max(1, deadline - Date.now());
    const completion = execution.then((value) => ({ kind: "complete", value }));
    let deadlineTimer;
    const expired = new Promise((resolve) => {
      deadlineTimer = setTimeout(() => resolve({ kind: "deadline" }), remaining());
    });
    let clicksPerformed = 0;
    try {
      for (const selector of plan) {
        const click = page.locator(selector).first().click({ timeout: remaining() }).then(
          () => ({ kind: "clicked" }),
          (error) => ({ kind: "click-error", error }),
        );
        const outcome = await Promise.race([completion, click, expired]);
        if (outcome.kind === "complete") break;
        if (outcome.kind === "deadline" || outcome.kind === "click-error") {
          // The page neither finished nor made the control this loop waits for: it is stuck. Name
          // that, with what the page logged, instead of a bare locator timeout.
          throw new Error(`${browserName}: the page did not finish or offer a declared control to click (${selector}; ${clicksPerformed} of ${plan.length} clicks done) within ${PAGE_DEADLINE_MS} ms: ${outcome.error?.message ?? "total deadline"}; page diagnostics: ${diagnostics.join("; ")}`);
        }
        clicksPerformed += 1;
      }
      const outcome = await Promise.race([completion, expired]);
      if (outcome.kind === "deadline") {
        throw new Error(`${browserName}: the page did not finish within ${PAGE_DEADLINE_MS} ms (${clicksPerformed} of ${plan.length} declared clicks done); page diagnostics: ${diagnostics.join("; ")}`);
      }
    } finally {
      clearTimeout(deadlineTimer);
    }
    const result = await execution;
    if (result.resumes !== undefined) {
      result.resumes.planned = plan.length;
      result.resumes.clicksPerformed = clicksPerformed;
    }
    gate(browserName, "browser-execution", result.qualificationError === undefined,
      `${JSON.stringify(result.qualificationError, (_key, value) => (typeof value === "bigint" ? `${value}n` : value))}${diagnostics.length === 0 ? "" : `; ${diagnostics.join("; ")}`}`);
    const outcome = validate(browserName, result);
    if (proveMutations) mutationProofs(browserName, result);
    if (Array.isArray(result.renderAllocations)) {
      // Issue #1333 D3: the per-instance counts the render-allocations gate just held to zero.
      process.stdout.write(`${browserName}: render-allocations: ${result.renderAllocations
        .map((row) => `${row.workload}=${row.count}`).join(" ")}\n`);
    }
    if (Array.isArray(result.sdkResponse?.renderAllocations)) {
      // Issue #1476 D4: the per-SDK-instance counts the sdk-render-allocations gate just held to zero.
      process.stdout.write(`${browserName}: sdk-render-allocations: ${result.sdkResponse.renderAllocations
        .map((row) => `${row.workload}=${row.count}`).join(" ")}\n`);
    }
    process.stdout.write(`${browserName}: all qualification gates passed (${browser.version()})\n`);
    const row = normalizedRow(browserName, browser.version(), outcome);
    if (sdkEnabled) row.gates.sdkResponse = "pass";
    return row;
  } finally {
    await browser.close();
  }
}

function validateCheckedRow(browserName, actual, checked) {
  const expected = checked.browsers.find((row) => row.browser === browserName);
  gate(browserName, "deployment-matrix", expected !== undefined,
    "checked matrix has no browser row");
  gate(browserName, "deployment-matrix", JSON.stringify(actual) === JSON.stringify(expected),
    "checked browser floor/outcome differs from this CI run");
  const mutated = structuredClone(expected);
  mutated.versionFloor = `${mutated.versionFloor}-red-mutation`;
  assert.throws(
    () => gate(browserName, "deployment-matrix",
      JSON.stringify(actual) === JSON.stringify(mutated), "red mutation"),
    (error) => error instanceof Error
      && error.message.startsWith(`${browserName}: deployment-matrix:`),
    `${browserName}: deployment-matrix: red mutation escaped its gate`,
  );
}

async function main() {
  // Issue #272: before a browser is launched, prove every qualification session document declares
  // the canonical-PCM identity of the audio this harness actually feeds it. It is a static check --
  // no artifacts, no browser -- and it runs first so a false identity can never be sealed by a
  // green matrix run. See `docs/derivations/241-browser-source-identities.md`.
  const identities = await checkSessionIdentities();
  process.stdout.write(
    `session identities: ${identities.length} qualification documents declare their fed PCM\n`,
  );
  const artifacts = option("--artifacts");
  if (artifacts === null) {
    throw new Error("usage: npm run qualify -- --artifacts DIR [--browser NAME] [--check-matrix|--record-matrix --candidate-commit 40_HEX] [--self-test-mutations]");
  }
  const browserOption = option("--browser") ?? "all";
  const browserNames = browserOption === "all" ? Object.keys(ENGINES) : [browserOption];
  for (const browserName of browserNames) {
    if (!(browserName in ENGINES)) throw new Error(`unknown browser: ${browserName}`);
  }
  const checkMatrix = process.argv.includes("--check-matrix");
  const recordMatrix = process.argv.includes("--record-matrix");
  const proveMutations = process.argv.includes("--self-test-mutations");
  if (recordMatrix && browserOption !== "all") {
    throw new Error("--record-matrix requires --browser all");
  }
  const candidateCommit = option("--candidate-commit");
  if (recordMatrix && (candidateCommit === null || !CANONICAL_COMMIT.test(candidateCommit))) {
    throw new Error("--record-matrix requires --candidate-commit as canonical lowercase 40-hex");
  }
  const artifactDigest = await sha256(path.join(path.resolve(artifacts), WASM_ARTIFACT));
  const sdkRoot = option("--sdk-root");
  // The bundle is always built and served: the raw live-control rows send their `SET_CONTROL`
  // transaction through it (#1563B). `--sdk-root` selects which SDK it comes from, and whether the
  // SDK rows run; without it the repository's own `sdk/` (with its `node_modules`) is bundled.
  const sdkBundle = await buildSdkBundle(path.resolve(sdkRoot ?? path.join(HERE, "../../../sdk")));

  if (proveMutations) {
    const served = await artifactSetProofs(artifacts);
    process.stdout.write(`artifact set: the exact ${served}-file shipped set is pinned\n`);
  }

  // Issue #1061: `--check-matrix` holds each browser's floor and outcome, the Playwright version
  // and the generated document to `results.json` -- all independent of the module. Its lineage
  // (`wasmSha256`, `candidateCommit`) describes the last recorded qualification; only a release
  // change must record one of the bytes it ships, and `scripts/web-audioworklet-identity.py`
  // checks that (docs/RELEASE.md).
  const checked = checkMatrix
    ? JSON.parse(await readFile(RESULTS_PATH, "utf8"))
    : null;
  const server = await startQualificationServer({ artifacts, sdkBundle });
  try {
    const rows = [];
    for (const browserName of browserNames) {
      const row = await qualifyBrowser(browserName, ENGINES[browserName], server.origin, proveMutations, sdkRoot !== null);
      rows.push(row);
      if (checked !== null) validateCheckedRow(browserName, row, checked);
    }
    if (checked !== null) {
      gate("matrix", "playwright-version", checked.playwrightVersion === PLAYWRIGHT_VERSION,
        "package and checked results use different Playwright versions");
      gate("matrix", "generated-document", renderMatrix(checked)
        === await readFile(MATRIX_PATH, "utf8"), "document was not regenerated from results.json");
    }
    if (recordMatrix) {
      const results = {
        schema: "miso.web.qualification.matrix.v1",
        candidateCommit,
        wasmSha256: artifactDigest,
        playwrightVersion: PLAYWRIGHT_VERSION,
        platform: "linux-headless",
        artifact: "single shipped simd128 AudioWorklet artifact",
        defaultRingFrames: 5120,
        minimumStallMs: 100,
        browsers: rows,
      };
      await writeFile(RESULTS_PATH, `${JSON.stringify(results, null, 2)}\n`);
      await writeFile(MATRIX_PATH, renderMatrix(results));
      process.stdout.write(`recorded ${path.relative(process.cwd(), RESULTS_PATH)} and ${path.relative(process.cwd(), MATRIX_PATH)}\n`);
    }
  } finally {
    await server.close();
    if (sdkBundle !== undefined) await rm(sdkBundle, { recursive: true, force: true });
  }
}

await main();
