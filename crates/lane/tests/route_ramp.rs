//! Issue #1219: the indexed ramp of a send's 2x2 coefficients, `route_mix_ramp_block`.
//!
//! Every gate compares the kernel, at `f32`, `Simd4` and (where `avx2` is enabled) `Simd8`, bit for
//! bit with an independent scalar oracle written here in plain `f32` arithmetic: the law
//! `c(k) = round(round(k * step) + start)` for `1 <= k < length`, `c(k) = target` from
//! `k = length`, frame `f` of a block at ramp index `k = position + f + 1`, then the 2x2 mix in
//! `mix2x2_block`'s order. NaN words are folded (`dsp_reference::class_a`, decision 10). Red
//! mutations: `tests/MUTATIONS.md`, issue #1219.
#![allow(missing_docs)]

use dsp_reference::class_a;
use lane::kernels::{
    INDEXED_RAMP_LENGTH_MAXIMUM, IndexedRamp, mix2x2_block, route_mix_column_ramps_block,
    route_mix_ramp_block,
};
use lane::{CanonicalFpEnv, Lane};

/// The block lengths of every gate: whole vectors at every width, a tail, and a tail only.
const BLOCKS: [usize; 3] = [128, 125, 1];

/// Xorshift64* (Vigna 2016): seeded, so every host and every run sees the same planes.
struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    /// A signed value of moderate exponent, `(1 + m) * 2^e`, `e` in `[-12, 4)`.
    fn moderate(&mut self) -> f32 {
        let bits = self.next_u32();
        let exponent = (bits >> 23) % 16 + 127 - 12;
        f32::from_bits((bits & 0x8000_0000) | (exponent << 23) | (bits & 0x007f_ffff))
    }

    /// A coefficient in `(-4, 4)`, or exactly `-0.0` one time in eight.
    fn coefficient(&mut self) -> f32 {
        if self.next_u32().is_multiple_of(8) {
            -0.0
        } else {
            let unit = (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32;
            unit * 8.0 - 4.0
        }
    }
}

/// Signed zeros, subnormals and non-finite words, NaN payloads included.
const HOSTILE: [f32; 10] = [
    -0.0,
    0.0,
    f32::from_bits(1),
    f32::from_bits(0x8000_0003),
    f32::MIN_POSITIVE,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
    f32::from_bits(0xffc0_1234),
    -1.0e38,
];

/// One block's two planes: moderate audio with a hostile word every fifth frame, offset by `salt`.
fn planes(rng: &mut Rng, frames: usize, salt: usize) -> (Vec<f32>, Vec<f32>) {
    let mut left = Vec::with_capacity(frames);
    let mut right = Vec::with_capacity(frames);
    for frame in 0..frames {
        left.push(if (frame + salt).is_multiple_of(5) {
            HOSTILE[(frame / 5 + salt) % HOSTILE.len()]
        } else {
            rng.moderate()
        });
        right.push(if (frame + salt + 2).is_multiple_of(5) {
            HOSTILE[(frame / 5 + salt + 3) % HOSTILE.len()]
        } else {
            rng.moderate()
        });
    }
    (left, right)
}

/// The oracle's law, built from `(start, target, length)` alone, never from an `IndexedRamp`.
#[derive(Clone, Copy)]
struct Law {
    start: [f32; 4],
    step: [f32; 4],
    target: [f32; 4],
    length: u32,
}

impl Law {
    fn new(start: [f32; 4], target: [f32; 4], length: u32) -> Self {
        let difference: [f32; 4] = core::array::from_fn(|index| target[index] - start[index]);
        if length == 0 || difference.iter().any(|value| !value.is_finite()) {
            return Self {
                start: target,
                step: [0.0; 4],
                target,
                length: 0,
            };
        }
        let step = difference.map(|difference| difference / length as f32);
        Self {
            start,
            step,
            target,
            length,
        }
    }

    /// `c(k)` with two explicit roundings: the product, then the sum.
    fn at(&self, k: u32) -> [f32; 4] {
        if k >= self.length {
            return self.target;
        }
        if k == 0 {
            return self.start;
        }
        core::array::from_fn(|index| {
            let product = k as f32 * self.step[index];
            product + self.start[index]
        })
    }

