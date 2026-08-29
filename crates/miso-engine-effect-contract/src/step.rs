//! Exact-decimal parameter lattices and named step sizes (issue #242).
//!
//! The lattice governs persisted state; live performance values remain continuous `f32`.  The
//! descriptor is the sole authority for the decimal step, its perceptual unit, render precision,
//! and the five named ladder multipliers.  This module is control-plane-only and may allocate.
//!
//! Binding endpoint/default law: #239 ruling 5461507633 section B. Arithmetic rows contain every
//! `min + k * step` interior plus the declared bounds/default. Geometric rows contain the declared
//! minimum, every regular interior point below the maximum, and the declared maximum; a declared
//! default is also an intrinsic member. The endpoint/default detents are deliberately allowed to
//! make one irregular adjacency rather than making a round maximum or a meaningful default
//! unreachable.

use crate::{ParameterDescriptor, ParameterDomain, canonical_bits, is_negative_zero};

/// Five stable named step sizes, smallest first.
impl super::StepSize {
    /// All sizes in ascending ladder order.
    pub const ALL: [Self; 5] = [Self::Xs, Self::Sm, Self::Md, Self::Lg, Self::Xl];

    /// Canonical lowercase spelling used by metadata and agent surfaces.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }

    const fn offset(self) -> usize {
        self as usize - 1
    }
}

/// Per-parameter integer multiples of the declared lattice step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StepLadder {
    /// Multipliers in `xs/sm/md/lg/xl` order.
    pub multiples: [u8; 5],
}

impl StepLadder {
    /// Build a ladder. Descriptor validation rejects zero, nonascending, or unencodable rows.
    #[must_use]
    pub const fn new(multiples: [u8; 5]) -> Self {
        Self { multiples }
    }

    /// The multiplier for one named size.
    #[must_use]
    pub const fn multiple(self, size: super::StepSize) -> u8 {
        self.multiples[size.offset()]
    }

    pub(crate) fn valid(self) -> bool {
        self.multiples[0] != 0
            // The frozen descriptor record packs xs..lg in five bits and xl in six. Keeping
            // that representability rule in the contract validator prevents a static descriptor
            // which its canonical wire cannot carry.
            && self.multiples[..4].iter().all(|multiple| *multiple <= 31)
            && self.multiples[4] <= 63
            && self.multiples.windows(2).all(|pair| pair[0] < pair[1])
    }
}

/// #127's adopted shared human/agent ladder.
pub const DEFAULT_STEP_LADDER: StepLadder = StepLadder::new([1, 3, 5, 10, 30]);
/// The #242 fader example: 0.1 dB quantum with useful perceptual/coarse rungs.
pub const FADER_STEP_LADDER: StepLadder = StepLadder::new([1, 5, 10, 30, 60]);

/// One parameter's persisted-value lattice declaration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterLattice {
    /// Decimal step magnitude. `Cents` means cents; `Ratio` means the multiplicative ratio `r`.
    pub step: f32,
    /// Interpretation of `step`.
    pub step_unit: super::StepUnit,
    /// Fixed digits after the decimal point in every canonical rendering (`0..=8`).
    pub precision: u8,
    /// Named-size multiples of lattice indices.
    pub ladder: StepLadder,
}

impl ParameterLattice {
    /// An arithmetic lattice in the parameter's declared unit.
    #[must_use]
    pub const fn arithmetic(step: f32, precision: u8) -> Self {
        Self {
            step,
            step_unit: super::StepUnit::Absolute,
            precision,
            ladder: DEFAULT_STEP_LADDER,
        }
    }

    /// A cents-spaced logarithmic hertz lattice.
    #[must_use]
    pub const fn cents(step_cents: f32, precision: u8) -> Self {
        Self {
            step: step_cents,
            step_unit: super::StepUnit::Cents,
            precision,
            ladder: DEFAULT_STEP_LADDER,
        }
    }

    /// A geometric non-hertz logarithmic lattice with exact-decimal step ratio `r`.
    #[must_use]
    pub const fn ratio(step_ratio: f32, precision: u8) -> Self {
        Self {
            step: step_ratio,
            step_unit: super::StepUnit::Ratio,
            precision,
            ladder: DEFAULT_STEP_LADDER,
        }
    }

    /// Boolean or enumeration choice-index lattice.
    #[must_use]
    pub const fn indices() -> Self {
        Self {
            step: 1.0,
            step_unit: super::StepUnit::Index,
            precision: 0,
            ladder: DEFAULT_STEP_LADDER,
        }
    }

