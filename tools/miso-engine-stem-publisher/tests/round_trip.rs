//! Publisher/client round-trip and transport-identity separation checks.

use std::{fs, path::Path};

use miso_engine_stem_publisher::{publish_wave, run_cli};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/flac-delivery/v1"
);

#[test]
fn every_master_round_trips_and_encoder_settings_move_only_transport() {
    let root = Path::new(FIXTURES);
    for name in [
        "pcm16-mono-boundaries",
        "pcm16-stereo-boundaries",
        "pcm24-mono-boundaries",
        "pcm24-stereo-boundaries",
    ] {
        let temporary = create_temp_dir();
        let master = root.join(format!("masters/{name}.wav"));
        let block32 = temporary.join("block32");
        let block4096 = temporary.join("block4096");
        let first = publish_wave(&master, &block32, 32).expect("publish block 32");
        let second = publish_wave(&master, &block4096, 4096).expect("publish block 4096");
        assert_eq!(first.identity, second.identity, "{name}");
        assert_eq!(first.channels, second.channels);
        assert_eq!(first.bit_depth, second.bit_depth);
        assert_eq!(first.frames, second.frames);
        assert_eq!(first.pcm_bytes, second.pcm_bytes);
        assert_ne!(
            fs::read(block32.join(first.delivery_file)).expect("block32 FLAC"),
            fs::read(block4096.join(second.delivery_file)).expect("block4096 FLAC"),
            "encoder setting red mutation must move transport bytes: {name}"
        );
        fs::remove_dir_all(temporary).expect("remove test directory");
    }
}

#[test]
fn corrupt_delivery_is_refused_before_publish_verification() {
    let root = Path::new(FIXTURES);
    let temporary = create_temp_dir();
    let output = temporary.join("published");
    let master = root.join("masters/pcm24-stereo-boundaries.wav");
    let published = publish_wave(&master, &output, 4096).expect("publish fixture");
    let mut corrupt = fs::read(output.join(published.delivery_file)).expect("delivery");
    corrupt.truncate(corrupt.len() - 1);
    let corrupt_path = temporary.join("corrupt.flac");
    fs::write(&corrupt_path, corrupt).expect("corrupt fixture");
    let error = run_cli([
        "verify".into(),
        "--master".into(),
        master.into_os_string(),
        "--flac".into(),
        corrupt_path.into_os_string(),
    ])
    .expect_err("corruption cannot publish");
    assert_eq!(error.code(), "round_trip.decode.refused");
    fs::remove_dir_all(temporary).expect("remove test directory");
}

#[test]
fn catalog_row_carries_shape_and_provenance_but_not_transport_identity() {
    let root = Path::new(FIXTURES);
    let temporary = create_temp_dir();
    let output = temporary.join("published");
    let published = publish_wave(
        &root.join("masters/pcm16-mono-boundaries.wav"),
        &output,
        4096,
    )
    .expect("publish fixture");
    let row = fs::read_to_string(output.join("catalog-row.json")).expect("catalog row");
    for key in [
        "\"identity\"",
        "\"channels\"",
        "\"bit_depth\"",
        "\"frames\"",
        "\"pcm_bytes\"",
        "\"flac_bytes\"",
        "\"encoder\"",
        "\"decoder\"",
        "\"artifact_sha256\"",
    ] {
        assert!(row.contains(key), "missing {key}");
    }
    assert!(row.contains(&published.identity));
    assert!(!row.contains("flac_identity"));
    assert!(!row.contains("flac_sha256"));
    fs::remove_dir_all(temporary).expect("remove test directory");
}

fn create_temp_dir() -> std::path::PathBuf {
    for nonce in 0_u32..100 {
        let path = std::env::temp_dir().join(format!(
            "miso-engine-stem-publisher-test-{}-{nonce}",
            std::process::id()
        ));
        if fs::create_dir(&path).is_ok() {
            return path;
        }
    }
    panic!("could not create test directory");
}
