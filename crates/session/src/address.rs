//! The control address (decision 17, C2): stable IDs, never indices.
//!
//! It lives in `session` because a stored automation target is a [`ControlAddress`] (slice
//! S12a, #1566); `controls` re-exports every name.

use crate::{ParameterChannel, RackName, StableId};

/// Which side of a dual-mono strip a control value lives on: `left`, `right`, or `both` (the
/// existing `ParameterChannel` codes).
///
/// A per-lane row is listed as a `left` row and a `right` row and never as `both`; a shared row is
/// listed once, as `both`. On a write, `both` on a per-lane row is an alias that sets both lanes
/// in one edit; `left` or `right` on a shared row is refused as `control.lane`.
pub type Lane = ParameterChannel;

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

/// The field-free kind of an [`Owner`]: what a host that holds no model names a row's owner by
/// (decision 17, slice S7c). Slice S3 gives the kinds wire codes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OwnerKind {
    /// [`Owner::Session`].
    Session,
    /// [`Owner::Strip`].
    Strip,
    /// [`Owner::Route`].
    Route,
    /// [`Owner::Vca`].
    Vca,
}

impl OwnerKind {
    /// Every owner kind, in code order.
    pub const ALL: [Self; 4] = [Self::Session, Self::Strip, Self::Route, Self::Vca];

    /// The kind's stable address code: session 1, strip 2, route 3, VCA 4 (#1551 D3). A code is
    /// never reused.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Session => 1,
            Self::Strip => 2,
            Self::Route => 3,
            Self::Vca => 4,
        }
    }

    /// The kind's name in the SDK's address (`session`, `strip`, `route`, `vca`).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Strip => "strip",
            Self::Route => "route",
            Self::Vca => "vca",
        }
    }
}

impl Owner {
    /// The owner's stable ID: absent for the session, which has none.
    #[must_use]
    pub const fn id(&self) -> Option<&StableId> {
        match self {
            Self::Session => None,
            Self::Strip(id) | Self::Route(id) | Self::Vca(id) => Some(id),
        }
    }

    /// The field-free kind of this owner.
    #[must_use]
    pub const fn kind(&self) -> OwnerKind {
        match self {
            Self::Session => OwnerKind::Session,
            Self::Strip(_) => OwnerKind::Strip,
            Self::Route(_) => OwnerKind::Route,
            Self::Vca(_) => OwnerKind::Vca,
        }
    }
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

/// The field-free kind of a [`Section`], as [`OwnerKind`] is of an [`Owner`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SectionKind {
    /// [`Section::Own`].
    Own,
    /// [`Section::Builtins`].
    Builtins,
    /// [`Section::Console`].
    Console,
    /// [`Section::Inserts`].
    Inserts,
}

impl SectionKind {
    /// Every section kind, in code order.
    pub const ALL: [Self; 4] = [Self::Inserts, Self::Builtins, Self::Console, Self::Own];

    /// The kind's stable code: the `RackName` codes (inserts 2, builtins 4, console 5) and `own` 6,
    /// appended. A code is never reused.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Inserts => RackName::Inserts.wire(),
            Self::Builtins => RackName::Builtins.wire(),
            Self::Console => RackName::Console.wire(),
            Self::Own => 6,
        }
    }

    /// The kind's name in the SDK's address (`inserts`, `builtins`, `console`, `own`).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Inserts => "inserts",
            Self::Builtins => "builtins",
            Self::Console => "console",
            Self::Own => "own",
        }
    }
}

impl Section {
    /// The field-free kind of this section.
    #[must_use]
    pub const fn kind(&self) -> SectionKind {
        match self {
            Self::Own => SectionKind::Own,
            Self::Builtins => SectionKind::Builtins,
            Self::Console(_) => SectionKind::Console,
            Self::Inserts(_) => SectionKind::Inserts,
        }
    }

    /// The section's stable code: the `RackName` codes (inserts 2, builtins 4, console 5) and
    /// `own` 6, appended.
    #[must_use]
    pub const fn code(&self) -> u8 {
        self.kind().code()
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
