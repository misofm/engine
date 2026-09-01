//! E9 — non-finite values are caught once per block at the output boundary, not per sample.
//!
//! Master plan D7 and section 4.4. The pre-audit crate sanitised every input sample, checked every
//! intermediate for finiteness and had a `recover` path that reset `G` and emitted the delayed
//! sample; the counters counted *samples*. There is now one vector scan of the finished block per
//! channel, `bank::check_block`, and the counters count *blocks*.
//!
//! The channels are checked independently, because this effect keeps a separate ring, cursor and
//! recursive word per channel: a `DualMono` instance whose right channel diverged has a left
//! channel that is still exactly correct.

mod support;

use effect_contract::{EffectProcessBlock, PreparedSidechainPort};
use effect_runtime::state_payload::read_f32;

use support::{
    STATE_HEADER_WORDS, noise, prepare, render_scalar, request, sidechain_port, snapshot,
    values_with,
};

/// A NaN in the right input leaves the left channel bit-identical to a clean run, and trips the
/// boundary counter exactly once — in the block where the NaN reaches the output, not in the block
/// where it entered.
///
/// The latency is 960 samples and the quantum is 128, so a NaN at sample 0 emerges in block 7.
/// That gap is the whole difference between a per-sample sanitiser and a boundary check.
///
/// Red mutation (MUTATIONS.md row 17): stop zeroing and resetting the failing channel.
#[test]
fn a_nan_is_caught_at_the_block_boundary_not_per_sample() {
    let values = values_with(&[(0, -30.0), (7, 0.0)]);
    let clean_left = noise(2_048, 0x7A_70_00_01, 0.6);
    let clean_right = noise(2_048, 0x7A_70_00_02, 0.6);

    let mut clean = prepare(request(&values));
    let mut reference_left = clean_left.clone();
    let mut reference_right = clean_right.clone();
    render_scalar(
        clean.as_mut(),
        &mut reference_left,
        &mut reference_right,
        128,
        128,
        &[],
    );

    let mut effect = prepare(request(&values));
    let mut left = clean_left.clone();
    let mut right = clean_right.clone();
    right[0] = f32::NAN;
    let mut blocks = Vec::new();
    let mut state_after_the_rejected_block = None;
    let mut offset = 0;
    while offset < left.len() {
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                None,
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        blocks.push(report);
        offset += 128;
        if blocks.len() == 8 {
            state_after_the_rejected_block = Some(snapshot(effect.as_ref()));
        }
    }

    // Exactly one block was rejected, and it is block 7 — where sample 960 leaves the latency.
    let tripped: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, report)| report.nonfinite_right_blocks != 0)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(tripped, vec![7], "one rejected block, at the latency");
    assert_eq!(
        blocks[7].nonfinite_right_blocks, 1,
        "the counter counts blocks"
    );
    assert!(
        blocks
            .iter()
            .all(|report| report.nonfinite_left_blocks == 0),
        "the left channel never tripped"
    );
    assert!(
        blocks
            .iter()
            .all(|report| report.sanitized_main_samples == 0
                && report.sanitized_sidechain_samples == 0),
        "D7: nothing is sanitised per value any more"
    );

    // The left channel is bit-identical to the clean run, throughout.
    assert_eq!(
        left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        reference_left
            .iter()
            .map(|s| s.to_bits())
            .collect::<Vec<_>>(),
        "a right-channel divergence must not touch the left channel"
    );

    // The rejected block is zeroed, and the right channel's state is back to default.
    assert!(
        right[7 * 128..8 * 128].iter().all(|s| s.to_bits() == 0),
        "a rejected block is zeroed"
    );
    let (_, right_state) = state_after_the_rejected_block.expect("snapshot after block 7");
    assert_eq!(read_f32(&right_state, 2).to_bits(), 0.0_f32.to_bits(), "G");
    assert!(
        right_state[STATE_HEADER_WORDS * 4..]
            .chunks_exact(4)
            .take(128)
            .all(|word| word == 0.0_f32.to_le_bytes()),
        "the rings of the failing channel were cleared"
    );
}

