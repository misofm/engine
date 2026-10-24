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
import { StructView } from "../src/core/abi.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
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
  liveControls: Object.freeze({
    meterBlocks: 6,
    observationTaps: 2,
  }),
});

const PHYSICAL = Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128 });

function wordAt(block, name) {
  const row = ABI_LAYOUT.structures.bootOptions.fields.find((field) => field.name === name);
  const view = new DataView(block.buffer, block.byteOffset, block.byteLength);
  return row.type === "u64" ? view.getBigUint64(row.offset, true) : view.getUint32(row.offset, true);
}

test("generated u64 offsets preserve BigInt", () => {
  const memory = new WebAssembly.Memory({ initial: 1 });
  const leaves = [
    { structure: "spectrumStreamMetadata", name: "captureEpoch", value: 0x0020_0000_0000_0001n },
    { structure: "spectrumStreamMetadata", name: "analysisEpoch", value: 0x0040_0000_0000_0003n },
    { structure: "spectrumStreamMetadata", name: "historyStartSample", value: 0x0080_0000_0000_0005n },
    { structure: "liveResponseResult", name: "snapshotToken", value: 0x0100_0000_0000_0007n },
    { structure: "liveResponseResult", name: "capturedSample", value: 0x0200_0000_0000_0009n },
    { structure: "observationResult", name: "firstSample", value: 0x0400_0000_0000_000bn },
    { structure: "observationResult", name: "sequence", value: 0xffff_ffff_ffff_ffd1n },
    { structure: "observationResult", name: "endSample", value: 0xffff_ffff_ffff_fff1n },
  ];
  const structures = [...new Set(leaves.map(({ structure }) => structure))];
  const pointers = new Map();
  let nextPointer = 1;
  for (const structure of structures) {
    pointers.set(structure, nextPointer);
    nextPointer += ABI_LAYOUT.structures[structure].bytes;
  }

  const ranges = structures.map((structure) => ({
    structure,
    pointer: pointers.get(structure),
    end: pointers.get(structure) + ABI_LAYOUT.structures[structure].bytes,
  }));
  for (const range of ranges) {
    assert.ok(range.pointer > 0, `${range.structure} must use a nonzero pointer`);
  }
  for (let index = 0; index < ranges.length; index += 1) {
    for (let other = index + 1; other < ranges.length; other += 1) {
      assert.ok(
        ranges[index].end <= ranges[other].pointer || ranges[other].end <= ranges[index].pointer,
        `${ranges[index].structure} and ${ranges[other].structure} must be disjoint`,
      );
    }
  }

  const views = new Map(
    structures.map((structure) => [
      structure,
      new StructView(memory, structure, pointers.get(structure)),
    ]),
  );
  const generatedLeaves = leaves.map((leaf) => {
    const layout = ABI_LAYOUT.structures[leaf.structure];
    assert.ok(layout, `${leaf.structure} must exist in the generated ABI`);
    const row = layout.fields.find(({ name }) => name === leaf.name);
    assert.ok(row, `${leaf.structure}.${leaf.name} must exist in the generated ABI`);
    assert.equal(row.type, "u64", `${leaf.structure}.${leaf.name} must be a scalar u64`);
    return { ...leaf, row };
  });

  for (const leaf of generatedLeaves) {
    const data = new DataView(
      memory.buffer,
      pointers.get(leaf.structure),
      ABI_LAYOUT.structures[leaf.structure].bytes,
    );
    data.setBigUint64(leaf.row.offset, leaf.value, true);
  }

  for (const leaf of generatedLeaves) {
    const view = views.get(leaf.structure);
    assert.equal(view.u64(leaf.name), leaf.value, `${leaf.structure}.${leaf.name}`);
    assert.equal(view.snapshot()[leaf.name], leaf.value, `${leaf.structure}.${leaf.name} snapshot`);
  }
});

