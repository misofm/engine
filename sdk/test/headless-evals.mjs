#!/usr/bin/env node
/** Phase 2 E6-E10a: real Node/Bun Wasm host, native WAV oracle, console timing/refusals, hash order. */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = resolve(import.meta.dirname, "..", "..");
const wasmPath = resolve(repoRoot, "sdk/assets/miso-engine-v2-audio-worklet.simd128.wasm");
const fixtures = resolve(repoRoot, "fixtures/native-pcm-runner/v1");
const expectedNative = Object.freeze({
  48_000: "cef2b4282bb8478687b4dec5f764a9f04bc64fc7a35d3a8edd5b398a80494771",
  96_000: "dcb0de625cb09c064ea424dff6b1eca01896ba1e7ee602c72dc7454ad9b74f16",
});

function digest(bytes) { return createHash("sha256").update(bytes).digest("hex"); }

function deterministicPlanes(frames, phase) {
  const left = new Float32Array(frames), right = new Float32Array(frames);
  for (let index = 0; index < frames; index += 1) {
    left[index] = Math.fround(Math.sin((index + phase) * 0.03125) * 0.4);
    right[index] = Math.fround(Math.cos((index + phase) * 0.0234375) * 0.3);
  }
  return [left, right];
}

function e6Fixture(sdk) {
  const frames = 4_096;
  const first = deterministicPlanes(frames, 3), second = deterministicPlanes(frames, 19);
  const compressor = sdk.effect("miso.compressor", { threshold: -24, ratio: 4 }, { slotId: "comp" });
  const equalizer = sdk.effect("miso.parametric-eq", { "band-1-enabled": true, "band-1-kind": "bell", "band-1-frequency": 1_200, "band-1-gain": -3 }, { slotId: "eq" });
  const limiter = sdk.effect("miso.true-peak-limiter", { ceiling: -1, release: 80 }, { slotId: "limit" });
  const plan = sdk.session({ id: "sdk-e6", sampleRateHz: 48_000 })
    .source("alpha", { channels: 2, frames })
    .source("beta", { channels: 2, frames })
    .track("alpha-track", { source: "alpha", pan: { left: -1, right: 1 }, dynamic: [compressor, equalizer, limiter] })
    .track("beta-track", { source: "beta", pan: { left: -1, right: 1 } })
    .automate({
      id: "compressor-threshold",
      target: { trackId: "alpha-track", rack: "dynamic", slotId: "comp", parameter: "threshold", channel: "both" },
      segments: [{ shape: "linear", startSample: 0n, endSample: 512n, startValue: -24, endValue: -18 }],
    })
    .build();
  return Object.freeze({
    plan,
    sources: Object.freeze({
      alpha: Object.freeze({ wav: sdk.wav32fBytes(first[0], first[1], 48_000) }),
      beta: Object.freeze({ wav: sdk.wav32fBytes(second[0], second[1], 48_000) }),
    }),
  });
}

async function e6Digest(sdk, wasm) {
  const fixture = e6Fixture(sdk);
  const engine = await sdk.createOfflineEngine({ session: fixture.plan, sources: fixture.sources, wasm: { bytes: wasm } });
  try {
    const audio = engine.renderAll();
    assert.ok(audio.left.some((value) => value !== 0) || audio.right.some((value) => value !== 0), "E6 fixture must render non-silent evidence");
    return digest(sdk.f32lePlanarBytes(audio.left, audio.right, 128));
  } finally { engine.dispose(); }
}

function assertE6(values) {
  assert.equal(new Set(values).size, 1, `E6 digest mismatch: ${values.join(", ")}`);
}

function assertE7(nativeBytes, sdkBytes, expected, rate) {
  assert.deepEqual(sdkBytes, nativeBytes, `E7 ${rate} Hz SDK/native bytes differ`);
  assert.equal(digest(sdkBytes), expected, `E7 ${rate} Hz digest differs from the pinned manifest`);
}

