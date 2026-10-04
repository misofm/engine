//! Issue #1051: the soft clip's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # Own snapshots with subnormal words
//!
//! The first run of this differential found that the soft clip refused its own snapshot when it
//! held subnormal words it was legally given (a subnormal mix inside `[0, 1]`, a subnormal input
//! sample in its dry history). #1071 made the restore accept every word the effect can hold, so
//! this test runs without a narrowing.
//!
//! # Ramps that cross a domain edge
//!
//! A D11 ramp rounds its step once and then adds it, so a ramp toward an edge of a parameter's
//! converted range can cross that edge before its final sample assigns the target (#1071 attempt
//! 2). The shared harness's automation rarely starts a ramp a few ulps from an edge, and a crafted
//! restore cannot stand in for one: the harness does not assert that a crafted payload restores,
//! and its own-snapshot restores seldom land inside a ramp's last samples. So
//! [`a_restored_near_edge_ramp_continues_bit_for_bit`] drives the ramp itself, through the public
//! contract calls, and restores its own mid-flight snapshot.

mod support;

use dsp_reference::randomized::{Draw, Profile, replaying, run_seeds};
use effect_contract::{BankWidth, ParameterChannel};
use support::{
    as_input, bits, prepare, prepare_bank, process, process_bank, values_from, word, word_f32,
};

/// Each parameter's external range: drive and output in decibels, the mix as it is.
const RANGES: [(f32, f32); 3] = [(-24.0, 36.0), (-24.0, 24.0), (0.0, 1.0)];
/// Samples rendered after the restore: the rest of the ramp and well past its snap.
const CONTINUATION: usize = 96;

/// The converted value of parameter `parameter`'s external `value`.
fn converted(parameter: usize, value: f32) -> f32 {
    if parameter == 2 {
        value
    } else {
        math::db_to_gain_f32(value)
    }
}

/// `value` moved `ulps` steps toward `toward`, on the `f32` grid.
fn inward(value: f32, toward: f32, ulps: u32) -> f32 {
    let mut value = value;
    for _ in 0..ulps {
        value = if toward > value {
            value.next_up()
        } else {
            value.next_down()
        };
    }
    value
}

/// One parameter ramped from a few ulps inside an edge of its range to that edge, both channels,
/// snapshotted mid-flight, restored into a fresh scalar instance and into a lane of a bank at every
/// width this build binds, and continued: every restore must accept the snapshot and render what
/// the source renders, bit for bit. Returns the crossed edge's index (`2 * parameter + at_top`) if
/// the snapshot's current had crossed it.
fn near_edge_case(draw: &mut Draw) -> Option<usize> {
    let parameter = draw.below(3);
    let (minimum, maximum) = RANGES[parameter];
    let at_top = draw.chance(1, 2);
    let (edge, other) = if at_top {
        (maximum, minimum)
    } else {
        (minimum, maximum)
    };
    // A decibel ulp near an edge moves the gain by about two to seven of its own ulps.
    let ulps = 1 + draw.below(if parameter == 2 { 256 } else { 40 }) as u32;
    let start = inward(edge, other, ulps);
    let mut initial = [(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)];
    for (index, pair) in initial.iter_mut().enumerate() {
        let (low, high) = RANGES[index];
        *pair = (draw.in_domain(low, high), draw.in_domain(low, high));
    }
    initial[parameter] = (start, start);
    let values = values_from(initial);
    let mut source = prepare(&values);

    // A ramp crosses its edge, if at all, in its last few dozen samples.
    let frames = if draw.chance(3, 4) {
        32 + draw.below(32)
    } else {
        1 + draw.below(63)
    };
    let input = |draw: &mut Draw, count: usize| -> Vec<f32> {
        (0..count).map(|_| draw.sample(Profile::Clean)).collect()
    };
    let mut left = input(draw, frames);
    let mut right = input(draw, frames);
    let spans = [
        support::point(parameter as u32, ParameterChannel::Left, edge, 0),
        support::point(parameter as u32, ParameterChannel::Right, edge, 0),
    ];
    process(source.as_mut(), &mut left, &mut right, 0, &spans);
    let payload = support::snapshot(source.as_ref());
    let base = parameter * 4;
    assert!(word(&payload.1, base + 3) > 0, "the ramp is in flight");
    let (low, high) = (converted(parameter, minimum), converted(parameter, maximum));
    let crossed = [&payload.1, &payload.2].into_iter().any(|section| {
        let current = word_f32(section, base);
        current < low || current > high
    });

    let continuation_left = input(draw, CONTINUATION);
    let continuation_right = input(draw, CONTINUATION);
    let first_sample = frames as u64;
    let mut expected_left = continuation_left.clone();
    let mut expected_right = continuation_right.clone();
    process(
        source.as_mut(),
        &mut expected_left,
        &mut expected_right,
        first_sample,
        &[],
    );

    let context =
        format!("parameter {parameter} from {start:e} to {edge:e}, snapshot after {frames} frames");
    let mut scalar = prepare(&values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]));
    scalar
        .restore_state_payload(1, as_input(&payload))
        .unwrap_or_else(|error| panic!("{context}: own snapshot refused ({})", error.code));
    let mut actual_left = continuation_left.clone();
    let mut actual_right = continuation_right.clone();
    process(
        scalar.as_mut(),
        &mut actual_left,
        &mut actual_right,
        first_sample,
        &[],
    );
    assert_eq!(bits(&actual_left), bits(&expected_left), "{context}: left");
    assert_eq!(
        bits(&actual_right),
        bits(&expected_right),
        "{context}: right"
    );

    // A build executes one bank width, the native one: eight lanes on AVX2, four on NEON and
    // Wasm. CI runs both.
    if let Some(width) = BankWidth::for_backend(lane::Backend::current()) {
        let lanes = width.lanes() as usize;
        let per_lane: Vec<Vec<_>> = (0..lanes).map(|_| values.to_vec()).collect();
        let mut bank = prepare_bank(width, &per_lane).expect("a native bank binds");
        let lane = draw.below(lanes);
        bank.restore_track_state_payload(lane as u32, 1, as_input(&payload))
            .unwrap_or_else(|error| {
                panic!(
                    "{context}: own snapshot refused by {width:?} lane {lane} ({})",
                    error.code
                )
            });
        let mut bank_left = vec![0.0_f32; CONTINUATION * lanes];
        let mut bank_right = vec![0.0_f32; CONTINUATION * lanes];
        for frame in 0..CONTINUATION {
            bank_left[frame * lanes + lane] = continuation_left[frame];
            bank_right[frame * lanes + lane] = continuation_right[frame];
        }
        let offsets = vec![0_u32; lanes + 1];
        process_bank(
            bank.as_mut(),
            width,
            &mut bank_left,
            &mut bank_right,
            CONTINUATION,
            first_sample,
            &[],
            &offsets,
        );
        let lane_of = |plane: &[f32]| -> Vec<f32> {
            (0..CONTINUATION)
                .map(|frame| plane[frame * lanes + lane])
                .collect()
        };
        assert_eq!(
            bits(&lane_of(&bank_left)),
            bits(&expected_left),
            "{context}: {width:?} lane {lane} left"
        );
        assert_eq!(
            bits(&lane_of(&bank_right)),
            bits(&expected_right),
            "{context}: {width:?} lane {lane} right"
        );
    }
    crossed.then_some(2 * parameter + usize::from(at_top))
}

