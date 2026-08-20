//! Portable target-smoke values used by bootstrap hosts and CI.

use miso_engine_core::{
    EngineVersion, QuantumFrames, SampleRateHz, TargetCapabilities, target_capabilities,
};
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
    /// Control-plane target capabilities.
    pub capabilities: TargetCapabilities,
}

/// Return a portable target-smoke result without allocating or starting audio processing.
#[must_use]
pub fn target_smoke() -> TargetSmoke {
    TargetSmoke {
        version: EngineVersion::CURRENT,
        sample_rate: SampleRateHz(48_000),
        quantum_frames: QuantumFrames(128),
        capabilities: target_capabilities(),
    }
}

/// Exercise the browser single-owner queue path without shared memory or atomics.
#[must_use]
pub fn local_realtime_ring_smoke(value: u32) -> bool {
    let mut ring = miso_engine_core::realtime::LocalRing::new(
        NonZeroUsize::new(1).expect("one-slot smoke ring"),
        miso_engine_core::realtime::QueueGeneration(1),
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
        assert!(report.capabilities.scalar);
    }

    #[test]
    fn local_realtime_ring_round_trips() {
        assert!(local_realtime_ring_smoke(42));
    }
}
