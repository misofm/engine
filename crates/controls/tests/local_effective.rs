//! TBD9 D7: the effective values of one strip and one route, read through `LocalInputs` and
//! `any_solo_in`, are the dense `effective`'s, so a host that classifies one edit applies the very
//! rules of the whole-model pass without building its tables.

mod common;

use common::{effects_fixture, id};
use controls::{
    EffectiveInputs, LocalInputs, ModelInputs, MonitorOverlay, any_solo, any_solo_in, effective,
    effective_follow, effective_gate, effective_strip,
};
use session::{DualMonoFader, SessionModel, Vca};

/// The effects fixture with a third track and a nested VCA, tracks in ID order (a compiled
/// session's normalized order, which `LocalInputs` searches).
fn model() -> SessionModel {
    let mut model = effects_fixture();
    let mut bass = model.tracks[0].clone();
    bass.id = id("bass");
    model.tracks.push(bass);
    model.tracks.sort_by(|left, right| left.id.cmp(&right.id));
    model.vcas.push(Vca {
        id: id("master"),
        fader: DualMonoFader {
            left_db: -2.0,
            right_db: 1.5,
            left_mute: false,
            right_mute: true,
        },
        members: vec![id("riders"), id("bass"), id("bus")],
    });
    model
}

/// Red if `LocalInputs` or `any_solo_in` gives any strip's fader, mute or VCA mute, any route's
/// gate or follow, or the any-solo flag a value other than the dense pass gives, under every
/// overlay and VCA setting here (a solo-safe submix, a nested VCA, a muted VCA, a following send).
#[test]
fn one_strip_and_one_route_equal_the_dense_pass() {
    let overlays = [
        &[][..],
        &["vocal"],
        &["guitar", "vocal"],
        &["bus"],
        &["bus", "bass"],
        &["nobody"],
    ];
    let vca_settings = [
        (false, false, 0.0),
        (true, false, -4.0),
        (false, true, 3.0),
        (true, true, 24.0),
    ];
    for names in overlays {
        for (riders_mute, master_mute, offset) in vca_settings {
            let mut model = model();
            model.vcas[0].fader.left_mute = riders_mute;
            model.vcas[1].fader.right_mute = master_mute;
            model.vcas[1].fader.left_db = offset;
            let overlay = MonitorOverlay::with_soloed(names.iter().map(|name| id(name)));
            let dense = effective(&model, &overlay);
            let inputs = ModelInputs::new(&model, &overlay);
            assert_eq!(
                any_solo_in(&model, &overlay),
                any_solo(&inputs),
                "{names:?}"
            );
            let any = any_solo_in(&model, &overlay);
            for strip in 0..inputs.strip_count() {
                let local = LocalInputs::new(&model, &overlay, strip).expect("a strip");
                assert_eq!(
                    effective_strip(&local, strip, any),
                    dense.strips[strip],
                    "{names:?} strip {strip}"
                );
            }
            for route in 0..inputs.route_count() {
                let source = inputs.route(route).source_strip;
                let local = LocalInputs::new(&model, &overlay, source).expect("a source strip");
                let source = effective_strip(&local, source, any);
                assert_eq!(
                    local.route(route),
                    inputs.route(route),
                    "{names:?} route {route}"
                );
                assert_eq!(
                    effective_gate(&local, route, &source),
                    dense.routes[route].gate
                );
                assert_eq!(
                    effective_follow(&local, route, &source),
                    dense.routes[route].follow
                );
            }
            assert!(LocalInputs::new(&model, &overlay, inputs.strip_count()).is_none());
        }
    }
}

/// `strip_index` is a scan's answer on a sorted model and on an unsorted one (the fixture lists
/// `vocal` before `guitar`), and `None` for an unknown ID. Red if the binary search is trusted on a
/// slice that is out of order.
#[test]
fn strip_index_is_the_scans_answer() {
    let sorted = model();
    let unsorted = effects_fixture();
    assert!(
        unsorted.tracks[0].id > unsorted.tracks[1].id,
        "the fixture is out of order"
    );
    for model in [&sorted, &unsorted] {
        for (at, strip) in model.strips().enumerate() {
            assert_eq!(controls::strip_index(model, strip.id), Some(at));
        }
        assert_eq!(controls::strip_index(model, &id("nobody")), None);
    }
}
