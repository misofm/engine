#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The console benchmark's *subject*, linked by the native bench (the wasm guest it was first
//! built for went with #1039, ruling R9).
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
//! The measurement. The production subject has no clock, percentile, record or statistic.
//! Timing belongs to the driver: the native bench or the embedding host of the shipped browser
//! artifact. R9 retired the Wasmtime driver and descriptive nightly benchmarks.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::Arc;

use bench_support::digest::Sha256Sink;
use builtins::{MeterConfig, MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::{
    MeterConsumer, MeterRequest, SelectedMeterRequest, TrackControlProducer, TrackControlRequest,
};
use effect_compiler::{
    EffectCompileCaps, EffectControlProducer, EffectObservationHandle, attach_effect_live_controls,
    attach_effect_observation, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::{
    BankWidth, ChannelSymmetryWitness, EffectControlRecord, ParameterChannel, PreparedEffectTarget,
};
use engine::realtime::{
    LiveStamp, LiveValuesSuperseded, PlanRevisionGate, PlanUnitEligibility, PlanarBufferMut,
    PreparedRenderPlan, RenderError, RenderIo, RenderTime,
};
use engine::{QuantumFrames, SampleRateHz};
use graph::{
    GraphNodeBinding, GraphNodeId, GraphPreparedSourceSet, GraphPreparedSourceSetDriver,
    GraphRuntimeBindings, GraphSourceInputClaim, GraphSourceSetResourceReport, TrackStage,
};
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use lane::Backend;
use session::{
    CompileCaps, DualMonoFader, MatrixOrPan, SessionModel, StableId, compile_session,
    parse_session_json,
};
use source::{
    HostChunkError, HostChunkProvider, HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig,
    PublishedGeneration, SourceFrame, SourceGeneration, SourceGraphSource, SourceGraphTrackMapping,
    SourceTimeline, TimelineProducer, prepare_session_graph_source_set,
};

pub mod fader_ride;
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
/// Cap on declared observation taps per effect, passed to the observation attach.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 8;
/// Bounded depth of each meter stream. Drained outside the clock after every observation.
pub const METER_QUEUE_DEPTH: usize = 8;
/// Blocks per meter window on the metered console row (issue #881): the default web boot's.
///
/// `WebBootOptions::live_control_defaults` sets `live_control_meter_blocks` to the web host's
/// `DEFAULT_METER_BLOCKS`, twelve, and its `live_control_request` turns that into a period of
/// `12 * quantum_frames`. Mirrored here rather than imported: the browser host is not a
/// dependency of this subject, and this crate names the number it renders with.
pub const WEB_METER_BLOCKS: u32 = 12;
/// Bounded depth of each meter stream on the metered console row: the depth the web host's
/// `live_control_request` asks for (one window per post, plus headroom for a stalled control side).
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
/// The app-shape fixture (issue #1085): what `misofm/app` compiles, on the standing console.
///
/// Generated from the standing fixture by `scripts/derive-app-console-fixture.py`, which keeps the
/// standing EQ and compressor as the session's two `pre_insert` slots and each track's entries for
/// them (decision 12's console shape, #1093), drops the limiter's `post_insert` slot and entries,
/// and bypasses both entries on every track whose index is 2 mod 3. B0 wrote the same effects
/// into every track's `dynamic` rack; the console migration moved them to `pre_insert` with their
/// bits unchanged. A committed document rather than a strip edit, because the browser arm boots
/// the same bytes the native row renders.
const SIXTY_FOUR_TRACK_APP: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-app.json");
/// The bus-and-send fixture (issue #1227): the standing console's 64 tracks feeding eight
/// processed submix strips, two effect returns and two sends per track.
///
/// Generated from the standing fixture by `scripts/derive-sends-console-fixture.py`, which keeps
/// the source, the rate, the quantum, the console declaration and every track as written, and
/// adds ten submixes (`bus-0`..`bus-7`, `fx-a`, `fx-b`, each with the standing strip and one
/// insert), re-points each track's main route into its bus, and adds a pre-fader send into `fx-a`
/// and a post-fader send into `fx-b` per track and a return per submix. A committed document
/// rather than a strip edit, because every strip edit is a removal (`apply_strip`) and this one
/// is an addition, and because the browser arm boots the same bytes the native row renders.
const SIXTY_FOUR_TRACK_SENDS: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-sends.json");

/// The console app's C1 session at 16, 32 and 64 tracks (issue #2132), copied from the app's session
/// builder (`misofm/console`, `app/src/studio/console-session.ts`, `buildConsoleSession`) as it
/// stands, with every track assigned to group `index mod 4`. Not referenced across repositories.
const C1_SIXTEEN: &str = include_str!("../../../fixtures/session/v1/console-c1-16.json");
const C1_THIRTY_TWO: &str = include_str!("../../../fixtures/session/v1/console-c1-32.json");
const C1_SIXTY_FOUR: &str = include_str!("../../../fixtures/session/v1/console-c1-64.json");
/// The irregular graph (issue #2132): C1's console slots on every strip, every one active; ragged
/// inserts (0 to 3 per track), one sidechained insert, three submix depths, mono and stereo stems
/// mixed, and two tracks routed to the master directly.
const IRREGULAR_GRAPH: &str = include_str!("../../../fixtures/session/v1/irregular-graph.json");

/// The C1 rows' track counts (issue #2132).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1Tracks {
    /// Sixteen tracks.
    Sixteen,
    /// Thirty-two tracks.
    ThirtyTwo,
    /// Sixty-four tracks.
    SixtyFour,
}

impl C1Tracks {
    /// The track count.
    #[must_use]
    pub const fn count(self) -> u32 {
        match self {
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
            Self::SixtyFour => 64,
        }
    }
}

/// What a C1 row's console slots are doing (issue #2132).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1State {
    /// The session as the console loads it: every console slot bypassed, the limiter bypassed.
    Load,
    /// A seeded mix ([`C1_TYPICAL_SEED`]): on each strip, independently, the EQ is active with
    /// probability 75 %, the compressor 50 %, the gate 10 % and the drive 10 %. The limiter stays
    /// bypassed.
    Typical,
    /// Every console slot and the limiter active.
    AllActive,
    /// [`Self::AllActive`] with every odd track fed exact zeros.
    Sparse,
    /// [`Self::AllActive`] with `ch00`'s gate bypassed and un-bypassed in turns of
    /// [`C1_TOGGLE_BLOCKS`] blocks, through its live-control channel
    /// ([`SessionRuntime::before_block`]), so every later slice's `to_bits` evidence carries
    /// un-bypass transitions.
    Toggle,
    /// [`Self::AllActive`] with `miso.reverb` at mix 1 inserted on the `aux.b` return and every
    /// track's post-fader send to `aux.b` unmuted at [`C1_REVERB_SEND_DB`] (issue #2147), so the
    /// row measures a fed per-node reverb.
    Reverb,
}

/// The gain of every track's send to `aux.b` in [`C1State::Reverb`].
pub const C1_REVERB_SEND_DB: f32 = -6.0;

/// The seed of [`C1State::Typical`]'s slot draw.
pub const C1_TYPICAL_SEED: u64 = 2132;

/// The blocks between two of [`C1State::Toggle`]'s bypass writes.
pub const C1_TOGGLE_BLOCKS: u64 = 64;

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
    /// same arm `sixty_four_track_gain_pan_ring` runs. Both run over the same lanes every block.
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
    /// It is also the native pure-audio-path target. Its sixty-four `TrackStage::Input` nodes are
    /// claimed by the prepared source set every row binds (issue #965), and every claim's only
    /// reader is its cohort's `PostInputBuiltins` bank, whose gather reads the played block in place
    /// (issue #918, `source_plane_table` clause (b)): no claim is copied into the arena and the
    /// input units are left out of the dispatched-unit table (issue #936). That is the feed
    /// `crates/host-core/tests/source_in_place.rs` pins.
    ///
    /// Its name keeps the `ring` of issue #956, which re-based the driver-fed row of issue #928
    /// onto this session. Its bound-feed twin, `sixty_four_track_gain_pan_only`, rendered the same
    /// bits through a processor per track input no host binds, and issue #965 retired it.
    SixtyFourTrackGainPanRing,
    /// The gain/pan row fed by a live producer on another core (issue #938).
    ///
    /// The plan is [`Self::SixtyFourTrackGainPanRing`]'s: the same fixture, strip edit, compile
    /// and bind. Only the feed differs. The ring row's driver lends one frozen block per track,
    /// every block, so its input stays warm in the render core's cache. In a product session a
    /// producer on another core (the browser's PCM pump in a Worker, a mobile decode thread)
    /// writes every source's ring, and render reads blocks that core wrote. This row measures that
    /// read: its sixty-four track inputs are claimed by the production source set
    /// (`source::prepare_session_graph_source_set`), one host-region ring per track
    /// ([`LIVE_RING_FRAMES`] deep), and one producer thread submits each track's frozen block into
    /// its ring through `HostChunkProvider::submit`, as a host submits decoded PCM. So the row
    /// renders the ring row's bits, and the in-run and test digests say so.
    ///
    /// The producer is started by [`SessionRuntime::start_live_producer`] before any block is
    /// rendered and joined by [`SessionRuntime::finish_live_producer`] after the last one. A
    /// driver calls [`SessionRuntime::await_live_feed`] outside its clock before each block, so
    /// every ring is at least [`LIVE_LEAD_BLOCKS`] blocks ahead of render when the block starts
    /// and no block underruns; the report's underrun count says whether one did.
    ///
    /// Not in [`DRIVER_FED_WORKLOADS`]: see [`LIVE_FED_WORKLOADS`].
    SixtyFourTrackGainPanLive,
    /// The mono qualification session: sixty-four collapse-eligible strips, rendered as written.
    ///
    /// The same strip, the same coefficients and the same input as `sixty_four_track_console`,
    /// from a fixture whose every track satisfies the channel-symmetry witness' two structural
    /// terms. This row arms mono collapse, while [`Self::SixtyFourTrackConsoleMonoDual`] forces
    /// it off on the same prepared session. Their current digest equality checks that collapsing
    /// the eligible cohorts preserves the dual-channel output.
    SixtyFourTrackConsoleMono,
    /// The mono row's control arm: the identical session with the collapse forced off.
    ///
    /// See [`Self::SixtyFourTrackConsoleMono`]. The arms share the fixture, parameters and sources;
    /// this arm renders both channels independently.
    SixtyFourTrackConsoleMonoDual,
    /// The mixed-cohort row: thirty-two collapse-eligible tracks and thirty-two that are not,
    /// alternating in track order. Cohort pooling separates the mono and stereo classes.
    ///
    /// Derived in code from the mono fixture by putting `right_source_channel = 1` back on the odd
    /// tracks -- undoing, on half the tracks, the one edit the generator made to the source
    /// mapping. Those tracks then read two different source channels, which clears the witness'
    /// `SOURCE` term, and they render genuinely different left and right samples rather than
    /// merely declaring that they might.
    ///
    /// It checks that the structural witness reaches the cohort planner: half the resulting
    /// cohorts are wholly eligible and half are wholly ineligible. The interleaved track order
    /// also makes the route fold decline when pooling would change reduction order.
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
    /// `builtins_compiler::prepare_selected_session_builtins_with_live_controls`, the entry
    /// `host_core::prepare_host_runtime_with_selected_meters` calls for that boot, with the
    /// product's `Concurrent` delivery. No live-control channel is attached.
    ///
    /// The meters are the only difference from the standing row: both plans run the same 48
    /// bank-chain stages per block, and this crate's pair test pins both counts. So this row minus
    /// the standing row is the meters' cost.
    ///
    /// It is the row the observer path can move. Every other session row renders with no
    /// observer, and the `console_meters` arm binds all-metric meters at a four-block window
    /// through the concurrent entry, which is not what a browser binds. This is the configuration
    /// issue #943 (one sample-peak pass per bank, at the resident final lane) moves.
    ///
    /// Meters observe and never change signal flow, so this row renders the standing console row's
    /// bits: the run asserts it
    /// before it emits either record, and the aggregate validator pins it. Its snapshots are
    /// consumed after every block, outside the clock, as a host holding the meter lease consumes
    /// them.
    ///
    /// Not in [`WORKLOADS`]: see [`METERED_WORKLOADS`].
    SixtyFourTrackConsoleMetered,
    /// Buses and sends (issue #1227): the standing console's 64 tracks feeding eight processed
    /// submix strips, with two effect returns and two sends per track.
    ///
    /// Track `chNN`'s main route feeds `bus-<NN div 8>`; every track also sends `pre_fader` into
    /// `fx-a` (a delay) and `post_fader` into `fx-b` (an EQ); each of the ten submixes carries
    /// the standing strip plus one insert and returns `post_pan` to the output. No route or strip
    /// is muted, so every send is active, and there is no VCA (a VCA is baked into the faders at
    /// preparation and adds no render work). The tracks are the standing console row's, as
    /// written, so this row and [`Self::SixtyFourTrackConsole`] differ by the buses and sends
    /// alone: both are prepared at [`PlanConfig::BASELINE`] and fed through a source set, the
    /// shape the C ABI prepares today (no live controls, no meters).
    ///
    /// The committed sends fixture as written (`scripts/derive-sends-console-fixture.py`), so the
    /// browser arm can boot the same document.
    ///
    /// Not in [`WORKLOADS`]: see [`BUS_SEND_WORKLOADS`].
    SixtyFourTrackConsoleSends,
    /// The standing strip at ten tracks (issue #1085): one full eight-lane bank and a remainder of
    /// two.
    ///
    /// The first ten tracks of the standing fixture, renamed and routed as the ragged nine-track
    /// row's are (`synthesise_tracks`). With [`Self::NineTrackRaggedStrip`] (remainder one),
    /// [`Self::ThirteenTrackRaggedStrip`] (remainder five), [`Self::SixteenTrackStrip`] (two full
    /// banks) and [`Self::SixtyFourTrackConsole`] (eight), it is the strip-at-N row set the console
    /// strip's padded banks exercise. Console remainders stay banked with inactive padding lanes;
    /// the rows state no derived floor.
    ///
    /// Not in [`WORKLOADS`]: see [`CONSOLE_STRIP_WORKLOADS`].
    TenTrackRaggedStrip,
    /// The standing strip at thirteen tracks: one full eight-lane bank and a remainder of five.
    ///
    /// See [`Self::TenTrackRaggedStrip`].
    ThirteenTrackRaggedStrip,
    /// The standing strip at sixteen tracks: two full eight-lane banks and no remainder.
    ///
    /// See [`Self::TenTrackRaggedStrip`].
    SixteenTrackStrip,
    /// The app shape (issue #1085): what `misofm/app` compiles, on the standing console.
    ///
    /// Every track carries the pre-insert console's EQ -> compressor, with no limiter, and both
    /// effects are bypassed on every track whose index is 2 mod 3 (21 of 64). A bypassed lane
    /// stays in its bank and runs the wet path before selecting its bypass output (decision 12).
    ///
    /// The committed app fixture as written (`scripts/derive-app-console-fixture.py`), so the
    /// browser arm boots the same document. Its record states the bypass pattern it observed in
    /// the compiled session ([`SessionRuntime::bypass_census`]).
    ///
    /// Not in [`WORKLOADS`]: see [`CONSOLE_STRIP_WORKLOADS`].
    SixtyFourTrackAppShape,
    /// Sparse activity (issue #1085): the standing console with every odd track fed silence.
    ///
    /// The plan is the unmodified sixty-four-track console, as on the idle row; only the input
    /// differs. Even tracks read the tone and odd tracks exact zeros, so every bank holds both,
    /// and no bank is wholly silent. That is the case the silent fast path cannot serve: it is
    /// decided per bank (`block_is_positive_zero` over every lane), so one active track keeps its
    /// bank-mates processing. The row is read against [`Self::SixtyFourTrackConsole`] (every track
    /// active) and [`Self::SixtyFourTrackIdle`] (every track silent).
    ///
    /// No settling is needed, unlike the idle row's: a silent lane starts at zero state and its
    /// input is exact zeros, so it never leaves zero, and its active bank-mates are the console
    /// row's own.
    ///
    /// Not in [`WORKLOADS`]: see [`CONSOLE_STRIP_WORKLOADS`].
    SixtyFourTrackConsoleSparse,
    /// Toggling activity (issue #1783): the standing console with every odd track flipping between
    /// exact zeros and the tone every [`TOGGLE_HALF_PERIOD_BLOCKS`] blocks
    /// ([`InputSignal::OddTracksToggle`]).
    ///
    /// The plan is the unmodified standing console, as on the sparse row; only the input differs.
    /// Read against [`Self::SixtyFourTrackConsoleSparse`] (the odd tracks always silent, the floor)
    /// and [`Self::SixtyFourTrackConsole`] (always active, the ceiling). A lane that has carried
    /// the tone needs about 620 blocks to rest (#1784), so no toggling lane rests within a half
    /// period: the row measures the activity test under flipping input, not a lane cycling
    /// through rest, and it cannot thrash a packer (#1782).
    ///
    /// Not in [`WORKLOADS`]: see [`PACKING_WORKLOADS`].
    SixtyFourTrackConsoleToggle,
    /// The standing strip at sixteen tracks with every console slot bypassed on every track
    /// (issue #2129 gate 6): [`Self::SixteenTrackStrip`]'s two full eight-lane banks per slot,
    /// each of whose lanes is settled bypassed, so every bank skips its wet path and renders the
    /// shunt's latency-matched dry words. Read against [`Self::SixteenTrackStrip`].
    ///
    /// Not in [`WORKLOADS`]: see [`BYPASS_WORKLOADS`].
    SixteenTrackStripBypassed,
    /// One active lane per bank (issue #1783): the standing console with one track per bank fed
    /// the tone and every other track exact zeros ([`InputSignal::OneTrackPerBank`]).
    ///
    /// The best case for packing (#1782): every bank pays a whole bank's work for one lane. Read
    /// against [`Self::SixtyFourTrackConsole`] and [`Self::SixtyFourTrackIdle`]. Like the sparse
    /// row it needs no settling: a silent lane starts at zero state and stays there.
    ///
    /// Not in [`WORKLOADS`]: see [`PACKING_WORKLOADS`].
    SixtyFourTrackConsoleOnePerBank,
    /// The console app's C1 shape (issue #2132): `tracks` tracks feeding four groups, two aux
    /// returns and a master, every strip carrying `gate, eq, comp | drive`, in `state`.
    ///
    /// Not in any standing list: see [`BANK_REPORT_WORKLOADS`].
    C1Console {
        /// The track count.
        tracks: C1Tracks,
        /// What the console slots are doing.
        state: C1State,
    },
    /// The irregular graph (issue #2132), every console slot active.
    ///
    /// Not in any standing list: see [`BANK_REPORT_WORKLOADS`].
    IrregularGraph,
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
    /// Every row's track inputs are claimed by a prepared source set, the feed every host binds
    /// (issue #965). [`SourceFeed::LiveProducer`] for the live row (issue #938), whose set is the
    /// production one over rings a producer thread fills, and [`SourceFeed::PlayedPlanes`] for
    /// every other row.
    #[must_use]
    pub const fn source_feed(self) -> SourceFeed {
        match self {
            Self::SixtyFourTrackGainPanLive => SourceFeed::LiveProducer,
            _ => SourceFeed::PlayedPlanes,
        }
    }
}

