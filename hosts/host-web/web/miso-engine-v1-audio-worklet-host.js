const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_BACKPRESSURE = 6;
const RESULT_UNSUPPORTED = 7;
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";
// Issue #1332 S7 (D2): the control Worker of `worker` mode, resolved beside the worklet module URL.
const CONTROL_WORKER_FILE = "miso-engine-v1-control-worker.js";
// Issue #1380 D1: the shared memory's pages, the ABI layout document's `memory.initialPages` and
// `memory.maximumPages`. In `worker` mode this realm creates the one memory both instances share.
const MEMORY_INITIAL_PAGES = 20;
const MEMORY_MAXIMUM_PAGES = 8192;
// Issue #1332 S7 (D7): in `worker` mode the page's requests go to the control Worker, except these,
// which the worklet answers for its render half.
const RENDER_PORT_TAGS = new Set(["miso.meters.v1", "miso.telemetry.v1", "miso.renderallocations.v1"]);
const READY_FIELDS = [
  "tag", "requestId", "result", "backend", "resources", "memoryBytes", "sourceRingFrames",
];
const VALID_RESULTS = new Set([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255]);

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

const HOST_ARRAY_IS_ARRAY = Array.isArray;
const HOST_OBJECT_PROTOTYPE = Object.prototype;
const HOST_OBJECT_GET_PROTOTYPE_OF = Object.getPrototypeOf;
const HOST_REFLECT_OWN_KEYS = Reflect.ownKeys;
const HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR = Object.getOwnPropertyDescriptor;
const HOST_HAS_OWN = Object.prototype.hasOwnProperty;
const HOST_UINT8_ARRAY = Uint8Array;
const HOST_UINT8_ARRAY_SET = Uint8Array.prototype.set;
const HOST_TYPED_ARRAY_PROTOTYPE = HOST_OBJECT_GET_PROTOTYPE_OF(HOST_UINT8_ARRAY.prototype);
const HOST_TYPED_ARRAY_TAG_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  HOST_TYPED_ARRAY_PROTOTYPE,
  Symbol.toStringTag,
).get;
const HOST_TYPED_ARRAY_BUFFER_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  HOST_TYPED_ARRAY_PROTOTYPE,
  "buffer",
).get;
const HOST_TYPED_ARRAY_BYTE_OFFSET_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  HOST_TYPED_ARRAY_PROTOTYPE,
  "byteOffset",
).get;
const HOST_TYPED_ARRAY_BYTE_LENGTH_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  HOST_TYPED_ARRAY_PROTOTYPE,
  "byteLength",
).get;
const HOST_TYPED_ARRAY_LENGTH_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  HOST_TYPED_ARRAY_PROTOTYPE,
  "length",
).get;
const HOST_ARRAY_BUFFER_BYTE_LENGTH_GETTER = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(
  ArrayBuffer.prototype,
  "byteLength",
).get;

// The one frozen shipping backend (issue 024 `BACKEND_SIMD128`). It is still sent explicitly so
// the processor can cross-check it against the backend row the Rust artifact reports: a module
// built without `+simd128` reports `0` and is rejected transactionally rather than rendered with.
const SHIPPING_BACKEND = "simd128";

const OPTION_FIELDS = [
  "context",
  "document",
  "options",
  "simd128ModuleUrl",
  "workletModuleUrl",
];
const BOOT_OPTION_FIELDS = [
  "sourceRingFrames", "maximumMemoryBytes", "reserved1", "liveControlMeterBlocks",
  "reserved3", "reserved2",
];

const SOURCE_FIELDS = [
  "sourceId",
  "generation",
  "startFrame",
  "sampleRateHz",
  "planes",
  "frames",
  "endOfRegion",
];

const SEEK_FIELDS = ["sourceId", "generation", "sourceFrame"];
// Issue #1644 D1: a session seek names its generation and frame, and optionally its anchor.
const SESSION_SEEK_FIELDS = ["generation", "frame"];
const ANCHORED_SESSION_SEEK_FIELDS = ["generation", "frame", "anchorSample"];
// Issue #1297a: an anchored source seek, a seek report request, and the report's words.
const SEEK_AT_FIELDS = ["sourceId", "generation", "sourceFrame", "anchorSample"];
// Issue #2116: the plan latency reply's words, as the C ABI's plan resource report carries them.
const PLAN_LATENCY_FIELDS = ["samples", "tailKind", "tailSamples"];
const SEEK_REPORT_FIELDS = [
  "generation", "firstSample", "sourceFrame", "cumulativeUnderrunFrames", "cumulativeHeldFrames",
];
// Issue #1562a D1: the control frames in flight on the port at once. The worklet answers them in
// arrival order; a flood is refused here with `RESULT_BACKPRESSURE` before any transfer.
const MAXIMUM_CONTROL_IN_FLIGHT = 64;
// Issue #1332 S8: how long a dispose waits for its reply before the page tears down directly and
// rejects with `miso.teardown.v1` `dispose-timeout`. A live worklet (`single` mode) or Worker
// answers in milliseconds; in `worker` mode the Worker itself gives up on the worklet's release
// after 2 s and answers `release-timeout`, so 5 s leaves it room to dispose and answer first. A
// browser may run the timer late (a throttled background tab), which only lengthens the wait.
const DISPOSE_TIMEOUT_MS = 5000;
// The engine's `maximum_control_frame_bytes` (`CONTROL_FRAME_BYTES` in the Rust host).
const MAXIMUM_CONTROL_FRAME_BYTES = 16384;
// Issue #207: one entry of the session map's source list. The names are the `.d.ts`'s and the
// worklet's, spelled once here so the acknowledgement validator and the type declaration cannot
// drift apart without `scripts/check-session-map-shape.py` seeing it.
const SESSION_SOURCE_FIELDS = ["id", "channels", "frames"];

const RESOURCE_FIELDS = [
  "sampleRateHz", "quantumFrames", "backend", "optionsBytes", "statusBytes", "sessionDocumentBytes",
  "diagnosticBytes", "idStagingBytes", "sourcePcmStagingBytes", "outputPcmBytes",
  "bridgeMetadataBytes", "bridgeRetainedBytes", "largestBridgeAllocationBytes",
  "sourceTotalBytes", "sourceOverheadBytes", "effectScalarStateBytes", "effectScalarScratchBytes",
  "builtinRetainedBytes", "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes",
  "graphMetadataBytes", "graphDelayBytes", "largestNamedAllocationBytes",
  // Issue #143: carved from the report's first reserved word.
  "observationRetainedBytes",
];
const RESOURCE_U64_FIELDS = RESOURCE_FIELDS.slice(3);

function hasExactFields(value, fields) {
  if (value === null || typeof value !== "object") return false;
  const keys = Object.keys(value).sort();
  const expected = [...fields].sort();
  return keys.length === expected.length && keys.every((key, index) => key === expected[index]);
}

function capturePlainRecord(value, fields) {
  if (value === null || typeof value !== "object" || HOST_ARRAY_IS_ARRAY(value)) return null;
  const prototype = HOST_OBJECT_GET_PROTOTYPE_OF(value);
  if (prototype !== HOST_OBJECT_PROTOTYPE && prototype !== null) return null;
  const known = new Set(fields);
  const descriptors = new Map();
  let enumerableCount = 0;
  const keys = HOST_REFLECT_OWN_KEYS(value);
  for (let index = 0; index < keys.length; index += 1) {
    const key = keys[index];
    if (typeof key !== "string" || !known.has(key)) return null;
    const descriptor = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(value, key);
    if (descriptor === undefined
        || !HOST_HAS_OWN.call(descriptor, "value")
        || !HOST_HAS_OWN.call(descriptor, "enumerable")) return null;
    descriptors.set(key, { value: descriptor.value, enumerable: descriptor.enumerable });
    if (descriptor.enumerable) enumerableCount += 1;
  }
  return { descriptors, enumerableCount };
}

function hasEnumerableShape(captured, fields) {
  if (captured.enumerableCount !== fields.length) return false;
  return hasRequiredEnumerableFields(captured, fields);
}

