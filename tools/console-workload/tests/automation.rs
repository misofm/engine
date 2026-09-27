//! The premises the `console_automation` bench row rests on, asserted rather than assumed.
//!
//! That row exists because the standing console rows cannot see a compressor's ramping body:
//! `console_model` clears the fixture's automation table unconditionally, both fixture gates
//! assert the standing sessions declare none, and the only arm in the benchmark that delivers
//! spans drives parametric EQs. The row's answer is the console's real traffic shape -- one Point
//! span per block, on one track, through the live-console control queue.
//!
//! Three things have to be true for its paired ramp delta to mean what it says, and none of them
//! is something a benchmark may assume:
//!
//! * restating a parameter at the value it already holds must move no rendered bit, or the row's
//!   class-A arm is not a baseline;
//! * moving it must move rendered bits, or the row measures the cost of nothing; and
//! * "one track" must be a stable choice, or the row addresses a different track between builds.
//!
//! The benchmark asserts the first two in-run as well. They are here because a benchmark runs on a
//! quiesced host under a preflight and this suite runs on every change.

use bench_support::digest::Sha256Sink;
use console_workload::{ObservationArm, PlanConfig, SessionRuntime, Workload};
use effect_contract::ParameterChannel;

/// The row's subject and its plan, transcribed from `tools/bench/src/console.rs`.
const WORKLOAD: Workload = Workload::SixtyFourTrackCompressorOnly;
const CONFIG: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};
const EFFECT_ID: &str = "miso.compressor";
const PARAMETER_INDEX: u32 = 0;
const BASE_DB: f32 = -24.0;
const STEP_DB: f32 = 0.5;

/// Enough blocks for every detector to be well past its transient, and for a restated parameter
/// to have been restated many times over.
const BLOCKS: u64 = 64;

/// Blocks of untimed pre-roll, matching the benchmark's: every arm is settled at [`BASE_DB`]
/// before anything is compared, which is what makes the restated arm and the quiet arm comparable.
const PREROLL: u64 = 64;

/// What the benchmark's `automated` arm pushes before block `block`.
///
/// It alternates either side of the base rather than restating the base on alternate blocks, so
/// that every block opens a real smoothing window; see the constant's documentation in the bench.
fn moving_value(block: u64) -> f32 {
    if block.is_multiple_of(2) {
        BASE_DB + STEP_DB
    } else {
        BASE_DB - STEP_DB
    }
}

/// Builds one arm and settles it at the base threshold, as the benchmark's pre-roll does.
fn settled_arm() -> (SessionRuntime, usize) {
    let mut runtime = SessionRuntime::build(WORKLOAD, CONFIG);
    let channel = runtime
        .first_track_control_channel(EFFECT_ID)
        .expect("the compressor decomposition row prepares a compressor control channel");
    assert!(
        runtime.push_parameter(channel, PARAMETER_INDEX, ParameterChannel::Left, BASE_DB),
        "the bounded control queue refused the pre-roll push"
    );
    for block in 0..PREROLL {
        runtime.render(block).expect("pre-roll render");
    }
    (runtime, channel)
}

/// The #144 stationary hoist, at console level and on the compressor.
///
/// The benchmark's `restated` arm pushes the threshold's own value on every block and is asserted
/// byte-identical to an arm that pushes nothing. That is the whole basis for reading the paired
/// ramp delta as the cost of the *window* rather than the cost of the queue drain: both arms
/// deliver one record per block through the same bounded queue, and only one of them opens a ramp.
///
/// Block by block rather than one folded digest: the property is bit-exactness, and a test for
/// bit-exactness should not rest on a collision argument.
#[test]
fn restating_the_threshold_moves_no_rendered_bit() {
    let (mut quiet, _) = settled_arm();
    let (mut restated, channel) = settled_arm();
    for block in 0..BLOCKS {
        assert!(
            restated.push_parameter(channel, PARAMETER_INDEX, ParameterChannel::Left, BASE_DB),
            "block {block}: the bounded control queue refused a restating push"
        );
        quiet.render(block).expect("quiet render");
        restated.render(block).expect("restated render");
        let mut left = Sha256Sink::new();
        let mut right = Sha256Sink::new();
        quiet.hash_output(&mut left);
        restated.hash_output(&mut right);
        assert_eq!(
            left.finish_hex(),
            right.finish_hex(),
            "block {block}: restating the compressor's threshold moved a rendered bit"
        );
    }
}

