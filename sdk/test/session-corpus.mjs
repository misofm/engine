#!/usr/bin/env node
/** E3-E5 Session V1 corpus: SDK builder, four-stage CLI, and shipped-Wasm effect oracle. */

import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const root = resolve(import.meta.dirname, "..", "..");
const sdkRoot = resolve(root, "sdk");
const validator = process.argv[2];
const selfTest = process.argv[3] === "--self-test";

if (!validator || process.argv.length > 4 || (process.argv[3] && !selfTest)) {
  throw new Error("usage: node sdk/test/session-corpus.mjs VALIDATOR [--self-test]");
}

const SAMPLE_RATE = 48_000;
const WASM_ASSET = resolve(sdkRoot, "assets", "miso-engine-v2-audio-worklet.simd128.wasm");
const TAPS = [
  "input", "post_input_builtins", "post_simd1", "post_dynamic",
  "post_simd2_pre_fader", "post_fader", "post_matrix",
];

function run(command, args, options = {}) {
  return spawnSync(command, args, { encoding: "utf8", ...options });
}

async function compileSdk() {
  const out = await mkdtemp(resolve(tmpdir(), "miso-sdk-corpus-ts-"));
  const result = run(resolve(sdkRoot, "node_modules", ".bin", "tsc"), [
    "--project", "tsconfig.json", "--noEmit", "false", "--rootDir", "src", "--outDir", out,
  ], { cwd: sdkRoot });
  assert.equal(result.status, 0, `SDK corpus compilation failed:\n${result.stdout}${result.stderr}`);
  return { engine: await import(pathToFileURL(resolve(out, "index.js")).href), out };
}

function f32Bits(value) {
  const bytes = new ArrayBuffer(4);
  const view = new DataView(bytes);
  view.setFloat32(0, value, true);
  return view.getUint32(0, true);
}

function fromF32Bits(bits) {
  const bytes = new ArrayBuffer(4);
  const view = new DataView(bytes);
  view.setUint32(0, bits, true);
  return view.getFloat32(0, true);
}

function nextUp(value) {
  const rounded = Math.fround(value);
  if (rounded === Infinity) return rounded;
  if (Object.is(rounded, -0) || rounded === 0) return fromF32Bits(1);
  const bits = f32Bits(rounded);
  return fromF32Bits(rounded > 0 ? bits + 1 : bits - 1);
}

function nextDown(value) {
  const rounded = Math.fround(value);
  if (rounded === -Infinity) return rounded;
  if (Object.is(rounded, -0) || rounded === 0) return fromF32Bits(0x8000_0001);
  const bits = f32Bits(rounded);
  return fromF32Bits(rounded > 0 ? bits - 1 : bits + 1);
}

function floatToken(value) {
  const normalized = Math.fround(value);
  const bits = f32Bits(normalized);
  let text;
  for (let precision = 1; precision <= 9; precision += 1) {
    const candidate = normalized.toPrecision(precision);
    if (f32Bits(Number(candidate)) === bits) {
      text = candidate;
      break;
    }
  }
  assert.notEqual(text, undefined, "finite f32 must have a shortest decimal token");
  if (/e/i.test(text)) {
    const [coefficient, exponentText] = text.toLowerCase().split("e");
    const exponent = Number(exponentText);
    const negative = coefficient.startsWith("-");
    const digits = coefficient.replace("-", "").replace(".", "");
    const point = (coefficient.includes(".") ? coefficient.indexOf(".") : coefficient.length) + exponent;
    const unsigned = point <= 0
      ? `0.${"0".repeat(-point)}${digits}`
      : point >= digits.length
        ? `${digits}${"0".repeat(point - digits.length)}`
        : `${digits.slice(0, point)}.${digits.slice(point)}`;
    text = negative ? `-${unsigned}` : unsigned;
  }
  return text.includes(".") || /e/i.test(text) ? text : `${text}.0`;
}

function replaceExactlyOnce(source, pattern, replacement, label) {
  const flags = pattern.flags.includes("g") ? pattern.flags : `${pattern.flags}g`;
  const matches = source.match(new RegExp(pattern.source, flags));
  assert.equal(matches?.length, 1, `${label}: mutation anchor must occur exactly once`);
  return source.replace(pattern, replacement);
}

async function runCli(toml, directory, name) {
  const path = resolve(directory, `${name}.toml`);
  await writeFile(path, toml, "utf8");
  return run(validator, ["validate", "--canonical", path]);
}

