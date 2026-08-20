//! Manual little-endian framing and bounded TLV validation for MISO Control BTLV v1.

use core::{fmt, num::NonZeroU64};

use crate::{OUTER_HEADER_BYTES, PROTOCOL_MAJOR_V1, PROTOCOL_MINOR_V1, TLV_PREFIX_BYTES};

const MAGIC: [u8; 8] = *b"MISOCTL\0";
const FLAG_REVISION_ANY: u8 = 1;
const KNOWN_FLAG_BITS: u8 = FLAG_REVISION_ANY;

/// A version advertised by a BTLV endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersion {
    /// Wire major version.
    pub major: u16,
    /// Wire minor version.
    pub minor: u16,
}

impl ProtocolVersion {
    /// The only protocol version emitted by this implementation.
    pub const V1: Self = Self {
        major: PROTOCOL_MAJOR_V1,
        minor: PROTOCOL_MINOR_V1,
    };
}

/// A nonzero request identity scoped to one logical endpoint lifetime.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequestId(NonZeroU64);

impl RequestId {
    /// Construct a request ID, rejecting the reserved zero value.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Return the raw wire value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// The accepted session model's revision carrier.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SessionRevision(pub u64);

/// A request's revision precondition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedRevision {
    /// Require this exact session-model revision.
    Exact(SessionRevision),
    /// Query the current state without a revision precondition.
    Any,
}

/// Absolute time in engine sample frames.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SampleTime(pub u64);

/// The three frame classes frozen by BTLV v1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FrameKind {
    /// A request to a control endpoint.
    Command = 1,
    /// A request-correlated endpoint result.
    Response = 2,
    /// An unsolicited endpoint event.
    Event = 3,
}

impl FrameKind {
    fn parse(value: u8) -> Result<Self, DecodeError> {
        match value {
            1 => Ok(Self::Command),
            2 => Ok(Self::Response),
            3 => Ok(Self::Event),
            _ => Err(DecodeError::InvalidKind),
        }
    }
}

/// The frozen v1 command and event registry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum MessageId {
    /// `CAPABILITIES_GET`.
    CapabilitiesGet = 0x0001,
    /// `SESSION_SNAPSHOT_GET`.
    SessionSnapshotGet = 0x0002,
    /// `SESSION_TRANSACTION_APPLY`.
    SessionTransactionApply = 0x0003,
    /// `PARAMETER_METADATA_GET`.
    ParameterMetadataGet = 0x0004,
    /// `PARAMETER_STATE_GET`.
    ParameterStateGet = 0x0005,
    /// `AUTOMATION_ENQUEUE`.
    AutomationEnqueue = 0x0006,
    /// `TRANSPORT_GET`.
    TransportGet = 0x0007,
    /// `TRANSPORT_SET`.
    TransportSet = 0x0008,
    /// `TELEMETRY_CONFIGURE`.
    TelemetryConfigure = 0x0009,
    /// `COUNTERS_GET`.
    CountersGet = 0x000a,
    /// `DIAGNOSTICS_GET`.
    DiagnosticsGet = 0x000b,
    /// `SESSION_COMMITTED`.
    SessionCommitted = 0x8001,
    /// `AUTOMATION_CANCELED`.
    AutomationCanceled = 0x8002,
    /// `TRANSPORT_STATE`.
    TransportState = 0x8010,
    /// `METER_BATCH`.
    MeterBatch = 0x8020,
    /// `COUNTER_SNAPSHOT`.
    CounterSnapshot = 0x8021,
    /// `DIAGNOSTIC`.
    Diagnostic = 0x8030,
}

impl MessageId {
    /// Return the frozen numeric ID.
    #[must_use]
    pub const fn raw(self) -> u16 {
        self as u16
    }

    fn parse(value: u16) -> Result<Self, DecodeError> {
        let message = match value {
            0x0001 => Self::CapabilitiesGet,
            0x0002 => Self::SessionSnapshotGet,
            0x0003 => Self::SessionTransactionApply,
            0x0004 => Self::ParameterMetadataGet,
            0x0005 => Self::ParameterStateGet,
            0x0006 => Self::AutomationEnqueue,
            0x0007 => Self::TransportGet,
            0x0008 => Self::TransportSet,
            0x0009 => Self::TelemetryConfigure,
            0x000a => Self::CountersGet,
            0x000b => Self::DiagnosticsGet,
            0x8001 => Self::SessionCommitted,
            0x8002 => Self::AutomationCanceled,
            0x8010 => Self::TransportState,
            0x8020 => Self::MeterBatch,
            0x8021 => Self::CounterSnapshot,
            0x8030 => Self::Diagnostic,
            0x6000..=0x6fff => return Err(DecodeError::PcmForbidden),
            _ => return Err(DecodeError::UnsupportedMessage),
        };
        Ok(message)
    }

    fn permits_kind(self, kind: FrameKind) -> bool {
        match kind {
            FrameKind::Command | FrameKind::Response => self.raw() < 0x8000,
            FrameKind::Event => self.raw() >= 0x8000,
        }
    }
}

