#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The console benchmark's *subject*, shared by the native bench and the wasm guest.
//!
//! # Why this crate exists
//!
//! Issue [#163](https://github.com/misofm/engine/issues/163) phase 2 opens with the owner
//! ruling that the unfused multiply-add contract change must be confirmed **at console level**,
//! not at kernel level. Confirming it at console level means running the console workloads under
//! the target the product ships on, and `docs/rulings/wasm-kernel-timing-interim.md` recorded why
//! that was not reachable in phase 0: `tools/bench/src/console.rs` and every compiler
//! it drives were absent from a `wasm32` build of the bench crate.
//!
//! The recorded blocker turned out to be **bench-tool-level, not crate-level**. All four
//! compilers -- `session`, `builtins-compiler`,
//! `effect-compiler` and `graph-compiler` -- build for
//! `wasm32-unknown-unknown` today, unchanged. The `cfg(not(target_arch = "wasm32"))` gates were
//! entries in the *bench manifest*, expressing that the bench binary is a native tool, and not a
//! statement that the subject could not target wasm. So the port needed no crate change at all:
//! it needed the subject to live somewhere both a native binary and a wasm guest can link it.
//!
//! That is this crate. It holds the console workloads, the model derivation, and the
//! prepared-plan runtime that renders them -- lifted verbatim out of `console.rs`, which now links
//! it. Nothing was reimplemented for wasm and nothing is conditional on the target, so the wasm
//! guest and the native bench execute **the same subject**: the same fixtures, the same strip
//! edits, the same caps, the same source bindings, the same `PreparedRenderPlan`, the same render
//! call. A number taken through the guest and a number taken through the bench differ in the
//! target that executed them and in nothing else, which is the only condition under which their
//! ratio means anything.
//!
//! This follows the shape gate G5 already established: `wasm-gate-corpus` is an
//! `rlib` precisely so the native leg links the identical code the `cdylib` guest does.
//!
//! # What is deliberately *not* here
//!
//! The measurement. There is no clock in this crate, no percentile, no record and no statistic.
//! `wasm32-unknown-unknown` cannot construct a `std::time::Instant`, so a subject that timed
//! itself could not be linked into the guest at all -- and a subject that times itself on one
//! target and is timed from outside on another is two subjects. Timing belongs to whichever
//! driver owns a clock: `console.rs` for the native bench, the wasmtime host for the guest.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use std::collections::BTreeSet;

use bench_support::digest::Sha256Sink;
use builtins::{MeterConfig, MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::{MeterConsumer, MeterRequest, SelectedMeterRequest};
use effect_compiler::{
    EffectCompileCaps, EffectControlProducer, EffectObservationHandle, attach_effect_console,
    attach_effect_observation, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::{
    ChannelSymmetryWitness, EffectControlRecord, ParameterChannel, PreparedEffectTarget,
};
use engine::realtime::{
    PlanUnitEligibility, PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime,
};
use graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphPreparedSourceSet,
    GraphPreparedSourceSetDriver, GraphRuntimeBindings, GraphRuntimeProcessor,
    GraphSourceInputClaim, GraphSourceSetResourceReport, TrackStage,
};
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use lane::Backend;
use session::{
    CompileCaps, DualMonoFader, MatrixOrPan, SessionModel, StableId, compile_session,
    parse_session_json,
};

pub mod mixing_automation;

/// Sample rate every console workload is prepared and rendered at.
pub const SAMPLE_RATE_HZ: u32 = 48_000;
/// Frames per rendered block.
pub const QUANTUM: usize = 128;
/// Plan identifier handed to the graph compiler. Issue #149, the console qualification issue.
pub const PLAN_ID: u64 = 149;
/// Blocks per published meter window and per published observation window.
///
/// Deliberately one number for both: a gain-reduction value and the peak beside it in one console
/// frame have to describe the same span of samples, which is the rule
/// `attach_effect_observation` states and the rule a host follows.
pub const WINDOW_BLOCKS: u32 = 4;
/// Bounded depth of each effect's live-console control channel in the observation arms.
pub const CONTROL_QUEUE_DEPTH: usize = 8;
/// Cap on declared observation taps per effect, passed to the observation attach.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 8;
/// Bounded depth of each meter stream. Drained outside the clock after every observation.
pub const METER_QUEUE_DEPTH: usize = 8;
/// Blocks per meter window on the metered console row (issue #881): the default web boot's.
///
/// `WebBootOptions::console_defaults` sets `console_meter_blocks` to the web host's
/// `DEFAULT_METER_BLOCKS`, twelve, and its `console_request` turns that into a period of
/// `12 * quantum_frames`. Mirrored here rather than imported: the browser host is not a
/// dependency of this subject, and this crate names the number it renders with.
pub const WEB_METER_BLOCKS: u32 = 12;
/// Bounded depth of each meter stream on the metered console row: the depth the web host's
/// `console_request` asks for (one window per post, plus headroom for a stalled control side).
pub const WEB_METER_QUEUE_DEPTH: usize = 8;

const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
/// The retired 64-track fixture: EQ on `simd1`, compressor in the `dynamic` rack, no limiter.
///
/// Kept, and still rendered, by exactly one row. See [`Workload::SixtyFourTrackConsoleLegacy`].
const SIXTY_FOUR_TRACK_LEGACY: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");
/// The standing 64-track qualification fixture (#175): the intended production rack layout.
///
/// EQ and compressor share one two-slot chain on `simd1`; a true-peak limiter sits alone on
/// `simd2`. Generated from the retired fixture by `scripts/derive-intended-console-fixture.py`,
/// which moves the compressor's declaration verbatim, so every EQ and compressor coefficient is
/// byte-identical between the two files and the only arithmetic that is new is the limiter's.
const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");
/// The mono qualification fixture: the standing strip, collapse-eligible upstream of the seam.
///
/// Generated from the standing fixture by `scripts/derive-mono-console-fixture.py`, which makes
/// three edits and every one of them is upstream of the fader/matrix seam -- both channels read
/// source channel 0, `builtins.right` copies `builtins.left`, and every `channel = "right"`
/// effect parameter takes its `channel = "left"` sibling's value. Those are exactly the two
/// structural terms of the per-track channel-symmetry witness
/// (`effect_contract::ChannelSymmetryWitness`: `SOURCE` and `DESIGNED`), so every
/// track of this fixture is collapse-eligible.
///
/// The fader and pan asymmetry and the limiter's `maximum` link are deliberately *kept*; the
/// generator's header says why, and so does the fixture's own.
const SIXTY_FOUR_TRACK_MONO: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json");

/// The standing session workloads, in emission order.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Workload {
    /// The inherited ragged baseline: nine tracks, EQ only, one full bank plus a scalar tail.
    ///
    /// Kept because it is the fixture the sprint's prior numbers were taken on. It carries no
    /// compressor, so its per-track cost is **not** comparable with the console workloads; that is
    /// what `NineTrackRaggedStrip` is for.
    NineTrackBaseline,
    /// The same channel strip as the console fixture, truncated to nine tracks.
    ///
    /// This is the honest ragged-versus-full comparison: identical strip, identical parameters,
    /// nine tracks (one full eight-lane bank plus a one-track tail) against sixty-four (eight full
    /// banks, no tail). Any per-track difference between this row and the console row is the cost
    /// of the ragged shape and nothing else.
    NineTrackRaggedStrip,
    /// The qualification session: sixty-four full channel strips, eight full banks, no tail.
    SixtyFourTrackConsole,
    /// The stretch fixture: the same strip at 128 tracks, synthesised from the 64-track model.
    OneTwentyEightTrackStretch,
    /// Decomposition (#163 item 0c): the console strip with the dynamic rack emptied.
    ///
    /// Sixty-four tracks of EQ and nothing else. `NineTrackBaseline` is also EQ-only, but at nine
    /// ragged tracks on a different fixture, so it cannot be subtracted from the console row. This
    /// row can: it is the *same* fixture, the same parameters and the same track count, with one
    /// rack emptied, so `sixty_four_track_console - sixty_four_track_eq_only` is the compressor's
    /// share of the block and nothing else.
    SixtyFourTrackEqOnly,
    /// Decomposition: the console strip with SIMD rack 1 emptied. Compressor and builtins only.
    SixtyFourTrackCompressorOnly,
    /// Decomposition: every rack emptied. Input trim/HPF/LPF, fader and pan matrix only.
    SixtyFourTrackBuiltinsOnly,
    /// Decomposition: every rack emptied, **and** every input builtin and the fader asked for their
    /// identity.
    ///
    /// Polarity off, trim 0 dB, HPF and LPF at 0 Hz, fader 0 dB unmuted, and a pan of
    /// `left = right = 1.0` with no smoothing. That pan is **not** the identity matrix: it routes
    /// both inputs hard right (`ll = lr = cos(pi / 2)`, which is `6.1e-17` in `f32`, and
    /// `rl = rr = 1`). The pan law cannot produce the identity, so no standing row exercises an
    /// identity matrix; ruling 4 of issue #944 corrected this doc and kept the row's content, and
    /// with it the row's digest.
    ///
    /// What this row is **not**: it is not the cost of dispatch alone, and the record says
    /// `identity` rather than `dispatch` for that reason. A builtin filter at 0 Hz is *disabled*,
    /// and `SvfSection::design` implements disabled by designing an identity section -- `m0 = 1`,
    /// `m1 = m2 = 0`, `k = 0` (`builtins`, the version-1 cutoff contract). A 0 dB fader
    /// is still a multiply and a mask clear, and the hard-right pan is a real 2x2 matrix: with no
    /// identity lane in any bank, it runs the settled matrix's select-free arm (issue #944), the
    /// same arm `sixty_four_track_gain_pan_only` runs. Both run over the same lanes every block.
    ///
    /// The two SVF sections no longer do. A prepared section that is the exact identity in every
    /// lane and every word is the map `v |-> v + 0.0`, so a run of them is one `add(+0.0)`, and
    /// `input_chain_block_elided` emits that instead of the recurrence when the bank's prepared
    /// words say it may. The decision is made once, at bank construction, from the coefficient and
    /// state bits.
    ///
    /// So this row measures: source fill, per-node graph dispatch, buffer plumbing, route
    /// summation, the sanitisation and boundary-scan passes the D7 policy requires of every block,
    /// the fader kernel running its identity coefficients, and the matrix kernel running a
    /// hard-right pan.
    ///
    /// **The near-equality reading is retired.** Before the elision, the two rack-free rows ran the
    /// same instructions over the same lanes with different constants, and their near-equality was
    /// the evidence for that reading; the ruling's own text recorded 22.833 and 21.962 µs. Now the
    /// gap is the elision, and it is the *expected* shape: this row must come in materially below
    /// `sixty_four_track_builtins_only`, and a return to near-equality would mean the elision
    /// stopped firing. Neither the old gap nor the new one is pinned as a number here -- they are
    /// host-dependent, and the sealed records under `artifacts/` are where the measurements live.
    SixtyFourTrackDispatchOnly,
    /// The idle row: the full console strip rendering silence.
    ///
    /// Honest statement of what this measures, because the name invites a stronger reading than
    /// the number supports. The plan is the unmodified sixty-four-track console. Every effect,
    /// builtin, fader and matrix is prepared and armed exactly as in `sixty_four_track_console`.
    /// The only difference is the input: every track's source binding writes zeros instead of a
    /// tone, and the arm is warmed for long enough that every recursive filter and every detector
    /// has settled to its silent steady state before the clock starts.
    ///
    /// It is therefore **not** "the cost of a stopped engine". No transport gate, silence gate or
    /// early-out exists on any render path in this tree (#163 phase 4 is the issue that would add
    /// one), so a prepared console renders silence through the entire chain at very nearly the
    /// cost of rendering music. That equality is the finding; this row is the number that states
    /// it. Nothing here is scheduled, decoded or transported: the row measures render only.
    SixtyFourTrackIdle,
    /// The retired layout, kept for one transition record (#175): EQ on `simd1` and the
    /// compressor in the `dynamic` rack -- **two one-slot chains** -- and no limiter.
    ///
    /// This is the shape every console record up to and including
    /// `artifacts/issue163-phase2/` measured, rendered here from the unmodified retired fixture.
    /// It exists so the handover to the intended-placement fixture is a *measured* step rather
    /// than an announced one: this row and `sixty_four_track_console` are taken on one host in
    /// one run, so the number the retired authority reported and the number the standing
    /// authority reports can be read against each other exactly once, and afterwards the retired
    /// row can go.
    ///
    /// It is also one half of the chain-shape row-pair. Against
    /// `sixty_four_track_eq_comp_simd1` -- the same two effects, the same coefficients, the same
    /// order, differing only in whether they are one two-slot chain or two one-slot chains -- the
    /// difference is the per-chain AoSoA round-trip and nothing else, and the two rows must
    /// render byte-identically (#166).
    SixtyFourTrackConsoleLegacy,
    /// The other half of the chain-shape row-pair: EQ and compressor as **one two-slot chain**
    /// on `simd1`, with `simd2` emptied.
    ///
    /// The standing fixture with its limiter removed, which makes it the intended layout's
    /// chain shape carrying the retired layout's arithmetic. Two subtractions meet here:
    /// `sixty_four_track_console - sixty_four_track_eq_comp_simd1` is the limiter's cost, and
    /// `sixty_four_track_console_legacy - sixty_four_track_eq_comp_simd1` is the chain-shape
    /// delta -- one AoSoA round-trip per bank per block, and no arithmetic at all.
    SixtyFourTrackEqCompSimd1,
    /// Decomposition: every rack emptied and every input builtin asked for its identity, with the
    /// fixture's **real** fader and pan values left as written.
    ///
    /// The controlled partner of `sixty_four_track_dispatch_only`. The two rows execute the same
    /// instructions over the same lanes -- both elide their prepared-identity input sections, both
    /// run `gain_mute_block` and the settled matrix's select-free arm on every bank -- and differ
    /// only in the *constants* those two kernels carry: 0 dB and a hard-right pan there, the
    /// fixture's declared per-channel fader trims and pan positions here.
    ///
    /// That makes the pair a direct measurement of a claim the floor table asserts: a 0 dB fader
    /// costs exactly what a real one costs, because `gain_mute_block` has no identity arm, and a
    /// hard-right pan costs what a real pan costs, because neither is the identity matrix. The
    /// settled matrix does have one data-dependent path since issue #944 -- a bank with any
    /// identity lane keeps the per-lane identity select, and a bank with none skips it -- and
    /// neither row has an identity lane, so both take the same arm. The two rows share a floor
    /// (22 lane-ops) for precisely that reason, and a material gap between them would mean one of
    /// the two kernels had acquired another data-dependent path.
    ///
    /// It is the bound-feed twin of [`Self::SixtyFourTrackGainPanRing`], and it is **not** the
    /// native pure-path target: every one of its track inputs is a `FrozenGraphSource` processor,
    /// one dispatched unit per track per block that copies a frozen block into the arena, and no
    /// host feeds a session that way. The ring row is the same session fed the way a host feeds it.
    SixtyFourTrackGainPanOnly,
    /// The native pure-audio-path target: [`Self::SixtyFourTrackGainPanOnly`]'s session fed the way
    /// a host feeds a session, through a prepared source set instead of a bound processor per track
    /// input (issue #956, re-basing the driver-fed row of issue #928).
    ///
    /// The same strip edit, the same compile with builtins and the same frozen tone as
    /// `gain_pan_only`; the difference is how its sixty-four `TrackStage::Input` nodes are fed.
    /// The builtins artifact is bound through `into_bound_with_source_set`, the entry `host-core`
    /// binds a session through (`crates/host-core/src/prepare.rs`), and the set's driver,
    /// `FrozenSourceDriver`, serves each claim the frozen block the bound feed's processor copies.
    /// Every claim's only reader is its cohort's `PostInputBuiltins` bank, whose gather reads the
    /// played block in place (issue #918, `source_plane_table` clause (b)), so no claim is copied
    /// into the arena and the input units are left out of the dispatched-unit table (issue #936).
    /// That is the production feed, the one `crates/host-core/tests/source_in_place.rs` pins, and
    /// it is why this row -- not the bound-feed `gain_pan_only` -- is the native pure-path target.
    ///
    /// Both feeds deliver the same frozen words, honouring the same channel mapping, so the two
    /// rows render the same bits; a digest difference between them is a harness defect, never a
    /// finding. The row replaced the builtins-less `sixty_four_track_plumbing_ring`, which fed the
    /// same driver into a plan no host compiles (issue #956).
    ///
    /// Not in [`WORKLOADS`]: see [`DRIVER_FED_WORKLOADS`].
    SixtyFourTrackGainPanRing,
    /// The mono qualification session: sixty-four collapse-eligible strips, rendered as written.
    ///
    /// The same strip, the same coefficients and the same input as `sixty_four_track_console`,
    /// from a fixture whose every track satisfies the channel-symmetry witness' two structural
    /// terms. Today it is an ordinary session row: no code reads the witness and nothing collapses,
    /// so this row and [`Self::SixtyFourTrackConsoleMonoDual`] compile, prepare and render exactly
    /// the same plan.
    ///
    /// That is deliberate and it is the point. When the collapse lands, *this* row is the one that
    /// takes it and the `_dual` row is the one that forces it off, and the digest equality between
    /// them -- asserted in-run today, trivially -- becomes the standing class-A gate on the whole
    /// mechanism. Building the pair now means the gate exists before the thing it gates, rather
    /// than being written by the same change it is supposed to check.
    SixtyFourTrackConsoleMono,
    /// The mono row's control arm: the identical session with the collapse forced off.
    ///
    /// See [`Self::SixtyFourTrackConsoleMono`]. The two arms are one session today and the
    /// `console_mono` record says so in its own `arms_identical_today` field, so a reader cannot
    /// mistake today's zero delta for a measured saving.
    SixtyFourTrackConsoleMonoDual,
    /// The mixed-cohort row: thirty-two collapse-eligible tracks and thirty-two that are not,
    /// alternating, so every eight-lane cohort carries four of each.
    ///
    /// Derived in code from the mono fixture by putting `right_source_channel = 1` back on the odd
    /// tracks -- undoing, on half the tracks, the one edit the generator made to the source
    /// mapping. Those tracks then read two different source channels, which clears the witness'
    /// `SOURCE` term, and they render genuinely different left and right samples rather than
    /// merely declaring that they might.
    ///
    /// It exists because a cohort is banked, not a track. A collapse that is decided per track has
    /// to survive a bank whose lanes disagree about it, and the uniform rows cannot see that
    /// failure at all: `_mono` collapses every lane and `console` collapses none, so both are
    /// homogeneous cohorts. Alternating is what makes every cohort mixed rather than only the
    /// boundary ones.
    ///
    /// Its class-A statement is a *shape* statement and is asserted natively, in
    /// `tools/console-workload/tests/chain_shape.rs`: a mixed cohort must realise the
    /// same `[chains, slots]` and the same planar/AoSoA round-trip count as a uniform one. The
    /// wasm host reports no shape, which is why that gate lives beside the fixtures rather than in
    /// the record.
    SixtyFourTrackConsoleHalfMono,
    /// The standing console session with the meter every browser track carries (issue #881).
    ///
    /// [`Self::SixtyFourTrackConsole`]'s session exactly as written -- the same fixture, strip,
    /// compile and sources -- prepared with one `SAMPLE_PEAK` meter at `PostMatrix` on every track.
    /// That is the meter set the default web boot binds: a window of [`WEB_METER_BLOCKS`] blocks
    /// (12 x 128 frames), no peak hold, peak decay off, a queue [`WEB_METER_QUEUE_DEPTH`] snapshots
    /// deep, and handles `index + 1` in the compiled session's normalized track order. The meters
    /// are bound as **permanent** observers through
    /// `builtins_compiler::prepare_selected_session_builtins_between_render_calls`, which is the
    /// entry `host_core::prepare_host_runtime_with_selected_meters_between_render_calls` calls for
    /// that boot. No live-console control channel is attached.
    ///
    /// **The meters are not the only difference from the standing row.** That entry also selects
    /// `BuiltinControlDelivery::BetweenRenderCalls`, as the default web boot does, and under that
    /// delivery the builtins pair each cohort's fader bank and matrix bank into one fused stage
    /// (`FaderMatrixBankProcessor`, rendering through `fader_matrix_block`). The standing row is
    /// prepared with `Concurrent` delivery, so it keeps the fader and the matrix as two stages and
    /// its matrix renders through `MatrixStage::process`. Both plans bank the same 48 memberships
    /// in eight chains, but the standing row runs 48 bank-chain stages per block and this row 40;
    /// this crate's pair test pins both counts. So this row minus the standing row is the meters'
    /// cost **plus** the fused-versus-split fader and matrix, and a change to either fader/matrix
    /// path can move one row of the pair and not the other.
    ///
    /// It is the row the observer path can move. Every other session row renders with no
    /// observer, and the `console_meters` arm binds all-metric meters at a four-block window
    /// through the concurrent entry, which is not what a browser binds. This is the configuration
    /// issue #943 (one sample-peak pass per bank, at the resident final lane) moves.
    ///
    /// Meters observe and never change signal flow, and the fused fader and matrix render the
    /// split pair's bits, so this row renders the standing console row's bits: the run asserts it
    /// before it emits either record, and the aggregate validator pins it. Its snapshots are
    /// consumed after every block, outside the clock, as a host holding the meter lease consumes
    /// them.
    ///
    /// Not in [`WORKLOADS`]: see [`METERED_WORKLOADS`].
    SixtyFourTrackConsoleMetered,
}

