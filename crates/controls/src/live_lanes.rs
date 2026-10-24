//! Which live lanes a plan carries, chosen from the session's live control rows (decision 17,
//! C6, "Live lanes on every plan").

use session::SessionModel;

use crate::effective::MonitorOverlay;
use crate::effective::effective;
use crate::registry::{ControlRow, owner_count, owner_rows_in};
use crate::{AutomationIndex, ControlClass, Family, ModelView};

/// The optional live lanes a plan attaches. A strip's fader/mute and matrix/pan lanes are always
/// attached and have no flag.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LiveLanes {
    /// Each strip's input trim, polarity and filter lane: some strip's input-family row is live.
    pub strip_input: bool,
    /// One lane per prepared effect instance: some effect instance has a live row.
    pub effects: bool,
    /// One lane per route whose rows are live ([`crate::route_rows_live`]: into a submix, or from a
    /// tap before its strip's fader, #1391): some route row is live.
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
///
/// The list is walked one owner at a time through one reused buffer, so the transient is the rows
/// of the largest owner, not every control row of the session at full size (a 192-track, 768-effect
/// session lists 53,763 rows of 232 bytes, which `list` collects into one vector grown by
/// doubling). The class of a row does not depend on the rate or on the overlay.
#[must_use]
pub fn live_lanes(model: &SessionModel) -> LiveLanes {
    let overlay = MonitorOverlay::default();
    let effective = effective(model, &overlay);
    // The automation table is walked once for every owner's rows (#1844).
    let automation = AutomationIndex::of(model);
    let view = ModelView::whole(model).indexed(&automation);
    let mut lanes = LiveLanes::default();
    let mut rows = Vec::new();
    for owner in 0..owner_count(model) {
        rows.clear();
        owner_rows_in(
            view,
            &effective,
            &overlay,
            owner,
            model.sample_rate_hz,
            &mut rows,
        );
        let owned = LiveLanes::of_rows(rows.drain(..));
        lanes.strip_input |= owned.strip_input;
        lanes.effects |= owned.effects;
        lanes.routes |= owned.routes;
    }
    lanes
}
