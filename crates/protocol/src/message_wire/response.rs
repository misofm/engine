//! `RESPONSE_GET` (`0x000e`, decision 17, C13): the request that asks for the frequency response of
//! one native effect or of the input filters, and the reply that carries its curves.
//!
//! The request names exactly one target (a configuration, a control address or a strip), the axis
//! and the output shape. The reply is either a lone `request_tag` (the strip target, which answers
//! its curves later as an event) or one complete set of curves. Both directions apply the same
//! shape rules, so no encoder writes a frame the decoder refuses and no decoder admits an array
//! that does not match its declared shape.

use super::{
    ControlAddress, ControlRowKind, Message, ParameterChannel, checked_sink_len,
    checked_writer_len, control_address_is_valid, decode_control_address, read_bool, read_f32,
    read_string_within, read_u8, read_u32, read_u64, stable_id_fits, text_fits,
    write_control_address_fields,
};
use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{CountSink, Sink, SliceSink},
    schema::{self, response_configuration, response_override, response_request, response_result},
};

control_code_table! {
    /// How a response axis spaces its points.
    #[allow(missing_docs)]
    ResponseGrid {
        Linear = 1,
        Logarithmic = 2,
    }
}

/// The control ID that a response override uses for the effect-wide bypass: lane `both`, value
/// exactly `0` or `1`.
pub const RESPONSE_BYPASS_CONTROL: u32 = 0;

/// What an effect-or-filters response is computed for.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseConfiguration {
    /// One native effect at one quality and link mode.
    Effect {
        /// The native effect ID.
        effect: String,
        /// The preparation quality.
        quality: session::EffectQuality,
        /// The channel-link mode.
        link_mode: session::LinkMode,
    },
    /// The builtin input high-pass and low-pass filters.
    InputFilters,
}

/// The one target of a `RESPONSE_GET`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseTarget {
    /// An explicit configuration, computed from the request alone.
    Configuration(ResponseConfiguration),
    /// A control address of the session's own effect, with `control` and `lane` absent.
    Address(ControlAddress),
    /// A strip, by stable ID.
    Strip(String),
}

/// One value the request sets over the target's defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseOverride {
    /// The descriptor parameter ID (or the bypass for [`RESPONSE_BYPASS_CONTROL`]).
    pub control: u32,
    /// The lane it sets.
    pub lane: ParameterChannel,
    /// The value, in the control's own domain.
    pub value: f32,
}

/// A `RESPONSE_GET` request.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseGetRequest {
    /// The one target.
    pub target: ResponseTarget,
    /// The axis spacing.
    pub grid: ResponseGrid,
    /// The number of axis points.
    pub points: u32,
    /// The inclusive lower axis endpoint, in Hz.
    pub minimum_hz: f32,
    /// The inclusive upper axis endpoint, in Hz.
    pub maximum_hz: f32,
    /// The sides to return.
    pub channels: ParameterChannel,
    /// Whether to return the per-section curves as well as the totals.
    pub sections: bool,
    /// The overrides over the target's defaults.
    pub overrides: Vec<ResponseOverride>,
    /// An opaque caller identity, echoed in the reply.
    pub configuration_id: Option<u64>,
    /// The sample rate; a session endpoint refuses it (`response.rate.session`).
    pub sample_rate_hz: Option<u32>,
    /// The render quantum; a session endpoint refuses it.
    pub quantum_frames: Option<u32>,
    /// The most owner payload bytes preparation may retain.
    pub maximum_prepared_bytes: Option<u64>,
    /// The most native effect state bytes the request may admit.
    pub maximum_state_bytes: Option<u64>,
    /// The most native effect scratch bytes the request may admit.
    pub maximum_scratch_bytes: Option<u64>,
    /// The most automation spans per block the request may admit.
    pub maximum_automation_spans: Option<u32>,
    /// The most reply frame bytes the caller accepts.
    pub maximum_reply_bytes: Option<u32>,
}

