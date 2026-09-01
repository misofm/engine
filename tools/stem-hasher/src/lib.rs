//! Reference implementation of the canonical-PCM stem identity contract.
//!
//! This native-only tool deliberately reuses the engine's RIFF/WAVE and RF64/WAVE parser. It
//! serializes and hashes one bounded chunk at a time; a whole stem is never retained in memory.

use std::{
    ffi::{OsStr, OsString},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use sha2::{Digest, Sha256};
use source::{NativeWaveEncoding, NativeWaveError, NativeWaveParseCaps, parse_native_wave};

const STREAM_BYTES: usize = 48 * 1024;
const WAVE_MAXIMUM_CHUNKS: u32 = 4_096;
const WAVE_MAXIMUM_SKIPPED_METADATA_BYTES: u64 = 16 * 1024 * 1024;

/// Canonical-PCM sample-depth tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalBitDepth {
    /// Signed 16-bit integer PCM.
    Pcm16,
    /// Signed packed 24-bit integer PCM.
    Pcm24,
    /// Raw IEEE-754 little-endian 32-bit float bits.
    Float32,
}

impl CanonicalBitDepth {
    /// Integer token used by the session declaration and CLI.
    #[must_use]
    pub const fn bits(self) -> u16 {
        match self {
            Self::Pcm16 => 16,
            Self::Pcm24 => 24,
            Self::Float32 => 32,
        }
    }

    /// Exact declaration/CLI token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Pcm16 => "16",
            Self::Pcm24 => "24",
            Self::Float32 => "32f",
        }
    }

    /// Parse one exact declaration/CLI token.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "16" => Some(Self::Pcm16),
            "24" => Some(Self::Pcm24),
            "32f" => Some(Self::Float32),
            _ => None,
        }
    }

    /// Exact bytes in one canonical sample.
    #[must_use]
    pub const fn bytes_per_sample(self) -> u16 {
        self.bits() / 8
    }
}

impl TryFrom<u16> for CanonicalBitDepth {
    type Error = StemHasherError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(Self::Pcm16),
            24 => Ok(Self::Pcm24),
            _ => Err(StemHasherError::new("shape.bit_depth.unsupported")),
        }
    }
}

/// The declaration facts that make canonical sample bytes interpretable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalPcmShape {
    /// Interleaved channel count.
    pub channels: u16,
    /// Exact canonical sample-depth token.
    pub bit_depth: CanonicalBitDepth,
    /// Exact interleaved frame count.
    pub frames: u64,
}

impl CanonicalPcmShape {
    /// Validate a nonempty canonical-PCM shape and its exact byte arithmetic.
    pub fn new(
        channels: u16,
        bit_depth: CanonicalBitDepth,
        frames: u64,
    ) -> Result<Self, StemHasherError> {
        if channels == 0 {
            return Err(StemHasherError::new("shape.channels.zero"));
        }
        if frames == 0 {
            return Err(StemHasherError::new("shape.frames.zero"));
        }
        let shape = Self {
            channels,
            bit_depth,
            frames,
        };
        let _ = shape.canonical_bytes()?;
        Ok(shape)
    }

    /// Exact `frames * channels * bit_depth / 8` preimage length.
    pub fn canonical_bytes(self) -> Result<u64, StemHasherError> {
        self.frames
            .checked_mul(u64::from(self.channels))
            .and_then(|samples| samples.checked_mul(u64::from(self.bit_depth.bytes_per_sample())))
            .ok_or_else(|| StemHasherError::new("shape.byte_length.overflow"))
    }
}

/// Successful canonicalization and SHA-256 identity result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StemIdentityReport {
    /// Shape derived from WAVE or supplied for raw PCM.
    pub shape: CanonicalPcmShape,
    /// SHA-256 digest of canonical sample bytes only.
    pub digest: [u8; 32],
    /// Exact bytes hashed and optionally emitted.
    pub canonical_bytes: u64,
}

impl StemIdentityReport {
    /// Canonical scheme-prefixed identity string.
    #[must_use]
    pub fn identity(self) -> String {
        format!("sha256:{}", lowercase_hex(self.digest))
    }
}

/// Stable reference-hasher error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StemHasherError {
    code: &'static str,
}

impl StemHasherError {
    const fn new(code: &'static str) -> Self {
        Self { code }
    }

