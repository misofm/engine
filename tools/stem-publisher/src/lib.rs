//! Offline WAVE-to-FLAC publisher with a mandatory shipped-decoder round-trip guard.

use std::{
    ffi::{OsStr, OsString},
    fs::{self, File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use flac_decoder::decode_flac_to_writer;
use flacenc::{
    bitsink::ByteSink, component::BitRepr, config::Encoder, error::Verify, source::MemSource,
};
use source::{NativeWaveParseCaps, parse_native_wave};
use stem_hasher::{
    CanonicalBitDepth, CanonicalPcmShape, StemIdentityReport, canonicalize_raw_pcm,
    canonicalize_wave,
};

/// Pinned deterministic publisher block size.
pub const DEFAULT_BLOCK_FRAMES: usize = 4096;
/// Exact pinned encoder implementation.
pub const ENCODER_NAME: &str = "flacenc";
/// Exact pinned encoder version.
pub const ENCODER_VERSION: &str = "0.5.1";
/// Exact pinned shipped decoder implementation.
pub const DECODER_NAME: &str = "symphonia";
/// Exact pinned shipped decoder version.
pub const DECODER_VERSION: &str = "0.6.1";

/// FLAC is integer PCM: a `32f` master is refused typed and never encoded.
const REFUSAL_32F: &str = "master.bit_depth.32f.refused";
const WAVE_MAXIMUM_CHUNKS: u32 = 4_096;
const WAVE_MAXIMUM_SKIPPED_METADATA_BYTES: u64 = 16 * 1024 * 1024;
const DECODER_ARTIFACT_SHA256: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../sidecars/flac-decoder/decoder-artifact.sha256"
));

/// Stable publisher refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublisherError {
    code: &'static str,
}

impl PublisherError {
    const fn new(code: &'static str) -> Self {
        Self { code }
    }

    /// Stable dotted reason.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl std::fmt::Display for PublisherError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "miso.stem.publisher.v1\t{}", self.code)
    }
}

impl std::error::Error for PublisherError {}

/// Complete delivery row emitted only after round-trip verification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedStem {
    /// Canonical-PCM identity.
    pub identity: String,
    /// Source channels.
    pub channels: u16,
    /// Source-native launch depth.
    pub bit_depth: u16,
    /// Source frames.
    pub frames: u64,
    /// Exact canonical bytes.
    pub pcm_bytes: u64,
    /// Exact delivery object bytes.
    pub flac_bytes: u64,
    /// WAVE/FLAC sample rate.
    pub sample_rate_hz: u32,
    /// Deterministic encoder block setting.
    pub block_frames: usize,
    /// Bare delivery object file name.
    pub delivery_file: String,
}

impl PublishedStem {
    /// Canonical JSON catalog row with transport provenance kept outside identity.
    #[must_use]
    pub fn catalog_json(&self) -> String {
        let decoder_sha256 = DECODER_ARTIFACT_SHA256.trim();
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"miso.stem.delivery.v1\",\n",
                "  \"identity\": \"{}\",\n",
                "  \"channels\": {},\n",
                "  \"bit_depth\": {},\n",
                "  \"frames\": {},\n",
                "  \"pcm_bytes\": {},\n",
                "  \"flac_bytes\": {},\n",
                "  \"sample_rate_hz\": {},\n",
                "  \"delivery\": {{ \"kind\": \"flac\", \"file\": \"{}\" }},\n",
                "  \"encoder\": {{ \"name\": \"{}\", \"version\": \"{}\", ",
                "\"block_frames\": {}, \"multithread\": false }},\n",
                "  \"decoder\": {{ \"name\": \"{}\", \"version\": \"{}\", ",
                "\"artifact_sha256\": \"{}\" }}\n",
                "}}\n"
            ),
            self.identity,
            self.channels,
            self.bit_depth,
            self.frames,
            self.pcm_bytes,
            self.flac_bytes,
            self.sample_rate_hz,
            self.delivery_file,
            ENCODER_NAME,
            ENCODER_VERSION,
            self.block_frames,
            DECODER_NAME,
            DECODER_VERSION,
            decoder_sha256,
        )
    }
}

