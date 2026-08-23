//! Schema-specific BTLV transaction framing and the first V1 session-edit encoder subset.
//!
//! This staged module deliberately has no arbitrary-field escape hatch. It currently encodes the
//! six session-scalar/profile/limits edit opcodes; subsequent Issue-005 subparts extend this same
//! canonical builder with the remaining accepted model structures.

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
    CommandFrame, DecodeError, DecodeScratch, DecodedFrame, EncodeError, ExpectedRevision, Frame,
    MessageId, ProtocolCodec, RequestId, SessionEditV1,
    btlv::{
        CountSink, Sink, SliceSink, WIRE_BOOL, WIRE_F32, WIRE_MESSAGE, WIRE_U8, WIRE_U16, WIRE_U32,
        WIRE_U64, WIRE_UTF8,
    },
};

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
        if !matches!(transaction.expected_revision, ExpectedRevision::Exact(_))
            || transaction.edits.is_empty()
        {
            return Err(EncodeError::MessageKindMismatch);
        }
        let mut sizing = CountSink::new(self.limits());
        encode_transaction_payload_into(&mut sizing, transaction.edits)?;
        let required = crate::OUTER_HEADER_BYTES
            .checked_add(sizing.written())
            .ok_or(EncodeError::LimitExceeded)?;
        if required > self.limits().max_frame_bytes {
            return Err(EncodeError::LimitExceeded);
        }
        Ok(required)
    }

    /// Encode a canonical session transaction. A short caller buffer is left wholly unmodified.
    pub fn encode_session_transaction(
        &self,
        transaction: &SessionTransactionFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_session_transaction_len(transaction)?;
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
        put_u32(
            output,
            40,
            u32::try_from(transaction.edits.len()).map_err(|_| EncodeError::LimitExceeded)?,
        );
        let mut writer = SliceSink::new(
            &mut output[crate::OUTER_HEADER_BYTES..required],
            self.limits(),
        );
        encode_transaction_payload_into(&mut writer, transaction.edits)?;
        debug_assert_eq!(writer.written(), required - crate::OUTER_HEADER_BYTES);
        Ok(required)
    }

    /// Decode and validate only the borrowed outer transaction frame without allocating edits.
    pub fn decode_session_transaction_outer<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let limits = self.limits();
        let envelope_codec = ProtocolCodec::new(crate::ProtocolLimits {
            // The transaction/edit/payload wrappers are fixed protocol envelopes. They do not
            // consume the configured logical model-message nesting allowance.
            max_nesting: limits.max_nesting.saturating_add(3),
            ..limits
        });
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
        let outer = self.decode_session_transaction_outer(input, scratch)?;
        let header = outer
            .frame
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        let top = Message::tlvs(outer.frame.payload, header.tlv_count)?;
        top.schema(&[Rule::repeated(1)])?;
        if top.fields.is_empty() {
            return Err(DecodeError::InvalidTlv);
        }
        let mut edits = Vec::with_capacity(top.fields.iter().filter(|field| field.id == 1).count());
        for value in top.values(1, WIRE_MESSAGE)? {
            edits.push(parse_edit(Message::nested(value)?)?);
        }
        Ok(DecodedSessionTransaction {
            frame: outer.frame,
            edits,
        })
    }

    /// Decode a transaction only after its exact repeated edit count is within the endpoint cap.
    pub fn decode_session_transaction_limited<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
        maximum_edits: u32,
    ) -> Result<DecodedSessionTransaction<'a>, DecodeError> {
        let outer = self.decode_session_transaction_outer(input, scratch)?;
        let header = outer
            .frame
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        let top = Message::tlvs(outer.frame.payload, header.tlv_count)?;
        top.schema(&[Rule::repeated(1)])?;
        let count = u32::try_from(top.values(1, WIRE_MESSAGE)?.count())
            .map_err(|_| DecodeError::LimitExceeded)?;
        if count == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        if count > maximum_edits {
            return Err(DecodeError::LimitExceeded);
        }
        let mut edits = Vec::with_capacity(count as usize);
        for value in top.values(1, WIRE_MESSAGE)? {
            edits.push(parse_edit(Message::nested(value)?)?);
        }
        Ok(DecodedSessionTransaction {
            frame: outer.frame,
            edits,
        })
    }
}

struct MessageBuilder {
    fields: Vec<u8>,
    count: u32,
}

impl MessageBuilder {
    fn new() -> Self {
        Self {
            fields: Vec::new(),
            count: 0,
        }
    }
    fn field(&mut self, id: u16, wire: u8, value: &[u8]) -> Result<(), EncodeError> {
        self.count = self
            .count
            .checked_add(1)
            .ok_or(EncodeError::OutputTooSmall {
                required: usize::MAX,
            })?;
        self.fields.extend_from_slice(&id.to_le_bytes());
        self.fields.push(wire);
        self.fields.push(1);
        self.fields.extend_from_slice(
            &u32::try_from(value.len())
                .map_err(|_| EncodeError::OutputTooSmall {
                    required: usize::MAX,
                })?
                .to_le_bytes(),
        );
        self.fields.extend_from_slice(value);
        self.fields
            .resize(self.fields.len() + padding(value.len()), 0);
        Ok(())
    }
    fn u8(&mut self, id: u16, value: u8) -> Result<(), EncodeError> {
        self.field(id, WIRE_U8, &[value])
    }
    fn u16(&mut self, id: u16, value: u16) -> Result<(), EncodeError> {
        self.field(id, WIRE_U16, &value.to_le_bytes())
    }
    fn u32(&mut self, id: u16, value: u32) -> Result<(), EncodeError> {
        self.field(id, WIRE_U32, &value.to_le_bytes())
    }
    fn u64(&mut self, id: u16, value: u64) -> Result<(), EncodeError> {
        self.field(id, WIRE_U64, &value.to_le_bytes())
    }
    fn f32(&mut self, id: u16, value: f32) -> Result<(), EncodeError> {
        self.field(id, WIRE_F32, &value.to_le_bytes())
    }
    fn boolean(&mut self, id: u16, value: bool) -> Result<(), EncodeError> {
        self.field(id, WIRE_BOOL, &[u8::from(value)])
    }
    fn id(&mut self, id: u16, value: &StableId) -> Result<(), EncodeError> {
        self.field(id, WIRE_UTF8, value.as_str().as_bytes())
    }
    fn text(&mut self, id: u16, value: &str) -> Result<(), EncodeError> {
        self.field(id, WIRE_UTF8, value.as_bytes())
    }
    fn message(&mut self, id: u16, value: Vec<u8>) -> Result<(), EncodeError> {
        self.field(id, WIRE_MESSAGE, &value)
    }
    fn finish(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.fields.len());
        bytes.extend_from_slice(&self.count.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&self.fields);
        bytes
    }
}

fn transaction_payload(edits: &[SessionEditV1]) -> Result<Vec<u8>, EncodeError> {
    let mut top = MessageBuilder::new();
    for edit in edits {
        top.message(1, edit_message(edit)?)?;
    }
    let complete = top.finish();
    Ok(complete[8..].to_vec())
}

fn edit_message(edit: &SessionEditV1) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u16(1, edit.opcode().raw())?;
    message.message(2, edit_payload(edit)?)?;
    Ok(message.finish())
}

