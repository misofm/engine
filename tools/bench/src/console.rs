//! Issue #149 item 3: the standing console qualification benchmark.
//!
//! # Why this subject exists
//!
//! The number the sprint inherited is a ragged nine-track EQ-only session. Nine tracks at the
//! launch eight-lane width is one full bank plus a one-track scalar tail -- the worst shape a bank
//! can be asked to render, and not the shape of a mixing session. This subject measures the shape
//! a session actually has: sixty-four tracks, each carrying the full channel strip (input
//! trim/HPF/LPF, a parametric EQ in SIMD rack 1, a compressor in the dynamic rack, a fader and a
//! pan matrix), rendered through a real [`engine::realtime::PreparedRenderPlan`] at
//! 48 kHz and a 128-frame quantum.
//!
//! Sixty-four tracks is eight full banks and no tail, so the per-track cost reported here is the
//! cost of a full bank rather than the cost of a remainder. The nine-track fixture is kept as a
//! workload so the two numbers are produced by one binary, on one host, in one run -- the only way
//! the comparison means anything.
//!
//! # The measurement discipline
//!
//! Frozen before timing and not tuned afterwards (AGENTS.md): fixed fixtures, fixed observation
//! count, one warmup pass and two measured rounds supplied by the runner. Percentiles are the
//! shared nearest-rank implementation, and the timed region is `timing::timed`, which panics if
//! the body hashed anything -- so evidence collection cannot drift inside the clock (#104 F1).
//!
//! ## Paired alternation
//!
//! The hoist arms are **interleaved observation by observation**, not run one after the other. A
//! benchmark that renders all of arm A and then all of arm B measures the difference between two
//! points in time as much as the difference between two arms: the host's clock ramps, another
//! tenant lands on the sibling core, a thermal limit engages. Alternating means every drift a run
//! suffers is shared by both arms, and the per-observation pairing makes the delta a distribution
//! rather than a difference of two summaries.
//!
//! ## What the hoist arms measure
//!
//! Issue #144 item 6 elides the smoothing window a parameter opens when it is retargeted to the
//! value it already holds. The two arms deliver the *same* automation traffic and differ only in
//! whether that traffic is stationary:
//!
//! * `restated` sends every parameter its current value on every block. The hoist settles it.
//! * `moving` alternates each parameter between two nearby values on every block, so a real
//!   window is open at all times and the hoist can never fire.
//!
//! A first attempt used a one-ULP move for the control arm and it did **not** work, which is worth
//! recording: a one-ULP change in a dB gain designs to the *same* `f32` SVF coefficient words, so
//! the EQ's hoist fired on the designed words and the "control" arm was hoisted too. That is a
//! real property of the optimisation -- it catches redundancy at the coefficient level, not only
//! at the parameter level -- but it makes one ULP useless as a control. The moving arm therefore
//! uses the smallest step that actually changes the designed words.
//!
//! The two arms deliver the same number of spans and do the same control-plane work, so the delta
//! between them is the ramping decision and the window it opens. `moving` is a faithful cost
//! stand-in for the pre-hoist behaviour of `restated`, which opened exactly that window with a
//! step of `+0.0` -- the same ramped kernel over the same lanes for the same sixty-four samples.
//! Their *audio* differs, and only `quiet` and `restated` are asserted bit-identical.
//!
//! `quiet` delivers no automation at all and is the standing baseline row.
//!
//! # The decomposition rows (issue #163 item 0c)
//!
//! One number for a whole console strip says how long the block took and nothing about where the
//! time went. The five rows added here answer that by subtraction, and they can only be subtracted
//! because every one of them is the *same* fixture with part of the strip removed in code:
//! `sixty_four_track_eq_only`, `sixty_four_track_compressor_only`, `sixty_four_track_builtins_only`,
//! `sixty_four_track_dispatch_only` and `sixty_four_track_idle`. Same tracks, same parameters, same
//! sources, same binary, same run.
//!
//! They are derived from the checked-in model by [`console_workload`]'s strip
//! edits rather than being five more
//! 900-line TOMLs, for the reason the 128-track stretch fixture already gives: nothing about an
//! emptied rack is a new *shape* to review, and five near-duplicate fixtures would be five files
//! that can drift apart from the one they were copied from. Every derived row says
//! `synthetic_fixture: true` and names what its strip carries, and the validator pins that pairing
//! per workload kind, so a row cannot quietly claim a rack it emptied.
//!
//! # The console-facility arms (issue #163 item 0d)
//!
//! Every session row above renders a plan with no meter stream, no live-control channel
//! and no observation capacity -- which is not what a running console is. `console_meters` and
//! `console_observation` measure what those facilities cost, under the same paired-alternation
//! protocol as the hoist arms, and through the production entry points rather than a hand-built
//! stand-in: a meter is a `MeterRequest` handed to `prepare_session_builtins`, and observation is
//! `attach_effect_observation` plus an `EffectControlRecord::Observe` pushed through the same
//! bounded queue a host pushes it through.
//!
//! `console_observation` measures the issue #143 two-level zero rather than restating it: `absent`
//! has no lane at all, `unarmed` has a lane with nothing armed, `armed` has every declared tap
//! armed. All three carry the control channel, so the deltas are the lane and the arming and never
//! the queue drain.
//!
//! Both records carry a class-A statement asserted in-run: observing a console must not change
//! what the console renders, so every arm of both measurements must produce byte-identical output.
//!
//! That statement cannot say whether the metered plan is the *same shape* as the unmetered one: the
//! route fold and the scatter redirect render the bits of the path they replace, so a meters-on arm
//! that stopped folding would pass it and charge the lost fold to metering. `console_meters`
//! therefore states each arm's `bank_route_folds` and `bank_scatter_redirects` (issue #914), read
//! once per arm after the warm-up, and the validator requires the two arms to agree and every
//! route of the console to fold.
//!
//! # The metered console row (issue #881)
//!
//! `console_meters` meters the console the way no browser does: all-metric meters at a four-block
//! window, prepared through the concurrent entry. `sixty_four_track_console_metered` is a session
//! row that meters it the way the default web boot does -- one `SAMPLE_PEAK` meter at `PostMatrix`
//! per track, a twelve-block window, no hold, no decay, bound as permanent observers through
//! `prepare_selected_session_builtins_with_live_controls` with the product's `Concurrent` delivery
//! -- so the observer path the browser pays for has a row that can move. Both rows run the same 48
//! chain stages (pinned in `console-workload`'s pair test), so the row minus
//! `sixty_four_track_console` is the meters alone. It is timed exactly like every other session
//! row: the clock holds the render call alone, and the row's meter streams are drained of every
//! snapshot after each block, outside the clock, where the output identity is also taken. Its
//! record adds a meter group (streams, tap, metric set, window, snapshots consumed, snapshots
//! dropped) and the plan's `bank_route_folds` and `bank_scatter_redirects`, read once at bind; the
//! rows' digests are asserted equal to `sixty_four_track_console`'s in-run, because a meter
//! observes and never changes signal flow.
//!
//! # The automation-active row (`console_automation`)
//!
//! Every session row above renders with automation **cleared**: `console_model` empties the
//! fixture's table unconditionally, and both fixture gates assert the standing sessions declare
//! none. The compressor's ramping body is therefore dead code in the whole standing table, and the
//! one arm that does deliver spans -- `console_hoist` -- drives banks of parametric EQs. No
//! compressor in this benchmark had ever seen an automation span, so no change to a compressor's
//! ramping path could move a number in it.
//!
//! `console_automation` closes that gap with the console's real traffic shape: **one Point span
//! per block, on one track**, pushed through the same bounded live-control queue a host pushes
//! through, into a real prepared plan. Its three arms carry the identical control channel and
//! differ only in what rides it -- `quiet` pushes nothing, `restated` restates the value in force
//! (the #144 hoist settles it, so no window opens), `automated` moves it every block. The paired
//! ramp delta is `automated - restated`: the same queue drain, the same span count, and a
//! smoothing window open in one arm and not the other. `quiet == restated` is the class-A
//! statement, asserted in-run.
//!
//! # The mixing-automation row (`console_mixing_automation`, issue #1003)
//!
//! `console_automation` rides one compressor threshold on one track of the compressor-only row.
//! A mixing session rides EQ gains, compressor thresholds and limiter ceilings across the console,
//! and on the mono console -- the product's common case -- that traffic is where three costs no
//! other row sees live: the EQ's whole-bank ramping path, the mono collapse a one-channel record
//! retires for the life of the plan, and the fact that tracks spread across banks each take a
//! bank down.
//!
//! This row is the mono fixture as written, with the live-control channel, riding eight
//! controls on eight tracks, one per eight-lane bank (`console_workload::mixing_automation` is the
//! table). Each control is pushed in the shape a real host pushes it: the EQ as one owner edit on
//! `Both`, which designs one `Both` target, and the compressor and the limiter as a Left then a
//! Right record. Its three arms alternate per observation, and their deltas are named for what
//! they measure: `restated - quiet` is the collapse the restating records retire, and `automated -
//! restated` is the ramping. Every base is the held value, so `quiet == restated` is the collapse's
//! own class-A statement, asserted in-run. An untimed preflight asserts, before any number is
//! taken, that every automated effect moves bits on its own, and that the EQ's restatement keeps
//! every cohort collapsed (`bench console --preflight` runs it alone). The row states each arm's
//! `bank_collapse_counters`, because a collapse renders the bits a dual bank renders and nothing
//! else can say whether it held.
//!
//! # The bus-and-send row (issue #1227)
//!
//! `sixty_four_track_console_sends` is the standing sixty-four-track console's tracks routed
//! through eight processed buses and two effect returns, each track also sending pre- and
//! post-fader, from its committed fixture
//! (`fixtures/session/v1/console-sixty-four-track-sends.json`, derived by
//! `scripts/derive-sends-console-fixture.py`). It is emitted after the metered row and is timed
//! like every other session row, at [`PlanConfig::BASELINE`] with its sources bound: a
//! plan prepared without live controls, the C ABI's plan less the fader and matrix lanes it
//! attaches (#1256); routes stay static there too. It
//! states no floor (`floor::floor_row` says why). Its digest is asserted in-run to differ from
//! `sixty_four_track_console`'s (`SENDS_PAIR`), because a sends row that rendered the standing
//! console's bits would not have compiled its own fixture.
//!
//! # The console-strip rows (issue #1085)
//!
//! Five session rows, emitted after the bus-and-send row (#1227), freeze the shapes the console
//! strip (decision 12) changes before any of its slices lands, so the same rows can be timed on
//! today's engine and on the console model: the standing strip at ten, thirteen and sixteen tracks
//! (with the nine- and sixty-four-track rows, the strip at N in {9, 10, 13, 16, 64}); the app
//! shape, whose every track carries EQ -> compressor with a third of them bypassed (B0 wrote it
//! into `dynamic`; since #1093 it is the session's two `pre_insert` slots, with the same bits); and
//! sparse activity, the standing console with every odd track fed silence. They are timed exactly
//! like every other session row. The app shape's record adds the bypass pattern it observed in its
//! compiled session, and the sparse row's digest is asserted in-run to differ from both the
//! all-active and the idle row's, because a sparse row that rendered either one's bits would not
//! have been fed what it claims. None of the five states a floor (see `floor::floor_row`).
//!
//! Every session row's `strip_layout` is named in decision 12's console vocabulary since this
//! issue (`console_workload::Workload::strip_layout`), so the rows are pinned by one validator
//! rule on both shapes.
//!
//! # The fader-ride row (issue #1750)
//!
//! `console_fader_ride` is the standing console prepared with the live lanes a product host
//! attaches for it (`controls::live_lanes`) on the product's `Concurrent` delivery, riding one
//! fader and one pan in every bank chain, one transaction per block, emitted last. Its `quiet` arm
//! is the same plan with no writes, alternated per observation, so `riding - quiet` is the ride's
//! cost: the split ramping output path a live console pays today (#1754 would pair the two stages
//! and re-run it). Its per-block witness is a stepped twin replayed after the timed loop
//! (`FaderRideMeasurement` says how and `console_workload::fader_ride` what it proves), and its
//! riding digest is asserted in-run to differ from its quiet arm's.
//!
//! # The product-lane row (issue #1758)
//!
//! `console_product_lanes` is the fader-ride row's quiet arm on its own: the standing console
//! prepared with the product's live lanes and never written, rendered back to back by the session
//! rows' loop (`SessionMeasurement::run_prepared`) straight after them, and emitted last. Its p50
//! minus `sixty_four_track_console`'s is what the lanes cost a console that nobody edits, without
//! the ride row's two-plan alternation. Its digest is asserted in-run to equal the standing row's.
//!
//! Issue #1763 splits it: three more arms of the standing console attach one lane group each
//! (fader/matrix, fader/matrix plus strip input, effects only), and one arm attaches the product
//! set of `sixty_four_track_app_shape`, which has no console slot with latency. Each arm is timed
//! the same way, names its `standing_row`, and is asserted in-run to render that row's bits.

use crate::floor::{self, CoreClock};
use bench_support::alloc as bench_alloc;
use bench_support::digest::Sha256Sink;
use bench_support::json::escape as json_escape;
use bench_support::metadata::Metadata;
use bench_support::stats::{Percentiles, format_f64, microseconds};
use bench_support::timing;
use builtins::{MeterMetricSet, MeterSnapshot, MeterTap};
use console_workload::fader_ride::{self, FaderRide};
use console_workload::mixing_automation::{
    self, Lowering, MixingArm, Preflight, PushTally, ResolvedControl,
};
use console_workload::{
    BypassCensus, LaneGroup, ObservationArm, PlanConfig, QUANTUM, SAMPLE_RATE_HZ, SessionRuntime,
    SourceFeed, WINDOW_BLOCKS, Workload, native_session_rows,
};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectId, EffectQuality, EffectTargetRequest,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedEffectTarget,
    PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
};
use engine::realtime::audit;
use lane::Backend;

const OBSERVATIONS: usize = 1_000;
const ISSUE: u32 = 149;

/// The standing console row and its metered twin (issue #881): one session, prepared once through
/// the `Concurrent` entry with no meter and once with its meter set. A meter observes and never
/// changes signal flow, so their digests must agree, and the run asserts it before it emits either
/// record. Their timings differ by the meters.
const METERED_PAIR: [Workload; 2] = [
    Workload::SixtyFourTrackConsole,
    Workload::SixtyFourTrackConsoleMetered,
];

/// The gain/pan ring row and its live-producer twin (issue #938): one plan, fed by a frozen driver
/// and by a producer thread writing the same words into production rings. Their digests must agree,
/// and the run asserts it before it emits either record.
const LIVE_PAIR: [Workload; 2] = [
    Workload::SixtyFourTrackGainPanRing,
    Workload::SixtyFourTrackGainPanLive,
];

/// The session rows whose record states the bypass their compiled session carries (issue #1085):
/// the app shape alone. Every other row's record is unchanged.
const BYPASS_ROWS: [Workload; 1] = [Workload::SixtyFourTrackAppShape];

/// The sparse-activity row and the two rows it is read against (issue #1085): every track active,
/// every track silent, every odd track silent. One session, three inputs, so three digests.
const SPARSE_TRIPLE: [Workload; 3] = [
    Workload::SixtyFourTrackConsole,
    Workload::SixtyFourTrackIdle,
    Workload::SixtyFourTrackConsoleSparse,
];

/// The toggle row (issue #1783): the standing console with its odd tracks toggling between
/// silence and the tone, read against the same session with every track active and with the odd
/// tracks always silent.
const TOGGLE_TRIPLE: [Workload; 3] = [
    Workload::SixtyFourTrackConsole,
    Workload::SixtyFourTrackConsoleSparse,
    Workload::SixtyFourTrackConsoleToggle,
];

