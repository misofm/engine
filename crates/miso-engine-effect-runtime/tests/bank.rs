//! The block boundary check (master plan §4.4) and the homogeneous-bank driver.

use miso_engine_effect_runtime::bank::{
    BLOCK_LIMIT, BankKernel, HomogeneousBank, NonFiniteReport, check_block, finish_block,
    finish_channel, nonfinite_lane_mask,
};
use miso_engine_lane::{Lane, Simd4, Simd8};

/// A finite block is accepted and left alone.
#[test]
fn a_finite_block_is_accepted() {
    let block: Vec<f32> = (0..512).map(|i| (i as f32) * 0.001 - 0.25).collect();
    assert!(check_block::<f32>(&block));
    assert!(check_block::<Simd4>(&block));
    assert!(check_block::<Simd8>(&block));
    assert_eq!(nonfinite_lane_mask::<Simd4>(&block), 0);
}

/// Every value the check must reject, and the threshold it must reject them at.
///
/// Red mutations, both recorded:
/// * `BLOCK_LIMIT` raised to `1e40` — which is `inf` in `f32`, so only infinities are caught and
///   the `1e30`/`1e31` rows go green-to-red.
/// * `mask_not(abs < limit)` replaced by `abs >= limit` — the NaN rows are then missed, because an
///   ordered compare against NaN is false in both directions.
#[test]
fn the_check_rejects_at_the_threshold() {
    for (value, accepted) in [
        (0.0f32, true),
        (-0.0, true),
        (1.0, true),
        (-1.0, true),
        (1e29, true),
        (-1e29, true),
        (f32::from_bits(BLOCK_LIMIT.to_bits() - 1), true),
        (BLOCK_LIMIT, false),
        (-BLOCK_LIMIT, false),
        (1e31, false),
        (-1e31, false),
        (f32::INFINITY, false),
        (f32::NEG_INFINITY, false),
        (f32::NAN, false),
        (-f32::NAN, false),
        (f32::MAX, false),
        (f32::MIN_POSITIVE, true),
        (f32::from_bits(1), true),
    ] {
        let mut block = vec![0.0f32; 64];
        block[37] = value;
        assert_eq!(
            check_block::<f32>(&block),
            accepted,
            "scalar: {value} ({:#010x})",
            value.to_bits()
        );
        assert_eq!(check_block::<Simd4>(&block), accepted, "Simd4: {value}");
        assert_eq!(check_block::<Simd8>(&block), accepted, "Simd8: {value}");
    }
}

/// The failing-lane bitmask attributes a rejection to the track that caused it.
#[test]
fn the_lane_mask_names_the_failing_lanes() {
    let mut block = vec![0.0f32; 4 * 16];
    block[4 * 3 + 1] = f32::NAN;
    block[4 * 9 + 2] = 1e31;
    assert_eq!(nonfinite_lane_mask::<Simd4>(&block), 0b0110);
    assert_eq!(
        nonfinite_lane_mask::<f32>(&block),
        0b1,
        "at W = 1 there is one lane"
    );

    let mut wide = vec![0.0f32; 8 * 8];
    wide[7] = f32::INFINITY;
    assert_eq!(nonfinite_lane_mask::<Simd8>(&wide), 0b1000_0000);
}

/// A rejected block is zeroed on both channels, the reset runs, and the counter advances.
#[test]
fn a_rejected_block_is_zeroed_reset_and_counted() {
    let mut report = NonFiniteReport::new();
    assert!(!report.tripped());

    let mut left = vec![1.0f32; 32];
    let mut right = vec![2.0f32; 32];
    let mut reset_calls = 0u32;
    assert!(finish_block::<Simd4>(
        &mut left,
        &mut right,
        &mut report,
        || reset_calls += 1
    ));
    assert_eq!(reset_calls, 0, "a clean block must not reset");
    assert!(left.iter().all(|v| *v == 1.0), "a clean block is untouched");
    assert_eq!(report.nonfinite_blocks, 0);

    left[13] = f32::NAN;
    assert!(!finish_block::<Simd4>(
        &mut left,
        &mut right,
        &mut report,
        || reset_calls += 1
    ));
    assert_eq!(reset_calls, 1);
    assert!(left.iter().all(|v| v.to_bits() == 0), "left must be zeroed");
    assert!(
        right.iter().all(|v| v.to_bits() == 0),
        "right must be zeroed with it"
    );
    assert_eq!(report.nonfinite_blocks, 1);
    assert_eq!(report.nonfinite_lanes, 1 << 1);
    assert!(report.tripped());
}

