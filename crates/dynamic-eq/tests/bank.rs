//! The bank kernel contract: every lane of a bank renders its scalar instance's bits, and a
//! collapsed bank renders a dual bank's on `L == R` (issue #973).

mod support;

use effect_contract::{LinkMode, PreparedAutomationSpan};
use support::{
    ATTACK, DIRECTION, ENABLED, FREQUENCY, GAIN, KNEE, NATIVE_LANES, Q, RATIO, RELEASE, THRESHOLD,
    Values, defaults, index, noise, prepare, prepare_bank, process, process_bank, set_band,
    snapshot_bank, spans_both,
};

const FRAMES: usize = 64;
const BLOCKS: usize = 40;
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

/// Lane `lane`'s prepared values: each lane its own bands, gains, thresholds and directions;
/// band 3 starts disabled on odd lanes, band 2 hears louder than its threshold on every lane.
fn lane_values(lane: usize) -> Values {
    let mut values = defaults();
    let l = lane as f32;
    for band in 0..4 {
        let b = band as f32;
        set_band(
            &mut values,
            band,
            FREQUENCY,
            150.0 * (1.0 + l) * (1.0 + 3.0 * b),
        );
        set_band(&mut values, band, Q, 0.5 + 0.7 * l);
        set_band(&mut values, band, GAIN, -6.0 + 3.0 * l - 2.0 * b);
        set_band(&mut values, band, THRESHOLD, -36.0 + 2.0 * l + 3.0 * b);
        set_band(&mut values, band, RATIO, 2.0 + l);
        set_band(
            &mut values,
            band,
            DIRECTION,
            if (lane + band).is_multiple_of(2) {
                1.0
            } else {
                2.0
            },
        );
        set_band(&mut values, band, KNEE, if band == 1 { 0.0 } else { 6.0 });
    }
    if lane % 2 == 1 {
        set_band(&mut values, 3, ENABLED, 0.0);
    }
    values
}

/// Block `step`'s spans for one lane: a band move and a gain change, then a band disabled, a
/// direction and a knee flipped, a band re-enabled, then the ballistics; each starts a 64-sample
/// ramp.
fn lane_spans(step: usize, lane: usize) -> Vec<PreparedAutomationSpan> {
    let sample = (step * FRAMES) as u64;
    let l = lane as f32;
    let points: Vec<(usize, f32)> = match step {
        3 => vec![
            (index(0, FREQUENCY), 300.0 + 100.0 * l),
            (index(0, GAIN), 9.0 - l),
        ],
        6 => vec![
            (index(1, ENABLED), 0.0),
            (
                index(2, DIRECTION),
                if lane.is_multiple_of(2) { 2.0 } else { 1.0 },
            ),
            (index(2, KNEE), 12.0),
        ],
        9 => vec![(index(1, ENABLED), 1.0), (index(3, ENABLED), 1.0)],
        11 => vec![
            (index(0, THRESHOLD), -45.0),
            (index(0, ATTACK), 0.3),
            (index(0, RELEASE), 300.0),
        ],
        _ => vec![],
    };
    let mut spans: Vec<_> = points
        .iter()
        .flat_map(|&(parameter, value)| spans_both(parameter as u32, value, sample))
        .collect();
    spans.sort_by_key(|span| (span.parameter_index, span.channel as u32));
    spans
}

/// Lane `lane`'s input word at `frame` on one channel; silence from block 30 on.
fn word(signals: &[Vec<f32>], lane: usize, channel: usize, frame: usize) -> f32 {
    if frame >= 30 * FRAMES {
        return 0.0;
    }
    signals[lane * 2 + channel][frame]
}

