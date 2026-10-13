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
    /// `control.ramp.fixed`: an explicit ramp length on a row whose ramp is fixed (an
    /// acknowledgement must not hide an ignored field).
    RampFixed,
    /// `control.monitor`: [`set`](crate::set) was asked to write a monitor row (solo). A monitor
    /// value lives in the overlay, which is not a session mutation; the overlay writes it.
    Monitor,
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
            Self::RampFixed => "control.ramp.fixed",
            Self::Monitor => "control.monitor",
        }
    }
}

impl fmt::Display for ControlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ControlError {}