function e7Plan(sdk, rate) {
  let builder = sdk.session({
    id: "parametric-eq-nine-track", sampleRateHz: rate, revision: 42, quantumFrames: 128,
    limits: { pcmRingFrames: 1_024, controlQueueMessages: 64, memoryBytes: 16_777_216 },
  }).source("fixture-source", { channels: 2, frames: 1_024, sampleRateHz: rate });
  const builtins = { polarityInvert: false, trimDb: 0, hpfHz: 20, lpfHz: 20_000 };
  for (let index = 0; index < 9; index += 1) {
    const equalizer = index === 0
      ? sdk.effect("miso.parametric-eq", {
          "band-1-enabled": true, "band-1-kind": "bell", "band-1-frequency": [120, 2_400],
          "band-1-gain": [6, -9], "band-1-q": 0.70710677, "band-1-shelf-slope": 1,
        }, { slotId: "eq" })
      : sdk.effect("miso.parametric-eq", {}, { slotId: "eq" });
    builder = builder.track(`eq${index}`, {
      source: "fixture-source", builtins, simd1: [equalizer], pan: { left: 1, right: 1, smoothingSamples: 16 },
    });
  }
  builder = builder.output("main-out");
  for (let index = 0; index < 9; index += 1) {
    builder = builder.route({
      id: `eq${index}-main`, source: { kind: "track", trackId: `eq${index}`, tap: "post_matrix" },
      destination: { kind: "output_input", outputId: "main-out" },
    });
  }
  return builder.build();
}

function assertE8(firstAffected, appliedAtSample, expected) {
  assert.equal(firstAffected, expected, "E8 first changed sample is not the next block boundary");
  assert.equal(appliedAtSample, BigInt(expected), "E8 acknowledgement names the wrong sample");
}

function assertE9(actual) {
  const expected = ["backpressure", "unknownTrack", "unknownParameter", "unknownTap", "observationUnbound", "none"];
  assert.deepEqual(actual, expected, "E9 typed refusal sequence drifted");
}

function assertE10(error, compileCalls, expected, actual) {
  assert.equal(error?.code, "miso.asset.hash-mismatch.v1", "E10a requires the typed hash error");
  assert.equal(error?.asset, "miso-engine-v2-audio-worklet.simd128.wasm");
  assert.equal(error?.expectedSha256, expected);
  assert.equal(error?.actualSha256, actual);
  assert.equal(compileCalls, 0, "E10a must reject before Wasm compilation/instantiation");
}

async function freshDigest(dist) {
  const result = spawnSync(process.execPath, [import.meta.filename, "--fresh-worker", dist], { cwd: repoRoot, encoding: "utf8" });
  assert.equal(result.status, 0, `E6 fresh process failed:\n${result.stdout}${result.stderr}`);
  return result.stdout.trim();
}

async function runE6(sdk, wasm, dist) {
  const first = await e6Digest(sdk, wasm), second = await e6Digest(sdk, wasm), fresh = await freshDigest(dist);
  assertE6([first, second, fresh]);
  return first;
}

