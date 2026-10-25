const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_BACKPRESSURE = 6;
const RESULT_UNSUPPORTED = 7;
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";
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

const HOST_ARRAY = Array;
const HOST_ARRAY_PROTOTYPE = Array.prototype;
const HOST_ARRAY_IS_ARRAY = Array.isArray;
const HOST_OBJECT_PROTOTYPE = Object.prototype;
const HOST_OBJECT_GET_PROTOTYPE_OF = Object.getPrototypeOf;
const HOST_REFLECT_OWN_KEYS = Reflect.ownKeys;
const HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR = Object.getOwnPropertyDescriptor;
const HOST_OBJECT_DEFINE_PROPERTY = Object.defineProperty;
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
  "liveControlObservationTaps", "reserved2", "spectrumHopFrames", "spectrum",
  "spectrumCollection",
];
const NEW_NO_SPECTRUM_BOOT_OPTION_FIELDS = BOOT_OPTION_FIELDS.slice(0, -2);
const SPECTRUM_BOOT_OPTION_FIELDS = BOOT_OPTION_FIELDS.filter((field) => field !== "spectrumHopFrames");
const LEGACY_BOOT_OPTION_FIELDS = SPECTRUM_BOOT_OPTION_FIELDS.slice(0, -2);
const SPECTRUM_HOP_FRAMES = new Set([0, 256, 512, 1024, 2048]);

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
const SEEK_REPORT_FIELDS = [
  "generation", "firstSample", "sourceFrame", "cumulativeUnderrunFrames", "cumulativeHeldFrames",
];
// Issue #137 D1, #1677: the frozen 272-byte little-endian command record. It names its tap by
// stable key: a one-byte length and a 128-byte zero-padded field for each of the strip's and the
// effect's ID. The IDs are at most 127 UTF-8 bytes (`session::STABLE_ID_MAXIMUM_BYTES`).
const COMMAND_RECORD_BYTES = 272;
const RECORD_ID_FIELD_BYTES = 128;
const RECORD_ID_MAXIMUM_BYTES = RECORD_ID_FIELD_BYTES - 1;
const MAXIMUM_COMMAND_RECORDS = 256;
// Issue #1562a D1: the control frames in flight on the port at once. The worklet answers them in
// arrival order; a flood is refused here with `RESULT_BACKPRESSURE` before any transfer.
const MAXIMUM_CONTROL_IN_FLIGHT = 64;
// The engine's `maximum_control_frame_bytes` (`CONTROL_FRAME_BYTES` in the Rust host).
const MAXIMUM_CONTROL_FRAME_BYTES = 16384;
// Issue #143: the largest `liveControlObservationTaps` the frozen configuration accepts.
const MAXIMUM_OBSERVATION_TAPS = 16;
// Issue #143: the two observation command kinds, named here so `observe()` never writes a literal.
// Not `MISO_`-prefixed: `scripts/check-env-vocabulary.sh` reserves that prefix for the engine's
// environment vocabulary, and a `MISO_`-prefixed identifier in any tracked file is a failure.
const OBSERVE_SUBSCRIBE_KIND = 7;
const OBSERVE_UNSUBSCRIBE_KIND = 8;
const SUBSCRIPTION_FIELDS = [
  "stripId", "rack", "effectId", "tapId", "windowBlocks", "armed",
];
const OBSERVATION_ADDRESS_FIELDS = ["trackIndex", "rack", "effectIndex", "tapId", "channels"];
const MAXIMUM_OBSERVATION_READS = MAXIMUM_COMMAND_RECORDS;
const OBSERVATION_CHANNELS = new Set([1, 2, 3]);
const OBSERVATION_STATUSES = new Set([1, 2, 3]);
const OBSERVATION_SELECTION_BYTES = 32;
const OBSERVATION_RESULT_BYTES = 96;
const OBSERVATION_MAP_FIELDS = [
  "trackIndex", "rack", "effectIndex", "effectSlotId", "nativeEffectId", "tapIds",
];
const OBSERVATION_ROW_FIELDS = [
  "trackIndex", "rack", "effectIndex", "tapId", "channels", "status", "sampleRateHz",
  "firstSample", "endSample", "sequence", "blocks", "leftPresent", "rightPresent", "left", "right",
];
// Issue #207: one entry of the session map's source list. The names are the `.d.ts`'s and the
// worklet's, spelled once here so the acknowledgement validator and the type declaration cannot
// drift apart without `scripts/check-session-map-shape.py` seeing it.
const SESSION_SOURCE_FIELDS = ["id", "channels", "frames"];
const TRACK_RESPONSE_REQUEST_FIELDS = [
  "trackId", "grid", "channels", "points", "minimumHz", "maximumHz", "maximumResultBytes",
];
const TRACK_RESPONSE_REPLY_FIELDS = ["tag", "requestId", "result", "snapshot"];
const LIVE_RESPONSE_CAPTURE_GRID_LINEAR = 1;
const LIVE_RESPONSE_CAPTURE_GRID_LOGARITHMIC = 2;
const LIVE_RESPONSE_CAPTURE_CHANNEL_LEFT = 1;
const LIVE_RESPONSE_CAPTURE_CHANNEL_RIGHT = 2;
const LIVE_RESPONSE_CAPTURE_CHANNEL_BOTH = 3;
const LIVE_RESPONSE_MAXIMUM_POINTS = 4096;
const LIVE_RESPONSE_MAXIMUM_ID_BYTES = 127;
const LIVE_RESPONSE_MAXIMUM_BYTES = 1 << 20;
const SPECTRUM_BOOT_FIELDS = ["target", "targetId", "channels", "maximumCaptureBytes"];
const SPECTRUM_COLLECTION_ENTRY_FIELDS = ["target", "targetId", "channels"];
const SPECTRUM_COLLECTION_FIELDS = ["entries", "maximumCaptureBytes"];
const SPECTRUM_TARGETS = new Set(["trackPostInput", "trackPostPan", "output"]);
const SPECTRUM_CHANNELS = new Set(["left", "right", "both"]);
const SPECTRUM_MAXIMUM_ID_BYTES = 127;
const SPECTRUM_MAXIMUM_BYTES = 1 << 20;
const SPECTRUM_STREAM_METADATA_BYTES = 120;
const SPECTRUM_STREAM_STATUSES = new Set([0, 1, 2, 3, 4, 5, 6]);
const SPECTRUM_STREAM_START_REQUEST_FIELDS = ["tag", "requestId", "operation", "smoothingMs"];
const SPECTRUM_STREAM_READ_REQUEST_FIELDS = ["tag", "requestId", "operation", "buffer"];
const SPECTRUM_STREAM_STOP_REQUEST_FIELDS = ["tag", "requestId", "operation"];
const SPECTRUM_SELECT_REQUEST_FIELDS = ["tag", "requestId", "operation", "target", "targetId", "channels"];
const SPECTRUM_STREAM_SELECT_REQUEST_FIELDS = [
  "tag", "requestId", "operation", "target", "targetId", "channels", "smoothingMs",
];
const SPECTRUM_SELECT_REPLY_FIELDS = ["tag", "requestId", "result", "operation"];
const SPECTRUM_STREAM_SELECT_REPLY_FIELDS = ["tag", "requestId", "result", "operation", "metadata"];
const SPECTRUM_STREAM_START_REPLY_FIELDS = ["tag", "requestId", "result", "operation", "metadata"];
const SPECTRUM_STREAM_STOP_REPLY_FIELDS = ["tag", "requestId", "result", "operation", "metadata"];
const SPECTRUM_STREAM_READ_REPLY_FIELDS = [
  "tag", "requestId", "operation", "result", "byteLength", "buffer", "metadata",
];

