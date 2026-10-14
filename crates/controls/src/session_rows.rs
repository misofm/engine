//! The session's own rows: the live ramp lengths (`control_smoothing`, #1054).
//!
//! They are settings, not render values: they choose how long other changes ramp, and no render
//! record carries them. A session that says `default` reads as the researched defaults; the first
//! write makes the lengths explicit.

use effect_contract::{ParameterLattice, ParameterUnit};
use session::{
    CONTROL_SMOOTHING_DEFAULT, CONTROL_SMOOTHING_MS_MAXIMUM, ControlSmoothing, SessionModel,
};

use crate::Family;
use crate::descriptor::{BASE, ControlDescriptor};
use crate::error::ControlError;
use crate::registry::{ControlRow, Resolved, push_control};
use session::{Owner, Section};

const FADER_MS: u32 = 1;
const MUTE_MS: u32 = 2;
const PAN_MS: u32 = 3;

/// One row of the session table.
pub(crate) struct SessionRow {
    pub(crate) id: u32,
    pub(crate) descriptor: ControlDescriptor,
    /// The committed length, in milliseconds.
    read: fn(&ControlSmoothing) -> f32,
    /// Replace the length, in milliseconds.
    write: fn(&mut ControlSmoothing, f32),
}

/// What a session row starts from: a length in `[0, 1000]` ms.
const MILLISECONDS: ControlDescriptor = ControlDescriptor {
    unit: ParameterUnit::Milliseconds,
    display_unit: "ms",
    maximum: CONTROL_SMOOTHING_MS_MAXIMUM,
    lattice: ParameterLattice::arithmetic(1.0, 0),
    family: Family::Setting,
    ..BASE
};

/// The session rows, in listing order. Never reuse an ID.
pub(crate) static ROWS: [SessionRow; 3] = [
    SessionRow {
        id: FADER_MS,
        descriptor: ControlDescriptor {
            name: "fader_ms",
            display_name: "Fader Ramp",
            default: CONTROL_SMOOTHING_DEFAULT.fader_ms,
            ..MILLISECONDS
        },
        read: |lengths| lengths.fader_ms,
        write: |lengths, value| lengths.fader_ms = value,
    },
    SessionRow {
        id: MUTE_MS,
        descriptor: ControlDescriptor {
            name: "mute_ms",
            display_name: "Mute Ramp",
            default: CONTROL_SMOOTHING_DEFAULT.mute_ms,
            ..MILLISECONDS
        },
        read: |lengths| lengths.mute_ms,
        write: |lengths, value| lengths.mute_ms = value,
    },
    SessionRow {
        id: PAN_MS,
        descriptor: ControlDescriptor {
            name: "pan_ms",
            display_name: "Pan Ramp",
            default: CONTROL_SMOOTHING_DEFAULT.pan_ms,
            ..MILLISECONDS
        },
        read: |lengths| lengths.pan_ms,
        write: |lengths, value| lengths.pan_ms = value,
    },
];

/// Append every control row of the session.
pub(crate) fn push_rows(out: &mut Vec<ControlRow>, model: &SessionModel) {
    let lengths = model.control_smoothing.unwrap_or(CONTROL_SMOOTHING_DEFAULT);
    for row in &ROWS {
        push_control(
            out,
            &Owner::Session,
            &Section::Own,
            row.id,
            &row.descriptor,
            |_| (row.read)(&lengths),
        );
    }
}

/// Find one row of the session by its control ID, as [`push_rows`] would list it.
pub(crate) fn resolve(model: &SessionModel, control: u32) -> Result<Resolved, ControlError> {
    let row = ROWS
        .iter()
        .find(|row| row.id == control)
        .ok_or(ControlError::AddressUnknown)?;
    let lengths = model.control_smoothing.unwrap_or(CONTROL_SMOOTHING_DEFAULT);
    Ok(Resolved::stored(row.descriptor, |_| (row.read)(&lengths)))
}

/// Write one of the session's rows.
pub(crate) fn write(
    model: &mut SessionModel,
    control: u32,
    value: f32,
) -> Result<(), ControlError> {
    let row = ROWS
        .iter()
        .find(|row| row.id == control)
        .ok_or(ControlError::AddressUnknown)?;
    let mut lengths = model.control_smoothing.unwrap_or(CONTROL_SMOOTHING_DEFAULT);
    (row.write)(&mut lengths, value);
    model.control_smoothing = Some(lengths);
    Ok(())
}
