//! The control descriptor (decision 17, C3) and the one validator, [`check`].

use builtins::BuiltinParameterDomain;
use effect_contract::{
    ParameterDescriptor, ParameterLattice, ParameterUnit, parameter_value_valid,
};

use crate::error::ControlError;
use crate::{ControlClass, ControlKind, Family, LiveRampRow, RebuildReason, Taper};
use session::Lane;

/// The class of a value that a ruling makes live and whose issue has not landed.
pub(crate) const fn ruled_live_not_built(issue: u32, reopen: &'static str) -> ControlClass {
    ControlClass::RebuildOnly {
        reason: RebuildReason::RuledLiveNotBuilt { issue },
        reopen,
    }
}

/// Whether a row holds one value per dual-mono lane or one value for its owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lanes {
    /// One value per lane (a fader, a trim, an effect parameter): listed as `left` and `right`.
    PerLane,
    /// One value for the owner (a matrix coefficient, a route gain): listed as `both`.
    Shared,
}

/// Where a control's value lives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlScope {
    /// In the session document: a write is a session mutation that advances the revision.
    Model,
    /// Host-local and never saved (solo): a write is not a session mutation.
    Monitor,
}

/// How a live change to a control is ramped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Motion {
    /// The `control_smoothing` row that gives the ramp length. A write may carry its own length.
    Ramp(LiveRampRow),
    /// A ramp that the value's own kernel owns (an effect's coefficient ramp, the HPF and LPF
    /// coefficient ramp, the bypass crossfade), or none. A write may not carry a length.
    Fixed,
}

/// One named choice of an enumeration: the effect contract's own type, so an effect parameter's
/// choices are used as declared and not copied.
pub use effect_contract::EnumChoice as Choice;

/// Which function decides whether a value is in a control's domain.
///
/// A builtin row and an effect parameter already have a domain function that preparation and
/// session validation use; the control core calls that same function, so there is one rule per
/// value and the row cannot drift from it. Only the core's own rows (a strip's own rows, a
/// route's, a VCA's, the session's, and the bypass and link-mode rows) are decided here, by the
/// descriptor's own bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DomainRule {
    /// The descriptor's own `kind`, `minimum`, `maximum`, `disabled_value` and `choices`.
    Range,
    /// A builtin row's [`BuiltinParameterDomain`] at the session's sample rate:
    /// `BuiltinParameterDomain::contains`, which for a filter cutoff is
    /// `validate_builtin_filter_cutoff`, the check session validation uses.
    Builtin {
        /// The row's declared domain.
        domain: BuiltinParameterDomain,
        /// The session's sample rate.
        sample_rate: u32,
    },
    /// An effect parameter's contract row: `effect_contract::parameter_value_valid`.
    Contract(&'static ParameterDescriptor),
}

/// What one control is: everything a frontend or an agent needs to draw it, and everything the
/// validator needs to check a write. The same type describes every control.
///
/// `minimum` and `maximum` are resolved at the session's sample rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlDescriptor {
    /// The stable machine name (`fader_db`).
    pub name: &'static str,
    /// The name a person reads.
    pub display_name: &'static str,
    /// The unit of the value.
    pub unit: ParameterUnit,
    /// The unit as a person reads it (`dB`, `Hz`, `ms`, `samples`), or `""` for a unitless value.
    pub display_unit: &'static str,
    /// Continuous, toggle or enumeration.
    pub kind: ControlKind,
    /// Per-lane or shared.
    pub lanes: Lanes,
    /// The inclusive lower bound.
    pub minimum: f32,
    /// The inclusive upper bound, at the session's sample rate.
    pub maximum: f32,
    /// A value outside the range that also is accepted (a filter's `0` is "off").
    pub disabled_value: Option<f32>,
    /// The value of a new session.
    pub default: f32,
    /// How a frontend maps the range to a control.
    pub taper: Taper,
    /// The persisted-value lattice step and the named step ladder.
    pub lattice: ParameterLattice,
    /// The choices of an enumeration; empty for any other kind.
    pub choices: &'static [Choice],
    /// How a live change is ramped.
    pub motion: Motion,
    /// Live, rebuild-only (with its reason) or derived.
    pub class: ControlClass,
    /// Whether the value is in the session or in the host.
    pub scope: ControlScope,
    /// The render family that carries a live change.
    pub family: Family,
    /// `false` for a row the owner's present shape does not use (the matrix rows of a strip in pan
    /// mode, and the reverse). The row set changes only with the session's structure, so an
    /// inactive row is listed and refuses a write.
    pub active: bool,
    /// Which function decides the domain (see [`DomainRule`]).
    pub domain: DomainRule,
}

