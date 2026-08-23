//! Native little-endian RIFF/WAVE and RF64/WAVE metadata parsing and bounded PCM decoding.
//!
//! This module is deliberately excluded from browser Wasm artifacts. It is a worker/control-plane
//! boundary: file seek/read operations and decoding must never be called from a render callback.

use core::{cmp, num::NonZeroUsize};
use std::io::{Read, Seek, SeekFrom};

use miso_engine_core::SampleRateHz;

use crate::{SourceDiagnosticCode, SourceFrame};

const RIFF: [u8; 4] = *b"RIFF";
const RF64: [u8; 4] = *b"RF64";
const WAVE: [u8; 4] = *b"WAVE";
const FMT: [u8; 4] = *b"fmt ";
const DATA: [u8; 4] = *b"data";
const DS64: [u8; 4] = *b"ds64";
const RF64_PLACEHOLDER: u32 = u32::MAX;

const PCM_GUID: [u8; 16] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
];
const IEEE_FLOAT_GUID: [u8; 16] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
];

/// A WAVE container family accepted by the native source path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeWaveContainer {
    /// Classic little-endian RIFF/WAVE.
    Riff,
    /// RF64/WAVE with mandatory `ds64` size metadata.
    Rf64,
}

/// One supported interleaved native WAVE scalar encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeWaveEncoding {
    /// Unsigned 8-bit PCM.
    UnsignedPcm8,
    /// Signed little-endian 16-bit PCM.
    SignedPcm16,
    /// Signed little-endian packed 24-bit PCM.
    SignedPcm24,
    /// Signed little-endian 32-bit PCM.
    SignedPcm32,
    /// IEEE little-endian 32-bit float.
    Float32,
    /// IEEE little-endian 64-bit float.
    Float64,
}

impl NativeWaveEncoding {
    /// Bytes retained in a container sample for this encoding.
    #[must_use]
    pub const fn bytes_per_sample(self) -> u16 {
        match self {
            Self::UnsignedPcm8 => 1,
            Self::SignedPcm16 => 2,
            Self::SignedPcm24 => 3,
            Self::SignedPcm32 | Self::Float32 => 4,
            Self::Float64 => 8,
        }
    }
}

/// Parser limits that bound chunk traversal and skipped metadata without retaining payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeWaveParseCaps {
    /// Maximum chunk headers inspected in one container.
    pub max_chunk_count: u32,
    /// Maximum total unknown or RF64-table metadata bytes skipped by the parser.
    pub max_skipped_metadata_bytes: u64,
}

/// Parsed native WAVE metadata; no decoded data payload is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeWaveMetadata {
    /// Container family.
    pub container: NativeWaveContainer,
    /// Decoded source sample rate carried losslessly from `fmt `.
    pub sample_rate_hz: SampleRateHz,
    /// Interleaved source channel count.
    pub channel_count: u16,
    /// Accepted source scalar encoding.
    pub encoding: NativeWaveEncoding,
    /// Exact bytes in one interleaved source frame.
    pub block_align_bytes: u16,
    /// Exact bytes per second declared in `fmt `.
    pub byte_rate: u32,
    /// Absolute byte offset of the `data` payload.
    pub data_offset_bytes: u64,
    /// Exact declared `data` payload bytes.
    pub data_length_bytes: u64,
    /// Exact decoded source frame count.
    pub total_frames: u64,
}

/// A finite source-frame region selected before decoding begins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeWaveRegion {
    /// Absolute first decoded source frame.
    pub start_frame: SourceFrame,
    /// Exact source frames available in the region.
    pub length_frames: u64,
}

/// Native WAVE parsing or decode rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeWaveError {
    /// File I/O failed outside the render plane.
    Io(std::io::ErrorKind),
    /// RIFF, RF64, chunk, or declared-size structure is malformed.
    ContainerInvalid,
    /// The container is valid enough to identify an unsupported native format.
    FormatUnsupported,
    /// Checked offset, size, or platform conversion overflowed.
    ArithmeticOverflow,
    /// A fixed parser/decode cap was exceeded before retaining data.
    ResourceLimit,
    /// A requested region exceeds the parsed source frame range.
    RegionOutOfBounds,
    /// A caller-supplied planar destination disagrees with prepared decode shape.
    OutputShape,
}

impl NativeWaveError {
    /// Stable source diagnostic code for this native boundary result.
    #[must_use]
    pub const fn diagnostic_code(self) -> SourceDiagnosticCode {
        match self {
            Self::Io(_) | Self::ContainerInvalid => SourceDiagnosticCode::ContainerInvalid,
            Self::FormatUnsupported => SourceDiagnosticCode::FormatUnsupported,
            Self::ArithmeticOverflow => SourceDiagnosticCode::ResourceArithmeticOverflow,
            Self::ResourceLimit => SourceDiagnosticCode::ResourceLimit,
            Self::RegionOutOfBounds => SourceDiagnosticCode::RegionOutOfBounds,
            Self::OutputShape => SourceDiagnosticCode::ChannelsMismatch,
        }
    }
}

/// One bounded native decoder result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeDecodeReport {
    /// Source frames copied to caller-owned planar output.
    pub decoded_frames: u32,
    /// Whether the declared region is exhausted after this call.
    pub end_of_region: bool,
    /// Cumulative sanitations of non-finite or subnormal decoded floats.
    pub sanitized_sample_count: u64,
}

/// A native decoder with one preallocated interleaved read scratch allocation.
pub struct NativeWaveDecoder<R: Read + Seek> {
    reader: R,
    metadata: NativeWaveMetadata,
    region: NativeWaveRegion,
    next_region_frame: u64,
    max_frames_per_decode: NonZeroUsize,
    scratch: Box<[u8]>,
    reader_position: Option<u64>,
    buffer_region_frame: u64,
    buffer_frames: usize,
    buffer_cursor: usize,
    sanitized_sample_count: u64,
}

pub(crate) trait PlanarSink {
    fn plane(&mut self, channel: usize) -> &mut [f32];
}

impl PlanarSink for [&mut [f32]] {
    #[inline(always)]
    fn plane(&mut self, channel: usize) -> &mut [f32] {
        self[channel]
    }
}

pub(crate) struct Strided<'a> {
    out: &'a mut [f32],
    stride: usize,
}

impl PlanarSink for Strided<'_> {
    #[inline(always)]
    fn plane(&mut self, channel: usize) -> &mut [f32] {
        let start = channel * self.stride;
        &mut self.out[start..start + self.stride]
    }
}

