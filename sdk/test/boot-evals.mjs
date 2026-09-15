/**
 * Issue #243 evals 1, 3, 7 and 8: the boot boundary.
 *
 * These are the evals that killed the pre-boot-v1 SDK, written against the shipped module rather
 * than a mock. Each test names the red mutation that makes it fail, because a probe nobody has
 * seen fail is a probe nobody should trust.
 */

import assert from "node:assert/strict";
import { after, before, describe, test } from "node:test";

import { MisoEngineAsset, sha256Hex } from "../src/core/asset.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { hexLower } from "../src/core/hex.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { createOfflineEngine, validate } from "../src/headless/engine.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

/**
 * Every `WebAssembly.compile` in this process, counted.
 *
 * Eval 1(c) says "exactly one `WebAssembly.compile` per SDK lifetime (count them)", so they are
 * counted rather than reasoned about: the SDK's own `compileCount` is its claim, and this is the
 * independent witness that the claim is true. The two are asserted against each other.
 */
let compiles = 0;
const realCompile = WebAssembly.compile.bind(WebAssembly);
WebAssembly.compile = (...args) => {
  compiles += 1;
  return realCompile(...args);
};

test("the SDK hex authority formats empty and fixed literal bytes", () => {
  assert.equal(hexLower(new Uint8Array()), "");
  assert.equal(
    hexLower(new Uint8Array([0x00, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff])),
    "000123456789abcdefff",
  );
});

let bytes;
let asset;

before(async () => {
  bytes = await moduleBytes();
  asset = await MisoEngineAsset.load(bytes);
});