/// The bus-and-send row and the standing console row whose tracks it carries (issue #1227): one
/// set of tracks, feeding the output directly on one row and through buses and sends on the other.
/// Their digests must differ, and the run asserts it before it emits either record: an equality
/// means the sends fixture never reached the plan.
const SENDS_PAIR: [Workload; 2] = [
    Workload::SixtyFourTrackConsole,
    Workload::SixtyFourTrackConsoleSends,
];

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    match arguments.as_slice() {
        [] => {}
        // Issue #1003: the mixing-automation row's untimed preflight, alone. It takes no round
        // marker because it produces no record.
        [flag] if flag == "--preflight" => {
            mixing_preflight(Backend::current());
            return;
        }
        _ => panic!("benchmark accepts no arguments but --preflight"),
    }
    let round = round_from_runner();
    // Feature detection ends here, before every timed observation.
    let backend = Backend::current();
    let metadata = &Metadata::gather();
    // Issue #938: this thread renders every row, pinned to the runner's core before any plan is
    // built; the live row's producer goes to the one other core of the process mask.
    let placement = affinity::Placement::pin_render_thread(metadata);
    placement.refuse_unplaced_producer(metadata);

    // Every workload is measured before any record is emitted. A decomposition row's floor
    // columns subtract the control row the floor table names, and a subtraction between two rows
    // means something only when both were measured in one process, on one host, in one round --
    // the rule #163 item 0c already states for the decomposition itself. Emission is untimed, and
    // its order is unchanged.
    let clock = CoreClock::from_runner(metadata);
    // Every `native_session_rows()` row: the standing rows, then the gain/pan row (issue #956),
    // the metered row (issue #881), the bus-and-send row (issue #1227), the console-strip rows
    // (issue #1085) and the packing rows (issue #1783). One list, so the floor subtraction below
    // finds a control wherever it sits.
    let rows: Vec<Workload> = native_session_rows().collect();
    let sessions: Vec<SessionMeasurement> = rows
        .iter()
        .map(|workload| SessionMeasurement::run(*workload, &placement))
        .collect();
    // Issue #1758: the standing console prepared with the product's lanes, timed by the session
    // rows' own loop straight after them, and issue #1763's lane-group arms. Emitted last.
    let product_lanes: Vec<ProductLanesMeasurement> = PRODUCT_LANE_ARMS
        .iter()
        .map(ProductLanesMeasurement::run)
        .collect();
    let digest_of = |workload: Workload| {
        rows.iter()
            .position(|row| *row == workload)
            .map(|index| &sessions[index].output_sha256)
            .expect("both rows of the pair are measured")
    };
    // The class-A statement of the metered row (issue #881), asserted in-run like the facility
    // arms' before a number is published: a meter observes, so the metered console renders the
    // standing console's bits.
    let [unmetered, metered] = METERED_PAIR.map(digest_of);
    assert_eq!(
        metered, unmetered,
        "the metered console row rendered different output: a meter changed the signal"
    );
    // And the live row's (issue #938): its producer submits the ring row's frozen words into
    // production rings, so it renders the ring row's bits. A difference is a feed defect, and the
    // row would publish the cost of other input under its name.
    let [ring, live] = LIVE_PAIR.map(digest_of);
    assert_eq!(
        live, ring,
        "the live-producer row rendered different output from the ring row"
    );
    // And the sparse row's (issue #1085): half its inputs are silent, so it renders neither the
    // all-active row's bits nor the idle row's. Either equality means the feed never reached the
    // plan, and the row would publish another row's cost under its name.
    let [active, idle, sparse] = SPARSE_TRIPLE.map(digest_of);
    assert!(
        sparse != active && sparse != idle,
        "the sparse-activity row rendered the all-active or the idle row's output"
    );
    // And the toggle row's (issue #1783): its odd tracks spend half the run silent and half on
    // the tone, so it renders neither the all-active row's bits nor the sparse row's. An equality
    // with the sparse row means the driver never toggled.
    let [active, sparse, toggle] = TOGGLE_TRIPLE.map(digest_of);
    assert!(
        toggle != active && toggle != sparse,
        "the toggle row rendered the all-active or the sparse row's output"
    );
    // And the bus-and-send row's (issue #1227): its tracks feed buses and sends, so it does not
    // render the standing console's bits. An equality means its fixture never reached the plan.
    let [standing, sends] = SENDS_PAIR.map(digest_of);
    assert!(
        sends != standing,
        "the bus-and-send row rendered the standing console row's output"
    );
    // And the product-lane rows' (issues #1758, #1763): live lanes that nobody writes read the
    // values the plan was prepared with, so each arm renders its standing row's bits over the same
    // block window. A difference is a harness defect, never a finding.
    for arm in &product_lanes {
        assert_eq!(
            &arm.session.output_sha256,
            digest_of(arm.arm.workload),
            "the {} arm rendered different output from its standing row",
            arm.arm.kind
        );
    }
    for (workload, session) in rows.iter().zip(&sessions) {
        let control = floor::floor_row(*workload)
            .and_then(|row| row.control)
            .and_then(|control| rows.iter().position(|row| *row == control))
            .map(|index| sessions[index].p50_ns());
        println!(
            "{}",
            session.record(*workload, round, backend, metadata, clock.as_ref(), control)
        );
    }
    for workload in [
        Workload::NineTrackRaggedStrip,
        Workload::SixtyFourTrackConsole,
    ] {
        let hoist = HoistMeasurement::run(workload, backend);
        println!("{}", hoist.record(workload, round, backend, metadata));
    }
    let meters = FacilityMeasurement::run(&METER_CONFIGS);
    println!("{}", meters.meters_record(round, backend, metadata));
    let observation = FacilityMeasurement::run(&OBSERVATION_CONFIGS);
    println!(
        "{}",
        observation.observation_record(round, backend, metadata)
    );
    let placement = PlacementMeasurement::run();
    println!("{}", placement.record(round, backend, metadata));
    let automation = AutomationMeasurement::run();
    println!("{}", automation.record(round, backend, metadata));
    let mono = MonoMeasurement::run();
    println!("{}", mono.record(round, backend, metadata));
    let mixing = MixingAutomationMeasurement::run(backend);
    println!("{}", mixing.record(round, backend, metadata));
    let ride = FaderRideMeasurement::run();
    println!("{}", ride.record(round, backend, metadata));
    for arm in &product_lanes {
        println!("{}", arm.record(round, backend, metadata));
    }
}

/// `bench console --preflight`: the mixing-automation row's premises, untimed, with what they
/// resolved. Panics on the first premise that fails, so a nonzero exit is the refusal.
fn mixing_preflight(backend: Backend) {
    let preflight = mixing_automation::preflight(backend);
    for control in &preflight.controls {
        println!(
            "control {} {} {} channel {} base {} values {:?} lowering {}",
            control.control.track_id,
            control.slot_id,
            control.control.parameter,
            control.channel,
            control.base,
            control.values,
            control.control.effect.lowering().name()
        );
    }
    for (index, (arm, effects)) in mixing_automation::PREFLIGHT_ARMS.iter().enumerate() {
        println!(
            "arm {} digest {} bank_collapse_counters {:?} pushes {}/{}",
            mixing_automation::preflight_arm_name(*arm, *effects),
            preflight.digests[index],
            preflight.collapse[index],
            preflight.tallies[index].accepted,
            preflight.tallies[index].attempted
        );
    }
    preflight.assert_premises();
    println!(
        "console_mixing_automation preflight: PASS ({} controls resolved, backend {})",
        preflight.controls.len(),
        backend_name(backend)
    );
}

fn round_from_runner() -> u32 {
    match Metadata::gather().var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the console benchmark must be launched by its fixed runner"),
    }
}

// ---------------------------------------------------------------------------------------------
// The standing session measurement: a real prepared plan, rendered block by block.
// ---------------------------------------------------------------------------------------------

struct SessionMeasurement {
    /// Timed observations. [`OBSERVATIONS`] in every run the runner launches; the record states
    /// this rather than the constant, so a shortened run cannot pass for the frozen one.
    observations: usize,
    ns_per_block: Vec<u64>,
    output_sha256: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
    /// The metered row's meter evidence (issue #881), and `None` on every other row, whose record
    /// is unchanged.
    meters: Option<MeteredEvidence>,
    /// The app shape's bypass census (issue #1085), read at bind, and `None` on every other row.
    bypass: Option<BypassCensus>,
    /// The live row's producer placement and underruns (issue #938), and `None` on every other
    /// row, whose record is unchanged.
    live: Option<LiveEvidence>,
}

/// What the live-producer row states beside its timing (issue #938).
struct LiveEvidence {
    /// The core the producer thread was pinned to, or `None` when it was not pinned.
    producer_cpu: Option<usize>,
    /// Where that core sits relative to the render core ([`affinity::Placement::name`]).
    placement: &'static str,
    /// Missing in-region frames over every ring, warmup and timed blocks together.
    underruns: u64,
}

impl LiveEvidence {
    /// The live row's producer group, with the trailing comma a splice needs.
    fn record_fields(&self) -> String {
        format!(
            "\"producer_cpu\":{cpu},\"producer_placement\":\"{placement}\",\"underruns\":{underruns},",
            cpu = self
                .producer_cpu
                .map_or_else(|| "null".to_owned(), |cpu| cpu.to_string()),
            placement = self.placement,
            underruns = self.underruns,
        )
    }
}

/// What the metered console row states beside its timing (issue #881).
///
/// Every field is observed rather than restated, all of it outside the clock: the streams and
/// their taps and the two plan-shape counters are read once, at bind, where they are fixed, and
/// the rest is folded in from every snapshot the timed blocks publish, drained after each block.
struct MeteredEvidence {
    /// Meter streams bound into the plan: one per track.
    streams: u64,
    /// Whether every bound stream observes the post-matrix tap.
    post_matrix: bool,
    /// The presence mask and window length, in frames, of the first consumed snapshot.
    shape: Option<(MeterMetricSet, u32)>,
    /// Whether every later snapshot carried the first one's presence mask and window length.
    uniform: bool,
    /// Snapshots consumed over the timed blocks. With every stream drained after every block, this
    /// is exactly `streams * floor(observations / WEB_METER_BLOCKS)`: one per closed window.
    snapshots: u64,
    /// Per stream, in handle order, the cumulative drop counter its last consumed snapshot
    /// carried. Sized at bind, so consuming a snapshot allocates nothing.
    stream_dropped: Vec<u64>,
    /// Bank-chain lanes whose route this plan folded into its chain's epilogue (issue #914).
    bank_route_folds: u64,
    /// Bank-chain lanes whose scatter this plan pointed straight at its consumer (issue #914).
    bank_scatter_redirects: u64,
}

impl SessionMeasurement {
    /// One session row of the runner's run, its live producer (issue #938) on `placement`'s core.
    fn run(workload: Workload, placement: &affinity::Placement) -> Self {
        Self::run_placed(workload, placement, OBSERVATIONS)
    }

    /// [`SessionMeasurement::run`] over `observations` timed blocks with an unpinned producer.
    /// Only the unit tests below call it, and the record it produces says so in its
    /// `observations` field, which the validator pins to the frozen count.
    #[cfg(test)]
    fn run_for(workload: Workload, observations: usize) -> Self {
        Self::run_placed(workload, &affinity::Placement::UNPINNED, observations)
    }

    /// [`SessionMeasurement::run`] over `observations` timed blocks.
    ///
    /// The live row (issue #938) starts its producer here, pinned by `placement`, before the
    /// warmup and the clock, and joins it after the last timed block; its record states where the
    /// producer ran and how many frames its rings underran.
    fn run_placed(
        workload: Workload,
        placement: &affinity::Placement,
        observations: usize,
    ) -> Self {
        let mut runtime = SessionRuntime::new(workload);
        let live = workload.source_feed() == SourceFeed::LiveProducer;
        if live {
            runtime.start_live_producer(placement.producer_pin());
        }
        let mut measured = Self::run_prepared(workload, &mut runtime, observations);
        if live {
            let report = runtime.finish_live_producer();
            measured.live = Some(LiveEvidence {
                producer_cpu: placement.producer_cpu,
                placement: placement.name,
                underruns: report.underrun_frames,
            });
        }
        measured
    }

    /// The timed loop of every session row, over a runtime the caller prepared: one block of the
    /// production render entry per observation, back to back. The product-lane row (issue #1758)
    /// passes the standing console prepared with the product's lanes, so the two rows are timed by
    /// this one loop and differ only in how their plan was prepared.
    fn run_prepared(workload: Workload, runtime: &mut SessionRuntime, observations: usize) -> Self {
        let mut durations = Vec::with_capacity(observations);
        let mut output_hash = Sha256Sink::new();
        let mut render_errors = 0_u64;
        // Issue #881: the metered row's evidence, bound here, before the clock starts. `None` on
        // every other row, whose loop below is unchanged.
        let mut meters = workload
            .web_meters()
            .then(|| MeteredEvidence::bind(runtime));
        // Issue #1085: fixed at compile time, so read once, before the clock starts.
        let bypass = BYPASS_ROWS
            .contains(&workload)
            .then(|| runtime.bypass_census());
        // Untimed settling. Only the idle row asks for any, and it asks for a lot: see
        // `Workload::warmup_blocks`.
        for observation in 0..workload.warmup_blocks() {
            runtime.await_live_feed(observation as u64);
            let _ = runtime.render(observation as u64);
        }
        audit::warm_up();
        audit::reset();
        for observation in 0..observations {
            // Issue #938: the live row's rings hold this block and more before the clock starts.
            // A no-op on every other row.
            runtime.await_live_feed(observation as u64);
            // The timed region is one block of the production render entry and nothing else. The
            // output identity is taken outside it, which `timing::timed` enforces structurally.
            let (elapsed_ns, result) = timing::timed(|| runtime.render(observation as u64));
            if result.is_err() {
                render_errors += 1;
            }
            runtime.hash_output(&mut output_hash);
            // Issue #881: the metered row holds its meter streams and consumes every snapshot
            // published by the block just rendered, here -- after the clock stopped and after the
            // output identity was taken, so the timed region is the render call alone, exactly as
            // on every other row.
            if let Some(evidence) = meters.as_mut() {
                runtime.drain_meter_snapshots(|snapshot| evidence.consume(snapshot));
            }
            durations.push(elapsed_ns);
        }
        Self {
            observations,
            ns_per_block: durations,
            output_sha256: output_hash.finish_hex(),
            audit: audit::snapshot(),
            render_errors,
            meters,
            bypass,
            live: None,
        }
    }

    /// The p50 nanoseconds per block: what a control row contributes to a floor subtraction.
    fn p50_ns(&self) -> u64 {
        Percentiles::from_samples(&self.ns_per_block).p50
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
        clock: Option<&CoreClock>,
        control_p50_ns: Option<u64>,
    ) -> String {
        let percentiles = Percentiles::from_samples(&self.ns_per_block);
        let tracks = f64::from(workload.tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_session\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"synthetic_fixture\":{synthetic},",
                "\"strip_content\":\"{strip}\",\"strip_layout\":\"{layout}\",",
                "\"input_signal\":\"{signal}\",\"source_feed\":\"{feed}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},\"observations\":{obs},",
                "\"units\":\"us_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"min_us_per_block\":{min},\"p50_us_per_block\":{p50},",
                "\"p95_us_per_block\":{p95},\"p99_us_per_block\":{p99},",
                "\"max_us_per_block\":{max},\"p50_us_per_block_per_track\":{per_track},",
                "\"min_ns_per_block\":{min_ns},\"p50_ns_per_block\":{p50_ns},",
                "\"p95_ns_per_block\":{p95_ns},\"p99_ns_per_block\":{p99_ns},",
                "\"max_ns_per_block\":{max_ns},\"output_sha256\":\"{digest}\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{meters}{bypass}{live}{floor}{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"nearest-rank percentiles over per-block nanoseconds; ",
                "one warmup pass and two measured rounds; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            feed = workload.source_feed().name(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = self.observations,
            min = microseconds(percentiles.min),
            p50 = microseconds(percentiles.p50),
            p95 = microseconds(percentiles.p95),
            p99 = microseconds(percentiles.p99),
            max = microseconds(percentiles.max),
            per_track = format_f64(percentiles.p50 as f64 / 1_000.0 / tracks),
            min_ns = percentiles.min,
            p50_ns = percentiles.p50,
            p95_ns = percentiles.p95,
            p99_ns = percentiles.p99,
            max_ns = percentiles.max,
            digest = self.output_sha256,
            errors = self.render_errors,
            forbidden = self.audit.total(),
            // Issue #881: the metered row's meter group. Empty on every other row, whose record
            // is byte-for-byte the shape it always had.
            meters = self
                .meters
                .as_ref()
                .map_or_else(String::new, MeteredEvidence::record_fields),
            // Issue #1085: the app shape's bypass, as its compiled session carries it. Empty on
            // every other row.
            bypass = self.bypass.map_or_else(String::new, |census| {
                format!(
                    "\"bypass_pattern\":\"{}\",\"bypassed_tracks\":{},",
                    census.pattern.name(),
                    census.bypassed_tracks
                )
            }),
            // Issue #938: the live row's producer group. Empty on every other row.
            live = self
                .live
                .as_ref()
                .map_or_else(String::new, LiveEvidence::record_fields),
            // Issue #184's floor accounting. The whole group is absent when the runner had no
            // performance counter to measure the pinned core's clock with, which is the shape
            // every sealed record already has and is what makes the columns additive.
            floor = clock.map_or_else(String::new, |clock| {
                floor::record_fields(workload, percentiles.p50, clock, control_p50_ns)
            }),
            metadata = metadata.record_fields(),
        )
    }
}

impl MeteredEvidence {
    /// Reads what the plan fixed at bind: its streams, their taps and its #914 counters.
    fn bind(runtime: &SessionRuntime) -> Self {
        Self {
            streams: runtime.meter_streams() as u64,
            post_matrix: runtime.meter_taps().all(|tap| tap == MeterTap::PostMatrix),
            shape: None,
            uniform: true,
            snapshots: 0,
            stream_dropped: vec![0; runtime.meter_streams()],
            bank_route_folds: runtime.bank_route_folds(),
            bank_scatter_redirects: runtime.bank_scatter_redirects(),
        }
    }

    /// Folds one consumed snapshot in. A handle is `index + 1` in stream order.
    fn consume(&mut self, snapshot: &MeterSnapshot) {
        self.snapshots += 1;
        let observed = (snapshot.present_metrics, snapshot.frames);
        match self.shape {
            None => self.shape = Some(observed),
            Some(first) => self.uniform &= first == observed,
        }
        let stream = usize::try_from(snapshot.handle.0.get() - 1)
            .ok()
            .and_then(|index| self.stream_dropped.get_mut(index))
            .expect("a meter handle of a bound stream");
        *stream = snapshot.cumulative_dropped_snapshots;
    }

    /// Snapshots the streams dropped: each stream's cumulative drop counter as its last consumed
    /// snapshot carried it, summed.
    fn dropped(&self) -> u64 {
        self.stream_dropped.iter().sum()
    }

    /// The metered row's meter group, with the trailing comma a splice needs.
    ///
    /// The meter shape is stated as observed, not as configured: the tap from the bound streams,
    /// and the metric set and the window from the snapshots themselves. The validator pins each to
    /// the default web boot's, so a row whose meters drifted to another tap, metric set or window
    /// states that it did (`other`, or a window of zero when the snapshots disagree) and is refused
    /// rather than publishing another meter set's cost under this row's name.
    fn record_fields(&self) -> String {
        // The row has to have metered. A silently empty meter set would report the cost of
        // nothing as the cost of the browser's meters.
        assert!(
            self.streams > 0 && self.snapshots > 0,
            "the metered row consumed no meter snapshot: it is not measuring meters"
        );
        let shape = self.shape.filter(|_| self.uniform);
        let metrics = match shape {
            Some((metrics, _)) if metrics == MeterMetricSet::SAMPLE_PEAK => "sample_peak",
            Some((metrics, _)) if metrics == MeterMetricSet::ALL => "all",
            _ => "other",
        };
        let window_blocks = shape
            .map(|(_, frames)| frames as usize)
            .filter(|frames| frames % QUANTUM == 0)
            .map_or(0, |frames| frames / QUANTUM);
        format!(
            concat!(
                "\"meter_streams\":{streams},\"meter_tap\":\"{tap}\",",
                "\"meter_metrics\":\"{metrics}\",\"meter_window_blocks\":{window},",
                "\"meter_snapshots\":{snapshots},\"meter_dropped_snapshots\":{dropped},",
                "\"bank_route_folds\":{folds},\"bank_scatter_redirects\":{redirects},"
            ),
            streams = self.streams,
            tap = if self.post_matrix {
                "post_matrix"
            } else {
                "other"
            },
            metrics = metrics,
            window = window_blocks,
            snapshots = self.snapshots,
            dropped = self.dropped(),
            folds = self.bank_route_folds,
            redirects = self.bank_scatter_redirects,
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The hoist measurement: paired alternation between a stationary and a moving arm.
// ---------------------------------------------------------------------------------------------

/// Which arm of the paired comparison an observation belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    /// No automation traffic at all: the standing baseline.
    Quiet,
    /// Every parameter restated at the value it already holds. The hoist settles it.
    Restated,
    /// Every parameter alternating between two nearby values. A window is always open.
    Moving,
}

const ARMS: [Arm; 3] = [Arm::Quiet, Arm::Restated, Arm::Moving];

/// The dB step the moving arm alternates over.
///
/// Small enough to be ordinary console traffic, large enough that the designed `f32` coefficient
/// words genuinely change -- a one-ULP step does not, which is why it cannot be the control.
const MOVING_STEP_DB: f32 = 0.25;

impl Arm {
    const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Moving => "moving",
        }
    }
}

