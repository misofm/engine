//! Schema-closed complete BTLV frame encoding and decoding.
//!
//! This layer joins the frozen outer header to the existing typed payload registries. It does not
//! offer a raw message ID, field, or payload escape hatch: every public enum variant corresponds
//! to exactly one registered command, successful response, non-OK response, or event schema.

use crate::btlv::{CountSink, MessageMeasure, Sink, SliceSink};
use crate::{
    Capabilities, CommandHeader, ControlListPage, ControlListRequest, ControlStatePage,
    CounterSnapshot, CounterSnapshotRef, CountersRequest, DecodeError, DecodeScratch,
    DecodedCapabilities, DecodedMeterBatch, Diagnostic, DiagnosticEvent, DiagnosticsPage,
    DiagnosticsRequest, EncodeError, EventHeader, FrameKind, LoudnessReply, LoudnessRequest,
    MessageId, MeterBatch, NonOkResponse, OUTER_HEADER_BYTES, ObservationEnded,
    ObservationReadPage, ObservationReadRequest, ProtocolCodec, RequestId, ResponseGetRequest,
    ResponseHeader, ResponseReady, ResponseResult, SessionCommitted, SessionEdit,
    SessionReplaceRequest, SessionRevision, SessionSnapshot, SessionSnapshotRequest,
    SpectrumCaptureReply, SpectrumCaptureRequest, SpectrumConfigureRequest, SpectrumConfigured,
    SpectrumStreamReply, SpectrumStreamRequest, StatusCode, TelemetryConfiguration,
    TransactionApplied, TransportSetRequest, TransportSnapshot, TransportStateEvent,
};

