//! Safe browser-Wasm preparation and ownership shell.
//!
//! This module deliberately contains no raw pointer handling or JavaScript integration. It owns
//! the complete immutable session and render plan that the AudioWorklet boundary drives.
//!
//! # The control plane (issue #1381a)
//!
//! The one instance's session is `control_plane::SessionState`, built at boot through
//! [`WebRuntimePreparer`]: it owns the committed model, the source, strip and effect producers, the
//! plan's revision gate, the plan publisher and the retirer. The host renders through the plan
//! owner (`PlanState`), which adopts a published successor at a block boundary with its
//! [`RenderCompanions`], and the service step (`miso_engine_web_v1_service`) reclaims the displaced
//! plan and its attachment on the control side, in the worklet's message handler, never in
//! `process()`. Every value reaches the cells through `SET_CONTROL` and the core
//! (`control_command`, #1562a); the browser keeps no value admission of its own (#1565A), and an
//! observation tap is armed by the core's `Observe` edit through the same entry (#1589): the
//! control half is the one producer of every observation queue.
//!
//! # What is here and what is not
//!
//! The compile pipeline is **not** here. Parsing, compiling, source rings, effect and builtin
//! preparation, the graph compile, the identity bindings and the engine-owned resource report are
//! `host-core`, shared with the C ABI host (issue #103). This crate owns exactly what is
//! the browser's: the frozen issue-024 ABI structs and result codes, the fixed staging buffers the
//! JavaScript side reads and writes through raw addresses, the browser bridge's own resource rows,
//! and the render/failure state machine. Before #106 F1 it carried a second, already-diverged copy
//! of the shared pipeline -- 288 lines of `compile_ready` alone, only one of whose two copies
//! rejected source generation `0`.

use core::mem::{MaybeUninit, size_of};
use core::num::NonZeroUsize;
use core::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use builtins_compiler::MeterConsumer;
use control_plane::{
    CompileFailure, ControlAdapter, ControlLimits, PlanQueries, PlanResources, PlanState,
    SessionSeekFailure, SessionState, SourceFailure, SourceRefusalKind,
};
use engine::realtime::{
    HostAttachment, ObserverCarry, ObserverKind, PlanarBufferMut, PublishedWordsReader,
    PublishedWordsWriter, QueueFull, published_words,
};
use host_core::{
    CompiledSession, EffectObservationHandle, GainReductionTaps, HostLiveControlRequest,
    HostLiveLanes, HostPrepareCaps, HostPrepareReport, HostShapePolicy, MAXIMUM_VCA_REACH_PAIRS,
    MAXIMUM_VCAS_BOUNDED, MeterCarry, MeterDelivery, MeterFrame, MeterLevel, MeterTables,
    MeterValidity, PrepareDiagnostics, PrepareRejection, PreparedHost, PreparedSpectrumCaptures,
    SourceControlError, SourceSubmission, SuccessorBase, TapView, compile_host_model,
    compiled_session_shape, control_table_bytes, parse_host_session, source_id_arena_bytes,
};
use session::{CompileCaps, SOURCE_CHANNELS_LAUNCH_MAXIMUM, STABLE_ID_MAXIMUM_BYTES};

use host_core::EffectRack;
/// Live effect addressing in the session's own terms (decision 12, issue #1096).
pub use host_core::{LiveEffectAddress, LiveEffectRack};

pub use host_core::{SOURCE_STALL_TOLERANCE_MS, default_source_ring_frames};

/// Browser host ABI version 1.0.
pub const ABI_VERSION: u32 = 0x0001_0000;

/// Conservative transient parse projection in bytes per staged document byte.
///
/// Issue #338 re-measured the then-pinned `jstrict 0.14.0` JSON frontend plus typed model and compilation
/// over the minimal document, dense one-, 64-, and 192-track documents, and the exact 1 MiB
/// admitted ceiling. The largest observed ratio was 14.738 bytes per input byte; 17 left 15.3%
/// headroom. Issue #1093 re-measured it after decision 12's required root `console` grew the
/// minimal document from 447 to 511 bytes and its parse, model and compile peak from 6,780 to
/// 8,944 bytes: 17.503 bytes per input byte, now the largest ratio (the dense documents stay
/// below 13.1). 20 left 14.3% headroom. Issue #1381a re-measured the exact 1 MiB, 192-track by
/// 4-effect document on the shipped module (`scripts/check-web-boot-budget.mjs`): the boot now
/// builds the control plane's session, whose catalog, producers and protocol state the browser did
/// not hold, and the wasm high-water growth rose from under 20 MiB to 25,427,968 bytes (24.25
/// bytes per input byte). Root ruled 25 for the interval (2026-10-14). Issue #1611 found the cause
/// of that growth in the control plane's live-lane choice (`controls::live_lanes`), which collected
/// every control row of the session (53,763 rows of 232 bytes for this document) into one vector
/// grown by doubling to 15.2 MB, only to read three booleans, and on wasm the freed large blocks of
/// that doubling are not reused; it now walks the rows one owner at a time. The same document grows
/// the wasm high-water by 20,447,232 bytes (19.5 bytes per input byte, 312 pages), under the 20 MiB
/// pin of 20,971,520 bytes plus one page, so the multiplier is 20 again. Boot checks
/// `document_bytes * PARSE_TRANSIENT_MULTIPLIER` against the effective budget before UTF-8 decode
/// or parser allocation, and the peak-transient test keeps every phase visible so frontend growth
/// cannot silently outrun this projection.
///
/// Issue #2191: this projection, with the control frame's rows, is the document's only default
/// limit. A boot (and a staged document, and a `SESSION_REPLACE`) admits up to
/// `(budget - frame rows) / PARSE_TRANSIENT_MULTIPLIER` bytes, 26,814,976 on wasm at the 512 MiB
/// default and the 128 KiB frame; the compiled 1 MiB cap is gone, and the
/// `maximum_document_bytes` boot word only tightens it. It bounds the parse transient, not
/// effect-state growth (#2194).
pub const PARSE_TRANSIENT_MULTIPLIER: u64 = 20;

/// What `miso_engine_web_v1_source_fill` answers for a refused read (issue #1387): no ring holds
/// `u32::MAX` queued chunks, and a granted read's room word is below `u32::MAX`.
pub const SOURCE_FILL_REFUSED: u64 = u64::MAX;

/// The control frame a zero `maximum_control_frame_bytes` boot word selects (issue #2191): 128
/// KiB (root ruling). The console's atomic bus add is a submix, a route to the master and one
/// muted route per track, about `1344 + 336 * tracks` bytes, so one frame holds a bus add over
/// about 385 tracks; a host that needs more sets the word. Measured cost: the control plane's
/// frame rows ([`control_plane::frame_scaled_retained_bytes`]: two frames of scratch and the
/// replay cache of two frames, plus about 55 KB of fixed tables) are 580,096 bytes at 128 KiB and
/// 154,112 at 16 KiB, and with the page's one-frame staging a one-track session retains 1,179,991
/// bytes at 128 KiB against 639,319 at 16 KiB: about 4.7 bytes per frame byte, 0.1 % of
/// [`DEFAULT_MAXIMUM_MEMORY_BYTES`] at the default.
pub const DEFAULT_CONTROL_FRAME_BYTES: u32 = 128 << 10;
/// The smallest nonzero `maximum_control_frame_bytes` boot word (issue #2191): 16 KiB, the frame
/// the browser qualified before the word existed. A smaller frame would refuse ordinary replies.
pub const MINIMUM_CONTROL_FRAME_BYTES: u32 = 16 << 10;
/// The largest `maximum_control_frame_bytes` boot word (issue #2191): 16 MiB, so every
/// frame-scaled buffer and its doubled replay cache stay far inside the `u32` staging addresses
/// and the 512 MiB module maximum.
pub const MAXIMUM_CONTROL_FRAME_BYTES: u32 = 16 << 20;

/// Default host memory ceiling used only when the embedding passes zero.
///
/// 512 MiB matches the native runner profile and the browser app's historical quota-reserve
/// scale. It leaves more than four times the retained headroom of every frozen corpus fixture and
/// more than seven times the measured ~70 MiB worst transient at the 1 MiB document cap.
pub const DEFAULT_MAXIMUM_MEMORY_BYTES: u64 = 512 << 20;

/// Initial size, in 64 KiB pages, of the shared linear memory the browser module imports (issue
/// #1380 D1).
///
/// Every instantiation site creates `new WebAssembly.Memory({ initial, maximum, shared: true })`
/// with this count, read from the ABI layout document's `memory.initialPages`, and
/// `scripts/build-web-audioworklet.sh` links the module with `--initial-memory` of exactly this
/// many pages. It is the linker's own minimum for today's module: the 1 MiB stack plus the static
/// data and the thread-local block. Boot grows the memory past it on every session, so a larger
/// value would only add pages the allocator never uses (it hands out only pages it grew itself).
/// A module whose data outgrows it fails to link, loudly, and the fix is to raise this count.
pub const WASM_INITIAL_MEMORY_PAGES: u32 = 20;

/// Maximum size, in 64 KiB pages, of the shared linear memory the browser module imports: 512
/// MiB (issue #1380 D1, owner decision of 2026-10-27).
///
/// A shared memory must declare its maximum, and a browser reserves it up front; 8192 pages is
/// what #1331a reserved and ran in desktop Chromium and WebKit. It is the module's hard ceiling,
/// so boot refuses a `maximum_memory_bytes` above it with `web.boot.memory_maximum` (D2) rather
/// than fail later inside `memory.grow`. [`DEFAULT_MAXIMUM_MEMORY_BYTES`] equals it.
pub const WASM_MAXIMUM_MEMORY_PAGES: u32 = 8192;

/// Bytes in one WebAssembly page.
pub const WASM_PAGE_BYTES: u64 = 1 << 16;

const _: () = assert!(
    DEFAULT_MAXIMUM_MEMORY_BYTES <= WASM_MAXIMUM_MEMORY_PAGES as u64 * WASM_PAGE_BYTES,
    "the default boot budget must fit the module's maximum memory"
);
const _: () = assert!(WASM_INITIAL_MEMORY_PAGES <= WASM_MAXIMUM_MEMORY_PAGES);

/// Fixed live diagnostic capacity after a successful boot.
pub const DIAGNOSTIC_BYTES: u32 = 1 << 14;

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
/// Boot refused the effective memory budget.
pub const RESULT_REFUSED_BUDGET: u32 = 5;
/// A bounded source queue is full.
pub const RESULT_BACKPRESSURE: u32 = 6;
/// Requested capability is unsupported.
pub const RESULT_UNSUPPORTED: u32 = 7;
/// Rendering failed.
pub const RESULT_RENDER_REJECTED: u32 = 8;
/// The browser configuration changed and requires a fresh host.
pub const RESULT_REPREPARE_REQUIRED: u32 = 9;
/// The control plane refused a structural command frame with a typed diagnostic (issue #1562a),
/// the browser's counterpart of the C ABI's compile-rejected result. The diagnostic is found at
/// the control response, in place of a response frame.
pub const RESULT_COMPILE_REJECTED: u32 = 10;
/// An internal invariant failed *and was detected by a checked path*.
///
/// A panic does not produce this code on `wasm32`: the target is `panic = abort`, so a panic traps
/// the instance and kills the processor. The worklet converts that trap into this code from
/// JavaScript (see the `.d.ts` header); nothing inside Rust catches it.
pub const RESULT_INTERNAL: u32 = 255;

/// Boot refused the staged document; diagnostics replaced its staged prefix.
pub const RESULT_REFUSED_DOCUMENT: u32 = RESULT_INVALID_ARGUMENT;
/// Boot refused the options structure.
pub const RESULT_REFUSED_OPTIONS: u32 = RESULT_ABI_MISMATCH;
/// Boot was attempted while a live handle already existed.
pub const RESULT_REFUSED_LIFECYCLE: u32 = RESULT_WRONG_STATE;

/// The reliable event lane of `miso_engine_web_v1_control_event` (issue #1570b D1): the C ABI's
/// `EVENT_LANE_RELIABLE`.
pub const EVENT_LANE_RELIABLE: u32 = 0;
/// The lossy telemetry lane of `miso_engine_web_v1_control_event` (issue #1570b D1): the C ABI's
/// `EVENT_LANE_LOSSY`.
pub const EVENT_LANE_LOSSY: u32 = 1;

/// A session and render plan are ready.
pub const STATE_READY: u32 = 2;
/// A sticky preparation or render failure occurred.
pub const STATE_FAILED: u32 = 3;
/// Ownership has been explicitly disposed.
pub const STATE_DISPOSED: u32 = 4;

/// Scalar backend: what a native (non-wasm) build reports; no wasm build is scalar (#1062).
pub const BACKEND_SCALAR: u32 = 0;
/// Base Wasm `simd128` backend.
pub const BACKEND_SIMD128: u32 = 1;

/// Source-ID staging buffer.
pub const BUFFER_SOURCE_ID: u32 = 2;
/// Planar source PCM staging buffer.
pub const BUFFER_SOURCE_PCM: u32 = 3;
/// Fixed diagnostic buffer.
pub const BUFFER_DIAGNOSTIC: u32 = 4;
/// Contiguous dual-mono output buffer.
pub const BUFFER_OUTPUT_PCM: u32 = 5;
// Buffer kind 6 was the observation-command staging (kinds 7 and 8), deleted by #1589c; the
// value is retired and never reused.
/// Fixed decimated meter-frame buffer (issue #137 D2).
pub const BUFFER_METER_FRAME: u32 = 7;

// Command kinds 7 and 8 (the observation subscribe and unsubscribe records) were deleted by #1589c:
// the core's `Observe` edit arms a tap. Their numbers are retired and never reallocated.
/// The `rack` of an effect-addressed record: one of the strip's inserts (decision 12, #1096).
///
/// It keeps the code the retired `dynamic` rack had. `effect_index` is the insert's index.
pub const RACK_INSERTS: u8 = 1;
/// The `rack` of an effect-addressed record: a session console slot (decision 12, #1096).
///
/// Appended. `effect_index` is the console slot's index.
///
/// The retired `simd1` and `simd2` codes, `0` and `2`, are refused and never reallocated.
/// `ABI_VERSION` did not move.
pub const RACK_CONSOLE: u8 = 3;
/// The `rack` of a builtin-addressed record, which names no effect rack.
pub const RACK_NOT_APPLICABLE: u8 = 255;

/// The dense effect position of one live address, given its track's first position and its
/// lowered rack counts in chain order (issue #1096).
///
/// The one place a session address becomes a position in the effect band: preparation places
/// every producer and reader with it and a read finds it with it, so the two cannot disagree. `None` when
/// the track has no such console slot or insert.
fn dense_effect_slot(base: u32, counts: [u32; 3], address: LiveEffectAddress) -> Option<usize> {
    let (rack, index) = address.lower(counts)?;
    let earlier = match rack {
        EffectRack::Simd1 => 0,
        EffectRack::Dynamic => counts[0],
        EffectRack::Simd2 => counts[0].checked_add(counts[1])?,
    };
    usize::try_from(base.checked_add(earlier)?.checked_add(index)?).ok()
}

/// The strip index of `id`: its position among the tracks, or `tracks.len() + j` for submix `j`
/// (issue #1207 D2).
///
/// host-core's strip list is the tracks in canonical ID order followed by the submixes in
/// canonical ID order. Each segment is sorted but their concatenation is not -- a submix ID may
/// sort before a track ID -- so the two segments are searched apart; a single binary search over
/// the concatenation would silently miss such a submix.
fn strip_index(tracks: &[Box<str>], submixes: &[Box<str>], id: &[u8]) -> Option<usize> {
    // `str` orders by its bytes, so a byte-wise search is the search the canonical order allows.
    if let Ok(track) = tracks.binary_search_by(|track| track.as_bytes().cmp(id)) {
        return Some(track);
    }
    let submix = submixes
        .binary_search_by(|submix| submix.as_bytes().cmp(id))
        .ok()?;
    tracks.len().checked_add(submix)
}

/// Default meter window in render blocks: ~31 frames per second at 48 kHz with a 128-frame quantum.
pub const DEFAULT_METER_BLOCKS: u32 = 12;

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
/// `first_sample`/`end_sample` are half-open and describe only the track and master peak window.
/// Gain-reduction slots are latest independently aged effect folds and do not inherit this span.
/// A consumer correlates the peak span against an arm's sample (the `arm_sample` an
/// `OBSERVATION_READ` row reports) rather than against a wall clock.
///
/// The gain-reduction section is recomputed from the taps armed when each frame is polled. A
/// disarm (an `Observe` edit with `armed: false`) is in effect from the block render applies it,
/// so every meter frame polled after that block leaves that tap out. An arm, also one that re-arms
/// an armed tap, restarts its tap's window at the block render applies it, and the frame leaves
/// that tap out until a window that opens at or after that sample closes. A strip's slot
/// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at or
/// after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is `null`
/// when the designated strip has none.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebMeterHeader {
    /// Exact structure byte size.
    pub struct_size: u32,
    /// ABI version.
    pub abi_version: u32,
    /// Tracks the frame carries. With [`Self::submix_count`] it makes the `f32` view's
    /// `3(T + S) + 3` shape checkable.
    pub track_count: u32,
    /// Complete windows folded by the most recent poll.
    pub windows: u32,
    /// Absolute sample the reported window opened at, inclusive.
    pub first_sample: u64,
    /// Absolute sample the reported window closed at, exclusive.
    pub end_sample: u64,
    /// Monotonic frame sequence, incremented once per posted frame.
    pub sequence: u64,
    /// Designated master strip index (tracks first, then submixes) plus one, or `0` when none was
    /// designated (issue #143 D6; a submix can be the master since issue #1213 D5).
    pub master_track_plus_one: u32,
    /// `1` when `master_gr_db` is meaningful, `0` when no master tap is bound.
    pub master_gr_present: u32,
    /// Publication generation. It increments on each real lease transition and on a detected
    /// meter-producer reset, so retained frames cannot cross either delivery epoch.
    pub reserved: [u64; 2],
    /// Submix strips the frame carries after the tracks (issue #1209 D2), in canonical submix
    /// order. Appended in place, so every earlier offset is unmoved.
    pub submix_count: u32,
    /// `0` while this header describes the plan that renders; `1` once a swap block replaced it
    /// (issue #1290d1). The plan's meter frame travels with its header, so the worklet's views
    /// over the frame are the plan's: a header that reads `1` is the page's signal to switch to
    /// the views it cut over [`Self::successor_pointer`]'s header and frame, in the block that
    /// adopted the plan. The retired header stays alive until the service step after the block
    /// that follows that block (#1332 S5).
    pub retired: u32,
    /// Linear-memory address of the `3(T + S) + 3` word `f32` frame this header describes, or `0`
    /// for a handle with no compiled session (issue #1290d1).
    pub frame_pointer: u32,
    /// Linear-memory address of the header that replaces this one: the newest committed
    /// replacement plan's, from its commit until its adoption block, and the adopted plan's once
    /// [`Self::retired`] is set. `0` when no replacement waits. On the single instance a page
    /// cuts the views over that header and its frame in the message handler that commits the
    /// plan, so the render path switches views by reference and never cuts one (issue #1290d1).
    /// With the halves on two threads that advance notice does not hold (#1332 S5): the word is
    /// published after the commit and read at a block's start, so the plan is usually adopted
    /// in the block that first names it, and after a supersession the word may name freed
    /// tables for up to one block. There it is advisory, and the page must not dereference it
    /// until the S7 protocol (adoption deferred until the worklet acknowledges the header the
    /// Worker posts) makes it safe.
    pub successor_pointer: u32,
    /// The structure epoch of the control list this plan's strip order belongs to (issue #1562b):
    /// the `structure_epoch` that `CONTROL_LIST_GET` pages carry, as the commit that prepared the
    /// plan left it, and as each later commit that rode the plan's live lanes left it. Strips
    /// (tracks, then submixes) are listed in the order the frame carries them, so a client that
    /// holds a list of another epoch reads the list again before it names a frame's strips. A
    /// plan that renders keeps the epoch of its own commit while a replacement waits: the
    /// replacement's header carries the newer one, and the page sees it in the adoption block.
    /// Appended in place, so every earlier offset is unmoved.
    pub structure_epoch: u64,
}

/// `WebMeterHeader::reserved[1]` validity bits. The high 32 bits carry a saturating loss count.
/// Each is the core frame's [`MeterValidity`] bit of the same name (#1582 D2).
pub const METER_VALID_COMPLETE: u64 = MeterValidity::COMPLETE.bits() as u64;
/// The master peak covers the exact published track interval ([`MeterValidity::OUTPUT`]).
pub const METER_VALID_MASTER: u64 = MeterValidity::OUTPUT.bits() as u64;
/// One or more queued windows were dropped or rejected before this publication.
pub const METER_VALID_LOSS: u64 = MeterValidity::LOSS.bits() as u64;
/// Gain-reduction values are present from the effect tap's own latest window; their timing is
/// intentionally independent and never supplies the track/master interval.
pub const METER_VALID_GAIN_REDUCTION: u64 = MeterValidity::GAIN_REDUCTION.bits() as u64;
const METER_LOSS_SHIFT: u32 = 32;
/// Depth of every strip meter queue, track or submix (issue #1448 D1): one window per strip per
/// post, plus headroom for a control-side stall of a few windows. The live-control request builds
/// each meter queue with it, and each poll caps a meter's count at entry by it. It is host-core's
/// one depth (#1581 D3), the output meter's too, so the master and the strips queue alike.
const METER_QUEUE_DEPTH: usize = host_core::METER_QUEUE_DEPTH;

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
        submix_count: 0,
        retired: 0,
        frame_pointer: 0,
        successor_pointer: 0,
        structure_epoch: 0,
    }
}

/// Byte size of [`WebBootOptions`].
pub const BOOT_OPTIONS_BYTES: u32 = size_of::<WebBootOptions>() as u32;
/// Byte size of [`WebStatus`].
pub const STATUS_BYTES: u32 = size_of::<WebStatus>() as u32;
/// Byte size of [`WebResourceReport`].
pub const RESOURCE_REPORT_BYTES: u32 = size_of::<WebResourceReport>() as u32;

/// Exact versioned boot policy shared with JavaScript.
///
/// An all-zero value selects engine defaults. If either handshake word is nonzero, both must name
/// this exact layout and ABI version.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebBootOptions {
    /// Zero for defaults, otherwise [`BOOT_OPTIONS_BYTES`].
    pub struct_size: u32,
    /// Zero for defaults, otherwise [`ABI_VERSION`].
    pub abi_version: u32,
    /// Required physical output sample rate, or zero to accept the document's rate.
    pub require_sample_rate_hz: u32,
    /// Required physical output quantum, or zero to accept the document's quantum.
    pub require_quantum_frames: u32,
    /// Per-source ring override, or zero for the engine's default ring
    /// ([`default_source_ring_frames`]: the 100 ms stall ring plus the largest warm-successor prime
    /// plus one quantum, #1406 D2).
    pub source_ring_frames: u32,
    /// Must be zero.
    pub reserved0: u32,
    /// Total boot memory budget, or zero for [`DEFAULT_MAXIMUM_MEMORY_BYTES`].
    /// Above [`WASM_MAXIMUM_MEMORY_PAGES`] pages boot refuses it with `web.boot.memory_maximum`.
    pub maximum_memory_bytes: u64,
    /// The control frame in bytes, or zero for [`DEFAULT_CONTROL_FRAME_BYTES`] (issue #2191).
    ///
    /// It bounds every control command frame the page stages, every response and event frame,
    /// and the boot command; the control plane sizes its frame-scaled buffers and its replay cache
    /// (`max(64 KiB, 2 * frame)`) from it, and the memory budget charges them. A nonzero value
    /// outside [`MINIMUM_CONTROL_FRAME_BYTES`]..=[`MAXIMUM_CONTROL_FRAME_BYTES`] is refused
    /// `web.options.control_frame_bytes`. The offset was the retired live-command queue word
    /// (#1565); `miso_engine_web_v1_control_frame_bytes` reports the resolved value.
    pub maximum_control_frame_bytes: u64,
    /// Meter window in render blocks, or `0` for host-core's default window with no delivery
    /// (issues #137 D2, #1581 D2).
    ///
    /// Every plan meters every strip (one post-matrix left and right sample peak per strip, tracks
    /// then submixes) and the output, with a `blocks * quantum_frames` window; `0` is the core
    /// default, about 32 ms. A nonzero value also lets the port lease deliver: the lease opens the
    /// output meter's gate and gates whether a finished window is posted. With `0` the lease is
    /// refused, the output meter's gate stays shut, and the strip meters still fold. `12` is ~31
    /// frames per second at 48 kHz with a 128-frame quantum.
    pub live_control_meter_blocks: u64,
    /// The longest session document in bytes, or zero for the budget's own limit (issue #2191).
    ///
    /// A document of `n` bytes is projected to need `n * PARSE_TRANSIENT_MULTIPLIER` bytes of
    /// transient memory to parse beside the control frame's rows, so the effective budget already
    /// admits at most `(budget - frame rows) / PARSE_TRANSIENT_MULTIPLIER` bytes (26,814,976 on
    /// wasm at the defaults) and a zero word adds no other limit. A nonzero
    /// word is a tighter cap for a host that wants one (documents from other people): the boot
    /// document, `miso_engine_web_v1_document_ptr`, `miso_engine_web_v1_replace_document_ptr` and
    /// `SESSION_REPLACE` refuse a longer document, naming its bytes and the limit. The offset was
    /// the observation-taps word until #1590 D5 retired it.
    pub maximum_document_bytes: u64,
    /// Must be zero. This was the master boot word (the designated master strip index plus one)
    /// until #1689 retired it: the master is the strip's `master` row, written by `SET_CONTROL`
    /// after boot, as on every host. The offset keeps its place and the word is refused when
    /// nonzero (`web.options.reserved2`).
    pub reserved2: u64,
}

impl WebBootOptions {
    /// Explicit handshake with every policy word left at its engine default.
    #[must_use]
    pub const fn explicit_defaults() -> Self {
        Self {
            struct_size: BOOT_OPTIONS_BYTES,
            abi_version: ABI_VERSION,
            require_sample_rate_hz: 0,
            require_quantum_frames: 0,
            source_ring_frames: 0,
            reserved0: 0,
            maximum_memory_bytes: 0,
            maximum_control_frame_bytes: 0,
            live_control_meter_blocks: 0,
            maximum_document_bytes: 0,
            reserved2: 0,
        }
    }

    /// Explicit defaults with the default meter window attached.
    #[must_use]
    pub const fn live_control_defaults() -> Self {
        Self {
            live_control_meter_blocks: DEFAULT_METER_BLOCKS as u64,
            ..Self::explicit_defaults()
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
    /// The structural commands this instance's control half ran in place of its render half
    /// (#1290a D6, #1294b), saturating: [`AudioWorkletEngineHost::blocking_rebuilds`], refreshed
    /// as every exported call records its result; zero after a dispose. A page without cross-origin
    /// isolation runs each rebuild in the worklet's message handler, so each one counts here.
    pub blocking_rebuilds: u32,
    /// Exact next render time.
    pub next_absolute_sample: u64,
    /// Number of successful rendered quanta.
    pub rendered_quanta: u64,
    /// The session's `live_values_superseded` count (decision 15, D15-2): the writes to live
    /// values that a later committed value replaced before the block that would have applied them,
    /// summed over every cell the session holds (a `Both` record counts in each channel cell).
    /// Loaded after every rendered block, so it is never behind the last one. It counts in the
    /// C ABI's unit (`COUNTERS_GET`): each read adds the writes it replaced. For one script of
    /// strip fader, mute and pan edits that stages at most one write per cell in a submission, it
    /// equals the C ABI's count. It does not for a script the two hosts stage differently: the
    /// browser also counts the extra writes one submission makes to one cell (send edits in wire
    /// order; a VCA ride on a member that also has a fader command), and the C ABI rebuilds,
    /// writing no cell, for input, route, VCA and submix edits.
    pub live_values_superseded: u64,
    /// The applied-revision watermark's revision (#1349 D2): the highest committed revision in
    /// effect together with every revision before it. Copied from the render report of the block
    /// that advanced it; the initial revision from boot until then.
    pub watermark_revision: u64,
    /// The absolute render sample of the first block in which `watermark_revision` was in effect;
    /// `0` for the initial revision.
    pub watermark_first_sample: u64,
    /// The watermark's outcome flags, in the bits of the C ABI's `miso_engine_v1_watermark`:
    /// `EXACT = 1`, `TRANSITION_FALLBACK = 2`, `SUPERSEDED = 4`. The browser assigns no bit.
    pub watermark_outcome_flags: u64,
}

/// The applied-revision watermark of the plan (#1381a D5), in the order and sizes of the C ABI's
/// `miso_engine_v1_watermark` (#1314 D6). It has no pre-roll counter (D15-8, round-5 amendment).
///
/// The host keeps one at a stable address, written by [`AudioWorkletEngineHost::read_watermark`]
/// and found through `miso_engine_web_v1_watermark_ptr`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebPlanWatermark {
    /// Exact structure byte size ([`WATERMARK_BYTES`]).
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The highest committed revision in effect together with every revision before it.
    pub revision: u64,
    /// The absolute render sample of the first block in which `revision` was in effect.
    pub first_sample: u64,
    /// The OR of the outcome flags over the revisions the last advance covered.
    pub outcome_flags: u64,
    /// Saturating count of revisions completed as exact.
    pub exact_count: u64,
    /// Saturating count of revisions completed as a transition fallback.
    pub transition_fallback_count: u64,
    /// Saturating count of revisions completed as superseded.
    pub superseded_count: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 5],
}

