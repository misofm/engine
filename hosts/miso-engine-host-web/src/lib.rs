//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the AudioWorklet boundary drives.
//!
//! # What is here and what is not
//!
//! The compile pipeline is **not** here. Parsing, compiling, source rings, effect and builtin
//! preparation, the graph compile, the identity bindings and the engine-owned resource report are
//! `miso-engine-host-core`, shared with the C ABI host (issue #103). This crate owns exactly what is
//! the browser's: the frozen issue-024 ABI structs and result codes, the fixed staging buffers the
//! JavaScript side reads and writes through raw addresses, the browser bridge's own resource rows,
//! and the render/failure state machine. Before #106 F1 it carried a second, already-diverged copy
//! of the shared pipeline -- 288 lines of `compile_ready` alone, only one of whose two copies
//! rejected source generation `0`.

use core::mem::{MaybeUninit, size_of};
use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::BTreeMap;

use miso_engine_builtins::{BuiltinLaneSelector, Matrix2x2, MeterSnapshot, MeterTap, pan_matrix};
use miso_engine_builtins_compiler::{
    MeterConsumer, TrackControlProducer, TrackControlRecordV1, TrackFaderRecordV1,
    TrackInputRecordV1,
};
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_effect_contract::{
    EffectControlRecord, ParameterChannel, ParameterChannelPolicy, parameter_value_valid,
};
use miso_engine_host_core::{
    CompiledSession, ConsoleSoloState, EffectControlProducer, EffectObservationHandle,
    EffectRack, HostConsoleRequestV1, HostPrepareCaps, HostShapePolicy, PreparedHost,
    SourceControlError, SourceSubmission, control_table_bytes, prepare_host_session_with_console,
    source_id_arena_bytes,
};

/// Frozen browser host ABI version 1.0.
pub const ABI_VERSION: u32 = 0x0001_0000;

/// Successful operation.
pub const RESULT_OK: u32 = 0;
/// Invalid caller input.
pub const RESULT_INVALID_ARGUMENT: u32 = 1;
/// ABI layout/version mismatch.
pub const RESULT_ABI_MISMATCH: u32 = 2;
/// Operation is invalid in the current state.
pub const RESULT_WRONG_STATE: u32 = 3;
/// A supplied buffer is too small.
pub const RESULT_BUFFER_TOO_SMALL: u32 = 4;
/// Preparation or compilation was rejected.
pub const RESULT_PREPARE_REJECTED: u32 = 5;
/// A bounded source queue is full.
pub const RESULT_BACKPRESSURE: u32 = 6;
/// Requested capability is unsupported.
pub const RESULT_UNSUPPORTED: u32 = 7;
/// Rendering failed.
pub const RESULT_RENDER_REJECTED: u32 = 8;
/// The browser configuration changed and requires a fresh host.
pub const RESULT_REPREPARE_REQUIRED: u32 = 9;
/// An internal invariant failed *and was detected by a checked path*.
///
/// A panic does not produce this code on `wasm32`: the target is `panic = abort`, so a panic traps
/// the instance and kills the processor. The worklet converts that trap into this code from
/// JavaScript (see the `.d.ts` header); nothing inside Rust catches it.
pub const RESULT_INTERNAL: u32 = 255;

/// Newly allocated configuration state.
pub const STATE_CONFIG: u32 = 0;
/// Fixed buffers have been prepared.
pub const STATE_PREPARED: u32 = 1;
/// A session and render plan are ready.
pub const STATE_READY: u32 = 2;
/// A sticky preparation or render failure occurred.
pub const STATE_FAILED: u32 = 3;
/// Ownership has been explicitly disposed.
pub const STATE_DISPOSED: u32 = 4;

/// Scalar Wasm backend.
pub const BACKEND_SCALAR: u32 = 0;
/// Base Wasm `simd128` backend.
pub const BACKEND_SIMD128: u32 = 1;

/// Session TOML staging buffer.
pub const BUFFER_SESSION_TOML: u32 = 1;
/// Source-ID staging buffer.
pub const BUFFER_SOURCE_ID: u32 = 2;
/// Planar source PCM staging buffer.
pub const BUFFER_SOURCE_PCM: u32 = 3;
/// Fixed diagnostic buffer.
pub const BUFFER_DIAGNOSTIC: u32 = 4;
/// Contiguous dual-mono output buffer.
pub const BUFFER_OUTPUT_PCM: u32 = 5;
/// Fixed live-console command staging buffer (issue #137 D1).
pub const BUFFER_COMMAND: u32 = 6;
/// Fixed decimated meter-frame buffer (issue #137 D2).
pub const BUFFER_METER_FRAME: u32 = 7;

/// Exact byte size of one staged `miso.command.v1` record.
pub const COMMAND_RECORD_BYTES: u32 = 48;
/// Largest number of records one `miso.command.v1` submission may stage.
///
/// The staging buffer is allocated at preparation for exactly this many records, so a submission
/// can never grow the module's memory and the render path never sees an allocation.
pub const MAXIMUM_COMMAND_RECORDS: u32 = 256;
/// Per-track control-queue depth used when the configuration asks for the default.
pub const DEFAULT_COMMAND_QUEUE_RECORDS: u32 = 64;
/// Largest `console_observation_taps` the browser configuration accepts.
///
/// The frame carries one gain-reduction slot per track, so a session cannot usefully bind more
/// taps per effect than a consumer can read; the cap keeps a mistyped configuration from asking
/// preparation for an unbounded menu. Every launch effect declares one tap.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 16;

/// Retarget the track's pan pair (`left`, `right`) over an explicit ramp window.
pub const COMMAND_PAN: u32 = 1;
/// Retarget the track's full 2x2 matrix over an explicit ramp window.
pub const COMMAND_MATRIX: u32 = 2;
/// Retarget a lane fader in decibels over an explicit ramp window (live since issue #140 B).
pub const COMMAND_FADER_DB: u32 = 3;
/// Set a lane mute, as a fader endpoint over the same ramp window (live since issue #140 B).
pub const COMMAND_MUTE: u32 = 4;
/// Set an effect parameter (live since issue #140 A).
pub const COMMAND_EFFECT_PARAM: u32 = 5;
/// Set an effect bypass (live since issue #140 A).
pub const COMMAND_EFFECT_BYPASS: u32 = 6;
/// Arm one declared observation tap of one effect instance (issue #143 D3).
///
/// `parameter_id` carries the effect-local `tap_id`, `smoothing_samples` carries the window length
/// in render blocks (`0` = the plan's default), and every value word must be `0.0`: a subscription
/// changes what is read, never what is rendered, so a nonzero value would be a caller mistake
/// rather than a meaningful field.
pub const COMMAND_OBSERVE_SUBSCRIBE: u32 = 7;
/// Disarm one declared observation tap of one effect instance (issue #143 D3).
pub const COMMAND_OBSERVE_UNSUBSCRIBE: u32 = 8;
/// Engage or clear one track's solo-in-place bit (issue #210 phase 1).
///
/// The shape mirrors [`COMMAND_MUTE`] because solo *is* a mute composition: `rack = 255`,
/// `channel = 255` (solo is a strip gesture, not a lane one), `values[0]` exactly `0.0` or `1.0`,
/// and `smoothing_samples` the engage/disengage fade -- the same declick window a mute takes.
///
/// It moves no state of its own on the render thread. Admission composes
/// `effective_mute = user_mute || (any_solo && !my_solo)` over the console's
/// [`miso_engine_host_core::ConsoleSoloState`] and emits the *existing*
/// `TrackFaderRecordV1::Mute` records into the *existing* per-track fader queues, so this kind is
/// on the `render` plane (it moves what the render thread reads) while adding nothing below
/// `admit_commands`. Refusals reuse the existing vocabulary: `malformed` for a wrong-shaped
/// record, `domain` for a `values[0]` outside `{0.0, 1.0}` (exactly as `mute` does),
/// `unknownTrack`, `backpressure` and `wrongState`.
pub const COMMAND_SOLO: u32 = 9;
/// Retarget a lane's input trim in decibels over an explicit ramp window (#210 phase 3).
///
/// `channel` is `0` left, `1` right or `2` both; `values[0]` is the new `trim_db` and must lie in
/// `trim_db`'s own declared domain, `[-144, 24]`; `values[1..]` must be `0.0`; `rack` is `255`.
/// `smoothing_samples` is the ramp window, in sample updates, exactly as it is for `faderDb`.
///
/// A `channel = 2` command is **one** record carrying `BuiltinLaneSelector::Both`, not two
/// per-lane records: the input stage is upstream of the fader/matrix seam, and a both-channel
/// retarget must be admitted as one symmetry-preserving event or it would retire the track's mono
/// collapse. `miso_engine_builtins_compiler::TrackInputRecordV1` carries the argument in full.
///
/// The lane's polarity is preserved: a trim ride does not clear a flip.
pub const COMMAND_TRIM_DB: u32 = 10;
/// Set or clear a lane's input polarity inversion (#210 phase 3).
///
/// `channel` is `0` left, `1` right or `2` both; `values[0]` must be exactly `0.0` or `1.0`;
/// `values[1..]` must be `0.0`; `rack` is `255`. `smoothing_samples` is the ramp window.
///
/// A flip is a retarget of the **same** coefficient the trim rides to its own negation, so it
/// declicks through the trim ramp -- the linear ramp carries the coefficient through zero -- and
/// costs no DSP of its own. The lane's trim magnitude is preserved.
pub const COMMAND_POLARITY_INVERT: u32 = 11;
// Issue #210's design proposed *reserving* kind 12 for `soloMode` (0 = SIP, 1 = PFL) here, so
// phase 5 would not have to renumber. It cannot be done, and the reason is a gate rather than a
// preference: `scripts/check-command-kind-vocabulary.py` requires the Rust constants to be
// contiguous from 1 and requires every other spelling -- the decode whitelist, the host JS set,
// the `.d.ts` enum, the metadata generator's rows and the shipped JSON, whose row *position*
// stands for its value -- to be that same list. A declared 12 with 10 and 11 absent is a gap in
// the authority; a declared 12 with nothing decoding it is a kind no caller can send. Either is
// red, and correctly so.
//
// So kinds are allocated when they ship, in the order they ship, and nothing is renumbered by a
// later phase: 9 is spent here, and `soloMode` takes whatever the next unclaimed value is when
// phase 5 threads it through all seven spellings. Recorded so the design's reservation is not
// read as a missing deliverable.

/// The submission was admitted whole.
pub const COMMAND_REASON_NONE: u32 = 0;
/// A record's fixed shape is wrong: an unknown kind, a nonzero reserved word, or a non-finite value.
pub const COMMAND_REASON_MALFORMED: u32 = 1;
/// `track_index` is not a track of the compiled session.
pub const COMMAND_REASON_UNKNOWN_TRACK: u32 = 2;
/// `rack` is not one of the three declared racks.
pub const COMMAND_REASON_UNKNOWN_RACK: u32 = 3;
/// `effect_index` is not an effect of the addressed rack.
pub const COMMAND_REASON_UNKNOWN_EFFECT: u32 = 4;
/// `parameter_id` is not a parameter of the addressed effect.
pub const COMMAND_REASON_UNKNOWN_PARAMETER: u32 = 5;
/// A value is outside the addressed parameter's declared domain.
pub const COMMAND_REASON_DOMAIN: u32 = 6;
/// The record is well formed and correctly addressed, but this session cannot apply its kind.
///
/// This is not "malformed" and it is not "unknown target": the parameter exists and the value is
/// legal, and this session has no post-preparation write path for it. The parameter-metadata JSON
/// (deliverable 4) marks every such parameter, so a caller never has to discover this at runtime.
///
/// Issue #140 emptied this of everything the ABI *declares*: pan, matrix, fader, mute, effect
/// parameter and effect bypass are all live, and `liveUpdatable` is `true` for each. It remains
/// reachable for the states that are genuinely not addressable -- a host with no console attached
/// at all, and an effect whose parameter declares `AutomationRate::None`, which no launch effect
/// does but which a future one may.
pub const COMMAND_REASON_UNSUPPORTED_KIND: u32 = 7;
/// A bounded control queue had no room for the submission; nothing was admitted.
pub const COMMAND_REASON_BACKPRESSURE: u32 = 8;
/// The host is not `STATE_READY`.
pub const COMMAND_REASON_WRONG_STATE: u32 = 9;
/// `parameter_id` is not a declared observation tap of the addressed effect (issue #143).
///
/// Deliberately **not** `UNKNOWN_PARAMETER`: a parameter and a tap are different namespaces on the
/// same effect, and a caller that confuses them learns which one it got wrong.
pub const COMMAND_REASON_UNKNOWN_TAP: u32 = 10;
/// The tap exists and the address is right, but this session bound no observation capacity.
///
/// The honest form of "you asked for a subscription this preparation cannot deliver": the effect
/// is there, the tap is declared, and the plan holds no lane to arm because the host asked for
/// none. A caller fixes it by preparing with `console_observation_taps` set, not by retrying.
pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;

/// The longest main-thread stall the default source ring hides without an underrun.
///
/// Chrome reports any main-thread task over 50 ms as a long task; a major garbage collection or a
/// layout burst in a real page runs 10-100 ms. The AudioWorklet render thread keeps pulling
/// quanta throughout, so a ring shorter than the stall underruns every source in the session. This
/// is a JIT-streaming budget, not a latency budget: the ring is prefilled ahead of the render
/// position and adds no output latency.
pub const SOURCE_STALL_TOLERANCE_MS: u32 = 100;

/// The default per-source ring capacity in frames for one rate and quantum.
///
/// `ceil(tolerance * fs / quantum)` quanta to cover the stall, plus two: one quantum is held by the
/// render consumer while it is being read, and one is in the recycle path between the consumer and
/// the producer. The result is always a whole number of quanta, which is what `PcmSourceRing`
/// requires of a capacity.
///
/// At 48 kHz / 128 that is 5 120 frames, 40 KiB for a stereo `f32` source -- 1 024 such sources fit
/// inside the 64 MiB default `maximum_source_total_bytes`.
///
/// `quantum_frames` must be nonzero. Every caller inside this crate runs after `validate_config`,
/// which rejects a zero quantum before any ring is built.
#[must_use]
pub const fn default_source_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
    let stall_frames = (sample_rate_hz as u64 * SOURCE_STALL_TOLERANCE_MS as u64) / 1000;
    let quanta = stall_frames.div_ceil(quantum_frames as u64) + 2;
    (quanta * quantum_frames as u64) as u32
}

