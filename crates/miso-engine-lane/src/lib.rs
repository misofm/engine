//! `Lane`: the workspace SIMD foundation and its pinned per-operation numeric contract.
//!
//! This crate is the single home of every SIMD vocabulary type, of `mul_add`, and of the software
//! FMA used where hardware FMA does not exist. Everything else in the workspace is written once,
//! generic over [`Lane`], and instantiated per width; lane identity is therefore a property of the
//! code rather than of a fixture corpus (master plan for issue #83, §1 and §3).
//!
//! # What is pinned
//!
//! * The semantics of every operation (master plan §3.2). The scalar [`Lane`] implementation for
//!   [`prim@f32`] is the oracle and is written operation by operation to that table, never to a
//!   `std` convenience: `f32::max`, for example, has a different NaN rule than decision D8 and is
//!   forbidden on any render path.
//! * `max`/`min` are `select(a > b, a, b)` / `select(a < b, a, b)` (D8), provided as trait defaults
//!   so that no backend can substitute an IEEE `maximum`.
//! * Fusion exists only where [`Lane::fma`] is written (D3). Rust never contracts `a * b + c`, so
//!   this is mechanically checkable: `scripts/check-lane-policy.sh` fails if `mul_add` appears
//!   outside this crate.
//! * Denormal handling is one mechanism on every target: [`flush`] with [`FLUSH_EPS`] (D7).
//!
//! # Backends
//!
//! Exactly three (D4): `Scalar` ([`prim@f32`], `WIDTH = 1`, the oracle), `Simd4`
//! ([`wide::f32x4`]) and `Simd8` ([`wide::f32x8`]). The operation bodies come from the `wide`
//! crate, which selects its own backend at **compile time**, so there is no runtime SIMD dispatch:
//! native `x86_64` builds are pinned to `x86-64-v3` by `.cargo/config.toml`, this crate refuses to
//! compile on `x86` without `avx2` and `fma`, and a host attests the CPU once at boot with
//! [`attest_host`]. `wide` is a vocabulary, not a semantics authority: its `max`, `min` and
//! `mul_add` differ per target and are never forwarded.
//!
//! # Realtime rules
//!
//! Every operation and every kernel body is `#[inline(always)]`, allocation-free, branch-free per
//! sample, and validated with `debug_assert!` only: block shapes are validated once at plan
//! preparation, never per block on the render thread.

#![no_std]
// `std` is used for exactly three things, none of them on a render path: the `f32::floor`,
// `f32::sqrt` and `f32::mul_add` inherent methods (not available in `core` on the pinned
// toolchain), and `is_x86_feature_detected!` inside `attest_host`. The crate stays `#![no_std]` so
// that `Vec`, `String` and the rest of the allocating prelude are not reachable by accident.
extern crate std;

// Master plan #83 D4: native x86 is x86-64-v3, pinned at compile time by `.cargo/config.toml` and
// attested at boot by `attest_host`. Without the pin, `wide` silently lowers `f32x8` to two SSE2
// `m128` values and its `mul_add` to an unfused `(a * b) + c`, which would break D3 and D5 without
// any test noticing. Refusing to compile is the guard (master plan §11).
// `cfg(doc)` is excluded because `rustdoc` does not receive `[target.*] rustflags` from
// `.cargo/config.toml`, and `RUSTDOCFLAGS` on the command line replaces any `rustdocflags` set
// there. Documentation is not a build artefact, so the guard has nothing to protect in that pass.
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(all(target_feature = "avx2", target_feature = "fma")),
    not(doc)
))]
compile_error!(
    "miso-engine-lane requires x86-64-v3: build with -C target-feature=+avx2,+fma (the workspace \
     .cargo/config.toml sets this). Master plan #83 D4: there is no runtime SIMD dispatch and no \
     silent scalar fallback."
);

mod backend;
mod bits;
pub mod kernels;
mod scalar;
mod simd4;
mod simd8;
pub mod softfma;
mod wide_impl;

pub use backend::{Backend, HostAttestation, attest_host};

/// Magnitude below which a recursive state word is flushed to `+0.0` by [`flush`].
///
/// `1.0e-20` is about `2^-66`; `f32` subnormals start at `2^-126`, so the flush band strictly
/// contains the band hardware FTZ/DAZ acts on. Flushing is therefore the only denormal mechanism
/// that matters, and hardware FTZ is provably inert and never load-bearing — which is what makes
/// the same bits come out of an AudioWorklet (Chrome forces FTZ) and a native render (D7).
pub const FLUSH_EPS: f32 = 1.0e-20;

/// Flushes lanes whose magnitude is below [`FLUSH_EPS`] to exactly `+0.0`.
///
/// `flush(x) = andnot(abs(x) < FLUSH_EPS, x)` (D7): three operations, applied to each recursive
/// state word once per sample inside the kernel. NaN passes through unchanged (`abs(NaN) < eps` is
/// false under the ordered compare) and is caught by the once-per-block boundary check; `-0.0`
/// becomes `+0.0`.
#[inline(always)]
pub fn flush<L: Lane>(x: L) -> L {
    x.andnot(x.abs().lt(L::splat(FLUSH_EPS)))
}