async function runE7(sdk, wasm, nativeRunner) {
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-e7-"));
  const evidence = {};
  try {
    for (const rate of [48_000, 96_000]) {
      const stem = `riff-${rate}`;
      const sourcePath = resolve(fixtures, `${stem}.wav`);
      const nativeOutput = resolve(directory, `${stem}.native.f32le`);
      const sdkOutput = resolve(directory, `${stem}.sdk.f32le`);
      const native = spawnSync(nativeRunner, [
        "--session", resolve(fixtures, `${stem}.toml`), "--source-root", fixtures,
        "--frames", "1024", "--output", nativeOutput,
      ], { cwd: repoRoot, encoding: "utf8" });
      assert.equal(native.status, 0, `E7 native runner failed at ${rate} Hz:\n${native.stdout}${native.stderr}`);
      const memoryWave = sdk.parseWave(await readFile(sourcePath), `${stem}-memory`);
      const pathWave = sdk.openWaveFile(sourcePath, `${stem}-path`);
      try {
        assert.equal(pathWave.frames, memoryWave.frames);
        assert.equal(pathWave.sampleRateHz, memoryWave.sampleRateHz);
        assert.deepEqual(pathWave.decode(3, 17), sdk.decodeWave(memoryWave, 3, 17), "path WAV bounded decode must equal in-memory decode");
      } finally { pathWave.close(); }
      assert.throws(() => pathWave.decode(0, 1), (error) => error?.code === "miso.source.v1" && error.path === "wav.lifecycle");
      const toml = await readFile(resolve(fixtures, `${stem}.toml`), "utf8");
      const rawOutput = resolve(directory, `${stem}.raw-sdk.f32le`);
      const raw = await sdk.createOfflineEngine({
        session: { toml },
        sources: { "fixture-source": { wav: sourcePath } },
        wasm: { bytes: wasm },
      });
      try {
        const map = await raw.console.sessionMap();
        assert.equal(map.sources.length, 1, "raw TOML must discover the compiled source map");
        assert.deepEqual(map.sources[0], {
          id: "fixture-source", channels: 2, sampleRateHz: rate, startFrame: 0n, frames: 1_024n,
        });
        await raw.renderToFile(rawOutput, { format: "f32le-planar" });
      } finally { raw.dispose(); }
      const engine = await sdk.createOfflineEngine({ session: e7Plan(sdk, rate), sources: { "fixture-source": { wav: sourcePath } }, wasm: { bytes: wasm } });
      let maximumRenderRequest = 0;
      const render = engine.render.bind(engine);
      engine.render = (frames) => { maximumRenderRequest = Math.max(maximumRenderRequest, frames); return render(frames); };
      engine.renderAll = () => { throw new Error("renderToFile retained the whole output"); };
      let report;
      try {
        report = await engine.renderToFile(sdkOutput, { format: "f32le-planar" });
        assert.ok(maximumRenderRequest > 0 && maximumRenderRequest <= 128, "file output must render in quantum-bounded blocks");
        const preserved = await readFile(sdkOutput);
        await assert.rejects(engine.renderToFile(sdkOutput, { format: "f32le-planar" }), (error) => error?.code === "miso.offline.v1" && error.phase === "output");
        assert.deepEqual(await readFile(sdkOutput), preserved, "overwrite refusal must preserve the existing output");
      } finally { engine.dispose(); }
      const nativeBytes = await readFile(nativeOutput), sdkBytes = await readFile(sdkOutput);
      assertE7(nativeBytes, sdkBytes, expectedNative[rate], rate);
      assert.deepEqual(await readFile(rawOutput), nativeBytes, `E7 ${rate} Hz raw-TOML bytes differ from native`);
      assert.equal(report.frames, 1_024);
      assert.equal(report.bytes, sdkBytes.byteLength);
      assert.equal(report.sha256, digest(sdkBytes));
      evidence[rate] = digest(sdkBytes);
    }
    const rf64Path = resolve(fixtures, "rf64-48000.wav");
    const rf64Memory = sdk.parseWave(await readFile(rf64Path), "rf64-memory");
    const rf64File = sdk.openWaveFile(rf64Path, "rf64-path");
    try { assert.deepEqual(rf64File.decode(1, 17), sdk.decodeWave(rf64Memory, 1, 17), "RF64 path and memory decoding must agree"); }
    finally { rf64File.close(); }
    const rf64NativeOutput = resolve(directory, "rf64.native.f32le");
    const rf64Native = spawnSync(nativeRunner, [
      "--session", resolve(fixtures, "rf64-48000.toml"), "--source-root", fixtures,
      "--frames", "512", "--output", rf64NativeOutput,
    ], { cwd: repoRoot, encoding: "utf8" });
    assert.equal(rf64Native.status, 0, `RF64 native runner failed:\n${rf64Native.stdout}${rf64Native.stderr}`);
    const rf64Raw = await sdk.createOfflineEngine({
      session: { toml: await readFile(resolve(fixtures, "rf64-48000.toml"), "utf8") },
      sources: { "fixture-source": { wav: rf64Path } },
      wasm: { bytes: wasm },
    });
    try {
      const map = await rf64Raw.console.sessionMap();
      assert.deepEqual(map.sources[0], {
        id: "fixture-source", channels: 2, sampleRateHz: 48_000, startFrame: 1n, frames: 514n,
      }, "raw RF64 source shape must come from the compiled engine session");
      const audio = rf64Raw.render(512);
      assert.deepEqual(
        sdk.f32lePlanarBytes(audio.left, audio.right, 128),
        new Uint8Array(await readFile(rf64NativeOutput)),
        "raw RF64 nonzero-region output must be byte-identical to native",
      );
    } finally { rf64Raw.dispose(); }
  } finally { await rm(directory, { recursive: true, force: true }); }
  return Object.freeze(evidence);
}

