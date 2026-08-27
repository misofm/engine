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