/// Exact byte size of [`WebPlanWatermark`], equal to the C ABI watermark's.
pub const WATERMARK_BYTES: u32 = size_of::<WebPlanWatermark>() as u32;

/// The active plan's output latency and tail (#1688 D2), in the order and sizes of the C ABI
/// report's `latency_samples`, `tail_kind` and `tail_samples` words.
///
/// The host keeps one at a stable address, written by
/// [`AudioWorkletEngineHost::read_plan_latency`] and found through
/// `miso_engine_web_v1_plan_latency_ptr`. The words are the control plane's row for the active
/// plan epoch, copied: the browser computes nothing.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebPlanLatency {
    /// Exact structure byte size ([`PLAN_LATENCY_BYTES`]); zero until the first successful read.
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The plan's output latency in samples.
    pub latency_samples: u64,
    /// `0` for a tail that ends after `tail_samples`, `1` for an unbounded one: the C ABI's
    /// `TAIL_FINITE` and `TAIL_INFINITE`.
    pub tail_kind: u64,
    /// The finite tail in samples; zero for an unbounded tail.
    pub tail_samples: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 4],
}

/// Exact byte size of [`WebPlanLatency`].
pub const PLAN_LATENCY_BYTES: u32 = size_of::<WebPlanLatency>() as u32;

/// Where a source landed after its newest seek (#1293 D6), in the order and sizes of the C ABI's
/// `miso_engine_v1_seek_report`.
///
/// The host keeps one at a stable address, written by
/// [`AudioWorkletEngineHost::read_seek_report`] and found through
/// `miso_engine_web_v1_source_seek_report_ptr`. The words are the control plane's
/// `SessionState::seek_report`, copied: the browser computes nothing (root ruling R7: 64 bytes,
/// the C struct's layout, reserved words zero).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebSeekReport {
    /// Exact structure byte size ([`SEEK_REPORT_BYTES`]); zero until the first successful read.
    pub struct_size: u32,
    /// Required zero.
    pub reserved0: u32,
    /// The generation in effect; `0` until the source's ring has rendered a block.
    pub generation: u64,
    /// The source-read sample of the first block `generation` played in: the clock
    /// `miso_engine_web_v1_source_seek_at`'s anchor is on.
    pub first_sample: u64,
    /// The source frame that block started at; a late anchored seek's includes its lateness.
    pub source_frame: u64,
    /// Saturating cumulative missing in-region frames (true underruns).
    pub cumulative_underrun_frames: u64,
    /// Saturating cumulative in-region frames held while an anchored seek waited.
    pub cumulative_held_frames: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 2],
}

/// Exact byte size of [`WebSeekReport`], equal to the C ABI seek report's.
pub const SEEK_REPORT_BYTES: u32 = size_of::<WebSeekReport>() as u32;

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
    /// Boot-options allocation bytes.
    pub options_bytes: u64,
    /// Status allocation bytes.
    pub status_bytes: u64,
    /// Session JSON staging bytes.
    pub session_document_bytes: u64,
    /// Diagnostic bytes.
    pub diagnostic_bytes: u64,
    /// Source/track-ID staging bytes.
    pub id_staging_bytes: u64,
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
    /// Carved out of the report's first reserved word. The row is *walked* over the built runtime
    /// rather than computed from the configuration; it is zero for a session with no effect that
    /// declares a tap (#1590 D5). The 224-byte layout is unchanged.
    pub observation_retained_bytes: u64,
    /// Required-zero expansion words.
    pub reserved: [u64; 3],
}

/// The render half's own buffer (#1332 S5): the block it renders into.
struct PreparedBuffers {
    output_pcm: Box<[f32]>,
}

/// The control half's own staging (#1332 S5): what the page writes for a control call and what a
/// control call writes for the page, so no control call touches the render half's memory. The
/// single instance publishes a render failure's diagnostic here too
/// ([`AudioWorkletEngineHost::render_next`]), so the page reads one diagnostic buffer.
struct ControlStaging {
    diagnostic: Box<[u8]>,
    /// The valid prefix of [`Self::diagnostic`].
    diagnostic_len: usize,
    source_id: Box<[u8]>,
    source_pcm: Box<[f32]>,
    /// One control frame (issue #1562a D1): the boot's resolved `maximum_control_frame_bytes`
    /// (issue #2191). Staged by the page, read by the control plane in place.
    control_request: Box<[u8]>,
    plane_references: Box<[MaybeUninit<&'static [f32]>]>,
}

impl ControlStaging {
    /// No staging: empty boxes, which allocate nothing. The boot replaces it with the allocated
    /// staging once the session is prepared.
    fn empty() -> Self {
        Self {
            diagnostic: Box::default(),
            diagnostic_len: 0,
            source_id: Box::default(),
            source_pcm: Box::default(),
            control_request: Box::default(),
            plane_references: Vec::new().into_boxed_slice(),
        }
    }

    /// Copy `diagnostic` into the diagnostic buffer, truncated to its capacity.
    fn write_diagnostic(&mut self, diagnostic: &[u8]) {
        let kept = diagnostic.len().min(self.diagnostic.len());
        if let (Some(target), Some(source)) =
            (self.diagnostic.get_mut(..kept), diagnostic.get(..kept))
        {
            target.copy_from_slice(source);
        }
        self.diagnostic_len = kept;
    }

    /// The valid diagnostic prefix.
    fn diagnostic(&self) -> &[u8] {
        self.diagnostic.get(..self.diagnostic_len).unwrap_or(&[])
    }
}

type FfiSourceStaging<'a> = (
    &'a mut [f32],
    &'a mut [MaybeUninit<&'static [f32]>],
    &'a mut [u8],
);

/// What the browser knows of one effect instance of the plan, in the dense `effect_slot` order
/// (#1381a): what the instance declares and, when it declares a tap, where its reader set is in
/// the attachment's [`RenderCompanions`].
///
/// The queries that name an effect read this table, and only the window reads reach the
/// attachment.
struct EffectInstance {
    /// The position of this effect's [`EffectObservationHandle`] in `RenderCompanions`'s list, or
    /// `None` for an effect that declares no tap.
    reader: Option<u32>,
    /// Normalized identity of the strip this instance belongs to.
    track_id: Box<str>,
    /// The session-declared effect identifier.
    effect_id: Box<str>,
    /// The effect's declared parameter table and observation menu.
    descriptor: &'static effect_contract::EffectDescriptor,
}

/// What the browser derives from one plan's shape (issue #1290a): the strip order, the dense effect
/// table that addresses the plan's effects and observation readers, and the per-meter and
/// per-effect scratch that the control and delivery paths use. Every table is a function of the
/// plan's session model, producers and companions, so the plan's own preparation builds it
/// ([`build_plan_shape`]) and every plan carries its own in its [`RenderCompanions`] (#1332 S4):
/// a committed plan's tables reach the render half with the plan, through the plan exchange, and
/// the block that adopts the plan swaps them with the render half's, a pointer exchange that
/// allocates and frees nothing.
///
/// The published meter frame and its header are here too (#1290d1): the frame is `3(T + S) + 3`
/// words for this plan's strips, so a plan with another strip count publishes a frame of its own
/// size. They travel with the tables, in the same pointer exchange. On the single instance the
/// page cuts its views over the successor's frame in the message handler that commits it
/// ([`WebMeterHeader::retired`] and [`WebMeterHeader::successor_pointer`] tell the render path
/// which views to use); on two threads that advance cut is an S7 protocol, not a property of the
/// words (see [`WebMeterHeader::successor_pointer`]).
struct PlanShape {
    /// Canonical normalized track order: the leading segment of the strip order that
    /// `track_index` addresses (issue #1213 D1).
    tracks: Vec<Box<str>>,
    /// Canonical normalized submix order, the strip order's trailing segment: submix `j` is strip
    /// `tracks.len() + j` (issue #1207 D2), which is the index word an observation selection
    /// addresses it by (issue #1213 D1).
    submixes: Vec<Box<str>>,
    /// Every effect instance of the plan in the dense `effect_slot` order, whose length is the
    /// number of instances. It is how the reads reach an effect's readers.
    effect_instances: Box<[Option<EffectInstance>]>,
    /// Issue #143: the meter consumers (issue #137 D2, one per selected meter, empty when
    /// `live_control_meter_blocks` was zero) and the observation reader sets are the plan's
    /// [`RenderCompanions`]; `effect_instances` says where each effect's reader set is.
    /// Strip index of each observed effect slot (a submix's is `tracks.len() + j`), or `u32::MAX`
    /// for a slot with no taps. Built once at compilation, so the poll's fold is arithmetic rather
    /// than a search.
    observation_tracks: Box<[u32]>,
    /// The designated master strip index (tracks first, then submixes), or `None` (issue #143
    /// D6, issue #1213 D5).
    master_track: Option<u32>,
    /// This plan's own words (#1332 S5), `[master + 1, structure epoch]`: the master strip as an
    /// index into this plan's strip order (`0` for none) and the control list's structure epoch,
    /// written by the control half, whose writer this plan's preparation left
    /// ([`PreparedSuccessor`]): stamped before the plan is published
    /// ([`control_plane::RuntimePreparer::stamp_before_publication`]) and after every control
    /// call while it is the newest plan. The render half reads them into the header
    /// ([`RenderReady::publish_words`]), so a plan adopted on another thread after a later commit
    /// still shows its own master and structure epoch.
    words: PublishedWordsReader<2>,
    /// The plan's meter delivery tables (#1582 R1): built with the plan, off the render thread,
    /// and handed to the delivery by pointer exchange at the block that adopts the plan. `None`
    /// while the plan renders (the delivery holds them); a retired plan's are back here, for the
    /// control side to drop.
    meter_tables: Option<MeterTables>,
    /// The render half of the plan's meter frame ring (#1588 D2): render pushes each frame the
    /// delivery publishes for this plan, while the protocol's table is on, until it adopts the
    /// next plan. The preparer moves it here before the plan is published
    /// ([`control_plane::RuntimePreparer::keep_meter_frame_producer`], #1332 S4), so it travels
    /// with the plan and the block that adopts the plan swaps it in with the tables. `None` only
    /// for a plan whose control plane kept it.
    meter_ring: Option<PlanMeterRing>,
    /// The published frame, `3(T + S) + 3` words (issue #1209 D1): `[track0 L, track0 R, .., trackN
    /// L, trackN R, bus0 L, bus0 R, .., busM L, busM R, master L, master R, track0 GR, .., trackN
    /// GR, bus0 GR, .., busM GR, master GR]`, one peak pair and one gain-reduction word per strip.
    /// With `S = 0` every word is where it always was.
    meter_frame: Box<[f32]>,
    /// The header of [`Self::meter_frame`]: the plan's strip counts and designated master, and the
    /// window the poll published. The swap block copies the stream's fields from the retiring
    /// plan's header into the adopted plan's.
    meter_header: WebMeterHeader,
    /// The full-width address of the header [`WebMeterHeader::successor_pointer`] names, or `0`.
    /// The header word is a `u32` linear-memory address, which reads `0` for an address above
    /// 4 GiB (a native test host), so a gate that compares the word with the same conversion
    /// would pass vacuously. This is the address itself, set beside the word, and a gate compares
    /// it with the header's own address.
    #[cfg(any(test, feature = "test-support"))]
    successor_address: usize,
}

/// A plan's meter frame ring producer and the provider epoch it serves (#1588 D2, D3): every
/// frame pushed into it names that epoch ([`MeterFrame::plan_epoch`](host_core::MeterFrame)). A
/// replacement plan's preparer does not know the epoch yet, so the block that adopts the plan
/// sets it, with the tables' own ([`RenderReady::adopt_shape`]).
struct PlanMeterRing {
    epoch: u64,
    producer: host_core::MeterFrameProducer,
}

/// The capacity of the hand-back ring that returns retired plans' tables from the render half to
/// the control half (#1290a, #1332 S5), which drops them in its service step.
///
/// The block that adopts a plan parks the tables it retires in [`RenderReady::parked`], and the
/// next block, before it renders, pushes them into the ring: so the tables outlive the
/// `process()` call of the adopting block, in which the worklet's `postMeterFrame` still reads
/// the retired header (#1290d1), and are freed on the control thread only, one block after
/// their adoption at the earliest.
///
/// The bound is exact. The ring holds what render pushed since the last service step. After that
/// step render adopts at most one plan per retirement credit (a displaced plan holds its credit
/// until the next step reclaims it, and a full retirement queue defers the swap): three
/// (`control_plane::compile::RETIREMENT_CAPACITY`). To their tables add the one shape that was
/// still parked at the step (its plan was adopted in the block just before it), or, before the
/// first block, the tables of the one plan a `SPECTRUM_CONFIGURE` may adopt in place
/// (`PlanState::adopt_before_first_block`), which the first block parks: four. A change to
/// either needs a fifth slot. Were the ring ever full, the
/// tables would stay parked, and the tables the next adoption retires would stay in the adopted
/// plan's companions and drop with that plan on the control thread when the service step
/// reclaims it, never on the render thread: safe, only later.
const RETIRED_SHAPES: usize = 4;

/// The words of the successor record (#1332 S5), which the control half publishes after every
/// control call that succeeded and the render half reads before every block: `[newest epoch,
/// successor header address]`. They are the only global words: while a newer plan waits, the
/// rendering plan's header names the newest committed replacement's header, so on the single
/// instance the page cuts its views over the replacement's frame in advance (#1290d1). On two
/// threads the record gives no advance notice and may name freed tables for up to one block
/// after a supersession: it is advisory until the S7 protocol (see
/// [`WebMeterHeader::successor_pointer`]). Everything that describes one plan is that plan's own
/// ([`PlanShape::words`]).
type SuccessorWords = [u64; 2];

/// The words of one plan ([`PlanShape::words`]): its master strip, as `index + 1` in its own
/// strip order (`0` for none), and the structure epoch of the committed control list.
fn plan_words(master_track: Option<u32>, structure_epoch: u64) -> [u64; 2] {
    [
        master_track.map_or(0, |track| u64::from(track) + 1),
        structure_epoch,
    ]
}

/// The master strip `master` names, as an index into `model`'s strip order (tracks first, then
/// submixes), or `None` when the model does not hold it (#1683, #1689).
fn master_index(model: &session::SessionModel, master: Option<&session::StableId>) -> Option<u32> {
    let id = master?.as_str().as_bytes();
    // The model's strip lists are in canonical order, which orders by bytes, as
    // [`strip_index`] searches the plan's own tables.
    let index = match model
        .tracks
        .binary_search_by(|track| track.id.as_str().as_bytes().cmp(id))
    {
        Ok(track) => Some(track),
        Err(_) => model
            .submixes
            .binary_search_by(|submix| submix.id.as_str().as_bytes().cmp(id))
            .ok()
            .and_then(|submix| model.tracks.len().checked_add(submix)),
    };
    index.and_then(|index| u32::try_from(index).ok())
}

/// The session's lifecycle word (#1332 S5): [`STATE_READY`] until either half records a sticky
/// failure ([`STATE_FAILED`]). Shared by the two halves, so the control half refuses after a
/// render failure and the render half silences after a control failure, without either reading
/// the other's memory. One `Release` store to fail, one `Acquire` load to check.
type Lifecycle = Arc<AtomicU32>;

/// What a preparation leaves for the commit that may publish its plan (#1290a, #1332 S4, S5).
struct PreparedSuccessor {
    /// The address of the plan's meter header, whose tables the plan itself carries. Only an
    /// address: nothing dereferences it.
    header: usize,
    /// The writer of the plan's own words ([`PlanShape::words`]), which the control half keeps
    /// while the plan is the newest.
    words: PublishedWordsWriter<2>,
}

/// The control half of the single instance (#1332 S4): everything one compiled session owns on
/// the control side, which the control exports run.
///
/// Field order is the drop order and is load-bearing: the control plane's session drops first (its
/// producers, then the plan exchange's control ends and the retirement queue), and the compiled
/// session model outlives it. The render half ([`RenderHalf`], whose [`RenderReady`] owns the
/// plan, the source consumers and the active attachment) is the host's own field, declared after
/// this one. Nothing here is ever dropped from `render_next`; see [`RenderHalf::fail`].
struct ControlHalf {
    /// The control plane (#1381a D1): the committed model at its boot revision, the source, strip
    /// and effect producers, the plan's revision gate, the plan publisher and the retirer. It is
    /// the one owner of the producers; the browser holds none of its own.
    control: WebSession,
    /// The limits the control half was built with (#1570b D6), for the cross-host proof.
    #[cfg(feature = "test-support")]
    limits: ControlLimits,
    /// The any-thread projection of the plan: its resource row and the applied-revision watermark
    /// (#1381a D5).
    queries: PlanQueries<WebAdapter>,
    /// Where the preparer leaves the facts a commit reads (#1290a): the address of a replacement
    /// plan's meter header.
    record: Arc<Mutex<BootRecord>>,
    /// The structural commands this instance's control half ran in place of its render half
    /// (#1290a D6), saturating: a rebuild's preparation and publication run on the thread that
    /// also renders, so each one is a blocking rebuild, counted and reported. A rebuild published
    /// while the render half renders on another thread (#1332 S6) blocks nothing and counts nothing.
    blocking_rebuilds: u32,
    /// Whether the render half is on this thread (#1332 S6): `false` from a release to the
    /// adoption back.
    render_here: bool,
    /// The writer of the newest committed plan's own words (#1332 S5, [`PlanShape::words`]):
    /// the boot plan's, then each published replacement's, which its preparation left.
    plan_words: PublishedWordsWriter<2>,
    /// The writer of the successor record ([`SuccessorWords`]), which the render half reads.
    successor: PublishedWordsWriter<2>,
    /// The address of the newest committed replacement's meter header, which its preparer left
    /// in the record; meaningful while that plan waits.
    successor_header: usize,
    /// The control end of the hand-back ring (#1332 S5, [`RETIRED_SHAPES`]): the retired plans'
    /// tables, which the service step drops here, on the control thread.
    retired_shapes: engine::realtime::Consumer<Box<PlanShape>>,
    /// The session's lifecycle word, shared with the render half (#1332 S5).
    lifecycle: Lifecycle,
    /// The control half's staging (#1332 S5): the control request, the diagnostic and the source
    /// staging.
    staging: ControlStaging,
    /// The result code of the last control call this half ran (#1332 S4). The single instance
    /// also publishes it in the status word, unchanged.
    last_result: u32,
    /// The bytes the bridge retains for the session's lifetime outside what the control plane
    /// holds and counts itself (issues #2194, #2199): the boot report's bridge rows less the boot
    /// model, less the boot plan's control rows (the control plane reports the newest plan's,
    /// [`SessionState::held_control_bytes`]) and less the boot document's staging (the staging is
    /// freed by the first control command and a replacement's document is its parse transient's).
    /// The boot plan's shape tables stay: a ceiling once that plan is reclaimed. A replacement's
    /// parse projection adds them to [`SessionState::held_retained_bytes`] and
    /// [`SessionState::held_control_bytes`].
    bridge_fixed_bytes: u64,
}

impl ControlHalf {
    /// The compiled session model, which the control plane retains (#1381a): read by the
    /// source-introspection queries (issue #207) and the addressing tables.
    ///
    /// The queries read it directly rather than copying a parallel table out of it, so the order
    /// they report *is* the normalized model's order by construction: `compile_session` sorts
    /// `sources` by stable ID, and there is no second list that could disagree with it.
    fn session(&self) -> &CompiledSession {
        self.control.committed_session()
    }

    /// Run one control command frame through the control plane (#1290a). When it publishes a
    /// replacement plan (the control plane's newest plan epoch advances exactly when a rebuild was
    /// prepared, committed and published), the plan carries its own tables to its adoption block
    /// and the rebuild is counted as a blocking rebuild (D6); a refused or live command counts
    /// nothing.
    ///
    /// `plan` is the render half's (#1589 R2): its active plan's companions hold the observation
    /// readers, which the swap block pairs, so the command borrows them, with that plan's epoch,
    /// for an `OBSERVATION_READ`; and before the first block a `SPECTRUM_CONFIGURE` has it adopt
    /// its successor in place (#1597), which the first block then renders with no swap.
    ///
    /// With the render half on another thread (#1332 S5) there is no plan to lend: the command
    /// runs as the C ABI's does, and the S6 Worker routes the two commands that need the plan.
    fn run_command(
        &mut self,
        plan: Option<&mut WebPlan>,
        request_len: usize,
        document: &[u8],
        output_capacity: u64,
    ) -> Result<usize, control_plane::CommandError> {
        let Self {
            control, staging, ..
        } = self;
        let request = staging
            .control_request
            .get(..request_len)
            .ok_or(control_plane::CommandError::Invalid)?;
        let before = control.newest_plan_epoch();
        let result = match plan {
            Some(plan) => control.command_with_plan(request, document, output_capacity, plan),
            None => control.command_with_document(request, document, output_capacity),
        };
        self.after_control(before, result)
    }

    /// Declare a discontinuity through the control plane (#1323), the function the C ABI's
    /// `miso_engine_v1_declare_discontinuity` runs. A plan it publishes is counted as
    /// [`Self::run_command`] counts a rebuild.
    fn declare_discontinuity(&mut self) -> Result<(), control_plane::CommandError> {
        self.run_control(WebSession::declare_discontinuity)
    }

    /// Seek the session through the control plane (#1643 D1): `SessionState::session_seek`, the
    /// call the C ABI's `miso_engine_v1_session_seek` and `_session_seek_at` run. The plain form
    /// declares the discontinuity in the same call (root ruling B16-R9), so a plan it publishes
    /// is counted as [`Self::declare_discontinuity`] counts one; the anchored form declares
    /// nothing and so publishes nothing.
    fn session_seek(
        &mut self,
        generation: u64,
        timeline_frame: u64,
        anchor_sample: Option<u64>,
        at_once: bool,
    ) -> Result<(), SessionSeekFailure> {
        if anchor_sample.is_some() {
            return self
                .control
                .session_seek(generation, timeline_frame, anchor_sample);
        }
        self.run_control(|control| {
            if at_once {
                control.session_seek_at_once(generation, timeline_frame)
            } else {
                control.session_seek(generation, timeline_frame, None)
            }
        })
    }

    /// Run one control-plane call that may publish a replacement plan (#1290a, #1323), then
    /// publish the words the render half reads (#1332 S4, S5): when the call published a plan,
    /// the control half keeps that plan's word writer and header address, and after every call
    /// that succeeded it publishes the newest plan's own words and the successor record.
    fn run_control<T, E>(
        &mut self,
        call: impl FnOnce(&mut WebSession) -> Result<T, E>,
    ) -> Result<T, E> {
        let before = self.control.newest_plan_epoch();
        let result = call(&mut self.control);
        self.after_control(before, result)
    }

    /// The bookkeeping of [`Self::run_control`] after its call: `before` is the newest plan epoch
    /// the call started from.
    fn after_control<T, E>(&mut self, before: u64, result: Result<T, E>) -> Result<T, E> {
        // Whatever the preparer left is taken: a command refused after its preparation leaves an
        // address and a writer nothing will read; the writer drops here, on the control thread.
        let prepared = self
            .record
            .lock()
            .map_or(None, |mut record| record.successor.take());
        let epoch = self.control.newest_plan_epoch();
        let published = result.is_ok() && epoch != before;
        if published {
            // A rebuild commits only after the preparer returned `Ok`, and the preparer returns
            // `Ok` only once it has left the header's address and the words' writer in the record
            // (it refuses the preparation, before anything is committed, when it cannot). So a
            // committed rebuild always has them, and no refusal can follow a commit. The previous
            // plan's writer drops here: that plan keeps the words it has. The published plan's
            // own words were stamped before its publication, by the commit itself, and nothing
            // after the commit changes them: they are not written again.
            let prepared = prepared.expect("a committed rebuild's preparer left the plan's words");
            self.successor_header = prepared.header;
            self.plan_words = prepared.words;
            if self.render_here {
                self.blocking_rebuilds = self.blocking_rebuilds.saturating_add(1);
            }
        }
        if result.is_ok() {
            self.sync_words(epoch, !published);
        }
        result
    }

    /// The newest plan's master and structure words (#1683, #1562b), after a call that succeeded:
    /// the strip the core designates now, by its ID in the newest committed session's strip order,
    /// which is the newest plan's (a rebuild prepares the plan from the session it commits, and no
    /// other commit adds, removes or reorders a strip), so a strip added or removed before the
    /// master never moves the reading to another strip; and the structure epoch the control list
    /// holds now. A plan that does not hold the strip has no master. They go to the newest plan's
    /// own words (`plan`: a call that published the plan leaves the words its commit stamped),
    /// and the successor record names the newest epoch and its header (#1332 S5). Control side
    /// only; each publication is a wait-free record store.
    fn sync_words(&mut self, newest_epoch: u64, plan: bool) {
        if plan {
            let master = master_index(
                self.control.committed_session().normalized_model(),
                self.control.master_strip(),
            );
            self.plan_words
                .publish(&plan_words(master, self.control.structure_epoch()));
        }
        let words: SuccessorWords = [newest_epoch, self.successor_header as u64];
        self.successor.publish(&words);
    }

    /// Drop the tables of the plans render retired (#1290a, #1332 S5), which it handed back
    /// through the ring at least one block after it adopted their successors. Control side only:
    /// it frees. Bounded by the ring's capacity and the pushes render makes meanwhile.
    fn reclaim_shapes(&mut self) {
        while let Ok(shape) = self.retired_shapes.try_pop() {
            drop(shape);
        }
    }

    /// Whether the session is ready, as the lifecycle word says (#1332 S5).
    fn ready(&self) -> bool {
        self.lifecycle.load(Ordering::Acquire) == STATE_READY
    }
}

/// The render half of the single instance (#1332 S4): the status the instance publishes, the
/// output buffer and, once a session is prepared, what renders it ([`RenderReady`]). Since #1332
/// S5 it touches no memory of the control half's, and the control half none of its own, so it can
/// render on another thread ([`RenderHost`]).
struct RenderHalf {
    /// The status the instance publishes (`miso_engine_web_v1_status_ptr`): the render half's
    /// clock and counters, and the result of the last call.
    status: WebStatus,
    buffers: Option<PreparedBuffers>,
    /// The diagnostic of the render half's own sticky failure (#1332 S5), always a fixed text,
    /// until the single instance copies it into the control half's diagnostic.
    failure: Option<&'static [u8]>,
    /// What renders the session, from boot until [`AudioWorkletEngineHost::dispose`].
    ready: Option<RenderReady>,
    /// Whether this is the status shell [`AudioWorkletEngineHost::release_render`] leaves
    /// (#1332 S5): a render call on it is refused and changes nothing, since the session renders
    /// elsewhere.
    released: bool,
    /// Whether the host was prepared with a meter window (issue #137 D2), so the lease is served
    /// (#1332 S6: the render half answers the lease wherever it renders).
    lease_supported: bool,
}

