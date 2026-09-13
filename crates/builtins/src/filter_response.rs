//! Control-plane magnitude responses for the prepared input HPF/LPF subtotal.
//!
//! The query owns no render state and does not apply trim, polarity, fader, mute or matrix
//! coefficients. It prepares the caller's complete builtin configuration through the same owner
//! validation used by [`super::BuiltinChain`], then evaluates the two exact rounded SVF sections
//! in each channel. The stationary transfer is derived independently from the recurrence in
//! `lane::kernels::svf_step`; no render call or mutable [`super::InputBuiltins`] is involved.

use core::mem::size_of;

use effect_contract::{
    EffectPrepareError, ObservationCost, ParameterUnit, PreparedResponseAnalysis,
    ResponseAmplitudeReference, ResponseAnalysisDescriptor, ResponseAnalysisError,
    ResponseAnalysisMode, ResponseBypassSemantics, ResponseChannelLayout,
    ResponseConfigurationView, ResponseOutput, ResponsePrepareLimits, ResponseQuery,
    ResponseQueryCadence, ResponseSectionDescriptor, ResponseSectionOutput,
    ResponseSnapshotSection, ResponseSummary, ResponseTotalScope,
};
use engine::{SampleRateHz, is_launch_sample_rate};

use super::{BuiltinParameterError, BuiltinParameters, SvfSection, prepare_sections};

/// The fixed public magnitude floor, in dB relative to unit amplitude.
const INPUT_FILTER_RESPONSE_FLOOR_DB: f32 = -120.0;
const SECTION_COUNT: usize = 2;

static INPUT_FILTER_RESPONSE_SECTIONS: [ResponseSectionDescriptor; SECTION_COUNT] = [
    ResponseSectionDescriptor { id: 1, name: "HPF" },
    ResponseSectionDescriptor { id: 2, name: "LPF" },
];

static INPUT_FILTER_RESPONSE_DESCRIPTOR: ResponseAnalysisDescriptor = ResponseAnalysisDescriptor {
    id: 1,
    name: "Input Filter Response",
    sections: &INPUT_FILTER_RESPONSE_SECTIONS,
    axis_unit: ParameterUnit::Hz,
    unit: ParameterUnit::Db,
    amplitude_reference: ResponseAmplitudeReference::Unity,
    channels: ResponseChannelLayout::Independent,
    mode: ResponseAnalysisMode::RequestedConfiguration,
    cadence: ResponseQueryCadence::ExplicitQuery,
    section_output: ResponseSectionOutput::TotalAndOptionalSections,
    cost: ObservationCost::Computed,
    floor_db: INPUT_FILTER_RESPONSE_FLOOR_DB,
    total_scope: ResponseTotalScope::BuiltinInputFilterSubtotal,
    bypass: ResponseBypassSemantics::NoEffectBypass,
};

/// A caller-owned request for one stationary input-filter response.
#[derive(Clone, Copy, Debug)]
pub struct InputFilterResponseRequest<'a> {
    /// Opaque caller-assigned correlation identity, echoed without narrowing.
    pub configuration_id: u64,
    /// Explicit sample rate used by builtin preparation and response evaluation.
    pub sample_rate_hz: u32,
    /// Complete builtin configuration. Only the input HPF/LPF fields affect the returned values;
    /// all other fields still undergo the ordinary builtin validation.
    pub configuration: BuiltinParameters,
    /// Frequencies in Hz, in strictly increasing order and within `[0, sample_rate / 2]`.
    pub frequencies_hz: &'a [f32],
    /// Caller-supplied point budget. No compiled point ceiling is imposed.
    pub maximum_points: usize,
}

/// Caller-owned output buffers for one input-filter response query.
#[derive(Debug)]
pub struct InputFilterResponseOutput<'a> {
    /// Total HPF/LPF subtotal for the left channel, one value per requested frequency.
    pub total_left_db: &'a mut [f32],
    /// Total HPF/LPF subtotal for the right channel, one value per requested frequency.
    pub total_right_db: &'a mut [f32],
    /// Optional section-major left curves: HPF first, then LPF.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional section-major right curves: HPF first, then LPF.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Semantic mode of an input-filter response result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFilterResponseMode {
    /// The stationary response of the explicitly supplied configuration.
    RequestedConfiguration,
}