impl Workload {
    /// Whether this row is prepared with the default web boot's meter set (issue #881).
    ///
    /// True for exactly one row, [`Self::SixtyFourTrackConsoleMetered`]. Every other row's meters
    /// come from [`PlanConfig::meters`], which is false on every session row.
    #[must_use]
    pub const fn web_meters(self) -> bool {
        matches!(self, Self::SixtyFourTrackConsoleMetered)
    }

    /// Whether this row renders with the mono collapse **forced off**.
    ///
    /// True for exactly one row. `sixty_four_track_console_mono_dual` compiles the same
    /// collapse-eligible fixture as `sixty_four_track_console_mono` and renders it dual, which is
    /// what makes the pair a measurement of the collapse rather than of two sessions: the arms
    /// differ by this `bool` and by nothing else, and the run asserts their digests agree before it
    /// emits a number.
    ///
    /// Every other row -- the stereo fixture included -- leaves the collapse armed and simply
    /// declines it, which is what makes "the stereo row is byte-identical to its seal" a statement
    /// about the *dispatch* rather than about a switch someone remembered to set.
    #[must_use]
    pub const fn collapse_forced_off(self) -> bool {
        matches!(self, Self::SixtyFourTrackConsoleMonoDual)
    }

    /// How this row's track inputs reach the graph, named in the record as `source_feed`.
    ///
    /// [`SourceFeed::PlayedPlanes`] for exactly one row, the driver-fed gain/pan row; every other
    /// row binds a `FrozenGraphSource` processor per track input.
    #[must_use]
    pub const fn source_feed(self) -> SourceFeed {
        match self {
            Self::SixtyFourTrackGainPanRing => SourceFeed::PlayedPlanes,
            _ => SourceFeed::Bound,
        }
    }
}

/// How a row's track inputs reach the graph (issue #928).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceFeed {
    /// Every `TrackStage::Input` is bound to a `FrozenGraphSource` processor: one dispatched unit
    /// per track per block, which copies the track's frozen block into its arena buffer.
    Bound,
    /// The inputs are claimed by a prepared source set whose driver copies each claim's block on
    /// request and lends its played planes in place: the production feed, over the same frozen
    /// blocks.
    PlayedPlanes,
}

impl SourceFeed {
    /// The record-side name of this feed.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::PlayedPlanes => "played_planes",
        }
    }
}

/// The standing session workloads, in the order their records are emitted.
///
/// **Append-only.** The wasm console guest is addressed by *index* into this array
/// (`miso_console_prepare(index)`), so reordering it silently re-labels every wasm record. New
/// rows go on the end.
///
/// One row has left it, deliberately and once: `sixty_four_track_plumbing_only`, index 11, a plan
/// compiled without builtins, which no host builds (issue #956). Every row after it moved down one
/// index, and the wasm arm's validator and its mutation suite were re-indexed in the same change.
pub const WORKLOADS: [Workload; 15] = [
    Workload::NineTrackBaseline,
    Workload::NineTrackRaggedStrip,
    Workload::SixtyFourTrackConsole,
    Workload::OneTwentyEightTrackStretch,
    Workload::SixtyFourTrackEqOnly,
    Workload::SixtyFourTrackCompressorOnly,
    Workload::SixtyFourTrackBuiltinsOnly,
    Workload::SixtyFourTrackDispatchOnly,
    Workload::SixtyFourTrackIdle,
    Workload::SixtyFourTrackConsoleLegacy,
    Workload::SixtyFourTrackEqCompSimd1,
    Workload::SixtyFourTrackGainPanOnly,
    Workload::SixtyFourTrackConsoleMono,
    Workload::SixtyFourTrackConsoleMonoDual,
    Workload::SixtyFourTrackConsoleHalfMono,
];

/// The session rows the native bench emits after [`WORKLOADS`]: the rows whose track inputs are
/// claimed by a prepared source set rather than bound to a processor (issue #928).
///
/// Kept out of [`WORKLOADS`] on purpose. That array is the wasm console arm's address space --
/// the guest prepares a row by its index (`miso_console_prepare(index)`), the wasm host iterates
/// it, and the wasm arm's validator pins its fifteen kinds -- so a row appended there would change
/// what that arm measures and break its next capture. This row exists to measure the native
/// source feed, and carrying it into the wasm arm is that arm's own change.
///
/// The row is banked, so the shape tests under `tests/` that state a law of every banked row --
/// one folded route per track, and the folded master's bits -- iterate it beside [`WORKLOADS`].
/// What is its own -- its bits against its bound twin's, its bank shape, its source-plane counts
/// and its dispatched units -- is asserted beside that twin by this crate's own test of the pair.
pub const DRIVER_FED_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackGainPanRing];

/// The session rows the native bench emits after [`DRIVER_FED_WORKLOADS`]: the rows prepared with
/// the default web boot's meter set (issue #881).
///
/// Kept out of [`WORKLOADS`] for the reason [`DRIVER_FED_WORKLOADS`] is: that array is the wasm
/// console arm's address space, and a row appended there would change what that arm measures.
/// Carrying the metered row into the wasm arm is that arm's own change.
///
/// The census and shape tests under `tests/` iterate [`WORKLOADS`] only. What this row shares with
/// the standing console row -- its bits, its bank memberships and its folds -- and where it
/// differs -- its fused fader and matrix -- are asserted beside it by this crate's own test of the
/// pair.
pub const METERED_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackConsoleMetered];

/// Every session row the native bench emits, in emission order: [`WORKLOADS`], then
/// [`DRIVER_FED_WORKLOADS`], then [`METERED_WORKLOADS`].
pub fn native_session_rows() -> impl Iterator<Item = Workload> {
    WORKLOADS
        .into_iter()
        .chain(DRIVER_FED_WORKLOADS)
        .chain(METERED_WORKLOADS)
}

/// What a decomposition row does to the fixture's channel strip before it is compiled.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Strip {
    /// The fixture is compiled exactly as written (after any track-count synthesis).
    AsWritten,
    /// The `simd1` chain keeps only its EQ slot. One one-slot chain.
    ///
    /// On the standing fixture this drops the compressor from the two-slot chain; on the retired
    /// fixture it dropped the compressor's separate `dynamic` chain. Either way the surviving
    /// arithmetic is one EQ per track on `simd1`, which is why the `sixty_four_track_eq_only`
    /// digest is expected to be unchanged across the fixture handover.
    EqOnly,
    /// The `simd1` chain keeps only its compressor slot. One one-slot chain.
    CompressorOnly,
    /// The limiter is removed from `simd2`; the `simd1` chain is left as written.
    ///
    /// The chain-shape row. Everything that computes anything is unchanged from the standing
    /// fixture except that the limiter is gone, so this row against
    /// `SixtyFourTrackConsoleLegacy` is a comparison of chain *shape* over identical arithmetic.
    LimiterRemoved,
    /// Every rack is emptied; the builtins, fader and matrix are left as written.
    BuiltinsOnly,
    /// Every rack is emptied and every builtin, fader and matrix is set to its identity.
    Identity,
    /// Every rack is emptied and every *input* builtin is set to its identity; the fader and the
    /// pan matrix keep the values the fixture declared.
    ///
    /// One field apart from [`Self::Identity`], deliberately: the two rows exist to be subtracted
    /// from each other, and a second transcription of the neutralisation would be a second thing
    /// that could drift.
    GainPan,
    /// The mono fixture with the odd tracks' stereo source mapping put back.
    ///
    /// The one edit in this enum that *widens* a row rather than narrowing it, and it is called
    /// out here because the rule above ("every edit is a removal or a neutralisation") is the rule
    /// that makes the decomposition rows subtractable. This row is not a decomposition row: it is
    /// not a subset of any other row's work and nothing subtracts it. It restores, on half the
    /// tracks, the exact field `scripts/derive-mono-console-fixture.py` changed -- so its odd
    /// tracks carry the standing fixture's source mapping and its even tracks the mono fixture's,
    /// and no third session exists anywhere.
    HalfMono,
}

/// Retains only the slots of `rack` whose native effect id is `effect_id`.
///
/// Used instead of clearing a whole rack because the standing fixture's `simd1` is a *two-slot*
/// chain: a decomposition row that wants the EQ alone has to drop one slot out of a chain rather
/// than empty a rack. Matching on the contract's effect id rather than the session's local slot
/// id means a fixture that renamed a slot cannot silently turn a decomposition row into a row
/// that measures nothing.
fn retain_effect(rack: &mut session::Rack, effect_id: &str) {
    rack.effects.retain(|effect| {
        matches!(
            &effect.identity,
            session::EffectIdentity::Native { effect_id: id }
                if id.as_str() == effect_id
        )
    });
}

