//! Strict bounded `.mepcm` v1 parser and local CRC-32C implementation.

use miso_engine_core::{
    SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate,
};

const HEADER_LEN: usize = 48;
const MAGIC: &[u8; 8] = b"MISOEPCM";

/// Limits applied before fixture allocation/decode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureLimits {
    /// Maximum accepted frames.
    pub max_frames: u64,
    /// Maximum accepted channels.
    pub max_channels: u16,
    /// Maximum payload bytes.
    pub max_payload_bytes: u64,
}
impl Default for FixtureLimits {
    fn default() -> Self {
        Self {
            max_frames: 1_000_000,
            max_channels: 64,
            max_payload_bytes: 1_048_576,
        }
    }
}

/// Fixture decoding errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixtureError {
    /// Bytes are too short for a header.
    TruncatedHeader,
    /// Magic/version/header length do not match v1.
    InvalidHeader,
    /// Flags, encoding, reserved bytes, or supported rate are invalid.
    InvalidField,
    /// A declared size exceeded limits or overflowed.
    LimitsExceeded,
    /// Exact payload length does not match the declaration.
    LengthMismatch,
    /// CRC-32C did not match.
    CrcMismatch,
}

/// Validated PCM fixture preserving raw `f32` bit patterns.
#[derive(Clone, Debug, PartialEq)]
pub struct PcmFixture {
    rate: SampleRateHz,
    channels: u16,
    frames: u64,
    samples: Vec<f32>,
    checksum: u32,
}
impl PcmFixture {
    /// Parses exactly one v1 fixture with no trailing data.
    pub fn parse(bytes: &[u8], limits: FixtureLimits) -> Result<Self, FixtureError> {
        if bytes.len() < HEADER_LEN {
            return Err(FixtureError::TruncatedHeader);
        }
        if &bytes[..8] != MAGIC
            || read_u16(bytes, 8) != 1
            || read_u16(bytes, 10) != HEADER_LEN as u16
        {
            return Err(FixtureError::InvalidHeader);
        }
        if read_u32(bytes, 12) != 0 || read_u16(bytes, 22) != 1 || read_u32(bytes, 44) != 0 {
            return Err(FixtureError::InvalidField);
        }
        let rate = SampleRateHz(read_u32(bytes, 16));
        if !(is_launch_sample_rate(rate) || is_extended_compatibility_sample_rate(rate)) {
            return Err(FixtureError::InvalidField);
        }
        let channels = read_u16(bytes, 20);
        let frames = read_u64(bytes, 24);
        let payload_len = read_u64(bytes, 32);
        if channels == 0
            || channels > limits.max_channels
            || frames == 0
            || frames > limits.max_frames
            || payload_len > limits.max_payload_bytes
        {
            return Err(FixtureError::LimitsExceeded);
        }
        let expected = u64::from(channels)
            .checked_mul(frames)
            .and_then(|n| n.checked_mul(4))
            .ok_or(FixtureError::LimitsExceeded)?;
        if expected != payload_len
            || HEADER_LEN
                .checked_add(
                    usize::try_from(payload_len).map_err(|_| FixtureError::LimitsExceeded)?,
                )
                .ok_or(FixtureError::LimitsExceeded)?
                != bytes.len()
        {
            return Err(FixtureError::LengthMismatch);
        }
        let expected_crc = read_u32(bytes, 40);
        let mut crc_bytes = bytes.to_vec();
        crc_bytes[40..44].fill(0);
        if crc32c(&crc_bytes) != expected_crc {
            return Err(FixtureError::CrcMismatch);
        }
        let samples = bytes[HEADER_LEN..]
            .chunks_exact(4)
            .map(|word| f32::from_bits(u32::from_le_bytes(word.try_into().expect("exact chunk"))))
            .collect();
        Ok(Self {
            rate,
            channels,
            frames,
            samples,
            checksum: expected_crc,
        })
    }
    /// Encodes this fixture using the v1 fixed header.
    pub fn encode(
        rate: SampleRateHz,
        channels: u16,
        frames: u64,
        samples: &[f32],
    ) -> Result<Vec<u8>, FixtureError> {
        if !(is_launch_sample_rate(rate) || is_extended_compatibility_sample_rate(rate)) {
            return Err(FixtureError::InvalidField);
        }
        let payload_len = u64::from(channels)
            .checked_mul(frames)
            .and_then(|n| n.checked_mul(4))
            .ok_or(FixtureError::LimitsExceeded)?;
        let sample_payload_len = samples
            .len()
            .checked_mul(4)
            .ok_or(FixtureError::LimitsExceeded)?;
        if channels == 0
            || frames == 0
            || usize::try_from(payload_len).ok() != Some(sample_payload_len)
        {
            return Err(FixtureError::LengthMismatch);
        }
        let encoded_len = HEADER_LEN
            .checked_add(sample_payload_len)
            .ok_or(FixtureError::LimitsExceeded)?;
        let mut bytes = Vec::with_capacity(encoded_len);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&(HEADER_LEN as u16).to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&rate.0.to_le_bytes());
        bytes.extend_from_slice(&channels.to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&frames.to_le_bytes());
        bytes.extend_from_slice(&payload_len.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        for sample in samples {
            bytes.extend_from_slice(&sample.to_bits().to_le_bytes());
        }
        let crc = crc32c(&bytes);
        bytes[40..44].copy_from_slice(&crc.to_le_bytes());
        Ok(bytes)
    }
    /// Rate.
    pub const fn rate(&self) -> SampleRateHz {
        self.rate
    }
    /// Channels.
    pub const fn channels(&self) -> u16 {
        self.channels
    }
    /// Frames.
    pub const fn frames(&self) -> u64 {
        self.frames
    }
    /// Channel-major raw samples.
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
    /// Canonical CRC-32C stored in the fixture header.
    pub const fn checksum(&self) -> u32 {
        self.checksum
    }
}

/// Computes reflected CRC-32C (Castagnoli) using init/final XOR `0xffffffff`.
pub fn crc32c(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0x82F6_3B78
            } else {
                crc >> 1
            };
        }
    }
    !crc
}
fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("header bounds"))
}
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("header bounds"))
}
fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("header bounds"))
}