/// Encode one canonical PCM preimage with the exact pinned encoder configuration.
pub fn encode_canonical_pcm(
    canonical_pcm: &[u8],
    shape: CanonicalPcmShape,
    sample_rate_hz: u32,
    block_frames: usize,
) -> Result<Vec<u8>, PublisherError> {
    if sample_rate_hz == 0 {
        return Err(PublisherError::new("wave.sample_rate.invalid"));
    }
    let expected_bytes = usize::try_from(
        shape
            .canonical_bytes()
            .map_err(|_| PublisherError::new("pcm.shape.invalid"))?,
    )
    .map_err(|_| PublisherError::new("pcm.shape.invalid"))?;
    if canonical_pcm.len() != expected_bytes {
        return Err(PublisherError::new("pcm.length.mismatch"));
    }
    let samples = pcm_samples(canonical_pcm, shape.bit_depth)?;
    let mut config = Encoder::default();
    config.block_size = block_frames;
    config.multithread = false;
    config.workers = None;
    let config = config
        .into_verified()
        .map_err(|_| PublisherError::new("encoder.config.invalid"))?;
    let source = MemSource::from_samples(
        &samples,
        usize::from(shape.channels),
        usize::from(shape.bit_depth.bits()),
        usize::try_from(sample_rate_hz)
            .map_err(|_| PublisherError::new("wave.sample_rate.invalid"))?,
    );
    let stream = flacenc::encode_with_fixed_block_size(&config, source, block_frames)
        .map_err(|_| PublisherError::new("encoder.encode.refused"))?;
    let mut sink = ByteSink::new();
    stream
        .write(&mut sink)
        .map_err(|_| PublisherError::new("encoder.output.refused"))?;
    let mut encoded = sink.into_inner();
    pin_fixed_streaminfo_block_size(&mut encoded, block_frames)?;
    Ok(encoded)
}

/// Recompute a delivery object's identity through the shipped decoder core and shared hasher.
pub fn verify_round_trip(
    expected: StemIdentityReport,
    expected_sample_rate_hz: u32,
    flac: &[u8],
) -> Result<(), PublisherError> {
    let mut decoded = Vec::new();
    let decoded_report = decode_flac_to_writer(flac, expected.canonical_bytes, &mut decoded)
        .map_err(|_| PublisherError::new("round_trip.decode.refused"))?;
    if decoded_report.stream.channels != expected.shape.channels
        || decoded_report.stream.bit_depth.bits() != expected.shape.bit_depth.bits()
        || decoded_report.stream.frames != expected.shape.frames
        || decoded_report.stream.sample_rate_hz != expected_sample_rate_hz
    {
        return Err(PublisherError::new("round_trip.shape.mismatch"));
    }
    let recomputed = canonicalize_raw_pcm(&mut &decoded[..], expected.shape, &mut std::io::sink())
        .map_err(|_| PublisherError::new("round_trip.hash.refused"))?;
    if recomputed.identity() != expected.identity() {
        return Err(PublisherError::new("round_trip.identity.mismatch"));
    }
    Ok(())
}

