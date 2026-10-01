//! Portable target-smoke values: the per-target lane-width pin CI compiles and tests.

use engine::{EngineVersion, QuantumFrames, SampleRateHz};
use lane::Backend;

/// A portable bootstrap result with the canonical smoke sample rate and render quantum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetSmoke {
    /// Engine API version.
    pub version: EngineVersion,
    /// Bootstrap sample rate, fixed to 48 kHz.
    pub sample_rate: SampleRateHz,
    /// Bootstrap render quantum, fixed to 128 frames.
    pub quantum_frames: QuantumFrames,
    /// The lane backend this build was compiled for (#83 D4).
    pub backend: Backend,
}

/// Return a portable target-smoke result without allocating or starting audio processing.
#[must_use]
pub fn target_smoke() -> TargetSmoke {
    TargetSmoke {
        version: EngineVersion::CURRENT,
        sample_rate: SampleRateHz(48_000),
        quantum_frames: QuantumFrames(128),
        backend: Backend::current(),
    }
}

#[cfg(test)]
mod tests {
    use super::target_smoke;

    #[test]
    fn smoke_values_are_canonical() {
        let report = target_smoke();

        assert_eq!(report.sample_rate.0, 48_000);
        assert_eq!(report.quantum_frames.0, 128);

        // Literal, per-width expected backends -- not `report.backend == lane::Backend::current()`,
        // which would compare the same compile-time constant against itself and could never fail.
        // A change to either `lane::Backend::current()`'s selection or to this pin must fail this
        // test. The width follows the target features, keyed exactly as `current()` keys it (issue
        // #1112): `avx2` (`x86-64-v3` pins AVX2/FMA) is eight lanes, and `neon` (baseline on
        // AArch64) or `simd128` is four. Every other build has no row because `lane` refuses to
        // compile for it (issues #1041 and #1062), so this crate cannot be built there either.
        #[cfg(target_feature = "avx2")]
        assert_eq!(
            report.backend,
            lane::Backend::Simd8,
            "AVX2 is the 8-lane width, eight f32 lanes"
        );
        #[cfg(target_feature = "neon")]
        assert_eq!(
            report.backend,
            lane::Backend::Simd4,
            "NEON is the 4-lane width, four f32 lanes"
        );
        #[cfg(target_feature = "simd128")]
        assert_eq!(
            report.backend,
            lane::Backend::Simd4,
            "wasm simd128 is the 4-lane width, four f32 lanes"
        );
    }
}
