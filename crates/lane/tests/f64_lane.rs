//! Issue #949, gates 1 to 3: the `f64` lane vocabulary is exact IEEE binary64 at every width.
//!
//! * **Gate 1, widen identity.** Every lane of `Widen::widen` at `Simd4` and `Simd8` (and the
//!   scalar `Widen for f32`) is compared with an independent oracle that builds the binary64 bits
//!   from the `f32` fields with integer arithmetic. The release build sweeps all 2^32 `f32` bit
//!   patterns at every width; the debug build sweeps every 65,537th pattern plus a directed pool.
//! * **Gate 2, `add`/`mul` identity.** `LaneF64::add` and `LaneF64::mul` at both vector widths
//!   against scalar `f64` `+` and `*`, over every ordered pair of a directed pool plus seeded random
//!   pairs.
//! * **Gate 3, square exactness witness.** `widen(x).mul(widen(x))` against an integer
//!   construction of `x * x`. This is the premise #950's class-A argument rests on: the square is
//!   exact, so only the order of the additions can move a bit.
//!
//! # Why every vector result goes through `black_box`
//!
//! `widen` is literally `fpext`, the same IR instruction as `f64::from`, and the extracted lane of
//! a vector `fmul` scalarises. Given the chance, LLVM proves a vector-against-scalar comparison
//! true and deletes the loop: the research measured an exhaustive 2^33-comparison sweep finishing
//! in 5 ms. So every vector result is stored through `core::hint::black_box`, and the widen oracle
//! is integer construction rather than `f64::from`. The release run time printed by gate 1 is the
//! evidence that the sweep executed.

#![allow(missing_docs)]

use core::hint::black_box;
use std::time::Instant;

use lane::{LaneF64, Widen};

/// Widest lane count under test.
const MAX_WIDTH: usize = 8;

/// Xorshift64\*: seeded and portable, so every host sees the same inputs.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// Gate 1's oracle: the binary64 bits of `x`, built from its fields with integer arithmetic only.
///
/// `None` for a NaN, whose widened payload is not pinned. The subnormal row normalises the
/// significand: with `k` leading zeros in the 23-bit field, the value `m * 2^-149` has its leading
/// one at bit `22 - k`, so its unbiased exponent is `-127 - k` (biased `896 - k`) and the fraction
/// is the `22 - k` bits below that one, shifted up to the top of the 52-bit field.
fn oracle_widen_bits(x: f32) -> Option<u64> {
    let bits = x.to_bits();
    let sign = u64::from(bits >> 31) << 63;
    let exponent = u64::from((bits >> 23) & 0xFF);
    let mantissa = bits & 0x7F_FFFF;
    match (exponent, mantissa) {
        (0, 0) => Some(sign),
        (0, _) => {
            let k = mantissa.leading_zeros() - 9;
            let fraction = u64::from((mantissa << (k + 1)) & 0x7F_FFFF) << 29;
            Some(sign | (896 - u64::from(k)) << 52 | fraction)
        }
        (255, 0) => Some(sign | 0x7FF << 52),
        (255, _) => None,
        _ => Some(sign | (exponent + 896) << 52 | u64::from(mantissa) << 29),
    }
}

/// The directed widen pool: both zeros, the smallest and largest subnormal and `MIN_POSITIVE` of
/// both signs, `±1.0` and its neighbours, `±MAX`, both infinities, and three NaN payloads (quiet,
/// negative quiet with a payload, and signalling).
const WIDEN_POOL: [u32; 21] = [
    0x0000_0000,
    0x8000_0000,
    0x0000_0001,
    0x8000_0001,
    0x007F_FFFF,
    0x807F_FFFF,
    0x0080_0000,
    0x8080_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x3F80_0001,
    0x3F7F_FFFF,
    0xBF80_0001,
    0xBF7F_FFFF,
    0x7F7F_FFFF,
    0xFF7F_FFFF,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_0000,
    0xFFC0_0001,
    0x7F80_0001,
];

