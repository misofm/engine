//! `SPECTRUM_CONFIGURE` (`0x0010`, decision 17, C9 and C13; #1594): the request that sets, changes
//! or removes the spectrum taps of a running session, and the reply that names the configuration
//! it set and the plan that carries it.
//!
//! The codec owns the code tables and the form's entry count: a value outside a table, an identity
//! that is empty or longer than [`SPECTRUM_TARGET_ID_MAXIMUM_BYTES`], a hop outside
//! [`SPECTRUM_HOP_FRAMES`] and a single form that does not hold exactly one entry are
//! `INVALID_FIELD` on encode and on decode. The budget rules (#1591 D2) are the core's: the
//! session refuses them with `INVALID_FIELD` and the `spectrum.budget` code. Both directions apply
//! the same rules, so no encoder writes a frame the decoder refuses.

use super::{
    Message, ParameterChannel, checked_sink_len, checked_writer_len, read_bool, read_string_within,
    read_u8, read_u32, read_u64, text_fits,
};
use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{CountSink, Sink, SliceSink},
    schema::{self, spectrum_configure_request, spectrum_configured, spectrum_entry},
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