struct HoistMeasurement {
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
}

impl HoistMeasurement {
    /// Runs the three arms **interleaved**, one observation each, round-robin.
    fn run(workload: Workload, backend: Backend) -> Self {
        let lanes = BankWidth::for_backend(backend)
            .expect("a native bank width")
            .lanes() as usize;
        // The arm carries the workload's whole track count, as the banks a session would form:
        // nine tracks is two banks (one full, one holding the ragged remainder), sixty-four is
        // eight full banks. Measuring one bank for both would report the same number twice.
        let banks = (workload.tracks() as usize).div_ceil(lanes);
        let mut arms: Vec<HoistArm> = ARMS
            .iter()
            .map(|arm| HoistArm::new(*arm, backend, lanes, banks))
            .collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock.
        for arm in &mut arms {
            for observation in 0..64 {
                arm.fill_input();
                arm.render(observation);
            }
        }

        let mut samples: [Vec<u64>; 3] = [
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
        ];
        // Paired alternation: arm 0, arm 1, arm 2, arm 0, ... Every drift the run suffers is
        // shared by all three arms, and the deltas are per-observation pairs rather than a
        // difference of two summaries taken minutes apart.
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.fill_input();
                let (elapsed_ns, ()) = timing::timed(|| arm.render(observation));
                arm.absorb_output();
                samples[index].push(elapsed_ns);
            }
        }

        let mut arms = arms.into_iter();
        let digests = [
            arms.next().expect("quiet arm").finish_digest(),
            arms.next().expect("restated arm").finish_digest(),
            arms.next().expect("moving arm").finish_digest(),
        ];
        // The measurement is its own class-A gate: the quiet arm and the restated arm must render
        // byte-identical output, because a restated parameter is by construction a no-op. If the
        // hoist ever changed a rendered bit, this run would say so before any number is reported.
        assert_eq!(
            digests[0],
            digests[1],
            "{}: restating a parameter changed rendered output -- the hoist is not bit-identical",
            workload.kind()
        );
        assert_ne!(
            digests[1],
            digests[2],
            "{}: the moving arm must actually move, or it is not a control arm",
            workload.kind()
        );

        Self {
            ns_per_block: samples,
            digests,
        }
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
    ) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let moving = Percentiles::from_samples(&self.ns_per_block[2]);
        // The paired delta: moving minus restated, per observation, then summarised. This is the
        // ramp arithmetic the hoist elides, measured on pairs taken microseconds apart.
        let paired_delta = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]);

        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_hoist\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"bank_boundary\":\"effect_bank\",",
                "\"observations\":{obs},\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"moving_p50_ns\":{n50},\"moving_p95_ns\":{n95},\"moving_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"moving_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is moving minus restated ",
                "per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            arm0 = ARMS[0].name(),
            arm1 = ARMS[1].name(),
            arm2 = ARMS[2].name(),
            q50 = quiet.p50,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            n50 = moving.p50,
            n95 = moving.p95,
            n99 = moving.p99,
            delta = paired_delta,
            qd = self.digests[0],
            rd = self.digests[1],
            nd = self.digests[2],
            metadata = metadata.record_fields(),
        )
    }
}

/// One arm: a bank of parametric EQs and a bank of compressors, driven with one traffic pattern.
struct HoistArm {
    width: BankWidth,
    banks: Vec<Box<dyn PreparedNativeEffectBank>>,
    left: Vec<f32>,
    right: Vec<f32>,
    targets: Vec<([PreparedEffectTarget; 12], usize)>,
    alternate_targets: Vec<([PreparedEffectTarget; 12], usize)>,
    offsets: Vec<u32>,
    /// One frozen input block, copied in before every observation.
    ///
    /// The bank renders in place, so without this the arm would be filtering its own output a
    /// thousand times over and every arm would decay to the same silence -- which is exactly what
    /// the first version of this subject did, and why its digests compared equal.
    source_left: Vec<f32>,
    source_right: Vec<f32>,
    lanes: usize,
    digest: Sha256Sink,
}

impl HoistArm {
    fn new(arm: Arm, backend: Backend, lanes: usize, banks: usize) -> Self {
        let width = BankWidth::for_backend(backend).expect("a native bank width");
        // The factory comes from the launch registry rather than from a direct dependency on the
        // effect crate: this subject measures what a session would actually instantiate, and the
        // bench crate keeps the dependency boundary it already had.
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let eq = registry
            .get_shared_ascii("miso.parametric-eq")
            .expect("the launch registry carries the parametric EQ");
        let eq_values: Vec<_> = (0..lanes)
            .map(|track| eq_track_values(&*eq, track))
            .collect();
        let eq_requests: Vec<_> = eq_values.iter().map(|values| eq_request(values)).collect();
        let prepared_banks: Vec<Box<dyn PreparedNativeEffectBank>> = (0..banks)
            .map(|_| {
                eq.bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &eq_requests,
                    active_mask: width.full_mask(),
                })
                .expect("valid eq bank request")
                .expect("the native width must bind an eq bank")
                .processor
            })
            .collect();

        // Both target sets are prepared once, before any timing, and selected by block parity
        // inside the render call. The timed region borrows each target prefix and copies its
        // fixed-size words into prepared lanes; it never invokes the EQ designer or allocates.
        let build = |offset_db: f32| {
            let mut targets = Vec::with_capacity(banks * lanes);
            let capability = eq
                .target_preparation()
                .expect("the launch EQ exposes target preparation");
            for _bank in 0..banks {
                for (track, values) in eq_values.iter().enumerate().take(lanes) {
                    let mut candidate = values.clone();
                    let mut changed = vec![false; candidate.len()];
                    if arm != Arm::Quiet {
                        candidate[3 * 2].value = eq_band0_gain(track) + offset_db;
                        changed[3 * 2] = true;
                    }
                    let mut prepared = [PreparedEffectTarget {
                        slot: 0,
                        channel: ParameterChannel::Left,
                        words: [0; 12],
                    }; 12];
                    let count = capability
                        .prepare_targets(
                            EffectTargetRequest {
                                sample_rate: SAMPLE_RATE_HZ,
                                values: &candidate,
                                changed: &changed,
                            },
                            &mut prepared,
                        )
                        .expect("EQ target preparation");
                    targets.push((prepared, count));
                }
            }
            targets
        };
        let targets = build(0.0);
        let alternate_targets = match arm {
            Arm::Moving => build(MOVING_STEP_DB),
            _ => build(0.0),
        };

        Self {
            width,
            banks: prepared_banks,
            left: vec![0.0; QUANTUM * lanes * banks],
            right: vec![0.0; QUANTUM * lanes * banks],
            targets,
            alternate_targets,
            offsets: vec![0; lanes + 1],
            source_left: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * 0.4)
                .collect(),
            source_right: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * -0.4)
                .collect(),
            lanes,
            digest: Sha256Sink::new(),
        }
    }

    /// Restores the frozen input. Outside the clock, like every other evidence step.
    fn fill_input(&mut self) {
        self.left.copy_from_slice(&self.source_left);
        self.right.copy_from_slice(&self.source_right);
    }

    /// Folds this observation's output into the arm's running identity. Outside the clock.
    fn absorb_output(&mut self) {
        for value in self.left.iter().chain(self.right.iter()) {
            self.digest.update(value.to_bits().to_le_bytes());
        }
    }

    fn render(&mut self, observation: u64) {
        let first_sample = observation * QUANTUM as u64;
        let targets = if observation.is_multiple_of(2) {
            &self.targets
        } else {
            &self.alternate_targets
        };
        let stride = QUANTUM * self.lanes;
        for (index, bank) in self.banks.iter_mut().enumerate() {
            let range = index * stride..(index + 1) * stride;
            for lane in 0..self.lanes {
                let (prepared, count) = &targets[index * self.lanes + lane];
                for target in &prepared[..*count] {
                    bank.apply_prepared_target_lane(lane, target)
                        .expect("prepared EQ target application");
                }
            }
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut self.left[range.clone()],
                    &mut self.right[range],
                    None,
                    QUANTUM as u32,
                    self.width,
                    first_sample,
                    &[],
                    &self.offsets,
                    128,
                )
                .expect("console hoist block"),
            );
        }
    }

    fn finish_digest(self) -> String {
        self.digest.finish_hex()
    }
}

/// Band 0's left gain for `track`, the parameter the hoist arms restate or nudge.
fn eq_band0_gain(track: usize) -> f32 {
    -7.5 + (track % 15) as f32
}

/// A prepare request for one lane of the hoist bank, at the launch rate and quantum.
fn eq_request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE_HZ,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 16,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 48,
        },
        tail_bound: launch_native_effect_registry()
            .expect("launch effect registry")
            .tail_bound(
                EffectId::new("miso.parametric-eq").expect("static effect id"),
                SAMPLE_RATE_HZ,
                EffectQuality::Normal,
            )
            .expect("a declared row"),
    }
}

/// Band 1 enabled as a peaking filter, with its left gain at this track's held value.
///
/// Band 1 has to be **enabled explicitly**: the descriptor's default leaves it disabled, and a
/// disabled band ignores its gain entirely -- an earlier version of this subject left the defaults
/// alone and both hoist arms rendered identical audio because the parameter they were moving was
/// not in the signal path at all.
fn eq_track_values(factory: &dyn NativeEffectFactory, track: usize) -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in factory.descriptor().parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            let value = match (index, channel) {
                // Band 1: enabled, peaking, at a per-track frequency.
                (0, _) => 1.0,
                (1, _) => 3.0,
                (2, _) => 400.0 + track as f32 * 37.0,
                (3, ParameterChannel::Left) => eq_band0_gain(track),
                (4, _) => 0.9,
                _ => parameter.default_value,
            };
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    values
}

// ---------------------------------------------------------------------------------------------
// The console-facility measurements (#163 item 0d): meters on/off, observation armed/unarmed.
// ---------------------------------------------------------------------------------------------

/// The workload every facility arm renders. One row, the qualification session.
const FACILITY_WORKLOAD: Workload = Workload::SixtyFourTrackConsole;

/// The meter arms, in emission order.
const METER_CONFIGS: [PlanConfig; 2] = [
    PlanConfig::BASELINE,
    PlanConfig {
        meters: true,
        ..PlanConfig::BASELINE
    },
];

/// The observation arms, in emission order: the two levels of the #143 zero, then armed.
const OBSERVATION_CONFIGS: [PlanConfig; 3] = [
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Absent,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Unarmed,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Armed,
    },
];

/// A set of prepared arms of one workload, alternated observation by observation.
///
/// The #104 paired-alternation protocol, the same one the hoist arms use and for the same reason:
/// running all of arm A and then all of arm B measures the difference between two points in time
/// as much as the difference between two arms. Every arm here is a *complete* sixty-four-track
/// console plan built by `SessionRuntime::build` from one `console_model`, so two arms differ in
/// exactly the one `PlanConfig` field that separates them and in nothing else.
struct FacilityMeasurement {
    /// Timed observations per arm. [`OBSERVATIONS`] in every run the runner launches; the record
    /// states this rather than the constant, so a shortened run cannot pass for the frozen one.
    observations: usize,
    ns_per_block: Vec<Vec<u64>>,
    digests: Vec<String>,
    audit: audit::AuditSnapshot,
    render_errors: u64,
    meter_frames: u64,
    observation_lanes: usize,
    observation_taps: usize,
    published_windows: Vec<u64>,
    /// Per arm, in `configs` order, the plan's `bank_route_folds` (issue #914).
    bank_route_folds: Vec<u64>,
    /// Per arm, in `configs` order, the plan's `bank_scatter_redirects` (issue #914).
    bank_scatter_redirects: Vec<u64>,
}

impl FacilityMeasurement {
    fn run(configs: &[PlanConfig]) -> Self {
        Self::run_for(configs, OBSERVATIONS)
    }

    /// [`FacilityMeasurement::run`] over `observations` timed rounds. Only the unit test below
    /// passes anything but [`OBSERVATIONS`], and the record it produces says so in its
    /// `observations` field, which the validator pins to the frozen count.
    fn run_for(configs: &[PlanConfig], observations: usize) -> Self {
        let mut arms: Vec<SessionRuntime> = configs
            .iter()
            .map(|config| SessionRuntime::build(FACILITY_WORKLOAD, *config))
            .collect();
        let mut hashes: Vec<Sha256Sink> = configs.iter().map(|_| Sha256Sink::new()).collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock
        // -- and so the armed arm's queued subscriptions are drained by a render that is not
        // measured. Sixty-four blocks is sixteen full observation windows.
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
            arm.drain_meters();
        }

        // Issue #914: the plan-shape counters, read once per arm, here -- after the warm-up and
        // before the audit is armed, so outside every clock. Both are fixed at bind, and both
        // optimisations they count render the same bits as the path they replace, so no digest and
        // no timing can say whether either fired; only these can. They are recorded rather than
        // asserted in-run: an arm that stopped folding is a finding the record must carry intact
        // and the validator must refuse, not a reason to lose the record.
        let bank_route_folds: Vec<u64> =
            arms.iter().map(SessionRuntime::bank_route_folds).collect();
        let bank_scatter_redirects: Vec<u64> = arms
            .iter()
            .map(SessionRuntime::bank_scatter_redirects)
            .collect();

        let mut samples: Vec<Vec<u64>> = configs
            .iter()
            .map(|_| Vec::with_capacity(observations))
            .collect();
        let mut render_errors = 0_u64;
        let mut meter_frames = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..observations as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            // Every evidence step is outside the clock and after the whole round-robin, so one
            // arm's bookkeeping never lands between another arm's two timed blocks.
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
                meter_frames += arm.drain_meters();
            }
        }
        let snapshot = audit::snapshot();

        let observation_lanes = arms
            .iter()
            .map(SessionRuntime::observation_lanes)
            .max()
            .unwrap_or(0);
        let observation_taps = arms
            .iter()
            .map(SessionRuntime::observation_taps)
            .max()
            .unwrap_or(0);
        let published_windows: Vec<u64> =
            arms.iter().map(SessionRuntime::published_windows).collect();
        let digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();

        // The class-A statement, asserted in-run rather than only claimed in a record: attaching a
        // meter stream, attaching observation capacity and arming a tap are all *observation*. None
        // of them may change a rendered bit. If one ever did, this run says so before it reports a
        // number.
        for (index, digest) in digests.iter().enumerate() {
            assert_eq!(
                digest, &digests[0],
                "arm {index} rendered different output: an observation facility changed the signal"
            );
        }
        Self {
            observations,
            ns_per_block: samples,
            digests,
            audit: snapshot,
            render_errors,
            meter_frames,
            observation_lanes,
            observation_taps,
            published_windows,
            bank_route_folds,
            bank_scatter_redirects,
        }
    }

    fn meters_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let off = Percentiles::from_samples(&self.ns_per_block[0]);
        let on = Percentiles::from_samples(&self.ns_per_block[1]);
        // The meter arm must actually have metered. A silently empty meter set would report a
        // delta of nothing and look like a wonderful result.
        assert!(
            self.meter_frames > 0,
            "the meters-on arm published no meter frame: it is not measuring meters"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_meters\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",\"arms\":[\"meters_off\",\"meters_on\"],",
                "\"meter_streams\":{streams},\"meter_tap\":\"post_matrix\",",
                "\"meter_window_blocks\":{window},\"meter_frames_drained\":{frames},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"meters_off_p50_ns\":{o50},\"meters_off_p95_ns\":{o95},\"meters_off_p99_ns\":{o99},",
                "\"meters_on_p50_ns\":{n50},\"meters_on_p95_ns\":{n95},\"meters_on_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"meters_off_output_sha256\":\"{od}\",\"meters_on_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"meters_off == meters_on, asserted in-run\",",
                "\"meters_off_bank_route_folds\":{off_folds},\"meters_on_bank_route_folds\":{on_folds},",
                "\"meters_off_bank_scatter_redirects\":{off_redirects},",
                "\"meters_on_bank_scatter_redirects\":{on_redirects},",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"two arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is meters_on minus ",
                "meters_off per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = self.observations,
            streams = FACILITY_WORKLOAD.tracks(),
            window = WINDOW_BLOCKS,
            frames = self.meter_frames,
            o50 = off.p50,
            o95 = off.p95,
            o99 = off.p99,
            n50 = on.p50,
            n95 = on.p95,
            n99 = on.p99,
            delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            od = self.digests[0],
            nd = self.digests[1],
            off_folds = self.bank_route_folds[0],
            on_folds = self.bank_route_folds[1],
            off_redirects = self.bank_scatter_redirects[0],
            on_redirects = self.bank_scatter_redirects[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }

    fn observation_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let absent = Percentiles::from_samples(&self.ns_per_block[0]);
        let unarmed = Percentiles::from_samples(&self.ns_per_block[1]);
        let armed = Percentiles::from_samples(&self.ns_per_block[2]);
        // The two halves of the honesty gate. An unarmed lane that published anything would not be
        // unarmed, and an armed lane that published nothing would be measuring the unarmed cost
        // twice and reporting the difference as noise.
        assert_eq!(
            self.published_windows[1], 0,
            "the unarmed arm published an observation window"
        );
        assert!(
            self.published_windows[2] > 0,
            "the armed arm published no observation window: it is not actually armed"
        );
        assert!(
            self.observation_lanes > 0 && self.observation_taps > 0,
            "no observation lane was prepared: the arms are all the level-1 zero"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_observation\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"observation_lanes\":{lanes},\"observation_taps\":{taps},",
                "\"observation_window_blocks\":{window},",
                "\"unarmed_windows_published\":{uwin},\"armed_windows_published\":{awin},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"absent_p50_ns\":{a50},\"absent_p95_ns\":{a95},\"absent_p99_ns\":{a99},",
                "\"unarmed_p50_ns\":{u50},\"unarmed_p95_ns\":{u95},\"unarmed_p99_ns\":{u99},",
                "\"armed_p50_ns\":{r50},\"armed_p95_ns\":{r95},\"armed_p99_ns\":{r99},",
                "\"paired_capacity_delta_median_ns\":{capacity},",
                "\"paired_arm_delta_median_ns\":{arming},",
                "\"absent_output_sha256\":\"{ad}\",\"unarmed_output_sha256\":\"{ud}\",",
                "\"armed_output_sha256\":\"{rd}\",",
                "\"bit_identity\":\"absent == unarmed == armed, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; capacity delta is unarmed minus absent ",
                "and arm delta is armed minus unarmed, per observation; descriptive only; no ",
                "threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = self.observations,
            arm0 = ObservationArm::Absent.name(),
            arm1 = ObservationArm::Unarmed.name(),
            arm2 = ObservationArm::Armed.name(),
            lanes = self.observation_lanes,
            taps = self.observation_taps,
            window = WINDOW_BLOCKS,
            uwin = self.published_windows[1],
            awin = self.published_windows[2],
            a50 = absent.p50,
            a95 = absent.p95,
            a99 = absent.p99,
            u50 = unarmed.p50,
            u95 = unarmed.p95,
            u99 = unarmed.p99,
            r50 = armed.p50,
            r95 = armed.p95,
            r99 = armed.p99,
            capacity = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            arming = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]),
            ad = self.digests[0],
            ud = self.digests[1],
            rd = self.digests[2],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The placement measurement: the #175 chain-shape row-pair, alternated observation by observation.
