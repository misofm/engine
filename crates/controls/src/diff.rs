//! The row diff (decision 17, C6 and #1554 D2): which controls a transaction changes.

use session::SessionModel;

use crate::automation::AutomationIndex;
use crate::descriptor::{ControlDescriptor, ControlScope};
use crate::effective::MonitorOverlay;
use crate::effective::{Effective, effective};
use crate::lane::Side;
use crate::registry::{ControlRow, Source, owner_count, owner_rows_in, resolve_in};
use crate::view::ModelView;
use crate::{ControlClass, strip};
use session::{ControlAddress, Owner, Section};

/// One control whose committed value differs between two models.
#[derive(Clone, Debug, PartialEq)]
pub struct ChangedControl {
    /// The control's address (a per-lane control is one change per changed lane).
    pub address: ControlAddress,
    /// The control's descriptor in the later model.
    pub descriptor: ControlDescriptor,
}

/// The controls whose value bits differ between `current` and `next`, in list order, or `None`
/// when the two models do not list the same addresses in the same order (their structure
/// differs).
///
/// A VCA's rows are the inputs of the effective values, and a VCA added or removed changes the row
/// set without changing the structure (decision 17, C6, #1558 D5): the VCA rows are compared apart
/// from the others. When both models list the same VCA addresses, their changed rows are in the
/// result after every other row's; when the lists differ, the result has no VCA row, and a caller
/// that needs to know compares the effective values (`effective`).
///
/// A derived row is skipped: its value follows other controls, which are in the result themselves.
/// A row that a strip's shape does not use reports its default in both models, so it differs only
/// through its active flag, which is not a value. The host-local monitor values (solo) are not in
/// a session, so they are the same in both.
///
/// Allocation: one owner's rows of each model at a time, their VCA rows and the result, on the
/// control thread (issue #2232: before, both whole lists were held at once, about 190 KB per
/// console strip).
#[must_use]
pub fn changed_controls(
    current: &SessionModel,
    next: &SessionModel,
    sample_rate: u32,
) -> Option<Vec<ChangedControl>> {
    let overlay = MonitorOverlay::default();
    let (before_effective, after_effective) =
        (effective(current, &overlay), effective(next, &overlay));
    let (before_automation, after_automation) =
        (AutomationIndex::of(current), AutomationIndex::of(next));
    let mut before = OwnerRows::new(
        ModelView::whole(current).indexed(&before_automation),
        &before_effective,
        &overlay,
        sample_rate,
    );
    let mut after = OwnerRows::new(
        ModelView::whole(next).indexed(&after_automation),
        &after_effective,
        &overlay,
        sample_rate,
    );
    let mut changed = Vec::new();
    loop {
        match (before.next(), after.next()) {
            (Some(old), Some(new)) => note(&mut changed, old, new)?,
            (None, None) => break,
            // The two lists hold a different number of rows: their structure differs.
            _ => return None,
        }
    }
    // The VCA rows are the same addresses, or no VCA row is compared.
    if before.vca.len() == after.vca.len()
        && before
            .vca
            .iter()
            .zip(&after.vca)
            .all(|(old, new)| old.address == new.address)
    {
        for (old, new) in before.vca.into_iter().zip(after.vca) {
            note(&mut changed, old, new)?;
        }
    }
    Some(changed)
}

/// One pair of rows at the same position of the two lists: `None` when their addresses differ,
/// else `new` joins `changed` when it is not derived and its value bits differ.
fn note(changed: &mut Vec<ChangedControl>, old: ControlRow, new: ControlRow) -> Option<()> {
    if old.address != new.address {
        return None;
    }
    if new.descriptor.class != ControlClass::Derived && old.value.to_bits() != new.value.to_bits() {
        changed.push(ChangedControl {
            address: new.address,
            descriptor: new.descriptor,
        });
    }
    Some(())
}

/// [`list`]'s rows of one model, read one owner at a time through one buffer (issue #2232, as
/// the provider catalog reads them since #1400b): [`Iterator::next`] yields the rows that are not
/// a VCA's, in list order, and keeps the VCA rows in `vca`, in list order.
struct OwnerRows<'m> {
    view: ModelView<'m>,
    effective: &'m Effective,
    overlay: &'m MonitorOverlay,
    sample_rate: u32,
    owner: usize,
    owners: usize,
    /// The current owner's rows not yet yielded, last first.
    pending: Vec<ControlRow>,
    vca: Vec<ControlRow>,
}

impl<'m> OwnerRows<'m> {
    fn new(
        view: ModelView<'m>,
        effective: &'m Effective,
        overlay: &'m MonitorOverlay,
        sample_rate: u32,
    ) -> Self {
        Self {
            owners: owner_count(view.structure()),
            view,
            effective,
            overlay,
            sample_rate,
            owner: 0,
            pending: Vec::new(),
            vca: Vec::new(),
        }
    }
}

