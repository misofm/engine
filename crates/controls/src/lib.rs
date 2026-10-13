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

mod address;
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
mod registry;
mod route;
mod session_rows;
mod strip;
mod tables;
mod vca;

pub use address::{ControlAddress, Lane, Owner, OwnerKind, Section, SectionKind};
pub use descriptor::{
    Choice, ControlDescriptor, ControlScope, DomainRule, EFFECTIVE_SLICE, Lanes, Motion, check,
};
pub use diff::{ChangedControl, changed_controls};
pub use effect_contract::control::{
    ControlClass, ControlKind, Family, LiveRampRow, RebuildReason, Taper,
};
pub use effect_rows::{BYPASS_CONTROL, LINK_MODE_CONTROL};
pub use effective::{
    Effective, EffectiveInputs, EffectiveRoute, EffectiveRouteInput, EffectiveShapeError,
    EffectiveStrip, ModelInputs, MonitorOverlay, any_solo, effective, effective_gate,
    effective_into, effective_strip,
};
pub use error::ControlError;
pub use live_lanes::{LiveLanes, live_lanes};
pub use live_ramps::LiveRamps;
pub use mask::mask_values;
pub use registry::{ControlRow, get, list, row_descriptor, row_motion, set};
pub use tables::{EffectTableRows, OutputCase, RowSource, TableRow, effect_table_rows, table_rows};