/// Default meter window in render blocks: ~31 frames per second at 48 kHz with a 128-frame quantum.
pub const DEFAULT_METER_BLOCKS: u32 = 12;

/// Exact versioned live-console command report shared with JavaScript (issue #137 D1).
///
/// One submission is one transaction: either every staged record was admitted, or none was and
/// `rejected_index`/`reason` name the first record that broke a rule. `applied_at_sample` is the
/// absolute sample the admitted records take effect at -- the first sample of the next rendered
/// block, because the matrix stage drains its queue at the top of the block before it touches a
/// single sample of audio.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebCommandReport {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// One of the frozen `RESULT_*` values.
    pub result: u32,
    /// One of the frozen `COMMAND_REASON_*` values.
    pub reason: u32,
    /// Zero-based index of the first refused record, or `0` when the submission was admitted.
    pub rejected_index: u32,
    /// Number of records admitted by this submission: the whole batch, or zero.
    pub admitted: u32,
    /// Absolute sample at which the admitted records take effect.
    pub applied_at_sample: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
}

/// Byte size of [`WebCommandReport`].
pub const COMMAND_REPORT_BYTES: u32 = size_of::<WebCommandReport>() as u32;

/// The sample window and shape of the meter frame the `f32` buffer cannot carry (issue #143 D5).
///
/// # Why a second structure rather than more `f32`s
///
/// The meter frame is a `Float32Array` and the window it describes is a pair of absolute sample
/// counts. A `u64` does not survive an `f32`, and splitting one across two lanes would put a
/// decoding rule in the app that nothing could check. So the window rides a fixed structure the
/// JavaScript side reads through `miso_engine_web_v1_meter_header_ptr`, exactly as the status and
/// the resource report already do, and the `f32` buffer stays what it is: numbers a meter draws.
///
/// `first_sample`/`end_sample` are half-open and describe the window the frame's values were
/// folded over, so a consumer correlates them against a command's `applied_at_sample` rather than
/// against a wall clock.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebMeterHeader {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// Tracks the frame carries, so the `f32` view's `3T + 3` shape is checkable.
    pub track_count: u32,
    /// Complete windows folded by the most recent poll.
    pub windows: u32,
    /// Absolute sample the reported window opened at, inclusive.
    pub first_sample: u64,
    /// Absolute sample the reported window closed at, exclusive.
    pub end_sample: u64,
    /// Monotonic frame sequence, incremented once per posted frame.
    pub sequence: u64,
    /// Designated master track plus one, or `0` when none was designated (issue #143 D6).
    pub master_track_plus_one: u32,
    /// `1` when `master_gr_db` is meaningful, `0` when no master tap is bound.
    pub master_gr_present: u32,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
}

/// Byte size of [`WebMeterHeader`].
pub const METER_HEADER_BYTES: u32 = size_of::<WebMeterHeader>() as u32;

/// The header a handle with no compiled session reports: the zeroed shape, never a stale one.
static EMPTY_METER_HEADER: WebMeterHeader = empty_meter_header();

const fn empty_meter_header() -> WebMeterHeader {
    WebMeterHeader {
        struct_size: METER_HEADER_BYTES,
        abi_version: ABI_VERSION,
        track_count: 0,
        windows: 0,
        first_sample: 0,
        end_sample: 0,
        sequence: 0,
        master_track_plus_one: 0,
        master_gr_present: 0,
        reserved: [0; 2],
    }
}

/// Byte size of [`WebPrepareConfig`].
pub const PREPARE_CONFIG_BYTES: u32 = size_of::<WebPrepareConfig>() as u32;
/// Byte size of [`WebStatus`].
pub const STATUS_BYTES: u32 = size_of::<WebStatus>() as u32;
/// Byte size of [`WebResourceReport`].
pub const RESOURCE_REPORT_BYTES: u32 = size_of::<WebResourceReport>() as u32;

/// Exact versioned preparation configuration shared with JavaScript.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebPrepareConfig {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// Must equal [`ABI_VERSION`].
    pub abi_version: u32,
    /// Explicit browser context sample rate.
    pub sample_rate_hz: u32,
    /// Explicit caller-supplied render quantum.
    pub quantum_frames: u32,
    /// Session TOML staging capacity.
    pub session_toml_bytes: u32,
    /// Diagnostic staging capacity.
    pub diagnostic_bytes: u32,
    /// Source-ID staging capacity.
    pub source_id_bytes: u32,
    /// Maximum staged source plane count.
    pub maximum_source_channels: u32,
    /// Per-source bounded ring capacity in frames.
    pub source_ring_frames: u32,
    /// Maximum effect automation spans per render block.
    pub maximum_automation_spans_per_block: u32,
    /// Maximum tracks.
    pub maximum_tracks: u64,
    /// Maximum sources.
    pub maximum_sources: u64,
    /// Maximum routes.
    pub maximum_routes: u64,
    /// Maximum effects.
    pub maximum_effects: u64,
    /// Maximum graph session-plus-plan bytes.
    pub maximum_graph_session_plus_plan_bytes: u64,
    /// Maximum source-owned total bytes.
    pub maximum_source_total_bytes: u64,
    /// Maximum source overhead bytes.
    pub maximum_source_overhead_bytes: u64,
    /// Maximum scalar effect state bytes.
    pub maximum_effect_state_bytes: u64,
    /// Maximum scalar effect scratch bytes.
    pub maximum_effect_scratch_bytes: u64,
    /// Maximum retained builtin bytes.
    pub maximum_builtin_retained_bytes: u64,
    /// Maximum browser bridge retained bytes.
    pub maximum_host_retained_bytes: u64,
    /// Maximum single named allocation.
    pub maximum_named_allocation_bytes: u64,
    /// Maximum meter streams.
    pub maximum_meter_streams: u64,
    /// Maximum meter items.
    pub maximum_meter_items: u64,
    /// Maximum meter bytes.
    pub maximum_meter_bytes: u64,
    /// Per-track live-console control-queue depth in records, or `0` to attach no control channel
    /// and no command staging at all (issue #137 D1).
    ///
    /// Carved out of the frozen configuration's first reserved word, which every V1 writer already
    /// sets to zero. Zero is the honest form of "no live console": no queue is allocated, no
    /// staging buffer is allocated, the matrix processors keep the exact storage they had before
    /// this ABI existed, and a submission is refused with [`RESULT_UNSUPPORTED`]. The 192-byte
    /// layout is unchanged either way.
    pub console_command_queue_records: u64,
    /// Meter window in render blocks, or `0` to attach no meters at all (issue #137 D2).
    ///
    /// Zero is the honest form of "metering off costs nothing": no observer is bound, so the
    /// render path does not fold a single sample. A nonzero value binds one post-matrix meter per
    /// track with a `blocks * quantum_frames` window; the port lease then gates whether a finished
    /// window is posted. `12` is ~31 frames per second at 48 kHz with a 128-frame quantum.
    pub console_meter_blocks: u64,
    /// Maximum declared observation taps to bind per effect, or `0` for no observation capacity
    /// at all (issue #143 D3, level 1).
    ///
    /// Carved out of the frozen configuration's second reserved word, which every V1 writer
    /// already sets to zero — the same expansion `console_command_queue_records` took. Zero is the
    /// honest form: no lane, no accumulator and no conflating cell is allocated anywhere in the
    /// compiled plan, `observation_retained_bytes` is zero, and a subscription is refused with
    /// [`COMMAND_REASON_OBSERVATION_UNBOUND`]. The 192-byte layout is unchanged either way.
    ///
    /// Requires `console_command_queue_records != 0`: a subscription rides the effect's own
    /// command queue, so observation without a console has no delivery path.
    pub console_observation_taps: u64,
    /// The designated master track, **plus one**, or `0` for none (issue #143 D6).
    ///
    /// V1 has no structural master bus, so `masterGrDb` is a designation rather than a discovery.
    /// Plus one because zero has to keep meaning "unset" in a word every V1 writer already zeroes.
    pub console_master_track_plus_one: u64,
}

impl WebPrepareConfig {
    /// A bounded launch configuration suitable for small embedding tests and examples.
    #[must_use]
    pub const fn launch_defaults(sample_rate_hz: u32, quantum_frames: u32) -> Self {
        Self {
            struct_size: PREPARE_CONFIG_BYTES,
            abi_version: ABI_VERSION,
            sample_rate_hz,
            quantum_frames,
            session_toml_bytes: 1 << 20,
            diagnostic_bytes: 1 << 14,
            source_id_bytes: 1 << 10,
            maximum_source_channels: 8,
            source_ring_frames: default_source_ring_frames(sample_rate_hz, quantum_frames),
            maximum_automation_spans_per_block: 256,
            maximum_tracks: 1_024,
            maximum_sources: 1_024,
            maximum_routes: 4_096,
            maximum_effects: 8_192,
            maximum_graph_session_plus_plan_bytes: 64 << 20,
            maximum_source_total_bytes: 64 << 20,
            maximum_source_overhead_bytes: 16 << 20,
            maximum_effect_state_bytes: 16 << 20,
            maximum_effect_scratch_bytes: 16 << 20,
            maximum_builtin_retained_bytes: 64 << 20,
            maximum_host_retained_bytes: 16 << 20,
            maximum_named_allocation_bytes: 64 << 20,
            maximum_meter_streams: 1_024,
            maximum_meter_items: 1 << 20,
            maximum_meter_bytes: 16 << 20,
            console_command_queue_records: 0,
            console_meter_blocks: 0,
            console_observation_taps: 0,
            console_master_track_plus_one: 0,
        }
    }

    /// The launch defaults with the live web console attached (issue #137 D1/D2).
    #[must_use]
    pub const fn console_defaults(sample_rate_hz: u32, quantum_frames: u32) -> Self {
        Self {
            console_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
            console_meter_blocks: DEFAULT_METER_BLOCKS as u64,
            ..Self::launch_defaults(sample_rate_hz, quantum_frames)
        }
    }
}

/// Fixed browser-visible status snapshot.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebStatus {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// One of the frozen `STATE_*` values.
    pub state: u32,
    /// Most recent frozen result code.
    pub last_result: u32,
    /// Selected backend.
    pub backend: u32,
    /// Prepared sample rate.
    pub sample_rate_hz: u32,
    /// Prepared quantum.
    pub quantum_frames: u32,
    /// Required zero.
    pub reserved0: u32,
    /// Exact next render time.
    pub next_absolute_sample: u64,
    /// Number of successful rendered quanta.
    pub rendered_quanta: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

/// Exact retained-resource projection for the browser bridge and production artifacts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebResourceReport {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// Prepared sample rate.
    pub sample_rate_hz: u32,
    /// Prepared render quantum.
    pub quantum_frames: u32,
    /// Selected backend.
    pub backend: u32,
    /// Required zero words.
    pub reserved0: [u32; 3],
    /// Config allocation bytes.
    pub config_bytes: u64,
    /// Status allocation bytes.
    pub status_bytes: u64,
    /// Session TOML staging bytes.
    pub session_toml_bytes: u64,
    /// Diagnostic bytes.
    pub diagnostic_bytes: u64,
    /// Source-ID staging bytes.
    pub source_id_bytes: u64,
    /// Source PCM staging bytes.
    pub source_pcm_staging_bytes: u64,
    /// Output PCM bytes.
    pub output_pcm_bytes: u64,
    /// Browser bridge metadata bytes.
    pub bridge_metadata_bytes: u64,
    /// Total retained browser bridge bytes.
    pub bridge_retained_bytes: u64,
    /// Largest browser bridge allocation.
    pub largest_bridge_allocation_bytes: u64,
    /// Production source-owned total bytes.
    pub source_total_bytes: u64,
    /// Production source overhead bytes.
    pub source_overhead_bytes: u64,
    /// Scalar effect state bytes.
    pub effect_scalar_state_bytes: u64,
    /// Scalar effect scratch bytes.
    pub effect_scalar_scratch_bytes: u64,
    /// Retained builtin bytes.
    pub builtin_retained_bytes: u64,
    /// Graph session-plus-plan bytes.
    pub graph_session_plus_plan_bytes: u64,
    /// Incremental plan bytes.
    pub graph_incremental_plan_bytes: u64,
    /// Graph metadata bytes.
    pub graph_metadata_bytes: u64,
    /// Graph delay bytes.
    pub graph_delay_bytes: u64,
    /// Largest named production or bridge allocation.
    pub largest_named_allocation_bytes: u64,
    /// Engine-owned bytes the plan's observation lanes and conflating cells retain (issue #143).
    ///
    /// Carved out of the report's first reserved word. Exactly zero for a session prepared with
    /// `console_observation_taps == 0`, and that zero is *walked* over the built runtime rather
    /// than computed from the configuration. The 224-byte layout is unchanged.
    pub observation_retained_bytes: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 3],
}

struct PreparedBuffers {
    session_toml: Box<[u8]>,
    diagnostic: Box<[u8]>,
    source_id: Box<[u8]>,
    source_pcm: Box<[f32]>,
    output_pcm: Box<[f32]>,
    /// Fixed `MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES` live-console staging (issue #137 D1).
    command: Box<[u8]>,
    plane_references: Box<[MaybeUninit<&'static [f32]>]>,
}

type FfiSourceStaging<'a> = (
    &'a mut [f32],
    &'a mut [MaybeUninit<&'static [f32]>],
    &'a mut [u8],
);

