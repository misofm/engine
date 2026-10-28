import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
const root = new URL("../", import.meta.url);
const commandArguments = process.argv.slice(2);
const bootDataModeFlags = [
  ["--boot-data-document", "document"],
].filter(([flag]) => commandArguments.includes(flag));
if (bootDataModeFlags.length > 1) throw new TypeError("only one boot-data test mode is allowed");
const bootDataTestMode = bootDataModeFlags[0]?.[1] ?? null;
const qualificationModuleIndex = commandArguments.indexOf("--qualification-module");
if (qualificationModuleIndex !== -1 && typeof commandArguments[qualificationModuleIndex + 1] !== "string") {
  throw new TypeError("--qualification-module requires a file path");
}

const abiLayout = JSON.parse(await readFile(
  new URL("../sdk/assets/miso-engine-v1-abi-layout.json", import.meta.url), "utf8",
));
// Issue #1496 D1/D3: the exports either AudioWorklet calls after construction are one table,
// `tools/realtime-policy/render-thread-exports.json`, which the realtime-policy tool also reads to
// prove each wrapped entry's body is a `render_locked` call. This suite records the calls the real
// engine worklet makes once its processor has posted `miso.ready.v1`, and fails on a call the
// table does not list for the engine worklet.
const renderThreadTable = JSON.parse(await readFile(
  new URL("../tools/realtime-policy/render-thread-exports.json", import.meta.url), "utf8",
));
assert.equal(renderThreadTable.table, "render-thread-exports");
const renderThreadPrefix = renderThreadTable.prefix;
assert.equal(typeof renderThreadPrefix, "string");
const engineRenderThreadExports = new Set(renderThreadTable.exports
  .filter((entry) => entry.callers.some((caller) => caller.worklet === "engine"))
  .map((entry) => entry.name));
const unsupportedPreparationModuleBytes = Uint8Array.from([
  0, 97, 115, 109, 1, 0, 0, 0,
  1, 5, 1, 96, 0, 1, 127,
  3, 2, 1, 0,
  5, 3, 1, 0, 1,
  7, 46, 2, 6, 109, 101, 109, 111, 114, 121, 2, 0,
  33, 109, 105, 115, 111, 95, 101, 110, 103, 105, 110, 101, 95, 119, 101, 98,
  95, 118, 49, 95, 99, 111, 109, 109, 97, 110, 100, 95, 115, 117, 98, 109, 105, 116, 0, 0,
  10, 6, 1, 4, 0, 65, 7, 11,
]);
// Issue #151: the host module is overridable for exactly the reason the worklet module already is
// -- so a red mutation of the shipped host runs this same suite and is required to fail it.
const hostUrl = process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet-host.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE);
const workletUrl = process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE);
const qualificationUrl = qualificationModuleIndex === -1
  ? new URL("hosts/host-web/qualification/qualification.js", root)
  : pathToFileURL(commandArguments[qualificationModuleIndex + 1]);

const limits = Object.freeze({
  sourceRingFrames: 256,
  maximumMemoryBytes: 0n,
  // Issue #137 D1/D2: the live-control meter word. `reserved1` (offset 32, issue #1565),
  // `reserved3` (offset 48, the observation-taps word until issue #1590) and `reserved2` (offset
  // 56, the master boot word until issue #1689) must be zero; the live-control tests below
  // override the meter word.
  reserved1: 0n,
  liveControlMeterBlocks: 2n,
  reserved3: 0n,
  reserved2: 0n,
});

function resourceReport(backend, quantumFrames) {
  return Object.freeze({
    sampleRateHz: 48000,
    quantumFrames,
    backend,
    optionsBytes: 1n,
    statusBytes: 1n,
    sessionDocumentBytes: 1n,
    diagnosticBytes: 1n,
    idStagingBytes: 1n,
    sourcePcmStagingBytes: 1n,
    outputPcmBytes: 1n,
    bridgeMetadataBytes: 1n,
    bridgeRetainedBytes: 1n,
    largestBridgeAllocationBytes: 1n,
    sourceTotalBytes: 1n,
    sourceOverheadBytes: 1n,
    effectScalarStateBytes: 1n,
    effectScalarScratchBytes: 1n,
    builtinRetainedBytes: 1n,
    graphSessionPlusPlanBytes: 1n,
    graphIncrementalPlanBytes: 1n,
    graphMetadataBytes: 1n,
    graphDelayBytes: 1n,
    largestNamedAllocationBytes: 1n,
    observationRetainedBytes: 1n,
  });
}

function errorResult(promise, result) {
  return promise.then(
    () => assert.fail("expected rejection"),
    (error) => {
      assert.deepEqual(Object.keys(error).sort(), ["requestId", "result", "tag"]);
      assert.equal(error.tag, "miso.error.v1");
      assert.equal(error.result, result);
      return error;
    },
  );
}

async function localErrorResult(promise, result) {
  const error = await errorResult(promise, result);
  assert.equal(error.requestId, 0, "local refusal carries no allocated ID");
  return error;
}

