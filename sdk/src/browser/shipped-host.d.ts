// Browser AudioWorklet host, ABI version 1.
//
// # The live console (issue 137)
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
// ## What the control path can and cannot move, and why
//
// **This is the most important thing to know before writing an app against it.** Issue #140 made
// every declared kind live, so the honest summary is now short: `MisoCommandKind.Pan`,
// `.Matrix`, `.FaderDb`, `.Mute`, `.EffectParam`, `.EffectBypass`, `.Solo`, `.TrimDb` and
// `.PolarityInvert` are all **applied**. So are `.ObserveSubscribe` and `.ObserveUnsubscribe`
// (issue 143), on the observation plane rather than the render one -- they move the
// `miso.observe.v1` subscription map, not anything rendered. All eleven are in the metadata JSON's
// `commandKinds`, each with the `plane` it applies on.
//
// * `matrix_ll/lr/rl/rr`, `fader_db`, `mute` and -- since issue 210 phase 3 -- `trim_db` and
//   `polarity_invert` declare `BuiltinParameterUpdateRate::BlockTarget` with a linear smoothing
//   policy. `hpf_hz`, `lpf_hz` and `delay_samples` (issue #210 phase 2, the track's input-side
//   time alignment) still declare `PreparedOnly` and have no command kind at all: they change
//   through a session edit. Live filter moves are deferred, at the price of the parametric EQ's
//   coefficient-ramp machinery; the ruling is recorded and is not an oversight.
// * An effect parameter is delivered to the running plan as a `PreparedAutomationSpan` -- the
//   route the effect contract always had and that #137 found nothing was feeding. A parameter is
//   movable exactly when its own descriptor declares it automatable; the build-time
//   parameter-metadata JSON carries that as `liveUpdatable`, so an app never has to discover it
//   at runtime.
// * `.EffectBypass` is applied *outside* the effect, by a latency-preserving shunt: the wet path
//   keeps running, so state stays continuous and un-bypassing does not click, and the dry signal
//   is delayed by exactly the effect's declared latency, so every compiled PDC route timing stays
//   correct.
//
// `MisoCommandReason.UnsupportedKind` (`result: 7`) has **not** gone away, and it still means
// exactly what it said: the target is real and the value is legal, and *this session* has no write
// path for it. A host compiled with `consoleCommandQueueRecords === 0n` is that session -- there
// is no control channel and no staging buffer -- and so is a future effect parameter that declares
// `AutomationRate::None`. It stays distinguishable from `Malformed` and from the `Unknown*`
// reasons for that reason.
//
// ## Addressing is session-stable and string-free
//
// A command names a track by its index in the compiled session's canonical normalized track order,
// which `sessionMap()` returns, plus a rack, an effect index and a numeric parameter ID. No string
// crosses the command path. The identity mapping is `sessionMap()` for the session and the
// build-time metadata JSON for the effect vocabulary.
//
// ## One batch is one transaction
//
// The worklet validates a whole submission -- shape, addressing, domain, and free queue room --
// before it pushes a single record. A refused batch admits nothing, so a half-applied fader move
// cannot exist, and a flood is refused before it reaches a queue. `RESULT_BACKPRESSURE` (6) is
// returned by the main-realm host locally when its own in-flight bound is reached, and by the
// engine when a bounded per-track control queue has no room.
//
// ## Application timing is exact
//
// Every console stage -- matrix/pan, fader/mute, and each driven effect -- drains its control
// queue at the top of the block, before it touches a sample. `appliedAtSample` on the
// acknowledgement is therefore the first sample of the next rendered block, and every sample of
// that block carries the change. It is an exact statement, not an estimate.
//
// A batch may address several kinds at once. It is still one transaction: the free-room check is
// per destination queue, and one full queue refuses the whole batch, including the records bound
// for queues that had room.
//
// ## What metering costs
//
// `consoleMeterBlocks === 0n` binds no meter observer at all: the render path folds nothing, and
// `meters({ enabled: true })` is refused with `RESULT_UNSUPPORTED` rather than reporting zeros. A
// nonzero value binds one post-matrix meter per track with a `blocks * quantumFrames` window and
// makes the port lease a second, finer switch over the master fold and every drain and post. The
// honest summary: *not attaching* meters costs nothing at all; attaching them and releasing the
// lease costs one branch per block plus the per-track observation fold, which runs whenever the
// observers exist.
//
// Gain reduction **is** in the meter frame, since issue #143. The frame carries `trackGrDb` --
// one non-negative decibel magnitude per track -- `masterGrDb`, and the `firstSample`/`endSample`
// of the window they were folded over. It rides the existing `miso.meter.v1` post: there is no
// second message and no second clock, so the pinned occurrence rule is unchanged.
//
// What a tap costs is declared, not guessed. Every effect publishes an `observations` menu in the
// build-time metadata JSON; a `resident` tap is a copy out of state the block already wrote and is
// `subscribable`, a `computed` tap is an analysis pass that does not ship in V1 and is refused with
// `MisoCommandReason.UnsupportedKind`. A session that never asks for observation
// (`consoleObservationTaps === 0n`) allocates none of it and renders byte-identical audio; inside a
// session that does, an unarmed tap costs one predicted branch per driven effect per block.
//
// ## What a frame costs the render callback
//
// One `postMessage` per window, not per block. The body is a frozen object allocated at
// construction whose two arrays are a `Float32Array` of `2 * trackCount + 2` peaks and one of
// `trackCount` gain-reduction magnitudes, both allocated at construction and overwritten in place;
// nothing is transferred, so the caller keeps no ownership obligation. The single allocation left
// is the structured clone `postMessage` performs, which is `4 * (3 * trackCount + 2)` bytes plus a
// handful of small numbers -- 392 bytes for a 32-track console. That cost has not been separately benchmarked in a browser; the telemetry
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
// nowhere for a further chunk to go -- plus one unsettled seek per source and one unsettled status.
// A request over its bound rejects locally with `RESULT_BACKPRESSURE` (6) before any transfer, so
// the caller keeps its planes and can retry.
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