    /// The block at `position`: frame `f` at `k = position + f + 1`, then the 2x2 mix from the
    /// frame's original `l` and `r`, `l' = lr * r + ll * l`, `r' = rr * r + rl * l`.
    fn render(&self, left: &mut [f32], right: &mut [f32], position: u32) {
        for (frame, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let [ll, lr, rl, rr] = self.at(position + frame as u32 + 1);
            let (l, r) = (*left, *right);
            let (left_product, right_product) = (ll * l, rl * l);
            *left = lr * r + left_product;
            *right = rr * r + right_product;
        }
    }
}

/// Panics at the first word whose folded bits differ from `reference`'s.
fn assert_same(case: &str, plane: &str, got: &[f32], want: &[f32], reference: &str) {
    assert_eq!(got.len(), want.len());
    for (frame, (got, want)) in got.iter().zip(want).enumerate() {
        assert!(
            class_a::same(*got, *want),
            "{case}: {plane} frame {frame}: {:#010x} against {reference}'s {:#010x}",
            got.to_bits(),
            want.to_bits(),
        );
    }
}

/// Renders one block with the kernel at `L` and with `oracle`, from the same planes, and compares.
fn compare_block<L: Lane>(
    case: &str,
    ramp: &IndexedRamp,
    oracle: &Law,
    position: u32,
    planes: &(Vec<f32>, Vec<f32>),
) {
    let (mut left, mut right) = planes.clone();
    let (mut want_left, mut want_right) = planes.clone();
    route_mix_ramp_block::<L>(&mut left, &mut right, ramp, position);
    oracle.render(&mut want_left, &mut want_right, position);
    assert_same(case, "left", &left, &want_left, "the oracle");
    assert_same(case, "right", &right, &want_right, "the oracle");
}

/// Runs consecutive blocks from `first` with the caller's saturating advance,
/// `position = min(position + frames, length)`, until one block after the ramp settles or
/// `blocks` blocks have run.
fn run_from<L: Lane>(ramp: &IndexedRamp, oracle: &Law, first: u32, frames: usize, blocks: usize) {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ u64::from(first) ^ ((frames as u64) << 40));
    let mut position = first;
    let mut settled_blocks = 0;
    for block in 0..blocks {
        let case = format!(
            "width {}, length {}, {frames} frames from position {first}, block {block} at \
             position {position}",
            L::WIDTH,
            ramp.length
        );
        compare_block::<L>(
            &case,
            ramp,
            oracle,
            position,
            &planes(&mut rng, frames, block),
        );
        position = ramp.length.min(position + frames as u32);
        if position == ramp.length {
            settled_blocks += 1;
            if settled_blocks == 2 {
                break;
            }
        }
    }
}

/// Two coefficient pairs: inexact steps both ways, a coefficient that does not move, and targets
/// of `-0.0` and of a value `start + length * step` does not reach exactly.
const STARTS: [[f32; 4]; 2] = [
    [0.891_250_9, -0.3, 0.123_456_7, 1.0],
    [-1.25, 0.0, 2.511_886_4, -0.0],
];
const TARGETS: [[f32; 4]; 2] = [
    [-0.0, 0.707_106_77, -1.584_893_2, 1.0],
    [0.316_227_77, -3.162_277_7, 0.1, 0.5],
];

fn law_width<L: Lane>() {
    for length in [0, 1, 37, 4800, INDEXED_RAMP_LENGTH_MAXIMUM] {
        for (start, target) in STARTS.into_iter().zip(TARGETS) {
            let ramp = IndexedRamp::new(start, target, length);
            let oracle = Law::new(start, target, length);
            for frames in BLOCKS {
                // From the start, and from positions that put the snap at every offset of a vector
                // and of a block, mid-block and mid-vector included.
                let mut firsts = vec![0];
                for back in [1, 2, 3, 5, 6, 7, 9, 13, 64, 125, 127, 128, 129, 200, 300] {
                    firsts.push(length.saturating_sub(back));
                }
                firsts.sort_unstable();
                firsts.dedup();
                for first in firsts {
                    let blocks = if frames == 1 { 320 } else { 48 };
                    run_from::<L>(&ramp, &oracle, first, frames, blocks);
                }
            }
        }
    }
}

