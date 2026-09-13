const ABI_VERSION = 0x00010000;
const BOOT_OPTIONS_BYTES = 64;
// Issue #143 D5: the fixed structure carrying the sample window an `f32` frame cannot hold.
const METER_HEADER_BYTES = 64;
const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_UNSUPPORTED = 7;
const RESULT_REPREPARE_REQUIRED = 9;
const COMMAND_REASON_UNSUPPORTED_KIND = 7;
const RESULT_INTERNAL = 255;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_OUTPUT_PCM = 5;
const BUFFER_COMMAND = 6;
const BUFFER_METER_FRAME = 7;
const STATE_READY = 2;
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";

// Issue #137 D1/D2/D3.
const COMMAND_RECORD_BYTES = 48;
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

const INIT_FIELDS = ["module", "document", "options"];
const OPTION_FIELDS = [
  "sourceRingFrames", "maximumMemoryBytes", "consoleCommandQueueRecords", "consoleMeterBlocks",
  "consoleObservationTaps", "consoleMasterTrackPlusOne",
];
const SOURCE_FIELDS = [
  "tag", "requestId", "sourceId", "generation", "startFrame", "sampleRateHz", "planes", "frames",
  "endOfRegion",
];
const SEEK_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame"];
const COMMAND_FIELDS = ["tag", "requestId", "count", "records"];
const LEASE_FIELDS = ["tag", "requestId", "enabled"];