/// Frozen live-console command kinds (issue 137 D1).
///
/// All eleven are one vocabulary, proved across every file that spells it -- this enum, the Rust
/// `COMMAND_*` constants, the wire's decode whitelist, the host JS `COMMAND_KINDS` set, the
/// metadata generator and the shipped `commandKinds` rows -- by
/// `scripts/check-command-kind-vocabulary.py`. Every one of them is *applied*: nothing here is
/// declared and refused (issue 140). Nine of them move state the render thread reads; the two
/// observation kinds move the `miso.observe.v1` subscription map and nothing rendered, which is
/// what the metadata JSON's per-kind `plane` field reports.
export const enum MisoCommandKind {
  /// Retarget the track's pan pair over an explicit ramp window. Applied.
  Pan = 1,
  /// Retarget the track's full 2x2 matrix over an explicit ramp window. Applied.
  Matrix = 2,
  /// Retarget a lane fader in decibels over an explicit ramp window. Applied (issue #140 B).
  FaderDb = 3,
  /// Set a lane mute, as a fader endpoint over the same window. Applied (issue #140 B).
  Mute = 4,
  /// Set an effect parameter, delivered as a prepared automation span. Applied (issue #140 A).
  EffectParam = 5,
  /// Set an effect bypass, through the latency-preserving shunt. Applied (issue #140 A).
  EffectBypass = 6,
  /// Arm one declared observation tap of one effect instance. Applied (issue #143).
  ///
  /// `parameterId` carries the effect-local **tap id** from the metadata JSON's per-effect
  /// `observations` array, `smoothingSamples` carries the window length in render blocks (`0` for
  /// the plan's default), and every `values` word must be `0`: a subscription changes what is read,
  /// never what is rendered.
  ///
  /// Applied on the `miso.observe.v1` plane: it binds an entry in the subscription map. The
  /// metadata JSON reports it as `"plane": "observation"`.
  ObserveSubscribe = 7,
  /// Disarm one declared observation tap of one effect instance. Applied (issue #143).
  ///
  /// Applied on the `miso.observe.v1` plane: it clears an entry from the subscription map. The
  /// metadata JSON reports it as `"plane": "observation"`.
  ObserveUnsubscribe = 8,
  /// Engage or clear one track's solo-in-place bit. Applied (issue 210 phase 1).
  ///
  /// Console state, not a strip DSP parameter -- which is why solo has no row in the metadata
  /// JSON's `builtins` table while it does have a `commandKinds` row. `rack` and `channel` are
  /// both `255`: solo addresses a strip, not a lane. `values[0]` is exactly `0` or `1`, and
  /// `smoothingSamples` is the engage/disengage fade -- the same declick window `Mute` takes, and
  /// worth giving the same value the app uses for a mute.
  ///
  /// Solo is composed, not stored twice: the engine keeps your per-lane mute intent and your solo
  /// bits as separate states and gates each strip on
  /// `userMute || (anySoloEngaged && !thisTrackSoloed)`. So muting a soloed track silences it,
  /// clearing solo restores exactly the mutes you had -- per lane -- and the whole gesture rides
  /// the same declicked fader endpoint a `Mute` rides.
  ///
  /// Solo is **monitoring state and is never persisted in a session** (issue 210 decision D1): an
  /// offline or stem render of a session can never come out soloed, and reloading a session
  /// starts with every solo bit clear.
  ///
  /// Metering is console-correct across a solo, and the code for it is unchanged: the gate sits at
  /// the fader, so a session's pre-fader send taps (`input`, `post_input_builtins`, `post_simd1`,
  /// `post_dynamic`, `post_simd2_pre_fader`) keep reading the un-gated signal, while
  /// `post_fader`, `post_matrix`, the submixes, the output and this host's own meter frame --
  /// including `masterGrDb` -- read the gated mix. A gain-reduction reading on a strip that solo
  /// has silenced falls toward zero reduction, because that is the true state of its signal path
  /// and not an artifact.
  Solo = 9,
  /// Retarget a lane's input trim in decibels over an explicit ramp window. Applied (issue 210
  /// phase 3).
  ///
  /// The gain-riding control a fader cannot substitute for: trim sits at the head of the strip,
  /// **before** the racks, so riding it changes what the compressor hears. `rack` is `255`;
  /// `channel` is `0` left, `1` right or `2` both; `values[0]` is the new `trim_db` and must lie
  /// in `[-144, 24]`, the domain the metadata JSON's `builtins` row declares; `smoothingSamples`
  /// is the ramp window in sample updates, read under the same linear-N law `FaderDb` uses.
  ///
  /// A `channel: 2` command is one record and moves both lanes together, over one window, at one
  /// block boundary. That is not only an economy: the engine collapses a mono-sourced strip to one
  /// plane when its two channels are doing identical work, and a both-channel trim ride keeps that
  /// true, while a single-lane ride correctly ends it for the life of the plan.
  ///
  /// A trim ride does **not** clear a polarity flip: the two are separate parameters that share
  /// one coefficient.
  TrimDb = 10,
  /// Set or clear a lane's input polarity inversion. Applied (issue 210 phase 3).
  ///
  /// `rack` is `255`; `channel` is `0` left, `1` right or `2` both; `values[0]` is exactly `0` or
  /// `1`; `smoothingSamples` is the flip's declick window.
  ///
  /// The flip is declicked, and it costs nothing to be: polarity is folded into the sign of the
  /// same trim coefficient `TrimDb` rides, so a flip is a retarget of that coefficient to its own
  /// negation and the linear ramp carries it through zero. Send a `smoothingSamples` you would be
  /// happy with on a fader move -- a few milliseconds -- and the flip is inaudible; send `0` and
  /// it is a hard switch, which is occasionally what a phase-alignment check wants.
  ///
  /// A flip does **not** change the lane's trim magnitude.
  PolarityInvert = 11,
}

