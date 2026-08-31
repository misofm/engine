//! The D7 sanitisation counter: that its and-form is the select-form to the bit, and that what it
//! counts is what the policy says it counts.
//!
//! Candidate A1 replaced `count + select(bad, 1.0, 0.0)` with `count + (1.0 & bad)` at every copy
//! of the sanitise prologue — `sanitize_gain_block`, `input_chain_block`, and the two elision
//! variants. That is an **equivalence** under the canonical-mask contract, not a behaviour change,
//! and this file is deliberately built so that reverting A1 leaves every test in it green.
//!
//! Which raises the obvious question of what the file is for. Two things:
//!
//! * the equivalence is *asserted* here rather than argued — the select-form replicas below are the
//!   pre-A1 bodies, and they are run against the tree's kernels over hostile blocks with evolving
//!   state, so a target or a `Lane` backend on which `select` and the and-form disagree fails here
//!   rather than in a sealed digest six steps downstream;
//! * the counter's *contract* is pinned against an independent scalar oracle that re-derives what
//!   should have been counted from the policy — `|x| < 1e30` is clean, anything else is sanitised —
//!   rather than from either kernel. That is the half with teeth: it is what goes red if the
//!   at-limit case stops counting, or if a lane is miscounted, whichever form is in the tree.
//!
//! Clean-audio digests are blind to this path: a sanitised sample is `+0.0` in the output either
//! way, so only the counter can see a miscount. The qualification corpus embeds the counters in its
//! digest (`builtins`), which is the other end of the same rope.

use lane::kernels::builtins::{
    InputChainCoef, InputChainPlan, InputChainState, NONFINITE_LIMIT, input_chain_block,
    input_chain_block_elided, input_chain_plan, sanitize_gain_block,
};
use lane::kernels::{SvfCoef, svf_step};
use lane::{Lane, Simd4, Simd8};

/// Frames per block.
const FRAMES: usize = 128;

/// Blocks per case; the retained state evolves across all of them.
const BLOCKS: u32 = 64;

/// Widest bank under test.
const MAX_WIDTH: usize = 8;

// ---------------------------------------------------------------------------------------------
// The pre-A1 bodies, transcribed. Only step 3 differs from the tree's.
// ---------------------------------------------------------------------------------------------

/// `sanitize_gain_block` with the select-form counter.
fn sanitize_gain_block_select<L: Lane>(io: &mut [f32], frames: usize, gain: L) -> L {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut count = L::zero();
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(L::select(bad, one, zero));
        x.andnot(bad).mul(gain).store(frame);
    }
    count
}

/// `input_chain_block` with the select-form counter; every other operation is the tree's, in order.
fn input_chain_block_select<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> [L; 2] {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut count = [zero; 2];
    let mut state = s.section;
    let mut nc1 = [[zero; 2]; 2];
    for (channel, coefficients) in c.section.iter().enumerate() {
        for (section, coefficient) in coefficients.iter().enumerate() {
            nc1[channel][section] = coefficient.c1.neg();
        }
    }
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        for (channel, frame) in [left_frame, right_frame].into_iter().enumerate() {
            let x = L::load(frame);
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(L::select(bad, one, zero));
            let mut v = x.andnot(bad).mul(c.trim[channel]);
            for section in 0..2 {
                let coefficient = &c.section[channel][section];
                let v0 = v;
                let (v1, v2) = svf_step(
                    v0,
                    nc1[channel][section],
                    coefficient.a2,
                    coefficient.a3,
                    &mut state[channel][section],
                );
                v = coefficient
                    .m2
                    .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
            }
            v.store(frame);
        }
    }
    s.section = state;
    count
}

// ---------------------------------------------------------------------------------------------
// Fixtures.
// ---------------------------------------------------------------------------------------------

