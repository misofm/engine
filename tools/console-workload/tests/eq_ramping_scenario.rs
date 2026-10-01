//! EQ gain rides through the live-control path agree between supported widths.
//!
//! These scenarios drive `SessionRuntime::push_parameter`, the EQ's prepared-target owner, the
//! bank chain and mono collapse, hashing every master word after the pre-roll for each scenario.
//!
//! * `sixty_four_track_eq_only` with band-1 gain riding on eight of 64 tracks (one lane per
//!   eight-lane bank) and on all 64. Each edit is a `Left` then a `Right` owner edit, the
//!   automation diagnosis' shape, so each lane takes two one-channel targets.
//! * `sixty_four_track_console_mono` with the same rides pushed as one `Both` owner edit per block:
//!   the SDK's lowering, which designs one `Both` target and keeps the mono collapse, so the
//!   collapsed body's ramping path is the one exercised (VERIFY-AUTOMATION amendment A3).
//!
//! Each at every supported vector width. Every arm first settles every
//! track's band-1 gain at 3 dB and renders a pre-roll, then alternates 3 +/- 0.25 dB every block,
//! so a 64-sample window opens in every block. A `settled` arm (the same settling, no ride) is
//! rendered beside each and must differ from both rides, which must also differ from each other.

use bench_support::digest::Sha256Sink;
use console_workload::{ObservationArm, PlanConfig, SessionRuntime, Workload};
use effect_contract::ParameterChannel;
use lane::Backend;

const CONFIG: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};
const EQ: &str = "miso.parametric-eq";
/// Band-1 gain, by descriptor index.
const GAIN: u32 = 3;
const BASE_DB: f32 = 3.0;
const STEP_DB: f32 = 0.25;
const PREROLL: u64 = 64;
const BLOCKS: u64 = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Settled,
    EightOf64,
    All64,
}

impl Arm {
    fn tracks(self) -> Vec<usize> {
        match self {
            Self::Settled => Vec::new(),
            Self::EightOf64 => (0..8).map(|bank| bank * 8).collect(),
            Self::All64 => (0..64).collect(),
        }
    }
}

fn track_id(track: usize) -> String {
    format!("ch{track:02}")
}

/// The channels one edit is pushed on: the diagnosis' `Left` then `Right` on the stereo row, the
/// SDK's one `Both` on the mono row.
fn sides(workload: Workload) -> &'static [ParameterChannel] {
    match workload {
        Workload::SixtyFourTrackConsoleMono => &[ParameterChannel::Both],
        _ => &[ParameterChannel::Left, ParameterChannel::Right],
    }
}

/// One scenario's digest over every block after the pre-roll.
fn digest(workload: Workload, dispatch: Backend, arm: Arm) -> String {
    let mut runtime = SessionRuntime::build_with_dispatch(workload, CONFIG, dispatch);
    let channels: Vec<usize> = (0..64)
        .map(|track| {
            runtime
                .control_channel(&track_id(track), EQ)
                .expect("an EQ control channel on every track")
        })
        .collect();
    for &channel in &channels {
        for &side in sides(workload) {
            assert!(
                runtime.push_parameter(channel, GAIN, side, BASE_DB),
                "the settling push was refused"
            );
        }
    }
    for block in 0..PREROLL {
        runtime.render(block).expect("pre-roll render");
    }
    let riding: Vec<usize> = arm.tracks().into_iter().map(|t| channels[t]).collect();
    let mut hash = Sha256Sink::new();
    for block in PREROLL..PREROLL + BLOCKS {
        let value = if block.is_multiple_of(2) {
            BASE_DB + STEP_DB
        } else {
            BASE_DB - STEP_DB
        };
        for &channel in &riding {
            for &side in sides(workload) {
                assert!(
                    runtime.push_parameter(channel, GAIN, side, value),
                    "block {block}: a ride push was refused"
                );
            }
        }
        runtime.render(block).expect("render");
        runtime.hash_output(&mut hash);
    }
    if workload == Workload::SixtyFourTrackConsoleMono {
        // The `Both` lowering keeps every cohort collapsed, so the collapsed ramping body is what
        // rendered: the mono fixture's cohorts take the collapse on every block, riding or not.
        let [collapsed, cohorts] = runtime.bank_collapse_counters();
        assert!(
            cohorts > 0,
            "the mono row prepared no collapse-eligible cohort"
        );
        assert_eq!(
            collapsed,
            cohorts * (PREROLL + BLOCKS),
            "{arm:?} at {dispatch:?}: a Both ride retired a cohort's collapse"
        );
    }
    hash.finish_hex()
}

fn check(workload: Workload) {
    let mut first_digests = None;
    for &dispatch in Backend::VECTOR.iter().rev() {
        if dispatch.width() > Backend::current().width() {
            continue;
        }
        let settled = digest(workload, dispatch, Arm::Settled);
        let eight = digest(workload, dispatch, Arm::EightOf64);
        let all = digest(workload, dispatch, Arm::All64);
        let name = workload.kind();
        assert_ne!(
            settled, eight,
            "{name} {dispatch:?}: eight_of_64 moved no bit"
        );
        assert_ne!(settled, all, "{name} {dispatch:?}: all_64 moved no bit");
        assert_ne!(
            eight, all,
            "{name} {dispatch:?}: all_64 rendered eight_of_64's bits"
        );
        let digests = [settled, eight, all];
        if let Some(expected) = &first_digests {
            assert_eq!(
                &digests, expected,
                "{name} {dispatch:?}: [settled, eight_of_64, all_64] differs by width"
            );
        } else {
            first_digests = Some(digests);
        }
    }
}

#[test]
fn eq_only_gain_rides_move_bits_and_agree_between_supported_widths() {
    check(Workload::SixtyFourTrackEqOnly);
}

#[test]
fn mono_console_both_rides_move_bits_and_agree_between_supported_widths() {
    check(Workload::SixtyFourTrackConsoleMono);
}
