//! Core engine value types and target capability detection.
//!
//! Detection in this crate is control-plane work. It is never required from a realtime render
//! callback.

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

/// Returns whether `rate` is accepted for a launch engine session or render plan.
#[must_use]
pub const fn is_launch_sample_rate(rate: SampleRateHz) -> bool {
    matches!(rate.0, 44_100 | 48_000 | 88_200 | 96_000)
}

/// Returns whether `rate` is an extended compatibility-only corpus rate.
#[must_use]
pub const fn is_extended_compatibility_sample_rate(rate: SampleRateHz) -> bool {
    matches!(rate.0, 176_400 | 192_000 | 352_800 | 384_000)
}

/// A caller-selected render quantum in PCM frames.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QuantumFrames(pub u32);

/// Allocation-free rendering primitives and plan hand-off ownership.
pub mod realtime;

/// Target capabilities selected before preparing a render plan.
///
/// The struct is non-exhaustive because later target capabilities may be added without changing
/// session semantics. `x86_avx2` and `x86_fma` are independently detected.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetCapabilities {
    /// Scalar processing is always available.
    pub scalar: bool,
    /// The Wasm module was compiled with `simd128` enabled.
    pub wasm_simd128: bool,
    /// The AArch64 target exposes NEON.
    pub aarch64_neon: bool,
    /// The current x86 CPU supports AVX2.
    pub x86_avx2: bool,
    /// The current x86 CPU supports FMA.
    pub x86_fma: bool,
}

/// Detect the target capabilities for control-plane plan selection.
///
/// The result may be queried while a plan is compiled. It must be stored with the prepared plan
/// rather than detected from a render callback.
#[must_use]
pub fn target_capabilities() -> TargetCapabilities {
    let (x86_avx2, x86_fma) = detect_x86_capabilities();
    assemble_capabilities(
        cfg!(all(target_arch = "wasm32", target_feature = "simd128")),
        cfg!(all(target_arch = "aarch64", target_feature = "neon")),
        x86_avx2,
        x86_fma,
    )
}

fn assemble_capabilities(
    wasm_simd128: bool,
    aarch64_neon: bool,
    x86_avx2: bool,
    x86_fma: bool,
) -> TargetCapabilities {
    TargetCapabilities {
        scalar: true,
        wasm_simd128,
        aarch64_neon,
        x86_avx2,
        x86_fma,
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn detect_x86_capabilities() -> (bool, bool) {
    (
        std::is_x86_feature_detected!("avx2"),
        std::is_x86_feature_detected!("fma"),
    )
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const fn detect_x86_capabilities() -> (bool, bool) {
    (false, false)
}

#[cfg(test)]
mod tests {
    use super::{
        EXTENDED_COMPATIBILITY_SAMPLE_RATES, EngineVersion, LAUNCH_SAMPLE_RATES, QuantumFrames,
        SampleRateHz, assemble_capabilities, is_extended_compatibility_sample_rate,
        is_launch_sample_rate,
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

    #[test]
    fn avx2_without_fma_is_a_distinct_capability() {
        let capabilities = assemble_capabilities(false, false, true, false);

        assert!(capabilities.scalar);
        assert!(capabilities.x86_avx2);
        assert!(!capabilities.x86_fma);
    }
}
