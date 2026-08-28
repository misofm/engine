//! Native decoder checks against the one #241 canonical-PCM corpus.

use std::{fs, path::Path};

use miso_engine_flac_decoder::{
    RESULT_DECODE_REFUSED, RESULT_RESOURCE_LIMIT, RESULT_SHAPE_MISMATCH, decode_flac_to_writer,
};
use miso_engine_stem_hasher::{CanonicalBitDepth, CanonicalPcmShape, canonicalize_raw_pcm};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/flac-delivery/v1"
);
const IDENTITY_FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/stem-identity/v1"
);

#[derive(Debug)]
struct Vector<'a> {
    vector: &'a str,
    bit_depth: u16,
    channels: u16,
    frames: u64,
    configured_block_frames: u16,
    identity: &'a str,
    pcm_file: &'a str,
    flac_file: &'a str,
}

#[test]
fn every_flac_variant_decodes_to_the_shared_canonical_pcm_pin() {
    let manifest = fs::read_to_string(Path::new(FIXTURES).join("FLAC_VECTORS.tsv"))
        .expect("FLAC vector manifest");
    let vectors = parse_vectors(&manifest);
    assert_eq!(
        vectors.len(),
        8,
        "four shared vectors times two block sizes"
    );
    assert_eq!(
        vectors
            .iter()
            .map(|vector| vector.configured_block_frames)
            .collect::<std::collections::BTreeSet<_>>(),
        [32, 4096].into_iter().collect()
    );
    for vector in vectors {
        let encoded = fs::read(Path::new(FIXTURES).join(vector.flac_file)).expect("FLAC fixture");
        let expected =
            fs::read(Path::new(IDENTITY_FIXTURES).join(vector.pcm_file)).expect("shared PCM");
        let mut actual = Vec::new();
        let report = decode_flac_to_writer(
            &encoded,
            u64::try_from(expected.len()).expect("fixture length"),
            &mut actual,
        )
        .expect("decode fixture");
        assert_eq!(actual, expected, "{}", vector.flac_file);
        assert_eq!(report.stream.channels, vector.channels);
        assert_eq!(report.stream.bit_depth.bits(), vector.bit_depth);
        assert_eq!(report.stream.frames, vector.frames);
        assert_eq!(
            report.stream.minimum_block_frames, vector.configured_block_frames,
            "STREAMINFO preserves the configured block size even for a short final block"
        );
        assert_eq!(
            report.stream.maximum_block_frames,
            vector.configured_block_frames
        );

        let shape = CanonicalPcmShape::new(
            vector.channels,
            CanonicalBitDepth::try_from(vector.bit_depth).expect("depth"),
            vector.frames,
        )
        .expect("shape");
        let identity = canonicalize_raw_pcm(&mut &actual[..], shape, &mut std::io::sink())
            .expect("shared hasher")
            .identity();
        assert_eq!(identity, vector.identity, "{}", vector.vector);
    }
}

#[test]
fn one_lsb_output_mutation_is_detected_by_every_identity_pin() {
    let manifest = fs::read_to_string(Path::new(FIXTURES).join("FLAC_VECTORS.tsv"))
        .expect("FLAC vector manifest");
    for vector in parse_vectors(&manifest) {
        let encoded = fs::read(Path::new(FIXTURES).join(vector.flac_file)).expect("FLAC fixture");
        let mut decoded = Vec::new();
        decode_flac_to_writer(&encoded, u64::MAX, &mut decoded).expect("decode");
        decoded[0] ^= 1;
        let shape = CanonicalPcmShape::new(
            vector.channels,
            CanonicalBitDepth::try_from(vector.bit_depth).expect("depth"),
            vector.frames,
        )
        .expect("shape");
        let mutated = canonicalize_raw_pcm(&mut &decoded[..], shape, &mut std::io::sink())
            .expect("hash mutation")
            .identity();
        assert_ne!(mutated, vector.identity, "{}", vector.flac_file);
    }
}

#[test]
fn budget_and_corrupt_transport_refuse_typed() {
    let manifest = fs::read_to_string(Path::new(FIXTURES).join("FLAC_VECTORS.tsv"))
        .expect("FLAC vector manifest");
    let first = parse_vectors(&manifest).remove(0);
    let mut encoded = fs::read(Path::new(FIXTURES).join(first.flac_file)).expect("FLAC fixture");
    let expected_bytes = first.frames * u64::from(first.channels) * u64::from(first.bit_depth / 8);
    let budget = decode_flac_to_writer(&encoded, expected_bytes - 1, &mut std::io::sink())
        .expect_err("budget refusal");
    assert_eq!(budget.result(), RESULT_RESOURCE_LIMIT);

    encoded.truncate(encoded.len() - 1);
    let corrupt = decode_flac_to_writer(&encoded, u64::MAX, &mut std::io::sink())
        .expect_err("truncated FLAC refusal");
    assert!(
        [RESULT_DECODE_REFUSED, RESULT_SHAPE_MISMATCH].contains(&corrupt.result()),
        "corrupt transport must refuse at syntax/CRC or final shape/MD5"
    );
}

fn parse_vectors(manifest: &str) -> Vec<Vector<'_>> {
    let mut lines = manifest.lines();
    assert_eq!(lines.next(), Some("schema_version\t1"));
    assert_eq!(
        lines.next(),
        Some(
            "vector\tbit_depth\tchannels\tframes\tconfigured_block_frames\tidentity\tpcm_file\tflac_file\tflac_sha256"
        )
    );
    lines
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 9, "{line}");
            Vector {
                vector: fields[0],
                bit_depth: fields[1].parse().expect("bit depth"),
                channels: fields[2].parse().expect("channels"),
                frames: fields[3].parse().expect("frames"),
                configured_block_frames: fields[4].parse().expect("block size"),
                identity: fields[5],
                pcm_file: fields[6],
                flac_file: fields[7],
            }
        })
        .collect()
}