/// Widens one group of `L::WIDTH` values and counts the lanes that disagree with the oracle.
#[inline(always)]
fn widen_group_mismatches<L: Widen>(src: &[f32; MAX_WIDTH]) -> u64 {
    let width = L::WIDTH;
    let widened = black_box(L::load(&src[..width]).widen());
    let mut out = [0.0f64; MAX_WIDTH];
    widened.store(&mut out[..width]);
    let out = black_box(out);
    let mut mismatches = 0;
    for lane in 0..width {
        let agrees = match oracle_widen_bits(src[lane]) {
            Some(bits) => out[lane].to_bits() == bits,
            None => out[lane].is_nan(),
        };
        mismatches += u64::from(!agrees);
    }
    mismatches
}

/// Gate 1 over the bit patterns `start..end`, lane `i` of each group taking pattern `p + i`.
fn widen_range_mismatches<L: Widen>(start: u64, end: u64) -> u64 {
    let width = L::WIDTH as u64;
    let mut src = [0.0f32; MAX_WIDTH];
    let mut mismatches = 0;
    let mut pattern = start;
    while pattern < end {
        for (lane, value) in src.iter_mut().enumerate().take(L::WIDTH) {
            *value = f32::from_bits(((pattern + lane as u64) & 0xFFFF_FFFF) as u32);
        }
        mismatches += widen_group_mismatches::<L>(&src);
        pattern += width;
    }
    mismatches
}

/// Gate 1 over a list of patterns, at every rotation, so that each pattern sits in every lane.
fn widen_list_mismatches<L: Widen>(patterns: &[u32]) -> u64 {
    let mut src = [0.0f32; MAX_WIDTH];
    let mut mismatches = 0;
    for rotation in 0..L::WIDTH {
        let mut index = 0;
        while index < patterns.len() {
            for (lane, value) in src.iter_mut().enumerate().take(L::WIDTH) {
                *value = f32::from_bits(patterns[(rotation + index + lane) % patterns.len()]);
            }
            mismatches += widen_group_mismatches::<L>(&src);
            index += L::WIDTH;
        }
    }
    mismatches
}

/// Every 65,537th `f32` bit pattern: `k * 0x1_0001 = (k << 16) | k` for every `k` in `0..65536`,
/// which is every sign and exponent with a spread of significands, ending at `0xFFFF_FFFF`.
fn sparse_patterns() -> Vec<u32> {
    (0..=0xFFFF_u32).map(|k| k * 0x1_0001).collect()
}

/// Gate 1 over all 2^32 patterns at one width, split across threads. Returns the mismatch count.
fn widen_exhaustive<L: Widen>(name: &str) -> u64 {
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get().min(4)) as u64;
    let span = (1_u64 << 32) / threads;
    let started = Instant::now();
    let mismatches: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let end = if t + 1 == threads {
                    1 << 32
                } else {
                    (t + 1) * span
                };
                scope.spawn(move || widen_range_mismatches::<L>(t * span, end))
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("sweep thread panicked"))
            .sum()
    });
    eprintln!(
        "gate 1: widen at {name}: all 2^32 patterns, {mismatches} mismatches, {:.2?} on {threads} threads",
        started.elapsed()
    );
    mismatches
}

#[test]
fn gate1_oracle_is_the_conversion_on_known_values() {
    // The oracle is independent of `f64::from`, so it is checked here against values whose
    // binary64 bits are known by hand, before it is trusted as the gate's reference.
    let known: [(u32, u64); 8] = [
        (0x3F80_0000, 0x3FF0_0000_0000_0000),
        (0xC000_0000, 0xC000_0000_0000_0000),
        (0x0000_0001, 0x36A0_0000_0000_0000),
        (0x0040_0000, 0x3800_0000_0000_0000),
        (0x007F_FFFF, 0x380F_FFFF_C000_0000),
        (0x0080_0000, 0x3810_0000_0000_0000),
        (0x7F7F_FFFF, 0x47EF_FFFF_E000_0000),
        (0xFF80_0000, 0xFFF0_0000_0000_0000),
    ];
    for (input, expected) in known {
        assert_eq!(
            oracle_widen_bits(f32::from_bits(input)),
            Some(expected),
            "oracle at {input:#010x}"
        );
    }
    assert_eq!(oracle_widen_bits(f32::from_bits(0x7FC0_0000)), None);
}