    /// Stable lowercase dotted reason.
    #[must_use]
    pub const fn code(self) -> &'static str {
        self.code
    }
}

impl std::fmt::Display for StemHasherError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "stem-hasher.v1\t{}", self.code)
    }
}

impl std::error::Error for StemHasherError {}

/// Canonicalize and hash little-endian raw samples of the declared shape.
///
/// The input must contain exactly the declared number of bytes. `canonical_output` receives the
/// same samples in the normative serialization while SHA-256 is computed incrementally.
pub fn canonicalize_raw_pcm<R: Read, W: Write>(
    input: &mut R,
    shape: CanonicalPcmShape,
    canonical_output: &mut W,
) -> Result<StemIdentityReport, StemHasherError> {
    canonicalize_stream(input, shape, canonical_output, true)
}

/// Canonicalize and hash one RIFF/WAVE or RF64/WAVE input.
///
/// Container metadata is parsed through the engine's native source parser and excluded from the
/// preimage. Accepted depths are signed PCM16, packed signed PCM24, and raw-bit IEEE float32.
pub fn canonicalize_wave<R: Read + Seek, W: Write>(
    input: &mut R,
    canonical_output: &mut W,
) -> Result<StemIdentityReport, StemHasherError> {
    let metadata = parse_native_wave(
        input,
        NativeWaveParseCaps {
            max_chunk_count: WAVE_MAXIMUM_CHUNKS,
            max_skipped_metadata_bytes: WAVE_MAXIMUM_SKIPPED_METADATA_BYTES,
        },
    )
    .map_err(map_wave_error)?;
    let bit_depth = match metadata.encoding {
        NativeWaveEncoding::SignedPcm16 => CanonicalBitDepth::Pcm16,
        NativeWaveEncoding::SignedPcm24 => CanonicalBitDepth::Pcm24,
        NativeWaveEncoding::Float32 => CanonicalBitDepth::Float32,
        NativeWaveEncoding::UnsignedPcm8
        | NativeWaveEncoding::SignedPcm32
        | NativeWaveEncoding::Float64 => {
            return Err(StemHasherError::new("wave.bit_depth.unsupported"));
        }
    };
    let shape = CanonicalPcmShape::new(metadata.channel_count, bit_depth, metadata.total_frames)?;
    let expected_block_align = metadata
        .channel_count
        .checked_mul(bit_depth.bytes_per_sample())
        .ok_or_else(|| StemHasherError::new("shape.byte_length.overflow"))?;
    if metadata.block_align_bytes != expected_block_align
        || metadata.data_length_bytes != shape.canonical_bytes()?
    {
        return Err(StemHasherError::new("wave.shape.mismatch"));
    }
    input
        .seek(SeekFrom::Start(metadata.data_offset_bytes))
        .map_err(|_| StemHasherError::new("input.seek"))?;
    canonicalize_stream(input, shape, canonical_output, false)
}

fn canonicalize_stream<R: Read, W: Write>(
    input: &mut R,
    shape: CanonicalPcmShape,
    canonical_output: &mut W,
    require_end_of_input: bool,
) -> Result<StemIdentityReport, StemHasherError> {
    let canonical_bytes = shape.canonical_bytes()?;
    let sample_bytes = usize::from(shape.bit_depth.bytes_per_sample());
    let mut input_bytes = vec![0_u8; STREAM_BYTES];
    let mut canonical = Vec::with_capacity(STREAM_BYTES);
    let mut remaining = canonical_bytes;
    let mut hasher = Sha256::new();
    while remaining != 0 {
        let read_bytes = usize::try_from(remaining.min(STREAM_BYTES as u64))
            .map_err(|_| StemHasherError::new("shape.byte_length.overflow"))?;
        input
            .read_exact(&mut input_bytes[..read_bytes])
            .map_err(map_input_read_error)?;
        canonical.clear();
        for sample in input_bytes[..read_bytes].chunks_exact(sample_bytes) {
            serialize_sample(shape.bit_depth, sample, &mut canonical);
        }
        debug_assert_eq!(canonical.len(), read_bytes);
        hasher.update(&canonical);
        canonical_output
            .write_all(&canonical)
            .map_err(|_| StemHasherError::new("output.write"))?;
        remaining -= u64::try_from(read_bytes).expect("bounded chunk length fits u64");
    }
    if require_end_of_input {
        let mut extra = [0_u8; 1];
        match input.read(&mut extra) {
            Ok(0) => {}
            Ok(_) => return Err(StemHasherError::new("input.length.mismatch")),
            Err(_) => return Err(StemHasherError::new("input.read")),
        }
    }
    Ok(StemIdentityReport {
        shape,
        digest: hasher.finalize().into(),
        canonical_bytes,
    })
}

