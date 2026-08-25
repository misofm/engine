//! Issue #144 item 6 at the effect boundary: a redundant automation point is a no-op.
//!
//! The hoist settles a lane whose six designed words already agree bitwise instead of arming a
//! sixty-four-sample window of `+0.0` increments. The property that makes that class A is stated
//! here at the level a session actually observes: **restating a parameter at the value it already
//! holds must not move a single rendered bit**, against the same bank rendered with no automation
//! at all.
//!
//! A hoist that fired on a value that is genuinely moving would fail
//! `a_real_automation_point_still_ramps`; a hoist that mishandled the settle would fail the
//! redundant case.

mod support;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedAutomationSpan, StatePayloadOutput, StatePayloadSizes,
};
use miso_engine_lane::Backend;
use miso_engine_parametric_eq::ParametricEqFactory;
use support::{COMMON_BYTES, LANE_BYTES, point, request, set_initial, values, word};

const FRAMES: usize = 128;
const BLOCKS: usize = 4;

/// Band 0's left gain for `track`, in the domain the prepared values use.
fn band0_left_gain(track: usize) -> f32 {
    -9.0 + track as f32
}

fn configured(track: usize) -> Vec<miso_engine_effect_contract::InitialParameterValue> {
    let mut prepared = values();
    set_initial(&mut prepared, 0, ParameterChannel::Left, 1.0);
    set_initial(&mut prepared, 0, ParameterChannel::Right, 1.0);
    set_initial(&mut prepared, 1, ParameterChannel::Left, 3.0);
    set_initial(&mut prepared, 1, ParameterChannel::Right, 3.0);
    set_initial(
        &mut prepared,
        2,
        ParameterChannel::Left,
        400.0 + track as f32 * 37.0,
    );
    set_initial(
        &mut prepared,
        2,
        ParameterChannel::Right,
        1500.0 + track as f32 * 53.0,
    );
    set_initial(
        &mut prepared,
        3,
        ParameterChannel::Left,
        band0_left_gain(track),
    );
    set_initial(&mut prepared, 4, ParameterChannel::Left, 0.9);
    prepared
}

fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

/// Renders `BLOCKS` blocks, delivering `spans` on the first block only, and returns output bits.
fn render(spans: &[PreparedAutomationSpan], offsets: &[u32], lanes: usize) -> Vec<u32> {
    let (width, backend) = native_bank().expect("a native bank width");
    let factory = ParametricEqFactory;
    let prepared: Vec<_> = (0..lanes).map(configured).collect();
    let requests: Vec<_> = prepared
        .iter()
        .map(|values| request(values, false))
        .collect();
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("valid bank request")
        .expect("the native width must bind");

    let source: Vec<f32> = (0..FRAMES * lanes)
        .map(|index| ((index as f32) * 0.017).sin() * 0.4)
        .collect();
    let mut left = source.clone();
    let mut right: Vec<f32> = source.iter().map(|value| -value).collect();
    let empty_offsets = vec![0_u32; lanes + 1];

    let mut bits = Vec::with_capacity(FRAMES * lanes * 2 * BLOCKS);
    for block in 0..BLOCKS {
        let (block_spans, block_offsets): (&[PreparedAutomationSpan], &[u32]) = if block == 0 {
            (spans, offsets)
        } else {
            (&[], &empty_offsets)
        };
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                width,
                (block * FRAMES) as u64,
                block_spans,
                block_offsets,
                128,
            )
            .expect("bank block"),
        );
        bits.extend(left.iter().map(|value| value.to_bits()));
        bits.extend(right.iter().map(|value| value.to_bits()));
    }
    bits
}