#[test]
fn gate1_widen_is_exact_on_the_directed_pool_and_the_sparse_sweep() {
    let mut patterns = sparse_patterns();
    patterns.extend_from_slice(&WIDEN_POOL);
    lane::each_lane!(|L| {
        let name = core::any::type_name::<L>();
        assert_eq!(
            widen_list_mismatches::<L>(&patterns),
            0,
            "gate 1: widen at {name} over the directed pool and sparse sweep"
        );
    });
}

#[test]
fn gate1_widen_is_exact_on_every_f32_bit_pattern() {
    if cfg!(debug_assertions) {
        // The exhaustive sweep is the release gate; the debug build runs the sparse sweep above.
        return;
    }
    lane::each_lane!(|L| {
        let name = core::any::type_name::<L>();
        assert_eq!(widen_exhaustive::<L>(name), 0, "gate 1: widen at {name}");
    });
}

/// Gate 2's directed pool: both zeros, the smallest and largest subnormal and `MIN_POSITIVE` of
/// both signs, `±1.0`, `1 + 2^-52`, `±MAX`, both infinities, the tie-rounding members (`2^-53`,
/// `1.5` and `2^53`; see `gate2_pool_contains_round_to_nearest_even_ties`) and two NaN payloads.
fn f64_pool() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::from_bits(0x000F_FFFF_FFFF_FFFF),
        -f64::from_bits(0x000F_FFFF_FFFF_FFFF),
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        1.0,
        -1.0,
        1.0 + f64::EPSILON,
        f64::MAX,
        -f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x3CA0_0000_0000_0000),
        1.5,
        f64::from_bits(0x4340_0000_0000_0000),
        f64::from_bits(0x7FF8_0000_0000_0000),
        f64::from_bits(0xFFF8_0000_0000_0001),
    ]
}

#[test]
fn gate2_pool_contains_round_to_nearest_even_ties() {
    // The pool's rounding members produce exact ties, so a lowering that rounded a tie any other
    // way than to even would be seen. Checked on the scalar oracle, which is the reference.
    let half_ulp = f64::from_bits(0x3CA0_0000_0000_0000);
    let one_plus = 1.0 + f64::EPSILON;
    let two_53 = f64::from_bits(0x4340_0000_0000_0000);
    assert_eq!(half_ulp, f64::EPSILON / 2.0);
    // 1 + 2^-53 lies halfway between 1 (even) and 1 + 2^-52 (odd).
    assert_eq!(black_box(1.0) + black_box(half_ulp), 1.0);
    // (1 + 2^-52) + 2^-53 lies halfway between 1 + 2^-52 (odd) and 1 + 2^-51 (even).
    assert_eq!(
        (black_box(one_plus) + black_box(half_ulp)).to_bits(),
        0x3FF0_0000_0000_0002
    );
    // 2^53 + 1 lies halfway between 2^53 (even) and 2^53 + 2 (odd).
    assert_eq!(black_box(two_53) + black_box(1.0), two_53);
    // 1.5 * (1 + 2^-52) = 1.5 + 2^-52 + 2^-53 lies halfway between 1.5 + 2^-52 (odd) and
    // 1.5 + 2^-51 (even).
    assert_eq!(
        (black_box(1.5) * black_box(one_plus)).to_bits(),
        0x3FF8_0000_0000_0002
    );
}

