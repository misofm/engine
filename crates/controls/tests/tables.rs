//! The row-table accessors a generator reads (decision 17, slice S2b) carry every row that
//! [`controls::list`] gives, with the descriptor `list` gives: a row added to a table and missed
//! by the accessor, or described differently, turns this red.

mod common;

use common::{RATE, effects_fixture};
use controls::{
    ControlDescriptor, ControlRow, MonitorOverlay, Owner, RowSource, Section, effect_table_rows,
    list, table_rows,
};
use session::{EffectIdentity, RouteDestination};

/// The table row that a listed row came from.
fn table_descriptor(model: &session::SessionModel, listed: &ControlRow) -> ControlDescriptor {
    let control = listed.address.control;
    let find = |source| {
        table_rows(source)
            .into_iter()
            .find(|row| row.control == control)
            .unwrap_or_else(|| panic!("no table row for {:?}", listed.address))
    };
    match (&listed.address.owner, &listed.address.section) {
        (Owner::Session, _) => find(RowSource::SessionOwn).at_rate(RATE),
        (Owner::Vca(_), _) => find(RowSource::VcaOwn).at_rate(RATE),
        (Owner::Strip(_), Section::Own) => find(RowSource::StripOwn).at_rate(RATE),
        (Owner::Strip(_), Section::Builtins) => find(RowSource::Builtins).at_rate(RATE),
        (Owner::Route(id), _) => {
            let row = find(RowSource::RouteOwn);
            let route = model.routes.iter().find(|route| route.id == *id).unwrap();
            // #1391: a route into the output from before the fader has its own case.
            let case = if controls::route_rows_live(route) {
                row.into_output_before_fader
            } else {
                row.into_output
            };
            match (route.destination.clone(), case) {
                (RouteDestination::SubmixInput { .. }, _) => row.descriptor,
                (RouteDestination::OutputInput { .. }, Some(output)) => ControlDescriptor {
                    class: output.class,
                    active: output.active,
                    ..row.descriptor
                },
                (RouteDestination::OutputInput { .. }, None) => panic!("route row has no case"),
            }
        }
        (Owner::Strip(strip), Section::Console(_) | Section::Inserts(_)) => {
            let identity = match &listed.address.section {
                Section::Console(slot) => model
                    .console
                    .slots()
                    .find(|candidate| candidate.slot == *slot)
                    .map(|candidate| candidate.identity.clone()),
                Section::Inserts(effect) => model
                    .strips()
                    .find(|candidate| candidate.id == strip)
                    .and_then(|strip| {
                        strip
                            .inserts
                            .effects
                            .iter()
                            .find(|candidate| candidate.id == *effect)
                            .map(|candidate| candidate.identity.clone())
                    }),
                _ => None,
            }
            .expect("listed instance exists");
            let EffectIdentity::Native { effect_id } = identity else {
                panic!("fixture effects are native");
            };
            let rows = effect_table_rows(effect_id.as_str()).expect("launch effect has rows");
            let row = if control == controls::BYPASS_CONTROL {
                rows.bypass
            } else if control == controls::LINK_MODE_CONTROL {
                match listed.address.section {
                    Section::Console(_) => rows.console_link_mode,
                    _ => rows.link_mode,
                }
                .expect("a listed link mode has a row")
            } else {
                rows.parameters
                    .into_iter()
                    .find(|row| row.control == control)
                    .expect("a listed parameter has a row")
            };
            row.descriptor
        }
    }
}

#[test]
fn every_listed_row_is_in_the_tables_with_its_descriptor() {
    let model = effects_fixture();
    let mut seen = 0;
    for listed in list(&model, &MonitorOverlay::default(), RATE) {
        let expected = table_descriptor(&model, &listed);
        // `active` follows the owner's shape in a listing; the table row is the active row.
        let listed_descriptor = ControlDescriptor {
            active: expected.active,
            ..listed.descriptor
        };
        assert_eq!(
            listed_descriptor, expected,
            "descriptor of {:?}",
            listed.address
        );
        seen += 1;
    }
    assert!(seen > 400, "the fixture lists every launch effect: {seen}");
}