/// The curves of one response.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseCurves {
    /// The sample rate the owner was prepared at.
    pub sample_rate_hz: u32,
    /// The caller's identity (`0` when the request gave none).
    pub configuration_id: u64,
    /// The lowest level a curve reports, in dB.
    pub floor_db: f32,
    /// Effect-wide bypass as the owner holds it.
    pub bypass: bool,
    /// Number of owner sections.
    pub section_count: u32,
    /// Enabled-section mask of the left side: bit `min(index, 31)` per enabled section.
    pub enabled_left: u32,
    /// Enabled-section mask of the right side.
    pub enabled_right: u32,
    /// Owner payload bytes retained by the preparation.
    pub retained_bytes: u64,
    /// The engine-generated frequency axis.
    pub frequencies_hz: Vec<f32>,
    /// The left total, when the channels include left.
    pub left_db: Option<Vec<f32>>,
    /// The right total, when the channels include right.
    pub right_db: Option<Vec<f32>>,
    /// The left section curves, section-major, with the left total.
    pub sections_left_db: Option<Vec<f32>>,
    /// The right section curves, section-major, with the right total.
    pub sections_right_db: Option<Vec<f32>>,
}

/// The success reply of a `RESPONSE_GET`.
#[derive(Clone, Debug, PartialEq)]
pub enum ResponseResult {
    /// The strip target: its curves arrive later as an event carrying this tag.
    Tag(u64),
    /// One complete set of curves.
    Curves(ResponseCurves),
}

const FRAME_FIELD_PREFIX_BYTES: usize = 8;

/// The bytes one field occupies: its prefix, its value and the padding to eight bytes.
fn field_bytes(value_len: usize) -> Option<usize> {
    FRAME_FIELD_PREFIX_BYTES
        .checked_add(value_len)?
        .checked_add(crate::btlv::padding(value_len))
}

/// The exact frame length, in bytes, of a curve reply: the outer header and the fields of a reply
/// with `points` axis points, `channels` sides, `section_count` owner sections, and the section
/// curves when `sections` is true. `None` when the length does not fit a `usize`.
#[must_use]
pub fn response_result_encoded_len(
    points: usize,
    channels: ParameterChannel,
    section_count: usize,
    sections: bool,
) -> Option<usize> {
    let curve = points.checked_mul(4)?;
    let section_curves = curve.checked_mul(section_count)?;
    let sides = match channels {
        ParameterChannel::Left | ParameterChannel::Right => 1_usize,
        ParameterChannel::Both => 2,
    };
    let mut total = crate::OUTER_HEADER_BYTES;
    // Fields 2 to 9: rate, identity, floor, bypass, section count, both masks, retained bytes.
    for length in [4, 8, 4, 1, 4, 4, 4, 8] {
        total = total.checked_add(field_bytes(length)?)?;
    }
    // Field 10, the totals and the section curves.
    total = total.checked_add(field_bytes(curve)?)?;
    for _ in 0..sides {
        total = total.checked_add(field_bytes(curve)?)?;
        if sections {
            total = total.checked_add(field_bytes(section_curves)?)?;
        }
    }
    Some(total)
}

fn curve_len(values: &[f32]) -> Result<usize, EncodeError> {
    values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)
}