/// What a row's track inputs carry: the tone, exact zeros, or one per track (issue #1085).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputSignal {
    /// Every track reads the tone ([`source_block`]).
    Tone,
    /// Every track reads exact zeros.
    Silence,
    /// Even-indexed tracks read the tone and odd-indexed tracks exact zeros, so every bank of the
    /// standing console holds both at either lane width.
    OddTracksSilent,
    /// One track per bank reads the tone and every other track exact zeros (issue #1783): the
    /// track whose index is a multiple of this build's lane width ([`Backend::current`]), so every
    /// bank of the standing console holds exactly one active lane.
    ///
    /// Keyed on the lane width because "one lane per bank" cannot be width-independent: a frozen
    /// `% 8` pattern leaves every second four-lane bank wholly silent, which is the whole-bank skip
    /// (#1107), not the packing case (#1782) this row is for. The record names the backend it ran
    /// at, so the pattern it fed is stated. A plan dispatched at a width other than the build's
    /// (`SessionRuntime::build_with_dispatch`) is not fed this pattern by any row.
    OneTrackPerBank,
    /// Even tracks read the tone; odd tracks alternate between exact zeros and the tone every
    /// [`TOGGLE_HALF_PERIOD_BLOCKS`] blocks, starting silent (issue #1783).
    ///
    /// Odd tracks only, so no bank flips as a whole (that is the whole-bank skip's entry and exit,
    /// #1107) and every bank always holds an active lane.
    OddTracksToggle,
}

/// The toggle row's half period, in blocks (issue #1783): odd tracks hold silence for this many
/// blocks, then the tone for this many, and so on.
///
/// "A few blocks": longer than any per-block activity scan, and far shorter than every rest bound
/// on the standing console -- a compressor that has carried the tone needs about 620 blocks to
/// rest (#1784: 79,200 samples). So no toggling lane ever rests, and the row measures the activity
/// test under flipping input; it cannot cycle a lane through rest, and so cannot thrash a packer.
/// The period is in the row's input name, so a changed period cannot share a record name.
pub const TOGGLE_HALF_PERIOD_BLOCKS: u64 = 8;

impl InputSignal {
    /// The record-side name of this input.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Tone => "tone",
            Self::Silence => "silence",
            Self::OddTracksSilent => "odd_tracks_silent",
            Self::OneTrackPerBank => "one_track_per_bank",
            Self::OddTracksToggle => "odd_tracks_toggle_8_blocks",
        }
    }

    /// Whether the track at `track` (its index in the model's track order) reads exact zeros in
    /// the first block.
    #[must_use]
    pub const fn silent(self, track: usize) -> bool {
        self.silent_in_block(track, 0)
    }

    /// Whether the track at `track` reads exact zeros in block `block` (counted in quanta of
    /// source time). Only [`Self::OddTracksToggle`] depends on the block.
    #[must_use]
    pub const fn silent_in_block(self, track: usize, block: u64) -> bool {
        match self {
            Self::Tone => false,
            Self::Silence => true,
            Self::OddTracksSilent => track % 2 == 1,
            Self::OneTrackPerBank => !track.is_multiple_of(Backend::current().width()),
            Self::OddTracksToggle => {
                track % 2 == 1 && (block / TOGGLE_HALF_PERIOD_BLOCKS).is_multiple_of(2)
            }
        }
    }

    /// Whether any track's input changes from block to block.
    #[must_use]
    pub const fn toggles(self) -> bool {
        matches!(self, Self::OddTracksToggle)
    }
}

/// How a row's track inputs reach the graph (issue #928).
///
/// Both feeds are source sets: issue #965 retired the bound feed (a `FrozenGraphSource` processor
/// per track input, one dispatched copy per track per block), which no host binds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceFeed {
    /// The inputs are claimed by a prepared source set whose driver copies each claim's block on
    /// request and lends its played planes in place: the production feed, over the same frozen
    /// blocks.
    PlayedPlanes,
    /// The inputs are claimed by the production source set over one host-region ring per track,
    /// which a producer thread fills with the same frozen blocks (issue #938).
    LiveProducer,
}

impl SourceFeed {
    /// The record-side name of this feed.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PlayedPlanes => "played_planes",
            Self::LiveProducer => "live_producer",
        }
    }
}

/// The standing session workloads, in the order their records are emitted.
///
/// Two rows have left it: `sixty_four_track_plumbing_only`, a plan compiled without builtins,
/// which no host builds (issue #956), and `sixty_four_track_gain_pan_only`, the bound-feed twin of
/// [`Workload::SixtyFourTrackGainPanRing`], fed through a processor per track input no host binds
/// (issue #965).
pub const WORKLOADS: [Workload; 14] = [
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
    Workload::SixtyFourTrackConsoleMono,
    Workload::SixtyFourTrackConsoleMonoDual,
    Workload::SixtyFourTrackConsoleHalfMono,
];

/// The session row the native bench emits after [`WORKLOADS`]: the gain/pan row, the native
/// pure-path target (issues #928 and #956).
///
/// Its own array because it is emitted in that place; since issue #965 every row's track inputs
/// are claimed by a prepared source set, so the feed no longer sets it apart. The row is banked,
/// so the shape tests under `tests/` that state a law of every banked row -- one folded route per
/// track, and the folded master's bits -- iterate it beside [`WORKLOADS`].
pub const DRIVER_FED_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackGainPanRing];

/// The session row the native bench emits after [`DRIVER_FED_WORKLOADS`]: the gain/pan row fed by
/// a live producer on another core (issue #938).
///
/// Its own array, not [`DRIVER_FED_WORKLOADS`], because a live row renders its input only while
/// its producer runs ([`SessionRuntime::start_live_producer`]). The shape tests under `tests/` that
/// iterate the driver-fed rows render without one; this row's plan is the ring row's, so what
/// they state of that plan they state of this one, and this crate's own test of the row checks
/// its bits against the ring row's with the producer live.
pub const LIVE_FED_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackGainPanLive];

/// The session rows the native bench emits after [`LIVE_FED_WORKLOADS`]: the rows prepared with
/// the default web boot's meter set (issue #881).
///
/// The census and shape tests under `tests/` iterate [`WORKLOADS`] only. What this row shares with
/// the standing console row -- its bits, its bank memberships, its stages and its folds -- is
/// asserted beside it by this crate's own test of the pair.
pub const METERED_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackConsoleMetered];

/// The bus-and-send rows the native bench emits after [`METERED_WORKLOADS`] (issue #1227).
///
/// The shape tests under `tests/` that state a law of every banked standing row (one folded route
/// per track) do not iterate it: its tracks feed submixes, not the output. Its plan facts -- one
/// route transform per route, a reduction per submix input and at the output, no compensated,
/// muted or follow-zeroed route -- are derived from the fixture by this crate's own test of the
/// row.
pub const BUS_SEND_WORKLOADS: [Workload; 1] = [Workload::SixtyFourTrackConsoleSends];

/// The console-strip rows the native bench emits after [`BUS_SEND_WORKLOADS`] (issue #1085).
///
/// The shapes the console strip moves, frozen before any console slice lands so the same rows can
/// be measured before and after: the strip at ten, thirteen and sixteen tracks (with the existing
/// nine- and sixty-four-track rows, the strip at N in {9, 10, 13, 16, 64}), the app shape, and
/// sparse activity.
pub const CONSOLE_STRIP_WORKLOADS: [Workload; 5] = [
    Workload::TenTrackRaggedStrip,
    Workload::ThirteenTrackRaggedStrip,
    Workload::SixteenTrackStrip,
    Workload::SixtyFourTrackAppShape,
    Workload::SixtyFourTrackConsoleSparse,
];

/// The packing rows the native bench emits after [`CONSOLE_STRIP_WORKLOADS`] (issue #1783): the
/// standing console with toggling activity and with one active lane per bank, the two inputs
/// packing (#1782) is read on.
///
/// A separate array, so the console-strip rows #1085 froze for before/after reads stay as they
/// were.
pub const PACKING_WORKLOADS: [Workload; 2] = [
    Workload::SixtyFourTrackConsoleToggle,
    Workload::SixtyFourTrackConsoleOnePerBank,
];

/// The bypass-skip row the native bench emits after [`PACKING_WORKLOADS`] (issue #2129 gate 6):
/// the sixteen-track strip with every console slot bypassed, read against
/// [`Workload::SixteenTrackStrip`]. Descriptive; it states no floor.
pub const BYPASS_WORKLOADS: [Workload; 1] = [Workload::SixteenTrackStripBypassed];

/// The bank-utilisation rows (issue #2132): the C1 shape at 16, 32 and 64 tracks in each
/// [`C1State`], then the irregular graph. `bench console --bank-report` measures them; no standing
/// record carries them, so the standing record set and its validator are unchanged.
pub const BANK_REPORT_WORKLOADS: [Workload; 17] = {
    const fn c1(tracks: C1Tracks, state: C1State) -> Workload {
        Workload::C1Console { tracks, state }
    }
    use C1State::{AllActive, Load, Reverb, Sparse, Toggle, Typical};
    use C1Tracks::{Sixteen, SixtyFour, ThirtyTwo};
    [
        c1(Sixteen, Load),
        c1(Sixteen, Typical),
        c1(Sixteen, AllActive),
        c1(Sixteen, Sparse),
        c1(Sixteen, Toggle),
        c1(Sixteen, Reverb),
        c1(ThirtyTwo, Load),
        c1(ThirtyTwo, Typical),
        c1(ThirtyTwo, AllActive),
        c1(ThirtyTwo, Sparse),
        c1(ThirtyTwo, Toggle),
        c1(SixtyFour, Load),
        c1(SixtyFour, Typical),
        c1(SixtyFour, AllActive),
        c1(SixtyFour, Sparse),
        c1(SixtyFour, Toggle),
        Workload::IrregularGraph,
    ]
};

/// The rows whose bank counters the four-lane wasm guest reads (issue #2143): the 16-track C1
/// shape in [`C1State::Load`], [`C1State::Typical`], [`C1State::AllActive`] and
/// [`C1State::Sparse`], in that order. One list for both legs: `wasm-gate-guest` renders it under
/// wasmtime and `wasm-gates` names its rows from it.
pub const W4_COUNTER_WORKLOADS: [Workload; 4] = {
    const fn c1(state: C1State) -> Workload {
        Workload::C1Console {
            tracks: C1Tracks::Sixteen,
            state,
        }
    }
    [
        c1(C1State::Load),
        c1(C1State::Typical),
        c1(C1State::AllActive),
        c1(C1State::Sparse),
    ]
};

/// The blocks [`bank_counters`] renders per row (issue #2143).
pub const W4_COUNTER_BLOCKS: u64 = 256;

/// The counters [`bank_counters`] reads, in its array's order (issue #2143).
pub const BANK_COUNTER_NAMES: [&str; 7] = [
    "banks",
    "mergeable_partial_pairs",
    "bank_slot_runs_total",
    "bank_slot_runs_track_group",
    "silent_skips",
    "bypass_skips",
    "pack_groups",
];

/// One row's bank counters after `blocks` rendered blocks at this build's width (issue #2143), in
/// [`BANK_COUNTER_NAMES`] order:
///
/// 0. `banks` and 1. `mergeable_partial_pairs`: the compile's [`graph_compiler::BankUtilisation`];
/// 2. `bank_slot_runs_total`: every effect bank unit's runs (`graph::test_only_bank_slot_runs`);
/// 3. `bank_slot_runs_track_group`: the runs of the rows (pack groups, or units in none) that hold
///    a track's lane, so `2 - 3` is the submixes' runs;
/// 4. `silent_skips`: the whole-chain skips of the same rows, in unit-blocks;
/// 5. `bypass_skips`: the slot-blocks the same rows' banks skipped their wet path because every
///    lane was settled bypassed (issue #2129); `2` already leaves them out;
/// 6. `pack_groups`: the rows that are pack groups (a group spans two units or more).
///
/// Built at [`Backend::current()`], so it describes a four-lane plan only on a four-lane build (the
/// `simd128` guest, or NEON): an eight-lane build cannot bank C1's console at four lanes
/// (`console.slot.foreign_width`, #1694).
///
/// # Panics
///
/// As [`SessionRuntime::build`], and if a block fails to render.
#[must_use]
pub fn bank_counters(workload: Workload, blocks: u64) -> [u64; 7] {
    let model = console_model(workload);
    let tracks: BTreeSet<&str> = model.tracks.iter().map(|track| track.id.as_str()).collect();
    let mut runtime = SessionRuntime::new(workload);
    for block in 0..blocks {
        runtime.before_block(block);
        runtime
            .render(block)
            .unwrap_or_else(|_| panic!("{}: block {block} rendered", workload.kind()));
    }
    let utilisation = runtime.utilisation;
    let track_units: BTreeSet<u32> = graph::bank_effect_lanes(&mut runtime.plan)
        .unwrap_or_default()
        .into_iter()
        .filter(|(strip, _)| tracks.contains(strip.as_ref()))
        .filter_map(|(_, lane)| match lane.location {
            graph::GraphEffectSite::Lane(at) => Some(at.unit),
            graph::GraphEffectSite::Node(_) => None,
        })
        .collect();
    let rows = graph::test_only_bank_slot_runs(&mut runtime.plan);
    let holds_a_track = |row: &graph::TestOnlyBankSlotRuns| {
        track_units
            .range(row.first_unit..=row.last_unit)
            .next()
            .is_some()
    };
    [
        u64::from(utilisation.banks),
        u64::from(utilisation.mergeable_partial_pairs),
        rows.iter().map(|row| row.runs).sum(),
        rows.iter()
            .filter(|row| holds_a_track(row))
            .map(|row| row.runs)
            .sum(),
        rows.iter().map(|row| row.skips).sum(),
        rows.iter().map(|row| row.bypass_skips).sum(),
        rows.iter()
            .filter(|row| row.last_unit > row.first_unit)
            .count() as u64,
    ]
}