impl ControlDescriptor {
    /// Whether an agent may automate this control. Computed, never declared: a live model value
    /// that render reads (decision 14 rule 4 with decision 15, "automatable iff live").
    #[must_use]
    pub fn automatable(&self) -> bool {
        self.class == ControlClass::Live
            && self.scope == ControlScope::Model
            && self.family != Family::Setting
    }

    /// Whether `value` is in this control's domain. Values are not snapped to the lattice.
    ///
    /// A builtin row and an effect parameter delegate to the function the rest of the engine uses
    /// for them ([`DomainRule`]); the descriptor's own bounds decide only the core's own rows.
    #[must_use]
    pub fn contains(&self, value: f32) -> bool {
        match self.domain {
            DomainRule::Contract(parameter) => parameter_value_valid(parameter, value),
            DomainRule::Builtin {
                domain,
                sample_rate,
            } => domain.contains(value, sample_rate) && self.is_whole_if_samples(value),
            DomainRule::Range => value.is_finite() && self.range_contains(value),
        }
    }

    /// A sample count is a whole number of frames.
    fn is_whole_if_samples(&self, value: f32) -> bool {
        self.unit != ParameterUnit::Samples || (value as u32) as f32 == value
    }

    /// The core's own rows: the descriptor's kind and bounds.
    fn range_contains(&self, value: f32) -> bool {
        match self.kind {
            // The exact encodings, as the builtins' boolean domain has them: `-0.0` is refused.
            ControlKind::Toggle => {
                value.to_bits() == 0.0_f32.to_bits() || value.to_bits() == 1.0_f32.to_bits()
            }
            ControlKind::Enumeration => self.choices.iter().any(|choice| choice.value == value),
            ControlKind::Continuous => {
                if self
                    .disabled_value
                    .is_some_and(|disabled| value.to_bits() == disabled.to_bits())
                {
                    return true;
                }
                self.is_whole_if_samples(value) && value >= self.minimum && value <= self.maximum
            }
        }
    }
}

/// The one validator (decision 17, C5): whether a write of `value` to the control `descriptor`
/// describes, on `lane`, with an optional explicit ramp length in samples, is acceptable. It reads
/// nothing but its arguments and allocates nothing.
///
/// The refusals come in this order: a derived row (`control.read_only`), an inactive row
/// (`control.inactive`), a lane the row does not accept (`control.lane`), a value outside the
/// domain (`control.domain`), an explicit ramp on a fixed-ramp row (`control.ramp.fixed`).
///
/// A shared row accepts only `both`. A per-lane row accepts `left`, `right` and `both`, where
/// `both` writes both lanes.
///
/// # Errors
///
/// The refusal above, as a [`ControlError`].
pub fn check(
    descriptor: &ControlDescriptor,
    lane: Lane,
    value: f32,
    ramp: Option<u32>,
) -> Result<(), ControlError> {
    if descriptor.class == ControlClass::Derived {
        return Err(ControlError::ReadOnly);
    }
    if !descriptor.active {
        return Err(ControlError::Inactive);
    }
    if descriptor.lanes == Lanes::Shared && lane != Lane::Both {
        return Err(ControlError::Lane);
    }
    if !descriptor.contains(value) {
        return Err(ControlError::Domain);
    }
    if ramp.is_some() && descriptor.motion == Motion::Fixed {
        return Err(ControlError::RampFixed);
    }
    Ok(())
}

/// The neutral descriptor that the row tables of this crate update with `..BASE`: a live,
/// active, shared, unit-less model value in `[0, 1]` that no render family reads.
pub(crate) const BASE: ControlDescriptor = ControlDescriptor {
    name: "",
    display_name: "",
    unit: ParameterUnit::Linear,
    display_unit: "",
    kind: ControlKind::Continuous,
    lanes: Lanes::Shared,
    minimum: 0.0,
    maximum: 1.0,
    disabled_value: None,
    default: 0.0,
    taper: Taper::Linear,
    lattice: ParameterLattice::arithmetic(0.01, 2),
    choices: &[],
    motion: Motion::Fixed,
    class: ControlClass::Live,
    scope: ControlScope::Model,
    family: Family::Setting,
    active: true,
    domain: DomainRule::Range,
};