/// Publish one WAVE master into a new output directory.
pub fn publish_wave(
    input: &Path,
    output_directory: &Path,
    block_frames: usize,
) -> Result<PublishedStem, PublisherError> {
    let (identity_report, sample_rate_hz, canonical_pcm) = read_wave_master(input)?;
    let flac = encode_canonical_pcm(
        &canonical_pcm,
        identity_report.shape,
        sample_rate_hz,
        block_frames,
    )?;
    verify_round_trip(identity_report, sample_rate_hz, &flac)?;
    let identity = identity_report.identity();
    let delivery_file = format!("{}.flac", identity.trim_start_matches("sha256:"));
    let published = PublishedStem {
        identity,
        channels: identity_report.shape.channels,
        bit_depth: identity_report.shape.bit_depth.bits(),
        frames: identity_report.shape.frames,
        pcm_bytes: identity_report.canonical_bytes,
        flac_bytes: u64::try_from(flac.len())
            .map_err(|_| PublisherError::new("encoder.output.too_large"))?,
        sample_rate_hz,
        block_frames,
        delivery_file,
    };
    fs::create_dir(output_directory).map_err(|_| PublisherError::new("output.create"))?;
    let result = (|| {
        write_new(&output_directory.join(&published.delivery_file), &flac)?;
        write_new(
            &output_directory.join("catalog-row.json"),
            published.catalog_json().as_bytes(),
        )?;
        Ok(published)
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(output_directory);
    }
    result
}

fn read_wave_master(input: &Path) -> Result<(StemIdentityReport, u32, Vec<u8>), PublisherError> {
    let mut wave = File::open(input).map_err(|_| PublisherError::new("input.open"))?;
    let mut canonical_pcm = Vec::new();
    let identity_report = canonicalize_wave(&mut wave, &mut canonical_pcm)
        .map_err(|_| PublisherError::new("wave.canonicalize.refused"))?;
    wave.seek(SeekFrom::Start(0))
        .map_err(|_| PublisherError::new("input.seek"))?;
    let metadata = parse_native_wave(
        &mut wave,
        NativeWaveParseCaps {
            max_chunk_count: WAVE_MAXIMUM_CHUNKS,
            max_skipped_metadata_bytes: WAVE_MAXIMUM_SKIPPED_METADATA_BYTES,
        },
    )
    .map_err(|_| PublisherError::new("wave.parse.refused"))?;
    if identity_report.shape.bit_depth == CanonicalBitDepth::Float32 {
        return Err(PublisherError::new(REFUSAL_32F));
    }
    Ok((identity_report, metadata.sample_rate_hz.0, canonical_pcm))
}

fn pcm_samples(bytes: &[u8], bit_depth: CanonicalBitDepth) -> Result<Vec<i32>, PublisherError> {
    match bit_depth {
        CanonicalBitDepth::Pcm16 => Ok(bytes
            .chunks_exact(2)
            .map(|sample| i32::from(i16::from_le_bytes([sample[0], sample[1]])))
            .collect()),
        CanonicalBitDepth::Pcm24 => Ok(bytes
            .chunks_exact(3)
            .map(|sample| {
                let sign = if sample[2] & 0x80 == 0 { 0 } else { 0xff };
                i32::from_le_bytes([sample[0], sample[1], sample[2], sign])
            })
            .collect()),
        // FLAC is integer PCM; a `32f` preimage is never encodable transport.
        CanonicalBitDepth::Float32 => Err(PublisherError::new(REFUSAL_32F)),
    }
}

fn pin_fixed_streaminfo_block_size(
    encoded: &mut [u8],
    block_frames: usize,
) -> Result<(), PublisherError> {
    let block_frames = u16::try_from(block_frames)
        .map_err(|_| PublisherError::new("encoder.block_size.invalid"))?;
    if encoded.len() < 12
        || &encoded[..4] != b"fLaC"
        || encoded[4] & 0x7f != 0
        || u32::from_be_bytes([0, encoded[5], encoded[6], encoded[7]]) != 34
    {
        return Err(PublisherError::new("encoder.streaminfo.invalid"));
    }
    // FLAC permits the final frame to be shorter than the fixed block size. flacenc 0.5.1 writes
    // that short final length into STREAMINFO's minimum field, which produces an invalid value
    // below FLAC's 16-frame STREAMINFO floor for #241's tiny vectors. libFLAC records the fixed
    // configured size in both fields. Pin the same standards-valid fixed-block declaration; the
    // audio frames, sample MD5, and therefore canonical identity are untouched.
    let bytes = block_frames.to_be_bytes();
    encoded[8..10].copy_from_slice(&bytes);
    encoded[10..12].copy_from_slice(&bytes);
    Ok(())
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), PublisherError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| PublisherError::new("output.create"))?;
    output
        .write_all(bytes)
        .and_then(|()| output.flush())
        .and_then(|()| output.sync_all())
        .map_err(|_| PublisherError::new("output.write"))
}

