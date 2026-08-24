const RESULT_OK = 0;
const RESULT_BACKPRESSURE = 6;
const RESULT_UNSUPPORTED = 7;
const PROCESSOR_NAME = "miso-engine-v2-audio-worklet";
const VALID_RESULTS = new Set([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255]);

// Canonical minimal module: `() -> v128` implemented by `i32.const 0; i8x16.splat`.
//
// Owner decision W4-D1 (#83): exactly one artifact ships and it is built with `+simd128`, so this
// probe is the browser twin of D4's native `attest_host` boot attestation -- a browser that cannot
// validate a `simd128` module is refused up front with a typed error, never silently degraded to a
// scalar build. `WebAssembly.validate` is synchronous, allocation-free and runs before any network
// request, so an unsupported browser pays nothing.
const SIMD128_PROBE = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
  0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7b,
  0x03, 0x02, 0x01, 0x00,
  0x0a, 0x08, 0x01, 0x06, 0x00, 0x41, 0x00, 0xfd, 0x0f, 0x0b,
]);

// The one frozen shipping backend (issue 024 `BACKEND_SIMD128`). It is still sent explicitly so
// the processor can cross-check it against the backend row the Rust artifact reports: a module
// built without `+simd128` reports `0` and is rejected transactionally rather than rendered with.
const SHIPPING_BACKEND = "simd128";

