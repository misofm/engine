// Browser AudioWorklet host, ABI version 1.
//
// # The live controls (issue 137)
//
// V1 as issue 024 froze it was a deterministic *renderer*: create, stream, render, dispose. Issue
// 137 adds the three things a live mixing console needs on top, additively -- no existing message,
// field or live result code changed meaning, and the resource report keeps its 224-byte layout.
//
//   1. `miso.command.v1`, a control path from the main realm into the engine, acknowledged with
//      the exact absolute sample the batch took effect at.
//   2. `miso.meters.v1` / `miso.meter.v1`, a decimated peak stream on a lease.
//   3. `miso.telemetry.v1`, windowed render-time telemetry on a lease, measured in JavaScript
//      around the render export.
//
// ## What the command path carries, and what it does not
//
// **This is the most important thing to know before writing an app against it.** The browser has
// no value command of its own (issue #1565). Every live value -- a fader, a mute, a pan or matrix,
// an input trim, polarity or filter, a send, a VCA, a solo, an effect parameter or bypass -- moves
// through `control()`: a `SESSION_TRANSACTION_APPLY` carrying `SET_CONTROL` edits, the same command
// frame the C ABI's `miso_engine_v1_submit_command` takes, decoded and classified once by the
// engine's control plane. `command()` carries exactly two kinds, `MisoCommandKind.ObserveSubscribe`
// and `.ObserveUnsubscribe` (issue 143), on the observation plane rather than the render one: they
// move the `miso.observe.v1` subscription map, not anything rendered. A record of any other kind --
// including the deleted kinds 1-6 and 9-17, whose numbers are never reallocated -- is refused
// `MisoCommandReason.Malformed` at its index.
//
// `MisoCommandReason.UnsupportedKind` (`result: 7`) means the target is real and the value is
// legal, and *this session* has no path for it. A host booted with
// `liveControlObservationTaps === 0n` is that session: it has no command staging, so `command()`
// acknowledges every batch `UnsupportedKind`.
//
// ## Addressing is by stable key (issue 1677)
//
// A command names its tap by the stable key the engine's `OBSERVATION_ENDED` event uses: the
// strip's ID (a track or a submix), the rack (`1` inserts, `3` console), the insert's or the
// console slot's ID, and the effect-local tap ID. It carries no position, so a rebuild cannot make
// a record arm another effect's tap. The engine resolves the key on the newest committed plan --
// the successor between a rebuild's commit and the block that adopts it -- and refuses a key that
// plan does not have with a typed reason (`UnknownTrack`, `UnknownEffect`, `UnknownTap`) before it
// queues anything. The build-time metadata JSON is the vocabulary of the tap IDs.
//
// ## One batch is one transaction
//
// The worklet validates a whole submission -- shape, addressing and free queue room -- before it
// pushes a single record. A refused batch admits nothing. Each effect keeps one bounded
// observation queue, one slot per declared tap, so subscriptions stay in order, and a submission
// that outruns it is refused before it reaches the queue. A submission names each observation tap
// at most once: a second `ObserveSubscribe` or `ObserveUnsubscribe` for the same tap is
// `Malformed`. `RESULT_BACKPRESSURE` (6) is returned by the main-realm host locally when its own
// in-flight bound (256 records, `maximumCommandRecords`) is reached, and by the engine when an
// effect's observation queue has no room because earlier submissions are not yet drained; a render
// clears it.
//
// ## Application timing is exact
//
// An effect stage drains its observation queue at the top of the block, before it touches a
// sample. `appliedAtSample` on the acknowledgement is the first sample of the next rendered block,
// when the subscription takes effect.
//
// A batch may address several taps at once. It is still one transaction: the free-room check is
// per observation queue, and one full queue refuses the whole batch.
//
// ## What metering costs
//
// Every plan meters every strip (one post-matrix peak pair per strip, tracks then submixes) and the
// output. `liveControlMeterBlocks === 0n` gives the strip meters the engine's default window (about
// 32 ms) and delivers nothing: `meters({ enabled: true })` is refused with `RESULT_UNSUPPORTED`
// rather than reporting zeros, and the output fold stays shut at one load per block. A nonzero
// value sets a `blocks * quantumFrames` window and makes the port lease a second, finer switch
// over the output fold and every drain and post. The honest summary: releasing the lease, or
// never taking it, costs one load per block plus the per-strip observation fold, which runs on
// every plan.
//
// Gain reduction **is** in the meter frame, since issue #143. The frame carries `trackGrDb` --
// one non-negative decibel magnitude per track -- and `masterGrDb`. These are latest folds from
// independently aged effect observations; their sequence/span is not transported in this frame,
// and they never supply or inherit the `firstSample`/`endSample` peak timestamps. The additive
// `generation`, `validity` and `lossCount` fields make lease resets, producer resets and dropped
// peak windows visible without changing the peak array.
//
// The gain-reduction section is recomputed from the taps armed when each frame is polled. An
// unsubscribe disarms its tap when it is admitted, so every meter frame received after its
// `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an armed
// tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the frame leaves
// that tap out until a window that opens at or after that sample closes. A strip's slot
// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at or
// after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is `null`
// when the designated strip has none.
//
// What a tap costs is declared, not guessed. Every effect publishes an `observations` menu in the
// build-time metadata JSON; a `resident` tap is a copy out of state the block already wrote and is
// `subscribable`, a `computed` tap is an analysis pass that does not ship in V1 and is refused with
// `MisoCommandReason.UnsupportedKind`. A session that never asks for observation
// (`liveControlObservationTaps === 0n`) allocates none of it and renders byte-identical audio;
// inside a session that does, an unarmed tap costs one predicted branch per driven effect per
// block.
//
// ## What a frame costs the render callback
//
// One `postMessage` per window, not per block. The body is a frozen object allocated at
// construction whose four arrays are a `Float32Array` of `2 * trackCount + 2` peaks, one of
// `trackCount` gain-reduction magnitudes, and the buses' `2 * submixCount` peaks and
// `submixCount` magnitudes, all allocated at construction and overwritten in place; nothing is
// transferred, so the caller keeps no ownership obligation. The counts are the plan's: a structural
// apply that adds or removes a strip gives its successor a frame of its own size, whose body and
// views are made in the message handler that committed it (never in the render callback), and the
// first window the successor closes is published in it, in the same stream (`sequence`,
// `generation` and `lossCount` continue). The single allocation left is the
// structured clone `postMessage` performs, which is `4 * (3 * (trackCount + submixCount) + 2)`
// bytes plus a handful of small numbers -- 392 bytes for a 32-track console with no buses. That cost has not been separately benchmarked in a browser; the telemetry
// lease measures the render export, not the post around it.
//
// # Exactly one artifact
//
// Owner decision W4-D1: the shipped module is built with `+simd128`. There is no scalar artifact
// and no dual-artifact selection. `createMisoAudioWorkletHost` validates a canned `simd128` module
// with `WebAssembly.validate` before it fetches anything and rejects with a typed
// `MisoUnsupportedBrowser` (`tag: "miso.unsupported.v1"`, `capability: "simd128"`) when the probe
// fails -- the browser twin of the native x86-64-v3 boot attestation. A browser below the floor is
// refused, never silently degraded.
//
// # Trap means processor death
//
// `wasm32-unknown-unknown` is `panic = abort`. A Rust panic inside the worklet aborts the Wasm
// instance; there is nothing to catch inside Rust and no `catch_unwind` exists on this path.
// `process()` converts a throw from the render export into sticky `RESULT_INTERNAL` (255) and
// positive-zero output, and the user agent may also fire `processorerror`. Any other export that
// throws settles the pending request as `miso.error.v1` with result 255.
//
// # A render failure never frees
//
// `RESULT_RENDER_REJECTED` (8) and `RESULT_INTERNAL` (255) from `process()` are sticky: the engine
// keeps the render plan, compiled session and source rings alive in its one-slot retirement queue
// and emits positive-zero silence. `dispose()` -- delivered on the port, never inside `process()`
// -- is the single point at which that storage is freed.
//
// # Streaming model
//
// Sources stream just-in-time into bounded per-source rings. One message carries exactly one
// quantum of planar PCM (the final chunk of a region may be shorter). Up to
// `sourceRingFrames / quantumFrames` chunks may be unsettled per source ID at once -- there is
// nowhere for a further chunk to go -- plus one unsettled seek per source. A request over its bound
// rejects locally with `RESULT_BACKPRESSURE` (6) before any transfer, so the caller keeps its planes
// and can retry. Status is never refused for being in flight: see `status()`.
//
// The default ring covers a 100 ms main-thread stall: `(ceil(100ms * fs / quantum) + 2) * quantum`
// frames, which is 5 120 frames (40 KiB for stereo `f32`) at 48 kHz with a 128-frame quantum. It is
// a prefill ahead of the render position and adds no output latency.
//
// Two options deliberately not taken, recorded so they are not rediscovered as bugs:
//
//   1. Zero-copy submission (`source_reserve` / `source_commit`) and a chunk size decoupled from
//      the quantum. Both need a `source` contract change, which is that crate's issue
//      (#101); today `validate_submission_metadata` requires exactly one quantum unless the chunk
//      ends the region.
//   2. A `SharedArrayBuffer` ring under `crossOriginIsolated === true`, which would remove the
//      message round trip entirely. It requires the engine's atomics-free policy to be revisited
//      and is an owner decision, not a host change.
//
// # Compilation happens on the rendering thread
//
// Session JSON is compiled inside the `AudioWorkletProcessor` constructor, which runs on the
// rendering thread before the first `process()` call. This is documented V1 behaviour (issue 024,
// owner open question 2): construction allocates, later rendering does not.
//
/// The frozen backend names of the issue-024 ABI. Only `"simd128"` is shipped (W4-D1); `"scalar"`
/// remains a legal ABI value because the Rust artifact still reports backend `0` when it is built
/// without `+simd128`, and the processor rejects such a module rather than rendering with it.
export type MisoWebBackend = "scalar" | "simd128";