/// Metadata accompanying a successful input-filter response query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputFilterResponseSummary {
    /// The caller's opaque correlation identity.
    pub configuration_id: u64,
    /// The stationary response mode used by this query.
    pub mode: InputFilterResponseMode,
    /// The validated launch sample rate.
    pub sample_rate_hz: u32,
    /// Number of points written to each total output.
    pub points: usize,
    /// Fixed public magnitude floor in dB.
    pub floor_db: f32,
    /// Per-channel section enable state in `[HPF, LPF]` order.
    pub enabled_left: [bool; SECTION_COUNT],
    /// Per-channel section enable state in `[HPF, LPF]` order.
    pub enabled_right: [bool; SECTION_COUNT],
}

/// Why an input-filter response request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFilterResponseError {
    /// The explicit rate is outside the four launch-supported rates.
    UnsupportedSampleRate,
    /// The ordinary builtin preparation validator rejected the complete configuration.
    Configuration(BuiltinParameterError),
    /// The frequency grid is empty, non-finite, out of range, duplicated or descending.
    InvalidFrequencyGrid,
    /// The caller's point budget cannot admit the requested grid or section shape arithmetic.
    Capacity,
    /// A total or selected section output does not have its exact required shape.
    OutputShape,
    /// The realized state-space response was non-finite or singular.
    Numerical,
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const ONE: Self = Self { re: 1.0, im: 0.0 };

    #[inline]
    const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    #[inline]
    fn magnitude(self) -> f64 {
        math::sqrt(self.re * self.re + self.im * self.im)
    }
}

#[derive(Clone, Copy)]
struct PointResponse {
    total_db: f32,
    section_db: [f32; SECTION_COUNT],
}

/// Evaluate one rounded TPT section using its independently derived state-space transfer.
///
/// `A`, `B`, `C`, and `D` are obtained from the owner recurrence's two integrators and output
/// mix. At the legal near-Nyquist cutoff maximum, endpoint evaluation can make the determinant
/// small; exact rounded `f32` words and all matrix products are promoted to `f64` to retain
/// useful conditioning. Identity is handled before inversion because its unreduced state-space
/// form has a removable DC singularity. A non-finite or genuinely singular determinant remains a
/// typed numerical refusal rather than a fabricated flat response.
#[inline]
fn section_response(section: SvfSection, z: Complex) -> Result<Complex, InputFilterResponseError> {
    if !section.enabled {
        return Ok(Complex::ONE);
    }

    let c1 = f64::from(section.c1);
    let a2 = f64::from(section.a2);
    let a3 = f64::from(section.a3);
    let m0 = f64::from(section.m0);
    let m1 = f64::from(section.m1);
    let m2 = f64::from(section.m2);

    let a00 = 1.0 - 2.0 * c1;
    let a01 = -2.0 * a2;
    let a10 = 2.0 * a2;
    let a11 = 1.0 - 2.0 * a3;
    let b0 = 2.0 * a2;
    let b1 = 2.0 * a3;
    let c0 = m1 * (1.0 - c1) + m2 * a2;
    let c1_output = -m1 * a2 + m2 * (1.0 - a3);
    let d = m0 + m1 * a2 + m2 * a3;

    let m00 = z.sub(Complex::new(a00, 0.0));
    let m01 = Complex::new(-a01, 0.0);
    let m10 = Complex::new(-a10, 0.0);
    let m11 = z.sub(Complex::new(a11, 0.0));
    let determinant = m00.mul(m11).sub(m01.mul(m10));
    let determinant_norm = determinant.re * determinant.re + determinant.im * determinant.im;
    if !determinant.is_finite() || !determinant_norm.is_finite() || determinant_norm == 0.0 {
        return Err(InputFilterResponseError::Numerical);
    }

    let state0 = Complex::new(b0, 0.0)
        .mul(m11)
        .sub(m01.mul(Complex::new(b1, 0.0)));
    let state1 = m00
        .mul(Complex::new(b1, 0.0))
        .sub(m10.mul(Complex::new(b0, 0.0)));
    let inverse_determinant = Complex::new(
        determinant.re / determinant_norm,
        -determinant.im / determinant_norm,
    );
    let response = Complex::new(d, 0.0)
        .add(Complex::new(c0, 0.0).mul(state0).mul(inverse_determinant))
        .add(
            Complex::new(c1_output, 0.0)
                .mul(state1)
                .mul(inverse_determinant),
        );
    if response.is_finite() {
        Ok(response)
    } else {
        Err(InputFilterResponseError::Numerical)
    }
}