/// The frozen v1 response status registry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum StatusCode {
    /// `OK`.
    Ok = 0,
    /// `MALFORMED_FRAME`.
    MalformedFrame = 1,
    /// `UNSUPPORTED_VERSION`.
    UnsupportedVersion = 2,
    /// `UNSUPPORTED_MESSAGE`.
    UnsupportedMessage = 3,
    /// `UNKNOWN_REQUIRED_FIELD`.
    UnknownRequiredField = 4,
    /// `INVALID_FIELD`.
    InvalidField = 5,
    /// `LIMIT_EXCEEDED`.
    LimitExceeded = 6,
    /// `REVISION_CONFLICT`.
    RevisionConflict = 7,
    /// `REVISION_EXHAUSTED`.
    RevisionExhausted = 8,
    /// `REQUEST_ID_REUSE`.
    RequestIdReuse = 9,
    /// `REPLAY_EXPIRED`.
    ReplayExpired = 10,
    /// `BACKPRESSURE`.
    Backpressure = 11,
    /// `VALIDATION_FAILED`.
    ValidationFailed = 12,
    /// `NOT_FOUND`.
    NotFound = 13,
    /// `UNAVAILABLE`.
    Unavailable = 14,
    /// `TIME_IN_PAST`.
    TimeInPast = 15,
    /// `AUTOMATION_ORDER`.
    AutomationOrder = 16,
    /// `PCM_FORBIDDEN`.
    PcmForbidden = 17,
    /// `INTERNAL`.
    Internal = 18,
}

impl StatusCode {
    fn parse(value: u16) -> Result<Self, DecodeError> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::MalformedFrame),
            2 => Ok(Self::UnsupportedVersion),
            3 => Ok(Self::UnsupportedMessage),
            4 => Ok(Self::UnknownRequiredField),
            5 => Ok(Self::InvalidField),
            6 => Ok(Self::LimitExceeded),
            7 => Ok(Self::RevisionConflict),
            8 => Ok(Self::RevisionExhausted),
            9 => Ok(Self::RequestIdReuse),
            10 => Ok(Self::ReplayExpired),
            11 => Ok(Self::Backpressure),
            12 => Ok(Self::ValidationFailed),
            13 => Ok(Self::NotFound),
            14 => Ok(Self::Unavailable),
            15 => Ok(Self::TimeInPast),
            16 => Ok(Self::AutomationOrder),
            17 => Ok(Self::PcmForbidden),
            18 => Ok(Self::Internal),
            _ => Err(DecodeError::InvalidStatus),
        }
    }
}

/// A decoded BTLV header independent of payload ownership.
///
/// Correlation and revision carriers are deliberately kind-specific. In particular, events have
/// no request correlation ID on the outer wire and responses/events never expose a revision
/// precondition as an [`ExpectedRevision`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameHeader {
    /// A nonzero request-correlated command header.
    Command(CommandHeader),
    /// A nonzero request-correlated response header.
    Response(ResponseHeader),
    /// An unsolicited event header whose wire request ID is exactly zero.
    Event(EventHeader),
}

impl FrameHeader {
    /// Return this header's frozen frame class.
    #[must_use]
    pub const fn kind(self) -> FrameKind {
        match self {
            Self::Command(_) => FrameKind::Command,
            Self::Response(_) => FrameKind::Response,
            Self::Event(_) => FrameKind::Event,
        }
    }

    /// Borrow a typed command header only when this is a command.
    #[must_use]
    pub const fn command(self) -> Option<CommandHeader> {
        match self {
            Self::Command(header) => Some(header),
            Self::Response(_) | Self::Event(_) => None,
        }
    }

    /// Borrow a typed response header only when this is a response.
    #[must_use]
    pub const fn response(self) -> Option<ResponseHeader> {
        match self {
            Self::Response(header) => Some(header),
            Self::Command(_) | Self::Event(_) => None,
        }
    }

    /// Borrow a typed event header only when this is an event.
    #[must_use]
    pub const fn event(self) -> Option<EventHeader> {
        match self {
            Self::Event(header) => Some(header),
            Self::Command(_) | Self::Response(_) => None,
        }
    }
}

/// The kind-specific decoded command outer header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandHeader {
    /// Sender protocol version.
    pub version: ProtocolVersion,
    /// Frozen command registry entry.
    pub message_id: MessageId,
    /// Nonzero endpoint request correlation identity.
    pub request_id: RequestId,
    /// Command-only query/mutation revision precondition.
    pub expected_revision: ExpectedRevision,
    /// Exact unpadded TLV payload byte count including TLV padding.
    pub payload_len: u32,
    /// Number of top-level TLVs.
    pub tlv_count: u32,
}

/// The kind-specific decoded response outer header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseHeader {
    /// Sender protocol version.
    pub version: ProtocolVersion,
    /// Echoed frozen command registry entry.
    pub message_id: MessageId,
    /// Nonzero command correlation identity.
    pub request_id: RequestId,
    /// Registered authoritative endpoint result status.
    pub status: StatusCode,
    /// Observed or committed authoritative session revision.
    pub revision: SessionRevision,
    /// Exact unpadded TLV payload byte count including TLV padding.
    pub payload_len: u32,
    /// Number of top-level TLVs.
    pub tlv_count: u32,
}