// Decision 12 (#1096): an effect is addressed as `1` inserts, by its index in the track's inserts,
// or `3` console, by the slot's index in the session's slot order (`pre_insert`, then
// `post_insert`). The retired `simd1` (`0`) and `simd2` (`2`) codes are refused and never
// reallocated.
const RACK_INSERTS = 1;
const RACK_CONSOLE = 3;
const effectRack = (rack) => rack === RACK_INSERTS || rack === RACK_CONSOLE;

/// Order two IDs by their code units, which is their byte order for the ASCII IDs a session holds.
function compareIds(left, right) {
  return left < right ? -1 : left > right ? 1 : 0;
}

/// The UTF-8 bytes of a record's ID field. It runs on the caller's thread, never in `process()`.
function idBytes(id) {
  return new TextEncoder().encode(id);
}

/// A record's ID: a string whose UTF-8 form fits the 127 bytes of its field. Whether it names
/// anything is the engine's to say, with a typed reason.
function validRecordId(id) {
  return typeof id === "string" && idBytes(id).length <= RECORD_ID_MAXIMUM_BYTES;
}

function validSubscription(subscription) {
  return hasExactFields(subscription, SUBSCRIPTION_FIELDS)
    && validRecordId(subscription.stripId)
    && effectRack(subscription.rack)
    && validRecordId(subscription.effectId)
    && validU32(subscription.tapId) && subscription.tapId > 0
    && validU32(subscription.windowBlocks)
    && typeof subscription.armed === "boolean";
}

function validObservationAddress(address) {
  return hasExactFields(address, OBSERVATION_ADDRESS_FIELDS)
    && validU32(address.trackIndex)
    && effectRack(address.rack)
    && validU32(address.effectIndex)
    && validU32(address.tapId) && address.tapId > 0
    && validU32(address.channels) && OBSERVATION_CHANNELS.has(address.channels);
}

function validObservationMapBinding(binding) {
  return hasExactFields(binding, OBSERVATION_MAP_FIELDS)
    && validU32(binding.trackIndex)
    && effectRack(binding.rack)
    && validU32(binding.effectIndex)
    && typeof binding.effectSlotId === "string" && binding.effectSlotId.length > 0
    && typeof binding.nativeEffectId === "string" && binding.nativeEffectId.length > 0
    && Array.isArray(binding.tapIds) && binding.tapIds.length > 0
    && binding.tapIds.every((tapId) => validU32(tapId) && tapId > 0);
}

function validObservationRow(row) {
  return hasExactFields(row, OBSERVATION_ROW_FIELDS)
    && validU32(row.trackIndex)
    && effectRack(row.rack)
    && validU32(row.effectIndex)
    && validU32(row.tapId) && row.tapId > 0
    && validU32(row.channels) && OBSERVATION_CHANNELS.has(row.channels)
    && validU32(row.status) && OBSERVATION_STATUSES.has(row.status)
    && validU32(row.sampleRateHz) && validU64(row.firstSample)
    && validU64(row.endSample) && validU64(row.sequence)
    && validU32(row.blocks) && validU32(row.leftPresent) && row.leftPresent <= 1
    && validU32(row.rightPresent) && row.rightPresent <= 1
    && typeof row.left === "number" && Number.isFinite(row.left)
    && typeof row.right === "number" && Number.isFinite(row.right);
}
// Issue #143 froze reasons 10 and 11; issue #151 found that this file still wrote the bound as
// the literal `<= 9`, so the only two reasons the observation path ever returns read as malformed
// acknowledgements and killed the whole host with a sticky 255.
//
// The vocabulary is therefore a table, spelled once, in the frozen value order. The bound is
// derived from it and never written as a literal again, and
// `scripts/check-command-reason-vocabulary.py` holds this table equal to
// `hosts/host-web/src/lib.rs`'s `COMMAND_REASON_*` constants, the `.d.ts`
// `MisoCommandReason` enum, the metadata generator's `commandReasons` rows and the schema
// gate's own list. Bump a Rust reason without the rest and that gate is red.
//
// Reasons 12-14 belonged to the value kinds #1565A deleted. They are retired and never
// reallocated, so their entries are `null`: an acknowledgement that names one is malformed.
// Issue #1676 added 15, `observationLimit`.
const COMMAND_REASONS = Object.freeze([
  "none",
  "malformed",
  "unknownTrack",
  "unknownRack",
  "unknownEffect",
  "unknownParameter",
  "domain",
  "unsupportedKind",
  "backpressure",
  "wrongState",
  "unknownTap",
  "observationUnbound",
  null,
  null,
  null,
  "observationLimit",
]);

/// `true` for exactly the reasons this ABI version declares.
///
/// A reason outside the table is a genuinely malformed acknowledgement and still fails the host:
/// the fix for #151 widened the vocabulary, it did not stop validating it.
function validCommandReason(reason) {
  return validU32(reason) && reason < COMMAND_REASONS.length && COMMAND_REASONS[reason] !== null;
}