impl Workload {
    /// The record-side name of this workload.
    pub const fn kind(self) -> &'static str {
        match self {
            Self::NineTrackBaseline => "nine_track_baseline",
            Self::NineTrackRaggedStrip => "nine_track_ragged_strip",
            Self::SixtyFourTrackConsole => "sixty_four_track_console",
            Self::OneTwentyEightTrackStretch => "one_twenty_eight_track_stretch",
            Self::SixtyFourTrackEqOnly => "sixty_four_track_eq_only",
            Self::SixtyFourTrackCompressorOnly => "sixty_four_track_compressor_only",
            Self::SixtyFourTrackBuiltinsOnly => "sixty_four_track_builtins_only",
            Self::SixtyFourTrackDispatchOnly => "sixty_four_track_dispatch_only",
            Self::SixtyFourTrackIdle => "sixty_four_track_idle",
            Self::SixtyFourTrackConsoleLegacy => "sixty_four_track_console_legacy",
            Self::SixtyFourTrackEqCompSimd1 => "sixty_four_track_eq_comp_simd1",
            Self::SixtyFourTrackGainPanOnly => "sixty_four_track_gain_pan_only",
            Self::SixtyFourTrackGainPanRing => "sixty_four_track_gain_pan_ring",
            Self::SixtyFourTrackConsoleMono => "sixty_four_track_console_mono",
            Self::SixtyFourTrackConsoleMonoDual => "sixty_four_track_console_mono_dual",
            Self::SixtyFourTrackConsoleHalfMono => "sixty_four_track_console_half_mono",
            Self::SixtyFourTrackConsoleMetered => "sixty_four_track_console_metered",
        }
    }
    /// How many console tracks this workload renders.
    pub const fn tracks(self) -> u32 {
        match self {
            Self::NineTrackBaseline | Self::NineTrackRaggedStrip => 9,
            Self::OneTwentyEightTrackStretch => 128,
            _ => 64,
        }
    }
    /// The checked-in fixture this workload's model is derived from.
    pub const fn fixture_id(self) -> &'static str {
        match self {
            Self::NineTrackBaseline => "fixtures/session/v1/parametric-eq-nine-track.json",
            Self::SixtyFourTrackConsoleLegacy => {
                "fixtures/session/v1/console-sixty-four-track.json"
            }
            Self::SixtyFourTrackConsoleMono
            | Self::SixtyFourTrackConsoleMonoDual
            | Self::SixtyFourTrackConsoleHalfMono => {
                "fixtures/session/v1/console-sixty-four-track-mono.json"
            }
            _ => "fixtures/session/v1/console-sixty-four-track-intended.json",
        }
    }
    /// `true` when the rendered model was derived in code from the named fixture.
    ///
    /// Two derivations qualify and both must say so: cloning the strips to a different track
    /// count, and emptying or neutralising part of the strip for a decomposition row. A derived
    /// model reported as a checked-in fixture would be exactly the "measuring a fiction" failure
    /// the bench discipline exists to catch, so the flag is pinned per kind in the validator.
    pub const fn synthetic(self) -> bool {
        !matches!(
            self,
            Self::NineTrackBaseline
                | Self::SixtyFourTrackConsole
                | Self::SixtyFourTrackConsoleLegacy
                // Both mono arms render the mono fixture exactly as it is checked in. They are two
                // rows of one session, not two sessions -- which is the property the row-pair's
                // digest equality will rest on once the collapse exists.
                | Self::SixtyFourTrackConsoleMono
                | Self::SixtyFourTrackConsoleMonoDual
                // The metered row renders the standing fixture as written. Its meters are a
                // preparation facility, not an edit to the session.
                | Self::SixtyFourTrackConsoleMetered
        )
    }
    /// The edit this row makes to the fixture's channel strip.
    const fn strip(self) -> Strip {
        match self {
            Self::SixtyFourTrackEqOnly => Strip::EqOnly,
            Self::SixtyFourTrackCompressorOnly => Strip::CompressorOnly,
            Self::SixtyFourTrackBuiltinsOnly => Strip::BuiltinsOnly,
            Self::SixtyFourTrackDispatchOnly => Strip::Identity,
            Self::SixtyFourTrackEqCompSimd1 => Strip::LimiterRemoved,
            // The driver-fed twin takes the same strip edit and the same compile with builtins;
            // only how `build_full` binds its track inputs differs (`Workload::source_feed`).
            Self::SixtyFourTrackGainPanOnly | Self::SixtyFourTrackGainPanRing => Strip::GainPan,
            Self::SixtyFourTrackConsoleHalfMono => Strip::HalfMono,
            _ => Strip::AsWritten,
        }
    }
    /// What every track of this row actually carries, named in the record.
    ///
    /// Derived from the fixture for the `AsWritten` rows -- the nine-track fixture's dynamic rack
    /// is empty as written, which is why it reads `eq` rather than `eq+compressor`.
    pub const fn strip_content(self) -> &'static str {
        match self {
            Self::NineTrackBaseline | Self::SixtyFourTrackEqOnly => "eq",
            Self::SixtyFourTrackCompressorOnly => "compressor",
            Self::SixtyFourTrackBuiltinsOnly => "builtins",
            Self::SixtyFourTrackDispatchOnly => "identity",
            Self::SixtyFourTrackConsoleLegacy | Self::SixtyFourTrackEqCompSimd1 => "eq+compressor",
            // The feed is not strip content; the record names it separately, as `source_feed`.
            Self::SixtyFourTrackGainPanOnly | Self::SixtyFourTrackGainPanRing => "gain+pan",
            _ => "eq+compressor+limiter",
        }
    }

    /// Where this row's effects sit in the track strip, named in the record.
    ///
    /// `strip_content` says *what* every track carries; this says *where*. The two were one field
    /// until #175, which is the issue that made the distinction load-bearing: the chain-shape
    /// row-pair is two rows with identical `strip_content` (`eq+compressor`), identical
    /// coefficients and identical order whose whole difference is that one is a two-slot chain on
    /// `simd1` and the other is a `simd1` chain plus a `dynamic` chain. Without this field those
    /// two rows are indistinguishable in a record, and the number that separates them -- one
    /// AoSoA round-trip per bank per block -- would be attributed to nothing.
    ///
    /// The vocabulary is `rack:slot[+slot]`, racks in strip order, joined by `,`. `builtins` is
    /// the row that carries no rack effect at all.
    pub const fn strip_layout(self) -> &'static str {
        match self {
            Self::NineTrackBaseline | Self::SixtyFourTrackEqOnly => "simd1:eq",
            Self::SixtyFourTrackCompressorOnly => "simd1:compressor",
            Self::SixtyFourTrackBuiltinsOnly
            | Self::SixtyFourTrackDispatchOnly
            | Self::SixtyFourTrackGainPanOnly
            | Self::SixtyFourTrackGainPanRing => "builtins",
            // The retired layout: two one-slot chains, one per rack.
            Self::SixtyFourTrackConsoleLegacy => "simd1:eq,dynamic:compressor",
            // The chain-shape row: one two-slot chain, no limiter.
            Self::SixtyFourTrackEqCompSimd1 => "simd1:eq+compressor",
            // The intended production layout.
            _ => "simd1:eq+compressor,simd2:limiter",
        }
    }
    /// What every track's source binding writes into the graph.
    pub const fn input_signal(self) -> &'static str {
        match self {
            Self::SixtyFourTrackIdle => "silence",
            _ => "tone",
        }
    }
    /// Untimed blocks rendered before the clock starts.
    ///
    /// The idle row needs a real settling period rather than a token one: every SVF, every
    /// smoother and every compressor detector has to reach its silent steady state, or the row
    /// would report the decay rather than the floor.
    pub const fn warmup_blocks(self) -> usize {
        match self {
            Self::SixtyFourTrackIdle => 512,
            _ => 0,
        }
    }
}

/// Applies a decomposition row's edit to a parsed session model.
///
/// Every edit is a *removal or a neutralisation*, never an addition: a row can only ever measure a
/// subset of what `sixty_four_track_console` measures, which is what makes the differences between
/// the rows subtractions rather than comparisons of two different sessions.
fn apply_strip(model: &mut SessionModel, strip: Strip) {
    if strip == Strip::AsWritten {
        return;
    }
    for (index, track) in model.tracks.iter_mut().enumerate() {
        match strip {
            Strip::AsWritten => unreachable!("returned above"),
            Strip::EqOnly => retain_effect(&mut track.simd1, "miso.parametric-eq"),
            Strip::CompressorOnly => retain_effect(&mut track.simd1, "miso.compressor"),
            // `simd2` is cleared for every derived row below, so this arm's whole edit is that
            // clearing: the `simd1` chain is deliberately left exactly as the fixture wrote it.
            Strip::LimiterRemoved => {}
            // The racks go and nothing else does.
            Strip::BuiltinsOnly => {
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
            }
            Strip::Identity | Strip::GainPan => {
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                for channel in [&mut track.builtins.left, &mut track.builtins.right] {
                    channel.polarity_invert = false;
                    channel.trim_db = 0.0;
                    // Zero is how a builtin filter is disabled: `InputBuiltins::prepare` designs
                    // the section from the declared frequency and treats zero as "not enabled".
                    channel.hpf_hz = 0.0;
                    channel.lpf_hz = 0.0;
                }
                // The one field that separates the two rows. `GainPan` keeps the fixture's
                // declared fader trims and pan positions; `Identity` asks the fader for 0 dB and
                // the pan for `left = right = 1.0`. That pan is not the identity matrix: it routes
                // both inputs hard right (`ll = lr = 6.1e-17`, `rl = rr = 1`), so this row's
                // matrix banks have no identity lane and run the same select-free arm as
                // `GainPan`'s (issue #944). The doc is corrected; the content, and so the digest,
                // is kept (ruling 4).
                if strip == Strip::Identity {
                    track.fader = DualMonoFader {
                        left_db: 0.0,
                        right_db: 0.0,
                        left_mute: false,
                        right_mute: false,
                    };
                    track.matrix_or_pan = MatrixOrPan::Pan {
                        left: 1.0,
                        right: 1.0,
                        smoothing_samples: 0,
                    };
                }
            }
            // Half the tracks get the standing fixture's stereo source mapping back. The racks
            // are untouched: this row renders the whole strip, and the only thing that varies
            // across its lanes is whether a track's two channels read one source channel or two.
            Strip::HalfMono => {
                if index % 2 == 1 {
                    track.right_source_channel = 1;
                }
                continue;
            }
        }
        track.simd2.effects.clear();
    }
}
/// Which console-side facilities a prepared arm carries.
///
/// The session rows all use [`PlanConfig::BASELINE`], which is what the console benchmark has
/// always measured: no meter streams, no live-console control channel, no observation capacity.
/// The #163 item 0d arms differ from it in exactly one field each, so the paired delta between two
/// arms is the cost of that one facility.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlanConfig {
    /// One meter stream per track at the post-matrix tap, as a production console prepares.
    pub meters: bool,
    /// One bounded live-console control channel per prepared effect.
    pub control: bool,
    /// Effect observation capacity, and whether its taps are armed.
    pub observation: ObservationArm,
}

/// The three points of the issue #143 two-level zero, as benchmark arms.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObservationArm {
    /// Level 1: no lane exists. `attach_effect_observation` is never called.
    Absent,
    /// Level 2: the lane exists and no tap is armed. One predicted branch per effect per block.
    Unarmed,
    /// Every declared tap of every observed effect is armed.
    Armed,
}

impl ObservationArm {
    /// The record-side name of this arm.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Unarmed => "unarmed",
            Self::Armed => "armed",
        }
    }
}

impl PlanConfig {
    /// What every `console_session` row measures, and what the console bench has always measured.
    pub const BASELINE: Self = Self {
        meters: false,
        control: false,
        observation: ObservationArm::Absent,
    };
}

/// The parsed, edited session model a workload renders.
///
/// Split out of `SessionRuntime` so the meter and observation arms build the *same* model the
/// `sixty_four_track_console` row builds, through the same code, rather than a second transcription
/// of it.
fn console_model(workload: Workload) -> SessionModel {
    let text = match workload {
        Workload::NineTrackBaseline => NINE_TRACK,
        Workload::SixtyFourTrackConsoleLegacy => SIXTY_FOUR_TRACK_LEGACY,
        Workload::SixtyFourTrackConsoleMono
        | Workload::SixtyFourTrackConsoleMonoDual
        | Workload::SixtyFourTrackConsoleHalfMono => SIXTY_FOUR_TRACK_MONO,
        _ => SIXTY_FOUR_TRACK,
    };
    let mut model = parse_session_json(text).expect("frozen console session fixture");
    model.automation.clear();
    if model.tracks.len() != workload.tracks() as usize {
        synthesise_tracks(&mut model, workload.tracks() as usize);
    }
    apply_strip(&mut model, workload.strip());
    assert_eq!(
        model.tracks.len(),
        workload.tracks() as usize,
        "{}: the fixture must carry exactly the declared track count",
        workload.kind()
    );
    model
}

/// One meter stream per track at the post-matrix tap, in canonical track order.
///
/// This is the shape `host-core` prepares for a real console session: handles are
/// `index + 1` so they are nonzero and stable, the tap is the one a console meters by default, and
/// the window is [`WINDOW_BLOCKS`] blocks. Nothing here is a benchmark convenience -- an arm that
/// metered a shape no host prepares would report a cost nobody pays.
fn meter_requests(model: &SessionModel) -> Vec<MeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(WINDOW_BLOCKS * QUANTUM as u32).expect("nonzero period"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(METER_QUEUE_DEPTH).expect("nonzero depth"),
        reset_generation: 0,
    };
    model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(index as u64 + 1).expect("nonzero handle")),
            track_id: track.id.as_str().to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        })
        .collect()
}

/// The meter set the default web boot binds, one per track (issue #881).
///
/// The request `host_core` builds when a browser boots with its console defaults: one
/// [`MeterMetricSet::SAMPLE_PEAK`] meter at [`MeterTap::PostMatrix`] per track of the compiled
/// session's normalized model, handles `index + 1` in that order, a window of
/// [`WEB_METER_BLOCKS`] blocks, no peak hold, peak decay off, a [`WEB_METER_QUEUE_DEPTH`]-deep
/// queue and reset generation zero. Transcribed field for field from
/// `prepare_host_runtime_with_console_policy_and_spectrum` and the web host's
/// `console_request`, because this subject does not link either host.
fn web_meter_requests(session: &session::CompiledSession) -> Vec<SelectedMeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(WEB_METER_BLOCKS * QUANTUM as u32).expect("nonzero period"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(WEB_METER_QUEUE_DEPTH).expect("nonzero depth"),
        reset_generation: 0,
    };
    session
        .normalized_model()
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| SelectedMeterRequest {
            request: MeterRequest {
                handle: MeterHandle(NonZeroU64::new(index as u64 + 1).expect("nonzero handle")),
                track_id: track.id.as_str().to_owned(),
                tap: MeterTap::PostMatrix,
                config,
            },
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect()
}

/// A block the plan refused, or an output buffer it could not be given.
///
/// Deliberately opaque, and deliberately not a `Result<_, ()>`: a driver counts these into a
/// record's `render_errors` rather than branching on why one happened, and the plan's own error
/// taxonomy is not something a benchmark record reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderFailed;