describe("finding 3 -- the scratch and worklet boots agree on every policy word", () => {
  test("the two option blocks are byte-equal with the two require_* words masked", () => {
    // "Identical options struct" was literally impossible: the scratch boot writes `require_* = 0`
    // while the worklet writes the physical rate and quantum, and two structs differing in two
    // words are not identical. The divergence A-1 actually named was the LIVE-CONTROL words, so the
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
      liveControlMeterBlocks: 6n,
      liveControlObservationTaps: 2n,
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

    // RED MUTATION: let the worklet's policy diverge on one live-control word -- say a scratch
    // boot that budgets no live controls while the worklet budgets them. The masked comparison goes
    // red, which is A-1's actual hazard: the scratch instance would size a different plan and
    // answer a question about a session nobody is going to run.
    const divergent = bootOptionsAgree(
      { ...POLICY, liveControls: { ...POLICY.liveControls, observationTaps: 0 } },
      PHYSICAL,
    );
    assert.notEqual(
      wordAt(divergent.scratch, "liveControlObservationTaps"),
      wordAt(scratch, "liveControlObservationTaps"),
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
      assert.equal(wordAt(block, "reserved1"), 0n, "reserved1 must be zero or boot refuses web.options.reserved1");
      assert.equal(wordAt(block, "reserved2"), 0n, "reserved2 must be zero or boot refuses web.options.reserved2");
    }
    assert.equal(bootOptionsAgree({}, PHYSICAL).maskedEqual, true);
  });

  test("reserved1 and reserved2 stay zero whatever live controls the caller sets", () => {
    // Offset 32 was the live-command queue size and offset 56 the master boot word (#1689); the
    // host now refuses a nonzero word. Red if the writer ever puts a policy value back into either.
    const { scratch, worklet } = bootOptionsAgree(POLICY, PHYSICAL);
    for (const word of ["reserved1", "reserved2"]) {
      assert.equal(wordAt(scratch, word), 0n, `scratch ${word}`);
      assert.equal(wordAt(worklet, word), 0n, `worklet ${word}`);
    }
  });

  test("a caller's retired masterTrackPlusOne is refused and points to strip.master", () => {
    // Issue #1689, root decision: refused, never ignored. Red if an untyped caller's leftover word
    // is silently dropped (its app would boot with no master and never be told).
    const stale = { ...POLICY, liveControls: { ...POLICY.liveControls, masterTrackPlusOne: 1 } };
    assert.throws(
      () => bootOptionsAgree(stale, PHYSICAL),
      (error) => error instanceof MisoUsageError && /strip\.master\(true\)/.test(error.message),
    );
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
        { id: "stem", spec: { channels: 2, bitDepth, frames: 480, content: `blake3:${"0".repeat(64)}` } },
      ]));
    }
    assert.throws(
      () => assertWebDeliverableSources([
        { id: "stem", spec: { channels: 2, bitDepth: "32f", frames: 480, content: `blake3:${"0".repeat(64)}` } },
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
        { id: "drums", spec: { channels: 2, bitDepth: 24, frames: 480, content: `blake3:${"0".repeat(64)}` } },
        { id: "vocals", spec: { channels: 1, bitDepth: "32f", frames: 480, content: `blake3:${"0".repeat(64)}` } },
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
    // The live-control words of POLICY, but no ring override: the engine requires an explicit ring
    // to be a whole number of quanta, and 512 is not a multiple of 127. Leaving it absent selects
    // the engine's own derivation, which is what a caller with no reason to override should do --
    // and it keeps this test about the shape words rather than about ring arithmetic.
    const policy = { maximumMemoryBytes: POLICY.maximumMemoryBytes, liveControls: POLICY.liveControls };
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

  // Issue #1532: every wait of close() that can hang is bounded at 10 s. These tests tick the
  // mocked setTimeout; the only deadline they cross is the close bound itself (the scripted
  // host and context set none).
  const flush = () => new Promise((resolve) => setImmediate(resolve));
  const never = () => new Promise(() => {});

  async function boundedCloseEngine({ dispose, close }) {
    const calls = { dispose: 0, close: 0 };
    const engine = await createEngine({
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: () => { calls.close += 1; return close(); },
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        dispose: () => { calls.dispose += 1; return dispose(); },
      }),
    });
    return { engine, calls };
  }

  /** Collect `unhandledRejection` events for the length of a test. */
  function watchUnhandled(t) {
    const seen = [];
    const listener = (reason) => { seen.push(reason); };
    process.on("unhandledRejection", listener);
    t.after(() => process.off("unhandledRejection", listener));
    return seen;
  }

  const isTimeout = (code, step) => (error) => {
    assert.ok(error instanceof MisoEngineError, "a typed SDK error");
    assert.equal(error.phase, "lifecycle");
    assert.equal(error.code, code);
    assert.equal(error.result, undefined, "an SDK-side code carries no engine result");
    assert.match(error.message, new RegExp(step));
    return true;
  };

  test("a host dispose that never settles rejects close() at 10 s and still closes the context",
    { timeout: 5_000 }, async (t) => {
      const { engine, calls } = await boundedCloseEngine({ dispose: never, close: async () => {} });
      t.mock.timers.enable({ apis: ["setTimeout"] });
      let settled = false;
      const closing = engine.close().then(() => { settled = true; }, () => { settled = true; });
      await flush();
      t.mock.timers.tick(9_999);
      await flush();
      assert.equal(settled, false, "still waiting one millisecond before the bound");
      assert.equal(calls.close, 0, "the context closes only after the dispose step ends");
      t.mock.timers.tick(1);
      await closing;
      assert.equal(settled, true);
      assert.equal(calls.dispose, 1);
      assert.equal(calls.close, 1, "the context close is attempted after a dispose timeout");
      await assert.rejects(() => engine.close(), isTimeout("hostDisposeTimeout", "worklet host dispose"));
    });

  test("an AudioContext close that never settles rejects close() at 10 s",
    { timeout: 5_000 }, async (t) => {
      const { engine, calls } = await boundedCloseEngine({ dispose: async () => {}, close: never });
      t.mock.timers.enable({ apis: ["setTimeout"] });
      const closing = engine.close();
      const outcome = assert.rejects(closing, isTimeout("contextCloseTimeout", "AudioContext close"));
      await flush();
      t.mock.timers.tick(10_000);
      await outcome;
      assert.equal(calls.dispose, 1);
      assert.equal(calls.close, 1);
    });

  test("a timed-out close is not retried: the same error comes back and nothing runs again",
    { timeout: 5_000 }, async (t) => {
      const { engine, calls } = await boundedCloseEngine({ dispose: never, close: async () => {} });
      t.mock.timers.enable({ apis: ["setTimeout"] });
      const first = engine.close().then(() => undefined, (error) => error);
      await flush();
      t.mock.timers.tick(10_000);
      const error = await first;
      assert.ok(error instanceof MisoEngineError);
      const second = await engine.close().then(() => undefined, (rejection) => rejection);
      assert.equal(second, error, "the memoized rejection is returned as is");
      assert.equal(calls.dispose, 1);
      assert.equal(calls.close, 1);
    });

  test("a step that settles after its timeout is observed and leaves no unhandled rejection",
    { timeout: 5_000 }, async (t) => {
      const unhandled = watchUnhandled(t);
      let rejectDispose;
      const { engine } = await boundedCloseEngine({
        dispose: () => new Promise((_, reject) => { rejectDispose = reject; }),
        close: async () => {},
      });
      t.mock.timers.enable({ apis: ["setTimeout"] });
      const closing = engine.close();
      const outcome = assert.rejects(closing, isTimeout("hostDisposeTimeout", "worklet host dispose"));
      await flush();
      t.mock.timers.tick(10_000);
      await outcome;
      rejectDispose(new Error("late port failure"));
      await flush();
      await flush();
      assert.deepEqual(unhandled, [], "the late rejection never reaches process");
    });

  test("a settled close leaves no timer behind", { timeout: 5_000 }, async () => {
    const timers = () => process.getActiveResourcesInfo().filter((name) => name === "Timeout").length;
    const { engine } = await boundedCloseEngine({ dispose: async () => {}, close: async () => {} });
    const before = timers();
    await engine.close();
    assert.ok(timers() <= before, `timers after close ${timers()} exceed ${before} before it`);
  });

  describe("the boot-path context closes are bounded too", () => {
    const SCRIPTED_SHAPE = Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128, tracks: ["t"], submixes: [] });
    const scripted = async () => SCRIPTED_SHAPE;

    test("a wrong-rate candidate whose close never settles rejects createEngine at 10 s",
      { timeout: 5_000 }, async (t) => {
        t.mock.timers.enable({ apis: ["setTimeout"] });
        let constructed = 0;
        const opening = createEngine({
          document: "opaque",
          simd128ModuleUrl: "/wasm",
          workletModuleUrl: "/worklet",
          scratchBoot: scripted,
          contextAttempts: 2,
          createContext: () => {
            constructed += 1;
            return { sampleRate: 44_100, renderQuantumSize: 128, state: "running", close: never, audioWorklet: { addModule: async () => {} } };
          },
        });
        const outcome = assert.rejects(opening, isTimeout("contextCloseTimeout", "AudioContext close"));
        await flush();
        t.mock.timers.tick(10_000);
        await outcome;
        assert.equal(constructed, 1, "no second candidate is built after the close timeout");
      });

    test("a quantum refusal reaches the caller after the bound, not the close timeout",
      { timeout: 5_000 }, async (t) => {
        t.mock.timers.enable({ apis: ["setTimeout"] });
        const opening = createEngine({
          document: "opaque",
          simd128ModuleUrl: "/wasm",
          workletModuleUrl: "/worklet",
          scratchBoot: scripted,
          createContext: () => ({ sampleRate: 48_000, renderQuantumSize: 256, state: "running", close: never, audioWorklet: { addModule: async () => {} } }),
        });
        const outcome = assert.rejects(opening, (error) => {
          assert.ok(error instanceof MisoEngineError);
          assert.equal(error.code, "reprepareRequired");
          assert.equal(error.diagnosticPath, "$.quantum_frames");
          return true;
        });
        await flush();
        t.mock.timers.tick(10_000);
        await outcome;
      });

    test("a worklet boot failure reaches the caller after the bound, not the close timeout",
      { timeout: 5_000 }, async (t) => {
        t.mock.timers.enable({ apis: ["setTimeout"] });
        const failure = new Error("worklet refused");
        const opening = createEngine({
          document: "opaque",
          simd128ModuleUrl: "/wasm",
          workletModuleUrl: "/worklet",
          scratchBoot: scripted,
          createContext: () => ({ sampleRate: 48_000, renderQuantumSize: 128, state: "running", close: never, audioWorklet: { addModule: async () => {} } }),
          createHost: async () => { throw failure; },
        });
        const outcome = assert.rejects(opening, (error) => error === failure);
        await flush();
        t.mock.timers.tick(10_000);
        await outcome;
      });
  });

  test("a 32f source refuses before any context is constructed", async () => {
    let constructed = 0;
    await assert.rejects(
      () => createEngine({
        document: sessionDocument({ bitDepth: "32f" }),
        sources: [{
          id: "stem",
          spec: { channels: 2, bitDepth: "32f", frames: 4_800, content: `blake3:${"0".repeat(64)}` },
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