    /// Replace the default named-size ladder for this parameter.
    #[must_use]
    pub const fn with_ladder(mut self, ladder: StepLadder) -> Self {
        self.ladder = ladder;
        self
    }
}

/// The #127 JND-based class table, materialized into every descriptor's `lattice` field.
///
/// Per-parameter declarations may override this value. In particular, builtins give fader its
/// ruled ladder and every rate-keyed cutoff carries its own endpoint table outside this effect
/// descriptor shape.
#[must_use]
pub const fn default_parameter_lattice(
    unit: crate::ParameterUnit,
    domain: crate::ParameterDomain,
    mapping: crate::ParameterMapping,
) -> ParameterLattice {
    match domain {
        crate::ParameterDomain::Boolean | crate::ParameterDomain::Enumeration => {
            ParameterLattice::indices()
        }
        crate::ParameterDomain::Continuous => match mapping {
            crate::ParameterMapping::Linear => match unit {
                crate::ParameterUnit::Db => ParameterLattice::arithmetic(0.1, 1),
                crate::ParameterUnit::Hz => ParameterLattice::arithmetic(0.001, 3),
                crate::ParameterUnit::Milliseconds => ParameterLattice::arithmetic(0.1, 1),
                crate::ParameterUnit::Samples => ParameterLattice::arithmetic(1.0, 0),
                crate::ParameterUnit::Linear => ParameterLattice::arithmetic(0.01, 2),
                crate::ParameterUnit::Ratio => ParameterLattice::arithmetic(0.1, 1),
            },
            crate::ParameterMapping::Logarithmic => match unit {
                crate::ParameterUnit::Hz => ParameterLattice::cents(20.0, 3),
                crate::ParameterUnit::Milliseconds => ParameterLattice::ratio(1.02, 3),
                // Eight digits preserve the declared Butterworth Q default 0.70710677 exactly.
                crate::ParameterUnit::Ratio => ParameterLattice::ratio(1.02, 8),
                // No shipped log row uses these classes; descriptor validation will still give
                // them a deterministic geometric law if one is added deliberately.
                crate::ParameterUnit::Db
                | crate::ParameterUnit::Samples
                | crate::ParameterUnit::Linear => ParameterLattice::ratio(1.02, 3),
            },
            // `Exponential` is a normalized-control mapping over a finite inclusive value domain;
            // unlike `Logarithmic`, it does not turn the persisted unit into a ratio domain.
            crate::ParameterMapping::Exponential => match unit {
                crate::ParameterUnit::Db => ParameterLattice::arithmetic(0.1, 1),
                crate::ParameterUnit::Hz => ParameterLattice::arithmetic(0.001, 3),
                crate::ParameterUnit::Milliseconds => ParameterLattice::arithmetic(0.1, 1),
                crate::ParameterUnit::Samples => ParameterLattice::arithmetic(1.0, 0),
                crate::ParameterUnit::Linear => ParameterLattice::arithmetic(0.01, 2),
                crate::ParameterUnit::Ratio => ParameterLattice::arithmetic(0.1, 1),
            },
            crate::ParameterMapping::Stepped => ParameterLattice::arithmetic(1.0, 0),
        },
    }
}

/// One sorted legal persisted value and its lossless wire index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LatticePoint {
    /// Zero-based ordinal in ascending numeric order.
    pub index: u32,
    /// The one canonical decimal rendering.
    pub canonical: String,
    /// True for descriptor-declared min/max/default rather than a regular generated interior.
    pub intrinsic: bool,
}

/// Why a descriptor cannot define a finite unambiguous lattice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LatticeError {
    /// Step, precision, ladder, or domain shape is invalid.
    Declaration,
    /// Canonical rounding collapsed or reordered generated points.
    Rendering,
    /// The finite lattice would exceed the bounded control-plane representation.
    TooManyPoints,
}

const MAXIMUM_LATTICE_POINTS: usize = 1_000_000;

/// Render a descriptor number with its pinned precision.
///
/// This is descriptor-authoring conversion, not persisted document conversion. Document values
/// are parsed as exact scaled decimals and matched against these strings before the sole
/// [`decimal_to_f32`] boundary.
#[must_use]
pub fn canonical_descriptor_decimal(value: f32, precision: u8) -> Option<String> {
    if !value.is_finite() || is_negative_zero(value) || precision > 8 {
        return None;
    }
    Some(format!("{:.*}", usize::from(precision), f64::from(value)))
}

