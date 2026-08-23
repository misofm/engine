//! Deterministic transcendental math for the engine.
//!
//! Every target executes the same operation sequence, so a rendered result does not depend on
//! which host libm, which instruction set or which optimisation level built it. That is the whole
//! reason this crate exists: the platform `f32`/`f64` transcendental methods are *not* specified
//! to agree across targets, and the render path's determinism claim (master plan D5) does not
//! survive them. Master plan decision D6 therefore bans them in production crates outside this
//! one; `scripts/check-math-policy.sh` enforces the ban.
//!
//! Two layers:
//!
//! * **Scalar** ([`exp`] .. [`tanhf`]) — vendored from rust-lang/libm 0.2.16 (MIT), the pure-Rust
//!   musl port, with every target-conditional path and intrinsic fast path removed. Accuracy is
//!   musl's, at or below 1 ulp for the functions in this set. Used by the control plane (filter
//!   coefficient design, dB conversions at event rate) and by scalar render paths.
//! * **Lane** ([`exp2_lane`], [`log2_lane`], behind the `lane` feature) — `2^x` and `log2(x)`
//!   evaluated with `miso_engine_lane::Lane` basic operations only, for per-sample dB↔gain work
//!   inside a SIMD bank. Maximum error is proven below 2 ulp by an exhaustive sweep of all
//!   2^32 `f32` inputs (gate M1).
//!
//! The crate is `no_std`; `std` is used only by its tests.
//!
//! # Determinism
//!
//! No function in this crate branches on target features, calls an intrinsic, or fuses a multiply
//! and an add. Gate M3 pins that structurally (a source scan) and numerically (digests over a
//! one-million-point corpus, re-checked under wasmtime by job 83d).

#![no_std]

#[cfg(test)]
extern crate std;

#[macro_use]
mod vendored;

pub mod corpus;

/// `e^x` (f64).
#[inline]
pub fn exp(x: f64) -> f64 {
    vendored::exp(x)
}

/// `2^x` (f64).
#[inline]
pub fn exp2(x: f64) -> f64 {
    vendored::exp2(x)
}

/// `e^x - 1` (f64), accurate for small `x`.
#[inline]
pub fn expm1(x: f64) -> f64 {
    vendored::expm1(x)
}

/// Natural logarithm (f64).
#[inline]
pub fn log(x: f64) -> f64 {
    vendored::log(x)
}

/// Base-2 logarithm (f64).
#[inline]
pub fn log2(x: f64) -> f64 {
    vendored::log2(x)
}

/// Base-10 logarithm (f64).
///
/// Vendored rather than computed as `log2(x) * LOG10_2`: that identity is not bit-equal to musl's
/// `log10`, and the fixtures pin bits.
#[inline]
pub fn log10(x: f64) -> f64 {
    vendored::log10(x)
}

/// `x^y` (f64).
#[inline]
pub fn pow(x: f64, y: f64) -> f64 {
    vendored::pow(x, y)
}

/// Sine of `x` radians (f64).
#[inline]
pub fn sin(x: f64) -> f64 {
    vendored::sin(x)
}

/// Cosine of `x` radians (f64).
#[inline]
pub fn cos(x: f64) -> f64 {
    vendored::cos(x)
}

/// Tangent of `x` radians (f64).
#[inline]
pub fn tan(x: f64) -> f64 {
    vendored::tan(x)
}

/// Hyperbolic tangent (f64).
#[inline]
pub fn tanh(x: f64) -> f64 {
    vendored::tanh(x)
}

/// Arc tangent of `x`, in radians, in `[-pi/2, pi/2]` (f64).
#[inline]
pub fn atan(x: f64) -> f64 {
    vendored::atan(x)
}

/// Arc tangent of `y / x` using the signs of both arguments to select the quadrant (f64).
#[inline]
pub fn atan2(y: f64, x: f64) -> f64 {
    vendored::atan2(y, x)
}

/// `e^x` (f32).
#[inline]
pub fn expf(x: f32) -> f32 {
    vendored::expf(x)
}

/// `2^x` (f32).
#[inline]
pub fn exp2f(x: f32) -> f32 {
    vendored::exp2f(x)
}

/// `e^x - 1` (f32), accurate for small `x`.
#[inline]
pub fn expm1f(x: f32) -> f32 {
    vendored::expm1f(x)
}

/// Natural logarithm (f32).
#[inline]
pub fn logf(x: f32) -> f32 {
    vendored::logf(x)
}

/// Base-2 logarithm (f32).
#[inline]
pub fn log2f(x: f32) -> f32 {
    vendored::log2f(x)
}

/// Base-10 logarithm (f32).
#[inline]
pub fn log10f(x: f32) -> f32 {
    vendored::log10f(x)
}

/// `x^y` (f32).
#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    vendored::powf(x, y)
}

/// Sine of `x` radians (f32).
#[inline]
pub fn sinf(x: f32) -> f32 {
    vendored::sinf(x)
}

/// Cosine of `x` radians (f32).
#[inline]
pub fn cosf(x: f32) -> f32 {
    vendored::cosf(x)
}

/// Tangent of `x` radians (f32).
#[inline]
pub fn tanf(x: f32) -> f32 {
    vendored::tanf(x)
}

/// Hyperbolic tangent (f32).
#[inline]
pub fn tanhf(x: f32) -> f32 {
    vendored::tanhf(x)
}

/// Largest integer not greater than `x` (f64).
///
/// Exact on every target, and not a transcendental: exported because `core` does not provide it
/// and `no_std` consumers of this crate need it for the same reason this crate does.
#[inline]
pub fn floor(x: f64) -> f64 {
    vendored::floor(x)
}

/// Largest integer not greater than `x` (f32). See [`floor`].
#[inline]
pub fn floorf(x: f32) -> f32 {
    vendored::floorf(x)
}

/// Correctly rounded square root (f64).
///
/// IEEE 754 specifies `sqrt` exactly, so this agrees with hardware `sqrt` on every target; it
/// exists because `core` has no `f64::sqrt`.
#[inline]
pub fn sqrt(x: f64) -> f64 {
    vendored::sqrt(x)
}

/// Correctly rounded square root (f32). See [`sqrt`].
#[inline]
pub fn sqrtf(x: f32) -> f32 {
    vendored::sqrtf(x)
}

/// Amplitude gain for a level in decibels (f64): `10^(db/20)`, evaluated as `2^(db * log2(10)/20)`.
///
/// This is the canonical dB→gain conversion for the engine. Using one spelling everywhere is what
/// makes coefficient bits reproducible across crates.
#[inline]
pub fn db_to_gain(db: f64) -> f64 {
    exp2(db * (core::f64::consts::LOG2_10 / 20.0))
}

/// Level in decibels for an amplitude gain (f64): `20 * log10(g)`, evaluated as
/// `log2(g) * 20 * log10(2)`. Inverse of [`db_to_gain`] to within rounding.
#[inline]
pub fn gain_to_db(gain: f64) -> f64 {
    log2(gain) * (20.0 * core::f64::consts::LOG10_2)
}

/// Amplitude gain for a level in decibels (f32). See [`db_to_gain`].
#[inline]
pub fn db_to_gain_f32(db: f32) -> f32 {
    exp2f(db * (core::f32::consts::LOG2_10 / 20.0))
}

/// Level in decibels for an amplitude gain (f32). See [`gain_to_db`].
#[inline]
pub fn gain_to_db_f32(gain: f32) -> f32 {
    log2f(gain) * (20.0 * core::f32::consts::LOG10_2)
}
