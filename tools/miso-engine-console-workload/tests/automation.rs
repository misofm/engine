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

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_console_workload::{ObservationArm, PlanConfig, SessionRuntime, Workload};
use miso_engine_effect_contract::ParameterChannel;

/// The row's subject and its plan, transcribed from `tools/miso-engine-bench/src/console.rs`.
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
/// `attach_effect_console_v1` returns channels in prepared-entry order, which is sorted by effect
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
