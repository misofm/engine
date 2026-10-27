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
    return result;
  } catch (error) {
    throw new Error(`${browserName}: pcm-feed-suspended: ${error.message}`, { cause: error });
  } finally {
    await browser.close();
  }
}
