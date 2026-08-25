//! Named nudge sizes: the per-parameter step ladder (issue #127).
//!
//! # Why a per-parameter ladder rather than one global step
//!
//! An agent that wants "threshold down a bit" has to invent a number, and the number it invents is
//! wrong in a different way for every parameter: 0.5 is a sensible move for a dB threshold, a
//! nonsense move for a 10 Hz-to-20 kHz frequency and a catastrophe for a 0..1 mix. A single global
//! `step`/`pageStep` pair -- which is as far as every plugin ABI surveyed for this issue goes
//! (VST3, CLAP, AU, LV2, ARIA, MIDI coarse/fine) -- only moves the problem: the *ratio* between
//! fine and coarse is per-parameter too. So the descriptor declares the ladder, in the parameter's
//! own units, and every consumer names a rung instead of a number.
//!
//! # The five rungs
//!
//! [`NudgeSizeV1`] is `xs / sm / md / lg / xl`, and the four larger rungs are multiples of `xs`
//! chosen by the [`NudgeRatioClassV1`]. One vocabulary is shared by humans and agents; there are
//! deliberately no per-frontend step preferences.
//!
//! # Declared in the parameter's unit, resolved in normalized space
//!
//! A ladder is authored in the unit the parameter is *thought* in -- dB for a level, cents for a
//! frequency, per cent for a time constant or a ratio, whole choices for an enumeration -- which
//! is the form a person can review. It is *resolved* into the mapping's normalized `[0, 1]`
//! domain, because that is the form the arithmetic is exact in: [`map_normalized`] returns the
//! declared minimum at `x == 0` and the declared maximum at `x == 1`, so clamping at a domain edge
//! is exact rather than nearly-exact, and a `Logarithmic` parameter gets equal-ratio stepping out
//! of the mapping itself instead of out of a per-decade banding table.
//!
//! # The grid
//!
//! A nudge does not add a step to wherever the value happens to sit. It rounds the current
//! position to the nearest multiple of the `xs` step, then moves a whole number of `xs` steps from
//! there:
//!
//! ```text
//! k  = round(x / xs)
//! x' = clamp((k + count * multiplier) * xs, 0, 1)
//! ```
//!
//! Two consequences, both wanted. Nudged values land on a fixed, declared grid, so a frequency
//! nudge produces the same handful of values every session instead of an endless supply of
//! `1005.79 Hz`. And from any grid point the operation is exactly reversible: `+1 * size` followed
//! by `-1 * size` restores the starting bits, because both directions are integer arithmetic on
//! `k`. The *first* nudge from an arbitrary starting value snaps by at most half an `xs` step, and
//! at a domain edge the clamp is one-way -- both are documented asymmetries, not rounding drift.
//!
//! # No allocation, and no per-call derivation
//!
//! [`resolve_nudge_ladder_v1`] returns a `Copy` [`ResolvedNudgeLadderV1`]; nothing in this module
//! allocates, and [`NativeEffectRegistry::nudge_ladders`](crate::NativeEffectRegistry::nudge_ladders)
//! resolves every parameter of every registered effect once, at registry construction, so a
//! command path never re-derives one.

use crate::{
    NudgeRatioClassV1, NudgeSizeV1, NudgeStepUnitV1, ParameterDescriptorV1, ParameterDomain,
    ParameterMapping, ParameterUnit, canonical_bits, inverse_map_normalized,
    inverse_map_stepped_normalized, is_negative_zero, map_normalized, map_stepped_normalized,
    parameter_value_valid,
};

impl NudgeSizeV1 {
    /// The five rungs, smallest first. Declaration order is the ladder order.
    pub const ALL: [Self; 5] = [Self::Xs, Self::Sm, Self::Md, Self::Lg, Self::Xl];

    /// The lowercase wire/agent name of this rung.
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

