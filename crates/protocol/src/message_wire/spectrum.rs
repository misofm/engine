//! `SPECTRUM_CONFIGURE` (`0x0010`, decision 17, C9 and C13; #1594): the request that sets, changes
//! or removes the spectrum taps of a running session, and the reply that names the configuration
//! it set and the plan that carries it. `SPECTRUM_CAPTURE` (`0x0011`, #1595 D2): one operation on
//! the one-shot window, and the reply that carries the analyzed window when it is ready.
//!
//! The codec owns the code tables and the form's entry count: a value outside a table, an identity
//! that is empty or longer than [`SPECTRUM_TARGET_ID_MAXIMUM_BYTES`], a hop outside
//! [`SPECTRUM_HOP_FRAMES`] and a single form that does not hold exactly one entry are
//! `INVALID_FIELD` on encode and on decode. The budget rules (#1591 D2) are the core's: the
//! session refuses them with `INVALID_FIELD` and the `spectrum.budget` code. Both directions apply
//! the same rules, so no encoder writes a frame the decoder refuses.

use std::borrow::Cow;

use super::{
    Message, ParameterChannel, checked_sink_len, checked_writer_len, read_bool, read_f32,
    read_string_within, read_u8, read_u32, read_u64, text_fits,
};
use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{CountSink, Sink, SliceSink},
    schema::{
        self, spectrum_capture_reply, spectrum_capture_request, spectrum_configure_request,
        spectrum_configured, spectrum_entry,
    },
};

control_code_table! {
    /// Whether a configuration holds one fixed tap or a collection of taps.
    #[allow(missing_docs)]
    SpectrumForm {
        Single = 1,
        Collection = 2,
    }
}

control_code_table! {
    /// Which graph boundary a spectrum entry observes.
    SpectrumTargetKind {
        /// A track's (or a submix's) strip immediately after its input section.
        TrackPostInput = 1,
        /// A track's (or a submix's) strip immediately after its fader and pan stage.
        TrackPostPan = 2,
        /// A session output.
        Output = 3,
    }
}

/// The longest target identity a spectrum entry carries, in bytes.
pub const SPECTRUM_TARGET_ID_MAXIMUM_BYTES: usize = 127;

/// The explicit stream hops a configuration may name, in frames, ascending.
pub const SPECTRUM_HOP_FRAMES: [u32; 4] = [256, 512, 1024, 2048];

/// One tap of a spectrum configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpectrumEntry {
    /// The boundary it observes.
    pub target_kind: SpectrumTargetKind,
    /// The strip or output it observes, by stable ID.
    pub target_id: String,
    /// The planes it captures: left 1, right 2, both 3.
    pub channels: ParameterChannel,
}

/// A `SPECTRUM_CONFIGURE` request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpectrumConfigureRequest {
    /// The form: a single form holds exactly one entry, a collection zero or more.
    pub form: SpectrumForm,
    /// The taps, in order.
    pub entries: Vec<SpectrumEntry>,
    /// The bytes the prepared captures may retain: per tap for a single form, in aggregate for a
    /// collection. An empty collection with budget 0 removes every tap.
    pub maximum_capture_bytes: u64,
    /// The explicit stream hop in frames, one of [`SPECTRUM_HOP_FRAMES`]; `None` for the session
    /// shape's default cadence.
    pub hop_frames: Option<u32>,
}

/// The success reply of a `SPECTRUM_CONFIGURE`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpectrumConfigured {
    /// The committed session revision, which the command never changes.
    pub revision: u64,
    /// The epoch of the configuration now set.
    pub configured_epoch: u64,
    /// The epoch of the configuration the running plan carries.
    pub adopted_epoch: u64,
    /// Whether the command prepared a replacement plan.
    pub rebuild: bool,
    /// The epoch of that plan: present exactly when `rebuild` is true.
    pub plan_epoch: Option<u64>,
}

fn entry_is_valid(limits: crate::ProtocolLimits, entry: &SpectrumEntry) -> bool {
    (1..=SPECTRUM_TARGET_ID_MAXIMUM_BYTES).contains(&entry.target_id.len())
        && text_fits(limits, &entry.target_id)
}

