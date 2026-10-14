//! The model after a transaction of values, without building it (TBD10).
//!
//! A transaction of `SetControl`s changes the values of a few owners (a track, a submix, a route, a
//! VCA) and nothing else. [`ModelView`] is the committed model and those owners, edited, read as one
//! model: a strip, route or VCA is the edited one when the edit holds it, else the committed one.
//! The classifier, the catalog stage and the effective values read the "next" model through it, so a
//! commit of values clones, compiles and lists no more than the owners it changed.

use session::{Route, SessionModel, StableId, StripRef, Vca};

use crate::registry::strip_at;

/// The committed model (`structure`: counts, order, IDs, header, console slots) and the edited
/// owners (`owners`: a small model with the header and the edited clones in ID order), read as one.
/// For a model that is whole, both are the same model.
#[derive(Clone, Copy, Debug)]
pub struct ModelView<'a> {
    structure: &'a SessionModel,
    owners: &'a SessionModel,
}

impl<'a> ModelView<'a> {
    /// A model read whole.
    #[must_use]
    pub const fn whole(model: &'a SessionModel) -> Self {
        Self {
            structure: model,
            owners: model,
        }
    }

    /// The committed model, edited at the owners `owners` holds. `owners` is sorted by ID in each
    /// of its tables, and holds only owners that `structure` also holds.
    #[must_use]
    pub const fn edited(structure: &'a SessionModel, owners: &'a SessionModel) -> Self {
        Self { structure, owners }
    }

    /// The model the view reads its structure from: the counts, the order, the IDs, the header and
    /// the console slots. A value edit changes none of them.
    #[must_use]
    pub const fn structure(&self) -> &'a SessionModel {
        self.structure
    }

    /// The model that holds the edited owners (the same as [`Self::structure`] for a whole model).
    #[must_use]
    pub const fn owners(&self) -> &'a SessionModel {
        self.owners
    }

    /// Whether the view reads one model.
    #[must_use]
    pub fn is_whole(&self) -> bool {
        core::ptr::eq(self.structure, self.owners)
    }

    /// The strip at `index` of [`SessionModel::strips`] order, edited if the view holds an edit.
    ///
    /// # Panics
    ///
    /// If the model has no such strip.
    #[must_use]
    pub fn strip(&self, index: usize) -> StripRef<'a> {
        let base = strip_at(self.structure, index);
        if self.is_whole() {
            return base;
        }
        edited_strip(self.owners, base.id).unwrap_or(base)
    }

    /// The route at `index` of `routes`, edited if the view holds an edit.
    ///
    /// # Panics
    ///
    /// If the model has no such route.
    #[must_use]
    pub fn route(&self, index: usize) -> &'a Route {
        let base = &self.structure.routes[index];
        if self.is_whole() {
            return base;
        }
        find(&self.owners.routes, &base.id, |route| &route.id).unwrap_or(base)
    }

    /// The VCA at `index` of `vcas`, edited if the view holds an edit.
    ///
    /// # Panics
    ///
    /// If the model has no such VCA.
    #[must_use]
    pub fn vca(&self, index: usize) -> &'a Vca {
        let base = &self.structure.vcas[index];
        if self.is_whole() {
            return base;
        }
        find(&self.owners.vcas, &base.id, |vca| &vca.id).unwrap_or(base)
    }

    /// The dense model the view stands for: a clone of the committed model with the edited owners
    /// in place. Linear in the session; for tests and for a caller that has no view-shaped code.
    #[must_use]
    pub fn to_model(&self) -> SessionModel {
        let mut model = self.structure.clone();
        if self.is_whole() {
            return model;
        }
        for track in &self.owners.tracks {
            if let Some(at) = position(&model.tracks, &track.id, |track| &track.id) {
                model.tracks[at] = track.clone();
            }
        }
        for submix in &self.owners.submixes {
            if let Some(at) = position(&model.submixes, &submix.id, |submix| &submix.id) {
                model.submixes[at] = submix.clone();
            }
        }
        for vca in &self.owners.vcas {
            if let Some(at) = position(&model.vcas, &vca.id, |vca| &vca.id) {
                model.vcas[at] = vca.clone();
            }
        }
        for route in &self.owners.routes {
            if let Some(at) = position(&model.routes, &route.id, |route| &route.id) {
                model.routes[at] = route.clone();
            }
        }
        model
    }
}

/// The strip `id` of `model`, by binary search over the tracks and then the submixes (a compiled
/// session's normalized model keeps both in ID order).
fn edited_strip<'a>(model: &'a SessionModel, id: &StableId) -> Option<StripRef<'a>> {
    find(&model.tracks, id, |track| &track.id)
        .map(StripRef::track)
        .or_else(|| find(&model.submixes, id, |submix| &submix.id).map(StripRef::submix))
}

/// The item whose ID is `id` in `items`, which are in ID order.
pub(crate) fn find<'a, T>(
    items: &'a [T],
    id: &StableId,
    id_of: impl Fn(&T) -> &StableId,
) -> Option<&'a T> {
    position(items, id, id_of).map(|at| &items[at])
}

/// The position of the item whose ID is `id` in `items`: a binary search, then a scan (the same
/// answer on any slice, in `O(log n)` on one in ID order).
pub(crate) fn position<T>(
    items: &[T],
    id: &StableId,
    id_of: impl Fn(&T) -> &StableId,
) -> Option<usize> {
    // A short slice (an edited-owner list is usually one or two items) is scanned once.
    if items.len() <= 8 {
        return items.iter().position(|item| id_of(item) == id);
    }
    items
        .binary_search_by(|item| id_of(item).cmp(id))
        .ok()
        .or_else(|| items.iter().position(|item| id_of(item) == id))
}
