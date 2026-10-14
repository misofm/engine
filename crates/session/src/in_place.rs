//! The pieces of a value-only commit that the session owns (TBD10): which owners an edit
//! touched, whether their values are valid, whether their counts are those of the owners they
//! replace, and how many bytes they take in the canonical text.
//!
//! A host that changes the value of a control writes it into a **clone of the owner** (a track, a
//! submix, a route or a VCA), checks the clone with [`owners_values_valid`] and
//! [`owners_counts_equal`], measures the change of the canonical text with
//! [`canonical_owner_bytes`], and installs the clones with [`CompiledSession::swap_owners`]. The
//! rest of the compiled session is the same value, so nothing else is cloned, validated or written.

use crate::canonical::owner_bytes;
use crate::{CompiledSession, Route, SessionModel, Submix, Track, Vca};

/// One owner of a session, borrowed.
#[derive(Clone, Copy, Debug)]
pub enum OwnerRef<'a> {
    /// A track.
    Track(&'a Track),
    /// A submix.
    Submix(&'a Submix),
    /// A VCA group.
    Vca(&'a Vca),
    /// A route.
    Route(&'a Route),
}

/// Where the edited owners sit: for each, its index in the **owners** model (a small model that
/// holds the edited clones) and in the **committed** model, per kind.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OwnerSlots {
    /// `(owners index, committed index)` of each edited track.
    pub tracks: Vec<(usize, usize)>,
    /// The same for submixes.
    pub submixes: Vec<(usize, usize)>,
    /// The same for VCAs.
    pub vcas: Vec<(usize, usize)>,
    /// The same for routes.
    pub routes: Vec<(usize, usize)>,
}

impl OwnerSlots {
    /// The edited owners as held by `owners`, tracks, submixes, VCAs, then routes.
    ///
    /// # Panics
    ///
    /// If a slot is out of range of `owners`.
    pub fn in_owners<'a>(&'a self, owners: &'a SessionModel) -> impl Iterator<Item = OwnerRef<'a>> {
        self.refs(owners, |&(at, _)| at)
    }

    /// The edited owners as the committed model holds them, in the same order.
    ///
    /// # Panics
    ///
    /// If a slot is out of range of `committed`.
    pub fn in_committed<'a>(
        &'a self,
        committed: &'a SessionModel,
    ) -> impl Iterator<Item = OwnerRef<'a>> {
        self.refs(committed, |&(_, at)| at)
    }

    fn refs<'a>(
        &'a self,
        model: &'a SessionModel,
        at: fn(&(usize, usize)) -> usize,
    ) -> impl Iterator<Item = OwnerRef<'a>> {
        self.tracks
            .iter()
            .map(move |slot| OwnerRef::Track(&model.tracks[at(slot)]))
            .chain(
                self.submixes
                    .iter()
                    .map(move |slot| OwnerRef::Submix(&model.submixes[at(slot)])),
            )
            .chain(
                self.vcas
                    .iter()
                    .map(move |slot| OwnerRef::Vca(&model.vcas[at(slot)])),
            )
            .chain(
                self.routes
                    .iter()
                    .map(move |slot| OwnerRef::Route(&model.routes[at(slot)])),
            )
    }

    /// Whether no owner is edited.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
            && self.submixes.is_empty()
            && self.vcas.is_empty()
            && self.routes.is_empty()
    }
}

/// The bytes the owners take in the canonical text of a session.
#[must_use]
pub fn canonical_owner_bytes<'a>(owners: impl IntoIterator<Item = OwnerRef<'a>>) -> usize {
    owners
        .into_iter()
        .map(|owner| match owner {
            OwnerRef::Track(track) => owner_bytes(track),
            OwnerRef::Submix(submix) => owner_bytes(submix),
            OwnerRef::Vca(vca) => owner_bytes(vca),
            OwnerRef::Route(route) => owner_bytes(route),
        })
        .sum()
}

/// Whether the numeric values of every owner pass the session's validation: the rules
/// `compile_session` applies to a track's, submix's, VCA's or route's values, run on the owner alone.
/// The references between entities are not read; an edit of values cannot change them.
#[must_use]
pub fn owners_values_valid<'a>(owners: impl IntoIterator<Item = OwnerRef<'a>>) -> bool {
    owners.into_iter().all(crate::validate::owner_values_valid)
}