/// A failure on the right channel zeroes the left one too: the pair shares a reset.
#[test]
fn the_two_channels_fail_together() {
    let mut report = NonFiniteReport::new();
    let mut left = vec![0.5f32; 16];
    let mut right = vec![0.5f32; 16];
    right[3] = f32::INFINITY;
    assert!(!finish_block::<Simd4>(
        &mut left,
        &mut right,
        &mut report,
        || {}
    ));
    assert!(left.iter().all(|v| v.to_bits() == 0));
    assert_eq!(report.nonfinite_lanes, 1 << 3);
}

/// The counter counts blocks, not samples.
#[test]
fn the_counter_counts_blocks() {
    let mut report = NonFiniteReport::new();
    for _ in 0..7 {
        let mut left = vec![f32::NAN; 16];
        let mut right = vec![f32::NAN; 16];
        finish_block::<Simd4>(&mut left, &mut right, &mut report, || {});
    }
    assert_eq!(report.nonfinite_blocks, 7);
}

/// A one-pole slot, standing in for a real effect kernel.
struct OnePoleSlot;

#[derive(Clone, Copy)]
struct Coef<L: Lane> {
    c: L,
}

#[derive(Clone, Copy)]
struct State<L: Lane> {
    y: L,
}

impl<L: Lane> Default for State<L> {
    fn default() -> Self {
        Self { y: L::zero() }
    }
}

impl<L: Lane> BankKernel<L> for OnePoleSlot {
    type Coef = Coef<L>;
    type State = State<L>;

    fn identity_coef() -> Self::Coef {
        // A negative coefficient is this test kernel's inactive marker: the block body selects the
        // input straight through for those lanes, which is an exact identity rather than a
        // near-identity. A real effect's identity coefficients do the same job (an SVF's
        // `(m0, m1, m2) = (1, 0, 0)`, a gain's `1.0`).
        Coef { c: L::splat(-1.0) }
    }

    fn process_block(io: &mut [f32], frames: usize, coef: &Self::Coef, state: &mut Self::State) {
        let mut y = state.y;
        let identity = coef.c.lt(L::zero());
        for frame in io.chunks_exact_mut(L::WIDTH) {
            let x = L::load(frame);
            y = coef.c.fma(x.sub(y), y);
            L::select(identity, x, y).store(frame);
        }
        state.y = y;
        let _ = frames;
    }
}

/// A prepared bank allocates once, and an inactive slot is an exact identity.
#[test]
fn an_identity_slot_passes_the_block_through_unchanged() {
    let mut bank = HomogeneousBank::<Simd4, OnePoleSlot>::prepare(3);
    assert_eq!(bank.slots(), 3);
    let source: Vec<f32> = (0..4 * 32).map(|i| (i as f32) * 0.01 - 0.6).collect();
    let mut left = source.clone();
    let mut right = source.clone();
    assert!(bank.process_block(&mut left, &mut right, 32));
    for (index, (a, b)) in left.iter().zip(&source).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "sample {index}");
    }
    assert_eq!(bank.report().nonfinite_blocks, 0);
}