/// Every session row the native bench emits, in emission order: [`WORKLOADS`], then
/// [`DRIVER_FED_WORKLOADS`], then [`LIVE_FED_WORKLOADS`], then [`METERED_WORKLOADS`], then
/// [`BUS_SEND_WORKLOADS`], then [`CONSOLE_STRIP_WORKLOADS`], then [`PACKING_WORKLOADS`], then
/// [`BYPASS_WORKLOADS`].
pub fn native_session_rows() -> impl Iterator<Item = Workload> {
    WORKLOADS
        .into_iter()
        .chain(DRIVER_FED_WORKLOADS)
        .chain(LIVE_FED_WORKLOADS)
        .chain(METERED_WORKLOADS)
        .chain(BUS_SEND_WORKLOADS)
        .chain(CONSOLE_STRIP_WORKLOADS)
        .chain(PACKING_WORKLOADS)
        .chain(BYPASS_WORKLOADS)
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

/// Retains the `pre_insert` console slots `keep` accepts, and the `post_insert` slots only when
/// `keep_post_insert` is set, in the session's declarations and in every track's entries together.
///
/// Console slots are session-level (decision 12): a slot is declared once and every track carries
/// an entry for it, so a decomposition row that narrows the strip edits both. The standing
/// fixture's `pre_insert` is a *two-slot* chain (EQ, compressor), so a row that wants the EQ alone
/// drops one slot out of a chain rather than emptying a section. Matching on the contract's effect
/// id rather than the slot id means a fixture that renamed a slot cannot silently turn a
/// decomposition row into a row that measures nothing.
fn retain_console(
    model: &mut SessionModel,
    keep: impl Fn(&session::ConsoleSlot) -> bool,
    keep_post_insert: bool,
) {
    model.console.pre_insert.retain(|slot| keep(slot));
    if !keep_post_insert {
        model.console.post_insert.clear();
    }
    let kept: Vec<&session::StableId> = model.console.slots().map(|slot| &slot.slot).collect();
    for track in &mut model.tracks {
        track.console.retain(|entry| kept.contains(&&entry.slot));
    }
}

/// Whether a console slot is the native effect `effect_id`.
fn is_native(slot: &session::ConsoleSlot, effect_id: &str) -> bool {
    matches!(
        &slot.identity,
        session::EffectIdentity::Native { effect_id: id } if id.as_str() == effect_id
    )
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
            Self::SixtyFourTrackGainPanRing => "sixty_four_track_gain_pan_ring",
            Self::SixtyFourTrackGainPanLive => "sixty_four_track_gain_pan_live",
            Self::SixtyFourTrackConsoleMono => "sixty_four_track_console_mono",
            Self::SixtyFourTrackConsoleMonoDual => "sixty_four_track_console_mono_dual",
            Self::SixtyFourTrackConsoleHalfMono => "sixty_four_track_console_half_mono",
            Self::SixtyFourTrackConsoleMetered => "sixty_four_track_console_metered",
            Self::SixtyFourTrackConsoleSends => "sixty_four_track_console_sends",
            Self::TenTrackRaggedStrip => "ten_track_ragged_strip",
            Self::ThirteenTrackRaggedStrip => "thirteen_track_ragged_strip",
            Self::SixteenTrackStrip => "sixteen_track_strip",
            Self::SixtyFourTrackAppShape => "sixty_four_track_app_shape",
            Self::SixtyFourTrackConsoleSparse => "sixty_four_track_console_sparse",
            Self::SixtyFourTrackConsoleToggle => "sixty_four_track_console_toggle",
            Self::SixtyFourTrackConsoleOnePerBank => "sixty_four_track_console_one_per_bank",
            Self::SixteenTrackStripBypassed => "sixteen_track_strip_bypassed",
            Self::C1Console { tracks, state } => {
                use C1State::{AllActive, Load, Reverb, Sparse, Toggle, Typical};
                use C1Tracks::{Sixteen, SixtyFour, ThirtyTwo};
                match (tracks, state) {
                    (Sixteen, Load) => "c1_console_16_load",
                    (Sixteen, Typical) => "c1_console_16_typical",
                    (Sixteen, AllActive) => "c1_console_16_all_active",
                    (Sixteen, Sparse) => "c1_console_16_sparse",
                    (Sixteen, Toggle) => "c1_console_16_toggle",
                    (Sixteen, Reverb) => "c1_console_16_reverb",
                    (ThirtyTwo, Load) => "c1_console_32_load",
                    (ThirtyTwo, Typical) => "c1_console_32_typical",
                    (ThirtyTwo, AllActive) => "c1_console_32_all_active",
                    (ThirtyTwo, Sparse) => "c1_console_32_sparse",
                    (ThirtyTwo, Toggle) => "c1_console_32_toggle",
                    (ThirtyTwo, Reverb) => "c1_console_32_reverb",
                    (SixtyFour, Load) => "c1_console_64_load",
                    (SixtyFour, Typical) => "c1_console_64_typical",
                    (SixtyFour, AllActive) => "c1_console_64_all_active",
                    (SixtyFour, Sparse) => "c1_console_64_sparse",
                    (SixtyFour, Toggle) => "c1_console_64_toggle",
                    (SixtyFour, Reverb) => "c1_console_64_reverb",
                }
            }
            Self::IrregularGraph => "irregular_graph",
        }
    }
    /// How many console tracks this workload renders.
    pub const fn tracks(self) -> u32 {
        match self {
            Self::NineTrackBaseline | Self::NineTrackRaggedStrip => 9,
            Self::TenTrackRaggedStrip => 10,
            Self::ThirteenTrackRaggedStrip => 13,
            Self::SixteenTrackStrip | Self::SixteenTrackStripBypassed => 16,
            Self::OneTwentyEightTrackStretch => 128,
            Self::C1Console { tracks, .. } => tracks.count(),
            Self::IrregularGraph => 24,
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
            Self::SixtyFourTrackAppShape => "fixtures/session/v1/console-sixty-four-track-app.json",
            Self::SixtyFourTrackConsoleSends => {
                "fixtures/session/v1/console-sixty-four-track-sends.json"
            }
            Self::C1Console {
                tracks: C1Tracks::Sixteen,
                ..
            } => "fixtures/session/v1/console-c1-16.json",
            Self::C1Console {
                tracks: C1Tracks::ThirtyTwo,
                ..
            } => "fixtures/session/v1/console-c1-32.json",
            Self::C1Console {
                tracks: C1Tracks::SixtyFour,
                ..
            } => "fixtures/session/v1/console-c1-64.json",
            Self::IrregularGraph => "fixtures/session/v1/irregular-graph.json",
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
                // digest equality rests on while collapse is armed on only one arm.
                | Self::SixtyFourTrackConsoleMono
                | Self::SixtyFourTrackConsoleMonoDual
                // The metered row renders the standing fixture as written. Its meters are a
                // preparation facility, not an edit to the session.
                | Self::SixtyFourTrackConsoleMetered
                // The app shape is a committed fixture, rendered as written. The sparse row, like
                // the idle row, is reported as derived: its session is the standing one, but what
                // it renders is fed in code.
                | Self::SixtyFourTrackAppShape
                // The bus-and-send row is a committed fixture, rendered as written.
                | Self::SixtyFourTrackConsoleSends
                // C1 at load and the irregular graph are committed fixtures, rendered as written.
                | Self::C1Console {
                    state: C1State::Load,
                    ..
                }
                | Self::IrregularGraph
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
            Self::SixtyFourTrackGainPanRing | Self::SixtyFourTrackGainPanLive => Strip::GainPan,
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
            Self::SixtyFourTrackConsoleLegacy
            | Self::SixtyFourTrackEqCompSimd1
            | Self::SixtyFourTrackAppShape => "eq+compressor",
            Self::SixtyFourTrackGainPanRing | Self::SixtyFourTrackGainPanLive => "gain+pan",
            Self::C1Console { .. } | Self::IrregularGraph => "gate+eq+compressor+drive",
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
    /// The vocabulary is `section:slot[+slot]`, sections in strip order, joined by `,`.
    /// `builtins` is the row that carries no rack effect at all.
    ///
    /// # Layout-neutral since issue #1085
    ///
    /// The sections are named in the console vocabulary of decision 12
    /// (`docs/rulings/engine-footprint-2026-09-29.md`). The migration mapped `simd1` to
    /// `pre_insert`, `dynamic` to `inserts`, and `simd2` to `post_insert`, preserving each row's
    /// placement and record spelling. The one row whose layout the
    /// console migration moved (#1093) is the app shape, whose EQ and compressor went from
    /// `inserts` to `pre_insert`; the validator accepts both spellings for that row alone, so S0's
    /// baseline records and a rerun on the console model are held to the same rule.
    pub const fn strip_layout(self) -> &'static str {
        match self {
            Self::NineTrackBaseline | Self::SixtyFourTrackEqOnly => "pre_insert:eq",
            Self::SixtyFourTrackCompressorOnly => "pre_insert:compressor",
            Self::SixtyFourTrackBuiltinsOnly
            | Self::SixtyFourTrackDispatchOnly
            | Self::SixtyFourTrackGainPanRing
            | Self::SixtyFourTrackGainPanLive => "builtins",
            // The retired layout: two one-slot chains, one per rack.
            Self::SixtyFourTrackConsoleLegacy => "pre_insert:eq,inserts:compressor",
            // The chain-shape row: one two-slot chain, no limiter.
            Self::SixtyFourTrackEqCompSimd1 => "pre_insert:eq+compressor",
            // The app shape: EQ -> compressor as the session's two `pre_insert` slots (#1093), no
            // limiter. Before the console migration it was `inserts:eq+compressor`.
            Self::SixtyFourTrackAppShape => "pre_insert:eq+compressor",
            // The console app's C1 strip (issue #2132).
            Self::C1Console { .. } | Self::IrregularGraph => {
                "pre_insert:gate+eq+compressor,post_insert:drive"
            }
            // The intended production layout.
            _ => "pre_insert:eq+compressor,post_insert:limiter",
        }
    }
    /// What every track's source binding writes into the graph, named in the record.
    pub const fn input_signal(self) -> &'static str {
        self.input().name()
    }
    /// Which tracks this row feeds the tone and which exact zeros.
    pub const fn input(self) -> InputSignal {
        match self {
            Self::SixtyFourTrackIdle => InputSignal::Silence,
            Self::SixtyFourTrackConsoleSparse => InputSignal::OddTracksSilent,
            Self::SixtyFourTrackConsoleToggle => InputSignal::OddTracksToggle,
            Self::SixtyFourTrackConsoleOnePerBank => InputSignal::OneTrackPerBank,
            Self::C1Console {
                state: C1State::Sparse,
                ..
            } => InputSignal::OddTracksSilent,
            _ => InputSignal::Tone,
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
    // The console edits are session-level (decision 12). Every derived row but `HalfMono` drops
    // the `post_insert` limiter; the `pre_insert` chain (EQ, compressor) is narrowed or emptied
    // as the row says, and `LimiterRemoved` leaves it exactly as the fixture wrote it.
    match strip {
        Strip::AsWritten | Strip::HalfMono => {}
        Strip::EqOnly => retain_console(model, |slot| is_native(slot, "miso.parametric-eq"), false),
        Strip::CompressorOnly => {
            retain_console(model, |slot| is_native(slot, "miso.compressor"), false)
        }
        Strip::LimiterRemoved => retain_console(model, |_| true, false),
        Strip::BuiltinsOnly | Strip::Identity | Strip::GainPan => {
            retain_console(model, |_| false, false);
        }
    }
    for (index, track) in model.tracks.iter_mut().enumerate() {
        match strip {
            Strip::AsWritten | Strip::EqOnly | Strip::CompressorOnly | Strip::LimiterRemoved => {}
            // The racks go and nothing else does.
            Strip::BuiltinsOnly => track.inserts.effects.clear(),
            Strip::Identity | Strip::GainPan => {
                track.inserts.effects.clear();
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
            }
        }
    }
}
/// Which console-side facilities a prepared arm carries.
///
/// The session rows all use [`PlanConfig::BASELINE`], which is what the console benchmark has
/// always measured: no meter streams, no live-control channel, no observation capacity.
/// The #163 item 0d arms differ from it in exactly one field each, so the paired delta between two
/// arms is the cost of that one facility.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlanConfig {
    /// One meter stream per track at the post-matrix tap, as a production console prepares.
    pub meters: bool,
    /// One bounded live-control channel per prepared effect.
    pub control: bool,
    /// Effect observation capacity, and whether its taps are armed.
    pub observation: ObservationArm,
}

/// Which of the product's live lanes an arm attaches (issue #1763), each group a subset of
/// `controls::live_lanes` of the session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneGroup {
    /// The whole product set: every strip's fader/mute and matrix/pan lanes, and the input and
    /// effect lanes `controls::live_lanes` selects.
    Product,
    /// Every strip's fader/mute and matrix/pan lanes only.
    FaderMatrix,
    /// Every strip's fader/mute, matrix/pan and input lanes.
    StripInput,
    /// One lane per effect instance and no strip channel.
    Effects,
}

impl LaneGroup {
    /// Whether the arm passes strip live-control requests (fader/mute and matrix/pan lanes).
    fn strip_requests(self) -> bool {
        self != Self::Effects
    }

    /// The optional lanes the group attaches, given the session's product set.
    fn lanes(self, product: controls::LiveLanes) -> controls::LiveLanes {
        let none = controls::LiveLanes::default();
        match self {
            Self::Product => product,
            Self::FaderMatrix => none,
            Self::StripInput => controls::LiveLanes {
                strip_input: true,
                ..none
            },
            Self::Effects => controls::LiveLanes {
                effects: true,
                ..none
            },
        }
    }
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

/// Every console slot of `model` folded into each track's inserts, in chain order: `pre_insert`
/// ahead of the track's own inserts and `post_insert` after them, each entry an ordinary effect
/// with the slot's ID. The lowered chain, and so the render, is unchanged (#163's placement
/// invariance); only the internal rack its effects sit in moves.
fn fold_console_into_inserts(model: &mut SessionModel) {
    let chains: Vec<Vec<session::Effect>> = model
        .tracks
        .iter()
        .map(|track| {
            let racks = model.lower_track(track);
            let mut chain = racks.pre_insert;
            chain.extend_from_slice(racks.inserts);
            chain.extend(racks.post_insert);
            chain
        })
        .collect();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for (track, chain) in model.tracks.iter_mut().zip(chains) {
        track.console.clear();
        track.inserts.effects = chain;
    }
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
        Workload::SixtyFourTrackAppShape => SIXTY_FOUR_TRACK_APP,
        Workload::SixtyFourTrackConsoleSends => SIXTY_FOUR_TRACK_SENDS,
        Workload::C1Console { tracks, .. } => match tracks {
            C1Tracks::Sixteen => C1_SIXTEEN,
            C1Tracks::ThirtyTwo => C1_THIRTY_TWO,
            C1Tracks::SixtyFour => C1_SIXTY_FOUR,
        },
        Workload::IrregularGraph => IRREGULAR_GRAPH,
        _ => SIXTY_FOUR_TRACK,
    };
    let mut model = parse_session_json(text).expect("frozen console session fixture");
    model.automation.clear();
    if model.tracks.len() != workload.tracks() as usize {
        synthesise_tracks(&mut model, workload.tracks() as usize);
    }
    apply_strip(&mut model, workload.strip());
    if workload == Workload::SixteenTrackStripBypassed {
        // Issue #2129 gate 6: every console slot bypassed on every track.
        for entry in model.tracks.iter_mut().flat_map(|track| &mut track.console) {
            entry.bypass = true;
        }
    }
    if let Workload::C1Console { state, .. } = workload {
        apply_c1_state(&mut model, state);
    }
    assert_eq!(
        model.tracks.len(),
        workload.tracks() as usize,
        "{}: the fixture must carry exactly the declared track count",
        workload.kind()
    );
    model
}