// ---------------------------------------------------------------------------------------------

/// The two arms of the chain-shape comparison, in record order.
///
/// `split_chains` is the retired layout: EQ on `simd1` and the compressor in the `dynamic` rack,
/// two one-slot bank chains per cohort. `merged_chain` is the standing fixture with its limiter
/// removed: the same two effects, the same coefficients, the same order, as one two-slot chain on
/// `simd1`.
///
/// They carry *identical arithmetic*. Every difference between their timings is chain shape --
/// how many planar/AoSoA round-trips the plan pays and how the slots are grouped -- and nothing
/// else. That is what makes this a paired measurement rather than two rows a reader subtracts.
const PLACEMENT_ARMS: [Workload; 2] = [
    Workload::SixtyFourTrackConsoleLegacy,
    Workload::SixtyFourTrackEqCompSimd1,
];

/// The #175 chain-shape row-pair.
///
/// Structurally a sibling of [`FacilityMeasurement`], and deliberately so: the two arms are
/// alternated observation by observation, for the reason the module header gives. This pair
/// especially needs it. The hypothesis under test predicted a *saving*, and the measured answer is
/// that there is none -- and "no difference" is precisely the claim a run-A-then-run-B benchmark
/// cannot make honestly, because the drift between two points in time is indistinguishable from
/// the effect at that scale. Alternating makes the delta a paired statistic over simultaneous
/// conditions, so a reported zero is a measured zero.
struct PlacementMeasurement {
    ns_per_block: [Vec<u64>; 2],
    digests: [String; 2],
    /// Transposes attributable to the timed region, per arm, per block.
    transposes_per_block: [u64; 2],
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl PlacementMeasurement {
    fn run() -> Self {
        let mut arms: Vec<SessionRuntime> = PLACEMENT_ARMS
            .iter()
            .map(|workload| SessionRuntime::new(*workload))
            .collect();
        let mut hashes = PLACEMENT_ARMS.map(|_| Sha256Sink::new());
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
        }
        // Counted across the timed region only, so the warmup's round-trips are not attributed to
        // it. Read before and after, outside the clock.
        let before: Vec<u64> = arms.iter().map(SessionRuntime::bank_transposes).collect();

        let mut samples = PLACEMENT_ARMS.map(|_| Vec::with_capacity(OBSERVATIONS));
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests = hashes.map(Sha256Sink::finish_hex);

        // The class-A statement, asserted in-run. Rack placement regroups lanes; it never changes
        // per-lane arithmetic (AGENTS.md, #166). If the two arms ever disagree, this run says so
        // before it reports a delta -- because a timing difference between two arms that compute
        // different things is not a chain-shape measurement at all.
        assert_eq!(
            digests[0], digests[1],
            "the two placements rendered different output: a placement change moved a rendered bit"
        );

        let transposes: Vec<u64> = arms
            .iter()
            .zip(before.iter())
            .map(|(arm, start)| (arm.bank_transposes() - start) / OBSERVATIONS as u64)
            .collect();

        Self {
            ns_per_block: samples,
            digests,
            transposes_per_block: [transposes[0], transposes[1]],
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let split = Percentiles::from_samples(&self.ns_per_block[0]);
        let merged = Percentiles::from_samples(&self.ns_per_block[1]);
        let delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]);
        let tracks = f64::from(PLACEMENT_ARMS[0].tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_placement\",",
                "\"workload_kind\":\"sixty_four_track_placement\",\"tracks\":{tracks},",
                "\"round\":{round},\"backend\":\"{backend}\",",
                "\"observations\":{obs},\"units\":\"ns_per_block\",",
                "\"percentile_method\":\"nearest_rank\",",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"split_chains\",\"merged_chain\"],",
                "\"split_chains_layout\":\"{split_layout}\",",
                "\"merged_chain_layout\":\"{merged_layout}\",",
                "\"split_chains_p50_ns\":{split_p50},\"split_chains_p95_ns\":{split_p95},",
                "\"split_chains_p99_ns\":{split_p99},",
                "\"merged_chain_p50_ns\":{merged_p50},\"merged_chain_p95_ns\":{merged_p95},",
                "\"merged_chain_p99_ns\":{merged_p99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"paired_delta_median_ns_per_track\":{delta_per_track},",
                "\"split_chains_transposes_per_block\":{split_transposes},",
                "\"merged_chain_transposes_per_block\":{merged_transposes},",
                "\"split_chains_output_sha256\":\"{split_digest}\",",
                "\"merged_chain_output_sha256\":\"{merged_digest}\",",
                "\"bit_identity\":\"split_chains == merged_chain, asserted in-run\",",
                "\"render_errors\":{errors},",
                "\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            tracks = PLACEMENT_ARMS[0].tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            split_layout = PLACEMENT_ARMS[0].strip_layout(),
            merged_layout = PLACEMENT_ARMS[1].strip_layout(),
            split_p50 = split.p50,
            split_p95 = split.p95,
            split_p99 = split.p99,
            merged_p50 = merged.p50,
            merged_p95 = merged.p95,
            merged_p99 = merged.p99,
            delta = delta,
            delta_per_track = format_f64(delta as f64 / tracks),
            split_transposes = self.transposes_per_block[0],
            merged_transposes = self.transposes_per_block[1],
            split_digest = self.digests[0],
            merged_digest = self.digests[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = PLACEMENT_STATISTICAL_METHOD,
        )
    }
}

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const PLACEMENT_STATISTICAL_METHOD: &str = "two arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; paired delta is merged_chain minus split_chains per \
observation; descriptive only; no threshold";

// ---------------------------------------------------------------------------------------------
// The mono row-pair: the gate the mono collapse will be measured and constrained by.
// ---------------------------------------------------------------------------------------------

/// The two arms of the mono row-pair, in record order.
///
/// `collapse_eligible` is the mono fixture rendered as written; `collapse_forced_off` is the same
/// session with the collapse suppressed. **Today they are one session.** No code in this tree
/// reads the channel-symmetry witness to decide anything rendered, so the two arms compile,
/// prepare and render identically, the paired delta is noise about zero, and the record says so in
/// [`MONO_ARM_DIFFERENCE`] rather than leaving a reader to infer it.
///
/// The pair is built now, before the thing it measures, deliberately. When the collapse lands,
/// this arm takes it and that arm forces it off, and the digest equality asserted below stops
/// being trivial: it becomes the standing class-A gate on the entire mechanism -- a collapsed
/// track must render, to the bit, what the same track renders uncollapsed. A gate written by the
/// same change it is supposed to check is not a gate, which is the whole reason for the ordering.
const MONO_ARMS: [Workload; 2] = [
    Workload::SixtyFourTrackConsoleMono,
    Workload::SixtyFourTrackConsoleMonoDual,
];

/// Pinned verbatim in the validator: what separates the two arms.
///
/// The sentence is the honesty, and it moved when the thing it describes arrived. It used to read
/// "no collapse exists in this tree", because a reader who found a two-arm paired record with a
/// near-zero delta and no such field would reasonably have read it as "the collapse saves
/// nothing". It now names the mechanism, and the delta beside it is the mechanism's cost.
///
/// Both arms compile the *same* fixture. The difference is one bind-time switch
/// (`PreparedRenderPlan::force_mono_collapse_off`) and nothing else -- not a second session, not a
/// second parameter set -- which is what makes the paired delta the collapse and the digest
/// equality beside it a class-A statement about it. Changing this claim means editing this
/// constant, the validator that pins it and the mutation case that guards it, which is exactly the
/// visibility the change deserves.
const MONO_ARM_DIFFERENCE: &str = "collapse_eligible takes the mono collapse on every cohort; collapse_forced_off renders the same fixture dual";

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const MONO_STATISTICAL_METHOD: &str = "two arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; paired delta is collapse_forced_off minus \
collapse_eligible per observation; descriptive only; no threshold";

/// The mono-collapse row-pair, structurally a sibling of [`PlacementMeasurement`].
///
/// Alternated observation by observation for the same reason that one is: the claim under test is
/// (today) that there is *no* difference, and "no difference" is precisely what a run-A-then-run-B
/// benchmark cannot say honestly, because drift between two points in time is indistinguishable
/// from an effect at that scale.
struct MonoMeasurement {
    ns_per_block: [Vec<u64>; 2],
    digests: [String; 2],
    /// Transposes attributable to the timed region, per arm, per block.
    transposes_per_block: [u64; 2],
    /// Tracks whose structural (`SOURCE`) witness holds, per arm.
    mono_source_tracks: [u64; 2],
    /// `[collapse-eligible lanes, lanes]` over the track lanes of the built runtime's bank chains,
    /// per arm: the prepared half of the premise, counted over exactly the lanes it is about
    /// (`SessionRuntime::bank_symmetry_counters`, issue #911).
    bank_symmetry: [[u64; 2]; 2],
    /// Lanes the built runtime's whole channel-symmetry census counts, per arm.
    census_lanes: [u64; 2],
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl MonoMeasurement {
    fn run() -> Self {
        let mut arms: Vec<SessionRuntime> =
            MONO_ARMS.iter().map(|w| SessionRuntime::new(*w)).collect();
        let mut hashes = MONO_ARMS.map(|_| Sha256Sink::new());
        // The evidence that the fixture is what the row claims, read before anything is timed. A
        // pair measured on a session with no mono-source track would be a pair measuring the
        // standing console twice under another name, and the record carries both numbers so a
        // reader can check that rather than trust the fixture's file name.
        let mono_source_tracks: Vec<u64> = arms
            .iter()
            .map(SessionRuntime::structural_mono_tracks)
            .collect();
        // The prepared half of that evidence, counted over the lanes the premise is about: the
        // track lanes of the bank chains, the only units the collapse acts on. The whole census
        // (`symmetry_counters`) also counts every single op -- on this fixture the 64 track
        // `Input` ops (source-set claims since issue #965, the engine's symmetric source-input
        // kind) and the `main-out` identity (symmetric, naming no track, nothing upstream of the
        // seam). The premise used to read the census's
        // eligible half, which equalled this count only while that output declined too; issue
        // #221 rebound it as the engine's identity and the premise failed at `65 != 64` (issue
        // #911). `bank_symmetry_counters` names what it excludes, and `chain_shape.rs` pins every
        // row of both arms, so the next change to the unit inventory fails at `cargo test`.
        let bank_symmetry: Vec<[u64; 2]> = arms
            .iter()
            .map(SessionRuntime::bank_symmetry_counters)
            .collect();
        let census: Vec<[u64; 2]> = arms.iter().map(SessionRuntime::symmetry_counters).collect();
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
        }
        let before: Vec<u64> = arms.iter().map(SessionRuntime::bank_transposes).collect();

        let mut samples = MONO_ARMS.map(|_| Vec::with_capacity(OBSERVATIONS));
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests = hashes.map(Sha256Sink::finish_hex);

        // The class-A statement, asserted in-run. Trivially true today and load-bearing the day
        // the collapse exists: a track whose two channels are doing identical work must render,
        // to the bit, what the same track renders with both channels computed.
        assert_eq!(
            digests[0], digests[1],
            "the two mono arms rendered different output: a collapse-eligible session and the \
             same session with the collapse forced off must agree to the bit"
        );
        // And the premise the equality is a statement *about*. Without this, a fixture that had
        // silently lost its mono source mapping would still pass the assertion above -- by being
        // two copies of an ordinary stereo session.
        for (index, tracks) in mono_source_tracks.iter().enumerate() {
            assert_eq!(
                *tracks,
                u64::from(MONO_ARMS[index].tracks()),
                "{}: every track of the mono fixture must carry a mono source mapping",
                MONO_ARMS[index].kind()
            );
            // Both halves of the pair: one bank lane per track, and every one of them eligible. A
            // track that stopped banking would lower the lane count, not only the eligible one.
            let tracks = u64::from(MONO_ARMS[index].tracks());
            assert_eq!(
                bank_symmetry[index],
                [tracks, tracks],
                "{}: every track of the mono fixture must carry a symmetric prepared witness on \
                 its bank-chain lane ([eligible, lanes] over the chains' track lanes)",
                MONO_ARMS[index].kind()
            );
        }

        let transposes: Vec<u64> = arms
            .iter()
            .zip(before.iter())
            .map(|(arm, start)| (arm.bank_transposes() - start) / OBSERVATIONS as u64)
            .collect();

        Self {
            ns_per_block: samples,
            digests,
            transposes_per_block: [transposes[0], transposes[1]],
            mono_source_tracks: [mono_source_tracks[0], mono_source_tracks[1]],
            bank_symmetry: [bank_symmetry[0], bank_symmetry[1]],
            census_lanes: [census[0][1], census[1][1]],
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let eligible = Percentiles::from_samples(&self.ns_per_block[0]);
        let forced = Percentiles::from_samples(&self.ns_per_block[1]);
        let delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]);
        let tracks = f64::from(MONO_ARMS[0].tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_mono\",",
                "\"workload_kind\":\"sixty_four_track_mono_pair\",\"tracks\":{tracks},",
                "\"round\":{round},\"backend\":\"{backend}\",",
                "\"observations\":{obs},\"units\":\"ns_per_block\",",
                "\"percentile_method\":\"nearest_rank\",",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"collapse_eligible\",\"collapse_forced_off\"],",
                "\"fixture_id\":\"{fixture}\",",
                "\"collapse_eligible_p50_ns\":{eligible_p50},",
                "\"collapse_eligible_p95_ns\":{eligible_p95},",
                "\"collapse_eligible_p99_ns\":{eligible_p99},",
                "\"collapse_forced_off_p50_ns\":{forced_p50},",
                "\"collapse_forced_off_p95_ns\":{forced_p95},",
                "\"collapse_forced_off_p99_ns\":{forced_p99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"paired_delta_median_ns_per_track\":{delta_per_track},",
                "\"collapse_eligible_transposes_per_block\":{eligible_transposes},",
                "\"collapse_forced_off_transposes_per_block\":{forced_transposes},",
                "\"mono_source_tracks\":{mono_tracks},",
                "\"symmetric_lanes\":{symmetric_lanes},\"lanes\":{lanes},",
                "\"collapse_eligible_output_sha256\":\"{eligible_digest}\",",
                "\"collapse_forced_off_output_sha256\":\"{forced_digest}\",",
                "\"bit_identity\":\"collapse_eligible == collapse_forced_off, asserted in-run\",",
                "\"arm_difference\":\"{difference}\",",
                "\"render_errors\":{errors},",
                "\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            tracks = MONO_ARMS[0].tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            fixture = json_escape(MONO_ARMS[0].fixture_id()),
            eligible_p50 = eligible.p50,
            eligible_p95 = eligible.p95,
            eligible_p99 = eligible.p99,
            forced_p50 = forced.p50,
            forced_p95 = forced.p95,
            forced_p99 = forced.p99,
            delta = delta,
            delta_per_track = format_f64(delta as f64 / tracks),
            eligible_transposes = self.transposes_per_block[0],
            forced_transposes = self.transposes_per_block[1],
            // One number, not two: the run asserts the arms agree before it emits, so a pair of
            // columns here could only ever restate the assertion.
            mono_tracks = self.mono_source_tracks[0],
            // The premise's count beside the whole census it is a subset of. On every record
            // sealed before issue #221 the census's eligible half *was* the bank-lane count,
            // because every other unit of this fixture declined, so the column keeps its meaning;
            // `lanes > symmetric_lanes` in the validator still says the census holds lanes that
            // are not eligible track lanes.
            symmetric_lanes = self.bank_symmetry[0][0],
            lanes = self.census_lanes[0],
            eligible_digest = self.digests[0],
            forced_digest = self.digests[1],
            difference = MONO_ARM_DIFFERENCE,
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = MONO_STATISTICAL_METHOD,
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The compressor-automation measurement: the ramping body no standing row can see.
// ---------------------------------------------------------------------------------------------

/// The workload every automation arm renders: the compressor decomposition row.
///
/// Deliberately the decomposition row and not the whole strip. The subject here is the
/// compressor's *ramping body*, and a row that also carries an EQ and a limiter buries a
/// microsecond of it under two other effects' steady-state cost.
const AUTOMATION_WORKLOAD: Workload = Workload::SixtyFourTrackCompressorOnly;

/// The plan every automation arm is prepared with: the live-control channel and nothing
/// else.
///
/// All three arms carry the channel, so the deltas between them are the *traffic* and never the
/// queue's presence -- the same discipline `console_observation` uses for its three arms.
const AUTOMATION_CONFIG: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};

/// The effect the arms address, by contract id rather than by session slot name.
const AUTOMATION_EFFECT_ID: &str = "miso.compressor";

/// Descriptor index of the compressor's `threshold`. An index into the parameter table, never the
/// wire `parameter_id`.
const AUTOMATION_PARAMETER_INDEX: u32 = 0;

/// The parameter's descriptor name, recorded so a reader does not have to resolve the index.
const AUTOMATION_PARAMETER_NAME: &str = "threshold";

/// The threshold, in dB, every arm is settled at before the clock starts.
///
/// Inside the descriptor's `[-80, 0]` domain and far enough below the source tone (0.6 peak, less
/// a -6 dB input trim) that the static curve is genuinely engaged. A threshold the signal never
/// reaches would make the arms differ in a coefficient nothing reads, and the in-run inequality
/// gate below would catch it -- but as a failure, after a run.
const AUTOMATION_BASE_DB: f32 = -24.0;

/// The dB step the automated arm alternates over, either side of the base.
///
/// It alternates `base + step`, `base - step`, `base + step`, ... rather than restating the base
/// on every other block, so that **every** timed block opens a real smoothing window. Alternating
/// against the base itself would let the #144 stationary hoist settle half the blocks and the row
/// would report half the ramping cost it claims to.
const AUTOMATION_STEP_DB: f32 = 0.5;

/// Which traffic pattern an automation arm delivers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AutomationArm {
    /// The channel is attached and nothing is ever pushed. The standing baseline.
    Quiet,
    /// The threshold restated at the value it already holds, every block. The #144 hoist settles
    /// it, so no window ever opens and this arm must render `quiet`'s bits exactly.
    Restated,
    /// The threshold moved every block, so a `Linear 64` window is open on every block of the run.
    Automated,
}

