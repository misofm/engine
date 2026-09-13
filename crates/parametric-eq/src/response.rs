//! Control-plane magnitude response evaluation for the native parametric EQ.
//!
//! The query deliberately lives beside the effect rather than in an engine or host adapter.  It
//! evaluates the same rounded `f32` coefficient words that the render kernel receives, while its
//! state-space arithmetic is an independent implementation of the recurrence in `lane::kernels`.
//! It never reads a prepared effect or any mutable render state.

use effect_contract::{EffectPrepareError, PrepareEffectRequest, expected_prepared_metadata};
use engine::{SampleRateHz, is_launch_sample_rate};

use super::{BandTarget, EQ_SECTION_COUNT, EqSvfWords, PARAMETRIC_EQ_DESCRIPTOR, band_targets};

/// The fixed magnitude floor returned by this query, in dB relative to unit amplitude.
const EQ_RESPONSE_FLOOR_DB: f32 = -120.0;

/// A caller-owned request for one stationary EQ response.
#[derive(Clone, Copy, Debug)]
pub struct EqResponseRequest<'a> {
    /// Opaque caller-assigned correlation token, echoed by [`query_response_into`].
    pub configuration_id: u64,
    /// An immutable, already validated effect-owned response configuration.
    pub configuration: &'a EqResponseConfiguration,
    /// Frequencies in Hz, in the order to return. The grid must be finite, strictly increasing,
    /// and inside the inclusive `[0, sample_rate / 2]` interval.
    pub frequencies_hz: &'a [f32],
    /// Caller-owned point budget. No compiled point ceiling is imposed by this API.
    pub maximum_points: usize,
}

/// An immutable response configuration prepared from an ordinary EQ preparation request.
///
/// Preparation runs on the control/worker plane and owns the sample-rate, bypass/enable flags,
/// and the exact rounded coefficient words. The source parameter slice is not retained, so later
/// caller mutation cannot change a query. Preparation may use the existing descriptor validator's
/// transient allocations; querying this owned value never validates descriptors or allocates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqResponseConfiguration {
    sample_rate_hz: u32,
    bypass: bool,
    enabled_left: [bool; EQ_SECTION_COUNT],
    enabled_right: [bool; EQ_SECTION_COUNT],
    left_words: [EqSvfWords; EQ_SECTION_COUNT],
    right_words: [EqSvfWords; EQ_SECTION_COUNT],
}

impl EqResponseConfiguration {
    /// Validates and prepares an immutable response configuration from the ordinary effect request.
    ///
    /// This does not instantiate or process a prepared effect. Its full request validation is the
    /// same `expected_prepared_metadata` path used by the effect factory.
    pub fn prepare(request: PrepareEffectRequest<'_>) -> Result<Self, EqResponseError> {
        let metadata = expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, request)
            .map_err(EqResponseError::Configuration)?;
        let sample_rate = SampleRateHz(metadata.sample_rate);
        if !is_launch_sample_rate(sample_rate) {
            return Err(EqResponseError::Configuration(EffectPrepareError {
                code: "effect.quality.unsupported",
            }));
        }
        let left_targets = band_targets(request.initial_values, 0, sample_rate)
            .map_err(EqResponseError::Configuration)?;
        let right_targets = band_targets(request.initial_values, 1, sample_rate)
            .map_err(EqResponseError::Configuration)?;
        Ok(Self {
            sample_rate_hz: metadata.sample_rate,
            bypass: metadata.bypass,
            enabled_left: core::array::from_fn(|section| left_targets[section].enabled),
            enabled_right: core::array::from_fn(|section| right_targets[section].enabled),
            left_words: realized_words(&left_targets, sample_rate)?,
            right_words: realized_words(&right_targets, sample_rate)?,
        })
    }
}

/// Caller-owned buffers for one response query.
#[derive(Debug)]
pub struct EqResponseOutput<'a> {
    /// Total left-channel magnitude, one word per requested frequency.
    pub total_left_db: &'a mut [f32],
    /// Total right-channel magnitude, one word per requested frequency.
    pub total_right_db: &'a mut [f32],
    /// Optional left-channel section curves in section-major order.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional right-channel section curves in section-major order.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Semantic mode of an EQ response result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqResponseMode {
    /// The stationary response of the explicitly supplied configuration.
    RequestedConfiguration,
}