/// Each pack group's ideal bank-slot runs per block (issue #2132): `sum over slots of ceil(A / W)`,
/// `A` the strips of the group whose effect at that slot is not bypassed and whose input is not
/// silent. A track is silent when `input` feeds it exact zeros; a submix when every unmuted route
/// into it comes from a silent strip. Bind-time, off the clock; the strips are read where bind put
/// them, and a move inside a group changes no group's count.
///
/// Also returns the row's **key-class** ideal: the runs the active lanes of each key class need if
/// they were packed together, whatever the pack groups are today. A key class is the compile's
/// bound groups of one dependency level, rack, pool class, leader program and fold target (a group
/// whose members route to different places is a class of its own); its ideal is
/// `sum over slots of ceil(A / W)` over all its groups' active members at once.
fn ideal_bank_slot_runs(
    plan: &mut PreparedRenderPlan,
    model: &SessionModel,
    input: InputSignal,
    report: &graph_compiler::GraphRackBankReport,
) -> (Vec<(u32, u64)>, u64) {
    let mut silent: std::collections::BTreeMap<&str, bool> = model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| (track.id.as_str(), input.silent(index)))
        .collect();
    for submix in &model.submixes {
        silent.insert(submix.id.as_str(), true);
    }
    // Only true -> false moves, so this reaches its fixed point within one pass per submix.
    loop {
        let mut changed = false;
        for submix in &model.submixes {
            let id = submix.id.as_str();
            let heard = model.routes.iter().any(|route| {
                let into = matches!(
                    &route.destination,
                    session::RouteDestination::SubmixInput { submix_id } if submix_id.as_str() == id
                );
                let from = match &route.source {
                    session::RouteSource::Track { track_id, .. } => track_id.as_str(),
                    session::RouteSource::Submix { submix_id, .. } => submix_id.as_str(),
                };
                into && !route.mute && !silent.get(from).copied().unwrap_or(true)
            });
            if heard && silent[id] {
                silent.insert(id, false);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let active = |strip: &str, effect: &str| -> bool {
        if silent.get(strip).copied().unwrap_or(false) {
            return false;
        }
        let (console, inserts) = match model.tracks.iter().find(|track| track.id.as_str() == strip)
        {
            Some(track) => (&track.console, &track.inserts),
            None => match model
                .submixes
                .iter()
                .find(|submix| submix.id.as_str() == strip)
            {
                Some(submix) => (&submix.console, &submix.inserts),
                None => return false,
            },
        };
        console
            .iter()
            .find(|entry| entry.slot.as_str() == effect)
            .map(|entry| !entry.bypass)
            .or_else(|| {
                inserts
                    .effects
                    .iter()
                    .find(|insert| insert.id.as_str() == effect)
                    .map(|insert| !insert.bypass)
            })
            .unwrap_or(false)
    };
    let key_class = key_class_ideal(report, model, &active);
    let lanes = graph::bank_effect_lanes(plan).unwrap_or_default();
    let groups = graph::test_only_bank_slot_runs(plan)
        .into_iter()
        .map(|group| {
            // (slot, active lanes) over the group's units.
            let mut by_slot: std::collections::BTreeMap<u16, u64> = Default::default();
            for (strip, lane) in &lanes {
                let graph::GraphEffectSite::Lane(at) = lane.location else {
                    continue;
                };
                if (group.first_unit..=group.last_unit).contains(&at.unit) {
                    *by_slot.entry(at.slot).or_default() +=
                        u64::from(active(strip, &lane.effect_id));
                }
            }
            let width = u64::from(group.width.max(1));
            let ideal = by_slot.values().map(|count| count.div_ceil(width)).sum();
            (group.first_unit, ideal)
        })
        .collect();
    (groups, key_class)
}

/// The key-class ideal of [`ideal_bank_slot_runs`], from the compile's bank report.
fn key_class_ideal(
    report: &graph_compiler::GraphRackBankReport,
    model: &SessionModel,
    active: &dyn Fn(&str, &str) -> bool,
) -> u64 {
    let mut destinations: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
    for route in &model.routes {
        let from = match &route.source {
            session::RouteSource::Track { track_id, .. } => track_id.as_str(),
            session::RouteSource::Submix { submix_id, .. } => submix_id.as_str(),
        };
        let to = match &route.destination {
            session::RouteDestination::SubmixInput { submix_id } => submix_id.as_str(),
            session::RouteDestination::OutputInput { output_id } => output_id.as_str(),
        };
        destinations.entry(from).or_default().push(to);
    }
    for list in destinations.values_mut() {
        list.sort_unstable();
        list.dedup();
    }
    let none: Vec<&str> = Vec::new();
    let key = |index: usize| {
        let group = &report.plan.groups[index];
        let mut folds = group
            .members
            .iter()
            .flatten()
            .map(|member| destinations.get(member.track_id.as_str()).unwrap_or(&none));
        let first = folds.next();
        // A mixed group is a class of its own: its index makes the key unique.
        let fold = match first {
            Some(first) if folds.all(|fold| fold == first) => format!("{first:?}"),
            _ => format!("mixed#{index}"),
        };
        format!(
            "{}|{:?}|{:?}|{:?}|{fold}",
            group.level, group.rack, group.class, group.program
        )
    };
    let width = report
        .plan
        .groups
        .first()
        .map_or(1, |group| group.members.len().max(1)) as u64;
    let mut by_class: std::collections::BTreeMap<(String, usize), u64> = Default::default();
    for bound in &report.bound_slots {
        let count = bound
            .members
            .iter()
            .filter(|member| active(member.track_id.as_str(), member.effect_id.as_str()))
            .count() as u64;
        *by_class.entry((key(bound.group), bound.slot)).or_default() += count;
    }
    by_class.values().map(|count| count.div_ceil(width)).sum()
}

/// Sets a C1 row's console slots and limiter to its [`C1State`] (issue #2132). The fixture is the
/// load state; every other state un-bypasses slots and nothing else.
fn apply_c1_state(model: &mut SessionModel, state: C1State) {
    let mut seed = C1_TYPICAL_SEED;
    // xorshift64*, in [0, 1): deterministic and the same on every target.
    let mut draw = move || {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        (seed.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / (1_u64 << 53) as f64
    };
    let active = |slot: &str, draw: &mut dyn FnMut() -> f64| match state {
        C1State::Load => false,
        C1State::Typical => {
            let probability = match slot {
                "eq" => 0.75,
                "comp" => 0.5,
                "gate" | "drive" => 0.1,
                other => panic!("C1 has no console slot {other}"),
            };
            draw() < probability
        }
        C1State::AllActive | C1State::Sparse | C1State::Toggle | C1State::Reverb => true,
    };
    let limiter = matches!(
        state,
        C1State::AllActive | C1State::Sparse | C1State::Toggle | C1State::Reverb
    );
    let tracks = model
        .tracks
        .iter_mut()
        .map(|track| (&mut track.console, &mut track.inserts));
    let submixes = model
        .submixes
        .iter_mut()
        .map(|submix| (&mut submix.console, &mut submix.inserts));
    for (console, inserts) in tracks.chain(submixes) {
        for entry in console.iter_mut() {
            entry.bypass = !active(entry.slot.as_str(), &mut draw);
        }
        for effect in &mut inserts.effects {
            if effect.id.as_str() == "limiter" {
                effect.bypass = !limiter;
            }
        }
    }
    if state == C1State::Reverb {
        add_c1_reverb(model);
    }
}

/// [`C1State::Reverb`]'s edit (issue #2147): `miso.reverb` at mix 1 (its parameter 8) as the
/// `aux.b` return's one insert, built from `aux.a`'s delay insert (the same native, dual-mono,
/// sidechain-free declaration), and every track's post-fader send to `aux.b` unmuted at
/// [`C1_REVERB_SEND_DB`].
fn add_c1_reverb(model: &mut SessionModel) {
    let submix = |model: &SessionModel, id: &str| {
        model
            .submixes
            .iter()
            .position(|submix| submix.id.as_str() == id)
            .unwrap_or_else(|| panic!("C1 has a submix {id}"))
    };
    let aux_a = submix(model, "aux.a");
    let mut reverb = model.submixes[aux_a].inserts.effects[0].clone();
    reverb.id = StableId::parse("reverb").expect("stable id");
    reverb.identity = session::EffectIdentity::Native {
        effect_id: StableId::parse("miso.reverb").expect("stable id"),
    };
    reverb.bypass = false;
    reverb.params.truncate(1);
    reverb.params[0].parameter_id = 8;
    reverb.params[0].value = 1.0;
    let aux_b = submix(model, "aux.b");
    assert!(
        model.submixes[aux_b].inserts.effects.is_empty(),
        "C1's aux.b has no insert"
    );
    model.submixes[aux_b].inserts.effects.push(reverb);
    for route in &mut model.routes {
        let into_aux_b = matches!(
            &route.destination,
            session::RouteDestination::SubmixInput { submix_id } if submix_id.as_str() == "aux.b"
        );
        if into_aux_b && matches!(route.source, session::RouteSource::Track { .. }) {
            route.mute = false;
            route.gain_db = C1_REVERB_SEND_DB;
        }
    }
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
/// The request `host_core` builds when a browser boots with its live-control defaults: one
/// [`MeterMetricSet::SAMPLE_PEAK`] meter at [`MeterTap::PostMatrix`] per track of the compiled
/// session's normalized model, handles `index + 1` in that order, a window of
/// [`WEB_METER_BLOCKS`] blocks, no peak hold, peak decay off, a [`WEB_METER_QUEUE_DEPTH`]-deep
/// queue and reset generation zero. Transcribed field for field from
/// `prepare_host_runtime_with_live_controls_policy_and_spectrum` and the web host's
/// `live_control_request`, because this subject does not link either host.
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
    /// Issue #1750: each track's fader/mute and matrix/pan cell writers, for an arm prepared by
    /// [`SessionRuntime::build_with_lane_group`] with strip lanes, and empty on every other arm.
    /// Declared before `plan`, the drop order `PreparedBuiltinsGraphBound` states: producers
    /// before the plan that owns their readers. Never touched inside the clock.
    strip_controls: Vec<TrackControlProducer>,
    /// The session's one `live_values_superseded` count, shared by every live-control channel this
    /// arm attached (decision 15, D15-2).
    superseded: LiveValuesSuperseded,
    /// The optional lanes an arm prepared by [`SessionRuntime::build_with_lane_group`] attached,
    /// read from its channels, and `None` on every other arm.
    live_lanes: Option<controls::LiveLanes>,
    plan: PreparedRenderPlan,
    /// The plan's gate handle: every live value is a latest-target cell, written under a stamp
    /// of this gate and revealed by publishing the stamp's revision on it (decision 15, D15-2).
    gate: PlanRevisionGate,
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
    /// Which tracks the compiled session bypasses, taken at compile time (issue #1085).
    bypass: BypassCensus,
    /// The live row's rings and producer (issue #938), and `None` on every other row.
    live: Option<LiveFeed>,
    /// The compiled plan's bank utilisation (issue #2132), taken from its bank report at compile.
    utilisation: graph_compiler::BankUtilisation,
    /// Each pack group's ideal bank-slot runs per block (issue #2132), keyed by its first unit,
    /// derived at bind from this row's own activity: `sum over slots of ceil(A / W)`, with `A` the
    /// group's strips whose slot is active and whose input is not silent.
    bsr_ideal: Vec<(u32, u64)>,
    /// The row's key-class ideal runs per block (issue #2132): what the active lanes of each key
    /// class would need packed together, whatever today's pack groups are.
    bsr_ideal_key_class: u64,
    /// The live-control channel [`C1State::Toggle`] writes `ch00`'s gate bypass through, and
    /// `None` on every other row.
    toggle: Option<usize>,
}

/// One pack group's bank-slot runs (issue #2132), as [`SessionRuntime::bank_slot_runs`] reads
/// them: the graph's counters beside the row's own ideal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BankGroupRuns {
    /// The group's first runtime unit.
    pub first_unit: u32,
    /// Its last runtime unit, inclusive.
    pub last_unit: u32,
    /// Lanes per unit.
    pub width: u32,
    /// Effect slots per unit.
    pub effect_slots: u32,
    /// Bank-slot runs since bind.
    pub runs: u64,
    /// The row's ideal runs per block for this group.
    pub ideal: u64,
}

/// One transaction of live writes, as a host makes it: stamp the next revision on the plan's gate,
/// run `write` with the stamp, then publish that revision (decision 15, D15-2).
fn write_published<R>(gate: &mut PlanRevisionGate, write: impl FnOnce(LiveStamp<'_>) -> R) -> R {
    let revision = gate.last_published() + 1;
    let stamp = gate.stamp(revision).expect("a legal revision");
    let result = write(stamp);
    gate.publish(revision);
    result
}

/// Which tracks of a compiled session bypass their rack effects, and the pattern they form
/// (issue #1085).
///
/// Read from the compiled session's normalized model, so a record states the bypass the plan was
/// built from rather than the one its workload was meant to carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BypassCensus {
    /// Tracks that carry at least one rack effect and bypass every one of them.
    pub bypassed_tracks: u64,
    /// The pattern the bypassed tracks form.
    pub pattern: BypassPattern,
}

/// The shape of a session's bypass, as a record names it (issue #1085).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassPattern {
    /// No rack effect of any track is bypassed.
    None,
    /// Every rack effect of every track whose index is 2 mod 3 is bypassed, and no other effect
    /// is: the app shape's pattern.
    IndexMod3Is2,
    /// Any other bypass: a partly bypassed track, or another set of bypassed tracks.
    Other,
}

impl BypassPattern {
    /// The record-side name of this pattern.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::IndexMod3Is2 => "index_mod_3_is_2",
            Self::Other => "other",
        }
    }
}

