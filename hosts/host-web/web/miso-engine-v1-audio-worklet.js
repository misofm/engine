const ABI_VERSION = 0x00010000;
const BOOT_OPTIONS_BYTES = 64;
// Issue #143 D5: the fixed structure carrying the sample window an `f32` frame cannot hold.
// Issue #1209 D2 appended `submix_count` at 64 and its pad at 68. Issue #1290d1 made the pad
// `retired` and appended the frame and successor pointers: the frame travels with the plan, so the
// header names its own frame and, while a replacement plan waits, the replacement's header.
// Issue #1562b appended `structure_epoch` (u64 at 80): the control list's structure epoch of the
// plan's strip order.
const METER_HEADER_BYTES = 88;
const METER_HEADER_RETIRED = 68;
const METER_HEADER_FRAME_POINTER = 72;
const METER_HEADER_SUCCESSOR_POINTER = 76;
const METER_HEADER_STRUCTURE_EPOCH = 80;
const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_BACKPRESSURE = 6;
const RESULT_UNSUPPORTED = 7;
const RESULT_REPREPARE_REQUIRED = 9;
const COMMAND_REASON_UNSUPPORTED_KIND = 7;
const RESULT_INTERNAL = 255;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_DIAGNOSTIC = 4;
const BUFFER_OUTPUT_PCM = 5;
const BUFFER_COMMAND = 6;
const BUFFER_METER_FRAME = 7;
const STATE_READY = 2;
// Issue #1294a: the 96-byte watermark record `miso_engine_web_v1_watermark_ptr` names.
const WATERMARK_BYTES = 96;
// The outcome flag each of the record's three counts (exact, transition fallback, superseded) names.
const WATERMARK_COUNT_FLAGS = [1n, 2n, 4n];
// Issue #1297a: the 64-byte seek report record `miso_engine_web_v1_source_seek_report_ptr` names,
// in the C ABI's order: struct size, a reserved word, then generation, first sample, source frame,
// cumulative underrun frames and cumulative held frames at these offsets.
const SEEK_REPORT_BYTES = 64;
const SEEK_REPORT_WORDS = [
  ["generation", 8], ["firstSample", 16], ["sourceFrame", 24],
  ["cumulativeUnderrunFrames", 32], ["cumulativeHeldFrames", 40],
];
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";

// Issue #137 D1/D2/D3.
const COMMAND_RECORD_BYTES = 272;
const MAXIMUM_COMMAND_RECORDS = 256;
const COMMAND_REPORT_BYTES = 48;
const OBSERVATION_SELECTION_BYTES = 32;
const OBSERVATION_RESULT_BYTES = 96;
const MAXIMUM_OBSERVATION_READS = MAXIMUM_COMMAND_RECORDS;
const OBSERVATION_MAP_FIELDS = [
  "trackIndex", "rack", "effectIndex", "effectSlotId", "nativeEffectId", "tapIds",
];
const OBSERVATION_ADDRESS_FIELDS = ["trackIndex", "rack", "effectIndex", "tapId", "channels"];
const OBSERVATION_CHANNELS = new Set([1, 2, 3]);
const OBSERVATION_STATUSES = new Set([1, 2, 3]);
const OBSERVATION_ROW_FIELDS = [
  "trackIndex", "rack", "effectIndex", "tapId", "channels", "status", "sampleRateHz",
  "firstSample", "endSample", "sequence", "blocks", "leftPresent", "rightPresent", "left", "right",
];
// The telemetry window: 128 blocks is ~341 ms at 48 kHz with a 128-frame quantum, long enough for
// a millisecond-resolution clock to accumulate a usable ratio and short enough to be a live meter.
const TELEMETRY_WINDOW_BLOCKS = 128;

// Live-track response capture uses one fixed raw snapshot staging area. The Worklet only fills
// the request/identity staging and copies the completed bytes; all grid validation and numerical
// analysis happen in the existing analysis Worker through the separate Rust export.
const LIVE_RESPONSE_REQUEST_BYTES = 48;
const LIVE_RESPONSE_MAXIMUM_ID_BYTES = 127;
const LIVE_RESPONSE_MAXIMUM_POINTS = 4096;
const LIVE_RESPONSE_CAPTURE_GRID_LINEAR = 1;
const LIVE_RESPONSE_CAPTURE_GRID_LOGARITHMIC = 2;
const LIVE_RESPONSE_CAPTURE_CHANNEL_LEFT = 1;
const LIVE_RESPONSE_CAPTURE_CHANNEL_RIGHT = 2;
const LIVE_RESPONSE_CAPTURE_CHANNEL_BOTH = 3;
const TRACK_RESPONSE_FIELDS = [
  "tag", "requestId", "trackId", "grid", "channels", "points", "minimumHz", "maximumHz",
  "maximumResultBytes",
];
const SPECTRUM_REQUEST_BYTES = 40;
const SPECTRUM_COLLECTION_REQUEST_BYTES = 32;
const SPECTRUM_COLLECTION_ENTRY_BYTES = 24;
const SPECTRUM_MAXIMUM_ID_BYTES = 127;
const SPECTRUM_MAXIMUM_BYTES = 1 << 20;
const SPECTRUM_STREAM_METADATA_BYTES = 120;
const SPECTRUM_TARGET_TRACK_POST_INPUT = 1;
const SPECTRUM_TARGET_TRACK_POST_PAN = 2;
const SPECTRUM_TARGET_OUTPUT = 3;
const SPECTRUM_CHANNEL_LEFT = 1;
const SPECTRUM_CHANNEL_RIGHT = 2;
const SPECTRUM_CHANNEL_BOTH = 3;
const SPECTRUM_STREAM_STATUSES = new Set([0, 1, 2, 3, 4, 5, 6]);

const INIT_FIELDS = ["module", "document", "options"];
const OPTION_FIELDS = [
  "sourceRingFrames", "maximumMemoryBytes", "reserved1", "liveControlMeterBlocks",
  "liveControlObservationTaps", "reserved2", "spectrumHopFrames", "spectrum",
  "spectrumCollection",
];
const SPECTRUM_OPTION_FIELDS = OPTION_FIELDS.filter((field) => field !== "spectrumHopFrames");
const LEGACY_OPTION_FIELDS = SPECTRUM_OPTION_FIELDS.slice(0, -2);
const SPECTRUM_HOP_FRAMES = new Set([0, 256, 512, 1024, 2048]);
const SOURCE_FIELDS = [
  "tag", "requestId", "sourceId", "generation", "startFrame", "sampleRateHz", "planes", "frames",
  "endOfRegion",
];
const SEEK_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame"];
// Issue #1644 D1 (root R1): one session seek message for both exports. `anchorSample` is always
// present; `-1n` is a plain seek, any other value the absolute render sample of the anchored form.
const SESSION_SEEK_FIELDS = ["tag", "requestId", "generation", "sourceFrame", "anchorSample"];
// Issue #1297a: the anchored source seek and the seek report, both control work of the handler.
const SEEK_AT_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame", "anchorSample"];
const SEEK_REPORT_FIELDS = ["tag", "requestId", "sourceId"];
const COMMAND_FIELDS = ["tag", "requestId", "count", "records"];
const CONTROL_FIELDS = ["tag", "requestId", "frame"];
/// Issue #1697: a `SESSION_REPLACE` frame carries its document beside it.
const CONTROL_DOCUMENT_FIELDS = [...CONTROL_FIELDS, "document"];
// Issue #1570b D4: the two event lanes in drain order (the C ABI's lane codes: reliable, lossy) and
// the most `control_event` calls one drain makes on a lane. The queues hold at most 2 reliable
// events, 2 render diagnostics and 1 telemetry event, so a bound of 16 is never the limit.
const CONTROL_EVENT_LANES = [0, 1];
const MAXIMUM_CONTROL_EVENTS_PER_LANE = 16;
const LEASE_FIELDS = ["tag", "requestId", "enabled"];
const SPECTRUM_FIELDS = ["tag", "requestId", "operation", "channels"];
const SPECTRUM_STREAM_START_FIELDS = ["tag", "requestId", "operation", "smoothingMs"];
const SPECTRUM_STREAM_READ_FIELDS = ["tag", "requestId", "operation", "buffer"];
const SPECTRUM_STREAM_STOP_FIELDS = ["tag", "requestId", "operation"];
const SPECTRUM_SELECT_FIELDS = ["tag", "requestId", "operation", "target", "targetId", "channels"];
const SPECTRUM_STREAM_SELECT_FIELDS = [
  "tag", "requestId", "operation", "target", "targetId", "channels", "smoothingMs",
];

// Decision 12 (#1096): an effect is addressed as `1` inserts or `3` console. The retired `simd1`
// (`0`) and `simd2` (`2`) codes are refused and never reallocated.
const RACK_INSERTS = 1;
const RACK_CONSOLE = 3;
const effectRack = (rack) => rack === RACK_INSERTS || rack === RACK_CONSOLE;

function validObservationAddress(address) {
  return exactFields(address, OBSERVATION_ADDRESS_FIELDS)
    && u32(address.trackIndex)
    && effectRack(address.rack)
    && u32(address.effectIndex)
    && u32(address.tapId) && address.tapId > 0
    && u32(address.channels) && OBSERVATION_CHANNELS.has(address.channels);
}