const COMMAND_FIELDS = ["kind", "rack", "stripId", "effectId", "tapId", "windowBlocks"];
// Issue #143 added kinds 7 and 8, the two observation subscribe/unsubscribe records. Issue #1565a
// deleted every value kind (1-6 and 9-17): a value goes through the `control` message, and a record
// of a deleted kind is refused here as an unknown kind is. Kind numbers are never reallocated.
//
// This set is the bound: `validCommand` asks it, never a hand-written range. It is one of
// the spellings `scripts/check-command-kind-vocabulary.py` holds to the Rust `COMMAND_*`
// constants, so a kind that exists on the wire and not here is red before it ships.
const COMMAND_KINDS = new Set([7, 8]);

function validCommand(command) {
  return hasExactFields(command, COMMAND_FIELDS)
    && COMMAND_KINDS.has(command.kind)
    && effectRack(command.rack)
    && validRecordId(command.stripId) && validRecordId(command.effectId)
    && validU32(command.tapId) && validU32(command.windowBlocks);
}

/// Encode one live-control submission into the 272-byte semantic wire record (#1677).
function encodeCommands(commands) {
  const records = new Uint8Array(commands.length * COMMAND_RECORD_BYTES);
  const view = new DataView(records.buffer);
  for (let index = 0; index < commands.length; index += 1) {
    const command = commands[index];
    const offset = index * COMMAND_RECORD_BYTES;
    const stripId = idBytes(command.stripId);
    const effectId = idBytes(command.effectId);
    records[offset] = command.kind;
    records[offset + 1] = command.rack;
    records[offset + 2] = stripId.length;
    records[offset + 3] = effectId.length;
    view.setUint32(offset + 4, command.tapId, true);
    view.setUint32(offset + 8, command.windowBlocks, true);
    records.set(stripId, offset + 16);
    records.set(effectId, offset + 16 + RECORD_ID_FIELD_BYTES);
  }
  return records;
}
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

function canonicalArrayIndex(key, length) {
  if (key === "") return -1;
  const index = Number(key);
  return Number.isInteger(index) && index >= 0 && index <= 0xfffffffe
      && index < length && String(index) === key
    ? index
    : -1;
}

function snapshotSpectrumEntry(value) {
  const captured = capturePlainRecord(value, SPECTRUM_COLLECTION_ENTRY_FIELDS);
  if (!captured || !hasEnumerableShape(captured, SPECTRUM_COLLECTION_ENTRY_FIELDS)) return undefined;
  return {
    target: captured.descriptors.get("target").value,
    targetId: captured.descriptors.get("targetId").value,
    channels: captured.descriptors.get("channels").value,
  };
}

function snapshotSpectrumEntries(value) {
  if (!HOST_ARRAY_IS_ARRAY(value)
      || HOST_OBJECT_GET_PROTOTYPE_OF(value) !== HOST_ARRAY_PROTOTYPE) return undefined;
  const lengthDescriptor = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(value, "length");
  if (lengthDescriptor === undefined
      || !HOST_HAS_OWN.call(lengthDescriptor, "value")
      || !HOST_HAS_OWN.call(lengthDescriptor, "enumerable")) return undefined;
  const length = lengthDescriptor.value;
  if (!Number.isSafeInteger(length) || length < 0 || length > 0xffffffff) return undefined;
  const keys = HOST_REFLECT_OWN_KEYS(value);
  const indices = [];
  let sawLength = false;
  for (let keyIndex = 0; keyIndex < keys.length; keyIndex += 1) {
    const key = keys[keyIndex];
    if (key === "length") {
      sawLength = true;
      continue;
    }
    const index = typeof key === "string" ? canonicalArrayIndex(key, length) : -1;
    if (index < 0) return undefined;
    const descriptor = HOST_OBJECT_GET_OWN_PROPERTY_DESCRIPTOR(value, key);
    if (descriptor === undefined
        || !HOST_HAS_OWN.call(descriptor, "value")
        || !HOST_HAS_OWN.call(descriptor, "enumerable")) return undefined;
    indices.push({ index, value: descriptor.value, enumerable: descriptor.enumerable });
  }
  if (!sawLength) return undefined;
  let copy;
  try {
    copy = new HOST_ARRAY(length);
    for (let index = 0; index < indices.length; index += 1) {
      const source = indices[index];
      const entry = snapshotSpectrumEntry(source.value);
      if (entry === undefined) return undefined;
      HOST_OBJECT_DEFINE_PROPERTY(copy, source.index, {
        value: entry,
        writable: true,
        enumerable: source.enumerable,
        configurable: true,
      });
    }
  } catch (_) {
    return undefined;
  }
  return copy;
}

function snapshotSpectrumBoot(value) {
  if (value === null) return null;
  const captured = capturePlainRecord(value, SPECTRUM_BOOT_FIELDS);
  if (!captured || !hasEnumerableShape(captured, SPECTRUM_BOOT_FIELDS)) return undefined;
  const snapshot = {
    target: captured.descriptors.get("target").value,
    targetId: captured.descriptors.get("targetId").value,
    channels: captured.descriptors.get("channels").value,
    maximumCaptureBytes: captured.descriptors.get("maximumCaptureBytes").value,
  };
  return validSpectrumBoot(snapshot) ? snapshot : undefined;
}

function snapshotSpectrumCollection(value) {
  if (value === null) return null;
  const captured = capturePlainRecord(value, SPECTRUM_COLLECTION_FIELDS);
  if (!captured || !hasEnumerableShape(captured, SPECTRUM_COLLECTION_FIELDS)) return undefined;
  const entries = snapshotSpectrumEntries(captured.descriptors.get("entries").value);
  if (entries === undefined) return undefined;
  const snapshot = {
    entries,
    maximumCaptureBytes: captured.descriptors.get("maximumCaptureBytes").value,
  };
  return validSpectrumCollectionBoot(snapshot) ? snapshot : undefined;
}

