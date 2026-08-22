const RESULT_OK = 0;
const RESULT_BACKPRESSURE = 6;
const RESULT_UNSUPPORTED = 7;
const PROCESSOR_NAME = "miso-engine-v2-audio-worklet";

// Canonical minimal module: `() -> v128` implemented by `i32.const 0; i8x16.splat`.
const SIMD128_PROBE = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
  0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7b,
  0x03, 0x02, 0x01, 0x00,
  0x0a, 0x08, 0x01, 0x06, 0x00, 0x41, 0x00, 0xfd, 0x0f, 0x0b,
]);

const OPTION_FIELDS = [
  "context",
  "quantumFrames",
  "sessionToml",
  "limits",
  "scalarModuleUrl",
  "simd128ModuleUrl",
  "workletModuleUrl",
];

const SOURCE_FIELDS = [
  "requestId",
  "sourceId",
  "generation",
  "startFrame",
  "sampleRateHz",
  "planes",
  "frames",
  "endOfRegion",
];

const SEEK_FIELDS = ["requestId", "sourceId", "generation", "sourceFrame"];

function hasExactFields(value, fields) {
  if (value === null || typeof value !== "object") return false;
  const keys = Object.keys(value).sort();
  const expected = [...fields].sort();
  return keys.length === expected.length && keys.every((key, index) => key === expected[index]);
}

function webError(result, requestId = 0, planes) {
  const record = { tag: "miso.error.v1", requestId, result };
  if (planes !== undefined) record.planes = planes;
  return Object.freeze(record);
}

function validRequestId(value) {
  return Number.isSafeInteger(value) && value > 0;
}

async function fetchModule(url) {
  const response = await fetch(url);
  if (!response.ok) throw webError(RESULT_UNSUPPORTED);
  return WebAssembly.compile(await response.arrayBuffer());
}

async function selectModule(options) {
  if (WebAssembly.validate(SIMD128_PROBE)) {
    try {
      return {
        backend: "simd128",
        module: await fetchModule(options.simd128ModuleUrl),
      };
    } catch (_) {
      // The frozen fallback is scalar and occurs only before processor construction.
    }
  }
  return {
    backend: "scalar",
    module: await fetchModule(options.scalarModuleUrl),
  };
}

class MisoAudioWorkletHostV1 {
  #port;
  #pending = null;
  #lastRequestId = 0;
  #stickyError = null;
  #disposed = false;