function concatenate(...arrays) {
  const result = new Float32Array(arrays.reduce((size, array) => size + array.length, 0));
  let offset = 0;
  for (const array of arrays) { result.set(array, offset); offset += array.length; }
  return result;
}

async function runE8(sdk, wasm) {
  const frames = 512, planes = [new Float32Array(frames).fill(0.25), new Float32Array(frames).fill(-0.25)];
  const plan = sdk.session({ id: "sdk-e8", sampleRateHz: 48_000 }).source("tone", { channels: 2, frames })
    .track("track", { source: "tone", pan: { left: -1, right: 1 } }).build();
  const baseline = await sdk.createOfflineEngine({ session: plan, sources: { tone: planes }, wasm: { bytes: wasm } });
  const commanded = await sdk.createOfflineEngine({ session: plan, sources: { tone: planes }, wasm: { bytes: wasm } });
  try {
    const expected = baseline.render(384).left;
    const before = commanded.render(256).left;
    const ack = await commanded.console.track("track").fader(-6);
    const after = commanded.render(128).left;
    const actual = concatenate(before, after);
    const firstAffected = actual.findIndex((value, index) => Object.is(value, expected[index]) === false);
    assertE8(firstAffected, ack.appliedAtSample, 256);
    assert.equal(ack.raw.appliedAtSample, ack.appliedAtSample, "E8 raw and typed acknowledgements must agree");
    return Object.freeze({ firstAffected, appliedAtSample: ack.appliedAtSample });
  } finally { baseline.dispose(); commanded.dispose(); }
}

async function runE9(sdk, wasm) {
  const frames = 256, planes = [new Float32Array(frames).fill(0.5), new Float32Array(frames).fill(0.5)];
  const compressor = sdk.effect("miso.compressor", { threshold: -18 }, { slotId: "comp" });
  const plan = sdk.session({ id: "sdk-e9", sampleRateHz: 48_000 }).source("tone", { channels: 2, frames }).track("track", { source: "tone", dynamic: [compressor] }).build();
  const engine = await sdk.createOfflineEngine({
    session: plan, sources: { tone: planes }, wasm: { bytes: wasm },
    limits: { consoleCommandQueueRecords: 1n, consoleObservationTaps: 0n },
  });
  try {
    const raw = (trackIndex, value) => ({ kind: 3, rack: 255, channel: 2, trackIndex, effectIndex: 0, parameterId: 0, smoothingSamples: 0, values: [value, 0, 0, 0] });
    const overfill = await engine.console.submit([raw(0, -3), raw(0, -6)]);
    const unknownTrack = await engine.console.setParam({ trackId: "missing", rack: "dynamic", effectIndex: 0, parameter: "threshold" }, -24);
    const unknownParameter = await engine.console.setParam({ trackId: "track", rack: "dynamic", effectIndex: 0, parameter: "missing" }, -24);
    const unknownTap = (await engine.console.observe({ trackId: "track", rack: "dynamic", effectIndex: 0, tap: "missing" })).ack;
    const unbound = (await engine.console.observe({ trackId: "track", rack: "dynamic", effectIndex: 0, tap: "Gain Reduction" })).ack;
    const valid = await engine.console.track("track").fader(-6);
    const reasons = [overfill.reason, unknownTrack.reason, unknownParameter.reason, unknownTap.reason, unbound.reason, valid.reason];
    assert.equal(overfill.admitted, 0, "E9 backpressure must admit nothing");
    assert.equal(valid.ok, true, "E9 valid command must succeed after refusals");
    assertE9(reasons);
    return reasons;
  } finally { engine.dispose(); }
}

