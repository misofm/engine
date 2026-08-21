//! Independent `f64` curve and transition primitives for the launch gate/expander.
//!
//! This small oracle deliberately owns no production types. It is a direct transcription of the
//! frozen mathematical contract for focused conformance tests, not a realtime processor.

/// One f64 gate/expander curve input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceGateExpanderParameters {
    /// Threshold in dB.
    pub threshold_db: f64,
    /// Downward-expansion ratio.
    pub ratio: f64,
    /// Maximum attenuation in dB.
    pub range_db: f64,
}

/// The externally observable hysteresis state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceGatePhase {
    /// The gate is attenuating according to the expansion curve.
    Closed,
    /// The gate is fully open.
    Open,
}

/// The input is outside the frozen launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceGateExpanderError {
    /// A curve parameter or level was non-finite or invalid.
    InvalidInput,
}

/// Computes the frozen static attenuation target in dB.
///
/// The detector level is expressed in dB after its `[-160, 24]` clamp. An open gate is exact
/// unity; a closed gate applies the hard downward-expansion curve and the explicit range cap.
pub fn reference_gate_expander_gain_reduction_db(
    level_db: f64,
    parameters: ReferenceGateExpanderParameters,
    phase: ReferenceGatePhase,
) -> Result<f64, ReferenceGateExpanderError> {
    if !level_db.is_finite()
        || !(-160.0..=24.0).contains(&level_db)
        || !parameters.threshold_db.is_finite()
        || !(-80.0..=0.0).contains(&parameters.threshold_db)
        || !parameters.ratio.is_finite()
        || !(1.0..=20.0).contains(&parameters.ratio)
        || !parameters.range_db.is_finite()
        || !(0.0..=96.0).contains(&parameters.range_db)
    {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    if phase == ReferenceGatePhase::Open {
        return Ok(0.0);
    }
    Ok(
        ((parameters.ratio - 1.0) * (level_db - parameters.threshold_db))
            .clamp(-parameters.range_db, 0.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_is_identity_at_ratio_one_and_range_limited_when_closed() {
        let identity = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 1.0,
            range_db: 80.0,
        };
        assert_eq!(
            reference_gate_expander_gain_reduction_db(-80.0, identity, ReferenceGatePhase::Closed),
            Ok(0.0)
        );
        let limited = ReferenceGateExpanderParameters {
            ratio: 20.0,
            range_db: 12.0,
            ..identity
        };
        assert_eq!(
            reference_gate_expander_gain_reduction_db(-80.0, limited, ReferenceGatePhase::Closed),
            Ok(-12.0)
        );
    }
}