/// Everything one compiled session owns on the browser side.
///
/// Field order is the drop order and is load-bearing: [`PreparedHost`] drops its plan (which owns
/// the source consumers) before its control-side producers, and the compiled session model outlives
/// both. Nothing here is ever dropped from `render_next`; see [`AudioWorkletEngineHost::fail`].
struct ReadyOwnership {
    /// Issue #137 D1 / #140 B: per-track control-side producers -- matrix/pan and fader/mute --
    /// declared first so they are released before the plan that owns their consumer endpoints.
    controls: Vec<TrackControlProducer>,
    /// Issue #140 A: one control-side producer per prepared effect instance, in the dense
    /// `queue_slot` order [`ReadyOwnership::effect_slot`] computes, so an addressed command
    /// reaches its queue with one index and no search.
    effect_controls: Box<[Option<EffectControlProducer>]>,
    /// Per-track prefix sum of effect instances, so `effect_slot` is arithmetic, not a lookup.
    effect_base: Box<[u32]>,
    /// Room needed per destination queue by the submission being validated. Allocated at
    /// compilation, one entry per queue (see [`ReadyOwnership::effect_slot`]).
    command_wanted: Box<[u32]>,
    /// Decoded submission staging. Two entries per staged record, because one wire record
    /// addressed to `channel = both` on a per-lane effect parameter lowers to one span per lane,
    /// plus `2 * track_count` for the coalesced solo emission (issue #210 phase 1).
    command_decoded: Box<[(u32, AdmittedCommand)]>,
    /// Issue #210 phase 1: the console's solo bits and its mirrors of user mute and of what the
    /// render plane was last told. Solo composes into the *existing* mute records at admission and
    /// adds nothing below it, so this is the whole of solo-in-place on the host side.
    solo: ConsoleSoloState,
    /// Records admitted per destination queue since the last successful render.
    ///
    /// The browser's control plane and render plane are the same thread and every console stage
    /// drains its whole queue at the top of every block, so a successful render empties every
    /// control queue. That is what makes this an exact free-slot count rather than an estimate,
    /// and it is what lets a submission be refused *before* anything is pushed.
    in_flight: Box<[u32]>,
    /// Canonical normalized track order: the addressing authority for `track_index`.
    tracks: Vec<Box<str>>,
    /// Effects declared per track per rack, `[simd1, dynamic, simd2]`, so an effect-addressed
    /// command is answered with `UNKNOWN_RACK` / `UNKNOWN_EFFECT` before anything else.
    rack_effects: Box<[[u32; 3]]>,
    host: PreparedHost,
    /// Issue #137 D2: meter consumers, declared after the plan that owns their producers. Empty
    /// when `console_meter_blocks` was zero, in which case no observer exists at all.
    meters: Vec<MeterConsumer>,
    /// Issue #143: one reader set per observed effect instance, in the same dense `effect_slot`
    /// order the command producers use, so an addressed subscription reaches its lane with one
    /// index and no search. Empty when the configuration named no observation capacity.
    effect_observations: Box<[Option<EffectObservationHandle>]>,
    /// Track index of each observed effect slot, or `u32::MAX` for a slot with no taps. Built once
    /// at compilation, so the poll's fold is arithmetic rather than a search.
    observation_tracks: Box<[u32]>,
    /// Whether each track contributed a gain-reduction reading to the most recent poll. Allocated
    /// at compilation; it is what makes `masterGrDb` absent rather than zero.
    observation_present: Box<[bool]>,
    /// Armed-tap bitmask per effect slot, maintained at **admission**.
    ///
    /// A conflating cell holds its last window forever -- that is what makes it wait-free -- so
    /// "has this tap been unsubscribed" is not a question the transport can answer, and it must
    /// not be: a reader that treated an unconsumed window as absent would make an armed tap's
    /// reading flicker to zero on every poll between windows, exactly as the peak section would if
    /// it forgot its last value. The subscription state is control-plane state, so it lives on the
    /// control plane, updated by the one function that admits the record.
    observation_armed: Box<[u32]>,
    /// The designated master track, or `None` (issue #143 D6).
    master_track: Option<u32>,
    /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R, track0 GR, .., trackN GR,
    /// master GR]` -- `3T + 3` words. The peak section is byte-for-byte where it always was.
    meter_frame: Box<[f32]>,
    /// The window and shape the `f32` frame cannot carry (issue #143 D5).
    meter_header: WebMeterHeader,
    /// Master peaks folded over the rendered block while the lease is on.
    master_peak: [f32; 2],
    /// Windows folded into `meter_frame` since the host was compiled.
    meter_windows: u64,
    /// The compiled session model, retained so the browser bridge keeps charging itself for what
    /// it holds -- and, since issue #207, read by the source-introspection queries.
    ///
    /// The queries read it directly rather than copying a parallel table out of it, so the order
    /// they report *is* the normalized model's order by construction: `compile_session` sorts
    /// `sources` by stable ID, and there is no second list that could disagree with it.
    session: CompiledSession,
}

impl ReadyOwnership {
    /// The dense destination-queue index of one addressed console channel.
    ///
    /// The layout is frozen here and nowhere else:
    ///
    /// | range | queue |
    /// |---|---|
    /// | `0 .. tracks` | track `t`'s matrix/pan queue |
    /// | `tracks .. 2 * tracks` | track `t`'s fader/mute queue |
    /// | `2 * tracks .. 3 * tracks` | track `t`'s input trim/polarity queue (#210 phase 3) |
    /// | `3 * tracks ..` | effect instances, in `(track, simd1, dynamic, simd2, position)` order |
    ///
    /// One index therefore serves both the free-room pre-check and the push, and neither pass has
    /// to search. `None` means the address names no channel this session prepared.
    ///
    /// The input band was **appended** to the per-track prefix rather than inserted, so the two
    /// existing bands keep their indices and the only arithmetic that moved is the effect base's
    /// `2 * tracks` -> `3 * tracks`. Every site that spells that constant is below and in
    /// [`Self::queue_capacity`], [`Self::push`] and the `queue_count` allocation; there is no
    /// fourth spelling.
    fn effect_slot(&self, track: usize, rack: u8, effect_index: u32) -> Option<usize> {
        let base = *self.effect_base.get(track)? as usize;
        let counts = self.rack_effects.get(track)?;
        let mut offset = 0_usize;
        for earlier in 0..rack as usize {
            offset = offset.checked_add(*counts.get(earlier)? as usize)?;
        }
        if effect_index >= *counts.get(rack as usize)? {
            return None;
        }
        let slot = base
            .checked_add(offset)?
            .checked_add(effect_index as usize)?;
        (slot < self.effect_controls.len()).then_some(slot)
    }

    /// Free room in one destination queue, or `None` when the queue does not exist.
    fn queue_capacity(&self, slot: usize) -> Option<u32> {
        let tracks = self.tracks.len();
        if slot < tracks {
            let producer = self.controls.get(slot)?;
            return u32::try_from(producer.producer.capacity()).ok();
        }
        if slot < tracks * 2 {
            let producer = self.controls.get(slot - tracks)?;
            return u32::try_from(producer.fader.capacity()).ok();
        }
        if slot < tracks * 3 {
            let producer = self.controls.get(slot - tracks * 2)?;
            return u32::try_from(producer.input.capacity()).ok();
        }
        let producer = self.effect_controls.get(slot - tracks * 3)?.as_ref()?;
        u32::try_from(producer.producer.capacity()).ok()
    }

    /// Push one admitted record into its destination queue. `Err` only on a full queue, which the
    /// free-room pass has already ruled out.
    fn push(&mut self, slot: usize, record: AdmittedCommand) -> Result<(), ()> {
        let tracks = self.tracks.len();
        match record {
            AdmittedCommand::Matrix(record) => {
                let producer = self.controls.get_mut(slot).ok_or(())?;
                producer.producer.try_push(record).map_err(|_| ())
            }
            AdmittedCommand::Fader(record) => {
                let producer = self.controls.get_mut(slot - tracks).ok_or(())?;
                producer.fader.try_push(record).map_err(|_| ())
            }
            AdmittedCommand::Input(record) => {
                let producer = self.controls.get_mut(slot - tracks * 2).ok_or(())?;
                producer.input.try_push(record).map_err(|_| ())
            }
            AdmittedCommand::Effect(record) => {
                let effect = slot - tracks * 3;
                // Issue #143: the armed set is control-plane state and is updated here, where the
                // record is admitted, so it can never disagree with what the render side was told.
                if let EffectControlRecord::Observe {
                    tap_index, armed, ..
                } = record
                    && let Some(mask) = self.observation_armed.get_mut(effect)
                    && tap_index < u32::BITS
                {
                    let bit = 1_u32 << tap_index;
                    *mask = if armed { *mask | bit } else { *mask & !bit };
                }
                let producer = self
                    .effect_controls
                    .get_mut(effect)
                    .ok_or(())?
                    .as_mut()
                    .ok_or(())?;
                producer.producer.try_push(record).map_err(|_| ())
            }
        }
    }
}

/// One decoded record and the payload its destination queue takes (issue #140 C).
#[derive(Clone, Copy)]
enum AdmittedCommand {
    /// The matrix/pan channel #137 D1 shipped.
    Matrix(TrackControlRecordV1),
    /// The fader/mute channel (#140 B).
    Fader(TrackFaderRecordV1),
    /// The input trim/polarity channel (#210 phase 3).
    Input(TrackInputRecordV1),
    /// One effect instance's channel (#140 A).
    Effect(EffectControlRecord),
}

/// One compiled source's declared shape, in canonical normalized source order (issue #207).
///
/// This is exactly what the *compiled session* knows about a source and nothing more: the strict
/// TOML declares a native rate, a channel count and an inclusive region, `compile_session`
/// normalizes it, and preparation builds the source ring at `region_start_frame` with a region
/// ending at `region_start_frame + region_frames`. A headless driver that must feed the render
/// loop needs precisely these four numbers, because the ring it is feeding was positioned by them.
///
/// `sample_rate_hz` is declared per source and is therefore reported per source, but a *compiled*
/// session's per-source rate necessarily equals the session rate: `prepare_host_session` refuses
/// `host.source.rate.mismatch` before a plan exists, because V1 has no sample-rate conversion. It
/// is reported so a consumer reads the declaration rather than inferring it from that invariant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceShape {
    /// Declared source channels; nonzero for every source a compiled session holds.
    pub channel_count: u32,
    /// Declared native source sample rate in hertz; equal to the session rate once compiled.
    pub sample_rate_hz: u32,
    /// First source sample frame of the declared region. Zero is an ordinary value.
    pub region_start_frame: u64,
    /// Length of the declared region in source sample frames; nonzero for every compiled source.
    pub region_frames: u64,
}

/// Safe ownership object backing one future AudioWorklet Wasm handle.
pub struct AudioWorkletEngineHost {
    config: WebPrepareConfig,
    status: WebStatus,
    resources: WebResourceReport,
    command_report: WebCommandReport,
    /// Issue #137 D2: the meter lease. `false` skips the master fold and every drain.
    meter_lease: bool,
    buffers: Option<PreparedBuffers>,
    ready: Option<ReadyOwnership>,
    diagnostic_len: usize,
}

impl AudioWorkletEngineHost {
    /// Create a configuration-only host. No session or staging storage is allocated.
    #[must_use]
    pub const fn new(config: WebPrepareConfig) -> Self {
        let backend = selected_backend();
        Self {
            config,
            status: WebStatus {
                struct_size: STATUS_BYTES,
                abi_version: ABI_VERSION,
                state: STATE_CONFIG,
                last_result: RESULT_OK,
                backend,
                sample_rate_hz: 0,
                quantum_frames: 0,
                reserved0: 0,
                next_absolute_sample: 0,
                rendered_quanta: 0,
                reserved: [0; 4],
            },
            resources: empty_resource_report(backend),
            command_report: empty_command_report(),
            meter_lease: false,
            buffers: None,
            ready: None,
            diagnostic_len: 0,
        }
    }

    /// Read the immutable preparation configuration.
    #[must_use]
    pub const fn config(&self) -> &WebPrepareConfig {
        &self.config
    }

    /// Mutably borrow configuration storage before preparation.
    pub fn config_mut(&mut self) -> Option<&mut WebPrepareConfig> {
        (self.status.state == STATE_CONFIG).then_some(&mut self.config)
    }

    /// Read fixed status without allocation.
    #[must_use]
    pub const fn status(&self) -> &WebStatus {
        &self.status
    }

    /// Read the current exact resource projection.
    #[must_use]
    pub const fn resources(&self) -> &WebResourceReport {
        &self.resources
    }

    /// Read the last live-console submission report (issue #137 D1).
    #[must_use]
    pub const fn command_report(&self) -> &WebCommandReport {
        &self.command_report
    }

    /// Canonical normalized track order, the addressing authority for `track_index`.
    #[must_use]
    pub fn console_tracks(&self) -> &[Box<str>] {
        self.ready.as_ref().map_or(&[], |ready| &ready.tracks)
    }

    /// Number of sources the compiled session declares; zero before compilation (issue #207).
    ///
    /// This is the bounds authority for every other source query, exactly as
    /// [`AudioWorkletEngineHost::console_tracks`] is for `track_index`: the shape queries report a
    /// sentinel for an out-of-range index where they can, but `region_start_frame` has no spare
    /// value, so a caller establishes the range here and then indexes inside it.
    #[must_use]
    pub fn session_source_count(&self) -> usize {
        self.ready
            .as_ref()
            .map_or(0, |ready| ready.session.normalized_model().sources.len())
    }

    /// One canonical source ID, or `None` for an out-of-range index (issue #207).
    ///
    /// The order is the normalized model's: `compile_session` sorts `sources` by stable ID, and
    /// this reads that list rather than a copy of it, so "canonical source order" has exactly one
    /// definition in this crate.
    #[must_use]
    pub fn session_source_id(&self, index: u32) -> Option<&str> {
        let ready = self.ready.as_ref()?;
        let source = ready
            .session
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(source.id.as_str())
    }

    /// One source's declared shape, or `None` for an out-of-range index (issue #207).
    #[must_use]
    pub fn session_source_shape(&self, index: u32) -> Option<SessionSourceShape> {
        let ready = self.ready.as_ref()?;
        let source = ready
            .session
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(SessionSourceShape {
            channel_count: u32::from(source.mapping.channel_count),
            sample_rate_hz: source.sample_rate_hz,
            region_start_frame: source.mapping.region.start_sample,
            region_frames: source.mapping.region.length_samples,
        })
    }

