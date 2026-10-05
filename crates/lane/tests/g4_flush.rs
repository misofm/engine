//! Gate G4: the flush law.
//!
//! Master plan #83 D7 and §3.6. `flush(x)` maps every `|x| < FLUSH_EPS` — which includes every
//! subnormal and `-0.0` — to exactly `+0.0`, leaves every `|x| >= FLUSH_EPS` unchanged bit for
//! bit, and passes NaN and infinity through. The same law holds at every width, since `flush` is
//! written once, generic over `Lane`.
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): `lt` becomes `le` in `flush`,
//! which fails at `x = +-FLUSH_EPS`.
//!
//! The pair law (issue #1328, amendment A8): `flush_pair(n1, n2, x)` applies the per-word law to
//! each word and, when the input `x` is exactly zero (either sign) *and* both magnitudes are below
//! `REST_EPS`, zeroes the pair together. A non-zero input -- a subnormal, a NaN or an infinity
//! included -- or one word at or above `REST_EPS` leaves both words on the per-word law bit for
//! bit; a NaN in either word passes through; `-0.0` becomes `+0.0`. Mutation evidence is in the
//! issue's attempt record.

mod support;

use lane::{FLUSH_EPS, Lane, REST_EPS, flush, flush_pair};
use support::Xorshift64Star;

/// Step through the subnormal range. The `--release` run is exhaustive (every one of the 2^23
/// subnormals of each sign); a debug run strides, to keep the workspace suite quick.
const SUBNORMAL_STRIDE: u32 = if cfg!(debug_assertions) { 1_021 } else { 1 };

/// Random normals swept per width.
const RANDOM_NORMALS: usize = if cfg!(debug_assertions) {
    20_000
} else {
    1_000_000
};

/// Applies `flush` at one width and returns the result bits of lane 0.
fn flush_bits<L: Lane>(value: f32) -> u32 {
    let mut bits = [0u32; 8];
    flush(L::splat(value)).store_bits(&mut bits);
    bits[0]
}

/// Asserts the flush law for one value at one width.
fn check<L: Lane>(width_name: &str, value: f32) {
    let actual = flush_bits::<L>(value);
    let magnitude = f32::from_bits(value.to_bits() & 0x7FFF_FFFF);
    if magnitude < FLUSH_EPS {
        assert_eq!(
            actual, 0x0000_0000,
            "{width_name}: flush({value:e}) must be +0.0, got {actual:#010x}"
        );
    } else {
        assert_eq!(
            actual,
            value.to_bits(),
            "{width_name}: flush({value:e}) must be unchanged"
        );
    }
}

/// Sweeps one width over the whole corpus.
fn sweep<L: Lane>(width_name: &str) {
    for value in [
        0.0f32,
        -0.0,
        FLUSH_EPS,
        -FLUSH_EPS,
        f32::from_bits(FLUSH_EPS.to_bits() + 1),
        f32::from_bits(FLUSH_EPS.to_bits() - 1),
        f32::from_bits((FLUSH_EPS.to_bits() + 1) | 0x8000_0000),
        f32::from_bits((FLUSH_EPS.to_bits() - 1) | 0x8000_0000),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1.0,
        -1.0,
        f32::MAX,
        f32::MIN,
    ] {
        check::<L>(width_name, value);
    }

    // Infinity and NaN pass through untouched: `abs(NaN) < eps` is false under the ordered
    // comparison, so a NaN reaches the once-per-block boundary check instead of being hidden here.
    for value in [f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            flush_bits::<L>(value),
            value.to_bits(),
            "{width_name}: flush({value:e}) must pass infinity through"
        );
    }
    for bits in [0x7FC0_0000u32, 0xFFC0_0001, 0x7F80_0001] {
        assert!(
            f32::from_bits(flush_bits::<L>(f32::from_bits(bits))).is_nan(),
            "{width_name}: flush of NaN {bits:#010x} must stay NaN"
        );
    }

    let mut bits = 1u32;
    while bits < 0x0080_0000 {
        check::<L>(width_name, f32::from_bits(bits));
        check::<L>(width_name, f32::from_bits(bits | 0x8000_0000));
        bits += SUBNORMAL_STRIDE;
    }

    let mut random = Xorshift64Star::new(0x0F1E_2D3C_4B5A_6978);
    for _ in 0..RANDOM_NORMALS {
        let value = random.next_moderate();
        check::<L>(width_name, value);
    }
}

#[test]
fn g4_flush_law_holds_at_every_width() {
    lane::each_lane!(|L| sweep::<L>(core::any::type_name::<L>()));
}

