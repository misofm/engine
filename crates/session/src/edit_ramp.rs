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

use crate::StableId;

/// The object a live row belongs to: the key, with [`EditRampRow`], of one recorded length.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditRampTarget {
    /// A strip: a track or a submix, named by its stable ID.
    Strip(StableId),
    /// A route.
    Route(StableId),
    /// A VCA group.
    Vca(StableId),
    /// The session's console declaration (the slot defaults).
    Console,
}

/// One live row an edit writes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditRampRow {
    /// A strip's fader dB.
    Fader,
    /// A strip's mute (the fader edit writes the mutes too).
    Mute,
    /// A strip's pan or raw matrix.
    Matrix,
    /// A strip's input trim and polarity.
    Input,
    /// A detector link mode: a console slot's default or a strip's console entry.
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

/// The object a recorded length belongs to.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Owner {
    Strip(StableId),
    Route(StableId),
    Vca(StableId),
    Console,
}

/// One entry's key. Entries of one owner are adjacent, so a rewrite of one owner is one range.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Key {
    owner: Owner,
    row: EditRampRow,
}

impl Key {
    fn new(target: &EditRampTarget, row: EditRampRow) -> Self {
        Self {
            owner: owner_of(target),
            row,
        }
    }
}

fn owner_of(target: &EditRampTarget) -> Owner {
    match target {
        EditRampTarget::Strip(id) => Owner::Strip(id.clone()),
        EditRampTarget::Route(id) => Owner::Route(id.clone()),
        EditRampTarget::Vca(id) => Owner::Vca(id.clone()),
        EditRampTarget::Console => Owner::Console,
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
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
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

    /// Remove every entry of `target`: an upsert rewrites all of its rows with no length.
    pub fn clear_target(&mut self, target: &EditRampTarget) {
        let owner = owner_of(target);
        // Entries sort by owner, then row, so the target's are one run.
        let start = Key {
            owner: owner.clone(),
            row: EditRampRow::Fader,
        };
        let doomed: Vec<Key> = self
            .entries
            .range(start..)
            .map(|(key, _)| key)
            .take_while(|key| key.owner == owner)
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
            .map(|(key, samples)| {
                let target = match &key.owner {
                    Owner::Strip(id) => EditRampTarget::Strip(id.clone()),
                    Owner::Route(id) => EditRampTarget::Route(id.clone()),
                    Owner::Vca(id) => EditRampTarget::Vca(id.clone()),
                    Owner::Console => EditRampTarget::Console,
                };
                (target, key.row, *samples)
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
    fn clearing_a_target_drops_its_rows_and_nothing_else() {
        let strip = |name: &str| EditRampTarget::Strip(id(name));
        let mut ramps = EditRamps::new();
        ramps.set(&strip("a"), EditRampRow::Matrix, Some(5));
        ramps.set(&strip("a"), EditRampRow::Link, Some(6));
        ramps.set(&strip("b"), EditRampRow::Link, Some(7));
        ramps.set(
            &EditRampTarget::Route(id("a")),
            EditRampRow::RouteGain,
            Some(8),
        );
        ramps.clear_target(&strip("a"));
        assert_eq!(
            ramps.entries(),
            vec![
                (strip("b"), EditRampRow::Link, 7),
                (EditRampTarget::Route(id("a")), EditRampRow::RouteGain, 8),
            ]
        );
    }
}
