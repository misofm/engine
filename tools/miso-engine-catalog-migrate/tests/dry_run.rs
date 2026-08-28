//! Fixture mini-catalog dry-run and pinned mapping checks.

use std::{fs, path::Path};

use miso_engine_catalog_migrate::check_catalog;
use miso_engine_flac_decoder::decode_flac_to_writer;
use miso_engine_stem_hasher::{CanonicalBitDepth, CanonicalPcmShape, canonicalize_raw_pcm};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/flac-delivery/v1"
);

#[test]
fn mini_catalog_reproduces_the_pinned_one_way_oracle() {
    let root = Path::new(FIXTURES);
    check_catalog(
        &root.join("mini-catalog/catalog.tsv"),
        &root.join("mini-catalog/embeddings.tsv"),
        &root.join("mini-catalog/expected"),
    )
    .expect("pinned dry-run");
}

#[test]
fn migrated_documents_are_canonical_and_real_delivery_bytes_verify() {
    let root = Path::new(FIXTURES);
    let manifest = fs::read_to_string(root.join("mini-catalog/expected/manifest.tsv"))
        .expect("pinned manifest");
    let mut identities = Vec::new();
    for line in manifest.lines().skip(2) {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 6);
        let name = fields[0];
        let identity = fields[1];
        assert!(identity_format(identity));
        let channels = fields[2].parse::<u16>().expect("channels");
        let bit_depth = match fields[3] {
            "16" => CanonicalBitDepth::Pcm16,
            "24" => CanonicalBitDepth::Pcm24,
            _ => panic!("non-launch bit depth in migrated manifest"),
        };
        let frames = fields[4].parse::<u64>().expect("frames");
        let pcm_bytes = fields[5].parse::<u64>().expect("PCM bytes");
        let flac = fs::read(root.join(format!("flac/{name}-b4096.flac")))
            .expect("real FLAC delivery object");
        let mut decoded = Vec::new();
        let decoded_report = decode_flac_to_writer(&flac, pcm_bytes, &mut decoded)
            .expect("shipped decoder accepts migrated delivery");
        assert_eq!(decoded_report.stream.channels, channels);
        assert_eq!(decoded_report.stream.bit_depth.bits(), bit_depth.bits());
        assert_eq!(decoded_report.stream.frames, frames);
        let shape = CanonicalPcmShape {
            channels,
            bit_depth,
            frames,
        };
        let recomputed = canonicalize_raw_pcm(&mut &decoded[..], shape, &mut std::io::sink())
            .expect("shared hasher accepts decoded PCM");
        assert_eq!(recomputed.identity(), identity);
        identities.push(identity);
    }

    let replacements =
        fs::read_to_string(root.join("mini-catalog/expected/document-replacements.tsv"))
            .expect("pinned document replacements");
    for line in replacements.lines().skip(2) {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 4);
        assert!(identity_format(fields[2]));
        assert!(identity_format(fields[3]));
        assert!(identities.contains(&fields[3]));
    }
}

#[test]
fn container_hash_mutation_makes_the_mapping_gate_red() {
    let root = Path::new(FIXTURES);
    let original = fs::read_to_string(root.join("mini-catalog/catalog.tsv")).expect("catalog");
    let mut lines = original.lines();
    let mut mutated = format!("{}\n{}\n", lines.next().unwrap(), lines.next().unwrap());
    for (index, line) in lines.enumerate() {
        let mut fields = line.split('\t').collect::<Vec<_>>();
        if index == 0 {
            fields[1] = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
        }
        mutated.push_str(&fields.join("\t"));
        mutated.push('\n');
    }
    let temporary = create_temp_dir();
    let catalog = temporary.join("catalog.tsv");
    fs::write(&catalog, mutated).expect("write mutation");
    let error = check_catalog(
        &catalog,
        &root.join("mini-catalog/embeddings.tsv"),
        &root.join("mini-catalog/expected"),
    )
    .expect_err("container-hash mutation must be red");
    assert!(["old_identity.container_mismatch", "embeddings.row.invalid"].contains(&error.code()));
    fs::remove_dir_all(temporary).expect("remove test directory");
}

fn create_temp_dir() -> std::path::PathBuf {
    for nonce in 0_u32..100 {
        let path = std::env::temp_dir().join(format!(
            "miso-engine-catalog-migrate-test-{}-{nonce}",
            std::process::id()
        ));
        if fs::create_dir(&path).is_ok() {
            return path;
        }
    }
    panic!("could not create test directory");
}

fn identity_format(identity: &str) -> bool {
    identity.len() == 71
        && identity.starts_with("sha256:")
        && identity[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