/// The render clock (issue #137 D3).
///
/// `WorkletGlobalScope` does not include `Performance` in the specification, and user agents
/// disagree about exposing it, so the clock is probed once at construction and the telemetry frame
/// reports the resolution it actually got. `currentTime` is deliberately not used: it advances by
/// exactly one quantum per block no matter how long the render took, so it can measure the
/// deadline but never the work.
function renderClock() {
  const now = globalThis.performance?.now;
  if (typeof now === "function") {
    return { read: () => globalThis.performance.now(), resolutionMs: 0.005 };
  }
  return { read: () => Date.now(), resolutionMs: 1 };
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

function validU64(value) {
  return typeof value === "bigint" && value >= 0n && value <= 0xffffffffn;
}

// Issue #207: the whole `u64` range, not the configuration words' 32-bit window above. A source
// region is declared in sample frames and the introspection queries return it as a `u64`, so the
// check that reads one back has to admit the range the ABI actually carries.
//
// The lower bound is not decoration. These are the first exports whose *result* is a Wasm `i64`,
// and the JS API converts an `i64` result to a BigInt by its **signed** interpretation: a returned
// value at or above `2 ** 63` arrives negative. At 48 kHz that is six million years of frames, so
// it cannot arise from a real session -- but it is exactly what a mis-wired export would produce,
// and the `>= 0n` leg is what turns that into a refused initialization instead of a negative
// region a consumer would carry.
function u64(value) {
  return typeof value === "bigint" && value >= 0n && value <= 0xffffffffffffffffn;
}

function writeBoundedUtf8(value, memoryBuffer, pointer, capacity) {
  let byteLength = 0;
  for (let index = 0; index < value.length; index += 1) {
    let codePoint = value.charCodeAt(index);
    if (codePoint >= 0xd800 && codePoint <= 0xdbff) {
      const low = value.charCodeAt(index + 1);
      if (low >= 0xdc00 && low <= 0xdfff) {
        codePoint = 0x10000 + ((codePoint - 0xd800) << 10) + low - 0xdc00;
        index += 1;
      } else {
        codePoint = 0xfffd;
      }
    } else if (codePoint >= 0xdc00 && codePoint <= 0xdfff) {
      codePoint = 0xfffd;
    }
    byteLength += codePoint <= 0x7f ? 1 : codePoint <= 0x7ff ? 2 : codePoint <= 0xffff ? 3 : 4;
    if (byteLength > capacity) return -1;
  }

  const destination = new Uint8Array(memoryBuffer, pointer, byteLength);
  let offset = 0;
  for (let index = 0; index < value.length; index += 1) {
    let codePoint = value.charCodeAt(index);
    if (codePoint >= 0xd800 && codePoint <= 0xdbff) {
      const low = value.charCodeAt(index + 1);
      if (low >= 0xdc00 && low <= 0xdfff) {
        codePoint = 0x10000 + ((codePoint - 0xd800) << 10) + low - 0xdc00;
        index += 1;
      } else {
        codePoint = 0xfffd;
      }
    } else if (codePoint >= 0xdc00 && codePoint <= 0xdfff) {
      codePoint = 0xfffd;
    }

    if (codePoint <= 0x7f) {
      destination[offset] = codePoint;
      offset += 1;
    } else if (codePoint <= 0x7ff) {
      destination[offset] = (codePoint >>> 6) | 0xc0;
      destination[offset + 1] = (codePoint & 0x3f) | 0x80;
      offset += 2;
    } else if (codePoint <= 0xffff) {
      destination[offset] = (codePoint >>> 12) | 0xe0;
      destination[offset + 1] = ((codePoint >>> 6) & 0x3f) | 0x80;
      destination[offset + 2] = (codePoint & 0x3f) | 0x80;
      offset += 3;
    } else {
      destination[offset] = (codePoint >>> 18) | 0xf0;
      destination[offset + 1] = ((codePoint >>> 12) & 0x3f) | 0x80;
      destination[offset + 2] = ((codePoint >>> 6) & 0x3f) | 0x80;
      destination[offset + 3] = (codePoint & 0x3f) | 0x80;
      offset += 4;
    }
  }
  return byteLength;
}

function boundedUtf8Length(value, capacity) {
  let byteLength = 0;
  for (let index = 0; index < value.length; index += 1) {
    let codePoint = value.charCodeAt(index);
    if (codePoint >= 0xd800 && codePoint <= 0xdbff) {
      const low = value.charCodeAt(index + 1);
      if (low >= 0xdc00 && low <= 0xdfff) {
        codePoint = 0x10000 + ((codePoint - 0xd800) << 10) + low - 0xdc00;
        index += 1;
      } else {
        codePoint = 0xfffd;
      }
    } else if (codePoint >= 0xdc00 && codePoint <= 0xdfff) {
      codePoint = 0xfffd;
    }
    byteLength += codePoint <= 0x7f ? 1 : codePoint <= 0x7ff ? 2 : codePoint <= 0xffff ? 3 : 4;
    if (byteLength > capacity) return -1;
  }
  return byteLength;
}

class MisoEngineAudioWorkletProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.ready = false;
    this.disposed = false;
    this.stickyResult = RESULT_OK;
    this.lastRequestId = 0;
    this.handle = 0;
    this.handleDisposed = false;
    this.initializationErrorPosted = false;
    // Issue #137: both leases start released, so a session that never asks for live controls pays
    // a single `false` test per block and nothing else.
    this.meterLease = false;
    this.telemetryLease = false;
    this.meterSequence = 0;
    this.telemetryBlocks = 0;
    this.telemetryElapsedMs = 0;
    this.telemetryBudgetMs = 0;
    this.telemetryPeakMs = 0;
    this.telemetryDeadlineMisses = 0;
    this.observations = Object.freeze([]);
    this.observationSelectionView = null;
    // Issue #1562 (batch 7 CI fold): every persistent view over the module's memory is registered
    // here by `bindView`, so a growth of that memory (a control allocation, counted on the control
    // side and no fault) re-cuts them all in `rebindMemoryViews`, in a message handler and never
    // in `process()`. `staleViewQuanta` counts the quanta `process()` silenced because it found
    // the memory moved before a handler re-bound; the status reports it.
    this.viewSpecs = [];
    this.staleViewQuanta = 0;
    // Issue #1290d1: the meter frame travels with the plan, so the views over it do too. The
    // active set is the views over the frame the host publishes into; the pending set was cut, in
    // the message handler that committed a replacement plan, over that plan's header and frame.
    // `process()` switches from the one to the other by reference when the header says the plan
    // was replaced, and never cuts a view.
    this.meterSet = null;
    this.pendingMeterSet = null;
    // Issue #1294a: the watermark `(revision, first_sample)` pair last posted (or read at boot).
    this.watermarkRevision = -1n;
    this.watermarkSample = -1n;
    // The record's three outcome counts as of that post: a count that rose since names a flag.
    this.watermarkCounts = [0n, 0n, 0n];
    // Issue #1644 D2: the generation of the last session seek the engine accepted (undefined:
    // none). The SDK's feed drain runs on this same object, between the same handler calls, and
    // consumes a ring seek at exactly this generation without seeking the source again.
    this.lastSessionSeekGeneration = undefined;
    // The session diagnostic staging, where a refused call writes its core code (null: none).
    this.diagnosticView = null;
    this.clock = renderClock();
    // Issue #1381a D6, `single` mode: the instance's control work runs here, in the message
    // handler, outside `process()`. The service step follows every message, the main realm's
    // `miso.service.v1` tick included, so a displaced plan is reclaimed here and never in render.
    this.port.onmessage = (event) => {
      try {
        this.receive(event.data);
        this.rebindMemoryViews();
        // A service tick is no request: an error it finds belongs to none, never to the last
        // request the page already saw answered.
        const requestId = event.data?.tag === "miso.service.v1" ? 0 : this.lastRequestId;
        this.serviceControl(requestId);
        this.rebindMemoryViews();
        this.pushWatermark(requestId);
        this.drainControlEvents(requestId);
      } finally {
        this.rebindMemoryViews();
        // A command may have committed a replacement plan, whose frame is sized for its strips:
        // its views are cut here, before any quantum can adopt it (#1290d1).
        this.syncMeterViews();
      }
    };
    try {
      const result = this.initialize(options?.processorOptions);
      if (result !== RESULT_OK) this.failInitialization(result, 0);
    } catch (_) {
      this.failInitialization(RESULT_INTERNAL, 0);
    }
  }

  initialize(init) {
    const currentOptions = exactFields(init?.options, OPTION_FIELDS);
    const oldSpectrumOptions = exactFields(init?.options, SPECTRUM_OPTION_FIELDS);
    const oldOptions = init?.options?.spectrum === undefined
      && exactFields(init?.options, LEGACY_OPTION_FIELDS);
    if (!exactFields(init, INIT_FIELDS) || !(init.document instanceof Uint8Array)
        || (!currentOptions && !oldSpectrumOptions && !oldOptions)) {
      return RESULT_INVALID_ARGUMENT;
    }
    const spectrumHopFrames = currentOptions ? init.options.spectrumHopFrames : 0;
    if (!SPECTRUM_HOP_FRAMES.has(spectrumHopFrames)) return RESULT_INVALID_ARGUMENT;
    this.instance = new WebAssembly.Instance(init.module, {});
    this.exports = this.instance.exports;
    if (this.exports.miso_engine_web_v1_abi_version() !== ABI_VERSION) {
      return 2;
    }
    let spectrumHopBoot;
    if (spectrumHopFrames !== 0) {
      const capability = this.exports.miso_engine_web_v1_spectrum_hop_capability;
      spectrumHopBoot = this.exports.miso_engine_web_v1_boot_with_spectrum_hop;
      if (typeof capability !== "function" || typeof spectrumHopBoot !== "function") return 2;
      let supported;
      try {
        supported = capability();
      } catch (_) {
        return 2;
      }
      if (supported !== 1) return 2;
    }
    const optionsPointer = this.exports.miso_engine_web_v1_boot_options_ptr();
    if (!u32(optionsPointer) || optionsPointer === 0) {
      return RESULT_INTERNAL;
    }
    try {
      this.writeBootOptions(optionsPointer, init.options);
    } catch (_) {
      return RESULT_INVALID_ARGUMENT;
    }
    const hasSpectrum = init.options.spectrum !== null && init.options.spectrum !== undefined;
    const hasSpectrumCollection = init.options.spectrumCollection !== null
      && init.options.spectrumCollection !== undefined;
    if (hasSpectrum && hasSpectrumCollection) {
      return RESULT_INVALID_ARGUMENT;
    }
    const spectrumResult = hasSpectrumCollection
      ? this.stageSpectrumCollectionRequest(init.options.spectrumCollection)
      : this.stageSpectrumRequest(init.options.spectrum);
    if (spectrumResult !== RESULT_OK) return spectrumResult;
    const documentPointer = this.exports.miso_engine_web_v1_document_ptr(init.document.byteLength);
    if (!u32(documentPointer) || documentPointer === 0) {
      return this.exports.miso_engine_web_v1_boot_result();
    }
    new Uint8Array(this.exports.memory.buffer, documentPointer, init.document.byteLength)
      .set(init.document);
    this.handle = spectrumHopFrames === 0
      ? this.exports.miso_engine_web_v1_boot(init.document.byteLength)
      : spectrumHopBoot(init.document.byteLength, spectrumHopFrames);
    if (this.handle === 0) {
      return this.exports.miso_engine_web_v1_boot_result();
    }
    this.backend = "simd128";
    // Boot allocated the observation, live-response and spectrum stagings (issue #1333 D5), so
    // these queries allocate nothing and cannot grow linear memory. Read every one before
    // caching any views all the same: a view taken before a growth would be detached.
    const observationStagingPointer = this.exports.miso_engine_web_v1_observation_id_ptr();
    if (!u32(observationStagingPointer) || observationStagingPointer === 0) return RESULT_INTERNAL;
    const trackResponseRequestPointer = this.exports.miso_engine_web_v1_track_response_request_ptr();
    const trackResponseRequestBytes = this.exports.miso_engine_web_v1_track_response_request_bytes();
    const trackResponseTrackIdPointer = this.exports.miso_engine_web_v1_track_response_track_id_ptr();
    const trackResponseTrackIdCapacity = this.exports.miso_engine_web_v1_track_response_track_id_capacity();
    const trackResponseSnapshotPointer = this.exports.miso_engine_web_v1_track_response_snapshot_ptr();
    const trackResponseSnapshotCapacity = this.exports.miso_engine_web_v1_track_response_snapshot_capacity();
    if (!u32(trackResponseRequestPointer) || trackResponseRequestPointer === 0
        || trackResponseRequestBytes !== LIVE_RESPONSE_REQUEST_BYTES
        || !u32(trackResponseTrackIdPointer) || trackResponseTrackIdPointer === 0
        || trackResponseTrackIdCapacity !== LIVE_RESPONSE_MAXIMUM_ID_BYTES
        || !u32(trackResponseSnapshotPointer) || trackResponseSnapshotPointer === 0
        || !u32(trackResponseSnapshotCapacity) || trackResponseSnapshotCapacity === 0) {
      return RESULT_INTERNAL;
    }
    this.memoryBuffer = this.exports.memory.buffer;
    this.bindView("trackResponseRequestView", DataView,
      trackResponseRequestPointer, trackResponseRequestBytes);
    this.bindView("trackResponseTrackIdView", Uint8Array,
      trackResponseTrackIdPointer, trackResponseTrackIdCapacity);
    this.trackResponseSnapshotPointer = trackResponseSnapshotPointer;
    this.trackResponseSnapshotCapacity = trackResponseSnapshotCapacity;
    this.sourceIdPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_ID);
    this.sourceIdCapacity = this.exports.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SOURCE_ID);
    this.sourcePcmPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_PCM);
    const sourcePcmCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_SOURCE_PCM,
    );
    const outputPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_OUTPUT_PCM);
    const outputCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_OUTPUT_PCM,
    );
    const statusPointer = this.exports.miso_engine_web_v1_status_ptr(this.handle);
    const resourcePointer = this.exports.miso_engine_web_v1_resource_ptr(this.handle);
    if (!u32(this.sourceIdPointer) || this.sourceIdPointer === 0 || !u32(this.sourceIdCapacity)
        || !u32(this.sourcePcmPointer) || this.sourcePcmPointer === 0
        || !u32(sourcePcmCapacity) || sourcePcmCapacity % 4 !== 0
        || !u32(outputPointer) || outputPointer === 0
        || !u32(statusPointer) || statusPointer === 0
        || !u32(resourcePointer) || resourcePointer === 0) return RESULT_INTERNAL;
    this.bindView("statusView", DataView, statusPointer, 80);
    this.bindView("resourceView", DataView, resourcePointer, 224);
    const status = this.readStatus();
    this.sampleRateHz = status.sampleRateHz;
    this.quantumFrames = status.quantumFrames;
    if (this.sampleRateHz !== globalThis.sampleRate
        || this.quantumFrames !== (globalThis.renderQuantumSize ?? 128)
        || this.quantumFrames === 0 || sourcePcmCapacity % (this.quantumFrames * 4) !== 0
        || outputCapacity !== this.quantumFrames * 2 * 4) {
      return RESULT_REPREPARE_REQUIRED;
    }
    this.maximumSourceChannels = sourcePcmCapacity / (this.quantumFrames * 4);
    this.bindView("sourcePcm", Float32Array, this.sourcePcmPointer, sourcePcmCapacity / 4);
    // Issue #1644 D2 (R4 amended): the feed drain tells a stale-generation seek refusal from any
    // other `invalidArgument` by the code the refusal wrote here. A module without the staging
    // leaves the view null, and the drain then handles every refusal as before.
    const diagnosticPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_DIAGNOSTIC);
    const diagnosticCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_DIAGNOSTIC,
    );
    if (u32(diagnosticPointer) && diagnosticPointer !== 0
        && u32(diagnosticCapacity) && diagnosticCapacity > 0) {
      this.bindView("diagnosticView", Uint8Array, diagnosticPointer, diagnosticCapacity);
    }
    this.bindView("outputLeft", Float32Array, outputPointer, this.quantumFrames);
    this.bindView("outputRight", Float32Array,
      outputPointer + this.quantumFrames * 4, this.quantumFrames);
    this.resources = this.readResources();
    const expectedBackend = 1;
    if (this.resources.backend !== expectedBackend || status.backend !== expectedBackend
        || this.resources.sampleRateHz !== this.sampleRateHz
        || this.resources.quantumFrames !== this.quantumFrames
        || status.state !== STATE_READY || status.lastResult !== RESULT_OK
        || status.nextAbsoluteSample !== 0n || status.renderedQuanta !== 0n) {
      return RESULT_INVALID_ARGUMENT;
    }
    if (!this.bindLiveControls(init)) return RESULT_INTERNAL;
    const watermarkPointer = this.exports.miso_engine_web_v1_watermark_ptr(this.handle);
    if (!u32(watermarkPointer) || watermarkPointer === 0) return RESULT_INTERNAL;
    this.bindView("watermarkView", DataView, watermarkPointer, WATERMARK_BYTES);
    // Issue #1297a: the seek report record is at a stable address, bound once here.
    const seekReportPointer = this.exports.miso_engine_web_v1_source_seek_report_ptr(this.handle);
    if (!u32(seekReportPointer) || seekReportPointer === 0) return RESULT_INTERNAL;
    this.bindView("seekReportView", DataView, seekReportPointer, SEEK_REPORT_BYTES);
    // The boot watermark is the baseline, not an advance: only a later change is posted.
    if (this.exports.miso_engine_web_v1_watermark_read(this.handle) === RESULT_OK) {
      this.watermarkRevision = this.watermarkView.getBigUint64(8, true);
      this.watermarkSample = this.watermarkView.getBigUint64(16, true);
      this.watermarkCounts = [32, 40, 48].map((offset) => this.watermarkView.getBigUint64(offset, true));
    }
    this.memoryBytes = this.memoryBuffer.byteLength;
    this.port.postMessage({
      tag: "miso.ready.v1",
      requestId: 0,
      result: RESULT_OK,
      backend: this.backend,
      resources: this.resources,
      memoryBytes: this.memoryBytes,
    });
    this.ready = true;
    return RESULT_OK;
  }

  /// Issue #207, #1562b: the compiled session's sources in canonical order, read from the engine's
  /// exports, or `null` for a row the engine cannot have written. Every read is checked against
  /// something the engine already guarantees, so a mis-wired export fails here rather than
  /// surfacing as a plausible-looking wrong number: compilation refuses a zero channel count or
  /// full-source frame count, and the staging holds the most channels a source can
  /// declare (`maximumSourceChannels`), so a channel count above it is a mis-wired export. The
  /// sources are the committed model's, so a rebuild that adds or removes one is seen by the next
  /// read. Control side only; allocates.
  readSourceRows() {
    const count = this.exports.miso_engine_web_v1_source_count(this.handle);
    if (!u32(count)) return null;
    const rows = [];
    for (let index = 0; index < count; index += 1) {
      const length = this.exports.miso_engine_web_v1_source_id(this.handle, index);
      if (!u32(length) || length === 0 || length > this.sourceIdCapacity) return null;
      const bytes = new Uint8Array(this.memoryBuffer, this.sourceIdPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return null;
        id += String.fromCharCode(bytes[byte]);
      }
      const channels = this.exports.miso_engine_web_v1_source_channels(this.handle, index);
      const frames = this.exports.miso_engine_web_v1_source_frames(this.handle, index);
      if (!u32(channels) || channels === 0 || channels > this.maximumSourceChannels
          || !u64(frames) || frames === 0n) return null;
      rows.push({ id, channels, frames });
    }
    return rows;
  }

  /// Issue #143, #1562b: the observation bindings of the plan that renders, in the prepared
  /// owner's stable map order, or `null` for a binding the engine cannot have written. A binding
  /// names a strip by its index in that plan's strip order (the tracks, then the submixes), which
  /// is the control list's order at [`Self::renderingStructureEpoch`]. A rebuild changes the
  /// bindings at the block that adopts it, so every map request reads them again: a reboot or a
  /// swap cannot retain an old numeric owner address behind a stable SDK selection. Control side
  /// only; allocates.
  readObservationBindings() {
    const count = this.exports.miso_engine_web_v1_observation_count(this.handle);
    if (!u32(count)) return null;
    const idPointer = this.exports.miso_engine_web_v1_observation_id_ptr();
    const idCapacity = this.exports.miso_engine_web_v1_observation_id_capacity();
    if (!u32(idPointer) || idPointer === 0 || !u32(idCapacity) || idCapacity === 0) return null;
    const readId = (length) => {
      if (!u32(length) || length === 0 || length > idCapacity) return null;
      const bytes = new Uint8Array(this.memoryBuffer, idPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return null;
        id += String.fromCharCode(bytes[byte]);
      }
      return id;
    };
    const bindings = [];
    for (let index = 0; index < count; index += 1) {
      const trackIndex = this.exports.miso_engine_web_v1_observation_track_index(this.handle, index);
      const rack = this.exports.miso_engine_web_v1_observation_rack(this.handle, index);
      const effectIndex = this.exports.miso_engine_web_v1_observation_effect_index(this.handle, index);
      const effectSlotId = readId(
        this.exports.miso_engine_web_v1_observation_effect_slot_id(this.handle, index),
      );
      const nativeEffectId = readId(
        this.exports.miso_engine_web_v1_observation_native_effect_id(this.handle, index),
      );
      const tapCount = this.exports.miso_engine_web_v1_observation_tap_count(this.handle, index);
      if (!u32(trackIndex) || !effectRack(rack) || !u32(effectIndex)
          || effectSlotId === null || nativeEffectId === null || !u32(tapCount)
          || tapCount === 0 || tapCount > 32) return null;
      const tapIds = [];
      for (let tapIndex = 0; tapIndex < tapCount; tapIndex += 1) {
        const tapId = this.exports.miso_engine_web_v1_observation_tap_id(
          this.handle, index, tapIndex,
        );
        if (!u32(tapId) || tapId === 0) return null;
        tapIds.push(tapId);
      }
      bindings.push(Object.freeze({
        trackIndex,
        rack,
        effectIndex,
        effectSlotId,
        nativeEffectId,
        tapIds: Object.freeze(tapIds),
      }));
    }
    return Object.freeze(bindings);
  }

  /// Issue #1562b: the structure epoch of the plan that renders, read from its meter header (the
  /// header the module publishes through `meter_header_ptr` is the rendering plan's, with or
  /// without a meter lease). It is the number of the control list whose strip order the plan's
  /// bindings and frames follow. `0` for a header the memory cannot hold. Control side only.
  renderingStructureEpoch() {
    const pointer = this.exports.miso_engine_web_v1_meter_header_ptr(this.handle);
    if (!u32(pointer) || pointer === 0
        || pointer + METER_HEADER_BYTES > this.memoryBuffer.byteLength) return 0;
    const header = new DataView(this.memoryBuffer, pointer, METER_HEADER_BYTES);
    return header.getUint32(METER_HEADER_STRUCTURE_EPOCH, true)
      + header.getUint32(METER_HEADER_STRUCTURE_EPOCH + 4, true) * 4294967296;
  }

  /// Cut one persistent view over the module's memory and remember how, so a growth of that memory
  /// can cut it again. Control side only.
  bindView(field, Constructor, pointer, length) {
    this.viewSpecs.push({ field, Constructor, pointer, length });
    this[field] = new Constructor(this.memoryBuffer, pointer, length);
  }

  /// Issue #1562 (batch 7 CI fold, root ruling A, #1332 D5 "growth is not a fault"): a control
  /// call may grow the module's linear memory, which detaches every view cut before. Re-cut them
  /// all against the live buffer. The module's addresses are stable across a growth, so each view
  /// keeps its pointer and length. Runs at the end of every message-handler path that can call
  /// into the module, and never in `process()`, which only compares the buffers and silences.
  rebindMemoryViews() {
    const buffer = this.exports?.memory.buffer;
    if (buffer === undefined || buffer === this.memoryBuffer) return;
    this.memoryBuffer = buffer;
    for (const spec of this.viewSpecs) {
      this[spec.field] = new spec.Constructor(buffer, spec.pointer, spec.length);
    }
    if (this.meterSet !== null) this.cutMeterViews(this.meterSet, buffer);
    if (this.pendingMeterSet !== null) this.cutMeterViews(this.pendingMeterSet, buffer);
  }

  /// Issue #1290d1: cut the views of one meter set over `buffer` and remember nothing else; the
  /// set's pointers and counts are its identity. Control side only.
  cutMeterViews(set, buffer) {
    const strips = set.trackCount + set.submixCount;
    set.header = new DataView(buffer, set.headerPointer, METER_HEADER_BYTES);
    set.frame = new Float32Array(buffer, set.framePointer, strips * 3 + 3);
    // Fixed views over the one buffer, built here and never again: the frozen render-callback
    // policy forbids `subarray` inside it, and rightly -- a per-block view is a per-block
    // allocation. With buses the master pair no longer follows the tracks, so the message's
    // `2T + 2` peaks are assembled from the track view and the master view.
    set.trackPeaks = new Float32Array(buffer, set.framePointer, set.trackCount * 2);
    set.submixPeaks = new Float32Array(
      buffer, set.framePointer + set.trackCount * 2 * 4, set.submixCount * 2);
    set.masterPeaks = new Float32Array(buffer, set.framePointer + strips * 2 * 4, 2);
    set.trackGain = new Float32Array(
      buffer, set.framePointer + (strips * 2 + 2) * 4, set.trackCount);
    set.submixGain = new Float32Array(
      buffer, set.framePointer + (strips * 2 + 2 + set.trackCount) * 4, set.submixCount);
  }

  /// Issue #1290d1: the meter set for the header at `headerPointer`: the views over the header and
  /// over the frame the header names, and the message that frame is posted in. `null` for a header
  /// or frame the module's memory cannot hold. The strip counts are the plan's, so a plan with
  /// another strip count gets a set of its own. Control side only; allocates.
  cutMeterSet(headerPointer) {
    const buffer = this.memoryBuffer;
    if (!u32(headerPointer) || headerPointer === 0
        || headerPointer + METER_HEADER_BYTES > buffer.byteLength) return null;
    const header = new DataView(buffer, headerPointer, METER_HEADER_BYTES);
    // The header owns the strip counts; the strip order is the core's control list (the owner
    // header rows), which the frame's sections follow.
    if (header.getUint32(0, true) !== METER_HEADER_BYTES
        || header.getUint32(4, true) !== ABI_VERSION) return null;
    const trackCount = header.getUint32(8, true);
    const submixCount = header.getUint32(64, true);
    const framePointer = header.getUint32(METER_HEADER_FRAME_POINTER, true);
    // Issue #1209 D1: the frame is `3(T + S) + 3` words -- a peak pair per strip (the tracks,
    // then the submixes), the master pair, then one non-negative gain-reduction magnitude per
    // strip in the same order and the master's.
    const words = (trackCount + submixCount) * 3 + 3;
    if (framePointer === 0 || framePointer % 4 !== 0
        || framePointer + words * 4 > buffer.byteLength) return null;
    const set = {
      headerPointer,
      framePointer,
      trackCount,
      submixCount,
      masterGainIndex: words - 1,
      header,
      frame: null,
      trackPeaks: null,
      submixPeaks: null,
      masterPeaks: null,
      trackGain: null,
      submixGain: null,
      message: {
        tag: "miso.meter.v1",
        sequence: 0,
        generation: 0n,
        validity: 0,
        lossCount: 0,
        windows: 0,
        trackCount,
        // Issue #1562b: the structure epoch of the control list the frame's strip order belongs
        // to, so a page that holds a list of another epoch reads the list again before it names
        // a frame's strips. Refreshed with every post; a safe integer (it advances by one at a
        // structural commit).
        structureEpoch: 0,
        // The frozen `2T + 2` track and master peaks: an existing reader indexes it exactly as
        // before, whatever the submix count.
        peaks: new Float32Array(trackCount * 2 + 2),
        // Issue #143: one non-negative decibel magnitude per track. Positional and always finite;
        // `0` deliberately conflates "not reducing" with "no observed effect", because the array
        // is read without null checks and the distinction lives in the subscription map.
        trackGrDb: new Float32Array(trackCount),
        masterGrDb: null,
        firstSample: 0n,
        endSample: 0n,
        // Issue #1209 D4: the buses, appended. `submixPeaks` is `[bus0 L, bus0 R, ..]`, and
        // `submixGrDb` one non-negative decibel magnitude per bus, in canonical submix order.
        submixCount,
        submixPeaks: new Float32Array(submixCount * 2),
        submixGrDb: new Float32Array(submixCount),
      },
    };
    this.cutMeterViews(set, buffer);
    return set;
  }

  /// Issue #1290d1: bring the page's meter sets in line with the host's, in a message handler. The
  /// active set follows the header the host publishes through (a swap the render path did not
  /// switch for, a lease released across it, is picked up here), and the pending set follows the
  /// header the active one names as its successor: the replacement plan a command just committed,
  /// whose views are cut now so the quantum that adopts it only switches to them. A header or
  /// frame the module's memory cannot hold is a protocol fault, sticky.
  syncMeterViews() {
    if (!this.metersAttached || this.disposed || !this.ready || this.stickyResult !== RESULT_OK
        || this.handle === 0) return;
    this.rebindMemoryViews();
    const current = this.exports.miso_engine_web_v1_meter_header_ptr(this.handle);
    let active = this.meterSet;
    if (active === null || active.headerPointer !== current) {
      active = this.cutMeterSet(current);
      if (active === null) {
        this.sticky(RESULT_INTERNAL, this.lastRequestId);
        return;
      }
      this.meterSet = active;
      this.pendingMeterSet = null;
    }
    const successor = active.header.getUint32(METER_HEADER_SUCCESSOR_POINTER, true);
    if (successor === 0) {
      this.pendingMeterSet = null;
      return;
    }
    if (this.pendingMeterSet === null || this.pendingMeterSet.headerPointer !== successor) {
      const pending = this.cutMeterSet(successor);
      if (pending === null) {
        this.sticky(RESULT_INTERNAL, this.lastRequestId);
        return;
      }
      this.pendingMeterSet = pending;
    }
  }

  stageSpectrumRequest(options) {
    if (options === null || options === undefined) return RESULT_OK;
    const target = options?.target === "trackPostInput"
      ? SPECTRUM_TARGET_TRACK_POST_INPUT
      : options?.target === "trackPostPan"
        ? SPECTRUM_TARGET_TRACK_POST_PAN
        : options?.target === "output" ? SPECTRUM_TARGET_OUTPUT : 0;
    const channels = options?.channels === "left"
      ? SPECTRUM_CHANNEL_LEFT
      : options?.channels === "right"
        ? SPECTRUM_CHANNEL_RIGHT
        : options?.channels === "both" ? SPECTRUM_CHANNEL_BOTH : 0;
    if (!exactFields(options, ["target", "targetId", "channels", "maximumCaptureBytes"])
        || target === 0 || channels === 0
        || typeof options.targetId !== "string" || options.targetId.length === 0
        || !u32(options.maximumCaptureBytes) || options.maximumCaptureBytes === 0
        || options.maximumCaptureBytes > SPECTRUM_MAXIMUM_BYTES) {
      return RESULT_INVALID_ARGUMENT;
    }
    const requestPointer = this.exports.miso_engine_web_v1_spectrum_request_ptr();
    const requestBytes = this.exports.miso_engine_web_v1_spectrum_request_bytes();
    const idPointer = this.exports.miso_engine_web_v1_spectrum_target_id_ptr();
    const idCapacity = this.exports.miso_engine_web_v1_spectrum_target_id_capacity();
    if (!u32(requestPointer) || requestPointer === 0 || requestBytes !== SPECTRUM_REQUEST_BYTES
        || !u32(idPointer) || idPointer === 0 || idCapacity !== SPECTRUM_MAXIMUM_ID_BYTES) {
      return RESULT_INTERNAL;
    }
    const idBytes = writeBoundedUtf8(options.targetId, this.exports.memory.buffer, idPointer, idCapacity);
    if (idBytes <= 0) return RESULT_INVALID_ARGUMENT;
    const view = new DataView(this.exports.memory.buffer, requestPointer, requestBytes);
    view.setUint32(0, SPECTRUM_REQUEST_BYTES, true);
    view.setUint32(4, ABI_VERSION, true);
    view.setUint32(8, target, true);
    view.setUint32(12, channels, true);
    view.setUint32(16, idBytes, true);
    view.setUint32(20, 0, true);
    view.setBigUint64(24, BigInt(options.maximumCaptureBytes), true);
    view.setUint32(32, 0, true);
    view.setUint32(36, 0, true);
    return RESULT_OK;
  }

  stageSpectrumCollectionRequest(options) {
    if (options === null || options === undefined) return RESULT_OK;
    if (!exactFields(options, ["entries", "maximumCaptureBytes"])
        || !Array.isArray(options.entries) || options.entries.length === 0
        || options.entries.length > 0xffffffff
        || !Number.isSafeInteger(options.maximumCaptureBytes)
        || options.maximumCaptureBytes <= 0) {
      return RESULT_INVALID_ARGUMENT;
    }
    const requestPointer = this.exports.miso_engine_web_v1_spectrum_collection_request_ptr();
    const requestBytes = this.exports.miso_engine_web_v1_spectrum_collection_request_bytes();
    if (!u32(requestPointer) || requestPointer === 0 || requestBytes !== SPECTRUM_COLLECTION_REQUEST_BYTES) {
      return RESULT_INTERNAL;
    }
    let bytes = this.exports.memory.buffer;
    const request = new DataView(bytes, requestPointer, requestBytes);
    request.setUint32(0, SPECTRUM_COLLECTION_REQUEST_BYTES, true);
    request.setUint32(4, ABI_VERSION, true);
    request.setUint32(8, options.entries.length, true);
    request.setUint32(12, 0, true);
    request.setBigUint64(16, BigInt(options.maximumCaptureBytes), true);
    request.setUint32(24, 0, true);
    request.setUint32(28, 0, true);

    const entryPointer = this.exports.miso_engine_web_v1_spectrum_collection_entry_ptr();
    const entryBytes = this.exports.miso_engine_web_v1_spectrum_collection_entry_bytes();
    const entryCapacity = this.exports.miso_engine_web_v1_spectrum_collection_entry_capacity();
    if (!u32(entryPointer) || entryPointer === 0 || entryBytes !== SPECTRUM_COLLECTION_ENTRY_BYTES
        || entryCapacity !== options.entries.length) {
      return RESULT_INTERNAL;
    }
    bytes = this.exports.memory.buffer;
    const entries = new DataView(bytes, entryPointer, entryCapacity * entryBytes);
    const encodedIds = [];
    for (let index = 0; index < options.entries.length; index += 1) {
      const entry = options.entries[index];
      const target = entry?.target === "trackPostInput"
        ? SPECTRUM_TARGET_TRACK_POST_INPUT
        : entry?.target === "trackPostPan"
          ? SPECTRUM_TARGET_TRACK_POST_PAN
          : entry?.target === "output" ? SPECTRUM_TARGET_OUTPUT : 0;
      const channels = entry?.channels === "left"
        ? SPECTRUM_CHANNEL_LEFT
        : entry?.channels === "right"
          ? SPECTRUM_CHANNEL_RIGHT
          : entry?.channels === "both" ? SPECTRUM_CHANNEL_BOTH : 0;
      if (!exactFields(entry, ["target", "targetId", "channels"])
          || target === 0 || channels === 0
          || typeof entry.targetId !== "string" || entry.targetId.length === 0) {
        return RESULT_INVALID_ARGUMENT;
      }
      const idBytes = boundedUtf8Length(entry.targetId, SPECTRUM_MAXIMUM_ID_BYTES);
      if (idBytes <= 0) {
        return RESULT_INVALID_ARGUMENT;
      }
      encodedIds.push({ value: entry.targetId, bytes: idBytes });
      const offset = entryPointer + index * entryBytes;
      entries.setUint32(offset - entryPointer, target, true);
      entries.setUint32(offset - entryPointer + 4, channels, true);
      entries.setUint32(offset - entryPointer + 8, idBytes, true);
      entries.setUint32(offset - entryPointer + 12, 0, true);
      entries.setUint32(offset - entryPointer + 16, 0, true);
      entries.setUint32(offset - entryPointer + 20, 0, true);
    }
    const totalIdBytes = encodedIds.reduce((total, id) => total + id.bytes, 0);
    const idPointer = this.exports.miso_engine_web_v1_spectrum_collection_target_ids_ptr();
    const idCapacity = this.exports.miso_engine_web_v1_spectrum_collection_target_ids_capacity();
    if (!u32(idPointer) || idPointer === 0 || idCapacity !== totalIdBytes) {
      return RESULT_INTERNAL;
    }
    bytes = this.exports.memory.buffer;
    let idOffset = idPointer;
    for (const id of encodedIds) {
      const written = writeBoundedUtf8(id.value, bytes, idOffset, id.bytes);
      if (written !== id.bytes) return RESULT_INVALID_ARGUMENT;
      idOffset += id.bytes;
    }
    return RESULT_OK;
  }

  /// Bind every live-control view and preallocate every frame the render callback posts.
  ///
  /// Issue #137: nothing in `process()` may allocate, so every message body, every typed-array
  /// copy and every track identity is built here, once, on the construction path that is already
  /// allowed to allocate. `process()` mutates the frozen shapes and posts them; the structured
  /// clone `postMessage` performs is the only allocation left, and it is the one the ABI cannot
  /// avoid.
  bindLiveControls(init) {
    // Issue #1565a D4: the command staging exists exactly when observation taps are configured.
    this.liveControlsAttached = init.options.liveControlObservationTaps !== 0n;
    const commandPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_COMMAND);
    const commandCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_COMMAND,
    );
    const reportPointer = this.exports.miso_engine_web_v1_command_report_ptr(this.handle);
    if (!u32(reportPointer) || reportPointer === 0) return false;
    this.bindView("commandReport", DataView, reportPointer, COMMAND_REPORT_BYTES);
    if (this.liveControlsAttached) {
      if (!u32(commandPointer) || commandPointer === 0
          || commandCapacity !== MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES) return false;
      this.bindView("commandStaging", Uint8Array, commandPointer, commandCapacity);
    } else if (commandPointer !== 0 || commandCapacity !== 0) {
      // Released live controls must own no staging at all; a nonzero row here would mean the engine
      // charged for a buffer the ABI says does not exist.
      return false;
    }

    // Issue #207: source introspection and the observation bindings are control-path state, read
    // here once at boot and again by the request handlers (a rebuild can change both, #1562b). The
    // construction path and the handlers are the ones that may allocate; `process()` never touches
    // them. A headless consumer of the session map cannot otherwise learn which sources a
    // compiled session declares, how many channels each carries, or which frames it is waiting
    // for; the ABI exposed track discovery and nothing at all about sources.
    const sources = this.readSourceRows();
    if (sources === null) return false;
    this.sources = sources;
    this.sourceCount = sources.length;
    const observations = this.readObservationBindings();
    if (observations === null) return false;
    this.observations = observations;
    const observationSelectionPointer = this.exports.miso_engine_web_v1_observation_selection_ptr();
    const observationSelectionBytes = this.exports.miso_engine_web_v1_observation_selection_bytes();
    const observationSelectionCapacity = this.exports.miso_engine_web_v1_observation_selection_capacity();
    if (!u32(observationSelectionPointer) || observationSelectionPointer === 0
        || observationSelectionBytes !== OBSERVATION_SELECTION_BYTES
        || !u32(observationSelectionCapacity)
        || observationSelectionCapacity < 1
        || observationSelectionCapacity > MAXIMUM_OBSERVATION_READS) return false;
    this.bindView("observationSelectionView", DataView, observationSelectionPointer,
      observationSelectionCapacity * observationSelectionBytes);

    const framePointer = this.exports.miso_engine_web_v1_buffer_ptr(
      this.handle,
      BUFFER_METER_FRAME,
    );
    const frameCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_METER_FRAME,
    );
    this.metersAttached = init.options.liveControlMeterBlocks !== 0n;
    this.observationAttached = init.options.liveControlObservationTaps !== 0n;
    if (this.metersAttached) {
      const headerPointer = this.exports.miso_engine_web_v1_meter_header_ptr(this.handle);
      if (!u32(headerPointer) || headerPointer === 0) return false;
      const set = this.cutMeterSet(headerPointer);
      // The buffer exports and the header describe one frame: the capacity check rejects a header
      // whose counts do not match the frame the engine sized.
      if (set === null || !u32(framePointer) || framePointer !== set.framePointer
          || frameCapacity !== (set.trackCount + set.submixCount) * 3 * 4 + 12) return false;
      this.meterSet = set;
    }
    this.telemetryMessage = {
      tag: "miso.telemetry.v1",
      sequence: 0,
      blocks: TELEMETRY_WINDOW_BLOCKS,
      cpuPercent: 0,
      peakBlockMs: 0,
      meanBlockMs: 0,
      budgetMs: 0,
      deadlineMisses: 0,
      resolutionMs: this.clock.resolutionMs,
      belowResolution: true,
    };
    return true;
  }

  failInitialization(result, requestId) {
    if (this.handle !== 0 && !this.handleDisposed) {
      this.handleDisposed = true;
      try {
        this.exports.miso_engine_web_v1_dispose(this.handle);
      } catch (_) {
        // Cleanup is best-effort only when a malformed module throws from its dispose export.
      }
    }
    this.handle = 0;
    this.ready = false;
    this.disposed = true;
    this.stickyResult = result;
    if (!this.initializationErrorPosted) {
      this.initializationErrorPosted = true;
      try {
        this.port.postMessage({ tag: "miso.error.v1", requestId, result });
      } catch (_) {
        // A failed MessagePort cannot receive the one address-free construction error.
      }
    }
  }

  writeBootOptions(pointer, options) {
    if (!u32(options.sourceRingFrames) || !validU64(options.maximumMemoryBytes)) {
      throw new RangeError("Invalid boot option");
    }
    const liveControlWords = [
      options.reserved1, options.liveControlMeterBlocks,
      options.liveControlObservationTaps, options.reserved2,
    ];
    if (liveControlWords.some((value) => !validU64(value))) {
      throw new RangeError("Invalid live-control boot option");
    }
    // Offsets 32 and 56 are reserved and must be zero (issue #1565a D4, issue #1689 D2): a
    // writer that still sets a retired word is refused, never ignored.
    if (options.reserved1 !== 0n || options.reserved2 !== 0n) {
      throw new RangeError("Invalid observation boot option");
    }
    const view = new DataView(this.exports.memory.buffer, pointer, BOOT_OPTIONS_BYTES);
    view.setUint32(0, BOOT_OPTIONS_BYTES, true);
    view.setUint32(4, ABI_VERSION, true);
    view.setUint32(8, globalThis.sampleRate, true);
    view.setUint32(12, globalThis.renderQuantumSize ?? 128, true);
    view.setUint32(16, options.sourceRingFrames, true);
    view.setUint32(20, 0, true);
    view.setBigUint64(24, options.maximumMemoryBytes, true);
    liveControlWords.forEach((value, index) => view.setBigUint64(32 + index * 8, value, true));
  }

  readResources() {
    const view = this.resourceView;
    // Issue #143: the report's first reserved word became `observationRetainedBytes`; the other
    // three are still required zero and the 224-byte layout is unchanged.
    if (view.getUint32(0, true) !== 224 || view.getUint32(4, true) !== ABI_VERSION
        || view.getUint32(20, true) !== 0 || view.getUint32(24, true) !== 0
        || view.getUint32(28, true) !== 0
        || [200, 208, 216].some((offset) => view.getBigUint64(offset, true) !== 0n)) {
      throw new RangeError("Invalid resource report");
    }
    const names = [
      "optionsBytes", "statusBytes", "sessionDocumentBytes", "diagnosticBytes", "idStagingBytes",
      "sourcePcmStagingBytes", "outputPcmBytes", "bridgeMetadataBytes", "bridgeRetainedBytes",
      "largestBridgeAllocationBytes", "sourceTotalBytes", "sourceOverheadBytes",
      "effectScalarStateBytes", "effectScalarScratchBytes", "builtinRetainedBytes",
      "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes", "graphMetadataBytes",
      "graphDelayBytes", "largestNamedAllocationBytes",
      // Issue #143: carved from the report's first reserved word; zero for a session prepared
      // with `liveControlObservationTaps === 0n`.
      "observationRetainedBytes",
    ];
    const resources = {
      sampleRateHz: view.getUint32(8, true),
      quantumFrames: view.getUint32(12, true),
      backend: view.getUint32(16, true),
    };
    names.forEach((name, index) => { resources[name] = view.getBigUint64(32 + index * 8, true); });
    return Object.freeze(resources);
  }

  readStatus() {
    const view = this.statusView;
    if (view.getUint32(0, true) !== 80 || view.getUint32(4, true) !== ABI_VERSION) {
      throw new RangeError("Invalid status report");
    }
    return Object.freeze({
      state: view.getUint32(8, true),
      lastResult: this.stickyResult || view.getUint32(12, true),
      backend: view.getUint32(16, true),
      sampleRateHz: view.getUint32(20, true),
      quantumFrames: view.getUint32(24, true),
      // Issue #1294b: the structural commands this instance rebuilt in its own message handler
      // (a page without cross-origin isolation), counted by the engine.
      blockingRebuilds: view.getUint32(28, true),
      nextAbsoluteSample: view.getBigUint64(32, true),
      renderedQuanta: view.getBigUint64(40, true),
      liveValuesSuperseded: view.getBigUint64(48, true),
      // Issue #1349 D2: the applied-revision watermark's three words, copied by the engine from
      // the report of the block that advanced it. The worklet assigns no flag bit of its own.
      appliedRevision: view.getBigUint64(56, true),
      appliedSample: view.getBigUint64(64, true),
      appliedOutcome: view.getBigUint64(72, true),
    });
  }

  transferList(planes) {
    if (!Array.isArray(planes)) return [];
    return [...new Set(planes
      .filter((plane) => plane instanceof Float32Array && plane.buffer instanceof ArrayBuffer)
      .map((plane) => plane.buffer))];
  }

  acknowledge(requestId, result, planes) {
    const message = { tag: "miso.ack.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.port.postMessage(message, this.transferList(planes));
  }

  sticky(result, requestId, planes) {
    this.ready = false;
    this.stickyResult = result;
    const message = { tag: "miso.error.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.port.postMessage(message, this.transferList(planes));
  }

  /// Issue #1381a D5, D6: one control-plane service step. It commits and acknowledges nothing, so a
  /// success posts nothing; an epoch failure is sticky. Its allocations and frees are the
  /// instance's control allocations, which the status reports.
  serviceControl(requestId = this.lastRequestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK || this.handle === 0) return;
    const result = this.exports.miso_engine_web_v1_service(this.handle);
    if (result !== RESULT_OK) this.sticky(result, requestId);
  }

  /// Issue #1294a (D3): after each service step, in the message handler and never in `process()`,
  /// read the applied-revision watermark and post `miso.watermark.v1` when its
  /// `(revision, first_sample)` pair changed. The record's flags cover only the engine's last
  /// advance, so two advances between two reads would lose the first one's flags: the posted
  /// `outcome` also ORs in the flag of each cumulative count that rose since the last post
  /// (exact 1, transition fallback 2, superseded 4), and the newest post loses no flag. A read the engine gave up on (`BACKPRESSURE`, a
  /// bounded seqlock retry) skips this tick and the next message reads again; any other refusal
  /// is sticky, as the service step's is.
  pushWatermark(requestId = this.lastRequestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK || this.handle === 0) return;
    const result = this.exports.miso_engine_web_v1_watermark_read(this.handle);
    if (result === RESULT_BACKPRESSURE) return;
    if (result !== RESULT_OK) {
      this.sticky(result, requestId);
      return;
    }
    this.rebindMemoryViews();
    const view = this.watermarkView;
    if (view.getUint32(0, true) !== WATERMARK_BYTES || view.getUint32(4, true) !== 0) {
      this.sticky(RESULT_INTERNAL, requestId);
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
    this.port.postMessage({
      tag: "miso.watermark.v1",
      revision,
      sample,
      outcome,
      exactCount: counts[0],
      transitionFallbackCount: counts[1],
      supersededCount: counts[2],
    });
  }

  /// Issue #1570b D4: drain the engine's two event lanes after a message and its service step, in
  /// the message handler and never in `process()`. The reliable lane first, then the lossy lane;
  /// each is read until `control_event` returns an empty frame, at most
  /// `MAXIMUM_CONTROL_EVENTS_PER_LANE` calls (more cannot be queued: the rest waits for the next
  /// drain). Each frame is copied out of the module's memory and posted in dequeue order as
  /// `control-event { lane, frame }`. A dequeue the engine refuses stops that lane's drain and
  /// takes the error path once: it is sticky for the host, never a retry loop. Nothing here
  /// decodes a frame, and a commit's reliable events are what keeps the next commit from
  /// backpressure, so this drain is what lets a page send unlimited transactions.
  drainControlEvents(requestId = this.lastRequestId) {
    if (this.disposed || !this.ready || this.stickyResult !== RESULT_OK || this.handle === 0) return;
    for (const lane of CONTROL_EVENT_LANES) {
      for (let calls = 0; calls < MAXIMUM_CONTROL_EVENTS_PER_LANE; calls += 1) {
        const result = this.exports.miso_engine_web_v1_control_event(this.handle, lane);
        this.rebindMemoryViews();
        if (result !== RESULT_OK) {
          this.sticky(result, requestId);
          return;
        }
        const bytes = this.exports.miso_engine_web_v1_control_response_bytes(this.handle);
        if (bytes === 0) break;
        const pointer = this.exports.miso_engine_web_v1_control_response_ptr(this.handle);
        if (pointer === 0) {
          this.sticky(RESULT_INTERNAL, requestId);
          return;
        }
        const frame = new Uint8Array(bytes);
        frame.set(new Uint8Array(this.memoryBuffer, pointer, bytes));
        this.port.postMessage({ tag: "miso.control-event.v1", lane, frame }, [frame.buffer]);
      }
    }
  }

  receive(message) {
    if (this.disposed) return;
    // The main realm's service tick carries no request ID: it is not a request, so it takes no
    // place in the request sequence and gets no reply. The step itself runs after this handler.
    if (message?.tag === "miso.service.v1" && exactFields(message, ["tag"])) return;
    const returnedPlanes = Array.isArray(message?.planes) ? message.planes : undefined;
    if (!Number.isSafeInteger(message?.requestId) || message.requestId <= this.lastRequestId) {
      this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returnedPlanes);
      return;
    }
    this.lastRequestId = message.requestId;
    if (message.tag === "miso.dispose.v1" && exactFields(message, ["tag", "requestId"])) {
      const result = this.exports.miso_engine_web_v1_dispose(this.handle);
      if (result === RESULT_OK) {
        this.disposed = true;
        this.handleDisposed = true;
        this.handle = 0;
        this.lastSessionSeekGeneration = undefined;
      }
      this.acknowledge(message.requestId, result);
      return;
    }
    if (this.stickyResult !== RESULT_OK) {
      this.sticky(this.stickyResult, message.requestId, returnedPlanes);
      return;
    }
    this.rebindMemoryViews();
    if (message?.tag === "miso.source.v1" && exactFields(message, SOURCE_FIELDS)) {
      this.receiveSource(message);
    } else if (message?.tag === "miso.seek.v1" && exactFields(message, SEEK_FIELDS)) {
      this.receiveSeek(message);
    } else if (message?.tag === "miso.session-seek.v1"
        && exactFields(message, SESSION_SEEK_FIELDS)) {
      this.receiveSessionSeek(message);
    } else if (message?.tag === "miso.seek-at.v1" && exactFields(message, SEEK_AT_FIELDS)) {
      this.receiveSeekAt(message);
    } else if (message?.tag === "miso.seek-report.v1"
        && exactFields(message, SEEK_REPORT_FIELDS)) {
      this.receiveSeekReport(message);
    } else if (message?.tag === "miso.command.v1" && exactFields(message, COMMAND_FIELDS)) {
      this.receiveCommand(message);
    } else if (message?.tag === "miso.control.v1"
        && (exactFields(message, CONTROL_FIELDS)
          || exactFields(message, CONTROL_DOCUMENT_FIELDS))) {
      this.receiveControl(message);
    } else if (message?.tag === "miso.meters.v1" && exactFields(message, LEASE_FIELDS)) {
      this.receiveMeterLease(message);
    } else if (message?.tag === "miso.telemetry.v1" && exactFields(message, LEASE_FIELDS)) {
      this.receiveTelemetryLease(message);
    } else if (message?.tag === "miso.sessionmap.v1"
        && exactFields(message, ["tag", "requestId"])) {
      // The sources are the committed model's (a rebuild can change them), read again here.
      const sources = this.readSourceRows();
      if (sources === null) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      this.sources = sources;
      this.sourceCount = sources.length;
      this.port.postMessage({
        tag: "miso.sessionmap.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        sources: this.sources.map((source) => ({
          id: source.id,
          channels: source.channels,
          frames: source.frames,
        })),
        metersAttached: this.metersAttached === true,
      });
    } else if (message?.tag === "miso.observationmap.v1"
        && exactFields(message, ["tag", "requestId"])) {
      // The bindings are the rendering plan's (a rebuild changes them at its adoption block), and
      // the reply names the structure epoch of the control list whose strip order they follow.
      const bindings = this.readObservationBindings();
      if (bindings === null) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      this.observations = bindings;
      this.port.postMessage({
        tag: "miso.observationmap.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        structureEpoch: this.renderingStructureEpoch(),
        bindings: this.observations,
      });
    } else if (message?.tag === "miso.observation.v1"
        && exactFields(message, ["tag", "requestId", "selections"])) {
      this.receiveObservationRead(message);
    } else if (message?.tag === "miso.trackresponse.v1"
        && exactFields(message, TRACK_RESPONSE_FIELDS)) {
      this.receiveTrackResponse(message);
    } else if (message?.tag === "miso.spectrum.v1"
        && (exactFields(message, SPECTRUM_FIELDS)
          || exactFields(message, SPECTRUM_SELECT_FIELDS)
          || exactFields(message, SPECTRUM_STREAM_SELECT_FIELDS)
          || exactFields(message, SPECTRUM_STREAM_START_FIELDS)
          || exactFields(message, SPECTRUM_STREAM_READ_FIELDS)
          || exactFields(message, SPECTRUM_STREAM_STOP_FIELDS))) {
      this.receiveSpectrum(message);
    } else if (message?.tag === "miso.renderallocations.v1"
        && exactFields(message, ["tag", "requestId"])) {
      // Issue #1333 D3: the allocator calls this instance made inside render-locked windows --
      // every export this processor calls on the render thread after boot. Answered here, never
      // in process(); qualification asserts exactly zero.
      this.port.postMessage({
        tag: "miso.renderallocations.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        count: this.exports.miso_engine_web_v1_render_allocation_count(),
      });
    } else if (message?.tag === "miso.status.v1"
        && exactFields(message, ["tag", "requestId"])) {
      this.port.postMessage({
        tag: "miso.status.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        ...this.readStatus(),
        memoryBytes: this.exports.memory.buffer.byteLength,
        // Issue #1381a (#1332 D1): the allocator calls this instance made outside render-locked
        // windows, which in `single` mode is the one place the engine allocates on an audio
        // thread: boot, control messages and service steps.
        singleModeControlAllocations: this.exports.miso_engine_web_v1_control_allocation_count(),
        // Quanta `process()` silenced because the module's memory had moved before a message
        // handler re-bound the views (batch 7 CI fold). Zero when every growth is re-bound in the
        // handler that caused it.
        staleViewQuanta: this.staleViewQuanta,
      });
    } else {
      this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returnedPlanes);
    }
  }

  receiveObservationRead(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || !Array.isArray(message.selections)
        || message.selections.length === 0
        || message.selections.length > MAXIMUM_OBSERVATION_READS
        || !message.selections.every(validObservationAddress)
        || this.observationSelectionView === null) {
      if (this.observationSelectionView === null && Array.isArray(message.selections)
          && message.selections.length > 0 && message.selections.length <= MAXIMUM_OBSERVATION_READS
          && message.selections.every(validObservationAddress)) {
        this.port.postMessage({
          tag: "miso.observation.v1",
          requestId: message.requestId,
          result: RESULT_UNSUPPORTED,
          rows: [],
        });
      } else {
        this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      }
      return;
    }
    const view = this.observationSelectionView;
    const count = message.selections.length;
    for (let index = 0; index < count; index += 1) {
      const selection = message.selections[index];
      const offset = index * OBSERVATION_SELECTION_BYTES;
      view.setUint32(offset, OBSERVATION_SELECTION_BYTES, true);
      view.setUint32(offset + 4, ABI_VERSION, true);
      view.setUint32(offset + 8, selection.trackIndex, true);
      view.setUint32(offset + 12, selection.rack, true);
      view.setUint32(offset + 16, selection.effectIndex, true);
      view.setUint32(offset + 20, selection.tapId, true);
      view.setUint32(offset + 24, selection.channels, true);
      view.setUint32(offset + 28, 0, true);
    }
    const result = this.exports.miso_engine_web_v1_observation_read(this.handle, count);
    if (result !== RESULT_OK) {
      this.port.postMessage({
        tag: "miso.observation.v1",
        requestId: message.requestId,
        result,
        rows: [],
      });
      return;
    }
    const resultPointer = this.exports.miso_engine_web_v1_observation_result_ptr();
    const resultBytes = this.exports.miso_engine_web_v1_observation_result_bytes();
    if (!u32(resultPointer) || resultPointer === 0
        || resultBytes !== count * OBSERVATION_RESULT_BYTES) {
      this.sticky(RESULT_INTERNAL, message.requestId);
      return;
    }
    const output = new DataView(this.memoryBuffer, resultPointer, resultBytes);
    const rows = [];
    for (let index = 0; index < count; index += 1) {
      const offset = index * OBSERVATION_RESULT_BYTES;
      if (output.getUint32(offset, true) !== OBSERVATION_RESULT_BYTES
          || output.getUint32(offset + 4, true) !== ABI_VERSION
          || output.getUint32(offset + 36, true) !== 0
          || output.getUint32(offset + 84, true) !== 0
          || output.getUint32(offset + 88, true) !== 0
          || output.getUint32(offset + 92, true) !== 0) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      const row = Object.freeze({
        trackIndex: output.getUint32(offset + 12, true),
        rack: output.getUint32(offset + 16, true),
        effectIndex: output.getUint32(offset + 20, true),
        tapId: output.getUint32(offset + 24, true),
        channels: output.getUint32(offset + 28, true),
        status: output.getUint32(offset + 8, true),
        sampleRateHz: output.getUint32(offset + 32, true),
        firstSample: output.getBigUint64(offset + 40, true),
        endSample: output.getBigUint64(offset + 48, true),
        sequence: output.getBigUint64(offset + 56, true),
        blocks: output.getUint32(offset + 64, true),
        leftPresent: output.getUint32(offset + 68, true),
        rightPresent: output.getUint32(offset + 72, true),
        left: output.getFloat32(offset + 76, true),
        right: output.getFloat32(offset + 80, true),
      });
      rows.push(row);
    }
    this.port.postMessage({
      tag: "miso.observation.v1",
      requestId: message.requestId,
      result: RESULT_OK,
      rows,
    });
  }

  /// Capture one immutable selected-track response snapshot for the analysis Worker.
  ///
  /// This handler never constructs a response collector or evaluates a curve. Rust copies the
  /// prepared owners into the prewarmed fixed staging area; this side makes one detached byte
  /// copy for the Worker and returns it to the main realm. A capture refusal returns no bytes and
  /// leaves the Worklet's render state untouched.
  receiveTrackResponse(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.trackId !== "string" || message.trackId.length === 0
        || message.trackId.length > LIVE_RESPONSE_MAXIMUM_ID_BYTES
        || !Number.isInteger(message.grid)
        || (message.grid !== LIVE_RESPONSE_CAPTURE_GRID_LINEAR
          && message.grid !== LIVE_RESPONSE_CAPTURE_GRID_LOGARITHMIC)
        || !Number.isInteger(message.channels)
        || (message.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_LEFT
          && message.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_RIGHT
          && message.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_BOTH)
        || !Number.isInteger(message.points) || message.points < 2
        || message.points > LIVE_RESPONSE_MAXIMUM_POINTS
        || typeof message.minimumHz !== "number" || !Number.isFinite(message.minimumHz)
        || typeof message.maximumHz !== "number" || !Number.isFinite(message.maximumHz)
        || !Number.isInteger(message.maximumResultBytes) || message.maximumResultBytes <= 0
        || message.maximumResultBytes > this.trackResponseSnapshotCapacity) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const idBytes = writeBoundedUtf8(
      message.trackId,
      this.memoryBuffer,
      this.trackResponseTrackIdView.byteOffset,
      this.trackResponseTrackIdView.byteLength,
    );
    if (idBytes < 0) {
      const snapshot = new Uint8Array(0);
      this.port.postMessage({
        tag: "miso.trackresponse.v1",
        requestId: message.requestId,
        result: RESULT_INVALID_ARGUMENT,
        snapshot,
      }, [snapshot.buffer]);
      return;
    }
    const request = this.trackResponseRequestView;
    request.setUint32(0, LIVE_RESPONSE_REQUEST_BYTES, true);
    request.setUint32(4, ABI_VERSION, true);
    request.setUint32(8, idBytes, true);
    request.setUint32(12, message.grid, true);
    request.setUint32(16, message.channels, true);
    request.setUint32(20, message.points, true);
    request.setFloat32(24, message.minimumHz, true);
    request.setFloat32(28, message.maximumHz, true);
    request.setUint32(32, message.maximumResultBytes, true);
    request.setUint32(36, 0, true);
    request.setUint32(40, 0, true);
    request.setUint32(44, 0, true);
    let result;
    try {
      result = this.exports.miso_engine_web_v1_track_response_capture(this.handle);
    } catch (_) {
      result = RESULT_INTERNAL;
    }
    this.rebindMemoryViews();
    const resultPointer = this.exports.miso_engine_web_v1_track_response_result_ptr();
    const resultBytes = this.exports.miso_engine_web_v1_track_response_result_bytes();
    if (!u32(resultPointer) || resultPointer === 0 || !u32(resultBytes)
        || resultBytes > this.trackResponseSnapshotCapacity) {
      this.sticky(RESULT_INTERNAL, message.requestId);
      return;
    }
    const snapshot = result === RESULT_OK
      ? new Uint8Array(this.memoryBuffer, resultPointer, resultBytes).slice()
      : new Uint8Array(0);
    this.port.postMessage({
      tag: "miso.trackresponse.v1",
      requestId: message.requestId,
      result,
      snapshot,
    }, [snapshot.buffer]);
  }

  /// Arm, read or cancel the one prepared spectrum observer. The Worklet only copies raw samples;
  /// FFT/window/log analysis remains in the off-render SDK Worker.
  receiveSpectrum(message) {
    const selectionOperation = message.operation === "select"
      || message.operation === "streamSelect";
    if (selectionOperation) {
      const streamSelection = message.operation === "streamSelect";
      if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
          || !exactFields(
            message,
            streamSelection ? [
              "tag", "requestId", "operation", "target", "targetId", "channels", "smoothingMs",
            ] : ["tag", "requestId", "operation", "target", "targetId", "channels"],
          )
          || (message.target !== "trackPostInput"
            && message.target !== "trackPostPan" && message.target !== "output")
          || typeof message.targetId !== "string" || message.targetId.length === 0
          || (message.channels !== "left" && message.channels !== "right" && message.channels !== "both")
          || (streamSelection
            && (typeof message.smoothingMs !== "number" || !Number.isFinite(message.smoothingMs)
              || message.smoothingMs < 0 || message.smoothingMs > 10_000))) {
        this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
        return;
      }
      const target = message.target === "trackPostInput"
        ? SPECTRUM_TARGET_TRACK_POST_INPUT
        : message.target === "trackPostPan"
          ? SPECTRUM_TARGET_TRACK_POST_PAN
          : SPECTRUM_TARGET_OUTPUT;
      const channels = message.channels === "left"
        ? SPECTRUM_CHANNEL_LEFT
        : message.channels === "right" ? SPECTRUM_CHANNEL_RIGHT : SPECTRUM_CHANNEL_BOTH;
      const pointer = this.exports.miso_engine_web_v1_spectrum_target_id_ptr();
      const capacity = this.exports.miso_engine_web_v1_spectrum_target_id_capacity();
      if (!u32(pointer) || pointer === 0 || !u32(capacity) || capacity === 0) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      const idBytes = writeBoundedUtf8(message.targetId, this.memoryBuffer, pointer, capacity);
      if (idBytes <= 0) {
        this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
        return;
      }
      let result;
      try {
        result = streamSelection
          ? this.exports.miso_engine_web_v1_spectrum_stream_select(
            this.handle, target, channels, idBytes, message.smoothingMs,
          )
          : this.exports.miso_engine_web_v1_spectrum_select(
            this.handle, target, channels, idBytes,
          );
      } catch (_) {
        result = RESULT_INTERNAL;
      }
      this.rebindMemoryViews();
      if (!streamSelection) {
        this.port.postMessage({
          tag: "miso.spectrum.v1",
          requestId: message.requestId,
          result,
          operation: message.operation,
        });
        return;
      }
      const metadata = this.readSpectrumStreamMetadata();
      if (metadata === null) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      this.port.postMessage({
        tag: "miso.spectrum.v1",
        requestId: message.requestId,
        result,
        operation: message.operation,
        metadata,
      });
      return;
    }
    const streamOperation = message.operation === "streamStart"
      || message.operation === "streamRead" || message.operation === "streamStop";
    if (streamOperation) {
      if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
          || (message.operation === "streamStart"
            && (typeof message.smoothingMs !== "number" || !Number.isFinite(message.smoothingMs)
              || message.smoothingMs < 0 || message.smoothingMs > 10_000))
          || (message.operation === "streamRead"
            && (!(message.buffer instanceof ArrayBuffer)
              || message.buffer.byteLength === 0 || message.buffer.byteLength > SPECTRUM_MAXIMUM_BYTES))) {
        this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
        return;
      }
      let result;
      try {
        result = message.operation === "streamStart"
          ? this.exports.miso_engine_web_v1_spectrum_stream_start(this.handle, message.smoothingMs)
          : message.operation === "streamStop"
            ? this.exports.miso_engine_web_v1_spectrum_stream_stop(this.handle)
            : this.exports.miso_engine_web_v1_spectrum_stream_read(this.handle);
      } catch (_) {
        result = RESULT_INTERNAL;
      }
      this.rebindMemoryViews();
      const metadata = this.readSpectrumStreamMetadata();
      if (metadata === null) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      if (message.operation === "streamRead") {
        let byteLength = 0;
        if (result === RESULT_OK && metadata.status === 6) {
          const pointer = this.exports.miso_engine_web_v1_spectrum_capture_ptr();
          const bytes = this.exports.miso_engine_web_v1_spectrum_capture_bytes();
          const capacity = this.exports.miso_engine_web_v1_spectrum_capture_capacity();
          if (!u32(pointer) || pointer === 0 || !u32(bytes) || bytes === 0
              || !u32(capacity) || bytes > capacity || bytes > SPECTRUM_MAXIMUM_BYTES
              || bytes > message.buffer.byteLength) {
            result = RESULT_INTERNAL;
          } else {
            new Uint8Array(message.buffer, 0, bytes)
              .set(new Uint8Array(this.memoryBuffer, pointer, bytes));
            byteLength = bytes;
          }
        }
        this.port.postMessage({
          tag: "miso.spectrum.v1",
          requestId: message.requestId,
          result,
          operation: message.operation,
          byteLength,
          buffer: message.buffer,
          metadata,
        }, [message.buffer]);
      } else {
        this.port.postMessage({
          tag: "miso.spectrum.v1",
          requestId: message.requestId,
          result,
          operation: message.operation,
          metadata,
        });
      }
      return;
    }
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || (message.operation !== "arm" && message.operation !== "read" && message.operation !== "cancel")
        || (message.channels !== "left" && message.channels !== "right" && message.channels !== "both")) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const channels = message.channels === "left"
      ? SPECTRUM_CHANNEL_LEFT
      : message.channels === "right" ? SPECTRUM_CHANNEL_RIGHT : SPECTRUM_CHANNEL_BOTH;
    let result;
    try {
      result = message.operation === "arm"
        ? this.exports.miso_engine_web_v1_spectrum_arm(this.handle)
        : message.operation === "cancel"
          ? this.exports.miso_engine_web_v1_spectrum_cancel(this.handle)
          : this.exports.miso_engine_web_v1_spectrum_read(this.handle, channels);
    } catch (_) {
      result = RESULT_INTERNAL;
    }
    this.rebindMemoryViews();
    let snapshot = new Uint8Array(0);
    if (message.operation === "read" && result === RESULT_OK) {
      const pointer = this.exports.miso_engine_web_v1_spectrum_capture_ptr();
      const bytes = this.exports.miso_engine_web_v1_spectrum_capture_bytes();
      const capacity = this.exports.miso_engine_web_v1_spectrum_capture_capacity();
      if (!u32(pointer) || pointer === 0 || !u32(bytes) || bytes === 0
          || !u32(capacity) || bytes > capacity || bytes > SPECTRUM_MAXIMUM_BYTES) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      snapshot = new Uint8Array(this.memoryBuffer, pointer, bytes).slice();
    }
    this.port.postMessage({
      tag: "miso.spectrum.v1",
      requestId: message.requestId,
      result,
      operation: message.operation,
      snapshot,
    }, [snapshot.buffer]);
  }

  readSpectrumStreamMetadata() {
    this.rebindMemoryViews();
    const pointer = this.exports.miso_engine_web_v1_spectrum_stream_metadata_ptr();
    const bytes = this.exports.miso_engine_web_v1_spectrum_stream_metadata_bytes();
    if (!u32(pointer) || pointer === 0 || bytes !== SPECTRUM_STREAM_METADATA_BYTES
        || pointer + bytes > this.memoryBuffer.byteLength) return null;
    const view = new DataView(this.memoryBuffer, pointer, bytes);
    const metadata = {
      structSize: view.getUint32(0, true),
      abiVersion: view.getUint32(4, true),
      result: view.getUint32(8, true),
      status: view.getUint32(12, true),
      target: view.getUint32(16, true),
      channels: view.getUint32(20, true),
      sampleRateHz: view.getUint32(24, true),
      quantumFrames: view.getUint32(28, true),
      hopFrames: view.getUint32(32, true),
      sourceUnderrun: view.getUint32(36, true),
      captureEpoch: view.getBigUint64(48, true),
      sequence: view.getBigUint64(56, true),
      droppedCaptures: view.getBigUint64(64, true),
      windows: view.getBigUint64(72, true),
      capturedSample: view.getBigUint64(80, true),
      endSample: view.getBigUint64(88, true),
      analysisEpoch: view.getBigUint64(96, true),
      historyStartSample: view.getBigUint64(104, true),
      smoothingMs: view.getFloat64(112, true),
    };
    if (metadata.structSize !== SPECTRUM_STREAM_METADATA_BYTES
        || metadata.abiVersion !== ABI_VERSION
        || !SPECTRUM_STREAM_STATUSES.has(metadata.status)
        || metadata.target < SPECTRUM_TARGET_TRACK_POST_INPUT
        || metadata.target > SPECTRUM_TARGET_OUTPUT
        || ![SPECTRUM_CHANNEL_LEFT, SPECTRUM_CHANNEL_RIGHT, SPECTRUM_CHANNEL_BOTH].includes(metadata.channels)
        || metadata.sourceUnderrun > 1
        || !Number.isFinite(metadata.smoothingMs) || metadata.smoothingMs < 0
        || metadata.smoothingMs > 10_000) return null;
    return Object.freeze(metadata);
  }

  receiveSource(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.sourceId !== "string" || typeof message.generation !== "bigint"
        || message.generation <= 0n || typeof message.startFrame !== "bigint"
        || message.startFrame < 0n || message.sampleRateHz !== this.sampleRateHz
        || !u32(message.frames) || message.frames > this.quantumFrames
        || typeof message.endOfRegion !== "boolean" || !Array.isArray(message.planes)
        || message.planes.length === 0 || message.planes.length > this.maximumSourceChannels
        || message.planes.some((plane) => !(plane instanceof Float32Array)
          || !(plane.buffer instanceof ArrayBuffer)
          || (typeof SharedArrayBuffer !== "undefined" && plane.buffer instanceof SharedArrayBuffer)
          || plane.length !== message.frames)) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0, message.planes);
      return;
    }
    const idLength = writeBoundedUtf8(
      message.sourceId,
      this.memoryBuffer,
      this.sourceIdPointer,
      this.sourceIdCapacity,
    );
    if (idLength < 0) {
      this.sticky(4, message.requestId, message.planes);
      return;
    }
    message.planes.forEach((plane, channel) => {
      this.sourcePcm.set(plane, channel * this.quantumFrames);
    });
    const result = this.exports.miso_engine_web_v1_source_submit(
      this.handle, idLength, message.generation, message.startFrame, message.planes.length,
      message.frames, message.endOfRegion ? 1 : 0,
    );
    this.acknowledge(message.requestId, result, message.planes);
  }

  /// Copy one staged `miso.command.v1` submission into Wasm and admit it (issue #137 D1).
  ///
  /// The records arrive as one flat transferable byte block, so a whole fader gesture costs one
  /// message and one copy rather than one message per parameter. The block is handed straight back
  /// on the acknowledgement, exactly as source planes are, so the caller keeps its storage.
  receiveCommand(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || !u32(message.count) || message.count > MAXIMUM_COMMAND_RECORDS
        || !(message.records instanceof Uint8Array)
        || !(message.records.buffer instanceof ArrayBuffer)
        || (typeof SharedArrayBuffer !== "undefined"
          && message.records.buffer instanceof SharedArrayBuffer)
        || message.records.byteLength !== message.count * COMMAND_RECORD_BYTES) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    if (!this.liveControlsAttached) {
      // A session prepared with `liveControlObservationTaps === 0n` has no control channel and
      // no staging buffer. That is a typed refusal of a well-formed request, not a protocol error,
      // so the batch is acknowledged and its record block goes back to the caller untouched.
      this.port.postMessage({
        tag: "miso.ack.v1",
        requestId: message.requestId,
        result: RESULT_UNSUPPORTED,
        reason: COMMAND_REASON_UNSUPPORTED_KIND,
        rejectedIndex: 0,
        admitted: 0,
        appliedAtSample: 0n,
        records: message.records,
      }, [message.records.buffer]);
      return;
    }
    this.commandStaging.set(message.records, 0);
    const result = this.exports.miso_engine_web_v1_command_submit(this.handle, message.count);
    const report = this.commandReport;
    if (report.getUint32(0, true) !== COMMAND_REPORT_BYTES
        || report.getUint32(4, true) !== ABI_VERSION
        || report.getBigUint64(32, true) !== 0n || report.getBigUint64(40, true) !== 0n) {
      this.sticky(RESULT_INTERNAL, message.requestId);
      return;
    }
    this.port.postMessage({
      tag: "miso.ack.v1",
      requestId: message.requestId,
      result,
      reason: report.getUint32(12, true),
      rejectedIndex: report.getUint32(16, true),
      admitted: report.getUint32(20, true),
      appliedAtSample: report.getBigUint64(24, true),
      records: message.records,
    }, [message.records.buffer]);
  }

  /// Issue #1562a D1: one command frame through the control plane's `SessionState::command`, the
  /// entry the C ABI's `miso_engine_v1_submit_command` runs. It runs here, in the message handler
  /// and never in `process()`: `single` mode's control work, whose allocations the status counts.
  /// The worklet decodes nothing in the frame. The reply carries the exports' result word, the
  /// response frame bytes unchanged (after a failure, the refusal's diagnostic text), and the
  /// committed revision after the call. A frame the staging buffer cannot hold is a typed refusal
  /// of a well-formed request, not a protocol error.
  ///
  /// Issue #1697: an optional `document` (the same checks as `frame`) is the document a
  /// `SESSION_REPLACE` frame names; it is staged beside the frame, here and never in `process()`.
  receiveControl(message) {
    const wholeBytes = (bytes) => bytes instanceof Uint8Array
      && bytes.buffer instanceof ArrayBuffer
      && !(typeof SharedArrayBuffer !== "undefined" && bytes.buffer instanceof SharedArrayBuffer)
      && bytes.byteLength !== 0 && bytes.byteOffset === 0
      && bytes.byteLength === bytes.buffer.byteLength;
    const frame = message.frame;
    const document = message.document;
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || !wholeBytes(frame) || ("document" in message && !wholeBytes(document))) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const reply = this.readControlCommand(frame, document);
    this.port.postMessage({
      tag: "miso.control.v1",
      requestId: message.requestId,
      result: reply.result,
      response: reply.response,
      revision: reply.revision,
    }, [reply.response.buffer]);
    // The call may have grown the module's memory (a control allocation, counted on the control
    // side and no fault): the reply above was read from the new buffer and is whole, and the views
    // the render path holds are re-cut now, in this handler, before the next quantum renders.
    this.rebindMemoryViews();
  }

  /// Stage `frame`, run it and copy the response out of the module's memory. The module's memory
  /// may have grown under the call: the reply is then read from the fresh buffer, and the caller's
  /// handler re-binds the views (no fault). A frame the staging cannot hold (a zero
  /// pointer) has no response: the previous command's bytes are not its reply.
  ///
  /// Issue #1697: a `document` is staged through `replace_document_ptr` after the frame's staging
  /// is known to hold the frame and before the frame is written, so a refused frame never leaves a
  /// document staged for a later command; a document the staging refuses (a zero pointer, for
  /// example a length over the boot bound) is the same typed refusal with no response, and runs
  /// no command. The command frees the staged document, whatever its result. The staging may grow
  /// the module's memory, so every write cuts its view from the live buffer.
  readControlCommand(frame, document) {
    const length = frame.byteLength;
    let pointer = this.exports.miso_engine_web_v1_control_request_ptr(this.handle, length);
    if (pointer !== 0 && document !== undefined) {
      const documentPointer = this.exports.miso_engine_web_v1_replace_document_ptr(
        this.handle,
        document.byteLength,
      );
      if (documentPointer === 0) {
        pointer = 0;
      } else {
        new Uint8Array(this.exports.memory.buffer, documentPointer, document.byteLength)
          .set(document);
      }
    }
    let result = RESULT_INVALID_ARGUMENT;
    if (pointer !== 0) {
      new Uint8Array(this.exports.memory.buffer, pointer, length).set(frame);
      result = this.exports.miso_engine_web_v1_control_command(this.handle, length);
    }
    const buffer = this.exports.memory.buffer;
    // A growth under the call is no fault: the reply is read from the live buffer, and every
    // view is re-cut by the caller's handler before it ends.
    let response = new Uint8Array(0);
    if (pointer !== 0) {
      const responsePointer = this.exports.miso_engine_web_v1_control_response_ptr(this.handle);
      const responseBytes = this.exports.miso_engine_web_v1_control_response_bytes(this.handle);
      if (responsePointer !== 0) {
        response = new Uint8Array(responseBytes);
        response.set(new Uint8Array(buffer, responsePointer, responseBytes));
      }
    }
    const revision = this.exports.miso_engine_web_v1_control_revision(this.handle);
    return { result, response, revision };
  }

  /// Take or release the decimated meter lease (issue #137 D2).
  receiveMeterLease(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.enabled !== "boolean") {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const result = this.exports.miso_engine_web_v1_meter_lease(
      this.handle,
      message.enabled ? 1 : 0,
    );
    if (result === RESULT_OK) {
      this.meterLease = message.enabled;
      this.meterSequence = 0;
    }
    this.acknowledge(message.requestId, result);
  }

  /// Take or release the render-telemetry lease (issue #137 D3).
  ///
  /// JavaScript only: Wasm never learns that the lease exists, and with the lease released the
  /// render callback makes no timing call at all.
  receiveTelemetryLease(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.enabled !== "boolean") {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    this.telemetryLease = message.enabled;
    this.telemetryBlocks = 0;
    this.telemetryElapsedMs = 0;
    this.telemetryBudgetMs = 0;
    this.telemetryPeakMs = 0;
    this.telemetryDeadlineMisses = 0;
    this.telemetryMessage.sequence = 0;
    this.acknowledge(message.requestId, RESULT_OK);
  }

  receiveSeek(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.sourceId !== "string" || typeof message.generation !== "bigint"
        || message.generation <= 0n || typeof message.sourceFrame !== "bigint"
        || message.sourceFrame < 0n) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const idLength = writeBoundedUtf8(
      message.sourceId,
      this.memoryBuffer,
      this.sourceIdPointer,
      this.sourceIdCapacity,
    );
    if (idLength < 0) {
      this.sticky(4, message.requestId);
      return;
    }
    this.acknowledge(
      message.requestId,
      this.exports.miso_engine_web_v1_source_seek(
        this.handle, idLength, message.generation, message.sourceFrame,
      ),
    );
  }

  /// Issue #1644 D1, `single` mode: seek the timeline and every source under one generation, all
  /// or nothing, in the message handler and never in `process()`. A plain seek (`anchorSample`
  /// `-1n`) calls `session_seek`, which declares the discontinuity in the same call; an anchored
  /// one calls `session_seek_at` once. A refusal is the reply's result and is not sticky; an OK
  /// records the generation for the feed drain (D2) before the reply.
  receiveSessionSeek(message) {
    if (typeof message.generation !== "bigint" || message.generation <= 0n
        || message.generation > 0xffffffffffffffffn
        || typeof message.sourceFrame !== "bigint" || message.sourceFrame < 0n
        || message.sourceFrame > 0xffffffffffffffffn
        || typeof message.anchorSample !== "bigint" || message.anchorSample < -1n
        || message.anchorSample > 0xffffffffffffffffn) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const result = message.anchorSample === -1n
      ? this.exports.miso_engine_web_v1_session_seek(
        this.handle, message.generation, message.sourceFrame,
      )
      : this.exports.miso_engine_web_v1_session_seek_at(
        this.handle, message.generation, message.sourceFrame, message.anchorSample,
      );
    if (result === RESULT_OK) this.lastSessionSeekGeneration = message.generation;
    this.port.postMessage({
      tag: "miso.session-seek.v1",
      requestId: message.requestId,
      result,
      generation: message.generation,
    });
  }

  /// Issue #1297a, `single` mode: one anchored source seek, applied by the message handler between
  /// blocks and never in `process()`: `source_seek_at` runs the service step. It calls the export
  /// once; a refusal is the reply's result and is not sticky. A malformed message calls nothing.
  receiveSeekAt(message) {
    if (typeof message.sourceId !== "string"
        || typeof message.generation !== "bigint" || message.generation <= 0n
        || message.generation > 0xffffffffffffffffn
        || typeof message.sourceFrame !== "bigint" || message.sourceFrame < 0n
        || message.sourceFrame > 0xffffffffffffffffn
        || typeof message.anchorSample !== "bigint" || message.anchorSample < 0n
        || message.anchorSample > 0xffffffffffffffffn) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId);
      return;
    }
    const idLength = this.stageSourceId(message.sourceId);
    if (idLength < 0) {
      this.sticky(4, message.requestId);
      return;
    }
    this.acknowledge(
      message.requestId,
      this.exports.miso_engine_web_v1_source_seek_at(
        this.handle, idLength, message.generation, message.sourceFrame, message.anchorSample,
      ),
    );
  }

  /// Issue #1297a, `single` mode: read where one source landed after its newest seek. The handler
  /// calls `source_seek_report` once (it runs the service step, so never in `process()`) and copies
  /// the record's words into the reply; a refusal is the reply's result with a null report, and is
  /// not sticky. The worklet computes nothing from the words.
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
      this.rebindMemoryViews();
      const view = this.seekReportView;
      if (view.getUint32(0, true) !== SEEK_REPORT_BYTES) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      report = {};
      for (const [field, offset] of SEEK_REPORT_WORDS) {
        report[field] = view.getBigUint64(offset, true);
      }
    }
    this.port.postMessage({
      tag: "miso.seek-report.v1",
      requestId: message.requestId,
      result,
      report,
    });
  }

  /// Copy one source ID into the staging; the byte count, or `-1` when it does not fit.
  stageSourceId(sourceId) {
    return writeBoundedUtf8(sourceId, this.memoryBuffer, this.sourceIdPointer, this.sourceIdCapacity);
  }

  // PROCESS_POLICY_BEGIN
  silence(outputs) {
    for (let outputIndex = 0; outputIndex < outputs.length; outputIndex += 1) {
      const output = outputs[outputIndex];
      for (let planeIndex = 0; planeIndex < output.length; planeIndex += 1) {
        output[planeIndex].fill(0);
      }
    }
  }

  /// Fold one finished meter window and post it (issue #137 D2).
  ///
  /// Allocation-free on this side: `meter_poll` moves `Copy` snapshots into a Wasm buffer sized at
  /// compilation, `set` copies them into a `Float32Array` allocated at construction, and the frozen
  /// message body is mutated in place. The structured clone `postMessage` performs is the only
  /// allocation, and it is the one this ABI exists to pay: one flat numeric payload per window
  /// instead of one per block.
  postMeterFrame() {
    let set = this.meterSet;
    // A host booted without meters has no frame; the lease is then a no-op the host refuses.
    if (set === null) return;
    // Issue #1290d1: a block that adopted a replacement plan published its frame and header in
    // the plan's own, and marked the header it left. Switch to the views the handler that
    // committed the plan cut over them: a reference assignment, no view is made here. A header
    // that says it was replaced, without the set cut for the header it names, is a fault.
    if (set.header.getUint32(METER_HEADER_RETIRED, true) !== 0) {
      const next = this.pendingMeterSet;
      if (next === null
          || next.headerPointer !== set.header.getUint32(METER_HEADER_SUCCESSOR_POINTER, true)) {
        this.stickyResult = RESULT_INTERNAL;
        this.ready = false;
        return;
      }
      this.meterSet = next;
      this.pendingMeterSet = null;
      set = next;
    }
    const windows = this.exports.miso_engine_web_v1_meter_poll(this.handle);
    if (windows === 0) return;
    const header = set.header;
    const metadata = header.getBigUint64(56, true);
    // The Rust adapter only publishes complete track/master spans. Keep this guard at the
    // transport boundary so a future invalid publication cannot reach legacy numeric consumers.
    if ((metadata & 1n) === 0n || (metadata & 2n) === 0n) return;
    this.meterSequence += 1;
    const message = set.message;
    message.sequence = this.meterSequence;
    message.generation = header.getBigUint64(48, true);
    message.validity = Number(metadata & 0xffffffffn);
    message.lossCount = Number(metadata >> 32n);
    message.windows = windows;
    // Two 32-bit words, so the read allocates nothing.
    message.structureEpoch = header.getUint32(METER_HEADER_STRUCTURE_EPOCH, true)
      + header.getUint32(METER_HEADER_STRUCTURE_EPOCH + 4, true) * 4294967296;
    message.peaks.set(set.trackPeaks);
    message.peaks.set(set.masterPeaks, set.trackCount * 2);
    message.submixPeaks.set(set.submixPeaks);
    // Issue #143 D5: the gain-reduction section rides the same post. There is no second message
    // and no second clock -- the pinned-occurrence rule does not move.
    message.trackGrDb.set(set.trackGain);
    message.submixGrDb.set(set.submixGain);
    message.masterGrDb = header.getUint32(44, true) === 1
      ? set.frame[set.masterGainIndex]
      : null;
    message.firstSample = header.getBigUint64(16, true);
    message.endSample = header.getBigUint64(24, true);
    this.port.postMessage(set.message);
  }

  /// Fold one render-time window and post it (issue #137 D3).
  recordRenderTime(elapsedMs, frames) {
    const budgetMs = (frames / this.sampleRateHz) * 1000;
    this.telemetryElapsedMs += elapsedMs;
    this.telemetryBudgetMs += budgetMs;
    if (elapsedMs > this.telemetryPeakMs) this.telemetryPeakMs = elapsedMs;
    if (elapsedMs > budgetMs) this.telemetryDeadlineMisses += 1;
    this.telemetryBlocks += 1;
    if (this.telemetryBlocks < TELEMETRY_WINDOW_BLOCKS) return;
    const frame = this.telemetryMessage;
    frame.sequence += 1;
    frame.cpuPercent = (this.telemetryElapsedMs / this.telemetryBudgetMs) * 100;
    frame.peakBlockMs = this.telemetryPeakMs;
    frame.meanBlockMs = this.telemetryElapsedMs / this.telemetryBlocks;
    frame.budgetMs = this.telemetryBudgetMs / this.telemetryBlocks;
    frame.deadlineMisses = this.telemetryDeadlineMisses;
    // A window that measured exactly zero did not prove the render is free; it proved the clock
    // could not see it. Saying so is the whole point of shipping the resolution alongside.
    frame.belowResolution = this.telemetryElapsedMs === 0;
    this.port.postMessage(frame);
    this.telemetryBlocks = 0;
    this.telemetryElapsedMs = 0;
    this.telemetryBudgetMs = 0;
    this.telemetryPeakMs = 0;
    this.telemetryDeadlineMisses = 0;
  }

  process(_inputs, outputs) {
    if (this.disposed) {
      this.silence(outputs);
      return false;
    }
    const output = outputs[0];
    if (!this.ready || this.stickyResult !== RESULT_OK) {
      this.silence(outputs);
      return true;
    }
    // Memory moved and no handler has re-bound yet: the cached views are detached. Render nothing
    // for this quantum, count it, and wait for the handler to re-bind. This never re-binds here.
    if (this.exports.memory.buffer !== this.memoryBuffer) {
      this.staleViewQuanta += 1;
      this.silence(outputs);
      return true;
    }
    const actualFrames = output !== undefined && output.length === 2
        && output[0].length === this.quantumFrames && output[1].length === this.quantumFrames
      ? this.quantumFrames
      : 0;
    // `wasm32-unknown-unknown` is `panic = abort`: a Rust panic traps the instance and surfaces
    // here as a throw. There is no `catch_unwind` inside Rust to convert it, so the containment is
    // this: sticky RESULT_INTERNAL and positive-zero output, never a torn or stale block. The user
    // agent may also fire `processorerror`; both paths end with the node silent and disposable.
    // Issue #137 D3: no clock is read while the lease is released.
    const started = this.telemetryLease ? this.clock.read() : 0;
    let result;
    try {
      result = this.exports.miso_engine_web_v1_render(this.handle, actualFrames);
    } catch (_) {
      result = RESULT_INTERNAL;
    }
    if (result !== RESULT_OK) {
      this.stickyResult = result;
      this.ready = false;
      this.silence(outputs);
      return true;
    }
    if (actualFrames === 0) {
      this.stickyResult = RESULT_REPREPARE_REQUIRED;
      this.ready = false;
      this.silence(outputs);
      return true;
    }
    output[0].set(this.outputLeft);
    output[1].set(this.outputRight);
    if (this.meterLease) this.postMeterFrame();
    if (this.telemetryLease) this.recordRenderTime(this.clock.read() - started, actualFrames);
    return true;
  }
  // PROCESS_POLICY_END
}

registerProcessor(PROCESSOR_NAME, MisoEngineAudioWorkletProcessor);
