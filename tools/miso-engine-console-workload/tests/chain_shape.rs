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

use std::collections::BTreeMap;

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_console_workload::{
    ObservationArm, PlanConfig, SessionRuntime, WORKLOADS, Workload,
};
use miso_engine_effect_contract::ChannelSymmetryWitnessV1;

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

/// Bank slots one cohort of the intended strip binds, in cascade order.
///
/// Six since issue #212 banked the strip's own fader and matrix: the post-input builtin stage, the
/// EQ, the compressor, the limiter, the fader, the pan matrix. It was four before, when the fader
/// and the matrix were 128 individually dispatched per-track ops sitting *between* the cohorts'
/// chains -- which is the whole point of the count below, because banking them added sixteen slots
/// to the 64-track fixture and not one round-trip.
const SLOTS_PER_COHORT: u64 = 6;

/// The eight cohorts of the 64-track fixture at the launch eight-lane width.
///
/// Read from the plan rather than written down: a four-lane host runs sixteen cohorts, and the law
/// under test is per cohort, not per eight tracks.
fn cohorts(slots: u64) -> u64 {
    assert_eq!(
        slots % SLOTS_PER_COHORT,
        0,
        "the intended strip binds {SLOTS_PER_COHORT} slots per cohort"
    );
    slots / SLOTS_PER_COHORT
}

/// The finding: six bank slots per cohort, one chain per cohort, one round-trip per chain.
///
/// # What issue #212 had to move here, and what it deliberately did not
///
/// Banking the fader and the matrix takes the 64-track fixture from 32 slots to 48 -- two more per
/// cohort -- and leaves `chains` and `transposes` **exactly** where they were, at one per cohort
/// per block. That equality is the claim: sixteen new bank slots joined chains that already
/// existed, so the strip pays no round-trip for them, and the 128 per-track fader and matrix ops
/// that used to sit between the cohorts' chains are gone rather than relocated.
///
/// A count is the only thing that can see this. Fusing the fader in is bit-identical by
/// construction (`FaderRampStage` is one body at every width), so a digest gate would stay green
/// whether the merge fired or not; only `chains` staying at 8 while `slots` went to 48 says it did.
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
        "the whole strip is one chain per cohort: builtins -> EQ -> compressor -> limiter -> \
         fader -> matrix"
    );
    assert_eq!(
        slots - chains,
        (SLOTS_PER_COHORT - 1) * cohorts,
        "every slot but the first of each cohort is fused into its predecessor"
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

/// The ragged nine-track fixture: one full eight-lane bank plus a one-member tail, and what the
/// tail costs.
///
/// # The partial bank keeps the per-lane scalar path
///
/// `BankChain::new` takes the tiled `W`-frame transpose only when *every* lane of the chain is
/// active, because the tiled scatter fully overwrites every lane's planar buffer and holds every
/// lane's planar view at once -- neither is true of a padded bank. The nine-track fixture is the
/// only fixture in the suite that exercises that fallback, and it exercises it on every block: its
/// second cohort holds one track in an eight-lane bank.
///
/// # The tail cohort costs one chain, and that is reported rather than hidden
///
/// Issue #212 took this fixture from `[2 chains, 5 slots]` to `[3 chains, 9 slots]`. The full
/// eight-lane cohort gained the fader and matrix slots into the chain it already had, exactly as
/// the 64-track fixture did. The one-track tail could not: its effects never bank (a one-member
/// group strands), so its post-input bank's successor is a per-node EQ op and cannot be chained
/// into, while its fader and matrix banks *can* chain into each other. That is one planar/AoSoA
/// round-trip per block that the tail did not pay before, on one track.
///
/// It is the honest price of treating the tail like any other cohort rather than special-casing
/// it, it is bounded by the number of ragged tails a session has, and the `nine_track_ragged_strip`
/// bench row is where it is measured rather than argued.
#[test]
fn the_ragged_tail_banks_like_any_other_cohort_and_pays_one_chain_for_it() {
    let (digest, [chains, slots], transposes) =
        render(Workload::NineTrackRaggedStrip, PlanConfig::BASELINE, BLOCKS);
    assert_eq!(
        transposes,
        BLOCKS * chains,
        "G5: one planar/AoSoA round-trip per realised chain per block"
    );
    assert!(
        chains < slots,
        "the ragged fixture must still fuse something, or it is not testing a chain"
    );
    // Read as: the eight-lane cohort runs the whole strip as one chain of six slots; the one-track
    // tail runs its post-input bank alone and its fader and matrix banks as a pair.
    assert_eq!(
        [chains, slots],
        [3, 9],
        "one full strip chain, plus the tail's lone post-input bank and its fused fader/matrix pair"
    );
    // Bits, against the same fixture rendered with every console facility attached: a partial
    // bank's scalar transpose must be the tiled path's equal, whatever is bound around it.
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
    ] {
        let (other_digest, shape, other_transposes) =
            render(Workload::NineTrackRaggedStrip, config, BLOCKS);
        assert_eq!(
            other_digest, digest,
            "{name}: the ragged fixture's bits moved"
        );
        assert_eq!(
            shape,
            [chains, slots],
            "{name}: the ragged chain shape moved"
        );
        assert_eq!(other_transposes, transposes, "{name}: G5 moved");
    }
}

