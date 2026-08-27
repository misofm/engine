//! Issue #202 rec 2: the intended strip renders as **one bank chain per cohort**, and the
//! facilities a console attaches do not change that.
//!
//! The audit's finding was that the 64-track intended strip paid 24 planar/AoSoA round-trips a
//! block -- eight cohorts times `{builtins, simd1, simd2}` -- and that 16 of them separated stages
//! with nothing planar reading in between. `runtime::cohort_runs` now takes its merge candidates
//! from the lowered program's dataflow instead of the cohort planner's per-rack groups, so the
//! whole strip fuses: `builtins -> EQ -> compressor -> limiter`, one chain, one round-trip.
//!
//! These assertions live beside the benchmark's fixtures rather than inside the engine because the
//! *production* plan is what the rows measure, and it is assembled here: the graph compiler on its
//! own attaches no builtin banks, so the `builtins -> simd1` boundary does not exist in a plan
//! built without `prepare_session_builtins`.
//!
//! Every claim is a **count**, deliberately. A merge that silently stopped firing would render
//! byte-identical audio and only show up as a slower row, which is exactly the failure mode a
//! digest gate cannot see.

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_console_workload::{ObservationArm, PlanConfig, SessionRuntime, Workload};

/// Enough blocks for the limiter's lookahead to clear and every detector to settle.
const BLOCKS: u64 = 64;

/// Renders `blocks` blocks and returns `(digest, [chains, slots], transposes over the render)`.
fn render(workload: Workload, config: PlanConfig, blocks: u64) -> (String, [u64; 2], u64) {
    let mut runtime = SessionRuntime::build(workload, config);
    let mut digest = Sha256Sink::new();
    for block in 0..blocks {
        runtime.render(block).expect("console render");
        runtime.hash_output(&mut digest);
    }
    (
        digest.finish_hex(),
        runtime.bank_shape(),
        runtime.bank_transposes(),
    )
}

/// The eight cohorts of the 64-track fixture at the launch eight-lane width.
///
/// Read from the plan rather than written down: a four-lane host runs sixteen cohorts, and the law
/// under test is per cohort, not per eight tracks.
fn cohorts(slots: u64) -> u64 {
    // Four bank slots per cohort: the post-input builtin stage, the EQ, the compressor, the
    // limiter.
    assert_eq!(
        slots % 4,
        0,
        "the intended strip binds four slots per cohort"
    );
    slots / 4
}

/// The finding: four bank slots per cohort, one chain per cohort, one round-trip per chain.
#[test]
fn the_intended_strip_is_one_chain_per_cohort() {
    let (_, [chains, slots], transposes) = render(
        Workload::SixtyFourTrackConsole,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    assert!(slots > 0, "the intended strip must bank");
    let cohorts = cohorts(slots);
    assert_eq!(
        chains, cohorts,
        "the whole strip is one chain per cohort: builtins -> EQ -> compressor -> limiter"
    );
    assert_eq!(
        slots - chains,
        3 * cohorts,
        "three of the four slots per cohort are fused into their predecessor"
    );
    assert_eq!(
        transposes,
        BLOCKS * chains,
        "G5: one planar/AoSoA round-trip per realised chain per block"
    );
}

/// Attaching the console's facilities changes neither the chain shape nor a rendered bit.
///
/// The three arms this checks are the three that could plausibly interfere:
///
/// * **Meters.** The console leases one meter per track at `post_matrix`, which is a bindable stage
///   with an op of its own -- not one of the three elided rack boundaries -- so it is not an
///   observer of any alias a chain spans. A meter leased at `post_simd1` *would* decline the merge,
///   and the engine's `a_leased_stage_meter_declines_the_merge_and_still_meters` pins that; this
///   arm pins that the meters a console actually leases do not.
/// * **The live-console control channel.** Per effect, drained inside the slot's own stage.
/// * **Armed effect observation.** `ObservationLaneV1` reads the effect's *resident state* through
///   `observe_resident`; it never reads a planar stage buffer. So it is not a
///   `GraphNodeObserverBinding`, `runtime::chains_into` does not see it, and it must neither
///   decline the merge nor be disturbed by one -- an armed lane on a fused slot still publishes.
#[test]
fn console_facilities_do_not_change_the_chain_shape_or_the_bits() {
    let baseline = render(
        Workload::SixtyFourTrackConsole,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    for (name, config) in [
        (
            "meters",
            PlanConfig {
                meters: true,
                control: false,
                observation: ObservationArm::Absent,
            },
        ),
        (
            "control",
            PlanConfig {
                meters: false,
                control: true,
                observation: ObservationArm::Absent,
            },
        ),
        (
            "unarmed observation",
            PlanConfig {
                meters: false,
                control: true,
                observation: ObservationArm::Unarmed,
            },
        ),
        (
            "armed observation",
            PlanConfig {
                meters: false,
                control: true,
                observation: ObservationArm::Armed,
            },
        ),
    ] {
        let arm = render(Workload::SixtyFourTrackConsole, config, BLOCKS);
        assert_eq!(
            arm.1, baseline.1,
            "{name}: attaching a console facility changed the plan's [chains, slots]"
        );
        assert_eq!(
            arm.2, baseline.2,
            "{name}: attaching a console facility changed the round-trip count"
        );
        assert_eq!(
            arm.0, baseline.0,
            "{name}: attaching a console facility moved a rendered bit"
        );
    }
}

/// An armed observation lane on a fused slot still publishes -- so the arm above is not passing
/// because nothing was observed.
#[test]
fn an_armed_observation_lane_on_a_fused_chain_still_publishes() {
    let mut runtime = SessionRuntime::build(
        Workload::SixtyFourTrackConsole,
        PlanConfig {
            meters: false,
            control: true,
            observation: ObservationArm::Armed,
        },
    );
    assert!(
        runtime.observation_taps() > 0,
        "the armed arm must declare taps"
    );
    for block in 0..BLOCKS {
        runtime.render(block).expect("console render");
    }
    let [chains, slots] = runtime.bank_shape();
    assert_eq!(
        chains,
        cohorts(slots),
        "the observed strip is still one chain per cohort"
    );
    assert!(
        runtime.published_windows() > 0,
        "an armed tap on a fused chain slot must still publish windows"
    );
}

/// The rack a slot was placed in no longer changes how many round-trips its cohort pays.
///
/// Issue #181 measured the retired layout (EQ on `simd1`, compressor in `dynamic`) at two chains
/// per cohort against the intended one at one, because the cohort planner pools per rack and the
/// cross-rack pair was never proposed as a candidate. It is now, so the two are equal -- and the
/// `console_placement` bench row's paired delta is a measurement of that equality.
#[test]
fn the_two_placements_realise_the_same_chain_shape() {
    let split = render(
        Workload::SixtyFourTrackConsoleLegacy,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let merged = render(
        Workload::SixtyFourTrackEqCompSimd1,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    assert_eq!(
        split.1, merged.1,
        "the two placements must realise the same [chains, slots]"
    );
    assert_eq!(
        split.2, merged.2,
        "and therefore the same number of planar/AoSoA round-trips"
    );
    assert_eq!(split.0, merged.0, "placement must move no rendered bit");
    assert!(
        merged.1[0] < merged.1[1],
        "each cohort must still fuse more than one slot into its chain"
    );
}
