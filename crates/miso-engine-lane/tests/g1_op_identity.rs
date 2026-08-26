//! Gate G1: every lane operation is the scalar operation, lane by lane, bit for bit.
//!
//! Master plan #83 §3.6. For every operation of the `Lane` surface, the `wide::f32x4` and
//! `wide::f32x8` results are compared with the scalar oracle over a directed edge pool (both
//! operand orders, signed zeros, both NaN payloads, subnormals, exponent boundaries, and the
//! triple where a fused multiply-add differs from a multiply followed by an add) and over seeded
//! pseudo-random vectors.
//!
//! Red-mutations proven for this gate (see `tests/MUTATIONS.md`):
//! * `Lane::select` forwarded to `wide`'s sign-bit `select` instead of `bitselect`.
//! * `Lane::max`/`min` forwarded to `wide`'s `max`/`min` instead of the D8 default.
//! * `Lane::neg` written as `zero - x`.

mod support;

use miso_engine_lane::Lane;
use miso_engine_lane::{Simd4, Simd8};
use support::{
    ALL_OPS, EDGES, FUSED_WITNESS_A, FUSED_WITNESS_C, MAX_WIDTH, Op, Xorshift64Star, apply,
    run_op_bits,
};

/// Seed of every random sweep in this gate.
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

/// Random vectors per cheap operation. The `--release` count is the gate; a debug run keeps the
/// workspace test suite usable and still sweeps the whole directed pool.
const RANDOM_VECTORS: usize = if cfg!(debug_assertions) {
    20_000
} else {
    1_000_000
};

/// Random vectors for `div` and `sqrt`, which are an order of magnitude slower.
const RANDOM_VECTORS_SLOW: usize = if cfg!(debug_assertions) {
    2_000
} else {
    100_000
};

/// Compares one corpus at `L` against the scalar oracle, reporting the first five differences.
fn compare<L: Lane>(op: Op, width_name: &str, a: &[f32], b: &[f32], c: &[f32]) {
    let mut oracle = vec![0u32; a.len()];
    let mut actual = vec![0u32; a.len()];
    run_op_bits::<f32>(op, a, b, c, &mut oracle);
    run_op_bits::<L>(op, a, b, c, &mut actual);
    let mut reported = 0;
    let mut differences = 0;
    for index in 0..a.len() {
        if oracle[index] != actual[index] {
            differences += 1;
            if reported < 5 {
                reported += 1;
                eprintln!(
                    "G1 {op} at {width_name}: lane {index}: a={a:#010x} b={b:#010x} c={c:#010x} \
                     oracle={oracle:#010x} actual={actual:#010x}",
                    op = op.name(),
                    a = a[index].to_bits(),
                    b = b[index].to_bits(),
                    c = c[index].to_bits(),
                    oracle = oracle[index],
                    actual = actual[index],
                );
            }
        }
    }
    assert_eq!(
        differences,
        0,
        "G1: {} differs from the scalar oracle at {} in {} of {} lanes",
        op.name(),
        width_name,
        differences,
        a.len()
    );
}

/// Builds the directed pool for one operation: every ordered pair (and, for ternary operations,
/// every ordered pair against a rotating third operand) of [`EDGES`].
fn directed_pool(op: Op) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let (mut a, mut b, mut c) = (Vec::new(), Vec::new(), Vec::new());
    for (left_index, left) in EDGES.iter().enumerate() {
        for (right_index, right) in EDGES.iter().enumerate() {
            for third_offset in 0..3 {
                let third = EDGES[(left_index + right_index + third_offset * 7) % EDGES.len()];
                if op.nan_payload_sensitive() {
                    let nan_operands = usize::from(left.is_nan())
                        + usize::from(right.is_nan())
                        + if op.arity() == 3 {
                            usize::from(third.is_nan())
                        } else {
                            0
                        };
                    if nan_operands > 1 {
                        continue;
                    }
                }
                a.push(*left);
                b.push(*right);
                c.push(third);
                if op.arity() < 3 {
                    break;
                }
            }
        }
    }
    // The fused-versus-unfused witness, in every operand order that matters.
    for triple in [
        (FUSED_WITNESS_A, FUSED_WITNESS_A, FUSED_WITNESS_C),
        (FUSED_WITNESS_A, FUSED_WITNESS_C, FUSED_WITNESS_A),
        (FUSED_WITNESS_C, FUSED_WITNESS_A, FUSED_WITNESS_A),
    ] {
        a.push(triple.0);
        b.push(triple.1);
        c.push(triple.2);
    }
    while a.len() % MAX_WIDTH != 0 {
        a.push(0.0);
        b.push(0.0);
        c.push(0.0);
    }
    (a, b, c)
}

