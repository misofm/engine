// The control Worker of the browser engine's `worker` mode (issue #1332 S7, D2, D7).
//
// On a cross-origin-isolated page the main realm starts this Worker beside the AudioWorklet and
// gives it the compiled module and the one shared `WebAssembly.Memory`. The Worker instantiates
// the module first (#1331 recipe rule 1), boots the session through the same exports the
// `single`-mode worklet boots it through, reserves the worklet instance's stack and TLS block and
// releases the host's render half into a token. The worklet adopts the token on the same memory
// and only renders. Every control call runs here: command frames, sources and seeks, the session
// map, the plan latency, the service step, the watermark and the protocol events. Nothing here
// runs on an audio thread, so its allocations are not counted as the worklet's; this file reports
// no allocation count of its own (the host status's `singleModeControlAllocations` is the
// worklet instance's counter, 0 in this mode).
//
// The messages on this Worker's ports are internal (D7, D15-11): the main-realm host is the only
// sender on the Worker's own port, and the worklet is the only peer on the `MessageChannel` port
// the boot message carries. Policy (`scripts/check-web-audioworklet.sh`): no network call or
// socket, no hand-made memory growth, no blocking atomic wait, and one pinned service-tick timer.

const ABI_VERSION = 0x00010000;
const BOOT_OPTIONS_BYTES = 64;
const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_WRONG_STATE = 3;
const RESULT_BACKPRESSURE = 6;
const RESULT_REPREPARE_REQUIRED = 9;
const RESULT_INTERNAL = 255;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const STATE_READY = 2;
const RESOURCE_BYTES = 224;
const STATUS_BYTES = 80;
const WATERMARK_BYTES = 96;
const WATERMARK_COUNT_FLAGS = [1n, 2n, 4n];
const SEEK_REPORT_BYTES = 64;
const SEEK_REPORT_WORDS = [
  ["generation", 8], ["firstSample", 16], ["sourceFrame", 24],
  ["cumulativeUnderrunFrames", 32], ["cumulativeHeldFrames", 40],
];
const PLAN_LATENCY_BYTES = 64;
const PLAN_LATENCY_WORDS = [["samples", 8], ["tailKind", 16], ["tailSamples", 24]];
// The instance reservation record `miso_engine_web_v1_instance_reserve` returns (#1332 S6): four
// little-endian `u32` words, `structSize`, `tlsBase`, `stackTop`, `stackBottom`.
const RESERVATION_BYTES = 16;
const CONTROL_EVENT_LANES = [0, 1];
// #1332 S8: how long a dispose waits for the worklet's `miso.released.v1` before it disposes the
// host with no token and answers the page `release-timeout`. The worklet answers a release from
// its message handler between two quanta, so a live worklet answers in milliseconds; 2 s is
// about 750 quanta at 48 kHz / 128 and stays well inside the page's own 5 s dispose bound, so the
// page hears this Worker's typed reason before it gives up on the Worker itself.
const RELEASE_TIMEOUT_MS = 2000;
// #1332 S8: how often the service tick asks the worklet whether render failed (it has no timer
// of its own and `process()` posts no fatal), so a sticky render failure reaches the page within
// about this long.
const RENDER_CHECK_MS = 50;
const MAXIMUM_CONTROL_EVENTS_PER_LANE = 16;
const BUFFER_DIAGNOSTIC = 4;

// Issue #1387 (S11, D5): the SDK's MSB1 shared PCM rings (`sdk/src/browser/pcm-ring.ts`, the ring
// authority), drained here in `worker` mode by the same rules the worklet prelude
// (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`) drains them by in `single` mode.
// This file loads no code (its capability policy in `scripts/check-web-audioworklet.sh` bans it),
// so the consumer is written here. The rings arrive on a feed port the main-realm host opens
// (`openFeedPort`, `miso.feed-port.v1`), in a port message: a `SharedArrayBuffer` never rides in
// processor options.
const SAB_WRAP_MASK = (1 << 30) - 1;
const SAB_MAGIC = 0x4d534231;
const SAB_VERSION = 1;
const RING = {
  MAGIC: 0, VERSION: 1, CAPACITY: 2, CHANNELS: 3, FRAME_CAPACITY: 4, HEADER_OFFSET: 5,
  PCM_OFFSET: 6, ID_LENGTH: 7, WRITE_INDEX: 8, READ_INDEX: 9, GENERATION_TAG: 10, SEEK_EPOCH: 11,
  WRITER_STATE: 12, ATTACHED: 13, SUBMITTED: 16, STALE: 17, REFUSED: 18, LAST_RESULT: 19,
  UNDERRUNS: 20, DRAIN_BLOCKS: 21, SEEKS_APPLIED: 22, DEPTH: 23, TORN: 24, FINISHED: 25,
  ERRORS: 26, SUBMITTED_GENERATION_TAG: 27,
};
const RING_CONTROL_BYTES = 144;
const RING_I64_OFFSET = 112;
const RING_ID_OFFSET = 144;
const RING_HEADER_OFFSET = 272;
const SLOT_HEADER_BYTES = 32;
const SLOT_FLAG_END_OF_REGION = 1;
const NO_ANCHOR = -1n;
// The render half's status record (`miso.render-clock.v1`): `renderedQuanta` is the `u64` at 40;
// its low word is read, and differences wrap.
const STATUS_RENDERED_QUANTA_WORD = 10;
// The code a seek refusal writes at the start of the session diagnostic when the source already
// has the generation (#1644 D2).
const STALE_GENERATION_CODE = new TextEncoder().encode("source.generation.stale");
// Issue #1387 (root ruling 2): the engine source ring of `worker` mode when the boot asks for the
// default (0): 65 536 frames, about 1.37 s at 48 kHz (512 KiB per stereo stem). The drain is a
// timer on another thread, so the engine ring alone must ride out a render burst; single mode
// keeps the engine's default (its drain runs inside `process()`). An explicit value overrides.
const WORKER_DEFAULT_SOURCE_RING_FRAMES = 65536;
// Issue #1387 (root ruling 1): the shortest period a browser keeps a repeating timer at (the HTML
// timer clamp), so one Worker tick is at least this long.
const TIMER_CLAMP_MS = 4;
// Issue #1387 (root ruling 1): the longest run of quanta a browser renders back to back while the
// Worker cannot refill, in frames. A resume renders its output buffer ahead: 13 quanta measured on
// Chromium 151 (headroom 8 underran 5 of them, 12 and more none), so 2048 frames bounds it.
const RENDER_BURST_FRAMES = 2048;

const BOOT_FIELDS = [
  "tag", "module", "memory", "document", "options", "sampleRateHz", "quantumFrames", "workletPort",
];
const OPTION_FIELDS = [
  "sourceRingFrames", "maximumMemoryBytes", "reserved1", "liveControlMeterBlocks",
  "reserved3", "reserved2",
];
const SOURCE_FIELDS = [
  "tag", "requestId", "sourceId", "generation", "startFrame", "sampleRateHz", "planes", "frames",
  "endOfRegion",
];
const SEEK_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame"];
const SESSION_SEEK_FIELDS = ["tag", "requestId", "generation", "sourceFrame", "anchorSample"];
const SEEK_AT_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame", "anchorSample"];
const SEEK_REPORT_FIELDS = ["tag", "requestId", "sourceId"];
const CONTROL_FIELDS = ["tag", "requestId", "frame"];
const CONTROL_DOCUMENT_FIELDS = [...CONTROL_FIELDS, "document"];
const RESOURCE_NAMES = [
  "optionsBytes", "statusBytes", "sessionDocumentBytes", "diagnosticBytes", "idStagingBytes",
  "sourcePcmStagingBytes", "outputPcmBytes", "bridgeMetadataBytes", "bridgeRetainedBytes",
  "largestBridgeAllocationBytes", "sourceTotalBytes", "sourceOverheadBytes",
  "effectScalarStateBytes", "effectScalarScratchBytes", "builtinRetainedBytes",
  "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes", "graphMetadataBytes",
  "graphDelayBytes", "largestNamedAllocationBytes", "observationRetainedBytes",
];

/// The Worker's clock, for the release deadline and the render check only (no deadline of the
/// engine depends on it; those are counted in render samples).
function clock() {
  return globalThis.performance.now();
}

function exactFields(value, fields) {
  if (value === null || typeof value !== "object") return false;
  const keys = Object.keys(value).sort();
  const expected = [...fields].sort();
  return keys.length === expected.length && keys.every((key, index) => key === expected[index]);
}

function u32(value) {
  return Number.isInteger(value) && value >= 0 && value <= 0xffffffff;
}