/// Frozen typed reasons a live-console submission was refused (issue 137 D1).
export const enum MisoCommandReason {
  /// The submission was admitted whole.
  None = 0,
  /// Unknown kind, nonzero reserved word, non-finite value, or a field set for the wrong kind.
  Malformed = 1,
  /// `trackIndex` is not a track of the compiled session.
  UnknownTrack = 2,
  /// `rack` is not one of the three declared racks.
  UnknownRack = 3,
  /// `effectIndex` is not an effect of the addressed rack.
  UnknownEffect = 4,
  /// `parameterId` is not a parameter of the addressed effect.
  UnknownParameter = 5,
  /// A value is outside the addressed parameter's declared domain.
  Domain = 6,
  /// Well formed and correctly addressed; this ABI version cannot apply that kind.
  UnsupportedKind = 7,
  /// A bounded control queue had no room; nothing was admitted.
  Backpressure = 8,
  /// The engine is not `STATE_READY`.
  WrongState = 9,
  /// `parameterId` is not a declared observation tap of the addressed effect (issue 143).
  ///
  /// Deliberately not `UnknownParameter`: a parameter and a tap are different namespaces on the
  /// same effect, and a caller that confuses them learns which one it got wrong.
  UnknownTap = 10,
  /// The tap exists and the address is right; this session bound no observation capacity.
  ///
  /// Prepare with `consoleObservationTaps` set. Retrying will not help, which is why this is its
  /// own reason and not `Backpressure`.
  ObservationUnbound = 11,
}

