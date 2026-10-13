//! A route's rows (decision 17, C3).
//!
//! The class of a route's value depends on the route's shape, by a rule in the row: a send into a
//! submix is ruled live and its issue (S6) has not landed, and a route into the output is
//! rebuild-only for decision 14's fold reason.

use effect_contract::{ParameterLattice, ParameterUnit};
use session::{
    ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM, ROUTE_GAIN_DB_MAXIMUM, ROUTE_GAIN_DB_MINIMUM, Route,
    RouteDestination, SessionModel, StableId,
};

use crate::address::{Owner, Section};
use crate::descriptor::{
    BASE, ControlDescriptor, EFFECTIVE_SLICE, Lanes, Motion, ROUTE_SLICE, ruled_live_not_built,
};
use crate::error::ControlError;
use crate::lane::{Side, toggle};
use crate::registry::{ControlRow, Resolved, Source, push_control};
use crate::{ControlClass, ControlKind, Family, LiveRampRow, RebuildReason, Taper};

const GAIN_DB: u32 = 1;
const MUTE: u32 = 2;
const MATRIX_LL: u32 = 3;
const MATRIX_LR: u32 = 4;
const MATRIX_RL: u32 = 5;
const MATRIX_RR: u32 = 6;
const FOLLOWS_MUTE: u32 = 7;
/// The own-row ID of `effective_gate`.
const EFFECTIVE_GATE: u32 = 64;

const SEND_REOPEN: &str =
    "The route slice (decision 17, S6) builds the live path and flips this row to live.";
const FOLLOW_REOPEN: &str = "The effective-value slice (decision 17, S7a) builds the live path \
     and flips this row to live.";

/// A route into the output keeps the single-master route fold, and a live route never folds
/// (decision 14, rule 3, optimisation; decision 13 O9).
const OUTPUT_FOLD: ControlClass = ControlClass::RebuildOnly {
    reason: RebuildReason::Optimisation,
    reopen: "An owner product request together with live route lanes in the epilogue fold \
             (decision 13, O9 and O4), or a real-path measurement showing that losing the fold \
             costs little.",
};

/// How a route's own row reaches its value.
pub(crate) enum RouteValue {
    /// A field of the model.
    Stored {
        /// The committed value.
        read: fn(&Route) -> f32,
        /// The model write, after the value passed [`crate::check`].
        write: fn(&mut Route, f32),
    },
    /// Computed from other controls.
    Derived {
        /// The value of one lane, from the route's effective gate.
        read: fn([bool; 2], Side) -> f32,
    },
}

/// A matrix-coefficient row of the route table.
macro_rules! matrix_row {
    ($id:expr, $name:literal, $display_name:literal, $default:expr, $field:ident) => {
        RouteRow {
            id: $id,
            descriptor: ControlDescriptor {
                name: $name,
                display_name: $display_name,
                default: $default,
                ..ROUTE_BASE
            },
            output_class: OUTPUT_FOLD,
            output_active: true,
            value: RouteValue::Stored {
                read: |route| route.channel_matrix.$field,
                write: |route, value| route.channel_matrix.$field = value,
            },
        }
    };
}

/// One row of the route table.
pub(crate) struct RouteRow {
    pub(crate) id: u32,
    /// The descriptor of the row on a route into a submix.
    pub(crate) descriptor: ControlDescriptor,
    /// The class on a route into the output.
    pub(crate) output_class: ControlClass,
    /// Whether the row applies to a route into the output.
    pub(crate) output_active: bool,
    pub(crate) value: RouteValue,
}

/// What every route row starts from: a shared, ruled-live send value in `[-1, 1]` on the route
/// family.
const ROUTE_BASE: ControlDescriptor = ControlDescriptor {
    minimum: -ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM,
    maximum: ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM,
    motion: Motion::Ramp(LiveRampRow::SendMatrix),
    class: ruled_live_not_built(ROUTE_SLICE, SEND_REOPEN),
    family: Family::Route,
    ..BASE
};