/// The session's output: the left plane then the right plane, `QUANTUM` words each, on a 64-byte
/// boundary (issue #935).
///
/// It was a `Vec<f32>`, which lands on whatever 16-byte boundary the allocator returns. At 16 or
/// 48 mod 64 half of the 32-byte loads and stores of the output split a cache line, so the same
/// binary measured a different amount of work depending on where the heap put this buffer. A
/// `QUANTUM`-word plane is a whole number of cache lines (asserted below), so aligning the buffer
/// aligns both planes. Every reader still sees the `&[f32]` / `&mut [f32]` the `Vec` lent it.
#[repr(C, align(64))]
struct OutputPlanes([f32; QUANTUM * 2]);

// `QUANTUM * 4` bytes is a multiple of 64, so a plane that starts a 64-byte line ends one: the
// right plane of `OutputPlanes` and of `FrozenGraphSource` is aligned because the left one is.
const _: () = assert!((QUANTUM * core::mem::size_of::<f32>()).is_multiple_of(64));

/// One prepared console arm: a real [`PreparedRenderPlan`] and the buffer it renders into.
///
/// Built by [`SessionRuntime::build`] from a [`Workload`] and a [`PlanConfig`], through the
/// production compile and attach entry points in the order a host calls them. The only thing a
/// driver may do inside a clock is [`SessionRuntime::render`]; every other method on this type is
/// evidence collection and belongs outside it.
pub struct SessionRuntime {
    plan: PreparedRenderPlan,
    /// Boxed so the render loop reads its planes through one stable, 64-byte-aligned pointer.
    output: Box<OutputPlanes>,
    /// Control-side halves, held for the arms that attached them. Never touched inside the clock.
    meter_consumers: Vec<MeterConsumer>,
    controls: Vec<EffectControlProducer>,
    observations: Vec<EffectObservationHandle>,
    /// Every track's *structural* channel-symmetry witness, in normalized track order, taken at
    /// compile time.
    ///
    /// The `SOURCE` term is a function over the compiled session rather than a field of the
    /// prepared plan (`session_structural_symmetry` says why: the cohort planner needs the
    /// class before any prepared object exists), so it cannot be read back off the plan the way
    /// [`SessionRuntime::symmetry_counters`] reads the rest of the witness. It is taken once, in
    /// `build_full`, and kept.
    ///
    /// Kept whole rather than as a count because the *join* is what a caller needs: this half is
    /// keyed by track id and the runtime half ([`SessionRuntime::unit_eligibility`]) by anonymous
    /// lanes, and a collapse decision is their conjunction. `PlanUnitEligibility::lane_tracks`
    /// is the relation between the two keys.
    structural_symmetry: Vec<(Box<str>, ChannelSymmetryWitness)>,
}

impl SessionRuntime {
    /// The arm every `console_session` row renders: [`PlanConfig::BASELINE`] at the backend this
    /// build detected.
    pub fn new(workload: Workload) -> Self {
        Self::build(workload, PlanConfig::BASELINE)
    }

    /// Compiles, prepares and binds one console arm.
    ///
    /// # Panics
    ///
    /// Panics if any stage of the production pipeline refuses the frozen fixture. Every such
    /// refusal is a defect in the subject rather than a condition a measurement may report
    /// around, so this fails loudly instead of recording a number for a plan it did not build.
    pub fn build(workload: Workload, config: PlanConfig) -> Self {
        Self::build_with_dispatch(workload, config, Backend::current())
    }

    /// Compiles, prepares and binds one console arm at an explicitly chosen lane width.
    ///
    /// [`SessionRuntime::build`] dispatches at [`Backend::current()`], which is what the engine
    /// does in production and what every native console record was taken at. This entry point
    /// exists for one job: the #163 phase 2 wasm console arm compares a `wasm32` target against a
    /// native one, and those two targets do not offer the same lane width. `simd128` is four
    /// lanes; the native backend this host records on is eight. A ratio taken across that pair
    /// confounds *which target executed the code* with *how wide its vectors were*.
    ///
    /// Driving the native leg at `Simd4` as well as at `Backend::current()` separates the two:
    /// wasm-at-four against native-at-four is one target difference at one width, and native-at-
    /// eight stays in the table as the backend the product actually records on. This is the
    /// discipline phase 0b's kernel arm already used, applied to the console subject.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build`].
    pub fn build_with_dispatch(workload: Workload, config: PlanConfig, dispatch: Backend) -> Self {
        Self::build_full(workload, config, dispatch, SourceSignal::Local)
    }

