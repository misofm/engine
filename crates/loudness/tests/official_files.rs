//! ITU-R BS.2217-2's complete mono/stereo subset, using the unmodified official PCM files.
//! Acquisition, licensing, exclusions and the explicit qualification command: data/README.md.

mod common;

use std::{env, fs, path::Path, process::Command};

use common::{BLOCK, assert_within};
use loudness::{LoudnessAnalysis, LoudnessObserver};

const INVENTORY: &str = include_str!("data/official-loudness.tsv");

fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(bytes[at..at + 2].try_into().expect("u16"))
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().expect("u32"))
}

struct Pcm<'a> {
    rate: u32,
    channels: usize,
    samples: &'a [u8],
}

/// The corpus uses little-endian signed 16-bit PCM, including two extensible PCM headers.
/// Unknown RIFF metadata is skipped by chunk length; only the original data bytes are decoded.
fn pcm(bytes: &[u8], riff_length_delta: usize) -> Pcm<'_> {
    assert!(bytes.len() >= 12, "truncated RIFF");
    assert_eq!(&bytes[..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(
        u32_at(bytes, 4) as usize + 8 + riff_length_delta,
        bytes.len(),
        "pinned official RIFF length delta"
    );
    let (mut format, mut samples) = (None, None);
    let mut offset = 12;
    while offset < bytes.len() {
        assert!(bytes.len() - offset >= 8, "truncated RIFF chunk header");
        let length = u32_at(bytes, offset + 4) as usize;
        let start = offset + 8;
        let end = start.checked_add(length).expect("RIFF chunk length");
        let chunk = bytes.get(start..end).expect("RIFF chunk bounds");
        match &bytes[offset..offset + 4] {
            b"fmt " => assert!(format.replace(chunk).is_none(), "duplicate fmt"),
            b"data" => assert!(samples.replace(chunk).is_none(), "duplicate data"),
            _ => {}
        }
        offset = end + length % 2;
        assert!(offset <= bytes.len(), "missing RIFF padding");
    }
    let format = format.expect("PCM format");
    assert!(format.len() >= 16, "truncated PCM format");
    if u16_at(format, 0) == 0xfffe {
        assert_eq!(format.len(), 40);
        assert_eq!(u16_at(format, 16), 22, "extensible size");
        assert_eq!(u16_at(format, 18), 16, "valid PCM bits");
        assert_eq!(
            &format[24..40],
            &[1, 0, 0, 0, 0, 0, 16, 0, 128, 0, 0, 170, 0, 56, 155, 113],
            "PCM subtype GUID"
        );
    } else {
        assert_eq!(u16_at(format, 0), 1, "integer PCM");
    }
    let channels = usize::from(u16_at(format, 2));
    assert!(matches!(channels, 1 | 2), "mono/stereo only");
    assert_eq!(u16_at(format, 14), 16, "PCM bits");
    let rate = u32_at(format, 4);
    let align = u16_at(format, 12);
    assert_eq!(usize::from(align), channels * 2);
    assert_eq!(u32_at(format, 8), rate * u32::from(align));
    let samples = samples.expect("PCM data");
    assert_eq!(samples.len() % usize::from(align), 0, "complete PCM frames");
    Pcm {
        rate,
        channels,
        samples,
    }
}

fn measure(pcm: &Pcm<'_>) -> Option<f64> {
    let mut observer = LoudnessObserver::new(pcm.rate).expect("native launch rate");
    let mut analysis = LoudnessAnalysis::new();
    let (mut left, mut right) = ([0.0; BLOCK], [0.0; BLOCK]);
    let mut first_sample = 0;
    let frame_bytes = pcm.channels * 2;
    for block in pcm.samples.chunks(BLOCK * frame_bytes) {
        let frames = block.len() / frame_bytes;
        for (index, frame) in block.chunks_exact(frame_bytes).enumerate() {
            left[index] = f32::from(i16::from_le_bytes([frame[0], frame[1]])) / 32768.0;
            // BS.2217-2 gives mono and each stereo channel unit energy weighting. Left plus
            // zero right preserves the documented mono expectation without duplicating power.
            right[index] = if pcm.channels == 2 {
                f32::from(i16::from_le_bytes([frame[2], frame[3]])) / 32768.0
            } else {
                0.0
            };
        }
        observer.observe(
            &left[..frames],
            &right[..frames],
            first_sample,
            &mut |record| analysis.ingest(&record),
        );
        first_sample += frames as u64;
    }
    // File-based integration ends exactly at EOF: no invented padding, resampling or gain.
    analysis.integrated()
}

#[test]
#[ignore = "external official ITU audio: acquire with data/fetch-official-loudness.py; see data/README.md"]
fn itu_bs2217_published_mono_stereo_files() {
    let directory = env::var_os("MISO_LOUDNESS_FIXTURES")
        .expect("set MISO_LOUDNESS_FIXTURES to the verified external corpus directory");
    // Sequential, offline verification is part of the explicit gate, not an optional pre-step.
    // Verify every required file before measuring any; missing/corrupt cases cannot become skips.
    let verification = Command::new("python3")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/fetch-official-loudness.py"))
        .arg(&directory)
        .arg("--verify")
        .output()
        .expect("run the offline fixture verifier with Python 3");
    assert!(
        verification.status.success(),
        "fixture verification failed:\n{}\n{}",
        String::from_utf8_lossy(&verification.stdout),
        String::from_utf8_lossy(&verification.stderr)
    );
    let mut count = 0;
    for row in INVENTORY.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 14, "fixture manifest columns");
        if fields[1] != "required" {
            continue;
        }
        let bytes = fs::read(Path::new(&directory).join(fields[11])).expect("required WAV");
        let pcm = pcm(&bytes, fields[13].parse().expect("RIFF length delta"));
        assert_eq!(pcm.channels, fields[3].parse::<usize>().expect("channels"));
        assert_eq!(pcm.rate, fields[4].parse::<u32>().expect("native rate"));
        assert_eq!(
            pcm.samples.len() / (pcm.channels * 2),
            fields[6].parse::<usize>().expect("frames")
        );
        let expected = fields[7].parse::<f64>().expect("official I");
        let tolerance = fields[8].parse::<f64>().expect("official tolerance");
        let reading = measure(&pcm);
        eprintln!(
            "{}: I={reading:?} LKFS, expected {expected} +/- {tolerance}, {} Hz, {} channel(s)",
            fields[11], pcm.rate, pcm.channels
        );
        assert_within(fields[11], reading, expected, tolerance);
        count += 1;
    }
    assert_eq!(count, 19, "all published mono/stereo cases measured");
}