impl BypassCensus {
    /// Takes the census of `model`'s tracks, in their order.
    fn of(model: &SessionModel) -> Self {
        let mut any = false;
        let mut exact = true;
        let mut bypassed_tracks = 0;
        for (index, track) in model.tracks.iter().enumerate() {
            // A track's effects are its console entries and its inserts (decision 12).
            let bypasses = || {
                track
                    .console
                    .iter()
                    .map(|entry| entry.bypass)
                    .chain(track.inserts.effects.iter().map(|effect| effect.bypass))
            };
            let some = bypasses().any(|bypass| bypass);
            let every = bypasses().next().is_some() && bypasses().all(|bypass| bypass);
            any |= some;
            bypassed_tracks += u64::from(every);
            exact &= every == (index % 3 == 2) && some == every;
        }
        let pattern = match (any, exact) {
            (false, _) => BypassPattern::None,
            (true, true) => BypassPattern::IndexMod3Is2,
            (true, false) => BypassPattern::Other,
        };
        Self {
            bypassed_tracks,
            pattern,
        }
    }
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
    /// At a vector width other than the build's own, the session's console is folded into every
    /// track's inserts first (`fold_console_into_inserts`). A build's effect factories bind only
    /// the widths it executes (D4), and since #1098 a console slot never renders per node on a
    /// vector backend, so the console as written is refused there. Folded, the chain is the same
    /// and so is every rendered bit (#163), its effects render per node wherever this build cannot
    /// bank them, exactly as the console did at that width before #1098, and a live control
    /// addresses the effect as an insert at the same chain position.
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
        Self::build_inner(workload, config, dispatch, source, None)
    }

    /// Compiles, prepares and binds one console arm with the live-control lanes a product host
    /// attaches for its session (issue #1750).
    ///
    /// The lane set is `controls::live_lanes` of the compiled session, the selection the C ABI and
    /// the browser make (`control-plane`'s `compile`): every strip's fader/mute and matrix/pan
    /// lanes, each strip's input lane when an input row is live, and one lane per effect instance
    /// when an effect row is live, with no meter and no observation capacity (the C ABI's default
    /// `HostLiveControlRequest`). The strips are prepared through the `Concurrent` delivery entry
    /// (`prepare_session_builtins_with_live_controls`) those hosts reach through `host-core`.
    /// [`SessionRuntime::live_lanes`] states the set; [`SessionRuntime::write_strip_transaction`]
    /// writes the strip cells.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build`], and if the session asks for route lanes, which this harness
    /// does not attach: only a route into a submix has one.
    pub fn build_with_product_lanes(workload: Workload) -> Self {
        Self::build_with_lane_group(workload, LaneGroup::Product)
    }

    /// Compiles, prepares and binds one console arm with one group of the product's live lanes
    /// (issue #1763), as [`SessionRuntime::build_with_product_lanes`] does for the whole set.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build_with_product_lanes`], and if `group` attaches a lane the
    /// product set of the session does not carry.
    pub fn build_with_lane_group(workload: Workload, group: LaneGroup) -> Self {
        Self::build_inner(
            workload,
            PlanConfig::BASELINE,
            Backend::current(),
            SourceSignal::Local,
            Some(group),
        )
    }

    /// The optional lanes this arm attached, read from its channels
    /// ([`SessionRuntime::build_with_lane_group`]), or `None` for every other arm. A strip's
    /// fader/mute and matrix/pan lanes have no flag: [`SessionRuntime::lane_set_name`] says
    /// whether they were attached.
    #[must_use]
    pub fn live_lanes(&self) -> Option<controls::LiveLanes> {
        self.live_lanes
    }

    /// The lane groups this arm attached, read from its channels and joined by `+` in the order
    /// `fader_matrix`, `strip_input`, `effects`, `routes`, or `None` for an arm with no lane
    /// group. `fader_matrix` is present only when a strip channel was attached.
    #[must_use]
    pub fn lane_set_name(&self) -> Option<String> {
        let lanes = self.live_lanes?;
        let groups: Vec<&str> = [
            (!self.strip_controls.is_empty(), "fader_matrix"),
            (lanes.strip_input, "strip_input"),
            (lanes.effects, "effects"),
            (lanes.routes, "routes"),
        ]
        .into_iter()
        .filter_map(|(attached, group)| attached.then_some(group))
        .collect();
        Some(groups.join("+"))
    }

    fn build_inner(
        workload: Workload,
        config: PlanConfig,
        dispatch: Backend,
        source: SourceSignal,
        lane_group: Option<LaneGroup>,
    ) -> Self {
        let strip_controls_requested = lane_group.is_some_and(LaneGroup::strip_requests);
        let mut model = console_model(workload);
        assert!(
            lane_group.is_none() || config == PlanConfig::BASELINE,
            "{}: the product lane set is the arm's whole live configuration",
            workload.kind()
        );
        if dispatch != Backend::current() && BankWidth::for_backend(dispatch).is_some() {
            fold_console_into_inserts(&mut model);
        }
        let session = compile_session(&model, compile_caps()).expect("compiled console session");
        let meters = if config.meters {
            meter_requests(&model)
        } else {
            Vec::new()
        };
        // Issue #881: the metered row carries the default web boot's meter set and no
        // other console facility, so a facility arm on top of it would meter every track twice and
        // measure a shape no host prepares.
        assert!(
            !workload.web_meters() || config == PlanConfig::BASELINE,
            "{}: the metered row carries its own meters and no other console facility",
            workload.kind()
        );
        assert!(
            !strip_controls_requested || !workload.web_meters(),
            "{}: strip live controls ride the product's Concurrent preparation",
            workload.kind()
        );
        let web_meters = if workload.web_meters() {
            web_meter_requests(&session)
        } else {
            Vec::new()
        };
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let mut effects = prepare_native_session_effects(&session, registry, effect_caps())
            .expect("prepared console effects");
        // Both attaches are the production entry points, called in the order a host calls them.
        // The control channel is attached for every observation arm including `Absent`, so the
        // paired delta between the arms is the observation lane and not the cell reads.
        let superseded = LiveValuesSuperseded::new();
        // Issue #1750: the lanes a product host attaches for this session, chosen from its live
        // rows exactly as `control-plane` chooses them; issue #1763: or one group of them.
        let live_lanes = lane_group.map(|group| {
            let product = controls::live_lanes(session.normalized_model());
            let lanes = group.lanes(product);
            assert!(
                (!lanes.strip_input || product.strip_input)
                    && (!lanes.effects || product.effects)
                    && (!lanes.routes || product.routes),
                "{}: {group:?} attaches a lane the product set does not carry",
                workload.kind()
            );
            lanes
        });
        if let Some(lanes) = live_lanes {
            assert!(
                !lanes.routes || model.submixes.is_empty(),
                "{}: route lanes are not attached by this harness",
                workload.kind()
            );
        }
        let toggles = matches!(
            workload,
            Workload::C1Console {
                state: C1State::Toggle,
                ..
            }
        );
        let controls = if config.control || toggles || live_lanes.is_some_and(|lanes| lanes.effects)
        {
            attach_effect_live_controls(
                &mut effects,
                &superseded,
                match config.observation {
                    ObservationArm::Absent => 0,
                    ObservationArm::Unarmed | ObservationArm::Armed => MAXIMUM_OBSERVATION_TAPS,
                },
            )
            .expect("live-control channels")
        } else {
            Vec::new()
        };
        let observations = if config.observation == ObservationArm::Absent {
            Vec::new()
        } else {
            attach_effect_observation(&mut effects, MAXIMUM_OBSERVATION_TAPS, WINDOW_BLOCKS, &[])
                .expect("effect observation capacity")
        };
        let input = workload.input();
        let mappings = channel_mappings(&model);
        // Issue #881: the metered row is prepared through the builtins entry the default web boot's
        // host preparation reaches, which binds its selected meters as permanent observers, with the
        // product's `Concurrent` delivery. It attaches no control channel. Every other row keeps the
        // entry it has always been prepared through.
        let builtins = if workload.web_meters() {
            // Decision 17, C7: no solo on the bench path.
            let effective = controls::effective(
                session.normalized_model(),
                &controls::MonitorOverlay::default(),
            );
            builtins_compiler::prepare_selected_session_builtins_with_live_controls(
                &session,
                &effective,
                &web_meters,
                &[],
                builtin_caps(),
            )
        } else if strip_controls_requested {
            // Issue #1750: the product's strip lanes, through the entry `host-core` calls for a
            // `Concurrent` host, under the effective values it composes (decision 17, C7: no solo),
            // in the canonical floating-point environment (#1646 D2).
            let effective = {
                let _canonical = lane::CanonicalFpEnv::enter();
                controls::effective(
                    session.normalized_model(),
                    &controls::MonitorOverlay::default(),
                )
            };
            let requests: Vec<TrackControlRequest> = model
                .tracks
                .iter()
                .map(|track| TrackControlRequest {
                    track_id: track.id.as_str().to_owned(),
                    superseded: superseded.clone(),
                    input_lane: live_lanes.is_some_and(|lanes| lanes.strip_input),
                })
                .collect();
            builtins_compiler::prepare_session_builtins_with_live_controls(
                &session,
                &effective,
                &meters,
                &requests,
                builtin_caps(),
            )
        } else {
            builtins_compiler::prepare_session_builtins(&session, &meters, builtin_caps())
        }
        .expect("prepared console builtins");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            input_floors: Default::default(),
            dispatch,
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
            // #1379 H3: the strip lanes this row attaches are live; no row has a route into a
            // submix, so no route lane is.
            gain_liveness: graph_compiler::GainLiveness {
                strip_fader_matrix: strip_controls_requested,
                routes: false,
            },
        })
        .unwrap_or_else(|_| panic!("{}: production console graph", workload.kind()));

        let utilisation = artifact.report().rack_cohorts.utilisation;
        let rack_cohorts = artifact.report().rack_cohorts.clone();
        let envelope = artifact.envelope();
        // Issue #965: every row binds its track inputs the way `host-core` binds a session
        // (`crates/host-core/src/prepare.rs`, `into_bound_with_source_set`): the artifact's track
        // inputs are claimed by a prepared source set, and every other external node is
        // acknowledged as an identity binding.
        //
        // `observers` stays empty on purpose: it is the *external* observer slot. A meter
        // observer is compiler-owned and is appended to this vector by the sealed builtins
        // artifact inside `into_bound_with_source_set`, which is why `meters: true` is expressed
        // as a meter *request* and not as a hand-built observer. Driving the real path is the
        // whole point of the arm.
        let mut claims: Vec<GraphSourceInputClaim> = artifact
            .external_binding_nodes()
            .filter(|node| is_track_input(node))
            .map(|node| GraphSourceInputClaim { node: node.clone() })
            .collect();
        // The set requires its claims strictly ascending, and the driver serves claim `i` as the
        // `i`-th of them.
        claims.sort_unstable();
        // Issue #938: the live row's claims go to the production source set over rings a producer
        // thread fills; every other row's to the frozen driver.
        let (source_set, live) = match workload.source_feed() {
            SourceFeed::PlayedPlanes => {
                let driver = FrozenSourceDriver::new(&claims, input, &source, &mappings);
                let resources = driver.resource_report();
                let set =
                    GraphPreparedSourceSet::new(envelope, claims, resources, Box::new(driver));
                (set, None)
            }
            SourceFeed::LiveProducer => {
                let largest_arrival = artifact
                    .report()
                    .input_arrivals
                    .iter()
                    .map(|(_, arrival)| arrival.0)
                    .max()
                    .unwrap_or(0);
                let (set, feed) = LiveFeed::prepare(
                    envelope,
                    &claims,
                    input,
                    &source,
                    &mappings,
                    largest_arrival,
                );
                (set, Some(feed))
            }
        };
        let nodes = artifact
            .external_binding_nodes()
            .filter(|node| !is_track_input(node))
            .map(|node| GraphNodeBinding::identity(node.clone()))
            .collect();
        let bound = artifact
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
                    "{}: console graph bindings: {}",
                    workload.kind(),
                    failure.code
                )
            });
        let (plan, meter_consumers, strip_controls) =
            (bound.plan, bound.meter_consumers, bound.track_controls);
        assert_eq!(
            strip_controls.len(),
            if strip_controls_requested {
                model.tracks.len()
            } else {
                0
            },
            "{}: one strip live-control channel per requested track",
            workload.kind()
        );
        assert_eq!(
            meter_consumers.len(),
            meters.len() + web_meters.len(),
            "{}: every requested meter stream must reach the plan",
            workload.kind()
        );

        // Issue #1763: the record names what the channels carry, not what the arm asked for.
        let attached = live_lanes.map(|_| controls::LiveLanes {
            strip_input: strip_controls.iter().any(|channel| channel.input.is_some()),
            effects: !controls.is_empty(),
            routes: false,
        });
        assert_eq!(
            attached,
            live_lanes,
            "{}: the attached lanes are the requested ones",
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
        let (bsr_ideal, bsr_ideal_key_class) =
            ideal_bank_slot_runs(&mut plan, &model, input, &rack_cohorts);
        let gate = plan.take_revision_gate().expect("the plan's gate handle");
        let mut runtime = Self {
            strip_controls,
            superseded,
            live_lanes: attached,
            plan,
            gate,
            output: Box::new(OutputPlanes([0.0; QUANTUM * 2])),
            meter_consumers,
            controls,
            observations,
            structural_symmetry: structural,
            bypass: BypassCensus::of(session.normalized_model()),
            live,
            utilisation,
            bsr_ideal,
            bsr_ideal_key_class,
            toggle: None,
        };
        if toggles {
            runtime.toggle = Some(
                runtime
                    .control_channel("ch00", "miso.gate-expander")
                    .expect("ch00's gate has a live-control channel"),
            );
        }
        if config.observation == ObservationArm::Armed {
            runtime.arm_observation();
        }
        runtime
    }

    /// Arms every declared tap of every prepared effect, the way a subscribing console does.
    ///
    /// Off the clock, and through the same observation queue a host pushes: the records are
    /// drained by the render thread at the top of the next block, so the arm must render at least
    /// one untimed block after this before it is actually armed. Its warmup does.
    fn arm_observation(&mut self) {
        for producer in &mut self.controls {
            for tap_index in 0..producer.descriptor.observations.len() as u32 {
                producer
                    .try_push_observe(tap_index, true, WINDOW_BLOCKS, 0)
                    .expect("room in the observation queue");
            }
        }
    }

    /// The live-control channel of the alphabetically first track carrying `effect_id`.
    ///
    /// "One track" has to be chosen by a stable key rather than by taking the first matching
    /// channel: [`attach_effect_live_controls`] returns channels in prepared-entry order, which is
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

    /// The live-control channel of `track_id`'s `effect_id` (a contract id), by stable
    /// identity rather than by position (issue #1003).
    ///
    /// `attach_effect_live_controls` returns channels in prepared-entry order, which moves when the
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

    /// Writes one live-control bypass value into one prepared effect's bypass cell.
    ///
    /// The production control path, like [`SessionRuntime::push_parameter`]: the value is read by
    /// the render thread at the top of the next block, and it moves the channel-symmetry
    /// witness' `UNBYPASSED` term at the same boundary it takes effect on.
    ///
    /// It is here because bypass is the transition the mono collapse is hardest on and the one no
    /// other control record reaches. A bypassed lane's dry signal is delayed through the slot's own
    /// latency **line**, which persists across blocks, so engaging a bypass after a collapsed run
    /// reads samples the collapsed blocks put into that line. Nothing else in this harness can set
    /// that sequence up.
    ///
    /// Returns `false` when the lane refused the record, which a caller counts rather than
    /// ignores. Off the clock.
    pub fn push_bypass(&mut self, channel: usize, bypassed: bool) -> bool {
        let Self { controls, gate, .. } = self;
        let producer = &mut controls[channel];
        let record = EffectControlRecord::Bypass(effect_contract::BypassSwitch::step(bypassed));
        producer.preflight(record).is_ok()
            && write_published(gate, |stamp| producer.write(stamp, record).is_ok())
    }

    /// Writes one live-control parameter retarget into one prepared effect's cell.
    ///
    /// This is the production control path and nothing else: the value is read by the render
    /// thread at the top of the next block and staged as a single
    /// [`PreparedAutomationSpan`](effect_contract::PreparedAutomationSpan) at that
    /// block's first sample. One call per block therefore *is* "one span per block", by the
    /// contract's own construction rather than by a hand-built span a benchmark asserts is
    /// equivalent.
    ///
    /// Off the clock, like the observation arming and every other control-side method on this
    /// type. Returns `false` when the lane refused the record, which a driver counts rather than
    /// ignores -- a silently refused write would report the cost of automation that never
    /// happened.
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
                let Self { controls, gate, .. } = &mut *self;
                write_published(gate, |stamp| {
                    controls[channel].publish_candidate_targets(
                        stamp,
                        base_revision,
                        &targets[..count],
                    )
                })
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
        let Self { controls, gate, .. } = self;
        let producer = &mut controls[channel];
        let record = EffectControlRecord::Parameter {
            parameter_index,
            channel: parameter_channel,
            value,
        };
        producer.preflight(record).is_ok()
            && write_published(gate, |stamp| producer.write(stamp, record).is_ok())
    }

    /// The strip live-control channel of `track_id` (issue #1750), or `None` on an arm prepared
    /// without strip controls or for a track the session does not declare.
    #[must_use]
    pub fn strip_control(&self, track_id: &str) -> Option<usize> {
        self.strip_controls
            .iter()
            .position(|producer| &*producer.track_id == track_id)
    }

    /// Writes one transaction of strip live values, as a host commits one (issue #1750): stamp the
    /// next revision on the plan's gate, run `write` over every strip channel with the stamp, then
    /// publish the revision. The next block applies every value of it (decision 15, D15-2).
    ///
    /// Off the clock. A cell write cannot fail, so there is nothing to refuse; the caller counts
    /// what it wrote, and [`SessionRuntime::published_revision`] counts the transactions.
    pub fn write_strip_transaction<R>(
        &mut self,
        write: impl FnOnce(&mut [TrackControlProducer], LiveStamp<'_>) -> R,
    ) -> R {
        let Self {
            strip_controls,
            gate,
            ..
        } = self;
        write_published(gate, |stamp| write(strip_controls, stamp))
    }

    /// The last revision published on this arm's gate: one per live-control transaction.
    #[must_use]
    pub fn published_revision(&self) -> u64 {
        self.gate.last_published()
    }

    /// Live values a later committed value replaced before any block applied them, over every
    /// live-control channel of this arm (decision 15, D15-2).
    #[must_use]
    pub fn live_values_superseded(&self) -> u64 {
        self.superseded.get()
    }

    /// This arm's last rendered block, left plane then right plane. Outside the clock.
    #[must_use]
    pub fn output(&self) -> &[f32] {
        &self.output.0
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

    /// Which tracks the compiled session bypasses, and the pattern they form (issue #1085).
    ///
    /// Read outside the clock, like every other evidence accessor on this type.
    #[must_use]
    pub fn bypass_census(&self) -> BypassCensus {
        self.bypass
    }

    /// The compiled plan's bank utilisation (issue #2132).
    #[must_use]
    pub const fn bank_utilisation(&self) -> graph_compiler::BankUtilisation {
        self.utilisation
    }

    /// The row's key-class ideal bank-slot runs per block (issue #2132), beside the per-group ideal
    /// [`Self::bank_slot_runs`] reports.
    #[must_use]
    pub const fn bsr_ideal_key_class(&self) -> u64 {
        self.bsr_ideal_key_class
    }

    /// Every pack group's bank-slot runs since bind beside its ideal per block (issue #2132), in
    /// unit order. Off the clock.
    pub fn bank_slot_runs(&mut self) -> Vec<BankGroupRuns> {
        let ideal = &self.bsr_ideal;
        graph::test_only_bank_slot_runs(&mut self.plan)
            .into_iter()
            .map(|group| BankGroupRuns {
                first_unit: group.first_unit,
                last_unit: group.last_unit,
                width: group.width,
                effect_slots: group.effect_slots,
                runs: group.runs,
                ideal: ideal
                    .iter()
                    .find(|(first, _)| *first == group.first_unit)
                    .map_or(0, |(_, ideal)| *ideal),
            })
            .collect()
    }

    /// The row's control work before block `block` renders, off the clock: on
    /// [`C1State::Toggle`], every [`C1_TOGGLE_BLOCKS`] blocks, `ch00`'s gate is bypassed and then
    /// un-bypassed in turn; a no-op on every other row.
    ///
    /// # Panics
    ///
    /// Panics if the gate's lane refuses the write.
    pub fn before_block(&mut self, block: u64) {
        if let Some(channel) = self.toggle
            && block > 0
            && block.is_multiple_of(C1_TOGGLE_BLOCKS)
        {
            let bypassed = (block / C1_TOGGLE_BLOCKS) % 2 == 1;
            assert!(
                self.push_bypass(channel, bypassed),
                "ch00's gate refused a bypass write"
            );
        }
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
    /// This runtime census is source agnostic: the collapse decision also needs the structural
    /// witness joined at preparation. It lets the mono rows report the prepared runtime terms;
    /// it alone does not establish that their source mappings permit collapse.
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
    ///   The inputs are claims of a prepared source set (issue #965), which the engine lowers to
    ///   its own source-input kind, and the output is bound through
    ///   `GraphNodeBinding::identity`, which it lowers to its own identity kind. Both report
    ///   `SYMMETRIC` -- truthfully, nothing in a played block's read or an identity can make two
    ///   channels disagree -- while rendering nothing a collapse acts on. That is why the mono
    ///   fixture's census reads `[129, 129]`: it read `[64, 129]` while both were opaque
    ///   processors, `[65, 129]` after issue #221 (`d1cb3653`) rebound the output, and
    ///   `[129, 129]` after issue #965 claimed the inputs, with not one track's witness changed.
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

    /// Starts the live row's producer thread and returns once every ring is full (issue #938).
    ///
    /// `pin` runs first on the producer thread, before it submits anything: a driver places the
    /// thread there (the native bench pins it to its producer core). The thread then tops every
    /// ring up, sleeps while they are full and tops them up again, until
    /// [`SessionRuntime::finish_live_producer`] stops it. It allocates nothing after it starts.
    ///
    /// # Panics
    ///
    /// Panics if this row has no live feed, if the producer was already started, or if the
    /// producer thread panics before the rings are full (`pin` included).
    pub fn start_live_producer(&mut self, pin: impl FnOnce() + Send + 'static) {
        let feed = self.live.as_mut().expect("a live-fed row");
        let LiveState::Ready(mut sources) =
            core::mem::replace(&mut feed.state, LiveState::Finished)
        else {
            panic!("the live producer was already started");
        };
        let shared = Arc::clone(&feed.shared);
        let handle = std::thread::Builder::new()
            .name("console-live-producer".to_owned())
            .spawn(move || {
                pin();
                while !shared.stop.load(Ordering::Acquire) {
                    let submitted = sources.top_up();
                    shared
                        .submitted_blocks
                        .store(sources.submitted_blocks(), Ordering::Release);
                    if !submitted {
                        std::thread::sleep(LIVE_PRODUCER_IDLE);
                    }
                }
                sources
            })
            .expect("the live producer thread");
        feed.state = LiveState::Running(handle);
        feed.wait_for(feed.ring_blocks);
    }

    /// Waits, outside any clock, until every live ring holds block `observation` and
    /// [`LIVE_LEAD_BLOCKS`] more (issue #938). Returns at once on every row without a live feed.
    ///
    /// # Panics
    ///
    /// Panics if this row's producer is not running or has ended.
    pub fn await_live_feed(&self, observation: u64) {
        if let Some(feed) = &self.live {
            feed.wait_for(observation + 1 + LIVE_LEAD_BLOCKS);
        }
    }

    /// Stops and joins the live row's producer, then reads every ring's underrun count (issue
    /// #938). Call it after the last rendered block; the producer is joined on drop otherwise.
    ///
    /// # Panics
    ///
    /// Panics if this row has no live feed, if it was already finished, or if the producer
    /// thread panicked.
    pub fn finish_live_producer(&mut self) -> LiveProducerReport {
        let sources = self
            .live
            .as_mut()
            .expect("a live-fed row")
            .join()
            .expect("the live producer was already finished");
        let underrun_frames = sources
            .providers
            .iter()
            .map(|provider| {
                // Render has stopped, so no publication is in flight and the first read lands.
                provider
                    .seek_report()
                    .expect("a seek report with render stopped")
                    .cumulative_underrun_frames
            })
            .sum();
        LiveProducerReport {
            underrun_frames,
            submitted_blocks: sources.submitted_blocks(),
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
    let template = std::mem::replace(&mut model.tracks, Vec::with_capacity(tracks));
    let route = model.routes[0].clone();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template[index % template.len()].clone();
        track.id = StableId::parse(&format!("ch{index:03}")).expect("synthetic track id");
        let mut next = route.clone();
        next.id = StableId::parse(&format!("ch{index:03}-main")).expect("synthetic route id");
        next.source = session::RouteSource::Track {
            track_id: track.id.clone(),
            tap: session::SendTap::PostPan,
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
        maximum_automation_spans_per_block: 128,
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
        maximum_finite_tail_samples: u64::MAX,
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
/// been reported as a cross-target numeric divergence by anyone who did not look. Injecting one
/// table isolates the engine comparison from this input-generation difference.
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
    fn block(&self, track: usize, silent: bool) -> Option<Cow<'_, [f32]>> {
        match self {
            Self::Local => Some(Cow::Owned(source_block(track, silent))),
            Self::Injected(table) => table
                .get(track * SOURCE_BLOCK_VALUES..(track + 1) * SOURCE_BLOCK_VALUES)
                .map(Cow::Borrowed),
        }
    }
}

/// Every track input is a frozen block per observation; nothing is decoded on the render path.
///
/// Aligned to 64 bytes (issue #935) so both planes start a cache line wherever the struct is
/// boxed: every row's source set holds one slice of them ([`FrozenSourceDriver`]). Unaligned,
/// the allocator's 16-byte placement split half the 32-byte loads that copy or read these words, and moved the ring row's time between
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

/// One track input's frozen block in source block `at`, from its track id: the words the source
/// set serves for that track.
///
/// The one place a track id becomes samples; [`FrozenSourceDriver`] builds every claim with it.
fn frozen_track_source(
    track_id: &str,
    input: InputSignal,
    at: u64,
    source: &SourceSignal,
    mappings: &[(usize, usize)],
) -> FrozenGraphSource {
    let (block, mapping) = track_block(track_id, input, at, source, mappings);
    FrozenGraphSource::from_block(&block, mapping)
}

/// One track input's unmapped stereo block (left plane, then right) in source block `at` and its
/// declared channel mapping, from its track id: what [`frozen_track_source`] maps into planes and
/// what the live producer submits into the track's ring (issue #938), whose set applies the
/// mapping.
fn track_block<'a>(
    track_id: &str,
    input: InputSignal,
    at: u64,
    source: &'a SourceSignal,
    mappings: &[(usize, usize)],
) -> (Cow<'a, [f32]>, (usize, usize)) {
    let track = track_id
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .unwrap_or(0);
    let block = source
        .block(track, input.silent_in_block(track, at))
        .unwrap_or_else(|| panic!("the injected source table must cover track {track}"));
    let mapping = mappings
        .get(track)
        .copied()
        .unwrap_or_else(|| panic!("the model must declare a source mapping for track {track}"));
    (block, mapping)
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

/// A prepared source set's driver over the benchmark's frozen blocks (issue #928).
///
/// The production feed on the benchmark's input. A host binds a session's track inputs to a
/// source set whose driver plays a block per quantum; this one plays the same frozen block every
/// quantum. It holds one `FrozenGraphSource` per claim, in claim order, each built by
/// [`frozen_track_source`], so claim `i` carries exactly that track's frozen words, channel
/// mapping included. Every row binds its track inputs through it (issue #965).
///
/// It offers the graph both ways to read a claim: `copy_track_input` copies the claim's block into
/// the arena buffer the executor hands it, and `played_planes` lends the same words in place.
/// Which one a claim takes is decided by the graph at bind (the lent-claims path of issue #918),
/// not by this driver.
///
/// A toggling input (issue #1783, [`InputSignal::OddTracksToggle`]) holds a second block per claim,
/// its input in the other phase, built here too; `begin_block` picks the phase from the block's
/// first sample, so the render thread only selects between two preallocated slices.
struct FrozenSourceDriver {
    /// Claim `i`'s `(left, right)` block in the first phase, one allocation for the whole set.
    claims: Box<[FrozenGraphSource]>,
    /// Claim `i`'s block in the second phase, or empty when the input never toggles.
    toggled: Box<[FrozenGraphSource]>,
    /// Whether this block serves [`Self::toggled`].
    in_toggled_phase: bool,
    /// Issue #2133: whether claim `i`'s first-phase block is exact `+0.0` on both planes, decided
    /// here where the blocks are written, so render answers `played_block_silent` by one load as
    /// the engine's own ring does.
    claims_silent: Box<[bool]>,
    /// The same for [`Self::toggled`].
    toggled_silent: Box<[bool]>,
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
        input: InputSignal,
        source: &SourceSignal,
        mappings: &[(usize, usize)],
    ) -> Self {
        let phase = |at: u64| -> Box<[FrozenGraphSource]> {
            claims
                .iter()
                .map(|claim| match &claim.node {
                    GraphNodeId::TrackStage {
                        track_id,
                        stage: TrackStage::Input,
                    } => frozen_track_source(track_id.as_str(), input, at, source, mappings),
                    node => panic!("a source claim must name a track input, not {node:?}"),
                })
                .collect()
        };
        let toggled = if input.toggles() {
            phase(TOGGLE_HALF_PERIOD_BLOCKS)
        } else {
            Box::default()
        };
        let silent = |blocks: &[FrozenGraphSource]| -> Box<[bool]> {
            blocks
                .iter()
                .map(|block| {
                    block
                        .left
                        .iter()
                        .chain(&block.right)
                        .all(|word| word.to_bits() == 0)
                })
                .collect()
        };
        let claims = phase(0);
        Self {
            claims_silent: silent(&claims),
            toggled_silent: silent(&toggled),
            claims,
            toggled,
            in_toggled_phase: false,
        }
    }

    /// The blocks this block serves: the first phase's, or the second's on a toggling input.
    fn served(&self) -> &[FrozenGraphSource] {
        if self.in_toggled_phase {
            &self.toggled
        } else {
            &self.claims
        }
    }

    /// The set's engine-owned bytes: the boxed slices of frozen planes.
    ///
    /// Charged as overhead (source-plane storage) and not as PCM already charged by the session
    /// declaration, because no session declaration charges the benchmark's frozen tone.
    fn resource_report(&self) -> GraphSourceSetResourceReport {
        let bytes = (core::mem::size_of_val(&*self.claims)
            + core::mem::size_of_val(&*self.toggled)
            + core::mem::size_of_val(&*self.claims_silent)
            + core::mem::size_of_val(&*self.toggled_silent)) as u64;
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: 0,
            overhead_bytes: bytes,
            total_engine_owned_bytes: bytes,
            largest_allocation_bytes: bytes,
        }
    }
}

// REALTIME_POLICY_BEGIN
// The methods below run on the render thread: no allocation, lock, syscall or panic path.
impl GraphPreparedSourceSetDriver for FrozenSourceDriver {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::slice(&self.claims)
            + effect_contract::retained::slice(&self.toggled)
            + effect_contract::retained::slice(&self.claims_silent)
            + effect_contract::retained::slice(&self.toggled_silent)
    }

    fn claim_count(&self) -> usize {
        self.claims.len()
    }

    /// Nothing to play: the block is frozen, or on a toggling input one of two frozen blocks,
    /// picked by the block's source time. A frame count other than the quantum the planes were
    /// built at is refused rather than served short.
    fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError> {
        if usize::try_from(frames) != Ok(QUANTUM) {
            return Err(RenderError::InvalidEnvelope);
        }
        self.in_toggled_phase = !self.toggled.is_empty()
            && (first_sample / QUANTUM as u64 / TOGGLE_HALF_PERIOD_BLOCKS) % 2 == 1;
        Ok(())
    }

    /// Copies claim `claim_index`'s frozen block into the destinations the executor hands it.
    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        let Some(claim) = self.served().get(claim_index) else {
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
        self.served()
            .get(claim_index)
            .map(|claim| (&claim.left[..], &claim.right[..]))
    }

    /// Whether claim `claim_index`'s served block is all `+0.0`, as [`FrozenSourceDriver::new`]
    /// recorded it (issue #2133).
    fn played_block_silent(&self, claim_index: usize) -> bool {
        let silent = if self.in_toggled_phase {
            &self.toggled_silent
        } else {
            &self.claims_silent
        };
        silent.get(claim_index).copied().unwrap_or(false)
    }
}
// REALTIME_POLICY_END

/// Frames each live source ring holds (issue #938): the stall ring a product host's default ring
/// is built on.
///
/// `host_core::stall_ring_frames(48_000, 128)`: 100 ms of stall tolerance rounded up to whole
/// quanta (38), plus one quantum held by the consumer and one in the recycle path, so forty
/// quanta. The default ring adds a warm successor's prime headroom to it (#1406 D2); the console
/// benchmark keeps the stall ring, so its ring bytes do not move with that headroom. Mirrored
/// rather than imported, as [`WEB_METER_BLOCKS`] is: the host crate is not a dependency of this
/// subject. The live row's test pins the two equal.
pub const LIVE_RING_FRAMES: u64 = 40 * QUANTUM as u64;

/// Blocks every live ring holds beyond the one a block reads, at least, when the block starts
/// (issue #938): [`SessionRuntime::await_live_feed`] waits outside the clock until they are there.
pub const LIVE_LEAD_BLOCKS: u64 = 2;

/// How long the live producer sleeps after a pass that found every ring full.
///
/// The producer does not spin: a spinning producer polls the recycle queue render writes, and that
/// traffic is a cost no host's pump adds. A host tops its rings up in batches, from a timer or a
/// message; this one does after each sleep. A full ring holds forty blocks, which render consumes in
/// no less than a few hundred microseconds even when it runs flat out, so one sleep never empties a
/// ring, and the render side's wait catches the case that it nearly does.
const LIVE_PRODUCER_IDLE: std::time::Duration = std::time::Duration::from_micros(100);

/// What the live producer and its rings report after it has been joined (issue #938).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiveProducerReport {
    /// Missing in-region frames, summed over every ring: the frames render played as silence
    /// because the producer had not submitted them. Zero is the only admissible value.
    pub underrun_frames: u64,
    /// Blocks the producer submitted into each ring, the smallest over the rings.
    pub submitted_blocks: u64,
}

