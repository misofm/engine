import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { chromium, firefox, webkit } from "playwright";
import { startFlacDecoderServer } from "./flac-decoder-server.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, "../../..");
const ENGINES = { chromium, firefox, webkit };

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

async function vectors() {
  const manifest = await readFile(
    path.join(ROOT, "fixtures/flac-delivery/v1/FLAC_VECTORS.tsv"),
    "utf8",
  );
  const lines = manifest.trimEnd().split("\n");
  assert.equal(lines.shift(), "schema_version\t1");
  assert.equal(
    lines.shift(),
    "vector\tbit_depth\tchannels\tframes\tconfigured_block_frames\tidentity\tpcm_file\tflac_file\tflac_sha256",
  );
  return Promise.all(lines.map(async (line) => {
    const fields = line.split("\t");
    const pcm = new Uint8Array(await readFile(
      path.join(ROOT, "fixtures/flac-delivery/v1", fields[6]),
    ));
    return {
      vector: fields[0],
      bitDepth: Number(fields[1]),
      channels: Number(fields[2]),
      frames: fields[3],
      blockFrames: Number(fields[4]),
      identity: fields[5],
      pcmFile: fields[6],
      flacFile: path.basename(fields[7]),
      canonicalBytes: pcm.byteLength,
      canonicalHex: Buffer.from(pcm).toString("hex"),
    };
  }));
}

function validate(browserName, result, expected) {
  assert.equal(result.schema, "miso.flac.browser.v1", `${browserName}: ${result.message ?? "schema"}`);
  assert.equal(result.worker, true);
  assert.equal(result.rows.length, expected.length);
  for (let index = 0; index < expected.length; index += 1) {
    const row = result.rows[index];
    const vector = expected[index];
    assert.equal(row.vector, vector.vector);
    assert.equal(row.canonicalHex, vector.canonicalHex, `${browserName}: ${vector.flacFile}`);
    assert.equal(`sha256:${row.digest}`, vector.identity, `${browserName}: ${vector.flacFile}`);
    assert.deepEqual(
      row.blockFrames,
      Array(Number(vector.frames) / vector.blockFrames).fill(vector.blockFrames),
      `${browserName}: ${vector.flacFile}: name/actual frame content`,
    );
    assert.notEqual(
      `sha256:${row.mutatedDigest}`,
      vector.identity,
      `${browserName}: one-LSB red mutation escaped`,
    );
    assert.deepEqual(row.stream, {
      sampleRateHz: 48000,
      channels: vector.channels,
      bitDepth: vector.bitDepth,
      frames: vector.frames,
    });
    assert.equal(Object.is(row.firstPumpSample, 0), true);
  }
  assert.equal(
    result.provenanceMutation,
    "miso.flac.decoder.artifact_mismatch",
    `${browserName}: artifact red mutation escaped`,
  );
}

async function main() {
  const artifacts = option("--decoder-artifacts");
  const browserName = option("--browser");
  if (artifacts === null || browserName === null || !(browserName in ENGINES)) {
    throw new Error(
      "usage: npm run flac-decoder -- --decoder-artifacts DIR --browser chromium|firefox|webkit",
    );
  }
  const expected = await vectors();
  const server = await startFlacDecoderServer({ artifacts });
  const launchOptions = { headless: true };
  if (browserName === "chromium") {
    launchOptions.channel = "chromium";
    launchOptions.args = ["--disable-dev-shm-usage"];
  }
  const browser = await ENGINES[browserName].launch(launchOptions);
  try {
    const page = await browser.newPage();
    page.setDefaultTimeout(120000);
    await page.goto(`${server.origin}/`);
    const result = await page.evaluate((vectorRows) => new Promise((resolve, reject) => {
      const worker = new Worker("/qualification/flac-decoder-worker.js", { type: "module" });
      worker.onmessage = ({ data }) => {
        worker.terminate();
        resolve(data);
      };
      worker.onerror = (error) => {
        worker.terminate();
        reject(new Error(error.message));
      };
      worker.postMessage({ vectors: vectorRows });
    }), expected);
    validate(browserName, result, expected);
    process.stdout.write(
      `${browserName}: FLAC Worker vectors, identity, pump, and provenance gates passed (${browser.version()})\n`,
    );
  } finally {
    await browser.close();
    await server.close();
  }
}

await main();