function writePrepareConfig(exports, handle) {
  const pointer = exports.miso_engine_web_v1_config_ptr(handle);
  assert.notEqual(pointer, 0, "Wasm config pointer");
  const view = new DataView(exports.memory.buffer, pointer, 192);
  const u32 = [192, 0x0001_0000, SAMPLE_RATE, 128, 1 << 20, 1 << 14, 1 << 10, 8, 128, 256];
  const u64 = [
    1_024n, 1_024n, 4_096n, 8_192n, 64n << 20n, 64n << 20n, 16n << 20n,
    16n << 20n, 16n << 20n, 64n << 20n, 16n << 20n, 64n << 20n,
    1_024n, 1n << 20n, 16n << 20n,
  ];
  u32.forEach((value, index) => view.setUint32(index * 4, value, true));
  u64.forEach((value, index) => view.setBigUint64(40 + index * 8, value, true));
  for (let index = 0; index < 4; index += 1) view.setBigUint64(160 + index * 8, 0n, true);
}

function wasmCompile(module, toml) {
  const instance = new WebAssembly.Instance(module, {});
  const exports = instance.exports;
  const handle = exports.miso_engine_web_v1_config_new();
  assert.notEqual(handle, 0, "fresh Wasm handle");
  try {
    writePrepareConfig(exports, handle);
    assert.equal(exports.miso_engine_web_v1_prepare(handle), 0, "fresh Wasm prepare");
    const encoded = new TextEncoder().encode(toml);
    const capacity = exports.miso_engine_web_v1_buffer_capacity(handle, 1);
    assert.ok(encoded.byteLength <= capacity, `session TOML exceeds Wasm staging capacity: ${encoded.byteLength}/${capacity}`);
    const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, 1);
    new Uint8Array(exports.memory.buffer, pointer, encoded.byteLength).set(encoded);
    const result = exports.miso_engine_web_v1_compile(handle, encoded.byteLength);
    const diagnosticPointer = exports.miso_engine_web_v1_buffer_ptr(handle, 4);
    const diagnosticCapacity = exports.miso_engine_web_v1_buffer_capacity(handle, 4);
    const diagnostic = new TextDecoder()
      .decode(new Uint8Array(exports.memory.buffer, diagnosticPointer, diagnosticCapacity))
      .split("\0", 1)[0];
    return { result, diagnostic };
  } finally {
    assert.equal(exports.miso_engine_web_v1_dispose(handle), 0, "fresh Wasm dispose");
  }
}

function baseSession(engine, id) {
  return engine.session({ id, sampleRateHz: SAMPLE_RATE, limits: { pcmRingFrames: 128 } })
    .source("source", { channels: 2, frames: 128 });
}

function effectPlan(engine, id, effectId, parameterName, value) {
  return baseSession(engine, id)
    .track("track", {
      source: "source",
      dynamic: [engine.effect(effectId, { [parameterName]: value }, { slotId: "effect" })],
    })
    .build();
}

function builtinTrackSpec(name, value) {
  switch (name) {
    case "polarity_invert": return { source: "source", builtins: { polarityInvert: value } };
    case "trim_db": return { source: "source", builtins: { trimDb: value } };
    case "hpf_hz": return { source: "source", builtins: { hpfHz: value } };
    case "lpf_hz": return { source: "source", builtins: { lpfHz: value } };
    case "fader_db": return { source: "source", fader: { leftDb: value } };
    case "mute": return { source: "source", fader: { leftMute: value } };
    case "matrix_ll": return { source: "source", pan: { matrix: { ll: value, lr: 0, rl: 0, rr: 1 } } };
    case "matrix_lr": return { source: "source", pan: { matrix: { ll: 1, lr: value, rl: 0, rr: 1 } } };
    case "matrix_rl": return { source: "source", pan: { matrix: { ll: 1, lr: 0, rl: value, rr: 1 } } };
    case "matrix_rr": return { source: "source", pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: value } } };
    default: throw new Error(`unknown builtin ${name}`);
  }
}

function builtinPlan(engine, id, name, value) {
  return baseSession(engine, id).track("track", builtinTrackSpec(name, value)).build();
}

function builtinChoices(descriptor) {
  if (descriptor.domain === "booleanExact") return [false, true];
  if (descriptor.domain === "disabledOrRateKeyedHertz") {
    return [descriptor.disabledValue, descriptor.minimum, descriptor.maximumByRate[String(SAMPLE_RATE)]];
  }
  return [descriptor.minimum, descriptor.default, descriptor.maximum];
}

