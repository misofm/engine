//! E2 — the bank and `W` scalar instances render the same bits, per lane.
//!
//! Two halves, because a bank binding is only available at the width this build was compiled for
//! (D4 revision 4: no runtime SIMD dispatch):
//!
//! * the **production** half compares a bound bank against `W` separately prepared scalar
//!   instances, sample by sample, plus their per-track payload bytes and their reports;
//! * the **width** half runs the frozen corpus through the same kernel at `W = 1`, 4 and 8 and
//!   compares the result words, which covers the two widths a given host cannot bind.
//!
//! The tracks are deliberately heterogeneous — different lookahead, threshold, ratio, knee and mix
//! — because a bank that used lane 0's coefficients for every lane would pass a homogeneous test.

mod support;

use miso_engine_compressor::corpus::{CASE_NAMES, POINTS, run_case};
use miso_engine_effect_contract::{AutomationSpanKind, ParameterChannel, PreparedAutomationSpan};
use miso_engine_lane::{Simd4, Simd8};

use support::{
    bind_bank, native_bank_width, noise, prepare, render_bank, render_scalar, request_with_quantum,
    snapshot, snapshot_track, values_with,
};

const FRAMES: usize = 2_048;
const QUANTUM: u32 = 128;

/// Per-track parameters: no two tracks agree on anything the kernel reads per lane.
fn track_values(track: usize) -> [miso_engine_effect_contract::InitialParameterValue; 16] {
    values_with(&[
        (0, -10.0 - 6.0 * track as f32),
        (1, 1.5 + 2.0 * track as f32),
        (2, 3.0 * (track % 4) as f32),
        (3, 0.5 + track as f32),
        (4, 20.0 + 40.0 * track as f32),
        (5, -3.0 + track as f32),
        (6, 0.25 + 0.1 * (track % 4) as f32),
        (7, 2.5 * (track % 5) as f32),
    ])
}

/// A bank renders each lane exactly as the scalar instance for that track does.
///
/// Red mutations (MUTATIONS.md rows 1 and 12), both proven: make `gather_detector` use `delay[0]`
/// for every lane, and make the `design_lane` scatter write lane 0 for every lane.
#[test]
fn bank_matches_scalar_per_lane_bits() {
    let Some((_, width)) = native_bank_width() else {
        println!("scalar-only build: the width half of E2 covers this case");
        return;
    };
    let lanes = width.lanes() as usize;
    let values: Vec<_> = (0..lanes).map(track_values).collect();
    let requests: Vec<_> = values
        .iter()
        .map(|v| request_with_quantum(v, QUANTUM))
        .collect();
    let mut bank = bind_bank(&requests).expect("bank must bind at this build's width");

    // One Point early on track 1 and one later on the last track: automation is per lane too.
    let spans = vec![
        (
            3 * 128_u64,
            1_usize,
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: 0,
                end_sample: 0,
                start_value: -44.0,
                end_value: -44.0,
            },
        ),
        (
            9 * 128_u64,
            lanes - 1,
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Right,
                parameter_index: 6,
                start_sample: 0,
                end_sample: 0,
                start_value: 0.9,
                end_value: 0.9,
            },
        ),
    ];

    let mut bank_left = vec![0.0_f32; FRAMES * lanes];
    let mut bank_right = vec![0.0_f32; FRAMES * lanes];
    let mut per_track: Vec<(Vec<f32>, Vec<f32>)> = Vec::new();
    for track in 0..lanes {
        let left = noise(FRAMES, 0x1A_4E_00_10 + track as u64, 0.8);
        let right = noise(FRAMES, 0x1A_4E_00_20 + track as u64, 0.8);
        for frame in 0..FRAMES {
            bank_left[frame * lanes + track] = left[frame];
            bank_right[frame * lanes + track] = right[frame];
        }
        per_track.push((left, right));
    }
    render_bank(
        bank.as_mut(),
        &mut bank_left,
        &mut bank_right,
        lanes,
        width,
        128,
        QUANTUM,
        &spans,
    );

    for track in 0..lanes {
        let mut effect = prepare(request_with_quantum(&values[track], QUANTUM));
        let scalar_spans: Vec<(u64, PreparedAutomationSpan)> = spans
            .iter()
            .filter(|(_, lane, _)| *lane == track)
            .map(|(at, _, span)| (*at, *span))
            .collect();
        let (input_left, input_right) = &per_track[track];
        let mut left = input_left.clone();
        let mut right = input_right.clone();
        render_scalar(
            effect.as_mut(),
            &mut left,
            &mut right,
            128,
            QUANTUM,
            &scalar_spans,
        );

        let bank_lane_left: Vec<u32> = (0..FRAMES)
            .map(|frame| bank_left[frame * lanes + track].to_bits())
            .collect();
        let bank_lane_right: Vec<u32> = (0..FRAMES)
            .map(|frame| bank_right[frame * lanes + track].to_bits())
            .collect();
        assert_eq!(
            bank_lane_left,
            left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
            "track {track}, left"
        );
        assert_eq!(
            bank_lane_right,
            right.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
            "track {track}, right"
        );
        assert!(
            left[1_500..].iter().any(|sample| *sample != 0.0),
            "track {track} rendered nothing"
        );

        let (scalar_state_left, scalar_state_right) = snapshot(effect.as_ref());
        let (bank_state_left, bank_state_right) =
            snapshot_track(bank.as_ref(), track as u32, effect.as_ref());
        assert_eq!(
            bank_state_left, scalar_state_left,
            "track {track} left state"
        );
        assert_eq!(
            bank_state_right, scalar_state_right,
            "track {track} right state"
        );
    }
}

/// The one kernel body produces identical result words at `W = 1`, 4 and 8.
///
/// Run over the frozen corpus of `tests/cross_target.rs`, so this is the same computation the wasm
/// leg replays; the difference is that this test compares the three widths to *each other* rather
/// than to a pin, and reports the first differing word rather than a digest.
#[test]
fn every_width_produces_the_same_words() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut scalar = vec![0_u32; POINTS];
        let mut wide4 = vec![0_u32; POINTS];
        let mut wide8 = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut scalar);
        run_case::<Simd4>(case, &mut wide4);
        run_case::<Simd8>(case, &mut wide8);
        for (index, ((one, four), eight)) in scalar
            .iter()
            .zip(wide4.iter())
            .zip(wide8.iter())
            .enumerate()
        {
            assert_eq!(
                (one, four, eight),
                (one, one, one),
                "{name}: word {index} differs across widths"
            );
        }
    }
}
