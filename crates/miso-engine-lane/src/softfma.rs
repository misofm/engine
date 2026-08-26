//! The MXCSR helpers gate G6 needs.
//!
//! # Why this file kept its name
//!
//! Until issue #163 phase 2 this module held the workspace's exact software FMA: for `f32` inputs
//! the product is exact in `f64` (24 + 24 <= 53 bits), so computing the sum in `f64` with
//! round-to-odd and demoting once reproduces the IEEE fused result exactly (Boldo and Melquiond,
//! *Emulation of a FMA and correctly rounded sums: proved algorithms using rounding to odd*,
//! IEEE TC 2008). It was what the wasm backend used for [`crate::Lane::fma`], and gate G3 proved
//! it bit-identical to the hardware instruction.
//!
//! Phase 2 made the numeric contract unfused on every backend, so nothing needs an exact fused
//! multiply-add any more and the emulation was retired along with G3 -- it cost about 54
//! instructions per operation on wasm, measured 5.5x on the SVF kernel, which was the tax the
//! phase existed to delete. The file keeps its name because
//! `scripts/check-lane-policy.sh`, `scripts/check-realtime-policy.sh` and
//! `docs/REALTIME_DEPENDENCY_POLICY.md` name this path as one of the two places in the lane crate
//! allowed to carry `unsafe`, and renaming it would edit policy files to no numeric purpose.
//!
//! If a site is ever ruled to need exact fused semantics, the emulation returns from history as a
//! separately named `fma_exact` with a seal listing its admitted sites --
//! `docs/rulings/unfused-multiply-add-audit.md` records the reopening conditions.
//!
//! # Why this file carries `unsafe`
//!
//! The `x86` MXCSR helpers below are used by gate G6 to prove that hardware flush-to-zero is inert
//! under the D7 flush law. The workspace denies `unsafe` outside an enumerated allowlist and
//! forbids inline assembly, so the helpers live here, in one of the two lane files
//! `scripts/check-realtime-policy.sh` allows to carry `unsafe`, and use the deprecated
//! `_mm_getcsr`/`_mm_setcsr` intrinsics rather than the inline assembly their deprecation note
//! recommends.

#![allow(unsafe_code)]

/// `(a * b) + c` for `f32`, restated through `f64` -- the independent oracle for the unfused
/// contract (issue #163 phase 2).
///
/// This is the successor to the retired `fma_f32_via_f64`. Its job is unchanged: give the
/// workspace's evidence code a way to compute a multiply-add that does **not** go through the
/// production `f32` expression it is checking, so an oracle assertion is a second opinion rather
/// than a restatement of the thing under test.
///
/// The name deliberately spells out "multiply_add" rather than the usual contraction:
/// `scripts/check-lane-policy.sh` refuses the `mul_add` token outside this crate by plain
/// substring match, and a name containing it would make every caller of an *unfused* oracle look
/// like a fused-arithmetic violation.
///
/// # Why the `f64` route reproduces the `f32` result exactly
///
/// * The product is exact. Two `f32` significands are 24 bits each, and `24 + 24 = 48 <= 53`, so
///   `f64::from(a) * f64::from(b)` is the exact real product and the narrowing back to `f32` is
///   the operation's only rounding — which is what `a * b` in `f32` does.
/// * The sum double-rounds innocuously. Rounding the sum to `f64` and then to `f32` gives the same
///   result as rounding it directly to `f32`, because the wider format carries at least
///   `2p + 2 = 50` bits and `f64` carries 53 (Figueroa, *When is double rounding innocuous?*,
///   ACM SIGNUM 1995).
///
/// # Limit
///
/// Both arguments assume a **normal** result. Double rounding into the subnormal band can differ
/// from a single rounding, so this function is not a valid oracle there. Every render-path
/// recurrence it checks is covered by the D7 flush law ([`crate::flush`] with
/// [`crate::FLUSH_EPS`] = 1e-20, far above the subnormal band), which removes those values before
/// they can be observed.
#[inline]
#[must_use]
pub fn unfused_multiply_add_via_f64(a: f32, b: f32, c: f32) -> f32 {
    let product = (f64::from(a) * f64::from(b)) as f32;
    (f64::from(product) + f64::from(c)) as f32
}

/// MXCSR flush-to-zero bit (`FTZ`): denormal results become zero.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const MXCSR_FTZ: u32 = 0x8000;

/// MXCSR denormals-are-zero bit (`DAZ`): denormal operands are treated as zero.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const MXCSR_DAZ: u32 = 0x0040;

/// Reads the current thread's MXCSR control word.
///
/// Gate G6 support, never called from a render path.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(deprecated)]
#[must_use]
pub fn read_mxcsr() -> u32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_getcsr;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_getcsr;
    // SAFETY: `_mm_getcsr` reads a control register and is sound on any SSE host; SSE2 is baseline
    // on x86_64 and required by the crate's x86-64-v3 compile guard.
    unsafe { _mm_getcsr() }
}

/// Writes the current thread's MXCSR control word.
///
/// Gate G6 support, never called from a render path: FTZ and DAZ are *observed*, never relied on
/// (D7). The write affects only the calling thread, so a test must restore the previous value
/// before it returns.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(deprecated)]
pub fn write_mxcsr(value: u32) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_setcsr;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_setcsr;
    // SAFETY: `_mm_setcsr` writes a control register and is sound on any SSE host. The value is a
    // control word previously read by `read_mxcsr` with at most the FTZ and DAZ bits changed, so no
    // rounding mode or exception mask is disturbed.
    unsafe { _mm_setcsr(value) }
}