    /// The decimated meter frame (issue #137 D2, extended by #143 D5).
    ///
    /// `3T + 3` words: two peaks per track, master left and right, then one **non-negative**
    /// gain-reduction magnitude in decibels per track and the master's. The peak section is
    /// byte-for-byte where it always was.
    #[must_use]
    pub fn meter_frame(&self) -> &[f32] {
        self.ready.as_ref().map_or(&[], |ready| &ready.meter_frame)
    }

    /// The sample window and shape the `f32` frame cannot carry (issue #143 D5).
    #[must_use]
    pub fn meter_header(&self) -> &WebMeterHeader {
        self.ready
            .as_ref()
            .map_or(&EMPTY_METER_HEADER, |ready| &ready.meter_header)
    }

    /// Armed taps, in the dense effect-slot order. Off-ABI introspection for the tests.
    #[must_use]
    pub fn observation_armed_taps(&self) -> u32 {
        self.ready.as_ref().map_or(0, |ready| {
            ready
                .observation_armed
                .iter()
                .map(|mask| mask.count_ones())
                .sum()
        })
    }

    /// Whether preparation bound any observation taps at all (issue #143 D3, level 1).
    #[must_use]
    pub fn observation_attached(&self) -> bool {
        self.ready.as_ref().is_some_and(|ready| {
            ready
                .effect_observations
                .iter()
                .any(std::option::Option::is_some)
        })
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.ready.as_ref().map_or(0, |ready| ready.meter_windows)
    }

    /// Whether meter observers were attached at preparation (issue #137 D2).
    #[must_use]
    pub fn meters_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| !ready.meters.is_empty())
    }

    /// Take or release the meter lease. Returns the frozen result code.
    ///
    /// A lease over a host that attached no observers is refused with [`RESULT_UNSUPPORTED`]: the
    /// caller asked for numbers this preparation cannot produce, and silently reporting zeros
    /// would be worse than saying so.
    pub fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if enabled && !self.meters_attached() {
            return self.record(RESULT_UNSUPPORTED);
        }
        self.meter_lease = enabled;
        if let Some(ready) = self.ready.as_mut() {
            ready.master_peak = [0.0, 0.0];
            ready.meter_frame.fill(0.0);
            // Issue #143 E8: a retaken lease restarts the frame sequence and the reported window,
            // so a consumer never folds a window from before the release into one after it.
            ready.meter_header.sequence = 0;
            ready.meter_header.windows = 0;
            ready.meter_header.first_sample = 0;
            ready.meter_header.end_sample = 0;
            ready.meter_header.master_gr_present = 0;
        }
        self.record(RESULT_OK)
    }

    /// Allocate and publish every fixed staging buffer transactionally.
    pub fn prepare(&mut self) -> u32 {
        if self.status.state != STATE_CONFIG {
            return self.record(RESULT_WRONG_STATE);
        }
        let result = prepare_buffers(self.config);
        let (buffers, report) = match result {
            Ok(value) => value,
            Err(code) => return self.record(code),
        };
        self.buffers = Some(buffers);
        self.resources = report;
        self.status.state = STATE_PREPARED;
        self.status.sample_rate_hz = self.config.sample_rate_hz;
        self.status.quantum_frames = self.config.quantum_frames;
        self.record(RESULT_OK)
    }

    /// Mutable session TOML staging storage, available only after preparation.
    pub fn session_toml_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.session_toml)
    }

    /// Mutable source-ID staging storage, available only after preparation.
    pub fn source_id_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.source_id)
    }

    /// Mutable contiguous planar source PCM staging storage.
    pub fn source_pcm_mut(&mut self) -> Option<&mut [f32]> {
        self.buffers.as_mut().map(|value| &mut *value.source_pcm)
    }

    /// The valid fixed diagnostic prefix.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        self.buffers
            .as_ref()
            .map_or(&[], |value| &value.diagnostic[..self.diagnostic_len])
    }

    /// Contiguous dual-mono render output, left plane then right plane.
    #[must_use]
    pub fn output_pcm(&self) -> Option<&[f32]> {
        self.buffers.as_ref().map(|value| &*value.output_pcm)
    }

    /// Mutable live-console command staging, or `None` when no console was attached.
    ///
    /// This is the buffer the JavaScript side writes records into through
    /// `miso_engine_web_v1_buffer_ptr(handle, BUFFER_COMMAND)`; it is public so an embedding that
    /// drives the safe host directly -- the native parity twin, the metadata round-trip gate --
    /// stages exactly the bytes the browser stages.
    pub fn command_staging_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers
            .as_mut()
            .map(|value| &mut *value.command)
            .filter(|value| !value.is_empty())
    }

    /// Exact byte size of the live-console command staging buffer; zero when none was attached.
    #[must_use]
    pub fn command_staging_bytes(&self) -> u64 {
        self.buffers
            .as_ref()
            .map_or(0, |value| value.command.len() as u64)
    }

    /// Whether preparation attached a live-console control channel (issue #137 D1).
    #[must_use]
    pub fn console_attached(&self) -> bool {
        self.ready
            .as_ref()
            .is_some_and(|ready| !ready.controls.is_empty())
    }

    /// Copy one canonical console track ID into source-ID staging; returns its byte length.
    pub(crate) fn copy_console_track_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(id) = ready.tracks.get(index as usize) else {
            return 0;
        };
        let bytes = id.as_bytes();
        let length = bytes.len();
        let Some(buffers) = self.buffers.as_mut() else {
            return 0;
        };
        let Some(target) = buffers.source_id.get_mut(..length) else {
            return 0;
        };
        target.copy_from_slice(bytes);
        u32::try_from(length).unwrap_or(0)
    }

    /// Copy one canonical source ID into source-ID staging; returns its byte length (issue #207).
    ///
    /// The twin of [`AudioWorkletEngineHost::copy_console_track_id`], and it cannot fail on
    /// capacity the way that one can: `compile_ready` refuses `web.source.id.capacity` unless
    /// every source ID fits this exact buffer, so the only zero this returns is "no such source".
    pub(crate) fn copy_session_source_id(&mut self, index: u32) -> u32 {
        let Some(ready) = self.ready.as_ref() else {
            return 0;
        };
        let Some(source) = ready.session.normalized_model().sources.get(index as usize) else {
            return 0;
        };
        let bytes = source.id.as_str().as_bytes();
        let length = bytes.len();
        let Some(buffers) = self.buffers.as_mut() else {
            return 0;
        };
        let Some(target) = buffers.source_id.get_mut(..length) else {
            return 0;
        };
        target.copy_from_slice(bytes);
        u32::try_from(length).unwrap_or(0)
    }

    pub(crate) fn diagnostic_buffer_mut(&mut self) -> Option<&mut [u8]> {
        self.buffers.as_mut().map(|value| &mut *value.diagnostic)
    }

    pub(crate) fn ffi_source_staging_mut(&mut self) -> Option<FfiSourceStaging<'_>> {
        self.buffers.as_mut().map(|value| {
            (
                &mut *value.source_pcm,
                &mut *value.plane_references,
                &mut *value.source_id,
            )
        })
    }

    /// Parse, compile and atomically publish one immutable session and plan.
    pub fn compile(&mut self, toml_bytes: usize) -> u32 {
        if self.status.state != STATE_PREPARED {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(buffers) = self.buffers.as_ref() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let Some(bytes) = buffers.session_toml.get(..toml_bytes) else {
            return self.record(RESULT_BUFFER_TOO_SMALL);
        };
        let toml = match core::str::from_utf8(bytes) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INVALID_ARGUMENT, b"web.toml.utf8\t$\n"),
        };
        match compile_ready(toml, self.config, self.resources) {
            Ok((ready, resources)) => {
                self.ready = Some(ready);
                self.resources = resources;
                self.diagnostic_len = 0;
                self.status.state = STATE_READY;
                self.record(RESULT_OK)
            }
            Err(failure) => self.fail(RESULT_PREPARE_REJECTED, &failure),
        }
    }

    /// Submit one generation-tagged, exact-rate borrowed planar source chunk.
    ///
    /// The host owns two rules: the `STATE_READY` gate, and the staging-shaped bound
    /// `frames <= quantum_frames` (the JavaScript side copies into a one-quantum staging plane, so
    /// a longer chunk could not have been staged). Everything else -- rate, channel count, region
    /// bounds, end-of-region symmetry, generation `0`, ring backpressure -- is the facade's, in the
    /// facade's fixed order, so the browser host and the C ABI host can no longer disagree about
    /// what a rejection is. Generation `0` is now rejected here too; before #106 F1 only the C ABI
    /// copy caught it.
    #[allow(clippy::too_many_arguments)]
    pub fn submit_source(
        &mut self,
        source_id: &[u8],
        generation: u64,
        start_frame: u64,
        sample_rate_hz: u32,
        planes: &[&[f32]],
        frames: u32,
        end_of_region: bool,
    ) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if frames > self.config.quantum_frames {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let result = ready.host.sources.submit(
            source_id,
            SourceSubmission {
                generation,
                start_frame,
                sample_rate_hz,
                planes,
                frames,
                end_of_region,
            },
        );
        let code = match result {
            Ok(_) => RESULT_OK,
            Err(error) => source_result(error),
        };
        self.record(code)
    }

    /// Queue one strictly increasing generation-tagged absolute source seek.
    pub fn seek_source(&mut self, source_id: &[u8], generation: u64, source_frame: u64) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let code = match ready.host.sources.seek(source_id, generation, source_frame) {
            Ok(()) => RESULT_OK,
            Err(error) => source_result(error),
        };
        self.record(code)
    }

    /// Render exactly one prepared quantum at the next absolute sample time.
    pub fn render_next(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        // Lossless on every supported target (wasm32 and every 64-bit host); `try_from` would
        // leave an `expect_failed` trap owner in the render export's call graph.
        let quantum = self.config.quantum_frames as usize;
        let Some(ready) = self.ready.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let output = match PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INTERNAL, b"web.internal.output\t$\n"),
        };
        let result = ready.host.plan.render(
            RenderIo {
                input: None,
                output,
            },
            RenderTime {
                absolute_sample: self.status.next_absolute_sample,
            },
        );
        match result {
            Ok(report) => {
                self.status.next_absolute_sample = report.next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
                // Issue #137 D1: the matrix stage drained every control queue at the top of this
                // block, so the exact free-slot count is the whole capacity again.
                ready.in_flight.fill(0);
                // Issue #137 D2: the master bus is the host's own output plane, so there is
                // nothing to observe and nothing to expose -- one branch and one pass over a
                // buffer already in cache, and only while the lease is held.
                if self.meter_lease {
                    // Indexed with `get`, never with a range: a slice index would put
                    // `slice_index_fail` in the render export's call graph, and the shipped
                    // artifact's gate would -- correctly -- refuse the build.
                    let mut peaks = ready.master_peak;
                    for (plane, peak) in peaks.iter_mut().enumerate() {
                        let start = plane * quantum;
                        let Some(samples) = buffers
                            .output_pcm
                            .get(start..)
                            .and_then(|tail| tail.get(..quantum))
                        else {
                            break;
                        };
                        for sample in samples {
                            let magnitude = sample.abs();
                            if magnitude > *peak {
                                *peak = magnitude;
                            }
                        }
                    }
                    ready.master_peak = peaks;
                }
                self.record(RESULT_OK)
            }
            Err(_) => self.fail(RESULT_RENDER_REJECTED, b"web.render.rejected\t$\n"),
        }
    }

    /// Admit one staged `miso.command.v1` submission as a single transaction (issue #137 D1).
    ///
    /// # The two-pass shape is the contract, not an optimisation
    ///
    /// Pass one validates every record -- shape, addressing, domain, and free queue room -- and
    /// pushes nothing. Pass two pushes. A submission is therefore all-or-nothing: eval E4's
    /// "engine state untouched" is a property of the code, not of a lucky ordering, and eval E3's
    /// flood is refused before a single record reaches a queue.
    ///
    /// # Why this is not on the render thread's critical path
    ///
    /// This runs from `port.onmessage`, which the user agent dispatches between render quanta on
    /// the same thread as `process()`. It performs no allocation, no compilation and no plan
    /// replacement: both staging arrays were allocated at compilation, and the call moves at most
    /// [`MAXIMUM_COMMAND_RECORDS`] `Copy` records into bounded per-track queues. The block that
    /// follows drains them.
    pub fn submit_commands(&mut self, count: u32) -> u32 {
        self.command_report = empty_command_report();
        if self.status.state != STATE_READY {
            return self.finish_commands(RESULT_WRONG_STATE, COMMAND_REASON_WRONG_STATE, 0, 0);
        }
        if count > MAXIMUM_COMMAND_RECORDS {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        }
        if !self.console_attached() {
            return self.finish_commands(RESULT_UNSUPPORTED, COMMAND_REASON_UNSUPPORTED_KIND, 0, 0);
        }
        let applied_at_sample = self.status.next_absolute_sample;
        let staged = count as usize * COMMAND_RECORD_BYTES as usize;
        if self.buffers.is_none() {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        }
        if self.ready.is_none() {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        }
        // Disjoint borrows: the staged bytes live in `buffers`, the console lives in `ready`.
        let Some((buffers, ready)) = self.buffers.as_ref().zip(self.ready.as_mut()) else {
            return self.fail(RESULT_INTERNAL, b"web.internal.console\t$\n");
        };
        let Some(bytes) = buffers.command.get(..staged) else {
            return self.finish_commands(RESULT_INVALID_ARGUMENT, COMMAND_REASON_MALFORMED, 0, 0);
        };
        match admit_commands(ready, bytes, count as usize) {
            Ok(()) => {
                self.command_report.applied_at_sample = applied_at_sample;
                self.finish_commands(RESULT_OK, COMMAND_REASON_NONE, 0, count)
            }
            Err(CommandRejection {
                result,
                reason,
                index,
            }) => self.finish_commands(result, reason, index, 0),
        }
    }

    /// Entries in the decode staging array, for the issue #210 phase 1 sizing pin.
    #[cfg(test)]
    pub(crate) fn command_staging_entries(&self) -> Option<usize> {
        self.ready.as_ref().map(|ready| ready.command_decoded.len())
    }

    /// The console's solo state, for the issue #210 phase 1 evals.
    ///
    /// The ABI has no readback of it on purpose -- solo is control-plane state and the app is the
    /// one that issued every gesture that moved it -- so this exists only for the tests that have
    /// to prove the mirror agrees with the session it was prepared from.
    #[cfg(test)]
    pub(crate) fn console_solo(&self) -> Option<&ConsoleSoloState> {
        self.ready.as_ref().map(|ready| &ready.solo)
    }

    /// Drain every finished meter window into the frame buffer (issue #137 D2).
    ///
    /// Returns the number of complete windows folded by this call. Zero work happens without the
    /// lease, and a host prepared with `console_meter_blocks == 0` has no observer to drain.
    ///
    /// Allocation-free by construction: it moves `Copy` snapshots out of bounded queues into a
    /// buffer allocated at compilation.
    pub fn poll_meters(&mut self) -> u32 {
        if !self.meter_lease || self.status.state != STATE_READY {
            return 0;
        }
        let Some(ready) = self.ready.as_mut() else {
            return 0;
        };
        // Every index below is a `get_mut`, never a `[]`: a bounds check would put
        // `panic_bounds_check` in this export's call graph, and this export is called from
        // `process()`, so the shipped artifact's gate covers it exactly as it covers the render
        // export.
        let mut windows = 0_u32;
        for (index, meter) in ready.meters.iter_mut().enumerate() {
            let mut popped = 0_u32;
            while let Ok(snapshot) = meter.consumer.try_pop() {
                let MeterSnapshot { left, right, .. } = snapshot;
                if let Some(slot) = ready.meter_frame.get_mut(index * 2) {
                    *slot = left.sample_peak;
                }
                if let Some(slot) = ready.meter_frame.get_mut(index * 2 + 1) {
                    *slot = right.sample_peak;
                }
                popped = popped.saturating_add(1);
            }
            // Every track's meter shares one window length and one start sample, so they finish
            // together; the minimum is the number of windows the whole frame actually covers.
            windows = if index == 0 {
                popped
            } else {
                windows.min(popped)
            };
        }
        let tracks = ready.tracks.len();
        let master = tracks * 2;
        for (plane, peak) in ready.master_peak.iter().enumerate() {
            if let Some(slot) = ready.meter_frame.get_mut(master + plane) {
                *slot = *peak;
            }
        }
        ready.master_peak = [0.0, 0.0];

        // Issue #143 D5: the gain-reduction section. This is the control plane -- `poll_meters` is
        // called from `process()` but after the render export, and it performs no allocation, no
        // lock and no unbounded loop -- so the one unit conversion the whole design permits lives
        // here: a tap that publishes a **linear** magnitude (the true-peak limiter's recursive
        // reduction word) becomes decibels once per closed window, never per sample and never on a
        // lane kernel.
        let gain_base = tracks * 2 + 2;
        for slot in ready.meter_frame.iter_mut().skip(gain_base) {
            *slot = 0.0;
        }
        for slot in ready.observation_present.iter_mut() {
            *slot = false;
        }
        let mut first_sample = u64::MAX;
        let mut end_sample = 0_u64;
        let mut observed_any = false;
        for (effect, entry) in ready.effect_observations.iter().enumerate() {
            let Some(handle) = entry.as_ref() else {
                continue;
            };
            let Some(track) = ready.observation_tracks.get(effect).copied() else {
                continue;
            };
            if track == u32::MAX {
                continue;
            }
            let armed = ready.observation_armed.get(effect).copied().unwrap_or(0);
            if armed == 0 {
                continue;
            }
            for (tap_index, reader) in handle.readers.iter().enumerate() {
                if u32::try_from(tap_index)
                    .ok()
                    .is_none_or(|index| index >= u32::BITS || armed & (1_u32 << index) == 0)
                {
                    continue;
                }
                let Some(window) = reader.read() else {
                    continue;
                };
                reader.acknowledge(window.sequence);
                let Some(tap) = handle.descriptor.observations.get(tap_index) else {
                    continue;
                };
                let magnitude = if window.left > window.right {
                    window.left
                } else {
                    window.right
                };
                let value = observed_decibels(*tap, magnitude);
                // Several armed taps on one track fold max-magnitude into the one positional slot
                // the frame carries for that track, on the control plane.
                if let Some(slot) = ready.meter_frame.get_mut(gain_base + track as usize)
                    && value > *slot
                {
                    *slot = value;
                }
                if let Some(present) = ready.observation_present.get_mut(track as usize) {
                    *present = true;
                }
                observed_any = true;
                if window.first_sample < first_sample {
                    first_sample = window.first_sample;
                }
                if window.end_sample > end_sample {
                    end_sample = window.end_sample;
                }
            }
        }
        // `masterGrDb` is a designation, not a discovery (D6). It is absent -- not zero -- when no
        // track was designated or the designated track published nothing, because zero would be
        // indistinguishable from "the master is not reducing".
        let master_present = match ready.master_track {
            Some(track) => ready
                .observation_present
                .get(track as usize)
                .copied()
                .unwrap_or(false),
            None => false,
        };
        if master_present
            && let Some(track) = ready.master_track
            && let Some(value) = ready.meter_frame.get(gain_base + track as usize).copied()
            && let Some(slot) = ready.meter_frame.get_mut(gain_base + tracks)
        {
            *slot = value;
        }
        ready.meter_header.windows = windows;
        ready.meter_header.sequence = ready.meter_header.sequence.saturating_add(1);
        ready.meter_header.master_gr_present = u32::from(master_present);
        if observed_any {
            ready.meter_header.first_sample = first_sample;
            ready.meter_header.end_sample = end_sample;
        }
        ready.meter_windows = ready.meter_windows.saturating_add(u64::from(windows));
        windows
    }

    fn finish_commands(&mut self, result: u32, reason: u32, index: u32, admitted: u32) -> u32 {
        self.command_report.result = result;
        self.command_report.reason = reason;
        self.command_report.rejected_index = index;
        self.command_report.admitted = admitted;
        if admitted == 0 {
            self.command_report.applied_at_sample = self.status.next_absolute_sample;
        }
        self.record(result)
    }

    pub(crate) fn record_boundary_result(&mut self, code: u32) -> u32 {
        self.record(code)
    }

    /// Mark an observed browser output-shape mismatch as sticky reprepare-required.
    pub fn reject_output_quantum(&mut self, actual_frames: u32) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if actual_frames == self.config.quantum_frames {
            return self.record(RESULT_OK);
        }
        self.status.state = STATE_FAILED;
        self.record(RESULT_REPREPARE_REQUIRED)
    }

    /// Release all prepared ownership. Repeated disposal is harmless.
    ///
    /// This runs in the worklet's `port.onmessage` handler, never inside `process()`; it is the
    /// single point where the plan, the compiled session and the source rings are freed.
    pub fn dispose(&mut self) -> u32 {
        self.meter_lease = false;
        self.ready = None;
        self.buffers = None;
        self.diagnostic_len = 0;
        self.status.state = STATE_DISPOSED;
        self.record(RESULT_OK)
    }

    fn record(&mut self, code: u32) -> u32 {
        self.status.last_result = code;
        code
    }

    /// Record a sticky failure without ever releasing ownership.
    ///
    /// `self.ready` **is** this host's one-slot retirement queue (AGENTS.md: "the displaced plan
    /// goes to a bounded retirement queue and is reclaimed off the render thread"). `fail` is
    /// reachable from `render_next`, which the AudioWorklet calls on the rendering thread, so it
    /// must not move, overwrite or drop the plan, the compiled session or the source rings:
    /// dropping them would free `Arc<spsc::Ring<_>>` payloads and run `dlmalloc::free` inside
    /// `process()`. The ownership stays exactly where it is and only [`Self::dispose`] — a control
    /// path call, delivered on `port.onmessage` — reclaims it.
    ///
    /// A separate `retired` field would not help: `self.retired = self.ready.take()` compiles to a
    /// conditional `drop_glue::<Option<ReadyOwnership>>` call on the overwritten value, which the
    /// call-graph gate sees even though it never executes.
    ///
    /// The output block is filled with positive zero so a failed render emits silence rather than
    /// the previous quantum.
    fn fail(&mut self, code: u32, diagnostic: &[u8]) -> u32 {
        self.status.state = STATE_FAILED;
        if let Some(buffers) = self.buffers.as_mut() {
            buffers.output_pcm.fill(0.0);
            self.diagnostic_len = diagnostic.len().min(buffers.diagnostic.len());
            buffers.diagnostic[..self.diagnostic_len]
                .copy_from_slice(&diagnostic[..self.diagnostic_len]);
        }
        self.record(code)
    }
}