async function testMainRealm() {
  const original = {
    fetch: globalThis.fetch,
    AudioWorkletNode: globalThis.AudioWorkletNode,
    validate: WebAssembly.validate,
    compile: WebAssembly.compile,
    setInterval: globalThis.setInterval,
    clearInterval: globalThis.clearInterval,
  };
  // Issue #1381a D6: the host's one timer, the single-mode service tick, is driven by hand, so a
  // test posts exactly the ticks it means to and no real interval keeps the process alive.
  const intervals = [];
  globalThis.setInterval = (callback, delay) => {
    intervals.push({ callback, delay, cleared: false });
    return intervals.length;
  };
  globalThis.clearInterval = (id) => {
    if (intervals[id - 1] !== undefined) intervals[id - 1].cleared = true;
  };
  const events = [];
  let fetchCount = 0;
  let holdSource = false;
  let holdAll = false;
  const heldAll = [];
  let failSource = false;
  let held = null;
  let readyMutation = null;
  let statusMutation = null;
  let renderAllocationsMutation = null;
  let controlMutation = null;
  let lastControlDocument;
  let sessionMapMutation = null;
  let planeMutation = null;
  // Issue #1644: every session seek message the fake port received, and the result it answers.
  const sessionSeekMessages = [];
  let sessionSeekResult = 0;
  let sessionSeekGenerationOffset = 0n;
  // Issue #1297a: every seek-at and seek report message the fake port received, and the reply a
  // seek report gets (a function of the received message).
  const sourceSeekMessages = [];
  let seekReportReply = null;
  let mixedSuccess = false;
  // The result the fake worklet answers a dispose with (0: a clean dispose).
  let disposeResult = 0;
  let compileGate = null;
  let addModuleGate = null;
  let readyGate = null;

  const snapshotGate = () => {
    let release;
    let markStarted;
    const promise = new Promise((resolve) => { release = resolve; });
    const started = new Promise((resolve) => { markStarted = resolve; });
    return { promise, started, start: markStarted, release };
  };

  class FakePort {
    onmessage = null;
    onmessageerror = null;
    closeCount = 0;

    close() {
      this.closeCount += 1;
    }

    postMessage(message, transfer = []) {
      const received = structuredClone(message, { transfer });
      if (received.tag === "miso.control-port.v1") {
        // Issue #1332 S7: the control Worker's channel port reaches the worklet; no reply.
        events.push(["control-port"]);
        return;
      }
      if (received.tag === "miso.service.v1") {
        // The service tick is no request: it carries no request ID and gets no reply.
        events.push(["service"]);
        return;
      }
      events.push(["request", received.tag, received.requestId, transfer.length]);
      const respond = () => {
        let response;
        let responseTransfer = [];
        if (received.tag === "miso.source.v1") {
          response = {
            tag: failSource ? "miso.error.v1" : "miso.ack.v1",
            requestId: received.requestId,
            result: failSource ? 1 : (mixedSuccess ? 0 : 6),
            planes: received.planes,
          };
          responseTransfer = [...new Set(received.planes.map((plane) => plane.buffer))];
        } else if (received.tag === "miso.status.v1") {
          response = {
            tag: "miso.status.v1", requestId: received.requestId, result: 0, state: 2,
            lastResult: 0, backend: 1, sampleRateHz: 48000, quantumFrames: 64, blockingRebuilds: 1,
            nextAbsoluteSample: 64n, renderedQuanta: 1n, liveValuesSuperseded: 7n,
            appliedRevision: 9n, appliedSample: 64n, appliedOutcome: 1n,
            memoryBytes: 65536, singleModeControlAllocations: 3, staleViewQuanta: 0,
            meterFramesSkipped: 0,
          };
          if (statusMutation !== null) response = statusMutation(response);
        } else if (received.tag === "miso.renderallocations.v1") {
          response = {
            tag: "miso.renderallocations.v1", requestId: received.requestId, result: 0, count: 0,
          };
          if (renderAllocationsMutation !== null) response = renderAllocationsMutation(response);
        } else if (received.tag === "miso.control.v1") {
          // Issue #1562a D1: the frame comes back reversed as the response, so the test can see
          // that the host sent the caller's bytes and returns the engine's unchanged.
          const frame = received.frame;
          lastControlDocument = received.document;
          response = {
            tag: "miso.control.v1", requestId: received.requestId, result: 0,
            response: new Uint8Array([...frame].reverse()), revision: 2n,
          };
          if (controlMutation !== null) response = controlMutation(response);
          responseTransfer = response.response instanceof Uint8Array ? [response.response.buffer] : [];
        } else if (received.tag === "miso.sessionmap.v1") {
          response = {
            tag: "miso.sessionmap.v1",
            requestId: received.requestId,
            result: 0,
            sources: [
              { id: "bass", channels: 1, frames: 96000n },
              { id: "drums", channels: 2, frames: 2048n },
            ],
            metersAttached: true,
          };
          if (sessionMapMutation !== null) response = sessionMapMutation(response);
        } else if (received.tag === "miso.session-seek.v1") {
          sessionSeekMessages.push(received);
          response = {
            tag: "miso.session-seek.v1", requestId: received.requestId,
            result: sessionSeekResult, generation: received.generation + sessionSeekGenerationOffset,
          };
        } else if (received.tag === "miso.seek-report.v1") {
          sourceSeekMessages.push(received);
          response = { tag: "miso.seek-report.v1", ...seekReportReply(received) };
        } else {
          if (received.tag === "miso.seek-at.v1") sourceSeekMessages.push(received);
          response = {
            tag: "miso.ack.v1", requestId: received.requestId,
            result: received.tag === "miso.dispose.v1" ? disposeResult : 0,
          };
        }
        if (received.tag === "miso.source.v1" && planeMutation !== null) {
          response = planeMutation(response);
        }
        const delivered = structuredClone(response, { transfer: responseTransfer });
        queueMicrotask(() => this.onmessage?.({ data: delivered }));
      };
      if (holdAll) heldAll.push(respond);
      else if (holdSource && received.tag === "miso.source.v1") held = respond;
      else respond();
    }
  }

  class FakeNode {
    static latest;
    static count = 0;

    constructor(context, _name, options) {
      FakeNode.count += 1;
      this.port = new FakePort();
      this.onprocessorerror = null;
      this.options = options;
      this.constructionQuantumFrames = context.renderQuantumSize ?? 128;
      this.constructionContext = context;
      this.constructionModule = options.processorOptions.module;
      if (bootDataTestMode !== null) {
        const constructorDocument = options.processorOptions.document;
        this.constructionDocumentShape = {
          byteOffset: constructorDocument.byteOffset,
          byteLength: constructorDocument.byteLength,
          buffer: constructorDocument.buffer,
        };
        this.constructionSnapshot = {
          document: new Uint8Array(constructorDocument),
          module: options.processorOptions.module,
          options: structuredClone(options.processorOptions.options),
        };
      }
      this.disposeMessages = 0;
      this.disconnectCount = 0;
      FakeNode.latest = this;
      queueMicrotask(async () => {
        let data = options.processorOptions.instanceMode === "worker" ? {
          tag: "miso.ready.v1", requestId: 0, result: 0, backend: "simd128", instanceMode: "worker",
        } : {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: "simd128",
          resources: resourceReport(1, this.constructionQuantumFrames),
          memoryBytes: 65536,
          // Issue #1406b: the ring the engine resolved. The fake engine reports an override as
          // given and, for a zero override, 110 quanta: the engine's default ring at 48 kHz / 64.
          sourceRingFrames: options.processorOptions.options.sourceRingFrames
            || 110 * this.constructionQuantumFrames,
          instanceMode: "single",
        };
        if (readyGate !== null) {
          const gate = readyGate;
          readyGate = null;
          gate.start();
          await gate.promise;
        }
        if (readyMutation !== null) data = readyMutation(data);
        this.port.onmessage?.({ data });
      });
    }

    disconnect() {
      this.disconnectCount += 1;
    }
  }

  globalThis.AudioWorkletNode = FakeNode;
  WebAssembly.validate = () => true;
  globalThis.fetch = async (url) => {
    fetchCount += 1;
    return {
      ok: true,
      arrayBuffer: async () => new TextEncoder().encode(String(url)).buffer,
      json: async () => abiLayout,
    };
  };
  const unsupportedPreparationModule = await original.compile(unsupportedPreparationModuleBytes);
  WebAssembly.compile = async (bytes) => {
    const url = new TextDecoder().decode(bytes);
    events.push(["compile", url]);
    if (compileGate !== null) {
      const gate = compileGate;
      compileGate = null;
      gate.start();
      await gate.promise;
    }
    return unsupportedPreparationModule;
  };
  const context = {
    state: "suspended",
    sampleRate: 48000,
    renderQuantumSize: 64,
    audioWorklet: {
      addModule: async (url) => {
        events.push(["addModule", url]);
        if (addModuleGate !== null) {
          const gate = addModuleGate;
          addModuleGate = null;
          gate.start();
          await gate.promise;
        }
      },
    },
  };
  try {
    const { createMisoAudioWorkletHost } = await import(`${hostUrl.href}?main-test`);

    const bootDataFactory = ({
      document = new Uint8Array([0x7b, 0x22, 0x73, 0x22, 0x7d]),
      options = limits,
      includePreparedModule = false,
      workletModuleUrl = "processor.js",
    } = {}) => {
      const factory = {
        context,
        document,
        options,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl,
      };
      if (includePreparedModule) factory.preparedModule = unsupportedPreparationModule;
      return factory;
    };

    const runBootDataDocument = async () => {
      const gate = snapshotGate();
      compileGate = gate;
      const backing = new ArrayBuffer(16);
      const document = new Uint8Array(backing, 3, 5);
      document.set([11, 22, 33, 44, 55]);
      const originalVisible = [...document];
      const originalWorkletUrl = "boot-data-document-processor.js";
      const factory = bootDataFactory({ document, workletModuleUrl: originalWorkletUrl });
      const eventsBefore = events.length;
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        document[1] = 99;
        factory.workletModuleUrl = "mutated-during-compile.js";
        gate.release();
        hostValue = await boot;
        const node = FakeNode.latest;
        assert.deepEqual(
          [...node.constructionSnapshot.document],
          originalVisible,
          "boot-data.document.before-construction",
        );
        assert.equal(node.constructionDocumentShape.byteOffset, 0, "boot-data.document.offset-zero");
        assert.equal(
          node.constructionDocumentShape.byteLength,
          originalVisible.length,
          "boot-data.document.visible-length",
        );
        assert.notEqual(
          node.constructionDocumentShape.buffer,
          backing,
          "boot-data.document.private-storage",
        );
        assert.equal(node.constructionContext, context, "boot-data.document.context-identity");
        assert.equal(node.constructionModule, unsupportedPreparationModule, "boot-data.document.module");
        assert.deepEqual(
          events.slice(eventsBefore),
          [["compile", "simd.wasm"], ["addModule", originalWorkletUrl]],
          "boot-data.document.captured-loading-inputs",
        );
      } finally {
        gate.release();
        if (compileGate === gate) compileGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }
    };

    if (bootDataTestMode === "document") {
      await runBootDataDocument();
      return;
    }

    const runPlainDataMatrix = async () => {
      const makeFactory = (overrides = {}) => ({
        context, document: new Uint8Array(), options: limits,
        simd128ModuleUrl: "simd.wasm", workletModuleUrl: "processor.js", ...overrides,
      });
      const addWitness = (object, key, setterOnly = false, enumerable = true) => {
        let calls = 0;
        const descriptor = setterOnly
          ? { configurable: true, enumerable, set() { calls += 1; } }
          : { configurable: true, enumerable, get() {
            calls += 1; throw new Error(`unexpected ${String(key)} read`);
          } };
        Object.defineProperty(object, key, descriptor);
        return () => calls;
      };
      const assertRefusal = async (label, factory, witness = () => 0) => {
        const before = [events.length, fetchCount, FakeNode.count];
        await localErrorResult(createMisoAudioWorkletHost(factory), 1);
        assert.equal(witness(), 0, `${label}: witness or callback invoked`);
        assert.equal(fetchCount, before[1], `${label}: fetch occurred`);
        assert.equal(events.length, before[0], `${label}: compile/addModule occurred`);
        assert.equal(FakeNode.count, before[2], `${label}: node constructed`);
      };
      const assertAccepted = async (label, factory, check = () => {}) => {
        let host;
        let node;
        try {
          host = await createMisoAudioWorkletHost(factory);
          node = FakeNode.latest;
          check(host, node.options.processorOptions.options);
        } finally {
          if (host !== undefined) {
            await host.dispose();
            assert.equal(node.disconnectCount, 1, `${label}: host was disposed`);
          }
        }
      };

      const descriptorCases = [
        ["factory", () => { const f = makeFactory(); return [f, f, "context"]; }],
        ["boot", () => { const o = { ...limits }; return [makeFactory({ options: o }), o, "sourceRingFrames"]; }],
      ];
      for (const setterOnly of [false, true]) {
        for (const [label, makeCase] of descriptorCases) {
          const [factory, object, key] = makeCase();
          await assertRefusal(`${setterOnly ? "setter-only" : "accessor"} ${label}`, factory,
            addWitness(object, key, setterOnly));
        }
      }
      for (const setterOnly of [false, true]) {
        const factory = makeFactory();
        await assertRefusal(`hidden ${setterOnly ? "setter-only" : "accessor"}`, factory,
          addWitness(factory, "context", setterOnly, false));
      }

      const refusalCases = [
        ["unknown enumerable", () => Object.assign(makeFactory(), { unknown: undefined })],
        ["unknown hidden", () => {
          const factory = makeFactory();
          Object.defineProperty(factory, "unknown", { value: undefined });
          return factory;
        }],
        ["unknown symbol", () => {
          const factory = makeFactory();
          Object.defineProperty(factory, Symbol("unknown"), { value: undefined });
          return factory;
        }],
      ];
      for (const [label, makeCase] of refusalCases) {
        await assertRefusal(label, makeCase());
      }

      const literalOptions = { ...limits };
      await assertAccepted("same-realm literal records", makeFactory({ options: literalOptions }));
      {
        const options = Object.assign(Object.create(null), limits);
        const factory = Object.assign(Object.create(null), makeFactory({ options }));
        await assertAccepted("null-prototype records", factory);
      }
      {
        const options = Object.freeze({ ...limits });
        await assertAccepted("frozen records", Object.freeze(makeFactory({ options })));
      }
      {
        const enumerableUndefined = makeFactory();
        enumerableUndefined.preparedModule = undefined;
        await assertRefusal("enumerable undefined prepared module", enumerableUndefined);
        const enumerableNull = makeFactory();
        enumerableNull.preparedModule = null;
        await assertRefusal("enumerable null prepared module", enumerableNull);
        const hiddenUndefined = makeFactory();
        Object.defineProperty(hiddenUndefined, "preparedModule", { value: undefined });
        await assertAccepted("hidden undefined prepared module", hiddenUndefined);
        const hiddenDefined = makeFactory();
        Object.defineProperty(hiddenDefined, "preparedModule", {
          value: unsupportedPreparationModule,
        });
        await assertRefusal("hidden defined prepared module", hiddenDefined);
      }
      const plainOptions = (overrides = {}) => ({ ...limits, ...overrides });
      const domainRefusals = [
        ["u32 type", plainOptions({ sourceRingFrames: "256" })],
        ["u32 bound", plainOptions({ sourceRingFrames: 0x1_0000_0000 })],
        ["u64 type", plainOptions({ maximumMemoryBytes: 0 })],
        ["u64 bound", plainOptions({ maximumMemoryBytes: 0x1_0000_0000_0000_0000n })],
        ["reserved1 nonzero", plainOptions({ reserved1: 1n })],
        ["reserved1 type", plainOptions({ reserved1: 0 })],
        // Issue #1590 gate 4: the retired observation-taps word is refused.
        ["reserved3 nonzero", plainOptions({ reserved3: 4n })],
        // Issue #1689 gate 2: the retired master word is refused at both ends of its range.
        ["reserved2 one", plainOptions({ reserved2: 1n })],
        ["reserved2 all ones", plainOptions({ reserved2: 0xffff_ffff_ffff_ffffn })],
      ];
      for (const [label, options] of domainRefusals) {
        await assertRefusal(label, makeFactory({ options }));
      }
      {
        const savedQuantum = context.renderQuantumSize;
        try {
          delete context.renderQuantumSize;
          await assertAccepted(
            "default context quantum",
            makeFactory(),
            (host) => {
              assert.equal(host.resources.quantumFrames, 128, "default quantum is captured");
              assert.equal(FakeNode.latest.constructionQuantumFrames, 128,
                "construction uses captured default quantum");
            },
          );
        } finally {
          context.renderQuantumSize = savedQuantum;
        }
      }

      await assertRefusal("SharedArrayBuffer document", makeFactory({
        document: new Uint8Array(new SharedArrayBuffer(4)),
      }));
      {
        const backing = new ArrayBuffer(4);
        const document = new Uint8Array(backing);
        structuredClone(backing, { transfer: [backing] });
        await assertRefusal("detached document", makeFactory({ document }));
      }
      {
        let iteratorCalls = 0;
        const forged = Object.create(Uint8Array.prototype);
        Object.defineProperty(forged, Symbol.iterator, {
          configurable: true,
          get() {
            iteratorCalls += 1;
            throw new Error("forged document iterator invoked");
          },
        });
        await assertRefusal("forged document", makeFactory({ document: forged }), () => iteratorCalls);
      }
      {
        const backing = new ArrayBuffer(8);
        const document = new Uint8Array(backing, 2, 3);
        document.set([11, 22, 33]);
        const originalVisible = [document[0], document[1], document[2]];
        let decorationCalls = 0;
        const decoration = {
          configurable: true,
          get() {
            decorationCalls += 1;
            throw new Error("document decoration invoked");
          },
        };
        Object.defineProperty(document, "buffer", decoration);
        Object.defineProperty(document, "constructor", decoration);
        Object.defineProperty(document, Symbol.toStringTag, decoration);
        Object.defineProperty(document, Symbol.iterator, decoration);
        Object.defineProperty(backing, "slice", decoration);
        const gate = snapshotGate();
        compileGate = gate;
        let hostValue;
        try {
          const boot = createMisoAudioWorkletHost(makeFactory({ document }));
          await gate.started;
          document[1] = 99;
          gate.release();
          hostValue = await boot;
          const copied = FakeNode.latest.options.processorOptions.document;
          assert.deepEqual([...copied], originalVisible, "decorated document copied before mutation");
          assert.equal(copied.byteOffset, 0, "decorated document offset-zero copy");
          assert.equal(copied.byteLength, originalVisible.length, "decorated document length");
          assert.notEqual(copied.buffer, backing, "decorated document private copy");
          assert.equal(decorationCalls, 0, "decorated document hooks were not invoked");
        } finally {
          gate.release();
          if (compileGate === gate) compileGate = null;
          if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
        }
      }
      {
        const emptyHost = await createMisoAudioWorkletHost(makeFactory({ document: new Uint8Array() }));
        await emptyHost.dispose();
      }
      // #2166 parity row: the host hands the worklet a private copy of the boot command, and
      // refuses one that is not a nonempty Uint8Array.
      {
        const bootCommand = new Uint8Array([3, 1, 4]);
        const commanded = await createMisoAudioWorkletHost(makeFactory({ bootCommand }));
        const copied = FakeNode.latest.options.processorOptions.bootCommand;
        assert.deepEqual([...copied], [3, 1, 4], "the boot command reaches the worklet");
        assert.notEqual(copied.buffer, bootCommand.buffer, "the boot command is a private copy");
        await commanded.dispose();
        await assertRefusal("empty boot command", makeFactory({ bootCommand: new Uint8Array() }));
        await assertRefusal("array boot command", makeFactory({ bootCommand: [3, 1, 4] }));
      }

      {
        let audioWorkletReads = 0;
        const unsupportedContext = Object.create(context);
        Object.defineProperty(unsupportedContext, "audioWorklet", {
          configurable: true,
          get() {
            audioWorkletReads += 1;
            throw new Error("audioWorklet accessed before SIMD refusal");
          },
        });
        const beforeEvents = events.length;
        const beforeFetches = fetchCount;
        const beforeNodes = FakeNode.count;
        const validate = WebAssembly.validate;
        try {
          WebAssembly.validate = () => false;
          const refusal = await createMisoAudioWorkletHost(makeFactory({
            context: unsupportedContext,
          })).then(() => assert.fail("expected unsupported SIMD refusal"), (error) => error);
          assert.deepEqual(Object.keys(refusal).sort(), ["capability", "requestId", "result", "tag"]);
          assert.equal(refusal.tag, "miso.unsupported.v1");
          assert.equal(refusal.capability, "simd128");
          assert.equal(refusal.result, 7);
          assert.equal(refusal.requestId, 0);
          assert.equal(audioWorkletReads, 0, "unsupported SIMD touched audioWorklet");
          assert.equal(fetchCount, beforeFetches, "unsupported SIMD fetched a module");
          assert.equal(events.length, beforeEvents, "unsupported SIMD compiled or loaded a module");
          assert.equal(FakeNode.count, beforeNodes, "unsupported SIMD constructed a node");
        } finally {
          WebAssembly.validate = validate;
        }
      }
      for (const [label, mutate, restore] of [
        ["state", () => { context.state = "running"; }, () => { context.state = "suspended"; }],
        ["sample rate", () => { context.sampleRate = 44100; }, () => { context.sampleRate = 48000; }],
        ["quantum", () => { context.renderQuantumSize = 128; }, () => { context.renderQuantumSize = 64; }],
      ]) {
        const gate = snapshotGate();
        addModuleGate = gate;
        const beforeEvents = events.length;
        const beforeNodes = FakeNode.count;
        let boot;
        try {
          boot = createMisoAudioWorkletHost(makeFactory());
          await gate.started;
          mutate();
          gate.release();
          await localErrorResult(boot, 1);
          assert.equal(FakeNode.count, beforeNodes, `${label} drift constructed a node`);
          assert.deepEqual(
            events.slice(beforeEvents),
            [["compile", "simd.wasm"], ["addModule", "processor.js"]],
            `${label} drift was checked after loading and before construction`,
          );
        } finally {
          restore();
          gate.release();
          if (addModuleGate === gate) addModuleGate = null;
        }
      }

      {
        const addGate = snapshotGate();
        const readyPause = snapshotGate();
        const options = {
          sourceRingFrames: 0,
          maximumMemoryBytes: 0n,
          reserved1: 0n,
          liveControlMeterBlocks: 7n,
          reserved3: 0n,
          reserved2: 0n,
        };
        const expectedOptions = { ...options };
        const factory = makeFactory({ options, workletModuleUrl: "single-boot.js" });
        const originalFactory = {
          context: factory.context,
          options: factory.options,
          simd128ModuleUrl: factory.simd128ModuleUrl,
          workletModuleUrl: factory.workletModuleUrl,
          hasPreparedModule: Object.hasOwn(factory, "preparedModule"),
          preparedModule: factory.preparedModule,
        };
        const originalContext = {
          state: context.state,
          sampleRate: context.sampleRate,
          renderQuantumSize: context.renderQuantumSize,
        };
        const replacementModule = await original.compile(unsupportedPreparationModuleBytes);
        const replacementContext = {
          state: "suspended", sampleRate: 44100, renderQuantumSize: 128,
          audioWorklet: { addModule: async () => undefined },
        };
        let hostValue;
        let boot;
        const previousMixedSuccess = mixedSuccess;
        try {
          addModuleGate = addGate;
          readyGate = readyPause;
          boot = createMisoAudioWorkletHost(factory);
          await addGate.started;
          options.sourceRingFrames = 128;
          options.liveControlMeterBlocks = 99n;
          factory.options = { ...options, reserved3: 4n };
          factory.context = replacementContext;
          factory.preparedModule = replacementModule;
          factory.simd128ModuleUrl = "mutated-simd.wasm";
          factory.workletModuleUrl = "mutated-processor-before-construction.js";
          addGate.release();
          await readyPause.started;
          context.state = "running";
          context.sampleRate = 44100;
          context.renderQuantumSize = 128;
          options.sourceRingFrames = 256;
          options.liveControlMeterBlocks = 101n;
          factory.options = { ...options, reserved3: 8n };
          readyPause.release();
          hostValue = await boot;
          const node = FakeNode.latest;
          const captured = node.options.processorOptions.options;
          assert.equal(hostValue.resources.sampleRateHz, 48000);
          assert.equal(hostValue.resources.quantumFrames, 64);
          assert.deepEqual(captured, expectedOptions, "boot values are captured");
          assert.notEqual(captured, options, "boot snapshot is fresh");
          assert.deepEqual(
            events.slice(-2),
            [["compile", "simd.wasm"], ["addModule", "single-boot.js"]],
            "loading inputs are captured",
          );
          const status = await hostValue.status();
          assert.equal(status.sampleRateHz, 48000, "status keeps captured sample rate");
          assert.equal(status.quantumFrames, 64, "status keeps captured quantum");
          assert.equal(node.constructionContext, context, "construction keeps context identity");
          assert.equal(node.constructionModule, unsupportedPreparationModule,
            "construction keeps compiled module identity");
          assert.equal(node.constructionQuantumFrames, 64,
            "construction keeps captured quantum after context mutation");

          mixedSuccess = true;
          holdAll = true;
          // Issue #1406b: with a zero override the bound is the ring the engine reports at boot,
          // 110 quanta at 48 kHz / 64 (the old host rule gave 77).
          const heldSources = Array.from({ length: 110 }, (_value, index) => {
            const buffer = new ArrayBuffer(4);
            const request = hostValue.submitSource({
              sourceId: "single-source", generation: 1n, startFrame: BigInt(index),
              sampleRateHz: 48000, planes: [new Float32Array(buffer)], frames: 1,
              endOfRegion: false,
            });
            assert.equal(buffer.byteLength, 0, `single source ${index} transferred`);
            return request;
          });
          const beforeSourceOverflow = events.length;
          const overflowBuffer = new ArrayBuffer(4);
          await localErrorResult(hostValue.submitSource({
            sourceId: "single-source", generation: 1n, startFrame: 110n,
            sampleRateHz: 48000, planes: [new Float32Array(overflowBuffer)], frames: 1,
            endOfRegion: false,
          }), 6);
          assert.equal(events.length, beforeSourceOverflow, "source capacity refusal posted no event");
          assert.equal(overflowBuffer.byteLength, 4, "source refusal retained caller storage");
          holdAll = false;
          for (const respond of heldAll.splice(0)) respond();
          const sourceAcks = await Promise.all(heldSources);
          assert(sourceAcks.every((ack) => ack.result === 0), "all default-depth sources accepted");
          const nextSourceBuffer = new ArrayBuffer(4);
          const nextSource = hostValue.submitSource({
            sourceId: "single-source", generation: 1n, startFrame: 111n,
            sampleRateHz: 48000, planes: [new Float32Array(nextSourceBuffer)], frames: 1,
            endOfRegion: false,
          });
          const nextSourceAck = await nextSource;
          assert.equal(nextSourceAck.requestId, sourceAcks.at(-1).requestId + 1,
            "source capacity refusal did not burn an ID");

        } finally {
          addGate.release();
          readyPause.release();
          if (addModuleGate === addGate) addModuleGate = null;
          if (readyGate === readyPause) readyGate = null;
          holdAll = false;
          for (const respond of heldAll.splice(0)) respond();
          mixedSuccess = previousMixedSuccess;
          context.state = originalContext.state;
          context.sampleRate = originalContext.sampleRate;
          context.renderQuantumSize = originalContext.renderQuantumSize;
          factory.context = originalFactory.context;
          factory.options = originalFactory.options;
          factory.simd128ModuleUrl = originalFactory.simd128ModuleUrl;
          factory.workletModuleUrl = originalFactory.workletModuleUrl;
          if (originalFactory.hasPreparedModule) factory.preparedModule = originalFactory.preparedModule;
          else delete factory.preparedModule;
          if (boot !== undefined && hostValue === undefined) await boot.catch(() => undefined);
          if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
        }
      }

      {
        const gate = snapshotGate();
        addModuleGate = gate;
        const options = { ...limits, sourceRingFrames: 128 };
        const factory = makeFactory({ options, workletModuleUrl: "explicit-depth.js" });
        let boot;
        let hostValue;
        const previousMixedSuccess = mixedSuccess;
        try {
          boot = createMisoAudioWorkletHost(factory);
          await gate.started;
          options.sourceRingFrames = 256;
          factory.options = { ...options };
          gate.release();
          hostValue = await boot;
          mixedSuccess = true;
          holdAll = true;
          const heldSources = Array.from({ length: 2 }, (_value, index) => {
            const buffer = new ArrayBuffer(4);
            const request = hostValue.submitSource({
              sourceId: "explicit-source", generation: 1n, startFrame: BigInt(index),
              sampleRateHz: 48000, planes: [new Float32Array(buffer)], frames: 1,
              endOfRegion: false,
            });
            assert.equal(buffer.byteLength, 0, `explicit source ${index} transferred`);
            return request;
          });
          const beforeOverflow = events.length;
          const overflowBuffer = new ArrayBuffer(4);
          await localErrorResult(hostValue.submitSource({
            sourceId: "explicit-source", generation: 1n, startFrame: 2n,
            sampleRateHz: 48000, planes: [new Float32Array(overflowBuffer)], frames: 1,
            endOfRegion: false,
          }), 6);
          assert.equal(events.length, beforeOverflow,
            "explicit source capacity refusal posted no event");
          assert.equal(overflowBuffer.byteLength, 4,
            "explicit source refusal retained caller storage");
          holdAll = false;
          for (const respond of heldAll.splice(0)) respond();
          const sourceAcks = await Promise.all(heldSources);
          assert.equal(sourceAcks.length, 2, "explicit source depth accepts exactly two held submissions");
          assert(sourceAcks.every((ack) => ack.result === 0),
            "explicit source depth held submissions accepted");
          const nextBuffer = new ArrayBuffer(4);
          const nextAck = await hostValue.submitSource({
            sourceId: "explicit-source", generation: 1n, startFrame: 3n,
            sampleRateHz: 48000, planes: [new Float32Array(nextBuffer)], frames: 1,
            endOfRegion: false,
          });
          assert.equal(nextAck.requestId, sourceAcks.at(-1).requestId + 1,
            "explicit source refusal did not burn an ID");
        } finally {
          gate.release();
          if (addModuleGate === gate) addModuleGate = null;
          holdAll = false;
          for (const respond of heldAll.splice(0)) respond();
          mixedSuccess = previousMixedSuccess;
          if (boot !== undefined && hostValue === undefined) await boot.catch(() => undefined);
          if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
        }
      }
    };

    const host = await createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode("{\"schema_version\":0}"),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    assert.equal(host.backend, "simd128", "W4-D1 ships exactly one artifact");
    assert.equal(host.memoryBytes, 65536);
    assert.equal(Object.getOwnPropertyDescriptor(host, "memoryBytes").writable, false);
    assert.deepEqual(events.slice(0, 2), [
      ["compile", "simd.wasm"], ["addModule", "processor.js"],
    ]);
    if (process.env.MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST === "1") {
      const last = await host.status();
      assert.equal(last.requestId, Number.MAX_SAFE_INTEGER);
      await localErrorResult(host.status(), 1);
      await localErrorResult(host.status(), 1);
      assert.deepEqual(
        events.filter((event) => event[0] === "request").map((event) => event[2]),
        [Number.MAX_SAFE_INTEGER],
        "safe-integer exhaustion never posts or wraps",
      );
      await host.dispose().catch(() => undefined);
      return;
    }

    const storage = new ArrayBuffer(32);
    const left = new Float32Array(storage, 0, 2);
    const right = new Float32Array(storage, 16, 2);
    left.set([1, 2]);
    right.set([3, 4]);
    holdSource = true;
    const sourcePromise = host.submitSource({
      sourceId: "source", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [left, right], frames: 2, endOfRegion: false,
    });
    assert.equal(storage.byteLength, 0, "postMessage transfers caller ownership");
    // #106 F3: a held source chunk no longer blocks unrelated requests. Before the bounded
    // pipeline this rejected with RESULT_BACKPRESSURE because the host allowed one request of any
    // kind in flight.
    const heldStatus = host.status();
    held();
    holdSource = false;
    assert.equal((await heldStatus).tag, "miso.status.v1");
    const ack = await sourcePromise;
    assert.deepEqual(Object.keys(ack).sort(), ["planes", "requestId", "result", "tag"]);
    assert.equal(ack.tag, "miso.ack.v1");
    assert.equal(ack.result, 6, "engine backpressure remains a resolved ACK");
    assert.equal(ack.planes[0].byteOffset, 0);
    assert.equal(ack.planes[1].byteOffset, 16);
    assert.equal(ack.planes[0].buffer, ack.planes[1].buffer);
    assert.equal(events.find((event) => event[1] === "miso.source.v1")[3], 1);

    // Request 2 was consumed by the status above, which now settles independently.
    const seek = await host.seekSource({
      sourceId: "source", generation: 2n, sourceFrame: 10n,
    });
    assert.deepEqual(seek, { tag: "miso.ack.v1", requestId: 3, result: 0 });
    const status = await host.status();
    assert.equal(status.tag, "miso.status.v1");
    assert.equal(status.memoryBytes, host.memoryBytes);
    assert.equal(status.liveValuesSuperseded, 7n, "the status reply carries the superseded count");
    assert.deepEqual(
      [status.appliedRevision, status.appliedSample, status.appliedOutcome],
      [9n, 64n, 1n],
      "the status reply carries the applied-revision watermark",
    );
    assert.equal(
      status.singleModeControlAllocations,
      3,
      "the status reply carries the single-mode control allocation count",
    );
    assert.equal(status.staleViewQuanta, 0, "the status reply carries the stale-view quantum count");

    // Issue #1562a D1, gate 7's host half: a control frame is copied, posted whole as one
    // `miso.control.v1` message with its buffer transferred, and the reply is the engine's
    // response bytes unchanged with the committed revision. The host decodes nothing in the frame.
    {
      const frame = new Uint8Array([1, 2, 3]);
      const reply = await host.control(frame);
      assert.equal(reply.tag, "miso.control.v1");
      assert.equal(reply.result, 0);
      assert.deepEqual([...reply.response], [3, 2, 1]);
      assert.equal(reply.revision, 2n);
      assert.equal(frame.byteLength, 3, "the caller keeps its frame: the host sends a copy");
      const posted = events.filter((event) => event[1] === "miso.control.v1");
      assert.equal(posted.length, 1);
      assert.equal(posted[0][3], 1, "the frame's buffer is transferred");
      await localErrorResult(host.control(new Uint8Array(0)), 1);
      await localErrorResult(host.control(new Uint8Array(16385)), 1);
      await localErrorResult(host.control([1, 2]), 1);
      assert.equal(
        events.filter((event) => event[1] === "miso.control.v1").length,
        1,
        "a refused frame never reaches the port",
      );
      const largest = await host.control(new Uint8Array(16384));
      assert.equal(largest.response.byteLength, 16384, "the largest frame is sent");
      // Issue #1697: a document is copied and posted beside the frame, both buffers transferred;
      // an empty or non-array document is refused before the port.
      const document = new Uint8Array([1, 2, 3, 4]);
      await host.control(new Uint8Array([7]), document);
      const withDocument = events.filter((event) => event[1] === "miso.control.v1");
      assert.equal(withDocument.length, 3);
      assert.equal(withDocument[2][3], 2, "the frame's and the document's buffers are transferred");
      assert.deepEqual([...lastControlDocument], [1, 2, 3, 4], "the document is posted");
      assert.equal(document.byteLength, 4, "the caller keeps its document: the host sends a copy");
      await localErrorResult(host.control(new Uint8Array([7]), new Uint8Array(0)), 1);
      await localErrorResult(host.control(new Uint8Array([7]), [1]), 1);
      assert.equal(
        events.filter((event) => event[1] === "miso.control.v1").length,
        3,
        "a refused document never reaches the port",
      );
    }

    // Issue #1570b D5, gate 4's host half: an engine event reaches every listener as
    // `{ lane, frame }` in arrival order; a frame that arrives with no listener is dropped and
    // counted, and unsubscribing stops delivery.
    {
      const deliver = (lane, bytes) => host.node.port.onmessage({
        data: { tag: "miso.control-event.v1", lane, frame: new Uint8Array(bytes) },
      });
      assert.equal(host.discardedControlEvents, 0);
      deliver(0, [1]);
      deliver(1, [2]);
      assert.equal(host.discardedControlEvents, 2, "each frame with no listener is counted");
      const seen = [];
      const other = [];
      const off = host.onControlEvent((event) => seen.push([event.lane, [...event.frame]]));
      host.onControlEvent((event) => other.push(event.lane));
      deliver(1, [3, 4]);
      deliver(0, [5]);
      assert.deepEqual(seen, [[1, [3, 4]], [0, [5]]], "events arrive in order with their lane");
      assert.deepEqual(other, [1, 0], "every listener is called");
      off();
      deliver(0, [6]);
      assert.equal(seen.length, 2, "an unsubscribed listener hears nothing more");
      assert.equal(other.length, 3);
      assert.equal(host.discardedControlEvents, 2, "a frame with a listener left is not a drop");
      assert.throws(() => host.onControlEvent("not a function"));
    }

    // Issue #1294a: a watermark post reaches every listener as one frozen record of its six
    // words, and unsubscribing stops delivery. (A malformed post fails the host, below.)
    {
      const words = {
        revision: 7n, sample: 256n, outcome: 2n, exactCount: 6n, transitionFallbackCount: 1n,
        supersededCount: 0n,
      };
      const deliver = (extra = {}) => host.node.port.onmessage({
        data: { tag: "miso.watermark.v1", ...words, ...extra },
      });
      const seen = [];
      const off = host.onWatermark((watermark) => seen.push(watermark));
      deliver();
      assert.deepEqual(seen, [words], "the listener gets the posted words");
      assert(Object.isFrozen(seen[0]));
      off();
      deliver({ revision: 8n });
      assert.equal(seen.length, 1, "an unsubscribed listener hears nothing more");
      assert.throws(() => host.onWatermark("not a function"));
    }

    failSource = true;
    const failedStorage = new ArrayBuffer(16);
    const failedSource = host.submitSource({
      sourceId: "source", generation: 3n, startFrame: 2n,
      sampleRateHz: 48000, planes: [new Float32Array(failedStorage)], frames: 4,
      endOfRegion: true,
    });
    assert.equal(failedStorage.byteLength, 0);
    const returnedError = await failedSource.then(
      () => assert.fail("expected transferred processor error"),
      (error) => error,
    );
    assert.deepEqual(Object.keys(returnedError).sort(), ["planes", "requestId", "result", "tag"]);
    assert.equal(returnedError.tag, "miso.error.v1");
    assert.equal(returnedError.planes[0].byteLength, 16);
    await host.dispose();
    await host.dispose();
    const disposeEvents = events.filter((event) => event[1] === "miso.dispose.v1");
    assert.equal(disposeEvents.length, 1, "settled disposal is idempotent");
    assert.equal(FakeNode.latest.port.closeCount, 1);
    assert.equal(FakeNode.latest.disconnectCount, 1);

    readyMutation = (ready) => ({
      ...ready,
      resources: { ...ready.resources, backend: 0 },
    });
    await errorResult(createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    }), 255);
    assert.equal(FakeNode.latest.port.closeCount, 1, "creation rejection closes the port");
    assert.equal(FakeNode.latest.disconnectCount, 1, "creation rejection disconnects the node");
    readyMutation = null;

    const schemaHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    statusMutation = (statusValue) => ({ ...statusValue, memoryBytes: 65537 });
    await errorResult(schemaHost.status(), 255);
    statusMutation = null;
    await schemaHost.dispose();

    // Issue #1562a D1: a malformed control reply is a schema failure, sticky for the host.
    for (const mutate of [
      (reply) => ({ ...reply, revision: -1n }),
      (reply) => ({ ...reply, revision: 1 }),
      (reply) => ({ ...reply, response: [1] }),
      (reply) => ({ ...reply, result: 11 }),
      (reply) => ({ ...reply, extra: 1 }),
    ]) {
      const controlHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      controlMutation = (reply) => {
        const mutated = mutate(reply);
        return mutated;
      };
      await errorResult(controlHost.control(new Uint8Array([1])), 255);
      controlMutation = null;
      await controlHost.dispose().catch(() => undefined);
    }

    // Issue #1294a: a malformed watermark post is a schema failure, sticky for the host.
    for (const malformed of [
      { revision: 1, sample: 0n }, { revision: -1n, sample: 0n }, { revision: 1n },
    ]) {
      const watermarkHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      const doomed = watermarkHost.status();
      watermarkHost.node.port.onmessage({
        data: {
          tag: "miso.watermark.v1", outcome: 1n, exactCount: 1n, transitionFallbackCount: 0n,
          supersededCount: 0n, ...malformed,
        },
      });
      await errorResult(doomed, 255);
      await watermarkHost.dispose().catch(() => undefined);
    }

    // #1536: concurrent status calls share one read and are never refused for being in flight.
    const sharedStatusHost = async () => createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const settleTicks = () => new Promise((resolve) => setTimeout(resolve, 0));
    const releaseOne = async () => {
      heldAll.shift()();
      await settleTicks();
    };
    // Attach the handler at the call so a rejection that lands while the test awaits a release is
    // never unhandled; `rejected` then checks the stored outcome like `errorResult`.
    const watch = (promise) => promise.then((value) => ({ value }), (error) => ({ error }));
    const rejected = async (watched, result) => {
      const outcome = await watched;
      assert(outcome.error !== undefined, "expected rejection");
      assert.deepEqual(Object.keys(outcome.error).sort(), ["requestId", "result", "tag"]);
      assert.equal(outcome.error.result, result);
      return outcome.error;
    };
    const requestsSince = (from, tag) => events.slice(from).filter((event) => event[1] === tag);
    {
      // Gate 1. Test value: a refused second call (the defect that fails an overlapping managed
      // observation close), or one posted read per call (an unbounded in-flight count).
      const host = await sharedStatusHost();
      const from = events.length;
      holdAll = true;
      const calls = [host.status(), host.status(), host.status()];
      assert.equal(requestsSince(from, "miso.status.v1").length, 1, "three calls post one read");
      holdAll = false;
      await releaseOne();
      const replies = await Promise.all(calls);
      assert.equal(new Set(replies.map((reply) => reply.requestId)).size, 1, "one shared reply");
      assert.equal(replies[0], replies[1], "the joined callers share the frozen reply");
      assert.equal(
        (await host.renderAllocationCount()).requestId, replies[0].requestId + 1,
        "joined callers allocated no IDs",
      );
      await host.dispose();
    }
    {
      // Gate 2. Test value: joining a read sent before a request the caller already sent.
      const host = await sharedStatusHost();
      const from = events.length;
      holdAll = true;
      const a = host.status();
      const other = host.renderAllocationCount();
      const b = host.status();
      const c = host.status();
      assert.equal(requestsSince(from, "miso.status.v1").length, 1, "one read on the port");
      await releaseOne();
      const first = await a;
      const reads = requestsSince(from, "miso.status.v1");
      assert.equal(reads.length, 2, "exactly one follow-up read is posted");
      assert.equal(first.requestId, reads[0][2]);
      assert(reads[1][2] > events.slice(from).find((event) => event[1] === "miso.renderallocations.v1")[2],
        "the follow-up is newer than the other request");
      await releaseOne();
      await releaseOne();
      holdAll = false;
      const [replyB, replyC] = await Promise.all([b, c]);
      await other;
      assert.equal(replyB.requestId, reads[1][2], "b reads after the other request");
      assert.equal(replyB, replyC, "b and c share the follow-up");
      await host.dispose();
    }
    {
      // Gate 3a. Test value: a joined caller left pending for ever, or a read per caller after fail.
      const host = await sharedStatusHost();
      const from = events.length;
      holdAll = true;
      const calls = [host.status(), host.status(), host.status()].map(watch);
      statusMutation = (reply) => ({ ...reply, memoryBytes: 65537 });
      await releaseOne();
      statusMutation = null;
      holdAll = false;
      for (const call of calls) await rejected(call, 255);
      const posted = events.length;
      assert.equal((await errorResult(host.status(), 255)).tag, "miso.error.v1", "sticky");
      assert.equal(events.length, posted, "the sticky error posts nothing");
      assert.equal(requestsSince(from, "miso.status.v1").length, 1);
      await host.dispose().catch(() => undefined);
    }
    {
      // Gate 3b. Test value: a follow-up caller left pending when the host fails, or a follow-up
      // read posted after the host failed.
      const host = await sharedStatusHost();
      const node = FakeNode.latest;
      holdAll = true;
      const calls = [
        host.status(), host.status(), host.renderAllocationCount(), host.status(),
      ].map(watch);
      const posted = events.length;
      holdAll = false;
      node.onprocessorerror();
      for (const call of calls) await rejected(call, 255);
      assert.equal(events.length, posted, "nothing is posted after the failure");
      heldAll.length = 0;
      await host.dispose().catch(() => undefined);
    }
    {
      // Gate 3c. Test value: a follow-up read posted behind the dispose request, or a follow-up
      // caller left pending for ever once disposal starts.
      const host = await sharedStatusHost();
      const from = events.length;
      holdAll = true;
      const read = host.status();
      const other = host.renderAllocationCount();
      const follow = watch(host.status());
      const disposal = host.dispose();
      assert.equal((await localErrorResult(host.status(), 3)).result, 3, "status after dispose was sent");
      await releaseOne();
      await releaseOne();
      await releaseOne();
      holdAll = false;
      assert.equal((await read).result, 0, "the read sent before dispose settles normally");
      await other;
      assert.equal((await rejected(follow, 3)).requestId, 0);
      await disposal;
      assert.equal(requestsSince(from, "miso.status.v1").length, 1, "no read after dispose");
      assert.equal(
        events.slice(from).findLastIndex((event) => event[1] === "miso.dispose.v1"),
        events.length - from - 1,
        "dispose is the last request posted",
      );
    }
    {
      // Gate 4. Test value: dropping or cancelling a read when its first caller stops waiting (the
      // awaitSpectrum deadline), which hangs the next caller or posts a second read.
      const host = await sharedStatusHost();
      const from = events.length;
      holdAll = true;
      const abandoned = Promise.race([host.status(), new Promise((resolve) => setTimeout(resolve, 1, "timeout"))]);
      assert.equal(await abandoned, "timeout");
      const later = host.status();
      assert.equal(requestsSince(from, "miso.status.v1").length, 1, "the later call joined");
      holdAll = false;
      await releaseOne();
      assert.equal((await later).result, 0);
      await host.dispose();
    }

    // #1399 gate 3: the reply's field set and the count's type are checked. A reply without the
    // count, and one whose count is not a u64 `bigint`, fail the whole host with 255; a count at
    // the top of the u64 range is accepted and read back whole.
    const countHost = () => createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    for (const mutation of [
      // No count; a number rather than a bigint; a negative count; a count past u64.
      ({ liveValuesSuperseded: _dropped, ...rest }) => rest,
      (reply) => ({ ...reply, liveValuesSuperseded: 7 }),
      (reply) => ({ ...reply, liveValuesSuperseded: -1n }),
      (reply) => ({ ...reply, liveValuesSuperseded: 1n << 64n }),
      // #1294b: the blocking-rebuild count is a required u32 number.
      ({ blockingRebuilds: _dropped, ...rest }) => rest,
      (reply) => ({ ...reply, blockingRebuilds: 1n }),
      // #1349 gate 5: each of the three watermark words is required and must be a u64 bigint.
      ({ appliedRevision: _dropped, ...rest }) => rest,
      ({ appliedSample: _dropped, ...rest }) => rest,
      ({ appliedOutcome: _dropped, ...rest }) => rest,
      (reply) => ({ ...reply, appliedRevision: 9 }),
      (reply) => ({ ...reply, appliedSample: -1n }),
      (reply) => ({ ...reply, appliedOutcome: 1n << 64n }),
      // The stale-view count: absent, negative, and a memory smaller than the boot size (memory
      // only grows, so a shrink is a corrupt reply).
      ({ staleViewQuanta: _dropped, ...rest }) => rest,
      (reply) => ({ ...reply, staleViewQuanta: -1 }),
      (reply) => ({ ...reply, memoryBytes: 32768 }),
    ]) {
      const refused = await countHost();
      statusMutation = mutation;
      await errorResult(refused.status(), 255);
      statusMutation = null;
      await refused.dispose();
    }
    const wide = await countHost();
    statusMutation = (reply) => ({ ...reply, liveValuesSuperseded: (1n << 64n) - 1n });
    assert.equal((await wide.status()).liveValuesSuperseded, (1n << 64n) - 1n);
    // A memory a control call grew since boot is a valid status, not a corrupt reply.
    statusMutation = (reply) => ({ ...reply, memoryBytes: 131072 });
    assert.equal((await wide.status()).memoryBytes, 131072);
    statusMutation = null;
    await wide.dispose();

    // Issue #1477 D3: the render allocation reply check, conjunct by conjunct. Any other tag,
    // field set, result or count fails the whole host with 255; the well-formed reply resolves at
    // both ends of the u32 range. Every refusal case runs before the verdict, so a lost conjunct
    // names each reply it lets through.
    const allocationHost = () => createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const unrefusedAllocationReplies = [];
    for (const [what, mutation] of [
      ["a wrong tag", (reply) => ({ ...reply, tag: "miso.status.v1" })],
      ["no count", ({ count: _dropped, ...rest }) => rest],
      ["an extra field", (reply) => ({ ...reply, extra: 0 })],
      ["result 6", (reply) => ({ ...reply, result: 6 })],
      ["count -1", (reply) => ({ ...reply, count: -1 })],
      ["count 4294967296", (reply) => ({ ...reply, count: 4294967296 })],
      ["count 1.5", (reply) => ({ ...reply, count: 1.5 })],
      ["count \"0\"", (reply) => ({ ...reply, count: "0" })],
      ["count 0n", (reply) => ({ ...reply, count: 0n })],
    ]) {
      const host = await allocationHost();
      renderAllocationsMutation = mutation;
      const outcome = host.renderAllocationCount();
      await errorResult(outcome, 255).catch(async () => {
        const seen = await outcome.then(
          (reply) => `resolved with count ${String(reply?.count)}`,
          (error) => error?.tag === "miso.error.v1"
            ? `refused with result ${String(error.result)}, fields ${Object.keys(error).sort().join(",")}`
            : `rejected with ${String(error)}`,
        );
        unrefusedAllocationReplies.push(`${what}: ${seen}`);
      });
      renderAllocationsMutation = null;
      await host.dispose();
    }
    assert.deepEqual(unrefusedAllocationReplies, [],
      "the host did not refuse every malformed render allocation reply with 255");
    for (const [what, mutation, count] of [
      ["count 0", null, 0],
      ["count 4294967295", (reply) => ({ ...reply, count: 4294967295 }), 4294967295],
    ]) {
      const host = await allocationHost();
      renderAllocationsMutation = mutation;
      const reply = await host.renderAllocationCount().catch((error) => {
        throw new Error(`the host refused a render allocation reply with ${what}`, { cause: error });
      });
      renderAllocationsMutation = null;
      assert.equal(reply.tag, "miso.renderallocations.v1");
      assert.equal(reply.result, 0);
      assert.equal(reply.count, count, `render allocation reply with ${what}`);
      await host.dispose();
    }

    // The session map's field set is exact (#1210 gate 3, as #1565B restates it): a reply that
    // lacks the source list, or still carries a strip list the core's control list replaced, fails
    // the whole host.
    for (const [what, mutation] of [
      ["no source list", ({ sources: _dropped, ...rest }) => rest],
      ["a retired track list", (map) => ({ ...map, tracks: ["kick"] })],
      ["a retired submix list", (map) => ({ ...map, submixes: ["aaa-bus"] })],
      // Issue #1565: the route and VCA lists are deleted from the exact field set, so a reply
      // that still carries one fails the whole host.
      ["a deleted route list", (map) => ({ ...map, routes: ["zz-send"] })],
      ["a deleted VCA list", (map) => ({ ...map, vcas: ["zz-vca"] })],
    ]) {
      const mapHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      sessionMapMutation = mutation;
      await errorResult(mapHost.sessionMap(), 255).catch((error) => {
        throw new Error(`the host accepted a session map with ${what}`, { cause: error });
      });
      sessionMapMutation = null;
      await mapHost.dispose();
    }

    const planeHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    failSource = false;
    planeMutation = (response) => ({
      ...response,
      planes: [new Float32Array(response.planes[0].buffer, 4, 3)],
    });
    const malformedStorage = new ArrayBuffer(16);
    await errorResult(planeHost.submitSource({
      sourceId: "source", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [new Float32Array(malformedStorage)], frames: 4,
      endOfRegion: false,
    }), 255);
    planeMutation = null;
    await planeHost.dispose();

    // #106 F3: the in-flight bound is per source and equals the ring depth in quanta
    // (sourceRingFrames 256 / quantumFrames 64 = 4). Red mutation: return `true` from
    // `#saturated` once one source request is unsettled -> the second request rejects with 6.
    const pipelineHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    failSource = false;
    holdAll = true;
    const chunk = (sourceId, startFrame) => {
      const buffer = new ArrayBuffer(8);
      return {
        request: pipelineHost.submitSource({
          sourceId,
          generation: 1n,
          startFrame: BigInt(startFrame),
          sampleRateHz: 48000,
          planes: [new Float32Array(buffer)],
          frames: 2,
          endOfRegion: false,
        }),
        buffer,
      };
    };
    const inFlight = [1, 2, 3, 4].map((startFrame) => chunk("source", startFrame));
    for (const [index, entry] of inFlight.entries()) {
      assert.equal(entry.buffer.byteLength, 0, `chunk ${index} was transferred`);
    }
    const beforeSourceOverflow = events.length;
    const overflow = chunk("source", 5);
    await localErrorResult(overflow.request, 6);
    assert.equal(events.length, beforeSourceOverflow, "source saturation posts no refusal");
    assert.equal(
      overflow.buffer.byteLength,
      8,
      "a locally refused chunk keeps its planes: nothing is transferred and the caller can retry",
    );
    // A different source has its own budget and is accepted while the first is saturated.
    const other = chunk("other-source", 6);
    assert.equal(other.buffer.byteLength, 0, "the bound is per source, not per host");
    // One unsettled seek per source: the ring carries a single command slot.
    const firstSeek = pipelineHost.seekSource({
      sourceId: "source", generation: 2n, sourceFrame: 0n,
    });
    const beforeSeekOverflow = events.length;
    await localErrorResult(pipelineHost.seekSource({
      sourceId: "source", generation: 3n, sourceFrame: 0n,
    }), 6);
    assert.equal(events.length, beforeSeekOverflow, "seek saturation posts no refusal");
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    const settled = await Promise.all([
      ...inFlight.map((entry) => entry.request),
      other.request,
      firstSeek,
    ]);
    assert.deepEqual(
      settled.map((message) => message.requestId),
      [1, 2, 3, 4, 5, 6],
      "acknowledgements arrive in request order",
    );
    // With the budget released the source accepts chunks again.
    const again = chunk("source", 9);
    assert.equal(again.buffer.byteLength, 0);
    assert.equal((await again.request).requestId, 7, "source saturation does not burn an ID");

    // Issue #1644 gate 2, the host half: one session seek message per call, with the plain form's
    // `-1n` anchor, and one unsettled at a time; a second is refused locally and posts nothing.
    holdAll = true;
    const firstSessionSeek = pipelineHost.seekSession({ generation: 2n, frame: 480n });
    const beforeSessionSeekOverflow = events.length;
    await localErrorResult(pipelineHost.seekSession({
      generation: 3n, frame: 0n, anchorSample: 128n,
    }), 6);
    assert.equal(events.length, beforeSessionSeekOverflow, "a second session seek posts nothing");
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    assert.deepEqual(await firstSessionSeek, {
      tag: "miso.session-seek.v1", requestId: 8, result: 0, generation: 2n,
    });
    sessionSeekResult = 1;
    assert.equal((await pipelineHost.seekSession({
      generation: 3n, frame: 0n, anchorSample: 128n,
    })).result, 1, "a refused session seek resolves with the engine's result");
    sessionSeekResult = 0;
    sessionSeekGenerationOffset = 1n;
    await errorResult(pipelineHost.seekSession({ generation: 4n, frame: 0n }), 255);
    sessionSeekGenerationOffset = 0n;
    sessionSeekMessages.pop();
    assert.deepEqual(sessionSeekMessages, [
      { tag: "miso.session-seek.v1", generation: 2n, sourceFrame: 480n, anchorSample: -1n, requestId: 8 },
      { tag: "miso.session-seek.v1", generation: 3n, sourceFrame: 0n, anchorSample: 128n, requestId: 9 },
    ]);
    await pipelineHost.dispose();

    {
      // Issue #1297a, the host half: `seekSourceAt` posts one exact message and shares the
      // per-source seek bound with `seekSource`; `seekReport` resolves with the record's words
      // or, for a refusal, a null report; a reply whose report disagrees with its result fails
      // the host.
      const seekHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      const words = {
        generation: 5n, firstSample: 1024n, sourceFrame: 48000n,
        cumulativeUnderrunFrames: 7n, cumulativeHeldFrames: 9n,
      };
      holdAll = true;
      const firstSeekAt = seekHost.seekSourceAt({
        sourceId: "drums", generation: 2n, sourceFrame: 480n, anchorSample: 128n,
      });
      const beforeSeekAtOverflow = events.length;
      await localErrorResult(seekHost.seekSource({
        sourceId: "drums", generation: 3n, sourceFrame: 0n,
      }), 6);
      assert.equal(events.length, beforeSeekAtOverflow, "a second seek of a source posts nothing");
      holdAll = false;
      for (const respond of heldAll) respond();
      heldAll.length = 0;
      assert.equal((await firstSeekAt).result, 0);
      seekReportReply = (received) => ({
        requestId: received.requestId, result: 0, report: words,
      });
      holdAll = true;
      const firstReport = seekHost.seekReport("drums");
      const beforeReportOverflow = events.length;
      await localErrorResult(seekHost.seekReport("drums"), 6);
      assert.equal(events.length, beforeReportOverflow, "a second report of a source posts nothing");
      holdAll = false;
      for (const respond of heldAll) respond();
      heldAll.length = 0;
      assert.equal((await firstReport).result, 0);
      await localErrorResult(seekHost.seekSourceAt({
        sourceId: "drums", generation: 3n, sourceFrame: 0n, anchorSample: -1n,
      }), 1);
      seekReportReply = (received) => ({
        requestId: received.requestId, result: 0, report: words,
      });
      assert.deepEqual(await seekHost.seekReport("$timeline"), {
        tag: "miso.seek-report.v1", requestId: 3, result: 0, report: words,
      });
      seekReportReply = (received) => ({ requestId: received.requestId, result: 6, report: null });
      assert.deepEqual((await seekHost.seekReport("drums")).report, null);
      seekReportReply = (received) => ({ requestId: received.requestId, result: 6, report: words });
      await errorResult(seekHost.seekReport("drums"), 255);
      assert.deepEqual(sourceSeekMessages, [
        { tag: "miso.seek-at.v1", sourceId: "drums", generation: 2n, sourceFrame: 480n,
          anchorSample: 128n, requestId: 1 },
        { tag: "miso.seek-report.v1", sourceId: "drums", requestId: 2 },
        { tag: "miso.seek-report.v1", sourceId: "$timeline", requestId: 3 },
        { tag: "miso.seek-report.v1", sourceId: "drums", requestId: 4 },
        { tag: "miso.seek-report.v1", sourceId: "drums", requestId: 5 },
      ]);
      await seekHost.dispose();
      // An OK reply whose word is a number, not a `bigint`, fails a fresh host.
      const wordHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      seekReportReply = (received) => ({
        requestId: received.requestId, result: 0, report: { ...words, generation: 5 },
      });
      await errorResult(wordHost.seekReport("drums"), 255);
      await wordHost.dispose();
    }

    // W4-D1: a browser that cannot validate simd128 is refused with the typed record, before any
    // network request. Red mutation: delete the `WebAssembly.validate(SIMD128_PROBE)` guard in
    // `createMisoAudioWorkletHost` -> the rejection becomes a generic 255 and `compileCount` grows.
    const compileCount = events.filter((event) => event[0] === "compile").length;
    WebAssembly.validate = () => false;
    const refusal = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    }).then(() => assert.fail("expected unsupported-browser refusal"), (error) => error);
    WebAssembly.validate = () => true;
    assert.deepEqual(
      Object.keys(refusal).sort(),
      ["capability", "requestId", "result", "tag"],
    );
    assert.equal(refusal.tag, "miso.unsupported.v1");
    assert.equal(refusal.capability, "simd128");
    assert.equal(refusal.result, 7);
    assert.equal(refusal.requestId, 0);
    assert.equal(Object.isFrozen(refusal), true);
    assert.equal(
      events.filter((event) => event[0] === "compile").length,
      compileCount,
      "the probe refuses before any artifact is fetched",
    );

    // Issue #137 D1/D2/D3: the live controls' main-realm half.
    const liveControlHost = await createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode("{\"schema_version\":0}"),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const map = await liveControlHost.sessionMap();
    // The strip order is the core's control list, not this map (#1565B); the fake's two tracks size
    // the meter frame, and bus strips follow them.
    assert.equal(Object.hasOwn(map, "tracks"), false);
    assert.equal(Object.hasOwn(map, "submixes"), false);
    const fakeTrackCount = 2;
    // Issue #241: the source list crosses the port with its `bigint` frame count intact, and the host's
    // acknowledgement validator accepted it -- a malformed row fails the whole host with 255, so
    // reaching this line is itself the assertion that the shape is the declared one.
    assert.deepEqual(map.sources, [
      { id: "bass", channels: 1, frames: 96000n },
      { id: "drums", channels: 2, frames: 2048n },
    ], "the canonical source order and shape are the ABI");
    assert.equal(map.metersAttached, true);
    assert.equal(Object.hasOwn(map, "routes"), false, "issue #1565: the route list is deleted");
    assert.equal(Object.hasOwn(map, "vcas"), false, "issue #1565: the VCA list is deleted");

    // Issue #393: one real host fixture owns every request ID across interleaved request classes.
    // Twenty-five rounds cover 200 successful calls, including both lease orderings and direct
    // source/seek traffic. No caller-side ID is supplied anywhere in this loop.
    const mixedAcks = [];
    const mixedSendStart = events.length;
    for (let round = 0; round < 25; round += 1) {
      mixedAcks.push((await liveControlHost.meters({ enabled: true, onFrame: null })).requestId);
      mixedAcks.push((await liveControlHost.meters({ enabled: false, onFrame: null })).requestId);
      mixedAcks.push((await liveControlHost.telemetry({ enabled: true, onFrame: null })).requestId);
      mixedAcks.push((await liveControlHost.telemetry({ enabled: false, onFrame: null })).requestId);
      const mixedBuffer = new ArrayBuffer(16);
      const mixedLeft = new Float32Array(mixedBuffer, 0, 2);
      const mixedRight = new Float32Array(mixedBuffer, 8, 2);
      mixedLeft.set([round, round + 1]);
      mixedRight.set([round + 2, round + 3]);
      mixedSuccess = true;
      const mixedSourceAck = await liveControlHost.submitSource({
        sourceId: "mixed-source", generation: BigInt(round + 1), startFrame: 0n,
        sampleRateHz: 48000, planes: [mixedLeft, mixedRight], frames: 2, endOfRegion: true,
      });
      assert.equal(mixedSourceAck.result, 0, "mixed source admission succeeds");
      assert.equal(mixedBuffer.byteLength, 0, "successful mixed source transfers its buffer");
      assert.equal(mixedSourceAck.planes.length, 2);
      assert.equal(mixedSourceAck.planes[0].byteOffset, 0);
      assert.equal(mixedSourceAck.planes[1].byteOffset, 8);
      assert.equal(mixedSourceAck.planes[0].buffer, mixedSourceAck.planes[1].buffer);
      assert.equal(mixedSourceAck.planes[0].buffer.byteLength, 16);
      const mixedSourceEvent = events.findLast((event) => event[1] === "miso.source.v1");
      assert.equal(mixedSourceEvent[2], mixedSourceAck.requestId);
      assert.equal(mixedSourceEvent[3], 1, "shared mixed source storage transfers once");
      mixedAcks.push(mixedSourceAck.requestId);
      mixedAcks.push((await liveControlHost.seekSource({
        sourceId: "mixed-source", generation: BigInt(round + 1), sourceFrame: 0n,
      })).requestId);
      mixedAcks.push((await liveControlHost.status()).requestId);
      mixedAcks.push((await liveControlHost.sessionMap()).requestId);
    }
    assert.equal(mixedAcks.length, 200, "mixed host regression covers 200 successful calls");
    const mixedSendIds = events.slice(mixedSendStart).map((event) => event[2]);
    assert.deepEqual(mixedAcks, mixedSendIds, "acknowledgements match actual outbound send order");
    assert.equal(mixedAcks[0], 2, "sessionMap consumes the first host allocation");
    assert.equal(mixedAcks.at(-1), 201, "200 mixed calls occupy IDs 2 through 201");
    assert(mixedAcks.every((id, index) => index === 0 || id === mixedAcks[index - 1] + 1),
      "mixed acknowledgements have adjacent IDs");
    assert.equal(new Set(mixedAcks).size, mixedAcks.length, "mixed acknowledgements are unique");
    console.log("Issue393 mixed calls: 200 result-zero, send IDs 2..201, adjacent");
    // The shipped SDK consumer uses the same real host: sessionMap -> direct meter lease -> the
    // SDK's live controls. This is the collision reproducer that the host-owned ledger fixes.
    const { createBrowserLiveControls } = await import(new URL("./sdk/src/browser/live-controls.ts", root));
    // The strip order is the core's control list (#1564 D5): the fake worklet does not run a core,
    // so the list is scripted; every other call still goes through the real host.
    const { scriptedControl } = await import(new URL("./sdk/test/control-support.mjs", root));
    const browserLiveControls = await createBrowserLiveControls({
      ...liveControlHost,
      sessionMap: () => liveControlHost.sessionMap(),
      meters: (request) => liveControlHost.meters(request),
      control: scriptedControl({ strips: { tracks: ["kick", "snare"], submixes: ["aaa-bus", "zzz-bus"] } }),
    });
    await liveControlHost.meters({ enabled: true, onFrame: null });
    assert.ok(browserLiveControls.edit.track("kick"),
      "the SDK live controls resolve through the real host after direct meters");
    // Every bounded response class refuses locally with requestId 0 and leaves the next accepted
    // request exactly one ID later. Hold each class independently so the fake port cannot answer.
    const assertBoundedNoBurn = async (label, heldCall, refusedCall, nextCall) => {
      const before = events.length;
      holdAll = true;
      const held = heldCall();
      await localErrorResult(refusedCall(), 6);
      assert.equal(events.length, before + 1, `${label}: refusal posted no message`);
      holdAll = false;
      for (const respond of heldAll.splice(0)) respond();
      const first = await held;
      const next = await nextCall();
      assert.equal(next.requestId, first.requestId + 1, `${label}: refusal did not burn an ID`);
      return next;
    };
    await assertBoundedNoBurn(
      "sessionMap", () => liveControlHost.sessionMap(), () => liveControlHost.sessionMap(),
      () => liveControlHost.sessionMap(),
    );
    await assertBoundedNoBurn(
      "meters", () => liveControlHost.meters({ enabled: false, onFrame: null }),
      () => liveControlHost.meters({ enabled: true, onFrame: null }),
      () => liveControlHost.meters({ enabled: false, onFrame: null }),
    );
    await assertBoundedNoBurn(
      "telemetry", () => liveControlHost.telemetry({ enabled: false, onFrame: null }),
      () => liveControlHost.telemetry({ enabled: true, onFrame: null }),
      () => liveControlHost.telemetry({ enabled: false, onFrame: null }),
    );
    // A malformed request never reaches the port.
    const beforeMalformed = events.length;
    const oldShapeBuffer = new ArrayBuffer(256);
    await localErrorResult(liveControlHost.submitSource({
      requestId: 999, sourceId: "extra", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [new Float32Array(oldShapeBuffer)], frames: 64, endOfRegion: false,
    }), 1);
    assert.equal(oldShapeBuffer.byteLength, 256, "malformed source keeps caller ownership");
    await localErrorResult(liveControlHost.seekSource({
      requestId: 999, sourceId: "extra", generation: 1n, sourceFrame: 0n,
    }), 1);
    await localErrorResult(liveControlHost.meters({ requestId: 999, enabled: true, onFrame: null }), 1);
    await localErrorResult(liveControlHost.telemetry({ requestId: 999, enabled: true, onFrame: null }), 1);
    assert.equal(events.length, beforeMalformed, "a malformed batch costs no message");

    // Leases and their unsolicited frames.
    const meterFrames = [];
    const telemetryFrames = [];
    assert.equal(
      (await liveControlHost.meters({ enabled: true, onFrame: (frame) => meterFrames.push(frame) })).result,
      0,
    );
    assert.equal(
      (await liveControlHost.telemetry({ enabled: true, onFrame: (frame) => telemetryFrames.push(frame) })).result,
      0,
    );
    const node = FakeNode.latest;
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
        windows: 1, trackCount: 2, structureEpoch: 1,
        peaks: new Float32Array([0.125, 0.25, 0.375, 0.5, 0.625, 0.75]),
        trackGrDb: new Float32Array([6.5, 0]), masterGrDb: 6.5,
        firstSample: 512n, endSample: 768n,
        // Issue #1209 gate 3: the 15-field frame with one bus is delivered with its bus section.
        submixCount: 1, submixPeaks: new Float32Array([0.875, 0.9375]),
        submixGrDb: new Float32Array([1.25]),
      },
    });
    node.port.onmessage({
      data: {
        tag: "miso.telemetry.v1", sequence: 1, blocks: 128, cpuPercent: 4.5, peakBlockMs: 0.2,
        meanBlockMs: 0.1, budgetMs: 1.3, deadlineMisses: 0, resolutionMs: 0.005,
        belowResolution: false,
      },
    });
    assert.equal(meterFrames.length, 1);
    assert.deepEqual([...meterFrames[0].peaks], [0.125, 0.25, 0.375, 0.5, 0.625, 0.75]);
    assert.equal(meterFrames[0].submixCount, 1);
    assert.deepEqual([...meterFrames[0].submixPeaks], [0.875, 0.9375]);
    assert.deepEqual([...meterFrames[0].submixGrDb], [1.25]);
    assert.equal(Object.isFrozen(meterFrames[0]), true);

    // Issue #143 E4: the frame as an **app** reads it.
    //
    // `appGainReduction` is the app's own fold, copied verbatim from `meters.ts`'s
    // `Math.max(0, x ?? 0)` lines. The whole point of the declared `peakMagnitude` fold is that
    // this line is a *no-op*: the engine publishes a non-negative magnitude, so clamping at zero
    // changes nothing. Publish the raw negative decibels instead and every one of these becomes
    // `0` -- the exact dead-meter bug -- which is what the red mutation demonstrates.
    const appGainReduction = (frame) => ({
      tracks: Array.from({ length: frame.trackCount }, (_, index) =>
        Math.max(0, frame.trackGrDb[index] ?? 0)),
      master: Math.max(0, frame.masterGrDb ?? 0),
    });
    const ingested = appGainReduction(meterFrames[0]);
    assert.equal(ingested.tracks.length, meterFrames[0].trackCount);
    assert.equal(ingested.tracks[0], 6.5, "trackGrDb(0) > 0 survives the app's clamp unchanged");
    assert(ingested.tracks[0] > 0);
    assert.equal(ingested.tracks[1], 0, "a track with no observed effect reads exactly zero");
    assert.equal(ingested.master, 6.5);
    assert.equal(meterFrames[0].endSample - meterFrames[0].firstSample, 256n);

    // Every shape rule is a hard failure, not a silent skip. Each entry is one red mutation of
    // one rule in the `miso.meter.v1` branch of `#receive`. `MISSING` removes the field.
    const MISSING = Symbol("missing");
    for (const broken of [
      { generation: 0n },
      { generation: 1 },
      { validity: 0x1 },
      { validity: 0x13 },
      { validity: 0x7, lossCount: 0 },
      { validity: 0x3, lossCount: 1 },
      { lossCount: 0x1_0000_0000 },
      // Issue #1562b: the structure epoch is a positive safe integer.
      { structureEpoch: MISSING },
      { structureEpoch: 0 },
      { structureEpoch: 1.5 },
      { structureEpoch: 1n },
      { trackGrDb: new Float32Array(1) },
      { trackGrDb: [0, 0] },
      { trackGrDb: new Float32Array([-6.5, 0]) },
      { trackGrDb: new Float32Array([Number.NaN, 0]) },
      { masterGrDb: -1 },
      { masterGrDb: "6.5" },
      { firstSample: 512 },
      { endSample: 256n },
      { endSample: 512n },
      // Issue #1209 gate 3: the bus section's shape rules, against a one-bus base frame.
      { submixGrDb: MISSING },
      { submixPeaks: MISSING },
      { submixCount: MISSING },
      { submixPeaks: new Float32Array(1) },
      { submixPeaks: new Float32Array(4) },
      { submixCount: 2 },
      { submixCount: -1, submixPeaks: new Float32Array(0), submixGrDb: new Float32Array(0) },
      { submixCount: 1.5 },
      { submixPeaks: [0, 0] },
      { submixPeaks: new Float32Array([-0.5, 0]) },
      { submixPeaks: new Float32Array([0, Number.POSITIVE_INFINITY]) },
      { submixGrDb: new Float32Array(0) },
      { submixGrDb: new Float32Array([-1]) },
      { submixGrDb: new Float32Array([Number.NaN]) },
      { submixGrDb: [0] },
      { unexpected: 0 },
    ]) {
      const rejecting = await createMisoAudioWorkletHost({
        context,
        document: new TextEncoder().encode("{\"schema_version\":0}"),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      const rejected = rejecting.status();
      const data = {
        tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
        windows: 1, trackCount: 2, structureEpoch: 1,
        peaks: new Float32Array(6), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 512n, endSample: 768n, submixCount: 1,
        submixPeaks: new Float32Array(2), submixGrDb: new Float32Array(1), ...broken,
      };
      for (const [key, value] of Object.entries(broken)) {
        if (value === MISSING) delete data[key];
      }
      FakeNode.latest.port.onmessage({ data });
      await errorResult(rejected, 255);
    }
    assert.equal(telemetryFrames.length, 1);
    assert.equal(telemetryFrames[0].cpuPercent, 4.5);

    // A released lease detaches the callback: a late frame is delivered nowhere.
    const framesAtRelease = meterFrames.length;
    await liveControlHost.meters({ enabled: false, onFrame: null });
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 2, generation: 1n, validity: 0x3, lossCount: 0,
        windows: 1, trackCount: 2, structureEpoch: 1,
        peaks: new Float32Array(6), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 768n, endSample: 1024n,
        submixCount: 0, submixPeaks: new Float32Array(0), submixGrDb: new Float32Array(0),
      },
    });
    assert.equal(meterFrames.length, framesAtRelease, "a released lease receives nothing");

    // A malformed frame is a hard failure, not a silent skip: a console that ignores a broken
    // frame is a console that lies to its user. Red mutation: replace the `#fail` in the
    // `miso.meter.v1` branch of `#receive` with `return` -> this status stays pending forever.
    const doomed = liveControlHost.status();
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 3, generation: 1n, validity: 0x3, lossCount: 0,
        windows: 1, trackCount: 2, structureEpoch: 1,
        peaks: new Float32Array(4), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 0n, endSample: 0n,
        submixCount: 0, submixPeaks: new Float32Array(0), submixGrDb: new Float32Array(0),
      },
    });
    await errorResult(doomed, 255);
    await liveControlHost.dispose();
    const disposedEvents = events.length;
    await localErrorResult(liveControlHost.status(), 3);
    await localErrorResult(liveControlHost.sessionMap(), 3);
    await localErrorResult(liveControlHost.meters({ enabled: false, onFrame: null }), 3);
    await localErrorResult(liveControlHost.telemetry({ enabled: false, onFrame: null }), 3);
    await localErrorResult(liveControlHost.seekSource({
      sourceId: "disposed", generation: 1n, sourceFrame: 0n,
    }), 3);
    await localErrorResult(liveControlHost.submitSource({
      sourceId: "disposed", generation: 1n, startFrame: 0n, sampleRateHz: 48000,
      planes: [new Float32Array(2)], frames: 2, endOfRegion: true,
    }), 3);
    assert.equal(events.length, disposedEvents, "disposed refusals post no messages or consume IDs");

    // Issue #1381a D6, gate 6: the host's one timer is the single-mode service tick. It starts
    // with the host, once per quantum, and each tick is the bare `miso.service.v1` message: no
    // request ID, so the tick takes no place in the request sequence and never makes a status read
    // follow another; and it gets no reply. A dispose stops it before the dispose request.
    {
      const tickHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      const timer = intervals.at(-1);
      assert.equal(timer.cleared, false, "the tick runs while the host does");
      assert.equal(timer.delay, 1, "one quantum of 64 frames at 48 kHz, rounded to a millisecond");
      const before = events.length;
      timer.callback();
      timer.callback();
      assert.deepEqual(events.slice(before), [["service"], ["service"]], "a tick posts the message");
      const read = tickHost.status();
      timer.callback();
      assert.equal((await read).requestId, (await tickHost.status()).requestId - 1,
        "a tick consumes no request ID");
      // A failed single-mode dispose may be retried, so it leaves exactly one live service tick.
      const timersBefore = intervals.length;
      disposeResult = 3;
      await errorResult(tickHost.dispose(), 3);
      disposeResult = 0;
      const live = intervals.filter((entry) => !entry.cleared);
      assert.equal(live.length, 1, "a failed dispose leaves exactly one live service tick");
      assert.equal(intervals.length, timersBefore + 1, "the tick is restarted, once");
      await tickHost.dispose();
      assert.equal(live[0].cleared, true, "the retried dispose stops the restarted tick");
      assert.equal(timer.cleared, true, "dispose stops the tick");
      const afterDispose = events.length;
      timer.callback();
      assert.equal(events.length, afterDispose, "a tick after dispose posts nothing");
    }

    // Issue #1332 S7 (D1, D2, D6, D7): `worker` mode on an isolated page. A fake control Worker
    // answers the boot with the token and the reservation, then serves the routed requests.
    {
      const originalWorker = globalThis.Worker;
      const originalIsolated = Object.getOwnPropertyDescriptor(globalThis, "crossOriginIsolated");
      const originalMemory = WebAssembly.Memory;
      const workers = [];
      let workerBehaviour = "ready";
      class FakeWorker {
        onmessage = null;
        onerror = null;
        onmessageerror = null;
        terminated = false;
        constructor(url) {
          this.url = String(url);
          this.posts = [];
          workers.push(this);
        }
        postMessage(message) {
          this.posts.push(message);
          const reply = (data) => queueMicrotask(() => this.onmessage?.({ data }));
          if (message.tag === "miso.worker-boot.v1") {
            if (workerBehaviour === "error") {
              queueMicrotask(() => this.onerror?.({ preventDefault() {} }));
            } else if (workerBehaviour === "messageerror") {
              queueMicrotask(() => this.onmessageerror?.({}));
            } else if (workerBehaviour === "refuse") {
              reply({ tag: "miso.error.v1", requestId: 0, result: 3 });
            } else {
              reply({
                tag: "miso.worker-ready.v1", requestId: 0, result: 0, handle: 5,
                token: (5n << 32n) | 1n, reservation: 70000, metersAttached: true,
                backend: "simd128", resources: resourceReport(1, 64), memoryBytes: 1310720,
                sourceRingFrames: 40 * 64,
              });
            }
          } else if (message.tag === "miso.control.v1") {
            reply({
              tag: "miso.control.v1", requestId: message.requestId, result: 0,
              response: new Uint8Array([1]), revision: 3n,
            });
          } else if (message.tag === "miso.dispose.v1") {
            // `disposeTrap`: the Worker's dispose traps, which reaches the page as `error`.
            if (workerBehaviour === "disposeTrap") queueMicrotask(() => this.onerror?.({ preventDefault() {} }));
            else reply({
              tag: "miso.ack.v1", requestId: message.requestId,
              result: workerBehaviour === "dispose255" ? 255 : 0,
            });
          }
        }
        terminate() {
          this.terminated = true;
        }
      }
      const memories = [];
      try {
        globalThis.Worker = FakeWorker;
        Object.defineProperty(globalThis, "crossOriginIsolated", { value: true, configurable: true });
        WebAssembly.Memory = class {
          constructor(descriptor) {
            this.descriptor = { ...descriptor };
            this.buffer = new SharedArrayBuffer(65536);
            memories.push(this);
          }
        };
        const nodesBefore = FakeNode.count;
        const host = await createMisoAudioWorkletHost({
          context,
          document: new Uint8Array([1, 2]),
          options: limits,
          simd128ModuleUrl: "simd.wasm",
          workletModuleUrl: "https://example.test/assets/miso-engine-v1-audio-worklet.js",
        });
        assert.equal(host.instanceMode, "worker", "an isolated page runs worker mode");
        assert.equal(FakeNode.count, nodesBefore + 1);
        const worker = workers.at(-1);
        assert.equal(worker.url, "https://example.test/assets/miso-engine-v1-control-worker.js",
          "the Worker is resolved beside the worklet module");
        assert.deepEqual(memories.at(-1).descriptor, {
          initial: abiLayout.memory.initialPages, maximum: abiLayout.memory.maximumPages, shared: true,
        }, "the main realm creates the one shared memory at the layout's pages");
        const boot = worker.posts[0];
        assert.equal(boot.tag, "miso.worker-boot.v1");
        assert.equal(boot.memory, memories.at(-1), "the Worker boots on that memory");
        assert.deepEqual([...boot.document], [1, 2]);
        const node = FakeNode.latest;
        const processorOptions = node.options.processorOptions;
        assert.deepEqual(Object.keys(processorOptions).sort(),
          ["instanceMode", "memory", "metersAttached", "module", "reservation", "token"]);
        assert.equal(processorOptions.instanceMode, "worker");
        assert.equal(processorOptions.memory, memories.at(-1), "the worklet joins the same memory");
        assert.equal(processorOptions.token, (5n << 32n) | 1n, "the worklet gets the Worker's token");
        assert.equal(processorOptions.reservation, 70000, "and the reservation record");
        assert.equal(host.memoryBytes, 1310720, "the ready fields are the Worker's");
        assert.equal(intervals.filter((timer) => !timer.cleared).length, 0,
          "worker mode holds no main-realm service tick");
        // D7: a control frame goes to the Worker; a meter lease to the worklet.
        const reply = await host.control(new Uint8Array([7]));
        assert.equal(reply.revision, 3n);
        assert.equal(worker.posts.at(-1).tag, "miso.control.v1", "control goes to the Worker");
        const eventsBefore = events.length;
        await host.meters({ enabled: true, onFrame: () => undefined });
        assert.deepEqual(events.slice(eventsBefore).map((event) => event[1]), ["miso.meters.v1"],
          "the meter lease goes to the worklet");
        await host.dispose();
        assert.equal(worker.posts.at(-1).tag, "miso.dispose.v1", "dispose goes to the Worker");
        assert.equal(worker.terminated, true, "the Worker ends after its dispose reply");
        assert.equal(node.disconnectCount, 1, "and the node is disconnected");

        // The worker-mode dispose contract: a dispose that fails (a trap in the Worker's dispose)
        // rejects with the typed error and still tears down once; every later call returns the
        // same settled promise, and no second dispose is sent.
        {
          const trapped = await createMisoAudioWorkletHost({
            context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
            workletModuleUrl: "https://example.test/w.js",
          });
          const trappedWorker = workers.at(-1);
          const trappedNode = FakeNode.latest;
          workerBehaviour = "disposeTrap";
          const first = trapped.dispose();
          await errorResult(first, 255);
          workerBehaviour = "ready";
          assert.equal(trappedWorker.terminated, true, "a failed dispose still terminates the Worker");
          assert.equal(trappedNode.disconnectCount, 1, "and disconnects the node once");
          assert.equal(trapped.dispose(), first, "a later dispose returns the same promise");
          await errorResult(trapped.dispose(), 255);
          assert.equal(trappedWorker.posts.filter((message) => message.tag === "miso.dispose.v1").length, 1);
          assert.equal(trappedNode.disconnectCount, 1, "teardown runs exactly once");
        }
        // The Worker answers the dispose with 255 (a trap it caught): the dispose rejects with it
        // and still tears down once.
        {
          const refusing = await createMisoAudioWorkletHost({
            context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
            workletModuleUrl: "https://example.test/w.js",
          });
          const refusingWorker = workers.at(-1);
          const refusingNode = FakeNode.latest;
          workerBehaviour = "dispose255";
          await errorResult(refusing.dispose(), 255);
          workerBehaviour = "ready";
          assert.equal(refusingWorker.terminated, true, "a refused dispose terminates the Worker");
          assert.equal(refusingNode.disconnectCount, 1, "and disconnects the node once");
        }

        // S7 verify attempt 3: a request sent behind a dispose is refused locally (it cannot fail
        // the host and reject the dispose); a second dispose is the first one's promise (one
        // dispose request); a request the dispose overtook settles with the typed disposed error.
        {
          const second = await createMisoAudioWorkletHost({
            context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
            workletModuleUrl: "https://example.test/w.js",
          });
          const secondWorker = workers.at(-1);
          const overtaken = second.planLatency();
          const first = second.dispose();
          assert.equal(second.dispose(), first, "a second dispose is the first one's promise");
          await errorResult(second.sessionMap(), 3);
          await errorResult(second.control(new Uint8Array([1])), 3);
          await first;
          assert.deepEqual(secondWorker.posts.map((message) => message.tag).filter((tag) => tag !== "miso.worker-boot.v1"),
            ["miso.plan-latency.v1", "miso.dispose.v1"], "nothing is posted behind the dispose, once");
          assert.equal(secondWorker.terminated, true);
          await errorResult(overtaken, 3);
        }

        // S7 verify MINOR 1: a Worker error between its boot reply and the worklet's ready is not
        // lost: the host the factory returns is failed, so a request rejects instead of hanging.
        {
          const gate = snapshotGate();
          readyGate = gate;
          const pending = createMisoAudioWorkletHost({
            context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
            workletModuleUrl: "https://example.test/w.js",
          });
          await gate.started;
          workers.at(-1).onerror?.({ preventDefault() {} });
          gate.release();
          const failedHost = await pending;
          await errorResult(failedHost.control(new Uint8Array([1])), 255);
        }
        // D6: a Worker `error` or `messageerror` before the boot reply rejects with `worker-failed`,
        // no worklet starts and nothing falls back to `single` mode. A refused boot rejects with
        // the engine's result code.
        for (const behaviour of ["error", "messageerror"]) {
          workerBehaviour = behaviour;
          const before = FakeNode.count;
          await assert.rejects(createMisoAudioWorkletHost({
            context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
            workletModuleUrl: "https://example.test/w.js",
          }), (error) => {
            assert.deepEqual(error, {
              tag: "miso.worker-failed.v1", requestId: 0, result: 255, reason: "worker-failed",
            });
            return true;
          }, `a Worker ${behaviour} is worker-failed`);
          assert.equal(FakeNode.count, before, `no worklet starts after a Worker ${behaviour}`);
          assert.equal(workers.at(-1).terminated, true);
        }
        workerBehaviour = "refuse";
        await assert.rejects(createMisoAudioWorkletHost({
          context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
          workletModuleUrl: "https://example.test/w.js",
        }), (error) => error.tag === "miso.error.v1" && error.result === 3);
        workerBehaviour = "ready";
        // An explicit `single` runs one instance on an isolated page; `worker` on a page without
        // isolation is refused.
        const workersBefore = workers.length;
        const single = await createMisoAudioWorkletHost({
          context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
          workletModuleUrl: "processor.js", instanceMode: "single",
        });
        assert.equal(single.instanceMode, "single");
        assert.equal(workers.length, workersBefore, "an explicit single mode starts no Worker");
        await single.dispose();
        Object.defineProperty(globalThis, "crossOriginIsolated", { value: false, configurable: true });
        await errorResult(createMisoAudioWorkletHost({
          context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
          workletModuleUrl: "processor.js", instanceMode: "worker",
        }), 1);
        const plain = await createMisoAudioWorkletHost({
          context, document: new Uint8Array(), options: limits, simd128ModuleUrl: "simd.wasm",
          workletModuleUrl: "processor.js",
        });
        assert.equal(plain.instanceMode, "single", "a page without isolation runs single mode");
        await plain.dispose();
      } finally {
        globalThis.Worker = originalWorker;
        WebAssembly.Memory = originalMemory;
        if (originalIsolated === undefined) delete globalThis.crossOriginIsolated;
        else Object.defineProperty(globalThis, "crossOriginIsolated", originalIsolated);
      }
    }
    await runPlainDataMatrix();
  } finally {
    globalThis.fetch = original.fetch;
    globalThis.AudioWorkletNode = original.AudioWorkletNode;
    WebAssembly.validate = original.validate;
    WebAssembly.compile = original.compile;
    globalThis.setInterval = original.setInterval;
    globalThis.clearInterval = original.clearInterval;
  }
}