/// Everything the render half owns of one prepared session (#1332 S4).
///
/// Field order is the drop order: the tables and the meter delivery, then the plan owner (which
/// owns the source consumers and the active attachment), as the two dropped before #1332 S4.
/// Nothing here is ever dropped from `render_next`; see [`RenderHalf::fail`].
struct RenderReady {
    /// The tables of the plan that renders (#1290a): the boot plan's until the block that adopts a
    /// replacement, which swaps in the replacement's own from its companions.
    shape: Box<PlanShape>,
    /// The epoch of the plan `shape` describes.
    shape_epoch: u64,
    /// The tables the last adoption retired (#1332 S5), parked for one block: the next block
    /// hands them back to the control half before it renders ([`Self::hand_back_parked`]), so the
    /// adopting block's `process()` can still read the retired header (#1290d1).
    parked: Option<Box<PlanShape>>,
    /// The render end of the hand-back ring ([`RETIRED_SHAPES`]).
    retired_shapes: engine::realtime::Producer<Box<PlanShape>>,
    /// The reader of the successor record ([`SuccessorWords`]), which the control half publishes.
    successor: PublishedWordsReader<2>,
    /// The session's lifecycle word, shared with the control half (#1332 S5).
    lifecycle: Lifecycle,
    /// Test support (#1332 S5): a two-step barrier the next adopting block waits on twice, just
    /// before it takes the adopted plan's tables, so a test can commit on the control thread
    /// inside that window. Absent from the shipped module.
    #[cfg(feature = "test-support")]
    adoption_pause: Option<Arc<std::sync::Barrier>>,
    /// A handle on the session's `live_values_superseded` count (#1312 D2): the one count the
    /// control plane made, which every cell reader of every plan the session prepares adds to,
    /// and which the status reports (#1399).
    live_values_superseded: host_core::LiveValuesSuperseded,
    /// The session's one meter delivery (#1582 D1): the output meter's reader and gate (#1581 D5),
    /// the level the lease drives, the window grid, the generation, the loss count and the
    /// rendering plan's meter tables. The poll calls it and copies the frame it publishes into
    /// the plan's [`PlanShape::meter_frame`] and [`PlanShape::meter_header`].
    meters: MeterDelivery,
    /// The session's meter level word (#1588 D1): the lease sets [`MeterLevel::LEASE`], the
    /// control plane sets [`MeterLevel::TELEMETRY`] when it accepts a nonempty meter table. Only
    /// the render half applies it to [`Self::meters`] ([`Self::apply_meter_level`]).
    meter_level: MeterLevel,
    /// The word [`Self::apply_meter_level`] last applied: the clients the delivery serves now.
    meter_applied: u32,
    /// Test-only proof that incomplete master intervals never reach effect-reader scanning.
    #[cfg(test)]
    meter_effect_scan_count: u64,
    /// The render half of the plan exchange (#1381a D4): the plan owner that `render_next` renders
    /// through, and whose active plan's attachment holds the meter consumers and the observation
    /// readers.
    plan: WebPlan,
}

/// One compiled source's declared shape, in canonical normalized source order (issue #207).
///
/// The strict JSON declares one channel count and full-source frame count; sample rate is solely a
/// session-root fact and every prepared source begins at frame zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionSourceShape {
    /// Declared source channels; nonzero for every source a compiled session holds.
    pub channel_count: u32,
    /// Exact source length in sample frames; nonzero for every compiled source.
    pub frames: u64,
}

/// Safe ownership object backing one future AudioWorklet Wasm handle.
pub struct AudioWorkletEngineHost {
    options: WebBootOptions,
    /// The per-source ring the boot resolved (issue #1406b): the override, or on a zero override
    /// [`default_source_ring_frames`] at the session's rate and quantum.
    source_ring_frames: u32,
    /// The control frame the boot resolved (issue #2191): the boot word, or on a zero word
    /// [`DEFAULT_CONTROL_FRAME_BYTES`].
    control_frame_bytes: u32,
    /// The longest replacement document the boot admits (issue #2191, [`maximum_document_bytes`]).
    maximum_document_bytes: u32,
    resources: WebResourceReport,
    /// The watermark `read_watermark` last copied (#1381a D5), at a stable address in the
    /// control half: all zero until the first successful read.
    watermark: WebPlanWatermark,
    /// The plan latency `read_plan_latency` last copied (#1688 D2), at a stable address in the
    /// control half: all zero until the first successful read.
    plan_latency: WebPlanLatency,
    /// The seek report `read_seek_report` last copied (#1293 D6), at a stable address in the
    /// control half: all zero until the first successful read.
    seek_report: WebSeekReport,
    /// What the last control command left to read (issue #1562a D1).
    control_response: ControlResponse,
    /// The control half (#1332 S4): the control plane and what the control exports run. Declared
    /// before the render half, so it drops first. Dropped, with the render half's session, only by
    /// [`Self::dispose`].
    control: Option<ControlHalf>,
    /// The render half (#1332 S4): the status, the staging buffers and the plan owner that
    /// `render_next` renders through, beside the control half, so the two borrow apart. Boxed
    /// since #1332 S6, so a release and an adoption exchange two boxes and allocate nothing.
    render: Box<RenderHalf>,
    /// The status shell a release leaves in [`Self::render`] (#1332 S6), allocated at boot, so
    /// [`Self::release_render`] allocates nothing and [`Self::adopt_render`] frees nothing: it is
    /// here while the render half is, and in the host's `render` slot while the half is away.
    spare_shell: Option<Box<RenderHalf>>,
}

/// What [`AudioWorkletEngineHost::control_response`] reads (issue #1562a D1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ControlResponse {
    /// Nothing: no control command has run, or the last one was refused before it ran.
    None,
    /// The control plane's response frame of this length, in its response scratch.
    Frame(usize),
    /// The refusal's diagnostic, `diagnostic_len` bytes of the diagnostic buffer.
    Diagnostic,
}

impl AudioWorkletEngineHost {
    /// Parse, validate, self-configure, project, prepare and publish one running host.
    ///
    /// The document is interpreted once. Every refusal is typed and nothing is published until
    /// both the shared runtime and all bridge staging buffers exist.
    pub fn boot(document: &[u8], options: WebBootOptions) -> Result<Self, BootFailure> {
        Self::boot_with_command(document, options, None)
    }

