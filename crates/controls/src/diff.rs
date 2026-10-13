//! The row diff (decision 17, C6 and #1554 D2): which controls a transaction changes.

use session::SessionModel;

use crate::ControlClass;
use crate::address::ControlAddress;
use crate::descriptor::ControlDescriptor;
use crate::effective::MonitorOverlay;
use crate::registry::list;

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
    let mut before = list(current, &overlay, sample_rate);
    let mut after = list(next, &overlay, sample_rate);
    let mut changed = Vec::new();
    loop {
        match (before.next(), after.next()) {
            (None, None) => return Some(changed),
            (Some(old), Some(new)) => {
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
            _ => return None,
        }
    }
}