    /// Compiles, prepares and binds one console arm, choosing both the lane width and where its
    /// input samples come from.
    ///
    /// The source choice exists for exactly one reason, recorded on [`source_block`]: the tone is
    /// a libm sine, and libm differs between the native and wasm targets. A driver comparing two
    /// targets injects one target's samples into both, so a digest difference between the legs is
    /// a difference in how the *engine* computed and never a difference in what it was asked to
    /// compute.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build`], and additionally if an injected table does not cover every
    /// track the workload binds.
    pub fn build_full(
        workload: Workload,
        config: PlanConfig,
        dispatch: Backend,
        source: SourceSignal,
    ) -> Self {
        let model = console_model(workload);
        let session = compile_session(&model, compile_caps()).expect("compiled console session");
        let meters = if config.meters {
            meter_requests(&model)
        } else {
            Vec::new()
        };
        // Issue #881: the metered row carries the default web boot's meter set and delivery and no
        // other console facility, so a facility arm on top of it would meter every track twice and
        // measure a shape no host prepares.
        assert!(
            !workload.web_meters() || config == PlanConfig::BASELINE,
            "{}: the metered row carries its own meters and no other console facility",
            workload.kind()
        );
        let web_meters = if workload.web_meters() {
            web_meter_requests(&session)
        } else {
            Vec::new()
        };
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let mut effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .expect("prepared console effects");
        // Both attaches are the production entry points, called in the order a host calls them.
        // The control channel is attached for every observation arm including `Absent`, so the
        // paired delta between the arms is the observation lane and not the control queue drain.
        let controls = if config.control {
            attach_effect_console(
                &mut effects,
                NonZeroUsize::new(CONTROL_QUEUE_DEPTH).expect("nonzero depth"),
            )
            .expect("live-console control channels")
        } else {
            Vec::new()
        };
        let observations = if config.observation == ObservationArm::Absent {
            Vec::new()
        } else {
            attach_effect_observation(&mut effects, MAXIMUM_OBSERVATION_TAPS, WINDOW_BLOCKS)
                .expect("effect observation capacity")
        };
        let silent = workload.input_signal() == "silence";
        let mappings = channel_mappings(&model);
        // Issue #881: the metered row is prepared through the builtins entry the default web boot's
        // host preparation reaches, which binds its selected meters as permanent observers. It
        // attaches no control channel, but its between-render-calls delivery is not inert: under
        // it each cohort's fader and matrix banks fuse into one stage, as they do in the browser,
        // where the `Concurrent` rows keep two (`Workload::SixtyFourTrackConsoleMetered` says what
        // that means for the pair). Every other row keeps the entry it has always been prepared
        // through.
        let builtins = if workload.web_meters() {
            builtins_compiler::prepare_selected_session_builtins_between_render_calls(
                &session,
                &web_meters,
                &[],
                builtin_caps(),
            )
        } else {
            builtins_compiler::prepare_session_builtins(&session, &meters, builtin_caps())
        }
        .expect("prepared console builtins");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch,
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
        })
        .unwrap_or_else(|_| panic!("{}: production console graph", workload.kind()));

        let envelope = artifact.envelope();
        // `observers` stays empty on purpose in both feeds: it is the *external* observer slot. A
        // meter observer is compiler-owned and is appended to this vector by the sealed builtins
        // artifact inside `into_bound` (or `into_bound_with_source_set`), which is why
        // `meters: true` is expressed as a meter *request* and not as a hand-built observer.
        // Driving the real path is the whole point of the arm.
        let bound = match workload.source_feed() {
            SourceFeed::Bound => {
                let nodes = artifact
                    .external_binding_nodes()
                    .map(|node| source_binding(node, silent, &source, &mappings))
                    .collect();
                artifact
                    .into_bound(GraphRuntimeBindings {
                        envelope,
                        nodes,
                        observers: Vec::new(),
                    })
                    .unwrap_or_else(|_| panic!("{}: console graph bindings", workload.kind()))
            }
            // Issue #928, re-based by #956: the same artifact, its track inputs claimed by a
            // prepared source set instead of bound to processors -- the shape `host-core` binds a
            // session in (`crates/host-core/src/prepare.rs`, `into_bound_with_source_set`). Every
            // external node that is not a track input is acknowledged by the same `source_binding`
            // the bound feed uses, so the feed is the only thing the two rows bind differently.
            SourceFeed::PlayedPlanes => {
                let mut claims: Vec<GraphSourceInputClaim> = artifact
                    .external_binding_nodes()
                    .filter(|node| is_track_input(node))
                    .map(|node| GraphSourceInputClaim { node: node.clone() })
                    .collect();
                // The set requires its claims strictly ascending, and the driver serves claim `i`
                // as the `i`-th of them.
                claims.sort_unstable();
                let driver = FrozenSourceDriver::new(&claims, silent, &source, &mappings);
                let resources = driver.resource_report();
                let source_set =
                    GraphPreparedSourceSet::new(envelope, claims, resources, Box::new(driver));
                let nodes = artifact
                    .external_binding_nodes()
                    .filter(|node| !is_track_input(node))
                    .map(|node| source_binding(node, silent, &source, &mappings))
                    .collect();
                artifact
                    .into_bound_with_source_set(
                        GraphRuntimeBindings {
                            envelope,
                            nodes,
                            observers: Vec::new(),
                        },
                        source_set,
                    )
                    .unwrap_or_else(|failure| {
                        panic!(
                            "{}: driver-fed console graph bindings: {}",
                            workload.kind(),
                            failure.code
                        )
                    })
            }
        };
        let (plan, meter_consumers) = (bound.plan, bound.meter_consumers);
        assert_eq!(
            meter_consumers.len(),
            meters.len() + web_meters.len(),
            "{}: every requested meter stream must reach the plan",
            workload.kind()
        );

        let mut plan = plan;
        // The collapse's structural join, performed exactly where the M1 plumbing said it would
        // be: `session_structural_symmetry` is keyed by track id, the plan's rows are keyed by
        // anonymous lanes, and this is the one call site that holds both. A plan nobody joins
        // never collapses, so this is not an optimisation switch -- it is the arming.
        let structural = builtins_compiler::session_structural_symmetry(&session);
        let eligible: BTreeSet<&str> = structural
            .iter()
            .filter(|(_, witness)| witness.eligible())
            .map(|(track, _)| track.as_ref())
            .collect();
        plan.arm_mono_collapse(&|track: &str| eligible.contains(track));
        // The mono measurement's second arm. Both arms compile the *same* fixture -- that is what
        // makes the paired delta the collapse and nothing else -- so the difference between them
        // has to be this switch and cannot be a fixture edit.
        plan.force_mono_collapse_off(workload.collapse_forced_off());
        let mut runtime = Self {
            plan,
            output: Box::new(OutputPlanes([0.0; QUANTUM * 2])),
            meter_consumers,
            controls,
            observations,
            structural_symmetry: builtins_compiler::session_structural_symmetry(&session),
        };
        if config.observation == ObservationArm::Armed {
            runtime.arm_observation();
        }
        runtime
    }

    /// Arms every declared tap of every prepared effect, the way a subscribing console does.
    ///
    /// Off the clock, and through the same bounded control queue a host pushes: the records are
    /// drained by the render thread at the top of the next block, so the arm must render at least
    /// one untimed block after this before it is actually armed. Its warmup does.
    fn arm_observation(&mut self) {
        for producer in &mut self.controls {
            for tap_index in 0..producer.descriptor.observations.len() as u32 {
                producer
                    .try_push(EffectControlRecord::Observe {
                        tap_index,
                        armed: true,
                        window_blocks: WINDOW_BLOCKS,
                    })
                    .expect("room in the bounded control queue");
            }
        }
    }

    /// The live-console control channel of the alphabetically first track carrying `effect_id`.
    ///
    /// "One track" has to be chosen by a stable key rather than by taking the first matching
    /// channel: [`attach_effect_console`] returns channels in prepared-entry order, which is
    /// sorted by effect id and not by track, so a positional choice would silently address a
    /// different track when the entry set changes. The track id is the session-stable identity, so
    /// picking its minimum is deterministic across every build of every fixture.
    ///
    /// Returns `None` when the arm attached no control channels (a `control: false` plan) or when
    /// no prepared effect carries that id.
    #[must_use]
    pub fn first_track_control_channel(&self, effect_id: &str) -> Option<usize> {
        self.controls
            .iter()
            .enumerate()
            .filter(|(_, producer)| producer.descriptor.id.as_str() == effect_id)
            .min_by(|(_, left), (_, right)| left.track_id.cmp(&right.track_id))
            .map(|(index, _)| index)
    }

    /// The live-console control channel of `track_id`'s `effect_id` (a contract id), by stable
    /// identity rather than by position (issue #1003).
    ///
    /// `attach_effect_console` returns channels in prepared-entry order, which moves when the
    /// entry set does, so a row that automates named tracks resolves each one by its session id.
    /// Returns `None` for a `control: false` plan or when the track carries no such effect.
    #[must_use]
    pub fn control_channel(&self, track_id: &str, effect_id: &str) -> Option<usize> {
        self.controls.iter().position(|producer| {
            &*producer.track_id == track_id && producer.descriptor.id.as_str() == effect_id
        })
    }

    /// Session-stable `(track_id, effect_id)` identity of one prepared control channel.
    ///
    /// So a record can *name* the track and slot it automated instead of asserting one in prose.
    #[must_use]
    pub fn control_identity(&self, channel: usize) -> (&str, &str) {
        let producer = &self.controls[channel];
        (&producer.track_id, &producer.effect_id)
    }

    /// Pushes one live-console bypass record into one prepared effect's bounded queue.
    ///
    /// The production control path, like [`SessionRuntime::push_parameter`]: the record is drained
    /// by the render thread at the top of the next block, and it moves the channel-symmetry
    /// witness' `UNBYPASSED` term at the same boundary it takes effect on.
    ///
    /// It is here because bypass is the transition the mono collapse is hardest on and the one no
    /// other control record reaches. A bypassed lane's dry signal is delayed through the slot's own
    /// latency **line**, which persists across blocks, so engaging a bypass after a collapsed run
    /// reads samples the collapsed blocks put into that line. Nothing else in this harness can set
    /// that sequence up.
    ///
    /// Returns `false` when the bounded queue was full, which a caller counts rather than ignores.
    /// Off the clock.
    pub fn push_bypass(&mut self, channel: usize, bypassed: bool) -> bool {
        self.controls[channel]
            .try_push(EffectControlRecord::Bypass(bypassed))
            .is_ok()
    }

    /// Pushes one live-console parameter retarget into one prepared effect's bounded queue.
    ///
    /// This is the production control path and nothing else: the record is drained by the render
    /// thread at the top of the next block and staged as a single
    /// [`AutomationSpanKind::Point`](effect_contract::AutomationSpanKind) span at that
    /// block's first sample. One call per block therefore *is* "one Point span per block", by the
    /// contract's own construction rather than by a hand-built span a benchmark asserts is
    /// equivalent.
    ///
    /// Off the clock, like the observation arming and every other control-side method on this
    /// type. Returns `false` when the bounded queue was full, which a driver counts
    /// rather than ignores -- a silently refused push would report the cost of automation that
    /// never happened.
    pub fn push_parameter(
        &mut self,
        channel: usize,
        parameter_index: u32,
        parameter_channel: ParameterChannel,
        value: f32,
    ) -> bool {
        if self.controls[channel].has_owner() {
            let result = (|| {
                let base_revision = self.controls[channel]
                    .owner()
                    .ok_or(())?
                    .committed_revision();
                self.controls[channel]
                    .begin_owner(base_revision)
                    .map_err(|_| ())?;
                self.controls[channel]
                    .edit_owner(parameter_index, parameter_channel, value)
                    .map_err(|_| ())?;
                let mut targets = [PreparedEffectTarget {
                    slot: 0,
                    channel: ParameterChannel::Left,
                    words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
                }; 12];
                let count = self.controls[channel]
                    .owner()
                    .ok_or(())?
                    .prepare_targets_into(&mut targets)
                    .map_err(|_| ())?;
                self.controls[channel]
                    .preflight_candidate_targets(base_revision, &targets[..count])
                    .map_err(|_| ())?;
                self.controls[channel]
                    .publish_candidate_targets(base_revision, &targets[..count])
                    .map_err(|_| ())?;
                self.controls[channel].commit_owner().map_err(|_| ())?;
                Ok::<(), ()>(())
            })();
            if result.is_err() {
                let _ = self.controls[channel].discard_owner();
                return false;
            }
            return true;
        }
        self.controls[channel]
            .try_push(EffectControlRecord::Parameter {
                parameter_index,
                channel: parameter_channel,
                value,
            })
            .is_ok()
    }

    /// Prepared observation lanes. Zero for an `Absent` arm.
    ///
    /// A method rather than a public field: the arms hold control-side halves that must never be
    /// reachable from a timed region, and an accessor is what keeps that boundary reviewable.
    pub fn observation_lanes(&self) -> usize {
        self.observations.len()
    }

    /// Declared observation taps across every prepared lane. Zero for an `Absent` arm.
    pub fn observation_taps(&self) -> usize {
        self.observations
            .iter()
            .map(|handle| handle.readers.len())
            .sum()
    }

    /// Observation windows this arm has published and not yet acknowledged. Outside the clock.
    pub fn published_windows(&self) -> u64 {
        self.observations
            .iter()
            .flat_map(|handle| handle.readers.iter())
            .filter_map(|reader| reader.read())
            .map(|window| window.sequence)
            .sum()
    }

    /// Every armed tap's most recent published window, in prepared-lane then reader order.
    ///
    /// The *values*, not a count. `published_windows` says a tap published; this says what it
    /// published, which is the only way to state that a collapsed cohort's right-channel taps carry
    /// the reading a dual run would have produced. A collapsed bank evolves one channel's state, so
    /// a right-channel tap read straight off that state would be frozen at the value it held when
    /// the collapse engaged -- and no digest of the rendered audio could see it, because the audio
    /// is correct either way.
    ///
    /// A reader with nothing published yet contributes `None`, so the shape of the result is
    /// itself part of the comparison. Read outside the clock.
    #[must_use]
    pub fn observation_readings(&self) -> Vec<Option<(u64, f32, f32)>> {
        self.observations
            .iter()
            .flat_map(|handle| handle.readers.iter())
            .map(|reader| {
                reader
                    .read()
                    .map(|window| (window.sequence, window.left, window.right))
            })
            .collect()
    }

    /// Completed planar/AoSoA transpose round-trips since this plan was bound.
    ///
    /// The G5 shape gate's counter (master plan §4.5), surfaced so a benchmark can *record* the
    /// chain shape it measured instead of asserting one in prose. Issue #175 is the reason it is
    /// here: the intended production layout was expected to pay fewer round-trips than the retired
    /// one, and a claim like that belongs in the record next to the timing it is supposed to
    /// explain.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_transposes(&self) -> u64 {
        self.plan.bank_transposes()
    }

    /// `[bank chains, bound bank slots]` this arm's plan realises (issue #181's G5 shape, widened
    /// by #202 rec 2).
    ///
    /// Beside [`SessionRuntime::bank_transposes`] because the two answer different questions and
    /// were indistinguishable while every chain carried one slot: the counter says how many
    /// planar/AoSoA round-trips were paid, this says how many chains and how many slots the plan
    /// built. A merge that silently stopped firing would leave every digest and every timing
    /// plausible and only move this pair, so a test that wants to assert a merge *fired* has to
    /// read it.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_shape(&self) -> [u64; 2] {
        self.plan.bank_shape()
    }

    /// Tracks whose structural channel-symmetry witness holds: the `SOURCE` term, per track.
    ///
    /// The other half of the mono evidence, and it has to be reported beside
    /// [`SessionRuntime::symmetry_counters`] rather than folded into it. The plan's census carries
    /// every term the *prepared* objects can speak to -- `DESIGNED`, `LIVE`, `UNBYPASSED`,
    /// `RESTORED` -- and deliberately not `SOURCE`, which lives in the compiled session. So a row
    /// can have a full census and no mono source at all: `sixty_four_track_dispatch_only` does,
    /// because an identity strip's designed words are trivially symmetric while its tracks still
    /// read two different source channels. Reporting only the census would make that row look
    /// collapse-eligible, which it is not.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn structural_mono_tracks(&self) -> u64 {
        self.structural_symmetry
            .iter()
            .filter(|(_, witness)| witness.eligible())
            .count() as u64
    }

    /// Every track's structural channel-symmetry witness, in normalized track order.
    ///
    /// The control-plane half of the collapse decision, keyed by track id. Conjoin it with
    /// [`SessionRuntime::unit_eligibility`] through that surface's `lane_tracks` to get a real
    /// per-cohort answer: the runtime half is deliberately **source agnostic** (`SOURCE` is not
    /// one of its four terms), so a plan whose every designed word is symmetric reports every
    /// lane eligible whatever its tracks' source mappings are. Neither half is the answer alone.
    #[must_use]
    pub fn structural_symmetry(&self) -> &[(Box<str>, ChannelSymmetryWitness)] {
        &self.structural_symmetry
    }

    /// `[collapse-eligible lanes, lanes]` this arm's plan realises: the channel-symmetry census
    /// (mono-collapse M0).
    ///
    /// A lane is eligible when every term of its channel-symmetry witness holds, which is decided
    /// at preparation for the two structural terms and maintained at the drains for the rest.
    /// **Nothing in this tree reads it to decide anything rendered**; it is control-plane evidence,
    /// and it is surfaced here so the mono rows can *record* that their fixture is what it claims
    /// to be rather than assert it in prose. A mono row whose census showed no eligible lane would
    /// be measuring the standing session under a different name.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn symmetry_counters(&self) -> [u64; 2] {
        self.plan.symmetry_counters()
    }

    /// Force every bank chain's mono collapse off, or back on, between blocks.
    ///
    /// The arm switch, exposed mid-session for one job: the transition oracle. A run that collapses
    /// for a while and then stops must render, from the block it stops on, exactly what a run that
    /// never collapsed renders -- which is the whole claim the disengage state copy makes, and the
    /// only way to test it is to take the transition in the middle of a session.
    ///
    /// Outside the clock, like every other control on this type.
    pub fn force_mono_collapse_off(&mut self, forced: bool) {
        self.plan.force_mono_collapse_off(forced);
    }

    /// `[blocks rendered with the mono collapse taken, cohorts that can take it at all]`.
    ///
    /// The evidence that the collapse *fired*, and the only evidence there can be: a collapsed
    /// block renders the bits a dual block renders, so no digest and no output comparison can see
    /// it. The second number is fixed at bind and is what "eight of eight cohorts" is read off; the
    /// first is per block, so a plan that rendered `n` blocks with all `c` cohorts collapsed
    /// reports `n * c`.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_collapse_counters(&self) -> [u64; 2] {
        self.plan.bank_collapse_counters()
    }

    /// `[disengages, re-engages, agreement proofs]` over every cohort (mono-collapse M3).
    ///
    /// The transition evidence the block count cannot carry. A row that collapses on every block
    /// reports `[0, 0, 0]`; a session that stops and starts reports the cycle it took.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_collapse_transitions(&self) -> [u64; 3] {
        self.plan.bank_collapse_transitions()
    }

    /// One collapse-eligibility row per scheduling unit (mono-collapse M1).
    ///
    /// The per-cohort form of [`SessionRuntime::symmetry_counters`]. The census is a pair of
    /// totals and a collapse decides per cohort, so the shape a mixed session realises -- four
    /// all-eligible cohorts and four all-ineligible ones, rather than eight half-and-half ones --
    /// is only visible here. See `engine::realtime::PlanUnitEligibility` for what a
    /// row carries and the two checks a caller owes before reading one as evidence.
    #[must_use]
    pub fn unit_eligibility(&self) -> Vec<PlanUnitEligibility> {
        self.plan.unit_eligibility()
    }

    /// `[collapse-eligible lanes, lanes]` over the **track lanes of this plan's bank chains** only
    /// (issue #911): the part of [`SessionRuntime::symmetry_counters`] a mono row's premise is
    /// about.
    ///
    /// The collapse acts on bank chains and on nothing else (`arm_mono_collapse` skips every
    /// single op), so "every track of the fixture carries a symmetric prepared witness" is a
    /// statement about the lanes that render a track's upstream-of-seam strip on a chain. The
    /// census counts every scheduling unit, and three kinds of census row are excluded here, each
    /// of them because it is not such a lane:
    ///
    /// * **Every single dispatched op.** On the console fixtures these are the tracks' `Input`
    ///   stages and the `main-out` output (plus the route ops of a row whose route fold declines).
    ///   The inputs are bound to this crate's `FrozenGraphSource`, a host-supplied processor the
    ///   engine cannot see through, so they decline. The output is bound through
    ///   `GraphNodeBinding::identity`, so the engine lowers it to its own identity kind, which
    ///   reports `SYMMETRIC` -- truthfully, nothing in an identity can make two channels disagree
    ///   -- while naming no track and rendering nothing upstream of the seam. That one row is why
    ///   the mono fixture's census has read `[65, 129]` rather than `[64, 129]` since issue #221
    ///   (`d1cb3653`) moved this crate's binding of the output off an opaque do-nothing processor,
    ///   with not one track's witness changed.
    /// * **Vacuous bank chains** ([`PlanUnitEligibility::witness_is_vacuous`]): a seam-side-only
    ///   chain's witness is an unconditional `SYMMETRIC`, so its lanes would count a constant.
    /// * **A bank lane naming no track**, which is not a track lane by definition. No console
    ///   fixture builds one; the filter is here so the count cannot silently include one.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_symmetry_counters(&self) -> [u64; 2] {
        self.plan
            .unit_eligibility()
            .iter()
            .filter(|row| row.banked && !row.witness_is_vacuous())
            .flat_map(|row| row.lane_tracks.iter().zip(row.lane_eligible.iter()))
            .filter(|(track, _)| !track.is_empty())
            .fold([0, 0], |mut total, (_, eligible)| {
                total[0] += u64::from(*eligible);
                total[1] += 1;
                total
            })
    }

    /// Bank-chain lanes whose route and master accumulation this plan folded into the chain's own
    /// epilogue (issue #218).
    ///
    /// A count, for the reason every other shape number here is a count: the fold renders the same
    /// bits by construction, so nothing but a count can say whether it fired.
    #[must_use]
    pub fn bank_route_folds(&self) -> u64 {
        self.plan.bank_route_folds()
    }

    /// Bank-chain lanes whose scatter this plan pointed straight at their consumer's buffer (issue
    /// #202 rec 3).
    ///
    /// A count for the reason [`SessionRuntime::bank_route_folds`] is one: the redirect renders the
    /// same bits by construction, so nothing but a count can say whether it fired. Fixed at bind. A
    /// folded lane is not counted: its tile goes to the chain's epilogue, so it has no scatter to
    /// point anywhere.
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bank_scatter_redirects(&self) -> u64 {
        self.plan.bank_scatter_redirects()
    }

    /// Meter frames drained from every stream. Outside the clock, like every evidence step.
    pub fn drain_meters(&mut self) -> u64 {
        self.drain_meter_snapshots(|_| {})
    }

    /// Meter streams bound into this plan: one per track on a metered arm, none on any other.
    #[must_use]
    pub fn meter_streams(&self) -> usize {
        self.meter_consumers.len()
    }

    /// The tap each bound meter stream observes, in stream order.
    pub fn meter_taps(&self) -> impl Iterator<Item = MeterTap> + '_ {
        self.meter_consumers.iter().map(|stream| stream.tap)
    }

    /// Pops every published meter snapshot, stream by stream, hands each to `visit`, and returns
    /// how many there were. Outside the clock, like every evidence step, and allocation-free, so a
    /// driver can consume every snapshot after every block the way a host holding the meter lease
    /// does.
    pub fn drain_meter_snapshots(&mut self, mut visit: impl FnMut(&MeterSnapshot)) -> u64 {
        let mut snapshots = 0;
        for stream in &mut self.meter_consumers {
            while let Ok(snapshot) = stream.consumer.try_pop() {
                visit(&snapshot);
                snapshots += 1;
            }
        }
        snapshots
    }

    /// Renders exactly one block. This is the whole of what a driver may time.
    ///
    /// # Errors
    ///
    /// Returns [`RenderFailed`] if the output buffer could not be described or the plan refused
    /// the
    /// block. The caller counts these into `render_errors` rather than panicking, so a run that
    /// fails to render still reports how often it failed.
    pub fn render(&mut self, observation: u64) -> Result<(), RenderFailed> {
        self.plan
            .render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut self.output.0, 2, QUANTUM, QUANTUM)
                        .map_err(|_| RenderFailed)?,
                },
                RenderTime {
                    absolute_sample: observation * QUANTUM as u64,
                },
            )
            .map(|_| ())
            .map_err(|_| RenderFailed)
    }

    /// Folds this block's rendered output into a digest. Outside the clock, always.
    pub fn hash_output(&self, hash: &mut Sha256Sink) {
        for value in &self.output.0 {
            hash.update(value.to_bits().to_le_bytes());
        }
    }
}

/// Clones the fixture's strips up to `tracks`, keeping every track's parameters distinct.
///
/// The stretch fixture is synthetic and says so in its record. It is a clone of the 64-track
/// model rather than a second checked-in session because nothing about 128 tracks is a new
/// *shape* -- it is sixteen full banks instead of eight -- and a second 288 KiB fixture would be
/// 128 tracks of duplicated text to review for no additional coverage.
fn synthesise_tracks(model: &mut session::SessionModel, tracks: usize) {
    let template: Vec<_> = model.tracks.clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template[index % template.len()].clone();
        track.id = StableId::parse(&format!("ch{index:03}")).expect("synthetic track id");
        let mut next = route.clone();
        next.id = StableId::parse(&format!("ch{index:03}-main")).expect("synthetic route id");
        next.source = session::RouteSource::Track {
            track_id: track.id.clone(),
            tap: session::SendTap::PostMatrix,
        };
        model.tracks.push(track);
        model.routes.push(next);
    }
}
fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> builtins_compiler::BuiltinCompileCaps {
    builtins_compiler::BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

fn effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 28,
        maximum_scratch_bytes: 1 << 28,
        maximum_automation_spans_per_block: 32,
    }
}

