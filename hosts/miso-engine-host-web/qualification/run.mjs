import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { chromium, firefox, webkit } from "playwright";
import { renderMatrix } from "./generate-matrix.mjs";
import { startQualificationServer } from "./server.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const HOST_WEB = path.dirname(HERE);
const RESULTS_PATH = path.join(HERE, "results.json");
const MATRIX_PATH = path.join(HOST_WEB, "BROWSER_DEPLOYMENT_MATRIX.md");
const PLAYWRIGHT_VERSION = JSON.parse(
  await readFile(path.join(HERE, "node_modules", "playwright", "package.json"), "utf8"),
).version;
const ENGINES = { chromium, firefox, webkit };
const MUTATIONS = [
  "attestation", "boot", "native-corpus-digest", "main-thread-stall",
  // Issue #137 E8/E6: the live-console row and the console load carried across the stall.
  "control-path-applied", "control-path-meter", "control-path-command", "stall-console-load",
];

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

function gate(browserName, name, condition, detail) {
  if (!condition) throw new Error(`${browserName}: ${name}: ${detail}`);
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
  const artifacts = option("--artifacts");
  if (artifacts === null) {
    throw new Error("usage: npm run qualify -- --artifacts DIR [--browser NAME] [--check-matrix|--record-matrix] [--self-test-mutations]");
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

  const checked = checkMatrix
    ? JSON.parse(await readFile(RESULTS_PATH, "utf8"))
    : null;
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