/// A NaN that reaches only the **detector** is clamped to the level floor, by design.
///
/// A detector is a measurement, and a measurement that came back as nonsense produces "no gain
/// reduction" rather than poisoning a recursive word for the rest of the session; a NaN in the
/// **signal** path is a different matter and is caught at the block boundary by the test above.
///
/// Two clamps make this so, and either alone would: `Lane::max(a, b)` is `select(a > b, a, b)`, so
/// `detected.max(level_floor)` returns `level_floor` on an unordered pair, and `log2_lane` clamps
/// its own argument up to `f32::MIN_POSITIVE` the same way. Swapping the operands of the first
/// clamp is therefore an **equivalent** mutation, recorded as such in `tests/MUTATIONS.md`; what
/// this test pins is the resulting behaviour, which is what a caller can observe.
#[test]
fn a_nan_in_the_sidechain_alone_is_clamped_to_the_level_floor() {
    let values = values_with(&[(0, -30.0), (6, 1.0), (7, 0.0)]);
    let mut preparation = request(&values);
    preparation.ports.sidechain = PreparedSidechainPort::Connected {
        id: sidechain_port(),
        required: false,
    };
    let mut effect = prepare(preparation);

    let mut left = vec![0.5_f32; 2_048];
    let mut right = vec![0.5_f32; 2_048];
    let mut sidechain = vec![0.0_f32; 2_048];
    sidechain[0] = f32::NAN;

    let mut trips = 0_u64;
    let mut offset = 0;
    while offset < left.len() {
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                Some((
                    &sidechain[offset..offset + 128],
                    &sidechain[offset..offset + 128],
                )),
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        trips += report.nonfinite_left_blocks + report.nonfinite_right_blocks;
        offset += 128;
    }
    assert_eq!(trips, 0, "a NaN detector level is clamped, not propagated");
    // Sample 960 is the frame whose detector read the NaN. A silent detector means no gain
    // reduction at all, so the dry signal comes through exactly.
    assert_eq!(left[960].to_bits(), 0.5_f32.to_bits());
    assert_eq!(left[1_500].to_bits(), 0.5_f32.to_bits());
}

/// A value at or above the section 4.4 magnitude limit is rejected even though it is finite.
#[test]
fn the_boundary_limit_rejects_a_finite_but_absurd_value() {
    let values = values_with(&[(0, 0.0), (1, 1.0), (5, 0.0), (6, 0.5), (7, 0.0)]);
    let mut effect = prepare(request(&values));
    let mut left = vec![0.0_f32; 2_048];
    let mut right = vec![0.0_f32; 2_048];
    // Ratio 1 and makeup 0 make this the identity path, so the input reaches the output unchanged
    // and the only thing under test is the limit itself.
    left[0] = 1.0e31;
    let mut trips = 0_u64;
    let mut offset = 0;
    while offset < left.len() {
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                None,
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        trips += report.nonfinite_left_blocks;
        offset += 128;
    }
    assert_eq!(trips, 1, "1e31 is above the 1e30 boundary limit");

    // And a value just below it is not rejected.
    let mut effect = prepare(request(&values));
    let mut left = vec![0.0_f32; 2_048];
    let mut right = vec![0.0_f32; 2_048];
    left[0] = 9.9e29;
    let mut trips = 0_u64;
    let mut offset = 0;
    while offset < left.len() {
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                None,
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        trips += report.nonfinite_left_blocks;
        offset += 128;
    }
    assert_eq!(trips, 0, "9.9e29 is below the limit and passes through");
}

/// The recursive word flushes to exactly `+0.0`, which is what makes a settled compressor an exact
/// identity again.
///
/// D7: `flush` is applied to `g` and to nothing else. `flush(x)` is `+0.0` below `1e-20`, so a
/// gain reduction releasing toward zero arrives at **exactly** zero after a bounded number of
/// samples instead of asymptoting through the subnormals. Once it does, `G == 0 && makeup == +0`
/// makes the stage the dry identity, bit for bit.
///
/// The window matters: with a 5 ms release at 48 kHz the coefficient is about `4.2e-3`, so `G`
/// crosses `1e-20` after roughly 11,600 samples and would not underflow to zero on its own until
/// roughly 25,400. This test looks in between.
///
/// Red mutation (MUTATIONS.md row 2): drop the `flush` — RED, `G` is still a tiny non-zero number
/// and the identity never engages.
#[test]
fn the_recursive_word_flushes_to_exactly_zero() {
    let values = values_with(&[
        (0, -18.0),
        (1, 8.0),
        (2, 0.0),
        (3, 0.1),
        (4, 5.0),
        (5, 0.0),
        (6, 0.5),
        (7, 0.0),
    ]);
    let mut effect = prepare(request(&values));

    // A loud burst drives the envelope well below zero.
    let mut loud = vec![0.9_f32; 4_096];
    let mut loud_right = vec![0.9_f32; 4_096];
    render_scalar(effect.as_mut(), &mut loud, &mut loud_right, 128, 128, &[]);
    let after_burst = read_f32(&snapshot(effect.as_ref()).0, 2);
    assert!(
        after_burst < -5.0,
        "the burst must reduce gain: {after_burst}"
    );

    // Then silence, for long enough to cross the flush band but not to underflow on its own.
    let mut quiet = vec![0.0_f32; 16_384];
    let mut quiet_right = vec![0.0_f32; 16_384];
    render_scalar(effect.as_mut(), &mut quiet, &mut quiet_right, 128, 128, &[]);
    let settled = read_f32(&snapshot(effect.as_ref()).0, 2);
    assert_eq!(
        settled.to_bits(),
        0.0_f32.to_bits(),
        "G must flush to exactly +0.0, is {settled:e}"
    );

    // And the stage is now the exact dry identity, at a mix strictly between the ends.
    let signal = noise(2_048, 0x7A_70_00_03, 0.05);
    let mut left = signal.clone();
    let mut right = signal.clone();
    render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
    for index in 960..2_048 {
        assert_eq!(left[index].to_bits(), signal[index - 960].to_bits());
    }
}
