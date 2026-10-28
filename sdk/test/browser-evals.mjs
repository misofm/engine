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
import { readFile } from "node:fs/promises";
import vm from "node:vm";

import { BUNDLED_ENGINE_ASSETS } from "../src/assets.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { StructView } from "../src/core/abi.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { ControlTransactionQueue } from "../src/core/control-queue.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { effect, session } from "../src/core/session.ts";
import { transaction } from "../src/core/session-transaction.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import {
  POLICY_WORDS,
  ROLE_DEFINED_WORDS,
  SessionSeekProducerError,
  assertQuantumMatch,
  assertWebDeliverableSources,
  attachEngineFeed,
  bootOptionsAgree,
  createEngine,
  scratchBootInWorker,
  scratchBootOptions,
  workletBootOptions,
} from "../src/browser/index.ts";
import { toWebBootOptions } from "../src/browser/host-mirror.ts";
import { onEndpointIds } from "./control-support.mjs";
import { effectEntry, moduleBytes, sessionDocument } from "./support.mjs";

/** A policy with every shared word set to something distinctive, so a dropped word shows. */
const POLICY = Object.freeze({
  sourceRingFrames: 512,
  maximumMemoryBytes: 64n << 20n,
  liveControls: Object.freeze({
    meterBlocks: 6,
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
    { structure: "planWatermark", name: "revision", value: 0x0100_0000_0000_0007n },
    { structure: "planWatermark", name: "firstSample", value: 0x0200_0000_0000_0009n },
    { structure: "seekReport", name: "firstSample", value: 0x0400_0000_0000_000bn },
    { structure: "seekReport", name: "generation", value: 0xffff_ffff_ffff_ffd1n },
    { structure: "seekReport", name: "sourceFrame", value: 0xffff_ffff_ffff_fff1n },
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
      { ...POLICY, liveControls: { ...POLICY.liveControls, meterBlocks: 0 } },
      PHYSICAL,
    );
    assert.notEqual(
      wordAt(divergent.scratch, "liveControlMeterBlocks"),
      wordAt(scratch, "liveControlMeterBlocks"),
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
      assert.equal(wordAt(block, "reserved3"), 0n, "reserved3 must be zero or boot refuses web.options.reserved3");
      assert.equal(wordAt(block, "reserved2"), 0n, "reserved2 must be zero or boot refuses web.options.reserved2");
    }
    assert.equal(bootOptionsAgree({}, PHYSICAL).maskedEqual, true);
  });

  test("reserved1, reserved3 and reserved2 stay zero whatever live controls the caller sets", () => {
    // Offset 32 was the live-command queue size, offset 48 the observation-taps word (#1590) and
    // offset 56 the master boot word (#1689); the host now refuses a nonzero word. Red if the
    // writer ever puts a policy value back into any of them.
    const { scratch, worklet } = bootOptionsAgree(POLICY, PHYSICAL);
    for (const word of ["reserved1", "reserved3", "reserved2"]) {
      assert.equal(wordAt(scratch, word), 0n, `scratch ${word}`);
      assert.equal(wordAt(worklet, word), 0n, `worklet ${word}`);
    }
  });

  test("a caller's retired masterTrackPlusOne or observationTaps is refused and says why", () => {
    // Issue #1689, root decision, and #1590 D5: refused, never ignored. Red if an untyped caller's
    // leftover word is silently dropped (its app would boot with no master and never be told, or
    // believe it had sized the observation capacity).
    const stale = { ...POLICY, liveControls: { ...POLICY.liveControls, masterTrackPlusOne: 1 } };
    assert.throws(
      () => bootOptionsAgree(stale, PHYSICAL),
      (error) => error instanceof MisoUsageError && /strip\.master\(true\)/.test(error.message),
    );
    const taps = { ...POLICY, liveControls: { ...POLICY.liveControls, observationTaps: 4 } };
    assert.throws(
      () => bootOptionsAgree(taps, PHYSICAL),
      (error) => error instanceof MisoUsageError && /observationTaps was retired \(issue #1590\)/.test(error.message),
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
    assert.equal(shape.sourceRingFrames, 14_097);
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

  test("#2100 gate 1: pcmFeed loads the feed prelude before the engine worklet, and is off by default", async () => {
    // Red if the prelude loads after the engine worklet (the worklet then registers the unwrapped
    // engine processor and no ring ever attaches), if `{ moduleUrl }` is ignored, or if a boot
    // without `pcmFeed` loads the prelude. Issue 1387: red if `pcmFeed` still forces
    // `instanceMode: "single"` on the host (the control Worker drains the feed in worker mode), or
    // if an explicit `instanceMode` does not reach it.
    const modes = [];
    const loads = async (pcmFeed, instanceMode) => {
      const loaded = [];
      const engine = await createEngine({
        document: sessionDocument(),
        simd128ModuleUrl: "/wasm",
        workletModuleUrl: "/worklet",
        scratchBoot,
        ...(pcmFeed === undefined ? {} : { pcmFeed }),
        ...(instanceMode === undefined ? {} : { instanceMode }),
        createContext: (request) => ({
          sampleRate: request.sampleRate,
          renderQuantumSize: 128,
          state: "suspended",
          close: async () => {},
          audioWorklet: { addModule: async (url) => { loaded.push(url); } },
        }),
        // As the shipped host does: its first step on the context loads the engine worklet.
        createHost: async (request) => {
          modes.push(request.instanceMode);
          await request.context.audioWorklet.addModule(request.workletModuleUrl);
          return { dispose: async () => {} };
        },
      });
      await engine.close();
      return loaded;
    };
    assert.deepEqual(await loads(true), [BUNDLED_ENGINE_ASSETS.pcmFeedWorklet.href, "/worklet"]);
    assert.deepEqual(await loads({ moduleUrl: "/feed" }), ["/feed", "/worklet"]);
    assert.deepEqual(await loads(undefined), ["/worklet"]);
    await loads(true, "single");
    assert.deepEqual(modes, [undefined, undefined, undefined, "single"]);
  });

  test("#2100 gate 2: position() is the seek's frame plus the frames rendered since", async () => {
    // Red if the frame is built from the wrong anchor (the render sample alone, or the report
    // without its first sample), if a seek no block rendered yet reports the old timeline, or if
    // the report is read before the status, so that a block rendered between them mixes an old
    // anchor with a new sample.
    const quantum = 128n;
    let next = 0n;
    let report = { generation: 1n, firstSample: 0n, sourceFrame: 0n, cumulativeUnderrunFrames: 0n, cumulativeHeldFrames: 0n };
    let pending;
    let renderAtStatus = false;
    /** One rendered block, as the host's timeline does: an accepted seek starts at the next block. */
    const tick = () => {
      if (pending !== undefined) {
        report = { ...report, generation: pending.generation, firstSample: next, sourceFrame: pending.frame };
        pending = undefined;
      }
      next += quantum;
    };
    const engine = await createEngine({
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: async () => {},
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        seekSession: async ({ generation, frame }) => {
          pending = { generation, frame };
          return { tag: "miso.session-seek.v1", requestId: 1, result: 0, generation };
        },
        status: async () => {
          if (renderAtStatus) { renderAtStatus = false; tick(); }
          return { nextAbsoluteSample: next };
        },
        seekReport: async (sourceId) => {
          assert.equal(sourceId, "$timeline");
          return { tag: "miso.seek-report.v1", requestId: 1, result: 0, report: { ...report } };
        },
        dispose: async () => {},
      }),
    });
    try {
      tick();
      tick();
      assert.deepEqual(await engine.position(), { frame: 256n, generation: 1n, atSample: 256n });
      assert.deepEqual(await engine.seek(480n), { generation: 2n });
      assert.deepEqual(await engine.position(), { frame: 480n, generation: 2n, atSample: 256n },
        "a seek no block rendered yet plays at the next block");
      for (let block = 0; block < 3; block += 1) tick();
      assert.deepEqual(await engine.position(), { frame: 480n + 3n * quantum, generation: 2n, atSample: 640n });
      await engine.seek(96n);
      renderAtStatus = true;
      assert.deepEqual(await engine.position(), { frame: 96n + quantum, generation: 3n, atSample: 768n },
        "a block that renders the seek during the reads is counted once");
    } finally {
      await engine.close();
    }
    await assert.rejects(engine.position(), MisoUsageError);
  });

  test("#2116: planLatency() reads the real worklet's plan latency, 486 samples for a limiter at 48 kHz", async () => {
    // Red if the worklet reads the wrong record word or skips the read, if the host drops or
    // misvalidates the reply, or if the SDK decodes it differently from the headless engine. The
    // shipped host and worklet run for real over the real module; only the port is a stand-in.
    const workletSource = await readFile(new URL("../../hosts/host-web/web/miso-engine-v1-audio-worklet.js", import.meta.url), "utf8");
    const { createMisoAudioWorkletHost } = await import("../../hosts/host-web/web/miso-engine-v1-audio-worklet-host.js");
    const deliver = (target, data, transfer = []) => {
      const cloned = structuredClone(data, { transfer });
      queueMicrotask(() => target.onmessage?.({ data: cloned }));
    };
    let processorPort;
    class AudioWorkletProcessor {
      constructor() { this.port = processorPort; }
    }
    const registrations = new Map();
    const sandbox = vm.createContext({
      // The main realm's constructors, so the worklet's `instanceof` checks see the posted views.
      AudioWorkletProcessor, WebAssembly, TextEncoder, TextDecoder, ArrayBuffer, DataView,
      Uint8Array, Uint32Array, Int32Array, Float32Array, BigInt64Array, BigUint64Array,
      sampleRate: 48_000, renderQuantumSize: 128,
      registerProcessor: (name, constructor) => { registrations.set(name, constructor); },
    });
    vm.runInContext(workletSource, sandbox);
    class AudioWorkletNode {
      constructor(_context, name, options) {
        const nodePort = { onmessage: null, postMessage: (data, transfer) => deliver(processorPort, data, transfer) };
        processorPort = { onmessage: null, postMessage: (data, transfer) => deliver(nodePort, data, transfer) };
        this.port = nodePort;
        this.onprocessorerror = null;
        new (registrations.get(name))(structuredClone(options));
      }
      disconnect() {}
    }
    const module = new WebAssembly.Module(bytes);
    const engine = await createEngine({
      document: sessionDocument({ effects: { inserts: [effectEntry("limit", "miso.true-peak-limiter")] } }),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "suspended",
        close: async () => {},
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async (request) => {
        const previous = globalThis.AudioWorkletNode;
        globalThis.AudioWorkletNode = AudioWorkletNode;
        try {
          return await createMisoAudioWorkletHost({
            ...request, options: toWebBootOptions(request.options), preparedModule: module,
          });
        } finally {
          globalThis.AudioWorkletNode = previous;
        }
      },
    });
    try {
      const [latency, again] = await Promise.all([engine.planLatency(), engine.planLatency()]);
      assert.equal(latency.samples, 486);
      assert.deepEqual(again, latency, "a second read waits for the first instead of being refused");
      assert.ok(latency.tailSamples === null || latency.tailSamples >= latency.samples, "the tail includes the latency");
    } finally {
      await engine.close();
    }
    await assert.rejects(engine.planLatency(), MisoUsageError);
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

    // #1296: a host stand-in that posts no watermark cannot report completion, so applied()
    // refuses typed instead of waiting forever.
    await assert.rejects(engine.applied(0n), (error) => error instanceof MisoEngineError && error.code === "unsupported");
    await Promise.all([engine.close(), engine.close()]);
    assert.deepEqual(events, ["host", "context"]);
    await engine.close();
    assert.deepEqual(events, ["host", "context"], "a settled close stays one operation");
  });

  test("seek repositions the producers only after the engine accepted the session seek", async () => {
    // Issue #1644 gate 3: the order is the claim. A producer seeking first, or without an OK,
    // makes the session seek stale and the ring stall; a stale default generation does the same.
    const log = [];
    let result = 0;
    let localRefusal = null;
    const engine = await createEngine({
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot,
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: async () => {},
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        seekSession: async (request) => {
          if (localRefusal !== null) throw localRefusal;
          log.push(["host", request]);
          await Promise.resolve();
          log.push(["reply", result]);
          return { tag: "miso.session-seek.v1", requestId: 1, result, generation: request.generation };
        },
        dispose: async () => {},
      }),
    });
    const producer = (name) => ({ seek: async (frame, generation) => { log.push([name, frame, generation]); } });
    const producers = [producer("a"), producer("b")];
    assert.deepEqual(await engine.seek(480n, { producers }), { generation: 2n });
    assert.deepEqual(await engine.seek(96n, { producers }), { generation: 3n }, "the next default is one above");
    assert.deepEqual(log.splice(0), [
      ["host", { generation: 2n, frame: 480n }], ["reply", 0], ["a", 480, 2n], ["b", 480, 2n],
      ["host", { generation: 3n, frame: 96n }], ["reply", 0], ["a", 96, 3n], ["b", 96, 3n],
    ]);
    result = 1;
    await assert.rejects(engine.seek(0n, { generation: 3n, producers }), (error) =>
      error instanceof MisoEngineError && error.result === 1 && error.code === "invalidArgument");
    assert.deepEqual(log.splice(0), [["host", { generation: 3n, frame: 0n }], ["reply", 1]],
      "a refused seek repositions nothing");
    await assert.rejects(engine.seek(2n ** 53n, { producers }), MisoUsageError);
    assert.deepEqual(log, [], "an unsafe frame posts nothing");
    result = 0;
    // The host's local refusal of a second unsettled seek is its numeric record.
    localRefusal = Object.freeze({ tag: "miso.error.v1", requestId: 0, result: 6 });
    await assert.rejects(engine.seek(0n, { producers }), (error) =>
      error instanceof MisoEngineError && error.code === "backpressure" && error.result === 6);
    localRefusal = null;
    const failure = new RangeError("seek generation must be a bigint above the pump's own");
    const failing = { seek: async () => { throw failure; } };
    await assert.rejects(engine.seek(0n, { generation: 9n, producers: [producers[0], failing, producers[1]] }), (error) =>
      error instanceof SessionSeekProducerError && error.generation === 9n && error.producerIndex === 1 && error.cause === failure);
    assert.deepEqual(log.splice(0), [["host", { generation: 9n, frame: 0n }], ["reply", 0], ["a", 0, 9n]],
      "a producer failure after the OK names the applied generation and stops");
    await engine.close();
    await assert.rejects(engine.seek(0n), MisoUsageError);
    assert.deepEqual(log, [], "a closed engine posts nothing");
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

/**
 * #1296: the browser engine's session edits over #1389's shared core. The host stand-in's control
 * plane is a real core (a `WasmBoundary` on the shipped module): `control(frame, document)` runs the
 * frame through the entry the worklet's message handler calls and answers `{ result, response,
 * revision }` as the host does, and `tick()` renders one block and posts the watermark when its
 * `(revision, sample)` pair moved, as the worklet does. Real bits in a browser are #1295's.
 */
describe("browser session edits", () => {
  const QUANTUM = 128;
  const SOURCE = { channels: 2, bitDepth: 24, frames: 48_000, content: `blake3:${"a".repeat(64)}` };
  const EQ = () => effect("miso.parametric-eq", { "band-1-gain": { left: 6, right: -4 } });
  /** `tracks` tracks on one source, each with an EQ insert and a route to the output. */
  const declare = (tracks, sampleRateHz = 48_000) => {
    let builder = session({ id: "browser.apply", sampleRateHz, quantumFrames: QUANTUM }).source("a", SOURCE);
    for (const id of tracks) builder = builder.track(id, { source: "a", inserts: [EQ()] });
    builder = builder.output("out");
    for (const id of tracks) {
      builder = builder.route({
        id: `${id}.main`,
        source: { kind: "track", trackId: id, tap: "post_pan" },
        destination: { kind: "output_input", outputId: "out" },
      });
    }
    return builder;
  };
  const base = () => declare(["t1", "t2", "t3", "t4"]);
  const withFifth = () => declare(["t1", "t2", "t3", "t4", "t5"]);
  let asset;
  let bytes;

  before(async () => {
    bytes = await moduleBytes();
    asset = await MisoEngineAsset.load(bytes);
  });

  async function boot(document) {
    const core = await WasmBoundary.boot(asset, new TextEncoder().encode(document.toJson()));
    const posts = new Set();
    const sent = [];
    let next = 0;
    let posted = core.readWatermark();
    // The core's own strip-order reads (its session map) draw from its allocator: the page's
    // frames go onto it, held strictly ascending.
    let pageReplacement;
    const forwardFrame = onEndpointIds(core.controlRequestIds, (frame) => core.controlFrame(frame, pageReplacement));
    const forward = (frame, replacement) => {
      pageReplacement = replacement;
      try { return forwardFrame(frame); } finally { pageReplacement = undefined; }
    };
    const host = {
      async sessionMap() {
        const map = core.sessionMap();
        return { tag: "miso.sessionmap.v1", requestId: (next += 1), result: 0, sources: map.sources, metersAttached: map.metersAttached };
      },
      async control(frame, replacement) {
        sent.push({ frame: frame.slice(), document: replacement?.slice() });
        let result = 0;
        let response;
        try {
          response = forward(frame, replacement);
        } catch (error) {
          if (typeof error?.result !== "number") throw error;
          result = error.result;
          response = new TextEncoder().encode(error.diagnostics.map((row) => `${row.code}\t${row.path}\n`).join(""));
        }
        return { tag: "miso.control.v1", requestId: (next += 1), result, response, revision: core.controlRevision() };
      },
      onControlEvent: (listener) => core.onControlEvent(listener),
      onWatermark(listener) {
        posts.add(listener);
        return () => posts.delete(listener);
      },
      async dispose() { core.dispose(); },
    };
    const post = (record) => { for (const listener of [...posts]) listener(record); };
    const tick = () => {
      core.render(QUANTUM);
      const now = core.readWatermark();
      if (now.revision === posted.revision && now.sample === posted.sample) return;
      posted = now;
      const outcome = (now.outcome.exact ? 1n : 0n) | (now.outcome.transitionFallback ? 2n : 0n) | (now.outcome.superseded ? 4n : 0n);
      post({ revision: now.revision, sample: now.sample, outcome, exactCount: 0n, transitionFallbackCount: 0n, supersededCount: 0n });
    };
    const engine = await createEngine({
      document,
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot: ({ document: staged, options }) => scratchBootInWorker({ moduleBytes: bytes, document: staged, options }),
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: QUANTUM,
        state: "running",
        close: async () => {},
        // `holdResume` stands for a browser that holds `resume()` until a user gesture.
        holdResume: false,
        resume() {
          if (this.holdResume) return new Promise(() => {});
          this.state = "running";
          return Promise.resolve();
        },
        async suspend() { this.state = "suspended"; },
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => host,
    });
    return { engine, host, sent, tick, post };
  }

  /** Whether `promise` has settled, after every queued reaction ran. */
  async function settled(promise) {
    let done = false;
    promise.then(() => { done = true; }, () => { done = true; });
    await new Promise((resolve) => setImmediate(resolve));
    return done;
  }

  test("gate 1: a replace sends its document beside one frame, commits a rebuild and rebinds live controls", async () => {
    // Red if the browser transport drops the document (the core refuses the replace), if
    // `revision` does not follow the reply, if `liveControls()` stays bound to the booted document
    // after a committed replace, or if a refused replace rebinds it.
    const { engine, sent } = await boot(base());
    try {
      await engine.liveControls();
      sent.length = 0;
      const outcome = await engine.replaceSession(withFifth());
      assert.deepEqual(outcome, { revision: 1n, path: "rebuild" });
      assert.equal(engine.revision, 1n);
      const replaces = sent.filter((row) => row.document !== undefined);
      assert.equal(replaces.length, 1, "one frame carries the document");
      assert.equal(new TextDecoder().decode(replaces[0].document), withFifth().toJson());
      const rebound = await engine.liveControls();
      assert.doesNotThrow(() => rebound.withSession(withFifth()));
      // A refused replace (a new rate needs a new engine) commits nothing and rebinds nothing.
      const refused = declare(["t1", "t2", "t3", "t4", "t5"], 96_000);
      await assert.rejects(engine.replaceSession(refused));
      assert.equal(engine.revision, 1n);
      const controls = await engine.liveControls();
      assert.throws(() => controls.withSession(refused), MisoUsageError);
      assert.doesNotThrow(() => controls.withSession(withFifth()));
    } finally {
      await engine.close();
    }
  });

  test("gate 2: a live apply settles at the watermark post that covers it, and close rejects it", async () => {
    // Red if `revision` misses a live edit's commit, if applied() resolves at the commit or on a
    // post for an older revision, if the engine does not settle it from the host's watermark posts,
    // if close() leaves it pending, or if the engine does not tell its live controls' queue the
    // revision an apply committed beside it (the queue's next submit is then refused
    // `revision_conflict`).
    const { engine, tick, post } = await boot(base());
    try {
      const earlier = await engine.liveControls();
      await earlier.submit(earlier.edit.track("t2").faderDb(-1));
      assert.equal(engine.revision, 1n, "a live edit's revision is seen");
      tick();
      const fader = transaction(base()).upsertTrack("t1", { source: "a", inserts: [EQ()], fader: { leftDb: -12, rightDb: -12 } });
      const outcome = await engine.apply(fader.build());
      assert.deepEqual(outcome, { revision: 2n, path: "live" });
      const applied = engine.applied(outcome.revision);
      assert.equal(await settled(applied), false, "pending at the commit");
      post({ revision: 0n, sample: 64n, outcome: 1n, exactCount: 0n, transitionFallbackCount: 0n, supersededCount: 0n });
      assert.equal(await settled(applied), false, "an older revision's post does not settle it");
      tick();
      const snapshot = await applied;
      assert.deepEqual(snapshot, { revision: 2n, sample: BigInt(QUANTUM), outcome: { exact: true, transitionFallback: false, superseded: false } });
      assert.deepEqual(engine.watermark(), snapshot);
      assert.deepEqual(await earlier.submit(earlier.edit.track("t2").faderDb(-3)),
        { ok: true, revision: 3n, path: "live", appliedOperations: 1 });
      const pending = engine.applied(3n);
      await engine.close();
      await assert.rejects(pending, MisoUsageError);
    } finally {
      await engine.close();
    }
  });

  test("another sender's commit is seen through a decoded event and the watermark", async () => {
    // Red if `revision` ignores the `sessionCommitted` events the engine decodes for `onEvent`, or
    // the watermark's revision: a commit another sender made on the same host is then never seen.
    // Red too if applied() refuses such a commit as uncommitted without asking the core.
    const { engine, host, tick } = await boot(base());
    try {
      // The other sender shares the host's one request-ID allocator, as every sender must.
      const { requestIds } = (await engine.liveControls()).queue;
      const other = new ControlTransactionQueue(async (frame) => (await host.control(frame)).response, { initialRevision: 0n, requestIds });
      const row = (await engine.controls()).controls.find((entry) => entry.address.owner.kind === "strip"
        && entry.address.owner.id === "t1" && entry.name === "fader_db" && entry.address.lane === "left");
      const fader = (value) => [{ address: row.address, value }];
      const stop = engine.onEvent(() => {});
      await other.submit(fader(-1));
      assert.equal(engine.revision, 1n, "seen through the event");
      stop();
      await other.submit(fader(-2));
      assert.equal(engine.revision, 1n, "no listener: no event read");
      tick();
      assert.equal(engine.revision, 2n, "seen through the watermark");
      // Committed unseen (no listener, no render): applied() asks the core, and waits.
      await other.submit(fader(-3));
      const unseen = engine.applied(3n);
      assert.equal(await settled(unseen), false, "pending, not refused as uncommitted");
      tick();
      assert.equal((await unseen).revision, 3n);
    } finally {
      await engine.close();
    }
  });

  test("#1297b D6: a removed fed source drains until applied(revision) resolves, then its ring is detached", async () => {
    // Red if the SDK detaches at the commit (the strip's ramp-out plays silence), never detaches (a
    // ring leaks per removed stem), detaches a source the edit kept, or misses a removal that a
    // replace made (the committed session map no longer lists it).
    const fed = (ids) => {
      let builder = session({ id: "browser.feed", sampleRateHz: 48_000, quantumFrames: QUANTUM });
      for (const id of ids) builder = builder.source(id, { ...SOURCE, content: `blake3:${id.repeat(64)}` });
      for (const id of ids) builder = builder.track(`t${id}`, { source: id });
      builder = builder.output("out");
      for (const id of ids) {
        builder = builder.route({ id: `t${id}.main`, source: { kind: "track", trackId: `t${id}`, tap: "post_pan" }, destination: { kind: "output_input", outputId: "out" } });
      }
      return builder;
    };
    const { engine, tick } = await boot(fed(["a", "b", "c"]));
    const detached = [];
    const node = {
      port: {
        onmessage: null,
        postMessage(data) {
          // Cloned as a browser port does; the ring is named by its index in the attach list.
          const message = structuredClone(data);
          if (message.op !== "detach-source") return;
          detached.push(message.index);
          node.port.onmessage?.({ data: structuredClone({ op: "source-detached", requestId: message.requestId, detached: true }) });
        },
      },
      disconnect() {},
    };
    const feed = attachEngineFeed({
      context: { audioWorklet: { addModule: async () => {} } },
      sources: ["a", "b", "c"].map((sourceId) => ({ sourceId, channels: 2 })),
      quantumFrames: QUANTUM,
      engine,
      createNode: () => node,
    });
    const [a, b, c] = feed.rings;
    const untilApplied = async (revision) => {
      const applied = engine.applied(revision);
      for (let block = 0; !(await settled(applied)); block += 1) {
        assert.deepEqual(detached, [], `nothing is detached before revision ${revision} is applied (block ${block})`);
        assert.ok(block < 1_000, "the removal is applied");
        tick();
      }
      await new Promise((resolve) => setImmediate(resolve));
    };
    try {
      const removal = await engine.apply(transaction(fed(["a", "b", "c"])).removeRoute("tb.main").removeTrack("tb").removeSource("b").build());
      await untilApplied(removal.revision);
      assert.deepEqual(detached.splice(0), [1]);
      assert.deepEqual(feed.rings, [a, c]);
      const replaced = await engine.replaceSession(fed(["a"]));
      await untilApplied(replaced.revision);
      assert.deepEqual(detached.splice(0), [2]);
      assert.deepEqual(feed.rings, [a]);
    } finally {
      feed.close();
      await engine.close();
    }
  });

  test("#2100 gate 3: play() and stop() move the context and the transport together", async () => {
    // Red if `stop()` only suspends the context or only stops the transport (the two halves
    // drift apart), if `play()` leaves the transport stopped or the context suspended, if a
    // `resume()` the browser holds blocks a later `stop()`, if a refused play leaves the context
    // running when it was suspended or suspends it when it was running, if a refused play that a
    // newer play overtook restores the context, or if `close()` leaves a waiting call pending.
    const { engine, host } = await boot(base());
    const within = (promise) => Promise.race([promise, new Promise((_, reject) => {
      setTimeout(() => reject(new Error("the call did not settle")), 2_000).unref();
    })]);
    try {
      const expected = { play: ["playing", "running"], stop: ["stopped", "suspended"] };
      for (const call of ["play", "stop", "play", "stop"]) {
        const [transport, context] = expected[call];
        assert.equal((await engine[call]()).state, transport, call);
        assert.equal(engine.context.state, context, `${call} moves the context`);
        assert.equal((await engine.transport()).state, transport, `${call} moves the transport`);
      }
      // A play outside a gesture: its resume stays pending, and a later stop still settles.
      engine.context.holdResume = true;
      const held = engine.play();
      assert.equal((await within(engine.stop())).state, "stopped", "a held resume does not block stop()");
      await assert.rejects(within(held), MisoUsageError);
      assert.equal((await engine.transport()).state, "stopped");
      engine.context.holdResume = false;
      // A refused play restores the context state from before the call.
      const control = host.control;
      const refusal = new Error("transport refused");
      host.control = async () => { throw refusal; };
      await assert.rejects(engine.play(), (error) => error === refusal);
      assert.equal(engine.context.state, "suspended", "a refused play from suspended suspends again");
      host.control = control;
      // Only the newest call restores: a refused play that a newer play overtook leaves it running.
      let release;
      const slow = new Promise((resolve) => { release = resolve; });
      host.control = async () => { host.control = control; await slow; throw refusal; };
      const overtaken = engine.play();
      const newest = engine.play();
      release();
      await assert.rejects(overtaken, (error) => error === refusal);
      assert.equal((await newest).state, "playing");
      assert.equal(engine.context.state, "running", "an overtaken refusal does not suspend the newest play");
      assert.equal((await engine.transport()).state, "playing");
      assert.equal((await engine.play()).state, "playing");
      host.control = async () => { throw refusal; };
      await assert.rejects(engine.play(), (error) => error === refusal);
      assert.equal(engine.context.state, "running", "a refused play from running keeps it running");
      host.control = control;
      // A call that waits when close() runs rejects as a usage error.
      engine.context.holdResume = true;
      const waiting = engine.play();
      await engine.close();
      await assert.rejects(within(waiting), MisoUsageError);
    } finally {
      await engine.close();
    }
  });

  test("gate 3: a refused apply leaves the revision", async () => {
    // Red if a refusal advances `revision`.
    const { engine } = await boot(base());
    try {
      await assert.rejects(engine.apply(transaction(base()).setSampleRateHz(96_000).build()), (error) =>
        error instanceof MisoEngineError && error.code === "reprepareRequired");
      assert.equal(engine.revision, 0n);
    } finally {
      await engine.close();
    }
  });

  test("#2190: a host's raw miso.error.v1 rejection of a protocol frame is a MisoEngineError", async () => {
    // Red if the transport passes the host's local refusal record (not an Error) through to the app.
    const { engine, host } = await boot(base());
    const forward = host.control;
    let refuse = false;
    host.control = async (frame, replacement) => {
      if (refuse) throw Object.freeze({ tag: "miso.error.v1", requestId: 0, result: 1 });
      return forward.call(host, frame, replacement);
    };
    try {
      // The queue's probes and the capabilities read go through; the next frame, COUNTERS_GET, the
      // host refuses locally.
      await engine.capabilities();
      refuse = true;
      await assert.rejects(engine.counters(), (error) =>
        error instanceof MisoEngineError && error.code === "invalidArgument" && error.result === 1);
    } finally {
      await engine.close();
    }
  });
});