/// Issue #218: every track's route and the whole master reduction fold into the cohorts'
/// epilogues, on every standing workload.
///
/// The count is the claim. Folding renders the same bits by construction -- that is the next test
/// -- so a bind that silently stopped admitting the fold would be invisible to a digest gate and
/// would only show up as a slower row, which is the failure mode this file exists for.
///
/// One fold per *track*, not per cohort: the epilogue is per lane. The 128-track stretch row
/// therefore folds twice what the 64-track console row does, which is where the doubled absolute
/// saving on that row comes from.
#[test]
fn every_standing_workload_folds_one_route_per_track() {
    for workload in WORKLOADS {
        let mut runtime = SessionRuntime::build(workload, PlanConfig::BASELINE);
        for block in 0..4 {
            runtime.render(block).expect("console render");
        }
        // The overhead floor row is the one exception, and it is an exception by construction
        // rather than by exemption: it prepares no builtins, so it binds no bank chain, so there
        // is no epilogue for a route to fold into. Zero is the right answer here and a nonzero
        // count would mean the row had acquired a bank it is defined by not having -- which is why
        // the arm asserts the shape as well as the fold, instead of skipping the row.
        if workload == Workload::SixtyFourTrackPlumbingOnly {
            assert_eq!(
                runtime.bank_shape(),
                [0, 0],
                "the plumbing row must bind no bank chain at all"
            );
            assert_eq!(
                runtime.bank_transposes(),
                0,
                "a plan with no bank transposes nothing"
            );
            assert_eq!(
                runtime.bank_route_folds(),
                0,
                "a plan with no chain has no epilogue to fold a route into"
            );
            continue;
        }
        // The half-mono row is the second exception, and mono-collapse M1 made it one
        // deliberately. Issue #218's fold is admissible only when the folded chains' lanes, taken
        // in render order, are **exactly** the master reduction's contributor order -- a
        // floating-point sum is not associative, so the same summands in a different order are
        // different bits, and `route_fold` proves the two sequences equal rather than assuming it.
        // The reduction's order is track order. M1 pools cohorts by collapse class, and this row's
        // classes *alternate* by construction, so its chains cover `{ch00, ch02, ...}` and
        // `{ch01, ch03, ...}` and the two sequences are no longer equal. The fold declines, for
        // exactly the clause its own doc comment names: "a cohort whose planner ordered its lanes
        // differently from the edge order ... declines the whole fold".
        //
        // This is a forfeited optimisation and not a wrong render -- the row's digest is
        // byte-identical across M1 -- which is why it is asserted here as a count.
        //
        // It is also a property of the **interleaving** and not of pooling: the graph compiler's
        // `class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session` runs the same
        // 64 tracks split contiguously (32 mono then 32 stereo, the shape a real session takes)
        // and all 64 routes still fold. Asserting zero here rather than skipping the row is what
        // keeps that reading honest: a future change that quietly restored the fold on the
        // alternating row would be changing the association proof, and this would say so.
        if workload == Workload::SixtyFourTrackConsoleHalfMono {
            assert_eq!(
                runtime.bank_route_folds(),
                0,
                "the alternating row's pooled lane sets are not the reduction's order, so the \
                 fold declines"
            );
            assert_eq!(
                runtime.bank_shape(),
                [8, 48],
                "and it declines without moving a bank: the shape is the uniform rows'"
            );
            continue;
        }
        assert_eq!(
            runtime.bank_route_folds(),
            u64::from(workload.tracks()),
            "{}: every track's route must fold into its cohort's epilogue",
            workload.kind()
        );
    }
}