/// The kind-specific decoded event outer header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventHeader {
    /// Sender protocol version.
    pub version: ProtocolVersion,
    /// Frozen event registry entry.
    pub message_id: MessageId,
    /// Observed or committed authoritative session revision.
    pub revision: SessionRevision,
    /// Exact unpadded TLV payload byte count including TLV padding.
    pub payload_len: u32,
    /// Number of top-level TLVs.
    pub tlv_count: u32,
}

/// A typed v1 command header. Payload schemas are added in the next issue-005 tranche.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandFrame {
    /// Correlates this request and its response.
    pub request_id: RequestId,
    /// Query precondition; a mutation dispatcher will reject `Any` separately.
    pub expected_revision: ExpectedRevision,
    /// Frozen command registry entry.
    pub message_id: MessageId,
}

/// A typed v1 response header with an empty payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseFrame {
    /// Correlates the response to the originating request.
    pub request_id: RequestId,
    /// Committed or observed session revision.
    pub revision: SessionRevision,
    /// Echoed command ID.
    pub message_id: MessageId,
    /// Typed response status.
    pub status: StatusCode,
}

/// A typed v1 event header with an empty payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventFrame {
    /// Observed committed session revision.
    pub revision: SessionRevision,
    /// Frozen event registry entry.
    pub message_id: MessageId,
}

/// The typed, no-arbitrary-field encoder surface currently available in this tranche.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Frame {
    /// A command with an empty payload.
    Command(CommandFrame),
    /// A response with an empty payload.
    Response(ResponseFrame),
    /// An event with an empty payload.
    Event(EventFrame),
}

/// Explicit limits that bound BTLV decode work and caller scratch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolLimits {
    /// Maximum full frame size in bytes, including the 48-byte outer header.
    pub max_frame_bytes: usize,
    /// Maximum top-level or nested TLV count.
    pub max_tlv_count: u32,
    /// Maximum UTF-8 field byte length.
    pub max_string_bytes: usize,
    /// Maximum nested MESSAGE depth, excluding the outer payload.
    pub max_nesting: u8,
}

impl Default for ProtocolLimits {
    fn default() -> Self {
        Self {
            max_frame_bytes: 1024 * 1024,
            max_tlv_count: 1024,
            max_string_bytes: 64 * 1024,
            max_nesting: 4,
        }
    }
}

/// Fixed caller-provided bookkeeping used during bounded decode.
pub struct DecodeScratch<'a> {
    field_ids: &'a mut [u16],
    used: usize,
}

impl<'a> DecodeScratch<'a> {
    /// Borrow caller-owned field-ID slots. The codec never retains this slice.
    #[must_use]
    pub fn new(field_ids: &'a mut [u16]) -> Self {
        Self { field_ids, used: 0 }
    }

    /// Number of top-level TLV entries validated by the last decode.
    #[must_use]
    pub const fn used(&self) -> usize {
        self.used
    }

    fn reset(&mut self) {
        self.used = 0;
    }

    fn push(&mut self, field_id: u16) -> Result<(), DecodeError> {
        let Some(slot) = self.field_ids.get_mut(self.used) else {
            return Err(DecodeError::ScratchTooSmall);
        };
        *slot = field_id;
        self.used += 1;
        Ok(())
    }
}

/// A zero-copy validated frame borrowed only from the decode input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedFrame<'a> {
    /// Fully validated outer header.
    pub header: FrameHeader,
    /// Fully bounds-checked BTLV payload, including canonical zero padding.
    pub payload: &'a [u8],
}

/// A deterministic malformed-frame class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// The input ended before a required fixed or declared byte range.
    Truncated,
    /// The magic sequence differs from `MISOCTL\\0`.
    BadMagic,
    /// The major version is not BTLV major one.
    UnsupportedVersion,
    /// Header length was not exactly 48.
    BadHeaderLength,
    /// Frame kind was not command, response, or event.
    InvalidKind,
    /// A non-v1 header flag was set.
    InvalidFlags,
    /// Command/event status was nonzero or a response status was unregistered.
    InvalidStatus,
    /// Request ID zero is reserved.
    InvalidRequestId,
    /// The revision-any flag and revision carrier disagree.
    InvalidRevisionEncoding,
    /// A declared frame or TLV limit was exceeded.
    LimitExceeded,
    /// Outer payload size did not exactly match the input length.
    BadPayloadLength,
    /// The outer reserved word or nested reserved word was nonzero.
    NonzeroReserved,
    /// Message ID is not allocated in BTLV major one.
    UnsupportedMessage,
    /// A media/PCM-reserved message ID was encountered.
    PcmForbidden,
    /// Message class did not match its registry range.
    MessageKindMismatch,
    /// A TLV field ID, flags, order, count, or padding was invalid.
    InvalidTlv,
    /// A scalar, packed, bool, or nested-message length was invalid.
    InvalidValueLength,
    /// A UTF-8 field was not valid UTF-8.
    InvalidUtf8,
    /// Caller-provided decode bookkeeping cannot represent the declared fields.
    ScratchTooSmall,
    /// An empty schema received an unrecognized mandatory field.
    UnknownRequiredField,
}