fn graph_caps() -> graph::GraphCompileCaps {
    graph::GraphCompileCaps {
        maximum_nodes: 100_000,
        maximum_edges: 100_000,
        maximum_schedule_items: 100_000,
        maximum_dependency_levels: 100_000,
        maximum_audio_buffer_samples: 100_000_000,
        maximum_delay_samples_per_edge: 1_000_000,
        maximum_total_delay_samples: 100_000_000,
        maximum_graph_bytes: 100_000_000,
        maximum_plan_bytes: 1_000_000_000,
        maximum_single_allocation_bytes: 100_000_000,
        maximum_finite_tail_samples: 10_000_000,
    }
}

/// Values in one track's frozen input block: `QUANTUM` left samples then `QUANTUM` right.
pub const SOURCE_BLOCK_VALUES: usize = QUANTUM * 2;

/// The tone's phase advance per frame, in radians (about 130 Hz at 48 kHz).
///
/// Named (issue #1011) so the browser arm of `console_mixing_automation`, which streams a tone of
/// its own through the web host, can state its tone against this one from the constants rather
/// than from a transcription. [`source_block`] computes exactly what it always did.
pub const TONE_RADIANS_PER_FRAME: f32 = 0.017;
/// The tone's per-track phase offset, in radians: track `t` starts its block at `t` times this.
pub const TONE_TRACK_PHASE_RADIANS: f32 = 0.31;
/// The tone's left-channel peak amplitude. The right channel is the left scaled by `-0.75`.
pub const TONE_AMPLITUDE: f32 = 0.6;

/// One track's frozen input block, left channel followed by right.
///
/// # Why this is public
///
/// Because a cross-target driver has to be able to compute it on **one** target and hand the
/// result to the other. The tone is a sine, `f32::sin` is a libm call, and libm is not the same
/// implementation on `x86_64-unknown-linux-gnu` as it is on `wasm32-unknown-unknown`. Measured on
/// the #163 phase 2 arm: with this tone computed locally on each target, four of the nine console
/// rows rendered different bits on wasm than on native; with the identical tone injected into
/// both, all nine rows agree to the byte, at both lane widths.
///
/// So that difference was never the engine's. It was the benchmark's *input*, and it would have
/// been reported as a cross-target numeric divergence by anyone who did not look. Nothing
/// downstream of this function calls libm.
#[must_use]
pub fn source_block(track: usize, silent: bool) -> Vec<f32> {
    let mut values = vec![0.0; SOURCE_BLOCK_VALUES];
    if !silent {
        for frame in 0..QUANTUM {
            let value = ((frame as f32) * TONE_RADIANS_PER_FRAME
                + track as f32 * TONE_TRACK_PHASE_RADIANS)
                .sin()
                * TONE_AMPLITUDE;
            values[frame] = value;
            values[QUANTUM + frame] = -value * 0.75;
        }
    }
    values
}

/// Where a prepared arm's input samples come from.
///
/// Both variants produce the *same numbers* when the driver supplies what [`source_block`] would
/// have produced. The distinction exists so that a cross-target comparison can guarantee that
/// rather than assume it.
pub enum SourceSignal {
    /// Computed by [`source_block`] on whichever target renders. What the native bench uses, and
    /// what every recorded native console number was taken with.
    Local,
    /// Supplied by the driver: `tracks * SOURCE_BLOCK_VALUES` values in track-major order.
    Injected(Vec<f32>),
}

impl SourceSignal {
    /// This signal's block for one track, or `None` if an injected table does not cover it.
    fn block(&self, track: usize, silent: bool) -> Option<Vec<f32>> {
        match self {
            Self::Local => Some(source_block(track, silent)),
            Self::Injected(table) => table
                .get(track * SOURCE_BLOCK_VALUES..(track + 1) * SOURCE_BLOCK_VALUES)
                .map(<[f32]>::to_vec),
        }
    }
}

/// Every track input is a frozen block per observation; nothing is decoded on the render path.
///
/// Aligned to 64 bytes (issue #935) so both planes start a cache line wherever the struct is
/// boxed: the bound feed boxes one per track input ([`bound_track_source`]) and the driver-fed row
/// one slice of them ([`FrozenSourceDriver`]). Unaligned, the allocator's 16-byte placement split
/// half the 32-byte loads that copy or read these words, and moved the ring row's time between
/// builds for no reason in the engine. `repr(C)` keeps `left` at offset 0 and `right` at
/// `QUANTUM * 4`, a line multiple, and the size stays exactly the two planes: no padding.
#[repr(C, align(64))]
struct FrozenGraphSource {
    left: [f32; QUANTUM],
    right: [f32; QUANTUM],
}

impl FrozenGraphSource {
    /// `silent` writes exact zeros rather than a scaled-down tone.
    ///
    /// Exact zeros because "quiet" and "silent" are different measurements: a very small nonzero
    /// signal keeps every filter and every detector working, and on some hosts pushes them into
    /// denormal arithmetic, which would make the idle row report a cost *higher* than the console
    /// row for reasons that have nothing to do with idling.
    ///
    /// # Why the channel mapping is honoured here
    ///
    /// `mapping` is the track's declared `(left_source_channel, right_source_channel)`, and this
    /// is where the declaration becomes samples. Every fixture in this suite but the mono one maps
    /// `(0, 1)`, so this changed no existing row's bits when it arrived -- but the mono fixture
    /// maps `(0, 0)`, and a binding that ignored that would have written the tone into the left
    /// plane and its scaled inverse into the right plane of a session that *declares* both
    /// channels to be one source channel.
    ///
    /// That is not a cosmetic difference. The channel-symmetry witness' `SOURCE` term is decided
    /// from the declaration, so a mono session fed asymmetric samples would be a session the
    /// witness calls collapse-eligible and whose two channels genuinely differ -- exactly the
    /// state in which a collapse renders wrong audio, arriving through the *benchmark's* input
    /// rather than through the engine. The subject honours the mapping so that the mono row-pair's
    /// digest equality is a statement about the collapse and not about the harness.
    ///
    /// # Panics
    ///
    /// Panics if the mapping names a channel the frozen block does not carry. The block is
    /// stereo by construction ([`SOURCE_BLOCK_VALUES`]), so a third channel index is a fixture the
    /// subject cannot feed, and feeding it silence instead would be a measurement of a fiction.
    fn from_block(block: &[f32], mapping: (usize, usize)) -> Self {
        let planes = [&block[..QUANTUM], &block[QUANTUM..SOURCE_BLOCK_VALUES]];
        let plane = |channel: usize| {
            *planes
                .get(channel)
                .unwrap_or_else(|| panic!("the frozen source block carries no channel {channel}"))
        };
        let mut left = [0.0; QUANTUM];
        let mut right = [0.0; QUANTUM];
        left.copy_from_slice(plane(mapping.0));
        right.copy_from_slice(plane(mapping.1));
        Self { left, right }
    }
}

impl GraphRuntimeProcessor for FrozenGraphSource {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
        block.left.copy_from_slice(&self.left);
        block.right.copy_from_slice(&self.right);
        Ok(())
    }
}

/// Every track's declared `(left_source_channel, right_source_channel)`, in model order.
///
/// Read from the compiled model rather than assumed, because it is the field the mono fixture
/// moves and the field the `half_mono` row moves back on half its tracks. See
/// [`FrozenGraphSource::from_block`] for why the subject honours it instead of always writing a
/// stereo pair.
fn channel_mappings(model: &SessionModel) -> Vec<(usize, usize)> {
    model
        .tracks
        .iter()
        .map(|track| {
            (
                usize::from(track.left_source_channel),
                usize::from(track.right_source_channel),
            )
        })
        .collect()
}

/// One track input's frozen block, from its track id: the words both feeds serve for that track.
///
/// The one place a track id becomes samples. The bound feed wraps the result in a processor
/// ([`source_binding`]) and the driver-fed row serves it from a source set
/// ([`FrozenSourceDriver`]), so the two feeds cannot disagree about which block, or which channel
/// mapping, a track reads.
fn frozen_track_source(
    track_id: &str,
    silent: bool,
    source: &SourceSignal,
    mappings: &[(usize, usize)],
) -> FrozenGraphSource {
    let track = track_id
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .unwrap_or(0);
    let block = source
        .block(track, silent)
        .unwrap_or_else(|| panic!("the injected source table must cover track {track}"));
    let mapping = mappings
        .get(track)
        .copied()
        .unwrap_or_else(|| panic!("the model must declare a source mapping for track {track}"));
    FrozenGraphSource::from_block(&block, mapping)
}

/// Whether `node` is a track's input stage: the nodes a source set may claim.
fn is_track_input(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::Input,
            ..
        }
    )
}

/// The bound feed's processor for one track input: its frozen block, boxed at the 64-byte
/// alignment [`FrozenGraphSource`] carries.
fn bound_track_source(
    track_id: &str,
    silent: bool,
    source: &SourceSignal,
    mappings: &[(usize, usize)],
) -> Box<FrozenGraphSource> {
    Box::new(frozen_track_source(track_id, silent, source, mappings))
}

fn source_binding(
    node: &GraphNodeId,
    silent: bool,
    source: &SourceSignal,
    mappings: &[(usize, usize)],
) -> GraphNodeBinding {
    if let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    {
        GraphNodeBinding::new(
            node.clone(),
            bound_track_source(track_id.as_str(), silent, source, mappings),
        )
    } else {
        GraphNodeBinding::identity(node.clone())
    }
}

/// A prepared source set's driver over the benchmark's frozen blocks (issue #928).
///
/// The production feed on the benchmark's input. A host binds a session's track inputs to a
/// source set whose driver plays a block per quantum; this one plays the same frozen block every
/// quantum, as the bound feed's processor copies the same block every quantum. It holds one
/// `FrozenGraphSource` per claim, in claim order, each built by [`frozen_track_source`] -- the
/// function the bound feed's processors are built by -- so claim `i` carries exactly the words the
/// bound feed would copy for that track, channel mapping included.
///
/// It offers the graph both ways to read a claim: `copy_track_input` copies the claim's block into
/// the arena buffer the executor hands it, and `played_planes` lends the same words in place.
/// Which one a claim takes is decided by the graph at bind (the lent-claims path of issue #918),
/// not by this driver.
struct FrozenSourceDriver {
    /// Claim `i`'s `(left, right)` block, one allocation for the whole set.
    claims: Box<[FrozenGraphSource]>,
}

impl FrozenSourceDriver {
    /// One frozen block per claim, in the order `claims` lists them.
    ///
    /// # Panics
    ///
    /// Panics if a claim names anything but a track input: a source set may claim nothing else,
    /// and the graph would refuse the set at bind anyway.
    fn new(
        claims: &[GraphSourceInputClaim],
        silent: bool,
        source: &SourceSignal,
        mappings: &[(usize, usize)],
    ) -> Self {
        let claims = claims
            .iter()
            .map(|claim| match &claim.node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::Input,
                } => frozen_track_source(track_id.as_str(), silent, source, mappings),
                node => panic!("a source claim must name a track input, not {node:?}"),
            })
            .collect();
        Self { claims }
    }

    /// The set's engine-owned bytes: the one boxed slice of frozen planes.
    ///
    /// Charged as overhead (source-plane storage) and not as PCM already charged by the session
    /// declaration, because no session declaration charges the benchmark's frozen tone.
    fn resource_report(&self) -> GraphSourceSetResourceReport {
        let bytes = core::mem::size_of_val(&*self.claims) as u64;
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: 0,
            overhead_bytes: bytes,
            total_engine_owned_bytes: bytes,
            largest_allocation_bytes: bytes,
        }
    }
}

// REALTIME_POLICY_BEGIN
// The five methods below run on the render thread: no allocation, lock, syscall or panic path.
impl GraphPreparedSourceSetDriver for FrozenSourceDriver {
    fn claim_count(&self) -> usize {
        self.claims.len()
    }

    /// Nothing to play: the block is frozen. A frame count other than the quantum the planes
    /// were built at is refused rather than served short.
    fn begin_block(&mut self, _first_sample: u64, frames: u32) -> Result<(), RenderError> {
        if usize::try_from(frames) == Ok(QUANTUM) {
            Ok(())
        } else {
            Err(RenderError::InvalidEnvelope)
        }
    }

    /// Copies claim `claim_index`'s frozen block into the destinations the executor hands it.
    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        let Some(claim) = self.claims.get(claim_index) else {
            return Err(RenderError::InvalidEnvelope);
        };
        if left.len() != QUANTUM || right.len() != QUANTUM {
            return Err(RenderError::InvalidEnvelope);
        }
        left.copy_from_slice(&claim.left);
        right.copy_from_slice(&claim.right);
        Ok(())
    }

    fn provides_played_planes(&self) -> bool {
        true
    }

    /// Lends claim `claim_index`'s frozen block in place: the words `copy_track_input` copies.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])> {
        self.claims
            .get(claim_index)
            .map(|claim| (&claim.left[..], &claim.right[..]))
    }
}
// REALTIME_POLICY_END

#[cfg(test)]
mod tests {
    use super::*;
    use engine::realtime::audit;

    /// The row facts a record states, less the feed: what the two gain/pan rows must share.
    fn stated_facts(workload: Workload) -> (u32, &'static str, bool, &'static str, &'static str) {
        (
            workload.tracks(),
            workload.fixture_id(),
            workload.synthetic(),
            workload.strip_content(),
            workload.strip_layout(),
        )
    }

    /// The gain/pan row's standing 64-block digest: the pin `tests/chain_shape.rs` takes in
    /// `the_select_free_matrix_arm_renders_the_base_bits`.
    const GAIN_PAN_DIGEST: &str =
        "01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4";

    /// Issues #928 and #956: the driver-fed row is the gain/pan row in every stated fact but its
    /// feed, and it is the only driver-fed row.
    #[test]
    fn the_driver_fed_row_states_the_gain_pan_rows_facts_and_its_own_feed() {
        let bound = Workload::SixtyFourTrackGainPanOnly;
        let ring = Workload::SixtyFourTrackGainPanRing;
        assert_eq!(ring.kind(), "sixty_four_track_gain_pan_ring");
        assert_eq!(stated_facts(ring), stated_facts(bound));
        assert_eq!(ring.input_signal(), bound.input_signal());
        assert_eq!(ring.warmup_blocks(), bound.warmup_blocks());
        assert!(ring.strip() == Strip::GainPan && bound.strip() == Strip::GainPan);
        assert!(!ring.collapse_forced_off());
        assert!(!ring.web_meters());
        assert_eq!(ring.source_feed().name(), "played_planes");
        assert_eq!(bound.source_feed().name(), "bound");
        for workload in WORKLOADS {
            assert_eq!(
                workload.source_feed(),
                SourceFeed::Bound,
                "{}",
                workload.kind()
            );
        }
        assert!(
            DRIVER_FED_WORKLOADS == [ring],
            "the driver-fed row is the only one"
        );
        let mut kinds: Vec<&str> = WORKLOADS
            .iter()
            .chain(DRIVER_FED_WORKLOADS.iter())
            .map(|workload| workload.kind())
            .collect();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(kinds.len(), WORKLOADS.len() + DRIVER_FED_WORKLOADS.len());
    }

