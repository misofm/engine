//! A strip's rows: its own table (`matrix_mode`, solo, the pan window and the derived effective
//! values) and its builtin rows ([`crate::builtin_rows`]).

use effect_contract::{ParameterLattice, ParameterUnit};
use session::{
    DualMonoBuiltins, DualMonoFader, EffectiveStripFader, MatrixOrPan, SessionModel, StableId,
    StripKind, StripRef,
};

use crate::address::{Owner, Section};
use crate::builtin_rows;
use crate::descriptor::{BASE, Choice, ControlDescriptor, Lanes, Motion};
use crate::error::ControlError;
use crate::lane::Side;
use crate::lane::{is_on, toggle};
use crate::registry::{ControlRow, push_control};
use crate::{ControlClass, ControlKind, ControlScope, Family, LiveRampRow, Taper};

/// The own-row ID of `matrix_mode`.
pub(crate) const MATRIX_MODE: u32 = 1;
/// The own-row ID of `solo`.
pub(crate) const SOLO: u32 = 2;
/// The own-row ID of `pan_window_samples`.
pub(crate) const PAN_WINDOW_SAMPLES: u32 = 3;
/// The own-row ID of `effective_fader_db`.
pub(crate) const EFFECTIVE_FADER_DB: u32 = 64;
/// The own-row ID of `effective_mute`.
pub(crate) const EFFECTIVE_MUTE: u32 = 65;

/// The longest pan window a row accepts: 1000 ms at 96 kHz, the longest `pan_ms` the session
/// accepts at the highest launch rate.
const PAN_WINDOW_SAMPLES_MAXIMUM: f32 = 96_000.0;

const MODE_CHOICES: [Choice; 2] = [
    Choice {
        value: 0.0,
        label: "pan",
    },
    Choice {
        value: 1.0,
        label: "matrix",
    },
];

/// How a strip's own row reaches its value.
pub(crate) enum StripValue {
    /// A field of the model.
    Stored {
        /// The committed value.
        read: fn(&StripRef<'_>, Side) -> f32,
        /// The model write, after the value passed [`crate::check`].
        write: fn(&mut StripMut<'_>, Side, f32) -> Result<(), ControlError>,
    },
    /// Computed from other controls.
    Derived {
        /// The value, from the strip's effective fader and mute.
        read: fn(&EffectiveStripFader, Side) -> f32,
    },
    /// Held by the host, not the model: the overlay writes it, and until it exists the row reads
    /// as its default.
    Monitor,
}

/// One row of a strip's own table.
pub(crate) struct StripRow {
    pub(crate) id: u32,
    /// Whether a submix strip has this row. Solo is a track control: a submix is solo-safe.
    pub(crate) on_submix: bool,
    pub(crate) descriptor: ControlDescriptor,
    pub(crate) value: StripValue,
}

/// A strip's own rows, in listing order. Never reuse an ID.
pub(crate) static ROWS: [StripRow; 5] = [
    StripRow {
        id: MATRIX_MODE,
        on_submix: true,
        descriptor: ControlDescriptor {
            name: "matrix_mode",
            display_name: "Pan or matrix",
            kind: ControlKind::Enumeration,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            choices: &MODE_CHOICES,
            motion: Motion::Ramp(LiveRampRow::Pan),
            family: Family::StripMatrix,
            ..BASE
        },
        value: StripValue::Stored {
            read: |strip, _| toggle(matches!(strip.matrix_or_pan, MatrixOrPan::Matrix { .. })),
            write: write_matrix_mode,
        },
    },
    StripRow {
        id: SOLO,
        on_submix: false,
        descriptor: ControlDescriptor {
            name: "solo",
            display_name: "Solo",
            kind: ControlKind::Toggle,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            motion: Motion::Ramp(LiveRampRow::Solo),
            scope: ControlScope::Monitor,
            family: Family::StripLevel,
            ..BASE
        },
        value: StripValue::Monitor,
    },
    StripRow {
        id: PAN_WINDOW_SAMPLES,
        on_submix: true,
        descriptor: ControlDescriptor {
            name: "pan_window_samples",
            display_name: "Pan window",
            unit: ParameterUnit::Samples,
            maximum: PAN_WINDOW_SAMPLES_MAXIMUM,
            lattice: ParameterLattice::arithmetic(1.0, 0),
            ..BASE
        },
        value: StripValue::Stored {
            read: |strip, _| pan_window(strip.matrix_or_pan) as f32,
            write: |strip, _, value| {
                *pan_window_mut(strip.matrix_or_pan) = value as u32;
                Ok(())
            },
        },
    },
    StripRow {
        id: EFFECTIVE_FADER_DB,
        on_submix: true,
        descriptor: ControlDescriptor {
            name: "effective_fader_db",
            display_name: "Effective fader",
            unit: ParameterUnit::Db,
            lanes: Lanes::PerLane,
            minimum: -144.0,
            maximum: 24.0,
            taper: Taper::Decibel,
            lattice: ParameterLattice::arithmetic(0.1, 1),
            class: ControlClass::Derived,
            family: Family::StripLevel,
            ..BASE
        },
        value: StripValue::Derived {
            read: |effective, side| side.pick(effective.db[0], effective.db[1]),
        },
    },
    StripRow {
        id: EFFECTIVE_MUTE,
        on_submix: true,
        descriptor: ControlDescriptor {
            name: "effective_mute",
            display_name: "Effective mute",
            kind: ControlKind::Toggle,
            lanes: Lanes::PerLane,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            class: ControlClass::Derived,
            family: Family::StripLevel,
            ..BASE
        },
        value: StripValue::Derived {
            read: |effective, side| toggle(side.pick(effective.mute[0], effective.mute[1])),
        },
    },
];

/// The `smoothing_samples` of either shape.
fn pan_window(shape: &MatrixOrPan) -> u32 {
    match shape {
        MatrixOrPan::Pan {
            smoothing_samples, ..
        }
        | MatrixOrPan::Matrix {
            smoothing_samples, ..
        } => *smoothing_samples,
    }
}

fn pan_window_mut(shape: &mut MatrixOrPan) -> &mut u32 {
    match shape {
        MatrixOrPan::Pan {
            smoothing_samples, ..
        }
        | MatrixOrPan::Matrix {
            smoothing_samples, ..
        } => smoothing_samples,
    }
}

/// Switch between pan and matrix (decision 17, C12). To `matrix` the matrix is seeded with the
/// pan's exact 2x2 coefficients, so nothing moves; to `pan` each lane's pan is its default. The
/// pan window carries over, and a switch to the present mode changes nothing.
fn write_matrix_mode(
    strip: &mut StripMut<'_>,
    _side: Side,
    value: f32,
) -> Result<(), ControlError> {
    let smoothing_samples = pan_window(strip.matrix_or_pan);
    *strip.matrix_or_pan = match (&*strip.matrix_or_pan, is_on(value)) {
        (MatrixOrPan::Pan { left, right, .. }, true) => {
            let [ll, lr, rl, rr] = builtin_rows::matrix_of_pan(*left, *right)?;
            MatrixOrPan::Matrix {
                ll,
                lr,
                rl,
                rr,
                smoothing_samples,
            }
        }
        (MatrixOrPan::Matrix { .. }, false) => {
            let pan = builtin_rows::default_of(builtin_rows::PAN);
            MatrixOrPan::Pan {
                left: pan,
                right: pan,
                smoothing_samples,
            }
        }
        _ => return Ok(()),
    };
    Ok(())
}

/// The mutable parts of one strip that a control write reaches.
pub(crate) struct StripMut<'a> {
    pub(crate) builtins: &'a mut DualMonoBuiltins,
    pub(crate) fader: &'a mut DualMonoFader,
    pub(crate) matrix_or_pan: &'a mut MatrixOrPan,
}

/// The mutable strip `id` (a track or a submix).
pub(crate) fn strip_mut<'a>(model: &'a mut SessionModel, id: &StableId) -> Option<StripMut<'a>> {
    if let Some(track) = model.tracks.iter_mut().find(|track| &track.id == id) {
        return Some(StripMut {
            builtins: &mut track.builtins,
            fader: &mut track.fader,
            matrix_or_pan: &mut track.matrix_or_pan,
        });
    }
    let submix = model.submixes.iter_mut().find(|submix| &submix.id == id)?;
    Some(StripMut {
        builtins: &mut submix.builtins,
        fader: &mut submix.fader,
        matrix_or_pan: &mut submix.matrix_or_pan,
    })
}