/// Closed publisher command line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublisherCommand {
    /// Encode, verify, and emit one delivery row.
    Publish {
        /// WAVE master.
        input: PathBuf,
        /// New output directory.
        output_directory: PathBuf,
        /// Fixed encoder block setting.
        block_frames: usize,
    },
    /// Verify an existing FLAC against a WAVE master without publishing.
    Verify {
        /// WAVE master.
        master: PathBuf,
        /// FLAC delivery object.
        flac: PathBuf,
    },
}

/// Parse the exact `publish`/`verify` CLI.
pub fn parse_cli(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<PublisherCommand, PublisherError> {
    let mut arguments = arguments.into_iter();
    let mode = arguments
        .next()
        .ok_or_else(|| PublisherError::new("cli.mode.missing"))?;
    let mut input = None;
    let mut output_directory = None;
    let mut block_frames = None;
    let mut master = None;
    let mut flac = None;
    while let Some(option) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| PublisherError::new("cli.option.value.missing"))?;
        match option.to_str() {
            Some("--input") if input.is_none() => input = Some(PathBuf::from(value)),
            Some("--output-dir") if output_directory.is_none() => {
                output_directory = Some(PathBuf::from(value));
            }
            Some("--block-frames") if block_frames.is_none() => {
                block_frames = Some(parse_usize(&value)?);
            }
            Some("--master") if master.is_none() => master = Some(PathBuf::from(value)),
            Some("--flac") if flac.is_none() => flac = Some(PathBuf::from(value)),
            Some("--input" | "--output-dir" | "--block-frames" | "--master" | "--flac") => {
                return Err(PublisherError::new("cli.option.duplicate"));
            }
            Some(_) | None => return Err(PublisherError::new("cli.option.unknown")),
        }
    }
    match mode.to_str() {
        Some("publish") => {
            if master.is_some() || flac.is_some() {
                return Err(PublisherError::new("cli.publish.option.forbidden"));
            }
            Ok(PublisherCommand::Publish {
                input: input.ok_or_else(|| PublisherError::new("cli.input.missing"))?,
                output_directory: output_directory
                    .ok_or_else(|| PublisherError::new("cli.output_dir.missing"))?,
                block_frames: block_frames.unwrap_or(DEFAULT_BLOCK_FRAMES),
            })
        }
        Some("verify") => {
            if input.is_some() || output_directory.is_some() || block_frames.is_some() {
                return Err(PublisherError::new("cli.verify.option.forbidden"));
            }
            Ok(PublisherCommand::Verify {
                master: master.ok_or_else(|| PublisherError::new("cli.master.missing"))?,
                flac: flac.ok_or_else(|| PublisherError::new("cli.flac.missing"))?,
            })
        }
        Some(_) | None => Err(PublisherError::new("cli.mode.unknown")),
    }
}

fn parse_usize(value: &OsStr) -> Result<usize, PublisherError> {
    value
        .to_str()
        .ok_or_else(|| PublisherError::new("cli.scalar.utf8"))?
        .parse()
        .map_err(|_| PublisherError::new("cli.scalar.invalid"))
}

/// Execute the closed publisher CLI.
pub fn run_cli(arguments: impl IntoIterator<Item = OsString>) -> Result<(), PublisherError> {
    match parse_cli(arguments)? {
        PublisherCommand::Publish {
            input,
            output_directory,
            block_frames,
        } => {
            let published = publish_wave(&input, &output_directory, block_frames)?;
            println!("{}\t{}", published.identity, published.delivery_file);
        }
        PublisherCommand::Verify { master, flac } => {
            let (report, sample_rate_hz, _) = read_wave_master(&master)?;
            let encoded = fs::read(flac).map_err(|_| PublisherError::new("input.open"))?;
            verify_round_trip(report, sample_rate_hz, &encoded)?;
            println!("{}", report.identity());
        }
    }
    Ok(())
}