// The real constructor: `testProcessor` stubs `WebAssembly.Memory` while its cases run.
const SharedMemory = WebAssembly.Memory;

// `submixIds` (#1209 MINOR-2, #1210 MINOR-1): the session's submix IDs in canonical order; the
// meter header's `submix_count` and the frame layout follow it, so a caller can run the worklet
// with no submix (the production case before K2) or with several (their order is observable).
function createFakeExports(quantum, backend = 1, submixIds = ["drums-bus"]) {
  // Three pages, so the staged document at 150,000 lies beyond the first two. The module's memory
  // is shared (#1380), so the fake's is a real shared memory too (#1332 S3).
  const memory = new SharedMemory({ initial: 3, maximum: 48, shared: true });
  const statusPointer = 16384;
  const resourcePointer = 17000;
  let status = new DataView(memory.buffer, statusPointer, 80);
  status.setUint32(0, 80, true);
  status.setUint32(4, 0x00010000, true);
  status.setUint32(8, 2, true);
  status.setUint32(16, backend, true);
  status.setUint32(20, 48000, true);
  status.setUint32(24, quantum, true);
  const resources = new DataView(memory.buffer, resourcePointer, 224);
  resources.setUint32(0, 224, true);
  resources.setUint32(4, 0x00010000, true);
  resources.setUint32(8, 48000, true);
  resources.setUint32(12, quantum, true);
  resources.setUint32(16, backend, true);
  // Twenty-one u64 rows now: issue #143 carved `observationRetainedBytes` from the first of the
  // report's four reserved words, so offset 192 is a real row and 200..216 stay required zero.
  for (let index = 0; index < 21; index += 1) resources.setBigUint64(32 + index * 8, 1n, true);
  const calls = {
    render: [], source: [], sourceIdBytes: [], seek: [], seekIdBytes: [], dispose: 0,
    sessionSeek: [], sessionSeekResult: 0,
    sourceResult: 0, bootResult: 0, boot: [],
    // Issue #1381a: the service step's handles and result, and the control counter's value.
    service: [], serviceResult: 0, controlAllocations: 5,
    // Issue #1562a D1: the control frames the fake received, the result it answers with, the
    // response it leaves, the committed revision, and whether it grows its memory under a command.
    control: [], controlResult: 0, controlResponse: new Uint8Array([9, 8, 7]), controlRevision: 4n,
    controlGrowsMemory: false, controlRequestRefusals: 0, growths: 0,
    // Issue #1697: every `replace_document_ptr` length, how many the staging refuses next, and
    // the length staged for the next command (which frees it, as the export does).
    replaceDocument: [], replaceDocumentRefusals: 0, stagedDocumentBytes: 0,
    // Issue #1570b D1: the lanes' queued event frames (lane 0 reliable, lane 1 lossy), every
    // `control_event` call the fake received, and the result it answers with.
    events: [[], []], eventCalls: [], eventResult: 0,
    // Issue #1294a: the watermark record `watermark_read` copies, the result it answers with (a
    // non-zero one writes nothing, as the export's `BACKPRESSURE` does), and its call count.
    watermark: {
      revision: 3n, sample: 0n, outcome: 1n, exactCount: 3n, transitionFallbackCount: 0n,
      supersededCount: 0n,
    },
    watermarkResult: 0, watermarkReads: 0, watermarkBytes: 96,
    // Issue #1297a: the anchored seek and seek report calls in order, the results they return,
    // the words a successful read writes, and the record's struct size word.
    seekAt: [], seekAtResult: 0, seekReport: [], seekReportResult: 0, seekReportBytes: 64,
    seekReportGrowsMemory: false,
    seekReportWords: [5n, 1024n, 48000n, 7n, 9n],
    // Issue #1332 S7: the handoff calls in order, the adoption's result, the stack pointer global
    // and the successor header the control half names.
    initTls: [], reserve: [], bind: [], release: [], adopt: [], adoptResult: 0,
    stackPointer: { value: 1048576 }, successorHeader: 0,
  };
  const watermarkPointer = 66000;
  const seekReportPointer = 66200;
  // What `control_response_*` reads now: the last control command's response, or the event frame
  // the last `control_event` dequeued (empty for an empty lane), as the real exports share their
  // staging (issue #1570b D1).
  let currentEvent = null;
  const controlRequestPointer = 50000;
  const controlResponsePointer = 56000;
  const documentPointer = 150000;
  // Issue #137: the meter frame (kind 7). Kind 6 was the command staging (#1589c).
  const meterFramePointer = 40000;
  const meterHeaderPointer = 41100;
  const trackIds = ["kick", "snare"];
  // Issue #1210 D3: `submixIds` (a parameter) are the submix IDs in canonical order, as many as
  // the meter header's `submix_count` below; the worklet refuses construction when the two disagree.
  // Issue #241: the compiled session's sources, in canonical (stable-ID sorted) order. Channel and
  // frame count differ between rows, so a worklet that reads the wrong row/query is visible here.
  const sourceRows = [
    { id: "bass", channels: 1, frames: 96000n },
    { id: "drums", channels: 2, frames: 2048n },
  ];
  // Issue #1209 D1: `3(T + S) + 3` -- a peak pair per strip (the tracks, then the submixes), the
  // master pair, then one gain-reduction magnitude per strip and the master's.
  const submixCount = submixIds.length;
  const strips = trackIds.length + submixCount;
  const meterFrameFloats = strips * 3 + 3;
  // Issue #1290d1: the header is 88 bytes (#1562b appended `structure_epoch` at 80) and names its
  // frame (offset 72); `retired` (68) and `successor` (76) are zero until a replacement plan is
  // committed and adopted. The replacement
  // plan's own header and frame, sized for one more track, sit beside it: `commitMeterSuccessor`
  // names it in the active header, as the host does when a command commits a rebuild, and
  // `adoptOnNextRender` makes the next render call model the swap block (the old header is marked
  // retired and names the new one, whose stream fields are the old header's).
  const successorHeaderPointer = 41300;
  const successorFramePointer = 44000;
  const meterHeader = new DataView(memory.buffer, meterHeaderPointer, 88);
  const successorHeader = new DataView(memory.buffer, successorHeaderPointer, 88);
  for (const [header, tracks, framePointer, epoch] of [
    [meterHeader, trackIds.length, meterFramePointer, 1n],
    [successorHeader, trackIds.length + 1, successorFramePointer, 2n],
  ]) {
    header.setUint32(0, 88, true);
    header.setBigUint64(80, epoch, true);
    header.setUint32(64, submixCount, true);
    header.setUint32(4, 0x00010000, true);
    header.setUint32(8, tracks, true);
    header.setUint32(40, 1, true);
    header.setUint32(44, 1, true);
    header.setBigUint64(16, 512n, true);
    header.setBigUint64(24, 768n, true);
    header.setBigUint64(48, 1n, true);
    header.setBigUint64(56, 0xbn, true);
    header.setUint32(72, framePointer, true);
  }
  let meterAdopted = false;
  const adoptMeterPlan = () => {
    meterHeader.setUint32(68, 1, true);
    meterHeader.setUint32(76, successorHeaderPointer, true);
    for (let offset = 12; offset < 64; offset += 4) {
      successorHeader.setUint32(offset, meterHeader.getUint32(offset, true), true);
    }
    meterAdopted = true;
  };
  const pointers = {
    2: 4096, 3: 5000, 4: 150000, 5: 8192,
    7: meterFramePointer,
  };
  const capacities = {
    2: 64, 3: 2 * quantum * 4, 4: 256, 5: 2 * quantum * 4,
    7: meterFrameFloats * 4,
  };
  calls.meterLease = [];
  calls.meterWindows = 0;
  const boot = () => {
    const options = new DataView(memory.buffer, 512, 64);
    if (options.getUint32(8, true) !== status.getUint32(20, true)
        || options.getUint32(12, true) !== status.getUint32(24, true)) {
      calls.bootResult = 9;
      return 0;
    }
    calls.bootResult = 0;
    return 1;
  };
  // A real shared linear-memory growth (doubling): `memory.buffer` becomes a new object over the
  // grown length, every view over the older one stays valid for its bytes, and the fake's own
  // views follow the new object.
  const growMemory = () => {
    memory.grow(memory.buffer.byteLength / 65536);
    status = new DataView(memory.buffer, statusPointer, 80);
    calls.growths += 1;
  };
  const exports = {
    memory,
    miso_engine_web_v1_abi_version: () => 0x00010000,
    miso_engine_web_v1_boot_options_ptr: () => 512,
    miso_engine_web_v1_document_ptr: () => 2048,
    // #2166: the boot command is staged at a fixed address and read back when the boot takes it.
    miso_engine_web_v1_boot_command_ptr: (length) => { calls.bootCommandLength = length; return 190000; },
    miso_engine_web_v1_boot: (...args) => {
      calls.boot.push(args);
      if (calls.bootCommandLength !== undefined) {
        calls.bootCommand = new Uint8Array(memory.buffer, 190000, calls.bootCommandLength).slice();
        calls.bootCommandLength = undefined;
      }
      return boot();
    },
    miso_engine_web_v1_boot_result: () => calls.bootResult,
    miso_engine_web_v1_boot_diagnostic_bytes: () => 0,
    miso_engine_web_v1_buffer_ptr: (_handle, kind) => pointers[kind] ?? 0,
    miso_engine_web_v1_buffer_capacity: (_handle, kind) => capacities[kind] ?? 0,
    miso_engine_web_v1_status_ptr: () => statusPointer,
    miso_engine_web_v1_resource_ptr: () => resourcePointer,
    miso_engine_web_v1_render: (_handle, actualFrames) => {
      calls.render.push(actualFrames);
      if (actualFrames !== quantum) return 9;
      new Float32Array(memory.buffer, 8192, quantum).fill(0.25);
      new Float32Array(memory.buffer, 8192 + quantum * 4, quantum).fill(-0.25);
      if (calls.adoptOnNextRender) {
        calls.adoptOnNextRender = false;
        adoptMeterPlan();
      }
      const quanta = status.getBigUint64(40, true) + 1n;
      status.setBigUint64(40, quanta, true);
      status.setBigUint64(32, quanta * BigInt(quantum), true);
      return 0;
    },
    miso_engine_web_v1_source_submit: (...args) => {
      calls.source.push(args);
      calls.sourceIdBytes.push(new Uint8Array(memory.buffer, pointers[2], args[1]).slice());
      return calls.sourceResult;
    },
    // Issue #1644: both session seek exports and the declaration record into one list, in order.
    miso_engine_web_v1_session_seek: (...args) => {
      calls.sessionSeek.push(["session_seek", ...args]);
      return calls.sessionSeekResult;
    },
    miso_engine_web_v1_session_seek_at: (...args) => {
      calls.sessionSeek.push(["session_seek_at", ...args]);
      return calls.sessionSeekResult;
    },
    miso_engine_web_v1_declare_discontinuity: (...args) => {
      calls.sessionSeek.push(["declare_discontinuity", ...args]);
      return 0;
    },
    miso_engine_web_v1_source_seek_at: (...args) => {
      calls.seekAt.push([...args, new TextDecoder().decode(
        new Uint8Array(memory.buffer, pointers[2], args[1]).slice())]);
      return calls.seekAtResult;
    },
    miso_engine_web_v1_source_seek_report_ptr: () => seekReportPointer,
    miso_engine_web_v1_plan_latency_ptr: () => 66300,
    miso_engine_web_v1_source_seek_report: (...args) => {
      calls.seekReport.push([...args, new TextDecoder().decode(
        new Uint8Array(memory.buffer, pointers[2], args[1]).slice())]);
      if (calls.seekReportResult !== 0) return calls.seekReportResult;
      // The service step the real export runs first may grow the memory.
      if (calls.seekReportGrowsMemory) growMemory();
      const record = new DataView(memory.buffer, seekReportPointer, 64);
      record.setUint32(0, calls.seekReportBytes, true);
      calls.seekReportWords.forEach((word, index) => record.setBigUint64(8 + index * 8, word, true));
      return 0;
    },
    miso_engine_web_v1_source_seek: (...args) => {
      calls.seek.push(args);
      calls.seekIdBytes.push(new Uint8Array(memory.buffer, pointers[2], args[1]).slice());
      return 0;
    },
    miso_engine_web_v1_meter_lease: (_handle, enabled) => {
      calls.meterLease.push(enabled);
      return 0;
    },
    miso_engine_web_v1_meter_header_ptr: () => (
      meterAdopted ? successorHeaderPointer : meterHeaderPointer),
    miso_engine_web_v1_meter_poll: () => {
      if (calls.meterWindows === 0) return 0;
      calls.meterWindows -= 1;
      // Issue #143: the sections carry **different** values, so a worklet that read the peak
      // view where the gain-reduction view belongs (or the reverse) is visible here rather than
      // hidden by a uniform fill. Issue #1209: so do the submix sections and the master pair,
      // which follows the submix peaks, so a worklet that reads the master after the tracks or
      // a bus from a track's slot is visible too.
      const tracks = trackIds.length + (meterAdopted ? 1 : 0);
      const stripsNow = tracks + submixCount;
      const frame = new Float32Array(
        memory.buffer,
        meterAdopted ? successorFramePointer : meterFramePointer,
        stripsNow * 3 + 3,
      );
      frame.fill(0.5, 0, tracks * 2);
      frame.fill(0.75, tracks * 2, stripsNow * 2);
      frame.fill(0.625, stripsNow * 2, stripsNow * 2 + 2);
      frame.fill(6.5, stripsNow * 2 + 2, stripsNow * 2 + 2 + tracks);
      frame.fill(2.25, stripsNow * 2 + 2 + tracks, stripsNow * 3 + 2);
      frame[stripsNow * 3 + 2] = 7.5;
      return 1;
    },
    miso_engine_web_v1_source_count: () => sourceRows.length,
    miso_engine_web_v1_resolved_source_ring_frames: () => 40 * quantum,
    miso_engine_web_v1_source_id: (_handle, index) => {
      const id = sourceRows[index]?.id;
      if (id === undefined) return 0;
      const bytes = new Uint8Array(memory.buffer, pointers[2], id.length);
      for (let byte = 0; byte < id.length; byte += 1) bytes[byte] = id.charCodeAt(byte);
      return id.length;
    },
    miso_engine_web_v1_source_channels: (_handle, index) => sourceRows[index]?.channels ?? 0,
    miso_engine_web_v1_source_frames: (_handle, index) => sourceRows[index]?.frames ?? 0n,
    miso_engine_web_v1_service: (handle) => {
      calls.service.push(handle);
      return calls.serviceResult;
    },
    miso_engine_web_v1_watermark_ptr: () => watermarkPointer,
    miso_engine_web_v1_watermark_read: () => {
      calls.watermarkReads += 1;
      if (calls.watermarkResult !== 0) return calls.watermarkResult;
      const record = new DataView(memory.buffer, watermarkPointer, 96);
      const words = calls.watermark;
      record.setUint32(0, calls.watermarkBytes, true);
      record.setUint32(4, 0, true);
      [
        words.revision, words.sample, words.outcome, words.exactCount,
        words.transitionFallbackCount, words.supersededCount,
      ].forEach((word, index) => record.setBigUint64(8 + index * 8, word, true));
      return 0;
    },
    miso_engine_web_v1_control_allocation_count: () => calls.controlAllocations,
    miso_engine_web_v1_control_request_ptr: (_handle, length) => {
      if (calls.controlRequestRefusals > 0 || length === 0 || length > 16384) {
        calls.controlRequestRefusals = Math.max(0, calls.controlRequestRefusals - 1);
        return 0;
      }
      return controlRequestPointer;
    },
    miso_engine_web_v1_replace_document_ptr: (_handle, length) => {
      calls.replaceDocument.push(length);
      calls.stagedDocumentBytes = 0;
      if (calls.replaceDocumentRefusals > 0 || length === 0 || length > 40000) {
        calls.replaceDocumentRefusals = Math.max(0, calls.replaceDocumentRefusals - 1);
        return 0;
      }
      calls.stagedDocumentBytes = length;
      return documentPointer;
    },
    miso_engine_web_v1_control_command: (handle, length) => {
      const documentBytes = calls.stagedDocumentBytes;
      calls.stagedDocumentBytes = 0;
      calls.control.push({
        handle,
        frame: new Uint8Array(memory.buffer, controlRequestPointer, length).slice(),
        document: documentBytes === 0
          ? null
          : new Uint8Array(memory.buffer, documentPointer, documentBytes).slice(),
      });
      new Uint8Array(memory.buffer, controlResponsePointer, calls.controlResponse.length)
        .set(calls.controlResponse);
      if (calls.controlGrowsMemory) growMemory();
      currentEvent = null;
      return calls.controlResult;
    },
    miso_engine_web_v1_control_event: (handle, lane) => {
      calls.eventCalls.push({ handle, lane });
      currentEvent = new Uint8Array(0);
      if (calls.eventResult !== 0) return calls.eventResult;
      const frame = calls.events[lane]?.shift();
      if (frame !== undefined) {
        currentEvent = frame;
        new Uint8Array(memory.buffer, controlResponsePointer, frame.length).set(frame);
      }
      return 0;
    },
    miso_engine_web_v1_control_response_ptr: () => {
      const response = currentEvent ?? calls.controlResponse;
      return response.length === 0 ? 0 : controlResponsePointer;
    },
    miso_engine_web_v1_control_response_bytes: () => (currentEvent ?? calls.controlResponse).length,
    miso_engine_web_v1_control_revision: () => calls.controlRevision,
    miso_engine_web_v1_dispose: () => {
      calls.dispose += 1;
      return 0;
    },
    // Issue #1332 S7: the exports of the two-instance handoff. The globals are objects with a
    // `value`, as the module's exported globals are.
    __stack_pointer: calls.stackPointer,
    __tls_size: { value: 64 },
    __tls_align: { value: 16 },
    __wasm_init_tls: (base) => {
      calls.initTls.push([base, calls.stackPointer.value]);
    },
    miso_engine_web_v1_instance_reserve: (size, align) => {
      calls.reserve.push([size, align]);
      const record = new DataView(memory.buffer, 70000, 16);
      record.setUint32(0, 16, true);
      record.setUint32(4, 71000, true);
      record.setUint32(8, 180000, true);
      record.setUint32(12, 170000, true);
      return 70000;
    },
    miso_engine_web_v1_instance_bind: (record) => {
      calls.bind.push(record);
      return 0;
    },
    miso_engine_web_v1_host_release: (handle) => {
      calls.release.push(handle);
      return (BigInt(handle) << 32n) | 1n;
    },
    miso_engine_web_v1_host_adopt: (token) => {
      calls.adopt.push(token);
      return calls.adoptResult;
    },
    miso_engine_web_v1_successor_header_ptr: () => calls.successorHeader,
  };
  return {
    exports, calls, trackIds, submixIds, sourceRows, meterFrameFloats, meterHeader, growMemory,
    successorHeader, commitMeterSuccessor: () => meterHeader.setUint32(76, successorHeaderPointer, true),
    memory,
  };
}