/// Whether the owners of `old` and `new`, pairwise, hold the same number of everything the resource
/// estimate counts: console entries, inserts, parameter entries per effect, VCA members. An edit that
/// adds or removes one is not a change of values.
#[must_use]
pub fn owners_counts_equal<'a>(
    old: impl IntoIterator<Item = OwnerRef<'a>>,
    new: impl IntoIterator<Item = OwnerRef<'a>>,
) -> bool {
    let (mut old, mut new) = (old.into_iter(), new.into_iter());
    loop {
        match (old.next(), new.next()) {
            (None, None) => return true,
            (Some(old), Some(new)) if counts_equal(old, new) => {}
            _ => return false,
        }
    }
}

fn counts_equal(old: OwnerRef<'_>, new: OwnerRef<'_>) -> bool {
    let strip = |old: crate::StripRef<'_>, new: crate::StripRef<'_>| {
        old.console.len() == new.console.len()
            && old
                .console
                .iter()
                .zip(new.console)
                .all(|(old, new)| old.params.len() == new.params.len())
            && old.inserts.effects.len() == new.inserts.effects.len()
            && old
                .inserts
                .effects
                .iter()
                .zip(&new.inserts.effects)
                .all(|(old, new)| old.params.len() == new.params.len())
    };
    match (old, new) {
        (OwnerRef::Track(old), OwnerRef::Track(new)) => {
            strip(crate::StripRef::track(old), crate::StripRef::track(new))
        }
        (OwnerRef::Submix(old), OwnerRef::Submix(new)) => {
            strip(crate::StripRef::submix(old), crate::StripRef::submix(new))
        }
        (OwnerRef::Vca(old), OwnerRef::Vca(new)) => old.members.len() == new.members.len(),
        (OwnerRef::Route(_), OwnerRef::Route(_)) => true,
        _ => false,
    }
}

impl CompiledSession {
    /// Exchange each edited owner of the compiled model with the one in `owners` at its slot. The
    /// edited clones go in and the owners they replace come out in `owners`, so a second call with
    /// the same slots undoes the first. Allocation-free and infallible for slots that are in range,
    /// which a caller that read them from this model at its current revision knows.
    ///
    /// This does not touch the revision, the canonical text or the estimate: call
    /// [`Self::finish_in_place`] after, once.
    ///
    /// # Panics
    ///
    /// If a slot is out of range of either model.
    pub fn swap_owners(&mut self, owners: &mut SessionModel, slots: &OwnerSlots) {
        let model = &mut self.normalized.0;
        // A slot pair that names two different owners would put one owner's values in another's
        // place; the slots come from the same IDs, so a debug build checks it.
        for &(at, committed) in &slots.tracks {
            debug_assert_eq!(model.tracks[committed].id, owners.tracks[at].id);
            core::mem::swap(&mut model.tracks[committed], &mut owners.tracks[at]);
        }
        for &(at, committed) in &slots.submixes {
            debug_assert_eq!(model.submixes[committed].id, owners.submixes[at].id);
            core::mem::swap(&mut model.submixes[committed], &mut owners.submixes[at]);
        }
        for &(at, committed) in &slots.vcas {
            debug_assert_eq!(model.vcas[committed].id, owners.vcas[at].id);
            core::mem::swap(&mut model.vcas[committed], &mut owners.vcas[at]);
        }
        for &(at, committed) in &slots.routes {
            debug_assert_eq!(model.routes[committed].id, owners.routes[at].id);
            core::mem::swap(&mut model.routes[committed], &mut owners.routes[at]);
        }
    }

    /// Finish a commit of values: record the revision the owners were committed at and the resource
    /// estimate of the model that now holds them, and drop the canonical text, which is written again
    /// from the model on its next read.
    pub fn finish_in_place(&mut self, revision: u64, estimate: crate::ResourceEstimate) {
        self.normalized.0.revision = revision;
        self.resource_estimate = estimate;
        self.canonical_json = std::sync::OnceLock::new();
    }
}