fn edit_payload(edit: &SessionEditV1) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match edit {
        SessionEditV1::SetSessionId { session_id } => message.id(1, session_id)?,
        SessionEditV1::SetSampleRateHz { sample_rate_hz } => message.u32(1, *sample_rate_hz)?,
        SessionEditV1::SetQuantumFrames { quantum_frames } => message.u32(1, *quantum_frames)?,
        SessionEditV1::SetRenderProfile { render_profile } => {
            message.message(1, render_profile_message(render_profile)?)?
        }
        SessionEditV1::SetOutputProfile { output_profile } => {
            message.message(1, output_profile_message(output_profile)?)?
        }
        SessionEditV1::SetLimits { limits } => message.message(1, limits_message(limits)?)?,
        SessionEditV1::UpsertSource { source } => message.message(1, source_message(source)?)?,
        SessionEditV1::RemoveSource { source_id } => message.id(1, source_id)?,
        SessionEditV1::SetSourceSampleRateHz {
            source_id,
            sample_rate_hz,
        } => {
            message.id(1, source_id)?;
            message.u32(2, *sample_rate_hz)?;
        }
        SessionEditV1::SetSourceContent { source_id, content } => {
            message.id(1, source_id)?;
            message.message(2, content_message(content)?)?;
        }
        SessionEditV1::SetSourceMapping { source_id, mapping } => {
            message.id(1, source_id)?;
            message.message(2, mapping_message(mapping)?)?;
        }
        SessionEditV1::UpsertTrack { track } => message.message(1, track_message(track)?)?,
        SessionEditV1::RemoveTrack { track_id } => message.id(1, track_id)?,
        SessionEditV1::SetTrackSourceAssignment {
            track_id,
            source_id,
            left_source_channel,
            right_source_channel,
        } => {
            message.id(1, track_id)?;
            message.id(2, source_id)?;
            message.u8(3, *left_source_channel)?;
            message.u8(4, *right_source_channel)?;
        }
        SessionEditV1::SetTrackBuiltins { track_id, builtins } => {
            message.id(1, track_id)?;
            message.message(2, builtins_message(builtins)?)?;
        }
        SessionEditV1::SetTrackRack {
            track_id,
            rack_name,
            rack,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.message(3, rack_message(rack)?)?;
        }
        SessionEditV1::PutTrackEffect {
            track_id,
            rack_name,
            final_position,
            effect,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.u32(3, *final_position)?;
            message.message(4, effect_message(effect)?)?;
        }
        SessionEditV1::RemoveTrackEffect {
            track_id,
            rack_name,
            effect_id,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
        }
        SessionEditV1::SetTrackEffectOrder {
            track_id,
            rack_name,
            effect_ids,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            for effect_id in effect_ids {
                message.id(3, effect_id)?;
            }
        }
        SessionEditV1::SetEffectIdentity {
            track_id,
            rack_name,
            effect_id,
            identity,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.message(4, identity_message(identity)?)?;
        }
        SessionEditV1::SetEffectQuality {
            track_id,
            rack_name,
            effect_id,
            quality,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.u8(4, enum_quality(*quality))?;
        }
        SessionEditV1::SetEffectBypass {
            track_id,
            rack_name,
            effect_id,
            bypass,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.boolean(4, *bypass)?;
        }
        SessionEditV1::SetEffectLinkMode {
            track_id,
            rack_name,
            effect_id,
            link_mode,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.u8(4, enum_link(*link_mode))?;
        }
        SessionEditV1::SetEffectSidechain {
            track_id,
            rack_name,
            effect_id,
            sidechain,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.message(4, sidechain_message(sidechain)?)?;
        }
        SessionEditV1::UpsertEffectParam {
            track_id,
            rack_name,
            effect_id,
            param,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.message(4, param_message(param)?)?;
        }
        SessionEditV1::RemoveEffectParam {
            track_id,
            rack_name,
            effect_id,
            parameter_id,
            channel,
        } => {
            message.id(1, track_id)?;
            message.u8(2, enum_rack(*rack_name))?;
            message.id(3, effect_id)?;
            message.u32(4, *parameter_id)?;
            message.u8(5, enum_channel(*channel))?;
        }
        SessionEditV1::SetTrackFader { track_id, fader } => {
            message.id(1, track_id)?;
            message.message(2, fader_message(fader)?)?;
        }
        SessionEditV1::SetTrackMatrixOrPan {
            track_id,
            matrix_or_pan,
        } => {
            message.id(1, track_id)?;
            message.message(2, matrix_or_pan_message(matrix_or_pan)?)?;
        }
        SessionEditV1::UpsertSubmix { submix } => message.message(1, submix_message(submix)?)?,
        SessionEditV1::RemoveSubmix { submix_id } => message.id(1, submix_id)?,
        SessionEditV1::UpsertOutput { output } => message.message(1, output_message(output)?)?,
        SessionEditV1::RemoveOutput { output_id } => message.id(1, output_id)?,
        SessionEditV1::UpsertRoute { route } => message.message(1, route_message(route)?)?,
        SessionEditV1::RemoveRoute { route_id } => message.id(1, route_id)?,
        SessionEditV1::SetRouteSource { route_id, source } => {
            message.id(1, route_id)?;
            message.message(2, route_source_message(source)?)?;
        }
        SessionEditV1::SetRouteDestination {
            route_id,
            destination,
        } => {
            message.id(1, route_id)?;
            message.message(2, route_destination_message(destination)?)?;
        }
        SessionEditV1::SetRouteChannelMatrix {
            route_id,
            channel_matrix,
        } => {
            message.id(1, route_id)?;
            message.message(2, channel_matrix_message(channel_matrix)?)?;
        }
        SessionEditV1::SetRouteGainDb { route_id, gain_db } => {
            message.id(1, route_id)?;
            message.f32(2, *gain_db)?;
        }
        SessionEditV1::UpsertAutomation { automation } => {
            message.message(1, automation_message(automation)?)?
        }
        SessionEditV1::RemoveAutomation { automation_id } => message.id(1, automation_id)?,
        SessionEditV1::SetAutomationTarget {
            automation_id,
            target,
        } => {
            message.id(1, automation_id)?;
            message.message(2, automation_target_message(target)?)?;
        }
        SessionEditV1::SetAutomationSegments {
            automation_id,
            segments,
        } => {
            message.id(1, automation_id)?;
            for segment in segments {
                message.message(2, automation_segment_message(segment)?)?;
            }
        }
    }
    Ok(message.finish())
}

fn render_profile_message(value: &RenderProfile) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.u8(
        2,
        match value.mode {
            miso_engine_session::RenderMode::SingleThread => 1,
            miso_engine_session::RenderMode::DependencyWaves => 2,
        },
    )?;
    Ok(message.finish())
}

fn output_profile_message(value: &OutputProfile) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.u8(2, value.channels)?;
    message.u8(3, 1)?;
    Ok(message.finish())
}

fn limits_message(value: &SessionLimits) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u64(1, value.pcm_ring_frames)?;
    message.u64(2, value.control_queue_messages)?;
    message.u64(3, value.memory_bytes)?;
    Ok(message.finish())
}

fn content_message(value: &SourceContent) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.text(1, &value.identity)?;
    message.text(2, &value.locator)?;
    Ok(message.finish())
}

fn region_message(value: &SourceRegion) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u64(1, value.start_sample)?;
    message.u64(2, value.length_samples)?;
    Ok(message.finish())
}

fn mapping_message(value: &SourceMapping) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u8(1, value.channel_count)?;
    message.message(2, region_message(&value.region)?)?;
    Ok(message.finish())
}

fn source_message(value: &Source) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.u32(2, value.sample_rate_hz)?;
    message.message(3, content_message(&value.content)?)?;
    message.message(4, mapping_message(&value.mapping)?)?;
    Ok(message.finish())
}

fn builtins_message(value: &DualMonoBuiltins) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.message(1, channel_builtins_message(&value.left)?)?;
    message.message(2, channel_builtins_message(&value.right)?)?;
    Ok(message.finish())
}
fn channel_builtins_message(value: &ChannelBuiltins) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.boolean(1, value.polarity_invert)?;
    message.f32(2, value.trim_db)?;
    message.f32(3, value.hpf_hz)?;
    message.f32(4, value.lpf_hz)?;
    Ok(message.finish())
}
fn rack_message(value: &Rack) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    for effect in &value.effects {
        message.message(1, effect_message(effect)?)?;
    }
    Ok(message.finish())
}
fn identity_message(value: &EffectIdentity) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match value {
        EffectIdentity::Native { effect_id } => {
            message.u8(1, 1)?;
            message.id(2, effect_id)?;
        }
        EffectIdentity::ThirdPartyCid { cid } => {
            message.u8(1, 2)?;
            message.text(2, cid)?;
        }
    }
    Ok(message.finish())
}
fn route_source_message(value: &RouteSource) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match value {
        RouteSource::Track { track_id, tap } => {
            message.u8(1, 1)?;
            message.id(2, track_id)?;
            message.u8(3, enum_tap(*tap))?;
        }
        RouteSource::SubmixOutput { submix_id } => {
            message.u8(1, 2)?;
            message.id(2, submix_id)?;
        }
    }
    Ok(message.finish())
}
fn route_destination_message(value: &RouteDestination) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match value {
        RouteDestination::SubmixInput { submix_id } => {
            message.u8(1, 1)?;
            message.id(2, submix_id)?;
        }
        RouteDestination::OutputInput { output_id } => {
            message.u8(1, 2)?;
            message.id(2, output_id)?;
        }
    }
    Ok(message.finish())
}
fn sidechain_message(value: &SidechainDeclaration) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match value {
        SidechainDeclaration::None => message.u8(1, 1)?,
        SidechainDeclaration::Routed(value) => {
            message.u8(1, 2)?;
            message.message(2, route_source_message(&value.source)?)?;
            message.id(3, &value.port_id)?;
        }
    }
    Ok(message.finish())
}
fn param_message(value: &EffectParam) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u32(1, value.parameter_id)?;
    message.u8(2, enum_channel(value.channel))?;
    message.u8(3, enum_unit(value.unit))?;
    message.f32(4, value.value)?;
    Ok(message.finish())
}
fn effect_message(value: &Effect) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.message(2, identity_message(&value.identity)?)?;
    message.u8(3, enum_quality(value.quality))?;
    message.boolean(4, value.bypass)?;
    message.u8(5, enum_link(value.link_mode))?;
    for parameter in &value.params {
        message.message(6, param_message(parameter)?)?;
    }
    message.message(7, sidechain_message(&value.sidechain)?)?;
    Ok(message.finish())
}
fn fader_message(value: &DualMonoFader) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.f32(1, value.left_db)?;
    message.f32(2, value.right_db)?;
    message.boolean(3, value.left_mute)?;
    message.boolean(4, value.right_mute)?;
    Ok(message.finish())
}
fn matrix_or_pan_message(value: &MatrixOrPan) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    match value {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => {
            message.u8(1, 1)?;
            message.f32(2, *left)?;
            message.f32(3, *right)?;
            message.u32(4, *smoothing_samples)?;
        }
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => {
            message.u8(1, 2)?;
            message.f32(2, *ll)?;
            message.f32(3, *lr)?;
            message.f32(4, *rl)?;
            message.f32(5, *rr)?;
            message.u32(6, *smoothing_samples)?;
        }
    }
    Ok(message.finish())
}
fn track_message(value: &miso_engine_session::Track) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.id(2, &value.source_id)?;
    message.u8(3, value.left_source_channel)?;
    message.u8(4, value.right_source_channel)?;
    message.message(5, builtins_message(&value.builtins)?)?;
    message.message(6, rack_message(&value.simd1)?)?;
    message.message(7, rack_message(&value.dynamic)?)?;
    message.message(8, rack_message(&value.simd2)?)?;
    message.message(9, fader_message(&value.fader)?)?;
    message.message(10, matrix_or_pan_message(&value.matrix_or_pan)?)?;
    Ok(message.finish())
}
fn submix_message(value: &Submix) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    Ok(message.finish())
}
fn output_message(value: &Output) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    Ok(message.finish())
}
fn channel_matrix_message(value: &ChannelMatrix) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.f32(1, value.ll)?;
    message.f32(2, value.lr)?;
    message.f32(3, value.rl)?;
    message.f32(4, value.rr)?;
    Ok(message.finish())
}
fn route_message(value: &Route) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.message(2, route_source_message(&value.source)?)?;
    message.message(3, route_destination_message(&value.destination)?)?;
    message.message(4, channel_matrix_message(&value.channel_matrix)?)?;
    message.f32(5, value.gain_db)?;
    Ok(message.finish())
}
fn automation_target_message(value: &AutomationTarget) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.entity_id)?;
    message.u8(2, enum_rack(value.rack))?;
    message.id(3, &value.effect_id)?;
    message.u32(4, value.parameter_id)?;
    message.u8(5, enum_channel(value.channel))?;
    Ok(message.finish())
}
fn automation_segment_message(value: &AutomationSegment) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.u8(1, enum_shape(value.shape))?;
    message.u64(2, value.start_sample)?;
    message.u64(3, value.end_sample)?;
    message.f32(4, value.start_value)?;
    message.f32(5, value.end_value)?;
    message.u8(6, enum_unit(value.unit))?;
    Ok(message.finish())
}
fn automation_message(value: &Automation) -> Result<Vec<u8>, EncodeError> {
    let mut message = MessageBuilder::new();
    message.id(1, &value.id)?;
    message.message(2, automation_target_message(&value.target)?)?;
    for segment in &value.segments {
        message.message(3, automation_segment_message(segment)?)?;
    }
    Ok(message.finish())
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
const fn enum_channel(value: miso_engine_session::ParameterChannel) -> u8 {
    match value {
        miso_engine_session::ParameterChannel::Left => 1,
        miso_engine_session::ParameterChannel::Right => 2,
        miso_engine_session::ParameterChannel::Both => 3,
    }
}
const fn enum_unit(value: miso_engine_session::ParameterUnit) -> u8 {
    match value {
        miso_engine_session::ParameterUnit::Db => 1,
        miso_engine_session::ParameterUnit::Hz => 2,
        miso_engine_session::ParameterUnit::Milliseconds => 3,
        miso_engine_session::ParameterUnit::Samples => 4,
        miso_engine_session::ParameterUnit::Linear => 5,
        miso_engine_session::ParameterUnit::Ratio => 6,
    }
}
const fn enum_rack(value: RackName) -> u8 {
    match value {
        RackName::Simd1 => 1,
        RackName::Dynamic => 2,
        RackName::Simd2 => 3,
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

fn tx_count(base: u32, repeated: usize) -> Result<u32, EncodeError> {
    base.checked_add(u32::try_from(repeated).map_err(|_| EncodeError::LimitExceeded)?)
        .ok_or(EncodeError::LimitExceeded)
}

fn tx_start_message(sink: &mut dyn Sink, count: u32) -> Result<(), EncodeError> {
    sink.message_header(count)
}

fn tx_field(sink: &mut dyn Sink, id: u16, wire: u8, value: &[u8]) -> Result<(), EncodeError> {
    sink.field(id, wire, value)
}

fn tx_u8(sink: &mut dyn Sink, id: u16, value: u8) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_U8, &[value])
}
fn tx_u16(sink: &mut dyn Sink, id: u16, value: u16) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_U16, &value.to_le_bytes())
}
fn tx_u32(sink: &mut dyn Sink, id: u16, value: u32) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_U32, &value.to_le_bytes())
}
fn tx_u64(sink: &mut dyn Sink, id: u16, value: u64) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_U64, &value.to_le_bytes())
}
fn tx_f32(sink: &mut dyn Sink, id: u16, value: f32) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_F32, &value.to_le_bytes())
}
fn tx_bool(sink: &mut dyn Sink, id: u16, value: bool) -> Result<(), EncodeError> {
    tx_field(sink, id, WIRE_BOOL, &[u8::from(value)])
}
fn tx_text(sink: &mut dyn Sink, id: u16, value: &str) -> Result<(), EncodeError> {
    if value.len() > sink.limits().max_string_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    tx_field(sink, id, WIRE_UTF8, value.as_bytes())
}
fn tx_id(sink: &mut dyn Sink, id: u16, value: &StableId) -> Result<(), EncodeError> {
    tx_text(sink, id, value.as_str())
}
fn tx_message(
    sink: &mut dyn Sink,
    id: u16,
    mut encode: impl FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    sink.nested(id, &mut encode)
}

