//! The control core (decision 17): every value of a session is one kind of control, listed,
//! validated and written here and nowhere else.
//!
//! A control has one stable [`ControlAddress`] built from stable IDs, one [`ControlDescriptor`]
//! that says what the value is (unit, range, taper, choices, ramp, class, render family), and one
//! validator, [`check`]. [`list`] yields every control of a session with its descriptor and its
//! committed value; [`get`] and [`set`] read and write one control by address.
//!
//! The rows come from tables and nowhere else:
//!
//! - a strip's builtin rows are `builtins::BUILTIN_PARAMETER_DESCRIPTORS`, bound to the model by
//!   one `match` arm per builtin ID in `builtin_rows`;
//! - a native effect's rows (its bypass, each parameter of its contract descriptor, and an
//!   insert's link mode) are derived from the launch registry's `ParameterDescriptor` in
//!   `effect_rows`, with no table of their own;
//! - a strip's own rows, a route's, a VCA's and the session's are the tables in `strip`,
//!   `route`, `vca` and `session_rows`; each row carries its model accessors.
//!
//! To add a control, add a row to its table (and the model field it reads); see decision 17,
//! "How to add a control".
//!
//! This crate depends on `session`, `builtins`, `effect-contract` and `effect-compiler` (for the
//! launch effect registry) and on nothing above them.
//! Lowering a changed value into a render record is `host-core`'s job, keyed by [`Family`].

mod automation;
mod builtin_rows;
mod descriptor;
mod diff;
mod effect_rows;
mod effective;
mod error;
mod lane;
mod live_lanes;
mod live_ramps;
mod mask;
mod observation;
mod owner_edits;
mod registry;
mod route;
mod session_rows;
mod strip;
mod tables;
mod vca;
mod view;

pub use automation::{ControlDiagnostic, automation_diagnostics};
pub use descriptor::{Choice, ControlDescriptor, ControlScope, DomainRule, Lanes, Motion, check};
pub use diff::{ChangedControl, changed_controls, changed_rows, writes_only_its_row};
pub use effect_contract::control::{
    ControlClass, ControlKind, Family, LiveRampRow, RebuildReason, Taper,
};
pub use effect_rows::{BYPASS_CONTROL, LINK_MODE_CONTROL, check_expected_effect};
pub use effective::{
    Effective, EffectiveInputs, EffectiveRoute, EffectiveRouteInput, EffectiveShapeError,
    EffectiveSource, EffectiveStrip, LocalEffective, LocalInputs, ModelInputs, MonitorChange,
    MonitorOverlay, MonitorWrite, any_solo, any_solo_in, effective, effective_follow,
    effective_gate, effective_into, effective_strip, solo_term,
};
pub use error::ControlError;
pub use live_lanes::{LiveLanes, live_lanes};
pub use live_ramps::LiveRamps;
pub use mask::mask_values;
pub use observation::{
    ObservationBinding, ObservationBindings, ObservationChange, TapKey, TapSection,
};
pub use owner_edits::{OwnerEditError, OwnerEdits};
pub use registry::{
    ControlRow, check_monitor, derived_rows_in, derived_rows_with, get, list, owner_count,
    owner_index, owner_rows_in, owner_rows_with, row_descriptor, row_motion, row_scope, set,
    strip_index,
};
pub use session::{ControlAddress, Lane, Owner, OwnerKind, Section, SectionKind};
pub use tables::{EffectTableRows, OutputCase, RowSource, TableRow, effect_table_rows, table_rows};
pub use view::ModelView;
