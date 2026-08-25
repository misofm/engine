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
    PrepareEffectBankRequest, PreparedAutomationSpan,
};
use miso_engine_lane::Backend;
use miso_engine_parametric_eq::ParametricEqFactory;
use support::{point, request, set_initial, values};

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
