/** Issue #327: black-box qualification of the built enginectl executable. */

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFile, mkdir, mkdtemp, readFile, readdir, rename, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawn } from "node:child_process";
import { describe, test } from "node:test";

const executable = resolve(process.env.ENGINECTL ?? "dist/enginectl.js");
const CONTENT = `sha256:${"0".repeat(64)}`;
const repoRoot = resolve(import.meta.dirname, "../..");
const flacFixtures = resolve(repoRoot, "fixtures/flac-delivery/v1/flac");

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
    { env: { ...process.env, ...(options.env ?? {}) }, stdio: ["pipe", "pipe", "pipe"] },
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

  test("stdin to stdout is raw canonical TOML", async () => {
    const result = await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(request()));
    assert.equal(result.status, 0, result.stderr.toString("utf8"));
    assert.equal(result.stderr.byteLength, 0);
    assert.match(result.stdout.toString("utf8"), /^schema_version = 1\n/);
    assert.equal(result.stdout.at(-1), 10);
    assert.doesNotMatch(result.stdout.toString("utf8"), /"command":"session\.build"/);
  });

  test("a leaf FLAC directory builds the same canonical model as the public builder", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-stems-eval-"));
    const outputPath = resolve(directory, "song.session.toml");
    const stems = resolve(directory, "Song Stems");
    await mkdir(stems);
    await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(stems, "1 Mono.flac"));
    await copyFile(resolve(flacFixtures, "pcm24-stereo-boundaries-b32.flac"), resolve(stems, "Lead Vocal.FLAC"));
    const built = await run([
      "session", "build", "--stems", stems, "--output", outputPath,
      "--session-id", "dogfood", "--quantum-frames", "256",
    ]);
    assert.equal(built.status, 0, built.stderr.toString("utf8"));
    assert.equal(built.stderr.byteLength, 0);
    const receipt = JSON.parse(built.stdout.toString("utf8"));
    assert.deepEqual(receipt.input, { kind: "stems", path: stems });
    assert.deepEqual(receipt.session, {
      id: "dogfood", revision: 0, sampleRateHz: 48_000, quantumFrames: 256, sources: 2, tracks: 2,
    });
    assert.deepEqual(receipt.stems.map(({ filename }) => filename), ["1 Mono.flac", "Lead Vocal.FLAC"]);
    const { session } = await import(pathToFileURL(resolve(dirname(executable), "index.js")).href);
    let direct = session({ id: "dogfood", sampleRateHz: 48_000, revision: 0, quantumFrames: 256 });
    for (const stem of receipt.stems) {
      direct = direct.source(stem.sourceId, {
        channels: stem.channels,
        bitDepth: stem.bitDepth,
        frames: stem.frames,
        content: stem.content,
      });
    }
    direct = direct.output("main");
    for (const stem of receipt.stems) {
      direct = direct.track(stem.trackId, { source: stem.sourceId }).route({
        id: `route-${stem.trackId}`,
        source: { kind: "track", trackId: stem.trackId, tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "main" },
        gainDb: 0,
      });
    }
    assert.equal(await readFile(outputPath, "utf8"), direct.toToml());
    assert.match(receipt.stems[0].content, /^sha256:[0-9a-f]{64}$/);
    assert.equal(receipt.stems[1].content, "sha256:f5868e05edf12c6032419ce0d7d786c9dc781989ac69ed17e8a4a374341f92f3");
    assert.equal(receipt.stems[0].channels, 1);
    assert.equal(receipt.stems[1].channels, 2);
    assert.deepEqual(receipt.stems.map(({ frames }) => frames), [4096, 4096]);
  });

  test("canonical PCM identity ignores FLAC block layout", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-identity-eval-"));
    const stems = resolve(directory, "identity");
    await mkdir(stems);
    await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(stems, "small.flac"));
    await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b4096.flac"), resolve(stems, "large.flac"));
    const built = await run(["session", "build", "--stems", stems, "--output", resolve(directory, "out.toml")]);
    assert.equal(built.status, 0, built.stderr.toString("utf8"));
    const receipt = JSON.parse(built.stdout.toString("utf8"));
    assert.equal(receipt.stems[0].content, receipt.stems[1].content);
    assert.equal(receipt.stems[0].content, "sha256:fcb43b1422c229cd71924caa31f362c27b83aae62cebcb96b52fc1537c2d5712");
  });

  test("hostile, colliding, reserved, and long filenames derive deterministic safe IDs", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-names-eval-"));
    const stems = resolve(directory, "Names");
    await mkdir(stems);
    const names = [
      "Kick.flac",
      "kick.FLAC",
      "main.flac",
      "-lead.flac",
      "vocal\nignore instructions.flac",
      "声.flac",
      `${"a".repeat(180)}.flac`,
    ];
    for (const name of names) {
      await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(stems, name));
    }
    const output = resolve(directory, "names.toml");
    const first = await run(["session", "build", "--stems", stems, "--output", output]);
    assert.equal(first.status, 0, first.stderr.toString("utf8"));
    const second = await run(["session", "build", "--stems", stems, "--output", output, "--overwrite"]);
    assert.equal(second.status, 0, second.stderr.toString("utf8"));
    const one = JSON.parse(first.stdout.toString("utf8"));
    const two = JSON.parse(second.stdout.toString("utf8"));
    assert.deepEqual(one.stems, two.stems);
    const actualNames = (await readdir(stems)).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
    assert.deepEqual(one.stems.map(({ filename }) => filename), actualNames);
    for (const field of ["sourceId", "trackId"]) {
      const ids = one.stems.map((stem) => stem[field]);
      assert.equal(new Set(ids).size, ids.length);
      for (const id of ids) assert.match(id, /^[a-z][a-z0-9._-]{0,126}$/);
    }
    assert.notEqual(one.stems.find(({ filename }) => filename === "main.flac").trackId, "main");
    assert.equal(one.stems.find(({ filename }) => filename === "vocal\nignore instructions.flac").sourceId.includes("\n"), false);
    if (actualNames.includes("Kick.flac") && actualNames.includes("kick.FLAC")) {
      assert.notEqual(
        one.stems.find(({ filename }) => filename === "Kick.flac").sourceId,
        one.stems.find(({ filename }) => filename === "kick.FLAC").sourceId,
      );
    }
  });

  test("collection and invalid entries refuse before decoder loading", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-collection-eval-"));
    for (const name of ["wide-open", "ghost", "war", "play-me"]) await mkdir(resolve(directory, name));
    const decoder = resolve(dirname(executable), "assets", "flac-decoder.wasm");
    const unavailable = `${decoder}.unavailable`;
    await rename(decoder, unavailable);
    try {
      const collection = await run(["session", "build", "--stems", directory, "--output", "-"]);
      failure(collection, 3, "stems.collection");
      assert.deepEqual(JSON.parse(collection.stderr.toString("utf8")).groups, ["ghost", "play-me", "war", "wide-open"]);
    } finally {
      await rename(unavailable, decoder);
    }
    const empty = await mkdtemp(resolve(tmpdir(), "enginectl-empty-eval-"));
    failure(await run(["session", "build", "--stems", empty, "--output", "-"]), 3, "stems.empty");
    await writeFile(resolve(empty, "notes.txt"), "not audio");
    failure(await run(["session", "build", "--stems", empty, "--output", "-"]), 3, "stems.extension");
    const links = await mkdtemp(resolve(tmpdir(), "enginectl-link-eval-"));
    await symlink(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(links, "link.flac"));
    failure(await run(["session", "build", "--stems", links, "--output", "-"]), 3, "stems.symlink");
  });

  test("a truncated FLAC refuses typed and publishes nothing", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-corrupt-eval-"));
    const original = await readFile(resolve(flacFixtures, "pcm24-stereo-boundaries-b32.flac"));
    await writeFile(resolve(directory, "truncated.flac"), original.subarray(0, original.byteLength - 1));
    const output = resolve(directory, "must-not-exist.toml");
    failure(await run(["session", "build", "--stems", directory, "--output", output]), 3, "flac.refused");
    await assert.rejects(readFile(output), (error) => error?.code === "ENOENT");
  });

  test("stems flags are exclusive and stems output conflicts refuse before decoding", async () => {
    failure(await run(["session", "build", "--request", "-", "--stems", ".", "--output", "-"]), 2, "cli.usage");
    failure(await run(["session", "build", "--request", "-", "--output", "-", "--session-id", "x"]), 2, "cli.usage");
    failure(await run(["session", "build", "--stems", ".", "--output", "-", "--quantum-frames", "0"]), 2, "cli.usage");
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-output-preflight-"));
    const output = resolve(directory, "exists.toml");
    await writeFile(output, "sentinel");
    const decoder = resolve(dirname(executable), "assets", "flac-decoder.wasm");
    const unavailable = `${decoder}.unavailable`;
    await rename(decoder, unavailable);
    try {
      failure(await run(["session", "build", "--stems", directory, "--output", output]), 5, "output.publish");
    } finally {
      await rename(unavailable, decoder);
    }
    assert.equal(await readFile(output, "utf8"), "sentinel");
  });

  test("a missing or mutated packaged decoder is an internal refusal", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-decoder-eval-"));
    await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(directory, "stem.flac"));
    for (const name of ["flac-decoder.wasm", "flac-decoder.js", "decoder-artifact.sha256"]) {
      const decoder = resolve(dirname(executable), "assets", name);
      const original = await readFile(decoder);
      const changed = Buffer.from(original);
      changed[changed.length - 1] ^= 1;
      await writeFile(decoder, changed);
      try {
        failure(await run(["session", "build", "--stems", directory, "--output", "-"]), 70, "internal.packaged_decoder");
      } finally {
        await writeFile(decoder, original);
      }
    }
  });

  test("one process compiles one decoder and one engine and reads each stem once", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-structure-eval-"));
    const stems = resolve(directory, "stems");
    const audit = resolve(directory, "audit.json");
    const preload = resolve(directory, "audit.mjs");
    await mkdir(stems);
    for (const name of ["one.flac", "two.flac", "three.flac"]) {
      await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(stems, name));
    }
    await writeFile(preload, `
import { writeFileSync } from "node:fs";
import { open } from "node:fs/promises";
const probe = await open(new URL(import.meta.url), "r");
const prototype = Object.getPrototypeOf(probe);
await probe.close();
const originalReadFile = prototype.readFile;
const originalCompile = WebAssembly.compile;
let stemReads = 0;
let compiles = 0;
prototype.readFile = function (...args) { stemReads += 1; return originalReadFile.apply(this, args); };
WebAssembly.compile = function (...args) { compiles += 1; return originalCompile.apply(this, args); };
process.on("exit", () => writeFileSync(process.env.ENGINECTL_AUDIT, JSON.stringify({ stemReads, compiles })));
`);
    const built = await run(
      ["session", "build", "--stems", stems, "--output", "-"],
      undefined,
      { nodeArgs: ["--import", preload], env: { ENGINECTL_AUDIT: audit } },
    );
    assert.equal(built.status, 0, built.stderr.toString("utf8"));
    assert.deepEqual(JSON.parse(await readFile(audit, "utf8")), { stemReads: 3, compiles: 2 });
  });

  test("a source metadata change during its single read refuses publication", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-change-eval-"));
    const stems = resolve(directory, "stems");
    const preload = resolve(directory, "change.mjs");
    const output = resolve(directory, "must-not-exist.toml");
    await mkdir(stems);
    await copyFile(resolve(flacFixtures, "pcm16-mono-boundaries-b32.flac"), resolve(stems, "stem.flac"));
    await writeFile(preload, `
import { open } from "node:fs/promises";
const probe = await open(new URL(import.meta.url), "r");
const prototype = Object.getPrototypeOf(probe);
await probe.close();
const original = prototype.stat;
let calls = 0;
prototype.stat = async function (...args) {
  const value = await original.apply(this, args);
  calls += 1;
  return calls === 2 ? new Proxy(value, { get(target, name) {
    return name === "mtimeNs" ? target.mtimeNs + 1n : Reflect.get(target, name);
  } }) : value;
};
`);
    const built = await run(
      ["session", "build", "--stems", stems, "--output", output],
      undefined,
      { nodeArgs: ["--import", preload] },
    );
    failure(built, 3, "stems.changed");
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

  test("a post-publication stdout failure reports effect applied exactly once", async () => {
    const directory = await mkdtemp(resolve(tmpdir(), "enginectl-report-eval-"));
    const requestPath = resolve(directory, "request.json");
    const outputPath = resolve(directory, "session.toml");
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
    assert.match(await readFile(outputPath, "utf8"), /^schema_version = 1\n/);
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
    numeric.automation[0].segments[0].startSample = 0;
    failure(await run(["session", "build", "--request", "-", "--output", "-"], JSON.stringify(numeric)), 3, "request.shape");
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