function snapshotBootOptions(value) {
  const captured = capturePlainRecord(value, BOOT_OPTION_FIELDS);
  if (!captured || !hasRequiredEnumerableFields(captured, LEGACY_BOOT_OPTION_FIELDS)) return undefined;
  const hopDescriptor = captured.descriptors.get("spectrumHopFrames");
  const spectrumDescriptor = captured.descriptors.get("spectrum");
  const collectionDescriptor = captured.descriptors.get("spectrumCollection");
  if (hopDescriptor !== undefined) {
    // The current SDK shape carries the hop as an ordinary enumerable data field. A hidden field
    // is neither a valid new shape nor an old compatibility shape, and must not bypass validation.
    const noSpectrumFields = spectrumDescriptor === undefined && collectionDescriptor === undefined;
    if (!hopDescriptor.enumerable
        || (!hasEnumerableShape(captured, BOOT_OPTION_FIELDS)
          && !(noSpectrumFields && hasEnumerableShape(captured, NEW_NO_SPECTRUM_BOOT_OPTION_FIELDS)))) {
      return undefined;
    }
  } else {
    // Direct callers from before the hop was added remain accepted. Keep the old spectrum shape
    // rules exactly: both spectrum fields enumerable means the eight-field shape, while hidden or
    // absent optional fields may be normalized as before.
    const spectrumIsEnumerable = spectrumDescriptor?.enumerable === true;
    const collectionIsEnumerable = collectionDescriptor?.enumerable === true;
    if (spectrumIsEnumerable || collectionIsEnumerable) {
      if (!hasEnumerableShape(captured, SPECTRUM_BOOT_OPTION_FIELDS)) return undefined;
    } else {
      if (!hasEnumerableShape(captured, LEGACY_BOOT_OPTION_FIELDS)) return undefined;
      if (spectrumDescriptor !== undefined
          && spectrumDescriptor.value !== undefined && spectrumDescriptor.value !== null) return undefined;
    }
  }
  const spectrumValue = spectrumDescriptor === undefined ? undefined : spectrumDescriptor.value;
  const spectrum = snapshotSpectrumBoot(spectrumValue ?? null);
  const spectrumCollection = snapshotSpectrumCollection(
    (collectionDescriptor === undefined ? undefined : collectionDescriptor.value) ?? null,
  );
  if (spectrum === undefined || spectrumCollection === undefined) return undefined;
  const snapshot = {
    sourceRingFrames: captured.descriptors.get("sourceRingFrames").value,
    maximumMemoryBytes: captured.descriptors.get("maximumMemoryBytes").value,
    reserved1: captured.descriptors.get("reserved1").value,
    liveControlMeterBlocks: captured.descriptors.get("liveControlMeterBlocks").value,
    liveControlObservationTaps: captured.descriptors.get("liveControlObservationTaps").value,
    reserved2: captured.descriptors.get("reserved2").value,
    spectrumHopFrames: hopDescriptor === undefined ? 0 : hopDescriptor.value,
    spectrum,
    spectrumCollection,
  };
  return validBootOptions(snapshot) ? snapshot : undefined;
}