fn request_is_valid(limits: crate::ProtocolLimits, value: &SpectrumConfigureRequest) -> bool {
    (value.form != SpectrumForm::Single || value.entries.len() == 1)
        && value
            .entries
            .iter()
            .all(|entry| entry_is_valid(limits, entry))
        && value
            .hop_frames
            .is_none_or(|hop| SPECTRUM_HOP_FRAMES.contains(&hop))
}

const fn reply_is_valid(value: &SpectrumConfigured) -> bool {
    value.rebuild == value.plan_epoch.is_some()
}

pub(crate) fn write_spectrum_configure_request(
    sink: &mut dyn Sink,
    value: &SpectrumConfigureRequest,
) -> Result<(), EncodeError> {
    if !request_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let count = spectrum_configure_request::SPEC.field_count(&[
        (spectrum_configure_request::ENTRY, value.entries.len()),
        (
            spectrum_configure_request::HOP_FRAMES,
            usize::from(value.hop_frames.is_some()),
        ),
    ])?;
    sink.check_field_count(count)?;
    write_spec!(sink, spectrum_configure_request::FORM, &[value.form.code()])?;
    for entry in &value.entries {
        sink.nested_spec(spectrum_configure_request::ENTRY, &mut |sink| {
            sink.message_header(spectrum_entry::SPEC.field_count(&[])?)?;
            write_spec!(
                sink,
                spectrum_entry::TARGET_KIND,
                &[entry.target_kind.code()]
            )?;
            write_spec!(sink, spectrum_entry::TARGET_ID, entry.target_id.as_bytes())?;
            write_spec!(
                sink,
                spectrum_entry::CHANNELS,
                &[schema::parameter_channel_wire(entry.channels)]
            )
        })?;
    }
    write_spec!(
        sink,
        spectrum_configure_request::MAXIMUM_CAPTURE_BYTES,
        &value.maximum_capture_bytes.to_le_bytes()
    )?;
    if let Some(hop) = value.hop_frames {
        write_spec!(
            sink,
            spectrum_configure_request::HOP_FRAMES,
            &hop.to_le_bytes()
        )?;
    }
    Ok(())
}