fn serialize_sample(bit_depth: CanonicalBitDepth, source: &[u8], output: &mut Vec<u8>) {
    match bit_depth {
        CanonicalBitDepth::Pcm16 => {
            let value = i16::from_le_bytes(source.try_into().expect("PCM16 sample width"));
            output.extend_from_slice(&value.to_le_bytes());
        }
        CanonicalBitDepth::Pcm24 => {
            let sign = if source[2] & 0x80 == 0 { 0x00 } else { 0xff };
            let value = i32::from_le_bytes([source[0], source[1], source[2], sign]);
            output.extend_from_slice(&value.to_le_bytes()[..3]);
        }
        CanonicalBitDepth::Float32 => {
            let bits = u32::from_le_bytes(source.try_into().expect("f32 sample width"));
            output.extend_from_slice(&bits.to_le_bytes());
        }
    }
}

fn map_input_read_error(error: io::Error) -> StemHasherError {
    if error.kind() == io::ErrorKind::UnexpectedEof {
        StemHasherError::new("input.length.mismatch")
    } else {
        StemHasherError::new("input.read")
    }
}

fn map_wave_error(error: NativeWaveError) -> StemHasherError {
    match error {
        NativeWaveError::Io(_) => StemHasherError::new("wave.io"),
        NativeWaveError::ContainerInvalid => StemHasherError::new("wave.container.invalid"),
        NativeWaveError::FormatUnsupported => StemHasherError::new("wave.format.unsupported"),
        NativeWaveError::ArithmeticOverflow => StemHasherError::new("wave.arithmetic.overflow"),
        NativeWaveError::ResourceLimit => StemHasherError::new("wave.resource.limit"),
        NativeWaveError::RegionOutOfBounds => StemHasherError::new("wave.region.out_of_bounds"),
        NativeWaveError::OutputShape => StemHasherError::new("wave.output.shape"),
    }
}

fn lowercase_hex(digest: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

/// CLI input kind and declaration facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliInput {
    /// RIFF/WAVE or RF64/WAVE; shape comes from the engine parser.
    Wave,
    /// Headerless signed little-endian PCM with an explicit shape.
    Raw(CanonicalPcmShape),
}

/// Optional canonical-preimage destination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliOutput {
    /// Hash only; print the identity to standard output.
    IdentityOnly,
    /// Create this file without replacing an existing path, then print identity to stdout.
    File(PathBuf),
    /// Write canonical bytes to stdout and the identity to stderr.
    Stdout,
}

/// Exact validated command-line arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliArgs {
    /// Input file path.
    pub input: PathBuf,
    /// Container/raw mode and shape authority.
    pub kind: CliInput,
    /// Optional canonical byte destination.
    pub output: CliOutput,
}