function u64(value) {
  return typeof value === "bigint" && value >= 0n && value <= 0xffffffffffffffffn;
}

function bounded64(value, minimum) {
  return typeof value === "bigint" && value >= minimum && value <= 0xffffffffffffffffn;
}

function wholeBytes(bytes) {
  return bytes instanceof Uint8Array && bytes.buffer instanceof ArrayBuffer
    && !(typeof SharedArrayBuffer !== "undefined" && bytes.buffer instanceof SharedArrayBuffer)
    && bytes.byteLength !== 0 && bytes.byteOffset === 0
    && bytes.byteLength === bytes.buffer.byteLength;
}

class ControlWorker {
  constructor(scope) {
    this.scope = scope;
    this.booted = false;
    this.ready = false;
    this.disposed = false;
    this.stickyResult = RESULT_OK;
    this.lastRequestId = 0;
    this.handle = 0;
    this.serviceTick = null;
    this.watermarkRevision = -1n;
    this.watermarkSample = -1n;
    this.watermarkCounts = [0n, 0n, 0n];
    // R-S7-meters (2): the meter view last named to the worklet.
    this.postedHeader = -1;
    this.postedMemoryBytes = -1;
    // Status reads waiting for the worklet's render fields, in request order.
    this.statusReads = [];
    // The page's dispose requests, answered together once the host is freed (a second dispose
    // while one runs joins it; it posts no second release).
    this.disposeRequestIds = [];
    // The result of the finished dispose (undefined until then); a later dispose request gets it.
    this.disposeOutcome = undefined;
    // Set once a dispose began: every later control call is refused `WRONG_STATE`.
    // #1332 S8: the reason a dispose ended without the half (`release-timeout`), the clock time
    // it gives up on the release, the token the boot released (taken back when the worklet never
    // adopted it) and the last render check.
    this.disposeReason = undefined;
    this.releaseDeadline = 0;
    this.bootToken = 0n;
    this.lastRenderCheck = 0;
    this.disposing = false;
    // Issue #1387 (S11): the feed ports (one per SDK feed), the rings bound from them, the render
    // half's status address (the drain's render clock) and the last session seek's generation.
    this.feeds = [];
    this.rings = [];
    this.renderStatusPointer = 0;
    this.renderClockWords = null;
    this.lastSessionSeekGeneration = undefined;
    scope.onmessage = (event) => this.onPageMessage(event.data);
    scope.onmessageerror = () => this.fatal(RESULT_INTERNAL, this.lastRequestId);
  }

  /// One message from the main realm. The first is the boot; every later one is a request, run
  /// here and followed by the service step, the watermark, the protocol events and the meter view.
  onPageMessage(message) {
    if (!this.booted) {
      this.booted = true;
      const result = this.boot(message);
      if (result !== RESULT_OK) this.failBoot(result);
      return;
    }
    if (message?.tag === "miso.feed-port.v1" && exactFields(message, ["tag", "port"])
        && typeof message.port?.postMessage === "function") {
      this.openFeed(message.port);
      return;
    }
    if (this.disposed) {
      // Only a dispose is answered once the host is gone: with the finished dispose's outcome.
      if (message?.tag === "miso.dispose.v1" && Number.isSafeInteger(message.requestId)) {
        this.answerDispose(message.requestId);
      }
      return;
    }
    this.receive(message);
    const requestId = message?.tag === "miso.service.v1" ? 0 : this.lastRequestId;
    this.afterControl(requestId);
  }

  afterControl(requestId) {
    this.drainFeeds();
    this.serviceControl(requestId);
    this.pushWatermark(requestId);
    this.drainControlEvents(requestId);
    this.postMeterView();
  }

  get buffer() {
    return this.memory.buffer;
  }

  view(Constructor, pointer, length) {
    return new Constructor(this.memory.buffer, pointer, length);
  }

  /// D2: instantiate on the shared memory first, boot through the existing boot exports, reserve
  /// the worklet's stack and TLS, release the render half and reply with its token.
  boot(message) {
    if (!exactFields(message, message?.bootCommand === undefined
      ? BOOT_FIELDS : [...BOOT_FIELDS, "bootCommand"])
        || message.tag !== "miso.worker-boot.v1"
        || !(message.module instanceof WebAssembly.Module)
        || !(message.memory?.buffer instanceof SharedArrayBuffer)
        || !(message.document instanceof Uint8Array)
        || (message.bootCommand !== undefined && !(message.bootCommand instanceof Uint8Array))
        || !exactFields(message.options, OPTION_FIELDS)
        || !u32(message.sampleRateHz) || message.sampleRateHz === 0
        || !u32(message.quantumFrames) || message.quantumFrames === 0
        || typeof message.workletPort?.postMessage !== "function") {
      return RESULT_INVALID_ARGUMENT;
    }
    this.workletPort = message.workletPort;
    this.workletPort.onmessage = (event) => this.onWorkletMessage(event.data);
    this.memory = message.memory;
    this.sampleRateHz = message.sampleRateHz;
    this.quantumFrames = message.quantumFrames;
    try {
      this.instance = new WebAssembly.Instance(message.module, { env: { memory: this.memory } });
    } catch (_) {
      return RESULT_INTERNAL;
    }
    this.exports = this.instance.exports;
    if (this.exports.miso_engine_web_v1_abi_version() !== ABI_VERSION) return 2;
    const optionsPointer = this.exports.miso_engine_web_v1_boot_options_ptr();
    if (!u32(optionsPointer) || optionsPointer === 0) return RESULT_INTERNAL;
    if (!this.writeBootOptions(optionsPointer, message.options)) return RESULT_INVALID_ARGUMENT;
    const document = message.document;
    const documentPointer = this.exports.miso_engine_web_v1_document_ptr(document.byteLength);
    if (!u32(documentPointer) || documentPointer === 0) {
      return this.exports.miso_engine_web_v1_boot_result();
    }
    this.view(Uint8Array, documentPointer, document.byteLength).set(document);
    // #2166: the boot command (the boot's spectrum taps), staged beside the document.
    if (message.bootCommand !== undefined) {
      const command = message.bootCommand;
      const commandPointer = this.exports.miso_engine_web_v1_boot_command_ptr(command.byteLength);
      if (!u32(commandPointer) || commandPointer === 0) return RESULT_INVALID_ARGUMENT;
      this.view(Uint8Array, commandPointer, command.byteLength).set(command);
    }
    this.handle = this.exports.miso_engine_web_v1_boot(document.byteLength);
    if (this.handle === 0) return this.exports.miso_engine_web_v1_boot_result();
    const ready = this.readReadyFields(message.options);
    if (typeof ready === "number") return this.disposeAfterFailedBoot(ready);
    // #1331 recipe rule 2: the worklet instance's own stack and TLS block, reserved here.
    const tlsSize = this.exports.__tls_size?.value;
    const tlsAlign = this.exports.__tls_align?.value;
    if (!u32(tlsSize) || !u32(tlsAlign)) return this.disposeAfterFailedBoot(RESULT_INTERNAL);
    const reservation = this.exports.miso_engine_web_v1_instance_reserve(tlsSize, tlsAlign);
    if (!u32(reservation) || reservation === 0
        || reservation + RESERVATION_BYTES > this.buffer.byteLength
        || this.view(DataView, reservation, RESERVATION_BYTES).getUint32(0, true) !== RESERVATION_BYTES) {
      return this.disposeAfterFailedBoot(RESULT_INTERNAL);
    }
    // D2.4: the render half leaves this instance; the token is the only way back to it.
    const token = this.exports.miso_engine_web_v1_host_release(this.handle);
    const unsigned = typeof token === "bigint" ? BigInt.asUintN(64, token) : 0n;
    if (unsigned === 0n || Number(unsigned >> 32n) !== this.handle) {
      return this.disposeAfterFailedBoot(RESULT_INTERNAL);
    }
    this.ready = true;
    this.bootToken = unsigned;
    this.lastRenderCheck = clock();
    this.postedMemoryBytes = this.buffer.byteLength;
    this.scope.postMessage({
      tag: "miso.worker-ready.v1",
      requestId: 0,
      result: RESULT_OK,
      handle: this.handle,
      token: unsigned,
      reservation,
      metersAttached: ready.metersAttached,
      backend: "simd128",
      resources: ready.resources,
      memoryBytes: this.buffer.byteLength,
      sourceRingFrames: ready.sourceRingFrames,
    });
    this.startServiceTick();
    return RESULT_OK;
  }