/// Builds a seeded random corpus of `vectors * MAX_WIDTH` lanes.
fn random_pool(vectors: usize, seed: u64) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let lanes = vectors * MAX_WIDTH;
    let mut random = Xorshift64Star::new(seed);
    let mut a = Vec::with_capacity(lanes);
    let mut b = Vec::with_capacity(lanes);
    let mut c = Vec::with_capacity(lanes);
    for index in 0..lanes {
        a.push(random.next_mixed(index));
        b.push(random.next_mixed(index + 1));
        c.push(random.next_mixed(index));
    }
    (a, b, c)
}

#[test]
fn g1_directed_edge_pool_is_lane_identical() {
    for op in ALL_OPS {
        let (a, b, c) = directed_pool(*op);
        compare::<Simd4>(*op, "Simd4", &a, &b, &c);
        compare::<Simd8>(*op, "Simd8", &a, &b, &c);
    }
}

#[test]
fn g1_random_vectors_are_lane_identical() {
    for op in ALL_OPS {
        let vectors = if matches!(op, Op::Div | Op::Sqrt) {
            RANDOM_VECTORS_SLOW
        } else {
            RANDOM_VECTORS
        };
        let (a, b, c) = random_pool(vectors, SEED ^ (op.name().len() as u64));
        compare::<Simd4>(*op, "Simd4", &a, &b, &c);
        compare::<Simd8>(*op, "Simd8", &a, &b, &c);
    }
}

