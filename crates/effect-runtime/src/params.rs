//! Parameter domain validation, clamping and defaults, driven by a descriptor.
//!
//! The audit found the same domain predicate copied into six effect crates and reinvented in two
//! more, in every case reduced to the continuous case only, because the contract's own
//! `parameter_value_valid` is private. This module is one implementation covering all three
//! domain kinds.
//!
//! # `-0.0`
//!
//! Negative zero is the recurring bug in the copies: five crates reject it explicitly at prepare,
//! one accepts it, and the contract's own predicate accepts it by normalising the comparison. The
//! rule here is a single one — **`-0.0` is a valid way to write zero and is normalised to `+0.0`
//! on the way in** ([`normalize_zero`]). A parameter that reaches a kernel as `-0.0` changes the
//! sign of the first sample it multiplies, so it is never stored; but rejecting a control message
//! for writing zero the other way is not a service to anyone.
//!
//! # Mapping
//!
//! The mapping describes how a normalised `[0, 1]` control position becomes a value, which is a
//! control-plane concern: a fader is linear in its own travel and logarithmic in frequency. The
//! logarithmic mapping is exact at both ends by construction (`t = 0` returns `minimum` and
//! `t = 1` returns `maximum`) and uses `math`, never the platform libm (D6).

use math::{exp2f, log2f};

/// How a normalised `[0, 1]` position maps onto a parameter's range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterMapping {
    /// `minimum + t * (maximum - minimum)`.
    Linear,
    /// Constant ratio per unit of travel: `minimum * (maximum / minimum)^t`. Requires a strictly
    /// positive `minimum`; falls back to [`ParameterMapping::Linear`] otherwise, because a
    /// logarithmic sweep through zero does not exist.
    Logarithmic,
}

/// What values a parameter admits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterKind {
    /// Every finite value in `[minimum, maximum]`.
    Continuous,
    /// Exactly `0.0` or `1.0`.
    Boolean,
    /// Exactly one of a fixed set of values.
    Enumeration(&'static [f32]),
}

/// The domain, mapping and default of one parameter.
///
/// Deliberately small: this is what validation and clamping need, and nothing else. An effect's
/// full descriptor — identifier, unit, automation rate, smoothing rule — stays with the effect and
/// with the contract.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterSpec {
    /// Which values are admitted.
    pub kind: ParameterKind,
    /// Lowest admitted value, for [`ParameterKind::Continuous`].
    pub minimum: f32,
    /// Highest admitted value, for [`ParameterKind::Continuous`].
    pub maximum: f32,
    /// How a normalised position maps onto `[minimum, maximum]`.
    pub mapping: ParameterMapping,
    /// The value a freshly prepared effect starts from.
    pub default: f32,
}

impl ParameterSpec {
    /// A continuous, linearly mapped parameter.
    #[must_use]
    pub const fn continuous(minimum: f32, maximum: f32, default: f32) -> Self {
        Self {
            kind: ParameterKind::Continuous,
            minimum,
            maximum,
            mapping: ParameterMapping::Linear,
            default,
        }
    }

    /// A continuous, logarithmically mapped parameter — a frequency or a time constant.
    #[must_use]
    pub const fn logarithmic(minimum: f32, maximum: f32, default: f32) -> Self {
        Self {
            kind: ParameterKind::Continuous,
            minimum,
            maximum,
            mapping: ParameterMapping::Logarithmic,
            default,
        }
    }

    /// A boolean parameter, admitting exactly `0.0` and `1.0`.
    #[must_use]
    pub const fn boolean(default: f32) -> Self {
        Self {
            kind: ParameterKind::Boolean,
            minimum: 0.0,
            maximum: 1.0,
            mapping: ParameterMapping::Linear,
            default,
        }
    }
}

/// `+0.0` for either zero, the value unchanged otherwise.
///
/// `value == 0.0` is true for `-0.0`, which is the whole trick: no bit inspection is needed.
#[must_use]
pub fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

/// `true` if `value` is the negative zero bit pattern.
#[must_use]
pub fn is_negative_zero(value: f32) -> bool {
    value.to_bits() == (-0.0_f32).to_bits()
}

