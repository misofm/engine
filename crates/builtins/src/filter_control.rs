//! Pure preparation and validation for live builtin input-filter targets.

use super::{
    BUTTERWORTH_K, BuiltinLaneSelector, BuiltinParameterError, SvfSection,
    validate_builtin_filter_cutoff,
};

/// The fixed coefficient update window for live input HPF/LPF targets.
pub const INPUT_FILTER_RAMP_SAMPLES: u32 = 64;

/// A final semantic HPF/LPF pair, normalized so disabled values are positive zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputFilterPair {
    pub hpf_hz: f32,
    pub lpf_hz: f32,
}

/// One prepared section target.  The six words are `[c1, a2, a3, m0, m1, m2]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedInputFilterTarget {
    pub lanes: BuiltinLaneSelector,
    pub section: u32,
    /// The final full pair is association data; only `section` is written by the target.
    pub pair: [f32; 2],
    pub coefficients: [f32; 6],
}

/// The two section targets produced for one final pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedInputFilterPair {
    pub pair: InputFilterPair,
    pub targets: [PreparedInputFilterTarget; 2],
}

/// Validate and normalize a semantic HPF/LPF pair using the existing cutoff authority.
pub fn validate_input_filter_pair(
    sample_rate_hz: u32,
    hpf_hz: f32,
    lpf_hz: f32,
) -> Result<InputFilterPair, BuiltinParameterError> {
    validate_builtin_filter_cutoff(hpf_hz, sample_rate_hz, 0.0, 10.0)?;
    validate_builtin_filter_cutoff(lpf_hz, sample_rate_hz, 0.0, 10.0)?;
    if hpf_hz > 0.0 && lpf_hz > 0.0 && hpf_hz >= lpf_hz {
        return Err(BuiltinParameterError::FilterOrder);
    }
    Ok(InputFilterPair {
        hpf_hz: if hpf_hz == 0.0 { 0.0 } else { hpf_hz },
        lpf_hz: if lpf_hz == 0.0 { 0.0 } else { lpf_hz },
    })
}

/// Design and validate the fixed HPF/LPF target pair using the existing builtin SVF authority.
pub fn prepare_input_filter_pair(
    sample_rate_hz: u32,
    hpf_hz: f32,
    lpf_hz: f32,
) -> Result<PreparedInputFilterPair, BuiltinParameterError> {
    let pair = validate_input_filter_pair(sample_rate_hz, hpf_hz, lpf_hz)?;
    let sections = [
        SvfSection::design(sample_rate_hz, pair.hpf_hz, true)?,
        SvfSection::design(sample_rate_hz, pair.lpf_hz, false)?,
    ];
    let targets = core::array::from_fn(|section| PreparedInputFilterTarget {
        lanes: BuiltinLaneSelector::Both,
        section: section as u32,
        pair: [pair.hpf_hz, pair.lpf_hz],
        coefficients: section_words(sections[section]),
    });
    for target in targets {
        validate_prepared_input_filter_target(&target)?;
    }
    Ok(PreparedInputFilterPair { pair, targets })
}

/// Check the trusted prepared target contract before it reaches an input stage.
pub fn validate_prepared_input_filter_target(
    target: &PreparedInputFilterTarget,
) -> Result<(), BuiltinParameterError> {
    if target.section > 1
        || !target
            .pair
            .iter()
            .all(|value| value.is_finite() && (value.to_bits() == 0 || *value > 0.0))
        || (target.pair[0] > 0.0 && target.pair[1] > 0.0 && target.pair[0] >= target.pair[1])
        || !target
            .coefficients
            .iter()
            .all(|value| normal_or_zero(*value))
    {
        return Err(BuiltinParameterError::FilterCoefficients);
    }
    let c = target.coefficients;
    let identity = [0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0];
    let is_identity = c
        .iter()
        .zip(identity)
        .all(|(a, b)| a.to_bits() == b.to_bits());
    let disabled = target.pair[target.section as usize].to_bits() == 0.0_f32.to_bits();
    if disabled {
        return is_identity
            .then_some(())
            .ok_or(BuiltinParameterError::FilterCoefficients);
    }
    if is_identity {
        return Err(BuiltinParameterError::FilterCoefficients);
    }
    let expected_mix = if target.section == 0 {
        [1.0_f32, -BUTTERWORTH_K, -1.0]
    } else {
        [0.0_f32, 0.0, 1.0]
    };
    if c[0] <= 0.0
        || c[0] > 1.0
        || c[1] <= 0.0
        || !(0.0..=1.0).contains(&c[2])
        || !c[3..]
            .iter()
            .zip(expected_mix)
            .all(|(a, b)| a.to_bits() == b.to_bits())
    {
        return Err(BuiltinParameterError::FilterCoefficients);
    }
    // Spectral norm of the rounded-SVF state matrix, evaluated in bounded f64 arithmetic.
    let a00 = 1.0 - 2.0 * f64::from(c[0]);
    let a01 = -2.0 * f64::from(c[1]);
    let a10 = 2.0 * f64::from(c[1]);
    let a11 = 1.0 - 2.0 * f64::from(c[2]);
    let frobenius_sq = a00 * a00 + a01 * a01 + a10 * a10 + a11 * a11;
    let determinant = a00 * a11 - a01 * a10;
    let discriminant = (frobenius_sq * frobenius_sq - 4.0 * determinant * determinant).max(0.0);
    let norm_sq = 0.5 * (frobenius_sq + math::sqrt(discriminant));
    let norm = math::sqrt(norm_sq);
    if !norm.is_finite() || norm > 1.0 + (1.0 / 4_194_304.0) {
        return Err(BuiltinParameterError::FilterCoefficients);
    }
    Ok(())
}

fn section_words(section: SvfSection) -> [f32; 6] {
    [
        section.c1, section.a2, section.a3, section.m0, section.m1, section.m2,
    ]
}

fn normal_or_zero(value: f32) -> bool {
    value.is_finite() && !value.is_subnormal()
}