  constructor(node, backend, resources, memoryBytes) {
    Object.defineProperties(this, {
      node: { value: node, enumerable: true },
      backend: { value: backend, enumerable: true },
      resources: { value: Object.freeze(resources), enumerable: true },
      memoryBytes: { value: memoryBytes, enumerable: true },
    });
    this.#port = node.port;
    this.#port.onmessage = (event) => this.#receive(event.data);
    this.#port.onmessageerror = () => this.#fail(webError(
      255,
      this.#pending?.requestId ?? 0,
    ));
    this.node.onprocessorerror = () => this.#fail(webError(
      255,
      this.#pending?.requestId ?? 0,
    ));
  }

  #receive(message) {
    const errorFields = message?.planes === undefined
      ? ["tag", "requestId", "result"]
      : ["tag", "requestId", "result", "planes"];
    if (message?.tag === "miso.error.v1" && hasExactFields(message, errorFields)) {
      this.#fail(Object.freeze(message));
      return;
    }
    if (this.#pending === null || message?.requestId !== this.#pending.requestId) {
      this.#fail(webError(255, message?.requestId ?? 0));
      return;
    }
    const pending = this.#pending;
    const expectedFields = pending.response === "source"
      ? ["tag", "requestId", "result", "planes"]
      : pending.response === "status"
        ? [
          "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
          "quantumFrames", "nextAbsoluteSample", "renderedQuanta", "memoryBytes",
        ]
        : ["tag", "requestId", "result"];
    const expectedTag = pending.response === "status" ? "miso.status.v1" : "miso.ack.v1";
    if (message.tag !== expectedTag || !hasExactFields(message, expectedFields)) {
      this.#fail(webError(255, message.requestId));
      return;
    }
    this.#pending = null;
    if (pending.response === "dispose" && message.result !== RESULT_OK) {
      pending.reject(webError(message.result, message.requestId));
    } else {
      pending.resolve(Object.freeze(message));
    }
  }

  #fail(error) {
    this.#stickyError = error;
    if (this.#pending !== null) {
      const pending = this.#pending;
      this.#pending = null;
      pending.reject(error);
    }
  }

  #request(message, transfer = [], response, allowSticky = false) {
    if (this.#disposed) return Promise.reject(webError(3, message.requestId));
    if (this.#stickyError !== null && !allowSticky) return Promise.reject(this.#stickyError);
    if (this.#pending !== null) {
      return Promise.reject(webError(RESULT_BACKPRESSURE, message.requestId));
    }
    if (!validRequestId(message.requestId) || message.requestId <= this.#lastRequestId) {
      return Promise.reject(webError(1, message.requestId));
    }
    this.#lastRequestId = message.requestId;
    return new Promise((resolve, reject) => {
      this.#pending = { requestId: message.requestId, resolve, reject, response };
      try {
        this.#port.postMessage(message, transfer);
      } catch (error) {
        this.#pending = null;
        reject(webError(255, message.requestId));
      }
    });
  }

  submitSource(request) {
    if (!hasExactFields(request, SOURCE_FIELDS)
        || typeof request.sourceId !== "string"
        || typeof request.generation !== "bigint" || request.generation <= 0n
        || typeof request.startFrame !== "bigint" || request.startFrame < 0n
        || !Number.isSafeInteger(request.sampleRateHz) || request.sampleRateHz <= 0
        || !Number.isSafeInteger(request.frames) || request.frames < 0
        || typeof request.endOfRegion !== "boolean"
        || !Array.isArray(request.planes)
        || request.planes.length === 0
        || request.planes.some((plane) => !(plane instanceof Float32Array)
          || plane.length !== request.frames
          || !(plane.buffer instanceof ArrayBuffer)
          || (typeof SharedArrayBuffer !== "undefined"
            && plane.buffer instanceof SharedArrayBuffer))) {
      return Promise.reject(webError(1, request?.requestId ?? 0));
    }
    const transfer = [...new Set(request.planes.map((plane) => plane.buffer))];
    return this.#request({ tag: "miso.source.v1", ...request }, transfer, "source");
  }

  seekSource(request) {
    if (!hasExactFields(request, SEEK_FIELDS)
        || typeof request.sourceId !== "string"
        || typeof request.generation !== "bigint" || request.generation <= 0n
        || typeof request.sourceFrame !== "bigint" || request.sourceFrame < 0n) {
      return Promise.reject(webError(1, request?.requestId ?? 0));
    }
    return this.#request({ tag: "miso.seek.v1", ...request }, [], "seek");
  }

  status() {
    return this.#request(
      { tag: "miso.status.v1", requestId: this.#lastRequestId + 1 },
      [],
      "status",
    );
  }

  async dispose() {
    if (this.#disposed) return;
    const requestId = this.#lastRequestId + 1;
    await this.#request({ tag: "miso.dispose.v1", requestId }, [], "dispose", true);
    this.#disposed = true;
    this.#port.onmessage = null;
    this.#port.onmessageerror = null;
    this.node.onprocessorerror = null;
  }
}

export async function createMisoAudioWorkletHost(options) {
  if (!hasExactFields(options, OPTION_FIELDS)
      || options.context?.state !== "suspended"
      || !Number.isInteger(options.quantumFrames) || options.quantumFrames <= 0
      || !(options.sessionToml instanceof Uint8Array)
      || options.limits === null || typeof options.limits !== "object"
      || typeof options.scalarModuleUrl !== "string"
      || typeof options.simd128ModuleUrl !== "string"
      || typeof options.workletModuleUrl !== "string") {
    throw webError(1);
  }
  const exposedQuantum = options.context.renderQuantumSize;
  if (typeof exposedQuantum === "number" && exposedQuantum !== 0
      && exposedQuantum !== options.quantumFrames) {
    throw webError(9);
  }
  const selected = await selectModule(options);
  await options.context.audioWorklet.addModule(options.workletModuleUrl);
  const node = new AudioWorkletNode(options.context, PROCESSOR_NAME, {
    numberOfInputs: 0,
    numberOfOutputs: 1,
    outputChannelCount: [2],
    processorOptions: {
      requestId: 0,
      module: selected.module,
      backend: selected.backend,
      sampleRateHz: options.context.sampleRate,
      quantumFrames: options.quantumFrames,
      sessionToml: new Uint8Array(options.sessionToml),
      limits: { ...options.limits },
    },
  });
  const ready = await new Promise((resolve, reject) => {
    let settled = false;
    const finish = (operation, value) => {
      if (settled) return;
      settled = true;
      node.port.onmessage = null;
      node.port.onmessageerror = null;
      node.onprocessorerror = null;
      operation(value);
    };
    node.port.onmessage = (event) => {
      const message = event.data;
      if (hasExactFields(
        message,
        ["tag", "requestId", "result", "backend", "resources", "memoryBytes"],
      )
          && message.tag === "miso.ready.v1" && message.requestId === 0
          && message.result === RESULT_OK && message.backend === selected.backend) {
        finish(resolve, message);
      } else if (hasExactFields(
        message,
        message?.planes === undefined
          ? ["tag", "requestId", "result"]
          : ["tag", "requestId", "result", "planes"],
      )
          && message.tag === "miso.error.v1" && message.requestId === 0) {
        finish(reject, Object.freeze(message));
      } else {
        finish(reject, webError(255));
      }
    };
    node.port.onmessageerror = () => finish(reject, webError(255));
    node.onprocessorerror = () => finish(reject, webError(255));
  });
  return new MisoAudioWorkletHostV1(
    node,
    selected.backend,
    ready.resources,
    ready.memoryBytes,
  );
}
