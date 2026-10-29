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
    // Both modes run before either fails, so a red run reports both.
    const failures = [];
    for (const mode of ["worker", "single"]) {
      await qualifyLiveSeek(browser, browserName, origin, mode).catch((error) => failures.push(error.message));
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
 * #2185: the live plain seek phase, in one instance mode on a fresh isolated page. Chromium single
 * mode gates every seek at 0 underrun frames. Chromium worker mode is reported only: it is the open
 * #2185 worker residual. WebKit's headless sink renders in bursts (#1387), so its rows are reported,
 * not gated.
 */
async function qualifyLiveSeek(browser, browserName, origin, mode) {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  try {
    await page.goto(`${origin}/qualification/index.html`);
    let deadline;
    const running = page.evaluate(async (instanceMode) => {
      const { runLivePcmFeedSeek } = await import("/sdk/pcm-feed-suspended-client.js");
      return runLivePcmFeedSeek(instanceMode);
    }, mode);
    const clicked = page.click(`[data-live-seek-resume="${mode}"]`, { timeout: 30_000 });
    const result = await Promise.race([
      Promise.all([running, clicked]).then(([value]) => value),
      new Promise((_, reject) => {
        deadline = setTimeout(() => reject(new Error("live seek row exceeded 30 s")), 30_000);
      }),
    ]).finally(() => clearTimeout(deadline));
    if (errors.length > 0) throw new Error(errors.join("; "));
    // Root ruling (#2185 attempt 2): Chromium single mode is gated at 0 frames. Worker mode is the
    // open #2185 residual (render can pop the Worker's `source_seek` before its first submit), so
    // it is reported, never declared met.
    const gated = browserName === "chromium" && mode === "single";
    const label = gated ? "" : mode === "worker"
      ? " (reported, not gated: open #2185 worker residual)" : " (reported, not gated)";
    process.stdout.write(`${browserName}: pcm-feed-live-seek ${mode}${label}: ${JSON.stringify(result)}\n`);
    const missing = result.seeks.filter((row) => row.underrunFrames !== "0");
    if (result.seeksApplied !== result.seeks.length || result.refused !== 0
        || result.renderAllocations.result !== 0 || result.renderAllocations.count !== 0
        || (gated && missing.length !== 0)) {
      throw new Error(`live seek ${mode} invariant failed: ${JSON.stringify(result)}`);
    }
  } catch (error) {
    throw new Error(`${browserName}: pcm-feed-live-seek ${mode}: ${error.message}`, { cause: error });
  } finally {
    await page.close();
  }
}
