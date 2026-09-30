//! Issue #1005 gate 3: EQ gain rides through the live-control path, pinned before the
//! ramping-block elision landed.
//!
//! A ramping EQ bank used to run all six sections per channel as single-chain passes; #1005 runs
//! only the live or ramping ones, under the stationary elision gate. That is a schedule change,
//! so every rendered bit must stay where it was. These scenarios drive the production path end to
//! end -- `SessionRuntime::push_parameter`, the EQ's prepared-target owner, the bank chain, the
//! mono collapse -- and fold every master output word of every block into one SHA-256 per
//! scenario. The digests were pinned on the unmodified batch head (`a1fcab3d`).
//!
//! * `sixty_four_track_eq_only` with band-1 gain riding on eight of 64 tracks (one lane per
//!   eight-lane bank) and on all 64. Each edit is a `Left` then a `Right` owner edit, the
//!   automation diagnosis' shape, so each lane takes two one-channel targets.
//! * `sixty_four_track_console_mono` with the same rides pushed as one `Both` owner edit per block:
//!   the SDK's lowering, which designs one `Both` target and keeps the mono collapse, so the
//!   collapsed body's ramping path is the one exercised (VERIFY-AUTOMATION amendment A3).
//!
//! Each at the native width (`Simd8`) and at `Simd4` dispatch. Every arm first settles every
//! track's band-1 gain at 3 dB and renders a pre-roll, then alternates 3 +/- 0.25 dB every block,
//! so a 64-sample window opens in every block. A `settled` arm (the same settling, no ride) is
//! rendered beside each and must differ from both rides (VERIFY-AUTOMATION F6), so no pin here
//! could pass on a ride that never moved a bit.

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

/// `[settled, eight_of_64, all_64]` on `sixty_four_track_eq_only`, pinned at `a1fcab3d`. The two
/// widths render the same bits, as every width does (decision D5).
const EQ_ONLY_PINS: [&str; 3] = [
    "81363c22f9171648b9bf1629f863d559be8a0cb76f3bcf2a8fadff6c357cf8dd",
    "5470166c531bb9a57fd715c6d6a8a27472b6110ea6a36c6625e46b29a66ebcf7",
    "b51448fdb027d73ffbedb741ea876731cb82a87b33c7c56f3dac807ef4061f3b",
];

/// `[settled, eight_of_64, all_64]` on `sixty_four_track_console_mono`, pinned at `a1fcab3d`.
const MONO_PINS: [&str; 3] = [
    "f973869ed747bd9fd32052defb35096f58c5d2b489dffe7e4c7db0184f41c335",
    "bf0e96d189258941db89b49bd3c854a4df9e8bb3d73dc64f0b755f038c03649a",
    "6e063a3e64202ccabc76621c747399f0ab47b90575b357f6d59cc79b3be16fa7",
];

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

fn check(workload: Workload, dispatch: Backend, pins: [&str; 3]) {
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
    assert_eq!(
        [settled.as_str(), eight.as_str(), all.as_str()],
        pins,
        "{name} {dispatch:?}: [settled, eight_of_64, all_64] moved"
    );
}

#[test]
fn eq_only_gain_rides_render_the_pinned_bits_at_simd8() {
    check(Workload::SixtyFourTrackEqOnly, Backend::Simd8, EQ_ONLY_PINS);
}

#[test]
fn eq_only_gain_rides_render_the_pinned_bits_at_simd4() {
    check(Workload::SixtyFourTrackEqOnly, Backend::Simd4, EQ_ONLY_PINS);
}

#[test]
fn mono_console_both_rides_render_the_pinned_bits_at_simd8() {
    check(
        Workload::SixtyFourTrackConsoleMono,
        Backend::Simd8,
        MONO_PINS,
    );
}

#[test]
fn mono_console_both_rides_render_the_pinned_bits_at_simd4() {
    check(
        Workload::SixtyFourTrackConsoleMono,
        Backend::Simd4,
        MONO_PINS,
    );
}
