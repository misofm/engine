import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { cp, mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
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
];

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

function gate(browserName, name, condition, detail) {
  if (!condition) throw new Error(`${browserName}: ${name}: ${detail}`);
}

async function sha256(file) {
  return createHash("sha256").update(await readFile(file)).digest("hex");
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
  return "simd128 supported";
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
  return copy;
}

function mutationProofs(browserName, result) {
  for (const mutation of MUTATIONS) {
    assert.throws(
      () => validate(browserName, mutate(result, mutation)),
      (error) => error instanceof Error && error.message.startsWith(`${browserName}:`),
      `${browserName}: ${mutation}: red mutation escaped its gate`,
    );
  }
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
      && error.message === "artifact directory must contain the exact shipped six-file set",
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
    // Substitution keeps the *count* at six, so only the name test can catch it. Without this row
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

async function qualifyBrowser(browserName, engine, origin, proveMutations) {
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
    const result = await page.evaluate(async () => {
      try {
        const module = await import("/qualification/qualification.js");
        return await module.runQualification();
      } catch (error) {
        return {
          qualificationError: {
            message: error?.message ?? String(error),
            name: error?.name ?? typeof error,
            value: error !== null && typeof error === "object" ? { ...error } : error,
          },
        };
      }
    });
    gate(browserName, "browser-execution", result.qualificationError === undefined,
      `${JSON.stringify(result.qualificationError)}${diagnostics.length === 0 ? "" : `; ${diagnostics.join("; ")}`}`);
    const outcome = validate(browserName, result);
    if (proveMutations) mutationProofs(browserName, result);
    process.stdout.write(`${browserName}: all qualification gates passed (${browser.version()})\n`);
    return normalizedRow(browserName, browser.version(), outcome);
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
  const server = await startQualificationServer({ artifacts });
  try {
    const rows = [];
    for (const browserName of browserNames) {
      const row = await qualifyBrowser(browserName, ENGINES[browserName], server.origin, proveMutations);
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
  }
}

await main();
