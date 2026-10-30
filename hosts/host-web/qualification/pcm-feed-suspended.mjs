/** Run the shared-memory row on its own isolated page; the main matrix stays non-isolated. */
export async function qualifySuspendedPcmFeed(browserName, engine, origin) {
  const browser = await engine.launch({ headless: true,
    ...(browserName === "chromium" ? { channel: "chromium", args: ["--disable-dev-shm-usage"] } : {}),
  });
  try {
    const page = await browser.newPage();
    const errors = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    await page.goto(`${origin}/qualification/index.html`);
    let deadline;
    const result = await Promise.race([
      page.evaluate(async () => {
        const { runSuspendedPcmFeed } = await import("/sdk/pcm-feed-suspended-client.js");
        return runSuspendedPcmFeed();
      }),
      new Promise((_, reject) => {
        deadline = setTimeout(() => reject(new Error("suspended PCM feed row exceeded 30 s")), 30_000);
      }),
    ]).finally(() => clearTimeout(deadline));
    if (errors.length > 0) throw new Error(errors.join("; "));
    process.stdout.write(`${browserName}: pcm-feed-suspended: ${JSON.stringify(result)} (${browser.version()})\n`);
    // Both modes and both phases run before any fails, so a red run reports every one.
    const failures = [];
    for (const phase of LIVE_PHASES) {
      for (const mode of ["worker", "single"]) {
        await qualifyLiveSeek(browser, browserName, origin, mode, phase).catch((error) => failures.push(error.message));
      }
    }
    if (failures.length !== 0) throw new Error(failures.join("; "));
    return result;
  } catch (error) {
    throw new Error(`${browserName}: pcm-feed-suspended: ${error.message}`, { cause: error });
  } finally {
    await browser.close();
  }
}

/**
 * The live seek phases: #2185's plain ring seek (`runLivePcmFeedSeek`) and #2205's transport seek
 * (`runLiveTransportSeek`, `engine.seek(frame, { producers })` with a producer that publishes 20 ms
 * after the seek).
 */
export const LIVE_PHASES = [
  { name: "pcm-feed-live-seek", run: "runLivePcmFeedSeek" },
  { name: "pcm-feed-transport-seek", run: "runLiveTransportSeek" },
];

/**
 * One live seek phase in one instance mode on a fresh isolated page. Chromium gates every seek at
 * 0 underrun frames in both modes (#2185, #2205): the engine keeps the old position playing until
 * the new one has its runway. WebKit's headless sink renders in bursts (#1387), so its rows are
 * reported, not gated. A transport row also holds the source and the timeline to one landing block,
 * and the phase reports the latency the landing adds (landed sample minus the render sample at
 * accept).
 */
export async function qualifyLiveSeek(browser, browserName, origin, mode, phase = LIVE_PHASES[0]) {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  try {
    await page.goto(`${origin}/qualification/index.html`);
    let deadline;
    const running = page.evaluate(async ({ instanceMode, run }) => {
      const client = await import("/sdk/pcm-feed-suspended-client.js");
      return client[run](instanceMode);
    }, { instanceMode: mode, run: phase.run });
    const clicked = page.click(`[data-live-seek-resume="${mode}"]`, { timeout: 30_000 });
    const result = await Promise.race([
      Promise.all([running, clicked]).then(([value]) => value),
      new Promise((_, reject) => {
        deadline = setTimeout(() => reject(new Error(`${phase.name} row exceeded 30 s`)), 30_000);
      }),
    ]).finally(() => clearTimeout(deadline));
    if (errors.length > 0) throw new Error(errors.join("; "));
    const gated = browserName === "chromium";
    const label = gated ? "" : " (reported, not gated)";
    const latencies = result.seeks.filter((row) => row.addedLatencySamples !== undefined)
      .map((row) => (Number(row.addedLatencySamples) / 48).toFixed(1));
    const latency = latencies.length === 0 ? "" : ` added latency ms [${latencies.join(", ")}]`;
    process.stdout.write(`${browserName}: ${phase.name} ${mode}${label}:${latency} ${JSON.stringify(result)}\n`);
    const missing = result.seeks.filter((row) => row.underrunFrames !== "0");
    const split = result.seeks.filter((row) => row.timelineLandedSample !== undefined
      && (row.timelineLandedSample !== row.sourceLandedSample || row.timelineGeneration !== row.generation));
    if (result.refused !== 0 || split.length !== 0
        || result.seeksApplied !== result.seeks.length
        || result.renderAllocations.result !== 0 || result.renderAllocations.count !== 0
        || (gated && missing.length !== 0)) {
      throw new Error(`${phase.name} ${mode} invariant failed: ${JSON.stringify(result)}`);
    }
    return result;
  } catch (error) {
    throw new Error(`${browserName}: ${phase.name} ${mode}: ${error.message}`, { cause: error });
  } finally {
    await page.close();
  }
}