/// The sole canonical-decimal-to-engine-`f32` conversion site.
///
/// Callers must first prove `canonical` is a descriptor-generated lattice rendering. Keeping the
/// parse here makes a repo-wide audit of the persisted-value precision boundary mechanical.
#[must_use]
pub fn decimal_to_f32(canonical: &str) -> Option<f32> {
    let value = canonical.parse::<f32>().ok()?;
    value
        .is_finite()
        .then_some(if value == 0.0 { 0.0 } else { value })
}

fn scaled(text: &str, precision: u8) -> Option<i128> {
    let (negative, text) = text
        .strip_prefix('-')
        .map_or((false, text), |rest| (true, rest));
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    if whole.is_empty()
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || fraction.len() != usize::from(precision)
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    let scale = 10_i128.checked_pow(u32::from(precision))?;
    let whole = whole.parse::<i128>().ok()?.checked_mul(scale)?;
    let fraction = if fraction.is_empty() {
        0
    } else {
        fraction.parse::<i128>().ok()?
    };
    let magnitude = whole.checked_add(fraction)?;
    Some(if negative { -magnitude } else { magnitude })
}

fn insert_decimal(
    values: &mut Vec<(i128, String, bool)>,
    text: String,
    precision: u8,
    intrinsic: bool,
) -> Result<(), LatticeError> {
    let value = scaled(&text, precision).ok_or(LatticeError::Rendering)?;
    match values.binary_search_by_key(&value, |(candidate, _, _)| *candidate) {
        Ok(index) => values[index].2 |= intrinsic,
        Err(index) => values.insert(index, (value, text, intrinsic)),
    }
    Ok(())
}

/// Render one intrinsic point and prove the rendering is lossless.
///
/// An intrinsic point is a value the descriptor declares outright -- a bound,
/// a default, an enumeration choice. Its canonical rendering is the only text
/// that ever reaches [`decimal_to_f32`], so a rendering that does not convert
/// back to the declared `f32` would silently move the declared value. The
/// launch case this catches is a bound with more decimals than the row's
/// pinned precision: `0.995` at two decimals renders `1.00`, which is outside
/// its own domain.
fn intrinsic_decimal(value: f32, precision: u8) -> Result<String, LatticeError> {
    let text = canonical_descriptor_decimal(value, precision).ok_or(LatticeError::Declaration)?;
    let restored = decimal_to_f32(&text).ok_or(LatticeError::Rendering)?;
    if canonical_bits(restored) != canonical_bits(value) {
        return Err(LatticeError::Rendering);
    }
    Ok(text)
}

fn intrinsic_values(
    domain: ParameterDomain,
    minimum: Option<f32>,
    maximum: Option<f32>,
    default_value: f32,
    enum_choice_values: &[f32],
    lattice: ParameterLattice,
    maximum_is_member: bool,
) -> Result<Vec<(i128, String, bool)>, LatticeError> {
    let precision = lattice.precision;
    let mut values = Vec::with_capacity(4);
    match domain {
        ParameterDomain::Continuous => {
            let (minimum, maximum) = minimum.zip(maximum).ok_or(LatticeError::Declaration)?;
            // #239 ruling 5461507633 B2/B3: both declared bounds and the
            // declared default are lattice members by declaration. A rate-keyed
            // maximum is not a declared bound -- it is S1's clamp -- so that
            // one shape asks for the top point to be generated, not admitted.
            let mut intrinsic = vec![minimum, default_value];
            if maximum_is_member {
                intrinsic.push(maximum);
            }
            for value in intrinsic {
                insert_decimal(
                    &mut values,
                    intrinsic_decimal(value, precision)?,
                    precision,
                    true,
                )?;
            }
        }
        ParameterDomain::Boolean => {
            insert_decimal(&mut values, "0".to_owned(), 0, true)?;
            insert_decimal(&mut values, "1".to_owned(), 0, true)?;
        }
        ParameterDomain::Enumeration => {
            // The persisted document spells an enumeration as its CHOICE VALUE,
            // so the choice values are the lattice's canonical renderings. The
            // point's `index` remains the choice index, which is what the
            // persist plane carries.
            for value in enum_choice_values {
                insert_decimal(
                    &mut values,
                    intrinsic_decimal(*value, precision)?,
                    precision,
                    true,
                )?;
            }
        }
    }
    Ok(values)
}