    /// [`Self::boot`] with the boot command frame `miso_engine_web_v1_boot_command_ptr` staged
    /// (#2166): the control plane applies it before it prepares the first plan
    /// ([`control_plane::compile_children_with_boot_command`]), so a boot's spectrum taps are the
    /// first plan's and the boot counts no blocking rebuild. The browser decodes nothing in it.
    pub fn boot_with_command(
        document: &[u8],
        options: WebBootOptions,
        boot_command: Option<&[u8]>,
    ) -> Result<Self, BootFailure> {
        let document_bytes = u32::try_from(document.len()).map_err(|_| {
            BootFailure::fixed(RESULT_REFUSED_DOCUMENT, "web.document.maximum_bytes")
        })?;
        let options = validate_options(options)?;
        let control_frame_bytes = resolved_control_frame_bytes(&options)
            .ok_or_else(|| BootFailure::fixed(RESULT_INTERNAL, "web.internal.options"))?;
        let memory_budget = memory_budget(&options);
        if let Some(refusal) = document_length_refusal(&options, document_bytes) {
            return Err(refusal);
        }
        let document = core::str::from_utf8(document)
            .map_err(|_| BootFailure::fixed(RESULT_REFUSED_DOCUMENT, "web.document.utf8"))?;
        let model = parse_host_session(document)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        let compile_caps = CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        };
        let session = compile_host_model(&model, compile_caps)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        // Issue #2199: the parsed model is not read again, and the projection below allocates
        // beside the compiled one only.
        drop(model);
        let shape = compiled_session_shape(&session)
            .map_err(|failure| BootFailure::document(failure.into_bytes()))?;
        if (options.require_sample_rate_hz != 0
            && options.require_sample_rate_hz != shape.sample_rate_hz)
            || (options.require_quantum_frames != 0
                && options.require_quantum_frames != shape.quantum_frames)
        {
            return Err(BootFailure::fixed(
                RESULT_REPREPARE_REQUIRED,
                "host.session.shape",
            ));
        }
        let source_ring_frames = if options.source_ring_frames == 0 {
            default_source_ring_frames(shape.sample_rate_hz, shape.quantum_frames)
        } else {
            options.source_ring_frames
        };
        if source_ring_frames < shape.quantum_frames
            || !source_ring_frames.is_multiple_of(shape.quantum_frames)
        {
            return Err(BootFailure::fixed(
                RESULT_REFUSED_OPTIONS,
                "web.options.source_ring_frames",
            ));
        }
        let projection = project_buffers(
            document_bytes,
            shape.sample_rate_hz,
            shape.quantum_frames,
            control_frame_bytes,
        )?;
        let caps = prepare_caps(&session, options, source_ring_frames, memory_budget);
        let retained_projection = projected_retained_bytes(
            &session,
            options,
            &caps,
            memory_budget,
            control_frame_bytes,
            projection.report.bridge_retained_bytes,
            boot_command,
        )?;
        if retained_projection.total > memory_budget {
            return Err(BootFailure::retained_projection(
                retained_projection,
                memory_budget,
            ));
        }
        // The control plane compiles the session again and retains the one model the host reads
        // (#1381a). What this function holds of its own is freed first, so the boot's transient
        // peak is the one compile's, as it was before.
        drop(session);
        let (mut control, ready, resources) = compile_ready(
            document,
            &caps,
            options,
            memory_budget,
            control_frame_bytes,
            projection.report,
            boot_command,
        )?;
        let exact_retained = exact_retained_bytes(&resources)?;
        if exact_retained > memory_budget {
            return Err(BootFailure::exact_budget(
                "host.budget.retained_exact",
                exact_retained,
                memory_budget,
            ));
        }
        // #1349 D2: the status opens with the plan's initial watermark, `(initial revision, 0,
        // EXACT)`. No render is in flight at boot, so the read cannot be busy.
        let initial_watermark = control
            .queries
            .watermark()
            .map_err(|_| BootFailure::fixed(RESULT_INTERNAL, "web.internal.watermark"))?;
        let (staging, buffers) = allocate_buffers(projection)?;
        control.staging = staging;
        let backend = selected_backend();
        // Issue #137 D2: a lease over a host prepared with the boot word `0` is refused.
        let lease_supported = options.live_control_meter_blocks != 0;
        Ok(Self {
            options,
            source_ring_frames,
            control_frame_bytes,
            maximum_document_bytes: maximum_document_bytes(&options),
            resources,
            watermark: WebPlanWatermark {
                struct_size: WATERMARK_BYTES,
                ..WebPlanWatermark::default()
            },
            plan_latency: WebPlanLatency::default(),
            seek_report: WebSeekReport::default(),
            control_response: ControlResponse::None,
            control: Some(control),
            render: Box::new(RenderHalf {
                status: WebStatus {
                    struct_size: STATUS_BYTES,
                    abi_version: ABI_VERSION,
                    state: STATE_READY,
                    last_result: RESULT_OK,
                    backend,
                    sample_rate_hz: shape.sample_rate_hz,
                    quantum_frames: shape.quantum_frames,
                    blocking_rebuilds: 0,
                    next_absolute_sample: 0,
                    rendered_quanta: 0,
                    live_values_superseded: 0,
                    watermark_revision: initial_watermark.revision,
                    watermark_first_sample: initial_watermark.first_sample,
                    watermark_outcome_flags: initial_watermark.outcome_flags,
                },
                buffers: Some(buffers),
                failure: None,
                ready: Some(ready),
                released: false,
                lease_supported,
            }),
            spare_shell: Some(Box::new(RenderHalf::shell(lease_supported))),
        })
    }

    /// The control frame the boot resolved (issue #2191): the boot word, or on a zero word
    /// [`DEFAULT_CONTROL_FRAME_BYTES`]. A host bounds the frames it stages by this value.
    #[must_use]
    pub const fn control_frame_bytes(&self) -> u32 {
        self.control_frame_bytes
    }

    /// The longest replacement document the boot admits (issue #2191): the budget's parse
    /// projection limit, or the `maximum_document_bytes` word when that is smaller.
    #[must_use]
    pub const fn maximum_document_bytes(&self) -> u32 {
        self.maximum_document_bytes
    }

    /// Why a replacement document of `document_bytes` cannot be parsed beside the live session
    /// (issue #2194), or `None`: its parse transient
    /// (`document_bytes * PARSE_TRANSIENT_MULTIPLIER`, root ruling R2) plus what the live session
    /// retains (the bridge's fixed rows and every plan and model the control plane holds) over the
    /// budget is `host.budget.parse_projection`, naming the projection, the budget, the document
    /// and the live session's retained bytes; an internal fault reading the retained bytes is
    /// `web.internal.retained`. Control side only, before the staging allocates.
    pub(crate) fn replace_parse_refusal(&self, document_bytes: u32) -> Option<Vec<u8>> {
        let control = self.control.as_ref()?;
        let budget = memory_budget(&self.options);
        let arithmetic = || b"host.budget.arithmetic\t$\n".to_vec();
        let held = match control.control.held_retained_bytes() {
            Ok(held) => held,
            // A poisoned report lock or a missing model row is a platform fault, not a sum.
            Err(control_plane::CommandError::Internal) => {
                return Some(b"web.internal.retained\t$\n".to_vec());
            }
            Err(_) => return Some(arithmetic()),
        };
        let held_control = match control.control.held_control_bytes() {
            Ok(held) => held,
            Err(control_plane::CommandError::Internal) => {
                return Some(b"web.internal.retained\t$\n".to_vec());
            }
            Err(_) => return Some(arithmetic()),
        };
        let Some(live_retained) = held
            .checked_add(held_control)
            .and_then(|live| live.checked_add(control.bridge_fixed_bytes))
        else {
            return Some(arithmetic());
        };
        let Some(projection) =
            (u64::from(document_bytes) * PARSE_TRANSIENT_MULTIPLIER).checked_add(live_retained)
        else {
            return Some(arithmetic());
        };
        (projection > budget).then(|| {
            format!(
                "host.budget.parse_projection\t$.maximum_memory_bytes[projected_bytes={projection},budget_bytes={budget},document_bytes={document_bytes},live_retained_bytes={live_retained}]\n"
            )
            .into_bytes()
        })
    }

    /// Clear the control response before a replacement document is staged (issue #2194), so a
    /// staging refused without a diagnostic leaves none for the page to misread.
    pub(crate) fn clear_control_response(&mut self) {
        self.control_response = ControlResponse::None;
    }

    /// Refuse a replacement document's staging (issue #2194): `diagnostic` into the control half's
    /// diagnostic buffer, which [`Self::diagnostic`] reads, and as the control response, where the
    /// page reads a refused control call's diagnostic; nothing without a control half.
    pub(crate) fn refuse_replace_document(&mut self, diagnostic: &[u8]) {
        if let Some(control) = self.control.as_mut() {
            control.staging.write_diagnostic(diagnostic);
            self.control_response = ControlResponse::Diagnostic;
        }
    }

    /// Read the immutable boot options.
    #[must_use]
    pub const fn options(&self) -> &WebBootOptions {
        &self.options
    }

    /// The per-source ring frames the boot resolved (issue #1406b): what a zero override gives is
    /// the engine's own default ring, so a host bounds its sources by this value, not by a copy of
    /// the rule.
    #[must_use]
    pub const fn source_ring_frames(&self) -> u32 {
        self.source_ring_frames
    }

    /// Read fixed status without allocation.
    #[must_use]
    pub const fn status(&self) -> &WebStatus {
        &self.render.status
    }

    /// Read the current exact resource projection.
    #[must_use]
    pub const fn resources(&self) -> &WebResourceReport {
        &self.resources
    }

    /// The active plan's companions, for the tests that read the consumers and readers a plan
    /// carries (#1381a).
    #[cfg(test)]
    pub(crate) fn test_companions(&mut self) -> &mut RenderCompanions {
        self.plan_mut()
            .and_then(|plan| plan.active_attachment_mut::<RenderCompanions>())
            .expect("a ready host's plan carries its companions")
    }

    /// The control half, for the tests that read what it holds (#1332 S4).
    #[cfg(test)]
    pub(crate) fn test_control(&self) -> &ControlHalf {
        self.control
            .as_ref()
            .expect("a ready host has its control half")
    }

    /// The control half, mutably, for the tests (#1332 S4).
    #[cfg(test)]
    pub(crate) fn test_control_mut(&mut self) -> &mut ControlHalf {
        self.control
            .as_mut()
            .expect("a ready host has its control half")
    }

    /// The render half's session, for the tests that read what it holds (#1332 S4).
    #[cfg(test)]
    pub(crate) fn test_render(&self) -> &RenderReady {
        self.render
            .ready
            .as_ref()
            .expect("a ready host has its render half")
    }

    /// The render half's session, mutably, for the tests (#1332 S4).
    #[cfg(test)]
    pub(crate) fn test_render_mut(&mut self) -> &mut RenderReady {
        self.render
            .ready
            .as_mut()
            .expect("a ready host has its render half")
    }

    /// The session's `OBSERVATION_READ` index of the plan render runs (#1589 R2): the epoch of the
    /// render half's tables, whose readers the active plan's [`RenderCompanions`] hold. Test
    /// support: the tests find a tap's readers through it.
    #[cfg(test)]
    fn observation_index(&self) -> Option<&host_core::observation::ObservationIndex> {
        let epoch = self.render.ready.as_ref()?.shape_epoch;
        self.control.as_ref()?.control.observation_index(epoch)
    }

    /// The render half's plan owner, for the calls that need no render in flight.
    fn plan_mut(&mut self) -> Option<&mut WebPlan> {
        self.render.ready.as_mut().map(|ready| &mut ready.plan)
    }

    /// The tables of the plan that renders, or `None` for a host with no session.
    fn shape(&self) -> Option<&PlanShape> {
        self.render.ready.as_ref().map(|ready| &*ready.shape)
    }

    /// The observation handle of the effect at dense slot `effect`, from the active plan's
    /// companions (#1381a).
    #[cfg(test)]
    pub(crate) fn test_observation_handle(
        &mut self,
        effect: usize,
    ) -> Option<&EffectObservationHandle> {
        let reader = self
            .shape()?
            .effect_instances
            .get(effect)?
            .as_ref()?
            .reader?;
        self.test_companions()
            .effect_observations
            .get(reader as usize)
    }

    /// The [`ControlLimits`] the control half was built with (#1570b D6), read-only: the cross-host
    /// proof builds the C ABI's compile limits from it field for field, so both hosts advertise the
    /// same capabilities.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn control_limits_for_test(&self) -> Option<ControlLimits> {
        self.control.as_ref().map(|control| control.limits)
    }

    /// The epoch of the plan that renders (#1381a gates 2 and 4): 0 for the boot plan, and one
    /// more for each published plan the owner adopted.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn active_plan_epoch_for_test(&self) -> Option<u64> {
        self.render
            .ready
            .as_ref()
            .map(|ready| ready.plan.owner().active_epoch().0)
    }

    /// Whether no seek waits in a command slot of the newest committed session's timeline and
    /// sources (batch 16b review, MINOR f): a session seek one above the published generation
    /// passes its check, which a full slot refuses. `None` for a host with no control half. It
    /// shows a stray push on a host that cannot render the block that would empty the slot.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn session_seek_slots_empty_for_test(&self) -> Option<bool> {
        let control = &self.control.as_ref()?.control;
        let providers = control
            .test_pending_providers()
            .last()
            .unwrap_or_else(|| control.test_providers());
        let sources = providers.test_sources();
        let next = sources.published_generation().checked_add(1)?;
        Some(sources.check_session_seek(next, 0, None).is_ok())
    }

    /// The session timeline's sample at `source_read_sample` in the plan that renders (#1641's
    /// test reader, #1643 gates); `None` for a host with no plan, a plan with no timeline, or a
    /// sample the reader has no record for. Between render calls only.
    #[cfg(feature = "test-support")]
    pub fn timeline_at_for_test(&mut self, source_read_sample: u64) -> Option<i64> {
        let plan = self.plan_mut()?;
        host_core::test_only_timeline_at(
            plan.test_owner_mut().active_plan_mut(),
            source_read_sample,
        )
    }

    /// Render one block through the plan owner at `absolute_sample` rather than the host's clock,
    /// and nothing else (#1398 gate 1b); `false` when the owner refused it or the host is not
    /// ready. A mistimed block adopts a published successor before the owner checks its time, and
    /// is refused without publishing the epoch atomic: the window in which render has swapped
    /// plans and the control plane has not seen the epoch. The host's own records (clock, shape,
    /// status) do not move.
    #[cfg(feature = "test-support")]
    pub fn render_plan_at_for_test(&mut self, absolute_sample: u64) -> bool {
        let quantum = self.render.status.quantum_frames as usize;
        let RenderHalf { buffers, ready, .. } = &mut *self.render;
        let (Some(ready), Some(buffers)) = (ready.as_mut(), buffers.as_mut()) else {
            return false;
        };
        let plan = &mut ready.plan;
        PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum)
            .is_ok_and(|output| plan.render(absolute_sample, output).is_ok())
    }

    /// The staging buffer for one control command frame of `len` bytes (issue #1562a D1), or
    /// `None` when `len` is zero or exceeds the boot's resolved control frame
    /// ([`Self::control_frame_bytes`]) or the host is not ready. The page writes the frame here and
    /// calls [`Self::control_command`].
    pub fn control_request_mut(&mut self, len: u32) -> Option<&mut [u8]> {
        if !self.control_ready() || len == 0 || len > self.control_frame_bytes {
            return None;
        }
        self.control
            .as_mut()
            .map(|control| &mut *control.staging.control_request)
    }

    /// Run the staged control command frame of `len` bytes through the control plane (issue
    /// #1562a D1): the same [`control_plane::SessionState::command`] the C ABI's
    /// `miso_engine_v1_submit_command` runs, so any frame it accepts is accepted here: a
    /// transaction (live, model-only or monitor), the control list and state queries, the
    /// snapshot, counters and capabilities. The browser decodes nothing in the frame.
    ///
    /// This is control work: the worklet's message handler in `single` mode, never `process()`.
    /// Its allocations are the instance's counted control allocations. It does not touch the
    /// status's sticky state: a refusal leaves the host ready and the model, revision, overlay and
    /// cells as they were (D4, the acked-batch rule is the core's).
    ///
    /// Results: [`RESULT_OK`], with the response frame at [`Self::control_response`] (a refused
    /// transaction is still [`RESULT_OK`] with a non-ok response frame, as on the C ABI);
    /// [`RESULT_WRONG_STATE`] for a host that is not ready; [`RESULT_INVALID_ARGUMENT`] for a
    /// length that is zero or over the frame ceiling, or a frame the control plane cannot
    /// correlate; [`RESULT_BUFFER_TOO_SMALL`] for a response over the control frame;
    /// [`RESULT_BACKPRESSURE`] for a queue or plan with no room now; [`RESULT_COMPILE_REJECTED`]
    /// for a typed refusal before commit (a transaction that needs a plan rebuild is prepared and
    /// published like any other, #1562b); [`RESULT_REPREPARE_REQUIRED`] for a transaction that changes the
    /// session's sample rate or quantum (#1290a: the session is re-prepared from its document);
    /// [`RESULT_INTERNAL`] for a bookkeeping failure. For each failure the
    /// response is the refusal's diagnostic (`code\tpath\n` lines).
    ///
    /// A frame refused after the core has seen it has consumed its request ID: the same bytes sent
    /// again are answered [`RESULT_OK`] with a non-ok response frame whose code is
    /// `protocol.failure` (the replay cache), and a retry must take a new request ID to meet the
    /// refusal again.
    ///
    /// It is [`Self::control_command_with_document`] with an empty document, so a
    /// `SESSION_REPLACE` frame sent here is refused `session.replace.document_length`.
    pub fn control_command(&mut self, len: u32) -> u32 {
        self.control_command_with_document(len, &[])
    }

    /// [`Self::control_command`] with the document a `SESSION_REPLACE` frame names (#1697, #1386
    /// D7), borrowed for this call only: the core checks its length and SHA-256, parses it and
    /// diffs it against the committed model, exactly as for the C ABI's
    /// `miso_engine_v1_submit_document_command`, and every other command ignores it. The browser
    /// decodes nothing in it either. `miso_engine_web_v1_control_command` passes the document
    /// `miso_engine_web_v1_replace_document_ptr` staged. Results as [`Self::control_command`].
    pub fn control_command_with_document(&mut self, len: u32, document: &[u8]) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_WRONG_STATE);
        }
        if len == 0 || len > self.control_frame_bytes {
            return self.refuse_frame_length(len);
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        // #1332 S5: the render half's plan is lent only while it is here, on this thread.
        let result = control.run_command(
            self.render.ready.as_mut().map(|ready| &mut ready.plan),
            len as usize,
            document,
            u64::from(self.control_frame_bytes),
        );
        self.publish_words_here();
        match result {
            Ok(length) => {
                self.control_response = ControlResponse::Frame(length);
                self.record(RESULT_OK)
            }
            Err(error) => self.refuse_control(&error),
        }
    }

    /// Declare a discontinuity (#1323): the next block this host renders need not continue the
    /// previous output, because the page has stopped or is about to seek every source. The page
    /// calls it at a stop and before a seek of every source made with per-source seeks (a plain
    /// [`Self::session_seek`] declares it in the same call, root ruling B16-R9). It runs the
    /// control plane's
    /// `SessionState::declare_discontinuity`, the function the C ABI's
    /// `miso_engine_v1_declare_discontinuity` runs: if the newest plan runs behind its natural
    /// timing (a latency floor of a structural transaction raised it, #1285), the committed
    /// session is prepared again with no floors and only its sources carried, and published for
    /// the next block; otherwise nothing is published. The model and the revision do not change.
    /// Control work, as [`Self::control_command`]: never `process()`.
    ///
    /// Results: [`RESULT_OK`]; [`RESULT_WRONG_STATE`] for a host that is not ready; and the
    /// refusals of [`Self::control_command`] for a rebuild, with the core's diagnostic at
    /// [`Self::control_response`]. A refusal changes nothing.
    pub fn declare_discontinuity(&mut self) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let result = control.declare_discontinuity();
        self.publish_words_here();
        match result {
            Ok(()) => self.record(RESULT_OK),
            Err(error) => self.refuse_control(&error),
        }
    }

    /// A refused control call (#1562a D1, #1323): the core's diagnostic into the session
    /// diagnostic and the response, and the result code for `error`.
    /// Refuse a control frame of `len` bytes, zero or over the boot's control frame (issue
    /// #2191), with a diagnostic that names both, as the response. Control side only: the
    /// diagnostic is formatted on the control path, never in `process()`.
    fn refuse_frame_length(&mut self, len: u32) -> u32 {
        let limit = self.control_frame_bytes;
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let diagnostic =
            format!("web.control.frame_bytes\t$[frame_bytes={len},limit_bytes={limit}]\n");
        control.staging.write_diagnostic(diagnostic.as_bytes());
        self.control_response = ControlResponse::Diagnostic;
        self.record(RESULT_INVALID_ARGUMENT)
    }

    fn refuse_control(&mut self, error: &control_plane::CommandError) -> u32 {
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let diagnostic = error.diagnostic();
        control.staging.write_diagnostic(&diagnostic);
        self.control_response = ControlResponse::Diagnostic;
        self.record(match error {
            control_plane::CommandError::Invalid => RESULT_INVALID_ARGUMENT,
            control_plane::CommandError::BufferTooSmall { .. } => RESULT_BUFFER_TOO_SMALL,
            control_plane::CommandError::Backpressure => RESULT_BACKPRESSURE,
            // #1290a: a transaction that changes the rate or the quantum cannot be applied
            // to a running plan; the page re-prepares the session from its document.
            control_plane::CommandError::CompileRejected(_)
                if diagnostic.starts_with(control_plane::REPREPARE_REQUIRED.as_bytes()) =>
            {
                RESULT_REPREPARE_REQUIRED
            }
            control_plane::CommandError::CompileRejected(_) => RESULT_COMPILE_REJECTED,
            control_plane::CommandError::Internal => RESULT_INTERNAL,
        })
    }

    /// A refused source call (#1706 D3): the control plane's diagnostic for `failure` into the
    /// session diagnostic and the response, as a refused control call writes its own, and the
    /// result code of the failure's kind. It records nothing: each caller records exactly when
    /// its call records its results. The copy is into preallocated storage, so `submit_source`
    /// calls it from `process()`.
    pub(crate) fn refuse_source(&mut self, failure: SourceFailure) -> u32 {
        if let Some(control) = self.control.as_mut() {
            control
                .staging
                .write_diagnostic(failure.diagnostic().as_bytes());
            self.control_response = ControlResponse::Diagnostic;
        }
        match failure.kind() {
            SourceRefusalKind::InvalidArgument => RESULT_INVALID_ARGUMENT,
            SourceRefusalKind::Backpressure => RESULT_BACKPRESSURE,
            SourceRefusalKind::Internal => RESULT_INTERNAL,
        }
    }

    /// A source ID the staging cannot hold (#1706 D4): the core's malformed-ID refusal, unrecorded
    /// as `refuse_source`.
    pub(crate) fn refuse_source_id(&mut self) -> u32 {
        self.refuse_source(SourceFailure::Control(SourceControlError::InvalidId))
    }

    /// Dequeue the next event of `lane` (issue #1570b D1): `0` is the reliable lane, `1` the lossy
    /// lane, the C ABI's codes. This is the browser form of `miso_engine_v1_dequeue_event`: the
    /// same [`control_plane::SessionState::dequeue_event`], with the whole response scratch as
    /// capacity. The frame is then at [`Self::control_response`], which shares the staging of
    /// [`Self::control_command`] and is valid until the next control call. An empty lane leaves it
    /// empty (zero bytes).
    ///
    /// Control work, like the command entry: the worklet's message handler in `single` mode, never
    /// `process()`. Like [`Self::service`] it leaves the status's last result alone, so a drain
    /// never overwrites a refusal the host has yet to read.
    ///
    /// Results: [`RESULT_OK`]; [`RESULT_INVALID_ARGUMENT`] for a lane other than `0` or `1`, which
    /// dequeues nothing; [`RESULT_WRONG_STATE`] for a host that is not ready (and so
    /// [`RESULT_REFUSED_LIFECYCLE`], the same code, for an instance with no control half);
    /// [`RESULT_BACKPRESSURE`], [`RESULT_BUFFER_TOO_SMALL`] and [`RESULT_INTERNAL`] for the
    /// matching [`control_plane::EventError`]. A refusal leaves the response empty and writes the
    /// core's text for it ([`control_plane::EventError::diagnostic`]) into the session diagnostic.
    pub fn control_event(&mut self, lane: u32) -> u32 {
        self.control_response = ControlResponse::None;
        let lane = match lane {
            EVENT_LANE_RELIABLE => control_plane::EventLane::Reliable,
            EVENT_LANE_LOSSY => control_plane::EventLane::Lossy,
            _ => return RESULT_INVALID_ARGUMENT,
        };
        if !self.control_ready() {
            return RESULT_REFUSED_LIFECYCLE;
        }
        let Some(control) = self.control.as_mut() else {
            return RESULT_REFUSED_LIFECYCLE;
        };
        match control
            .control
            .dequeue_event(lane, u64::from(self.control_frame_bytes))
        {
            Ok(Some(length)) => {
                self.control_response = ControlResponse::Frame(length);
                RESULT_OK
            }
            Ok(None) => RESULT_OK,
            Err(error) => {
                control.staging.write_diagnostic(error.diagnostic());
                match error {
                    control_plane::EventError::BufferTooSmall { .. } => RESULT_BUFFER_TOO_SMALL,
                    control_plane::EventError::Backpressure => RESULT_BACKPRESSURE,
                    control_plane::EventError::Internal => RESULT_INTERNAL,
                }
            }
        }
    }

    /// What the last [`Self::control_command`] left to read (issue #1562a D1): its response frame
    /// after a [`RESULT_OK`], the refusal's diagnostic after another result, empty before any
    /// command. Valid until the next control call.
    #[must_use]
    pub fn control_response(&self) -> &[u8] {
        match (self.control_response, self.control.as_ref()) {
            (ControlResponse::Frame(length), Some(control)) => {
                control.control.command_response(length)
            }
            (ControlResponse::Diagnostic, Some(control)) => control.staging.diagnostic(),
            _ => &[],
        }
    }

    /// How many structural commands ran their rebuild on this instance's own thread (#1290a D6),
    /// saturating at `u32::MAX`: a page without cross-origin isolation runs the control half in
    /// the worklet's message handler, outside `process()`, so each published replacement plan is
    /// a blocking rebuild that decision 15 accepts only when it is counted and reported. A
    /// refused command, a live command and a command that fails after preparation count nothing.
    ///
    /// The status word [`WebStatus::blocking_rebuilds`] carries it to the page (#1294b), whose
    /// `miso.status.v1` reply names it `blockingRebuilds`; #1295's real-browser gate expects
    /// exactly one on the non-isolated leg and none on the isolated one.
    #[must_use]
    pub fn blocking_rebuilds(&self) -> u32 {
        self.control
            .as_ref()
            .map_or(0, |control| control.blocking_rebuilds)
    }

    /// The committed session revision (issue #1562a, carry-forward 2): what the SDK's transaction
    /// queue starts its expected revision from after a successful boot. It advances by one for
    /// each transaction the control plane commits, whatever its path, and is `0` for a session
    /// that was never edited and for a host that is not ready.
    #[must_use]
    pub fn committed_revision(&self) -> u64 {
        self.control.as_ref().map_or(0, |control| {
            control
                .control
                .committed_session()
                .normalized_model()
                .revision
        })
    }

    /// The epoch of the newest committed plan and the structure epoch of the committed control
    /// list, as the control half sees them after its last call (#1332 S5): what a plan published
    /// by that call carries as its own words. Test support.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn newest_plan_words(&self) -> Option<(u64, u64)> {
        self.control.as_ref().map(|control| {
            (
                control.control.newest_plan_epoch(),
                control.control.structure_epoch(),
            )
        })
    }

    /// Run the control plane's service step once (#1381a D5, #1348 D1): bring the provider epochs
    /// up to render and reclaim every retired plan and its attachment, here, never on the render
    /// thread. It commits and acknowledges nothing, and it leaves the status's last result alone,
    /// so a periodic tick never overwrites a refusal the host has yet to read.
    ///
    /// An epoch failure is sticky: the host fails with the session's diagnostic.
    pub fn service(&mut self) -> u32 {
        if !self.control_ready() {
            return RESULT_WRONG_STATE;
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        match control.control.service() {
            Ok(()) => {
                // #1332 S4, S5: the tables render handed back go with the plans the step
                // reclaimed, here, on the control thread.
                control.reclaim_shapes();
                RESULT_OK
            }
            Err(error) => {
                let diagnostic = error.diagnostic().into_owned();
                self.fail(RESULT_INTERNAL, &diagnostic)
            }
        }
    }

    /// Copy the plan's applied-revision watermark into the host's record (#1381a D5).
    ///
    /// A read that gives up after the watermark's bounded attempts returns
    /// [`RESULT_BACKPRESSURE`] and leaves the record untouched: a read to retry, never a refused
    /// edit. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_watermark(&mut self) -> u32 {
        if !self.control_ready() {
            return RESULT_WRONG_STATE;
        }
        let Some(control) = self.control.as_ref() else {
            return RESULT_WRONG_STATE;
        };
        let Ok(watermark) = control.queries.watermark() else {
            return RESULT_BACKPRESSURE;
        };
        self.watermark = WebPlanWatermark {
            struct_size: WATERMARK_BYTES,
            reserved0: 0,
            revision: watermark.revision,
            first_sample: watermark.first_sample,
            outcome_flags: watermark.outcome_flags,
            exact_count: watermark.exact,
            transition_fallback_count: watermark.transition_fallback,
            superseded_count: watermark.superseded,
            reserved: [0; 5],
        };
        RESULT_OK
    }

    /// Copy the active plan's output latency and tail into the host's record (#1688 D2).
    ///
    /// The words are the control plane's resource row for the active plan epoch, the same
    /// `PlanQueries::resources()` row the C ABI reports, with the tail in the control plane's
    /// two-word encoding: a successor's values appear from the first read after the block that
    /// adopts it. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_plan_latency(&mut self) -> u32 {
        if !self.control_ready() {
            return RESULT_WRONG_STATE;
        }
        let Some(control) = self.control.as_ref() else {
            return RESULT_WRONG_STATE;
        };
        let resources = control.queries.resources();
        let (tail_kind, tail_samples) = control_plane::tail_words(resources.output_tail_every_peak);
        self.plan_latency = WebPlanLatency {
            struct_size: PLAN_LATENCY_BYTES,
            reserved0: 0,
            latency_samples: resources.latency_samples,
            tail_kind,
            tail_samples,
            reserved: [0; 4],
        };
        RESULT_OK
    }

    /// The record [`Self::read_plan_latency`] fills.
    #[must_use]
    pub const fn plan_latency(&self) -> &WebPlanLatency {
        &self.plan_latency
    }

    /// Copy where the staged source landed after its newest seek into the host's record (#1293
    /// D6): the control plane's `SessionState::seek_report`, the call the C ABI's
    /// `miso_engine_v1_source_seek_report` runs, after its service step. Control side only: the
    /// message handler, never `process()`.
    ///
    /// A host that is not ready refuses with [`RESULT_REFUSED_LIFECYCLE`]; a read that landed
    /// inside render's publication on every bounded attempt is [`RESULT_BACKPRESSURE`] (read
    /// again); a source the session does not declare is [`RESULT_INVALID_ARGUMENT`]. Every refusal
    /// leaves the record alone. Like [`Self::service`] it leaves the status's last result alone.
    pub fn read_seek_report(&mut self, source_id: &[u8]) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return RESULT_REFUSED_LIFECYCLE;
        }
        let Some(control) = self.control.as_mut() else {
            return RESULT_REFUSED_LIFECYCLE;
        };
        match control.control.seek_report(source_id) {
            Ok(report) => {
                self.seek_report = WebSeekReport {
                    struct_size: SEEK_REPORT_BYTES,
                    reserved0: 0,
                    generation: report.generation,
                    first_sample: report.first_sample,
                    source_frame: report.source_frame,
                    cumulative_underrun_frames: report.cumulative_underrun_frames,
                    cumulative_held_frames: report.cumulative_held_frames,
                    reserved: [0; 2],
                };
                RESULT_OK
            }
            Err(failure) => self.refuse_source(failure),
        }
    }

    /// The record [`Self::read_seek_report`] fills.
    #[must_use]
    pub const fn seek_report(&self) -> &WebSeekReport {
        &self.seek_report
    }

    /// The staged source's ring occupancy as its producer sees it (issue #1387): the queued
    /// chunks in the high word, the chunks a submit would accept now in the low word, or
    /// [`SOURCE_FILL_REFUSED`] for a host that is not ready or a source the session does not
    /// declare. A pure read of the producer's queue cursors (no lock, no allocation, no service
    /// step), so the control Worker reads it while the worklet renders. It changes no status.
    #[must_use]
    pub fn source_fill(&self, source_id: &[u8]) -> u64 {
        if !self.control_ready() {
            return SOURCE_FILL_REFUSED;
        }
        let Some(control) = self.control.as_ref() else {
            return SOURCE_FILL_REFUSED;
        };
        match control.control.source_admission(source_id) {
            Ok(admission) => {
                (admission.queued.min(u64::from(u32::MAX)) << 32)
                    | admission.room.min(u64::from(u32::MAX - 1))
            }
            Err(_) => SOURCE_FILL_REFUSED,
        }
    }

    /// The record [`Self::read_watermark`] fills.
    #[must_use]
    pub const fn watermark(&self) -> &WebPlanWatermark {
        &self.watermark
    }

    /// Canonical normalized track order, the addressing authority for `track_index`.
    #[must_use]
    pub fn live_control_tracks(&self) -> &[Box<str>] {
        self.shape().map_or(&[], |shape| &shape.tracks)
    }

    /// Canonical normalized submix order (issue #1210 D1): the trailing segment of the strip order,
    /// submix `j` being strip `live_control_tracks().len() + j`, in the order of the meter frame's
    /// submix sections.
    #[must_use]
    pub fn live_control_submixes(&self) -> &[Box<str>] {
        self.shape().map_or(&[], |shape| &shape.submixes)
    }

    /// Number of sources the compiled session declares; zero before compilation (issue #207).
    ///
    /// This is the bounds authority for every other source query, exactly as
    /// [`AudioWorkletEngineHost::live_control_tracks`] is for `track_index`: the shape queries
    /// report a sentinel for an out-of-range index.
    #[must_use]
    pub fn session_source_count(&self) -> usize {
        self.control.as_ref().map_or(0, |control| {
            control.session().normalized_model().sources.len()
        })
    }

    /// One canonical source ID, or `None` for an out-of-range index (issue #207).
    ///
    /// The order is the normalized model's: `compile_session` sorts `sources` by stable ID, and
    /// this reads that list rather than a copy of it, so "canonical source order" has exactly one
    /// definition in this crate.
    #[must_use]
    pub fn session_source_id(&self, index: u32) -> Option<&str> {
        let control = self.control.as_ref()?;
        let source = control
            .session()
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(source.id.as_str())
    }

    /// One source's declared shape, or `None` for an out-of-range index (issue #207).
    #[must_use]
    pub fn session_source_shape(&self, index: u32) -> Option<SessionSourceShape> {
        let control = self.control.as_ref()?;
        let source = control
            .session()
            .normalized_model()
            .sources
            .get(index as usize)?;
        Some(SessionSourceShape {
            channel_count: u32::from(source.channels),
            frames: source.frames,
        })
    }

    /// The decimated meter frame (issue #137 D2, extended by #143 D5).
    ///
    /// `3(T + S) + 3` words (issue #1209 D1): two peaks per strip (the tracks, then the submixes in
    /// canonical order), master left and right, then one **non-negative** gain-reduction magnitude
    /// in decibels per strip in the same order and the master's. With no submixes every word is
    /// where it always was. Gain reduction is the latest fold from each independently aged effect
    /// observation and does not share the peak interval in [`WebMeterHeader`].
    ///
    /// The gain-reduction section is recomputed from the taps armed when each frame is polled. An
    /// unsubscribe disarms its tap when it is admitted, so every meter frame received after its
    /// `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an
    /// armed tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the
    /// frame leaves that tap out until a window that opens at or after that sample closes. A
    /// strip's slot (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows
    /// that opened at or after their last subscribe, and `0` when no armed tap has such a window.
    /// `masterGrDb` is `null` when the designated strip has none.
    #[must_use]
    pub fn meter_frame(&self) -> &[f32] {
        self.shape().map_or(&[], |shape| &shape.meter_frame)
    }

    /// The sample window and shape the `f32` frame cannot carry (issue #143 D5).
    #[must_use]
    pub fn meter_header(&self) -> &WebMeterHeader {
        self.shape()
            .map_or(&EMPTY_METER_HEADER, |shape| &shape.meter_header)
    }

    /// The full-width address of the header the rendering plan's header names as its successor,
    /// or `0` when none waits (a gate compares it with the candidate's or the adopted header's
    /// address; the header's own `u32` word reads `0` above 4 GiB on a native host).
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn meter_successor_address(&self) -> usize {
        self.shape().map_or(0, |shape| shape.successor_address)
    }

    /// Armed taps of the rendering plan, as their slots report them (#1585 D6): the taps render
    /// has armed. Off-ABI introspection for the tests.
    #[must_use]
    pub fn observation_armed_taps(&mut self) -> u32 {
        let Some(plan) = self.plan_mut() else {
            return 0;
        };
        plan.active_attachment_mut::<RenderCompanions>()
            .map_or(0, |companions| {
                companions
                    .effect_observations
                    .iter()
                    .flat_map(|handle| handle.readers.iter())
                    .filter(|reader| reader.read_state().is_some_and(|state| state.armed))
                    .count()
                    .try_into()
                    .unwrap_or(u32::MAX)
            })
    }

    /// Whether preparation bound any observation taps at all (issue #143 D3, level 1).
    #[must_use]
    pub fn observation_attached(&self) -> bool {
        self.shape().is_some_and(|shape| {
            shape
                .effect_instances
                .iter()
                .flatten()
                .any(|instance| instance.reader.is_some())
        })
    }

    /// Number of complete meter windows folded since compilation.
    #[must_use]
    pub fn meter_windows(&self) -> u64 {
        self.render
            .ready
            .as_ref()
            .map_or(0, |ready| ready.meters.folded_windows())
    }

    #[cfg(test)]
    pub(crate) fn meter_poll_work(&mut self) -> (u64, u64) {
        let scans = self
            .render
            .ready
            .as_ref()
            .map_or(0, |ready| ready.meter_effect_scan_count);
        let meters = self
            .plan_mut()
            .and_then(|plan| plan.active_attachment_mut::<RenderCompanions>())
            .map_or(0, |companions| {
                companions
                    .meters
                    .iter()
                    .map(|meter| meter.consumer.empty_count() + meter.consumer.success_count())
                    .sum()
            });
        (meters, scans)
    }

    /// Whether the host was prepared with a meter window (issue #137 D2). It is the
    /// configuration's word, not the plan's strip count: a session with no strip (#1682) is
    /// metered by the master alone, and takes the lease so that the first strip's meters are
    /// delivered from the block that adopts them.
    #[must_use]
    pub fn meters_attached(&self) -> bool {
        self.render.ready.is_some() && self.render.lease_supported
    }

    /// Take or release the meter lease. Returns the frozen result code.
    ///
    /// A lease over a host prepared with the boot word `0` is refused with [`RESULT_UNSUPPORTED`]:
    /// the caller asked for numbers this preparation does not deliver, and silently reporting
    /// zeros would be worse than saying so. The lease drives the output meter's gate (#1581 D5).
    ///
    /// The render half's own call (#1332 S6): with the half released, the shell left here refuses
    /// with [`RESULT_WRONG_STATE`].
    pub fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        let code = self.render.set_meter_lease(enabled);
        self.record(code)
    }

    /// Mutable source-ID staging storage, available only after preparation.
    pub fn source_id_mut(&mut self) -> Option<&mut [u8]> {
        self.control
            .as_mut()
            .map(|control| &mut *control.staging.source_id)
    }

    /// Mutable contiguous planar source PCM staging storage.
    pub fn source_pcm_mut(&mut self) -> Option<&mut [f32]> {
        self.control
            .as_mut()
            .map(|control| &mut *control.staging.source_pcm)
    }

    /// The valid fixed diagnostic prefix: the control half's (#1332 S5), into which the single
    /// instance also copies a render failure's.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        self.control
            .as_ref()
            .map_or(&[], |control| control.staging.diagnostic())
    }

    /// Contiguous dual-mono render output, left plane then right plane.
    #[must_use]
    pub fn output_pcm(&self) -> Option<&[f32]> {
        self.render.buffers.as_ref().map(|value| &*value.output_pcm)
    }

    /// Copy one canonical source ID into ID staging; returns its byte length (issue #207).
    ///
    /// Zero means "no such source", and also a staging that cannot hold the ID: staging is sized
    /// for the longest ID the grammar allows, so the second cannot happen for a compiled session.
    pub(crate) fn copy_session_source_id(&mut self, index: u32) -> u32 {
        let Some(ControlHalf {
            control, staging, ..
        }) = self.control.as_mut()
        else {
            return 0;
        };
        let Some(source) = control
            .committed_session()
            .normalized_model()
            .sources
            .get(index as usize)
        else {
            return 0;
        };
        Self::copy_id_into_staging(Some(staging), source.id.as_str().as_bytes()).unwrap_or(0)
    }

    /// Copy `bytes` into ID staging and return their length, or why they cannot be staged. Never
    /// panics: this runs inside a render-locked export, where a panic is a trap.
    fn copy_id_into_staging(
        buffers: Option<&mut ControlStaging>,
        bytes: &[u8],
    ) -> Result<u32, IdStagingError> {
        let buffers = buffers.ok_or(IdStagingError::NoStaging)?;
        let Some(target) = buffers.source_id.get_mut(..bytes.len()) else {
            return Err(IdStagingError::TooLong);
        };
        target.copy_from_slice(bytes);
        u32::try_from(bytes.len()).map_err(|_| IdStagingError::TooLong)
    }

    pub(crate) fn diagnostic_buffer_mut(&mut self) -> Option<&mut [u8]> {
        self.control
            .as_mut()
            .map(|control| &mut *control.staging.diagnostic)
    }

    pub(crate) fn ffi_source_staging_mut(&mut self) -> Option<FfiSourceStaging<'_>> {
        self.control.as_mut().map(|control| {
            let value = &mut control.staging;
            (
                &mut *value.source_pcm,
                &mut *value.plane_references,
                &mut *value.source_id,
            )
        })
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
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_WRONG_STATE);
        }
        // The boot's quantum, which the report keeps (#1332 S5: the control half reads no
        // render-half memory).
        if frames > self.resources.quantum_frames {
            return self.record(RESULT_INVALID_ARGUMENT);
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        // #1381a D2: through the control plane's source routing, which feeds the sources of the
        // newest committed session. It runs no service step: this is called from `process()` in the
        // PCM-feed worklet, and the service step locks and allocates (batch 7 review B1).
        let result = control.control.submit_unserviced(
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
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Queue one strictly increasing generation-tagged source seek (#2205): the C ABI's
    /// `miso_engine_v1_source_seek`. It goes through the control plane's source routing (#1381a
    /// D2) and never prepares the plan's consumer, also when the render half is on this instance:
    /// the consumer observes the seek through its command queue and lands it when ready (the old
    /// generation plays on until the new one has its runway, at most R blocks after the block that
    /// observed it). A live ring seek in
    /// `single` mode takes this path. A seek changes source-content position while the absolute
    /// render clock stays continuous; it therefore neither resets meter accumulators nor advances
    /// the meter publication epoch.
    pub fn seek_source(&mut self, source_id: &[u8], generation: u64, source_frame: u64) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let code = match control
            .control
            .seek_unserviced(source_id, generation, source_frame)
        {
            Ok(()) => RESULT_OK,
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Apply one strictly increasing generation-tagged source seek between render blocks, and
    /// prepare the plan's consumer at once (#2205): the suspended `prepare-seek` op's and the
    /// headless engine's seek, which have no live audio to keep. The seek goes through the control
    /// plane's source routing as [`Self::seek_source`]'s; this host also owns the render plan on its
    /// one thread, so a successful admission prepares the plan's consumer too
    /// (`SeekClock::Hold`: the seek applies now and the old generation's queued PCM is dropped), and
    /// new PCM can enter even when old queues were full.
    ///
    /// With the render half on another thread (#1332 S5) the plan is not here to prepare: the
    /// seek goes through the control plane's routing alone, exactly as [`Self::seek_source`].
    pub fn prepare_source_seek(
        &mut self,
        source_id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_WRONG_STATE);
        }
        let Some(control) = self.control.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        let Some(ready) = self.render.ready.as_mut() else {
            let code = match control
                .control
                .seek_unserviced(source_id, generation, source_frame)
            {
                Ok(()) => RESULT_OK,
                Err(failure) => self.refuse_source(failure),
            };
            return self.record(code);
        };
        let plan = &mut ready.plan;
        // #1325: the consumer to prepare is the plan that renders, by its own source rows. Through
        // a strip removal's phase 1 that plan still renders a source the newest committed session
        // removed, and its source indices are not the newest session's. A source it does not
        // render (one a waiting successor adds, or one removed and adopted) has no consumer here
        // to prepare; the control plane's routing answers it.
        let rendering = control
            .control
            .rendering_source_index(plan.owner().active_epoch().0, source_id);
        if rendering.is_some_and(|index| !plan.can_prepare_source_seek(index)) {
            return self.record(RESULT_WRONG_STATE);
        }
        let code = match control
            .control
            .seek_unserviced(source_id, generation, source_frame)
        {
            Ok(()) => match rendering {
                Some(index) if !plan.prepare_source_seek(index, generation, source_frame) => {
                    RESULT_INTERNAL
                }
                _ => RESULT_OK,
            },
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Queue one anchored source seek (#1293 D5): `source_frame` enters the graph in the block that
    /// starts at absolute render sample `anchor_sample`.
    ///
    /// It runs the control plane's `SessionState::seek_at`, the call the C ABI's
    /// `miso_engine_v1_source_seek_at` runs, after its service step, so it is a control-side call
    /// (the message handler, never `process()`). It never prepares the plan's consumer: the
    /// anchored seek is held by the ring and the consumer applies it at its anchor block, where
    /// `prepare_source_seek` would refuse a held seek. A host that is not ready refuses with
    /// [`RESULT_REFUSED_LIFECYCLE`] and changes nothing; the core's refusals map through
    /// `refuse_source` (an unknown source or an anchor that is not a quantum multiple is
    /// [`RESULT_INVALID_ARGUMENT`]).
    pub fn seek_source_at(
        &mut self,
        source_id: &[u8],
        generation: u64,
        source_frame: u64,
        anchor_sample: u64,
    ) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        }
        let Some(control) = self.control.as_mut() else {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        };
        let code = match control
            .control
            .seek_at(source_id, generation, source_frame, anchor_sample)
        {
            Ok(()) => RESULT_OK,
            Err(failure) => self.refuse_source(failure),
        };
        self.record(code)
    }

    /// Seek the timeline and every source of the newest committed session to timeline sample
    /// `timeline_frame` under one new `generation`, all or nothing (#1643 D1).
    ///
    /// It runs the control plane's `SessionState::session_seek`, the call the C ABI's
    /// `miso_engine_v1_session_seek` and `_session_seek_at` run, after its service step, so it is
    /// a control-side call (the message handler, never `process()`). With `anchor_sample`, every
    /// consumer applies the seek in the block that starts there. This adapter holds no seek
    /// logic, and it never prepares the plan's consumer: every consumer observes the seek through
    /// its command queue at the next block, the timeline included, and a plain seek lands for all
    /// of them in one block once every source that waits has its runway, or at the deadline
    /// (#2205). A host that is not ready
    /// refuses with [`RESULT_REFUSED_LIFECYCLE`] and changes nothing; the core's refusals map
    /// through `refuse_source` (a stale generation or an anchor that is not a quantum
    /// multiple is [`RESULT_INVALID_ARGUMENT`], one full slot [`RESULT_BACKPRESSURE`]). The plain
    /// form declares the discontinuity in the same call (root ruling B16-R9): a plan it publishes
    /// is kept and counted as [`Self::declare_discontinuity`]'s, and a refused declaration
    /// reports as that call's does, with nothing pushed. The anchored form declares nothing.
    pub fn session_seek(
        &mut self,
        generation: u64,
        timeline_frame: u64,
        anchor_sample: Option<u64>,
    ) -> u32 {
        self.session_seek_landing(generation, timeline_frame, anchor_sample, false)
    }

    /// [`Self::session_seek`]'s plain form for a seek with no live audio to keep (#2205): the
    /// headless engine's `seek`, or a browser engine whose transport is not playing. It lands for
    /// every source and the timeline at the first block that observes it, without waiting for a
    /// runway (`SessionState::session_seek_at_once`), in either instance mode: the landing is
    /// decided at that block, so a render half on another thread lands it the same way. Same
    /// results, declaration and thread rule as the plain form.
    pub fn session_prepare_seek(&mut self, generation: u64, timeline_frame: u64) -> u32 {
        let code = self.session_seek_landing(generation, timeline_frame, None, true);
        if code != RESULT_OK {
            return code;
        }
        // With the render half on this instance (`single` mode), prepare every consumer the
        // rendering plan holds now, as `prepare_source_seek` does for one source: each applies the
        // seek before the next block and drops the old generation's queued PCM, so a suspended
        // feed can refill the engine's ring before `play()`. A consumer whose ring did not get
        // this seek (a fresh ring of a pending candidate) is not ready and stays as it is; the
        // at-once landing still moves it at its next block.
        let (Some(control), Some(ready)) = (self.control.as_ref(), self.render.ready.as_mut())
        else {
            return code;
        };
        let plan = &mut ready.plan;
        let epoch = plan.owner().active_epoch().0;
        control
            .control
            .for_each_rendering_session_frame(epoch, timeline_frame, |index, frame| {
                if plan.can_prepare_source_seek(index) {
                    let _ready = plan.prepare_source_seek(index, generation, frame);
                }
            });
        code
    }

    fn session_seek_landing(
        &mut self,
        generation: u64,
        timeline_frame: u64,
        anchor_sample: Option<u64>,
        at_once: bool,
    ) -> u32 {
        self.control_response = ControlResponse::None;
        if !self.control_ready() {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        }
        let Some(control) = self.control.as_mut() else {
            return self.record(RESULT_REFUSED_LIFECYCLE);
        };
        let result = control.session_seek(generation, timeline_frame, anchor_sample, at_once);
        self.publish_words_here();
        match result {
            Ok(()) => self.record(RESULT_OK),
            Err(SessionSeekFailure::Seek(failure)) => {
                let code = self.refuse_source(failure);
                self.record(code)
            }
            // B16-R9: the plain form's declaration refused, reported as
            // [`Self::declare_discontinuity`] reports its own.
            Err(SessionSeekFailure::Declaration(error)) => self.refuse_control(&error),
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Render exactly one prepared quantum at the next absolute sample time: the render half's
    /// own call (#1332 S4). The single instance then copies a render failure's diagnostic into
    /// the control half's, the one diagnostic its page reads (#1332 S5): a fixed text into
    /// preallocated storage.
    pub fn render_next(&mut self) -> u32 {
        let code = self.render.render_next();
        if let Some(failure) = self.render.failure.take()
            && let Some(control) = self.control.as_mut()
        {
            control.staging.write_diagnostic(failure);
        }
        code
    }

    /// Drain every finished meter window into the frame buffer (issue #137 D2) and, while the
    /// protocol's meter table is on, into the rendering plan's frame ring (#1588 D2): the render
    /// half's own call (#1332 S4).
    ///
    /// Returns the number of complete windows folded by this call. Zero work happens while no
    /// client is on: without the lease, which a host prepared with
    /// `live_control_meter_blocks == 0` never grants, and without a meter table. The worklet
    /// calls it after each block it renders under the lease; with the table alone the render
    /// export folds after its block itself ([`Self::render_next`]).
    ///
    /// Allocation-free by construction: it moves `Copy` snapshots out of bounded queues into
    /// buffers allocated at compilation.
    pub fn poll_meters(&mut self) -> u32 {
        self.render.poll_meters()
    }
    // REALTIME_POLICY_END
    pub(crate) fn record_boundary_result(&mut self, code: u32) -> u32 {
        self.record(code)
    }

    /// Mark an observed browser output-shape mismatch as sticky reprepare-required.
    pub fn reject_output_quantum(&mut self, actual_frames: u32) -> u32 {
        let code = self.render.reject_output_quantum(actual_frames);
        self.record(code)
    }

    /// Release all prepared ownership. Repeated disposal is harmless.
    ///
    /// This runs in the worklet's `port.onmessage` handler, never inside `process()`; it is the
    /// single point where the plan, the compiled session and the source rings are freed: the
    /// control half first, then the render half's session, then the staging.
    ///
    /// A render half released to another thread ([`Self::release_render`]) must be adopted back
    /// first (S6's Worker rejoins before it disposes): what it owns is freed only where it is.
    ///
    /// A host whose render half is still away (a worklet that never handed it back) disposes its
    /// control half all the same: the lifecycle word goes to [`STATE_DISPOSED`] first, so that
    /// half silences from its next block, and what it owns stays with it (#1332 S6).
    pub fn dispose(&mut self) -> u32 {
        if let Some(control) = self.control.as_ref() {
            control.lifecycle.store(STATE_DISPOSED, Ordering::Release);
        }
        self.control = None;
        self.render.ready = None;
        self.render.buffers = None;
        self.render.failure = None;
        self.render.status.state = STATE_DISPOSED;
        self.record(RESULT_OK)
    }

    /// Release the render half to the thread that will render it (#1332 S5, the Rust form of
    /// S6's `host_release`): the plan owner, the meter delivery, the output buffer and the status
    /// it keeps. This host keeps the control half, whose calls from here on touch nothing of the
    /// render half's: the words and the lifecycle are shared atomics, the retired tables come
    /// back through the hand-back ring, and the staging is the control half's own. `None` when
    /// the render half is not here or no session is prepared.
    ///
    /// Control path. It allocates nothing (#1332 S6): the status shell it leaves was allocated
    /// at boot, and the release exchanges two boxes. The shell's status starts as the render
    /// half's was at the release; from then on it names the control half's results and the
    /// lifecycle state, while the render clock and counters are in the released half's status.
    pub fn release_render(&mut self) -> Option<RenderHost> {
        self.render.ready.as_ref()?;
        // #1332 S6: the shell was allocated at boot; the release exchanges two boxes.
        let mut shell = self.spare_shell.take()?;
        shell.status = self.render.status;
        core::mem::swap(&mut self.render, &mut shell);
        if let Some(control) = self.control.as_mut() {
            control.render_here = false;
        }
        Some(RenderHost { half: shell })
    }

    /// Take back a render half [`Self::release_render`] released (#1332 S5, the Rust form of
    /// S6's `host_adopt`), or hand it back unchanged when this host already has one, has no
    /// control half, or is not the session the render half renders. A render failure recorded
    /// while it was away becomes this host's diagnostic.
    ///
    /// It allocates and frees nothing (#1332 S6): the shell goes back to its spare slot.
    ///
    /// # Errors
    ///
    /// The render half itself, untouched, when it cannot be adopted.
    pub fn adopt_render(&mut self, render: RenderHost) -> Result<(), RenderHost> {
        let ours = match (self.control.as_ref(), render.half.ready.as_ref()) {
            (Some(control), Some(ready)) => Arc::ptr_eq(&control.lifecycle, &ready.lifecycle),
            _ => false,
        };
        if !ours || self.render.ready.is_some() || self.spare_shell.is_some() {
            return Err(render);
        }
        let RenderHost { mut half } = render;
        core::mem::swap(&mut self.render, &mut half);
        // `half` is the shell now, and the spare slot is empty (checked above): an exchange, and
        // the `None` it returns is forgotten, so no drop glue joins the call.
        let mut slot = Some(half);
        core::mem::swap(&mut self.spare_shell, &mut slot);
        core::mem::forget(slot);
        if let Some(control) = self.control.as_mut() {
            control.render_here = true;
        }
        if let Some(failure) = self.render.failure.take()
            && let Some(control) = self.control.as_mut()
        {
            control.staging.write_diagnostic(failure);
        }
        Ok(())
    }

    /// Whether the render half is here (#1332 S6): `false` from a release to the adoption back.
    #[must_use]
    pub fn render_here(&self) -> bool {
        !self.render.released
    }

    /// The address of the meter header of the newest replacement plan the control half
    /// committed, or `0` before the first (#1332 S6, the meter-view hook for S7). The Worker reads
    /// it after a control call that may publish a plan and names it to the worklet, whose message
    /// handler makes sure its views cover the header and its frame before render adopts the plan.
    /// Only an address: nothing here dereferences it, and after a supersession it may name tables
    /// already freed (the successor record's rule, #1290d1).
    #[must_use]
    pub fn successor_header_address(&self) -> usize {
        self.control
            .as_ref()
            .map_or(0, |control| control.successor_header)
    }

    /// Whether the control half is here and the session is ready, as the lifecycle word says
    /// (#1332 S5): the check every control call makes, which reads no render-half memory.
    fn control_ready(&self) -> bool {
        self.control.as_ref().is_some_and(ControlHalf::ready)
    }

    /// The single instance's hand-over after a control call (#1332 S5): with the render half
    /// here, on this thread, it reads the words the call published at once, so the header names
    /// them before the next block, as before the split. A render half on another thread reads
    /// them before its next block.
    fn publish_words_here(&mut self) {
        if let Some(ready) = self.render.ready.as_mut() {
            ready.publish_words();
        }
    }

    /// Set the lifecycle state of both halves, for the tests that drive a refusal of a host that
    /// is not ready (#1332 S5).
    #[cfg(test)]
    pub(crate) fn set_state_for_test(&mut self, state: u32) {
        self.render.status.state = state;
        if let Some(control) = self.control.as_ref() {
            control.lifecycle.store(state, Ordering::Release);
        }
    }

    /// Every control call returns its result through here, so the status's `blocking_rebuilds`
    /// word (#1294b) is refreshed in this one place: current after the control call that rebuilt,
    /// zero after a dispose (the control half is gone), as [`Self::blocking_rebuilds`] reports.
    /// The control half records its own result (#1332 S4), and the single instance publishes it in
    /// the status unchanged.
    fn record(&mut self, code: u32) -> u32 {
        if let Some(control) = self.control.as_mut() {
            control.last_result = code;
            // #1332 S6: with the render half away, the shell's state is the lifecycle word's, so
            // the Worker's status shows a failure of either half.
            if self.render.released {
                self.render.status.state = control.lifecycle.load(Ordering::Acquire);
            }
        }
        self.render.status.last_result = code;
        self.render.status.blocking_rebuilds = self.blocking_rebuilds();
        code
    }

    /// Record a sticky failure of a control call without ever releasing ownership (#1332 S5): the
    /// lifecycle word, which a render half on another thread reads before its next block, and
    /// the control half's diagnostic; with the render half here, its status and silence at once
    /// ([`RenderHalf::silence`]); then the control half's record.
    fn fail(&mut self, code: u32, diagnostic: &[u8]) -> u32 {
        if let Some(control) = self.control.as_mut() {
            control.lifecycle.store(STATE_FAILED, Ordering::Release);
            control.staging.write_diagnostic(diagnostic);
        }
        self.render.silence();
        self.record(code)
    }
}

impl RenderReady {
    // REALTIME_POLICY_BEGIN: #1332 S4, S5, the render half's application of the control half's words.
    /// Apply the control half's words to the rendering plan's tables (#1332 S4, S5): the plan's
    /// own master and structure words ([`PlanShape::words`]), and, from the successor record,
    /// the successor its header names while a newer plan waits, `0` otherwise (#1290d1). Two
    /// bounded seqlock reads and scalar writes; a read that lands inside a publication on every
    /// attempt leaves the words as they were, for the next block to read.
    fn publish_words(&mut self) {
        let shape = &mut *self.shape;
        if let Ok([master_plus_one, structure_epoch]) = shape.words.read() {
            let master = master_plus_one
                .checked_sub(1)
                .and_then(|track| u32::try_from(track).ok());
            shape.master_track = master;
            shape.meter_header.master_track_plus_one =
                master.map_or(0, |track| track.saturating_add(1));
            shape.meter_header.structure_epoch = structure_epoch;
        }
        if let Ok([newest_epoch, header]) = self.successor.read() {
            let successor = if newest_epoch > self.shape_epoch {
                usize::try_from(header).unwrap_or(0)
            } else {
                0
            };
            shape.meter_header.successor_pointer = u32::try_from(successor).unwrap_or(0);
            #[cfg(any(test, feature = "test-support"))]
            {
                shape.successor_address = successor;
            }
        }
    }

    /// Hand the tables the previous block's adoption parked back to the control half (#1332 S5):
    /// one push into the hand-back ring, before this block renders, so they are freed on the
    /// control thread, and never before the adopting block's `process()` has returned. A full
    /// ring keeps them parked (see [`RETIRED_SHAPES`]). Pointer moves only: it frees nothing.
    fn hand_back_parked(&mut self) {
        if self.parked.is_none() || self.retired_shapes.available_capacity() == 0 {
            return;
        }
        let Some(shape) = self.parked.take() else {
            return;
        };
        if let Err(QueueFull { value, .. }) = self.retired_shapes.try_push(shape) {
            // Unreachable (the room was checked, and only the control half's pops change it),
            // and still a move: the tables go back to the empty slot, and what the slot held,
            // `None`, is forgotten rather than dropped, so no drop glue joins the render call.
            let mut back = Some(value);
            core::mem::swap(&mut self.parked, &mut back);
            core::mem::forget(back);
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN: #1588 D1, the render half's application of the meter level word.
    /// Apply the session's meter level word from `next_sample`, the next block's first sample
    /// (#1588 D1). When the word differs from the one last applied, the delivery's level follows
    /// whether any client is on, and a change that keeps it on restarts the delivery epoch
    /// ([`MeterDelivery::restart`]): either client's change restarts the epoch once, and a client
    /// alone sees exactly the level transitions it asks for. The render half calls it before
    /// each block, so a configure the control half accepted between two blocks turns the level
    /// on from the next block's first sample, as the C ABI's requested gate does, and the lease
    /// export calls it at once. Returns whether the publication was cleared, so the caller
    /// republishes the lease frame. One `Acquire` load; allocation-free.
    fn apply_meter_level(&mut self, next_sample: u64) -> bool {
        let word = self.meter_level.load();
        if word == self.meter_applied {
            return false;
        }
        self.meter_applied = word;
        let on = word != 0;
        self.meters.set_level(on, next_sample) || (on && self.meters.restart(next_sample))
    }

    /// Copy the delivery's frame into the lease frame and its header (#1582 D4).
    fn publish_lease_frame(&mut self) {
        publish_web_meter_frame(
            self.meters.frame(),
            self.shape.master_track,
            &mut self.shape.meter_frame,
            &mut self.shape.meter_header,
        );
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Take the tables of the plan whose epoch is `epoch` from its companions (#1290a, #1332 S4),
    /// at the block that adopted it. The tables it retires are parked in [`Self::parked`] for
    /// one block (#1332 S5). Pointer exchanges: it
    /// allocates and frees nothing, and indexes nothing, so it adds no trap to the render export.
    ///
    /// The companions' `carried` is the pairing the swap block's attachment hook made (#1381b):
    /// the readers the executors carried, which this hand-over follows with the meter delivery's
    /// per-meter state (a carried tap's arm state is in its slot, #1585 D6). A swap that carried
    /// a meter keeps the meter delivery running, because the carried meters keep their window grid
    /// and the host's master windows are continuous; one that carried none restarts it, because
    /// its meters open their first window at the adoption block. A swap that adds or removes a
    /// strip in front shifts the meters' handles (a handle is a position): each carried meter
    /// remembers the handle its predecessor published under ([`MeterDelivery::adopt`], which takes
    /// the adopted plan's [`PlanShape::meter_tables`]), so a window closed before the swap,
    /// pending or still queued, is accepted by the strip it was carried to, not dropped.
    ///
    /// The meter frame travels with the tables (#1290d1): the adopted plan's frame and header are
    /// the published ones from this block on. The retiring header is marked
    /// ([`WebMeterHeader::retired`]) and names the adopted one, so the page's render path switches
    /// to the views it cut over them in advance. The control half's words then apply to the
    /// adopted plan ([`Self::publish_words`]). Pointer exchanges and scalar writes only.
    fn adopt_shape(&mut self, epoch: u64, first_sample: u64) {
        self.shape_epoch = epoch;
        let Self {
            plan,
            shape,
            meters,
            parked,
            ..
        } = self;
        let Some(companions) = plan.active_attachment_mut::<RenderCompanions>() else {
            return;
        };
        let RenderCompanions {
            shape: slot,
            carried,
            carried_len,
            ..
        } = companions;
        // What the swap block's hook paired (#1381b); empty when nothing was.
        let carried = carried.get(..*carried_len as usize).unwrap_or(&[]);
        let meters_carried = carried
            .iter()
            .any(|reader| reader.kind == ObserverKind::Meter);
        let Some(incoming) = slot.as_mut() else {
            debug_assert!(false, "an adopted plan carries no tables");
            return;
        };
        core::mem::swap(shape, incoming);
        // `shape` is the adopted plan's and `incoming` the retired one's, in the adopted plan's
        // companions. The stream the page reads continues in the adopted plan's header (#1290d1):
        // its window, sequence, generation and loss are the stream's, its strip counts and frame
        // are the plan's. The retired header says so and names the adopted one, so the page's
        // render path switches to the views it cut in advance.
        let retired = incoming;
        let stream = retired.meter_header;
        retired.meter_header.retired = 1;
        let adopted_header = core::ptr::from_ref(&shape.meter_header).addr();
        retired.meter_header.successor_pointer = u32::try_from(adopted_header).unwrap_or(0);
        #[cfg(any(test, feature = "test-support"))]
        {
            retired.successor_address = adopted_header;
        }
        shape.meter_header.windows = stream.windows;
        shape.meter_header.first_sample = stream.first_sample;
        shape.meter_header.end_sample = stream.end_sample;
        shape.meter_header.sequence = stream.sequence;
        shape.meter_header.master_gr_present = stream.master_gr_present;
        shape.meter_header.reserved = stream.reserved;
        // #1588 D3: the adopted plan's frames name its epoch, the epoch of the ring its tables
        // carry, which its preparer could not know.
        debug_assert!(
            shape.meter_ring.is_some(),
            "a published plan has no meter ring"
        );
        if let Some(ring) = shape.meter_ring.as_mut() {
            ring.epoch = epoch;
        }
        // The delivery adopts the plan's meter tables (#1582 R1): the pending strip snapshots
        // follow their meters, and a swap that carried no meter restarts the delivery at this
        // block, on a new grid (#1581 D5). The retired tables come back in the adopted shape's
        // slot and go to the retired one, by pointer exchange. Every plan's tables are built with
        // its meter tables (`build_plan_shape`), and only this exchange takes them.
        debug_assert!(
            shape.meter_tables.is_some(),
            "an adopted plan shape has no meter tables"
        );
        if let Some(tables) = shape.meter_tables.as_mut() {
            tables.set_plan_epoch(epoch);
            meters.adopt(
                tables,
                carried
                    .iter()
                    .filter(|reader| reader.kind == ObserverKind::Meter)
                    .map(|reader| MeterCarry {
                        successor: reader.successor,
                        predecessor: reader.predecessor,
                        successor_handle: reader.successor_handle,
                        predecessor_handle: reader.predecessor_handle,
                    }),
                meters_carried,
                first_sample,
            );
        }
        core::mem::swap(&mut shape.meter_tables, &mut retired.meter_tables);
        // A carried tap's arm state needs no hand-over: it is in the tap's slot, which moves with
        // the tap (#1585 D6). The retired tables leave the adopted plan's companions and are
        // parked for one block, by exchange with the empty slot (#1332 S5): the next block hands
        // them back to the control half.
        if parked.is_none() {
            core::mem::swap(parked, slot);
        }
        self.publish_words();
    }
    // REALTIME_POLICY_END
}

#[cfg(feature = "test-support")]
impl RenderReady {
    /// Wait twice on the adoption barrier a test set, once: render has reached the adoption
    /// block, then the test releases it (#1332 S5). Test support only.
    fn pause_before_adoption(&mut self) {
        if let Some(barrier) = self.adoption_pause.take() {
            barrier.wait();
            barrier.wait();
        }
    }
}

impl RenderHalf {
    /// The status shell a release leaves in the host (#1332 S6), allocated at boot: no session,
    /// no buffers, `released`. Its status is the render half's, copied at the release.
    fn shell(lease_supported: bool) -> Self {
        Self {
            status: WebStatus {
                struct_size: STATUS_BYTES,
                abi_version: ABI_VERSION,
                state: STATE_READY,
                last_result: RESULT_OK,
                backend: 0,
                sample_rate_hz: 0,
                quantum_frames: 0,
                blocking_rebuilds: 0,
                next_absolute_sample: 0,
                rendered_quanta: 0,
                live_values_superseded: 0,
                watermark_revision: 0,
                watermark_first_sample: 0,
                watermark_outcome_flags: 0,
            },
            buffers: None,
            failure: None,
            ready: None,
            released: true,
            lease_supported,
        }
    }

    // REALTIME_POLICY_BEGIN: #1332 S6, the lease and the output-shape check on the render half.
    /// [`AudioWorkletEngineHost::set_meter_lease`] on this half: wherever it renders, and refused
    /// with [`RESULT_WRONG_STATE`] on the shell a release leaves.
    fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        let from = self.status.next_absolute_sample;
        let lease_supported = self.lease_supported;
        let Some(ready) = self.ready.as_mut() else {
            // The shell: the session renders elsewhere.
            return self.record(RESULT_WRONG_STATE);
        };
        if enabled && !lease_supported {
            return self.record(RESULT_UNSUPPORTED);
        }
        // The lease is the core delivery's level (#1582 D4). Repeated acquisition/release is
        // idempotent. A real transition starts a new publication generation, clears the pending
        // state and the publication, marks producer-queued intervals from the previous lease for
        // bounded rejection by the next poll, and drives the output meter's gate (#1581 D5); the
        // cleared publication is copied into the page's frame and header.
        // #1588 D1: the lease is its own bit of the session's level word, applied here at once,
        // so a lease alone moves the level exactly as before.
        ready.meter_level.set(MeterLevel::LEASE, enabled);
        if ready.apply_meter_level(from) {
            ready.publish_lease_frame();
        }
        self.record(RESULT_OK)
    }

    /// [`AudioWorkletEngineHost::reject_output_quantum`] on this half.
    fn reject_output_quantum(&mut self, actual_frames: u32) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        if actual_frames == self.status.quantum_frames {
            return self.record(RESULT_OK);
        }
        self.status.state = STATE_FAILED;
        if let Some(ready) = self.ready.as_ref() {
            ready.lifecycle.store(STATE_FAILED, Ordering::Release);
        }
        self.record(RESULT_REPREPARE_REQUIRED)
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`AudioWorkletEngineHost::render_next`]: render exactly one prepared quantum at the next
    /// absolute sample time.
    fn render_next(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return self.record(RESULT_WRONG_STATE);
        }
        // Lossless on every supported target (wasm32 and every 64-bit host); `try_from` would
        // leave an `expect_failed` trap owner in the render export's call graph.
        let quantum = self.status.quantum_frames as usize;
        let Some(ready) = self.ready.as_mut() else {
            // #1332 S5: the shell a release leaves renders nothing and records no failure, so
            // the shared lifecycle word and this status cannot disagree.
            if self.released {
                return self.record(RESULT_WRONG_STATE);
            }
            return self.fail(RESULT_INTERNAL, b"web.internal.ready\t$\n");
        };
        // #1332 S5: a sticky failure the control half recorded, on whatever thread it runs, is
        // this half's from this block: silence, as the single instance's failure makes it at once.
        if ready.lifecycle.load(Ordering::Acquire) != STATE_READY {
            self.silence();
            return self.record(RESULT_WRONG_STATE);
        }
        // #1332 S5: the tables the last block's adoption retired go back to the control half,
        // and the words the control half published since the last block apply to this one.
        ready.hand_back_parked();
        ready.publish_words();
        let Some(buffers) = self.buffers.as_mut() else {
            return self.fail(RESULT_INTERNAL, b"web.internal.buffers\t$\n");
        };
        let output = match PlanarBufferMut::try_new(&mut buffers.output_pcm, 2, quantum, quantum) {
            Ok(value) => value,
            Err(_) => return self.fail(RESULT_INTERNAL, b"web.internal.output\t$\n"),
        };
        let render_first_sample = self.status.next_absolute_sample;
        // #1588 D1: a meter client the control half turned on or off since the last block takes
        // effect from this block's first sample, before the block is observed.
        if ready.apply_meter_level(render_first_sample)
            && ready.meter_applied & MeterLevel::LEASE != 0
        {
            ready.publish_lease_frame();
        }
        // #1381a D4: through the plan owner, which adopts a published successor at this block's
        // boundary and continues the clock.
        let result = ready.plan.render(self.status.next_absolute_sample, output);
        match result {
            Ok(watermark) => {
                // #1349 D1/D2: copied from the report of the block that advanced the watermark;
                // a block that did not advance it leaves the three words as they were.
                if let Some(watermark) = watermark {
                    self.status.watermark_revision = watermark.revision;
                    self.status.watermark_first_sample = watermark.first_sample;
                    self.status.watermark_outcome_flags = watermark.outcome_flags;
                }
                let next_absolute_sample = ready.plan.owner().next_absolute_sample();
                // #1290a: the block that adopted a replacement plan takes the replacement's own
                // tables from its companions, a pointer exchange that allocates and frees nothing;
                // the control side drops the retired ones at its next step.
                let epoch = ready.plan.owner().active_epoch().0;
                if epoch != ready.shape_epoch {
                    // #882: the host's carried-meter fact (`adopt_shape`'s `meters_carried`) is the
                    // engine's, which chose this block's strip meter level.
                    debug_assert_eq!(
                        ready
                            .plan
                            .active_attachment_mut::<RenderCompanions>()
                            .is_some_and(|companions| companions
                                .carried()
                                .iter()
                                .any(|reader| reader.kind == ObserverKind::Meter)),
                        ready.plan.owner().active_strip_meters_carried(),
                        "the host and the engine disagree on whether the swap carried a meter"
                    );
                    // #1381b: what the swap block's hook paired travels with the tables, which the
                    // adopted plan's companions carry (#1332 S4).
                    #[cfg(feature = "test-support")]
                    ready.pause_before_adoption();
                    ready.adopt_shape(epoch, render_first_sample);
                }
                self.status.next_absolute_sample = next_absolute_sample;
                self.status.rendered_quanta = self.status.rendered_quanta.saturating_add(1);
                // #1570b D3: the block's activity, published by the control plane's one rule
                // (the peak scan runs only for an armed observer, `NaN` otherwise), so the
                // service step stages the telemetry events and `TRANSPORT_GET` reports the
                // effective sample exactly as on the C ABI. The same call folds the session's
                // output meter (#1581 D5), whose gate the lease drives: the master peaks the poll
                // pairs with the strip windows. At most `4 * quantum` loads, no allocation,
                // indexed with `get` so no `slice_index_fail` joins the call graph.
                if let (Some(left), Some(right)) = (
                    buffers.output_pcm.get(..quantum),
                    buffers
                        .output_pcm
                        .get(quantum..)
                        .and_then(|tail| tail.get(..quantum)),
                ) {
                    ready.plan.observe_rendered_block(
                        [left, right],
                        render_first_sample,
                        next_absolute_sample,
                    );
                }
                // #1588 D2: with the protocol's meter table alone, the render half folds after
                // its block here, since the worklet polls only under the lease.
                if ready.meter_applied == MeterLevel::TELEMETRY {
                    poll_meter_delivery(ready, next_absolute_sample);
                }
                // #1399: the session's count, loaded after the block that added to it, so the
                // status is never behind the last rendered block.
                self.status.live_values_superseded = ready.live_values_superseded.get();
                self.record(RESULT_OK)
            }
            Err(_) => self.fail(RESULT_RENDER_REJECTED, b"web.render.rejected\t$\n"),
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`AudioWorkletEngineHost::poll_meters`].
    fn poll_meters(&mut self) -> u32 {
        if self.status.state != STATE_READY {
            return 0;
        }
        let next_sample = self.status.next_absolute_sample;
        let Some(ready) = self.ready.as_mut() else {
            return 0;
        };
        poll_meter_delivery(ready, next_sample)
    }
    // REALTIME_POLICY_END

    /// The render half's record of a call it ran: the status's last result. Only a control call
    /// moves `blocking_rebuilds`, and it refreshes the word itself.
    fn record(&mut self, code: u32) -> u32 {
        self.status.last_result = code;
        code
    }

    /// Record a sticky failure without ever releasing ownership.
    ///
    /// `self.ready` (with the host's control half) **is** this host's one-slot retirement queue
    /// (AGENTS.md: "the displaced plan goes to a bounded retirement queue and is reclaimed off the
    /// render thread"). `fail` is reachable from `render_next`, which the AudioWorklet calls on the
    /// rendering thread, so it must not move, overwrite or drop the plan, the compiled session or
    /// the source rings: dropping them would free `Arc<spsc::Ring<_>>` payloads and run
    /// `dlmalloc::free` inside `process()`. The ownership stays exactly where it is and only
    /// [`AudioWorkletEngineHost::dispose`] — a control path call, delivered on `port.onmessage` —
    /// reclaims it.
    ///
    /// A separate `retired` field would not help: `self.retired = self.ready.take()` compiles to a
    /// conditional `drop_glue::<Option<RenderReady>>` call on the overwritten value, which the
    /// call-graph gate sees even though it never executes.
    ///
    /// The output block is filled with positive zero so a failed render emits silence rather than
    /// the previous quantum.
    ///
    /// The diagnostic is a fixed text the half keeps (#1332 S5): the single instance copies it
    /// into the control half's diagnostic after the call, and the lifecycle word tells the
    /// control half, on whatever thread it runs, that the session failed.
    fn fail(&mut self, code: u32, diagnostic: &'static [u8]) -> u32 {
        if let Some(ready) = self.ready.as_ref() {
            ready.lifecycle.store(STATE_FAILED, Ordering::Release);
        }
        self.silence();
        self.failure = Some(diagnostic);
        self.record(code)
    }

    /// The sticky failed state and positive-zero silence (#1332 S5), without a diagnostic: what
    /// a failure of either half leaves in the render half.
    fn silence(&mut self) {
        self.status.state = STATE_FAILED;
        if let Some(buffers) = self.buffers.as_mut() {
            buffers.output_pcm.fill(0.0);
        }
    }
}

/// The render half of one running host, released to the thread that renders it (#1332 S5):
/// [`AudioWorkletEngineHost::release_render`] hands it out and
/// [`AudioWorkletEngineHost::adopt_render`] takes it back. It owns the plan owner (with the
/// source consumers and the active attachment), the meter delivery, the output buffer and its
/// status; the control half keeps everything else. Nothing the two halves share is reached but
/// through atomics: the plan exchange, the source rings, the plan's own words, the successor
/// record, the lifecycle word, the meter level word and the hand-back ring.
///
/// It must go back to its host before the host disposes, so that what it owns is freed on the
/// control path (S6's Worker rejoins before it disposes); a render half dropped on its own frees
/// the plan wherever it is dropped.
pub struct RenderHost {
    /// Boxed, so the handle that crosses threads is one pointer (the handoff registry's word).
    pub(crate) half: Box<RenderHalf>,
}

impl RenderHost {
    // REALTIME_POLICY_BEGIN
    /// [`AudioWorkletEngineHost::render_next`] on this thread: render exactly one prepared
    /// quantum at the next absolute sample time. A failure's fixed diagnostic stays here
    /// ([`Self::diagnostic`]) until the host adopts the half back.
    pub fn render_next(&mut self) -> u32 {
        self.half.render_next()
    }

    /// [`AudioWorkletEngineHost::poll_meters`] on this thread.
    pub fn poll_meters(&mut self) -> u32 {
        self.half.poll_meters()
    }
    // REALTIME_POLICY_END

    /// The render half's status: its clock, its counters and the result of its last call.
    #[must_use]
    pub const fn status(&self) -> &WebStatus {
        &self.half.status
    }

    /// The block the last render wrote, left plane then right plane.
    #[must_use]
    pub fn output_pcm(&self) -> Option<&[f32]> {
        self.half.buffers.as_ref().map(|value| &*value.output_pcm)
    }

    /// The rendering plan's meter header (issue #143 D5, #1290d1).
    #[must_use]
    pub fn meter_header(&self) -> &WebMeterHeader {
        self.half
            .ready
            .as_ref()
            .map_or(&EMPTY_METER_HEADER, |ready| &ready.shape.meter_header)
    }

    /// The fixed diagnostic of the render half's sticky failure, or empty.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        self.half.failure.unwrap_or(&[])
    }

    // REALTIME_POLICY_BEGIN: #1332 S6, the released half's lease, shape check and failure.
    /// [`AudioWorkletEngineHost::set_meter_lease`] on this thread.
    pub fn set_meter_lease(&mut self, enabled: bool) -> u32 {
        self.half.set_meter_lease(enabled)
    }

    /// [`AudioWorkletEngineHost::reject_output_quantum`] on this thread.
    pub fn reject_output_quantum(&mut self, actual_frames: u32) -> u32 {
        self.half.reject_output_quantum(actual_frames)
    }

    /// Record a sticky failure of this half with its fixed diagnostic (#1332 S6: the instance's
    /// stack canary found overwritten at the release). The lifecycle word tells the control half.
    pub fn fail(&mut self, code: u32, diagnostic: &'static [u8]) -> u32 {
        self.half.fail(code, diagnostic)
    }
    // REALTIME_POLICY_END

    /// The rendering plan's meter frame (see [`AudioWorkletEngineHost::meter_frame`]).
    #[must_use]
    pub fn meter_frame(&self) -> &[f32] {
        self.half
            .ready
            .as_ref()
            .map_or(&[], |ready| &ready.shape.meter_frame)
    }

    /// Make the next adopting block wait twice on `barrier` just before it takes the adopted
    /// plan's tables (#1332 S5): once when it gets there, once to be released. Test support.
    #[cfg(feature = "test-support")]
    pub fn pause_before_adoption(&mut self, barrier: Arc<std::sync::Barrier>) {
        if let Some(ready) = self.half.ready.as_mut() {
            ready.adoption_pause = Some(barrier);
        }
    }

    /// The full-width address the rendering plan's header names as its successor, or `0`
    /// (see [`AudioWorkletEngineHost::meter_successor_address`]). Test support (#1332 S5).
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn meter_successor_address(&self) -> usize {
        self.half
            .ready
            .as_ref()
            .map_or(0, |ready| ready.shape.successor_address)
    }

    /// The epoch of the plan whose tables the render half holds: the plan that rendered the last
    /// block. Test support (#1332 S5).
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn plan_epoch(&self) -> u64 {
        self.half
            .ready
            .as_ref()
            .map_or(0, |ready| ready.shape_epoch)
    }
}

/// Run `operation` inside the render-locked window (#1333 D1), as the render exports run their
/// bodies, so a test that drives a [`RenderHost`] on its own thread counts what it allocates in
/// the render-locked count. Test support (#1332 S5).
#[cfg(feature = "test-support")]
pub fn render_locked_for_test<R>(operation: impl FnOnce() -> R) -> R {
    render_lock::render_locked(operation)
}

/// One plan's observation taps as the core's gain-reduction fold sees them (#1582 D3): the
/// shape's dense effect table and strip table over the plan companions' reader sets. Each tap's
/// arm state is its slot's, which render writes (#1585 D6).
struct WebGainReductionTaps<'a> {
    effect_instances: &'a [Option<EffectInstance>],
    observation_tracks: &'a [u32],
    effect_observations: &'a [EffectObservationHandle],
    /// Test-only: the effect slots visited, so a test proves an incomplete master interval never
    /// reaches the scan.
    #[cfg(test)]
    scans: u64,
}

// REALTIME_POLICY_BEGIN: #1582 D3, the browser's taps for the meter frame's fold.
impl GainReductionTaps for WebGainReductionTaps<'_> {
    fn for_each_tap(&mut self, visit: &mut impl FnMut(TapView<'_>)) {
        for (effect, entry) in self.effect_instances.iter().enumerate() {
            #[cfg(test)]
            {
                self.scans = self.scans.saturating_add(1);
            }
            let Some(observed) = entry.as_ref() else {
                continue;
            };
            let Some(handle) = observed
                .reader
                .and_then(|reader| self.effect_observations.get(reader as usize))
            else {
                continue;
            };
            let Some(strip) = self.observation_tracks.get(effect).copied() else {
                continue;
            };
            // Issue #1209 D5: the frame carries one gain-reduction word per strip, so a submix's
            // effect folds into its own word at strip `T + j`.
            if strip == u32::MAX {
                continue;
            }
            for (reader, descriptor) in handle.readers.iter().zip(observed.descriptor.observations)
            {
                // A torn read is no arm this poll: the next poll reads the tap again.
                let Some(state) = reader.read_state() else {
                    continue;
                };
                visit(TapView {
                    strip,
                    descriptor,
                    reader,
                    armed: state.armed,
                    arm_sample: state.arm_sample,
                });
            }
        }
    }
}

/// Fold at most one closed meter window through the session's delivery (#1582 D4) at
/// `next_sample`, the next block's first sample, and hand the frame it publishes to the clients
/// the applied level word names (#1588 D1, D2): to the rendering plan's frame ring while the
/// protocol's table is on (a full ring drops and counts the frame), and to the lease frame while
/// the lease is on. Returns the windows the published frame folds, or `0`.
///
/// The core delivery folds the windows; the host keeps only the copies. This function has no
/// runtime-checked `[]` index, so it adds no bounds check of its own. The one
/// `panic_bounds_check` the fold brings is the slot index of the SPSC queues inlined into
/// `MeterDelivery::poll`, which the ring keeps in range, and the ring push's own queues are the
/// same kind. Both callers are on the render thread (`process()`), so the shipped artifact's
/// call-graph gate covers it. Inlined into both, so the active attachment's downcast stays the
/// caller's own pinned `call_indirect` pair.
#[inline(always)]
fn poll_meter_delivery(ready: &mut RenderReady, next_sample: u64) -> u32 {
    // #1381a D3: the meter consumers and the observation readers are the active plan's
    // companions. The downcast allocates and drops nothing.
    let Some(companions) = ready.plan.active_attachment_mut::<RenderCompanions>() else {
        return 0;
    };
    let shape = &mut *ready.shape;
    let mut taps = WebGainReductionTaps {
        effect_instances: &shape.effect_instances,
        observation_tracks: &shape.observation_tracks,
        effect_observations: &companions.effect_observations,
        #[cfg(test)]
        scans: 0,
    };
    let published = ready
        .meters
        .poll(&mut companions.meters, &mut taps, next_sample);
    #[cfg(test)]
    {
        ready.meter_effect_scan_count = ready.meter_effect_scan_count.saturating_add(taps.scans);
    }
    let Some(frame) = published else {
        return 0;
    };
    // #1588 D2: only while the table is on, and only into the rendering plan's ring, which the
    // block that adopted the plan swapped in with its tables.
    if ready.meter_applied & MeterLevel::TELEMETRY != 0
        && let Some(ring) = shape.meter_ring.as_mut()
    {
        debug_assert_eq!(
            frame.plan_epoch, ring.epoch,
            "a meter frame went to another plan's ring"
        );
        ring.producer.publish_frame(frame);
    }
    // R4: one copy per published window, only while the lease is on.
    if ready.meter_applied & MeterLevel::LEASE != 0 {
        publish_web_meter_frame(
            frame,
            shape.master_track,
            &mut shape.meter_frame,
            &mut shape.meter_header,
        );
    }
    frame.windows
}

/// Copy the core frame into the page's frame words and header (#1582 D4), the layout of
/// [`PlanShape::meter_frame`]: every strip's peak pair, tracks then submixes (issue #1209 D1, so
/// the master follows the last submix rather than the last track), the master's, every strip's
/// gain-reduction word, then the master's ([`MeterFrame::master_gain_reduction`] of `master`, the
/// plan's master strip; `0.0` when absent). Bounded by the strip count.
fn publish_web_meter_frame(
    frame: &MeterFrame,
    master: Option<u32>,
    words: &mut [f32],
    header: &mut WebMeterHeader,
) {
    let mut slots = words.iter_mut();
    // The source goes first in each zip, so its end leaves the next slot unconsumed.
    for (peak, slot) in frame.strip_peaks.iter().flatten().zip(&mut slots) {
        *slot = *peak;
    }
    let master_reading = frame.master_gain_reduction(master);
    let master_gain = master_reading.unwrap_or(0.0);
    let tail = frame
        .output_peak
        .iter()
        .chain(&frame.strip_gain_reduction_db)
        .chain(core::iter::once(&master_gain));
    for (value, slot) in tail.zip(slots) {
        *slot = *value;
    }
    header.windows = frame.windows;
    header.sequence = frame.sequence;
    header.master_gr_present = u32::from(master_reading.is_some());
    header.first_sample = frame.first_sample;
    header.end_sample = frame.end_sample;
    header.reserved[0] = frame.generation;
    header.reserved[1] = u64::from(frame.validity.bits())
        | (frame.loss_count.min(u64::from(u32::MAX)) << METER_LOSS_SHIFT);
}
// REALTIME_POLICY_END

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
        options_bytes: size_of::<WebBootOptions>() as u64,
        status_bytes: size_of::<WebStatus>() as u64,
        session_document_bytes: 0,
        diagnostic_bytes: 0,
        id_staging_bytes: 0,
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

/// One typed boot refusal. The FFI copies its bounded diagnostic over the staged document prefix.
#[derive(Debug, Eq, PartialEq)]
pub struct BootFailure {
    result: u32,
    diagnostic: Vec<u8>,
}

impl BootFailure {
    /// Frozen boot result code.
    #[must_use]
    pub const fn result(&self) -> u32 {
        self.result
    }

    /// `code\tpath\n` diagnostic bytes.
    #[must_use]
    pub fn diagnostic(&self) -> &[u8] {
        &self.diagnostic
    }

    fn fixed(result: u32, code: &str) -> Self {
        Self {
            result,
            diagnostic: fixed_diagnostic(code),
        }
    }

    fn document(diagnostic: Vec<u8>) -> Self {
        Self {
            result: RESULT_REFUSED_DOCUMENT,
            diagnostic,
        }
    }

    /// `host.budget.retained_projection`, naming the projection, the budget and its effect row
    /// (issue #2194).
    fn retained_projection(projection: RetainedProjection, budget: u64) -> Self {
        Self {
            result: RESULT_REFUSED_BUDGET,
            diagnostic: format!(
                "host.budget.retained_projection\t$.maximum_memory_bytes[projected_bytes={},budget_bytes={budget},effect_state_bytes={},graph_bytes={},builtin_bytes={},control_bytes={}]\n",
                projection.total,
                projection.effect_state_bytes,
                projection.graph_bytes,
                projection.builtin_bytes,
                projection.control_bytes
            )
            .into_bytes(),
        }
    }

    fn exact_budget(code: &str, exact: u64, budget: u64) -> Self {
        Self {
            result: RESULT_REFUSED_BUDGET,
            diagnostic: format!(
                "{code}\t$.maximum_memory_bytes[exact_bytes={exact},budget_bytes={budget}]\n"
            )
            .into_bytes(),
        }
    }
}

impl From<Vec<u8>> for BootFailure {
    fn from(diagnostic: Vec<u8>) -> Self {
        Self::document(diagnostic)
    }
}

#[derive(Clone, Copy)]
struct PreparedBufferProjection {
    source_samples: u64,
    /// The control frame the boot resolved (issue #2191), the control request staging's length.
    control_frame_bytes: u32,
    report: WebResourceReport,
}

/// Why an ID could not be copied into ID staging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IdStagingError {
    /// The host owns no staging buffers.
    NoStaging,
    /// The ID is longer than the staging buffer.
    TooLong,
}

fn project_buffers(
    document_bytes: u32,
    sample_rate_hz: u32,
    quantum_frames: u32,
    control_frame_bytes: u32,
) -> Result<PreparedBufferProjection, BootFailure> {
    // The staging holds what a rebuild may add, not only what the boot session declares (#1562b):
    // a source, track or submix ID of the longest length the grammar allows, and a source of the
    // most channels a source can have (two). Sizing from the boot session would let an acked rebuild declare a
    // source the host could never stage or feed.
    let maximum_source_channels = SOURCE_CHANNELS_LAUNCH_MAXIMUM;
    let id_staging_bytes = STABLE_ID_MAXIMUM_BYTES as u64;
    let arithmetic = || BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic");
    let source_samples = u64::from(maximum_source_channels)
        .checked_mul(u64::from(quantum_frames))
        .ok_or_else(arithmetic)?;
    let source_pcm_bytes = source_samples.checked_mul(4).ok_or_else(arithmetic)?;
    let output_pcm_bytes = u64::from(quantum_frames)
        .checked_mul(8)
        .ok_or_else(arithmetic)?;
    let plane_reference_bytes = u64::from(maximum_source_channels)
        .checked_mul(size_of::<&[f32]>() as u64)
        .ok_or_else(arithmetic)?;
    let host_shell_bytes =
        u64::try_from(size_of::<AudioWorkletEngineHost>()).map_err(|_| arithmetic())?;
    // #1332 S6: the render half and the status shell a release leaves are two boxes of their own,
    // and the status row is the half's.
    let render_halves_bytes =
        u64::try_from(2 * size_of::<RenderHalf>()).map_err(|_| arithmetic())?;
    let fixed_metadata = host_shell_bytes
        .checked_add(render_halves_bytes)
        .ok_or_else(arithmetic)?
        .checked_sub(u64::from(BOOT_OPTIONS_BYTES) + u64::from(STATUS_BYTES))
        .ok_or_else(arithmetic)?;
    let bridge_metadata = fixed_metadata
        .checked_add(plane_reference_bytes)
        .ok_or_else(arithmetic)?;
    let rows = [
        u64::from(BOOT_OPTIONS_BYTES),
        u64::from(STATUS_BYTES),
        u64::from(document_bytes),
        u64::from(DIAGNOSTIC_BYTES),
        id_staging_bytes,
        source_pcm_bytes,
        output_pcm_bytes,
        u64::from(control_frame_bytes),
        bridge_metadata,
    ];
    let retained = rows.into_iter().try_fold(0_u64, |total, row| {
        total.checked_add(row).ok_or_else(arithmetic)
    })?;
    // `bridge_metadata` is the sum of retained payloads, not one allocation. Keep the largest
    // row below to the individually allocated fixed buffers.
    let largest = rows[..rows.len() - 1]
        .iter()
        .copied()
        .max()
        .unwrap_or(0)
        .max(host_shell_bytes)
        .max(plane_reference_bytes);
    let mut report = empty_resource_report(selected_backend());
    report.sample_rate_hz = sample_rate_hz;
    report.quantum_frames = quantum_frames;
    report.session_document_bytes = u64::from(document_bytes);
    report.diagnostic_bytes = u64::from(DIAGNOSTIC_BYTES);
    report.id_staging_bytes = id_staging_bytes;
    report.source_pcm_staging_bytes = source_pcm_bytes;
    report.output_pcm_bytes = output_pcm_bytes;
    report.bridge_metadata_bytes = bridge_metadata;
    report.bridge_retained_bytes = retained;
    report.largest_bridge_allocation_bytes = largest;
    report.largest_named_allocation_bytes = largest;
    Ok(PreparedBufferProjection {
        source_samples,
        control_frame_bytes,
        report,
    })
}

fn allocate_buffers(
    projection: PreparedBufferProjection,
) -> Result<(ControlStaging, PreparedBuffers), BootFailure> {
    let allocation = |_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.allocation");
    let staging = ControlStaging {
        diagnostic: boxed_zero_u8(DIAGNOSTIC_BYTES).map_err(allocation)?,
        diagnostic_len: 0,
        source_id: boxed_zero_u8(
            u32::try_from(projection.report.id_staging_bytes)
                .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.platform"))?,
        )
        .map_err(allocation)?,
        source_pcm: boxed_zero_f32(projection.source_samples).map_err(allocation)?,
        control_request: boxed_zero_u8(projection.control_frame_bytes).map_err(allocation)?,
        plane_references: boxed_uninit_planes(
            u32::try_from(projection.source_samples / u64::from(projection.report.quantum_frames))
                .map_err(|_| BootFailure::fixed(RESULT_REFUSED_BUDGET, "web.resource.platform"))?,
        )
        .map_err(allocation)?,
    };
    let buffers = PreparedBuffers {
        output_pcm: boxed_zero_f32(u64::from(projection.report.quantum_frames) * 2)
            .map_err(allocation)?,
    };
    Ok((staging, buffers))
}

/// The boot memory budget `options` select: the word, or [`DEFAULT_MAXIMUM_MEMORY_BYTES`] for
/// zero.
pub(crate) const fn memory_budget(options: &WebBootOptions) -> u64 {
    if options.maximum_memory_bytes == 0 {
        DEFAULT_MAXIMUM_MEMORY_BYTES
    } else {
        options.maximum_memory_bytes
    }
}

/// The longest document `options` admit (issue #2191):
/// `(budget - frame rows) / PARSE_TRANSIENT_MULTIPLIER` ([`control_frame_rows`]; 26,814,976 on
/// wasm at the 512 MiB default and the 128 KiB frame), or the `maximum_document_bytes` word when
/// that is smaller. The boot, both document stagings
/// and the control plane's `SESSION_REPLACE` share it.
pub(crate) fn maximum_document_bytes(options: &WebBootOptions) -> u32 {
    let rows = control_frame_rows(staged_control_frame_bytes(options)).unwrap_or(u64::MAX);
    let derived = memory_budget(options).saturating_sub(rows) / PARSE_TRANSIENT_MULTIPLIER;
    let limit = match options.maximum_document_bytes {
        0 => derived,
        word => word.min(derived),
    };
    u32::try_from(limit).unwrap_or(u32::MAX)
}

/// Why a document of `document_bytes` is refused under `options` (issue #2191), or `None`: over
/// the `maximum_document_bytes` word, `web.document.maximum_bytes` naming its bytes and the word;
/// otherwise over the budget's parse projection, `host.budget.parse_projection` naming the
/// projection and the budget. Either names the bytes, before a byte of the document is parsed.
pub(crate) fn document_length_refusal(
    options: &WebBootOptions,
    document_bytes: u32,
) -> Option<BootFailure> {
    let word = options.maximum_document_bytes;
    if word != 0 && u64::from(document_bytes) > word {
        return Some(BootFailure {
            result: RESULT_REFUSED_DOCUMENT,
            diagnostic: format!(
                "web.document.maximum_bytes\t$.maximum_document_bytes[document_bytes={document_bytes},limit_bytes={word}]\n"
            )
            .into_bytes(),
        });
    }
    let budget = memory_budget(options);
    let frame = staged_control_frame_bytes(options);
    let arithmetic = || BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic");
    let rows = match control_frame_rows(frame) {
        Ok(rows) => rows,
        Err(failure) => return Some(failure),
    };
    let Some(projection) =
        (u64::from(document_bytes) * PARSE_TRANSIENT_MULTIPLIER).checked_add(rows)
    else {
        return Some(arithmetic());
    };
    (projection > budget).then(|| BootFailure {
        result: RESULT_REFUSED_BUDGET,
        diagnostic: format!(
            "host.budget.parse_projection\t$.maximum_memory_bytes[projected_bytes={projection},budget_bytes={budget},document_bytes={document_bytes},control_frame_bytes={frame},control_frame_rows_bytes={rows}]\n"
        )
        .into_bytes(),
    })
}

/// The control frame staged options select for a projection (#2191): the resolved frame, or the
/// default for a word boot will refuse (`web.options.control_frame_bytes`) anyway.
fn staged_control_frame_bytes(options: &WebBootOptions) -> u32 {
    resolved_control_frame_bytes(options).unwrap_or(DEFAULT_CONTROL_FRAME_BYTES)
}

/// The control plane's retained rows a control frame of `control_frame_bytes` sizes (#2191):
/// [`control_plane::frame_scaled_retained_bytes`] at the limits the boot gives the control plane,
/// the same rows its compile charges. Boot adds them to the parse projection and to the retained
/// projection, so a frame the budget cannot hold is refused before any memory grows.
pub(crate) fn control_frame_rows(control_frame_bytes: u32) -> Result<u64, BootFailure> {
    control_plane::frame_scaled_retained_bytes::<WebAdapter>(control_frame_limits(
        control_frame_bytes,
    ))
    .map_err(|failure| BootFailure {
        result: RESULT_REFUSED_BUDGET,
        diagnostic: failure.diagnostic().into_owned(),
    })
}

/// The control frame `options` select (issue #2191): [`DEFAULT_CONTROL_FRAME_BYTES`] for a zero
/// word, the word itself inside [`MINIMUM_CONTROL_FRAME_BYTES`]..=[`MAXIMUM_CONTROL_FRAME_BYTES`],
/// and `None` outside it, which boot refuses `web.options.control_frame_bytes`. The boot command
/// staging reads the staged options through it too, so both bound a frame alike.
pub(crate) fn resolved_control_frame_bytes(options: &WebBootOptions) -> Option<u32> {
    match options.maximum_control_frame_bytes {
        0 => Some(DEFAULT_CONTROL_FRAME_BYTES),
        word => u32::try_from(word).ok().filter(|frame| {
            (MINIMUM_CONTROL_FRAME_BYTES..=MAXIMUM_CONTROL_FRAME_BYTES).contains(frame)
        }),
    }
}

fn validate_options(mut options: WebBootOptions) -> Result<WebBootOptions, BootFailure> {
    if options.struct_size == 0 && options.abi_version == 0 {
        options.struct_size = BOOT_OPTIONS_BYTES;
        options.abi_version = ABI_VERSION;
    } else if options.struct_size != BOOT_OPTIONS_BYTES {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.struct_size",
        ));
    } else if options.abi_version != ABI_VERSION {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.abi_version",
        ));
    }
    // Issue #143: a master designation must be a plausible track index. The session-level range
    // check is the facade's; this one only refuses what cannot be a track index at all.
    if options.reserved0 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved0",
        ));
    }
    // #2191: the control frame word. Zero is the default; a frame below the floor starves the
    // replies, and one above the ceiling outgrows the staging addresses.
    if resolved_control_frame_bytes(&options).is_none() {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.control_frame_bytes",
        ));
    }
    // #1689: the word at offset 56 was the master boot word, reserved the same way.
    if options.reserved2 != 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.reserved2",
        ));
    }
    // #1380 D2: the module's shared memory cannot grow past its declared maximum, so a budget
    // above it is refused here, typed, rather than accepted and failed later as a `memory.grow`
    // trap.
    if options.maximum_memory_bytes > u64::from(WASM_MAXIMUM_MEMORY_PAGES) * WASM_PAGE_BYTES {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.boot.memory_maximum",
        ));
    }
    if options.live_control_meter_blocks > u64::from(u32::MAX) {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_OPTIONS,
            "web.options.live_controls",
        ));
    }
    Ok(options)
}