impl<R: Read + Seek> NativeWaveDecoder<R> {
    /// Preallocate bounded worker scratch and validate the exact requested region.
    pub fn prepare(
        reader: R,
        metadata: NativeWaveMetadata,
        region: NativeWaveRegion,
        max_frames_per_decode: NonZeroUsize,
    ) -> Result<Self, NativeWaveError> {
        validate_region(metadata, region)?;
        let scratch_bytes = max_frames_per_decode
            .get()
            .checked_mul(usize::from(metadata.block_align_bytes))
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let mut scratch = Vec::new();
        scratch
            .try_reserve_exact(scratch_bytes)
            .map_err(|_| NativeWaveError::ResourceLimit)?;
        scratch.resize(scratch_bytes, 0);
        Ok(Self {
            reader,
            metadata,
            region,
            next_region_frame: 0,
            max_frames_per_decode,
            scratch: scratch.into_boxed_slice(),
            reader_position: None,
            buffer_region_frame: 0,
            buffer_frames: 0,
            buffer_cursor: 0,
            sanitized_sample_count: 0,
        })
    }

    /// Decode a contiguous prefix of the prepared region into caller-owned planar storage.
    ///
    /// All planes must have the same length, that length must be no larger than the prepared
    /// decode bound, and no allocation occurs after [`Self::prepare`].
    pub fn decode_into(
        &mut self,
        output_planes: &mut [&mut [f32]],
    ) -> Result<NativeDecodeReport, NativeWaveError> {
        let requested_frames = self.validate_output_shape(output_planes)?;
        self.decode_next(requested_frames, output_planes)
    }

    fn decode_next<S: PlanarSink + ?Sized>(
        &mut self,
        requested_frames: usize,
        sink: &mut S,
    ) -> Result<NativeDecodeReport, NativeWaveError> {
        let remaining = self
            .region
            .length_frames
            .saturating_sub(self.next_region_frame);
        let decoded_frames = cmp::min(
            u64::try_from(requested_frames).expect("usize fits u64"),
            remaining,
        );
        let decoded_frames_usize =
            usize::try_from(decoded_frames).map_err(|_| NativeWaveError::ArithmeticOverflow)?;
        let mut copied = 0;
        while copied < decoded_frames_usize {
            if self.buffer_cursor == self.buffer_frames {
                self.fill_buffer()?;
            }
            let chunk = cmp::min(
                decoded_frames_usize - copied,
                self.buffer_frames - self.buffer_cursor,
            );
            let block_align = usize::from(self.metadata.block_align_bytes);
            let source_start = self.buffer_cursor * block_align;
            let source_end = (self.buffer_cursor + chunk) * block_align;
            let sanitized = convert_frames(
                self.metadata,
                &self.scratch[source_start..source_end],
                block_align,
                usize::from(self.metadata.channel_count),
                chunk,
                sink,
                copied,
            );
            self.sanitized_sample_count = self.sanitized_sample_count.saturating_add(sanitized);
            self.buffer_cursor += chunk;
            copied += chunk;
            self.next_region_frame = self
                .next_region_frame
                .checked_add(u64::try_from(chunk).expect("usize fits u64"))
                .ok_or(NativeWaveError::ArithmeticOverflow)?;
        }
        Ok(NativeDecodeReport {
            decoded_frames: u32::try_from(decoded_frames_usize)
                .map_err(|_| NativeWaveError::ArithmeticOverflow)?,
            end_of_region: self.next_region_frame == self.region.length_frames,
            sanitized_sample_count: self.sanitized_sample_count,
        })
    }

    /// Exact parsed metadata used by this prepared decoder.
    #[must_use]
    pub const fn metadata(&self) -> NativeWaveMetadata {
        self.metadata
    }

    /// Exact prepared finite decode region.
    #[must_use]
    pub const fn region(&self) -> NativeWaveRegion {
        self.region
    }

    /// Return the absolute source frame that the next decode will read.
    #[must_use]
    pub(crate) fn next_source_frame(&self) -> SourceFrame {
        SourceFrame(
            self.region
                .start_frame
                .0
                .saturating_add(self.next_region_frame),
        )
    }

    /// Reposition the prepared decoder at a validated absolute source frame in its region.
    pub(crate) fn seek_to_source_frame(
        &mut self,
        frame: SourceFrame,
    ) -> Result<(), NativeWaveError> {
        validate_seek_frame(self.region, frame)?;
        let target = frame.0 - self.region.start_frame.0;
        let buffered_end = self
            .buffer_region_frame
            .checked_add(u64::try_from(self.buffer_frames).expect("usize fits u64"))
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        if target >= self.buffer_region_frame && target < buffered_end {
            self.buffer_cursor = usize::try_from(target - self.buffer_region_frame)
                .map_err(|_| NativeWaveError::ArithmeticOverflow)?;
        } else {
            self.buffer_region_frame = target;
            self.buffer_frames = 0;
            self.buffer_cursor = 0;
        }
        self.next_region_frame = target;
        Ok(())
    }

    /// Decode into a contiguous `[channel][frames]` worker block.
    pub(crate) fn decode_planar(
        &mut self,
        out: &mut [f32],
        frames: usize,
    ) -> Result<NativeDecodeReport, NativeWaveError> {
        let channels = usize::from(self.metadata.channel_count);
        let expected_samples = channels
            .checked_mul(frames)
            .ok_or(NativeWaveError::OutputShape)?;
        if channels == 0
            || frames == 0
            || frames > self.max_frames_per_decode.get()
            || out.len() != expected_samples
        {
            return Err(NativeWaveError::OutputShape);
        }
        self.decode_next(
            frames,
            &mut Strided {
                out,
                stride: frames,
            },
        )
    }

    fn validate_output_shape(
        &self,
        output_planes: &mut [&mut [f32]],
    ) -> Result<usize, NativeWaveError> {
        if output_planes.len() != usize::from(self.metadata.channel_count) {
            return Err(NativeWaveError::OutputShape);
        }
        let Some(first) = output_planes.first() else {
            return Err(NativeWaveError::OutputShape);
        };
        let frames = first.len();
        if frames > self.max_frames_per_decode.get()
            || output_planes.iter().any(|plane| plane.len() != frames)
        {
            return Err(NativeWaveError::OutputShape);
        }
        Ok(frames)
    }

    fn fill_buffer(&mut self) -> Result<(), NativeWaveError> {
        let remaining = self
            .region
            .length_frames
            .checked_sub(self.next_region_frame)
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let decoded_frames = cmp::min(
            self.max_frames_per_decode.get(),
            usize::try_from(remaining).unwrap_or(usize::MAX),
        );
        if decoded_frames == 0 {
            self.buffer_region_frame = self.next_region_frame;
            self.buffer_frames = 0;
            self.buffer_cursor = 0;
            return Ok(());
        }
        let source_frame = self
            .region
            .start_frame
            .0
            .checked_add(self.next_region_frame)
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let byte_offset = source_frame
            .checked_mul(u64::from(self.metadata.block_align_bytes))
            .and_then(|offset| self.metadata.data_offset_bytes.checked_add(offset))
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let read_bytes = decoded_frames
            .checked_mul(usize::from(self.metadata.block_align_bytes))
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        if self.reader_position != Some(byte_offset)
            && let Err(error) = self.reader.seek(SeekFrom::Start(byte_offset))
        {
            self.reader_position = None;
            return Err(io_error(error));
        }
        if let Err(error) = self.reader.read_exact(&mut self.scratch[..read_bytes]) {
            self.reader_position = None;
            return Err(io_error(error));
        }
        self.reader_position = Some(
            byte_offset
                .checked_add(u64::try_from(read_bytes).expect("usize fits u64"))
                .ok_or(NativeWaveError::ArithmeticOverflow)?,
        );
        self.buffer_region_frame = self.next_region_frame;
        self.buffer_frames = decoded_frames;
        self.buffer_cursor = 0;
        Ok(())
    }
}