/// Build every legal persisted value for an effect parameter in ascending order.
///
/// This is deliberately an off-render/control-plane operation. Registry construction can cache
/// the result; command and render paths never allocate or derive the lattice.
pub fn parameter_lattice_points(
    parameter: &ParameterDescriptor,
) -> Result<Vec<LatticePoint>, LatticeError> {
    let choices: Vec<f32> = parameter
        .enum_choices
        .iter()
        .map(|choice| choice.value)
        .collect();
    parameter_lattice_points_parts(
        parameter.unit,
        parameter.domain,
        parameter.mapping,
        parameter.minimum,
        parameter.maximum,
        parameter.default_value,
        &choices,
        parameter.lattice,
        true,
    )
}

/// Build a lattice from decoded descriptor parts.
///
/// Canonical descriptor-wire verification uses this entry point so the wire and static-contract
/// paths execute one implementation of the endpoint/default and geometric-spacing laws.
#[allow(clippy::too_many_arguments)]
pub fn parameter_lattice_points_parts(
    unit: crate::ParameterUnit,
    domain: ParameterDomain,
    mapping: crate::ParameterMapping,
    minimum: Option<f32>,
    maximum: Option<f32>,
    default_value: f32,
    enum_choice_values: &[f32],
    declaration: ParameterLattice,
    maximum_is_member: bool,
) -> Result<Vec<LatticePoint>, LatticeError> {
    if !(declaration.step.is_finite()
        && declaration.step > 0.0
        && !is_negative_zero(declaration.step)
        && declaration.precision <= 8
        && declaration.ladder.valid())
    {
        return Err(LatticeError::Declaration);
    }
    let expected_unit = match domain {
        ParameterDomain::Boolean | ParameterDomain::Enumeration => super::StepUnit::Index,
        ParameterDomain::Continuous => match mapping {
            crate::ParameterMapping::Linear | crate::ParameterMapping::Exponential => {
                super::StepUnit::Absolute
            }
            crate::ParameterMapping::Logarithmic if unit == crate::ParameterUnit::Hz => {
                super::StepUnit::Cents
            }
            crate::ParameterMapping::Logarithmic => super::StepUnit::Ratio,
            crate::ParameterMapping::Stepped => {
                return Err(LatticeError::Declaration);
            }
        },
    };
    if declaration.step_unit != expected_unit
        || (expected_unit == super::StepUnit::Index
            && (canonical_bits(declaration.step) != canonical_bits(1.0)
                || declaration.precision != 0))
        || (expected_unit == super::StepUnit::Ratio && declaration.step <= 1.0)
    {
        return Err(LatticeError::Declaration);
    }

    let precision = declaration.precision;
    let mut values = intrinsic_values(
        domain,
        minimum,
        maximum,
        default_value,
        enum_choice_values,
        declaration,
        maximum_is_member,
    )?;
    if domain == ParameterDomain::Continuous {
        let minimum = minimum.ok_or(LatticeError::Declaration)?;
        let maximum = maximum.ok_or(LatticeError::Declaration)?;
        let min_text =
            canonical_descriptor_decimal(minimum, precision).ok_or(LatticeError::Declaration)?;
        let max_text =
            canonical_descriptor_decimal(maximum, precision).ok_or(LatticeError::Declaration)?;
        let min_scaled = scaled(&min_text, precision).ok_or(LatticeError::Rendering)?;
        let max_scaled = scaled(&max_text, precision).ok_or(LatticeError::Rendering)?;
        if min_scaled >= max_scaled {
            return Err(LatticeError::Rendering);
        }
        match declaration.step_unit {
            super::StepUnit::Absolute => {
                let step_text = canonical_descriptor_decimal(declaration.step, precision)
                    .ok_or(LatticeError::Declaration)?;
                let step_scaled = scaled(&step_text, precision).ok_or(LatticeError::Rendering)?;
                if step_scaled <= 0 {
                    return Err(LatticeError::Declaration);
                }
                let mut value = min_scaled
                    .checked_add(step_scaled)
                    .ok_or(LatticeError::TooManyPoints)?;
                while if maximum_is_member {
                    value < max_scaled
                } else {
                    value <= max_scaled
                } {
                    if values.len() >= MAXIMUM_LATTICE_POINTS {
                        return Err(LatticeError::TooManyPoints);
                    }
                    let negative = value < 0;
                    let magnitude = value.unsigned_abs();
                    let scale = 10_u128.pow(u32::from(precision));
                    let whole = magnitude / scale;
                    let fraction = magnitude % scale;
                    let text = if precision == 0 {
                        format!("{}{}", if negative { "-" } else { "" }, whole)
                    } else {
                        format!(
                            "{}{}.{:0width$}",
                            if negative { "-" } else { "" },
                            whole,
                            fraction,
                            width = usize::from(precision)
                        )
                    };
                    insert_decimal(&mut values, text, precision, false)?;
                    value = value
                        .checked_add(step_scaled)
                        .ok_or(LatticeError::TooManyPoints)?;
                }
            }
            super::StepUnit::Cents | super::StepUnit::Ratio => {
                if minimum <= 0.0 {
                    return Err(LatticeError::Declaration);
                }
                let step = f64::from(declaration.step);
                let ratio = if declaration.step_unit == super::StepUnit::Cents {
                    miso_engine_math::exp2(step / 1200.0)
                } else {
                    step
                };
                let minimum = f64::from(minimum);
                let maximum = f64::from(maximum);
                let mut k = 1_u32;
                loop {
                    if values.len() >= MAXIMUM_LATTICE_POINTS {
                        return Err(LatticeError::TooManyPoints);
                    }
                    let value = minimum * miso_engine_math::pow(ratio, f64::from(k));
                    if !(value.is_finite()
                        && if maximum_is_member {
                            value < maximum
                        } else {
                            value <= maximum
                        })
                    {
                        break;
                    }
                    let text = format!("{:.*}", usize::from(precision), value);
                    let rendered = scaled(&text, precision).ok_or(LatticeError::Rendering)?;
                    if if maximum_is_member {
                        rendered >= max_scaled
                    } else {
                        rendered > max_scaled
                    } {
                        break;
                    }
                    insert_decimal(&mut values, text, precision, false)?;
                    k = k.checked_add(1).ok_or(LatticeError::TooManyPoints)?;
                }
            }
            super::StepUnit::Index => return Err(LatticeError::Declaration),
        }
    }
    if values.len() > u32::MAX as usize {
        return Err(LatticeError::TooManyPoints);
    }
    Ok(values
        .into_iter()
        .enumerate()
        .map(|(index, (_, canonical, intrinsic))| LatticePoint {
            index: index as u32,
            canonical,
            intrinsic,
        })
        .collect())
}

