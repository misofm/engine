//! Focused semantic and native render gates for the canonical fold-mono command.

use host_web::{AudioWorkletEngineHost, RESULT_OK, WebBootOptions};
use session::{RouteSource, SourceBitDepth, StableId, canonical_session_json, parse_session_json};
use session_validator::{
    FOLD_MONO_MAX_ENTRIES, FOLD_MONO_MAX_SESSION_BYTES, fold_mono_session_document,
};

const OLD: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const NEW: &str = "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn identity_session() -> session::SessionModel {
    let mut model = parse_session_json(include_str!(
        "../../../hosts/host-web/tests/browser-v1/session.json"
    ))
    .expect("browser fixture parses");
    model.sources[0].content = OLD.to_owned();
    model.sources[0].bit_depth = SourceBitDepth::Pcm24;
    model.sources[0].frames = 256;
    model.tracks[0].builtins.left.trim_db = 3.0;
    model.tracks[0].builtins.right.trim_db = -3.0;
    model.tracks[0].fader.left_db = -0.0;
    model.tracks[0].builtins.left.delay_samples = 1;
    model.tracks[0].builtins.right.delay_samples = 2;
    model.tracks[0].matrix_or_pan = session::MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.25,
        rl: -0.2,
        rr: 0.75,
        smoothing_samples: 0,
    };
    model
}

fn native_effect_session() -> session::SessionModel {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("native effect fixture parses");
    model.sources[0].content = OLD.to_owned();
    model.sources[0].bit_depth = SourceBitDepth::Pcm24;
    model.sources[0].frames = 256;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.automation.clear();
    model.tracks[0].builtins.left.trim_db = 3.0;
    model.tracks[0].builtins.right.trim_db = -3.0;
    model.tracks[0].builtins.left.delay_samples = 1;
    model.tracks[0].builtins.right.delay_samples = 2;
    model.tracks[0].builtins.left.hpf_hz = 80.0;
    model.tracks[0].builtins.left.lpf_hz = 18_000.0;
    model.tracks[0].builtins.right.hpf_hz = 140.0;
    model.tracks[0].builtins.right.lpf_hz = 12_000.0;
    model.tracks[0].fader.left_db = -3.0;
    model.tracks[0].fader.right_db = 2.0;
    model
}

fn asymmetric_session() -> session::SessionModel {
    let mut model = identity_session();
    model.tracks[0].builtins.left.hpf_hz = 80.0;
    model.tracks[0].builtins.left.lpf_hz = 18_000.0;
    model.tracks[0].builtins.right.hpf_hz = 140.0;
    model.tracks[0].builtins.right.lpf_hz = 12_000.0;
    model.tracks[0].fader.left_db = -3.0;
    model.tracks[0].fader.right_db = 2.0;
    model
}

fn canonical_model(model: &session::SessionModel) -> String {
    canonical_session_json(model).expect("test model canonicalizes")
}

fn pairs() -> Vec<(String, String)> {
    vec![(OLD.to_owned(), NEW.to_owned())]
}

#[test]
fn empty_map_is_an_exact_noop_and_noncanonical_input_is_refused() {
    let input = canonical_model(&identity_session());
    assert_eq!(fold_mono_session_document(&input, &[]).unwrap(), input);
    assert!(fold_mono_session_document(&input.replace('\n', " "), &[]).is_err());
}