#[inline]
fn magnitude_db(response: Complex) -> Result<f32, InputFilterResponseError> {
    if !response.is_finite() {
        return Err(InputFilterResponseError::Numerical);
    }
    let magnitude = response.magnitude();
    if !magnitude.is_finite() {
        return Err(InputFilterResponseError::Numerical);
    }
    if magnitude == 0.0 {
        return Ok(INPUT_FILTER_RESPONSE_FLOOR_DB);
    }
    if magnitude == 1.0 {
        return Ok(0.0);
    }
    let db = 20.0 * math::log10(magnitude);
    if db == f64::NEG_INFINITY || (db.is_finite() && db < f64::from(INPUT_FILTER_RESPONSE_FLOOR_DB))
    {
        return Ok(INPUT_FILTER_RESPONSE_FLOOR_DB);
    }
    if db.is_finite() {
        let output = db as f32;
        if output.is_finite() {
            Ok(output)
        } else {
            Err(InputFilterResponseError::Numerical)
        }
    } else {
        Err(InputFilterResponseError::Numerical)
    }
}

#[inline]
fn point_response(
    sections: [SvfSection; SECTION_COUNT],
    frequency_hz: f32,
    sample_rate_hz: u32,
) -> Result<PointResponse, InputFilterResponseError> {
    let angle = core::f64::consts::TAU * f64::from(frequency_hz) / f64::from(sample_rate_hz);
    let z = Complex::new(math::cos(angle), math::sin(angle));
    if !z.is_finite() {
        return Err(InputFilterResponseError::Numerical);
    }
    let mut total = Complex::ONE;
    let mut section_db = [0.0_f32; SECTION_COUNT];
    for (index, section) in sections.into_iter().enumerate() {
        let response = section_response(section, z)?;
        section_db[index] = magnitude_db(response)?;
        // Keep the product unfloored. Public section curves may be at the floor while the
        // combined subtotal still has a meaningful response.
        total = total.mul(response);
        if !total.is_finite() {
            return Err(InputFilterResponseError::Numerical);
        }
    }
    Ok(PointResponse {
        total_db: magnitude_db(total)?,
        section_db,
    })
}

fn validate_grid_and_output(
    request: &InputFilterResponseRequest<'_>,
    output: &InputFilterResponseOutput<'_>,
) -> Result<usize, InputFilterResponseError> {
    let points = request.frequencies_hz.len();
    let nyquist = request.sample_rate_hz as f32 * 0.5;
    if points == 0
        || request
            .frequencies_hz
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency < 0.0 || *frequency > nyquist)
        || request
            .frequencies_hz
            .windows(2)
            .any(|pair| pair[1] <= pair[0])
    {
        return Err(InputFilterResponseError::InvalidFrequencyGrid);
    }
    if request.maximum_points == 0 || points > request.maximum_points {
        return Err(InputFilterResponseError::Capacity);
    }
    let section_points = points
        .checked_mul(SECTION_COUNT)
        .ok_or(InputFilterResponseError::Capacity)?;
    if output.total_left_db.len() != points || output.total_right_db.len() != points {
        return Err(InputFilterResponseError::OutputShape);
    }
    if output
        .sections_left_db
        .as_ref()
        .is_some_and(|buffer| buffer.len() != section_points)
        || output
            .sections_right_db
            .as_ref()
            .is_some_and(|buffer| buffer.len() != section_points)
    {
        return Err(InputFilterResponseError::OutputShape);
    }
    Ok(points)
}

fn prepared_sections(
    sample_rate_hz: u32,
    configuration: BuiltinParameters,
) -> Result<[[SvfSection; SECTION_COUNT]; 2], InputFilterResponseError> {
    if !is_launch_sample_rate(SampleRateHz(sample_rate_hz)) {
        return Err(InputFilterResponseError::UnsupportedSampleRate);
    }
    let (input, _, _) = prepare_sections(sample_rate_hz, configuration)
        .map_err(InputFilterResponseError::Configuration)?;
    let track = input.stage.lane_track(0);
    Ok([
        [track.left.hpf, track.left.lpf],
        [track.right.hpf, track.right.lpf],
    ])
}