function createTelemetryClock(elapsedMsByBlock) {
  let reads = 0;
  let timeMs = 0;
  return {
    now() {
      const block = Math.floor(reads / 2);
      if (block >= elapsedMsByBlock.length) throw new Error("telemetry clock read past fixture");
      if (reads % 2 === 1) timeMs += elapsedMsByBlock[block];
      reads += 1;
      return timeMs;
    },
    get reads() {
      return reads;
    },
  };
}

function withTelemetryClock(clock, callback) {
  const originalPerformance = Object.getOwnPropertyDescriptor(globalThis, "performance");
  Object.defineProperty(globalThis, "performance", {
    configurable: true,
    enumerable: originalPerformance?.enumerable ?? true,
    writable: true,
    value: { now: clock.now },
  });
  try {
    return callback();
  } finally {
    if (originalPerformance === undefined) delete globalThis.performance;
    else Object.defineProperty(globalThis, "performance", originalPerformance);
  }
}

// Issue #288: exercise the qualification caller's real boot objects through the same host and
// worklet guards as the main-realm suite. The stop sentinel deliberately ends each caller after
// real create/dispose, before any source/render/stall loop; the diagnostic initializer still runs.
async function testQualificationBoot({
  registered, makeFake, setNextFake, setProcessorPortFactory, document, mainRealmTextEncoder,
}) {
  const originalOfflineAudioContext = globalThis.OfflineAudioContext;
  const originalAudioWorkletNode = globalThis.AudioWorkletNode;
  const originalFetch = globalThis.fetch;
  const originalCompile = WebAssembly.compile;
  const originalValidate = WebAssembly.validate;
  const originalSampleRate = globalThis.sampleRate;
  const originalRenderQuantumSize = globalThis.renderQuantumSize;
  const qualificationUrlWithCacheKey = `${qualificationUrl.href}?boot-contract-test`;
  const {
    createMisoAudioWorkletHost,
  } = await import(`${hostUrl.href}?qualification-boot-host`);
  const { qualificationBootContract: hooks } = await import(qualificationUrlWithCacheKey);
  const observed = [];
  const fetchUrls = [];
  let sentinelStops = 0;
  let realReady = 0;
  let realDisposed = 0;

  class QualificationPort {
    constructor() {
      this.peer = null;
      this.handler = null;
      this.pending = [];
      this.closed = false;
    }

    get onmessage() {
      return this.handler;
    }

    set onmessage(handler) {
      this.handler = handler;
      if (typeof handler !== "function") return;
      for (const event of this.pending.splice(0)) queueMicrotask(() => this.handler?.(event));
    }

    postMessage(message, transfer = []) {
      this.peer?.deliver(message, transfer);
    }

    deliver(message, transfer = []) {
      const event = { data: structuredClone(message, { transfer }) };
      if (typeof this.handler !== "function") this.pending.push(event);
      else queueMicrotask(() => this.handler?.(event));
    }

    close() {
      this.closed = true;
    }
  }

  class QualificationOfflineAudioContext {
    constructor(numberOfChannels, length, sampleRate) {
      this.numberOfChannels = numberOfChannels;
      this.length = length;
      this.sampleRate = sampleRate;
      this.renderQuantumSize = 128;
      this.state = "suspended";
      this.destination = {};
      this.audioWorklet = {
        addModule: async () => undefined,
      };
    }

  }

  class QualificationNode {
    static latest = null;

    constructor(context, _name, options = {}) {
      const hostPort = new QualificationPort();
      const processorPort = new QualificationPort();
      hostPort.peer = processorPort;
      processorPort.peer = hostPort;
      this.port = hostPort;
      this.options = options;
      this.onprocessorerror = null;
      this.disconnectCount = 0;
      this.fake = makeFake(context.renderQuantumSize, 1);
      setNextFake(this.fake);
      setProcessorPortFactory(() => processorPort);
      try {
        this.processor = new registered({ processorOptions: options.processorOptions });
      } finally {
        setProcessorPortFactory(null);
      }
      QualificationNode.latest = this;
    }

    disconnect() {
      this.disconnectCount += 1;
    }
  }

  class QualificationStop extends Error {
    constructor(label) {
      super(`qualification boot probe stopped before ${label} loop`);
      this.label = label;
    }
  }

  globalThis.OfflineAudioContext = QualificationOfflineAudioContext;
  globalThis.AudioWorkletNode = QualificationNode;
  globalThis.sampleRate = 48000;
  globalThis.renderQuantumSize = 128;
  globalThis.fetch = async (url) => {
    fetchUrls.push(String(url));
    return {
      ok: true,
      arrayBuffer: async () => new Uint8Array([0]).buffer,
      json: async () => abiLayout,
    };
  };
  WebAssembly.compile = async () => Object.freeze({ qualification: true });
  WebAssembly.validate = () => true;

  const bounded = (promise, label) => {
    let timer;
    const timeout = new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(`qualification boot contract: ${label} timed out`)), 1000);
    });
    return Promise.race([promise, timeout]).finally(() => clearTimeout(timer));
  };
  // Issue #1477 D1: the host runs in the main realm, which has `TextEncoder`; the processor scope
  // this suite models has none. The host's option guards run synchronously before its first
  // await, so the encoder is present for exactly that span and absent again before any worklet
  // code runs.
  const createInMainRealm = (factory) => {
    const processorScopeEncoder = globalThis.TextEncoder;
    globalThis.TextEncoder = mainRealmTextEncoder;
    try {
      return createMisoAudioWorkletHost(factory);
    } finally {
      globalThis.TextEncoder = processorScopeEncoder;
    }
  };
  const forwardingCreateHost = (label) => async (options) => {
    observed.push({ label, options });
    if (options.document !== undefined) {
      assert(options.document instanceof Uint8Array && options.document.byteLength > 0,
        `qualification boot contract: ${label} document must be nonempty`);
    }
    assert.equal(options.context.sampleRate, 48000,
      `qualification boot contract: ${label} sample rate mismatch`);
    assert.equal(options.context.renderQuantumSize, 128,
      `qualification boot contract: ${label} quantum mismatch`);
    if (label === "renderCorpusSegment" && options.options !== undefined) {
      await assert.rejects(
        () => createInMainRealm({
          ...options,
          options: { ...options.options, qualificationExtra: true },
        }),
        (error) => error?.tag === "miso.error.v1" && error.result === 1,
        "qualification boot contract: real guard must reject an extra option",
      );
    }
    let host;
    try {
      host = await bounded(createInMainRealm(options), `${label} real host`);
    } catch (error) {
      throw new Error(
        `qualification boot contract: ${label} real host guard rejected `
        + `(${error?.tag ?? error?.message ?? String(error)} result=${error?.result ?? "?"})`,
      );
    }
    assert.equal(host.backend, "simd128", `qualification boot contract: ${label} backend`);
    // Issue #1491 D1: exactly one boot per caller.
    assert.equal(QualificationNode.latest.fake.calls.boot.length, 1,
      `qualification boot contract: ${label} must boot exactly once`);
    realReady += 1;
    try {
      await bounded(host.dispose(), `${label} real dispose`);
    } catch (error) {
      throw new Error(`qualification boot contract: ${label} real host dispose failed: ${error}`);
    }
    realDisposed += 1;
    if (label === "typedUnsupportedAttestation") {
      // The real guard has already accepted and disposed this boot. The helper's catch is then
      // exercised with the typed refusal it expects, instead of mistaking a guard failure for it.
      throw Object.freeze({
        tag: "miso.unsupported.v1", requestId: 0, result: 7, capability: "simd128",
      });
    }
    throw new QualificationStop(label);
  };

  const expectStop = async (label, operation) => {
    try {
      await operation();
    } catch (error) {
      if (error instanceof QualificationStop && error.label === label) {
        sentinelStops += 1;
        return;
      }
      throw error;
    }
    throw new Error(`qualification boot contract: ${label} did not reach its stop sentinel`);
  };

  try {
    const documentBytes = new Uint8Array(document);
    await expectStop("renderCorpusSegment", () => hooks.renderCorpusSegment(
      forwardingCreateHost("renderCorpusSegment"),
      documentBytes,
      [{ startFrame: 0, leftBase: 0, leftStep: 0, frames: 128, final: true }],
    ));
    assert.equal(
      await hooks.typedUnsupportedAttestation(
        forwardingCreateHost("typedUnsupportedAttestation"), documentBytes,
      ),
      true,
      "qualification boot contract: typed unsupported catch did not receive its typed refusal",
    );
    await expectStop("runLiveControlQualification", () => hooks.runLiveControlQualification(
      forwardingCreateHost("runLiveControlQualification"), documentBytes,
    ));
    await expectStop("runObservationRun", () => hooks.runObservationRun(
      forwardingCreateHost("runObservationRun"), documentBytes, true,
    ));
    await expectStop("runStallQualification", () => hooks.runStallQualification(
      forwardingCreateHost("runStallQualification"), documentBytes,
    ));
    // Issue #1477 D1: both forms of the staging-read run boot the same options before they part,
    // so one call witnesses the shape.
    await expectStop("runStagingReadRun", () => hooks.runStagingReadRun(
      forwardingCreateHost("runStagingReadRun"), documentBytes, false,
    ));

    assert.deepEqual(
      observed.map(({ label }) => label),
      [
        "renderCorpusSegment", "typedUnsupportedAttestation", "runLiveControlQualification",
        "runObservationRun", "runStallQualification", "runStagingReadRun",
      ],
      "qualification boot contract: caller witness set changed",
    );
    assert.equal(realReady, 6, "qualification boot contract: six real ready witnesses required");
    assert.equal(realDisposed, 6, "qualification boot contract: six real dispose witnesses required");
    assert.equal(sentinelStops, 5, "qualification boot contract: every non-attestation path must stop its sentinel");
    const plain = observed.filter(({ label }) =>
      label === "renderCorpusSegment" || label === "typedUnsupportedAttestation");
    assert(plain.every(({ options }) => options.options.reserved1 === 0n
      && options.options.liveControlMeterBlocks === 0n
      && options.options.reserved3 === 0n
      && options.options.reserved2 === 0n),
    "qualification boot contract: plain option variant changed");
    const liveControlOptions = observed
      .find(({ label }) => label === "runLiveControlQualification").options.options;
    assert.equal(liveControlOptions.reserved1, 0n);
    assert.equal(liveControlOptions.liveControlMeterBlocks, 2n);
    assert.equal(liveControlOptions.reserved3, 0n);
    assert.equal(liveControlOptions.reserved2, 0n);
    const observationOptions = observed.find(({ label }) => label === "runObservationRun").options.options;
    assert.equal(observationOptions.reserved1, 0n);
    assert.equal(observationOptions.liveControlMeterBlocks, 2n);
    assert.equal(observationOptions.reserved3, 0n);
    assert.equal(observationOptions.reserved2, 0n);
    // Issue #1477 D2: the staging-read run boots the observation caller's live-control fields.
    const stagingReadOptions = observed.find(({ label }) => label === "runStagingReadRun").options.options;
    assert.equal(stagingReadOptions.reserved1, 0n);
    assert.equal(stagingReadOptions.liveControlMeterBlocks, 2n);
    assert.equal(stagingReadOptions.reserved3, 0n);
    assert.equal(stagingReadOptions.reserved2, 0n);

    const diagnosis = await bounded(hooks.diagnoseReady(documentBytes), "diagnoseReady");
    assert.equal(diagnosis.kind, "message", "qualification boot contract: diagnoseReady did not reach node message");
    assert.equal(diagnosis.message.tag, "miso.ready.v1",
      `qualification boot contract: diagnoseReady initializer was refused `
      + `(tag=${diagnosis.message.tag} result=${diagnosis.message.result})`);
    assert.equal(diagnosis.message.result, 0,
      `qualification boot contract: diagnoseReady initializer was refused `
      + `(tag=${diagnosis.message.tag} result=${diagnosis.message.result})`);
    assert.equal(diagnosis.message.backend, "simd128");
    const diagnosticNode = QualificationNode.latest;
    assert(diagnosticNode?.processor instanceof registered,
      "qualification boot contract: diagnoseReady did not use the registered real worklet class");
    assert.equal(diagnosticNode.processor.ready, true,
      "qualification boot contract: diagnoseReady initializer did not become ready");
    assert(diagnosticNode.options.processorOptions.document instanceof Uint8Array
      && diagnosticNode.options.processorOptions.document.byteLength > 0);
    assert(fetchUrls.length >= 6 && fetchUrls.every((url) => typeof url === "string"),
      "qualification boot contract: fetch escaped the hermetic fixture");
    console.log(
      `qualification boot contract passed: callers=${observed.length} `
      + `real-ready=${realReady} real-disposed=${realDisposed} diagnose-ready=1`,
    );
  } finally {
    if (originalOfflineAudioContext === undefined) delete globalThis.OfflineAudioContext;
    else globalThis.OfflineAudioContext = originalOfflineAudioContext;
    if (originalAudioWorkletNode === undefined) delete globalThis.AudioWorkletNode;
    else globalThis.AudioWorkletNode = originalAudioWorkletNode;
    if (originalFetch === undefined) delete globalThis.fetch;
    else globalThis.fetch = originalFetch;
    WebAssembly.compile = originalCompile;
    WebAssembly.validate = originalValidate;
    if (originalSampleRate === undefined) delete globalThis.sampleRate;
    else globalThis.sampleRate = originalSampleRate;
    if (originalRenderQuantumSize === undefined) delete globalThis.renderQuantumSize;
    else globalThis.renderQuantumSize = originalRenderQuantumSize;
    setProcessorPortFactory(null);
  }
}