    /// Parse a rung from its lowercase name, or `None` for anything else.
    ///
    /// This is the single place an unknown size name is refused; the control plane turns the
    /// `None` into a typed refusal rather than guessing a neighbouring rung.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|size| size.as_str() == name)
    }

    /// Zero-based position on the ladder.
    #[must_use]
    pub const fn rung(self) -> usize {
        self as usize - 1
    }
}

impl NudgeRatioClassV1 {
    /// The `xs` multiplier of each rung, in [`NudgeSizeV1::ALL`] order.
    ///
    /// `Human` is the shared human/agent vocabulary: five rungs roughly 2-3x apart, so "one md" is
    /// a move a person would recognise as a click of a coarse encoder. `Wide` is the
    /// coarse-to-fine search ladder an agent uses to bracket a value in a few moves; no launch
    /// parameter declares it, and it exists so that adopting it later is a per-parameter edit
    /// rather than a vocabulary change.
    #[must_use]
    pub const fn multipliers(self) -> [u16; 5] {
        match self {
            Self::Human => [1, 3, 5, 10, 30],
            Self::Wide => [1, 4, 16, 64, 256],
        }
    }

    /// The `xs` multiplier of one rung.
    #[must_use]
    pub const fn multiplier(self, size: NudgeSizeV1) -> u16 {
        self.multipliers()[size.rung()]
    }
}

/// One parameter's declared nudge ladder, in the parameter's own unit.
///
/// `xs` is the smallest rung; the other four are `xs` times the [`NudgeRatioClassV1`] multiplier.
/// `step_unit` says what `xs` is measured in, and which mappings the ladder is legal on -- see
/// [`NudgeStepUnitV1`].
///
/// The field is deliberately `xs` rather than a unit-suffixed name (issue #147's convention): the
/// unit is not fixed by the field, it is chosen by the sibling `step_unit` for exactly the same
/// reason [`ParameterDescriptorV1::minimum`] is not `minimum_db`. The unit-in-the-name rule
/// applies to the *names an agent types* -- a parameter's `display_name`, a rung name -- and every
/// unit-bearing number this module publishes carries its unit alongside it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NudgeLadderV1 {
    /// The `xs` rung, measured in `step_unit`. Finite and strictly positive.
    pub xs: f32,
    /// What `xs` is measured in.
    pub step_unit: NudgeStepUnitV1,
    /// How the four larger rungs are derived from `xs`.
    pub ratio_class: NudgeRatioClassV1,
}

impl NudgeLadderV1 {
    /// An `xs` rung of `xs` units of the parameter's own unit, on a `Linear` mapping.
    #[must_use]
    pub const fn absolute(xs: f32) -> Self {
        Self {
            xs,
            step_unit: NudgeStepUnitV1::Absolute,
            ratio_class: NudgeRatioClassV1::Human,
        }
    }

    /// An `xs` rung of `xs` cents, on a `Logarithmic` mapping.
    #[must_use]
    pub const fn cents(xs: f32) -> Self {
        Self {
            xs,
            step_unit: NudgeStepUnitV1::Cents,
            ratio_class: NudgeRatioClassV1::Human,
        }
    }

    /// An `xs` rung of `xs` per cent of the current value, on a `Logarithmic` mapping.
    #[must_use]
    pub const fn percent(xs: f32) -> Self {
        Self {
            xs,
            step_unit: NudgeStepUnitV1::Percent,
            ratio_class: NudgeRatioClassV1::Human,
        }
    }

    /// An `xs` rung of `count` whole enumeration choices, on a `Stepped` mapping.
    #[must_use]
    pub const fn steps(count: u16) -> Self {
        Self {
            xs: count as f32,
            step_unit: NudgeStepUnitV1::Steps,
            ratio_class: NudgeRatioClassV1::Human,
        }
    }
}

/// A ladder resolved against one parameter's domain: five step sizes in normalized `x` space.
///
/// `Copy`, five `f32`s wide, and produced by pure arithmetic -- see the module note on
/// derivation cost.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedNudgeLadderV1 {
    xs_normalized: f32,
    ratio_class: NudgeRatioClassV1,
}