function snapshotFactory(value) {
  const captured = capturePlainRecord(value, [...OPTION_FIELDS, "preparedModule"]);
  if (!captured || !hasRequiredEnumerableFields(captured, OPTION_FIELDS)) return undefined;
  const preparedDescriptor = captured.descriptors.get("preparedModule");
  const preparedModule = preparedDescriptor === undefined ? undefined : preparedDescriptor.value;
  if (preparedModule === undefined) {
    if (captured.enumerableCount !== OPTION_FIELDS.length) return undefined;
  } else if (!preparedDescriptor.enumerable
      || captured.enumerableCount !== OPTION_FIELDS.length + 1) {
    return undefined;
  }
  return { descriptors: captured.descriptors, preparedModule };
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

function validSpectrumBoot(options) {
  if (options === null) return true;
  return hasExactFields(options, SPECTRUM_BOOT_FIELDS)
    && SPECTRUM_TARGETS.has(options.target)
    && typeof options.targetId === "string"
    && options.targetId.length > 0
    && new TextEncoder().encode(options.targetId).byteLength <= SPECTRUM_MAXIMUM_ID_BYTES
    && SPECTRUM_CHANNELS.has(options.channels)
    && Number.isSafeInteger(options.maximumCaptureBytes)
    && options.maximumCaptureBytes > 0
    && options.maximumCaptureBytes <= SPECTRUM_MAXIMUM_BYTES;
}

function validSpectrumCollectionBoot(options) {
  if (options === null) return true;
  return hasExactFields(options, SPECTRUM_COLLECTION_FIELDS)
    && Array.isArray(options.entries)
    && options.entries.length > 0
    && options.entries.every((entry) => hasExactFields(entry, SPECTRUM_COLLECTION_ENTRY_FIELDS)
      && SPECTRUM_TARGETS.has(entry.target)
      && typeof entry.targetId === "string"
      && entry.targetId.length > 0
      && new TextEncoder().encode(entry.targetId).byteLength <= SPECTRUM_MAXIMUM_ID_BYTES
      && SPECTRUM_CHANNELS.has(entry.channels))
    && Number.isSafeInteger(options.maximumCaptureBytes)
    && options.maximumCaptureBytes > 0;
}

function validSpectrumStreamMetadata(metadata) {
  return hasExactFields(metadata, [
    "structSize", "abiVersion", "result", "status", "target", "channels", "sampleRateHz",
    "quantumFrames", "hopFrames", "sourceUnderrun", "captureEpoch", "sequence",
    "droppedCaptures", "windows", "capturedSample", "endSample", "analysisEpoch",
    "historyStartSample", "smoothingMs",
  ])
    && metadata.structSize === SPECTRUM_STREAM_METADATA_BYTES
    && metadata.abiVersion === 0x00010000
    && validResult(metadata.result)
    && SPECTRUM_STREAM_STATUSES.has(metadata.status)
    && validU32(metadata.target) && metadata.target > 0 && metadata.target <= 3
    && validU32(metadata.channels) && metadata.channels > 0 && metadata.channels <= 3
    && validU32(metadata.sampleRateHz) && metadata.sampleRateHz > 0
    && validU32(metadata.quantumFrames) && metadata.quantumFrames > 0
    && validU32(metadata.hopFrames) && metadata.hopFrames > 0
    && validU32(metadata.sourceUnderrun) && metadata.sourceUnderrun <= 1
    && validU64(metadata.captureEpoch) && validU64(metadata.sequence)
    && validU64(metadata.droppedCaptures) && validU64(metadata.windows)
    && validU64(metadata.capturedSample) && validU64(metadata.endSample)
    && metadata.endSample >= metadata.capturedSample
    && validU64(metadata.analysisEpoch) && validU64(metadata.historyStartSample)
    && typeof metadata.smoothingMs === "number" && Number.isFinite(metadata.smoothingMs)
    && metadata.smoothingMs >= 0 && metadata.smoothingMs <= 10_000;
}

function validBootOptions(options) {
  return hasExactFields(options, BOOT_OPTION_FIELDS)
    && SPECTRUM_HOP_FRAMES.has(options.spectrumHopFrames)
    && validU32(options.sourceRingFrames)
    && validU64(options.maximumMemoryBytes)
    && options.reserved1 === 0n
    && validU64(options.liveControlMeterBlocks)
    && options.liveControlMeterBlocks <= 0xffffffffn
    && validU64(options.liveControlObservationTaps)
    && options.liveControlObservationTaps <= BigInt(MAXIMUM_OBSERVATION_TAPS)
    && options.reserved2 === 0n
    && validSpectrumBoot(options.spectrum ?? null)
    && validSpectrumCollectionBoot(options.spectrumCollection ?? null)
    && (options.spectrum === null || options.spectrum === undefined
      || options.spectrumCollection === null || options.spectrumCollection === undefined);
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
/// is never refused for being in flight: concurrent calls share one read, and a reply reflects
/// every request the host sent before the call (see `status()`). Dispose is terminal and
/// waits for nothing: the worklet handles messages in arrival order, so every earlier request is
/// already settled by the time its acknowledgement is posted.
///
/// Nothing is dropped silently: a request over its bound is rejected locally, before transfer, with
/// a typed `RESULT_BACKPRESSURE` and its planes still owned by the caller.
class MisoAudioWorkletHost {
  #port;
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
  #inFlightCommands = 0;
  #inFlightControl = 0;
  #inFlightObservationReads = 0;
  #inFlightTrackResponses = 0;
  #inFlightSpectrum = 0;
  #inFlightLease = new Set();
  #liveControlMeterBlocks;
  /// Issue #143: the host's own record of the taps it armed, keyed by the four addressing numbers.
  /// It is what `trackGrDb`'s positional array structurally cannot express.
  #observations = new Map();
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
    liveControlMeterBlocks,
  ) {
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
    this.#liveControlMeterBlocks = liveControlMeterBlocks;
    this.#port = node.port;
    this.#port.onmessage = (event) => this.#receive(event.data);
    this.#port.onmessageerror = () => this.#fail(webError(255, this.#oldestRequestId()));
    // A user-agent/processor crash cannot return storage already transferred out of this realm.
    this.node.onprocessorerror = () => this.#fail(webError(255, this.#oldestRequestId()));
    this.#startServiceTick();
  }

  // Issue #1381a D6: in `single` mode the instance's control work runs in the worklet's message
  // handler, so the worklet needs a message to run its service step on. This is that message: one
  // `miso.service.v1` per quantum, from the one timer the main realm may hold. The browser may
  // clamp the period; no deadline depends on it, because deadlines are counted in render samples.
  // The loop starts when boot succeeds and stops before dispose, and on a failure.
  // SERVICE_TICK_BEGIN
  #startServiceTick() {
    const periodMs = Math.max(1, Math.round((1000 * this.#quantumFrames) / this.#sampleRateHz));
    this.#serviceTick = setInterval(() => this.#postServiceTick(), periodMs);
  }
  // SERVICE_TICK_END

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

  #submitOrdinaryRecords(records, count) {
    return this.#request(
      { tag: "miso.command.v1", count, records },
      [records.buffer],
      "command",
    );
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
    } else if (pending.response === "command") {
      this.#inFlightCommands -= 1;
    } else if (pending.response === "control") {
      this.#inFlightControl -= 1;
    } else if (pending.response === "observationRead") {
      this.#inFlightObservationReads -= 1;
    } else if (pending.response === "trackResponse") {
      this.#inFlightTrackResponses -= 1;
    } else if (pending.response === "spectrum") {
      this.#inFlightSpectrum -= 1;
    } else if (pending.response === "lease" || pending.response === "sessionMap"
      || pending.response === "observationMap") {
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
      : pending.response === "command"
        ? [
          "tag", "requestId", "result", "reason", "rejectedIndex", "admitted", "appliedAtSample",
          "records",
        ]
        : pending.response === "control"
          ? ["tag", "requestId", "result", "response", "revision"]
        : pending.response === "sessionMap"
          ? ["tag", "requestId", "result", "sources", "metersAttached"]
          : pending.response === "observationMap"
            ? ["tag", "requestId", "result", "structureEpoch", "bindings"]
            : pending.response === "observationRead"
              ? ["tag", "requestId", "result", "rows"]
              : pending.response === "trackResponse"
                ? TRACK_RESPONSE_REPLY_FIELDS
                : pending.response === "spectrum"
                  ? pending.spectrumOperation === "streamRead"
                    ? SPECTRUM_STREAM_READ_REPLY_FIELDS
                    : pending.spectrumOperation === "streamSelect"
                      ? SPECTRUM_STREAM_SELECT_REPLY_FIELDS
                    : pending.spectrumOperation === "streamStart"
                      ? SPECTRUM_STREAM_START_REPLY_FIELDS
                      : pending.spectrumOperation === "streamStop"
                        ? SPECTRUM_STREAM_STOP_REPLY_FIELDS
                        : pending.spectrumOperation === "select"
                          ? SPECTRUM_SELECT_REPLY_FIELDS
                        : ["tag", "requestId", "result", "operation", "snapshot"]
          : pending.response === "status"
        ? [
          "tag", "requestId", "result", "state", "lastResult", "backend", "sampleRateHz",
          "quantumFrames", "blockingRebuilds", "nextAbsoluteSample", "renderedQuanta",
          "liveValuesSuperseded", "appliedRevision", "appliedSample", "appliedOutcome", "memoryBytes", "singleModeControlAllocations", "staleViewQuanta",
        ]
        : pending.response === "renderAllocations"
          ? ["tag", "requestId", "result", "count"]
        : pending.response === "sessionSeek"
          ? ["tag", "requestId", "result", "generation"]
        : pending.response === "seekReport"
          ? ["tag", "requestId", "result", "report"]
        : ["tag", "requestId", "result"];
    const expectedTag = pending.response === "status"
      ? "miso.status.v1"
      : pending.response === "renderAllocations"
        ? "miso.renderallocations.v1"
      : pending.response === "control"
        ? "miso.control.v1"
      : pending.response === "sessionMap"
        ? "miso.sessionmap.v1"
        : pending.response === "observationMap"
          ? "miso.observationmap.v1"
          : pending.response === "observationRead"
            ? "miso.observation.v1"
            : pending.response === "trackResponse"
              ? "miso.trackresponse.v1"
              : pending.response === "spectrum"
                ? "miso.spectrum.v1"
                : pending.response === "sessionSeek"
                  ? "miso.session-seek.v1"
                : pending.response === "seekReport"
                  ? "miso.seek-report.v1"
        : "miso.ack.v1";
    const validSourcePlanes = pending.response !== "source"
      || validReturnedPlanes(message.planes, pending.planeShape);
    const validCommandAck = pending.response !== "command" || (
      validCommandReason(message.reason)
      && validU32(message.rejectedIndex) && message.rejectedIndex < MAXIMUM_COMMAND_RECORDS
      && validU32(message.admitted) && message.admitted <= pending.commandCount
      && (message.result === RESULT_OK) === (message.admitted === pending.commandCount)
      && validU64(message.appliedAtSample)
      && message.records instanceof Uint8Array
      && message.records.byteLength === pending.commandCount * COMMAND_RECORD_BYTES
    );
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
    const validObservationMap = pending.response !== "observationMap" || (
      message.result === RESULT_OK && Array.isArray(message.bindings)
      // Issue #1562b: the structure epoch of the control list whose strip order the bindings
      // follow (the rendering plan's), a positive safe integer.
      && Number.isSafeInteger(message.structureEpoch) && message.structureEpoch > 0
      && message.bindings.every(validObservationMapBinding)
    );
    const validObservationRead = pending.response !== "observationRead" || (
      Array.isArray(message.rows)
      && (message.result !== RESULT_OK || message.rows.length === pending.observationCount)
      && (message.result === RESULT_OK || message.rows.length === 0)
      && message.rows.every(validObservationRow)
    );
    const validTrackResponse = pending.response !== "trackResponse" || (
      message.snapshot instanceof Uint8Array
      && message.snapshot.buffer instanceof ArrayBuffer
      && ((message.result === RESULT_OK
        && message.snapshot.byteLength > 0
        && message.snapshot.byteLength <= pending.trackResponseMaximumBytes)
        || (message.result !== RESULT_OK && message.snapshot.byteLength === 0))
    );
    const validSpectrum = pending.response !== "spectrum" || (
      message.operation === pending.spectrumOperation
      && ((pending.spectrumOperation === "streamSelect")
        ? validSpectrumStreamMetadata(message.metadata)
        : (pending.spectrumOperation === "streamStart" || pending.spectrumOperation === "streamStop")
        ? validSpectrumStreamMetadata(message.metadata)
        : pending.spectrumOperation === "streamRead"
          ? validSpectrumStreamMetadata(message.metadata)
            && Number.isSafeInteger(message.byteLength) && message.byteLength >= 0
            && message.byteLength <= SPECTRUM_MAXIMUM_BYTES
            && message.buffer instanceof ArrayBuffer
            && message.buffer.byteLength === pending.spectrumBufferBytes
            && (message.result === RESULT_OK
              ? (message.metadata.status === 6
                ? message.byteLength > 0 && message.byteLength <= message.buffer.byteLength
                : message.byteLength === 0)
              : message.byteLength === 0)
          : pending.spectrumOperation === "select"
            ? true
            : message.snapshot instanceof Uint8Array
            && message.snapshot.buffer instanceof ArrayBuffer
            && (message.result === RESULT_OK
              ? (pending.spectrumOperation === "read"
                ? message.snapshot.byteLength > 0 && message.snapshot.byteLength <= SPECTRUM_MAXIMUM_BYTES
                : message.snapshot.byteLength === 0)
              : message.snapshot.byteLength === 0))
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
    if (message.tag !== expectedTag || !hasExactFields(message, expectedFields)
        || !validRequestId(message.requestId) || !validResult(message.result)
        || !validSourcePlanes || !validStatus || !validRenderAllocations || !validCommandAck || !validControl || !validSessionMap
        || !validObservationMap || !validObservationRead || !validTrackResponse || !validSpectrum
        || !validSessionSeek || !validSeekReport) {
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
    this.#inFlightControl = 0;
    this.#inFlightObservationReads = 0;
    this.#inFlightTrackResponses = 0;
    this.#inFlightSpectrum = 0;
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
    // Issue #137 D1: the local bound is the worklet-side queue depth, so a flood is refused here,
    // before any transfer, and the caller keeps its record block. The engine-side bound is the
    // authority; this one only avoids paying a message round trip to be told so.
    if (response === "command") {
      return this.#inFlightCommands >= MAXIMUM_COMMAND_RECORDS;
    }
    if (response === "control") return this.#inFlightControl >= MAXIMUM_CONTROL_IN_FLIGHT;
    if (response === "observationRead") return this.#inFlightObservationReads >= 1;
    if (response === "trackResponse") return this.#inFlightTrackResponses >= 1;
    if (response === "spectrum") return this.#inFlightSpectrum >= 1;
    if (response === "lease" || response === "sessionMap" || response === "observationMap") {
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
    } else if (response === "command") {
      this.#inFlightCommands += 1;
    } else if (response === "control") {
      this.#inFlightControl += 1;
    } else if (response === "observationRead") {
      this.#inFlightObservationReads += 1;
    } else if (response === "trackResponse") {
      this.#inFlightTrackResponses += 1;
    } else if (response === "spectrum") {
      this.#inFlightSpectrum += 1;
    } else if (response === "lease" || response === "sessionMap" || response === "observationMap") {
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
        commandCount: stamped.count ?? 0,
        observationCount: Array.isArray(stamped.selections) ? stamped.selections.length : 0,
        trackResponseMaximumBytes: stamped.maximumResultBytes ?? LIVE_RESPONSE_MAXIMUM_BYTES,
        spectrumOperation: stamped.operation,
        spectrumBuffer: stamped.buffer,
        spectrumBufferBytes: stamped.buffer instanceof ArrayBuffer ? stamped.buffer.byteLength : 0,
        generation: stamped.generation,
        resolve,
        reject,
        response,
        planeShape: expectedPlanes === undefined ? undefined : planeShape(expectedPlanes),
      };
      this.#pending.set(requestId, pending);
      this.#reserve(response, sourceId);
      try {
        this.#port.postMessage(stamped, transfer);
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
  /// on the render thread after boot: render, command submission, meter polling and the
  /// observation, response and spectrum staging reads. Decision 15 requires it to stay zero. It
  /// is per instance, so a caller reads it on each host before that host's `dispose()`. The
  /// worklet answers in its message handler, never in `process()`.
  renderAllocationCount() {
    return this.#request({ tag: "miso.renderallocations.v1" }, [], "renderAllocations");
  }

  /// Submit one live-control command batch (issue #137 D1).
  ///
  /// The whole batch is one transaction: the acknowledgement's `result` is `RESULT_OK` only when
  /// every record was admitted, and `appliedAtSample` is the exact absolute sample the batch takes
  /// effect at. A refusal names `reason` and `rejectedIndex` and admits nothing.
  command(request) {
    if (!hasExactFields(request, ["commands"])
        || !Array.isArray(request.commands)
        || request.commands.length === 0
        || request.commands.length > MAXIMUM_COMMAND_RECORDS
        || !request.commands.every(validCommand)) {
      return Promise.reject(webError(1));
    }
    return this.#submitOrdinaryRecords(encodeCommands(request.commands), request.commands.length);
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

  /// Subscribe to, or unsubscribe from, declared observation taps (issue #143).
  ///
  /// # Why this is not just `command()` with two more kinds
  ///
  /// It *is* those two kinds on the wire -- one batch, one transaction, the same
  /// `appliedAtSample`. What it adds is the answer to the question `trackGrDb` structurally cannot
  /// answer: **which** tracks have an observed effect at all. The frame's array is positional, so
  /// a track with no tap and a track that is not reducing both read `0`; the map this returns is
  /// where that distinction lives, and it is why the array can be read without null checks.
  ///
  /// The map is the host's own record of what it successfully armed. A batch is all-or-nothing, so
  /// it is updated only on `result === 0`, and it is cleared whenever the plan is replaced --
  /// subscriptions belong to the plan they were applied to (D7).
  async observe(request) {
    if (!hasExactFields(request, ["subscriptions"])
        || !Array.isArray(request.subscriptions)
        || request.subscriptions.length === 0
        || request.subscriptions.length > MAXIMUM_COMMAND_RECORDS
        || !request.subscriptions.every(validSubscription)) {
      return Promise.reject(webError(1));
    }
    const commands = request.subscriptions.map((subscription) => ({
      kind: subscription.armed
        ? OBSERVE_SUBSCRIBE_KIND
        : OBSERVE_UNSUBSCRIBE_KIND,
      rack: subscription.rack,
      stripId: subscription.stripId,
      effectId: subscription.effectId,
      tapId: subscription.tapId,
      windowBlocks: subscription.windowBlocks,
    }));
    const ack = await this.command({ commands });
    if (ack.result === 0) {
      for (const subscription of request.subscriptions) {
        // The tap's stable key, the one the core's `OBSERVATION_ENDED` names.
        const key = JSON.stringify([
          subscription.stripId, subscription.rack, subscription.effectId, subscription.tapId,
        ]);
        if (subscription.armed) {
          this.#observations.set(key, {
            stripId: subscription.stripId,
            rack: subscription.rack,
            effectId: subscription.effectId,
            tapId: subscription.tapId,
            windowBlocks: subscription.windowBlocks === 0
              ? Number(this.#liveControlMeterBlocks)
              : subscription.windowBlocks,
          });
        } else {
          this.#observations.delete(key);
        }
      }
    }
    return Object.freeze({
      tag: "miso.observe.v1",
      requestId: ack.requestId,
      result: ack.result,
      reason: ack.reason,
      bindings: Object.freeze([...this.#observations.values()]
        .sort((left, right) => compareIds(left.stripId, right.stripId)
          || left.rack - right.rack
          || compareIds(left.effectId, right.effectId)
          || left.tapId - right.tapId)
        .map(Object.freeze)),
    });
  }

  /// Read the compiled session's canonical track order (issue #137 D1), its sources (issue #207)
  /// and its canonical submix order (issue #1210 D3).
  ///
  /// This is the addressing authority for the strip positions the meter frame and the observation
  /// map use: the app never guesses an index. A command record does not use one: it names its tap
  /// by stable key (#1677).
  sessionMap() {
    return this.#request({ tag: "miso.sessionmap.v1" }, [], "sessionMap", false, undefined, "sessionMap");
  }

  /// Read the current prepared owner's stable observation bindings.
  observationMap() {
    return this.#request(
      { tag: "miso.observationmap.v1" },
      [],
      "observationMap",
      false,
      undefined,
      "observationMap",
    );
  }

  /// Read one bounded batch of numeric addresses resolved from `observationMap()`.
  readObservations(request) {
    if (!hasExactFields(request, ["selections"])
        || !Array.isArray(request.selections)
        || request.selections.length === 0
        || request.selections.length > MAXIMUM_OBSERVATION_READS
        || !request.selections.every(validObservationAddress)) {
      return Promise.reject(webError(1));
    }
    return this.#request(
      { tag: "miso.observation.v1", selections: request.selections },
      [],
      "observationRead",
    );
  }

  /// Capture one immutable selected-track response snapshot for the analysis Worker.
  ///
  /// The Worklet performs only the owner copy and returns detached bytes. Grid evaluation stays
  /// on the existing Rust analysis path, so this request never runs curve math in `process()` or
  /// in the Worklet message handler.
  captureTrackResponse(request) {
    if (!hasExactFields(request, TRACK_RESPONSE_REQUEST_FIELDS)
        || typeof request.trackId !== "string"
        || request.trackId.length === 0
        || request.trackId.length > LIVE_RESPONSE_MAXIMUM_ID_BYTES
        || !Number.isInteger(request.grid)
        || (request.grid !== LIVE_RESPONSE_CAPTURE_GRID_LINEAR
          && request.grid !== LIVE_RESPONSE_CAPTURE_GRID_LOGARITHMIC)
        || !Number.isInteger(request.channels)
        || (request.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_LEFT
          && request.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_RIGHT
          && request.channels !== LIVE_RESPONSE_CAPTURE_CHANNEL_BOTH)
        || !Number.isInteger(request.points) || request.points < 2
        || request.points > LIVE_RESPONSE_MAXIMUM_POINTS
        || typeof request.minimumHz !== "number" || !Number.isFinite(request.minimumHz)
        || typeof request.maximumHz !== "number" || !Number.isFinite(request.maximumHz)
        || !Number.isInteger(request.maximumResultBytes)
        || request.maximumResultBytes <= 0
        || request.maximumResultBytes > LIVE_RESPONSE_MAXIMUM_BYTES) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.trackresponse.v1", ...request },
      [],
      "trackResponse",
    );
  }

  /// Arm the prepared one-shot spectrum observer.
  armSpectrum() {
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "arm", channels: "both" },
      [],
      "spectrum",
    );
  }

  /// Read one completed spectrum window. A backpressure result carries an empty snapshot.
  readSpectrum(request) {
    if (!hasExactFields(request, ["channels"]) || !SPECTRUM_CHANNELS.has(request.channels)) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "read", channels: request.channels },
      [],
      "spectrum",
    );
  }

  /// Select one exact prepared collection entry before starting its managed stream.
  selectSpectrum(request) {
    if (!hasExactFields(request, ["target", "targetId", "channels"])
        || !SPECTRUM_TARGETS.has(request.target)
        || typeof request.targetId !== "string"
        || request.targetId.length === 0
        || new TextEncoder().encode(request.targetId).byteLength > SPECTRUM_MAXIMUM_ID_BYTES
        || !SPECTRUM_CHANNELS.has(request.channels)) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "select", ...request },
      [],
      "spectrum",
    );
  }

  /// Atomically update the active collection entry and smoothing profile.
  selectSpectrumStream(request) {
    if (!hasExactFields(request, ["target", "targetId", "channels", "smoothingMs"])
        || !SPECTRUM_TARGETS.has(request.target)
        || typeof request.targetId !== "string"
        || request.targetId.length === 0
        || new TextEncoder().encode(request.targetId).byteLength > SPECTRUM_MAXIMUM_ID_BYTES
        || !SPECTRUM_CHANNELS.has(request.channels)
        || typeof request.smoothingMs !== "number"
        || !Number.isFinite(request.smoothingMs)
        || request.smoothingMs < 0 || request.smoothingMs > 10_000) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "streamSelect", ...request },
      [],
      "spectrum",
    );
  }

  /// Cancel the prepared one-shot spectrum observer.
  cancelSpectrum() {
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "cancel", channels: "both" },
      [],
      "spectrum",
    );
  }

  /// Start the managed continuous spectrum stream for the prepared boundary.
  startSpectrumStream(smoothingMs = 100) {
    if (typeof smoothingMs !== "number" || !Number.isFinite(smoothingMs)
        || smoothingMs < 0 || smoothingMs > 10_000) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "streamStart", smoothingMs },
      [],
      "spectrum",
    );
  }

  /// Read the next scheduled managed stream window into the supplied reusable buffer.
  readSpectrumStream(buffer) {
    if (!(buffer instanceof ArrayBuffer) || buffer.byteLength === 0
        || buffer.byteLength > SPECTRUM_MAXIMUM_BYTES) {
      return Promise.reject(webError(RESULT_INVALID_ARGUMENT));
    }
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "streamRead", buffer },
      [buffer],
      "spectrum",
    );
  }

  /// Stop the managed continuous spectrum stream.
  stopSpectrumStream() {
    return this.#request(
      { tag: "miso.spectrum.v1", operation: "streamStop" },
      [],
      "spectrum",
    );
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

  async dispose() {
    if (this.#disposed) return;
    this.#stopServiceTick();
    await this.#request({ tag: "miso.dispose.v1" }, [], "dispose", true);
    this.#disposed = true;
    this.#observations.clear();
    this.#onMeterFrame = null;
    this.#onTelemetryFrame = null;
    this.#controlEventListeners.clear();
    this.#watermarkListeners.clear();
    this.#port.onmessage = null;
    this.#port.onmessageerror = null;
    this.node.onprocessorerror = null;
    try { this.#port.close?.(); } catch (_) { /* already closed */ }
    try { this.node.disconnect(); } catch (_) { /* never connected or already disconnected */ }
  }
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
  let simd128ModuleUrl;
  let workletModuleUrl;
  try {
    factorySnapshot = snapshotFactory(options);
    if (factorySnapshot === undefined) throw new Error("invalid factory");
    const descriptors = factorySnapshot.descriptors;
    context = descriptors.get("context").value;
    contextState = context?.state;
    quantumFrames = context?.renderQuantumSize ?? 128;
    sampleRateHz = context?.sampleRate;
    preparedModule = factorySnapshot.preparedModule;
    simd128ModuleUrl = descriptors.get("simd128ModuleUrl").value;
    workletModuleUrl = descriptors.get("workletModuleUrl").value;
    documentSnapshot = snapshotDocument(descriptors.get("document").value);
    bootOptions = snapshotBootOptions(descriptors.get("options").value);
    if ((preparedModule !== undefined && !(preparedModule instanceof WebAssembly.Module))
        || contextState !== "suspended"
        || !validU32(quantumFrames) || quantumFrames === 0
        || !validU32(sampleRateHz) || sampleRateHz === 0
        || documentSnapshot === undefined
        || bootOptions === undefined
        || typeof simd128ModuleUrl !== "string"
        || typeof workletModuleUrl !== "string") throw new Error("invalid factory data");
  } catch (_) {
    throw webError(RESULT_INVALID_ARGUMENT);
  }
  // W4-D1: attest before allocating anything. Thrown outside the `try` below so it reaches the
  // caller as itself rather than being folded into the generic 255 rejection.
  if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");
  let node;
  try {
    // Capture this capability only after SIMD attestation, so unsupported browsers do not touch it.
    const audioWorklet = context.audioWorklet;
    const selected = {
      backend: SHIPPING_BACKEND,
      module: preparedModule ?? await fetchModule(simd128ModuleUrl),
    };
    await audioWorklet.addModule(workletModuleUrl);
    if (context.state !== "suspended"
        || context.sampleRate !== sampleRateHz
        || (context.renderQuantumSize ?? 128) !== quantumFrames) {
      throw webError(RESULT_INVALID_ARGUMENT);
    }
    node = new AudioWorkletNode(context, PROCESSOR_NAME, {
      numberOfInputs: 0,
      numberOfOutputs: 1,
      outputChannelCount: [2],
      processorOptions: {
        module: selected.module,
        document: documentSnapshot,
        options: bootOptions,
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
          ["tag", "requestId", "result", "backend", "resources", "memoryBytes", "sourceRingFrames"],
        )
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
      // Issue #143: the plan's default observation window is the meter window; a subscription that
      // names `windowBlocks: 0` gets it, and the returned map says which one it got.
      Number(bootOptions.liveControlMeterBlocks),
    );
  } catch (error) {
    cleanupNode(node);
    if (isUnsupportedBrowser(error)) throw error;
    if (error?.tag === "miso.error.v1" && hasExactFields(error, ["tag", "requestId", "result"])
        && error.requestId === 0 && validResult(error.result)) throw error;
    throw webError(255);
  }
}