/// Metadata accompanying a successful response query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqResponseSummary {
    /// The caller's opaque correlation token.
    pub configuration_id: u64,
    /// The stationary response mode used by this query.
    pub mode: EqResponseMode,
    /// The validated preparation sample rate.
    pub sample_rate_hz: u32,
    /// Number of points written to each total output.
    pub points: usize,
    /// Fixed public magnitude floor in dB.
    pub floor_db: f32,
    /// Whether the effect-wide bypass was requested. Total curves are exact 0 dB when true.
    pub bypass: bool,
    /// Per-section left-channel enable state in descriptor order.
    pub enabled_left: [bool; EQ_SECTION_COUNT],
    /// Per-section right-channel enable state in descriptor order.
    pub enabled_right: [bool; EQ_SECTION_COUNT],
}

/// Why a response request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqResponseError {
    /// The ordinary effect preparation validator rejected the configuration.
    Configuration(EffectPrepareError),
    /// The frequency grid is empty, non-finite, out of range, duplicated, or descending.
    InvalidFrequencyGrid,
    /// The caller's point budget cannot admit the requested grid or section shape arithmetic.
    Capacity,
    /// One total or selected section buffer does not have its exact required shape.
    OutputShape,
    /// The realized state-space response was not finite or was singular.
    Numerical,
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    #[inline]
    fn new(re: f64, im: f64) -> Self {
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
    section_db: [f32; EQ_SECTION_COUNT],
}

