import { createHash } from "node:crypto";
import { execFile } from "node:child_process";
import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { chromium, firefox, webkit } from "playwright";
import { ARTIFACT_NAMES, exactArtifacts, startQualificationServer } from "./server.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const HOST_WEB = path.dirname(HERE);
const ROOT = path.resolve(HERE, "../../..");
const SESSION_PATH = path.join(HOST_WEB, "tests", "browser-v1", "session.json");
const WASM_ARTIFACT = "miso-engine-v1-audio-worklet.simd128.wasm";
const LAYOUT_ARTIFACT = "miso-engine-v1-abi-layout.json";
const WASM_PIN_PATH = path.join(HOST_WEB, "web", "miso-engine-v1-audio-worklet-artifact.sha256");
const SOURCE_LAYOUT_PATH = path.join(ROOT, "sdk", "assets", LAYOUT_ARTIFACT);
const RESULTS_PATH = path.join(HERE, "results.json");
const SOURCE_COMMIT = "f2355988c0e51f4d283dd028bb98ce17f5038113";
const EXPECTED_WASM_SHA256 =
  "18b9dbfa61ae1188fcb00f18317702e37feb37c4843ac2b885194a4c77322cab";
const EXPECTED_LAYOUT_SHA256 =
  "8cd125f03d7bd5e98d93c4756af3bfe3282d709bceed1b55772140d5b57cb25d";
const PLAYWRIGHT_PACKAGE_PATH = path.join(HERE, "node_modules", "playwright", "package.json");
const STAGE_TIMEOUT_MS = 15_000;
const BROWSER_TIMEOUT_MS = 120_000;
const TEARDOWN_TIMEOUT_MS = 15_000;
const ENGINES = { chromium, firefox, webkit };
const STATUS_FIELDS = [
  "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
  "quantumFrames", "nextAbsoluteSample", "renderedQuanta", "memoryBytes",
];
const FACTORY_FIELDS = [
  "context", "document", "options", "simd128ModuleUrl", "workletModuleUrl",
];
const BOOT_OPTION_FIELDS = [
  "sourceRingFrames", "maximumMemoryBytes", "consoleCommandQueueRecords", "consoleMeterBlocks",
  "consoleObservationTaps", "consoleMasterTrackPlusOne",
];
const execFileAsync = promisify(execFile);

function argument(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) return null;
  const value = process.argv[index + 1];
  if (value === undefined || value.startsWith("--")) throw new Error(`${name} requires a value`);
  return value;
}

function json(value) {
  return JSON.stringify(value, (_key, item) => (
    typeof item === "bigint" ? item.toString() : item
  ));
}

function errorRecord(error) {
  if (error === null || typeof error !== "object") return { message: String(error) };
  return {
    name: error.name ?? typeof error,
    message: error.message ?? String(error),
    code: error.code,
    stage: error.stage,
    deadlineMs: error.deadlineMs,
    stack: error.stack,
    value: error.value,
  };
}

function failure(message, code = "FIXTURE_FAILURE") {
  const error = new Error(message);
  error.code = code;
  return error;
}

function timeoutError(label, deadlineMs) {
  const error = failure(`${label} timed out after ${deadlineMs} ms`, "NODE_DEADLINE");
  error.stage = label;
  error.deadlineMs = deadlineMs;
  return error;
}

function withDeadline(label, operation, deadlineMs) {
  return new Promise((resolve, reject) => {
    let settled = false;
    const timer = setTimeout(() => {
      settled = true;
      reject(timeoutError(label, deadlineMs));
    }, deadlineMs);
    Promise.resolve()
      .then(operation)
      .then((value) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        resolve(value);
      }, (error) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        reject(error);
      });
  });
}

async function sha256(file) {
  return createHash("sha256").update(await readFile(file)).digest("hex");
}

async function artifactSnapshot(directory) {
  await exactArtifacts(directory);
  const entries = await Promise.all([...ARTIFACT_NAMES].sort().map(async (name) => [
    name,
    await sha256(path.join(directory, name)),
  ]));
  return Object.fromEntries(entries);
}

function equalRecord(left, right) {
  const names = new Set([...Object.keys(left), ...Object.keys(right)]);
  return [...names].every((name) => left[name] === right[name]);
}

