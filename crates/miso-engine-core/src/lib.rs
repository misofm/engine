//! Core engine value types and the allocation-free realtime plumbing.
//!
//! SIMD kernels, lane widths and backend selection are `miso-engine-lane`'s (#83 D4/D10); this
//! crate carries no architecture code and no backend enum.

/// The version of the engine API represented by this build.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EngineVersion {
    /// Major API version.
    pub major: u32,
    /// Minor API version.
    pub minor: u32,
    /// Patch API version.
    pub patch: u32,
}

impl EngineVersion {
    /// The API version for this bootstrap build.
    pub const CURRENT: Self = Self {
        major: 0,
        minor: 1,
        patch: 0,
    };
}

/// A caller-selected sample rate in hertz.
///
/// This is a lossless carrier. Constructing it does not promise engine render support: source
/// metadata and compatibility fixtures also use arbitrary nonzero rates.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SampleRateHz(pub u32);

/// Exact launch-supported engine session and render rates.
pub const LAUNCH_SAMPLE_RATES: [SampleRateHz; 4] = [
    SampleRateHz(44_100),
    SampleRateHz(48_000),
    SampleRateHz(88_200),
    SampleRateHz(96_000),
];

/// Exact extended compatibility rates retained for corpus and descriptor evidence only.
pub const EXTENDED_COMPATIBILITY_SAMPLE_RATES: [SampleRateHz; 4] = [
    SampleRateHz(176_400),
    SampleRateHz(192_000),
    SampleRateHz(352_800),
    SampleRateHz(384_000),
];

const fn rate_set_contains(rates: &[SampleRateHz], rate: SampleRateHz) -> bool {
    let mut index = 0;
    while index < rates.len() {
        if rates[index].0 == rate.0 {
            return true;
        }
        index += 1;
    }
    false
}

/// Returns whether `rate` is accepted for a launch engine session or render plan.
#[must_use]
pub const fn is_launch_sample_rate(rate: SampleRateHz) -> bool {
    rate_set_contains(&LAUNCH_SAMPLE_RATES, rate)
}

/// Returns whether `rate` is an extended compatibility-only corpus rate.
#[must_use]
pub const fn is_extended_compatibility_sample_rate(rate: SampleRateHz) -> bool {
    rate_set_contains(&EXTENDED_COMPATIBILITY_SAMPLE_RATES, rate)
}

/// A caller-selected render quantum in PCM frames.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QuantumFrames(pub u32);

/// Allocation-free rendering primitives and plan hand-off ownership.
pub mod realtime;

#[cfg(test)]
mod tests {
    use super::{
        EXTENDED_COMPATIBILITY_SAMPLE_RATES, EngineVersion, LAUNCH_SAMPLE_RATES, QuantumFrames,
        SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate,
    };

    #[test]
    fn sample_rate_tiers_are_exact_sorted_disjoint_and_classified() {
        assert_eq!(
            LAUNCH_SAMPLE_RATES.map(|rate| rate.0),
            [44_100, 48_000, 88_200, 96_000]
        );
        assert_eq!(
            EXTENDED_COMPATIBILITY_SAMPLE_RATES.map(|rate| rate.0),
            [176_400, 192_000, 352_800, 384_000]
        );
        for rate in LAUNCH_SAMPLE_RATES {
            assert!(is_launch_sample_rate(rate));
            assert!(!is_extended_compatibility_sample_rate(rate));
        }
        for rate in EXTENDED_COMPATIBILITY_SAMPLE_RATES {
            assert!(!is_launch_sample_rate(rate));
            assert!(is_extended_compatibility_sample_rate(rate));
        }
        for rate in [SampleRateHz(0), SampleRateHz(32_000), SampleRateHz(192_001)] {
            assert!(!is_launch_sample_rate(rate));
            assert!(!is_extended_compatibility_sample_rate(rate));
        }
    }

    #[test]
    fn quantum_is_a_lossless_carrier() {
        assert_eq!(QuantumFrames(128).0, 128);
    }

    #[test]
    fn version_is_stable_for_bootstrap() {
        assert_eq!(
            EngineVersion::CURRENT,
            EngineVersion {
                major: 0,
                minor: 1,
                patch: 0
            }
        );
    }
}
