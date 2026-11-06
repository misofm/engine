// Issue #2243: the on-demand browser soak. A 10-minute deadline run at each launch rate and a
// 60-minute streaming soak of the 64-track console session, on Playwright Chromium and WebKit, in
// both instance modes. Run before a minor release, never per PR (docs/RELEASE.md, "The soak").
//
//   npm run soak -- --artifacts DIR [--sdk-root DIR] [--browser all|chromium|webkit]
//     [--cells deadline,soak] [--rates 44100,48000,88200,96000] [--modes worker,single]
//     [--minutes-deadline 10] [--minutes-soak 60] [--soak-rate 96000] [--warmup-seconds 60]
//     [--parallel-per-browser 2] [--out FILE.jsonl] [--expect-red]
//
// A real AudioContext needs an output device: run it under a private PulseAudio null sink, as the
// qualification job does. It exits 1 when a gated check is red (or, with `--expect-red`, when the
// mutation is not caught).
import { readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { chromium, webkit } from "playwright";
import { buildSdkBundle } from "./sdk-bundle.mjs";
import { startQualificationServer } from "./server.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const SESSION = path.resolve(HERE, "../../../fixtures/session/v1/console-sixty-four-track-intended.json");
const ENGINES = { chromium, webkit };
const RATES = [44_100, 48_000, 88_200, 96_000];
const QUANTUM = 128;
/** Rendered time may trail wall time by at most this much over a cell, after warm-up. */
const CLOCK_PACE_SECONDS = 0.25;
const SAMPLE_INTERVAL_SECONDS = 10;
/** Margin past the play time for a cell's boot, close and page round trips. */
const CELL_MARGIN_SECONDS = 120;
/**
 * The gates and the browsers that gate each. WebKit's headless Linux sink renders in bursts (#1387)
 * and stalls under about 1 ms of work per call (#2245), so its underruns, misses and clock pace are
 * reported, not gated; its render allocations and memory are gated as on Chromium.
 */
const GATES = Object.freeze({
  "render-allocations": new Set(["chromium", "webkit"]),
  "underrun-frames": new Set(["chromium"]),
  "deadline-misses": new Set(["chromium"]),
  "clock-pace": new Set(["chromium"]),
  "memory-flat": new Set(["chromium", "webkit"]),
});

function option(name, fallback = null) {
  const index = process.argv.indexOf(name);
  if (index === -1) return fallback;
  const value = process.argv[index + 1];
  if (value === undefined || value.startsWith("--")) throw new Error(`${name} needs a value`);
  return value;
}

function positive(name, fallback) {
  const value = Number(option(name, String(fallback)));
  if (!Number.isFinite(value) || value <= 0) throw new Error(`${name} must be a positive number`);
  return value;
}

function list(name, fallback, allowed) {
  const values = option(name, fallback).split(",");
  for (const value of values) {
    if (!allowed.includes(value)) throw new Error(`${name}: unknown value ${value}`);
  }
  return values;
}

/** The session at `rate`, its one source stretched to the cell's play time plus a margin. */
function cellDocument(session, rate, seconds) {
  const document = structuredClone(session);
  document.sample_rate_hz = rate;
  const frames = BigInt(rate) * BigInt(Math.ceil(seconds) + CELL_MARGIN_SECONDS);
  document.sources[0].frames = frames.toString();
  return { document: JSON.stringify(document), frames: frames.toString() };
}

/** Each gate's measured value and verdict for one cell's page result. */
function judge(browserName, rate, result) {
  const samples = result.samples;
  const after = samples.filter((sample) => sample.afterWarmup);
  const first = after[0];
  const last = samples[samples.length - 1];
  const measured = {};
  const pass = {};
  measured["render-allocations"] = { result: result.renderAllocations.result, count: result.renderAllocations.count };
  pass["render-allocations"] = result.renderAllocations.result === 0 && result.renderAllocations.count === 0;
  if (first === undefined) {
    for (const gate of ["underrun-frames", "deadline-misses", "clock-pace", "memory-flat"]) {
      measured[gate] = "no sample after warm-up";
      pass[gate] = false;
    }
    return { measured, pass };
  }
  const underrunFrames = BigInt(last.underrunFrames) - BigInt(first.underrunFrames);
  measured["underrun-frames"] = {
    afterWarmup: underrunFrames.toString(), total: last.underrunFrames,
    ringUnderrunsAfterWarmup: last.ringUnderruns - first.ringUnderruns,
  };
  pass["underrun-frames"] = underrunFrames === 0n;
  const telemetry = result.telemetry;
  measured["deadline-misses"] = {
    afterWarmup: telemetry.missesAfterWarmup, total: telemetry.misses,
    windowsAfterWarmup: telemetry.windowsAfterWarmup, blocksAfterWarmup: telemetry.blocksAfterWarmup,
    peakBlockMsAfterWarmup: telemetry.peakBlockMsAfterWarmup, peakBlockMs: telemetry.peakBlockMs,
    budgetMs: telemetry.budgetMs, resolutionMs: telemetry.resolutionMs,
    meanCpuPercentAfterWarmup: telemetry.windowsAfterWarmup === 0 ? null
      : Number((telemetry.cpuPercentSum / telemetry.windowsAfterWarmup).toFixed(2)),
    belowResolutionWindows: telemetry.belowResolution,
  };
  // A lease that delivered no window after warm-up proves nothing.
  pass["deadline-misses"] = telemetry.missesAfterWarmup === 0 && telemetry.windowsAfterWarmup > 0;
  const renderedSeconds = Number(BigInt(last.renderedQuanta) - BigInt(first.renderedQuanta)) * QUANTUM / rate;
  const wallSeconds = last.wallSeconds - first.wallSeconds;
  const contextSeconds = last.contextSeconds - first.contextSeconds;
  const failedResults = samples.filter((sample) => sample.lastResult !== 0).length;
  const rebuilds = last.blockingRebuilds - samples[0].blockingRebuilds;
  const notRunning = samples.filter((sample) => sample.contextState !== "running").length;
  measured["clock-pace"] = {
    renderedSeconds: Number(renderedSeconds.toFixed(4)), wallSeconds: Number(wallSeconds.toFixed(4)),
    contextSeconds: Number(contextSeconds.toFixed(4)), samplesWithLastResult: failedResults,
    blockingRebuilds: rebuilds, samplesNotRunning: notRunning,
  };
  pass["clock-pace"] = renderedSeconds >= wallSeconds - CLOCK_PACE_SECONDS
    && failedResults === 0 && rebuilds === 0 && notRunning === 0;
  const peakMemory = Math.max(...after.map((sample) => sample.memoryBytes));
  measured["memory-flat"] = {
    first: samples[0].memoryBytes, firstAfterWarmup: first.memoryBytes, maxAfterWarmup: peakMemory,
    last: last.memoryBytes, growthAfterWarmup: peakMemory - first.memoryBytes,
  };
  pass["memory-flat"] = peakMemory - first.memoryBytes === 0;
  measured.singleModeControlAllocations = last.singleModeControlAllocations;
  return { measured, pass };
}

/** The `run.mjs` launch options of `browserName`. */
function launchOptions(browserName) {
  return browserName === "chromium"
    ? { headless: true, channel: "chromium", args: ["--autoplay-policy=no-user-gesture-required", "--disable-dev-shm-usage"] }
    : { headless: true };
}

/**
 * Run one cell in its own browser and return its record row, so no two cells share a browser
 * process. Cells that run at once still share the host and its sink: the record carries the host
 * load and the concurrency beside the numbers.
 */
async function runCell(browserName, origin, session, cell, warmupSeconds) {
  const seconds = cell.minutes * 60;
  const label = `${cell.kind}-${cell.mode}-${cell.rate}`;
  const started = Date.now();
  const row = { browser: browserName, kind: cell.kind, mode: cell.mode, rate: cell.rate, minutes: cell.minutes, warmupSeconds };
  let browser;
  try {
    browser = await ENGINES[browserName].launch(launchOptions(browserName));
    row.browserVersion = browser.version();
    const page = await browser.newPage();
    const errors = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    await page.goto(`${origin}/qualification/index.html`);
    const { document, frames } = cellDocument(session.document, cell.rate, seconds);
    const running = page.evaluate(async (request) => {
      const { runSoak } = await import("/sdk/soak-client.js");
      return runSoak(request);
    }, {
      instanceMode: cell.mode, sampleRateHz: cell.rate, seconds, warmupSeconds,
      sampleIntervalSeconds: SAMPLE_INTERVAL_SECONDS, document, sourceId: session.sourceId,
      sourceFrames: frames, tracks: session.tracks, resumeLabel: label,
    });
    // A context that cannot be built never shows the button: the page's result settles first.
    const clicked = Promise.race([
      page.click(`[data-soak-resume="${label}"]`, { timeout: 60_000 }),
      running.then(() => undefined, () => undefined),
    ]);
    let deadline;
    const result = await Promise.race([
      Promise.all([running, clicked]).then(([value]) => value),
      new Promise((_, reject) => {
        deadline = setTimeout(() => reject(new Error(`cell exceeded ${seconds + CELL_MARGIN_SECONDS} s`)),
          (seconds + CELL_MARGIN_SECONDS) * 1_000);
      }),
    ]).finally(() => clearTimeout(deadline));
    if (errors.length > 0) throw new Error(errors.join("; "));
    if (result.contextUnavailable !== undefined) {
      // WebKit may not build a context at every launch rate on this host (root ruling, #2243):
      // recorded, not red. On Chromium it is a failure.
      row.outcome = browserName === "webkit" ? "context-unavailable" : "red";
      row.error = result.contextUnavailable;
      return row;
    }
    const { measured, pass } = judge(browserName, cell.rate, result);
    row.instanceMode = result.instanceMode;
    row.measured = measured;
    row.gates = Object.fromEntries(Object.entries(GATES).map(([gate, browsers]) => [gate, {
      gated: browsers.has(browserName), pass: pass[gate],
    }]));
    row.samples = result.samples.length;
    row.feed = result.feed;
    const red = Object.values(row.gates).some((gate) => gate.gated && !gate.pass)
      || result.instanceMode !== cell.mode;
    row.outcome = red ? "red" : browserName === "chromium" ? "pass" : "pass-gated-reported";
    return row;
  } catch (error) {
    row.outcome = "red";
    row.error = error.message;
    return row;
  } finally {
    row.wallSeconds = Number(((Date.now() - started) / 1_000).toFixed(1));
    await browser?.close().catch(() => {});
  }
}

/** Run `tasks` (thunks) with at most `limit` at once. */
async function pool(tasks, limit) {
  const results = [];
  let next = 0;
  const workers = Array.from({ length: Math.min(limit, tasks.length) }, async () => {
    while (next < tasks.length) {
      const index = next;
      next += 1;
      results[index] = await tasks[index]();
    }
  });
  await Promise.all(workers);
  return results;
}

function describe(row) {
  const head = `${row.browser} ${row.kind} ${row.mode} ${row.rate} Hz ${row.minutes} min: ${row.outcome}`;
  if (row.gates === undefined) return `${head} (${row.error})`;
  const gates = Object.entries(row.gates).map(([gate, { gated, pass }]) =>
    `${gate}=${pass ? "pass" : "red"}${gated ? "" : "(reported)"}`).join(" ");
  const misses = row.measured["deadline-misses"];
  return `${head} ${gates} peakBlockMs=${misses.peakBlockMsAfterWarmup} budgetMs=${misses.budgetMs}`
    + ` misses=${misses.afterWarmup} underrunFrames=${row.measured["underrun-frames"].afterWarmup}`
    + ` memoryGrowth=${row.measured["memory-flat"].growthAfterWarmup} allocations=${row.measured["render-allocations"].count}`;
}

async function main() {
  const artifacts = option("--artifacts");
  if (artifacts === null) throw new Error("usage: npm run soak -- --artifacts DIR [--sdk-root DIR] (see soak.mjs)");
  const browserOption = option("--browser", "all");
  const browserNames = browserOption === "all" ? Object.keys(ENGINES) : [browserOption];
  for (const name of browserNames) if (!(name in ENGINES)) throw new Error(`unknown browser: ${name}`);
  const kinds = list("--cells", "deadline,soak", ["deadline", "soak"]);
  const rates = list("--rates", RATES.join(","), RATES.map(String)).map(Number);
  const modes = list("--modes", "worker,single", ["worker", "single"]);
  const minutesDeadline = positive("--minutes-deadline", 10);
  const minutesSoak = positive("--minutes-soak", 60);
  const soakRate = Number(list("--soak-rate", "96000", RATES.map(String))[0]);
  const warmupSeconds = positive("--warmup-seconds", 60);
  const parallel = positive("--parallel-per-browser", 2);
  const out = option("--out");
  const expectRed = process.argv.includes("--expect-red");
  for (const minutes of [kinds.includes("deadline") ? minutesDeadline : Infinity, kinds.includes("soak") ? minutesSoak : Infinity]) {
    if (minutes * 60 <= warmupSeconds) throw new Error("every cell must play longer than its warm-up");
  }
  if (out !== null) {
    // Refuse to overwrite a record before any workload runs.
    await readFile(out).then(() => { throw new Error(`refusing to overwrite ${out}`); }, (error) => {
      if (error.code !== "ENOENT") throw error;
    });
  }
  const parsed = JSON.parse(await readFile(SESSION, "utf8"));
  if (parsed.sources.length !== 1 || parsed.sources[0].channels !== 2 || parsed.submixes.length !== 0) {
    throw new Error("the soak session must have one stereo source and no submixes");
  }
  const session = { document: parsed, sourceId: parsed.sources[0].id, tracks: parsed.tracks.map((track) => track.id) };

  const sdkBundle = await buildSdkBundle(path.resolve(option("--sdk-root", path.join(HERE, "../../../sdk"))));
  const server = await startQualificationServer({ artifacts, sdkBundle, isolated: true });
  const started = Date.now();
  try {
    const perBrowser = await Promise.all(browserNames.map(async (browserName) => {
      const run = (cell) => async () => {
        const row = await runCell(browserName, server.origin, session, cell, warmupSeconds);
        process.stdout.write(`${describe(row)}\n`);
        return row;
      };
      // The soak cells start at once; the deadline cells run beside them, `parallel` at a time.
      const soak = kinds.includes("soak")
        ? modes.map((mode) => run({ kind: "soak", mode, rate: soakRate, minutes: minutesSoak })) : [];
      const deadline = kinds.includes("deadline")
        ? rates.flatMap((rate) => modes.map((mode) => run({ kind: "deadline", mode, rate, minutes: minutesDeadline })))
        : [];
      const [soakRows, deadlineRows] = await Promise.all([
        Promise.all(soak.map((task) => task())), pool(deadline, parallel),
      ]);
      return [...deadlineRows, ...soakRows];
    }));
    const rows = perBrowser.flat();
    if (out !== null) await writeFile(out, rows.map((row) => JSON.stringify(row)).join("\n") + "\n", { flag: "wx" });
    const red = rows.filter((row) => row.outcome === "red");
    process.stdout.write(`soak: ${rows.length} cells, ${red.length} red, ${((Date.now() - started) / 60_000).toFixed(1)} min\n`);
    if (expectRed) {
      // The mutation's proof: a per-block leaked allocation is red on both gates it reaches.
      const caught = ["render-allocations", "memory-flat"].map((gate) => {
        const cells = rows.filter((row) => row.gates?.[gate]?.pass === false);
        process.stdout.write(`expect-red: ${gate} red in ${cells.length} of ${rows.length} cells\n`);
        return cells.length > 0;
      });
      if (caught.includes(false)) {
        process.stdout.write("expect-red: the mutation was not caught by every gate it reaches\n");
        process.exitCode = 1;
      } else {
        process.stdout.write("expect-red: the mutation is red on render-allocations and memory-flat\n");
      }
    } else if (red.length !== 0) {
      process.exitCode = 1;
    }
  } finally {
    await server.close();
    await rm(sdkBundle, { recursive: true, force: true });
  }
}

await main();
