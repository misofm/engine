import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
const root = new URL("../", import.meta.url);
const hostUrl = new URL("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js", root);
const workletUrl = process.env.MISO_WEB_WORKLET_TEST_MODULE === undefined
  ? new URL("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js", root)
  : pathToFileURL(process.env.MISO_WEB_WORKLET_TEST_MODULE);

const limits = Object.freeze({
  sessionTomlBytes: 4096,
  diagnosticBytes: 512,
  sourceIdBytes: 64,
  maximumSourceChannels: 2,
  sourceRingFrames: 256,
  maximumAutomationSpansPerBlock: 8,
  maximumTracks: 8n,
  maximumSources: 4n,
  maximumRoutes: 8n,
  maximumEffects: 8n,
  maximumGraphSessionPlusPlanBytes: 1_000_000n,
  maximumSourceTotalBytes: 1_000_000n,
  maximumSourceOverheadBytes: 1_000_000n,
  maximumEffectStateBytes: 1_000_000n,
  maximumEffectScratchBytes: 1_000_000n,
  maximumBuiltinRetainedBytes: 1_000_000n,
  maximumHostRetainedBytes: 2_000_000n,
  maximumNamedAllocationBytes: 1_000_000n,
  maximumMeterStreams: 8n,
  maximumMeterItems: 16n,
  maximumMeterBytes: 4096n,
});

