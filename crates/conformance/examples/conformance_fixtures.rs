//! Generates or verifies the small checked-in issue-002 fixture corpus.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use conformance::{PcmFixture, SplitMix64, parse_manifest};
use engine::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES, SampleRateHz};

fn main() {
    let write = match env::args().nth(1).as_deref() {
        Some("--write") => true,
        Some("--check") | None => false,
        _ => panic!("usage: conformance_fixtures [--check|--write]"),
    };
    let root = PathBuf::from("fixtures/conformance");
    let files = generated();
    if write {
        fs::create_dir_all(root.join("v1")).expect("create fixture directory");
        for (name, bytes) in &files {
            fs::write(root.join("v1").join(name), bytes).expect("write fixture");
        }
        fs::write(root.join("MANIFEST.tsv"), manifest(&files)).expect("write manifest");
    }
    verify(&root, &files);
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let mut result = Vec::new();
    for rate in LAUNCH_SAMPLE_RATES
        .into_iter()
        .chain(EXTENDED_COMPATIBILITY_SAMPLE_RATES)
        .map(|rate| rate.0)
    {
        let mut samples = vec![0.0_f32; 2 * 128];
        samples[7] = 1.0;
        samples[128 + 19] = -0.625;
        result.push((
            format!("rate-{rate:06}-impulse-dual-mono.mepcm"),
            PcmFixture::encode(SampleRateHz(rate), 2, 128, &samples).expect("valid impulse"),
        ));
    }
    let mut noise = Vec::with_capacity(512);
    let mut prng = SplitMix64::default();
    for _ in 0..512 {
        noise.push(prng.next_bipolar_f32());
    }
    result.push((
        "prng-noise-048000-dual-mono.mepcm".to_owned(),
        PcmFixture::encode(SampleRateHz(48_000), 2, 256, &noise).expect("valid noise"),
    ));
    let sine = (0..256)
        .map(|n| (core::f32::consts::TAU * 997.0 * n as f32 / 48_000.0).sin())
        .collect::<Vec<_>>();
    result.push((
        "sine-048000-mono.mepcm".to_owned(),
        PcmFixture::encode(SampleRateHz(48_000), 1, 256, &sine).expect("valid sine"),
    ));
    let mut multitone = Vec::with_capacity(512);
    for channel in 0..2 {
        for n in 0..256 {
            let phase = channel as f32 * 0.17;
            multitone.push(
                0.45 * (core::f32::consts::TAU * 41_000.0 * n as f32 / 96_000.0 + phase).sin()
                    + 0.25
                        * (core::f32::consts::TAU * 43_123.0 * n as f32 / 96_000.0 + phase).sin(),
            );
        }
    }
    result.push((
        "multitone-near-nyquist-096000-dual-mono.mepcm".to_owned(),
        PcmFixture::encode(SampleRateHz(96_000), 2, 256, &multitone).expect("valid multitone"),
    ));
    result.sort_by(|left, right| left.0.cmp(&right.0));
    result
}

fn manifest(files: &[(String, Vec<u8>)]) -> String {
    let mut out = String::from("miso-engine-fixture-manifest-v1\n");
    for (name, bytes) in files {
        let fixture = PcmFixture::parse(bytes, Default::default()).expect("generated fixture");
        out.push_str(&format!(
            "{:08x}\t{}\tv1/{name}\n",
            fixture.checksum(),
            bytes.len()
        ));
    }
    out
}

fn verify(root: &Path, expected: &[(String, Vec<u8>)]) {
    let manifest_bytes = fs::read(root.join("MANIFEST.tsv")).expect("read manifest");
    let entries = parse_manifest(&manifest_bytes).expect("valid manifest");
    assert_eq!(entries.len(), expected.len());
    for ((name, generated), entry) in expected.iter().zip(entries) {
        assert_eq!(entry.path, format!("v1/{name}"));
        let bytes = fs::read(root.join(&entry.path)).expect("listed fixture");
        assert_eq!(bytes, *generated);
        assert_eq!(entry.length, bytes.len());
        let fixture = PcmFixture::parse(&bytes, Default::default()).expect("valid fixture");
        assert_eq!(entry.crc32c, fixture.checksum());
    }
    let mut disk = fs::read_dir(root.join("v1"))
        .expect("read v1")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .into_string()
                .expect("utf8")
        })
        .collect::<Vec<_>>();
    disk.sort();
    assert_eq!(
        disk,
        expected
            .iter()
            .map(|item| item.0.clone())
            .collect::<Vec<_>>()
    );
}