/// The boot's retained projection ([`projected_retained_bytes`]): the total and the rows its
/// diagnostic names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RetainedProjection {
    /// Every projected row.
    pub(crate) total: u64,
    /// The effect row (issue #2194): every instance's declared state and scratch, and the prepared
    /// inline bytes and undeclared heap of each instance the bank plan leaves per node
    /// ([`host_core::HostProjection::per_node_processor_bytes`]). Since issue #2219 it excludes
    /// the processor of a banked member, which the plan never prepares; the diagnostic keeps the
    /// name `effect_state_bytes`.
    pub(crate) effect_state_bytes: u64,
    /// The graph row (issue #2199): the boot plan's graph and plan bytes beyond the declared effect
    /// state and scratch the effect row charges. Since issue #2223 it holds every bank's
    /// processor bytes (`GraphResourceEstimate::effect_bank_processor_bytes`).
    pub(crate) graph_bytes: u64,
    /// The builtins' retained payload, inside the graph row; named, not added.
    pub(crate) builtin_bytes: u64,
    /// The control rows (issue #2199): the control plane's retained bytes for the boot plan, its
    /// frame-scaled rows (#2191) among them, the boot plan's shape tables and the session's
    /// cross-thread words.
    pub(crate) control_bytes: u64,
}

/// Project every retained byte of the boot whose shape is known after compile, before the
/// control plane compiles or prepares anything (issue #2199).
///
/// Bridge storage is the exact layout projection ([`project_buffers`]), and the compiled model and
/// queue rows are the session estimator's. Everything the boot plan retains is the boot's
/// preparation projected ([`control_plane::project_boot`]): host-core's
/// [`HostProjection`](host_core::HostProjection) of it (the graph and plan, the builtins, the
/// sources and the effect producers exact; the send lanes and the inventory at their ceilings) and
/// the control plane's retained bytes for it. The effect row (issue #2194) charges the effect
/// state and scratch of every instance and the processors of the per-node instances only (issue
/// #2219: a banked member has none), so the graph row adds only what the graph holds beyond the
/// declared state and scratch. The boot plan's shape tables and the session's cross-thread words
/// are their layouts. So a session the budget cannot hold is refused here, before a processor,
/// bank, plan, ring or lane exists; a projection under the exact aggregate is a defect, which the
/// representative drift gate holds.
fn projected_retained_bytes(
    compiled: &CompiledSession,
    options: WebBootOptions,
    caps: &HostPrepareCaps,
    memory_budget: u64,
    control_frame_bytes: u32,
    bridge_retained_bytes: u64,
    boot_command: Option<&[u8]>,
) -> Result<RetainedProjection, BootFailure> {
    let arithmetic = || BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic");
    let estimate = compiled.resource_estimate();
    // The lanes the control plane chooses, and the request and lanes [`WebRuntimePreparer`]
    // attaches over them.
    let (live_controls, lanes) = WebRuntimePreparer::new(options)
        .request(
            compiled,
            controls::live_lanes(compiled.normalized_model()).into(),
        )
        .map_err(|failure| BootFailure {
            result: RESULT_REFUSED_OPTIONS,
            diagnostic: failure.as_bytes().to_vec(),
        })?;
    // The budget is this projection's to check, as one aggregate: the preparation's own byte
    // ceilings, each the whole budget, would refuse a row before the projection names them all.
    let unbounded = HostPrepareCaps {
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_bytes: u64::MAX,
        ..*caps
    };
    let boot = control_plane::project_boot::<WebAdapter>(
        compiled,
        &unbounded,
        control_limits(
            caps,
            memory_budget,
            control_frame_bytes,
            maximum_document_bytes(&options),
        ),
        &live_controls,
        lanes,
        boot_command,
    )
    .map_err(|(failure, rejection)| {
        // As [`boot_refusal`] classifies the control plane's own preparation's refusal.
        let result = match (rejection, &failure) {
            (Some(PrepareRejection::Resource | PrepareRejection::Platform), _)
            | (None, CompileFailure::Resource(_)) => RESULT_REFUSED_BUDGET,
            (Some(_), _) | (None, CompileFailure::Diagnostics(_)) => RESULT_REFUSED_DOCUMENT,
        };
        BootFailure {
            result,
            diagnostic: failure.diagnostic().into_owned(),
        }
    })?;
    let host = boot.host;
    let effect_state_bytes = host
        .effects
        .state_bytes
        .checked_add(host.effects.scratch_bytes)
        .and_then(|declared| declared.checked_add(host.per_node_processor_bytes))
        .ok_or_else(arithmetic)?;
    let graph_bytes = host
        .graph_session_plus_plan_bytes
        .checked_sub(host.graph_declared_effect_bytes)
        .ok_or_else(arithmetic)?;
    let shape_bytes = projected_shape_bytes(&host).ok_or_else(arithmetic)?;
    let control_bytes = boot
        .control_retained_bytes
        .checked_add(shape_bytes)
        .ok_or_else(arithmetic)?;
    let total = [
        bridge_retained_bytes,
        estimate.compiled_model_bytes,
        estimate.queue_bytes,
        host.source_total_bytes,
        effect_state_bytes,
        graph_bytes,
        control_bytes,
    ]
    .into_iter()
    .try_fold(0_u64, |total, row| {
        total.checked_add(row).ok_or_else(arithmetic)
    })?;
    Ok(RetainedProjection {
        total,
        effect_state_bytes,
        graph_bytes,
        builtin_bytes: host.builtin_retained_payload_bytes,
        control_bytes,
    })
}

