//! Which lane width this build uses, and the boot attestation that proves the CPU can run it.
//!
//! There is no runtime SIMD dispatch (D4, revision 4). `wide` picks its instruction set from
//! `cfg(target_feature)` at compile time, the workspace pins `x86_64` to `x86-64-v3`, and NEON is
//! baseline on AArch64, so the backend is a compile-time constant. What remains is to refuse to
//! start on a CPU that cannot execute the pinned instructions — never to fall back silently.
//!
//! One build-time cfg, `miso_wasm_simd8`, moves the wasm arm from [`Backend::Simd4`] to
//! [`Backend::Simd8`]. It is measurement infrastructure for issue #183 step 2 and nothing else:
//! no default build, gate or shipped artifact sets it, so with it absent this file compiles to the
//! same constant it did before it existed. It is deliberately a cfg rather than a Cargo feature,
//! because a feature is unified across a whole invocation's dependency graph and would let one
//! crate's opt-in silently rewidth another's.

use core::fmt;

/// The lane width this build was compiled for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Backend {
    /// `f32`, one lane: the oracle, and the scalar tail of every bank.
    Scalar,
    /// [`wide::f32x4`], four lanes: AArch64 NEON and wasm `simd128`.
    Simd4,
    /// [`wide::f32x8`], eight lanes: one `__m256` on `x86-64-v3`.
    Simd8,
}

impl Backend {
    /// The backend this build uses, decided entirely at compile time.
    #[must_use]
    pub const fn current() -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            Self::Simd8
        }
        #[cfg(target_arch = "aarch64")]
        {
            Self::Simd4
        }
        // The production wasm width. `wide` lowers `f32x8` to two `v128` values there, so eight
        // lanes are two instructions per operation rather than one; whether that is a win is a
        // measurement, and until it is made the shipped artifact is four lanes.
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            not(miso_wasm_simd8)
        ))]
        {
            Self::Simd4
        }
        // Issue #183 step 2: the eight-lane wasm *measurement* build, opt in at build time with
        // `RUSTFLAGS="--cfg miso_wasm_simd8"`. It exists so the paired W4/W8 wasmtime console
        // record can be taken from two guests that differ in nothing but this constant; it is not
        // a shipping configuration and no default build sets it, so the W4 artifact every gate and
        // digest check covers is byte-identical with or without this arm present.
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128", miso_wasm_simd8))]
        {
            Self::Simd8
        }
        #[cfg(not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            all(target_arch = "wasm32", target_feature = "simd128")
        )))]
        {
            Self::Scalar
        }
    }

    /// Number of `f32` lanes this backend processes at once.
    #[must_use]
    pub const fn width(self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Simd4 => 4,
            Self::Simd8 => 8,
        }
    }
}

/// Why a host may not run this build (see [`attest_host`]).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HostAttestation {
    /// The CPU lacks a named `x86` feature the build is pinned to.
    MissingX86Feature {
        /// The missing feature, as spelled by `is_x86_feature_detected!`.
        feature: &'static str,
    },
}

impl fmt::Display for HostAttestation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingX86Feature { feature } => write!(
                formatter,
                "this CPU does not support the x86 feature '{feature}'; the engine is built for \
                 x86-64-v3 (AVX2 and FMA) and has no scalar fallback"
            ),
        }
    }
}

impl core::error::Error for HostAttestation {}

/// Attests once, at boot, that this CPU can execute the instructions this build is pinned to.
///
/// Master plan #83 D4: the engine is compiled for `x86-64-v3` and dispatches nothing at runtime,
/// so a CPU without AVX2 and FMA would execute an illegal instruction rather than degrade. Every
/// host and C-ABI entry point calls this before creating an engine and refuses to start on an
/// error — never a silent scalar fallback. On non-`x86` targets the pinned instruction sets are
/// baseline (NEON) or a whole-artifact build flag (wasm `simd128`), so this returns `Ok`.
///
/// This is control-plane work: it runs once, never from a render callback.
///
/// # Errors
///
/// Returns [`HostAttestation::MissingX86Feature`] naming the first pinned feature this CPU lacks.
pub fn attest_host() -> Result<(), HostAttestation> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if !std::is_x86_feature_detected!("avx2") {
            return Err(HostAttestation::MissingX86Feature { feature: "avx2" });
        }
        if !std::is_x86_feature_detected!("fma") {
            return Err(HostAttestation::MissingX86Feature { feature: "fma" });
        }
    }
    Ok(())
}