fn finite_all(values: &[f32]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// A request's own rules, shared by both directions.
fn response_request_is_valid(limits: crate::ProtocolLimits, value: &ResponseGetRequest) -> bool {
    let target_ok = match &value.target {
        ResponseTarget::Configuration(ResponseConfiguration::Effect { effect, .. }) => {
            !effect.is_empty() && text_fits(limits, effect)
        }
        ResponseTarget::Configuration(ResponseConfiguration::InputFilters) => true,
        ResponseTarget::Address(address) => {
            control_address_is_valid(limits, address, ControlRowKind::SectionHeader)
        }
        // A strip's curves are the strip's own: it has no sections to ask for and no override.
        ResponseTarget::Strip(id) => {
            stable_id_fits(limits, id) && !value.sections && value.overrides.is_empty()
        }
    };
    // Nothing the request carries as a number is NaN or infinite: the axis endpoints and every
    // override value are finite before any host looks at them, so a host never prepares an owner
    // for a request whose axis or values cannot be meant.
    let finite = value.minimum_hz.is_finite()
        && value.maximum_hz.is_finite()
        && value.overrides.iter().all(|row| row.value.is_finite());
    // The bypass is the effect-wide switch: lane `both`, value exactly 0 or 1.
    let overrides_ok = value.overrides.iter().all(|row| {
        let effect = matches!(
            value.target,
            ResponseTarget::Configuration(ResponseConfiguration::Effect { .. })
        );
        !(effect && row.control == RESPONSE_BYPASS_CONTROL)
            || (row.lane == ParameterChannel::Both
                && (row.value.to_bits() == 0.0_f32.to_bits()
                    || row.value.to_bits() == 1.0_f32.to_bits()))
    });
    target_ok && overrides_ok && finite
}

/// A reply's own rules, shared by both directions.
fn response_result_is_valid(value: &ResponseResult) -> bool {
    let ResponseResult::Curves(curves) = value else {
        return true;
    };
    let points = curves.frequencies_hz.len();
    let Some(section_points) = usize::try_from(curves.section_count)
        .ok()
        .and_then(|count| count.checked_mul(points))
    else {
        return false;
    };
    let totals_fit = |total: &Option<Vec<f32>>| {
        total
            .as_ref()
            .is_none_or(|values| values.len() == points && finite_all(values))
    };
    // A side's section curves exist only with that side's total, and then have every section.
    let sections_fit = |total: &Option<Vec<f32>>, sections: &Option<Vec<f32>>| match sections {
        None => true,
        Some(values) => {
            total.is_some()
                && curves.section_count != 0
                && values.len() == section_points
                && finite_all(values)
        }
    };
    points >= 2
        && curves.floor_db.is_finite()
        && finite_all(&curves.frequencies_hz)
        && (curves.left_db.is_some() || curves.right_db.is_some())
        && totals_fit(&curves.left_db)
        && totals_fit(&curves.right_db)
        && sections_fit(&curves.left_db, &curves.sections_left_db)
        && sections_fit(&curves.right_db, &curves.sections_right_db)
}

fn write_packed(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[f32],
) -> Result<(), EncodeError> {
    sink.stream_field_spec(spec, curve_len(values)?, &mut |sink| {
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
    value.chunks_exact(4).map(read_f32).collect()
}

pub(crate) fn write_response_request(
    sink: &mut dyn Sink,
    value: &ResponseGetRequest,
) -> Result<(), EncodeError> {
    if !response_request_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let configuration = matches!(value.target, ResponseTarget::Configuration(_));
    let address = matches!(value.target, ResponseTarget::Address(_));
    let strip = matches!(value.target, ResponseTarget::Strip(_));
    let one = |present: bool| usize::from(present);
    let count = response_request::SPEC.field_count(&[
        (response_request::CONFIGURATION, one(configuration)),
        (response_request::ADDRESS, one(address)),
        (response_request::STRIP_ID, one(strip)),
        (response_request::OVERRIDE, value.overrides.len()),
        (
            response_request::CONFIGURATION_ID,
            one(value.configuration_id.is_some()),
        ),
        (
            response_request::SAMPLE_RATE_HZ,
            one(value.sample_rate_hz.is_some()),
        ),
        (
            response_request::QUANTUM_FRAMES,
            one(value.quantum_frames.is_some()),
        ),
        (
            response_request::MAXIMUM_PREPARED_BYTES,
            one(value.maximum_prepared_bytes.is_some()),
        ),
        (
            response_request::MAXIMUM_STATE_BYTES,
            one(value.maximum_state_bytes.is_some()),
        ),
        (
            response_request::MAXIMUM_SCRATCH_BYTES,
            one(value.maximum_scratch_bytes.is_some()),
        ),
        (
            response_request::MAXIMUM_AUTOMATION_SPANS,
            one(value.maximum_automation_spans.is_some()),
        ),
        (
            response_request::MAXIMUM_REPLY_BYTES,
            one(value.maximum_reply_bytes.is_some()),
        ),
    ])?;
    sink.check_field_count(count)?;
    match &value.target {
        ResponseTarget::Configuration(configuration) => {
            let fields = response_configuration::SPEC.field_count(&[
                (
                    response_configuration::EFFECT,
                    usize::from(matches!(
                        configuration,
                        ResponseConfiguration::Effect { .. }
                    )),
                ),
                (
                    response_configuration::QUALITY,
                    usize::from(matches!(
                        configuration,
                        ResponseConfiguration::Effect { .. }
                    )),
                ),
                (
                    response_configuration::LINK_MODE,
                    usize::from(matches!(
                        configuration,
                        ResponseConfiguration::Effect { .. }
                    )),
                ),
            ])?;
            sink.nested_spec(response_request::CONFIGURATION, &mut |sink| {
                sink.message_header(fields)?;
                match configuration {
                    ResponseConfiguration::Effect {
                        effect,
                        quality,
                        link_mode,
                    } => {
                        write_spec!(sink, response_configuration::KIND, &[1])?;
                        write_spec!(sink, response_configuration::EFFECT, effect.as_bytes())?;
                        write_spec!(
                            sink,
                            response_configuration::QUALITY,
                            &[crate::session_wire::enum_quality(*quality)]
                        )?;
                        write_spec!(
                            sink,
                            response_configuration::LINK_MODE,
                            &[crate::session_wire::enum_link(*link_mode)]
                        )
                    }
                    ResponseConfiguration::InputFilters => {
                        write_spec!(sink, response_configuration::KIND, &[2])
                    }
                }
            })?;
        }
        ResponseTarget::Address(address) => {
            sink.nested_spec(response_request::ADDRESS, &mut |sink| {
                write_control_address_fields(sink, address)
            })?;
        }
        ResponseTarget::Strip(id) => {
            write_spec!(sink, response_request::STRIP_ID, id.as_bytes())?;
        }
    }
    write_spec!(sink, response_request::GRID, &[value.grid.code()])?;
    write_spec!(sink, response_request::POINTS, &value.points.to_le_bytes())?;
    write_spec!(
        sink,
        response_request::MINIMUM_HZ,
        &value.minimum_hz.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_request::MAXIMUM_HZ,
        &value.maximum_hz.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_request::CHANNELS,
        &[schema::parameter_channel_wire(value.channels)]
    )?;
    write_spec!(
        sink,
        response_request::SECTIONS,
        &[u8::from(value.sections)]
    )?;
    for row in &value.overrides {
        sink.nested_spec(response_request::OVERRIDE, &mut |sink| {
            sink.message_header(response_override::SPEC.field_count(&[])?)?;
            write_spec!(sink, response_override::CONTROL, &row.control.to_le_bytes())?;
            write_spec!(
                sink,
                response_override::LANE,
                &[schema::parameter_channel_wire(row.lane)]
            )?;
            write_spec!(sink, response_override::VALUE, &row.value.to_le_bytes())
        })?;
    }
    if let Some(id) = value.configuration_id {
        write_spec!(sink, response_request::CONFIGURATION_ID, &id.to_le_bytes())?;
    }
    if let Some(rate) = value.sample_rate_hz {
        write_spec!(sink, response_request::SAMPLE_RATE_HZ, &rate.to_le_bytes())?;
    }
    if let Some(frames) = value.quantum_frames {
        write_spec!(
            sink,
            response_request::QUANTUM_FRAMES,
            &frames.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.maximum_prepared_bytes {
        write_spec!(
            sink,
            response_request::MAXIMUM_PREPARED_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.maximum_state_bytes {
        write_spec!(
            sink,
            response_request::MAXIMUM_STATE_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.maximum_scratch_bytes {
        write_spec!(
            sink,
            response_request::MAXIMUM_SCRATCH_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    if let Some(spans) = value.maximum_automation_spans {
        write_spec!(
            sink,
            response_request::MAXIMUM_AUTOMATION_SPANS,
            &spans.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.maximum_reply_bytes {
        write_spec!(
            sink,
            response_request::MAXIMUM_REPLY_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    Ok(())
}

fn decode_response_request(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ResponseGetRequest, DecodeError> {
    let limits = codec.limits();
    let message = message.schema_spec(&response_request::SPEC)?;
    let configuration = optional_spec!(message, response_request::CONFIGURATION)?;
    let address = optional_spec!(message, response_request::ADDRESS)?;
    let strip = optional_spec!(message, response_request::STRIP_ID)?;
    // Exactly one target.
    let target = match (configuration, address, strip) {
        (Some(configuration), None, None) => {
            ResponseTarget::Configuration(decode_response_configuration(
                codec,
                Message::nested_at_depth(configuration, limits, 1)?,
            )?)
        }
        (None, Some(address), None) => ResponseTarget::Address(decode_control_address(
            limits,
            Message::nested_at_depth(address, limits, 1)?,
        )?),
        (None, None, Some(strip)) => {
            ResponseTarget::Strip(read_string_within(limits, strip)?.to_owned())
        }
        _ => return Err(DecodeError::InvalidField),
    };
    let overrides = values_spec!(message, response_request::OVERRIDE)?
        .map(|row| {
            let row =
                Message::nested_at_depth(row, limits, 1)?.schema_spec(&response_override::SPEC)?;
            Ok(ResponseOverride {
                control: read_u32(one_spec!(row, response_override::CONTROL)?)?,
                lane: schema::parameter_channel_from_wire(read_u8(one_spec!(
                    row,
                    response_override::LANE
                )?)?)
                .map_err(|_| DecodeError::InvalidField)?,
                value: read_f32(one_spec!(row, response_override::VALUE)?)?,
            })
        })
        .collect::<Result<Vec<_>, DecodeError>>()?;
    let optional_u32 =
        |spec: schema::FieldSpec| optional_spec!(message, spec)?.map(read_u32).transpose();
    let optional_u64 =
        |spec: schema::FieldSpec| optional_spec!(message, spec)?.map(read_u64).transpose();
    let value = ResponseGetRequest {
        target,
        grid: ResponseGrid::from_code(read_u8(one_spec!(message, response_request::GRID)?)?)?,
        points: read_u32(one_spec!(message, response_request::POINTS)?)?,
        minimum_hz: read_f32(one_spec!(message, response_request::MINIMUM_HZ)?)?,
        maximum_hz: read_f32(one_spec!(message, response_request::MAXIMUM_HZ)?)?,
        channels: schema::parameter_channel_from_wire(read_u8(one_spec!(
            message,
            response_request::CHANNELS
        )?)?)
        .map_err(|_| DecodeError::InvalidField)?,
        sections: read_bool(one_spec!(message, response_request::SECTIONS)?)?,
        overrides,
        configuration_id: optional_u64(response_request::CONFIGURATION_ID)?,
        sample_rate_hz: optional_u32(response_request::SAMPLE_RATE_HZ)?,
        quantum_frames: optional_u32(response_request::QUANTUM_FRAMES)?,
        maximum_prepared_bytes: optional_u64(response_request::MAXIMUM_PREPARED_BYTES)?,
        maximum_state_bytes: optional_u64(response_request::MAXIMUM_STATE_BYTES)?,
        maximum_scratch_bytes: optional_u64(response_request::MAXIMUM_SCRATCH_BYTES)?,
        maximum_automation_spans: optional_u32(response_request::MAXIMUM_AUTOMATION_SPANS)?,
        maximum_reply_bytes: optional_u32(response_request::MAXIMUM_REPLY_BYTES)?,
    };
    if !response_request_is_valid(limits, &value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

fn decode_response_configuration(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ResponseConfiguration, DecodeError> {
    let message = message.schema_spec(&response_configuration::SPEC)?;
    let effect = optional_spec!(message, response_configuration::EFFECT)?;
    let quality = optional_spec!(message, response_configuration::QUALITY)?;
    let link_mode = optional_spec!(message, response_configuration::LINK_MODE)?;
    match read_u8(one_spec!(message, response_configuration::KIND)?)? {
        // The effect, its quality and its link mode are present exactly together.
        1 => {
            let (Some(effect), Some(quality), Some(link_mode)) = (effect, quality, link_mode)
            else {
                return Err(DecodeError::InvalidField);
            };
            Ok(ResponseConfiguration::Effect {
                effect: read_string_within(codec.limits(), effect)?.to_owned(),
                quality: crate::session_wire::parse_quality(read_u8(quality)?)
                    .map_err(|_| DecodeError::InvalidField)?,
                link_mode: crate::session_wire::parse_link(read_u8(link_mode)?)
                    .map_err(|_| DecodeError::InvalidField)?,
            })
        }
        2 if effect.is_none() && quality.is_none() && link_mode.is_none() => {
            Ok(ResponseConfiguration::InputFilters)
        }
        _ => Err(DecodeError::InvalidField),
    }
}

pub(crate) fn write_response_result(
    sink: &mut dyn Sink,
    value: &ResponseResult,
) -> Result<(), EncodeError> {
    if !response_result_is_valid(value) {
        return Err(EncodeError::InvalidField);
    }
    let r = &response_result::SPEC;
    let curves = match value {
        ResponseResult::Tag(tag) => {
            let count = r.field_count(&[(response_result::REQUEST_TAG, 1)])?;
            sink.check_field_count(count)?;
            return write_spec!(sink, response_result::REQUEST_TAG, &tag.to_le_bytes());
        }
        ResponseResult::Curves(curves) => curves,
    };
    let present = |values: &Option<Vec<f32>>| usize::from(values.is_some());
    let count = r.field_count(&[
        (response_result::SAMPLE_RATE_HZ, 1),
        (response_result::CONFIGURATION_ID, 1),
        (response_result::FLOOR_DB, 1),
        (response_result::BYPASS, 1),
        (response_result::SECTION_COUNT, 1),
        (response_result::ENABLED_LEFT, 1),
        (response_result::ENABLED_RIGHT, 1),
        (response_result::RETAINED_BYTES, 1),
        (response_result::FREQUENCIES_HZ, 1),
        (response_result::LEFT_DB, present(&curves.left_db)),
        (response_result::RIGHT_DB, present(&curves.right_db)),
        (
            response_result::SECTIONS_LEFT_DB,
            present(&curves.sections_left_db),
        ),
        (
            response_result::SECTIONS_RIGHT_DB,
            present(&curves.sections_right_db),
        ),
    ])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        response_result::SAMPLE_RATE_HZ,
        &curves.sample_rate_hz.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_result::CONFIGURATION_ID,
        &curves.configuration_id.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_result::FLOOR_DB,
        &curves.floor_db.to_le_bytes()
    )?;
    write_spec!(sink, response_result::BYPASS, &[u8::from(curves.bypass)])?;
    write_spec!(
        sink,
        response_result::SECTION_COUNT,
        &curves.section_count.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_result::ENABLED_LEFT,
        &curves.enabled_left.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_result::ENABLED_RIGHT,
        &curves.enabled_right.to_le_bytes()
    )?;
    write_spec!(
        sink,
        response_result::RETAINED_BYTES,
        &curves.retained_bytes.to_le_bytes()
    )?;
    write_packed(
        sink,
        response_result::FREQUENCIES_HZ,
        &curves.frequencies_hz,
    )?;
    for (spec, values) in [
        (response_result::LEFT_DB, &curves.left_db),
        (response_result::RIGHT_DB, &curves.right_db),
        (response_result::SECTIONS_LEFT_DB, &curves.sections_left_db),
        (
            response_result::SECTIONS_RIGHT_DB,
            &curves.sections_right_db,
        ),
    ] {
        if let Some(values) = values {
            write_packed(sink, spec, values)?;
        }
    }
    Ok(())
}

fn decode_response_result(message: Message<'_>) -> Result<ResponseResult, DecodeError> {
    let message = message.schema_spec(&response_result::SPEC)?;
    if let Some(tag) = optional_spec!(message, response_result::REQUEST_TAG)? {
        // The tag stands alone.
        for spec in [
            response_result::SAMPLE_RATE_HZ,
            response_result::CONFIGURATION_ID,
            response_result::FLOOR_DB,
            response_result::BYPASS,
            response_result::SECTION_COUNT,
            response_result::ENABLED_LEFT,
            response_result::ENABLED_RIGHT,
            response_result::RETAINED_BYTES,
            response_result::FREQUENCIES_HZ,
            response_result::LEFT_DB,
            response_result::RIGHT_DB,
            response_result::SECTIONS_LEFT_DB,
            response_result::SECTIONS_RIGHT_DB,
        ] {
            if optional_spec!(message, spec)?.is_some() {
                return Err(DecodeError::InvalidField);
            }
        }
        return Ok(ResponseResult::Tag(read_u64(tag)?));
    }
    let optional_curve =
        |spec: schema::FieldSpec| optional_spec!(message, spec)?.map(read_packed).transpose();
    let curves = ResponseCurves {
        sample_rate_hz: read_u32(one_spec!(message, response_result::SAMPLE_RATE_HZ)?)?,
        configuration_id: read_u64(one_spec!(message, response_result::CONFIGURATION_ID)?)?,
        floor_db: read_f32(one_spec!(message, response_result::FLOOR_DB)?)?,
        bypass: read_bool(one_spec!(message, response_result::BYPASS)?)?,
        section_count: read_u32(one_spec!(message, response_result::SECTION_COUNT)?)?,
        enabled_left: read_u32(one_spec!(message, response_result::ENABLED_LEFT)?)?,
        enabled_right: read_u32(one_spec!(message, response_result::ENABLED_RIGHT)?)?,
        retained_bytes: read_u64(one_spec!(message, response_result::RETAINED_BYTES)?)?,
        frequencies_hz: read_packed(one_spec!(message, response_result::FREQUENCIES_HZ)?)?,
        left_db: optional_curve(response_result::LEFT_DB)?,
        right_db: optional_curve(response_result::RIGHT_DB)?,
        sections_left_db: optional_curve(response_result::SECTIONS_LEFT_DB)?,
        sections_right_db: optional_curve(response_result::SECTIONS_RIGHT_DB)?,
    };
    let result = ResponseResult::Curves(curves);
    if !response_result_is_valid(&result) {
        return Err(DecodeError::InvalidField);
    }
    Ok(result)
}

impl ProtocolCodec {
    /// Return the exact caller-buffer size of a `RESPONSE_GET` request.
    pub fn encoded_response_request_len(
        &self,
        value: &ResponseGetRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_response_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `RESPONSE_GET` request without allocation.
    pub fn encode_response_request(
        &self,
        value: &ResponseGetRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_response_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_response_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `RESPONSE_GET` request.
    pub fn decode_response_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ResponseGetRequest, DecodeError> {
        decode_response_request(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Return the exact caller-buffer size of a `RESPONSE_GET` reply.
    pub fn encoded_response_result_len(
        &self,
        value: &ResponseResult,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_response_result(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a `RESPONSE_GET` reply without allocation.
    pub fn encode_response_result(
        &self,
        value: &ResponseResult,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_response_result_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_response_result(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a `RESPONSE_GET` reply.
    pub fn decode_response_result(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ResponseResult, DecodeError> {
        decode_response_result(Message::top_level(payload, count, self.limits())?)
    }
}
