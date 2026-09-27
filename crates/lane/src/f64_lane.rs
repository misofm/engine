//! `f64` lanes: the widening companion of the `f32` [`Lane`] (issue #949).
//!
//! Two traits, deliberately separate from [`Lane`]: [`LaneF64`], `W` lanes of IEEE binary64 with
//! `load`, `store`, `add` and `mul`, and [`Widen`], which turns an `f32` lane value into its
//! `W`-lane `f64` companion. [`Lane`] cannot grow instead: it is `f32`-only and deliberately
//! minimal (master plan §3.1), and it is implemented outside this crate by a test wrapper, which a
//! new required item would break.
//!
//! The surface is exactly what a lane-parallel, sample-serial `f64` sum needs -- the meter's
//! `energy += x * x` is its first caller (issue #950) -- and nothing else goes in: no `sub`, `div`,
//! `sqrt`, comparison, select, `splat` or fused operation. Each future need is its own
//! owner-visible addition.
//!
//! # What is and is not forwarded to `wide`
//!
//! The same rule as `wide_impl.rs`: `wide` is a vocabulary, not a semantics authority.
//!
//! * `add` and `mul` forward `wide`'s `+` and `*` operators. In `wide` 1.6.1 those are
//!   `vaddpd`/`vmulpd` on x86, `f64x2.add`/`f64x2.mul` on wasm `simd128`, `fadd`/`fmul` on
//!   AArch64 NEON, and a per-lane `f64` array elsewhere -- the IEEE basic operations on every
//!   target.
//! * `wide`'s `mul_add`, `mul_sub`, `mul_neg_add` and `mul_neg_sub` are never called. `f64x4`'s
//!   `mul_add` is a hardware fused multiply-add under `avx` plus `fma` and on NEON, and an unfused
//!   pair elsewhere: the per-target split [`Lane::fma`] was unfused to remove (issue #163 phase 2).
//! * `wide` has **no** `f32`-to-`f64` conversion, and `core::arch` is confined to `softfma.rs` and
//!   `fpenv.rs` by `scripts/policies/lane-source.toml`. So [`Widen`] is written portably, as
//!   `<F64>::new(self.to_array().map(f64::from))`, and the vector lowering is a codegen outcome:
//!   see [Lowering](#lowering).
//!
//! # Lowering
//!
//! At the release profile (fat LTO, one codegen unit) LLVM's SLP vectorizer turns the portable
//! widen into the packed conversion, measured for issue #949: `vcvtps2pd` on `x86-64-v3` (two per
//! `Simd8` value, plus one `vextractf128`) and `f64x2.promote_low_f32x4` on wasm `simd128`, with no
//! scalar `vcvtss2sd` or `f64.promote_f32` left. Correctness never depends on that outcome -- a
//! scalarised widen is the same `fpext` per lane, with the same bits -- and the wasm `simd128`
//! lowering is pinned by `check_f64_lane_lowering` in `scripts/run-wasm-gates.sh`, which fails
//! the gate if the guest probe's body stops using the three vector opcodes or picks up a scalar
//! `f64` one.
//!
//! Beware the rlib: Cargo builds workspace rlibs with `-C linker-plugin-lto`, whose pre-link
//! pipeline runs no SLP vectorizer, so `--emit asm` on this crate shows eight scalar
//! `vcvtss2sd`. Census a linked artifact, never the rlib.

use crate::Lane;

/// `W` lanes of IEEE binary64: the accumulator companion of an `f32` [`Lane`].
///
/// Implemented by [`prim@f64`] (`WIDTH = 1`, the oracle), [`wide::f64x4`] and [`wide::f64x8`].
/// Every method is `#[inline(always)]` in every implementation. Outside this crate the vector
/// types are named `<Simd4 as Widen>::F64` and `<Simd8 as Widen>::F64`, so that no other crate has
/// to name `wide` (`scripts/check-lane-policy.sh`).
///
/// # Numeric contract
///
/// * `add` and `mul` are IEEE-754 binary64, round-to-nearest-even, lane by lane. Every non-NaN
///   result is therefore bit-identical to scalar `f64` `+` and `*` at every width and on every
///   target; a NaN result is a NaN whose payload is unspecified.
/// * Nothing is fused, for the two reasons [`Lane::fma`] gives. Rust never contracts: rustc emits
///   no `contract` fast-math flag and no `llvm.fmuladd` for `a * b + c`, so the pinned `+fma`
///   makes the instruction available without licensing LLVM to fuse a separate multiply and add.
///   And `wide`'s `mul_add` is never forwarded. `e.add(w.mul(w))` is two roundings on every
///   backend.
/// * The floating-point environment matters, exactly as `wide_impl.rs`, "The one precondition",
///   records for `f32`: under x86 `MXCSR.FTZ` a subnormal result would flush to zero, and under
///   `MXCSR.DAZ` a subnormal input would be read as zero. Every native
///   render entry installs the canonical environment, with FTZ and DAZ clear, through
///   [`crate::fpenv::CanonicalFpEnv`] (issue #146); AArch64 clears `FPCR` the same way, and wasm
///   has no flush mode.
pub trait LaneF64: Copy + Send + Sync + 'static {
    /// Number of `f64` lanes in one value.
    const WIDTH: usize;

    /// Reads exactly [`LaneF64::WIDTH`] values from the front of `src`.
    ///
    /// # Panics
    ///
    /// Panics if `src` is shorter than [`LaneF64::WIDTH`]. As for [`Lane::load`], the bounds check
    /// is a debugging aid, not a render-path branch.
    fn load(src: &[f64]) -> Self;

    /// Writes exactly [`LaneF64::WIDTH`] values to the front of `dst`.
    ///
    /// # Panics
    ///
    /// Panics if `dst` is shorter than [`LaneF64::WIDTH`].
    fn store(self, dst: &mut [f64]);

    /// `self + b`, IEEE binary64 round-to-nearest-even, per lane.
    fn add(self, b: Self) -> Self;

    /// `self * b`, IEEE binary64 round-to-nearest-even, per lane. Never fused with a following
    /// [`LaneF64::add`].
    fn mul(self, b: Self) -> Self;
}