/// A diverging slot trips the check, the block is zeroed and the bank's state is reset.
#[test]
fn a_diverging_bank_is_caught_and_reset() {
    let mut bank = HomogeneousBank::<Simd4, OnePoleSlot>::prepare(1);
    // c = 4 makes `y += 4 * (x - y)` an unstable recurrence.
    *bank.coefficients_mut(0) = Coef {
        c: Simd4::splat(4.0),
    };
    let mut left = vec![1.0f32; 4 * 64];
    let mut right = vec![1.0f32; 4 * 64];
    let mut blocks = 0;
    for _ in 0..40 {
        if !bank.process_block(&mut left, &mut right, 64) {
            blocks += 1;
            break;
        }
        left.fill(1.0);
        right.fill(1.0);
    }
    assert_eq!(blocks, 1, "the divergence must be caught");
    assert!(left.iter().all(|v| v.to_bits() == 0));
    assert_eq!(bank.report().nonfinite_blocks, 1);
    assert_eq!(bank.report().nonfinite_lanes, 0b1111);

    // After the reset the bank starts from silence again.
    let mut fresh = vec![0.0f32; 4 * 64];
    let mut fresh_right = vec![0.0f32; 4 * 64];
    assert!(bank.process_block(&mut fresh, &mut fresh_right, 64));
    assert!(fresh.iter().all(|v| v.to_bits() == 0));
}

/// `reset` clears state without touching coefficients.
#[test]
fn reset_clears_state_and_keeps_coefficients() {
    let mut bank = HomogeneousBank::<f32, OnePoleSlot>::prepare(1);
    *bank.coefficients_mut(0) = Coef { c: 0.5f32 };
    let mut left = vec![1.0f32; 8];
    let mut right = vec![1.0f32; 8];
    assert!(bank.process_block(&mut left, &mut right, 8));
    let settled = left[7];
    assert!(settled > 0.9, "the follower should have risen: {settled}");

    bank.reset();
    let mut again = vec![1.0f32; 8];
    let mut again_right = vec![1.0f32; 8];
    assert!(bank.process_block(&mut again, &mut again_right, 8));
    for (index, (a, b)) in again.iter().zip(&left).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "sample {index} after reset");
    }
    assert_eq!(bank.coefficients_mut(0).c.to_bits(), 0.5f32.to_bits());
}

/// `finish_channel` applies the section 4.4 policy to one channel and names the failing lanes.
///
/// Red mutation: return `0` after a failing block instead of the lane mask — RED, the caller can
/// no longer attribute a rejected block to a track. Recorded in `tests/MUTATIONS.md`.
#[test]
fn finish_channel_is_per_channel_and_reports_its_lanes() {
    type L = miso_engine_lane::Simd4;

    // A clean block is left exactly alone and no reset runs.
    let mut clean = vec![0.25f32; 4 * 16];
    let mut reset_ran = false;
    let mask = finish_channel::<L>(&mut clean, || reset_ran = true);
    assert_eq!(mask, 0);
    assert!(!reset_ran);
    assert!(clean.iter().all(|v| v.to_bits() == 0.25f32.to_bits()));

    // A NaN in lane 2 and an out-of-bounds magnitude in lane 0 name exactly those two lanes.
    let mut failing = vec![0.25f32; 4 * 16];
    failing[3 * 4 + 2] = f32::NAN;
    failing[7 * 4] = 1.0e30;
    let mut reset_ran = false;
    let mask = finish_channel::<L>(&mut failing, || reset_ran = true);
    assert_eq!(mask, 0b0101);
    assert!(reset_ran, "a rejected block resets the channel");
    assert!(failing.iter().all(|v| v.to_bits() == 0));

    // The other channel of the same effect is untouched: this is the whole point of the per-channel
    // form. Nothing links the two calls.
    let mut other = vec![-0.5f32; 4 * 16];
    let mask = finish_channel::<L>(&mut other, || panic!("the clean channel must not reset"));
    assert_eq!(mask, 0);
    assert!(other.iter().all(|v| v.to_bits() == (-0.5f32).to_bits()));

    // The threshold is the same one `finish_block` uses.
    let mut edge = vec![0.0f32; 4 * 4];
    edge[1] = f32::from_bits(1.0e30f32.to_bits() - 1);
    assert_eq!(finish_channel::<L>(&mut edge, || {}), 0);
    edge[1] = 1.0e30;
    assert_eq!(finish_channel::<L>(&mut edge, || {}), 0b0010);
}
