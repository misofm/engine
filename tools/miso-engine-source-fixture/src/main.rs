//! Generated/checksummed source fixture corpus checker V1.
//!
//! The fixture bytes and expected planar `f32` bits are intentionally assembled independently from
//! the production parser/decoder. This remains the one source fixture generator/checker tool.

use std::{collections::BTreeMap, io::Cursor, num::NonZeroUsize};

use miso_engine_core::SampleRateHz;
use miso_engine_source::{
    NativeWaveDecoder, NativeWaveParseCaps, NativeWaveRegion, SourceFrame, parse_native_wave,
};
use sha2::{Digest, Sha256};

const MANIFEST: &str = include_str!("../../../fixtures/sources/v1/manifest.sha256");
const CAPS: NativeWaveParseCaps = NativeWaveParseCaps {
    max_chunk_count: 16,
    max_skipped_metadata_bytes: 16,
};

struct FixtureCase {
    id: &'static str,
    bytes: Vec<u8>,
    channels: u16,
    expected_planes: Vec<Vec<u32>>,
    expected_sanitized: u64,
}

fn main() {
    if let Err(error) = check() {
        eprintln!("source fixture check failed: {error}");
        std::process::exit(1);
    }
}

fn check() -> Result<(), String> {
    let manifest = manifest_checksums()?;
    let fixtures = generated_fixtures();
    if manifest.len() != fixtures.len() {
        return Err(format!(
            "manifest has {} fixtures but generator has {}",
            manifest.len(),
            fixtures.len()
        ));
    }

    let mut failures = Vec::new();
    for fixture in &fixtures {
        let actual_checksum = sha256_hex(&fixture.bytes);
        match manifest.get(fixture.id) {
            Some(expected_checksum) if expected_checksum == &actual_checksum => {}
            Some(expected_checksum) => failures.push(format!(
                "{} checksum mismatch: expected {expected_checksum}, actual {actual_checksum}",
                fixture.id
            )),
            None => failures.push(format!("manifest omits {}", fixture.id)),
        }
        if let Err(error) = independently_oracled_decode(fixture) {
            failures.push(format!("{} oracle: {error}", fixture.id));
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("; "));
    }
    corruption_rejects(&fixtures)?;
    Ok(())
}

fn manifest_checksums() -> Result<BTreeMap<&'static str, &'static str>, String> {
    let mut prior = "";
    let mut checksums = BTreeMap::new();
    for line in MANIFEST.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (checksum, fixture_id) = line
            .split_once("  ")
            .ok_or_else(|| format!("invalid manifest line {line:?}"))?;
        if checksum.len() != 64
            || !checksum
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte.is_ascii_lowercase())
        {
            return Err(format!("invalid lowercase SHA-256 for {fixture_id}"));
        }
        if fixture_id <= prior {
            return Err("manifest fixture IDs must be strictly sorted".to_owned());
        }
        prior = fixture_id;
        if checksums.insert(fixture_id, checksum).is_some() {
            return Err(format!("duplicate manifest fixture ID {fixture_id}"));
        }
    }
    Ok(checksums)
}

fn independently_oracled_decode(fixture: &FixtureCase) -> Result<(), String> {
    let mut cursor = Cursor::new(&fixture.bytes);
    let metadata =
        parse_native_wave(&mut cursor, CAPS).map_err(|error| format!("parse: {error:?}"))?;
    if metadata.sample_rate_hz != SampleRateHz(48_000)
        || metadata.channel_count != fixture.channels
        || metadata.total_frames != fixture.expected_planes[0].len() as u64
    {
        return Err("metadata differs from independent fixture declaration".to_owned());
    }
    let frame_count = fixture.expected_planes[0].len();
    let mut decoder = NativeWaveDecoder::prepare(
        cursor,
        metadata,
        NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: frame_count as u64,
        },
        NonZeroUsize::new(frame_count).ok_or_else(|| "empty fixture".to_owned())?,
    )
    .map_err(|error| format!("decoder: {error:?}"))?;
    let mut actual_planes = vec![vec![0.0_f32; frame_count]; usize::from(fixture.channels)];
    let report = {
        let mut planes: Vec<&mut [f32]> = actual_planes.iter_mut().map(Vec::as_mut_slice).collect();
        decoder
            .decode_into(&mut planes)
            .map_err(|error| format!("decode: {error:?}"))?
    };
    let actual_bits: Vec<Vec<u32>> = actual_planes
        .iter()
        .map(|plane| plane.iter().copied().map(f32::to_bits).collect())
        .collect();
    if actual_bits != fixture.expected_planes {
        return Err("production PCM differs from independent oracle bits".to_owned());
    }
    if report.sanitized_sample_count != fixture.expected_sanitized || !report.end_of_region {
        return Err("unexpected PCM sanitation or region status".to_owned());
    }
    Ok(())
}