impl Iterator for OwnerRows<'_> {
    type Item = ControlRow;

    fn next(&mut self) -> Option<ControlRow> {
        loop {
            if let Some(row) = self.pending.pop() {
                if matches!(row.address.owner, Owner::Vca(_)) {
                    self.vca.push(row);
                    continue;
                }
                return Some(row);
            }
            if self.owner >= self.owners {
                return None;
            }
            owner_rows_in(
                self.view,
                self.effective,
                self.overlay,
                self.owner,
                self.sample_rate,
                &mut self.pending,
            );
            self.pending.reverse();
            self.owner += 1;
        }
    }
}

/// Whether any control row's `active` flag differs between `current` and `next` (#1631 D1).
///
/// A row's activity depends on a value of the model, not on the row's own value: a strip's
/// `matrix_mode` decides which of its pan and matrix rows are active. Today the predicate is true
/// exactly when a strip's `matrix_or_pan` variant differs; a later row whose activity depends on a
/// value joins it here, so a caller keeps no row list. Two models that do not hold the same strips
/// in the same order differ in structure, which counts as a change.
///
/// Allocation: none.
#[must_use]
pub fn activity_changed(current: &SessionModel, next: &SessionModel) -> bool {
    let mut before = current.strips();
    let mut after = next.strips();
    loop {
        match (before.next(), after.next()) {
            (None, None) => return false,
            (Some(old), Some(new)) => {
                if old.id != new.id
                    || std::mem::discriminant(old.matrix_or_pan)
                        != std::mem::discriminant(new.matrix_or_pan)
                {
                    return true;
                }
            }
            _ => return true,
        }
    }
}

/// Whether a write to the row at `address` changes that row alone: the session's rows do not (a ramp
/// length changes every owner's descriptors) and a strip's `matrix_mode` does not (the switch flips
/// which rows are active and rewrites the pan or the matrix). A host that applies a transaction of
/// single-row writes in place, and [`changed_rows`], need exactly this rule (#1612, #1613).
#[must_use]
pub fn writes_only_its_row(address: &ControlAddress) -> bool {
    !match (&address.owner, &address.section) {
        (Owner::Session, _) => true,
        (Owner::Strip(_), Section::Own) => address.control == strip::MATRIX_MODE,
        _ => false,
    }
}

/// The controls of the one row `address` names whose value bits differ between `current` and `next`,
/// one change per listed lane, exactly as [`changed_controls`] gives them for that row (#1612 D4).
/// The lane of `address` does not matter: the row's lanes are all compared.
///
/// It resolves the row in each model and reads its committed value, so its cost is the size of the
/// row, not of the model.
///
/// `None` when the row cannot be judged on its own, and a caller then runs [`changed_controls`]:
/// the address does not resolve in a model, the row is derived, the row is held by the host (solo),
/// the row's descriptor differs between the models in its class or its `active` flag, or a write to
/// the row moves more than the row. The last holds for the session's rows (a ramp length changes
/// every owner's descriptors) and for a strip's `matrix_mode` (the switch flips which rows are
/// active and rewrites the pan or the matrix).
#[must_use]
pub fn changed_rows(
    current: &SessionModel,
    next: &SessionModel,
    address: &ControlAddress,
    sample_rate: u32,
) -> Option<Vec<ChangedControl>> {
    changed_rows_in(
        ModelView::whole(current),
        ModelView::whole(next),
        address,
        sample_rate,
    )
}

/// [`changed_rows`] on two whole views (#1914): each reads the entries that drive a strip's
/// `fader_db` or a VCA's `offset_db` from its [`crate::AutomationIndex`] when it is
/// [`ModelView::indexed`], so a caller that judges many rows walks each automation table once
/// instead of once per row.
///
/// # Panics
///
/// In a debug build, if either view is not whole ([`ModelView::is_whole`]).
#[must_use]
pub fn changed_rows_in(
    current: ModelView<'_>,
    next: ModelView<'_>,
    address: &ControlAddress,
    sample_rate: u32,
) -> Option<Vec<ChangedControl>> {
    if !writes_only_its_row(address) {
        return None;
    }
    let before = resolve_in(current, address, sample_rate).ok()?;
    let after = resolve_in(next, address, sample_rate).ok()?;
    let descriptor = after.descriptor;
    if descriptor.class == ControlClass::Derived
        || descriptor.scope == ControlScope::Monitor
        || before.descriptor.class != descriptor.class
        || before.descriptor.active != descriptor.active
    {
        return None;
    }
    let (Source::Stored(old), Source::Stored(new)) = (before.source, after.source) else {
        return None;
    };
    Some(
        descriptor
            .lanes
            .listed()
            .iter()
            .filter(|&&lane| {
                let side = Side::of(lane);
                side.pick(old[0], old[1]).to_bits() != side.pick(new[0], new[1]).to_bits()
            })
            .map(|&lane| ChangedControl {
                address: ControlAddress {
                    lane,
                    ..address.clone()
                },
                descriptor,
            })
            .collect(),
    )
}
