/** Issue #327: black-box qualification of the built enginectl executable. */

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFile, mkdir, mkdtemp, readFile, readdir, realpath, rename, stat, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawn } from "node:child_process";
import { describe, test } from "node:test";

const executable = resolve(process.env.ENGINECTL ?? "dist/enginectl.js");
const CONTENT = `sha256:${"0".repeat(64)}`;
const repoRoot = resolve(import.meta.dirname, "../..");

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

async function run(args, input, options = {}) {
  const child = spawn(
    process.execPath,
    [...(options.nodeArgs ?? []), executable, ...args],
    {
      cwd: options.cwd,
      env: { ...process.env, ...(options.env ?? {}) },
      stdio: ["pipe", "pipe", "pipe"],
    },
  );
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

async function runModule(script, options = {}) {
  const child = spawn(process.execPath, ["--input-type=module", "--eval", script], {
    cwd: options.cwd,
    env: { ...process.env, ...(options.env ?? {}) },
    stdio: ["ignore", "ignore", "pipe"],
  });
  const stderr = [];
  child.stderr.on("data", (chunk) => stderr.push(chunk));
  const status = await new Promise((accept, reject) => {
    child.once("error", reject);
    child.once("close", accept);
  });
  return { status, stderr: Buffer.concat(stderr) };
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
        if (args.length === 3) {
          const help = result.stdout.toString("utf8");
          for (const semantic of [
            /--request reads one strict JSON request/,
            /--output - writes only raw canonical JSON \(with its final LF\)/,
            /successful stderr is empty/,
            /published atomically before stdout emits one compact JSON receipt plus LF/,
            /refused unless --overwrite is present/,

            /Failures leave stdout empty and write one JSON stderr\s+document/,
            /exit 2 is usage, 3 is input\/build refusal, 4 is engine refusal, 5 is output refusal/,
            /70 is internal or packaged-asset failure/,
            /non-interactive and offline/,
          ]) assert.match(help, semantic);
        }
      }
    } finally {
      await rename(unavailable, wasm);
    }
  });

  test("an early-closing stdout consumer is clean cancellation", async () => {
    const child = spawn(process.execPath, [executable, "--help"], { stdio: ["ignore", "pipe", "pipe"] });
    const stderr = [];
    child.stderr.on("data", (chunk) => stderr.push(chunk));
    child.stdout.destroy();
    const status = await new Promise((accept, reject) => {
      child.once("error", reject);
      child.once("close", accept);
    });
    assert.equal(status, 0);
    assert.equal(Buffer.concat(stderr).byteLength, 0);
  });

  test("stdin to stdout is raw canonical JSON", async () => {
    const result = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(request()));
    assert.equal(result.status, 0, result.stderr.toString("utf8"));
    assert.equal(result.stderr.byteLength, 0);
    assert.match(result.stdout.toString("utf8"), /^\{\n  "schema_version": 1,/);
    assert.equal(result.stdout.at(-1), 10);
    assert.doesNotMatch(result.stdout.toString("utf8"), /"command":"session\.build"/);

    const maximum = request();
    maximum.session.revision = "18446744073709551615";
    maximum.sources[0].spec.frames = "18446744073709551615";
    maximum.sources[1].spec.frames = "18446744073709551615";
    maximum.automation[0].segments[0].endSample = "18446744073709551615";
    const accepted = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(maximum));
    assert.equal(accepted.status, 0, accepted.stderr.toString("utf8"));
    const document = JSON.parse(accepted.stdout.toString("utf8"));
    assert.equal(document.revision, "18446744073709551615");
    assert.equal(document.sources[0].frames, "18446744073709551615");
    assert.equal(document.automation[0].segments[0].end_sample, "18446744073709551615");
  });

  test("retired --stems is unknown and publishes no output", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-retired-stems-"));
    const output = resolve(directory, "must-not-exist.json");
    const result = await run(["session", "build", "--stems", directory, "--output", output]);
    failure(result, 2, "cli.usage");
    await assert.rejects(readFile(output), (error) => error?.code === "ENOENT");
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
    assert.equal(built.stdout.toString("utf8"), direct.toJson());

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
    const outputPath = resolve(directory, "session.json");
    const preloadPath = resolve(directory, "publication-order.mjs");
    const auditPath = resolve(directory, "publication-order.json");
    await writeFile(requestPath, JSON.stringify(request()));
    await writeFile(preloadPath, `
import { existsSync, writeFileSync } from "node:fs";
const original = process.stdout.write.bind(process.stdout);
process.stdout.write = function (...args) {
  writeFileSync(process.env.ENGINECTL_AUDIT, JSON.stringify({ published: existsSync(process.env.ENGINECTL_OUTPUT) }));
  return original(...args);
};
`);
    const result = await run(
      ["session", "build", "--request", requestPath, "--output", outputPath],
      undefined,
      {
        nodeArgs: ["--import", preloadPath],
        env: { ENGINECTL_AUDIT: auditPath, ENGINECTL_OUTPUT: outputPath },
      },
    );
    assert.equal(result.status, 0, result.stderr.toString("utf8"));
    assert.equal(result.stderr.byteLength, 0);
    assert.deepEqual(JSON.parse(await readFile(auditPath, "utf8")), { published: true });
    const receipt = JSON.parse(result.stdout.toString("utf8"));
    const bytes = await readFile(outputPath);
    assert.equal(receipt.output.path, outputPath);
    assert.equal(receipt.output.bytes, bytes.byteLength);
    assert.equal(receipt.output.sha256, createHash("sha256").update(bytes).digest("hex"));
    assert.equal("resolvedPath" in receipt.output, false);
    assert.equal(
      result.stdout.toString("utf8"),
      `${JSON.stringify({
        schemaVersion: 1,
        command: "session.build",
        output: {
          path: outputPath,
          bytes: bytes.byteLength,
          sha256: createHash("sha256").update(bytes).digest("hex"),
        },
      })}\n`,
    );

    await writeFile(outputPath, "sentinel");
    const refused = await run(["session", "build", "--request", requestPath, "--output", outputPath]);
    failure(refused, 5, "output.publish");
    assert.equal(await readFile(outputPath, "utf8"), "sentinel");
    assert.equal((await readdir(directory)).some((name) => name.includes(".enginectl-") && name.endsWith(".tmp")), false);

    const replaced = await run(["session", "build", "--request", requestPath, "--output", outputPath, "--overwrite"]);
    assert.equal(replaced.status, 0, replaced.stderr.toString("utf8"));
    assert.match(await readFile(outputPath, "utf8"), /^\{\n  "schema_version": 1,/);
  });

  test("a post-publication stdout failure reports effect applied exactly once", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-report-eval-"));
    const requestPath = resolve(directory, "request.json");
    const outputPath = resolve(directory, "session.json");
    const preloadPath = resolve(directory, "fail-stdout.mjs");
    await writeFile(requestPath, JSON.stringify(request()));
    await writeFile(preloadPath, `
process.stdout.write = function () {
  const error = Object.assign(new Error("forced stdout stream failure"), { code: "EIO" });
  queueMicrotask(() => process.stdout.emit("error", error));
  return false;
};
`);
    const result = await run(
      ["session", "build", "--request", requestPath, "--output", outputPath],
      undefined,
      { nodeArgs: ["--import", preloadPath] },
    );
    assert.equal(result.status, 70);
    assert.equal(result.stdout.byteLength, 0);
    assert.match(await readFile(outputPath, "utf8"), /^\{\n  "schema_version": 1,/);
    const lines = result.stderr.toString("utf8").trimEnd().split("\n");
    assert.equal(lines.length, 1);
    const document = JSON.parse(lines[0]);
    assert.equal(document.error.code, "output.report");
    assert.equal(document.effect, "applied");
    assert.doesNotMatch(result.stderr.toString("utf8"), /node:events|Unhandled|\x1b\[/);
  });

  test("usage and request refusals are structured and create no output", async () => {
    failure(await run(["session", "build", "--request", "-", "--request", "-", "--output", "-"]), 2, "cli.usage");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], "{"), 3, "request.json");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], Buffer.from([0xff])), 3, "request.utf8");
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(request({ surprise: true }))), 3, "request.shape");
    const numeric = request();
    numeric.automation[0].segments[0].startSample = Number.MAX_SAFE_INTEGER + 1;
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(numeric)), 3, "request.shape");
    for (const invalid of ["00", "+1", " 1", "18446744073709551616"]) {
      const malformed = request();
      malformed.session.revision = invalid;
      failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(malformed)), 3, "request.shape");
    }
    const badEffect = request();
    badEffect.tracks[0].spec.dynamic[0].effectId = "miso.nope";
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(badEffect)), 3, "request.shape");
    const magicParameter = JSON.stringify(request()).replace(
      '"parameters":{"threshold":-18',
      '"parameters":{"__proto__":0,"threshold":-18',
    );
    const magicResult = await run(["session", "build", "--request", "-", "--output", "-"], magicParameter);
    failure(magicResult, 3, "request.shape");
    assert.match(JSON.parse(magicResult.stderr.toString("utf8")).error.message, /has no parameter '__proto__'/);
    failure(await run(["session", "build", "--request", "-", "--output", "-"], Buffer.alloc(4 * 1024 * 1024 + 1, 0x20)), 3, "request.too_large");
  });

  test("a missing packaged Wasm asset is internal, not a session refusal", async () => {
    const wasm = resolve(dirname(executable), "assets", "miso-engine-v1-audio-worklet.simd128.wasm");
    const unavailable = `${wasm}.unavailable`;
    await rename(wasm, unavailable);
    try {
      const result = await run(
        ["session", "build", "--request", "-", "--output", "-"],
        JSON.stringify(request()),
      );
      failure(result, 70, "internal.packaged_asset");
      const document = JSON.parse(result.stderr.toString("utf8"));
      assert.equal(document.phase, "asset");
      assert.ok(document.diagnostics.length > 0);
    } finally {
      await rename(unavailable, wasm);
    }
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
    assert.equal(typeof document.phase, typeof "");
    assert.equal(typeof document.result, "number");
    assert.ok(Array.isArray(document.diagnostics));
    assert.ok(document.diagnostics.length > 0);
  });
});