    /// Issue #956 gate 1 (#928 gate 1, re-based): the driver-fed gain/pan row renders the
    /// bound-feed row's bits, and every claim is read in place by its bank's gather.
    ///
    /// Sixty-four blocks of both rows, digested by `hash_output`, must agree to the byte and equal
    /// the gain/pan row's standing pin: the two rows are one session and one frozen tone, and the
    /// feed is the only thing bound differently. Beside the digest, the plan facts the feed must
    /// not move: the same bank chains and slots, the same transposes, every route folded, and no
    /// collapse or transition.
    ///
    /// The source-plane counts (`graph::test_only_source_plane_counts`, `[claims copied into the
    /// arena, gathers served from a played block, gathers served silence]`) are the statement of
    /// which feed ran. The bound row has no source set and counts nothing. The driver-fed row's
    /// sixty-four claims are each read by one reader, their cohort's `PostInputBuiltins` bank, whose
    /// gather reads the played block in place (issue #918, `source_plane_table` clause (b)): no
    /// claim is copied, sixty-four gathers per block are served from played blocks, and none
    /// silence.
    ///
    /// Every render runs under the realtime audit, and the audited allocator aborts the process on
    /// an allocation inside a render scope, so zero forbidden operations is the driver's five
    /// methods and the source-set loop around them staying allocation-, lock- and syscall-free.
    #[test]
    fn the_driver_fed_gain_pan_row_renders_the_bound_rows_bits() {
        const BLOCKS: u64 = 64;
        let run = |workload: Workload| {
            let mut runtime = SessionRuntime::new(workload);
            let mut digest = Sha256Sink::new();
            let mut audible = false;
            graph::test_only_source_plane_reset();
            audit::warm_up();
            audit::reset();
            for block in 0..BLOCKS {
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut digest);
                audible |= runtime.output.0.iter().any(|word| *word != 0.0);
            }
            let forbidden = audit::snapshot().total();
            let planes = graph::test_only_source_plane_counts();
            (runtime, digest.finish_hex(), audible, forbidden, planes)
        };
        let (bound, bound_digest, bound_audible, bound_forbidden, bound_planes) =
            run(Workload::SixtyFourTrackGainPanOnly);
        let (ring, ring_digest, ring_audible, ring_forbidden, ring_planes) =
            run(Workload::SixtyFourTrackGainPanRing);

