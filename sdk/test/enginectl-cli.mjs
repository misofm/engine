/** Issue #327: black-box qualification of the built enginectl executable. */

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtemp, readFile, readdir, rename, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawn } from "node:child_process";
import { describe, test } from "node:test";

const executable = resolve(process.env.ENGINECTL ?? "dist/enginectl.js");
const CONTENT = `sha256:${"0".repeat(64)}`;

function request(overrides = {}) {
  return {
    schemaVersion: 1,
    session: { id: "cli.eval", sampleRateHz: 48_000, revision: 0, quantumFrames: 128 },
    sources: [
      { id: "stem", spec: { channels: 2, bitDepth: "32f", frames: 48_000, content: CONTENT } },
      { id: "di", spec: { channels: 1, bitDepth: 24, frames: 48_000, content: CONTENT } },
    ],
    tracks: [
      {
        id: "vocal",
        spec: {
          source: "stem",
          builtins: { left: { trimDb: -1, hpfHz: 80 }, right: { trimDb: -1, hpfHz: 80 } },
          fader: { leftDb: -3, rightDb: -3 },
          pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: 1 }, smoothingSamples: 8 },
          simd1: [{ effectId: "miso.parametric-eq", parameters: { "band-1-enabled": true, "band-1-gain": -2 }, options: { slotId: "eq" } }],
          dynamic: [{
            effectId: "miso.compressor",
            parameters: { threshold: -18, ratio: 4 },
            options: {
              slotId: "comp",
              sidechain: {
                source: { kind: "track", trackId: "bass", tap: "post_fader" },
                portId: "sidechain-in",
              },
            },
          }],
          simd2: [{ effectId: "miso.soft-clip", parameters: { drive: 3 } }],
        },
      },
      { id: "bass", spec: { source: { id: "di", left: 0, right: 0 }, pan: { left: -1, right: 1 } } },
    ],
    submixes: ["bus"],
    outputs: ["main"],
    routes: [
      { id: "to-bus", source: { kind: "track", trackId: "vocal", tap: "post_matrix" }, destination: { kind: "submix_input", submixId: "bus" } },
      { id: "bass-bus", source: { kind: "track", trackId: "bass", tap: "post_matrix" }, destination: { kind: "submix_input", submixId: "bus" } },
      { id: "to-main", source: { kind: "submix_output", submixId: "bus" }, destination: { kind: "output_input", outputId: "main" } },
    ],
    automation: [{
      id: "eq-ride",
      target: { trackId: "vocal", rack: "simd1", slotId: "eq", parameter: "band-1-gain", channel: "both" },
      segments: [{ shape: "linear", startSample: "0", endSample: "480", startValue: -2, endValue: 0 }],
    }],
    ...overrides,
  };
}

async function run(args, input) {
  const child = spawn(process.execPath, [executable, ...args], { stdio: ["pipe", "pipe", "pipe"] });
  const stdout = [];
  const stderr = [];
  child.stdout.on("data", (chunk) => stdout.push(chunk));
  child.stderr.on("data", (chunk) => stderr.push(chunk));
  if (input === undefined) child.stdin.end();
  else child.stdin.end(input);
  const status = await new Promise((accept, reject) => {
    child.once("error", reject);
    child.once("close", accept);
  });
  return { status, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) };
}