async function runE10(sdk, wasm) {
  const bad = new Uint8Array(wasm); bad[bad.length - 1] ^= 1;
  const expected = digest(wasm), actual = digest(bad);
  const original = WebAssembly.compile;
  let compileCalls = 0;
  WebAssembly.compile = async (...args) => { compileCalls += 1; return original(...args); };
  let caught;
  try {
    await sdk.createOfflineEngine({ session: sdk.session({ id: "sdk-e10", sampleRateHz: 48_000 }).build(), sources: {}, wasm: { bytes: bad } });
  } catch (error) { caught = error; }
  finally { WebAssembly.compile = original; }
  assertE10(caught, compileCalls, expected, actual);
  return Object.freeze({ expected, actual });
}

async function checkValidation(sdk, wasm) {
  const plan = sdk.session({ id: "sdk-validation", sampleRateHz: 48_000 })
    .source("source", { channels: 1, frames: 128 })
    .track("track", { source: "source" })
    .build();
  const good = await sdk.validateSession(plan.toml, { bytes: wasm });
  const badToml = plan.toml.replace("schema_version = 1", "schema_version = 2");
  const first = await sdk.validateSession(badToml, { bytes: wasm });
  const second = await sdk.validateSession(badToml, { bytes: wasm });
  assert.equal(good.ok, true);
  assert.equal(first.ok, false);
  assert.deepEqual(second, first, "validateSession must use a fresh deterministic instance per call");
  assert.ok(first.diagnostics.length > 0 && first.diagnostics.every((item) => item.code && item.path), "validateSession diagnostics must be typed code/path rows");
}

function pcmWave(bits, values) {
  const sampleBytes = bits / 8, dataBytes = values.length * sampleBytes, padding = dataBytes & 1;
  const bytes = new Uint8Array(44 + dataBytes + padding), view = new DataView(bytes.buffer);
  const put = (offset, value) => bytes.set(new TextEncoder().encode(value), offset);
  put(0, "RIFF"); view.setUint32(4, 36 + dataBytes + padding, true); put(8, "WAVE"); put(12, "fmt ");
  view.setUint32(16, 16, true); view.setUint16(20, 1, true); view.setUint16(22, 1, true);
  view.setUint32(24, 48_000, true); view.setUint32(28, 48_000 * sampleBytes, true);
  view.setUint16(32, sampleBytes, true); view.setUint16(34, bits, true); put(36, "data"); view.setUint32(40, dataBytes, true);
  values.forEach((value, index) => {
    const offset = 44 + index * sampleBytes;
    if (bits === 16) view.setInt16(offset, value, true);
    else { const encoded = value < 0 ? value + 0x1_000000 : value; view.setUint8(offset, encoded); view.setUint8(offset + 1, encoded >>> 8); view.setUint8(offset + 2, encoded >>> 16); }
  });
  return bytes;
}