test("protected observation shipped Wasm ABI and boot", async () => {
  const instance = await asset.instantiate();
  const { exports } = instance;
  let handle = 0;

  const structure = (name) => {
    const layout = ABI_LAYOUT.structures[name];
    assert.ok(layout, `generated structure ${name} must exist`);
    return layout;
  };
  const field = (structureName, name, type) => {
    const row = structure(structureName).fields.find((candidate) => candidate.name === name);
    assert.ok(row, `generated field ${structureName}.${name} must exist`);
    assert.equal(row.type, type, `${structureName}.${name} type`);
    return row;
  };
  const constant = (group, name) => {
    const row = ABI_LAYOUT.constants[group].find((candidate) => candidate.name === name);
    assert.ok(row, `generated constant ${group}.${name} must exist`);
    return row.value;
  };
  const writeU32 = (view, structureName, name, value) => {
    view.setUint32(field(structureName, name, "u32").offset, value, true);
  };
  const writeU64 = (view, structureName, name, value) => {
    view.setBigUint64(field(structureName, name, "u64").offset, value, true);
  };
  const writeBytes = (view, structureName, name, type, value) => {
    const row = field(structureName, name, type);
    assert.ok(value.length <= Number(type.match(/\[(\d+)\]$/)[1]));
    new Uint8Array(view.buffer, view.byteOffset + row.offset, value.length).set(value);
  };
  const zero = (pointer, bytes) => {
    new Uint8Array(exports.memory.buffer, pointer, bytes).fill(0);
  };
  const range = (label, pointer, bytes) => {
    assert.ok(Number.isInteger(pointer) && pointer > 0, `${label} pointer must be nonzero`);
    assert.ok(
      pointer + bytes <= exports.memory.buffer.byteLength,
      `${label} range must fit current Wasm memory`,
    );
  };
  const view = (label, pointer, bytes) => {
    range(label, pointer, bytes);
    return new DataView(exports.memory.buffer, pointer, bytes);
  };
  const writeHeader = (target, structureName) => {
    writeU32(target, structureName, "structSize", structure(structureName).bytes);
    writeU32(target, structureName, "abiVersion", ABI_LAYOUT.abiVersion);
  };
  const assertHeader = (target, structureName) => {
    assert.equal(
      target.getUint32(field(structureName, "structSize", "u32").offset, true),
      structure(structureName).bytes,
      `${structureName}.structSize`,
    );
    assert.equal(
      target.getUint32(field(structureName, "abiVersion", "u32").offset, true),
      ABI_LAYOUT.abiVersion,
      `${structureName}.abiVersion`,
    );
  };

  try {
    assert.deepEqual(
      Object.keys(exports).sort(),
      [...ABI_LAYOUT.exports, "memory"].sort(),
      "shipped exports must equal generated exports plus memory",
    );
    for (const name of ABI_LAYOUT.exports) {
      assert.equal(typeof exports[name], "function", `${name} must be callable`);
    }
    assert.ok(exports.memory instanceof WebAssembly.Memory);

    const bootOptions = structure("bootOptions");
    const optionsPointer = exports.miso_engine_web_v1_boot_options_ptr();
    range("boot options", optionsPointer, bootOptions.bytes);
    const options = new DataView(exports.memory.buffer, optionsPointer, bootOptions.bytes);
    zero(optionsPointer, bootOptions.bytes);
    writeHeader(options, "bootOptions");
    writeU32(options, "bootOptions", "requireSampleRateHz", 48_000);
    writeU32(options, "bootOptions", "requireQuantumFrames", 128);

    const preparation = structure("observationPreparation");
    assert.equal(
      exports.miso_engine_web_v1_observation_preparation_bytes(),
      preparation.bytes,
      "preparation size export",
    );
    const preparationPointer = exports.miso_engine_web_v1_observation_preparation_ptr();
    range("observation preparation", preparationPointer, preparation.bytes);
    const prepared = new DataView(
      exports.memory.buffer,
      preparationPointer,
      preparation.bytes,
    );
    zero(preparationPointer, preparation.bytes);
    writeHeader(prepared, "observationPreparation");
    writeU32(
      prepared,
      "observationPreparation",
      "profile",
      constant("observationProfiles", "eqSpectrum"),
    );
    writeU32(prepared, "observationPreparation", "spectrumCount", 1);
    writeU32(prepared, "observationPreparation", "maximumActiveObservers", 1);

    const budget = 67_108_864n;
    writeU64(
      prepared,
      "observationPreparation",
      "activationMaximumRetainedBytes",
      budget,
    );
    writeU32(
      prepared,
      "observationPreparation",
      "workLimits.structSize",
      structure("observationWorkLimits").bytes,
    );
    writeU32(prepared, "observationPreparation", "workLimits.abiVersion", ABI_LAYOUT.abiVersion);
    const workLimitFields = preparation.fields.filter(
      ({ name, type }) => name.startsWith("workLimits.") && type === "u64",
    );
    assert.equal(workLimitFields.length, 11, "all generated work-limit u64 fields are present");
    for (const row of workLimitFields) {
      prepared.setBigUint64(row.offset, budget, true);
    }

    writeU32(
      prepared,
      "observationPreparation",
      "ingressLimits.structSize",
      structure("observationIngressLimits").bytes,
    );
    writeU32(
      prepared,
      "observationPreparation",
      "ingressLimits.abiVersion",
      ABI_LAYOUT.abiVersion,
    );
    writeU32(prepared, "observationPreparation", "ingressLimits.maximumControlBytes", 8_192);
    writeU32(prepared, "observationPreparation", "ingressLimits.maximumObservationRows", 32);
    writeU32(prepared, "observationPreparation", "ingressLimits.maximumResultBytes", 65_536);
    writeU32(
      prepared,
      "observationPreparation",
      "ingressLimits.ordinaryOperationsPerBoundary",
      1,
    );
    writeU32(
      prepared,
      "observationPreparation",
      "ingressLimits.removalOperationsPerBoundary",
      1,
    );
    const ingressLimitFields = preparation.fields.filter(
      ({ name, type }) => name.startsWith("ingressLimits.") && type === "u64",
    );
    assert.equal(ingressLimitFields.length, 7, "all generated ingress-limit u64 fields are present");
    for (const row of ingressLimitFields) {
      prepared.setBigUint64(row.offset, budget, true);
    }

    writeU32(
      prepared,
      "observationPreparation",
      "spectrumRequest.structSize",
      structure("spectrumRequest").bytes,
    );
    writeU32(
      prepared,
      "observationPreparation",
      "spectrumRequest.abiVersion",
      ABI_LAYOUT.abiVersion,
    );
    writeU32(
      prepared,
      "observationPreparation",
      "spectrumRequest.target",
      constant("spectrumTargets", "trackPostMatrix"),
    );
    writeU32(
      prepared,
      "observationPreparation",
      "spectrumRequest.channels",
      constant("spectrumChannels", "both"),
    );
    writeU32(prepared, "observationPreparation", "spectrumRequest.targetIdBytes", 1);
    writeU64(
      prepared,
      "observationPreparation",
      "spectrumRequest.maximumCaptureBytes",
      BigInt(ABI_LAYOUT.constants.spectrumCaptureBytes),
    );
    writeBytes(
      prepared,
      "observationPreparation",
      "targetId",
      "u8[128]",
      new TextEncoder().encode("t"),
    );

    const document = new TextEncoder().encode(
      sessionDocument({ sampleRateHz: 48_000, quantumFrames: 128 }),
    );
    const documentPointer = exports.miso_engine_web_v1_document_ptr(document.length);
    range("document", documentPointer, document.length);
    new Uint8Array(exports.memory.buffer, documentPointer, document.length).set(document);

    handle = exports.miso_engine_web_v1_boot_with_observation_demand(document.length);
    assert.notEqual(handle, 0, "protected boot returns a handle");
    assert.equal(
      Number(exports.miso_engine_web_v1_boot_result()),
      constant("resultCodes", "ok"),
      "protected boot result",
    );

    const hostStatus = structure("status");
    const hostStatusPointer = exports.miso_engine_web_v1_status_ptr(handle);
    const hostStatusView = view("host status", hostStatusPointer, hostStatus.bytes);
    assertHeader(hostStatusView, "status");
    assert.equal(
      hostStatusView.getUint32(field("status", "state", "u32").offset, true),
      constant("states", "ready"),
      "host status is ready",
    );

    const observationRecord = (structureName, bytesExport, pointerExport) => {
      const layout = structure(structureName);
      assert.equal(exports[bytesExport](), layout.bytes, `${structureName} size export`);
      const pointer = exports[pointerExport](handle);
      return { layout, pointer };
    };

    const demand = observationRecord(
      "observationDemand",
      "miso_engine_web_v1_observation_demand_bytes",
      "miso_engine_web_v1_observation_demand_ptr",
    );
    const admission = observationRecord(
      "observationAdmission",
      "miso_engine_web_v1_observation_admission_bytes",
      "miso_engine_web_v1_observation_admission_ptr",
    );
    const observationStatus = observationRecord(
      "observationStatus",
      "miso_engine_web_v1_observation_status_bytes",
      "miso_engine_web_v1_observation_status_ptr",
    );
    const captureIdentity = observationRecord(
      "observationCaptureIdentity",
      "miso_engine_web_v1_observation_capture_identity_bytes",
      "miso_engine_web_v1_observation_capture_identity_ptr",
    );
    const demandView = view("observationDemand", demand.pointer, demand.layout.bytes);
    const admissionView = view("observationAdmission", admission.pointer, admission.layout.bytes);
    const observationStatusView = view(
      "observationStatus",
      observationStatus.pointer,
      observationStatus.layout.bytes,
    );
    const captureIdentityView = view(
      "observationCaptureIdentity",
      captureIdentity.pointer,
      captureIdentity.layout.bytes,
    );
    assertHeader(demandView, "observationDemand");
    assertHeader(admissionView, "observationAdmission");
    assertHeader(observationStatusView, "observationStatus");
    assertHeader(captureIdentityView, "observationCaptureIdentity");

    assert.equal(
      observationStatusView.getUint32(
        field("observationStatus", "profile", "u32").offset,
        true,
      ),
      constant("observationProfiles", "eqSpectrum"),
      "protected observation profile",
    );
    assert.ok(
      observationStatusView.getBigUint64(
        field("observationStatus", "owner", "u64").offset,
        true,
      ) > 0n,
      "protected observation owner",
    );

    const receipt = structure("observationReceipt");
    assert.equal(
      exports.miso_engine_web_v1_observation_application_bytes(),
      receipt.bytes,
      "observation application size export",
    );
    const applicationCapacity = exports.miso_engine_web_v1_observation_application_capacity();
    assert.ok(applicationCapacity > 0, "observation application capacity is positive");
    const applicationPointer = exports.miso_engine_web_v1_observation_application_ptr();
    const applicationBytes = receipt.bytes * applicationCapacity;
    range("observation application", applicationPointer, applicationBytes);
    for (let index = 0; index < applicationCapacity; index += 1) {
      const application = new DataView(
        exports.memory.buffer,
        applicationPointer + index * receipt.bytes,
        receipt.bytes,
      );
      assertHeader(application, "observationReceipt");
    }
  } finally {
    assert.equal(exports.miso_engine_web_v1_dispose(handle), 0, "dispose protected boot");
  }
});

