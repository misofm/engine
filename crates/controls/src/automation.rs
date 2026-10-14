//! The row rule of a stored automation (decision 17, C10; #1566 D6): one function that holds every
//! stored automation of a session to the control it names.

use session::{ControlAddress, Owner, Section, SessionModel};

use crate::descriptor::{Lanes, check};
use crate::effect_rows::{has_rows, session_unit};
use crate::error::ControlError;
use crate::registry::resolve;
use session::Lane;

/// Whether `target` names an effect instance (a console slot or an insert) whose identity has no
/// control rows: a `cid` effect, or a native effect the launch registry lacks.
fn effect_has_no_rows(model: &SessionModel, target: &ControlAddress) -> bool {
    let Owner::Strip(strip_id) = &target.owner else {
        return false;
    };
    let identity = match &target.section {
        Section::Console(slot) => model
            .console
            .slots()
            .find(|candidate| candidate.slot == *slot)
            .map(|candidate| &candidate.identity),
        Section::Inserts(effect) => model
            .strips()
            .find(|strip| strip.id == strip_id)
            .and_then(|strip| {
                strip
                    .inserts
                    .effects
                    .iter()
                    .find(|candidate| candidate.id == *effect)
            })
            .map(|candidate| &candidate.identity),
        Section::Own | Section::Builtins => None,
    };
    identity.is_some_and(|identity| !has_rows(identity))
}

/// One refusal of a stored automation: a stable code and the session path it belongs to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlDiagnostic {
    /// The stable machine code (a [`ControlError`] code).
    pub code: &'static str,
    /// `$.automation[id=<id>].target`, or `$.automation[id=<id>].segments[<i>]`.
    pub path: String,
}

/// Every refusal of the stored automation of `model`, judged by the row each target names.
///
/// The codes are the control validator's own ([`check`]) and one more, so a stored automation, a
/// `SET_CONTROL` write and an enqueued automation are refused alike:
///
/// - `control.address.unknown`: no row at the address (an unknown builtin ID, effect parameter or
///   own row);
/// - `control.automation.target`: the row is not [`automatable`](crate::ControlDescriptor::automatable);
/// - `control.inactive`: the row is automatable but the owner's shape does not use it (a matrix row
///   on a pan strip);
/// - `control.lane`: `left` or `right` on a shared row;
/// - `control.domain`: a segment whose unit is not the row's unit, or a value (`start_value` or
///   `end_value`) the row refuses.
///
/// A target is reported once, at `.target`; its segments are judged only when the target is
/// accepted. A segment is reported once, at `.segments[<i>]`. Whether the owner, the console slot
/// and the insert exist is the session validator's rule, not this one's; an address it names but the
/// model lacks is `control.address.unknown` here too. A target in an effect that has no rows (a
/// `cid` effect, a native effect the registry lacks) is not reported here: effect preparation
/// refuses that effect with its own code.
///
/// Control plane only: it allocates its result.
#[must_use]
pub fn automation_diagnostics(model: &SessionModel, sample_rate: u32) -> Vec<ControlDiagnostic> {
    let mut diagnostics = Vec::new();
    for automation in &model.automation {
        let target = &automation.target;
        let at = |suffix: &str| format!("$.automation[id={}]{suffix}", automation.id.as_str());
        let refuse = |error: ControlError, path: String| ControlDiagnostic {
            code: error.code(),
            path,
        };
        let descriptor = match resolve(model, target, sample_rate) {
            Ok(resolved) => resolved.descriptor,
            // An effect that carries no rows is refused by effect preparation with its own code
            // (`effect.native.unavailable`, `effect.third_party.unavailable_at_launch`); one fault is
            // reported once.
            Err(_) if effect_has_no_rows(model, target) => continue,
            Err(error) => {
                diagnostics.push(refuse(error, at(".target")));
                continue;
            }
        };
        if !descriptor.automatable() {
            diagnostics.push(refuse(ControlError::AutomationTarget, at(".target")));
            continue;
        }
        // The target's own faults, in `check`'s order: an inactive row, then a lane the row lacks.
        if !descriptor.active {
            diagnostics.push(refuse(ControlError::Inactive, at(".target")));
            continue;
        }
        if descriptor.lanes == Lanes::Shared && target.lane != Lane::Both {
            diagnostics.push(refuse(ControlError::Lane, at(".target")));
            continue;
        }
        for (position, segment) in automation.segments.iter().enumerate() {
            let refused = session_unit(descriptor.unit) != segment.unit
                || [segment.start_value, segment.end_value]
                    .into_iter()
                    .any(|value| check(&descriptor, target.lane, value, None).is_err());
            if refused {
                diagnostics.push(refuse(
                    ControlError::Domain,
                    at(&format!(".segments[{position}]")),
                ));
            }
        }
    }
    diagnostics
}
