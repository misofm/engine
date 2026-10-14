//! Dual-mono lane helpers shared by the row tables.

use crate::descriptor::Lanes;
use session::Lane;

/// One side of a dual-mono strip, as a row accessor sees it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl Side {
    /// `left` for `Left` and `right` for `Right`.
    pub(crate) fn pick<T>(self, left: T, right: T) -> T {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

impl Lanes {
    /// The lanes this row lists: `left` and `right` for a per-lane row, `both` for a shared one.
    pub(crate) const fn listed(self) -> &'static [Lane] {
        match self {
            Self::PerLane => &[Lane::Left, Lane::Right],
            Self::Shared => &[Lane::Both],
        }
    }

    /// The sides a write to `lane` touches. A shared row has one value, so it is written once
    /// (through `Left`, which its accessor ignores); a per-lane row is written on the named side,
    /// or on both for the `both` alias.
    pub(crate) const fn targets(self, lane: Lane) -> &'static [Side] {
        match (self, lane) {
            (Self::Shared, _) => &[Side::Left],
            (Self::PerLane, Lane::Left) => &[Side::Left],
            (Self::PerLane, Lane::Right) => &[Side::Right],
            (Self::PerLane, Lane::Both) => &[Side::Left, Side::Right],
        }
    }
}

/// Whether a numeric control value means "on".
pub(crate) fn is_on(value: f32) -> bool {
    value.to_bits() == 1.0_f32.to_bits()
}

/// The numeric encoding of a toggle.
pub(crate) fn toggle(on: bool) -> f32 {
    if on { 1.0 } else { 0.0 }
}

impl Side {
    /// The side a per-lane row reads for `lane`. A shared row ignores it.
    pub(crate) const fn of(lane: Lane) -> Self {
        match lane {
            Lane::Right => Self::Right,
            Lane::Left | Lane::Both => Self::Left,
        }
    }
}

/// `value` for the `left` and the `right` side.
pub(crate) fn both_sides<T>(value: impl Fn(Side) -> T) -> [T; 2] {
    [value(Side::Left), value(Side::Right)]
}
