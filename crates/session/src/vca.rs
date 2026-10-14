//! The VCA arithmetic and reach (#1242): which VCAs reach a strip, and the one sum they give its
//! fader.
//!
//! Decision 13 (a) (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`): a strip's effective
//! lane gain is `clamp(own_db + sum(offsets), -144, 24)` over the **set** of VCAs that reach it,
//! and its effective lane mute is its own mute ORed with every reaching VCA's mute. The composition
//! itself (the fader, the mute, the solo term and the route gate) is `controls::effective` (decision
//! 17, C7); this module keeps the arithmetic ([`vca_effective_db`]) and the reach
//! ([`SessionModel::vca_reach`]) that it calls, so there is one sum and one reach.

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
    let mut offsets = offsets_db.into_iter().peekable();
    if offsets.peek().is_none() {
        return member_db;
    }
    let mut sum = f64::from(member_db);
    for offset in offsets {
        sum += f64::from(offset);
    }
    // `f64::clamp` keeps a NaN, which a validated model never carries.
    sum.clamp(FADER_MINIMUM_DB, FADER_MAXIMUM_DB) as f32
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

    /// [`Self::vca_reach`] of the one strip named `strip`: the indices into `self.vcas` of every
    /// VCA from which it is reachable, each once, sorted by VCA ID (then index). Empty for an ID
    /// that names no strip or no VCA. It reads only the VCAs' member lists, never the strip
    /// table, so a host that classifies one edit does not build the table for every strip.
    #[must_use]
    pub fn vca_reach_of(&self, strip: &StableId) -> Vec<usize> {
        if self.vcas.is_empty() {
            return Vec::new();
        }
        // The VCAs that list `name` directly.
        let parents = |name: &StableId| -> Vec<usize> {
            self.vcas
                .iter()
                .enumerate()
                .filter(|(_, vca)| vca.members.contains(name))
                .map(|(index, _)| index)
                .collect()
        };
        let mut visited = vec![false; self.vcas.len()];
        let mut reach = Vec::new();
        let mut stack = parents(strip);
        while let Some(index) = stack.pop() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            reach.push(index);
            stack.extend(parents(&self.vcas[index].id));
        }
        // Stable on equal IDs, which only an unvalidated model carries.
        reach.sort_by(|left, right| {
            self.vcas[*left]
                .id
                .cmp(&self.vcas[*right].id)
                .then(left.cmp(right))
        });
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
        let mut visited = vec![false; self.vcas.len()];
        let mut stack = vec![vca];
        let mut strips = Vec::new();
        while let Some(index) = stack.pop() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            for member in &self.vcas[index].members {
                stack.extend(
                    self.vcas
                        .iter()
                        .enumerate()
                        .filter(|(_, nested)| nested.id == *member)
                        .map(|(nested, _)| nested),
                );
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