#[test]
fn g1_signed_zero_max_and_min_follow_d8() {
    /// `max(-0.0, +0.0)` is `+0.0` and `max(+0.0, -0.0)` is `-0.0`: `max` returns its second
    /// operand on equal lanes, and the two zeros compare equal.
    fn check<L: Lane>(width_name: &str) {
        let plus = L::splat(0.0);
        let minus = L::splat(-0.0);
        let mut bits = [0u32; 8];
        L::max(minus, plus).store_bits(&mut bits);
        assert_eq!(bits[0], 0x0000_0000, "{width_name}: max(-0.0, +0.0)");
        L::max(plus, minus).store_bits(&mut bits);
        assert_eq!(bits[0], 0x8000_0000, "{width_name}: max(+0.0, -0.0)");
        L::min(minus, plus).store_bits(&mut bits);
        assert_eq!(bits[0], 0x0000_0000, "{width_name}: min(-0.0, +0.0)");
        L::min(plus, minus).store_bits(&mut bits);
        assert_eq!(bits[0], 0x8000_0000, "{width_name}: min(+0.0, -0.0)");
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}

#[test]
fn g1_nan_max_and_min_follow_d8() {
    /// `max(NaN, x)` is `x` (the comparison is false, so the second operand wins) and
    /// `max(x, NaN)` is NaN. An IEEE `maximum` would answer the other way round on one of these.
    fn check<L: Lane>(width_name: &str) {
        let mut bits = [0u32; 8];
        L::max(L::splat(f32::NAN), L::splat(1.0)).store_bits(&mut bits);
        assert_eq!(bits[0], 1.0f32.to_bits(), "{width_name}: max(NaN, 1.0)");
        L::max(L::splat(1.0), L::splat(f32::NAN)).store_bits(&mut bits);
        assert!(
            f32::from_bits(bits[0]).is_nan(),
            "{width_name}: max(1.0, NaN)"
        );
        L::min(L::splat(f32::NAN), L::splat(1.0)).store_bits(&mut bits);
        assert_eq!(bits[0], 1.0f32.to_bits(), "{width_name}: min(NaN, 1.0)");
        L::min(L::splat(1.0), L::splat(f32::NAN)).store_bits(&mut bits);
        assert!(
            f32::from_bits(bits[0]).is_nan(),
            "{width_name}: min(1.0, NaN)"
        );
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}

#[test]
fn g1_fma_is_unfused_on_every_backend() {
    /// `(1 + 2^-12)^2 - (1 + 2^-11)` is `2^-24` with one rounding and exactly `0` with two.
    ///
    /// That makes this triple a *witness*: it separates the two contracts by a value, not by a
    /// digest, so it says which arithmetic ran without depending on any pinned corpus. Before
    /// issue #163 phase 2 the assertion ran the other way and proved `Lane::fma` was fused. It now
    /// proves the opposite, and it is the sharpest gate the phase installs -- a backend that
    /// quietly regained a hardware fused multiply-add fails here immediately, at every width,
    /// with no re-pin able to hide it.
    ///
    /// The unfused answer is `+0.0` exactly, so the check is on bits: a `-0.0` would compare equal
    /// under `==` and would mean the addition had taken a different path.
    fn check<L: Lane>(width_name: &str) {
        let a = L::splat(FUSED_WITNESS_A);
        let c = L::splat(FUSED_WITNESS_C);
        let mut through_fma = [0u32; 8];
        let mut written_out = [0u32; 8];
        a.fma(a, c).store_bits(&mut through_fma);
        a.mul(a).add(c).store_bits(&mut written_out);
        assert_eq!(
            through_fma[0], 0x0000_0000,
            "{width_name}: Lane::fma must be unfused, so this witness is exactly +0.0 \
             (got {:#010x}; {:#010x} would mean a fused multiply-add came back)",
            through_fma[0], 0x3380_0000_u32
        );
        assert_eq!(
            written_out[0], through_fma[0],
            "{width_name}: Lane::fma must equal the written-out multiply and add"
        );
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}

#[test]
fn g1_mask_any_matches_the_oracle() {
    /// `mask_any` is the one operation that leaves the vector domain, so it is checked directly
    /// rather than through `select`.
    fn check<L: Lane>(width_name: &str) {
        let mut lanes = vec![0.0f32; L::WIDTH];
        let all_below = L::load(&lanes);
        assert!(
            !L::mask_any(all_below.gt(L::splat(0.0))),
            "{width_name}: no lane is greater than zero"
        );
        lanes[L::WIDTH - 1] = 1.0;
        let one_above = L::load(&lanes);
        assert!(
            L::mask_any(one_above.gt(L::splat(0.0))),
            "{width_name}: the last lane is greater than zero"
        );
        assert!(
            !L::mask_any(L::splat(f32::NAN).gt(L::splat(0.0))),
            "{width_name}: an unordered comparison sets no lane"
        );
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}

#[test]
fn g1_exp2_int_is_exact_on_the_integer_range() {
    /// `exp2_int` builds the exponent field directly, so it is exact for every integer in
    /// `[-126, 127]` and clamps outside it (NaN clamps to the low end).
    fn check<L: Lane>(width_name: &str) {
        for exponent in -126..=127 {
            let mut bits = [0u32; 8];
            L::exp2_int(L::splat(exponent as f32)).store_bits(&mut bits);
            let expected = f32::from_bits(((exponent + 127) as u32) << 23);
            assert_eq!(
                bits[0],
                expected.to_bits(),
                "{width_name}: exp2_int({exponent})"
            );
        }
        let mut bits = [0u32; 8];
        L::exp2_int(L::splat(f32::NAN)).store_bits(&mut bits);
        assert_eq!(
            bits[0],
            f32::from_bits(1u32 << 23).to_bits(),
            "{width_name}: exp2_int(NaN) clamps to 2^-126"
        );
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}

#[test]
fn g1_frexp_reconstructs_positive_normals() {
    /// `frexp` splits a positive normal into a significand in `[1, 2)` and an exact exponent.
    fn check<L: Lane>(width_name: &str) {
        let mut random = Xorshift64Star::new(SEED);
        for _ in 0..1_000 {
            let value =
                f32::from_bits((random.next_u32() & 0x7FFF_FFFF).clamp(1 << 23, 0x7F00_0000));
            let (significand, exponent) = L::frexp(L::splat(value));
            let mut significand_bits = [0u32; 8];
            let mut exponent_bits = [0u32; 8];
            significand.store_bits(&mut significand_bits);
            exponent.store_bits(&mut exponent_bits);
            let m = f32::from_bits(significand_bits[0]);
            let e = f32::from_bits(exponent_bits[0]);
            assert!(
                (1.0..2.0).contains(&m),
                "{width_name}: significand of {value}"
            );
            let mut reconstructed = [0u32; 8];
            apply::<L>(Op::Exp2Int, L::splat(e), L::zero(), L::zero())
                .mul(L::splat(m))
                .store_bits(&mut reconstructed);
            assert_eq!(
                reconstructed[0],
                value.to_bits(),
                "{width_name}: frexp round trip of {value}"
            );
        }
    }
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}
