//! The typed refusals of the control validator (decision 17, C5).

use core::fmt;

/// Why a control read or write was refused. The codes are stable; a client matches on
/// [`Self::code`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlError {
    /// `control.address.unknown`: no such owner, section, slot or row.
    AddressUnknown,
    /// `control.read_only`: a derived row.
    ReadOnly,
    /// `control.inactive`: a row the owner's present shape does not use (switch `matrix_mode`
    /// first, in the same transaction).
    Inactive,
    /// `control.lane`: a `left` or `right` lane on a shared row (a read of an unlisted lane is
    /// refused the same way).
    Lane,
    /// `control.domain`: not finite, outside the range, a toggle not exactly 0 or 1, not an
    /// enumeration choice, not a whole number where the unit is samples, or a rate-keyed frequency
    /// above its rate bound.
    Domain,
    /// `control.automation.target`: a stored automation names a row that is not automatable (a
    /// rebuild-only, derived or monitor row, or a setting). Raised by
    /// [`automation_diagnostics`](crate::automation_diagnostics) only; a write never meets it.
    AutomationTarget,
    /// `control.ramp.fixed`: an explicit ramp length on a row whose ramp is fixed (an
    /// acknowledgement must not hide an ignored field).
    RampFixed,
    /// `control.monitor`: [`set`](crate::set) was asked to write a monitor row (solo). A monitor
    /// value lives in the overlay, which is not a session mutation; the overlay writes it.
    Monitor,
    /// `control.ramp.conflict`: one transaction gave two controls that share one ramp row two
    /// different ramp lengths (an explicit length against another, or against none, which means
    /// the session's `control_smoothing`). The row can carry one length, so the later edit is
    /// refused rather than silently deciding for both.
    RampConflict,
    /// `control.effect.changed`: an edit of an effect section's row named the native effect it was
    /// written for (`SET_CONTROL`'s expected effect), and the insert or console slot it addresses
    /// now runs another effect (a rebuild replaced it), or a `cid` one. The numeric row of an
    /// effect's control depends on the effect, so the edit is refused before anything is written,
    /// never applied to the row with the same number of another effect.
    EffectChanged,
}

impl ControlError {
    /// The stable machine code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::AddressUnknown => "control.address.unknown",
            Self::ReadOnly => "control.read_only",
            Self::Inactive => "control.inactive",
            Self::Lane => "control.lane",
            Self::Domain => "control.domain",
            Self::AutomationTarget => "control.automation.target",
            Self::RampFixed => "control.ramp.fixed",
            Self::Monitor => "control.monitor",
            Self::RampConflict => "control.ramp.conflict",
            Self::EffectChanged => "control.effect.changed",
        }
    }
}

impl fmt::Display for ControlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ControlError {}