/// Computes one stationary HPF/LPF subtotal into caller-owned buffers.
///
/// Full builtin preparation and grid/output validation happen before any output write. A fixed
/// numerical preflight is followed by deterministic recomputation, so every typed refusal leaves
/// all caller buffers unchanged without a point-sized heap cache.
pub fn query_input_filter_response_into(
    request: InputFilterResponseRequest<'_>,
    output: InputFilterResponseOutput<'_>,
) -> Result<InputFilterResponseSummary, InputFilterResponseError> {
    let sections = prepared_sections(request.sample_rate_hz, request.configuration)?;
    let points = validate_grid_and_output(&request, &output)?;

    for &frequency_hz in request.frequencies_hz {
        point_response(sections[0], frequency_hz, request.sample_rate_hz)?;
        point_response(sections[1], frequency_hz, request.sample_rate_hz)?;
    }

    let InputFilterResponseOutput {
        total_left_db,
        total_right_db,
        mut sections_left_db,
        mut sections_right_db,
    } = output;
    for (point, &frequency_hz) in request.frequencies_hz.iter().enumerate() {
        let left = point_response(sections[0], frequency_hz, request.sample_rate_hz)?;
        let right = point_response(sections[1], frequency_hz, request.sample_rate_hz)?;
        total_left_db[point] = left.total_db;
        total_right_db[point] = right.total_db;
        if let Some(section_output) = sections_left_db.as_deref_mut() {
            for (section, value) in left.section_db.into_iter().enumerate() {
                section_output[section * points + point] = value;
            }
        }
        if let Some(section_output) = sections_right_db.as_deref_mut() {
            for (section, value) in right.section_db.into_iter().enumerate() {
                section_output[section * points + point] = value;
            }
        }
    }

    Ok(InputFilterResponseSummary {
        configuration_id: request.configuration_id,
        mode: InputFilterResponseMode::RequestedConfiguration,
        sample_rate_hz: request.sample_rate_hz,
        points,
        floor_db: INPUT_FILTER_RESPONSE_FLOOR_DB,
        enabled_left: [sections[0][0].enabled, sections[0][1].enabled],
        enabled_right: [sections[1][0].enabled, sections[1][1].enabled],
    })
}

/// Evaluate copied live HPF/LPF words as unfloored positive magnitudes.
///
/// This is intentionally a word consumer rather than a parameter designer.  It lets a host
/// compose the input subtotal with other linear owners in one f64 log-space pass and apply the
/// public response floor only after all owners have been included.
pub fn query_input_filter_snapshot_magnitudes_into(
    sample_rate_hz: u32,
    bypassed: bool,
    left: &[ResponseSnapshotSection],
    right: &[ResponseSnapshotSection],
    frequencies_hz: &[f32],
    maximum_points: usize,
    output_left: &mut [f64],
    output_right: &mut [f64],
) -> Result<(), InputFilterResponseError> {
    if !is_launch_sample_rate(SampleRateHz(sample_rate_hz)) {
        return Err(InputFilterResponseError::UnsupportedSampleRate);
    }
    let points = frequencies_hz.len();
    let nyquist = sample_rate_hz as f32 * 0.5;
    if points == 0
        || frequencies_hz
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency < 0.0 || *frequency > nyquist)
        || frequencies_hz.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(InputFilterResponseError::InvalidFrequencyGrid);
    }
    if maximum_points == 0 || points > maximum_points {
        return Err(InputFilterResponseError::Capacity);
    }
    if left.len() != SECTION_COUNT
        || right.len() != SECTION_COUNT
        || output_left.len() != points
        || output_right.len() != points
    {
        return Err(InputFilterResponseError::OutputShape);
    }
    let decode = |sections: &[ResponseSnapshotSection]| {
        let mut decoded = [SvfSection::IDENTITY; SECTION_COUNT];
        for (index, section) in sections.iter().enumerate() {
            if section.word_count != 7 {
                return Err(InputFilterResponseError::OutputShape);
            }
            let words: [f32; 7] = core::array::from_fn(|word| f32::from_bits(section.words[word]));
            if words.iter().any(|word| !word.is_finite()) {
                return Err(InputFilterResponseError::Numerical);
            }
            decoded[index] = if section.enabled {
                SvfSection {
                    c1: words[0],
                    a2: words[1],
                    a3: words[2],
                    k: words[3],
                    m0: words[4],
                    m1: words[5],
                    m2: words[6],
                    enabled: true,
                }
            } else {
                SvfSection::IDENTITY
            };
        }
        Ok(decoded)
    };
    let left_sections = decode(left)?;
    let right_sections = decode(right)?;
    for (index, &frequency_hz) in frequencies_hz.iter().enumerate() {
        output_left[index] =
            snapshot_point_magnitude(left_sections, frequency_hz, sample_rate_hz, bypassed)?;
        output_right[index] =
            snapshot_point_magnitude(right_sections, frequency_hz, sample_rate_hz, bypassed)?;
    }
    Ok(())
}

