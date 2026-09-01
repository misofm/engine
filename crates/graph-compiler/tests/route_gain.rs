#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! #99 F4: the route gain coefficient is the workspace's deterministic dB->linear conversion.
//!
//! This file lives in `tests/` deliberately: it is the *oracle* side of the comparison and calls
//! the platform `f64::powf`, which `clippy.toml`'s `disallowed-methods` (formerly
//! `scripts/check-math-policy.sh`) exempts here per this file's top-of-file `#![allow]` and
//! forbids in `src/`.
//!
//! What the oracle proves and what it does not: agreement with the host's `powf` is a sanity
//! check that the replacement is the *same function*, not the acceptance gate. The gate F4 exists
//! for is master plan D5 -- the same session yields the same coefficient bits on x86_64, aarch64
//! and wasm32 -- and that is structural, because `math` is a vendored pure-Rust libm
//! with no target-conditional fast paths. `route_transform_uses_the_canonical_conversion` is the
//! test that pins the compiler to it.

/// Distance in representable `f32` steps. Both arguments are finite, normal and same-signed here.
fn ulp_gap(a: f32, b: f32) -> u32 {
    a.to_bits().abs_diff(b.to_bits())
}

fn oracle(db: f32) -> f32 {
    10_f64.powf(f64::from(db) / 20.0) as f32
}

/// Worst deviation from the f64 oracle over `[lo, hi]` dB in 0.25 dB steps, and where it occurs.
fn worst_gap(lo: f32, hi: f32) -> (u32, f32) {
    let (mut worst, mut worst_db) = (0_u32, lo);
    let mut db = lo;
    while db <= hi {
        let gap = ulp_gap(math::db_to_gain_f32(db), oracle(db));
        if gap > worst {
            worst = gap;
            worst_db = db;
        }
        db += 0.25;
    }
    (worst, worst_db)
}

/// `db_to_gain_f32` agrees with a `f64` `10^(db/20)` oracle, with a deviation that grows with
/// `|db|` in a way this test pins rather than hides.
///
/// Measured (x86_64, 0.25 dB steps): 2 ulp worst over [-24, +24], 5 over [-60, +24], 10 over
/// [-120, +24] at -115 dB. #99's plan predicted "<= 2 ulp" over the whole sweep; that is a **miss**,
/// and the cause is in the conversion, not in this crate: `db_to_gain_f32(db)` is
/// `exp2f(db * (LOG2_10 / 20))`, whose scaling multiply is evaluated in `f32`, so the absolute
/// error of the `exp2` argument grows proportionally to `|db|` and the result's relative error
/// grows with it (~0.06 ulp per dB, plus `exp2f`'s own ~1 ulp and the oracle's own double
/// rounding). 10 ulp at -115 dB is 1.2e-6 relative -- 1.0e-5 dB on a coefficient that is already
/// 120 dB down -- and it is the *same* 10 ulp on every target, which is the property F4 buys.
/// Tightening it would mean an `f64` scaling multiply inside `math` (#83b's crate,
/// not this one); recorded on #99 as a deferred observation rather than patched from here.
#[test]
fn route_gain_matches_f64_oracle_within_two_ulp() {
    let (mixing_range, mixing_db) = worst_gap(-24.0, 24.0);
    assert!(
        mixing_range <= 2,
        "db_to_gain_f32 deviates {mixing_range} ulp from the f64 oracle at {mixing_db} dB, \
         inside the +/-24 dB mixing range"
    );
    let (full_range, full_db) = worst_gap(-120.0, 24.0);
    assert!(
        full_range <= 10,
        "db_to_gain_f32 deviates {full_range} ulp from the f64 oracle at {full_db} dB"
    );
    for db in [-6.0_f32, -3.0, -0.1, 0.0, 0.1, 3.0, 6.0] {
        let gap = ulp_gap(math::db_to_gain_f32(db), oracle(db));
        assert!(gap <= 2, "db_to_gain_f32 deviates {gap} ulp at {db} dB");
    }
}

/// `db_to_gain_f32(0) = exp2f(0) = 1.0` exactly, so a 0 dB route keeps `0x3f80_0000` and every
/// checked-in graph fixture (all of whose routes are 0 dB) is byte-identical across F4.
#[test]
fn unity_route_gain_is_exactly_one() {
    assert_eq!(math::db_to_gain_f32(0.0).to_bits(), 0x3f80_0000);
    assert_eq!(math::db_to_gain_f32(-0.0).to_bits(), 0x3f80_0000);
}

/// The canonical conversion and the platform `powf` it replaced are observably different bits at
/// -19 dB, which is the witness `route_transform_uses_the_canonical_db_to_gain_conversion` (in
/// `src/lib.rs`, where `powf` is forbidden) pins as a literal. If this ever stops holding, that
/// test proves nothing and its witness must be re-derived from this sweep.
#[test]
fn the_replaced_platform_conversion_had_different_bits_at_the_pinned_witness() {
    assert_eq!(math::db_to_gain_f32(-19.0).to_bits(), 0x3de5_ca16);
    assert_eq!(oracle(-19.0).to_bits(), 0x3de5_ca15);
}