function hasRequiredEnumerableFields(captured, fields) {
  for (let index = 0; index < fields.length; index += 1) {
    const descriptor = captured.descriptors.get(fields[index]);
    if (descriptor === undefined || !descriptor.enumerable) return false;
  }
  return true;
}

function snapshotDocument(value) {
  if (value === null || typeof value !== "object") return undefined;
  try {
    if (HOST_TYPED_ARRAY_TAG_GETTER.call(value) !== "Uint8Array") return undefined;
    const backing = HOST_TYPED_ARRAY_BUFFER_GETTER.call(value);
    const backingByteLength = HOST_ARRAY_BUFFER_BYTE_LENGTH_GETTER.call(backing);
    const byteOffset = HOST_TYPED_ARRAY_BYTE_OFFSET_GETTER.call(value);
    const byteLength = HOST_TYPED_ARRAY_BYTE_LENGTH_GETTER.call(value);
    const length = HOST_TYPED_ARRAY_LENGTH_GETTER.call(value);
    if (byteOffset > backingByteLength || byteLength > backingByteLength - byteOffset
        || byteLength !== length) return undefined;
    const copy = new HOST_UINT8_ARRAY(length);
    HOST_UINT8_ARRAY_SET.call(copy, value);
    return copy;
  } catch (_) {
    return undefined;
  }
}

function snapshotBootOptions(value) {
  const captured = capturePlainRecord(value, BOOT_OPTION_FIELDS);
  if (!captured || !hasEnumerableShape(captured, BOOT_OPTION_FIELDS)) return undefined;
  const snapshot = {
    sourceRingFrames: captured.descriptors.get("sourceRingFrames").value,
    maximumMemoryBytes: captured.descriptors.get("maximumMemoryBytes").value,
    reserved1: captured.descriptors.get("reserved1").value,
    liveControlMeterBlocks: captured.descriptors.get("liveControlMeterBlocks").value,
    reserved3: captured.descriptors.get("reserved3").value,
    reserved2: captured.descriptors.get("reserved2").value,
  };
  return validBootOptions(snapshot) ? snapshot : undefined;
}

// The optional factory fields: each is absent or an enumerable own value. `bootCommand` (#2166) is
// one encoded SPECTRUM_CONFIGURE command frame the engine applies before it prepares the first
// plan, so the boot's spectrum taps are that plan's and the boot counts no blocking rebuild.
// `instanceMode` (#1332 S7) is `"single"` to run one instance on an isolated page too (the SDK's
// PCM feed, whose drain runs in the worklet, until the Worker drains it), or `"worker"`, which a
// page without cross-origin isolation refuses. Absent: `"worker"` exactly when the page is isolated.
const OPTIONAL_FIELDS = ["preparedModule", "bootCommand", "instanceMode"];

function snapshotFactory(value) {
  const captured = capturePlainRecord(value, [...OPTION_FIELDS, ...OPTIONAL_FIELDS]);
  if (!captured || !hasRequiredEnumerableFields(captured, OPTION_FIELDS)) return undefined;
  let present = 0;
  const optional = {};
  for (let index = 0; index < OPTIONAL_FIELDS.length; index += 1) {
    const descriptor = captured.descriptors.get(OPTIONAL_FIELDS[index]);
    const field = descriptor === undefined ? undefined : descriptor.value;
    if (field === undefined) continue;
    if (!descriptor.enumerable) return undefined;
    present += 1;
    optional[OPTIONAL_FIELDS[index]] = field;
  }
  if (captured.enumerableCount !== OPTION_FIELDS.length + present) return undefined;
  let bootCommand;
  if (optional.bootCommand !== undefined) {
    bootCommand = snapshotDocument(optional.bootCommand);
    if (bootCommand === undefined || bootCommand.byteLength === 0) return undefined;
  }
  return {
    descriptors: captured.descriptors,
    preparedModule: optional.preparedModule,
    bootCommand,
    instanceMode: optional.instanceMode,
  };
}