/// A hostile block: sanitised samples in *some* lanes on a lane- and frame-dependent pattern, so a
/// body that counted per block, or broadcast one lane's verdict, is caught.
///
/// The value classes are the ones the D7 boundary actually turns on. `+/-1e30` is **at** the limit
/// and must count, because the compare is `|x| < limit`; `9.999e29` is just below it and must not.
/// A NaN with a payload must count exactly like a default NaN, and a subnormal and `-0.0` must not
/// count at all.
fn fill_hostile(buffer: &mut [f32], seed: u32, width: usize) {
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(12_345);
    for (index, sample) in buffer.iter_mut().enumerate() {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        *sample = ((state >> 8) as f32 / (1_u32 << 24) as f32) * 1.6 - 0.8;
        let (frame, lane) = (index / width, index % width);
        match (frame + lane * 7) % 23 {
            0 if lane % 3 == 0 => *sample = f32::NAN,
            1 if lane % 3 == 1 => *sample = f32::INFINITY,
            2 if lane % 3 == 2 => *sample = f32::NEG_INFINITY,
            // Exactly the limit, both signs: sanitised.
            3 if lane % 2 == 0 => *sample = 1.0e30,
            4 if lane % 2 == 1 => *sample = -1.0e30,
            // A NaN carrying a payload, and a signalling-pattern NaN: both sanitised.
            5 => *sample = f32::from_bits(0x7FC0_1234),
            6 if lane % 4 == 3 => *sample = f32::from_bits(0x7F80_0001),
            // Clean controls, immediately below the limit and at the bottom of the range.
            7 if lane == 0 => *sample = 9.999e29,
            8 if lane == 1 => *sample = -9.999e29,
            9 => *sample = f32::from_bits(0x0000_0001),
            10 => *sample = -0.0,
            _ => {}
        }
    }
}

/// The policy, restated in scalar `f32` and owned by neither kernel: a sample is sanitised exactly
/// when `|x| < NONFINITE_LIMIT` is false.
fn oracle_counts(buffer: &[f32], width: usize) -> Vec<u32> {
    let mut counts = vec![0_u32; width];
    for (index, sample) in buffer.iter().enumerate() {
        // Written through `partial_cmp` on purpose: the D7 verdict is "the ordered `<` did not
        // hold", and NaN is sanitised precisely because it compares unordered. A `>=` would read
        // as the same rule and would not be.
        let clean = matches!(
            sample.abs().partial_cmp(&NONFINITE_LIMIT),
            Some(core::cmp::Ordering::Less)
        );
        if !clean {
            counts[index % width] += 1;
        }
    }
    counts
}

/// A real Butterworth design per lane, so the recurrence is live under the counter.
fn design(rate: f64, cutoff: f64, high_pass: bool) -> [f32; 6] {
    let k = core::f64::consts::SQRT_2;
    let g = (core::f64::consts::PI * cutoff / rate).tan();
    let t1 = g * (g + k);
    let denominator = 1.0 + t1;
    let (m0, m1, m2) = if high_pass {
        (1.0, -k, -1.0)
    } else {
        (0.0, 0.0, 1.0)
    };
    [
        (t1 / denominator) as f32,
        (g / denominator) as f32,
        (g * g / denominator) as f32,
        m0 as f32,
        m1 as f32,
        m2 as f32,
    ]
}

fn section<L: Lane>(cutoff: f64, high_pass: bool) -> SvfCoef<L> {
    let mut words = [[0.0_f32; MAX_WIDTH]; 6];
    for lane in 0..L::WIDTH {
        let design = design(48_000.0, cutoff * (1.0 + lane as f64 * 0.03), high_pass);
        for (word, value) in words.iter_mut().zip(design) {
            word[lane] = value;
        }
    }
    SvfCoef {
        c1: L::load(&words[0]),
        a2: L::load(&words[1]),
        a3: L::load(&words[2]),
        m0: L::load(&words[3]),
        m1: L::load(&words[4]),
        m2: L::load(&words[5]),
    }
}

fn chain_coef<L: Lane>() -> InputChainCoef<L> {
    let trim = |base: f32| -> L {
        let mut words = [0.0_f32; MAX_WIDTH];
        for (lane, word) in words.iter_mut().enumerate() {
            *word = base + lane as f32 * 0.01;
        }
        L::load(&words)
    };
    InputChainCoef {
        trim: [trim(0.9), trim(0.95)],
        section: [
            [section::<L>(45.0, true), section::<L>(17_800.0, false)],
            [section::<L>(52.0, true), section::<L>(18_200.0, false)],
        ],
    }
}

/// One `u32` per lane.
fn bits<L: Lane>(value: L) -> Vec<u32> {
    let mut words = [0_u32; MAX_WIDTH];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].to_vec()
}

/// A lane counter read back as the exact integer it is.
fn totals<L: Lane>(value: L) -> Vec<u32> {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[..L::WIDTH].iter().map(|word| *word as u32).collect()
}