#[test]
fn all_stereo_track_channel_pairs_fold_without_touching_other_model_values() {
    let mut model = identity_session();
    let second_source_id = StableId::parse("fixture-source-two").unwrap();
    let second_track_id = StableId::parse("track-two").unwrap();
    let third_track_id = StableId::parse("track-three").unwrap();
    let fourth_track_id = StableId::parse("track-four").unwrap();
    let mut second_source = model.sources[0].clone();
    second_source.id = second_source_id.clone();
    model.sources.push(second_source);
    let base_track = model.tracks[0].clone();
    let base_route = model.routes[0].clone();
    for (track_id, left, right, route_id) in [
        (second_track_id.clone(), 1, 0, "track-two-main"),
        (third_track_id.clone(), 0, 0, "track-three-main"),
        (fourth_track_id.clone(), 1, 1, "track-four-main"),
    ] {
        let mut track = base_track.clone();
        track.id = track_id.clone();
        track.source_id = second_source_id.clone();
        track.left_source_channel = left;
        track.right_source_channel = right;
        model.tracks.push(track);
        let mut route = base_route.clone();
        route.id = StableId::parse(route_id).unwrap();
        route.source = RouteSource::Track {
            track_id,
            tap: session::SendTap::PostMatrix,
        };
        model.routes.push(route);
    }
    let before = canonical_model(&model);
    let transformed = fold_mono_session_document(&before, &pairs()).unwrap();
    let after = parse_session_json(&transformed).unwrap();
    assert_eq!(after.revision, model.revision + 1);
    assert_eq!(after.sources.len(), model.sources.len());
    assert!(after.sources.iter().all(|source| {
        source.content == NEW
            && source.channels == 1
            && source.bit_depth == SourceBitDepth::Pcm24
            && source.frames == 256
    }));
    assert!(
        after
            .tracks
            .iter()
            .all(|track| track.left_source_channel == 0 && track.right_source_channel == 0)
    );
    assert_eq!(
        after.tracks[0].builtins.left.trim_db.to_bits(),
        3.0_f32.to_bits()
    );
    assert_eq!(
        after.tracks[0].builtins.right.trim_db.to_bits(),
        (-3.0_f32).to_bits()
    );
    assert_eq!(
        after.tracks[0].fader.left_db.to_bits(),
        (-0.0_f32).to_bits()
    );
    assert_eq!(after.tracks[0].matrix_or_pan, model.tracks[0].matrix_or_pan);

    // Restore precisely the permitted source/map/revision fields and demand the original bytes;
    // this catches preservation regressions that ordinary numeric equality (including -0.0) hides.
    let mut restored = after;
    restored.revision = model.revision;
    for source in &mut restored.sources {
        source.content = OLD.to_owned();
        source.channels = 2;
    }
    for track in &mut restored.tracks {
        let original = model
            .tracks
            .iter()
            .find(|candidate| candidate.id == track.id)
            .expect("restored track has original");
        track.left_source_channel = original.left_source_channel;
        track.right_source_channel = original.right_source_channel;
    }
    assert_eq!(canonical_model(&restored), before);
}

#[test]
fn mappings_are_simultaneous_and_resulting_shapes_must_converge() {
    let mut model = identity_session();
    let existing_mono_id = StableId::parse("existing-mono").unwrap();
    let mut existing_mono = model.sources[0].clone();
    existing_mono.id = existing_mono_id;
    existing_mono.content = NEW.to_owned();
    existing_mono.channels = 1;
    model.sources.push(existing_mono);
    let input = canonical_model(&model);
    let transformed = fold_mono_session_document(&input, &pairs()).unwrap();
    let output = parse_session_json(&transformed).unwrap();
    assert_eq!(output.sources.len(), 2);
    assert!(output.sources.iter().all(|source| source.content == NEW));

    let mut chain_model = identity_session();
    let mut mapped_source = chain_model.sources[0].clone();
    mapped_source.id = StableId::parse("mapped-source").unwrap();
    mapped_source.content = NEW.to_owned();
    chain_model.sources.push(mapped_source);
    let chain_input = canonical_model(&chain_model);
    let chain = fold_mono_session_document(
        &chain_input,
        &[
            (OLD.to_owned(), NEW.to_owned()),
            (
                NEW.to_owned(),
                "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                    .to_owned(),
            ),
        ],
    )
    .unwrap();
    let chain_output = parse_session_json(&chain).unwrap();
    assert!(
        chain_output
            .sources
            .iter()
            .any(|source| source.content == NEW)
    );
}

#[test]
fn mapped_source_shape_and_map_errors_are_transactional() {
    let mut model = identity_session();
    let input = canonical_model(&model);
    for replacement in [
        (
            "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            NEW,
        ),
        (OLD, OLD),
        (
            "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            NEW,
        ),
    ] {
        assert!(
            fold_mono_session_document(
                &input,
                &[(replacement.0.to_owned(), replacement.1.to_owned())]
            )
            .is_err()
        );
    }
    model.sources[0].channels = 1;
    model.tracks[0].right_source_channel = 0;
    let mono_input = canonical_model(&model);
    assert!(fold_mono_session_document(&mono_input, &pairs()).is_err());
    model.sources[0].channels = 2;
    model.sources[0].bit_depth = SourceBitDepth::Float32;
    let float_input = canonical_model(&model);
    assert!(fold_mono_session_document(&float_input, &pairs()).is_err());
    model.sources[0].bit_depth = SourceBitDepth::Pcm24;
    model.revision = u64::MAX;
    let overflow_input = canonical_model(&model);
    assert!(fold_mono_session_document(&overflow_input, &pairs()).is_err());
}

