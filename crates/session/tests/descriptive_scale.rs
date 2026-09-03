//! E8: ignored descriptive control-plane timings at the mandated scale points.

use std::time::Instant;

use session::{
    CompileCaps, RouteSource, SendTap, StableId, canonical_session_json, compile_session,
    parse_session_json,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const SCALES: [usize; 2] = [32_768, 65_536];

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

fn model_at(scale: usize) -> session::SessionModel {
    let mut model = parse_session_json(CANONICAL).expect("fixture parses");
    let mut track = model.tracks[0].clone();
    track.simd1.effects.clear();
    track.dynamic.effects.clear();
    track.simd2.effects.clear();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    model.tracks.reserve(scale);
    model.routes.reserve(scale);
    for index in 0..scale {
        let track_id = StableId::parse(&format!("track-{index:05}")).expect("track ID");
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        let mut next_route = route.clone();
        next_route.id = StableId::parse(&format!("route-{index:05}")).expect("route ID");
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostMatrix,
        };
        model.tracks.push(next_track);
        model.routes.push(next_route);
    }
    model
}

#[test]
#[ignore = "descriptive release qualification; run exactly once with --ignored --exact"]
fn canonical_compile_parse_timings_at_32768_and_65536_tracks() {
    assert_eq!(SCALES, [32_768, 65_536], "mandated scale set");
    println!("tracks\tround\tcanonical_ms\tcompile_ms\tparse_ms");
    for scale in SCALES {
        let model = model_at(scale);
        assert_eq!(model.tracks.len(), scale);
        assert_eq!(model.routes.len(), scale);
        assert!(model.tracks.windows(2).all(|w| w[0].id < w[1].id));
        let warm_text = canonical_session_json(&model).expect("warmup canonical");
        let warm_compiled = compile_session(&model, caps()).expect("warmup compile");
        let warm_parsed = parse_session_json(&warm_text).expect("warmup parse");
        assert_eq!(warm_compiled.normalized_model().tracks.len(), scale);
        assert_eq!(warm_parsed.tracks.len(), scale);
        for round in 1..=2 {
            let started = Instant::now();
            let text = canonical_session_json(&model).expect("valid scale model");
            let canonical_ms = started.elapsed().as_millis();
            let started = Instant::now();
            let compiled = compile_session(&model, caps()).expect("scale compiles");
            let compile_ms = started.elapsed().as_millis();
            let started = Instant::now();
            let parsed = parse_session_json(&text).expect("scale canonical parses");
            let parse_ms = started.elapsed().as_millis();
            assert_eq!(compiled.normalized_model().tracks.len(), scale);
            assert_eq!(parsed.tracks.len(), scale);
            assert_eq!(parsed.routes.len(), scale);
            println!("{scale}\t{round}\t{canonical_ms}\t{compile_ms}\t{parse_ms}");
        }
    }
}
