import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
const root = new URL("../", import.meta.url);
const commandArguments = process.argv.slice(2);
const bootDataModeFlags = [
  ["--boot-data-document", "document"],
  ["--boot-data-nested", "nested"],
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
  // Issue #137 D1/D2 and #143 D3: the live-control words. `reserved1` (offset 32, issue #1565)
  // and `reserved2` (offset 56, the master boot word until issue #1689) must be zero. All zero is
  // "no meter observers, no observation capacity"; the live-control tests below override them.
  reserved1: 0n,
  liveControlMeterBlocks: 2n,
  liveControlObservationTaps: 2n,
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
  let sessionMapMutation = null;
  let planeMutation = null;
  let commandResult = 0;
  let mixedSuccess = false;
  let commandMutation = null;
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
            lastResult: 0, backend: 1, sampleRateHz: 48000, quantumFrames: 64,
            nextAbsoluteSample: 64n, renderedQuanta: 1n, liveValuesSuperseded: 7n,
            appliedRevision: 9n, appliedSample: 64n, appliedOutcome: 1n,
            memoryBytes: 65536, singleModeControlAllocations: 3, staleViewQuanta: 0,
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
          response = {
            tag: "miso.control.v1", requestId: received.requestId, result: 0,
            response: new Uint8Array([...frame].reverse()), revision: 2n,
          };
          if (controlMutation !== null) response = controlMutation(response);
          responseTransfer = response.response instanceof Uint8Array ? [response.response.buffer] : [];
        } else if (received.tag === "miso.command.v1") {
          response = {
            tag: "miso.ack.v1",
            requestId: received.requestId,
            result: commandResult,
            reason: commandResult === 0 ? 0 : 8,
            rejectedIndex: 0,
            admitted: commandResult === 0 ? received.count : 0,
            appliedAtSample: 512n,
            records: received.records,
          };
          if (commandMutation !== null) response = commandMutation(response);
          responseTransfer = [received.records.buffer];
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
        } else {
          response = { tag: "miso.ack.v1", requestId: received.requestId, result: 0 };
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
        let data = {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: "simd128",
          resources: resourceReport(1, this.constructionQuantumFrames),
          memoryBytes: 65536,
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

    const runBootDataNested = async () => {
      const gate = snapshotGate();
      addModuleGate = gate;
      const firstEntry = { target: "output", targetId: "first", channels: "both" };
      const thirdEntry = { target: "trackPostPan", targetId: "third", channels: "left" };
      const entries = [firstEntry, , thirdEntry];
      const collection = { entries, maximumCaptureBytes: 4096 };
      const factory = bootDataFactory({
        document: new Uint8Array(),
        includePreparedModule: true,
        workletModuleUrl: "boot-data-nested-processor.js",
        options: { ...limits, spectrum: null, spectrumCollection: collection },
      });
      const expected = {
        ...limits,
        spectrumHopFrames: 0,
        spectrum: null,
        spectrumCollection: {
          entries: [
            { target: "output", targetId: "first", channels: "both" },
            ,
            { target: "trackPostPan", targetId: "third", channels: "left" },
          ],
          maximumCaptureBytes: 4096,
        },
      };
      const eventsBefore = events.length;
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        firstEntry.targetId = "mutated-first";
        thirdEntry.channels = "right";
        entries[0] = { target: "output", targetId: "replaced-entry", channels: "right" };
        factory.options.spectrumCollection = {
          entries: [{ target: "output", targetId: "replaced-collection", channels: "both" }],
          maximumCaptureBytes: 8192,
        };
        gate.release();
        hostValue = await boot;
        const node = FakeNode.latest;
        assert.deepEqual(
          node.constructionSnapshot.options,
          expected,
          "boot-data.nested.before-construction",
        );
        const capturedCollection = node.constructionSnapshot.options.spectrumCollection;
        assert.equal(
          Object.hasOwn(capturedCollection.entries, 1),
          false,
          "boot-data.nested.sparse-hole",
        );
        assert.notEqual(
          node.options.processorOptions.options.spectrumCollection,
          collection,
          "boot-data.nested.private-collection",
        );
        assert.notEqual(
          node.options.processorOptions.options.spectrumCollection.entries,
          entries,
          "boot-data.nested.private-entries",
        );
        assert.equal(node.constructionModule, unsupportedPreparationModule, "boot-data.nested.module");
        assert.deepEqual(
          events.slice(eventsBefore),
          [["addModule", "boot-data-nested-processor.js"]],
          "boot-data.nested.supplied-module-loading",
        );
      } finally {
        gate.release();
        if (addModuleGate === gate) addModuleGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }
    };

    if (bootDataTestMode === "document") {
      await runBootDataDocument();
      return;
    }
    if (bootDataTestMode === "nested") {
      await runBootDataNested();
      return;
    }

    const runPlainDataMatrix = async () => {
      const makeFactory = (overrides = {}) => ({
        context, document: new Uint8Array(), options: limits,
        simd128ModuleUrl: "simd.wasm", workletModuleUrl: "processor.js", ...overrides,
      });
      const makeEntry = () => ({ target: "output", targetId: "main-out", channels: "both" });
      const makeSpectrum = () => ({
        target: "trackPostPan", targetId: "eq0", channels: "both", maximumCaptureBytes: 4096,
      });
      const makeCollectionOptions = (entries) => ({
        ...limits, spectrum: null, spectrumCollection: { entries, maximumCaptureBytes: 4096 },
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
        ["spectrum", () => {
          const s = makeSpectrum(); const o = { ...limits, spectrum: s, spectrumCollection: null };
          return [makeFactory({ options: o }), s, "target"];
        }],
        ["collection", () => {
          const e = [makeEntry()]; const o = makeCollectionOptions(e);
          return [makeFactory({ options: o }), o.spectrumCollection, "entries"];
        }],
        ["array-index", () => {
          const e = [makeEntry()]; const o = makeCollectionOptions(e);
          return [makeFactory({ options: o }), e, "0"];
        }],
        ["entry", () => {
          const e = makeEntry(); const o = makeCollectionOptions([e]);
          return [makeFactory({ options: o }), e, "targetId"];
        }],
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

      const decoratedEntries = [
        ["every=false", "every", false],
        ["every=undefined", "every", undefined, false],
        ["every callback", "every", "callback"],
        ["map", "map", "callback"],
        ["constructor", "constructor", "callback"],
        ["Symbol.iterator", Symbol.iterator, "callback"],
        ["Symbol.species", Symbol.species, "callback"],
      ];
      for (const [label, key, value, enumerable = true] of decoratedEntries) {
        let calls = 0;
        let retained = null;
        const entries = [makeEntry()];
        const decoration = value === "callback" ? (...args) => {
          calls += 1;
          retained = args[0] ?? null;
          return true;
        } : value;
        Object.defineProperty(entries, key, {
          configurable: true, enumerable, value: decoration, writable: true,
        });
        await assertRefusal(`decorated entries ${label}`, makeFactory({
          options: makeCollectionOptions(entries),
        }), () => calls);
        assert.equal(retained, null, `decorated entries ${label}: callback escaped`);
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
        ["custom-prototype entries", () => {
          const entries = [makeEntry()]; Object.setPrototypeOf(entries, { custom: true });
          return makeFactory({ options: makeCollectionOptions(entries) });
        }],
        ["subclass entries", () => {
          class EntriesSubclass extends Array {}
          const entries = new EntriesSubclass(makeEntry());
          return makeFactory({ options: makeCollectionOptions(entries) });
        }],
        ["noncanonical array index", () => {
          const entries = [makeEntry()]; Object.defineProperty(entries, "01", { value: makeEntry() });
          return makeFactory({ options: makeCollectionOptions(entries) });
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
        const spectrum = Object.assign(Object.create(null), makeSpectrum());
        const spectrumOptions = Object.assign(Object.create(null), limits, {
          spectrum, spectrumCollection: null,
        });
        await assertAccepted(
          "null-prototype spectrum record",
          makeFactory({ options: spectrumOptions }),
        );
        const entry = Object.assign(Object.create(null), makeEntry());
        const collection = Object.assign(Object.create(null), {
          entries: [entry], maximumCaptureBytes: 4096,
        });
        const collectionOptions = Object.assign(Object.create(null), limits, {
          spectrum: null, spectrumCollection: collection,
        });
        await assertAccepted(
          "null-prototype collection and entry records",
          makeFactory({ options: collectionOptions }),
        );
      }
      {
        const options = Object.freeze({ ...limits });
        await assertAccepted("frozen records", Object.freeze(makeFactory({ options })));
      }
      await assertAccepted(
        "legacy six-field shape",
        makeFactory({ options: { ...limits } }),
        (_host, options) => {
          assert.equal(options.spectrum, null, "legacy spectrum normalizes to null");
          assert.equal(options.spectrumCollection, null,
            "legacy spectrum collection normalizes to null");
        },
      );
      await assertAccepted(
        "extended eight-field shape and null normalization",
        makeFactory({ options: { ...limits, spectrum: null, spectrumCollection: null } }),
        (_host, options) => {
          assert.equal(options.spectrum, null, "explicit spectrum null is preserved");
          assert.equal(options.spectrumCollection, null, "explicit collection null is preserved");
        },
      );
      await assertAccepted(
        "extended undefined spectrum normalizes to null",
        makeFactory({ options: { ...limits, spectrum: undefined, spectrumCollection: null } }),
        (_host, options) => {
          assert.equal(options.spectrum, null, "enumerable undefined spectrum normalizes to null");
          assert.equal(options.spectrumCollection, null, "enumerable null collection is preserved");
        },
      );
      await assertAccepted(
        "extended undefined spectrum keeps collection",
        makeFactory({
          options: {
            ...limits,
            spectrum: undefined,
            spectrumCollection: { entries: [makeEntry()], maximumCaptureBytes: 4096 },
          },
        }),
        (_host, options) => {
          assert.equal(options.spectrum, null, "undefined spectrum normalizes to null");
          assert.equal(options.spectrumCollection.entries.length, 1,
            "nonnull collection remains effective");
          assert.equal(options.spectrumCollection.entries[0].target, "output",
            "nonnull collection entry remains effective");
        },
      );
      for (const hop of [256, 1024]) {
        await assertAccepted(
          `explicit spectrum hop ${hop}`,
          makeFactory({
            options: {
              ...limits,
              spectrumHopFrames: hop,
              spectrum: null,
              spectrumCollection: null,
            },
          }),
          (_host, options) => {
            assert.equal(options.spectrumHopFrames, hop, "explicit hop is forwarded");
            assert.equal(options.spectrum, null);
            assert.equal(options.spectrumCollection, null);
          },
        );
      }
      for (const hop of [256, 1024]) {
        await assertAccepted(
          `explicit spectrum hop ${hop} without optional fields`,
          makeFactory({ options: { ...limits, spectrumHopFrames: hop } }),
          (_host, options) => {
            assert.equal(options.spectrumHopFrames, hop, "explicit hop is forwarded");
            assert.equal(options.spectrum, null, "omitted spectrum normalizes to null");
            assert.equal(options.spectrumCollection, null,
              "omitted spectrum collection normalizes to null");
          },
        );
      }
      {
        const first = Object.freeze(makeEntry());
        const second = Object.freeze(makeEntry());
        const entries = new Array(3);
        Object.defineProperty(entries, "0", {
          value: first, writable: false, enumerable: false, configurable: false,
        });
        entries[2] = second;
        Object.freeze(entries);
        const collection = Object.freeze({ entries, maximumCaptureBytes: 4096 });
        const options = Object.freeze({
          ...limits, spectrum: null, spectrumCollection: collection,
        });
        await assertAccepted(
          "frozen sparse exact entries",
          Object.freeze(makeFactory({ options })),
          (_host, captured) => {
            const copiedCollection = captured.spectrumCollection;
            const copiedEntries = copiedCollection.entries;
            assert.equal(copiedEntries.length, 3, "sparse entries length captured");
            assert.equal(Object.hasOwn(copiedEntries, 0), true, "sparse index is present");
            assert.equal(Object.hasOwn(copiedEntries, 1), false, "sparse hole is retained");
            assert.equal(
              Object.getOwnPropertyDescriptor(copiedEntries, "0").enumerable,
              false,
              "hidden index enumerability is retained",
            );
            assert.equal(
              Object.getOwnPropertyDescriptor(copiedEntries, "2").enumerable,
              true,
              "visible index enumerability is retained",
            );
            assert.notEqual(copiedCollection, collection, "collection snapshot is fresh");
            assert.notEqual(copiedEntries, entries, "entries snapshot is fresh");
            assert.notEqual(copiedEntries[0], first, "nested entry snapshot is fresh");
            assert.notEqual(copiedEntries[2], second, "second nested snapshot is fresh");
          },
        );
      }
      {
        const options = { ...limits };
        Object.defineProperty(options, "spectrumCollection", {
          value: { entries: [makeEntry()], maximumCaptureBytes: 4096 },
          enumerable: false,
        });
        await assertAccepted(
          "hidden collection remains effective",
          makeFactory({ options }),
          (_host, captured) => {
            assert.equal(captured.spectrumCollection.entries.length, 1);
            assert.equal(captured.spectrumCollection.entries[0].target, "output");
          },
        );
      }
      {
        const options = { ...limits };
        Object.defineProperty(options, "spectrum", { value: null, enumerable: false });
        await assertAccepted(
          "hidden null spectrum normalizes to null",
          makeFactory({ options }),
          (_host, captured) => assert.equal(captured.spectrum, null),
        );
      }

      await assertRefusal(
        "exact seven-field shape",
        makeFactory({ options: { ...limits, spectrum: null } }),
      );
      await assertRefusal(
        "enumerable undefined spectrum shape",
        makeFactory({ options: { ...limits, spectrum: undefined } }),
      );
      {
        const options = { ...limits };
        Object.defineProperty(options, "spectrum", {
          value: makeSpectrum(), enumerable: false,
        });
        await assertRefusal("hidden nonnull spectrum", makeFactory({ options }));
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
      {
        const collection = { maximumCaptureBytes: 4096 };
        Object.defineProperty(collection, "entries", {
          value: [makeEntry()], enumerable: false,
        });
        await assertRefusal(
          "hidden required collection entries",
          makeFactory({ options: { ...limits, spectrum: null, spectrumCollection: collection } }),
        );
      }
      await assertRefusal(
        "mutually exclusive spectrum settings",
        makeFactory({
          options: {
            ...limits,
            spectrum: makeSpectrum(),
            spectrumCollection: { entries: [makeEntry()], maximumCaptureBytes: 4096 },
          },
        }),
      );

      const plainOptions = (overrides = {}) => ({ ...limits, ...overrides });
      const singleSpectrumOptions = (overrides = {}) => ({
        ...limits, spectrum: { ...makeSpectrum(), ...overrides }, spectrumCollection: null,
      });
      const collectionDomainOptions = (overrides = {}) => ({
        ...limits,
        spectrum: null,
        spectrumCollection: { entries: [makeEntry()], maximumCaptureBytes: 4096, ...overrides },
      });
      const domainRefusals = [
        ["u32 type", plainOptions({ sourceRingFrames: "256" })],
        ["u32 bound", plainOptions({ sourceRingFrames: 0x1_0000_0000 })],
        ["u64 type", plainOptions({ maximumMemoryBytes: 0 })],
        ["u64 bound", plainOptions({ maximumMemoryBytes: 0x1_0000_0000_0000_0000n })],
        ["reserved1 nonzero", plainOptions({ reserved1: 1n })],
        ["reserved1 type", plainOptions({ reserved1: 0 })],
        ["observation taps bound", plainOptions({ liveControlObservationTaps: 17n })],
        // Issue #1689 gate 2: the retired master word is refused at both ends of its range.
        ["reserved2 one", plainOptions({ reserved2: 1n })],
        ["reserved2 all ones", plainOptions({ reserved2: 0xffff_ffff_ffff_ffffn })],
        ["spectrum hop unsupported", plainOptions({
          spectrumHopFrames: 300, spectrum: null, spectrumCollection: null,
        })],
        ["invalid spectrum target", singleSpectrumOptions({ target: "unknown" })],
        ["invalid spectrum channel", singleSpectrumOptions({ channels: "center" })],
        ["empty spectrum ID", singleSpectrumOptions({ targetId: "" })],
        ["oversize UTF-8 spectrum ID", singleSpectrumOptions({ targetId: "é".repeat(64) })],
        ["single capture zero", singleSpectrumOptions({ maximumCaptureBytes: 0 })],
        ["single capture bound", singleSpectrumOptions({ maximumCaptureBytes: 1_048_577 })],
        ["collection capture zero", collectionDomainOptions({ maximumCaptureBytes: 0 })],
      ];
      for (const [label, options] of domainRefusals) {
        await assertRefusal(label, makeFactory({ options }));
      }
      await assertAccepted(
        "collection capture above single limit",
        makeFactory({ options: collectionDomainOptions({ maximumCaptureBytes: 1_048_577 }) }),
      );
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
        const spectrum = {
          target: "trackPostPan", targetId: "single", channels: "left", maximumCaptureBytes: 4096,
        };
        const options = {
          sourceRingFrames: 0,
          maximumMemoryBytes: 0n,
          reserved1: 0n,
          liveControlMeterBlocks: 7n,
          liveControlObservationTaps: 0n,
          reserved2: 0n,
          spectrumHopFrames: 256,
          spectrum,
          spectrumCollection: null,
        };
        const expectedOptions = { ...options, spectrum: { ...spectrum } };
        const factory = makeFactory({ options, workletModuleUrl: "single-spectrum.js" });
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
          spectrum.targetId = "mutated-spectrum-before-construction";
          options.spectrumHopFrames = 1024;
          options.sourceRingFrames = 128;
          options.liveControlMeterBlocks = 99n;
          factory.options = { ...options, spectrum: { ...spectrum, target: "output" } };
          factory.context = replacementContext;
          factory.preparedModule = replacementModule;
          factory.simd128ModuleUrl = "mutated-simd.wasm";
          factory.workletModuleUrl = "mutated-processor-before-construction.js";
          addGate.release();
          await readyPause.started;
          context.state = "running";
          context.sampleRate = 44100;
          context.renderQuantumSize = 128;
          spectrum.targetId = "mutated-spectrum-while-ready";
          options.spectrumHopFrames = 1024;
          options.sourceRingFrames = 256;
          options.liveControlMeterBlocks = 101n;
          factory.options = { ...options, spectrum: { ...spectrum, target: "input" } };
          readyPause.release();
          hostValue = await boot;
          const node = FakeNode.latest;
          const captured = node.options.processorOptions.options;
          assert.equal(hostValue.resources.sampleRateHz, 48000);
          assert.equal(hostValue.resources.quantumFrames, 64);
          assert.deepEqual(captured, expectedOptions, "single spectrum boot values are captured");
          assert.equal(captured.spectrumCollection, null);
          assert.notEqual(captured, options, "single spectrum boot snapshot is fresh");
          assert.notEqual(captured.spectrum, spectrum, "single spectrum snapshot is fresh");
          assert.deepEqual(
            events.slice(-2),
            [["compile", "simd.wasm"], ["addModule", "single-spectrum.js"]],
            "single spectrum loading inputs are captured",
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
          const heldSources = Array.from({ length: 77 }, (_value, index) => {
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
            sourceId: "single-source", generation: 1n, startFrame: 77n,
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
            sourceId: "single-source", generation: 1n, startFrame: 78n,
            sampleRateHz: 48000, planes: [new Float32Array(nextSourceBuffer)], frames: 1,
            endOfRegion: false,
          });
          const nextSourceAck = await nextSource;
          assert.equal(nextSourceAck.requestId, sourceAcks.at(-1).requestId + 1,
            "source capacity refusal did not burn an ID");

          // Issue #1565: the in-flight bound is the fixed `maximumCommandRecords` (256), not a boot
          // word, so 256 commands hold and the 257th is refused locally.
          const localCommand = {
            kind: 7, rack: 1, stripId: "track-0", effectId: "comp", tapId: 1, windowBlocks: 0,
          };
          holdAll = true;
          const heldCommands = Array.from({ length: 256 }, () =>
            hostValue.command({ commands: [localCommand] }));
          const beforeCommandOverflow = events.length;
          await localErrorResult(hostValue.command({ commands: [localCommand] }), 6);
          assert.equal(events.length, beforeCommandOverflow, "command capacity refusal posted no event");
          holdAll = false;
          for (const respond of heldAll.splice(0)) respond();
          const commandAcks = await Promise.all(heldCommands);
          const nextCommand = await hostValue.command({ commands: [localCommand] });
          assert.equal(nextCommand.requestId, commandAcks.at(-1).requestId + 1,
            "command capacity refusal did not burn an ID");
          const observeAck = await hostValue.observe({
            subscriptions: [{
              stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 0, armed: true,
            }],
          });
          assert.equal(observeAck.result, 0, "captured observation default command accepted");
          assert.equal(observeAck.bindings[0].windowBlocks, 7,
            "windowBlocks: 0 uses captured liveControlMeterBlocks");
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
    await pipelineHost.dispose();

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

    // Issue #1565: the only kinds `command()` carries are the two observation kinds.
    const sub = {
      kind: 7, rack: 1, stripId: "track-1", effectId: "comp", tapId: 1, windowBlocks: 64,
    };
    // Issue #393: one real host fixture owns every request ID across interleaved request classes.
    // Twenty-five rounds cover 250 successful calls, including both lease orderings around a
    // command and direct source/seek traffic. No caller-side ID is supplied anywhere in this loop.
    const mixedAcks = [];
    const mixedSendStart = events.length;
    const mixedSubscription = {
      stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 1, armed: true,
    };
    for (let round = 0; round < 25; round += 1) {
      mixedAcks.push((await liveControlHost.command({ commands: [sub] })).requestId);
      mixedAcks.push((await liveControlHost.observe({ subscriptions: [mixedSubscription] })).requestId);
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
    assert.equal(mixedAcks.length, 250, "mixed host regression covers 250 successful calls");
    const mixedSendIds = events.slice(mixedSendStart).map((event) => event[2]);
    assert.deepEqual(mixedAcks, mixedSendIds, "acknowledgements match actual outbound send order");
    assert.equal(mixedAcks[0], 2, "sessionMap consumes the first host allocation");
    assert.equal(mixedAcks.at(-1), 251, "250 mixed calls occupy IDs 2 through 251");
    assert(mixedAcks.every((id, index) => index === 0 || id === mixedAcks[index - 1] + 1),
      "mixed acknowledgements have adjacent IDs");
    assert(mixedAcks.every((_id, index) => index % 10 !== 1 || mixedAcks[index] === mixedAcks[index - 1] + 1),
      "observe delegates with exactly one allocation");
    assert.equal(new Set(mixedAcks).size, mixedAcks.length, "mixed acknowledgements are unique");
    console.log("Issue393 mixed calls: 250 result-zero, send IDs 2..251, adjacent, observe single allocation");
    // The shipped SDK consumer uses the same real host: sessionMap -> direct meter lease ->
    // semantic live-control submit. This is the collision reproducer that the host-owned ledger
    // fixes.
    const { createBrowserLiveControls } = await import(new URL("./sdk/src/browser/live-controls.ts", root));
    // The strip order is the core's control list (#1564 D5): the fake worklet does not run a core,
    // so the list is scripted; every other call still goes through the real host.
    const { scriptedControl } = await import(new URL("./sdk/test/control-support.mjs", root));
    const browserLiveControls = await createBrowserLiveControls({
      ...liveControlHost,
      sessionMap: () => liveControlHost.sessionMap(),
      meters: (request) => liveControlHost.meters(request),
      command: (request) => liveControlHost.command(request),
      control: scriptedControl({ strips: { tracks: ["kick", "snare"], submixes: ["aaa-bus", "zzz-bus"] } }),
    });
    await liveControlHost.meters({ enabled: true, onFrame: null });
    // Issue #1565: the host's own command path carries observation records only, so the SDK edit
    // that rides it is an observation subscription; a value edit goes through `control()`.
    const sdkReport = await browserLiveControls.submit(
      browserLiveControls.edit.track("kick").console("comp", "miso.compressor")
        .observe("Gain Reduction", true, 1),
    );
    assert.equal(sdkReport.ok, true, "SDK live controls submit through the real host after direct meters");
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
    const commandAck = await liveControlHost.command({ commands: [sub] });
    assert.equal(commandAck.tag, "miso.ack.v1");
    assert.equal(commandAck.result, 0);
    assert.equal(commandAck.admitted, 1);
    assert.equal(commandAck.appliedAtSample, 512n, "the ack names the exact application sample");
    const staged = events.findLast((event) => event[1] === "miso.command.v1");
    assert.equal(staged[3], 1, "the record block is transferred, never copied");
    assert.equal(commandAck.records.byteLength, 272, "the block comes straight back to the caller");
    const decoded = new DataView(
      commandAck.records.buffer,
      commandAck.records.byteOffset,
      commandAck.records.byteLength,
    );
    assert.equal(decoded.getUint8(0), 7, "kind");
    assert.equal(decoded.getUint8(1), 1, "rack");
    assert.equal(decoded.getUint8(2), 7, "strip ID length");
    assert.equal(decoded.getUint8(3), 4, "effect ID length");
    assert.equal(decoded.getUint32(4, true), 1, "tap id");
    assert.equal(decoded.getUint32(8, true), 64, "window blocks");
    const idText = (offset, length) => new TextDecoder().decode(
      new Uint8Array(commandAck.records.buffer, commandAck.records.byteOffset + offset, length));
    assert.equal(idText(16, 7), "track-1", "strip ID");
    assert.equal(idText(16 + 128, 4), "comp", "effect ID");
    assert.equal(decoded.getUint8(16 + 7), 0, "the strip ID field is zero padded");

    // A malformed command never reaches the port.
    const beforeMalformed = events.length;
    await errorResult(
      liveControlHost.command({ commands: [{ ...sub, kind: 99 }] }),
      1,
    );
    // Issue #1565: every deleted value kind is an unknown kind to the host's own gate.
    for (const kind of [1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 13, 14, 15, 16, 17]) {
      await errorResult(liveControlHost.command({ commands: [{ ...sub, kind }] }), 1);
    }
    // An ID that does not fit its 127-byte field, or is not a string, never reaches the port.
    for (const broken of [
      { stripId: "x".repeat(128) }, { effectId: "é".repeat(64) }, { stripId: 3 }, { effectId: null },
      { tapId: -1 }, { windowBlocks: 1.5 }, { channel: 255 }, { trackIndex: 0 },
    ]) {
      await errorResult(liveControlHost.command({ commands: [{ ...sub, ...broken }] }), 1);
    }
    await errorResult(
      liveControlHost.command({ commands: [{ kind: 7, rack: 1, stripId: "a", effectId: "b", tapId: 1 }] }),
      1,
    );
    await errorResult(liveControlHost.command({ commands: [] }), 1);
    // The retired `simd1`/`simd2` rack codes and an unallocated one never reach the port (#1096).
    for (const rack of [0, 2, 4, 255]) {
      await errorResult(
        liveControlHost.command({ commands: [{ ...sub, rack }] }),
        1,
      );
    }
    await localErrorResult(liveControlHost.command({ requestId: 999, commands: [sub] }), 1);
    await localErrorResult(liveControlHost.observe({ requestId: 999, subscriptions: [{
      stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 1, armed: true,
    }] }), 1);
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

    // Engine backpressure is a resolved acknowledgement that admits nothing.
    commandResult = 6;
    const refused = await liveControlHost.command({ commands: [sub] });
    assert.equal(refused.result, 6);
    assert.equal(refused.admitted, 0);
    assert.equal(refused.reason, 8);
    commandResult = 0;

    // Local backpressure: the in-flight bound is `maximumCommandRecords` (256, issue #1565), so a
    // 257th unsettled batch is refused here, before any transfer, and the caller keeps its records.
    holdAll = true;
    const commandEventsBefore = events.length;
    const held4 = Array.from({ length: 256 }, () => liveControlHost.command({ commands: [sub] }));
    await localErrorResult(liveControlHost.command({ commands: [sub] }), 6);
    await localErrorResult(liveControlHost.observe({ subscriptions: [mixedSubscription] }), 6);
    assert.equal(events.length, commandEventsBefore + 256, "command/observe saturation posts no refusal");
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    const heldCommandAcks = await Promise.all(held4);
    const commandAfterBound = await liveControlHost.command({ commands: [sub] });
    assert.equal(
      commandAfterBound.requestId,
      Math.max(...heldCommandAcks.map((ack) => ack.requestId)) + 1,
      "command saturation does not burn an ID",
    );

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

    // Issue #143: the `miso.observe.v1` acknowledgement's subscription map.
    //
    // The map is the answer to what `trackGrDb` cannot say -- which tracks have an observed effect
    // at all -- so it is checked as carefully as the frame: the exact wire kinds go out, the map
    // is canonically ordered, `windowBlocks: 0` resolves to the plan default, and an unsubscribe
    // removes exactly one entry.
    const observeAck = await liveControlHost.observe({
      subscriptions: [
        { stripId: "track-1", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 0, armed: true },
        { stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 8, armed: true },
      ],
    });
    assert.equal(observeAck.tag, "miso.observe.v1");
    assert.equal(observeAck.result, 0);
    assert.deepEqual(observeAck.bindings.map((binding) => binding.stripId), ["track-0", "track-1"],
      "the map is canonically ordered by (stripId, rack, effectId, tapId)");
    assert.deepEqual(observeAck.bindings[0], {
      stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 8,
    });
    assert.equal(observeAck.bindings[1].windowBlocks, Number(limits.liveControlMeterBlocks),
      "`windowBlocks: 0` resolves to the plan default, and the map says which one it got");
    assert.equal(Object.isFrozen(observeAck.bindings), true);
    const unsubscribed = await liveControlHost.observe({
      subscriptions: [
        { stripId: "track-1", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 0, armed: false },
      ],
    });
    assert.equal(unsubscribed.bindings.length, 1, "an unsubscribe removes exactly one entry");
    assert.equal(unsubscribed.bindings[0].stripId, "track-0");
    // Issue #1213 (attempt 2, verdict MAJOR-1), as #1677 leaves it: a bus tap is named by its
    // submix's ID like a track's, and the binding carries no position of any plan.
    {
      const busSubscription = {
        stripId: "bus-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 0,
      };
      const busAck = await liveControlHost.observe({
        subscriptions: [{ ...busSubscription, armed: true }],
      });
      assert.equal(busAck.result, 0);
      const busBinding = busAck.bindings.find((binding) => binding.stripId === "bus-0");
      assert.deepEqual(busBinding, { ...busSubscription, windowBlocks: Number(limits.liveControlMeterBlocks) });
      assert.deepEqual(Object.keys(busBinding).sort(), ["effectId", "rack", "stripId", "tapId", "windowBlocks"]);
      const busOff = await liveControlHost.observe({
        subscriptions: [{ ...busSubscription, armed: false }],
      });
      assert.deepEqual(busOff.bindings, unsubscribed.bindings, "the bus binding disarms");
    }
    for (const broken of [
      // The retired `simd1`/`simd2` codes (`0`, `2`) and an unallocated one are refused (#1096).
      { stripId: 0 }, { stripId: "x".repeat(128) }, { effectId: "x".repeat(128) }, { rack: 0 },
      { rack: 2 }, { rack: 4 }, { tapId: 0 }, { armed: "yes" }, { windowBlocks: -1 },
      { trackIndex: 0 }, { effectIndex: 0 },
    ]) {
      await errorResult(liveControlHost.observe({
        subscriptions: [{
          stripId: "track-0", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 0, armed: true,
          ...broken,
        }],
      }), 1);
    }
    await errorResult(liveControlHost.observe({ subscriptions: [] }), 1);

    // Issue #143's two reasons, and the #151 defect they exposed: a refused subscription is a
    // typed *per-request* rejection and costs the host nothing.
    //
    // Reasons 10 (`unknownTap`) and 11 (`observationUnbound`) are the only two the observation
    // path ever returns, and `#receive` used to bound `reason` at the literal `<= 9`. Either one
    // therefore read as a malformed acknowledgement and tripped the sticky 255 that fails every
    // unsettled request and every later one -- so one refused subscription cost the whole
    // session. That is what kept the app's gain-reduction meters dead: the app arms its taps
    // once at startup, and a single refusal took the live controls with it.
    //
    // Red mutation: restore `validU32(message.reason) && message.reason <= 9` in `#receive` ->
    // every assertion below fails with the sticky signature, starting with `refused.tag` because
    // the promise rejects with `{tag: "miso.error.v1", result: 255}` instead of settling.
    // `scripts/test-web-audioworklet.sh` runs exactly that mutation and requires this file red.
    const mapBeforeRefusal = unsubscribed.bindings;
    for (const { reason, result, what } of [
      // The address resolves and the tap id does not. A bad address, like every other unknown, so
      // `RESULT_INVALID_ARGUMENT`.
      { reason: 10, result: 1, what: "an undeclared tap" },
      // The tap is declared and correctly addressed; this preparation bound no observation
      // capacity, which is what `RESULT_UNSUPPORTED` means. Retrying will never help.
      { reason: 11, result: 7, what: "a session that bound no observation capacity" },
      // Issue #1676 (#1685: by exact bytes): past what one ended event holds. The value follows the retired 12-14, so a table
      // bound that stopped at 12 or accepted a retired hole would read it as malformed.
      { reason: 15, result: 7, what: "a subscription past what one ended event holds" },
    ]) {
      commandResult = result;
      commandMutation = (response) => ({ ...response, reason, rejectedIndex: 0, admitted: 0 });
      const refused = await liveControlHost.observe({
        subscriptions: [
          { stripId: "track-0", rack: 1, effectId: "comp", tapId: 9, windowBlocks: 0, armed: true },
        ],
      });
      commandMutation = null;
      commandResult = 0;
      assert.equal(refused.tag, "miso.observe.v1", `${what} settles as a typed observation ack`);
      assert.equal(refused.result, result, `${what} carries its own result code`);
      assert.equal(refused.reason, reason, "the ack names which namespace the caller got wrong");
      assert.equal(Object.isFrozen(refused), true);
      assert.deepEqual(
        refused.bindings,
        mapBeforeRefusal,
        "a batch is all-or-nothing: a refused batch arms nothing and leaves the map untouched",
      );

      // The host is fully healthy afterwards. Each of these is a request the sticky error would
      // have failed with `{tag: "miso.error.v1", result: 255}`.
      assert.equal((await liveControlHost.status()).result, 0, `${what}: status still answers`);
      const laterCommand = await liveControlHost.command({
        commands: [sub],
      });
      assert.equal(laterCommand.result, 0, `${what}: the command path still admits a batch`);
      assert.equal(laterCommand.admitted, 1);
      assert.deepEqual(
        (await liveControlHost.sessionMap()).sources.map((source) => source.id),
        ["bass", "drums"],
        `${what}: the session map still answers`,
      );
      const framesBefore = meterFrames.length;
      node.port.onmessage({
        data: {
          tag: "miso.meter.v1", sequence: 10 + reason, generation: 1n, validity: 0xb,
          lossCount: 0, windows: 1, trackCount: 2, structureEpoch: 1,
          peaks: new Float32Array([0.5, 0.5, 0.5, 0.5, 0.5, 0.5]),
          trackGrDb: new Float32Array([3.25, 0]), masterGrDb: 3.25,
          firstSample: 1024n, endSample: 1280n,
          submixCount: 0, submixPeaks: new Float32Array(0), submixGrDb: new Float32Array(0),
        },
      });
      assert.equal(
        meterFrames.length,
        framesBefore + 1,
        `${what}: the meter lease still delivers -- this is the dead-GR-meter regression`,
      );
      assert.equal(meterFrames.at(-1).trackGrDb[0], 3.25);

      // And a *correct* subscription still arms, so nothing about the map machinery was poisoned.
      const recovered = await liveControlHost.observe({
        subscriptions: [
          { stripId: "track-1", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 4, armed: true },
        ],
      });
      assert.equal(recovered.result, 0, `${what}: a correct subscription still arms after it`);
      assert.equal(recovered.reason, 0);
      assert.deepEqual(recovered.bindings.map((binding) => binding.stripId), ["track-0", "track-1"]);
      const undo = await liveControlHost.observe({
        subscriptions: [
          { stripId: "track-1", rack: 1, effectId: "comp", tapId: 1, windowBlocks: 4, armed: false },
        ],
      });
      assert.deepEqual(undo.bindings, mapBeforeRefusal, "the map returns to where it started");
    }

    // Issue #1565: reasons 12-14 (`notSoloable`, `unknownRoute`, `unknownVca`) left the
    // vocabulary with the value kinds. An acknowledgement that still carries one is a malformed
    // reply and fails the whole host, as any reason outside the table does.
    for (const reason of [12, 13, 14]) {
      const reasonHost = await createMisoAudioWorkletHost({
        context,
        document: new Uint8Array(),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      commandResult = 1;
      commandMutation = (response) => ({ ...response, reason, rejectedIndex: 0, admitted: 0 });
      await errorResult(reasonHost.command({ commands: [sub] }), 255).catch((error) => {
        throw new Error(`the host accepted deleted command reason ${reason}`, { cause: error });
      });
      commandMutation = null;
      commandResult = 0;
      await reasonHost.dispose();
    }

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
    await localErrorResult(liveControlHost.command({ commands: [sub] }), 3);
    await localErrorResult(liveControlHost.observe({ subscriptions: [mixedSubscription] }), 3);
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

    // Issue #143 D7 / #151: recompile and re-subscribe, the way the app re-arms after the plan is
    // replaced.
    //
    // A structural session edit produces a replacement plan, and in the browser that is a new host
    // over the new session JSON. Subscriptions belong to the plan they were applied to: the
    // replacement's lanes exist and are unarmed, nothing carries over, and the app re-arms from
    // scratch against the new addressing. This is also the moment the #151 defect was most likely
    // to fire in the field, because a replacement moves effect indices -- a stale tap address
    // comes back as reason 10, and before the fix that killed the freshly prepared host on its
    // first gesture.
    const prepare = (sessionDocument) => createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode(sessionDocument),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const tap = (stripId, effectId, armed) => ({
      stripId, rack: 1, effectId, tapId: 1, windowBlocks: 0, armed,
    });

    const beforeEdit = await prepare("{\"schema_version\":0}");
    const armedBefore = await beforeEdit.observe({
      subscriptions: [tap("track-0", "comp", true), tap("track-1", "comp", true)],
    });
    assert.equal(armedBefore.result, 0);
    assert.deepEqual(armedBefore.bindings.map((binding) => binding.stripId), ["track-0", "track-1"]);
    await beforeEdit.dispose();

    const afterEdit = await prepare("format_version = 0 # one effect inserted");
    // The replacement's map starts empty: request ids restart at 1 and nothing carried over.
    // The app's first re-arm names the *old* effect, which the replacement no longer has.
    commandResult = 1;
    commandMutation = (response) => ({ ...response, reason: 10, rejectedIndex: 0, admitted: 0 });
    const staleRearm = await afterEdit.observe({
      subscriptions: [tap("track-0", "comp", true), tap("track-1", "comp", true)],
    });
    commandMutation = null;
    commandResult = 0;
    assert.equal(staleRearm.tag, "miso.observe.v1");
    assert.equal(staleRearm.reason, 10, "a stale tap key is refused, per request");
    assert.deepEqual(staleRearm.bindings, [], "the replacement plan's map is still empty");

    // The app names the replacement's effects and re-arms. The host was never sticky, so this is a
    // plain second call rather than a rebuild.
    assert.equal((await afterEdit.sessionMap()).result, 0, "the session map still answers");
    const rearmed = await afterEdit.observe({
      subscriptions: [tap("track-0", "comp-2", true), tap("track-1", "comp-2", true)],
    });
    assert.equal(rearmed.result, 0, "the replacement plan re-arms after the refusal");
    assert.deepEqual(rearmed.bindings.map((binding) => binding.effectId), ["comp-2", "comp-2"],
      "the map holds the replacement's keys, not the retired plan's");
    assert.deepEqual(rearmed.bindings.map((binding) => binding.windowBlocks),
      [Number(limits.liveControlMeterBlocks), Number(limits.liveControlMeterBlocks)],
      "`windowBlocks: 0` resolves against the replacement's own default");

    // The rest of the live-control surface is live on the replacement.
    const rearmedFrames = [];
    assert.equal(
      (await afterEdit.meters({
        enabled: true, onFrame: (frame) => rearmedFrames.push(frame),
      })).result,
      0,
    );
    FakeNode.latest.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
        windows: 1, trackCount: 2, structureEpoch: 1,
        peaks: new Float32Array(6), trackGrDb: new Float32Array([1.5, 2.5]), masterGrDb: 2.5,
        firstSample: 0n, endSample: 256n,
        submixCount: 0, submixPeaks: new Float32Array(0), submixGrDb: new Float32Array(0),
      },
    });
    assert.equal(rearmedFrames.length, 1, "the replacement's meter sequence restarts at 1");
    assert.deepEqual([...rearmedFrames[0].trackGrDb], [1.5, 2.5]);
    assert.equal((await afterEdit.command({ commands: [sub] })).result, 0);
    assert.equal((await afterEdit.status()).result, 0);
    await afterEdit.dispose();
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
      await tickHost.dispose();
      assert.equal(timer.cleared, true, "dispose stops the tick");
      const afterDispose = events.length;
      timer.callback();
      assert.equal(events.length, afterDispose, "a tick after dispose posts nothing");
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

// `submixIds` (#1209 MINOR-2, #1210 MINOR-1): the session's submix IDs in canonical order; the
// meter header's `submix_count` and the frame layout follow it, so a caller can run the worklet
// with no submix (the production case before K2) or with several (their order is observable).
function createFakeExports(quantum, backend = 1, liveControlsAttached = true, submixIds = ["drums-bus"]) {
  // Three pages: the 272-byte command records stage 69,632 bytes, beyond the first page.
  const memory = { buffer: new ArrayBuffer(196608) };
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
    sourceResult: 0, bootResult: 0, boot: [], bootWithSpectrumHop: [], spectrumHopCapability: 0,
    spectrumHopCapabilityResult: 1,
    // Issue #1381a: the service step's handles and result, and the control counter's value.
    service: [], serviceResult: 0, controlAllocations: 5,
    // Issue #1562a D1: the control frames the fake received, the result it answers with, the
    // response it leaves, the committed revision, and whether it grows its memory under a command.
    control: [], controlResult: 0, controlResponse: new Uint8Array([9, 8, 7]), controlRevision: 4n,
    controlGrowsMemory: false, controlRequestRefusals: 0, growths: 0,
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
  };
  const watermarkPointer = 66000;
  // What `control_response_*` reads now: the last control command's response, or the event frame
  // the last `control_event` dequeued (empty for an empty lane), as the real exports share their
  // staging (issue #1570b D1).
  let currentEvent = null;
  const controlRequestPointer = 50000;
  const controlResponsePointer = 56000;
  // Issue #137: command staging (kind 6), the meter frame (kind 7) and the command report.
  const commandPointer = 70000;
  const meterFramePointer = 40000;
  const reportPointer = 41000;
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
  let report = new DataView(memory.buffer, reportPointer, 48);
  report.setUint32(0, 48, true);
  report.setUint32(4, 0x00010000, true);
  const pointers = {
    2: 4096, 3: 5000, 5: 8192, 6: liveControlsAttached ? commandPointer : 0, 7: meterFramePointer,
  };
  const capacities = {
    2: 64, 3: 2 * quantum * 4, 5: 2 * quantum * 4,
    6: liveControlsAttached ? 256 * 272 : 0,
    7: meterFrameFloats * 4,
  };
  // Issue #143: the additive observation ABI is present even in this hermetic processor fake.
  // This fixture does not model a compiled observation row, but the worklet still prewarms and
  // discovers the fixed staging views before it can publish `miso.ready.v1`.
  const observationIdPointer = 17500;
  const observationSelectionPointer = 18000;
  const observationResultPointer = 19000;
  const observationSelectionCapacity = 4;
  // Issue #779: prewarmed fixed live-response request, identity and raw-snapshot staging. The
  // fake refuses capture because it has no prepared graph, but it must expose the complete ABI so
  // construction tests exercise the same prewarm path as the shipped Worklet.
  const trackResponseRequestPointer = 20000;
  const trackResponseTrackIdPointer = 20100;
  const trackResponseSnapshotPointer = 22000;
  const trackResponseSnapshotCapacity = 30000;
  calls.commands = [];
  calls.commandResult = 0;
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
  // A real linear-memory growth: the old buffer is detached in a browser, the new one holds the
  // same bytes at the same addresses, and the fake's own views follow it.
  const growMemory = () => {
    const grown = new ArrayBuffer(memory.buffer.byteLength * 2);
    new Uint8Array(grown).set(new Uint8Array(memory.buffer));
    memory.buffer = grown;
    status = new DataView(grown, statusPointer, 80);
    report = new DataView(grown, reportPointer, 48);
    calls.growths += 1;
  };
  const exports = {
    memory,
    miso_engine_web_v1_abi_version: () => 0x00010000,
    miso_engine_web_v1_boot_options_ptr: () => 512,
    miso_engine_web_v1_document_ptr: () => 2048,
    miso_engine_web_v1_boot: (...args) => { calls.boot.push(args); return boot(); },
    miso_engine_web_v1_spectrum_hop_capability: () => {
      calls.spectrumHopCapability += 1;
      return calls.spectrumHopCapabilityResult;
    },
    miso_engine_web_v1_boot_with_spectrum_hop: (...args) => {
      calls.bootWithSpectrumHop.push(args);
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
    miso_engine_web_v1_source_seek: (...args) => {
      calls.seek.push(args);
      calls.seekIdBytes.push(new Uint8Array(memory.buffer, pointers[2], args[1]).slice());
      return 0;
    },
    miso_engine_web_v1_command_report_ptr: () => reportPointer,
    miso_engine_web_v1_command_submit: (_handle, count) => {
      calls.commands.push(new Uint8Array(memory.buffer, commandPointer, count * 272).slice());
      report.setUint32(8, calls.commandResult, true);
      report.setUint32(12, calls.commandResult === 0 ? 0 : 8, true);
      report.setUint32(16, 0, true);
      report.setUint32(20, calls.commandResult === 0 ? count : 0, true);
      report.setBigUint64(24, status.getBigUint64(32, true), true);
      return calls.commandResult;
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
    miso_engine_web_v1_source_id: (_handle, index) => {
      const id = sourceRows[index]?.id;
      if (id === undefined) return 0;
      const bytes = new Uint8Array(memory.buffer, pointers[2], id.length);
      for (let byte = 0; byte < id.length; byte += 1) bytes[byte] = id.charCodeAt(byte);
      return id.length;
    },
    miso_engine_web_v1_source_channels: (_handle, index) => sourceRows[index]?.channels ?? 0,
    miso_engine_web_v1_source_frames: (_handle, index) => sourceRows[index]?.frames ?? 0n,
    miso_engine_web_v1_observation_count: () => 0,
    miso_engine_web_v1_observation_id_ptr: () => observationIdPointer,
    miso_engine_web_v1_observation_id_capacity: () => 128,
    miso_engine_web_v1_observation_track_index: () => 0,
    miso_engine_web_v1_observation_rack: () => 1,
    miso_engine_web_v1_observation_effect_index: () => 0,
    miso_engine_web_v1_observation_effect_slot_id: () => 0,
    miso_engine_web_v1_observation_native_effect_id: () => 0,
    miso_engine_web_v1_observation_tap_count: () => 0,
    miso_engine_web_v1_observation_tap_id: () => 0,
    miso_engine_web_v1_observation_selection_ptr: () => observationSelectionPointer,
    miso_engine_web_v1_observation_selection_bytes: () => 32,
    miso_engine_web_v1_observation_selection_capacity: () => observationSelectionCapacity,
    miso_engine_web_v1_observation_read: () => 0,
    miso_engine_web_v1_observation_result_ptr: () => observationResultPointer,
    miso_engine_web_v1_observation_result_bytes: (_handle, count = 0) => count * 96,
    miso_engine_web_v1_track_response_request_ptr: () => trackResponseRequestPointer,
    miso_engine_web_v1_track_response_request_bytes: () => 48,
    miso_engine_web_v1_track_response_track_id_ptr: () => trackResponseTrackIdPointer,
    miso_engine_web_v1_track_response_track_id_capacity: () => 127,
    miso_engine_web_v1_track_response_snapshot_ptr: () => trackResponseSnapshotPointer,
    miso_engine_web_v1_track_response_snapshot_capacity: () => trackResponseSnapshotCapacity,
    miso_engine_web_v1_track_response_snapshot_set_bytes: (bytes) => (
      bytes >= 104 && bytes <= trackResponseSnapshotCapacity ? 0 : 1
    ),
    miso_engine_web_v1_track_response_capture: () => 7,
    miso_engine_web_v1_track_response_analysis: () => 7,
    miso_engine_web_v1_track_response_result_ptr: () => trackResponseSnapshotPointer,
    miso_engine_web_v1_track_response_result_bytes: () => 0,
    miso_engine_web_v1_track_response_close: () => 0,
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
    miso_engine_web_v1_control_command: (handle, length) => {
      calls.control.push({
        handle,
        frame: new Uint8Array(memory.buffer, controlRequestPointer, length).slice(),
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

  // Issue #1477 D1: the staging-read caller boots a spectrum collection, so the fake carries the
  // pre-boot collection staging exports with the bridge's sizing rules (`hosts/host-web/src/ffi.rs`):
  // the entry capacity is the request header's entry count, and the identity capacity is the sum
  // of the staged entries' identity byte lengths.
  //
  // Issue #1491 D1: the fake's boot exports read the staged collection back the way the bridge
  // lays it out (`staged_spectrum_request`) and record it as the boot's staging witness, so a
  // worklet that skips the staging, or stages bytes that differ from its options, is visible
  // before boot. The witness is `null` when the request pointer was never asked for or the
  // request header's entry count is 0; the boot result is unchanged.
  //
  // Every offset, size, field type, entry stride and wire code here comes from the generated
  // layout (`sdk/assets/miso-engine-v1-abi-layout.json`, made from the Rust structs by
  // `parameter-metadata`), never from the worklet's hand-typed copy, so a bridge layout change
  // the worklet misses is red here. The identity decoder keeps a leading U+FEFF, as the bridge's
  // `core::str::from_utf8` does.
  const identityDecoder = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });
  const collectionRequestLayout = abiLayout.structures.spectrumCollectionRequest;
  const collectionEntryLayout = abiLayout.structures.spectrumCollectionEntry;
  const layoutField = (layout, name) => {
    const field = layout.fields.find((candidate) => candidate.name === name);
    assert.ok(field, `qualification boot contract: the generated layout has no field ${name}`);
    return field;
  };
  const readLayoutField = (bytes, base, { type }, offset) => {
    if (type === "u32") return bytes.getUint32(base + offset, true);
    if (type === "u64") return bytes.getBigUint64(base + offset, true);
    const array = /^u32\[(\d+)\]$/.exec(type);
    assert.ok(array, `qualification boot contract: unknown generated field type ${type}`);
    return Array.from({ length: Number(array[1]) },
      (_, index) => bytes.getUint32(base + offset + 4 * index, true));
  };
  // Decodes every field of one generated structure at `base`, keyed by its generated name.
  const readLayoutStruct = (bytes, base, layout) => Object.fromEntries(
    layout.fields.map((field) => [field.name, readLayoutField(bytes, base, field, field.offset)]));
  const requestEntryCountOffset = layoutField(collectionRequestLayout, "entryCount").offset;
  const entryTargetIdBytesOffset = layoutField(collectionEntryLayout, "targetIdBytes").offset;
  const withSpectrumCollectionStaging = (fake) => {
    const requestPointer = 42000;
    const entryPointer = 42100;
    const idPointer = 42600;
    let requested = false;
    let entryCapacity = 0;
    let idCapacity = 0;
    const view = () => new DataView(fake.exports.memory.buffer);
    const decodeStagedCollection = () => {
      if (!requested) return null;
      const bytes = view();
      const request = readLayoutStruct(bytes, requestPointer, collectionRequestLayout);
      if (request.entryCount === 0) return null;
      const entries = [];
      let idOffset = idPointer;
      for (let index = 0; index < request.entryCount; index += 1) {
        const entry = readLayoutStruct(
          bytes, entryPointer + index * collectionEntryLayout.bytes, collectionEntryLayout);
        entry.targetId = identityDecoder.decode(
          new Uint8Array(fake.exports.memory.buffer, idOffset, entry.targetIdBytes).slice(),
        );
        entries.push(entry);
        idOffset += entry.targetIdBytes;
      }
      return { request, entries };
    };
    fake.spectrumStagingWitnesses = [];
    for (const name of ["miso_engine_web_v1_boot", "miso_engine_web_v1_boot_with_spectrum_hop"]) {
      const boot = fake.exports[name];
      if (typeof boot !== "function") continue;
      fake.exports[name] = (...args) => {
        fake.spectrumStagingWitnesses.push(decodeStagedCollection());
        return boot(...args);
      };
    }
    Object.assign(fake.exports, {
      miso_engine_web_v1_spectrum_collection_request_ptr: () => {
        requested = true;
        entryCapacity = 0;
        idCapacity = 0;
        return requestPointer;
      },
      miso_engine_web_v1_spectrum_collection_request_bytes: () => collectionRequestLayout.bytes,
      miso_engine_web_v1_spectrum_collection_entry_ptr: () => {
        entryCapacity = view().getUint32(requestPointer + requestEntryCountOffset, true);
        return entryCapacity * collectionEntryLayout.bytes <= idPointer - entryPointer
          ? entryPointer : 0;
      },
      miso_engine_web_v1_spectrum_collection_entry_capacity: () => entryCapacity,
      miso_engine_web_v1_spectrum_collection_entry_bytes: () => collectionEntryLayout.bytes,
      miso_engine_web_v1_spectrum_collection_target_ids_ptr: () => {
        idCapacity = 0;
        for (let index = 0; index < entryCapacity; index += 1) {
          idCapacity += view().getUint32(
            entryPointer + index * collectionEntryLayout.bytes + entryTargetIdBytesOffset, true);
        }
        return idPointer;
      },
      miso_engine_web_v1_spectrum_collection_target_ids_capacity: () => idCapacity,
    });
    return fake;
  };

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
      const processorOptions = options.processorOptions?.options;
      this.fake = withSpectrumCollectionStaging(makeFake(
        context.renderQuantumSize,
        1,
        processorOptions?.liveControlObservationTaps !== 0n,
      ));
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
  // this suite models has none. The host's option guards (a spectrum identity's UTF-8 length
  // among them) run synchronously before its first await, so the encoder is present for exactly
  // that span and absent again before any worklet code runs.
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
    // Issue #1491 D1: exactly one boot per caller, and its staging witness is kept with the
    // caller's witnessed options.
    const stagingWitnesses = QualificationNode.latest.fake.spectrumStagingWitnesses;
    assert.equal(stagingWitnesses.length, 1,
      `qualification boot contract: ${label} must boot exactly once`);
    observed.at(-1).stagedSpectrumCollection = stagingWitnesses[0];
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
      && options.options.liveControlObservationTaps === 0n
      && options.options.reserved2 === 0n),
    "qualification boot contract: plain option variant changed");
    const liveControlOptions = observed
      .find(({ label }) => label === "runLiveControlQualification").options.options;
    assert.equal(liveControlOptions.reserved1, 0n);
    assert.equal(liveControlOptions.liveControlMeterBlocks, 2n);
    assert.equal(liveControlOptions.liveControlObservationTaps, 0n);
    assert.equal(liveControlOptions.reserved2, 0n);
    const observationOptions = observed.find(({ label }) => label === "runObservationRun").options.options;
    assert.equal(observationOptions.reserved1, 0n);
    assert.equal(observationOptions.liveControlMeterBlocks, 2n);
    assert.equal(observationOptions.liveControlObservationTaps, 4n);
    assert.equal(observationOptions.reserved2, 0n);
    // Issue #1477 D2: the staging-read run boots the observation caller's live-control fields with
    // no fixed spectrum target and the two-entry collection, in order.
    const stagingReadOptions = observed.find(({ label }) => label === "runStagingReadRun").options.options;
    assert.equal(stagingReadOptions.reserved1, 0n);
    assert.equal(stagingReadOptions.liveControlMeterBlocks, 2n);
    assert.equal(stagingReadOptions.liveControlObservationTaps, 4n);
    assert.equal(stagingReadOptions.reserved2, 0n);
    assert.equal(stagingReadOptions.spectrum, null,
      "qualification boot contract: staging-read run must boot no fixed spectrum target");
    assert.deepEqual(
      stagingReadOptions.spectrumCollection.entries,
      [
        { target: "trackPostPan", targetId: "track", channels: "both" },
        { target: "output", targetId: "main-out", channels: "both" },
      ],
      "qualification boot contract: staging-read spectrum collection changed",
    );
    // Issue #1491 D2: the collection the worklet staged before boot is the caller's witnessed
    // collection, entry for entry. The header, the wire codes and the reserved words (which the
    // bridge refuses unless zero) come from the generated layout; the entries and the budget come
    // from the witnessed options.
    const wireCodes = (table) => Object.fromEntries(
      abiLayout.constants[table].map(({ name, value }) => [name, value]));
    const spectrumTargetCodes = wireCodes("spectrumTargets");
    const spectrumChannelCodes = wireCodes("spectrumChannels");
    const zeroReserved = (layout) => Object.fromEntries(layout.fields
      .filter(({ name }) => name.startsWith("reserved"))
      .map(({ name, type }) => {
        const array = /^u32\[(\d+)\]$/.exec(type);
        return [name, array ? Array(Number(array[1])).fill(0) : type === "u64" ? 0n : 0];
      }));
    const stagingReadCollection = stagingReadOptions.spectrumCollection;
    assert.deepEqual(
      observed.find(({ label }) => label === "runStagingReadRun").stagedSpectrumCollection,
      {
        request: {
          ...zeroReserved(collectionRequestLayout),
          structSize: collectionRequestLayout.bytes,
          abiVersion: abiLayout.abiVersion,
          entryCount: stagingReadCollection.entries.length,
          maximumCaptureBytes: BigInt(stagingReadCollection.maximumCaptureBytes),
        },
        entries: stagingReadCollection.entries.map(({ target, targetId, channels }) => ({
          ...zeroReserved(collectionEntryLayout),
          target: spectrumTargetCodes[target],
          channels: spectrumChannelCodes[channels],
          targetIdBytes: new mainRealmTextEncoder().encode(targetId).length,
          targetId,
        })),
      },
      "qualification boot contract: staging-read staged collection differs from its options",
    );
    // Issue #1491 D3: the five callers with no collection stage none.
    for (const { label, stagedSpectrumCollection } of observed) {
      if (label === "runStagingReadRun") continue;
      assert.equal(stagedSpectrumCollection, null,
        `qualification boot contract: ${label} stages a spectrum collection; `
        + "only runStagingReadRun boots one");
    }

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
  WebAssembly.Instance = class {
    constructor() {
      instanceCount += 1;
      if (throwInstance) throw new Error("synthetic instantiation failure");
      this.exports = recordingExports(nextFake.exports, latestPort);
    }
  };
  try {
    await import(`${workletUrl.href}?processor-test`);
    assert.equal(typeof registered, "function");
    const construct = (fake, options = limits) => {
      nextFake = fake;
      return new registered({
        processorOptions: {
          module: {},
          document: processorSessionDocument,
          options,
        },
      });
    };
    const makeProcessor = (submixIds = undefined) => {
      const fake = createFakeExports(64, 1, true, submixIds);
      const processor = construct(fake);
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "simd128",
        resources: processor.resources, memoryBytes: 196608,
      });
      return { processor, fake };
    };

    const explicitHopOptions = (spectrumHopFrames) => ({
      ...limits,
      spectrumHopFrames,
      spectrum: null,
      spectrumCollection: null,
    });

    for (const hop of [256, 1024]) {
      const fake = createFakeExports(64);
      const processor = construct(fake, explicitHopOptions(hop));
      assert.equal(processor.port.posts[0].message.result, 0,
        `H${hop} additive boot is ready`);
      assert.deepEqual(fake.calls.boot, [], `H${hop} never calls legacy boot`);
      assert.deepEqual(
        fake.calls.bootWithSpectrumHop,
        [[processorSessionDocument.byteLength, hop]],
        `H${hop} additive boot arguments`,
      );
      assert.equal(fake.calls.spectrumHopCapability, 1, `H${hop} probes capability once`);
    }

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
      assert.equal(processor.port.posts[0].message.result, 0, "omitted hop uses legacy boot");
      assert.deepEqual(fake.calls.boot, [[processorSessionDocument.byteLength]],
        "omitted hop legacy boot arguments");
      assert.deepEqual(fake.calls.bootWithSpectrumHop, [], "omitted hop skips additive boot");
      assert.equal(fake.calls.spectrumHopCapability, 0, "omitted hop skips capability probe");
    }

    {
      const fake = createFakeExports(64);
      const oldSpectrumOptions = {
        ...limits, spectrum: null, spectrumCollection: null,
      };
      const processor = construct(fake, oldSpectrumOptions);
      assert.equal(processor.port.posts[0].message.result, 0, "old spectrum shape remains accepted");
      assert.deepEqual(fake.calls.boot, [[processorSessionDocument.byteLength]],
        "old spectrum shape uses legacy boot");
      assert.equal(fake.calls.spectrumHopCapability, 0, "old spectrum shape skips capability probe");
    }

    for (const [label, mutate] of [
      ["missing capability", (fake) => { delete fake.exports.miso_engine_web_v1_spectrum_hop_capability; }],
      ["wrong capability", (fake) => {
        fake.calls.spectrumHopCapabilityResult = 0;
      }],
      ["throwing capability", (fake) => {
        fake.exports.miso_engine_web_v1_spectrum_hop_capability = () => {
          fake.calls.spectrumHopCapability += 1;
          throw new Error("synthetic capability failure");
        };
      }],
      ["missing additive boot", (fake) => {
        delete fake.exports.miso_engine_web_v1_boot_with_spectrum_hop;
      }],
    ]) {
      const fake = createFakeExports(64);
      mutate(fake);
      const processor = construct(fake, explicitHopOptions(256));
      assert.equal(processor.port.posts[0].message.result, 2, `${label} is typed ABI refusal`);
      assert.deepEqual(fake.calls.boot, [], `${label} never calls legacy boot`);
      assert.deepEqual(fake.calls.bootWithSpectrumHop, [], `${label} never calls additive boot`);
      assert.equal(fake.calls.dispose, 0, `${label} publishes no handle to dispose`);
    }

    {
      const fake = createFakeExports(64);
      fake.exports.miso_engine_web_v1_abi_version = () => 0x00020000;
      fake.exports.miso_engine_web_v1_spectrum_hop_capability = () => {
        throw new Error("ABI check must win");
      };
      const processor = construct(fake, explicitHopOptions(256));
      assert.equal(processor.port.posts[0].message.result, 2, "ABI mismatch wins before capability");
      assert.equal(fake.calls.spectrumHopCapability, 0, "ABI mismatch does not probe capability");
      assert.deepEqual(fake.calls.boot, [], "ABI mismatch never calls legacy boot");
      assert.deepEqual(fake.calls.bootWithSpectrumHop, [], "ABI mismatch never calls additive boot");
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
      // Root ruling A (#1332 D5, "growth is not a fault"): the module's memory moving under a
      // command detaches every cached view, and the handler re-cuts them all before it ends. The
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
      assert.equal(status.staleViewQuanta, 0, "no quantum ran on a detached view");
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
    // carries the word.
    {
      const fake = createFakeExports(64);
      new DataView(fake.exports.memory.buffer, 16384).setBigUint64(48, 0x0102_0304_0506n, true);
      const processor = construct(fake);
      assert.equal(processor.port.posts[0].message.tag, "miso.ready.v1");
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const status = processor.port.posts.at(-1).message;
      assert.equal(status.liveValuesSuperseded, 0x0102_0304_0506n);
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
      // Root ruling A: `process()` never re-binds. A memory that moved with no handler since
      // renders silence for the quantum, counts it, and is no fault; the next message handler
      // re-binds and the quantum after renders again.
      const { processor, fake } = makeProcessor();
      fake.growMemory();
      const left = new Float32Array(64).fill(1);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [], "no render on a detached view");
      assert(left.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert(right.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert.equal(processor.stickyResult, 0, "a moved memory is not sticky");
      assert.equal(processor.ready, true);
      assert.equal(processor.staleViewQuanta, 1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.staleViewQuanta, 2, "every silenced quantum is counted");
      assert.equal(
        processor.outputLeft.buffer === fake.exports.memory.buffer,
        false,
        "process() did not re-bind",
      );
      processor.port.onmessage({ data: { tag: "miso.status.v1", requestId: 1 } });
      const status = processor.port.posts.map((post) => post.message)
        .filter((message) => message.tag === "miso.status.v1").at(-1);
      assert.equal(status.staleViewQuanta, 2, "the status reports the silenced quanta");
      assert.equal(status.memoryBytes, 393216);
      left.fill(1);
      right.fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [64], "the handler re-bound and render resumed");
      assert(left.every((sample) => sample === 0.25));
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
      // Issue #137 D1: the worklet copies a staged batch into Wasm, admits it, and hands the
      // record block straight back with the typed report.
      const { processor, fake } = makeProcessor();
      const records = new Uint8Array(2 * 272);
      const view = new DataView(records.buffer);
      records[0] = 7; // subscribe
      records[1] = 1;
      records[2] = 5;
      records[3] = 4;
      view.setUint32(4, 1, true);
      records.set(Buffer.from("track"), 16);
      records.set(Buffer.from("comp0"), 16 + 128);
      records[272] = 8; // unsubscribe
      records[273] = 1;
      records[274] = 5;
      records[275] = 4;
      view.setUint32(272 + 4, 1, true);
      processor.receive(structuredClone(
        { tag: "miso.command.v1", requestId: 1, count: 2, records },
        { transfer: [records.buffer] },
      ));
      const ack = processor.port.posts.at(-1);
      assert.equal(ack.message.tag, "miso.ack.v1");
      assert.equal(ack.message.result, 0);
      assert.equal(ack.message.admitted, 2);
      assert.equal(ack.message.reason, 0);
      assert.equal(ack.transferCount, 1, "the record block goes back to the caller");
      assert.equal(fake.calls.commands.length, 1);
      assert.equal(fake.calls.commands[0].length, 544, "exactly the staged bytes are submitted");
      assert.equal(fake.calls.commands[0][0], 7);
      assert.equal(fake.calls.commands[0][272], 8);

      // A record block whose length disagrees with `count` is sticky-invalid, never truncated.
      const short = new Uint8Array(272);
      processor.receive({ tag: "miso.command.v1", requestId: 2, count: 2, records: short });
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
      assert.equal(processor.port.posts.at(-1).message.result, 1);
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
      // Issue #1562b: a rebuild can change the committed sources and, at its adoption block, the
      // observation bindings, so both maps are read from the engine at each request. Red if the
      // worklet answers from the boot's rows (a source added by a rebuild would never be listed),
      // or if the observation map does not name the structure epoch of the plan that renders.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      fake.sourceRows.push({ id: "zz-added", channels: 1, frames: 480n });
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 1 });
      assert.deepEqual(
        processor.port.posts.at(-1).message.sources.map((source) => source.id),
        ["bass", "drums", "zz-added"],
        "the session map lists a source a rebuild added",
      );
      processor.receive({ tag: "miso.observationmap.v1", requestId: 2 });
      const before = processor.port.posts.at(-1).message;
      assert.equal(before.tag, "miso.observationmap.v1");
      assert.equal(before.structureEpoch, 1, "the boot plan's list");
      assert.deepEqual([...before.bindings], []);

      // A replacement plan waits: the rendering plan's epoch is still the boot's.
      fake.commitMeterSuccessor();
      processor.port.onmessage({ data: { tag: "miso.service.v1" } });
      processor.receive({ tag: "miso.observationmap.v1", requestId: 3 });
      assert.equal(processor.port.posts.at(-1).message.structureEpoch, 1, "not adopted yet");
      // The adopting block: the successor's header is the rendering plan's now.
      fake.calls.adoptOnNextRender = true;
      assert.equal(processor.process([], [[left, right]]), true);
      processor.receive({ tag: "miso.observationmap.v1", requestId: 4 });
      assert.equal(
        processor.port.posts.at(-1).message.structureEpoch,
        2,
        "the adopted plan's list",
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

    await testQualificationBoot({
      registered,
      makeFake: createFakeExports,
      setNextFake: (fake) => { nextFake = fake; },
      setProcessorPortFactory: (factory) => { processorPortFactory = factory; },
      document: processorSessionDocument,
      mainRealmTextEncoder: originalTextEncoder,
    });

    // Issue #1496 D3. The cases above reach the engine worklet's render, command, source, seek and
    // meter paths, so the recording is not empty; an empty one would mean the proxy lost its hook.
    // A call that the table does not list for the engine worklet is a post-construction export
    // that no `render_locked` rule has been asked to prove.
    for (const name of ["render", "command_submit", "source_submit", "meter_poll"]) {
      assert.ok(recordedPostConstructionCalls.has(name),
        `the recording proxy saw the engine worklet call ${name} after construction`);
    }
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
  }
}

await testMainRealm();
await testProcessor();
console.log("web AudioWorklet hermetic tests passed");