async function checkEdges(sdk, wasm) {
  const pcm16 = sdk.decodeWave(sdk.parseWave(pcmWave(16, [-32_768, 0, 32_767]), "pcm16"), 0, 3, "pcm16")[0];
  const pcm24 = sdk.decodeWave(sdk.parseWave(pcmWave(24, [-8_388_608, 0, 8_388_607]), "pcm24"), 0, 3, "pcm24")[0];
  assert.deepEqual(Array.from(pcm16), [-1, 0, Math.fround(32_767 / 32_768)]);
  assert.deepEqual(Array.from(pcm24), [-1, 0, Math.fround(8_388_607 / 8_388_608)]);
  const malformed = pcmWave(16, [0]); new DataView(malformed.buffer).setUint32(4, 0, true);
  assert.throws(() => sdk.parseWave(malformed, "malformed"), (error) => error?.code === "miso.source.v1" && error.path === "wav.riff");
  const rf64 = new Uint8Array(await readFile(resolve(fixtures, "rf64-48000.wav")));
  const zeroSampleCount = new Uint8Array(rf64); new DataView(zeroSampleCount.buffer).setBigUint64(36, 0n, true);
  assert.equal(sdk.parseWave(zeroSampleCount, "rf64-zero-count").frames, 516, "RF64 zero sample-count means unspecified");
  const wrongRoot = new Uint8Array(rf64); new DataView(wrongRoot.buffer).setUint32(4, 0, true);
  assert.throws(() => sdk.parseWave(wrongRoot, "rf64-root"), (error) => error?.code === "miso.source.v1" && error.path === "wav.riff");
  const cleanRiff = sdk.wav32fBytes(new Float32Array(1), new Float32Array(1), 48_000);
  const trailingRiff = new Uint8Array(cleanRiff.byteLength + 1); trailingRiff.set(cleanRiff); new DataView(trailingRiff.buffer).setUint32(4, trailingRiff.byteLength - 8, true);
  assert.throws(() => sdk.parseWave(trailingRiff, "riff-trailing"), (error) => error?.code === "miso.source.v1" && error.path === "wav.chunks");
  const malformedDirectory = await mkdtemp(resolve(tmpdir(), "miso-sdk-wav-red-"));
  try {
    const zeroPath = resolve(malformedDirectory, "rf64-zero-count.wav");
    const rootPath = resolve(malformedDirectory, "rf64-root.wav");
    const trailingPath = resolve(malformedDirectory, "riff-trailing.wav");
    await Promise.all([writeFile(zeroPath, zeroSampleCount), writeFile(rootPath, wrongRoot), writeFile(trailingPath, trailingRiff)]);
    const zeroFile = sdk.openWaveFile(zeroPath, "rf64-zero-path");
    try { assert.equal(zeroFile.frames, 516); } finally { zeroFile.close(); }
    assert.throws(() => sdk.openWaveFile(rootPath, "rf64-root-path"), (error) => error?.code === "miso.source.v1" && error.path === "wav.riff");
    assert.throws(() => sdk.openWaveFile(trailingPath, "riff-trailing-path"), (error) => error?.code === "miso.source.v1" && error.path === "wav.chunks");
  } finally { await rm(malformedDirectory, { recursive: true, force: true }); }
  const frames = 256, planes = deterministicPlanes(frames, 7);
  const plan = sdk.session({ id: "sdk-partial", sampleRateHz: 48_000 }).source("source", { channels: 2, frames }).track("track", { source: "source", pan: { left: -1, right: 1 } }).build();
  const whole = await sdk.createOfflineEngine({ session: plan, sources: { source: planes }, wasm: { bytes: wasm } });
  const split = await sdk.createOfflineEngine({ session: plan, sources: { source: planes }, wasm: { bytes: wasm } });
  try {
    assert.throws(() => split.console.track("track").fader(-145), (error) => error?.code === "miso.command.v1" && error.path === "fader.db");
    assert.throws(() => split.console.track("track").pan(-1.01, 1), (error) => error?.code === "miso.command.v1" && error.path === "pan.left");
    const expected = whole.render(128), first = split.render(17), second = split.render(111);
    assert.deepEqual(concatenate(first.left, second.left), expected.left, "partial render calls must retain an exact quantum tail");
    assert.deepEqual(concatenate(first.right, second.right), expected.right, "partial render calls must retain an exact quantum tail");
  } finally { whole.dispose(); split.dispose(); }
  const mismatched = sdk.wav32fBytes(planes[0], planes[1], 44_100);
  const compile = WebAssembly.compile;
  let compileCalls = 0;
  WebAssembly.compile = async (...args) => { compileCalls += 1; return compile(...args); };
  try {
    await assert.rejects(
      sdk.createOfflineEngine({ session: plan, sources: { source: { wav: mismatched } }, wasm: { bytes: wasm } }),
      (error) => error?.code === "miso.source.v1" && error.path === "sample_rate_hz",
      "source/session rate mismatch must reject before render",
    );
  } finally { WebAssembly.compile = compile; }
  assert.equal(compileCalls, 0, "source/session rate mismatch must reject before WebAssembly compilation");
}