/// Move a legal persisted index by a named ladder size, clamped to the lattice endpoints.
#[must_use]
pub fn resolve_parameter_step(
    points: &[LatticePoint],
    current: u32,
    size: super::StepSize,
    count: i32,
    ladder: StepLadder,
) -> Option<u32> {
    if points.is_empty() || usize::try_from(current).ok()? >= points.len() || count == 0 {
        return None;
    }
    let delta = i64::from(ladder.multiple(size)) * i64::from(count);
    let target = i64::from(current)
        .saturating_add(delta)
        .clamp(0, i64::try_from(points.len() - 1).ok()?);
    u32::try_from(target).ok()
}

// ---------------------------------------------------------------------------
// Exact-decimal document matching (#242 S2, as amended by #239 ruling
// 5462028562 section B).
//
// Lattice membership of a persisted value is decided on the DOCUMENT'S decimal
// text in exact decimal arithmetic. It is never decided by comparing `f32`
// words: two different decimals routinely round to one `f32`, so an `f32`
// comparison silently admits off-lattice text (the `0.3`-with-step-`0.1`
// class). Equivalent spellings of one number (`0.3`, `0.30`, `3e-1`) are the
// same decimal and are all accepted; the accepted spelling is preserved in the
// document, and only the matched point's canonical rendering ever reaches
// [`decimal_to_f32`].
// ---------------------------------------------------------------------------

/// A decimal literal normalized to sign, integer digits and fraction digits.
///
/// Leading integer zeros and trailing fraction zeros are removed, so two
/// spellings of one number normalize to one value and compare equal without
/// any scaling arithmetic that could overflow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactDecimal {
    negative: bool,
    integer: String,
    fraction: String,
}

/// Longest decimal literal admitted, in significant characters.
///
/// Persisted parameter text far past this cannot be a lattice rendering, and
/// the bound keeps the normalizer's working strings small.
const MAXIMUM_DECIMAL_CHARACTERS: usize = 512;

