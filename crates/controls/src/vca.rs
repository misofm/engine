//! A VCA group's rows (decision 17, C3). A VCA has no audio path; its offset and mute reach render
//! through each member's effective fader and mute.

use effect_contract::{ParameterLattice, ParameterUnit};
use session::Vca;

use crate::address::{Owner, Section};
use crate::descriptor::{
    BASE, ControlDescriptor, EFFECTIVE_SLICE, Lanes, Motion, ruled_live_not_built,
};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::registry::{ControlRow, push_control};
use crate::{ControlKind, Family, LiveRampRow, Taper};

const OFFSET_DB: u32 = 1;
const MUTE: u32 = 2;

const REOPEN: &str = "The effective-value slice (decision 17, S7a) builds the live path and \
     flips this row to live.";

/// One row of the VCA table.
pub(crate) struct VcaRow {
    pub(crate) id: u32,
    pub(crate) descriptor: ControlDescriptor,
    /// The committed value of one lane.
    read: fn(&Vca, Side) -> f32,
    /// The model write, after the value passed [`crate::check`].
    write: fn(&mut Vca, Side, f32),
}

/// What a VCA row starts from. A VCA's fader is stored per lane, so its rows are per lane.
const VCA_BASE: ControlDescriptor = ControlDescriptor {
    lanes: Lanes::PerLane,
    class: ruled_live_not_built(EFFECTIVE_SLICE, REOPEN),
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
            minimum: -144.0,
            maximum: 24.0,
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

/// Append every control row of one VCA.
pub(crate) fn push_rows(out: &mut Vec<ControlRow>, vca: &Vca) {
    let owner = Owner::Vca(vca.id.clone());
    for row in &ROWS {
        push_control(
            out,
            &owner,
            &Section::Own,
            row.id,
            &row.descriptor,
            |side| (row.read)(vca, side),
        );
    }
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