async function sourceProvenance() {
  const [wasmPin, matrixText, layoutDigest, commitResult] = await Promise.all([
    readFile(WASM_PIN_PATH, "utf8").then((text) => text.trim()),
    readFile(RESULTS_PATH, "utf8").then((text) => JSON.parse(text)),
    sha256(SOURCE_LAYOUT_PATH),
    execFileAsync("git", ["rev-parse", `${SOURCE_COMMIT}^{commit}`], { cwd: ROOT }),
  ]);
  const resolvedCommit = commitResult.stdout.trim();
  if (wasmPin !== EXPECTED_WASM_SHA256) {
    throw failure("source-authoritative WASM pin differs from the frozen issue pin");
  }
  if (layoutDigest !== EXPECTED_LAYOUT_SHA256) {
    throw failure("source-authoritative ABI layout differs from the frozen issue pin");
  }
  if (matrixText.candidateCommit !== SOURCE_COMMIT || matrixText.wasmSha256 !== EXPECTED_WASM_SHA256) {
    throw failure("qualification results do not carry the frozen source/artifact lineage");
  }
  if (resolvedCommit !== SOURCE_COMMIT) {
    throw failure("frozen source commit did not resolve to itself");
  }
  return {
    sourceCommit: SOURCE_COMMIT,
    resolvedSourceCommit: resolvedCommit,
    wasmPinPath: path.relative(ROOT, WASM_PIN_PATH),
    wasmPin,
    sourceLayoutPath: path.relative(ROOT, SOURCE_LAYOUT_PATH),
    sourceLayoutSha256: layoutDigest,
    matrixCandidateCommit: matrixText.candidateCommit,
    matrixWasmSha256: matrixText.wasmSha256,
    identityAndCompanionPinsVerified: true,
    publicationProvenanceIndependentlyEstablished: false,
  };
}

async function preflight(artifacts, browserNames) {
  const [hashes, provenance, sessionText, playwrightPackage] = await Promise.all([
    artifactSnapshot(artifacts),
    sourceProvenance(),
    readFile(SESSION_PATH, "utf8"),
    readFile(PLAYWRIGHT_PACKAGE_PATH, "utf8").then((text) => JSON.parse(text)),
  ]);
  if (hashes[WASM_ARTIFACT] !== EXPECTED_WASM_SHA256) {
    throw failure("supplied artifact WASM differs from the frozen issue pin");
  }
  if (hashes[LAYOUT_ARTIFACT] !== EXPECTED_LAYOUT_SHA256) {
    throw failure("supplied artifact ABI layout differs from the frozen issue pin");
  }
  const session = JSON.parse(sessionText);
  if (session.sample_rate_hz !== 48_000 || session.quantum_frames !== 128
      || !Array.isArray(session.tracks) || session.tracks.length !== 1) {
    throw failure("frozen browser session is not the required 48 kHz, 128-frame, one-track document");
  }
  const names = [...ARTIFACT_NAMES].sort();
  return {
    artifactDirectory: artifacts,
    artifactNames: names,
    artifactHashes: hashes,
    wasmSha256: hashes[WASM_ARTIFACT],
    abiLayoutSha256: hashes[LAYOUT_ARTIFACT],
    session: {
      path: path.relative(ROOT, SESSION_PATH),
      bytes: Buffer.byteLength(sessionText),
      sha256: createHash("sha256").update(sessionText).digest("hex"),
      sampleRateHz: session.sample_rate_hz,
      quantumFrames: session.quantum_frames,
      tracks: session.tracks.length,
    },
    source: provenance,
    nodeVersion: process.version,
    playwrightVersion: playwrightPackage.version,
    browserNames,
    launchOptions: Object.fromEntries(browserNames.map((name) => [
      name,
      name === "chromium" ? { headless: true, channel: "chromium" } : { headless: true },
    ])),
    autoplayOverride: false,
    fakeExports: false,
    offlineProcessing: false,
  };
}

async function rehashAfterBrowser(artifacts, before) {
  const hashes = await artifactSnapshot(artifacts);
  if (!equalRecord(hashes, before)) {
    throw failure("artifact drift detected after browser run", "ARTIFACT_DRIFT");
  }
  return hashes;
}

