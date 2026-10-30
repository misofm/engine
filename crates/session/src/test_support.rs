//! Test support (issue #2219): session shapes several crates' gates build, behind the
//! `test-support` feature, so each shape has one builder.

use crate::{RouteSource, SendTap, StableId, canonical_session_json, parse_session_json};

/// The dense console shape of issue #2191, as canonical JSON: `tracks` tracks on the nine-track
/// fixture's second track, each with four full EQ inserts (the densest per-track declaration the
/// schema accepts: a console entry is smaller than the effect it configures) and a post-pan route
/// to the main output.
///
/// The browser's boot gate (`host-web`'s `tests/boot_transient_budget.rs` natively and
/// `examples/dense_document.rs` for the wasm leg) and the control plane's replace gate
/// (`control-plane`'s `tests/session_replace.rs`) build it here.
///
/// # Panics
///
/// Never for a track count whose IDs are stable IDs: the seed fixture parses and the shape
/// canonicalizes.
#[must_use]
pub fn dense_document(tracks: usize) -> Vec<u8> {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    let effect = model.lower_track(&track).pre_insert[0].clone();
    model.console.pre_insert.clear();
    track.console.clear();
    track.inserts.effects.clear();
    for index in 0..4 {
        let mut effect = effect.clone();
        effect.id = StableId::parse(&format!("effect-{index}")).expect("effect ID");
        track.inserts.effects.push(effect);
    }
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    model.tracks.reserve(tracks);
    model.routes.reserve(tracks);
    for index in 0..tracks {
        let track_id = StableId::parse(&format!("track-{index:03}")).expect("track ID");
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        model.tracks.push(next_track);

        let mut next_route = route.clone();
        next_route.id = StableId::parse(&format!("route-{index:03}")).expect("route ID");
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    canonical_session_json(&model)
        .expect("the dense shape canonicalizes")
        .into_bytes()
}