/// One live-console command. `255` means "not applicable to this kind".
export interface MisoCommand {
  kind: MisoCommandKind;
  /// `0` simd1, `1` dynamic, `2` simd2, `255` for a builtin-addressed kind.
  rack: number;
  /// `0` left, `1` right, `2` both, `255` for a kind with no lane.
  channel: number;
  /// Index into the canonical track order `sessionMap()` returns.
  trackIndex: number;
  effectIndex: number;
  parameterId: number;
  /// Ramp window in sample updates for `Pan`, `Matrix`, `FaderDb`, `Mute`, `Solo`, `TrimDb` and
  /// `PolarityInvert`; the observation kinds read it as a window length in render blocks, and it
  /// is ignored by the rest.
  smoothingSamples: number;
  /// `Pan`: `[left, right, 0, 0]`. `Matrix`: `[ll, lr, rl, rr]`. `Mute`, `Solo` and
  /// `PolarityInvert`: `[0|1, 0, 0, 0]` exactly. Everything else: `[value, 0, 0, 0]`.
  values: [number, number, number, number];
}

export interface MisoCommandRequest {
  requestId: number;
  commands: MisoCommand[];
}

/// One live-console acknowledgement. `result` is `0` only when the whole batch was admitted.
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
  /// Canonical normalized track order. `trackIndex` indexes this.
  readonly tracks: string[];
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