fn render_two_blocks(document: &str, mono: bool) -> Vec<f32> {
    let mut host = AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: 512,
            ..WebBootOptions::explicit_defaults()
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "boot failed: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    });
    let mut output = Vec::new();
    let quantum = host.status().quantum_frames as usize;
    for block in 0..2 {
        let plane: Vec<f32> = (0..quantum)
            .map(|index| (index + block * quantum) as f32 * 0.013 - 0.5)
            .collect();
        let planes: Vec<&[f32]> = if mono {
            vec![&plane]
        } else {
            vec![&plane, &plane]
        };
        let submit = host.submit_source(
            b"fixture-source",
            1,
            (block * quantum) as u64,
            host.status().sample_rate_hz,
            &planes,
            quantum as u32,
            block == 1,
        );
        assert_eq!(submit, RESULT_OK, "submit failed: {:?}", host.diagnostic());
        assert_eq!(host.render_next(), RESULT_OK);
        output.extend_from_slice(host.output_pcm().expect("render output"));
    }
    output
}

#[test]
fn native_render_is_bit_identical_for_duplicate_stereo_and_prepared_mono() {
    let input = canonical_model(&asymmetric_session());
    let transformed = fold_mono_session_document(&input, &pairs()).unwrap();
    let stereo = render_two_blocks(&input, false);
    let mono = render_two_blocks(&transformed, true);
    assert!(stereo.iter().any(|sample| *sample != 0.0));
    assert!(
        stereo
            .chunks_exact(2 * 128)
            .flat_map(|block| block[..128].iter().zip(&block[128..]))
            .any(|(left, right)| left.to_bits() != right.to_bits())
    );
    assert_eq!(
        stereo
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        mono.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
    );
}

#[test]
fn native_effect_fixture_is_bit_identical_after_mono_preparation() {
    let input = canonical_model(&native_effect_session());
    let transformed = fold_mono_session_document(&input, &pairs()).unwrap();
    let stereo = render_two_blocks(&input, false);
    let mono = render_two_blocks(&transformed, true);
    assert!(stereo.iter().any(|sample| *sample != 0.0));
    assert_eq!(
        stereo
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        mono.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
    );
}

#[test]
fn cli_rejects_non_lf_maps_without_writing_document() {
    let input_path = std::env::temp_dir().join(format!(
        "session-validator-fold-input-{}.json",
        std::process::id()
    ));
    let map_path = std::env::temp_dir().join(format!(
        "session-validator-fold-map-{}.tsv",
        std::process::id()
    ));
    std::fs::write(&input_path, canonical_model(&identity_session())).unwrap();
    std::fs::write(&map_path, format!("{OLD}\t{NEW}")).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_session_validator"))
        .args(["fold-mono", "--map"])
        .arg(&map_path)
        .arg(&input_path)
        .output()
        .unwrap();
    std::fs::remove_file(input_path).unwrap();
    std::fs::remove_file(map_path).unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("LF-terminated"));
}

#[test]
fn cli_enforces_bounded_map_entries_and_session_input() {
    let map_path = std::env::temp_dir().join(format!(
        "session-validator-fold-limit-map-{}.tsv",
        std::process::id()
    ));
    let input_path = std::env::temp_dir().join(format!(
        "session-validator-fold-limit-input-{}.json",
        std::process::id()
    ));
    let mut oversized_map = String::new();
    for index in 0..=FOLD_MONO_MAX_ENTRIES {
        oversized_map.push_str(&format!("sha256:{index:064x}\tsha256:{:064x}\n", index + 1));
    }
    std::fs::write(&map_path, oversized_map).unwrap();
    std::fs::write(&input_path, canonical_model(&identity_session())).unwrap();
    let map_output = std::process::Command::new(env!("CARGO_BIN_EXE_session_validator"))
        .args(["fold-mono", "--map"])
        .arg(&map_path)
        .arg(&input_path)
        .output()
        .unwrap();
    assert_eq!(map_output.status.code(), Some(1));
    assert!(map_output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&map_output.stderr).contains("entry limit"));

    std::fs::write(&map_path, b"").unwrap();
    std::fs::write(
        &input_path,
        vec![b' '; FOLD_MONO_MAX_SESSION_BYTES.saturating_add(1)],
    )
    .unwrap();
    let input_output = std::process::Command::new(env!("CARGO_BIN_EXE_session_validator"))
        .args(["fold-mono", "--map"])
        .arg(&map_path)
        .arg(&input_path)
        .output()
        .unwrap();
    std::fs::remove_file(map_path).unwrap();
    std::fs::remove_file(input_path).unwrap();
    assert_eq!(input_output.status.code(), Some(2));
    assert!(input_output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&input_output.stderr).contains("byte limit"));
}