async function testProcessor() {
  const originalProcessor = globalThis.AudioWorkletProcessor;
  const originalRegister = globalThis.registerProcessor;
  const originalInstance = WebAssembly.Instance;
  const originalMemory = WebAssembly.Memory;
  const originalSampleRate = globalThis.sampleRate;
  const originalRenderQuantumSize = globalThis.renderQuantumSize;
  const originalTextEncoder = globalThis.TextEncoder;
  const processorSessionDocument = new originalTextEncoder().encode("{\"schema_version\":0}");
  const nonAsciiSourceId = "caf\u00e9-\u96ea-\ud83d\ude00";
  const nonAsciiSourceIdUtf8 = new originalTextEncoder().encode(nonAsciiSourceId);
  let registered;
  let nextFake;
  let instanceCount = 0;
  let throwInstance = false;
  let throwReadyPost = false;
  let processorPortFactory = null;
  // Issue #1496 D3: a call counts once the processor that made it has posted `miso.ready.v1`;
  // everything before is construction. The recording proxy is installed by the stubbed
  // `WebAssembly.Instance`, which every processor of this suite (the cases and the qualification
  // boot alike) constructs, and `readyPorts` is the only state it reads.
  const readyPorts = new WeakSet();
  const recordedPostConstructionCalls = new Set();
  let latestPort = null;
  // A read of an export after ready is recorded as well as each call, so a worklet that reaches
  // for an export the fakes do not define is named instead of failing as a bare TypeError, and a
  // function the worklet cached before ready is still recorded when it is called later.
  const recordingExports = (exports, port) => new Proxy(exports, {
    get(target, property, receiver) {
      const value = Reflect.get(target, property, receiver);
      if (typeof property !== "string" || !property.startsWith(renderThreadPrefix)) return value;
      const name = property.slice(renderThreadPrefix.length);
      if (readyPorts.has(port)) {
        recordedPostConstructionCalls.add(name);
        // The fakes define only the exports the cases exercise. An unlisted export that they do
        // not define would stop the run with a bare TypeError before the final check names it.
        if (value === undefined && !engineRenderThreadExports.has(name)) return () => 0;
      }
      if (typeof value !== "function") return value;
      return (...args) => {
        if (readyPorts.has(port)) recordedPostConstructionCalls.add(name);
        return Reflect.apply(value, receiver, args);
      };
    },
  });
  class FakePort {
    onmessage = null;
    posts = [];

    postMessage(message, transfer = []) {
      if (throwReadyPost && message?.tag === "miso.ready.v1") {
        throw new Error("synthetic ready publication failure");
      }
      this.posts.push({ message: structuredClone(message, { transfer }), transferCount: transfer.length });
    }
  }
  class FakeProcessor {
    constructor() {
      this.port = processorPortFactory?.() ?? new FakePort();
      latestPort = this.port;
      const port = this.port;
      const post = port.postMessage;
      port.postMessage = function (message, ...rest) {
        const result = Reflect.apply(post, this, [message, ...rest]);
        if (message?.tag === "miso.ready.v1") readyPorts.add(port);
        return result;
      };
    }
  }
  globalThis.AudioWorkletProcessor = FakeProcessor;
  globalThis.registerProcessor = (_name, implementation) => { registered = implementation; };
  globalThis.sampleRate = 48000;
  globalThis.renderQuantumSize = 64;
  globalThis.TextEncoder = undefined;
  // Issue #1380 D3: every instance the worklet makes imports exactly `{ env: { memory } }`, a
  // memory it created itself as shared at the ABI layout's page counts. The stub records each
  // descriptor (no real 512 MiB reservation per case); a violation is collected rather than thrown,
  // since the worklet turns a throw at instantiation into a typed failure, and checked at the end.
  const memoryDescriptors = new WeakMap();
  // A memory one instance already imported: a second instance on it would share it (#1380 D3).
  const importedMemories = new WeakSet();
  const importViolations = [];
  let importsChecked = 0;
  WebAssembly.Memory = class {
    constructor(descriptor) {
      memoryDescriptors.set(this, { ...descriptor });
      this.buffer = new SharedArrayBuffer(descriptor.initial * 65536);
    }
  };
  const expectedMemory = {
    initial: abiLayout.memory.initialPages,
    maximum: abiLayout.memory.maximumPages,
    shared: true,
  };
  // Issue #1332 S7: in `worker` mode the worklet joins the memory its options carry.
  let workerModeMemory = null;
  WebAssembly.Instance = class {
    constructor(_module, imports) {
      instanceCount += 1;
      if (workerModeMemory !== null) {
        assert.equal(imports.env.memory, workerModeMemory, "worker mode joins the Worker's memory");
        this.exports = recordingExports(nextFake.exports, latestPort);
        return;
      }
      importsChecked += 1;
      try {
        assert.deepEqual(Object.keys(imports), ["env"]);
        assert.deepEqual(Object.keys(imports.env), ["memory"]);
        assert.deepEqual(memoryDescriptors.get(imports.env.memory), expectedMemory);
        assert.ok(!importedMemories.has(imports.env.memory), "the memory is fresh for this instance");
        importedMemories.add(imports.env.memory);
      } catch (error) {
        importViolations.push(error.message);
      }
      if (throwInstance) throw new Error("synthetic instantiation failure");
      this.exports = recordingExports(nextFake.exports, latestPort);
    }
  };
  try {
    await import(`${workletUrl.href}?processor-test`);
    assert.equal(typeof registered, "function");
    const construct = (fake, options = limits, bootCommand = undefined) => {
      nextFake = fake;
      return new registered({
        processorOptions: {
          module: {},
          document: processorSessionDocument,
          options,
          ...(bootCommand === undefined ? {} : { bootCommand }),
        },
      });
    };
    const makeProcessor = (submixIds = undefined) => {
      const fake = createFakeExports(64, 1, submixIds);
      const processor = construct(fake);
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "simd128",
        resources: processor.resources, memoryBytes: 196608, sourceRingFrames: 40 * 64,
        instanceMode: "single",
      });
      return { processor, fake };
    };

    // Issue #1689 gate 2: the processor's `writeBootOptions` refuses the retired master word before
    // the module sees it, at both ends of its range; zero (every `limits` boot here) is accepted.
    for (const reserved2 of [1n, 0xffff_ffff_ffff_ffffn]) {
      const fake = createFakeExports(64);
      const processor = construct(fake, { ...limits, reserved2 });
      assert.equal(processor.port.posts[0].message.result, 1, `reserved2 ${reserved2} is refused`);
      assert.deepEqual(fake.calls.boot, [], `reserved2 ${reserved2} never reaches the module`);
    }

    {
      const fake = createFakeExports(64);
      const processor = construct(fake);
      assert.equal(processor.port.posts[0].message.result, 0, "the boot is ready");
      assert.deepEqual(fake.calls.boot, [[processorSessionDocument.byteLength]],
        "the boot export's arguments");
    }

    // #2166 parity row: the processor stages the boot command before the boot export, so the
    // engine prepares the first plan with it; a boot without one stages nothing.
    {
      const fake = createFakeExports(64);
      const processor = construct(fake, limits, new Uint8Array([7, 6, 5, 4]));
      assert.equal(processor.port.posts[0].message.result, 0, "the boot with a command is ready");
      assert.deepEqual([...fake.calls.bootCommand], [7, 6, 5, 4], "the boot took the staged command");
      const plain = createFakeExports(64);
      construct(plain);
      assert.equal(plain.calls.bootCommand, undefined, "a boot without a command stages none");
    }

    // #1597: the processor's boot options lost their spectrum fields; one that comes back is
    // refused before the module is instantiated.
    {
      const fake = createFakeExports(64);
      const processor = construct(fake, { ...limits, spectrum: null, spectrumCollection: null });
      assert.equal(processor.port.posts[0].message.result, 1, "a retired spectrum field is refused");
      assert.deepEqual(fake.calls.boot, [], "a retired spectrum field never reaches the module");
    }

    {
      const fake = createFakeExports(64);
      fake.exports.miso_engine_web_v1_abi_version = () => 0x00020000;
      const processor = construct(fake);
      assert.equal(processor.port.posts[0].message.result, 2, "ABI mismatch is a typed refusal");
      assert.deepEqual(fake.calls.boot, [], "ABI mismatch never calls boot");
    }

    {
      const before = instanceCount;
      globalThis.sampleRate = 44100;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      globalThis.sampleRate = 48000;
      assert.equal(instanceCount, before + 1, "physical mismatch is answered by atomic boot");
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 9);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      const before = instanceCount;
      globalThis.renderQuantumSize = 128;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      globalThis.renderQuantumSize = 64;
      assert.equal(instanceCount, before + 1, "physical mismatch is answered by atomic boot");
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 9);
    }

    {
      throwInstance = true;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      throwInstance = false;
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 255);
      assert.equal(processor.disposed, true);
    }

    for (const [result, mutate] of [
      [255, (fake) => { fake.exports.miso_engine_web_v1_boot_options_ptr = () => 0; }],
      [5, (fake) => {
        fake.calls.bootResult = 5;
        fake.exports.miso_engine_web_v1_document_ptr = () => 0;
      }],
      [1, (fake) => {
        fake.exports.miso_engine_web_v1_boot = () => 0;
        fake.exports.miso_engine_web_v1_boot_result = () => 1;
      }],
    ]) {
      const fake = createFakeExports(64);
      mutate(fake);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 0, "a refused boot publishes no handle to dispose");
      assert.equal(processor.port.posts[0].message.result, result);
      assert.equal(processor.disposed, true);
    }

    const failureMutations = [
      (fake) => { fake.exports.miso_engine_web_v1_buffer_capacity = () => 0; },
      (fake) => { fake.exports.miso_engine_web_v1_status_ptr = () => 65500; },
      (fake) => { new DataView(fake.exports.memory.buffer, 17000).setUint32(20, 1, true); },
    ];
    for (const mutate of failureMutations) {
      const fake = createFakeExports(64);
      mutate(fake);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "post-handle construction failure disposes exactly once");
      assert.equal(processor.disposed, true);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
      assert.equal(processor.port.posts.length, 1);
      assert.equal(processor.port.posts[0].message.tag, "miso.error.v1");
    }

    {
      const fake = createFakeExports(64, 0);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "swapped backend artifact is transactionally disposed");
      assert.equal(processor.port.posts[0].message.result, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    for (const offset of [16384 + 16, 17000 + 16]) {
      const fake = createFakeExports(64);
      new DataView(fake.exports.memory.buffer).setUint32(offset, 0, true);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "each Rust backend row is independently authoritative");
      assert.equal(processor.port.posts[0].message.result, 1);
    }

    {
      const fake = createFakeExports(64);
      throwReadyPost = true;
      const processor = construct(fake);
      throwReadyPost = false;
      assert.equal(fake.calls.dispose, 1, "ready publication failure disposes exactly once");
      assert.equal(processor.disposed, true);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [64]);
      assert(left.every((sample) => sample === 0.25));
      assert(right.every((sample) => sample === -0.25));
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const status = processor.port.posts.at(-1).message;
      assert.equal(status.memoryBytes, 196608);
      assert.equal(status.nextAbsoluteSample, 64n);
      assert.equal(
        status.singleModeControlAllocations,
        5,
        "the status carries the control allocation count",
      );
    }

    // Issue #1381a D6, gate 4's worklet half: the control handler runs one service step after
    // every message it handles, the main realm's `miso.service.v1` tick included, and never in
    // `process()`. The tick is no request: it has no request ID, takes no place in the request
    // sequence and gets no reply. A step commits and acknowledges nothing, so a good one posts
    // nothing.
    {
      const { processor, fake } = makeProcessor();
      const posts = processor.port.posts.length;
      assert.equal(fake.calls.service.length, 0, "construction runs no service step");
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      assert.equal(fake.calls.service.length, 0, "process() never services");
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.deepEqual(fake.calls.service, [1], "the tick services once, on the host's handle");
      assert.equal(processor.port.posts.length, posts, "the tick gets no reply");
      assert.equal(processor.lastRequestId, 0, "the tick takes no request ID");
      processor.port.onmessage({ data: { tag: "miso.status.v1", requestId: 1 } });
      assert.equal(fake.calls.service.length, 2, "a request is followed by one service step");
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.status.v1");
      // A tick with an extra field is not the tick: it is an unknown message, which is sticky.
      processor.port.onmessage({ data: { tag: "miso.service.v1", requestId: 2 } });
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
      assert.equal(processor.ready, false);
      const stepsAtFailure = fake.calls.service.length;
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(
        fake.calls.service.length,
        stepsAtFailure,
        "a failed worklet runs no further service step",
      );
    }
    {
      // An epoch failure of the service step is sticky, with the export's result.
      const { processor, fake } = makeProcessor();
      fake.calls.serviceResult = 255;
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      const failure = processor.port.posts.at(-1).message;
      assert.equal(failure.tag, "miso.error.v1");
      assert.equal(failure.result, 255);
      assert.equal(processor.ready, false);
      const left = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, new Float32Array(64)]]), true);
      assert(left.every((sample) => sample === 0), "a failed worklet renders silence");
    }
    {
      // A dispose stops the service steps: the handle is gone.
      const { processor, fake } = makeProcessor();
      processor.port.onmessage({ data: { tag: "miso.dispose.v1", requestId: 1 } });
      assert.equal(fake.calls.dispose, 1);
      const steps = fake.calls.service.length;
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(fake.calls.service.length, steps, "no service step after dispose");
    }

    // Issue #1294a (D3): after each service step the handler reads the watermark and posts
    // `miso.watermark.v1` only when its `(revision, first_sample)` pair changed, on a tick and on a
    // request alike, never from `process()`; the boot watermark is the baseline, and a
    // `BACKPRESSURE` read skips one message without failing the worklet.
    {
      const { processor, fake } = makeProcessor();
      const watermarks = () => processor.port.posts
        .map((post) => post.message).filter((message) => message.tag === "miso.watermark.v1");
      assert.equal(fake.calls.watermarkReads, 1, "boot reads the baseline watermark");
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(fake.calls.watermarkReads, 2, "a tick reads the watermark");
      assert.equal(watermarks().length, 0, "an unchanged watermark is not posted");
      Object.assign(fake.calls.watermark, {
        revision: 5n, sample: 128n, outcome: 3n, exactCount: 4n, transitionFallbackCount: 1n,
      });
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      assert.equal(fake.calls.watermarkReads, 2, "process() never reads the watermark");
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.deepEqual(watermarks(), [{
        tag: "miso.watermark.v1", revision: 5n, sample: 128n, outcome: 3n, exactCount: 4n,
        transitionFallbackCount: 1n, supersededCount: 0n,
      }], "an advance is posted once, with the record's words");
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(watermarks().length, 1, "the same pair is not posted again");
      fake.calls.watermarkResult = 6;
      Object.assign(fake.calls.watermark, { revision: 6n, sample: 192n, exactCount: 5n });
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(watermarks().length, 1, "a BACKPRESSURE read posts nothing");
      assert.equal(processor.ready, true, "a BACKPRESSURE read is not sticky");
      fake.calls.watermarkResult = 0;
      processor.port.onmessage({ data: { tag: "miso.status.v1", requestId: 1 } });
      assert.deepEqual(
        processor.port.posts.slice(-2).map((post) => post.message.tag),
        ["miso.status.v1", "miso.watermark.v1"],
        "a request is followed by the watermark its service step advanced",
      );
      assert.equal(watermarks().at(-1).revision, 6n, "the next read posts the newest record");
      assert.equal(watermarks().at(-1).sample, 192n);
      // Two advances between two reads: the first a transition fallback, the second exact. The
      // record's flags are the second's alone; the post's outcome carries both.
      Object.assign(fake.calls.watermark, {
        revision: 8n, sample: 320n, outcome: 1n, exactCount: 6n, transitionFallbackCount: 2n,
      });
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(watermarks().at(-1).outcome, 3n, "a count that rose since the last post names its flag");
    }
    for (const [what, mutate, result] of [
      ["a refused read", (calls) => { calls.watermarkResult = 5; }, 5],
      ["a record of the wrong size", (calls) => { calls.watermarkBytes = 88; }, 255],
    ]) {
      const { processor, fake } = makeProcessor();
      mutate(fake.calls);
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(processor.ready, false, `${what} is sticky`);
      const failure = processor.port.posts.at(-1).message;
      assert.equal(failure.tag, "miso.error.v1", `${what} posts the error`);
      assert.equal(failure.result, result, `${what} carries its result`);
    }

    // Issue #1562a D1, gate 7's worklet half: a `miso.control.v1` message stages its frame,
    // runs the control command in the message handler and replies with the response frame
    // unchanged and the committed revision. The worklet decodes nothing in the frame.
    {
      const { processor, fake } = makeProcessor();
      const frame = new Uint8Array([1, 2, 3, 4, 5]);
      processor.port.onmessage({ data: { tag: "miso.control.v1", requestId: 1, frame } });
      assert.equal(fake.calls.control.length, 1);
      assert.deepEqual([...fake.calls.control[0].frame], [1, 2, 3, 4, 5], "the frame is staged as is");
      const reply = processor.port.posts.at(-1).message;
      assert.equal(reply.tag, "miso.control.v1");
      assert.equal(reply.requestId, 1);
      assert.equal(reply.result, 0);
      assert.deepEqual([...reply.response], [9, 8, 7], "the response frame comes back unchanged");
      assert.equal(reply.revision, 4n, "the reply carries the committed revision");
      assert.equal(
        fake.calls.service.length, 1,
        "the control message is followed by one service step, and process() runs none",
      );
      // A refusal carries its result and diagnostic bytes, and leaves the worklet ready.
      fake.calls.controlResult = 10;
      fake.calls.controlResponse = Uint8Array.from(
        "web.apply.shape_unavailable\t$\n", (character) => character.charCodeAt(0),
      );
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 2, frame: new Uint8Array([7]) },
      });
      const refused = processor.port.posts.at(-1).message;
      assert.equal(refused.tag, "miso.control.v1");
      assert.equal(refused.result, 10);
      assert.equal(
        String.fromCharCode(...refused.response),
        "web.apply.shape_unavailable\t$\n",
      );
      assert.equal(processor.ready, true, "a typed refusal is not sticky");
      // A frame the staging buffer refuses is a typed result with no response, not sticky.
      fake.calls.controlRequestRefusals = 1;
      // The previous command's response is still in the module; a refused staging has none.
      fake.calls.controlResponse = new Uint8Array([9, 9, 9]);
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 3, frame: new Uint8Array([7]) },
      });
      const oversize = processor.port.posts.at(-1).message;
      assert.equal(oversize.tag, "miso.control.v1");
      assert.equal(oversize.result, 1);
      assert.equal(oversize.response.byteLength, 0);
      assert.equal(fake.calls.control.length, 2, "a refused staging runs no command");
      assert.equal(processor.ready, true);
    }
    {
      // A malformed control message is a protocol error, like a malformed command, and sticky.
      for (const bad of [
        { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array(0) },
        { tag: "miso.control.v1", requestId: 1, frame: [1, 2] },
        { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1]), extra: 1 },
        { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array(new ArrayBuffer(8), 2, 4) },
      ]) {
        const { processor, fake } = makeProcessor();
        processor.port.onmessage({ data: bad });
        assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
        assert.equal(fake.calls.control.length, 0, "a malformed message reaches no export");
        assert.equal(processor.ready, false);
      }
    }
    {
      // Issue #1697: a `document` beside the frame is staged through `replace_document_ptr` and
      // reaches the command with the frame; a document the staging refuses answers a typed
      // refusal with no response (never the previous command's) and runs no command; a frame the
      // staging refuses stages no document, so none is left for a later command.
      const { processor, fake } = makeProcessor();
      const document = new Uint8Array([123, 34, 97, 34, 125]);
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1, 2]), document },
      });
      assert.deepEqual(fake.calls.replaceDocument, [5]);
      assert.equal(fake.calls.control.length, 1);
      assert.deepEqual([...fake.calls.control[0].frame], [1, 2]);
      assert.deepEqual([...fake.calls.control[0].document], [...document], "the document is staged");
      assert.deepEqual([...processor.port.posts.at(-1).message.response], [9, 8, 7]);
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 2, frame: new Uint8Array([3]) },
      });
      assert.equal(fake.calls.control[1].document, null, "a frame alone carries no document");
      fake.calls.replaceDocumentRefusals = 1;
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 3, frame: new Uint8Array([4]), document },
      });
      const refused = processor.port.posts.at(-1).message;
      assert.equal(refused.tag, "miso.control.v1");
      assert.equal(refused.requestId, 3);
      assert.equal(refused.result, 1);
      assert.equal(refused.response.byteLength, 0, "a refused document has no response");
      assert.equal(fake.calls.control.length, 2, "a refused document runs no command");
      fake.calls.controlRequestRefusals = 1;
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 4, frame: new Uint8Array([5]), document },
      });
      assert.equal(processor.port.posts.at(-1).message.result, 1);
      assert.deepEqual(fake.calls.replaceDocument, [5, 5], "a refused frame stages no document");
      assert.equal(fake.calls.control.length, 2);
      assert.equal(processor.ready, true, "a refused staging is not sticky");
      for (const bad of [
        new Uint8Array(0),
        [1, 2],
        new Uint8Array(new ArrayBuffer(8), 2, 4),
        undefined,
      ]) {
        const { processor, fake } = makeProcessor();
        processor.port.onmessage({
          data: { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1]), document: bad },
        });
        assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
        assert.equal(fake.calls.replaceDocument.length, 0, "a malformed document reaches no export");
        assert.equal(fake.calls.control.length, 0, "a malformed document runs no command");
        assert.equal(processor.ready, false);
      }
    }
    {
      // Root ruling A (#1332 D5, "growth is not a fault"): the module's memory growing under a
      // command gives it a new buffer object, and the handler re-cuts every view before it ends. The
      // reply is posted whole, the worklet stays ready, a later request is served, and the next
      // `process()` renders the engine's output through the new views.
      const { processor, fake } = makeProcessor();
      const before = fake.exports.memory.buffer;
      fake.calls.controlGrowsMemory = true;
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1]) },
      });
      assert.equal(fake.calls.growths, 1);
      assert.notEqual(fake.exports.memory.buffer, before, "the fake really grew its memory");
      const posts = processor.port.posts.map((post) => post.message);
      const reply = posts.at(-1);
      assert.equal(reply.tag, "miso.control.v1");
      assert.equal(reply.requestId, 1);
      assert.deepEqual([...reply.response], [9, 8, 7], "the reply was read from the new buffer");
      assert(!posts.some((post) => post.tag === "miso.error.v1"), "a growth posts no error");
      assert.equal(processor.ready, true);
      assert.equal(processor.stickyResult, 0);
      // Every view the instance holds is cut against the new buffer, the feed's too.
      for (const spec of processor.viewSpecs) {
        assert.equal(
          processor[spec.field].buffer,
          fake.exports.memory.buffer,
          `${spec.field} is re-bound to the grown memory`,
        );
      }
      // The meter set's views are the plan's, not registered by name (#1290d1): the active set
      // is re-cut with the rest.
      for (const field of [
        "header", "frame", "trackPeaks", "submixPeaks", "masterPeaks", "trackGain", "submixGain",
      ]) {
        assert.equal(
          processor.meterSet[field].buffer,
          fake.exports.memory.buffer,
          `the meter set's ${field} is re-bound to the grown memory`,
        );
      }
      assert(processor.viewSpecs.length >= 5, "every persistent view is registered");
      const left = new Float32Array(64).fill(1);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [64], "the next quantum renders");
      assert(left.every((sample) => sample === 0.25), "non-silent output through the new views");
      assert(right.every((sample) => sample === -0.25));
      // A later request is served: another control call (growing again) and the status.
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 2, frame: new Uint8Array([2]) },
      });
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.control.v1");
      assert.equal(processor.port.posts.at(-1).message.requestId, 2);
      assert.equal(processor.ready, true);
      processor.port.onmessage({ data: { tag: "miso.status.v1", requestId: 3 } });
      const status = processor.port.posts.map((post) => post.message)
        .filter((message) => message.tag === "miso.status.v1").at(-1);
      assert.equal(status.result, 0);
      assert.equal(status.memoryBytes, 196608 * 4, "the status reports the grown memory");
      assert.equal(status.renderedQuanta, 1n, "the status reads the grown memory");
      assert.equal(processor.process([], [[left.fill(1), right.fill(1)]]), true);
      assert(left.every((sample) => sample === 0.25));
      assert.deepEqual(fake.calls.render, [64, 64]);
    }
    {
      // A typed refusal keeps its behaviour when the same call also grew the memory: replied
      // whole, not sticky. Only the growth stopped being a fault.
      const { processor, fake } = makeProcessor();
      fake.calls.controlGrowsMemory = true;
      fake.calls.controlResult = 10;
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1]) },
      });
      assert.equal(processor.port.posts.at(-1).message.result, 10, "the refusal is replied whole");
      assert.equal(processor.ready, true, "a typed refusal is not sticky, grown or not");
    }

    // Issue #1570b D4, gate 4's worklet half: after a `control` message and after a service tick
    // the control half drains the reliable lane until `control_event` returns 0 bytes, then the
    // lossy lane, and posts each frame as a `control-event` message in that order. It stops at 16
    // calls per lane, and `process()` never calls `control_event`.
    {
      const { processor, fake } = makeProcessor();
      const events = (lane) => processor.port.posts
        .map((post) => post.message)
        .filter((message) => message.tag === "miso.control-event.v1" && message.lane === lane);
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      assert.deepEqual(fake.calls.eventCalls, [], "process() never dequeues an event");
      // A service tick drains: two reliable frames, then one lossy frame, in dequeue order.
      fake.calls.events[0].push(new Uint8Array([1, 1]), new Uint8Array([2, 2, 2]));
      fake.calls.events[1].push(new Uint8Array([9]));
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.deepEqual(
        fake.calls.eventCalls.map((call) => [call.handle, call.lane]),
        [[1, 0], [1, 0], [1, 0], [1, 1], [1, 1]],
        "the reliable lane is read until empty, then the lossy lane until empty",
      );
      const posted = processor.port.posts.map((post) => post.message)
        .filter((message) => message.tag === "miso.control-event.v1");
      assert.deepEqual(
        posted.map((message) => [message.lane, [...message.frame]]),
        [[0, [1, 1]], [0, [2, 2, 2]], [1, [9]]],
        "each frame is posted whole, reliable lane first, in dequeue order",
      );
      assert(posted.every((message) => Object.keys(message).length === 3));
      assert(
        processor.port.posts.filter((post) => post.message.tag === "miso.control-event.v1")
          .every((post) => post.transferCount === 1),
        "each frame's buffer is transferred",
      );
      // A control message drains after its own reply, never before it.
      fake.calls.events[0].push(new Uint8Array([7]));
      const before = processor.port.posts.length;
      processor.port.onmessage({
        data: { tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([1]) },
      });
      const tags = processor.port.posts.slice(before).map((post) => post.message.tag);
      assert.deepEqual(tags, ["miso.control.v1", "miso.control-event.v1"]);
      assert.equal(events(0).length, 3);
      assert.equal(events(1).length, 1);
      assert.equal(processor.ready, true);
    }
    {
      // At most 16 calls per lane per drain: the rest waits for the next drain.
      const { processor, fake } = makeProcessor();
      for (let index = 0; index < 20; index += 1) fake.calls.events[0].push(new Uint8Array([index]));
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.deepEqual(
        fake.calls.eventCalls.filter((call) => call.lane === 0).length,
        16,
        "a drain makes 16 reliable calls",
      );
      assert.equal(
        fake.calls.eventCalls.filter((call) => call.lane === 1).length, 1,
        "the lossy lane is read after the bounded reliable lane",
      );
      assert.equal(fake.calls.events[0].length, 4);
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(fake.calls.events[0].length, 0, "the next drain takes the rest");
      const frames = processor.port.posts.map((post) => post.message)
        .filter((message) => message.tag === "miso.control-event.v1");
      assert.deepEqual(frames.map((message) => message.frame[0]), [...Array(20).keys()]);
    }
    {
      // A refused dequeue stops the drain, takes the error path once and is not retried.
      const { processor, fake } = makeProcessor();
      fake.calls.events[0].push(new Uint8Array([5]));
      fake.calls.eventResult = 6;
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(fake.calls.eventCalls.length, 1, "a refusal ends the drain");
      const failure = processor.port.posts.at(-1).message;
      assert.equal(failure.tag, "miso.error.v1");
      assert.equal(failure.result, 6);
      assert.equal(processor.ready, false);
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(fake.calls.eventCalls.length, 1, "a failed worklet drains no more");
    }
    {
      // A dispose drains nothing: the handle is gone.
      const { processor, fake } = makeProcessor();
      processor.port.onmessage({ data: { tag: "miso.dispose.v1", requestId: 1 } });
      assert.deepEqual(fake.calls.eventCalls, []);
    }

    // #1399 gate 3: a status block whose word at 48 is nonzero is a valid block, and the reply
    // carries the word; #1294b: likewise the blocking-rebuild word at 28.
    {
      const fake = createFakeExports(64);
      new DataView(fake.exports.memory.buffer, 16384).setBigUint64(48, 0x0102_0304_0506n, true);
      new DataView(fake.exports.memory.buffer, 16384).setUint32(28, 3, true);
      const processor = construct(fake);
      assert.equal(processor.port.posts[0].message.tag, "miso.ready.v1");
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const status = processor.port.posts.at(-1).message;
      assert.equal(status.liveValuesSuperseded, 0x0102_0304_0506n);
      assert.equal(status.blockingRebuilds, 3, "the status reply carries the word at 28");
      new DataView(fake.exports.memory.buffer, 16384).setBigUint64(48, (1n << 64n) - 1n, true);
      processor.receive({ tag: "miso.status.v1", requestId: 2 });
      assert.equal(
        processor.port.posts.at(-1).message.liveValuesSuperseded,
        (1n << 64n) - 1n,
        "the word is read whole",
      );
    }
    // #1349 gate 5: the worklet's status block holds the engine's watermark `(r, s, EXACT)` at
    // 56, 64 and 72 from boot on. Construction accepts it and the reply carries the three words
    // whole, so a reader that still demanded zero there would fail every boot.
    {
      const fake = createFakeExports(64);
      const block = new DataView(fake.exports.memory.buffer, 16384);
      block.setBigUint64(56, 0x0102_0304n, true);
      block.setBigUint64(64, 0x0a0b_0c0dn, true);
      block.setBigUint64(72, 1n, true);
      const processor = construct(fake);
      assert.equal(processor.port.posts[0].message.tag, "miso.ready.v1");
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const status = processor.port.posts.at(-1).message;
      assert.equal(status.appliedRevision, 0x0102_0304n);
      assert.equal(status.appliedSample, 0x0a0b_0c0dn);
      assert.equal(status.appliedOutcome, 1n);
      block.setBigUint64(56, (1n << 64n) - 1n, true);
      block.setBigUint64(72, 7n, true);
      processor.receive({ tag: "miso.status.v1", requestId: 2 });
      const wide = processor.port.posts.at(-1).message;
      assert.equal(wide.appliedRevision, (1n << 64n) - 1n, "the revision is read whole");
      assert.equal(wide.appliedOutcome, 7n, "the flags word is copied, not interpreted");
    }

    {
      const { processor, fake } = makeProcessor();
      processor.ready = false;
      const left = new Float32Array(64).fill(-0);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, []);
      assert(left.every((sample) => Object.is(sample, 0)), "pre-ready left is positive zero");
      assert(right.every((sample) => Object.is(sample, 0)), "pre-ready right is positive zero");
    }

    {
      const { processor, fake } = makeProcessor();
      const wrong = new Float32Array(32).fill(1);
      assert.equal(processor.process([], [[wrong]]), true);
      assert.deepEqual(fake.calls.render, [0]);
      assert(wrong.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert.equal(processor.stickyResult, 9);
    }

    {
      // Issue #1332 S3: a growth is no fault for `process()` either. On the shared memory a view
      // over the older buffer object stays valid for its bytes, so a quantum after a growth with
      // no handler since renders over the views the instance holds; `process()` never re-binds,
      // and the next handler re-cuts the views.
      const { processor, fake } = makeProcessor();
      const before = processor.outputLeft.buffer;
      fake.growMemory();
      assert.notEqual(fake.exports.memory.buffer, before, "the fake really grew its memory");
      const left = new Float32Array(64).fill(1);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [64], "the quantum after a growth renders");
      assert(left.every((sample) => sample === 0.25), "through the views over the older buffer");
      assert(right.every((sample) => sample === -0.25));
      assert.equal(processor.stickyResult, 0);
      assert.equal(processor.outputLeft.buffer, before, "process() did not re-bind");
      processor.port.onmessage({ data: { tag: "miso.status.v1", requestId: 1 } });
      const status = processor.port.posts.map((post) => post.message)
        .filter((message) => message.tag === "miso.status.v1").at(-1);
      assert.equal(status.memoryBytes, 393216);
      assert.equal(processor.outputLeft.buffer, fake.exports.memory.buffer, "the handler re-cut the views");
    }

    {
      const { processor, fake } = makeProcessor();
      fake.calls.sourceResult = 6;
      const storage = new ArrayBuffer(32);
      const incoming = structuredClone({
        tag: "miso.source.v1", requestId: 1, sourceId: nonAsciiSourceId, generation: 1n,
        startFrame: 0n, sampleRateHz: 48000,
        planes: [new Float32Array(storage, 0, 2), new Float32Array(storage, 16, 2)],
        frames: 2, endOfRegion: false,
      }, { transfer: [storage] });
      processor.receive(incoming);
      const response = processor.port.posts.at(-1);
      assert.equal(response.transferCount, 1);
      assert.equal(response.message.tag, "miso.ack.v1");
      assert.equal(response.message.result, 6);
      assert.equal(response.message.planes[0].byteOffset, 0);
      assert.equal(response.message.planes[1].byteOffset, 16);
      assert.equal(response.message.planes[0].buffer, response.message.planes[1].buffer);
      assert.deepEqual(
        fake.calls.sourceIdBytes[0],
        nonAsciiSourceIdUtf8,
        "non-ASCII submit IDs are byte-identical to UTF-8 without TextEncoder in the worklet",
      );
      processor.receive({
        tag: "miso.seek.v1", requestId: 2, sourceId: nonAsciiSourceId,
        generation: 2n, sourceFrame: 12n,
      });
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 2, result: 0,
      });
      assert.deepEqual(
        fake.calls.seekIdBytes[0],
        nonAsciiSourceIdUtf8,
        "non-ASCII seek IDs are byte-identical to UTF-8 without TextEncoder in the worklet",
      );
      processor.receive({ tag: "miso.dispose.v1", requestId: 3 });
      assert.equal(fake.calls.dispose, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      // Issue #1644 gate 2, the worklet half: a plain session seek calls `session_seek` once and an
      // anchored one `session_seek_at` once, never the declaration; an OK records the generation
      // for the feed drain before the reply, a refusal is the reply's result, records nothing and
      // is not sticky, and a malformed message calls no export.
      const { processor, fake } = makeProcessor();
      const handle = processor.handle;
      const seek = (requestId, generation, sourceFrame, anchorSample) => {
        processor.receive({
          tag: "miso.session-seek.v1", requestId, generation, sourceFrame, anchorSample,
        });
        return processor.port.posts.at(-1).message;
      };
      assert.deepEqual(seek(1, 2n, 480n, -1n), {
        tag: "miso.session-seek.v1", requestId: 1, result: 0, generation: 2n,
      });
      assert.equal(processor.lastSessionSeekGeneration, 2n);
      // The feed drain reads a refused seek's code through this boot-time view (#1644 D2).
      assert.deepEqual(
        [processor.diagnosticView.byteOffset, processor.diagnosticView.length,
          processor.diagnosticView.buffer === processor.memoryBuffer],
        [150000, 256, true],
      );
      assert.deepEqual(seek(2, 3n, 0n, 128n).result, 0);
      fake.calls.sessionSeekResult = 1;
      assert.deepEqual(seek(3, 4n, 0n, -1n), {
        tag: "miso.session-seek.v1", requestId: 3, result: 1, generation: 4n,
      });
      assert.equal(processor.lastSessionSeekGeneration, 3n, "a refused seek records nothing");
      assert.deepEqual(fake.calls.sessionSeek, [
        ["session_seek", handle, 2n, 480n],
        ["session_seek_at", handle, 3n, 0n, 128n],
        ["session_seek", handle, 4n, 0n],
      ]);
      assert.equal(processor.stickyResult, 0, "a refused session seek is not sticky");
      seek(4, 0n, 0n, -1n);
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
      assert.equal(fake.calls.sessionSeek.length, 3, "a malformed session seek calls no export");
    }

    {
      // Issue #1297a, the worklet half: an anchored source seek calls `source_seek_at` once in the
      // handler with the staged ID, and `process()` adds no call; a seek report copies the
      // record's words; a refusal is the reply's result (a null report) and is not sticky; a
      // malformed message calls no export; a record with the wrong size word is sticky.
      const { processor, fake } = makeProcessor();
      const handle = processor.handle;
      const last = () => processor.port.posts.at(-1).message;
      processor.receive({
        tag: "miso.seek-at.v1", requestId: 1, sourceId: "drums", generation: 2n,
        sourceFrame: 480n, anchorSample: 128n,
      });
      assert.deepEqual(last(), { tag: "miso.ack.v1", requestId: 1, result: 0 });
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      fake.calls.seekAtResult = 1;
      processor.receive({
        tag: "miso.seek-at.v1", requestId: 2, sourceId: "drums", generation: 3n,
        sourceFrame: 0n, anchorSample: 100n,
      });
      assert.deepEqual(last(), { tag: "miso.ack.v1", requestId: 2, result: 1 });
      assert.deepEqual(fake.calls.seekAt, [
        [handle, 5, 2n, 480n, 128n, "drums"],
        [handle, 5, 3n, 0n, 100n, "drums"],
      ]);
      fake.calls.seekReportGrowsMemory = true;
      processor.receive({ tag: "miso.seek-report.v1", requestId: 3, sourceId: "$timeline" });
      fake.calls.seekReportGrowsMemory = false;
      assert.deepEqual(last(), {
        tag: "miso.seek-report.v1", requestId: 3, result: 0,
        report: {
          generation: 5n, firstSample: 1024n, sourceFrame: 48000n,
          cumulativeUnderrunFrames: 7n, cumulativeHeldFrames: 9n,
        },
      });
      fake.calls.seekReportResult = 6;
      processor.receive({ tag: "miso.seek-report.v1", requestId: 4, sourceId: "drums" });
      assert.deepEqual(last(), { tag: "miso.seek-report.v1", requestId: 4, result: 6, report: null });
      assert.deepEqual(fake.calls.seekReport, [[handle, 9, "$timeline"], [handle, 5, "drums"]]);
      assert.equal(processor.stickyResult, 0, "a refused seek-at or seek report is not sticky");
      fake.calls.seekReportResult = 0;
      fake.calls.seekReportBytes = 48;
      processor.receive({ tag: "miso.seek-report.v1", requestId: 5, sourceId: "drums" });
      assert.deepEqual(last(), { tag: "miso.error.v1", requestId: 5, result: 255 });
      for (const message of [
        { tag: "miso.seek-at.v1", requestId: 1, sourceId: "drums", generation: 2n,
          sourceFrame: 0n, anchorSample: -1n },
        { tag: "miso.seek-at.v1", requestId: 1, sourceId: "drums", generation: 0n,
          sourceFrame: 0n, anchorSample: 0n },
        { tag: "miso.seek-report.v1", requestId: 1, sourceId: 7 },
      ]) {
        const { processor: malformed, fake: untouched } = makeProcessor();
        malformed.receive(message);
        assert.equal(malformed.port.posts.at(-1).message.tag, "miso.error.v1");
        assert.deepEqual([untouched.calls.seekAt, untouched.calls.seekReport], [[], []],
          "a malformed seek-at or seek report calls no export");
      }
    }

    {
      // #106 F5: `panic = abort` means a Rust panic reaches JavaScript as a throw from the render
      // export. `process()` is the only place that can contain it. Red mutation: remove the `try`
      // around `miso_engine_web_v1_render` in `process()` -> this `process()` call throws.
      const { processor } = makeProcessor();
      processor.exports.miso_engine_web_v1_render = () => {
        throw new Error("synthetic wasm trap from the render export");
      };
      const left = new Float32Array(64).fill(-0);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true, "a trap keeps the node alive");
      assert(left.every((sample) => Object.is(sample, 0)), "trapped left is positive zero");
      assert(right.every((sample) => Object.is(sample, 0)), "trapped right is positive zero");
      assert.equal(processor.stickyResult, 255);
      assert.equal(processor.ready, false);
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const settled = processor.port.posts.at(-1).message;
      assert.equal(settled.tag, "miso.error.v1");
      assert.equal(settled.result, 255, "the trap is sticky for every later request");
    }

    {
      const { processor } = makeProcessor();
      const storage = new ArrayBuffer(16);
      const incoming = structuredClone({
        tag: "miso.source.v1", requestId: 1, sourceId: "source", generation: 1n,
        startFrame: 0n, sampleRateHz: 48000, planes: [new Float32Array(storage)],
        frames: 3, endOfRegion: false,
      }, { transfer: [storage] });
      processor.receive(incoming);
      const response = processor.port.posts.at(-1);
      assert.equal(response.message.tag, "miso.error.v1");
      assert.equal(response.message.result, 1);
      assert.equal(response.transferCount, 1);
      assert.equal(response.message.planes[0].byteLength, 16);
    }

    {
      // Issue #137 D2/D3: both leases start released, and a released lease costs the render
      // callback one boolean test and no message.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      fake.calls.meterWindows = 3;
      const before = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, before, "no lease, no frame");
      assert.equal(fake.calls.meterLease.length, 0, "no lease, no engine call");

      processor.receive({ tag: "miso.meters.v1", requestId: 1, enabled: true });
      assert.deepEqual(fake.calls.meterLease, [1]);
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 1, result: 0,
      });
      assert.equal(processor.process([], [[left, right]]), true);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1");
      assert.equal(frame.sequence, 1);
      assert.equal(frame.generation, 1n);
      assert.equal(frame.validity, 0xb);
      assert.equal(frame.lossCount, 0);
      assert.equal(frame.windows, 1);
      assert.equal(frame.trackCount, 2);
      // Issue #1209 D4: `peaks` keeps its `2T + 2` shape, the master pair read from after the bus.
      assert.deepEqual([...frame.peaks], [0.5, 0.5, 0.5, 0.5, 0.625, 0.625]);
      // Issue #143: the gain-reduction section is its own view, its own length, and its own value.
      assert.equal(frame.trackGrDb.length, 2);
      assert(frame.trackGrDb.every((value) => value === 6.5));
      assert.equal(frame.masterGrDb, 7.5, "the header says the master reading is present");
      // Issue #1209 D4: the bus section, appended.
      assert.equal(frame.submixCount, 1);
      assert.deepEqual([...frame.submixPeaks], [0.75, 0.75]);
      assert.deepEqual([...frame.submixGrDb], [2.25]);
      assert.equal(frame.firstSample, 512n);
      assert.equal(frame.endSample, 768n);
      assert.equal(frame.structureEpoch, 1, "the plan's list epoch rides the frame");

      // A nonzero poll is still withheld unless both complete peak-validity bits are present.
      // This exercises the real worklet guard against a malformed/future Rust publication.
      fake.meterHeader.setBigUint64(56, 0x1n, true);
      const invalidPosts = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, invalidPosts, "invalid peak metadata is not posted");

      // Loss metadata crosses the BigInt header as bounded numeric message fields.
      fake.meterHeader.setBigUint64(56, (2n << 32n) | 0xfn, true);
      assert.equal(processor.process([], [[left, right]]), true);
      const lossyFrame = processor.port.posts.at(-1).message;
      assert.equal(lossyFrame.tag, "miso.meter.v1");
      assert.equal(lossyFrame.sequence, 2);
      assert.equal(lossyFrame.generation, 1n);
      assert.equal(lossyFrame.validity, 0xf);
      assert.equal(lossyFrame.lossCount, 2);
      // The epoch is read as two words: one past 2^32 still arrives whole, with no BigInt.
      fake.meterHeader.setBigUint64(80, (3n << 32n) | 7n, true);
      fake.calls.meterWindows = 1;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.at(-1).message.structureEpoch, 3 * 4294967296 + 7);

      // Releasing the lease stops the frames immediately.
      processor.receive({ tag: "miso.meters.v1", requestId: 2, enabled: false });
      assert.deepEqual(fake.calls.meterLease, [1, 0]);
      const quiet = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, quiet, "a released lease posts nothing");
    }

    // #1209 MINOR-2: the frame decode with no submix (every session before K2, and today's
    // production case) and with two. Red if the worklet refuses, or mis-slices, a frame whose
    // submix section is empty: the default fake has one submix, so nothing else runs `S = 0`.
    for (const submixIds of [[], ["aa-bus", "zz-bus"]]) {
      const { processor, fake } = makeProcessor(submixIds);
      const S = submixIds.length;
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      processor.receive({ tag: "miso.meters.v1", requestId: 1, enabled: true });
      assert.equal(processor.port.posts.at(-1).message.result, 0);
      fake.calls.meterWindows = 1;
      assert.equal(processor.process([], [[left, right]]), true);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1", `S = ${S}: a frame is posted`);
      assert.equal(frame.trackCount, 2);
      assert.deepEqual([...frame.peaks], [0.5, 0.5, 0.5, 0.5, 0.625, 0.625], `S = ${S}: tracks, master`);
      assert.deepEqual([...frame.trackGrDb], [6.5, 6.5], `S = ${S}: track gain reduction`);
      assert.equal(frame.masterGrDb, 7.5, `S = ${S}: the master reading follows every strip's`);
      assert.equal(frame.submixCount, S);
      assert.deepEqual([...frame.submixPeaks], Array(2 * S).fill(0.75), `S = ${S}: bus peaks`);
      assert.deepEqual([...frame.submixGrDb], Array(S).fill(2.25), `S = ${S}: bus gain reduction`);
    }

    {
      // Issue #1290d1: the meter frame travels with the plan. A command that commits a
      // replacement plan names the plan's header in the active one, and the handler cuts the
      // views over that header and its frame (control side, allocating). The block that adopts the
      // plan marks the header it left retired, and `process()` switches to the views by reference:
      // it cuts none, so the very objects the handler made are the ones it posts from.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      processor.receive({ tag: "miso.meters.v1", requestId: 1, enabled: true });
      fake.calls.meterWindows = 1;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.at(-1).message.trackCount, 2);
      assert.equal(processor.pendingMeterSet, null, "nothing waits");

      fake.commitMeterSuccessor();
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      const pending = processor.pendingMeterSet;
      assert.notEqual(pending, null, "the handler cut the successor's views");
      assert.equal(pending.trackCount, 3);
      assert.equal(pending.frame.length, (3 + 1) * 3 + 3);
      const active = processor.meterSet;
      assert.equal(active.trackCount, 2, "the plan that renders keeps its views");

      // A block that does not adopt keeps the active set, whatever waits.
      fake.calls.meterWindows = 1;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.meterSet, active);
      assert.equal(processor.port.posts.at(-1).message.trackCount, 2);

      // The adopting block: the frame posted is the successor's, sized for its strips.
      fake.calls.adoptOnNextRender = true;
      fake.calls.meterWindows = 1;
      const posts = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, posts + 1, "the first block after adoption posts");
      assert.equal(processor.meterSet, pending, "switched by reference to the set cut in advance");
      assert.equal(processor.pendingMeterSet, null);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.trackCount, 3);
      assert.deepEqual([...frame.peaks], [...Array(6).fill(0.5), 0.625, 0.625]);
      assert.deepEqual([...frame.trackGrDb], [6.5, 6.5, 6.5]);
      assert.deepEqual([...frame.submixPeaks], [0.75, 0.75]);
      assert.deepEqual([...frame.submixGrDb], [2.25]);
      assert.equal(frame.masterGrDb, 7.5);
      assert.equal(frame.sequence, 3, "the page's sequence continues across the swap");
      assert.equal(frame.structureEpoch, 2, "the adopted plan names its own list");

      // A memory growth re-cuts both sets, so a later swap still reads live memory.
      fake.growMemory();
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      assert.equal(processor.meterSet.frame.buffer, fake.exports.memory.buffer);
      assert.equal(processor.meterSet.header.buffer, fake.exports.memory.buffer);
    }

    {
      // The lease released across a swap: nothing switched the views, and the next handler finds
      // the host's header is not the one the page holds and cuts the new plan's.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      fake.commitMeterSuccessor();
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      fake.calls.adoptOnNextRender = true;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.meterSet.trackCount, 2, "no lease, no switch");
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      assert.equal(processor.meterSet.trackCount, 3);
      assert.equal(processor.pendingMeterSet, null);
      fake.calls.meterWindows = 1;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.at(-1).message.trackCount, 3);
    }

    {
      // A header that says it was replaced with no set cut for its successor is a protocol
      // fault: sticky and silent, never a frame read through views over a retired plan.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      processor.receive({ tag: "miso.meters.v1", requestId: 1, enabled: true });
      fake.calls.adoptOnNextRender = true;
      fake.calls.meterWindows = 1;
      const posts = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, posts);
      assert.equal(processor.stickyResult, 255);
      assert.equal(processor.ready, false);
    }

    {
      // Issue #137 D3: a full telemetry window posts exactly one frame, and the frame is honest
      // about the resolution of the clock it actually found. The real process clock made the
      // zero-miss assertion scheduler-sensitive, so each window uses a local monotonic fixture.
      const belowBudgetMs = 0.5;
      const aboveBudgetMs = 2;
      const noMissWindow = Array.from({ length: 128 }, () => belowBudgetMs);
      const oneMissWindow = noMissWindow.map((duration, block) => block === 64
        ? aboveBudgetMs
        : duration);
      const runTelemetryWindow = (elapsedMsByBlock, expectedDeadlineMisses) => {
        const clock = createTelemetryClock(elapsedMsByBlock);
        return withTelemetryClock(clock, () => {
          const { processor } = makeProcessor();
          processor.receive({ tag: "miso.telemetry.v1", requestId: 1, enabled: true });
          assert.deepEqual(processor.port.posts.at(-1).message, {
            tag: "miso.ack.v1", requestId: 1, result: 0,
          });
          const left = new Float32Array(64);
          const right = new Float32Array(64);
          let frames = 0;
          let telemetry;
          for (let block = 0; block < 128; block += 1) {
            assert.equal(processor.process([], [[left, right]]), true);
            const last = processor.port.posts.at(-1).message;
            if (last.tag === "miso.telemetry.v1") {
              frames += 1;
              telemetry = last;
            }
          }
          assert.equal(frames, 1, "one frame per 128-block window and no more");
          assert.equal(telemetry.blocks, 128);
          assert.equal(telemetry.sequence, 1);
          assert.equal(telemetry.deadlineMisses, expectedDeadlineMisses);
          assert(telemetry.budgetMs > 1.3 && telemetry.budgetMs < 1.4, telemetry.budgetMs);
          assert(telemetry.resolutionMs > 0);
          assert.equal(typeof telemetry.belowResolution, "boolean");
          for (const field of ["cpuPercent", "peakBlockMs", "meanBlockMs"]) {
            assert(Number.isFinite(telemetry[field]) && telemetry[field] >= 0, field);
          }
          assert.equal(clock.reads, 256, "two clock reads per leased rendered block");
          processor.receive({ tag: "miso.telemetry.v1", requestId: 2, enabled: false });
          const quiet = processor.port.posts.length;
          const readsAfterRelease = clock.reads;
          for (let block = 0; block < 200; block += 1) {
            assert.equal(processor.process([], [[left, right]]), true);
          }
          assert.equal(clock.reads, readsAfterRelease, "a released lease reads no clock");
          assert.equal(processor.port.posts.length, quiet, "a released lease posts nothing");
        });
      };

      // The first fresh processor has 128 positive sub-budget blocks; the second has exactly one
      // over-budget block, preserving the behavioral deadline-miss discriminator.
      runTelemetryWindow(noMissWindow, 0);
      runTelemetryWindow(oneMissWindow, 1);
    }

    {
      // The session map answers from the engine's rows. Two submixes
      // (#1210 MINOR-1, #1214 MINOR-2): the SDK indexes bus commands by this list's order, so red
      // if the worklet enumerates or posts the submix IDs in any other order.
      const { processor, fake } = makeProcessor(["aa-bus", "zz-bus"]);
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 1 });
      const map = processor.port.posts.at(-1).message;
      assert.equal(map.tag, "miso.sessionmap.v1");
      assert.equal(Object.hasOwn(map, "tracks"), false, "issue #1565: the strip order is the core's list");
      assert.deepEqual(map.sources, fake.sourceRows, "issue #207: canonical source order and shape");
      assert.equal(map.metersAttached, true);
      assert.equal(Object.hasOwn(map, "submixes"), false, "issue #1565: the strip order is the core's list");
      assert.equal(Object.hasOwn(map, "routes"), false, "issue #1565: no route list");
      assert.equal(Object.hasOwn(map, "vcas"), false, "issue #1565: no VCA list");
      // A second map answers from the same numbers, and `process()` never sees any of this.
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 2 });
      assert.deepEqual(processor.port.posts.at(-1).message.sources, fake.sourceRows);
    }

    {
      // Issue #1562b: a rebuild can change the committed sources, so the session map is read from
      // the engine at each request. Red if the worklet answers from the boot's rows (a source
      // added by a rebuild would never be listed). (#1590 deleted the observation map.)
      const { processor, fake } = makeProcessor();
      fake.sourceRows.push({ id: "zz-added", channels: 1, frames: 480n });
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 1 });
      assert.deepEqual(
        processor.port.posts.at(-1).message.sources.map((source) => source.id),
        ["bass", "drums", "zz-added"],
        "the session map lists a source a rebuild added",
      );
    }

    {
      // Issue #241: every source read is checked against something compilation already guarantees,
      // so a mis-wired export fails initialization instead of reaching a consumer as a plausible
      // number. Each mutation below is a different export lying, and each must be caught.
      for (const [what, mutate] of [
        ["a zero channel count", (e) => { e.miso_engine_web_v1_source_channels = () => 0; }],
        ["a channel count past the configured maximum",
          (e) => { e.miso_engine_web_v1_source_channels = () => 3; }],
        ["a zero frame count", (e) => { e.miso_engine_web_v1_source_frames = () => 0n; }],
        ["an empty source ID", (e) => { e.miso_engine_web_v1_source_id = () => 0; }],
        ["a source ID longer than staging",
          (e) => { e.miso_engine_web_v1_source_id = () => 65; }],
        // The meter header's strip counts size the frame views, so a header whose counts do not
        // match the frame the engine sized fails initialization instead of misplacing a view.
        ["a meter header submix count the frame disagrees with",
          (_e, fake) => { fake.meterHeader.setUint32(64, 2, true); }],
        ["a meter header track count the frame disagrees with",
          (_e, fake) => { fake.meterHeader.setUint32(8, 3, true); }],
      ]) {
        const fake = createFakeExports(64);
        mutate(fake.exports, fake);
        const processor = construct(fake);
        assert.equal(
          processor.port.posts[0].message.result,
          255,
          `${what} must fail initialization`,
        );
      }
    }

    // Issue #1332 S7 (D3): the worklet's `worker` mode. The control Worker booted the session on
    // the memory it passes; the worklet sets its own stack and TLS from the reservation, binds it,
    // adopts the token and only renders. Red if the worklet boots, skips the bind or the adoption.
    const workerToken = (1n << 32n) | 5n;
    const constructWorkerMode = (fake, overrides = {}) => {
      nextFake = fake;
      workerModeMemory = fake.memory;
      const record = new DataView(fake.memory.buffer, 70000, 16);
      record.setUint32(0, 16, true);
      record.setUint32(4, 71000, true);
      record.setUint32(8, 180000, true);
      const processor = new registered({
        processorOptions: {
          instanceMode: "worker", module: {}, memory: fake.memory, token: workerToken,
          reservation: 70000, metersAttached: true, ...overrides,
        },
      });
      const controlPosts = [];
      const controlPort = { onmessage: null, postMessage: (message) => controlPosts.push(message) };
      processor.port.onmessage({ data: { tag: "miso.control-port.v1", port: controlPort } });
      const fromWorker = (data) => controlPort.onmessage({ data });
      return { processor, controlPosts, fromWorker };
    };
    const renderStatus = (processor, fromWorker, controlPosts) => {
      fromWorker({ tag: "miso.render-status.v1", requestId: 9 });
      const reply = controlPosts.at(-1);
      assert.equal(reply.tag, "miso.render-status.v1");
      return reply.status;
    };
    {
      const fake = createFakeExports(64);
      const { processor } = constructWorkerMode(fake);
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "simd128", instanceMode: "worker",
      }, "worker mode is ready without a boot");
      assert.deepEqual(fake.calls.boot, [], "worker mode never calls boot on the worklet");
      assert.deepEqual(fake.calls.initTls, [[71000, 180000]],
        "the TLS block is initialised once, after the stack pointer is the reservation's top");
      assert.deepEqual(fake.calls.bind, [70000], "the worklet binds the reservation");
      assert.deepEqual(fake.calls.adopt, [workerToken], "the worklet adopts the Worker's token");
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(left[0], 0.25, "the adopted half renders");
      // A refused adoption fails construction; nothing is booted or disposed.
      const refused = createFakeExports(64);
      refused.calls.adoptResult = 3;
      const { processor: failed } = constructWorkerMode(refused);
      assert.deepEqual(failed.port.posts[0].message, { tag: "miso.error.v1", requestId: 0, result: 3 });
      assert.equal(refused.calls.dispose, 0, "worker mode never disposes in the worklet");
      // Requests the control Worker owns are refused on the worklet's port.
      const { processor: strict } = constructWorkerMode(createFakeExports(64));
      strict.port.onmessage({ data: { tag: "miso.status.v1", requestId: 1 } });
      assert.equal(strict.port.posts.at(-1).message.tag, "miso.error.v1");
    }
    // R-S7-meters (3): `process()` switches to the pre-cut set only when the header render reports,
    // its frame pointer and its strip counts all match; otherwise it skips the frame and counts it.
    {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      assert.equal(processor.port.posts.at(-1).message.result, 0);
      const successor = fake.successorHeader.byteOffset;
      fromWorker({ tag: "miso.meter-view.v1", header: successor, memoryBytes: fake.memory.buffer.byteLength });
      fake.calls.adoptOnNextRender = true;
      fake.calls.meterWindows = 1;
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      assert.equal(processor.process([], [[left, right]]), true);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1", "a full match switches and posts");
      assert.equal(frame.trackCount, 3, "the frame is the successor plan's");
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 0);
    }
    for (const variant of ["counts differ", "no pre-cut set"]) {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      const successor = fake.successorHeader.byteOffset;
      if (variant === "counts differ") {
        fromWorker({ tag: "miso.meter-view.v1", header: successor, memoryBytes: fake.memory.buffer.byteLength });
        // The header was reused for a plan of another strip count after the handler cut its set.
        fake.successorHeader.setUint32(8, 7, true);
      }
      fake.calls.adoptOnNextRender = true;
      fake.calls.meterWindows = 1;
      const posts = processor.port.posts.length;
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), true);
      assert.equal(processor.port.posts.length, posts, `${variant}: the frame is skipped`);
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 1,
        `${variant}: the skip is counted`);
    }
    // D4.1: a release stops rendering and hands the token to the Worker; nothing is disposed here.
    {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      fromWorker({ tag: "miso.release.v1" });
      assert.deepEqual(controlPosts.at(-1), { tag: "miso.released.v1", token: (1n << 32n) | 1n });
      assert.deepEqual(fake.calls.release, [1]);
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }
    // R-S7-meters as amended (S7 verify MAJOR 1): a set is the four-part key, compared on every
    // block. A freed header address reused by a later plan (another frame, other counts) is not
    // the active set even though the pointer did not change: the frame is skipped and counted,
    // and the next view message converges on the new plan's set.
    {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      const reused = fake.meterHeader.byteOffset;
      fake.meterHeader.setUint32(8, 3, true);
      fake.meterHeader.setUint32(72, 44000, true);
      fake.calls.meterWindows = 1;
      const posts = processor.port.posts.length;
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      assert.equal(processor.port.posts.length, posts, "a reused header address is not the active set");
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 1);
      fromWorker({ tag: "miso.meter-view.v1", header: reused, memoryBytes: fake.memory.buffer.byteLength });
      fake.calls.meterWindows = 1;
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1", "the reused address converges on its new set");
      assert.equal(frame.trackCount, 3);
    }
    // S7 verify MAJOR 2: plans B and C committed back to back, and render adopted B before the
    // handler had cut it while C's view is pending. The handler makes B active and keeps C, so
    // the meters converge when render adopts C.
    {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      const headerB = fake.successorHeader.byteOffset;
      const headerC = 42000;
      new Uint8Array(fake.memory.buffer, headerC, 88).set(new Uint8Array(fake.memory.buffer, headerB, 88));
      const viewC = new DataView(fake.memory.buffer, headerC, 88);
      viewC.setUint32(8, 1, true);
      viewC.setUint32(72, 46000, true);
      let current = fake.meterHeader.byteOffset;
      fake.exports.miso_engine_web_v1_meter_header_ptr = () => current;
      fromWorker({ tag: "miso.meter-view.v1", header: headerC, memoryBytes: fake.memory.buffer.byteLength });
      current = headerB;
      fake.calls.meterWindows = 1;
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 1,
        "B was never cut: one skip");
      processor.port.onmessage({ data: { tag: "miso.telemetry.v1", requestId: 2, enabled: false } });
      current = headerC;
      fake.calls.meterWindows = 1;
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1", "the pending C survives B's adoption");
      assert.equal(frame.trackCount, 1);
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 1);
    }
    // S7 verify attempt 3, MINOR 1: a plan C that reuses the active set A's address and words
    // while render reports B (uncut) is cut as pending, so the meters converge when C is adopted.
    {
      const fake = createFakeExports(64);
      const { processor, controlPosts, fromWorker } = constructWorkerMode(fake);
      processor.port.onmessage({ data: { tag: "miso.meters.v1", requestId: 1, enabled: true } });
      const headerA = fake.meterHeader.byteOffset;
      const headerB = fake.successorHeader.byteOffset;
      let current = headerB;
      fake.exports.miso_engine_web_v1_meter_header_ptr = () => current;
      fromWorker({ tag: "miso.meter-view.v1", header: headerA, memoryBytes: fake.memory.buffer.byteLength });
      current = headerA;
      fake.calls.meterWindows = 1;
      processor.process([], [[new Float32Array(64), new Float32Array(64)]]);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1", "C at A's address with A's words is cut and adopted");
      assert.equal(frame.trackCount, 2);
      assert.equal(renderStatus(processor, fromWorker, controlPosts).meterFramesSkipped, 0);
    }
    // A release that throws (a trapped instance) still answers the Worker, with no token.
    {
      const fake = createFakeExports(64);
      const { controlPosts, fromWorker } = constructWorkerMode(fake);
      fake.exports.miso_engine_web_v1_host_release = () => { throw new Error("trapped"); };
      fromWorker({ tag: "miso.release.v1" });
      assert.deepEqual(controlPosts.at(-1), { tag: "miso.released.v1", token: 0n });
    }
    workerModeMemory = null;

    await testQualificationBoot({
      registered,
      makeFake: createFakeExports,
      setNextFake: (fake) => { nextFake = fake; },
      setProcessorPortFactory: (factory) => { processorPortFactory = factory; },
      document: processorSessionDocument,
      mainRealmTextEncoder: originalTextEncoder,
    });

    // Issue #1496 D3. The cases above reach the engine worklet's render, source, seek and meter
    // paths, so the recording is not empty; an empty one would mean the proxy lost its hook.
    // A call that the table does not list for the engine worklet is a post-construction export
    // that no `render_locked` rule has been asked to prove.
    for (const name of ["render", "source_submit", "meter_poll"]) {
      assert.ok(recordedPostConstructionCalls.has(name),
        `the recording proxy saw the engine worklet call ${name} after construction`);
    }
    assert.ok(importsChecked > 0, "the worklet instantiated the module at least once");
    assert.deepEqual(importViolations, [],
      "every worklet instance imports one fresh shared memory at the ABI layout's pages");
    const unlisted = [...recordedPostConstructionCalls].filter((name) => !engineRenderThreadExports.has(name));
    assert.deepEqual(unlisted, [],
      `the engine worklet calls these exports after construction, and tools/realtime-policy/render-thread-exports.json does not list them for it: ${unlisted.map((name) => renderThreadPrefix + name).join(", ")}`);
  } finally {
    globalThis.AudioWorkletProcessor = originalProcessor;
    globalThis.registerProcessor = originalRegister;
    globalThis.sampleRate = originalSampleRate;
    if (originalRenderQuantumSize === undefined) delete globalThis.renderQuantumSize;
    else globalThis.renderQuantumSize = originalRenderQuantumSize;
    globalThis.TextEncoder = originalTextEncoder;
    WebAssembly.Instance = originalInstance;
    WebAssembly.Memory = originalMemory;
  }
}

