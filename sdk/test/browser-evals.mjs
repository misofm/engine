/**
 * Issue #243 S3 and adopted-ruling findings 3 and 6: the browser choreography's decidable half.
 *
 * A browser session is mostly `AudioContext` construction and `addModule`, and no Node test can
 * exercise those -- #246 owns those matrices end to end. But the decisions that actually determine
 * whether a session is correct are values, not APIs: which words the two boots share, when a
 * quantum is refused, which documents a browser may open, and in what order the expensive steps
 * happen. Those are proved here, against the real generated layout and a scripted context.
 */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import {
  POLICY_WORDS,
  ROLE_DEFINED_WORDS,
  assertQuantumMatch,
  assertWebDeliverableSources,
  bootOptionsAgree,
  createEngine,
  scratchBootInWorker,
  scratchBootOptions,
  workletBootOptions,
} from "../src/browser/index.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

/** A policy with every shared word set to something distinctive, so a dropped word shows. */
const POLICY = Object.freeze({
  sourceRingFrames: 512,
  maximumMemoryBytes: 64n << 20n,
  console: Object.freeze({
    commandQueueRecords: 48,
    meterBlocks: 6,
    observationTaps: 2,
    masterTrackPlusOne: 1,
  }),
});

const PHYSICAL = Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128 });

function wordAt(block, name) {
  const row = ABI_LAYOUT.structures.bootOptions.fields.find((field) => field.name === name);
  const view = new DataView(block.buffer, block.byteOffset, block.byteLength);
  return row.type === "u64" ? view.getBigUint64(row.offset, true) : view.getUint32(row.offset, true);
}

describe("finding 3 -- the scratch and worklet boots agree on every policy word", () => {
  test("the two option blocks are byte-equal with the two require_* words masked", () => {
    // "Identical options struct" was literally impossible: the scratch boot writes `require_* = 0`
    // while the worklet writes the physical rate and quantum, and two structs differing in two
    // words are not identical. The divergence A-1 actually named was the CONSOLE words, so the
    // rule is stated over the words it is about -- and asserted over BYTES, because bytes are what
    // the engine reads.
    const agreement = bootOptionsAgree(POLICY, PHYSICAL);
    assert.equal(agreement.maskedEqual, true, "the two boots disagree on a shared policy word");
    assert.equal(
      agreement.scratchRequireWordsAreZero,
      true,
      "the scratch boot must require nothing, or it would refuse the documents it exists to read",
    );
  });

  test("every shared policy word survives into both blocks with the caller's value", () => {
    const { scratch, worklet } = bootOptionsAgree(POLICY, PHYSICAL);
    const expected = {
      sourceRingFrames: 512,
      maximumMemoryBytes: 64n << 20n,
      consoleCommandQueueRecords: 48n,
      consoleMeterBlocks: 6n,
      consoleObservationTaps: 2n,
      consoleMasterTrackPlusOne: 1n,
    };
    assert.deepEqual(
      [...POLICY_WORDS].sort(),
      Object.keys(expected).sort(),
      "the policy-word list and this test's expectations are the same set",
    );
    for (const word of POLICY_WORDS) {
      assert.equal(wordAt(scratch, word), expected[word], `scratch ${word}`);
      assert.equal(wordAt(worklet, word), expected[word], `worklet ${word}`);
    }
  });

  test("the two role-defined words are exactly where the two boots differ", () => {
    const { scratch, worklet } = bootOptionsAgree(POLICY, PHYSICAL);
    assert.deepEqual([...ROLE_DEFINED_WORDS], ["requireSampleRateHz", "requireQuantumFrames"]);
    assert.equal(wordAt(scratch, "requireSampleRateHz"), 0);
    assert.equal(wordAt(scratch, "requireQuantumFrames"), 0);
    assert.equal(wordAt(worklet, "requireSampleRateHz"), PHYSICAL.sampleRateHz);
    assert.equal(wordAt(worklet, "requireQuantumFrames"), PHYSICAL.quantumFrames);

    // RED MUTATION: let the worklet's policy diverge on one console word -- say a scratch boot
    // that budgets no console while the worklet budgets one. The masked comparison goes red, which
    // is A-1's actual hazard: the scratch instance would size a different plan and answer a
    // question about a session nobody is going to run.
    const divergent = bootOptionsAgree(
      { ...POLICY, console: { ...POLICY.console, commandQueueRecords: 0 } },
      PHYSICAL,
    );
    assert.notEqual(
      wordAt(divergent.scratch, "consoleCommandQueueRecords"),
      wordAt(scratch, "consoleCommandQueueRecords"),
      "the mutation must actually change a shared word",
    );
  });

  test("an empty policy still writes the handshake pair in both blocks", () => {
    // Absent means zero means the engine's default -- but the two handshake words are the SDK's,
    // not the caller's, and they are what turns a layout skew into a typed refusal.
    const { scratch, worklet } = bootOptionsAgree({}, PHYSICAL);
    for (const block of [scratch, worklet]) {
      assert.equal(wordAt(block, "structSize"), ABI_LAYOUT.structures.bootOptions.bytes);
      assert.equal(wordAt(block, "abiVersion"), ABI_LAYOUT.abiVersion);
      assert.equal(wordAt(block, "reserved0"), 0, "the reserved word must be zero or boot refuses");
    }
    assert.equal(bootOptionsAgree({}, PHYSICAL).maskedEqual, true);
  });
});