/// `add` (`op == 0`) or `mul` of one group at `F::WIDTH` against scalar `+`/`*`, lane by lane.
#[inline(always)]
fn binop_group_mismatches<F: LaneF64>(a: &[f64; MAX_WIDTH], b: &[f64; MAX_WIDTH], op: u8) -> u64 {
    let width = F::WIDTH;
    let va = F::load(&a[..width]);
    let vb = F::load(&b[..width]);
    let result = black_box(if op == 0 { va.add(vb) } else { va.mul(vb) });
    let mut out = [0.0f64; MAX_WIDTH];
    result.store(&mut out[..width]);
    let out = black_box(out);
    let mut mismatches = 0;
    for lane in 0..width {
        let expected = if op == 0 {
            a[lane] + b[lane]
        } else {
            a[lane] * b[lane]
        };
        let agrees = if expected.is_nan() {
            out[lane].is_nan()
        } else {
            out[lane].to_bits() == expected.to_bits()
        };
        mismatches += u64::from(!agrees);
    }
    mismatches
}

/// Every ordered pair of `pool`, laid out consecutively and read at every rotation.
fn pool_pair_mismatches<F: LaneF64>(pool: &[f64]) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &x in pool {
        for &y in pool {
            left.push(x);
            right.push(y);
        }
    }
    let mut a = [0.0f64; MAX_WIDTH];
    let mut b = [0.0f64; MAX_WIDTH];
    let mut mismatches = 0;
    for rotation in 0..F::WIDTH {
        let mut index = 0;
        while index < left.len() {
            for lane in 0..F::WIDTH {
                let at = (rotation + index + lane) % left.len();
                a[lane] = left[at];
                b[lane] = right[at];
            }
            for op in 0..2 {
                mismatches += binop_group_mismatches::<F>(&a, &b, op);
            }
            index += F::WIDTH;
        }
    }
    mismatches
}

/// One seeded random operand pair: either two arbitrary bit patterns (every class, NaN included,
/// mostly far-apart exponents) or a pair whose exponents lie within 60 binades of each other, so
/// that the addition actually rounds.
fn random_pair(rng: &mut Rng) -> (f64, f64) {
    let a = rng.next_u64();
    let b = if rng.next_u64() & 1 == 0 {
        rng.next_u64()
    } else {
        let exponent = ((a >> 52) & 0x7FF) as i64 + (rng.next_u64() % 121) as i64 - 60;
        let exponent = exponent.clamp(0, 0x7FE) as u64;
        (rng.next_u64() & 0x800F_FFFF_FFFF_FFFF) | exponent << 52
    };
    (f64::from_bits(a), f64::from_bits(b))
}

/// `pairs` seeded random groups at `F::WIDTH`, every lane an independent pair.
fn random_pair_mismatches<F: LaneF64>(seed: u64, pairs: usize) -> u64 {
    let mut rng = Rng(seed);
    let mut a = [0.0f64; MAX_WIDTH];
    let mut b = [0.0f64; MAX_WIDTH];
    let mut mismatches = 0;
    let mut done = 0;
    while done < pairs {
        for lane in 0..F::WIDTH {
            (a[lane], b[lane]) = random_pair(&mut rng);
        }
        for op in 0..2 {
            mismatches += binop_group_mismatches::<F>(&a, &b, op);
        }
        done += F::WIDTH;
    }
    mismatches
}

#[test]
fn gate2_add_and_mul_are_exact_on_every_ordered_pool_pair() {
    let pool = f64_pool();
    // Each `f32` lane type's `f64` companion: `f64`, then the vector widths this build has.
    lane::each_lane!(|L| {
        let name = core::any::type_name::<<L as Widen>::F64>();
        let mismatches = pool_pair_mismatches::<<L as Widen>::F64>(&pool);
        assert_eq!(mismatches, 0, "gate 2: add/mul at {name} over the pool");
    });
}

