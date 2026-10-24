//! The control list and the one read and write path: [`list`], [`get`], [`set`].

use session::{SessionModel, StableId, StripRef};

use crate::Family;
use crate::automation::AutomationIndex;
use crate::descriptor::{ControlDescriptor, ControlScope, Lanes, Motion, check};
use crate::effective::{
    Effective, EffectiveSource, EffectiveStrip, MonitorOverlay, MonitorRow, MonitorWrite, effective,
};
use crate::error::ControlError;
use crate::lane::{Side, both_sides, toggle};
use crate::view::ModelView;
use crate::{builtin_rows, effect_rows, route, session_rows, strip, vca};
use session::{ControlAddress, Lane, Owner, OwnerKind, Section, SectionKind};

/// One control of a session: where it is, what it is, and its committed value.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlRow {
    /// The control's address. A per-lane control has one row per lane.
    pub address: ControlAddress,
    /// What the control is, resolved at the session's sample rate.
    pub descriptor: ControlDescriptor,
    /// The committed value. A derived row reports its computed value, a monitor row (solo) the
    /// overlay's, and a row that the owner's shape does not use the descriptor's default.
    pub value: f32,
    /// Whether a rendered stored automation drives the row's lane (#1650 D4): the `fader_db` row,
    /// since #1651 (root ruling R18-2) the `mute` row, and since #1655 a VCA's `offset_db` and
    /// `mute`. On such a lane the `fader_db` row's value is the driving entry's hand-move offset,
    /// not the static fader, and so is a VCA's `offset_db` (#1655 D5); the `mute` rows' values stay
    /// the static (hand) mute, which render ORs with the curve (OQ1 = C). A VCA ride does not mark
    /// its members' rows: they list their own entries only.
    pub automated: bool,
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
    push_control_automated(out, owner, section, control, descriptor, value, |_| false);
}

/// [`push_control`] for a row that a rendered stored automation can drive: `automated` says, per
/// side, whether one does (#1650 D4).
pub(crate) fn push_control_automated(
    out: &mut Vec<ControlRow>,
    owner: &Owner,
    section: &Section,
    control: u32,
    descriptor: &ControlDescriptor,
    value: impl Fn(Side) -> f32,
    automated: impl Fn(Side) -> bool,
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
            automated: automated(Side::of(lane)),
        });
    }
}

/// Every control of a session, with its descriptor at `sample_rate` and its committed value.
/// A derived row reports the value [`effective`] computes under `overlay`, and a monitor row
/// (solo) reports the overlay's.
///
/// Order: strips in [`SessionModel::strips`] order, then routes, then VCAs, then the session's
/// rows. Within a strip: its own rows, then its builtin rows, then the effect rows in chain order: the pre-insert console slots, the inserts, the post-insert
/// console slots (each instance: bypass, parameters, link mode). A per-lane row is listed as `left`
/// then `right`; a shared row once, as `both`. Every address appears exactly once.
///
/// The list is the rows of every owner in turn, from [`owner_rows_in`] over the whole model and
/// nowhere else: the rows [`owner_rows_with`] gives, with the automation table walked once
/// ([`AutomationIndex`], #1710).
pub fn list(
    model: &SessionModel,
    overlay: &MonitorOverlay,
    sample_rate: u32,
) -> impl Iterator<Item = ControlRow> + use<> {
    let effective = effective(model, overlay);
    let automation = AutomationIndex::of(model);
    let view = ModelView::whole(model).indexed(&automation);
    let mut rows = Vec::new();
    for owner in 0..owner_count(model) {
        owner_rows_in(view, &effective, overlay, owner, sample_rate, &mut rows);
    }
    rows.into_iter()
}

/// How many owners [`list`] walks: the strips, the routes, the VCAs, then the session. An owner
/// can list no row; [`owner_rows_with`] then appends nothing.
#[must_use]
pub fn owner_count(model: &SessionModel) -> usize {
    model.tracks.len() + model.submixes.len() + model.routes.len() + model.vcas.len() + 1
}

/// The index of `owner` in [`list`]'s owner order (see [`owner_count`]): the strips (tracks, then
/// submixes), the routes, the VCAs, then the session. `None` when the model has no such owner. It is
/// the inverse of the order [`owner_rows_with`] walks, so a caller that holds an [`Owner`] and wants
/// its rows names the index here and nowhere else.
#[must_use]
pub fn owner_index(model: &SessionModel, owner: &Owner) -> Option<usize> {
    let strips = model.tracks.len() + model.submixes.len();
    match owner {
        Owner::Strip(id) => strip_index(model, id),
        Owner::Route(id) => search(&model.routes, id, |route| &route.id)
            .or_else(|| model.routes.iter().position(|route| route.id == *id))
            .map(|index| strips + index),
        Owner::Vca(id) => model
            .vcas
            .iter()
            .position(|vca| vca.id == *id)
            .map(|index| strips + model.routes.len() + index),
        Owner::Session => Some(owner_count(model) - 1),
    }
}

