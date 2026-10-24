//! A VCA group's rows (decision 17, C3). A VCA has no audio path; its offset and mute reach render
//! through each member's effective fader and mute.

use effect_contract::{ParameterLattice, ParameterUnit};
use session::{Automation, FADER_MAXIMUM_DB, FADER_MINIMUM_DB, SessionModel, StableId, Vca};

use crate::descriptor::{BASE, ControlDescriptor, Lanes, Motion};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::registry::{ControlRow, Resolved, push_control_automated};
use crate::view::ModelView;
use crate::{ControlKind, Family, LiveRampRow, Taper};
use session::{Owner, Section};

const OFFSET_DB: u32 = 1;
const MUTE: u32 = 2;
/// The ID of the VCA's `offset_db` row: per lane, it takes a hand-move offset (#1655 D5).
pub(crate) const VCA_OFFSET_DB: u32 = OFFSET_DB;
/// The ID of the VCA's `mute` row (#1655 D4).
pub(crate) const VCA_MUTE: u32 = MUTE;

/// One row of the VCA table.
pub(crate) struct VcaRow {
    pub(crate) id: u32,
    pub(crate) descriptor: ControlDescriptor,
    /// The committed value of one lane.
    read: fn(&Vca, Side) -> f32,
    /// The model write, after the value passed [`crate::check`].
    write: fn(&mut Vca, Side, f32),
}

/// What a VCA row starts from. A VCA's fader is stored per lane, so its rows are per lane. A VCA
/// has no render cell: a change reaches render as its members' effective fader and mute (#1558),
/// so the family is the strip level's.
const VCA_BASE: ControlDescriptor = ControlDescriptor {
    lanes: Lanes::PerLane,
    family: Family::StripLevel,
    ..BASE
};

/// The VCA rows, in listing order. Never reuse an ID.
pub(crate) static ROWS: [VcaRow; 2] = [
    VcaRow {
        id: OFFSET_DB,
        descriptor: ControlDescriptor {
            name: "offset_db",
            display_name: "VCA Offset",
            unit: ParameterUnit::Db,
            display_unit: "dB",
            minimum: FADER_MINIMUM_DB as f32,
            maximum: FADER_MAXIMUM_DB as f32,
            taper: Taper::Decibel,
            lattice: ParameterLattice::arithmetic(0.1, 1),
            motion: Motion::Ramp(LiveRampRow::VcaOffset),
            ..VCA_BASE
        },
        read: |vca, side| side.pick(vca.fader.left_db, vca.fader.right_db),
        write: |vca, side, value| {
            *side.pick(&mut vca.fader.left_db, &mut vca.fader.right_db) = value;
        },
    },
    VcaRow {
        id: MUTE,
        descriptor: ControlDescriptor {
            name: "mute",
            display_name: "VCA Mute",
            kind: ControlKind::Toggle,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            motion: Motion::Ramp(LiveRampRow::VcaMute),
            ..VCA_BASE
        },
        read: |vca, side| toggle(side.pick(vca.fader.left_mute, vca.fader.right_mute)),
        write: |vca, side, value| {
            *side.pick(&mut vca.fader.left_mute, &mut vca.fader.right_mute) = is_on(value);
        },
    },
];

/// The stored automation entries that drive one VCA's rows, per lane (`[left, right]`, #1655):
/// `offset` the `offset_db` entries ([`crate::vca_offset_automation`], or
/// [`crate::ModelView::vca_automation`]), `muted` whether a `mute` entry drives the lane.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct VcaEntries<'a> {
    pub(crate) offset: [Option<&'a Automation>; 2],
    pub(crate) muted: [bool; 2],
}

/// The committed value of `row` on `side`: the row's model field, except that an automated
/// `offset_db` lane reads its entry's hand-move offset (#1655 D5; the static offset is the lane's
/// fallback, not heard).
fn value(row: &VcaRow, vca: &Vca, side: Side, offset: [Option<&Automation>; 2]) -> f32 {
    match side.pick(offset[0], offset[1]) {
        Some(entry) if row.id == OFFSET_DB => {
            side.pick(entry.offset.left_db, entry.offset.right_db)
        }
        _ => (row.read)(vca, side),
    }
}

/// Append every control row of one VCA. A lane an entry drives lists `automated = true` (#1650 D4
/// on the VCA): an automated `offset_db` lane lists its entry's offset as its value, an automated
/// `mute` lane its static (hand) mute, which render ORs with the curve (D5).
pub(crate) fn push_rows(out: &mut Vec<ControlRow>, vca: &Vca, entries: VcaEntries<'_>) {
    let owner = Owner::Vca(vca.id.clone());
    for row in &ROWS {
        push_control_automated(
            out,
            &owner,
            &Section::Own,
            row.id,
            &row.descriptor,
            |side| value(row, vca, side, entries.offset),
            |side| match row.id {
                OFFSET_DB => side.pick(entries.offset[0], entries.offset[1]).is_some(),
                _ => side.pick(entries.muted[0], entries.muted[1]),
            },
        );
    }
}

/// Find one row of VCA `id` by its control ID, as [`push_rows`] would list it.
pub(crate) fn resolve(
    view: ModelView<'_>,
    id: &StableId,
    control: u32,
) -> Result<Resolved, ControlError> {
    let vca = view
        .structure()
        .vcas
        .iter()
        .find(|vca| vca.id == *id)
        .ok_or(ControlError::AddressUnknown)?;
    let row = ROWS
        .iter()
        .find(|row| row.id == control)
        .ok_or(ControlError::AddressUnknown)?;
    let offset = if control == OFFSET_DB {
        view.vca_automation(id)
    } else {
        [None; 2]
    };
    Ok(Resolved::stored(row.descriptor, |side| {
        value(row, vca, side, offset)
    }))
}

/// Write `value` as the hand-move offset of each side in `sides` of VCA `vca`'s `offset_db` that a
/// rendered entry of `model` drives (#1655 D5), and return the sides no entry drives, which keep
/// the static offset write. The caller has run [`crate::check`] with the row's own domain,
/// `[-144, 24]` dB, which is the offset domain.
pub(crate) fn write_offset(
    model: &mut SessionModel,
    vca: &StableId,
    sides: &[Side],
    value: f32,
) -> [bool; 2] {
    let entries = if model.automation.is_empty() {
        [None; 2]
    } else {
        crate::automation::vca_offset_automation(model, vca)
    };
    let ids = entries.map(|entry| entry.map(|entry| entry.id.clone()));
    let mut static_sides = [false; 2];
    for &side in sides {
        let Some(id) = side.pick(&ids[0], &ids[1]) else {
            static_sides[side.pick(0, 1)] = true;
            continue;
        };
        if let Some(entry) = model.automation.iter_mut().find(|entry| entry.id == *id) {
            *side.pick(&mut entry.offset.left_db, &mut entry.offset.right_db) = value;
        }
    }
    static_sides
}

/// Write one of a VCA's rows on `side`.
pub(crate) fn write(
    vca: &mut Vca,
    control: u32,
    side: Side,
    value: f32,
) -> Result<(), ControlError> {
    let row = ROWS
        .iter()
        .find(|row| row.id == control)
        .ok_or(ControlError::AddressUnknown)?;
    (row.write)(vca, side, value);
    Ok(())
}