impl DecodeError {
    /// The status an endpoint should use if it chooses to encode this parsing failure.
    #[must_use]
    pub const fn status(self) -> StatusCode {
        match self {
            Self::UnsupportedVersion => StatusCode::UnsupportedVersion,
            Self::UnsupportedMessage | Self::MessageKindMismatch => StatusCode::UnsupportedMessage,
            Self::PcmForbidden => StatusCode::PcmForbidden,
            Self::LimitExceeded | Self::ScratchTooSmall => StatusCode::LimitExceeded,
            Self::UnknownRequiredField => StatusCode::UnknownRequiredField,
            _ => StatusCode::MalformedFrame,
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for DecodeError {}

/// A caller-output encoding failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeError {
    /// The typed frame cannot use the requested frame class or message ID.
    MessageKindMismatch,
    /// A configured protocol frame, field-count, or UTF-8 byte limit would be exceeded.
    LimitExceeded,
    /// The caller buffer is insufficient; no output bytes were written.
    OutputTooSmall {
        /// Exact canonical frame byte length required.
        required: usize,
    },
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageKindMismatch => formatter.write_str("message kind mismatch"),
            Self::LimitExceeded => formatter.write_str("protocol encoding limit exceeded"),
            Self::OutputTooSmall { required } => write!(formatter, "output too small: {required}"),
        }
    }
}

impl std::error::Error for EncodeError {}

/// Manual BTLV codec configured with explicit finite decode limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolCodec {
    limits: ProtocolLimits,
}

impl ProtocolCodec {
    /// Construct a codec after validating only meaningful nonzero byte bounds.
    #[must_use]
    pub const fn new(limits: ProtocolLimits) -> Self {
        Self { limits }
    }

    /// Borrow this codec's fixed limits.
    #[must_use]
    pub const fn limits(&self) -> ProtocolLimits {
        self.limits
    }

