//! Emit #240's accepted 512-track × 4-effect document at the exact 1 MiB staging ceiling.

use std::io::{self, Write as _};

use host_web::MAXIMUM_DOCUMENT_BYTES;
use session::{RouteSource, SendTap, StableId, canonical_session_toml, parse_session_toml};

const TRACKS: usize = 512;

fn main() -> io::Result<()> {
    let mut model = parse_session_toml(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.toml"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    let effect = track.simd1.effects[0].clone();
    track.simd1.effects.clear();
    for index in 0..4 {
        let mut effect = effect.clone();
        effect.id = StableId::parse(&format!("effect-{index}")).expect("effect ID");
        track.simd1.effects.push(effect);
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
            tap: SendTap::PostMatrix,
        };
        model.routes.push(next_route);
    }
    let mut document = canonical_session_toml(&model)
        .expect("worst accepted shape canonicalizes")
        .into_bytes();
    let maximum = MAXIMUM_DOCUMENT_BYTES as usize;
    assert!(document.len() + 2 <= maximum);
    document.extend_from_slice(b"\n#");
    document.resize(maximum, b'x');
    io::stdout().write_all(&document)
}