/// One decimated meter window (issue 137 D2, extended by issue 143).
export interface MisoMeterFrame {
  readonly tag: "miso.meter.v1";
  readonly sequence: number;
  /// Complete windows folded into this frame; normally `1`.
  readonly windows: number;
  readonly trackCount: number;
  /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R]` peak magnitudes.
  readonly peaks: Float32Array;
  /// Gain reduction per track, in **non-negative decibels**, `trackCount` long (issue 143).
  ///
  /// Non-negative because the tap declares a `peakMagnitude` fold, not because the app clamps it:
  /// an effect holds its reduction as negative decibels internally, and the fold is `max(|x|)` over
  /// the window. An app's `Math.max(0, x ?? 0)` is therefore a no-op rather than a silent zeroing.
  ///
  /// Every entry is finite. `0` deliberately conflates "not reducing" with "no observed effect on
  /// this track", because the array is positional and read without null checks; the distinction
  /// lives in the `miso.observe.v1` acknowledgement's subscription map. Several armed taps on one
  /// track fold max-magnitude into the one slot, on the control plane.
  readonly trackGrDb: Float32Array;
  /// The designated master track's own folded reading, or `null` (issue 143 D6).
  ///
  /// `null` -- never `0` -- when no track was designated or the designated track published no
  /// window, because `0` would be indistinguishable from "the master is not reducing".
  readonly masterGrDb: number | null;
  /// Absolute sample the reported window opened at, inclusive.
  ///
  /// Correlate this against a `MisoCommandAck.appliedAtSample`: the first window whose
  /// `firstSample >= appliedAtSample` is the first that reflects the command. Consecutive windows
  /// tile with no gap, so `firstSample` of the next frame is this frame's `endSample`.
  readonly firstSample: bigint;
  /// Absolute sample the reported window closed at, exclusive.
  readonly endSample: bigint;
}

/// One arm/disarm request for a single declared observation tap (issues 143, 151).
///
/// Exactly these six fields, and no others: `observe()` rejects a subscription object carrying an
/// unknown field with `RESULT_INVALID_ARGUMENT` (1) before anything reaches the port, so an
/// optional-looking extra property is a local refusal rather than a field the engine ignores.
export interface MisoObservationSubscription {
  /// Index into the canonical track order `sessionMap()` returns.
  trackIndex: number;
  /// `0` simd1, `1` dynamic, `2` simd2. There is no `255` here: a tap always names a rack.
  rack: number;
  effectIndex: number;
  /// The effect-local tap id from the metadata JSON's per-effect `observations[].id`. Never `0`.
  ///
  /// This is a *tap* id, not a parameter id: they are different namespaces on one effect, and a
  /// parameter id sent here comes back as `MisoCommandReason.UnknownTap`, never
  /// `UnknownParameter`.
  tapId: number;
  /// Render blocks per published window, or `0` for the plan's default (`consoleMeterBlocks`).
  ///
  /// The returned binding reports the window that is actually in force, so a caller that sends `0`
  /// reads the resolved number back rather than having to know the default.
  windowBlocks: number;
  /// `true` arms the tap (wire kind `ObserveSubscribe`), `false` disarms it (`ObserveUnsubscribe`).
  armed: boolean;
}

/// One observation batch. Like a command batch, it is one transaction (issues 143, 151).
export interface MisoObservationRequest {
  requestId: number;
  /// At least one and at most `256` subscriptions, arming and disarming freely mixed.
  subscriptions: MisoObservationSubscription[];
}

/// The subscription map one `miso.observe.v1` acknowledgement carries (issue 143).
///
/// This is where "which tracks have an observed effect at all" lives. `trackGrDb` is positional
/// and cannot express absence; this can.
export interface MisoObservationBinding {
  readonly trackIndex: number;
  /// `0` simd1, `1` dynamic, `2` simd2.
  readonly rack: number;
  readonly effectIndex: number;
  /// The effect-local tap id, matching the metadata JSON's per-effect `observations[].id`.
  readonly tapId: number;
  /// Index into `MisoMeterFrame.trackGrDb` this tap folds into.
  readonly frameSlot: number;
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
/// # Subscriptions are per plan
///
/// A structural session edit replaces the plan, and subscriptions belong to the plan they were
/// applied to: the replacement's lanes exist and are unarmed, and the readers the old plan handed
/// out simply stop advancing. An app re-subscribes against the new plan and receives a fresh map
/// whose sequences restart at `1`. Nothing carries over, and nothing has to be torn down.
export interface MisoObservationAck {
  readonly tag: "miso.observe.v1";
  readonly requestId: number;
  readonly result: number;
  readonly reason: MisoCommandReason;
  /// Every armed tap of the current plan, in canonical `(track, rack, effectIndex, tapId)` order.
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
  /// Per-track control-queue depth in records, or `0n` to attach no control channel (issue 137).
  consoleCommandQueueRecords: bigint;
  /// Meter window in render blocks, or `0n` to bind no meter observer at all (issue 137).
  consoleMeterBlocks: bigint;
  /// Maximum declared observation taps to bind per effect, or `0n` for none at all (issue 143).
  ///
  /// Zero is the honest form: no lane, no accumulator and no conflating cell is allocated anywhere
  /// in the compiled plan, `observationRetainedBytes` is `0n`, and a subscription is refused with
  /// `MisoCommandReason.ObservationUnbound`. Requires `consoleCommandQueueRecords !== 0n`,
  /// because a subscription rides the effect's own command queue.
  consoleObservationTaps: bigint;
  /// The designated master track **plus one**, or `0n` for none (issue 143).
  ///
  /// V1 has no structural master bus -- submixes and outputs carry no effect racks -- so
  /// `masterGrDb` is a designation rather than a discovery. Plus one because zero has to keep
  /// meaning "unset". Requires `consoleObservationTaps !== 0n`.
  consoleMasterTrackPlusOne: bigint;
}

export interface MisoWebResourceReport {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly backend: number;
  readonly optionsBytes: bigint;
  readonly statusBytes: bigint;
  readonly sessionDocumentBytes: bigint;
  readonly diagnosticBytes: bigint;
  readonly sourceIdBytes: bigint;
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
  /// Exactly `0n` for a session prepared with `consoleObservationTaps === 0n`, and that zero is
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
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  readonly memoryBytes: number;
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
  requestId: number;
  sourceId: string;
  generation: bigint;
  startFrame: bigint;
  sampleRateHz: number;
  planes: Float32Array[];
  frames: number;
  endOfRegion: boolean;
}

export interface MisoSeekRequest {
  requestId: number;
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
  seekSource(request: MisoSeekRequest): Promise<MisoAck>;
  status(): Promise<MisoStatus>;
  /// Submit one live-console batch as a single transaction (issue 137 D1).
  command(request: MisoCommandRequest): Promise<MisoCommandAck>;
  /// Arm or disarm declared observation taps and read back the subscription map (issues 143, 151).
  ///
  /// It is wire kinds `ObserveSubscribe`/`ObserveUnsubscribe` on one `miso.command.v1` batch --
  /// one transaction, one `appliedAtSample` -- plus the map that answers what the positional
  /// `MisoMeterFrame.trackGrDb` array structurally cannot: which tracks have an observed effect
  /// at all. See `MisoObservationAck` for what a refusal settles as.
  observe(request: MisoObservationRequest): Promise<MisoObservationAck>;
  /// Read the compiled session's canonical track and source order (issues 137 D1, 207).
  ///
  /// `tracks` is what `trackIndex` addresses; `sources` is what `submitSource`/`seekSource` feed,
  /// with the channel count and region every submission has to agree with.
  sessionMap(): Promise<MisoSessionMap>;
  /// Take or release the decimated meter lease (issue 137 D2).
  meters(
    request: { requestId: number; enabled: boolean; onFrame: ((frame: MisoMeterFrame) => void) | null },
  ): Promise<MisoAck>;
  /// Take or release the render-telemetry lease (issue 137 D3).
  telemetry(
    request: { requestId: number; enabled: boolean; onFrame: ((frame: MisoTelemetryFrame) => void) | null },
  ): Promise<MisoAck>;
  dispose(): Promise<void>;
}

export interface CreateMisoAudioWorkletHostOptions {
  context: BaseAudioContext;
  document: Uint8Array;
  options: MisoWebBootOptions;
  simd128ModuleUrl: string;
  workletModuleUrl: string;
}

export function createMisoAudioWorkletHost(
  options: CreateMisoAudioWorkletHostOptions,
): Promise<MisoAudioWorkletHost>;