/// The bridge rows of the boot plan's [`PlanShape`] ([`PlanShape::bridge_rows`]), the plan's
/// [`RenderCompanions`] ([`RenderCompanions::retained_bytes`], which its preparer charges to the
/// plan's graph row) and the session's cross-thread words ([`cross_thread_bytes`]), from the boot
/// preparation's projection, before any exists (issue #2199): the dense instance table and the
/// producers' IDs it copies, the meter tables and frame, the plan words, the companions' shell,
/// meter consumers, observation handles and carried readers, and the retired-shape ring. The
/// observation handles' vector grows by pushes, so its capacity is charged at its growth bound.
fn projected_shape_bytes(host: &host_core::HostProjection) -> Option<u64> {
    let handles = host.observation_handles;
    let handle_capacity = if handles == 0 {
        0
    } else {
        handles.checked_next_power_of_two()?.max(4)
    };
    let companions = size_of::<RenderCompanions>()
        .checked_add(host.meters.checked_mul(size_of::<MeterConsumer>())?)?
        .checked_add(handle_capacity.checked_mul(size_of::<EffectObservationHandle>())?)?
        .checked_add(
            host.meters
                .checked_add(handles)?
                .checked_mul(size_of::<CarriedReader>())?,
        )?;
    let instance_table = u64::try_from(host.effect_instances)
        .ok()?
        .checked_mul((size_of::<Option<EffectInstance>>() + size_of::<u32>()) as u64)?;
    let (meter_tables, _) = MeterTables::projected_bytes(host.meters, host.strips);
    let frame = host
        .strips
        .checked_mul(3)?
        .checked_add(3)?
        .checked_mul(size_of::<f32>())?;
    let words = engine::realtime::published_words_retained_bytes::<2>();
    let (_, retired) = engine::realtime::bounded_spsc_move::<Box<PlanShape>>(
        NonZeroUsize::new(RETIRED_SHAPES)?,
        engine::realtime::QueueGeneration(0),
    )
    .ok()?;
    let shared = cross_thread_bytes(retired.retained_bytes())?;
    instance_table
        .checked_add(host.effect_producer_id_bytes)?
        .checked_add(u64::try_from(meter_tables).ok()?)?
        .checked_add(u64::try_from(frame).ok()?)?
        .checked_add(u64::try_from(words).ok()?)?
        .checked_add(u64::try_from(companions).ok()?)?
        .checked_add(shared)
}