describe("the pre-worklet quantum refusal", () => {
  test("a matching quantum passes, including the absent-renderQuantumSize default", () => {
    assert.doesNotThrow(() => assertQuantumMatch(128, 128));
    assert.doesNotThrow(() => assertQuantumMatch(undefined, 128), "absent reads as the spec's 128");
  });

  test("a mismatch is typed, and names the worklet's own diagnostic", () => {
    assert.throws(
      () => assertQuantumMatch(128, 127),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "reprepareRequired");
        assert.equal(error.diagnosticCode, "host.session.shape");
        assert.equal(error.diagnosticPath, "$.quantum_frames");
        return true;
      },
    );
    // The check exists because discovering this inside the worklet costs an addModule, a node
    // construction and a boot first, and reports it as a sticky failure on a live audio graph.
    assert.throws(() => assertQuantumMatch(undefined, 127), MisoEngineError);
  });
});

describe("finding 6 -- web delivery is integer-only at launch", () => {
  test("16 and 24 open; 32f refuses typed, naming the token", () => {
    for (const bitDepth of [16, 24]) {
      assert.doesNotThrow(() => assertWebDeliverableSources([
        { id: "stem", spec: { channels: 2, bitDepth, frames: 480, content: `sha256:${"0".repeat(64)}` } },
      ]));
    }
    assert.throws(
      () => assertWebDeliverableSources([
        { id: "stem", spec: { channels: 2, bitDepth: "32f", frames: 480, content: `sha256:${"0".repeat(64)}` } },
      ]),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "unsupported");
        assert.equal(error.phase, "source");
        assert.equal(error.diagnosticCode, "stem.depth.unsupported_at_launch");
        assert.ok(error.diagnosticPath.includes("32f"), "the refusal names the token");
        return true;
      },
    );
  });

  test("the refusal is per source, and names which one", () => {
    assert.throws(
      () => assertWebDeliverableSources([
        { id: "drums", spec: { channels: 2, bitDepth: 24, frames: 480, content: `sha256:${"0".repeat(64)}` } },
        { id: "vocals", spec: { channels: 1, bitDepth: "32f", frames: 480, content: `sha256:${"0".repeat(64)}` } },
      ]),
      (error) => {
        assert.ok(error.message.includes("vocals"), "the refusal names the offending source");
        assert.ok(!error.message.includes("drums"));
        return true;
      },
    );
  });
});

