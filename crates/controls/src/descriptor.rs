//! The control descriptor (decision 17, C3) and the one validator, [`check`].

use effect_contract::{ParameterLattice, ParameterUnit};

use crate::address::Lane;
use crate::error::ControlError;
use crate::{ControlClass, ControlKind, Family, LiveRampRow, RebuildReason, Taper};

/// The issue that makes a route into a submix live (decision 17, slice S6). `0` until root numbers
/// it (numbered by root); its last step flips the route rows from rebuild-only to live.
pub const ROUTE_SLICE: u32 = 0;

/// The issue that adds the effective-value function (decision 17, slice S7a): VCA offset and mute,
/// and a send's `follows_mute`, become live with it. `0` until root numbers it (numbered by root).
pub const EFFECTIVE_SLICE: u32 = 0;

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

/// One named choice of an enumeration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Choice {
    /// The value written to select this choice.
    pub value: f32,
    /// The stable machine label.
    pub label: &'static str,
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

    /// The unit as a person reads it.
    #[must_use]
    pub const fn display_unit(&self) -> &'static str {
        match self.unit {
            ParameterUnit::Db => "dB",
            ParameterUnit::Hz => "Hz",
            ParameterUnit::Milliseconds => "ms",
            ParameterUnit::Samples => "samples",
            ParameterUnit::Ratio => ":1",
            ParameterUnit::Linear => "",
        }
    }

    /// Whether `value` is in this control's domain. Values are not snapped to the lattice.
    #[must_use]
    pub fn contains(&self, value: f32) -> bool {
        if !value.is_finite() {
            return false;
        }
        match self.kind {
            // The exact encodings, as the builtins' own boolean domain has it.
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
                // A sample count is a whole number of frames.
                let whole = self.unit != ParameterUnit::Samples || (value as u32) as f32 == value;
                whole && value >= self.minimum && value <= self.maximum
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
};