/// The mixed-eligibility cohort banks exactly like a uniform one -- and, since mono-collapse M1,
/// is no longer mixed.
///
/// # Why this gate exists before the thing it gates
///
/// A cohort is banked; collapse eligibility is decided per *track*. So the failure the mono work
/// has to survive is a bank whose eight lanes disagree about whether they collapse, and neither
/// uniform row can see it: `_mono` is eligible in every lane and `console` in none, so both are
/// homogeneous. `half_mono` alternates -- even tracks read one source channel, odd tracks read two
/// -- which makes every eight-lane cohort four and four.
///
/// # What M1 moved, and what it did not
///
/// M1 gave the cohort planner a third pool key beside the level and the rack: the track's
/// collapse class (`CohortPoolClassV1`). The 32 mono tracks and the 32 stereo ones now pool
/// separately, so this row's eight cohorts are **four all-mono and four all-stereo** where they
/// were eight half-and-half. That is the regroup the collapse needs and it is the whole of M1's
/// behaviour change.
///
/// It is **class A**, and the shape assertions below are how that is stated: same `[chains,
/// slots]` as both uniform rows, same planar/AoSoA round-trip count, same digest. Pooling
/// regroups lanes and never changes per-lane arithmetic (AGENTS.md), so a regroup that moved a
/// rendered bit would be a defect and not a trade-off. The one thing it *does* move is the #218
/// route fold, which `every_standing_workload_folds_one_route_per_track` states and derives.
///
/// # The per-cohort assertion is the new one, and it is the one that could not be made before
///
/// The census (`[eligible lanes, lanes]`) is a pair of totals and cannot tell four-and-four from
/// eight-and-zero: both give 32 eligible lanes over 64. `unit_eligibility` is per unit, so the
/// difference between "every cohort is half eligible" and "half the cohorts are wholly eligible"
/// is visible -- and that difference is precisely what M2's dispatch will read.
#[test]
fn the_half_mono_cohort_banks_like_a_uniform_one() {
    let mono = render(
        Workload::SixtyFourTrackConsoleMono,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let half = render(
        Workload::SixtyFourTrackConsoleHalfMono,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let console = render(
        Workload::SixtyFourTrackConsole,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    assert_eq!(
        half.1, mono.1,
        "a mixed-eligibility cohort must realise the uniform mono shape"
    );
    assert_eq!(
        half.1, console.1,
        "and the standing stereo shape: eligibility moves no bank today"
    );
    assert_eq!(
        half.2, mono.2,
        "a mixed cohort must pay the uniform count of round-trips"
    );
    assert_eq!(half.2, console.2);
    assert_eq!(half.1[0], cohorts(half.1[1]));
    // The derivation actually fired. Half the tracks read two source channels again, so the
    // structural witness declines them -- and they render different bits from the uniform mono
    // session, which is what makes this a mixed cohort rather than a relabelled one.
    let counted = |workload| {
        let runtime = SessionRuntime::build(workload, PlanConfig::BASELINE);
        (
            runtime.structural_mono_tracks(),
            runtime.symmetry_counters(),
        )
    };
    assert_eq!(
        counted(Workload::SixtyFourTrackConsoleMono),
        (64, [64, 129]),
        "every track of the mono fixture is collapse-eligible"
    );
    // The half-mono row's own census, derived rather than pinned -- and it is the standing worked
    // example of why `symmetry_counters` is monotone evidence and **not** a pool sizing number.
    //
    // The uniform mono row counts 129 "lanes": 64 bank-chain lanes, 64 source-input ops and the
    // master, its 64 route ops having been absorbed by the #218 fold and never built as units at
    // all. The half-mono row's fold declines (see
    // `every_standing_workload_folds_one_route_per_track` for the association-order derivation),
    // so its 64 route ops *are* dispatched: 193 lanes, not 129. And a `Route` reports
    // `ChannelSymmetryWitnessV1::SYMMETRIC` -- it is not per-track upstream work, so nothing about
    // it can make two channels disagree -- which adds 64 to the eligible half as well.
    //
    // So the pair moves from `[64, 129]` to `[128, 193]` on a row where **not one track's
    // symmetry changed**: 32 of its tracks are collapse-eligible before and after, which is what
    // `structural_mono_tracks` still reports and what the per-cohort rows below actually measure.
    // Two censuses are comparable only when the plans' unit inventories are; this is that caveat
    // with numbers on it.
    assert_eq!(
        counted(Workload::SixtyFourTrackConsoleHalfMono),
        (32, [64 + 64, 129 + 64]),
        "half the tracks of the half-mono row read two source channels; the census additionally \
         carries the 64 route ops this row's declined fold left dispatched"
    );
    assert_eq!(
        counted(Workload::SixtyFourTrackConsole).0,
        0,
        "no track of the standing fixture reads one source channel"
    );
    assert_ne!(
        half.0, mono.0,
        "the half-mono row must render different bits from the uniform mono row, or its odd \
         tracks are not reading a second source channel at all"
    );

    // Mono-collapse M1: the cohorts are pooled by class, so no bank chain is mixed any more.
    //
    // Read as: of the row's `chains` upstream-of-seam bank chains, half are wholly eligible and
    // half are wholly ineligible, and every chain is one or the other. Before M1 every one of them
    // was four-and-four, which the census below cannot distinguish from this.
    //
    // Red mutation: make `SessionPoolClassesV1::class_of` return `CohortPoolClassV1::Stereo`
    // unconditionally -- the pre-M1 behaviour, one pool per (level, rack) -- and every chain goes
    // back to holding four even tracks and four odd ones: the per-chain homogeneity assertion
    // fails and `collapsible` falls to zero, while every shape and digest assertion above stays
    // green. That is the shape of failure this test exists for, and the reason it is a *count*.
    let runtime = SessionRuntime::build(
        Workload::SixtyFourTrackConsoleHalfMono,
        PlanConfig::BASELINE,
    );
    let rows = runtime.unit_eligibility();
    let cohorts_in_plan = half.1[0];
    let banked: Vec<_> = rows.iter().filter(|row| row.banked).collect();
    assert_eq!(
        banked.len() as u64,
        cohorts_in_plan,
        "one row per bank chain"
    );
    // Every chain of this row spans the whole strip, so none of them is seam-side only. The check
    // is not decoration: a fader-only chain's witness is unconditionally symmetric
    // (`SEAM_SIDE_WITNESS`), and counting one as "wholly eligible" would be counting an
    // unconditional `true`.
    assert!(
        banked.iter().all(|row| !row.witness_is_vacuous()),
        "every chain here renders upstream-of-seam stages, so no row's witness is vacuous"
    );
    // The runtime half alone cannot see the split, and that is not a defect: `SOURCE` is not one
    // of its four terms (`session_structural_symmetry_v1` says why it lives on the control plane),
    // and this row's designed words are symmetric on *every* track -- the half-mono derivation
    // moves the source mapping and nothing else. So every chain reports every lane eligible here,
    // exactly as the uniform mono row does.
    assert!(
        banked.iter().all(|row| row.all_lanes_eligible()),
        "the runtime witness is source agnostic, so it admits every lane of this row"
    );

    // The join is what answers the question. The structural half is keyed by **track id** and the
    // runtime half by anonymous **lanes**; `lane_tracks` is the only relation between the two
    // keys, and a collapse decision is their conjunction. Performing it here is what proves the
    // plumbing M2's dispatch needs is actually connected end to end.
    let structural: BTreeMap<&str, ChannelSymmetryWitnessV1> = runtime
        .structural_symmetry()
        .iter()
        .map(|(track, witness)| (track.as_ref(), *witness))
        .collect();
    let mut collapsible = 0_u64;
    for row in &banked {
        let lanes: Vec<ChannelSymmetryWitnessV1> = row
            .lane_tracks
            .iter()
            .map(|track| {
                structural
                    .get(track.as_ref())
                    .copied()
                    .expect("every bank lane names a track of the session")
            })
            .collect();
        assert_eq!(lanes.len() as u32, row.lanes(), "one track per active lane");
        let eligible = lanes.iter().filter(|witness| witness.eligible()).count();
        // The M1 claim, per cohort: no chain is mixed any more. Before M1 every one of them was
        // four-and-four, and the census cannot tell that apart from this at all.
        assert!(
            eligible == 0 || eligible == lanes.len(),
            "a chain mixed the two pool classes: {:?}",
            row.lane_tracks
        );
        if eligible == lanes.len() && row.all_lanes_eligible() {
            collapsible += 1;
        }
    }
    assert_eq!(
        collapsible,
        cohorts_in_plan / 2,
        "half this row's cohorts are wholly collapse-eligible once both halves are conjoined"
    );
    // And the rows are the census, so the two surfaces cannot drift apart.
    let folded = rows.iter().fold([0_u64, 0], |mut total, row| {
        total[0] += u64::from(row.eligible_lanes());
        total[1] += u64::from(row.lanes());
        total
    });
    assert_eq!(
        folded,
        runtime.symmetry_counters(),
        "the per-unit rows must sum to the census"
    );
}

/// A seam-side-only chain's witness is *vacuous*, and the plan surface says so.
///
/// # The trap this closes
///
/// `SEAM_SIDE_WITNESS` is an unconditional `ChannelSymmetryWitnessV1::SYMMETRIC`: the collapse
/// duplicates its one computed plane *into* the fader and the matrix, so their per-channel words
/// are free to differ and are deliberately never compared. A unit built only from those two stages
/// therefore reports every lane eligible on **every** session, mono or stereo. A dispatch that
/// read that as "this cohort may collapse" would be reading a constant `true` as evidence, and no
/// digest could see the mistake -- which is why the classification is a queryable field rather
/// than a note in a doc comment.
///
/// The nine-track ragged fixture is where such a unit actually exists. Its one-track tail cannot
/// bank its effects (a one-member group strands), so its post-input bank has no successor to chain
/// into, while its fader and matrix banks chain into each other: one unit, two stages, both
/// seam-side. `the_ragged_tail_banks_like_any_other_cohort_and_pays_one_chain_for_it` is where
/// that shape is derived; this is what it means for the collapse.
///
/// Red mutation: make `runtime::upstream_of_seam` return `true` for `PostFader`/`PostMatrix` ->
/// no row is vacuous and the first assertion fails. Make it return `false` for
/// `GraphNodeId::Effect` -> the full strip's chains lose two thirds of their upstream stages and
/// the last assertion fails.
#[test]
fn a_seam_side_only_chain_reads_as_vacuous_rather_than_eligible() {
    let ragged = SessionRuntime::build(Workload::NineTrackRaggedStrip, PlanConfig::BASELINE);
    let rows = ragged.unit_eligibility();
    let vacuous: Vec<_> = rows
        .iter()
        .filter(|row| row.banked && row.witness_is_vacuous())
        .collect();
    assert_eq!(
        vacuous.len(),
        1,
        "the ragged tail's fused fader/matrix pair is the one seam-side-only chain"
    );
    let seam = vacuous[0];
    assert_eq!(seam.stages, 2, "the fader and the matrix, fused");
    assert_eq!(seam.upstream_of_seam_stages, 0);
    assert!(
        seam.all_lanes_eligible(),
        "and it claims eligibility unconditionally, which is exactly why the flag is needed"
    );

    // The other side of the pair: the full 64-track strip's chains span both sides of the seam,
    // four upstream stages (post-input builtins, EQ, compressor, limiter) and two seam-side ones,
    // so none of them is vacuous.
    let strip = SessionRuntime::build(Workload::SixtyFourTrackConsole, PlanConfig::BASELINE);
    for row in strip.unit_eligibility().iter().filter(|row| row.banked) {
        assert!(!row.witness_is_vacuous());
        assert_eq!(row.stages, SLOTS_PER_COHORT as u32);
        assert_eq!(
            row.upstream_of_seam_stages,
            SLOTS_PER_COHORT as u32 - 2,
            "the fader and the matrix are the two seam-side slots of the strip"
        );
    }
}

/// The mono row-pair is one session today, to the bit.
///
/// The bench asserts this in-run before it emits its `console_mono` record; asserting it here too
/// is what keeps it a *gate* rather than a benchmark-only claim, because the bench is a one-shot
/// measurement runner and this file is swept.
#[test]
fn the_mono_row_pair_renders_one_session() {
    let eligible = render(
        Workload::SixtyFourTrackConsoleMono,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let forced = render(
        Workload::SixtyFourTrackConsoleMonoDual,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    assert_eq!(
        eligible.0, forced.0,
        "the mono row-pair must render identical output"
    );
    assert_eq!(eligible.1, forced.1, "and realise an identical bank shape");
    assert_eq!(
        eligible.2, forced.2,
        "and pay an identical round-trip count"
    );
}

/// The overhead floor row prepares nothing, and what it renders is the plumbing alone.
///
/// Three claims, and the third is the one that makes the row a *floor* rather than another
/// decomposition row: `plumbing_only` binds no bank, so it pays neither the D7 sanitise and
/// boundary passes nor the fader and matrix kernels that `gain_pan_only` and `dispatch_only` both
/// pay -- and it therefore renders different bits from both, which is how a reader knows the two
/// rows are not measuring the same plan under two names.
#[test]
fn the_plumbing_row_binds_no_strip_at_all() {
    let plumbing = render(
        Workload::SixtyFourTrackPlumbingOnly,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let identity = render(
        Workload::SixtyFourTrackDispatchOnly,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    let gain_pan = render(
        Workload::SixtyFourTrackGainPanOnly,
        PlanConfig::BASELINE,
        BLOCKS,
    );
    assert_eq!(
        plumbing.1,
        [0, 0],
        "the plumbing row must bind no bank slot"
    );
    assert_eq!(plumbing.2, 0, "and therefore no planar/AoSoA round-trip");
    assert_ne!(
        plumbing.0, identity.0,
        "the plumbing row must render different bits from the identity row, or the strip it \
         claims not to prepare is being prepared"
    );
    // The identity pair, from the other side: `gain_pan_only` and `dispatch_only` bank the same
    // slots and pay the same round-trips -- the floor table costs them at one inventory for that
    // reason -- and render different bits, because their fader and matrix carry different
    // constants.
    assert_eq!(
        gain_pan.1, identity.1,
        "the two identity-section rows must realise the same bank shape"
    );
    assert_eq!(gain_pan.2, identity.2);
    assert_ne!(
        gain_pan.0, identity.0,
        "a real fader and pan must not render what a 0 dB fader and a hard-identity pan render"
    );
}

/// The folded master carries the reduction's own bits, and the meter arm is the oracle that says
/// so.
///
/// # Why the meter arm is a legitimate oracle
///
/// The console leases one meter per track at `post_matrix`, and `post_matrix` is the chain's last
/// slot. An observer there reads a planar buffer the fold stops writing, so `route_fold` declines
/// the whole plan -- which leaves the Job-2 shape standing: a route op per track, then the D9
/// `sum2_block`/`sum_into_block` reduction over their buffers. The two arms therefore differ in
/// *exactly* the thing under test and in a meter that AGENTS.md requires not to change signal
/// flow.
///
/// # What this catches that nothing else does
///
/// The association order. A floating-point sum is not associative, so accumulating the same 64
/// contributions in a different order is a different number; `route_fold` proves the epilogues'
/// order is the reduction's order on the lowered program, and this is the end-to-end check of that
/// proof on a session with eight cohorts whose partial sums genuinely differ.
///
/// Red mutation: build `RouteFold::runs` from the candidate list reversed (leaving the association
/// proof reading the forward order, so the plan still folds) -- every 64-track row's digest
/// diverges from its meter arm at the first block.
#[test]
fn the_folded_master_is_the_reductions_own_bits() {
    const METERED: PlanConfig = PlanConfig {
        meters: true,
        control: false,
        observation: ObservationArm::Absent,
    };
    for workload in WORKLOADS {
        // The oracle is a meter lease, and a meter stream is leased from the prepared builtins
        // session that the overhead floor row deliberately does not have. There is nothing to
        // check on that row anyway: it folds no route, so the fold's association order is not a
        // property it has. `every_standing_workload_folds_one_route_per_track` pins what it does
        // have.
        if workload == Workload::SixtyFourTrackPlumbingOnly {
            continue;
        }
        let folded = render(workload, PlanConfig::BASELINE, BLOCKS);
        let mut metered_runtime = SessionRuntime::build(workload, METERED);
        let mut metered = Sha256Sink::new();
        for block in 0..BLOCKS {
            metered_runtime.render(block).expect("console render");
            metered_runtime.hash_output(&mut metered);
        }
        assert_eq!(
            metered_runtime.bank_route_folds(),
            0,
            "{}: a meter on the matrix must decline the fold, or this is not an oracle",
            workload.kind()
        );
        assert_eq!(
            folded.0,
            metered.finish_hex(),
            "{}: the folded master is not the reduction's bits",
            workload.kind()
        );
    }
}

/// The collapse fires on every cohort of the mono row, on half the half-mono row's, and on nothing
/// else -- and the arms of the mono pair differ by the switch and not by a fixture edit.
///
/// # Why a count, and why this count
///
/// A collapsed block renders the bits a dual block renders. That is the whole claim, and it means
/// no digest, no output comparison and no shape number can see whether the collapse fired at all:
/// a dispatch that silently stopped engaging would leave every other assertion in this file green
/// and only show up as a slower row. `bank_collapse_counters` is the only honest statement, exactly
/// as `bank_route_folds` and `bank_scatter_redirects` are for the two optimisations before it.
///
/// The pair is `[collapsed blocks, collapsible cohorts]`. The second is fixed at bind and is what
/// "eight of eight cohorts" is read off; the first is per block, so `BLOCKS` blocks with every
/// cohort collapsing is `BLOCKS * cohorts` and nothing else -- which is what makes "it fired on
/// **every** block" a checkable statement rather than "it fired at least once".
///
/// # The three rows, and what each one is here to catch
///
/// * `_mono` -- every track carries a mono source mapping and symmetric designed words, so every
///   cohort collapses on every block. A dispatch that never engages fails here.
/// * `_mono_dual` -- the *same* fixture with the collapse forced off. Its cohorts are still
///   collapsible (the count is bind-time), and not one block took it. A force-off switch that did
///   not actually reach the chains fails here, and that is the switch the paired measurement's
///   second arm is.
/// * `half_mono` -- half the tracks read two source channels. Since M1 they pool into four all-mono
///   cohorts and four all-stereo ones, so exactly half the cohorts are collapsible. A dispatch that
///   dropped the **structural** join -- the `SOURCE` term, which the chain's own witness cannot see
///   (`PlanUnitEligibilityV1::lane_eligible` says why) -- would report eight collapsible cohorts
///   here, and `the_half_mono_cohort_banks_like_a_uniform_one`'s `assert_ne!` would then fail
///   because the odd tracks' right channels would be the duplicated left ones. That is the pair of
///   failures the join has to be checked by, and it is checked by both.
/// * `console` and every non-mono row -- no track reads one source channel, and the designed words
///   differ per channel besides, so nothing is collapsible and nothing collapses. A dispatch that
///   dropped `all_lanes_symmetric` would collapse this row and move its digest away from its seal.
#[test]
fn the_collapse_fires_on_every_mono_cohort_and_no_other() {
    let counted = |workload| {
        let mut runtime = SessionRuntime::build(workload, PlanConfig::BASELINE);
        for block in 0..BLOCKS {
            runtime.render(block).expect("console render");
        }
        (runtime.bank_collapse_counters(), runtime.bank_shape())
    };

    let (mono, mono_shape) = counted(Workload::SixtyFourTrackConsoleMono);
    let cohorts_in_plan = cohorts(mono_shape[1]);
    assert_eq!(
        mono,
        [BLOCKS * cohorts_in_plan, cohorts_in_plan],
        "every cohort of the mono row must collapse on every block"
    );

    let (dual, dual_shape) = counted(Workload::SixtyFourTrackConsoleMonoDual);
    assert_eq!(
        dual_shape, mono_shape,
        "the two arms of the pair are one fixture and must realise one shape"
    );
    assert_eq!(
        dual,
        [0, cohorts_in_plan],
        "the forced-off arm must render the same collapsible cohorts and take none of them"
    );

    let (half, half_shape) = counted(Workload::SixtyFourTrackConsoleHalfMono);
    assert_eq!(half_shape, mono_shape);
    assert_eq!(
        half,
        [BLOCKS * cohorts_in_plan / 2, cohorts_in_plan / 2],
        "half the half-mono row's cohorts are collapsible, and each of those collapses every block"
    );

    for workload in WORKLOADS {
        if matches!(
            workload,
            Workload::SixtyFourTrackConsoleMono
                | Workload::SixtyFourTrackConsoleMonoDual
                | Workload::SixtyFourTrackConsoleHalfMono
        ) {
            continue;
        }
        assert_eq!(
            counted(workload).0,
            [0, 0],
            "{}: no track of a stereo-source fixture may collapse",
            workload.kind()
        );
    }
}

/// The transition oracle: a run that stops collapsing mid-session is a run that never collapsed.
///
/// # What this is the only test of
///
/// A collapsed block evolves **one** channel's state. The right channel's rings, ramps, cursors and
/// recursive words stand still, and the block that stops collapsing has to put them back before the
/// first dual block reads them. `PreparedNativeEffectBank::desymmetrize_channels` is that copy, and
/// its correctness is a property of the *list of words* each effect copies -- a list no digest of a
/// wholly collapsed run and no digest of a wholly dual run can see, because neither ever crosses
/// the boundary.
///
/// This crosses it. Both arms render the same fixture for the same number of blocks and differ only
/// in that one of them collapsed for the first half. If the copy is complete they agree to the bit
/// from the transition block onward -- which, since they also agreed before it, means one digest.
///
/// # The red mutation, and it is per field
///
/// Delete any one line of any `copy_state_from` -- the compressor's `cursor`, the limiter's
/// `box_sum`, the EQ's `identity`, the input chain's second integrator -- and this test fails while
/// every other test in the tree stays green. That is the whole reason it exists: a partial copy is
/// invisible to a run that never disengages, and every other gate here is such a run.
#[test]
fn a_run_that_stops_collapsing_renders_what_a_never_collapsed_run_renders() {
    // Long enough that the limiter's lookahead line and the compressor's detector ring are full of
    // collapsed-run samples before the transition, so a stale ring is a difference the delay lines
    // carry out rather than one the transition block hides.
    const SWITCH: u64 = BLOCKS / 2;

    let mut mixed =
        SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, PlanConfig::BASELINE);
    let mut never =
        SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, PlanConfig::BASELINE);
    never.force_mono_collapse_off(true);

    let mut mixed_digest = Sha256Sink::new();
    let mut never_digest = Sha256Sink::new();
    for block in 0..BLOCKS {
        if block == SWITCH {
            mixed.force_mono_collapse_off(true);
        }
        mixed.render(block).expect("console render");
        never.render(block).expect("console render");
        mixed.hash_output(&mut mixed_digest);
        never.hash_output(&mut never_digest);
    }

    let collapsed = mixed.bank_collapse_counters();
    let cohorts_in_plan = cohorts(mixed.bank_shape()[1]);
    assert_eq!(
        collapsed,
        [SWITCH * cohorts_in_plan, cohorts_in_plan],
        "the mixed arm must have collapsed for exactly the blocks before the transition"
    );
    assert_eq!(
        never.bank_collapse_counters(),
        [0, cohorts_in_plan],
        "the reference arm must never have collapsed"
    );
    assert_eq!(
        mixed_digest.finish_hex(),
        never_digest.finish_hex(),
        "a session that stopped collapsing mid-render must be bit-identical to one that never did"
    );
}

/// A chain that has disengaged does not collapse again in this plan.
///
/// M2 owns the engage direction and the disengage direction; **re-engage** is M3's, and this states
/// that as a checkable property rather than as a note. Re-engaging would have to argue that the
/// dual blocks in between left the two channels agreeing again, which is a statement about *why*
/// the witness went false and not about the witness itself -- so until that argument exists, a
/// chain that has stopped stays stopped. Declining is always safe.
#[test]
fn a_disengaged_chain_stays_dual_even_when_the_switch_comes_back() {
    let mut runtime =
        SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, PlanConfig::BASELINE);
    let cohorts_in_plan = cohorts(runtime.bank_shape()[1]);
    for block in 0..4 {
        runtime.render(block).expect("console render");
    }
    runtime.force_mono_collapse_off(true);
    for block in 4..8 {
        runtime.render(block).expect("console render");
    }
    runtime.force_mono_collapse_off(false);
    for block in 8..12 {
        runtime.render(block).expect("console render");
    }
    assert_eq!(
        runtime.bank_collapse_counters(),
        [4 * cohorts_in_plan, cohorts_in_plan],
        "only the four blocks before the disengage collapsed"
    );
}

/// A live one-channel retarget disengages the collapse on the block it lands, not the block after.
///
/// # The ordering this is really about
///
/// Two rules meet on one block boundary and they have to be observed in one order.
///
/// * An admitted record takes effect on the **first sample of the block that drains it** (#137 E1,
///   and every console gate in the tree rests on it).
/// * A `ParameterChannel::Left` retarget clears the channel-symmetry witness' `LIVE` term, which is
///   what tells the collapse to stop.
///
/// If the dispatch read the witness before the drain, the record's *audio* would land on a block
/// that still ran collapsed -- and a collapsed block publishes the left plane on both channels, so
/// a retarget addressed to the left channel would silently reach the right one too. That is a
/// wrong-audio bug on exactly one block, in the direction nobody would look. `BankChain::run`
/// therefore drains every slot first (`BankStage::begin_block`), then reads the witness.
///
/// # Why this is also the strongest copy-list oracle in the tree
///
/// The transition here happens with **ramps in flight**: the retarget opens a smoothing window on
/// the left channel at the same block the chain stops collapsing. So the disengage copy has to
/// carry not just the rings but the ramp state and the coefficient words derived from it, and the
/// blocks after it render a moving coefficient on one channel and a settled one on the other.
///
/// Red mutations: swap the drain and the dispatch in `BankChain::run` (the ordering above); drop
/// any ramp or coefficient entry from any effect's `copy_state_from`.
#[test]
fn a_live_one_channel_retarget_disengages_on_the_block_it_lands() {
    const CONTROL: PlanConfig = PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Absent,
    };
    const SWITCH: u64 = BLOCKS / 2;

    let mut collapsing = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, CONTROL);
    let mut never = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, CONTROL);
    never.force_mono_collapse_off(true);

    // One retarget per strip slot, all on the same track and all on the **left** channel only.
    //
    // Three, not one, because each slot's disengage copy has a different list and only its own
    // parameter can move it: the compressor's threshold opens its ramps and rewrites its
    // coefficient words, the EQ's band gain opens the cascade's per-lane ramp countdown, and the
    // limiter's ceiling both opens its two linear ramps and drops the ceiling far enough that the
    // detector's twelve oversampling taps actually decide an output sample -- which is what makes
    // the limiter's `history` a word the copy has to carry rather than one no fixture reads.
    let retargets: Vec<(usize, u32, f32)> = [
        ("miso.compressor", 0_u32, -24.0_f32),
        ("miso.parametric-eq", 3, 6.0),
        ("miso.true-peak-limiter", 0, -20.0),
    ]
    .into_iter()
    .map(|(effect, parameter, value)| {
        let channel = collapsing
            .first_track_control_channel(effect)
            .unwrap_or_else(|| panic!("the mono fixture carries a {effect} on every track"));
        assert_eq!(
            never.first_track_control_channel(effect),
            Some(channel),
            "both arms must address the same track, or they are two sessions"
        );
        (channel, parameter, value)
    })
    .collect();
    let track = collapsing.control_identity(retargets[0].0).0.to_owned();
    for (channel, _, _) in &retargets {
        assert_eq!(
            collapsing.control_identity(*channel).0,
            track,
            "every retarget must address one track, so exactly one cohort disengages"
        );
    }

    let mut collapsing_digest = Sha256Sink::new();
    let mut never_digest = Sha256Sink::new();
    for block in 0..BLOCKS {
        if block == SWITCH {
            for runtime in [&mut collapsing, &mut never] {
                for (channel, parameter, value) in &retargets {
                    assert!(
                        runtime.push_parameter(
                            *channel,
                            *parameter,
                            miso_engine_effect_contract::ParameterChannel::Left,
                            *value,
                        ),
                        "the bounded control queue must have room"
                    );
                }
            }
        }
        collapsing.render(block).expect("console render");
        never.render(block).expect("console render");
        collapsing.hash_output(&mut collapsing_digest);
        never.hash_output(&mut never_digest);
    }

    let cohorts_in_plan = cohorts(collapsing.bank_shape()[1]);
    let collapsed = collapsing.bank_collapse_counters();
    assert_eq!(
        collapsed[1], cohorts_in_plan,
        "every cohort of the mono fixture is collapsible"
    );
    // One cohort holds the retargeted track and stops on the block the record lands; the other
    // seven never see a record and collapse throughout. If the dispatch had read the witness
    // before the drain, the retargeted cohort would have collapsed on the landing block too, and
    // this count would be one cohort-block higher.
    assert_eq!(
        collapsed[0],
        BLOCKS * cohorts_in_plan - (BLOCKS - SWITCH),
        "the retargeted cohort must stop collapsing on the block its record lands"
    );
    assert_eq!(
        collapsing_digest.finish_hex(),
        never_digest.finish_hex(),
        "a session that disengaged on a live retarget must render what a never-collapsed one does"
    );
}

/// A collapsed cohort's right-channel observation taps read what a dual run's would.
///
/// # The one contract surface a digest gate cannot cover
///
/// The collapse's whole claim is that the *audio* is identical, so every digest in this file is
/// blind to what it does to state. A resident observation tap reads state directly --
/// `observe_resident_bank` extracts the compressor's `gain_reduction_db` and the limiter's
/// `reduction` per lane, per channel -- and a collapsed bank's right channel is frozen at the value
/// it held when the collapse engaged. A tap that published that frozen word would be publishing a
/// number no dual run ever produces, on a session whose audio is bit-perfect.
///
/// So the seam applies to taps too: a collapsed block publishes the left channel's reading for the
/// right channel's tap. That is the same induction the fader duplication and the disengage copy
/// rest on -- the right channel of a collapsed track *is* its left channel -- and this is the gate
/// on it.
///
/// Red mutation: delete the `sample.right = sample.left` loop in `ConsoleEffectBankStage::process_
/// mono`. Every digest assertion in this file stays green and the two arms' tap readings diverge on
/// the first window a compressor actually reduces.
#[test]
fn a_collapsed_cohorts_right_channel_taps_read_what_a_dual_runs_do() {
    const OBSERVED: PlanConfig = PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Armed,
    };
    let mut collapsed = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, OBSERVED);
    let mut dual = SessionRuntime::build(Workload::SixtyFourTrackConsoleMonoDual, OBSERVED);
    assert!(
        collapsed.observation_taps() > 0,
        "the armed arm must declare taps"
    );

    let mut collapsed_digest = Sha256Sink::new();
    let mut dual_digest = Sha256Sink::new();
    for block in 0..BLOCKS {
        collapsed.render(block).expect("console render");
        dual.render(block).expect("console render");
        collapsed.hash_output(&mut collapsed_digest);
        dual.hash_output(&mut dual_digest);
    }

    assert!(
        collapsed.bank_collapse_counters()[0] > 0,
        "the observed arm must actually have collapsed, or this proves nothing"
    );
    assert_eq!(
        dual.bank_collapse_counters()[0],
        0,
        "the reference arm must not have collapsed"
    );
    assert_eq!(
        collapsed_digest.finish_hex(),
        dual_digest.finish_hex(),
        "observing a collapsed session must not move a rendered bit"
    );

    let observed = collapsed.observation_readings();
    let reference = dual.observation_readings();
    assert_eq!(
        observed.len(),
        reference.len(),
        "both arms declare the same taps"
    );
    // The readings have to *mean* something, or agreeing about them is agreeing about nothing: at
    // least one window must carry a non-zero reduction on both channels.
    assert!(
        reference
            .iter()
            .flatten()
            .any(|(_, left, right)| *left != 0.0 && *right != 0.0),
        "the corpus must drive at least one tap away from its rest value on both channels"
    );
    for (index, (observed, reference)) in observed.iter().zip(reference.iter()).enumerate() {
        assert_eq!(
            observed.map(|(sequence, left, right)| (sequence, left.to_bits(), right.to_bits())),
            reference.map(|(sequence, left, right)| (sequence, left.to_bits(), right.to_bits())),
            "tap {index}: a collapsed cohort's published window must be the dual run's, both channels"
        );
    }
}