async function qualifyBrowser(browserName, engine, origin, artifacts, preflightRecord) {
  const launchOptions = browserName === "chromium"
    ? { headless: true, channel: "chromium" }
    : { headless: true };
  const row = {
    browser: browserName,
    launchOptions,
    outcome: "fail",
    diagnostics: [],
  };
  let browser;
  let page;
  try {
    browser = await withDeadline(`${browserName}: launch`, () => engine.launch(launchOptions), STAGE_TIMEOUT_MS);
    row.browserVersion = browser.version();
    page = await withDeadline(`${browserName}: newPage`, () => browser.newPage(), STAGE_TIMEOUT_MS);
    page.setDefaultTimeout(STAGE_TIMEOUT_MS);
    page.setDefaultNavigationTimeout(STAGE_TIMEOUT_MS);
    page.on("console", (message) => row.diagnostics.push(
      `console ${message.type()}: ${message.text()}`,
    ));
    page.on("pageerror", (error) => row.diagnostics.push(`pageerror: ${error.message}`));
    page.on("response", (response) => {
      if (!response.ok()) row.diagnostics.push(`HTTP ${response.status()}: ${response.url()}`);
    });
    await withDeadline(
      `${browserName}: navigate qualification fixture`,
      () => page.goto(`${origin}/qualification/index.html`, { waitUntil: "load" }),
      STAGE_TIMEOUT_MS,
    );
    const fixture = await withDeadline(
      `${browserName}: page.evaluate suspended lifecycle`,
      () => page.evaluate(runSuspendedFixture, {
        hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
        wasmModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
        workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
      }),
      BROWSER_TIMEOUT_MS,
    );
    row.fixture = fixture;
    if (fixture?.pass !== true) {
      throw failure(
        `${browserName}: suspended fixture failed: ${json(fixture?.qualificationError ?? fixture)}`,
        "FIXTURE_FAILURE",
      );
    }
    row.outcome = "pass";
  } catch (error) {
    row.error = errorRecord(error);
    if (row.diagnostics.length > 0) row.error.diagnostics = row.diagnostics;
  } finally {
    if (browser !== undefined) {
      try {
        row.browserCleanup = {
          outcome: "pass",
          acknowledgement: "browser.close resolved",
        };
        await withDeadline(
          `${browserName}: browser teardown`,
          () => browser.close(),
          TEARDOWN_TIMEOUT_MS,
        );
      } catch (error) {
        row.outcome = "fail";
        row.browserCleanup = {
          outcome: "unknown-after-loss",
          error: errorRecord(error),
        };
      }
    } else {
      row.browserCleanup = { outcome: "not-created" };
    }
    try {
      row.artifactAfterBrowser = await withDeadline(
        `${browserName}: artifact rehash`,
        () => rehashAfterBrowser(artifacts, preflightRecord.artifactHashes),
        STAGE_TIMEOUT_MS,
      );
    } catch (error) {
      row.outcome = "fail";
      row.artifactAfterBrowser = { outcome: "fail", error: errorRecord(error) };
    }
  }
  return row;
}