/// Typed refusal of a browser that cannot run the shipped artifact.
///
/// Distinct from `MisoError` on purpose: a caller must be able to tell "this browser is out of
/// scope" apart from "something went wrong". It is thrown by `createMisoAudioWorkletHost` before
/// any node exists and never crosses the `MessagePort`.
export interface MisoUnsupportedBrowser {
  readonly tag: "miso.unsupported.v1";
  readonly requestId: 0;
  readonly result: 7;
  readonly capability: "simd128";
}

/// Frozen live-control command kinds (issue 137 D1).
///
/// Only the two observation kinds remain (issue #1565). They are one vocabulary, proved across
/// every file that spells it -- this enum, the Rust `COMMAND_*` constants, the wire's decode
/// whitelist, the host JS `COMMAND_KINDS` set, the metadata generator and the shipped
/// `commandKinds` rows -- by `scripts/check-command-kind-vocabulary.py`. Kinds 1-6 and 9-17 were
/// value kinds and are deleted: every value goes through `control()`. Their numbers are never
/// reallocated, and a record that carries one is refused `MisoCommandReason.Malformed`.
export const enum MisoCommandKind {
  /// Arm one declared observation tap of one effect instance. Applied (issue #143).
  ///
  /// `tapId` carries the effect-local **tap id** from the metadata JSON's per-effect
  /// `observations` array and `windowBlocks` the window length in render blocks (`0` for the
  /// plan's default): a subscription changes what is read, never what is rendered.
  ///
  /// Applied on the `miso.observe.v1` plane: it binds an entry in the subscription map. The
  /// metadata JSON reports it as `"plane": "observation"`.
  ObserveSubscribe = 7,
  /// Disarm one declared observation tap of one effect instance. Applied (issue #143).
  ///
  /// Applied on the `miso.observe.v1` plane: it clears an entry from the subscription map. The
  /// metadata JSON reports it as `"plane": "observation"`.
  ObserveUnsubscribe = 8,
}