/// A run that *starts* collapsing mid-session is a run that always collapsed.
///
/// The disengage direction has a copy behind it; the **engage** direction has only an argument, and
/// this is the gate on that argument. Engaging is sound exactly when the right channel's state
/// already equals the left channel's at that block, and it does: the two channels of a
/// collapse-eligible track are fed one source channel, read designed words that compare bit-equal,
/// and have therefore evolved through the same values since the plan was built. Nothing copies
/// anything here, so if that induction were wrong the two arms would diverge from the block the
/// switch flips.
///
/// Red mutation: make `render_mono`, `process_block_mono` or `input_chain_block_mono` read any word
/// the dual body reads from the *right* channel — the state that has been evolving in parallel and
/// that a collapsed block stops touching.
#[test]
fn a_run_that_starts_collapsing_renders_what_an_always_collapsed_run_renders() {
    const SWITCH: u64 = BLOCKS / 2;

    let mut late = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, PlanConfig::BASELINE);
    let mut always =
        SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, PlanConfig::BASELINE);
    late.force_mono_collapse_off(true);

    let mut late_digest = Sha256Sink::new();
    let mut always_digest = Sha256Sink::new();
    for block in 0..BLOCKS {
        if block == SWITCH {
            late.force_mono_collapse_off(false);
        }
        late.render(block).expect("console render");
        always.render(block).expect("console render");
        late.hash_output(&mut late_digest);
        always.hash_output(&mut always_digest);
    }

    let cohorts_in_plan = cohorts(late.bank_shape()[1]);
    assert_eq!(
        late.bank_collapse_counters(),
        [(BLOCKS - SWITCH) * cohorts_in_plan, cohorts_in_plan],
        "the late arm must have collapsed only from the switch onward"
    );
    assert_eq!(
        always.bank_collapse_counters(),
        [BLOCKS * cohorts_in_plan, cohorts_in_plan],
        "the reference arm must have collapsed throughout"
    );
    assert_eq!(
        late_digest.finish_hex(),
        always_digest.finish_hex(),
        "a session that started collapsing mid-render must be bit-identical to one that always did"
    );
}

