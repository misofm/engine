//! Per-edit ramp lengths of one session transaction (#1394, decision 15 D15-1).
//!
//! A live-value edit may carry its own ramp length in samples. The length is **transaction
//! metadata, never model state**: it is not in [`crate::SessionModel`], not in canonical JSON, and
//! it lives only as long as the prepared transaction that recorded it. The next transaction that
//! writes the same row without a length uses the session's `control_smoothing` again.
//!
//! The types live here, beside the model, so that the protocol crate (which records them while it
//! applies a transaction's edits) and the host-core classifier (which reads them to pick each live
//! record's length) share them without a new dependency between the two.

use std::collections::BTreeMap;

use crate::{RackName, StableId};

/// The object a live row belongs to: the key, with [`EditRampRow`], of one recorded length.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditRampTarget {
    /// A strip: a track or a submix, named by its stable ID.
    Strip(StableId),
    /// One effect instance of a strip.
    Effect {
        /// The strip that owns the instance.
        strip: StableId,
        /// The rack that holds it.
        rack: RackName,
        /// The instance's stable ID.
        effect: StableId,
    },
    /// A route.
    Route(StableId),
    /// A VCA group.
    Vca(StableId),
    /// The session's console declaration (the slot defaults).
    Console,
}

/// One live row an edit writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditRampRow {
    /// A strip's fader dB.
    Fader,
    /// A strip's mute (the fader edit writes the mutes too).
    Mute,
    /// A strip's pan or raw matrix.
    Matrix,
    /// A strip's input trim and polarity.
    Input,
    /// A detector link mode: an effect's, a console slot's default or a strip's console entry.
    Link,
    /// A route's gain.
    RouteGain,
    /// A route's mute or its `follows_mute` toggle.
    RouteMute,
    /// A route's channel matrix.
    RouteMatrix,
    /// A VCA's fader offset.
    VcaFader,
    /// A VCA's mute.
    VcaMute,
}

impl EditRampRow {
    const fn code(self) -> u8 {
        self as u8
    }

    const fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Fader,
            1 => Self::Mute,
            2 => Self::Matrix,
            3 => Self::Input,
            4 => Self::Link,
            5 => Self::RouteGain,
            6 => Self::RouteMute,
            7 => Self::RouteMatrix,
            8 => Self::VcaFader,
            _ => Self::VcaMute,
        }
    }
}

/// The object a recorded length belongs to, without the effect part.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Owner {
    Strip(StableId),
    Route(StableId),
    Vca(StableId),
    Console,
}

/// One entry's key. Entries of one owner are adjacent, and within a strip its own rows (no
/// effect) come before its effect instances' rows, so a rewrite of one owner is one range.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Key {
    owner: Owner,
    /// `(RackName wire code, effect ID)` for an effect instance's row; `None` for the owner's own.
    effect: Option<(u8, StableId)>,
    row: u8,
}

impl Key {
    fn new(target: &EditRampTarget, row: EditRampRow) -> Self {
        let (owner, effect) = split(target);
        Self {
            owner,
            effect,
            row: row.code(),
        }
    }
}

fn split(target: &EditRampTarget) -> (Owner, Option<(u8, StableId)>) {
    match target {
        EditRampTarget::Strip(id) => (Owner::Strip(id.clone()), None),
        EditRampTarget::Effect {
            strip,
            rack,
            effect,
        } => (
            Owner::Strip(strip.clone()),
            Some((rack.wire(), effect.clone())),
        ),
        EditRampTarget::Route(id) => (Owner::Route(id.clone()), None),
        EditRampTarget::Vca(id) => (Owner::Vca(id.clone()), None),
        EditRampTarget::Console => (Owner::Console, None),
    }
}

/// The ramp length, in samples, that the edits of one transaction gave to the rows they wrote.
///
/// `(target, row)` has an entry only when the **last** edit of the transaction that wrote that row
/// carried a length. An explicit 0 is an entry: it steps that edit and is not "unset". Transaction
/// metadata, never model state: it is neither compared with nor merged into a session model, and
/// no canonical JSON shows it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EditRamps {
    entries: BTreeMap<Key, u32>,
}

