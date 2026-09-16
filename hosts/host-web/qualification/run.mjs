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
  // Issue #137 E8/E6: the live-console row and the console load carried across the stall.
  "control-path-applied", "control-path-meter", "control-path-command", "stall-console-load",
  // Issue #143 E12: the observation row. `observation-armed` is the eval's named red mutation --
  // a run whose armed tap published nothing, which is exactly what a browser that lost the
  // transport would produce.
  "observation-armed", "observation-unsubscribe", "observation-identity", "observation-window",
  "sdk-observation-window-reversed", "sdk-observation-window-malformed",
  "sdk-response", "sdk-observation", "sdk-spectrum", "sdk-spectrum-hop-missing",
  "sdk-spectrum-hop-wrong",
];

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
  if (!useDistribution && sdkRoot !== path.dirname(sourceRoot)) {
    throw new Error("selected SDK package has no built distribution");
  }
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

function validateLineage(checked, artifactDigest) {
  gate("matrix", "candidate-lineage", CANONICAL_COMMIT.test(checked.candidateCommit),
    "checked candidateCommit is not canonical lowercase 40-hex");
  gate("matrix", "artifact-lineage", checked.wasmSha256 === artifactDigest,
    "checked wasmSha256 differs from the artifact under qualification");
}

function lineageMutationProofs(checked, artifactDigest) {
  assert.throws(
    () => validateLineage({ ...checked, candidateCommit: "0".repeat(39) }, artifactDigest),
    /matrix: candidate-lineage:/,
    "matrix: malformed candidate lineage escaped its gate",
  );
  assert.throws(
    () => validateLineage({ ...checked, wasmSha256: "0".repeat(64) }, artifactDigest),
    /matrix: artifact-lineage:/,
    "matrix: mismatched artifact lineage escaped its gate",
  );
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
  const live = result.console;
  gate(browserName, "control-path", live?.commandResult === 0 && live?.commandReason === 0
    && live?.commandAdmitted === 1 && live?.appliedAtSample === "0"
    && Array.isArray(live?.tracks) && live.tracks.length === 1,
  "the live-console command was not admitted");
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

  // Issue #143 E12: subscribe -> nonzero `trackGrDb` -> unsubscribe -> zero, with `firstSample`
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
  gate(browserName, "observation-unsubscribe", observation?.disarmed?.unsubscribeResult === 0
    && observation?.disarmed?.unsubscribeBindings === 0
    && observation?.disarmed?.maximumTrackGrDb === 0,
  "an unsubscribed tap kept publishing");
  gate(browserName, "observation-identity", observation?.identicalAudio === true,
    "arming a declared tap moved a rendered sample");

  const stall = result.stall;
  // #137 E6: the frozen stall requirements are unchanged, and they are now met with the control
  // path and the meter fold both live across the fault.
  gate(browserName, "main-thread-stall", stall?.consoleCommandResult === 0
    && stall?.consoleMeterLeaseResult === 0 && stall?.consoleMeterFrames >= 1,
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
  if (result.sdkResponse !== null) validateSdkResponse(browserName, result.sdkResponse);
  return "simd128 supported";
}

function validateSdkResponse(browserName, response) {
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
    ["miso.parametric-eq", "eq-simd1", "simd1", "parametricEq", true, false],
    ["miso.compressor", "comp", "dynamic", "unavailable", false, false],
    ["miso.parametric-eq", "eq-simd2", "simd2", "parametricEq", true, true],
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
    ["miso.parametric-eq", "eq-simd1", "simd1", "parametricEq", true, false],
    ["miso.compressor", "comp", "dynamic", "unavailable", false, false],
    ["miso.parametric-eq", "eq-simd2", "simd2", "parametricEq", true, true],
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
  const spectrum = response?.spectrum;
  const spectrumRows = Array.isArray(spectrum?.targets) ? spectrum.targets : [];
  const expectedSpectrumTargets = [
    "trackPostInputBuiltins:track",
    "trackPostMatrix:track",
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
  gate(browserName, "sdk-spectrum-continuous", continuous?.pendingBeforeRender === true
    && continuous?.sharedJob === true && continuous?.automaticDelivery === true
    && continuous?.windows >= 1 && continuous?.gap === true
    && continuous?.ownedArrays === true && continuous?.sharedAfterFirstClose === true
    && continuous?.staleReadRefused === true,
  "continuous spectrum did not prove warmup, shared ownership, capture loss, or close lifecycle");
  gate(browserName, "sdk-spectrum-continuous", continuous?.statuses?.includes("ready") === true
    && continuous?.statuses?.includes("gap") === true
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
  const collection = spectrum?.collection;
  const collectionFirstCaptured = canonicalU64(collection?.firstCapturedSample);
  const collectionFirstEnd = canonicalU64(collection?.firstEndSample);
  const collectionSecondCaptured = canonicalU64(collection?.secondCapturedSample);
  const collectionSecondEnd = canonicalU64(collection?.secondEndSample);
  const collectionCaptured = canonicalU64(collection?.capturedSample);
  const collectionEnd = canonicalU64(collection?.endSample);
  gate(browserName, "sdk-spectrum-collection", JSON.stringify(collection?.selections) === JSON.stringify([
      "trackPostMatrix:track-a", "trackPostMatrix:track-b", "trackPostMatrix:track-a",
    ])
    && JSON.stringify(collection?.masks) === JSON.stringify(["both", "left"])
    && collection?.toBOk === true && collection?.toAOks === true
    && collection?.toBTarget === "trackPostMatrix:track-b"
    && collection?.toBChannels === "left"
    && collection?.bCleared === true && collection?.aCleared === true
    && collection?.switchedWithoutRestart === true,
  "spectrum collection did not atomically switch the existing owner A-to-B-to-A with fresh results");
  gate(browserName, "sdk-spectrum-collection", collection?.firstTarget === "trackPostMatrix:track-a"
    && collection?.firstChannels === "both"
    && collectionFirstCaptured === 0n
    && collectionFirstEnd === 2_048n
    && collection?.secondTarget === "trackPostMatrix:track-b"
    && collection?.secondChannels === "left"
    && collectionSecondCaptured !== undefined && collectionSecondEnd !== undefined
    && collectionSecondEnd > collectionSecondCaptured
    && Math.abs(collection?.firstPeakHz - 750) < 0.01
    && Math.abs(collection?.secondPeakHz - 750) < 0.01
    && collection?.distinctSelectedSignal === true
    && collection?.resultTarget === "trackPostMatrix:track-a"
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
}

function mutate(result, mutation) {
  const copy = structuredClone(result);
  if (mutation === "attestation") copy.attestation.outcome = "miso.unsupported.v1";
  if (mutation === "boot") copy.boot.ready = false;
  if (mutation === "native-corpus-digest") copy.corpus.browserDigests[0] = "0".repeat(64);
  if (mutation === "main-thread-stall") copy.stall.measuredStallMs = 0;
  if (mutation === "control-path-applied") copy.console.exactRetargetedOutput = false;
  if (mutation === "control-path-meter") copy.console.masterPeak = 0;
  if (mutation === "control-path-command") copy.console.commandAdmitted = 0;
  if (mutation === "stall-console-load") copy.stall.consoleMeterFrames = 0;
  // Issue #143 E12's named red mutation: `observationArmed = 0`.
  if (mutation === "observation-armed") copy.observation.armed.maximumTrackGrDb = 0;
  if (mutation === "observation-unsubscribe") copy.observation.disarmed.maximumTrackGrDb = 1;
  if (mutation === "observation-identity") copy.observation.identicalAudio = false;
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
  return copy;
}

function mutationProofs(browserName, result) {
  const mutations = result.sdkResponse === null
    ? MUTATIONS.filter((mutation) => !mutation.startsWith("sdk-"))
    : MUTATIONS;
  for (const mutation of mutations) {
    assert.throws(
      () => validate(browserName, mutate(result, mutation)),
      (error) => error instanceof Error && error.message.startsWith(`${browserName}:`),
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
    if (sdkEnabled) {
      // Resume the real collection AudioContext with a trusted gesture across browsers.
      await Promise.race([execution, page.locator("#spectrum-collection-resume").click()]);
    }
    const result = await execution;
    gate(browserName, "browser-execution", result.qualificationError === undefined,
      `${JSON.stringify(result.qualificationError)}${diagnostics.length === 0 ? "" : `; ${diagnostics.join("; ")}`}`);
    const outcome = validate(browserName, result);
    if (proveMutations) mutationProofs(browserName, result);
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
  let sdkBundle;
  if (sdkRoot !== null) sdkBundle = await buildSdkBundle(path.resolve(sdkRoot));

  if (proveMutations) {
    const served = await artifactSetProofs(artifacts);
    process.stdout.write(`artifact set: the exact ${served}-file shipped set is pinned\n`);
  }

  const checked = checkMatrix
    ? JSON.parse(await readFile(RESULTS_PATH, "utf8"))
    : null;
  if (checked !== null) {
    validateLineage(checked, artifactDigest);
    lineageMutationProofs(checked, artifactDigest);
  }
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