/// Engaging a live bypass after a collapsed run renders what a never-collapsed run renders.
///
/// # The seam this is the gate on, and why the other transition oracles miss it
///
/// A bypassed lane does not emit silence: it emits its own **dry** input, delayed by the slot's
/// declared latency so that switching bypass on and off does not move the signal in time. That
/// delay is a `BypassShunt` line that persists across blocks, and it is fed on *every* block --
/// bypassed or not -- precisely so that the first bypassed block emits correctly delayed audio
/// instead of `latency` samples of stale zeros.
///
/// So the shunt's line carries state out of the collapsed window, and it is the one piece of
/// per-block state the seam cannot repair: the seam duplicates the resident *plane* after the slot
/// has run, while the line was written before it. A collapsed block that handed the line its
/// ungathered right plane would poison it for `latency` samples, and the poison would surface only
/// on a block that is both bypassed and dual -- which is exactly the block a live `Bypass(true)`
/// produces, because that record clears the witness' `UNBYPASSED` term on the same boundary it
/// takes effect on. `a_run_that_stops_collapsing_...` cannot see it (it never bypasses) and
/// `a_live_one_channel_retarget_...` cannot see it (a parameter write reads no dry line).
///
/// # Why all three effects, and why the EQ is in the list
///
/// The failure is proportional to declared latency, so the three slots of the strip fail for
/// different lengths and the EQ, at zero latency, does not fail at all. Running all three is what
/// distinguishes "the collapse is wrong about dry signal" from "one effect is wrong": with the
/// defect present the limiter diverges for its lookahead and the compressor for its own, while the
/// EQ stays green. A single-effect test would have read as an effect bug.
///
/// Red mutation: `ConsoleEffectBankStage::process_inner::<true>` captures `block.right` -- the
/// ungathered resident scratch -- instead of `block.left`. The limiter and compressor arms fail on
/// the blocks following the bypass; the EQ arm stays green, which is the shape that names the
/// cause.
#[test]
fn a_bypass_engaged_after_a_collapsed_run_renders_the_dual_bits() {
    const CONTROL: PlanConfig = PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Absent,
    };
    const SWITCH: u64 = BLOCKS / 2;

    for effect in [
        "miso.true-peak-limiter",
        "miso.compressor",
        "miso.parametric-eq",
    ] {
        let mut collapsing = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, CONTROL);
        let mut never = SessionRuntime::build(Workload::SixtyFourTrackConsoleMono, CONTROL);
        never.force_mono_collapse_off(true);

        let channel = collapsing
            .first_track_control_channel(effect)
            .unwrap_or_else(|| panic!("the mono fixture carries a {effect} on every track"));
        assert_eq!(
            never.first_track_control_channel(effect),
            Some(channel),
            "both arms must address the same track, or they are two sessions"
        );

        // Block by block rather than one digest over the run: the divergence is a burst the length
        // of the slot's latency, and which blocks it lands on is what names the cause.
        let mut diverged: Vec<u64> = Vec::new();
        for block in 0..BLOCKS {
            if block == SWITCH {
                for runtime in [&mut collapsing, &mut never] {
                    assert!(
                        runtime.push_bypass(channel, true),
                        "the bounded control queue must have room"
                    );
                }
            }
            collapsing.render(block).expect("console render");
            never.render(block).expect("console render");
            let mut collapsing_digest = Sha256Sink::new();
            let mut never_digest = Sha256Sink::new();
            collapsing.hash_output(&mut collapsing_digest);
            never.hash_output(&mut never_digest);
            if collapsing_digest.finish_hex() != never_digest.finish_hex() {
                diverged.push(block);
            }
        }

        let collapsed = collapsing.bank_collapse_counters();
        let cohorts_in_plan = cohorts(collapsing.bank_shape()[1]);
        // One cohort holds the bypassed track and stops on the block the record lands; the other
        // seven never see a record and collapse throughout. A bypass that had stopped *every*
        // cohort, or one that had stopped none, would both read as this test passing its digest
        // check for the wrong reason.
        assert_eq!(
            collapsed,
            [
                BLOCKS * cohorts_in_plan - (BLOCKS - SWITCH),
                cohorts_in_plan
            ],
            "{effect}: the bypassed cohort must stop collapsing on the block its record lands, and \
             every other cohort must keep collapsing"
        );
        assert_eq!(
            never.bank_collapse_counters(),
            [0, cohorts_in_plan],
            "{effect}: the reference arm must never have collapsed"
        );
        assert!(
            diverged.is_empty(),
            "{effect}: a bypass engaged after a collapsed run diverged from the never-collapsed \
             run on blocks {diverged:?}"
        );
    }
}