fn snapshot_point_magnitude(
    sections: [SvfSection; SECTION_COUNT],
    frequency_hz: f32,
    sample_rate_hz: u32,
    bypassed: bool,
) -> Result<f64, InputFilterResponseError> {
    if bypassed {
        return Ok(1.0);
    }
    let angle = core::f64::consts::TAU * f64::from(frequency_hz) / f64::from(sample_rate_hz);
    let z = Complex::new(math::cos(angle), math::sin(angle));
    let mut product = Complex::ONE;
    for section in sections {
        product = product.mul(section_response(section, z)?);
        if !product.is_finite() {
            return Err(InputFilterResponseError::Numerical);
        }
    }
    let magnitude = product.magnitude();
    (magnitude.is_finite())
        .then_some(magnitude)
        .ok_or(InputFilterResponseError::Numerical)
}

/// Returns the builtin owner's immutable native response declaration.
pub fn input_filter_response_descriptor() -> &'static ResponseAnalysisDescriptor {
    &INPUT_FILTER_RESPONSE_DESCRIPTOR
}

fn builtin_configuration_error(error: BuiltinParameterError) -> ResponseAnalysisError {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        BuiltinParameterError::EmptyBlock
        | BuiltinParameterError::LaneLength
        | BuiltinParameterError::SampleTimeOverflow => "builtin.resource.arithmetic_overflow",
    };
    ResponseAnalysisError::Configuration(EffectPrepareError { code })
}

fn builtin_response_error(error: InputFilterResponseError) -> ResponseAnalysisError {
    match error {
        InputFilterResponseError::UnsupportedSampleRate => {
            ResponseAnalysisError::Configuration(EffectPrepareError {
                code: "effect.quality.unsupported",
            })
        }
        InputFilterResponseError::Configuration(error) => builtin_configuration_error(error),
        InputFilterResponseError::InvalidFrequencyGrid => {
            ResponseAnalysisError::InvalidFrequencyGrid
        }
        InputFilterResponseError::Capacity => ResponseAnalysisError::Capacity,
        InputFilterResponseError::OutputShape => ResponseAnalysisError::OutputShape,
        InputFilterResponseError::Numerical => ResponseAnalysisError::Numerical,
    }
}

struct PreparedInputFilterResponse {
    sample_rate_hz: u32,
    configuration: BuiltinParameters,
    enabled_left: [bool; SECTION_COUNT],
    enabled_right: [bool; SECTION_COUNT],
}

/// Prepares the builtin owner's response provider from a complete immutable builtin configuration.
pub fn prepare_input_filter_response(
    sample_rate_hz: u32,
    configuration: BuiltinParameters,
    limits: ResponsePrepareLimits,
) -> Result<Box<dyn PreparedResponseAnalysis>, ResponseAnalysisError> {
    if size_of::<PreparedInputFilterResponse>() > limits.maximum_prepared_bytes {
        return Err(ResponseAnalysisError::ResourceLimit);
    }
    let sections =
        prepared_sections(sample_rate_hz, configuration).map_err(builtin_response_error)?;
    let provider = PreparedInputFilterResponse {
        sample_rate_hz,
        configuration,
        enabled_left: [sections[0][0].enabled, sections[0][1].enabled],
        enabled_right: [sections[1][0].enabled, sections[1][1].enabled],
    };
    Ok(Box::new(provider))
}

impl PreparedResponseAnalysis for PreparedInputFilterResponse {
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor {
        &INPUT_FILTER_RESPONSE_DESCRIPTOR
    }

    fn configuration(&self) -> ResponseConfigurationView<'_> {
        ResponseConfigurationView {
            sample_rate_hz: self.sample_rate_hz,
            enabled_left: &self.enabled_left,
            enabled_right: &self.enabled_right,
            bypass: None,
        }
    }

    fn retained_bytes(&self) -> usize {
        size_of::<Self>()
    }

    fn query_into(
        &self,
        query: ResponseQuery<'_>,
        output: ResponseOutput<'_>,
    ) -> Result<ResponseSummary, ResponseAnalysisError> {
        let summary = query_input_filter_response_into(
            InputFilterResponseRequest {
                configuration_id: query.configuration_id,
                sample_rate_hz: self.sample_rate_hz,
                configuration: self.configuration,
                frequencies_hz: query.frequencies_hz,
                maximum_points: query.maximum_points,
            },
            InputFilterResponseOutput {
                total_left_db: output.total_left_db,
                total_right_db: output.total_right_db,
                sections_left_db: output.sections_left_db,
                sections_right_db: output.sections_right_db,
            },
        )
        .map_err(builtin_response_error)?;
        Ok(ResponseSummary {
            configuration_id: summary.configuration_id,
            mode: ResponseAnalysisMode::RequestedConfiguration,
            sample_rate_hz: summary.sample_rate_hz,
            points: summary.points,
            floor_db: summary.floor_db,
        })
    }
}
