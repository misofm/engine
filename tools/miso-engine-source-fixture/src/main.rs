//! Generated/checksummed source fixture corpus checker V1.
//!
//! The fixture bytes and expected planar `f32` bits are intentionally assembled independently from
//! the production parser/decoder. This remains the one source fixture generator/checker tool.

use std::{
    collections::{BTreeMap, VecDeque},
    io::Cursor,
    num::NonZeroUsize,
};

use miso_engine_core::{QuantumFrames, SampleRateHz};
use miso_engine_source::{
    HostChunkError, HostPlanarChunk, NativeWaveDecoder, NativeWaveParseCaps, NativeWaveRegion,
    PcmSourceRing, PcmSourceRingConfig, SourceCommand, SourceDiagnosticCode, SourceFrame,
    SourceGeneration, parse_native_wave,
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
    total_frames: u64,
    region: NativeWaveRegion,
    expected_planes: Vec<Vec<u32>>,
    expected_sanitized: u64,
}

struct InvalidCase {
    id: &'static str,
    bytes: Vec<u8>,
    caps: NativeWaveParseCaps,
    expected: SourceDiagnosticCode,
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
    exact_diagnostic_matrix(&fixtures)?;
    frozen_seek_ring_schedules()?;
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
        || metadata.total_frames != fixture.total_frames
    {
        return Err("metadata differs from independent fixture declaration".to_owned());
    }
    let frame_count = fixture.expected_planes[0].len();
    let mut decoder = NativeWaveDecoder::prepare(
        cursor,
        metadata,
        fixture.region,
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

fn exact_diagnostic_matrix(fixtures: &[FixtureCase]) -> Result<(), String> {
    let riff = fixtures
        .iter()
        .find(|fixture| fixture.id == "riff-pcm16-stereo-v1")
        .ok_or_else(|| "missing RIFF corruption base".to_owned())?;
    let rf64 = fixtures
        .iter()
        .find(|fixture| fixture.id == "rf64-extensible-f32-odd-junk-v1")
        .ok_or_else(|| "missing RF64 corruption base".to_owned())?;

    let mut rifx = riff.bytes.clone();
    rifx[..4].copy_from_slice(b"RIFX");
    let mut bad_root_size = riff.bytes.clone();
    bad_root_size[4..8].copy_from_slice(&0_u32.to_le_bytes());
    let mut truncated = riff.bytes.clone();
    truncated.pop();
    let mut malformed_ds64_table = rf64.bytes.clone();
    let mut malformed_ds64_size = rf64.bytes.clone();
    malformed_ds64_size[16..20].copy_from_slice(&27_u32.to_le_bytes());
    // The RF64 `ds64` table-length field is at file offset 36. A nonzero table requires a
    // larger `ds64` payload than this independently generated fixture contains.
    malformed_ds64_table[36..40].copy_from_slice(&1_u32.to_le_bytes());
    let mut rf64_missing_placeholder = rf64.bytes.clone();
    let data_header = rf64_missing_placeholder
        .windows(4)
        .position(|window| window == b"data")
        .ok_or_else(|| "RF64 fixture has no data chunk".to_owned())?;
    rf64_missing_placeholder[data_header + 4..data_header + 8]
        .copy_from_slice(&0_u32.to_le_bytes());
    let mut unsupported_guid = rf64.bytes.clone();
    // The extensible GUID begins at fmt payload byte 24; mutate only its tag byte.
    let fmt_header = unsupported_guid
        .windows(4)
        .position(|window| window == b"fmt ")
        .ok_or_else(|| "RF64 fixture has no fmt chunk".to_owned())?;
    unsupported_guid[fmt_header + 8 + 24] = 0x7f;
    let mut mismatched_valid_bits = rf64.bytes.clone();
    mismatched_valid_bits[fmt_header + 8 + 18..fmt_header + 8 + 20]
        .copy_from_slice(&16_u16.to_le_bytes());
    let mut bad_byte_rate = riff.bytes.clone();
    bad_byte_rate[12 + 8 + 8..12 + 8 + 12].copy_from_slice(&1_u32.to_le_bytes());
    let mut bad_block_align = riff.bytes.clone();
    bad_block_align[12 + 8 + 12..12 + 8 + 14].copy_from_slice(&1_u16.to_le_bytes());
    let indivisible_data = riff_wave(&classic_format(1, 1, 16), &[], &[0, 0, 0]);
    let duplicate_fmt = riff_wave(
        &classic_format(1, 1, 8),
        &[(b"fmt ", classic_format(1, 1, 8))],
        &[128],
    );
    let duplicate_data = riff_wave(&classic_format(1, 1, 8), &[(b"data", vec![128])], &[128]);
    let compressed = riff_wave(&classic_format(6, 1, 16), &[], &[0, 0]);
    let chunk_count = riff_wave(&classic_format(1, 1, 8), &[(b"JUNK", vec![])], &[128]);
    let skipped_metadata = riff_wave(&classic_format(1, 1, 8), &[(b"JUNK", vec![1; 17])], &[128]);

    let cases = vec![
        InvalidCase {
            id: "container-rifx",
            bytes: rifx,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "riff-root-size",
            bytes: bad_root_size,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "truncated-container",
            bytes: truncated,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "rf64-ds64-size",
            bytes: malformed_ds64_size,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "rf64-ds64-table",
            bytes: malformed_ds64_table,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "rf64-data-placeholder",
            bytes: rf64_missing_placeholder,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "duplicate-fmt",
            bytes: duplicate_fmt,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "duplicate-data",
            bytes: duplicate_data,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "unsupported-compression-tag",
            bytes: compressed,
            caps: CAPS,
            expected: SourceDiagnosticCode::FormatUnsupported,
        },
        InvalidCase {
            id: "unsupported-extensible-guid",
            bytes: unsupported_guid,
            caps: CAPS,
            expected: SourceDiagnosticCode::FormatUnsupported,
        },
        InvalidCase {
            id: "extensible-valid-container-bits",
            bytes: mismatched_valid_bits,
            caps: CAPS,
            expected: SourceDiagnosticCode::FormatUnsupported,
        },
        InvalidCase {
            id: "byte-rate",
            bytes: bad_byte_rate,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "block-align",
            bytes: bad_block_align,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "data-frame-divisibility",
            bytes: indivisible_data,
            caps: CAPS,
            expected: SourceDiagnosticCode::ContainerInvalid,
        },
        InvalidCase {
            id: "chunk-count-cap",
            bytes: chunk_count,
            caps: NativeWaveParseCaps {
                max_chunk_count: 2,
                max_skipped_metadata_bytes: 16,
            },
            expected: SourceDiagnosticCode::ResourceLimit,
        },
        InvalidCase {
            id: "skipped-metadata-cap",
            bytes: skipped_metadata,
            caps: CAPS,
            expected: SourceDiagnosticCode::ResourceLimit,
        },
    ];
    for case in cases {
        let error = match parse_native_wave(&mut Cursor::new(case.bytes), case.caps) {
            Ok(_) => return Err(format!("{} unexpectedly parsed", case.id)),
            Err(error) => error,
        };
        if error.diagnostic_code() != case.expected {
            return Err(format!(
                "{} diagnostic mismatch: expected {}, actual {}",
                case.id,
                case.expected,
                error.diagnostic_code()
            ));
        }
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
            total_frames: 2,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 2,
            },
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
            total_frames: 4,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 4,
            },
            expected_planes: float_sanitize_bits.clone(),
            expected_sanitized: 2,
        },
        FixtureCase {
            id: "riff-f64-signed-zero-sanitize-v1",
            bytes: riff_wave(&classic_format(3, 1, 64), &[], &f64_pcm),
            channels: 1,
            total_frames: 4,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 4,
            },
            expected_planes: float_sanitize_bits,
            expected_sanitized: 2,
        },
        FixtureCase {
            id: "riff-pcm16-stereo-v1",
            bytes: riff_wave(&classic_format(1, 2, 16), &[], &pcm16),
            channels: 2,
            total_frames: 2,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 2,
            },
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
            total_frames: 1,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 1,
            },
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
            total_frames: 3,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 3,
            },
            expected_planes: vec![vec![(-1.0_f32).to_bits(), 0, 1.0_f32.to_bits()]],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-u8-mono-v1",
            bytes: riff_wave(&classic_format(1, 1, 8), &[], &[0, 128, 255]),
            channels: 1,
            total_frames: 3,
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 3,
            },
            expected_planes: vec![vec![(-1.0_f32).to_bits(), 0, (127.0_f32 / 128.0).to_bits()]],
            expected_sanitized: 0,
        },
        FixtureCase {
            id: "riff-pcm16-stereo-nonzero-short-region-v1",
            bytes: riff_wave(
                &classic_format(1, 2, 16),
                &[],
                &[
                    0_i16.to_le_bytes(),
                    0_i16.to_le_bytes(),
                    i16::MAX.to_le_bytes(),
                    (-16_384_i16).to_le_bytes(),
                    i16::MIN.to_le_bytes(),
                    16_384_i16.to_le_bytes(),
                ]
                .concat(),
            ),
            channels: 2,
            total_frames: 3,
            region: NativeWaveRegion {
                start_frame: SourceFrame(1),
                length_frames: 1,
            },
            expected_planes: vec![
                vec![(32_767.0_f32 / 32_768.0).to_bits()],
                vec![(-0.5_f32).to_bits()],
            ],
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

// This is a deliberately small independent schedule oracle, not another source ring. It models
// only the frozen action language below: one producer, one bounded FIFO of complete quanta, and
// block-boundary seeks. The production ring is exercised only after this model has produced every
// expected outcome from the sealed transcript.
const SEEK_SCHEDULE_SEED: u64 = 0x0000_0000_010a_5ee1;
const SEEK_SCHEDULE_COUNT: usize = 256;
const SEEK_QUANTUM: u32 = 4;
const FROZEN_SEEK_TRANSCRIPT_SHA256: &str =
    "ec3b7fef8e86937d4431466d2cea8a68ec56feb2897bcdc655fa10d5bf30a41c";

#[derive(Clone, Debug)]
struct SeekSchedule {
    index: u16,
    capacity_quanta: usize,
    actions: Vec<SeekAction>,
}

#[derive(Clone, Debug)]
enum SeekAction {
    Submit {
        generation: u64,
        start_frame: u64,
        frames: u32,
        end_of_region: bool,
        sample_bits: u32,
    },
    Seek {
        generation: u64,
        frame: u64,
    },
    Render,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModelSubmit {
    Accepted,
    Full,
    StaleGeneration,
    EndOfRegion,
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModelOutcome {
    Submit(ModelSubmit),
    Seek,
    Render(ModelRender),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ModelRender {
    output_bits: [u32; SEEK_QUANTUM as usize],
    copied_frames: u32,
    underrun_frames: u32,
    underrun_event: bool,
    end_of_region: bool,
    active_generation: u64,
    cumulative_read_frames: u64,
    cumulative_underrun_frames: u64,
    cumulative_underrun_events: u64,
    stale_discards: u64,
}

#[derive(Clone, Copy, Debug)]
struct ModelBlock {
    generation: u64,
    start_frame: u64,
    frames: u32,
    end_of_region: bool,
    sample_bits: u32,
}

struct IndependentSeekModel {
    capacity: usize,
    free_blocks: usize,
    data: VecDeque<ModelBlock>,
    command: Option<(u64, u64)>,
    active_generation: u64,
    producer_generation: u64,
    producer_next_frame: u64,
    producer_end_submitted: bool,
    next_frame: u64,
    end_frame: Option<u64>,
    end_of_region: bool,
    cumulative_read_frames: u64,
    stale_discards: u64,
    underrun_frames: u64,
    underrun_events: u64,
}

impl IndependentSeekModel {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            free_blocks: capacity,
            data: VecDeque::with_capacity(capacity),
            command: None,
            active_generation: 1,
            producer_generation: 1,
            producer_next_frame: 0,
            producer_end_submitted: false,
            next_frame: 0,
            end_frame: None,
            end_of_region: false,
            cumulative_read_frames: 0,
            stale_discards: 0,
            underrun_frames: 0,
            underrun_events: 0,
        }
    }

    fn submit(&mut self, block: ModelBlock) -> ModelSubmit {
        if block.generation != self.producer_generation {
            return ModelSubmit::StaleGeneration;
        }
        if self.producer_end_submitted {
            return ModelSubmit::EndOfRegion;
        }
        if block.start_frame != self.producer_next_frame
            || block.frames > SEEK_QUANTUM
            || (block.frames < SEEK_QUANTUM && !block.end_of_region)
            || (block.frames == 0 && !block.end_of_region)
        {
            return ModelSubmit::Invalid;
        }
        if self.free_blocks == 0 || self.data.len() == self.capacity {
            return ModelSubmit::Full;
        }
        self.free_blocks -= 1;
        self.producer_next_frame = self
            .producer_next_frame
            .saturating_add(u64::from(block.frames));
        if block.end_of_region {
            self.producer_end_submitted = true;
        }
        self.data.push_back(block);
        ModelSubmit::Accepted
    }

    fn seek(&mut self, generation: u64, frame: u64) -> Result<(), String> {
        if generation <= self.producer_generation {
            return Err("schedule generated a non-increasing seek".to_owned());
        }
        if self.command.is_some() {
            return Err("schedule overfilled its bounded seek command slot".to_owned());
        }
        self.producer_generation = generation;
        self.producer_next_frame = frame;
        self.producer_end_submitted = false;
        self.command = Some((generation, frame));
        Ok(())
    }

    fn render(&mut self) -> ModelRender {
        if let Some((generation, frame)) = self.command.take() {
            self.active_generation = generation;
            self.next_frame = frame;
            self.end_frame = None;
            self.end_of_region = false;
        }
        let mut selected = None;
        while let Some(block) = self.data.pop_front() {
            if block.generation != self.active_generation || block.start_frame < self.next_frame {
                self.stale_discards = self.stale_discards.saturating_add(1);
                self.free_blocks += 1;
                continue;
            }
            if block.end_of_region {
                self.end_frame = Some(block.start_frame.saturating_add(u64::from(block.frames)));
            }
            selected = Some(block);
            break;
        }

        let mut output_bits = [0_u32; SEEK_QUANTUM as usize];
        let mut copied_frames = 0;
        let mut underrun_frames = 0;
        if self.end_frame.is_some_and(|end| self.next_frame >= end) {
            self.end_of_region = true;
        } else if let Some(block) = selected {
            if block.start_frame == self.next_frame {
                let frames = usize::try_from(block.frames).expect("frozen quantum fits usize");
                output_bits[..frames].fill(block.sample_bits);
                copied_frames = block.frames;
                self.next_frame = self.next_frame.saturating_add(u64::from(block.frames));
                self.cumulative_read_frames = self
                    .cumulative_read_frames
                    .saturating_add(u64::from(block.frames));
                if block.end_of_region {
                    self.end_of_region = true;
                }
                self.free_blocks += 1;
            } else {
                // The selected future block remains queued in the production endpoint. Frozen
                // schedules never produce one, so treat it as a generator/model defect.
                self.data.push_front(block);
                underrun_frames = self.available_until_end();
                self.note_underrun(underrun_frames);
            }
        } else {
            underrun_frames = self.available_until_end();
            self.note_underrun(underrun_frames);
        }

        ModelRender {
            output_bits,
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
            stale_discards: self.stale_discards,
        }
    }

    fn available_until_end(&self) -> u32 {
        self.end_frame
            .map(|end| end.saturating_sub(self.next_frame))
            .unwrap_or(u64::from(SEEK_QUANTUM))
            .min(u64::from(SEEK_QUANTUM)) as u32
    }

    fn note_underrun(&mut self, frames: u32) {
        self.next_frame = self.next_frame.saturating_add(u64::from(SEEK_QUANTUM));
        if frames != 0 {
            self.underrun_frames = self.underrun_frames.saturating_add(u64::from(frames));
            self.underrun_events = self.underrun_events.saturating_add(1);
        }
        if self.end_frame.is_some_and(|end| self.next_frame >= end) {
            self.end_of_region = true;
        }
    }
}

fn frozen_seek_ring_schedules() -> Result<(), String> {
    let schedules = generate_frozen_seek_schedules();
    if schedules.len() != SEEK_SCHEDULE_COUNT {
        return Err(format!("expected {SEEK_SCHEDULE_COUNT} schedules"));
    }
    let transcript = seek_transcript_sha256(&schedules);
    if transcript != FROZEN_SEEK_TRANSCRIPT_SHA256 {
        return Err(format!(
            "frozen seek transcript mismatch: expected {FROZEN_SEEK_TRANSCRIPT_SHA256}, actual {transcript}"
        ));
    }
    for schedule in &schedules {
        let expected = model_schedule(schedule)?;
        exercise_production_schedule(schedule, &expected)?;
    }
    Ok(())
}

fn generate_frozen_seek_schedules() -> Vec<SeekSchedule> {
    let capacities = [1_usize, 2, 3, 8];
    let mut state = SEEK_SCHEDULE_SEED;
    (0..SEEK_SCHEDULE_COUNT)
        .map(|index| {
            state = xorshift64(state);
            let capacity_quanta = capacities[index % capacities.len()];
            let base = 1_024_u64 + u64::try_from(index).expect("fixed count") * 1_024;
            let first_seek = base;
            let second_seek = base + 512;
            let first_terminal_frames = if index % 17 == 0 {
                0
            } else {
                1 + u32::try_from(state & 0x03).expect("two bits") % (SEEK_QUANTUM - 1)
            };
            let second_terminal_frames = if index % 19 == 0 {
                0
            } else {
                1 + u32::try_from((state >> 8) & 0x03).expect("two bits") % (SEEK_QUANTUM - 1)
            };
            let mut actions = Vec::with_capacity(capacity_quanta * 3 + 20);
            for slot in 0..capacity_quanta {
                actions.push(SeekAction::Submit {
                    generation: 1,
                    start_frame: u64::try_from(slot).expect("small slot") * u64::from(SEEK_QUANTUM),
                    frames: SEEK_QUANTUM,
                    end_of_region: false,
                    sample_bits: schedule_sample_bits(index, slot, 1),
                });
            }
            // This is the frozen full transition; it must accept no prefix or state change.
            actions.push(SeekAction::Submit {
                generation: 1,
                start_frame: u64::try_from(capacity_quanta).expect("small capacity")
                    * u64::from(SEEK_QUANTUM),
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, capacity_quanta, 1),
            });
            actions.push(SeekAction::Seek {
                generation: 2,
                frame: first_seek,
            });
            actions.push(SeekAction::Render);
            // This late new-generation block is discarded after the declared request boundary.
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 0, 2),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_seek + u64::from(SEEK_QUANTUM) * 2,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 1, 2),
            });
            actions.push(SeekAction::Render);
            for round in 0..=capacity_quanta {
                let start_frame = first_seek
                    + u64::from(SEEK_QUANTUM) * u64::try_from(round + 3).expect("small wrap round");
                actions.push(SeekAction::Submit {
                    generation: 2,
                    start_frame,
                    frames: SEEK_QUANTUM,
                    end_of_region: false,
                    sample_bits: schedule_sample_bits(index, round + 2, 2),
                });
                actions.push(SeekAction::Render);
            }
            let first_terminal_start = first_seek
                + u64::from(SEEK_QUANTUM)
                    * u64::try_from(capacity_quanta + 4).expect("small terminal round");
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_terminal_start,
                frames: first_terminal_frames,
                end_of_region: true,
                sample_bits: schedule_sample_bits(index, capacity_quanta + 3, 2),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Seek {
                generation: 3,
                frame: second_seek,
            });
            actions.push(SeekAction::Render);
            // A delayed old-generation attempt is rejected at the producer boundary.
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: second_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 0, 3),
            });
            actions.push(SeekAction::Submit {
                generation: 3,
                start_frame: second_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 1, 3),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Submit {
                generation: 3,
                start_frame: second_seek + u64::from(SEEK_QUANTUM) * 2,
                frames: second_terminal_frames,
                end_of_region: true,
                sample_bits: schedule_sample_bits(index, 2, 3),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Render);
            SeekSchedule {
                index: u16::try_from(index).expect("fixed schedule count"),
                capacity_quanta,
                actions,
            }
        })
        .collect()
}