impl EditRamps {
    /// No lengths: every row uses the session's `control_smoothing`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The length recorded for `row` of `target`, if the transaction gave one.
    #[must_use]
    pub fn get(&self, target: &EditRampTarget, row: EditRampRow) -> Option<u32> {
        self.entries.get(&Key::new(target, row)).copied()
    }

    /// Record the length the latest edit gave `row` of `target`: `Some(n)` sets the entry and
    /// `None` removes it (the later edit carried no length, so the session default applies).
    pub fn set(&mut self, target: &EditRampTarget, row: EditRampRow, samples: Option<u32>) {
        let key = Key::new(target, row);
        match samples {
            Some(samples) => {
                self.entries.insert(key, samples);
            }
            None => {
                self.entries.remove(&key);
            }
        }
    }

    /// Remove every entry of `target`. For a strip, its effect instances' entries go too: an
    /// upsert rewrites all of those rows with no length.
    pub fn clear_target(&mut self, target: &EditRampTarget) {
        let (owner, effect) = split(target);
        // Entries sort by owner, then effect, then row, so the target's are one run. A strip
        // takes every effect instance of its own with it; an effect target only its own rows.
        let only_effect = matches!(target, EditRampTarget::Effect { .. });
        let start = Key {
            owner: owner.clone(),
            effect: effect.clone(),
            row: 0,
        };
        let doomed: Vec<Key> = self
            .entries
            .range(start..)
            .map(|(key, _)| key)
            .take_while(|key| key.owner == owner && (!only_effect || key.effect == effect))
            .cloned()
            .collect();
        for key in doomed {
            self.entries.remove(&key);
        }
    }

    /// Every entry, ordered by target kind, then ID, then row.
    #[must_use]
    pub fn entries(&self) -> Vec<(EditRampTarget, EditRampRow, u32)> {
        self.entries
            .iter()
            .filter_map(|(key, samples)| {
                let target = match (&key.owner, &key.effect) {
                    (Owner::Strip(id), None) => EditRampTarget::Strip(id.clone()),
                    (Owner::Strip(strip), Some((rack, effect))) => EditRampTarget::Effect {
                        strip: strip.clone(),
                        // A recorded rack code always came from a `RackName`.
                        rack: RackName::from_wire(*rack)?,
                        effect: effect.clone(),
                    },
                    (Owner::Route(id), _) => EditRampTarget::Route(id.clone()),
                    (Owner::Vca(id), _) => EditRampTarget::Vca(id.clone()),
                    (Owner::Console, _) => EditRampTarget::Console,
                };
                Some((target, EditRampRow::from_code(key.row), *samples))
            })
            .collect()
    }

    /// Whether the transaction recorded no length at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("valid stable ID")
    }

    #[test]
    fn a_later_set_without_a_length_removes_the_entry_and_an_explicit_zero_is_an_entry() {
        let strip = EditRampTarget::Strip(id("a"));
        let mut ramps = EditRamps::new();
        ramps.set(&strip, EditRampRow::Fader, Some(0));
        assert_eq!(ramps.get(&strip, EditRampRow::Fader), Some(0));
        ramps.set(&strip, EditRampRow::Fader, Some(37));
        assert_eq!(ramps.get(&strip, EditRampRow::Fader), Some(37));
        ramps.set(&strip, EditRampRow::Fader, None);
        assert_eq!(ramps.get(&strip, EditRampRow::Fader), None);
        assert!(ramps.is_empty());
    }

    #[test]
    fn clearing_a_strip_drops_its_effects_and_nothing_else() {
        let effect = |strip: &str| EditRampTarget::Effect {
            strip: id(strip),
            rack: RackName::Inserts,
            effect: id("eq"),
        };
        let mut ramps = EditRamps::new();
        ramps.set(
            &EditRampTarget::Strip(id("a")),
            EditRampRow::Matrix,
            Some(5),
        );
        ramps.set(&effect("a"), EditRampRow::Link, Some(6));
        ramps.set(&effect("b"), EditRampRow::Link, Some(7));
        ramps.set(
            &EditRampTarget::Route(id("a")),
            EditRampRow::RouteGain,
            Some(8),
        );
        ramps.clear_target(&EditRampTarget::Strip(id("a")));
        assert_eq!(
            ramps.entries(),
            vec![
                (effect("b"), EditRampRow::Link, 7),
                (EditRampTarget::Route(id("a")), EditRampRow::RouteGain, 8),
            ]
        );
    }
}