const OPTION_FIELDS = [
  "context",
  "quantumFrames",
  "sessionToml",
  "limits",
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
const LIMIT_FIELDS = [
  "sessionTomlBytes", "diagnosticBytes", "sourceIdBytes", "maximumSourceChannels",
  "sourceRingFrames", "maximumAutomationSpansPerBlock", "maximumTracks", "maximumSources",
  "maximumRoutes", "maximumEffects", "maximumGraphSessionPlusPlanBytes",
  "maximumSourceTotalBytes", "maximumSourceOverheadBytes", "maximumEffectStateBytes",
  "maximumEffectScratchBytes", "maximumBuiltinRetainedBytes", "maximumHostRetainedBytes",
  "maximumNamedAllocationBytes", "maximumMeterStreams", "maximumMeterItems", "maximumMeterBytes",
];
const RESOURCE_FIELDS = [
  "sampleRateHz", "quantumFrames", "backend", "configBytes", "statusBytes", "sessionTomlBytes",
  "diagnosticBytes", "sourceIdBytes", "sourcePcmStagingBytes", "outputPcmBytes",
  "bridgeMetadataBytes", "bridgeRetainedBytes", "largestBridgeAllocationBytes",
  "sourceTotalBytes", "sourceOverheadBytes", "effectScalarStateBytes", "effectScalarScratchBytes",
  "builtinRetainedBytes", "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes",
  "graphMetadataBytes", "graphDelayBytes", "largestNamedAllocationBytes",
];
const RESOURCE_U64_FIELDS = RESOURCE_FIELDS.slice(3);

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

function validU32(value) {
  return Number.isInteger(value) && value >= 0 && value <= 0xffffffff;
}

function validU64(value, positive = false) {
  return typeof value === "bigint" && value >= (positive ? 1n : 0n)
    && value <= 0xffffffffffffffffn;
}

function validResult(value) {
  return VALID_RESULTS.has(value);
}

function numericBackend(backend) {
  return backend === "scalar" ? 0 : 1;
}

function validLimits(limits) {
  if (!hasExactFields(limits, LIMIT_FIELDS)) return false;
  return LIMIT_FIELDS.slice(0, 6).every((field) => validU32(limits[field]))
    && limits.sessionTomlBytes > 0 && limits.diagnosticBytes > 0 && limits.sourceIdBytes > 0
    && limits.maximumSourceChannels > 0 && limits.sourceRingFrames > 0
    && LIMIT_FIELDS.slice(6).every((field) => validU64(limits[field], true));
}

function validResources(resources, backend, sampleRateHz, quantumFrames) {
  return hasExactFields(resources, RESOURCE_FIELDS)
    && resources.backend === numericBackend(backend)
    && resources.sampleRateHz === sampleRateHz
    && resources.quantumFrames === quantumFrames
    && RESOURCE_U64_FIELDS.every((field) => validU64(resources[field]));
}

function planeShape(planes) {
  const groups = new Map();
  return planes.map((plane) => {
    if (!groups.has(plane.buffer)) groups.set(plane.buffer, groups.size);
    return {
      byteOffset: plane.byteOffset,
      byteLength: plane.byteLength,
      length: plane.length,
      group: groups.get(plane.buffer),
    };
  });
}

function validReturnedPlanes(planes, expected) {
  if (!Array.isArray(planes) || planes.length !== expected.length) return false;
  const buffers = [];
  for (let index = 0; index < planes.length; index += 1) {
    const plane = planes[index];
    const shape = expected[index];
    if (!(plane instanceof Float32Array) || !(plane.buffer instanceof ArrayBuffer)
        || (typeof SharedArrayBuffer !== "undefined" && plane.buffer instanceof SharedArrayBuffer)
        || plane.byteOffset !== shape.byteOffset || plane.byteLength !== shape.byteLength
        || plane.length !== shape.length) return false;
    if (buffers[shape.group] === undefined) buffers[shape.group] = plane.buffer;
    if (buffers[shape.group] !== plane.buffer) return false;
    for (let group = 0; group < buffers.length; group += 1) {
      if (group !== shape.group && buffers[group] === plane.buffer) return false;
    }
  }
  return true;
}

function cleanupNode(node) {
  if (node === undefined) return;
  node.port.onmessage = null;
  node.port.onmessageerror = null;
  node.onprocessorerror = null;
  try { node.port.close?.(); } catch (_) { /* already closed */ }
  try { node.disconnect(); } catch (_) { /* never connected or already disconnected */ }
}

async function fetchModule(url) {
  const response = await fetch(url);
  if (!response.ok) throw webError(RESULT_UNSUPPORTED);
  return WebAssembly.compile(await response.arrayBuffer());
}

/// The typed refusal for a browser that cannot run the shipped artifact.
///
/// Deliberately not a `miso.error.v1`: a caller must be able to tell "this browser is out of
/// scope" apart from "something went wrong", and the generic record carries no room to say which
/// capability is missing. It never crosses the `MessagePort`, so it is not part of the frozen
/// worklet message schema; it is thrown by `createMisoAudioWorkletHost` before any node exists.
function unsupportedBrowser(capability) {
  return Object.freeze({
    tag: "miso.unsupported.v1",
    requestId: 0,
    result: RESULT_UNSUPPORTED,
    capability,
  });
}

function isUnsupportedBrowser(value) {
  return value?.tag === "miso.unsupported.v1"
    && hasExactFields(value, ["tag", "requestId", "result", "capability"]);
}

/// The browser-side streaming model (#106 F3).
///
/// The engine streams sources just-in-time into bounded per-source rings, so the main realm must be
/// allowed to run ahead of the render position: with exactly one request in flight the feed can
/// never be deeper than one message round trip, and any main-thread stall longer than that
/// underruns every source. The bound is therefore per source and equal to the ring depth in quanta
/// -- there is nowhere for a further chunk to go -- rather than one for the whole host.
///
/// Seeks stay at one unsettled per source because the ring carries a single command slot. Status
/// stays at one because a second answer would carry no new information. Dispose is terminal and
/// waits for nothing: the worklet handles messages in arrival order, so every earlier request is
/// already settled by the time its acknowledgement is posted.
///
/// Nothing is dropped silently: a request over its bound is rejected locally, before transfer, with
/// a typed `RESULT_BACKPRESSURE` and its planes still owned by the caller.
class MisoAudioWorkletHostV1 {
  #port;
  #pending = new Map();
  #inFlightSources = new Map();
  #inFlightSeeks = new Map();
  #inFlightStatus = 0;
  #ringBlocks;
  #lastRequestId = 0;
  #stickyError = null;
  #disposed = false;
  #numericBackend;
  #sampleRateHz;
  #quantumFrames;

  constructor(node, backend, sampleRateHz, quantumFrames, resources, memoryBytes, ringBlocks) {
    Object.defineProperties(this, {
      node: { value: node, enumerable: true },
      backend: { value: backend, enumerable: true },
      resources: { value: Object.freeze(resources), enumerable: true },
      memoryBytes: { value: memoryBytes, enumerable: true },
    });
    this.#numericBackend = numericBackend(backend);
    this.#sampleRateHz = sampleRateHz;
    this.#quantumFrames = quantumFrames;
    this.#ringBlocks = ringBlocks;
    this.#port = node.port;
    this.#port.onmessage = (event) => this.#receive(event.data);
    this.#port.onmessageerror = () => this.#fail(webError(255, this.#oldestRequestId()));
    // A user-agent/processor crash cannot return storage already transferred out of this realm.
    this.node.onprocessorerror = () => this.#fail(webError(255, this.#oldestRequestId()));
  }

  // Request IDs are strictly monotonic and the worklet handles messages in arrival order, so the
  // first key of the insertion-ordered map is the oldest unsettled request.
  #oldestRequestId() {
    for (const requestId of this.#pending.keys()) return requestId;
    return 0;
  }

  #release(pending) {
    this.#pending.delete(pending.requestId);
    if (pending.response === "source") {
      const count = (this.#inFlightSources.get(pending.sourceId) ?? 1) - 1;
      if (count <= 0) this.#inFlightSources.delete(pending.sourceId);
      else this.#inFlightSources.set(pending.sourceId, count);
    } else if (pending.response === "seek") {
      this.#inFlightSeeks.delete(pending.sourceId);
    } else if (pending.response === "status") {
      this.#inFlightStatus -= 1;
    }
  }

  #receive(message) {
    const pending = this.#pending.get(message?.requestId) ?? null;
    const errorFields = message?.planes === undefined
      ? ["tag", "requestId", "result"]
      : ["tag", "requestId", "result", "planes"];
    if (message?.tag === "miso.error.v1" && hasExactFields(message, errorFields)) {
      const validPlanes = pending?.response === "source"
        ? validReturnedPlanes(message.planes, pending.planeShape)
        : message.planes === undefined;
      if (validRequestId(message.requestId) && validResult(message.result)
          && pending !== null && validPlanes) {
        this.#fail(Object.freeze(message));
      } else {
        this.#fail(webError(255, pending?.requestId ?? this.#oldestRequestId()));
      }
      return;
    }
    if (pending === null) {
      this.#fail(webError(255, message?.requestId ?? 0));
      return;
    }
    const expectedFields = pending.response === "source"
      ? ["tag", "requestId", "result", "planes"]
      : pending.response === "status"
        ? [
          "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
          "quantumFrames", "nextAbsoluteSample", "renderedQuanta", "memoryBytes",
        ]
        : ["tag", "requestId", "result"];
    const expectedTag = pending.response === "status" ? "miso.status.v1" : "miso.ack.v1";
    const validSourcePlanes = pending.response !== "source"
      || validReturnedPlanes(message.planes, pending.planeShape);
    const validStatus = pending.response !== "status" || (
      message.result === RESULT_OK && validU32(message.state) && message.state <= 4
      && validResult(message.lastResult) && message.backend === this.#numericBackend
      && message.sampleRateHz === this.#sampleRateHz
      && message.quantumFrames === this.#quantumFrames
      && validU64(message.nextAbsoluteSample) && validU64(message.renderedQuanta)
      && Number.isSafeInteger(message.memoryBytes) && message.memoryBytes === this.memoryBytes
    );
    if (message.tag !== expectedTag || !hasExactFields(message, expectedFields)
        || !validRequestId(message.requestId) || !validResult(message.result)
        || !validSourcePlanes || !validStatus) {
      this.#fail(webError(255, message.requestId));
      return;
    }
    this.#release(pending);
    if (pending.response === "dispose" && message.result !== RESULT_OK) {
      pending.reject(webError(message.result, message.requestId));
    } else {
      pending.resolve(Object.freeze(message));
    }
  }

  // An error is sticky for the whole host, so every unsettled request is rejected with it. The
  // maps are cleared first so a `reject` handler that re-enters sees a settled host.
  #fail(error) {
    this.#stickyError = error;
    const unsettled = [...this.#pending.values()];
    this.#pending.clear();
    this.#inFlightSources.clear();
    this.#inFlightSeeks.clear();
    this.#inFlightStatus = 0;
    for (const pending of unsettled) pending.reject(error);
  }

  // The per-kind bound. `true` means the request has nowhere to go and is refused locally, before
  // any transfer, so the caller keeps its planes and can retry.
  #saturated(response, sourceId) {
    if (response === "source") {
      return (this.#inFlightSources.get(sourceId) ?? 0) >= this.#ringBlocks;
    }
    if (response === "seek") return this.#inFlightSeeks.has(sourceId);
    if (response === "status") return this.#inFlightStatus >= 1;
    return false; // dispose is terminal and waits for nothing
  }

  #reserve(response, sourceId) {
    if (response === "source") {
      this.#inFlightSources.set(sourceId, (this.#inFlightSources.get(sourceId) ?? 0) + 1);
    } else if (response === "seek") {
      this.#inFlightSeeks.set(sourceId, true);
    } else if (response === "status") {
      this.#inFlightStatus += 1;
    }
  }

  #request(
    message,
    transfer = [],
    response,
    allowSticky = false,
    expectedPlanes = undefined,
    sourceId = undefined,
  ) {
    if (this.#disposed) return Promise.reject(webError(3, message.requestId));
    if (this.#stickyError !== null && !allowSticky) return Promise.reject(this.#stickyError);
    if (this.#saturated(response, sourceId)) {
      return Promise.reject(webError(RESULT_BACKPRESSURE, message.requestId));
    }
    if (!validRequestId(message.requestId) || message.requestId <= this.#lastRequestId) {
      return Promise.reject(webError(1, message.requestId));
    }
    this.#lastRequestId = message.requestId;
    return new Promise((resolve, reject) => {
      const pending = {
        requestId: message.requestId,
        sourceId,
        resolve,
        reject,
        response,
        planeShape: expectedPlanes === undefined ? undefined : planeShape(expectedPlanes),
      };
      this.#pending.set(pending.requestId, pending);
      this.#reserve(response, sourceId);
      try {
        this.#port.postMessage(message, transfer);
      } catch (error) {
        this.#release(pending);
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
    return this.#request(
      { tag: "miso.source.v1", ...request },
      transfer,
      "source",
      false,
      request.planes,
      request.sourceId,
    );
  }

  seekSource(request) {
    if (!hasExactFields(request, SEEK_FIELDS)
        || typeof request.sourceId !== "string"
        || typeof request.generation !== "bigint" || request.generation <= 0n
        || typeof request.sourceFrame !== "bigint" || request.sourceFrame < 0n) {
      return Promise.reject(webError(1, request?.requestId ?? 0));
    }
    return this.#request(
      { tag: "miso.seek.v1", ...request },
      [],
      "seek",
      false,
      undefined,
      request.sourceId,
    );
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
    try { this.#port.close?.(); } catch (_) { /* already closed */ }
    try { this.node.disconnect(); } catch (_) { /* never connected or already disconnected */ }
  }
}

export async function createMisoAudioWorkletHost(options) {
  if (!hasExactFields(options, OPTION_FIELDS)
      || options.context?.state !== "suspended"
      || !validU32(options.quantumFrames) || options.quantumFrames === 0
      || !validU32(options.context?.sampleRate) || options.context.sampleRate === 0
      || !(options.sessionToml instanceof Uint8Array)
      || !validLimits(options.limits)
      || options.sessionToml.byteLength > options.limits.sessionTomlBytes
      || typeof options.simd128ModuleUrl !== "string"
      || typeof options.workletModuleUrl !== "string") {
    throw webError(1);
  }
  const exposedQuantum = options.context.renderQuantumSize;
  if (typeof exposedQuantum === "number" && exposedQuantum !== 0
      && exposedQuantum !== options.quantumFrames) {
    throw webError(9);
  }
  // W4-D1: attest before allocating anything. Thrown outside the `try` below so it reaches the
  // caller as itself rather than being folded into the generic 255 rejection.
  if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");
  let node;
  try {
    const selected = {
      backend: SHIPPING_BACKEND,
      module: await fetchModule(options.simd128ModuleUrl),
    };
    await options.context.audioWorklet.addModule(options.workletModuleUrl);
    node = new AudioWorkletNode(options.context, PROCESSOR_NAME, {
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
            && message.result === RESULT_OK && message.backend === selected.backend
            && Number.isSafeInteger(message.memoryBytes) && message.memoryBytes > 0
            && validResources(
              message.resources,
              selected.backend,
              options.context.sampleRate,
              options.quantumFrames,
            )) {
          finish(resolve, {
            ...message,
            resources: Object.freeze(message.resources),
          });
        } else if (hasExactFields(message, ["tag", "requestId", "result"])
            && message.tag === "miso.error.v1" && message.requestId === 0
            && validResult(message.result)) {
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
      options.context.sampleRate,
      options.quantumFrames,
      ready.resources,
      ready.memoryBytes,
      // The per-source in-flight bound is the ring depth in quanta. `validLimits` has already
      // established that both are nonzero u32s, and `validate_config` in Rust rejects a ring that
      // is not a whole number of quanta, so this division is exact for any session that prepares.
      Math.floor(options.limits.sourceRingFrames / options.quantumFrames) || 1,
    );
  } catch (error) {
    cleanupNode(node);
    if (isUnsupportedBrowser(error)) throw error;
    if (error?.tag === "miso.error.v1" && hasExactFields(error, ["tag", "requestId", "result"])
        && error.requestId === 0 && validResult(error.result)) throw error;
    throw webError(255);
  }
}