        assert!(
            bound_audible && ring_audible,
            "both rows must render the tone, or their equality says nothing"
        );
        assert_eq!(
            bound_digest, GAIN_PAN_DIGEST,
            "the gain/pan row moved: this is not the row the pin was taken on"
        );
        assert_eq!(
            ring_digest, bound_digest,
            "the driver-fed row must render the bound-feed row's bits: a difference is a harness \
             defect, never a finding"
        );
        let tracks = u64::from(Workload::SixtyFourTrackGainPanRing.tracks());
        assert_ne!(
            bound.bank_shape(),
            [0, 0],
            "the gain/pan row binds bank chains"
        );
        assert_eq!(
            ring.bank_shape(),
            bound.bank_shape(),
            "the feed moves no bank chain or slot"
        );
        assert_eq!(ring.bank_transposes(), bound.bank_transposes());
        for (name, runtime) in [("bound", &bound), ("driver-fed", &ring)] {
            assert_eq!(
                runtime.bank_route_folds(),
                tracks,
                "{name}: every route folds into its cohort's epilogue"
            );
            assert_eq!(
                runtime.bank_collapse_counters()[0],
                0,
                "{name}: no collapsed block"
            );
            assert_eq!(
                runtime.bank_collapse_transitions(),
                [0, 0, 0],
                "{name}: no transition"
            );
        }
        assert_eq!(
            bound_planes,
            [0, 0, 0],
            "the bound feed binds no source set"
        );
        assert_eq!(
            ring_planes,
            [0, tracks * BLOCKS, 0],
            "every claim is read in place by its bank's gather every block, none copied"
        );
        assert_eq!(
            (bound_forbidden, ring_forbidden),
            (0, 0),
            "no forbidden operation on either render path"
        );
    }

    /// Issue #956 gate 2 (#936 gate 1, re-homed): at the native width the executor dispatches
    /// only the units that do work.
    ///
    /// Both gain/pan rows bind the same units: sixty-four track inputs, the cohorts' bank chains
    /// and the Output op. On the bound-feed row every input is a host processor
    /// (`NodeKind::Bound`) that writes its buffer, so every unit is dispatched every block. On the
    /// driver-fed row every input is a claim its bank's gather reads in place (issue #918): a
    /// plain, unobserved `SourceInput` op whose dispatch would return at once, which bind leaves
    /// out of the dispatched-unit table. So the loop dispatches every unit but the sixty-four
    /// inputs, once per block (`graph::test_only_unit_dispatches`, reset before every block), and
    /// the row still renders the bound row's bits
    /// (`the_driver_fed_gain_pan_row_renders_the_bound_rows_bits`).
    #[test]
    fn the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs() {
        const BLOCKS: u64 = 64;
        let tracks = u64::from(Workload::SixtyFourTrackGainPanRing.tracks());
        let mut censuses = Vec::new();
        for (workload, skipped) in [
            (Workload::SixtyFourTrackGainPanRing, tracks),
            (Workload::SixtyFourTrackGainPanOnly, 0),
        ] {
            let mut runtime = SessionRuntime::new(workload);
            let units = runtime.unit_eligibility().len() as u64;
            assert!(
                units > tracks,
                "{}: the inputs, the bank chains and the Output",
                workload.kind()
            );
            censuses.push(units);
            for block in 0..BLOCKS {
                graph::test_only_unit_dispatch_reset();
                runtime.render(block).expect("console render");
                assert_eq!(
                    graph::test_only_unit_dispatches(),
                    units - skipped,
                    "{}, block {block}: units dispatched",
                    workload.kind()
                );
            }
        }
        assert_eq!(
            censuses[0], censuses[1],
            "the census keeps every unit: the feed changes what is dispatched, not what is bound"
        );
    }

    /// Issue #956 gate 3 (#936 gate 5, re-homed): on the driver-fed gain/pan row, the runtime
    /// metadata the compile with builtins charges grows by exactly the byte lengths it charges for
    /// the executor's two bind-sized tables, and the tables the row binds fit in them.
    ///
    /// The row's graph is the one `build_full` compiles: `prepare_session_builtins` with no meter,
    /// then `compile_with_builtins` at the native width. Past the semantic estimate its graph
    /// metadata carries two charges. The runtime metadata is recomputed here from the compile's own
    /// inputs: one emitted op per scheduled node and one response owner per track (its id,
    /// `input-filters` and `miso.builtin.input-filters`) -- the row prepares no effect. The bank
    /// slot reservation is recomputed from the builtin banks the artifact retains: one slot per
    /// bank, masked at its width. Nothing else is charged there: the strip is banked whole at a
    /// vector width, so no scalar owner is retained, and with no effect there is no effect control
    /// or effect bank. The runtime metadata is the terms it carried before issue #936 -- the
    /// split-table field, the per-op layout delta, the observation state and the response owners,
    /// none of which the issue moved (`graph`'s
    /// `runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow` pins that the executor's
    /// layout witnesses carry the new table) -- plus the two tables, each at one entry per emitted
    /// op. The row binds every unit but its sixty-four inputs as dispatched units (4 bytes each)
    /// and copies no claim.
    #[test]
    fn the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables() {
        let workload = Workload::SixtyFourTrackGainPanRing;
        let model = console_model(workload);
        let session = compile_session(&model, compile_caps()).expect("compiled console session");
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .expect("prepared console effects");
        let builtins = builtins_compiler::prepare_session_builtins(&session, &[], builtin_caps())
            .expect("prepared console builtins");
        let Ok(artifact) = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: Backend::current(),
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
        }) else {
            panic!("gain/pan console graph");
        };
        let emitted = artifact
            .graph()
            .dependency_levels
            .iter()
            .map(|level| level.nodes.len() as u64)
            .sum::<u64>();
        let fixed = ["input-filters".len(), "miso.builtin.input-filters".len()];
        let (strings, largest) =
            model
                .tracks
                .iter()
                .fold((0_u64, 0_u64), |(total, largest), track| {
                    let id = track.id.as_str().len() as u64;
                    (
                        total + id + (fixed[0] + fixed[1]) as u64,
                        largest.max(id).max(fixed[0] as u64).max(fixed[1] as u64),
                    )
                });
        let resource =
            graph::GraphRuntimeMetadataResourceEstimate::checked_for_with_response_bindings(
                emitted,
                model.tracks.len() as u64,
                strings,
                largest,
            )
            .expect("runtime metadata");
        let banks: Vec<_> = artifact.prepared_builtin_banks().collect();
        assert!(!banks.is_empty(), "the gain/pan strip is banked");
        let mask_bytes = banks
            .iter()
            .map(|bank| u64::from(bank.width.lanes()))
            .max()
            .expect("one bank at least")
            * core::mem::size_of::<bool>() as u64;
        let slots =
            graph::GraphBankSlotResourceEstimate::checked_for_mask(banks.len() as u64, mask_bytes)
                .expect("bank slot reservation");
        let report = artifact.report();
        assert_eq!(
            report.estimate.graph_metadata_bytes - report.semantic_estimate.graph_metadata_bytes,
            resource.total_bytes + slots.total_bytes,
            "the runtime metadata and the bank slot reservation are the row's only charges past \
             the semantic estimate"
        );
        let tables = [
            emitted * core::mem::size_of::<u32>() as u64,
            emitted * core::mem::size_of::<(usize, u32)>() as u64,
        ];
        assert_eq!(
            [
                resource.active_unit_table_bytes,
                resource.source_input_table_bytes
            ],
            tables,
            "each table at one entry per emitted op"
        );
        let before = resource.runtime_field_bytes
            + resource.emitted_op_layout_delta_bytes * emitted
            + resource.observation_runtime_state_bytes
            + resource.response_binding_table_bytes
            + resource.response_binding_string_bytes;
        assert_eq!(
            resource.total_bytes,
            before + tables[0] + tables[1],
            "the charge grows by exactly the two tables"
        );
        let ring = SessionRuntime::new(workload);
        let bound = graph::test_only_executor_table_bytes();
        let dispatched = ring.unit_eligibility().len() as u64 - u64::from(workload.tracks());
        assert_eq!(
            bound,
            [dispatched * core::mem::size_of::<u32>() as u64, 0],
            "the row dispatches every unit but its inputs and copies no claim"
        );
        assert!(bound[0] <= tables[0] && bound[1] <= tables[1]);
    }

    /// The driver's five methods, called directly under the realtime audit: `played_planes` lends
    /// exactly the words `copy_track_input` copies, claim by claim, and both are the claimed
    /// track's frozen block through that track's declared channel mapping.
    ///
    /// Run over the half-mono model, whose even tracks map `(0, 0)` and odd tracks `(0, 1)`, so a
    /// driver that ignored the mapping, or served one claim another track's block, fails here even
    /// though the stereo gain/pan fixture could not show it. The expectation is computed from the
    /// model's track *position* and `source_block`, not through `frozen_track_source`, so it is not
    /// the driver checked against itself.
    #[test]
    fn the_frozen_source_driver_lends_the_words_it_copies() {
        let model = console_model(Workload::SixtyFourTrackConsoleHalfMono);
        let mappings = channel_mappings(&model);
        let mut claims: Vec<GraphSourceInputClaim> = model
            .tracks
            .iter()
            .map(|track| GraphSourceInputClaim {
                node: GraphNodeId::TrackStage {
                    track_id: graph::StableGraphId::parse(track.id.as_str()).expect("track id"),
                    stage: TrackStage::Input,
                },
            })
            .collect();
        claims.sort_unstable();
        let mut driver = FrozenSourceDriver::new(&claims, false, &SourceSignal::Local, &mappings);
        let count = claims.len();
        let report = driver.resource_report();
        assert_eq!(
            report.total_engine_owned_bytes,
            (count * SOURCE_BLOCK_VALUES * core::mem::size_of::<f32>()) as u64
        );
        assert_eq!(
            report.pcm_payload_already_charged_bytes + report.overhead_bytes,
            report.total_engine_owned_bytes
        );

        // Everything the scope writes is allocated before it opens.
        let mut copied = vec![0.0_f32; count * SOURCE_BLOCK_VALUES];
        let mut lent = vec![f32::NAN; count * SOURCE_BLOCK_VALUES];
        let mut copy_ok = vec![false; count];
        let mut lent_lengths = vec![(0, 0); count];
        let (mut short_left, mut short_right) = ([0.0_f32; QUANTUM - 1], [0.0_f32; QUANTUM - 1]);
        let (mut spare_left, mut spare_right) = ([0.0_f32; QUANTUM], [0.0_f32; QUANTUM]);
        let mut facts = [false; 7];
        audit::warm_up();
        audit::reset();
        audit::in_render_scope(|| {
            facts[0] = driver.claim_count() == count;
            facts[1] = driver.provides_played_planes();
            facts[2] = driver.begin_block(0, QUANTUM as u32).is_ok();
            facts[3] = driver.begin_block(0, QUANTUM as u32 / 2).is_err();
            for claim in 0..count {
                let (left, right) = copied[claim * SOURCE_BLOCK_VALUES..][..SOURCE_BLOCK_VALUES]
                    .split_at_mut(QUANTUM);
                copy_ok[claim] = driver.copy_track_input(claim, left, right).is_ok();
                if let Some((left, right)) = driver.played_planes(claim) {
                    lent_lengths[claim] = (left.len(), right.len());
                    if left.len() == QUANTUM && right.len() == QUANTUM {
                        let (lent_left, lent_right) = lent[claim * SOURCE_BLOCK_VALUES..]
                            [..SOURCE_BLOCK_VALUES]
                            .split_at_mut(QUANTUM);
                        lent_left.copy_from_slice(left);
                        lent_right.copy_from_slice(right);
                    }
                }
            }
            facts[4] = driver
                .copy_track_input(count, &mut spare_left, &mut spare_right)
                .is_err();
            facts[5] = driver.played_planes(count).is_none();
            facts[6] = driver
                .copy_track_input(0, &mut short_left, &mut short_right)
                .is_err();
        });
        assert_eq!(
            audit::snapshot().total(),
            0,
            "the driver's methods did something forbidden on the render thread"
        );
        assert_eq!(
            facts, [true; 7],
            "[claim count, lends, quantum admitted, half quantum refused, claim past the end not \
             copied, not lent, short destination refused]"
        );
        assert!(copy_ok.iter().all(|ok| *ok), "every claim copies");
        assert!(
            lent_lengths
                .iter()
                .all(|lengths| *lengths == (QUANTUM, QUANTUM)),
            "every claim lends one quantum per plane"
        );
        assert_eq!(copied, lent, "a lent plane is the plane the copy writes");

        let mut mono_claims = 0;
        for (claim, entry) in claims.iter().enumerate() {
            let GraphNodeId::TrackStage { track_id, .. } = &entry.node else {
                unreachable!("every claim is a track input");
            };
            let position = model
                .tracks
                .iter()
                .position(|track| track.id.as_str() == track_id.as_str())
                .expect("the claimed track is in the model");
            let block = source_block(position, false);
            let (left_channel, right_channel) = mappings[position];
            let words = &copied[claim * SOURCE_BLOCK_VALUES..][..SOURCE_BLOCK_VALUES];
            assert_eq!(
                &words[..QUANTUM],
                &block[left_channel * QUANTUM..][..QUANTUM],
                "{track_id:?}: left plane"
            );
            assert_eq!(
                &words[QUANTUM..],
                &block[right_channel * QUANTUM..][..QUANTUM],
                "{track_id:?}: right plane"
            );
            if left_channel == right_channel {
                mono_claims += 1;
                assert_eq!(
                    words[..QUANTUM],
                    words[QUANTUM..],
                    "{track_id:?}: one source"
                );
            } else {
                assert_ne!(
                    words[..QUANTUM],
                    words[QUANTUM..],
                    "{track_id:?}: two sources"
                );
            }
        }
        assert_eq!(
            mono_claims,
            count / 2,
            "the half-mono model maps half its tracks mono"
        );
    }

    /// Issue #935 gate 2 (re-homed onto the gain/pan pair by #956): every frozen block the harness
    /// serves, and both output planes, start a 64-byte line.
    ///
    /// The ring row's claims are built by `FrozenSourceDriver::new`, the constructor `build_full`
    /// calls, over that row's own track inputs, and each is read back through `played_planes`,
    /// the accessor the graph reads a claim in place through. The bound row's blocks are built by
    /// `bound_track_source`, the function `source_binding` boxes them with, and all sixty-four are
    /// held at once so each is its own allocation rather than one reused slot. The output planes
    /// are the two runtimes' own. Alignment follows from the types' layout, but every claim and
    /// every track is checked anyway: at the allocator's 16-byte granularity one allocation in
    /// four lands on a 64-byte boundary by luck.
    #[test]
    fn the_frozen_blocks_and_the_output_planes_start_a_cache_line() {
        let aligned = |plane: &[f32]| (plane.as_ptr() as usize).is_multiple_of(64);
        let plane_bytes = QUANTUM * core::mem::size_of::<f32>();
        assert_eq!(core::mem::align_of::<FrozenGraphSource>(), 64);
        assert_eq!(core::mem::offset_of!(FrozenGraphSource, right), plane_bytes);
        assert_eq!(
            core::mem::size_of::<FrozenGraphSource>(),
            2 * plane_bytes,
            "no padding: the driver's resource report is exactly the planes"
        );
        assert_eq!(core::mem::align_of::<OutputPlanes>(), 64);

        let ring = Workload::SixtyFourTrackGainPanRing;
        let model = console_model(ring);
        let mappings = channel_mappings(&model);
        let mut claims: Vec<GraphSourceInputClaim> = model
            .tracks
            .iter()
            .map(|track| GraphSourceInputClaim {
                node: GraphNodeId::TrackStage {
                    track_id: graph::StableGraphId::parse(track.id.as_str()).expect("track id"),
                    stage: TrackStage::Input,
                },
            })
            .collect();
        claims.sort_unstable();
        let driver = FrozenSourceDriver::new(&claims, false, &SourceSignal::Local, &mappings);
        assert_eq!(driver.claim_count(), ring.tracks() as usize);
        for claim in 0..driver.claim_count() {
            let (left, right) = driver.played_planes(claim).expect("every claim lends");
            assert!(
                aligned(left) && aligned(right),
                "ring claim {claim}: left {:p}, right {:p}",
                left.as_ptr(),
                right.as_ptr()
            );
        }

        let bound = Workload::SixtyFourTrackGainPanOnly;
        let model = console_model(bound);
        let mappings = channel_mappings(&model);
        let blocks: Vec<Box<FrozenGraphSource>> = model
            .tracks
            .iter()
            .map(|track| {
                bound_track_source(track.id.as_str(), false, &SourceSignal::Local, &mappings)
            })
            .collect();
        assert_eq!(blocks.len(), bound.tracks() as usize);
        for (track, block) in model.tracks.iter().zip(&blocks) {
            assert!(
                aligned(&block.left) && aligned(&block.right),
                "bound {}: left {:p}, right {:p}",
                track.id.as_str(),
                block.left.as_ptr(),
                block.right.as_ptr()
            );
        }

        for workload in [bound, ring] {
            let runtime = SessionRuntime::new(workload);
            let (left, right) = runtime.output.0.split_at(QUANTUM);
            assert!(
                aligned(left) && aligned(right),
                "{} output: left {:p}, right {:p}",
                workload.kind(),
                left.as_ptr(),
                right.as_ptr()
            );
        }
    }

    /// Issue #881: the metered row states every fact the standing console row states -- it is that
    /// session as written -- and it is the only row prepared with the web boot's meter set.
    #[test]
    fn the_metered_row_states_the_console_rows_facts_and_is_the_only_web_metered_row() {
        let console = Workload::SixtyFourTrackConsole;
        let metered = Workload::SixtyFourTrackConsoleMetered;
        assert_eq!(metered.kind(), "sixty_four_track_console_metered");
        assert_eq!(stated_facts(metered), stated_facts(console));
        assert_eq!(metered.input_signal(), console.input_signal());
        assert_eq!(metered.warmup_blocks(), console.warmup_blocks());
        assert!(metered.strip() == Strip::AsWritten && console.strip() == Strip::AsWritten);
        assert_eq!(metered.source_feed(), SourceFeed::Bound);
        assert!(!metered.collapse_forced_off());
        assert!(metered.web_meters());
        assert!(
            METERED_WORKLOADS == [metered],
            "the metered row is the only one"
        );
        for workload in WORKLOADS.into_iter().chain(DRIVER_FED_WORKLOADS) {
            assert!(!workload.web_meters(), "{}", workload.kind());
        }
        let rows: Vec<Workload> = native_session_rows().collect();
        assert_eq!(
            rows.len(),
            WORKLOADS.len() + DRIVER_FED_WORKLOADS.len() + METERED_WORKLOADS.len()
        );
        assert!(
            rows.last() == Some(&metered),
            "the metered row is emitted last, after the driver-fed row"
        );
        let mut kinds: Vec<&str> = rows.iter().map(|workload| workload.kind()).collect();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(kinds.len(), rows.len(), "every emitted kind is distinct");
    }

    /// Issue #881 gate 3: the metered row renders the standing console row's bits, differs from it
    /// in plan shape exactly by its fused fader and matrix, and every track publishes one
    /// sample-peak snapshot per twelve-block window, on time, with none dropped.
    ///
    /// Sixty-four blocks of both rows, digested by `hash_output`. The pin is the standing console
    /// row's 64-block digest (`tests/chain_shape.rs`,
    /// `the_select_free_matrix_arm_renders_the_base_bits`), and the metered row reproduces it,
    /// because a meter observes and never changes signal flow and the fused fader and matrix render
    /// the split pair's bits.
    ///
    /// The plan shape is **not** the standing row's, and this test pins how it differs. The
    /// metered row is prepared with between-render-calls delivery, the default web boot's, which
    /// fuses each cohort's fader and matrix banks into one stage; the standing row's `Concurrent`
    /// delivery keeps two. Both bank the same `[chains, slots]` (eight chains, 48 memberships) and
    /// transpose the same number of times, and both fold every route of the console (the #885
    /// contract). What moves is the stage count their chains run each block
    /// (`graph::test_only_bank_chain_construction_facts`, read across each bind): 48 on the
    /// standing row, one per membership, and 40 on the metered row, one fewer per cohort. Each
    /// plan's scatter redirect count is pinned on its own (0 on both today), because the two plans
    /// are two deliveries and neither is the other's baseline.
    ///
    /// The snapshots are drained after every block, which is what the bench does outside its
    /// clock. Sixty-four blocks close five twelve-block windows per track, so the row publishes
    /// `5 * 64` snapshots, and each window is published by the block that closes it: sixty-four
    /// snapshots after blocks 11, 23, 35, 47 and 59 and none after any other. Each track's five
    /// carry its own handle, consecutive window sequences, contiguous 1536-frame sample spans from
    /// sample zero, the `SAMPLE_PEAK` presence mask the selected entry prepares, zero dropped
    /// snapshots and zero discontinuities, and a positive peak on both channels (the tone is
    /// audible and no fader is muted). The unmetered row binds no stream and publishes nothing.
    ///
    /// Red mutations (run): select `MeterMetricSet::ALL` in `web_meter_requests` -- the presence
    /// assertion fails; set `WEB_METER_BLOCKS` to 4 -- the window count fails.
    #[test]
    fn the_metered_console_row_renders_the_console_bits_and_publishes_every_window() {
        const BLOCKS: u64 = 64;
        const CONSOLE_DIGEST: &str =
            "fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de";
        // The browser's window, written out rather than read back from `WEB_METER_BLOCKS`: the
        // web host's `DEFAULT_METER_BLOCKS` (12) blocks of the 128-frame quantum. A mirror that
        // drifted from the host would otherwise move this test with it.
        let window = 12;
        let window_frames = window * 128;
        assert_eq!(QUANTUM, 128);
        let run = |workload: Workload| {
            graph::test_only_reset_bank_chain_construction_facts();
            let mut runtime = SessionRuntime::new(workload);
            let chains = graph::test_only_bank_chain_construction_facts();
            let mut digest = Sha256Sink::new();
            let mut published: Vec<(u64, MeterSnapshot)> = Vec::new();
            audit::warm_up();
            audit::reset();
            for block in 0..BLOCKS {
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut digest);
                runtime.drain_meter_snapshots(|snapshot| published.push((block, *snapshot)));
            }
            let forbidden = audit::snapshot().total();
            (runtime, chains, digest.finish_hex(), published, forbidden)
        };
        let (console, console_chains, console_digest, console_published, console_forbidden) =
            run(Workload::SixtyFourTrackConsole);
        let (metered, metered_chains, metered_digest, metered_published, metered_forbidden) =
            run(Workload::SixtyFourTrackConsoleMetered);

        assert_eq!(
            console_digest, CONSOLE_DIGEST,
            "the standing console row moved: this is not the row the pin was taken on"
        );
        assert_eq!(
            metered_digest, console_digest,
            "the metered row must render the standing console row's bits: a meter observes and \
             never changes signal flow"
        );
        assert_eq!(
            (console_forbidden, metered_forbidden),
            (0, 0),
            "no forbidden operation on either render path"
        );
        // One cohort per lane-width of tracks, six bank slots each: eight cohorts at the eight-lane
        // launch width, sixteen on a four-lane (AArch64 NEON) build (#1017).
        let tracks = u64::from(Workload::SixtyFourTrackConsoleMetered.tracks());
        let width = Backend::current().width() as u64;
        let cohorts = tracks / width;
        assert_eq!(
            [console.bank_shape(), metered.bank_shape()],
            [[cohorts, 6 * cohorts], [cohorts, 6 * cohorts]],
            "both plans bank the same memberships in the same {cohorts} chains at width {width}"
        );
        assert_eq!(metered.bank_transposes(), console.bank_transposes());
        // The delivery difference, pinned rather than described: the same six memberships per
        // cohort (48 at the launch width) run as six chain stages per cohort on the standing row and
        // as five on the metered row, whose between-render-calls delivery fuses each cohort's fader
        // and matrix into one stage.
        assert_eq!(
            [
                console_chains.run_memberships,
                console_chains.runtime_slots,
                metered_chains.run_memberships,
                metered_chains.runtime_slots,
            ],
            [6 * cohorts, 6 * cohorts, 6 * cohorts, 5 * cohorts].map(|count| count as usize),
            "[standing memberships, standing stages, metered memberships, metered stages] at width \
             {width}"
        );
        assert_eq!(
            [console.bank_route_folds(), metered.bank_route_folds()],
            [tracks, tracks],
            "every route of the console folds, under either delivery and metered or not (#885)"
        );
        assert_eq!(
            [
                console.bank_scatter_redirects(),
                metered.bank_scatter_redirects()
            ],
            [0, 0],
            "each plan's own redirect count: two deliveries, so two pins and not one comparison"
        );

        assert_eq!(console.meter_streams(), 0);
        assert!(
            console_published.is_empty(),
            "the unmetered row publishes nothing"
        );
        assert_eq!(
            metered.meter_streams() as u64,
            tracks,
            "one stream per track"
        );
        assert!(
            metered.meter_taps().all(|tap| tap == MeterTap::PostMatrix),
            "every stream observes the post-matrix tap"
        );
        let windows = BLOCKS / window;
        assert_eq!(
            metered_published.len() as u64,
            tracks * windows,
            "five windows per track, none dropped and none extra"
        );
        for block in 0..BLOCKS {
            let drained = metered_published
                .iter()
                .filter(|(after, _)| *after == block)
                .count() as u64;
            let closes = (block + 1) % window == 0;
            assert_eq!(
                drained,
                if closes { tracks } else { 0 },
                "block {block}: every track publishes on the block that closes its window"
            );
        }
        for handle in 1..=tracks {
            let stream: Vec<&(u64, MeterSnapshot)> = metered_published
                .iter()
                .filter(|(_, snapshot)| snapshot.handle.0.get() == handle)
                .collect();
            assert_eq!(
                stream.len() as u64,
                windows,
                "handle {handle}: every window"
            );
            for (sequence, (after, snapshot)) in stream.into_iter().enumerate() {
                let sequence = sequence as u64;
                assert_eq!(snapshot.window_sequence, sequence, "handle {handle}");
                assert_eq!(*after, (sequence + 1) * window - 1, "handle {handle}");
                assert_eq!(snapshot.start_sample, sequence * window_frames);
                assert_eq!(snapshot.end_sample, (sequence + 1) * window_frames);
                assert_eq!(u64::from(snapshot.frames), window_frames);
                assert!(
                    snapshot.present_metrics == MeterMetricSet::SAMPLE_PEAK,
                    "handle {handle}: the selected entry computes the sample peak only"
                );
                assert_eq!(
                    (snapshot.reset_generation, snapshot.observation_generation),
                    (0, 0),
                    "handle {handle}: a permanent observer is never re-armed"
                );
                assert_eq!(
                    snapshot.cumulative_dropped_snapshots, 0,
                    "handle {handle}: dropped a snapshot"
                );
                assert_eq!(snapshot.cumulative_discontinuities, 0, "handle {handle}");
                for peak in [snapshot.left.sample_peak, snapshot.right.sample_peak] {
                    assert!(
                        peak.is_finite() && peak > 0.0,
                        "handle {handle}: the meter read the audible tone ({peak})"
                    );
                }
            }
        }
    }
}
