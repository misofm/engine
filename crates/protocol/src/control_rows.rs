//! Name-to-address lookup for the control rows a `SET_CONTROL` edit writes (decision 17, C5,
//! #1561). The row tables are the control core's; this resolves a row's name to its ID there, so a
//! caller that writes a value by name holds no second copy of an ID.

use controls::{
    BYPASS_CONTROL, ControlAddress, LINK_MODE_CONTROL, Lane, Owner, RowSource, Section, table_rows,
};
use session::{DualMonoBuiltins, DualMonoFader, MatrixOrPan, StableId};

use crate::SessionEdit;

/// The address of the row of `owner`'s `section` called `name`, written on `lane`, or `None` when
/// the owner and section have no such row.
///
/// `name` is the row's name in its table (`fader_db`, `trim_db`, `gain_db`, `offset_db`,
/// `fader_ms`, ...). In an insert or console section `bypass` and `link_mode` name the effect's
/// two fixed rows; an effect parameter has no name here, its row is its parameter ID, so a caller builds that
/// address from [`ControlAddress`] directly.
#[must_use]
pub fn row_address(
    owner: Owner,
    section: Section,
    name: &str,
    lane: Lane,
) -> Option<ControlAddress> {
    let control = match (&owner, &section) {
        (Owner::Strip(_), Section::Builtins) => table_id(RowSource::Builtins, name)?,
        (Owner::Strip(_), Section::Own) => table_id(RowSource::StripOwn, name)?,
        (Owner::Route(_), Section::Own) => table_id(RowSource::RouteOwn, name)?,
        (Owner::Vca(_), Section::Own) => table_id(RowSource::VcaOwn, name)?,
        (Owner::Session, Section::Own) => table_id(RowSource::SessionOwn, name)?,
        (Owner::Strip(_), Section::Inserts(_) | Section::Console(_)) => match name {
            "bypass" => BYPASS_CONTROL,
            "link_mode" => LINK_MODE_CONTROL,
            _ => return None,
        },
        _ => return None,
    };
    Some(ControlAddress {
        owner,
        section,
        control,
        lane,
    })
}

fn table_id(source: RowSource, name: &str) -> Option<u32> {
    table_rows(source)
        .into_iter()
        .find(|row| row.descriptor.name == name)
        .map(|row| row.control)
}

/// The `SET_CONTROL` edits that give strip `strip` the whole fader `fader`: the fader dB and the
/// mute of each lane, four edits with no ramp length of their own.
///
/// A caller that holds a whole `DualMonoFader` (a snapshot, a preset) writes it as rows; this is
/// the one composition of those rows, so it holds no ID of its own.
///
/// # Panics
///
/// Never for a built control core: the rows it names are in the builtin table.
#[must_use]
pub fn fader_edits(strip: &StableId, fader: &DualMonoFader) -> Vec<SessionEdit> {
    let row = |name, lane, value| builtin_edit(strip, name, lane, value);
    vec![
        row("fader_db", Lane::Left, fader.left_db),
        row("fader_db", Lane::Right, fader.right_db),
        row("mute", Lane::Left, toggle(fader.left_mute)),
        row("mute", Lane::Right, toggle(fader.right_mute)),
    ]
}

/// The `SET_CONTROL` edits that give strip `strip` the whole builtin section `builtins`: the
/// polarity, trim, high-pass and low-pass cutoffs and the delay of each lane, ten edits with no
/// ramp length of their own.
///
/// # Panics
///
/// Never for a built control core: the rows it names are in the builtin table.
#[must_use]
pub fn builtins_edits(strip: &StableId, builtins: &DualMonoBuiltins) -> Vec<SessionEdit> {
    let mut edits = Vec::with_capacity(10);
    for (lane, side) in [(Lane::Left, &builtins.left), (Lane::Right, &builtins.right)] {
        let row = |name, value| builtin_edit(strip, name, lane, value);
        edits.extend([
            row("polarity_invert", toggle(side.polarity_invert)),
            row("trim_db", side.trim_db),
            row("hpf_hz", side.hpf_hz),
            row("lpf_hz", side.lpf_hz),
            row("delay_samples", side.delay_samples as f32),
        ]);
    }
    edits
}

/// The `SET_CONTROL` edits that give strip `strip` the pan or matrix `shape`: the `matrix_mode`
/// row first (a no-op when the strip is already in that mode), then the rows the mode uses and the
/// pan window. No edit carries a ramp length of its own.
///
/// # Panics
///
/// Never for a built control core: the rows it names are in the builtin and strip tables.
#[must_use]
pub fn matrix_or_pan_edits(strip: &StableId, shape: &MatrixOrPan) -> Vec<SessionEdit> {
    let own = |name, value| {
        SessionEdit::set_control(
            row_address(Owner::Strip(strip.clone()), Section::Own, name, Lane::Both)
                .expect("the strip table has the row"),
            value,
            None,
        )
    };
    let builtin = |name, lane, value| builtin_edit(strip, name, lane, value);
    match shape {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => vec![
            own("matrix_mode", 0.0),
            builtin("pan", Lane::Left, *left),
            builtin("pan", Lane::Right, *right),
            own("pan_window_samples", *smoothing_samples as f32),
        ],
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => vec![
            own("matrix_mode", 1.0),
            builtin("matrix_ll", Lane::Both, *ll),
            builtin("matrix_lr", Lane::Both, *lr),
            builtin("matrix_rl", Lane::Both, *rl),
            builtin("matrix_rr", Lane::Both, *rr),
            own("pan_window_samples", *smoothing_samples as f32),
        ],
    }
}

fn builtin_edit(strip: &StableId, name: &str, lane: Lane, value: f32) -> SessionEdit {
    SessionEdit::set_control(
        row_address(Owner::Strip(strip.clone()), Section::Builtins, name, lane)
            .expect("the builtin table has the row"),
        value,
        None,
    )
}

fn toggle(on: bool) -> f32 {
    f32::from(u8::from(on))
}
