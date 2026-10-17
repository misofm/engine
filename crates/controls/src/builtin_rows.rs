//! A strip's builtin rows: `BUILTIN_PARAMETER_DESCRIPTORS` bound to the session model.
//!
//! The descriptor table says what each value is. This module is the only place that says which
//! model field each builtin ID reads and writes: one `match` arm per ID in [`read`] and in
//! [`write`].
//!
//! The binding has model scope for one row (#1650 D3, root's ruling R17-1 = A): on a lane that a
//! rendered stored automation drives, `fader_db` reads and writes the driving entry's hand-move
//! offset ([`FaderOffsets`], [`write_fader_offset`]), not the strip's static fader, which is then
//! the lane's fallback and no control value.

use builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS, BuiltinParameterDescriptor, BuiltinParameterDomain,
    BuiltinParameterScope, builtin_filter_cutoff_maximum_hz, builtin_parameter_unit, pan_matrix,
};
use session::{Automation, ChannelBuiltins, Lane, MatrixOrPan, StableId, StripRef};

use crate::ControlScope;
use crate::descriptor::{AutomationRule, ControlDescriptor, DomainRule, Lanes, Motion};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::strip::StripMut;

// The builtin IDs. `ids_match_the_descriptor_table` pins each to its row's name.
const POLARITY_INVERT: u32 = 1;
const TRIM_DB: u32 = 2;
pub(crate) const HPF_HZ: u32 = 3;
pub(crate) const LPF_HZ: u32 = 4;
pub(crate) const FADER_DB: u32 = 5;
pub(crate) const MUTE: u32 = 6;
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
        domain: DomainRule::Builtin {
            domain: row.domain,
            sample_rate,
        },
        automation: AutomationRule::Computed,
    }
}

/// The hand-move offsets of a strip's fader lanes (#1650 D3): per lane (`[left, right]`), the
/// offset of the rendered stored automation entry that drives the lane, or `None` for a lane that
/// no rendered entry drives. The `fader_db` row reads a lane's offset where it is `Some`.
pub(crate) type FaderOffsets = [Option<f32>; 2];

/// The [`FaderOffsets`] of the entries that drive a strip's fader lanes
/// ([`crate::fader_automation`], or [`crate::ModelView::fader_automation`]).
pub(crate) fn fader_offsets(entries: [Option<&Automation>; 2]) -> FaderOffsets {
    [
        entries[0].map(|entry| entry.offset.left_db),
        entries[1].map(|entry| entry.offset.right_db),
    ]
}

/// The rendered entries of `model` that drive strip `strip`'s fader lanes:
/// [`crate::fader_automation`], with an early out on an empty table.
fn fader_entries<'a>(
    model: &'a session::SessionModel,
    strip: &StableId,
) -> [Option<&'a Automation>; 2] {
    if model.automation.is_empty() {
        return [None, None];
    }
    crate::automation::fader_automation(model, strip)
}

/// The [`FaderOffsets`] of strip `strip` in a whole model.
pub(crate) fn model_fader_offsets(model: &session::SessionModel, strip: &StableId) -> FaderOffsets {
    fader_offsets(fader_entries(model, strip))
}

/// Write `value` as the hand-move offset of each side in `sides` of strip `strip`'s fader that a
/// rendered entry of `model` drives (#1650 D3), and return the sides no entry drives, which keep
/// the static fader write. The caller has run [`crate::check`] with the row's own domain,
/// `[-144, 24]` dB, which is the offset domain (#1650 D2).
pub(crate) fn write_fader_offset(
    model: &mut session::SessionModel,
    strip: &StableId,
    sides: &[Side],
    value: f32,
) -> [bool; 2] {
    let ids = fader_entries(model, strip).map(|entry| entry.map(|entry| entry.id.clone()));
    let mut static_sides = [false; 2];
    for &side in sides {
        let lane = side.pick(0, 1);
        let Some(id) = &ids[lane] else {
            static_sides[lane] = true;
            continue;
        };
        if let Some(entry) = model.automation.iter_mut().find(|entry| entry.id == *id) {
            debug_assert!(matches!(
                (entry.target.lane, side),
                (Lane::Both, _) | (Lane::Left, Side::Left) | (Lane::Right, Side::Right)
            ));
            *side.pick(&mut entry.offset.left_db, &mut entry.offset.right_db) = value;
        }
    }
    static_sides
}

fn channel<'a>(strip: &StripRef<'a>, side: Side) -> &'a ChannelBuiltins {
    side.pick(&strip.builtins.left, &strip.builtins.right)
}

/// The committed value of builtin `row` on `side`. A row that the strip's shape does not use
/// reads as its default. `offsets` are the strip's [`FaderOffsets`]: an automated fader lane reads
/// its entry's hand-move offset (#1650 D3).
pub(crate) fn read(
    strip: &StripRef<'_>,
    row: &BuiltinParameterDescriptor,
    side: Side,
    offsets: FaderOffsets,
) -> f32 {
    let lane = channel(strip, side);
    let fader = strip.fader;
    match row.id {
        POLARITY_INVERT => toggle(lane.polarity_invert),
        TRIM_DB => lane.trim_db,
        HPF_HZ => lane.hpf_hz,
        LPF_HZ => lane.lpf_hz,
        FADER_DB => side
            .pick(offsets[0], offsets[1])
            .unwrap_or_else(|| side.pick(fader.left_db, fader.right_db)),
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
/// the value is in domain and the row is active. A `fader_db` lane that a rendered entry drives is
/// written by [`write_fader_offset`], never here.
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