impl ResolvedNudgeLadderV1 {
    /// The `xs` rung as a fraction of the normalized domain.
    #[must_use]
    pub const fn xs_normalized(self) -> f32 {
        self.xs_normalized
    }

    /// The ratio class the four larger rungs are derived with.
    #[must_use]
    pub const fn ratio_class(self) -> NudgeRatioClassV1 {
        self.ratio_class
    }

    /// One rung as a fraction of the normalized domain.
    #[must_use]
    pub fn step_normalized(self, size: NudgeSizeV1) -> f32 {
        self.xs_normalized * f32::from(self.ratio_class.multiplier(size))
    }
}

/// Why a nudge could not be resolved into a value.
///
/// Every variant is a *typed* refusal: the control plane maps it to a refusal reason and never
/// falls back to a guessed value or a silent no-op.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NudgeErrorV1 {
    /// The parameter declares no ladder, so it has no named sizes to move by. Boolean parameters
    /// and any mapping this module cannot step exactly are permanently in this class.
    Undeclared,
    /// The current value is not inside the parameter's declared domain, so there is no normalized
    /// position to step from.
    Value,
    /// `count` is zero: a nudge that moves no rungs is a caller mistake, not a no-op, because it
    /// would still snap the value to the grid.
    Count,
}

/// The `(unit, domain, mapping)` class defaults, in the parameter's own unit.
///
/// # Anchoring
///
/// Each class's `xs` is set at or just above the just-noticeable difference for that kind of
/// quantity, so the smallest rung is the smallest move that is *audible* rather than the smallest
/// move that is representable:
///
/// | class | `xs` | anchor |
/// |---|---|---|
/// | dB, `Linear` | 0.5 dB | level JND is about 0.5-1 dB |
/// | Hz, `Logarithmic` | 20 cents | pitch JND is low tens of cents |
/// | Hz, `Linear` | 1 Hz | no launch parameter; a whole hertz is the smallest round move |
/// | ms, `Logarithmic` | 5 % | time-constant JND is a few per cent |
/// | ms, `Linear` | 0.1 ms | lookahead-class times, where a tenth of a millisecond matters |
/// | ratio, `Logarithmic` | 2.5 % | 0.1 at a ratio of 4:1, the move a console detent makes |
/// | ratio, `Linear` | 0.1 | a ratio-like quantity read in tenths |
/// | linear, `Linear` | 0.01 | a hundredth of a normalized control |
/// | linear, `Logarithmic` | 1 % | no launch parameter |
/// | samples, `Linear` | 1 sample | the quantum of the unit |
/// | any, `Enumeration`/`Stepped` | 1 choice | the quantum of the domain |
///
/// # Why classes are not enough on their own
///
/// A class default is a starting point, never a ruling. Two parameters can share a unit and a
/// mapping and still want different steps -- `band-N-shelf-slope` and a compression ratio are both
/// `Ratio`, and 0.1 is right for one and three times the whole domain for the other. Every effect
/// may override per parameter, and the launch set does exactly that in three places, each with the
/// reason written at the override.
///
/// `Exponential` mappings and `Boolean` domains return `None`: an exponential mapping has no
/// constant-unit step, and a boolean has nothing between its two values. Neither is used by any
/// launch parameter that would want a ladder.
#[must_use]
pub const fn default_nudge_ladder_v1(
    unit: ParameterUnit,
    domain: ParameterDomain,
    mapping: ParameterMapping,
) -> Option<NudgeLadderV1> {
    match (domain, mapping) {
        (ParameterDomain::Enumeration, ParameterMapping::Stepped) => Some(NudgeLadderV1::steps(1)),
        (ParameterDomain::Continuous, ParameterMapping::Linear) => Some(match unit {
            ParameterUnit::Db => NudgeLadderV1::absolute(0.5),
            ParameterUnit::Hz => NudgeLadderV1::absolute(1.0),
            ParameterUnit::Milliseconds => NudgeLadderV1::absolute(0.1),
            ParameterUnit::Samples => NudgeLadderV1::absolute(1.0),
            ParameterUnit::Linear => NudgeLadderV1::absolute(0.01),
            ParameterUnit::Ratio => NudgeLadderV1::absolute(0.1),
        }),
        (ParameterDomain::Continuous, ParameterMapping::Logarithmic) => Some(match unit {
            ParameterUnit::Hz => NudgeLadderV1::cents(20.0),
            ParameterUnit::Db | ParameterUnit::Samples | ParameterUnit::Milliseconds => {
                NudgeLadderV1::percent(5.0)
            }
            ParameterUnit::Linear => NudgeLadderV1::percent(1.0),
            ParameterUnit::Ratio => NudgeLadderV1::percent(2.5),
        }),
        _ => None,
    }
}