fn model_schedule(schedule: &SeekSchedule) -> Result<Vec<ModelOutcome>, String> {
    let mut model = IndependentSeekModel::new(schedule.capacity_quanta);
    schedule
        .actions
        .iter()
        .map(|action| match action {
            SeekAction::Submit {
                generation,
                start_frame,
                frames,
                end_of_region,
                sample_bits,
            } => Ok(ModelOutcome::Submit(model.submit(ModelBlock {
                generation: *generation,
                start_frame: *start_frame,
                frames: *frames,
                end_of_region: *end_of_region,
                sample_bits: *sample_bits,
            }))),
            SeekAction::Seek { generation, frame } => {
                model.seek(*generation, *frame)?;
                Ok(ModelOutcome::Seek)
            }
            SeekAction::Render => Ok(ModelOutcome::Render(model.render())),
        })
        .collect()
}

fn exercise_production_schedule(
    schedule: &SeekSchedule,
    expected: &[ModelOutcome],
) -> Result<(), String> {
    let config = PcmSourceRingConfig {
        channel_count: 1,
        quantum_frames: QuantumFrames(SEEK_QUANTUM),
        frame_capacity: u64::try_from(schedule.capacity_quanta).expect("small capacity")
            * u64::from(SEEK_QUANTUM),
        initial_generation: SourceGeneration(1),
    };
    let (producer, mut consumer, _) = PcmSourceRing::prepare(config)
        .map_err(|error| format!("schedule {} ring prepare: {error:?}", schedule.index))?;
    let mut host = producer.into_host_chunk_provider(SampleRateHz(48_000));
    let mut output = [f32::from_bits(0xffff_ffff); SEEK_QUANTUM as usize];
    for (step, (action, expected)) in schedule.actions.iter().zip(expected).enumerate() {
        match (action, expected) {
            (
                SeekAction::Submit {
                    generation,
                    start_frame,
                    frames,
                    end_of_region,
                    sample_bits,
                },
                ModelOutcome::Submit(expected_submit),
            ) => {
                let plane = [f32::from_bits(*sample_bits); SEEK_QUANTUM as usize];
                let planes = [&plane[..usize::try_from(*frames).expect("frozen frame count")]];
                let actual = host.submit(HostPlanarChunk {
                    sample_rate_hz: SampleRateHz(48_000),
                    generation: SourceGeneration(*generation),
                    start_frame: SourceFrame(*start_frame),
                    planes: &planes,
                    frames: *frames,
                    end_of_region: *end_of_region,
                });
                let actual_submit = match &actual {
                    Ok(_) => ModelSubmit::Accepted,
                    Err(HostChunkError::Full { .. }) => ModelSubmit::Full,
                    Err(HostChunkError::StaleGeneration { .. }) => ModelSubmit::StaleGeneration,
                    Err(HostChunkError::EndOfRegionAlreadySubmitted) => ModelSubmit::EndOfRegion,
                    Err(_) => ModelSubmit::Invalid,
                };
                if actual_submit != *expected_submit {
                    return Err(format!(
                        "schedule {} step {step} submit mismatch: expected {expected_submit:?}, actual {actual_submit:?} ({actual:?})",
                        schedule.index,
                    ));
                }
            }
            (SeekAction::Seek { generation, frame }, ModelOutcome::Seek) => host
                .try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(*generation),
                    frame: SourceFrame(*frame),
                })
                .map_err(|error| {
                    format!("schedule {} step {step} seek: {error:?}", schedule.index)
                })?,
            (SeekAction::Render, ModelOutcome::Render(expected_render)) => {
                let report = consumer.read_block(&mut [&mut output]).map_err(|error| {
                    format!("schedule {} step {step} render: {error:?}", schedule.index)
                })?;
                let actual_bits = output.map(f32::to_bits);
                if actual_bits != expected_render.output_bits
                    || report.copied_frames != expected_render.copied_frames
                    || report.underrun_frames != expected_render.underrun_frames
                    || report.underrun_event != expected_render.underrun_event
                    || report.end_of_region != expected_render.end_of_region
                    || report.active_generation
                        != SourceGeneration(expected_render.active_generation)
                    || report.cumulative_read_frames != expected_render.cumulative_read_frames
                    || report.cumulative_underrun_frames
                        != expected_render.cumulative_underrun_frames
                    || report.cumulative_underrun_events
                        != expected_render.cumulative_underrun_events
                    || consumer.telemetry().stale_generation_discard_count
                        != expected_render.stale_discards
                {
                    return Err(format!(
                        "schedule {} step {step} render/model mismatch: expected {expected_render:?}, actual bits {actual_bits:?}, report {report:?}, telemetry {:?}",
                        schedule.index,
                        consumer.telemetry()
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "schedule {} step {step} internal action/outcome mismatch",
                    schedule.index
                ));
            }
        }
    }
    Ok(())
}