fn encode_transaction_payload_into(
    sink: &mut dyn Sink,
    edits: &[SessionEditV1],
) -> Result<(), EncodeError> {
    if tx_count(0, edits.len())? > sink.limits().max_tlv_count {
        return Err(EncodeError::LimitExceeded);
    }
    for edit in edits {
        tx_message(sink, 1, |nested| tx_edit_message(nested, edit))?;
    }
    Ok(())
}

fn tx_edit_message(sink: &mut dyn Sink, edit: &SessionEditV1) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_u16(sink, 1, edit.opcode().raw())?;
    tx_message(sink, 2, |nested| tx_edit_payload(nested, edit))
}

fn tx_edit_payload(sink: &mut dyn Sink, edit: &SessionEditV1) -> Result<(), EncodeError> {
    let count = match edit {
        SessionEditV1::SetSessionId { .. }
        | SessionEditV1::SetSampleRateHz { .. }
        | SessionEditV1::SetQuantumFrames { .. }
        | SessionEditV1::SetRenderProfile { .. }
        | SessionEditV1::SetOutputProfile { .. }
        | SessionEditV1::SetLimits { .. }
        | SessionEditV1::UpsertSource { .. }
        | SessionEditV1::RemoveSource { .. }
        | SessionEditV1::UpsertTrack { .. }
        | SessionEditV1::RemoveTrack { .. }
        | SessionEditV1::UpsertSubmix { .. }
        | SessionEditV1::RemoveSubmix { .. }
        | SessionEditV1::UpsertOutput { .. }
        | SessionEditV1::RemoveOutput { .. }
        | SessionEditV1::UpsertRoute { .. }
        | SessionEditV1::RemoveRoute { .. }
        | SessionEditV1::UpsertAutomation { .. }
        | SessionEditV1::RemoveAutomation { .. } => 1,
        SessionEditV1::SetSourceSampleRateHz { .. }
        | SessionEditV1::SetSourceContent { .. }
        | SessionEditV1::SetSourceMapping { .. }
        | SessionEditV1::SetTrackBuiltins { .. }
        | SessionEditV1::SetTrackFader { .. }
        | SessionEditV1::SetTrackMatrixOrPan { .. }
        | SessionEditV1::SetRouteSource { .. }
        | SessionEditV1::SetRouteDestination { .. }
        | SessionEditV1::SetRouteChannelMatrix { .. }
        | SessionEditV1::SetRouteGainDb { .. }
        | SessionEditV1::SetAutomationTarget { .. } => 2,
        SessionEditV1::SetTrackRack { .. } => 3,
        SessionEditV1::SetTrackSourceAssignment { .. }
        | SessionEditV1::PutTrackEffect { .. }
        | SessionEditV1::SetEffectIdentity { .. }
        | SessionEditV1::SetEffectQuality { .. }
        | SessionEditV1::SetEffectBypass { .. }
        | SessionEditV1::SetEffectLinkMode { .. }
        | SessionEditV1::SetEffectSidechain { .. }
        | SessionEditV1::UpsertEffectParam { .. } => 4,
        SessionEditV1::RemoveTrackEffect { .. } => 3,
        SessionEditV1::SetTrackEffectOrder { effect_ids, .. } => tx_count(2, effect_ids.len())?,
        SessionEditV1::RemoveEffectParam { .. } => 5,
        SessionEditV1::SetAutomationSegments { segments, .. } => tx_count(1, segments.len())?,
    };
    tx_start_message(sink, count)?;
    match edit {
        SessionEditV1::SetSessionId { session_id } => tx_id(sink, 1, session_id),
        SessionEditV1::SetSampleRateHz { sample_rate_hz } => tx_u32(sink, 1, *sample_rate_hz),
        SessionEditV1::SetQuantumFrames { quantum_frames } => tx_u32(sink, 1, *quantum_frames),
        SessionEditV1::SetRenderProfile { render_profile } => {
            tx_message(sink, 1, |v| tx_render_profile(v, render_profile))
        }
        SessionEditV1::SetOutputProfile { output_profile } => {
            tx_message(sink, 1, |v| tx_output_profile(v, output_profile))
        }
        SessionEditV1::SetLimits { limits } => tx_message(sink, 1, |v| tx_limits(v, limits)),
        SessionEditV1::UpsertSource { source } => tx_message(sink, 1, |v| tx_source(v, source)),
        SessionEditV1::RemoveSource { source_id } => tx_id(sink, 1, source_id),
        SessionEditV1::SetSourceSampleRateHz {
            source_id,
            sample_rate_hz,
        } => {
            tx_id(sink, 1, source_id)?;
            tx_u32(sink, 2, *sample_rate_hz)
        }
        SessionEditV1::SetSourceContent { source_id, content } => {
            tx_id(sink, 1, source_id)?;
            tx_message(sink, 2, |v| tx_content(v, content))
        }
        SessionEditV1::SetSourceMapping { source_id, mapping } => {
            tx_id(sink, 1, source_id)?;
            tx_message(sink, 2, |v| tx_mapping(v, mapping))
        }
        SessionEditV1::UpsertTrack { track } => tx_message(sink, 1, |v| tx_track(v, track)),
        SessionEditV1::RemoveTrack { track_id } => tx_id(sink, 1, track_id),
        SessionEditV1::SetTrackSourceAssignment {
            track_id,
            source_id,
            left_source_channel,
            right_source_channel,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_id(sink, 2, source_id)?;
            tx_u8(sink, 3, *left_source_channel)?;
            tx_u8(sink, 4, *right_source_channel)
        }
        SessionEditV1::SetTrackBuiltins { track_id, builtins } => {
            tx_id(sink, 1, track_id)?;
            tx_message(sink, 2, |v| tx_builtins(v, builtins))
        }
        SessionEditV1::SetTrackRack {
            track_id,
            rack_name,
            rack,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_u8(sink, 2, enum_rack(*rack_name))?;
            tx_message(sink, 3, |v| tx_rack(v, rack))
        }
        SessionEditV1::PutTrackEffect {
            track_id,
            rack_name,
            final_position,
            effect,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_u8(sink, 2, enum_rack(*rack_name))?;
            tx_u32(sink, 3, *final_position)?;
            tx_message(sink, 4, |v| tx_effect(v, effect))
        }
        SessionEditV1::RemoveTrackEffect {
            track_id,
            rack_name,
            effect_id,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_u8(sink, 2, enum_rack(*rack_name))?;
            tx_id(sink, 3, effect_id)
        }
        SessionEditV1::SetTrackEffectOrder {
            track_id,
            rack_name,
            effect_ids,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_u8(sink, 2, enum_rack(*rack_name))?;
            for effect_id in effect_ids {
                tx_id(sink, 3, effect_id)?;
            }
            Ok(())
        }
        SessionEditV1::SetEffectIdentity {
            track_id,
            rack_name,
            effect_id,
            identity,
        } => tx_effect_edit_message(sink, track_id, *rack_name, effect_id, |v| {
            tx_identity(v, identity)
        }),
        SessionEditV1::SetEffectQuality {
            track_id,
            rack_name,
            effect_id,
            quality,
        } => tx_effect_edit_scalar(
            sink,
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
            tx_effect_edit_prefix(sink, track_id, *rack_name, effect_id)?;
            tx_bool(sink, 4, *bypass)
        }
        SessionEditV1::SetEffectLinkMode {
            track_id,
            rack_name,
            effect_id,
            link_mode,
        } => tx_effect_edit_scalar(sink, track_id, *rack_name, effect_id, enum_link(*link_mode)),
        SessionEditV1::SetEffectSidechain {
            track_id,
            rack_name,
            effect_id,
            sidechain,
        } => tx_effect_edit_message(sink, track_id, *rack_name, effect_id, |v| {
            tx_sidechain(v, sidechain)
        }),
        SessionEditV1::UpsertEffectParam {
            track_id,
            rack_name,
            effect_id,
            param,
        } => tx_effect_edit_message(sink, track_id, *rack_name, effect_id, |v| {
            tx_param(v, param)
        }),
        SessionEditV1::RemoveEffectParam {
            track_id,
            rack_name,
            effect_id,
            parameter_id,
            channel,
        } => {
            tx_effect_edit_prefix(sink, track_id, *rack_name, effect_id)?;
            tx_u32(sink, 4, *parameter_id)?;
            tx_u8(sink, 5, enum_channel(*channel))
        }
        SessionEditV1::SetTrackFader { track_id, fader } => {
            tx_id(sink, 1, track_id)?;
            tx_message(sink, 2, |v| tx_fader(v, fader))
        }
        SessionEditV1::SetTrackMatrixOrPan {
            track_id,
            matrix_or_pan,
        } => {
            tx_id(sink, 1, track_id)?;
            tx_message(sink, 2, |v| tx_matrix_or_pan(v, matrix_or_pan))
        }
        SessionEditV1::UpsertSubmix { submix } => tx_message(sink, 1, |v| tx_submix(v, submix)),
        SessionEditV1::RemoveSubmix { submix_id } => tx_id(sink, 1, submix_id),
        SessionEditV1::UpsertOutput { output } => tx_message(sink, 1, |v| tx_output(v, output)),
        SessionEditV1::RemoveOutput { output_id } => tx_id(sink, 1, output_id),
        SessionEditV1::UpsertRoute { route } => tx_message(sink, 1, |v| tx_route(v, route)),
        SessionEditV1::RemoveRoute { route_id } => tx_id(sink, 1, route_id),
        SessionEditV1::SetRouteSource { route_id, source } => {
            tx_id(sink, 1, route_id)?;
            tx_message(sink, 2, |v| tx_route_source(v, source))
        }
        SessionEditV1::SetRouteDestination {
            route_id,
            destination,
        } => {
            tx_id(sink, 1, route_id)?;
            tx_message(sink, 2, |v| tx_route_destination(v, destination))
        }
        SessionEditV1::SetRouteChannelMatrix {
            route_id,
            channel_matrix,
        } => {
            tx_id(sink, 1, route_id)?;
            tx_message(sink, 2, |v| tx_channel_matrix(v, channel_matrix))
        }
        SessionEditV1::SetRouteGainDb { route_id, gain_db } => {
            tx_id(sink, 1, route_id)?;
            tx_f32(sink, 2, *gain_db)
        }
        SessionEditV1::UpsertAutomation { automation } => {
            tx_message(sink, 1, |v| tx_automation(v, automation))
        }
        SessionEditV1::RemoveAutomation { automation_id } => tx_id(sink, 1, automation_id),
        SessionEditV1::SetAutomationTarget {
            automation_id,
            target,
        } => {
            tx_id(sink, 1, automation_id)?;
            tx_message(sink, 2, |v| tx_automation_target(v, target))
        }
        SessionEditV1::SetAutomationSegments {
            automation_id,
            segments,
        } => {
            tx_id(sink, 1, automation_id)?;
            for segment in segments {
                tx_message(sink, 2, |v| tx_automation_segment(v, segment))?;
            }
            Ok(())
        }
    }
}