/// The route rows, in listing order. Never reuse an ID.
pub(crate) static ROWS: [RouteRow; 8] = [
    RouteRow {
        id: GAIN_DB,
        descriptor: ControlDescriptor {
            name: "gain_db",
            display_name: "Send Gain",
            unit: ParameterUnit::Db,
            display_unit: "dB",
            minimum: ROUTE_GAIN_DB_MINIMUM,
            maximum: ROUTE_GAIN_DB_MAXIMUM,
            taper: Taper::Decibel,
            lattice: ParameterLattice::arithmetic(0.1, 1),
            motion: Motion::Ramp(LiveRampRow::SendGain),
            ..ROUTE_BASE
        },
        output_class: OUTPUT_FOLD,
        output_active: true,
        value: RouteValue::Stored {
            read: |route| route.gain_db,
            write: |route, value| route.gain_db = value,
        },
    },
    RouteRow {
        id: MUTE,
        descriptor: ControlDescriptor {
            name: "mute",
            display_name: "Send Mute",
            kind: ControlKind::Toggle,
            minimum: 0.0,
            maximum: 1.0,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            motion: Motion::Ramp(LiveRampRow::SendMute),
            ..ROUTE_BASE
        },
        output_class: OUTPUT_FOLD,
        output_active: true,
        value: RouteValue::Stored {
            read: |route| toggle(route.mute),
            write: |route, value| route.mute = crate::lane::is_on(value),
        },
    },
    matrix_row!(
        MATRIX_LL,
        "matrix_ll",
        "Send Matrix Left from Left",
        1.0,
        ll
    ),
    matrix_row!(
        MATRIX_LR,
        "matrix_lr",
        "Send Matrix Left from Right",
        0.0,
        lr
    ),
    matrix_row!(
        MATRIX_RL,
        "matrix_rl",
        "Send Matrix Right from Left",
        0.0,
        rl
    ),
    matrix_row!(
        MATRIX_RR,
        "matrix_rr",
        "Send Matrix Right from Right",
        1.0,
        rr
    ),
    RouteRow {
        id: FOLLOWS_MUTE,
        descriptor: ControlDescriptor {
            name: "follows_mute",
            display_name: "Send Follows Mute",
            kind: ControlKind::Toggle,
            minimum: 0.0,
            maximum: 1.0,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            motion: Motion::Ramp(LiveRampRow::SendFollowsMute),
            class: ruled_live_not_built(EFFECTIVE_SLICE, FOLLOW_REOPEN),
            ..ROUTE_BASE
        },
        // A route into the output carries `false` (#1218): the row does not apply.
        output_class: OUTPUT_FOLD,
        output_active: false,
        value: RouteValue::Stored {
            read: |route| toggle(route.follows_mute),
            write: |route, value| route.follows_mute = crate::lane::is_on(value),
        },
    },
    RouteRow {
        id: EFFECTIVE_GATE,
        descriptor: ControlDescriptor {
            name: "effective_gate",
            display_name: "Effective Gate",
            kind: ControlKind::Toggle,
            lanes: Lanes::PerLane,
            minimum: 0.0,
            maximum: 1.0,
            taper: Taper::Stepped,
            lattice: ParameterLattice::indices(),
            motion: Motion::Fixed,
            class: ControlClass::Derived,
            ..ROUTE_BASE
        },
        output_class: ControlClass::Derived,
        output_active: true,
        value: RouteValue::Derived {
            // `crate::effective_gate`, per source lane.
            read: |gate, side| toggle(side.pick(gate[0], gate[1])),
        },
    },
];

/// The descriptor of one row on one route: the class and applicability depend on the route's
/// destination.
fn descriptor(row: &RouteRow, route: &Route) -> ControlDescriptor {
    match route.destination {
        RouteDestination::SubmixInput { .. } => row.descriptor,
        RouteDestination::OutputInput { .. } => ControlDescriptor {
            class: row.output_class,
            active: row.output_active,
            ..row.descriptor
        },
    }
}

/// Append every control row of one route. `gate` is the route's effective gate, per source lane.
pub(crate) fn push_rows(out: &mut Vec<ControlRow>, route: &Route, gate: [bool; 2]) {
    let owner = Owner::Route(route.id.clone());
    for row in &ROWS {
        let descriptor = descriptor(row, route);
        push_control(
            out,
            &owner,
            &Section::Own,
            row.id,
            &descriptor,
            |side| match &row.value {
                RouteValue::Stored { read, .. } => read(route),
                RouteValue::Derived { read } => read(gate, side),
            },
        );
    }
}

/// Find one row of route `id` by its control ID, as [`push_rows`] would list it.
pub(crate) fn resolve(
    model: &SessionModel,
    id: &StableId,
    control: u32,
) -> Result<Resolved, ControlError> {
    let route = model
        .routes
        .iter()
        .find(|route| route.id == *id)
        .ok_or(ControlError::AddressUnknown)?;
    let row = ROWS
        .iter()
        .find(|row| row.id == control)
        .ok_or(ControlError::AddressUnknown)?;
    let descriptor = descriptor(row, route);
    Ok(match &row.value {
        RouteValue::Stored { read, .. } => Resolved::stored(descriptor, |_| read(route)),
        RouteValue::Derived { read } => Resolved {
            descriptor,
            source: Source::RouteDerived(*read),
        },
    })
}

/// Write one of a route's rows.
pub(crate) fn write(route: &mut Route, control: u32, value: f32) -> Result<(), ControlError> {
    match ROWS.iter().find(|row| row.id == control) {
        Some(RouteRow {
            value: RouteValue::Stored { write, .. },
            ..
        }) => {
            write(route, value);
            Ok(())
        }
        Some(_) => Err(ControlError::ReadOnly),
        None => Err(ControlError::AddressUnknown),
    }
}