#[test]
fn gate2_add_and_mul_are_exact_on_seeded_random_pairs() {
    let pairs = if cfg!(debug_assertions) {
        20_000
    } else {
        1_000_000
    };
    // One seed per width: `0x0949_F64A_DD00_0000` plus the lane count.
    lane::each_vector_lane!(|L, N| {
        let name = core::any::type_name::<<L as Widen>::F64>();
        let seed = 0x0949_F64A_DD00_0000 + N as u64;
        assert_eq!(
            random_pair_mismatches::<<L as Widen>::F64>(seed, pairs),
            0,
            "gate 2: add/mul at {name} over {pairs} random pairs"
        );
    });
}

/// Gate 3's oracle: `x * x` for a finite `x = ±M * 2^E`, as `(M * M) * 2^(2E)`.
///
/// `M * M` is below `2^48`, so its conversion is exact, and `2E` lies in `[-298, 208]`, so the power
/// of two is a normal `f64` and the product is exact.
fn oracle_square_bits(x: f32) -> u64 {
    let bits = x.to_bits();
    let exponent = ((bits >> 23) & 0xFF) as i32;
    let mantissa = u64::from(bits & 0x7F_FFFF);
    let (significand, scale) = if exponent == 0 {
        (mantissa, -149)
    } else {
        (mantissa | 0x80_0000, exponent - 150)
    };
    let power = f64::from_bits(((2 * scale + 1023) as u64) << 52);
    ((significand * significand) as f64 * power).to_bits()
}

/// Gate 3's directed inputs: `±MAX`, `±MIN_POSITIVE`, both subnormal extremes, `1 ± ulp`, both
/// zeros, and `±2^k` for `k` in `-34..=5`.
fn square_pool() -> Vec<f32> {
    let mut pool: Vec<f32> = [
        0x7F7F_FFFF_u32,
        0xFF7F_FFFF,
        0x0080_0000,
        0x8080_0000,
        0x0000_0001,
        0x8000_0001,
        0x007F_FFFF,
        0x807F_FFFF,
        0x3F80_0001,
        0x3F7F_FFFF,
        0x0000_0000,
        0x8000_0000,
    ]
    .into_iter()
    .map(f32::from_bits)
    .collect();
    for k in -34_i32..=5 {
        let power = f32::from_bits(((127 + k) as u32) << 23);
        pool.push(power);
        pool.push(-power);
    }
    pool
}

/// `widen(x).mul(widen(x))` at `L::WIDTH` against the integer oracle, lane by lane.
fn square_mismatches<L: Widen>(inputs: &[f32]) -> u64 {
    let width = L::WIDTH;
    let mut src = [0.0f32; MAX_WIDTH];
    let mut mismatches = 0;
    let mut index = 0;
    while index < inputs.len() {
        for (lane, value) in src.iter_mut().enumerate().take(width) {
            *value = inputs[(index + lane) % inputs.len()];
        }
        let widened = L::load(&src[..width]).widen();
        let square = black_box(widened.mul(widened));
        let mut out = [0.0f64; MAX_WIDTH];
        square.store(&mut out[..width]);
        let out = black_box(out);
        for lane in 0..width {
            mismatches += u64::from(out[lane].to_bits() != oracle_square_bits(src[lane]));
        }
        index += width;
    }
    mismatches
}

#[test]
fn gate3_the_square_of_a_widened_f32_is_exact() {
    let mut inputs = square_pool();
    let mut rng = Rng(0x0949_5A0A_2E00_0003);
    let mut finite = 0;
    while finite < 100_000 {
        let x = f32::from_bits(rng.next_u64() as u32);
        if x.is_finite() {
            inputs.push(x);
            finite += 1;
        }
    }
    lane::each_lane!(|L| {
        let name = core::any::type_name::<L>();
        assert_eq!(
            square_mismatches::<L>(&inputs),
            0,
            "gate 3: square witness at {name}"
        );
    });
}