/// The class default for one parameter descriptor, ignoring whatever it declares.
///
/// Authoring tests compare this against the declared ladder so that "this parameter deviates from
/// its class" is always a deliberate, listed override rather than a typo nobody noticed.
#[must_use]
pub const fn class_nudge_ladder_v1(p: &ParameterDescriptorV1) -> Option<NudgeLadderV1> {
    default_nudge_ladder_v1(p.unit, p.domain, p.mapping)
}

fn ln(v: f64) -> f64 {
    miso_engine_math::log(v)
}

/// Resolve a parameter's declared ladder into normalized `x` space.
///
/// `None` when the parameter declares no ladder, or when the ladder cannot be expressed in the
/// parameter's domain -- an `Absolute` step on a logarithmic mapping, a `Cents` step on a linear
/// one, a `Steps` step on a continuous one, a step so small it rounds to zero, or one whose `xs`
/// alone would cross the whole domain. Allocation-free.
#[must_use]
pub fn resolve_nudge_ladder_v1(p: &ParameterDescriptorV1) -> Option<ResolvedNudgeLadderV1> {
    resolve_nudge_ladder_parts_v1(
        p.nudge?,
        p.domain,
        p.mapping,
        p.minimum,
        p.maximum,
        p.enum_choices.len(),
    )
}

/// Resolve a ladder against a domain described by its parts rather than by a static descriptor.
///
/// The descriptor wire verifier reads a parameter out of borrowed bytes and has no
/// [`ParameterDescriptorV1`] to hand, so the law lives here and both callers reach it. A second
/// copy of this arithmetic in the verifier is exactly the drift the shared function prevents.
#[must_use]
pub fn resolve_nudge_ladder_parts_v1(
    ladder: NudgeLadderV1,
    domain: ParameterDomain,
    mapping: ParameterMapping,
    minimum: Option<f32>,
    maximum: Option<f32>,
    choice_count: usize,
) -> Option<ResolvedNudgeLadderV1> {
    if !(ladder.xs.is_finite() && ladder.xs > 0.0) {
        return None;
    }
    let xs = f64::from(ladder.xs);
    let bounds = || {
        let (a, b) = minimum.zip(maximum)?;
        (a.is_finite() && b.is_finite() && a < b).then_some((f64::from(a), f64::from(b)))
    };
    let normalized = match (ladder.step_unit, domain, mapping) {
        (NudgeStepUnitV1::Absolute, ParameterDomain::Continuous, ParameterMapping::Linear) => {
            let (a, b) = bounds()?;
            xs / (b - a)
        }
        (NudgeStepUnitV1::Cents, ParameterDomain::Continuous, ParameterMapping::Logarithmic) => {
            let (a, b) = bounds()?;
            if a <= 0.0 {
                return None;
            }
            (xs / 1200.0) * core::f64::consts::LN_2 / ln(b / a)
        }
        (NudgeStepUnitV1::Percent, ParameterDomain::Continuous, ParameterMapping::Logarithmic) => {
            let (a, b) = bounds()?;
            if a <= 0.0 {
                return None;
            }
            ln(1.0 + xs / 100.0) / ln(b / a)
        }
        (NudgeStepUnitV1::Steps, ParameterDomain::Enumeration, ParameterMapping::Stepped) => {
            if choice_count < 2 || xs.fract() != 0.0 {
                return None;
            }
            xs / (choice_count - 1) as f64
        }
        _ => return None,
    };
    let normalized = normalized as f32;
    if !(normalized.is_finite() && normalized > 0.0 && normalized <= 1.0) {
        return None;
    }
    Some(ResolvedNudgeLadderV1 {
        xs_normalized: normalized,
        ratio_class: ladder.ratio_class,
    })
}