/// Gate 1, the law: the kernel equals the oracle bit for bit at every width, for `length` in
/// `{0, 1, 37, 4800, 2^22}`, blocks of 128, 125 and 1 frames, ramps that end mid-block and
/// mid-vector, and planes with signed zeros, subnormals and non-finite samples.
///
/// Red mutations (#1219 M2-M4, M8-M10): the settled frames computed instead of assigned; `k` off
/// by one in the vector body or in the outlined tail; the snap one frame late; every frame of a
/// vector taking the vector's first index; the ramp body's left coefficients swapped.
#[test]
fn the_kernel_is_the_indexed_ramp_law_at_every_width() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| law_width::<L>());
}

fn retarget_width<L: Lane>() {
    let first = IndexedRamp::new(STARTS[1], TARGETS[0], 4800);
    let first_oracle = Law::new(STARTS[1], TARGETS[0], 4800);
    for position in [0, 1, 2, 1037, 4799, 4800] {
        let current = first.coefficients_at(position);
        let want = first_oracle.at(position);
        for (index, (current, want)) in current.iter().zip(want).enumerate() {
            assert_eq!(
                current.to_bits(),
                want.to_bits(),
                "coefficients_at({position})[{index}]"
            );
        }
        for length in [37, 4800] {
            let ramp = IndexedRamp::new(current, TARGETS[1], length);
            let oracle = Law::new(want, TARGETS[1], length);
            for frames in BLOCKS {
                run_from::<L>(&ramp, &oracle, 0, frames, 64);
                run_from::<L>(&ramp, &oracle, length - 3, frames, 4);
            }
        }
    }
}

/// Gate 1, the retarget: a ramp retargeted at `position` starts from `coefficients_at(position)`,
/// which is the oracle's `c(position)` exactly, at the start (a `-0.0` start word), mid-ramp and
/// settled, and the new ramp is the law from there at every width.
///
/// Red mutations (#1219 M6, M7): `coefficients_at(0)` computed instead of returning `start`; the
/// mid-ramp `coefficients_at` fused (`mul_add`) instead of rounded twice.
#[test]
fn a_retarget_starts_from_the_exact_current_coefficients() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| retarget_width::<L>());
}

fn overflow_width<L: Lane>(ramp: &IndexedRamp, target: [f32; 4]) {
    for frames in BLOCKS {
        let mut rng = Rng(0x5eed ^ frames as u64);
        for position in [0, 1, 7, 4795, 4799, 4800] {
            let (left, right) = planes(&mut rng, frames, position as usize);
            let (mut got_left, mut got_right) = (left.clone(), right.clone());
            let (mut want_left, mut want_right) = (left, right);
            route_mix_ramp_block::<L>(&mut got_left, &mut got_right, ramp, position);
            mix2x2_block::<f32>(&mut want_left, &mut want_right, target);
            let case = format!("width {}, {frames} frames at position {position}", L::WIDTH);
            assert_same(&case, "left", &got_left, &want_left, "mix2x2_block");
            assert_same(&case, "right", &got_right, &want_right, "mix2x2_block");
        }
    }
}

/// Gate 2: a difference that overflows (`3e38 - -3e38`) in one coefficient makes the whole ramp a
/// step to the target: every `coefficients_at(k)` is the target and the kernel renders exactly
/// `mix2x2_block` with it, at every width.
///
/// Red mutation (#1219 M5): drop the non-finite branch from `IndexedRamp::new`.
#[test]
fn an_overflowing_difference_is_a_step_to_the_target() {
    let _canonical = CanonicalFpEnv::enter();
    let start = [0.5, -3.0e38, 0.25, 1.0];
    let target = [-0.75, 3.0e38, 0.125, 0.5];
    let ramp = IndexedRamp::new(start, target, 4800);
    for k in [0, 1, 2, 2400, 4799, 4800, 4801] {
        assert_eq!(
            ramp.coefficients_at(k).map(f32::to_bits),
            target.map(f32::to_bits),
            "coefficients_at({k})"
        );
    }
    lane::each_lane!(|L| overflow_width::<L>(&ramp, target));
}

fn settled_width<L: Lane>() {
    let mut rng = Rng(0x0123_4567_89ab_cdef);
    for case in 0..64 {
        let start: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let target: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let length = 1 + rng.next_u32() % 6000;
        let ramp = if case == 0 {
            IndexedRamp::settled(target)
        } else {
            IndexedRamp::new(start, target, length)
        };
        for frames in BLOCKS {
            let position = ramp.length;
            for block in 0..3 {
                let (left, right) = planes(&mut rng, frames, block);
                let (mut got_left, mut got_right) = (left.clone(), right.clone());
                let (mut want_left, mut want_right) = (left, right);
                route_mix_ramp_block::<L>(&mut got_left, &mut got_right, &ramp, position);
                mix2x2_block::<f32>(&mut want_left, &mut want_right, target);
                let label = format!(
                    "width {}, case {case}, length {}, {frames} frames, block {block} at position \
                     {position}",
                    L::WIDTH,
                    ramp.length
                );
                assert_same(&label, "left", &got_left, &want_left, "mix2x2_block");
                assert_same(&label, "right", &got_right, &want_right, "mix2x2_block");
            }
        }
    }
}