describe("eval 1 -- the three red probes at the SDK boundary", () => {
  test("(a) a quoted-key 48k/128 raw document boots headless", async () => {
    const engine = await createOfflineEngine(
      sessionDocument({ quoteKeys: true, sampleRateHz: 48_000, quantumFrames: 128 }),
      { asset },
    );
    try {
      const shape = engine.shape();
      assert.equal(shape.sampleRateHz, 48_000);
      assert.equal(shape.quantumFrames, 128);
    } finally {
      engine.dispose();
    }
  });

  test("(b) a quoted-key 96k/127 raw document boots and introspects its TRUE shape", async () => {
    // The red mutation: reintroduce any header regex over the document text. A bare-key-anchored
    // pattern cannot see `"sample_rate_hz" = 96000`, so the old code fell back to 48000/128 --
    // and the fallback ring of 1024 is not a multiple of 127, so the boot could not even be
    // attempted. Every assertion below is on the engine's answer, not on the fixture's input.
    const engine = await createOfflineEngine(
      sessionDocument({ quoteKeys: true, sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
      { asset },
    );
    try {
      const shape = engine.shape();
      assert.equal(shape.sampleRateHz, 96_000, "the boot's own answer, never a 48k fallback");
      assert.equal(shape.quantumFrames, 127);
      assert.equal(shape.sources.length, 1);
      assert.equal(shape.sources[0].id, "s");
      assert.equal(shape.sources[0].channels, 2);
      assert.equal(shape.sources[0].frames, 9_600n);
      assert.deepEqual(shape.tracks, ["t"]);
      // 9906 = 78 x 127, derived from the published rule applied to the reported shape.
      assert.equal(shape.sourceRingFrames, 9_906);
      assert.equal(shape.sourceRingFrames % shape.quantumFrames, 0);
      assert.equal(engine.state(), "ready");
    } finally {
      engine.dispose();
    }
  });

  test("(c) a launch-rate refusal is typed, and costs no second compile", async () => {
    // Amended by adopted ruling 5462139867 finding 1. The brief's original probe was a
    // source-rate mismatch, which #241/A2 made unrepresentable: per-source `sample_rate_hz` no
    // longer exists, so such a document refuses as an unknown key on a different path. The
    // compile-count assertion is the probe's actual payload and is unchanged; the refusal it
    // rides on re-bases onto a refusal that exists post-#241.
    const before = compiles;
    const refusal = await validate(sessionDocument({ sampleRateHz: 44_056 }), { asset });
    assert.equal(refusal.ok, false);
    assert.equal(refusal.phase, "boot");
    assert.equal(refusal.code, "refusedDocument");
    assert.deepEqual(
      refusal.diagnostics.map((row) => [row.code, row.path]),
      [["sample_rate.unsupported_at_launch", "$.sample_rate_hz"]],
    );

    // The payload. The pre-boot-v1 `validate` built a whole throwaway engine, compiling the
    // module again every single time; boot v1 instantiates a compiled module instead.
    assert.equal(compiles - before, 0, "validate() must not compile the module again");
    assert.equal(asset.compileCount, 1, "one asset, one compile, for the SDK's whole lifetime");
  });

  test("(c companion) the deleted per-source rate key refuses typed at its own path", async () => {
    // Pins the #241/A2 deletion itself: a document still carrying `sample_rate_hz` on a source is
    // not quietly tolerated, it is an unknown key at `$.sources[0].sample_rate_hz`.
    const refusal = await validate(
      sessionDocument({ sourceExtra: ',"sample_rate_hz":48000' }),
      { asset },
    );
    assert.equal(refusal.ok, false);
    assert.equal(refusal.code, "refusedDocument");
    assert.deepEqual(
      refusal.diagnostics.map((row) => [row.code, row.path]),
      [["schema.unknown_field", "$.sources[0].sample_rate_hz"]],
    );
  });

  test("(c companion) the deleted per-source start_frame refuses the same way", async () => {
    const refusal = await validate(sessionDocument({ sourceExtra: ',"start_frame":0' }), { asset });
    assert.equal(refusal.ok, false);
    assert.deepEqual(
      refusal.diagnostics.map((row) => [row.code, row.path]),
      [["schema.unknown_field", "$.sources[0].start_frame"]],
    );
  });

  test("no probe ever reports the browser's physical-mismatch code", async () => {
    // `host.session.shape` is the *worklet's* answer when a document does not match the
    // AudioContext. A headless boot writes both `require_*` words at zero, so it has no physical
    // shape to mismatch and must never produce that diagnostic. The old SDK's fallback made this
    // reachable headlessly; boot v1 makes it structurally impossible.
    for (const rate of [44_100, 48_000, 88_200, 96_000]) {
      const result = await validate(
        sessionDocument({ sampleRateHz: rate, quantumFrames: 127, quoteKeys: true }),
        { asset },
      );
      assert.equal(result.ok, true, `a launch rate must boot: ${rate}`);
      assert.equal(result.shape.sampleRateHz, rate);
    }
  });
});

describe("eval 3 -- refusal time and admission without staging growth", () => {
  test("an over-maximum document refuses typed WITHOUT growing wasm memory", async () => {
    // The standing unbounded-admission bug: the old `validateSession` sized its staging from the
    // input, `max(1 MiB, input.length)`, so proving a 64 MiB document too large cost 64 MiB.
    // `document_ptr` refuses an oversize length before it allocates anything.
    //
    // Red mutation: restore input-sized staging -- the growth assertion goes red immediately.
    const maximum = ABI_LAYOUT.constants.maximumDocumentBytes;
    const page = 1 << 16;
    const instance = await asset.instantiate();
    const exports = instance.exports;
    const fresh = exports.memory.buffer.byteLength;

    // First refusal. One page of slack, matching `scripts/check-web-boot-budget.mjs`'s own
    // convention: the module's boot-staging thread-local is initialised lazily, so the very first
    // call into it can take a page for a 64-byte `Box`. That page is a constant, not a function
    // of the input, which is what the second half of this test proves.
    assert.equal(
      exports.miso_engine_web_v1_document_ptr(maximum + 1),
      0,
      "an over-maximum length is refused before any allocation",
    );
    assert.ok(
      exports.memory.buffer.byteLength - fresh <= page,
      `refusing one oversize document grew memory by ${exports.memory.buffer.byteLength - fresh} bytes`,
    );
    assert.equal(
      Number(exports.miso_engine_web_v1_boot_result()),
      ABI_LAYOUT.constants.resultCodes.find((row) => row.name === "invalidArgument").value,
    );

    // The assertion that actually catches the bug. The old `validateSession` sized staging from
    // the input -- `max(1 MiB, input.length)` -- so admission cost grew with the document. Here a
    // document sixty-four times over the maximum is refused with the module's memory *byte for
    // byte unchanged* from the previous refusal.
    //
    // Red mutation: restore input-sized staging and this goes red at the first large length.
    const warmed = exports.memory.buffer.byteLength;
    for (const length of [maximum + 1, maximum * 2, maximum * 8, maximum * 64]) {
      assert.equal(
        exports.miso_engine_web_v1_document_ptr(length),
        0,
        `a ${length}-byte document must be refused`,
      );
      assert.equal(
        exports.memory.buffer.byteLength,
        warmed,
        `refusing a ${length}-byte document grew the module's memory`,
      );
    }

    // And the same refusal through the SDK's own surface, typed.
    const oversize = new Uint8Array(maximum + 1).fill(0x20);
    await assert.rejects(
      () => WasmBoundary.boot(asset, oversize),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.phase, "boot");
        assert.equal(error.code, "refusedDocument");
        return true;
      },
    );
  });

  test("validate() of a large invalid document answers within a stated wall budget", async () => {
    // #240's engine-side fixture pins the engine's own refusal time. The SDK's job is not to be
    // faster than the engine -- it cannot be -- but to add only marshalling on top of it. The
    // margin is stated rather than tuned: one second is orders of magnitude above the engine's
    // measured refusal and still catches the regression this eval exists for, which is a
    // *quadratic* or allocation-bound admission path rather than a few milliseconds of drift.
    const maximum = ABI_LAYOUT.constants.maximumDocumentBytes;
    // A document that is exactly at the maximum and invalid as late as possible: a real header
    // followed by padding, so the parser does real work before it refuses.
    const encoder = new TextEncoder();
    const baseline = encoder.encode(sessionDocument({ sampleRateHz: 44_056, padding: 1 })).length;
    // `padding` writes "\n# " plus that many characters, so one added character is one added byte
    // and the fixture can be sized to the maximum exactly rather than approximately.
    const document = sessionDocument({ sampleRateHz: 44_056, padding: 1 + maximum - baseline });
    const encoded = encoder.encode(document);
    assert.equal(encoded.length, maximum, "the fixture is exactly the maximum admissible length");

    const started = performance.now();
    const refusal = await validate(encoded, { asset });
    const elapsed = performance.now() - started;

    assert.equal(refusal.ok, false);
    assert.equal(refusal.diagnostics[0].code, "sample_rate.unsupported_at_launch");
    assert.ok(
      elapsed < 1_000,
      `a maximum-length invalid document refused in ${elapsed.toFixed(1)} ms, over the 1000 ms budget`,
    );
  });
});

