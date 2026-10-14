//! The pieces of a value-only commit (TBD10): the canonical bytes of an owner, the owners' values
//! and counts, and installing edited owners in a compiled session.
//!
//! A host that changes values commits without compiling the session again, so every figure it keeps
//! must be the figure a compile would have given: the canonical text, the resource estimate and the
//! model. Red if a swap plus `finish_in_place` leaves any of them different from
//! `compile_session` of the edited model, or if a value the compiler refuses passes
//! `owners_values_valid`.

use session::{
    CompileCaps, DualMonoFader, MatrixOrPan, OwnerRef, OwnerSlots, SessionModel, StableId, Submix,
    Vca, canonical_owner_bytes, canonical_session_json, compile_session, owners_counts_equal,
    owners_values_valid, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

/// The fixture with a submix, two sends into it, and two VCAs: every kind of owner.
fn model() -> SessionModel {
    let mut model = parse_session_json(EXAMPLE).expect("fixture parses");
    model
        .submixes
        .push(Submix::unity(id("bus"), &model.console));
    let mut send = model.routes[0].clone();
    send.id = id("send");
    send.destination = session::RouteDestination::SubmixInput {
        submix_id: id("bus"),
    };
    model.routes.push(send);
    let first = model.tracks[0].id.clone();
    model.vcas.push(Vca {
        id: id("v0"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![first],
    });
    // The normalized form, which is what a compiled session holds and a host edits.
    compile_session(&model, caps())
        .expect("compiles")
        .normalized_model()
        .clone()
}

/// `model()` with values changed in one owner of each kind, to values whose canonical spellings
/// differ in length from the old ones.
fn edited(model: &SessionModel) -> SessionModel {
    let mut edited = model.clone();
    edited.tracks[0].fader.left_db = -123.456_79;
    edited.tracks[0].fader.right_mute = !edited.tracks[0].fader.right_mute;
    edited.tracks[0].builtins.left.trim_db = 0.5;
    edited.submixes[0].fader.left_db = -9.5;
    edited.vcas[0].fader.right_db = 7.0;
    edited.routes[0].gain_db = -0.000_123_4;
    edited.routes[1].mute = true;
    edited
}

fn slots(model: &SessionModel, edited: &SessionModel) -> (OwnerSlots, SessionModel) {
    // The owners model: the header and the edited clones, as `controls::OwnerEdits` builds it.
    let mut owners = model.clone();
    owners.sources.clear();
    owners.tracks = vec![edited.tracks[0].clone()];
    owners.submixes = vec![edited.submixes[0].clone()];
    owners.vcas = vec![edited.vcas[0].clone()];
    owners.routes = vec![edited.routes[0].clone(), edited.routes[1].clone()];
    let slots = OwnerSlots {
        tracks: vec![(0, 0)],
        submixes: vec![(0, 0)],
        vcas: vec![(0, 0)],
        routes: vec![(0, 0), (1, 1)],
    };
    (slots, owners)
}

/// The canonical text of a session is its owners' text and the rest, so a value edit moves its length
/// by what its owners' bytes move by.
#[test]
fn the_canonical_bytes_of_the_owners_account_for_the_change_of_the_text() {
    let model = model();
    let compiled = compile_session(&model, caps()).expect("compiles");
    let edited = edited(&model);
    let (slots, owners) = slots(compiled.normalized_model(), &edited);
    let before = canonical_owner_bytes(slots.in_committed(compiled.normalized_model()));
    let after = canonical_owner_bytes(slots.in_owners(&owners));
    assert_ne!(before, after, "the edit changes the text's length");
    let recompiled = compile_session(&edited, caps()).expect("compiles");
    assert_eq!(
        recompiled.canonical_json().len() as i64 - compiled.canonical_json().len() as i64,
        after as i64 - before as i64
    );
    assert_eq!(
        recompiled.resource_estimate().canonical_bytes,
        compiled.resource_estimate().canonical_bytes + after as u64 - before as u64
    );
}

/// Installing edited owners and finishing gives the compiled session of the edited model: the model,
/// the canonical text (written again on first read) and the estimate. A second swap undoes the first.
#[test]
fn swapping_the_owners_in_gives_the_compiled_session_of_the_edited_model() {
    let model = model();
    let mut compiled = compile_session(&model, caps()).expect("compiles");
    let original = compiled.clone();
    let edited = edited(&model);
    let recompiled = compile_session(&edited, caps()).expect("compiles");
    let (slots, mut owners) = slots(compiled.normalized_model(), &edited);
    assert!(owners_values_valid(slots.in_owners(&owners)));
    assert!(owners_counts_equal(
        slots.in_committed(compiled.normalized_model()),
        slots.in_owners(&owners)
    ));
    compiled.swap_owners(&mut owners, &slots);
    compiled.finish_in_place(
        recompiled.normalized_model().revision,
        recompiled.resource_estimate(),
    );
    assert_eq!(compiled.normalized_model(), recompiled.normalized_model());
    assert_eq!(compiled.canonical_json(), recompiled.canonical_json());
    assert_eq!(
        compiled.canonical_json(),
        canonical_session_json(compiled.normalized_model()).expect("canonical")
    );
    assert_eq!(compiled.resource_estimate(), recompiled.resource_estimate());
    // The owners that came out are the committed ones: swapping again restores the session.
    compiled.swap_owners(&mut owners, &slots);
    assert_eq!(
        compiled.normalized_model().tracks,
        original.normalized_model().tracks
    );
    assert_eq!(
        compiled.normalized_model().routes,
        original.normalized_model().routes
    );
}

/// The values the compiler refuses are refused by `owners_values_valid`, one owner kind at a time,
/// and a clone with another count of anything is not a change of values.
#[test]
fn owners_with_values_the_compiler_refuses_are_not_valid_and_counts_are_compared() {
    let model = model();
    let valid = |change: &dyn Fn(&mut SessionModel)| {
        let mut edited = model.clone();
        change(&mut edited);
        let accepted = compile_session(&edited, caps()).is_ok();
        let (slots, owners) = slots(&model, &edited);
        assert_eq!(
            owners_values_valid(slots.in_owners(&owners)),
            accepted,
            "the compiler and the owners' validator agree"
        );
        accepted
    };
    assert!(valid(&|edited| edited.tracks[0].fader.left_db = -6.0));
    assert!(!valid(&|edited| edited.tracks[0].fader.left_db = f32::NAN));
    assert!(!valid(
        &|edited| edited.tracks[0].builtins.left.hpf_hz = -1.0
    ));
    assert!(!valid(&|edited| {
        edited.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
            left: 1.5,
            right: 0.0,
            smoothing_samples: 0,
        };
    }));
    assert!(!valid(
        &|edited| edited.submixes[0].fader.right_db = f32::INFINITY
    ));
    assert!(!valid(&|edited| edited.vcas[0].fader.left_db = 30.0));
    assert!(!valid(&|edited| edited.routes[0].gain_db = 40.0));
    assert!(!valid(&|edited| edited.routes[0].channel_matrix.ll = 2.0));
    assert!(!valid(&|edited| edited.routes[1].follows_mute = true));

    let mut more = model.vcas[0].clone();
    more.members.push(id("extra"));
    assert!(!owners_counts_equal(
        [OwnerRef::Vca(&model.vcas[0])],
        [OwnerRef::Vca(&more)]
    ));
    let mut track = model.tracks[0].clone();
    track.inserts.effects[0].params.clear();
    assert!(!owners_counts_equal(
        [OwnerRef::Track(&model.tracks[0])],
        [OwnerRef::Track(&track)],
    ));
    assert!(owners_counts_equal(
        [OwnerRef::Track(&model.tracks[0])],
        [OwnerRef::Track(&model.tracks[0].clone())],
    ));
}
