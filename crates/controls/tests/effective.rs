//! The effective values (decision 17, C7): the solo term, the follow gate per source lane, the
//! allocation-free form, and the derived rows that read them.

mod common;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use common::{RATE, address, fixture, id, strip_own};
use controls::{
    Effective, EffectiveInputs, EffectiveShapeError, EffectiveStrip, Lane, ModelInputs,
    MonitorOverlay, Owner, Section, effective, effective_into, get, list,
};
use session::{
    DualMonoFader, RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix, Vca,
};

const BOTH_OFF: [bool; 2] = [false, false];
const BOTH_ON: [bool; 2] = [true, true];

/// Tracks `vocal` (strip 0) and `guitar` (strip 1), submix `bus` (strip 2), no VCA, and one send
/// from `guitar` into `bus` that follows its source's mute.
fn two_tracks_and_a_bus() -> SessionModel {
    let mut model = fixture();
    model.vcas.clear();
    model.routes.retain(|route| route.id.as_str() == "send");
    let send = &mut model.routes[0];
    send.source = RouteSource::Track {
        track_id: id("guitar"),
        tap: SendTap::PostFader,
    };
    assert!(send.follows_mute);
    model
}

fn soloing(ids: &[&str]) -> MonitorOverlay {
    MonitorOverlay::with_soloed(ids.iter().map(|name| id(name)))
}

fn mutes(effective: &Effective) -> Vec<[bool; 2]> {
    effective.strips.iter().map(|strip| strip.mute).collect()
}

/// Gate 2. Soloing track 1 mutes track 2's two lanes, leaves track 1 and the submix unmuted and
/// gates the send from track 2 on both source lanes; soloing the submix changes nothing; a user
/// mute stays muted under solo; a VCA mute mutes a soloed track; an unknown or empty solo is no
/// solo.
///
/// Red if the solo term mutes a solo-safe submix, ignores a user or VCA mute, lets a submix ID
/// count as a solo, or leaks into `vca_mute`.
#[test]
fn solo_mutes_the_other_tracks_and_gates_their_sends() {
    let mut model = two_tracks_and_a_bus();
    let nothing = effective(&model, &MonitorOverlay::default());
    assert_eq!(mutes(&nothing), [BOTH_OFF; 3]);
    assert_eq!(nothing.routes[0].gate, BOTH_OFF);

    let solo_vocal = effective(&model, &soloing(&["vocal"]));
    assert_eq!(mutes(&solo_vocal), [BOTH_OFF, BOTH_ON, BOTH_OFF]);
    assert_eq!(
        solo_vocal.routes[0].gate, BOTH_ON,
        "the send's source is muted"
    );
    assert!(
        solo_vocal
            .strips
            .iter()
            .all(|strip| strip.vca_mute == BOTH_OFF)
    );

    let solo_both = effective(&model, &soloing(&["guitar", "vocal"]));
    assert_eq!(mutes(&solo_both), [BOTH_OFF; 3]);

    for overlay in [
        soloing(&["bus"]),
        soloing(&["nobody"]),
        soloing(&["bus", "nobody"]),
    ] {
        assert_eq!(effective(&model, &overlay), nothing, "{overlay:?}");
    }
    // A submix ID beside a track solo changes nothing about that solo.
    assert_eq!(effective(&model, &soloing(&["bus", "vocal"])), solo_vocal);

    // The user's mute stays under solo, on its own lane only.
    model.tracks[0].fader.left_mute = true;
    let muted = effective(&model, &soloing(&["vocal"]));
    assert_eq!(muted.strips[0].mute, [true, false]);
    assert_eq!(muted.strips[0].vca_mute, BOTH_OFF);

    // A submix's own mute stays muted under solo too.
    model.tracks[0].fader.left_mute = false;
    model.submixes[0].fader.right_mute = true;
    assert_eq!(
        mutes(&effective(&model, &soloing(&["vocal"]))),
        [BOTH_OFF, BOTH_ON, [false, true]]
    );

    // A VCA mute mutes a soloed track, and says so in `vca_mute`.
    model.submixes[0].fader.right_mute = false;
    model.vcas.push(Vca {
        id: id("riders"),
        fader: DualMonoFader {
            left_db: -6.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: true,
        },
        members: vec![id("vocal")],
    });
    let vca_muted = effective(&model, &soloing(&["vocal"]));
    assert_eq!(vca_muted.strips[0].mute, [false, true]);
    assert_eq!(vca_muted.strips[0].vca_mute, [false, true]);
    assert_eq!(
        vca_muted.strips[0].fader_db[0],
        model.tracks[0].fader.left_db - 6.0
    );
}