  writeBootOptions(pointer, options) {
    const words = [options.reserved1, options.liveControlMeterBlocks, options.reserved3, options.reserved2];
    if (!u32(options.sourceRingFrames) || !bounded64(options.maximumMemoryBytes, 0n)
        || options.maximumMemoryBytes > 0xffffffffn
        || words.some((value) => !bounded64(value, 0n) || value > 0xffffffffn)
        || options.reserved1 !== 0n || options.reserved3 !== 0n || options.reserved2 !== 0n) {
      return false;
    }
    const view = this.view(DataView, pointer, BOOT_OPTIONS_BYTES);
    view.setUint32(0, BOOT_OPTIONS_BYTES, true);
    view.setUint32(4, ABI_VERSION, true);
    view.setUint32(8, this.sampleRateHz, true);
    view.setUint32(12, this.quantumFrames, true);
    // Root ruling 2: a default request gets the worker-mode default, a whole number of quanta.
    const ringFrames = options.sourceRingFrames !== 0 ? options.sourceRingFrames
      : Math.ceil(WORKER_DEFAULT_SOURCE_RING_FRAMES / this.quantumFrames) * this.quantumFrames;
    view.setUint32(16, ringFrames, true);
    view.setUint32(20, 0, true);
    view.setBigUint64(24, options.maximumMemoryBytes, true);
    words.forEach((value, index) => view.setBigUint64(32 + index * 8, value, true));
    this.metersAttached = options.liveControlMeterBlocks !== 0n;
    return true;
  }