/// Append every control row of one strip, in listing order: own, then builtins. The console and
/// insert sections are empty until effect rows exist (decision 17, S2).
pub(crate) fn push_rows(
    out: &mut Vec<ControlRow>,
    strip: &StripRef<'_>,
    effective: &EffectiveStripFader,
    sample_rate: u32,
) {
    let owner = Owner::Strip(strip.id.clone());
    let on_submix = matches!(strip.kind, StripKind::Submix(_));
    for row in ROWS.iter().filter(|row| row.on_submix || !on_submix) {
        push_control(
            out,
            &owner,
            &Section::Own,
            row.id,
            &row.descriptor,
            |side| match &row.value {
                StripValue::Stored { read, .. } => read(strip, side),
                StripValue::Derived { read } => read(effective, side),
                StripValue::Monitor => row.descriptor.default,
            },
        );
    }
    for row in builtin_rows::rows() {
        let descriptor = builtin_rows::descriptor(row, sample_rate, strip.matrix_or_pan);
        push_control(
            out,
            &owner,
            &Section::Builtins,
            row.id,
            &descriptor,
            |side| builtin_rows::read(strip, row, side),
        );
    }
}

/// Write one of a strip's own rows.
pub(crate) fn write_own(
    strip: &mut StripMut<'_>,
    control: u32,
    side: Side,
    value: f32,
) -> Result<(), ControlError> {
    match ROWS.iter().find(|row| row.id == control) {
        Some(StripRow {
            value: StripValue::Stored { write, .. },
            ..
        }) => write(strip, side, value),
        Some(StripRow {
            value: StripValue::Derived { .. },
            ..
        }) => Err(ControlError::ReadOnly),
        Some(StripRow {
            value: StripValue::Monitor,
            ..
        }) => Err(ControlError::Monitor),
        None => Err(ControlError::AddressUnknown),
    }
}