#[test]
fn g4_flush_is_lane_wise() {
    // Mixed lanes: only the lanes below the threshold are cleared, and the others keep their bits.
    let lanes = [
        1.0f32,
        1.0e-30,
        -1.0e-30,
        -2.0,
        f32::from_bits(1),
        FLUSH_EPS,
        -FLUSH_EPS,
        0.5,
    ];
    let mut bits = [0u32; 8];
    // At the build's own width, one vector per `WIDTH` lanes: eight lanes in one where `avx2` is
    // enabled, two of four in a 4-lane (NEON/simd128) build (issue #1112).
    let width = <lane::Native as Lane>::WIDTH;
    for (input, output) in lanes.chunks(width).zip(bits.chunks_mut(width)) {
        flush(lane::Native::load(input)).store_bits(output);
    }
    let expected = [
        1.0f32.to_bits(),
        0,
        0,
        (-2.0f32).to_bits(),
        0,
        FLUSH_EPS.to_bits(),
        (-FLUSH_EPS).to_bits(),
        0.5f32.to_bits(),
    ];
    assert_eq!(bits, expected, "G4: flush must act lane by lane");
}

/// The pair law restated on plain `f32` comparisons, independent of `Lane`: the oracle the vector
/// widths are held to bit for bit.
fn pair_oracle(n1: f32, n2: f32, x: f32) -> (u32, u32) {
    let rest = x == 0.0 && n1.abs() < REST_EPS && n2.abs() < REST_EPS;
    let word = |n: f32| {
        if rest || n.abs() < FLUSH_EPS {
            0
        } else {
            n.to_bits()
        }
    };
    (word(n1), word(n2))
}

/// Runs `flush_pair` lane-wise at one width over `(n1, n2, x)` triples, `L::WIDTH` per vector.
fn pair_bits<L: Lane>(pairs: &[(f32, f32, f32)]) -> Vec<(u32, u32)> {
    let mut out = Vec::with_capacity(pairs.len());
    for chunk in pairs.chunks(L::WIDTH) {
        let mut first = vec![0.0f32; L::WIDTH];
        let mut second = vec![0.0f32; L::WIDTH];
        let mut input = vec![0.0f32; L::WIDTH];
        for (lane, &(n1, n2, x)) in chunk.iter().enumerate() {
            first[lane] = n1;
            second[lane] = n2;
            input[lane] = x;
        }
        let (ic1, ic2) = flush_pair(L::load(&first), L::load(&second), L::load(&input));
        let (mut bits1, mut bits2) = ([0u32; 8], [0u32; 8]);
        ic1.store_bits(&mut bits1);
        ic2.store_bits(&mut bits2);
        out.extend((0..chunk.len()).map(|lane| (bits1[lane], bits2[lane])));
    }
    out
}

/// Magnitudes around both thresholds, with both signs and both zeros.
fn pair_edges() -> Vec<f32> {
    let mut edges = Vec::new();
    for magnitude in [
        0.0f32,
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        f32::from_bits(FLUSH_EPS.to_bits() - 1),
        FLUSH_EPS,
        f32::from_bits(FLUSH_EPS.to_bits() + 1),
        1.0e-17,
        f32::from_bits(REST_EPS.to_bits() - 1),
        REST_EPS,
        f32::from_bits(REST_EPS.to_bits() + 1),
        5.0e-15,
        1.0e-6,
        1.0,
        f32::MAX,
        f32::INFINITY,
    ] {
        edges.push(magnitude);
        edges.push(-magnitude);
    }
    edges
}

/// One random word whose exponent straddles both thresholds (`2^-90 .. 2^-30`), or, one time in
/// eight, an arbitrary bit pattern (NaN, infinity, subnormal, huge).
fn pair_word(random: &mut Xorshift64Star) -> f32 {
    let bits = random.next_u32();
    if bits & 7 == 0 {
        return random.next_bit_pattern();
    }
    let exponent = ((bits >> 3) % 60) + 127 - 90;
    f32::from_bits((bits & 0x8000_0000) | (exponent << 23) | (random.next_u32() & 0x007F_FFFF))
}

/// Inputs around the one that opens the pair rule: both zeros, the smallest subnormal, the
/// smallest normal, a tiny and an ordinary normal, an infinity and a NaN.
const PAIR_INPUTS: [f32; 9] = [
    0.0,
    -0.0,
    f32::from_bits(1),
    -f32::MIN_POSITIVE,
    1.0e-30,
    -0.5,
    f32::INFINITY,
    f32::NAN,
    3.0e-11,
];

/// One random input: zero of either sign half the time (so the pair rule is reached), otherwise
/// a word from [`pair_word`].
fn pair_input(random: &mut Xorshift64Star) -> f32 {
    match random.next_u32() & 3 {
        0 => 0.0,
        1 => -0.0,
        _ => pair_word(random),
    }
}