/// `true` if `value` is inside the parameter's domain.
///
/// Non-finite values are rejected first, for every kind: an infinity is inside no range and a NaN
/// compares false against both ends, so the explicit rejection is what makes the answer the same
/// on every target rather than a consequence of comparison order.
///
/// `-0.0` compares equal to `+0.0` throughout, so a zero written either way is accepted wherever
/// zero is admitted.
#[must_use]
pub fn parameter_value_valid(spec: &ParameterSpec, value: f32) -> bool {
    if !value.is_finite() {
        return false;
    }
    match spec.kind {
        ParameterKind::Continuous => value >= spec.minimum && value <= spec.maximum,
        ParameterKind::Boolean => value == 0.0 || value == 1.0,
        ParameterKind::Enumeration(choices) => choices.contains(&value),
    }
}

/// Brings `value` into the parameter's domain.
///
/// * Continuous — clamped to `[minimum, maximum]` with the D8 select form, written as explicit
///   comparisons rather than `f32::clamp` so that the NaN rule is the engine's and not the
///   standard library's: **NaN clamps to the default**, it does not propagate into a coefficient.
/// * Boolean — anything at or above `0.5` becomes `1.0`, anything below becomes `+0.0`, NaN
///   becomes the default.
/// * Enumeration — an exact match is kept; anything else becomes the default.
///
/// The result is always a valid value for the spec, and never `-0.0`.
#[must_use]
pub fn clamp_to_domain(spec: &ParameterSpec, value: f32) -> f32 {
    if !value.is_finite() {
        return normalize_zero(spec.default);
    }
    let clamped = match spec.kind {
        ParameterKind::Continuous => {
            if value < spec.minimum {
                spec.minimum
            } else if value > spec.maximum {
                spec.maximum
            } else {
                value
            }
        }
        ParameterKind::Boolean => {
            if value >= 0.5 {
                1.0
            } else {
                0.0
            }
        }
        ParameterKind::Enumeration(choices) => {
            if choices.contains(&value) {
                value
            } else {
                spec.default
            }
        }
    };
    normalize_zero(clamped)
}

/// Maps a normalised position `t` in `[0, 1]` onto the parameter's range.
///
/// `t` is clamped to `[0, 1]` first, so a control surface that overshoots cannot leave the domain.
/// The result is always a valid value for the spec.
///
/// Frozen operation order, logarithmic: `lo = log2(minimum)`, `hi = log2(maximum)`,
/// `exp2(lo + t * (hi - lo))`, with `t == 0` and `t == 1` short-circuited to the exact endpoints
/// so a mapping round-trip is exact where it is observable.
#[must_use]
pub fn map_normalized(spec: &ParameterSpec, t: f32) -> f32 {
    let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
    if t == 0.0 {
        return clamp_to_domain(spec, spec.minimum);
    }
    if t == 1.0 {
        return clamp_to_domain(spec, spec.maximum);
    }
    let value = match spec.mapping {
        ParameterMapping::Logarithmic if spec.minimum > 0.0 && spec.maximum > 0.0 => {
            let lo = log2f(spec.minimum);
            let hi = log2f(spec.maximum);
            exp2f(lo + t * (hi - lo))
        }
        _ => spec.minimum + t * (spec.maximum - spec.minimum),
    };
    clamp_to_domain(spec, value)
}

/// The normalised position of `value`: the inverse of [`map_normalized`].
///
/// Returns `0.0` for a degenerate range (`maximum <= minimum`), where no position is meaningful.
#[must_use]
pub fn inverse_map_normalized(spec: &ParameterSpec, value: f32) -> f32 {
    let value = clamp_to_domain(spec, value);
    if spec.minimum.is_nan() || spec.maximum.is_nan() || spec.maximum <= spec.minimum {
        return 0.0;
    }
    let t = match spec.mapping {
        ParameterMapping::Logarithmic if spec.minimum > 0.0 && spec.maximum > 0.0 => {
            let lo = log2f(spec.minimum);
            let hi = log2f(spec.maximum);
            (log2f(value) - lo) / (hi - lo)
        }
        _ => (value - spec.minimum) / (spec.maximum - spec.minimum),
    };
    if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) }
}

/// Fills `out` with each spec's default, clamped into its own domain.
///
/// Returns the number of values written, which is `min(specs.len(), out.len())`. A default that
/// its own descriptor would reject is clamped rather than trusted: a table with a typo in it
/// should prepare an effect at the edge of its range, not outside it.
pub fn initial_defaults(specs: &[ParameterSpec], out: &mut [f32]) -> usize {
    let count = core::cmp::min(specs.len(), out.len());
    for (slot, spec) in out[..count].iter_mut().zip(specs[..count].iter()) {
        *slot = clamp_to_domain(spec, spec.default);
    }
    count
}