/// Frozen typed reasons a live-control submission was refused (issue 137 D1).
export const enum MisoCommandReason {
  /// The submission was admitted whole.
  None = 0,
  /// Unknown kind, nonzero reserved word, an ID field that is not its length's bytes followed by
  /// zeros, or a submission that names one observation tap more than once.
  Malformed = 1,
  /// `stripId` is not a track or a submix of the newest committed plan. The spelling is kept (issue
  /// 1213, DESIGN P17).
  UnknownTrack = 2,
  /// `rack` is not an effect rack (`1` inserts or `3` console) on an effect-addressed kind. The
  /// retired `simd1` (`0`) and `simd2` (`2`) codes are refused with it only on the raw export
  /// path (issue #1096); `MisoAudioWorkletHost.command()` refuses them locally, before the port,
  /// by rejecting with a `miso.error.v1` invalid argument, as it refuses any out-of-set code.
  UnknownRack = 3,
  /// `effectId` is not a console slot or insert (per `rack`) of the addressed strip on the newest
  /// committed plan.
  UnknownEffect = 4,
  /// `parameterId` is not a parameter of the addressed effect.
  UnknownParameter = 5,
  /// A value is outside the addressed parameter's declared domain.
  Domain = 6,
  /// Well formed and correctly addressed; this ABI version cannot apply that kind.
  UnsupportedKind = 7,
  /// An effect's bounded observation queue had no room for the `ObserveSubscribe` or
  /// `ObserveUnsubscribe` records the submission queues because earlier submissions are not yet
  /// drained; nothing was admitted, and a render clears it. The reason is also returned with the internal result
  /// code, not a backpressure one, when the plan's revision gate refuses the batch's stamp (a
  /// revision past the gate's ceiling); that batch wrote nothing.
  ///
  /// Never the answer to a full observation event, which is `ObservationLimit`.
  Backpressure = 8,
  /// The engine is not `STATE_READY`.
  WrongState = 9,
  /// `tapId` is not a declared observation tap of the addressed effect (issue 143).
  ///
  /// Deliberately not `UnknownParameter`: a parameter and a tap are different namespaces on the
  /// same effect, and a caller that confuses them learns which one it got wrong.
  UnknownTap = 10,
  /// The tap exists and the address is right; this session bound no observation capacity.
  ///
  /// Prepare with `liveControlObservationTaps` set. Retrying will not help, which is why this is
  /// its own reason and not `Backpressure`.
  ObservationUnbound = 11,
  // Reasons 12-14 (`NotSoloable`, `UnknownRoute`, `UnknownVca`) were deleted with the value kinds
  // (issue #1565); their numbers are never reused.
  /// The subscription would leave more taps armed than the engine can end honestly in one
  /// `OBSERVATION_ENDED` event in one control frame (issue 1685: the event's exact encoded bytes
  /// against the browser's 16 KiB frame). `rejectedIndex` names the subscription that crossed it,
  /// after every unsubscribe of the same submission is counted; nothing was admitted. The result
  /// is `RESULT_UNSUPPORTED`.
  ///
  /// A render does not make room, so retrying the same submission will not help, which is why this
  /// is its own reason and not `Backpressure`: unsubscribe another tap first.
  ObservationLimit = 15,
}

/// One live-control command record: the subscription or withdrawal of one tap, named by its stable
/// key (issue 1677).
///
/// Only the two observation kinds are carried (issue #1565). An effect is addressed in the
/// session's terms (decision 12, issue #1096): a console slot as `rack` `3` with `effectId` the
/// slot's ID, and an insert as `rack` `1` with `effectId` the insert's ID. The retired `simd1` (`0`)
/// and `simd2` (`2`) codes are refused and never reallocated; the layout JSON's `racks` table is the
/// vocabulary. An ID is at most 127 UTF-8 bytes; a longer one rejects locally.
export interface MisoCommand {
  kind: MisoCommandKind;
  /// `1` inserts, `3` console.
  rack: number;
  /// The strip's ID: a track's or a submix's.
  stripId: string;
  /// The console slot's ID or the insert's ID.
  effectId: string;
  /// The effect-local observation tap id.
  tapId: number;
  /// The observation window length in render blocks, or `0` for the plan's default.
  windowBlocks: number;
}

export interface MisoCommandRequest {
  commands: MisoCommand[];
}

/// One live-control acknowledgement. `result` is `0` only when the whole batch was admitted.
export interface MisoCommandAck {
  readonly tag: "miso.ack.v1";
  readonly requestId: number;
  readonly result: number;
  readonly reason: MisoCommandReason;
  /// Index of the first refused record; `0` on success.
  readonly rejectedIndex: number;
  /// The whole batch, or zero.
  readonly admitted: number;
  /// The exact absolute sample the admitted records take effect at.
  readonly appliedAtSample: bigint;
  /// The caller's record block, handed straight back.
  readonly records: Uint8Array;
}

/// One compiled source, as the engine declares it (issue 207).
///
/// Named without a version suffix on purpose: issue 215 removes all version scoping at this
/// sprint's close, and a type minted now has nothing pinned against it to break.
///
/// This is what the *compiled* session knows about a source -- not what a decoder will eventually
/// find in a file. Every source is declared at the session rate and spans its full content from
/// frame zero, so only its channel count and total frame count belong in the per-source map.
export interface MisoSessionSource {
  /// Stable source ID -- the string `submitSource`/`seekSource` address the source by.
  readonly id: string;
  /// Declared source channels. Always at least one, and never above `maximumSourceChannels`.
  readonly channels: number;
  /// Full content length in source sample frames. Always at least one.
  readonly frames: bigint;
}

/// The compiled session's addressing authority (issue 137 D1, extended by issue 207).
export interface MisoSessionMap {
  readonly tag: "miso.sessionmap.v1";
  readonly requestId: number;
  readonly result: number;
  /// Canonical normalized source order (issue 207).
  ///
  /// The engine's own order: session compilation sorts `sources` by stable ID, and the queries
  /// behind this list read that sorted model rather than a copy of it. This is what makes a
  /// session compiled from raw JSON drivable -- the shapes a render loop must feed are here, and
  /// there is nowhere else in the browser ABI to learn them.
  readonly sources: MisoSessionSource[];
  /// Whether preparation bound meter observers at all.
  readonly metersAttached: boolean;
}

/** Numeric current-owner address used by the additive selected-observation request. */
export interface MisoObservationAddress {
  /// Strip index: the compiled session's tracks first, then its submixes (issue 1213).
  readonly trackIndex: number;
  /** `1` inserts or `3` console, addressed as `MisoCommand` addresses an effect. */
  readonly rack: number;
  readonly effectIndex: number;
  readonly tapId: number;
  /** `1` left, `2` right or `3` both. */
  readonly channels: number;
}

