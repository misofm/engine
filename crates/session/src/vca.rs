//! The VCA arithmetic and reach (#1242): which VCAs reach a strip, and the one sum they give its
//! fader.
//!
//! Decision 13 (a) (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`): a strip's effective
//! lane gain is `clamp(own_db + sum(offsets), -144, 24)` over the **set** of VCAs that reach it,
//! and its effective lane mute is its own mute ORed with every reaching VCA's mute. The composition
//! itself (the fader, the mute, the solo term and the route gate) is `controls::effective` (decision
//! 17, C7); this module keeps the arithmetic ([`vca_effective_db`]) and the reach
//! ([`SessionModel::vca_reach`]) that it calls, so there is one sum and one reach.

use std::cell::OnceCell;
use std::collections::BTreeMap;

use crate::StableId;
use crate::model::SessionModel;

/// The lower edge of the fader domain, in dB.
pub const FADER_MINIMUM_DB: f64 = -144.0;
/// The upper edge of the fader domain, in dB.
pub const FADER_MAXIMUM_DB: f64 = 24.0;

/// Decision 13 (a): `clamp(member_db + sum(offsets), -144, 24)`, summed in `f64` in the order
/// given (the member's own value first, then each offset), clamped in `f64`, rounded once to
/// `f32`. With no offset it returns `member_db` bit for bit, unclamped.
#[must_use]
pub fn vca_effective_db(member_db: f32, offsets_db: impl IntoIterator<Item = f32>) -> f32 {
    let mut sum = VcaSum::new(member_db);
    for offset in offsets_db {
        sum.add(offset);
    }
    sum.finish()
}

/// [`vca_effective_db`] as an accumulator, for a caller that reads each reaching VCA once and
/// feeds both lanes (batch 7 review M1): `add` the offsets in order, then `finish`. The one sum.
#[derive(Clone, Copy, Debug)]
pub struct VcaSum {
    member_db: f32,
    sum: f64,
    any: bool,
}

impl VcaSum {
    /// A sum that starts at the member's own value.
    #[must_use]
    pub fn new(member_db: f32) -> Self {
        Self {
            member_db,
            sum: f64::from(member_db),
            any: false,
        }
    }

    /// Adds the next reaching VCA's offset.
    pub fn add(&mut self, offset_db: f32) {
        self.sum += f64::from(offset_db);
        self.any = true;
    }

    /// The clamped, once-rounded result; the member's own value, bit for bit, when no offset came.
    #[must_use]
    pub fn finish(self) -> f32 {
        if !self.any {
            return self.member_db;
        }
        // `f64::clamp` keeps a NaN, which a validated model never carries.
        self.sum.clamp(FADER_MINIMUM_DB, FADER_MAXIMUM_DB) as f32
    }
}

impl SessionModel {
    /// Per strip, in [`Self::strips`] order: the indices into `self.vcas` of every VCA from which
    /// the strip is reachable through membership (directly or through nested VCAs), each once,
    /// sorted by VCA ID. Terminates on any model (a visited set), cyclic or not.
    #[must_use]
    pub fn vca_reach(&self) -> Vec<Vec<usize>> {
        if self.vcas.is_empty() {
            return self.strips().map(|_| Vec::new()).collect();
        }
        // Member ID -> the VCAs that list it directly, in declaration order.
        let mut parents: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (index, vca) in self.vcas.iter().enumerate() {
            for member in &vca.members {
                parents.entry(member.as_str()).or_default().push(index);
            }
        }
        let mut visited = vec![false; self.vcas.len()];
        let mut stack: Vec<usize> = Vec::new();
        self.strips()
            .map(|strip| {
                visited.fill(false);
                let mut reach = Vec::new();
                stack.clear();
                stack.extend(parents.get(strip.id.as_str()).into_iter().flatten());
                while let Some(index) = stack.pop() {
                    if visited[index] {
                        continue;
                    }
                    visited[index] = true;
                    reach.push(index);
                    stack.extend(
                        parents
                            .get(self.vcas[index].id.as_str())
                            .into_iter()
                            .flatten(),
                    );
                }
                // Stable on equal IDs, which only an unvalidated model carries.
                reach.sort_by(|left, right| {
                    self.vcas[*left]
                        .id
                        .cmp(&self.vcas[*right].id)
                        .then(left.cmp(right))
                });
                reach
            })
            .collect()
    }

    /// The number of (strip, VCA) reach pairs [`Self::vca_reach`] holds, counted without building
    /// the table and without sorting, and stopped as soon as the count passes `limit`: the result
    /// is the exact count when it is at most `limit`, and some value above `limit` otherwise.
    ///
    /// A count of VCAs does not bound a VCA move's work: where one VCA holds every strip and each
    /// later VCA holds the one before, every strip reaches every VCA (ruling V-Q3). The walk is
    /// [`Self::vca_reach`]'s, so the count and the table cannot disagree.
    #[must_use]
    pub fn vca_reach_pairs(&self, limit: u64) -> u64 {
        if self.vcas.is_empty() {
            return 0;
        }
        let mut parents: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (index, vca) in self.vcas.iter().enumerate() {
            for member in &vca.members {
                parents.entry(member.as_str()).or_default().push(index);
            }
        }
        let mut visited = vec![false; self.vcas.len()];
        let mut stack: Vec<usize> = Vec::new();
        let mut total = 0_u64;
        for strip in self.strips() {
            visited.fill(false);
            stack.clear();
            stack.extend(parents.get(strip.id.as_str()).into_iter().flatten());
            while let Some(index) = stack.pop() {
                if visited[index] {
                    continue;
                }
                visited[index] = true;
                total = total.saturating_add(1);
                if total > limit {
                    return total;
                }
                stack.extend(
                    parents
                        .get(self.vcas[index].id.as_str())
                        .into_iter()
                        .flatten(),
                );
            }
        }
        total
    }

