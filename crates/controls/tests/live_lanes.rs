//! Decision 17, C6: the live lanes of a plan follow from the session's live control rows.

mod common;

use common::{RATE, effects_fixture, fixture};
use controls::{ControlClass, ControlRow, Family, LiveLanes, MonitorOverlay, list};

fn rows(model: &session::SessionModel) -> Vec<ControlRow> {
    list(model, &MonitorOverlay::default(), RATE).collect()
}

/// Red if the input or route lane ignores its family, or if a row that is not live asks for a
/// lane: the test table flips every route row to derived, then every input row to derived.
#[test]
fn a_live_route_or_input_row_asks_for_its_lane() {
    let model = fixture();
    let set = |family: Family, class: ControlClass| {
        let mut table = rows(&model);
        let mut changed = 0;
        for row in &mut table {
            if row.descriptor.family == family {
                row.descriptor.class = class;
                changed += 1;
            }
        }
        assert!(changed > 0, "the fixture lists a {family:?} row");
        LiveLanes::of_rows(table)
    };
    // The input rows (#1556) and the send's rows (#1557) are live today.
    let today = LiveLanes::of_rows(rows(&model));
    assert!(today.strip_input && today.routes);
    assert_eq!(
        set(Family::Route, ControlClass::Derived),
        LiveLanes {
            strip_input: true,
            ..LiveLanes::default()
        }
    );
    assert_eq!(
        set(Family::StripInput, ControlClass::Derived),
        LiveLanes {
            routes: true,
            ..LiveLanes::default()
        }
    );
}

/// Red if a row's `active` flag, which a value (`matrix_mode`) flips, changes which lanes a plan
/// has: the lanes follow class and family alone, so the same session in two states has one plan
/// shape and a live edit can never find its lane missing.
#[test]
fn a_rows_active_flag_never_changes_the_lanes() {
    let model = effects_fixture();
    let lanes_with = |active: bool| {
        let mut table = rows(&model);
        for row in &mut table {
            row.descriptor.class = ControlClass::Live;
            row.descriptor.active = active;
        }
        LiveLanes::of_rows(table)
    };
    let all = LiveLanes {
        strip_input: true,
        effects: true,
        routes: true,
    };
    assert_eq!(lanes_with(true), all);
    assert_eq!(lanes_with(false), all);
}