/// The other half of the honesty gate: the automated arm must actually automate.
///
/// A step that designed to the same coefficient words would leave the two arms rendering identical
/// audio and the row would report the cost of a window that never opened -- which is exactly the
/// failure the EQ hoist arm recorded when it tried a one-ULP control step. The assertion is made
/// on the *first* block, not on a folded digest, so the row cannot pass on a window that opens
/// only later in a run.
#[test]
fn moving_the_threshold_moves_rendered_bits_on_every_block() {
    let (mut quiet, _) = settled_arm();
    let (mut automated, channel) = settled_arm();
    for block in 0..BLOCKS {
        assert!(
            automated.push_parameter(
                channel,
                PARAMETER_INDEX,
                ParameterChannel::Left,
                moving_value(block),
            ),
            "block {block}: the bounded control queue refused a moving push"
        );
        quiet.render(block).expect("quiet render");
        automated.render(block).expect("automated render");
        let mut left = Sha256Sink::new();
        let mut right = Sha256Sink::new();
        quiet.hash_output(&mut left);
        automated.hash_output(&mut right);
        assert_ne!(
            left.finish_hex(),
            right.finish_hex(),
            "block {block}: moving the compressor's threshold changed nothing"
        );
    }
}

/// "One track" is chosen by the stable session identity, not by position.
///
/// `attach_effect_console` returns channels in prepared-entry order, which is sorted by effect
/// id and not by track, so taking the first matching channel would silently address a different
/// track when the entry set changed. The row names the track it automated in its record; this pins
/// that the name is derived from a key that cannot drift.
#[test]
fn the_automated_track_is_the_stable_minimum() {
    let runtime = SessionRuntime::build(WORKLOAD, CONFIG);
    let channel = runtime
        .first_track_control_channel(EFFECT_ID)
        .expect("a compressor control channel");
    let (track_id, _) = runtime.control_identity(channel);
    assert_eq!(
        track_id, "ch00",
        "the automated track must be the alphabetically first track carrying the compressor"
    );
    assert!(
        runtime
            .first_track_control_channel("miso.parametric-eq")
            .is_none(),
        "the compressor decomposition row must carry no EQ: its strip edit drops that slot"
    );
}

/// A `control: false` plan has no channel to address, and the accessor says so rather than
/// panicking or silently returning channel zero.
#[test]
fn a_plan_without_a_control_channel_resolves_nothing() {
    let runtime = SessionRuntime::new(WORKLOAD);
    assert!(
        runtime.first_track_control_channel(EFFECT_ID).is_none(),
        "PlanConfig::BASELINE attaches no live-console control channel"
    );
}

// -------------------------------------------------------------------------------------------------
// The `console_mixing_automation` row (issue #1003): the mono console riding eight controls.
// -------------------------------------------------------------------------------------------------

mod mixing {
    use bench_support::digest::Sha256Sink;
    use console_workload::SessionRuntime;
    use console_workload::mixing_automation::{
        self, Arm, CONTROLS, Effect, Lowering, MixingArm, MixingAutomation, PREFLIGHT_BLOCKS,
        ResolveError,
    };
    use effect_contract::ParameterChannel;
    use lane::Backend;

    /// Every control resolves by `(track_id, effect contract id)` to its own channel, and names
    /// the slot, the lowering and the held base the row claims.
    ///
    /// The bases are the fixture's held values (VERIFY-AUTOMATION A2's table), read from the model
    /// rather than written into the row: a base that is not the held value makes `restated` an
    /// edit, and the row's class-A statement false.
    #[test]
    fn the_eight_controls_resolve_by_id_to_their_held_values() {
        let runtime = SessionRuntime::build(mixing_automation::WORKLOAD, mixing_automation::CONFIG);
        let automation = MixingAutomation::resolve(&runtime).expect("the eight controls resolve");
        let expected = [
            ("ch00", "eq", -7.5, Lowering::OwnerBoth),
            ("ch08", "comp", -18.0, Lowering::LeftThenRight),
            ("ch16", "limiter", -1.0, Lowering::LeftThenRight),
            ("ch24", "eq", 1.5, Lowering::OwnerBoth),
            ("ch32", "comp", -27.0, Lowering::LeftThenRight),
            ("ch40", "limiter", -1.75, Lowering::LeftThenRight),
            ("ch48", "eq", -4.5, Lowering::OwnerBoth),
            ("ch56", "comp", -9.0, Lowering::LeftThenRight),
        ];
        let controls = automation.controls();
        assert_eq!(controls.len(), expected.len());
        for (control, (track, slot, base, lowering)) in controls.iter().zip(expected) {
            assert_eq!(
                runtime.control_identity(control.channel),
                (track, slot),
                "{track}: the channel resolved by id addresses that track's slot"
            );
            assert_eq!(control.slot_id, slot);
            assert_eq!(
                control.base.to_bits(),
                f32::to_bits(base),
                "{track}: held base"
            );
            assert_eq!(
                control.control.effect.lowering(),
                lowering,
                "{track}: lowering"
            );
            assert!(
                control.values[0] > control.base && control.base > control.values[1],
                "{track}: the ride straddles its base"
            );
        }
        let mut channels: Vec<usize> = controls.iter().map(|control| control.channel).collect();
        channels.sort_unstable();
        channels.dedup();
        assert_eq!(channels.len(), CONTROLS.len(), "eight distinct channels");
        // One track per eight-lane bank.
        let mut banks: Vec<usize> = controls
            .iter()
            .map(|control| control.track_index / 8)
            .collect();
        banks.dedup();
        assert_eq!(banks, (0..8).collect::<Vec<_>>());
        assert_eq!(automation.pushes_per_block(None), 13);
    }

