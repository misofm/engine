//! Which lane width this build uses, and the boot attestation that proves the CPU can run it.
//!
//! There is no runtime SIMD dispatch (D4, revision 4). `wide` picks its instruction set from
//! `cfg(target_feature)` at compile time, the workspace pins `x86_64` to `x86-64-v3`, and NEON is
//! baseline on AArch64, so the backend is a compile-time constant. What remains is to refuse to
//! start on a CPU that cannot execute the pinned instructions — never to fall back silently.

use core::fmt;

/// The lane width this build was compiled for.
///
/// # `Scalar` is a test-only oracle (issue #1059)
///
/// A plan compiled at `Scalar` binds no bank: every track renders one at a time, unbanked. That
/// whole-plan scalar renderer is the correctness reference for vector banking -- "a placement
/// change must not move a rendered bit" -- and nothing else (owner ruling 2026-09-28, decision 3,
/// `docs/rulings/engine-footprint-2026-09-28.md`). No shipped target selects it, so the variant
/// exists only in builds that enable this crate's `test-support` feature, and shipped artifacts
/// cannot name it. The one-lane [`prim@f32`] [`Lane`](crate::Lane), which every effect's
/// per-node leg and every frame loop's tail use, is a different thing and is not gated.
///
/// Code outside this crate must not match on the variants exhaustively, because the variant set
/// depends on a feature that Cargo unifies across a build: ask [`Backend::width`] or compare
/// with `==`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Backend {
    /// `f32`, one lane, whole plan: the unbanked test oracle (see the type's documentation).
    #[cfg(feature = "test-support")]
    Scalar,
    /// [`wide::f32x4`], four lanes: AArch64 NEON and wasm `simd128`.
    Simd4,
    /// [`wide::f32x8`], eight lanes: one `__m256` on `x86-64-v3`.
    ///
    /// Absent on `wasm32` (issue #1110): the browser can never select it, so the browser build
    /// cannot name it either (see [`Simd8`](crate::Simd8)).
    #[cfg(not(target_arch = "wasm32"))]
    Simd8,
}

impl Backend {
    /// The backend this build uses, decided entirely at compile time.
    #[must_use]
    pub const fn current() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self::Simd8
        }
        #[cfg(target_arch = "aarch64")]
        {
            Self::Simd4
        }
        // The one wasm width. `wide` lowers `f32x8` to two `v128` values there, and issue #183
        // measured eight lanes as a null (`docs/rulings/wasm-simd8-null.md`); #1038 removed the
        // cfg that selected them. A re-measurement re-adds one in its own issue.
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            Self::Simd4
        }
        // No arm for any other target: `lib.rs` refuses to compile for every one of them, 32-bit
        // targets (issue #1041) and wasm32 without `simd128` (issue #1062) alike, so no target
        // selects `Scalar` and the variant exists only in `test-support` builds.
    }

    /// Number of `f32` lanes this backend processes at once.
    #[must_use]
    pub const fn width(self) -> usize {
        match self {
            #[cfg(feature = "test-support")]
            Self::Scalar => 1,
            Self::Simd4 => 4,
            #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(target_arch = "x86_64")]
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