/// The index of the strip `id` in [`SessionModel::strips`] order (the tracks, then the submixes),
/// or `None` when the model has no such strip: the same answer as a scan, in `O(log n)` on a model
/// whose tracks and submixes are in ID order (a compiled session's normalized model), and by a scan
/// on any other.
#[must_use]
pub fn strip_index(model: &SessionModel, id: &StableId) -> Option<usize> {
    search(&model.tracks, id, |track| &track.id)
        .or_else(|| {
            search(&model.submixes, id, |submix| &submix.id).map(|at| model.tracks.len() + at)
        })
        .or_else(|| model.strips().position(|strip| strip.id == id))
}

/// A binary search for the item whose ID is `id`. Right when `items` are in ID order; on any other
/// slice it may miss, and the caller then scans.
fn search<T>(items: &[T], id: &StableId, id_of: impl Fn(&T) -> &StableId) -> Option<usize> {
    items.binary_search_by(|item| id_of(item).cmp(id)).ok()
}

/// Which kind of owner an index of [`list`]'s owner order names, and where in its table.
enum OwnerSlot {
    Strip(usize),
    Route(usize),
    Vca(usize),
    Session,
    None,
}

fn owner_slot(model: &SessionModel, owner: usize) -> OwnerSlot {
    let strips = model.tracks.len() + model.submixes.len();
    if owner < strips {
        return OwnerSlot::Strip(owner);
    }
    let owner = owner - strips;
    if owner < model.routes.len() {
        return OwnerSlot::Route(owner);
    }
    let owner = owner - model.routes.len();
    if owner < model.vcas.len() {
        OwnerSlot::Vca(owner)
    } else if owner == model.vcas.len() {
        OwnerSlot::Session
    } else {
        OwnerSlot::None
    }
}

/// The strip at `index` of [`SessionModel::strips`] order: the tracks, then the submixes.
pub(crate) fn strip_at(model: &SessionModel, index: usize) -> StripRef<'_> {
    match model.tracks.get(index) {
        Some(track) => StripRef::track(track),
        None => StripRef::submix(&model.submixes[index - model.tracks.len()]),
    }
}

/// Append the rows of owner number `owner` of [`list`]'s order (see [`owner_count`]) to `out`, from
/// the effective values the caller computed once with [`effective`] for the same `model` and
/// `overlay`. [`list`] is this function for every owner in turn, so a caller that reads an owner at
/// a time (to compact each into a smaller form, reusing one `out`, or to refresh only the owners
/// that changed) gets the very rows `list` gives, in the same order, without the whole list ever
/// being held.
///
/// Appends nothing for an `owner` of `owner_count(model)` or more.
pub fn owner_rows_with(
    model: &SessionModel,
    effective: &Effective,
    overlay: &MonitorOverlay,
    owner: usize,
    sample_rate: u32,
    out: &mut Vec<ControlRow>,
) {
    owner_rows_in(
        ModelView::whole(model),
        effective,
        overlay,
        owner,
        sample_rate,
        out,
    );
}

/// [`owner_rows_with`] over a [`ModelView`] and any source of effective values: the rows of owner
/// number `owner` of the model the view stands for, read from the committed model and the owners the
/// view holds edited, with the effective values of `effective` (#1613). The one body of both.
pub fn owner_rows_in(
    view: ModelView<'_>,
    effective: &impl EffectiveSource,
    overlay: &MonitorOverlay,
    owner: usize,
    sample_rate: u32,
    out: &mut Vec<ControlRow>,
) {
    let model = view.structure();
    match owner_slot(model, owner) {
        OwnerSlot::Strip(index) => {
            let strip = view.strip(index);
            let driven = strip::Driven {
                offsets: builtin_rows::fader_offsets(view.fader_automation(strip.id)),
                // Which entry drives a mute lane is read from the committed table, as for the
                // fader (a transaction of values leaves the table's entries and targets as
                // committed).
                muted: view.mute_automation(strip.id).map(|entry| entry.is_some()),
            };
            strip::push_rows(
                out,
                &model.console,
                &strip,
                driven,
                &effective.strip(index),
                overlay,
                sample_rate,
            );
        }
        OwnerSlot::Route(index) => {
            route::push_rows(out, view.route(index), effective.route_gate(index));
        }
        OwnerSlot::Vca(index) => {
            let vca = view.vca(index);
            let entries = if model.automation.is_empty() {
                vca::VcaEntries::default()
            } else {
                vca::VcaEntries {
                    offset: view.vca_automation(&vca.id),
                    // Which entry drives a mute lane is read from the committed table.
                    muted: view
                        .vca_mute_automation(&vca.id)
                        .map(|entry| entry.is_some()),
                }
            };
            vca::push_rows(out, vca, entries);
        }
        OwnerSlot::Session => session_rows::push_rows(out, model),
        OwnerSlot::None => {}
    }
}

