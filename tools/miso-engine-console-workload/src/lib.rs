//! The console benchmark's *subject*, shared by the native bench and the wasm guest.
//!
//! # Why this crate exists
//!
//! Issue [#163](https://github.com/misofm/engine-v2/issues/163) phase 2 opens with the owner
//! ruling that the unfused multiply-add contract change must be confirmed **at console level**,
//! not at kernel level. Confirming it at console level means running the console workloads under
//! the target the product ships on, and `docs/rulings/wasm-kernel-timing-interim.md` recorded why
//! that was not reachable in phase 0: `tools/miso-engine-bench/src/console.rs` and every compiler
//! it drives were absent from a `wasm32` build of the bench crate.
//!
//! The recorded blocker turned out to be **bench-tool-level, not crate-level**. All four
//! compilers -- `miso-engine-session`, `miso-engine-builtins-compiler`,
//! `miso-engine-effect-compiler` and `miso-engine-graph-compiler` -- build for
//! `wasm32-unknown-unknown` today, unchanged. The `cfg(not(target_arch = "wasm32"))` gates were
//! entries in the *bench manifest*, expressing that the bench binary is a native tool, and not a
//! statement that the subject could not target wasm. So the port needed no crate change at all:
//! it needed the subject to live somewhere both a native binary and a wasm guest can link it.
//!
//! That is this crate. It holds the sixteen console workloads, the model derivation, and the
//! prepared-plan runtime that renders them -- lifted verbatim out of `console.rs`, which now links
//! it. Nothing was reimplemented for wasm and nothing is conditional on the target, so the wasm
//! guest and the native bench execute **the same subject**: the same fixtures, the same strip
//! edits, the same caps, the same source bindings, the same `PreparedRenderPlan`, the same render
//! call. A number taken through the guest and a number taken through the bench differ in the
//! target that executed them and in nothing else, which is the only condition under which their
//! ratio means anything.
//!
//! This follows the shape gate G5 already established: `miso-engine-wasm-gate-corpus` is an
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

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
use miso_engine_builtins_compiler::{MeterConsumer, MeterRequest};
use miso_engine_core::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime};
use miso_engine_effect_compiler::{
    EffectCompileCaps, EffectControlProducerV1, EffectObservationHandleV1,
    attach_effect_console_v1, attach_effect_observation_v1, launch_native_effect_registry_v1,
    prepare_native_session_effects,
};
use miso_engine_effect_contract::{EffectControlRecordV1, ParameterChannel};
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompileRequest, GraphCompiler};
use miso_engine_lane::Backend;
use miso_engine_session::{
    CompileCaps, DualMonoFader, MatrixOrPan, SessionTomlV1, StableId, compile_session,
    parse_session_toml,
};

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
/// `attach_effect_observation_v1` states and the rule a host follows.
pub const WINDOW_BLOCKS: u32 = 4;
/// Bounded depth of each effect's live-console control channel in the observation arms.
pub const CONTROL_QUEUE_DEPTH: usize = 8;
/// Cap on declared observation taps per effect, passed to the observation attach.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 8;
/// Bounded depth of each meter stream. Drained outside the clock after every observation.
pub const METER_QUEUE_DEPTH: usize = 8;

