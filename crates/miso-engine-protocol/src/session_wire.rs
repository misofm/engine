//! Schema-specific BTLV transaction framing for all V1 session-edit opcodes.
//!
//! This module deliberately has no arbitrary-field escape hatch. Its canonical builders consume
//! the same field specifications used by validation and decoding.

use miso_engine_session::{
    Automation, AutomationSegment, AutomationShape, AutomationTarget, ChannelBuiltins,
    ChannelMatrix, DualMonoBuiltins, DualMonoFader, Effect, EffectIdentity, EffectParam,
    EffectQuality, MatrixOrPan, Output, OutputProfile, Rack, RackName, RenderProfile, Route,
    RouteDestination, RouteSource, SessionLimits, SidechainDeclaration, Source, SourceContent,
    SourceMapping, SourceRegion, StableId, Submix,
};

/// Build the checked-in canonical fixture transaction that contains every V1 edit opcode.
///
/// This is conformance data, not a session-edit convenience API.  It deliberately derives its
/// nested values from the checked-in strict V1 TOML fixture, so the transaction follows the
/// accepted typed model rather than maintaining a second shadow session representation.
#[must_use]
pub fn complete_all_opcode_fixture() -> Vec<SessionEditV1> {
    let session = miso_engine_session::parse_session_toml(include_str!(
        "../../../fixtures/session/v1/canonical.toml"
    ))
    .expect("checked-in canonical session fixture is valid");
    let source = session.sources[0].clone();
    let track = session.tracks[0].clone();
    let effect = track.dynamic.effects[0].clone();
    let route = session.routes[0].clone();
    let automation = session.automation[0].clone();
    let track_id = track.id.clone();
    let effect_id = effect.id.clone();
    let id = |value| StableId::parse(value).expect("literal stable ID");
    vec![
        SessionEditV1::SetSessionId {
            session_id: id("demo.session"),
        },
        SessionEditV1::SetSampleRateHz {
            sample_rate_hz: 48_000,
        },
        SessionEditV1::SetQuantumFrames {
            quantum_frames: 128,
        },
        SessionEditV1::SetRenderProfile {
            render_profile: session.render_profile.clone(),
        },
        SessionEditV1::SetOutputProfile {
            output_profile: session.output_profile.clone(),
        },
        SessionEditV1::SetLimits {
            limits: session.limits.clone(),
        },
        SessionEditV1::UpsertSource {
            source: source.clone(),
        },
        SessionEditV1::RemoveSource {
            source_id: source.id.clone(),
        },
        SessionEditV1::SetSourceSampleRateHz {
            source_id: source.id.clone(),
            sample_rate_hz: 48_000,
        },
        SessionEditV1::SetSourceContent {
            source_id: source.id.clone(),
            content: source.content.clone(),
        },
        SessionEditV1::SetSourceMapping {
            source_id: source.id.clone(),
            mapping: source.mapping.clone(),
        },
        SessionEditV1::UpsertTrack {
            track: track.clone(),
        },
        SessionEditV1::RemoveTrack {
            track_id: track_id.clone(),
        },
        SessionEditV1::SetTrackSourceAssignment {
            track_id: track_id.clone(),
            source_id: source.id.clone(),
            left_source_channel: 0,
            right_source_channel: 1,
        },
        SessionEditV1::SetTrackBuiltins {
            track_id: track_id.clone(),
            builtins: track.builtins.clone(),
        },
        SessionEditV1::SetTrackRack {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            rack: track.dynamic.clone(),
        },
        SessionEditV1::PutTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            final_position: 0,
            effect: effect.clone(),
        },
        SessionEditV1::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
        },
        SessionEditV1::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_ids: vec![effect_id.clone()],
        },
        SessionEditV1::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            identity: effect.identity.clone(),
        },
        SessionEditV1::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            quality: effect.quality,
        },
        SessionEditV1::SetEffectBypass {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            bypass: effect.bypass,
        },
        SessionEditV1::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            link_mode: effect.link_mode,
        },
        SessionEditV1::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            sidechain: effect.sidechain.clone(),
        },
        SessionEditV1::UpsertEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            param: effect.params[0].clone(),
        },
        SessionEditV1::RemoveEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            parameter_id: effect.params[0].parameter_id,
            channel: effect.params[0].channel,
        },
        SessionEditV1::SetTrackFader {
            track_id: track_id.clone(),
            fader: track.fader.clone(),
        },
        SessionEditV1::SetTrackMatrixOrPan {
            track_id: track_id.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        },
        SessionEditV1::UpsertSubmix {
            submix: Submix { id: id("drums") },
        },
        SessionEditV1::RemoveSubmix {
            submix_id: id("drums"),
        },
        SessionEditV1::UpsertOutput {
            output: Output { id: id("alt-out") },
        },
        SessionEditV1::RemoveOutput {
            output_id: id("alt-out"),
        },
        SessionEditV1::UpsertRoute {
            route: route.clone(),
        },
        SessionEditV1::RemoveRoute {
            route_id: route.id.clone(),
        },
        SessionEditV1::SetRouteSource {
            route_id: route.id.clone(),
            source: route.source.clone(),
        },
        SessionEditV1::SetRouteDestination {
            route_id: route.id.clone(),
            destination: route.destination.clone(),
        },
        SessionEditV1::SetRouteChannelMatrix {
            route_id: route.id.clone(),
            channel_matrix: route.channel_matrix.clone(),
        },
        SessionEditV1::SetRouteGainDb {
            route_id: route.id.clone(),
            gain_db: route.gain_db,
        },
        SessionEditV1::UpsertAutomation {
            automation: automation.clone(),
        },
        SessionEditV1::RemoveAutomation {
            automation_id: automation.id.clone(),
        },
        SessionEditV1::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: automation.target.clone(),
        },
        SessionEditV1::SetAutomationSegments {
            automation_id: automation.id.clone(),
            segments: automation.segments.clone(),
        },
    ]
}

use crate::{
    CommandFrame, CommandHeader, DecodeError, DecodeScratch, DecodedFrame, EncodeError,
    ExpectedRevision, Frame, MessageId, ProtocolCodec, RequestId, SessionEditV1,
    btlv::{
        CountSink, Fields as Message, MessageMeasure, Sink, SliceSink, read_f32,
        read_u8 as read_u8_exact, read_u16 as read_u16_exact, read_u32 as read_u32_exact,
        read_u64 as read_u64_exact,
    },
    schema::{self, FieldSpec},
};

macro_rules! one_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.one(spec.id, spec.wire.raw())
    }};
}

macro_rules! values_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.values(spec.id, spec.wire.raw())
    }};
}

/// One typed `SESSION_TRANSACTION_APPLY` command ready for schema-specific BTLV encoding.
pub struct SessionTransactionFrame<'a> {
    /// Nonzero endpoint request correlation identity.
    pub request_id: RequestId,
    /// Structural transaction precondition. A dispatcher rejects `Any` for mutation execution.
    pub expected_revision: ExpectedRevision,
    /// Edits in frozen wire execution order.
    pub edits: &'a [SessionEditV1],
}

/// A strictly decoded transaction with a borrowed frame and owned accepted-session edit values.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedSessionTransaction<'a> {
    /// Validated outer command frame borrowed from caller input.
    pub frame: DecodedFrame<'a>,
    /// Typed session edits in the exact received wire order.
    pub edits: Vec<SessionEditV1>,
}

impl ProtocolCodec {
    /// Return the exact caller-output length for a currently supported transaction encoder.
    pub fn encoded_session_transaction_len(
        &self,
        transaction: &SessionTransactionFrame<'_>,
    ) -> Result<usize, EncodeError> {
        let measure = self.measure_session_transaction_payload(transaction)?;
        crate::OUTER_HEADER_BYTES
            .checked_add(measure.length)
            .ok_or(EncodeError::LimitExceeded)
    }

    fn measure_session_transaction_payload(
        &self,
        transaction: &SessionTransactionFrame<'_>,
    ) -> Result<MessageMeasure, EncodeError> {
        if !matches!(transaction.expected_revision, ExpectedRevision::Exact(_))
            || transaction.edits.is_empty()
        {
            return Err(EncodeError::MessageKindMismatch);
        }
        let mut sizing = CountSink::new(transaction_envelope_limits(self.limits()));
        encode_transaction_payload_into(&mut sizing, transaction.edits)?;
        let measure = sizing.measure_message()?;
        let required = crate::OUTER_HEADER_BYTES
            .checked_add(measure.length)
            .ok_or(EncodeError::LimitExceeded)?;
        if required > self.limits().max_frame_bytes {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(measure)
    }

    /// Encode a canonical session transaction. A short caller buffer is left wholly unmodified.
    pub fn encode_session_transaction(
        &self,
        transaction: &SessionTransactionFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let measure = self.measure_session_transaction_payload(transaction)?;
        let required = crate::OUTER_HEADER_BYTES
            .checked_add(measure.length)
            .ok_or(EncodeError::LimitExceeded)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        self.encode(
            &Frame::Command(CommandFrame {
                request_id: transaction.request_id,
                expected_revision: transaction.expected_revision,
                message_id: MessageId::SessionTransactionApply,
            }),
            &mut output[..crate::OUTER_HEADER_BYTES],
        )?;
        put_u32(
            output,
            20,
            u32::try_from(required - crate::OUTER_HEADER_BYTES)
                .map_err(|_| EncodeError::LimitExceeded)?,
        );
        put_u32(output, 40, measure.field_count);
        let mut writer = SliceSink::new(
            &mut output[crate::OUTER_HEADER_BYTES..required],
            transaction_envelope_limits(self.limits()),
        );
        encode_transaction_payload_into(&mut writer, transaction.edits)?;
        if writer.measure_message()? != measure {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(required)
    }

    /// Decode and validate only the borrowed outer transaction frame without allocating edits.
    pub fn decode_session_transaction_outer<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let limits = self.limits();
        // The transaction/edit/payload wrappers are fixed protocol envelopes. They do not consume
        // the configured logical model-message nesting allowance.
        let envelope_codec = ProtocolCodec::new(transaction_envelope_limits(limits));
        let frame = envelope_codec.decode(input, scratch)?;
        let Some(header) = frame.header.command() else {
            return Err(DecodeError::MessageKindMismatch);
        };
        if header.message_id != MessageId::SessionTransactionApply {
            return Err(DecodeError::MessageKindMismatch);
        }
        if !matches!(header.expected_revision, ExpectedRevision::Exact(_)) {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(DecodedSessionTransaction {
            frame,
            edits: Vec::new(),
        })
    }

    /// Decode the supported V1 transaction edit subset into typed accepted-session model values.
    /// This is control-plane convenience decoding; the outer [`ProtocolCodec::decode`] path stays
    /// borrowed and caller-scratch bounded.
    pub fn decode_session_transaction<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let frame = self.decode_header(input)?;
        let header = transaction_header(frame)?;
        scratch.prepare(header.tlv_count)?;
        self.decode_session_transaction_frame(frame)
    }

    pub(crate) fn decode_session_transaction_frame<'a>(
        &self,
        frame: DecodedFrame<'a>,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        self.decode_session_transaction_frame_limited(frame, None)
    }

    /// Decode a transaction only after its exact repeated edit count is within the endpoint cap.
    pub fn decode_session_transaction_limited<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
        maximum_edits: u32,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let frame = self.decode_header(input)?;
        let header = transaction_header(frame)?;
        scratch.prepare(header.tlv_count)?;
        self.decode_session_transaction_frame_limited(frame, Some(maximum_edits))
    }

    pub(crate) fn decode_session_transaction_frame_limited<'a>(
        &self,
        frame: DecodedFrame<'a>,
        maximum_edits: Option<u32>,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let header = transaction_header(frame)?;
        let limits = self.limits();
        let envelope_limits = transaction_envelope_limits(limits);
        let top = Message::top_level(frame.payload, header.tlv_count, envelope_limits)?
            .schema_spec(&schema::session::transaction::SPEC)?;
        let count = u32::try_from(values_spec!(top, schema::session::transaction::EDIT)?.count())
            .map_err(|_| DecodeError::LimitExceeded)?;
        if count == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        if maximum_edits.is_some_and(|maximum| count > maximum) {
            return Err(DecodeError::LimitExceeded);
        }
        let mut edits = Vec::with_capacity(count as usize);
        for value in values_spec!(top, schema::session::transaction::EDIT)? {
            edits.push(parse_edit(top.nested_value(value)?)?);
        }
        Ok(DecodedSessionTransaction { frame, edits })
    }
}