/// Append only the derived rows of owner number `owner` (a strip's effective fader and mute, a
/// route's effective gate), in the order [`owner_rows_with`] gives them, from the same `effective`
/// values. Another owner kind has no derived row and appends nothing.
///
/// A derived value follows other owners' controls (a VCA, a followed mute), so a catalog that
/// refreshes only the changed owners reads these for every owner that has them.
pub fn derived_rows_with(
    model: &SessionModel,
    effective: &Effective,
    owner: usize,
    out: &mut Vec<ControlRow>,
) {
    derived_rows_in(ModelView::whole(model), effective, owner, out);
}

/// [`derived_rows_with`] over a [`ModelView`] and any source of effective values (#1613).
pub fn derived_rows_in(
    view: ModelView<'_>,
    effective: &impl EffectiveSource,
    owner: usize,
    out: &mut Vec<ControlRow>,
) {
    match owner_slot(view.structure(), owner) {
        OwnerSlot::Strip(index) => {
            strip::push_derived_rows(out, &view.strip(index), &effective.strip(index));
        }
        OwnerSlot::Route(index) => {
            route::push_derived_rows(out, view.route(index), effective.route_gate(index));
        }
        OwnerSlot::Vca(_) | OwnerSlot::Session | OwnerSlot::None => {}
    }
}

/// How the non-effect row `control` of `section` of `owner` moves, with no sample rate: the
/// motion of a row (a ramp row or a fixed ramp) does not depend on the rate, so a caller that
/// needs only the ramp row asks this and not [`row_descriptor`] at a placeholder rate. The same
/// rows and `None` cases as [`row_descriptor`].
#[must_use]
pub fn row_motion(owner: OwnerKind, section: SectionKind, control: u32) -> Option<Motion> {
    match (owner, section) {
        (OwnerKind::Strip, SectionKind::Builtins) => {
            builtin_rows::row(control).map(|row| row.ramp.map_or(Motion::Fixed, Motion::Ramp))
        }
        _ => row_descriptor(owner, section, control, 0).map(|descriptor| descriptor.motion),
    }
}

/// Where the non-effect row `control` of `section` of `owner` is held, with no sample rate: the
/// scope of a row does not depend on the rate (every builtin row is a model row), so a caller that
/// needs only the scope asks this and not [`row_descriptor`] at a placeholder rate. The same rows
/// and `None` cases as [`row_descriptor`].
#[must_use]
pub fn row_scope(owner: OwnerKind, section: SectionKind, control: u32) -> Option<ControlScope> {
    match (owner, section) {
        (OwnerKind::Strip, SectionKind::Builtins) => {
            builtin_rows::row(control).map(|_| ControlScope::Model)
        }
        _ => row_descriptor(owner, section, control, 0).map(|descriptor| descriptor.scope),
    }
}

/// The descriptor of one non-effect row at `sample_rate`, without a model: for a host that checks
/// a write against the row before it holds the session (decision 17, slice S7c).
///
/// `control` is the row ID in its table: a builtin ID for `strip` and `builtins`, an own-row ID
/// otherwise. A strip's row is active (the host checks the strip's pan or matrix shape itself), and
/// a route's row is the row of a route into a submix, the class the browser admits. `None` for an
/// effect section (the console and the inserts), whose rows come from the effect descriptors, and
/// for an ID that no table of that owner has.
#[must_use]
pub fn row_descriptor(
    owner: OwnerKind,
    section: SectionKind,
    control: u32,
    sample_rate: u32,
) -> Option<ControlDescriptor> {
    match (owner, section) {
        (OwnerKind::Strip, SectionKind::Builtins) => builtin_rows::row(control)
            .map(|row| builtin_rows::shaped_descriptor(row, sample_rate, true)),
        (OwnerKind::Strip, SectionKind::Own) => strip::ROWS
            .iter()
            .find(|row| row.id == control)
            .map(|row| row.descriptor),
        (OwnerKind::Route, SectionKind::Own) => route::ROWS
            .iter()
            .find(|row| row.id == control)
            .map(|row| row.descriptor),
        (OwnerKind::Vca, SectionKind::Own) => vca::ROWS
            .iter()
            .find(|row| row.id == control)
            .map(|row| row.descriptor),
        (OwnerKind::Session, SectionKind::Own) => session_rows::ROWS
            .iter()
            .find(|row| row.id == control)
            .map(|row| row.descriptor),
        _ => None,
    }
}