/// An `f32` [`Lane`] that widens, lane for lane, to its [`LaneF64`] companion.
///
/// Implemented by [`prim@f32`] (`F64 = f64`, the oracle), [`wide::f32x4`] (`F64 = wide::f64x4`)
/// and [`wide::f32x8`] (`F64 = wide::f64x8`). `F64::WIDTH` equals `Self::WIDTH` and lane `i` of
/// the result is lane `i` of the input.
///
/// # Numeric contract
///
/// `widen` is exact for every non-NaN input: every `f32`, subnormals and infinities included, is
/// exactly representable in binary64, so the result is the same real number. A NaN input gives
/// some NaN, whose payload is not pinned (wasm `f64.promote_f32` is nondeterministic on NaN, and
/// x86 quiets a signalling NaN).
///
/// The one environment dependence is x86 `MXCSR.DAZ`, which would flush a subnormal input of the
/// conversion to zero. It is the precondition `wide_impl.rs` records for `f32`, and it is met the
/// same way: every native render entry installs [`crate::fpenv::CanonicalFpEnv`], with DAZ clear.
pub trait Widen: Lane {
    /// The `f64` lanes this width widens to, with the same [`LaneF64::WIDTH`].
    type F64: LaneF64;

    /// Each lane converted to `f64`: exact for every non-NaN input.
    fn widen(self) -> Self::F64;
}

impl LaneF64 for f64 {
    const WIDTH: usize = 1;

    #[inline(always)]
    fn load(src: &[f64]) -> Self {
        src[0]
    }

    #[inline(always)]
    fn store(self, dst: &mut [f64]) {
        dst[0] = self;
    }

    #[inline(always)]
    fn add(self, b: Self) -> Self {
        self + b
    }

    #[inline(always)]
    fn mul(self, b: Self) -> Self {
        self * b
    }
}

/// Implements [`LaneF64`] for one `wide` `f64` vector type.
///
/// Arguments: the vector type and its lane count. `load` and `store` copy through a `[f64; W]`,
/// as `wide_impl.rs` does for `f32`: `f64x8` is `#[repr(C, align(64))]`, so a slice is never
/// transmuted into one.
macro_rules! impl_lane_f64_for_wide {
    ($simd:ty, $width:literal) => {
        impl LaneF64 for $simd {
            const WIDTH: usize = $width;

            #[inline(always)]
            fn load(src: &[f64]) -> Self {
                let mut lanes = [0.0f64; $width];
                lanes.copy_from_slice(&src[..$width]);
                <$simd>::new(lanes)
            }

            #[inline(always)]
            fn store(self, dst: &mut [f64]) {
                dst[..$width].copy_from_slice(&self.to_array());
            }

            #[inline(always)]
            fn add(self, b: Self) -> Self {
                self + b
            }

            #[inline(always)]
            fn mul(self, b: Self) -> Self {
                // One rounding. `wide`'s fused spellings are deliberately not called (module
                // documentation), so a following `add` rounds again on every backend.
                self * b
            }
        }
    };
}

impl_lane_f64_for_wide!(wide::f64x4, 4);
impl_lane_f64_for_wide!(wide::f64x8, 8);

impl Widen for f32 {
    type F64 = f64;

    #[inline(always)]
    fn widen(self) -> f64 {
        f64::from(self)
    }
}

impl Widen for wide::f32x4 {
    type F64 = wide::f64x4;

    #[inline(always)]
    fn widen(self) -> wide::f64x4 {
        <wide::f64x4>::new(self.to_array().map(f64::from))
    }
}

impl Widen for wide::f32x8 {
    type F64 = wide::f64x8;

    #[inline(always)]
    fn widen(self) -> wide::f64x8 {
        <wide::f64x8>::new(self.to_array().map(f64::from))
    }
}

// Lane `i` of a widened value is lane `i` of its input, so the two widths must agree. Checked at
// compile time here and again by gate 1 (`tests/f64_lane.rs`).
const _: () = assert!(<f64 as LaneF64>::WIDTH == <f32 as Lane>::WIDTH);
const _: () = assert!(<wide::f64x4 as LaneF64>::WIDTH == <wide::f32x4 as Lane>::WIDTH);
const _: () = assert!(<wide::f64x8 as LaneF64>::WIDTH == <wide::f32x8 as Lane>::WIDTH);
