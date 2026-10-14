//! The row diff (decision 17, C6 and #1554 D2): which controls a transaction changes.

use session::SessionModel;

use crate::ControlClass;
use crate::address::{ControlAddress, Owner};
use crate::descriptor::ControlDescriptor;
use crate::effective::MonitorOverlay;
use crate::registry::{ControlRow, list};

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
/// Allocation: the list of each model and the result, on the control thread.
#[must_use]
pub fn changed_controls(
    current: &SessionModel,
    next: &SessionModel,
    sample_rate: u32,
) -> Option<Vec<ChangedControl>> {
    let overlay = MonitorOverlay::default();
    let is_vca = |row: &ControlRow| matches!(row.address.owner, Owner::Vca(_));
    let (before_vca, before): (Vec<_>, Vec<_>) =
        list(current, &overlay, sample_rate).partition(is_vca);
    let (after_vca, after): (Vec<_>, Vec<_>) = list(next, &overlay, sample_rate).partition(is_vca);
    let mut changed = Vec::new();
    // The VCA rows are the same addresses, or no VCA row is compared.
    let vca_rows = (before_vca.len() == after_vca.len()
        && before_vca
            .iter()
            .zip(&after_vca)
            .all(|(old, new)| old.address == new.address))
    .then_some((before_vca, after_vca));
    if before.len() != after.len() {
        return None;
    }
    for (old, new) in before.into_iter().zip(after).chain(
        vca_rows
            .into_iter()
            .flat_map(|(old, new)| old.into_iter().zip(new)),
    ) {
        if old.address != new.address {
            return None;
        }
        if new.descriptor.class == ControlClass::Derived
            || old.value.to_bits() == new.value.to_bits()
        {
            continue;
        }
        changed.push(ChangedControl {
            address: new.address,
            descriptor: new.descriptor,
        });
    }
    Some(changed)
}