// This function is serialized into the page. It deliberately contains no host/runtime fallback:
// the imported host and every module it loads come from the exact artifact server.
async function runSuspendedFixture(urls) {
  const SAMPLE_RATE = 48_000;
  const QUANTUM_FRAMES = 128;
  const STATUS_FIELDS_IN_ORDER = [
    "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
    "quantumFrames", "nextAbsoluteSample", "renderedQuanta", "memoryBytes",
  ];
  const FACTORY_FIELDS_IN_ORDER = [
    "context", "document", "options", "simd128ModuleUrl", "workletModuleUrl",
  ];
  const BOOT_OPTION_FIELDS_IN_ORDER = [
    "sourceRingFrames", "maximumMemoryBytes", "consoleCommandQueueRecords", "consoleMeterBlocks",
    "consoleObservationTaps", "consoleMasterTrackPlusOne",
  ];
  const STAGE_TIMEOUT = 15_000;
  const stages = [];
  const cleanup = [];
  const boundaries = [];
  const statuses = [];
  const disposals = [];
  const redControls = {
    statusSuppression: null,
    renderedCounterMutations: {},
    resumeTrap: null,
  };
  let context = null;
  let activeHost = null;
  let resumeTarget = null;
  let resumeOwner = null;
  let resumeDescriptor = null;
  let resumeOwnDescriptor = null;
  let resumeResolution = null;
  let resumeTrapInstalled = false;
  let resumeAttempts = 0;
  let nativeResumeCalls = 0;
  let currentTimeBaseline = null;
  let initialContext = null;
  let finalContextBeforeTeardown = null;
  let hostFactory = null;

  const errorDetails = (error) => {
    if (error === null || typeof error !== "object") return { message: String(error) };
    return {
      name: error.name ?? typeof error,
      message: error.message ?? String(error),
      code: error.code,
      field: error.field,
      stack: error.stack,
    };
  };
  const fixtureFailure = (message, code = "FIXTURE_FAILURE") => {
    const error = new Error(message);
    error.code = code;
    return error;
  };
  const timeout = (label) => {
    const error = fixtureFailure(`${label} timed out after ${STAGE_TIMEOUT} ms`, "STAGE_TIMEOUT");
    error.stage = label;
    return error;
  };
  const exactFields = (value, fields) => {
    if (value === null || typeof value !== "object") return false;
    const actual = Object.keys(value).sort();
    const expected = [...fields].sort();
    return actual.length === expected.length && actual.every((key, index) => key === expected[index]);
  };
  const gate = (field, condition, detail) => {
    if (!condition) {
      const error = fixtureFailure(`${field}: ${detail}`, "STATUS_ASSERTION");
      error.field = field;
      throw error;
    }
  };
  const clock = () => ({ state: context.state, currentTime: context.currentTime });
  const assertSuspendedClock = (label) => {
    gate(`${label}.context.state`, context.state === "suspended", `was ${context.state}`);
    gate(
      `${label}.currentTime`,
      Object.is(context.currentTime, currentTimeBaseline),
      `changed from ${currentTimeBaseline} to ${context.currentTime}`,
    );
    gate(`${label}.resumeAttempts`, resumeAttempts === 0, `was ${resumeAttempts}`);
  };
  const boundary = (label) => {
    assertSuspendedClock(label);
    boundaries.push({ ...clock(), label, resumeAttempts });
  };
  const plainStatus = (status) => ({
    ...status,
    nextAbsoluteSample: status.nextAbsoluteSample.toString(),
    renderedQuanta: status.renderedQuanta.toString(),
  });
  const assertStatus = (status, host, label) => {
    gate(`${label}.shape`, exactFields(status, STATUS_FIELDS_IN_ORDER), "unexpected status fields");
    gate(`${label}.tag`, status.tag === "miso.status.v1", "unexpected status tag");
    gate(`${label}.requestId`, Number.isSafeInteger(status.requestId) && status.requestId > 0, "invalid request ID");
    gate(`${label}.result`, status.result === 0, `result was ${status.result}`);
    gate(`${label}.state`, status.state === 2, `state was ${status.state}`);
    gate(`${label}.lastResult`, status.lastResult === 0, `lastResult was ${status.lastResult}`);
    gate(`${label}.backend`, host.backend === "simd128" && status.backend === 1, "backend mismatch");
    gate(`${label}.sampleRateHz`, status.sampleRateHz === SAMPLE_RATE, "sample rate mismatch");
    gate(`${label}.quantumFrames`, status.quantumFrames === QUANTUM_FRAMES, "quantum mismatch");
    gate(`${label}.nextAbsoluteSample`, typeof status.nextAbsoluteSample === "bigint"
      && status.nextAbsoluteSample === 0n, "nextAbsoluteSample was not zero");
    gate(`${label}.renderedQuanta`, typeof status.renderedQuanta === "bigint"
      && status.renderedQuanta === 0n, "renderedQuanta was not zero");
    gate(`${label}.memoryBytes`, Number.isSafeInteger(status.memoryBytes) && status.memoryBytes > 0, "invalid memory size");
    assertSuspendedClock(label);
  };
  const runStage = async (label, operation, expectedError = null) => {
    const started = performance.now();
    let timer;
    try {
      const operationPromise = Promise.resolve().then(operation);
      const deadlinePromise = new Promise((_, reject) => {
        timer = setTimeout(() => reject(timeout(label)), STAGE_TIMEOUT);
      });
      const value = await Promise.race([operationPromise, deadlinePromise]);
      if (expectedError !== null) throw fixtureFailure(`${label} resolved instead of refusing`, "EXPECTED_REFUSAL_MISSING");
      stages.push({ label, outcome: "pass", elapsedMs: performance.now() - started });
      return value;
    } catch (error) {
      const expected = expectedError !== null && expectedError(error);
      stages.push({
        label,
        outcome: expected ? "expected-refusal" : (error.code === "STAGE_TIMEOUT" ? "timeout" : "fail"),
        elapsedMs: performance.now() - started,
        error: errorDetails(error),
      });
      if (expected) return { expected: true, error: errorDetails(error) };
      throw error;
    } finally {
      clearTimeout(timer);
    }
  };
  const runTeardown = async (label, operation) => {
    const started = performance.now();
    let timer;
    try {
      const operationPromise = Promise.resolve().then(operation);
      const deadlinePromise = new Promise((_, reject) => {
        timer = setTimeout(() => reject(timeout(label)), STAGE_TIMEOUT);
      });
      const value = await Promise.race([operationPromise, deadlinePromise]);
      cleanup.push({ label, outcome: "pass", elapsedMs: performance.now() - started });
      return { ok: true, value };
    } catch (error) {
      cleanup.push({
        label,
        outcome: "unknown-after-loss",
        elapsedMs: performance.now() - started,
        error: errorDetails(error),
      });
      return { ok: false, error };
    } finally {
      clearTimeout(timer);
    }
  };
  const bootOptions = () => ({
    sourceRingFrames: 0,
    maximumMemoryBytes: 67_108_864n,
    consoleCommandQueueRecords: 0n,
    consoleMeterBlocks: 0n,
    consoleObservationTaps: 0n,
    consoleMasterTrackPlusOne: 0n,
  });
  const makeFactoryOptions = () => ({
    context,
    document: new Uint8Array(sessionDocument),
    options: bootOptions(),
    simd128ModuleUrl: urls.wasmModuleUrl,
    workletModuleUrl: urls.workletModuleUrl,
  });
  const boot = async (label) => {
    const options = makeFactoryOptions();
    gate(`${label}.factory`, exactFields(options, FACTORY_FIELDS_IN_ORDER), "factory fields drifted");
    gate(`${label}.bootOptions`, exactFields(options.options, BOOT_OPTION_FIELDS_IN_ORDER), "boot options fields drifted");
    const host = await runStage(label, () => hostFactory(options));
    gate(`${label}.backend`, host?.backend === "simd128", "host backend was not simd128");
    gate(`${label}.node`, host?.node instanceof AudioWorkletNode, "host did not return a real AudioWorkletNode");
    activeHost = host;
    return host;
  };
  const nativeDispose = async (label, host, receiver) => {
    gate(`${label}.receiver`, receiver === undefined || host.node.port.onmessage === receiver,
      "host message receiver was changed before disposal");
    const acknowledgement = await runStage(label, () => host.dispose());
    gate(`${label}.acknowledgement`, acknowledgement === undefined, "dispose did not resolve undefined");
    disposals.push({ label, outcome: "native-acknowledged", acknowledgement: "undefined", receiverVerified: true });
    if (activeHost === host) activeHost = null;
  };
  const idempotentDispose = async (label, host) => {
    const acknowledgement = await runStage(label, () => host.dispose());
    gate(`${label}.acknowledgement`, acknowledgement === undefined, "second dispose did not resolve undefined");
    disposals.push({ label, outcome: "idempotent-local", acknowledgement: "undefined" });
  };
  const mutateRenderedCounters = (captured, host) => {
    for (const field of ["nextAbsoluteSample", "renderedQuanta"]) {
      const mutated = { ...captured, [field]: captured[field] + 1n };
      let refused = false;
      try {
        assertStatus(mutated, host, `red.${field}`);
      } catch (error) {
        refused = error.code === "STATUS_ASSERTION" && error.field === `red.${field}.${field}`;
      }
      gate(`red.${field}`, refused, "rendered-counter mutation was accepted");
      redControls.renderedCounterMutations[field] = { outcome: "refused", mutatedValue: mutated[field].toString() };
    }
  };
  const suppressOneStatus = async (host) => {
    const port = host.node.port;
    const receiver = port.onmessage;
    gate("red.statusSuppression.receiver", typeof receiver === "function", "host receiver was not callable");
    let suppressed = 0;
    let requestId = null;
    port.onmessage = (event) => {
      const message = event.data;
      if (suppressed === 0 && message?.tag === "miso.status.v1"
          && Number.isSafeInteger(message.requestId) && message.requestId > 0) {
        suppressed += 1;
        requestId = message.requestId;
        return;
      }
      receiver(event);
    };
    let timeoutOutcome;
    try {
      const statusPromise = host.status();
      statusPromise.catch(() => undefined);
      timeoutOutcome = await runStage(
        "red.suppressedStatus",
        () => statusPromise,
        (error) => error.code === "STAGE_TIMEOUT",
      );
    } finally {
      port.onmessage = receiver;
    }
    gate("red.statusSuppression.reply", suppressed === 1 && requestId !== null,
      "no actual matching status reply was suppressed");
    gate("red.statusSuppression.timeout", timeoutOutcome?.expected === true,
      "suppressed status did not fail specifically as a timeout");
    gate("red.statusSuppression.receiverRestored", port.onmessage === receiver,
      "status receiver was not restored before cleanup");
    redControls.statusSuppression = {
      outcome: "expected-timeout",
      requestId,
      suppressedReplies: suppressed,
      deadlineMs: STAGE_TIMEOUT,
      receiverRestored: true,
    };
  };
  const resumeRedControl = async () => {
    const before = clock();
    const attemptsBefore = resumeAttempts;
    const nativeBefore = nativeResumeCalls;
    const refusal = await runStage(
      "red.resumeTrap",
      () => context.resume(),
      (error) => error.code === "RESUME_TRAP",
    );
    gate("red.resumeTrap.refusal", refusal?.expected === true, "resume did not refuse through the trap");
    gate("red.resumeTrap.attempts", resumeAttempts === attemptsBefore + 1, "resume trap count was not incremented once");
    gate("red.resumeTrap.native", nativeResumeCalls === nativeBefore, "native resume was invoked");
    gate("red.resumeTrap.state", context.state === before.state, "resume changed context state");
    gate("red.resumeTrap.currentTime", Object.is(context.currentTime, before.currentTime), "resume changed currentTime");
    redControls.resumeTrap = {
      outcome: "expected-refusal",
      attemptsBefore,
      attemptsAfter: resumeAttempts,
      nativeResumeCalls,
      stateBefore: before.state,
      stateAfter: context.state,
      currentTimeBefore: before.currentTime,
      currentTimeAfter: context.currentTime,
      unchanged: true,
    };
  };

  let sessionDocument;
  const report = {
    schema: "miso.web.suspended-host.result.v1",
    pass: false,
    factory: {
      sourceRingFrames: 0,
      maximumMemoryBytes: "67108864",
      consoleCommandQueueRecords: "0",
      consoleMeterBlocks: "0",
      consoleObservationTaps: "0",
      consoleMasterTrackPlusOne: "0",
      preparedModule: false,
      spectrumPreparation: false,
    },
  };
  try {
    resumeTarget = AudioContext.prototype;
    resumeOwnDescriptor = Object.getOwnPropertyDescriptor(resumeTarget, "resume");
    let candidatePrototype = resumeTarget;
    let ownerDepth = 0;
    while (candidatePrototype !== null) {
      const candidateDescriptor = Object.getOwnPropertyDescriptor(candidatePrototype, "resume");
      if (candidateDescriptor !== undefined) {
        if (typeof candidateDescriptor.value !== "function") {
          throw fixtureFailure(
            `resolved resume descriptor at prototype depth ${ownerDepth} is not callable`,
            "RESUME_TRAP_INSTALL",
          );
        }
        resumeOwner = candidatePrototype;
        resumeDescriptor = candidateDescriptor;
        break;
      }
      candidatePrototype = Object.getPrototypeOf(candidatePrototype);
      ownerDepth += 1;
    }
    if (resumeDescriptor === null) {
      throw fixtureFailure("AudioContext prototype chain has no callable resume method", "RESUME_TRAP_INSTALL");
    }
    const ownerName = resumeOwner === resumeTarget
      ? "AudioContext.prototype"
      : `${resumeOwner?.constructor?.name ?? "unknown"}.prototype`;
    resumeResolution = {
      target: "AudioContext.prototype",
      owner: ownerName,
      ownerDepth,
      callable: typeof resumeDescriptor.value === "function",
      originalOwnDescriptor: resumeOwnDescriptor === undefined ? null : {
        configurable: resumeOwnDescriptor.configurable,
        enumerable: resumeOwnDescriptor.enumerable,
        writable: resumeOwnDescriptor.writable,
        callable: typeof resumeOwnDescriptor.value === "function",
      },
    };
    const resumeTrap = function suspendedQualificationResumeTrap() {
      resumeAttempts += 1;
      return Promise.reject(fixtureFailure("resume refused by qualification trap", "RESUME_TRAP"));
    };
    Object.defineProperty(resumeTarget, "resume", { ...resumeDescriptor, value: resumeTrap });
    resumeTrapInstalled = true;

    context = await runStage("create.realAudioContext", () => new AudioContext({ sampleRate: SAMPLE_RATE }));
    initialContext = clock();
    gate(
      "initialContext.state",
      initialContext.state === "suspended" || initialContext.state === "running",
      `unexpected initial state ${initialContext.state}`,
    );
    await runStage("suspend.initialContext", () => context.suspend());
    gate("postSuspension.state", context.state === "suspended", `was ${context.state}`);
    currentTimeBaseline = context.currentTime;
    report.initialContext = initialContext;
    report.postSuspension = { ...clock(), currentTimeBaseline, resumeAttempts };

    const sessionResponse = await runStage("fetch.frozenSession", () => fetch("/fixture/session.json"));
    gate("fetch.frozenSession.response", sessionResponse.ok, `HTTP ${sessionResponse.status}`);
    const sessionText = await runStage("read.frozenSession", () => sessionResponse.text());
    const session = JSON.parse(sessionText);
    gate("frozenSession.sampleRate", session.sample_rate_hz === SAMPLE_RATE, "session sample rate mismatch");
    gate("frozenSession.quantum", session.quantum_frames === QUANTUM_FRAMES, "session quantum mismatch");
    gate("frozenSession.tracks", Array.isArray(session.tracks) && session.tracks.length === 1, "session track count mismatch");
    sessionDocument = new TextEncoder().encode(sessionText);

    const hostModule = await runStage("import.shippedHost", () => import(urls.hostModuleUrl));
    hostFactory = hostModule.createMisoAudioWorkletHost;
    gate("import.shippedHost.export", typeof hostFactory === "function", "host factory export missing");

    boundary("before.firstBoot");
    const firstHost = await boot("first.boot");
    boundary("after.firstBoot");
    boundary("before.firstStatus");
    const firstStatus = await runStage("first.status", () => firstHost.status());
    assertStatus(firstStatus, firstHost, "first.status");
    statuses.push({ label: "first.status", status: plainStatus(firstStatus) });
    boundary("after.firstStatus");

    mutateRenderedCounters(firstStatus, firstHost);
    await suppressOneStatus(firstHost);
    boundary("after.redControls.beforeFirstDispose");

    const firstReceiver = firstHost.node.port.onmessage;
    await nativeDispose("first.dispose", firstHost, firstReceiver);
    boundary("after.firstDispose");
    await idempotentDispose("first.disposeAgain", firstHost);
    boundary("after.firstDisposeAgain");

    boundary("before.secondBoot");
    const secondHost = await boot("second.boot");
    boundary("after.secondBoot");
    boundary("before.secondStatus");
    const secondStatus = await runStage("second.status", () => secondHost.status());
    assertStatus(secondStatus, secondHost, "second.status");
    statuses.push({ label: "second.status", status: plainStatus(secondStatus) });
    boundary("after.secondStatus");
    const secondReceiver = secondHost.node.port.onmessage;
    await nativeDispose("second.dispose", secondHost, secondReceiver);
    boundary("after.secondDispose");

    await resumeRedControl();
    report.pass = true;
    report.lifecycle = "boot -> status -> dispose -> dispose -> boot -> status -> dispose";
  } catch (error) {
    report.qualificationError = errorDetails(error);
  } finally {
    finalContextBeforeTeardown = context === null ? null : clock();
    if (activeHost !== null) {
      const host = activeHost;
      const result = await runTeardown("cleanup.activeHost.dispose", () => host.dispose());
      cleanup.push({
        label: "cleanup.activeHost.nativeAcknowledgement",
        outcome: result.ok && result.value === undefined ? "native-acknowledged" : "unknown-after-loss",
      });
      activeHost = null;
    } else {
      cleanup.push({ label: "cleanup.activeHost.dispose", outcome: "not-needed" });
    }
    if (context !== null) await runTeardown("cleanup.context.close", () => context.close());
    else cleanup.push({ label: "cleanup.context.close", outcome: "not-created" });
    if (resumeTrapInstalled) {
      try {
        if (resumeOwnDescriptor === undefined) {
          if (!delete resumeTarget.resume) {
            throw fixtureFailure("temporary own resume trap could not be deleted", "RESUME_TRAP_RESTORE");
          }
        } else {
          Object.defineProperty(resumeTarget, "resume", resumeOwnDescriptor);
        }
        cleanup.push({
          label: "cleanup.resumeTrap.restore",
          outcome: "pass",
          action: resumeOwnDescriptor === undefined ? "deleted-own-trap" : "restored-original-own-descriptor",
          owner: resumeResolution?.owner,
          callable: resumeResolution?.callable,
        });
      } catch (error) {
        cleanup.push({ label: "cleanup.resumeTrap.restore", outcome: "unknown-after-loss", error: errorDetails(error) });
      }
      resumeTrapInstalled = false;
    } else {
      cleanup.push({ label: "cleanup.resumeTrap.restore", outcome: "not-installed" });
    }
  }

  report.initialContext = initialContext;
  report.postSuspension = report.postSuspension ?? null;
  report.resumeResolution = resumeResolution;
  report.finalContextBeforeTeardown = finalContextBeforeTeardown;
  report.currentTimeBaseline = currentTimeBaseline;
  report.resumeAttempts = resumeAttempts;
  report.nativeResumeCalls = nativeResumeCalls;
  report.statuses = statuses;
  report.disposals = disposals;
  report.boundaries = boundaries;
  report.redControls = redControls;
  report.stages = stages;
  report.cleanup = cleanup;
  if (cleanup.some((entry) => entry.outcome === "unknown-after-loss")) {
    report.pass = false;
    report.qualificationError ??= errorDetails(fixtureFailure("cleanup did not complete with a proven acknowledgement", "CLEANUP_FAILURE"));
  }
  return report;
}

