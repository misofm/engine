import assert from "node:assert/strict";
const root = new URL("../", import.meta.url);
const hostUrl = new URL("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js", root);
const workletUrl = new URL("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js", root);

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
  let failSource = false;
  let held = null;

  class FakePort {
    onmessage = null;
    onmessageerror = null;

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
        } else {
          response = { tag: "miso.ack.v1", requestId: received.requestId, result: 0 };
        }
        const delivered = structuredClone(response, { transfer: responseTransfer });
        queueMicrotask(() => this.onmessage?.({ data: delivered }));
      };
      if (holdSource && received.tag === "miso.source.v1") held = respond;
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
      FakeNode.latest = this;
      queueMicrotask(() => this.port.onmessage?.({
        data: {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: options.processorOptions.backend,
          resources: { quantumFrames: 64 }, memoryBytes: 65536,
        },
      }));
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
    if (url.includes("simd")) throw new Error("synthetic unsupported SIMD artifact");
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
      scalarModuleUrl: "scalar.wasm",
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    assert.equal(host.backend, "scalar");
    assert.equal(host.memoryBytes, 65536);
    assert.equal(Object.getOwnPropertyDescriptor(host, "memoryBytes").writable, false);
    assert.deepEqual(events.slice(0, 3), [
      ["compile", "simd.wasm"], ["compile", "scalar.wasm"], ["addModule", "processor.js"],
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
    await errorResult(host.status(), 6);
    held();
    holdSource = false;
    const ack = await sourcePromise;
    assert.deepEqual(Object.keys(ack).sort(), ["planes", "requestId", "result", "tag"]);
    assert.equal(ack.tag, "miso.ack.v1");
    assert.equal(ack.result, 6, "engine backpressure remains a resolved ACK");
    assert.equal(ack.planes[0].byteOffset, 0);
    assert.equal(ack.planes[1].byteOffset, 16);
    assert.equal(ack.planes[0].buffer, ack.planes[1].buffer);
    assert.equal(events.find((event) => event[1] === "miso.source.v1")[3], 1);

    const seek = await host.seekSource({
      requestId: 2, sourceId: "source", generation: 2n, sourceFrame: 10n,
    });
    assert.deepEqual(seek, { tag: "miso.ack.v1", requestId: 2, result: 0 });
    const status = await host.status();
    assert.equal(status.tag, "miso.status.v1");
    assert.equal(status.memoryBytes, host.memoryBytes);
    failSource = true;
    const failedStorage = new ArrayBuffer(16);
    const failedSource = host.submitSource({
      requestId: 4, sourceId: "source", generation: 3n, startFrame: 2n,
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
  } finally {
    globalThis.fetch = original.fetch;
    globalThis.AudioWorkletNode = original.AudioWorkletNode;
    WebAssembly.validate = original.validate;
    WebAssembly.compile = original.compile;
  }
}

function createFakeExports(quantum) {
  const memory = { buffer: new ArrayBuffer(65536) };
  const statusPointer = 16384;
  const resourcePointer = 17000;
  const status = new DataView(memory.buffer, statusPointer, 80);
  status.setUint32(0, 80, true);
  status.setUint32(4, 0x00010000, true);
  status.setUint32(8, 2, true);
  status.setUint32(16, 1, true);
  status.setUint32(20, 48000, true);
  status.setUint32(24, quantum, true);
  const resources = new DataView(memory.buffer, resourcePointer, 224);
  resources.setUint32(0, 224, true);
  resources.setUint32(4, 0x00010000, true);
  resources.setUint32(8, 48000, true);
  resources.setUint32(12, quantum, true);
  resources.setUint32(16, 1, true);
  for (let index = 0; index < 20; index += 1) resources.setBigUint64(32 + index * 8, 1n, true);
  const calls = { render: [], source: [], seek: [], dispose: 0, sourceResult: 0 };
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
      return calls.sourceResult;
    },
    miso_engine_web_v1_source_seek: (...args) => {
      calls.seek.push(args);
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
  let registered;
  let nextFake;
  class FakePort {
    onmessage = null;
    posts = [];

    postMessage(message, transfer = []) {
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
  WebAssembly.Instance = class {
    constructor() {
      this.exports = nextFake.exports;
    }
  };
  try {
    await import(`${workletUrl.href}?processor-test`);
    assert.equal(typeof registered, "function");
    const makeProcessor = () => {
      nextFake = createFakeExports(64);
      const processor = new registered({
        processorOptions: {
          requestId: 0,
          module: {},
          backend: "scalar",
          sampleRateHz: 48000,
          quantumFrames: 64,
          sessionToml: new TextEncoder().encode("format_version = 2"),
          limits,
        },
      });
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "scalar",
        resources: processor.resources, memoryBytes: 65536,
      });
      return { processor, fake: nextFake };
    };

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
        tag: "miso.source.v1", requestId: 1, sourceId: "source", generation: 1n,
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
      processor.receive({
        tag: "miso.seek.v1", requestId: 2, sourceId: "source", generation: 2n, sourceFrame: 12n,
      });
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 2, result: 0,
      });
      processor.receive({ tag: "miso.dispose.v1", requestId: 3 });
      assert.equal(fake.calls.dispose, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
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
    WebAssembly.Instance = originalInstance;
  }
}

await testMainRealm();
await testProcessor();
console.log("web AudioWorklet hermetic tests passed");