/// One width of `f32` lanes with pinned IEEE-754 semantics.
///
/// Implemented by [`prim@f32`] (`WIDTH = 1`, the oracle), [`wide::f32x4`] and [`wide::f32x8`].
/// Every method is `#[inline(always)]` in every implementation. The surface is deliberately
/// minimal (master plan §3.1): no horizontal operations, no gather, no reciprocal or reciprocal
/// square-root approximations, and no runtime dispatch.
///
/// # Numeric contract
///
/// `add`, `sub`, `mul`, `div` and `sqrt` are IEEE-754 round-to-nearest-even; `fma` rounds once;
/// `neg` and `abs` are sign-bit operations, never `0.0 - x`; comparisons are ordered (NaN compares
/// false); `select` is bitwise per lane. A [`Lane::Mask`] lane is either all zero bits or all one
/// bits — masks are produced only by the comparison and mask operations of this trait.
pub trait Lane: Copy + Send + Sync + 'static {
    /// Number of `f32` lanes in one value.
    const WIDTH: usize;

    /// Result of a comparison: per lane either all zero bits or all one bits.
    type Mask: Copy;

    /// Broadcasts one value to every lane.
    fn splat(x: f32) -> Self;

    /// All lanes `+0.0`.
    fn zero() -> Self;

    /// Reads exactly [`Lane::WIDTH`] values from the front of `src`.
    ///
    /// # Panics
    ///
    /// Panics if `src` is shorter than [`Lane::WIDTH`]. Block shapes are validated once at plan
    /// preparation, so this bounds check is a debugging aid, not a render-path branch.
    fn load(src: &[f32]) -> Self;

    /// Writes exactly [`Lane::WIDTH`] values to the front of `dst`.
    ///
    /// # Panics
    ///
    /// Panics if `dst` is shorter than [`Lane::WIDTH`].
    fn store(self, dst: &mut [f32]);

    /// `self + b`, IEEE round-to-nearest-even.
    fn add(self, b: Self) -> Self;

    /// `self - b`, IEEE round-to-nearest-even.
    fn sub(self, b: Self) -> Self;

    /// `self * b`, IEEE round-to-nearest-even.
    fn mul(self, b: Self) -> Self;

    /// `self / b`, IEEE-exact. Audit every render-path use: division is not cheap.
    fn div(self, b: Self) -> Self;

    /// `sqrt(self)`, IEEE-exact on every target.
    fn sqrt(self) -> Self;

    /// `self * b + c` with a single rounding (D3).
    ///
    /// This is the only place fusion is allowed to exist. Hardware FMA is used on `x86` (pinned to
    /// `+fma`) and on AArch64 NEON; every other target uses the exact software FMA of
    /// [`softfma`], which is bit-identical to the hardware instruction (gate G3).
    fn fma(self, b: Self, c: Self) -> Self;

    /// `-self` as a sign-bit flip. Never `0.0 - self`, which is wrong for `+0.0`.
    fn neg(self) -> Self;

    /// `|self|` as a sign-bit clear.
    fn abs(self) -> Self;

    /// IEEE `floor(self)`.
    fn floor(self) -> Self;

    /// Ordered `self < b`: NaN in either operand yields an all-zero lane.
    fn lt(self, b: Self) -> Self::Mask;

    /// Ordered `self <= b`.
    fn le(self, b: Self) -> Self::Mask;

    /// Ordered `self > b`.
    fn gt(self, b: Self) -> Self::Mask;

    /// Ordered `self >= b`.
    fn ge(self, b: Self) -> Self::Mask;

    /// Ordered `self == b`. `+0.0` and `-0.0` compare equal.
    fn eq(self, b: Self) -> Self::Mask;

    /// Bitwise `a & b` on two masks.
    fn mask_and(a: Self::Mask, b: Self::Mask) -> Self::Mask;

    /// Bitwise `a | b` on two masks.
    fn mask_or(a: Self::Mask, b: Self::Mask) -> Self::Mask;

    /// Bitwise `!a` on a mask.
    fn mask_not(a: Self::Mask) -> Self::Mask;

    /// `true` if any lane of the mask is set.
    ///
    /// This is the only operation that leaves the vector domain. It is a control-plane and
    /// once-per-block operation; it must never appear in a per-sample loop.
    fn mask_any(m: Self::Mask) -> bool;

    /// Per-lane bitwise `m ? a : b`.
    fn select(m: Self::Mask, a: Self, b: Self) -> Self;

    /// Clears every lane of `self` whose mask lane is set, making it exactly `+0.0`.
    fn andnot(self, m: Self::Mask) -> Self;

    /// `select(self > b, self, b)`: returns `b` on equal lanes and on unordered lanes (D8).
    ///
    /// Consequences that are deliberate and gated: `max(-0.0, +0.0)` is `+0.0`,
    /// `max(+0.0, -0.0)` is `-0.0`, `max(NaN, x)` is `x` and `max(x, NaN)` is `NaN`.
    #[inline(always)]
    fn max(self, b: Self) -> Self {
        Self::select(self.gt(b), self, b)
    }

    /// `select(self < b, self, b)`: the mirror of [`Lane::max`] (D8).
    #[inline(always)]
    fn min(self, b: Self) -> Self {
        Self::select(self.lt(b), self, b)
    }

    /// `2^n` for integer-valued `n`, by exponent-field construction (no rounding).
    ///
    /// `n` is clamped to `[-126, 127]` with the D8 form first, so every target sees the same
    /// in-range input and NaN maps to `-126` (master plan §11). For an integer-valued `n` the
    /// result is exact; for a non-integer `n` the result is unspecified but identical on every
    /// backend, because the clamp, the add and the shift are the same operations everywhere.
    fn exp2_int(n: Self) -> Self;

    /// Splits a positive normal `self` into `(m, e)` with `self = m * 2^e` and `m` in `[1, 2)`.
    ///
    /// Used by `log2`. For inputs that are not positive normals the result is unspecified but,
    /// again, identical on every backend.
    fn frexp(self) -> (Self, Self);

    /// Writes the raw bits of each lane to the front of `dst`. Tests and digests only.
    ///
    /// # Panics
    ///
    /// Panics if `dst` is shorter than [`Lane::WIDTH`].
    fn store_bits(self, dst: &mut [u32]);
}
