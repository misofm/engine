//! Portable target-smoke values used by bootstrap hosts and CI.

use engine::{EngineVersion, QuantumFrames, SampleRateHz};
use lane::Backend;
use std::num::NonZeroUsize;

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

/// Exercise the browser single-owner queue path without shared memory or atomics.
#[must_use]
pub fn local_realtime_ring_smoke(value: u32) -> bool {
    let mut ring = engine::realtime::LocalRing::new(
        NonZeroUsize::new(1).expect("one-slot smoke ring"),
        engine::realtime::QueueGeneration(1),
    )
    .expect("valid smoke ring");
    ring.try_push(value).is_ok() && ring.try_pop() == Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{local_realtime_ring_smoke, target_smoke};

    #[test]
    fn smoke_values_are_canonical() {
        let report = target_smoke();

        assert_eq!(report.sample_rate.0, 48_000);
        assert_eq!(report.quantum_frames.0, 128);

        // Literal, per-target expected backends -- not `report.backend == lane::Backend::current()`,
        // which would compare the same compile-time constant against itself and could never fail.
        // A change to either `lane::Backend::current()`'s target selection or to this pin must fail
        // this test (AGENTS.md: `x86-64-v3` is pinned to AVX2/FMA, NEON is baseline on AArch64, the
        // shipped wasm width is four lanes unless issue #183 step 2's measurement cfg widens it, and
        // every other target is the scalar fallback).
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        assert_eq!(
            report.backend,
            lane::Backend::Simd8,
            "x86-64-v3 is pinned to AVX2, eight f32 lanes"
        );
        #[cfg(target_arch = "aarch64")]
        assert_eq!(
            report.backend,
            lane::Backend::Simd4,
            "AArch64 NEON is baseline, four f32 lanes"
        );
        #[cfg(all(
            target_arch = "wasm32",
            target_feature = "simd128",
            not(miso_wasm_simd8)
        ))]
        assert_eq!(
            report.backend,
            lane::Backend::Simd4,
            "the shipped wasm width is four lanes"
        );
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128", miso_wasm_simd8))]
        assert_eq!(
            report.backend,
            lane::Backend::Simd8,
            "issue #183 step 2's eight-lane wasm measurement build"
        );
        #[cfg(not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            all(target_arch = "wasm32", target_feature = "simd128")
        )))]
        assert_eq!(
            report.backend,
            lane::Backend::Scalar,
            "every other target is the scalar fallback"
        );
    }

    #[test]
    fn local_realtime_ring_round_trips() {
        assert!(local_realtime_ring_smoke(42));
    }
}