/// Exact aggregate retained bytes reported by the fully prepared engine plus the browser bridge.
///
/// `source_total_bytes` owns the source-ring PCM charge plus source overhead. The graph row does
/// not repeat it, and `bridge_retained_bytes` already includes the compiled model, the facade's
/// control table/ID arena and the prepared plan's state inventory. These three disjoint rows are
/// therefore the complete retained set.
fn exact_retained_bytes(report: &WebResourceReport) -> Result<u64, BootFailure> {
    report
        .bridge_retained_bytes
        .checked_add(report.graph_session_plus_plan_bytes)
        .and_then(|total| total.checked_add(report.source_total_bytes))
        .ok_or_else(|| BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic"))
}

/// Translate document-derived shape plus host policy into the shared preparation caps.
///
/// Count ceilings are intentionally unbounded: boot v1 has one physics gate, not structural caps.
/// Byte ceilings are the same effective memory budget, so every subsystem retains its own checked
/// pre-allocation arithmetic while the aggregate projection is checked before preparation starts.
fn prepare_caps(
    compiled: &CompiledSession,
    options: WebBootOptions,
    source_ring_frames: u32,
    memory_budget: u64,
) -> HostPrepareCaps {
    let sample_rate_hz = if options.require_sample_rate_hz == 0 {
        compiled.sample_rate().0
    } else {
        options.require_sample_rate_hz
    };
    let quantum_frames = if options.require_quantum_frames == 0 {
        compiled.quantum().0
    } else {
        options.require_quantum_frames
    };
    HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz,
            quantum_frames,
        },
        source_ring_frames,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: MAXIMUM_VCAS_BOUNDED,
        maximum_vca_reach_pairs: MAXIMUM_VCA_REACH_PAIRS,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: memory_budget,
        maximum_source_total_bytes: memory_budget,
        maximum_source_overhead_bytes: memory_budget,
        maximum_effect_state_bytes: memory_budget,
        maximum_effect_scratch_bytes: memory_budget,
        maximum_builtin_retained_bytes: memory_budget,
        maximum_named_allocation_bytes: memory_budget,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: memory_budget,
    }
}

/// The control plane's limits for the browser's session (#1381a D1): the preparation caps the boot
/// computed, as plain values, and the control plane's own fixed allocations.
///
/// The byte ceilings are the memory budget, as the caps' are, and counts are unbounded: the
/// boot's one physics gate is the budget. The control frame (the boot word, issue #2191), the
/// replay cache it implies and the diagnostic are the control plane's fixed allocations; they are
/// the same for every session of one frame, and the budget's exact check
/// ([`exact_retained_bytes`]) charges them in the bridge rows.
fn control_limits(
    caps: &HostPrepareCaps,
    memory_budget: u64,
    control_frame_bytes: u32,
    maximum_document_bytes: u32,
) -> ControlLimits {
    ControlLimits {
        source_ring_frames: caps.source_ring_frames,
        maximum_document_bytes: u64::from(maximum_document_bytes),
        maximum_tracks: caps.maximum_tracks,
        maximum_sources: caps.maximum_sources,
        maximum_routes: caps.maximum_routes,
        maximum_effects: caps.maximum_effects,
        maximum_graph_session_plus_plan_bytes: caps.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: caps.maximum_source_total_bytes,
        maximum_source_overhead_bytes: caps.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: caps.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: caps.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: caps.maximum_builtin_retained_bytes,
        maximum_control_retained_bytes: memory_budget,
        maximum_named_allocation_bytes: caps.maximum_named_allocation_bytes,
        maximum_meter_streams: caps.maximum_meter_streams,
        maximum_meter_items: caps.maximum_meter_items,
        maximum_meter_bytes: caps.maximum_meter_bytes,
        maximum_submixes: caps.maximum_submixes,
        maximum_vcas: caps.maximum_vcas,
        maximum_vca_reach_pairs: caps.maximum_vca_reach_pairs,
        // The boot sets the real figure once its bridge rows are exact (`compile_ready`).
        maximum_replacement_peak_bytes: memory_budget,
        ..control_frame_limits(control_frame_bytes)
    }
}

/// The control plane's limits that the control frame decides (#2191): the frame, the replay
/// cache of `max(64 KiB, 2 * frame)` and its entries, and the diagnostic. [`control_limits`]
/// takes them from here and [`control_frame_rows`] projects from here, so the boot projects the
/// frame rows the control plane charges. The session fields are zero: no frame row reads them.
const fn control_frame_limits(control_frame_bytes: u32) -> ControlLimits {
    ControlLimits {
        source_ring_frames: 0,
        maximum_document_bytes: 0,
        maximum_diagnostic_bytes: DIAGNOSTIC_BYTES as u64,
        maximum_tracks: 0,
        maximum_sources: 0,
        maximum_routes: 0,
        maximum_effects: 0,
        maximum_graph_session_plus_plan_bytes: 0,
        maximum_source_total_bytes: 0,
        maximum_source_overhead_bytes: 0,
        maximum_effect_state_bytes: 0,
        maximum_effect_scratch_bytes: 0,
        maximum_builtin_retained_bytes: 0,
        maximum_control_retained_bytes: 0,
        maximum_named_allocation_bytes: 0,
        maximum_meter_streams: 0,
        maximum_meter_items: 0,
        maximum_meter_bytes: 0,
        maximum_control_frame_bytes: control_frame_bytes as u64,
        maximum_replay_bytes: control_replay_bytes(control_frame_bytes as u64),
        maximum_replay_entries: CONTROL_REPLAY_ENTRIES,
        maximum_submixes: 0,
        maximum_vcas: 0,
        maximum_vca_reach_pairs: 0,
        maximum_replacement_peak_bytes: 0,
    }
}

/// The control plane's replay cache for a control frame of `frame_bytes` (issue #2191):
/// [`MINIMUM_CONTROL_REPLAY_BYTES`], or twice the frame when that is more. The protocol reserves a
/// request plus its largest response before it runs a command and answers backpressure when the
/// reservation exceeds the cache, so a cache smaller than two frames refuses every large command.
const fn control_replay_bytes(frame_bytes: u64) -> u64 {
    let doubled = frame_bytes * 2;
    if doubled > MINIMUM_CONTROL_REPLAY_BYTES {
        doubled
    } else {
        MINIMUM_CONTROL_REPLAY_BYTES
    }
}
/// The smallest replay cache: what the browser's 16 KiB frame had before issue #2191.
const MINIMUM_CONTROL_REPLAY_BYTES: u64 = 64 << 10;
/// Entries of the control plane's replay cache.
const CONTROL_REPLAY_ENTRIES: u64 = 32;

/// A refused boot preparation as the browser reports it (#1381a): the control plane's diagnostic
/// bytes, and the result code the refusal's class chooses: a resource or platform refusal is a
/// budget refusal and the rest are document refusals, as boot chose them before the control plane
/// prepared the plan. The class is what the preparer saw; a refusal it
/// did not see is the control plane's own, and a resource fault is a budget refusal.
fn boot_refusal(failure: &CompileFailure, record: &Mutex<BootRecord>) -> BootFailure {
    let rejection = record.lock().ok().and_then(|record| record.rejection);
    let result = match (rejection, failure) {
        (Some(PrepareRejection::Resource | PrepareRejection::Platform), _)
        | (None, CompileFailure::Resource(_)) => RESULT_REFUSED_BUDGET,
        (Some(_), _) | (None, CompileFailure::Diagnostics(_)) => RESULT_REFUSED_DOCUMENT,
    };
    BootFailure {
        result,
        diagnostic: failure.diagnostic().into_owned(),
    }
}

/// The browser as a control-plane adapter (#1381a): its plan resource table stores the control
/// plane's own row, and it keeps no per-session handle beside the two halves.
///
/// The row is the control plane's [`PlanResources`] unchanged, because the browser's resource
/// report projects from it directly (`compile_ready`). The adapter's own fixed allocations are
/// the browser bridge's, which [`WebResourceReport`] charges in its bridge rows.
pub struct WebAdapter;

impl ControlAdapter for WebAdapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
    /// #1596: the browser reads spectrum through the session's three frames, as the C ABI does.
    const SPECTRUM: bool = true;
    /// #1697: `miso_engine_web_v1_replace_document_ptr` stages the document a `SESSION_REPLACE`
    /// names, and [`AudioWorkletEngineHost::control_command_with_document`] passes it beside the
    /// frame.
    const DOCUMENT_TRANSPORT: bool = true;
    // #1588 R1: the browser's meter batches come from its render half's fold, which pushes each
    // published frame into the plan's ring while the protocol's table is on (#1588b).
    const METER_DELIVERY: bool = true;
    const RENDER_METER_FOLD: bool = true;
    /// #1589a: the session serves the `Observe` edit and `OBSERVATION_READ` (with capability bit
    /// 15): every plan binds every declared tap (#1590 D5,
    /// [`control_plane::RuntimePreparer::observation_queues`]). The edit's records ride the
    /// effects' producers through the one control plane, the only writer of each queue since
    /// #1589c deleted kinds 7 and 8; the read reads the readers the render half keeps, which the
    /// control entry lends with their plan's epoch (R2).
    const OBSERVE_EDIT: bool = true;
}

/// The control-protocol session the browser host owns (#1381a): the committed model at its boot
/// revision, the source, strip and effect producers, the plan publisher and the retirer.
type WebSession = SessionState<WebAdapter>;
/// The render half of the browser's plan exchange: the plan owner that renders and adopts.
type WebPlan = PlanState<WebAdapter>;

/// The render-side halves a browser plan's preparation returns beside the plan (issue #1400 D5):
/// the meter consumers and the effect observation handles.
///
/// They travel inside the plan as its [`HostAttachment`], so a plan the control plane prepares
/// carries everything the render side reads, and they drop with the plan on the retirement path.
/// The browser renders through the plan owner (#1381a D4) and reaches the active plan's companions
/// through `PlanState::active_attachment_mut`: the meter poll and the observation reads read the
/// readers there. A plan's spectrum captures are not among them: the preparer returns them to the
/// session, whose spectrum monitor installs them at the plan's promotion (#1596 D3).
///
/// The plan's own tables ride here too (#1332 S4): its `PlanShape`, from its preparation to the
/// block that adopts it, so a committed plan carries what its render half reads through the plan
/// exchange, and nothing beside the plan has to follow it.
///
/// Field order is the drop order: the meter consumers, then the effect observation handles.
/// Neither reads the other while it drops; the order is kept so the two owners release alike.
pub struct RenderCompanions {
    /// One consumer per selected meter, in strip order; empty when metering is off.
    meters: Vec<MeterConsumer>,
    /// One reader set per observed effect instance, in `HostLiveControlHandles` order.
    effect_observations: Vec<EffectObservationHandle>,
    /// What the swap block that adopted this plan carried (#1381b): one move per paired meter or
    /// observation reader, written by [`HostAttachment::adopt_predecessor`] and read once by the
    /// block's table hand-over ([`RenderReady::adopt_shape`]). Its length is fixed at
    /// preparation, one entry per reader, so the hook never grows it.
    carried: Box<[CarriedReader]>,
    /// How many entries of `carried` the last swap block wrote.
    carried_len: u32,
    /// The plan's carried-tap record (#1587b, #1332 S6b), shared with its control-plane epoch:
    /// the swap block that adopts the plan writes what it carried, and the control side pairs its
    /// index's read-only views from it at promotion, so an `OBSERVATION_READ` follows a carried
    /// tap on whichever thread the control half runs. `None` for a plan with no reader.
    carried_taps: Option<std::sync::Arc<control_plane::CarriedTaps>>,
    /// The plan's tables (#1290a, #1332 S4), boxed by its preparation, so their address (and the
    /// meter header's, which the page reads) never moves: a replacement's until the block that
    /// adopts the plan exchanges them with the render half's, which leaves the retired plan's
    /// here until the render half moves them to a slot the control side's next service step
    /// empties. `None` for the boot plan, whose tables the render half takes at boot.
    shape: Option<Box<PlanShape>>,
}

/// One reader the swap block paired with the predecessor's (#1381b): `successor` and `predecessor`
/// are positions in the two plans' own reader vectors (host-core's order), the indices of the
/// engine's carry record.
#[derive(Clone, Copy)]
struct CarriedReader {
    kind: ObserverKind,
    successor: u32,
    predecessor: u32,
    /// For a meter, the handles the two positions' meters publish under (`0` for any other kind):
    /// the poll accepts the predecessor's for windows it closed before the swap (#1290d1).
    successor_handle: u64,
    predecessor_handle: u64,
}

impl CarriedReader {
    /// The value of an unused entry.
    const UNUSED: Self = Self {
        kind: ObserverKind::Meter,
        successor: 0,
        predecessor: 0,
        successor_handle: 0,
        predecessor_handle: 0,
    };
}

impl RenderCompanions {
    /// The meter consumers: one per selected meter, in strip order.
    #[must_use]
    pub fn meters(&self) -> &[MeterConsumer] {
        &self.meters
    }

    /// The effect observation handles, one reader set per observed effect instance.
    #[must_use]
    pub fn effect_observations(&self) -> &[EffectObservationHandle] {
        &self.effect_observations
    }

    /// The heap the attachment itself holds for the plan's lifetime, and its largest single
    /// allocation: the box, and the buffers of the two vectors. What the consumers, the handles
    /// and the capture own is host-core's report's, which already counts it; `None` on overflow.
    fn retained_bytes(&self) -> Option<(u64, u64)> {
        // Every field is named, so a field added to the struct fails to compile here until its
        // heap is charged (or declared host-core's, as the capture is).
        let Self {
            meters,
            effect_observations,
            carried,
            carried_len: _,
            // A shared record, charged by the control plane with the plan's observation rows;
            // its pointer is in the shell.
            carried_taps: _,
            // A replacement's tables are charged with its attachment by its preparer
            // ([`PlanShape::retained_bytes`]), the boot plan's to the bridge (`compile_ready`).
            shape: _,
        } = self;
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let carried = u64::try_from(carried.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<CarriedReader>()).ok()?)?;
        let meters = u64::try_from(meters.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<MeterConsumer>()).ok()?)?;
        let observations = u64::try_from(effect_observations.capacity())
            .ok()?
            .checked_mul(u64::try_from(size_of::<EffectObservationHandle>()).ok()?)?;
        let total = shell
            .checked_add(meters)?
            .checked_add(observations)?
            .checked_add(carried)?;
        Some((total, shell.max(meters).max(observations).max(carried)))
    }
}

#[cfg(feature = "test-support")]
std::thread_local! {
    static COMPANION_DROPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// How many [`RenderCompanions`] dropped on this thread (#1381a gate 2: the retired plan's
/// attachment drops in the service step, on the control side, and never in a render call).
#[cfg(feature = "test-support")]
#[must_use]
pub fn companion_drops_on_this_thread() -> u64 {
    COMPANION_DROPS.with(core::cell::Cell::get)
}

#[cfg(feature = "test-support")]
impl Drop for RenderCompanions {
    fn drop(&mut self) {
        COMPANION_DROPS.with(|drops| drops.set(drops.get() + 1));
    }
}

/// What a preparation leaves for the host (#1381a D1): the facts of the first plan that
/// `PlanResources` and the attachment do not carry, and (#1290a, #1332 S4) where the meter header
/// of the plan the preparer prepared last is. The preparer writes it, boot and a committed command
/// read it.
#[derive(Default)]
struct BootRecord {
    /// The engine-owned report of the boot plan, after the attachment's rows were charged.
    report: Option<HostPrepareReport>,
    /// The session's `live_values_superseded` count, shared with every cell reader of its plans.
    superseded: Option<host_core::LiveValuesSuperseded>,
    /// The class of the preparation refusal, so a refused boot keeps its result code.
    rejection: Option<PrepareRejection>,
    /// What the preparer left of the plan it prepared last (#1290d1, #1332 S4, S5): its meter
    /// header's address and the writer of its own words. A command that publishes a replacement
    /// takes them, so the rendering plan's header names the replacement's while it waits and the
    /// control half writes the replacement's words; one that fails after preparing leaves them
    /// for the next preparation to replace, on the control thread.
    successor: Option<PreparedSuccessor>,
}

impl RenderCompanions {
    /// What the swap block that adopted this plan paired (#1381b); empty when nothing was.
    fn carried(&self) -> &[CarriedReader] {
        self.carried.get(..self.carried_len as usize).unwrap_or(&[])
    }
}

impl HostAttachment for RenderCompanions {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    // REALTIME_POLICY_BEGIN
    /// Pair the readers the executors carried (#1381 D3; #1327 D3): the successor's meter
    /// consumers and observation reader sets swap with the predecessor's at the records the
    /// engine's carry names, through host-core's [`host_core::pair_carried_meters`] and
    /// [`host_core::pair_carried_observations`]. The browser keeps both vectors in host-core's
    /// order, so the record's indices address them directly. The pairs are noted for the block's
    /// table hand-over, which moves the meter delivery's per-meter state with them.
    ///
    /// Render thread, in the swap block: swaps and scalar writes into storage sized at
    /// preparation. A predecessor of another type, or a record with no reader, pairs nothing.
    fn adopt_predecessor(
        &mut self,
        predecessor: &mut dyn HostAttachment,
        carries: &[ObserverCarry],
    ) {
        self.carried_len = 0;
        // The control side's half of the pairing (#1332 S6b), written before anything else, as
        // the C ABI's attachment writes it.
        if let Some(record) = &self.carried_taps {
            record.record(carries);
        }
        let Some(previous) = predecessor.as_any_mut().downcast_mut::<Self>() else {
            return;
        };
        host_core::pair_carried_meters(&mut self.meters, &mut previous.meters, carries);
        host_core::pair_carried_observations(
            &mut self.effect_observations,
            &mut previous.effect_observations,
            carries,
        );
        let mut written = 0;
        for carry in carries {
            let (successor_len, predecessor_len) = match carry.kind {
                ObserverKind::Meter => (self.meters.len(), previous.meters.len()),
                ObserverKind::Observation => (
                    self.effect_observations.len(),
                    previous.effect_observations.len(),
                ),
                ObserverKind::Spectrum => continue,
            };
            if carry.successor >= successor_len || carry.predecessor >= predecessor_len {
                continue;
            }
            let (Ok(successor), Ok(predecessor)) = (
                u32::try_from(carry.successor),
                u32::try_from(carry.predecessor),
            ) else {
                continue;
            };
            let Some(entry) = self.carried.get_mut(written) else {
                break;
            };
            let (successor_handle, predecessor_handle) = match carry.kind {
                ObserverKind::Meter => (
                    self.meters
                        .get(carry.successor)
                        .map_or(0, |meter| meter.handle.0.get()),
                    previous
                        .meters
                        .get(carry.predecessor)
                        .map_or(0, |meter| meter.handle.0.get()),
                ),
                _ => (0, 0),
            };
            *entry = CarriedReader {
                kind: carry.kind,
                successor,
                predecessor,
                successor_handle,
                predecessor_handle,
            };
            written += 1;
        }
        self.carried_len = u32::try_from(written).unwrap_or(0);
    }
    // REALTIME_POLICY_END
}

/// The diagnostic code of a preparation that could not leave the plan's tables for the host (a
/// record another thread poisoned): refused before the transaction is committed (#1290a).
const SHAPE_UNAVAILABLE: &str = "web.apply.shape_unavailable";

/// The browser's [`control_plane::RuntimePreparer`] (issue #1400 D5): host-core's one entry
/// ([`host_core::prepare_host_runtime_with_live_lanes_and_spectrum`]) for the session's boot
/// options and the spectrum configuration the session passes (#1593 D3), with the render-side
/// companions moved into the plan.
///
/// Every plan, the boot plan and every successor, delivers controls concurrently and meters what
/// host-core's selection meters (issue #1581 D1, [`host_core::meter_selection`]): one `PostMatrix`
/// sample-peak meter per strip, the tracks then the submixes, so `meters[i]` is strip `i`'s peak
/// pair. A successor prepares every source it leaves unchanged vacant, to carry the displaced
/// plan's ring.
///
/// It attaches the live lanes the control plane chose ([`controls::live_lanes`], passed in as the
/// `lanes` argument), so the boot word chooses no lane here.
pub struct WebRuntimePreparer {
    options: WebBootOptions,
    /// Where the boot preparation leaves its facts for the host (#1381a), or `None` for a preparer
    /// that is only called.
    record: Option<Arc<Mutex<BootRecord>>>,
}

impl WebRuntimePreparer {
    /// The live-control request and the lanes every plan of this preparer attaches over the
    /// control plane's `lanes`: the boot word's meter window, every declared observation tap
    /// (#1590 D5) and the effect lanes they ride (#1254 D2), so the effect lanes are always on,
    /// as on the C ABI, whatever the session's live rows chose; a session with no effect to
    /// observe still boots.
    fn request(
        &self,
        compiled: &CompiledSession,
        lanes: HostLiveLanes,
    ) -> Result<(HostLiveControlRequest, HostLiveLanes), PrepareDiagnostics> {
        let live_controls =
            live_control_request(self.options, compiled.quantum().0).ok_or_else(|| {
                PrepareDiagnostics::fixed(PrepareRejection::Session, "web.live_controls.config")
            })?;
        let lanes = HostLiveLanes {
            effects: lanes.effects || live_controls.observation_taps != 0,
            ..lanes
        };
        Ok((live_controls, lanes))
    }

    /// A preparer for a session booted with `options`. The spectrum request is the session's
    /// (#1593 D3): every call takes it as an argument.
    #[must_use]
    pub fn new(options: WebBootOptions) -> Self {
        Self {
            options,
            record: None,
        }
    }

    /// The same preparer, leaving its boot facts in `record`.
    fn recording(mut self, record: Arc<Mutex<BootRecord>>) -> Self {
        self.record = Some(record);
        self
    }
}

impl control_plane::RuntimePreparer for WebRuntimePreparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        // The session's configuration (#1593 D3): the boot request, then whatever of it the
        // session's edits left. The captures go to the session, whose spectrum monitor installs
        // them at the plan's promotion and serves every read (#1596 D3), as on the C ABI.
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            host_core::HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let prepared = self.prepare_plan(compiled, caps, lanes, spectrum, successor, superseded);
        if let (Err(failure), Some(record)) = (&prepared, &self.record)
            && let Ok(mut record) = record.lock()
        {
            record.rejection = Some(failure.kind());
        }
        prepared
    }

    /// Issue #2199: the same preparation, projected.
    fn project(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<host_core::HostProjection, PrepareDiagnostics> {
        let (live_controls, lanes) = self.request(compiled, lanes)?;
        host_core::project_host_session(
            compiled,
            caps,
            host_core::HostProjectionRequest {
                live_controls: Some(&live_controls),
                selected_meters: None,
                backend: host_core::Backend::current(),
                spectrum,
                successor,
                lanes,
                superseded: Some(superseded),
            },
        )
    }

    /// The boot word (#1581 T2 obligation), so the session's output meter closes its windows on
    /// the strip meters' grid: `0` is host-core's default for both.
    fn meter_window_blocks(&self) -> u32 {
        u32::try_from(self.options.live_control_meter_blocks).unwrap_or(0)
    }

    /// The readers the plan's [`RenderCompanions`] keep (#1589 R2), for the session's one index.
    fn kept_observations<'p>(&self, host: &'p mut PreparedHost) -> &'p [EffectObservationHandle] {
        host.plan
            .host_attachment_mut::<RenderCompanions>()
            .map_or(&[], |companions| &companions.effect_observations)
    }

    /// The plan's carried-tap record rides the plan's [`RenderCompanions`] (#1332 S6b), the
    /// plan's one attachment, whose swap-block hook writes it.
    fn keep_carried_taps(
        &self,
        plan: &mut engine::realtime::PreparedRenderPlan,
        record: std::sync::Arc<control_plane::CarriedTaps>,
    ) -> Option<std::sync::Arc<control_plane::CarriedTaps>> {
        match plan.host_attachment_mut::<RenderCompanions>() {
            Some(companions) => {
                companions.carried_taps = Some(record);
                None
            }
            None => Some(record),
        }
    }

    /// The render half of the plan's meter frame ring rides the plan's tables (#1588 D2, #1332
    /// S4), so it reaches the render half with the plan. Its epoch is set by the block that adopts
    /// the plan (the boot's by `compile_ready`).
    fn keep_meter_frame_producer(
        &self,
        plan: &mut engine::realtime::PreparedRenderPlan,
        producer: host_core::MeterFrameProducer,
    ) -> Option<host_core::MeterFrameProducer> {
        match plan
            .host_attachment_mut::<RenderCompanions>()
            .and_then(|companions| companions.shape.as_mut())
        {
            Some(shape) => {
                shape.meter_ring = Some(PlanMeterRing { epoch: 0, producer });
                None
            }
            None => Some(producer),
        }
    }

    /// Stamp the plan's own words before it is published (#1332 S5): the master the commit
    /// leaves, as an index into the plan's strip order, and the structure epoch, through the
    /// writer the plan's preparation left in the record. Control thread, before publication.
    fn stamp_before_publication(
        &self,
        plan: &mut engine::realtime::PreparedRenderPlan,
        committed: &CompiledSession,
        master: Option<&session::StableId>,
        structure_epoch: u64,
    ) {
        let Some(header) = plan
            .host_attachment_mut::<RenderCompanions>()
            .and_then(|companions| companions.shape.as_ref())
            .map(|shape| core::ptr::from_ref(&shape.meter_header).addr())
        else {
            return;
        };
        let Some(record) = &self.record else {
            return;
        };
        let Ok(mut record) = record.lock() else {
            return;
        };
        // The record names the plan this preparer prepared last, which is the one the commit
        // publishes: no preparation runs between a commit's preparation and its publication.
        let Some(successor) = record
            .successor
            .as_mut()
            .filter(|successor| successor.header == header)
        else {
            debug_assert!(false, "the published plan is the one prepared last");
            return;
        };
        successor.words.publish(&plan_words(
            master_index(committed.normalized_model(), master),
            structure_epoch,
        ));
    }
}