/// The state the live producer thread and render's wait share.
struct LiveShared {
    /// Set by the driver after the last block; the producer finishes its pass and returns.
    stop: AtomicBool,
    /// Blocks every ring has been submitted, the smallest over the rings. Published after every
    /// pass of the producer; read by [`SessionRuntime::await_live_feed`].
    submitted_blocks: AtomicU64,
}

/// The producer's half of every live ring, and the block it submits into each.
struct LiveSources {
    /// One provider per claim, in claim order: source `i` feeds claim `i`.
    providers: Vec<HostChunkProvider>,
    /// Source `i`'s frozen stereo block: `QUANTUM` left samples, then `QUANTUM` right.
    blocks: Vec<Box<[f32]>>,
    /// The next frame each provider accepts.
    next_frames: Vec<u64>,
}

impl LiveSources {
    /// One pass over every ring: submit each source's block until its ring is full. Allocates
    /// nothing: the chunk borrows the frozen block and the ring copies it into a recycled block.
    ///
    /// Returns whether any block was submitted.
    fn top_up(&mut self) -> bool {
        let mut submitted = false;
        for ((provider, block), next) in self
            .providers
            .iter_mut()
            .zip(&self.blocks)
            .zip(&mut self.next_frames)
        {
            let planes = [&block[..QUANTUM], &block[QUANTUM..SOURCE_BLOCK_VALUES]];
            loop {
                let chunk = HostPlanarChunk {
                    sample_rate_hz: SampleRateHz(SAMPLE_RATE_HZ),
                    generation: SourceGeneration(1),
                    start_frame: SourceFrame(*next),
                    planes: &planes,
                    frames: QUANTUM as u32,
                    end_of_region: false,
                };
                match provider.submit(chunk) {
                    Ok(_) => {
                        *next += QUANTUM as u64;
                        submitted = true;
                    }
                    Err(HostChunkError::Full { .. }) => break,
                    Err(error) => panic!("the live producer's chunk was refused: {error:?}"),
                }
            }
        }
        submitted
    }

    /// Blocks every ring has been submitted, the smallest over the rings.
    fn submitted_blocks(&self) -> u64 {
        self.next_frames.iter().copied().min().unwrap_or(0) / QUANTUM as u64
    }
}

/// Where the live producer is in its life.
enum LiveState {
    /// Prepared and not started: the providers wait on the control thread.
    Ready(LiveSources),
    /// Started: the producer thread owns the providers and returns them when it is stopped.
    Running(std::thread::JoinHandle<LiveSources>),
    /// Joined, or never startable again.
    Finished,
}

/// The live row's rings and producer (issue #938).
struct LiveFeed {
    shared: Arc<LiveShared>,
    state: LiveState,
    /// Blocks a full ring holds: what [`SessionRuntime::start_live_producer`] waits for.
    ring_blocks: u64,
    /// The session timeline's producer half, held for the plan's life as a host holds it. Nothing
    /// seeks.
    _timeline: TimelineProducer,
}

impl LiveFeed {
    /// The production source set over one host-region ring per claim, and the providers that feed
    /// them, built the way `host-core` builds a session's (`crates/host-core/src/prepare.rs`): rings
    /// by `PcmSourceRing::prepare_host_region` from frame zero under one published generation, a
    /// timeline whose history covers the largest input arrival, and the set by
    /// `prepare_session_graph_source_set`. One two-channel source per track, because every track
    /// of the benchmark carries its own tone; each claim maps its source's channels as its track
    /// declares them.
    ///
    /// # Panics
    ///
    /// Panics if a claim names anything but a track input, if `input` toggles (the producer submits
    /// one frozen block per track), or if a preparation step refuses: all are defects in the
    /// subject.
    fn prepare(
        envelope: engine::realtime::RenderEnvelope,
        claims: &[GraphSourceInputClaim],
        input: InputSignal,
        source: &SourceSignal,
        mappings: &[(usize, usize)],
        largest_arrival: u64,
    ) -> (GraphPreparedSourceSet, Self) {
        assert!(
            !input.toggles(),
            "the live feed submits one frozen block per track"
        );
        let published = PublishedGeneration::new(SourceGeneration(1));
        let config = PcmSourceRingConfig {
            channel_count: 2,
            quantum_frames: QuantumFrames(QUANTUM as u32),
            frame_capacity: LIVE_RING_FRAMES,
            initial_generation: SourceGeneration(1),
            seek_runway_blocks: 0,
        };
        let mut providers = Vec::with_capacity(claims.len());
        let mut blocks = Vec::with_capacity(claims.len());
        let mut sources = Vec::with_capacity(claims.len());
        let mut track_mappings = Vec::with_capacity(claims.len());
        for (index, claim) in claims.iter().enumerate() {
            let GraphNodeId::TrackStage {
                track_id,
                stage: TrackStage::Input,
            } = &claim.node
            else {
                panic!(
                    "a source claim must name a track input, not {:?}",
                    claim.node
                );
            };
            let (block, (left, right)) = track_block(track_id.as_str(), input, 0, source, mappings);
            let (producer, consumer, resources) =
                PcmSourceRing::prepare_host_region(config, SourceFrame(0), published.clone())
                    .expect("a live source ring");
            providers.push(producer.into_host_chunk_provider(SampleRateHz(SAMPLE_RATE_HZ)));
            blocks.push(Box::from(&*block));
            sources.push(SourceGraphSource::new(consumer, resources, 0, 0));
            track_mappings.push(SourceGraphTrackMapping {
                node: claim.node.clone(),
                source_index: index,
                left_channel: u32::try_from(left).expect("a stereo channel index"),
                right_channel: u32::try_from(right).expect("a stereo channel index"),
            });
        }
        let history_blocks = largest_arrival
            .div_ceil(QUANTUM as u64)
            .checked_add(1)
            .and_then(|blocks| usize::try_from(blocks).ok())
            .and_then(NonZeroUsize::new)
            .expect("a timeline history length");
        let (timeline_producer, timeline, _) = SourceTimeline::prepare(
            QuantumFrames(QUANTUM as u32),
            history_blocks,
            published.clone(),
        )
        .expect("a live session timeline");
        let set = prepare_session_graph_source_set(
            envelope,
            sources,
            timeline,
            published,
            track_mappings,
        )
        .expect("the live source set");
        let next_frames = vec![0; providers.len()];
        let feed = Self {
            shared: Arc::new(LiveShared {
                stop: AtomicBool::new(false),
                submitted_blocks: AtomicU64::new(0),
            }),
            state: LiveState::Ready(LiveSources {
                providers,
                blocks,
                next_frames,
            }),
            ring_blocks: LIVE_RING_FRAMES / QUANTUM as u64,
            _timeline: timeline_producer,
        };
        (set, feed)
    }

    /// Waits until every ring holds `blocks` submitted blocks. Panics if the producer thread has
    /// ended, so a producer that panicked fails the driver instead of hanging it.
    fn wait_for(&self, blocks: u64) {
        let LiveState::Running(handle) = &self.state else {
            panic!("the live producer is not running");
        };
        let mut spins = 0_u32;
        while self.shared.submitted_blocks.load(Ordering::Acquire) < blocks {
            assert!(
                !handle.is_finished(),
                "the live producer thread ended early"
            );
            spins += 1;
            if spins.is_multiple_of(64) {
                std::thread::yield_now();
            } else {
                core::hint::spin_loop();
            }
        }
    }

    /// Stops and joins the producer, if it runs, and returns its sources.
    fn join(&mut self) -> Option<LiveSources> {
        match core::mem::replace(&mut self.state, LiveState::Finished) {
            LiveState::Running(handle) => {
                self.shared.stop.store(true, Ordering::Release);
                Some(handle.join().expect("the live producer thread panicked"))
            }
            LiveState::Ready(sources) => Some(sources),
            LiveState::Finished => None,
        }
    }
}

