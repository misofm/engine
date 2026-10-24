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

use std::collections::{BTreeMap, BTreeSet};

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
///
/// It also remembers which rows the transaction wrote with no length and which targets it
/// rewrote whole, so [`Self::overlay`] can lay a later transaction over an earlier one with the
/// later winning on every row it wrote (#1310). Neither changes [`Self::get`]: such a row has no
/// length, and the session's `control_smoothing` applies.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EditRamps {
    entries: BTreeMap<Key, u32>,
    /// Rows the latest edit wrote with no length.
    unset: BTreeSet<Key>,
    /// Targets an upsert rewrote whole ([`Self::clear_target`]).
    cleared: BTreeSet<Owner>,
}

impl EditRamps {
    /// No lengths: every row uses the session's `control_smoothing`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            unset: BTreeSet::new(),
            cleared: BTreeSet::new(),
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
                self.unset.remove(&key);
                self.entries.insert(key, samples);
            }
            None => {
                self.entries.remove(&key);
                self.unset.insert(key);
            }
        }
    }

    /// Remove every entry of `target`: an upsert rewrites all of its rows with no length.
    pub fn clear_target(&mut self, target: &EditRampTarget) {
        let owner = owner_of(target);
        self.clear_owner(&owner);
        self.cleared.insert(owner);
    }

    /// Remove every entry and every unset row of `owner`.
    fn clear_owner(&mut self, owner: &Owner) {
        // Entries sort by owner, then row, so the target's are one run.
        let start = Key {
            owner: owner.clone(),
            row: EditRampRow::Fader,
        };
        let doomed: Vec<Key> = self
            .entries
            .range(start..)
            .map(|(key, _)| key)
            .take_while(|key| key.owner == *owner)
            .cloned()
            .collect();
        for key in doomed {
            self.entries.remove(&key);
        }
        self.unset.retain(|key| key.owner != *owner);
    }

    /// Lay `later`, a later transaction's lengths, over these (#1310): `later` wins on every row it
    /// wrote, with a length or without one, and on every row of a target it rewrote whole; every
    /// other row keeps its length here. The result is what one transaction holding both sets of
    /// edits, in order, records.
    pub fn overlay(&mut self, later: &Self) {
        for owner in &later.cleared {
            self.clear_owner(owner);
            self.cleared.insert(owner.clone());
        }
        for (key, samples) in &later.entries {
            self.unset.remove(key);
            self.entries.insert(key.clone(), *samples);
        }
        for key in &later.unset {
            self.entries.remove(key);
            self.unset.insert(key.clone());
        }
    }

    /// Whether the transaction wrote no live row at all: no length, no row without one, and no
    /// target rewritten whole. [`Self::overlay`] of such a transaction changes nothing.
    #[must_use]
    pub fn is_untouched(&self) -> bool {
        self.entries.is_empty() && self.unset.is_empty() && self.cleared.is_empty()
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

    /// The bytes a ramp table can retain on a session of `strips` strips, `routes` routes and
    /// `vcas` VCA groups whose IDs total `strip_id_bytes`, `route_id_bytes` and `vca_id_bytes`
    /// (#1696 D1, D2): a ceiling of the session's shape, never of its edits, or `None` on overflow.
    ///
    /// The bound: a key is `(owner, row)` over the session's owners -- five rows per strip, three
    /// per route, two per VCA and the console's one -- and [`Self::set`] keeps a key in exactly one
    /// of `entries` and `unset`, so the two hold at most that many keys together; `cleared` holds
    /// at most one owner per strip, route, VCA and the console. Each key and each cleared owner
    /// owns one copy of its owner's ID text. A B-tree node other than the root holds at least five
    /// keys (std's node: B = 6, eleven keys, twelve edges), so `n` keys sit in at most
    /// `1 + (n - 1) / 5` nodes, each charged at the internal node's layout.
    #[must_use]
    pub fn retained_ceiling(
        strips: usize,
        routes: usize,
        vcas: usize,
        strip_id_bytes: usize,
        route_id_bytes: usize,
        vca_id_bytes: usize,
    ) -> Option<usize> {
        let keys = strips
            .checked_mul(5)?
            .checked_add(routes.checked_mul(3)?)?
            .checked_add(vcas.checked_mul(2)?)?
            .checked_add(1)?;
        let owners = strips
            .checked_add(routes)?
            .checked_add(vcas)?
            .checked_add(1)?;
        let key_text = strip_id_bytes
            .checked_mul(5)?
            .checked_add(route_id_bytes.checked_mul(3)?)?
            .checked_add(vca_id_bytes.checked_mul(2)?)?;
        let owner_text = strip_id_bytes
            .checked_add(route_id_bytes)?
            .checked_add(vca_id_bytes)?;
        // `entries` and `unset` share the keys: each is one root plus a node per five more keys,
        // so together at most two roots and `keys / 5` nodes, at the map's (larger) node.
        let key_nodes = 2 + keys / B_TREE_MIN_KEYS;
        let owner_nodes = 1 + (owners - 1) / B_TREE_MIN_KEYS;
        key_nodes
            .checked_mul(b_tree_node_ceiling::<Key, u32>())?
            .checked_add(key_text)?
            .checked_add(owner_nodes.checked_mul(b_tree_node_ceiling::<Owner, ()>())?)?
            .checked_add(owner_text)
    }

    /// Whether the transaction recorded no length at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// The fewest keys a B-tree node other than the root holds: std's `B - 1` for `B = 6`.
const B_TREE_MIN_KEYS: usize = 5;
/// The keys and values a std B-tree node holds.
const B_TREE_CAPACITY: usize = 11;

const fn align_up(bytes: usize, align: usize) -> usize {
    bytes.div_ceil(align) * align
}

/// One std B-tree internal node of `K` keys and `V` values, a ceiling of every node of the tree:
/// the `#[repr(C)]` leaf (the parent pointer, the parent index and the length, then eleven keys and
/// eleven values) and the twelve child edges after it.
const fn b_tree_node_ceiling<K, V>() -> usize {
    let word = size_of::<usize>();
    let header = word + 2 * size_of::<u16>();
    let keys = align_up(header, align_of::<K>()) + B_TREE_CAPACITY * size_of::<K>();
    let values = align_up(keys, align_of::<V>()) + B_TREE_CAPACITY * size_of::<V>();
    align_up(values, word) + (B_TREE_CAPACITY + 1) * word
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("valid stable ID")
    }

    /// #1310: the later transaction wins on every row it wrote, a row it wrote with no length
    /// included, and on every row of a target it rewrote; the earlier lengths of other rows stay.
    /// Red if the overlay keeps an earlier length on a row the later one wrote without one.
    #[test]
    fn an_overlay_lets_the_later_transaction_win_on_every_row_it_wrote() {
        let strip = |name: &str| EditRampTarget::Strip(id(name));
        let mut earlier = EditRamps::new();
        earlier.set(&strip("a"), EditRampRow::Fader, Some(5));
        earlier.set(&strip("a"), EditRampRow::Matrix, Some(6));
        earlier.set(&strip("b"), EditRampRow::Fader, Some(7));
        earlier.set(&strip("c"), EditRampRow::Fader, Some(8));
        let mut later = EditRamps::new();
        later.set(&strip("a"), EditRampRow::Fader, None);
        later.set(&strip("b"), EditRampRow::Fader, Some(9));
        later.clear_target(&strip("c"));
        assert!(!later.is_untouched());
        earlier.overlay(&later);
        assert_eq!(earlier.get(&strip("a"), EditRampRow::Fader), None);
        assert_eq!(
            earlier.entries(),
            vec![
                (strip("a"), EditRampRow::Matrix, 6),
                (strip("b"), EditRampRow::Fader, 9),
            ]
        );
        assert!(EditRamps::new().is_untouched());
    }

    /// #1696 gate 1: a ramp table filled to every row of every owner -- with lengths, then half
    /// with no length, then every owner rewritten whole and filled again through an overlay --
    /// never retains more than [`EditRamps::retained_ceiling`] of the session's shape. Red when a
    /// B-tree node layout or an ID clone the ceiling does not account for lets a full table
    /// out-grow the row the control cap admits.
    #[test]
    fn a_full_ramp_table_stays_within_its_ceiling() {
        use bench_support::alloc::{
            assert_installed, current_thread_peak, current_thread_peak_reset,
        };
        const STRIP: [EditRampRow; 5] = [
            EditRampRow::Fader,
            EditRampRow::Mute,
            EditRampRow::Matrix,
            EditRampRow::Input,
            EditRampRow::Link,
        ];
        const ROUTE: [EditRampRow; 3] = [
            EditRampRow::RouteGain,
            EditRampRow::RouteMute,
            EditRampRow::RouteMatrix,
        ];
        const VCA: [EditRampRow; 2] = [EditRampRow::VcaFader, EditRampRow::VcaMute];
        assert_installed();
        let named = |prefix: &str, count: usize, width: usize| -> Vec<StableId> {
            (0..count)
                .map(|index| id(&format!("{prefix}-{index:0width$}")))
                .collect()
        };
        for (strips, routes, vcas, width) in [(1, 0, 0, 1), (3, 2, 1, 4), (40, 30, 7, 60)] {
            let strip_ids = named("strip", strips, width);
            let route_ids = named("route", routes, width);
            let vca_ids = named("vca", vcas, width);
            let text = |ids: &[StableId]| ids.iter().map(|id| id.as_str().len()).sum::<usize>();
            let ceiling = EditRamps::retained_ceiling(
                strips,
                routes,
                vcas,
                text(&strip_ids),
                text(&route_ids),
                text(&vca_ids),
            )
            .expect("the ceiling fits");
            let mut rows: Vec<(EditRampTarget, EditRampRow)> = Vec::new();
            let targets = strip_ids
                .iter()
                .map(|id| (EditRampTarget::Strip(id.clone()), &STRIP[..]))
                .chain(
                    route_ids
                        .iter()
                        .map(|id| (EditRampTarget::Route(id.clone()), &ROUTE[..])),
                )
                .chain(
                    vca_ids
                        .iter()
                        .map(|id| (EditRampTarget::Vca(id.clone()), &VCA[..])),
                )
                .chain([(EditRampTarget::Console, &[EditRampRow::Link][..])]);
            for (target, owned) in targets {
                rows.extend(owned.iter().map(|row| (target.clone(), *row)));
            }

            current_thread_peak_reset();
            let mut ramps = EditRamps::new();
            for (target, row) in &rows {
                ramps.set(target, *row, Some(7));
            }
            let full = current_thread_peak().live_bytes;
            for (target, row) in rows.iter().step_by(2) {
                ramps.set(target, *row, None);
            }
            let split = current_thread_peak().live_bytes;
            let mut later = EditRamps::new();
            for (target, _) in &rows {
                later.clear_target(target);
            }
            for (index, (target, row)) in rows.iter().enumerate() {
                later.set(target, *row, (index % 3 != 0).then_some(9));
            }
            ramps.overlay(&later);
            drop(later);
            let rewritten = current_thread_peak().live_bytes;
            for (phase, live) in [("full", full), ("split", split), ("rewritten", rewritten)] {
                assert!(
                    live <= ceiling as u64,
                    "{strips} strips, {routes} routes, {vcas} VCAs, {phase}: {live} bytes live, \
                     above the ceiling {ceiling}"
                );
            }
            drop(ramps);
        }
    }
}
