//! The control address (decision 17, C2): stable IDs, never indices.

use session::{RackName, StableId};

/// Which side of a dual-mono strip a control value lives on: `left`, `right`, or `both` (the
/// existing `ParameterChannel` codes).
///
/// A per-lane row is listed as a `left` row and a `right` row and never as `both`; a shared row is
/// listed once, as `both`. On a write, `both` on a per-lane row is an alias that sets both lanes
/// in one edit; `left` or `right` on a shared row is refused as `control.lane`.
pub type Lane = session::ParameterChannel;

/// The entity that owns a control.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Owner {
    /// The session itself (the ramp lengths).
    Session,
    /// A strip: a track or a submix share one ID namespace.
    Strip(StableId),
    /// A route.
    Route(StableId),
    /// A VCA group.
    Vca(StableId),
}

/// The part of an owner that a control belongs to.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Section {
    /// The owner's own rows: a strip's `matrix_mode`, a route's gain, a VCA's offset.
    Own,
    /// A strip's builtin section (trim, polarity, filters, fader, mute, pan or matrix, delay).
    Builtins,
    /// A strip's knobs for one session console slot.
    Console(StableId),
    /// One of a strip's inserts.
    Inserts(StableId),
}

impl Section {
    /// The section's stable code: the `RackName` codes (inserts 2, builtins 4, console 5) and
    /// `own` 6, appended.
    #[must_use]
    pub const fn code(&self) -> u8 {
        match self {
            Self::Inserts(_) => RackName::Inserts.wire(),
            Self::Builtins => RackName::Builtins.wire(),
            Self::Console(_) => RackName::Console.wire(),
            Self::Own => 6,
        }
    }
}

/// The address of one control.
///
/// `control` is the row ID inside the section: the builtin parameter ID, or the ID of a row in the
/// owner's own table. In an effect section, `0` is the effect's bypass and every other value is
/// the effect's parameter ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlAddress {
    /// Who owns the control.
    pub owner: Owner,
    /// Which part of the owner.
    pub section: Section,
    /// The row ID inside the section.
    pub control: u32,
    /// Which lane.
    pub lane: Lane,
}

impl ControlAddress {
    /// Whether `other` names the same row of the same owner and section, on any lane.
    pub(crate) fn same_row(&self, other: &Self) -> bool {
        self.owner == other.owner && self.section == other.section && self.control == other.control
    }
}