/// Parse the closed `wave`/`raw` CLI.
pub fn parse_cli(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<CliArgs, StemHasherError> {
    let mut arguments = arguments.into_iter();
    let mode = match arguments.next().as_deref().and_then(OsStr::to_str) {
        Some("wave") => CliInput::Wave,
        Some("raw") => CliInput::Raw(CanonicalPcmShape {
            channels: 0,
            bit_depth: CanonicalBitDepth::Pcm16,
            frames: 0,
        }),
        Some(_) => return Err(StemHasherError::new("cli.mode.unknown")),
        None => return Err(StemHasherError::new("cli.mode.missing")),
    };
    let mut input = None;
    let mut output = None;
    let mut channels = None;
    let mut bit_depth = None;
    let mut frames = None;
    while let Some(option) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| StemHasherError::new("cli.option.value.missing"))?;
        match option.to_str() {
            Some("--input") if input.is_none() => input = Some(PathBuf::from(value)),
            Some("--output") if output.is_none() => {
                output = Some(if value == OsStr::new("-") {
                    CliOutput::Stdout
                } else {
                    CliOutput::File(PathBuf::from(value))
                });
            }
            Some("--channels") if channels.is_none() => channels = Some(parse_u16(&value)?),
            Some("--bit-depth") if bit_depth.is_none() => {
                bit_depth = Some(parse_bit_depth(&value)?)
            }
            Some("--frames") if frames.is_none() => frames = Some(parse_u64(&value)?),
            Some("--input" | "--output" | "--channels" | "--bit-depth" | "--frames") => {
                return Err(StemHasherError::new("cli.option.duplicate"));
            }
            Some(_) | None => return Err(StemHasherError::new("cli.option.unknown")),
        }
    }
    let kind = match mode {
        CliInput::Wave => {
            if channels.is_some() || bit_depth.is_some() || frames.is_some() {
                return Err(StemHasherError::new("cli.wave.shape.forbidden"));
            }
            CliInput::Wave
        }
        CliInput::Raw(_) => CliInput::Raw(CanonicalPcmShape::new(
            channels.ok_or_else(|| StemHasherError::new("cli.channels.missing"))?,
            bit_depth.ok_or_else(|| StemHasherError::new("cli.bit_depth.missing"))?,
            frames.ok_or_else(|| StemHasherError::new("cli.frames.missing"))?,
        )?),
    };
    Ok(CliArgs {
        input: input.ok_or_else(|| StemHasherError::new("cli.input.missing"))?,
        kind,
        output: output.unwrap_or(CliOutput::IdentityOnly),
    })
}

fn parse_u16(value: &OsStr) -> Result<u16, StemHasherError> {
    value
        .to_str()
        .ok_or_else(|| StemHasherError::new("cli.scalar.utf8"))?
        .parse()
        .map_err(|_| StemHasherError::new("cli.scalar.invalid"))
}

fn parse_bit_depth(value: &OsStr) -> Result<CanonicalBitDepth, StemHasherError> {
    value
        .to_str()
        .ok_or_else(|| StemHasherError::new("cli.scalar.utf8"))
        .and_then(|value| {
            CanonicalBitDepth::from_token(value)
                .ok_or_else(|| StemHasherError::new("shape.bit_depth.unsupported"))
        })
}

fn parse_u64(value: &OsStr) -> Result<u64, StemHasherError> {
    value
        .to_str()
        .ok_or_else(|| StemHasherError::new("cli.scalar.utf8"))?
        .parse()
        .map_err(|_| StemHasherError::new("cli.scalar.invalid"))
}

/// Parse the CLI, stream the canonicalization, and report the identity.
pub fn run_cli(arguments: impl IntoIterator<Item = OsString>) -> Result<(), StemHasherError> {
    let arguments = parse_cli(arguments)?;
    let mut input = File::open(&arguments.input).map_err(|_| StemHasherError::new("input.open"))?;
    match arguments.output {
        CliOutput::IdentityOnly => {
            let report = canonicalize_cli_input(&mut input, arguments.kind, &mut io::sink())?;
            println!("{}", report.identity());
        }
        CliOutput::Stdout => {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let report = canonicalize_cli_input(&mut input, arguments.kind, &mut stdout)?;
            stdout
                .flush()
                .map_err(|_| StemHasherError::new("output.flush"))?;
            eprintln!("{}", report.identity());
        }
        CliOutput::File(path) => {
            let report = write_canonical_file(&mut input, arguments.kind, &path)?;
            println!("{}", report.identity());
        }
    }
    Ok(())
}

fn canonicalize_cli_input<R: Read + Seek, W: Write>(
    input: &mut R,
    kind: CliInput,
    output: &mut W,
) -> Result<StemIdentityReport, StemHasherError> {
    match kind {
        CliInput::Wave => canonicalize_wave(input, output),
        CliInput::Raw(shape) => canonicalize_raw_pcm(input, shape, output),
    }
}