#[test]
fn every_bank_lane_renders_its_scalar_instance() {
    let signals: Vec<Vec<f32>> = (0..NATIVE_LANES * 2)
        .map(|seed| noise(0x17 + seed as u64, FRAMES * BLOCKS, 0.9))
        .collect();
    let values: Vec<Values> = (0..NATIVE_LANES).map(lane_values).collect();
    let links: Vec<LinkMode> = (0..NATIVE_LANES).map(|lane| LINKS[lane % 3]).collect();
    let mut bank = prepare_bank(&values, &links);
    let mut scalars: Vec<_> = (0..NATIVE_LANES)
        .map(|lane| prepare(&values[lane], links[lane]))
        .collect();
    for step in 0..BLOCKS {
        let mut planes = [
            vec![0.0_f32; FRAMES * NATIVE_LANES],
            vec![0.0; FRAMES * NATIVE_LANES],
        ];
        let (mut spans, mut offsets) = (Vec::new(), vec![0_u32]);
        for lane in 0..NATIVE_LANES {
            spans.extend(lane_spans(step, lane));
            offsets.push(spans.len() as u32);
            for (channel, plane) in planes.iter_mut().enumerate() {
                for frame in 0..FRAMES {
                    plane[frame * NATIVE_LANES + lane] =
                        word(&signals, lane, channel, step * FRAMES + frame);
                }
            }
        }
        let [mut left, mut right] = planes;
        let report = process_bank(
            &mut bank,
            &mut left,
            &mut right,
            (step * FRAMES) as u64,
            &spans,
            &offsets,
            false,
        );
        for lane in 0..NATIVE_LANES {
            let mut lane_left: Vec<f32> = (0..FRAMES)
                .map(|frame| word(&signals, lane, 0, step * FRAMES + frame))
                .collect();
            let mut lane_right: Vec<f32> = (0..FRAMES)
                .map(|frame| word(&signals, lane, 1, step * FRAMES + frame))
                .collect();
            let scalar = process(
                &mut scalars[lane],
                &mut lane_left,
                &mut lane_right,
                (step * FRAMES) as u64,
                &lane_spans(step, lane),
            );
            assert_eq!(
                report.reports[lane], scalar,
                "block {step} lane {lane}: report"
            );
            for frame in 0..FRAMES {
                for (bank_word, scalar_word, side) in [
                    (left[frame * NATIVE_LANES + lane], lane_left[frame], "left"),
                    (
                        right[frame * NATIVE_LANES + lane],
                        lane_right[frame],
                        "right",
                    ),
                ] {
                    assert_eq!(
                        bank_word.to_bits(),
                        scalar_word.to_bits(),
                        "block {step} lane {lane} frame {frame} {side}"
                    );
                }
            }
        }
    }
    assert_eq!(
        bank.processor.lanes_at_rest(),
        0,
        "the trace ends before any lane rests, so the silent path is the scalar's too"
    );
}

/// For every link mode, a bank collapsed for the first half and then desymmetrized renders, block
/// for block, a never-collapsed bank's left plane, reports and left state on `L == R` input, and
/// both planes and states after the disengage; the witness declines exactly an asymmetric lane.
#[test]
fn a_collapsed_then_desymmetrized_bank_is_a_never_collapsed_bank() {
    let signals: Vec<Vec<f32>> = (0..NATIVE_LANES * 2)
        .map(|seed| noise(0x973 + seed as u64, FRAMES * BLOCKS, 0.9))
        .collect();
    let values: Vec<Values> = (0..NATIVE_LANES).map(lane_values).collect();
    for link in LINKS {
        let links = vec![link; NATIVE_LANES];
        let mut mixed = prepare_bank(&values, &links);
        let mut never = prepare_bank(&values, &links);
        assert!(mixed.processor.supports_mono_collapse());
        assert!((0..NATIVE_LANES).all(|lane| mixed.processor.lane_channel_symmetry(lane)));
        for step in 0..BLOCKS {
            let collapsed = step < BLOCKS / 2;
            if step == BLOCKS / 2 {
                mixed.processor.desymmetrize_channels();
                assert!(mixed.processor.channels_agree(), "{link:?}: after the copy");
            }
            let (mut spans, mut offsets) = (Vec::new(), vec![0_u32]);
            let mut never_left = vec![0.0_f32; FRAMES * NATIVE_LANES];
            for lane in 0..NATIVE_LANES {
                spans.extend(lane_spans(step, lane));
                offsets.push(spans.len() as u32);
                for frame in 0..FRAMES {
                    never_left[frame * NATIVE_LANES + lane] =
                        word(&signals, lane, 0, step * FRAMES + frame);
                }
            }
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * NATIVE_LANES];
            let mut mixed_right = if collapsed {
                stale.clone()
            } else {
                never_left.clone()
            };
            let first = (step * FRAMES) as u64;
            let expected = process_bank(
                &mut never,
                &mut never_left,
                &mut never_right,
                first,
                &spans,
                &offsets,
                false,
            );
            let actual = process_bank(
                &mut mixed,
                &mut mixed_left,
                &mut mixed_right,
                first,
                &spans,
                &offsets,
                collapsed,
            );
            let context = format!("{link:?} block {step}");
            assert_eq!(actual, expected, "{context}: reports");
            let bits = |plane: &[f32]| plane.iter().map(|w| w.to_bits()).collect::<Vec<_>>();
            assert_eq!(
                bits(&mixed_left),
                bits(&never_left),
                "{context}: left plane"
            );
            let right = if collapsed { &stale } else { &never_right };
            assert_eq!(bits(&mixed_right), bits(right), "{context}: right plane");
            for track in 0..NATIVE_LANES as u32 {
                let (_, mixed_l, mixed_r) = snapshot_bank(&mixed, track);
                let (_, never_l, never_r) = snapshot_bank(&never, track);
                assert_eq!(mixed_l, never_l, "{context} track {track}: left state");
                if !collapsed {
                    assert_eq!(mixed_r, never_r, "{context} track {track}: right state");
                }
            }
        }
        let mut asymmetric = values.clone();
        asymmetric[1][index(2, THRESHOLD) * 2 + 1].value = -7.0;
        let bank = prepare_bank(&asymmetric, &links);
        for lane in 0..NATIVE_LANES {
            assert_eq!(
                bank.processor.lane_channel_symmetry(lane),
                lane != 1,
                "{link:?} lane {lane}"
            );
        }
    }
}