  /// The fields of the boot's `miso.ready.v1`, read and checked as the `single`-mode worklet reads
  /// them, or the result code of the refusal.
  readReadyFields() {
    const e = this.exports;
    const statusPointer = e.miso_engine_web_v1_status_ptr(this.handle);
    const resourcePointer = e.miso_engine_web_v1_resource_ptr(this.handle);
    this.sourceIdPointer = e.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_ID);
    this.sourceIdCapacity = e.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SOURCE_ID);
    this.sourcePcmPointer = e.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_PCM);
    const sourcePcmCapacity = e.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SOURCE_PCM);
    this.statusPointer = statusPointer;
    if (!u32(statusPointer) || statusPointer === 0 || !u32(resourcePointer) || resourcePointer === 0
        || !u32(this.sourceIdPointer) || this.sourceIdPointer === 0 || !u32(this.sourceIdCapacity)
        || !u32(this.sourcePcmPointer) || this.sourcePcmPointer === 0
        || !u32(sourcePcmCapacity) || sourcePcmCapacity % (this.quantumFrames * 4) !== 0) {
      return RESULT_INTERNAL;
    }
    this.maximumSourceChannels = sourcePcmCapacity / (this.quantumFrames * 4);
    let status;
    let resources;
    try {
      status = this.readStatus();
      resources = this.readResources(resourcePointer);
    } catch (_) {
      return RESULT_INTERNAL;
    }
    if (status.sampleRateHz !== this.sampleRateHz || status.quantumFrames !== this.quantumFrames) {
      return RESULT_REPREPARE_REQUIRED;
    }
    if (resources.backend !== 1 || status.backend !== 1
        || resources.sampleRateHz !== this.sampleRateHz
        || resources.quantumFrames !== this.quantumFrames
        || status.state !== STATE_READY || status.lastResult !== RESULT_OK
        || status.nextAbsoluteSample !== 0n || status.renderedQuanta !== 0n) {
      return RESULT_INVALID_ARGUMENT;
    }
    if (this.readSourceRows() === null) return RESULT_INTERNAL;
    const sourceRingFrames = e.miso_engine_web_v1_resolved_source_ring_frames(this.handle);
    if (!u32(sourceRingFrames) || sourceRingFrames === 0
        || sourceRingFrames % this.quantumFrames !== 0) return RESULT_INTERNAL;
    this.feedHeadroom = this.feedHeadroomQuanta(sourceRingFrames / this.quantumFrames);
    if (e.miso_engine_web_v1_watermark_read(this.handle) === RESULT_OK) {
      const view = this.view(DataView, e.miso_engine_web_v1_watermark_ptr(this.handle), WATERMARK_BYTES);
      this.watermarkRevision = view.getBigUint64(8, true);
      this.watermarkSample = view.getBigUint64(16, true);
      this.watermarkCounts = [32, 40, 48].map((offset) => view.getBigUint64(offset, true));
    }
    return { resources, sourceRingFrames, metersAttached: this.metersAttached };
  }

  readResources(pointer) {
    const view = this.view(DataView, pointer, RESOURCE_BYTES);
    if (view.getUint32(0, true) !== RESOURCE_BYTES || view.getUint32(4, true) !== ABI_VERSION
        || view.getUint32(20, true) !== 0 || view.getUint32(24, true) !== 0
        || view.getUint32(28, true) !== 0
        || [200, 208, 216].some((offset) => view.getBigUint64(offset, true) !== 0n)) {
      throw new RangeError("Invalid resource report");
    }
    const resources = {
      sampleRateHz: view.getUint32(8, true),
      quantumFrames: view.getUint32(12, true),
      backend: view.getUint32(16, true),
    };
    RESOURCE_NAMES.forEach((name, index) => {
      resources[name] = view.getBigUint64(32 + index * 8, true);
    });
    return resources;
  }

  /// The control half's status: its lifecycle state, last result and blocking rebuilds are
  /// current; its render clock is the one at the release (the worklet's is current).
  readStatus() {
    const view = this.view(DataView, this.statusPointer, STATUS_BYTES);
    if (view.getUint32(0, true) !== STATUS_BYTES || view.getUint32(4, true) !== ABI_VERSION) {
      throw new RangeError("Invalid status report");
    }
    return {
      state: view.getUint32(8, true),
      lastResult: this.stickyResult || view.getUint32(12, true),
      backend: view.getUint32(16, true),
      sampleRateHz: view.getUint32(20, true),
      quantumFrames: view.getUint32(24, true),
      blockingRebuilds: view.getUint32(28, true),
      nextAbsoluteSample: view.getBigUint64(32, true),
      renderedQuanta: view.getBigUint64(40, true),
      liveValuesSuperseded: view.getBigUint64(48, true),
      appliedRevision: view.getBigUint64(56, true),
      appliedSample: view.getBigUint64(64, true),
      appliedOutcome: view.getBigUint64(72, true),
    };
  }

  readSourceRows() {
    const e = this.exports;
    const count = e.miso_engine_web_v1_source_count(this.handle);
    if (!u32(count)) return null;
    const rows = [];
    for (let index = 0; index < count; index += 1) {
      const length = e.miso_engine_web_v1_source_id(this.handle, index);
      if (!u32(length) || length === 0 || length > this.sourceIdCapacity) return null;
      const bytes = this.view(Uint8Array, this.sourceIdPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return null;
        id += String.fromCharCode(bytes[byte]);
      }
      const channels = e.miso_engine_web_v1_source_channels(this.handle, index);
      const frames = e.miso_engine_web_v1_source_frames(this.handle, index);
      if (!u32(channels) || channels === 0 || channels > this.maximumSourceChannels
          || !u64(frames) || frames === 0n) return null;
      rows.push({ id, channels, frames });
    }
    return rows;
  }

  disposeAfterFailedBoot(result) {
    try { this.exports.miso_engine_web_v1_dispose(this.handle); } catch (_) { /* best effort */ }
    this.handle = 0;
    return result;
  }

  /// D6: a refused boot answers with the result code the `single`-mode worklet answers, and the
  /// worklet is never started.
  failBoot(result) {
    this.disposed = true;
    this.scope.postMessage({ tag: "miso.error.v1", requestId: 0, result });
  }

  // The service tick of `worker` mode (#1332 S7): the control work runs here, so the main realm's
  // tick is not sent, and this Worker's one timer runs the service step, the watermark and the
  // protocol events once per quantum. No deadline depends on the period.
  // SERVICE_TICK_BEGIN
  startServiceTick() {
    const periodMs = Math.max(1, Math.round((1000 * this.quantumFrames) / this.sampleRateHz));
    this.serviceTick = setInterval(() => this.tick(), periodMs);
  }
  // SERVICE_TICK_END

  tick() {
    if (this.disposed) {
      this.stopServiceTick();
      return;
    }
    // #1332 S8: while a dispose waits for the worklet's release, the tick only keeps its deadline.
    if (this.disposing) {
      if (clock() >= this.releaseDeadline) this.finishDispose(0n, "release-timeout");
      return;
    }
    if (!this.ready) {
      this.stopServiceTick();
      return;
    }
    this.afterControl(0);
    this.checkRender();
  }

  /// #1332 S8: ask the worklet, at most every `RENDER_CHECK_MS`, whether render failed; a failed
  /// worklet posts its typed fatal to the page. Nothing is posted back when render is well.
  checkRender() {
    const now = clock();
    if (now - this.lastRenderCheck < RENDER_CHECK_MS) return;
    this.lastRenderCheck = now;
    this.workletPort.postMessage({ tag: "miso.render-check.v1" });
  }

  stopServiceTick() {
    if (this.serviceTick === null) return;
    clearInterval(this.serviceTick);
    this.serviceTick = null;
  }

  post(message, transfer = []) {
    this.scope.postMessage(message, transfer);
  }

  acknowledge(requestId, result, planes) {
    const message = { tag: "miso.ack.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.post(message, this.transferList(planes));
  }

  transferList(planes) {
    if (!Array.isArray(planes)) return [];
    return [...new Set(planes
      .filter((plane) => plane instanceof Float32Array && plane.buffer instanceof ArrayBuffer)
      .map((plane) => plane.buffer))];
  }

  sticky(result, requestId, planes) {
    this.ready = false;
    this.stickyResult = result;
    const message = { tag: "miso.error.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.post(message, this.transferList(planes));
  }

  fatal(result, requestId) {
    this.stopServiceTick();
    this.sticky(result, requestId);
  }

  receive(message) {
    if (message?.tag === "miso.service.v1" && exactFields(message, ["tag"])) return;
    const returnedPlanes = Array.isArray(message?.planes) ? message.planes : undefined;
    if (!Number.isSafeInteger(message?.requestId) || message.requestId <= this.lastRequestId) {
      this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returnedPlanes);
      return;
    }
    this.lastRequestId = message.requestId;
    if (message.tag === "miso.dispose.v1" && exactFields(message, ["tag", "requestId"])) {
      this.beginDispose(message.requestId);
      return;
    }
    if (this.stickyResult !== RESULT_OK) {
      this.sticky(this.stickyResult, message.requestId, returnedPlanes);
      return;
    }
    if (this.disposing) {
      this.refuseDisposing(message);
      return;
    }
    if (message.tag === "miso.source.v1" && exactFields(message, SOURCE_FIELDS)) {
      this.receiveSource(message);
    } else if (message.tag === "miso.seek.v1" && exactFields(message, SEEK_FIELDS)) {
      this.receiveSeek(message);
    } else if (message.tag === "miso.session-seek.v1" && exactFields(message, SESSION_SEEK_FIELDS)) {
      this.receiveSessionSeek(message);
    } else if (message.tag === "miso.seek-at.v1" && exactFields(message, SEEK_AT_FIELDS)) {
      this.receiveSeekAt(message);
    } else if (message.tag === "miso.seek-report.v1" && exactFields(message, SEEK_REPORT_FIELDS)) {
      this.receiveSeekReport(message);
    } else if (message.tag === "miso.plan-latency.v1" && exactFields(message, ["tag", "requestId"])) {
      this.receivePlanLatency(message);
    } else if (message.tag === "miso.control.v1"
        && (exactFields(message, CONTROL_FIELDS) || exactFields(message, CONTROL_DOCUMENT_FIELDS))) {
      this.receiveControl(message);
    } else if (message.tag === "miso.sessionmap.v1" && exactFields(message, ["tag", "requestId"])) {
      const sources = this.readSourceRows();
      if (sources === null) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      this.post({
        tag: "miso.sessionmap.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        sources,
        metersAttached: this.metersAttached === true,
      });
    } else if (message.tag === "miso.status.v1" && exactFields(message, ["tag", "requestId"])) {
      // The render fields are the worklet's: ask it, and answer when it replies.
      this.statusReads.push(message.requestId);
      this.workletPort.postMessage({ tag: "miso.render-status.v1", requestId: message.requestId });
    } else {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId, returnedPlanes);
    }
  }

  /// A request that arrives after a dispose began: a control call is refused `WRONG_STATE` in its
  /// own reply shape (never the sticky error, which would fail the dispose too); a status read is
  /// answered from this Worker's status, since the worklet has released its half.
  refuseDisposing(message) {
    const { tag, requestId } = message;
    if (tag === "miso.source.v1" || tag === "miso.seek.v1" || tag === "miso.seek-at.v1") {
      this.acknowledge(requestId, RESULT_WRONG_STATE, tag === "miso.source.v1" ? message.planes : undefined);
    } else if (tag === "miso.session-seek.v1") {
      this.post({ tag, requestId, result: RESULT_WRONG_STATE, generation: message.generation });
    } else if (tag === "miso.seek-report.v1") {
      this.post({ tag, requestId, result: RESULT_WRONG_STATE, report: null });
    } else if (tag === "miso.plan-latency.v1") {
      this.post({ tag, requestId, result: RESULT_WRONG_STATE, latency: null });
    } else if (tag === "miso.control.v1") {
      const response = new Uint8Array(0);
      this.post({
        tag, requestId, result: RESULT_WRONG_STATE, response,
        revision: this.exports.miso_engine_web_v1_control_revision(this.handle),
      }, [response.buffer]);
    } else if (tag === "miso.status.v1") {
      this.answerStatus(requestId, null);
    } else if (tag === "miso.sessionmap.v1") {
      // The handle is valid until `finishDispose`: the map is read as before.
      const sources = this.readSourceRows();
      this.post(sources === null
        ? { tag, requestId, result: RESULT_WRONG_STATE }
        : { tag, requestId, result: RESULT_OK, sources, metersAttached: this.metersAttached === true });
    } else {
      // No refusal during a dispose is sticky: a sticky error would reject the dispose itself.
      this.acknowledge(requestId, RESULT_WRONG_STATE);
    }
  }

  stageSourceId(sourceId) {
    const bytes = new TextEncoder().encode(sourceId);
    if (bytes.byteLength > this.sourceIdCapacity) return -1;
    this.view(Uint8Array, this.sourceIdPointer, bytes.byteLength).set(bytes);
    return bytes.byteLength;
  }

  receiveSource(message) {
    if (typeof message.sourceId !== "string" || !bounded64(message.generation, 1n)
        || !bounded64(message.startFrame, 0n) || message.sampleRateHz !== this.sampleRateHz
        || !u32(message.frames) || message.frames > this.quantumFrames
        || typeof message.endOfRegion !== "boolean" || !Array.isArray(message.planes)
        || message.planes.length === 0 || message.planes.length > this.maximumSourceChannels
        || message.planes.some((plane) => !(plane instanceof Float32Array)
          || !(plane.buffer instanceof ArrayBuffer) || plane.length !== message.frames)) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId, message.planes);
      return;
    }
    const idLength = this.stageSourceId(message.sourceId);
    if (idLength < 0) {
      this.sticky(4, message.requestId, message.planes);
      return;
    }
    const pcm = this.view(Float32Array, this.sourcePcmPointer, this.maximumSourceChannels * this.quantumFrames);
    message.planes.forEach((plane, channel) => pcm.set(plane, channel * this.quantumFrames));
    const result = this.exports.miso_engine_web_v1_source_submit(
      this.handle, idLength, message.generation, message.startFrame, message.planes.length,
      message.frames, message.endOfRegion ? 1 : 0,
    );
    this.acknowledge(message.requestId, result, message.planes);
  }

  receiveSeek(message) {
    if (typeof message.sourceId !== "string" || !bounded64(message.generation, 1n)
        || !bounded64(message.sourceFrame, 0n)) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const idLength = this.stageSourceId(message.sourceId);
    if (idLength < 0) {
      this.sticky(4, message.requestId);
      return;
    }
    this.acknowledge(message.requestId, this.exports.miso_engine_web_v1_source_seek(
      this.handle, idLength, message.generation, message.sourceFrame,
    ));
  }

  receiveSessionSeek(message) {
    if (!bounded64(message.generation, 1n) || !bounded64(message.sourceFrame, 0n)
        || typeof message.anchorSample !== "bigint" || message.anchorSample < -1n
        || message.anchorSample > 0xffffffffffffffffn) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const result = message.anchorSample === -1n
      ? this.exports.miso_engine_web_v1_session_seek(this.handle, message.generation, message.sourceFrame)
      : this.exports.miso_engine_web_v1_session_seek_at(
        this.handle, message.generation, message.sourceFrame, message.anchorSample,
      );
    // #1644 D2: the feed drain's stale-generation rule reads it.
    if (result === RESULT_OK) this.lastSessionSeekGeneration = message.generation;
    this.post({
      tag: "miso.session-seek.v1", requestId: message.requestId, result, generation: message.generation,
    });
  }

  receiveSeekAt(message) {
    if (typeof message.sourceId !== "string" || !bounded64(message.generation, 1n)
        || !bounded64(message.sourceFrame, 0n) || !bounded64(message.anchorSample, 0n)) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const idLength = this.stageSourceId(message.sourceId);
    if (idLength < 0) {
      this.sticky(4, message.requestId);
      return;
    }
    const result = this.exports.miso_engine_web_v1_source_seek_at(
      this.handle, idLength, message.generation, message.sourceFrame, message.anchorSample,
    );
    // #1297b: an applied anchored seek lets the drain consume that ring's anchored epoch.
    if (result === RESULT_OK) this.recordAnchoredSeek(message);
    this.acknowledge(message.requestId, result);
  }

  receiveSeekReport(message) {
    if (typeof message.sourceId !== "string") {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const idLength = this.stageSourceId(message.sourceId);
    if (idLength < 0) {
      this.sticky(4, message.requestId);
      return;
    }
    const result = this.exports.miso_engine_web_v1_source_seek_report(this.handle, idLength);
    let report = null;
    if (result === RESULT_OK) {
      const pointer = this.exports.miso_engine_web_v1_source_seek_report_ptr(this.handle);
      const view = this.view(DataView, pointer, SEEK_REPORT_BYTES);
      if (view.getUint32(0, true) !== SEEK_REPORT_BYTES) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      report = {};
      for (const [field, offset] of SEEK_REPORT_WORDS) report[field] = view.getBigUint64(offset, true);
    }
    this.post({ tag: "miso.seek-report.v1", requestId: message.requestId, result, report });
  }

  receivePlanLatency(message) {
    const result = this.exports.miso_engine_web_v1_plan_latency_read(this.handle);
    let latency = null;
    if (result === RESULT_OK) {
      const pointer = this.exports.miso_engine_web_v1_plan_latency_ptr(this.handle);
      const view = this.view(DataView, pointer, PLAN_LATENCY_BYTES);
      if (view.getUint32(0, true) !== PLAN_LATENCY_BYTES) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      latency = {};
      for (const [field, offset] of PLAN_LATENCY_WORDS) latency[field] = view.getBigUint64(offset, true);
    }
    this.post({ tag: "miso.plan-latency.v1", requestId: message.requestId, result, latency });
  }

  /// One command frame through the control plane, as the `single`-mode worklet runs it; the
  /// document a `SESSION_REPLACE` frame names is staged beside it.
  receiveControl(message) {
    const { frame, document } = message;
    if (!wholeBytes(frame) || ("document" in message && !wholeBytes(document))) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const e = this.exports;
    let pointer = e.miso_engine_web_v1_control_request_ptr(this.handle, frame.byteLength);
    if (pointer !== 0 && document !== undefined) {
      const documentPointer = e.miso_engine_web_v1_replace_document_ptr(this.handle, document.byteLength);
      if (documentPointer === 0) pointer = 0;
      else this.view(Uint8Array, documentPointer, document.byteLength).set(document);
    }
    let result = RESULT_INVALID_ARGUMENT;
    let response = new Uint8Array(0);
    if (pointer !== 0) {
      this.view(Uint8Array, pointer, frame.byteLength).set(frame);
      result = e.miso_engine_web_v1_control_command(this.handle, frame.byteLength);
      const responsePointer = e.miso_engine_web_v1_control_response_ptr(this.handle);
      const responseBytes = e.miso_engine_web_v1_control_response_bytes(this.handle);
      if (responsePointer !== 0) {
        response = new Uint8Array(responseBytes);
        response.set(this.view(Uint8Array, responsePointer, responseBytes));
      }
    }
    const revision = e.miso_engine_web_v1_control_revision(this.handle);
    this.post({
      tag: "miso.control.v1", requestId: message.requestId, result, response, revision,
    }, [response.buffer]);
  }

  serviceControl(requestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK) return;
    const result = this.exports.miso_engine_web_v1_service(this.handle);
    if (result !== RESULT_OK) this.fatal(result, requestId);
  }

  pushWatermark(requestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK) return;
    const e = this.exports;
    const result = e.miso_engine_web_v1_watermark_read(this.handle);
    if (result === RESULT_BACKPRESSURE) return;
    if (result !== RESULT_OK) {
      this.fatal(result, requestId);
      return;
    }
    const view = this.view(DataView, e.miso_engine_web_v1_watermark_ptr(this.handle), WATERMARK_BYTES);
    if (view.getUint32(0, true) !== WATERMARK_BYTES || view.getUint32(4, true) !== 0) {
      this.fatal(RESULT_INTERNAL, requestId);
      return;
    }
    const revision = view.getBigUint64(8, true);
    const sample = view.getBigUint64(16, true);
    if (revision === this.watermarkRevision && sample === this.watermarkSample) return;
    const counts = [32, 40, 48].map((offset) => view.getBigUint64(offset, true));
    let outcome = view.getBigUint64(24, true);
    counts.forEach((count, index) => {
      if (count > this.watermarkCounts[index]) outcome |= WATERMARK_COUNT_FLAGS[index];
    });
    this.watermarkRevision = revision;
    this.watermarkSample = sample;
    this.watermarkCounts = counts;
    this.post({
      tag: "miso.watermark.v1", revision, sample, outcome, exactCount: counts[0],
      transitionFallbackCount: counts[1], supersededCount: counts[2],
    });
  }

  drainControlEvents(requestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK) return;
    const e = this.exports;
    for (const lane of CONTROL_EVENT_LANES) {
      for (let calls = 0; calls < MAXIMUM_CONTROL_EVENTS_PER_LANE; calls += 1) {
        const result = e.miso_engine_web_v1_control_event(this.handle, lane);
        if (result !== RESULT_OK) {
          this.fatal(result, requestId);
          return;
        }
        const bytes = e.miso_engine_web_v1_control_response_bytes(this.handle);
        if (bytes === 0) break;
        const pointer = e.miso_engine_web_v1_control_response_ptr(this.handle);
        if (pointer === 0) {
          this.fatal(RESULT_INTERNAL, requestId);
          return;
        }
        const frame = new Uint8Array(bytes);
        frame.set(this.view(Uint8Array, pointer, bytes));
        this.post({ tag: "miso.control-event.v1", lane, frame }, [frame.buffer]);
      }
    }
  }

  // Issue #1387 (S11, D5): the MSB1 feed drain of `worker` mode. Every rule is the worklet
  // prelude's (`miso-engine-v1-pcm-feed-worklet.js`): apply a published plain seek through
  // `source_seek` (an anchored one only after this Worker's `source_seek_at` answered OK for the
  // same tuple), drop chunks whose generation tag is stale, stop at `RESULT_BACKPRESSURE` with the
  // chunk left in the ring, copy the planes into the same staging buffer and call the same
  // `source_submit` export the `miso.source.v1` path calls. Two things differ, both because this
  // runs on the control Worker and not on the audio thread: a pass submits until the engine's ring
  // pushes back (bounded by the MSB1 ring's capacity), not two chunks a block, so timer jitter
  // never starves render; and the block accounting (depth, underruns, drain blocks) counts the
  // quanta render reports since the last pass, read from the render half's status, so a
  // suspended context counts nothing. The drain runs on the service tick (this Worker's one
  // timer) and after every message.

  /// One feed port (one SDK feed): the attach protocol of the prelude's attach node.
  openFeed(port) {
    const feed = { port, delivered: [] };
    this.feeds.push(feed);
    port.onmessage = (event) => {
      this.receiveFeed(feed, event.data);
      this.drainFeeds();
    };
  }

  receiveFeed(feed, data) {
    if (data === null || typeof data !== "object") return;
    if (data.op === "attach" && Array.isArray(data.rings)) {
      this.withdrawRings(feed.delivered);
      feed.delivered = data.rings;
      if (this.disposed || this.disposing) return;
      for (const shared of data.rings) {
        const ring = this.bindRing(shared);
        if (ring === null) continue;
        Atomics.store(ring.control, RING.ATTACHED, 1);
        this.rings.push(ring);
      }
    } else if (data.op === "detach") {
      // The feed closed: its rings are withdrawn and its port is closed from this end.
      this.withdrawRings(feed.delivered);
      feed.delivered = [];
      this.feeds = this.feeds.filter((entry) => entry !== feed);
      feed.port.onmessage = null;
      try { feed.port.close?.(); } catch (_) { /* already closed */ }
    } else if (data.op === "detach-source") {
      // A port clones every message, so the ring is named by its index in the attach list.
      const index = data.index;
      const shared = Number.isSafeInteger(index) && index >= 0 && index < feed.delivered.length
        ? feed.delivered[index] : null;
      if (shared != null) {
        this.withdrawRings([shared]);
        feed.delivered = feed.delivered.map((entry, at) => (at === index ? null : entry));
      }
      feed.port.postMessage({ op: "source-detached", requestId: data.requestId, detached: shared != null });
    } else if (data.op === "prepare-seek") {
      const delivered = feed.delivered.filter((entry) => entry !== null);
      feed.port.postMessage({ op: "seek-prepared", requestId: data.requestId, ...this.prepareSharedSeeks(delivered, data.seeks) });
    }
  }

  withdrawRings(delivered) {
    this.rings = this.rings.filter((ring) => {
      if (!delivered.includes(ring.shared)) return true;
      Atomics.store(ring.control, RING.ATTACHED, 0);
      return false;
    });
  }

  /// At dispose: the rings are withdrawn and the ports closed.
  closeFeeds() {
    for (const feed of this.feeds) {
      this.withdrawRings(feed.delivered);
      feed.port.onmessage = null;
      try { feed.port.close?.(); } catch (_) { /* already closed */ }
    }
    this.feeds = [];
  }

  /// One MSB1 ring, checked as the prelude checks it, or null.
  bindRing(shared) {
    if (!(shared instanceof SharedArrayBuffer) || shared.byteLength < RING_HEADER_OFFSET) return null;
    const control = new Int32Array(shared, 0, RING_CONTROL_BYTES / 4);
    if (Atomics.load(control, RING.MAGIC) !== SAB_MAGIC || control[RING.VERSION] !== SAB_VERSION) return null;
    const capacity = control[RING.CAPACITY];
    const channels = control[RING.CHANNELS];
    const frameCapacity = control[RING.FRAME_CAPACITY];
    const headerOffset = control[RING.HEADER_OFFSET];
    const pcmOffset = control[RING.PCM_OFFSET];
    const idLength = control[RING.ID_LENGTH];
    if (capacity <= 0 || (capacity & (capacity - 1)) !== 0 || channels <= 0
        || channels > this.maximumSourceChannels || frameCapacity !== this.quantumFrames
        || idLength <= 0 || idLength > this.sourceIdCapacity || headerOffset !== RING_HEADER_OFFSET
        || headerOffset + capacity * SLOT_HEADER_BYTES !== pcmOffset
        || pcmOffset + capacity * channels * frameCapacity * 4 > shared.byteLength) {
      return null;
    }
    const planes = [];
    for (let slot = 0; slot < capacity; slot += 1) {
      const slotPlanes = [];
      for (let channel = 0; channel < channels; channel += 1) {
        slotPlanes.push(new Float32Array(shared, pcmOffset + (slot * channels + channel) * frameCapacity * 4, frameCapacity));
      }
      planes.push(slotPlanes);
    }
    return {
      shared,
      control,
      controlI64: new BigInt64Array(shared, RING_I64_OFFSET, 3),
      headers: new Int32Array(shared, headerOffset, (capacity * SLOT_HEADER_BYTES) / 4),
      headersI64: new BigInt64Array(shared, headerOffset, (capacity * SLOT_HEADER_BYTES) / 8),
      idBytes: new Uint8Array(shared, RING_ID_OFFSET, idLength),
      idLength,
      planes,
      capacity,
      channels,
      seenEpoch: Atomics.load(control, RING.SEEK_EPOCH),
      anchoredGeneration: 0n,
      anchoredFrame: 0n,
      anchoredSample: NO_ANCHOR,
      // The block accounting: the engine's cumulative underrun frames at the last read (null
      // until read), the frames counted and excluded since (a finished region, an idle writer),
      // the quanta counted, and the rendered quanta not yet accounted.
      reportUnderrun: null,
      countedFrames: 0n,
      excludedFrames: 0n,
      countedQuanta: 0,
      pendingElapsed: 0,
      // A consumed seek render has not shown in effect yet: its generation (u64), or null, and
      // whether its new generation may fill the headroom (a plain seek's).
      seekGeneration: null,
      seekAdmit: false,
      finished: false,
      // The render clock this ring's accounting has reached; null until the clock is known.
      renderedSeen: this.renderedQuanta(),
    };
  }

  /// Issue #1387 (root ruling 1, attempt 2): the headroom `H`, in chunks, the drain keeps free in
  /// each source's engine ring, read from the producer's real occupancy (`source_fill`). A seek's
  /// new generation is admitted only into free slots (the stale chunks stay queued until render
  /// pops the seek and drops them by tag), so a ring seek's pass submits up to `H` chunks at the
  /// new position at once, and they must last until the next pass refills: `H` covers the quanta
  /// render consumes in one Worker tick (a tick is at least `TIMER_CLAMP_MS`) plus a render burst
  /// (`RENDER_BURST_FRAMES`) plus the block whose render pops the seek. At 48 kHz and 128 frames:
  /// 2 + 16 + 1 = 19. #1387 MINOR 3: an explicit ring shorter than twice that (the
  /// qualification corpus boots rings down to one quantum) keeps half of itself free instead, so
  /// such a ring trades seek headroom for depth; the default ring (512 quanta) is far above it.
  feedHeadroomQuanta(ringQuanta) {
    const tickMs = Math.max(TIMER_CLAMP_MS, Math.round((1000 * this.quantumFrames) / this.sampleRateHz));
    const perTick = Math.ceil((tickMs * this.sampleRateHz) / (1000 * this.quantumFrames));
    return Math.min(perTick + Math.ceil(RENDER_BURST_FRAMES / this.quantumFrames) + 1, Math.floor(ringQuanta / 2));
  }

  /// The ring's source as render last published it (its seek report): the generation in effect
  /// and the cumulative underrun frames, or null when the read is refused or lands
  /// inside render's publication.
  readSourceReport(ring) {
    this.stageRingId(ring);
    if (this.exports.miso_engine_web_v1_source_seek_report(this.handle, ring.idLength) !== RESULT_OK) return null;
    const view = this.view(DataView, this.exports.miso_engine_web_v1_source_seek_report_ptr(this.handle), SEEK_REPORT_BYTES);
    if (view.getUint32(0, true) !== SEEK_REPORT_BYTES) return null;
    return { generation: view.getBigUint64(8, true), underrun: view.getBigUint64(32, true) };
  }

  /// The source's ring occupancy, `{ queued, room }` in chunks, or null when the engine refuses
  /// the read (the drain then submits as the prelude does, and the export counts the refusal).
  readFill(ring) {
    this.stageRingId(ring);
    const fill = BigInt.asUintN(64, BigInt(this.exports.miso_engine_web_v1_source_fill(this.handle, ring.idLength)));
    if (fill === 0xffffffffffffffffn) return null;
    return { queued: Number(fill >> 32n), room: Number(fill & 0xffffffffn) };
  }

  /// The low word of the render half's `renderedQuanta`, or null before the worklet named it.
  renderedQuanta() {
    if (this.renderStatusPointer === 0) return null;
    if (this.renderClockWords?.buffer !== this.buffer) this.renderClockWords = new Uint32Array(this.buffer);
    return Atomics.load(this.renderClockWords, this.renderStatusPointer / 4 + STATUS_RENDERED_QUANTA_WORD);
  }

  drainFeeds() {
    if (this.rings.length === 0 || this.disposed || this.disposing || !this.ready
        || this.stickyResult !== RESULT_OK) {
      return;
    }
    const rendered = this.renderedQuanta();
    for (const ring of this.rings) {
      try {
        this.drainSharedRing(ring, rendered);
      } catch (_) {
        ring.control[RING.ERRORS] += 1;
      }
    }
  }

  /// Stage a ring's source ID in the engine's ID staging buffer.
  stageRingId(ring) {
    this.view(Uint8Array, this.sourceIdPointer, ring.idLength).set(ring.idBytes);
  }

  drainSharedRing(ring, rendered) {
    const control = ring.control;
    // The quanta render took since the last pass: what the prelude counts once per block.
    const elapsed = rendered === null || ring.renderedSeen === null ? 0 : (rendered - ring.renderedSeen) >>> 0;
    if (rendered !== null) ring.renderedSeen = rendered;
    // As in the prelude, a ring waiting for its seek (an anchored start before `seek-at`)
    // counts no drain block and no underrun: the quanta pass uncounted.
    if (this.applySharedSeek(ring) !== RESULT_OK) {
      ring.pendingElapsed = 0;
      return;
    }
    const report = this.readSourceReport(ring);
    const before = this.readFill(ring);
    this.accountBlocks(ring, elapsed, report, before);
    // Root ruling 1: the headroom stays free, except for the chunks of a plain seek's new
    // generation, admitted until render shows that seek in effect.
    let allowed = before === null ? ring.capacity
      : ring.seekAdmit ? before.room : before.room - this.feedHeadroom;
    let queued = before === null ? 0 : before.queued;
    const generationTag = Atomics.load(control, RING.GENERATION_TAG);
    const write = Atomics.load(control, RING.WRITE_INDEX);
    const capacityMask = ring.capacity - 1;
    const staging = this.view(Float32Array, this.sourcePcmPointer, this.maximumSourceChannels * this.quantumFrames);
    let read = control[RING.READ_INDEX];
    for (let remaining = ring.capacity; read !== write && remaining > 0; remaining -= 1) {
      const slot = read & capacityMask;
      const word = slot * (SLOT_HEADER_BYTES / 4);
      if (ring.headers[word] !== read) {
        // Only a second writer makes the slot name another chunk than the index.
        control[RING.TORN] += 1;
        break;
      }
      if (ring.headers[word + 1] !== generationTag) {
        read = (read + 1) & SAB_WRAP_MASK;
        control[RING.STALE] += 1;
        continue;
      }
      const frames = ring.headers[word + 2];
      const flags = ring.headers[word + 3];
      if (frames <= 0 || frames > this.quantumFrames) {
        read = (read + 1) & SAB_WRAP_MASK;
        control[RING.ERRORS] += 1;
        continue;
      }
      if (allowed <= 0) break;
      // Every slot plane is pre-zeroed by the writer: the full quantum is copied, as the prelude
      // copies it.
      const slotPlanes = ring.planes[slot];
      for (let channel = 0; channel < ring.channels; channel += 1) {
        staging.set(slotPlanes[channel], channel * this.quantumFrames);
      }
      this.stageRingId(ring);
      const word64 = slot * (SLOT_HEADER_BYTES / 8);
      const result = this.exports.miso_engine_web_v1_source_submit(
        this.handle, ring.idLength, ring.headersI64[word64 + 2], ring.headersI64[word64 + 3],
        ring.channels, frames, (flags & SLOT_FLAG_END_OF_REGION) !== 0 ? 1 : 0,
      );
      // The engine's ring is full: the chunk stays where it is, and the writer is held back.
      if (result === RESULT_BACKPRESSURE) break;
      read = (read + 1) & SAB_WRAP_MASK;
      if (result === RESULT_OK) {
        // Published only after the engine accepted the PCM (the main-thread start gate).
        Atomics.store(control, RING.SUBMITTED_GENERATION_TAG, ring.headers[word + 1]);
        control[RING.SUBMITTED] += 1;
        allowed -= 1;
        queued += 1;
        if ((flags & SLOT_FLAG_END_OF_REGION) !== 0) {
          ring.finished = true;
          control[RING.FINISHED] = 1;
        }
      } else {
        control[RING.REFUSED] += 1;
        control[RING.LAST_RESULT] = result;
      }
    }
    Atomics.store(control, RING.READ_INDEX, read);
    control[RING.DEPTH] = queued;
  }

  /// The prelude's block accounting on the engine's counts (#1387 attempt 3): every rendered
  /// quantum is a drain block, and the underruns are the engine's own missing in-region frames for
  /// this source (its seek report; held frames, an anchored seek before its anchor, are not
  /// missing) while a live writer feeds the ring and its region has not ended, in quanta. The
  /// quanta come from the cumulative counted frames, never rounded per pass, so a region's end
  /// counts as the prelude counts it. A pass whose reads are refused carries its quanta to the
  /// next. It also ends a consumed seek's transition: render reports that seek's generation.
  accountBlocks(ring, elapsed, report, before) {
    const control = ring.control;
    if (report === null || before === null) {
      ring.pendingElapsed += elapsed;
      return;
    }
    const quanta = ring.pendingElapsed + elapsed;
    ring.pendingElapsed = 0;
    if (ring.seekGeneration !== null && report.generation >= ring.seekGeneration) {
      // Evidence that the seek took effect: render reports its generation. Never room alone.
      ring.seekGeneration = null;
      ring.seekAdmit = false;
    }
    if (ring.reportUnderrun !== null) {
      const missing = report.underrun - ring.reportUnderrun;
      if (ring.finished || Atomics.load(control, RING.WRITER_STATE) !== 1) ring.excludedFrames += missing;
      else ring.countedFrames += missing;
      const counted = Number((ring.countedFrames + BigInt(this.quantumFrames) - 1n) / BigInt(this.quantumFrames));
      control[RING.UNDERRUNS] += counted - ring.countedQuanta;
      ring.countedQuanta = counted;
    }
    ring.reportUnderrun = report.underrun;
    control[RING.DRAIN_BLOCKS] += quanta;
  }


  /// The prelude's seek rule: a published plain seek is applied through `source_seek` before any
  /// slot is read; an anchored one only once `source_seek_at` answered OK for the same tuple.
  applySharedSeek(ring, epoch = Atomics.load(ring.control, RING.SEEK_EPOCH), generation, frame) {
    const control = ring.control;
    if (epoch === ring.seenEpoch) return RESULT_OK;
    const requested = generation ?? ring.controlI64[0];
    const requestedFrame = frame ?? ring.controlI64[1];
    const anchor = ring.controlI64[2];
    if (anchor !== NO_ANCHOR) {
      if (ring.anchoredSample !== anchor || ring.anchoredGeneration !== requested
          || ring.anchoredFrame !== requestedFrame) {
        return RESULT_BACKPRESSURE;
      }
      ring.anchoredSample = NO_ANCHOR;
      return this.consumeSharedSeek(ring, epoch, requested, false);
    }
    this.stageRingId(ring);
    let result = this.exports.miso_engine_web_v1_source_seek(this.handle, ring.idLength, requested, requestedFrame);
    // #1644 D2: a session seek at this generation already moved the source, so its own seek is
    // refused as stale; that one refusal is the seek applied.
    if (result === RESULT_INVALID_ARGUMENT && BigInt.asUintN(64, requested) === this.lastSessionSeekGeneration
        && this.refusedAsStaleGeneration()) {
      result = RESULT_OK;
    }
    if (result === RESULT_BACKPRESSURE) return result;
    if (result !== RESULT_OK) {
      control[RING.REFUSED] += 1;
      control[RING.LAST_RESULT] = result;
      return result;
    }
    return this.consumeSharedSeek(ring, epoch, requested, true);
  }

  /// `plain`: a plain seek, whose stale chunks render drops at its next block, so its new
  /// generation may fill the headroom; an anchored seek's old generation keeps playing until its
  /// anchor, so its new chunks queue behind it within the headroom rule.
  consumeSharedSeek(ring, epoch, generation, plain) {
    const control = ring.control;
    ring.seenEpoch = epoch;
    control[RING.SEEKS_APPLIED] += 1;
    // Root ruling 1: a plain seek's new generation may fill the headroom until render reports
    // this generation in effect (`accountBlocks`).
    ring.seekGeneration = BigInt.asUintN(64, generation);
    ring.seekAdmit = plain;
    ring.finished = false;
    control[RING.FINISHED] = 0;
    return RESULT_OK;
  }

  refusedAsStaleGeneration() {
    const pointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_DIAGNOSTIC);
    const capacity = this.exports.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_DIAGNOSTIC);
    if (!u32(pointer) || pointer === 0 || !u32(capacity) || capacity < STALE_GENERATION_CODE.length) return false;
    const view = this.view(Uint8Array, pointer, STALE_GENERATION_CODE.length);
    return STALE_GENERATION_CODE.every((byte, index) => view[index] === byte);
  }

  /// #1297b: an OK `source_seek_at` is recorded on the ring of that source.
  recordAnchoredSeek(message) {
    const id = new TextEncoder().encode(message.sourceId);
    for (const ring of this.rings) {
      if (ring.idLength !== id.length || !id.every((byte, index) => ring.idBytes[index] === byte)) continue;
      ring.anchoredGeneration = BigInt.asIntN(64, message.generation);
      ring.anchoredFrame = BigInt.asIntN(64, message.sourceFrame);
      ring.anchoredSample = BigInt.asIntN(64, message.anchorSample);
    }
  }

  /// The prelude's `prepare-seek`: apply each ring's published seek and free its stale slots,
  /// while the context is suspended (the SDK checks it); a newer seek supersedes the request.
  prepareSharedSeeks(delivered, seeks) {
    if (!this.ready || this.disposed || this.disposing || this.stickyResult !== RESULT_OK
        || !Array.isArray(seeks) || seeks.length !== delivered.length) {
      return { kind: "refused", result: RESULT_WRONG_STATE };
    }
    const rings = delivered.map((shared) => this.rings.find((ring) => ring.shared === shared));
    if (rings.some((ring) => ring === undefined)) return { kind: "refused", result: RESULT_WRONG_STATE };
    const matches = (ring, seek) => seek && seek.epoch !== 0
      && Atomics.load(ring.control, RING.SEEK_EPOCH) === seek.epoch
      && Atomics.load(ring.controlI64, 0) === seek.generation
      && Atomics.load(ring.controlI64, 1) === seek.frame;
    if (!rings.every((ring, index) => matches(ring, seeks[index]))) return { kind: "superseded" };
    for (let index = 0; index < rings.length; index += 1) {
      const ring = rings[index];
      if (!matches(ring, seeks[index])) return { kind: "superseded" };
      const result = this.applySharedSeek(ring, seeks[index].epoch, seeks[index].generation, seeks[index].frame);
      if (result !== RESULT_OK) return { kind: "refused", result };
      if (!this.releaseStaleSharedSlots(ring, seeks[index].generation)) return { kind: "refused", result: RESULT_INVALID_ARGUMENT };
    }
    return rings.every((ring, index) => matches(ring, seeks[index])) ? { kind: "confirmed", seeks } : { kind: "superseded" };
  }

  releaseStaleSharedSlots(ring, generation) {
    const control = ring.control;
    const write = Atomics.load(control, RING.WRITE_INDEX);
    const capacityMask = ring.capacity - 1;
    let read = control[RING.READ_INDEX];
    for (let remaining = ring.capacity; read !== write && remaining > 0; remaining -= 1) {
      const slot = read & capacityMask;
      if (ring.headers[slot * (SLOT_HEADER_BYTES / 4)] !== read) {
        control[RING.TORN] += 1;
        Atomics.store(control, RING.READ_INDEX, read);
        return false;
      }
      // Future-generation PCM belongs to a superseding seek and is never discarded.
      if (BigInt.asUintN(64, ring.headersI64[slot * (SLOT_HEADER_BYTES / 8) + 2]) < BigInt.asUintN(64, generation)) {
        read = (read + 1) & SAB_WRAP_MASK;
        control[RING.STALE] += 1;
        continue;
      }
      break;
    }
    Atomics.store(control, RING.READ_INDEX, read);
    return true;
  }

  /// Root ruling R-S7-meters (2): after every control call that may publish a plan or grow the
  /// memory, name the newest committed replacement's meter header and the memory's length to the
  /// worklet, whose message handler cuts the views before render adopts that plan.
  postMeterView() {
    // Nothing is named once a dispose asked for the release: the worklet renders no more.
    if (this.disposed || this.disposing || this.handle === 0 || this.workletPort === undefined) return;
    // An `i32` result: normalised to its `u32` address (the memory is capped at 512 MiB, 8192
    // pages, so no address reaches 2 GiB; the normalisation is belt and braces).
    const header = this.exports.miso_engine_web_v1_successor_header_ptr(this.handle) >>> 0;
    const memoryBytes = this.buffer.byteLength;
    if (header === this.postedHeader && memoryBytes === this.postedMemoryBytes) return;
    this.postedHeader = header;
    this.postedMemoryBytes = memoryBytes;
    this.workletPort.postMessage({ tag: "miso.meter-view.v1", header, memoryBytes });
  }

  /// D4: the worklet releases the render half and posts its token; this Worker adopts it back,
  /// disposes the host (the only place that frees it) and answers the page's dispose.
  /// A dispose request that arrives once a dispose finished (the host freed, or the attempt
  /// failed: `disposed` is set either way) is answered at once with that outcome in
  /// `onPageMessage`: it never joins a dispose that will not complete.
  ///
  /// #1332 S8: the release is bounded: when no release arrives within `RELEASE_TIMEOUT_MS`, the
  /// host is disposed with no token and the page is answered `release-timeout`.
  beginDispose(requestId) {
    this.disposeRequestIds.push(requestId);
    if (this.disposing) return;
    this.disposing = true;
    this.releaseDeadline = clock() + RELEASE_TIMEOUT_MS;
    this.workletPort.postMessage({ tag: "miso.release.v1" });
    // The deadline runs on the service tick (this Worker's one timer).
    if (this.serviceTick === null) this.startServiceTick();
  }

  finishDispose(token, reason = undefined) {
    if (this.disposeRequestIds.length === 0 || this.disposeOutcome !== undefined) return;
    this.stopServiceTick();
    const e = this.exports;
    let result;
    try {
      // A half that never came back (a zero token) is left where it is: dispose then marks the
      // lifecycle disposed, so that half silences, and frees the control half (#1332 S6).
      // #1332 S8: with no token, the boot's token is tried: a half the worklet never adopted (its
      // construction failed first) is still in the registry under it and comes back here. A
      // token the worklet consumed matches no entry, and nothing changes.
      if (token !== 0n) e.miso_engine_web_v1_host_adopt(token);
      else if (this.bootToken !== 0n) e.miso_engine_web_v1_host_adopt(this.bootToken);
      // Every status read still in flight is answered before the host is freed.
      while (this.statusReads.length > 0) this.answerStatus(this.statusReads.shift(), null);
      result = e.miso_engine_web_v1_dispose(this.handle);
    } catch (_) {
      // A trap in the adoption or the dispose: the instance is unusable; the outcome is final.
      result = RESULT_INTERNAL;
    }
    this.disposed = true;
    this.handle = 0;
    this.closeFeeds();
    this.disposeOutcome = result;
    this.disposeReason = reason;
    for (const requestId of this.disposeRequestIds) this.answerDispose(requestId);
    this.disposeRequestIds = [];
  }

  /// The answer to a dispose once it finished: the typed teardown when the release timed out,
  /// the dispose's result otherwise (255 for a boot that never finished).
  answerDispose(requestId) {
    if (this.disposeReason !== undefined) {
      this.post({ tag: "miso.teardown.v1", requestId, result: RESULT_INTERNAL, reason: this.disposeReason });
    } else {
      this.acknowledge(requestId, this.disposeOutcome ?? RESULT_INTERNAL);
    }
  }

  /// #1332 S8: a half released after this Worker disposed its host (the release came late) is an
  /// orphan: adopted into the empty slot, it is a render-only session here, which this instance's
  /// dispose frees (it bound no reservation).
  reclaimOrphan(token) {
    if (token === 0n) return;
    try {
      if (this.exports.miso_engine_web_v1_host_adopt(token) === RESULT_OK) {
        this.exports.miso_engine_web_v1_dispose(Number(token >> 32n));
      }
    } catch (_) {
      // A trapped instance frees nothing more; the memory goes with the page.
    }
  }

  onWorkletMessage(message) {
    if (message?.tag === "miso.released.v1" && exactFields(message, ["tag", "token"])
        && typeof message.token === "bigint") {
      const token = BigInt.asUintN(64, message.token);
      if (this.disposeOutcome !== undefined) this.reclaimOrphan(token);
      else this.finishDispose(token);
      return;
    }
    if (message?.tag === "miso.render-clock.v1" && exactFields(message, ["tag", "status"])
        && u32(message.status) && message.status !== 0 && this.renderStatusPointer === 0) {
      this.renderStatusPointer = message.status;
      return;
    }
    if (message?.tag === "miso.render-status.v1" && exactFields(message, ["tag", "requestId", "status"])
        && this.statusReads[0] === message.requestId) {
      this.statusReads.shift();
      this.answerStatus(message.requestId, message.status);
      return;
    }
    this.fatal(RESULT_INTERNAL, this.lastRequestId);
  }

  /// The host status in `worker` mode: the render fields are the worklet's (its render half's
  /// status, its own counters), the control fields are this Worker's: the lifecycle state when it
  /// is not ready, the last result when the render half reports none, the blocking rebuilds (0 in
  /// this mode: the rebuilds run here, never on the audio thread) and the memory's length.
  ///
  /// With no render fields (`null`: the worklet released its half for a dispose), the answer is
  /// this Worker's own status record, whose render clock is the one at the last release.
  answerStatus(requestId, rendered) {
    let control;
    try {
      control = this.readStatus();
    } catch (_) {
      this.fatal(RESULT_INTERNAL, requestId);
      return;
    }
    if (rendered !== null && typeof rendered !== "object") {
      this.fatal(RESULT_INTERNAL, requestId);
      return;
    }
    const render = rendered ?? {
      ...control, singleModeControlAllocations: 0, staleViewQuanta: 0, meterFramesSkipped: 0,
    };
    this.post({
      tag: "miso.status.v1",
      requestId,
      result: RESULT_OK,
      ...render,
      state: control.state !== STATE_READY ? control.state : render.state,
      lastResult: render.lastResult || control.lastResult,
      blockingRebuilds: control.blockingRebuilds,
      memoryBytes: this.buffer.byteLength,
    });
  }
}

new ControlWorker(globalThis);