fn tx_effect_edit_prefix(
    sink: &mut dyn Sink,
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
) -> Result<(), EncodeError> {
    tx_id(sink, 1, track_id)?;
    tx_u8(sink, 2, enum_rack(rack_name))?;
    tx_id(sink, 3, effect_id)
}
fn tx_effect_edit_scalar(
    sink: &mut dyn Sink,
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
    value: u8,
) -> Result<(), EncodeError> {
    tx_effect_edit_prefix(sink, track_id, rack_name, effect_id)?;
    tx_u8(sink, 4, value)
}
fn tx_effect_edit_message(
    sink: &mut dyn Sink,
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
    encode: impl FnMut(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    tx_effect_edit_prefix(sink, track_id, rack_name, effect_id)?;
    tx_message(sink, 4, encode)
}

fn tx_render_profile(sink: &mut dyn Sink, value: &RenderProfile) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_id(sink, 1, &value.id)?;
    tx_u8(
        sink,
        2,
        match value.mode {
            miso_engine_session::RenderMode::SingleThread => 1,
            miso_engine_session::RenderMode::DependencyWaves => 2,
        },
    )
}
fn tx_output_profile(sink: &mut dyn Sink, value: &OutputProfile) -> Result<(), EncodeError> {
    tx_start_message(sink, 3)?;
    tx_id(sink, 1, &value.id)?;
    tx_u8(sink, 2, value.channels)?;
    tx_u8(sink, 3, 1)
}
fn tx_limits(sink: &mut dyn Sink, value: &SessionLimits) -> Result<(), EncodeError> {
    tx_start_message(sink, 3)?;
    tx_u64(sink, 1, value.pcm_ring_frames)?;
    tx_u64(sink, 2, value.control_queue_messages)?;
    tx_u64(sink, 3, value.memory_bytes)
}
fn tx_content(sink: &mut dyn Sink, value: &SourceContent) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_text(sink, 1, &value.identity)?;
    tx_text(sink, 2, &value.locator)
}
fn tx_region(sink: &mut dyn Sink, value: &SourceRegion) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_u64(sink, 1, value.start_sample)?;
    tx_u64(sink, 2, value.length_samples)
}
fn tx_mapping(sink: &mut dyn Sink, value: &SourceMapping) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_u8(sink, 1, value.channel_count)?;
    tx_message(sink, 2, |v| tx_region(v, &value.region))
}
fn tx_source(sink: &mut dyn Sink, value: &Source) -> Result<(), EncodeError> {
    tx_start_message(sink, 4)?;
    tx_id(sink, 1, &value.id)?;
    tx_u32(sink, 2, value.sample_rate_hz)?;
    tx_message(sink, 3, |v| tx_content(v, &value.content))?;
    tx_message(sink, 4, |v| tx_mapping(v, &value.mapping))
}
fn tx_builtins(sink: &mut dyn Sink, value: &DualMonoBuiltins) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    tx_message(sink, 1, |v| tx_channel_builtins(v, &value.left))?;
    tx_message(sink, 2, |v| tx_channel_builtins(v, &value.right))
}
fn tx_channel_builtins(sink: &mut dyn Sink, value: &ChannelBuiltins) -> Result<(), EncodeError> {
    tx_start_message(sink, 4)?;
    tx_bool(sink, 1, value.polarity_invert)?;
    tx_f32(sink, 2, value.trim_db)?;
    tx_f32(sink, 3, value.hpf_hz)?;
    tx_f32(sink, 4, value.lpf_hz)
}
fn tx_rack(sink: &mut dyn Sink, value: &Rack) -> Result<(), EncodeError> {
    tx_start_message(sink, tx_count(0, value.effects.len())?)?;
    for effect in &value.effects {
        tx_message(sink, 1, |v| tx_effect(v, effect))?;
    }
    Ok(())
}
fn tx_identity(sink: &mut dyn Sink, value: &EffectIdentity) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    match value {
        EffectIdentity::Native { effect_id } => {
            tx_u8(sink, 1, 1)?;
            tx_id(sink, 2, effect_id)
        }
        EffectIdentity::ThirdPartyCid { cid } => {
            tx_u8(sink, 1, 2)?;
            tx_text(sink, 2, cid)
        }
    }
}
fn tx_route_source(sink: &mut dyn Sink, value: &RouteSource) -> Result<(), EncodeError> {
    match value {
        RouteSource::Track { track_id, tap } => {
            tx_start_message(sink, 3)?;
            tx_u8(sink, 1, 1)?;
            tx_id(sink, 2, track_id)?;
            tx_u8(sink, 3, enum_tap(*tap))
        }
        RouteSource::SubmixOutput { submix_id } => {
            tx_start_message(sink, 2)?;
            tx_u8(sink, 1, 2)?;
            tx_id(sink, 2, submix_id)
        }
    }
}
fn tx_route_destination(sink: &mut dyn Sink, value: &RouteDestination) -> Result<(), EncodeError> {
    tx_start_message(sink, 2)?;
    match value {
        RouteDestination::SubmixInput { submix_id } => {
            tx_u8(sink, 1, 1)?;
            tx_id(sink, 2, submix_id)
        }
        RouteDestination::OutputInput { output_id } => {
            tx_u8(sink, 1, 2)?;
            tx_id(sink, 2, output_id)
        }
    }
}
fn tx_sidechain(sink: &mut dyn Sink, value: &SidechainDeclaration) -> Result<(), EncodeError> {
    match value {
        SidechainDeclaration::None => {
            tx_start_message(sink, 1)?;
            tx_u8(sink, 1, 1)
        }
        SidechainDeclaration::Routed(value) => {
            tx_start_message(sink, 3)?;
            tx_u8(sink, 1, 2)?;
            tx_message(sink, 2, |v| tx_route_source(v, &value.source))?;
            tx_id(sink, 3, &value.port_id)
        }
    }
}
fn tx_param(sink: &mut dyn Sink, value: &EffectParam) -> Result<(), EncodeError> {
    tx_start_message(sink, 4)?;
    tx_u32(sink, 1, value.parameter_id)?;
    tx_u8(sink, 2, enum_channel(value.channel))?;
    tx_u8(sink, 3, enum_unit(value.unit))?;
    tx_f32(sink, 4, value.value)
}
fn tx_effect(sink: &mut dyn Sink, value: &Effect) -> Result<(), EncodeError> {
    tx_start_message(sink, tx_count(6, value.params.len())?)?;
    tx_id(sink, 1, &value.id)?;
    tx_message(sink, 2, |v| tx_identity(v, &value.identity))?;
    tx_u8(sink, 3, enum_quality(value.quality))?;
    tx_bool(sink, 4, value.bypass)?;
    tx_u8(sink, 5, enum_link(value.link_mode))?;
    for param in &value.params {
        tx_message(sink, 6, |v| tx_param(v, param))?;
    }
    tx_message(sink, 7, |v| tx_sidechain(v, &value.sidechain))
}
fn tx_fader(sink: &mut dyn Sink, value: &DualMonoFader) -> Result<(), EncodeError> {
    tx_start_message(sink, 4)?;
    tx_f32(sink, 1, value.left_db)?;
    tx_f32(sink, 2, value.right_db)?;
    tx_bool(sink, 3, value.left_mute)?;
    tx_bool(sink, 4, value.right_mute)
}
fn tx_matrix_or_pan(sink: &mut dyn Sink, value: &MatrixOrPan) -> Result<(), EncodeError> {
    match value {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => {
            tx_start_message(sink, 4)?;
            tx_u8(sink, 1, 1)?;
            tx_f32(sink, 2, *left)?;
            tx_f32(sink, 3, *right)?;
            tx_u32(sink, 4, *smoothing_samples)
        }
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => {
            tx_start_message(sink, 6)?;
            tx_u8(sink, 1, 2)?;
            tx_f32(sink, 2, *ll)?;
            tx_f32(sink, 3, *lr)?;
            tx_f32(sink, 4, *rl)?;
            tx_f32(sink, 5, *rr)?;
            tx_u32(sink, 6, *smoothing_samples)
        }
    }
}
fn tx_track(sink: &mut dyn Sink, value: &miso_engine_session::Track) -> Result<(), EncodeError> {
    tx_start_message(sink, 10)?;
    tx_id(sink, 1, &value.id)?;
    tx_id(sink, 2, &value.source_id)?;
    tx_u8(sink, 3, value.left_source_channel)?;
    tx_u8(sink, 4, value.right_source_channel)?;
    tx_message(sink, 5, |v| tx_builtins(v, &value.builtins))?;
    tx_message(sink, 6, |v| tx_rack(v, &value.simd1))?;
    tx_message(sink, 7, |v| tx_rack(v, &value.dynamic))?;
    tx_message(sink, 8, |v| tx_rack(v, &value.simd2))?;
    tx_message(sink, 9, |v| tx_fader(v, &value.fader))?;
    tx_message(sink, 10, |v| tx_matrix_or_pan(v, &value.matrix_or_pan))
}
fn tx_submix(sink: &mut dyn Sink, value: &Submix) -> Result<(), EncodeError> {
    tx_start_message(sink, 1)?;
    tx_id(sink, 1, &value.id)
}
fn tx_output(sink: &mut dyn Sink, value: &Output) -> Result<(), EncodeError> {
    tx_start_message(sink, 1)?;
    tx_id(sink, 1, &value.id)
}
fn tx_channel_matrix(sink: &mut dyn Sink, value: &ChannelMatrix) -> Result<(), EncodeError> {
    tx_start_message(sink, 4)?;
    tx_f32(sink, 1, value.ll)?;
    tx_f32(sink, 2, value.lr)?;
    tx_f32(sink, 3, value.rl)?;
    tx_f32(sink, 4, value.rr)
}
fn tx_route(sink: &mut dyn Sink, value: &Route) -> Result<(), EncodeError> {
    tx_start_message(sink, 5)?;
    tx_id(sink, 1, &value.id)?;
    tx_message(sink, 2, |v| tx_route_source(v, &value.source))?;
    tx_message(sink, 3, |v| tx_route_destination(v, &value.destination))?;
    tx_message(sink, 4, |v| tx_channel_matrix(v, &value.channel_matrix))?;
    tx_f32(sink, 5, value.gain_db)
}
fn tx_automation_target(sink: &mut dyn Sink, value: &AutomationTarget) -> Result<(), EncodeError> {
    tx_start_message(sink, 5)?;
    tx_id(sink, 1, &value.entity_id)?;
    tx_u8(sink, 2, enum_rack(value.rack))?;
    tx_id(sink, 3, &value.effect_id)?;
    tx_u32(sink, 4, value.parameter_id)?;
    tx_u8(sink, 5, enum_channel(value.channel))
}
fn tx_automation_segment(
    sink: &mut dyn Sink,
    value: &AutomationSegment,
) -> Result<(), EncodeError> {
    tx_start_message(sink, 6)?;
    tx_u8(sink, 1, enum_shape(value.shape))?;
    tx_u64(sink, 2, value.start_sample)?;
    tx_u64(sink, 3, value.end_sample)?;
    tx_f32(sink, 4, value.start_value)?;
    tx_f32(sink, 5, value.end_value)?;
    tx_u8(sink, 6, enum_unit(value.unit))
}
fn tx_automation(sink: &mut dyn Sink, value: &Automation) -> Result<(), EncodeError> {
    tx_start_message(sink, tx_count(2, value.segments.len())?)?;
    tx_id(sink, 1, &value.id)?;
    tx_message(sink, 2, |v| tx_automation_target(v, &value.target))?;
    for segment in &value.segments {
        tx_message(sink, 3, |v| tx_automation_segment(v, segment))?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub(crate) struct Field<'a> {
    pub(crate) id: u16,
    pub(crate) wire: u8,
    pub(crate) mandatory: bool,
    pub(crate) value: &'a [u8],
}

pub(crate) struct Message<'a> {
    pub(crate) fields: Vec<Field<'a>>,
}

