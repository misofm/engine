//! The row tables as data, for a generator (decision 17, slice S2b): the SDK's control types are
//! written from these accessors and from nothing else, so a row added to a table reaches the SDK
//! when the metadata is regenerated.
//!
//! A [`TableRow`] is a row without a session. Its descriptor is the one [`crate::list`] gives for
//! an active row, with the two things that depend on something other than the row spelled out:
//! the rate (`hpf_hz` and `lpf_hz`, through [`TableRow::at_rate`]) and the owner's shape (a route
//! row's class and applicability by destination, [`TableRow::into_output`]; whether a submix has a
//! strip's own row, [`TableRow::on_submix`]).

use builtins::BuiltinParameterDescriptor;
use effect_compiler::launch_native_effect_registry;

use crate::descriptor::ControlDescriptor;
use crate::{ControlClass, builtin_rows, effect_rows, route, session_rows, strip, vca};

/// A row table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowSource {
    /// A strip's own rows (`matrix_mode`, `solo`, the pan window and the effective values).
    StripOwn,
    /// A strip's builtin rows (`BUILTIN_PARAMETER_DESCRIPTORS`).
    Builtins,
    /// A route's own rows.
    RouteOwn,
    /// A VCA's own rows.
    VcaOwn,
    /// The session's own rows.
    SessionOwn,
}

/// What a route row is on a route into the output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutputCase {
    /// The row's class on a route into the output.
    pub class: ControlClass,
    /// Whether the row applies to a route into the output.
    pub active: bool,
}

/// One row of a table, without a session.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableRow {
    /// The row ID in its table: a builtin ID, an own-row ID, or in an effect section `0` (bypass),
    /// the parameter ID or [`crate::LINK_MODE_CONTROL`].
    pub control: u32,
    /// The descriptor of an active row, at the first launch rate for a builtin row.
    pub descriptor: ControlDescriptor,
    /// `false` for a strip row that a submix does not have (solo); `true` for every other row.
    pub on_submix: bool,
    /// A route row's case on a route into the output; `descriptor` is its case on a route into a
    /// submix. `None` for a row of any other table.
    pub into_output: Option<OutputCase>,
    builtin: Option<&'static BuiltinParameterDescriptor>,
}

impl TableRow {
    const fn plain(control: u32, descriptor: ControlDescriptor) -> Self {
        Self {
            control,
            descriptor,
            on_submix: true,
            into_output: None,
            builtin: None,
        }
    }

    /// The descriptor at `sample_rate`: only a builtin filter's maximum depends on it.
    #[must_use]
    pub fn at_rate(&self, sample_rate: u32) -> ControlDescriptor {
        self.builtin.map_or(self.descriptor, |row| {
            builtin_rows::shaped_descriptor(row, sample_rate, true)
        })
    }
}

/// The rows of one table, in table order.
#[must_use]
pub fn table_rows(source: RowSource) -> Vec<TableRow> {
    match source {
        RowSource::StripOwn => strip::ROWS
            .iter()
            .map(|row| TableRow {
                on_submix: row.on_submix,
                ..TableRow::plain(row.id, row.descriptor)
            })
            .collect(),
        RowSource::Builtins => builtin_rows::rows()
            .iter()
            .map(|row| TableRow {
                builtin: Some(row),
                ..TableRow::plain(
                    row.id,
                    builtin_rows::shaped_descriptor(row, FIRST_RATE, true),
                )
            })
            .collect(),
        RowSource::RouteOwn => route::ROWS
            .iter()
            .map(|row| TableRow {
                into_output: Some(OutputCase {
                    class: row.output_class,
                    active: row.output_active,
                }),
                ..TableRow::plain(row.id, row.descriptor)
            })
            .collect(),
        RowSource::VcaOwn => vca::ROWS
            .iter()
            .map(|row| TableRow::plain(row.id, row.descriptor))
            .collect(),
        RowSource::SessionOwn => session_rows::ROWS
            .iter()
            .map(|row| TableRow::plain(row.id, row.descriptor))
            .collect(),
    }
}

/// The rate whose descriptor a builtin [`TableRow::descriptor`] holds.
const FIRST_RATE: u32 = 44_100;

/// The rows of one native effect: its bypass, each parameter in descriptor order, and the link
/// mode of an insert that supports more than one.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectTableRows {
    /// The bypass row (control [`crate::BYPASS_CONTROL`]).
    pub bypass: TableRow,
    /// One row per parameter of the effect's descriptor.
    pub parameters: Vec<TableRow>,
    /// The link-mode row (control [`crate::LINK_MODE_CONTROL`]), or `None` for an effect with one
    /// link mode. A console slot has no link-mode row, whatever the effect.
    pub link_mode: Option<TableRow>,
}

/// The rows of the launch effect `effect_id`, or `None` for an ID the launch registry does not
/// carry.
#[must_use]
pub fn effect_table_rows(effect_id: &str) -> Option<EffectTableRows> {
    let registry = launch_native_effect_registry().ok()?;
    let descriptor = registry.get_ascii(effect_id)?.descriptor();
    Some(EffectTableRows {
        bypass: TableRow::plain(
            effect_rows::BYPASS_CONTROL,
            effect_rows::bypass_descriptor(),
        ),
        parameters: descriptor
            .parameters
            .iter()
            .map(|parameter| {
                TableRow::plain(parameter.id.0, effect_rows::parameter_descriptor(parameter))
            })
            .collect(),
        link_mode: effect_rows::link_choices(descriptor).map(|choices| {
            TableRow::plain(
                effect_rows::LINK_MODE_CONTROL,
                effect_rows::link_mode_descriptor(effect_id, choices),
            )
        }),
    })
}
