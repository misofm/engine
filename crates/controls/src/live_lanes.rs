//! Which live lanes a plan carries, chosen from the session's live control rows (decision 17,
//! C6, "Live lanes on every plan").

use session::SessionModel;

use crate::effective::MonitorOverlay;
use crate::registry::{ControlRow, list};
use crate::{ControlClass, Family};

/// The optional live lanes a plan attaches. A strip's fader/mute and matrix/pan lanes are always
/// attached and have no flag.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LiveLanes {
    /// Each strip's input trim, polarity and filter lane: some strip's input-family row is live.
    pub strip_input: bool,
    /// One lane per prepared effect instance: some effect instance has a live row.
    pub effects: bool,
    /// One lane per route into a submix: some route row is live.
    pub routes: bool,
}

impl LiveLanes {
    /// The lanes the live rows in `rows` need, chosen by a row's class and family alone. A row's
    /// `active` flag is a value-dependent state (a `matrix_mode` switch flips it), and a plan's
    /// lanes must not depend on a value: an inactive live row asks for its lane as an active one
    /// does, so a live edit never finds the lane missing and a plan and its successor agree.
    #[must_use]
    pub fn of_rows(rows: impl IntoIterator<Item = ControlRow>) -> Self {
        let mut lanes = Self::default();
        for row in rows {
            let descriptor = row.descriptor;
            if descriptor.class != ControlClass::Live {
                continue;
            }
            match descriptor.family {
                Family::StripInput => lanes.strip_input = true,
                Family::Effect => lanes.effects = true,
                Family::Route => lanes.routes = true,
                Family::StripLevel | Family::StripMatrix | Family::Setting => {}
            }
        }
        lanes
    }
}

/// The live lanes a plan of `model` carries: the lanes whose family has a live row in the control
/// list (see [`LiveLanes`]).
///
/// The result depends on the session's shape and on the row classes, and on nothing a live edit
/// changes, so a plan and its successor of one session agree.
#[must_use]
pub fn live_lanes(model: &SessionModel) -> LiveLanes {
    // The class of a row does not depend on the rate or on the overlay.
    LiveLanes::of_rows(list(
        model,
        &MonitorOverlay::default(),
        model.sample_rate_hz,
    ))
}
