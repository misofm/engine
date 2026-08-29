import assert from "node:assert/strict";
import process from "node:process";
import { chromium, firefox, webkit } from "playwright";
import { startFlacDecoderServer } from "./flac-decoder-server.mjs";

const ENGINES = { chromium, firefox, webkit };

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

async function main() {
  const artifacts = option("--decoder-artifacts");
  const throughputFlac = option("--flac");
  const browserName = option("--browser");
  const expectedIdentity = option("--identity");
  const maximumCanonicalBytes = Number(option("--canonical-bytes"));
  if (
    artifacts === null
    || throughputFlac === null
    || browserName === null
    || expectedIdentity === null
    || !(browserName in ENGINES)
    || !/^sha256:[0-9a-f]{64}$/.test(expectedIdentity)
    || !Number.isSafeInteger(maximumCanonicalBytes)
    || maximumCanonicalBytes <= 0
  ) {
    throw new Error(
      "usage: npm run flac-throughput -- --decoder-artifacts DIR --flac FILE "
      + "--canonical-bytes N --identity sha256:HEX --browser chromium|firefox|webkit",
    );
  }
  const server = await startFlacDecoderServer({ artifacts, throughputFlac });
  const launchOptions = { headless: true };
  if (browserName === "chromium") {
    launchOptions.channel = "chromium";
    launchOptions.args = ["--disable-dev-shm-usage"];
  }
  const browser = await ENGINES[browserName].launch(launchOptions);
  try {
    const page = await browser.newPage();
    page.setDefaultTimeout(300000);
    await page.goto(`${server.origin}/`);
    const result = await page.evaluate((canonicalBytes) => new Promise((resolve, reject) => {
      const worker = new Worker(
        "/qualification/flac-decoder-throughput-worker.js",
        { type: "module" },
      );
      worker.onmessage = ({ data }) => {
        worker.terminate();
        resolve(data);
      };
      worker.onerror = (error) => {
        worker.terminate();
        reject(new Error(error.message));
      };
      worker.postMessage({ maximumCanonicalBytes: canonicalBytes });
    }), maximumCanonicalBytes);
    assert.equal(result.schema, "miso.flac.throughput.v1", result.message ?? "schema");
    assert.equal(result.canonicalBytes, maximumCanonicalBytes);
    assert.equal(`sha256:${result.digest}`, expectedIdentity);
    assert.ok(Number.isFinite(result.elapsedMs) && result.elapsedMs > 0);
    assert.ok(Number.isFinite(result.mebibytesPerSecond) && result.mebibytesPerSecond > 0);
    process.stdout.write(`${JSON.stringify({ browser: browserName, ...result })}\n`);
  } finally {
    await browser.close();
    await server.close();
  }
}

await main();