/** One resident observation binding, in the prepared owner's stable map order. */
export interface MisoObservationMapBinding {
  /// Strip index: the compiled session's tracks first, then its submixes (issue 1213).
  readonly trackIndex: number;
  /** `1` inserts or `3` console. */
  readonly rack: number;
  readonly effectIndex: number;
  readonly effectSlotId: string;
  readonly nativeEffectId: string;
  readonly tapIds: number[];
}

/** The current owner's observation map. */
export interface MisoObservationMap {
  readonly tag: "miso.observationmap.v1";
  readonly requestId: number;
  readonly result: number;
  /// The structure epoch of the control list whose strip order `bindings` follow (issue 1562b):
  /// the `structureEpoch` of `CONTROL_LIST_GET` pages, as the plan that renders carries it. A
  /// binding's `trackIndex` indexes the tracks, then the submixes, of the list of this epoch. A
  /// rebuild that adds, removes or reorders a strip changes it at the block that adopts the
  /// replacement plan; between that commit and the adoption the control list is already the
  /// replacement's, so a client names the strips of this map by the list of this epoch and never
  /// by the newest list. A positive safe integer.
  readonly structureEpoch: number;
  readonly bindings: MisoObservationMapBinding[];
}

/** One numeric row copied from the fixed observation-result records. */
export interface MisoObservationReadRow extends MisoObservationAddress {
  readonly status: number;
  readonly sampleRateHz: number;
  readonly firstSample: bigint;
  readonly endSample: bigint;
  readonly sequence: bigint;
  readonly blocks: number;
  readonly leftPresent: number;
  readonly rightPresent: number;
  readonly left: number;
  readonly right: number;
}

/** One bounded selected-observation reply. */
export interface MisoObservationReadReply {
  readonly tag: "miso.observation.v1";
  readonly requestId: number;
  readonly result: number;
  readonly rows: MisoObservationReadRow[];
}

/** Numeric grid selector used by the live track-response capture bridge. */
export type MisoTrackResponseGrid = 1 | 2;

/** Numeric channel selector used by the live track-response capture bridge. */
export type MisoTrackResponseChannels = 1 | 2 | 3;

/** One bounded capture request. Rust evaluates the grid after the Worklet copies this snapshot. */
export interface MisoTrackResponseCaptureRequest {
  readonly trackId: string;
  /** `1` linear or `2` logarithmic. */
  readonly grid: MisoTrackResponseGrid;
  /** `1` left, `2` right or `3` both. */
  readonly channels: MisoTrackResponseChannels;
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
  /** Total raw capture/result byte budget, including the fixed header. */
  readonly maximumResultBytes: number;
}

/** Detached immutable bytes captured from one prepared track boundary. */
export interface MisoTrackResponseCaptureReply {
  readonly tag: "miso.trackresponse.v1";
  readonly requestId: number;
  readonly result: number;
  /** Non-empty only for `result === 0`; the analysis Worker owns the next copy. */
  readonly snapshot: Uint8Array;
}

/// The track targets follow the `post_input` and `post_pan` taps (decision 12, issue #1096); their
/// wire codes, `1` and `2`, are unchanged.
export type MisoSpectrumTarget = "trackPostInput" | "trackPostPan" | "output";
export type MisoSpectrumChannels = "left" | "right" | "both";

/** One prepared spectrum boundary passed to the Worklet during boot. */
export interface MisoSpectrumBootOptions {
  readonly target: MisoSpectrumTarget;
  readonly targetId: string;
  readonly channels: MisoSpectrumChannels;
  readonly maximumCaptureBytes: number;
}

/** Several exact graph boundaries prepared for one atomic managed spectrum owner. */
export interface MisoSpectrumCollectionEntry {
  readonly target: MisoSpectrumTarget;
  readonly targetId: string;
  readonly channels: MisoSpectrumChannels;
}

/** Caller-sized collection staging; the limit covers all retained entries. */
export interface MisoSpectrumCollectionBootOptions {
  readonly entries: readonly MisoSpectrumCollectionEntry[];
  readonly maximumCaptureBytes: number;
}

export type MisoSpectrumOperation = "arm" | "read" | "cancel";

/** Reply for one bounded spectrum arm/read/cancel request. */
export interface MisoSpectrumReply {
  readonly tag: "miso.spectrum.v1";
  readonly requestId: number;
  readonly result: number;
  readonly operation: MisoSpectrumOperation;
  readonly snapshot: Uint8Array;
}

/** Raw generated metadata copied from one managed stream read. */
export interface MisoSpectrumStreamMetadata {
  readonly structSize: number;
  readonly abiVersion: number;
  readonly result: number;
  readonly status: number;
  readonly target: number;
  readonly channels: number;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly hopFrames: number;
  readonly sourceUnderrun: number;
  readonly captureEpoch: bigint;
  readonly sequence: bigint;
  readonly droppedCaptures: bigint;
  readonly windows: bigint;
  readonly capturedSample: bigint;
  readonly endSample: bigint;
  readonly analysisEpoch: bigint;
  readonly historyStartSample: bigint;
  readonly smoothingMs: number;
}

export interface MisoSpectrumStreamStartReply {
  readonly tag: "miso.spectrum.v1";
  readonly requestId: number;
  readonly result: number;
  readonly operation: "streamStart" | "streamStop";
  readonly metadata: MisoSpectrumStreamMetadata;
}

export interface MisoSpectrumSelectionReply {
  readonly tag: "miso.spectrum.v1";
  readonly requestId: number;
  readonly result: number;
  readonly operation: "select" | "streamSelect";
  readonly metadata?: MisoSpectrumStreamMetadata;
}

export interface MisoSpectrumStreamReadReply {
  readonly tag: "miso.spectrum.v1";
  readonly requestId: number;
  readonly result: number;
  readonly operation: "streamRead";
  readonly byteLength: number;
  readonly buffer: ArrayBuffer;
  readonly metadata: MisoSpectrumStreamMetadata;
}

