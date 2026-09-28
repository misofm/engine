//! Issue #1006 gate 3: the ramping prefix through the bank contract, pinned before the rewrite.
//!
//! A bank at this build's width (eight lanes natively), rendered dual (`process_bank`) and
//! collapsed (`process_bank_mono`) side by side over 128 blocks of mixed lengths, so that a
//! 64-sample window opens in one block and closes in the next. The automation is Points through
//! the contract's span table: a threshold ride on one lane, an attack ride on every lane, a
//! makeup and mix ride on an all-wet table, and one left-only threshold write. Between blocks
//! there are two `DiscontinuityKeepParameters` resets while windows are open and one restore of a
//! payload whose threshold ramp is at `remaining = 0` with `current != target`. The input carries
//! hostile words. Every output word, every report and every track's payload after every block is
//! folded into one SHA-256, pinned on the unmodified batch head (`081fdc6c`).
//!
//! Everything goes through the production entry points; the four-lane bank the browser binds is
//! covered at kernel level by `kernel::settled_body_tests::scenario_1006_ramping_prefix_is_pinned`.

mod support;

use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, LinkMode, ParameterChannel,
    PreparedAutomationSpan, PreparedNativeEffectBank, ResetKind,
};
use sha2::{Digest, Sha256};

const QUANTUM: u32 = 128;
const FRAMES: [usize; 12] = [128, 37, 64, 1, 100, 31, 33, 128, 50, 97, 7, 63];

/// Per-lane values: threshold, ratio, knee, attack, release, makeup, mix.
const MIXED: [[f32; 7]; 8] = [
    [-18.0, 4.0, 0.0, 10.0, 100.0, 0.0, 1.0],
    [-24.0, 8.0, 24.0, 1.0, 50.0, 3.0, 0.75],
    [-6.0, 1.0, 6.0, 5.0, 200.0, -6.0, 0.5],
    [-40.0, 20.0, 12.0, 0.1, 5.0, 12.0, 0.0],
    [0.0, 2.0, 6.0, 50.0, 1000.0, -24.0, 0.25],
    [-80.0, 1.5, 3.0, 20.0, 5000.0, 24.0, 0.9],
    [-12.0, 12.0, 18.0, 0.5, 20.0, -3.0, 0.6],
    [-30.0, 6.0, 0.0, 2.0, 300.0, 6.0, 1.0],
];

/// The standing console fixture's first eight compressors: all wet.
const WET: [[f32; 7]; 8] = [
    [-6.0, 1.5, 3.0, 2.0, 40.0, 0.0, 1.0],
    [-7.5, 2.25, 4.5, 3.5, 55.0, 0.5, 1.0],
    [-9.0, 3.0, 6.0, 5.0, 70.0, 1.0, 1.0],
    [-10.5, 3.75, 7.5, 6.5, 85.0, 1.5, 1.0],
    [-12.0, 4.5, 9.0, 8.0, 100.0, 2.0, 1.0],
    [-13.5, 5.25, 3.0, 9.5, 115.0, 2.5, 1.0],
    [-15.0, 6.0, 4.5, 11.0, 130.0, 3.0, 1.0],
    [-16.5, 6.75, 6.0, 12.5, 145.0, 0.0, 1.0],
];

const HOSTILE: [u32; 12] = [
    0x0000_0000,
    0x8000_0000,
    0x0000_0001,
    0x807f_ffff,
    0x7f80_0000,
    0xff80_0000,
    0x7fc0_1234,
    0xffa0_0001,
    0x7016_7699,
    0xf217_5d67,
    0x3f80_0000,
    0x322b_cc77,
];

fn point(
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
    first: u64,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: first,
        end_sample: first,
        start_value: value,
        end_value: value,
    }
}

/// The spans of block `block`, per lane, and their offsets.
fn spans(
    block: usize,
    lanes: usize,
    wet: bool,
    first: u64,
) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let both = [ParameterChannel::Left, ParameterChannel::Right];
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes + 1];
    for lane in 0..lanes {
        // Spans are ordered by `(parameter, channel)` within a lane, as the drain stages them.
        if lane == 1 && block.is_multiple_of(2) {
            let value = if block.is_multiple_of(4) {
                -30.0
            } else {
                -20.0
            };
            for channel in both {
                spans.push(point(0, channel, value, first));
            }
        } else if lane == 1 && block % 11 == 3 {
            spans.push(point(0, ParameterChannel::Left, -12.5, first));
        }
        if block % 5 == 1 {
            let value = if block % 10 == 1 { 3.0 } else { 12.0 };
            for channel in both {
                spans.push(point(3, channel, value, first));
            }
        }
        if wet && block % 4 == 1 {
            let value = if block % 8 == 1 { 3.0 } else { 0.0 };
            for channel in both {
                spans.push(point(5, channel, value, first));
            }
        }
        if wet && lane == 0 && block % 7 == 2 {
            let value = if block % 14 == 2 { 0.8 } else { 1.0 };
            for channel in both {
                spans.push(point(6, channel, value, first));
            }
        }
        offsets[lane + 1] = spans.len() as u32;
    }
    (spans, offsets)
}