/// The normalized position of a legal value in a nudgeable parameter's domain.
fn normalized_position(p: &ParameterDescriptorV1, value: f32) -> Option<f32> {
    match p.domain {
        ParameterDomain::Continuous => {
            let (a, b) = p.minimum.zip(p.maximum)?;
            inverse_map_normalized(p.mapping, a, b, value)
        }
        ParameterDomain::Enumeration => {
            let choices: [f32; 32] = core::array::from_fn(|index| {
                p.enum_choices.get(index).map_or(f32::NAN, |c| c.value)
            });
            let len = p.enum_choices.len().min(choices.len());
            inverse_map_stepped_normalized(&choices[..len], value)
        }
        ParameterDomain::Boolean => None,
    }
}

/// The value at a normalized position in a nudgeable parameter's domain.
fn value_at(p: &ParameterDescriptorV1, x: f32) -> Option<f32> {
    match p.domain {
        ParameterDomain::Continuous => {
            let (a, b) = p.minimum.zip(p.maximum)?;
            map_normalized(p.mapping, a, b, x)
        }
        ParameterDomain::Enumeration => {
            let choices: [f32; 32] = core::array::from_fn(|index| {
                p.enum_choices.get(index).map_or(f32::NAN, |c| c.value)
            });
            let len = p.enum_choices.len().min(choices.len());
            map_stepped_normalized(&choices[..len], x)
        }
        ParameterDomain::Boolean => None,
    }
}

/// Move `current` by `count` rungs of `size`, and return the resolved absolute value.
///
/// The result is the value the parameter should be *set* to; the caller applies it through the
/// ordinary absolute-value parameter path. See the module note for the grid, the first-nudge snap
/// and the one-way clamp at a domain edge.
///
/// # Errors
///
/// [`NudgeErrorV1`] -- an undeclared ladder, a current value outside the declared domain, or a
/// zero `count`.
pub fn nudge_parameter_value_v1(
    p: &ParameterDescriptorV1,
    current: f32,
    size: NudgeSizeV1,
    count: i32,
) -> Result<f32, NudgeErrorV1> {
    let ladder = resolve_nudge_ladder_v1(p).ok_or(NudgeErrorV1::Undeclared)?;
    if count == 0 {
        return Err(NudgeErrorV1::Count);
    }
    if !parameter_value_valid(p, current) {
        return Err(NudgeErrorV1::Value);
    }
    let x = normalized_position(p, current).ok_or(NudgeErrorV1::Value)?;
    let xs = f64::from(ladder.xs_normalized);
    let rungs = f64::from(ladder.ratio_class.multiplier(size)) * f64::from(count);
    let target = ((f64::from(x) / xs).round() + rungs) * xs;
    let clamped = target.clamp(0.0, 1.0) as f32;
    value_at(p, clamped).ok_or(NudgeErrorV1::Value)
}

/// The value one rung down and one rung up from `current`, for a describe-style read.
///
/// Both entries are absolute values in the parameter's own unit, resolved exactly as a nudge would
/// resolve them -- including the grid snap and the edge clamp -- so an agent planning a move reads
/// the same numbers it would get by making it.
///
/// # Errors
///
/// [`NudgeErrorV1`], for the same reasons [`nudge_parameter_value_v1`] does.
pub fn nudge_neighbours_v1(
    p: &ParameterDescriptorV1,
    current: f32,
    size: NudgeSizeV1,
) -> Result<(f32, f32), NudgeErrorV1> {
    Ok((
        nudge_parameter_value_v1(p, current, size, -1)?,
        nudge_parameter_value_v1(p, current, size, 1)?,
    ))
}