/// One decimated meter window (issue 137 D2, extended by issue 143).
export interface MisoMeterFrame {
  readonly tag: "miso.meter.v1";
  readonly sequence: number;
  /// Publication generation; changes on each real lease transition or detected producer reset.
  readonly generation: bigint;
  /// Header validity bits: complete (`1`), master aligned (`2`), loss (`4`), gain reduction (`8`).
  readonly validity: number;
  /// Saturating count of dropped or rejected meter windows observed before this frame.
  readonly lossCount: number;
  /// Complete windows folded into this frame; normally `1`.
  readonly windows: number;
  /// The tracks of the plan that rendered the window. A structural apply that adds or removes a
  /// track changes it between two frames of one stream; every array below is sized for it.
  readonly trackCount: number;
  /// The structure epoch of the control list that the plan's strip order belongs to (issue
  /// 1562b): the `structureEpoch` that `CONTROL_LIST_GET` pages carry, as the commit that prepared
  /// the plan left it. The frame's strips (the tracks, then the submixes) are named by the list
  /// of this epoch, so a page that holds a list of another epoch reads the list again before it
  /// names a frame's strips. A structural apply that adds, removes or reorders a strip changes it
  /// between two frames of one stream, in the frame of the block that adopted the plan, and a
  /// replacement that keeps the structure keeps it. A positive safe integer.
  readonly structureEpoch: number;
  /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R]` peak magnitudes.
  readonly peaks: Float32Array;
  /// Gain reduction per track, in **non-negative decibels**, `trackCount` long (issue 143).
  ///
  /// Non-negative because the tap declares a `peakMagnitude` fold, not because the app clamps it:
  /// an effect holds its reduction as negative decibels internally, and the fold is `max(|x|)` over
  /// the window. An app's `Math.max(0, x ?? 0)` is therefore a no-op rather than a silent zeroing.
  /// A `gainChangeDb` tap (`peakSigned`, issue 1918) is signed, positive for a boost: it adds only
  /// a cut's depth here and a boost adds `0`; its signed value is read with `readObservations`.
  ///
  /// Every entry is finite. `0` deliberately conflates "not reducing" with "no observed effect on
  /// this track", because the array is positional and read without null checks; the distinction
  /// lives in the `miso.observe.v1` acknowledgement's subscription map. Several armed taps on one
  /// track fold max-magnitude into the one slot, on the control plane. Each value is the latest
  /// available independently aged effect fold; this frame carries no GR sequence or sample span.
  ///
  /// The gain-reduction section is recomputed from the taps armed when each frame is polled. An
  /// unsubscribe disarms its tap when it is admitted, so every meter frame received after its
  /// `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an
  /// armed tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the frame
  /// leaves that tap out until a window that opens at or after that sample closes. A strip's slot
  /// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at
  /// or after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is
  /// `null` when the designated strip has none.
  readonly trackGrDb: Float32Array;
  /// The designated master strip's own folded reading, or `null` (issue 143 D6). The master is the
  /// strip whose `master` monitor row is on (issue 1683).
  ///
  /// `null` -- never `0` -- when no strip was designated or the designated strip published no
  /// window, because `0` would be indistinguishable from "the master is not reducing".
  readonly masterGrDb: number | null;
  /// Absolute sample the reported window opened at, inclusive.
  ///
  /// This timestamps only `peaks`. Correlate it against a `MisoCommandAck.appliedAtSample`: the
  /// first peak window whose `firstSample >= appliedAtSample` is the first wholly after the
  /// command. A generation change starts a fresh delivery epoch. Within one generation, a gap
  /// between this frame's `endSample` and the next frame's `firstSample`, or the loss bit/count,
  /// reports omitted/rejected telemetry rather than a contiguous measurement.
  readonly firstSample: bigint;
  /// Absolute sample the reported window closed at, exclusive.
  readonly endSample: bigint;
  /// Submix strips (buses) this frame carries, in canonical submix order (issue 1209).
  readonly submixCount: number;
  /// `[bus0 L, bus0 R, .., busM L, busM R]` post-matrix peak magnitudes, `2 * submixCount` long,
  /// timestamped by `firstSample`/`endSample` exactly as `peaks` is.
  readonly submixPeaks: Float32Array;
  /// Gain reduction per bus, in non-negative decibels, `submixCount` long; every entry finite. It
  /// follows `trackGrDb`'s rules: positional, `0` for "not reducing" or "no observed effect",
  /// independently aged.
  readonly submixGrDb: Float32Array;
}

/// One arm/disarm request for a single declared observation tap (issues 143, 151).
///
/// Exactly these six fields, and no others: `observe()` rejects a subscription object carrying an
/// unknown field with `RESULT_INVALID_ARGUMENT` (1) before anything reaches the port, so an
/// optional-looking extra property is a local refusal rather than a field the engine ignores.
export interface MisoObservationSubscription {
  /// The strip's ID, as `MisoCommand.stripId`: a track's or a submix's.
  stripId: string;
  /// `1` inserts or `3` console, as `MisoCommand` addresses an effect. There is no `255` here:
  /// a tap always names a rack.
  rack: number;
  /// The console slot's ID or the insert's ID, as `MisoCommand.effectId`.
  effectId: string;
  /// The effect-local tap id from the metadata JSON's per-effect `observations[].id`. Never `0`.
  ///
  /// This is a *tap* id, not a parameter id: they are different namespaces on one effect, and a
  /// parameter id sent here comes back as `MisoCommandReason.UnknownTap`, never
  /// `UnknownParameter`.
  tapId: number;
  /// Render blocks per published window, or `0` for the plan's default (`liveControlMeterBlocks`).
  ///
  /// The returned binding reports the window that is actually in force, so a caller that sends `0`
  /// reads the resolved number back rather than having to know the default.
  windowBlocks: number;
  /// `true` arms the tap (wire kind `ObserveSubscribe`), `false` disarms it (`ObserveUnsubscribe`).
  armed: boolean;
}

/// One observation batch. Like a command batch, it is one transaction (issues 143, 151).
export interface MisoObservationRequest {
  /// At least one and at most `256` subscriptions, arming and disarming freely mixed.
  subscriptions: MisoObservationSubscription[];
}

