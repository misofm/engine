//! Bit-exact FLAC-to-canonical-PCM decoder used by native tooling and the shipped Wasm adapter.
//!
//! FLAC bytes are transport only. This crate emits the samples-only, interleaved, source-depth
//! little-endian serialization defined by `docs/STEM_IDENTITY_V1.md`. Decoding is incremental at
//! FLAC packet boundaries so an ingest Worker can hash and persist one bounded block at a time.

use std::io::{Cursor, Error as IoError, Write};

use symphonia::core::{
    audio::GenericAudioBufferRef,
    codecs::{CodecParameters, audio::AudioDecoderOptions},
    formats::{FormatOptions, FormatReader, TrackType, probe::Hint},
    io::{MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
};

#[cfg(target_arch = "wasm32")]
mod ffi;

/// Stable ABI version exported by the Wasm decoder artifact.
pub const ABI_VERSION: u32 = 0x0001_0000;

/// Successful decoder operation.
pub const RESULT_OK: u32 = 0;
/// The stream has been completely decoded and its exact declared length was observed.
pub const RESULT_END: u32 = 1;
/// A caller argument or decoder state was invalid.
pub const RESULT_INVALID_ARGUMENT: u32 = 2;
/// FLAC syntax, checksums, or frame sequence were invalid.
pub const RESULT_DECODE_REFUSED: u32 = 3;
/// The FLAC source-native sample depth is outside launch `{16, 24}`.
pub const RESULT_BIT_DEPTH_UNSUPPORTED: u32 = 4;
/// STREAMINFO shape or decoded length did not agree with the actual packets.
pub const RESULT_SHAPE_MISMATCH: u32 = 5;
/// Declared canonical PCM exceeds the caller's ingest budget.
pub const RESULT_RESOURCE_LIMIT: u32 = 6;
/// An output sink refused a decoded block.
pub const RESULT_OUTPUT_WRITE: u32 = 7;
/// A checked implementation invariant failed.
pub const RESULT_INTERNAL: u32 = 255;

/// Source-native launch bit depth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlacBitDepth {
    /// Signed 16-bit PCM.
    Pcm16,
    /// Signed packed 24-bit PCM.
    Pcm24,
}

impl FlacBitDepth {
    /// Integer token carried by FLAC STREAMINFO and the session declaration.
    #[must_use]
    pub const fn bits(self) -> u16 {
        match self {
            Self::Pcm16 => 16,
            Self::Pcm24 => 24,
        }
    }

    /// Canonical bytes occupied by one sample.
    #[must_use]
    pub const fn bytes_per_sample(self) -> u16 {
        self.bits() / 8
    }

    const fn sample_in_range(self, sample: i32) -> bool {
        match self {
            Self::Pcm16 => sample >= i16::MIN as i32 && sample <= i16::MAX as i32,
            Self::Pcm24 => sample >= -8_388_608 && sample <= 8_388_607,
        }
    }
}

impl TryFrom<u32> for FlacBitDepth {
    type Error = FlacDecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(Self::Pcm16),
            24 => Ok(Self::Pcm24),
            _ => Err(FlacDecodeError::new(
                RESULT_BIT_DEPTH_UNSUPPORTED,
                "flac.bit_depth.unsupported",
            )),
        }
    }
}

/// Shape and byte count established from STREAMINFO before the first decoded block is emitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlacStreamInfo {
    /// Container sample rate. There is no implicit sample-rate conversion.
    pub sample_rate_hz: u32,
    /// Interleaved channel count.
    pub channels: u16,
    /// Source-native integer depth.
    pub bit_depth: FlacBitDepth,
    /// Exact frame count declared by STREAMINFO.
    pub frames: u64,
    /// Exact canonical output length.
    pub canonical_bytes: u64,
    /// Minimum encoded FLAC block size recorded in STREAMINFO.
    pub minimum_block_frames: u16,
    /// Maximum encoded FLAC block size recorded in STREAMINFO.
    pub maximum_block_frames: u16,
}

/// Successful full-stream decode report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlacDecodeReport {
    /// Validated stream facts.
    pub stream: FlacStreamInfo,
    /// Frames emitted to the canonical PCM sink.
    pub decoded_frames: u64,
    /// Canonical bytes emitted to the sink.
    pub decoded_bytes: u64,
}

/// Typed decoder refusal shared by native and Wasm adapters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlacDecodeError {
    result: u32,
    code: &'static str,
}

impl FlacDecodeError {
    const fn new(result: u32, code: &'static str) -> Self {
        Self { result, code }
    }

    /// Stable numeric Wasm result.
    #[must_use]
    pub const fn result(self) -> u32 {
        self.result
    }

    /// Stable dotted diagnostic code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        self.code
    }
}

impl std::fmt::Display for FlacDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "miso.flac.decoder.v1\t{}", self.code)
    }
}