/// Parse a native RIFF/WAVE or RF64/WAVE file without retaining a data payload.
pub fn parse_native_wave<R: Read + Seek>(
    reader: &mut R,
    caps: NativeWaveParseCaps,
) -> Result<NativeWaveMetadata, NativeWaveError> {
    if caps.max_chunk_count == 0 {
        return Err(NativeWaveError::ResourceLimit);
    }
    let file_length = reader.seek(SeekFrom::End(0)).map_err(io_error)?;
    reader.seek(SeekFrom::Start(0)).map_err(io_error)?;
    if file_length < 12 {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let mut header = [0_u8; 12];
    reader.read_exact(&mut header).map_err(io_error)?;
    let container = if header[..4] == RIFF {
        NativeWaveContainer::Riff
    } else if header[..4] == RF64 {
        NativeWaveContainer::Rf64
    } else {
        return Err(NativeWaveError::ContainerInvalid);
    };
    if header[8..12] != WAVE {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let root_size = le_u32(&header[4..8]);
    let riff_end = match container {
        NativeWaveContainer::Riff => {
            if root_size == RF64_PLACEHOLDER {
                return Err(NativeWaveError::ContainerInvalid);
            }
            let end = 8_u64
                .checked_add(u64::from(root_size))
                .ok_or(NativeWaveError::ArithmeticOverflow)?;
            if end != file_length {
                return Err(NativeWaveError::ContainerInvalid);
            }
            end
        }
        NativeWaveContainer::Rf64 => {
            if root_size != RF64_PLACEHOLDER {
                return Err(NativeWaveError::ContainerInvalid);
            }
            file_length
        }
    };
    let mut cursor = 12_u64;
    let mut chunk_count = 0_u32;
    let mut skipped_metadata_bytes = 0_u64;
    let mut format = None;
    let mut data = None;
    let mut ds64: Option<Ds64Chunk> = None;
    while cursor < riff_end {
        if riff_end - cursor < 8 {
            return Err(NativeWaveError::ContainerInvalid);
        }
        chunk_count = chunk_count.saturating_add(1);
        if chunk_count > caps.max_chunk_count {
            return Err(NativeWaveError::ResourceLimit);
        }
        reader.seek(SeekFrom::Start(cursor)).map_err(io_error)?;
        let mut chunk_header = [0_u8; 8];
        reader.read_exact(&mut chunk_header).map_err(io_error)?;
        let chunk_id = [
            chunk_header[0],
            chunk_header[1],
            chunk_header[2],
            chunk_header[3],
        ];
        let raw_size = le_u32(&chunk_header[4..8]);
        let payload_offset = cursor
            .checked_add(8)
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let payload_size = if chunk_id == DATA && container == NativeWaveContainer::Rf64 {
            if raw_size != RF64_PLACEHOLDER {
                return Err(NativeWaveError::ContainerInvalid);
            }
            let Some(info) = ds64 else {
                return Err(NativeWaveError::ContainerInvalid);
            };
            info.data_size
        } else {
            u64::from(raw_size)
        };
        let payload_end = payload_offset
            .checked_add(payload_size)
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        let padded_end = payload_end
            .checked_add(payload_size & 1)
            .ok_or(NativeWaveError::ArithmeticOverflow)?;
        if padded_end > riff_end {
            return Err(NativeWaveError::ContainerInvalid);
        }
        if chunk_id == FMT {
            if format.is_some() {
                return Err(NativeWaveError::ContainerInvalid);
            }
            let size = usize::try_from(payload_size).map_err(|_| NativeWaveError::ResourceLimit)?;
            if size > 40 {
                return Err(NativeWaveError::FormatUnsupported);
            }
            let mut bytes = [0_u8; 40];
            reader
                .seek(SeekFrom::Start(payload_offset))
                .map_err(io_error)?;
            reader.read_exact(&mut bytes[..size]).map_err(io_error)?;
            format = Some(parse_format(&bytes[..size])?);
        } else if chunk_id == DATA {
            if data.is_some() {
                return Err(NativeWaveError::ContainerInvalid);
            }
            if container == NativeWaveContainer::Rf64 && ds64.is_none() {
                return Err(NativeWaveError::ContainerInvalid);
            }
            data = Some(DataChunk {
                offset: payload_offset,
                length: payload_size,
            });
        } else if chunk_id == DS64 {
            if container != NativeWaveContainer::Rf64 || ds64.is_some() || data.is_some() {
                return Err(NativeWaveError::ContainerInvalid);
            }
            ds64 = Some(parse_ds64(
                reader,
                payload_offset,
                payload_size,
                caps,
                &mut skipped_metadata_bytes,
            )?);
        } else {
            skipped_metadata_bytes = skipped_metadata_bytes
                .checked_add(payload_size)
                .ok_or(NativeWaveError::ArithmeticOverflow)?;
            if skipped_metadata_bytes > caps.max_skipped_metadata_bytes {
                return Err(NativeWaveError::ResourceLimit);
            }
        }
        cursor = padded_end;
    }
    if cursor != riff_end {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let format = format.ok_or(NativeWaveError::ContainerInvalid)?;
    let data = data.ok_or(NativeWaveError::ContainerInvalid)?;
    if container == NativeWaveContainer::Rf64 {
        let info = ds64.ok_or(NativeWaveError::ContainerInvalid)?;
        if info.riff_size != file_length.saturating_sub(8) || info.data_size != data.length {
            return Err(NativeWaveError::ContainerInvalid);
        }
    }
    if data.length % u64::from(format.block_align_bytes) != 0 {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let total_frames = data.length / u64::from(format.block_align_bytes);
    if let Some(info) = ds64
        && info.sample_count != 0
        && info.sample_count != total_frames
    {
        return Err(NativeWaveError::ContainerInvalid);
    }
    Ok(NativeWaveMetadata {
        container,
        sample_rate_hz: SampleRateHz(format.sample_rate_hz),
        channel_count: format.channel_count,
        encoding: format.encoding,
        block_align_bytes: format.block_align_bytes,
        byte_rate: format.byte_rate,
        data_offset_bytes: data.offset,
        data_length_bytes: data.length,
        total_frames,
    })
}

pub(crate) fn validate_region(
    metadata: NativeWaveMetadata,
    region: NativeWaveRegion,
) -> Result<(), NativeWaveError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if end > metadata.total_frames {
        return Err(NativeWaveError::RegionOutOfBounds);
    }
    Ok(())
}

pub(crate) fn validate_seek_frame(
    region: NativeWaveRegion,
    frame: SourceFrame,
) -> Result<(), NativeWaveError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if frame.0 < region.start_frame.0 || frame.0 > end {
        return Err(NativeWaveError::RegionOutOfBounds);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct FormatChunk {
    sample_rate_hz: u32,
    channel_count: u16,
    encoding: NativeWaveEncoding,
    block_align_bytes: u16,
    byte_rate: u32,
}

#[derive(Clone, Copy)]
struct DataChunk {
    offset: u64,
    length: u64,
}

#[derive(Clone, Copy)]
struct Ds64Chunk {
    riff_size: u64,
    data_size: u64,
    sample_count: u64,
}

fn parse_format(bytes: &[u8]) -> Result<FormatChunk, NativeWaveError> {
    if bytes.len() != 16 && bytes.len() != 40 {
        return Err(NativeWaveError::FormatUnsupported);
    }
    let tag = le_u16(&bytes[0..2]);
    let channel_count = le_u16(&bytes[2..4]);
    let sample_rate_hz = le_u32(&bytes[4..8]);
    let byte_rate = le_u32(&bytes[8..12]);
    let block_align_bytes = le_u16(&bytes[12..14]);
    let bits_per_sample = le_u16(&bytes[14..16]);
    if channel_count == 0 || sample_rate_hz == 0 {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let (encoding, required_size) = match tag {
        0x0001 => (pcm_encoding(bits_per_sample)?, 16),
        0x0003 => (float_encoding(bits_per_sample)?, 16),
        0xfffe => {
            if bytes.len() != 40
                || le_u16(&bytes[16..18]) != 22
                || le_u16(&bytes[18..20]) != bits_per_sample
            {
                return Err(NativeWaveError::FormatUnsupported);
            }
            let guid = &bytes[24..40];
            if guid == PCM_GUID {
                (pcm_encoding(bits_per_sample)?, 40)
            } else if guid == IEEE_FLOAT_GUID {
                (float_encoding(bits_per_sample)?, 40)
            } else {
                return Err(NativeWaveError::FormatUnsupported);
            }
        }
        _ => return Err(NativeWaveError::FormatUnsupported),
    };
    if bytes.len() != required_size {
        return Err(NativeWaveError::FormatUnsupported);
    }
    let expected_block_align = u32::from(channel_count)
        .checked_mul(u32::from(encoding.bytes_per_sample()))
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if expected_block_align != u32::from(block_align_bytes) {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let expected_byte_rate = u64::from(sample_rate_hz)
        .checked_mul(u64::from(block_align_bytes))
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if expected_byte_rate != u64::from(byte_rate) {
        return Err(NativeWaveError::ContainerInvalid);
    }
    Ok(FormatChunk {
        sample_rate_hz,
        channel_count,
        encoding,
        block_align_bytes,
        byte_rate,
    })
}

fn parse_ds64<R: Read + Seek>(
    reader: &mut R,
    offset: u64,
    size: u64,
    caps: NativeWaveParseCaps,
    skipped_metadata_bytes: &mut u64,
) -> Result<Ds64Chunk, NativeWaveError> {
    if size < 28 {
        return Err(NativeWaveError::ContainerInvalid);
    }
    reader.seek(SeekFrom::Start(offset)).map_err(io_error)?;
    let mut header = [0_u8; 28];
    reader.read_exact(&mut header).map_err(io_error)?;
    let table_length = u64::from(le_u32(&header[24..28]));
    let expected_size = 28_u64
        .checked_add(
            table_length
                .checked_mul(12)
                .ok_or(NativeWaveError::ArithmeticOverflow)?,
        )
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if expected_size != size {
        return Err(NativeWaveError::ContainerInvalid);
    }
    let table_bytes = size - 28;
    *skipped_metadata_bytes = skipped_metadata_bytes
        .checked_add(table_bytes)
        .ok_or(NativeWaveError::ArithmeticOverflow)?;
    if *skipped_metadata_bytes > caps.max_skipped_metadata_bytes {
        return Err(NativeWaveError::ResourceLimit);
    }
    Ok(Ds64Chunk {
        riff_size: le_u64(&header[0..8]),
        data_size: le_u64(&header[8..16]),
        sample_count: le_u64(&header[16..24]),
    })
}

fn pcm_encoding(bits_per_sample: u16) -> Result<NativeWaveEncoding, NativeWaveError> {
    match bits_per_sample {
        8 => Ok(NativeWaveEncoding::UnsignedPcm8),
        16 => Ok(NativeWaveEncoding::SignedPcm16),
        24 => Ok(NativeWaveEncoding::SignedPcm24),
        32 => Ok(NativeWaveEncoding::SignedPcm32),
        _ => Err(NativeWaveError::FormatUnsupported),
    }
}

fn float_encoding(bits_per_sample: u16) -> Result<NativeWaveEncoding, NativeWaveError> {
    match bits_per_sample {
        32 => Ok(NativeWaveEncoding::Float32),
        64 => Ok(NativeWaveEncoding::Float64),
        _ => Err(NativeWaveError::FormatUnsupported),
    }
}

fn convert_frames<S: PlanarSink + ?Sized>(
    metadata: NativeWaveMetadata,
    src: &[u8],
    block_align: usize,
    channels: usize,
    frame_count: usize,
    sink: &mut S,
    destination_start: usize,
) -> u64 {
    debug_assert_eq!(src.len(), frame_count * block_align);
    let mut sanitized = 0_u64;
    match metadata.encoding {
        NativeWaveEncoding::UnsignedPcm8 => {
            for channel in 0..channels {
                let sample_offset = channel;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    *output = (f32::from(frame[sample_offset]) - 128.0) * 0.007_812_5;
                }
            }
        }
        NativeWaveEncoding::SignedPcm16 => {
            for channel in 0..channels {
                let sample_offset = channel * 2;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    *output = f32::from(i16::from_le_bytes([
                        frame[sample_offset],
                        frame[sample_offset + 1],
                    ])) * (1.0 / 32_768.0);
                }
            }
        }
        NativeWaveEncoding::SignedPcm24 => {
            for channel in 0..channels {
                let sample_offset = channel * 3;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    let raw = i32::from(frame[sample_offset])
                        | (i32::from(frame[sample_offset + 1]) << 8)
                        | (i32::from(frame[sample_offset + 2]) << 16);
                    let signed = if raw & 0x0080_0000 != 0 {
                        raw | !0x00ff_ffff
                    } else {
                        raw
                    };
                    *output = signed as f32 * (1.0 / 8_388_608.0);
                }
            }
        }
        NativeWaveEncoding::SignedPcm32 => {
            for channel in 0..channels {
                let sample_offset = channel * 4;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    *output = i32::from_le_bytes([
                        frame[sample_offset],
                        frame[sample_offset + 1],
                        frame[sample_offset + 2],
                        frame[sample_offset + 3],
                    ]) as f32
                        * (1.0 / 2_147_483_648.0);
                }
            }
        }
        NativeWaveEncoding::Float32 => {
            for channel in 0..channels {
                let sample_offset = channel * 4;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    let (sample, count) = sanitize_f32(f32::from_le_bytes([
                        frame[sample_offset],
                        frame[sample_offset + 1],
                        frame[sample_offset + 2],
                        frame[sample_offset + 3],
                    ]));
                    sanitized = sanitized.saturating_add(count);
                    *output = sample;
                }
            }
        }
        NativeWaveEncoding::Float64 => {
            for channel in 0..channels {
                let sample_offset = channel * 8;
                let dst =
                    &mut sink.plane(channel)[destination_start..destination_start + frame_count];
                for (frame, output) in src.chunks_exact(block_align).zip(dst.iter_mut()) {
                    let (sample, count) = sanitize_f64(f64::from_le_bytes([
                        frame[sample_offset],
                        frame[sample_offset + 1],
                        frame[sample_offset + 2],
                        frame[sample_offset + 3],
                        frame[sample_offset + 4],
                        frame[sample_offset + 5],
                        frame[sample_offset + 6],
                        frame[sample_offset + 7],
                    ]));
                    sanitized = sanitized.saturating_add(count);
                    *output = sample;
                }
            }
        }
    }
    sanitized
}

#[inline(always)]
fn sanitize_f32(value: f32) -> (f32, u64) {
    const EXPONENT: u32 = 0x7f80_0000;
    const MAGNITUDE: u32 = 0x7fff_ffff;

    let bits = value.to_bits();
    let magnitude = bits & MAGNITUDE;
    let exponent = bits & EXPONENT;
    let accepted = ((exponent != 0) & (exponent != EXPONENT)) | (magnitude == 0);
    if accepted { (value, 0) } else { (0.0, 1) }
}

#[inline(always)]
fn sanitize_f64(value: f64) -> (f32, u64) {
    const EXPONENT: u64 = 0x7ff0_0000_0000_0000;
    const MAGNITUDE: u64 = 0x7fff_ffff_ffff_ffff;

    let bits = value.to_bits();
    let magnitude = bits & MAGNITUDE;
    let exponent = bits & EXPONENT;
    let accepted64 = ((exponent != 0) & (exponent != EXPONENT)) | (magnitude == 0);
    let (converted, rejected32) = sanitize_f32(value as f32);
    let accepted = accepted64 & (rejected32 == 0);
    if accepted { (converted, 0) } else { (0.0, 1) }
}

fn le_u16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn le_u64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

fn io_error(error: std::io::Error) -> NativeWaveError {
    NativeWaveError::Io(error.kind())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    const CAPS: NativeWaveParseCaps = NativeWaveParseCaps {
        max_chunk_count: 16,
        max_skipped_metadata_bytes: 128,
    };

    struct CountingReader {
        cursor: Cursor<Vec<u8>>,
        reads: usize,
        seeks: usize,
        fail_next_read: bool,
        fail_next_seek: bool,
    }

    impl CountingReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                cursor: Cursor::new(bytes),
                reads: 0,
                seeks: 0,
                fail_next_read: false,
                fail_next_seek: false,
            }
        }
    }

    impl Read for CountingReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.reads += 1;
            if core::mem::take(&mut self.fail_next_read) {
                return Err(io::Error::other("injected read failure"));
            }
            self.cursor.read(buffer)
        }
    }

    impl Seek for CountingReader {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.seeks += 1;
            if core::mem::take(&mut self.fail_next_seek) {
                return Err(io::Error::other("injected seek failure"));
            }
            self.cursor.seek(position)
        }
    }

    fn buffered_float_decoder(frame_count: usize) -> NativeWaveDecoder<CountingReader> {
        let data: Vec<_> = (0..frame_count)
            .flat_map(|frame| (frame as f32).to_le_bytes())
            .collect();
        let bytes = riff_wave(format_chunk(NativeWaveEncoding::Float32, false), &data, &[]);
        let mut metadata_cursor = Cursor::new(bytes.clone());
        let metadata = parse_native_wave(&mut metadata_cursor, CAPS).expect("metadata");
        NativeWaveDecoder::prepare(
            CountingReader::new(bytes),
            metadata,
            NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: u64::try_from(frame_count).expect("frame count"),
            },
            NonZeroUsize::new(16).expect("buffer frames"),
        )
        .expect("decoder")
    }

    #[test]
    fn buffered_quanta_seek_and_straddled_refill_preserve_exact_pcm() {
        let mut decoder = buffered_float_decoder(24);
        for quantum in 0..4 {
            let mut planar = [0.0; 4];
            decoder
                .decode_planar(&mut planar, 4)
                .expect("buffered quantum");
            assert_eq!(
                planar,
                core::array::from_fn(|frame| (quantum * 4 + frame) as f32)
            );
        }
        assert_eq!(decoder.reader.reads, 1, "four quanta share one fill");
        assert_eq!(decoder.reader.seeks, 1, "initial fill seeks once");

        decoder
            .seek_to_source_frame(SourceFrame(3))
            .expect("in-buffer seek");
        let mut in_buffer = [0.0; 4];
        decoder
            .decode_planar(&mut in_buffer, 4)
            .expect("in-buffer decode");
        assert_eq!(in_buffer, [3.0, 4.0, 5.0, 6.0]);
        assert_eq!(decoder.reader.reads, 1);
        assert_eq!(decoder.reader.seeks, 1);

        decoder
            .seek_to_source_frame(SourceFrame(14))
            .expect("nonaligned buffered seek");
        let mut straddled = [0.0; 4];
        decoder
            .decode_planar(&mut straddled, 4)
            .expect("straddled refill");
        assert_eq!(straddled, [14.0, 15.0, 16.0, 17.0]);
        assert_eq!(decoder.reader.reads, 2, "straddle performs one refill");
        assert_eq!(
            decoder.reader.seeks, 1,
            "contiguous refill retains reader position"
        );
    }

    #[test]
    fn failed_io_forgets_reader_position_and_retry_reseeks() {
        let mut decoder = buffered_float_decoder(24);
        decoder.reader.fail_next_seek = true;
        let mut first = [0.0; 4];
        assert_eq!(
            decoder.decode_planar(&mut first, 4),
            Err(NativeWaveError::Io(io::ErrorKind::Other))
        );
        assert_eq!(decoder.reader_position, None);
        decoder
            .decode_planar(&mut first, 4)
            .expect("retry after seek failure");
        assert_eq!(decoder.reader.seeks, 2);

        decoder
            .seek_to_source_frame(SourceFrame(18))
            .expect("outside buffer");
        decoder.reader.fail_next_read = true;
        let seeks_before_read_failure = decoder.reader.seeks;
        let mut second = [0.0; 4];
        assert_eq!(
            decoder.decode_planar(&mut second, 4),
            Err(NativeWaveError::Io(io::ErrorKind::Other))
        );
        assert_eq!(decoder.reader_position, None);
        decoder
            .decode_planar(&mut second, 4)
            .expect("retry after read failure");
        assert_eq!(decoder.reader.seeks, seeks_before_read_failure + 2);
        assert_eq!(second, [18.0, 19.0, 20.0, 21.0]);
    }

    #[test]
    fn plane_and_strided_sinks_match_every_encoding_across_chunks_refills_and_seek() {
        let cases = [
            (
                NativeWaveEncoding::UnsignedPcm8,
                vec![0, 64, 128, 192, 255, 1],
            ),
            (
                NativeWaveEncoding::SignedPcm16,
                [i16::MIN, -1, 0, 1, i16::MAX, 1234]
                    .into_iter()
                    .flat_map(i16::to_le_bytes)
                    .collect(),
            ),
            (
                NativeWaveEncoding::SignedPcm24,
                vec![
                    0x00, 0x00, 0x80, 0xff, 0xff, 0xff, 0, 0, 0, 1, 0, 0, 0xff, 0xff, 0x7f, 0x56,
                    0x34, 0x12,
                ],
            ),
            (
                NativeWaveEncoding::SignedPcm32,
                [i32::MIN, -1, 0, 1, i32::MAX, 123_456]
                    .into_iter()
                    .flat_map(i32::to_le_bytes)
                    .collect(),
            ),
            (
                NativeWaveEncoding::Float32,
                [0.0_f32, -0.0, 1.0, f32::from_bits(1), f32::INFINITY, -2.0]
                    .into_iter()
                    .flat_map(f32::to_le_bytes)
                    .collect(),
            ),
            (
                NativeWaveEncoding::Float64,
                [0.0_f64, -0.0, 1.0, f64::from_bits(1), f64::INFINITY, -2.0]
                    .into_iter()
                    .flat_map(f64::to_le_bytes)
                    .collect(),
            ),
        ];

        for (encoding, data) in cases {
            let bytes = riff_wave(format_chunk(encoding, false), &data, &[]);
            let mut metadata_cursor = Cursor::new(bytes.clone());
            let metadata = parse_native_wave(&mut metadata_cursor, CAPS).expect("metadata");
            let prepare = || {
                NativeWaveDecoder::prepare(
                    Cursor::new(bytes.clone()),
                    metadata,
                    NativeWaveRegion {
                        start_frame: SourceFrame(0),
                        length_frames: 6,
                    },
                    NonZeroUsize::new(4).expect("four-frame buffer"),
                )
                .expect("decoder")
            };
            let mut planes_decoder = prepare();
            let mut strided_decoder = prepare();

            for frames in [2, 3] {
                let mut plane = vec![0.0; frames];
                let plane_report = planes_decoder
                    .decode_into(&mut [&mut plane])
                    .expect("plane decode");
                let mut strided = vec![0.0; frames];
                let strided_report = strided_decoder
                    .decode_planar(&mut strided, frames)
                    .expect("strided decode");
                assert_eq!(
                    plane
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    strided
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    "sink bits differ for {encoding:?}"
                );
                assert_eq!(plane_report, strided_report);
                if frames == 2 {
                    planes_decoder
                        .seek_to_source_frame(SourceFrame(3))
                        .expect("plane in-buffer seek");
                    strided_decoder
                        .seek_to_source_frame(SourceFrame(3))
                        .expect("strided in-buffer seek");
                }
            }

            planes_decoder
                .seek_to_source_frame(SourceFrame(0))
                .expect("plane rewind");
            strided_decoder
                .seek_to_source_frame(SourceFrame(0))
                .expect("strided rewind");
            for frames in [1, 2, 3] {
                let mut plane = vec![0.0; frames];
                let plane_report = planes_decoder
                    .decode_into(&mut [&mut plane])
                    .expect("partitioned plane decode");
                let mut strided = vec![0.0; frames];
                let strided_report = strided_decoder
                    .decode_planar(&mut strided, frames)
                    .expect("partitioned strided decode");
                assert_eq!(
                    plane
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    strided
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                );
                assert_eq!(plane_report, strided_report);
            }
        }

        let source = include_str!("native_wave.rs");
        let conversion = &source[source.find("fn convert_frames").expect("conversion")
            ..source.find("fn sanitize_f32").expect("sanitizer")];
        assert_eq!(conversion.matches("match metadata.encoding").count(), 1);
        assert!(conversion.contains("sink.plane(channel)"));
        assert!(conversion.contains("src.chunks_exact(block_align).zip(dst.iter_mut())"));
        assert!(!conversion.contains(&["sink", ".write"].concat()));
        assert!(!source.contains(&["fn decode", "_sample"].concat()));

        let sanitizers = &source[source.find("fn sanitize_f32").expect("f32 sanitizer")
            ..source.find("fn le_u16").expect("post-sanitizer helper")];
        assert!(sanitizers.contains("(exponent != 0) & (exponent != EXPONENT)"));
        assert!(sanitizers.contains("accepted64 & (rejected32 == 0)"));
        assert!(!sanitizers.contains("return "));
    }

    #[test]
    fn mask_sanitizers_freeze_float_boundary_bits_and_counts() {
        let f32_cases = [
            (0x0000_0000, 0x0000_0000, 0),
            (0x8000_0000, 0x8000_0000, 0),
            (0x0000_0001, 0x0000_0000, 1),
            (0x007f_ffff, 0x0000_0000, 1),
            (0x8000_0001, 0x0000_0000, 1),
            (0x807f_ffff, 0x0000_0000, 1),
            (0x0080_0000, 0x0080_0000, 0),
            (0x7f7f_ffff, 0x7f7f_ffff, 0),
            (0x7f80_0000, 0x0000_0000, 1),
            (0xff80_0000, 0x0000_0000, 1),
            (0x7fc0_0001, 0x0000_0000, 1),
            (0xffc0_0001, 0x0000_0000, 1),
        ];
        for (input, output, count) in f32_cases {
            let (sanitized, actual_count) = sanitize_f32(f32::from_bits(input));
            assert_eq!(sanitized.to_bits(), output);
            assert_eq!(actual_count, count);
        }

        let f64_cases = [
            (0x0000_0000_0000_0000, 0x0000_0000, 0),
            (0x8000_0000_0000_0000, 0x8000_0000, 0),
            (0x0000_0000_0000_0001, 0x0000_0000, 1),
            (0x000f_ffff_ffff_ffff, 0x0000_0000, 1),
            (0x0010_0000_0000_0000, 0x0000_0000, 0),
            (0x8010_0000_0000_0000, 0x8000_0000, 0),
            (0x47ef_ffff_e000_0000, 0x7f7f_ffff, 0),
            (0x7fef_ffff_ffff_ffff, 0x0000_0000, 1),
            (0x7ff0_0000_0000_0000, 0x0000_0000, 1),
            (0xfff0_0000_0000_0000, 0x0000_0000, 1),
            (0x7ff8_0000_0000_0001, 0x0000_0000, 1),
            (0xfff8_0000_0000_0001, 0x0000_0000, 1),
        ];
        for (input, output, count) in f64_cases {
            let (sanitized, actual_count) = sanitize_f64(f64::from_bits(input));
            assert_eq!(sanitized.to_bits(), output, "f64 input {input:#018x}");
            assert_eq!(actual_count, count, "f64 input {input:#018x}");
        }
    }

    #[test]
    fn classic_formats_decode_with_exact_pcm_scaling_and_float_sanitation() {
        let cases: &[(NativeWaveEncoding, Vec<u8>, Vec<u32>, u64)] = &[
            (
                NativeWaveEncoding::UnsignedPcm8,
                vec![0, 128, 255],
                vec![(-1.0_f32).to_bits(), 0, 0.992_187_5_f32.to_bits()],
                0,
            ),
            (
                NativeWaveEncoding::SignedPcm16,
                [
                    i16::MIN.to_le_bytes(),
                    0_i16.to_le_bytes(),
                    i16::MAX.to_le_bytes(),
                ]
                .concat(),
                vec![(-1.0_f32).to_bits(), 0, (32_767.0_f32 / 32_768.0).to_bits()],
                0,
            ),
            (
                NativeWaveEncoding::SignedPcm24,
                vec![0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0x7f],
                vec![
                    (-1.0_f32).to_bits(),
                    0,
                    (8_388_607.0_f32 / 8_388_608.0).to_bits(),
                ],
                0,
            ),
            (
                NativeWaveEncoding::SignedPcm32,
                [
                    i32::MIN.to_le_bytes(),
                    0_i32.to_le_bytes(),
                    i32::MAX.to_le_bytes(),
                ]
                .concat(),
                vec![
                    (-1.0_f32).to_bits(),
                    0,
                    (2_147_483_647.0_f32 / 2_147_483_648.0).to_bits(),
                ],
                0,
            ),
            (
                NativeWaveEncoding::Float32,
                [
                    1.5_f32.to_le_bytes(),
                    (-0.0_f32).to_le_bytes(),
                    f32::INFINITY.to_le_bytes(),
                    f32::from_bits(1).to_le_bytes(),
                ]
                .concat(),
                vec![1.5_f32.to_bits(), (-0.0_f32).to_bits(), 0, 0],
                2,
            ),
            (
                NativeWaveEncoding::Float64,
                [
                    1.5_f64.to_le_bytes(),
                    (-0.0_f64).to_le_bytes(),
                    f64::INFINITY.to_le_bytes(),
                    f64::from_bits(1).to_le_bytes(),
                ]
                .concat(),
                vec![1.5_f32.to_bits(), (-0.0_f32).to_bits(), 0, 0],
                2,
            ),
        ];
        for (encoding, data, expected, sanitation) in cases {
            let bytes = riff_wave(format_chunk(*encoding, false), data, &[]);
            let mut cursor = Cursor::new(bytes);
            let metadata = parse_native_wave(&mut cursor, CAPS).expect("parse");
            assert_eq!(metadata.encoding, *encoding);
            let region = NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: u64::try_from(expected.len()).expect("len"),
            };
            let mut decoder = NativeWaveDecoder::prepare(
                cursor,
                metadata,
                region,
                NonZeroUsize::new(expected.len()).expect("nonzero"),
            )
            .expect("decoder");
            let mut output = vec![0.0_f32; expected.len()];
            let report = {
                let mut planes = [&mut output[..]];
                decoder.decode_into(&mut planes).expect("decode")
            };
            assert_eq!(
                output
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                *expected
            );
            assert_eq!(report.sanitized_sample_count, *sanitation);
            assert!(report.end_of_region);
        }
    }

    #[test]
    fn extensible_and_rf64_metadata_are_accepted_with_checked_sizes() {
        let extensible = riff_wave(
            format_chunk(NativeWaveEncoding::Float32, true),
            &1.0_f32.to_le_bytes(),
            &[(b"JUNK", vec![1, 2, 3])],
        );
        let mut cursor = Cursor::new(extensible);
        let metadata = parse_native_wave(&mut cursor, CAPS).expect("extensible");
        assert_eq!(metadata.container, NativeWaveContainer::Riff);
        assert_eq!(metadata.encoding, NativeWaveEncoding::Float32);
        let rf64 = rf64_wave(
            format_chunk(NativeWaveEncoding::SignedPcm16, false),
            &0_i16.to_le_bytes(),
        );
        let mut rf64_cursor = Cursor::new(rf64);
        let rf64_metadata = parse_native_wave(&mut rf64_cursor, CAPS).expect("rf64");
        assert_eq!(rf64_metadata.container, NativeWaveContainer::Rf64);
        assert_eq!(rf64_metadata.total_frames, 1);
    }

    #[test]
    fn prepared_quantum_decode_uses_contiguous_planar_worker_storage() {
        let bytes = riff_wave(
            format_chunk(NativeWaveEncoding::Float32, false),
            &[
                0.25_f32.to_le_bytes(),
                (-0.5_f32).to_le_bytes(),
                0.75_f32.to_le_bytes(),
                (-1.0_f32).to_le_bytes(),
            ]
            .concat(),
            &[],
        );
        let mut cursor = Cursor::new(bytes);
        let metadata = parse_native_wave(&mut cursor, CAPS).expect("parse");
        let mut decoder = NativeWaveDecoder::prepare(
            cursor,
            metadata,
            NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 4,
            },
            NonZeroUsize::new(4).expect("four"),
        )
        .expect("decoder");
        assert_eq!(decoder.next_source_frame(), SourceFrame(0));
        let mut planar = [0.0; 4];
        let report = decoder
            .decode_planar(&mut planar, 4)
            .expect("decode quantum");
        assert_eq!(report.decoded_frames, 4);
        assert_eq!(planar, [0.25, -0.5, 0.75, -1.0]);
        decoder
            .seek_to_source_frame(SourceFrame(2))
            .expect("prepared seek");
        assert_eq!(decoder.next_source_frame(), SourceFrame(2));
    }

    #[test]
    fn invalid_native_containers_and_regions_have_frozen_diagnostics() {
        let mut rifx = riff_wave(
            format_chunk(NativeWaveEncoding::UnsignedPcm8, false),
            &[128],
            &[],
        );
        rifx[..4].copy_from_slice(b"RIFX");
        let mut cursor = Cursor::new(rifx);
        let error = parse_native_wave(&mut cursor, CAPS).expect_err("rifx");
        assert_eq!(error, NativeWaveError::ContainerInvalid);
        assert_eq!(
            error.diagnostic_code(),
            SourceDiagnosticCode::ContainerInvalid
        );

        let mut compressed = Cursor::new(riff_wave(raw_format(6, 16, 1, 48_000), &[0, 0], &[]));
        let unsupported = parse_native_wave(&mut compressed, CAPS).expect_err("compressed");
        assert_eq!(unsupported, NativeWaveError::FormatUnsupported);
        assert_eq!(
            unsupported.diagnostic_code(),
            SourceDiagnosticCode::FormatUnsupported
        );

        let valid = riff_wave(
            format_chunk(NativeWaveEncoding::SignedPcm16, false),
            &[0, 0],
            &[],
        );
        let mut valid_cursor = Cursor::new(valid);
        let metadata = parse_native_wave(&mut valid_cursor, CAPS).expect("valid");
        let out_of_bounds = match NativeWaveDecoder::prepare(
            valid_cursor,
            metadata,
            NativeWaveRegion {
                start_frame: SourceFrame(1),
                length_frames: 1,
            },
            NonZeroUsize::new(1).expect("one"),
        ) {
            Ok(_) => panic!("region should reject"),
            Err(error) => error,
        };
        assert_eq!(out_of_bounds, NativeWaveError::RegionOutOfBounds);
        assert_eq!(
            out_of_bounds.diagnostic_code(),
            SourceDiagnosticCode::RegionOutOfBounds
        );
    }

    #[test]
    fn malformed_ds64_byte_rate_duplicate_data_and_metadata_cap_reject_without_payload_retention() {
        let mut malformed = rf64_wave(
            format_chunk(NativeWaveEncoding::SignedPcm16, false),
            &[0, 0],
        );
        let ds64_offset = 12 + 8;
        malformed[ds64_offset + 24..ds64_offset + 28].copy_from_slice(&1_u32.to_le_bytes());
        let mut cursor = Cursor::new(malformed);
        assert_eq!(
            parse_native_wave(&mut cursor, CAPS).expect_err("ds64"),
            NativeWaveError::ContainerInvalid
        );

        let mut bad_rate = Cursor::new(riff_wave(raw_format(1, 16, 1, 48_000), &[0, 0], &[]));
        let mut bytes = bad_rate.into_inner();
        bytes[12 + 8 + 8..12 + 8 + 12].copy_from_slice(&1_u32.to_le_bytes());
        bad_rate = Cursor::new(bytes);
        assert_eq!(
            parse_native_wave(&mut bad_rate, CAPS).expect_err("byte rate"),
            NativeWaveError::ContainerInvalid
        );

        let duplicate = riff_wave(
            format_chunk(NativeWaveEncoding::UnsignedPcm8, false),
            &[128],
            &[(b"data", vec![128])],
        );
        let mut duplicate_cursor = Cursor::new(duplicate);
        assert_eq!(
            parse_native_wave(&mut duplicate_cursor, CAPS).expect_err("duplicate data"),
            NativeWaveError::ContainerInvalid
        );

        let capped = riff_wave(
            format_chunk(NativeWaveEncoding::UnsignedPcm8, false),
            &[128],
            &[(b"JUNK", vec![0; 129])],
        );
        let mut capped_cursor = Cursor::new(capped);
        assert_eq!(
            parse_native_wave(&mut capped_cursor, CAPS).expect_err("metadata cap"),
            NativeWaveError::ResourceLimit
        );
    }

    fn raw_format(tag: u16, bits: u16, channels: u16, sample_rate: u32) -> Vec<u8> {
        let bytes_per_sample = bits / 8;
        let block_align = channels * bytes_per_sample;
        let byte_rate = sample_rate * u32::from(block_align);
        [
            tag.to_le_bytes().to_vec(),
            channels.to_le_bytes().to_vec(),
            sample_rate.to_le_bytes().to_vec(),
            byte_rate.to_le_bytes().to_vec(),
            block_align.to_le_bytes().to_vec(),
            bits.to_le_bytes().to_vec(),
        ]
        .concat()
    }

    fn format_chunk(encoding: NativeWaveEncoding, extensible: bool) -> Vec<u8> {
        let (tag, bits) = match encoding {
            NativeWaveEncoding::UnsignedPcm8 => (1, 8),
            NativeWaveEncoding::SignedPcm16 => (1, 16),
            NativeWaveEncoding::SignedPcm24 => (1, 24),
            NativeWaveEncoding::SignedPcm32 => (1, 32),
            NativeWaveEncoding::Float32 => (3, 32),
            NativeWaveEncoding::Float64 => (3, 64),
        };
        if !extensible {
            return raw_format(tag, bits, 1, 48_000);
        }
        let mut bytes = raw_format(0xfffe, bits, 1, 48_000);
        bytes.extend_from_slice(&22_u16.to_le_bytes());
        bytes.extend_from_slice(&bits.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(if tag == 1 {
            &PCM_GUID
        } else {
            &IEEE_FLOAT_GUID
        });
        bytes
    }

    fn append_chunk(out: &mut Vec<u8>, id: &[u8; 4], data: &[u8]) {
        out.extend_from_slice(id);
        out.extend_from_slice(&u32::try_from(data.len()).expect("test chunk").to_le_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(0);
        }
    }

    fn riff_wave(format: Vec<u8>, data: &[u8], extra: &[(&[u8; 4], Vec<u8>)]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&0_u32.to_le_bytes());
        out.extend_from_slice(b"WAVE");
        append_chunk(&mut out, b"fmt ", &format);
        for (id, payload) in extra {
            append_chunk(&mut out, id, payload);
        }
        append_chunk(&mut out, b"data", data);
        let size = u32::try_from(out.len() - 8).expect("test riff length");
        out[4..8].copy_from_slice(&size.to_le_bytes());
        out
    }

    fn rf64_wave(format: Vec<u8>, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"RF64");
        out.extend_from_slice(&u32::MAX.to_le_bytes());
        out.extend_from_slice(b"WAVE");
        out.extend_from_slice(b"ds64");
        out.extend_from_slice(&28_u32.to_le_bytes());
        let ds64_offset = out.len();
        out.extend_from_slice(&[0; 28]);
        append_chunk(&mut out, b"fmt ", &format);
        out.extend_from_slice(b"data");
        out.extend_from_slice(&u32::MAX.to_le_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(0);
        }
        let riff_size = u64::try_from(out.len() - 8).expect("test rf64 length");
        let data_size = u64::try_from(data.len()).expect("test data length");
        let sample_count = data_size / 2;
        out[ds64_offset..ds64_offset + 8].copy_from_slice(&riff_size.to_le_bytes());
        out[ds64_offset + 8..ds64_offset + 16].copy_from_slice(&data_size.to_le_bytes());
        out[ds64_offset + 16..ds64_offset + 24].copy_from_slice(&sample_count.to_le_bytes());
        out
    }
}
