import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { cp, mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";
import { chromium, webkit } from "playwright";
import { qualifyBlake3 } from "./blake3.mjs";
import { qualifySuspendedPcmFeed } from "./pcm-feed-suspended.mjs";
import { renderMatrix } from "./generate-matrix.mjs";
import { checkSessionIdentities } from "./session-identities.mjs";
import {
  classifyStallSignature, largeReplaceSource, qualificationConstants, requestFailure, unsubscribeFailures,
} from "./qualification.js";
import { worstReplaceDocument } from "./worst-replace-document.js";
import { ARTIFACT_NAMES, exactArtifacts, startQualificationServer } from "./server.mjs";
import { PROTOCOL_BLOCKS, protocolCalls } from "./protocol-calls.mjs";
import { structuralEditReference } from "./structural-edit-reference.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const HOST_WEB = path.dirname(HERE);
const FIXTURE = path.join(HOST_WEB, "tests", "browser-v1");
const RESULTS_PATH = path.join(HERE, "results.json");
const MATRIX_PATH = path.join(HOST_WEB, "BROWSER_DEPLOYMENT_MATRIX.md");
const WASM_ARTIFACT = "miso-engine-v1-audio-worklet.simd128.wasm";
const CANONICAL_COMMIT = /^[0-9a-f]{40}$/;
const PLAYWRIGHT_VERSION = JSON.parse(
  await readFile(path.join(HERE, "node_modules", "playwright", "package.json"), "utf8"),
).version;
// Owner ruling 2026-10-25 (#2092): Chrome and Safari are the supported browsers; Firefox is not
// qualified, and `--browser firefox` is refused as an unknown browser.
const ENGINES = { chromium, webkit };
// Issue #1332 S9: each leg's page. The worker leg's origin serves COOP/COEP, so the host picks
// `worker` mode by default; the single leg's origin does not, so it runs `single`.
const LEGS = Object.freeze({
  single: Object.freeze({ isolated: false, instanceMode: "single" }),
  worker: Object.freeze({ isolated: true, instanceMode: "worker" }),
});
// Issue #2040: each red mutation and the one gate that must catch it. `mutationProofs` holds the
// thrown message to `${browserName}: ${gate}:`, so a gate that stops firing turns red even while
// another gate still throws on the same record. A mutation whose name names a gate names the gate
// it reaches.
const caughtBy = (gate, ...mutations) => mutations.map((mutation) => ({ mutation, gate }));
// Issue #1332 S9: a mutation of a claim only one leg makes runs on that leg alone.
const onLeg = (leg, entries) => entries.map((entry) => ({ ...entry, leg }));
const MUTATIONS = Object.freeze([
  ...caughtBy("attestation", "attestation"),
  ...caughtBy("AudioWorklet-boot", "boot"),
  ...caughtBy("native-corpus-digest", "native-corpus-digest"),
  // Issue #137 E8/E6: the live-control row and the live-control load carried across the stall.
  ...caughtBy("main-thread-stall", "main-thread-stall", "stall-live-controls-load"),
  ...caughtBy("control-path", "control-path-applied", "control-path-meter", "control-path-command"),
  // Issue #1562b: a structural apply that was not the rebuild path, that was applied early, that
  // moved the output off the declared delay, or whose meter stream was not the successor's.
  ...caughtBy("structural-apply-path", "structural-apply-path"),
  ...caughtBy("structural-apply-early", "structural-apply-early"),
  ...caughtBy("structural-apply-output", "structural-apply-output"),
  ...caughtBy("structural-apply-meter", "structural-apply-meter"),
  // #1682: an empty session that did not boot silent and empty, one whose first stem was not a
  // rebuild heard unchanged, and one whose meter stream did not follow the strip count.
  ...caughtBy("empty-boot-silence", "empty-boot-silence"),
  ...caughtBy("empty-boot-shape", "empty-boot-shape"),
  ...caughtBy("empty-boot-first-stem", "empty-boot-first-stem"),
  ...caughtBy("empty-boot-meter", "empty-boot-meter"),
  // Issue #1295: a `SESSION_REPLACE` on this (non-isolated) page. A replacement never sent, a Node
  // reference that applies it one block late, a commit off the rebuild path or applied early, an
  // outcome other than exact, a source underrun, a blocking rebuild left uncounted, a handler
  // allocation left unreported, and a 1 MiB replacement the core refused.
  ...caughtBy("structural-edit-output", "structural-edit-skipped", "structural-edit-adoption-shifted"),
  ...caughtBy("structural-edit-path", "structural-edit-path"),
  ...caughtBy("structural-edit-early", "structural-edit-early"),
  ...caughtBy("structural-edit-outcome", "structural-edit-outcome"),
  ...caughtBy("structural-edit-underruns", "structural-edit-underruns"),
  ...onLeg("single", caughtBy("structural-edit-blocking", "structural-edit-blocking-uncounted")),
  ...onLeg("single", caughtBy("structural-edit-control-allocations", "structural-edit-control-allocations-uncounted")),
  // Issue #1332 S9: on the worker leg, a replacement counted as a blocking rebuild, and control
  // allocations counted on the worklet instance.
  ...onLeg("worker", caughtBy("structural-edit-blocking", "structural-edit-blocking-counted")),
  ...onLeg("worker", caughtBy("structural-edit-control-allocations", "structural-edit-control-allocations-counted")),
  ...caughtBy("structural-replace-large", "structural-replace-large"),
  // Issue #1332 S3: after the 1 MiB replacement grew the shared memory in a handler, a capture
  // that is not the module's own render under Node.
  ...caughtBy("structural-replace-grown", "structural-replace-grown-output"),
  // Issue #1332 S9: a host or an SDK engine in the other leg's mode; a context whose successor
  // meter stream never arrived after the growth, or that stopped rendering after it; a worker-leg
  // replacement during which the worklet did not render; and meter frames skipped past the bound.
  ...caughtBy("instance-mode", "instance-mode", "sdk-instance-mode"),
  ...caughtBy("growth-while-rendering", "growth-while-rendering-meter", "growth-while-rendering-stopped"),
  ...onLeg("worker", caughtBy("growth-while-rendering-concurrent", "growth-while-rendering-concurrent")),
  ...caughtBy("meter-frames-skipped", "meter-frames-skipped"),
  // Issue #143 E12: the observation row. `observation-armed` is the eval's named red mutation --
  // a run whose armed tap published nothing, which is exactly what a browser that lost the
  // transport would produce.
  ...caughtBy("observation-armed", "observation-armed"),
  // Issue #1589c: an `Observe` arm the core refused.
  ...caughtBy("observation", "observation-arm-refused"),
  // Issue #1535 D9: a never-armed run that received no frame proves nothing.
  ...caughtBy("observation-disarmed", "observation-disarmed", "observation-disarmed-empty"),
  ...caughtBy("observation-identity", "observation-identity"),
  // Issue #1332 S9: a single-leg unsubscribe checked only past the render boundary.
  ...onLeg("single", caughtBy("observation-unsubscribe-rule", "observation-unsubscribe-rule")),
  ...caughtBy("observation-window", "observation-window"),
  ...caughtBy("sdk-observation-subscription", "sdk-observation-subscription-window-reversed",
    "sdk-observation-subscription-window-malformed"),
  ...caughtBy("sdk-response", "sdk-response"),
  ...caughtBy("sdk-observation", "sdk-observation"),
  ...caughtBy("sdk-spectrum", "sdk-spectrum"),
  ...onLeg("single", caughtBy("sdk-spectrum-control-allocations", "sdk-spectrum-control-allocations")),
  ...onLeg("worker", caughtBy("sdk-spectrum-control-allocations", "sdk-spectrum-control-allocations-counted")),
  ...caughtBy("sdk-spectrum-hop", "sdk-spectrum-hop-missing"),
  ...caughtBy("sdk-spectrum-continuous", "sdk-spectrum-continuous-hop-wrong"),
  ...caughtBy("sdk-spectrum-recovery", "sdk-spectrum-recovery-loss", "sdk-spectrum-recovery-unavailable",
    "sdk-spectrum-recovery-identity"),
  // Issue #1106: a hop probe whose audio start was not delayed, one that never waited, one that
  // spun, and a never-publishing wait that outlived its deadline.
  ...caughtBy("sdk-spectrum-hop-wait", "sdk-spectrum-hop-wait-fast-start", "sdk-spectrum-hop-wait-early",
    "sdk-spectrum-hop-wait-spin", "sdk-spectrum-hop-wait-never-unbounded"),
  // Issue #1579b: the browser's track responses through the core's frames, on a live context.
  ...caughtBy("sdk-track-response-subscription", "sdk-track-response-worker",
    "sdk-track-response-unchanged", "sdk-track-response-once", "sdk-track-response-early"),
  ...caughtBy("sdk-live-response", "sdk-live-response-serialized", "sdk-live-response-sections"),
  // Issue #1097 gate 4: a live bypass that reached no instance, or the wrong one. Issues #1341a1
  // and #1341a2: a live bypass is a ramp over the session's mute length, so a step, a ramp that
  // never settles and a lift that never lands are red too.
  ...caughtBy("sdk-live-bypass", "sdk-live-bypass-console", "sdk-live-bypass-insert",
    "sdk-live-bypass-address", "sdk-live-bypass-step", "sdk-live-bypass-unsettled", "sdk-live-bypass-lift"),
  // Issue #1333 D3: one render-locked allocator call on one instance, and a staging-read workload
  // whose observation read was refused.
  ...caughtBy("render-allocations", "render-allocations"),
  ...caughtBy("staging-reads", "staging-reads"),
  // Issue #1580 D2: the host-level track response through the core's frames: a `RESPONSE_READY`
  // whose outcome is not 0, and a run whose one event does not carry the reply's tag.
  ...caughtBy("track-response-host", "track-response-host-outcome", "track-response-host-tag"),
  // Issue #1476 D5: one SDK instance's allocator call, and one SDK instance closed without its read.
  ...caughtBy("sdk-render-allocations", "sdk-render-allocations", "sdk-render-allocations-missing"),
  // Batch-verdict 2, B2 (root 2026-10-12): a real AudioContext resumed outside a trusted click, and
  // a harness that clicked fewer or more controls than the page declared.
  ...caughtBy("resume-clicks", "resume-clicks-direct", "resume-clicks-missing", "resume-clicks-extra"),
  // Issue #1530 gate 4: the browser observation wait, one predicate each.
  ...caughtBy("sdk-observation-wait", "sdk-observation-wait-unbounded", "sdk-observation-wait-late",
    "sdk-observation-wait-early", "sdk-observation-wait-untyped", "sdk-observation-wait-rendered",
    "sdk-observation-wait-after-render", "sdk-observation-wait-resend"),
  // Issue #1589 D4 (R4): an `armed()` that never settled, or settled after a window it gave.
  ...caughtBy("sdk-observation-armed", "sdk-observation-armed"),
  // Issue #1533: a Playwright page error, a page-side unhandled rejection, and a result that
  // `qualifyBrowser` never gave its page-error lists.
  ...caughtBy("page-errors", "page-errors", "page-errors-window", "page-errors-missing"),
  // Issue #1571b: a browser answer that is not the headless engine's, a reliable event lost or out
  // of order, a lossy event that is not the headless engine's event of its block, and a refusal that
  // is not the core's.
  ...caughtBy("sdk-protocol-answers", "sdk-protocol-answers"),
  ...caughtBy("sdk-protocol-reliable", "sdk-protocol-reliable"),
  ...caughtBy("sdk-protocol-lossy", "sdk-protocol-lossy", "sdk-protocol-lossy-none",
    "sdk-protocol-lossy-first-block"),
  ...caughtBy("sdk-protocol-refusal", "sdk-protocol-refusal"),
]);

// The total time the page may take, from the first evaluate to the result, clicks included. A page
// that is stuck (WebKit can wedge a page whose context stalls under full CPU load) fails by name here
// instead of waiting on a locator.
const PAGE_DEADLINE_MS = 300_000;

// Issue #1332 S9: the raw hosts the page boots (one render-allocation row each), and the bound on
// the meter frames a worker-mode run may skip (R-S7-meters).
const RAW_INSTANCES = 16;
const METER_FRAMES_SKIPPED_BOUND = 4;

// Issue #1476 D4: every SDK engine `sdk-response-entry.ts` creates, in the order its count is
// read (with repeats). A new SDK instance adds its label here.
const SDK_RENDER_ALLOCATION_WORKLOADS = Object.freeze([
  "resident-observation",
  "observation-wait-deadline",
  "observation-wait-render",
  "track-response-live",
  "live-eq-cuts",
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
  "protocol-messages",
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
      entryPoints: [path.join(HERE, "pcm-feed-suspended-entry.ts")],
      plugins: useDistribution ? [shippedSdk] : [],
      outfile: path.join(bundle, "pcm-feed-suspended-client.js"),
      bundle: true, format: "esm", platform: "browser", target: "es2022",
    });
    await cp(useDistribution
      ? path.join(distribution, "assets", "miso-engine-v1-pcm-feed-worklet.js")
      : path.join(sourceRoot, "browser-assets", "miso-engine-v1-pcm-feed-worklet.js"),
    path.join(bundle, "miso-engine-v1-pcm-feed-worklet.js"));
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

/**
 * The headless SDK module of the SDK this run qualifies, chosen as `buildSdkBundle` chooses the
 * browser bundle's: the built distribution when the selected SDK has one, otherwise the source.
 */
async function sdkHeadlessEntry(sdkRoot) {
  const distribution = path.join(sdkRoot, "dist");
  const useDistribution = await readdir(distribution).then((entries) => entries.includes("core"), () => false);
  return pathToFileURL(useDistribution
    ? path.join(distribution, "headless", "index.js")
    : path.resolve(HERE, "../../../sdk/src/headless/index.ts")).href;
}

/**
 * Issue #1571 gates 2, 3 and 5, the headless half: `protocol-calls.mjs`'s sequence on the headless
 * SDK engine, booted from the document the page's browser half booted and the served module. The
 * engine reads its events after each rendered block.
 */
async function headlessProtocolReference(headlessEntry, artifacts, protocol) {
  const { MisoEngineAsset, createOfflineEngine } = await import(headlessEntry);
  const asset = await MisoEngineAsset.load(await readFile(path.join(path.resolve(artifacts), WASM_ARTIFACT)));
  const engine = await createOfflineEngine(protocol.document, { asset, sourceRingFrames: protocol.sourceRingFrames });
  try {
    return await protocolCalls(
      engine,
      async () => {
        const controls = engine.liveControls();
        return controls.submit(controls.edit.track("track").faderDb(-3));
      },
      async () => {
        for (let block = 0; block < PROTOCOL_BLOCKS; block += 1) engine.render();
      },
    );
  } finally {
    engine.dispose();
  }
}

function validate(browserName, result, leg) {
  gate(browserName, "result-schema", result?.schema === "miso.web.qualification.result.v1",
    "unexpected browser result");
  // Issue #1533 D3/D4: any page error or unhandled rejection is red, before every result gate and
  // before the attestation early return. No allow-list (D5).
  const pageErrors = result.pageErrors;
  gate(browserName, "page-errors", pageErrors !== null && typeof pageErrors === "object"
    && Array.isArray(pageErrors.playwright) && Array.isArray(pageErrors.window)
    && pageErrors.playwright.length === 0 && pageErrors.window.length === 0,
  JSON.stringify(pageErrors ?? null));
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
  // Issue #1332 S9: the leg's page ran every instance in the leg's mode. The worker leg's page is
  // cross-origin isolated and every host and SDK engine it booted reports `worker` (a control
  // Worker beside the render-only worklet); the single leg's page is not, and every one reports
  // `single`. A host that silently fell back to the other mode turns this red.
  const worker = leg === "worker";
  const mode = LEGS[leg].instanceMode;
  const sdkModes = result.sdkResponse?.instanceModes;
  gate(browserName, "instance-mode", result.crossOriginIsolated === LEGS[leg].isolated
    && Array.isArray(result.instanceModes)
    && result.instanceModes.length === RAW_INSTANCES
    && result.instanceModes.every((instanceMode) => instanceMode === mode)
    && (result.sdkResponse === null || (Array.isArray(sdkModes)
      && sdkModes.length === result.sdkResponse.renderAllocationInstances
      && sdkModes.every((instanceMode) => instanceMode === mode))),
  `the ${leg} leg did not run every instance in ${mode} mode: ${JSON.stringify({
    crossOriginIsolated: result.crossOriginIsolated, raw: result.instanceModes, sdk: sdkModes ?? null })}`);
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

  // Issue #1562b: a structural apply in a real browser. The core classifies a `delay_samples` edit
  // as a rebuild and the browser prepares and publishes the replacement plan. The edit restarts the
  // track, so it is a duck-swap (#1324a): the applied revision moves at the scheduled `S` and not
  // one block earlier, the duck is exact silence until `S`, what comes out after the fade-in is the
  // declared delay of the input, exactly, and the meter stream is the successor's.
  const structural = result.structuralApply;
  gate(browserName, "structural-apply-path", structural?.commandOk === true
    && structural?.commandPath === "rebuild" && structural?.commandApplied === 1
    && /^[0-9]+$/.test(structural?.committedRevision ?? ""),
  "the structural SET_CONTROL transaction was not committed on the rebuild path");
  gate(browserName, "structural-apply-early", structural?.pendingRevision === structural?.initialRevision
    && structural?.finalRevision === structural?.committedRevision
    && BigInt(structural?.finalRevision ?? "0") > BigInt(structural?.initialRevision ?? "0")
    && /^[0-9]+$/.test(structural?.scheduledSample ?? "")
    && BigInt(structural?.finalSample ?? "-1") === BigInt(structural.scheduledSample),
  `the applied revision moved before the swap block or did not reach the committed one: ${JSON.stringify(structural)}`);
  gate(browserName, "structural-apply-output", structural?.exactDelayedOutput === true
    && structural?.duckedSilent === true
    && structural?.renderedDigest === structural?.expectedDigest,
  "the duck was not silent until the swap, or the swapped plan did not render the input delayed by exactly the edit");
  gate(browserName, "structural-apply-meter", structural?.meterLeaseResult === 0
    && structural?.meterFrames >= 1 && structural?.meterFrameWidth === 4
    && structural?.meterTrackCount === 1 && structural?.sequenceIncreasing === true
    && Array.isArray(structural?.structureEpochs) && structural.structureEpochs.length === 1
    && Number.isSafeInteger(structural.structureEpochs[0]) && structural.structureEpochs[0] >= 1,
  `the meter stream after the swap was not the successor's with one structure epoch: ${JSON.stringify(structural)}`);

  // #1682: an empty session in a real browser. It boots and renders exact silence with no source
  // and no strip in the session map and the control list, the meter stream is the master's two
  // peaks alone, and one structural transaction adds the source, the track and the route: the fed
  // PCM then comes out unchanged from the swap block, the applied revision was not early, and the
  // meter stream grew to the strip.
  const empty = result.emptyBoot;
  gate(browserName, "empty-boot-silence", empty?.boot?.silent === true
    && empty?.boot?.renderedQuanta === String(empty?.boot?.expectedQuanta),
  `the empty session did not render exact silence for every quantum: ${JSON.stringify(empty?.boot)}`);
  gate(browserName, "empty-boot-shape", empty?.boot?.sources === 0 && empty?.boot?.tracks === 0
    && empty?.boot?.metersAttached === true,
  `the empty session map or control list named a source or a strip: ${JSON.stringify(empty?.boot)}`);
  gate(browserName, "empty-boot-meter", empty?.boot?.meterLeaseResult === 0
    && empty?.boot?.meterFrames >= 1
    && JSON.stringify(empty?.boot?.meterTrackCounts) === "[0]"
    && JSON.stringify(empty?.boot?.meterWidths) === "[2]",
  `the empty session's meter stream was not the master's two peaks: ${JSON.stringify(empty?.boot)}`);
  const first = empty?.firstStem;
  gate(browserName, "empty-boot-first-stem", first?.replyStatus === "ok" && first?.commandPath === "rebuild"
    && first?.replyBytes > 0
    && first?.sourcesAfter === 1
    && JSON.stringify(first?.tracksAfter) === JSON.stringify(["track"])
    && /^[0-9]+$/.test(first?.committedRevision ?? "")
    && first?.pendingRevision === first?.initialRevision
    && first?.finalRevision === first?.committedRevision
    && BigInt(first?.finalRevision ?? "0") > BigInt(first?.initialRevision ?? "0")
    && BigInt(first?.finalSample ?? "-1") === 0n,
  `the first stem was not committed once, by a structural transaction, and applied at the swap block: ${JSON.stringify(first)}`);
  // #1288: the added track fades in from its first played block over the session mute ramp, then
  // plays exactly as fed.
  gate(browserName, "empty-boot-first-stem", first?.heard === true && first?.exactInputOutput === true
    && first?.fadeShaped === true && first?.fireFrame === 0 && first?.rampFrames === 480
    && first?.renderedDigest === first?.expectedDigest,
  `the first stem did not fade in from its first played block and then play exactly as it was fed: ${JSON.stringify({ fadeShaped: first?.fadeShaped, exact: first?.exactInputOutput, fireFrame: first?.fireFrame, rampFrames: first?.rampFrames })}`);
  gate(browserName, "empty-boot-meter", first?.meterLeaseResult === 0 && first?.meterFrames >= 1
    && JSON.stringify(first?.meterTrackCounts) === "[1]"
    && JSON.stringify(first?.meterWidths) === "[4]"
    && Array.isArray(first?.structureEpochs) && first.structureEpochs.length === 1
    && Number.isSafeInteger(first.structureEpochs[0]) && first.structureEpochs[0] >= 1,
  `the meter stream after the first stem was not the strip's with one structure epoch: ${JSON.stringify(first)}`);

  // Issue #1295: one `SESSION_REPLACE` of document B before the first quantum, on this page without
  // cross-origin isolation (the module's `single` mode). The core commits it on the rebuild path,
  // the watermark reports R in effect from sample 0 (the swap block) with the outcome exact, no
  // source block underran, the capture is the module's own render of the same replace under Node,
  // and the host counted one blocking rebuild and the handler's allocations (D15-10).
  const edit = result.structuralReplace?.edit;
  gate(browserName, "structural-edit-path", edit?.replyStatus === "ok" && edit?.commandPath === "rebuild"
    && /^[0-9]+$/.test(edit?.committedRevision ?? "")
    && BigInt(edit.committedRevision) > BigInt(edit?.expectedRevision ?? "-1")
    && edit?.before?.appliedRevision === edit?.expectedRevision,
  `the replacement was not committed on the rebuild path: ${JSON.stringify({ status: edit?.replyStatus, path: edit?.commandPath, diagnostics: edit?.diagnostics })}`);
  gate(browserName, "structural-edit-early", edit?.pending?.appliedRevision === edit?.before?.appliedRevision
    && edit?.after?.appliedRevision === edit?.committedRevision
    && edit?.after?.appliedSample === "0"
    && edit?.watermark?.revision === edit?.committedRevision && edit?.watermark?.sample === "0",
  `the watermark did not report R in effect from the swap block, or moved before it: ${JSON.stringify({ before: edit?.before, pending: edit?.pending, after: edit?.after, watermark: edit?.watermark })}`);
  gate(browserName, "structural-edit-outcome", edit?.after?.appliedOutcome === "1" && edit?.watermark?.outcome === "1",
    `the replacement's outcome was not exact: status outcome ${edit?.after?.appliedOutcome}, watermark outcome ${edit?.watermark?.outcome}`);
  gate(browserName, "structural-edit-underruns", edit?.underrunBlocks === "0",
    `a source block underran over the capture: ${edit?.underrunBlocks}`);
  gate(browserName, "structural-edit-output", typeof edit?.renderedDigest === "string"
    && edit?.reference?.exact?.revision === edit?.committedRevision
    && edit.renderedDigest === edit.reference.exact.digest,
  `the capture is not the module's render of the same replace under Node: ${JSON.stringify({ browser: edit?.renderedDigest, reference: edit?.reference })}`);
  // Issue #1332 S9: in `worker` mode the Worker prepares the replacement off the audio thread, so
  // no rebuild blocks render and the worklet instance runs no control work: both counters stay 0
  // (D15-10; #1332 gate 5).
  gate(browserName, "structural-edit-blocking", edit?.before?.blockingRebuilds === 0
    && edit?.after?.blockingRebuilds === (worker ? 0 : 1),
  worker
    ? `the worker-mode replacement was counted as a blocking rebuild: ${edit?.before?.blockingRebuilds} -> ${edit?.after?.blockingRebuilds}`
    : `the single-mode replacement was not counted as exactly one blocking rebuild: ${edit?.before?.blockingRebuilds} -> ${edit?.after?.blockingRebuilds}`);
  gate(browserName, "structural-edit-control-allocations", worker
    ? edit?.before?.singleModeControlAllocations === 0 && edit?.after?.singleModeControlAllocations === 0
    : Number.isSafeInteger(edit?.before?.singleModeControlAllocations)
      && edit?.after?.singleModeControlAllocations > edit.before.singleModeControlAllocations,
  worker
    ? `the worklet instance counted control allocations in worker mode: ${edit?.before?.singleModeControlAllocations} -> ${edit?.after?.singleModeControlAllocations}`
    : `the handler's allocations under the replacement were not counted: ${edit?.before?.singleModeControlAllocations} -> ${edit?.after?.singleModeControlAllocations}`);
  // #1697 gate 3: the 1 MiB boot-ceiling document is committed, counted and adopted; its memory
  // growth is reported (below, against the boot's projection), never pinned.
  const large = result.structuralReplace?.large;
  gate(browserName, "structural-replace-large", large?.documentBytes === 1 << 20
    && large?.replyStatus === "ok" && large?.commandPath === "rebuild"
    && large?.after?.appliedRevision === large?.committedRevision
    && large?.before?.blockingRebuilds === 0 && large?.after?.blockingRebuilds === (worker ? 0 : 1)
    && (worker
      ? large?.before?.singleModeControlAllocations === 0 && large?.after?.singleModeControlAllocations === 0
      : large?.after?.singleModeControlAllocations > large?.before?.singleModeControlAllocations)
    && Number.isSafeInteger(large?.before?.memoryBytes) && Number.isSafeInteger(large?.after?.memoryBytes),
  `the 1 MiB replacement was not committed, counted and adopted: ${JSON.stringify(large)}`);
  // Issue #1332 S3: on a shared memory a growth gives `memory.buffer` a new identity, and a view
  // over the older object stays valid for its bytes, so the growth is no fault. The capture renders
  // 64 quanta on after the sixteen. Precondition: the memory grew. The capture is the module's
  // render of the same replace and feed under Node and plays after the growth, and the meter stream
  // carries the 192 strips past it. (A two-thread stale-view check is S9's.)
  gate(browserName, "structural-replace-grown", large?.after?.memoryBytes > large?.before?.memoryBytes,
    `the 1 MiB replacement did not grow the memory: ${JSON.stringify({ after: large?.after, before: large?.before })}`);
  gate(browserName, "structural-replace-grown", typeof large?.renderedDigest === "string"
    && large?.reference?.revision === large?.committedRevision
    && large.renderedDigest === large.reference.digest
    && large?.grownBlocks === 64 && large?.grownHeard === true,
  `the capture after the growth is not the module's render of the same replace under Node: ${JSON.stringify({ browser: large?.renderedDigest, reference: large?.reference, heard: large?.grownHeard })}`);
  gate(browserName, "structural-replace-grown", large?.meterLeaseResult === 0
    && large?.meterFramesAfterGrowth >= 1 && large?.meterFinite === true
    && large?.meterTrackCounts?.includes(192),
  `the meter stream failed after the growth: ${JSON.stringify({ lease: large?.meterLeaseResult, after: large?.meterFramesAfterGrowth, counts: large?.meterTrackCounts, finite: large?.meterFinite })}`);
  // Issue #1332 S9: the same 1 MiB replacement sent while a real context renders. The memory grew
  // (in the Worker on the worker leg, in the worklet's handler on the single leg), the revision
  // was applied, the context rendered 64 quanta past the commit with no render fault, and the
  // successor's 192 strips reached the page in a meter frame after the reply, over the re-cut views.
  const growth = result.structuralReplace?.growth;
  gate(browserName, "growth-while-rendering", growth?.meterLeaseResult === 0
    && growth?.replyStatus === "ok" && growth?.commandPath === "rebuild"
    && growth?.after?.memoryBytes > growth?.before?.memoryBytes
    && growth?.after?.appliedRevision === growth?.committedRevision
    && /^[0-9]+$/.test(growth?.after?.renderedQuanta ?? "") && /^[0-9]+$/.test(growth?.committed?.renderedQuanta ?? "")
    && BigInt(growth.after.renderedQuanta) >= BigInt(growth.committed.renderedQuanta) + 64n
    && growth?.after?.lastResult === 0
    && growth?.successorFramesAfterGrowth >= 1 && growth?.meterFinite === true,
  `the context did not render on across the growth with the successor's meter stream: ${JSON.stringify(growth ?? null)}`);
  // Worker leg only: the worklet rendered while the Worker parsed the document and grew the memory.
  // Meter frames are posted from `process()` (one per `meterBlocks` quanta), so frames that reached the
  // page between the send and the reply were rendered during the replacement: at least a quarter of
  // the quanta the reply's wall time spans, and at least four. On the single leg the parse runs in
  // the worklet's own handler and `process()` cannot run until it returns (a few frames already in
  // flight still land: 0 on Chromium, 5 on WebKit in an 834 ms reply), so no bound is held there.
  const spannedQuanta = Number.isFinite(growth?.replyMs) ? (growth.replyMs * 48_000) / (128 * 1_000) : Infinity;
  gate(browserName, "growth-while-rendering-concurrent", !worker
    || (Number.isSafeInteger(growth?.meterBlocks) && growth.meterBlocks >= 1
      && growth?.framesDuringReplace * growth.meterBlocks >= Math.max(4, spannedQuanta / 4)),
  `the worklet did not render while the Worker grew the memory: ${JSON.stringify({ framesDuringReplace: growth?.framesDuringReplace, replyMs: growth?.replyMs })}`);
  // R-S7-meters (3): a frame whose meter set the worklet has not cut is skipped and counted. In
  // single mode the handler that publishes a plan cuts its set first, so none is skipped; in worker
  // mode the Worker's view message may reach the worklet a few quanta after render adopts the plan,
  // so a bounded number may be. Reported below; held here.
  const skipped = [large?.after?.meterFramesSkipped, growth?.after?.meterFramesSkipped];
  gate(browserName, "meter-frames-skipped", skipped.every((count) => Number.isSafeInteger(count)
    && count >= 0 && count <= (worker ? METER_FRAMES_SKIPPED_BOUND : 0)),
  `meter frames skipped past the ${leg} leg's bound (${worker ? METER_FRAMES_SKIPPED_BOUND : 0}): ${JSON.stringify(skipped)}`);

  // Issue #143 E12: subscribe -> nonzero `trackGrDb`, an unarmed tap publishes none, with `firstSample`
  // monotonic and the windows tiling. The two runs render the *same* sixteen blocks, so their
  // audio must be identical: arming a declared tap may not move a sample.
  const observation = result.observation;
  // The arm is the core's `Observe` edit (#1589c): an ok monitor reply that names the arm's tag.
  gate(browserName, "observation", observation?.armed?.meterLeaseResult === 0
    && observation?.armed?.subscribeResult === 0 && observation?.armed?.subscribeStatus === "ok"
    && observation?.armed?.subscribePath === "monitor"
    && typeof observation?.armed?.subscribeArmTag === "string",
  `the Observe arm was not applied as a monitor edit with an arm tag: ${JSON.stringify(observation?.armed)}`);
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
  // A tap that was never armed publishes none, over at least one frame (#1535 D9).
  gate(browserName, "observation-disarmed", observation?.disarmed?.maximumTrackGrDb === 0
    && observation?.disarmed?.frames >= 1,
  `a tap that was never armed published a reduction, or the run received no frame: ${JSON.stringify(observation?.disarmed)}`);
  // Issue #1535: on one real AudioContext, an acknowledged unsubscribe stops the tap's reduction in
  // every later frame, while at least 48 later frames still carry the fed signal. The message lists
  // the failed checks (`unsubscribeFailures`, D5) first. Low-peak frames are reported, never failed
  // (root ruling R1).
  const unsubscribe = observation?.unsubscribe;
  const unsubscribeFailed = unsubscribeFailures(unsubscribe);
  gate(browserName, "observation-unsubscribe", unsubscribeFailed.length === 0,
    `${unsubscribeFailed.join(", ")}: ${JSON.stringify(unsubscribe ?? null)}`);
  // Issue #1332 S9: the leg's disarm rule. The single leg holds every frame after the
  // acknowledgement (one FIFO port); only the worker leg, whose acknowledgement and frames travel two
  // ports, may hold just the frames rendered past the boundary read after it.
  gate(browserName, "observation-unsubscribe-rule",
    unsubscribe?.disarmRule === (worker ? "render-position" : "port-order"),
  `the ${leg} leg checked the unsubscribe by the wrong rule: ${unsubscribe?.disarmRule}`);
  gate(browserName, "observation-identity", observation?.identicalAudio === true,
    "arming a declared tap moved a rendered sample");
  // Every real `AudioContext.resume()` ran inside a trusted click (the page counts them, split by
  // `window.event`) and the harness clicked exactly the controls the page declared: no direct
  // `resume()`, no missing click, no extra one (batch-verdict 2, B2).
  const resumes = result.resumes;
  gate(browserName, "resume-clicks", resumes?.direct === 0
    && Number.isInteger(resumes?.declared)
    && resumes.declared === (result.sdkResponse === null ? 2 : 5)
    && resumes.inClick === resumes.declared
    && resumes.planned === resumes.declared
    && resumes.clicksPerformed === resumes.declared,
  `a real AudioContext was resumed outside a trusted click, the page did not declare its growth and unsubscribe clicks (plus the three SDK clicks when the SDK runs), or the harness did not click exactly the declared controls: ${JSON.stringify(resumes)}`);

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
  // Issue #1333 D3: the staging read ran over real data -- an observation row -- so the
  // render-allocation gate below covers it.
  const stagingReads = result.stagingReads;
  gate(browserName, "staging-reads", stagingReads?.subscribeResult === 0
    && stagingReads?.subscribeStatus === "ok"
    && stagingReads?.observationReadResult === 0 && stagingReads?.observationRows === 1,
  `the staging reads did not all complete: ${JSON.stringify(stagingReads)}`);
  // Issue #1580 D2: the same run's strip `RESPONSE_GET`, sent through the host's `control` message,
  // was answered by exactly one `RESPONSE_READY`, which carries the reply's tag and outcome 0. The
  // render that answered it is inside the run's render-allocation count.
  const hostResponse = stagingReads?.trackResponse;
  gate(browserName, "track-response-host", hostResponse?.result === 0
    && hostResponse?.replyStatus === "ok" && hostResponse?.events === 1
    && hostResponse?.tagged === 1 && hostResponse?.status === "ok",
  `the host-level track response was not one tagged RESPONSE_READY with outcome 0: ${JSON.stringify(hostResponse)}`);
  // Issue #1333 D3: every booted instance, read before its dispose, made no allocator call in any
  // export its processor calls on the render thread after boot -- including everything the plan
  // executor does behind its `call_indirect`, which the static gate cannot see.
  const allocations = result.renderAllocations;
  gate(browserName, "render-allocations", Array.isArray(allocations)
    && allocations.length === RAW_INSTANCES
    && allocations.every((row) => typeof row?.workload === "string" && row.count === 0),
  `a worklet instance allocated on its render thread: ${JSON.stringify(allocations)}`);
  if (result.sdkResponse !== null) validateSdkResponse(browserName, result.sdkResponse, leg);
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
 * worklet. Each entry is one `Observe` transaction (#1589b): `ok` is call result 0 with an `ok`
 * reply; `bp` is the failed call of result 6 (backpressure), which carries no reply frame.
 */
function validateSdkObservationWait(browserName, wait) {
  const deadline = wait?.deadline;
  const render = wait?.render;
  const log = (entries) => JSON.stringify(Array.isArray(entries)
    ? entries.map((entry) => [entry?.result, entry?.status]) : null);
  const OK = [0, "ok"];
  const BP = [6, null];
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

const PROTOCOL_RELIABLE_KINDS = new Set(["sessionCommitted", "transportState", "observationEnded"]);
// #1583: no meterBatch until #1588 gives the browser a meter delivery (step 5 configures an
// empty meter table).
const PROTOCOL_LOSSY_KINDS = new Set(["counterSnapshot", "diagnostic"]);

/**
 * Issue #1571 gates 2, 3 and 5 in the browser. The page made the same calls, in the same order, on
 * the browser engine and on a headless engine of the same session and asset. Gate 2: every answer
 * (capabilities, the applied telemetry, the transport before and after `setTransport`, the counter
 * after one fader edit and the retained diagnostics) is equal. Gate 5: `diagnostics({ limit: 0 })`
 * is the core's refusal, the same on both. Gate 3: the reliable events are the headless engine's,
 * in its order (the edit's `sessionCommitted`, then `transportState` playing). The browser reads its
 * events once per message and its lossy lane holds one event (root's tranche-b note), so a block's
 * lossy telemetry may be replaced before it is read: each lossy event that arrived must be the
 * headless engine's event of the same block, in the headless order, and only the last block's
 * three (which no later block replaces) are required: each kind's last received event carries the
 * headless engine's last sample.
 */
function validateSdkProtocol(browserName, protocol) {
  const json = (value) => JSON.stringify(value);
  const browser = protocol?.browser;
  const headless = protocol?.headless;
  gate(browserName, "sdk-protocol-answers", protocol?.blocks === PROTOCOL_BLOCKS, "the page rendered another block count");
  gate(browserName, "sdk-protocol-answers", browser?.answers !== undefined && headless?.answers !== undefined
    && json(browser.answers) === json(headless.answers)
    && json(browser.after) === json(headless.after)
    && browser.editRevision === headless.editRevision
    && browser.answers.playing?.state === "playing"
    && browser.answers.stopped?.state === "stopped"
    && json(browser.answers.counters?.values?.map((row) => row.id)) === json(["liveValuesSuperseded"])
    && json(browser.answers.telemetry?.counterIds) === json(["liveValuesSuperseded"]),
  `the browser's answers are not the headless engine's: ${json({ browser: browser?.answers, headless: headless?.answers })}`);
  const refusal = browser?.answers?.refusal;
  gate(browserName, "sdk-protocol-refusal", refusal?.name === "MisoControlRefusal"
    && typeof refusal.status === "string" && typeof refusal.code === "string",
  // #2040: the browser-versus-headless equality is the answers gate's, which compares this object.
  `diagnostics({ limit: 0 }) is not the core's refusal: ${json({ browser: refusal, headless: headless?.answers?.refusal })}`);
  const events = Array.isArray(browser.events) ? browser.events : [];
  const reference = Array.isArray(headless.events) ? headless.events : [];
  // The request that caused an event is an endpoint's own ID: the browser reads the committed
  // revision with a probe before `TRANSPORT_SET`, which draws one ID the headless engine does not.
  const reliable = (rows) => rows.filter((event) => PROTOCOL_RELIABLE_KINDS.has(event.kind))
    .map(({ originRequestId: _origin, ...event }) => event);
  gate(browserName, "sdk-protocol-reliable", events.every((event) =>
    PROTOCOL_RELIABLE_KINDS.has(event.kind) || PROTOCOL_LOSSY_KINDS.has(event.kind))
    && json(reliable(events)) === json(reliable(reference))
    && json(reliable(events).map((event) => event.kind)) === json(["sessionCommitted", "transportState"])
    && reliable(events)[0].revision === browser.editRevision
    && reliable(events)[1].state === "playing",
  `the reliable events are not the headless engine's, in order: ${json({ browser: reliable(events), headless: reliable(reference) })}`);
  // Each lossy event that arrived is one of the headless engine's, and they keep its order.
  let at = 0;
  const lossy = events.filter((event) => PROTOCOL_LOSSY_KINDS.has(event.kind));
  const ordered = lossy.every((event) => {
    while (at < reference.length && json(reference[at]) !== json(event)) at += 1;
    if (at === reference.length) return false;
    at += 1;
    return true;
  });
  // The render's last block has no later block to replace its telemetry, so its diagnostic, meter
  // batch and counter snapshot arrive: for each kind, the last one received is the headless
  // engine's last (the last block's sample). A run that lacks a kind lost a lane, and one whose last
  // is an earlier block's read a block's telemetry late or never.
  const sampleOf = (event) => event.kind === "diagnostic" ? event.diagnostic?.sampleTime : event.observedSample;
  const lastOf = (rows, kind) => rows.filter((event) => event.kind === kind).at(-1);
  gate(browserName, "sdk-protocol-lossy", ordered
    && [...PROTOCOL_LOSSY_KINDS].every((kind) => {
      const received = lastOf(lossy, kind);
      const expected = lastOf(reference, kind);
      return received !== undefined && expected !== undefined
        && sampleOf(received) !== undefined && sampleOf(received) === sampleOf(expected);
    })
    && reference.filter((event) => PROTOCOL_LOSSY_KINDS.has(event.kind)).length === 2 * protocol.blocks,
  `a lossy event is not the headless engine's event of its block, in order: ${json({ browser: lossy, headless: reference })}`);
}

function validateSdkResponse(browserName, response, leg) {
  validateSdkRenderAllocations(browserName, response);
  validateSdkLiveBypass(browserName, response?.liveBypass);
  validateSdkProtocol(browserName, response?.protocol);
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
    && live.serialized === true && live.closedRefused === true
    && JSON.stringify(liveMembers.map((member) => member.rackValue)) === JSON.stringify([4, 5, 2, 5])
    && liveInputFilters !== undefined && liveCompressor !== undefined
    && JSON.stringify(liveMemberOrder) === JSON.stringify(expectedLiveMemberOrder)
    && live.excludedMemberCount === 1,
  "browser live response did not return the target subtotal, owned channels, membership, section codes, boundary identity, one capture at a time, or the closed refusal");
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
    && liveU64(managedInitial.snapshotToken),
  "managed track-response subscription did not match the one-shot owner membership and values");
  // #1579b gate 4: renders with no edit publish nothing; after an edit, one publication whose
  // capture is after the edit's applied sample (the clause `sdk-live-eq-cuts` carried). A capture
  // is stamped with its block's exclusive end, so equality would be a committed-model read.
  gate(browserName, "sdk-track-response-subscription", managedResponse?.unchangedOverRenders === true
    && managedResponse.unchangedNotificationCount === 0
    && managedResponse.changedPublication === true
    && liveU64(managedResponse.appliedAtSample)
    && BigInt(managedAfterRender?.capturedSample ?? -1) > BigInt(managedResponse.appliedAtSample)
    && managedResponse.publishedOnce === true
    && managedResponse.callbackCount === 1
    && managedResponse.callbackOwner === managedResponse.owner
    && managedResponse.callbackEpoch === managedResponse.epoch
    && managedResponse.callbackJob === managedResponse.job
    && BigInt(managedResponse.callbackRevision) > 1n
    && managedResponse.callbackMatches === true
    && managedResponse.automaticDelivery === true
    && managedAfterRender?.points === 5
    && finiteArray(managedAfterRender?.left)
    && finiteArray(managedAfterRender?.right),
  "managed track-response subscription published over renders with no edit, missed the edit, or published it more than once");
  const liveEq = observations?.liveEq;
  gate(browserName, "sdk-live-eq-cuts", liveEq !== undefined
    && liveEq.postRampComparedFrames >= 128
    && Number.isFinite(liveEq.postRampMaximumDifference)
    && liveEq.postRampMaximumDifference <= 1e-6,
  "packed browser EQ cuts did not reach settled headless PCM parity");
  // #1579b gate 4: the subscription reads the core's frames; no response Worker is created.
  gate(browserName, "sdk-track-response-subscription", managedResponse?.responseWorkersCreated === 0
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
    && observations.subscribeStatus === "ok"
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
    && validResidentWindow(observations.windows[0]),
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
    && resident.bounds?.maximumHandles > 0
    && resident.bounds?.maximumSelections > 0
    && resident.bounds?.maximumCadenceMs > 0,
  "resident observation subscription did not publish bounded owner identity and limits");
  // #1589 D4 (R4): `armed()` settled with the arm sample render applied, and no window it was given
  // opened before it.
  gate(browserName, "sdk-observation-armed", typeof resident.armedSample === "string"
    && /^[0-9]+$/.test(resident.armedSample)
    && Array.isArray(resident.windows) && resident.windows.length === 2
    && resident.windows.every((window) => window !== null
      && BigInt(window.firstSample) >= BigInt(resident.armedSample)),
  `armed() did not settle with the arm sample before the subscription's windows: ${JSON.stringify(resident.armedSample)}`);
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
  "SDK spectrum result did not preserve the complete bounded L/R window and sample span");
  // #1596 gate 5: on the single leg the core analyzes in the worklet's message handler, and the
  // handler's allocations are counted, never hidden. #1332 S9: on the worker leg it analyzes in the
  // Worker, and the worklet instance counts none.
  gate(browserName, "sdk-spectrum-control-allocations", spectrumRows.every((row) => Array.isArray(row.controlAllocations)
    && row.controlAllocations.every((value) => /^[0-9]+$/.test(value))
    && (leg === "worker"
      ? row.controlAllocations.every((value) => value === "0")
      : BigInt(row.controlAllocations[1]) > BigInt(row.controlAllocations[0]))),
  `the spectrum reads' control allocations are not the ${leg} leg's: ${JSON.stringify(spectrumRows.map((row) => row.controlAllocations))}`);
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
    // the render reports that gap.
    [`renderLoss (status=${continuous?.renderLoss?.status}, dropped=${continuous?.renderLoss?.droppedCaptures})`,
      continuous?.renderLoss?.status === "gap" && continuous?.renderLoss?.droppedCaptures === "16"],
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
  // Issue #1522 D2: each condition is named, and a failure prints the names that failed with the
  // record, so a red log says which condition failed. The record already carries scalars only (no
  // bin arrays); the per-notification lists are reduced to their length and distinct values.
  const hopConditions = {
    requestHop: configuredHop?.requestHopFrames === 1_024,
    boundsHop: configuredHop?.boundsHopFrames === 1_024,
    nativeStart: configuredHop?.nativeStart?.hopFrames === 1_024
      && configuredHop?.nativeStart?.sampleRateHz === 48_000
      && configuredHop?.nativeStart?.quantumFrames === 128
      && configuredHop?.nativeStart?.smoothingMs === 37.5,
    nativeRead: Array.isArray(configuredHop?.nativeRead)
      && configuredHop.nativeRead.length > 0
      && configuredHop.nativeRead.every((metadata) => metadata?.hopFrames === 1_024
        && metadata?.smoothingMs === 37.5),
    publicationCount: configuredHop?.publicationCount >= 2,
    notificationHops: Array.isArray(configuredHop?.notificationHops)
      && configuredHop.notificationHops.length >= 2
      && configuredHop.notificationHops.every((hop) => hop === 1_024),
    notificationSmoothing: Array.isArray(configuredHop?.notificationSmoothingMs)
      && configuredHop.notificationSmoothingMs.length >= 2
      && configuredHop.notificationSmoothingMs.every((smoothing) => smoothing === 37.5),
    publications: configuredPublications.length === 2
      && configuredPublications.every((publication) => publication?.metadata?.hopFrames === 1_024
        && publication.metadata.smoothingMs === 37.5
        && publication.metadata.sampleRateHz === 48_000
        && publication.metadata.quantumFrames === 128
        && canonicalU64(publication.metadata.capturedSample) !== undefined
        && canonicalU64(publication.metadata.endSample) > canonicalU64(publication.metadata.capturedSample)
        && publication.result?.windowFrames === 2_048
        && publication.result?.binCount === 1_025
        && publication.result?.capturedSample === publication.metadata.capturedSample
        && publication.result?.endSample === publication.metadata.endSample
        && publication.result?.finite === true
        && publication.result?.ownedArrays === true),
    startDelta: configuredHop?.startDelta === "1024",
    spans: configuredHop?.firstSpan === "2048" && configuredHop?.secondSpan === "2048",
    resultMetadata: configuredHop?.resultHopFrames === 1_024
      && configuredHop?.resultSmoothingMs === 37.5,
    ownedArrays: configuredHop?.ownedArrays === true,
  };
  const failedHopConditions = Object.keys(hopConditions).filter((name) => !hopConditions[name]);
  const listSummary = (list) => (Array.isArray(list)
    ? { length: list.length, distinct: [...new Set(list)] } : list);
  gate(browserName, "sdk-spectrum-hop", failedHopConditions.length === 0,
    "actual browser H1024 did not preserve the core's stream metadata, results, overlap, smoothing, and ownership: "
      + `failed ${JSON.stringify(failedHopConditions)} in ${JSON.stringify(configuredHop === undefined ? undefined : {
        ...configuredHop,
        notificationHops: listSummary(configuredHop?.notificationHops),
        notificationSmoothingMs: listSummary(configuredHop?.notificationSmoothingMs),
      })}`);
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

function mutate(result, mutation, leg) {
  const copy = structuredClone(result);
  const otherMode = leg === "worker" ? "single" : "worker";
  if (mutation === "instance-mode") copy.instanceModes[copy.instanceModes.length - 1] = otherMode;
  if (mutation === "sdk-instance-mode") copy.sdkResponse.instanceModes[0] = otherMode;
  const growth = copy.structuralReplace?.growth;
  if (mutation === "growth-while-rendering-meter") growth.successorFramesAfterGrowth = 0;
  if (mutation === "growth-while-rendering-stopped") growth.after.renderedQuanta = growth.committed.renderedQuanta;
  if (mutation === "growth-while-rendering-concurrent") growth.framesDuringReplace = 0;
  if (mutation === "meter-frames-skipped") {
    growth.after.meterFramesSkipped = leg === "worker" ? METER_FRAMES_SKIPPED_BOUND + 1 : 1;
  }
  if (mutation === "attestation") copy.attestation.outcome = "miso.unsupported.v1";
  if (mutation === "boot") copy.boot.ready = false;
  if (mutation === "native-corpus-digest") copy.corpus.browserDigests[0] = "0".repeat(64);
  if (mutation === "main-thread-stall") copy.stall.measuredStallMs = 0;
  if (mutation === "control-path-applied") copy.liveControls.exactRetargetedOutput = false;
  if (mutation === "control-path-meter") copy.liveControls.masterPeak = 0;
  if (mutation === "control-path-command") copy.liveControls.commandApplied = 0;
  if (mutation === "stall-live-controls-load") copy.stall.liveControlMeterFrames = 0;
  if (mutation === "structural-apply-path") copy.structuralApply.commandPath = "live";
  if (mutation === "structural-apply-early") {
    copy.structuralApply.pendingRevision = copy.structuralApply.committedRevision;
  }
  if (mutation === "structural-apply-output") copy.structuralApply.exactDelayedOutput = false;
  if (mutation === "structural-apply-meter") copy.structuralApply.meterFrames = 0;
  if (mutation === "empty-boot-silence") copy.emptyBoot.boot.silent = false;
  if (mutation === "empty-boot-shape") copy.emptyBoot.boot.tracks = 1;
  if (mutation === "empty-boot-first-stem") copy.emptyBoot.firstStem.exactInputOutput = false;
  if (mutation === "empty-boot-meter") copy.emptyBoot.firstStem.meterWidths = [2];
  const edit = copy.structuralReplace?.edit;
  // The capture of a page that never sent the replacement: the module's render of A alone.
  if (mutation === "structural-edit-skipped") edit.renderedDigest = edit.reference.skipped.digest;
  if (mutation === "structural-edit-adoption-shifted") edit.reference.exact = edit.reference.shifted;
  if (mutation === "structural-edit-path") edit.commandPath = "live";
  if (mutation === "structural-edit-early") edit.pending.appliedRevision = edit.committedRevision;
  if (mutation === "structural-edit-outcome") edit.after.appliedOutcome = "2";
  if (mutation === "structural-edit-underruns") edit.underrunBlocks = "1";
  if (mutation === "structural-edit-blocking-uncounted") edit.after.blockingRebuilds = 0;
  if (mutation === "structural-edit-control-allocations-uncounted") {
    edit.after.singleModeControlAllocations = edit.before.singleModeControlAllocations;
  }
  if (mutation === "structural-edit-blocking-counted") edit.after.blockingRebuilds = 1;
  if (mutation === "structural-edit-control-allocations-counted") edit.after.singleModeControlAllocations = 1;
  if (mutation === "structural-replace-large") copy.structuralReplace.large.replyStatus = "limit_exceeded";
  if (mutation === "structural-replace-grown-output") copy.structuralReplace.large.renderedDigest = "0".repeat(64);
  // Issue #143 E12's named red mutation: `observationArmed = 0`.
  if (mutation === "observation-armed") copy.observation.armed.maximumTrackGrDb = 0;
  if (mutation === "observation-arm-refused") copy.observation.armed.subscribeStatus = "not_found";
  if (mutation === "observation-disarmed") copy.observation.disarmed.maximumTrackGrDb = 1;
  if (mutation === "observation-disarmed-empty") copy.observation.disarmed.frames = 0;
  if (mutation === "observation-identity") copy.observation.identicalAudio = false;
  if (mutation === "observation-unsubscribe-rule") copy.observation.unsubscribe.disarmRule = "render-position";
  if (mutation === "resume-clicks-direct") copy.resumes.direct = 1;
  if (mutation === "sdk-track-response-worker") copy.sdkResponse.observations.managedResponse.responseWorkersCreated = 1;
  if (mutation === "sdk-track-response-unchanged") copy.sdkResponse.observations.managedResponse.unchangedOverRenders = false;
  if (mutation === "sdk-track-response-once") copy.sdkResponse.observations.managedResponse.publishedOnce = false;
  if (mutation === "sdk-track-response-early") {
    copy.sdkResponse.observations.managedResponse.appliedAtSample =
      copy.sdkResponse.observations.managedResponse.afterRender.capturedSample;
  }
  if (mutation === "sdk-live-response-serialized") copy.sdkResponse.observations.liveResponse.serialized = false;
  if (mutation === "sdk-live-response-sections") copy.sdkResponse.observations.liveResponse.members[0].rackValue = 0;
  if (mutation === "resume-clicks-missing") copy.resumes.clicksPerformed -= 1;
  if (mutation === "resume-clicks-extra") copy.resumes.clicksPerformed += 1;
  if (mutation === "observation-window") copy.observation.armed.firstSampleMonotonic = false;
  if (mutation === "sdk-observation-subscription-window-reversed") {
    const window = copy.sdkResponse.observations.resident.windows[0];
    window.endSample = window.firstSample;
  }
  if (mutation === "sdk-observation-subscription-window-malformed") {
    copy.sdkResponse.observations.resident.windows[0].endSample = "1024x";
  }
  if (mutation === "sdk-response") copy.sdkResponse.eq.points = 0;
  if (mutation === "sdk-observation") copy.sdkResponse.observations.readyStatuses[0] = "pending";
  if (mutation === "sdk-spectrum") copy.sdkResponse.spectrum.targets[0].finite = false;
  if (mutation === "sdk-spectrum-control-allocations") {
    const counts = copy.sdkResponse.spectrum.targets[0].controlAllocations;
    counts[1] = counts[0];
  }
  if (mutation === "sdk-spectrum-control-allocations-counted") {
    copy.sdkResponse.spectrum.targets[0].controlAllocations[1] = "1";
  }
  if (mutation === "sdk-spectrum-hop-missing") {
    delete copy.sdkResponse.spectrum.configuredHop.nativeStart.hopFrames;
  }
  if (mutation === "sdk-spectrum-continuous-hop-wrong") {
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
  if (mutation === "sdk-protocol-answers") copy.sdkResponse.protocol.browser.answers.capabilities.maximumFrameBytes = "1";
  if (mutation === "sdk-protocol-reliable") copy.sdkResponse.protocol.browser.events.reverse();
  if (mutation === "sdk-protocol-lossy") {
    // A lossy event of no block the headless engine rendered.
    copy.sdkResponse.protocol.browser.events.push({ kind: "counterSnapshot", observedSample: "1", values: [] });
  }
  if (mutation === "sdk-protocol-lossy-none") {
    copy.sdkResponse.protocol.browser.events = copy.sdkResponse.protocol.browser.events
      .filter((event) => event.kind === "sessionCommitted" || event.kind === "transportState");
  }
  if (mutation === "sdk-protocol-lossy-first-block") {
    // A host that delivered only the first block's telemetry: the headless engine's first three.
    const protocol = copy.sdkResponse.protocol;
    const first = protocol.headless.events.filter((event) =>
      PROTOCOL_LOSSY_KINDS.has(event.kind)).slice(0, PROTOCOL_LOSSY_KINDS.size);
    protocol.browser.events = [
      ...protocol.browser.events.filter((event) => PROTOCOL_RELIABLE_KINDS.has(event.kind)), ...first,
    ];
  }
  if (mutation === "sdk-protocol-refusal") {
    // Both engines give the same non-refusal, so the answers gate (which compares the two) holds
    // and only the refusal gate's own predicate sees it.
    for (const side of ["browser", "headless"]) copy.sdkResponse.protocol[side].answers.refusal.name = "Error";
  }
  if (mutation === "render-allocations") copy.renderAllocations[0].count = 1;
  if (mutation === "staging-reads") copy.stagingReads.observationReadResult = 6;
  if (mutation === "track-response-host-outcome") copy.stagingReads.trackResponse.status = "unavailable";
  if (mutation === "track-response-host-tag") copy.stagingReads.trackResponse.tagged = 0;
  const wait = copy.sdkResponse?.observations?.wait;
  if (mutation === "sdk-observation-wait-unbounded") wait.deadline.settled = false;
  if (mutation === "sdk-observation-wait-late") wait.deadline.elapsedMs = 5_000;
  if (mutation === "sdk-observation-wait-early") wait.deadline.elapsedMs = 0;
  if (mutation === "sdk-observation-wait-untyped") wait.deadline.errorCode = "invalidArgument";
  if (mutation === "sdk-observation-wait-rendered") wait.deadline.renderedQuantaAfterRefusal = "1";
  if (mutation === "sdk-observation-wait-after-render") wait.deadline.closeAfterRender.resolved = false;
  if (mutation === "sdk-observation-wait-resend") {
    wait.render.commands.splice(wait.render.commands.length - 1, 0,
      { result: 6, status: null, records: wait.render.commands[1].records });
  }
  if (mutation === "sdk-observation-armed") copy.sdkResponse.observations.resident.armedSample = null;
  if (mutation === "sdk-render-allocations") copy.sdkResponse.renderAllocations[0].count = 1;
  if (mutation === "sdk-render-allocations-missing") copy.sdkResponse.renderAllocations.splice(1, 1);
  if (mutation === "page-errors") copy.pageErrors.playwright.push("pageerror: Error: planted");
  if (mutation === "page-errors-window") copy.pageErrors.window.push("unhandledrejection: Error: planted");
  if (mutation === "page-errors-missing") delete copy.pageErrors;
  return copy;
}

/// Issue #1535 D5: one row per named check of `unsubscribeFailures`, each a green run at the bounds
/// (four armed frames, 48 signal frames, 40 sources in flight) with one field moved past its bound,
/// and the R1 row: low-peak frames after the acknowledgement fail nothing. Then the request-failure
/// form a refused host request reaches the page in (D3.1).
function unsubscribeFailureProofs() {
  const green = {
    contextStateBeforeBoot: "suspended",
    meterLeaseResult: 0,
    subscribeResult: 0,
    subscribePath: "monitor",
    armedFrames: 4,
    armedMaximumTrackGrDb: 5.2,
    renderedAfterSubscribe: "3",
    renderedBeforeUnsubscribe: "4",
    unsubscribeResult: 0,
    unsubscribePath: "monitor",
    framesAfterAcknowledgement: 60,
    maximumTrackGrDbAfterAcknowledgement: 0,
    framesAfterDisarm: 60,
    maximumTrackGrDbAfterDisarm: 0,
    signalShortfall: false,
    signalFramesAfterAcknowledgement: 48,
    lowPeakFramesAfterAcknowledgement: 0,
    minimumPeakAfterAcknowledgement: 0.27,
    feedBehind: null,
    sourceExhausted: null,
    maximumSourcesInFlight: 40,
  };
  const rows = [
    ["green", {}, []],
    ["R1 dropout", { lowPeakFramesAfterAcknowledgement: 3, minimumPeakAfterAcknowledgement: 0 }, []],
    ["context", { contextStateBeforeBoot: "running" }, ["context"]],
    ["armed frames", { armedFrames: 3 }, ["armed"]],
    ["armed reduction", { armedMaximumTrackGrDb: 0 }, ["armed"]],
    ["armed path", { subscribePath: null }, ["armed"]],
    ["render-between", { renderedBeforeUnsubscribe: "3" }, ["render-between"]],
    ["unsubscribe-ack", { unsubscribePath: null }, ["unsubscribe-ack"]],
    ["reduction", { maximumTrackGrDbAfterDisarm: 1 }, ["reduction"]],
    ["reduction, no frame (D9)", { framesAfterDisarm: 0 }, ["reduction"]],
    ["shortfall frames", { signalFramesAfterAcknowledgement: 47 }, ["shortfall"]],
    ["shortfall flag", { signalShortfall: true }, ["shortfall"]],
    ["shortfall, source exhausted", { sourceExhausted: { renderBlock: 481, fed: 512 } }, ["shortfall"]],
    ["feed in flight", { maximumSourcesInFlight: 41 }, ["feed"]],
    ["feed behind", { feedBehind: { block: 20, renderBlock: 21 } }, ["feed"]],
  ];
  for (const [name, change, expected] of rows) {
    assert.deepEqual(unsubscribeFailures({ ...green, ...change }), expected,
      `observation-unsubscribe: table row "${name}"`);
  }
  // A feed that stops at the acknowledgement cannot supply the signal frames from the ring alone.
  assert(qualificationConstants.unsubscribeSignalFrames > qualificationConstants.ringBlocks,
    "observation-unsubscribe: the signal-frame count does not exceed the ring depth");
  const refused = requestFailure("status", "status", { status: 1, source: 3 },
    Object.freeze({ tag: "miso.error.v1", requestId: 0, result: 6 })).message;
  for (const part of ["at status", "kind=status", "\"result\":6", "\"requestId\":0", "{\"status\":1,\"source\":3}"]) {
    assert(refused.includes(part), `request-failure: ${part} missing from ${refused}`);
  }
  assert(!refused.includes("[object Object]"), `request-failure: ${refused}`);
  assert(requestFailure("observe", "observe", {}, new Error("x")).message.endsWith("cause=x"),
    "request-failure: an Error cause lost its message");
}

/// The stall signature (`classifyStallSignature`, root ruling 2026-10-12) is `full` only when all of
/// its conditions hold. A full record is held full, and each condition removed alone, from an
/// otherwise full record, must give `full: false`, so a record missing any one condition can never
/// be listed as an exemption.
function stallSignatureProofs() {
  const sample = (requestId) => ({
    contextState: "running",
    currentTime: 0.021333333333333333,
    status: { requestId, lastResult: 0, renderedQuanta: 8n },
  });
  const full = { atResume: sample(39), atTimeout: sample(2282), repliesSent: 3, replyIds: [40, 100, 2282] };
  const signature = classifyStallSignature(full);
  assert.equal(signature.full, true, "stall-signature: the full record was not classified full");
  assert.deepEqual(signature.replyIds, [40, 2282]);
  assert.equal(signature.repliesAnswered, 3);
  const without = {
    "context state not running at the timeout": { atTimeout: { ...sample(2282), contextState: "suspended" } },
    "context state not running at the resume": { atResume: { ...sample(39), contextState: "interrupted" } },
    "currentTime moved": { atTimeout: { ...sample(2282), currentTime: 0.0426 } },
    "renderedQuanta moved": { atTimeout: { ...sample(2282), status: { ...sample(2282).status, renderedQuanta: 9n } } },
    "one status reply": { replyIds: [2282] },
    "no status reply": { replyIds: [] },
    "last result not 0": { atTimeout: { ...sample(2282), status: { ...sample(2282).status, lastResult: 6 } } },
    "timeout sample unanswered": { atTimeout: null },
  };
  for (const [missing, change] of Object.entries(without)) {
    assert.equal(
      classifyStallSignature({ ...full, ...change }).full,
      false,
      `stall-signature: a record without ${missing} was classified full`,
    );
  }
}

function mutationProofs(browserName, result, leg) {
  const mutations = MUTATIONS
    .filter(({ mutation }) => result.sdkResponse !== null || !mutation.startsWith("sdk-"))
    .filter((entry) => entry.leg === undefined || entry.leg === leg);
  for (const { mutation, gate: expected } of mutations) {
    assert.throws(
      () => validate(browserName, mutate(result, mutation, leg), leg),
      (error) => error instanceof Error && error.message.startsWith(`${browserName}: ${expected}:`),
      `${browserName}: ${mutation}: red mutation escaped its gate ${expected}`,
    );
  }
  if (result.sdkResponse === null) return;
  // #1522: the hop record's sample fields are decimal strings; a pair whose end has one more digit
  // than its start (here 98304 to 100352, about 2.05 s at 48 kHz) is a valid pair.
  const hopWidth = structuredClone(result);
  const hopPublications = hopWidth.sdkResponse.spectrum?.configuredHop?.publications;
  if (Array.isArray(hopPublications) && hopPublications.length === 2) {
    [["98304", "100352"], ["99328", "101376"]].forEach(([capturedSample, endSample], index) => {
      Object.assign(hopPublications[index].metadata, { capturedSample, endSample });
      Object.assign(hopPublications[index].result, { capturedSample, endSample });
    });
    assert.doesNotThrow(
      () => validate(browserName, hopWidth, leg),
      `${browserName}: sdk-spectrum-hop-width: valid decimal-width-crossing hop pair was refused`,
    );
  }
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
    () => validate(browserName, widthCrossing, leg),
    `${browserName}: sdk-observation-subscription-window-width: valid decimal-width-crossing span was refused`,
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

function normalizedRow(browserName, leg, browserVersion, outcome) {
  const passed = outcome === "simd128 supported";
  return {
    browser: browserName,
    leg,
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

/**
 * Issue #1295 D2: the structural-edit row's references, rendered by the served module under Node
 * from the page's own frame: the replace before block 0 (the swap block the page's capture
 * starts at), the same replace one block late, and no replace at all (the two mutations' captures).
 */
async function structuralEditReferences(artifacts, edit) {
  const [documentA, documentB, source] = await Promise.all([
    readFile(path.join(FIXTURE, "session.json")),
    readFile(path.join(FIXTURE, "structural-edit-session.json")),
    readFile(path.join(FIXTURE, "source.json"), "utf8").then(JSON.parse),
  ]);
  const frame = Uint8Array.from(edit.frameHex.match(/../g) ?? [], (byte) => Number.parseInt(byte, 16));
  const render = (replaceBlock) => structuralEditReference({
    modulePath: path.join(path.resolve(artifacts), WASM_ARTIFACT),
    documentA,
    documentB,
    source,
    frame,
    sourceRingFrames: edit.sourceRingFrames,
    blocks: edit.blocks,
    replaceBlock,
  });
  return { exact: await render(0), shifted: await render(1), skipped: await render(null) };
}

/// Issue #1332 S3: the Node reference of the grown capture: session A, the 1 MiB replace before
/// block 0 with the page's frame, the page's feed right after it, and the capture's quanta.
async function largeReplaceReference(artifacts, large) {
  const documentA = await readFile(path.join(FIXTURE, "session.json"));
  return structuralEditReference({
    modulePath: path.join(path.resolve(artifacts), WASM_ARTIFACT),
    documentA,
    documentB: worstReplaceDocument(),
    source: { sourceId: "fixture-source", blocks: [] },
    frame: Uint8Array.from(large.frameHex.match(/../g) ?? [], (byte) => Number.parseInt(byte, 16)),
    sourceRingFrames: large.sourceRingFrames,
    blocks: large.blocks,
    replaceBlock: 0,
    sourceAfterReplace: largeReplaceSource(),
  });
}

async function qualifyBrowser(browserName, leg, engine, origin, proveMutations, sdkEnabled, protocolReference, structuralReference) {
  // Issue #1332 S9: gate messages and reports name the leg, so a red line says which page it came from.
  const label = `${browserName} ${leg}`;
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
    // Issue #1533 D1: two channels, one gate. Playwright's `pageerror` (uncaught main-thread
    // exceptions), and page-side `error`/`unhandledrejection` listeners installed before any page
    // script runs.
    const pageErrors = [];
    page.on("pageerror", (error) => {
      diagnostics.push(`pageerror: ${error.message}`);
      pageErrors.push(`pageerror: ${error.name}: ${error.message}`);
    });
    page.on("response", (response) => {
      if (!response.ok()) diagnostics.push(`HTTP ${response.status()}: ${response.url()}`);
    });
    page.setDefaultTimeout(120000);
    await page.addInitScript(() => {
      const list = [];
      Object.defineProperty(window, "__qualificationPageErrors", {
        value: list, writable: false, configurable: false, enumerable: false,
      });
      window.addEventListener("unhandledrejection", (event) => {
        const reason = event.reason;
        list.push(`unhandledrejection: ${reason?.name ?? typeof reason}: ${reason?.message ?? String(reason)}`);
      });
      window.addEventListener("error", (event) => {
        list.push(`error: ${event.message} at ${event.filename}:${event.lineno}`);
      });
    });
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
          throw new Error(`${label}: the page did not finish or offer a declared control to click (${selector}; ${clicksPerformed} of ${plan.length} clicks done) within ${PAGE_DEADLINE_MS} ms: ${outcome.error?.message ?? "total deadline"}; page diagnostics: ${diagnostics.join("; ")}`);
        }
        clicksPerformed += 1;
      }
      const outcome = await Promise.race([completion, expired]);
      if (outcome.kind === "deadline") {
        throw new Error(`${label}: the page did not finish within ${PAGE_DEADLINE_MS} ms (${clicksPerformed} of ${plan.length} declared clicks done); page diagnostics: ${diagnostics.join("; ")}`);
      }
    } finally {
      clearTimeout(deadlineTimer);
    }
    const result = await execution;
    // Issue #1533 D2: read the page list once, after two task turns so a rejection the last
    // microtask checkpoint left unhandled has reached its event. A page that does not answer fails
    // by name at the 10 s timer (or with the evaluate's error); no retry. The failure is raised
    // after the `browser-execution` gate, which names a dead page's own error first (D4).
    let errorReadTimer;
    const errorRead = await Promise.race([
      page.evaluate(async () => {
        for (let turn = 0; turn < 2; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        return [...window.__qualificationPageErrors];
      }).then((list) => ({ kind: "read", list }), (error) => ({ kind: "error", error })),
      new Promise((resolve) => {
        errorReadTimer = setTimeout(() => resolve({ kind: "timeout" }), 10_000);
      }),
    ]).finally(() => clearTimeout(errorReadTimer));
    if (errorRead.kind === "read") {
      result.pageErrors = { playwright: [...pageErrors], window: [...errorRead.list] };
    }
    if (result.resumes !== undefined) {
      result.resumes.planned = plan.length;
      result.resumes.clicksPerformed = clicksPerformed;
    }
    const stallSignature = result.qualificationError?.value?.stallSignature;
    if (stallSignature !== undefined) {
      // A bounded render wait timed out. The signature is what classifies the failure (full means a
      // context that said "running" and rendered nothing while the processor still answered).
      process.stdout.write(`${label}: stall-signature: ${JSON.stringify(stallSignature)}\n`);
    }
    gate(label, "browser-execution", result.qualificationError === undefined,
      `${JSON.stringify(result.qualificationError, (_key, value) => (typeof value === "bigint" ? `${value}n` : value))}${diagnostics.length === 0 ? "" : `; ${diagnostics.join("; ")}`}`);
    if (errorRead.kind === "timeout") {
      throw new Error(`${label}: page-errors: the page did not answer the error read within 10 s; page diagnostics: ${diagnostics.join("; ")}`);
    }
    if (errorRead.kind === "error") {
      throw new Error(`${label}: page-errors: the error read failed: ${errorRead.error?.message ?? String(errorRead.error)}; page diagnostics: ${diagnostics.join("; ")}`);
    }
    if (result.sdkResponse?.protocol !== undefined) {
      // #1571: the headless half of the protocol row runs here, in Node, on the page's document.
      result.sdkResponse.protocol.headless = await protocolReference(result.sdkResponse.protocol);
    }
    if (result.structuralReplace?.edit?.frameHex !== undefined) {
      result.structuralReplace.edit.reference = await structuralReference(result.structuralReplace.edit);
    }
    if (result.structuralReplace?.large?.frameHex !== undefined) {
      result.structuralReplace.large.reference = await structuralReference(result.structuralReplace.large, "large");
    }
    const outcome = validate(label, result, leg);
    if (proveMutations) mutationProofs(label, result, leg);
    if (Array.isArray(result.renderAllocations)) {
      // Issue #1333 D3: the per-instance counts the render-allocations gate just held to zero.
      process.stdout.write(`${label}: render-allocations: ${result.renderAllocations
        .map((row) => `${row.workload}=${row.count}`).join(" ")}\n`);
    }
    const replace = result.structuralReplace;
    if (replace?.large !== undefined) {
      // #1295 (#1697 gate 3): the module's memory growth across the 1 MiB replacement, against the
      // boot's projection `document_bytes * PARSE_TRANSIENT_MULTIPLIER` (20); reported, not pinned.
      const growth = replace.large.after.memoryBytes - replace.large.before.memoryBytes;
      const projection = replace.large.documentBytes * 20;
      process.stdout.write(`${label}: structural-replace-large: ${JSON.stringify({
        documentBytes: replace.large.documentBytes,
        memoryBytesBefore: replace.large.before.memoryBytes,
        memoryBytesCommitted: replace.large.committed.memoryBytes,
        memoryBytesAfter: replace.large.after.memoryBytes,
        appliedSample: replace.large.after.appliedSample,
        growth,
        projection,
        aboveProjection: growth > projection,
        controlAllocations: replace.large.after.singleModeControlAllocations - replace.large.before.singleModeControlAllocations,
        underrunBlocks: replace.large.underrunBlocks,
        meterFramesAfterGrowth: replace.large.meterFramesAfterGrowth,
        meterFramesSkipped: replace.large.after.meterFramesSkipped,
        digest: replace.large.renderedDigest,
      })}\n`);
      // The commit-to-adoption delay of the exact case, recorded, not gated (D6 non-goal).
      process.stdout.write(`${label}: structural-edit: ${JSON.stringify({
        committedRevision: replace.edit.committedRevision,
        commitSample: replace.edit.pending.nextAbsoluteSample,
        appliedSample: replace.edit.after.appliedSample,
        outcome: replace.edit.after.appliedOutcome,
        blockingRebuilds: replace.edit.after.blockingRebuilds,
        controlAllocations: replace.edit.after.singleModeControlAllocations - replace.edit.before.singleModeControlAllocations,
        digest: replace.edit.renderedDigest,
      })}\n`);
    }
    if (replace?.growth !== undefined) {
      // Issue #1332 S9: the replacement sent while a real context rendered, with the meter frames
      // the worklet rendered while it was pending and the frames it skipped (bounded above).
      const growth = replace.growth;
      process.stdout.write(`${label}: growth-while-rendering: ${JSON.stringify({
        memoryBytesBefore: growth.before.memoryBytes,
        memoryBytesAfter: growth.after.memoryBytes,
        replyMs: Math.round(growth.replyMs),
        framesDuringReplace: growth.framesDuringReplace,
        quantaDuringReplace: Number(BigInt(growth.committed.renderedQuanta) - BigInt(growth.before.renderedQuanta)),
        quantaAfterCommit: Number(BigInt(growth.after.renderedQuanta) - BigInt(growth.committed.renderedQuanta)),
        successorFramesAfterGrowth: growth.successorFramesAfterGrowth,
        meterFramesSkipped: growth.after.meterFramesSkipped,
        blockingRebuilds: growth.after.blockingRebuilds,
      })}\n`);
    }
    const unsubscribe = result.observation?.unsubscribe;
    if (unsubscribe !== undefined) {
      // Issue #1535: what the observation-unsubscribe gate just held, with the dropouts it reports
      // and never fails (root ruling R1).
      process.stdout.write(`${label}: observation-unsubscribe: ${JSON.stringify({
        framesAfterAcknowledgement: unsubscribe.framesAfterAcknowledgement,
        disarmRule: unsubscribe.disarmRule,
        framesAfterDisarm: unsubscribe.framesAfterDisarm,
        signalFramesAfterAcknowledgement: unsubscribe.signalFramesAfterAcknowledgement,
        lowPeakFramesAfterAcknowledgement: unsubscribe.lowPeakFramesAfterAcknowledgement,
        minimumPeakAfterAcknowledgement: unsubscribe.minimumPeakAfterAcknowledgement,
        armedFrames: unsubscribe.armedFrames,
        fedBlocks: unsubscribe.fedBlocks,
        maximumSourcesInFlight: unsubscribe.maximumSourcesInFlight,
      })}\n`);
    }
    if (Array.isArray(result.sdkResponse?.renderAllocations)) {
      // Issue #1476 D4: the per-SDK-instance counts the sdk-render-allocations gate just held to zero.
      process.stdout.write(`${label}: sdk-render-allocations: ${result.sdkResponse.renderAllocations
        .map((row) => `${row.workload}=${row.count}`).join(" ")}\n`);
    }
    const protocol = result.sdkResponse?.protocol;
    if (protocol?.browser !== undefined) {
      // #1571b: what the sdk-protocol gates just held, with the lossy events the browser received
      // (any number may be replaced before they are read, and that fails nothing).
      const kinds = (events) => events.map((event) => event.kind).join(",");
      process.stdout.write(`${label}: sdk-protocol: browser events ${kinds(protocol.browser.events)}; headless events ${kinds(protocol.headless.events)}\n`);
    }
    process.stdout.write(`${label}: all qualification gates passed (${browser.version()})\n`);
    const row = normalizedRow(browserName, leg, browser.version(), outcome);
    if (sdkEnabled) row.gates.sdkResponse = "pass";
    return row;
  } finally {
    await browser.close();
  }
}

/// Issue #1332 S9: a run's row against the recorded one for its browser and leg. A recorded row
/// with no `leg` was recorded before the module had two modes (the last release's run, a page
/// without isolation); until a release re-records the matrix with both legs, it is the recorded
/// floor and outcome for its browser on either leg, and the comparison leaves the run's leg out.
function checkedRowFor(checked, browserName, leg) {
  return checked.browsers.find((row) => row.browser === browserName && row.leg === leg)
    ?? checked.browsers.find((row) => row.browser === browserName && row.leg === undefined);
}

function comparableRow(actual, expected) {
  if (expected?.leg !== undefined) return actual;
  const { leg: _leg, ...legless } = actual;
  return legless;
}

function validateCheckedRow(browserName, leg, actual, checked) {
  const label = `${browserName} ${leg}`;
  const expected = checkedRowFor(checked, browserName, leg);
  gate(label, "deployment-matrix", expected !== undefined,
    "checked matrix has no row for this browser and leg");
  gate(label, "deployment-matrix", JSON.stringify(comparableRow(actual, expected)) === JSON.stringify(expected),
    "checked browser floor/outcome differs from this CI run");
  const mutated = structuredClone(expected);
  mutated.versionFloor = `${mutated.versionFloor}-red-mutation`;
  assert.throws(
    () => gate(label, "deployment-matrix",
      JSON.stringify(comparableRow(actual, mutated)) === JSON.stringify(mutated), "red mutation"),
    (error) => error instanceof Error
      && error.message.startsWith(`${label}: deployment-matrix:`),
    `${label}: deployment-matrix: red mutation escaped its gate`,
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
    throw new Error("usage: npm run qualify -- --artifacts DIR [--browser NAME] [--leg single|worker|all] [--check-matrix|--record-matrix --candidate-commit 40_HEX] [--self-test-mutations] [--pcm-feed-suspended-only|--blake3-only [--record-blake3 --candidate-commit 40_HEX]]");
  }
  const browserOption = option("--browser") ?? "all";
  // Issue #1332 S9: the two legs. `single` is the page without cross-origin isolation (the module's
  // `single` mode, every control call in the worklet's message handler); `worker` is the same
  // qualification on an isolated page (a control Worker beside the render-only worklet).
  const legOption = option("--leg") ?? "all";
  const legNames = legOption === "all" ? Object.keys(LEGS) : [legOption];
  for (const legName of legNames) {
    if (!(legName in LEGS)) throw new Error(`unknown leg: ${legName}`);
  }
  const browserNames = browserOption === "all" ? Object.keys(ENGINES) : [browserOption];
  for (const browserName of browserNames) {
    if (!(browserName in ENGINES)) throw new Error(`unknown browser: ${browserName}`);
  }
  const checkMatrix = process.argv.includes("--check-matrix");
  const recordMatrix = process.argv.includes("--record-matrix");
  const proveMutations = process.argv.includes("--self-test-mutations");
  const blake3Only = process.argv.includes("--blake3-only");
  const recordBlake3 = process.argv.includes("--record-blake3");
  if (blake3Only && (recordMatrix || checkMatrix || proveMutations)) {
    throw new Error("--blake3-only cannot record/check the full matrix or its mutations");
  }
  if (recordBlake3 && (!blake3Only || browserOption !== "all")) {
    throw new Error("--record-blake3 requires --blake3-only --browser all");
  }
  const feedOnly = process.argv.includes("--pcm-feed-suspended-only");
  if (feedOnly && blake3Only) throw new Error("select only one focused qualification row");
  if (feedOnly && (recordMatrix || checkMatrix || proveMutations)) {
    throw new Error("--pcm-feed-suspended-only cannot record/check the full matrix or its mutations");
  }
  if (recordMatrix && (browserOption !== "all" || legOption !== "all")) {
    throw new Error("--record-matrix requires --browser all --leg all");
  }
  // The BLAKE3 Worker and the suspended PCM feed rows do not depend on the leg (the feed row runs
  // on an isolated page in an explicit `single` instance until S11): they run once, in the single
  // leg.
  if ((blake3Only || feedOnly) && !legNames.includes("single")) {
    throw new Error("--blake3-only and --pcm-feed-suspended-only run in the single leg");
  }
  const candidateCommit = option("--candidate-commit");
  if ((recordMatrix || recordBlake3) && (candidateCommit === null || !CANONICAL_COMMIT.test(candidateCommit))) {
    throw new Error("recording requires --candidate-commit as canonical lowercase 40-hex");
  }
  const artifactDigest = await sha256(path.join(path.resolve(artifacts), WASM_ARTIFACT));
  const sdkRoot = option("--sdk-root");
  // Full qualification builds and serves the bundle: raw live-control rows send `SET_CONTROL`
  // transaction through it (#1563B). `--sdk-root` selects which SDK it comes from, and whether the
  // SDK rows run; without it the repository's own `sdk/` (with its `node_modules`) is bundled.
  const sdkBundle = blake3Only ? undefined : await buildSdkBundle(path.resolve(sdkRoot ?? path.join(HERE, "../../../sdk")));
  const headlessEntry = blake3Only ? undefined : await sdkHeadlessEntry(path.resolve(sdkRoot ?? path.join(HERE, "../../../sdk")));

  if (proveMutations) {
    const served = await artifactSetProofs(artifacts);
    process.stdout.write(`artifact set: the exact ${served}-file shipped set is pinned\n`);
    unsubscribeFailureProofs();
    stallSignatureProofs();
    process.stdout.write("observation-unsubscribe: each named check is red past its bound; a dropout fails nothing\n");
    process.stdout.write("stall signature: each missing condition gives full=false\n");
  }

  // Issue #1061: `--check-matrix` holds each browser's floor and outcome, the Playwright version
  // and the generated document to `results.json` -- all independent of the module. Its lineage
  // (`wasmSha256`, `candidateCommit`) describes the last recorded qualification; only a release
  // change must record one of the bytes it ships, and `scripts/web-audioworklet-identity.py`
  // checks that (docs/RELEASE.md).
  const checked = checkMatrix
    ? JSON.parse(await readFile(RESULTS_PATH, "utf8"))
    : null;
  const singleLeg = legNames.includes("single");
  const server = await startQualificationServer({ artifacts, sdkBundle });
  const isolatedServer = (singleLeg && (sdkRoot !== null || feedOnly)) || (legNames.includes("worker") && !blake3Only && !feedOnly)
    ? await startQualificationServer({ artifacts, sdkBundle, isolated: true }) : undefined;
  try {
    const rows = [];
    const blake3Rows = [];
    for (const browserName of browserNames) {
      if (singleLeg && !feedOnly) blake3Rows.push(await qualifyBlake3(browserName, ENGINES[browserName], server.origin));
      if (blake3Only) continue;
      if (singleLeg && (sdkRoot !== null || feedOnly)) {
        await qualifySuspendedPcmFeed(browserName, ENGINES[browserName], isolatedServer.origin);
      }
      if (feedOnly) continue;
      for (const leg of legNames) {
        const started = Date.now();
        const row = await qualifyBrowser(browserName, leg, ENGINES[browserName],
          LEGS[leg].isolated ? isolatedServer.origin : server.origin, proveMutations, sdkRoot !== null,
          (protocol) => headlessProtocolReference(headlessEntry, artifacts, protocol),
          (edit, kind) => (kind === "large"
            ? largeReplaceReference(artifacts, edit) : structuralEditReferences(artifacts, edit)));
        process.stdout.write(`${browserName} ${leg}: leg took ${((Date.now() - started) / 1000).toFixed(1)} s\n`);
        rows.push(row);
        if (checked !== null) validateCheckedRow(browserName, leg, row, checked);
      }
    }
    if (checked !== null) {
      gate("matrix", "playwright-version", checked.playwrightVersion === PLAYWRIGHT_VERSION,
        "package and checked results use different Playwright versions");
      gate("matrix", "generated-document", renderMatrix(checked)
        === await readFile(MATRIX_PATH, "utf8"), "document was not regenerated from results.json");
    }
    const blake3Qualification = {
      candidateCommit,
      sha256: await sha256(path.join(path.resolve(artifacts), "miso-engine-v1-blake3.simd128.wasm")),
      browsers: blake3Rows,
    };
    if (recordBlake3) {
      const results = JSON.parse(await readFile(RESULTS_PATH, "utf8"));
      results.blake3Qualification = blake3Qualification;
      await writeFile(RESULTS_PATH, `${JSON.stringify(results, null, 2)}\n`);
    }
    if (recordMatrix) {
      const results = {
        schema: "miso.web.qualification.matrix.v1",
        candidateCommit,
        wasmSha256: artifactDigest,
        blake3Qualification,
        playwrightVersion: PLAYWRIGHT_VERSION,
        platform: "linux-headless",
        artifact: "single shipped simd128 AudioWorklet artifact",
        stallRingFrames: 5120,
        minimumStallMs: 100,
        browsers: rows,
      };
      await writeFile(RESULTS_PATH, `${JSON.stringify(results, null, 2)}\n`);
      await writeFile(MATRIX_PATH, renderMatrix(results));
      process.stdout.write(`recorded ${path.relative(process.cwd(), RESULTS_PATH)} and ${path.relative(process.cwd(), MATRIX_PATH)}\n`);
    }
  } finally {
    await server.close();
    await isolatedServer?.close();
    if (sdkBundle !== undefined) await rm(sdkBundle, { recursive: true, force: true });
  }
}

await main();