impl ExactDecimal {
    /// Parse one decimal literal exactly, or report that it is not one.
    ///
    /// Accepts an optional sign, digits with optional `_` separators, an
    /// optional fraction, and an optional decimal exponent. Hexadecimal,
    /// infinite and NaN spellings are rejected: they are not lattice
    /// renderings.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() || text.len() > MAXIMUM_DECIMAL_CHARACTERS {
            return None;
        }
        let (negative, rest) = match text.as_bytes()[0] {
            b'-' => (true, &text[1..]),
            b'+' => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exponent) = match rest.find(['e', 'E']) {
            Some(split) => {
                let raw = &rest[split + 1..];
                let (negative_exponent, digits) = match raw.as_bytes().first() {
                    Some(b'-') => (true, &raw[1..]),
                    Some(b'+') => (false, &raw[1..]),
                    _ => (false, raw),
                };
                if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                    return None;
                }
                // A wilder exponent than this cannot name a lattice point.
                let magnitude = digits.parse::<i32>().ok()?;
                (
                    &rest[..split],
                    if negative_exponent {
                        -magnitude
                    } else {
                        magnitude
                    },
                )
            }
            None => (rest, 0),
        };
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let whole: String = whole
            .chars()
            .filter(|character| *character != '_')
            .collect();
        let fraction: String = fraction
            .chars()
            .filter(|character| *character != '_')
            .collect();
        if whole.is_empty()
            || !whole.bytes().all(|byte| byte.is_ascii_digit())
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || (mantissa.contains('.') && fraction.is_empty())
        {
            return None;
        }
        let mut digits = whole;
        let mut point = digits.len();
        digits.push_str(&fraction);
        // Shift the decimal point by the exponent, padding with zeros on
        // whichever side the shift runs off.
        let shifted = i64::try_from(point).ok()? + i64::from(exponent);
        if shifted < 0 {
            let pad = usize::try_from(-shifted).ok()?;
            if digits.len() + pad > MAXIMUM_DECIMAL_CHARACTERS {
                return None;
            }
            let mut padded = "0".repeat(pad);
            padded.push_str(&digits);
            digits = padded;
            point = 0;
        } else {
            point = usize::try_from(shifted).ok()?;
            if point > digits.len() {
                let pad = point - digits.len();
                if digits.len() + pad > MAXIMUM_DECIMAL_CHARACTERS {
                    return None;
                }
                digits.push_str(&"0".repeat(pad));
            }
        }
        let (integer, fraction) = digits.split_at(point);
        let integer = integer.trim_start_matches('0');
        let fraction = fraction.trim_end_matches('0');
        Some(Self {
            // `-0` and `0` are one lattice value; the descriptor surface has no
            // signed zero (`canonical_descriptor_decimal` refuses `-0.0`).
            negative: negative && !(integer.is_empty() && fraction.is_empty()),
            integer: integer.to_owned(),
            fraction: fraction.to_owned(),
        })
    }
}

impl Ord for ExactDecimal {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;
        match (self.negative, other.negative) {
            (false, true) => return Ordering::Greater,
            (true, false) => return Ordering::Less,
            _ => {}
        }
        let magnitude = self
            .integer
            .len()
            .cmp(&other.integer.len())
            .then_with(|| self.integer.cmp(&other.integer))
            .then_with(|| {
                let width = self.fraction.len().max(other.fraction.len());
                let left = self.fraction.bytes().chain(core::iter::repeat(b'0'));
                let right = other.fraction.bytes().chain(core::iter::repeat(b'0'));
                left.take(width).cmp(right.take(width))
            });
        if self.negative {
            magnitude.reverse()
        } else {
            magnitude
        }
    }
}

impl PartialOrd for ExactDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The two legal values a refused persisted value falls between.
///
/// A bound is absent only when the refused value lies outside the lattice on
/// that side, which is the out-of-domain case rather than the off-lattice one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NearestLatticeValues {
    /// Greatest legal value below the refused one, in canonical rendering.
    pub lower: Option<String>,
    /// Least legal value above the refused one, in canonical rendering.
    pub upper: Option<String>,
}