#[cfg(test)]
std::thread_local! {
    static FRAME_COUNT_PASSES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static FRAME_SLICE_PASSES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn reset_frame_writer_passes() {
    FRAME_COUNT_PASSES.with(|passes| passes.set(0));
    FRAME_SLICE_PASSES.with(|passes| passes.set(0));
}

#[cfg(test)]
pub(crate) fn frame_writer_passes() -> (usize, usize) {
    (
        FRAME_COUNT_PASSES.with(core::cell::Cell::get),
        FRAME_SLICE_PASSES.with(core::cell::Cell::get),
    )
}

/// One schema-closed command payload ready for full-frame encoding.
#[allow(missing_docs)] // Variant names exactly mirror the frozen command registry.
pub enum CommandPayload<'a> {
    CapabilitiesGet,
    SessionSnapshotGet(SessionSnapshotRequest),
    SessionTransactionApply(&'a [SessionEdit]),
    TransportGet,
    TransportSet(TransportSetRequest),
    TelemetryConfigure(&'a TelemetryConfiguration),
    CountersGet(&'a CountersRequest),
    DiagnosticsGet(DiagnosticsRequest),
    ControlListGet(&'a ControlListRequest),
    ControlStateGet(&'a ControlListRequest),
    ResponseGet(&'a ResponseGetRequest),
    SessionReplace(SessionReplaceRequest),
    SpectrumConfigure(&'a SpectrumConfigureRequest),
    SpectrumCapture(&'a SpectrumCaptureRequest),
    SpectrumStream(&'a SpectrumStreamRequest),
    /// `LOUDNESS` (#2115).
    Loudness(LoudnessRequest),
    ObservationRead(&'a ObservationReadRequest),
}

/// One typed command frame with a command-only revision precondition.
#[allow(missing_docs)] // Header field names intentionally match the frozen outer header contract.
pub struct TypedCommandFrame<'a> {
    pub request_id: RequestId,
    pub expected_revision: crate::ExpectedRevision,
    pub payload: CommandPayload<'a>,
}

/// One schema-closed successful response payload.
#[allow(missing_docs)] // Variant names exactly mirror the frozen command registry.
pub enum SuccessResponsePayload<'a> {
    Capabilities(Capabilities<'a>),
    SessionSnapshot(SessionSnapshot<'a>),
    SessionTransactionApplied(TransactionApplied),
    TransportGetSnapshot(TransportSnapshot),
    TransportSetSnapshot(TransportSnapshot),
    TelemetryConfiguration(TelemetryConfiguration),
    CounterSnapshot(CounterSnapshot),
    DiagnosticsPage(DiagnosticsPage),
    ControlListPage(ControlListPage),
    ControlStatePage(ControlStatePage),
    ResponseResult(ResponseResult),
    /// The reply of a `SESSION_REPLACE`: the `SESSION_TRANSACTION_APPLY` reply's fields.
    SessionReplaced(TransactionApplied),
    SpectrumConfigured(SpectrumConfigured),
    SpectrumCaptured(SpectrumCaptureReply<'a>),
    SpectrumStreamed(SpectrumStreamReply<'a>),
    /// The `LOUDNESS` reply (#2115).
    Loudness(LoudnessReply),
    ObservationRead(ObservationReadPage<'a>),
}

/// One typed successful response frame. The payload selects and fixes the echoed command ID.
#[allow(missing_docs)]
pub struct TypedSuccessResponseFrame<'a> {
    pub request_id: RequestId,
    pub revision: SessionRevision,
    pub payload: SuccessResponsePayload<'a>,
}

/// One typed common non-OK response frame. The header message ID remains the failed command ID.
#[allow(missing_docs)]
pub struct TypedNonOkResponseFrame<'a> {
    pub request_id: RequestId,
    pub revision: SessionRevision,
    pub message_id: MessageId,
    pub status: StatusCode,
    pub payload: &'a NonOkResponse,
}

/// One schema-closed event payload.
#[allow(missing_docs)] // Variant names exactly mirror the frozen event registry.
pub enum EventPayload<'a> {
    SessionCommitted(SessionCommitted),
    TransportState(TransportStateEvent),
    ObservationEnded(&'a ObservationEnded),
    MeterBatch(MeterBatch<'a>),
    CounterSnapshot(CounterSnapshotRef<'a>),
    Diagnostic(&'a Diagnostic),
    ResponseReady(&'a ResponseReady),
}

/// One typed unsolicited event frame. The payload selects and fixes the event message ID.
#[allow(missing_docs)]
pub struct TypedEventFrame<'a> {
    pub revision: SessionRevision,
    pub payload: EventPayload<'a>,
}

/// A strictly decoded complete typed command payload.
#[allow(missing_docs)]
pub enum DecodedCommandPayload {
    CapabilitiesGet,
    SessionSnapshotGet(SessionSnapshotRequest),
    SessionTransactionApply(Vec<SessionEdit>),
    TransportGet,
    TransportSet(TransportSetRequest),
    TelemetryConfigure(TelemetryConfiguration),
    CountersGet(CountersRequest),
    DiagnosticsGet(DiagnosticsRequest),
    ControlListGet(ControlListRequest),
    ControlStateGet(ControlListRequest),
    ResponseGet(ResponseGetRequest),
    SessionReplace(SessionReplaceRequest),
    SpectrumConfigure(SpectrumConfigureRequest),
    SpectrumCapture(SpectrumCaptureRequest),
    SpectrumStream(SpectrumStreamRequest),
    /// `LOUDNESS` (#2115).
    Loudness(LoudnessRequest),
    ObservationRead(ObservationReadRequest),
}

/// A strictly decoded kind-specific command header and its exact registered payload.
#[allow(missing_docs)]
pub struct DecodedTypedCommandFrame {
    pub header: CommandHeader,
    pub payload: DecodedCommandPayload,
}

/// A strictly decoded successful response payload.
#[allow(missing_docs)]
pub enum DecodedSuccessResponsePayload<'a> {
    Capabilities(DecodedCapabilities<'a>),
    SessionSnapshot(SessionSnapshot<'a>),
    SessionTransactionApplied(TransactionApplied),
    TransportGetSnapshot(TransportSnapshot),
    TransportSetSnapshot(TransportSnapshot),
    TelemetryConfiguration(TelemetryConfiguration),
    CounterSnapshot(CounterSnapshot),
    DiagnosticsPage(DiagnosticsPage),
    ControlListPage(ControlListPage),
    ControlStatePage(ControlStatePage),
    ResponseResult(ResponseResult),
    SessionReplaced(TransactionApplied),
    SpectrumConfigured(SpectrumConfigured),
    SpectrumCaptured(SpectrumCaptureReply<'static>),
    SpectrumStreamed(SpectrumStreamReply<'static>),
    /// The `LOUDNESS` reply (#2115).
    Loudness(LoudnessReply),
    ObservationRead(ObservationReadPage<'static>),
}

/// A strictly decoded response payload selected by its non-OK status or successful schema.
#[allow(missing_docs)]
pub enum DecodedTypedResponseFrame<'a> {
    Success {
        header: ResponseHeader,
        payload: DecodedSuccessResponsePayload<'a>,
    },
    NonOk {
        header: ResponseHeader,
        payload: NonOkResponse,
    },
}

/// A strictly decoded event payload.
#[allow(missing_docs)]
pub enum DecodedEventPayload<'a> {
    SessionCommitted(SessionCommitted),
    TransportState(TransportStateEvent),
    ObservationEnded(ObservationEnded),
    MeterBatch(DecodedMeterBatch<'a>),
    CounterSnapshot(CounterSnapshot),
    Diagnostic(DiagnosticEvent),
    ResponseReady(ResponseReady),
}

/// A strictly decoded event header and its exact registered event payload.
#[allow(missing_docs)]
pub struct DecodedTypedEventFrame<'a> {
    pub header: EventHeader,
    pub payload: DecodedEventPayload<'a>,
}

impl<'a> CommandPayload<'a> {
    const fn message_id(&self) -> MessageId {
        match self {
            Self::CapabilitiesGet => MessageId::CapabilitiesGet,
            Self::SessionSnapshotGet(_) => MessageId::SessionSnapshotGet,
            Self::SessionTransactionApply(_) => MessageId::SessionTransactionApply,
            Self::TransportGet => MessageId::TransportGet,
            Self::TransportSet(_) => MessageId::TransportSet,
            Self::TelemetryConfigure(_) => MessageId::TelemetryConfigure,
            Self::CountersGet(_) => MessageId::CountersGet,
            Self::DiagnosticsGet(_) => MessageId::DiagnosticsGet,
            Self::ControlListGet(_) => MessageId::ControlListGet,
            Self::ControlStateGet(_) => MessageId::ControlStateGet,
            Self::ResponseGet(_) => MessageId::ResponseGet,
            Self::SessionReplace(_) => MessageId::SessionReplace,
            Self::SpectrumConfigure(_) => MessageId::SpectrumConfigure,
            Self::SpectrumCapture(_) => MessageId::SpectrumCapture,
            Self::SpectrumStream(_) => MessageId::SpectrumStream,
            Self::Loudness(_) => MessageId::Loudness,
            Self::ObservationRead(_) => MessageId::ObservationRead,
        }
    }
}

impl<'a> SuccessResponsePayload<'a> {
    const fn message_id(&self) -> MessageId {
        match self {
            Self::Capabilities(_) => MessageId::CapabilitiesGet,
            Self::SessionSnapshot(_) => MessageId::SessionSnapshotGet,
            Self::SessionTransactionApplied(_) => MessageId::SessionTransactionApply,
            Self::TransportGetSnapshot(_) => MessageId::TransportGet,
            Self::TransportSetSnapshot(_) => MessageId::TransportSet,
            Self::TelemetryConfiguration(_) => MessageId::TelemetryConfigure,
            Self::CounterSnapshot(_) => MessageId::CountersGet,
            Self::DiagnosticsPage(_) => MessageId::DiagnosticsGet,
            Self::ControlListPage(_) => MessageId::ControlListGet,
            Self::ControlStatePage(_) => MessageId::ControlStateGet,
            Self::ResponseResult(_) => MessageId::ResponseGet,
            Self::SessionReplaced(_) => MessageId::SessionReplace,
            Self::SpectrumConfigured(_) => MessageId::SpectrumConfigure,
            Self::SpectrumCaptured(_) => MessageId::SpectrumCapture,
            Self::SpectrumStreamed(_) => MessageId::SpectrumStream,
            Self::Loudness(_) => MessageId::Loudness,
            Self::ObservationRead(_) => MessageId::ObservationRead,
        }
    }
}

impl<'a> EventPayload<'a> {
    const fn message_id(&self) -> MessageId {
        match self {
            Self::SessionCommitted(_) => MessageId::SessionCommitted,
            Self::TransportState(_) => MessageId::TransportState,
            Self::ObservationEnded(_) => MessageId::ObservationEnded,
            Self::MeterBatch(_) => MessageId::MeterBatch,
            Self::CounterSnapshot(_) => MessageId::CounterSnapshot,
            Self::Diagnostic(_) => MessageId::Diagnostic,
            Self::ResponseReady(_) => MessageId::ResponseReady,
        }
    }
}

impl ProtocolCodec {
    /// Encode one complete command frame into caller-owned output without an arbitrary payload API.
    pub fn encode_command_frame_into(
        &self,
        frame: &TypedCommandFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if command_requires_exact_revision(&frame.payload)
            && !matches!(frame.expected_revision, crate::ExpectedRevision::Exact(_))
        {
            return Err(EncodeError::MessageKindMismatch);
        }
        if matches!(frame.payload, CommandPayload::CapabilitiesGet)
            && frame.expected_revision != crate::ExpectedRevision::Any
        {
            return Err(EncodeError::MessageKindMismatch);
        }
        if let CommandPayload::SessionTransactionApply(edits) = &frame.payload {
            return self.encode_session_transaction(
                &crate::SessionTransactionFrame {
                    request_id: frame.request_id,
                    expected_revision: frame.expected_revision,
                    edits,
                },
                output,
            );
        }
        let measure = measure_payload(self, |sink| write_command_payload(&frame.payload, sink))?;
        encode_complete_frame(
            self,
            FrameKind::Command,
            frame.payload.message_id(),
            StatusCode::Ok,
            frame.request_id.get(),
            command_revision(frame.expected_revision),
            command_flags(frame.expected_revision),
            measure,
            output,
            |sink| write_command_payload(&frame.payload, sink),
        )
    }

    /// Encode one complete successful response frame into caller-owned output.
    pub fn encode_success_response_frame_into(
        &self,
        frame: &TypedSuccessResponseFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let measure = self.measure_success_response_frame(frame)?;
        self.encode_success_response_frame_measured_into(frame, measure, output)
    }

    pub(crate) fn measure_success_response_frame(
        &self,
        frame: &TypedSuccessResponseFrame<'_>,
    ) -> Result<MessageMeasure, EncodeError> {
        measure_payload(self, |sink| {
            write_success_payload(self, &frame.payload, sink)
        })
    }

    pub(crate) fn encode_success_response_frame_measured_into(
        &self,
        frame: &TypedSuccessResponseFrame<'_>,
        measure: MessageMeasure,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        encode_complete_frame(
            self,
            FrameKind::Response,
            frame.payload.message_id(),
            StatusCode::Ok,
            frame.request_id.get(),
            frame.revision.0,
            0,
            measure,
            output,
            |sink| write_success_payload(self, &frame.payload, sink),
        )
    }

    /// Encode one complete common non-OK response frame into caller-owned output.
    pub fn encode_non_ok_response_frame_into(
        &self,
        frame: &TypedNonOkResponseFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if frame.message_id.raw() >= 0x8000
            || frame.status == StatusCode::Ok
            || (frame.status == StatusCode::Backpressure) != frame.payload.backpressure.is_some()
        {
            return Err(EncodeError::MessageKindMismatch);
        }
        let measure = measure_payload(self, |sink| {
            crate::message_wire::write_non_ok(self, sink, frame.payload)
        })?;
        encode_complete_frame(
            self,
            FrameKind::Response,
            frame.message_id,
            frame.status,
            frame.request_id.get(),
            frame.revision.0,
            0,
            measure,
            output,
            |sink| crate::message_wire::write_non_ok(self, sink, frame.payload),
        )
    }

    /// Encode one complete event frame into caller-owned output without an arbitrary payload API.
    pub fn encode_event_frame_into(
        &self,
        frame: &TypedEventFrame<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let measure =
            measure_payload(self, |sink| write_event_payload(self, &frame.payload, sink))?;
        encode_complete_frame(
            self,
            FrameKind::Event,
            frame.payload.message_id(),
            StatusCode::Ok,
            0,
            frame.revision.0,
            0,
            measure,
            output,
            |sink| write_event_payload(self, &frame.payload, sink),
        )
    }

    /// Decode a complete command and select its one exact registered schema.
    pub fn decode_typed_command(
        &self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedTypedCommandFrame, DecodeError> {
        self.decode_typed_command_with_transaction_limit(input, scratch, None, true)
    }

    pub(crate) fn decode_typed_command_limited(
        &self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        maximum_transaction_edits: u32,
    ) -> Result<DecodedTypedCommandFrame, DecodeError> {
        self.decode_typed_command_with_transaction_limit(
            input,
            scratch,
            Some(maximum_transaction_edits),
            false,
        )
    }

    fn decode_typed_command_with_transaction_limit(
        &self,
        input: &[u8],
        scratch: &mut DecodeScratch<'_>,
        maximum_transaction_edits: Option<u32>,
        enforce_exact_revision: bool,
    ) -> Result<DecodedTypedCommandFrame, DecodeError> {
        let decoded = self.decode_header(input)?;
        let header = decoded
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        scratch.prepare(header.tlv_count)?;
        if enforce_exact_revision
            && command_message_requires_exact(header.message_id)
            && !matches!(header.expected_revision, crate::ExpectedRevision::Exact(_))
        {
            return Err(DecodeError::InvalidTlv);
        }
        if header.message_id == MessageId::CapabilitiesGet
            && header.expected_revision != crate::ExpectedRevision::Any
        {
            return Err(DecodeError::InvalidTlv);
        }
        let payload = match header.message_id {
            MessageId::CapabilitiesGet => {
                self.decode_capabilities_request(decoded.payload, header.tlv_count)?;
                DecodedCommandPayload::CapabilitiesGet
            }
            MessageId::SessionSnapshotGet => DecodedCommandPayload::SessionSnapshotGet(
                self.decode_snapshot_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SessionTransactionApply => {
                let transaction = self
                    .decode_session_transaction_frame_limited(decoded, maximum_transaction_edits)?;
                DecodedCommandPayload::SessionTransactionApply(transaction.edits)
            }
            MessageId::TransportGet => {
                self.decode_transport_get_request(decoded.payload, header.tlv_count)?;
                DecodedCommandPayload::TransportGet
            }
            MessageId::TransportSet => DecodedCommandPayload::TransportSet(
                self.decode_transport_set_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::TelemetryConfigure => DecodedCommandPayload::TelemetryConfigure(
                self.decode_telemetry_configuration(decoded.payload, header.tlv_count)?,
            ),
            MessageId::CountersGet => DecodedCommandPayload::CountersGet(
                self.decode_counters_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::DiagnosticsGet => DecodedCommandPayload::DiagnosticsGet(
                self.decode_diagnostics_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ControlListGet => DecodedCommandPayload::ControlListGet(
                self.decode_control_list_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ControlStateGet => DecodedCommandPayload::ControlStateGet(
                self.decode_control_list_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ResponseGet => DecodedCommandPayload::ResponseGet(
                self.decode_response_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SessionReplace => DecodedCommandPayload::SessionReplace(
                self.decode_session_replace_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumConfigure => DecodedCommandPayload::SpectrumConfigure(
                self.decode_spectrum_configure_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumCapture => DecodedCommandPayload::SpectrumCapture(
                self.decode_spectrum_capture_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumStream => DecodedCommandPayload::SpectrumStream(
                self.decode_spectrum_stream_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::Loudness => DecodedCommandPayload::Loudness(
                self.decode_loudness_request(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ObservationRead => DecodedCommandPayload::ObservationRead(
                self.decode_observation_read_request(decoded.payload, header.tlv_count)?,
            ),
            _ => return Err(DecodeError::MessageKindMismatch),
        };
        Ok(DecodedTypedCommandFrame { header, payload })
    }

    /// Decode a complete response and select either its common non-OK or exact success schema.
    pub fn decode_typed_response<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedTypedResponseFrame<'a>, DecodeError> {
        let decoded = self.decode_header(input)?;
        let header = decoded
            .header
            .response()
            .ok_or(DecodeError::MessageKindMismatch)?;
        scratch.prepare(header.tlv_count)?;
        if header.status != StatusCode::Ok {
            let payload = self.decode_non_ok_payload(decoded.payload, header.tlv_count)?;
            if (header.status == StatusCode::Backpressure) != payload.backpressure.is_some() {
                return Err(DecodeError::InvalidTlv);
            }
            return Ok(DecodedTypedResponseFrame::NonOk { header, payload });
        }
        let payload = match header.message_id {
            MessageId::CapabilitiesGet => DecodedSuccessResponsePayload::Capabilities(
                self.decode_capabilities(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SessionSnapshotGet => DecodedSuccessResponsePayload::SessionSnapshot(
                self.decode_snapshot(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SessionTransactionApply => {
                DecodedSuccessResponsePayload::SessionTransactionApplied(
                    self.decode_transaction_applied(decoded.payload, header.tlv_count)?,
                )
            }
            MessageId::TransportGet => DecodedSuccessResponsePayload::TransportGetSnapshot(
                self.decode_transport_snapshot(decoded.payload, header.tlv_count)?,
            ),
            MessageId::TransportSet => DecodedSuccessResponsePayload::TransportSetSnapshot(
                self.decode_transport_snapshot(decoded.payload, header.tlv_count)?,
            ),
            MessageId::TelemetryConfigure => DecodedSuccessResponsePayload::TelemetryConfiguration(
                self.decode_telemetry_configuration_echo(decoded.payload, header.tlv_count)?,
            ),
            MessageId::CountersGet => DecodedSuccessResponsePayload::CounterSnapshot(
                self.decode_counter_snapshot(decoded.payload, header.tlv_count)?,
            ),
            MessageId::DiagnosticsGet => DecodedSuccessResponsePayload::DiagnosticsPage(
                self.decode_diagnostics_page(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ControlListGet => DecodedSuccessResponsePayload::ControlListPage(
                self.decode_control_list_page(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ControlStateGet => DecodedSuccessResponsePayload::ControlStatePage(
                self.decode_control_state_page(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ResponseGet => DecodedSuccessResponsePayload::ResponseResult(
                self.decode_response_result(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SessionReplace => DecodedSuccessResponsePayload::SessionReplaced(
                self.decode_transaction_applied(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumConfigure => DecodedSuccessResponsePayload::SpectrumConfigured(
                self.decode_spectrum_configured(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumCapture => DecodedSuccessResponsePayload::SpectrumCaptured(
                self.decode_spectrum_capture_reply(decoded.payload, header.tlv_count)?,
            ),
            MessageId::SpectrumStream => DecodedSuccessResponsePayload::SpectrumStreamed(
                self.decode_spectrum_stream_reply(decoded.payload, header.tlv_count)?,
            ),
            MessageId::Loudness => DecodedSuccessResponsePayload::Loudness(
                self.decode_loudness_reply(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ObservationRead => DecodedSuccessResponsePayload::ObservationRead(
                self.decode_observation_read_page(decoded.payload, header.tlv_count)?,
            ),
            _ => return Err(DecodeError::MessageKindMismatch),
        };
        Ok(DecodedTypedResponseFrame::Success { header, payload })
    }

    /// Decode a complete event and select its one exact registered event schema.
    pub fn decode_typed_event<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedTypedEventFrame<'a>, DecodeError> {
        let decoded = self.decode_header(input)?;
        let header = decoded
            .header
            .event()
            .ok_or(DecodeError::MessageKindMismatch)?;
        scratch.prepare(header.tlv_count)?;
        let payload = match header.message_id {
            MessageId::SessionCommitted => DecodedEventPayload::SessionCommitted(
                self.decode_session_committed(decoded.payload, header.tlv_count)?,
            ),
            MessageId::TransportState => DecodedEventPayload::TransportState(
                self.decode_transport_state_event(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ObservationEnded => DecodedEventPayload::ObservationEnded(
                self.decode_observation_ended(decoded.payload, header.tlv_count)?,
            ),
            MessageId::MeterBatch => DecodedEventPayload::MeterBatch(
                self.decode_meter_batch(decoded.payload, header.tlv_count)?,
            ),
            MessageId::CounterSnapshot => DecodedEventPayload::CounterSnapshot(
                self.decode_counter_snapshot_event(decoded.payload, header.tlv_count)?,
            ),
            MessageId::Diagnostic => DecodedEventPayload::Diagnostic(
                self.decode_diagnostic_event(decoded.payload, header.tlv_count)?,
            ),
            MessageId::ResponseReady => DecodedEventPayload::ResponseReady(
                self.decode_response_ready(decoded.payload, header.tlv_count)?,
            ),
            _ => return Err(DecodeError::MessageKindMismatch),
        };
        Ok(DecodedTypedEventFrame { header, payload })
    }
}

fn command_requires_exact_revision(payload: &CommandPayload<'_>) -> bool {
    command_message_requires_exact(payload.message_id())
}

const fn command_message_requires_exact(message_id: MessageId) -> bool {
    matches!(
        message_id,
        MessageId::SessionTransactionApply
            | MessageId::SessionReplace
            | MessageId::TransportSet
            | MessageId::TelemetryConfigure
    )
}

const fn command_revision(revision: crate::ExpectedRevision) -> u64 {
    match revision {
        crate::ExpectedRevision::Exact(value) => value.0,
        crate::ExpectedRevision::Any => 0,
    }
}

const fn command_flags(revision: crate::ExpectedRevision) -> u8 {
    match revision {
        crate::ExpectedRevision::Exact(_) => 0,
        crate::ExpectedRevision::Any => 1,
    }
}

fn measure_payload(
    codec: &ProtocolCodec,
    write: impl FnOnce(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<MessageMeasure, EncodeError> {
    #[cfg(test)]
    FRAME_COUNT_PASSES.with(|passes| passes.set(passes.get().saturating_add(1)));
    let mut sizing = CountSink::new(codec.limits());
    write(&mut sizing)?;
    let measure = sizing.measure_message()?;
    if measure.length > codec.limits().max_frame_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(measure)
}

#[allow(clippy::too_many_arguments)] // The frozen outer-header fields are deliberately explicit.
fn encode_complete_frame(
    codec: &ProtocolCodec,
    kind: FrameKind,
    message_id: MessageId,
    status: StatusCode,
    request_id: u64,
    revision: u64,
    flags: u8,
    measure: MessageMeasure,
    output: &mut [u8],
    write_payload: impl FnOnce(&mut dyn Sink) -> Result<(), EncodeError>,
) -> Result<usize, EncodeError> {
    let payload_u32 = u32::try_from(measure.length).map_err(|_| EncodeError::LimitExceeded)?;
    let required = OUTER_HEADER_BYTES
        .checked_add(measure.length)
        .ok_or(EncodeError::LimitExceeded)?;
    if required > codec.limits().max_frame_bytes
        || measure.field_count > codec.limits().max_tlv_count
    {
        return Err(EncodeError::LimitExceeded);
    }
    if output.len() < required {
        return Err(EncodeError::OutputTooSmall { required });
    }
    #[cfg(test)]
    FRAME_SLICE_PASSES.with(|passes| passes.set(passes.get().saturating_add(1)));
    codec.write_outer_header(
        output,
        kind,
        message_id,
        status,
        request_id,
        revision,
        flags,
        payload_u32,
        measure.field_count,
    )?;
    let mut writer = SliceSink::new(&mut output[OUTER_HEADER_BYTES..required], codec.limits());
    write_payload(&mut writer)?;
    if writer.measure_message()? != measure {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(required)
}

fn write_command_payload(
    payload: &CommandPayload<'_>,
    sink: &mut dyn Sink,
) -> Result<(), EncodeError> {
    match payload {
        CommandPayload::CapabilitiesGet => {
            sink.check_field_count(crate::schema::capabilities_request::SPEC.field_count(&[])?)
        }
        CommandPayload::SessionSnapshotGet(value) => {
            crate::message_wire::write_snapshot_request(sink, *value)
        }
        CommandPayload::SessionTransactionApply(_) => {
            unreachable!("handled as a complete transaction frame")
        }
        CommandPayload::TransportGet => {
            sink.check_field_count(crate::schema::transport_get::SPEC.field_count(&[])?)
        }
        CommandPayload::TransportSet(value) => {
            crate::message_wire::write_transport_set(sink, *value)
        }
        CommandPayload::TelemetryConfigure(value) => {
            crate::message_wire::write_telemetry_configuration(sink, value, false)
        }
        CommandPayload::CountersGet(value) => {
            crate::message_wire::write_counters_request(sink, value)
        }
        CommandPayload::DiagnosticsGet(value) => {
            crate::message_wire::write_diagnostics_request(sink, *value)
        }
        CommandPayload::ControlListGet(value) | CommandPayload::ControlStateGet(value) => {
            crate::message_wire::write_control_request(sink, value)
        }
        CommandPayload::ResponseGet(value) => {
            crate::message_wire::write_response_request(sink, value)
        }
        CommandPayload::SessionReplace(value) => {
            crate::message_wire::write_session_replace_request(sink, *value)
        }
        CommandPayload::SpectrumConfigure(value) => {
            crate::message_wire::write_spectrum_configure_request(sink, value)
        }
        CommandPayload::SpectrumCapture(value) => {
            crate::message_wire::write_spectrum_capture_request(sink, value)
        }
        CommandPayload::SpectrumStream(value) => {
            crate::message_wire::write_spectrum_stream_request(sink, value)
        }
        CommandPayload::Loudness(value) => {
            crate::message_wire::write_loudness_request(sink, *value)
        }
        CommandPayload::ObservationRead(value) => {
            crate::message_wire::write_observation_read_request(sink, value)
        }
    }
}

fn write_success_payload(
    codec: &ProtocolCodec,
    payload: &SuccessResponsePayload<'_>,
    sink: &mut dyn Sink,
) -> Result<(), EncodeError> {
    match payload {
        SuccessResponsePayload::Capabilities(value) => {
            crate::message_wire::write_capabilities(sink, value)
        }
        SuccessResponsePayload::SessionSnapshot(value) => {
            crate::message_wire::write_snapshot(sink, *value)
        }
        SuccessResponsePayload::SessionTransactionApplied(value)
        | SuccessResponsePayload::SessionReplaced(value) => {
            crate::message_wire::write_transaction_applied(sink, value)
        }
        SuccessResponsePayload::TransportGetSnapshot(value)
        | SuccessResponsePayload::TransportSetSnapshot(value) => {
            crate::message_wire::write_transport_snapshot(sink, *value)
        }
        SuccessResponsePayload::TelemetryConfiguration(value) => {
            crate::message_wire::write_telemetry_configuration(sink, value, true)
        }
        SuccessResponsePayload::CounterSnapshot(value) => {
            crate::message_wire::write_counter_snapshot(
                sink,
                CounterSnapshotRef {
                    observed_sample: value.observed_sample,
                    values: &value.values,
                },
            )
        }
        SuccessResponsePayload::DiagnosticsPage(value) => {
            crate::message_wire::write_diagnostics_page(codec, sink, value)
        }
        SuccessResponsePayload::ControlListPage(value) => {
            crate::message_wire::write_control_list_page(sink, value)
        }
        SuccessResponsePayload::ControlStatePage(value) => {
            crate::message_wire::write_control_state_page(sink, value)
        }
        SuccessResponsePayload::ResponseResult(value) => {
            crate::message_wire::write_response_result(sink, value)
        }
        SuccessResponsePayload::SpectrumConfigured(value) => {
            crate::message_wire::write_spectrum_configured(sink, value)
        }
        SuccessResponsePayload::SpectrumCaptured(value) => {
            crate::message_wire::write_spectrum_capture_reply(sink, value)
        }
        SuccessResponsePayload::SpectrumStreamed(value) => {
            crate::message_wire::write_spectrum_stream_reply(sink, value)
        }
        SuccessResponsePayload::Loudness(value) => {
            crate::message_wire::write_loudness_reply(sink, value)
        }
        SuccessResponsePayload::ObservationRead(value) => {
            crate::message_wire::write_observation_read_page(sink, value)
        }
    }
}

fn write_event_payload(
    codec: &ProtocolCodec,
    payload: &EventPayload<'_>,
    sink: &mut dyn Sink,
) -> Result<(), EncodeError> {
    match payload {
        EventPayload::SessionCommitted(value) => {
            crate::message_wire::write_session_committed(sink, *value)
        }
        EventPayload::TransportState(value) => {
            crate::message_wire::write_transport_state_event(sink, *value)
        }
        EventPayload::ObservationEnded(value) => {
            crate::message_wire::write_observation_ended(sink, value)
        }
        EventPayload::MeterBatch(value) => crate::message_wire::write_meter_batch(sink, *value),
        EventPayload::CounterSnapshot(value) => {
            crate::message_wire::write_counter_snapshot(sink, *value)
        }
        EventPayload::Diagnostic(value) => {
            crate::message_wire::write_diagnostic_event(codec, sink, value)
        }
        EventPayload::ResponseReady(value) => {
            crate::message_wire::write_response_ready(sink, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request_id() -> RequestId {
        RequestId::new(1).expect("nonzero request ID")
    }

    fn capabilities<'a>() -> Capabilities<'a> {
        Capabilities {
            minimum_version: crate::ProtocolVersion::V1,
            maximum_version: crate::ProtocolVersion::V1,
            maximum_frame_bytes: 4096,
            maximum_tlvs: 1024,
            maximum_string_bytes: 1024,
            maximum_nesting: 4,
            control_command_slots: 1,
            control_command_bytes: 64,
            reliable_response_slots: 1,
            reliable_event_slots: 1,
            telemetry_slots: 1,
            replay_entries: 1,
            replay_bytes: 64,
            maximum_cached_response_bytes: 64,
            maximum_control_page_items: 256,
            maximum_diagnostic_page_items: 256,
            maximum_telemetry_handles: 256,
            maximum_transaction_edits: 1,
            supported_commands: &[1, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14],
            supported_events: &[0x8001, 0x8010, 0x8020, 0x8021, 0x8030, 0x8040],
            flags: crate::CapabilityFlags::B4_BASE,
        }
    }

    fn diagnostic_event() -> DiagnosticEvent {
        DiagnosticEvent {
            diagnostic: crate::Diagnostic {
                code: "protocol.test".to_owned(),
                severity: crate::DiagnosticSeverity::Error,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: Some(1),
            },
        }
    }

    fn encode_command(codec: &ProtocolCodec, frame: &TypedCommandFrame<'_>) -> Vec<u8> {
        let mut output = vec![0_u8; 4096];
        let len = codec
            .encode_command_frame_into(frame, &mut output)
            .expect("command frame");
        output.truncate(len);
        output
    }

    fn encode_success(codec: &ProtocolCodec, frame: &TypedSuccessResponseFrame<'_>) -> Vec<u8> {
        let mut output = vec![0_u8; 4096];
        let len = codec
            .encode_success_response_frame_into(frame, &mut output)
            .expect("success response frame");
        output.truncate(len);
        output
    }

    fn encode_event(codec: &ProtocolCodec, frame: &TypedEventFrame<'_>) -> Vec<u8> {
        let mut output = vec![0_u8; 4096];
        let len = codec
            .encode_event_frame_into(frame, &mut output)
            .expect("event frame");
        output.truncate(len);
        output
    }

    fn assert_command_frame(
        codec: &ProtocolCodec,
        frame: &TypedCommandFrame<'_>,
        expected_id: MessageId,
    ) {
        let bytes = encode_command(codec, frame);
        let decoded = codec
            .decode_typed_command(&bytes, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .expect("typed command decode");
        assert_eq!(decoded.header.message_id, expected_id);
        assert_eq!(decoded.header.request_id, request_id());
        let mut canonical = vec![0_u8; bytes.len()];
        assert_eq!(
            codec.encode_command_frame_into(frame, &mut canonical),
            Ok(bytes.len())
        );
        assert_eq!(canonical, bytes, "full command frame is canonical");
        let mut short = vec![0xa5_u8; bytes.len() - 1];
        assert_eq!(
            codec.encode_command_frame_into(frame, &mut short),
            Err(EncodeError::OutputTooSmall {
                required: bytes.len()
            })
        );
        assert!(short.iter().all(|byte| *byte == 0xa5));
    }

    fn outer_field_count(frame: &[u8]) -> u32 {
        u32::from_le_bytes(frame[40..44].try_into().expect("outer TLV count"))
    }

    #[test]
    fn complete_frame_header_uses_sizing_sink_optional_and_repeated_counts() {
        let codec = ProtocolCodec::default();
        let event = |origin_request_id| {
            encode_event(
                &codec,
                &TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::TransportState(TransportStateEvent {
                        event_sequence: 1,
                        state: crate::TransportState::Playing,
                        effective_sample: crate::SampleTime(9),
                        origin_request_id,
                    }),
                },
            )
        };
        assert_eq!(outer_field_count(&event(None)), 3);
        assert_eq!(outer_field_count(&event(Some(request_id()))), 4);

        let diagnostic = crate::Diagnostic {
            code: "protocol.count".to_owned(),
            severity: crate::DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: None,
        };
        let payload = NonOkResponse {
            diagnostics: vec![diagnostic.clone(), diagnostic],
            omitted_diagnostics: 0,
            backpressure: None,
        };
        let frame = TypedNonOkResponseFrame {
            request_id: request_id(),
            revision: SessionRevision(7),
            message_id: MessageId::CapabilitiesGet,
            status: StatusCode::InvalidField,
            payload: &payload,
        };
        let mut output = vec![0_u8; 4096];
        let written = codec
            .encode_non_ok_response_frame_into(&frame, &mut output)
            .expect("non-OK frame");
        output.truncate(written);
        assert_eq!(outer_field_count(&output), 3);
    }

    #[test]
    fn full_command_frames_cover_every_registered_command_without_payload_escape() {
        let codec = ProtocolCodec::default();
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("renamed").expect("stable ID"),
        }];
        let control_request = crate::ControlListRequest {
            structure_epoch: 0,
            after_handle: 0,
            limit: 1,
            owner: None,
            headers_only: false,
        };
        let response_request = crate::ResponseGetRequest {
            target: crate::ResponseTarget::Configuration(
                crate::ResponseConfiguration::InputFilters,
            ),
            grid: crate::ResponseGrid::Logarithmic,
            points: 4,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
            channels: crate::ParameterChannel::Both,
            sections: false,
            overrides: Vec::new(),
            configuration_id: None,
            sample_rate_hz: None,
            quantum_frames: None,
            maximum_prepared_bytes: None,
            maximum_state_bytes: None,
            maximum_scratch_bytes: None,
            maximum_automation_spans: None,
            maximum_reply_bytes: None,
        };
        let spectrum_request = crate::SpectrumConfigureRequest {
            form: crate::SpectrumForm::Collection,
            entries: vec![crate::SpectrumEntry {
                target_kind: crate::SpectrumTargetKind::TrackPostPan,
                target_id: "t1".to_owned(),
                channels: crate::ParameterChannel::Both,
            }],
            maximum_capture_bytes: 1 << 16,
            hop_frames: Some(1024),
        };
        let telemetry = TelemetryConfiguration {
            meters: Vec::new(),
            meter_period_blocks: 0,
            meter_window_blocks: None,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
        };
        let counters = CountersRequest {
            all: true,
            ids: Vec::new(),
        };
        let observation_read = crate::ObservationReadRequest {
            selections: vec![crate::ObservationReadSelection {
                strip_id: session::StableId::parse("vocal").expect("strip"),
                section: controls::TapSection::Inserts(
                    session::StableId::parse("comp").expect("insert"),
                ),
                tap_id: 1,
                channels: crate::ParameterChannel::Both,
            }],
        };
        let frames = [
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::CapabilitiesGet,
                },
                MessageId::CapabilitiesGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                        offset: 0,
                        maximum_bytes: 1,
                    }),
                },
                MessageId::SessionSnapshotGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Exact(SessionRevision(7)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                MessageId::SessionTransactionApply,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::TransportGet,
                },
                MessageId::TransportGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Exact(SessionRevision(7)),
                    payload: CommandPayload::TransportSet(TransportSetRequest {
                        state: crate::TransportState::Playing,
                    }),
                },
                MessageId::TransportSet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Exact(SessionRevision(7)),
                    payload: CommandPayload::TelemetryConfigure(&telemetry),
                },
                MessageId::TelemetryConfigure,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::CountersGet(&counters),
                },
                MessageId::CountersGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::DiagnosticsGet(DiagnosticsRequest {
                        after_sequence: 0,
                        limit: 1,
                        minimum_severity: crate::DiagnosticSeverity::Info,
                    }),
                },
                MessageId::DiagnosticsGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::ControlListGet(&control_request),
                },
                MessageId::ControlListGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::ResponseGet(&response_request),
                },
                MessageId::ResponseGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::ControlStateGet(&control_request),
                },
                MessageId::ControlStateGet,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Exact(SessionRevision(7)),
                    payload: CommandPayload::SessionReplace(SessionReplaceRequest {
                        document_bytes: 1024,
                        document_sha256: [0x5a; 32],
                    }),
                },
                MessageId::SessionReplace,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::SpectrumConfigure(&spectrum_request),
                },
                MessageId::SpectrumConfigure,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::SpectrumCapture(
                        &crate::SpectrumCaptureRequest::Read {
                            channels: crate::ParameterChannel::Both,
                            include_window: true,
                        },
                    ),
                },
                MessageId::SpectrumCapture,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::SpectrumStream(&crate::SpectrumStreamRequest::Read),
                },
                MessageId::SpectrumStream,
            ),
            (
                TypedCommandFrame {
                    request_id: request_id(),
                    expected_revision: crate::ExpectedRevision::Any,
                    payload: CommandPayload::ObservationRead(&observation_read),
                },
                MessageId::ObservationRead,
            ),
        ];
        for (frame, expected_id) in &frames {
            assert_command_frame(&codec, frame, *expected_id);
        }
        let invalid = TypedCommandFrame {
            request_id: request_id(),
            expected_revision: crate::ExpectedRevision::Any,
            payload: CommandPayload::TransportSet(TransportSetRequest {
                state: crate::TransportState::Stopped,
            }),
        };
        assert_eq!(
            codec.encode_command_frame_into(&invalid, &mut [0_u8; 64]),
            Err(EncodeError::MessageKindMismatch)
        );
    }

    #[test]
    fn full_success_response_frames_cover_every_registered_command_response() {
        let codec = ProtocolCodec::default();
        let responses = [
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::Capabilities(capabilities()),
                },
                MessageId::CapabilitiesGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SessionSnapshot(SessionSnapshot {
                        total_bytes: 0,
                        offset: 0,
                        canonical_json_chunk: &[],
                        eof: true,
                    }),
                },
                MessageId::SessionSnapshotGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SessionTransactionApplied(
                        TransactionApplied::new(1, crate::EditPath::Live),
                    ),
                },
                MessageId::SessionTransactionApply,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::TransportGetSnapshot(TransportSnapshot {
                        state: crate::TransportState::Stopped,
                        effective_sample: crate::SampleTime(0),
                    }),
                },
                MessageId::TransportGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::TransportSetSnapshot(TransportSnapshot {
                        state: crate::TransportState::Playing,
                        effective_sample: crate::SampleTime(9),
                    }),
                },
                MessageId::TransportSet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::TelemetryConfiguration(
                        TelemetryConfiguration {
                            meters: Vec::new(),
                            meter_period_blocks: 0,
                            meter_window_blocks: core::num::NonZeroU32::new(12),
                            counter_ids: Vec::new(),
                            counter_period_blocks: 0,
                            diagnostics_enabled: false,
                            minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
                        },
                    ),
                },
                MessageId::TelemetryConfigure,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::CounterSnapshot(CounterSnapshot {
                        observed_sample: crate::SampleTime(0),
                        values: Vec::new(),
                    }),
                },
                MessageId::CountersGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::DiagnosticsPage(DiagnosticsPage {
                        last_sequence: 0,
                        eof: true,
                        diagnostics: Vec::new(),
                    }),
                },
                MessageId::DiagnosticsGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::ControlListPage(crate::ControlListPage {
                        revision: 7,
                        structure_epoch: 1,
                        last_handle: 0,
                        eof: true,
                        entries: Vec::new(),
                    }),
                },
                MessageId::ControlListGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::ControlStatePage(crate::ControlStatePage {
                        revision: 7,
                        structure_epoch: 1,
                        last_handle: 0,
                        eof: true,
                        records: Vec::new(),
                    }),
                },
                MessageId::ControlStateGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::ResponseResult(crate::ResponseResult::Tag(9)),
                },
                MessageId::ResponseGet,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SessionReplaced(TransactionApplied::new(
                        0,
                        crate::EditPath::Unchanged,
                    )),
                },
                MessageId::SessionReplace,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SpectrumConfigured(
                        crate::SpectrumConfigured {
                            revision: 7,
                            configured_epoch: 2,
                            adopted_epoch: 1,
                            rebuild: true,
                            plan_epoch: Some(2),
                        },
                    ),
                },
                MessageId::SpectrumConfigure,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SpectrumCaptured(
                        crate::SpectrumCaptureReply {
                            adopted_epoch: 1,
                            selection_epoch: 0,
                            status: crate::SpectrumCaptureStatus::Pending,
                            window: None,
                        },
                    ),
                },
                MessageId::SpectrumCapture,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::SpectrumStreamed(crate::SpectrumStreamReply {
                        adopted_epoch: 1,
                        selection_epoch: 0,
                        status: crate::SpectrumStreamStatus::Inactive,
                        target: None,
                        sample_rate_hz: 0,
                        quantum_frames: 0,
                        hop_frames: 0,
                        capture_epoch: 0,
                        sequence: 0,
                        dropped_captures: 0,
                        windows: 0,
                        captured_sample: 0,
                        end_sample: 0,
                        analysis_epoch: 0,
                        history_start_sample: 0,
                        smoothing_ms: 0.0,
                        source_underrun: false,
                        bins: None,
                    }),
                },
                MessageId::SpectrumStream,
            ),
            (
                TypedSuccessResponseFrame {
                    request_id: request_id(),
                    revision: SessionRevision(7),
                    payload: SuccessResponsePayload::ObservationRead(crate::ObservationReadPage {
                        observed_sample: 512,
                        rows: std::borrow::Cow::Owned(vec![crate::ObservationReadRow {
                            status: crate::ObservationRowStatus::Unarmed,
                            armed: false,
                            arm_sample: 0,
                            arm_tag: 0,
                            window: None,
                            left: None,
                            right: None,
                            unit: crate::ParameterUnit::Db,
                        }]),
                    }),
                },
                MessageId::ObservationRead,
            ),
        ];
        for (frame, expected_id) in &responses {
            let bytes = encode_success(&codec, frame);
            let decoded = codec
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut [0_u16; 32]))
                .expect("typed success response");
            let DecodedTypedResponseFrame::Success { header, .. } = decoded else {
                panic!("success response variant");
            };
            assert_eq!(header.message_id, *expected_id);
            let mut canonical = vec![0_u8; bytes.len()];
            assert_eq!(
                codec.encode_success_response_frame_into(frame, &mut canonical),
                Ok(bytes.len())
            );
            assert_eq!(canonical, bytes);
            let mut short = vec![0xa5_u8; bytes.len() - 1];
            assert_eq!(
                codec.encode_success_response_frame_into(frame, &mut short),
                Err(EncodeError::OutputTooSmall {
                    required: bytes.len()
                })
            );
            assert!(short.iter().all(|byte| *byte == 0xa5));
        }
    }

    #[test]
    fn full_non_ok_frames_cover_every_status_and_backpressure_is_exclusive() {
        let codec = ProtocolCodec::default();
        let diagnostic = crate::Diagnostic {
            code: "protocol.failure".to_owned(),
            severity: crate::DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: None,
        };
        let statuses = [
            StatusCode::MalformedFrame,
            StatusCode::UnsupportedVersion,
            StatusCode::UnsupportedMessage,
            StatusCode::UnknownRequiredField,
            StatusCode::InvalidField,
            StatusCode::LimitExceeded,
            StatusCode::RevisionConflict,
            StatusCode::RevisionExhausted,
            StatusCode::RequestIdReuse,
            StatusCode::ReplayExpired,
            StatusCode::Backpressure,
            StatusCode::ValidationFailed,
            StatusCode::NotFound,
            StatusCode::Unavailable,
            StatusCode::PcmForbidden,
            StatusCode::Internal,
        ];
        for status in statuses {
            let backpressure =
                (status == StatusCode::Backpressure).then_some(crate::Backpressure {
                    queue_kind: crate::BackpressureQueueKind::ReplayCache,
                    capacity: 1,
                    occupancy: 0,
                    requested_items: 1,
                    generation: None,
                    retry_boundary: None,
                    requested_bytes: None,
                    available_bytes: None,
                });
            let value = NonOkResponse {
                diagnostics: vec![diagnostic.clone()],
                omitted_diagnostics: 0,
                backpressure,
            };
            let frame = TypedNonOkResponseFrame {
                request_id: request_id(),
                revision: SessionRevision(7),
                message_id: MessageId::CapabilitiesGet,
                status,
                payload: &value,
            };
            let mut bytes = vec![0_u8; 512];
            let length = codec
                .encode_non_ok_response_frame_into(&frame, &mut bytes)
                .expect("non-OK frame");
            bytes.truncate(length);
            let decoded = codec
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
                .expect("non-OK decode");
            let DecodedTypedResponseFrame::NonOk { header, payload } = decoded else {
                panic!("non-OK response variant");
            };
            assert_eq!(header.status, status);
            assert_eq!(
                payload.backpressure.is_some(),
                status == StatusCode::Backpressure
            );
            let mut short = vec![0xa5_u8; bytes.len() - 1];
            assert_eq!(
                codec.encode_non_ok_response_frame_into(&frame, &mut short),
                Err(EncodeError::OutputTooSmall {
                    required: bytes.len()
                })
            );
            assert!(short.iter().all(|byte| *byte == 0xa5));
        }
    }

    #[test]
    fn full_event_frames_cover_every_registered_event_and_reject_pcm_reachability() {
        let codec = ProtocolCodec::default();
        let meter_records = [crate::MeterRecord {
            handle: 1,
            component: crate::MeterComponent::Left,
            flags: 1,
            value: 0.0,
        }];
        let diagnostic = diagnostic_event().diagnostic;
        let events = [
            (
                TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::SessionCommitted(SessionCommitted {
                        event_sequence: 1,
                        origin_request_id: request_id(),
                        previous_revision: SessionRevision(6),
                        applied_operations: 1,
                    }),
                },
                MessageId::SessionCommitted,
            ),
            (
                TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::TransportState(TransportStateEvent {
                        event_sequence: 1,
                        state: crate::TransportState::Stopped,
                        effective_sample: crate::SampleTime(0),
                        origin_request_id: None,
                    }),
                },
                MessageId::TransportState,
            ),
            (
                TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::MeterBatch(MeterBatch {
                        observed_sample: crate::SampleTime(1536),
                        first_sample: crate::SampleTime(0),
                        window_sequence: 1,
                        records: &meter_records,
                    }),
                },
                MessageId::MeterBatch,
            ),
            (
                TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
                        observed_sample: crate::SampleTime(0),
                        values: &[],
                    }),
                },
                MessageId::CounterSnapshot,
            ),
            (
                TypedEventFrame {
                    revision: SessionRevision(7),
                    payload: EventPayload::Diagnostic(&diagnostic),
                },
                MessageId::Diagnostic,
            ),
        ];
        for (frame, expected_id) in &events {
            let bytes = encode_event(&codec, frame);
            let decoded = codec
                .decode_typed_event(&bytes, &mut DecodeScratch::new(&mut [0_u16; 16]))
                .expect("typed event");
            assert_eq!(decoded.header.message_id, *expected_id);
            let mut canonical = vec![0_u8; bytes.len()];
            assert_eq!(
                codec.encode_event_frame_into(frame, &mut canonical),
                Ok(bytes.len())
            );
            assert_eq!(canonical, bytes);
            let mut short = vec![0xa5_u8; bytes.len() - 1];
            assert_eq!(
                codec.encode_event_frame_into(frame, &mut short),
                Err(EncodeError::OutputTooSmall {
                    required: bytes.len()
                })
            );
            assert!(short.iter().all(|byte| *byte == 0xa5));
        }
        let unreachable = TypedNonOkResponseFrame {
            request_id: request_id(),
            revision: SessionRevision(7),
            message_id: MessageId::Diagnostic,
            status: StatusCode::InvalidField,
            payload: &NonOkResponse {
                diagnostics: Vec::new(),
                omitted_diagnostics: 0,
                backpressure: None,
            },
        };
        assert_eq!(
            codec.encode_non_ok_response_frame_into(&unreachable, &mut [0_u8; 64]),
            Err(EncodeError::MessageKindMismatch)
        );
    }
}