impl std::error::Error for FlacDecodeError {}

/// Incremental FLAC decoder whose current output is one interleaved canonical-PCM block.
pub struct FlacBlockDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::audio::AudioDecoder>,
    track_id: u32,
    stream: FlacStreamInfo,
    decoded_samples: Vec<i32>,
    canonical_block: Vec<u8>,
    decoded_frames: u64,
    ended: bool,
}

impl FlacBlockDecoder {
    /// Parse STREAMINFO and enforce the launch depths and caller byte budget.
    pub fn new(input: Vec<u8>, maximum_canonical_bytes: u64) -> Result<Self, FlacDecodeError> {
        let (minimum_block_frames, maximum_block_frames) = parse_streaminfo_block_sizes(&input)?;
        let source = MediaSourceStream::new(
            Box::new(Cursor::new(input)),
            MediaSourceStreamOptions::default(),
        );
        let mut hint = Hint::new();
        hint.with_extension("flac");
        let format = symphonia::default::get_probe()
            .probe(
                &hint,
                source,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .map_err(map_symphonia_error)?;
        let track = format
            .default_track(TrackType::Audio)
            .ok_or_else(|| FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.track.missing"))?;
        let track_id = track.id;
        let frames = track
            .num_frames
            .filter(|frames| *frames != 0)
            .ok_or_else(|| FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.frames.missing"))?;
        let audio = match track.codec_params.as_ref() {
            Some(CodecParameters::Audio(audio)) => audio,
            _ => {
                return Err(FlacDecodeError::new(
                    RESULT_SHAPE_MISMATCH,
                    "flac.codec_parameters.missing",
                ));
            }
        };
        let sample_rate_hz = audio.sample_rate.filter(|rate| *rate != 0).ok_or_else(|| {
            FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.sample_rate.missing")
        })?;
        let channel_count = audio
            .channels
            .as_ref()
            .map_or(0, |channels| channels.count());
        let channels = u16::try_from(channel_count)
            .map_err(|_| FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.channels.invalid"))?;
        if channels == 0 {
            return Err(FlacDecodeError::new(
                RESULT_SHAPE_MISMATCH,
                "flac.channels.invalid",
            ));
        }
        let bit_depth = FlacBitDepth::try_from(audio.bits_per_sample.unwrap_or(0))?;
        let canonical_bytes = frames
            .checked_mul(u64::from(channels))
            .and_then(|samples| samples.checked_mul(u64::from(bit_depth.bytes_per_sample())))
            .ok_or_else(|| {
                FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.byte_length.overflow")
            })?;
        if canonical_bytes > maximum_canonical_bytes {
            return Err(FlacDecodeError::new(
                RESULT_RESOURCE_LIMIT,
                "flac.canonical_bytes.limit",
            ));
        }
        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(audio, &AudioDecoderOptions::default().verify(true))
            .map_err(map_symphonia_error)?;
        Ok(Self {
            format,
            decoder,
            track_id,
            stream: FlacStreamInfo {
                sample_rate_hz,
                channels,
                bit_depth,
                frames,
                canonical_bytes,
                minimum_block_frames,
                maximum_block_frames,
            },
            decoded_samples: Vec::new(),
            canonical_block: Vec::new(),
            decoded_frames: 0,
            ended: false,
        })
    }

    /// Validated STREAMINFO facts available before block decoding begins.
    #[must_use]
    pub const fn stream_info(&self) -> FlacStreamInfo {
        self.stream
    }

    /// Decode the next FLAC packet into one canonical interleaved PCM block.
    ///
    /// Returns `true` when a block is available through [`Self::canonical_block`], and `false`
    /// only after exact EOF, total-frame validation, and the decoder's FLAC MD5 verification.
    pub fn decode_next_block(&mut self) -> Result<bool, FlacDecodeError> {
        if self.ended {
            return Ok(false);
        }
        loop {
            let Some(packet) = self.format.next_packet().map_err(map_symphonia_error)? else {
                self.canonical_block.clear();
                let finalized = self.decoder.finalize();
                if self.decoded_frames != self.stream.frames || finalized.verify_ok == Some(false) {
                    return Err(FlacDecodeError::new(
                        RESULT_SHAPE_MISMATCH,
                        "flac.frames_or_md5.mismatch",
                    ));
                }
                self.ended = true;
                return Ok(false);
            };
            if packet.track_id != self.track_id {
                continue;
            }
            let audio = self.decoder.decode(&packet).map_err(map_symphonia_error)?;
            if audio.num_planes() != usize::from(self.stream.channels)
                || audio.spec().rate() != self.stream.sample_rate_hz
                || audio.frames() == 0
            {
                return Err(FlacDecodeError::new(
                    RESULT_SHAPE_MISMATCH,
                    "flac.block.shape",
                ));
            }
            if !matches!(audio, GenericAudioBufferRef::S32(_)) {
                return Err(FlacDecodeError::new(
                    RESULT_INTERNAL,
                    "flac.decoder.sample_format",
                ));
            }
            self.decoded_samples.clear();
            audio.copy_to_vec_interleaved(&mut self.decoded_samples);
            let block_frames = u64::try_from(audio.frames()).map_err(|_| {
                FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.block_frames.overflow")
            })?;
            let next_frames = self
                .decoded_frames
                .checked_add(block_frames)
                .ok_or_else(|| {
                    FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.frames.overflow")
                })?;
            if next_frames > self.stream.frames {
                return Err(FlacDecodeError::new(
                    RESULT_SHAPE_MISMATCH,
                    "flac.frames.mismatch",
                ));
            }
            let block_bytes = self
                .decoded_samples
                .len()
                .checked_mul(usize::from(self.stream.bit_depth.bytes_per_sample()))
                .ok_or_else(|| {
                    FlacDecodeError::new(RESULT_SHAPE_MISMATCH, "flac.block_bytes.overflow")
                })?;
            self.canonical_block.clear();
            self.canonical_block.reserve(block_bytes);
            let shift = 32 - u32::from(self.stream.bit_depth.bits());
            for &left_aligned in &self.decoded_samples {
                serialize_sample(
                    self.stream.bit_depth,
                    left_aligned >> shift,
                    &mut self.canonical_block,
                )?;
            }
            if self.canonical_block.len() != block_bytes {
                return Err(FlacDecodeError::new(
                    RESULT_INTERNAL,
                    "flac.block_bytes.internal",
                ));
            }
            self.decoded_frames = next_frames;
            return Ok(true);
        }
    }

    /// Current decoded block in canonical serialization.
    #[must_use]
    pub fn canonical_block(&self) -> &[u8] {
        &self.canonical_block
    }

    /// Successful report after [`Self::decode_next_block`] returns `false`.
    pub fn finish_report(&self) -> Result<FlacDecodeReport, FlacDecodeError> {
        if !self.ended || self.decoded_frames != self.stream.frames {
            return Err(FlacDecodeError::new(
                RESULT_INVALID_ARGUMENT,
                "flac.state.not_finished",
            ));
        }
        Ok(FlacDecodeReport {
            stream: self.stream,
            decoded_frames: self.decoded_frames,
            decoded_bytes: self.stream.canonical_bytes,
        })
    }
}

/// Decode a complete FLAC stream incrementally into a canonical-PCM sink.
pub fn decode_flac_to_writer<W: Write>(
    input: &[u8],
    maximum_canonical_bytes: u64,
    output: &mut W,
) -> Result<FlacDecodeReport, FlacDecodeError> {
    let mut decoder = FlacBlockDecoder::new(input.to_vec(), maximum_canonical_bytes)?;
    while decoder.decode_next_block()? {
        output
            .write_all(decoder.canonical_block())
            .map_err(map_output_error)?;
    }
    decoder.finish_report()
}

fn parse_streaminfo_block_sizes(input: &[u8]) -> Result<(u16, u16), FlacDecodeError> {
    if input.len() < 12
        || &input[..4] != b"fLaC"
        || input[4] & 0x7f != 0
        || u32::from_be_bytes([0, input[5], input[6], input[7]]) != 34
    {
        return Err(FlacDecodeError::new(
            RESULT_DECODE_REFUSED,
            "flac.streaminfo.invalid",
        ));
    }
    let minimum = u16::from_be_bytes([input[8], input[9]]);
    let maximum = u16::from_be_bytes([input[10], input[11]]);
    if minimum == 0 || maximum < minimum {
        return Err(FlacDecodeError::new(
            RESULT_SHAPE_MISMATCH,
            "flac.block_size.invalid",
        ));
    }
    Ok((minimum, maximum))
}

fn serialize_sample(
    bit_depth: FlacBitDepth,
    sample: i32,
    output: &mut Vec<u8>,
) -> Result<(), FlacDecodeError> {
    if !bit_depth.sample_in_range(sample) {
        return Err(FlacDecodeError::new(
            RESULT_SHAPE_MISMATCH,
            "flac.sample.out_of_range",
        ));
    }
    match bit_depth {
        FlacBitDepth::Pcm16 => output.extend_from_slice(&(sample as i16).to_le_bytes()),
        FlacBitDepth::Pcm24 => output.extend_from_slice(&sample.to_le_bytes()[..3]),
    }
    Ok(())
}

fn map_symphonia_error(_error: symphonia::core::errors::Error) -> FlacDecodeError {
    FlacDecodeError::new(RESULT_DECODE_REFUSED, "flac.decode.refused")
}

fn map_output_error(_error: IoError) -> FlacDecodeError {
    FlacDecodeError::new(RESULT_OUTPUT_WRITE, "flac.output.write")
}