const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
/// The retired 64-track fixture: EQ on `simd1`, compressor in the `dynamic` rack, no limiter.
///
/// Kept, and still rendered, by exactly one row. See [`Workload::SixtyFourTrackConsoleLegacy`].
const SIXTY_FOUR_TRACK_LEGACY: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.toml");
/// The standing 64-track qualification fixture (#175): the intended production rack layout.
///
/// EQ and compressor share one two-slot chain on `simd1`; a true-peak limiter sits alone on
/// `simd2`. Generated from the retired fixture by `scripts/derive-intended-console-fixture.py`,
/// which moves the compressor's declaration verbatim, so every EQ and compressor coefficient is
/// byte-identical between the two files and the only arithmetic that is new is the limiter's.
const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.toml");
/// The mono qualification fixture: the standing strip, collapse-eligible upstream of the seam.
///
/// Generated from the standing fixture by `scripts/derive-mono-console-fixture.py`, which makes
/// three edits and every one of them is upstream of the fader/matrix seam -- both channels read
/// source channel 0, `builtins.right` copies `builtins.left`, and every `channel = "right"`
/// effect parameter takes its `channel = "left"` sibling's value. Those are exactly the two
/// structural terms of the per-track channel-symmetry witness
/// (`miso_engine_effect_contract::ChannelSymmetryWitnessV1`: `SOURCE` and `DESIGNED`), so every
/// track of this fixture is collapse-eligible.
///
/// The fader and pan asymmetry and the limiter's `maximum` link are deliberately *kept*; the
/// generator's header says why, and so does the fixture's own.
const SIXTY_FOUR_TRACK_MONO: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.toml");

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
    /// Decomposition: every rack emptied **and** every builtin asked for its identity.
    ///
    /// Polarity off, trim 0 dB, HPF and LPF at 0 Hz, fader 0 dB unmuted, pan hard identity with no
    /// smoothing.
    ///
    /// What this row is **not**: it is not the cost of dispatch alone, and the record says
    /// `identity` rather than `dispatch` for that reason. A builtin filter at 0 Hz is *disabled*,
    /// and `SvfSection::design` implements disabled by designing an identity section -- `m0 = 1`,
    /// `m1 = m2 = 0`, `k = 0` (`miso-engine-builtins`, the version-1 cutoff contract). A 0 dB fader
    /// is still a multiply and a mask clear, and a settled identity pan matrix still evaluates both
    /// arms of its per-lane select; both of those run over the same lanes every block.
    ///
    /// The two SVF sections no longer do. A prepared section that is the exact identity in every
    /// lane and every word is the map `v |-> v + 0.0`, so a run of them is one `add(+0.0)`, and
    /// `input_chain_block_elided` emits that instead of the recurrence when the bank's prepared
    /// words say it may. The decision is made once, at bank construction, from the coefficient and
    /// state bits.
    ///
    /// So this row measures: source fill, per-node graph dispatch, buffer plumbing, route
    /// summation, the sanitisation and boundary-scan passes the D7 policy requires of every block,
    /// and the fader and matrix kernels running their identity coefficients.
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
    /// The overhead floor: every rack emptied **and no builtin bindings prepared at all**.
    ///
    /// The row below `sixty_four_track_dispatch_only`, and the reason it exists is that
    /// `dispatch_only` is not a floor. An identity strip still pays the D7 input sanitisation and
    /// output boundary scan, a 0 dB fader's multiply and mask clear, and a settled identity pan
    /// matrix's per-lane select -- 22 lane-ops of real arithmetic on every lane of every block.
    /// This row pays none of it: `prepare_session_builtins` is never called, so the strip's input
    /// stage, fader and matrix do not exist as bindings, every `TrackStage` lowers to an elided
    /// alias, and what remains between a track's source and the master bus is the route's
    /// `mix2x2` and the master reduction.
    ///
    /// So `sixty_four_track_gain_pan_only - sixty_four_track_plumbing_only` is the builtins
    /// scaffolding *without* its filters -- sanitise, boundary scan, fader and pan -- isolated
    /// from the graph plumbing underneath it for the first time. The row is the denominator every
    /// overhead claim in this stream was previously missing: before it, "overhead" meant
    /// `dispatch_only`, which is 22 lane-ops of spec-required arithmetic wearing the name of a
    /// floor.
    ///
    /// It is also the one row in the set that binds **no bank chain at all**, which is why the
    /// chain-shape gates name it explicitly instead of iterating over it: with no builtin banks
    /// there is nothing for a route to fold into, and a fold count of zero here is the correct
    /// answer rather than a regression.
    SixtyFourTrackPlumbingOnly,
    /// Decomposition: every rack emptied and every input builtin asked for its identity, with the
    /// fixture's **real** fader and pan values left as written.
    ///
    /// The controlled partner of `sixty_four_track_dispatch_only`. The two rows execute the same
    /// instructions over the same lanes -- both elide their prepared-identity input sections, both
    /// run `gain_mute_block` and `matrix2x2_block` unconditionally -- and differ only in the
    /// *constants* those two kernels carry: 0 dB and hard identity there, the fixture's declared
    /// per-channel fader trims and pan positions here.
    ///
    /// That makes the pair a direct measurement of a claim the floor table asserts and nothing had
    /// yet tested: a 0 dB fader and a settled identity matrix cost exactly what a real one costs,
    /// because neither kernel has an identity arm. The two rows share a floor (22 lane-ops) for
    /// precisely that reason, and a material gap between them would mean one of the two kernels
    /// had acquired a data-dependent path.
    SixtyFourTrackGainPanOnly,
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
    /// `tools/miso-engine-console-workload/tests/chain_shape.rs`: a mixed cohort must realise the
    /// same `[chains, slots]` and the same planar/AoSoA round-trip count as a uniform one. The
    /// wasm host reports no shape, which is why that gate lives beside the fixtures rather than in
    /// the record.
    SixtyFourTrackConsoleHalfMono,
}