/// **Restating a parameter at the value it already holds must not move a rendered bit.**
#[test]
fn a_redundant_automation_point_renders_exactly_the_unautomated_bank() {
    let Some((width, _)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let empty_offsets = vec![0_u32; lanes + 1];

    let redundant: Vec<PreparedAutomationSpan> = (0..lanes)
        .map(|track| point(3, ParameterChannel::Left, 0, band0_left_gain(track)))
        .collect();
    let offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();

    let quiet = render(&[], &empty_offsets, lanes);
    let restated = render(&redundant, &offsets, lanes);

    assert_eq!(
        quiet.len(),
        restated.len(),
        "both arms must render the same shape"
    );
    assert_eq!(
        quiet, restated,
        "a redundant automation point moved rendered bits"
    );
}

/// The guard on the other side: a point that genuinely moves must still ramp and must differ.
#[test]
fn a_real_automation_point_still_ramps() {
    let Some((width, _)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let empty_offsets = vec![0_u32; lanes + 1];

    // One ULP away from the prepared value: the smallest move the bit compare must still respect.
    let moved: Vec<PreparedAutomationSpan> = (0..lanes)
        .map(|track| {
            let held = band0_left_gain(track);
            point(
                3,
                ParameterChannel::Left,
                0,
                f32::from_bits(held.to_bits() + 1),
            )
        })
        .collect();
    let offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();

    let quiet = render(&[], &empty_offsets, lanes);
    let nudged = render(&moved, &offsets, lanes);

    assert_ne!(
        quiet, nudged,
        "a one-ULP automation point must still be a real retarget"
    );
}

/// The re-preparation half: a restated band must not be redesigned, and must still render exactly.
///
/// The cached read is only sound because `BandTarget::words` is a pure function of the band and
/// the sample rate, so the words held in `Section::target` are the words the design would return.
/// This test states the consequence a caller can observe: restating a band is indistinguishable
/// from not automating it at all, block after block, for a long enough run that a drifting cache
/// would show up.
#[test]
fn a_restated_band_is_indistinguishable_from_no_automation_over_many_blocks() {
    let Some((width, _)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let empty_offsets = vec![0_u32; lanes + 1];
    let redundant: Vec<PreparedAutomationSpan> = (0..lanes)
        .map(|track| point(3, ParameterChannel::Left, 0, band0_left_gain(track)))
        .collect();
    let offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();

    // Delivered on the first block only (the render helper's contract), but the comparison runs
    // for BLOCKS blocks so a cached word that had drifted would have time to show.
    assert_eq!(
        render(&[], &empty_offsets, lanes),
        render(&redundant, &offsets, lanes),
        "a restated band diverged from an unautomated one"
    );
}

/// The cached read must return the **designed** words, not the words currently in force.
///
/// Automation points land only at a block's first sample, but `frames` may be as short as one
/// sample, so a sixty-four-sample word ramp legitimately spans block boundaries — and a
/// restatement in the next block then takes the cached-design path while `Section::coef` and
/// `Section::target` disagree. That is the one window where reading the wrong side of the lane
/// is observable: a readback of the in-flight words would let `start_ramp` see a "stationary"
/// lane and settle it at mid-ramp coefficients, and the bank would hold the wrong design
/// forever. The restatement restarts the ramp, so the first blocks differ by design; where the
/// lane **ends up** must not depend on whether the target was restated on the way there.
#[test]
fn a_band_restated_mid_flight_still_settles_at_the_designed_words() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    const SHORT_FRAMES: usize = 32;
    const SHORT_BLOCKS: usize = 8;

    // A real one-ULP retarget; its sixty-four-sample ramp is still in flight one short block in.
    let retargeted = |track: usize| {
        let held = band0_left_gain(track);
        f32::from_bits(held.to_bits() + 1)
    };

    // Renders SHORT_BLOCKS thirty-two-frame blocks, delivering the retarget on block zero and,
    // when asked, a restatement of the same value on block one, and returns output bits.
    let render_short = |restate: bool| -> Vec<([u8; LANE_BYTES], [u8; LANE_BYTES])> {
        let factory = ParametricEqFactory;
        let prepared: Vec<_> = (0..lanes).map(configured).collect();
        let requests: Vec<_> = prepared
            .iter()
            .map(|values| request(values, false))
            .collect();
        let mut bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("valid bank request")
            .expect("the native width must bind");

        let source: Vec<f32> = (0..SHORT_FRAMES * lanes)
            .map(|index| ((index as f32) * 0.017).sin() * 0.4)
            .collect();
        let mut left = source.clone();
        let mut right: Vec<f32> = source.iter().map(|value| -value).collect();
        let empty_offsets = vec![0_u32; lanes + 1];
        let span_offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();

        for block in 0..SHORT_BLOCKS {
            let first_sample = (block * SHORT_FRAMES) as u64;
            let spans: Vec<PreparedAutomationSpan> = if block == 0 || (block == 1 && restate) {
                (0..lanes)
                    .map(|track| point(3, ParameterChannel::Left, first_sample, retargeted(track)))
                    .collect()
            } else {
                Vec::new()
            };
            let offsets = if spans.is_empty() {
                &empty_offsets
            } else {
                &span_offsets
            };
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    SHORT_FRAMES as u32,
                    width,
                    first_sample,
                    &spans,
                    offsets,
                    128,
                )
                .expect("bank block"),
            );
        }
        (0..lanes)
            .map(|track| {
                let mut common = [0_u8; COMMON_BYTES];
                let mut lane_left = [0_u8; LANE_BYTES];
                let mut lane_right = [0_u8; LANE_BYTES];
                bank.snapshot_track_state_payload(
                    track as u32,
                    StatePayloadOutput::new(
                        &mut common,
                        &mut lane_left,
                        &mut lane_right,
                        StatePayloadSizes {
                            common_bytes: COMMON_BYTES as u32,
                            left_bytes: LANE_BYTES as u32,
                            right_bytes: LANE_BYTES as u32,
                        },
                    )
                    .expect("state output"),
                )
                .expect("snapshot");
                (lane_left, lane_right)
            })
            .collect()
    };

    // The integrators are legitimately path-dependent (the restatement restarts the ramp), so
    // the comparison is the rest of the settled state: coefficients, steps, remaining, targets.
    // A readback of in-flight words would settle the restated arm's coefficients at mid-ramp
    // values, and they would stay there.
    const WORDS_PER_BAND: usize = 19;
    let once = render_short(false);
    let restated = render_short(true);
    for track in 0..lanes {
        for (label, arm_once, arm_restated) in [
            ("left", &once[track].0, &restated[track].0),
            ("right", &once[track].1, &restated[track].1),
        ] {
            for band in 0..LANE_BYTES / 4 / WORDS_PER_BAND {
                let base = band * WORDS_PER_BAND;
                for index in 2..WORDS_PER_BAND {
                    assert_eq!(
                        word(arm_once, base + index),
                        word(arm_restated, base + index),
                        "track {track} {label} band {band} word {index}: \
                         a mid-flight restatement changed where the lane settled"
                    );
                }
            }
        }
    }
}