async function main() {
  const artifactsArgument = argument("--artifacts");
  const browserArgument = argument("--browser") ?? "all";
  const logDirectory = await mkdtemp(path.join(os.tmpdir(), "miso-suspended-host-"));
  const rawEvents = [];
  const record = (event) => rawEvents.push({ at: new Date().toISOString(), ...event });
  let server;
  let preflightRecord;
  let passed = false;
  let fatalError;
  try {
    if (artifactsArgument === null) {
      throw new Error("usage: node suspended-host.mjs --artifacts DIR [--browser all|chromium|firefox|webkit]");
    }
    const artifacts = path.resolve(artifactsArgument);
    const browserNames = browserArgument === "all" ? Object.keys(ENGINES) : [browserArgument];
    if (browserNames.some((name) => !(name in ENGINES))) throw new Error(`unknown browser: ${browserArgument}`);
    preflightRecord = await withDeadline(
      "preflight",
      () => preflight(artifacts, browserNames),
      STAGE_TIMEOUT_MS,
    );
    record({ phase: "preflight", ...preflightRecord });
    process.stdout.write(`preflight ${json({ ...preflightRecord, logDirectory })}\n`);

    server = await withDeadline(
      "qualification server start",
      () => startQualificationServer({ artifacts }),
      STAGE_TIMEOUT_MS,
    );
    record({ phase: "server", origin: server.origin });
    const rows = [];
    for (const browserName of browserNames) {
      const row = await qualifyBrowser(browserName, ENGINES[browserName], server.origin, artifacts, preflightRecord);
      rows.push(row);
      record({ phase: "browser", ...row });
      process.stdout.write(`${browserName}: ${row.outcome}${row.browserVersion === undefined ? "" : ` (${row.browserVersion})`}\n`);
      if (row.outcome !== "pass") process.stdout.write(`${browserName} failure: ${json(row.error ?? row.fixture?.qualificationError)}\n`);
    }
    const allBrowsers = browserArgument === "all" && rows.length === Object.keys(ENGINES).length;
    passed = allBrowsers && rows.every((row) => row.outcome === "pass")
      && rows.every((row) => row.fixture?.pass === true);
    if (!allBrowsers) record({ phase: "closure", outcome: "fail", detail: "all browsers are required for closure" });
    record({ phase: "result", outcome: passed ? "PASS" : "FAIL", browsers: rows });
    if (passed) process.stdout.write(`PASS suspended ordinary lifecycle; raw log: ${logDirectory}/raw.json\n`);
    else process.stdout.write(`FAIL suspended ordinary lifecycle; raw log: ${logDirectory}/raw.json\n`);
  } catch (error) {
    fatalError = error;
    record({ phase: "fatal", error: errorRecord(error) });
    process.stderr.write(`FAIL ${json(errorRecord(error))}\n`);
  } finally {
    if (server !== undefined) {
      try {
        await withDeadline("qualification server teardown", () => server.close(), TEARDOWN_TIMEOUT_MS);
        record({ phase: "serverCleanup", outcome: "pass" });
      } catch (error) {
        passed = false;
        record({ phase: "serverCleanup", outcome: "unknown-after-loss", error: errorRecord(error) });
        process.stderr.write(`server cleanup failure: ${json(errorRecord(error))}\n`);
      }
    } else {
      record({ phase: "serverCleanup", outcome: "not-created" });
    }
    try {
      await writeFile(
        path.join(logDirectory, "raw.json"),
        `${JSON.stringify({ schema: "miso.web.suspended-host.log.v1", events: rawEvents }, (_key, value) => (
          typeof value === "bigint" ? value.toString() : value
        ), 2)}\n`,
      );
    } catch (error) {
      fatalError ??= error;
      passed = false;
      process.stderr.write(`raw log write failure: ${json(errorRecord(error))}\n`);
    }
  }
  if (fatalError !== undefined || !passed) process.exitCode = 1;
}

await main();
