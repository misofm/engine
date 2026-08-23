//! Independent offline `f64` reference primitives for conformance tests.
//!
//! This crate intentionally has no dependency on engine production kernels. It is not realtime
//! code and exists solely as an auditable numerical oracle.

mod biquad;
mod block;
mod compressor;
mod delay;
mod gate_expander;
mod lr4;
mod parametric_eq;
mod processor;
mod signals;
mod soft_clip;
mod spectrum;
mod svf;
mod tpt;
mod transient_shaper;

#[cfg(test)]
mod parametric_eq_candidates;
#[cfg(test)]
mod parametric_eq_recurrence_proof;
#[cfg(test)]
mod parametric_eq_time_domain_candidates;
#[cfg(test)]
mod portable_filter_quality;

pub use biquad::{ReferenceBiquad, ReferenceBiquadError, ReferenceFilterKind};
pub use block::{F64PlanarBuffer, ReferenceBlockError};
pub use compressor::{
    ReferenceCompressorError, ReferenceCompressorParameters, ReferencePeakCompressor,
};
pub use delay::{ReferenceDelayError, ReferenceDelayPair, ReferenceDelayParameters};
pub use gate_expander::{
    ReferenceGateExpanderError, ReferenceGateExpanderParameters, ReferenceGatePhase,
    reference_gate_expander_gain_reduction_db,
};
pub use lr4::{ReferenceLr4Crossover, ReferenceMultibandError};
pub use parametric_eq::{
    ReferenceParametricEqCoefficients, ReferenceParametricEqError, ReferenceParametricEqKind,
    ReferenceParametricEqSection,
};
pub use processor::{IdentityProcessor, OfflineF64Processor, render_planar_f64};
pub use signals::{
    ReferenceSignalError, deterministic_bipolar_noise, deterministic_impulse, deterministic_sine,
};
pub use soft_clip::{
    ReferenceSoftClip, ReferenceSoftClipError, reference_cubic_soft_clip, reference_halfband_63,
};
pub use spectrum::{Complex64, SpectrumError, direct_dft_bin, direct_dft_frequency, magnitude_db};
pub use svf::{
    ReferenceSvf, ReferenceSvfCoefficients, ReferenceSvfError, ReferenceSvfKind,
    ReferenceSvfStateSpace, shelf_slope_to_q,
};
pub use tpt::{
    ReferenceRetainedTptF32, ReferenceTptOutput, ReferenceTptRetainedAction,
    ReferenceTptRetainedStep, ReferenceTptStateSpace, rbj_butterworth_magnitude_db,
};
pub use transient_shaper::{
    ReferenceTransientShaper, ReferenceTransientShaperError, ReferenceTransientShaperParameters,
    reference_transient_shaper_coefficient,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_and_direct_dft_oracles_are_bounded_and_correct() {
        let input = deterministic_impulse(1, 8, 2).expect("impulse");
        let mut identity = IdentityProcessor;
        assert_eq!(
            render_planar_f64(&mut identity, &input).expect("render"),
            input
        );
        let delta = direct_dft_bin(input.channel(0).expect("channel"), 3).expect("dft");
        assert!(delta.re.abs() < 1e-12);
        assert!((delta.im - 1.0).abs() < 1e-12);
        assert_eq!(direct_dft_bin(&[], 0), Err(SpectrumError::EmptyInput));
        assert_eq!(
            direct_dft_bin(&vec![0.0; 4097], 0),
            Err(SpectrumError::TooManyFrames)
        );
    }

    #[test]
    fn sine_frequency_and_silence_are_finite() {
        let sine = deterministic_sine(1, 128, 48_000.0, 1_000.0).expect("sine");
        assert!(
            direct_dft_frequency(sine.channel(0).expect("channel"), 48_000.0, 1_000.0)
                .expect("dft")
                .re
                .is_finite()
        );
        assert_eq!(
            magnitude_db(Complex64 { re: 0.0, im: 0.0 }, -300.0),
            Ok(-300.0)
        );
    }
}
