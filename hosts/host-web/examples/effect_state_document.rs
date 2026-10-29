//! Emit the 512-track session with four delay inserts per track (issue #2194): about 1.4 MB of
//! JSON whose instances declare about 1.5 GB of effect state, three times the default budget.
//! `scripts/check-web-boot-budget.mjs` boots it in the wasm module and requires the refusal to come
//! before the linear memory grows past the document's parse projection.
//!
//! `tests/common/effect_state_document.rs` is a copy of [`document`] for `tests/boot_transient_budget.rs`.

use std::io::{self, Write as _};

use session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, RouteSource, SendTap, SidechainDeclaration,
    StableId, canonical_session_json, parse_session_json,
};

const TRACKS: usize = 512;
const DELAYS: usize = 4;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// `tracks` tracks on the nine-track fixture's first source, each with `DELAYS` default delay
/// inserts and a post-pan route to the main output.
pub(crate) fn document(tracks: usize) -> Vec<u8> {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    track.console.clear();
    track.inserts.effects = (0..DELAYS)
        .map(|index| Effect {
            id: id(&format!("delay-{index}")),
            identity: EffectIdentity::Native {
                effect_id: id("miso.delay"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        })
        .collect();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    for index in 0..tracks {
        let track_id = id(&format!("track-{index:03}"));
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        model.tracks.push(next_track);
        let mut next_route = route.clone();
        next_route.id = id(&format!("route-{index:03}"));
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    canonical_session_json(&model)
        .expect("canonical session")
        .into_bytes()
}

fn main() -> io::Result<()> {
    io::stdout().write_all(&document(TRACKS))
}
