//! The control list and the one read and write path: [`list`], [`get`], [`set`].

use std::collections::HashMap;

use session::SessionModel;

use crate::address::{ControlAddress, Lane, Owner, Section};
use crate::descriptor::{ControlDescriptor, ControlScope, Lanes, check};
use crate::error::ControlError;
use crate::lane::Side;
use crate::{builtin_rows, route, session_rows, strip, vca};

/// One control of a session: where it is, what it is, and its committed value.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlRow {
    /// The control's address. A per-lane control has one row per lane.
    pub address: ControlAddress,
    /// What the control is, resolved at the session's sample rate.
    pub descriptor: ControlDescriptor,
    /// The committed value. A derived row reports its computed value; a row that the owner's
    /// shape does not use, and a monitor row (solo, until the overlay exists), report the
    /// descriptor's default.
    pub value: f32,
}

/// Append the rows of one control: one per lane the descriptor lists. `value` gives the committed
/// value of a side; an inactive row reports its default instead.
pub(crate) fn push_control(
    out: &mut Vec<ControlRow>,
    owner: &Owner,
    section: &Section,
    control: u32,
    descriptor: &ControlDescriptor,
    value: impl Fn(Side) -> f32,
) {
    for &lane in descriptor.lanes.listed() {
        out.push(ControlRow {
            address: ControlAddress {
                owner: owner.clone(),
                section: section.clone(),
                control,
                lane,
            },
            descriptor: *descriptor,
            value: if descriptor.active {
                value(Side::of(lane))
            } else {
                descriptor.default
            },
        });
    }
}

/// Every control of a session, with its descriptor at `sample_rate` and its committed value.
///
/// Order: strips in [`SessionModel::strips`] order, then routes, then VCAs, then the session's
/// rows. Within a strip: its own rows, then its builtin rows (then the console and insert
/// sections, which have no rows until effect rows exist). A per-lane row is listed as `left`
/// then `right`; a shared row once, as `both`. Every address appears exactly once.
pub fn list(model: &SessionModel, sample_rate: u32) -> impl Iterator<Item = ControlRow> + use<> {
    let effective = model.effective_strip_faders();
    let source_mutes: HashMap<&str, [bool; 2]> = model
        .strips()
        .zip(&effective)
        .map(|(strip, effective)| (strip.id.as_str(), effective.mute))
        .collect();
    let mut rows = Vec::new();
    for (strip, effective) in model.strips().zip(&effective) {
        strip::push_rows(&mut rows, &strip, effective, sample_rate);
    }
    for route in &model.routes {
        let source_mute = source_mutes
            .get(route::source_strip(route).as_str())
            .copied()
            .unwrap_or_default();
        route::push_rows(&mut rows, route, source_mute);
    }
    for vca in &model.vcas {
        vca::push_rows(&mut rows, vca);
    }
    session_rows::push_rows(&mut rows, model);
    rows.into_iter()
}