describe("the browser open sequence", () => {
  let asset;
  let bytes;

  before(async () => {
    bytes = await moduleBytes();
    asset = await MisoEngineAsset.load(bytes);
    assert.equal(asset.compileCount, 1);
  });

  /** The scratch boot, run for real -- the same code a caller's Worker would import. */
  async function scratchBoot({ document, options }) {
    return scratchBootInWorker({ moduleBytes: bytes, document, options });
  }

  test("the scratch boot reads the document's shape from the engine", async () => {
    const shape = await scratchBoot({
      document: new TextEncoder().encode(
        sessionDocument({ quoteKeys: true, sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
      ),
      options: scratchBootOptions({}),
    });
    assert.equal(shape.sampleRateHz, 96_000);
    assert.equal(shape.quantumFrames, 127);
    assert.equal(shape.sourceRingFrames, 9_906);
  });

  test("a context that comes back at the wrong rate is closed and retried", async () => {
    // An AudioContext is ASKED for a rate; it is not obliged to give one. The verify-close-retry
    // exists because using a context at the wrong rate is worse than not having one.
    const closed = [];
    const constructed = [];
    let attempt = 0;
    const engine = await createEngine({
      document: sessionDocument({ sampleRateHz: 48_000, quantumFrames: 128 }),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => {
        constructed.push(request);
        attempt += 1;
        const sampleRate = attempt === 1 ? 44_100 : request.sampleRate;
        return {
          sampleRate,
          renderQuantumSize: 128,
          state: "running",
          close: async () => { closed.push(sampleRate); },
          audioWorklet: { addModule: async () => {} },
        };
      },
      createHost: async (request) => request,
    });

    assert.equal(constructed.length, 2, "the first context was rejected and a second constructed");
    assert.deepEqual(closed, [44_100], "the wrong-rate context was closed, not leaked");
    assert.equal(engine.context.sampleRate, 48_000);
    for (const request of constructed) {
      assert.equal(request.sampleRate, 48_000, "both attempts asked for the document's rate");
      assert.equal(request.renderSizeHint, 128, "renderSizeHint is passed unconditionally");
    }
  });

  test("exhausting the attempts is a typed refusal, not a context left open", async () => {
    const closed = [];
    await assert.rejects(
      () => createEngine({
        document: sessionDocument({ sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
        simd128ModuleUrl: "/wasm",
        workletModuleUrl: "/worklet",
        contextAttempts: 3,
        scratchBoot,
        createContext: () => ({
          sampleRate: 44_100,
          renderQuantumSize: 127,
          state: "running",
          close: async () => { closed.push(44_100); },
          audioWorklet: { addModule: async () => {} },
        }),
        createHost: async () => { throw new Error("must not be reached"); },
      }),
      (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "reprepareRequired");
        assert.equal(error.diagnosticPath, "$.sample_rate_hz");
        return true;
      },
    );
    assert.equal(closed.length, 3, "every rejected context was closed");
  });

  test("a quantum mismatch refuses BEFORE the worklet is created, and closes the context", async () => {
    let hostCreated = false;
    let contextClosed = false;
    await assert.rejects(
      () => createEngine({
        document: sessionDocument({ sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
        simd128ModuleUrl: "/wasm",
        workletModuleUrl: "/worklet",
        scratchBoot,
        createContext: (request) => ({
          sampleRate: request.sampleRate,
          renderQuantumSize: 128,
          state: "running",
          close: async () => { contextClosed = true; },
          audioWorklet: { addModule: async () => {} },
        }),
        createHost: async () => { hostCreated = true; },
      }),
      (error) => {
        assert.equal(error.diagnosticCode, "host.session.shape");
        return true;
      },
    );
    assert.equal(hostCreated, false, "the worklet must not be built for a shape it cannot render");
    assert.equal(contextClosed, true, "a refused open leaves no context behind");
  });

  test("the worklet boot carries the physical shape as its backstop", async () => {
    // The console words of POLICY, but no ring override: the engine requires an explicit ring to
    // be a whole number of quanta, and 512 is not a multiple of 127. Leaving it absent selects the
    // engine's own derivation, which is what a caller with no reason to override should do -- and
    // it keeps this test about the shape words rather than about ring arithmetic.
    const policy = { maximumMemoryBytes: POLICY.maximumMemoryBytes, console: POLICY.console };
    let seen;
    await createEngine({
      document: sessionDocument({ sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      policy,
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 127,
        state: "running",
        close: async () => {},
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async (request) => { seen = request; return request; },
    });
    assert.equal(seen.options.requireSampleRateHz, 96_000);
    assert.equal(seen.options.requireQuantumFrames, 127);
    // And the shared policy travelled unchanged from the scratch boot's own options.
    assert.deepEqual(
      { ...seen.options, requireSampleRateHz: 0, requireQuantumFrames: 0 },
      scratchBootOptions(policy),
      "the worklet's policy words are the scratch boot's, unchanged",
    );
    assert.deepEqual(
      workletBootOptions(policy, { sampleRateHz: 96_000, quantumFrames: 127 }),
      seen.options,
    );
  });

  test("close disposes the worklet before its context and is idempotent", async () => {
    const events = [];
    const engine = await createEngine({
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: async () => { events.push("context"); },
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        dispose: async () => { events.push("host"); },
      }),
    });

    await Promise.all([engine.close(), engine.close()]);
    assert.deepEqual(events, ["host", "context"]);
    await engine.close();
    assert.deepEqual(events, ["host", "context"], "a settled close stays one operation");
  });

  test("close still releases the AudioContext when host disposal rejects", async () => {
    const failure = new Error("port disappeared");
    let contextCloses = 0;
    const engine = await createEngine({
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: async () => { contextCloses += 1; },
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        dispose: async () => { throw failure; },
      }),
    });

    await assert.rejects(() => engine.close(), (error) => error === failure);
    assert.equal(contextCloses, 1);
    await assert.rejects(() => engine.close(), (error) => error === failure);
    assert.equal(contextCloses, 1, "a rejected close is still not replayed");
  });

  test("a 32f source refuses before any context is constructed", async () => {
    let constructed = 0;
    await assert.rejects(
      () => createEngine({
        document: sessionDocument({ bitDepth: "32f" }),
        sources: [{
          id: "stem",
          spec: { channels: 2, bitDepth: "32f", frames: 4_800, content: `sha256:${"0".repeat(64)}` },
        }],
        simd128ModuleUrl: "/wasm",
        workletModuleUrl: "/worklet",
        scratchBoot,
        createContext: () => { constructed += 1; throw new Error("must not be reached"); },
        createHost: async () => { throw new Error("must not be reached"); },
      }),
      (error) => {
        assert.equal(error.diagnosticCode, "stem.depth.unsupported_at_launch");
        return true;
      },
    );
    assert.equal(constructed, 0, "the refusal costs no AudioContext at all");
  });
});
