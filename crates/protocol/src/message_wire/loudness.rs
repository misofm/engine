//! `LOUDNESS` (`0x0014`, #2115): one operation on the session's output loudness meter, and the
//! reply that carries the meter's state after it.
//!
//! The codec owns the operation table and the reply's value rules: an operation outside the table
//! is `INVALID_FIELD`; every level is finite and at or above [`LOUDNESS_LEVEL_FLOOR`] (a loudness
//! range is finite and at least 0), and the true peak since reset is present exactly when
//! `integrating_since` is. A reply that breaks a rule is `INVALID_FIELD` on encode and on decode,
//! so no encoder writes a frame the decoder refuses.

use super::{
    Message, checked_sink_len, checked_writer_len, read_bool, read_f32, read_u8, read_u64,
};
use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{CountSink, Sink, SliceSink},
    schema::{self, loudness_reply, loudness_request},
};

/// The lowest level a `LOUDNESS` reply carries, in LUFS or dBTP: silence and a zero true peak
/// read this, the spectrum's finite-floor rule.
pub const LOUDNESS_LEVEL_FLOOR: f32 = -120.0;

/// One `LOUDNESS` operation (#2115).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoudnessRequest {
    /// Restart the analysis and arm the meter from the session's render position. Refused while
    /// the meter is armed.
    Start,
    /// Drain the delivered hops and answer the readings.
    Read,
    /// Restart integration (I, LRA, the maxima and the true peak since reset) at the latest
    /// render sample; M and S keep running.
    Reset,
    /// Disarm the meter; the readings stay readable.
    Stop,
}

impl LoudnessRequest {
    const fn op(self) -> u8 {
        match self {
            Self::Start => 1,
            Self::Read => 2,
            Self::Reset => 3,
            Self::Stop => 4,
        }
    }

    const fn from_op(op: u8) -> Result<Self, DecodeError> {
        match op {
            1 => Ok(Self::Start),
            2 => Ok(Self::Read),
            3 => Ok(Self::Reset),
            4 => Ok(Self::Stop),
            _ => Err(DecodeError::InvalidField),
        }
    }
}

/// The success reply of a `LOUDNESS`: the meter's state after the operation. Each level is
/// `None` until it is defined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoudnessReply {
    /// The end of the last delivered hop of this arming, or the sample the meter was armed from
    /// before its first hop, or 0 before any start.
    pub observed_sample: u64,
    /// Whether the meter is armed.
    pub armed: bool,
    /// The first sample integration counts from: the first hop boundary at or after the last
    /// start or reset that a delivered hop reached.
    pub integrating_since: Option<u64>,
    /// Hops render dropped because the record queue was full: monotone over the session's life.
    pub hops_dropped: u64,
    /// Non-finite output samples read as zero in kept hops since reset.
    pub sanitized_samples: u64,
    /// Gating blocks that pass both BS.1770 gates since reset.
    pub gated_block_count: u64,
    /// Momentary loudness (400 ms), LUFS.
    pub momentary_lufs: Option<f32>,
    /// Short-term loudness (3 s), LUFS.
    pub short_term_lufs: Option<f32>,
    /// Integrated (gated) loudness since reset, LUFS.
    pub integrated_lufs: Option<f32>,
    /// Loudness range since reset (EBU Tech 3342), LU.
    pub loudness_range_lu: Option<f32>,
    /// The largest momentary loudness since reset, LUFS.
    pub momentary_max_lufs: Option<f32>,
    /// The largest short-term loudness since reset, LUFS.
    pub short_term_max_lufs: Option<f32>,
    /// The true peak since reset, `[left, right]`, dBTP.
    pub true_peak_dbtp: Option<[f32; 2]>,
    /// The true peak of the last delivered hop, `[left, right]`, dBTP.
    pub hop_true_peak_dbtp: Option<[f32; 2]>,
}

fn level_is_valid(value: Option<f32>) -> bool {
    value.is_none_or(|level| level.is_finite() && level >= LOUDNESS_LEVEL_FLOOR)
}

fn pair_is_valid(value: Option<[f32; 2]>) -> bool {
    value.is_none_or(|[left, right]| level_is_valid(Some(left)) && level_is_valid(Some(right)))
}

fn reply_is_valid(value: &LoudnessReply) -> bool {
    [
        value.momentary_lufs,
        value.short_term_lufs,
        value.integrated_lufs,
        value.momentary_max_lufs,
        value.short_term_max_lufs,
    ]
    .into_iter()
    .all(level_is_valid)
        && value
            .loudness_range_lu
            .is_none_or(|range| range.is_finite() && range >= 0.0)
        && pair_is_valid(value.true_peak_dbtp)
        && pair_is_valid(value.hop_true_peak_dbtp)
        && value.true_peak_dbtp.is_some() == value.integrating_since.is_some()
}

pub(crate) fn write_loudness_request(
    sink: &mut dyn Sink,
    value: LoudnessRequest,
) -> Result<(), EncodeError> {
    use loudness_request as r;
    sink.check_field_count(r::SPEC.field_count(&[])?)?;
    write_spec!(sink, r::OP, &[value.op()])
}

fn decode_loudness_request(message: Message<'_>) -> Result<LoudnessRequest, DecodeError> {
    use loudness_request as r;
    let message = message.schema_spec(&r::SPEC)?;
    LoudnessRequest::from_op(read_u8(one_spec!(message, r::OP)?)?)
}