    /// A plan without the control channel resolves nothing, and says which control it missed.
    #[test]
    fn a_plan_without_a_control_channel_refuses_the_row() {
        let runtime = SessionRuntime::new(mixing_automation::WORKLOAD);
        assert_eq!(
            MixingAutomation::resolve(&runtime).err(),
            Some(ResolveError::MissingChannel {
                track_id: "ch00",
                effect: "miso.parametric-eq",
            })
        );
    }

    /// Restating every held value, in the host's lowerings, moves no rendered bit on any block.
    ///
    /// The collapse is at stake here and nowhere else in this file: the compressor's and the
    /// limiter's Left and Right records retire their cohorts' collapse, and the dual bank must then
    /// render the collapsed bank's bits. Block by block, because the property is bit-exactness.
    #[test]
    fn restating_the_held_values_moves_no_rendered_bit() {
        let backend = Backend::current();
        let mut quiet = MixingArm::prepare(Arm::Quiet, None, backend);
        let mut restated = MixingArm::prepare(Arm::Restated, None, backend);
        for block in 0..PREFLIGHT_BLOCKS {
            quiet.drive();
            restated.drive();
            quiet.render().expect("quiet render");
            restated.render().expect("restated render");
            let mut left = Sha256Sink::new();
            let mut right = Sha256Sink::new();
            quiet.runtime.hash_output(&mut left);
            restated.runtime.hash_output(&mut right);
            assert_eq!(
                left.finish_hex(),
                right.finish_hex(),
                "block {block}: restating the held values moved a rendered bit"
            );
        }
        assert_eq!(restated.tally.accepted, restated.tally.attempted);
        assert_eq!(restated.tally.attempted, 13 * PREFLIGHT_BLOCKS);
    }

    /// Every automated effect moves bits on its own, and the whole ride moves them on every
    /// block; the EQ's restatement keeps every cohort collapsed. The preflight the bench runs
    /// before it takes a number, run here on every change.
    #[test]
    fn the_preflight_premises_hold() {
        let preflight = mixing_automation::preflight(Backend::current());
        preflight.assert_premises();
        let quiet = preflight.collapse(Arm::Quiet, None);
        assert_eq!(
            quiet[1], 8,
            "the mono console forms eight cohorts at this width"
        );
        // Today, restating the compressor and the limiter retires the five cohorts they sit in;
        // the automation fixes move this, so it is stated, not asserted, by the bench.
        let restated = preflight.collapse(Arm::Restated, None);
        assert!(restated[0] <= quiet[0]);
    }

    /// The EQ is pushed as the SDK pushes it. A harness-only lowering -- one owner edit per
    /// channel, as `push_parameter` would make of a Left then a Right -- designs two one-channel
    /// targets and retires the collapse of every bank it touches (VERIFY-AUTOMATION F2), which is
    /// the loss the row must not measure. The `Both` edit keeps it.
    #[test]
    fn the_eq_lowering_keeps_the_collapse_and_the_harness_only_lowering_would_not() {
        let backend = Backend::current();
        let eq_both = MixingArm::prepare(Arm::Restated, Some(Effect::Eq), backend);
        let quiet = MixingArm::prepare(Arm::Quiet, None, backend);
        assert_eq!(
            eq_both.runtime.bank_collapse_counters(),
            quiet.runtime.bank_collapse_counters(),
            "a Both EQ restatement keeps every cohort collapsed"
        );
        let mut split = SessionRuntime::build_with_dispatch(
            mixing_automation::WORKLOAD,
            mixing_automation::CONFIG,
            backend,
        );
        let automation = MixingAutomation::resolve(&split).expect("controls");
        for control in automation
            .controls()
            .iter()
            .filter(|control| control.control.effect == Effect::Eq)
        {
            for side in [ParameterChannel::Left, ParameterChannel::Right] {
                assert!(split.push_parameter(
                    control.channel,
                    control.control.parameter_index,
                    side,
                    control.base
                ));
            }
        }
        for block in 0..mixing_automation::PREROLL_BLOCKS {
            split.render(block).expect("render");
        }
        let [kept, cohorts] = quiet.runtime.bank_collapse_counters();
        let [split_kept, _] = split.bank_collapse_counters();
        assert_eq!(
            split_kept,
            kept - 3 * mixing_automation::PREROLL_BLOCKS,
            "two one-channel EQ edits retire the three EQ cohorts of the {cohorts}"
        );
    }
}
