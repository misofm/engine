//! Emit the accepted dense 192-track × 4-effect JSON document, padded to 1 MiB: the staging
//! ceiling until issue #2191 made the memory budget the document's limit, kept as a
//! representative point of the parse projection.

use std::io::{self, Write as _};

use session::{RouteSource, SendTap, StableId, canonical_session_json, parse_session_json};

const TRACKS: usize = 192;
/// The padded length: the retired 1 MiB ceiling, a representative point (issue #2191).
const DOCUMENT_BYTES: usize = 1 << 20;

fn main() -> io::Result<()> {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    // Four full effect declarations per track, as inserts: the densest per-track declaration
    // the schema accepts (a console entry is smaller than the effect it configures).
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
    model.tracks.reserve(TRACKS);
    model.routes.reserve(TRACKS);
    for index in 0..TRACKS {
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
    let mut document = canonical_session_json(&model)
        .expect("worst accepted shape canonicalizes")
        .into_bytes();
    assert!(document.len() + 2 <= DOCUMENT_BYTES);
    document.resize(DOCUMENT_BYTES, b' ');
    io::stdout().write_all(&document)
}