function resourceReport(backend, quantumFrames) {
  return Object.freeze({
    sampleRateHz: 48000,
    quantumFrames,
    backend,
    configBytes: 1n,
    statusBytes: 1n,
    sessionTomlBytes: 1n,
    diagnosticBytes: 1n,
    sourceIdBytes: 1n,
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

async function testMainRealm() {
  const original = {
    fetch: globalThis.fetch,
    AudioWorkletNode: globalThis.AudioWorkletNode,
    validate: WebAssembly.validate,
    compile: WebAssembly.compile,
  };
  const events = [];
  let holdSource = false;
  let holdAll = false;
  const heldAll = [];
  let failSource = false;
  let held = null;
  let readyMutation = null;
  let statusMutation = null;
  let planeMutation = null;

  class FakePort {
    onmessage = null;
    onmessageerror = null;
    closeCount = 0;

    close() {
      this.closeCount += 1;
    }

    postMessage(message, transfer) {
      const received = structuredClone(message, { transfer });
      events.push(["request", received.tag, received.requestId, transfer.length]);
      const respond = () => {
        let response;
        let responseTransfer = [];
        if (received.tag === "miso.source.v1") {
          response = {
            tag: failSource ? "miso.error.v1" : "miso.ack.v1",
            requestId: received.requestId,
            result: failSource ? 1 : 6,
            planes: received.planes,
          };
          responseTransfer = [...new Set(received.planes.map((plane) => plane.buffer))];
        } else if (received.tag === "miso.status.v1") {
          response = {
            tag: "miso.status.v1", requestId: received.requestId, result: 0, state: 2,
            lastResult: 0, backend: 1, sampleRateHz: 48000, quantumFrames: 64,
            nextAbsoluteSample: 64n, renderedQuanta: 1n, memoryBytes: 65536,
          };
          if (statusMutation !== null) response = statusMutation(response);
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

    constructor(_context, _name, options) {
      this.port = new FakePort();
      this.onprocessorerror = null;
      this.options = options;
      this.disposeMessages = 0;
      this.disconnectCount = 0;
      FakeNode.latest = this;
      queueMicrotask(() => {
        let data = {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: options.processorOptions.backend,
          resources: resourceReport(options.processorOptions.backend === "scalar" ? 0 : 1, 64),
          memoryBytes: 65536,
        };
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
  globalThis.fetch = async (url) => ({
    ok: true,
    arrayBuffer: async () => new TextEncoder().encode(String(url)).buffer,
  });
  WebAssembly.compile = async (bytes) => {
    const url = new TextDecoder().decode(bytes);
    events.push(["compile", url]);
    return Object.freeze({ url });
  };
  const context = {
    state: "suspended",
    sampleRate: 48000,
    renderQuantumSize: 64,
    audioWorklet: { addModule: async (url) => events.push(["addModule", url]) },
  };
  try {
    const { createMisoAudioWorkletHost } = await import(`${hostUrl.href}?main-test`);
    const host = await createMisoAudioWorkletHost({
      context,
      quantumFrames: 64,
      sessionToml: new TextEncoder().encode("format_version = 2"),
      limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    assert.equal(host.backend, "simd128", "W4-D1 ships exactly one artifact");
    assert.equal(host.memoryBytes, 65536);
    assert.equal(Object.getOwnPropertyDescriptor(host, "memoryBytes").writable, false);
    assert.deepEqual(events.slice(0, 2), [
      ["compile", "simd.wasm"], ["addModule", "processor.js"],
    ]);

    const storage = new ArrayBuffer(32);
    const left = new Float32Array(storage, 0, 2);
    const right = new Float32Array(storage, 16, 2);
    left.set([1, 2]);
    right.set([3, 4]);
    holdSource = true;
    const sourcePromise = host.submitSource({
      requestId: 1, sourceId: "source", generation: 1n, startFrame: 0n,
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
      requestId: 3, sourceId: "source", generation: 2n, sourceFrame: 10n,
    });
    assert.deepEqual(seek, { tag: "miso.ack.v1", requestId: 3, result: 0 });
    const status = await host.status();
    assert.equal(status.tag, "miso.status.v1");
    assert.equal(status.memoryBytes, host.memoryBytes);
    failSource = true;
    const failedStorage = new ArrayBuffer(16);
    const failedSource = host.submitSource({
      requestId: 5, sourceId: "source", generation: 3n, startFrame: 2n,
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
      quantumFrames: 64,
      sessionToml: new Uint8Array(),
      limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    }), 255);
    assert.equal(FakeNode.latest.port.closeCount, 1, "creation rejection closes the port");
    assert.equal(FakeNode.latest.disconnectCount, 1, "creation rejection disconnects the node");
    readyMutation = null;

    const schemaHost = await createMisoAudioWorkletHost({
      context,
      quantumFrames: 64,
      sessionToml: new Uint8Array(),
      limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    statusMutation = (statusValue) => ({ ...statusValue, memoryBytes: 65537 });
    await errorResult(schemaHost.status(), 255);
    statusMutation = null;
    await schemaHost.dispose();

    const planeHost = await createMisoAudioWorkletHost({
      context,
      quantumFrames: 64,
      sessionToml: new Uint8Array(),
      limits,
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
      requestId: 1, sourceId: "source", generation: 1n, startFrame: 0n,
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
      quantumFrames: 64,
      sessionToml: new Uint8Array(),
      limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    failSource = false;
    holdAll = true;
    const chunk = (requestId, sourceId) => {
      const buffer = new ArrayBuffer(8);
      return {
        request: pipelineHost.submitSource({
          requestId,
          sourceId,
          generation: 1n,
          startFrame: BigInt(requestId),
          sampleRateHz: 48000,
          planes: [new Float32Array(buffer)],
          frames: 2,
          endOfRegion: false,
        }),
        buffer,
      };
    };
    const inFlight = [1, 2, 3, 4].map((requestId) => chunk(requestId, "source"));
    for (const [index, entry] of inFlight.entries()) {
      assert.equal(entry.buffer.byteLength, 0, `chunk ${index} was transferred`);
    }
    const overflow = chunk(5, "source");
    await errorResult(overflow.request, 6);
    assert.equal(
      overflow.buffer.byteLength,
      8,
      "a locally refused chunk keeps its planes: nothing is transferred and the caller can retry",
    );
    // A different source has its own budget and is accepted while the first is saturated.
    const other = chunk(6, "other-source");
    assert.equal(other.buffer.byteLength, 0, "the bound is per source, not per host");
    // One unsettled seek per source: the ring carries a single command slot.
    const firstSeek = pipelineHost.seekSource({
      requestId: 7, sourceId: "source", generation: 2n, sourceFrame: 0n,
    });
    await errorResult(pipelineHost.seekSource({
      requestId: 8, sourceId: "source", generation: 3n, sourceFrame: 0n,
    }), 6);
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
      [1, 2, 3, 4, 6, 7],
      "acknowledgements arrive in request order",
    );
    // With the budget released the source accepts chunks again.
    const again = chunk(9, "source");
    assert.equal(again.buffer.byteLength, 0);
    await again.request;
    await pipelineHost.dispose();

    // W4-D1: a browser that cannot validate simd128 is refused with the typed record, before any
    // network request. Red mutation: delete the `WebAssembly.validate(SIMD128_PROBE)` guard in
    // `createMisoAudioWorkletHost` -> the rejection becomes a generic 255 and `compileCount` grows.
    const compileCount = events.filter((event) => event[0] === "compile").length;
    WebAssembly.validate = () => false;
    const refusal = await createMisoAudioWorkletHost({
      context,
      quantumFrames: 64,
      sessionToml: new Uint8Array(),
      limits,
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
  } finally {
    globalThis.fetch = original.fetch;
    globalThis.AudioWorkletNode = original.AudioWorkletNode;
    WebAssembly.validate = original.validate;
    WebAssembly.compile = original.compile;
  }
}

function createFakeExports(quantum, backend = 0) {
  const memory = { buffer: new ArrayBuffer(65536) };
  const statusPointer = 16384;
  const resourcePointer = 17000;
  const status = new DataView(memory.buffer, statusPointer, 80);
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
  for (let index = 0; index < 20; index += 1) resources.setBigUint64(32 + index * 8, 1n, true);
  const calls = {
    render: [], source: [], sourceIdBytes: [], seek: [], seekIdBytes: [], dispose: 0,
    sourceResult: 0,
  };
  const pointers = { 1: 2048, 2: 4096, 3: 5000, 5: 8192 };
  const capacities = { 1: 4096, 2: 64, 3: 2 * quantum * 4, 5: 2 * quantum * 4 };
  const exports = {
    memory,
    miso_engine_web_v1_abi_version: () => 0x00010000,
    miso_engine_web_v1_config_bytes: () => 192,
    miso_engine_web_v1_config_new: () => 1,
    miso_engine_web_v1_config_ptr: () => 512,
    miso_engine_web_v1_prepare: () => 0,
    miso_engine_web_v1_compile: () => 0,
    miso_engine_web_v1_buffer_ptr: (_handle, kind) => pointers[kind] ?? 0,
    miso_engine_web_v1_buffer_capacity: (_handle, kind) => capacities[kind] ?? 0,
    miso_engine_web_v1_status_ptr: () => statusPointer,
    miso_engine_web_v1_resource_ptr: () => resourcePointer,
    miso_engine_web_v1_render: (_handle, actualFrames) => {
      calls.render.push(actualFrames);
      if (actualFrames !== quantum) return 9;
      new Float32Array(memory.buffer, 8192, quantum).fill(0.25);
      new Float32Array(memory.buffer, 8192 + quantum * 4, quantum).fill(-0.25);
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
    miso_engine_web_v1_dispose: () => {
      calls.dispose += 1;
      return 0;
    },
  };
  return { exports, calls };
}

async function testProcessor() {
  const originalProcessor = globalThis.AudioWorkletProcessor;
  const originalRegister = globalThis.registerProcessor;
  const originalInstance = WebAssembly.Instance;
  const originalSampleRate = globalThis.sampleRate;
  const originalTextEncoder = globalThis.TextEncoder;
  const processorSessionToml = new originalTextEncoder().encode("format_version = 2");
  const nonAsciiSourceId = "caf\u00e9-\u96ea-\ud83d\ude00";
  const nonAsciiSourceIdUtf8 = new originalTextEncoder().encode(nonAsciiSourceId);
  let registered;
  let nextFake;
  let instanceCount = 0;
  let throwInstance = false;
  let throwReadyPost = false;
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
      this.port = new FakePort();
    }
  }
  globalThis.AudioWorkletProcessor = FakeProcessor;
  globalThis.registerProcessor = (_name, implementation) => { registered = implementation; };
  globalThis.sampleRate = 48000;
  globalThis.TextEncoder = undefined;
  WebAssembly.Instance = class {
    constructor() {
      instanceCount += 1;
      if (throwInstance) throw new Error("synthetic instantiation failure");
      this.exports = nextFake.exports;
    }
  };
  try {
    await import(`${workletUrl.href}?processor-test`);
    assert.equal(typeof registered, "function");
    const construct = (fake, backend = "scalar") => {
      nextFake = fake;
      return new registered({
        processorOptions: {
          requestId: 0,
          module: {},
          backend,
          sampleRateHz: 48000,
          quantumFrames: 64,
          sessionToml: processorSessionToml,
          limits,
        },
      });
    };
    const makeProcessor = () => {
      const fake = createFakeExports(64);
      const processor = construct(fake);
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "scalar",
        resources: processor.resources, memoryBytes: 65536,
      });
      return { processor, fake };
    };

    {
      const before = instanceCount;
      globalThis.sampleRate = 44100;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      globalThis.sampleRate = 48000;
      assert.equal(instanceCount, before, "sample-rate mismatch precedes instantiation");
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 9);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      const before = instanceCount;
      globalThis.renderQuantumSize = 128;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      delete globalThis.renderQuantumSize;
      assert.equal(instanceCount, before, "quantum mismatch precedes instantiation");
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

    const failureMutations = [
      (fake) => { fake.exports.miso_engine_web_v1_config_ptr = () => 0; },
      (fake) => { fake.exports.miso_engine_web_v1_config_ptr = () => 65500; },
      (fake) => { fake.exports.miso_engine_web_v1_prepare = () => 5; },
      (fake) => { fake.exports.miso_engine_web_v1_buffer_capacity = () => 0; },
      (fake) => { fake.exports.miso_engine_web_v1_compile = () => 5; },
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
      const fake = createFakeExports(64, 1);
      const processor = construct(fake, "scalar");
      assert.equal(fake.calls.dispose, 1, "swapped backend artifact is transactionally disposed");
      assert.equal(processor.port.posts[0].message.result, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    for (const offset of [16384 + 16, 17000 + 16]) {
      const fake = createFakeExports(64);
      new DataView(fake.exports.memory.buffer).setUint32(offset, 1, true);
      const processor = construct(fake, "scalar");
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
      assert.equal(status.memoryBytes, 65536);
      assert.equal(status.nextAbsoluteSample, 64n);
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
      const { processor, fake } = makeProcessor();
      fake.exports.memory.buffer = new ArrayBuffer(65536);
      const left = new Float32Array(64).fill(1);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, []);
      assert(left.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert(right.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert.equal(processor.stickyResult, 9);
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
  } finally {
    globalThis.AudioWorkletProcessor = originalProcessor;
    globalThis.registerProcessor = originalRegister;
    globalThis.sampleRate = originalSampleRate;
    globalThis.TextEncoder = originalTextEncoder;
    WebAssembly.Instance = originalInstance;
  }
}

await testMainRealm();
await testProcessor();
console.log("web AudioWorklet hermetic tests passed");