/// One refused submission: the frozen result, the typed reason, and the offending record.
struct CommandRejection {
    result: u32,
    reason: u32,
    index: u32,
}

/// One decoded, still-unapplied `miso.command.v1` record (issue #137 D1).
///
/// # The frozen 48-byte little-endian layout
///
/// | offset | width | field |
/// |---|---|---|
/// | 0 | u8 | `kind`, one of the `COMMAND_*` values |
/// | 1 | u8 | `rack`: `0` simd1, `1` dynamic, `2` simd2, `255` not applicable |
/// | 2 | u8 | `channel`: `0` left, `1` right, `2` both, `255` not applicable |
/// | 3 | u8 | required zero |
/// | 4 | u32 | `track_index` into the canonical normalized track order |
/// | 8 | u32 | `effect_index` within the addressed rack |
/// | 12 | u32 | `parameter_id` from the effect contract |
/// | 16 | u32 | `smoothing_samples`, the ramp window for a retarget |
/// | 20 | u32 | required zero |
/// | 24..40 | 4 x f32 | `value0..value3` |
/// | 40 | u64 | required zero |
///
/// There is no string on this path. The identity mapping a caller needs -- track index to track
/// ID, effect index to effect ID, parameter ID to name and domain -- is deliverable 4's build-time
/// metadata plus the session map the ready message carries, never a per-command lookup.
#[derive(Clone, Copy)]
struct CommandRecord {
    kind: u32,
    rack: u8,
    channel: u8,
    track_index: u32,
    effect_index: u32,
    parameter_id: u32,
    smoothing_samples: u32,
    values: [f32; 4],
}