/// The subscription map one `miso.observe.v1` acknowledgement carries (issue 143).
///
/// This is where "which tracks have an observed effect at all" lives. `trackGrDb` is positional
/// and cannot express absence; this can.
export interface MisoObservationBinding {
  /// The strip's ID. The strip's gain-reduction entry in a meter frame is at its position in the
  /// control list's strip order (tracks, then submixes).
  readonly stripId: string;
  /// `1` inserts or `3` console.
  readonly rack: number;
  /// The console slot's ID or the insert's ID.
  readonly effectId: string;
  /// The effect-local tap id, matching the metadata JSON's per-effect `observations[].id`.
  readonly tapId: number;
  /// Render blocks per published window, as it is actually in force.
  readonly windowBlocks: number;
}

/// One observation acknowledgement (issue 143).
///
/// # A refusal is a resolved acknowledgement, not a thrown error
///
/// `observe()` settles with this record whenever the engine answered at all, including when it
/// refused: `result` is nonzero, `reason` names why, and `bindings` is the map **unchanged**,
/// because a batch is all-or-nothing and a refused batch armed nothing. The two reasons the
/// observation path returns are `MisoCommandReason.UnknownTap` (10, `result: 1`) and
/// `MisoCommandReason.ObservationUnbound` (11, `result: 7`).
///
/// A refusal is per request and costs the host nothing: later commands, meter frames, renders and
/// further `observe()` calls all keep working. Only a locally malformed request rejects, with
/// `MisoError` and `result: 1`, before anything is sent.
///
/// # Subscriptions follow the key
///
/// A tap is named by its stable key, so a structural session edit does not move it: the core keeps
/// the tap armed on the replacement plan while the strip, the effect and the tap still exist, and
/// announces a tap it ended with an `OBSERVATION_ENDED` event. `bindings` is this host's own record
/// of what it armed and withdrew; it does not drop a tap the engine ended.
export interface MisoObservationAck {
  readonly tag: "miso.observe.v1";
  readonly requestId: number;
  readonly result: number;
  readonly reason: MisoCommandReason;
  /// Every tap this host armed and has not withdrawn, in `(stripId, rack, effectId, tapId)` order.
  readonly bindings: MisoObservationBinding[];
}

/// One windowed render-telemetry frame (issue 137 D3). JavaScript only; Wasm never sees the lease.
export interface MisoTelemetryFrame {
  readonly tag: "miso.telemetry.v1";
  readonly sequence: number;
  /// Blocks in the window.
  readonly blocks: number;
  /// Render time as a percentage of the block budget over the window.
  readonly cpuPercent: number;
  readonly peakBlockMs: number;
  readonly meanBlockMs: number;
  readonly budgetMs: number;
  /// Blocks whose measured render time exceeded the block budget.
  readonly deadlineMisses: number;
  /// Resolution of the clock the worklet actually found.
  readonly resolutionMs: number;
  /// `true` when the window measured exactly zero -- the clock could not see the work.
  readonly belowResolution: boolean;
}

export interface MisoWebBootOptions {
  /// Per-source ring override, or `0` for the engine's 100 ms plus two-quanta derivation.
  sourceRingFrames: number;
  /// Total boot memory budget, or `0n` for the engine-owned default ceiling.
  maximumMemoryBytes: bigint;
  /// Prepared continuous-spectrum hop in frames, or `0` for the engine-derived default.
  spectrumHopFrames: number;
  /// Reserved (offset 32 of the boot options): must be exactly `0n`. It was the live-command queue
  /// size; the command staging now exists exactly when `liveControlObservationTaps !== 0n`, and the
  /// main-realm host's bound on commands in flight is `maximumCommandRecords` (issue #1565).
  reserved1: bigint;
  /// Meter window in render blocks, or `0n` for the engine's default window with no delivery
  /// (issues 137, 1581).
  liveControlMeterBlocks: bigint;
  /// Maximum declared observation taps to bind per effect, or `0n` for none at all (issue 143).
  ///
  /// Zero is the honest form: no lane, no accumulator and no conflating cell is allocated anywhere
  /// in the compiled plan, `observationRetainedBytes` is `0n`, and a subscription is refused with
  /// `MisoCommandReason.UnsupportedKind`. A nonzero value also allocates the command staging
  /// that carries the subscription records; without it `command()` is refused `UnsupportedKind`.
  liveControlObservationTaps: bigint;
  /// Reserved (offset 56 of the boot options): must be exactly `0n`. It was the master boot word
  /// until issue 1689 retired it: the app designates the master with the strip's `master` monitor
  /// row (`SET_CONTROL`), as on every host, and the core holds it by strip ID. With no tap on the
  /// strip `masterGrDb` is `null`.
  reserved2: bigint;
  /// One optional prepared graph boundary; `null` retains no capture storage.
  spectrum?: MisoSpectrumBootOptions | null;
  spectrumCollection?: MisoSpectrumCollectionBootOptions | null;
}

export interface MisoWebResourceReport {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly backend: number;
  readonly optionsBytes: bigint;
  readonly statusBytes: bigint;
  readonly sessionDocumentBytes: bigint;
  readonly diagnosticBytes: bigint;
  readonly idStagingBytes: bigint;
  readonly sourcePcmStagingBytes: bigint;
  readonly outputPcmBytes: bigint;
  readonly bridgeMetadataBytes: bigint;
  readonly bridgeRetainedBytes: bigint;
  readonly largestBridgeAllocationBytes: bigint;
  readonly sourceTotalBytes: bigint;
  readonly sourceOverheadBytes: bigint;
  readonly effectScalarStateBytes: bigint;
  readonly effectScalarScratchBytes: bigint;
  readonly builtinRetainedBytes: bigint;
  readonly graphSessionPlusPlanBytes: bigint;
  readonly graphIncrementalPlanBytes: bigint;
  readonly graphMetadataBytes: bigint;
  readonly graphDelayBytes: bigint;
  readonly largestNamedAllocationBytes: bigint;
  /// Engine-owned bytes the plan's observation lanes and conflating cells retain (issue 143).
  ///
  /// Exactly `0n` for a session prepared with `liveControlObservationTaps === 0n`, and that zero is
  /// walked over the built runtime rather than computed from the configuration.
  readonly observationRetainedBytes: bigint;
}