    /// Decode one complete outer frame without allocating or retaining caller input/scratch.
    pub fn decode<'a>(
        &self,
        input: &'a [u8],
        scratch: &mut DecodeScratch<'_>,
    ) -> Result<DecodedFrame<'a>, DecodeError> {
        scratch.reset();
        if input.len() > self.limits.max_frame_bytes {
            return Err(DecodeError::LimitExceeded);
        }
        let header_bytes = input
            .get(..OUTER_HEADER_BYTES)
            .ok_or(DecodeError::Truncated)?;
        if header_bytes[..8] != MAGIC {
            return Err(DecodeError::BadMagic);
        }
        let version = ProtocolVersion {
            major: read_u16(header_bytes, 8)?,
            minor: read_u16(header_bytes, 10)?,
        };
        if version.major != PROTOCOL_MAJOR_V1 {
            return Err(DecodeError::UnsupportedVersion);
        }
        if read_u16(header_bytes, 12)? != OUTER_HEADER_BYTES as u16 {
            return Err(DecodeError::BadHeaderLength);
        }
        let kind = FrameKind::parse(header_bytes[14])?;
        let flags = header_bytes[15];
        if flags & !KNOWN_FLAG_BITS != 0 {
            return Err(DecodeError::InvalidFlags);
        }
        let message_id = MessageId::parse(read_u16(header_bytes, 16)?)?;
        if !message_id.permits_kind(kind) {
            return Err(DecodeError::MessageKindMismatch);
        }
        let status = StatusCode::parse(read_u16(header_bytes, 18)?)?;
        if !matches!(kind, FrameKind::Response) && status != StatusCode::Ok {
            return Err(DecodeError::InvalidStatus);
        }
        let payload_len = read_u32(header_bytes, 20)?;
        let declared_len = OUTER_HEADER_BYTES
            .checked_add(usize::try_from(payload_len).map_err(|_| DecodeError::LimitExceeded)?)
            .ok_or(DecodeError::LimitExceeded)?;
        if declared_len != input.len() {
            return Err(DecodeError::BadPayloadLength);
        }
        let wire_request_id = read_u64(header_bytes, 24)?;
        let wire_revision = read_u64(header_bytes, 32)?;
        let tlv_count = read_u32(header_bytes, 40)?;
        if tlv_count > self.limits.max_tlv_count {
            return Err(DecodeError::LimitExceeded);
        }
        if read_u32(header_bytes, 44)? != 0 {
            return Err(DecodeError::NonzeroReserved);
        }
        let payload = &input[OUTER_HEADER_BYTES..];
        let count = parse_tlvs(payload, tlv_count, 0, self.limits, scratch, true)?;
        if count != tlv_count {
            return Err(DecodeError::InvalidTlv);
        }
        if message_id == MessageId::CapabilitiesGet && kind == FrameKind::Command {
            let mut cursor = 0_usize;
            for _ in 0..tlv_count {
                let prefix = payload
                    .get(cursor..cursor + TLV_PREFIX_BYTES)
                    .ok_or(DecodeError::Truncated)?;
                if prefix[3] & 1 != 0 {
                    return Err(DecodeError::UnknownRequiredField);
                }
                let length = usize::try_from(read_u32(prefix, 4)?)
                    .map_err(|_| DecodeError::LimitExceeded)?;
                cursor = cursor
                    .checked_add(TLV_PREFIX_BYTES + length + padding(length))
                    .ok_or(DecodeError::LimitExceeded)?;
            }
        }
        let header = match kind {
            FrameKind::Command => {
                let request_id =
                    RequestId::new(wire_request_id).ok_or(DecodeError::InvalidRequestId)?;
                let expected_revision = if flags & FLAG_REVISION_ANY != 0 {
                    if wire_revision != 0 {
                        return Err(DecodeError::InvalidRevisionEncoding);
                    }
                    ExpectedRevision::Any
                } else {
                    ExpectedRevision::Exact(SessionRevision(wire_revision))
                };
                FrameHeader::Command(CommandHeader {
                    version,
                    message_id,
                    request_id,
                    expected_revision,
                    payload_len,
                    tlv_count,
                })
            }
            FrameKind::Response => {
                if flags != 0 {
                    return Err(DecodeError::InvalidRevisionEncoding);
                }
                let request_id =
                    RequestId::new(wire_request_id).ok_or(DecodeError::InvalidRequestId)?;
                FrameHeader::Response(ResponseHeader {
                    version,
                    message_id,
                    request_id,
                    status,
                    revision: SessionRevision(wire_revision),
                    payload_len,
                    tlv_count,
                })
            }
            FrameKind::Event => {
                if flags != 0 {
                    return Err(DecodeError::InvalidRevisionEncoding);
                }
                if wire_request_id != 0 {
                    return Err(DecodeError::InvalidRequestId);
                }
                FrameHeader::Event(EventHeader {
                    version,
                    message_id,
                    revision: SessionRevision(wire_revision),
                    payload_len,
                    tlv_count,
                })
            }
        };
        Ok(DecodedFrame { header, payload })
    }

    /// Decode only a complete correlatable command outer header.
    ///
    /// This deliberately stops before TLV validation.  The controller uses it to return a
    /// canonical non-OK response for a malformed *payload* only after the complete command
    /// header has established a real request ID.  It is crate-visible rather than a second
    /// public wire surface: callers must use [`Self::decode`] or the schema-closed typed decoder.
    pub(crate) fn decode_correlatable_command_header(
        &self,
        input: &[u8],
    ) -> Result<CommandHeader, DecodeError> {
        if input.len() > self.limits.max_frame_bytes {
            return Err(DecodeError::LimitExceeded);
        }
        let header = input
            .get(..OUTER_HEADER_BYTES)
            .ok_or(DecodeError::Truncated)?;
        if header[..8] != MAGIC {
            return Err(DecodeError::BadMagic);
        }
        let version = ProtocolVersion {
            major: read_u16(header, 8)?,
            minor: read_u16(header, 10)?,
        };
        if version.major != PROTOCOL_MAJOR_V1 {
            return Err(DecodeError::UnsupportedVersion);
        }
        if read_u16(header, 12)? != OUTER_HEADER_BYTES as u16 {
            return Err(DecodeError::BadHeaderLength);
        }
        if FrameKind::parse(header[14])? != FrameKind::Command {
            return Err(DecodeError::MessageKindMismatch);
        }
        let flags = header[15];
        if flags & !KNOWN_FLAG_BITS != 0 {
            return Err(DecodeError::InvalidFlags);
        }
        let message_id = MessageId::parse(read_u16(header, 16)?)?;
        if !message_id.permits_kind(FrameKind::Command) {
            return Err(DecodeError::MessageKindMismatch);
        }
        if StatusCode::parse(read_u16(header, 18)?)? != StatusCode::Ok {
            return Err(DecodeError::InvalidStatus);
        }
        let payload_len = read_u32(header, 20)?;
        let declared_len = OUTER_HEADER_BYTES
            .checked_add(usize::try_from(payload_len).map_err(|_| DecodeError::LimitExceeded)?)
            .ok_or(DecodeError::LimitExceeded)?;
        if declared_len != input.len() {
            return Err(DecodeError::BadPayloadLength);
        }
        let request_id =
            RequestId::new(read_u64(header, 24)?).ok_or(DecodeError::InvalidRequestId)?;
        let wire_revision = read_u64(header, 32)?;
        let tlv_count = read_u32(header, 40)?;
        if tlv_count > self.limits.max_tlv_count {
            return Err(DecodeError::LimitExceeded);
        }
        if read_u32(header, 44)? != 0 {
            return Err(DecodeError::NonzeroReserved);
        }
        let expected_revision = if flags & FLAG_REVISION_ANY != 0 {
            if wire_revision != 0 {
                return Err(DecodeError::InvalidRevisionEncoding);
            }
            ExpectedRevision::Any
        } else {
            ExpectedRevision::Exact(SessionRevision(wire_revision))
        };
        Ok(CommandHeader {
            version,
            message_id,
            request_id,
            expected_revision,
            payload_len,
            tlv_count,
        })
    }

    /// Encode a canonical empty-payload frame into caller output without partial writes.
    pub fn encode(&self, frame: &Frame, output: &mut [u8]) -> Result<usize, EncodeError> {
        let (kind, message_id, status, request_id, revision, flags) = header_for_frame(*frame)?;
        let required = OUTER_HEADER_BYTES;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        self.write_outer_header(
            output, kind, message_id, status, request_id, revision, flags, 0, 0,
        )?;
        Ok(required)
    }

    /// Write one already-sized BTLV outer header for an internal schema-specific encoder.
    #[allow(clippy::too_many_arguments)] // The frozen 48-byte wire header is intentionally flat.
    pub(crate) fn write_outer_header(
        &self,
        output: &mut [u8],
        kind: FrameKind,
        message_id: MessageId,
        status: StatusCode,
        request_id: u64,
        revision: u64,
        flags: u8,
        payload_len: u32,
        tlv_count: u32,
    ) -> Result<(), EncodeError> {
        if output.len() < OUTER_HEADER_BYTES {
            return Err(EncodeError::OutputTooSmall {
                required: OUTER_HEADER_BYTES,
            });
        }
        let full_len = OUTER_HEADER_BYTES
            .checked_add(usize::try_from(payload_len).map_err(|_| EncodeError::LimitExceeded)?)
            .ok_or(EncodeError::LimitExceeded)?;
        if full_len > self.limits.max_frame_bytes || tlv_count > self.limits.max_tlv_count {
            return Err(EncodeError::LimitExceeded);
        }
        output[..8].copy_from_slice(&MAGIC);
        put_u16(output, 8, ProtocolVersion::V1.major);
        put_u16(output, 10, ProtocolVersion::V1.minor);
        put_u16(output, 12, OUTER_HEADER_BYTES as u16);
        output[14] = kind as u8;
        output[15] = flags;
        put_u16(output, 16, message_id.raw());
        put_u16(output, 18, status as u16);
        put_u32(output, 20, payload_len);
        put_u64(output, 24, request_id);
        put_u64(output, 32, revision);
        put_u32(output, 40, tlv_count);
        put_u32(output, 44, 0);
        Ok(())
    }
}