/// The standing session workloads, in the order their records are emitted.
///
/// **Append-only.** The wasm console guest is addressed by *index* into this array
/// (`miso_console_prepare(index)`), so reordering it silently re-labels every wasm record. New
/// rows go on the end.
pub const WORKLOADS: [Workload; 16] = [
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
    Workload::SixtyFourTrackPlumbingOnly,
    Workload::SixtyFourTrackGainPanOnly,
    Workload::SixtyFourTrackConsoleMono,
    Workload::SixtyFourTrackConsoleMonoDual,
    Workload::SixtyFourTrackConsoleHalfMono,
];

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
    /// Every rack is emptied **and no builtins are prepared at all**.
    ///
    /// The one strip edit that is not only a model edit. Clearing the racks is what this arm does
    /// to the *session*; the rest of it is what [`SessionRuntime::build_full`] does with the
    /// result, which is to take the builtins-less compile path
    /// (`GraphCompiler::compile`) instead of `compile_with_builtins`. The track's declared
    /// builtins, fader and pan are left exactly as the fixture wrote them and are simply never
    /// prepared, so nothing here neutralises a coefficient that a later reader might mistake for a
    /// measured identity.
    PlumbingOnly,
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
fn retain_effect(rack: &mut miso_engine_session::Rack, effect_id: &str) {
    rack.effects.retain(|effect| {
        matches!(
            &effect.identity,
            miso_engine_session::EffectIdentity::Native { effect_id: id }
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
            Self::SixtyFourTrackPlumbingOnly => "sixty_four_track_plumbing_only",
            Self::SixtyFourTrackGainPanOnly => "sixty_four_track_gain_pan_only",
            Self::SixtyFourTrackConsoleMono => "sixty_four_track_console_mono",
            Self::SixtyFourTrackConsoleMonoDual => "sixty_four_track_console_mono_dual",
            Self::SixtyFourTrackConsoleHalfMono => "sixty_four_track_console_half_mono",
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
            Self::NineTrackBaseline => "fixtures/session/v1/parametric-eq-nine-track.toml",
            Self::SixtyFourTrackConsoleLegacy => {
                "fixtures/session/v1/console-sixty-four-track.toml"
            }
            Self::SixtyFourTrackConsoleMono
            | Self::SixtyFourTrackConsoleMonoDual
            | Self::SixtyFourTrackConsoleHalfMono => {
                "fixtures/session/v1/console-sixty-four-track-mono.toml"
            }
            _ => "fixtures/session/v1/console-sixty-four-track-intended.toml",
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
            Self::SixtyFourTrackPlumbingOnly => Strip::PlumbingOnly,
            Self::SixtyFourTrackGainPanOnly => Strip::GainPan,
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
            // Nothing of the strip is prepared on this row -- not even the input stage -- so the
            // vocabulary needs a word that is not "builtins" and not an effect list.
            Self::SixtyFourTrackPlumbingOnly => "plumbing",
            Self::SixtyFourTrackGainPanOnly => "gain+pan",
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
            | Self::SixtyFourTrackGainPanOnly => "builtins",
            // The third word of the layout vocabulary, beside `rack:slot` and `builtins`: a plan
            // with no rack effect *and* no builtin binding. It is a distinct layout rather than an
            // empty `builtins` one, because the difference between it and the `builtins` rows is
            // exactly what the row measures.
            Self::SixtyFourTrackPlumbingOnly => "plumbing",
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
fn apply_strip(model: &mut SessionTomlV1, strip: Strip) {
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
            // The racks go and nothing else does. What separates this row from `BuiltinsOnly` is
            // not an edit to the session at all: it is that `build_full` never prepares builtins
            // for it. Neutralising the declared trims and cutoffs here would be worse than
            // pointless -- nothing reads them, and a later reader would take the zeros as a
            // measured identity rather than as an unprepared declaration.
            Strip::PlumbingOnly | Strip::BuiltinsOnly => {
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
                // declared fader trims and pan positions; `Identity` asks both kernels for the
                // value that would let them do nothing, which neither of them has an arm for.
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
    /// Level 1: no lane exists. `attach_effect_observation_v1` is never called.
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
fn console_model(workload: Workload) -> SessionTomlV1 {
    let text = match workload {
        Workload::NineTrackBaseline => NINE_TRACK,
        Workload::SixtyFourTrackConsoleLegacy => SIXTY_FOUR_TRACK_LEGACY,
        Workload::SixtyFourTrackConsoleMono
        | Workload::SixtyFourTrackConsoleMonoDual
        | Workload::SixtyFourTrackConsoleHalfMono => SIXTY_FOUR_TRACK_MONO,
        _ => SIXTY_FOUR_TRACK,
    };
    let mut model = parse_session_toml(text).expect("frozen console session fixture");
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
/// This is the shape `miso-engine-host-core` prepares for a real console session: handles are
/// `index + 1` so they are nonzero and stable, the tap is the one a console meters by default, and
/// the window is [`WINDOW_BLOCKS`] blocks. Nothing here is a benchmark convenience -- an arm that
/// metered a shape no host prepares would report a cost nobody pays.
fn meter_requests(model: &SessionTomlV1) -> Vec<MeterRequest> {
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

/// A block the plan refused, or an output buffer it could not be given.
///
/// Deliberately opaque, and deliberately not a `Result<_, ()>`: a driver counts these into a
/// record's `render_errors` rather than branching on why one happened, and the plan's own error
/// taxonomy is not something a benchmark record reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderFailed;

/// One prepared console arm: a real [`PreparedRenderPlan`] and the buffer it renders into.
///
/// Built by [`SessionRuntime::build`] from a [`Workload`] and a [`PlanConfig`], through the
/// production compile and attach entry points in the order a host calls them. The only thing a
/// driver may do inside a clock is [`SessionRuntime::render`]; every other method on this type is
/// evidence collection and belongs outside it.
pub struct SessionRuntime {
    plan: PreparedRenderPlan,
    output: Vec<f32>,
    /// Control-side halves, held for the arms that attached them. Never touched inside the clock.
    meter_consumers: Vec<MeterConsumer>,
    controls: Vec<EffectControlProducerV1>,
    observations: Vec<EffectObservationHandleV1>,
    /// Tracks whose *structural* channel-symmetry witness holds, taken at compile time.
    ///
    /// The `SOURCE` term is a function over the compiled session rather than a field of the
    /// prepared plan (`session_structural_symmetry_v1` says why: the cohort planner needs the
    /// class before any prepared object exists), so it cannot be read back off the plan the way
    /// [`SessionRuntime::symmetry_counters`] reads the rest of the witness. It is taken once, in
    /// `build_full`, and kept.
    structural_mono_tracks: u64,
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
        // The overhead floor row prepares no builtins at all, so it can carry no console facility
        // either: a meter stream is leased from the prepared builtins session and the record would
        // otherwise claim a facility that was silently dropped. Every arm that asks for one is
        // taken on `SixtyFourTrackConsole`, so this refusal is unreachable rather than limiting,
        // and it fails loudly instead of measuring something other than what it says.
        let plumbing_only = workload.strip() == Strip::PlumbingOnly;
        assert!(
            !plumbing_only || config == PlanConfig::BASELINE,
            "{}: the builtins-less row cannot carry a console facility",
            workload.kind()
        );
        let meters = if config.meters {
            meter_requests(&model)
        } else {
            Vec::new()
        };
        let registry = launch_native_effect_registry_v1().expect("launch effect registry");
        let mut effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .expect("prepared console effects");
        // Both attaches are the production entry points, called in the order a host calls them.
        // The control channel is attached for every observation arm including `Absent`, so the
        // paired delta between the arms is the observation lane and not the control queue drain.
        let controls = if config.control {
            attach_effect_console_v1(
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
            attach_effect_observation_v1(&mut effects, MAXIMUM_OBSERVATION_TAPS, WINDOW_BLOCKS)
                .expect("effect observation capacity")
        };
        let silent = workload.input_signal() == "silence";
        let mappings = channel_mappings(&model);
        let (plan, meter_consumers) = if plumbing_only {
            // The builtins-less compile path. `GraphCompiler::compile` is the same entry point
            // every non-console graph is built through and is not a benchmark-only shape: what it
            // produces here is the session's own dataflow with nothing attached to the track
            // stages, so each `TrackStage` lowers to an elided alias and the route and the master
            // reduction are all that stand between a track's source and the output.
            let compiled = GraphCompiler::compile(GraphCompileRequest {
                dispatch,
                plan_id: PLAN_ID,
                effects,
                caps: graph_caps(),
            })
            .unwrap_or_else(|_| panic!("{}: builtins-less console graph", workload.kind()));
            let graph = compiled.graph;
            let envelope = graph.envelope;
            let nodes = graph
                .required_bindings
                .iter()
                .map(|node| {
                    GraphNodeBinding::new(
                        node.clone(),
                        source_binding(node, silent, &source, &mappings),
                    )
                })
                .collect();
            let plan = graph
                .bind(GraphRuntimeBindings {
                    envelope,
                    nodes,
                    observers: Vec::new(),
                })
                .unwrap_or_else(|_| panic!("{}: console graph bindings", workload.kind()));
            (plan, Vec::new())
        } else {
            let builtins = miso_engine_builtins_compiler::prepare_session_builtins(
                &session,
                &meters,
                builtin_caps(),
            )
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
            let nodes = artifact
                .external_binding_nodes()
                .map(|node| {
                    GraphNodeBinding::new(
                        node.clone(),
                        source_binding(node, silent, &source, &mappings),
                    )
                })
                .collect();
            // `observers` stays empty on purpose: it is the *external* observer slot. A meter
            // observer is compiler-owned and is appended to this vector by the sealed builtins
            // artifact inside `into_bound`, which is why `meters: true` is expressed as a meter
            // *request* and not as a hand-built observer. Driving the real path is the whole point
            // of the arm.
            let bound = artifact
                .into_bound(GraphRuntimeBindings {
                    envelope,
                    nodes,
                    observers: Vec::new(),
                })
                .unwrap_or_else(|_| panic!("{}: console graph bindings", workload.kind()));
            (bound.plan, bound.meter_consumers)
        };
        assert_eq!(
            meter_consumers.len(),
            meters.len(),
            "{}: every requested meter stream must reach the plan",
            workload.kind()
        );

        let mut runtime = Self {
            plan,
            output: vec![0.0; QUANTUM * 2],
            meter_consumers,
            controls,
            observations,
            structural_mono_tracks: miso_engine_builtins_compiler::session_structural_symmetry_v1(
                &session,
            )
            .iter()
            .filter(|(_, witness)| witness.eligible())
            .count() as u64,
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
                    .producer
                    .try_push(EffectControlRecordV1::Observe {
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
    /// channel: [`attach_effect_console_v1`] returns channels in prepared-entry order, which is
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

    /// Session-stable `(track_id, effect_id)` identity of one prepared control channel.
    ///
    /// So a record can *name* the track and slot it automated instead of asserting one in prose.
    #[must_use]
    pub fn control_identity(&self, channel: usize) -> (&str, &str) {
        let producer = &self.controls[channel];
        (&producer.track_id, &producer.effect_id)
    }

    /// Pushes one live-console parameter retarget into one prepared effect's bounded queue.
    ///
    /// This is the production control path and nothing else: the record is drained by the render
    /// thread at the top of the next block and staged as a single
    /// [`AutomationSpanKind::Point`](miso_engine_effect_contract::AutomationSpanKind) span at that
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
        self.controls[channel]
            .producer
            .try_push(EffectControlRecordV1::Parameter {
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
    pub const fn structural_mono_tracks(&self) -> u64 {
        self.structural_mono_tracks
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

    /// Bank-chain lanes whose route and master accumulation this plan folded into the chain's own
    /// epilogue (issue #218).
    ///
    /// A count, for the reason every other shape number here is a count: the fold renders the same
    /// bits by construction, so nothing but a count can say whether it fired.
    #[must_use]
    pub fn bank_route_folds(&self) -> u64 {
        self.plan.bank_route_folds()
    }

    /// Meter frames drained from every stream. Outside the clock, like every evidence step.
    pub fn drain_meters(&mut self) -> u64 {
        let mut frames = 0;
        for stream in &mut self.meter_consumers {
            while stream.consumer.try_pop().is_ok() {
                frames += 1;
            }
        }
        frames
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
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
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
        for value in &self.output {
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
fn synthesise_tracks(model: &mut miso_engine_session::SessionTomlV1, tracks: usize) {
    let template: Vec<_> = model.tracks.clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template[index % template.len()].clone();
        track.id = StableId::parse(&format!("ch{index:03}")).expect("synthetic track id");
        let mut next = route.clone();
        next.id = StableId::parse(&format!("ch{index:03}-main")).expect("synthetic route id");
        next.source = miso_engine_session::RouteSource::Track {
            track_id: track.id.clone(),
            tap: miso_engine_session::SendTap::PostMatrix,
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

fn builtin_caps() -> miso_engine_builtins_compiler::BuiltinCompileCaps {
    miso_engine_builtins_compiler::BuiltinCompileCaps {
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

fn graph_caps() -> miso_engine_graph::GraphCompileCaps {
    miso_engine_graph::GraphCompileCaps {
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
            let value = ((frame as f32) * 0.017 + track as f32 * 0.31).sin() * 0.6;
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
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        block.left.copy_from_slice(&self.left);
        block.right.copy_from_slice(&self.right);
        Ok(())
    }
}

struct GraphIdentity;

impl GraphRuntimeProcessor for GraphIdentity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

/// Every track's declared `(left_source_channel, right_source_channel)`, in model order.
///
/// Read from the compiled model rather than assumed, because it is the field the mono fixture
/// moves and the field the `half_mono` row moves back on half its tracks. See
/// [`FrozenGraphSource::from_block`] for why the subject honours it instead of always writing a
/// stereo pair.
fn channel_mappings(model: &SessionTomlV1) -> Vec<(usize, usize)> {
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

fn source_binding(
    node: &GraphNodeId,
    silent: bool,
    source: &SourceSignal,
    mappings: &[(usize, usize)],
) -> Box<dyn GraphRuntimeProcessor> {
    if let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    {
        let id = track_id.as_str();
        let track = id
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
        Box::new(FrozenGraphSource::from_block(&block, mapping))
    } else {
        Box::new(GraphIdentity)
    }
}