impl Drop for LiveFeed {
    fn drop(&mut self) {
        if let LiveState::Running(handle) = core::mem::replace(&mut self.state, LiveState::Finished)
        {
            self.shared.stop.store(true, Ordering::Release);
            // A producer that panicked has already said why; a drop does not panic again.
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::realtime::audit;

    /// A real delay composed with the finite builtin sections must fit the tool's graph caps.
    /// The console fixture's infinite EQ tail would otherwise mask a finite-tail cap regression.
    #[test]
    fn tool_caps_admit_a_composed_finite_delay_tail() {
        let mut model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("one-track fixture");
        model.automation.clear();
        let effect = &mut model.tracks[0].inserts.effects[0];
        effect.identity = session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.delay").expect("delay id"),
        };
        effect.params.clear();
        let session = compile_session(&model, compile_caps()).expect("delay session");
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let effects = prepare_native_session_effects(&session, registry, effect_caps())
            .expect("prepared delay");
        let delay_tail = effects.entries[0].metadata.tail;
        assert!(matches!(delay_tail, effect_contract::TailSamples::Finite(n) if n > 10_000_000));
        let builtins = builtins_compiler::prepare_session_builtins(&session, &[], builtin_caps())
            .expect("prepared finite builtins");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            input_floors: Default::default(),
            dispatch: Backend::current(),
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
            gain_liveness: graph_compiler::GainLiveness::FIXED,
        })
        .unwrap_or_else(|failure| panic!("finite delay graph: {:?}", failure.diagnostics));
        for tail in [
            artifact.report().output_tail,
            artifact.report().output_tail_every_peak,
        ] {
            assert!(matches!(tail, effect_contract::TailSamples::Finite(_)) && tail >= delay_tail);
        }
    }

    /// The row facts a record states, less the feed.
    fn stated_facts(workload: Workload) -> (u32, &'static str, bool, &'static str, &'static str) {
        (
            workload.tracks(),
            workload.fixture_id(),
            workload.synthetic(),
            workload.strip_content(),
            workload.strip_layout(),
        )
    }

    /// Issue #2147 gate 1: the reverb row binds one per-node reverb, on `aux.b`, and feeds it:
    /// all sixteen tracks' sends to `aux.b` are unmuted, and the row renders other words than the
    /// [`C1State::AllActive`] row (the same session with no reverb and every send muted). With
    /// the sends muted the reverb reads silence, outputs silence, and the two rows' words agree.
    ///
    /// Test value: red when the row stops measuring a fed reverb.
    #[test]
    fn the_reverb_row_feeds_one_per_node_reverb_on_aux_b() {
        const BLOCKS: u64 = 64;
        let c1 = |state| Workload::C1Console {
            tracks: C1Tracks::Sixteen,
            state,
        };
        let render = |workload: Workload| {
            let mut runtime = SessionRuntime::new(workload);
            let mut sink = Sha256Sink::new();
            for block in 0..BLOCKS {
                runtime.before_block(block);
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut sink);
            }
            (runtime, sink.finish_hex())
        };
        let (mut runtime, reverb) = render(c1(C1State::Reverb));
        let model = console_model(c1(C1State::Reverb));
        let session = compile_session(&model, compile_caps()).expect("compiled session");
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let prepared = prepare_native_session_effects(&session, registry, effect_caps())
            .expect("prepared effects");
        let reverbs: Vec<&str> = prepared
            .entries
            .iter()
            .filter(|entry| entry.effect_id == "reverb")
            .map(|entry| entry.track_id.as_str())
            .collect();
        assert_eq!(reverbs, ["aux.b"], "the strips that carry a reverb");
        let banked = graph::bank_effect_lanes(&mut runtime.plan)
            .unwrap_or_default()
            .into_iter()
            .any(|(_, lane)| lane.effect_id.as_ref() == "reverb");
        assert!(!banked, "the reverb renders per node, in no bank");
        let fed = model
            .routes
            .iter()
            .filter(|route| {
                !route.mute
                    && matches!(route.source, session::RouteSource::Track { .. })
                    && matches!(
                        &route.destination,
                        session::RouteDestination::SubmixInput { submix_id }
                            if submix_id.as_str() == "aux.b"
                    )
            })
            .count();
        assert_eq!(fed, 16, "unmuted track sends into aux.b");
        assert_ne!(
            reverb,
            render(c1(C1State::AllActive)).1,
            "the reverb row renders the all-active row's words: its reverb reads silence"
        );
    }

    /// Issue #965 gate 1: every session row the bench emits is fed the way a host feeds a
    /// session, and every claim is read in place by its bank's gather.
    ///
    /// The source-plane counts (`graph::test_only_source_plane_counts`, `[claims copied into the
    /// arena, gathers served from a played block, gathers served silence]`) are the statement of
    /// which feed ran: a row bound through a processor per track input has no source set and
    /// counts nothing. On every row each track input's only reader is its cohort's
    /// `PostInputBuiltins` bank, whose gather reads the played block in place (issue #918,
    /// `source_plane_table` clause (b)), so no claim is copied and every track's claim is
    /// gathered from its played block once per block (none on the all-silent idle row, whose banks
    /// skip their work; the sparse row's silent tracks lend frozen zeros, so none is served
    /// silence either). A row whose inputs rendered per node instead would copy each claim into
    /// the arena and fail the copy count; no row does today, so the gate is exact on every row
    /// rather than "production feed, digest unchanged" on some.
    ///
    /// Every render runs under the realtime audit, and the audited allocator aborts the process on
    /// an allocation inside a render scope, so zero forbidden operations is the driver's methods
    /// and the source-set loop around them staying allocation-, lock- and syscall-free.
    #[test]
    fn every_row_is_fed_through_a_source_set_and_gathered_in_place() {
        const BLOCKS: u64 = 64;
        // Issue #2132: the bank-utilisation rows are fed and gathered as every session row is.
        for workload in native_session_rows().chain(BANK_REPORT_WORKLOADS) {
            let kind = workload.kind();
            let live = LIVE_FED_WORKLOADS.contains(&workload);
            assert_eq!(
                workload.source_feed(),
                if live {
                    SourceFeed::LiveProducer
                } else {
                    SourceFeed::PlayedPlanes
                },
                "{kind}"
            );
            let mut runtime = SessionRuntime::new(workload);
            // Issue #938: the live row's rings are the production source set's, fed by its
            // producer, and they are read in place exactly as the frozen driver's blocks are.
            if live {
                runtime.start_live_producer(|| {});
            }
            graph::test_only_source_plane_reset();
            audit::warm_up();
            audit::reset();
            // The claim is the feed, so every bank with an audible lane gathers: the per-block
            // lane packing (issue #1981) is off, which would let the sparse rows' banks skip.
            graph::test_only_disable_lane_packing(true);
            for block in 0..BLOCKS {
                runtime.await_live_feed(block);
                runtime.render(block).expect("console render");
            }
            graph::test_only_disable_lane_packing(false);
            let forbidden = audit::snapshot().total();
            let planes = graph::test_only_source_plane_counts();
            if live {
                assert_eq!(runtime.finish_live_producer().underrun_frames, 0, "{kind}");
            }
            assert_eq!(
                forbidden, 0,
                "{kind}: a forbidden operation on the render path"
            );
            // A bank whose every lane reads exact zeros skips its work, its gather included, so
            // the all-silent idle row gathers nothing.
            let gathers = if workload.input() == InputSignal::Silence {
                0
            } else {
                u64::from(workload.tracks()) * BLOCKS
            };
            assert_eq!(
                planes,
                [0, gathers, 0],
                "{kind}: no claim copied, every track's claim gathered from its played block once \
                 per block, none served silence"
            );
        }
    }

    /// Issue #938 gates 1 and 2: with its producer thread live, the live row renders the ring
    /// row's bits over 256 blocks and no ring underruns.
    ///
    /// 256 blocks is six times a ring's forty, so the producer has refilled every ring many times
    /// over while render read it, and every block render read was written by the producer thread
    /// into a recycled ring block. The ring row lends the same frozen words in place, so the two
    /// digests agree block for block; there is no first-block latency to shift by, because the
    /// session timeline and every ring start at frame zero and the plan has no input arrival.
    /// The ring depth is the stall ring a product host's default ring is built on, pinned here
    /// against the host's own formula because this subject mirrors it.
    #[test]
    fn the_live_row_renders_the_ring_rows_bits_with_no_underrun() {
        const BLOCKS: u64 = 256;
        assert_eq!(
            LIVE_RING_FRAMES,
            u64::from(host_core::stall_ring_frames(SAMPLE_RATE_HZ, QUANTUM as u32))
        );
        let digest = |runtime: &mut SessionRuntime| {
            let mut sink = Sha256Sink::new();
            for block in 0..BLOCKS {
                runtime.await_live_feed(block);
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut sink);
            }
            sink.finish_hex()
        };
        let ring = digest(&mut SessionRuntime::new(
            Workload::SixtyFourTrackGainPanRing,
        ));
        let mut live = SessionRuntime::new(Workload::SixtyFourTrackGainPanLive);
        live.start_live_producer(|| {});
        let live_digest = digest(&mut live);
        let report = live.finish_live_producer();
        assert_eq!(report.underrun_frames, 0, "a live ring underran");
        assert!(
            report.submitted_blocks >= BLOCKS + LIVE_LEAD_BLOCKS,
            "the producer stayed ahead: {report:?}"
        );
        assert_eq!(
            live_digest, ring,
            "the live feed served other words than the ring row's"
        );
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
        let injected = SourceSignal::Injected(
            (0..model.tracks.len())
                .flat_map(|track| source_block(track, false))
                .collect(),
        );
        let mut driver = FrozenSourceDriver::new(&claims, InputSignal::Tone, &injected, &mappings);
        let count = claims.len();
        let report = driver.resource_report();
        // The frozen planes, and one silence flag per claim (issue #2133); a tone never toggles.
        assert_eq!(
            report.total_engine_owned_bytes,
            (count * SOURCE_BLOCK_VALUES * core::mem::size_of::<f32>() + count) as u64
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

    /// Issue #935 gate 2 (re-homed onto the gain/pan row by #956 and #965): every frozen block the
    /// harness serves, and both output planes, start a 64-byte line.
    ///
    /// The ring row's claims are built by `FrozenSourceDriver::new`, the constructor `build_full`
    /// calls, over that row's own track inputs, and each is read back through `played_planes`,
    /// the accessor the graph reads a claim in place through. The output planes are the runtime's
    /// own. Alignment follows from the types' layout, but every claim and
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
        let driver =
            FrozenSourceDriver::new(&claims, InputSignal::Tone, &SourceSignal::Local, &mappings);
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

        let runtime = SessionRuntime::new(ring);
        let (left, right) = runtime.output.0.split_at(QUANTUM);
        assert!(
            aligned(left) && aligned(right),
            "output: left {:p}, right {:p}",
            left.as_ptr(),
            right.as_ptr()
        );
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
        assert!(!metered.collapse_forced_off());
        assert!(metered.web_meters());
        assert!(
            METERED_WORKLOADS == [metered],
            "the metered row is the only one"
        );
        for workload in WORKLOADS
            .into_iter()
            .chain(DRIVER_FED_WORKLOADS)
            .chain(LIVE_FED_WORKLOADS)
            .chain(BUS_SEND_WORKLOADS)
            .chain(CONSOLE_STRIP_WORKLOADS)
            .chain(PACKING_WORKLOADS)
            .chain(BYPASS_WORKLOADS)
        {
            assert!(!workload.web_meters(), "{}", workload.kind());
        }
        let rows: Vec<Workload> = native_session_rows().collect();
        assert_eq!(
            rows.len(),
            WORKLOADS.len()
                + DRIVER_FED_WORKLOADS.len()
                + LIVE_FED_WORKLOADS.len()
                + METERED_WORKLOADS.len()
                + BUS_SEND_WORKLOADS.len()
                + CONSOLE_STRIP_WORKLOADS.len()
                + PACKING_WORKLOADS.len()
                + BYPASS_WORKLOADS.len()
        );
        assert!(
            rows.get(WORKLOADS.len() + DRIVER_FED_WORKLOADS.len() + LIVE_FED_WORKLOADS.len())
                == Some(&metered),
            "the metered row is emitted after the gain/pan rows, before the bus-and-send row"
        );
        let mut kinds: Vec<&str> = rows.iter().map(|workload| workload.kind()).collect();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(kinds.len(), rows.len(), "every emitted kind is distinct");
    }

    /// Issue #881 gate 3: the metered row renders the standing console row's bits, has its plan
    /// shape, and every track publishes one sample-peak snapshot per twelve-block window, on time,
    /// with none dropped.
    ///
    /// Sixty-four blocks of both current rows, digested by `hash_output` and compared directly:
    /// a meter observes without changing signal flow. Independent lane matrix laws and the settled
    /// dispatch witness own matrix correctness; #1146 retired the historical standing-row digest
    /// pin.
    ///
    /// Both plans bank the same `[chains, slots]` (eight chains, 48 memberships at the launch
    /// width), transpose the same number of times, fold every route of the console (the #885
    /// contract), and run the same stage count each block
    /// (`graph::test_only_bank_chain_construction_facts`, read across each bind): one per
    /// membership, 48 on each. Each plan's scatter redirect count is pinned on its own (0 on both
    /// today).
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
        // The same six memberships per cohort (48 at the launch width) run as six chain stages
        // per cohort on both rows.
        assert_eq!(
            [
                console_chains.run_memberships,
                console_chains.runtime_slots,
                metered_chains.run_memberships,
                metered_chains.runtime_slots,
            ],
            [6 * cohorts, 6 * cohorts, 6 * cohorts, 6 * cohorts].map(|count| count as usize),
            "[standing memberships, standing stages, metered memberships, metered stages] at width \
             {width}"
        );
        assert_eq!(
            [console.bank_route_folds(), metered.bank_route_folds()],
            [tracks, tracks],
            "every route of the console folds, metered or not (#885)"
        );
        assert_eq!(
            [
                console.bank_scatter_redirects(),
                metered.bank_scatter_redirects()
            ],
            [0, 0],
            "each plan's own redirect count"
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
                    snapshot.reset_generation, 0,
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

    // -----------------------------------------------------------------------------------------
    // Issue #1227: the bus-and-send row.
    // -----------------------------------------------------------------------------------------

    /// Issue #1227: the bus-and-send row states the standing console row's track facts under its
    /// own kind and fixture, renders that fixture as written, and is emitted after the metered
    /// row and before the console-strip rows.
    ///
    /// Its unique catch is the parity the row's record cannot show: a non-removal strip edit
    /// attached to the row (a `HalfMono`-style edit, which still compiles, renders and passes the
    /// record validator, because `strip_content` is a separate wildcard) or a warmup (not a record
    /// key). Dropping the row's `fixture_id` arm, leaving it out of `synthetic`'s list or chaining
    /// [`BUS_SEND_WORKLOADS`] after [`CONSOLE_STRIP_WORKLOADS`] also fail here, and elsewhere.
    #[test]
    fn the_bus_send_row_states_its_facts_and_is_emitted_before_the_strip_rows() {
        let console = Workload::SixtyFourTrackConsole;
        let sends = Workload::SixtyFourTrackConsoleSends;
        assert_eq!(sends.kind(), "sixty_four_track_console_sends");
        assert_eq!(
            stated_facts(sends),
            (
                64,
                "fixtures/session/v1/console-sixty-four-track-sends.json",
                false,
                console.strip_content(),
                console.strip_layout()
            )
        );
        assert!(sends.strip() == Strip::AsWritten);
        assert_eq!(sends.input(), InputSignal::Tone);
        assert_eq!(sends.warmup_blocks(), console.warmup_blocks());
        assert!(!sends.web_meters() && !sends.collapse_forced_off());
        assert!(
            BUS_SEND_WORKLOADS == [sends],
            "the bus-and-send row is the only one"
        );
        assert!(!DRIVER_FED_WORKLOADS.contains(&sends));
        let rows: Vec<Workload> = native_session_rows().collect();
        assert!(
            rows.get(
                WORKLOADS.len()
                    + DRIVER_FED_WORKLOADS.len()
                    + LIVE_FED_WORKLOADS.len()
                    + METERED_WORKLOADS.len()
            ) == Some(&sends),
            "the bus-and-send row is emitted after the metered row, before the console-strip rows"
        );
    }

    /// Issue #1227 D6: the bus-and-send row's compiled plan carries what the fixture declares,
    /// derived from D1 rather than pinned, and every send in it is active.
    ///
    /// Compiled through the calls `build_full` makes at [`PlanConfig::BASELINE`] (no meters, no
    /// live controls, `Concurrent` builtins). The fold count
    /// is printed for the baseline's report, not asserted (#1227 D6 says why).
    ///
    /// Red mutations: drop the row's `console_model` arm (it renders the intended fixture: 64
    /// transforms, one reduction), mute one send or a track strip in the fixture (a `route-mute`
    /// or `route-follow-zeroed` row), or give a submix's insert latency (a nonzero compensation).
    /// A muted submix strip is not a route mute and its returns do not follow mutes, so it leaves
    /// this test green; `scripts/check-console-fixtures.sh`'s witness that every submix fader is
    /// unmuted holds it.
    #[test]
    fn the_bus_send_rows_plan_carries_every_route_unmuted_and_uncompensated() {
        let workload = Workload::SixtyFourTrackConsoleSends;
        let model = console_model(workload);
        let session = compile_session(&model, compile_caps()).expect("compiled console session");
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let effects = prepare_native_session_effects(&session, registry, effect_caps())
            .expect("prepared console effects");
        let builtins = builtins_compiler::prepare_session_builtins(&session, &[], builtin_caps())
            .expect("prepared console builtins");
        let Ok(artifact) = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            input_floors: Default::default(),
            dispatch: Backend::current(),
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
            gain_liveness: graph_compiler::GainLiveness::FIXED,
        }) else {
            panic!("bus-and-send console graph");
        };
        let evidence = GraphCompiler::evidence(artifact.graph(), artifact.report());
        let text = String::from_utf8(evidence.canonical_bytes).expect("canonical text is UTF-8");
        let rows = |tag: &str| -> Vec<Vec<&str>> {
            text.lines()
                .map(|line| line.split('\t').collect::<Vec<_>>())
                .filter(|fields| fields[0] == tag)
                .collect()
        };

        // D1: 64 tracks, 8 buses of 64 / 8 tracks each, and two effect returns.
        let tracks = 64_usize;
        let buses = 8_usize;
        let submixes = buses + 2;
        // One transform per route: each track's main route into its bus (64), its pre-fader send
        // into `fx-a` and its post-fader send into `fx-b` (2 x 64), and one return per submix (10).
        let routes = tracks + 2 * tracks + submixes;
        assert_eq!(routes, 202);
        let transforms = rows("route-transform");
        assert_eq!(transforms.len(), routes);
        // A reduction at every node that sums two or more routes: each bus input (8 tracks),
        // `fx-a`'s and `fx-b`'s inputs (64 sends each) and `main-out` (10 returns): 11.
        let reductions: BTreeSet<&str> = rows("reduction").iter().map(|fields| fields[1]).collect();
        assert_eq!(reductions.len(), submixes + 1);
        assert_eq!(reductions.len(), 11);
        // One timing row per route, none compensated: every effect but the limiter has latency 0,
        // every strip (track and submix) carries the limiter slot exactly once, and every tap a
        // route reads (`pre_fader`, `post_fader`, `post_pan`) lies after `post_insert`, so all
        // contributions to a reduction arrive at the same sample.
        let timings = rows("route-timing");
        assert_eq!(timings.len(), routes);
        let delayed: Vec<&Vec<&str>> = timings.iter().filter(|fields| fields[3] != "0").collect();
        let delayed_route_edges = delayed.len();
        assert_eq!(delayed_route_edges, 0, "compensated routes: {delayed:?}");
        // No route and no track strip is muted, so no gate is sealed and every send is active.
        assert!(rows("route-mute").is_empty());
        assert!(rows("route-follow-zeroed").is_empty());

        let runtime = SessionRuntime::new(workload);
        let folds = runtime.bank_route_folds();
        let route_transforms = transforms.len() as u64;
        println!(
            "bus_send_plan_facts route_transforms={route_transforms} reduction_nodes={} \
             bank_route_folds={folds} route_ops_per_block={} delayed_route_edges={delayed_route_edges}",
            reductions.len(),
            route_transforms - folds,
        );
    }

    // -----------------------------------------------------------------------------------------
    // Issue #1085: the console-strip rows.
    // -----------------------------------------------------------------------------------------

    /// The five console-strip rows, in emission order.
    const STRIP_ROWS: [Workload; 5] = CONSOLE_STRIP_WORKLOADS;

    /// The standing console's model: the fixture every strip-at-N row and the sparse row is cut
    /// from, and the app shape is derived from.
    fn standing_model() -> SessionModel {
        console_model(Workload::SixtyFourTrackConsole)
    }

    /// A track's facts less its identity: what a truncation must carry over unchanged.
    fn track_body(track: &session::Track) -> session::Track {
        let mut body = track.clone();
        body.id = StableId::parse("body").expect("stable id");
        body
    }

    /// Issue #1085: each console-strip row states the facts it is pinned on, the rows are emitted
    /// last, and only the app shape carries a bypass.
    ///
    /// Red mutations: give `SixteenTrackStrip` 64 tracks, report the app shape as synthetic, or
    /// give the sparse row the `Tone` input -- the stated-facts table fails; append a row to
    /// `WORKLOADS` instead -- the emission-order assertion fails.
    #[test]
    fn the_console_strip_rows_state_their_facts_and_are_emitted_last() {
        let standing = Workload::SixtyFourTrackConsole;
        let intended = "pre_insert:eq+compressor,post_insert:limiter";
        assert_eq!(standing.strip_layout(), intended);
        for (row, kind, tracks) in [
            (Workload::TenTrackRaggedStrip, "ten_track_ragged_strip", 10),
            (
                Workload::ThirteenTrackRaggedStrip,
                "thirteen_track_ragged_strip",
                13,
            ),
            (Workload::SixteenTrackStrip, "sixteen_track_strip", 16),
        ] {
            assert_eq!(row.kind(), kind);
            assert_eq!(
                stated_facts(row),
                (
                    tracks,
                    standing.fixture_id(),
                    true,
                    standing.strip_content(),
                    intended
                ),
                "{kind}"
            );
            assert_eq!(row.input(), InputSignal::Tone, "{kind}");
        }
        // With the two standing rows, the strip at N in {9, 10, 13, 16, 64}.
        let strip_counts: Vec<u32> = [Workload::NineTrackRaggedStrip, standing]
            .into_iter()
            .chain(STRIP_ROWS)
            .filter(|row| {
                row.strip_layout() == intended
                    && row.fixture_id() == standing.fixture_id()
                    && row.input() == InputSignal::Tone
            })
            .map(Workload::tracks)
            .collect();
        assert_eq!(strip_counts, [9, 64, 10, 13, 16]);

        let app = Workload::SixtyFourTrackAppShape;
        assert_eq!(app.kind(), "sixty_four_track_app_shape");
        assert_eq!(
            stated_facts(app),
            (
                64,
                "fixtures/session/v1/console-sixty-four-track-app.json",
                false,
                "eq+compressor",
                "pre_insert:eq+compressor"
            )
        );
        let sparse = Workload::SixtyFourTrackConsoleSparse;
        assert_eq!(sparse.kind(), "sixty_four_track_console_sparse");
        assert_eq!(
            stated_facts(sparse),
            (
                64,
                standing.fixture_id(),
                true,
                standing.strip_content(),
                intended
            )
        );
        assert_eq!(sparse.input_signal(), "odd_tracks_silent");
        assert_eq!(sparse.warmup_blocks(), standing.warmup_blocks());
        for row in STRIP_ROWS {
            assert!(row.strip() == Strip::AsWritten, "{}", row.kind());
            assert!(
                !row.web_meters() && !row.collapse_forced_off(),
                "{}",
                row.kind()
            );
        }
        let rows: Vec<Workload> = native_session_rows().collect();
        let strip_end = rows.len() - PACKING_WORKLOADS.len() - BYPASS_WORKLOADS.len();
        let packing_end = strip_end + PACKING_WORKLOADS.len();
        assert!(
            rows[strip_end - STRIP_ROWS.len()..strip_end] == STRIP_ROWS
                && rows[strip_end..packing_end] == PACKING_WORKLOADS
                && rows[packing_end..] == BYPASS_WORKLOADS,
            "the console-strip rows are emitted last but for the packing rows (#1783) and the \
             bypass-skip row (#2129)"
        );
        let mut kinds: Vec<&str> = rows.iter().map(|row| row.kind()).collect();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(kinds.len(), rows.len(), "every emitted kind is distinct");
        for row in rows {
            let census = BypassCensus::of(&console_model(row));
            let expected = if row == app {
                BypassCensus {
                    bypassed_tracks: 21,
                    pattern: BypassPattern::IndexMod3Is2,
                }
            } else if row == Workload::SixteenTrackStripBypassed {
                BypassCensus {
                    bypassed_tracks: 16,
                    pattern: BypassPattern::Other,
                }
            } else {
                BypassCensus {
                    bypassed_tracks: 0,
                    pattern: BypassPattern::None,
                }
            };
            assert_eq!(census, expected, "{}", row.kind());
        }
    }

    /// Issue #1085: a strip-at-N row is the standing fixture's first N tracks, each carried over
    /// whole under a new id, with one `post_pan` route per track to the fixture's output.
    ///
    /// Red mutation: clone `template[template.len() - 1 - index]` in `synthesise_tracks`, or apply
    /// a strip edit to the rows -- the per-track equality fails.
    #[test]
    fn the_strip_at_n_rows_are_the_standing_fixtures_first_n_tracks() {
        let standing = standing_model();
        for row in [
            Workload::NineTrackRaggedStrip,
            Workload::TenTrackRaggedStrip,
            Workload::ThirteenTrackRaggedStrip,
            Workload::SixteenTrackStrip,
        ] {
            let model = console_model(row);
            let tracks = row.tracks() as usize;
            assert_eq!(model.tracks.len(), tracks, "{}", row.kind());
            assert_eq!(model.routes.len(), tracks, "{}", row.kind());
            for (index, track) in model.tracks.iter().enumerate() {
                assert_eq!(track.id.as_str(), format!("ch{index:03}"), "{}", row.kind());
                assert_eq!(
                    track_body(track),
                    track_body(&standing.tracks[index]),
                    "{}: track {index} is the standing fixture's track {index}",
                    row.kind()
                );
                let route = &model.routes[index];
                assert!(
                    route.source
                        == session::RouteSource::Track {
                            track_id: track.id.clone(),
                            tap: session::SendTap::PostPan,
                        },
                    "{}: route {index}",
                    row.kind()
                );
                assert!(
                    route.destination == standing.routes[0].destination,
                    "{}: route {index} reaches the fixture's output",
                    row.kind()
                );
            }
            assert!(
                model.outputs == standing.outputs && model.sources == standing.sources,
                "{}: the outputs and sources are the fixture's",
                row.kind()
            );
        }
    }

    /// Issue #1085: the app shape is the standing strip's own EQ and compressor on every track,
    /// with no limiter, and both effects bypassed on exactly the tracks whose index is 2 mod 3.
    /// Everything else about each track is the standing fixture's.
    ///
    /// In decision 12's console shape since #1093: the EQ and compressor are the session's two
    /// `pre_insert` slots, declared exactly as the standing fixture declares them, the limiter's
    /// `post_insert` slot is gone, and each track's two entries are its standing entries with the
    /// bypass the pattern gives; no track carries an insert.
    ///
    /// Read from the committed fixture through the same parse the row renders, against the
    /// standing model, so the fixture and its generator are held to the row's claim here as well
    /// as by `scripts/check-console-fixtures.sh`.
    #[test]
    fn the_app_shape_keeps_the_standing_eq_and_compressor_slots_and_bypasses_every_third_track() {
        let standing = standing_model();
        let app = console_model(Workload::SixtyFourTrackAppShape);
        assert_eq!(app.console.pre_insert, standing.console.pre_insert);
        assert!(app.console.post_insert.is_empty(), "no limiter slot");
        assert_eq!(app.tracks.len(), standing.tracks.len());
        let mut bypassed = 0;
        for (index, (track, reference)) in app.tracks.iter().zip(&standing.tracks).enumerate() {
            assert!(
                track.inserts.effects.is_empty(),
                "track {index}: no inserts"
            );
            let want_bypass = index % 3 == 2;
            let entries: Vec<session::ConsoleEntry> = track
                .console
                .iter()
                .map(|entry| {
                    assert_eq!(entry.bypass, want_bypass, "track {index}");
                    session::ConsoleEntry {
                        bypass: false,
                        ..entry.clone()
                    }
                })
                .collect();
            assert_eq!(
                entries,
                reference.console[..2],
                "track {index}: the standing EQ and compressor entries"
            );
            bypassed += u64::from(want_bypass);
            let mut body = track.clone();
            body.console.clone_from(&reference.console);
            assert_eq!(&body, reference, "track {index}: nothing else moved");
        }
        assert_eq!(bypassed, 21);
        assert!(app.routes == standing.routes && app.outputs == standing.outputs);
        let runtime = SessionRuntime::new(Workload::SixtyFourTrackAppShape);
        assert_eq!(
            runtime.bypass_census(),
            BypassCensus {
                bypassed_tracks: 21,
                pattern: BypassPattern::IndexMod3Is2,
            },
            "the compiled session carries the app's bypass"
        );
    }

    /// Issue #1085: the bypass census names the app pattern only for the app pattern.
    ///
    /// Red mutation: drop the `some == every` term -- a track bypassing its EQ alone reads as the
    /// app pattern; drop `bypasses().next().is_some()` -- an empty track counts as bypassed; read
    /// the console entries alone -- a bypassed insert goes uncounted.
    #[test]
    fn the_bypass_census_names_the_app_pattern_and_nothing_else() {
        let app = console_model(Workload::SixtyFourTrackAppShape);
        assert_eq!(BypassCensus::of(&app).pattern, BypassPattern::IndexMod3Is2);
        let mut partial = app.clone();
        partial.tracks[2].console[1].bypass = false;
        assert_eq!(
            BypassCensus::of(&partial),
            BypassCensus {
                bypassed_tracks: 20,
                pattern: BypassPattern::Other,
            },
            "a track bypassing its EQ alone"
        );
        let mut extra = app.clone();
        for entry in &mut extra.tracks[0].console {
            entry.bypass = true;
        }
        assert_eq!(BypassCensus::of(&extra).pattern, BypassPattern::Other);
        let mut shifted = app.clone();
        for (index, track) in shifted.tracks.iter_mut().enumerate() {
            for entry in &mut track.console {
                entry.bypass = index % 3 == 1;
            }
        }
        assert_eq!(BypassCensus::of(&shifted).pattern, BypassPattern::Other);
        let mut inserted = app.clone();
        let slot = inserted.console.pre_insert[0].clone();
        inserted.tracks[0].inserts.effects.push(session::Effect {
            id: StableId::parse("extra").expect("stable id"),
            identity: slot.identity,
            quality: slot.quality,
            bypass: true,
            link_mode: slot.link_mode,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        });
        assert_eq!(
            BypassCensus::of(&inserted).pattern,
            BypassPattern::Other,
            "a bypassed insert on a track that bypasses nothing else"
        );
        let mut emptied = app;
        emptied.tracks[1].console.clear();
        assert_eq!(
            BypassCensus::of(&emptied).bypassed_tracks,
            21,
            "a track with no effect is not a bypassed track"
        );
    }

    /// Issue #1085: the sparse row feeds the tone to every even track and exact zeros to every odd
    /// one, and every bank of its plan holds both, so no bank is wholly silent.
    ///
    /// The second half is the row's premise: the silent fast path is decided per bank, so a row
    /// whose silent tracks filled whole banks would measure the skip rather than the case it
    /// cannot serve. Read off the plan's own lane table, so it holds at whatever lane width the
    /// cohort planner banks at.
    ///
    /// Red mutation: make `OddTracksSilent` silence `track >= 32` -- half the banks are wholly
    /// silent and the per-bank assertion fails; ignore the input in `frozen_track_source` -- the
    /// per-track block assertion fails.
    #[test]
    fn the_sparse_row_silences_every_odd_track_and_leaves_no_bank_wholly_silent() {
        let row = Workload::SixtyFourTrackConsoleSparse;
        let model = console_model(row);
        let mappings = channel_mappings(&model);
        for (index, track) in model.tracks.iter().enumerate() {
            let block = frozen_track_source(
                track.id.as_str(),
                row.input(),
                0,
                &SourceSignal::Local,
                &mappings,
            );
            let silent = block
                .left
                .iter()
                .chain(&block.right)
                .all(|word| word.to_bits() == 0);
            let audible = block.left.iter().any(|word| *word != 0.0);
            assert_eq!(silent, index % 2 == 1, "track {index}: exact zeros");
            assert_eq!(audible, index % 2 == 0, "track {index}: the tone");
        }
        let runtime = SessionRuntime::new(row);
        let banks: Vec<PlanUnitEligibility> = runtime
            .unit_eligibility()
            .into_iter()
            .filter(|unit| unit.banked && !unit.lane_tracks.is_empty())
            .collect();
        assert!(!banks.is_empty(), "the sparse row banks its strip");
        let mut lanes = 0;
        for bank in &banks {
            let parity: Vec<usize> = bank
                .lane_tracks
                .iter()
                .map(|track| {
                    track
                        .trim_start_matches(|c: char| !c.is_ascii_digit())
                        .parse::<usize>()
                        .expect("a track lane names a numbered track")
                        % 2
                })
                .collect();
            lanes += parity.len();
            assert!(
                parity.contains(&0) && parity.contains(&1),
                "unit {}: a bank of the sparse row holds an active and a silent track, {:?}",
                bank.unit,
                bank.lane_tracks
            );
        }
        assert!(lanes >= model.tracks.len(), "every track renders in a bank");
    }

    /// Every console-strip row renders an audible, distinct signal over 64 blocks with no
    /// forbidden operation. The sparse row also differs from the current all-active and idle
    /// controls. The model, bypass-census and source-word tests own the exact row derivations.
    #[test]
    fn the_console_strip_rows_render_distinct_audible_signals_without_forbidden_operations() {
        const BLOCKS: u64 = 64;
        let run = |workload: Workload| {
            let mut runtime = SessionRuntime::new(workload);
            let mut digest = Sha256Sink::new();
            let mut audible = false;
            audit::warm_up();
            audit::reset();
            for block in 0..BLOCKS {
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut digest);
                audible |= runtime.output.0.iter().any(|word| *word != 0.0);
            }
            (digest.finish_hex(), audible, audit::snapshot().total())
        };
        let (console, _, _) = run(Workload::SixtyFourTrackConsole);
        let (idle, idle_audible, _) = run(Workload::SixtyFourTrackIdle);
        assert!(!idle_audible, "the idle row renders silence");
        let mut digests = Vec::new();
        for workload in CONSOLE_STRIP_WORKLOADS {
            let (digest, audible, forbidden) = run(workload);
            assert!(audible, "{}: the row renders the tone", workload.kind());
            assert_eq!(forbidden, 0, "{}: a forbidden operation", workload.kind());
            digests.push(digest);
        }
        let sparse = &digests[4];
        assert!(
            *sparse != console && *sparse != idle,
            "the sparse row renders bits of its own"
        );
        let count = digests.len();
        digests.sort_unstable();
        digests.dedup();
        assert_eq!(digests.len(), count, "five rows, five renders");
    }
}