fn input(block: usize, lanes: usize, frames: usize, seed: u64) -> Vec<f32> {
    let mut words = support::noise(frames * lanes, seed + block as u64, 0.9);
    if block % 9 == 4 {
        for (index, word) in words.iter_mut().enumerate().step_by(13) {
            *word = f32::from_bits(HOSTILE[(index / 13 + block) % HOSTILE.len()]);
        }
    }
    if block % 9 == 7 {
        for word in &mut words {
            *word *= 1.0e-7;
        }
    }
    words
}

fn fold_report(
    hasher: &mut Sha256,
    report: &effect_contract::BankProcessReport,
    lanes: usize,
    totals: &mut [u64; 2],
) {
    for report in report.reports.iter().take(lanes) {
        totals[0] += report.invalid_spans;
        totals[1] += report.nonfinite_left_blocks + report.nonfinite_right_blocks;
        for value in [
            report.sanitized_main_samples,
            report.sanitized_sidechain_samples,
            report.invalid_spans,
            report.nonfinite_left_blocks,
            report.nonfinite_right_blocks,
        ] {
            hasher.update(value.to_le_bytes());
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    width: BankWidth,
    first: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
    mono: bool,
) -> effect_contract::BankProcessReport {
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        frames as u32,
        width,
        first,
        spans,
        offsets,
        QUANTUM,
    )
    .expect("bounded bank block");
    if mono {
        bank.process_bank_mono(block)
    } else {
        bank.process_bank(block)
    }
}

fn scenario(
    hasher: &mut Sha256,
    table: &[[f32; 7]; 8],
    wet: bool,
    link: LinkMode,
    totals: &mut [u64; 2],
) -> bool {
    let Some((_, width)) = support::native_bank_width() else {
        return false;
    };
    let lanes = width.lanes() as usize;
    let values: Vec<_> = (0..lanes)
        .map(|lane| {
            let row = table[lane % 8];
            let overrides: Vec<(usize, f32)> = row.iter().copied().enumerate().collect();
            support::values_with(&overrides)
        })
        .collect();
    let requests: Vec<_> = values
        .iter()
        .map(|values| {
            let mut request = support::request(values);
            request.link_mode = link;
            request
        })
        .collect();
    let sizes = support::prepare(support::request(&values[0]));
    let mut dual = support::bind_bank(&requests).expect("dual bank");
    let mut mono = support::bind_bank(&requests).expect("collapsed bank");
    let mut first = 0_u64;
    for block in 0..128 {
        if block == 51 || block == 90 {
            dual.reset(ResetKind::DiscontinuityKeepParameters);
            mono.reset(ResetKind::DiscontinuityKeepParameters);
        }
        if block == 70 {
            for bank in [dual.as_mut(), mono.as_mut()] {
                let (mut left, right) = support::snapshot_track(bank, 0, sizes.as_ref());
                left[4..8].copy_from_slice(&(-25.0_f32).to_le_bytes());
                left[8..12].copy_from_slice(&(-15.0_f32).to_le_bytes());
                left[12..16].copy_from_slice(&0_u32.to_le_bytes());
                support::restore_track(bank, 0, 1, &left, &right, sizes.as_ref())
                    .expect("a legal payload");
            }
        }
        let frames = FRAMES[block % FRAMES.len()];
        let (spans, offsets) = spans(block, lanes, wet, first);
        let mut left = input(block, lanes, frames, 0x1006);
        let mut right = input(block, lanes, frames, 0x6001);
        let mut plane = left.clone();
        let mut unused = vec![0.0_f32; frames * lanes];
        let report = render(
            dual.as_mut(),
            &mut left,
            &mut right,
            frames,
            width,
            first,
            &spans,
            &offsets,
            false,
        );
        let report_mono = render(
            mono.as_mut(),
            &mut plane,
            &mut unused,
            frames,
            width,
            first,
            &spans,
            &offsets,
            true,
        );
        for words in [&left, &right, &plane] {
            for word in words.iter() {
                hasher.update(word.to_bits().to_le_bytes());
            }
        }
        fold_report(hasher, &report, lanes, totals);
        fold_report(hasher, &report_mono, lanes, totals);
        for bank in [dual.as_ref(), mono.as_ref()] {
            for track in 0..lanes as u32 {
                let (left, right) = support::snapshot_track(bank, track, sizes.as_ref());
                hasher.update(&left);
                hasher.update(&right);
            }
        }
        first += frames as u64;
    }
    true
}

/// Pinned on the unmodified batch head (`081fdc6c`), in dev and release, at eight lanes.
const SCENARIO_1006_BANK_W8: &str =
    "61ecffd0f28e6100a8501b0a77f9b10492a78ac5a3e6644aa71d0ac0026e54be";

#[test]
fn the_ramping_prefix_renders_the_pinned_bank_scenario() {
    let mut hasher = Sha256::new();
    let mut ran = false;
    let mut totals = [0_u64; 2];
    for link in [LinkMode::DualMono, LinkMode::Maximum] {
        for (table, wet) in [(&MIXED, false), (&WET, true)] {
            ran |= scenario(&mut hasher, table, wet, link, &mut totals);
        }
    }
    if !ran {
        return;
    }
    assert_eq!(totals[0], 0, "every span must be admitted");
    assert!(
        totals[1] > 0,
        "the hostile input must reach the boundary check"
    );
    let digest: String = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    println!("scenario 1006 bank digest {digest}");
    let width = support::native_bank_width().map(|(_, width)| width.lanes());
    if width == Some(8) {
        assert_eq!(digest, SCENARIO_1006_BANK_W8);
    }
}