/// Match one persisted decimal against a lattice in exact decimal arithmetic.
///
/// On success the returned index selects the lattice point whose canonical
/// rendering is the sole input to [`decimal_to_f32`]. On failure the two
/// nearest legal values are named so the refusal can quote them.
///
/// # Errors
///
/// Returns the neighbouring legal values when `text` is not a decimal literal
/// or is not one of `points`.
pub fn lattice_index_for_decimal(
    points: &[LatticePoint],
    text: &str,
) -> Result<u32, NearestLatticeValues> {
    let bracket = |lower: Option<usize>, upper: Option<usize>| NearestLatticeValues {
        lower: lower.map(|index| points[index].canonical.clone()),
        upper: upper.map(|index| points[index].canonical.clone()),
    };
    let Some(value) = ExactDecimal::parse(text) else {
        // Not a decimal literal at all: no side of the lattice brackets it.
        return Err(NearestLatticeValues {
            lower: None,
            upper: None,
        });
    };
    // `points` is ascending by construction, so this is a plain binary search
    // whose comparisons are exact decimal comparisons.
    let mut low = 0_usize;
    let mut high = points.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let candidate = ExactDecimal::parse(&points[middle].canonical)
            .expect("descriptor renderings are decimal literals");
        match candidate.cmp(&value) {
            core::cmp::Ordering::Equal => return Ok(points[middle].index),
            core::cmp::Ordering::Less => low = middle + 1,
            core::cmp::Ordering::Greater => high = middle,
        }
    }
    Err(bracket(
        low.checked_sub(1),
        (low < points.len()).then_some(low),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AutomationRate, EnumChoice, ParameterChannelPolicy, ParameterId, ParameterMapping,
        ParameterUnit, SmoothingRule,
    };

    fn parameter(lattice: ParameterLattice) -> ParameterDescriptor {
        ParameterDescriptor {
            id: ParameterId(1),
            display_name: "frequency",
            display_unit: "Hz",
            unit: ParameterUnit::Hz,
            domain: ParameterDomain::Continuous,
            minimum: Some(10.0),
            maximum: Some(20_000.0),
            default_value: 80.0,
            mapping: ParameterMapping::Logarithmic,
            automation_rate: AutomationRate::Sample,
            channel_policy: ParameterChannelPolicy::PerLane,
            smoothing: SmoothingRule::Linear,
            smoothing_samples: 16,
            readable: true,
            automatable: true,
            enum_choices: &[],
            lattice,
        }
    }

    #[test]
    fn cents_lattice_retains_round_endpoint_and_declared_default() {
        let points = parameter_lattice_points(&parameter(ParameterLattice::cents(20.0, 3)))
            .expect("cents lattice");
        assert_eq!(points.first().unwrap().canonical, "10.000");
        assert_eq!(points.last().unwrap().canonical, "20000.000");
        assert!(
            points
                .iter()
                .any(|point| point.canonical == "80.000" && point.intrinsic)
        );
        assert!(points.last().unwrap().intrinsic);
    }

    #[test]
    fn ratio_lattice_retains_butterworth_default() {
        let mut parameter = parameter(ParameterLattice::ratio(1.02, 8));
        parameter.unit = ParameterUnit::Ratio;
        parameter.display_unit = ":1";
        parameter.minimum = Some(0.1);
        parameter.maximum = Some(18.0);
        parameter.default_value = 0.70710677;
        let points = parameter_lattice_points(&parameter).expect("ratio lattice");
        assert!(
            points
                .iter()
                .any(|point| point.canonical == "0.70710677" && point.intrinsic)
        );
        assert_eq!(points.last().unwrap().canonical, "18.00000000");
    }

    #[test]
    fn index_lattice_uses_choice_indices_not_choice_values() {
        static CHOICES: [EnumChoice; 3] = [
            EnumChoice {
                value: 1.0,
                label: "a",
            },
            EnumChoice {
                value: 4.0,
                label: "b",
            },
            EnumChoice {
                value: 9.0,
                label: "c",
            },
        ];
        let mut parameter = parameter(ParameterLattice::indices());
        parameter.unit = ParameterUnit::Linear;
        parameter.domain = ParameterDomain::Enumeration;
        parameter.mapping = ParameterMapping::Stepped;
        parameter.minimum = None;
        parameter.maximum = None;
        parameter.default_value = 4.0;
        parameter.enum_choices = &CHOICES;
        let points = parameter_lattice_points(&parameter).expect("index lattice");
        assert_eq!(
            points
                .iter()
                .map(|point| point.canonical.as_str())
                .collect::<Vec<_>>(),
            ["0", "1", "2"]
        );
    }
}