impl Default for ProtocolCodec {
    fn default() -> Self {
        Self::new(ProtocolLimits::default())
    }
}

fn header_for_frame(
    frame: Frame,
) -> Result<(FrameKind, MessageId, StatusCode, u64, u64, u8), EncodeError> {
    match frame {
        Frame::Command(frame) => {
            if frame.message_id.raw() >= 0x8000 {
                return Err(EncodeError::MessageKindMismatch);
            }
            let (revision, flags) = match frame.expected_revision {
                ExpectedRevision::Exact(revision) => (revision.0, 0),
                ExpectedRevision::Any => (0, FLAG_REVISION_ANY),
            };
            Ok((
                FrameKind::Command,
                frame.message_id,
                StatusCode::Ok,
                frame.request_id.get(),
                revision,
                flags,
            ))
        }
        Frame::Response(frame) => {
            if frame.message_id.raw() >= 0x8000 {
                return Err(EncodeError::MessageKindMismatch);
            }
            Ok((
                FrameKind::Response,
                frame.message_id,
                frame.status,
                frame.request_id.get(),
                frame.revision.0,
                0,
            ))
        }
        Frame::Event(frame) => {
            if frame.message_id.raw() < 0x8000 {
                return Err(EncodeError::MessageKindMismatch);
            }
            Ok((
                FrameKind::Event,
                frame.message_id,
                StatusCode::Ok,
                0,
                frame.revision.0,
                0,
            ))
        }
    }
}