fn write_canonical_file<R: Read + Seek>(
    input: &mut R,
    kind: CliInput,
    path: &PathBuf,
) -> Result<StemIdentityReport, StemHasherError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| StemHasherError::new("output.create"))?;
    let result = canonicalize_cli_input(input, kind, &mut output).and_then(|report| {
        output
            .flush()
            .and_then(|()| output.sync_all())
            .map_err(|_| StemHasherError::new("output.flush"))?;
        Ok(report)
    });
    if result.is_err() {
        drop(output);
        let _ = fs::remove_file(path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_is_closed_and_raw_shape_is_exact() {
        let raw = parse_cli(
            [
                "raw",
                "--input",
                "stem.pcm",
                "--channels",
                "2",
                "--bit-depth",
                "24",
                "--frames",
                "3",
            ]
            .map(OsString::from),
        )
        .expect("raw CLI");
        assert_eq!(
            raw.kind,
            CliInput::Raw(CanonicalPcmShape::new(2, CanonicalBitDepth::Pcm24, 3).expect("shape"))
        );
        assert_eq!(
            parse_cli(["wave", "--input", "stem.wav"].map(OsString::from))
                .expect("wave CLI")
                .kind,
            CliInput::Wave
        );
        for arguments in [
            vec!["raw"],
            vec![
                "raw",
                "--input",
                "x",
                "--channels",
                "0",
                "--bit-depth",
                "16",
                "--frames",
                "1",
            ],
            vec![
                "raw",
                "--input",
                "x",
                "--channels",
                "1",
                "--bit-depth",
                "32",
                "--frames",
                "1",
            ],
            vec!["wave", "--input", "x", "--channels", "1"],
            vec!["wave", "--input", "x", "--input", "y"],
            vec!["other", "--input", "x"],
        ] {
            assert!(parse_cli(arguments.into_iter().map(OsString::from)).is_err());
        }
    }

    #[test]
    fn raw_length_is_total_and_output_is_canonical() {
        let shape = CanonicalPcmShape::new(1, CanonicalBitDepth::Pcm16, 2).expect("shape");
        let mut output = Vec::new();
        let report = canonicalize_raw_pcm(&mut &b"\x00\x80\xff\x7f"[..], shape, &mut output)
            .expect("canonical");
        assert_eq!(output, b"\x00\x80\xff\x7f");
        assert_eq!(report.canonical_bytes, 4);
        for bytes in [&b"\x00\x80\xff"[..], &b"\x00\x80\xff\x7f\x00"[..]] {
            assert_eq!(
                canonicalize_raw_pcm(&mut &*bytes, shape, &mut io::sink())
                    .expect_err("length")
                    .code(),
                "input.length.mismatch"
            );
        }
    }

    #[test]
    fn wave_depth_set_rejects_integer_pcm32_but_accepts_float32() {
        let format = [
            1, 0, // integer PCM
            1, 0, // one channel
            0x80, 0xbb, 0, 0, // 48 kHz
            0, 0xee, 2, 0, // 192,000 bytes/s
            4, 0, // four-byte frame
            32, 0, // 32-bit PCM
        ];
        let mut wave = Vec::new();
        wave.extend_from_slice(b"RIFF");
        wave.extend_from_slice(&40_u32.to_le_bytes());
        wave.extend_from_slice(b"WAVEfmt ");
        wave.extend_from_slice(&16_u32.to_le_bytes());
        wave.extend_from_slice(&format);
        wave.extend_from_slice(b"data");
        wave.extend_from_slice(&4_u32.to_le_bytes());
        wave.extend_from_slice(&[0; 4]);
        assert_eq!(wave.len(), 48);
        assert_eq!(
            canonicalize_wave(&mut io::Cursor::new(wave), &mut io::sink())
                .expect_err("PCM32 is outside the launch set")
                .code(),
            "wave.bit_depth.unsupported"
        );

        let mut float_wave = Vec::new();
        float_wave.extend_from_slice(b"RIFF");
        float_wave.extend_from_slice(&40_u32.to_le_bytes());
        float_wave.extend_from_slice(b"WAVEfmt ");
        float_wave.extend_from_slice(&16_u32.to_le_bytes());
        float_wave.extend_from_slice(&[
            3, 0, // IEEE float
            1, 0, // one channel
            0x80, 0xbb, 0, 0, // 48 kHz
            0, 0xee, 2, 0, // 192,000 bytes/s
            4, 0, // four-byte frame
            32, 0, // 32-bit float
        ]);
        float_wave.extend_from_slice(b"data");
        float_wave.extend_from_slice(&4_u32.to_le_bytes());
        float_wave.extend_from_slice(&0x8000_0000_u32.to_le_bytes());
        let report = canonicalize_wave(&mut io::Cursor::new(float_wave), &mut io::sink())
            .expect("float32 is a canonical identity depth");
        assert_eq!(report.shape.bit_depth, CanonicalBitDepth::Float32);
    }
}