/// How a resolved row reaches its committed value.
pub(crate) enum Source {
    /// A field of the model: the value of the `left` and the `right` side (equal for a shared
    /// row; a row that the owner's shape does not use holds its default).
    Stored([f32; 2]),
    /// A strip's derived row: computed from the effective values.
    StripDerived(fn(&EffectiveStrip, Side) -> f32),
    /// A route's derived row: computed from the route's effective gate.
    RouteDerived(fn([bool; 2], Side) -> f32),
    /// Held by the host (solo, master): the overlay's value of that row.
    Monitor(MonitorRow),
}

/// One row found by address: its descriptor and where its value comes from. Built without
/// listing the owner's other rows and without computing a derived value, so a write costs the
/// size of the row, not the size of the model.
pub(crate) struct Resolved {
    pub(crate) descriptor: ControlDescriptor,
    pub(crate) source: Source,
}

impl Resolved {
    /// A stored row: `value` gives the committed value of a side, and a row the owner's shape does
    /// not use reports the descriptor's default (as [`push_control`] does).
    pub(crate) fn stored(descriptor: ControlDescriptor, value: impl Fn(Side) -> f32) -> Self {
        let values = if descriptor.active {
            both_sides(value)
        } else {
            [descriptor.default; 2]
        };
        Self {
            descriptor,
            source: Source::Stored(values),
        }
    }
}

/// Find the row at `address` (its owner, section and control, on any lane): [`resolve_in`] on the
/// whole model, so a strip's `fader_db` or a VCA's `offset_db` scans the automation table once.
pub(crate) fn resolve(
    model: &SessionModel,
    address: &ControlAddress,
    sample_rate: u32,
) -> Result<Resolved, ControlError> {
    resolve_in(ModelView::whole(model), address, sample_rate)
}

/// Find the row at `address` (its owner, section and control, on any lane) in the whole model
/// `view` reads. The driving entries of a strip's `fader_db` and a VCA's `offset_db` are read
/// through the view: a lookup when it is [`ModelView::indexed`] (a caller that resolves many
/// addresses, #1914), else one scan of the table.
pub(crate) fn resolve_in(
    view: ModelView<'_>,
    address: &ControlAddress,
    sample_rate: u32,
) -> Result<Resolved, ControlError> {
    debug_assert!(view.is_whole(), "resolve reads a whole model");
    let model = view.structure();
    let control = address.control;
    match (&address.owner, &address.section) {
        (Owner::Session, Section::Own) => session_rows::resolve(model, control),
        (Owner::Strip(id), section) => strip::resolve(view, id, section, control, sample_rate),
        (Owner::Route(id), Section::Own) => route::resolve(model, id, control),
        (Owner::Vca(id), Section::Own) => vca::resolve(view, id, control),
        _ => Err(ControlError::AddressUnknown),
    }
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
    overlay: &MonitorOverlay,
    address: &ControlAddress,
    sample_rate: u32,
) -> Result<f32, ControlError> {
    let resolved = resolve(model, address, sample_rate)?;
    if !resolved.descriptor.lanes.listed().contains(&address.lane) {
        return Err(ControlError::Lane);
    }
    let side = Side::of(address.lane);
    Ok(match resolved.source {
        Source::Stored(values) => side.pick(values[0], values[1]),
        Source::Monitor(row) => match &address.owner {
            Owner::Strip(id) => toggle(crate::strip::monitor_value(overlay, row, id)),
            _ => 0.0,
        },
        // Only a derived row pays for the effective values.
        Source::StripDerived(read) => {
            let effective = effective(model, overlay);
            let Owner::Strip(id) = &address.owner else {
                return Err(ControlError::AddressUnknown);
            };
            let index = model
                .strips()
                .position(|strip| strip.id == id)
                .ok_or(ControlError::AddressUnknown)?;
            read(&effective.strips[index], side)
        }
        Source::RouteDerived(read) => {
            let effective = effective(model, overlay);
            let Owner::Route(id) = &address.owner else {
                return Err(ControlError::AddressUnknown);
            };
            let index = model
                .routes
                .iter()
                .position(|route| route.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            read(effective.routes[index].gate, side)
        }
    })
}

/// Write `value` to the control at `address`: look it up, validate with [`check`], then change the
/// model. Nothing changes if any step refuses.
///
/// The row is found by its address and its descriptor and committed value are read from that row
/// alone: no other row of the owner is built and no effective value is computed (a transaction of
/// many writes costs the sum of its rows, not edits times the model).
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
    let resolved = resolve(model, address, sample_rate)?;
    let descriptor = resolved.descriptor;
    check(&descriptor, address.lane, value, ramp)?;
    // The effect contract treats the two zeros as one value and preparation normalises them.
    let value = if descriptor.family == Family::Effect {
        effect_contract::normalize_zero(value)
    } else {
        value
    };
    // `check` has refused a derived row, so only a monitor row or a stored row is left.
    let committed = match resolved.source {
        Source::Stored(committed) => committed,
        Source::Monitor(_) => return Err(ControlError::Monitor),
        Source::StripDerived(_) | Source::RouteDerived(_) => return Err(ControlError::ReadOnly),
    };
    let unchanged = descriptor
        .lanes
        .listed()
        .iter()
        .filter(|&&lane| address.lane == Lane::Both || lane == address.lane)
        .all(|&lane| Side::of(lane).pick(committed[0], committed[1]).to_bits() == value.to_bits());
    if unchanged {
        return Ok(());
    }
    write(model, address, descriptor.lanes, value)
}