/// Gate 3: a ramp whose `length` has elapsed renders exactly `mix2x2_block` with its target, for
/// random targets (`-0.0` among them) and lengths, at every width, over further blocks at the
/// saturated `position == length`; so does `IndexedRamp::settled`.
///
/// It compares with `mix2x2_block` itself, the kernel a freshly prepared plan runs, not with the
/// oracle: red mutation #1219 M11 (the settled frames re-implemented as a private loop, then
/// `mix2x2_block`'s order changed) is red here alone. Also red on M2 and M10.
#[test]
fn a_settled_ramp_is_the_static_mix() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| settled_width::<L>());
}

/// The oracle of `route_mix_column_ramps_block`: column 0 (`ll`, `rl`) by `zero` at
/// `k = positions[0] + f + 1`, column 1 (`lr`, `rr`) by `one` at `k = positions[1] + f + 1`, then
/// the 2x2 mix in `mix2x2_block`'s order.
fn render_columns(laws: [&Law; 2], left: &mut [f32], right: &mut [f32], positions: [u32; 2]) {
    for (frame, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
        let frame = frame as u32;
        let zero = laws[0].at(positions[0].saturating_add(frame).saturating_add(1));
        let one = laws[1].at(positions[1].saturating_add(frame).saturating_add(1));
        let [ll, _, rl, _] = zero;
        let [_, lr, _, rr] = one;
        let (l, r) = (*left, *right);
        let (left_product, right_product) = (ll * l, rl * l);
        *left = lr * r + left_product;
        *right = rr * r + right_product;
    }
}

fn shared_width<L: Lane>() {
    let mut rng = Rng(0x0007_a110_fc01_u64);
    for case in 0..96 {
        let start: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let target: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let length = match case % 4 {
            0 => 0,
            1 => 1 + rng.next_u32() % 64,
            2 => 1 + rng.next_u32() % 6000,
            _ => INDEXED_RAMP_LENGTH_MAXIMUM - rng.next_u32() % 300,
        };
        let ramp = IndexedRamp::new(start, target, length);
        for frames in BLOCKS {
            let back = rng.next_u32() % 300;
            for position in [0, 1, length.saturating_sub(back), length] {
                let (left, right) = planes(&mut rng, frames, case);
                let (mut got_left, mut got_right) = (left.clone(), right.clone());
                let (mut want_left, mut want_right) = (left, right);
                route_mix_column_ramps_block::<L>(
                    &mut got_left,
                    &mut got_right,
                    &[ramp; 2],
                    [position; 2],
                );
                route_mix_ramp_block::<L>(&mut want_left, &mut want_right, &ramp, position);
                let label = format!(
                    "width {}, case {case}, length {length}, {frames} frames at {position}",
                    L::WIDTH
                );
                assert_same(
                    &label,
                    "left",
                    &got_left,
                    &want_left,
                    "route_mix_ramp_block",
                );
                assert_same(
                    &label,
                    "right",
                    &got_right,
                    &want_right,
                    "route_mix_ramp_block",
                );
            }
        }
        // Two settled columns of different targets: the static mix of the two targets, which is
        // what a prepared route (and the six-word kernel's settled ramp) of those targets mixes.
        let other: [f32; 4] = core::array::from_fn(|_| rng.coefficient());
        let composite = [target[0], other[1], target[2], other[3]];
        let ramps = [ramp, IndexedRamp::new(start, other, length / 2)];
        for frames in BLOCKS {
            let (left, right) = planes(&mut rng, frames, case + 1);
            let (mut got_left, mut got_right) = (left.clone(), right.clone());
            let (mut want_left, mut want_right) = (left.clone(), right.clone());
            let (mut static_left, mut static_right) = (left, right);
            route_mix_column_ramps_block::<L>(
                &mut got_left,
                &mut got_right,
                &ramps,
                [length, length / 2],
            );
            route_mix_ramp_block::<L>(
                &mut want_left,
                &mut want_right,
                &IndexedRamp::settled(composite),
                0,
            );
            mix2x2_block::<f32>(&mut static_left, &mut static_right, composite);
            let label = format!("width {}, case {case}, settled, {frames} frames", L::WIDTH);
            assert_same(
                &label,
                "left",
                &got_left,
                &want_left,
                "route_mix_ramp_block",
            );
            assert_same(
                &label,
                "right",
                &got_right,
                &want_right,
                "route_mix_ramp_block",
            );
            assert_same(&label, "left", &got_left, &static_left, "mix2x2_block");
            assert_same(&label, "right", &got_right, &static_right, "mix2x2_block");
        }
    }
}