impl WebRuntimePreparer {
    /// The preparation itself. With no `successor` it prepares the session's first plan, whose
    /// facts the host's boot reads from the record.
    fn prepare_plan(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            host_core::HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let boot = successor.is_none();
        let (live_controls, lanes) = self.request(compiled, lanes)?;
        let (mut host, mut handles, spectrum_captures) =
            host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
                compiled,
                caps,
                &live_controls,
                lanes,
                spectrum,
                successor,
                superseded,
            )?;
        // #1290a: the plan's tables, built from the plan's own model, producers and companions.
        // They ride the plan's companions (#1332 S4): the render half takes the boot's at boot and
        // a replacement's at its adoption block. The boot report charges the boot's to the bridge
        // rows; a replacement plan charges its own to the plan's rows, so the swap budget counts
        // them.
        let shape = build_plan_shape(
            compiled.normalized_model(),
            &handles.effect_controls,
            &handles.effect_observations,
            handles.meters.len(),
        )
        .map_err(|code| {
            let rejection = if code.starts_with("web.resource.") {
                PrepareRejection::Platform
            } else {
                PrepareRejection::Session
            };
            PrepareDiagnostics::fixed(rejection, code)
        })?;
        let meters = core::mem::take(&mut handles.meters);
        let effect_observations = core::mem::take(&mut handles.effect_observations);
        let mut carried: Vec<CarriedReader> = Vec::new();
        carried
            .try_reserve_exact(meters.len() + effect_observations.len())
            .map_err(|_| {
                PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.allocation")
            })?;
        carried.resize(
            meters.len() + effect_observations.len(),
            CarriedReader::UNUSED,
        );
        // Boxed here, so the meter header's address, which the record leaves for the commit, is
        // the address the page reads for as long as the tables live.
        let (shape, words) = shape;
        let shape = Box::new(shape);
        let header = core::ptr::from_ref(&shape.meter_header).addr();
        let mut companions = RenderCompanions {
            meters,
            effect_observations,
            carried: carried.into_boxed_slice(),
            carried_len: 0,
            carried_taps: None,
            shape: None,
        };
        // The attachment is the plan's own allocation, so the plan's rows charge it: the control
        // plane builds its plan resource row from this report, and nothing else would count the
        // box or the two vectors the companions keep alive for the plan's lifetime.
        let arithmetic =
            || PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic");
        let (mut retained, mut largest) = companions.retained_bytes().ok_or_else(arithmetic)?;
        if !boot {
            let (tables, tables_largest) = shape.retained_bytes().ok_or_else(arithmetic)?;
            retained = retained.checked_add(tables).ok_or_else(arithmetic)?;
            largest = largest.max(tables_largest);
        }
        charge_attachment(&mut host.report, caps, retained, largest)?;
        companions.shape = Some(shape);
        host.plan.set_host_attachment(Box::new(companions));
        if let Some(record) = &self.record {
            // The hand-off is checked here, before anything is committed: a preparation that
            // cannot leave the header's address for the host is refused, so a committed rebuild
            // always has it (`ControlHalf::run_control` relies on it).
            let mut record = record.lock().map_err(|_| {
                PrepareDiagnostics::fixed(PrepareRejection::Platform, SHAPE_UNAVAILABLE)
            })?;
            if boot {
                record.report = Some(host.report);
                record.superseded = Some(handles.live_values_superseded.clone());
            }
            record.successor = Some(PreparedSuccessor { header, words });
        }
        Ok((host, handles, spectrum_captures))
    }
}

/// Build one plan's tables (issue #1290a): `model` is the plan's normalized session, `producers`
/// its effect producers in the control plane's list order, `observations` its companions'
/// observation handles, `meter_count` its companions' meters. The same function builds the boot
/// plan's and every replacement plan's, so the two cannot disagree on what an index means.
///
/// Issue #1207 D1: the handles' strips are the tracks, then the submixes. Since issue #1213 D1
/// every queue band, per-strip table and meter word covers both segments, tracks first, so a
/// track's entry is the same with or without submixes. #1381a: the strip order is the normalized
/// model's, the order host-core's addressing authority uses and the control plane's producers
/// follow.
///
/// Errors are the diagnostic code of the refusal.
fn build_plan_shape(
    model: &session::SessionModel,
    producers: &[host_core::EffectControlProducer],
    observations: &[EffectObservationHandle],
    meter_count: usize,
) -> Result<(PlanShape, PublishedWordsWriter<2>), &'static str> {
    let track_count = model.tracks.len();
    let submix_count = model.submixes.len();
    let strip_count = track_count + submix_count;
    let mut tracks: Vec<Box<str>> = Vec::new();
    tracks
        .try_reserve_exact(track_count)
        .map_err(|_| "web.resource.allocation")?;
    tracks.extend(
        model
            .tracks
            .iter()
            .map(|track| Box::from(track.id.as_str())),
    );
    let mut submixes: Vec<Box<str>> = Vec::new();
    submixes
        .try_reserve_exact(submix_count)
        .map_err(|_| "web.resource.allocation")?;
    submixes.extend(
        model
            .submixes
            .iter()
            .map(|submix| Box::from(submix.id.as_str())),
    );
    let mut rack_effects = Vec::new();
    rack_effects
        .try_reserve_exact(strip_count)
        .map_err(|_| "web.resource.allocation")?;
    let count = |effects: usize| -> Result<u32, &'static str> {
        u32::try_from(effects).map_err(|_| "web.live_controls.effects")
    };
    // The lowered racks (decision 12), in chain order: every strip carries every `pre_insert` and
    // `post_insert` slot, so those counts are the session's. A live address (rack byte `3`
    // console or `1` inserts, #1096) reaches them through `dense_effect_slot`. Tracks first, then
    // submixes (issue #1207 D3): the strip order, so `rack_effects[t]` of a track never moves.
    for track in &model.tracks {
        rack_effects.push([
            count(model.console.pre_insert.len())?,
            count(track.inserts.effects.len())?,
            count(model.console.post_insert.len())?,
        ]);
    }
    for submix in &model.submixes {
        rack_effects.push([
            count(model.console.pre_insert.len())?,
            count(submix.inserts.effects.len())?,
            count(model.console.post_insert.len())?,
        ]);
    }
    // Issue #140 A: the dense effect-queue index. `effect_base[s]` is the number of effect
    // instances declared by every earlier strip, so `effect_slot` is arithmetic rather than a
    // search, and the producers are permuted into exactly that order once, here. The tracks lead,
    // so every track effect keeps its dense index and the submixes' effects follow them.
    let mut effect_base = Vec::new();
    effect_base
        .try_reserve_exact(strip_count)
        .map_err(|_| "web.resource.allocation")?;
    let mut total_effects = 0_u32;
    for counts in &rack_effects {
        effect_base.push(total_effects);
        for count in counts {
            total_effects = total_effects
                .checked_add(*count)
                .ok_or("web.live_controls.effects")?;
        }
    }
    // #1381a: the effect producers are the control plane's. The browser keeps, in the dense slot
    // order, what each instance is, so a read reaches its readers.
    let mut effect_instances: Vec<Option<EffectInstance>> = Vec::new();
    effect_instances
        .try_reserve_exact(total_effects as usize)
        .map_err(|_| "web.resource.allocation")?;
    effect_instances.resize_with(total_effects as usize, || None);
    for producer in producers {
        let (track_id, address, effect_id, descriptor) = (
            &producer.track_id,
            &producer.address,
            &producer.effect_id,
            producer.descriptor,
        );
        let Some(strip) = strip_index(&tracks, &submixes, track_id.as_bytes()) else {
            return Err("web.live_controls.effects");
        };
        let Some(entry) = dense_effect_slot(effect_base[strip], rack_effects[strip], *address)
            .and_then(|slot| effect_instances.get_mut(slot))
        else {
            return Err("web.live_controls.effects");
        };
        if entry.is_some() {
            return Err("web.live_controls.effects");
        }
        *entry = Some(EffectInstance {
            reader: None,
            track_id: track_id.clone(),
            effect_id: effect_id.clone(),
            descriptor,
        });
    }
    // `observation_tracks[slot]` is the strip index of that effect slot's observed instance, or
    // `u32::MAX` for a slot with no taps. Built once, here, so the poll's fold is arithmetic.
    let mut observation_tracks = vec![u32::MAX; total_effects as usize];
    let meter_tables =
        MeterTables::new(meter_count, strip_count).map_err(|_| "web.resource.allocation")?;
    // The plan's published frame and the header that describes it (#1290d1). The frame's address
    // is the heap block's, so it stays put as the tables move.
    let meter_frame = zero_meter_frame(strip_count)?;
    let mut meter_header = empty_meter_header();
    meter_header.track_count =
        u32::try_from(track_count).map_err(|_| "web.live_controls.effects")?;
    meter_header.submix_count =
        u32::try_from(submix_count).map_err(|_| "web.live_controls.effects")?;
    meter_header.frame_pointer = address_word(meter_frame.as_ptr());
    for (reader, handle) in observations.iter().enumerate() {
        let Some(strip) = strip_index(&tracks, &submixes, handle.track_id.as_bytes()) else {
            return Err("web.live_controls.observation");
        };
        let Some(slot) = dense_effect_slot(effect_base[strip], rack_effects[strip], handle.address)
        else {
            return Err("web.live_controls.observation");
        };
        let Some(instance) = effect_instances
            .get_mut(slot)
            .and_then(Option::as_mut)
            .filter(|instance| instance.reader.is_none())
        else {
            return Err("web.live_controls.observation");
        };
        // The frame carries one gain-reduction slot per strip, so every observed effect of a strip
        // points at that strip and the poll folds them max-magnitude into the one slot; a
        // submix's effect points at its strip index `T + j` (issue #1209 D5).
        observation_tracks[slot] =
            u32::try_from(strip).map_err(|_| "web.live_controls.observation")?;
        instance.reader = Some(u32::try_from(reader).map_err(|_| "web.live_controls.observation")?);
    }
    // #1332 S5: the plan's own words, which its commit stamps before publication.
    let (words_writer, words) = published_words(plan_words(None, 0));
    let shape = PlanShape {
        tracks,
        submixes,
        effect_instances: effect_instances.into_boxed_slice(),
        observation_tracks: observation_tracks.into_boxed_slice(),
        // #1689: no plan is prepared with a master; the control half's words point the shape at
        // the strip the core's `master` row designates after every control command
        // (`ControlHalf::sync_words`, `RenderReady::publish_words`).
        master_track: None,
        words,
        meter_tables: Some(meter_tables),
        meter_ring: None,
        meter_frame,
        meter_header,
        #[cfg(any(test, feature = "test-support"))]
        successor_address: 0,
    };
    Ok((shape, words_writer))
}

/// `3(T + S) + 3` words: two peak lanes and one gain-reduction magnitude per strip, then the
/// master's (issue #1209 D1). With `S = 0` it is the `3T + 3` frame every reader already knows.
fn zero_meter_frame(strip_count: usize) -> Result<Box<[f32]>, &'static str> {
    let count = strip_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(3))
        .ok_or("web.resource.arithmetic")?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| "web.resource.allocation")?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

/// The linear-memory address a header names a structure by: the pointer's address as a `u32`, or
/// `0` where the address does not fit (a native test host; the shipped `wasm32` module's always
/// does). The same conversion the exports use.
fn address_word<T>(pointer: *const T) -> u32 {
    u32::try_from(pointer.addr()).unwrap_or(0)
}

/// The bridge rows one plan's tables are charged (issue #1290a): the boot charges them to the
/// bridge, a replacement plan charges them to its attachment, so the swap budget counts them.
struct ShapeRows {
    /// The dense instance tables' own bytes. The observation index is the epoch's (#1589 R2),
    /// charged with the plan's observation rows.
    instance_table_bytes: u64,
    /// The IDs the dense instance table copies out of the producers.
    instance_id_bytes: u64,
    /// The meter delivery tables (#1582 R1): per meter a pending snapshot, a remaining count and
    /// a carried-from handle, and the core meter frame's per-strip tables (#1582 D2).
    meter_tables_bytes: u64,
    /// The largest of those tables, a single allocation.
    meter_tables_largest: u64,
    /// The published meter frame (#1290d1).
    frame_bytes: u64,
    /// The plan's own words' shared record (#1332 S5).
    words_bytes: u64,
}

/// The heap the two halves share for one session beyond its plans (#1332 S5), charged to the
/// bridge: the successor record, the lifecycle word's shared allocation and the hand-back ring,
/// whose bytes `ring` is. `None` on overflow.
fn cross_thread_bytes(ring: u64) -> Option<u64> {
    // `Arc`'s allocation is `{ strong, weak, data }`, as the engine's mirrors spell it.
    #[repr(C)]
    struct SharedWord {
        strong: usize,
        weak: usize,
        word: AtomicU32,
    }
    let successor = u64::try_from(engine::realtime::published_words_retained_bytes::<2>()).ok()?;
    let lifecycle = u64::try_from(size_of::<SharedWord>()).ok()?;
    successor.checked_add(lifecycle)?.checked_add(ring)
}

impl PlanShape {
    /// The rows this shape is charged, or `None` on arithmetic overflow.
    fn bridge_rows(&self) -> Option<ShapeRows> {
        // The IDs the dense instance table copies out of the producers are retained beside it.
        let instance_id_bytes =
            self.effect_instances
                .iter()
                .flatten()
                .try_fold(0_u64, |total, instance| {
                    total.checked_add(
                        u64::try_from(instance.track_id.len() + instance.effect_id.len()).ok()?,
                    )
                })?;
        let instance_table_bytes = u64::try_from(self.effect_instances.len())
            .ok()?
            .checked_mul((size_of::<Option<EffectInstance>>() + size_of::<u32>()) as u64)?;
        // #1582 R1: the delivery tables, held here from the plan's preparation to its adoption
        // and from its retirement on; the boot plan's are charged before the delivery takes them.
        let (meter_tables_bytes, meter_tables_largest) =
            self.meter_tables.as_ref().map_or((0, 0), |tables| {
                (tables.heap_bytes(), tables.largest_allocation_bytes())
            });
        let meter_tables_bytes = u64::try_from(meter_tables_bytes).ok()?;
        let meter_tables_largest = u64::try_from(meter_tables_largest).ok()?;
        let frame_bytes = u64::try_from(self.meter_frame.len())
            .ok()?
            .checked_mul(size_of::<f32>() as u64)?;
        let words_bytes =
            u64::try_from(engine::realtime::published_words_retained_bytes::<2>()).ok()?;
        Some(ShapeRows {
            instance_table_bytes,
            instance_id_bytes,
            meter_tables_bytes,
            meter_tables_largest,
            frame_bytes,
            words_bytes,
        })
    }

    /// The heap a replacement plan's attachment holds for these tables, and the largest single
    /// allocation: the shell, then the rows. `None` on overflow.
    fn retained_bytes(&self) -> Option<(u64, u64)> {
        let rows = self.bridge_rows()?;
        let shell = u64::try_from(size_of::<Self>()).ok()?;
        let total = shell
            .checked_add(rows.instance_table_bytes)?
            .checked_add(rows.instance_id_bytes)?
            .checked_add(rows.meter_tables_bytes)?
            .checked_add(rows.frame_bytes)?
            .checked_add(rows.words_bytes)?;
        let largest = shell
            .max(rows.instance_table_bytes)
            .max(rows.meter_tables_largest)
            .max(rows.frame_bytes);
        Some((total, largest))
    }
}

/// Charge the render companions' own heap to the plan's rows, after checking the charged total
/// against the caps the plan was prepared under, so a plan that fits only without its companions is
/// refused before they are boxed.
///
/// The cap on `graph_session_plus_plan_bytes` governs the graph, the plan and the compiled model
/// together (`HostPrepareCaps`), and the report's graph row leaves the model out, so the model is
/// added for the comparison. Host-core's own admission also counts the effect and route controls,
/// which neither row holds, so it refuses first for a session it would refuse; this is the check
/// of what only the browser adds. A refusal leaves the report as it was.
fn charge_attachment(
    report: &mut host_core::HostPrepareReport,
    caps: &HostPrepareCaps,
    retained: u64,
    largest: u64,
) -> Result<(), PrepareDiagnostics> {
    let arithmetic =
        || PrepareDiagnostics::fixed(PrepareRejection::Platform, "web.resource.arithmetic");
    let graph_session_plus_plan = report
        .graph_session_plus_plan_bytes
        .checked_add(retained)
        .ok_or_else(arithmetic)?;
    let graph_incremental_plan = report
        .graph_incremental_plan_bytes
        .checked_add(retained)
        .ok_or_else(arithmetic)?;
    let governed = graph_session_plus_plan
        .checked_add(report.session_model_bytes)
        .ok_or_else(arithmetic)?;
    if governed > caps.maximum_graph_session_plus_plan_bytes {
        return Err(PrepareDiagnostics::fixed(
            PrepareRejection::Resource,
            "host.graph.resource.limit",
        ));
    }
    let largest_allocation = report.largest_engine_allocation_bytes.max(largest);
    if largest_allocation > caps.maximum_named_allocation_bytes {
        return Err(PrepareDiagnostics::fixed(
            PrepareRejection::Resource,
            "host.resource.limit",
        ));
    }
    report.graph_session_plus_plan_bytes = graph_session_plus_plan;
    report.graph_incremental_plan_bytes = graph_incremental_plan;
    report.largest_engine_allocation_bytes = largest_allocation;
    Ok(())
}

/// Prepare the session through the control plane, then fold its report into the frozen browser
/// report (#1381a D1).
///
/// The control plane's `compile_children` prepares the boot plan through [`WebRuntimePreparer`]
/// and owns the producers from then on: the sources, the strips' and effects' cells, the plan's
/// revision gate and the exchange's control ends. What this function builds is the browser's own:
/// the dense tables that address the readers in the plan's attachment and the report. Engine-owned rows come from the plan's
/// [`PlanResources`] row and the preparer's [`BootRecord`]; the browser bridge's own rows
/// (staging buffers, the host shell, the plane-reference table) were already computed by
/// `prepare_buffers`, and the three browser caps are applied here because they are the *browser's*
/// caps on the shared report.
fn compile_ready(
    document: &str,
    caps: &HostPrepareCaps,
    options: WebBootOptions,
    memory_budget: u64,
    control_frame_bytes: u32,
    mut report: WebResourceReport,
    boot_command: Option<&[u8]>,
) -> Result<(ControlHalf, RenderReady, WebResourceReport), BootFailure> {
    // #1312 D2, #1399: the session's `live_values_superseded` count is the control plane's, made
    // once before any plan is prepared; the preparer leaves a handle on it in the record, which the
    // status reads.
    let record = Arc::new(Mutex::new(BootRecord::default()));
    let preparer = WebRuntimePreparer::new(options).recording(Arc::clone(&record));
    let limits = control_limits(
        caps,
        memory_budget,
        control_frame_bytes,
        maximum_document_bytes(&options),
    );
    // #2166: a staged boot command (the SDK's boot spectrum taps) is applied before the first plan
    // is prepared, so the taps are that plan's and no rebuild is counted for them.
    let compiled = match boot_command {
        None => control_plane::compile_children::<WebAdapter>(document, limits, Box::new(preparer)),
        Some(frame) => control_plane::compile_children_with_boot_command::<WebAdapter>(
            document,
            limits,
            Box::new(preparer),
            frame,
        ),
    };
    let control_plane::CompiledChildren {
        session: mut control,
        mut plan,
    } = compiled.map_err(|failure| boot_refusal(&failure, &record))?;
    let (engine, live_values_superseded, boot_words) = {
        let mut record = record
            .lock()
            .map_err(|_| fixed_diagnostic("web.internal.boot_record"))?;
        // The boot plan's header address is the render half's own from here on: nothing names it
        // as a successor. Its words' writer is the control half's (#1332 S5).
        match (
            record.report.take(),
            record.superseded.take(),
            record.successor.take(),
        ) {
            (Some(engine), Some(superseded), Some(successor)) => {
                (engine, superseded, successor.words)
            }
            _ => return Err(fixed_diagnostic("web.internal.boot_record").into()),
        }
    };
    let resources = plan.queries().resources();
    let session = control.committed_session();
    let session_quantum_frames = session.quantum().0;

    let control_table = control_table_bytes(engine.source_count as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let id_arena = source_id_arena_bytes(engine.source_id_bytes as usize)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    // Charged because the browser bridge retains them: everything the control plane keeps for the
    // session (its `control_retained_bytes` row: the facade's control table and ID arena, the
    // plan's state inventory (issue #1272), the strip, effect and send producers with their
    // payload, the protocol queues, the replay cache and the session's `live_values_superseded`
    // count (#1399)), and the compiled session model, which the control plane's store keeps and
    // the bridge reads.
    let ready_metadata =
        checked_sum_prepare([resources.control_retained_bytes, engine.session_model_bytes])?;
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
        .max(control.catalog_resources().largest_allocation_bytes)
        .max(engine.session_largest_allocation_bytes);
    let largest_named = bridge_largest.max(engine.largest_engine_allocation_bytes);
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
    // Issue #143 R7: the engine's walked row, carried through unchanged.
    report.observation_retained_bytes = engine.observation_retained_bytes;
    // #1290a: the tables were built by the boot plan's own preparation (`build_plan_shape`), the
    // same function every replacement plan's preparation runs, and ride its companions until the
    // render half takes them here (#1332 S4). The boot plan's spectrum captures, if a boot command
    // set taps (#2166), went to the control plane's spectrum monitor.
    let mut shape = plan
        .active_attachment_mut::<RenderCompanions>()
        .ok_or_else(|| fixed_diagnostic("web.internal.companions"))?
        .shape
        .take()
        .ok_or_else(|| fixed_diagnostic("web.internal.companions"))?;
    let rows = shape
        .bridge_rows()
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(rows.instance_table_bytes)
        .and_then(|bytes| bytes.checked_add(rows.instance_id_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(rows.instance_table_bytes)
        .and_then(|bytes| bytes.checked_add(rows.instance_id_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(rows.instance_table_bytes);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    // The master windows are the control plane's output meter (#1581 D5): its queue and words are
    // in the control plane's retained row, charged above with it.
    let meter_delivery_bytes = rows
        .meter_tables_bytes
        .checked_add(rows.frame_bytes)
        .and_then(|bytes| bytes.checked_add(rows.words_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(meter_delivery_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(meter_delivery_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.largest_bridge_allocation_bytes = report
        .largest_bridge_allocation_bytes
        .max(rows.meter_tables_largest)
        .max(rows.frame_bytes);
    report.largest_named_allocation_bytes = report
        .largest_named_allocation_bytes
        .max(report.largest_bridge_allocation_bytes);
    let queries = plan.queries();
    // #1290a D3: a replacement is admitted against the one budget. What it leaves for the plans is
    // the budget less every byte the bridge retains, the committed model aside: the replacement's
    // own check counts both compiled models.
    control.set_replacement_peak_limit(
        memory_budget
            .saturating_sub(report.bridge_retained_bytes)
            .saturating_add(engine.session_model_bytes),
    );
    // #1581 D5: the session's output meter is the control plane's; its reader and gate are the
    // host's, handed out once, here.
    let (output_reader, output_gate) = control
        .take_output_meter()
        .ok_or_else(|| fixed_diagnostic("web.internal.output_meter"))?;
    let structure_epoch = control.structure_epoch();
    // #1582 D1: the session's one meter delivery takes the boot plan's tables (charged above with
    // the plan's rows) and the output meter's halves; the boot plan's first block is the grid's
    // origin.
    let mut meter_tables = shape
        .meter_tables
        .take()
        .ok_or_else(|| fixed_diagnostic("web.internal.meter_tables"))?;
    // #1588 D1, D2: the session's level word, and the boot plan's ring, which the preparer moved
    // into the boot plan's tables (#1332 S4); the boot plan's frames name its epoch (D3).
    let meter_level = control
        .meter_level()
        .ok_or_else(|| fixed_diagnostic("web.internal.meter_level"))?;
    let boot_epoch = control.newest_plan_epoch();
    meter_tables.set_plan_epoch(boot_epoch);
    shape
        .meter_ring
        .as_mut()
        .ok_or_else(|| fixed_diagnostic("web.internal.meter_ring"))?
        .epoch = boot_epoch;
    let meter_window_frames = options
        .live_control_meter_blocks
        .checked_mul(u64::from(session_quantum_frames))
        .unwrap_or(0);
    shape.meter_header.structure_epoch = structure_epoch;
    // #1332 S5: the words, the hand-back ring and the lifecycle word the two halves share. The
    // boot plan's words: no master (#1689) and the boot's structure epoch, which the boot plan's
    // header already carries; the successor record names no successor.
    let mut plan_words_writer = boot_words;
    plan_words_writer.publish(&plan_words(None, structure_epoch));
    let (successor_writer, successor_reader) = published_words::<2>([0, 0]);
    let (retired_producer, retired_consumer) =
        engine::realtime::bounded_spsc_move::<Box<PlanShape>>(
            NonZeroUsize::new(RETIRED_SHAPES)
                .ok_or_else(|| fixed_diagnostic("web.internal.retired"))?,
            engine::realtime::QueueGeneration(0),
        )
        .map_err(|_| fixed_diagnostic("web.resource.allocation"))?;
    let lifecycle: Lifecycle = Arc::new(AtomicU32::new(STATE_READY));
    let shared_bytes = cross_thread_bytes(retired_consumer.retained_bytes())
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_metadata_bytes = report
        .bridge_metadata_bytes
        .checked_add(shared_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    report.bridge_retained_bytes = report
        .bridge_retained_bytes
        .checked_add(shared_bytes)
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let bridge_fixed_bytes = report
        .bridge_retained_bytes
        .checked_sub(engine.session_model_bytes)
        .and_then(|bytes| bytes.checked_sub(resources.control_retained_bytes))
        .and_then(|bytes| bytes.checked_sub(report.session_document_bytes))
        .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))?;
    let control = ControlHalf {
        control,
        #[cfg(feature = "test-support")]
        limits,
        queries,
        record,
        blocking_rebuilds: 0,
        render_here: true,
        plan_words: plan_words_writer,
        successor: successor_writer,
        successor_header: 0,
        retired_shapes: retired_consumer,
        lifecycle: Arc::clone(&lifecycle),
        staging: ControlStaging::empty(),
        last_result: RESULT_OK,
        bridge_fixed_bytes,
    };
    let ready = RenderReady {
        shape,
        shape_epoch: 0,
        parked: None,
        retired_shapes: retired_producer,
        successor: successor_reader,
        lifecycle,
        #[cfg(feature = "test-support")]
        adoption_pause: None,
        live_values_superseded,
        meters: MeterDelivery::new(
            output_reader,
            output_gate,
            meter_window_frames,
            meter_tables,
        ),
        meter_level,
        meter_applied: 0,
        #[cfg(test)]
        meter_effect_scan_count: 0,
        plan,
    };
    Ok((control, ready, report))
}

/// Translate the browser configuration's meter word into the facade's live-control request.
///
/// The meter word passes unchanged (issue #1581 D2): `live_control_meter_blocks == 0` binds the
/// strip meters at host-core's default window, and the lease over such a host is still refused
/// (`set_meter_lease`). The port lease is a second, finer switch over posting.
fn live_control_request(
    options: WebBootOptions,
    quantum_frames: u32,
) -> Option<HostLiveControlRequest> {
    let meter_window_blocks = u32::try_from(options.live_control_meter_blocks).ok()?;
    // A window longer than `u32::MAX` frames is refused here, as host-core would.
    meter_window_blocks.checked_mul(quantum_frames)?;
    Some(HostLiveControlRequest {
        meter_window_blocks,
        meter_queue_depth: NonZeroUsize::new(METER_QUEUE_DEPTH)?,
        // #1590 D5: every declared tap of every effect, as the C ABI asks.
        observation_taps: host_core::launch_maximum_observation_taps(),
    })
}

/// One `code\t$\n` diagnostic line for a rule that names no session path.
fn fixed_diagnostic(code: &str) -> Vec<u8> {
    host_core::fixed_diagnostic_line(code)
}

fn checked_sum_prepare(values: impl IntoIterator<Item = u64>) -> Result<u64, Vec<u8>> {
    values.into_iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(value)
            .ok_or_else(|| fixed_diagnostic("web.resource.arithmetic"))
    })
}

fn boxed_zero_u8(bytes: u32) -> Result<Box<[u8]>, u32> {
    let count = usize::try_from(bytes).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize(count, 0);
    Ok(value.into_boxed_slice())
}

fn boxed_zero_f32(samples: u64) -> Result<Box<[f32]>, u32> {
    let count = usize::try_from(samples).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize(count, 0.0);
    Ok(value.into_boxed_slice())
}

fn boxed_uninit_planes(channels: u32) -> Result<Box<[MaybeUninit<&'static [f32]>]>, u32> {
    let count = usize::try_from(channels).map_err(|_| RESULT_REFUSED_BUDGET)?;
    let mut value = Vec::new();
    value
        .try_reserve_exact(count)
        .map_err(|_| RESULT_REFUSED_BUDGET)?;
    value.resize_with(count, MaybeUninit::uninit);
    Ok(value.into_boxed_slice())
}

mod ffi;
mod handoff;
mod render_lock;

pub use ffi::*;
pub use handoff::{
    CANARY_BYTES, HANDOFF_SLOTS, INSTANCE_RESERVATION_BYTES, INSTANCE_STACK_BYTES, Reservation,
    WebInstanceReservation,
};
pub use render_lock::RenderLockedAllocator;
#[cfg(feature = "render-lock-probe")]
pub use render_lock::miso_engine_web_v1_render_lock_probe;

#[cfg(test)]
mod tests;
