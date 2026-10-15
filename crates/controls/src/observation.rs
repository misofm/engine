//! The host's observation subscriptions (#1584): which effect taps a host armed and with which
//! window, kept apart from the monitor overlay (solo and subscriptions are different things).
//!
//! A subscription is host-local state. It is never saved and never in a snapshot, and it needs no
//! revision. A rebuild must not lose it: the host prepares each successor plan from the bindings
//! the transaction leaves ([`ObservationBindings::after`]), so a tap the host acknowledged is armed
//! in the successor from its first rendered block. A binding whose target the next model no longer
//! has ends, and [`ObservationChange::ended`] names it, so the host tells its subscribers.

use std::collections::TryReserveError;

use session::{Section, SessionModel, StableId};

use crate::effect_rows::resident_observation_tap;
use crate::registry::strip_index;

/// The effect section a tap belongs to: a console slot or an insert of one strip.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TapSection {
    /// A session console slot, by its slot ID.
    Console(StableId),
    /// One of the strip's inserts, by its effect ID.
    Inserts(StableId),
}

impl TapSection {
    /// The ID of the console slot or the insert.
    #[must_use]
    pub fn id(&self) -> &StableId {
        match self {
            Self::Console(id) | Self::Inserts(id) => id,
        }
    }

    /// The control section this names.
    #[must_use]
    pub fn section(&self) -> Section {
        match self {
            Self::Console(id) => Section::Console(id.clone()),
            Self::Inserts(id) => Section::Inserts(id.clone()),
        }
    }

    /// Whether this is a console slot.
    #[must_use]
    pub const fn is_console(&self) -> bool {
        matches!(self, Self::Console(_))
    }
}

/// The stable key of one observation tap: the strip, the effect section and the effect-local tap
/// ID. It names no plan position, so it survives every rebuild while the effect exists.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TapKey {
    /// The strip (a track or a submix).
    pub strip: StableId,
    /// The console slot or insert.
    pub section: TapSection,
    /// The effect-local tap ID (`ObservationDescriptor::id`).
    pub tap_id: u32,
}

/// One armed tap and the window it was armed with.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBinding {
    /// The tap.
    pub key: TapKey,
    /// The window length in blocks as it was asked for; `0` is the plan's meter window, which each
    /// plan resolves for itself.
    pub window_blocks: u32,
}

/// The taps a host has armed, sorted by key.
///
/// In this slice a binding is an absolute set or clear (the browser's observe kinds 7 and 8); a
/// shared arm with reference counts belongs to the transaction edit that arms through the core.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObservationBindings {
    /// Sorted by key, with no duplicate key.
    bindings: Vec<ObservationBinding>,
}

/// What [`ObservationBindings::after`] returns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationChange {
    /// The bindings whose target the next model still has.
    pub bindings: ObservationBindings,
    /// The keys of the bindings the change ended, in key order: the strip, the insert or the slot
    /// is gone, or the effect behind the ID has no resident tap with that ID.
    pub ended: Vec<TapKey>,
}

impl ObservationBindings {
    /// No binding. `const`, so a host may keep one in a `static`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    /// The bindings, sorted by key.
    #[must_use]
    pub fn bindings(&self) -> &[ObservationBinding] {
        &self.bindings
    }

    /// How many taps are armed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Whether no tap is armed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// The window `key` is armed with, or `None` when it is not armed.
    #[must_use]
    pub fn window_of(&self, key: &TapKey) -> Option<u32> {
        self.search(key)
            .ok()
            .map(|index| self.bindings[index].window_blocks)
    }

    /// Arm `key` with `window_blocks`, replacing any earlier window.
    pub fn set(&mut self, key: TapKey, window_blocks: u32) {
        match self.search(&key) {
            Ok(index) => self.bindings[index].window_blocks = window_blocks,
            Err(index) => self
                .bindings
                .insert(index, ObservationBinding { key, window_blocks }),
        }
    }

    /// Disarm `key`. Returns whether it was armed.
    pub fn clear(&mut self, key: &TapKey) -> bool {
        match self.search(key) {
            Ok(index) => {
                self.bindings.remove(index);
                true
            }
            Err(_) => false,
        }
    }

    fn search(&self, key: &TapKey) -> Result<usize, usize> {
        self.bindings
            .binary_search_by(|binding| binding.key.cmp(key))
    }

    /// Whether the next model still has the target of `key`: the strip, the section's instance
    /// and a resident tap of that ID.
    #[must_use]
    pub fn has_target(model: &SessionModel, key: &TapKey) -> bool {
        strip_index(model, &key.strip).is_some()
            && resident_observation_tap(model, &key.strip, &key.section.section(), key.tap_id)
                .is_some()
    }

    /// The bindings on a session whose strips and effects are `next`'s: each binding whose target
    /// `next` no longer has is dropped and its key returned in [`ObservationChange::ended`].
    ///
    /// Both lists are reserved exactly before anything is copied, so a refusal leaves `self`
    /// untouched and a caller can refuse the transaction before it commits anything.
    ///
    /// # Errors
    ///
    /// The allocation failed.
    pub fn after(&self, next: &SessionModel) -> Result<ObservationChange, TryReserveError> {
        let mut kept = Vec::new();
        kept.try_reserve_exact(self.bindings.len())?;
        let mut ended = Vec::new();
        ended.try_reserve_exact(self.bindings.len())?;
        for binding in &self.bindings {
            if Self::has_target(next, &binding.key) {
                kept.push(binding.clone());
            } else {
                ended.push(binding.key.clone());
            }
        }
        Ok(ObservationChange {
            bindings: Self { bindings: kept },
            ended,
        })
    }
}