// ---------------------------------------------------------------------------------------------
// The gates.
// ---------------------------------------------------------------------------------------------

/// The equivalence, at the level of the operation: for every subset of the lanes and every hostile
/// value class, `select(bad, 1.0, 0.0)` and `1.0 & bad` are the same bits.
///
/// This is the claim the canonical-mask contract makes — a comparison result is per lane all zero
/// bits or all one bits — checked rather than cited.
#[test]
fn the_and_form_is_the_select_form_on_every_lane_subset() {
    fn check<L: Lane>(width: &str) {
        let one = L::splat(1.0);
        let zero = L::zero();
        let limit = L::splat(NONFINITE_LIMIT);
        let sanitised = [
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            1.0e30,
            -1.0e30,
            -2.5e33,
            f32::from_bits(0x7F80_0001),
            f32::from_bits(0x7FC0_1234),
        ];
        let clean = [
            0.5,
            -0.0,
            0.0,
            9.999e29,
            -9.999e29,
            f32::from_bits(0x0000_0001),
            -0.75,
            f32::MIN_POSITIVE,
        ];
        for pattern in 0..(1_usize << L::WIDTH) {
            for (bad_class, clean_class) in sanitised.iter().zip(clean.iter()) {
                let mut words = [0.0_f32; MAX_WIDTH];
                for (lane, word) in words.iter_mut().enumerate().take(L::WIDTH) {
                    *word = if pattern >> lane & 1 == 1 {
                        *bad_class
                    } else {
                        *clean_class
                    };
                }
                let x = L::load(&words);
                let bad = L::mask_not(x.abs().lt(limit));
                assert_eq!(
                    bits::<L>(L::select(bad, one, zero)),
                    bits::<L>(one.andnot(L::mask_not(bad))),
                    "width={width}, pattern={pattern:#04x}, bad={bad_class:?}"
                );
                // And the verdict itself is the policy's, not the compare's accident.
                let expected: Vec<u32> = (0..L::WIDTH)
                    .map(|lane| u32::from(pattern >> lane & 1 == 1))
                    .collect();
                assert_eq!(
                    totals::<L>(L::select(bad, one, zero)),
                    expected,
                    "width={width}, pattern={pattern:#04x}: the boundary is `|x| < 1e30`"
                );
            }
        }
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}

/// The kernels, over 64 hostile blocks of evolving state: the tree's counter words and output
/// buffers are the select-form's to the bit, and both are the scalar oracle's count.
///
/// Every copy of the sanitise prologue is exercised: `input_chain_block` directly, the two elision
/// variants through `input_chain_block_elided` (an all-identity chain and a mixed one), and
/// `sanitize_gain_block` on its own leg.
#[test]
fn every_copy_of_the_sanitise_prologue_counts_what_the_policy_counts() {
    fn check<L: Lane>(width: &str) {
        let span = FRAMES * L::WIDTH;
        let c = chain_coef::<L>();

        // Leg 1: the chain kernel against the select-form replica and the oracle.
        let mut tree_state = InputChainState::<L>::default();
        let mut select_state = InputChainState::<L>::default();
        let mut tree_total = vec![0_u32; L::WIDTH];
        let mut oracle_total = vec![0_u32; L::WIDTH];
        for block in 0..BLOCKS {
            let mut tree_left = vec![0.0_f32; span];
            let mut tree_right = vec![0.0_f32; span];
            fill_hostile(&mut tree_left, block * 2, L::WIDTH);
            fill_hostile(&mut tree_right, block * 2 + 1, L::WIDTH);
            let oracle_left = oracle_counts(&tree_left, L::WIDTH);
            let oracle_right = oracle_counts(&tree_right, L::WIDTH);
            let mut select_left = tree_left.clone();
            let mut select_right = tree_right.clone();

            let report = input_chain_block::<L>(
                &mut tree_left,
                &mut tree_right,
                FRAMES,
                &c,
                &mut tree_state,
            );
            let select = input_chain_block_select::<L>(
                &mut select_left,
                &mut select_right,
                FRAMES,
                &c,
                &mut select_state,
            );

            assert_eq!(
                (&tree_left, &tree_right),
                (&select_left, &select_right),
                "width={width}, block={block}: output"
            );
            for (channel, oracle) in [oracle_left, oracle_right].into_iter().enumerate() {
                assert_eq!(
                    bits::<L>(report.sanitized[channel]),
                    bits::<L>(select[channel]),
                    "width={width}, block={block}, channel={channel}: counter words"
                );
                assert_eq!(
                    totals::<L>(report.sanitized[channel]),
                    oracle,
                    "width={width}, block={block}, channel={channel}: counter vs scalar oracle"
                );
                for (lane, count) in oracle.iter().enumerate() {
                    tree_total[lane] += count;
                    oracle_total[lane] += count;
                }
            }
        }
        assert_eq!(tree_total, oracle_total, "width={width}: lifetime totals");
        assert!(
            oracle_total.iter().all(|count| *count > 0),
            "width={width}: every lane must sanitise something, or the fixture is vacuous"
        );

        // Leg 2: the two elision variants carry the same prologue.
        let identity = InputChainCoef::<L> {
            trim: c.trim,
            section: [[identity_section::<L>(); 2]; 2],
        };
        let mut mixed = identity;
        mixed.section[0][1] = section::<L>(17_800.0, false);
        for (name, coefficients) in [("identity", identity), ("mixed", mixed)] {
            let mut elided_state = InputChainState::<L>::default();
            let mut plain_state = InputChainState::<L>::default();
            let plan = input_chain_plan::<L>(&coefficients, &elided_state);
            assert_ne!(plan, InputChainPlan::NONE, "width={width}, {name}: elides");
            for block in 0..BLOCKS {
                let mut elided_left = vec![0.0_f32; span];
                let mut elided_right = vec![0.0_f32; span];
                fill_hostile(&mut elided_left, 500 + block * 2, L::WIDTH);
                fill_hostile(&mut elided_right, 500 + block * 2 + 1, L::WIDTH);
                let oracle = [
                    oracle_counts(&elided_left, L::WIDTH),
                    oracle_counts(&elided_right, L::WIDTH),
                ];
                let mut plain_left = elided_left.clone();
                let mut plain_right = elided_right.clone();
                let elided = input_chain_block_elided::<L>(
                    &mut elided_left,
                    &mut elided_right,
                    FRAMES,
                    &coefficients,
                    &mut elided_state,
                    &plan,
                );
                let plain = input_chain_block::<L>(
                    &mut plain_left,
                    &mut plain_right,
                    FRAMES,
                    &coefficients,
                    &mut plain_state,
                );
                assert_eq!(
                    (&elided_left, &elided_right),
                    (&plain_left, &plain_right),
                    "width={width}, {name}, block={block}: output"
                );
                for (channel, oracle) in oracle.into_iter().enumerate() {
                    assert_eq!(
                        bits::<L>(elided.sanitized[channel]),
                        bits::<L>(plain.sanitized[channel]),
                        "width={width}, {name}, block={block}, channel={channel}: counter words"
                    );
                    assert_eq!(
                        totals::<L>(elided.sanitized[channel]),
                        oracle,
                        "width={width}, {name}, block={block}, channel={channel}: scalar oracle"
                    );
                }
            }
        }

        // Leg 3: `sanitize_gain_block` on its own, which is the `:93` site.
        let mut tree_count = L::zero();
        let mut select_count = L::zero();
        let mut oracle_gain = vec![0_u32; L::WIDTH];
        for block in 0..BLOCKS {
            let mut tree = vec![0.0_f32; span];
            fill_hostile(&mut tree, 1_000 + block, L::WIDTH);
            for (lane, count) in oracle_counts(&tree, L::WIDTH).into_iter().enumerate() {
                oracle_gain[lane] += count;
            }
            let mut select = tree.clone();
            tree_count = tree_count.add(sanitize_gain_block::<L>(&mut tree, FRAMES, c.trim[0]));
            select_count = select_count.add(sanitize_gain_block_select::<L>(
                &mut select,
                FRAMES,
                c.trim[0],
            ));
            assert_eq!(tree, select, "width={width}, gain block={block}: output");
        }
        assert_eq!(
            bits::<L>(tree_count),
            bits::<L>(select_count),
            "width={width}: sanitize_gain_block counter words"
        );
        assert_eq!(
            totals::<L>(tree_count),
            oracle_gain,
            "width={width}: sanitize_gain_block vs scalar oracle"
        );
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}

/// The disabled section, as `builtins` prepares it.
fn identity_section<L: Lane>() -> SvfCoef<L> {
    SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::splat(1.0),
        m1: L::zero(),
        m2: L::zero(),
    }
}
