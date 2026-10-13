//! A strip's builtin rows: `BUILTIN_PARAMETER_DESCRIPTORS` bound to the session model.
//!
//! The descriptor table says what each value is. This module is the only place that says which
//! model field each builtin ID reads and writes: one `match` arm per ID in [`read`] and in
//! [`write`].

use builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS, BuiltinParameterDescriptor, BuiltinParameterDomain,
    BuiltinParameterScope, builtin_filter_cutoff_maximum_hz, builtin_parameter_unit, pan_matrix,
};
use session::{ChannelBuiltins, MatrixOrPan, StripRef};

use crate::ControlScope;
use crate::descriptor::{ControlDescriptor, Lanes, Motion};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::strip::StripMut;

// The builtin IDs. `ids_match_the_descriptor_table` pins each to its row's name.
const POLARITY_INVERT: u32 = 1;
const TRIM_DB: u32 = 2;
const HPF_HZ: u32 = 3;
const LPF_HZ: u32 = 4;
const FADER_DB: u32 = 5;
const MUTE: u32 = 6;
const MATRIX_LL: u32 = 7;
const MATRIX_LR: u32 = 8;
const MATRIX_RL: u32 = 9;
const MATRIX_RR: u32 = 10;
const DELAY_SAMPLES: u32 = 11;
pub(crate) const PAN: u32 = 12;

/// The builtin rows, in table order.
pub(crate) fn rows() -> &'static [BuiltinParameterDescriptor] {
    &BUILTIN_PARAMETER_DESCRIPTORS
}

/// The row of one builtin ID.
pub(crate) fn row(id: u32) -> Option<&'static BuiltinParameterDescriptor> {
    rows().iter().find(|row| row.id == id)
}

/// The default of builtin `id`, from the table.
pub(crate) fn default_of(id: u32) -> f32 {
    row(id).map_or(0.0, |row| row.default)
}

/// Whether a strip in this shape uses the row: the pan rows in pan mode, the matrix rows in
/// matrix mode, every other row always.
pub(crate) fn is_active(id: u32, shape: &MatrixOrPan) -> bool {
    match id {
        PAN => matches!(shape, MatrixOrPan::Pan { .. }),
        MATRIX_LL | MATRIX_LR | MATRIX_RL | MATRIX_RR => {
            matches!(shape, MatrixOrPan::Matrix { .. })
        }
        _ => true,
    }
}

/// The control descriptor of a builtin row at a sample rate, for a strip in `shape`.
pub(crate) fn descriptor(
    row: &BuiltinParameterDescriptor,
    sample_rate: u32,
    shape: &MatrixOrPan,
) -> ControlDescriptor {
    shaped_descriptor(row, sample_rate, is_active(row.id, shape))
}

/// The control descriptor of a builtin row at a sample rate, with `active` given: the row of a
/// host that holds no model (it checks the strip's shape itself).
pub(crate) fn shaped_descriptor(
    row: &BuiltinParameterDescriptor,
    sample_rate: u32,
    active: bool,
) -> ControlDescriptor {
    let (minimum, maximum) = match row.domain {
        BuiltinParameterDomain::BooleanExact => (0.0, 1.0),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => (minimum, maximum),
        // A rate with no cutoff bound has no enabled cutoff: only the disabled value is in domain.
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { minimum_hz, .. } => (
            minimum_hz,
            builtin_filter_cutoff_maximum_hz(sample_rate).unwrap_or(0.0),
        ),
    };
    ControlDescriptor {
        name: row.name,
        display_name: row.display_name,
        display_unit: row.display_unit,
        unit: builtin_parameter_unit(row),
        kind: row.kind,
        lanes: match row.scope {
            BuiltinParameterScope::PerLane => Lanes::PerLane,
            BuiltinParameterScope::MatrixShared => Lanes::Shared,
        },
        minimum,
        maximum,
        disabled_value: row.disabled_value,
        default: row.default,
        taper: row.taper,
        lattice: row.lattice,
        choices: &[],
        motion: row.ramp.map_or(Motion::Fixed, Motion::Ramp),
        class: row.class,
        scope: ControlScope::Model,
        family: row.family,
        active,
    }
}