/// The three rules a declared [`NudgeLadderV1`] must satisfy, and which one it broke.
///
/// * **Step.** `xs` is finite, strictly positive, and not `-0.0` -- the canonicalisation every
///   other descriptor float obeys. A `Steps` ladder's `xs` is additionally a whole number of
///   choices.
/// * **Domain.** The ladder resolves against this parameter: the step unit is legal for the
///   mapping, the normalized `xs` is inside `(0, 1]`, and -- for a continuous parameter -- so is
///   the largest rung. An `xl` that crossed the whole domain would not be a nudge. A stepped
///   parameter is exempt from the `xl` half of that rule on purpose: `lg` and `xl` are 10 and 30
///   choices, they are *meant* to run off the end of a six-choice enumeration, and the clamp is
///   exact when they do. A `Boolean` parameter must declare no ladder at all.
/// * **Order.** The five resolved rungs strictly ascend. The multipliers make that true by
///   construction, which is exactly why it is checked: a duplicated or out-of-order entry in the
///   multiplier table is otherwise invisible.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NudgeRuleV1 {
    /// `xs` is not a usable magnitude.
    Step,
    /// The ladder does not fit the parameter's domain or mapping.
    Domain,
    /// The resolved rungs do not strictly ascend.
    Order,
}

/// Check a parameter's declared ladder, or `Ok(())` when it declares none.
///
/// # Errors
///
/// The first [`NudgeRuleV1`] broken, in Step-Domain-Order order.
pub fn check_nudge_ladder_v1(p: &ParameterDescriptorV1) -> Result<(), NudgeRuleV1> {
    let Some(ladder) = p.nudge else {
        return Ok(());
    };
    check_nudge_ladder_parts_v1(
        ladder,
        p.domain,
        p.mapping,
        p.minimum,
        p.maximum,
        p.enum_choices.len(),
    )
}

/// Check a ladder against a domain described by its parts.
///
/// The wire verifier's counterpart to [`check_nudge_ladder_v1`]; see
/// [`resolve_nudge_ladder_parts_v1`] for why the parts form exists.
///
/// # Errors
///
/// The first [`NudgeRuleV1`] broken, in Step-Domain-Order order.
pub fn check_nudge_ladder_parts_v1(
    ladder: NudgeLadderV1,
    domain: ParameterDomain,
    mapping: ParameterMapping,
    minimum: Option<f32>,
    maximum: Option<f32>,
    choice_count: usize,
) -> Result<(), NudgeRuleV1> {
    if !ladder.xs.is_finite()
        || ladder.xs <= 0.0
        || is_negative_zero(ladder.xs)
        || canonical_bits(ladder.xs) == canonical_bits(0.0)
        || (matches!(ladder.step_unit, NudgeStepUnitV1::Steps) && ladder.xs.fract() != 0.0)
    {
        return Err(NudgeRuleV1::Step);
    }
    let resolved =
        resolve_nudge_ladder_parts_v1(ladder, domain, mapping, minimum, maximum, choice_count)
            .ok_or(NudgeRuleV1::Domain)?;
    if matches!(domain, ParameterDomain::Continuous)
        && resolved.step_normalized(NudgeSizeV1::Xl) > 1.0
    {
        return Err(NudgeRuleV1::Domain);
    }
    let steps = NudgeSizeV1::ALL.map(|size| resolved.step_normalized(size));
    if steps
        .windows(2)
        .any(|pair| !(pair[0].is_finite() && pair[1].is_finite() && pair[0] < pair[1]))
    {
        return Err(NudgeRuleV1::Order);
    }
    Ok(())
}