fn corruption_rejects(fixtures: &[FixtureCase]) -> Result<(), String> {
    let riff = fixtures
        .iter()
        .find(|fixture| fixture.id == "riff-pcm16-stereo-v1")
        .ok_or_else(|| "missing RIFF corruption base".to_owned())?;

    let mut rifx = riff.bytes.clone();
    rifx[..4].copy_from_slice(b"RIFX");
    rejects(&rifx, CAPS, "big-endian RIFX")?;

    let compressed = riff_wave(&classic_format(6, 1, 16), &[], &[0, 0]);
    rejects(&compressed, CAPS, "unsupported compressed format")?;

    let cap_exceeded = riff_wave(&classic_format(1, 1, 8), &[(b"JUNK", vec![1; 17])], &[128]);
    rejects(&cap_exceeded, CAPS, "metadata cap")?;

    let duplicate_data = riff_wave(&classic_format(1, 1, 8), &[(b"data", vec![128])], &[128]);
    rejects(&duplicate_data, CAPS, "duplicate data")
}

fn rejects(bytes: &[u8], caps: NativeWaveParseCaps, label: &str) -> Result<(), String> {
    if parse_native_wave(&mut Cursor::new(bytes), caps).is_ok() {
        return Err(format!("{label} corruption unexpectedly parsed"));
    }
    Ok(())
}

fn generated_fixtures() -> Vec<FixtureCase> {
    let float_sanitize = [1.5_f32, -0.0, f32::INFINITY, f32::from_bits(1)];
    let mut f32_pcm = Vec::new();
    for value in float_sanitize {
        f32_pcm.extend_from_slice(&value.to_le_bytes());
    }
    let float_sanitize_bits = vec![vec![1.5_f32.to_bits(), (-0.0_f32).to_bits(), 0, 0]];

    let mut f64_pcm = Vec::new();
    for value in [1.5_f64, -0.0, f64::INFINITY, f64::from_bits(1)] {
        f64_pcm.extend_from_slice(&value.to_le_bytes());
    }

    let pcm16 = [
        i16::MIN.to_le_bytes(),
        i16::MAX.to_le_bytes(),
        0_i16.to_le_bytes(),
        (-16_384_i16).to_le_bytes(),
    ]
    .concat();
    let pcm24 = [[0x00, 0x00, 0x80], [0x00, 0x00, 0x00], [0xff, 0xff, 0x7f]].concat();
    let pcm32 = [
        i32::MIN.to_le_bytes(),
        0_i32.to_le_bytes(),
        i32::MAX.to_le_bytes(),
    ]
    .concat();

    let rf64_data = [
        0.25_f32.to_le_bytes(),
        (-0.25_f32).to_le_bytes(),
        0.5_f32.to_le_bytes(),
        (-0.5_f32).to_le_bytes(),
    ]
    .concat();

    vec![
        FixtureCase {
            id: "rf64-extensible-f32-odd-junk-v1",
            bytes: rf64_wave(
                &extensible_format(3, 2, 32),
                &[(b"JUNK", vec![7, 8, 9])],
                &rf64_data,
                2,
            ),
            channels: 2,
            expected_planes: vec![
                vec![0.25_f32.to_bits(), 0.5_f32.to_bits()],
                vec![(-0.25_f32).to_bits(), (-0.5_f32).to_bits()],
            ],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-f32-signed-zero-sanitize-v1",
            bytes: riff_wave(&classic_format(3, 1, 32), &[], &f32_pcm),
            channels: 1,
            expected_planes: float_sanitize_bits.clone(),
            expected_sanitized: 2,
        },
        FixtureCase {
            id: "riff-f64-signed-zero-sanitize-v1",
            bytes: riff_wave(&classic_format(3, 1, 64), &[], &f64_pcm),
            channels: 1,
            expected_planes: float_sanitize_bits,
            expected_sanitized: 2,
        },
        FixtureCase {
            id: "riff-pcm16-stereo-v1",
            bytes: riff_wave(&classic_format(1, 2, 16), &[], &pcm16),
            channels: 2,
            expected_planes: vec![
                vec![(-1.0_f32).to_bits(), 0],
                vec![(32_767.0_f32 / 32_768.0).to_bits(), (-0.5_f32).to_bits()],
            ],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-pcm24-multichannel-v1",
            bytes: riff_wave(&classic_format(1, 3, 24), &[], &pcm24),
            channels: 3,
            expected_planes: vec![
                vec![(-1.0_f32).to_bits()],
                vec![0],
                vec![(8_388_607.0_f32 / 8_388_608.0).to_bits()],
            ],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-pcm32-mono-v1",
            bytes: riff_wave(&classic_format(1, 1, 32), &[], &pcm32),
            channels: 1,
            expected_planes: vec![vec![(-1.0_f32).to_bits(), 0, 1.0_f32.to_bits()]],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-u8-mono-v1",
            bytes: riff_wave(&classic_format(1, 1, 8), &[], &[0, 128, 255]),
            channels: 1,
            expected_planes: vec![vec![(-1.0_f32).to_bits(), 0, (127.0_f32 / 128.0).to_bits()]],
            expected_sanitized: 0,
        },
    ]
}