fn transaction_header(frame: DecodedFrame<'_>) -> Result<CommandHeader, DecodeError> {
    let header = frame
        .header
        .command()
        .ok_or(DecodeError::MessageKindMismatch)?;
    if header.message_id != MessageId::SessionTransactionApply {
        return Err(DecodeError::MessageKindMismatch);
    }
    if !matches!(header.expected_revision, ExpectedRevision::Exact(_)) {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(header)
}

const fn enum_quality(value: EffectQuality) -> u8 {
    match value {
        EffectQuality::Draft => 1,
        EffectQuality::Normal => 2,
        EffectQuality::High => 3,
    }
}
const fn enum_link(value: miso_engine_session::LinkMode) -> u8 {
    match value {
        miso_engine_session::LinkMode::DualMono => 1,
        miso_engine_session::LinkMode::Maximum => 2,
        miso_engine_session::LinkMode::Average => 3,
    }
}
const fn enum_tap(value: miso_engine_session::SendTap) -> u8 {
    match value {
        miso_engine_session::SendTap::Input => 1,
        miso_engine_session::SendTap::PostInputBuiltins => 2,
        miso_engine_session::SendTap::PostSimd1 => 3,
        miso_engine_session::SendTap::PostDynamic => 4,
        miso_engine_session::SendTap::PostSimd2PreFader => 5,
        miso_engine_session::SendTap::PostFader => 6,
        miso_engine_session::SendTap::PostMatrix => 7,
    }
}
const fn enum_shape(value: AutomationShape) -> u8 {
    match value {
        AutomationShape::Step => 1,
        AutomationShape::Linear => 2,
        AutomationShape::Exponential => 3,
    }
}

fn tx_start_message(sink: &mut dyn Sink, count: u32) -> Result<(), EncodeError> {
    sink.message_header(count)
}

fn tx_field(sink: &mut dyn Sink, spec: FieldSpec, value: &[u8]) -> Result<(), EncodeError> {
    sink.field_spec(spec, value)
}

fn tx_u8(sink: &mut dyn Sink, spec: FieldSpec, value: u8) -> Result<(), EncodeError> {
    tx_field(sink, spec, &[value])
}
fn tx_u16(sink: &mut dyn Sink, spec: FieldSpec, value: u16) -> Result<(), EncodeError> {
    tx_field(sink, spec, &value.to_le_bytes())
}
fn tx_u32(sink: &mut dyn Sink, spec: FieldSpec, value: u32) -> Result<(), EncodeError> {
    tx_field(sink, spec, &value.to_le_bytes())
}
fn tx_u64(sink: &mut dyn Sink, spec: FieldSpec, value: u64) -> Result<(), EncodeError> {
    tx_field(sink, spec, &value.to_le_bytes())
}
fn tx_f32(sink: &mut dyn Sink, spec: FieldSpec, value: f32) -> Result<(), EncodeError> {
    tx_field(sink, spec, &value.to_le_bytes())
}
fn tx_bool(sink: &mut dyn Sink, spec: FieldSpec, value: bool) -> Result<(), EncodeError> {
    tx_field(sink, spec, &[u8::from(value)])
}
fn tx_text(sink: &mut dyn Sink, spec: FieldSpec, value: &str) -> Result<(), EncodeError> {
    if value.len() > sink.limits().max_string_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    tx_field(sink, spec, value.as_bytes())
}
fn tx_id(sink: &mut dyn Sink, spec: FieldSpec, value: &StableId) -> Result<(), EncodeError> {
    tx_text(sink, spec, value.as_str())
}
fn tx_message(
    sink: &mut dyn Sink,
    spec: FieldSpec,
    mut encode: impl FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut encode)
}

pub(crate) const fn transaction_envelope_limits(
    limits: crate::ProtocolLimits,
) -> crate::ProtocolLimits {
    crate::ProtocolLimits {
        max_nesting: limits.max_nesting.saturating_add(3),
        ..limits
    }
}

fn encode_transaction_payload_into(
    sink: &mut dyn Sink,
    edits: &[SessionEditV1],
) -> Result<(), EncodeError> {
    let count = schema::session::transaction::SPEC
        .field_count(&[(schema::session::transaction::EDIT, edits.len())])?;
    sink.check_field_count(count)?;
    for edit in edits {
        tx_message(sink, schema::session::transaction::EDIT, |nested| {
            tx_edit_message(nested, edit)
        })?;
    }
    Ok(())
}

fn tx_edit_message(sink: &mut dyn Sink, edit: &SessionEditV1) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::edit::SPEC.field_count(&[])?)?;
    tx_u16(sink, schema::session::edit::OPCODE, edit.opcode().raw())?;
    tx_message(sink, schema::session::edit::PAYLOAD, |nested| {
        tx_edit_payload(nested, edit)
    })
}

fn tx_edit_payload(sink: &mut dyn Sink, edit: &SessionEditV1) -> Result<(), EncodeError> {
    let spec = schema::session::payload_spec(edit.opcode());
    let fields = spec.fields;
    let count = match edit {
        SessionEditV1::SetTrackEffectOrder { effect_ids, .. } => {
            spec.field_count(&[(fields[2], effect_ids.len())])?
        }
        SessionEditV1::SetAutomationSegments { segments, .. } => {
            spec.field_count(&[(fields[1], segments.len())])?
        }
        _ => spec.field_count(&[])?,
    };
    tx_start_message(sink, count)?;
    match edit {
        SessionEditV1::SetSessionId { session_id } => tx_id(sink, fields[0], session_id),
        SessionEditV1::SetSampleRateHz { sample_rate_hz } => {
            tx_u32(sink, fields[0], *sample_rate_hz)
        }
        SessionEditV1::SetQuantumFrames { quantum_frames } => {
            tx_u32(sink, fields[0], *quantum_frames)
        }
        SessionEditV1::SetRenderProfile { render_profile } => {
            tx_message(sink, fields[0], |v| tx_render_profile(v, render_profile))
        }
        SessionEditV1::SetOutputProfile { output_profile } => {
            tx_message(sink, fields[0], |v| tx_output_profile(v, output_profile))
        }
        SessionEditV1::SetLimits { limits } => {
            tx_message(sink, fields[0], |v| tx_limits(v, limits))
        }
        SessionEditV1::UpsertSource { source } => {
            tx_message(sink, fields[0], |v| tx_source(v, source))
        }
        SessionEditV1::RemoveSource { source_id } => tx_id(sink, fields[0], source_id),
        SessionEditV1::SetSourceSampleRateHz {
            source_id,
            sample_rate_hz,
        } => {
            tx_id(sink, fields[0], source_id)?;
            tx_u32(sink, fields[1], *sample_rate_hz)
        }
        SessionEditV1::SetSourceContent { source_id, content } => {
            tx_id(sink, fields[0], source_id)?;
            tx_message(sink, fields[1], |v| tx_content(v, content))
        }
        SessionEditV1::SetSourceMapping { source_id, mapping } => {
            tx_id(sink, fields[0], source_id)?;
            tx_message(sink, fields[1], |v| tx_mapping(v, mapping))
        }
        SessionEditV1::UpsertTrack { track } => tx_message(sink, fields[0], |v| tx_track(v, track)),
        SessionEditV1::RemoveTrack { track_id } => tx_id(sink, fields[0], track_id),
        SessionEditV1::SetTrackSourceAssignment {
            track_id,
            source_id,
            left_source_channel,
            right_source_channel,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_id(sink, fields[1], source_id)?;
            tx_u8(sink, fields[2], *left_source_channel)?;
            tx_u8(sink, fields[3], *right_source_channel)
        }
        SessionEditV1::SetTrackBuiltins { track_id, builtins } => {
            tx_id(sink, fields[0], track_id)?;
            tx_message(sink, fields[1], |v| tx_builtins(v, builtins))
        }
        SessionEditV1::SetTrackRack {
            track_id,
            rack_name,
            rack,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_u8(sink, fields[1], schema::parameter_rack_wire(*rack_name))?;
            tx_message(sink, fields[2], |v| tx_rack(v, rack))
        }
        SessionEditV1::PutTrackEffect {
            track_id,
            rack_name,
            final_position,
            effect,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_u8(sink, fields[1], schema::parameter_rack_wire(*rack_name))?;
            tx_u32(sink, fields[2], *final_position)?;
            tx_message(sink, fields[3], |v| tx_effect(v, effect))
        }
        SessionEditV1::RemoveTrackEffect {
            track_id,
            rack_name,
            effect_id,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_u8(sink, fields[1], schema::parameter_rack_wire(*rack_name))?;
            tx_id(sink, fields[2], effect_id)
        }
        SessionEditV1::SetTrackEffectOrder {
            track_id,
            rack_name,
            effect_ids,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_u8(sink, fields[1], schema::parameter_rack_wire(*rack_name))?;
            for effect_id in effect_ids {
                tx_id(sink, fields[2], effect_id)?;
            }
            Ok(())
        }
        SessionEditV1::SetEffectIdentity {
            track_id,
            rack_name,
            effect_id,
            identity,
        } => tx_effect_edit_message(sink, fields, track_id, *rack_name, effect_id, |v| {
            tx_identity(v, identity)
        }),
        SessionEditV1::SetEffectQuality {
            track_id,
            rack_name,
            effect_id,
            quality,
        } => tx_effect_edit_scalar(
            sink,
            fields,
            track_id,
            *rack_name,
            effect_id,
            enum_quality(*quality),
        ),
        SessionEditV1::SetEffectBypass {
            track_id,
            rack_name,
            effect_id,
            bypass,
        } => {
            tx_effect_edit_prefix(sink, fields, track_id, *rack_name, effect_id)?;
            tx_bool(sink, fields[3], *bypass)
        }
        SessionEditV1::SetEffectLinkMode {
            track_id,
            rack_name,
            effect_id,
            link_mode,
        } => tx_effect_edit_scalar(
            sink,
            fields,
            track_id,
            *rack_name,
            effect_id,
            enum_link(*link_mode),
        ),
        SessionEditV1::SetEffectSidechain {
            track_id,
            rack_name,
            effect_id,
            sidechain,
        } => tx_effect_edit_message(sink, fields, track_id, *rack_name, effect_id, |v| {
            tx_sidechain(v, sidechain)
        }),
        SessionEditV1::UpsertEffectParam {
            track_id,
            rack_name,
            effect_id,
            param,
        } => tx_effect_edit_message(sink, fields, track_id, *rack_name, effect_id, |v| {
            tx_param(v, param)
        }),
        SessionEditV1::RemoveEffectParam {
            track_id,
            rack_name,
            effect_id,
            parameter_id,
            channel,
        } => {
            tx_effect_edit_prefix(sink, fields, track_id, *rack_name, effect_id)?;
            tx_u32(sink, fields[3], *parameter_id)?;
            tx_u8(sink, fields[4], schema::parameter_channel_wire(*channel))
        }
        SessionEditV1::SetTrackFader { track_id, fader } => {
            tx_id(sink, fields[0], track_id)?;
            tx_message(sink, fields[1], |v| tx_fader(v, fader))
        }
        SessionEditV1::SetTrackMatrixOrPan {
            track_id,
            matrix_or_pan,
        } => {
            tx_id(sink, fields[0], track_id)?;
            tx_message(sink, fields[1], |v| tx_matrix_or_pan(v, matrix_or_pan))
        }
        SessionEditV1::UpsertSubmix { submix } => {
            tx_message(sink, fields[0], |v| tx_submix(v, submix))
        }
        SessionEditV1::RemoveSubmix { submix_id } => tx_id(sink, fields[0], submix_id),
        SessionEditV1::UpsertOutput { output } => {
            tx_message(sink, fields[0], |v| tx_output(v, output))
        }
        SessionEditV1::RemoveOutput { output_id } => tx_id(sink, fields[0], output_id),
        SessionEditV1::UpsertRoute { route } => tx_message(sink, fields[0], |v| tx_route(v, route)),
        SessionEditV1::RemoveRoute { route_id } => tx_id(sink, fields[0], route_id),
        SessionEditV1::SetRouteSource { route_id, source } => {
            tx_id(sink, fields[0], route_id)?;
            tx_message(sink, fields[1], |v| tx_route_source(v, source))
        }
        SessionEditV1::SetRouteDestination {
            route_id,
            destination,
        } => {
            tx_id(sink, fields[0], route_id)?;
            tx_message(sink, fields[1], |v| tx_route_destination(v, destination))
        }
        SessionEditV1::SetRouteChannelMatrix {
            route_id,
            channel_matrix,
        } => {
            tx_id(sink, fields[0], route_id)?;
            tx_message(sink, fields[1], |v| tx_channel_matrix(v, channel_matrix))
        }
        SessionEditV1::SetRouteGainDb { route_id, gain_db } => {
            tx_id(sink, fields[0], route_id)?;
            tx_f32(sink, fields[1], *gain_db)
        }
        SessionEditV1::UpsertAutomation { automation } => {
            tx_message(sink, fields[0], |v| tx_automation(v, automation))
        }
        SessionEditV1::RemoveAutomation { automation_id } => tx_id(sink, fields[0], automation_id),
        SessionEditV1::SetAutomationTarget {
            automation_id,
            target,
        } => {
            tx_id(sink, fields[0], automation_id)?;
            tx_message(sink, fields[1], |v| tx_automation_target(v, target))
        }
        SessionEditV1::SetAutomationSegments {
            automation_id,
            segments,
        } => {
            tx_id(sink, fields[0], automation_id)?;
            for segment in segments {
                tx_message(sink, fields[1], |v| tx_automation_segment(v, segment))?;
            }
            Ok(())
        }
    }
}

fn tx_effect_edit_prefix(
    sink: &mut dyn Sink,
    fields: &[FieldSpec],
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
) -> Result<(), EncodeError> {
    tx_id(sink, fields[0], track_id)?;
    tx_u8(sink, fields[1], schema::parameter_rack_wire(rack_name))?;
    tx_id(sink, fields[2], effect_id)
}
fn tx_effect_edit_scalar(
    sink: &mut dyn Sink,
    fields: &[FieldSpec],
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
    value: u8,
) -> Result<(), EncodeError> {
    tx_effect_edit_prefix(sink, fields, track_id, rack_name, effect_id)?;
    tx_u8(sink, fields[3], value)
}
fn tx_effect_edit_message(
    sink: &mut dyn Sink,
    fields: &[FieldSpec],
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
    encode: impl FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    tx_effect_edit_prefix(sink, fields, track_id, rack_name, effect_id)?;
    tx_message(sink, fields[3], encode)
}

