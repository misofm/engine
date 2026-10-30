//! Emit the dense 6,000-track console shape of issue #2191 (about 16.5 MB of JSON): every track
//! with four full EQ inserts and a post-pan route to the main output. Its retained rows exceed the
//! default 512 MiB budget; `scripts/check-web-boot-budget.mjs` boots it in the wasm module and
//! requires the boot's retained projection to refuse it before the linear memory grows past the
//! budget plus one page (issue #2199).
//!
//! `tests/boot_transient_budget.rs` builds the same shape for its native twin of that gate and its
//! parse-transient cases.

use std::io::{self, Write as _};

use session::{RouteSource, SendTap, StableId, canonical_session_json, parse_session_json};

const TRACKS: usize = 6_000;

/// The dense accepted shape: `tracks` tracks, four effect declarations per track, each with a
/// post-pan route to the main output.
fn document(tracks: usize) -> Vec<u8> {
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

fn main() -> io::Result<()> {
    io::stdout().write_all(&document(TRACKS))
}