/// The reply's levels, each with its field, in tag order.
fn levels(value: &LoudnessReply) -> [Option<f32>; 10] {
    let left = |pair: Option<[f32; 2]>| pair.map(|[left, _]| left);
    let right = |pair: Option<[f32; 2]>| pair.map(|[_, right]| right);
    [
        value.momentary_lufs,
        value.short_term_lufs,
        value.integrated_lufs,
        value.loudness_range_lu,
        value.momentary_max_lufs,
        value.short_term_max_lufs,
        left(value.true_peak_dbtp),
        right(value.true_peak_dbtp),
        left(value.hop_true_peak_dbtp),
        right(value.hop_true_peak_dbtp),
    ]
}

pub(crate) fn write_loudness_reply(
    sink: &mut dyn Sink,
    value: &LoudnessReply,
) -> Result<(), EncodeError> {
    use loudness_reply as r;
    if !reply_is_valid(value) {
        return Err(EncodeError::InvalidField);
    }
    let levels = levels(value);
    let mut variable = [(
        r::INTEGRATING_SINCE,
        usize::from(value.integrating_since.is_some()),
    ); 11];
    for ((slot, spec), level) in variable[1..].iter_mut().zip(r::LEVELS).zip(levels) {
        *slot = (spec, usize::from(level.is_some()));
    }
    sink.check_field_count(r::SPEC.field_count(&variable)?)?;
    write_spec!(
        sink,
        r::OBSERVED_SAMPLE,
        &value.observed_sample.to_le_bytes()
    )?;
    write_spec!(sink, r::ARMED, &[u8::from(value.armed)])?;
    if let Some(since) = value.integrating_since {
        write_spec!(sink, r::INTEGRATING_SINCE, &since.to_le_bytes())?;
    }
    write_spec!(sink, r::HOPS_DROPPED, &value.hops_dropped.to_le_bytes())?;
    write_spec!(
        sink,
        r::SANITIZED_SAMPLES,
        &value.sanitized_samples.to_le_bytes()
    )?;
    write_spec!(
        sink,
        r::GATED_BLOCK_COUNT,
        &value.gated_block_count.to_le_bytes()
    )?;
    for (spec, level) in r::LEVELS.into_iter().zip(levels) {
        if let Some(level) = level {
            write_spec!(sink, spec, &level.to_le_bytes())?;
        }
    }
    Ok(())
}

fn decode_loudness_reply(message: Message<'_>) -> Result<LoudnessReply, DecodeError> {
    use loudness_reply as r;
    let message = message.schema_spec(&r::SPEC)?;
    let word = |spec: schema::FieldSpec| read_u64(one_spec!(message, spec)?);
    let level = |spec: schema::FieldSpec| optional_spec!(message, spec)?.map(read_f32).transpose();
    let pair = |left: schema::FieldSpec,
                right: schema::FieldSpec|
     -> Result<Option<[f32; 2]>, DecodeError> {
        match (level(left)?, level(right)?) {
            (Some(left), Some(right)) => Ok(Some([left, right])),
            (None, None) => Ok(None),
            _ => Err(DecodeError::InvalidField),
        }
    };
    let value = LoudnessReply {
        observed_sample: word(r::OBSERVED_SAMPLE)?,
        armed: read_bool(one_spec!(message, r::ARMED)?)?,
        integrating_since: optional_spec!(message, r::INTEGRATING_SINCE)?
            .map(read_u64)
            .transpose()?,
        hops_dropped: word(r::HOPS_DROPPED)?,
        sanitized_samples: word(r::SANITIZED_SAMPLES)?,
        gated_block_count: word(r::GATED_BLOCK_COUNT)?,
        momentary_lufs: level(r::MOMENTARY_LUFS)?,
        short_term_lufs: level(r::SHORT_TERM_LUFS)?,
        integrated_lufs: level(r::INTEGRATED_LUFS)?,
        loudness_range_lu: level(r::LOUDNESS_RANGE_LU)?,
        momentary_max_lufs: level(r::MOMENTARY_MAX_LUFS)?,
        short_term_max_lufs: level(r::SHORT_TERM_MAX_LUFS)?,
        true_peak_dbtp: pair(r::TRUE_PEAK_LEFT_DBTP, r::TRUE_PEAK_RIGHT_DBTP)?,
        hop_true_peak_dbtp: pair(r::HOP_TRUE_PEAK_LEFT_DBTP, r::HOP_TRUE_PEAK_RIGHT_DBTP)?,
    };
    if !reply_is_valid(&value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

impl ProtocolCodec {
    /// Return the exact caller-buffer size of a `LOUDNESS` reply.
    pub fn encoded_loudness_reply_len(&self, value: &LoudnessReply) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_loudness_reply(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `LOUDNESS` reply without allocation.
    pub fn encode_loudness_reply(
        &self,
        value: &LoudnessReply,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_loudness_reply_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_loudness_reply(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `LOUDNESS` request.
    pub fn decode_loudness_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<LoudnessRequest, DecodeError> {
        decode_loudness_request(Message::top_level(payload, count, self.limits())?)
    }

    /// Strictly decode a `LOUDNESS` reply.
    pub fn decode_loudness_reply(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<LoudnessReply, DecodeError> {
        decode_loudness_reply(Message::top_level(payload, count, self.limits())?)
    }
}