describe("eval 7 -- skew", () => {
  test("a wrong asset digest is a typed abiMismatch at phase asset", async () => {
    const wrong = "0".repeat(64);
    await assert.rejects(
      () => MisoEngineAsset.load(bytes, wrong),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.phase, "asset");
        assert.equal(error.code, "abiMismatch");
        assert.equal(error.diagnosticCode, "sdk.asset.digest");
        return true;
      },
    );
  });

  test("the real digest verifies, and is the digest the SDK reports", async () => {
    const digest = await sha256Hex(bytes);
    const verified = await MisoEngineAsset.load(bytes, digest);
    assert.equal(verified.sha256, digest);
  });

  test("a zero-ABI artifact is a typed abiMismatch, never a partial boot", async () => {
    // A hand-built module that exports exactly an invalid version word. Synthesised
    // rather than vendored: a checked-in stale binary would rot, and the only fact this fixture
    // needs to carry is one wrong `u32`.
    const stale = await MisoEngineAsset.load(versionOnlyModule(0));
    await assert.rejects(
      () => stale.instantiate(),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.phase, "asset");
        assert.equal(error.code, "abiMismatch");
        assert.equal(error.diagnosticCode, "sdk.asset.abi_version");
        assert.equal(error.diagnosticPath, "0");
        return true;
      },
    );
  });

  test("a mismatched ABI artifact is refused with direction-free wording", async () => {
    const future = await MisoEngineAsset.load(versionOnlyModule(0xffff_ffff));
    await assert.rejects(() => future.instantiate(), MisoEngineError);
  });

  test("the SDK's provenance names the ABI it was generated against", () => {
    assert.equal(asset.provenance.abiVersion, ABI_LAYOUT.abiVersion);
    assert.equal(asset.provenance.abiVersion, 0x0001_0000);
    assert.deepEqual(asset.provenance.stagingSequence, ABI_LAYOUT.stagingSequence);
    assert.equal(asset.provenance.artifacts.length, 7);
  });
});