function validObservationAddress(address) {
  return exactFields(address, OBSERVATION_ADDRESS_FIELDS)
    && u32(address.trackIndex)
    && u32(address.rack) && address.rack <= 2
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
    // Issue #137: both leases start released, so a session that never asks for a console pays a
    // single `false` test per block and nothing else.
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
    this.clock = renderClock();
    this.port.onmessage = (event) => this.receive(event.data);
    try {
      const result = this.initialize(options?.processorOptions);
      if (result !== RESULT_OK) this.failInitialization(result, 0);
    } catch (_) {
      this.failInitialization(RESULT_INTERNAL, 0);
    }
  }

  initialize(init) {
    if (!exactFields(init, INIT_FIELDS) || !(init.document instanceof Uint8Array)
        || !exactFields(init.options, OPTION_FIELDS)) {
      return RESULT_INVALID_ARGUMENT;
    }
    this.instance = new WebAssembly.Instance(init.module, {});
    this.exports = this.instance.exports;
    if (this.exports.miso_engine_web_v1_abi_version() !== ABI_VERSION) {
      return 2;
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
    const documentPointer = this.exports.miso_engine_web_v1_document_ptr(init.document.byteLength);
    if (!u32(documentPointer) || documentPointer === 0) {
      return this.exports.miso_engine_web_v1_boot_result();
    }
    new Uint8Array(this.exports.memory.buffer, documentPointer, init.document.byteLength)
      .set(init.document);
    this.handle = this.exports.miso_engine_web_v1_boot(init.document.byteLength);
    if (this.handle === 0) {
      return this.exports.miso_engine_web_v1_boot_result();
    }
    this.backend = "simd128";
    this.memoryBuffer = this.exports.memory.buffer;
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
    this.statusView = new DataView(this.memoryBuffer, statusPointer, 80);
    this.resourceView = new DataView(this.memoryBuffer, resourcePointer, 224);
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
    this.sourcePcm = new Float32Array(
      this.memoryBuffer,
      this.sourcePcmPointer,
      sourcePcmCapacity / 4,
    );
    this.outputLeft = new Float32Array(this.memoryBuffer, outputPointer, this.quantumFrames);
    this.outputRight = new Float32Array(
      this.memoryBuffer,
      outputPointer + this.quantumFrames * 4,
      this.quantumFrames,
    );
    this.resources = this.readResources();
    const expectedBackend = 1;
    if (this.resources.backend !== expectedBackend || status.backend !== expectedBackend
        || this.resources.sampleRateHz !== this.sampleRateHz
        || this.resources.quantumFrames !== this.quantumFrames
        || status.state !== STATE_READY || status.lastResult !== RESULT_OK
        || status.nextAbsoluteSample !== 0n || status.renderedQuanta !== 0n) {
      return RESULT_INVALID_ARGUMENT;
    }
    if (!this.bindConsole(init)) return RESULT_INTERNAL;
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

  /// Bind every live-console view and preallocate every frame the render callback posts.
  ///
  /// Issue #137: nothing in `process()` may allocate, so every message body, every typed-array
  /// copy and every track identity is built here, once, on the construction path that is already
  /// allowed to allocate. `process()` mutates the frozen shapes and posts them; the structured
  /// clone `postMessage` performs is the only allocation left, and it is the one the ABI cannot
  /// avoid.
  bindConsole(init) {
    this.consoleAttached = init.options.consoleCommandQueueRecords !== 0n;
    const commandPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_COMMAND);
    const commandCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_COMMAND,
    );
    const reportPointer = this.exports.miso_engine_web_v1_command_report_ptr(this.handle);
    if (!u32(reportPointer) || reportPointer === 0) return false;
    this.commandReport = new DataView(this.memoryBuffer, reportPointer, COMMAND_REPORT_BYTES);
    if (this.consoleAttached) {
      if (!u32(commandPointer) || commandPointer === 0
          || commandCapacity !== MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES) return false;
      this.commandStaging = new Uint8Array(this.memoryBuffer, commandPointer, commandCapacity);
    } else if (commandPointer !== 0 || commandCapacity !== 0) {
      // A released console must own no staging at all; a nonzero row here would mean the engine
      // charged for a buffer the ABI says does not exist.
      return false;
    }

    this.trackCount = this.exports.miso_engine_web_v1_console_track_count(this.handle);
    if (!u32(this.trackCount)) return false;
    // `TextDecoder` is no more guaranteed in a `WorkletGlobalScope` than `TextEncoder` is, and
    // a session track ID is `[a-z][a-z0-9._-]{0,126}` by the session schema, so every byte is
    // ASCII by construction. A byte that is not is a corrupt artifact, not a decoding problem.
    this.trackIds = [];
    for (let index = 0; index < this.trackCount; index += 1) {
      const length = this.exports.miso_engine_web_v1_console_track_id(this.handle, index);
      if (!u32(length) || length === 0 || length > this.sourceIdCapacity) return false;
      const bytes = new Uint8Array(this.memoryBuffer, this.sourceIdPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return false;
        id += String.fromCharCode(bytes[byte]);
      }
      this.trackIds.push(id);
    }

    // Issue #207: source introspection, read once here for the same reason the track identities
    // are -- the construction path is the one that may allocate, and `process()` never touches
    // this. A headless consumer of the session map cannot otherwise learn which sources a compiled
    // session declares, how many channels each carries, or which frames it is waiting for; the
    // ABI exposed track discovery and nothing at all about sources.
    //
    // Every read is checked against something the engine already guarantees, so a mis-wired export
    // fails initialization here rather than surfacing as a plausible-looking wrong number:
    // compilation refuses a zero channel count or full-source frame count, and preparation refuses
    // a channel count above `maximumSourceChannels`.
    this.sourceCount = this.exports.miso_engine_web_v1_source_count(this.handle);
    if (!u32(this.sourceCount)) return false;
    this.sources = [];
    for (let index = 0; index < this.sourceCount; index += 1) {
      const length = this.exports.miso_engine_web_v1_source_id(this.handle, index);
      if (!u32(length) || length === 0 || length > this.sourceIdCapacity) return false;
      const bytes = new Uint8Array(this.memoryBuffer, this.sourceIdPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return false;
        id += String.fromCharCode(bytes[byte]);
      }
      const channels = this.exports.miso_engine_web_v1_source_channels(this.handle, index);
      const frames = this.exports.miso_engine_web_v1_source_frames(this.handle, index);
      if (!u32(channels) || channels === 0 || channels > this.maximumSourceChannels
          || !u64(frames) || frames === 0n) return false;
      this.sources.push({ id, channels, frames });
    }

    // Observation bindings and the fixed selection staging view are control-path state. They are
    // read by request handlers only; the render callback never follows an observation pointer or
    // allocates a reply row. The map is rebuilt at each boot so a reboot cannot retain an old
    // numeric owner address behind a stable SDK selection.
    const observationCount = this.exports.miso_engine_web_v1_observation_count(this.handle);
    if (!u32(observationCount) || observationCount > MAXIMUM_OBSERVATION_READS) return false;
    const observationIdPointer = this.exports.miso_engine_web_v1_observation_id_ptr();
    const observationIdCapacity = this.exports.miso_engine_web_v1_observation_id_capacity();
    if (!u32(observationIdPointer) || observationIdPointer === 0
        || !u32(observationIdCapacity) || observationIdCapacity === 0) return false;
    const readObservationId = (length) => {
      if (!u32(length) || length === 0 || length > observationIdCapacity) return null;
      const bytes = new Uint8Array(this.memoryBuffer, observationIdPointer, length);
      let id = "";
      for (let byte = 0; byte < length; byte += 1) {
        if (bytes[byte] > 0x7f) return null;
        id += String.fromCharCode(bytes[byte]);
      }
      return id;
    };
    const bindings = [];
    for (let index = 0; index < observationCount; index += 1) {
      const trackIndex = this.exports.miso_engine_web_v1_observation_track_index(this.handle, index);
      const rack = this.exports.miso_engine_web_v1_observation_rack(this.handle, index);
      const effectIndex = this.exports.miso_engine_web_v1_observation_effect_index(this.handle, index);
      const effectSlotId = readObservationId(
        this.exports.miso_engine_web_v1_observation_effect_slot_id(this.handle, index),
      );
      const nativeEffectId = readObservationId(
        this.exports.miso_engine_web_v1_observation_native_effect_id(this.handle, index),
      );
      const tapCount = this.exports.miso_engine_web_v1_observation_tap_count(this.handle, index);
      if (!u32(trackIndex) || !u32(rack) || rack > 2 || !u32(effectIndex)
          || effectSlotId === null || nativeEffectId === null || !u32(tapCount)
          || tapCount === 0 || tapCount > 32) return false;
      const tapIds = [];
      for (let tapIndex = 0; tapIndex < tapCount; tapIndex += 1) {
        const tapId = this.exports.miso_engine_web_v1_observation_tap_id(
          this.handle, index, tapIndex,
        );
        if (!u32(tapId) || tapId === 0) return false;
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
    this.observations = Object.freeze(bindings);
    const observationSelectionPointer = this.exports.miso_engine_web_v1_observation_selection_ptr();
    const observationSelectionBytes = this.exports.miso_engine_web_v1_observation_selection_bytes();
    const observationSelectionCapacity = this.exports.miso_engine_web_v1_observation_selection_capacity();
    if (!u32(observationSelectionPointer) || observationSelectionPointer === 0
        || observationSelectionBytes !== OBSERVATION_SELECTION_BYTES
        || !u32(observationSelectionCapacity)
        || observationSelectionCapacity < 1
        || observationSelectionCapacity > MAXIMUM_OBSERVATION_READS) return false;
    this.observationSelectionView = new DataView(
      this.memoryBuffer,
      observationSelectionPointer,
      observationSelectionCapacity * observationSelectionBytes,
    );

    const framePointer = this.exports.miso_engine_web_v1_buffer_ptr(
      this.handle,
      BUFFER_METER_FRAME,
    );
    const frameCapacity = this.exports.miso_engine_web_v1_buffer_capacity(
      this.handle,
      BUFFER_METER_FRAME,
    );
    this.metersAttached = init.options.consoleMeterBlocks !== 0n;
    this.observationAttached = init.options.consoleObservationTaps !== 0n;
    if (this.metersAttached) {
      // Issue #143 D5: the frame is `3T + 3` words -- the peak section exactly where it was, then
      // one non-negative gain-reduction magnitude per track and the master's.
      if (!u32(framePointer) || framePointer === 0
          || frameCapacity !== (this.trackCount * 3 + 3) * 4) return false;
      const headerPointer = this.exports.miso_engine_web_v1_meter_header_ptr(this.handle);
      if (!u32(headerPointer) || headerPointer === 0) return false;
      this.meterView = new Float32Array(this.memoryBuffer, framePointer, frameCapacity / 4);
      // Two fixed views over the one buffer, built here and never again: the frozen render-callback
      // policy forbids `subarray` inside it, and rightly -- a per-block view is a per-block
      // allocation. The peak view is byte-for-byte the `2T + 2` region it always was.
      this.meterPeakView = new Float32Array(
        this.memoryBuffer,
        framePointer,
        this.trackCount * 2 + 2,
      );
      this.meterGainView = new Float32Array(
        this.memoryBuffer,
        framePointer + (this.trackCount * 2 + 2) * 4,
        this.trackCount,
      );
      this.meterMasterGainIndex = this.trackCount * 3 + 2;
      this.meterHeaderView = new DataView(this.memoryBuffer, headerPointer, METER_HEADER_BYTES);
      if (this.meterHeaderView.getUint32(0, true) !== METER_HEADER_BYTES
          || this.meterHeaderView.getUint32(4, true) !== ABI_VERSION
          || this.meterHeaderView.getUint32(8, true) !== this.trackCount) {
        return false;
      }
      this.meterMessage = {
        tag: "miso.meter.v1",
        sequence: 0,
        generation: 0n,
        validity: 0,
        lossCount: 0,
        windows: 0,
        trackCount: this.trackCount,
        // The frozen `2T + 2` peak view, unmoved: an existing reader indexes it exactly as before.
        peaks: new Float32Array(this.trackCount * 2 + 2),
        // Issue #143: one non-negative decibel magnitude per track. Positional and always finite;
        // `0` deliberately conflates "not reducing" with "no observed effect", because the array
        // is read without null checks and the distinction lives in the subscription map.
        trackGrDb: new Float32Array(this.trackCount),
        masterGrDb: null,
        firstSample: 0n,
        endSample: 0n,
      };
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
    const consoleWords = [
      options.consoleCommandQueueRecords, options.consoleMeterBlocks,
      options.consoleObservationTaps, options.consoleMasterTrackPlusOne,
    ];
    if (consoleWords.some((value) => !validU64(value))) {
      throw new RangeError("Invalid console boot option");
    }
    // A subscription rides the effect's own command queue, so capacity without one has no delivery
    // path; a master designation with no capacity would report a number nothing produces.
    if ((options.consoleObservationTaps !== 0n && options.consoleCommandQueueRecords === 0n)
        || (options.consoleMasterTrackPlusOne !== 0n
          && options.consoleObservationTaps === 0n)) {
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
    consoleWords.forEach((value, index) => view.setBigUint64(32 + index * 8, value, true));
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
      // with `consoleObservationTaps === 0n`.
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
    if (view.getUint32(0, true) !== 80 || view.getUint32(4, true) !== ABI_VERSION
        || view.getUint32(28, true) !== 0
        || [48, 56, 64, 72].some((offset) => view.getBigUint64(offset, true) !== 0n)) {
      throw new RangeError("Invalid status report");
    }
    return Object.freeze({
      state: view.getUint32(8, true),
      lastResult: this.stickyResult || view.getUint32(12, true),
      backend: view.getUint32(16, true),
      sampleRateHz: view.getUint32(20, true),
      quantumFrames: view.getUint32(24, true),
      nextAbsoluteSample: view.getBigUint64(32, true),
      renderedQuanta: view.getBigUint64(40, true),
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

  receive(message) {
    if (this.disposed) return;
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
      }
      this.acknowledge(message.requestId, result);
      return;
    }
    if (this.stickyResult !== RESULT_OK) {
      this.sticky(this.stickyResult, message.requestId, returnedPlanes);
      return;
    }
    if (this.exports.memory.buffer !== this.memoryBuffer) {
      this.sticky(RESULT_REPREPARE_REQUIRED, message.requestId, returnedPlanes);
      return;
    }
    if (message?.tag === "miso.source.v1" && exactFields(message, SOURCE_FIELDS)) {
      this.receiveSource(message);
    } else if (message?.tag === "miso.seek.v1" && exactFields(message, SEEK_FIELDS)) {
      this.receiveSeek(message);
    } else if (message?.tag === "miso.command.v1" && exactFields(message, COMMAND_FIELDS)) {
      this.receiveCommand(message);
    } else if (message?.tag === "miso.meters.v1" && exactFields(message, LEASE_FIELDS)) {
      this.receiveMeterLease(message);
    } else if (message?.tag === "miso.telemetry.v1" && exactFields(message, LEASE_FIELDS)) {
      this.receiveTelemetryLease(message);
    } else if (message?.tag === "miso.sessionmap.v1"
        && exactFields(message, ["tag", "requestId"])) {
      this.port.postMessage({
        tag: "miso.sessionmap.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        tracks: [...this.trackIds],
        sources: this.sources.map((source) => ({
          id: source.id,
          channels: source.channels,
          frames: source.frames,
        })),
        metersAttached: this.metersAttached === true,
      });
    } else if (message?.tag === "miso.observationmap.v1"
        && exactFields(message, ["tag", "requestId"])) {
      this.port.postMessage({
        tag: "miso.observationmap.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        bindings: this.observations,
      });
    } else if (message?.tag === "miso.observation.v1"
        && exactFields(message, ["tag", "requestId", "selections"])) {
      this.receiveObservationRead(message);
    } else if (message?.tag === "miso.status.v1"
        && exactFields(message, ["tag", "requestId"])) {
      this.port.postMessage({
        tag: "miso.status.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        ...this.readStatus(),
        memoryBytes: this.exports.memory.buffer.byteLength,
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
      if (!validObservationRow(row)) {
        this.sticky(RESULT_INTERNAL, message.requestId);
        return;
      }
      rows.push(row);
    }
    this.port.postMessage({
      tag: "miso.observation.v1",
      requestId: message.requestId,
      result: RESULT_OK,
      rows,
    });
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
    if (!this.consoleAttached) {
      // A session prepared with `consoleCommandQueueRecords === 0n` has no control channel and no
      // staging buffer. That is a typed refusal of a well-formed request, not a protocol error, so
      // the batch is acknowledged and its record block goes back to the caller untouched.
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
    const windows = this.exports.miso_engine_web_v1_meter_poll(this.handle);
    if (windows === 0) return;
    const metadata = this.meterHeaderView.getBigUint64(56, true);
    // The Rust adapter only publishes complete track/master spans. Keep this guard at the
    // transport boundary so a future invalid publication cannot reach legacy numeric consumers.
    if ((metadata & 1n) === 0n || (metadata & 2n) === 0n) return;
    this.meterSequence += 1;
    this.meterMessage.sequence = this.meterSequence;
    this.meterMessage.generation = this.meterHeaderView.getBigUint64(48, true);
    this.meterMessage.validity = Number(metadata & 0xffffffffn);
    this.meterMessage.lossCount = Number(metadata >> 32n);
    this.meterMessage.windows = windows;
    this.meterMessage.peaks.set(this.meterPeakView);
    // Issue #143 D5: the gain-reduction section rides the same post. There is no second message
    // and no second clock -- the pinned-occurrence rule does not move.
    this.meterMessage.trackGrDb.set(this.meterGainView);
    this.meterMessage.masterGrDb = this.meterHeaderView.getUint32(44, true) === 1
      ? this.meterView[this.meterMasterGainIndex]
      : null;
    this.meterMessage.firstSample = this.meterHeaderView.getBigUint64(16, true);
    this.meterMessage.endSample = this.meterHeaderView.getBigUint64(24, true);
    this.port.postMessage(this.meterMessage);
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
    if (this.exports.memory.buffer !== this.memoryBuffer) {
      this.stickyResult = RESULT_REPREPARE_REQUIRED;
      this.ready = false;
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