const AUTOMATION_ARMS: [AutomationArm; 3] = [
    AutomationArm::Quiet,
    AutomationArm::Restated,
    AutomationArm::Automated,
];

impl AutomationArm {
    const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Automated => "automated",
        }
    }

    /// The value this arm pushes before block `observation`, or `None` if it pushes nothing.
    fn value(self, observation: u64) -> Option<f32> {
        match self {
            Self::Quiet => None,
            Self::Restated => Some(AUTOMATION_BASE_DB),
            Self::Automated => Some(if observation.is_multiple_of(2) {
                AUTOMATION_BASE_DB + AUTOMATION_STEP_DB
            } else {
                AUTOMATION_BASE_DB - AUTOMATION_STEP_DB
            }),
        }
    }
}

/// The automation-active row: one Point span per block, on one track, through a real plan.
///
/// # Why this measurement had to be added
///
/// Every existing row of this stream renders with automation cleared -- `console_model` clears the
/// fixture's table unconditionally and both fixture gates assert the standing sessions declare
/// none -- so the compressor's ramping body is dead code in all of them. The one arm that does
/// deliver automation, `console_hoist`, drives banks of *parametric EQs*; no compressor in this
/// benchmark has ever seen a span. A change to the compressor's ramping path is therefore
/// invisible to the whole standing table, which is exactly the gap this row closes.
///
/// # What one Point span per block on one track actually exercises
///
/// Sixty-four tracks is eight eight-lane banks. Automating one track's left channel puts a
/// `Linear 64` window in flight on one lane of one bank, and the compressor cuts its block into a
/// ramping prefix and an idle remainder from the longest ramp anywhere in *either* channel of that
/// bank. So one automated lane drags fifteen unautomated channel-lanes of that bank through the
/// ramping body for sixty-four of the block's hundred and twenty-eight frames, while the other
/// seven banks stay entirely on the idle body. That asymmetry is the realistic console case -- a
/// console rides one parameter on one track, not sixty-four at once -- and it is the case whose
/// cost no other row in this file reports.
struct AutomationMeasurement {
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
    /// Pushes the bounded control queue accepted, per arm. A refused push is a measured lie.
    accepted: [u64; 3],
    track_id: String,
    effect_id: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl AutomationMeasurement {
    fn run() -> Self {
        let mut arms: Vec<SessionRuntime> = AUTOMATION_ARMS
            .iter()
            .map(|_| SessionRuntime::build(AUTOMATION_WORKLOAD, AUTOMATION_CONFIG))
            .collect();
        let channel = arms[0]
            .first_track_control_channel(AUTOMATION_EFFECT_ID)
            .expect("the compressor decomposition row prepares a compressor control channel");
        let (track_id, effect_id) = {
            let (track, effect) = arms[0].control_identity(channel);
            (track.to_owned(), effect.to_owned())
        };

        // Untimed pre-roll. Every arm -- `quiet` included -- is settled at the same base threshold
        // before the clock starts, which is what makes `quiet` and `restated` comparable at all:
        // `restated` can only be bit-identical to a baseline that holds the value it restates.
        // Sixty-four blocks is far more than the sixty-four *samples* the window takes.
        for arm in &mut arms {
            assert!(
                arm.push_parameter(
                    channel,
                    AUTOMATION_PARAMETER_INDEX,
                    ParameterChannel::Left,
                    AUTOMATION_BASE_DB,
                ),
                "the bounded control queue refused the pre-roll push"
            );
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
        }

        let mut hashes = AUTOMATION_ARMS.map(|_| Sha256Sink::new());
        let mut samples = AUTOMATION_ARMS.map(|_| Vec::with_capacity(OBSERVATIONS));
        let mut accepted = [0_u64; 3];
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                // The push is control-plane work and stays outside the clock, exactly where a host
                // does it. What the timed region pays for is the render-side drain, the span it
                // stages and the ramping body that span opens -- which is the cost under test.
                if let Some(value) = AUTOMATION_ARMS[index].value(observation)
                    && arm.push_parameter(
                        channel,
                        AUTOMATION_PARAMETER_INDEX,
                        ParameterChannel::Left,
                        value,
                    )
                {
                    accepted[index] += 1;
                }
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests = hashes.map(Sha256Sink::finish_hex);

        // The class-A statement, asserted in-run. Restating a parameter at the value it already
        // holds is by construction a no-op, and the #144 hoist is what makes it free; if it ever
        // moved a rendered bit, this run says so before it reports a number. This is the first
        // time that statement is made about the *compressor*, through a real prepared plan.
        assert_eq!(
            digests[0], digests[1],
            "restating the compressor's threshold changed rendered output: the hoist is not \
             bit-identical"
        );
        // And the honesty half: an arm that renders the restated arm's bits is not automating.
        assert_ne!(
            digests[1], digests[2],
            "the automated arm rendered the restated arm's output: no window ever opened"
        );
        // A silently refused push would report the cost of automation that never arrived.
        assert_eq!(accepted[0], 0, "the quiet arm pushed a record");
        for index in [1, 2] {
            assert_eq!(
                accepted[index],
                OBSERVATIONS as u64,
                "the {} arm's bounded control queue refused a push",
                AUTOMATION_ARMS[index].name()
            );
        }

        Self {
            ns_per_block: samples,
            digests,
            accepted,
            track_id,
            effect_id,
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let automated = Percentiles::from_samples(&self.ns_per_block[2]);
        // The row's headline: the ramping surcharge, taken against an arm that delivers the *same*
        // control traffic and differs only in whether that traffic opens a window.
        let ramp = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]);
        let control = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]);
        let tracks = f64::from(AUTOMATION_WORKLOAD.tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_automation\",",
                "\"workload_kind\":\"sixty_four_track_compressor_automation\",\"tracks\":{tracks},",
                "\"synthetic_fixture\":{synthetic},\"strip_content\":\"{strip}\",",
                "\"strip_layout\":\"{layout}\",\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"automated_track_id\":\"{track}\",\"automated_effect_id\":\"{effect}\",",
                "\"automated_effect\":\"{contract}\",\"automated_parameter\":\"{parameter}\",",
                "\"automated_parameter_index\":{parameter_index},",
                "\"automated_channel\":\"left\",\"automation_spans_per_block\":1,",
                "\"smoothing_samples\":{smoothing},",
                "\"restated_pushes_accepted\":{restated_pushes},",
                "\"automated_pushes_accepted\":{automated_pushes},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p95_ns\":{q95},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"automated_p50_ns\":{a50},\"automated_p95_ns\":{a95},\"automated_p99_ns\":{a99},",
                "\"paired_ramp_delta_median_ns\":{ramp},",
                "\"paired_ramp_delta_median_ns_per_track\":{ramp_per_track},",
                "\"paired_control_delta_median_ns\":{control},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"automated_output_sha256\":\"{ad}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            tracks = AUTOMATION_WORKLOAD.tracks(),
            synthetic = AUTOMATION_WORKLOAD.synthetic(),
            strip = AUTOMATION_WORKLOAD.strip_content(),
            layout = AUTOMATION_WORKLOAD.strip_layout(),
            signal = AUTOMATION_WORKLOAD.input_signal(),
            fixture = json_escape(AUTOMATION_WORKLOAD.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = OBSERVATIONS,
            arm0 = AUTOMATION_ARMS[0].name(),
            arm1 = AUTOMATION_ARMS[1].name(),
            arm2 = AUTOMATION_ARMS[2].name(),
            track = json_escape(&self.track_id),
            effect = json_escape(&self.effect_id),
            contract = AUTOMATION_EFFECT_ID,
            parameter = AUTOMATION_PARAMETER_NAME,
            parameter_index = AUTOMATION_PARAMETER_INDEX,
            smoothing = AUTOMATION_SMOOTHING_SAMPLES,
            restated_pushes = self.accepted[1],
            automated_pushes = self.accepted[2],
            q50 = quiet.p50,
            q95 = quiet.p95,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            a50 = automated.p50,
            a95 = automated.p95,
            a99 = automated.p99,
            ramp = ramp,
            ramp_per_track = format_f64(ramp as f64 / tracks),
            control = control,
            qd = self.digests[0],
            rd = self.digests[1],
            ad = self.digests[2],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = AUTOMATION_STATISTICAL_METHOD,
        )
    }
}

/// The compressor descriptor's `smoothing_samples` for every automatable parameter.
///
/// Recorded so the row states the window length its surcharge is the cost of, rather than leaving
/// a reader to look it up in the effect crate.
const AUTOMATION_SMOOTHING_SAMPLES: u32 = 64;

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const AUTOMATION_STATISTICAL_METHOD: &str = "three arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; ramp delta is automated minus restated and control delta \
is restated minus quiet, per observation; descriptive only; no threshold";

// ---------------------------------------------------------------------------------------------
// The mixing-automation measurement (issue #1003): the mono console riding eight controls.
// ---------------------------------------------------------------------------------------------

/// The record kind of the mixing-automation row.
const MIXING_WORKLOAD_KIND: &str = "sixty_four_track_console_mono_mixing_automation";

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const MIXING_STATISTICAL_METHOD: &str = "three arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; ramp delta is automated minus restated and collapse delta \
is restated minus quiet, per observation; descriptive only; no threshold";

/// The mixing-automation row: the mono console, eight tracks each riding one control, pushed in
/// the shapes a real host pushes them.
///
/// Structurally a sibling of [`AutomationMeasurement`]: three arms that carry the same control
/// channel, alternated observation by observation, with the push outside the clock and the render
/// call alone inside it. What it adds is the collapse. On the mono console a restating record can
/// retire a cohort's collapse for the life of the plan, so `restated` is not only the ramping
/// arm's control but a measurement of its own, and each arm states its `bank_collapse_counters`
/// read once after the run.
struct MixingAutomationMeasurement {
    /// Timed observations. [`OBSERVATIONS`] in every run the runner launches; the record states
    /// this rather than the constant, so a shortened run cannot pass for the frozen one.
    observations: usize,
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
    /// Pushes after the settling write, per arm.
    tallies: [PushTally; 3],
    /// `bank_collapse_counters` per arm, read once after the run.
    collapse: [[u64; 2]; 3],
    /// The eight controls as the `quiet` arm resolved them.
    controls: Vec<ResolvedControl>,
    /// The untimed preflight the run asserted before it took a number.
    preflight: Preflight,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl MixingAutomationMeasurement {
    fn run(backend: Backend) -> Self {
        Self::run_for(backend, OBSERVATIONS)
    }

    /// [`MixingAutomationMeasurement::run`] over `observations` timed rounds. Only the unit test
    /// below passes anything but [`OBSERVATIONS`], and the record says so.
    fn run_for(backend: Backend, observations: usize) -> Self {
        // The premises first, untimed and before any arm the clock sees is built: every automated
        // effect moves bits on its own, restating is bit-exact, and the EQ's restatement keeps the
        // collapse. A row whose premises fail reports nothing.
        let preflight = mixing_automation::preflight(backend);
        preflight.assert_premises();

        let mut arms: Vec<MixingArm> = mixing_automation::ARMS
            .iter()
            .map(|arm| MixingArm::prepare(*arm, None, backend))
            .collect();
        let mut hashes = mixing_automation::ARMS.map(|_| Sha256Sink::new());
        let mut samples = mixing_automation::ARMS.map(|_| Vec::with_capacity(observations));
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for _ in 0..observations {
            for (index, arm) in arms.iter_mut().enumerate() {
                // Control-plane work, outside the clock, exactly where a host does it: the EQ's
                // owner designs its target here, and the queue drain, the staging and the window
                // the traffic opens are what the timed region pays for.
                arm.drive();
                let (elapsed_ns, result) = timing::timed(|| arm.render());
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter().enumerate() {
                arm.runtime.hash_output(&mut hashes[index]);
            }
        }
        let snapshot = audit::snapshot();
        let digests = hashes.map(Sha256Sink::finish_hex);

        // The class-A statement, asserted in-run: every base is the held value, so restating it
        // is a no-op, and a collapse that a restating record retires renders the bits it rendered
        // collapsed.
        assert_eq!(
            digests[0], digests[1],
            "console_mixing_automation: restating the held values changed rendered output"
        );
        // The honesty half: an automated arm that renders the restated arm's bits opened no
        // window.
        assert_ne!(
            digests[1], digests[2],
            "console_mixing_automation: the automated arm rendered the restated arm's output"
        );
        let tallies = [arms[0].tally, arms[1].tally, arms[2].tally];
        assert_eq!(tallies[0].attempted, 0, "the quiet arm pushed a record");
        let per_block = arms[0].automation.pushes_per_block(None);
        for index in [1, 2] {
            assert_eq!(
                (tallies[index].attempted, tallies[index].accepted),
                (
                    per_block * observations as u64,
                    per_block * observations as u64
                ),
                "console_mixing_automation {}: a push was refused",
                mixing_automation::ARMS[index].name()
            );
        }
        let collapse = [
            arms[0].runtime.bank_collapse_counters(),
            arms[1].runtime.bank_collapse_counters(),
            arms[2].runtime.bank_collapse_counters(),
        ];
        Self {
            observations,
            ns_per_block: samples,
            digests,
            tallies,
            collapse,
            controls: arms[0].automation.controls().to_vec(),
            preflight,
            audit: snapshot,
            render_errors,
        }
    }

    /// The record-side JSON of the resolved controls, in track order.
    fn controls_json(&self) -> String {
        let rows: Vec<String> = self
            .controls
            .iter()
            .map(|control| {
                format!(
                    concat!(
                        "{{\"track_id\":\"{track}\",\"slot_id\":\"{slot}\",",
                        "\"effect\":\"{effect}\",\"parameter\":\"{parameter}\",",
                        "\"parameter_index\":{index},\"lowering\":\"{lowering}\",",
                        "\"base\":{base},\"step\":{step},",
                        "\"even_value\":{even},\"odd_value\":{odd}}}"
                    ),
                    track = json_escape(control.control.track_id),
                    slot = json_escape(&control.slot_id),
                    effect = control.control.effect.contract_id(),
                    parameter = control.control.parameter,
                    index = control.control.parameter_index,
                    lowering = control.control.effect.lowering().name(),
                    base = control.base,
                    step = control.control.step,
                    even = control.values[0],
                    odd = control.values[1],
                )
            })
            .collect();
        format!("[{}]", rows.join(","))
    }

    /// One `{"arm": value}` object over the preflight arms, in their order.
    fn preflight_json(&self, value: impl Fn(usize) -> String) -> String {
        let fields: Vec<String> = mixing_automation::PREFLIGHT_ARMS
            .iter()
            .enumerate()
            .map(|(index, (arm, effects))| {
                format!(
                    "\"{}\":{}",
                    mixing_automation::preflight_arm_name(*arm, *effects),
                    value(index)
                )
            })
            .collect();
        format!("{{{}}}", fields.join(","))
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let automated = Percentiles::from_samples(&self.ns_per_block[2]);
        let workload = mixing_automation::WORKLOAD;
        let owner_edits = self
            .controls
            .iter()
            .filter(|control| control.control.effect.lowering() == Lowering::OwnerBoth)
            .count();
        let parameter_records = 2 * self
            .controls
            .iter()
            .filter(|control| control.control.effect.lowering() == Lowering::LeftThenRight)
            .count();
        let counters = |[collapsed, cohorts]: [u64; 2]| format!("[{collapsed},{cohorts}]");
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_mixing_automation\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},",
                "\"synthetic_fixture\":{synthetic},\"strip_content\":\"{strip}\",",
                "\"strip_layout\":\"{layout}\",\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"preroll_blocks\":{preroll},",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"automated_controls\":{controls},",
                "\"owner_edits_per_block\":{owner_edits},",
                "\"parameter_records_per_block\":{parameter_records},",
                "\"smoothing_samples\":{smoothing},",
                "\"restated_pushes_accepted\":{restated_pushes},",
                "\"automated_pushes_accepted\":{automated_pushes},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p95_ns\":{q95},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"automated_p50_ns\":{a50},\"automated_p95_ns\":{a95},\"automated_p99_ns\":{a99},",
                "\"paired_ramp_delta_median_ns\":{ramp},",
                "\"paired_collapse_delta_median_ns\":{collapse_delta},",
                "\"quiet_bank_collapse_counters\":{quiet_collapse},",
                "\"restated_bank_collapse_counters\":{restated_collapse},",
                "\"automated_bank_collapse_counters\":{automated_collapse},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"automated_output_sha256\":\"{ad}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "\"preflight_blocks\":{preflight_blocks},",
                "\"preflight_output_sha256\":{preflight_digests},",
                "\"preflight_bank_collapse_counters\":{preflight_collapse},",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            kind = MIXING_WORKLOAD_KIND,
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = self.observations,
            preroll = mixing_automation::PREROLL_BLOCKS,
            arm0 = mixing_automation::ARMS[0].name(),
            arm1 = mixing_automation::ARMS[1].name(),
            arm2 = mixing_automation::ARMS[2].name(),
            controls = self.controls_json(),
            owner_edits = owner_edits,
            parameter_records = parameter_records,
            smoothing = mixing_automation::SMOOTHING_SAMPLES,
            restated_pushes = self.tallies[1].accepted,
            automated_pushes = self.tallies[2].accepted,
            q50 = quiet.p50,
            q95 = quiet.p95,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            a50 = automated.p50,
            a95 = automated.p95,
            a99 = automated.p99,
            ramp = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]),
            collapse_delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            quiet_collapse = counters(self.collapse[0]),
            restated_collapse = counters(self.collapse[1]),
            automated_collapse = counters(self.collapse[2]),
            qd = self.digests[0],
            rd = self.digests[1],
            ad = self.digests[2],
            preflight_blocks = self.preflight.blocks,
            preflight_digests =
                self.preflight_json(|index| format!("\"{}\"", self.preflight.digests[index])),
            preflight_collapse =
                self.preflight_json(|index| counters(self.preflight.collapse[index])),
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = MIXING_STATISTICAL_METHOD,
        )
    }
}

// ---------------------------------------------------------------------------------------------
// The fader-ride measurement (issue #1750): the standing console riding a fader and a pan per bank.
// ---------------------------------------------------------------------------------------------

/// The record kind of the fader-ride row.
const FADER_RIDE_WORKLOAD_KIND: &str = "sixty_four_track_console_fader_ride";

/// The two timed arms, in record order: no writes, then the ride.
const FADER_RIDE_ARMS: [&str; 2] = ["quiet", "riding"];

/// What the row's per-block witness shows. Pinned verbatim in the validator.
const FADER_RIDE_RAMP_WITNESS: &str = "each timed block of the riding arm differs from a stepped \
twin's, replayed untimed after the run; the twin holds every target from the block's first sample, \
so the riding arm did not; after the settled pre-roll the two agree";