#[derive(Clone, Copy)]
pub(crate) struct Rule {
    id: u16,
    repeated: bool,
    mandatory: bool,
}

impl Rule {
    pub(crate) const fn one(id: u16) -> Self {
        Self {
            id,
            repeated: false,
            mandatory: true,
        }
    }
    pub(crate) const fn optional(id: u16) -> Self {
        Self {
            id,
            repeated: false,
            mandatory: false,
        }
    }
    pub(crate) const fn repeated(id: u16) -> Self {
        Self {
            id,
            repeated: true,
            mandatory: true,
        }
    }
    pub(crate) const fn optional_repeated(id: u16) -> Self {
        Self {
            id,
            repeated: true,
            mandatory: false,
        }
    }
}

impl<'a> Message<'a> {
    pub(crate) fn nested(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let header = bytes.get(..8).ok_or(DecodeError::Truncated)?;
        let count = read_u32(header, 0)?;
        if read_u32(header, 4)? != 0 {
            return Err(DecodeError::NonzeroReserved);
        }
        Self::tlvs(&bytes[8..], count)
    }

    pub(crate) fn tlvs(bytes: &'a [u8], count: u32) -> Result<Self, DecodeError> {
        let mut cursor = 0usize;
        let mut fields =
            Vec::with_capacity(usize::try_from(count).map_err(|_| DecodeError::LimitExceeded)?);
        let mut prior = 0_u16;
        for index in 0..count {
            let prefix = bytes
                .get(cursor..cursor.checked_add(8).ok_or(DecodeError::LimitExceeded)?)
                .ok_or(DecodeError::Truncated)?;
            let id = read_u16(prefix, 0)?;
            let wire = prefix[2];
            if id == 0
                || !(1..=15).contains(&wire)
                || prefix[3] & !1 != 0
                || (index != 0 && id < prior)
            {
                return Err(DecodeError::InvalidTlv);
            }
            prior = id;
            let length =
                usize::try_from(read_u32(prefix, 4)?).map_err(|_| DecodeError::LimitExceeded)?;
            let value_start = cursor.checked_add(8).ok_or(DecodeError::LimitExceeded)?;
            let value_end = value_start
                .checked_add(length)
                .ok_or(DecodeError::LimitExceeded)?;
            let value = bytes
                .get(value_start..value_end)
                .ok_or(DecodeError::Truncated)?;
            let padded_end = value_end
                .checked_add(padding(length))
                .ok_or(DecodeError::LimitExceeded)?;
            if bytes
                .get(value_end..padded_end)
                .ok_or(DecodeError::Truncated)?
                .iter()
                .any(|byte| *byte != 0)
            {
                return Err(DecodeError::InvalidTlv);
            }
            fields.push(Field {
                id,
                wire,
                mandatory: prefix[3] & 1 != 0,
                value,
            });
            cursor = padded_end;
        }
        if cursor != bytes.len() {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(Self { fields })
    }

    pub(crate) fn schema(&self, rules: &[Rule]) -> Result<(), DecodeError> {
        for field in &self.fields {
            let rule = rules.iter().find(|rule| rule.id == field.id);
            match rule {
                Some(rule) if field.mandatory != rule.mandatory => {
                    return Err(DecodeError::InvalidTlv);
                }
                None if field.mandatory => return Err(DecodeError::UnknownRequiredField),
                _ => {}
            }
        }
        for rule in rules {
            if !rule.repeated
                && self
                    .fields
                    .iter()
                    .filter(|field| field.id == rule.id)
                    .count()
                    > 1
            {
                return Err(DecodeError::InvalidTlv);
            }
        }
        Ok(())
    }

    /// Apply a tagged-variant schema. Optional extensions remain skippable, but a field ID that
    /// belongs to this tagged message and not to the selected variant is never an extension.
    pub(crate) fn tagged_schema(
        &self,
        rules: &[Rule],
        known_ids: &[u16],
    ) -> Result<(), DecodeError> {
        self.schema(rules)?;
        if self.fields.iter().any(|field| {
            known_ids.contains(&field.id) && !rules.iter().any(|rule| rule.id == field.id)
        }) {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(())
    }

    pub(crate) fn one(&self, id: u16, wire: u8) -> Result<&'a [u8], DecodeError> {
        let mut values = self.fields.iter().filter(|field| field.id == id);
        let Some(field) = values.next() else {
            return Err(DecodeError::InvalidTlv);
        };
        if values.next().is_some() || field.wire != wire {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(field.value)
    }

    pub(crate) fn optional_one(&self, id: u16, wire: u8) -> Result<Option<&'a [u8]>, DecodeError> {
        let mut values = self.fields.iter().filter(|field| field.id == id);
        let Some(field) = values.next() else {
            return Ok(None);
        };
        if values.next().is_some() || field.wire != wire {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(Some(field.value))
    }

    pub(crate) fn values(
        &self,
        id: u16,
        wire: u8,
    ) -> Result<impl Iterator<Item = &'a [u8]> + '_, DecodeError> {
        if self
            .fields
            .iter()
            .any(|field| field.id == id && field.wire != wire)
        {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(self
            .fields
            .iter()
            .filter(move |field| field.id == id)
            .map(|field| field.value))
    }
}

fn parse_edit(message: Message<'_>) -> Result<SessionEditV1, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    let opcode = read_u16_exact(message.one(1, WIRE_U16)?)?;
    let payload = Message::nested(message.one(2, WIRE_MESSAGE)?)?;
    match crate::SessionEditOpcode::from_raw(opcode).ok_or(DecodeError::InvalidTlv)? {
        crate::SessionEditOpcode::SetSessionId => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetSessionId {
                session_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::SetSampleRateHz => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetSampleRateHz {
                sample_rate_hz: read_u32_exact(payload.one(1, WIRE_U32)?)?,
            })
        }
        crate::SessionEditOpcode::SetQuantumFrames => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetQuantumFrames {
                quantum_frames: read_u32_exact(payload.one(1, WIRE_U32)?)?,
            })
        }
        crate::SessionEditOpcode::SetRenderProfile => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetRenderProfile {
                render_profile: parse_render_profile(Message::nested(
                    payload.one(1, WIRE_MESSAGE)?,
                )?)?,
            })
        }
        crate::SessionEditOpcode::SetOutputProfile => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetOutputProfile {
                output_profile: parse_output_profile(Message::nested(
                    payload.one(1, WIRE_MESSAGE)?,
                )?)?,
            })
        }
        crate::SessionEditOpcode::SetLimits => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::SetLimits {
                limits: parse_limits(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertSource => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertSource {
                source: parse_source(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveSource => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveSource {
                source_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::SetSourceSampleRateHz => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetSourceSampleRateHz {
                source_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                sample_rate_hz: read_u32_exact(payload.one(2, WIRE_U32)?)?,
            })
        }
        crate::SessionEditOpcode::SetSourceContent => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetSourceContent {
                source_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                content: parse_content(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetSourceMapping => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetSourceMapping {
                source_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                mapping: parse_mapping(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertTrack => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertTrack {
                track: parse_track(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveTrack => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveTrack {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackSourceAssignment => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            Ok(SessionEditV1::SetTrackSourceAssignment {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                source_id: stable_id(payload.one(2, WIRE_UTF8)?)?,
                left_source_channel: read_u8_exact(payload.one(3, WIRE_U8)?)?,
                right_source_channel: read_u8_exact(payload.one(4, WIRE_U8)?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackBuiltins => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetTrackBuiltins {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                builtins: parse_builtins(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackRack => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3)])?;
            Ok(SessionEditV1::SetTrackRack {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                rack_name: parse_rack(read_u8_exact(payload.one(2, WIRE_U8)?)?)?,
                rack: parse_rack_message(Message::nested(payload.one(3, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::PutTrackEffect => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            Ok(SessionEditV1::PutTrackEffect {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                rack_name: parse_rack(read_u8_exact(payload.one(2, WIRE_U8)?)?)?,
                final_position: read_u32_exact(payload.one(3, WIRE_U32)?)?,
                effect: parse_effect(Message::nested(payload.one(4, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveTrackEffect => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::RemoveTrackEffect {
                track_id,
                rack_name,
                effect_id,
            })
        }
        crate::SessionEditOpcode::SetTrackEffectOrder => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::repeated(3)])?;
            Ok(SessionEditV1::SetTrackEffectOrder {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                rack_name: parse_rack(read_u8_exact(payload.one(2, WIRE_U8)?)?)?,
                effect_ids: payload
                    .values(3, WIRE_UTF8)?
                    .map(stable_id)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        crate::SessionEditOpcode::SetEffectIdentity => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::SetEffectIdentity {
                track_id,
                rack_name,
                effect_id,
                identity: parse_identity(Message::nested(payload.one(4, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectQuality => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::SetEffectQuality {
                track_id,
                rack_name,
                effect_id,
                quality: parse_quality(read_u8_exact(payload.one(4, WIRE_U8)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectBypass => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::SetEffectBypass {
                track_id,
                rack_name,
                effect_id,
                bypass: parse_bool(payload.one(4, WIRE_BOOL)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectLinkMode => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::SetEffectLinkMode {
                track_id,
                rack_name,
                effect_id,
                link_mode: parse_link(read_u8_exact(payload.one(4, WIRE_U8)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetEffectSidechain => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::SetEffectSidechain {
                track_id,
                rack_name,
                effect_id,
                sidechain: parse_sidechain(Message::nested(payload.one(4, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertEffectParam => {
            payload.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
            let (track_id, rack_name, effect_id) = parse_track_effect_ref(&payload)?;
            Ok(SessionEditV1::UpsertEffectParam {
                track_id,
                rack_name,
                effect_id,
                param: parse_param(Message::nested(payload.one(4, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveEffectParam => {
            payload.schema(&[
                Rule::one(1),
                Rule::one(2),
                Rule::one(3),
                Rule::one(4),
                Rule::one(5),
            ])?;
            Ok(SessionEditV1::RemoveEffectParam {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                rack_name: parse_rack(read_u8_exact(payload.one(2, WIRE_U8)?)?)?,
                effect_id: stable_id(payload.one(3, WIRE_UTF8)?)?,
                parameter_id: read_u32_exact(payload.one(4, WIRE_U32)?)?,
                channel: parse_channel(read_u8_exact(payload.one(5, WIRE_U8)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackFader => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetTrackFader {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                fader: parse_fader(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetTrackMatrixOrPan => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetTrackMatrixOrPan {
                track_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                matrix_or_pan: parse_matrix_or_pan(Message::nested(
                    payload.one(2, WIRE_MESSAGE)?,
                )?)?,
            })
        }
        crate::SessionEditOpcode::UpsertSubmix => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertSubmix {
                submix: parse_submix(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveSubmix => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveSubmix {
                submix_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertOutput => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertOutput {
                output: parse_output(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveOutput => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveOutput {
                output_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertRoute => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertRoute {
                route: parse_route(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveRoute => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveRoute {
                route_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::SetRouteSource => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetRouteSource {
                route_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                source: parse_route_source(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetRouteDestination => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetRouteDestination {
                route_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                destination: parse_route_destination(Message::nested(
                    payload.one(2, WIRE_MESSAGE)?,
                )?)?,
            })
        }
        crate::SessionEditOpcode::SetRouteChannelMatrix => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetRouteChannelMatrix {
                route_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                channel_matrix: parse_channel_matrix(Message::nested(
                    payload.one(2, WIRE_MESSAGE)?,
                )?)?,
            })
        }
        crate::SessionEditOpcode::SetRouteGainDb => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetRouteGainDb {
                route_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                gain_db: read_f32_exact(payload.one(2, WIRE_F32)?)?,
            })
        }
        crate::SessionEditOpcode::UpsertAutomation => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::UpsertAutomation {
                automation: parse_automation(Message::nested(payload.one(1, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::RemoveAutomation => {
            payload.schema(&[Rule::one(1)])?;
            Ok(SessionEditV1::RemoveAutomation {
                automation_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
            })
        }
        crate::SessionEditOpcode::SetAutomationTarget => {
            payload.schema(&[Rule::one(1), Rule::one(2)])?;
            Ok(SessionEditV1::SetAutomationTarget {
                automation_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                target: parse_automation_target(Message::nested(payload.one(2, WIRE_MESSAGE)?)?)?,
            })
        }
        crate::SessionEditOpcode::SetAutomationSegments => {
            payload.schema(&[Rule::one(1), Rule::repeated(2)])?;
            Ok(SessionEditV1::SetAutomationSegments {
                automation_id: stable_id(payload.one(1, WIRE_UTF8)?)?,
                segments: payload
                    .values(2, WIRE_MESSAGE)?
                    .map(|value| parse_automation_segment(Message::nested(value)?))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
    }
}

fn parse_render_profile(message: Message<'_>) -> Result<RenderProfile, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    let mode = match read_u8_exact(message.one(2, WIRE_U8)?)? {
        1 => miso_engine_session::RenderMode::SingleThread,
        2 => miso_engine_session::RenderMode::DependencyWaves,
        _ => return Err(DecodeError::InvalidTlv),
    };
    Ok(RenderProfile {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        mode,
    })
}

fn parse_output_profile(message: Message<'_>) -> Result<OutputProfile, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3)])?;
    if read_u8_exact(message.one(3, WIRE_U8)?)? != 1 {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(OutputProfile {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        channels: read_u8_exact(message.one(2, WIRE_U8)?)?,
        sample_format: miso_engine_session::SampleFormat::F32Planar,
    })
}

fn parse_limits(message: Message<'_>) -> Result<SessionLimits, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3)])?;
    Ok(SessionLimits {
        pcm_ring_frames: read_u64_exact(message.one(1, WIRE_U64)?)?,
        control_queue_messages: read_u64_exact(message.one(2, WIRE_U64)?)?,
        memory_bytes: read_u64_exact(message.one(3, WIRE_U64)?)?,
    })
}

fn parse_content(message: Message<'_>) -> Result<SourceContent, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    Ok(SourceContent {
        identity: utf8(message.one(1, WIRE_UTF8)?)?,
        locator: utf8(message.one(2, WIRE_UTF8)?)?,
    })
}

fn parse_region(message: Message<'_>) -> Result<SourceRegion, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    Ok(SourceRegion {
        start_sample: read_u64_exact(message.one(1, WIRE_U64)?)?,
        length_samples: read_u64_exact(message.one(2, WIRE_U64)?)?,
    })
}

fn parse_mapping(message: Message<'_>) -> Result<SourceMapping, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    Ok(SourceMapping {
        channel_count: read_u8_exact(message.one(1, WIRE_U8)?)?,
        region: parse_region(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
    })
}

fn parse_source(message: Message<'_>) -> Result<Source, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    Ok(Source {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        sample_rate_hz: read_u32_exact(message.one(2, WIRE_U32)?)?,
        content: parse_content(Message::nested(message.one(3, WIRE_MESSAGE)?)?)?,
        mapping: parse_mapping(Message::nested(message.one(4, WIRE_MESSAGE)?)?)?,
    })
}

fn parse_builtins(message: Message<'_>) -> Result<DualMonoBuiltins, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2)])?;
    Ok(DualMonoBuiltins {
        left: parse_channel_builtins(Message::nested(message.one(1, WIRE_MESSAGE)?)?)?,
        right: parse_channel_builtins(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
    })
}
fn parse_channel_builtins(message: Message<'_>) -> Result<ChannelBuiltins, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    Ok(ChannelBuiltins {
        polarity_invert: parse_bool(message.one(1, WIRE_BOOL)?)?,
        trim_db: read_f32_exact(message.one(2, WIRE_F32)?)?,
        hpf_hz: read_f32_exact(message.one(3, WIRE_F32)?)?,
        lpf_hz: read_f32_exact(message.one(4, WIRE_F32)?)?,
    })
}
fn parse_track(message: Message<'_>) -> Result<miso_engine_session::Track, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
        Rule::one(6),
        Rule::one(7),
        Rule::one(8),
        Rule::one(9),
        Rule::one(10),
    ])?;
    Ok(miso_engine_session::Track {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        source_id: stable_id(message.one(2, WIRE_UTF8)?)?,
        left_source_channel: read_u8_exact(message.one(3, WIRE_U8)?)?,
        right_source_channel: read_u8_exact(message.one(4, WIRE_U8)?)?,
        builtins: parse_builtins(Message::nested(message.one(5, WIRE_MESSAGE)?)?)?,
        simd1: parse_rack_message(Message::nested(message.one(6, WIRE_MESSAGE)?)?)?,
        dynamic: parse_rack_message(Message::nested(message.one(7, WIRE_MESSAGE)?)?)?,
        simd2: parse_rack_message(Message::nested(message.one(8, WIRE_MESSAGE)?)?)?,
        fader: parse_fader(Message::nested(message.one(9, WIRE_MESSAGE)?)?)?,
        matrix_or_pan: parse_matrix_or_pan(Message::nested(message.one(10, WIRE_MESSAGE)?)?)?,
    })
}
fn parse_rack_message(message: Message<'_>) -> Result<Rack, DecodeError> {
    message.schema(&[Rule::repeated(1)])?;
    Ok(Rack {
        effects: message
            .values(1, WIRE_MESSAGE)?
            .map(|value| parse_effect(Message::nested(value)?))
            .collect::<Result<Vec<_>, _>>()?,
    })
}
fn parse_identity(message: Message<'_>) -> Result<EffectIdentity, DecodeError> {
    match read_u8_exact(message.one(1, WIRE_U8)?)? {
        1 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2)], &[1, 2])?;
            Ok(EffectIdentity::Native {
                effect_id: stable_id(message.one(2, WIRE_UTF8)?)?,
            })
        }
        2 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2)], &[1, 2])?;
            Ok(EffectIdentity::ThirdPartyCid {
                cid: utf8(message.one(2, WIRE_UTF8)?)?,
            })
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_route_source(message: Message<'_>) -> Result<RouteSource, DecodeError> {
    match read_u8_exact(message.one(1, WIRE_U8)?)? {
        1 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2), Rule::one(3)], &[1, 2, 3])?;
            Ok(RouteSource::Track {
                track_id: stable_id(message.one(2, WIRE_UTF8)?)?,
                tap: parse_tap(read_u8_exact(message.one(3, WIRE_U8)?)?)?,
            })
        }
        2 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2)], &[1, 2, 3])?;
            Ok(RouteSource::SubmixOutput {
                submix_id: stable_id(message.one(2, WIRE_UTF8)?)?,
            })
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_route_destination(message: Message<'_>) -> Result<RouteDestination, DecodeError> {
    match read_u8_exact(message.one(1, WIRE_U8)?)? {
        1 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2)], &[1, 2])?;
            Ok(RouteDestination::SubmixInput {
                submix_id: stable_id(message.one(2, WIRE_UTF8)?)?,
            })
        }
        2 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2)], &[1, 2])?;
            Ok(RouteDestination::OutputInput {
                output_id: stable_id(message.one(2, WIRE_UTF8)?)?,
            })
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_sidechain(message: Message<'_>) -> Result<SidechainDeclaration, DecodeError> {
    match read_u8_exact(message.one(1, WIRE_U8)?)? {
        1 => {
            message.tagged_schema(&[Rule::one(1)], &[1, 2, 3])?;
            Ok(SidechainDeclaration::None)
        }
        2 => {
            message.tagged_schema(&[Rule::one(1), Rule::one(2), Rule::one(3)], &[1, 2, 3])?;
            Ok(SidechainDeclaration::Routed(
                miso_engine_session::Sidechain {
                    source: parse_route_source(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
                    port_id: stable_id(message.one(3, WIRE_UTF8)?)?,
                },
            ))
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_param(message: Message<'_>) -> Result<EffectParam, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    Ok(EffectParam {
        parameter_id: read_u32_exact(message.one(1, WIRE_U32)?)?,
        channel: parse_channel(read_u8_exact(message.one(2, WIRE_U8)?)?)?,
        unit: parse_unit(read_u8_exact(message.one(3, WIRE_U8)?)?)?,
        value: read_f32_exact(message.one(4, WIRE_F32)?)?,
    })
}
fn parse_effect(message: Message<'_>) -> Result<Effect, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
        Rule::repeated(6),
        Rule::one(7),
    ])?;
    Ok(Effect {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        identity: parse_identity(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
        quality: parse_quality(read_u8_exact(message.one(3, WIRE_U8)?)?)?,
        bypass: parse_bool(message.one(4, WIRE_BOOL)?)?,
        link_mode: parse_link(read_u8_exact(message.one(5, WIRE_U8)?)?)?,
        params: message
            .values(6, WIRE_MESSAGE)?
            .map(|value| parse_param(Message::nested(value)?))
            .collect::<Result<Vec<_>, _>>()?,
        sidechain: parse_sidechain(Message::nested(message.one(7, WIRE_MESSAGE)?)?)?,
    })
}
fn parse_fader(message: Message<'_>) -> Result<DualMonoFader, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    Ok(DualMonoFader {
        left_db: read_f32_exact(message.one(1, WIRE_F32)?)?,
        right_db: read_f32_exact(message.one(2, WIRE_F32)?)?,
        left_mute: parse_bool(message.one(3, WIRE_BOOL)?)?,
        right_mute: parse_bool(message.one(4, WIRE_BOOL)?)?,
    })
}
fn parse_matrix_or_pan(message: Message<'_>) -> Result<MatrixOrPan, DecodeError> {
    match read_u8_exact(message.one(1, WIRE_U8)?)? {
        1 => {
            message.tagged_schema(
                &[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)],
                &[1, 2, 3, 4, 5, 6],
            )?;
            Ok(MatrixOrPan::Pan {
                left: read_f32_exact(message.one(2, WIRE_F32)?)?,
                right: read_f32_exact(message.one(3, WIRE_F32)?)?,
                smoothing_samples: read_u32_exact(message.one(4, WIRE_U32)?)?,
            })
        }
        2 => {
            message.tagged_schema(
                &[
                    Rule::one(1),
                    Rule::one(2),
                    Rule::one(3),
                    Rule::one(4),
                    Rule::one(5),
                    Rule::one(6),
                ],
                &[1, 2, 3, 4, 5, 6],
            )?;
            Ok(MatrixOrPan::Matrix {
                ll: read_f32_exact(message.one(2, WIRE_F32)?)?,
                lr: read_f32_exact(message.one(3, WIRE_F32)?)?,
                rl: read_f32_exact(message.one(4, WIRE_F32)?)?,
                rr: read_f32_exact(message.one(5, WIRE_F32)?)?,
                smoothing_samples: read_u32_exact(message.one(6, WIRE_U32)?)?,
            })
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_submix(message: Message<'_>) -> Result<Submix, DecodeError> {
    message.schema(&[Rule::one(1)])?;
    Ok(Submix {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
    })
}
fn parse_output(message: Message<'_>) -> Result<Output, DecodeError> {
    message.schema(&[Rule::one(1)])?;
    Ok(Output {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
    })
}
fn parse_channel_matrix(message: Message<'_>) -> Result<ChannelMatrix, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::one(3), Rule::one(4)])?;
    Ok(ChannelMatrix {
        ll: read_f32_exact(message.one(1, WIRE_F32)?)?,
        lr: read_f32_exact(message.one(2, WIRE_F32)?)?,
        rl: read_f32_exact(message.one(3, WIRE_F32)?)?,
        rr: read_f32_exact(message.one(4, WIRE_F32)?)?,
    })
}
fn parse_route(message: Message<'_>) -> Result<Route, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
    ])?;
    Ok(Route {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        source: parse_route_source(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
        destination: parse_route_destination(Message::nested(message.one(3, WIRE_MESSAGE)?)?)?,
        channel_matrix: parse_channel_matrix(Message::nested(message.one(4, WIRE_MESSAGE)?)?)?,
        gain_db: read_f32_exact(message.one(5, WIRE_F32)?)?,
    })
}
fn parse_automation_target(message: Message<'_>) -> Result<AutomationTarget, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
    ])?;
    Ok(AutomationTarget {
        entity_id: stable_id(message.one(1, WIRE_UTF8)?)?,
        rack: parse_rack(read_u8_exact(message.one(2, WIRE_U8)?)?)?,
        effect_id: stable_id(message.one(3, WIRE_UTF8)?)?,
        parameter_id: read_u32_exact(message.one(4, WIRE_U32)?)?,
        channel: parse_channel(read_u8_exact(message.one(5, WIRE_U8)?)?)?,
    })
}
fn parse_automation_segment(message: Message<'_>) -> Result<AutomationSegment, DecodeError> {
    message.schema(&[
        Rule::one(1),
        Rule::one(2),
        Rule::one(3),
        Rule::one(4),
        Rule::one(5),
        Rule::one(6),
    ])?;
    Ok(AutomationSegment {
        shape: parse_shape(read_u8_exact(message.one(1, WIRE_U8)?)?)?,
        start_sample: read_u64_exact(message.one(2, WIRE_U64)?)?,
        end_sample: read_u64_exact(message.one(3, WIRE_U64)?)?,
        start_value: read_f32_exact(message.one(4, WIRE_F32)?)?,
        end_value: read_f32_exact(message.one(5, WIRE_F32)?)?,
        unit: parse_unit(read_u8_exact(message.one(6, WIRE_U8)?)?)?,
    })
}
fn parse_automation(message: Message<'_>) -> Result<Automation, DecodeError> {
    message.schema(&[Rule::one(1), Rule::one(2), Rule::repeated(3)])?;
    Ok(Automation {
        id: stable_id(message.one(1, WIRE_UTF8)?)?,
        target: parse_automation_target(Message::nested(message.one(2, WIRE_MESSAGE)?)?)?,
        segments: message
            .values(3, WIRE_MESSAGE)?
            .map(|value| parse_automation_segment(Message::nested(value)?))
            .collect::<Result<Vec<_>, _>>()?,
    })
}
fn parse_track_effect_ref(
    message: &Message<'_>,
) -> Result<(StableId, RackName, StableId), DecodeError> {
    Ok((
        stable_id(message.one(1, WIRE_UTF8)?)?,
        parse_rack(read_u8_exact(message.one(2, WIRE_U8)?)?)?,
        stable_id(message.one(3, WIRE_UTF8)?)?,
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
fn parse_channel(value: u8) -> Result<miso_engine_session::ParameterChannel, DecodeError> {
    match value {
        1 => Ok(miso_engine_session::ParameterChannel::Left),
        2 => Ok(miso_engine_session::ParameterChannel::Right),
        3 => Ok(miso_engine_session::ParameterChannel::Both),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_unit(value: u8) -> Result<miso_engine_session::ParameterUnit, DecodeError> {
    match value {
        1 => Ok(miso_engine_session::ParameterUnit::Db),
        2 => Ok(miso_engine_session::ParameterUnit::Hz),
        3 => Ok(miso_engine_session::ParameterUnit::Milliseconds),
        4 => Ok(miso_engine_session::ParameterUnit::Samples),
        5 => Ok(miso_engine_session::ParameterUnit::Linear),
        6 => Ok(miso_engine_session::ParameterUnit::Ratio),
        _ => Err(DecodeError::InvalidTlv),
    }
}
fn parse_rack(value: u8) -> Result<RackName, DecodeError> {
    match value {
        1 => Ok(RackName::Simd1),
        2 => Ok(RackName::Dynamic),
        3 => Ok(RackName::Simd2),
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
fn read_u8_exact(bytes: &[u8]) -> Result<u8, DecodeError> {
    bytes
        .first()
        .copied()
        .filter(|_| bytes.len() == 1)
        .ok_or(DecodeError::InvalidValueLength)
}
fn read_u16_exact(bytes: &[u8]) -> Result<u16, DecodeError> {
    if bytes.len() != 2 {
        return Err(DecodeError::InvalidValueLength);
    }
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}
fn read_u32_exact(bytes: &[u8]) -> Result<u32, DecodeError> {
    if bytes.len() != 4 {
        return Err(DecodeError::InvalidValueLength);
    }
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
fn read_u64_exact(bytes: &[u8]) -> Result<u64, DecodeError> {
    if bytes.len() != 8 {
        return Err(DecodeError::InvalidValueLength);
    }
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}
fn read_f32_exact(bytes: &[u8]) -> Result<f32, DecodeError> {
    if bytes.len() != 4 {
        return Err(DecodeError::InvalidValueLength);
    }
    let value = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
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
fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, DecodeError> {
    read_u16_exact(
        bytes
            .get(offset..offset + 2)
            .ok_or(DecodeError::Truncated)?,
    )
}
fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, DecodeError> {
    read_u32_exact(
        bytes
            .get(offset..offset + 4)
            .ok_or(DecodeError::Truncated)?,
    )
}

const fn padding(length: usize) -> usize {
    (8 - (length & 7)) & 7
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_session::{
        LinkMode, ParameterChannel, ParameterUnit, RenderMode, SampleFormat, SendTap, Sidechain,
        Track, parse_session_toml,
    };

    fn id(value: &str) -> StableId {
        StableId::parse(value).expect("stable ID")
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
        let legacy = transaction_payload(&edits).expect("allocating convenience payload");
        assert_eq!(&output[crate::OUTER_HEADER_BYTES..], legacy);
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
            assert_eq!(&output[crate::OUTER_HEADER_BYTES..], legacy);
        }
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
        assert_eq!(tx_count(u32::MAX, 1), Err(EncodeError::LimitExceeded));
        let mut overflow =
            CountSink::with_length_for_test(usize::MAX, crate::ProtocolLimits::default());
        assert_eq!(overflow.raw(&[0]), Err(EncodeError::LimitExceeded));
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
        let mut duplicate = MessageBuilder::new();
        duplicate.boolean(1, false).expect("field");
        duplicate.boolean(1, false).expect("duplicate field");
        duplicate.f32(2, 0.0).expect("field");
        duplicate.f32(3, 20.0).expect("field");
        duplicate.f32(4, 20_000.0).expect("field");
        assert!(
            parse_channel_builtins(Message::nested(&duplicate.finish()).expect("nested")).is_err()
        );

        let mut wrong_type = MessageBuilder::new();
        wrong_type.u8(1, 0).expect("bad type");
        wrong_type.f32(2, 0.0).expect("field");
        wrong_type.f32(3, 20.0).expect("field");
        wrong_type.f32(4, 20_000.0).expect("field");
        assert!(
            parse_channel_builtins(Message::nested(&wrong_type.finish()).expect("nested")).is_err()
        );

        let mut unknown = MessageBuilder::new();
        unknown.u8(1, 1).expect("tag");
        unknown.u8(2, 7).expect("known-incompatible field");
        assert_eq!(
            parse_sidechain(Message::nested(&unknown.finish()).expect("nested")),
            Err(DecodeError::UnknownRequiredField)
        );

        let reversed = [
            2, 0, WIRE_U8, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, WIRE_U8, 1, 1, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(
            Message::tlvs(&reversed, 2).err(),
            Some(DecodeError::InvalidTlv)
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
            let encoded = route_source_message(&source).expect("encode route source");
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
        let mut destination = MessageBuilder::new();
        destination.u8(1, 2).expect("kind");
        destination.id(2, &id("main-out")).expect("output");
        let mut bytes = destination.finish();
        bytes.extend_from_slice(&[99, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
        bytes[0..4].copy_from_slice(&3_u32.to_le_bytes());
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