/// The rows of one owner, in listing order.
fn owner_rows(
    model: &SessionModel,
    owner: &Owner,
    sample_rate: u32,
) -> Result<Vec<ControlRow>, ControlError> {
    let mut rows = Vec::new();
    match owner {
        Owner::Session => session_rows::push_rows(&mut rows, model),
        Owner::Strip(id) => {
            let (strip, effective) = model
                .strips()
                .zip(model.effective_strip_faders())
                .find(|(strip, _)| strip.id == id)
                .ok_or(ControlError::AddressUnknown)?;
            strip::push_rows(&mut rows, &strip, &effective, sample_rate);
        }
        Owner::Route(id) => {
            let route = model
                .routes
                .iter()
                .find(|route| route.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            let source = route::source_strip(route);
            let source_mute = model
                .strips()
                .zip(model.effective_strip_faders())
                .find(|(strip, _)| strip.id == source)
                .map_or([false; 2], |(_, effective)| effective.mute);
            route::push_rows(&mut rows, route, source_mute);
        }
        Owner::Vca(id) => {
            let vca = model
                .vcas
                .iter()
                .find(|vca| vca.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            vca::push_rows(&mut rows, vca);
        }
    }
    Ok(rows)
}

/// The committed value of the control at `address`.
///
/// The address must be one that [`list`] gives: a per-lane control is read on `left` or `right`,
/// never `both`, and a shared control on `both`.
///
/// # Errors
///
/// [`ControlError::AddressUnknown`] if no owner, section or row has that address;
/// [`ControlError::Lane`] if the row exists but not on that lane.
pub fn get(
    model: &SessionModel,
    address: &ControlAddress,
    sample_rate: u32,
) -> Result<f32, ControlError> {
    let rows = owner_rows(model, &address.owner, sample_rate)?;
    if let Some(row) = rows.iter().find(|row| row.address == *address) {
        return Ok(row.value);
    }
    if rows.iter().any(|row| row.address.same_row(address)) {
        Err(ControlError::Lane)
    } else {
        Err(ControlError::AddressUnknown)
    }
}

/// Write `value` to the control at `address`: look it up, validate with [`check`], then change the
/// model. Nothing changes if any step refuses.
///
/// `ramp` is an explicit ramp length in samples for the live path; the model write does not use
/// it (the classifier and the lowering do), but a ramp that the row cannot honour is refused here
/// so that an acknowledgement never hides an ignored field.
///
/// A value equal to the committed one is accepted and changes nothing. A write to a monitor row
/// (solo) is refused with [`ControlError::Monitor`]: it is not a session mutation, and the
/// overlay writes it.
///
/// # Errors
///
/// Any [`ControlError`].
pub fn set(
    model: &mut SessionModel,
    address: &ControlAddress,
    value: f32,
    ramp: Option<u32>,
    sample_rate: u32,
) -> Result<(), ControlError> {
    let rows = owner_rows(model, &address.owner, sample_rate)?;
    let mut targets = rows.iter().filter(|row| row.address.same_row(address));
    let descriptor = targets
        .next()
        .ok_or(ControlError::AddressUnknown)?
        .descriptor;
    check(&descriptor, address.lane, value, ramp)?;
    if descriptor.scope == ControlScope::Monitor {
        return Err(ControlError::Monitor);
    }
    let unchanged = rows
        .iter()
        .filter(|row| {
            row.address.same_row(address)
                && (address.lane == Lane::Both || row.address.lane == address.lane)
        })
        .all(|row| row.value.to_bits() == value.to_bits());
    if unchanged {
        return Ok(());
    }
    write(model, address, descriptor.lanes, value)
}

/// The model write behind [`set`], for a value that passed [`check`].
fn write(
    model: &mut SessionModel,
    address: &ControlAddress,
    lanes: Lanes,
    value: f32,
) -> Result<(), ControlError> {
    let control = address.control;
    let sides = lanes.targets(address.lane);
    match (&address.owner, &address.section) {
        (Owner::Session, Section::Own) => session_rows::write(model, control, value),
        (Owner::Strip(id), Section::Own) => {
            let mut target = strip::strip_mut(model, id).ok_or(ControlError::AddressUnknown)?;
            sides
                .iter()
                .try_for_each(|&side| strip::write_own(&mut target, control, side, value))
        }
        (Owner::Strip(id), Section::Builtins) => {
            let mut target = strip::strip_mut(model, id).ok_or(ControlError::AddressUnknown)?;
            sides
                .iter()
                .try_for_each(|&side| builtin_rows::write(&mut target, control, side, value))
        }
        (Owner::Route(id), Section::Own) => {
            let target = model
                .routes
                .iter_mut()
                .find(|route| route.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            route::write(target, control, value)
        }
        (Owner::Vca(id), Section::Own) => {
            let target = model
                .vcas
                .iter_mut()
                .find(|vca| vca.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            sides
                .iter()
                .try_for_each(|&side| vca::write(target, control, side, value))
        }
        _ => Err(ControlError::AddressUnknown),
    }
}