fn classic_format(tag: u16, channels: u16, bits_per_sample: u16) -> Vec<u8> {
    format_fields(tag, channels, bits_per_sample)
}

fn extensible_format(subformat_tag: u16, channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut format = format_fields(0xfffe, channels, bits_per_sample);
    format.extend_from_slice(&22_u16.to_le_bytes());
    format.extend_from_slice(&bits_per_sample.to_le_bytes());
    format.extend_from_slice(&0_u32.to_le_bytes());
    format.extend_from_slice(&subformat_guid(subformat_tag));
    format
}

fn format_fields(tag: u16, channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let bytes_per_sample = u32::from(bits_per_sample / 8);
    let block_align = u32::from(channels) * bytes_per_sample;
    [
        tag.to_le_bytes().to_vec(),
        channels.to_le_bytes().to_vec(),
        48_000_u32.to_le_bytes().to_vec(),
        (48_000 * block_align).to_le_bytes().to_vec(),
        u16::try_from(block_align)
            .expect("small fixture")
            .to_le_bytes()
            .to_vec(),
        bits_per_sample.to_le_bytes().to_vec(),
    ]
    .concat()
}

fn subformat_guid(tag: u16) -> [u8; 16] {
    [
        tag as u8,
        (tag >> 8) as u8,
        0,
        0,
        0,
        0,
        0x10,
        0,
        0x80,
        0,
        0,
        0xaa,
        0,
        0x38,
        0x9b,
        0x71,
    ]
}

fn riff_wave(format: &[u8], extra_chunks: &[(&[u8; 4], Vec<u8>)], pcm: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    append_chunk(&mut bytes, b"fmt ", format);
    for (id, payload) in extra_chunks {
        append_chunk(&mut bytes, id, payload);
    }
    append_chunk(&mut bytes, b"data", pcm);
    let riff_size = u32::try_from(bytes.len() - 8).expect("small fixture");
    bytes[4..8].copy_from_slice(&riff_size.to_le_bytes());
    bytes
}

fn rf64_wave(
    format: &[u8],
    extra_chunks: &[(&[u8; 4], Vec<u8>)],
    pcm: &[u8],
    sample_count: u64,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RF64");
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"ds64");
    bytes.extend_from_slice(&28_u32.to_le_bytes());
    let ds64_offset = bytes.len();
    bytes.resize(ds64_offset + 28, 0);
    append_chunk(&mut bytes, b"fmt ", format);
    for (id, payload) in extra_chunks {
        append_chunk(&mut bytes, id, payload);
    }
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(pcm);
    if pcm.len() % 2 == 1 {
        bytes.push(0);
    }
    let riff_size = u64::try_from(bytes.len() - 8).expect("small fixture");
    bytes[ds64_offset..ds64_offset + 8].copy_from_slice(&riff_size.to_le_bytes());
    bytes[ds64_offset + 8..ds64_offset + 16].copy_from_slice(
        &u64::try_from(pcm.len())
            .expect("small fixture")
            .to_le_bytes(),
    );
    bytes[ds64_offset + 16..ds64_offset + 24].copy_from_slice(&sample_count.to_le_bytes());
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
    fn generated_fixtures_match_manifest_oracles_and_mutation_policy() {
        check().expect("fixture check");
    }
}