fn decode_spectrum_configure_request(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<SpectrumConfigureRequest, DecodeError> {
    let limits = codec.limits();
    let message = message.schema_spec(&spectrum_configure_request::SPEC)?;
    let form = SpectrumForm::from_code(read_u8(one_spec!(
        message,
        spectrum_configure_request::FORM
    )?)?)?;
    let entries = values_spec!(message, spectrum_configure_request::ENTRY)?
        .map(|entry| {
            let entry =
                Message::nested_at_depth(entry, limits, 1)?.schema_spec(&spectrum_entry::SPEC)?;
            Ok(SpectrumEntry {
                target_kind: SpectrumTargetKind::from_code(read_u8(one_spec!(
                    entry,
                    spectrum_entry::TARGET_KIND
                )?)?)?,
                target_id: read_string_within(
                    limits,
                    one_spec!(entry, spectrum_entry::TARGET_ID)?,
                )?
                .to_owned(),
                channels: schema::parameter_channel_from_wire(read_u8(one_spec!(
                    entry,
                    spectrum_entry::CHANNELS
                )?)?)
                .map_err(|_| DecodeError::InvalidField)?,
            })
        })
        .collect::<Result<Vec<_>, DecodeError>>()?;
    let value = SpectrumConfigureRequest {
        form,
        entries,
        maximum_capture_bytes: read_u64(one_spec!(
            message,
            spectrum_configure_request::MAXIMUM_CAPTURE_BYTES
        )?)?,
        hop_frames: optional_spec!(message, spectrum_configure_request::HOP_FRAMES)?
            .map(read_u32)
            .transpose()?,
    };
    if !request_is_valid(limits, &value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

pub(crate) fn write_spectrum_configured(
    sink: &mut dyn Sink,
    value: &SpectrumConfigured,
) -> Result<(), EncodeError> {
    if !reply_is_valid(value) {
        return Err(EncodeError::InvalidField);
    }
    let count = spectrum_configured::SPEC.field_count(&[(
        spectrum_configured::PLAN_EPOCH,
        usize::from(value.plan_epoch.is_some()),
    )])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        spectrum_configured::REVISION,
        &value.revision.to_le_bytes()
    )?;
    write_spec!(
        sink,
        spectrum_configured::CONFIGURED_EPOCH,
        &value.configured_epoch.to_le_bytes()
    )?;
    write_spec!(
        sink,
        spectrum_configured::ADOPTED_EPOCH,
        &value.adopted_epoch.to_le_bytes()
    )?;
    write_spec!(
        sink,
        spectrum_configured::REBUILD,
        &[u8::from(value.rebuild)]
    )?;
    if let Some(epoch) = value.plan_epoch {
        write_spec!(sink, spectrum_configured::PLAN_EPOCH, &epoch.to_le_bytes())?;
    }
    Ok(())
}

fn decode_spectrum_configured(message: Message<'_>) -> Result<SpectrumConfigured, DecodeError> {
    let message = message.schema_spec(&spectrum_configured::SPEC)?;
    let value = SpectrumConfigured {
        revision: read_u64(one_spec!(message, spectrum_configured::REVISION)?)?,
        configured_epoch: read_u64(one_spec!(message, spectrum_configured::CONFIGURED_EPOCH)?)?,
        adopted_epoch: read_u64(one_spec!(message, spectrum_configured::ADOPTED_EPOCH)?)?,
        rebuild: read_bool(one_spec!(message, spectrum_configured::REBUILD)?)?,
        plan_epoch: optional_spec!(message, spectrum_configured::PLAN_EPOCH)?
            .map(read_u64)
            .transpose()?,
    };
    if !reply_is_valid(&value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

impl ProtocolCodec {
    /// Return the exact caller-buffer size of a `SPECTRUM_CONFIGURE` request.
    pub fn encoded_spectrum_configure_request_len(
        &self,
        value: &SpectrumConfigureRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_spectrum_configure_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `SPECTRUM_CONFIGURE` request without allocation.
    pub fn encode_spectrum_configure_request(
        &self,
        value: &SpectrumConfigureRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_spectrum_configure_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_spectrum_configure_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `SPECTRUM_CONFIGURE` request.
    pub fn decode_spectrum_configure_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SpectrumConfigureRequest, DecodeError> {
        decode_spectrum_configure_request(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Return the exact caller-buffer size of a `SPECTRUM_CONFIGURE` reply.
    pub fn encoded_spectrum_configured_len(
        &self,
        value: &SpectrumConfigured,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_spectrum_configured(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `SPECTRUM_CONFIGURE` reply without allocation.
    pub fn encode_spectrum_configured(
        &self,
        value: &SpectrumConfigured,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_spectrum_configured_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_spectrum_configured(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `SPECTRUM_CONFIGURE` reply.
    pub fn decode_spectrum_configured(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SpectrumConfigured, DecodeError> {
        decode_spectrum_configured(Message::top_level(payload, count, self.limits())?)
    }
}

/// The frames of one spectrum window (#1595 D2).
pub const SPECTRUM_CAPTURE_WINDOW_FRAMES: u32 = 2_048;

/// The bins of one analyzed window, DC through Nyquist (#1595 D2).
pub const SPECTRUM_CAPTURE_BIN_COUNT: u32 = SPECTRUM_CAPTURE_WINDOW_FRAMES / 2 + 1;

/// One `SPECTRUM_CAPTURE` operation (#1595 D2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpectrumCaptureRequest {
    /// Arm the selected capture for its next complete window.
    Arm,
    /// Take the completed window, if there is one, analyzed into dB bins.
    Read {
        /// The planes to analyze: a subset of the selected entry's.
        channels: ParameterChannel,
        /// Whether the reply also carries the raw window planes.
        include_window: bool,
    },
    /// Cancel a pending capture and discard a completed window.
    Cancel,
    /// Select one exact entry of a collection.
    Select {
        /// The boundary the entry observes.
        target_kind: SpectrumTargetKind,
        /// The strip or output it observes, by stable ID.
        target_id: String,
        /// The entry's planes.
        channels: ParameterChannel,
    },
}

impl SpectrumCaptureRequest {
    const fn op(&self) -> u8 {
        match self {
            Self::Arm => 1,
            Self::Read { .. } => 2,
            Self::Cancel => 3,
            Self::Select { .. } => 4,
        }
    }
}

control_code_table! {
    /// The status of a `SPECTRUM_CAPTURE` reply.
    SpectrumCaptureStatus {
        /// An arm, a cancel or a select succeeded.
        Done = 1,
        /// A read found no complete window yet.
        Pending = 2,
        /// A read returned a window: the reply carries it.
        Ready = 3,
    }
}

/// The analyzed window of a ready `SPECTRUM_CAPTURE` read. The planes borrow the session's
/// analysis when it encodes and own their words when decoded.
#[derive(Clone, Debug, PartialEq)]
pub struct SpectrumCaptureWindow<'a> {
    /// The boundary the window was captured at.
    pub target_kind: SpectrumTargetKind,
    /// The strip or output it was captured at, by stable ID.
    pub target_id: Cow<'a, str>,
    /// The analyzed planes.
    pub channels: ParameterChannel,
    /// The session rate the window was captured at.
    pub sample_rate_hz: u32,
    /// [`SPECTRUM_CAPTURE_WINDOW_FRAMES`].
    pub window_frames: u32,
    /// [`SPECTRUM_CAPTURE_BIN_COUNT`].
    pub bin_count: u32,
    /// The lowest level a bin reports, in dB.
    pub floor_db: f32,
    /// Absolute sample at the start of the window.
    pub captured_sample: u64,
    /// Exclusive absolute sample at its end: `captured_sample + window_frames`.
    pub end_sample: u64,
    /// The host-scoped monotonic token of the window, from 1.
    pub snapshot_token: u64,
    /// Whether a source underrun was known while the window was captured.
    pub source_underrun: bool,
    /// The frequency axis in hertz, `bin_count` words.
    pub frequencies_hz: Cow<'a, [f32]>,
    /// The left amplitude in dBFS, present exactly when `channels` holds left.
    pub left_db: Option<Cow<'a, [f32]>>,
    /// The right amplitude in dBFS, present exactly when `channels` holds right.
    pub right_db: Option<Cow<'a, [f32]>>,
    /// The left raw window, `window_frames` words: present exactly when the read asked for the
    /// window and `channels` holds left.
    pub left_window: Option<Cow<'a, [f32]>>,
    /// The right raw window, as `left_window`.
    pub right_window: Option<Cow<'a, [f32]>>,
}

/// The success reply of a `SPECTRUM_CAPTURE`.
#[derive(Clone, Debug, PartialEq)]
pub struct SpectrumCaptureReply<'a> {
    /// The epoch of the configuration the running plan carries.
    pub adopted_epoch: u64,
    /// The service's selection epoch, zero on a single configuration.
    pub selection_epoch: u64,
    /// The status.
    pub status: SpectrumCaptureStatus,
    /// The analyzed window: present exactly when `status` is `Ready`.
    pub window: Option<SpectrumCaptureWindow<'a>>,
}

const fn holds_left(channels: ParameterChannel) -> bool {
    matches!(channels, ParameterChannel::Left | ParameterChannel::Both)
}

const fn holds_right(channels: ParameterChannel) -> bool {
    matches!(channels, ParameterChannel::Right | ParameterChannel::Both)
}

fn capture_target_is_valid(limits: crate::ProtocolLimits, id: &str) -> bool {
    (1..=SPECTRUM_TARGET_ID_MAXIMUM_BYTES).contains(&id.len()) && text_fits(limits, id)
}

fn capture_request_is_valid(limits: crate::ProtocolLimits, value: &SpectrumCaptureRequest) -> bool {
    match value {
        SpectrumCaptureRequest::Select { target_id, .. } => {
            capture_target_is_valid(limits, target_id)
        }
        _ => true,
    }
}

fn plane_is_valid(plane: Option<&[f32]>, expected: bool, words: u32) -> bool {
    match plane {
        Some(plane) => expected && u32::try_from(plane.len()) == Ok(words),
        None => !expected,
    }
}

fn capture_window_is_valid(
    limits: crate::ProtocolLimits,
    value: &SpectrumCaptureWindow<'_>,
) -> bool {
    let left = holds_left(value.channels);
    let right = holds_right(value.channels);
    let windows = value.left_window.is_some() || value.right_window.is_some();
    capture_target_is_valid(limits, &value.target_id)
        && value.window_frames == SPECTRUM_CAPTURE_WINDOW_FRAMES
        && value.bin_count == SPECTRUM_CAPTURE_BIN_COUNT
        && value.floor_db.is_finite()
        && value.snapshot_token != 0
        && value
            .captured_sample
            .checked_add(u64::from(value.window_frames))
            == Some(value.end_sample)
        && plane_is_valid(Some(&value.frequencies_hz), true, value.bin_count)
        && plane_is_valid(value.left_db.as_deref(), left, value.bin_count)
        && plane_is_valid(value.right_db.as_deref(), right, value.bin_count)
        && plane_is_valid(
            value.left_window.as_deref(),
            left && windows,
            value.window_frames,
        )
        && plane_is_valid(
            value.right_window.as_deref(),
            right && windows,
            value.window_frames,
        )
}

fn capture_reply_is_valid(limits: crate::ProtocolLimits, value: &SpectrumCaptureReply<'_>) -> bool {
    match &value.window {
        Some(window) => {
            value.status == SpectrumCaptureStatus::Ready && capture_window_is_valid(limits, window)
        }
        None => value.status != SpectrumCaptureStatus::Ready,
    }
}

fn write_packed(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[f32],
) -> Result<(), EncodeError> {
    let bytes = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, bytes, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
}

fn read_packed(value: &[u8]) -> Result<Vec<f32>, DecodeError> {
    if !value.len().is_multiple_of(4) {
        return Err(DecodeError::InvalidValueLength);
    }
    value
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| read_f32(word))
        .collect()
}

pub(crate) fn write_spectrum_capture_request(
    sink: &mut dyn Sink,
    value: &SpectrumCaptureRequest,
) -> Result<(), EncodeError> {
    use spectrum_capture_request as r;
    if !capture_request_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let (channels, target, include_window) = match value {
        SpectrumCaptureRequest::Arm | SpectrumCaptureRequest::Cancel => (None, None, false),
        SpectrumCaptureRequest::Read {
            channels,
            include_window,
        } => (Some(*channels), None, *include_window),
        SpectrumCaptureRequest::Select {
            target_kind,
            target_id,
            channels,
        } => (
            Some(*channels),
            Some((*target_kind, target_id.as_str())),
            false,
        ),
    };
    let count = r::SPEC.field_count(&[
        (r::CHANNELS, usize::from(channels.is_some())),
        (r::TARGET_KIND, usize::from(target.is_some())),
        (r::TARGET_ID, usize::from(target.is_some())),
        (r::INCLUDE_WINDOW, usize::from(include_window)),
    ])?;
    sink.check_field_count(count)?;
    write_spec!(sink, r::OP, &[value.op()])?;
    if let Some(channels) = channels {
        write_spec!(
            sink,
            r::CHANNELS,
            &[schema::parameter_channel_wire(channels)]
        )?;
    }
    if let Some((kind, id)) = target {
        write_spec!(sink, r::TARGET_KIND, &[kind.code()])?;
        write_spec!(sink, r::TARGET_ID, id.as_bytes())?;
    }
    if include_window {
        write_spec!(sink, r::INCLUDE_WINDOW, &[1])?;
    }
    Ok(())
}

fn decode_spectrum_capture_request(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<SpectrumCaptureRequest, DecodeError> {
    use spectrum_capture_request as r;
    let limits = codec.limits();
    let message = message.schema_spec(&r::SPEC)?;
    let op = read_u8(one_spec!(message, r::OP)?)?;
    let channels = optional_spec!(message, r::CHANNELS)?
        .map(|value| {
            schema::parameter_channel_from_wire(read_u8(value)?)
                .map_err(|_| DecodeError::InvalidField)
        })
        .transpose()?;
    let target_kind = optional_spec!(message, r::TARGET_KIND)?
        .map(|value| SpectrumTargetKind::from_code(read_u8(value)?))
        .transpose()?;
    let target_id = optional_spec!(message, r::TARGET_ID)?
        .map(|value| read_string_within(limits, value))
        .transpose()?;
    let include_window = optional_spec!(message, r::INCLUDE_WINDOW)?
        .map(read_bool)
        .transpose()?;
    // Every presence rule of D2: a field outside its operation is `INVALID_FIELD`.
    let value = match (op, channels, target_kind, target_id, include_window) {
        (1, None, None, None, None) => SpectrumCaptureRequest::Arm,
        (2, Some(channels), None, None, include_window) => SpectrumCaptureRequest::Read {
            channels,
            include_window: include_window.unwrap_or(false),
        },
        (3, None, None, None, None) => SpectrumCaptureRequest::Cancel,
        (4, Some(channels), Some(target_kind), Some(target_id), None) => {
            SpectrumCaptureRequest::Select {
                target_kind,
                target_id: target_id.to_owned(),
                channels,
            }
        }
        _ => return Err(DecodeError::InvalidField),
    };
    if !capture_request_is_valid(limits, &value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

pub(crate) fn write_spectrum_capture_reply(
    sink: &mut dyn Sink,
    value: &SpectrumCaptureReply<'_>,
) -> Result<(), EncodeError> {
    use spectrum_capture_reply as r;
    if !capture_reply_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let present = |plane: &Option<Cow<'_, [f32]>>| usize::from(plane.is_some());
    let count = match &value.window {
        None => r::SPEC.field_count(&r::READY.map(|spec| (spec, 0)))?,
        Some(window) => r::SPEC.field_count(&[
            (r::LEFT_DB, present(&window.left_db)),
            (r::RIGHT_DB, present(&window.right_db)),
            (r::LEFT_WINDOW, present(&window.left_window)),
            (r::RIGHT_WINDOW, present(&window.right_window)),
            (r::TARGET_KIND, 1),
            (r::TARGET_ID, 1),
            (r::CHANNELS, 1),
            (r::SAMPLE_RATE_HZ, 1),
            (r::WINDOW_FRAMES, 1),
            (r::BIN_COUNT, 1),
            (r::FLOOR_DB, 1),
            (r::CAPTURED_SAMPLE, 1),
            (r::END_SAMPLE, 1),
            (r::SNAPSHOT_TOKEN, 1),
            (r::SOURCE_UNDERRUN, 1),
            (r::FREQUENCIES_HZ, 1),
        ])?,
    };
    sink.check_field_count(count)?;
    write_spec!(sink, r::ADOPTED_EPOCH, &value.adopted_epoch.to_le_bytes())?;
    write_spec!(
        sink,
        r::SELECTION_EPOCH,
        &value.selection_epoch.to_le_bytes()
    )?;
    write_spec!(sink, r::STATUS, &[value.status.code()])?;
    let Some(window) = &value.window else {
        return Ok(());
    };
    write_spec!(sink, r::TARGET_KIND, &[window.target_kind.code()])?;
    write_spec!(sink, r::TARGET_ID, window.target_id.as_bytes())?;
    write_spec!(
        sink,
        r::CHANNELS,
        &[schema::parameter_channel_wire(window.channels)]
    )?;
    write_spec!(
        sink,
        r::SAMPLE_RATE_HZ,
        &window.sample_rate_hz.to_le_bytes()
    )?;
    write_spec!(sink, r::WINDOW_FRAMES, &window.window_frames.to_le_bytes())?;
    write_spec!(sink, r::BIN_COUNT, &window.bin_count.to_le_bytes())?;
    write_spec!(sink, r::FLOOR_DB, &window.floor_db.to_le_bytes())?;
    write_spec!(
        sink,
        r::CAPTURED_SAMPLE,
        &window.captured_sample.to_le_bytes()
    )?;
    write_spec!(sink, r::END_SAMPLE, &window.end_sample.to_le_bytes())?;
    write_spec!(
        sink,
        r::SNAPSHOT_TOKEN,
        &window.snapshot_token.to_le_bytes()
    )?;
    write_spec!(
        sink,
        r::SOURCE_UNDERRUN,
        &[u8::from(window.source_underrun)]
    )?;
    write_packed(sink, r::FREQUENCIES_HZ, &window.frequencies_hz)?;
    for (spec, plane) in [
        (r::LEFT_DB, &window.left_db),
        (r::RIGHT_DB, &window.right_db),
        (r::LEFT_WINDOW, &window.left_window),
        (r::RIGHT_WINDOW, &window.right_window),
    ] {
        if let Some(plane) = plane {
            write_packed(sink, spec, plane)?;
        }
    }
    Ok(())
}

fn decode_spectrum_capture_reply(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<SpectrumCaptureReply<'static>, DecodeError> {
    use spectrum_capture_reply as r;
    let limits = codec.limits();
    let message = message.schema_spec(&r::SPEC)?;
    let status = SpectrumCaptureStatus::from_code(read_u8(one_spec!(message, r::STATUS)?)?)?;
    let window = if status == SpectrumCaptureStatus::Ready {
        let plane = |spec: schema::FieldSpec| {
            optional_spec!(message, spec)?
                .map(|value| read_packed(value).map(Cow::Owned))
                .transpose()
        };
        Some(SpectrumCaptureWindow {
            target_kind: SpectrumTargetKind::from_code(read_u8(one_spec!(
                message,
                r::TARGET_KIND
            )?)?)?,
            target_id: Cow::Owned(
                read_string_within(limits, one_spec!(message, r::TARGET_ID)?)?.to_owned(),
            ),
            channels: schema::parameter_channel_from_wire(read_u8(one_spec!(
                message,
                r::CHANNELS
            )?)?)
            .map_err(|_| DecodeError::InvalidField)?,
            sample_rate_hz: read_u32(one_spec!(message, r::SAMPLE_RATE_HZ)?)?,
            window_frames: read_u32(one_spec!(message, r::WINDOW_FRAMES)?)?,
            bin_count: read_u32(one_spec!(message, r::BIN_COUNT)?)?,
            floor_db: read_f32(one_spec!(message, r::FLOOR_DB)?)?,
            captured_sample: read_u64(one_spec!(message, r::CAPTURED_SAMPLE)?)?,
            end_sample: read_u64(one_spec!(message, r::END_SAMPLE)?)?,
            snapshot_token: read_u64(one_spec!(message, r::SNAPSHOT_TOKEN)?)?,
            source_underrun: read_bool(one_spec!(message, r::SOURCE_UNDERRUN)?)?,
            frequencies_hz: Cow::Owned(read_packed(one_spec!(message, r::FREQUENCIES_HZ)?)?),
            left_db: plane(r::LEFT_DB)?,
            right_db: plane(r::RIGHT_DB)?,
            left_window: plane(r::LEFT_WINDOW)?,
            right_window: plane(r::RIGHT_WINDOW)?,
        })
    } else {
        // A reply that is not ready carries none of the window's fields.
        for spec in r::READY {
            if optional_spec!(message, spec)?.is_some() {
                return Err(DecodeError::InvalidField);
            }
        }
        None
    };
    let value = SpectrumCaptureReply {
        adopted_epoch: read_u64(one_spec!(message, r::ADOPTED_EPOCH)?)?,
        selection_epoch: read_u64(one_spec!(message, r::SELECTION_EPOCH)?)?,
        status,
        window,
    };
    if !capture_reply_is_valid(limits, &value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

/// Zero words, as long as the longest plane, for the ready-reply measure.
static MEASURE_ZEROS: [f32; SPECTRUM_CAPTURE_WINDOW_FRAMES as usize] =
    [0.0; SPECTRUM_CAPTURE_WINDOW_FRAMES as usize];

/// Bytes of the longest target identity, for the ready-reply measure.
const MEASURE_ID: &str = match core::str::from_utf8(&[b'i'; SPECTRUM_TARGET_ID_MAXIMUM_BYTES]) {
    Ok(id) => id,
    Err(_) => panic!("ASCII is UTF-8"),
};

impl ProtocolCodec {
    /// Return the exact caller-buffer size of a `SPECTRUM_CAPTURE` request.
    pub fn encoded_spectrum_capture_request_len(
        &self,
        value: &SpectrumCaptureRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_spectrum_capture_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `SPECTRUM_CAPTURE` request without allocation.
    pub fn encode_spectrum_capture_request(
        &self,
        value: &SpectrumCaptureRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_spectrum_capture_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_spectrum_capture_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `SPECTRUM_CAPTURE` request.
    pub fn decode_spectrum_capture_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SpectrumCaptureRequest, DecodeError> {
        decode_spectrum_capture_request(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Return the exact caller-buffer size of a `SPECTRUM_CAPTURE` reply.
    pub fn encoded_spectrum_capture_reply_len(
        &self,
        value: &SpectrumCaptureReply<'_>,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_spectrum_capture_reply(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `SPECTRUM_CAPTURE` reply without allocation.
    pub fn encode_spectrum_capture_reply(
        &self,
        value: &SpectrumCaptureReply<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_spectrum_capture_reply_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_spectrum_capture_reply(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `SPECTRUM_CAPTURE` reply.
    pub fn decode_spectrum_capture_reply(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SpectrumCaptureReply<'static>, DecodeError> {
        decode_spectrum_capture_reply(self, Message::top_level(payload, count, self.limits())?)
    }

    /// The complete response frame, in bytes, of a ready `SPECTRUM_CAPTURE` read of `channels`
    /// (with the raw planes when `include_window`) at a target whose identity is
    /// `target_id_bytes` long (#1595 D5, R3). Every field of a ready reply has a fixed width but
    /// the identity and the planes, so the number is exact; it is measured by the encoder itself,
    /// so it cannot drift from it. Allocates nothing.
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidField`] for an identity of 0 or more than
    /// [`SPECTRUM_TARGET_ID_MAXIMUM_BYTES`] bytes; [`EncodeError::LimitExceeded`] when the payload
    /// exceeds this codec's frame limit.
    pub fn spectrum_capture_ready_frame_len(
        &self,
        channels: ParameterChannel,
        include_window: bool,
        target_id_bytes: usize,
    ) -> Result<usize, EncodeError> {
        let target_id = MEASURE_ID
            .get(..target_id_bytes)
            .ok_or(EncodeError::InvalidField)?;
        let bins = &MEASURE_ZEROS[..SPECTRUM_CAPTURE_BIN_COUNT as usize];
        let plane = |held: bool, words: &'static [f32]| held.then_some(Cow::Borrowed(words));
        let windows = |held: bool| plane(held && include_window, &MEASURE_ZEROS);
        let reply = SpectrumCaptureReply {
            adopted_epoch: 0,
            selection_epoch: 0,
            status: SpectrumCaptureStatus::Ready,
            window: Some(SpectrumCaptureWindow {
                target_kind: SpectrumTargetKind::Output,
                target_id: Cow::Borrowed(target_id),
                channels,
                sample_rate_hz: 0,
                window_frames: SPECTRUM_CAPTURE_WINDOW_FRAMES,
                bin_count: SPECTRUM_CAPTURE_BIN_COUNT,
                floor_db: 0.0,
                captured_sample: 0,
                end_sample: u64::from(SPECTRUM_CAPTURE_WINDOW_FRAMES),
                snapshot_token: 1,
                source_underrun: false,
                frequencies_hz: Cow::Borrowed(bins),
                left_db: plane(holds_left(channels), bins),
                right_db: plane(holds_right(channels), bins),
                left_window: windows(holds_left(channels)),
                right_window: windows(holds_right(channels)),
            }),
        };
        self.encoded_spectrum_capture_reply_len(&reply)?
            .checked_add(crate::OUTER_HEADER_BYTES)
            .ok_or(EncodeError::LimitExceeded)
    }
}