export interface MisoStatus {
  readonly tag: "miso.status.v1";
  readonly requestId: number;
  readonly result: number;
  readonly state: number;
  readonly lastResult: number;
  readonly backend: number;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  /// The structural commands this instance rebuilt on the audio thread (issue 1294): a page
  /// without cross-origin isolation runs each one in the worklet's message handler, outside
  /// `process()`, as a blocking rebuild that is counted here. Saturates at `2 ** 32 - 1`.
  readonly blockingRebuilds: number;
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  /// The session's count of live values that a later committed value replaced before a block
  /// applied them (decision 15, D15-2), in the C ABI's unit (`COUNTERS_GET`). Every value reaches
  /// the engine through `control()` (issue #1565), the one path the C ABI takes. A value on a strip
  /// that a plan swap starts at rest is not counted here: a `COUNTERS_GET` sent with `control()`
  /// reports each such strip, once per adopted plan, as `STRIPS_RESTARTED` (protocol counter 18,
  /// issue 1768).
  readonly liveValuesSuperseded: bigint;
  /// The applied-revision watermark's revision (issue 1349): the highest committed revision in
  /// effect together with every revision before it, copied from the engine's render report.
  /// @internal The SDK's `engine.apply` is the public surface.
  readonly appliedRevision: bigint;
  /// The absolute render sample of the first block in which `appliedRevision` was in effect.
  /// @internal
  readonly appliedSample: bigint;
  /// The watermark's outcome flags, the C ABI's bits: `EXACT` 1, `TRANSITION_FALLBACK` 2,
  /// `SUPERSEDED` 4. `TRANSITION_FALLBACK` also reports a rebuild that carries a send following a
  /// stored mute into one that follows none (conservative: it can be set when no send moves at the
  /// swap; #1713 removes it). It also reports any rebuild that carries an automated following
  /// send while a live send move may still ramp (conservative; #1718 removes it).
  /// @internal
  readonly appliedOutcome: bigint;
  /// The module's memory size now: at least the boot size (`MisoAudioWorkletHost.memoryBytes`),
  /// larger after a control call grew it.
  readonly memoryBytes: number;
  /// The allocator calls the instance made outside its render-locked windows (issue 1381a, issue
  /// 1332 D1): its boot, its control messages and its service steps. In this module's `single`
  /// mode the control work runs in the worklet's message handler, so this counts the one place the
  /// engine allocates on an audio thread; the render-locked count (`renderAllocationCount()`)
  /// stays exactly zero.
  readonly singleModeControlAllocations: number;
  /// The quanta the worklet's `process()` silenced because the module's memory had moved before a
  /// message handler re-bound its views (a control call may grow the memory, which is no fault).
  /// Zero while every growth is re-bound in the handler that caused it.
  readonly staleViewQuanta: number;
}

/// The instance's render-locked allocator count (issue 1333 D3). Decision 15 requires zero.
export interface MisoRenderAllocations {
  readonly tag: "miso.renderallocations.v1";
  readonly requestId: number;
  readonly result: number;
  readonly count: number;
}

/// The reply to one control frame (issue 1562a D1). `response` is the response frame the engine's
/// control plane built, unchanged, when `result` is `0`; for another result it is the refusal's
/// diagnostic text (`code\tpath\n` lines), for example `web.apply.shape_unavailable` with `result: 10`
/// for a preparation that could not hand its plan to the host. A transaction that needs a plan
/// rebuild is prepared and published like any other (issue 1562b). A refused transaction is still `result: 0` with a non-ok response frame, as on the C ABI.
/// `revision` is the committed session revision after the call.
export interface MisoControlReply {
  readonly tag: "miso.control.v1";
  readonly requestId: number;
  readonly result: number;
  readonly response: Uint8Array;
  readonly revision: bigint;
}

/// One protocol event of the engine's control plane (issue 1570b D5): the frame bytes the engine
/// encoded, unchanged, and the lane it came from, `0` for the reliable lane and `1` for the lossy
/// lane.
export interface MisoControlEvent {
  readonly lane: 0 | 1;
  readonly frame: Uint8Array;
}

/// @internal The applied-revision watermark (issue 1294a), posted each time its
/// `(revision, sample)` pair changes. `revision` is the highest committed revision in effect
/// together with every revision before it, `sample` the absolute render sample of the first block
/// in which it was in effect, `outcome` the OR of the outcome flags over the revisions the advance
/// covered (`EXACT` 1, `TRANSITION_FALLBACK` 2, `SUPERSEDED` 4), and the three counts the
/// saturating, cumulative counts of revisions completed with each outcome.
export interface MisoWatermark {
  readonly revision: bigint;
  readonly sample: bigint;
  readonly outcome: bigint;
  readonly exactCount: bigint;
  readonly transitionFallbackCount: bigint;
  readonly supersededCount: bigint;
}

export interface MisoAck {
  readonly tag: "miso.ack.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoError {
  readonly tag: "miso.error.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoSourceRequest {
  sourceId: string;
  generation: bigint;
  startFrame: bigint;
  sampleRateHz: number;
  planes: Float32Array[];
  frames: number;
  endOfRegion: boolean;
}

export interface MisoSeekRequest {
  sourceId: string;
  generation: bigint;
  sourceFrame: bigint;
}

