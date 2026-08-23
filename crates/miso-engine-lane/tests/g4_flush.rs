//! Gate G4: the flush law.
//!
//! Master plan #83 D7 and §3.6. `flush(x)` maps every `|x| < FLUSH_EPS` — which includes every
//! subnormal and `-0.0` — to exactly `+0.0`, leaves every `|x| >= FLUSH_EPS` unchanged bit for
//! bit, and passes NaN and infinity through. The same law holds at every width, since `flush` is
//! written once, generic over `Lane`.
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): `lt` becomes `le` in `flush`,
//! which fails at `x = +-FLUSH_EPS`.

mod support;

use miso_engine_lane::{FLUSH_EPS, Lane, flush};
use miso_engine_lane::{Simd4, Simd8};
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
    sweep::<f32>("f32");
    sweep::<Simd4>("Simd4");
    sweep::<Simd8>("Simd8");
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
    flush(Simd8::load(&lanes)).store_bits(&mut bits);
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