fn pair_sweep<L: Lane>(width_name: &str) {
    let edges = pair_edges();
    let mut pairs: Vec<(f32, f32, f32)> = edges
        .iter()
        .flat_map(|&n1| edges.iter().map(move |&n2| (n1, n2)))
        .flat_map(|(n1, n2)| PAIR_INPUTS.iter().map(move |&x| (n1, n2, x)))
        .collect();
    let mut random = Xorshift64Star::new(0x1328_FA17_0000_0001);
    pairs.extend((0..RANDOM_NORMALS).map(|_| {
        (
            pair_word(&mut random),
            pair_word(&mut random),
            pair_input(&mut random),
        )
    }));
    let actual = pair_bits::<L>(&pairs);
    for (&(n1, n2, x), &(ic1, ic2)) in pairs.iter().zip(&actual) {
        assert_eq!(
            (ic1, ic2),
            pair_oracle(n1, n2, x),
            "{width_name}: flush_pair({n1:e}, {n2:e}, {x:e}) breaks the pair law"
        );
    }
}

#[test]
fn g4_pair_law_holds_at_every_width() {
    lane::each_lane!(|L| pair_sweep::<L>(core::any::type_name::<L>()));
}

fn pair_cases<L: Lane>(width_name: &str) {
    let below = f32::from_bits(REST_EPS.to_bits() - 1);
    let small_pairs = [(below, -below), (-5.0e-15, 2.0e-20), (1.35e-16, -2.0e-21)];
    // Both words below `REST_EPS`, each above `FLUSH_EPS`, on a silent input (either zero): the
    // per-word law alone keeps them, the pair rule zeroes both.
    for x in [0.0f32, -0.0] {
        for (n1, n2) in small_pairs {
            assert_eq!(
                pair_bits::<L>(&[(n1, n2, x)]),
                [(0, 0)],
                "{width_name}: both below REST_EPS on input {x:e} must both become +0.0"
            );
        }
    }
    // The same pairs on a non-zero input -- the smallest subnormal of either sign, the smallest
    // normal, a tiny and an ordinary normal, an infinity, a NaN -- keep the per-word law bit for
    // bit (amendment A8): a section driven by any non-zero input applies its whole response.
    for x in [
        f32::from_bits(1),
        f32::from_bits(0x8000_0001),
        f32::MIN_POSITIVE,
        3.0e-11,
        -1.0,
        f32::NEG_INFINITY,
        f32::NAN,
    ] {
        for (n1, n2) in small_pairs {
            let expected = (flush(n1).to_bits(), flush(n2).to_bits());
            assert_eq!(
                pair_bits::<L>(&[(n1, n2, x)]),
                [expected],
                "{width_name}: flush_pair({n1:e}, {n2:e}) on input {x:e} must follow the \
                 per-word law"
            );
        }
    }
    // One word at or above `REST_EPS`, on a silent input: each word keeps the per-word law, so a
    // small partner word is kept and a word below `FLUSH_EPS` is still flushed.
    for (n1, n2) in [
        (REST_EPS, below),
        (-below, -REST_EPS),
        (0.5, 5.0e-15),
        (6.0e-20, 0.25),
        (1.0, 1.0e-21),
    ] {
        let expected = (flush(n1).to_bits(), flush(n2).to_bits());
        assert_eq!(
            pair_bits::<L>(&[(n1, n2, 0.0)]),
            [expected],
            "{width_name}: flush_pair({n1:e}, {n2:e}, 0) must follow the per-word law"
        );
    }
    // NaN in either word passes through both rules, and its partner follows the per-word law.
    for nan in [f32::NAN, f32::from_bits(0xFFC0_0001)] {
        for partner in [0.0f32, 1.0e-21, 5.0e-15, 0.5] {
            let [(ic1, ic2)] = pair_bits::<L>(&[(nan, partner, 0.0)])[..] else {
                unreachable!()
            };
            assert_eq!(
                ic1,
                nan.to_bits(),
                "{width_name}: a NaN first word must pass"
            );
            assert_eq!(ic2, flush(partner).to_bits(), "{width_name}: NaN's partner");
            let [(ic1, ic2)] = pair_bits::<L>(&[(partner, nan, 0.0)])[..] else {
                unreachable!()
            };
            assert_eq!(
                ic2,
                nan.to_bits(),
                "{width_name}: a NaN second word must pass"
            );
            assert_eq!(ic1, flush(partner).to_bits(), "{width_name}: NaN's partner");
        }
    }
    // `-0.0` becomes `+0.0` in either position, alone or beside a kept word, on either input.
    for x in [0.0f32, 0.25] {
        for (n1, n2) in [(-0.0f32, -0.0f32), (-0.0, 0.5), (0.5, -0.0)] {
            let [(ic1, ic2)] = pair_bits::<L>(&[(n1, n2, x)])[..] else {
                unreachable!()
            };
            assert_eq!(
                ic1,
                if n1 == 0.5 { n1.to_bits() } else { 0 },
                "{width_name}: -0.0"
            );
            assert_eq!(
                ic2,
                if n2 == 0.5 { n2.to_bits() } else { 0 },
                "{width_name}: -0.0"
            );
        }
    }
}

#[test]
fn g4_pair_law_cases_at_every_width() {
    lane::each_lane!(|L| pair_cases::<L>(core::any::type_name::<L>()));
}
