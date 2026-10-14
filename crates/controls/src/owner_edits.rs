//! A transaction of `SetControl`s applied to clones of the owners it names (TBD10).
//!
//! [`OwnerEdits`] holds the committed model and a small model of the edited owners. Each
//! [`OwnerEdits::set`] clones the owner it names the first time, then runs the very [`crate::set`]
//! on it: the same lookups, checks, value normalization and write as on a whole model, which reads
//! only the owner and the session header. The result is the owners as the transaction leaves them,
//! and where each sits in the committed model, with no clone of the session.

use session::{OwnerSlots, SessionModel};

use crate::address::{ControlAddress, Owner};
use crate::error::ControlError;
use crate::view::position;

/// Why an edit is not one the owners model can hold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerEditError {
    /// The address names the session, not an owner of the model's tables.
    Session,
    /// The address names no owner the committed model holds, or [`crate::set`] refused the write.
    Control(ControlError),
}

/// The committed model's owners edited by a transaction.
#[derive(Debug)]
pub struct OwnerEdits<'m> {
    committed: &'m SessionModel,
    owners: SessionModel,
}

impl<'m> OwnerEdits<'m> {
    /// An edit of `committed` that has changed nothing yet. It clones the session header (the
    /// console slots, the profiles, the rate), which does not grow with the owners.
    #[must_use]
    pub fn new(committed: &'m SessionModel) -> Self {
        Self {
            committed,
            owners: SessionModel {
                schema_version: committed.schema_version,
                session_id: committed.session_id.clone(),
                revision: committed.revision,
                sample_rate_hz: committed.sample_rate_hz,
                quantum_frames: committed.quantum_frames,
                render_profile: committed.render_profile.clone(),
                output_profile: committed.output_profile.clone(),
                sources: Vec::new(),
                console: committed.console.clone(),
                tracks: Vec::new(),
                submixes: Vec::new(),
                vcas: Vec::new(),
                control_smoothing: committed.control_smoothing,
                outputs: Vec::new(),
                routes: Vec::new(),
                automation: Vec::new(),
            },
        }
    }

    /// The committed model the edits are made against.
    #[must_use]
    pub const fn committed(&self) -> &'m SessionModel {
        self.committed
    }

    /// The edited owners so far, as a model.
    #[must_use]
    pub const fn owners(&self) -> &SessionModel {
        &self.owners
    }

    /// [`crate::set`] on the owner `address` names, cloned from the committed model if this is its
    /// first edit.
    ///
    /// # Errors
    ///
    /// [`OwnerEditError::Session`] for a session row, [`OwnerEditError::Control`] for an owner the
    /// committed model does not hold and for any refusal of [`crate::set`]. Nothing is written then.
    pub fn set(
        &mut self,
        address: &ControlAddress,
        value: f32,
        ramp: Option<u32>,
        sample_rate: u32,
    ) -> Result<(), OwnerEditError> {
        self.take(&address.owner)?;
        crate::registry::set(&mut self.owners, address, value, ramp, sample_rate)
            .map_err(OwnerEditError::Control)
    }

    /// Clone the owner into the owners model, in ID order, unless it is there already.
    fn take(&mut self, owner: &Owner) -> Result<(), OwnerEditError> {
        let unknown = OwnerEditError::Control(ControlError::AddressUnknown);
        let committed = self.committed;
        match owner {
            Owner::Session => return Err(OwnerEditError::Session),
            Owner::Strip(id) => {
                if let Some(at) = position(&committed.tracks, id, |track| &track.id) {
                    insert_sorted(&mut self.owners.tracks, &committed.tracks[at], |track| {
                        &track.id
                    });
                } else if let Some(at) = position(&committed.submixes, id, |submix| &submix.id) {
                    insert_sorted(
                        &mut self.owners.submixes,
                        &committed.submixes[at],
                        |submix| &submix.id,
                    );
                } else {
                    return Err(unknown);
                }
            }
            Owner::Route(id) => {
                let at = position(&committed.routes, id, |route| &route.id).ok_or(unknown)?;
                insert_sorted(&mut self.owners.routes, &committed.routes[at], |route| {
                    &route.id
                });
            }
            Owner::Vca(id) => {
                let at = position(&committed.vcas, id, |vca| &vca.id).ok_or(unknown)?;
                insert_sorted(&mut self.owners.vcas, &committed.vcas[at], |vca| &vca.id);
            }
        }
        Ok(())
    }

    /// The edited owners and where each sits in the committed model.
    ///
    /// # Panics
    ///
    /// If an owner is no longer in the committed model (it cannot be: it was cloned from it).
    #[must_use]
    pub fn finish(self) -> (SessionModel, OwnerSlots) {
        let committed = self.committed;
        let mut slots = OwnerSlots::default();
        for (at, track) in self.owners.tracks.iter().enumerate() {
            let home = position(&committed.tracks, &track.id, |track| &track.id)
                .expect("an edited track came from the committed model");
            slots.tracks.push((at, home));
        }
        for (at, submix) in self.owners.submixes.iter().enumerate() {
            let home = position(&committed.submixes, &submix.id, |submix| &submix.id)
                .expect("an edited submix came from the committed model");
            slots.submixes.push((at, home));
        }
        for (at, vca) in self.owners.vcas.iter().enumerate() {
            let home = position(&committed.vcas, &vca.id, |vca| &vca.id)
                .expect("an edited VCA came from the committed model");
            slots.vcas.push((at, home));
        }
        for (at, route) in self.owners.routes.iter().enumerate() {
            let home = position(&committed.routes, &route.id, |route| &route.id)
                .expect("an edited route came from the committed model");
            slots.routes.push((at, home));
        }
        (self.owners, slots)
    }
}

/// Insert a clone of `item` into `items` (in ID order) unless an item with its ID is there.
fn insert_sorted<T: Clone>(items: &mut Vec<T>, item: &T, id_of: impl Fn(&T) -> &session::StableId) {
    if let Err(at) = items.binary_search_by(|held| id_of(held).cmp(id_of(item))) {
        items.insert(at, item.clone());
    }
}