fn channel<'a>(strip: &StripRef<'a>, side: Side) -> &'a ChannelBuiltins {
    side.pick(&strip.builtins.left, &strip.builtins.right)
}

/// The committed value of builtin `row` on `side`. A row that the strip's shape does not use
/// reads as its default.
pub(crate) fn read(strip: &StripRef<'_>, row: &BuiltinParameterDescriptor, side: Side) -> f32 {
    let lane = channel(strip, side);
    let fader = strip.fader;
    match row.id {
        POLARITY_INVERT => toggle(lane.polarity_invert),
        TRIM_DB => lane.trim_db,
        HPF_HZ => lane.hpf_hz,
        LPF_HZ => lane.lpf_hz,
        FADER_DB => side.pick(fader.left_db, fader.right_db),
        MUTE => toggle(side.pick(fader.left_mute, fader.right_mute)),
        DELAY_SAMPLES => lane.delay_samples as f32,
        PAN => match strip.matrix_or_pan {
            MatrixOrPan::Pan { left, right, .. } => side.pick(*left, *right),
            MatrixOrPan::Matrix { .. } => row.default,
        },
        MATRIX_LL | MATRIX_LR | MATRIX_RL | MATRIX_RR => match strip.matrix_or_pan {
            MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => match row.id {
                MATRIX_LL => *ll,
                MATRIX_LR => *lr,
                MATRIX_RL => *rl,
                _ => *rr,
            },
            MatrixOrPan::Pan { .. } => row.default,
        },
        _ => row.default,
    }
}

/// Write builtin `id` on `side` (ignored by a shared row). The caller has run [`crate::check`], so
/// the value is in domain and the row is active.
///
/// # Errors
///
/// [`ControlError::Inactive`] if the strip is not in the shape the row needs, which a checked
/// write never is.
pub(crate) fn write(
    strip: &mut StripMut<'_>,
    id: u32,
    side: Side,
    value: f32,
) -> Result<(), ControlError> {
    let lane = side.pick(&mut strip.builtins.left, &mut strip.builtins.right);
    match id {
        POLARITY_INVERT => lane.polarity_invert = is_on(value),
        TRIM_DB => lane.trim_db = value,
        HPF_HZ => lane.hpf_hz = value,
        LPF_HZ => lane.lpf_hz = value,
        FADER_DB => *side.pick(&mut strip.fader.left_db, &mut strip.fader.right_db) = value,
        MUTE => *side.pick(&mut strip.fader.left_mute, &mut strip.fader.right_mute) = is_on(value),
        DELAY_SAMPLES => lane.delay_samples = value as u32,
        PAN => match strip.matrix_or_pan {
            MatrixOrPan::Pan { left, right, .. } => *side.pick(left, right) = value,
            MatrixOrPan::Matrix { .. } => return Err(ControlError::Inactive),
        },
        MATRIX_LL | MATRIX_LR | MATRIX_RL | MATRIX_RR => match strip.matrix_or_pan {
            MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => match id {
                MATRIX_LL => *ll = value,
                MATRIX_LR => *lr = value,
                MATRIX_RL => *rl = value,
                _ => *rr = value,
            },
            MatrixOrPan::Pan { .. } => return Err(ControlError::Inactive),
        },
        _ => return Err(ControlError::AddressUnknown),
    }
    Ok(())
}

/// The matrix that a pan pair is, for the switch from pan to matrix mode (decision 17, C12).
pub(crate) fn matrix_of_pan(left: f32, right: f32) -> Result<[f32; 4], ControlError> {
    let matrix = pan_matrix(left, right).map_err(|_| ControlError::Domain)?;
    Ok([matrix.ll, matrix.lr, matrix.rl, matrix.rr])
}