    /// [`Self::vca_reach`] of the one strip named `strip`: the indices into `self.vcas` of every
    /// VCA from which it is reachable, each once, sorted by VCA ID (then index). Empty for an ID
    /// that names no strip or no VCA. It reads only the VCAs' member lists, never the strip
    /// table, so a host that classifies one edit does not build the table for every strip.
    ///
    /// A caller that asks for many strips builds the [`VcaNesting`] once and uses
    /// [`Self::vca_reach_of_in`].
    #[must_use]
    pub fn vca_reach_of(&self, strip: &StableId) -> Vec<usize> {
        if self.vcas.is_empty() {
            return Vec::new();
        }
        self.vca_reach_of_in(&OnceCell::new(), strip)
    }

    /// Which VCAs list each VCA, for [`Self::vca_reach_of_in`]. Cost grows with the VCAs and their
    /// members, not with the strips.
    fn vca_nesting(&self) -> VcaNesting {
        let mut nested: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (index, vca) in self.vcas.iter().enumerate() {
            nested.entry(vca.id.as_str()).or_default().push(index);
        }
        let mut parents = vec![Vec::new(); self.vcas.len()];
        for (parent, vca) in self.vcas.iter().enumerate() {
            for member in &vca.members {
                for &child in nested.get(member.as_str()).into_iter().flatten() {
                    parents[child].push(parent);
                }
            }
        }
        // The position of each VCA in (ID, index) order, so that a reach sorts by integers.
        let mut order: Vec<usize> = (0..self.vcas.len()).collect();
        order.sort_by(|&left, &right| {
            self.vcas[left]
                .id
                .cmp(&self.vcas[right].id)
                .then(left.cmp(&right))
        });
        let mut rank = vec![0; self.vcas.len()];
        for (position, index) in order.into_iter().enumerate() {
            rank[index] = position;
        }
        VcaNesting { parents, rank }
    }

    /// [`Self::vca_reach_of`] for a caller that asks about many strips of this same model: the
    /// nesting of the VCAs is built into `nesting` the first time a strip is held by any VCA, and
    /// reused after, so the walk never searches the VCAs for a parent (a search per visited VCA is
    /// quadratic in the VCA count). A strip held by no VCA builds nothing.
    #[must_use]
    pub fn vca_reach_of_in(&self, nesting: &OnceCell<VcaNesting>, strip: &StableId) -> Vec<usize> {
        let mut visited = vec![false; self.vcas.len()];
        let mut reach = Vec::new();
        let mut stack: Vec<usize> = self
            .vcas
            .iter()
            .enumerate()
            .filter(|(_, vca)| vca.members.contains(strip))
            .map(|(index, _)| index)
            .collect();
        if stack.is_empty() {
            return reach;
        }
        let nesting = nesting.get_or_init(|| self.vca_nesting());
        debug_assert_eq!(nesting.parents.len(), self.vcas.len());
        while let Some(index) = stack.pop() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            reach.push(index);
            stack.extend(&nesting.parents[index]);
        }
        // By VCA ID, then index (the rank), which only an unvalidated model makes differ.
        reach.sort_unstable_by_key(|&index| nesting.rank[index]);
        reach
    }

    /// The strips VCA number `vca` reaches, through membership directly or through nested VCAs, as
    /// indices into [`Self::strips`], ascending and each once: the inverse of
    /// [`Self::vca_reach`] (strip `s` is in the result exactly when `vca` is in `s`'s reach).
    /// Terminates on any model. Empty for an index past `self.vcas`.
    ///
    /// A member is found by binary search in the tracks and then in the submixes, so the model
    /// must list both in ID order, as a compiled session's normalized model does.
    #[must_use]
    pub fn strips_reached_by(&self, vca: usize) -> Vec<usize> {
        if vca >= self.vcas.len() {
            return Vec::new();
        }
        let mut by_id: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (index, nested) in self.vcas.iter().enumerate() {
            by_id.entry(nested.id.as_str()).or_default().push(index);
        }
        let mut visited = vec![false; self.vcas.len()];
        let mut stack = vec![vca];
        let mut strips = Vec::new();
        while let Some(index) = stack.pop() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            for member in &self.vcas[index].members {
                if let Some(children) = by_id.get(member.as_str()) {
                    stack.extend(children);
                }
                if let Ok(at) = self.tracks.binary_search_by(|track| track.id.cmp(member)) {
                    strips.push(at);
                } else if let Ok(at) = self
                    .submixes
                    .binary_search_by(|submix| submix.id.cmp(member))
                {
                    strips.push(self.tracks.len() + at);
                }
            }
        }
        strips.sort_unstable();
        strips.dedup();
        strips
    }
}

/// Per VCA, the VCAs that list it as a member, in index order (built by
/// [`SessionModel::vca_reach_of_in`]): the upward edges of the VCA tree, so that the reach of many
/// strips walks each edge once per strip and never searches the VCAs for a parent.
#[derive(Clone, Debug, Default)]
pub struct VcaNesting {
    parents: Vec<Vec<usize>>,
    /// Each VCA's position in (ID, index) order.
    rank: Vec<usize>,
}