export interface MisoAudioWorkletHost {
  readonly node: AudioWorkletNode;
  readonly backend: MisoWebBackend;
  readonly resources: MisoWebResourceReport;
  readonly memoryBytes: number;
  submitSource(request: MisoSourceRequest): Promise<MisoAck>;
  /// Seeks source content while absolute render time remains continuous. Meter peak windows can
  /// straddle this content discontinuity; the seek does not advance the meter generation.
  seekSource(request: MisoSeekRequest): Promise<MisoAck>;
  /// Reads the engine status. Concurrent calls share one read, and a reply reflects every request
  /// the host sent before the call: a call joins the in-flight read while no later request was
  /// sent, and otherwise joins the one follow-up read sent when that read settles. A joined read
  /// settles every joined caller with the same reply or the same rejection, and a caller that stops
  /// waiting does not cancel it. `status()` is never refused for being in flight.
  status(): Promise<MisoStatus>;
  /// Read the allocator calls this instance made in the exports its processor calls on the render
  /// thread after boot (issue 1333 D3). Read it on each host before that host's `dispose()`.
  renderAllocationCount(): Promise<MisoRenderAllocations>;
  /// Send one command frame to the engine's control plane (issue 1562a D1): the frames the C ABI's
  /// `miso_engine_v1_submit_command` accepts, a `SESSION_TRANSACTION_APPLY` of `SET_CONTROL` edits,
  /// `CONTROL_LIST_GET`, `CONTROL_STATE_GET`, `SESSION_SNAPSHOT_GET`, counters and capabilities.
  /// Nothing in the frame is decoded here, and the engine decodes it in the worklet's message
  /// handler. The frame is copied. One request-ID allocator must serve every frame sent through
  /// this method, because the engine's replay cache refuses a reused ID with different bytes. A
  /// frame that is empty or over 16384 bytes rejects with `RESULT_INVALID_ARGUMENT` (1), and more
  /// than 64 frames in flight with `RESULT_BACKPRESSURE` (6), before the port.
  control(frame: Uint8Array): Promise<MisoControlReply>;
  /// Submit one batch of observation records (kinds 7 and 8) as a single transaction (issue 137
  /// D1). Values go through `control()`.
  command(request: MisoCommandRequest): Promise<MisoCommandAck>;
  /// Arm or disarm declared observation taps and read back the subscription map (issues 143, 151).
  ///
  /// It is wire kinds `ObserveSubscribe`/`ObserveUnsubscribe` on one `miso.command.v1` batch --
  /// one transaction, one `appliedAtSample` -- plus the map that answers what the positional
  /// `MisoMeterFrame.trackGrDb` array structurally cannot: which tracks have an observed effect
  /// at all. See `MisoObservationAck` for what a refusal settles as.
  observe(request: MisoObservationRequest): Promise<MisoObservationAck>;
  /// Read the current prepared owner's stable observation bindings.
  observationMap(): Promise<MisoObservationMap>;
  /// Read one bounded batch of numeric addresses resolved from `observationMap()`.
  readObservations(request: { selections: MisoObservationAddress[] }): Promise<MisoObservationReadReply>;
  /// Capture one immutable selected-track response snapshot for the existing Rust analysis Worker.
  captureTrackResponse(
    request: MisoTrackResponseCaptureRequest,
  ): Promise<MisoTrackResponseCaptureReply>;
  /// Arm the prepared spectrum boundary for its next complete window.
  armSpectrum(): Promise<MisoSpectrumReply>;
  /// Read one completed spectrum window; `result: 6` carries an empty pending snapshot.
  readSpectrum(request: { readonly channels: MisoSpectrumChannels }): Promise<MisoSpectrumReply>;
  /// Cancel the prepared spectrum boundary and discard any pending window.
  cancelSpectrum(): Promise<MisoSpectrumReply>;
  /// Start the continuously scheduled spectrum boundary.
  startSpectrumStream(smoothingMs?: number): Promise<MisoSpectrumStreamStartReply>;
  /// Select one prepared collection entry before starting its managed stream.
  selectSpectrum(request: {
    readonly target: MisoSpectrumTarget;
    readonly targetId: string;
    readonly channels: MisoSpectrumChannels;
  }): Promise<MisoSpectrumSelectionReply>;
  /// Atomically update the active collection entry and smoothing profile.
  selectSpectrumStream(request: {
    readonly target: MisoSpectrumTarget;
    readonly targetId: string;
    readonly channels: MisoSpectrumChannels;
    readonly smoothingMs: number;
  }): Promise<MisoSpectrumSelectionReply>;
  /// Read one scheduled stream window into a caller-owned reusable buffer.
  readSpectrumStream(buffer: ArrayBuffer): Promise<MisoSpectrumStreamReadReply>;
  /// Stop the continuously scheduled spectrum boundary.
  stopSpectrumStream(): Promise<MisoSpectrumStreamStartReply>;
  /// Read the compiled session's canonical source order (issues 137 D1, 207).
  ///
  /// `sources` is what `submitSource`/`seekSource` feed, with the channel count and region every
  /// submission has to agree with. The strip order is the control list's, not this map's (#1565).
  sessionMap(): Promise<MisoSessionMap>;
  /// Take or release the decimated meter lease (issue 137 D2).
  meters(
    request: { enabled: boolean; onFrame: ((frame: MisoMeterFrame) => void) | null },
  ): Promise<MisoAck>;
  /// Take or release the render-telemetry lease (issue 137 D3).
  telemetry(
    request: { enabled: boolean; onFrame: ((frame: MisoTelemetryFrame) => void) | null },
  ): Promise<MisoAck>;
  /// @internal Subscribe to the engine's protocol events (issue 1570b D5): every command's
  /// `SESSION_COMMITTED`, `OBSERVATION_ENDED` (reliable: a commit removed the effect of a
  /// subscribed observation tap, and the event names every ended tap key, #1584), `TRANSPORT_STATE`,
  /// `METER_BATCH`, `COUNTER_SNAPSHOT`, `DIAGNOSTIC` and the other events of the control protocol,
  /// as the engine encoded them. The listener gets each event with its lane (`0` reliable, `1`
  /// lossy) in arrival order. Returns the unsubscribe
  /// function. An event is not the ack of a command: the command's reply is.
  onControlEvent(listener: (event: MisoControlEvent) => void): () => void;
  /// @internal Subscribe to the applied-revision watermark (issue 1294a): the listener gets each
  /// change of the watermark's `(revision, sample)` pair, the newest record only. Returns the
  /// unsubscribe function. The SDK's `engine.apply` is the public surface.
  onWatermark(listener: (watermark: MisoWatermark) => void): () => void;
  /// @internal The count of engine events that arrived while no listener was subscribed (issue
  /// 1570b D5). Such an event is dropped here, which changes no engine state.
  readonly discardedControlEvents: number;
  dispose(): Promise<void>;
}

/// Boot preparation accepts same-realm plain-data records and undecorated arrays with standard
/// non-Proxy intrinsics. `context` and `preparedModule` are capability exceptions; `document` is
/// privately copied from a genuine `Uint8Array`. Invalid shapes reject the returned Promise before
/// loading.
export interface CreateMisoAudioWorkletHostOptions {
  context: BaseAudioContext;
  document: Uint8Array;
  options: MisoWebBootOptions;
  simd128ModuleUrl: string;
  preparedModule?: WebAssembly.Module;
  workletModuleUrl: string;
}

export function createMisoAudioWorkletHost(
  options: CreateMisoAudioWorkletHostOptions,
): Promise<MisoAudioWorkletHost>;