// Issue #1332 S7 (D2, D4, D7, R-S7-meters 2): the control Worker file, driven on the fake module.
// Red if it boots without reserving the worklet's stack and TLS or releasing the render half, if
// a control call does not name a changed successor header or memory length to the worklet, or if
// its dispose frees without taking the half back first.
async function testControlWorker() {
  const workerUrl = process.env.MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE === undefined
    ? new URL("hosts/host-web/web/miso-engine-v1-control-worker.js", root)
    : pathToFileURL(process.env.MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE);
  const original = {
    Instance: WebAssembly.Instance,
    postMessage: globalThis.postMessage,
    onmessage: globalThis.onmessage,
    setInterval: globalThis.setInterval,
    clearInterval: globalThis.clearInterval,
  };
  const module = await WebAssembly.compile(unsupportedPreparationModuleBytes);
  let fake;
  let posts;
  const intervals = [];
  try {
    globalThis.setInterval = (callback, delay) => {
      intervals.push({ callback, delay, cleared: false });
      return intervals.length;
    };
    globalThis.clearInterval = (id) => { intervals[id - 1].cleared = true; };
    globalThis.postMessage = (message, transfer = []) => {
      posts.push(structuredClone(message, { transfer }));
    };
    WebAssembly.Instance = class {
      constructor(_module, imports) {
        assert.equal(imports.env.memory, fake.memory, "the Worker instantiates on the page's memory");
        this.exports = fake.exports;
      }
    };
    const workerSource = await readFile(workerUrl, "utf8");
    const boot = async (overrides = {}) => {
      fake = createFakeExports(64);
      posts = [];
      // The file is a classic script with no module syntax (Node would load it once, as CommonJS),
      // so each case evaluates it afresh: one Worker per boot, as in the browser.
      new Function(workerSource)();
      const workletPosts = [];
      const workletPort = { onmessage: null, postMessage: (message) => workletPosts.push(message) };
      const send = (data) => globalThis.onmessage({ data });
      send({
        tag: "miso.worker-boot.v1", module, memory: fake.memory, document: new Uint8Array([1, 2, 3]),
        options: limits, sampleRateHz: 48000, quantumFrames: 64, workletPort, ...overrides,
      });
      return { workletPosts, send, fromWorklet: (data) => workletPort.onmessage({ data }) };
    };

    {
      const { workletPosts, send, fromWorklet } = await boot();
      const ready = posts[0];
      assert.equal(ready.tag, "miso.worker-ready.v1", JSON.stringify(ready, (_, v) => (typeof v === "bigint" ? `${v}` : v)));
      assert.deepEqual(
        [ready.handle, ready.token, ready.reservation, ready.metersAttached, ready.sourceRingFrames],
        [1, (1n << 32n) | 1n, 70000, true, 40 * 64],
        "the boot reply carries the handle, the token, the reservation and the ready fields",
      );
      assert.deepEqual(fake.calls.boot, [[3]], "the Worker boots the document");
      assert.deepEqual(fake.calls.reserve, [[64, 16]], "it reserves the worklet's stack and TLS");
      assert.deepEqual(fake.calls.release, [1], "it releases the render half");
      assert.equal(intervals.length, 1, "the Worker holds the service tick");
      // A control call runs here and is followed by the service step and the meter view.
      fake.calls.successorHeader = 41300;
      send({ tag: "miso.control.v1", requestId: 1, frame: new Uint8Array([5, 6]) });
      const reply = posts.find((message) => message.tag === "miso.control.v1");
      assert.deepEqual([reply.requestId, reply.result, [...reply.response], reply.revision],
        [1, 0, [9, 8, 7], 4n]);
      assert.deepEqual([...fake.calls.control[0].frame], [5, 6]);
      assert.deepEqual(fake.calls.service, [1], "the service step follows the request");
      assert.deepEqual(workletPosts, [{ tag: "miso.meter-view.v1", header: 41300, memoryBytes: 196608 }],
        "the committed successor's header is named to the worklet");
      intervals[0].callback();
      assert.equal(workletPosts.length, 1, "an unchanged view is not posted again");
      fake.calls.controlGrowsMemory = true;
      send({ tag: "miso.control.v1", requestId: 2, frame: new Uint8Array([5]) });
      assert.deepEqual(workletPosts.at(-1), { tag: "miso.meter-view.v1", header: 41300, memoryBytes: 393216 },
        "a growth is named to the worklet");
      // A status read takes the worklet's render fields and the Worker's control fields.
      send({ tag: "miso.status.v1", requestId: 3 });
      assert.deepEqual(workletPosts.at(-1), { tag: "miso.render-status.v1", requestId: 3 });
      fromWorklet({
        tag: "miso.render-status.v1", requestId: 3,
        status: { state: 2, lastResult: 0, renderedQuanta: 9n, blockingRebuilds: 7, meterFramesSkipped: 0 },
      });
      const status = posts.at(-1);
      assert.deepEqual([status.tag, status.requestId, status.renderedQuanta, status.blockingRebuilds, status.memoryBytes],
        ["miso.status.v1", 3, 9n, 0, 393216], "render fields from the worklet, control fields from the Worker");
      // D4: dispose takes the half back from the worklet, then frees it here. A status read in
      // flight across it is answered, and a control call after it began is refused WRONG_STATE.
      send({ tag: "miso.status.v1", requestId: 4 });
      send({ tag: "miso.dispose.v1", requestId: 5 });
      assert.deepEqual(workletPosts.at(-1), { tag: "miso.release.v1" });
      const controlsBefore = fake.calls.control.length;
      send({ tag: "miso.control.v1", requestId: 6, frame: new Uint8Array([5]) });
      assert.deepEqual([posts.at(-1).tag, posts.at(-1).requestId, posts.at(-1).result],
        ["miso.control.v1", 6, 3], "a control call after the dispose began is refused");
      assert.equal(fake.calls.control.length, controlsBefore, "and runs nothing");
      const releases = workletPosts.length;
      send({ tag: "miso.dispose.v1", requestId: 7 });
      assert.equal(workletPosts.length, releases, "a second dispose posts no second release");
      send({ tag: "miso.sessionmap.v1", requestId: 8 });
      assert.deepEqual([posts.at(-1).tag, posts.at(-1).result, posts.at(-1).sources.length],
        ["miso.sessionmap.v1", 0, 2], "the session map is answered while the dispose runs");
      assert.equal(fake.calls.dispose, 0, "nothing is freed before the half is back");
      fromWorklet({ tag: "miso.released.v1", token: (1n << 32n) | 2n });
      assert.deepEqual(fake.calls.adopt, [(1n << 32n) | 2n], "the Worker adopts the half back");
      assert.equal(fake.calls.dispose, 1, "and disposes the host");
      const statusReply = posts.find((message) => message.tag === "miso.status.v1" && message.requestId === 4);
      assert.equal(statusReply?.result, 0, "the status read in flight across the dispose is answered");
      assert.deepEqual(posts.slice(-2), [
        { tag: "miso.ack.v1", requestId: 5, result: 0 },
        { tag: "miso.ack.v1", requestId: 7, result: 0 },
      ], "both disposes are answered together");
      assert.equal(posts.some((message) => message.tag === "miso.error.v1"), false,
        "no refusal during the dispose is sticky");
      send({ tag: "miso.dispose.v1", requestId: 9 });
      assert.deepEqual(posts.at(-1), { tag: "miso.ack.v1", requestId: 9, result: 0 },
        "a dispose after the host is freed is answered at once with the outcome");
      assert.equal(intervals[0].cleared, true, "dispose stops the tick");
    }
    // A trap in the Worker's dispose is a final outcome: the dispose is answered 255, and a later
    // dispose is answered at once with it (it never joins a dispose that will not complete).
    {
      const { workletPosts, send, fromWorklet } = await boot();
      fake.exports.miso_engine_web_v1_dispose = () => { throw new Error("trap"); };
      send({ tag: "miso.dispose.v1", requestId: 1 });
      assert.ok(workletPosts.some((message) => message.tag === "miso.release.v1"));
      fromWorklet({ tag: "miso.released.v1", token: (1n << 32n) | 2n });
      assert.deepEqual(posts.at(-1), { tag: "miso.ack.v1", requestId: 1, result: 255 });
      send({ tag: "miso.dispose.v1", requestId: 2 });
      assert.deepEqual(posts.at(-1), { tag: "miso.ack.v1", requestId: 2, result: 255 });
    }
    // D6: a refused boot answers with the engine's result code and releases nothing.
    {
      await boot({ sampleRateHz: 44100 });
      assert.deepEqual(posts, [{ tag: "miso.error.v1", requestId: 0, result: 9 }]);
      assert.deepEqual(fake.calls.release, []);
    }
  } finally {
    WebAssembly.Instance = original.Instance;
    globalThis.postMessage = original.postMessage;
    globalThis.onmessage = original.onmessage;
    globalThis.setInterval = original.setInterval;
    globalThis.clearInterval = original.clearInterval;
  }
}

await testMainRealm();
await testProcessor();
await testControlWorker();
console.log("web AudioWorklet hermetic tests passed");