/// Evaluate the transfer of one rounded section from its realized recurrence.
///
/// For `A`, `B`, `C`, and `D`, see issue #764's frozen derivation. Identity words are handled
/// before inversion because their state-space representation has a removable DC singularity.
#[inline]
fn section_response(words: EqSvfWords, z: Complex) -> Result<Complex, EqResponseError> {
    if words == EqSvfWords::IDENTITY {
        return Ok(Complex::new(1.0, 0.0));
    }

    let c1 = f64::from(words.c1);
    let a2 = f64::from(words.a2);
    let a3 = f64::from(words.a3);
    let m0 = f64::from(words.m0);
    let m1 = f64::from(words.m1);
    let m2 = f64::from(words.m2);

    let a00 = 1.0 - 2.0 * c1;
    let a01 = -2.0 * a2;
    let a10 = 2.0 * a2;
    let a11 = 1.0 - 2.0 * a3;
    let b0 = 2.0 * a2;
    let b1 = 2.0 * a3;
    let c0 = m1 * (1.0 - c1) + m2 * a2;
    let c_1 = -m1 * a2 + m2 * (1.0 - a3);
    let d = m0 + m1 * a2 + m2 * a3;

    let m00 = z.sub(Complex::new(a00, 0.0));
    let m01 = Complex::new(-a01, 0.0);
    let m10 = Complex::new(-a10, 0.0);
    let m11 = z.sub(Complex::new(a11, 0.0));
    let determinant = m00.mul(m11).sub(m01.mul(m10));
    let determinant_norm = determinant.re * determinant.re + determinant.im * determinant.im;
    if !determinant.is_finite() || !determinant_norm.is_finite() || determinant_norm == 0.0 {
        return Err(EqResponseError::Numerical);
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
        .add(Complex::new(c_1, 0.0).mul(state1).mul(inverse_determinant));
    if response.is_finite() {
        Ok(response)
    } else {
        Err(EqResponseError::Numerical)
    }
}

#[inline]
fn magnitude_db(response: Complex) -> Result<f32, EqResponseError> {
    if !response.is_finite() {
        return Err(EqResponseError::Numerical);
    }
    let magnitude = response.magnitude();
    if !magnitude.is_finite() {
        return Err(EqResponseError::Numerical);
    }
    if magnitude == 0.0 {
        return Ok(EQ_RESPONSE_FLOOR_DB);
    }
    let db = 20.0 * math::log10(magnitude);
    if db == f64::NEG_INFINITY || (db.is_finite() && db < f64::from(EQ_RESPONSE_FLOOR_DB)) {
        return Ok(EQ_RESPONSE_FLOOR_DB);
    }
    if db.is_finite() {
        let output = db as f32;
        if output.is_finite() {
            Ok(output)
        } else {
            Err(EqResponseError::Numerical)
        }
    } else {
        Err(EqResponseError::Numerical)
    }
}

#[inline]
fn point_response(
    words: &[EqSvfWords; EQ_SECTION_COUNT],
    frequency_hz: f32,
    sample_rate_hz: u32,
    bypass: bool,
) -> Result<PointResponse, EqResponseError> {
    let angle = core::f64::consts::TAU * f64::from(frequency_hz) / f64::from(sample_rate_hz);
    let z = Complex::new(math::cos(angle), math::sin(angle));
    if !z.is_finite() {
        return Err(EqResponseError::Numerical);
    }
    let mut product = Complex::new(1.0, 0.0);
    let mut section_db = [0.0_f32; EQ_SECTION_COUNT];
    for (index, section_words) in words.iter().copied().enumerate() {
        let section = section_response(section_words, z)?;
        let db = magnitude_db(section)?;
        section_db[index] = db;
        // Keep the product unfloored. Public section curves may be at the floor while the cascade
        // still has a meaningful response after a later section supplies gain.
        product = product.mul(section);
        if !product.is_finite() {
            return Err(EqResponseError::Numerical);
        }
    }
    let total_db = if bypass { 0.0 } else { magnitude_db(product)? };
    Ok(PointResponse {
        total_db,
        section_db,
    })
}

fn validate_grid_and_output(
    request: EqResponseRequest<'_>,
    output: &EqResponseOutput<'_>,
    sample_rate_hz: u32,
) -> Result<usize, EqResponseError> {
    let points = request.frequencies_hz.len();
    if points == 0
        || request.frequencies_hz.iter().any(|frequency| {
            !frequency.is_finite() || *frequency < 0.0 || *frequency > sample_rate_hz as f32 * 0.5
        })
        || request
            .frequencies_hz
            .windows(2)
            .any(|pair| pair[1] <= pair[0])
    {
        return Err(EqResponseError::InvalidFrequencyGrid);
    }
    if request.maximum_points == 0 || points > request.maximum_points {
        return Err(EqResponseError::Capacity);
    }
    let section_points = points
        .checked_mul(EQ_SECTION_COUNT)
        .ok_or(EqResponseError::Capacity)?;
    if output.total_left_db.len() != points || output.total_right_db.len() != points {
        return Err(EqResponseError::OutputShape);
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
        return Err(EqResponseError::OutputShape);
    }
    Ok(points)
}

fn realized_words(
    targets: &[BandTarget; EQ_SECTION_COUNT],
    sample_rate: SampleRateHz,
) -> Result<[EqSvfWords; EQ_SECTION_COUNT], EqResponseError> {
    let mut words = [EqSvfWords::IDENTITY; EQ_SECTION_COUNT];
    for (section, target) in targets.iter().enumerate() {
        words[section] = target.words(sample_rate).map_err(|_| {
            EqResponseError::Configuration(EffectPrepareError {
                code: "effect.eq.coefficients",
            })
        })?;
    }
    Ok(words)
}

/// Computes one stationary response into caller-owned buffers.
///
/// Preparation validation and all numerical work complete before the first output word is
/// published. This preserves caller sentinels on every typed refusal, including a late numerical
/// failure. The evaluator is control/worker-plane only and has no access to mutable render state.
pub fn query_response_into(
    request: EqResponseRequest<'_>,
    output: EqResponseOutput<'_>,
) -> Result<EqResponseSummary, EqResponseError> {
    let points = validate_grid_and_output(request, &output, request.configuration.sample_rate_hz)?;

    // Preflight the complete grid before mutating any caller buffer. The fixed-size point result
    // is intentionally recomputed during publication instead of retaining a point-sized cache.
    for &frequency_hz in request.frequencies_hz {
        point_response(
            &request.configuration.left_words,
            frequency_hz,
            request.configuration.sample_rate_hz,
            request.configuration.bypass,
        )?;
        point_response(
            &request.configuration.right_words,
            frequency_hz,
            request.configuration.sample_rate_hz,
            request.configuration.bypass,
        )?;
    }

    let EqResponseOutput {
        total_left_db,
        total_right_db,
        mut sections_left_db,
        mut sections_right_db,
    } = output;
    for (point, &frequency_hz) in request.frequencies_hz.iter().enumerate() {
        let left = point_response(
            &request.configuration.left_words,
            frequency_hz,
            request.configuration.sample_rate_hz,
            request.configuration.bypass,
        )?;
        let right = point_response(
            &request.configuration.right_words,
            frequency_hz,
            request.configuration.sample_rate_hz,
            request.configuration.bypass,
        )?;
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

    Ok(EqResponseSummary {
        configuration_id: request.configuration_id,
        mode: EqResponseMode::RequestedConfiguration,
        sample_rate_hz: request.configuration.sample_rate_hz,
        points,
        floor_db: EQ_RESPONSE_FLOOR_DB,
        bypass: request.configuration.bypass,
        enabled_left: request.configuration.enabled_left,
        enabled_right: request.configuration.enabled_right,
    })
}