fn seek_transcript_sha256(schedules: &[SeekSchedule]) -> String {
    let mut bytes = Vec::new();
    for schedule in schedules {
        bytes.extend_from_slice(&schedule.index.to_le_bytes());
        bytes.extend_from_slice(
            &u64::try_from(schedule.capacity_quanta)
                .expect("small capacity")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(
            &u64::try_from(schedule.actions.len())
                .expect("small action count")
                .to_le_bytes(),
        );
        for action in &schedule.actions {
            match action {
                SeekAction::Submit {
                    generation,
                    start_frame,
                    frames,
                    end_of_region,
                    sample_bits,
                } => {
                    bytes.push(1);
                    bytes.extend_from_slice(&generation.to_le_bytes());
                    bytes.extend_from_slice(&start_frame.to_le_bytes());
                    bytes.extend_from_slice(&frames.to_le_bytes());
                    bytes.push(u8::from(*end_of_region));
                    bytes.extend_from_slice(&sample_bits.to_le_bytes());
                }
                SeekAction::Seek { generation, frame } => {
                    bytes.push(2);
                    bytes.extend_from_slice(&generation.to_le_bytes());
                    bytes.extend_from_slice(&frame.to_le_bytes());
                }
                SeekAction::Render => bytes.push(3),
            }
        }
    }
    sha256_hex(&bytes)
}

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

fn schedule_sample_bits(schedule: usize, slot: usize, generation: u64) -> u32 {
    let value = (schedule as f32 + 1.0) * 0.001 + slot as f32 * 0.01 + generation as f32 * 0.1;
    value.to_bits()
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