function redMutations(evidence) {
  assert.throws(() => assertE6([evidence.e6, `${evidence.e6.slice(0, -1)}0`, evidence.e6]), /E6 digest mismatch/, "E6 digest mutation must turn the gate red");
  const native = new Uint8Array([1, 2, 3]), changed = new Uint8Array(native); changed[1] ^= 1;
  assert.throws(() => assertE7(native, changed, digest(native), 48_000), /E7 48000 Hz SDK\/native bytes differ/, "E7 output mutation must turn the gate red");
  assert.throws(() => assertE8(evidence.e8.firstAffected, evidence.e8.appliedAtSample + 1n, 256), /acknowledgement/, "E8 ack mutation must turn the gate red");
  assert.throws(() => assertE9(["backpressure", "unknownTrack", "domain", "unknownTap", "observationUnbound", "none"]), /E9 typed refusal/, "E9 reason mutation must turn the gate red");
  assert.throws(() => assertE10({ code: "miso.asset.hash-mismatch.v1", asset: "miso-engine-v2-audio-worklet.simd128.wasm", expectedSha256: evidence.e10.expected, actualSha256: evidence.e10.actual }, 1, evidence.e10.expected, evidence.e10.actual), /before Wasm/, "E10a ordering mutation must turn the gate red");
}

async function worker(dist) {
  const sdk = await import(pathToFileURL(resolve(dist, "headless/index.js")).href);
  const wasm = await readFile(wasmPath);
  process.stdout.write(await e6Digest(sdk, wasm));
}

async function main() {
  if (process.argv[2] === "--fresh-worker") return worker(process.argv[3]);
  const [nativeRunner, dist, mode] = process.argv.slice(2);
  assert.ok(nativeRunner && dist && (mode === undefined || mode === "--self-test"), "usage: headless-evals.mjs NATIVE_RUNNER DIST [--self-test]");
  const sdk = await import(pathToFileURL(resolve(dist, "headless/index.js")).href);
  const wasm = await readFile(wasmPath);
  await checkValidation(sdk, wasm);
  await checkEdges(sdk, wasm);
  const evidence = {
    e6: await runE6(sdk, wasm, dist),
    e7: await runE7(sdk, wasm, nativeRunner),
    e8: await runE8(sdk, wasm),
    e9: await runE9(sdk, wasm),
    e10: await runE10(sdk, wasm),
  };
  if (mode === "--self-test") redMutations(evidence);
  const runtime = globalThis.Bun ? `bun-${globalThis.Bun.version}` : `node-${process.versions.node}`;
  console.log(`headless E6-E10a PASS runtime=${runtime} e6=${evidence.e6} e7-48=${evidence.e7[48_000]} e7-96=${evidence.e7[96_000]} e8=${evidence.e8.firstAffected} e9=${evidence.e9.join(",")} e10a=preinstantiate${mode ? " red-mutations=5/5" : ""}`);
}

await main();