function effectChoices(parameter) {
  if (parameter.domainName === "boolean") return [false, true];
  if (parameter.domainName === "enumeration") return parameter.enumChoices.map((choice) => choice.label);
  return [parameter.minimum, parameter.default, parameter.maximum];
}

function effectInvalidCases(parameter) {
  if (parameter.domainName === "continuous") {
    return [
      { label: "below", valid: parameter.minimum, invalid: nextDown(parameter.minimum) },
      { label: "above", valid: parameter.maximum, invalid: nextUp(parameter.maximum) },
    ];
  }
  if (parameter.domainName === "boolean") {
    return [
      { label: "below", valid: false, invalid: nextDown(0), local: -1 },
      { label: "above", valid: true, invalid: nextUp(1), local: 2 },
    ];
  }
  const values = parameter.enumChoices.map((choice) => choice.value);
  return [
    { label: "below", valid: parameter.enumChoices[0].label, invalid: nextDown(Math.min(...values)), local: "invalid-below" },
    { label: "above", valid: parameter.enumChoices.at(-1).label, invalid: nextUp(Math.max(...values)), local: "invalid-above" },
  ];
}

function builtinInvalidCases(descriptor) {
  if (descriptor.domain === "booleanExact") {
    return [
      { label: "below", valid: false, invalid: -1 },
      { label: "above", valid: true, invalid: 2 },
    ];
  }
  const maximum = descriptor.domain === "disabledOrRateKeyedHertz"
    ? descriptor.maximumByRate[String(SAMPLE_RATE)]
    : descriptor.maximum;
  return [
    { label: "below", valid: descriptor.minimum, invalid: nextDown(descriptor.minimum) },
    { label: "above", valid: maximum, invalid: nextUp(maximum) },
  ];
}

function mutateEffectValue(toml, parameter, invalid, label) {
  const pattern = new RegExp(`(parameter_id = ${parameter.id}, channel = "both", unit = "${parameter.unitName}", value = )[^ },]+`);
  return replaceExactlyOnce(toml, pattern, `$1${floatToken(invalid)}`, label);
}