function deferred() {
  let resolve;
  let reject;
  const promise = new Promise((res, rej) => { resolve = res; reject = rej; });
  return { promise, resolve, reject };
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

function validBootOptions(options) {
  return hasExactFields(options, BOOT_OPTION_FIELDS)
    && validU32(options.sourceRingFrames)
    && validU64(options.maximumMemoryBytes)
    && options.reserved1 === 0n
    && validU64(options.liveControlMeterBlocks)
    && options.liveControlMeterBlocks <= 0xffffffffn
    && options.reserved3 === 0n
    && options.reserved2 === 0n;
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

/// D6: the typed refusal of a control Worker that raised `error` or `messageerror` before its boot
/// reply. The page never falls back to `single` mode silently.
function workerFailed() {
  return Object.freeze({
    tag: "miso.worker-failed.v1",
    requestId: 0,
    result: 255,
    reason: "worker-failed",
  });
}

function isWorkerFailed(value) {
  return value?.tag === "miso.worker-failed.v1" && value.reason === "worker-failed"
    && hasExactFields(value, ["tag", "requestId", "result", "reason"]);
}

/// Issue #1332 S8: the typed reason a dispose ended in a teardown nobody answered. `context-closed`:
/// no dispose reply within `DISPOSE_TIMEOUT_MS` and the context is closed; `dispose-timeout`: no
/// reply within that bound on a context that is not; `release-timeout` (from the control Worker, with the dispose's request
/// ID): the worklet never released its render half, and the Worker disposed without it. The page
/// has terminated the Worker and disconnected the node either way.
function teardown(reason, requestId = 0) {
  return Object.freeze({ tag: "miso.teardown.v1", requestId, result: 255, reason });
}

function isTeardown(value) {
  return value?.tag === "miso.teardown.v1"
    && hasExactFields(value, ["tag", "requestId", "result", "reason"]);
}

// The main realm's one timer site (#1381a D6, #1332 S8): the single-mode service tick, and the
// dispose deadline in both modes.
// SERVICE_TICK_BEGIN
function startTimer(periodMs, run) {
  return setInterval(run, periodMs);
}
// SERVICE_TICK_END

/// A deadline that fires once, after `periodMs`, unless it is cancelled first.
function deadline(periodMs) {
  let timer = null;
  const promise = new Promise((resolve) => {
    timer = startTimer(periodMs, () => {
      clearInterval(timer);
      resolve(true);
    });
  });
  return { promise, cancel: () => clearInterval(timer) };
}

/// Issue #1332 S8: the factory failed after the control Worker booted and the worklet was built
/// (its construction failed, perhaps after it adopted the render half). Ask the Worker to dispose
/// -- it takes the half back from the worklet, which released it, or from the registry -- and
/// wait for the answer, bounded; the caller terminates the Worker next either way.
async function disposeBootedWorker(worker) {
  const timer = deadline(DISPOSE_TIMEOUT_MS);
  const answered = new Promise((resolve) => {
    worker.onmessage = (event) => { if (event.data?.requestId === 1) resolve(false); };
    worker.onerror = (event) => {
      event?.preventDefault?.();
      resolve(false);
    };
    worker.onmessageerror = () => resolve(false);
  });
  try {
    worker.postMessage({ tag: "miso.dispose.v1", requestId: 1 });
    await Promise.race([answered, timer.promise]);
  } catch (_) {
    // A Worker that cannot take the message is terminated next.
  } finally {
    timer.cancel();
    worker.onmessage = null;
    worker.onerror = null;
    worker.onmessageerror = null;
  }
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
/// is never refused for being in flight: concurrent calls share one read, and a reply reflects
/// every request the host sent before the call (see `status()`). Dispose is terminal and
/// waits for nothing: the worklet handles messages in arrival order, so every earlier request is
/// already settled by the time its acknowledgement is posted.
///
/// Nothing is dropped silently: a request over its bound is rejected locally, before transfer, with
/// a typed `RESULT_BACKPRESSURE` and its planes still owned by the caller.
class MisoAudioWorkletHost {
  #port;
  /// The worklet's port: the same as `#port` in `single` mode; in `worker` mode `#port` is the
  /// control Worker and this port carries only `RENDER_PORT_TAGS`.
  #renderPort;
  #worker;
  #pending = new Map();
  #inFlightSources = new Map();
  #inFlightSeeks = new Map();
  /// Issue #1644 D1: one session seek may be unsettled at a time.
  #inFlightSessionSeeks = 0;
  /// Issue #1297a: the sources with a seek report request unsettled (one each).
  #inFlightSeekReports = new Set();
  /// The one status request on the port, `{ requestId, promise }`, or null. A `status()` call joins
  /// it only while no later request was sent; otherwise it joins `#statusFollow`, the one pending
  /// read the host sends when this one settles.
  #statusRead = null;
  #statusFollow = null;
  #inFlightRenderAllocations = 0;
  /// Issue #2116: one plan latency read may be unsettled at a time.
  #inFlightPlanLatency = 0;
  #inFlightControl = 0;
  #inFlightLease = new Set();
  #ringBlocks;
  #onMeterFrame = null;
  #onTelemetryFrame = null;
  /// Issue #1570b D5: the listeners of the engine's protocol events, in subscription order, and the
  /// count of frames that arrived with none.
  #controlEventListeners = new Set();
  #discardedControlEvents = 0;
  /// Issue #1294a: the listeners of the applied-revision watermark, in subscription order.
  #watermarkListeners = new Set();
  #lastRequestId = 0;
  #stickyError = null;
  #disposed = false;
  /// The one dispose in flight or done: a second `dispose()` returns the first one's promise.
  #disposePromise = null;
  /// Issue #1332 S8: the control Worker raised `error` after its boot and was terminated.
  #workerGone = false;
  #serviceTick = null;
  #numericBackend;
  #sampleRateHz;
  #quantumFrames;

  constructor(
    node,
    backend,
    sampleRateHz,
    quantumFrames,
    resources,
    memoryBytes,
    ringBlocks,
    instanceMode = "single",
    worker = undefined,
    early = undefined,
  ) {
    Object.defineProperties(this, {
      node: { value: node, enumerable: true },
      backend: { value: backend, enumerable: true },
      resources: { value: Object.freeze(resources), enumerable: true },
      memoryBytes: { value: memoryBytes, enumerable: true },
      // Issue #1332 D1: where the control plane runs, `"worker"` or `"single"`.
      instanceMode: { value: instanceMode, enumerable: true },
    });
    this.#numericBackend = numericBackend(backend);
    this.#sampleRateHz = sampleRateHz;
    this.#quantumFrames = quantumFrames;
    this.#ringBlocks = ringBlocks;
    this.#renderPort = node.port;
    this.#worker = worker;
    this.#port = worker ?? node.port;
    for (const port of worker === undefined ? [node.port] : [node.port, worker]) {
      port.onmessage = (event) => this.#receive(event.data);
      port.onmessageerror = () => this.#fail(webError(255, this.#oldestRequestId()));
    }
    if (worker !== undefined) {
      worker.onerror = (event) => {
        event?.preventDefault?.();
        this.#failWorker();
      };
    }
    // A user-agent/processor crash cannot return storage already transferred out of this realm.
    this.node.onprocessorerror = () => this.#fail(webError(255, this.#oldestRequestId()));
    // In `worker` mode the control Worker runs its own service tick (#1332 S7).
    if (worker === undefined) this.#startServiceTick();
    // What the Worker posted, or how it failed, before this host took it over (#1332 S7).
    for (const message of early?.messages ?? []) this.#receive(message);
    if (early?.failed === "error") this.#failWorker();
    else if (early?.failed === "messageerror") this.#fail(webError(255, this.#oldestRequestId()));
  }

  /// Issue #1332 S8: a Worker `error` after the boot fails the host and terminates the Worker; a
  /// later dispose then tears down at once (nobody is left to answer it).
  #failWorker() {
    if (this.#workerGone) return;
    this.#workerGone = true;
    this.#worker.onerror = null;
    this.#worker.onmessage = null;
    this.#worker.onmessageerror = null;
    try { this.#worker.terminate(); } catch (_) { /* already gone */ }
    this.#fail(webError(255, this.#oldestRequestId()));
  }

  // Issue #1381a D6: in `single` mode the instance's control work runs in the worklet's message
  // handler, so the worklet needs a message to run its service step on. This is that message: one
  // `miso.service.v1` per quantum, from the one timer the main realm may hold. The browser may
  // clamp the period; no deadline depends on it, because deadlines are counted in render samples.
  // The loop starts when boot succeeds and stops before dispose, and on a failure.
  #startServiceTick() {
    const periodMs = Math.max(1, Math.round((1000 * this.#quantumFrames) / this.#sampleRateHz));
    this.#serviceTick = startTimer(periodMs, () => this.#postServiceTick());
  }

  #postServiceTick() {
    if (this.#disposed || this.#stickyError !== null) {
      this.#stopServiceTick();
      return;
    }
    try {
      this.#port.postMessage({ tag: "miso.service.v1" });
    } catch (_) {
      this.#stopServiceTick();
    }
  }

  #stopServiceTick() {
    if (this.#serviceTick === null) return;
    clearInterval(this.#serviceTick);
    this.#serviceTick = null;
  }

  // Request IDs are strictly monotonic and the worklet handles messages in arrival order, so the
  // first key of the insertion-ordered map is the oldest unsettled request.
  #oldestRequestId() {
    for (const requestId of this.#pending.keys()) return requestId;
    return 0;
  }

  #allocateRequestId() {
    if (this.#lastRequestId >= Number.MAX_SAFE_INTEGER) return null;
    this.#lastRequestId += 1;
    return this.#lastRequestId;
  }

  #release(pending) {
    this.#pending.delete(pending.requestId);
    if (pending.response === "source") {
      const count = (this.#inFlightSources.get(pending.sourceId) ?? 1) - 1;
      if (count <= 0) this.#inFlightSources.delete(pending.sourceId);
      else this.#inFlightSources.set(pending.sourceId, count);
    } else if (pending.response === "seek") {
      this.#inFlightSeeks.delete(pending.sourceId);
    } else if (pending.response === "sessionSeek") {
      this.#inFlightSessionSeeks -= 1;
    } else if (pending.response === "seekReport") {
      this.#inFlightSeekReports.delete(pending.sourceId);
    } else if (pending.response === "status") {
      if (this.#statusRead?.requestId === pending.requestId) {
        this.#statusRead = null;
        this.#sendStatusFollow();
      }
    } else if (pending.response === "renderAllocations") {
      this.#inFlightRenderAllocations -= 1;
    } else if (pending.response === "planLatency") {
      this.#inFlightPlanLatency -= 1;
    } else if (pending.response === "control") {
      this.#inFlightControl -= 1;
    } else if (pending.response === "lease" || pending.response === "sessionMap") {
      this.#inFlightLease.delete(pending.leaseKind);
    }
  }

  #receive(message) {
    // Issue #137 D2/D3: meter and telemetry frames are unsolicited -- they answer a lease, not a
    // request -- so they are dispatched before the correlation lookup and never reach it. A frame
    // whose shape is wrong is a hard failure exactly like a malformed acknowledgement: a console
    // that silently ignores a broken frame is a console that lies to its user.
    if (message?.tag === "miso.meter.v1") {
      // Issue #143 D5: the gain-reduction section and the peak sample window ride the same frame;
      // GR remains a latest independently aged fold and does not inherit the peak timestamps.
      // Every rule below is a shape rule the app is entitled to rely on without checking:
      // `trackGrDb` is exactly `trackCount` long, every entry is a finite non-negative magnitude,
      // `masterGrDb` is a finite number or `null` -- never `0` standing in for absence -- and the
      // window is half-open and non-empty. Issue #1209 D4: the buses ride three appended fields,
      // `submixPeaks` a pair per bus and `submixGrDb` one magnitude per bus, every value finite
      // and non-negative.
      if (!hasExactFields(message, [
        "tag", "sequence", "generation", "validity", "lossCount", "windows", "trackCount",
        "structureEpoch", "peaks", "trackGrDb", "masterGrDb", "firstSample", "endSample",
        "submixCount", "submixPeaks", "submixGrDb",
      ])
          || !Number.isSafeInteger(message.sequence) || message.sequence <= 0
          || typeof message.generation !== "bigint" || message.generation <= 0n
          || !Number.isSafeInteger(message.validity) || message.validity < 0
          || message.validity > 0xf || (message.validity & 0x3) !== 0x3
          || !Number.isSafeInteger(message.lossCount) || message.lossCount < 0
          || message.lossCount > 0xffffffff
          || ((message.validity & 0x4) !== 0) !== (message.lossCount > 0)
          || !Number.isSafeInteger(message.windows) || message.windows <= 0
          || !Number.isSafeInteger(message.trackCount) || message.trackCount < 0
          || !Number.isSafeInteger(message.structureEpoch) || message.structureEpoch <= 0
          || !(message.peaks instanceof Float32Array)
          || message.peaks.length !== message.trackCount * 2 + 2
          || !(message.trackGrDb instanceof Float32Array)
          || message.trackGrDb.length !== message.trackCount
          || !message.trackGrDb.every((value) => Number.isFinite(value) && value >= 0)
          || !(message.masterGrDb === null
            || (typeof message.masterGrDb === "number" && Number.isFinite(message.masterGrDb)
              && message.masterGrDb >= 0))
          || typeof message.firstSample !== "bigint" || typeof message.endSample !== "bigint"
          || message.firstSample < 0n || message.endSample <= message.firstSample
          || !Number.isSafeInteger(message.submixCount) || message.submixCount < 0
          || !(message.submixPeaks instanceof Float32Array)
          || message.submixPeaks.length !== message.submixCount * 2
          || !message.submixPeaks.every((value) => Number.isFinite(value) && value >= 0)
          || !(message.submixGrDb instanceof Float32Array)
          || message.submixGrDb.length !== message.submixCount
          || !message.submixGrDb.every((value) => Number.isFinite(value) && value >= 0)) {
        this.#fail(webError(255, this.#oldestRequestId()));
        return;
      }
      this.#onMeterFrame?.(Object.freeze(message));
      return;
    }
    if (message?.tag === "miso.telemetry.v1") {
      if (!hasExactFields(message, [
        "tag", "sequence", "blocks", "cpuPercent", "peakBlockMs", "meanBlockMs", "budgetMs",
        "deadlineMisses", "resolutionMs", "belowResolution",
      ])
          || !Number.isSafeInteger(message.sequence) || message.sequence <= 0
          || !Number.isSafeInteger(message.blocks) || message.blocks <= 0
          || !Number.isSafeInteger(message.deadlineMisses) || message.deadlineMisses < 0
          || typeof message.belowResolution !== "boolean"
          || ["cpuPercent", "peakBlockMs", "meanBlockMs", "budgetMs", "resolutionMs"]
            .some((field) => typeof message[field] !== "number"
              || !Number.isFinite(message[field]) || message[field] < 0)) {
        this.#fail(webError(255, this.#oldestRequestId()));
        return;
      }
      this.#onTelemetryFrame?.(Object.freeze(message));
      return;
    }
    // Issue #1570b D5: a protocol event of the engine's control plane is unsolicited too. It is
    // delivered whole and in arrival order, with the lane it came from; an event is not the ack of
    // a command (the command's reply is), so a frame that has no listener is dropped and counted,
    // and no engine state changes with it.
    if (message?.tag === "miso.control-event.v1") {
      if (!hasExactFields(message, ["tag", "lane", "frame"])
          || (message.lane !== 0 && message.lane !== 1)
          || !(message.frame instanceof Uint8Array)
          || !(message.frame.buffer instanceof ArrayBuffer)
          || message.frame.byteLength === 0
          || message.frame.byteLength > MAXIMUM_CONTROL_FRAME_BYTES) {
        this.#fail(webError(255, this.#oldestRequestId()));
        return;
      }
      if (this.#controlEventListeners.size === 0) {
        this.#discardedControlEvents += 1;
        return;
      }
      const event = Object.freeze({ lane: message.lane, frame: message.frame });
      for (const listener of [...this.#controlEventListeners]) listener(event);
      return;
    }
    // Issue #1294a: the worklet posts the applied-revision watermark each time it changes. It is
    // state, not an event: the newest post supersedes every earlier one, so one with no listener
    // is dropped uncounted.
    if (message?.tag === "miso.watermark.v1") {
      const words = [
        "revision", "sample", "outcome", "exactCount", "transitionFallbackCount", "supersededCount",
      ];
      if (!hasExactFields(message, ["tag", ...words])
          || words.some((word) => typeof message[word] !== "bigint" || message[word] < 0n
            || message[word] > 0xffffffffffffffffn)) {
        this.#fail(webError(255, this.#oldestRequestId()));
        return;
      }
      const watermark = Object.freeze({
        revision: message.revision,
        sample: message.sample,
        outcome: message.outcome,
        exactCount: message.exactCount,
        transitionFallbackCount: message.transitionFallbackCount,
        supersededCount: message.supersededCount,
      });
      for (const listener of [...this.#watermarkListeners]) listener(watermark);
      return;
    }
    const pending = this.#pending.get(message?.requestId) ?? null;
    // Issue #1332 S8: the control Worker answers a dispose whose release timed out with the typed
    // teardown; the dispose rejects with it (and the page tears down).
    if (message?.tag === "miso.teardown.v1" && this.#worker !== undefined) {
      if (pending?.response === "dispose" && isTeardown(message) && message.result === 255
          && message.reason === "release-timeout") {
        this.#release(pending);
        pending.reject(Object.freeze(message));
      } else {
        this.#fail(webError(255, pending?.requestId ?? this.#oldestRequestId()));
      }
      return;
    }
    const errorFields = message?.planes === undefined
      ? ["tag", "requestId", "result"]
      : ["tag", "requestId", "result", "planes"];
    // Issue #1332 S8: in `worker` mode the worklet posts a sticky render failure, which belongs to
    // no request, as request ID 0: the host fails with it, typed.
    if (message?.tag === "miso.error.v1" && this.#worker !== undefined && message.requestId === 0
        && hasExactFields(message, ["tag", "requestId", "result"])
        && validResult(message.result) && message.result !== RESULT_OK) {
      this.#fail(Object.freeze(message));
      return;
    }
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
      : pending.response === "control"
          ? ["tag", "requestId", "result", "response", "revision"]
        : pending.response === "sessionMap"
          ? ["tag", "requestId", "result", "sources", "metersAttached"]
          : pending.response === "status"
        ? [
          "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
          "quantumFrames", "blockingRebuilds", "nextAbsoluteSample", "renderedQuanta",
          "liveValuesSuperseded", "appliedRevision", "appliedSample", "appliedOutcome", "memoryBytes", "singleModeControlAllocations", "staleViewQuanta",
          "meterFramesSkipped",
        ]
        : pending.response === "renderAllocations"
          ? ["tag", "requestId", "result", "count"]
        : pending.response === "sessionSeek"
          ? ["tag", "requestId", "result", "generation"]
        : pending.response === "seekReport"
          ? ["tag", "requestId", "result", "report"]
        : pending.response === "planLatency"
          ? ["tag", "requestId", "result", "latency"]
        : ["tag", "requestId", "result"];
    const expectedTag = pending.response === "status"
      ? "miso.status.v1"
      : pending.response === "renderAllocations"
        ? "miso.renderallocations.v1"
      : pending.response === "control"
        ? "miso.control.v1"
      : pending.response === "sessionMap"
        ? "miso.sessionmap.v1"
              : pending.response === "sessionSeek"
                  ? "miso.session-seek.v1"
                : pending.response === "seekReport"
                  ? "miso.seek-report.v1"
                : pending.response === "planLatency"
                  ? "miso.plan-latency.v1"
        : "miso.ack.v1";
    const validSourcePlanes = pending.response !== "source"
      || validReturnedPlanes(message.planes, pending.planeShape);
    const validControl = pending.response !== "control" || (
      message.response instanceof Uint8Array
      && message.response.buffer instanceof ArrayBuffer
      && message.response.byteLength <= MAXIMUM_CONTROL_FRAME_BYTES
      && validU64(message.revision)
    );
    const validSessionMap = pending.response !== "sessionMap" || (
      message.result === RESULT_OK
      // Issue #207: the source list is held to what a *compiled* session can produce -- a nonzero
      // channel count and a nonzero full-source frame count -- so a malformed list fails the host
      // loudly instead of reaching a consumer as plausible numbers.
      && Array.isArray(message.sources)
      && message.sources.every((source) => hasExactFields(source, SESSION_SOURCE_FIELDS)
        && typeof source.id === "string" && source.id.length > 0
        && validU32(source.channels) && source.channels > 0
        && validU64(source.frames, true))
      && typeof message.metersAttached === "boolean"
    );
    const validStatus = pending.response !== "status" || (
      message.result === RESULT_OK && validU32(message.state) && message.state <= 4
      && validResult(message.lastResult) && message.backend === this.#numericBackend
      && message.sampleRateHz === this.#sampleRateHz
      && message.quantumFrames === this.#quantumFrames
      && validU32(message.blockingRebuilds)
      && validU64(message.nextAbsoluteSample) && validU64(message.renderedQuanta)
      && validU64(message.liveValuesSuperseded)
      && validU64(message.appliedRevision) && validU64(message.appliedSample)
      && validU64(message.appliedOutcome)
      // The module's memory only grows, and a control call may grow it (counted on the control
      // side, no fault; the worklet re-binds its views), so the live size is the boot size or more,
      // in whole 64 KiB Wasm pages.
      && Number.isSafeInteger(message.memoryBytes) && message.memoryBytes >= this.memoryBytes
      && message.memoryBytes % 65536 === 0
      && validU32(message.singleModeControlAllocations)
      && validU32(message.staleViewQuanta)
      && validU32(message.meterFramesSkipped)
    );
    const validRenderAllocations = pending.response !== "renderAllocations"
      || (message.result === RESULT_OK && validU32(message.count));
    const validSessionSeek = pending.response !== "sessionSeek"
      || message.generation === pending.generation;
    // Issue #1297a: an OK carries every report word, a refusal none.
    const validSeekReport = pending.response !== "seekReport"
      || (message.result === RESULT_OK
        ? hasExactFields(message.report, SEEK_REPORT_FIELDS)
          && SEEK_REPORT_FIELDS.every((field) => validU64(message.report[field]))
        : message.report === null);
    // Issue #2116: an OK carries the latency and a tail kind of 0 (finite) or 1 (unbounded, zero
    // samples); a refusal carries none.
    const validPlanLatency = pending.response !== "planLatency"
      || (message.result === RESULT_OK
        ? hasExactFields(message.latency, PLAN_LATENCY_FIELDS)
          && PLAN_LATENCY_FIELDS.every((field) => validU64(message.latency[field]))
          && (message.latency.tailKind === 0n
            || (message.latency.tailKind === 1n && message.latency.tailSamples === 0n))
        : message.latency === null);
    if (message.tag !== expectedTag || !hasExactFields(message, expectedFields)
        || !validRequestId(message.requestId) || !validResult(message.result)
        || !validSourcePlanes || !validStatus || !validRenderAllocations || !validControl || !validSessionMap
        || !validSessionSeek || !validSeekReport || !validPlanLatency) {
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
    this.#stopServiceTick();
    const unsettled = [...this.#pending.values()];
    this.#pending.clear();
    this.#inFlightSources.clear();
    this.#inFlightSeeks.clear();
    this.#inFlightSessionSeeks = 0;
    this.#inFlightSeekReports.clear();
    this.#statusRead = null;
    const follow = this.#statusFollow;
    this.#statusFollow = null;
    this.#inFlightRenderAllocations = 0;
    this.#inFlightPlanLatency = 0;
    this.#inFlightControl = 0;
    for (const pending of unsettled) pending.reject(error);
    follow?.reject(error);
  }

  // The per-kind bound. `true` means the request has nowhere to go and is refused locally, before
  // any transfer, so the caller keeps its planes and can retry.
  #saturated(response, sourceId) {
    if (response === "source") {
      return (this.#inFlightSources.get(sourceId) ?? 0) >= this.#ringBlocks;
    }
    if (response === "seek") return this.#inFlightSeeks.has(sourceId);
    if (response === "sessionSeek") return this.#inFlightSessionSeeks >= 1;
    if (response === "seekReport") return this.#inFlightSeekReports.has(sourceId);
    if (response === "renderAllocations") return this.#inFlightRenderAllocations >= 1;
    if (response === "planLatency") return this.#inFlightPlanLatency >= 1;
    if (response === "control") return this.#inFlightControl >= MAXIMUM_CONTROL_IN_FLIGHT;
    if (response === "lease" || response === "sessionMap") {
      return this.#inFlightLease.has(sourceId);
    }
    return false; // dispose is terminal and waits for nothing
  }

  #reserve(response, sourceId) {
    if (response === "source") {
      this.#inFlightSources.set(sourceId, (this.#inFlightSources.get(sourceId) ?? 0) + 1);
    } else if (response === "seek") {
      this.#inFlightSeeks.set(sourceId, true);
    } else if (response === "sessionSeek") {
      this.#inFlightSessionSeeks += 1;
    } else if (response === "seekReport") {
      this.#inFlightSeekReports.add(sourceId);
    } else if (response === "renderAllocations") {
      this.#inFlightRenderAllocations += 1;
    } else if (response === "planLatency") {
      this.#inFlightPlanLatency += 1;
    } else if (response === "control") {
      this.#inFlightControl += 1;
    } else if (response === "lease" || response === "sessionMap") {
      this.#inFlightLease.add(sourceId);
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
    if (this.#disposed) return Promise.reject(webError(3));
    // Once `dispose()` was sent, every other request is refused locally (result 3), as
    // `status()` is: nothing sent behind the dispose can fail the host and reject the dispose.
    if (response !== "dispose" && this.#disposePromise !== null) return Promise.reject(webError(3));
    if (this.#stickyError !== null && !allowSticky) return Promise.reject(this.#stickyError);
    if (this.#saturated(response, sourceId)) {
      return Promise.reject(webError(RESULT_BACKPRESSURE));
    }
    const requestId = this.#allocateRequestId();
    if (requestId === null) {
      return Promise.reject(webError(1));
    }
    const stamped = { ...message, requestId };
    return new Promise((resolve, reject) => {
      const pending = {
        requestId,
        sourceId,
        leaseKind: sourceId,
        generation: stamped.generation,
        resolve,
        reject,
        response,
        planeShape: expectedPlanes === undefined ? undefined : planeShape(expectedPlanes),
      };
      this.#pending.set(requestId, pending);
      this.#reserve(response, sourceId);
      try {
        (RENDER_PORT_TAGS.has(stamped.tag) ? this.#renderPort : this.#port)
          .postMessage(stamped, transfer);
      } catch (error) {
        this.#release(pending);
        reject(webError(255, requestId));
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
      return Promise.reject(webError(1));
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
      return Promise.reject(webError(1));
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

  /// @internal Seek one source so that `sourceFrame` enters the graph in the block that starts at
  /// absolute render sample `anchorSample`, a quantum multiple (issue #1297a). The worklet's
  /// message handler applies it, never `process()`. It resolves with the engine's result; a refusal
  /// changes nothing. It shares the per-source bound with `seekSource`: one seek of a source may be
  /// unsettled at a time, and a second is refused locally with `RESULT_BACKPRESSURE`.
  seekSourceAt(request) {
    if (!hasExactFields(request, SEEK_AT_FIELDS) || typeof request.sourceId !== "string"
        || !validU64(request.generation, true) || !validU64(request.sourceFrame)
        || !validU64(request.anchorSample)) {
      return Promise.reject(webError(1));
    }
    return this.#request(
      { tag: "miso.seek-at.v1", ...request },
      [],
      "seek",
      false,
      undefined,
      request.sourceId,
    );
  }

  /// @internal Read where one source landed after its newest seek (issue #1297a): the engine's
  /// result and, for an OK, the record's words as the C ABI reports them; `null` otherwise. The
  /// reserved ID `$timeline` reads the session timeline's report. One read of a source may be
  /// unsettled at a time; a second is refused locally with `RESULT_BACKPRESSURE`.
  seekReport(sourceId) {
    if (typeof sourceId !== "string") return Promise.reject(webError(1));
    return this.#request(
      { tag: "miso.seek-report.v1", sourceId },
      [],
      "seekReport",
      false,
      undefined,
      sourceId,
    );
  }

  /// Read the output latency and tail of the plan that renders now (issue #2116): the engine's
  /// result and, for an OK, the words of the C ABI's plan resource report (`latency_samples`,
  /// `tail_kind`, `tail_samples`); `null` otherwise. A rebuild's successor shows from the first
  /// read after the block that adopts it. One read may be unsettled at a time; a second is refused
  /// locally with `RESULT_BACKPRESSURE`. The worklet answers in its message handler, never in
  /// `process()`.
  planLatency() {
    return this.#request({ tag: "miso.plan-latency.v1" }, [], "planLatency");
  }

  /// @internal Seek the timeline and every source to `frame` under one new `generation`, all or
  /// nothing (issue #1644 D1). Without `anchorSample` it is a plain seek, which declares the
  /// discontinuity in the same call; with one, every consumer applies it in the block that starts
  /// at that absolute render sample, with continuity kept. It resolves with the engine's result and
  /// the generation; a refusal changes nothing. Reposition the PCM producers only after an OK: a
  /// ring seek at this generation is then consumed without a second per-source seek. One session
  /// seek may be unsettled at a time; a second is refused locally with `RESULT_BACKPRESSURE`. The
  /// SDK's `engine.seek` is the public surface.
  seekSession(request) {
    const anchored = hasExactFields(request, ANCHORED_SESSION_SEEK_FIELDS);
    if ((!anchored && !hasExactFields(request, SESSION_SEEK_FIELDS))
        || !validU64(request.generation, true) || !validU64(request.frame)
        || (anchored && !validU64(request.anchorSample))) {
      return Promise.reject(webError(1));
    }
    return this.#request(
      {
        tag: "miso.session-seek.v1",
        generation: request.generation,
        sourceFrame: request.frame,
        anchorSample: anchored ? request.anchorSample : -1n,
      },
      [],
      "sessionSeek",
    );
  }

  /// Read the engine status. Concurrent calls share one read and `status()` is never refused for
  /// being in flight: a call joins the in-flight read while no later request was sent, and
  /// otherwise joins the one follow-up read the host sends when that read settles. Every reply
  /// therefore reflects every request the host sent before the call. A joined read settles every
  /// joined caller with the same reply or the same rejection, and a caller that stops waiting does
  /// not cancel it. A call made once `dispose()` was sent is refused with result 3 and posts nothing.
  status() {
    if (this.#disposePending()) return Promise.reject(webError(3));
    const read = this.#statusRead;
    if (read !== null) {
      if (read.requestId === this.#lastRequestId) return read.promise;
      this.#statusFollow ??= deferred();
      return this.#statusFollow.promise;
    }
    return this.#sendStatus();
  }

  #disposePending() {
    for (const pending of this.#pending.values()) if (pending.response === "dispose") return true;
    return false;
  }

  #sendStatus() {
    const before = this.#lastRequestId;
    const promise = this.#request({ tag: "miso.status.v1" }, [], "status");
    // A request refused or failed to post before it was kept leaves nothing in flight to share.
    const requestId = this.#lastRequestId;
    if (requestId !== before && this.#pending.has(requestId)) {
      this.#statusRead = { requestId, promise };
    }
    return promise;
  }

  #sendStatusFollow() {
    const follow = this.#statusFollow;
    if (follow === null) return;
    this.#statusFollow = null;
    (this.#disposePending() ? Promise.reject(webError(3)) : this.#sendStatus())
      .then(follow.resolve, follow.reject);
  }

  /// Read this instance's render-locked allocator count (issue #1333 D3).
  ///
  /// The count is every allocator call the instance made inside the exports its processor calls
  /// on the render thread after boot: render, source submission and meter polling. Decision 15
  /// requires it to stay zero. It
  /// is per instance, so a caller reads it on each host before that host's `dispose()`. The
  /// worklet answers in its message handler, never in `process()`.
  renderAllocationCount() {
    return this.#request({ tag: "miso.renderallocations.v1" }, [], "renderAllocations");
  }

  /// Send one command frame to the engine's control plane and resolve with its reply (issue
  /// #1562a D1, #1332 D7). The frame is any command frame the C ABI's
  /// `miso_engine_v1_submit_command` accepts: a `SESSION_TRANSACTION_APPLY` carrying `SET_CONTROL`
  /// edits, `CONTROL_LIST_GET`, `CONTROL_STATE_GET`, `SESSION_SNAPSHOT_GET`, the counter and
  /// capability queries. The host decodes nothing in it and the engine decodes it once, in the
  /// worklet's message handler, never in `process()`.
  ///
  /// The reply is `{ result, response, revision }`: `response` is the response frame bytes
  /// unchanged on `RESULT_OK` (a refused transaction is still an ok call with a non-ok response
  /// frame, as on the C ABI), else the refusal's diagnostic text (`code\tpath\n` lines), for
  /// example `web.apply.shape_unavailable` for a preparation that could not hand its plan to the
  /// host (a transaction that needs a plan rebuild is prepared and published, #1562b). `revision` is the committed revision after the call.
  ///
  /// **Request IDs.** The frame carries the sender's own request ID, and the engine's replay cache
  /// refuses a reused ID with different bytes. One allocator must therefore serve every frame sent
  /// through this method (the SDK's transaction queue and any catalog read share it): this method
  /// never rewrites a frame, and a second allocator would collide.
  ///
  /// The frame is copied; the caller keeps its array. A frame over the engine's
  /// `maximum_control_frame_bytes` or one that is empty is refused locally with
  /// `RESULT_INVALID_ARGUMENT`, and more than 64 frames in flight with `RESULT_BACKPRESSURE`.
  ///
  /// **The document** (issue #1697). A `SESSION_REPLACE` frame names a document by its length and
  /// SHA-256; `document` carries those bytes beside the frame, up to the boot's 1 MiB document
  /// bound, and the engine checks, parses and diffs them as the C ABI's
  /// `miso_engine_v1_submit_document_command` does. It is copied too. A `document` that is not a
  /// `Uint8Array` or is empty is refused locally with `RESULT_INVALID_ARGUMENT`; one the engine's
  /// staging refuses (over the bound) answers `RESULT_INVALID_ARGUMENT` with an empty response.
  control(frame, document) {
    if (!(frame instanceof Uint8Array) || frame.byteLength === 0
        || frame.byteLength > MAXIMUM_CONTROL_FRAME_BYTES
        || (document !== undefined
          && (!(document instanceof Uint8Array) || document.byteLength === 0))) {
      return Promise.reject(webError(1));
    }
    const copy = new Uint8Array(frame.byteLength);
    copy.set(frame);
    if (document === undefined) {
      return this.#request({ tag: "miso.control.v1", frame: copy }, [copy.buffer], "control");
    }
    const documentCopy = new Uint8Array(document.byteLength);
    documentCopy.set(document);
    return this.#request(
      { tag: "miso.control.v1", frame: copy, document: documentCopy },
      [copy.buffer, documentCopy.buffer],
      "control",
    );
  }

  /// Read the compiled session's canonical track order (issue #137 D1), its sources (issue #207)
  /// and its canonical submix order (issue #1210 D3).
  ///
  /// This is the addressing authority for the strip positions the meter frame uses: the app never
  /// guesses an index. An `Observe` edit does not use one: it names its tap
  /// by stable key (#1677).
  sessionMap() {
    return this.#request({ tag: "miso.sessionmap.v1" }, [], "sessionMap", false, undefined, "sessionMap");
  }

  /// Take or release the decimated meter lease (issue #137 D2).
  ///
  /// `onFrame` receives every `miso.meter.v1` frame while the lease is held. Passing
  /// `enabled: false` releases it and detaches the callback.
  meters(request) {
    if (!hasExactFields(request, ["enabled", "onFrame"])
        || typeof request.enabled !== "boolean"
        || (request.onFrame !== null && typeof request.onFrame !== "function")) {
      return Promise.reject(webError(1));
    }
    this.#onMeterFrame = request.enabled ? request.onFrame : null;
    return this.#request(
      { tag: "miso.meters.v1", enabled: request.enabled },
      [],
      "lease",
      false,
      undefined,
      "meters",
    );
  }

  /// Take or release the render-telemetry lease (issue #137 D3).
  telemetry(request) {
    if (!hasExactFields(request, ["enabled", "onFrame"])
        || typeof request.enabled !== "boolean"
        || (request.onFrame !== null && typeof request.onFrame !== "function")) {
      return Promise.reject(webError(1));
    }
    this.#onTelemetryFrame = request.enabled ? request.onFrame : null;
    return this.#request(
      { tag: "miso.telemetry.v1", enabled: request.enabled },
      [],
      "lease",
      false,
      undefined,
      "telemetry",
    );
  }

  /// @internal Subscribe to the engine's protocol events (issue #1570b D5). Every listener is
  /// called with `{ lane, frame }` for each event, in arrival order: `lane` is `0` for the reliable
  /// lane and `1` for the lossy lane, and `frame` is the event frame bytes the engine encoded,
  /// unchanged. Returns the unsubscribe function. A frame that arrives while no listener is
  /// subscribed is dropped and counted in `discardedControlEvents`; an event is not the ack of a
  /// command, so the drop changes no engine state.
  onControlEvent(listener) {
    if (typeof listener !== "function") throw webError(1);
    this.#controlEventListeners.add(listener);
    return () => { this.#controlEventListeners.delete(listener); };
  }

  /// @internal Subscribe to the applied-revision watermark (issue #1294a). Every listener is called
  /// with `{ revision, sample, outcome, exactCount, transitionFallbackCount, supersededCount }`
  /// (each a `bigint`) each time the watermark's `(revision, sample)` pair changes: the highest
  /// revision in effect with every one before it, the first sample it was in effect, the outcome
  /// flags the advance covered and the cumulative outcome counts. Returns the unsubscribe function.
  onWatermark(listener) {
    if (typeof listener !== "function") throw webError(1);
    this.#watermarkListeners.add(listener);
    return () => { this.#watermarkListeners.delete(listener); };
  }

  /// @internal The count of engine events that arrived with no listener subscribed (issue #1570b D5).
  get discardedControlEvents() {
    return this.#discardedControlEvents;
  }

  /// The worker-mode dispose contract (#1332 S7, root): the promise is created once and every
  /// later call returns it, settled or not; it resolves for a clean dispose and rejects with the
  /// typed error otherwise, and either way the page terminates the Worker and disconnects the node,
  /// exactly once. A failed worker-mode host is unusable, so there is no retry. In `single` mode a
  /// failed dispose may be retried: the promise is dropped and the service tick it stopped runs
  /// again, unless it ended in a teardown.
  ///
  /// #1332 S8: a dispose ends in teardown even when nobody answers. With no reply within
  /// `DISPOSE_TIMEOUT_MS` the page tears down directly and rejects with `miso.teardown.v1`
  /// (`context-closed` when the context is closed, `dispose-timeout` otherwise); in `worker` mode
  /// the Worker's own bound on the worklet's release answers `release-timeout` first.
  dispose() {
    if (this.#worker !== undefined) {
      this.#disposePromise ??= this.#disposeWorkerMode();
      return this.#disposePromise;
    }
    this.#disposePromise ??= this.#disposeSingleMode().catch((error) => {
      if (!this.#disposed) {
        this.#disposePromise = null;
        if (this.#serviceTick === null) this.#startServiceTick();
      }
      throw error;
    });
    return this.#disposePromise;
  }

  async #disposeWorkerMode() {
    try {
      // A Worker that failed after its boot was terminated: nobody is left to answer.
      if (this.#workerGone) throw this.#stickyError;
      await this.#awaitDispose();
    } finally {
      this.#teardown();
    }
  }

  async #disposeSingleMode() {
    if (this.#disposed) return;
    this.#stopServiceTick();
    try {
      await this.#awaitDispose();
    } catch (error) {
      if (isTeardown(error)) this.#teardown();
      throw error;
    }
    this.#teardown();
  }

  /// Send the dispose and wait for its reply, at most `DISPOSE_TIMEOUT_MS`: past it, the dispose
  /// rejects with `context-closed` when the context is closed by then and `dispose-timeout`
  /// otherwise (the reply, should it come, is dropped by the teardown). A closed context alone
  /// is no failure: an `OfflineAudioContext` is closed once it rendered, and its worklet still
  /// answers.
  async #awaitDispose() {
    const reply = this.#request({ tag: "miso.dispose.v1" }, [], "dispose", true);
    reply.catch(() => undefined);
    const timer = deadline(DISPOSE_TIMEOUT_MS);
    try {
      const timedOut = await Promise.race([reply.then(() => false), timer.promise]);
      if (timedOut) {
        throw teardown(this.node.context?.state === "closed" ? "context-closed" : "dispose-timeout");
      }
    } finally {
      timer.cancel();
    }
  }

  /// Runs once, after the dispose reply (or, in worker mode, its failure).
  #teardown() {
    if (this.#disposed) return;
    this.#disposed = true;
    this.#stopServiceTick();
    // No request promise outlives the host: whatever is still unsettled (a reply the dispose
    // overtook) rejects with the typed disposed error.
    const unsettled = [...this.#pending.values()];
    this.#pending.clear();
    for (const pending of unsettled) pending.reject(webError(3));
    const follow = this.#statusFollow;
    this.#statusFollow = null;
    this.#statusRead = null;
    follow?.reject(webError(3));
    this.#onMeterFrame = null;
    this.#onTelemetryFrame = null;
    this.#controlEventListeners.clear();
    this.#watermarkListeners.clear();
    for (const port of [this.#renderPort, this.#port]) {
      port.onmessage = null;
      port.onmessageerror = null;
    }
    this.node.onprocessorerror = null;
    try { this.#renderPort.close?.(); } catch (_) { /* already closed */ }
    // D4.3: the Worker disposed the host (the only place that frees it) before its reply.
    if (this.#worker !== undefined) {
      this.#worker.onerror = null;
      try { this.#worker.terminate(); } catch (_) { /* already gone */ }
    }
    try { this.node.disconnect(); } catch (_) { /* never connected or already disconnected */ }
  }
}

/// Issue #1332 S7 (D2, D6): start the control Worker beside the worklet module, hand it the module
/// and the shared memory, and wait for its boot reply. A Worker `error` or `messageerror` before
/// the reply rejects with `worker-failed`; a refused boot rejects with the engine's result code,
/// as the `single`-mode worklet's boot does.
function bootControlWorker(workerUrl, boot) {
  const channel = new MessageChannel();
  let worker;
  try {
    // WORKER_SITE: the main realm's one `Worker` construction (D8).
    worker = new Worker(workerUrl);
  } catch (_) {
    return Promise.reject(workerFailed());
  }
  return new Promise((resolve, reject) => {
    let settled = false;
    // Between the boot reply and the host's construction (the worklet's adoption), the Worker
    // may already post (its tick's watermark or events) or fail: the messages are kept in order
    // and a failure is remembered, and the host replays both when it takes the Worker over.
    const early = { messages: [], failed: null };
    const finish = (operation, value) => {
      if (settled) return;
      settled = true;
      if (operation === reject) {
        worker.onmessage = null;
        worker.onerror = null;
        worker.onmessageerror = null;
        try { worker.terminate(); } catch (_) { /* already gone */ }
      } else {
        worker.onmessage = (event) => { early.messages.push(event.data); };
        worker.onerror = (event) => {
          event?.preventDefault?.();
          early.failed = "error";
        };
        worker.onmessageerror = () => { early.failed ??= "messageerror"; };
      }
      operation(value);
    };
    worker.onmessage = (event) => {
      const message = event.data;
      if (message?.tag === "miso.worker-ready.v1"
          && hasExactFields(message, [...READY_FIELDS, "handle", "token", "reservation", "metersAttached"])
          && message.requestId === 0 && message.result === RESULT_OK
          && validU32(message.handle) && message.handle !== 0
          && validU64(message.token, true) && Number(message.token >> 32n) === message.handle
          && validU32(message.reservation) && message.reservation !== 0
          && typeof message.metersAttached === "boolean") {
        finish(resolve, { worker, workletPort: channel.port2, ready: message, early });
      } else if (hasExactFields(message, ["tag", "requestId", "result"])
          && message.tag === "miso.error.v1" && message.requestId === 0
          && validResult(message.result)) {
        finish(reject, Object.freeze(message));
      } else {
        finish(reject, webError(255));
      }
    };
    worker.onerror = (event) => {
      event?.preventDefault?.();
      finish(reject, workerFailed());
    };
    worker.onmessageerror = () => finish(reject, workerFailed());
    try {
      worker.postMessage({ tag: "miso.worker-boot.v1", ...boot, workletPort: channel.port1 },
        [channel.port1]);
    } catch (_) {
      finish(reject, workerFailed());
    }
  });
}

export async function createMisoAudioWorkletHost(options) {
  let factorySnapshot;
  let documentSnapshot;
  let bootOptions;
  let context;
  let contextState;
  let sampleRateHz;
  let quantumFrames;
  let preparedModule;
  let bootCommand;
  let simd128ModuleUrl;
  let workletModuleUrl;
  let requestedMode;
  try {
    factorySnapshot = snapshotFactory(options);
    if (factorySnapshot === undefined) throw new Error("invalid factory");
    const descriptors = factorySnapshot.descriptors;
    context = descriptors.get("context").value;
    contextState = context?.state;
    quantumFrames = context?.renderQuantumSize ?? 128;
    sampleRateHz = context?.sampleRate;
    preparedModule = factorySnapshot.preparedModule;
    bootCommand = factorySnapshot.bootCommand;
    simd128ModuleUrl = descriptors.get("simd128ModuleUrl").value;
    workletModuleUrl = descriptors.get("workletModuleUrl").value;
    requestedMode = factorySnapshot.instanceMode;
    documentSnapshot = snapshotDocument(descriptors.get("document").value);
    bootOptions = snapshotBootOptions(descriptors.get("options").value);
    if ((preparedModule !== undefined && !(preparedModule instanceof WebAssembly.Module))
        || contextState !== "suspended"
        || !validU32(quantumFrames) || quantumFrames === 0
        || !validU32(sampleRateHz) || sampleRateHz === 0
        || documentSnapshot === undefined
        || bootOptions === undefined
        || typeof simd128ModuleUrl !== "string"
        || typeof workletModuleUrl !== "string"
        || (requestedMode !== undefined && requestedMode !== "single" && requestedMode !== "worker")
        || (requestedMode === "worker" && globalThis.crossOriginIsolated !== true)) {
      throw new Error("invalid factory data");
    }
  } catch (_) {
    throw webError(RESULT_INVALID_ARGUMENT);
  }
  // W4-D1: attest before allocating anything. Thrown outside the `try` below so it reaches the
  // caller as itself rather than being folded into the generic 255 rejection.
  if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");
  let node;
  // Issue #1332 D1: `worker` on a cross-origin-isolated page, `single` otherwise.
  const instanceMode = requestedMode
    ?? (globalThis.crossOriginIsolated === true ? "worker" : "single");
  let booted;
  let workerReady;
  let processorOptions = {
    module: undefined,
    document: documentSnapshot,
    options: bootOptions,
    ...(bootCommand === undefined ? {} : { bootCommand }),
  };
  try {
    // Capture this capability only after SIMD attestation, so unsupported browsers do not touch it.
    const audioWorklet = context.audioWorklet;
    const selected = {
      backend: SHIPPING_BACKEND,
      module: preparedModule ?? await fetchModule(simd128ModuleUrl),
    };
    processorOptions.module = selected.module;
    await audioWorklet.addModule(workletModuleUrl);
    if (context.state !== "suspended"
        || context.sampleRate !== sampleRateHz
        || (context.renderQuantumSize ?? 128) !== quantumFrames) {
      throw webError(RESULT_INVALID_ARGUMENT);
    }
    // Issue #1332 D1: the mode is chosen once, here. An isolated page runs the control plane in a
    // Worker on one shared memory; any other page runs one instance, in the worklet.
    if (instanceMode === "worker") {
      const memory = new WebAssembly.Memory({
        initial: MEMORY_INITIAL_PAGES,
        maximum: MEMORY_MAXIMUM_PAGES,
        shared: true,
      });
      const base = globalThis.document?.baseURI ?? globalThis.location?.href;
      const workerUrl = new URL(CONTROL_WORKER_FILE, new URL(workletModuleUrl, base));
      booted = await bootControlWorker(workerUrl, {
        module: selected.module,
        memory,
        document: documentSnapshot,
        options: bootOptions,
        sampleRateHz,
        quantumFrames,
        ...(bootCommand === undefined ? {} : { bootCommand }),
      });
      workerReady = booted.ready;
      if (workerReady.backend !== selected.backend
          || !Number.isSafeInteger(workerReady.memoryBytes) || workerReady.memoryBytes <= 0
          || !validU32(workerReady.sourceRingFrames) || workerReady.sourceRingFrames === 0
          || workerReady.sourceRingFrames % quantumFrames !== 0
          || !validResources(workerReady.resources, selected.backend, sampleRateHz, quantumFrames)) {
        throw webError(255);
      }
      processorOptions = {
        instanceMode,
        module: selected.module,
        memory,
        token: workerReady.token,
        reservation: workerReady.reservation,
        metersAttached: workerReady.metersAttached,
      };
    }
    node = new AudioWorkletNode(context, PROCESSOR_NAME, {
      numberOfInputs: 0,
      numberOfOutputs: 1,
      outputChannelCount: [2],
      processorOptions,
    });
    // D7: the control Worker's channel port reaches the worklet on its own port (processor options
    // cannot transfer one), before any request.
    if (booted !== undefined) {
      node.port.postMessage({ tag: "miso.control-port.v1", port: booted.workletPort },
        [booted.workletPort]);
    }
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
        if (workerReady !== undefined) {
          // `worker` mode: the worklet adopted the render half; the ready fields are the Worker's.
          if (hasExactFields(message, ["tag", "requestId", "result", "backend", "instanceMode"])
              && message.tag === "miso.ready.v1" && message.requestId === 0
              && message.result === RESULT_OK && message.backend === selected.backend
              && message.instanceMode === "worker") {
            finish(resolve, { ...workerReady, resources: Object.freeze(workerReady.resources) });
          } else if (hasExactFields(message, ["tag", "requestId", "result"])
              && message.tag === "miso.error.v1" && message.requestId === 0
              && validResult(message.result)) {
            finish(reject, Object.freeze(message));
          } else {
            finish(reject, webError(255));
          }
          return;
        }
        if (hasExactFields(message, [...READY_FIELDS, "instanceMode"])
            && message.instanceMode === "single"
            && message.tag === "miso.ready.v1" && message.requestId === 0
            && message.result === RESULT_OK && message.backend === selected.backend
            && Number.isSafeInteger(message.memoryBytes) && message.memoryBytes > 0
            && validU32(message.sourceRingFrames) && message.sourceRingFrames > 0
            && message.sourceRingFrames % quantumFrames === 0
            && validResources(
              message.resources,
              selected.backend,
              sampleRateHz,
              quantumFrames,
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
    return new MisoAudioWorkletHost(
      node,
      selected.backend,
      sampleRateHz,
      quantumFrames,
      ready.resources,
      ready.memoryBytes,
      // The per-source in-flight bound is the ring depth in quanta: the ring the engine reports it
      // resolved at boot (issue #1406b), so a zero override gives the engine's own default ring.
      ready.sourceRingFrames / quantumFrames,
      instanceMode,
      booted?.worker,
      booted?.early,
    );
  } catch (error) {
    if (booted !== undefined) {
      // #1332 S8: a worklet was built, so it may have adopted the render half: the Worker takes
      // it back and disposes before it is terminated.
      if (node !== undefined) await disposeBootedWorker(booted.worker);
      try { booted.worker.terminate(); } catch (_) { /* already gone */ }
    }
    cleanupNode(node);
    if (isUnsupportedBrowser(error) || isWorkerFailed(error)) throw error;
    if (error?.tag === "miso.error.v1" && hasExactFields(error, ["tag", "requestId", "result"])
        && error.requestId === 0 && validResult(error.result)) throw error;
    throw webError(255);
  }
}