impl CommandRecord {
    fn decode(bytes: &[u8]) -> Result<Self, u32> {
        if bytes.len() != COMMAND_RECORD_BYTES as usize {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let word = |offset: usize| {
            u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        };
        if bytes[3] != 0 || word(20) != 0 || bytes[40..48].iter().any(|value| *value != 0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let kind = u32::from(bytes[0]);
        if !matches!(
            kind,
            COMMAND_PAN
                | COMMAND_MATRIX
                | COMMAND_FADER_DB
                | COMMAND_MUTE
                | COMMAND_EFFECT_PARAM
                | COMMAND_EFFECT_BYPASS
                | COMMAND_OBSERVE_SUBSCRIBE
                | COMMAND_OBSERVE_UNSUBSCRIBE
                | COMMAND_SOLO
                | COMMAND_TRIM_DB
                | COMMAND_POLARITY_INVERT
        ) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        let mut values = [0.0_f32; 4];
        for (index, value) in values.iter_mut().enumerate() {
            *value = f32::from_le_bytes([
                bytes[24 + index * 4],
                bytes[25 + index * 4],
                bytes[26 + index * 4],
                bytes[27 + index * 4],
            ]);
            if !value.is_finite() {
                return Err(COMMAND_REASON_MALFORMED);
            }
        }
        Ok(Self {
            kind,
            rack: bytes[1],
            channel: bytes[2],
            track_index: word(4),
            effect_index: word(8),
            parameter_id: word(12),
            smoothing_samples: word(16),
            values,
        })
    }

    /// Lower one addressed track-builtin record, or say why it cannot be.
    ///
    /// Issue #140 C: `fader_db` and `mute` are no longer refused here. They lower onto the live
    /// ramped fader section (`FaderMuteRampBuiltinsV1`), which a console-attached track binds in
    /// place of the prepared `FaderMuteBuiltins`. A track with no console still refuses them --
    /// with [`COMMAND_REASON_UNSUPPORTED_KIND`], because the target exists and the value is legal
    /// and this *session* has no write path -- which is exactly what the reason means.
    fn into_track_record(self) -> Result<AdmittedCommand, u32> {
        match self.kind {
            COMMAND_PAN => {
                if self.rack != 255
                    || self.channel != 255
                    || self.values[2] != 0.0
                    || self.values[3] != 0.0
                {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let matrix = pan_matrix(self.values[0], self.values[1])
                    .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(AdmittedCommand::Matrix(TrackControlRecordV1 {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_MATRIX => {
                if self.rack != 255 || self.channel != 255 {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let matrix = Matrix2x2 {
                    ll: self.values[0],
                    lr: self.values[1],
                    rl: self.values[2],
                    rr: self.values[3],
                }
                .checked()
                .map_err(|_| COMMAND_REASON_DOMAIN)?;
                Ok(AdmittedCommand::Matrix(TrackControlRecordV1 {
                    matrix,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_FADER_DB => {
                if self.rack != 255 || self.values[1..].iter().any(|value| *value != 0.0) {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                // The declared domain of `fader_db` in `BUILTIN_PARAMETER_DESCRIPTORS_V1`.
                if !(-144.0..=24.0).contains(&self.values[0]) {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Ok(AdmittedCommand::Fader(TrackFaderRecordV1::FaderDb {
                    lanes,
                    db: self.values[0],
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            COMMAND_MUTE => {
                if self.rack != 255 || self.values[1..].iter().any(|value| *value != 0.0) {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                if self.values[0] != 0.0 && self.values[0] != 1.0 {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Ok(AdmittedCommand::Fader(TrackFaderRecordV1::Mute {
                    lanes,
                    muted: self.values[0] == 1.0,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            // #210 phase 3. The shape rules are `faderDb`'s, because the two parameters share a
            // domain and a smoothing law; the destination is the input queue rather than the
            // fader one, which `admit_commands_staged` decides from the lowered variant exactly as
            // it already decides between the matrix and fader bands.
            COMMAND_TRIM_DB => {
                if self.rack != 255 || self.values[1..].iter().any(|value| *value != 0.0) {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                // The declared domain of `trim_db` in `BUILTIN_PARAMETER_DESCRIPTORS_V1`. It is
                // the same range `fader_db` carries and is spelled again rather than shared, which
                // is this file's convention: the comment names the authority.
                if !(-144.0..=24.0).contains(&self.values[0]) {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Ok(AdmittedCommand::Input(TrackInputRecordV1::TrimDb {
                    lanes,
                    db: self.values[0],
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            // The shape rules are `mute`'s: a boolean-exact domain on `values[0]`.
            COMMAND_POLARITY_INVERT => {
                if self.rack != 255 || self.values[1..].iter().any(|value| *value != 0.0) {
                    return Err(COMMAND_REASON_MALFORMED);
                }
                let lanes = lane_selector(self.channel).ok_or(COMMAND_REASON_MALFORMED)?;
                if self.values[0] != 0.0 && self.values[0] != 1.0 {
                    return Err(COMMAND_REASON_DOMAIN);
                }
                Ok(AdmittedCommand::Input(TrackInputRecordV1::PolarityInvert {
                    lanes,
                    inverted: self.values[0] == 1.0,
                    smoothing_samples: self.smoothing_samples,
                }))
            }
            _ => Err(COMMAND_REASON_MALFORMED),
        }
    }

    /// Read one `solo` record's requested bit, or say why it cannot be read (issue #210 phase 1).
    ///
    /// Deliberately *not* an arm of [`Self::into_track_record`]: a solo record lowers to no record
    /// of its own. It moves console state, and the mute records that state composes to are staged
    /// once, coalesced, at the end of the submission's first pass. The shape rules are `mute`'s,
    /// with `channel = 255` because solo addresses a strip and not a lane, and the same
    /// `DOMAIN`-for-a-non-boolean rule `mute` uses for `values[0]`.
    const fn into_solo_request(self) -> Result<bool, u32> {
        if self.rack != 255 || self.channel != 255 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.values[1] != 0.0 || self.values[2] != 0.0 || self.values[3] != 0.0 {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.values[0] != 0.0 && self.values[0] != 1.0 {
            return Err(COMMAND_REASON_DOMAIN);
        }
        Ok(self.values[0] == 1.0)
    }

    /// Lower one effect-addressed record against the addressed effect's own descriptor.
    ///
    /// Writes one or two records into `out` and returns how many: a `channel = both` command on a
    /// parameter whose policy is `PerLane` lowers to **one record per lane**, because every launch
    /// effect counts a policy-violating span as `invalid_spans` rather than applying it. Doing the
    /// expansion here -- on the control plane, once per submission -- is what keeps the render
    /// side's drain a pure one-record-one-span map.
    /// Lower one observation subscribe/unsubscribe against the addressed effect's declared menu.
    ///
    /// `parameter_id` carries the effect-local `tap_id` and is translated to the `tap_index` the
    /// render side arms — an admission-time lookup, off the render thread, exactly as a parameter
    /// id is. A tap the descriptor does not declare is [`COMMAND_REASON_UNKNOWN_TAP`] and never
    /// [`COMMAND_REASON_UNKNOWN_PARAMETER`]: they are different namespaces on one effect, and a
    /// caller that confuses them learns which one it got wrong.
    fn into_observe_record(
        self,
        descriptor: &'static miso_engine_effect_contract::EffectDescriptor,
        observed: bool,
    ) -> Result<AdmittedCommand, u32> {
        if self.channel != 255 || self.values.iter().any(|value| *value != 0.0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.parameter_id == 0 {
            return Err(COMMAND_REASON_UNKNOWN_TAP);
        }
        let Some((tap_index, tap)) = descriptor
            .observations
            .iter()
            .enumerate()
            .find(|(_, tap)| tap.id.0 == self.parameter_id)
        else {
            return Err(COMMAND_REASON_UNKNOWN_TAP);
        };
        // A computed tap is declared, validated and refused: V1 ships no implementation for one,
        // and saying so is better than binding a lane that would never publish.
        if !matches!(
            tap.cost,
            miso_engine_effect_contract::ObservationCostV1::Resident
        ) {
            return Err(COMMAND_REASON_UNSUPPORTED_KIND);
        }
        // The tap exists and the address is right; this *session* bound no lane to arm.
        if !observed {
            return Err(COMMAND_REASON_OBSERVATION_UNBOUND);
        }
        let tap_index = u32::try_from(tap_index).map_err(|_| COMMAND_REASON_MALFORMED)?;
        Ok(AdmittedCommand::Effect(EffectControlRecord::Observe {
            tap_index,
            armed: self.kind == COMMAND_OBSERVE_SUBSCRIBE,
            window_blocks: self.smoothing_samples,
        }))
    }

    fn into_effect_records(
        self,
        descriptor: &'static miso_engine_effect_contract::EffectDescriptor,
        out: &mut [AdmittedCommand; 2],
    ) -> Result<usize, u32> {
        if self.kind == COMMAND_EFFECT_BYPASS {
            if self.channel != 255
                || self.parameter_id != 0
                || self.smoothing_samples != 0
                || self.values[1..].iter().any(|value| *value != 0.0)
            {
                return Err(COMMAND_REASON_MALFORMED);
            }
            if self.values[0] != 0.0 && self.values[0] != 1.0 {
                return Err(COMMAND_REASON_DOMAIN);
            }
            out[0] = AdmittedCommand::Effect(EffectControlRecord::Bypass(self.values[0] == 1.0));
            return Ok(1);
        }
        if self.channel > 2 || self.values[1..].iter().any(|value| *value != 0.0) {
            return Err(COMMAND_REASON_MALFORMED);
        }
        if self.parameter_id == 0 {
            return Err(COMMAND_REASON_UNKNOWN_PARAMETER);
        }
        let Some((parameter_index, parameter)) = descriptor
            .parameters
            .iter()
            .enumerate()
            .find(|(_, parameter)| parameter.id.0 == self.parameter_id)
        else {
            return Err(COMMAND_REASON_UNKNOWN_PARAMETER);
        };
        // A parameter the descriptor says cannot be automated has no write path at all. No launch
        // effect declares one, which is why this is `UNSUPPORTED_KIND` and not `DOMAIN`.
        if !parameter.automatable
            || parameter.automation_rate == miso_engine_effect_contract::AutomationRate::None
        {
            return Err(COMMAND_REASON_UNSUPPORTED_KIND);
        }
        if !parameter_value_valid(parameter, self.values[0]) {
            return Err(COMMAND_REASON_DOMAIN);
        }
        let parameter_index =
            u32::try_from(parameter_index).map_err(|_| COMMAND_REASON_MALFORMED)?;
        let record = |channel: ParameterChannel| {
            AdmittedCommand::Effect(EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value: self.values[0],
            })
        };
        match (parameter.channel_policy, self.channel) {
            // A shared parameter is addressed as "both" and by nothing else.
            (ParameterChannelPolicy::Shared, 2) => {
                out[0] = record(ParameterChannel::Both);
                Ok(1)
            }
            (ParameterChannelPolicy::Shared, _) => Err(COMMAND_REASON_MALFORMED),
            (ParameterChannelPolicy::PerLane, 0) => {
                out[0] = record(ParameterChannel::Left);
                Ok(1)
            }
            (ParameterChannelPolicy::PerLane, 1) => {
                out[0] = record(ParameterChannel::Right);
                Ok(1)
            }
            (ParameterChannelPolicy::PerLane, _) => {
                out[0] = record(ParameterChannel::Left);
                out[1] = record(ParameterChannel::Right);
                Ok(2)
            }
        }
    }
}

/// The wire `channel` byte as a builtin lane selector; `None` for a byte the ABI does not define.
const fn lane_selector(channel: u8) -> Option<BuiltinLaneSelector> {
    match channel {
        0 => Some(BuiltinLaneSelector::Left),
        1 => Some(BuiltinLaneSelector::Right),
        2 => Some(BuiltinLaneSelector::Both),
        _ => None,
    }
}

/// Validate a whole staged submission, then admit it. Nothing is pushed unless everything passes.
///
/// # The three passes are the contract, not an optimisation (#137 D1, extended by #140 C)
///
/// Pass one decodes and lowers every record and counts the room each *destination queue* needs.
/// Pass two checks that room against every one of those queues. Pass three pushes. A submission is
/// therefore all-or-nothing across every kind in it: a batch that moves a matrix, a fader and two
/// effect parameters is admitted whole or refused whole, and the refusal names the first record
/// that broke a rule.
///
/// One wire record can lower to two admitted records (`channel = both` on a per-lane effect
/// parameter), and one submission that touches solo owes up to two *more* per track (issue #210
/// phase 1), which is why `command_decoded` is `2 * MAXIMUM_COMMAND_RECORDS + 2 * track_count`
/// long and why the room counted is per lowered record rather than per wire record.
///
/// # The solo transaction (issue #210 phase 1)
///
/// Pass one now also mutates console solo state -- the solo bits, the user-mute mirror, the
/// emitted-mute mirror -- while it is still deciding whether the submission is admissible at all.
/// So the state carries its own shadow and this wrapper is where it is closed: `commit` once pass
/// three has actually pushed, `rollback` on every refusal. A refused submission leaves host state
/// exactly as it was, which is the same all-or-nothing contract the queues keep.
fn admit_commands(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
) -> Result<(), CommandRejection> {
    match admit_commands_staged(ready, bytes, count) {
        Ok(()) => {
            ready.solo.commit();
            Ok(())
        }
        Err(rejection) => {
            ready.solo.rollback();
            Err(rejection)
        }
    }
}

/// The three passes themselves. Every exit from here is a transaction boundary for its caller.
fn admit_commands_staged(
    ready: &mut ReadyOwnership,
    bytes: &[u8],
    count: usize,
) -> Result<(), CommandRejection> {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    let track_count = ready.tracks.len();
    ready.command_wanted.fill(0);
    let refuse = |reason: u32, index: usize| CommandRejection {
        result: match reason {
            // `ObservationUnbound` is "this preparation cannot deliver it", which is exactly what
            // `RESULT_UNSUPPORTED` means; `UnknownTap` is a bad address, like every other unknown.
            COMMAND_REASON_UNSUPPORTED_KIND | COMMAND_REASON_OBSERVATION_UNBOUND => {
                RESULT_UNSUPPORTED
            }
            COMMAND_REASON_BACKPRESSURE => RESULT_BACKPRESSURE,
            _ => RESULT_INVALID_ARGUMENT,
        },
        reason,
        index: index as u32,
    };
    let mut lowered = 0_usize;
    // Issue #210 phase 1: solo state changes are applied as they are read, but the mute records
    // they compose to are staged once, after the whole batch, by the coalescing pass below.
    let mut solo_seen = false;
    let mut solo_smoothing = 0_u32;
    for index in 0..count {
        let record = &bytes[index * record_bytes..(index + 1) * record_bytes];
        let command = CommandRecord::decode(record).map_err(|reason| refuse(reason, index))?;
        let track = command.track_index as usize;
        if track >= track_count {
            return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
        }
        let mut staged = [AdmittedCommand::Effect(EffectControlRecord::Bypass(false)); 2];
        let (slot, produced) = match command.kind {
            // The three kinds that lower to one record on one of the three per-track bands. The
            // band is read off the lowered variant rather than off the kind, so a kind added to
            // this arm cannot land in the wrong queue by forgetting a second table.
            COMMAND_PAN
            | COMMAND_MATRIX
            | COMMAND_FADER_DB
            | COMMAND_TRIM_DB
            | COMMAND_POLARITY_INVERT => {
                staged[0] = command
                    .into_track_record()
                    .map_err(|reason| refuse(reason, index))?;
                let slot = match staged[0] {
                    AdmittedCommand::Fader(_) => track_count + track,
                    AdmittedCommand::Input(_) => track_count * 2 + track,
                    _ => track,
                };
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                (slot, 1)
            }
            // A mute command carries the user's *intent*; what reaches the queue is the composed
            // effective mute. With no solo engaged the two are the same value and this stages
            // byte-for-byte what it staged before solo existed. Every lane a selector covers
            // shares one track-scoped solo term, so one record still carries the whole command.
            COMMAND_MUTE => {
                let lowered_record = command
                    .into_track_record()
                    .map_err(|reason| refuse(reason, index))?;
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                let AdmittedCommand::Fader(TrackFaderRecordV1::Mute {
                    lanes,
                    muted,
                    smoothing_samples,
                }) = lowered_record
                else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, index));
                };
                if !ready.solo.set_user_mute(track, lanes, muted) {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                }
                let effective = muted || (ready.solo.any_solo() && !ready.solo.solo(track));
                ready.solo.record_emitted(track, lanes, effective);
                staged[0] = AdmittedCommand::Fader(TrackFaderRecordV1::Mute {
                    lanes,
                    muted: effective,
                    smoothing_samples,
                });
                (track_count + track, 1)
            }
            // Solo lowers to nothing here. It moves one console bit; the records that bit composes
            // to are the coalescing pass's business, because a batch of alternating toggles would
            // otherwise fan out up to `2 * track_count` records *per transition*.
            COMMAND_SOLO => {
                let engaged = command
                    .into_solo_request()
                    .map_err(|reason| refuse(reason, index))?;
                if ready.controls.get(track).is_none() {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                }
                if !ready.solo.set_solo(track, engaged) {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                }
                solo_seen = true;
                solo_smoothing = command.smoothing_samples;
                (track_count + track, 0)
            }
            COMMAND_EFFECT_PARAM
            | COMMAND_EFFECT_BYPASS
            | COMMAND_OBSERVE_SUBSCRIBE
            | COMMAND_OBSERVE_UNSUBSCRIBE => {
                if command.rack > 2 {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_RACK, index));
                }
                let Some(counts) = ready.rack_effects.get(track) else {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));
                };
                if command.effect_index >= counts[command.rack as usize] {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_EFFECT, index));
                }
                let Some(effect) = ready.effect_slot(track, command.rack, command.effect_index)
                else {
                    return Err(refuse(COMMAND_REASON_UNKNOWN_EFFECT, index));
                };
                let Some(producer) = ready.effect_controls.get(effect).and_then(Option::as_ref)
                else {
                    return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
                };
                let produced = if matches!(
                    command.kind,
                    COMMAND_OBSERVE_SUBSCRIBE | COMMAND_OBSERVE_UNSUBSCRIBE
                ) {
                    let observed = ready
                        .effect_observations
                        .get(effect)
                        .is_some_and(Option::is_some);
                    staged[0] = command
                        .into_observe_record(producer.descriptor, observed)
                        .map_err(|reason| refuse(reason, index))?;
                    1
                } else {
                    command
                        .into_effect_records(producer.descriptor, &mut staged)
                        .map_err(|reason| refuse(reason, index))?
                };
                (track_count * 3 + effect, produced)
            }
            _ => return Err(refuse(COMMAND_REASON_MALFORMED, index)),
        };
        let Some(wanted) = ready.command_wanted.get_mut(slot) else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, index));
        };
        *wanted = wanted.saturating_add(produced as u32);
        for record in staged.into_iter().take(produced) {
            let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                return Err(refuse(COMMAND_REASON_MALFORMED, index));
            };
            *entry = (slot as u32, record);
            lowered += 1;
        }
    }
    // The coalesced net emission (issue #210 phase 1, correction 1). Every solo and mute change in
    // the batch has been applied; what the console owes the render plane is now the difference
    // between the composed effective mute and what the render plane was last told -- at most two
    // records per track, and **never** a redundant one. That last clause is load-bearing for bit
    // identity, not an optimisation: re-muting an already-settled-muted lane with a nonzero
    // smoothing window re-enters the ramp kernel and turns an exact `+0.0` into `-0.0` for a
    // negative input. It runs only for a batch that actually moved a solo bit; without one, every
    // mute command already staged its own record and the difference is empty by construction.
    //
    // The fade is the last solo record's `smoothing_samples`: a batch is one gesture, and the
    // gesture that moved the solo state is the one whose declick window the console asked for.
    if solo_seen {
        for track in 0..track_count {
            let slot = track_count + track;
            for (lanes, muted) in ready.solo.track_delta(track).into_iter().flatten() {
                let Some(wanted) = ready.command_wanted.get_mut(slot) else {
                    return Err(refuse(
                        COMMAND_REASON_UNSUPPORTED_KIND,
                        count.saturating_sub(1),
                    ));
                };
                *wanted = wanted.saturating_add(1);
                let Some(entry) = ready.command_decoded.get_mut(lowered) else {
                    return Err(refuse(COMMAND_REASON_MALFORMED, count.saturating_sub(1)));
                };
                *entry = (
                    slot as u32,
                    AdmittedCommand::Fader(TrackFaderRecordV1::Mute {
                        lanes,
                        muted,
                        smoothing_samples: solo_smoothing,
                    }),
                );
                lowered += 1;
                ready.solo.record_emitted(track, lanes, muted);
            }
        }
    }
    for entry in 0..lowered {
        let slot = ready.command_decoded[entry].0 as usize;
        let Some(capacity) = ready.queue_capacity(slot) else {
            return Err(refuse(COMMAND_REASON_UNSUPPORTED_KIND, entry));
        };
        let needed = ready.command_wanted[slot];
        if ready.in_flight[slot].saturating_add(needed) > capacity {
            return Err(refuse(COMMAND_REASON_BACKPRESSURE, entry));
        }
    }
    for entry in 0..lowered {
        let (slot, record) = ready.command_decoded[entry];
        let slot = slot as usize;
        if ready.push(slot, record).is_err() {
            return Err(CommandRejection {
                result: RESULT_INTERNAL,
                reason: COMMAND_REASON_BACKPRESSURE,
                index: entry as u32,
            });
        }
        ready.in_flight[slot] += 1;
    }
    Ok(())
}

/// One published observation magnitude, as the **decibels of reduction** the frame carries.
///
/// The tap declares what crosses the transport (`unit`) and what a consumer reads (`display_unit`,
/// `minimum`, `maximum`); this is the one place the two are reconciled, and it runs once per closed
/// window on the control plane. That placement is the whole of R4: the true-peak limiter's `d` is a
/// linear reduction word and turning it into decibels needs a logarithm, which a render thread may
/// not take.
///
/// The result is clamped into the tap's own declared range, so a consumer never has to guess what
/// a number outside it would have meant.
fn observed_decibels(
    tap: miso_engine_effect_contract::ObservationDescriptor,
    magnitude: f32,
) -> f32 {
    use miso_engine_effect_contract::ParameterUnit;
    let decibels = match tap.unit {
        // Already a decibel magnitude: the declared `PeakMagnitude` fold made it non-negative.
        ParameterUnit::Db => magnitude,
        // `gain = 1 - d`, so the reduction in decibels is `-20 log10(1 - d)`. `d == 1` is total
        // reduction and has no finite decibel value; the declared maximum is what a meter draws.
        ParameterUnit::Linear => {
            if magnitude <= 0.0 {
                0.0
            } else if magnitude >= 1.0 {
                tap.maximum
            } else {
                -20.0 * miso_engine_math::log10f(1.0 - magnitude)
            }
        }
        // No launch tap declares another unit, and a menu that did would be describing something
        // this frame slot has no meaning for.
        _ => 0.0,
    };
    if !decibels.is_finite() || decibels < tap.minimum {
        tap.minimum
    } else if decibels > tap.maximum {
        tap.maximum
    } else {
        decibels
    }
}

const fn empty_command_report() -> WebCommandReport {
    WebCommandReport {
        struct_size: COMMAND_REPORT_BYTES,
        abi_version: ABI_VERSION,
        result: RESULT_OK,
        reason: COMMAND_REASON_NONE,
        rejected_index: 0,
        admitted: 0,
        applied_at_sample: 0,
        reserved: [0; 2],
    }
}

const fn selected_backend() -> u32 {
    if cfg!(all(target_arch = "wasm32", target_feature = "simd128")) {
        BACKEND_SIMD128
    } else {
        BACKEND_SCALAR
    }
}

const fn empty_resource_report(backend: u32) -> WebResourceReport {
    WebResourceReport {
        struct_size: RESOURCE_REPORT_BYTES,
        abi_version: ABI_VERSION,
        sample_rate_hz: 0,
        quantum_frames: 0,
        backend,
        reserved0: [0; 3],
        config_bytes: size_of::<WebPrepareConfig>() as u64,
        status_bytes: size_of::<WebStatus>() as u64,
        session_toml_bytes: 0,
        diagnostic_bytes: 0,
        source_id_bytes: 0,
        source_pcm_staging_bytes: 0,
        output_pcm_bytes: 0,
        bridge_metadata_bytes: 0,
        bridge_retained_bytes: 0,
        largest_bridge_allocation_bytes: 0,
        source_total_bytes: 0,
        source_overhead_bytes: 0,
        effect_scalar_state_bytes: 0,
        effect_scalar_scratch_bytes: 0,
        builtin_retained_bytes: 0,
        graph_session_plus_plan_bytes: 0,
        graph_incremental_plan_bytes: 0,
        graph_metadata_bytes: 0,
        graph_delay_bytes: 0,
        largest_named_allocation_bytes: 0,
        observation_retained_bytes: 0,
        reserved: [0; 3],
    }
}

fn prepare_buffers(
    config: WebPrepareConfig,
) -> Result<(PreparedBuffers, WebResourceReport), u32> {
    validate_config(config)?;
    let source_samples = checked_product([
        u64::from(config.maximum_source_channels),
        u64::from(config.quantum_frames),
    ])?;
    let source_pcm_bytes = source_samples
        .checked_mul(4)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let output_pcm_bytes = u64::from(config.quantum_frames)
        .checked_mul(8)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let command_records = if config.console_command_queue_records == 0 {
        0
    } else {
        MAXIMUM_COMMAND_RECORDS
    };
    let command_bytes = u64::from(command_records)
        .checked_mul(u64::from(COMMAND_RECORD_BYTES))
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let plane_reference_bytes = u64::from(config.maximum_source_channels)
        .checked_mul(size_of::<&[f32]>() as u64)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let host_shell_bytes =
        u64::try_from(size_of::<AudioWorkletEngineHost>()).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let fixed_metadata = host_shell_bytes
        .checked_sub(u64::from(PREPARE_CONFIG_BYTES) + u64::from(STATUS_BYTES))
        .ok_or(RESULT_INTERNAL)?;
    let bridge_metadata = fixed_metadata
        .checked_add(plane_reference_bytes)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    let rows = [
        u64::from(PREPARE_CONFIG_BYTES),
        u64::from(STATUS_BYTES),
        u64::from(config.session_toml_bytes),
        u64::from(config.diagnostic_bytes),
        u64::from(config.source_id_bytes),
        source_pcm_bytes,
        output_pcm_bytes,
        command_bytes,
        bridge_metadata,
    ];
    let retained = checked_sum(rows)?;
    let largest = rows
        .into_iter()
        .max()
        .unwrap_or(0)
        .max(host_shell_bytes)
        .max(plane_reference_bytes);
    if retained > config.maximum_host_retained_bytes
        || largest > config.maximum_named_allocation_bytes
    {
        return Err(RESULT_PREPARE_REJECTED);
    }
    let buffers = PreparedBuffers {
        session_toml: boxed_zero_u8(config.session_toml_bytes)?,
        diagnostic: boxed_zero_u8(config.diagnostic_bytes)?,
        source_id: boxed_zero_u8(config.source_id_bytes)?,
        source_pcm: boxed_zero_f32(source_samples)?,
        output_pcm: boxed_zero_f32(u64::from(config.quantum_frames) * 2)?,
        command: boxed_zero_u8(command_records * COMMAND_RECORD_BYTES)?,
        plane_references: boxed_uninit_planes(config.maximum_source_channels)?,
    };
    let mut report = empty_resource_report(selected_backend());
    report.sample_rate_hz = config.sample_rate_hz;
    report.quantum_frames = config.quantum_frames;
    report.session_toml_bytes = u64::from(config.session_toml_bytes);
    report.diagnostic_bytes = u64::from(config.diagnostic_bytes);
    report.source_id_bytes = u64::from(config.source_id_bytes);
    report.source_pcm_staging_bytes = source_pcm_bytes;
    report.output_pcm_bytes = output_pcm_bytes;
    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = retained;
    report.largest_bridge_allocation_bytes = largest;
    report.largest_named_allocation_bytes = largest;
    Ok((buffers, report))
}

fn validate_config(config: WebPrepareConfig) -> Result<(), u32> {
    if config.struct_size != PREPARE_CONFIG_BYTES || config.abi_version != ABI_VERSION {
        return Err(RESULT_ABI_MISMATCH);
    }
    // Issue #143: observation capacity requires a command queue to carry the subscription, and a
    // master designation must be a plausible track index. The session-level range check is the
    // facade's; this one only refuses what cannot be a track index at all.
    if config.console_observation_taps > u64::from(MAXIMUM_OBSERVATION_TAPS)
        || (config.console_observation_taps != 0 && config.console_command_queue_records == 0)
        || config.console_master_track_plus_one > u64::from(u32::MAX)
        || (config.console_master_track_plus_one != 0 && config.console_observation_taps == 0)
        || config.console_command_queue_records > u64::from(MAXIMUM_COMMAND_RECORDS)
        || config.console_meter_blocks > u64::from(u32::MAX)
        || !matches!(config.sample_rate_hz, 44_100 | 48_000 | 88_200 | 96_000)
        || config.quantum_frames == 0
        || config.session_toml_bytes == 0
        || config.diagnostic_bytes == 0
        || config.source_id_bytes == 0
        || config.maximum_source_channels == 0
        || config.source_ring_frames < config.quantum_frames
        || !config
            .source_ring_frames
            .is_multiple_of(config.quantum_frames)
        || config.maximum_automation_spans_per_block == 0
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let limits = [
        config.maximum_tracks,
        config.maximum_sources,
        config.maximum_routes,
        config.maximum_effects,
        config.maximum_graph_session_plus_plan_bytes,
        config.maximum_source_total_bytes,
        config.maximum_source_overhead_bytes,
        config.maximum_effect_state_bytes,
        config.maximum_effect_scratch_bytes,
        config.maximum_builtin_retained_bytes,
        config.maximum_host_retained_bytes,
        config.maximum_named_allocation_bytes,
        config.maximum_meter_streams,
        config.maximum_meter_items,
        config.maximum_meter_bytes,
    ];
    if limits
        .into_iter()
        .any(|value| value == 0 || value > u64::from(u32::MAX))
    {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    let source_bytes = checked_product([
        u64::from(config.maximum_source_channels),
        u64::from(config.quantum_frames),
        4,
    ])?;
    let output_bytes = u64::from(config.quantum_frames)
        .checked_mul(8)
        .ok_or(RESULT_PREPARE_REJECTED)?;
    if source_bytes > i32::MAX as u64 || output_bytes > i32::MAX as u64 {
        return Err(RESULT_INVALID_ARGUMENT);
    }
    Ok(())
}

/// Map the facade's one typed source rejection onto the frozen browser result codes.
///
/// The facade owns the vocabulary and the diagnostic strings; the browser ABI owns the numbers.
/// Backpressure is bounded and retryable, an internal invariant failure is not the caller's fault,
/// and everything else is a malformed submission.
fn source_result(error: SourceControlError) -> u32 {
    if error.is_backpressure() {
        RESULT_BACKPRESSURE
    } else if error.is_internal() {
        RESULT_INTERNAL
    } else {
        RESULT_INVALID_ARGUMENT
    }
}

/// Translate the frozen browser configuration into the facade's caps, field for field.
///
/// This is the only place the mapping is spelled. `HostShapePolicy::Exact`: unlike the C ABI host,
/// the browser host is handed its `AudioContext` rate and the caller's quantum and must refuse any
/// session that declares anything else -- there is no resampler and no requantiser.
const fn prepare_caps(config: WebPrepareConfig) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz: config.sample_rate_hz,
            quantum_frames: config.quantum_frames,
        },
        source_ring_frames: config.source_ring_frames,
        maximum_source_channels: Some(config.maximum_source_channels),
        maximum_automation_spans_per_block: config.maximum_automation_spans_per_block,
        maximum_tracks: config.maximum_tracks,
        maximum_sources: config.maximum_sources,
        maximum_routes: config.maximum_routes,
        maximum_effects: config.maximum_effects,
        maximum_graph_session_plus_plan_bytes: config.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: config.maximum_source_total_bytes,
        maximum_source_overhead_bytes: config.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: config.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: config.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: config.maximum_builtin_retained_bytes,
        maximum_named_allocation_bytes: config.maximum_named_allocation_bytes,
        maximum_meter_streams: config.maximum_meter_streams,
        maximum_meter_items: config.maximum_meter_items,
        maximum_meter_bytes: config.maximum_meter_bytes,
    }
}

/// Run the shared pipeline, then fold its engine-owned report into the frozen browser report.
///
/// The split is the one the facade documents: engine-owned rows come from
/// `miso_engine_host_core::HostPrepareReport`, the browser bridge's own rows (staging buffers, the
/// host shell, the plane-reference table) were already computed by `prepare_buffers`, and the three
/// browser caps are applied here because they are the *browser's* caps on the shared report.
fn compile_ready(
    toml: &str,
    config: WebPrepareConfig,
    mut report: WebResourceReport,
) -> Result<(ReadyOwnership, WebResourceReport), Vec<u8>> {
    let caps = prepare_caps(config);
    let console = console_request(config).ok_or_else(|| fixed_diagnostic("web.console.config"))?;
    let (session, host, handles) = prepare_host_session_with_console(toml, &caps, &console)
        .map_err(|value| value.into_bytes())?;
    let engine = host.report;

    // Browser-only: every source ID must fit the fixed staging buffer JavaScript writes it into.
    // The facade has no such buffer, so this rule stays the host's.
    if host.sources.longest_id_bytes() > config.source_id_bytes as usize {
        return Err(fixed_diagnostic("web.source.id.capacity"));
    }

    let control_table = control_table_bytes(engine.source_count as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let id_arena = source_id_arena_bytes(engine.source_id_bytes as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    // Charged because the browser bridge retains them: the facade's control table and ID arena
    // (`control_retained_bytes` is exactly those two) and the compiled session model.
    let ready_metadata =
        checked_sum_prepare([engine.control_retained_bytes, engine.session_model_bytes])?;
    let bridge_metadata = report
        .bridge_metadata_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let bridge_retained = report
        .bridge_retained_bytes
        .checked_add(ready_metadata)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let bridge_largest = report
        .largest_bridge_allocation_bytes
        .max(control_table)
        .max(id_arena)
        .max(engine.session_largest_allocation_bytes);
    let largest_named = bridge_largest.max(engine.largest_engine_allocation_bytes);
    if bridge_retained > config.maximum_host_retained_bytes
        || largest_named > config.maximum_named_allocation_bytes
    {
        return Err(fixed_diagnostic("web.resource.limit"));
    }

    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = bridge_retained;
    report.largest_bridge_allocation_bytes = bridge_largest;
    report.source_total_bytes = engine.source_total_bytes;
    report.source_overhead_bytes = engine.source_overhead_bytes;
    report.effect_scalar_state_bytes = engine.effect_scalar_state_bytes;
    report.effect_scalar_scratch_bytes = engine.effect_scalar_scratch_bytes;
    report.builtin_retained_bytes = engine.builtin_retained_payload_bytes;
    report.graph_session_plus_plan_bytes = engine.graph_session_plus_plan_bytes;
    report.graph_incremental_plan_bytes = engine.graph_incremental_plan_bytes;
    report.graph_metadata_bytes = engine.graph_metadata_bytes;
    report.graph_delay_bytes = engine.graph_delay_bytes;
    report.largest_named_allocation_bytes = largest_named;
    // Issue #143 R7: the engine's walked row, carried through unchanged. Zero for a session
    // prepared with `console_observation_taps == 0`.
    report.observation_retained_bytes = engine.observation_retained_bytes;
    let track_count = handles.tracks.len();
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    // Issue #210 phase 1: the solo state's user-mute mirror starts from the *session's* baked
    // fader mutes -- the same `left_mute`/`right_mute` words `track_parameters` compiles into the
    // prepared fader section -- read in the same normalized track order `handles.tracks` carries,
    // because that order is the addressing authority for every queue, meter and command index.
    let mut prepared_mutes: Vec<[bool; 2]> = Vec::new();
    prepared_mutes
        .try_reserve_exact(track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    for track in &session.normalized_model().tracks {
        let count = |effects: usize| -> Result<u32, Vec<u8>> {
            u32::try_from(effects).map_err(|_| fixed_diagnostic("web.console.effects"))
        };
        rack_effects.push([
            count(track.simd1.effects.len())?,
            count(track.dynamic.effects.len())?,
            count(track.simd2.effects.len())?,
        ]);
        prepared_mutes.push([track.fader.left_mute, track.fader.right_mute]);
    }
    // Issue #140 A: the dense effect-queue index. `effect_base[t]` is the number of effect
    // instances declared by every earlier track, so `effect_slot` is arithmetic rather than a
    // search, and the producers are permuted into exactly that order once, here.
    let mut effect_base = Vec::new();
    effect_base
        .try_reserve_exact(track_count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    let mut total_effects = 0_u32;
    for counts in &rack_effects {
        effect_base.push(total_effects);
        for count in counts {
            total_effects = total_effects
                .checked_add(*count)
                .ok_or_else(|| fixed_diagnostic("web.console.effects"))?;
        }
    }
    let track_index: BTreeMap<&str, usize> = handles
        .tracks
        .iter()
        .enumerate()
        .map(|(index, id)| (id.as_ref(), index))
        .collect();
    let mut effect_controls: Vec<Option<EffectControlProducer>> = Vec::new();
    effect_controls
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    effect_controls.resize_with(total_effects as usize, || None);
    for producer in handles.effect_controls {
        let Some(track) = track_index.get(producer.track_id.as_ref()).copied() else {
            return Err(fixed_diagnostic("web.console.effects"));
        };
        let rack = match producer.rack {
            EffectRack::Simd1 => 0_usize,
            EffectRack::Dynamic => 1,
            EffectRack::Simd2 => 2,
        };
        let counts = &rack_effects[track];
        let offset: u32 = counts[..rack].iter().sum();
        let slot = (effect_base[track] + offset + producer.effect_index) as usize;
        let Some(entry) = effect_controls.get_mut(slot) else {
            return Err(fixed_diagnostic("web.console.effects"));
        };
        if entry.is_some() {
            return Err(fixed_diagnostic("web.console.effects"));
        }
        *entry = Some(producer);
    }
    // Three per-track bands since #210 phase 3: matrix/pan, fader/mute, input trim/polarity.
    let queue_count = track_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(total_effects as usize))
        .ok_or_else(|| fixed_diagnostic("web.console.effects"))?;
    // Issue #143: the observation handles are permuted into the same dense `effect_slot` order
    // the command producers use, so one index serves both the subscribe path and the poll.
    let mut effect_observations: Vec<Option<EffectObservationHandle>> = Vec::new();
    effect_observations
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    effect_observations.resize_with(total_effects as usize, || None);
    // `observation_tracks[slot]` is the track index of that effect slot's observed instance, or
    // `u32::MAX` for a slot with no taps. Built once, here, so the poll's fold is arithmetic.
    let mut observation_tracks = vec![u32::MAX; total_effects as usize];
    let observation_present = vec![false; track_count];
    for handle in handles.effect_observations {
        let Some(track) = track_index.get(handle.track_id.as_ref()).copied() else {
            return Err(fixed_diagnostic("web.console.observation"));
        };
        let rack = match handle.rack {
            EffectRack::Simd1 => 0_usize,
            EffectRack::Dynamic => 1,
            EffectRack::Simd2 => 2,
        };
        let counts = &rack_effects[track];
        let offset: u32 = counts[..rack].iter().sum();
        let slot = (effect_base[track] + offset + handle.effect_index) as usize;
        let Some(entry) = effect_observations.get_mut(slot) else {
            return Err(fixed_diagnostic("web.console.observation"));
        };
        if entry.is_some() {
            return Err(fixed_diagnostic("web.console.observation"));
        }
        // The frame carries one gain-reduction slot per track, so every observed effect of a track
        // points at that track and the poll folds them max-magnitude into the one slot.
        observation_tracks[slot] =
            u32::try_from(track).map_err(|_| fixed_diagnostic("web.console.observation"))?;
        *entry = Some(handle);
    }
    let mut meter_header = empty_meter_header();
    meter_header.track_count =
        u32::try_from(track_count).map_err(|_| fixed_diagnostic("web.console.effects"))?;
    meter_header.master_track_plus_one = handles
        .master_track
        .map_or(0, |track| track.saturating_add(1));
    let ready = ReadyOwnership {
        controls: handles.track_controls,
        effect_controls: effect_controls.into_boxed_slice(),
        effect_observations: effect_observations.into_boxed_slice(),
        observation_tracks: observation_tracks.into_boxed_slice(),
        observation_present: observation_present.into_boxed_slice(),
        observation_armed: boxed_zero_u32(total_effects as usize)?,
        master_track: handles.master_track,
        meter_header,
        effect_base: effect_base.into_boxed_slice(),
        command_wanted: boxed_zero_u32(queue_count)?,
        command_decoded: boxed_command_staging(track_count)?,
        solo: ConsoleSoloState::try_new(&prepared_mutes)
            .map_err(|_| fixed_diagnostic("web.resource.allocation"))?,
        in_flight: boxed_zero_u32(queue_count)?,
        tracks: handles.tracks,
        rack_effects: rack_effects.into_boxed_slice(),
        host,
        meters: handles.meters,
        meter_frame: boxed_zero_meter_frame(track_count)?,
        master_peak: [0.0, 0.0],
        meter_windows: 0,
        session,
    };
    Ok((ready, report))
}

/// Translate the browser configuration's two console words into the facade's console request.
///
/// `console_meter_blocks == 0` is the honest form of "metering off": no observer is bound, so the
/// render path folds nothing at all. The port lease is a second, finer switch over posting.
fn console_request(config: WebPrepareConfig) -> Option<HostConsoleRequestV1> {
    let control_queue_depth = match config.console_command_queue_records {
        0 => None,
        records => Some(NonZeroUsize::new(u32::try_from(records).ok()? as usize)?),
    };
    let meter_period_frames = if config.console_meter_blocks == 0 {
        None
    } else {
        let blocks = u32::try_from(config.console_meter_blocks).ok()?;
        Some(NonZeroU32::new(blocks.checked_mul(config.quantum_frames)?)?)
    };
    Some(HostConsoleRequestV1 {
        control_queue_depth,
        meter_period_frames,
        // One window per track per post, plus headroom for a control-side stall of a few windows.
        meter_queue_depth: NonZeroUsize::new(8)?,
        meter_tap: MeterTap::PostMatrix,
        // Issue #143 D3/D6: both are carved browser configuration words, translated once, here.
        observation_taps: u32::try_from(config.console_observation_taps).ok()?,
        master_track: match config.console_master_track_plus_one {
            0 => None,
            value => Some(u32::try_from(value.checked_sub(1)?).ok()?),
        },
    })
}

fn boxed_zero_u32(count: usize) -> Result<Box<[u32]>, Vec<u8>> {
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

/// `3T + 3` words: two peak lanes and one gain-reduction magnitude per track, then the master's.
///
/// The peak section keeps its exact `2T + 2` layout and its exact offsets, so every existing
/// reader of this buffer is unmoved; the gain-reduction section is appended after it.
fn boxed_zero_meter_frame(track_count: usize) -> Result<Box<[f32]>, Vec<u8>> {
    let count = track_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(3))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_command_staging(track_count: usize) -> Result<Box<[(u32, AdmittedCommand)]>, Vec<u8>> {
    // Two entries per staged wire record: one `channel = both` command on a per-lane effect
    // parameter lowers to one record per lane (#140 C).
    //
    // Plus `2 * track_count` for issue #210 phase 1's coalesced solo emission. A submission that
    // moves a solo bit owes the console the difference between the composed effective mute and
    // what the render plane was last told; that is at most two records per track, because
    // `TrackFaderRecordV1::Mute` carries one `muted` bool and a track whose user mute is
    // asymmetric needs one record per lane to restore. The two terms add rather than max: a batch
    // may carry 256 effect-parameter records *and* a solo toggle.
    let count = (MAXIMUM_COMMAND_RECORDS as usize * 2)
        .checked_add(track_count.checked_mul(2).ok_or_else(arithmetic)?)
        .ok_or_else(arithmetic)?;
    let empty = (
        0_u32,
        AdmittedCommand::Effect(EffectControlRecord::Bypass(false)),
    );
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    value.resize(count, empty);
    Ok(value.into_boxed_slice())
}

fn arithmetic() -> Vec<u8> {
    fixed_diagnostic("web.resource.arithmetic")
}

/// One `code\t$\n` diagnostic line for a rule that names no session path.
fn fixed_diagnostic(code: &str) -> Vec<u8> {
    miso_engine_host_core::fixed_diagnostic_line(code)
}

fn checked_sum(values: impl IntoIterator<Item = u64>) -> Result<u64, u32> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total.checked_add(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn checked_sum_prepare(values: impl IntoIterator<Item = u64>) -> Result<u64, Vec<u8>> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(value)
            .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))
    })
}

fn checked_product<const N: usize>(values: [u64; N]) -> Result<u64, u32> {
    values.into_iter().try_fold(1_u64, |total, value| {
        total.checked_mul(value).ok_or(RESULT_PREPARE_REJECTED)
    })
}

fn boxed_zero_u8(bytes: u32) -> Result<Box<[u8]>, u32> {
    let count = usize::try_from(bytes).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

fn boxed_zero_f32(samples: u64) -> Result<Box<[f32]>, u32> {
    let count = usize::try_from(samples).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_uninit_planes(channels: u32) -> Result<Box<[MaybeUninit<&'static [f32]>]>, u32> {
    let count = usize::try_from(channels).map_err(|_| RESULT_PREPARE_REJECTED)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_PREPARE_REJECTED)?;
    value.resize_with(count, MaybeUninit::uninit);
    Ok(value.into_boxed_slice())
}

mod ffi;

pub use ffi::*;

#[cfg(test)]
mod tests;
