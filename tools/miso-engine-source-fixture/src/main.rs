//! Generated/checksummed source fixture corpus checker V1.
//!
//! This first checkpoint intentionally owns one independently-oracled RIFF PCM16 stereo fixture.
//! Later issue-010 checkpoints extend this one tool and manifest rather than introducing another
//! fixture framework.

use std::io::Cursor;

use miso_engine_core::SampleRateHz;
use miso_engine_source::{
    NativeWaveDecoder, NativeWaveParseCaps, NativeWaveRegion, SourceFrame, parse_native_wave,
};
use sha2::{Digest, Sha256};

const MANIFEST: &str = include_str!("../../../fixtures/sources/v1/manifest.sha256");
const FIXTURE_ID: &str = "riff-pcm16-stereo-v1";
const CAPS: NativeWaveParseCaps = NativeWaveParseCaps {
    max_chunk_count: 8,
    max_skipped_metadata_bytes: 0,
};

fn main() {
    if let Err(error) = check() {
        eprintln!("source fixture check failed: {error}");
        std::process::exit(1);
    }
}

fn check() -> Result<(), String> {
    let fixture = generated_riff_pcm16_stereo();
    let expected_checksum = manifest_checksum(FIXTURE_ID)?;
    let actual_checksum = sha256_hex(&fixture);
    if expected_checksum != actual_checksum {
        return Err(format!(
            "{FIXTURE_ID} checksum mismatch: expected {expected_checksum}, actual {actual_checksum}"
        ));
    }
    independently_oracled_decode(&fixture)?;
    corruption_rejects(&fixture)?;
    Ok(())
}

fn manifest_checksum(id: &str) -> Result<&'static str, String> {
    let mut prior = "";
    let mut found = None;
    for line in MANIFEST.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (checksum, fixture_id) = line
            .split_once("  ")
            .ok_or_else(|| format!("invalid manifest line {line:?}"))?;
        if checksum.len() != 64 || !checksum.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!("invalid SHA-256 for {fixture_id}"));
        }
        if fixture_id <= prior {
            return Err("manifest fixture IDs must be strictly sorted".to_owned());
        }
        prior = fixture_id;
        if fixture_id == id {
            found = Some(checksum);
        }
    }
    found.ok_or_else(|| format!("manifest omits {id}"))
}

fn independently_oracled_decode(fixture: &[u8]) -> Result<(), String> {
    let mut cursor = Cursor::new(fixture);
    let metadata =
        parse_native_wave(&mut cursor, CAPS).map_err(|error| format!("parse: {error:?}"))?;
    if metadata.sample_rate_hz != SampleRateHz(48_000) || metadata.channel_count != 2 {
        return Err("metadata differs from independent fixture declaration".to_owned());
    }
    let mut decoder = NativeWaveDecoder::prepare(
        cursor,
        metadata,
        NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 2,
        },
        std::num::NonZeroUsize::new(2).expect("two"),
    )
    .map_err(|error| format!("decoder: {error:?}"))?;
    let mut left = [0.0_f32; 2];
    let mut right = [0.0_f32; 2];
    let report = {
        let mut planes = [&mut left[..], &mut right[..]];
        decoder
            .decode_into(&mut planes)
            .map_err(|error| format!("decode: {error:?}"))?
    };
    let expected_left = [(-1.0_f32).to_bits(), 0];
    let expected_right = [(32_767.0_f32 / 32_768.0).to_bits(), (-0.5_f32).to_bits()];
    if left.map(f32::to_bits) != expected_left || right.map(f32::to_bits) != expected_right {
        return Err("production PCM differs from the independent oracle".to_owned());
    }
    if report.sanitized_sample_count != 0 || !report.end_of_region {
        return Err("unexpected PCM sanitation or region status".to_owned());
    }
    Ok(())
}

fn corruption_rejects(fixture: &[u8]) -> Result<(), String> {
    let mut corrupted = fixture.to_vec();
    corrupted[..4].copy_from_slice(b"RIFX");
    if parse_native_wave(&mut Cursor::new(corrupted), CAPS).is_ok() {
        return Err("big-endian RIFX corruption unexpectedly parsed".to_owned());
    }
    Ok(())
}

fn generated_riff_pcm16_stereo() -> Vec<u8> {
    let format = [
        1_u16.to_le_bytes().to_vec(),
        2_u16.to_le_bytes().to_vec(),
        48_000_u32.to_le_bytes().to_vec(),
        192_000_u32.to_le_bytes().to_vec(),
        4_u16.to_le_bytes().to_vec(),
        16_u16.to_le_bytes().to_vec(),
    ]
    .concat();
    let pcm = [
        i16::MIN.to_le_bytes(),
        i16::MAX.to_le_bytes(),
        0_i16.to_le_bytes(),
        (-16_384_i16).to_le_bytes(),
    ]
    .concat();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    append_chunk(&mut bytes, b"fmt ", &format);
    append_chunk(&mut bytes, b"data", &pcm);
    let riff_size = u32::try_from(bytes.len() - 8).expect("small fixture");
    bytes[4..8].copy_from_slice(&riff_size.to_le_bytes());
    bytes
}

fn append_chunk(bytes: &mut Vec<u8>, id: &[u8; 4], payload: &[u8]) {
    bytes.extend_from_slice(id);
    bytes.extend_from_slice(
        &u32::try_from(payload.len())
            .expect("small fixture")
            .to_le_bytes(),
    );
    bytes.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        bytes.push(0);
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_fixture_matches_manifest_oracle_and_corruption_policy() {
        check().expect("fixture check");
    }
}