function builtinMutation(descriptor, invalid) {
  const token = descriptor.domain === "booleanExact" ? String(invalid) : floatToken(invalid);
  switch (descriptor.name) {
    case "polarity_invert": return [/(builtins = \{ left = \{ polarity_invert = )(?:false|true)/, `$1${token}`, "$.tracks[0].builtins.left.polarity_invert"];
    case "trim_db": return [/(builtins = \{ left = \{ polarity_invert = (?:false|true), trim_db = )[^ },]+/, `$1${token}`, "$.tracks[id=track].builtins.left.trim_db"];
    case "hpf_hz": return [/(builtins = \{ left = \{ polarity_invert = (?:false|true), trim_db = [^,]+, hpf_hz = )[^ },]+/, `$1${token}`, "$.tracks[id=track].builtins.left.hpf_hz"];
    case "lpf_hz": return [/(builtins = \{ left = \{ polarity_invert = (?:false|true), trim_db = [^,]+, hpf_hz = [^,]+, lpf_hz = )[^ },]+/, `$1${token}`, "$.tracks[id=track].builtins.left.lpf_hz"];
    case "fader_db": return [/left_db = [^ },]+/, `left_db = ${token}`, "$.tracks[id=track].fader.left_db"];
    case "mute": return [/left_mute = (?:false|true)/, `left_mute = ${token}`, "$.tracks[0].fader.left_mute"];
    case "matrix_ll": return [/(?<!channel_)matrix = \{ ll = [^ },]+/, `matrix = { ll = ${token}`, "$.tracks[id=track].matrix_or_pan.ll"];
    case "matrix_lr": return [/(?<!channel_)(matrix = \{ ll = [^,]+, lr = )[^ },]+/, `$1${token}`, "$.tracks[id=track].matrix_or_pan.lr"];
    case "matrix_rl": return [/(?<!channel_)(matrix = \{ ll = [^,]+, lr = [^,]+, rl = )[^ },]+/, `$1${token}`, "$.tracks[id=track].matrix_or_pan.rl"];
    case "matrix_rr": return [/(?<!channel_)(matrix = \{ ll = [^,]+, lr = [^,]+, rl = [^,]+, rr = )[^ },]+/, `$1${token}`, "$.tracks[id=track].matrix_or_pan.rr"];
    default: throw new Error(`unknown builtin ${descriptor.name}`);
  }
}

function assertLocalDomain(engine, make, descriptor, label) {
  assert.throws(
    make,
    (error) => error instanceof engine.MisoSessionError
      && error.descriptor?.id === descriptor.id
      && error.descriptor?.name === descriptor.name,
    `${label}: local error must carry the full generated descriptor`,
  );
}

function assertEffectDiagnostic(result, label) {
  assert.notEqual(result.result, 0, `${label}: full Wasm compile must reject`);
  const line = result.diagnostic.split("\n").find(Boolean);
  const expected = new Set([
    "effect.parameter.domain\t$.tracks[id=track].effects[id=effect]",
    "numeric.out_of_schema_range\t$.tracks[0].dynamic.effects[0].params[0].value",
  ]);
  assert.ok(expected.has(line), `${label}: typed full-pipeline diagnostic leaf: ${line}`);
  return line;
}

async function buildGreenCorpus(engine) {
  const documents = [];
  for (const effect of engine.CATALOG.effects) {
    for (const parameter of effect.parameters) {
      for (const [index, value] of effectChoices(parameter).entries()) {
        documents.push({
          name: `effect-${effect.id}-${parameter.id}-${index}`.replaceAll(".", "-"),
          plan: effectPlan(engine, `e${documents.length}`, effect.id, parameter.name, value),
        });
      }
    }
  }
  for (const descriptor of engine.CATALOG.builtins.parameters) {
    for (const [index, value] of builtinChoices(descriptor).entries()) {
      documents.push({
        name: `builtin-${descriptor.name}-${index}`,
        plan: builtinPlan(engine, `b${documents.length}`, descriptor.name, value),
      });
    }
  }
  for (const tap of TAPS) {
    const builder = baseSession(engine, `tap-${documents.length}`)
      .track("track", { source: "source" })
      .submix("bus")
      .output("out")
      .route({ id: "to-bus", source: { kind: "track", trackId: "track", tap }, destination: { kind: "submix_input", submixId: "bus" } })
      .route({ id: "to-out", source: { kind: "submix_output", submixId: "bus" }, destination: { kind: "output_input", outputId: "out" } });
    documents.push({ name: `tap-${tap}`, plan: builder.build() });
  }
  for (const shape of ["step", "linear", "exponential"]) {
    const plan = baseSession(engine, `auto-${shape}`)
      .track("track", { source: "source", dynamic: [engine.effect("miso.compressor", { ratio: 2 }, { slotId: "comp" })] })
      .automate({
        id: "auto", target: { trackId: "track", rack: "dynamic", slotId: "comp", parameter: "ratio", channel: "both" },
        segments: [{ shape, startSample: 0n, endSample: 128n, startValue: 1, endValue: 2 }],
      })
      .build();
    documents.push({ name: `automation-${shape}`, plan });
  }
  for (const count of [1, 2, 64]) {
    let builder = baseSession(engine, `tracks-${count}`);
    for (let index = 0; index < count; index += 1) builder = builder.track(`t${index}`, { source: "source" });
    documents.push({ name: `tracks-${count}`, plan: builder.build() });
  }
  assert.ok(documents.length >= 40, `E3 requires >=40 documents, got ${documents.length}`);
  return documents;
}

async function check() {
  const { engine, out } = await compileSdk();
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-corpus-"));
  try {
    const documents = await buildGreenCorpus(engine);
    for (const document of documents) {
      const result = await runCli(document.plan.toml, directory, document.name);
      assert.equal(result.status, 0, `${document.name}: E3 failed:\n${result.stderr}`);
      assert.equal(result.stdout, document.plan.toml, `${document.name}: E4 canonical drift`);
    }

    let builtinNegativeCases = 0;
    for (const descriptor of engine.CATALOG.builtins.parameters) {
      for (const item of builtinInvalidCases(descriptor)) {
        const label = `builtin-${descriptor.name}-${item.label}`;
        assertLocalDomain(engine, () => builtinPlan(engine, `local-${descriptor.id}`, descriptor.name, item.invalid), descriptor, label);
        const valid = builtinPlan(engine, `forced-${descriptor.id}`, descriptor.name, item.valid);
        const [pattern, replacement, path] = builtinMutation(descriptor, item.invalid);
        const forced = replaceExactlyOnce(valid.toml, pattern, replacement, label);
        const result = await runCli(forced, directory, label);
        assert.notEqual(result.status, 0, `${label}: E5a CLI must reject`);
        assert.match(result.stderr, /(?:numeric|builtin|schema)\.[a-z0-9_.]+\s+\$\./, `${label}: E5a dotted diagnostic`);
        assert.ok(result.stderr.includes(path), `${label}: E5a diagnostic leaf ${path}\n${result.stderr}`);
        builtinNegativeCases += 1;
      }
    }

    const wasmModule = await WebAssembly.compile(await readFile(WASM_ASSET));
    let effectNegativeCases = 0;
    let effectPrepareCases = 0;
    let schemaEnvelopeCases = 0;
    let ratioAsymmetry = false;
    for (const effect of engine.CATALOG.effects) {
      for (const parameter of effect.parameters) {
        for (const item of effectInvalidCases(parameter)) {
          const label = `effect-${effect.id}-${parameter.name}-${item.label}`;
          const localValue = item.local ?? item.invalid;
          assertLocalDomain(engine, () => engine.effect(effect.id, { [parameter.name]: localValue }), parameter, label);
          const valid = effectPlan(engine, `forced-${effect.contractMajor}-${parameter.id}`, effect.id, parameter.name, item.valid);
          const forced = mutateEffectValue(valid.toml, parameter, item.invalid, label);
          const diagnostic = assertEffectDiagnostic(wasmCompile(wasmModule, forced), label);
          if (diagnostic.startsWith("effect.parameter.domain\t")) effectPrepareCases += 1;
          else schemaEnvelopeCases += 1;
          if (effect.id === "miso.compressor" && parameter.name === "ratio" && item.label === "above") {
            assert.equal(floatToken(item.invalid), "20.000002", "ratio witness is the next f32 above 20");
            assert.equal(diagnostic, "effect.parameter.domain\t$.tracks[id=track].effects[id=effect]", "ratio E5b effect diagnostic");
            const cliResult = await runCli(forced, directory, "ratio-20-000002");
            assert.equal(cliResult.status, 0, `ratio asymmetry must PASS E5a CLI:\n${cliResult.stderr}`);
            assert.equal(cliResult.stdout, forced, "ratio asymmetry remains CLI-canonical");
            ratioAsymmetry = true;
          }
          effectNegativeCases += 1;
        }
      }
    }
    assert.equal(ratioAsymmetry, true, "E5 must exercise the approved ratio asymmetry");

    if (selfTest) {
      const original = documents[0].plan.toml;
      const e3Mutation = original.replace("schema_version = 1", "schema_version = 2");
      const e3Result = await runCli(e3Mutation, directory, "red-e3");
      assert.notEqual(e3Result.status, 0, "E3 deliberate schema mutation must fail the CLI");

      const e4Mutation = `# deliberate noncanonical comment\n${original}`;
      const e4Result = await runCli(e4Mutation, directory, "red-e4");
      assert.equal(e4Result.status, 0, `E4 mutation must remain valid:\n${e4Result.stderr}`);
      assert.throws(
        () => assert.equal(e4Result.stdout, e4Mutation, "canonical bytes"),
        assert.AssertionError,
        "E4 deliberate noncanonical mutation must fail byte equality",
      );

      const ratio = nextUp(20);
      const compressor = engine.CATALOG.effects.find((effect) => effect.id === "miso.compressor");
      const ratioDescriptor = compressor.parameters.find((parameter) => parameter.name === "ratio");
      assertLocalDomain(engine, () => engine.effect("miso.compressor", { ratio }), ratioDescriptor, "red-e5");
    }

    const parameterCount = engine.CATALOG.effects.reduce((sum, effect) => sum + effect.parameters.length, 0);
    console.log(
      `SDK Session corpus passed: documents=${documents.length}; effect-parameters=${parameterCount}; `
      + `builtin-negative-cases=${builtinNegativeCases}; effect-negative-cases=${effectNegativeCases}; `
      + `effect-prepare-cases=${effectPrepareCases}; schema-envelope-cases=${schemaEnvelopeCases}; ratio-asymmetry=PASS`,
    );
  } finally {
    await rm(directory, { recursive: true, force: true });
    await rm(out, { recursive: true, force: true });
  }
}

await check();