/// Pinned verbatim in the validator, for the reason every method sentence in this stream is.
const FADER_RIDE_STATISTICAL_METHOD: &str = "two arms alternated per observation; nearest-rank \
percentiles over per-block nanoseconds; ramp delta is riding minus quiet per observation; the \
stepped twin is replayed untimed after the run; descriptive only; no threshold";

/// The fader-ride row (issue #1750): the standing console prepared with the live lanes a product
/// host attaches for it, on the product's `Concurrent` delivery, riding one fader and one pan in
/// every bank chain, one transaction per block ([`console_workload::fader_ride`] is the ride).
///
/// # Why this row exists
///
/// Every other row renders with every fader, mute and matrix settled -- `console_automation` rides
/// a compressor threshold -- so no row reached the output section's ramping path. This one moves a
/// fader on one lane of every bank in every block, with the session's `fader_ms` ramp, which is
/// longer than a block. On the product's delivery the fader and the matrix are two chain stages, so
/// the row times the split ramping path a live console pays today; #1754 is the issue that would
/// pair them and re-run it.
///
/// # The arms
///
/// `quiet` and `riding` are the same plan with the same lanes, alternated observation by
/// observation as `console_automation`'s arms are; `quiet` takes no write. `riding - quiet`, paired
/// per observation, is the cost of the ride: the cell drains and the ramps they open.
///
/// # The witness
///
/// No production counter says a ramp ran, so the witness comes from the output. The riding arm's
/// every block, pre-roll included, is kept in storage allocated before the clock, and after the
/// timed loop a stepped twin -- the same plan, the same values as exact sets -- replays the whole
/// run and is compared block by block. Every timed block must differ, and the pre-roll shows the
/// comparison discriminates: the first block after a move differs, and once the move's ramps end
/// the two agree bit for bit. What a difference proves, and what it does not, is stated on
/// [`console_workload::fader_ride`].
struct FaderRideMeasurement {
    observations: usize,
    /// Per arm, in [`FADER_RIDE_ARMS`] order.
    ns_per_block: [Vec<u64>; 2],
    digests: [String; 2],
    ride: FaderRide,
    /// The product lane set both arms were prepared with.
    live_lanes: fader_ride::LiveLanes,
    /// Untimed blocks rendered before the clock: the signal settle and one settled move.
    preroll_blocks: u64,
    /// Timed blocks the riding arm rendered differently from its stepped twin.
    ramp_in_flight_blocks: u64,
    /// `[fader records, matrix records]` written to the riding arm over the timed blocks.
    written: [u64; 2],
    /// Transactions the riding arm's gate published over the timed blocks.
    transactions: u64,
    /// The riding arm's `live_values_superseded` after the run.
    superseded: u64,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl FaderRideMeasurement {
    fn run() -> Self {
        Self::run_for(OBSERVATIONS)
    }

    /// [`FaderRideMeasurement::run`] over `observations` timed blocks. Only the unit test below
    /// passes anything but [`OBSERVATIONS`], and the record says so.
    fn run_for(observations: usize) -> Self {
        let mut arms =
            [(); 2].map(|()| SessionRuntime::build_with_product_lanes(fader_ride::WORKLOAD));
        let live_lanes = arms[0].live_lanes().expect("a product lane set");
        let ride = FaderRide::resolve(&arms[1]);
        let settle = ride.settle_blocks();
        let preroll_blocks = 2 * settle;
        let block_samples = 2 * QUANTUM;
        // Every block the riding arm renders, pre-roll included, for the twin's replay. Allocated
        // here, before the clock.
        let mut riding_blocks =
            vec![0.0_f32; (preroll_blocks as usize + observations) * block_samples];
        let mut keep = |runtime: &SessionRuntime, block: u64| {
            let start = block as usize * block_samples;
            riding_blocks[start..start + block_samples].copy_from_slice(runtime.output());
        };

        // Untimed pre-roll, the same on both arms but for the riding arm's one move: the signal
        // reaches the output section with nothing moved, then the riding arm moves to the odd
        // block's values and every ramp of that move ends.
        for block in 0..preroll_blocks {
            if block == settle {
                ride.write(&mut arms[1], 1, false);
            }
            for arm in &mut arms {
                let _ = arm.render(block);
            }
            keep(&arms[1], block);
        }

        let revision = arms[1].published_revision();
        let mut samples = [(); 2].map(|()| Vec::with_capacity(observations));
        let mut hashes = [(); 2].map(|()| Sha256Sink::new());
        let mut written = [0_u64; 2];
        let mut render_errors = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..observations as u64 {
            let block = preroll_blocks + observation;
            for (index, arm) in arms.iter_mut().enumerate() {
                // Control-plane work, outside the clock, where a host does it: one transaction of
                // cell writes on the riding arm. The drain and the ramps it opens are what the
                // clock holds.
                if index == 1 {
                    let [faders, matrices] = ride.write(arm, observation, false);
                    written[0] += faders;
                    written[1] += matrices;
                }
                let (elapsed_ns, result) = timing::timed(|| arm.render(block));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            for (index, arm) in arms.iter().enumerate() {
                arm.hash_output(&mut hashes[index]);
            }
            keep(&arms[1], block);
        }
        let snapshot = audit::snapshot();
        let transactions = arms[1].published_revision() - revision;
        let superseded = arms[1].live_values_superseded();
        let digests = hashes.map(Sha256Sink::finish_hex);
        drop(arms);

        // The witness, replayed untimed: a twin takes the riding arm's every write as an exact
        // set, and each of its blocks is compared with the riding arm's kept block.
        let mut stepped = SessionRuntime::build_with_product_lanes(fader_ride::WORKLOAD);
        let mut differs = |stepped: &mut SessionRuntime, block: u64| {
            if stepped.render(block).is_err() {
                render_errors += 1;
            }
            let start = block as usize * block_samples;
            let kept = &riding_blocks[start..start + block_samples];
            stepped
                .output()
                .iter()
                .zip(kept)
                .any(|(left, right)| left.to_bits() != right.to_bits())
        };
        for block in 0..preroll_blocks {
            if block == settle {
                ride.write(&mut stepped, 1, true);
            }
            let differed = differs(&mut stepped, block);
            if block == settle {
                assert!(
                    differed,
                    "fader ride: a ramping block rendered its stepped twin's bits"
                );
            }
            if block + 1 == preroll_blocks {
                assert!(
                    !differed,
                    "fader ride: a settled ride did not render its stepped twin's bits"
                );
            }
        }
        let mut ramp_in_flight_blocks = 0_u64;
        for observation in 0..observations as u64 {
            ride.write(&mut stepped, observation, true);
            ramp_in_flight_blocks += u64::from(differs(&mut stepped, preroll_blocks + observation));
        }

        let observed = observations as u64;
        assert_eq!(
            ramp_in_flight_blocks, observed,
            "fader ride: a timed block rendered its stepped twin's bits"
        );
        assert_eq!(
            written,
            [
                ride.fader_records_per_block() * observed,
                ride.matrix_records_per_block() * observed
            ],
            "fader ride: a block wrote another record count"
        );
        assert_eq!(
            transactions, observed,
            "fader ride: one transaction per block"
        );
        assert_eq!(superseded, 0, "fader ride: a ridden value was superseded");
        assert_ne!(
            digests[0], digests[1],
            "fader ride: the riding arm rendered the quiet arm's output"
        );
        Self {
            observations,
            ns_per_block: samples,
            digests,
            ride,
            live_lanes,
            preroll_blocks,
            ramp_in_flight_blocks,
            written,
            transactions,
            superseded,
            audit: snapshot,
            render_errors,
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let riding = Percentiles::from_samples(&self.ns_per_block[1]);
        let workload = fader_ride::WORKLOAD;
        let tracks = |ids: &mut dyn Iterator<Item = &str>| {
            let quoted: Vec<String> = ids.map(|id| format!("\"{}\"", json_escape(id))).collect();
            format!("[{}]", quoted.join(","))
        };
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_fader_ride\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},",
                "\"synthetic_fixture\":{synthetic},\"strip_content\":\"{strip}\",",
                "\"strip_layout\":\"{layout}\",\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"preroll_blocks\":{preroll},",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\"],",
                "\"control_delivery\":\"concurrent\",\"live_lanes\":\"{lanes}\",",
                "\"banks\":{banks},\"fader_tracks\":{fader_tracks},\"pan_tracks\":{pan_tracks},",
                "\"fader_step_db\":{fader_step},\"pan_step\":{pan_step},",
                "\"fader_smoothing_samples\":{fader_smoothing},",
                "\"pan_smoothing_samples\":{pan_smoothing},",
                "\"fader_records_per_block\":{fader_per_block},",
                "\"matrix_records_per_block\":{matrix_per_block},",
                "\"fader_records_written\":{fader_written},",
                "\"matrix_records_written\":{matrix_written},",
                "\"transactions_published\":{transactions},",
                "\"live_values_superseded\":{superseded},",
                "\"ramp_in_flight_blocks\":{in_flight},\"ramp_witness\":\"{witness}\",",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p95_ns\":{q95},\"quiet_p99_ns\":{q99},",
                "\"riding_p50_ns\":{r50},\"riding_p95_ns\":{r95},\"riding_p99_ns\":{r99},",
                "\"paired_ramp_delta_median_ns\":{ramp},",
                "\"paired_ramp_delta_median_ns_per_track\":{ramp_per_track},",
                "\"quiet_output_sha256\":\"{qd}\",\"riding_output_sha256\":\"{rd}\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            kind = FADER_RIDE_WORKLOAD_KIND,
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = self.observations,
            preroll = self.preroll_blocks,
            arm0 = FADER_RIDE_ARMS[0],
            arm1 = FADER_RIDE_ARMS[1],
            lanes = fader_ride::lane_set_name(self.live_lanes),
            banks = self.ride.banks(),
            fader_tracks = tracks(&mut self.ride.faders().iter().map(|f| &*f.track_id)),
            pan_tracks = tracks(&mut self.ride.pans().iter().map(|p| &*p.track_id)),
            fader_step = fader_ride::FADER_STEP_DB,
            pan_step = fader_ride::PAN_STEP,
            fader_smoothing = self.ride.fader_smoothing(),
            pan_smoothing = self.ride.pan_smoothing(),
            fader_per_block = self.ride.fader_records_per_block(),
            matrix_per_block = self.ride.matrix_records_per_block(),
            fader_written = self.written[0],
            matrix_written = self.written[1],
            transactions = self.transactions,
            superseded = self.superseded,
            in_flight = self.ramp_in_flight_blocks,
            witness = FADER_RIDE_RAMP_WITNESS,
            q50 = quiet.p50,
            q95 = quiet.p95,
            q99 = quiet.p99,
            r50 = riding.p50,
            r95 = riding.p95,
            r99 = riding.p99,
            ramp = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            ramp_per_track = format_f64(
                paired_median(&self.ns_per_block[1], &self.ns_per_block[0]) as f64
                    / f64::from(workload.tracks())
            ),
            qd = self.digests[0],
            rd = self.digests[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata.record_fields(),
            method = FADER_RIDE_STATISTICAL_METHOD,
        )
    }
}

/// One arm of the product-lane row: the session it renders (read against that session's own
/// row), the lanes it attaches, and its record kind.
struct ProductLaneArm {
    workload: Workload,
    group: LaneGroup,
    kind: &'static str,
}

/// The product-lane row's arms in emission order: issue #1758's product set first, then issue
/// #1763's lane groups and the app-shape session's product set.
const PRODUCT_LANE_ARMS: [ProductLaneArm; 5] = [
    ProductLaneArm {
        workload: Workload::SixtyFourTrackConsole,
        group: LaneGroup::Product,
        kind: "sixty_four_track_console_product_lanes",
    },
    ProductLaneArm {
        workload: Workload::SixtyFourTrackConsole,
        group: LaneGroup::FaderMatrix,
        kind: "sixty_four_track_console_fader_matrix_lanes",
    },
    ProductLaneArm {
        workload: Workload::SixtyFourTrackConsole,
        group: LaneGroup::StripInput,
        kind: "sixty_four_track_console_strip_input_lanes",
    },
    ProductLaneArm {
        workload: Workload::SixtyFourTrackConsole,
        group: LaneGroup::Effects,
        kind: "sixty_four_track_console_effect_lanes",
    },
    ProductLaneArm {
        workload: Workload::SixtyFourTrackAppShape,
        group: LaneGroup::Product,
        kind: "sixty_four_track_app_shape_product_lanes",
    },
];

/// Held by the record's key list only: the subject writes it as a constant, so the validator does
/// not pin its text.
const PRODUCT_LANES_STATISTICAL_METHOD: &str = "one plan rendered back to back by the session \
rows' loop; nearest-rank percentiles over per-block nanoseconds; one warmup pass and two measured \
rounds; descriptive only; no threshold";

/// The product-lane row (issue #1758): the standing console prepared with the live lanes a product
/// host attaches for it (`controls::live_lanes`, on the product's `Concurrent` delivery), with no
/// write, rendered back to back by [`SessionMeasurement::run_prepared`], the loop that times
/// `sixty_four_track_console`.
///
/// # Why this row exists
///
/// In #1750's run the fader-ride row's `quiet` arm -- this plan, no writes -- took about ten per
/// cent longer per block than the standing row, which renders the same session with no live lane.
/// That arm alternates two plans per observation, so the gap could be the lanes' per-block cell
/// reads or the two plans' cache residency. This row removes the alternation: one plan, timed by
/// the standing row's own loop over the same block window, so its p50 minus the standing row's is
/// what the lanes cost a console that nobody edits.
///
/// Nobody writes a lane, so every lane reads the value the plan was prepared with and the row
/// renders the standing row's bits: the run asserts it before it emits a record, and the aggregate
/// validator pins it. The record states the lane set and that no transaction was published.
struct ProductLanesMeasurement {
    /// The arm measured.
    arm: &'static ProductLaneArm,
    /// The timed loop's output: the session rows' own measurement.
    session: SessionMeasurement,
    /// The lane groups the plan attached ([`SessionRuntime::lane_set_name`]).
    live_lanes: String,
    /// Transactions the plan's gate published over the run: none, because nobody writes.
    transactions: u64,
    /// The plan's `live_values_superseded` after the run.
    superseded: u64,
}

impl ProductLanesMeasurement {
    fn run(arm: &'static ProductLaneArm) -> Self {
        Self::run_for(arm, OBSERVATIONS)
    }

    /// [`ProductLanesMeasurement::run`] over `observations` timed blocks. Only the unit test below
    /// passes anything but [`OBSERVATIONS`], and the record says so.
    fn run_for(arm: &'static ProductLaneArm, observations: usize) -> Self {
        let workload = arm.workload;
        let mut runtime = SessionRuntime::build_with_lane_group(workload, arm.group);
        let live_lanes = runtime.lane_set_name().expect("a lane group");
        let revision = runtime.published_revision();
        let session = SessionMeasurement::run_prepared(workload, &mut runtime, observations);
        Self {
            arm,
            session,
            live_lanes,
            transactions: runtime.published_revision() - revision,
            superseded: runtime.live_values_superseded(),
        }
    }

    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let percentiles = Percentiles::from_samples(&self.session.ns_per_block);
        let workload = self.arm.workload;
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_product_lanes\",",
                "\"workload_kind\":\"{kind}\",\"standing_row\":\"{standing}\",",
                "\"tracks\":{tracks},\"synthetic_fixture\":{synthetic},",
                "\"strip_content\":\"{strip}\",\"strip_layout\":\"{layout}\",",
                "\"input_signal\":\"{signal}\",\"source_feed\":\"{feed}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},",
                "\"observations\":{obs},\"control_delivery\":\"concurrent\",",
                "\"live_lanes\":\"{lanes}\",\"transactions_published\":{transactions},",
                "\"live_values_superseded\":{superseded},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"min_ns_per_block\":{min},\"p50_ns_per_block\":{p50},",
                "\"p95_ns_per_block\":{p95},\"p99_ns_per_block\":{p99},",
                "\"max_ns_per_block\":{max},\"output_sha256\":\"{digest}\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"{method}\"}}"
            ),
            issue = ISSUE,
            kind = self.arm.kind,
            standing = workload.kind(),
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            layout = workload.strip_layout(),
            signal = workload.input_signal(),
            feed = workload.source_feed().name(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = self.session.observations,
            lanes = self.live_lanes,
            transactions = self.transactions,
            superseded = self.superseded,
            min = percentiles.min,
            p50 = percentiles.p50,
            p95 = percentiles.p95,
            p99 = percentiles.p99,
            max = percentiles.max,
            digest = self.session.output_sha256,
            errors = self.session.render_errors,
            forbidden = self.session.audit.total(),
            metadata = metadata.record_fields(),
            method = PRODUCT_LANES_STATISTICAL_METHOD,
        )
    }
}

/// The median of the per-observation differences `left[i] - right[i]`.
///
/// Pairs taken microseconds apart, then summarised -- never a difference of two summaries taken
/// minutes apart, which is the whole point of alternating the arms.
fn paired_median(left: &[u64], right: &[u64]) -> i64 {
    let mut paired: Vec<i64> = left
        .iter()
        .zip(right)
        .map(|(left, right)| *left as i64 - *right as i64)
        .collect();
    paired.sort_unstable();
    paired[paired.len() / 2]
}

// ---------------------------------------------------------------------------------------------
// Shared plumbing.
// ---------------------------------------------------------------------------------------------

pub(crate) fn backend_name(backend: Backend) -> &'static str {
    // By width, not by variant: `Backend::Scalar` exists only in `lane/test-support` builds
    // (#1059), so a match on the variants compiles in only one of them.
    match backend.width() {
        4 => "Simd4",
        8 => "Simd8",
        _ => "Scalar",
    }
}

/// Where the bench's render thread and the live row's producer thread run (issue #938).
///
/// The runner pins the process with `taskset`; this module pins each thread inside it. The render
/// thread (the one that runs `main`, and so renders every row) goes to the runner's core,
/// `MISO_ENGINE_BENCH_CPU_AFFINITY`, and the producer to the one other core of the process mask
/// the runner launched it with. The placement is then read from the kernel's topology rather than
/// taken on trust: `same_ccd` is a core that shares the render core's L3 and is not its SMT
/// sibling, the production placement the row measures.
///
/// Thread pinning is Linux-only, and so is this module's body: on any other target the bench pins
/// nothing and names the placement `unsupported_platform`, which a controlled run refuses. That is
/// a property of this tool, never of the engine.
pub(crate) mod affinity {
    use bench_support::metadata::Metadata;

    /// The render core and the producer core, and where the second sits relative to the first.
    pub(crate) struct Placement {
        /// The producer thread's core, or `None` when the producer is not pinned.
        pub(crate) producer_cpu: Option<usize>,
        /// `same_ccd`, `smt_sibling`, `other_l3`, `same_cpu`, `topology_unavailable`,
        /// `no_producer_cpu`, `unpinned` or `unsupported_platform`.
        pub(crate) name: &'static str,
    }

