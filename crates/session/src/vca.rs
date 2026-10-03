//! The VCA composition (#1242): one place computes which VCAs reach a strip and the effective
//! fader and mute they give it.
//!
//! Decision 13 (a) (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`): a strip's effective
//! lane gain is `clamp(own_db + sum(offsets), -144, 24)` over the **set** of VCAs that reach it,
//! and its effective lane mute is its own mute ORed with every reaching VCA's mute. Both run at
//! preparation, off the render thread; every host and compiler calls them rather than spelling
//! the rule again.

use std::collections::BTreeMap;

use crate::model::SessionModel;

/// The lower edge of the fader domain, in dB.
const FADER_MINIMUM_DB: f64 = -144.0;
/// The upper edge of the fader domain, in dB.
const FADER_MAXIMUM_DB: f64 = 24.0;

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

/// One strip's fader after its VCAs: what preparation bakes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveStripFader {
    /// `[left, right]` effective gain in dB: [`vca_effective_db`] over the reaching VCAs' offsets.
    pub db: [f32; 2],
    /// `[left, right]` effective mute: the strip's own mute or its VCA mute.
    pub mute: [bool; 2],
    /// `[left, right]`: whether any reaching VCA mutes the lane.
    pub vca_mute: [bool; 2],
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

    /// Per strip, in [`Self::strips`] order: `db[l] = vca_effective_db(own_db[l], reach offsets[l]
    /// in ascending VCA-ID order)`, `vca_mute[l] = any(reach v.l_mute)`,
    /// `mute[l] = own_mute[l] || vca_mute[l]`.
    #[must_use]
    pub fn effective_strip_faders(&self) -> Vec<EffectiveStripFader> {
        self.strips()
            .zip(self.vca_reach())
            .map(|(strip, reach)| {
                let vcas = || reach.iter().map(|&index| &self.vcas[index].fader);
                let own = strip.fader;
                let vca_mute = [
                    vcas().any(|fader| fader.left_mute),
                    vcas().any(|fader| fader.right_mute),
                ];
                EffectiveStripFader {
                    db: [
                        vca_effective_db(own.left_db, vcas().map(|fader| fader.left_db)),
                        vca_effective_db(own.right_db, vcas().map(|fader| fader.right_db)),
                    ],
                    mute: [own.left_mute || vca_mute[0], own.right_mute || vca_mute[1]],
                    vca_mute,
                }
            })
            .collect()
    }
}