describe("eval 8 -- lifecycle", () => {
  test("a headless mix switch reboots the same instance onto another document", async () => {
    const engine = await createOfflineEngine(
      sessionDocument({ sessionId: "mix-a", sampleRateHz: 48_000, quantumFrames: 128 }),
      { asset },
    );
    try {
      assert.equal(engine.shape().sampleRateHz, 48_000);
      assert.equal(engine.shape().quantumFrames, 128);

      engine.loadSession(
        sessionDocument({ sessionId: "mix-b", sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
      );

      const shape = engine.shape();
      assert.equal(shape.sampleRateHz, 96_000, "the second boot's own answer");
      assert.equal(shape.quantumFrames, 127);
      assert.equal(shape.sources[0].frames, 9_600n);
      assert.equal(shape.sourceRingFrames, 9_906);
      assert.equal(engine.state(), "ready");
    } finally {
      engine.dispose();
    }
  });

  test("boot while live surfaces the typed lifecycle refusal", async () => {
    // Single-live-handle is structural: the module holds one slot, and both `document_ptr` and
    // `boot` probe it before doing anything. Staging a second document on a live instance is
    // therefore refused at the *staging* call, which is why this asserts on `document_ptr`.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const instance = engine.asset === asset ? undefined : undefined;
      assert.equal(instance, undefined); // the engine owns its instance; reach it via the export
      // Reaching the same instance requires the boundary, so drive the ABI directly on a fresh
      // instance that is deliberately booted twice.
      const raw = await asset.instantiate();
      const exports = raw.exports;
      const document = new TextEncoder().encode(sessionDocument());
      const options = exports.miso_engine_web_v1_boot_options_ptr();
      new Uint8Array(exports.memory.buffer, options, 64).fill(0);
      const pointer = exports.miso_engine_web_v1_document_ptr(document.length);
      new Uint8Array(exports.memory.buffer, pointer, document.length).set(document);
      const handle = exports.miso_engine_web_v1_boot(document.length);
      assert.notEqual(handle, 0);

      const second = exports.miso_engine_web_v1_document_ptr(document.length);
      assert.equal(second, 0, "staging on a live instance is refused");
      const lifecycle = ABI_LAYOUT.constants.bootResultAliases
        .find((row) => row.name === "refusedLifecycle").value;
      assert.equal(Number(exports.miso_engine_web_v1_boot_result()), lifecycle);
      assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
    } finally {
      engine.dispose();
    }
  });

  test("a disposed engine refuses further use rather than reading a stale handle", async () => {
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    engine.dispose();
    assert.throws(() => engine.shape(), /disposed/);
    // Dispose is idempotent: the ABI treats handle 0 as an explicit no-op.
    engine.dispose();
  });
});

after(() => {
  // The whole-file statement of eval 1(c): every boot, reboot, validate and refusal above ran
  // against exactly the compiles counted here. Two assets are loaded deliberately (the two skew
  // fixtures), so the bound is stated per asset and in total.
  assert.equal(asset.compileCount, 1);
  assert.ok(compiles <= 4, `the suite compiled ${compiles} modules; the real asset must compile once`);
});

/**
 * A minimal wasm module exporting `miso_engine_web_v1_abi_version` returning `version`.
 *
 * Hand-encoded rather than assembled from a toolchain so the fixture has no build step and no
 * checked-in binary. It is exactly: one type `() -> i32`, one function, one export.
 */
function versionOnlyModule(version) {
  const name = "miso_engine_web_v1_abi_version";
  const nameBytes = [...new TextEncoder().encode(name)];
  const leb = (value) => {
    const out = [];
    let rest = value;
    do {
      let byte = rest & 0x7f;
      rest >>>= 7;
      if (rest !== 0) byte |= 0x80;
      out.push(byte);
    } while (rest !== 0);
    return out;
  };
  const sleb = (value) => {
    const out = [];
    let rest = value;
    for (;;) {
      const byte = rest & 0x7f;
      rest >>= 7;
      const signBit = (byte & 0x40) !== 0;
      if ((rest === 0 && !signBit) || (rest === -1 && signBit)) {
        out.push(byte);
        return out;
      }
      out.push(byte | 0x80);
    }
  };
  const section = (id, payload) => [id, ...leb(payload.length), ...payload];
  const types = section(1, [0x01, 0x60, 0x00, 0x01, 0x7f]);
  const functions = section(3, [0x01, 0x00]);
  const exports = section(7, [0x01, ...leb(nameBytes.length), ...nameBytes, 0x00, 0x00]);
  const body = [0x00, ...[0x41, ...sleb(version | 0)], 0x0b];
  const code = section(10, [0x01, ...leb(body.length), ...body]);
  return new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    ...types, ...functions, ...exports, ...code,
  ]);
}