/// Gate 3. A source muted on its left lane only gives `[true, false]` for a following route and
/// `[false, false]` for a route that does not follow; the route's own mute gates both lanes.
///
/// Red if the gate is computed per route instead of per source lane, which would gate both columns
/// of a send whose source mutes one lane, or if a route without `follows_mute` follows.
#[test]
fn the_follow_gate_is_per_source_lane() {
    let mut model = two_tracks_and_a_bus();
    let mut plain = model.routes[0].clone();
    plain.id = id("plain");
    plain.follows_mute = false;
    let mut muted = plain.clone();
    muted.id = id("muted");
    muted.mute = true;
    model.routes.extend([plain, muted]);
    model.tracks[1].fader.left_mute = true;

    let result = effective(&model, &MonitorOverlay::default());
    assert_eq!(result.strips[1].mute, [true, false]);
    assert_eq!(result.routes[0].gate, [true, false], "follows");
    assert_eq!(result.routes[1].gate, BOTH_OFF, "does not follow");
    assert_eq!(result.routes[2].gate, BOTH_ON, "its own mute");

    // A solo-muted source gates a following route on both lanes.
    let soloed = effective(&model, &soloing(&["vocal"]));
    assert_eq!(soloed.routes[0].gate, BOTH_ON);
    assert_eq!(soloed.routes[1].gate, BOTH_OFF);
}

/// 64 tracks, 4 submixes, 8 nested VCAs (each holds the next and eight tracks) and 8 following
/// sends. Not validated: the function reads it as it stands.
fn large_model() -> SessionModel {
    let mut model = fixture();
    model.routes.clear();
    model.vcas.clear();
    let template = model.tracks[0].clone();
    model.tracks = (0..64)
        .map(|index| {
            let mut track = template.clone();
            track.id = id(&format!("t{index:02}"));
            track.fader.left_db = -(index as f32) * 0.5;
            track.fader.right_mute = index % 7 == 0;
            track
        })
        .collect();
    model.submixes = (0..4)
        .map(|index| Submix::unity(id(&format!("s{index}")), &model.console))
        .collect();
    model.vcas = (0..8)
        .map(|index| {
            let mut members: Vec<StableId> = (index * 8..index * 8 + 8)
                .map(|track| id(&format!("t{track:02}")))
                .collect();
            if index + 1 < 8 {
                members.push(id(&format!("v{}", index + 1)));
            }
            Vca {
                id: id(&format!("v{index}")),
                fader: DualMonoFader {
                    left_db: -1.5,
                    right_db: 0.25,
                    left_mute: index == 5,
                    right_mute: false,
                },
                members,
            }
        })
        .collect();
    let template = fixture().routes[2].clone();
    model.routes = (0..8)
        .map(|index| {
            let mut route = template.clone();
            route.id = id(&format!("r{index}"));
            route.source = RouteSource::Track {
                track_id: id(&format!("t{:02}", index * 8)),
                tap: SendTap::PostFader,
            };
            route.destination = RouteDestination::SubmixInput {
                submix_id: id(&format!("s{}", index % 4)),
            };
            route.follows_mute = true;
            route
        })
        .collect();
    model
}