    impl Placement {
        /// No core named: the render thread stays where it is and the producer runs unpinned.
        #[cfg(test)]
        pub(crate) const UNPINNED: Self = Self {
            producer_cpu: None,
            name: "unpinned",
        };

        /// Pins the calling thread to the runner's core and chooses the producer's core: the
        /// process mask, read before the pin, less the render core, when exactly one core is left.
        ///
        /// Without a runner core (an uncontrolled run) nothing is pinned and the placement is
        /// `unpinned`.
        ///
        /// # Panics
        ///
        /// Panics if the runner named a core the kernel refuses to pin this thread to, or if the
        /// thread is not running there afterwards.
        pub(crate) fn pin_render_thread(metadata: &Metadata) -> Self {
            let render_cpu = metadata
                .var("MISO_ENGINE_BENCH_CPU_AFFINITY")
                .ok()
                .and_then(|cpu| cpu.parse::<usize>().ok());
            imp::pin_render_thread(render_cpu)
        }

        /// The closure the live producer thread runs first: pins it to the producer core, or does
        /// nothing when the producer is not pinned.
        pub(crate) fn producer_pin(&self) -> impl FnOnce() + Send + 'static {
            let cpu = self.producer_cpu;
            move || {
                if let Some(cpu) = cpu {
                    imp::pin_this_thread(cpu);
                }
            }
        }

        /// Refuses, before anything is timed, a controlled run whose producer is not on a core of
        /// the render core's L3 other than its SMT sibling ([`Placement::refusal`]).
        ///
        /// # Panics
        ///
        /// Panics with the refusal when the run is controlled and the placement is not `same_ccd`.
        pub(crate) fn refuse_unplaced_producer(&self, metadata: &Metadata) {
            let controlled = metadata
                .var("MISO_ENGINE_BENCH_MEASUREMENT_CONTROL")
                .is_ok_and(|control| control == "controlled");
            if let Some(refusal) = self.refusal(controlled) {
                panic!("{refusal}");
            }
        }