/// Validate a write to a monitor row (solo, or the master designation of #1683) and say what it
/// sets (decision 17, C8, #1559 D3).
///
/// The address is looked up and the value checked exactly as [`set`] does: an unknown strip (a
/// master designation that names a strip the session does not have is refused here, before any
/// commit), a submix's solo (it has no solo row, a submix is solo-safe) or an unknown control is
/// [`ControlError::AddressUnknown`], a lane other than `both` is [`ControlError::Lane`], a value
/// other than exactly `0` or `1` is [`ControlError::Domain`]. Nothing is written: the overlay is
/// the host's. `Ok(None)` for an address whose row is a model row, which [`set`] writes; its
/// value is not checked here.
///
/// # Errors
///
/// Any [`ControlError`] a write to the row would meet before its write.
pub fn check_monitor(
    model: &SessionModel,
    address: &ControlAddress,
    value: f32,
    ramp: Option<u32>,
    sample_rate: u32,
) -> Result<Option<MonitorWrite>, ControlError> {
    let resolved = resolve(model, address, sample_rate)?;
    if resolved.descriptor.scope != ControlScope::Monitor {
        return Ok(None);
    }
    check(&resolved.descriptor, address.lane, value, ramp)?;
    let Owner::Strip(strip) = &address.owner else {
        return Err(ControlError::AddressUnknown);
    };
    let Source::Monitor(row) = resolved.source else {
        return Err(ControlError::AddressUnknown);
    };
    Ok(Some(MonitorWrite {
        strip: strip.clone(),
        row,
        on: crate::lane::is_on(value),
        ramp,
    }))
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
            // An automated fader lane's value is its entry's hand-move offset (#1650 D3).
            let static_sides = if control == builtin_rows::FADER_DB {
                builtin_rows::write_fader_offset(model, id, sides, value)
            } else {
                [true; 2]
            };
            let mut target = strip::strip_mut(model, id).ok_or(ControlError::AddressUnknown)?;
            sides
                .iter()
                .filter(|&&side| static_sides[side.pick(0, 1)])
                .try_for_each(|&side| builtin_rows::write(&mut target, control, side, value))
        }
        (Owner::Strip(_), Section::Console(_) | Section::Inserts(_)) => {
            effect_rows::write(model, address, value)
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
            // An automated VCA offset lane's value is its entry's hand-move offset (#1655 D5).
            let static_sides = if control == vca::VCA_OFFSET_DB {
                vca::write_offset(model, id, sides, value)
            } else {
                [true; 2]
            };
            let target = model
                .vcas
                .iter_mut()
                .find(|vca| vca.id == *id)
                .ok_or(ControlError::AddressUnknown)?;
            sides
                .iter()
                .filter(|&&side| static_sides[side.pick(0, 1)])
                .try_for_each(|&side| vca::write(target, control, side, value))
        }
        _ => Err(ControlError::AddressUnknown),
    }
}