fn tx_render_profile(sink: &mut dyn Sink, value: &RenderProfile) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::render_profile::SPEC.field_count(&[])?,
    )?;
    tx_id(sink, schema::session::render_profile::ID, &value.id)?;
    tx_u8(
        sink,
        schema::session::render_profile::MODE,
        match value.mode {
            miso_engine_session::RenderMode::SingleThread => 1,
            miso_engine_session::RenderMode::DependencyWaves => 2,
        },
    )
}
fn tx_output_profile(sink: &mut dyn Sink, value: &OutputProfile) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::output_profile::SPEC.field_count(&[])?,
    )?;
    tx_id(sink, schema::session::output_profile::ID, &value.id)?;
    tx_u8(
        sink,
        schema::session::output_profile::CHANNELS,
        value.channels,
    )?;
    tx_u8(sink, schema::session::output_profile::LAYOUT, 1)
}
fn tx_limits(sink: &mut dyn Sink, value: &SessionLimits) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::limits::SPEC.field_count(&[])?)?;
    tx_u64(
        sink,
        schema::session::limits::PCM_RING_FRAMES,
        value.pcm_ring_frames,
    )?;
    tx_u64(
        sink,
        schema::session::limits::CONTROL_QUEUE_MESSAGES,
        value.control_queue_messages,
    )?;
    tx_u64(
        sink,
        schema::session::limits::MEMORY_BYTES,
        value.memory_bytes,
    )
}
fn tx_content(sink: &mut dyn Sink, value: &SourceContent) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::content::SPEC.field_count(&[])?)?;
    tx_text(sink, schema::session::content::IDENTITY, &value.identity)?;
    tx_text(sink, schema::session::content::LOCATOR, &value.locator)
}
fn tx_region(sink: &mut dyn Sink, value: &SourceRegion) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::region::SPEC.field_count(&[])?)?;
    tx_u64(
        sink,
        schema::session::region::START_SAMPLE,
        value.start_sample,
    )?;
    tx_u64(
        sink,
        schema::session::region::LENGTH_SAMPLES,
        value.length_samples,
    )
}
fn tx_mapping(sink: &mut dyn Sink, value: &SourceMapping) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::mapping::SPEC.field_count(&[])?)?;
    tx_u8(
        sink,
        schema::session::mapping::CHANNEL_COUNT,
        value.channel_count,
    )?;
    tx_message(sink, schema::session::mapping::REGION, |v| {
        tx_region(v, &value.region)
    })
}
fn tx_source(sink: &mut dyn Sink, value: &Source) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::source::SPEC.field_count(&[])?)?;
    tx_id(sink, schema::session::source::ID, &value.id)?;
    tx_u32(
        sink,
        schema::session::source::SAMPLE_RATE_HZ,
        value.sample_rate_hz,
    )?;
    tx_message(sink, schema::session::source::CONTENT, |v| {
        tx_content(v, &value.content)
    })?;
    tx_message(sink, schema::session::source::MAPPING, |v| {
        tx_mapping(v, &value.mapping)
    })
}
fn tx_builtins(sink: &mut dyn Sink, value: &DualMonoBuiltins) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::builtins::SPEC.field_count(&[])?)?;
    tx_message(sink, schema::session::builtins::LEFT, |v| {
        tx_channel_builtins(v, &value.left)
    })?;
    tx_message(sink, schema::session::builtins::RIGHT, |v| {
        tx_channel_builtins(v, &value.right)
    })
}
fn tx_channel_builtins(sink: &mut dyn Sink, value: &ChannelBuiltins) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::channel_builtins::SPEC.field_count(&[])?,
    )?;
    tx_bool(
        sink,
        schema::session::channel_builtins::POLARITY_INVERT,
        value.polarity_invert,
    )?;
    tx_f32(
        sink,
        schema::session::channel_builtins::TRIM_DB,
        value.trim_db,
    )?;
    tx_f32(
        sink,
        schema::session::channel_builtins::HPF_HZ,
        value.hpf_hz,
    )?;
    tx_f32(
        sink,
        schema::session::channel_builtins::LPF_HZ,
        value.lpf_hz,
    )
}
fn tx_rack(sink: &mut dyn Sink, value: &Rack) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::rack::SPEC
            .field_count(&[(schema::session::rack::EFFECT, value.effects.len())])?,
    )?;
    for effect in &value.effects {
        tx_message(sink, schema::session::rack::EFFECT, |v| {
            tx_effect(v, effect)
        })?;
    }
    Ok(())
}
fn tx_identity(sink: &mut dyn Sink, value: &EffectIdentity) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::effect_identity::SPEC.field_count(&[])?,
    )?;
    match value {
        EffectIdentity::Native { effect_id } => {
            tx_u8(sink, schema::session::effect_identity::TAG, 1)?;
            tx_id(sink, schema::session::effect_identity::VALUE, effect_id)
        }
        EffectIdentity::ThirdPartyCid { cid } => {
            tx_u8(sink, schema::session::effect_identity::TAG, 2)?;
            tx_text(sink, schema::session::effect_identity::VALUE, cid)
        }
    }
}
fn tx_route_source(sink: &mut dyn Sink, value: &RouteSource) -> Result<(), EncodeError> {
    match value {
        RouteSource::Track { track_id, tap } => {
            tx_start_message(sink, schema::session::route_source::TRACK.field_count(&[])?)?;
            tx_u8(sink, schema::session::route_source::TAG, 1)?;
            tx_id(sink, schema::session::route_source::ID, track_id)?;
            tx_u8(sink, schema::session::route_source::TAP, enum_tap(*tap))
        }
        RouteSource::SubmixOutput { submix_id } => {
            tx_start_message(
                sink,
                schema::session::route_source::SUBMIX.field_count(&[])?,
            )?;
            tx_u8(sink, schema::session::route_source::TAG, 2)?;
            tx_id(sink, schema::session::route_source::ID, submix_id)
        }
    }
}
fn tx_route_destination(sink: &mut dyn Sink, value: &RouteDestination) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::route_destination::SPEC.field_count(&[])?,
    )?;
    match value {
        RouteDestination::SubmixInput { submix_id } => {
            tx_u8(sink, schema::session::route_destination::TAG, 1)?;
            tx_id(sink, schema::session::route_destination::ID, submix_id)
        }
        RouteDestination::OutputInput { output_id } => {
            tx_u8(sink, schema::session::route_destination::TAG, 2)?;
            tx_id(sink, schema::session::route_destination::ID, output_id)
        }
    }
}
fn tx_sidechain(sink: &mut dyn Sink, value: &SidechainDeclaration) -> Result<(), EncodeError> {
    match value {
        SidechainDeclaration::None => {
            tx_start_message(sink, schema::session::sidechain::NONE.field_count(&[])?)?;
            tx_u8(sink, schema::session::sidechain::TAG, 1)
        }
        SidechainDeclaration::Routed(value) => {
            tx_start_message(sink, schema::session::sidechain::ROUTED.field_count(&[])?)?;
            tx_u8(sink, schema::session::sidechain::TAG, 2)?;
            tx_message(sink, schema::session::sidechain::SOURCE, |v| {
                tx_route_source(v, &value.source)
            })?;
            tx_id(sink, schema::session::sidechain::PORT_ID, &value.port_id)
        }
    }
}
fn tx_param(sink: &mut dyn Sink, value: &EffectParam) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::param::SPEC.field_count(&[])?)?;
    tx_u32(
        sink,
        schema::session::param::PARAMETER_ID,
        value.parameter_id,
    )?;
    tx_u8(
        sink,
        schema::session::param::CHANNEL,
        schema::parameter_channel_wire(value.channel),
    )?;
    tx_u8(
        sink,
        schema::session::param::UNIT,
        schema::parameter_unit_wire(value.unit),
    )?;
    tx_f32(sink, schema::session::param::VALUE, value.value)
}
fn tx_effect(sink: &mut dyn Sink, value: &Effect) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::effect::SPEC
            .field_count(&[(schema::session::effect::PARAM, value.params.len())])?,
    )?;
    tx_id(sink, schema::session::effect::ID, &value.id)?;
    tx_message(sink, schema::session::effect::IDENTITY, |v| {
        tx_identity(v, &value.identity)
    })?;
    tx_u8(
        sink,
        schema::session::effect::QUALITY,
        enum_quality(value.quality),
    )?;
    tx_bool(sink, schema::session::effect::BYPASS, value.bypass)?;
    tx_u8(
        sink,
        schema::session::effect::LINK_MODE,
        enum_link(value.link_mode),
    )?;
    for param in &value.params {
        tx_message(sink, schema::session::effect::PARAM, |v| tx_param(v, param))?;
    }
    tx_message(sink, schema::session::effect::SIDECHAIN, |v| {
        tx_sidechain(v, &value.sidechain)
    })
}
fn tx_fader(sink: &mut dyn Sink, value: &DualMonoFader) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::fader::SPEC.field_count(&[])?)?;
    tx_f32(sink, schema::session::fader::LEFT_DB, value.left_db)?;
    tx_f32(sink, schema::session::fader::RIGHT_DB, value.right_db)?;
    tx_bool(sink, schema::session::fader::LEFT_MUTE, value.left_mute)?;
    tx_bool(sink, schema::session::fader::RIGHT_MUTE, value.right_mute)
}
fn tx_matrix_or_pan(sink: &mut dyn Sink, value: &MatrixOrPan) -> Result<(), EncodeError> {
    match value {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => {
            tx_start_message(sink, schema::session::matrix_or_pan::PAN.field_count(&[])?)?;
            tx_u8(sink, schema::session::matrix_or_pan::TAG, 1)?;
            tx_f32(sink, schema::session::matrix_or_pan::A, *left)?;
            tx_f32(sink, schema::session::matrix_or_pan::B, *right)?;
            tx_u32(
                sink,
                schema::session::matrix_or_pan::PAN_SMOOTHING,
                *smoothing_samples,
            )
        }
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => {
            tx_start_message(
                sink,
                schema::session::matrix_or_pan::MATRIX.field_count(&[])?,
            )?;
            tx_u8(sink, schema::session::matrix_or_pan::TAG, 2)?;
            tx_f32(sink, schema::session::matrix_or_pan::A, *ll)?;
            tx_f32(sink, schema::session::matrix_or_pan::B, *lr)?;
            tx_f32(sink, schema::session::matrix_or_pan::C_OR_SMOOTHING, *rl)?;
            tx_f32(sink, schema::session::matrix_or_pan::D, *rr)?;
            tx_u32(
                sink,
                schema::session::matrix_or_pan::SMOOTHING,
                *smoothing_samples,
            )
        }
    }
}
fn tx_track(sink: &mut dyn Sink, value: &miso_engine_session::Track) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::track::SPEC.field_count(&[])?)?;
    tx_id(sink, schema::session::track::ID, &value.id)?;
    tx_id(sink, schema::session::track::SOURCE_ID, &value.source_id)?;
    tx_u8(
        sink,
        schema::session::track::LEFT_SOURCE_CHANNEL,
        value.left_source_channel,
    )?;
    tx_u8(
        sink,
        schema::session::track::RIGHT_SOURCE_CHANNEL,
        value.right_source_channel,
    )?;
    tx_message(sink, schema::session::track::BUILTINS, |v| {
        tx_builtins(v, &value.builtins)
    })?;
    tx_message(sink, schema::session::track::SIMD1, |v| {
        tx_rack(v, &value.simd1)
    })?;
    tx_message(sink, schema::session::track::DYNAMIC, |v| {
        tx_rack(v, &value.dynamic)
    })?;
    tx_message(sink, schema::session::track::SIMD2, |v| {
        tx_rack(v, &value.simd2)
    })?;
    tx_message(sink, schema::session::track::FADER, |v| {
        tx_fader(v, &value.fader)
    })?;
    tx_message(sink, schema::session::track::MATRIX_OR_PAN, |v| {
        tx_matrix_or_pan(v, &value.matrix_or_pan)
    })
}
fn tx_submix(sink: &mut dyn Sink, value: &Submix) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::submix::SPEC.field_count(&[])?)?;
    tx_id(sink, schema::session::submix::ID, &value.id)
}
fn tx_output(sink: &mut dyn Sink, value: &Output) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::output::SPEC.field_count(&[])?)?;
    tx_id(sink, schema::session::output::ID, &value.id)
}
fn tx_channel_matrix(sink: &mut dyn Sink, value: &ChannelMatrix) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::channel_matrix::SPEC.field_count(&[])?,
    )?;
    tx_f32(sink, schema::session::channel_matrix::LL, value.ll)?;
    tx_f32(sink, schema::session::channel_matrix::LR, value.lr)?;
    tx_f32(sink, schema::session::channel_matrix::RL, value.rl)?;
    tx_f32(sink, schema::session::channel_matrix::RR, value.rr)
}
fn tx_route(sink: &mut dyn Sink, value: &Route) -> Result<(), EncodeError> {
    tx_start_message(sink, schema::session::route::SPEC.field_count(&[])?)?;
    tx_id(sink, schema::session::route::ID, &value.id)?;
    tx_message(sink, schema::session::route::SOURCE, |v| {
        tx_route_source(v, &value.source)
    })?;
    tx_message(sink, schema::session::route::DESTINATION, |v| {
        tx_route_destination(v, &value.destination)
    })?;
    tx_message(sink, schema::session::route::CHANNEL_MATRIX, |v| {
        tx_channel_matrix(v, &value.channel_matrix)
    })?;
    tx_f32(sink, schema::session::route::GAIN_DB, value.gain_db)
}
fn tx_automation_target(sink: &mut dyn Sink, value: &AutomationTarget) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::automation_target::SPEC.field_count(&[])?,
    )?;
    tx_id(
        sink,
        schema::session::automation_target::ENTITY_ID,
        &value.entity_id,
    )?;
    tx_u8(
        sink,
        schema::session::automation_target::RACK,
        schema::parameter_rack_wire(value.rack),
    )?;
    tx_id(
        sink,
        schema::session::automation_target::EFFECT_ID,
        &value.effect_id,
    )?;
    tx_u32(
        sink,
        schema::session::automation_target::PARAMETER_ID,
        value.parameter_id,
    )?;
    tx_u8(
        sink,
        schema::session::automation_target::CHANNEL,
        schema::parameter_channel_wire(value.channel),
    )
}
fn tx_automation_segment(
    sink: &mut dyn Sink,
    value: &AutomationSegment,
) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::automation_segment::SPEC.field_count(&[])?,
    )?;
    tx_u8(
        sink,
        schema::session::automation_segment::SHAPE,
        enum_shape(value.shape),
    )?;
    tx_u64(
        sink,
        schema::session::automation_segment::START_SAMPLE,
        value.start_sample,
    )?;
    tx_u64(
        sink,
        schema::session::automation_segment::END_SAMPLE,
        value.end_sample,
    )?;
    tx_f32(
        sink,
        schema::session::automation_segment::START_VALUE,
        value.start_value,
    )?;
    tx_f32(
        sink,
        schema::session::automation_segment::END_VALUE,
        value.end_value,
    )?;
    tx_u8(
        sink,
        schema::session::automation_segment::UNIT,
        schema::parameter_unit_wire(value.unit),
    )
}
fn tx_automation(sink: &mut dyn Sink, value: &Automation) -> Result<(), EncodeError> {
    tx_start_message(
        sink,
        schema::session::automation::SPEC
            .field_count(&[(schema::session::automation::SEGMENT, value.segments.len())])?,
    )?;
    tx_id(sink, schema::session::automation::ID, &value.id)?;
    tx_message(sink, schema::session::automation::TARGET, |v| {
        tx_automation_target(v, &value.target)
    })?;
    for segment in &value.segments {
        tx_message(sink, schema::session::automation::SEGMENT, |v| {
            tx_automation_segment(v, segment)
        })?;
    }
    Ok(())
}