        /// The refusal of a run whose producer placement it cannot publish, or `None`.
        ///
        /// A controlled number for a producer on the render core itself, or on its sibling, or
        /// with no core of its own, would measure another placement under the row's name, so a
        /// controlled run admits `same_ccd` alone. An uncontrolled run states its placement and the
        /// validator reads it.
        pub(crate) fn refusal(&self, controlled: bool) -> Option<String> {
            (controlled && self.name != "same_ccd").then(|| {
                format!(
                    "refusing the run (live-producer row placement): producer placement {} (a \
                     controlled run needs the process mask to hold the render core and one core \
                     that shares its L3 and is not its SMT sibling)",
                    self.name
                )
            })
        }
    }

    /// The cores of a kernel CPU list such as `12-15,28-31`, or `None` if it does not parse.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn parse_cpu_list(list: &str) -> Option<Vec<usize>> {
        let mut cpus = Vec::new();
        for part in list.trim().split(',') {
            match part.split_once('-') {
                Some((first, last)) => {
                    let (first, last) = (first.parse().ok()?, last.parse::<usize>().ok()?);
                    if first > last {
                        return None;
                    }
                    cpus.extend(first..=last);
                }
                None => cpus.push(part.parse().ok()?),
            }
        }
        Some(cpus)
    }

    #[cfg(target_os = "linux")]
    mod imp {
        use super::{Placement, parse_cpu_list};
        use rustix::thread::{CpuSet, sched_getaffinity, sched_getcpu, sched_setaffinity};

        pub(super) fn pin_render_thread(render_cpu: Option<usize>) -> Placement {
            let Some(render_cpu) = render_cpu else {
                return Placement {
                    producer_cpu: None,
                    name: "unpinned",
                };
            };
            let mask = sched_getaffinity(None).expect("the process affinity mask");
            let others: Vec<usize> = (0..CpuSet::MAX_CPU)
                .filter(|&cpu| cpu != render_cpu && mask.is_set(cpu))
                .collect();
            pin_this_thread(render_cpu);
            match others.as_slice() {
                [producer_cpu] => Placement {
                    producer_cpu: Some(*producer_cpu),
                    name: classify(render_cpu, *producer_cpu),
                },
                _ => Placement {
                    producer_cpu: None,
                    name: "no_producer_cpu",
                },
            }
        }

        pub(super) fn pin_this_thread(cpu: usize) {
            let mut set = CpuSet::new();
            set.set(cpu);
            sched_setaffinity(None, &set)
                .unwrap_or_else(|error| panic!("pinning a bench thread to cpu {cpu}: {error}"));
            // The kernel moves the calling thread before `sched_setaffinity` returns.
            assert_eq!(
                sched_getcpu(),
                cpu,
                "a bench thread is not running on its core"
            );
        }

        /// Where `producer` sits relative to `render`, read from sysfs.
        fn classify(render: usize, producer: usize) -> &'static str {
            let read = |path: String| std::fs::read_to_string(path).ok();
            let cpu = format!("/sys/devices/system/cpu/cpu{render}");
            let siblings = read(format!("{cpu}/topology/thread_siblings_list"))
                .and_then(|list| parse_cpu_list(&list));
            let l3 = (0..16)
                .find(|index| {
                    read(format!("{cpu}/cache/index{index}/level"))
                        .as_deref()
                        .map(str::trim)
                        == Some("3")
                })
                .and_then(|index| read(format!("{cpu}/cache/index{index}/shared_cpu_list")))
                .and_then(|list| parse_cpu_list(&list));
            match (siblings, l3) {
                _ if producer == render => "same_cpu",
                (Some(siblings), _) if siblings.contains(&producer) => "smt_sibling",
                (Some(_), Some(l3)) if l3.contains(&producer) => "same_ccd",
                (Some(_), Some(_)) => "other_l3",
                _ => "topology_unavailable",
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    mod imp {
        use super::Placement;

        pub(super) fn pin_render_thread(_render_cpu: Option<usize>) -> Placement {
            Placement {
                producer_cpu: None,
                name: "unsupported_platform",
            }
        }

        pub(super) fn pin_this_thread(_cpu: usize) {
            unreachable!("no producer core is chosen on this platform");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console_workload::WEB_METER_BLOCKS;

    /// Timed observations in the short run: two meter windows, so the meters-on arm publishes.
    const SHORT_RUN_OBSERVATIONS: usize = 8;

    /// Issue #914: the `console_meters` record states both arms' fold and redirect counters, and
    /// on the sixty-four-track console the metered arm folds every route the unmetered arm folds.
    ///
    /// A short run of the real subject -- the same two `SessionRuntime` arms, the same warm-up, the
    /// same counter read, eight timed observations instead of a thousand -- so the record it prints
    /// is a descriptive `console_meters` record with the four counters populated
    /// (`cargo test -p bench the_meters_record -- --nocapture`). Its timings are not to be read, and
    /// its `observations` field says eight, which the validator refuses as a frozen measurement.
    ///
    /// Red mutations: emit `self.bank_scatter_redirects[1]` as `meters_on_bank_route_folds`, or
    /// misspell one of the four keys in the format string -- the per-key check fails; read the
    /// arms' `bank_scatter_redirects` into `bank_route_folds` -- the fold assertion fails; emit
    /// `OBSERVATIONS` rather than `self.observations` -- the shortened-run check fails.
    #[test]
    fn the_meters_record_carries_each_arms_fold_and_redirect_counters() {
        let meters = FacilityMeasurement::run_for(&METER_CONFIGS, SHORT_RUN_OBSERVATIONS);
        let tracks = u64::from(FACILITY_WORKLOAD.tracks());
        assert_eq!(
            meters.bank_route_folds,
            [tracks, tracks],
            "every route of the console folds, unmetered and metered alike (the #885 contract)"
        );
        assert_eq!(
            meters.bank_scatter_redirects[1], meters.bank_scatter_redirects[0],
            "a post-matrix meter moves no scatter redirect (the #886 contract)"
        );
        let record = meters.meters_record(1, Backend::current(), Metadata::gather());
        println!("{record}");
        for (key, value) in [
            ("meters_off_bank_route_folds", meters.bank_route_folds[0]),
            ("meters_on_bank_route_folds", meters.bank_route_folds[1]),
            (
                "meters_off_bank_scatter_redirects",
                meters.bank_scatter_redirects[0],
            ),
            (
                "meters_on_bank_scatter_redirects",
                meters.bank_scatter_redirects[1],
            ),
        ] {
            assert_eq!(
                record.matches(&format!("\"{key}\":")).count(),
                1,
                "{key} must appear exactly once"
            );
            assert!(
                record.contains(&format!("\"{key}\":{value},")),
                "{key} must carry {value}"
            );
        }
        assert!(
            record.contains(&format!("\"observations\":{SHORT_RUN_OBSERVATIONS},")),
            "a shortened run must say it was shortened"
        );
    }

    /// Whether the strict record validator (`scripts/console-benchmark-record-validator.jq`)
    /// accepts `record` after the jq edit `edit`.
    ///
    /// Panics when jq did not run or did not return a verdict, so a broken invocation cannot read
    /// as a refusal: exit 0 is an accepted record and exit 1 a refused one, and nothing else is
    /// either.
    fn record_validator_accepts(record: &str, edit: &str) -> bool {
        let scripts = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts");
        let jq = |arguments: &[&str], input: &str| {
            let mut child = std::process::Command::new("jq")
                .args(arguments)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("jq runs");
            std::io::Write::write_all(&mut child.stdin.take().expect("jq stdin"), input.as_bytes())
                .expect("jq takes the record");
            child.wait_with_output().expect("jq finishes")
        };
        let edited = jq(&["-c", edit], record);
        assert!(
            edited.status.success(),
            "the edit {edit:?} did not apply: {}",
            String::from_utf8_lossy(&edited.stderr)
        );
        let edited = String::from_utf8(edited.stdout).expect("jq writes UTF-8");
        let library = scripts.to_str().expect("a UTF-8 scripts path");
        let validator = scripts.join("console-benchmark-record-validator.jq");
        let verdict = jq(
            &[
                "-e",
                "-L",
                library,
                "-f",
                validator.to_str().expect("a UTF-8 validator path"),
            ],
            &edited,
        );
        match verdict.status.code() {
            Some(0) => true,
            Some(1) => false,
            status => panic!(
                "the record validator returned no verdict ({status:?}): {}",
                String::from_utf8_lossy(&verdict.stderr)
            ),
        }
    }

    /// Issue #1003: the mixing-automation row, run short through the real subject, prints its
    /// controls, its host lowerings, its preflight and its collapse counters, and the record
    /// validator pins them.
    ///
    /// The same `MixingAutomationMeasurement` the runner's run takes -- the preflight asserted,
    /// then the three arms alternated -- over eight timed blocks instead of a thousand (`cargo
    /// test -p bench the_mixing_automation_row -- --nocapture` prints the record). Its timings are
    /// not to be read. Its `observations`, its push counts and its quiet arm's collapse counter say
    /// it was shortened, so the validator is asked about it with those set to what the frozen run
    /// produces, and then with each claim the row makes taken away.
    ///
    /// Red mutations (run): push the EQ as `LeftThenRight` -- the preflight's collapse premise
    /// fails; ride the limiter at 0.25 dB -- its bit-movement premise fails; restate a value that
    /// is not held -- the in-run digest equality fails.
    #[test]
    fn the_mixing_automation_row_prints_its_controls_and_the_validator_pins_them() {
        let measured =
            MixingAutomationMeasurement::run_for(Backend::current(), SHORT_RUN_OBSERVATIONS);
        assert_eq!(measured.render_errors, 0);
        assert_eq!(
            measured.audit.total(),
            0,
            "a forbidden operation on the render path"
        );
        let record = measured.record(1, Backend::current(), Metadata::gather());
        println!("{record}");
        for key in [
            "quiet_bank_collapse_counters",
            "restated_bank_collapse_counters",
            "automated_bank_collapse_counters",
            "preflight_bank_collapse_counters",
            "preflight_output_sha256",
            "automated_controls",
        ] {
            assert_eq!(
                record.matches(&format!("\"{key}\":")).count(),
                1,
                "{key} appears exactly once"
            );
        }
        let frozen = concat!(
            ".observations = 1000 | .restated_pushes_accepted = 13000 | ",
            ".automated_pushes_accepted = 13000 | ",
            ".quiet_bank_collapse_counters[0] = .quiet_bank_collapse_counters[1] * 1064"
        );
        assert!(
            !record_validator_accepts(&record, "."),
            "a shortened run must not pass for the frozen one"
        );
        assert!(
            record_validator_accepts(&record, frozen),
            "the mixing-automation record at the frozen counts"
        );
        for (edit, why) in [
            (
                "del(.quiet_bank_collapse_counters)",
                "a record missing its collapse counters",
            ),
            (
                "del(.preflight_bank_collapse_counters)",
                "a record missing its preflight collapse counters",
            ),
            (
                ".automated_controls[0].lowering = \"left_then_right\"",
                "an EQ pushed as two one-channel edits",
            ),
            (
                concat!(
                    ".preflight_output_sha256.automated_limiter_only = ",
                    ".preflight_output_sha256.restated"
                ),
                "a limiter ride that moved no bit",
            ),
            (
                ".preflight_bank_collapse_counters.restated_eq_only = [0, 8]",
                "an EQ restatement that retired the collapse",
            ),
            (
                ".automated_output_sha256 = .restated_output_sha256",
                "an automated arm that rendered the restated bits",
            ),
        ] {
            assert!(
                !record_validator_accepts(&record, &format!("{frozen} | {edit}")),
                "{why}"
            );
        }
    }

    /// Issue #1750: the fader-ride row, run short through the real subject, differs from its
    /// stepped twin in every timed block, writes exactly its records, renders with no forbidden
    /// operation, and prints a record the validator accepts at the frozen counts and refuses as
    /// shortened.
    ///
    /// The same `FaderRideMeasurement` the runner's run takes -- the quiet and riding arms timed
    /// and alternated, then the stepped twin's untimed replay with its pre-roll checks -- over
    /// eight timed blocks instead of a thousand (`cargo test -p bench the_fader_ride_row --
    /// --nocapture` prints the record).
    /// Its timings are not to be read. The validator's own mutation cases are in
    /// `scripts/test-console-benchmark.sh`.
    ///
    /// Red mutations (run): write the pre-roll move stepped -- its first-block check fails; write
    /// the timed blocks' values stepped -- the ramp-in-flight count fails; write a second
    /// transaction per block -- the transaction count fails; rename a key the validator library
    /// lists -- the frozen record is refused.
    #[test]
    fn the_fader_ride_row_prints_its_witness_and_the_validator_pins_it() {
        let measured = FaderRideMeasurement::run_for(SHORT_RUN_OBSERVATIONS);
        assert_eq!(measured.render_errors, 0);
        assert_eq!(
            measured.audit.total(),
            0,
            "a forbidden operation on the render path"
        );
        let record = measured.record(1, Backend::current(), Metadata::gather());
        println!("{record}");
        let frozen = concat!(
            ".observations = 1000 | .transactions_published = 1000 | ",
            ".ramp_in_flight_blocks = 1000 | ",
            ".fader_records_written = .fader_records_per_block * 1000 | ",
            ".matrix_records_written = .matrix_records_per_block * 1000"
        );
        assert!(
            !record_validator_accepts(&record, "."),
            "a shortened run must not pass for the frozen one"
        );
        assert!(
            record_validator_accepts(&record, frozen),
            "the fader-ride record at the frozen counts"
        );
    }

    /// Issues #1758 and #1763: every product-lane arm, run short through the real subject,
    /// renders its standing row's bits over the same window with no forbidden operation and no
    /// write, and prints a record the validator accepts at the frozen count and refuses as
    /// shortened.
    ///
    /// The same `ProductLanesMeasurement` the runner's run takes, over eight timed blocks instead of
    /// a thousand (`cargo test -p bench the_product_lane_row -- --nocapture` prints the records).
    /// Its timings are not to be read. The validator's own mutation cases are in
    /// `scripts/test-console-benchmark.sh`.
    #[test]
    fn the_product_lane_row_renders_the_standing_bits_and_the_validator_pins_it() {
        for arm in &PRODUCT_LANE_ARMS {
            let measured = ProductLanesMeasurement::run_for(arm, SHORT_RUN_OBSERVATIONS);
            let standing = SessionMeasurement::run_for(arm.workload, SHORT_RUN_OBSERVATIONS);
            assert_eq!(
                measured.session.output_sha256, standing.output_sha256,
                "the {} arm must render its standing row's bits",
                arm.kind
            );
            assert_eq!(measured.session.render_errors, 0, "{}", arm.kind);
            assert_eq!(
                measured.session.audit.total(),
                0,
                "{}: a forbidden operation on the render path",
                arm.kind
            );
            let record = measured.record(1, Backend::current(), Metadata::gather());
            println!("{record}");
            assert!(
                !record_validator_accepts(&record, "."),
                "{}: a shortened run must not pass for the frozen one",
                arm.kind
            );
            assert!(
                record_validator_accepts(&record, ".observations = 1000"),
                "{}: the product-lane record at the frozen count",
                arm.kind
            );
        }
    }

    /// Issues #928, #956 and #965: the gain/pan row, run short through the real subject, renders
    /// with no forbidden operation, prints its record with its feed, and the record validator pins
    /// that feed.
    ///
    /// The same `SessionMeasurement` the runner's run takes, over eight timed blocks instead of a
    /// thousand (`cargo test -p bench the_gain_pan_row_prints -- --nocapture` prints the record).
    /// Its timings are not to be read, and its `observations` field says eight, which the
    /// validator refuses as a frozen measurement -- so the validator is asked about the record
    /// with that one field set to the frozen count, and then with the feed removed or set to the
    /// retired `bound` feed. `render_total_forbidden_operations` is the realtime audit of the
    /// render path, the source-set driver included, and the audited allocator would have aborted
    /// this process on an allocation inside it.
    ///
    /// Red mutations: drop `source_feed` from the format string -- the feed assertions fail; drop
    /// `source_feed` from `session_keys` in the validator library -- the removed-feed case is
    /// accepted; unpin the feed -- the `bound` case is accepted.
    #[test]
    fn the_gain_pan_row_prints_its_feed_and_the_validator_pins_it() {
        let workload = Workload::SixtyFourTrackGainPanRing;
        let measured = SessionMeasurement::run_for(workload, SHORT_RUN_OBSERVATIONS);
        assert_eq!(measured.render_errors, 0);
        assert_eq!(
            measured.audit.total(),
            0,
            "a forbidden operation on the render path"
        );
        let record = measured.record(
            workload,
            1,
            Backend::current(),
            Metadata::gather(),
            None,
            None,
        );
        println!("{record}");
        assert_eq!(
            record.matches("\"source_feed\":").count(),
            1,
            "the feed is stated exactly once"
        );
        assert!(
            record.contains("\"source_feed\":\"played_planes\","),
            "the record must state the played-planes feed"
        );
        assert!(
            record.contains(&format!("\"observations\":{SHORT_RUN_OBSERVATIONS},")),
            "a shortened run must say it was shortened"
        );
        assert!(
            record.contains("\"workload_kind\":\"sixty_four_track_gain_pan_ring\","),
            "the row names itself"
        );

        let frozen = ".observations = 1000";
        assert!(
            !record_validator_accepts(&record, "."),
            "a shortened run must not pass for the frozen one"
        );
        assert!(
            record_validator_accepts(&record, frozen),
            "the record at the frozen count"
        );
        assert!(
            !record_validator_accepts(&record, &format!("{frozen} | del(.source_feed)")),
            "a record missing source_feed"
        );
        assert!(
            !record_validator_accepts(&record, &format!("{frozen} | .source_feed = \"bound\"")),
            "a record claiming the retired bound feed"
        );
    }

    /// Issue #938: the live-producer row, run short through the real subject with its producer
    /// started and joined by `SessionMeasurement`, renders with no forbidden operation and no
    /// underrun, prints its producer group once, and the record validator accepts it at the frozen
    /// count. Its bits against the ring row's are `console-workload`'s claim, not this test's.
    ///
    /// The producer is unpinned here (a test has no runner core), so the record states
    /// `unpinned` and no core, which the validator accepts only off a controlled run. The
    /// validator's own mutations of the group are `scripts/test-console-benchmark.sh`'s.
    ///
    /// Red mutations: drop `{live}` from the format string -- the group assertions fail; skip
    /// `start_live_producer` in `run_placed` -- the producer is not running and the run panics.
    #[test]
    fn the_live_producer_row_prints_its_producer_group() {
        let live = Workload::SixtyFourTrackGainPanLive;
        let measured = SessionMeasurement::run_for(live, SHORT_RUN_OBSERVATIONS);
        assert_eq!(measured.render_errors, 0);
        assert_eq!(
            measured.audit.total(),
            0,
            "a forbidden operation on the render path"
        );
        let record = measured.record(live, 1, Backend::current(), Metadata::gather(), None, None);
        println!("{record}");
        assert!(
            record.contains(
                "\"producer_cpu\":null,\"producer_placement\":\"unpinned\",\"underruns\":0,"
            ),
            "the producer group, stated once: {record}"
        );
        assert_eq!(record.matches("\"underruns\":").count(), 1);
        assert!(record.contains("\"source_feed\":\"live_producer\","));
        assert!(
            record_validator_accepts(&record, ".observations = 1000"),
            "the record at the frozen count"
        );
    }

    /// Issue #938: a controlled run admits a producer on another core of the render core's L3
    /// alone; every other placement the bench can name is refused before timing, by name, and an
    /// uncontrolled run is never refused for its placement.
    #[test]
    fn a_controlled_run_refuses_every_producer_placement_but_same_ccd() {
        let placed = |name| affinity::Placement {
            producer_cpu: None,
            name,
        };
        assert_eq!(placed("same_ccd").refusal(true), None);
        for name in [
            "smt_sibling",
            "other_l3",
            "same_cpu",
            "topology_unavailable",
            "no_producer_cpu",
            "unpinned",
            "unsupported_platform",
        ] {
            let refusal = placed(name)
                .refusal(true)
                .expect("a controlled run refuses");
            assert!(refusal.contains(name), "{refusal}");
            assert_eq!(placed(name).refusal(false), None, "{name}");
        }
    }

    /// Issue #938: the bench reads the kernel's CPU lists, ranges and single cores alike, and
    /// refuses a list it cannot read rather than guessing a core set.
    #[test]
    fn the_cpu_list_parser_reads_ranges_and_single_cores() {
        assert_eq!(
            affinity::parse_cpu_list("12-15,28-31\n"),
            Some(vec![12, 13, 14, 15, 28, 29, 30, 31])
        );
        assert_eq!(affinity::parse_cpu_list("15,31"), Some(vec![15, 31]));
        assert_eq!(affinity::parse_cpu_list("7"), Some(vec![7]));
        assert_eq!(affinity::parse_cpu_list("3-1"), None);
        assert_eq!(affinity::parse_cpu_list("a-3"), None);
    }

    /// Issue #881: the metered console row, run short through the real subject, renders the
    /// standing console row's bits with no forbidden operation, consumes every window it
    /// publishes, prints its meter group, and the record validator pins that group.
    ///
    /// The same `SessionMeasurement` the runner's run takes, over three twelve-block windows of
    /// timed blocks instead of a thousand blocks (`cargo test -p bench the_metered_console_row --
    /// --nocapture` prints the record). Its timings are not to be read. Its `observations` and its
    /// `meter_snapshots` say it was shortened, and the validator refuses it as a frozen
    /// measurement, so the validator is asked about it with those two fields set to what the frozen
    /// run consumes, and then with each field of the meter group removed, the group grafted onto
    /// the standing row, or a meter fact changed.
    ///
    /// Red mutations (run): select `MeterMetricSet::ALL` for the web meters, or stop draining the
    /// metered row in `run_for` -- the observed-shape assertion fails.
    #[test]
    fn the_metered_console_row_prints_its_meters_and_the_validator_pins_them() {
        const WINDOWS: usize = 3;
        let observations = WINDOWS * WEB_METER_BLOCKS as usize;
        let measured =
            METERED_PAIR.map(|workload| SessionMeasurement::run_for(workload, observations));
        assert_eq!(
            measured[1].output_sha256, measured[0].output_sha256,
            "the metered row must render the standing console row's bits"
        );
        for (workload, measurement) in METERED_PAIR.iter().zip(&measured) {
            assert_eq!(measurement.render_errors, 0, "{}", workload.kind());
            assert_eq!(
                measurement.audit.total(),
                0,
                "{}: a forbidden operation on the render path",
                workload.kind()
            );
        }
        assert!(
            measured[0].meters.is_none(),
            "the standing row carries no meter"
        );
        let evidence = measured[1]
            .meters
            .as_ref()
            .expect("the metered row carries its meter evidence");
        let tracks = u64::from(Workload::SixtyFourTrackConsoleMetered.tracks());
        // The metered plan's own counter, read from a fresh bind of the same row: the standing
        // row is prepared with the other delivery and is not this plan's baseline.
        let plan = SessionRuntime::new(Workload::SixtyFourTrackConsoleMetered);
        assert!(evidence.post_matrix && evidence.uniform);
        assert!(
            evidence.shape
                == Some((
                    MeterMetricSet::SAMPLE_PEAK,
                    WEB_METER_BLOCKS * QUANTUM as u32
                )),
            "every snapshot is a sample peak over a twelve-block window"
        );
        assert_eq!(
            [
                evidence.streams,
                evidence.snapshots,
                evidence.dropped(),
                evidence.bank_route_folds,
                evidence.bank_scatter_redirects,
            ],
            [
                tracks,
                tracks * WINDOWS as u64,
                0,
                tracks,
                plan.bank_scatter_redirects(),
            ],
            "[streams, snapshots, dropped, folds, redirects]: one stream per track, one snapshot \
             per track per window with none dropped, every route folded (#885) and the metered \
             plan's own redirects"
        );

        let [standing, metered] = [0, 1].map(|index| {
            measured[index].record(
                METERED_PAIR[index],
                1,
                Backend::current(),
                Metadata::gather(),
                None,
                None,
            )
        });
        println!("{metered}");
        let group = [
            ("meter_streams", tracks.to_string()),
            ("meter_tap", "\"post_matrix\"".to_string()),
            ("meter_metrics", "\"sample_peak\"".to_string()),
            ("meter_window_blocks", WEB_METER_BLOCKS.to_string()),
            ("meter_snapshots", (tracks * WINDOWS as u64).to_string()),
            ("meter_dropped_snapshots", "0".to_string()),
            ("bank_route_folds", tracks.to_string()),
            (
                "bank_scatter_redirects",
                plan.bank_scatter_redirects().to_string(),
            ),
        ];
        for (key, value) in &group {
            assert_eq!(
                metered.matches(&format!("\"{key}\":")).count(),
                1,
                "{key} appears exactly once on the metered row"
            );
            assert!(
                metered.contains(&format!("\"{key}\":{value},")),
                "{key} must carry {value}"
            );
            assert!(
                !standing.contains(&format!("\"{key}\":")),
                "the standing row's record is unchanged: no {key}"
            );
        }
        assert!(
            metered.contains("\"workload_kind\":\"sixty_four_track_console_metered\","),
            "the metered row names itself"
        );

        let frozen_snapshots = tracks * (OBSERVATIONS as u64 / u64::from(WEB_METER_BLOCKS));
        let frozen = format!(".observations = 1000 | .meter_snapshots = {frozen_snapshots}");
        assert!(
            !record_validator_accepts(&metered, ".observations = 1000"),
            "a shortened run's snapshot count must not pass for the frozen one"
        );
        assert!(
            record_validator_accepts(&metered, &frozen),
            "the metered record at the frozen counts"
        );
        assert!(
            record_validator_accepts(&standing, ".observations = 1000"),
            "the standing record at the frozen count"
        );
        for (key, _) in &group {
            assert!(
                !record_validator_accepts(&metered, &format!("{frozen} | del(.{key})")),
                "a metered record missing {key}"
            );
        }
        assert!(
            !record_validator_accepts(
                &metered,
                &format!("{frozen} | .workload_kind = \"sixty_four_track_console\"")
            ),
            "the standing row carrying the meter group"
        );
        for (edit, why) in [
            (".meter_window_blocks = 4", "the facility arm's window"),
            (".meter_metrics = \"all\"", "all-metric meters"),
            (
                ".meter_tap = \"post_fader\"",
                "a tap the browser does not meter",
            ),
            (".meter_snapshots -= 1", "a snapshot lost"),
            (".meter_dropped_snapshots = 1", "a dropped snapshot"),
            (
                ".bank_route_folds = 0",
                "a metered plan whose fold declined",
            ),
        ] {
            assert!(
                !record_validator_accepts(&metered, &format!("{frozen} | {edit}")),
                "{why}"
            );
        }
    }

    /// Issue #1227: the bus-and-send row, run short through the real subject, renders with no
    /// error and no forbidden operation, prints the facts it is pinned on, and the record
    /// validator pins them: refused as shortened, accepted at the frozen count, and refused at the
    /// frozen count when it names the standing fixture instead of its own.
    ///
    /// The same `SessionMeasurement` the runner's run takes, over eight timed blocks instead of a
    /// thousand (`cargo test -p bench the_bus_send_row -- --nocapture` prints the record). Its
    /// timings are not to be read.
    ///
    /// Red mutations: drop the row's `session_kind_shape` branch or its `session_kinds` entry --
    /// the frozen record is refused; point the row's `fixture_id` at the intended fixture -- the
    /// record at the frozen count is refused.
    #[test]
    fn the_bus_send_row_prints_its_facts_and_the_validator_pins_them() {
        let workload = Workload::SixtyFourTrackConsoleSends;
        let measured = SessionMeasurement::run_for(workload, SHORT_RUN_OBSERVATIONS);
        assert_eq!(measured.render_errors, 0);
        assert_eq!(
            measured.audit.total(),
            0,
            "a forbidden operation on the render path"
        );
        assert!(measured.meters.is_none());
        let record = measured.record(
            workload,
            1,
            Backend::current(),
            Metadata::gather(),
            None,
            None,
        );
        println!("{record}");
        assert!(record.contains("\"workload_kind\":\"sixty_four_track_console_sends\","));
        assert!(record.contains(
            "\"fixture_id\":\"fixtures/session/v1/console-sixty-four-track-sends.json\","
        ));
        assert!(
            !record.contains("\"bypass_pattern\":"),
            "no bypass group on the bus-and-send row"
        );
        let frozen = ".observations = 1000";
        assert!(
            !record_validator_accepts(&record, "."),
            "a shortened run must not pass for the frozen one"
        );
        assert!(
            record_validator_accepts(&record, frozen),
            "the record at the frozen count"
        );
        assert!(
            !record_validator_accepts(
                &record,
                &format!(
                    "{frozen} | .fixture_id = \"{}\"",
                    Workload::SixtyFourTrackConsole.fixture_id()
                )
            ),
            "a bus-and-send record naming the standing fixture"
        );
    }

    /// Issue #1085: the five console-strip rows, run short through the real subject, render with
    /// no forbidden operation, print the facts they are pinned on, and the record validator pins
    /// them: each row's layout in the console vocabulary, the app shape's bypass group (on that row
    /// alone), and the sparse row's input. Issue #1783: the two packing rows likewise, each pinned
    /// on the input its driver fed.
    ///
    /// The same `SessionMeasurement` the runner's run takes, over eight timed blocks instead of a
    /// thousand (`cargo test -p bench the_console_strip_rows -- --nocapture` prints the records).
    /// Their timings are not to be read, and their `observations` say they were shortened.
    ///
    /// Red mutations: print the app shape's census from a row other than the app shape, or drop
    /// `BYPASS_ROWS` -- the bypass-group assertions fail; drop `bypass_session_keys` from the
    /// validator -- the app record is refused; print the rack-token layout -- every record is
    /// refused; name the toggle row's input `tone` or drop a packing row's kind from the validator
    /// -- the packing assertions fail.
    #[test]
    fn the_console_strip_rows_print_their_facts_and_the_validator_pins_them() {
        let frozen = ".observations = 1000";
        let mut records = Vec::new();
        for workload in console_workload::CONSOLE_STRIP_WORKLOADS
            .into_iter()
            .chain(console_workload::PACKING_WORKLOADS)
        {
            let measured = SessionMeasurement::run_for(workload, SHORT_RUN_OBSERVATIONS);
            assert_eq!(measured.render_errors, 0, "{}", workload.kind());
            assert_eq!(
                measured.audit.total(),
                0,
                "{}: a forbidden operation on the render path",
                workload.kind()
            );
            assert!(measured.meters.is_none(), "{}", workload.kind());
            let record = measured.record(
                workload,
                1,
                Backend::current(),
                Metadata::gather(),
                None,
                None,
            );
            println!("{record}");
            assert!(
                record.contains(&format!("\"workload_kind\":\"{}\",", workload.kind())),
                "{}: the row names itself",
                workload.kind()
            );
            assert!(
                record.contains(&format!(
                    "\"strip_layout\":\"{}\",",
                    workload.strip_layout()
                )),
                "{}: the row states its layout",
                workload.kind()
            );
            assert!(
                !record_validator_accepts(&record, "."),
                "{}: a shortened run must not pass for the frozen one",
                workload.kind()
            );
            assert!(
                record_validator_accepts(&record, frozen),
                "{}: the record at the frozen count",
                workload.kind()
            );
            let bypass_group = record.contains("\"bypass_pattern\":");
            assert_eq!(
                bypass_group,
                workload == Workload::SixtyFourTrackAppShape,
                "{}: the bypass group is the app shape's alone",
                workload.kind()
            );
            records.push((workload, record));
        }
        let record_of = |kind: Workload| {
            records
                .iter()
                .find(|(workload, _)| *workload == kind)
                .map(|(_, record)| record.as_str())
                .expect("every console-strip row was run")
        };

        let app = record_of(Workload::SixtyFourTrackAppShape);
        assert!(
            app.contains("\"bypass_pattern\":\"index_mod_3_is_2\",\"bypassed_tracks\":21,"),
            "the app shape states the bypass its compiled session carries"
        );
        assert!(app.contains("\"strip_layout\":\"pre_insert:eq+compressor\","));
        for (edit, why) in [
            ("del(.bypass_pattern)", "an app record missing its pattern"),
            ("del(.bypassed_tracks)", "an app record missing its count"),
            (".bypassed_tracks = 20", "one bypassed track fewer"),
            (
                ".bypass_pattern = \"other\"",
                "a bypass that is not the app's",
            ),
            (
                ".strip_layout = \"dynamic:eq+compressor\"",
                "the rack-token spelling",
            ),
            (
                ".strip_layout = \"pre_insert:eq+compressor,post_insert:limiter\"",
                "the standing strip's layout",
            ),
        ] {
            assert!(
                !record_validator_accepts(app, &format!("{frozen} | {edit}")),
                "{why}"
            );
        }
        assert!(
            record_validator_accepts(
                app,
                &format!("{frozen} | .strip_layout = \"inserts:eq+compressor\"")
            ),
            "the app shape before the console migration (S0's baseline spelling)"
        );
        let sparse = record_of(Workload::SixtyFourTrackConsoleSparse);
        assert!(sparse.contains("\"input_signal\":\"odd_tracks_silent\","));
        assert!(
            !record_validator_accepts(sparse, &format!("{frozen} | .input_signal = \"tone\"")),
            "a sparse row claiming every track active"
        );
        assert!(
            !record_validator_accepts(
                sparse,
                &format!(
                    "{frozen} | . + {{bypass_pattern: \"index_mod_3_is_2\", bypassed_tracks: 21}}"
                )
            ),
            "the bypass group on another row"
        );
        // #1783: each packing row states the input its driver fed, and the validator refuses it
        // under another row's input.
        for (workload, signal, foreign) in [
            (
                Workload::SixtyFourTrackConsoleToggle,
                "odd_tracks_toggle_8_blocks",
                "tone",
            ),
            (
                Workload::SixtyFourTrackConsoleOnePerBank,
                "one_track_per_bank",
                "odd_tracks_silent",
            ),
        ] {
            let record = record_of(workload);
            assert!(
                record.contains(&format!("\"input_signal\":\"{signal}\",")),
                "{}: the row states its input",
                workload.kind()
            );
            assert!(
                !record_validator_accepts(
                    record,
                    &format!("{frozen} | .input_signal = \"{foreign}\"")
                ),
                "{}: a packing row claiming another row's input",
                workload.kind()
            );
        }
        for (workload, tracks) in [
            (Workload::TenTrackRaggedStrip, 10),
            (Workload::ThirteenTrackRaggedStrip, 13),
            (Workload::SixteenTrackStrip, 16),
        ] {
            let record = record_of(workload);
            assert!(record.contains(&format!("\"tracks\":{tracks},")));
            assert!(
                !record_validator_accepts(record, &format!("{frozen} | .tracks = 64")),
                "{}: a strip-at-N row claiming the full console",
                workload.kind()
            );
            assert!(
                !record_validator_accepts(
                    record,
                    &format!("{frozen} | .strip_layout = \"simd1:eq+compressor,simd2:limiter\"")
                ),
                "{}: the rack-token spelling",
                workload.kind()
            );
        }
    }
}
