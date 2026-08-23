//! Gate G3: the software FMA is the hardware FMA, bit for bit.
//!
//! Master plan #83 §3.5 and §3.6. `fma_f32_via_f64` is what the wasm backend uses for
//! `Lane::fma`, so if it were only "close", decision D5 (bit identity across targets) would be
//! false on the web target. The oracle is `f32::mul_add`, which under the workspace's x86-64-v3
//! pin is `vfmadd`; the gate therefore runs on x86 and needs no browser (the v1 lesson:
//! software-versus-hardware agreement is the proof).
//!
//! Red-mutations proven for this gate (see `tests/MUTATIONS.md`):
//! * replace the direction-aware round-to-odd with `s_bits | 1` — the midpoint family fails;
//! * delete the `finite` guard — the edge pool fails, an infinite sum becomes NaN;
//! * return `(p + c) as f32` — the random sweep and the midpoint family fail.

mod support;

use miso_engine_lane::softfma::fma_f32_via_f64;
use support::{EDGES, FUSED_WITNESS_A, FUSED_WITNESS_C, Xorshift64Star};

/// Seed of the random sweep.
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

/// Random triples. The `--release` count is the gate; a debug run keeps the workspace suite quick.
const RANDOM_TRIPLES: usize = if cfg!(debug_assertions) {
    200_000
} else {
    10_000_000
};

/// Constructed midpoint triples, where an unconditional round-to-odd rounds the wrong way.
const MIDPOINT_TRIPLES: usize = if cfg!(debug_assertions) {
    100_000
} else {
    2_000_000
};

/// Compares the software FMA with the hardware FMA on one triple, counting mismatches.
fn check(a: f32, b: f32, c: f32, family: &str, mismatches: &mut usize) {
    let expected = f32::mul_add(a, b, c);
    let actual = fma_f32_via_f64(a, b, c);
    if expected.is_nan() && actual.is_nan() {
        return;
    }
    if expected.to_bits() != actual.to_bits() {
        *mismatches += 1;
        if *mismatches <= 5 {
            eprintln!(
                "G3 {family}: fma({a:e}, {b:e}, {c:e}) = {actual:e} ({actual_bits:#010x}), \
                 hardware {expected:e} ({expected_bits:#010x})",
                actual_bits = actual.to_bits(),
                expected_bits = expected.to_bits(),
            );
        }
    }
}

#[test]
fn g3_soft_fma_equals_hardware_fma_on_the_edge_pool() {
    let mut mismatches = 0;
    for a in EDGES {
        for b in EDGES {
            for c in EDGES {
                check(*a, *b, *c, "edges", &mut mismatches);
            }
        }
    }
    // The overflow and infinity triples the `finite` guard exists for.
    for (a, b, c) in [
        (f32::MAX, 2.0, 0.0),
        (f32::MAX, f32::MAX, -f32::MAX),
        (f32::INFINITY, 1.0, 1.0),
        (1.0, 1.0, f32::INFINITY),
        (f32::MAX, 1.0, f32::MAX),
        (-f32::MAX, 2.0, 0.0),
        (FUSED_WITNESS_A, FUSED_WITNESS_A, FUSED_WITNESS_C),
    ] {
        check(a, b, c, "overflow", &mut mismatches);
    }
    assert_eq!(mismatches, 0, "G3: {mismatches} edge-pool mismatches");
}

#[test]
fn g3_soft_fma_equals_hardware_fma_on_random_triples() {
    let mut random = Xorshift64Star::new(SEED);
    let mut mismatches = 0;
    for index in 0..RANDOM_TRIPLES {
        let a = random.next_mixed(index);
        let b = random.next_mixed(index + 1);
        let c = random.next_mixed(index);
        check(a, b, c, "random", &mut mismatches);
    }
    assert_eq!(
        mismatches, 0,
        "G3: {mismatches} mismatches in {RANDOM_TRIPLES} random triples"
    );
}

#[test]
fn g3_soft_fma_equals_hardware_fma_on_the_midpoint_family() {
    // An odd 25-bit product of two odd 13-bit significands is exactly an `f32` rounding midpoint,
    // which is where the direction of the round-to-odd step decides the result. A tiny `c` moves
    // the exact sum just off the midpoint in one direction or the other.
    let mut random = Xorshift64Star::new(SEED ^ 0x5555_5555_5555_5555);
    let mut mismatches = 0;
    for index in 0..MIDPOINT_TRIPLES {
        let significand_a = (random.next_u32() % (1 << 12)) | (1 << 12) | 1;
        let significand_b = (random.next_u32() % (1 << 12)) | (1 << 12) | 1;
        if u64::from(significand_a) * u64::from(significand_b) >= (1 << 25) {
            continue;
        }
        let exponent_a = (random.next_u32() % 40) as i32 - 20;
        let exponent_b = (random.next_u32() % 40) as i32 - 20;
        let a = scale(significand_a as f32, exponent_a - 12);
        let b = scale(significand_b as f32, exponent_b - 12);
        let product_exponent = exponent_a + exponent_b;
        let offsets = [-60, -50, -40, -35];
        let offset = offsets[(index + random.next_u32() as usize) % offsets.len()];
        let sign = if random.next_u32() & 1 == 0 {
            1.0
        } else {
            -1.0
        };
        let c = sign * scale(1.0, product_exponent + offset);
        check(a, b, c, "midpoint", &mut mismatches);
        check(a, b, -c, "midpoint", &mut mismatches);
    }
    assert_eq!(
        mismatches, 0,
        "G3: {mismatches} mismatches in the midpoint family"
    );
}

#[test]
fn g3_soft_fma_equals_hardware_fma_on_the_tie_family() {
    // `a` and `b` one unit in the last place apart from 1.0, with `c` chosen so that the sum lands
    // on or next to a tie. This is the family the unconditional `bits | 1` form fails.
    let epsilon = f32::EPSILON;
    let mut mismatches = 0;
    for k in 0..512u32 {
        for j in 0..512u32 {
            let a = 1.0 + (k as f32) * epsilon;
            let b = 1.0 + (j as f32) * epsilon;
            for m in [0.0f32, 1.0, 2.0, 3.0, 4096.0, 8_388_607.0] {
                for c in [
                    -(1.0 + m * epsilon),
                    -(2.0 + m * epsilon),
                    epsilon * epsilon * m,
                    -(a * b),
                ] {
                    check(a, b, c, "tie", &mut mismatches);
                }
            }
        }
    }
    assert_eq!(
        mismatches, 0,
        "G3: {mismatches} mismatches in the tie family"
    );
}

/// `value * 2^exponent`, exactly, without a transcendental call.
fn scale(value: f32, exponent: i32) -> f32 {
    let clamped = exponent.clamp(-126, 127);
    value * f32::from_bits(((clamped + 127) as u32) << 23)
}