/// Gate 4. `effective_into` on a prebuilt `ModelInputs` and `Effective` allocates and frees
/// nothing, and gives the value `effective` gives; a wrong-shaped `Effective` is refused before
/// any write.
///
/// Red if the function allocates (a collected reach, a boxed iterator, a cloned ID), which would
/// make it unusable where allocation is refused; or if the shape check follows the first write.
#[test]
fn effective_into_allocates_nothing_and_refuses_a_wrong_shape() {
    assert_installed();
    let model = large_model();
    let overlay = soloing(&["t08", "t33"]);
    let inputs = ModelInputs::new(&model, &overlay);
    assert_eq!((inputs.strip_count(), inputs.route_count()), (68, 8));
    let mut out = Effective::with_shape(68, 8);
    effective_into(&inputs, &mut out).expect("warm call");

    let mark = current_thread_counters();
    let result = effective_into(&inputs, &mut out);
    let delta = current_thread_delta_since(mark);
    assert_eq!(result, Ok(()));
    assert_eq!((delta.allocations, delta.deallocations), (0, 0));
    assert_eq!(out, effective(&model, &overlay));
    // The model is not trivial: solo, VCA mute and following sends all act.
    assert!(out.strips.iter().any(|strip| strip.vca_mute[0]));
    assert!(out.routes.iter().any(|route| route.gate == BOTH_ON));
    assert!(out.routes.iter().any(|route| route.gate == BOTH_OFF));

    for (strips, routes) in [(67, 8), (68, 7), (69, 9), (0, 0)] {
        let mut wrong = Effective::with_shape(strips, routes);
        wrong.strips.fill(EffectiveStrip {
            fader_db: [9.0; 2],
            mute: BOTH_ON,
            vca_mute: BOTH_ON,
        });
        let before = wrong.clone();
        assert_eq!(
            effective_into(&inputs, &mut wrong),
            Err(EffectiveShapeError {
                expected: (68, 8),
                actual: (strips, routes),
            })
        );
        assert_eq!(wrong, before, "nothing written for {strips} x {routes}");
    }
}

/// Decision 17 D6. `list` and `get` fill the derived rows from `effective` under the overlay, and
/// the solo row reads the overlay: soloing `vocal` gives `guitar` an effective mute on both lanes
/// and the send an effective gate on both source lanes.
///
/// Red if a derived row is computed without the overlay (the pre-S1b helper), or per route
/// instead of per source lane.
#[test]
fn the_derived_rows_follow_the_overlay() {
    const SOLO: u32 = 2;
    const EFFECTIVE_MUTE: u32 = 65;
    const EFFECTIVE_GATE: u32 = 64;
    let model = two_tracks_and_a_bus();
    let overlay = soloing(&["vocal"]);
    let gate = |lane| address(Owner::Route(id("send")), Section::Own, EFFECTIVE_GATE, lane);
    let rows: Vec<_> = list(&model, &overlay, RATE).collect();
    let value = |address| {
        rows.iter()
            .find(|row| row.address == address)
            .unwrap()
            .value
    };

    assert_eq!(value(strip_own("vocal", SOLO, Lane::Both)), 1.0);
    assert_eq!(value(strip_own("guitar", SOLO, Lane::Both)), 0.0);
    assert_eq!(value(strip_own("guitar", EFFECTIVE_MUTE, Lane::Left)), 1.0);
    assert_eq!(value(strip_own("guitar", EFFECTIVE_MUTE, Lane::Right)), 1.0);
    assert_eq!(value(strip_own("vocal", EFFECTIVE_MUTE, Lane::Left)), 0.0);
    assert_eq!(value(strip_own("bus", EFFECTIVE_MUTE, Lane::Left)), 0.0);
    assert_eq!(value(gate(Lane::Left)), 1.0);
    assert_eq!(value(gate(Lane::Right)), 1.0);
    for row in &rows {
        assert_eq!(
            get(&model, &overlay, &row.address, RATE),
            Ok(row.value),
            "{:?}",
            row.address
        );
    }
    let quiet: Vec<_> = list(&model, &MonitorOverlay::default(), RATE).collect();
    let mutes = |lane| {
        quiet
            .iter()
            .find(|row| row.address == strip_own("guitar", EFFECTIVE_MUTE, lane))
            .unwrap()
            .value
    };
    assert_eq!(mutes(Lane::Left), 0.0);
}