fn parse_edit(message: Message<'_>) -> Result<SessionEditV1, DecodeError> {
    let message = message.schema_spec(&schema::session::edit::SPEC)?;
    let opcode = crate::SessionEditOpcode::from_raw(read_u16_exact(one_spec!(
        message,
        schema::session::edit::OPCODE
    )?)?)
    .ok_or(DecodeError::InvalidTlv)?;
    let payload = message
        .nested_value(one_spec!(message, schema::session::edit::PAYLOAD)?)?
        .schema_spec(schema::session::payload_spec(opcode))?;
    let fields = schema::session::payload_spec(opcode).fields;
    match opcode {
        crate::SessionEditOpcode::SetSessionId => Ok(SessionEditV1::SetSessionId {
            session_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetSampleRateHz => Ok(SessionEditV1::SetSampleRateHz {
            sample_rate_hz: read_u32_exact(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetQuantumFrames => Ok(SessionEditV1::SetQuantumFrames {
            quantum_frames: read_u32_exact(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetRenderProfile => Ok(SessionEditV1::SetRenderProfile {
            render_profile: parse_render_profile(
                payload.nested_value(one_spec!(payload, fields[0])?)?,
            )?,
        }),
        crate::SessionEditOpcode::SetOutputProfile => Ok(SessionEditV1::SetOutputProfile {
            output_profile: parse_output_profile(
                payload.nested_value(one_spec!(payload, fields[0])?)?,
            )?,
        }),
        crate::SessionEditOpcode::SetLimits => Ok(SessionEditV1::SetLimits {
            limits: parse_limits(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::UpsertSource => Ok(SessionEditV1::UpsertSource {
            source: parse_source(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveSource => Ok(SessionEditV1::RemoveSource {
            source_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetSourceSampleRateHz => {
            Ok(SessionEditV1::SetSourceSampleRateHz {
                source_id: stable_id(one_spec!(payload, fields[0])?)?,
                sample_rate_hz: read_u32_exact(one_spec!(payload, fields[1])?)?,
            })
        }
        crate::SessionEditOpcode::SetSourceContent => Ok(SessionEditV1::SetSourceContent {
            source_id: stable_id(one_spec!(payload, fields[0])?)?,
            content: parse_content(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::SetSourceMapping => Ok(SessionEditV1::SetSourceMapping {
            source_id: stable_id(one_spec!(payload, fields[0])?)?,
            mapping: parse_mapping(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::UpsertTrack => Ok(SessionEditV1::UpsertTrack {
            track: parse_track(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveTrack => Ok(SessionEditV1::RemoveTrack {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetTrackSourceAssignment => {
            Ok(SessionEditV1::SetTrackSourceAssignment {
                track_id: stable_id(one_spec!(payload, fields[0])?)?,
                source_id: stable_id(one_spec!(payload, fields[1])?)?,
                left_source_channel: read_u8_exact(one_spec!(payload, fields[2])?)?,
                right_source_channel: read_u8_exact(one_spec!(payload, fields[3])?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackBuiltins => Ok(SessionEditV1::SetTrackBuiltins {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            builtins: parse_builtins(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::SetTrackRack => Ok(SessionEditV1::SetTrackRack {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            rack_name: schema::parameter_rack_from_wire(read_u8_exact(one_spec!(
                payload, fields[1]
            )?)?)?,
            rack: parse_rack_message(payload.nested_value(one_spec!(payload, fields[2])?)?)?,
        }),
        crate::SessionEditOpcode::PutTrackEffect => Ok(SessionEditV1::PutTrackEffect {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            rack_name: schema::parameter_rack_from_wire(read_u8_exact(one_spec!(
                payload, fields[1]
            )?)?)?,
            final_position: read_u32_exact(one_spec!(payload, fields[2])?)?,
            effect: parse_effect(payload.nested_value(one_spec!(payload, fields[3])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveTrackEffect => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::RemoveTrackEffect {
                track_id,
                rack_name,
                effect_id,
            })
        }
        crate::SessionEditOpcode::SetTrackEffectOrder => Ok(SessionEditV1::SetTrackEffectOrder {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            rack_name: schema::parameter_rack_from_wire(read_u8_exact(one_spec!(
                payload, fields[1]
            )?)?)?,
            effect_ids: values_spec!(payload, fields[2])?
                .map(stable_id)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        crate::SessionEditOpcode::SetEffectIdentity => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::SetEffectIdentity {
                track_id,
                rack_name,
                effect_id,
                identity: parse_identity(payload.nested_value(one_spec!(payload, fields[3])?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectQuality => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::SetEffectQuality {
                track_id,
                rack_name,
                effect_id,
                quality: parse_quality(read_u8_exact(one_spec!(payload, fields[3])?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectBypass => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::SetEffectBypass {
                track_id,
                rack_name,
                effect_id,
                bypass: parse_bool(one_spec!(payload, fields[3])?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectLinkMode => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::SetEffectLinkMode {
                track_id,
                rack_name,
                effect_id,
                link_mode: parse_link(read_u8_exact(one_spec!(payload, fields[3])?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectSidechain => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::SetEffectSidechain {
                track_id,
                rack_name,
                effect_id,
                sidechain: parse_sidechain(payload.nested_value(one_spec!(payload, fields[3])?)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertEffectParam => {
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload, fields)?;
            Ok(SessionEditV1::UpsertEffectParam {
                track_id,
                rack_name,
                effect_id,
                param: parse_param(payload.nested_value(one_spec!(payload, fields[3])?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveEffectParam => Ok(SessionEditV1::RemoveEffectParam {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            rack_name: schema::parameter_rack_from_wire(read_u8_exact(one_spec!(
                payload, fields[1]
            )?)?)?,
            effect_id: stable_id(one_spec!(payload, fields[2])?)?,
            parameter_id: read_u32_exact(one_spec!(payload, fields[3])?)?,
            channel: schema::parameter_channel_from_wire(read_u8_exact(one_spec!(
                payload, fields[4]
            )?)?)?,
        }),
        crate::SessionEditOpcode::SetTrackFader => Ok(SessionEditV1::SetTrackFader {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            fader: parse_fader(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::SetTrackMatrixOrPan => Ok(SessionEditV1::SetTrackMatrixOrPan {
            track_id: stable_id(one_spec!(payload, fields[0])?)?,
            matrix_or_pan: parse_matrix_or_pan(
                payload.nested_value(one_spec!(payload, fields[1])?)?,
            )?,
        }),
        crate::SessionEditOpcode::UpsertSubmix => Ok(SessionEditV1::UpsertSubmix {
            submix: parse_submix(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveSubmix => Ok(SessionEditV1::RemoveSubmix {
            submix_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::UpsertOutput => Ok(SessionEditV1::UpsertOutput {
            output: parse_output(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveOutput => Ok(SessionEditV1::RemoveOutput {
            output_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::UpsertRoute => Ok(SessionEditV1::UpsertRoute {
            route: parse_route(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveRoute => Ok(SessionEditV1::RemoveRoute {
            route_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetRouteSource => Ok(SessionEditV1::SetRouteSource {
            route_id: stable_id(one_spec!(payload, fields[0])?)?,
            source: parse_route_source(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::SetRouteDestination => Ok(SessionEditV1::SetRouteDestination {
            route_id: stable_id(one_spec!(payload, fields[0])?)?,
            destination: parse_route_destination(
                payload.nested_value(one_spec!(payload, fields[1])?)?,
            )?,
        }),
        crate::SessionEditOpcode::SetRouteChannelMatrix => {
            Ok(SessionEditV1::SetRouteChannelMatrix {
                route_id: stable_id(one_spec!(payload, fields[0])?)?,
                channel_matrix: parse_channel_matrix(
                    payload.nested_value(one_spec!(payload, fields[1])?)?,
                )?,
            })
        }
        crate::SessionEditOpcode::SetRouteGainDb => Ok(SessionEditV1::SetRouteGainDb {
            route_id: stable_id(one_spec!(payload, fields[0])?)?,
            gain_db: read_f32_exact(one_spec!(payload, fields[1])?)?,
        }),
        crate::SessionEditOpcode::UpsertAutomation => Ok(SessionEditV1::UpsertAutomation {
            automation: parse_automation(payload.nested_value(one_spec!(payload, fields[0])?)?)?,
        }),
        crate::SessionEditOpcode::RemoveAutomation => Ok(SessionEditV1::RemoveAutomation {
            automation_id: stable_id(one_spec!(payload, fields[0])?)?,
        }),
        crate::SessionEditOpcode::SetAutomationTarget => Ok(SessionEditV1::SetAutomationTarget {
            automation_id: stable_id(one_spec!(payload, fields[0])?)?,
            target: parse_automation_target(payload.nested_value(one_spec!(payload, fields[1])?)?)?,
        }),
        crate::SessionEditOpcode::SetAutomationSegments => {
            Ok(SessionEditV1::SetAutomationSegments {
                automation_id: stable_id(one_spec!(payload, fields[0])?)?,
                segments: values_spec!(payload, fields[1])?
                    .map(|value| parse_automation_segment(payload.nested_value(value)?))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
    }
}

fn parse_render_profile(message: Message<'_>) -> Result<RenderProfile, DecodeError> {
    let message = message.schema_spec(&schema::session::render_profile::SPEC)?;
    let mode = match read_u8_exact(one_spec!(message, schema::session::render_profile::MODE)?)? {
        1 => miso_engine_session::RenderMode::SingleThread,
        2 => miso_engine_session::RenderMode::DependencyWaves,
        _ => return Err(DecodeError::InvalidTlv),
    };
    Ok(RenderProfile {
        id: stable_id(one_spec!(message, schema::session::render_profile::ID)?)?,
        mode,
    })
}

fn parse_output_profile(message: Message<'_>) -> Result<OutputProfile, DecodeError> {
    let message = message.schema_spec(&schema::session::output_profile::SPEC)?;
    if read_u8_exact(one_spec!(message, schema::session::output_profile::LAYOUT)?)? != 1 {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(OutputProfile {
        id: stable_id(one_spec!(message, schema::session::output_profile::ID)?)?,
        channels: read_u8_exact(one_spec!(
            message,
            schema::session::output_profile::CHANNELS
        )?)?,
        sample_format: miso_engine_session::SampleFormat::F32Planar,
    })
}

fn parse_limits(message: Message<'_>) -> Result<SessionLimits, DecodeError> {
    let message = message.schema_spec(&schema::session::limits::SPEC)?;
    Ok(SessionLimits {
        pcm_ring_frames: read_u64_exact(one_spec!(
            message,
            schema::session::limits::PCM_RING_FRAMES
        )?)?,
        control_queue_messages: read_u64_exact(one_spec!(
            message,
            schema::session::limits::CONTROL_QUEUE_MESSAGES
        )?)?,
        memory_bytes: read_u64_exact(one_spec!(message, schema::session::limits::MEMORY_BYTES)?)?,
    })
}

fn parse_content(message: Message<'_>) -> Result<SourceContent, DecodeError> {
    let message = message.schema_spec(&schema::session::content::SPEC)?;
    Ok(SourceContent {
        identity: utf8(one_spec!(message, schema::session::content::IDENTITY)?)?,
        locator: utf8(one_spec!(message, schema::session::content::LOCATOR)?)?,
    })
}

fn parse_region(message: Message<'_>) -> Result<SourceRegion, DecodeError> {
    let message = message.schema_spec(&schema::session::region::SPEC)?;
    Ok(SourceRegion {
        start_sample: read_u64_exact(one_spec!(message, schema::session::region::START_SAMPLE)?)?,
        length_samples: read_u64_exact(one_spec!(
            message,
            schema::session::region::LENGTH_SAMPLES
        )?)?,
    })
}

fn parse_mapping(message: Message<'_>) -> Result<SourceMapping, DecodeError> {
    let message = message.schema_spec(&schema::session::mapping::SPEC)?;
    Ok(SourceMapping {
        channel_count: read_u8_exact(one_spec!(message, schema::session::mapping::CHANNEL_COUNT)?)?,
        region: parse_region(
            message.nested_value(one_spec!(message, schema::session::mapping::REGION)?)?,
        )?,
    })
}

fn parse_source(message: Message<'_>) -> Result<Source, DecodeError> {
    let message = message.schema_spec(&schema::session::source::SPEC)?;
    Ok(Source {
        id: stable_id(one_spec!(message, schema::session::source::ID)?)?,
        sample_rate_hz: read_u32_exact(one_spec!(
            message,
            schema::session::source::SAMPLE_RATE_HZ
        )?)?,
        content: parse_content(
            message.nested_value(one_spec!(message, schema::session::source::CONTENT)?)?,
        )?,
        mapping: parse_mapping(
            message.nested_value(one_spec!(message, schema::session::source::MAPPING)?)?,
        )?,
    })
}

fn parse_builtins(message: Message<'_>) -> Result<DualMonoBuiltins, DecodeError> {
    let message = message.schema_spec(&schema::session::builtins::SPEC)?;
    Ok(DualMonoBuiltins {
        left: parse_channel_builtins(
            message.nested_value(one_spec!(message, schema::session::builtins::LEFT)?)?,
        )?,
        right: parse_channel_builtins(
            message.nested_value(one_spec!(message, schema::session::builtins::RIGHT)?)?,
        )?,
    })
}
fn parse_channel_builtins(message: Message<'_>) -> Result<ChannelBuiltins, DecodeError> {
    let message = message.schema_spec(&schema::session::channel_builtins::SPEC)?;
    Ok(ChannelBuiltins {
        polarity_invert: parse_bool(one_spec!(
            message,
            schema::session::channel_builtins::POLARITY_INVERT
        )?)?,
        trim_db: read_f32_exact(one_spec!(
            message,
            schema::session::channel_builtins::TRIM_DB
        )?)?,
        hpf_hz: read_f32_exact(one_spec!(
            message,
            schema::session::channel_builtins::HPF_HZ
        )?)?,
        lpf_hz: read_f32_exact(one_spec!(
            message,
            schema::session::channel_builtins::LPF_HZ
        )?)?,
    })
}
fn parse_track(message: Message<'_>) -> Result<miso_engine_session::Track, DecodeError> {
    let message = message.schema_spec(&schema::session::track::SPEC)?;
    Ok(miso_engine_session::Track {
        id: stable_id(one_spec!(message, schema::session::track::ID)?)?,
        source_id: stable_id(one_spec!(message, schema::session::track::SOURCE_ID)?)?,
        left_source_channel: read_u8_exact(one_spec!(
            message,
            schema::session::track::LEFT_SOURCE_CHANNEL
        )?)?,
        right_source_channel: read_u8_exact(one_spec!(
            message,
            schema::session::track::RIGHT_SOURCE_CHANNEL
        )?)?,
        builtins: parse_builtins(
            message.nested_value(one_spec!(message, schema::session::track::BUILTINS)?)?,
        )?,
        simd1: parse_rack_message(
            message.nested_value(one_spec!(message, schema::session::track::SIMD1)?)?,
        )?,
        dynamic: parse_rack_message(
            message.nested_value(one_spec!(message, schema::session::track::DYNAMIC)?)?,
        )?,
        simd2: parse_rack_message(
            message.nested_value(one_spec!(message, schema::session::track::SIMD2)?)?,
        )?,
        fader: parse_fader(
            message.nested_value(one_spec!(message, schema::session::track::FADER)?)?,
        )?,
        matrix_or_pan: parse_matrix_or_pan(
            message.nested_value(one_spec!(message, schema::session::track::MATRIX_OR_PAN)?)?,
        )?,
    })
}
fn parse_rack_message(message: Message<'_>) -> Result<Rack, DecodeError> {
    let message = message.schema_spec(&schema::session::rack::SPEC)?;
    Ok(Rack {
        effects: values_spec!(message, schema::session::rack::EFFECT)?
            .map(|value| parse_effect(message.nested_value(value)?))
            .collect::<Result<Vec<_>, _>>()?,
    })
}
fn parse_identity(message: Message<'_>) -> Result<EffectIdentity, DecodeError> {
    let message = message.schema_spec(&schema::session::effect_identity::SPEC)?;
    match read_u8_exact(one_spec!(message, schema::session::effect_identity::TAG)?)? {
        1 => Ok(EffectIdentity::Native {
            effect_id: stable_id(one_spec!(message, schema::session::effect_identity::VALUE)?)?,
        }),
        2 => Ok(EffectIdentity::ThirdPartyCid {
            cid: utf8(one_spec!(message, schema::session::effect_identity::VALUE)?)?,
        }),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_route_source(message: Message<'_>) -> Result<RouteSource, DecodeError> {
    let (tag, message) = message.tagged_schema(
        schema::session::route_source::TAG,
        &[
            (1, &schema::session::route_source::TRACK),
            (2, &schema::session::route_source::SUBMIX),
        ],
        &schema::session::route_source::KNOWN,
    )?;
    match tag {
        1 => Ok(RouteSource::Track {
            track_id: stable_id(one_spec!(message, schema::session::route_source::ID)?)?,
            tap: parse_tap(read_u8_exact(one_spec!(
                message,
                schema::session::route_source::TAP
            )?)?)?,
        }),
        2 => Ok(RouteSource::SubmixOutput {
            submix_id: stable_id(one_spec!(message, schema::session::route_source::ID)?)?,
        }),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_route_destination(message: Message<'_>) -> Result<RouteDestination, DecodeError> {
    let message = message.schema_spec(&schema::session::route_destination::SPEC)?;
    match read_u8_exact(one_spec!(message, schema::session::route_destination::TAG)?)? {
        1 => Ok(RouteDestination::SubmixInput {
            submix_id: stable_id(one_spec!(message, schema::session::route_destination::ID)?)?,
        }),
        2 => Ok(RouteDestination::OutputInput {
            output_id: stable_id(one_spec!(message, schema::session::route_destination::ID)?)?,
        }),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_sidechain(message: Message<'_>) -> Result<SidechainDeclaration, DecodeError> {
    let (tag, message) = message.tagged_schema(
        schema::session::sidechain::TAG,
        &[
            (1, &schema::session::sidechain::NONE),
            (2, &schema::session::sidechain::ROUTED),
        ],
        &schema::session::sidechain::KNOWN,
    )?;
    match tag {
        1 => Ok(SidechainDeclaration::None),
        2 => Ok(SidechainDeclaration::Routed(
            miso_engine_session::Sidechain {
                source: parse_route_source(
                    message
                        .nested_value(one_spec!(message, schema::session::sidechain::SOURCE)?)?,
                )?,
                port_id: stable_id(one_spec!(message, schema::session::sidechain::PORT_ID)?)?,
            },
        )),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_param(message: Message<'_>) -> Result<EffectParam, DecodeError> {
    let message = message.schema_spec(&schema::session::param::SPEC)?;
    Ok(EffectParam {
        parameter_id: read_u32_exact(one_spec!(message, schema::session::param::PARAMETER_ID)?)?,
        channel: schema::parameter_channel_from_wire(read_u8_exact(one_spec!(
            message,
            schema::session::param::CHANNEL
        )?)?)?,
        unit: schema::parameter_unit_from_wire(read_u8_exact(one_spec!(
            message,
            schema::session::param::UNIT
        )?)?)?,
        value: read_f32_exact(one_spec!(message, schema::session::param::VALUE)?)?,
    })
}
fn parse_effect(message: Message<'_>) -> Result<Effect, DecodeError> {
    let message = message.schema_spec(&schema::session::effect::SPEC)?;
    Ok(Effect {
        id: stable_id(one_spec!(message, schema::session::effect::ID)?)?,
        identity: parse_identity(
            message.nested_value(one_spec!(message, schema::session::effect::IDENTITY)?)?,
        )?,
        quality: parse_quality(read_u8_exact(one_spec!(
            message,
            schema::session::effect::QUALITY
        )?)?)?,
        bypass: parse_bool(one_spec!(message, schema::session::effect::BYPASS)?)?,
        link_mode: parse_link(read_u8_exact(one_spec!(
            message,
            schema::session::effect::LINK_MODE
        )?)?)?,
        params: values_spec!(message, schema::session::effect::PARAM)?
            .map(|value| parse_param(message.nested_value(value)?))
            .collect::<Result<Vec<_>, _>>()?,
        sidechain: parse_sidechain(
            message.nested_value(one_spec!(message, schema::session::effect::SIDECHAIN)?)?,
        )?,
    })
}
fn parse_fader(message: Message<'_>) -> Result<DualMonoFader, DecodeError> {
    let message = message.schema_spec(&schema::session::fader::SPEC)?;
    Ok(DualMonoFader {
        left_db: read_f32_exact(one_spec!(message, schema::session::fader::LEFT_DB)?)?,
        right_db: read_f32_exact(one_spec!(message, schema::session::fader::RIGHT_DB)?)?,
        left_mute: parse_bool(one_spec!(message, schema::session::fader::LEFT_MUTE)?)?,
        right_mute: parse_bool(one_spec!(message, schema::session::fader::RIGHT_MUTE)?)?,
    })
}
fn parse_matrix_or_pan(message: Message<'_>) -> Result<MatrixOrPan, DecodeError> {
    let (tag, message) = message.tagged_schema(
        schema::session::matrix_or_pan::TAG,
        &[
            (1, &schema::session::matrix_or_pan::PAN),
            (2, &schema::session::matrix_or_pan::MATRIX),
        ],
        &schema::session::matrix_or_pan::KNOWN,
    )?;
    match tag {
        1 => Ok(MatrixOrPan::Pan {
            left: read_f32_exact(one_spec!(message, schema::session::matrix_or_pan::A)?)?,
            right: read_f32_exact(one_spec!(message, schema::session::matrix_or_pan::B)?)?,
            smoothing_samples: read_u32_exact(one_spec!(
                message,
                schema::session::matrix_or_pan::PAN_SMOOTHING
            )?)?,
        }),
        2 => Ok(MatrixOrPan::Matrix {
            ll: read_f32_exact(one_spec!(message, schema::session::matrix_or_pan::A)?)?,
            lr: read_f32_exact(one_spec!(message, schema::session::matrix_or_pan::B)?)?,
            rl: read_f32_exact(one_spec!(
                message,
                schema::session::matrix_or_pan::C_OR_SMOOTHING
            )?)?,
            rr: read_f32_exact(one_spec!(message, schema::session::matrix_or_pan::D)?)?,
            smoothing_samples: read_u32_exact(one_spec!(
                message,
                schema::session::matrix_or_pan::SMOOTHING
            )?)?,
        }),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_submix(message: Message<'_>) -> Result<Submix, DecodeError> {
    let message = message.schema_spec(&schema::session::submix::SPEC)?;
    Ok(Submix {
        id: stable_id(one_spec!(message, schema::session::submix::ID)?)?,
    })
}
fn parse_output(message: Message<'_>) -> Result<Output, DecodeError> {
    let message = message.schema_spec(&schema::session::output::SPEC)?;
    Ok(Output {
        id: stable_id(one_spec!(message, schema::session::output::ID)?)?,
    })
}
fn parse_channel_matrix(message: Message<'_>) -> Result<ChannelMatrix, DecodeError> {
    let message = message.schema_spec(&schema::session::channel_matrix::SPEC)?;
    Ok(ChannelMatrix {
        ll: read_f32_exact(one_spec!(message, schema::session::channel_matrix::LL)?)?,
        lr: read_f32_exact(one_spec!(message, schema::session::channel_matrix::LR)?)?,
        rl: read_f32_exact(one_spec!(message, schema::session::channel_matrix::RL)?)?,
        rr: read_f32_exact(one_spec!(message, schema::session::channel_matrix::RR)?)?,
    })
}
fn parse_route(message: Message<'_>) -> Result<Route, DecodeError> {
    let message = message.schema_spec(&schema::session::route::SPEC)?;
    Ok(Route {
        id: stable_id(one_spec!(message, schema::session::route::ID)?)?,
        source: parse_route_source(
            message.nested_value(one_spec!(message, schema::session::route::SOURCE)?)?,
        )?,
        destination: parse_route_destination(
            message.nested_value(one_spec!(message, schema::session::route::DESTINATION)?)?,
        )?,
        channel_matrix: parse_channel_matrix(
            message.nested_value(one_spec!(message, schema::session::route::CHANNEL_MATRIX)?)?,
        )?,
        gain_db: read_f32_exact(one_spec!(message, schema::session::route::GAIN_DB)?)?,
    })
}
fn parse_automation_target(message: Message<'_>) -> Result<AutomationTarget, DecodeError> {
    let message = message.schema_spec(&schema::session::automation_target::SPEC)?;
    Ok(AutomationTarget {
        entity_id: stable_id(one_spec!(
            message,
            schema::session::automation_target::ENTITY_ID
        )?)?,
        rack: schema::parameter_rack_from_wire(read_u8_exact(one_spec!(
            message,
            schema::session::automation_target::RACK
        )?)?)?,
        effect_id: stable_id(one_spec!(
            message,
            schema::session::automation_target::EFFECT_ID
        )?)?,
        parameter_id: read_u32_exact(one_spec!(
            message,
            schema::session::automation_target::PARAMETER_ID
        )?)?,
        channel: schema::parameter_channel_from_wire(read_u8_exact(one_spec!(
            message,
            schema::session::automation_target::CHANNEL
        )?)?)?,
    })
}
fn parse_automation_segment(message: Message<'_>) -> Result<AutomationSegment, DecodeError> {
    let message = message.schema_spec(&schema::session::automation_segment::SPEC)?;
    Ok(AutomationSegment {
        shape: parse_shape(read_u8_exact(one_spec!(
            message,
            schema::session::automation_segment::SHAPE
        )?)?)?,
        start_sample: read_u64_exact(one_spec!(
            message,
            schema::session::automation_segment::START_SAMPLE
        )?)?,
        end_sample: read_u64_exact(one_spec!(
            message,
            schema::session::automation_segment::END_SAMPLE
        )?)?,
        start_value: read_f32_exact(one_spec!(
            message,
            schema::session::automation_segment::START_VALUE
        )?)?,
        end_value: read_f32_exact(one_spec!(
            message,
            schema::session::automation_segment::END_VALUE
        )?)?,
        unit: schema::parameter_unit_from_wire(read_u8_exact(one_spec!(
            message,
            schema::session::automation_segment::UNIT
        )?)?)?,
    })
}
fn parse_automation(message: Message<'_>) -> Result<Automation, DecodeError> {
    let message = message.schema_spec(&schema::session::automation::SPEC)?;
    Ok(Automation {
        id: stable_id(one_spec!(message, schema::session::automation::ID)?)?,
        target: parse_automation_target(
            message.nested_value(one_spec!(message, schema::session::automation::TARGET)?)?,
        )?,
        segments: values_spec!(message, schema::session::automation::SEGMENT)?
            .map(|value| parse_automation_segment(message.nested_value(value)?))
            .collect::<Result<Vec<_>, _>>()?,
    })
}
fn parse_track_effect_ref(
    message: &Message<'_>,
    fields: &[FieldSpec],
) -> Result<(StableId, RackName, StableId), DecodeError> {
    Ok((
        stable_id(one_spec!(message, fields[0])?)?,
        schema::parameter_rack_from_wire(read_u8_exact(one_spec!(message, fields[1])?)?)?,
        stable_id(one_spec!(message, fields[2])?)?,
    ))
}
fn parse_quality(value: u8) -> Result<EffectQuality, DecodeError> {
    match value {
        1 => Ok(EffectQuality::Draft),
        2 => Ok(EffectQuality::Normal),
        3 => Ok(EffectQuality::High),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_link(value: u8) -> Result<miso_engine_session::LinkMode, DecodeError> {
    match value {
        1 => Ok(miso_engine_session::LinkMode::DualMono),
        2 => Ok(miso_engine_session::LinkMode::Maximum),
        3 => Ok(miso_engine_session::LinkMode::Average),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_tap(value: u8) -> Result<miso_engine_session::SendTap, DecodeError> {
    match value {
        1 => Ok(miso_engine_session::SendTap::Input),
        2 => Ok(miso_engine_session::SendTap::PostInputBuiltins),
        3 => Ok(miso_engine_session::SendTap::PostSimd1),
        4 => Ok(miso_engine_session::SendTap::PostDynamic),
        5 => Ok(miso_engine_session::SendTap::PostSimd2PreFader),
        6 => Ok(miso_engine_session::SendTap::PostFader),
        7 => Ok(miso_engine_session::SendTap::PostMatrix),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_shape(value: u8) -> Result<AutomationShape, DecodeError> {
    match value {
        1 => Ok(AutomationShape::Step),
        2 => Ok(AutomationShape::Linear),
        3 => Ok(AutomationShape::Exponential),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn utf8(bytes: &[u8]) -> Result<String, DecodeError> {
    core::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| DecodeError::InvalidUtf8)
}
fn stable_id(bytes: &[u8]) -> Result<StableId, DecodeError> {
    StableId::parse(&utf8(bytes)?).ok_or(DecodeError::InvalidTlv)
}
fn read_f32_exact(bytes: &[u8]) -> Result<f32, DecodeError> {
    let value = read_f32(bytes)?;
    if !value.is_finite() {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(value)
}
fn parse_bool(bytes: &[u8]) -> Result<bool, DecodeError> {
    match read_u8_exact(bytes)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btlv::{WIRE_BOOL, WIRE_F32, WIRE_U8, WIRE_U32, WIRE_UTF8};
    use miso_engine_session::{
        LinkMode, ParameterChannel, ParameterUnit, RenderMode, SampleFormat, SendTap, Sidechain,
        Track, parse_session_toml,
    };

    fn id(value: &str) -> StableId {
        StableId::parse(value).expect("stable ID")
    }

    fn raw_message(fields: Vec<(u16, u8, bool, Vec<u8>)>) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(fields.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        for (id, wire, mandatory, value) in fields {
            bytes.extend_from_slice(&id.to_le_bytes());
            bytes.push(wire);
            bytes.push(u8::from(mandatory));
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&value);
            bytes.resize(bytes.len() + crate::btlv::padding(value.len()), 0);
        }
        bytes
    }

    #[test]
    fn exact_set_session_id_golden_and_no_partial_output() {
        let codec = ProtocolCodec::default();
        let edits = [SessionEditV1::SetSessionId {
            session_id: id("next"),
        }];
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(1).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let required = codec
            .encoded_session_transaction_len(&transaction)
            .expect("length");
        assert_eq!(required, 112);
        let mut short = vec![0xaa; required - 1];
        assert_eq!(
            codec.encode_session_transaction(&transaction, &mut short),
            Err(EncodeError::OutputTooSmall { required })
        );
        assert_eq!(short, vec![0xaa; required - 1]);
        let mut output = vec![0; required];
        codec
            .encode_session_transaction(&transaction, &mut output)
            .expect("encode");
        assert_eq!(
            hex(&output),
            concat!(
                "4d49534f43544c0001000000300001000300000040000000",
                "010000000000000007000000000000000100000000000000",
                "01000b013800000002000000000000000100020102000000",
                "010000000000000002000b01180000000100000000000000",
                "01000901040000006e65787400000000"
            )
        );
        let mut slots = [0_u16; 1];
        assert!(
            codec
                .decode_session_transaction_outer(&output, &mut DecodeScratch::new(&mut slots))
                .is_ok()
        );
    }

    #[test]
    fn six_session_edit_encoders_are_canonical_and_ordered() {
        let edits = [
            SessionEditV1::SetSessionId {
                session_id: id("next"),
            },
            SessionEditV1::SetSampleRateHz {
                sample_rate_hz: 48_000,
            },
            SessionEditV1::SetQuantumFrames {
                quantum_frames: 128,
            },
            SessionEditV1::SetRenderProfile {
                render_profile: RenderProfile {
                    id: id("render"),
                    mode: RenderMode::SingleThread,
                },
            },
            SessionEditV1::SetOutputProfile {
                output_profile: OutputProfile {
                    id: id("output"),
                    channels: 2,
                    sample_format: SampleFormat::F32Planar,
                },
            },
            SessionEditV1::SetLimits {
                limits: SessionLimits {
                    pcm_ring_frames: 64,
                    control_queue_messages: 8,
                    memory_bytes: 1024,
                },
            },
        ];
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(2).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let required = codec
            .encoded_session_transaction_len(&transaction)
            .expect("length");
        let mut output = vec![0; required];
        assert_eq!(
            codec.encode_session_transaction(&transaction, &mut output),
            Ok(required)
        );
        assert_eq!(
            u32::from_le_bytes(output[40..44].try_into().expect("count")),
            6
        );
        assert!(
            output
                .windows(2)
                .any(|window| window == 0x0006_u16.to_le_bytes())
        );
    }

    #[test]
    fn transaction_outer_header_uses_sizing_sink_repeated_count() {
        let codec = ProtocolCodec::default();
        let edits = [
            SessionEditV1::SetSessionId {
                session_id: id("measured"),
            },
            SessionEditV1::SetSampleRateHz {
                sample_rate_hz: 48_000,
            },
            SessionEditV1::SetQuantumFrames {
                quantum_frames: 128,
            },
        ];
        for count in 1..=edits.len() {
            let transaction = SessionTransactionFrame {
                request_id: RequestId::new(3).expect("request"),
                expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
                edits: &edits[..count],
            };
            let required = codec
                .encoded_session_transaction_len(&transaction)
                .expect("measured length");
            let mut output = vec![0; required];
            assert_eq!(
                codec.encode_session_transaction(&transaction, &mut output),
                Ok(required)
            );
            assert_eq!(
                u32::from_le_bytes(output[40..44].try_into().expect("outer TLV count")),
                u32::try_from(count).expect("small count")
            );
        }
    }

    fn source() -> Source {
        Source {
            id: id("voice"),
            sample_rate_hz: 48_000,
            content: SourceContent {
                identity: "content-voice".to_owned(),
                locator: "host://voice".to_owned(),
            },
            mapping: SourceMapping {
                channel_count: 2,
                region: SourceRegion {
                    start_sample: 4,
                    length_samples: 48_000,
                },
            },
        }
    }

    fn all_opcode_edits_64() -> Vec<SessionEditV1> {
        let session =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("fixture");
        let source = session.sources[0].clone();
        let track = session.tracks[0].clone();
        let effect = track.dynamic.effects[0].clone();
        let route = session.routes[0].clone();
        let automation = session.automation[0].clone();
        let track_id = track.id.clone();
        let effect_id = effect.id.clone();
        let mut edits = vec![
            SessionEditV1::SetSessionId {
                session_id: id("demo.session"),
            },
            SessionEditV1::SetSampleRateHz {
                sample_rate_hz: 48_000,
            },
            SessionEditV1::SetQuantumFrames {
                quantum_frames: 128,
            },
            SessionEditV1::SetRenderProfile {
                render_profile: session.render_profile.clone(),
            },
            SessionEditV1::SetOutputProfile {
                output_profile: session.output_profile.clone(),
            },
            SessionEditV1::SetLimits {
                limits: session.limits.clone(),
            },
            SessionEditV1::UpsertSource {
                source: source.clone(),
            },
            SessionEditV1::RemoveSource {
                source_id: source.id.clone(),
            },
            SessionEditV1::SetSourceSampleRateHz {
                source_id: source.id.clone(),
                sample_rate_hz: 48_000,
            },
            SessionEditV1::SetSourceContent {
                source_id: source.id.clone(),
                content: source.content.clone(),
            },
            SessionEditV1::SetSourceMapping {
                source_id: source.id.clone(),
                mapping: source.mapping.clone(),
            },
            SessionEditV1::UpsertTrack {
                track: track.clone(),
            },
            SessionEditV1::RemoveTrack {
                track_id: track_id.clone(),
            },
            SessionEditV1::SetTrackSourceAssignment {
                track_id: track_id.clone(),
                source_id: source.id.clone(),
                left_source_channel: 0,
                right_source_channel: 1,
            },
            SessionEditV1::SetTrackBuiltins {
                track_id: track_id.clone(),
                builtins: track.builtins.clone(),
            },
            SessionEditV1::SetTrackRack {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                rack: track.dynamic.clone(),
            },
            SessionEditV1::PutTrackEffect {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                final_position: 0,
                effect: effect.clone(),
            },
            SessionEditV1::RemoveTrackEffect {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
            },
            SessionEditV1::SetTrackEffectOrder {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_ids: vec![effect_id.clone()],
            },
            SessionEditV1::SetEffectIdentity {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                identity: effect.identity.clone(),
            },
            SessionEditV1::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                quality: effect.quality,
            },
            SessionEditV1::SetEffectBypass {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                bypass: effect.bypass,
            },
            SessionEditV1::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                link_mode: effect.link_mode,
            },
            SessionEditV1::SetEffectSidechain {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                sidechain: effect.sidechain.clone(),
            },
            SessionEditV1::UpsertEffectParam {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                param: effect.params[0].clone(),
            },
            SessionEditV1::RemoveEffectParam {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: effect_id.clone(),
                parameter_id: effect.params[0].parameter_id,
                channel: effect.params[0].channel,
            },
            SessionEditV1::SetTrackFader {
                track_id: track_id.clone(),
                fader: track.fader.clone(),
            },
            SessionEditV1::SetTrackMatrixOrPan {
                track_id: track_id.clone(),
                matrix_or_pan: track.matrix_or_pan.clone(),
            },
            SessionEditV1::UpsertSubmix {
                submix: Submix { id: id("drums") },
            },
            SessionEditV1::RemoveSubmix {
                submix_id: id("drums"),
            },
            SessionEditV1::UpsertOutput {
                output: Output { id: id("alt-out") },
            },
            SessionEditV1::RemoveOutput {
                output_id: id("alt-out"),
            },
            SessionEditV1::UpsertRoute {
                route: route.clone(),
            },
            SessionEditV1::RemoveRoute {
                route_id: route.id.clone(),
            },
            SessionEditV1::SetRouteSource {
                route_id: route.id.clone(),
                source: route.source.clone(),
            },
            SessionEditV1::SetRouteDestination {
                route_id: route.id.clone(),
                destination: route.destination.clone(),
            },
            SessionEditV1::SetRouteChannelMatrix {
                route_id: route.id.clone(),
                channel_matrix: route.channel_matrix.clone(),
            },
            SessionEditV1::SetRouteGainDb {
                route_id: route.id.clone(),
                gain_db: route.gain_db,
            },
            SessionEditV1::UpsertAutomation {
                automation: automation.clone(),
            },
            SessionEditV1::RemoveAutomation {
                automation_id: automation.id.clone(),
            },
            SessionEditV1::SetAutomationTarget {
                automation_id: automation.id.clone(),
                target: automation.target.clone(),
            },
            SessionEditV1::SetAutomationSegments {
                automation_id: automation.id.clone(),
                segments: automation.segments.clone(),
            },
        ];
        while edits.len() < 64 {
            edits.push(SessionEditV1::SetSessionId {
                session_id: id("demo.session"),
            });
        }
        edits
    }

    fn track() -> Track {
        parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
            .expect("fixture")
            .tracks
            .into_iter()
            .next()
            .expect("track")
    }

    fn effect_with(
        slot: &str,
        identity: EffectIdentity,
        quality: EffectQuality,
        link_mode: LinkMode,
        params: Vec<EffectParam>,
        sidechain: SidechainDeclaration,
    ) -> Effect {
        Effect {
            id: id(slot),
            identity,
            quality,
            bypass: false,
            link_mode,
            params,
            sidechain,
        }
    }

    fn encode(edits: &[SessionEditV1]) -> Vec<u8> {
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(9).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&transaction)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&transaction, &mut bytes)
            .expect("encode");
        bytes
    }

    #[test]
    fn direct_full_schema_encoder_is_byte_identical_in_caller_storage() {
        let edits = all_opcode_edits_64();
        assert_eq!(edits.len(), 64);
        let codec = ProtocolCodec::default();
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(33).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let required = codec
            .encoded_session_transaction_len(&transaction)
            .expect("sizing");
        let mut output = vec![0_u8; required];
        codec
            .encode_session_transaction(&transaction, &mut output)
            .expect("initial direct encode");
        let canonical = output.clone();
        let mut scratch = [0_u16; 64];
        let decoded = codec
            .decode_session_transaction(&output, &mut DecodeScratch::new(&mut scratch))
            .expect("all opcodes decode");
        assert_eq!(decoded.edits, edits);
        for _ in 0..16 {
            assert_eq!(
                codec.encoded_session_transaction_len(&transaction),
                Ok(required)
            );
            output.fill(0);
            assert_eq!(
                codec.encode_session_transaction(&transaction, &mut output),
                Ok(required)
            );
            assert_eq!(output, canonical);
        }
    }

    #[test]
    fn transaction_repeated_fields_match_schema_derived_count() {
        let edits = [SessionEditV1::SetTrackEffectOrder {
            track_id: id("track.repeated"),
            rack_name: RackName::Dynamic,
            effect_ids: vec![id("effect.one"), id("effect.two"), id("effect.three")],
        }];
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(37).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let required = codec
            .encoded_session_transaction_len(&transaction)
            .expect("schema count includes every repeated field");
        let mut output = vec![0_u8; required];
        assert_eq!(
            codec.encode_session_transaction(&transaction, &mut output),
            Ok(required)
        );
        let decoded = codec
            .decode_session_transaction(&output, &mut DecodeScratch::new(&mut [0_u16; 8]))
            .expect("repeated transaction roundtrip");
        assert_eq!(decoded.edits, edits);
    }

    #[test]
    fn transaction_requires_exact_nonempty_edits_and_repeated_message_wire_type() {
        let codec = ProtocolCodec::default();
        let empty = SessionTransactionFrame {
            request_id: RequestId::new(1).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &[],
        };
        assert_eq!(
            codec.encoded_session_transaction_len(&empty),
            Err(EncodeError::MessageKindMismatch)
        );
        let any = SessionTransactionFrame {
            request_id: RequestId::new(1).expect("request"),
            expected_revision: ExpectedRevision::Any,
            edits: &[SessionEditV1::SetSessionId {
                session_id: StableId::parse("exact-required").expect("stable ID"),
            }],
        };
        assert_eq!(
            codec.encoded_session_transaction_len(&any),
            Err(EncodeError::MessageKindMismatch)
        );
        let mut wrong_wire = encode(&[SessionEditV1::SetSessionId {
            session_id: StableId::parse("wrong-wire").expect("stable ID"),
        }]);
        wrong_wire[crate::OUTER_HEADER_BYTES + 2] = WIRE_UTF8;
        assert_eq!(
            codec
                .decode_session_transaction(&wrong_wire, &mut DecodeScratch::new(&mut [0_u16; 16])),
            Err(DecodeError::InvalidTlv)
        );
    }

    #[test]
    fn direct_encoder_limits_and_short_buffers_preserve_caller_ownership() {
        let edits = all_opcode_edits_64();
        let frame = SessionTransactionFrame {
            request_id: RequestId::new(34).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let required = codec
            .encoded_session_transaction_len(&frame)
            .expect("length");
        let mut short = vec![0xa5; required - 1];
        assert_eq!(
            codec.encode_session_transaction(&frame, &mut short),
            Err(EncodeError::OutputTooSmall { required })
        );
        assert!(short.iter().all(|byte| *byte == 0xa5));
        let limited = ProtocolCodec::new(crate::ProtocolLimits {
            max_tlv_count: 1,
            ..crate::ProtocolLimits::default()
        });
        let mut untouched = [0x5a_u8; 64];
        assert_eq!(
            limited.encode_session_transaction(&frame, &mut untouched),
            Err(EncodeError::LimitExceeded)
        );
        assert!(untouched.iter().all(|byte| *byte == 0x5a));
        let frame_limited = ProtocolCodec::new(crate::ProtocolLimits {
            max_frame_bytes: crate::OUTER_HEADER_BYTES,
            ..crate::ProtocolLimits::default()
        });
        assert_eq!(
            frame_limited.encoded_session_transaction_len(&frame),
            Err(EncodeError::LimitExceeded)
        );
        assert_eq!(
            schema::session::rack::SPEC
                .field_count(&[(schema::session::rack::EFFECT, usize::MAX,)]),
            Err(EncodeError::LimitExceeded)
        );
        let mut overflow =
            CountSink::with_length_for_test(usize::MAX, crate::ProtocolLimits::default());
        assert_eq!(overflow.raw(&[0]), Err(EncodeError::LimitExceeded));
    }

    #[test]
    fn transaction_encoder_reserves_envelope_depth_for_frozen_deep_fixture() {
        let flat_edits = [SessionEditV1::SetSessionId {
            session_id: id("depth-envelope"),
        }];
        let flat = SessionTransactionFrame {
            request_id: RequestId::new(35).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &flat_edits,
        };
        let zero_logical_depth = ProtocolCodec::new(crate::ProtocolLimits {
            max_nesting: 0,
            ..crate::ProtocolLimits::default()
        });
        let flat_len = zero_logical_depth
            .encoded_session_transaction_len(&flat)
            .expect("three fixed envelopes do not consume logical nesting");
        let mut flat_bytes = vec![0; flat_len];
        zero_logical_depth
            .encode_session_transaction(&flat, &mut flat_bytes)
            .expect("flat transaction encodes at zero logical depth");
        zero_logical_depth
            .decode(&flat_bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
            .expect("generic decode reserves exactly the fixed envelopes");
        zero_logical_depth
            .decode_session_transaction(&flat_bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
            .expect("typed decode reserves exactly the fixed envelopes");

        let deep_edits = complete_all_opcode_fixture();
        let deep = SessionTransactionFrame {
            request_id: RequestId::new(36).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &deep_edits,
        };
        let codec = ProtocolCodec::default();
        let deep_len = codec
            .encoded_session_transaction_len(&deep)
            .expect("canonical deep transaction remains encodable");
        let mut deep_bytes = vec![0; deep_len];
        codec
            .encode_session_transaction(&deep, &mut deep_bytes)
            .expect("encode canonical deep transaction");
        assert_eq!(
            zero_logical_depth.encoded_session_transaction_len(&deep),
            Err(EncodeError::LimitExceeded)
        );
        assert_eq!(
            zero_logical_depth.decode(&deep_bytes, &mut DecodeScratch::new(&mut [0_u16; 128]),),
            Err(DecodeError::LimitExceeded)
        );
        assert_eq!(
            zero_logical_depth
                .decode_session_transaction(
                    &deep_bytes,
                    &mut DecodeScratch::new(&mut [0_u16; 128]),
                )
                .map(|_| ()),
            Err(DecodeError::LimitExceeded)
        );
    }

    #[test]
    fn transaction_descendants_retain_string_limits() {
        let edits = [SessionEditV1::SetRenderProfile {
            render_profile: RenderProfile {
                id: id("long-render-profile"),
                mode: RenderMode::SingleThread,
            },
        }];
        let frame = SessionTransactionFrame {
            request_id: RequestId::new(37).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("encode");
        let string_limited = ProtocolCodec::new(crate::ProtocolLimits {
            max_string_bytes: 4,
            ..crate::ProtocolLimits::default()
        });
        assert_eq!(
            string_limited
                .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 8]),)
                .map(|_| ()),
            Err(DecodeError::LimitExceeded)
        );
    }

    #[test]
    fn source_variants_decode_and_reencode_canonically() {
        let source = source();
        let edits = [
            SessionEditV1::UpsertSource {
                source: source.clone(),
            },
            SessionEditV1::RemoveSource {
                source_id: source.id.clone(),
            },
            SessionEditV1::SetSourceSampleRateHz {
                source_id: source.id.clone(),
                sample_rate_hz: 44_100,
            },
            SessionEditV1::SetSourceContent {
                source_id: source.id.clone(),
                content: source.content.clone(),
            },
            SessionEditV1::SetSourceMapping {
                source_id: source.id.clone(),
                mapping: source.mapping.clone(),
            },
        ];
        let bytes = encode(&edits);
        let codec = ProtocolCodec::default();
        let mut scratch = [0_u16; 5];
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
            .expect("typed decode");
        assert_eq!(decoded.edits, edits);
        assert_eq!(encode(&decoded.edits), bytes);
    }

    #[test]
    fn every_track_effect_opcode_and_nested_variant_round_trips_canonically() {
        let mut full_track = track();
        let parameters = vec![
            EffectParam {
                parameter_id: 11,
                channel: ParameterChannel::Left,
                unit: ParameterUnit::Db,
                value: -3.0,
            },
            EffectParam {
                parameter_id: 12,
                channel: ParameterChannel::Right,
                unit: ParameterUnit::Hz,
                value: 2_000.0,
            },
            EffectParam {
                parameter_id: 13,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Milliseconds,
                value: 12.0,
            },
            EffectParam {
                parameter_id: 14,
                channel: ParameterChannel::Left,
                unit: ParameterUnit::Samples,
                value: 4.0,
            },
            EffectParam {
                parameter_id: 15,
                channel: ParameterChannel::Right,
                unit: ParameterUnit::Linear,
                value: 0.5,
            },
            EffectParam {
                parameter_id: 16,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Ratio,
                value: 2.0,
            },
        ];
        let native = effect_with(
            "native-fx",
            EffectIdentity::Native {
                effect_id: id("parametric-eq"),
            },
            EffectQuality::Draft,
            LinkMode::DualMono,
            parameters.clone(),
            SidechainDeclaration::None,
        );
        let cid = effect_with(
            "cid-fx",
            EffectIdentity::ThirdPartyCid {
                cid: "bafycid-demo".to_owned(),
            },
            EffectQuality::Normal,
            LinkMode::Maximum,
            Vec::new(),
            SidechainDeclaration::Routed(Sidechain {
                source: RouteSource::Track {
                    track_id: id("vocal"),
                    tap: SendTap::PostFader,
                },
                port_id: id("detector"),
            }),
        );
        let high = effect_with(
            "high-fx",
            EffectIdentity::Native {
                effect_id: id("compressor"),
            },
            EffectQuality::High,
            LinkMode::Average,
            Vec::new(),
            SidechainDeclaration::Routed(Sidechain {
                source: RouteSource::SubmixOutput {
                    submix_id: id("drums"),
                },
                port_id: id("key"),
            }),
        );
        full_track.simd1.effects = vec![native.clone(), cid.clone()];
        full_track.dynamic.effects = vec![cid.clone(), high.clone()];
        full_track.simd2.effects = vec![high.clone(), native.clone()];
        let track_id = full_track.id.clone();
        let source_id = full_track.source_id.clone();
        let mut edits = vec![
            SessionEditV1::UpsertTrack {
                track: full_track.clone(),
            },
            SessionEditV1::RemoveTrack {
                track_id: track_id.clone(),
            },
            SessionEditV1::SetTrackSourceAssignment {
                track_id: track_id.clone(),
                source_id,
                left_source_channel: 1,
                right_source_channel: 0,
            },
            SessionEditV1::SetTrackBuiltins {
                track_id: track_id.clone(),
                builtins: full_track.builtins.clone(),
            },
        ];
        for (rack_name, rack) in [
            (RackName::Simd1, full_track.simd1.clone()),
            (RackName::Dynamic, full_track.dynamic.clone()),
            (RackName::Simd2, full_track.simd2.clone()),
        ] {
            edits.push(SessionEditV1::SetTrackRack {
                track_id: track_id.clone(),
                rack_name,
                rack,
            });
        }
        edits.extend([
            SessionEditV1::PutTrackEffect {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                final_position: 1,
                effect: native.clone(),
            },
            SessionEditV1::RemoveTrackEffect {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: cid.id.clone(),
            },
            SessionEditV1::SetTrackEffectOrder {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_ids: vec![cid.id.clone(), high.id.clone()],
            },
            SessionEditV1::SetEffectIdentity {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: native.id.clone(),
                identity: native.identity.clone(),
            },
            SessionEditV1::SetEffectIdentity {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: cid.id.clone(),
                identity: cid.identity.clone(),
            },
            SessionEditV1::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name: RackName::Simd1,
                effect_id: native.id.clone(),
                quality: EffectQuality::Draft,
            },
            SessionEditV1::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: cid.id.clone(),
                quality: EffectQuality::Normal,
            },
            SessionEditV1::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name: RackName::Simd2,
                effect_id: high.id.clone(),
                quality: EffectQuality::High,
            },
            SessionEditV1::SetEffectBypass {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: native.id.clone(),
                bypass: true,
            },
            SessionEditV1::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name: RackName::Simd1,
                effect_id: native.id.clone(),
                link_mode: LinkMode::DualMono,
            },
            SessionEditV1::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: cid.id.clone(),
                link_mode: LinkMode::Maximum,
            },
            SessionEditV1::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name: RackName::Simd2,
                effect_id: high.id.clone(),
                link_mode: LinkMode::Average,
            },
            SessionEditV1::SetEffectSidechain {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: native.id.clone(),
                sidechain: SidechainDeclaration::None,
            },
            SessionEditV1::SetEffectSidechain {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: cid.id.clone(),
                sidechain: cid.sidechain.clone(),
            },
            SessionEditV1::SetEffectSidechain {
                track_id: track_id.clone(),
                rack_name: RackName::Simd2,
                effect_id: high.id.clone(),
                sidechain: high.sidechain.clone(),
            },
        ]);
        for parameter in parameters {
            edits.push(SessionEditV1::UpsertEffectParam {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: native.id.clone(),
                param: parameter,
            });
        }
        edits.extend([
            SessionEditV1::RemoveEffectParam {
                track_id: track_id.clone(),
                rack_name: RackName::Dynamic,
                effect_id: native.id.clone(),
                parameter_id: 11,
                channel: ParameterChannel::Left,
            },
            SessionEditV1::SetTrackFader {
                track_id: track_id.clone(),
                fader: full_track.fader.clone(),
            },
            SessionEditV1::SetTrackMatrixOrPan {
                track_id: track_id.clone(),
                matrix_or_pan: full_track.matrix_or_pan.clone(),
            },
            SessionEditV1::SetTrackMatrixOrPan {
                track_id,
                matrix_or_pan: MatrixOrPan::Matrix {
                    ll: 1.0,
                    lr: 0.25,
                    rl: 0.5,
                    rr: 1.0,
                    smoothing_samples: 32,
                },
            },
        ]);
        let bytes = encode(&edits);
        let codec = ProtocolCodec::default();
        let mut scratch = [0_u16; 64];
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
            .expect("typed track/effect decode");
        assert_eq!(decoded.edits, edits);
        assert_eq!(encode(&decoded.edits), bytes);
        let SessionEditV1::UpsertTrack {
            track: decoded_track,
        } = &decoded.edits[0]
        else {
            panic!("first edit must be upsert track");
        };
        assert_eq!(
            decoded_track
                .dynamic
                .effects
                .iter()
                .map(|effect| effect.id.as_str())
                .collect::<Vec<_>>(),
            ["cid-fx", "high-fx"]
        );
    }

    #[test]
    fn track_effect_schema_rejects_duplicate_type_order_and_variant_errors() {
        let duplicate = raw_message(vec![
            (1, WIRE_BOOL, true, vec![0]),
            (1, WIRE_BOOL, true, vec![0]),
            (2, WIRE_F32, true, 0.0_f32.to_le_bytes().to_vec()),
            (3, WIRE_F32, true, 20.0_f32.to_le_bytes().to_vec()),
            (4, WIRE_F32, true, 20_000.0_f32.to_le_bytes().to_vec()),
        ]);
        assert!(parse_channel_builtins(Message::nested(&duplicate).expect("nested")).is_err());

        let wrong_type = raw_message(vec![
            (1, WIRE_U8, true, vec![0]),
            (2, WIRE_F32, true, 0.0_f32.to_le_bytes().to_vec()),
            (3, WIRE_F32, true, 20.0_f32.to_le_bytes().to_vec()),
            (4, WIRE_F32, true, 20_000.0_f32.to_le_bytes().to_vec()),
        ]);
        assert!(parse_channel_builtins(Message::nested(&wrong_type).expect("nested")).is_err());

        let unknown = raw_message(vec![
            (1, WIRE_U8, true, vec![1]),
            (2, WIRE_U8, true, vec![7]),
        ]);
        assert_eq!(
            parse_sidechain(Message::nested(&unknown).expect("nested")),
            Err(DecodeError::UnknownRequiredField)
        );

        let reversed = [
            2, 0, WIRE_U8, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, WIRE_U8, 1, 1, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(
            Message::raw(&reversed, 2)
                .schema_spec(&schema::session::sidechain::ROUTED)
                .err(),
            Some(DecodeError::InvalidTlv)
        );
    }

    #[test]
    fn tagged_pan_rejects_optional_field_known_only_to_matrix_variant() {
        let pan_with_matrix_field = raw_message(vec![
            (1, WIRE_U8, true, vec![1]),
            (2, WIRE_F32, true, 0.25_f32.to_le_bytes().to_vec()),
            (3, WIRE_F32, true, 0.75_f32.to_le_bytes().to_vec()),
            (4, WIRE_U32, true, 32_u32.to_le_bytes().to_vec()),
            (5, WIRE_F32, false, 1.0_f32.to_le_bytes().to_vec()),
        ]);
        assert_eq!(
            parse_matrix_or_pan(Message::nested(&pan_with_matrix_field).expect("nested")),
            Err(DecodeError::InvalidTlv)
        );
    }

    #[test]
    fn every_send_tap_tag_is_typed_and_canonical() {
        for tap in [
            SendTap::Input,
            SendTap::PostInputBuiltins,
            SendTap::PostSimd1,
            SendTap::PostDynamic,
            SendTap::PostSimd2PreFader,
            SendTap::PostFader,
            SendTap::PostMatrix,
        ] {
            let source = RouteSource::Track {
                track_id: id("vocal"),
                tap,
            };
            let limits = ProtocolCodec::default().limits();
            let mut count = CountSink::new(limits);
            tx_route_source(&mut count, &source).expect("size route source");
            let mut encoded = vec![0; count.written()];
            let mut writer = SliceSink::new(&mut encoded, limits);
            tx_route_source(&mut writer, &source).expect("encode route source");
            assert_eq!(
                parse_route_source(Message::nested(&encoded).expect("nested route source")),
                Ok(source)
            );
        }
    }

    #[test]
    fn every_route_and_automation_opcode_round_trips_canonically() {
        let session =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("fixture");
        let route = session.routes[0].clone();
        let target = session.automation[0].target.clone();
        let automation = Automation {
            id: id("ride"),
            target: target.clone(),
            segments: vec![
                AutomationSegment {
                    shape: AutomationShape::Step,
                    start_sample: 0,
                    end_sample: 16,
                    start_value: -6.0,
                    end_value: -6.0,
                    unit: ParameterUnit::Db,
                },
                AutomationSegment {
                    shape: AutomationShape::Linear,
                    start_sample: 16,
                    end_sample: 32,
                    start_value: 100.0,
                    end_value: 1_000.0,
                    unit: ParameterUnit::Hz,
                },
                AutomationSegment {
                    shape: AutomationShape::Exponential,
                    start_sample: 32,
                    end_sample: 48,
                    start_value: 1.0,
                    end_value: 2.0,
                    unit: ParameterUnit::Ratio,
                },
            ],
        };
        let edits = vec![
            SessionEditV1::UpsertSubmix {
                submix: Submix { id: id("drums") },
            },
            SessionEditV1::RemoveSubmix {
                submix_id: id("drums"),
            },
            SessionEditV1::UpsertOutput {
                output: Output { id: id("alt-out") },
            },
            SessionEditV1::RemoveOutput {
                output_id: id("alt-out"),
            },
            SessionEditV1::UpsertRoute {
                route: route.clone(),
            },
            SessionEditV1::RemoveRoute {
                route_id: route.id.clone(),
            },
            SessionEditV1::SetRouteSource {
                route_id: route.id.clone(),
                source: RouteSource::Track {
                    track_id: id("vocal"),
                    tap: SendTap::PostMatrix,
                },
            },
            SessionEditV1::SetRouteSource {
                route_id: route.id.clone(),
                source: RouteSource::SubmixOutput {
                    submix_id: id("drums"),
                },
            },
            SessionEditV1::SetRouteDestination {
                route_id: route.id.clone(),
                destination: RouteDestination::SubmixInput {
                    submix_id: id("drums"),
                },
            },
            SessionEditV1::SetRouteDestination {
                route_id: route.id.clone(),
                destination: RouteDestination::OutputInput {
                    output_id: id("main-out"),
                },
            },
            SessionEditV1::SetRouteChannelMatrix {
                route_id: route.id.clone(),
                channel_matrix: ChannelMatrix {
                    ll: 1.0,
                    lr: 0.25,
                    rl: 0.5,
                    rr: 1.0,
                },
            },
            SessionEditV1::SetRouteGainDb {
                route_id: route.id.clone(),
                gain_db: -1.5,
            },
            SessionEditV1::UpsertAutomation {
                automation: automation.clone(),
            },
            SessionEditV1::RemoveAutomation {
                automation_id: automation.id.clone(),
            },
            SessionEditV1::SetAutomationTarget {
                automation_id: automation.id.clone(),
                target: target.clone(),
            },
            SessionEditV1::SetAutomationSegments {
                automation_id: automation.id.clone(),
                segments: automation.segments.clone(),
            },
        ];
        let bytes = encode(&edits);
        let codec = ProtocolCodec::default();
        let mut scratch = [0_u16; 32];
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
            .expect("typed route/automation decode");
        assert_eq!(decoded.edits, edits);
        assert_eq!(encode(&decoded.edits), bytes);
        let SessionEditV1::UpsertAutomation { automation } = &decoded.edits[12] else {
            panic!("upsert automation");
        };
        assert_eq!(
            automation
                .segments
                .iter()
                .map(|segment| segment.start_sample)
                .collect::<Vec<_>>(),
            [0, 16, 32]
        );
    }

    #[test]
    fn route_and_automation_unknown_optional_fields_are_canonicalized_away() {
        let bytes = raw_message(vec![
            (1, WIRE_U8, true, vec![2]),
            (2, WIRE_UTF8, true, b"main-out".to_vec()),
            (99, WIRE_U8, false, vec![7]),
        ]);
        assert_eq!(
            parse_route_destination(Message::nested(&bytes).expect("nested destination")),
            Ok(RouteDestination::OutputInput {
                output_id: id("main-out"),
            })
        );

        let mut required = bytes;
        required[8 + 16 + 16 + 3] = 1;
        assert_eq!(
            parse_route_destination(Message::nested(&required).expect("nested destination")),
            Err(DecodeError::UnknownRequiredField)
        );
    }

    #[test]
    fn optional_fields_skip_but_required_and_corrupt_nested_fields_reject() {
        let edits = [SessionEditV1::SetSessionId {
            session_id: id("next"),
        }];
        let canonical = encode(&edits);
        let mut optional = canonical.clone();
        let optional_offset = optional.len();
        optional.extend_from_slice(&[99, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
        put_u32(&mut optional, 20, 80);
        put_u32(&mut optional, 40, 2);
        let codec = ProtocolCodec::default();
        let mut scratch = [0_u16; 2];
        let decoded = codec
            .decode_session_transaction(&optional, &mut DecodeScratch::new(&mut scratch))
            .expect("optional skips");
        assert_eq!(encode(&decoded.edits), canonical);
        optional[optional_offset + 3] = 1;
        assert_eq!(
            codec.decode_session_transaction(&optional, &mut DecodeScratch::new(&mut scratch)),
            Err(DecodeError::UnknownRequiredField)
        );
        let mut bad_type = canonical;
        bad_type[48 + 8 + 8 + 2] = WIRE_U32;
        assert!(
            codec
                .decode_session_transaction(&bad_type, &mut DecodeScratch::new(&mut [0_u16; 1]))
                .is_err()
        );
    }

    #[test]
    fn every_byte_of_transaction_golden_truncates() {
        let bytes = encode(&[SessionEditV1::SetSessionId {
            session_id: id("next"),
        }]);
        let codec = ProtocolCodec::default();
        for end in 0..bytes.len() {
            assert!(
                codec
                    .decode_session_transaction(
                        &bytes[..end],
                        &mut DecodeScratch::new(&mut [0_u16; 1])
                    )
                    .is_err()
            );
        }
    }

    fn hex(bytes: &[u8]) -> String {
        let mut output = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            use core::fmt::Write as _;
            write!(&mut output, "{byte:02x}").expect("string write");
        }
        output
    }
}