fn parse_tlvs(
    bytes: &[u8],
    declared_count: u32,
    depth: u8,
    limits: ProtocolLimits,
    scratch: &mut DecodeScratch<'_>,
    top_level: bool,
) -> Result<u32, DecodeError> {
    let mut cursor = 0usize;
    let mut previous_id = 0u16;
    for index in 0..declared_count {
        let prefix_end = cursor
            .checked_add(TLV_PREFIX_BYTES)
            .ok_or(DecodeError::LimitExceeded)?;
        let prefix = bytes
            .get(cursor..prefix_end)
            .ok_or(DecodeError::Truncated)?;
        let field_id = read_u16(prefix, 0)?;
        if field_id == 0 || (index != 0 && field_id < previous_id) {
            return Err(DecodeError::InvalidTlv);
        }
        previous_id = field_id;
        let wire_type = prefix[2];
        let flags = prefix[3];
        if !(1..=15).contains(&wire_type) || flags & !1 != 0 {
            return Err(DecodeError::InvalidTlv);
        }
        let value_len =
            usize::try_from(read_u32(prefix, 4)?).map_err(|_| DecodeError::LimitExceeded)?;
        let value_start = prefix_end;
        let value_end = value_start
            .checked_add(value_len)
            .ok_or(DecodeError::LimitExceeded)?;
        let value = bytes
            .get(value_start..value_end)
            .ok_or(DecodeError::Truncated)?;
        validate_value(wire_type, value, depth, limits, scratch)?;
        let padded_end = value_end
            .checked_add(padding(value_len))
            .ok_or(DecodeError::LimitExceeded)?;
        let padding_bytes = bytes
            .get(value_end..padded_end)
            .ok_or(DecodeError::Truncated)?;
        if padding_bytes.iter().any(|byte| *byte != 0) {
            return Err(DecodeError::InvalidTlv);
        }
        if top_level {
            scratch.push(field_id)?;
        }
        cursor = padded_end;
    }
    if cursor != bytes.len() {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(declared_count)
}

fn validate_value(
    wire_type: u8,
    value: &[u8],
    depth: u8,
    limits: ProtocolLimits,
    scratch: &mut DecodeScratch<'_>,
) -> Result<(), DecodeError> {
    let exact = match wire_type {
        1 | 8 => Some(1),
        2 => Some(2),
        3 | 6 => Some(4),
        4 | 5 | 7 => Some(8),
        _ => None,
    };
    if let Some(exact) = exact {
        if value.len() != exact {
            return Err(DecodeError::InvalidValueLength);
        }
        if wire_type == 8 && !matches!(value[0], 0 | 1) {
            return Err(DecodeError::InvalidValueLength);
        }
    }
    match wire_type {
        9 => {
            if value.len() > limits.max_string_bytes {
                return Err(DecodeError::LimitExceeded);
            }
            if core::str::from_utf8(value).is_err() {
                return Err(DecodeError::InvalidUtf8);
            }
        }
        11 => {
            if depth >= limits.max_nesting {
                return Err(DecodeError::LimitExceeded);
            }
            let nested_header = value.get(..8).ok_or(DecodeError::Truncated)?;
            let count = read_u32(nested_header, 0)?;
            if count > limits.max_tlv_count {
                return Err(DecodeError::LimitExceeded);
            }
            if read_u32(nested_header, 4)? != 0 {
                return Err(DecodeError::NonzeroReserved);
            }
            parse_tlvs(&value[8..], count, depth + 1, limits, scratch, false)?;
        }
        12 if !value.len().is_multiple_of(2) => return Err(DecodeError::InvalidValueLength),
        13 | 15 if !value.len().is_multiple_of(4) => {
            return Err(DecodeError::InvalidValueLength);
        }
        14 if !value.len().is_multiple_of(8) => return Err(DecodeError::InvalidValueLength),
        _ => {}
    }
    Ok(())
}

const fn padding(length: usize) -> usize {
    (8 - (length & 7)) & 7
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, DecodeError> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or(DecodeError::Truncated)?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, DecodeError> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or(DecodeError::Truncated)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, DecodeError> {
    let value = bytes
        .get(offset..offset + 8)
        .ok_or(DecodeError::Truncated)?;
    Ok(u64::from_le_bytes([
        value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
    ]))
}

fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_CAPABILITIES_ANY: [u8; OUTER_HEADER_BYTES] = [
        0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x01,
        0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];

    const EMPTY_CAPABILITIES_RESPONSE: [u8; OUTER_HEADER_BYTES] = [
        0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];

    const EMPTY_COUNTER_EVENT: [u8; OUTER_HEADER_BYTES] = [
        0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x03,
        0x00, 0x21, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];

    fn scratch() -> ([u16; 8], ProtocolCodec) {
        ([0; 8], ProtocolCodec::default())
    }

    #[test]
    fn exact_empty_capabilities_golden_round_trips() {
        let (mut slots, codec) = scratch();
        let decoded = codec
            .decode(&EMPTY_CAPABILITIES_ANY, &mut DecodeScratch::new(&mut slots))
            .expect("golden decodes");
        assert_eq!(
            decoded.header,
            FrameHeader::Command(CommandHeader {
                version: ProtocolVersion::V1,
                message_id: MessageId::CapabilitiesGet,
                request_id: RequestId::new(1).expect("nonzero"),
                expected_revision: ExpectedRevision::Any,
                payload_len: 0,
                tlv_count: 0,
            })
        );
        let frame = Frame::Command(CommandFrame {
            request_id: RequestId::new(1).expect("nonzero"),
            expected_revision: ExpectedRevision::Any,
            message_id: MessageId::CapabilitiesGet,
        });
        let mut encoded = [0xff; OUTER_HEADER_BYTES];
        assert_eq!(codec.encode(&frame, &mut encoded), Ok(OUTER_HEADER_BYTES));
        assert_eq!(encoded, EMPTY_CAPABILITIES_ANY);
    }

    #[test]
    fn response_and_event_goldens_use_kind_specific_correlation_and_revision() {
        let (mut slots, codec) = scratch();
        let response = Frame::Response(ResponseFrame {
            request_id: RequestId::new(1).expect("nonzero"),
            revision: SessionRevision(7),
            message_id: MessageId::CapabilitiesGet,
            status: StatusCode::Ok,
        });
        let event = Frame::Event(EventFrame {
            revision: SessionRevision(7),
            message_id: MessageId::CounterSnapshot,
        });
        let mut encoded = [0xff; OUTER_HEADER_BYTES];
        assert_eq!(
            codec.encode(&response, &mut encoded),
            Ok(OUTER_HEADER_BYTES)
        );
        assert_eq!(encoded, EMPTY_CAPABILITIES_RESPONSE);
        assert!(matches!(
            codec
                .decode(&encoded, &mut DecodeScratch::new(&mut slots))
                .expect("response decodes")
                .header,
            FrameHeader::Response(ResponseHeader {
                request_id,
                revision: SessionRevision(7),
                status: StatusCode::Ok,
                ..
            }) if request_id.get() == 1
        ));
        assert_eq!(codec.encode(&event, &mut encoded), Ok(OUTER_HEADER_BYTES));
        assert_eq!(encoded, EMPTY_COUNTER_EVENT);
        assert!(matches!(
            codec
                .decode(&encoded, &mut DecodeScratch::new(&mut slots))
                .expect("event decodes")
                .header,
            FrameHeader::Event(EventHeader {
                revision: SessionRevision(7),
                message_id: MessageId::CounterSnapshot,
                ..
            })
        ));
    }

    #[test]
    fn every_golden_truncation_rejects() {
        let (mut slots, codec) = scratch();
        for golden in [
            EMPTY_CAPABILITIES_ANY,
            EMPTY_CAPABILITIES_RESPONSE,
            EMPTY_COUNTER_EVENT,
        ] {
            for end in 0..golden.len() {
                assert!(
                    codec
                        .decode(&golden[..end], &mut DecodeScratch::new(&mut slots))
                        .is_err()
                );
            }
        }
    }

    #[test]
    fn revision_any_requires_zero_and_command_context() {
        let (mut slots, codec) = scratch();
        let mut bad = EMPTY_CAPABILITIES_ANY;
        bad[32] = 1;
        assert_eq!(
            codec.decode(&bad, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRevisionEncoding)
        );
        bad = EMPTY_CAPABILITIES_ANY;
        bad[14] = FrameKind::Response as u8;
        assert_eq!(
            codec.decode(&bad, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRevisionEncoding)
        );
    }

    #[test]
    fn kind_specific_request_and_revision_carriers_reject_cross_kind_forms() {
        let (mut slots, codec) = scratch();
        let mut response = EMPTY_CAPABILITIES_RESPONSE;
        response[24] = 0;
        assert_eq!(
            codec.decode(&response, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRequestId)
        );
        response = EMPTY_CAPABILITIES_RESPONSE;
        response[15] = FLAG_REVISION_ANY;
        assert_eq!(
            codec.decode(&response, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRevisionEncoding)
        );
        let mut event = EMPTY_COUNTER_EVENT;
        event[24] = 1;
        assert_eq!(
            codec.decode(&event, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRequestId)
        );
        event = EMPTY_COUNTER_EVENT;
        event[15] = FLAG_REVISION_ANY;
        assert_eq!(
            codec.decode(&event, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidRevisionEncoding)
        );
    }

    #[test]
    fn malformed_tlv_forms_reject_without_overread() {
        let (mut slots, codec) = scratch();
        let mut frame = EMPTY_CAPABILITIES_ANY.to_vec();
        frame[20] = 16;
        frame[40] = 1;
        frame.extend_from_slice(&[1, 0, 1, 1, 1, 0, 0, 0]);
        frame.extend_from_slice(&[0; 8]);
        assert_eq!(
            codec.decode(&frame, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::UnknownRequiredField)
        );
        frame[57] = 1;
        assert_eq!(
            codec.decode(&frame, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::InvalidTlv)
        );
    }

    #[test]
    fn output_too_small_does_not_mutate_caller_memory() {
        let codec = ProtocolCodec::default();
        let frame = Frame::Command(CommandFrame {
            request_id: RequestId::new(1).expect("nonzero"),
            expected_revision: ExpectedRevision::Any,
            message_id: MessageId::CapabilitiesGet,
        });
        let mut output = [0xaa; OUTER_HEADER_BYTES - 1];
        assert_eq!(
            codec.encode(&frame, &mut output),
            Err(EncodeError::OutputTooSmall {
                required: OUTER_HEADER_BYTES
            })
        );
        assert_eq!(output, [0xaa; OUTER_HEADER_BYTES - 1]);
    }

    #[test]
    fn media_range_is_semantically_forbidden() {
        let (mut slots, codec) = scratch();
        let mut frame = EMPTY_CAPABILITIES_ANY;
        frame[16..18].copy_from_slice(&0x6000_u16.to_le_bytes());
        assert_eq!(
            codec.decode(&frame, &mut DecodeScratch::new(&mut slots)),
            Err(DecodeError::PcmForbidden)
        );
    }
}