/// The soft clip's own mid-ramp snapshots near every domain edge restore and continue bit for bit,
/// into a scalar instance and a bank lane (#1071 attempt 2).
#[test]
fn a_restored_near_edge_ramp_continues_bit_for_bit() {
    let mut crossed = [0_u32; 6];
    let seeds = run_seeds(
        "a_restored_near_edge_ramp_continues_bit_for_bit",
        "cargo test -p soft-clip --test randomized -- --exact \
         a_restored_near_edge_ramp_continues_bit_for_bit",
        256,
        |seed| {
            if let Some(edge) = near_edge_case(&mut Draw::new(seed)) {
                crossed[edge] += 1;
            }
        },
    );
    println!(
        "{seeds} seeds; snapshots past each edge (drive, output, mix; bottom, top): {crossed:?}"
    );
    if !replaying() {
        // Every edge is crossable: the mix's bottom by a negative subnormal, the rest by a few
        // ulps. A generator that stops reaching one is red, not a quietly weaker gate.
        assert!(
            crossed.iter().all(|&count| count > 0),
            "an edge no snapshot reached past: {crossed:?}"
        );
    }
}

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    soft_clip::SoftClipFactory,
    seeds: 8,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: true,
    witness: false,
);

/// The D7 recovery's report against the contract, on fixed input (no seed): red until #1073
/// lands. This effect counts the frames of a failing block, on both channels and on every lane of the bank, where the contract counts blocks on the lane that failed.
#[test]
#[ignore = "#1073: the D7 recovery's report breaks the contract; see the test's documentation"]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(&soft_clip::SoftClipFactory);
}

/// #1278: the plan-swap carry restores every lane it carries, so a restore must accept every state
/// the effect itself reaches -- including a smoothed ramp to a domain edge whose iterated
/// `current + step` has rounded past the edge, at every launch rate. Red on a restore that holds a
/// moving ramp's `current` (or a subnormal step) to the strict domain.
///
/// The shared probe and [`a_restored_near_edge_ramp_continues_bit_for_bit`] overlap without either
/// superseding the other. The probe is the harness-wide contract every banked effect runs: seedless,
/// at every quality row, restoring after every sample of the ramp, so it reaches all six edges at
/// every remaining count by construction, where the seeded test draws its snapshot points. But it
/// only asserts that the restore is accepted. The seeded test also renders the
/// restored lane, scalar and in a bank lane, and compares it bit for bit, so a restore that accepts
/// an overshooting current and then clamps it into range is green here and red there.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(&soft_clip::SoftClipFactory);
}