function failure(result, status, code) {
  assert.equal(result.status, status);
  assert.equal(result.stdout.byteLength, 0);
  const document = JSON.parse(result.stderr.toString("utf8"));
  assert.equal(document.schemaVersion, 1);
  assert.equal(document.error.code, code);
  assert.equal(document.effect, "not_applied");
  assert.doesNotMatch(result.stderr.toString("utf8"), /\x1b\[/);
}

describe("enginectl session build", () => {
  test("help and version are independent of stdin and the engine", async () => {
    const wasm = resolve(dirname(executable), "assets", "miso-engine-v1-audio-worklet.simd128.wasm");
    const unavailable = `${wasm}.unavailable`;
    await rename(wasm, unavailable);
    try {
      for (const args of [["--help"], ["session", "--help"], ["session", "build", "--help"], ["--version"]]) {
        const result = await run(args);
        assert.equal(result.status, 0);
        assert.ok(result.stdout.byteLength > 0);
        assert.equal(result.stderr.byteLength, 0);
      }
    } finally {
      await rename(unavailable, wasm);
    }
  });

  test("stdin to stdout is raw canonical TOML", async () => {
    const result = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(request()));
    assert.equal(result.status, 0, result.stderr.toString("utf8"));
    assert.equal(result.stderr.byteLength, 0);
    assert.match(result.stdout.toString("utf8"), /^schema_version = 1\n/);
    assert.equal(result.stdout.at(-1), 10);
    assert.doesNotMatch(result.stdout.toString("utf8"), /"command":"session\.build"/);
  });

  test("rich mapping equals the direct builder and entity permutations are canonical", async () => {
    const { effect, session } = await import(pathToFileURL(resolve(dirname(executable), "index.js")).href);
    const direct = session({ id: "cli.eval", sampleRateHz: 48_000, revision: 0, quantumFrames: 128 })
      .source("stem", { channels: 2, bitDepth: "32f", frames: 48_000, content: CONTENT })
      .source("di", { channels: 1, bitDepth: 24, frames: 48_000, content: CONTENT })
      .submix("bus")
      .output("main")
      .track("vocal", {
        source: "stem",
        builtins: { left: { trimDb: -1, hpfHz: 80 }, right: { trimDb: -1, hpfHz: 80 } },
        fader: { leftDb: -3, rightDb: -3 },
        pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: 1 }, smoothingSamples: 8 },
        simd1: [effect("miso.parametric-eq", { "band-1-enabled": true, "band-1-gain": -2 }, { slotId: "eq" })],
        dynamic: [effect("miso.compressor", { threshold: -18, ratio: 4 }, {
          slotId: "comp",
          sidechain: {
            source: { kind: "track", trackId: "bass", tap: "post_fader" },
            portId: "sidechain-in",
          },
        })],
        simd2: [effect("miso.soft-clip", { drive: 3 })],
      })
      .track("bass", { source: { id: "di", left: 0, right: 0 }, pan: { left: -1, right: 1 } })
      .route({ id: "to-bus", source: { kind: "track", trackId: "vocal", tap: "post_matrix" }, destination: { kind: "submix_input", submixId: "bus" } })
      .route({ id: "bass-bus", source: { kind: "track", trackId: "bass", tap: "post_matrix" }, destination: { kind: "submix_input", submixId: "bus" } })
      .route({ id: "to-main", source: { kind: "submix_output", submixId: "bus" }, destination: { kind: "output_input", outputId: "main" } })
      .automation({
        id: "eq-ride",
        target: { trackId: "vocal", rack: "simd1", slotId: "eq", parameter: "band-1-gain", channel: "both" },
        segments: [{ shape: "linear", startSample: 0n, endSample: 480n, startValue: -2, endValue: 0 }],
      });
    const original = request();
    const built = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(original));
    assert.equal(built.status, 0, built.stderr.toString("utf8"));
    assert.equal(built.stdout.toString("utf8"), direct.toToml());

    const permuted = request();
    permuted.sources.reverse();
    permuted.tracks.reverse();
    permuted.routes.reverse();
    const reordered = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(permuted));
    assert.equal(reordered.status, 0, reordered.stderr.toString("utf8"));
    assert.deepEqual(reordered.stdout, built.stdout);
  });

  test("file output is published before its matching receipt", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-eval-"));
    const requestPath = resolve(directory, "request.json");
    const outputPath = resolve(directory, "session.toml");
    await writeFile(requestPath, JSON.stringify(request()));
    const result = await run(["session", "build", "--request", requestPath, "--output", outputPath]);
    assert.equal(result.status, 0, result.stderr.toString("utf8"));
    assert.equal(result.stderr.byteLength, 0);
    const receipt = JSON.parse(result.stdout.toString("utf8"));
    const bytes = await readFile(outputPath);
    assert.equal(receipt.output.path, outputPath);
    assert.equal(receipt.output.bytes, bytes.byteLength);
    assert.equal(receipt.output.sha256, createHash("sha256").update(bytes).digest("hex"));

    await writeFile(outputPath, "sentinel");
    const refused = await run(["session", "build", "--request", requestPath, "--output", outputPath]);
    failure(refused, 5, "output.publish");
    assert.equal(await readFile(outputPath, "utf8"), "sentinel");
    assert.equal((await readdir(directory)).some((name) => name.includes(".enginectl-") && name.endsWith(".tmp")), false);

    const replaced = await run(["session", "build", "--request", requestPath, "--output", outputPath, "--overwrite"]);
    assert.equal(replaced.status, 0, replaced.stderr.toString("utf8"));
    assert.match(await readFile(outputPath, "utf8"), /^schema_version = 1\n/);
  });

  test("usage and request refusals are structured and create no output", async () => {
    failure(await run(["session", "build", "--request", "-", "--request", "-", "--output", "-"]), 2, "cli.usage");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], "{"), 3, "request.json");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], Buffer.from([0xff])), 3, "request.utf8");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(request({ surprise: true }))), 3, "request.shape");
    const numeric = request();
    numeric.automation[0].segments[0].startSample = 0;
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(numeric)), 3, "request.shape");
    const badEffect = request();
    badEffect.tracks[0].spec.dynamic[0].effectId = "miso.nope";
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(badEffect)), 3, "request.shape");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], Buffer.alloc(4 * 1024 * 1024 + 1, 0x20)), 3, "request.too_large");
  });

  test("an engine graph refusal preserves its ordered diagnostics", async () => {
    const cyclic = request({
      submixes: ["bus", "loop"],
      routes: [
        { id: "to-bus", source: { kind: "track", trackId: "vocal", tap: "post_matrix" }, destination: { kind: "submix_input", submixId: "bus" } },
        { id: "bus-loop", source: { kind: "submix_output", submixId: "bus" }, destination: { kind: "submix_input", submixId: "loop" } },
        { id: "loop-bus", source: { kind: "submix_output", submixId: "loop" }, destination: { kind: "submix_input", submixId: "bus" } },
        { id: "loop-main", source: { kind: "submix_output", submixId: "loop" }, destination: { kind: "output_input", outputId: "main" } },
      ],
    });
    const result = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(cyclic));
    failure(result, 4, "engine.refused");
    const document = JSON.parse(result.stderr.toString("utf8"));
    assert.equal(typeof document.phase, "string");
    assert.equal(typeof document.result, "number");
    assert.ok(Array.isArray(document.diagnostics));
    assert.ok(document.diagnostics.length > 0);
  });
});
