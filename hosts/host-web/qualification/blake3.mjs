/** One browser parity row: the shipped SIMD module hashes the canonical identity corpus in a Worker. */
export async function qualifyBlake3(browserName, engine, origin) {
  const browser = await engine.launch({ headless: true,
    ...(browserName === "chromium" ? { channel: "chromium", args: ["--disable-dev-shm-usage"] } : {}),
  });
  try {
    const page = await browser.newPage();
    await page.goto(`${origin}/qualification/index.html`);
    const result = await page.evaluate(() => new Promise((resolve, reject) => {
      const worker = new Worker("/qualification/blake3-worker.js", { type: "module" });
      const timeout = setTimeout(() => { worker.terminate(); reject(new Error("Worker exceeded 30 s")); }, 30_000);
      const finish = (error, value) => {
        clearTimeout(timeout);
        worker.terminate();
        if (error) reject(error); else resolve(value);
      };
      worker.onerror = (event) => finish(new Error(event.message));
      worker.onmessage = ({ data }) => finish(data.error ? new Error(data.error) : undefined, data);
    }));
    if (result.backend !== "simd128" || !Array.isArray(result.vectors) || result.vectors.length === 0) {
      throw new Error(`invalid Worker result: ${JSON.stringify(result)}`);
    }
    process.stdout.write(`${browserName}: blake3-worker: ${JSON.stringify(result)} (${browser.version()})\n`);
    return { browser: browserName, backend: result.backend };
  } catch (error) {
    throw new Error(`${browserName}: blake3-worker: ${error.message}`, { cause: error });
  } finally {
    await browser.close();
  }
}
