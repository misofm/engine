//! Decision 17, C6: the live lanes of a plan follow from the session's live control rows.

mod common;

use common::{LAUNCH_EFFECTS, RATE, effects_fixture, fixture, insert_id};
use controls::{ControlClass, ControlRow, Family, LiveLanes, MonitorOverlay, list, live_lanes};
use session::{Effect, EffectIdentity, EffectQuality, LinkMode, SidechainDeclaration};

fn rows(model: &session::SessionModel) -> Vec<ControlRow> {
    list(model, &MonitorOverlay::default(), RATE).collect()
}

/// Red if a live row's lane is not attached (a live edit would find no cell), or if a lane is
/// attached for a row that is not live (a session with no effect would pay for effect cells).
#[test]
fn the_effect_lane_follows_the_effect_rows() {
    // No effect, no effect row, no lane. With today's classes nothing else is live.
    let mut model = fixture();
    assert_eq!(live_lanes(&model), LiveLanes::default());

    // One insert on one track gives the effect lane and no other.
    let effect = LAUNCH_EFFECTS[1];
    model.tracks[0].inserts.effects.push(Effect {
        id: insert_id(effect),
        identity: EffectIdentity::Native {
            effect_id: common::id(effect),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    assert_eq!(
        live_lanes(&model),
        LiveLanes {
            effects: true,
            ..LiveLanes::default()
        }
    );

    // A console slot is an effect instance too.
    let model = effects_fixture();
    assert!(live_lanes(&model).effects);
}

/// Red if the input or route lane ignores its family, or if a row that is not live asks for a
/// lane: the test table flips every route row, then every input row, to live.
#[test]
fn a_live_route_or_input_row_asks_for_its_lane() {
    let model = fixture();
    let flip = |family: Family| {
        let mut flipped = rows(&model);
        let mut flips = 0;
        for row in &mut flipped {
            if row.descriptor.family == family {
                row.descriptor.class = ControlClass::Live;
                flips += 1;
            }
        }
        assert!(flips > 0, "the fixture lists a {family:?} row");
        LiveLanes::of_rows(flipped)
    };
    assert_eq!(
        flip(Family::Route),
        LiveLanes {
            routes: true,
            ..LiveLanes::default()
        }
    );
    assert_eq!(
        flip(Family::StripInput),
        LiveLanes {
            strip_input: true,
            ..LiveLanes::default()
        }
    );

    // Today's table: no input and no route row is live.
    let today = LiveLanes::of_rows(rows(&model));
    assert!(!today.strip_input && !today.routes);
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