/// #1652 R18-5 gate: with one ramp in both columns at one position, the column-ramp kernel is the
/// six-word kernel bit for bit (lengths 0 to `2^22`, from the start, mid-ramp near the snap and
/// settled); with two settled columns it is `mix2x2_block` of the two targets, the static mix a
/// prepared route of those targets runs. At every width.
///
/// Test value: red if the column kernel splits its parts differently from the six-word kernel
/// (a frame computed instead of assigned at the snap), reads a coefficient from the wrong
/// column's ramp, or settles by its own law instead of `mix2x2_block`'s.
#[test]
fn equal_column_ramps_are_the_six_word_kernel_and_settled_ones_the_static_mix() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| shared_width::<L>());
}

fn columns_width<L: Lane>() {
    let mut rng = Rng(0x0c01_0000_5eed_u64);
    let cases = [
        // (length 0, length 1, position 0, position 1): the left column ends first, the right
        // first, one settled throughout, both ending in one vector, and a long and a short ramp.
        (48, 78, 0, 30),
        (78, 48, 30, 0),
        (0, 4800, 0, 17),
        (5, 6, 0, 0),
        (37, 4800, 2, 4795),
        (
            INDEXED_RAMP_LENGTH_MAXIMUM,
            13,
            INDEXED_RAMP_LENGTH_MAXIMUM - 129,
            0,
        ),
    ];
    for (case, (length_zero, length_one, position_zero, position_one)) in
        cases.into_iter().enumerate()
    {
        let starts: [[f32; 4]; 2] =
            core::array::from_fn(|_| core::array::from_fn(|_| rng.coefficient()));
        let targets: [[f32; 4]; 2] =
            core::array::from_fn(|_| core::array::from_fn(|_| rng.coefficient()));
        let ramps = [
            IndexedRamp::new(starts[0], targets[0], length_zero),
            IndexedRamp::new(starts[1], targets[1], length_one),
        ];
        let laws = [
            Law::new(starts[0], targets[0], length_zero),
            Law::new(starts[1], targets[1], length_one),
        ];
        for frames in BLOCKS {
            let mut positions = [position_zero, position_one];
            for block in 0..6 {
                let (left, right) = planes(&mut rng, frames, block);
                let (mut got_left, mut got_right) = (left.clone(), right.clone());
                let (mut want_left, mut want_right) = (left, right);
                route_mix_column_ramps_block::<L>(&mut got_left, &mut got_right, &ramps, positions);
                render_columns(
                    [&laws[0], &laws[1]],
                    &mut want_left,
                    &mut want_right,
                    positions,
                );
                let label = format!(
                    "width {}, case {case}, {frames} frames, block {block} at {positions:?}",
                    L::WIDTH
                );
                assert_same(&label, "left", &got_left, &want_left, "the column oracle");
                assert_same(
                    &label,
                    "right",
                    &got_right,
                    &want_right,
                    "the column oracle",
                );
                for (position, ramp) in positions.iter_mut().zip(&ramps) {
                    *position = ramp.length.min(position.saturating_add(frames as u32));
                }
            }
        }
    }
}

/// #1652 R18-5 gate: two column ramps of different lengths and positions follow each column's
/// own indexed law, equal to an independent scalar oracle per column, at every width: the left
/// column ending before the right and after it, one column settled throughout, both ending in one
/// vector, and a ramp at the `2^22` bound beside a short one.
///
/// Test value: red if a column takes the other column's index or length (the shared-ramp law),
/// if the one-ramping part assigns the ramping column's target, or if a part's tail starts at the
/// wrong frame index.
#[test]
fn each_column_follows_its_own_ramp() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| columns_width::<L>());
}
