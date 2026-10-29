//! Schema-specific common BTLV payloads shared by typed command responses.
//!
//! This module deliberately exposes typed values only. It has no arbitrary-message, arbitrary
//! field, or opaque byte-payload encoder. Encoding is a checked sizing pass followed by one
//! caller-buffer write pass; neither pass allocates.

use crate::{
    DecodeError, EncodeError, ProtocolCodec,
    btlv::{
        CountSink, Fields as Message, Sink, SliceSink, read_f32, read_u8, read_u16, read_u32,
        read_u64,
    },
    schema::{
        self, control_address, control_entry, control_list_page, control_list_request,
        control_state_page, enum_choice, set_control_address,
    },
};

#[cfg(test)]
const WIRE_U8: u8 = 1;

macro_rules! write_spec {
    ($writer:expr, $spec:expr, $bytes:expr $(,)?) => {{
        let spec = $spec;
        $writer.field_spec(spec, $bytes)
    }};
}

macro_rules! one_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.one(spec.id, spec.wire.raw())
    }};
}

macro_rules! optional_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.optional_one(spec.id, spec.wire.raw())
    }};
}

macro_rules! values_spec {
    ($message:expr, $spec:expr $(,)?) => {{
        let spec = $spec;
        $message.values(spec.id, spec.wire.raw())
    }};
}

/// Capability bits advertised by a typed BTLV endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityFlags(pub u64);

impl CapabilityFlags {
    /// Canonical BTLV, caller-buffer, replay, revisioned-edit, and snapshot support.
    pub const B1B_BASE: Self = Self((1 << 5) - 1);
    /// Complete issue-005 typed provider, telemetry, and reliable-event registry.
    pub const B4_BASE: Self = Self(((1 << 15) - 1) & !(1 << 5));
    /// Reliable session-committed event support.
    pub const SESSION_EVENT_STREAM: Self = Self(1 << 12);
    /// The spectrum commands `SPECTRUM_CONFIGURE`, `SPECTRUM_CAPTURE` and `SPECTRUM_STREAM`
    /// (#1594 D10, #1595 D8): set exactly when all three are listed.
    pub const SPECTRUM: Self = Self(1 << 16);
    /// `OBSERVATION_READ` and the `Observe` edit (#1586 D6): set exactly when `0x000f` is listed,
    /// which rides the session family.
    pub const OBSERVATION: Self = Self(1 << 15);
    /// The output loudness meter `LOUDNESS` (#2115): set exactly when `0x0014` is listed.
    pub const LOUDNESS: Self = Self(1 << 17);
    const KNOWN: u64 =
        (((1 << 15) - 1) & !(1 << 5)) | Self::OBSERVATION.0 | Self::SPECTRUM.0 | Self::LOUDNESS.0;
}

/// All 23 fields of a successful `CAPABILITIES_GET` response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // The 23 frozen field names mirror the normative registry verbatim.
pub struct Capabilities<'a> {
    /// Lowest supported protocol version.
    pub minimum_version: crate::ProtocolVersion,
    /// Highest supported protocol version.
    pub maximum_version: crate::ProtocolVersion,
    /// Effective protocol and endpoint resource bounds.
    pub maximum_frame_bytes: u64,
    /// Effective top-level/nested field bound.
    pub maximum_tlvs: u32,
    /// Effective UTF-8 field byte bound.
    pub maximum_string_bytes: u64,
    /// Effective nested MESSAGE depth.
    pub maximum_nesting: u8,
    /// Effective queue/replay/response bounds, in frozen field order.
    pub control_command_slots: u64,
    pub control_command_bytes: u64,
    pub reliable_response_slots: u64,
    pub reliable_event_slots: u64,
    pub telemetry_slots: u64,
    pub replay_entries: u64,
    pub replay_bytes: u64,
    pub maximum_cached_response_bytes: u64,
    pub maximum_control_page_items: u16,
    pub maximum_diagnostic_page_items: u16,
    pub maximum_telemetry_handles: u16,
    pub maximum_transaction_edits: u32,
    /// Strictly increasing frozen command IDs.
    pub supported_commands: &'a [u16],
    /// Strictly increasing frozen event IDs.
    pub supported_events: &'a [u16],
    /// Frozen behavior/provider/event flags.
    pub flags: CapabilityFlags,
}

/// Borrowed strict decode view of a capabilities response's packed ID fields.
///
/// This remains distinct from [`Capabilities`] for workspace source compatibility: existing
/// consumers construct `Capabilities` with native `u16` slice literals, while decode must borrow
/// allocation-free packed little-endian bytes from the input frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Borrowed counterpart of the same frozen 23-field registry.
pub struct DecodedCapabilities<'a> {
    pub minimum_version: crate::ProtocolVersion,
    pub maximum_version: crate::ProtocolVersion,
    pub maximum_frame_bytes: u64,
    pub maximum_tlvs: u32,
    pub maximum_string_bytes: u64,
    pub maximum_nesting: u8,
    pub control_command_slots: u64,
    pub control_command_bytes: u64,
    pub reliable_response_slots: u64,
    pub reliable_event_slots: u64,
    pub telemetry_slots: u64,
    pub replay_entries: u64,
    pub replay_bytes: u64,
    pub maximum_cached_response_bytes: u64,
    pub maximum_control_page_items: u16,
    pub maximum_diagnostic_page_items: u16,
    pub maximum_telemetry_handles: u16,
    pub maximum_transaction_edits: u32,
    /// LE packed strict-increasing `u16` IDs.
    pub supported_commands: &'a [u8],
    /// LE packed strict-increasing `u16` IDs.
    pub supported_events: &'a [u8],
    pub flags: CapabilityFlags,
}

/// Typed `SESSION_SNAPSHOT_GET` request fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact two-field wire schema.
pub struct SessionSnapshotRequest {
    pub offset: u64,
    pub maximum_bytes: u32,
}

/// Typed `SESSION_SNAPSHOT_GET` success fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact four-field wire schema.
pub struct SessionSnapshot<'a> {
    pub total_bytes: u64,
    pub offset: u64,
    pub canonical_json_chunk: &'a [u8],
    pub eof: bool,
}

/// How the host applied a committed transaction (#1313, decision 15 D15-3 and D15-17).
///
/// The code is known at submit and travels in the same response as the new revision. A fallback
/// (the transition) and a supersession are known only after submit; the watermark's outcome flags
/// report them. Zero and every unallocated code are refused on decode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EditPath {
    /// The records went to the running plan's live lanes; no plan was prepared.
    Live = 1,
    /// The transaction committed and render has nothing to do: the delta holds no record of any
    /// kind (a session or profile ID, the stored automation, identical values), or the
    /// controller owns no render plan.
    ModelOnly = 2,
    /// A successor plan was prepared and published.
    Rebuild = 3,
    /// The transaction held only writes to monitor rows (solo): they went to the host's
    /// monitor overlay, which is not a session mutation, so the revision did not advance and no
    /// `SESSION_COMMITTED` event followed (decision 17, C8, #1559). A transaction that holds a
    /// model edit never has this path.
    Monitor = 4,
    /// A `SESSION_REPLACE` whose document equals the committed model (#1386, root decision
    /// 2026-10-16): nothing committed, so the revision did not advance, no `SESSION_COMMITTED`
    /// event followed and no plan was prepared; `applied_operations` is 0. A
    /// `SESSION_TRANSACTION_APPLY` never has this path.
    Unchanged = 5,
}

impl EditPath {
    /// The wire code.
    #[must_use]
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// The path for a wire code; `None` for zero and every unallocated value.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Live),
            2 => Some(Self::ModelOnly),
            3 => Some(Self::Rebuild),
            4 => Some(Self::Monitor),
            5 => Some(Self::Unchanged),
            _ => None,
        }
    }
}

/// Typed successful transaction response payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionApplied {
    /// The number of edits the transaction applied.
    pub applied_operations: u32,
    /// How the host applied the transaction (#1313).
    pub edit_path: EditPath,
    /// The arm tag of each `Observe` edit, in transaction order (#1585 D5): the tag of the tap's
    /// binding after the edit. Empty (the field absent) exactly when the transaction holds no
    /// `Observe` edit.
    pub observe_arm_tags: Vec<u64>,
    /// The tags of the subscriptions the commit ended, in key order (#1585 D5). Empty (the field
    /// absent) exactly when the commit ended none.
    pub removed_arm_tags: Vec<u64>,
}

impl TransactionApplied {
    /// A reply with no arm tags.
    #[must_use]
    pub const fn new(applied_operations: u32, edit_path: EditPath) -> Self {
        Self {
            applied_operations,
            edit_path,
            observe_arm_tags: Vec::new(),
            removed_arm_tags: Vec::new(),
        }
    }
}

/// Bytes of a `SESSION_REPLACE` document digest: a SHA-256.
pub const SESSION_REPLACE_DIGEST_BYTES: usize = 32;

/// Typed `SESSION_REPLACE` request fields (#1386): the document the host passes beside the frame,
/// named by its exact length and its SHA-256. The frame never carries the document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionReplaceRequest {
    /// The exact length of the document, nonzero.
    pub document_bytes: u32,
    /// The SHA-256 of the document.
    pub document_sha256: [u8; SESSION_REPLACE_DIGEST_BYTES],
}

/// Typed reliable `SESSION_COMMITTED` payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)] // Exact four-field wire schema.
pub struct SessionCommitted {
    pub event_sequence: u64,
    pub origin_request_id: crate::RequestId,
    pub previous_revision: crate::SessionRevision,
    pub applied_operations: u32,
}

/// Fixed B2a rack registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterRack {
    /// A track's inserts (decision 12). It kept the retired `dynamic` rack's code `2`; codes `1`
    /// (`simd1`) and `3` (`simd2`) are retired, refused and never reallocated.
    Inserts = 2,
    /// The strip's own builtin section (#178, ruled by #210's D2). Its code `4` is unmoved.
    /// `RackName::wire()` spells the same explicit code table, and
    /// `parameter_enum_wire_mappings_are_exhaustive_and_roundtrip` holds the two to each other.
    Builtins = 4,
    /// A session console slot (decision 12), appended.
    Console = 5,
}
/// Fixed B2a parameter channel registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterChannel {
    Left = 1,
    Right = 2,
    Both = 3,
}
/// Fixed B2a unit registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ParameterUnit {
    Db = 1,
    Hz = 2,
    Milliseconds = 3,
    Samples = 4,
    Linear = 5,
    Ratio = 6,
}
impl ParameterRack {
    pub(crate) const fn from_session(value: session::RackName) -> Self {
        match value {
            session::RackName::Inserts => Self::Inserts,
            session::RackName::Builtins => Self::Builtins,
            session::RackName::Console => Self::Console,
        }
    }

    pub(crate) const fn into_session(self) -> session::RackName {
        match self {
            Self::Inserts => session::RackName::Inserts,
            Self::Builtins => session::RackName::Builtins,
            Self::Console => session::RackName::Console,
        }
    }
}

impl From<session::RackName> for ParameterRack {
    fn from(value: session::RackName) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterRack> for session::RackName {
    fn from(value: ParameterRack) -> Self {
        value.into_session()
    }
}

impl ParameterChannel {
    pub(crate) const fn from_session(value: session::ParameterChannel) -> Self {
        match value {
            session::ParameterChannel::Left => Self::Left,
            session::ParameterChannel::Right => Self::Right,
            session::ParameterChannel::Both => Self::Both,
        }
    }

    pub(crate) const fn into_session(self) -> session::ParameterChannel {
        match self {
            Self::Left => session::ParameterChannel::Left,
            Self::Right => session::ParameterChannel::Right,
            Self::Both => session::ParameterChannel::Both,
        }
    }
}

impl From<session::ParameterChannel> for ParameterChannel {
    fn from(value: session::ParameterChannel) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterChannel> for session::ParameterChannel {
    fn from(value: ParameterChannel) -> Self {
        value.into_session()
    }
}

impl ParameterUnit {
    pub(crate) const fn from_session(value: session::ParameterUnit) -> Self {
        match value {
            session::ParameterUnit::Db => Self::Db,
            session::ParameterUnit::Hz => Self::Hz,
            session::ParameterUnit::Milliseconds => Self::Milliseconds,
            session::ParameterUnit::Samples => Self::Samples,
            session::ParameterUnit::Linear => Self::Linear,
            session::ParameterUnit::Ratio => Self::Ratio,
        }
    }

    pub(crate) const fn into_session(self) -> session::ParameterUnit {
        match self {
            Self::Db => session::ParameterUnit::Db,
            Self::Hz => session::ParameterUnit::Hz,
            Self::Milliseconds => session::ParameterUnit::Milliseconds,
            Self::Samples => session::ParameterUnit::Samples,
            Self::Linear => session::ParameterUnit::Linear,
            Self::Ratio => session::ParameterUnit::Ratio,
        }
    }
}

impl From<session::ParameterUnit> for ParameterUnit {
    fn from(value: session::ParameterUnit) -> Self {
        Self::from_session(value)
    }
}

impl From<ParameterUnit> for session::ParameterUnit {
    fn from(value: ParameterUnit) -> Self {
        value.into_session()
    }
}

/// One declared enumeration choice in a typed descriptor.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct EnumChoice {
    pub value: f32,
    pub label: String,
}
/// Declare one control code table (decision 17, C4): a `repr(u8)` enum with explicit
/// discriminants, its allocated codes in ascending order, and the strict reader that refuses a
/// code outside the table as `INVALID_FIELD`.
macro_rules! control_code_table {
    (
        $(#[$meta:meta])*
        $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $code:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[repr(u8)]
        pub enum $name {
            $($(#[$variant_meta])* $variant = $code,)+
        }

        impl $name {
            /// Every allocated code, ascending.
            pub const CODES: &'static [u8] = &[$($code),+];

            /// The wire code.
            #[must_use]
            pub const fn code(self) -> u8 {
                self as u8
            }

            /// The variant of a wire code; a code outside the table is `INVALID_FIELD`.
            pub const fn from_code(code: u8) -> Result<Self, DecodeError> {
                match code {
                    $($code => Ok(Self::$variant),)+
                    _ => Err(DecodeError::InvalidField),
                }
            }
        }
    };
}

mod response;
pub use response::{
    RESPONSE_BYPASS_CONTROL, ResponseConfiguration, ResponseConfigurationKind, ResponseCurves,
    ResponseGetRequest, ResponseGrid, ResponseMember, ResponseMemberKind, ResponseMemberSection,
    ResponseOverride, ResponseReady, ResponseReadyOutcome, ResponseResult, ResponseTarget,
    TrackResponseCapture, response_ready_encoded_bound, response_result_encoded_len,
};
pub(crate) use response::{write_response_ready, write_response_request, write_response_result};

mod loudness;
pub use loudness::{LOUDNESS_LEVEL_FLOOR, LoudnessReply, LoudnessRequest};
pub(crate) use loudness::{write_loudness_reply, write_loudness_request};

mod spectrum;
pub use spectrum::{
    SPECTRUM_CAPTURE_BIN_COUNT, SPECTRUM_CAPTURE_WINDOW_FRAMES, SPECTRUM_HOP_FRAMES,
    SPECTRUM_SMOOTHING_MAXIMUM_MS, SPECTRUM_TARGET_ID_MAXIMUM_BYTES, SpectrumCaptureReply,
    SpectrumCaptureRequest, SpectrumCaptureStatus, SpectrumCaptureWindow, SpectrumConfigureRequest,
    SpectrumConfigured, SpectrumEntry, SpectrumForm, SpectrumStreamBins, SpectrumStreamReply,
    SpectrumStreamRequest, SpectrumStreamStatus, SpectrumStreamTarget, SpectrumTargetKind,
};
pub(crate) use spectrum::{
    write_spectrum_capture_reply, write_spectrum_capture_request, write_spectrum_configure_request,
    write_spectrum_configured, write_spectrum_stream_reply, write_spectrum_stream_request,
};

control_code_table! {
    /// Which kind of owner a control, or a control request, names.
    #[allow(missing_docs)]
    ControlOwnerKind {
        Session = 1,
        Strip = 2,
        Route = 3,
        Vca = 4,
    }
}

control_code_table! {
    /// The part of an owner a control belongs to. Codes `2`, `4` and `5` are `ParameterRack`'s;
    /// `own` is appended.
    #[allow(missing_docs)]
    ControlSection {
        Inserts = 2,
        Builtins = 4,
        Console = 5,
        Own = 6,
    }
}

control_code_table! {
    /// What an owner header row describes.
    #[allow(missing_docs)]
    ControlOwnerHeaderKind {
        Track = 1,
        Submix = 2,
        Route = 3,
        Vca = 4,
        Session = 5,
    }
}

control_code_table! {
    /// What kind of value a control holds.
    #[allow(missing_docs)]
    ControlKind {
        Continuous = 1,
        Toggle = 2,
        Enumeration = 3,
    }
}

control_code_table! {
    /// How a frontend maps a control's range to a knob.
    #[allow(missing_docs)]
    ControlTaper {
        Linear = 1,
        Logarithmic = 2,
        Exponential = 3,
        Decibel = 4,
        Stepped = 5,
    }
}

control_code_table! {
    /// The `control_smoothing` row that gives a live control its default ramp, in the declaration
    /// order of the live-ramp rows.
    #[allow(missing_docs)]
    ControlRampRow {
        Fader = 1,
        InputTrim = 2,
        SendGain = 3,
        VcaOffset = 4,
        LinkGlide = 5,
        Mute = 6,
        Solo = 7,
        PolarityInvert = 8,
        SendMute = 9,
        SendFollowsMute = 10,
        VcaMute = 11,
        BypassCrossfade = 12,
        Pan = 13,
        Matrix = 14,
        SendMatrix = 15,
    }
}

control_code_table! {
    /// How a change to a control reaches a running plan.
    #[allow(missing_docs)]
    ControlClass {
        Live = 1,
        RebuildOnly = 2,
        Derived = 3,
    }
}

control_code_table! {
    /// Why a control is rebuild-only.
    #[allow(missing_docs)]
    ControlReason {
        NewMemory = 1,
        NewGraph = 2,
        ProcessingOrder = 3,
        Latency = 4,
        Glitch = 5,
        Optimisation = 6,
        Correctness = 7,
        RuledLiveNotBuilt = 8,
    }
}

control_code_table! {
    /// Where a control's value lives.
    #[allow(missing_docs)]
    ControlScope {
        Model = 1,
        Monitor = 2,
    }
}

control_code_table! {
    /// The render family that carries a live change.
    #[allow(missing_docs)]
    ControlFamily {
        StripLevel = 1,
        StripMatrix = 2,
        StripInput = 3,
        Route = 4,
        Effect = 5,
        Setting = 6,
    }
}

/// The control list's page-size bound: `limit` is `1..=256`, and a page holds at most that many
/// rows.
pub const CONTROL_PAGE_ITEMS: u16 = 256;

/// Bytes of one `CONTROL_STATE_GET` record: `handle` (`u32` LE) then `value` (`f32` LE).
pub const CONTROL_STATE_RECORD_BYTES: u16 = 8;

/// A control list or control state request: one page of the rows after `after_handle`, of one
/// structure, optionally of one owner. `CONTROL_STATE_GET` takes the same request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlListRequest {
    /// The structure the caller's handles belong to; `0` for the first page, which has no cursor
    /// (`after_handle` `0`; a cursor with epoch `0` is `INVALID_FIELD`). A page read at
    /// another structure answers `REVISION_CONFLICT` (`control.list.stale`).
    pub structure_epoch: u64,
    /// The last handle the caller has; `0` starts the list.
    pub after_handle: u32,
    /// The most rows to return, `1..=256`.
    pub limit: u16,
    /// Restrict the page to one owner's rows.
    pub owner: Option<ControlOwnerFilter>,
    /// Serve only the header rows (owner and section headers) of the window, with their list
    /// handles and in list order (#2206): field 6, written only when `true`. The cursor, the
    /// limit, the owner filter and the `control.list.stale` rule are the full list's; `limit`
    /// counts header rows, and `eof` says no header row of the window follows the page.
    pub headers_only: bool,
}

/// The owner a request restricts its page to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlOwnerFilter {
    /// The kind of owner.
    pub kind: ControlOwnerKind,
    /// The owner's stable ID; absent exactly for the session owner.
    pub id: Option<String>,
}

/// The stable address of one row: stable IDs, never indices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlAddress {
    /// The kind of owner.
    pub owner_kind: ControlOwnerKind,
    /// The owner's stable ID; absent exactly for the session owner.
    pub owner_id: Option<String>,
    /// The part of the owner.
    pub section: ControlSection,
    /// The console slot or insert ID; present exactly for the console and inserts sections.
    pub slot_id: Option<String>,
    /// The row ID inside the section; present exactly in a control row.
    pub control: Option<u32>,
    /// The lane; present exactly in a control row.
    pub lane: Option<ParameterChannel>,
}

impl ControlOwnerKind {
    /// The wire kind of the control core's owner kind.
    #[must_use]
    pub const fn from_controls(kind: controls::OwnerKind) -> Self {
        match kind {
            controls::OwnerKind::Session => Self::Session,
            controls::OwnerKind::Strip => Self::Strip,
            controls::OwnerKind::Route => Self::Route,
            controls::OwnerKind::Vca => Self::Vca,
        }
    }

    /// The control core's owner kind of this wire kind.
    #[must_use]
    pub const fn to_controls(self) -> controls::OwnerKind {
        match self {
            Self::Session => controls::OwnerKind::Session,
            Self::Strip => controls::OwnerKind::Strip,
            Self::Route => controls::OwnerKind::Route,
            Self::Vca => controls::OwnerKind::Vca,
        }
    }
}

impl ControlSection {
    /// The wire section of the control core's section kind.
    #[must_use]
    pub const fn from_controls(kind: controls::SectionKind) -> Self {
        match kind {
            controls::SectionKind::Inserts => Self::Inserts,
            controls::SectionKind::Builtins => Self::Builtins,
            controls::SectionKind::Console => Self::Console,
            controls::SectionKind::Own => Self::Own,
        }
    }
}

impl ControlAddress {
    /// The wire address of a control core address, with every text copied by `text` (so a host
    /// that must not abort on allocation failure passes a fallible copy). The one conversion of
    /// the kind, section and lane codes: the control list and `SET_CONTROL` both use it.
    ///
    /// # Errors
    ///
    /// The first error `text` returns.
    pub fn try_from_controls<E>(
        address: &controls::ControlAddress,
        mut text: impl FnMut(&str) -> Result<String, E>,
    ) -> Result<Self, E> {
        let owner_id = match &address.owner {
            controls::Owner::Session => None,
            controls::Owner::Strip(id) | controls::Owner::Route(id) | controls::Owner::Vca(id) => {
                Some(id)
            }
        };
        let slot_id = match &address.section {
            controls::Section::Own | controls::Section::Builtins => None,
            controls::Section::Console(id) | controls::Section::Inserts(id) => Some(id),
        };
        Ok(Self {
            owner_kind: ControlOwnerKind::from_controls(address.owner.kind()),
            owner_id: owner_id.map(|id| text(id.as_str())).transpose()?,
            section: ControlSection::from_controls(address.section.kind()),
            slot_id: slot_id.map(|id| text(id.as_str())).transpose()?,
            control: Some(address.control),
            lane: Some(ParameterChannel::from_session(address.lane)),
        })
    }
}

impl From<&controls::ControlAddress> for ControlAddress {
    fn from(address: &controls::ControlAddress) -> Self {
        match Self::try_from_controls(address, |text| {
            Ok::<_, core::convert::Infallible>(text.to_owned())
        }) {
            Ok(address) => address,
            Err(never) => match never {},
        }
    }
}

/// The control core's address of a wire address: refused as `INVALID_FIELD` unless it names one
/// lane of one control and its owner and slot IDs agree with its kind and section (the rules of
/// a control row of the list).
impl TryFrom<&ControlAddress> for controls::ControlAddress {
    type Error = DecodeError;

    fn try_from(address: &ControlAddress) -> Result<Self, DecodeError> {
        let id = |text: &Option<String>| {
            text.as_deref()
                .map(|text| session::StableId::parse(text).ok_or(DecodeError::InvalidField))
                .transpose()
        };
        let owner = match (address.owner_kind, id(&address.owner_id)?) {
            (ControlOwnerKind::Session, None) => controls::Owner::Session,
            (ControlOwnerKind::Strip, Some(id)) => controls::Owner::Strip(id),
            (ControlOwnerKind::Route, Some(id)) => controls::Owner::Route(id),
            (ControlOwnerKind::Vca, Some(id)) => controls::Owner::Vca(id),
            _ => return Err(DecodeError::InvalidField),
        };
        let section = match (address.section, id(&address.slot_id)?) {
            (ControlSection::Own, None) => controls::Section::Own,
            (ControlSection::Builtins, None) => controls::Section::Builtins,
            (ControlSection::Console, Some(id)) => controls::Section::Console(id),
            (ControlSection::Inserts, Some(id)) => controls::Section::Inserts(id),
            _ => return Err(DecodeError::InvalidField),
        };
        let (Some(control), Some(lane)) = (address.control, address.lane) else {
            return Err(DecodeError::InvalidField);
        };
        Ok(Self {
            owner,
            section,
            control,
            lane: lane.into_session(),
        })
    }
}

/// One row of the control list. A header row opens an owner or a section; a control row is one
/// value.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlEntry {
    /// The row's handle in the list's structure: nonzero and ascending down the list.
    pub handle: u32,
    /// The row's stable address. An owner header carries the `own` section.
    pub address: ControlAddress,
    /// What the row is.
    pub body: ControlBody,
}

/// The three kinds of row.
#[derive(Clone, Debug, PartialEq)]
pub enum ControlBody {
    /// Opens an owner.
    OwnerHeader(ControlOwnerHeader),
    /// Opens a section of the owner above it.
    SectionHeader(ControlSectionHeader),
    /// One value.
    Control(ControlRow),
}

/// An owner header row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlOwnerHeader {
    /// What the owner is. It must agree with the address's owner kind.
    pub kind: ControlOwnerHeaderKind,
    /// The owner's name.
    pub name: String,
    /// A route's ends; present exactly for a route owner.
    pub route: Option<ControlRouteEnds>,
}

/// The ends of a route owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlRouteEnds {
    /// The route's source.
    pub source: String,
    /// The route's destination.
    pub destination: String,
    /// The send tap the route reads.
    pub tap: session::SendTap,
}

/// A section header row. Its section is the address's.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlSectionHeader {
    /// The section's name.
    pub name: String,
    /// The effect identity of a console slot or an insert; never on another section.
    pub effect: Option<String>,
}

/// A control row: the descriptor of one value and the value.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlRow {
    /// The stable machine name.
    pub name: String,
    /// The name a person reads.
    pub display_name: String,
    /// The unit as a person reads it; empty for a unitless value.
    pub display_unit: String,
    /// The unit of the value.
    pub unit: ParameterUnit,
    /// Continuous, toggle or enumeration.
    pub kind: ControlKind,
    /// The inclusive lower bound.
    pub minimum: f32,
    /// The inclusive upper bound.
    pub maximum: f32,
    /// A value outside the range that also is accepted.
    pub disabled_value: Option<f32>,
    /// The value of a new session.
    pub default: f32,
    /// How a frontend maps the range to a control.
    pub taper: ControlTaper,
    /// The lattice step.
    pub step: f32,
    /// The digits after the point a frontend shows.
    pub precision: u8,
    /// The named step ladder; empty when there is none.
    pub ladder: Vec<f32>,
    /// The choices of an enumeration; empty for any other kind.
    pub choices: Vec<EnumChoice>,
    /// The ramp row the default ramp length comes from; absent for a fixed ramp.
    pub ramp_row: Option<ControlRampRow>,
    /// The default ramp length in samples.
    pub default_ramp_samples: u32,
    /// Live, rebuild-only or derived.
    pub class: ControlClass,
    /// Why the control is rebuild-only; present exactly when `class` is rebuild-only.
    pub reason: Option<ControlReason>,
    /// The issue that makes a ruled-live control live; present exactly with
    /// [`ControlReason::RuledLiveNotBuilt`].
    pub reason_issue: Option<u32>,
    /// The condition that reopens the question; present exactly when `class` is rebuild-only.
    pub reopen: Option<String>,
    /// Whether the value is in the session or in the host.
    pub scope: ControlScope,
    /// Whether an agent may automate the control. Only a live model control is.
    pub automatable: bool,
    /// The render family that carries a live change.
    pub family: ControlFamily,
    /// `false` for a row the owner's present shape does not use.
    pub active: bool,
    /// The committed value. On an automated lane (`automated`) of the `fader_db` row it is the
    /// lane's stored hand-move offset in dB, which the curve renders under (#1650 D3).
    pub value: f32,
    /// Whether a rendered stored automation drives the row's lane (#1650 D4). A descriptor field:
    /// an automation edit that changes it advances the structure epoch. Only an automatable row is
    /// automated.
    pub automated: bool,
}

/// One page of the control list, in strictly ascending handle order.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlListPage {
    /// The session revision the page was read at.
    pub revision: u64,
    /// The structure the handles belong to.
    pub structure_epoch: u64,
    /// The last handle of the page, or the request's cursor on an empty page.
    pub last_handle: u32,
    /// Whether the list ends with this page.
    pub eof: bool,
    /// The rows.
    pub entries: Vec<ControlEntry>,
}

/// One control's value in a state page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlStateRecord {
    /// The control row's handle.
    pub handle: u32,
    /// The committed value.
    pub value: f32,
}

/// One page of control values: the control rows of the same rows a list page of the same request
/// holds, without the header rows.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlStatePage {
    /// The session revision the page was read at.
    pub revision: u64,
    /// The structure the handles belong to.
    pub structure_epoch: u64,
    /// The last handle the page covers: at least the last record's, and the cursor for the next
    /// request. Header rows are covered but have no record.
    pub last_handle: u32,
    /// Whether the list ends with this page.
    pub eof: bool,
    /// The values, in strictly ascending handle order.
    pub records: Vec<ControlStateRecord>,
}

/// Frozen absolute transport-state registry. V1 has no pause, toggle, rate, or loop state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TransportState {
    /// The endpoint transport is stopped.
    #[default]
    Stopped = 1,
    /// The endpoint transport is playing.
    Playing = 2,
}

/// Typed `TRANSPORT_SET` command: the absolute target state. The stored position is retired
/// (#1635); the engine has one playhead, which the host moves with source seeks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportSetRequest {
    /// Absolute target state.
    pub state: TransportState,
}

/// Typed `TRANSPORT_GET`/`TRANSPORT_SET` success payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportSnapshot {
    /// Effective absolute state.
    pub state: TransportState,
    /// Engine sample at which this observation/change is effective.
    pub effective_sample: crate::SampleTime,
}

/// Typed reliable `TRANSPORT_STATE` event payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportStateEvent {
    /// Endpoint-monotonic reliable event sequence.
    pub event_sequence: u64,
    /// Effective absolute state.
    pub state: TransportState,
    /// Effective engine sample.
    pub effective_sample: crate::SampleTime,
    /// Originating set request, when this state change came from a command.
    pub origin_request_id: Option<crate::RequestId>,
}

/// The effect section an [`ObservationEnded`] names: the `RackName` codes of the two sections that
/// hold an effect (inserts `2`, console `5`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ObservationSection {
    /// One of the strip's inserts.
    Inserts = 2,
    /// A session console slot.
    Console = 5,
}

impl ObservationSection {
    /// The section's stable wire code.
    #[must_use]
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// The section a wire code names, or `None` for any other.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            2 => Some(Self::Inserts),
            5 => Some(Self::Console),
            _ => None,
        }
    }
}

/// The stable key of one observation tap, as the wire names it: the strip, the section, the
/// insert's effect ID or the console slot ID, and the effect-local tap ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationTapKey {
    /// The strip (a track or a submix) the tap belongs to.
    pub strip_id: String,
    /// Whether the tap's effect is an insert or a console slot.
    pub section: ObservationSection,
    /// The insert's effect ID or the console slot ID.
    pub effect_id: String,
    /// The effect-local tap ID.
    pub tap_id: u32,
}

impl ObservationTapKey {
    /// The order keys travel in on the wire: the strip, then console slots before inserts, then
    /// the effect ID, then the tap ID. It is the order of the host's binding table.
    fn wire_order(&self) -> (&str, bool, &str, u32) {
        (
            &self.strip_id,
            self.section == ObservationSection::Inserts,
            &self.effect_id,
            self.tap_id,
        )
    }
}

/// Typed reliable `OBSERVATION_ENDED` event payload (#1584 D6a, folded to one event per commit): the
/// taps a host had armed no longer exist, because a transaction removed their strips, inserts or
/// console slots, or reused the IDs for effects without those taps. The event's revision is the
/// revision that removed them, and `keys` is every tap that commit ended.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationEnded {
    /// Endpoint-monotonic reliable event sequence.
    pub event_sequence: u64,
    /// The taps that ended: at least one, in strictly ascending wire order (the strip, console
    /// slots before inserts, the effect ID, the tap ID), so no key repeats.
    pub keys: Vec<ObservationTapKey>,
}

/// Frozen meter component registry for one fixed telemetry record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum MeterComponent {
    /// Left channel metric for a revision-scoped meter handle.
    Left = 1,
    /// Right channel metric for a revision-scoped meter handle.
    Right = 2,
    /// Aggregate metric for a revision-scoped meter handle.
    Aggregate = 3,
}

/// One exact 16-byte meter telemetry record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterRecord {
    /// Nonzero revision-scoped meter handle.
    pub handle: u32,
    /// Explicit left/right/aggregate component.
    pub component: MeterComponent,
    /// Frozen VALID/CLIPPED/HELD bit flags.
    pub flags: u16,
    /// Metric value; it is finite whenever VALID is set.
    pub value: f32,
}

/// Borrowed typed `METER_BATCH` telemetry payload: one closed meter window (#1583 D2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterBatch<'a> {
    /// One past the window's last sample: the window is `[first_sample, observed_sample)`.
    pub observed_sample: crate::SampleTime,
    /// The window's first sample, below `observed_sample`.
    pub first_sample: crate::SampleTime,
    /// The window's sequence since the accepted configuration, from `1`.
    pub window_sequence: u64,
    /// One through 256 validated fixed meter records.
    pub records: &'a [MeterRecord],
}

/// Borrowed strict decoded fixed-record `METER_BATCH` payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedMeterBatch<'a> {
    /// One past the window's last sample.
    pub observed_sample: crate::SampleTime,
    /// The window's first sample, below `observed_sample`.
    pub first_sample: crate::SampleTime,
    /// The window's sequence since the accepted configuration, from `1`.
    pub window_sequence: u64,
    /// Exact fixed record count.
    pub count: u16,
    record_bytes: &'a [u8],
}

impl<'a> DecodedMeterBatch<'a> {
    /// Decode one bounded fixed record without allocation.
    pub fn record(self, index: usize) -> Result<MeterRecord, DecodeError> {
        if index >= usize::from(self.count) {
            return Err(DecodeError::InvalidTlv);
        }
        let offset = index.checked_mul(16).ok_or(DecodeError::LimitExceeded)?;
        let bytes = self
            .record_bytes
            .get(offset..offset + 16)
            .ok_or(DecodeError::InvalidTlv)?;
        decode_meter_record(bytes)
    }
}

/// Typed reliable `DIAGNOSTIC` event payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticEvent {
    /// One provider diagnostic, whose provider sequence is always present here.
    pub diagnostic: Diagnostic,
}

/// Frozen registered non-resetting counter IDs. IDs 2 and 11 to 14 are retired (#1629): refused,
/// never reallocated.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum CounterId {
    ControlCommandBackpressure = 1,
    ReliableResponseBackpressure = 3,
    ReliableEventBackpressure = 4,
    TelemetryCoalesced = 5,
    TelemetryDropped = 6,
    MalformedFrames = 7,
    ReplayHits = 8,
    RequestIdReuse = 9,
    ReplayExpired = 10,
    ValidationFailures = 15,
    LiveValuesSuperseded = 16,
    TransitionReprepareRefusals = 17,
    StripsRestarted = 18,
    LiveBlockErrors = 19,
    SourceUnderrunBlocks = 20,
    SourceFramesDiscarded = 21,
    /// Meter windows that never reached a `METER_BATCH` (#1583 D5).
    MeterWindowsLost = 22,
}

/// What a meter table entry measures (#1583 D1, #1690 D3): the session output, a strip by its
/// stable ID, or the master strip.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MeterTarget {
    /// The session output.
    Output,
    /// The track or submix of this stable ID. The entry stays in the table while no plan has the
    /// strip; its records carry no valid bit until a commit brings the ID back.
    Strip(Box<str>),
    /// The strip the `master` monitor row designates now (#1690 D3), whichever strip that is: the
    /// entry follows the row, and stays in the table while no strip is the master, its records
    /// then carrying no valid bit. Valid with [`MeterMetric::GainReduction`] only.
    Master,
}

impl MeterTarget {
    /// The wire code: output `1`, strip `2`, master `3`.
    #[must_use]
    pub const fn code(&self) -> u8 {
        match self {
            Self::Output => 1,
            Self::Strip(_) => 2,
            Self::Master => 3,
        }
    }
}

/// The metric a meter table entry reports (#1583 D1).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum MeterMetric {
    /// The left and right sample peak.
    Peak = 1,
    /// A strip's gain reduction (a strip or master entry only).
    GainReduction = 2,
}

impl MeterMetric {
    /// The metric of a wire code, or `None` for an unlisted one.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Peak),
            2 => Some(Self::GainReduction),
            _ => None,
        }
    }
}

/// One meter table entry (#1583 D1). Its handle is its 1-based position in the table.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MeterEntry {
    /// What it measures.
    pub target: MeterTarget,
    /// What it reports.
    pub metric: MeterMetric,
}

/// The most entries a meter table holds (#1583 D2).
pub const MAXIMUM_METER_ENTRIES: usize = 256;

/// The most counter IDs one counter list carries (#2191): the list is strictly ascending and the
/// codec refuses a longer one on encode and on decode. A retained counter table needs no more.
pub const MAXIMUM_COUNTER_ENTRIES: usize = 256;

/// Complete endpoint-local telemetry configuration. It does not create meter production.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TelemetryConfiguration {
    /// The meter table (#1583 D1): entry `i` is handle `i + 1`, 0 to 256 entries, no duplicate,
    /// no gain reduction of the output, no peak of the master.
    pub meters: Vec<MeterEntry>,
    /// Blocks per meter batch; nonzero when the table is not empty.
    pub meter_period_blocks: u32,
    /// The meter window in blocks (#1583 D4): `None` in a command; in a success echo, the
    /// window of the provider's meter delivery, `None` when it has none.
    pub meter_window_blocks: Option<core::num::NonZeroU32>,
    /// The counters a snapshot reports, ascending.
    pub counter_ids: Vec<CounterId>,
    /// Blocks per counter snapshot; nonzero when a counter is configured.
    pub counter_period_blocks: u32,
    /// Whether diagnostics are sent as events.
    pub diagnostics_enabled: bool,
    /// The least severity sent.
    pub minimum_diagnostic_severity: DiagnosticSeverity,
}

/// Typed `COUNTERS_GET` selector.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CountersRequest {
    pub all: bool,
    /// Sorted unique numeric IDs; unknown values return typed provider `NOT_FOUND`.
    pub ids: Vec<u32>,
}

/// One ascending typed non-resetting counter value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CounterValue {
    pub id: CounterId,
    pub value: u64,
}

/// Typed `COUNTERS_GET` success page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CounterSnapshot {
    pub observed_sample: crate::SampleTime,
    pub values: Vec<CounterValue>,
}

/// Borrowed counter snapshot used by caller-buffer event egress. It is schema-identical to
/// [`CounterSnapshot`] while avoiding an allocation merely to turn prepared counter slots into
/// one `COUNTER_SNAPSHOT` event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterSnapshotRef<'a> {
    /// Observation timestamp shared by every value in this batch.
    pub observed_sample: crate::SampleTime,
    /// Ascending bounded counter values.
    pub values: &'a [CounterValue],
}

/// Typed nondestructive `DIAGNOSTICS_GET` cursor request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct DiagnosticsRequest {
    pub after_sequence: u64,
    pub limit: u16,
    pub minimum_severity: DiagnosticSeverity,
}

/// Typed bounded retained diagnostics page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct DiagnosticsPage {
    pub last_sequence: u64,
    pub eof: bool,
    pub diagnostics: Vec<Diagnostic>,
}

/// One structured diagnostic path segment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathSegment {
    /// A typed/session field name.
    Field(String),
    /// A zero-based array position.
    Index(u64),
    /// A stable-ID selector.
    StableId(String),
}

/// Frozen diagnostic severity registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    /// Informational diagnostic.
    Info = 1,
    /// Warning diagnostic.
    Warning = 2,
    /// Error diagnostic.
    Error = 3,
}

impl DiagnosticSeverity {
    fn decode(value: u8) -> Result<Self, DecodeError> {
        match value {
            1 => Ok(Self::Info),
            2 => Ok(Self::Warning),
            3 => Ok(Self::Error),
            _ => Err(DecodeError::InvalidTlv),
        }
    }
}

/// One typed protocol diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Stable dotted machine-readable code.
    pub code: String,
    /// Severity in the frozen registry.
    pub severity: DiagnosticSeverity,
    /// Ordered structured path; empty represents the document root.
    pub path: Vec<PathSegment>,
    /// Bounded non-stable explanatory detail, when available.
    pub detail: Option<String>,
    /// Zero-based session-edit operation index, when applicable.
    pub operation_index: Option<u32>,
    /// Relevant absolute sample time, when applicable.
    pub sample_time: Option<u64>,
    /// Provider diagnostic sequence, when applicable.
    pub provider_sequence: Option<u64>,
}

/// Exact queue-kind codes used by typed backpressure payloads. Code 2 is retired (#1629): refused,
/// never reallocated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BackpressureQueueKind {
    /// Reliable copied control command queue.
    ControlCommand = 1,
    /// Reliable response queue.
    ReliableResponse = 3,
    /// Reliable event queue.
    ReliableEvent = 4,
    /// Explicitly lossy telemetry queue.
    Telemetry = 5,
    /// Bounded exact-byte replay cache.
    ReplayCache = 6,
    /// The spectrum capture's one-shot arm slot (#1944): capacity 1, no queue generation.
    SpectrumArm = 7,
    /// The session's one track-response capture (#1578 D3): capacity 1, no queue generation. A
    /// strip `RESPONSE_GET` is refused while a capture is in flight or its `RESPONSE_READY` is
    /// unread.
    ResponseCapture = 8,
}

impl BackpressureQueueKind {
    fn decode(value: u8) -> Result<Self, DecodeError> {
        match value {
            1 => Ok(Self::ControlCommand),
            3 => Ok(Self::ReliableResponse),
            4 => Ok(Self::ReliableEvent),
            5 => Ok(Self::Telemetry),
            6 => Ok(Self::ReplayCache),
            7 => Ok(Self::SpectrumArm),
            8 => Ok(Self::ResponseCapture),
            _ => Err(DecodeError::InvalidTlv),
        }
    }

    const fn has_generation(self) -> bool {
        !matches!(
            self,
            Self::ReplayCache | Self::SpectrumArm | Self::ResponseCapture
        )
    }
}

/// Typed queue/replay saturation data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Backpressure {
    /// Saturated resource class.
    pub queue_kind: BackpressureQueueKind,
    /// Fixed capacity in logical items or entries.
    pub capacity: u64,
    /// Occupancy before the rejected atomic attempt.
    pub occupancy: u64,
    /// Atomic requested item count; exactly one in BTLV v1.
    pub requested_items: u16,
    /// Queue generation for SPSC queue kinds; absent for replay cache and the spectrum arm slot.
    pub generation: Option<u64>,
    /// Absolute retry boundary/sample hint, when known.
    pub retry_boundary: Option<u64>,
    /// Requested copied bytes when a byte budget rejected.
    pub requested_bytes: Option<u64>,
    /// Available copied bytes when a byte budget rejected.
    pub available_bytes: Option<u64>,
}

/// The only payload permitted on a non-OK BTLV response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonOkResponse {
    /// Leading deterministic diagnostics that fit the endpoint response bound.
    pub diagnostics: Vec<Diagnostic>,
    /// Count of deterministic diagnostics omitted after the retained prefix.
    pub omitted_diagnostics: u32,
    /// Typed saturation detail present if and only if status is `BACKPRESSURE`.
    pub backpressure: Option<Backpressure>,
}

impl NonOkResponse {
    /// Retain at most `maximum` leading diagnostics and record the exact omitted suffix count.
    #[must_use]
    pub fn bounded(diagnostics: &[Diagnostic], maximum: usize) -> Self {
        let retained = diagnostics.len().min(maximum);
        Self {
            diagnostics: diagnostics[..retained].to_vec(),
            omitted_diagnostics: u32::try_from(diagnostics.len().saturating_sub(retained))
                .unwrap_or(u32::MAX),
            backpressure: None,
        }
    }
}

impl ProtocolCodec {
    /// Return the exact direct-caller-buffer length for one common non-OK response payload.
    pub fn encoded_non_ok_payload_len(&self, value: &NonOkResponse) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_non_ok(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode one canonical common non-OK response payload into caller output.
    ///
    /// A short output buffer is never modified. This path has no heap allocation after the typed
    /// `NonOkResponse` input already exists.
    pub fn encode_non_ok_payload(
        &self,
        value: &NonOkResponse,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_non_ok_payload_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_non_ok(self, &mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Decode one common non-OK response payload after the outer frame has supplied its count.
    pub fn decode_non_ok_payload(
        &self,
        payload: &[u8],
        tlv_count: u32,
    ) -> Result<NonOkResponse, DecodeError> {
        if tlv_count > self.limits().max_tlv_count || payload.len() > self.limits().max_frame_bytes
        {
            return Err(DecodeError::LimitExceeded);
        }
        decode_non_ok(self, Message::top_level(payload, tlv_count, self.limits())?)
    }

    /// Exact payload length for the 23-field successful capabilities response.
    pub fn encoded_capabilities_len(&self, value: &Capabilities<'_>) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_capabilities(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a successful capabilities payload directly into caller-owned output.
    pub fn encode_capabilities(
        &self,
        value: &Capabilities<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_capabilities_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_capabilities(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the exact 23-field successful capabilities payload.
    pub fn decode_capabilities<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedCapabilities<'a>, DecodeError> {
        decode_capabilities(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Validate the empty capabilities command payload, allowing only skippable future fields.
    pub fn decode_capabilities_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<(), DecodeError> {
        Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::capabilities_request::SPEC)
            .map(|_| ())
    }

    /// Exact caller-output length for a snapshot request.
    pub const fn encoded_snapshot_request_len(&self, _value: SessionSnapshotRequest) -> usize {
        32
    }

    /// Encode the typed two-field snapshot request.
    pub fn encode_snapshot_request(
        &self,
        value: SessionSnapshotRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        if value.maximum_bytes == 0 {
            return Err(EncodeError::LimitExceeded);
        }
        let required = self.encoded_snapshot_request_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_snapshot_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the typed two-field snapshot request.
    pub fn decode_snapshot_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionSnapshotRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::snapshot_request::SPEC)?;
        let result = SessionSnapshotRequest {
            offset: read_u64(one_spec!(message, schema::snapshot_request::OFFSET)?)?,
            maximum_bytes: read_u32(one_spec!(message, schema::snapshot_request::MAXIMUM_BYTES)?)?,
        };
        if result.maximum_bytes == 0 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Exact caller-output length for a snapshot success payload.
    pub fn encoded_snapshot_len(&self, value: SessionSnapshot<'_>) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_snapshot(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a typed canonical-JSON snapshot chunk.
    pub fn encode_snapshot(
        &self,
        value: SessionSnapshot<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_snapshot_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed snapshot success payload.
    pub fn decode_snapshot<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<SessionSnapshot<'a>, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::snapshot::SPEC)?;
        let result = SessionSnapshot {
            total_bytes: read_u64(one_spec!(message, schema::snapshot::TOTAL_BYTES)?)?,
            offset: read_u64(one_spec!(message, schema::snapshot::OFFSET)?)?,
            canonical_json_chunk: one_spec!(message, schema::snapshot::CANONICAL_JSON_CHUNK)?,
            eof: read_bool(one_spec!(message, schema::snapshot::EOF)?)?,
        };
        if result.offset > result.total_bytes
            || u64::try_from(result.canonical_json_chunk.len())
                .map_err(|_| DecodeError::LimitExceeded)?
                > result.total_bytes.saturating_sub(result.offset)
            || result.eof
                != (result
                    .offset
                    .saturating_add(result.canonical_json_chunk.len() as u64)
                    == result.total_bytes)
        {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(result)
    }

    /// Encode the typed two-field `SESSION_REPLACE` request. `document_bytes` 0 is refused.
    pub fn encode_session_replace_request(
        &self,
        value: SessionReplaceRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_session_replace_request(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_session_replace_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the typed two-field `SESSION_REPLACE` request: `document_bytes` 0, or a
    /// digest that is not exactly 32 bytes, is `INVALID_FIELD`.
    pub fn decode_session_replace_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionReplaceRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::session_replace_request::SPEC)?;
        let document_bytes = read_u32(one_spec!(
            message,
            schema::session_replace_request::DOCUMENT_BYTES
        )?)?;
        let document_sha256: [u8; SESSION_REPLACE_DIGEST_BYTES] =
            one_spec!(message, schema::session_replace_request::DOCUMENT_SHA256)?
                .try_into()
                .map_err(|_| DecodeError::InvalidField)?;
        if document_bytes == 0 {
            return Err(DecodeError::InvalidField);
        }
        Ok(SessionReplaceRequest {
            document_bytes,
            document_sha256,
        })
    }

    /// Encode/decode helpers for the transaction success payload.
    pub fn encode_transaction_applied(
        &self,
        value: &TransactionApplied,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_transaction_applied(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transaction_applied(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode the transaction success payload: a present tag list is not empty.
    pub fn decode_transaction_applied(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransactionApplied, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::transaction_applied::SPEC)?;
        Ok(TransactionApplied {
            applied_operations: read_u32(one_spec!(
                message,
                schema::transaction_applied::APPLIED_OPERATIONS
            )?)?,
            edit_path: EditPath::from_code(read_u8(one_spec!(
                message,
                schema::transaction_applied::EDIT_PATH
            )?)?)
            .ok_or(DecodeError::InvalidTlv)?,
            observe_arm_tags: decode_arm_tags(optional_spec!(
                message,
                schema::transaction_applied::OBSERVE_ARM_TAGS
            )?)?,
            removed_arm_tags: decode_arm_tags(optional_spec!(
                message,
                schema::transaction_applied::REMOVED_ARM_TAGS
            )?)?,
        })
    }

    /// Encode/decode helpers for the four-field reliable session-committed event.
    pub fn encode_session_committed(
        &self,
        value: SessionCommitted,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_session_committed(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_session_committed(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable session-committed event payload.
    pub fn decode_session_committed(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<SessionCommitted, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::session_committed::SPEC)?;
        Ok(SessionCommitted {
            event_sequence: read_u64(one_spec!(
                message,
                schema::session_committed::EVENT_SEQUENCE
            )?)?,
            origin_request_id: crate::RequestId::new(read_u64(one_spec!(
                message,
                schema::session_committed::ORIGIN_REQUEST_ID
            )?)?)
            .ok_or(DecodeError::InvalidTlv)?,
            previous_revision: crate::SessionRevision(read_u64(one_spec!(
                message,
                schema::session_committed::PREVIOUS_REVISION
            )?)?),
            applied_operations: read_u32(one_spec!(
                message,
                schema::session_committed::APPLIED_OPERATIONS
            )?)?,
        })
    }

    /// Return the exact caller-buffer size of a control list or control state request.
    pub fn encoded_control_list_request_len(
        &self,
        value: &ControlListRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_control_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }
    /// Encode the request that `CONTROL_LIST_GET` and `CONTROL_STATE_GET` share, without
    /// allocation.
    pub fn encode_control_list_request(
        &self,
        value: &ControlListRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_control_list_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_control_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode the request that `CONTROL_LIST_GET` and `CONTROL_STATE_GET` share.
    pub fn decode_control_list_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ControlListRequest, DecodeError> {
        decode_control_request(self, Message::top_level(payload, count, self.limits())?)
    }
    /// Return the exact caller-buffer size of a control list page.
    pub fn encoded_control_list_page_len(
        &self,
        value: &ControlListPage,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_control_list_page(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }
    /// Encode a control list page without allocation.
    pub fn encode_control_list_page(
        &self,
        value: &ControlListPage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_control_list_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_control_list_page(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode a control list page.
    pub fn decode_control_list_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ControlListPage, DecodeError> {
        decode_control_list_page(self, Message::top_level(payload, count, self.limits())?)
    }
    /// Return the exact caller-buffer size of a control state page.
    pub fn encoded_control_state_page_len(
        &self,
        value: &ControlStatePage,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_control_state_page(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }
    /// Encode a control state page of fixed eight-byte records without allocation.
    pub fn encode_control_state_page(
        &self,
        value: &ControlStatePage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_control_state_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_control_state_page(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }
    /// Strictly decode a control state page.
    pub fn decode_control_state_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ControlStatePage, DecodeError> {
        decode_control_state_page(Message::top_level(payload, count, self.limits())?)
    }

    /// Encode the canonical empty `TRANSPORT_GET` command payload.
    pub const fn encode_transport_get_request(
        &self,
        _output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        Ok(0)
    }

    /// Strictly decode an empty `TRANSPORT_GET` request, skipping only unknown optional fields.
    pub fn decode_transport_get_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<(), DecodeError> {
        Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::transport_get::SPEC)
            .map(|_| ())
    }

    /// Return the exact caller-output size for a typed absolute transport set request.
    pub const fn encoded_transport_set_request_len(&self, _value: TransportSetRequest) -> usize {
        16
    }

    /// Encode a typed absolute transport-state set request without allocation.
    pub fn encode_transport_set_request(
        &self,
        value: TransportSetRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_transport_set_request_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_set(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed absolute transport-state set request without allocation.
    pub fn decode_transport_set_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSetRequest, DecodeError> {
        // Field 2, the stored position, is retired (#1635): refused in either flag form.
        let fields = Message::top_level(payload, count, self.limits())?.schema_spec_retiring(
            &schema::transport_set::SPEC,
            &schema::transport_set::RETIRED,
        )?;
        Ok(TransportSetRequest {
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_set::STATE
            )?)?)?,
        })
    }

    /// Encode a typed two-field transport snapshot directly into caller output.
    pub fn encode_transport_snapshot(
        &self,
        value: TransportSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_transport_snapshot(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed two-field transport snapshot without allocation.
    pub fn decode_transport_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportSnapshot, DecodeError> {
        // Field 2, the stored position, is retired (#1635).
        let fields = Message::top_level(payload, count, self.limits())?.schema_spec_retiring(
            &schema::transport_snapshot::SPEC,
            &schema::transport_snapshot::RETIRED,
        )?;
        Ok(TransportSnapshot {
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_snapshot::STATE
            )?)?)?,
            effective_sample: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_snapshot::EFFECTIVE_SAMPLE
            )?)?),
        })
    }

    /// Return the exact caller-output size for a reliable typed transport-state event.
    pub const fn encoded_transport_state_event_len(&self, value: TransportStateEvent) -> usize {
        if value.origin_request_id.is_some() {
            64
        } else {
            48
        }
    }

    /// Encode a reliable typed transport-state event payload without allocation.
    pub fn encode_transport_state_event(
        &self,
        value: TransportStateEvent,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_transport_state_event_len(value);
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_transport_state_event(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable typed transport-state event payload without allocation.
    pub fn decode_transport_state_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TransportStateEvent, DecodeError> {
        // Field 3, the stored position, is retired (#1635).
        let fields = Message::top_level(payload, count, self.limits())?.schema_spec_retiring(
            &schema::transport_state_event::SPEC,
            &schema::transport_state_event::RETIRED,
        )?;
        Ok(TransportStateEvent {
            event_sequence: read_u64(one_spec!(
                fields,
                schema::transport_state_event::EVENT_SEQUENCE
            )?)?,
            state: parse_transport_state(read_u8(one_spec!(
                fields,
                schema::transport_state_event::STATE
            )?)?)?,
            effective_sample: crate::SampleTime(read_u64(one_spec!(
                fields,
                schema::transport_state_event::EFFECTIVE_SAMPLE
            )?)?),
            origin_request_id: match optional_spec!(
                fields,
                schema::transport_state_event::ORIGIN_REQUEST_ID
            )?
            .map(read_u64)
            .transpose()?
            {
                Some(value) => Some(crate::RequestId::new(value).ok_or(DecodeError::InvalidTlv)?),
                None => None,
            },
        })
    }

    /// Return the exact caller-buffer size of a reliable typed observation-ended event.
    pub fn encoded_observation_ended_len(
        &self,
        value: &ObservationEnded,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_observation_ended(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a reliable typed observation-ended event payload into caller output.
    pub fn encode_observation_ended(
        &self,
        value: &ObservationEnded,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_observation_ended_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_observation_ended(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a reliable typed observation-ended event payload. Every ID must be a stable
    /// ID, every section an effect section, and the keys at least one and strictly ascending.
    pub fn decode_observation_ended(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ObservationEnded, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?
            .schema_spec(&schema::observation_ended::SPEC)?;
        let raw_keys = values_spec!(message, schema::observation_ended::KEY)?;
        let mut keys = Vec::with_capacity(raw_keys.count());
        for raw in values_spec!(message, schema::observation_ended::KEY)? {
            let fields = Message::nested_at_depth(raw, self.limits(), 1)?;
            let fields = fields.schema_spec(&schema::observation_tap_key::SPEC)?;
            let id = |spec: schema::FieldSpec| -> Result<String, DecodeError> {
                let text = read_string(self, one_spec!(fields, spec)?)?;
                session::StableId::parse(text).ok_or(DecodeError::InvalidField)?;
                Ok(text.to_owned())
            };
            let key = ObservationTapKey {
                strip_id: id(schema::observation_tap_key::STRIP_ID)?,
                section: ObservationSection::from_code(read_u8(one_spec!(
                    fields,
                    schema::observation_tap_key::SECTION
                )?)?)
                .ok_or(DecodeError::InvalidField)?,
                effect_id: id(schema::observation_tap_key::EFFECT_ID)?,
                tap_id: read_u32(one_spec!(fields, schema::observation_tap_key::TAP_ID)?)?,
            };
            if keys
                .last()
                .is_some_and(|last: &ObservationTapKey| last.wire_order() >= key.wire_order())
            {
                return Err(DecodeError::InvalidField);
            }
            keys.push(key);
        }
        if keys.is_empty() {
            return Err(DecodeError::InvalidField);
        }
        Ok(ObservationEnded {
            event_sequence: read_u64(one_spec!(
                message,
                schema::observation_ended::EVENT_SEQUENCE
            )?)?,
            keys,
        })
    }

    /// Return the exact caller-buffer bytes required for one fixed-record meter batch.
    pub fn encoded_meter_batch_len(&self, value: MeterBatch<'_>) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_meter_batch(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a canonical fixed-record lossy `METER_BATCH` payload without allocation.
    pub fn encode_meter_batch(
        &self,
        value: MeterBatch<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_meter_batch_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_meter_batch(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a bounded borrowed fixed-record lossy `METER_BATCH` payload.
    pub fn decode_meter_batch<'a>(
        &self,
        payload: &'a [u8],
        count: u32,
    ) -> Result<DecodedMeterBatch<'a>, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::meter_batch::SPEC)?;
        let result = DecodedMeterBatch {
            observed_sample: crate::SampleTime(read_u64(one_spec!(
                message,
                schema::meter_batch::OBSERVED_SAMPLE
            )?)?),
            first_sample: crate::SampleTime(read_u64(one_spec!(
                message,
                schema::meter_batch::FIRST_SAMPLE
            )?)?),
            window_sequence: read_u64(one_spec!(message, schema::meter_batch::WINDOW_SEQUENCE)?)?,
            count: read_u16(one_spec!(message, schema::meter_batch::COUNT)?)?,
            record_bytes: one_spec!(message, schema::meter_batch::RECORDS)?,
        };
        if !meter_window_is_valid(
            result.first_sample,
            result.observed_sample,
            result.window_sequence,
        ) {
            return Err(DecodeError::InvalidField);
        }
        if result.count == 0
            || result.count > 256
            || read_u16(one_spec!(message, schema::meter_batch::RECORD_BYTES)?)? != 16
            || result.record_bytes.len()
                != usize::from(result.count)
                    .checked_mul(16)
                    .ok_or(DecodeError::LimitExceeded)?
        {
            return Err(DecodeError::InvalidTlv);
        }
        for index in 0..usize::from(result.count) {
            let _ = result.record(index)?;
        }
        Ok(result)
    }

    /// Encode the lossy `COUNTER_SNAPSHOT` event payload using its exact shared counter schema.
    pub fn encode_counter_snapshot_event(
        &self,
        value: &CounterSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_counter_snapshot(value, output)
    }

    /// Strictly decode the lossy `COUNTER_SNAPSHOT` event payload.
    pub fn decode_counter_snapshot_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CounterSnapshot, DecodeError> {
        self.decode_counter_snapshot(payload, count)
    }

    /// Return the exact caller-buffer length for one reliable typed diagnostic event.
    pub fn encoded_diagnostic_event_len(
        &self,
        value: &DiagnosticEvent,
    ) -> Result<usize, EncodeError> {
        self.encoded_diagnostic_ref_len(&value.diagnostic)
    }

    /// Return the exact caller-buffer size for one borrowed reliable diagnostic event.
    pub fn encoded_diagnostic_ref_len(
        &self,
        diagnostic: &Diagnostic,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_diagnostic_event(self, &mut sink, diagnostic)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode one reliable typed diagnostic event directly into caller output without allocation.
    pub fn encode_diagnostic_event(
        &self,
        value: &DiagnosticEvent,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_diagnostic_ref(&value.diagnostic, output)
    }

    /// Encode one borrowed reliable diagnostic event directly into caller output.
    pub fn encode_diagnostic_ref(
        &self,
        diagnostic: &Diagnostic,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_diagnostic_ref_len(diagnostic)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostic_event(self, &mut writer, diagnostic)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode one reliable typed diagnostic event requiring provider sequence field 7.
    pub fn decode_diagnostic_event(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticEvent, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostic_event::SPEC)?;
        let diagnostic = decode_diagnostic(
            self,
            Message::nested_at_depth(
                one_spec!(message, schema::diagnostic_event::DIAGNOSTIC)?,
                self.limits(),
                1,
            )?,
        )?;
        if diagnostic.provider_sequence.is_none() {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(DiagnosticEvent { diagnostic })
    }

    /// Return the direct caller-buffer size for a complete telemetry configuration echo.
    pub fn encoded_telemetry_configuration_len(
        &self,
        value: &TelemetryConfiguration,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_telemetry_configuration(&mut sink, value, true)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a canonical telemetry configuration success echo without allocation (a command is an
    /// echo without a meter window).
    pub fn encode_telemetry_configuration(
        &self,
        value: &TelemetryConfiguration,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_telemetry_configuration_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_telemetry_configuration(&mut writer, value, true)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a telemetry configuration command, which carries no meter window.
    pub fn decode_telemetry_configuration(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TelemetryConfiguration, DecodeError> {
        self.decode_telemetry_configuration_as(payload, count, false)
    }

    /// Strictly decode a canonical telemetry configuration success echo, which may carry the
    /// provider's meter window (#1583 D4).
    pub fn decode_telemetry_configuration_echo(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<TelemetryConfiguration, DecodeError> {
        self.decode_telemetry_configuration_as(payload, count, true)
    }

    fn decode_telemetry_configuration_as(
        &self,
        payload: &[u8],
        count: u32,
        echo: bool,
    ) -> Result<TelemetryConfiguration, DecodeError> {
        // Field 1, the positional meter handles, is retired (#1583 D2): refused in either flag
        // form.
        let message = Message::top_level(payload, count, self.limits())?.schema_spec_retiring(
            &schema::telemetry_configuration::SPEC,
            &schema::telemetry_configuration::RETIRED,
        )?;
        let mut meters = Vec::new();
        for raw in values_spec!(message, schema::telemetry_configuration::METER)? {
            if meters.len() == MAXIMUM_METER_ENTRIES {
                return Err(DecodeError::InvalidField);
            }
            let entry = Message::nested_at_depth(raw, self.limits(), 1)?
                .schema_spec(&schema::meter_entry::SPEC)?;
            let strip_id = optional_spec!(entry, schema::meter_entry::STRIP_ID)?
                .map(|text| read_string(self, text))
                .transpose()?;
            let target = match (
                read_u8(one_spec!(entry, schema::meter_entry::TARGET)?)?,
                strip_id,
            ) {
                (1, None) => MeterTarget::Output,
                (2, Some(id)) => MeterTarget::Strip(id.into()),
                (3, None) => MeterTarget::Master,
                _ => return Err(DecodeError::InvalidField),
            };
            let metric =
                MeterMetric::from_code(read_u8(one_spec!(entry, schema::meter_entry::METRIC)?)?)
                    .ok_or(DecodeError::InvalidField)?;
            meters.push(MeterEntry { target, metric });
        }
        let meter_window_blocks = optional_spec!(
            message,
            schema::telemetry_configuration::METER_WINDOW_BLOCKS
        )?
        .map(|bytes| {
            read_u32(bytes).and_then(|blocks| {
                core::num::NonZeroU32::new(blocks).ok_or(DecodeError::InvalidField)
            })
        })
        .transpose()?;
        if meter_window_blocks.is_some() && !echo {
            return Err(DecodeError::InvalidField);
        }
        let result = TelemetryConfiguration {
            meters,
            meter_period_blocks: read_u32(one_spec!(
                message,
                schema::telemetry_configuration::METER_PERIOD_BLOCKS
            )?)?,
            meter_window_blocks,
            counter_ids: decode_counter_ids(one_spec!(
                message,
                schema::telemetry_configuration::COUNTER_IDS
            )?)?,
            counter_period_blocks: read_u32(one_spec!(
                message,
                schema::telemetry_configuration::COUNTER_PERIOD_BLOCKS
            )?)?,
            diagnostics_enabled: read_bool(one_spec!(
                message,
                schema::telemetry_configuration::DIAGNOSTICS_ENABLED
            )?)?,
            minimum_diagnostic_severity: DiagnosticSeverity::decode(read_u8(one_spec!(
                message,
                schema::telemetry_configuration::MINIMUM_DIAGNOSTIC_SEVERITY
            )?)?)?,
        };
        if !meter_table_is_valid(&result) {
            return Err(DecodeError::InvalidField);
        }
        validate_telemetry_configuration(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Return exact caller-buffer bytes for one typed counters selector.
    pub fn encoded_counters_request_len(
        &self,
        value: &CountersRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_counters_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a typed counters selector without allocation.
    pub fn encode_counters_request(
        &self,
        value: &CountersRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_counters_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_counters_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode an all-or-explicit-ID counters selector.
    pub fn decode_counters_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CountersRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::counters_request::SPEC)?;
        let all = read_bool(one_spec!(message, schema::counters_request::ALL)?)?;
        let ids = optional_spec!(message, schema::counters_request::IDS)?
            .map(|bytes| decode_nonzero_u32s(bytes, true))
            .transpose()?
            .unwrap_or_default();
        let result = CountersRequest { all, ids };
        validate_counters_request(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Return exact caller-buffer bytes for one typed nondestructive counter snapshot.
    pub fn encoded_counter_snapshot_len(
        &self,
        value: &CounterSnapshot,
    ) -> Result<usize, EncodeError> {
        self.encoded_counter_snapshot_ref_len(CounterSnapshotRef {
            observed_sample: value.observed_sample,
            values: &value.values,
        })
    }

    /// Return exact caller-buffer bytes for a borrowed fixed/prepared counter snapshot.
    pub fn encoded_counter_snapshot_ref_len(
        &self,
        value: CounterSnapshotRef<'_>,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_counter_snapshot(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a typed ascending nondestructive counter snapshot without allocation.
    pub fn encode_counter_snapshot(
        &self,
        value: &CounterSnapshot,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        self.encode_counter_snapshot_ref(
            CounterSnapshotRef {
                observed_sample: value.observed_sample,
                values: &value.values,
            },
            output,
        )
    }

    /// Encode a borrowed ascending counter snapshot without allocation.
    pub fn encode_counter_snapshot_ref(
        &self,
        value: CounterSnapshotRef<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_counter_snapshot_ref_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_counter_snapshot(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed ascending nondestructive counter snapshot.
    pub fn decode_counter_snapshot(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<CounterSnapshot, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::counter_snapshot::SPEC)?;
        let mut values =
            Vec::with_capacity(values_spec!(message, schema::counter_snapshot::VALUE)?.count());
        for raw in values_spec!(message, schema::counter_snapshot::VALUE)? {
            let counter = Message::nested_at_depth(raw, self.limits(), 1)?;
            let counter = counter.schema_spec(&schema::counter_value::SPEC)?;
            values.push(CounterValue {
                id: parse_counter_id(read_u32(one_spec!(counter, schema::counter_value::ID)?)?)?,
                value: read_u64(one_spec!(counter, schema::counter_value::VALUE)?)?,
            });
        }
        let result = CounterSnapshot {
            observed_sample: crate::SampleTime(read_u64(one_spec!(
                message,
                schema::counter_snapshot::OBSERVED_SAMPLE
            )?)?),
            values,
        };
        validate_counter_snapshot(&result).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(result)
    }

    /// Encode the exact three-field diagnostics cursor request without allocation.
    pub fn encode_diagnostics_request(
        &self,
        value: DiagnosticsRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let mut sizing = CountSink::new(self.limits());
        write_diagnostics_request(&mut sizing, value)?;
        let required = checked_sink_len(self, &mut sizing)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostics_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed diagnostics cursor request.
    pub fn decode_diagnostics_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsRequest, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostics_request::SPEC)?;
        let value = DiagnosticsRequest {
            after_sequence: read_u64(one_spec!(
                message,
                schema::diagnostics_request::AFTER_SEQUENCE
            )?)?,
            limit: read_u16(one_spec!(message, schema::diagnostics_request::LIMIT)?)?,
            minimum_severity: DiagnosticSeverity::decode(read_u8(one_spec!(
                message,
                schema::diagnostics_request::MINIMUM_SEVERITY
            )?)?)?,
        };
        if value.limit == 0 || value.limit > 256 {
            return Err(DecodeError::InvalidTlv);
        }
        Ok(value)
    }

    /// Return exact caller-buffer bytes for a typed diagnostics page.
    pub fn encoded_diagnostics_page_len(
        &self,
        value: &DiagnosticsPage,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_diagnostics_page(self, &mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode a typed diagnostics page directly into caller output without allocation.
    pub fn encode_diagnostics_page(
        &self,
        value: &DiagnosticsPage,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_diagnostics_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_diagnostics_page(self, &mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode a typed bounded diagnostics page.
    pub fn decode_diagnostics_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<DiagnosticsPage, DecodeError> {
        let message = Message::top_level(payload, count, self.limits())?;
        let message = message.schema_spec(&schema::diagnostics_page::SPEC)?;
        let diagnostics = values_spec!(message, schema::diagnostics_page::DIAGNOSTIC)?
            .map(|raw| decode_diagnostic(self, Message::nested_at_depth(raw, self.limits(), 1)?))
            .collect::<Result<Vec<_>, _>>()?;
        let value = DiagnosticsPage {
            last_sequence: read_u64(one_spec!(message, schema::diagnostics_page::LAST_SEQUENCE)?)?,
            eof: read_bool(one_spec!(message, schema::diagnostics_page::EOF)?)?,
            diagnostics,
        };
        validate_diagnostics_page(&value).map_err(|_| DecodeError::InvalidTlv)?;
        Ok(value)
    }
}

fn parse_transport_state(value: u8) -> Result<TransportState, DecodeError> {
    match value {
        1 => Ok(TransportState::Stopped),
        2 => Ok(TransportState::Playing),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn parse_meter_component(value: u16) -> Result<MeterComponent, DecodeError> {
    match value {
        1 => Ok(MeterComponent::Left),
        2 => Ok(MeterComponent::Right),
        3 => Ok(MeterComponent::Aggregate),
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn validate_meter_records(records: &[MeterRecord]) -> Result<(), EncodeError> {
    if records.is_empty() || records.len() > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    for record in records {
        if record.handle == 0
            || record.flags & !0b111 != 0
            || (record.flags & 1 != 0 && !record.value.is_finite())
        {
            return Err(EncodeError::LimitExceeded);
        }
    }
    Ok(())
}

fn encode_meter_record(value: &MeterRecord, output: &mut [u8]) {
    debug_assert_eq!(output.len(), 16);
    output[..4].copy_from_slice(&value.handle.to_le_bytes());
    output[4..6].copy_from_slice(&(value.component as u16).to_le_bytes());
    output[6..8].copy_from_slice(&value.flags.to_le_bytes());
    output[8..12].copy_from_slice(&value.value.to_le_bytes());
    output[12..16].fill(0);
}

fn decode_meter_record(value: &[u8]) -> Result<MeterRecord, DecodeError> {
    if value.len() != 16 || value[12..].iter().any(|byte| *byte != 0) {
        return Err(DecodeError::InvalidTlv);
    }
    let record = MeterRecord {
        handle: u32::from_le_bytes(value[..4].try_into().map_err(|_| DecodeError::Truncated)?),
        component: parse_meter_component(u16::from_le_bytes(
            value[4..6].try_into().map_err(|_| DecodeError::Truncated)?,
        ))?,
        flags: u16::from_le_bytes(value[6..8].try_into().map_err(|_| DecodeError::Truncated)?),
        value: f32::from_le_bytes(
            value[8..12]
                .try_into()
                .map_err(|_| DecodeError::Truncated)?,
        ),
    };
    if record.handle == 0
        || record.flags & !0b111 != 0
        || (record.flags & 1 != 0 && !record.value.is_finite())
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(record)
}

fn parse_counter_id(value: u32) -> Result<CounterId, DecodeError> {
    match value {
        1 => Ok(CounterId::ControlCommandBackpressure),
        3 => Ok(CounterId::ReliableResponseBackpressure),
        4 => Ok(CounterId::ReliableEventBackpressure),
        5 => Ok(CounterId::TelemetryCoalesced),
        6 => Ok(CounterId::TelemetryDropped),
        7 => Ok(CounterId::MalformedFrames),
        8 => Ok(CounterId::ReplayHits),
        9 => Ok(CounterId::RequestIdReuse),
        10 => Ok(CounterId::ReplayExpired),
        15 => Ok(CounterId::ValidationFailures),
        16 => Ok(CounterId::LiveValuesSuperseded),
        17 => Ok(CounterId::TransitionReprepareRefusals),
        18 => Ok(CounterId::StripsRestarted),
        19 => Ok(CounterId::LiveBlockErrors),
        20 => Ok(CounterId::SourceUnderrunBlocks),
        21 => Ok(CounterId::SourceFramesDiscarded),
        22 => Ok(CounterId::MeterWindowsLost),
        _ => Err(DecodeError::InvalidField),
    }
}

fn decode_nonzero_u32s(bytes: &[u8], allow_empty: bool) -> Result<Vec<u32>, DecodeError> {
    if !bytes.len().is_multiple_of(4) {
        return Err(DecodeError::InvalidValueLength);
    }
    let result = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| read_u32(word))
        .collect::<Result<Vec<_>, _>>()?;
    if (!allow_empty && result.is_empty())
        || result.len() > 256
        || result.contains(&0)
        || result.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(result)
}

fn decode_counter_ids(bytes: &[u8]) -> Result<Vec<CounterId>, DecodeError> {
    let values = decode_nonzero_u32s(bytes, true)?;
    values.into_iter().map(parse_counter_id).collect()
}

fn check_counter_ids(ids: &[CounterId], allow_empty: bool) -> Result<(), DecodeError> {
    if (!allow_empty && ids.is_empty())
        || ids.len() > MAXIMUM_COUNTER_ENTRIES
        || ids.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

/// The meter table rules (#1583 D2, #1690 D3), `INVALID_FIELD` on encode and on decode: at most
/// 256 entries, a strip ID that is a stable ID, no gain reduction of the output, the master's gain
/// reduction only, no duplicate entry, and a nonzero period when the table is not empty.
fn meter_table_is_valid(value: &TelemetryConfiguration) -> bool {
    let meters = &value.meters;
    meters.len() <= MAXIMUM_METER_ENTRIES
        && (meters.is_empty() || value.meter_period_blocks != 0)
        && meters.iter().enumerate().all(|(index, entry)| {
            let target = match &entry.target {
                MeterTarget::Output => entry.metric != MeterMetric::GainReduction,
                MeterTarget::Strip(id) => valid_stable_id(id),
                MeterTarget::Master => entry.metric == MeterMetric::GainReduction,
            };
            target
                && meters
                    .get(..index)
                    .is_some_and(|before| !before.contains(entry))
        })
}

/// The span rule of a `METER_BATCH` (#1583 D2): a nonempty window and a sequence from `1`.
fn meter_window_is_valid(
    first_sample: crate::SampleTime,
    end_sample: crate::SampleTime,
    window_sequence: u64,
) -> bool {
    first_sample.0 < end_sample.0 && window_sequence != 0
}

fn validate_telemetry_configuration(value: &TelemetryConfiguration) -> Result<(), DecodeError> {
    if !value.counter_ids.is_empty() && value.counter_period_blocks == 0 {
        return Err(DecodeError::InvalidTlv);
    }
    check_counter_ids(&value.counter_ids, true)
}

fn validate_counters_request(value: &CountersRequest) -> Result<(), DecodeError> {
    if value.all != value.ids.is_empty() {
        return Err(DecodeError::InvalidTlv);
    }
    if value.ids.len() > 256
        || value.ids.contains(&0)
        || value.ids.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_counter_snapshot(value: &CounterSnapshot) -> Result<(), DecodeError> {
    validate_counter_snapshot_ref(CounterSnapshotRef {
        observed_sample: value.observed_sample,
        values: &value.values,
    })
}

fn validate_counter_snapshot_ref(value: CounterSnapshotRef<'_>) -> Result<(), DecodeError> {
    if value.values.len() > 256 || value.values.windows(2).any(|pair| pair[0].id >= pair[1].id) {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}

fn validate_diagnostics_page(value: &DiagnosticsPage) -> Result<(), EncodeError> {
    if value.diagnostics.len() > 256
        || value
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.provider_sequence.is_none())
        || value
            .diagnostics
            .windows(2)
            .any(|pair| pair[0].provider_sequence >= pair[1].provider_sequence)
        || value
            .diagnostics
            .last()
            .is_some_and(|diagnostic| diagnostic.provider_sequence != Some(value.last_sequence))
    {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn write_counter_ids(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[CounterId],
) -> Result<(), EncodeError> {
    let length = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, length, &mut |sink| {
        for value in values {
            sink.raw(&(*value as u32).to_le_bytes())?;
        }
        Ok(())
    })
}

fn write_u32s(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[u32],
) -> Result<(), EncodeError> {
    let length = values
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, length, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
}

pub(crate) fn write_snapshot_request(
    sink: &mut dyn Sink,
    value: SessionSnapshotRequest,
) -> Result<(), EncodeError> {
    if value.maximum_bytes == 0 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::snapshot_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::snapshot_request::OFFSET,
        &value.offset.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::snapshot_request::MAXIMUM_BYTES,
        &value.maximum_bytes.to_le_bytes()
    )
}

pub(crate) fn write_snapshot(
    sink: &mut dyn Sink,
    value: SessionSnapshot<'_>,
) -> Result<(), EncodeError> {
    if value.offset > value.total_bytes
        || u64::try_from(value.canonical_json_chunk.len())
            .map_err(|_| EncodeError::LimitExceeded)?
            > value.total_bytes.saturating_sub(value.offset)
        || value.eof
            != (value
                .offset
                .saturating_add(value.canonical_json_chunk.len() as u64)
                == value.total_bytes)
    {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::snapshot::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::snapshot::TOTAL_BYTES,
        &value.total_bytes.to_le_bytes()
    )?;
    write_spec!(sink, schema::snapshot::OFFSET, &value.offset.to_le_bytes())?;
    write_spec!(
        sink,
        schema::snapshot::CANONICAL_JSON_CHUNK,
        value.canonical_json_chunk
    )?;
    write_spec!(sink, schema::snapshot::EOF, &[u8::from(value.eof)])
}

pub(crate) fn write_transaction_applied(
    sink: &mut dyn Sink,
    value: &TransactionApplied,
) -> Result<(), EncodeError> {
    use schema::transaction_applied::{OBSERVE_ARM_TAGS, REMOVED_ARM_TAGS};
    sink.check_field_count(schema::transaction_applied::SPEC.field_count(&[
        (
            OBSERVE_ARM_TAGS,
            usize::from(!value.observe_arm_tags.is_empty()),
        ),
        (
            REMOVED_ARM_TAGS,
            usize::from(!value.removed_arm_tags.is_empty()),
        ),
    ])?)?;
    write_spec!(
        sink,
        schema::transaction_applied::APPLIED_OPERATIONS,
        &value.applied_operations.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::transaction_applied::EDIT_PATH,
        &[value.edit_path.code()]
    )?;
    for (spec, tags) in [
        (OBSERVE_ARM_TAGS, &value.observe_arm_tags),
        (REMOVED_ARM_TAGS, &value.removed_arm_tags),
    ] {
        if !tags.is_empty() {
            write_packed_u64(sink, spec, tags)?;
        }
    }
    Ok(())
}

/// A `TransactionApplied` arm-tag list (#1585 D5): absent is empty, and a present list holds at
/// least one tag.
fn decode_arm_tags(bytes: Option<&[u8]>) -> Result<Vec<u64>, DecodeError> {
    let Some(bytes) = bytes else {
        return Ok(Vec::new());
    };
    if bytes.is_empty() || bytes.len() % 8 != 0 {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(bytes
        .as_chunks::<8>()
        .0
        .iter()
        .map(|word| u64::from_le_bytes(*word))
        .collect())
}

pub(crate) fn write_session_replace_request(
    sink: &mut dyn Sink,
    value: SessionReplaceRequest,
) -> Result<(), EncodeError> {
    if value.document_bytes == 0 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::session_replace_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::session_replace_request::DOCUMENT_BYTES,
        &value.document_bytes.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_replace_request::DOCUMENT_SHA256,
        &value.document_sha256
    )
}

pub(crate) fn write_session_committed(
    sink: &mut dyn Sink,
    value: SessionCommitted,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::session_committed::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::session_committed::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::ORIGIN_REQUEST_ID,
        &value.origin_request_id.get().to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::PREVIOUS_REVISION,
        &value.previous_revision.0.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::session_committed::APPLIED_OPERATIONS,
        &value.applied_operations.to_le_bytes()
    )
}

pub(crate) fn write_transport_set(
    sink: &mut dyn Sink,
    value: TransportSetRequest,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_set::SPEC.field_count(&[])?)?;
    write_spec!(sink, schema::transport_set::STATE, &[value.state as u8])
}

pub(crate) fn write_transport_snapshot(
    sink: &mut dyn Sink,
    value: TransportSnapshot,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_snapshot::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::transport_snapshot::STATE,
        &[value.state as u8]
    )?;
    write_spec!(
        sink,
        schema::transport_snapshot::EFFECTIVE_SAMPLE,
        &value.effective_sample.0.to_le_bytes()
    )
}

pub(crate) fn write_transport_state_event(
    sink: &mut dyn Sink,
    value: TransportStateEvent,
) -> Result<(), EncodeError> {
    sink.check_field_count(schema::transport_state_event::SPEC.field_count(&[(
        schema::transport_state_event::ORIGIN_REQUEST_ID,
        usize::from(value.origin_request_id.is_some()),
    )])?)?;
    write_spec!(
        sink,
        schema::transport_state_event::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::transport_state_event::STATE,
        &[value.state as u8]
    )?;
    write_spec!(
        sink,
        schema::transport_state_event::EFFECTIVE_SAMPLE,
        &value.effective_sample.0.to_le_bytes()
    )?;
    if let Some(origin) = value.origin_request_id {
        write_spec!(
            sink,
            schema::transport_state_event::ORIGIN_REQUEST_ID,
            &origin.get().to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_observation_ended(
    sink: &mut dyn Sink,
    value: &ObservationEnded,
) -> Result<(), EncodeError> {
    if value.keys.is_empty()
        || value.keys.iter().any(|key| {
            session::StableId::parse(&key.strip_id).is_none()
                || session::StableId::parse(&key.effect_id).is_none()
        })
        || !value
            .keys
            .windows(2)
            .all(|pair| pair[0].wire_order() < pair[1].wire_order())
    {
        return Err(EncodeError::InvalidField);
    }
    sink.check_field_count(
        schema::observation_ended::SPEC
            .field_count(&[(schema::observation_ended::KEY, value.keys.len())])?,
    )?;
    write_spec!(
        sink,
        schema::observation_ended::EVENT_SEQUENCE,
        &value.event_sequence.to_le_bytes()
    )?;
    for key in &value.keys {
        sink.nested_spec(schema::observation_ended::KEY, &mut |sink| {
            sink.message_header(schema::observation_tap_key::SPEC.field_count(&[])?)?;
            write_spec!(
                sink,
                schema::observation_tap_key::STRIP_ID,
                key.strip_id.as_bytes()
            )?;
            write_spec!(
                sink,
                schema::observation_tap_key::SECTION,
                &[key.section.code()]
            )?;
            write_spec!(
                sink,
                schema::observation_tap_key::EFFECT_ID,
                key.effect_id.as_bytes()
            )?;
            write_spec!(
                sink,
                schema::observation_tap_key::TAP_ID,
                &key.tap_id.to_le_bytes()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn write_meter_batch(
    sink: &mut dyn Sink,
    value: MeterBatch<'_>,
) -> Result<(), EncodeError> {
    if !meter_window_is_valid(
        value.first_sample,
        value.observed_sample,
        value.window_sequence,
    ) {
        return Err(EncodeError::InvalidField);
    }
    validate_meter_records(value.records)?;
    sink.check_field_count(schema::meter_batch::SPEC.field_count(&[])?)?;
    let count = u16::try_from(value.records.len()).map_err(|_| EncodeError::LimitExceeded)?;
    let length = value
        .records
        .len()
        .checked_mul(16)
        .ok_or(EncodeError::LimitExceeded)?;
    write_spec!(
        sink,
        schema::meter_batch::OBSERVED_SAMPLE,
        &value.observed_sample.0.to_le_bytes()
    )?;
    write_spec!(sink, schema::meter_batch::COUNT, &count.to_le_bytes())?;
    write_spec!(
        sink,
        schema::meter_batch::RECORD_BYTES,
        &16_u16.to_le_bytes()
    )?;
    sink.stream_field_spec(schema::meter_batch::RECORDS, length, &mut |sink| {
        for record in value.records {
            let mut bytes = [0_u8; 16];
            encode_meter_record(record, &mut bytes);
            sink.raw(&bytes)?;
        }
        Ok(())
    })?;
    write_spec!(
        sink,
        schema::meter_batch::FIRST_SAMPLE,
        &value.first_sample.0.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::meter_batch::WINDOW_SEQUENCE,
        &value.window_sequence.to_le_bytes()
    )
}

/// Write a telemetry configuration: a command when `echo` is false (no meter window), a success
/// echo when it is true.
pub(crate) fn write_telemetry_configuration(
    sink: &mut dyn Sink,
    value: &TelemetryConfiguration,
    echo: bool,
) -> Result<(), EncodeError> {
    if !meter_table_is_valid(value) || (value.meter_window_blocks.is_some() && !echo) {
        return Err(EncodeError::InvalidField);
    }
    validate_telemetry_configuration(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(schema::telemetry_configuration::SPEC.field_count(&[
        (schema::telemetry_configuration::METER, value.meters.len()),
        (
            schema::telemetry_configuration::METER_WINDOW_BLOCKS,
            usize::from(value.meter_window_blocks.is_some()),
        ),
    ])?)?;
    write_spec!(
        sink,
        schema::telemetry_configuration::METER_PERIOD_BLOCKS,
        &value.meter_period_blocks.to_le_bytes()
    )?;
    write_counter_ids(
        sink,
        schema::telemetry_configuration::COUNTER_IDS,
        &value.counter_ids,
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::COUNTER_PERIOD_BLOCKS,
        &value.counter_period_blocks.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::DIAGNOSTICS_ENABLED,
        &[u8::from(value.diagnostics_enabled)]
    )?;
    write_spec!(
        sink,
        schema::telemetry_configuration::MINIMUM_DIAGNOSTIC_SEVERITY,
        &[value.minimum_diagnostic_severity as u8]
    )?;
    for entry in &value.meters {
        sink.nested_spec(schema::telemetry_configuration::METER, &mut |sink| {
            let strip = match &entry.target {
                MeterTarget::Output | MeterTarget::Master => None,
                MeterTarget::Strip(id) => Some(id),
            };
            sink.message_header(
                schema::meter_entry::SPEC.field_count(&[(
                    schema::meter_entry::STRIP_ID,
                    usize::from(strip.is_some()),
                )])?,
            )?;
            write_spec!(sink, schema::meter_entry::TARGET, &[entry.target.code()])?;
            if let Some(id) = strip {
                write_spec!(sink, schema::meter_entry::STRIP_ID, id.as_bytes())?;
            }
            write_spec!(sink, schema::meter_entry::METRIC, &[entry.metric as u8])
        })?;
    }
    if let Some(blocks) = value.meter_window_blocks {
        write_spec!(
            sink,
            schema::telemetry_configuration::METER_WINDOW_BLOCKS,
            &blocks.get().to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_counters_request(
    sink: &mut dyn Sink,
    value: &CountersRequest,
) -> Result<(), EncodeError> {
    validate_counters_request(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(
        schema::counters_request::SPEC
            .field_count(&[(schema::counters_request::IDS, usize::from(!value.all))])?,
    )?;
    write_spec!(sink, schema::counters_request::ALL, &[u8::from(value.all)])?;
    if !value.all {
        write_u32s(sink, schema::counters_request::IDS, &value.ids)?;
    }
    Ok(())
}

pub(crate) fn write_counter_snapshot(
    sink: &mut dyn Sink,
    value: CounterSnapshotRef<'_>,
) -> Result<(), EncodeError> {
    validate_counter_snapshot_ref(value).map_err(|_| EncodeError::LimitExceeded)?;
    sink.check_field_count(
        schema::counter_snapshot::SPEC
            .field_count(&[(schema::counter_snapshot::VALUE, value.values.len())])?,
    )?;
    write_spec!(
        sink,
        schema::counter_snapshot::OBSERVED_SAMPLE,
        &value.observed_sample.0.to_le_bytes()
    )?;
    for counter in value.values {
        sink.nested_spec(schema::counter_snapshot::VALUE, &mut |sink| {
            sink.message_header(schema::counter_value::SPEC.field_count(&[])?)?;
            write_spec!(
                sink,
                schema::counter_value::ID,
                &(counter.id as u32).to_le_bytes()
            )?;
            write_spec!(
                sink,
                schema::counter_value::VALUE,
                &counter.value.to_le_bytes()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn write_diagnostics_request(
    sink: &mut dyn Sink,
    value: DiagnosticsRequest,
) -> Result<(), EncodeError> {
    if value.limit == 0 || value.limit > 256 {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::diagnostics_request::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::diagnostics_request::AFTER_SEQUENCE,
        &value.after_sequence.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::diagnostics_request::LIMIT,
        &value.limit.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::diagnostics_request::MINIMUM_SEVERITY,
        &[value.minimum_severity as u8]
    )
}

pub(crate) fn write_non_ok(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &NonOkResponse,
) -> Result<(), EncodeError> {
    check_non_ok_nesting(codec, value)?;
    let count = schema::non_ok::SPEC.field_count(&[
        (schema::non_ok::DIAGNOSTIC, value.diagnostics.len()),
        (
            schema::non_ok::BACKPRESSURE,
            usize::from(value.backpressure.is_some()),
        ),
    ])?;
    sink.check_field_count(count)?;
    for diagnostic in &value.diagnostics {
        write_diagnostic_field(codec, sink, schema::non_ok::DIAGNOSTIC, diagnostic)?;
    }
    write_spec!(
        sink,
        schema::non_ok::OMITTED_DIAGNOSTICS,
        &value.omitted_diagnostics.to_le_bytes()
    )?;
    if let Some(backpressure) = value.backpressure {
        write_backpressure_field(sink, schema::non_ok::BACKPRESSURE, backpressure)?;
    }
    Ok(())
}

/// Which kind of row an entry is: the codes of `control_entry` field 2.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum ControlRowKind {
    OwnerHeader = 1,
    SectionHeader = 2,
    Control = 3,
}

impl ControlRowKind {
    const fn from_code(code: u8) -> Result<Self, DecodeError> {
        match code {
            1 => Ok(Self::OwnerHeader),
            2 => Ok(Self::SectionHeader),
            3 => Ok(Self::Control),
            _ => Err(DecodeError::InvalidField),
        }
    }
}

impl ControlBody {
    const fn row_kind(&self) -> ControlRowKind {
        match self {
            Self::OwnerHeader(_) => ControlRowKind::OwnerHeader,
            Self::SectionHeader(_) => ControlRowKind::SectionHeader,
            Self::Control(_) => ControlRowKind::Control,
        }
    }
}

const fn has_slot(section: ControlSection) -> bool {
    matches!(section, ControlSection::Console | ControlSection::Inserts)
}

fn text_fits(limits: crate::ProtocolLimits, text: &str) -> bool {
    text.len() <= limits.max_string_bytes
}

fn stable_id_fits(limits: crate::ProtocolLimits, text: &str) -> bool {
    text_fits(limits, text) && valid_stable_id(text)
}

/// A request's own rules, shared by both directions: `limit` is `1..=256`, a cursor names the
/// structure it belongs to (epoch `0` means "the current one" and has no handles to continue
/// from, so it holds `after_handle` `0`), and the owner ID is present exactly for a kind other than
/// the session.
fn control_request_is_valid(limits: crate::ProtocolLimits, value: &ControlListRequest) -> bool {
    (1..=CONTROL_PAGE_ITEMS).contains(&value.limit)
        && (value.structure_epoch != 0 || value.after_handle == 0)
        && value.owner.as_ref().is_none_or(|owner| {
            (owner.kind == ControlOwnerKind::Session) == owner.id.is_none()
                && owner
                    .id
                    .as_deref()
                    .is_none_or(|id| stable_id_fits(limits, id))
        })
}

/// An address's own rules, shared by both directions: the owner ID is absent exactly for the
/// session, the slot ID is present exactly for the console and inserts sections, and the control
/// and lane are present exactly in a control row.
fn control_address_is_valid(
    limits: crate::ProtocolLimits,
    address: &ControlAddress,
    row: ControlRowKind,
) -> bool {
    (address.owner_kind == ControlOwnerKind::Session) == address.owner_id.is_none()
        && address
            .owner_id
            .as_deref()
            .is_none_or(|id| stable_id_fits(limits, id))
        && has_slot(address.section) == address.slot_id.is_some()
        && address
            .slot_id
            .as_deref()
            .is_none_or(|id| stable_id_fits(limits, id))
        && (row == ControlRowKind::Control) == address.control.is_some()
        && (row == ControlRowKind::Control) == address.lane.is_some()
        && (row != ControlRowKind::OwnerHeader || address.section == ControlSection::Own)
}

fn owner_header_matches(header: ControlOwnerHeaderKind, owner: ControlOwnerKind) -> bool {
    matches!(
        (header, owner),
        (
            ControlOwnerHeaderKind::Track | ControlOwnerHeaderKind::Submix,
            ControlOwnerKind::Strip
        ) | (ControlOwnerHeaderKind::Route, ControlOwnerKind::Route)
            | (ControlOwnerHeaderKind::Vca, ControlOwnerKind::Vca)
            | (ControlOwnerHeaderKind::Session, ControlOwnerKind::Session)
    )
}

/// An entry's own rules, shared by both directions. The wire has no spelling for a header row
/// with a value or a control row without one, so the typed body already holds that; what remains
/// is the agreement between the address, the header kind and the row.
fn control_entry_is_valid(limits: crate::ProtocolLimits, entry: &ControlEntry) -> bool {
    if entry.handle == 0 || !control_address_is_valid(limits, &entry.address, entry.body.row_kind())
    {
        return false;
    }
    match &entry.body {
        ControlBody::OwnerHeader(header) => {
            owner_header_matches(header.kind, entry.address.owner_kind)
                && (header.kind == ControlOwnerHeaderKind::Route) == header.route.is_some()
                && text_fits(limits, &header.name)
                && header.route.as_ref().is_none_or(|route| {
                    text_fits(limits, &route.source) && text_fits(limits, &route.destination)
                })
        }
        ControlBody::SectionHeader(header) => {
            text_fits(limits, &header.name)
                && header.effect.as_deref().is_none_or(|effect| {
                    has_slot(entry.address.section) && text_fits(limits, effect)
                })
        }
        ControlBody::Control(row) => control_row_is_valid(limits, row),
    }
}

/// A control row's own rules, shared by both directions: finite numbers, an ordered range,
/// choices exactly for an enumeration and holding its default and value, and the class, reason,
/// issue and reopen condition present exactly together. A control is automatable only if it is a
/// live model value, and automated only if it is automatable.
fn control_row_is_valid(limits: crate::ProtocolLimits, row: &ControlRow) -> bool {
    let finite = row.minimum.is_finite()
        && row.maximum.is_finite()
        && row.default.is_finite()
        && row.step.is_finite()
        && row.value.is_finite()
        && row.disabled_value.is_none_or(f32::is_finite)
        && row.ladder.iter().all(|step| step.is_finite())
        && row.choices.iter().all(|choice| choice.value.is_finite());
    if !finite
        || row.minimum > row.maximum
        || row.name.is_empty()
        || !text_fits(limits, &row.name)
        || !text_fits(limits, &row.display_name)
        || !text_fits(limits, &row.display_unit)
        || row
            .choices
            .iter()
            .any(|choice| !text_fits(limits, &choice.label))
        || row
            .reopen
            .as_deref()
            .is_some_and(|text| !text_fits(limits, text))
    {
        return false;
    }
    let choice_values = row.choices.iter().map(|choice| choice.value);
    let unique = row.choices.iter().enumerate().all(|(index, choice)| {
        row.choices[..index]
            .iter()
            .all(|earlier| earlier.value != choice.value)
    });
    let choices_fit = match row.kind {
        ControlKind::Enumeration => {
            !row.choices.is_empty()
                && unique
                && choice_values.clone().any(|value| value == row.default)
                && choice_values.clone().any(|value| value == row.value)
        }
        ControlKind::Continuous | ControlKind::Toggle => row.choices.is_empty(),
    };
    let rebuild_only = row.class == ControlClass::RebuildOnly;
    choices_fit
        && rebuild_only == row.reason.is_some()
        && rebuild_only == row.reopen.is_some()
        && (row.reason == Some(ControlReason::RuledLiveNotBuilt)) == row.reason_issue.is_some()
        && (!row.automatable
            || (row.class == ControlClass::Live
                && row.scope == ControlScope::Model
                && row.family != ControlFamily::Setting))
        && (!row.automated || row.automatable)
}

fn control_list_page_is_valid(limits: crate::ProtocolLimits, page: &ControlListPage) -> bool {
    page.entries.len() <= usize::from(CONTROL_PAGE_ITEMS)
        && page
            .entries
            .windows(2)
            .all(|pair| pair[0].handle < pair[1].handle)
        && page
            .entries
            .last()
            .is_none_or(|last| last.handle == page.last_handle)
        && page
            .entries
            .iter()
            .all(|entry| control_entry_is_valid(limits, entry))
}

fn control_state_page_is_valid(page: &ControlStatePage) -> bool {
    page.records.len() <= usize::from(CONTROL_PAGE_ITEMS)
        && page.records.first().is_none_or(|first| first.handle != 0)
        && page
            .records
            .windows(2)
            .all(|pair| pair[0].handle < pair[1].handle)
        && page
            .records
            .last()
            .is_none_or(|last| last.handle <= page.last_handle)
        && page.records.iter().all(|record| record.value.is_finite())
}

pub(crate) fn write_control_request(
    sink: &mut dyn Sink,
    value: &ControlListRequest,
) -> Result<(), EncodeError> {
    if !control_request_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let count = control_list_request::SPEC.field_count(&[
        (
            control_list_request::OWNER_KIND,
            usize::from(value.owner.is_some()),
        ),
        (
            control_list_request::OWNER_ID,
            usize::from(value.owner.as_ref().is_some_and(|owner| owner.id.is_some())),
        ),
        (
            control_list_request::HEADERS_ONLY,
            usize::from(value.headers_only),
        ),
    ])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        control_list_request::STRUCTURE_EPOCH,
        &value.structure_epoch.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_list_request::AFTER_HANDLE,
        &value.after_handle.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_list_request::LIMIT,
        &value.limit.to_le_bytes()
    )?;
    if let Some(owner) = &value.owner {
        write_spec!(sink, control_list_request::OWNER_KIND, &[owner.kind.code()])?;
        if let Some(id) = &owner.id {
            write_spec!(sink, control_list_request::OWNER_ID, id.as_bytes())?;
        }
    }
    if value.headers_only {
        write_spec!(sink, control_list_request::HEADERS_ONLY, &[1])?;
    }
    Ok(())
}

fn decode_control_request(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ControlListRequest, DecodeError> {
    let message = message.schema_spec(&control_list_request::SPEC)?;
    let kind = optional_spec!(message, control_list_request::OWNER_KIND)?
        .map(|code| ControlOwnerKind::from_code(read_u8(code)?))
        .transpose()?;
    let id = optional_spec!(message, control_list_request::OWNER_ID)?
        .map(|id| read_string(codec, id).map(str::to_owned))
        .transpose()?;
    // The flag is written only when set, so a present `false` is a second spelling of absence and
    // is refused.
    let headers_only = match optional_spec!(message, control_list_request::HEADERS_ONLY)?
        .map(read_bool)
        .transpose()?
    {
        None => false,
        Some(true) => true,
        Some(false) => return Err(DecodeError::InvalidField),
    };
    let value = ControlListRequest {
        structure_epoch: read_u64(one_spec!(message, control_list_request::STRUCTURE_EPOCH)?)?,
        after_handle: read_u32(one_spec!(message, control_list_request::AFTER_HANDLE)?)?,
        limit: read_u16(one_spec!(message, control_list_request::LIMIT)?)?,
        // An owner ID with no kind is refused: the filter is built from the kind.
        owner: match (kind, id) {
            (Some(kind), id) => Some(ControlOwnerFilter { kind, id }),
            (None, None) => None,
            (None, Some(_)) => return Err(DecodeError::InvalidField),
        },
        headers_only,
    };
    if !control_request_is_valid(codec.limits(), &value) {
        return Err(DecodeError::InvalidField);
    }
    Ok(value)
}

pub(crate) fn write_control_list_page(
    sink: &mut dyn Sink,
    value: &ControlListPage,
) -> Result<(), EncodeError> {
    // More rows than a page holds is the one excess a smaller page cures; every other failure is
    // the value's own and no page size changes it.
    if value.entries.len() > usize::from(CONTROL_PAGE_ITEMS) {
        return Err(EncodeError::LimitExceeded);
    }
    if !control_list_page_is_valid(sink.limits(), value) {
        return Err(EncodeError::InvalidField);
    }
    let count =
        control_list_page::SPEC.field_count(&[(control_list_page::ENTRY, value.entries.len())])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        control_list_page::REVISION,
        &value.revision.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_list_page::STRUCTURE_EPOCH,
        &value.structure_epoch.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_list_page::LAST_HANDLE,
        &value.last_handle.to_le_bytes()
    )?;
    write_spec!(sink, control_list_page::EOF, &[u8::from(value.eof)])?;
    for entry in &value.entries {
        write_control_entry(sink, entry)?;
    }
    Ok(())
}

fn write_control_address(sink: &mut dyn Sink, address: &ControlAddress) -> Result<(), EncodeError> {
    sink.nested_spec(control_entry::ADDRESS, &mut |sink| {
        write_control_address_fields(sink, address)
    })
}

/// The fields of one list-style address message: the control list's rows and a response target
/// write it. The caller has validated the address.
fn write_control_address_fields(
    sink: &mut dyn Sink,
    address: &ControlAddress,
) -> Result<(), EncodeError> {
    write_address_fields(sink, &control_address::SPEC, address, None)
}

/// The one encoder of the address fields: `spec` is the list's address message, or the set-control
/// address message with `expected_effect` as its seventh field. The caller has validated the
/// address.
fn write_address_fields(
    sink: &mut dyn Sink,
    spec: &'static schema::MessageSpec,
    address: &ControlAddress,
    expected_effect: Option<&str>,
) -> Result<(), EncodeError> {
    let mut variable = [(control_address::OWNER_ID, 0); 5];
    variable[0] = (
        control_address::OWNER_ID,
        usize::from(address.owner_id.is_some()),
    );
    variable[1] = (
        control_address::SLOT_ID,
        usize::from(address.slot_id.is_some()),
    );
    variable[2] = (
        control_address::CONTROL,
        usize::from(address.control.is_some()),
    );
    variable[3] = (control_address::LANE, usize::from(address.lane.is_some()));
    let mut count = 4;
    if let Some(expected) = expected_effect {
        debug_assert!(!expected.is_empty());
        variable[count] = (set_control_address::EXPECTED_EFFECT, 1);
        count += 1;
    }
    let fields = spec.field_count(&variable[..count])?;
    sink.message_header(fields)?;
    write_spec!(
        sink,
        control_address::OWNER_KIND,
        &[address.owner_kind.code()]
    )?;
    if let Some(id) = &address.owner_id {
        write_spec!(sink, control_address::OWNER_ID, id.as_bytes())?;
    }
    write_spec!(sink, control_address::SECTION, &[address.section.code()])?;
    if let Some(id) = &address.slot_id {
        write_spec!(sink, control_address::SLOT_ID, id.as_bytes())?;
    }
    if let Some(control) = address.control {
        write_spec!(sink, control_address::CONTROL, &control.to_le_bytes())?;
    }
    if let Some(lane) = address.lane {
        write_spec!(
            sink,
            control_address::LANE,
            &[schema::parameter_channel_wire(lane)]
        )?;
    }
    if let Some(expected) = expected_effect {
        write_spec!(
            sink,
            set_control_address::EXPECTED_EFFECT,
            expected.as_bytes()
        )?;
    }
    Ok(())
}

/// The address message of a stored automation target: the control list's one address encoding with
/// `control` and `lane` present, and no expected effect (the target is a row, not an edit). An
/// address that breaks the list's rules is `InvalidField`.
pub(crate) fn write_automation_target_address(
    sink: &mut dyn Sink,
    address: &controls::ControlAddress,
) -> Result<(), EncodeError> {
    let address = ControlAddress::from(address);
    if !control_address_is_valid(sink.limits(), &address, ControlRowKind::Control) {
        return Err(EncodeError::InvalidField);
    }
    write_control_address_fields(sink, &address)
}

/// A stored automation target, decoded by the control list's one address decoder and held to the
/// list's rules for a control row: every refusal is `InvalidField`, an unallocated owner, section
/// or lane code included. A target holds no expected effect in any section: the `SET_CONTROL`
/// address's seventh field is refused here as `SET_CONTROL` refuses it outside an effect section
/// (`InvalidField` in its optional form, `InvalidTlv` in its mandatory form), never skipped as an
/// unknown optional field.
pub(crate) fn decode_automation_target_address(
    message: Message<'_>,
) -> Result<controls::ControlAddress, DecodeError> {
    let limits = message.limits();
    let message = message.schema_spec(&set_control_address::SPEC)?;
    if optional_spec!(message, set_control_address::EXPECTED_EFFECT)?.is_some() {
        return Err(DecodeError::InvalidField);
    }
    let address = decode_control_address_fields(limits, &message)?;
    if !control_address_is_valid(limits, &address, ControlRowKind::Control) {
        return Err(DecodeError::InvalidField);
    }
    controls::ControlAddress::try_from(&address)
}

/// The address message of a `SET_CONTROL` edit (the body of its `address` field): the control
/// list's one address encoding, with `control` and `lane` present, and the native effect the edit
/// was written for as an optional seventh field (#1678). An address that breaks the list's rules,
/// or an expected effect on a section that runs no effect, is `InvalidField`.
pub(crate) fn write_set_control_edit_address(
    sink: &mut dyn Sink,
    address: &controls::ControlAddress,
    expected_effect: Option<&session::StableId>,
) -> Result<(), EncodeError> {
    let address = ControlAddress::from(address);
    if !control_address_is_valid(sink.limits(), &address, ControlRowKind::Control)
        || expected_effect.is_some_and(|effect| {
            !has_slot(address.section) || !stable_id_fits(sink.limits(), effect.as_str())
        })
    {
        return Err(EncodeError::InvalidField);
    }
    write_address_fields(
        sink,
        &set_control_address::SPEC,
        &address,
        expected_effect.map(session::StableId::as_str),
    )
}

/// The address message of a `SET_CONTROL` edit, decoded by the control list's one address decoder
/// and held to the list's rules for a control row: every refusal is `InvalidField`, an
/// unallocated owner, section or lane code included. The second value is the native effect the edit
/// was written for (#1678); it is refused unless the section is an `inserts` or `console` one.
pub(crate) fn decode_set_control_edit_address(
    message: Message<'_>,
) -> Result<(controls::ControlAddress, Option<session::StableId>), DecodeError> {
    let limits = message.limits();
    let message = message.schema_spec(&set_control_address::SPEC)?;
    let address = decode_control_address_fields(limits, &message)?;
    if !control_address_is_valid(limits, &address, ControlRowKind::Control) {
        return Err(DecodeError::InvalidField);
    }
    let expected = optional_spec!(message, set_control_address::EXPECTED_EFFECT)?
        .map(|value| {
            let text = read_string_within(limits, value)?;
            if !has_slot(address.section) {
                return Err(DecodeError::InvalidField);
            }
            session::StableId::parse(text).ok_or(DecodeError::InvalidField)
        })
        .transpose()?;
    Ok((controls::ControlAddress::try_from(&address)?, expected))
}

fn write_control_entry(sink: &mut dyn Sink, entry: &ControlEntry) -> Result<(), EncodeError> {
    use control_entry as e;
    let (header, route, effect, row) = match &entry.body {
        ControlBody::OwnerHeader(header) => (true, header.route.is_some(), false, None),
        ControlBody::SectionHeader(header) => (true, false, header.effect.is_some(), None),
        ControlBody::Control(row) => (false, false, false, Some(row)),
    };
    let one = |present: bool| usize::from(present);
    let control = row.is_some();
    let fields = e::SPEC.field_count(&[
        (e::HEADER_KIND, one(header)),
        (e::HEADER_NAME, one(header)),
        (e::EFFECT, one(effect)),
        (e::SOURCE, one(route)),
        (e::DESTINATION, one(route)),
        (e::TAP, one(route)),
        (e::NAME, one(control)),
        (e::DISPLAY_NAME, one(control)),
        (e::DISPLAY_UNIT, one(control)),
        (e::UNIT, one(control)),
        (e::KIND, one(control)),
        (e::MINIMUM, one(control)),
        (e::MAXIMUM, one(control)),
        (
            e::DISABLED_VALUE,
            one(row.is_some_and(|row| row.disabled_value.is_some())),
        ),
        (e::DEFAULT, one(control)),
        (e::TAPER, one(control)),
        (e::STEP, one(control)),
        (e::PRECISION, one(control)),
        (e::LADDER, one(control)),
        (e::CHOICE, row.map_or(0, |row| row.choices.len())),
        (
            e::RAMP_ROW,
            one(row.is_some_and(|row| row.ramp_row.is_some())),
        ),
        (e::DEFAULT_RAMP_SAMPLES, one(control)),
        (e::CLASS, one(control)),
        (e::REASON, one(row.is_some_and(|row| row.reason.is_some()))),
        (
            e::REASON_ISSUE,
            one(row.is_some_and(|row| row.reason_issue.is_some())),
        ),
        (e::REOPEN, one(row.is_some_and(|row| row.reopen.is_some()))),
        (e::SCOPE, one(control)),
        (e::AUTOMATABLE, one(control)),
        (e::FAMILY, one(control)),
        (e::ACTIVE, one(control)),
        (e::VALUE, one(control)),
        (e::AUTOMATED, one(control)),
    ])?;
    sink.nested_spec(control_list_page::ENTRY, &mut |sink| {
        sink.message_header(fields)?;
        write_spec!(sink, e::HANDLE, &entry.handle.to_le_bytes())?;
        write_spec!(sink, e::ROW_KIND, &[entry.body.row_kind() as u8])?;
        write_control_address(sink, &entry.address)?;
        match &entry.body {
            ControlBody::OwnerHeader(header) => {
                write_spec!(sink, e::HEADER_KIND, &[header.kind.code()])?;
                write_spec!(sink, e::HEADER_NAME, header.name.as_bytes())?;
                if let Some(route) = &header.route {
                    write_spec!(sink, e::SOURCE, route.source.as_bytes())?;
                    write_spec!(sink, e::DESTINATION, route.destination.as_bytes())?;
                    write_spec!(sink, e::TAP, &[crate::session_wire::enum_tap(route.tap)])?;
                }
            }
            ControlBody::SectionHeader(header) => {
                write_spec!(sink, e::HEADER_KIND, &[entry.address.section.code()])?;
                write_spec!(sink, e::HEADER_NAME, header.name.as_bytes())?;
                if let Some(effect) = &header.effect {
                    write_spec!(sink, e::EFFECT, effect.as_bytes())?;
                }
            }
            ControlBody::Control(row) => write_control_row(sink, row)?,
        }
        Ok(())
    })
}

fn write_control_row(sink: &mut dyn Sink, row: &ControlRow) -> Result<(), EncodeError> {
    use control_entry as e;
    write_spec!(sink, e::NAME, row.name.as_bytes())?;
    write_spec!(sink, e::DISPLAY_NAME, row.display_name.as_bytes())?;
    write_spec!(sink, e::DISPLAY_UNIT, row.display_unit.as_bytes())?;
    write_spec!(sink, e::UNIT, &[schema::parameter_unit_wire(row.unit)])?;
    write_spec!(sink, e::KIND, &[row.kind.code()])?;
    write_spec!(sink, e::MINIMUM, &row.minimum.to_le_bytes())?;
    write_spec!(sink, e::MAXIMUM, &row.maximum.to_le_bytes())?;
    if let Some(disabled) = row.disabled_value {
        write_spec!(sink, e::DISABLED_VALUE, &disabled.to_le_bytes())?;
    }
    write_spec!(sink, e::DEFAULT, &row.default.to_le_bytes())?;
    write_spec!(sink, e::TAPER, &[row.taper.code()])?;
    write_spec!(sink, e::STEP, &row.step.to_le_bytes())?;
    write_spec!(sink, e::PRECISION, &[row.precision])?;
    let ladder_bytes = row
        .ladder
        .len()
        .checked_mul(4)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(e::LADDER, ladder_bytes, &mut |sink| {
        for step in &row.ladder {
            sink.raw(&step.to_le_bytes())?;
        }
        Ok(())
    })?;
    for choice in &row.choices {
        write_enum_choice(sink, choice)?;
    }
    if let Some(ramp) = row.ramp_row {
        write_spec!(sink, e::RAMP_ROW, &[ramp.code()])?;
    }
    write_spec!(
        sink,
        e::DEFAULT_RAMP_SAMPLES,
        &row.default_ramp_samples.to_le_bytes()
    )?;
    write_spec!(sink, e::CLASS, &[row.class.code()])?;
    if let Some(reason) = row.reason {
        write_spec!(sink, e::REASON, &[reason.code()])?;
    }
    if let Some(issue) = row.reason_issue {
        write_spec!(sink, e::REASON_ISSUE, &issue.to_le_bytes())?;
    }
    if let Some(reopen) = &row.reopen {
        write_spec!(sink, e::REOPEN, reopen.as_bytes())?;
    }
    write_spec!(sink, e::SCOPE, &[row.scope.code()])?;
    write_spec!(sink, e::AUTOMATABLE, &[u8::from(row.automatable)])?;
    write_spec!(sink, e::FAMILY, &[row.family.code()])?;
    write_spec!(sink, e::ACTIVE, &[u8::from(row.active)])?;
    write_spec!(sink, e::VALUE, &row.value.to_le_bytes())?;
    write_spec!(sink, e::AUTOMATED, &[u8::from(row.automated)])
}

fn write_enum_choice(sink: &mut dyn Sink, choice: &EnumChoice) -> Result<(), EncodeError> {
    let fields = enum_choice::SPEC.field_count(&[])?;
    sink.nested_spec(control_entry::CHOICE, &mut |sink| {
        sink.message_header(fields)?;
        write_spec!(sink, enum_choice::VALUE, &choice.value.to_le_bytes())?;
        write_spec!(sink, enum_choice::LABEL, choice.label.as_bytes())
    })
}

fn decode_control_list_page(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ControlListPage, DecodeError> {
    let message = message.schema_spec(&control_list_page::SPEC)?;
    let entries = values_spec!(message, control_list_page::ENTRY)?
        .map(|entry| {
            decode_control_entry(codec, Message::nested_at_depth(entry, codec.limits(), 1)?)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let page = ControlListPage {
        revision: read_u64(one_spec!(message, control_list_page::REVISION)?)?,
        structure_epoch: read_u64(one_spec!(message, control_list_page::STRUCTURE_EPOCH)?)?,
        last_handle: read_u32(one_spec!(message, control_list_page::LAST_HANDLE)?)?,
        eof: read_bool(one_spec!(message, control_list_page::EOF)?)?,
        entries,
    };
    if !control_list_page_is_valid(codec.limits(), &page) {
        return Err(DecodeError::InvalidField);
    }
    Ok(page)
}

fn decode_control_address(
    limits: crate::ProtocolLimits,
    message: Message<'_>,
) -> Result<ControlAddress, DecodeError> {
    let message = message.schema_spec(&control_address::SPEC)?;
    decode_control_address_fields(limits, &message)
}

/// The fields of an address message whose schema is already selected: the list's address or the
/// set-control address, which share fields 1 to 6.
fn decode_control_address_fields(
    limits: crate::ProtocolLimits,
    message: &Message<'_>,
) -> Result<ControlAddress, DecodeError> {
    let text = |spec: schema::FieldSpec| {
        optional_spec!(message, spec)?
            .map(|value| read_string_within(limits, value).map(str::to_owned))
            .transpose()
    };
    Ok(ControlAddress {
        owner_kind: ControlOwnerKind::from_code(read_u8(one_spec!(
            message,
            control_address::OWNER_KIND
        )?)?)?,
        owner_id: text(control_address::OWNER_ID)?,
        section: ControlSection::from_code(read_u8(one_spec!(
            message,
            control_address::SECTION
        )?)?)?,
        slot_id: text(control_address::SLOT_ID)?,
        control: optional_spec!(message, control_address::CONTROL)?
            .map(read_u32)
            .transpose()?,
        lane: optional_spec!(message, control_address::LANE)?
            .map(|code| {
                schema::parameter_channel_from_wire(read_u8(code)?)
                    .map_err(|_| DecodeError::InvalidField)
            })
            .transpose()?,
    })
}

fn decode_control_entry(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ControlEntry, DecodeError> {
    use control_entry as e;
    let limits = codec.limits();
    let message = message.schema_spec(&e::SPEC)?;
    let handle = read_u32(one_spec!(message, e::HANDLE)?)?;
    let row_kind = ControlRowKind::from_code(read_u8(one_spec!(message, e::ROW_KIND)?)?)?;
    let address = decode_control_address(
        limits,
        Message::nested_at_depth(one_spec!(message, e::ADDRESS)?, limits, 2)?,
    )?;
    // A field outside the row's kind is refused, never skipped.
    let absent = |specs: &[schema::FieldSpec]| -> Result<(), DecodeError> {
        for spec in specs {
            if values_spec!(message, *spec)?.next().is_some() {
                return Err(DecodeError::InvalidField);
            }
        }
        Ok(())
    };
    let required = |spec: schema::FieldSpec| -> Result<&[u8], DecodeError> {
        optional_spec!(message, spec)?.ok_or(DecodeError::InvalidField)
    };
    let text = |spec: schema::FieldSpec| -> Result<String, DecodeError> {
        read_string(codec, required(spec)?).map(str::to_owned)
    };
    let optional_text = |spec: schema::FieldSpec| -> Result<Option<String>, DecodeError> {
        optional_spec!(message, spec)?
            .map(|value| read_string(codec, value).map(str::to_owned))
            .transpose()
    };
    let control_only = [
        e::NAME,
        e::DISPLAY_NAME,
        e::DISPLAY_UNIT,
        e::UNIT,
        e::KIND,
        e::MINIMUM,
        e::MAXIMUM,
        e::DISABLED_VALUE,
        e::DEFAULT,
        e::TAPER,
        e::STEP,
        e::PRECISION,
        e::LADDER,
        e::CHOICE,
        e::RAMP_ROW,
        e::DEFAULT_RAMP_SAMPLES,
        e::CLASS,
        e::REASON,
        e::REASON_ISSUE,
        e::REOPEN,
        e::SCOPE,
        e::AUTOMATABLE,
        e::FAMILY,
        e::ACTIVE,
        e::VALUE,
        e::AUTOMATED,
    ];
    let body = match row_kind {
        ControlRowKind::OwnerHeader => {
            absent(&control_only)?;
            absent(&[e::EFFECT])?;
            let kind = ControlOwnerHeaderKind::from_code(read_u8(required(e::HEADER_KIND)?)?)?;
            let route = if kind == ControlOwnerHeaderKind::Route {
                Some(ControlRouteEnds {
                    source: text(e::SOURCE)?,
                    destination: text(e::DESTINATION)?,
                    tap: crate::session_wire::parse_tap(read_u8(required(e::TAP)?)?)
                        .map_err(|_| DecodeError::InvalidField)?,
                })
            } else {
                absent(&[e::SOURCE, e::DESTINATION, e::TAP])?;
                None
            };
            ControlBody::OwnerHeader(ControlOwnerHeader {
                kind,
                name: text(e::HEADER_NAME)?,
                route,
            })
        }
        ControlRowKind::SectionHeader => {
            absent(&control_only)?;
            absent(&[e::SOURCE, e::DESTINATION, e::TAP])?;
            // A section header repeats its section in the header kind; the two must agree.
            let kind = ControlSection::from_code(read_u8(required(e::HEADER_KIND)?)?)?;
            if kind != address.section {
                return Err(DecodeError::InvalidField);
            }
            ControlBody::SectionHeader(ControlSectionHeader {
                name: text(e::HEADER_NAME)?,
                effect: optional_text(e::EFFECT)?,
            })
        }
        ControlRowKind::Control => {
            absent(&[
                e::HEADER_KIND,
                e::HEADER_NAME,
                e::EFFECT,
                e::SOURCE,
                e::DESTINATION,
                e::TAP,
            ])?;
            let float =
                |spec: schema::FieldSpec| -> Result<f32, DecodeError> { read_f32(required(spec)?) };
            let ladder = required(e::LADDER)?
                .as_chunks::<4>()
                .0
                .iter()
                .map(|word| read_f32(word))
                .collect::<Result<Vec<_>, _>>()?;
            let choices = values_spec!(message, e::CHOICE)?
                .map(|choice| decode_choice(Message::nested_at_depth(choice, limits, 2)?))
                .collect::<Result<Vec<_>, _>>()?;
            ControlBody::Control(ControlRow {
                name: text(e::NAME)?,
                display_name: text(e::DISPLAY_NAME)?,
                display_unit: text(e::DISPLAY_UNIT)?,
                unit: schema::parameter_unit_from_wire(read_u8(required(e::UNIT)?)?)
                    .map_err(|_| DecodeError::InvalidField)?,
                kind: ControlKind::from_code(read_u8(required(e::KIND)?)?)?,
                minimum: float(e::MINIMUM)?,
                maximum: float(e::MAXIMUM)?,
                disabled_value: optional_spec!(message, e::DISABLED_VALUE)?
                    .map(read_f32)
                    .transpose()?,
                default: float(e::DEFAULT)?,
                taper: ControlTaper::from_code(read_u8(required(e::TAPER)?)?)?,
                step: float(e::STEP)?,
                precision: read_u8(required(e::PRECISION)?)?,
                ladder,
                choices,
                ramp_row: optional_spec!(message, e::RAMP_ROW)?
                    .map(|code| ControlRampRow::from_code(read_u8(code)?))
                    .transpose()?,
                default_ramp_samples: read_u32(required(e::DEFAULT_RAMP_SAMPLES)?)?,
                class: ControlClass::from_code(read_u8(required(e::CLASS)?)?)?,
                reason: optional_spec!(message, e::REASON)?
                    .map(|code| ControlReason::from_code(read_u8(code)?))
                    .transpose()?,
                reason_issue: optional_spec!(message, e::REASON_ISSUE)?
                    .map(read_u32)
                    .transpose()?,
                reopen: optional_text(e::REOPEN)?,
                scope: ControlScope::from_code(read_u8(required(e::SCOPE)?)?)?,
                automatable: read_bool(required(e::AUTOMATABLE)?)?,
                family: ControlFamily::from_code(read_u8(required(e::FAMILY)?)?)?,
                active: read_bool(required(e::ACTIVE)?)?,
                value: float(e::VALUE)?,
                automated: read_bool(required(e::AUTOMATED)?)?,
            })
        }
    };
    let entry = ControlEntry {
        handle,
        address,
        body,
    };
    if !control_entry_is_valid(limits, &entry) {
        return Err(DecodeError::InvalidField);
    }
    Ok(entry)
}

fn decode_choice(message: Message<'_>) -> Result<EnumChoice, DecodeError> {
    let message = message.schema_spec(&enum_choice::SPEC)?;
    Ok(EnumChoice {
        value: read_f32(one_spec!(message, enum_choice::VALUE)?)?,
        label: core::str::from_utf8(one_spec!(message, enum_choice::LABEL)?)
            .map_err(|_| DecodeError::InvalidUtf8)?
            .to_owned(),
    })
}

pub(crate) fn write_control_state_page(
    sink: &mut dyn Sink,
    value: &ControlStatePage,
) -> Result<(), EncodeError> {
    if value.records.len() > usize::from(CONTROL_PAGE_ITEMS) {
        return Err(EncodeError::LimitExceeded);
    }
    if !control_state_page_is_valid(value) {
        return Err(EncodeError::InvalidField);
    }
    let bytes = value
        .records
        .len()
        .checked_mul(usize::from(CONTROL_STATE_RECORD_BYTES))
        .ok_or(EncodeError::LimitExceeded)?;
    sink.check_field_count(control_state_page::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        control_state_page::REVISION,
        &value.revision.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_state_page::STRUCTURE_EPOCH,
        &value.structure_epoch.to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_state_page::LAST_HANDLE,
        &value.last_handle.to_le_bytes()
    )?;
    write_spec!(sink, control_state_page::EOF, &[u8::from(value.eof)])?;
    write_spec!(
        sink,
        control_state_page::COUNT,
        &u16::try_from(value.records.len())
            .map_err(|_| EncodeError::LimitExceeded)?
            .to_le_bytes()
    )?;
    write_spec!(
        sink,
        control_state_page::RECORD_BYTES,
        &CONTROL_STATE_RECORD_BYTES.to_le_bytes()
    )?;
    sink.stream_field_spec(control_state_page::RECORDS, bytes, &mut |sink| {
        for record in &value.records {
            sink.raw(&record.handle.to_le_bytes())?;
            sink.raw(&record.value.to_le_bytes())?;
        }
        Ok(())
    })
}

fn decode_control_state_page(message: Message<'_>) -> Result<ControlStatePage, DecodeError> {
    let message = message.schema_spec(&control_state_page::SPEC)?;
    let count = usize::from(read_u16(one_spec!(message, control_state_page::COUNT)?)?);
    if count > usize::from(CONTROL_PAGE_ITEMS)
        || read_u16(one_spec!(message, control_state_page::RECORD_BYTES)?)?
            != CONTROL_STATE_RECORD_BYTES
    {
        return Err(DecodeError::InvalidField);
    }
    let bytes = one_spec!(message, control_state_page::RECORDS)?;
    if bytes.len() != count * usize::from(CONTROL_STATE_RECORD_BYTES) {
        return Err(DecodeError::InvalidValueLength);
    }
    let records = bytes
        .chunks_exact(usize::from(CONTROL_STATE_RECORD_BYTES))
        .map(|raw| {
            Ok(ControlStateRecord {
                handle: read_u32(&raw[..4])?,
                value: read_f32(&raw[4..])?,
            })
        })
        .collect::<Result<Vec<_>, DecodeError>>()?;
    let page = ControlStatePage {
        revision: read_u64(one_spec!(message, control_state_page::REVISION)?)?,
        structure_epoch: read_u64(one_spec!(message, control_state_page::STRUCTURE_EPOCH)?)?,
        last_handle: read_u32(one_spec!(message, control_state_page::LAST_HANDLE)?)?,
        eof: read_bool(one_spec!(message, control_state_page::EOF)?)?,
        records,
    };
    if !control_state_page_is_valid(&page) {
        return Err(DecodeError::InvalidField);
    }
    Ok(page)
}

pub(crate) fn write_capabilities(
    sink: &mut dyn Sink,
    value: &Capabilities<'_>,
) -> Result<(), EncodeError> {
    check_capabilities(value)?;
    sink.check_field_count(schema::capabilities::SPEC.field_count(&[])?)?;
    write_spec!(
        sink,
        schema::capabilities::MINIMUM_VERSION_MAJOR,
        &value.minimum_version.major.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MINIMUM_VERSION_MINOR,
        &value.minimum_version.minor.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_VERSION_MAJOR,
        &value.maximum_version.major.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_VERSION_MINOR,
        &value.maximum_version.minor.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_FRAME_BYTES,
        &value.maximum_frame_bytes.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TLVS,
        &value.maximum_tlvs.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_STRING_BYTES,
        &value.maximum_string_bytes.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_NESTING,
        &[value.maximum_nesting]
    )?;
    for (spec, field) in [
        (
            schema::capabilities::CONTROL_COMMAND_SLOTS,
            value.control_command_slots,
        ),
        (
            schema::capabilities::CONTROL_COMMAND_BYTES,
            value.control_command_bytes,
        ),
        (
            schema::capabilities::RELIABLE_RESPONSE_SLOTS,
            value.reliable_response_slots,
        ),
        (
            schema::capabilities::RELIABLE_EVENT_SLOTS,
            value.reliable_event_slots,
        ),
        (schema::capabilities::TELEMETRY_SLOTS, value.telemetry_slots),
        (schema::capabilities::REPLAY_ENTRIES, value.replay_entries),
        (schema::capabilities::REPLAY_BYTES, value.replay_bytes),
        (
            schema::capabilities::MAXIMUM_CACHED_RESPONSE_BYTES,
            value.maximum_cached_response_bytes,
        ),
    ] {
        write_spec!(sink, spec, &field.to_le_bytes())?;
    }
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_CONTROL_PAGE_ITEMS,
        &value.maximum_control_page_items.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_DIAGNOSTIC_PAGE_ITEMS,
        &value.maximum_diagnostic_page_items.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TELEMETRY_HANDLES,
        &value.maximum_telemetry_handles.to_le_bytes(),
    )?;
    write_spec!(
        sink,
        schema::capabilities::MAXIMUM_TRANSACTION_EDITS,
        &value.maximum_transaction_edits.to_le_bytes(),
    )?;
    write_packed_u16(
        sink,
        schema::capabilities::SUPPORTED_COMMANDS,
        value.supported_commands,
    )?;
    write_packed_u16(
        sink,
        schema::capabilities::SUPPORTED_EVENTS,
        value.supported_events,
    )?;
    write_spec!(
        sink,
        schema::capabilities::FLAGS,
        &value.flags.0.to_le_bytes()
    )?;
    Ok(())
}

fn write_packed_u64(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[u64],
) -> Result<(), EncodeError> {
    let value_len = values
        .len()
        .checked_mul(8)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, value_len, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
}

fn write_packed_u16(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    values: &[u16],
) -> Result<(), EncodeError> {
    let value_len = values
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::LimitExceeded)?;
    sink.stream_field_spec(spec, value_len, &mut |sink| {
        for value in values {
            sink.raw(&value.to_le_bytes())?;
        }
        Ok(())
    })
}

fn decode_capabilities<'a>(
    codec: &ProtocolCodec,
    message: Message<'a>,
) -> Result<DecodedCapabilities<'a>, DecodeError> {
    // Fields 9, 12, 19 and 20 are retired with `AUTOMATION_ENQUEUE` (#1627): refused in either
    // flag form, never skipped as unknown optional fields.
    let message = message
        .schema_spec_retiring(&schema::capabilities::SPEC, &schema::capabilities::RETIRED)?;
    let value = DecodedCapabilities {
        minimum_version: crate::ProtocolVersion {
            major: read_u16(one_spec!(
                message,
                schema::capabilities::MINIMUM_VERSION_MAJOR
            )?)?,
            minor: read_u16(one_spec!(
                message,
                schema::capabilities::MINIMUM_VERSION_MINOR
            )?)?,
        },
        maximum_version: crate::ProtocolVersion {
            major: read_u16(one_spec!(
                message,
                schema::capabilities::MAXIMUM_VERSION_MAJOR
            )?)?,
            minor: read_u16(one_spec!(
                message,
                schema::capabilities::MAXIMUM_VERSION_MINOR
            )?)?,
        },
        maximum_frame_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_FRAME_BYTES
        )?)?,
        maximum_tlvs: read_u32(one_spec!(message, schema::capabilities::MAXIMUM_TLVS)?)?,
        maximum_string_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_STRING_BYTES
        )?)?,
        maximum_nesting: read_u8(one_spec!(message, schema::capabilities::MAXIMUM_NESTING)?)?,
        control_command_slots: read_u64(one_spec!(
            message,
            schema::capabilities::CONTROL_COMMAND_SLOTS
        )?)?,
        control_command_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::CONTROL_COMMAND_BYTES
        )?)?,
        reliable_response_slots: read_u64(one_spec!(
            message,
            schema::capabilities::RELIABLE_RESPONSE_SLOTS
        )?)?,
        reliable_event_slots: read_u64(one_spec!(
            message,
            schema::capabilities::RELIABLE_EVENT_SLOTS
        )?)?,
        telemetry_slots: read_u64(one_spec!(message, schema::capabilities::TELEMETRY_SLOTS)?)?,
        replay_entries: read_u64(one_spec!(message, schema::capabilities::REPLAY_ENTRIES)?)?,
        replay_bytes: read_u64(one_spec!(message, schema::capabilities::REPLAY_BYTES)?)?,
        maximum_cached_response_bytes: read_u64(one_spec!(
            message,
            schema::capabilities::MAXIMUM_CACHED_RESPONSE_BYTES
        )?)?,
        maximum_control_page_items: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_CONTROL_PAGE_ITEMS
        )?)?,
        maximum_diagnostic_page_items: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_DIAGNOSTIC_PAGE_ITEMS
        )?)?,
        maximum_telemetry_handles: read_u16(one_spec!(
            message,
            schema::capabilities::MAXIMUM_TELEMETRY_HANDLES
        )?)?,
        maximum_transaction_edits: read_u32(one_spec!(
            message,
            schema::capabilities::MAXIMUM_TRANSACTION_EDITS
        )?)?,
        supported_commands: one_spec!(message, schema::capabilities::SUPPORTED_COMMANDS)?,
        supported_events: one_spec!(message, schema::capabilities::SUPPORTED_EVENTS)?,
        flags: CapabilityFlags(read_u64(one_spec!(message, schema::capabilities::FLAGS)?)?),
    };
    if !value.supported_commands.len().is_multiple_of(2)
        || !value.supported_events.len().is_multiple_of(2)
    {
        return Err(DecodeError::InvalidValueLength);
    }
    check_capabilities_invariants(CapabilityInvariantView::from(&value))?;
    let _ = codec;
    Ok(value)
}

fn check_capabilities(value: &Capabilities<'_>) -> Result<(), EncodeError> {
    check_capabilities_invariants(CapabilityInvariantView::from(value))
        .map_err(|_| EncodeError::LimitExceeded)
}

#[derive(Clone, Copy)]
enum IdSource<'a> {
    Native(&'a [u16]),
    LittleEndian(&'a [u8]),
}

impl IdSource<'_> {
    fn len(self) -> Option<usize> {
        match self {
            Self::Native(ids) => Some(ids.len()),
            Self::LittleEndian(bytes) if bytes.len().is_multiple_of(2) => Some(bytes.len() / 2),
            Self::LittleEndian(_) => None,
        }
    }

    fn get(self, index: usize) -> Option<u16> {
        match self {
            Self::Native(ids) => ids.get(index).copied(),
            Self::LittleEndian(bytes) => {
                let offset = index.checked_mul(2)?;
                Some(u16::from_le_bytes([
                    *bytes.get(offset)?,
                    *bytes.get(offset + 1)?,
                ]))
            }
        }
    }

    fn contains(self, wanted: u16) -> bool {
        self.len()
            .is_some_and(|len| (0..len).any(|index| self.get(index) == Some(wanted)))
    }

    fn is_strict_allocated(self, events: bool) -> bool {
        let Some(len) = self.len() else {
            return false;
        };
        let mut prior = None;
        for index in 0..len {
            let Some(id) = self.get(index) else {
                return false;
            };
            if !allocated_id(id, events) || prior.is_some_and(|previous| id <= previous) {
                return false;
            }
            prior = Some(id);
        }
        true
    }
}

#[derive(Clone, Copy)]
struct CapabilityInvariantView<'a> {
    minimum_version: crate::ProtocolVersion,
    maximum_version: crate::ProtocolVersion,
    maximum_frame_bytes: u64,
    maximum_tlvs: u32,
    maximum_nesting: u8,
    maximum_control_page_items: u16,
    maximum_diagnostic_page_items: u16,
    maximum_transaction_edits: u32,
    supported_commands: IdSource<'a>,
    supported_events: IdSource<'a>,
    flags: CapabilityFlags,
}

impl<'a> From<&Capabilities<'a>> for CapabilityInvariantView<'a> {
    fn from(value: &Capabilities<'a>) -> Self {
        Self {
            minimum_version: value.minimum_version,
            maximum_version: value.maximum_version,
            maximum_frame_bytes: value.maximum_frame_bytes,
            maximum_tlvs: value.maximum_tlvs,
            maximum_nesting: value.maximum_nesting,
            maximum_control_page_items: value.maximum_control_page_items,
            maximum_diagnostic_page_items: value.maximum_diagnostic_page_items,
            maximum_transaction_edits: value.maximum_transaction_edits,
            supported_commands: IdSource::Native(value.supported_commands),
            supported_events: IdSource::Native(value.supported_events),
            flags: value.flags,
        }
    }
}

impl<'a> From<&DecodedCapabilities<'a>> for CapabilityInvariantView<'a> {
    fn from(value: &DecodedCapabilities<'a>) -> Self {
        Self {
            minimum_version: value.minimum_version,
            maximum_version: value.maximum_version,
            maximum_frame_bytes: value.maximum_frame_bytes,
            maximum_tlvs: value.maximum_tlvs,
            maximum_nesting: value.maximum_nesting,
            maximum_control_page_items: value.maximum_control_page_items,
            maximum_diagnostic_page_items: value.maximum_diagnostic_page_items,
            maximum_transaction_edits: value.maximum_transaction_edits,
            supported_commands: IdSource::LittleEndian(value.supported_commands),
            supported_events: IdSource::LittleEndian(value.supported_events),
            flags: value.flags,
        }
    }
}

fn check_capabilities_invariants(value: CapabilityInvariantView<'_>) -> Result<(), DecodeError> {
    let has_command = |wanted| value.supported_commands.contains(wanted);
    let has_event = |wanted| value.supported_events.contains(wanted);
    let session_family = has_command(0x0003) && has_event(0x8001);
    if value.minimum_version.major != 1
        || value.maximum_version.major != 1
        || value.minimum_version.minor > value.maximum_version.minor
        || value.maximum_frame_bytes == 0
        || value.maximum_tlvs < 23
        || value.maximum_nesting == 0
        || value.maximum_control_page_items > 256
        || value.maximum_diagnostic_page_items > 256
        || ((value.maximum_transaction_edits != 0) != session_family)
        || value.flags.0 & !CapabilityFlags::KNOWN != 0
        || !value.supported_commands.is_strict_allocated(false)
        || !value.supported_events.is_strict_allocated(true)
        || ((value.flags.0 & (1 << 3) != 0) != session_family)
        || ((value.flags.0 & (1 << 4) != 0) != has_command(0x0002))
        || ((value.flags.0 & (1 << 6) != 0) != has_command(0x0009))
        || ((value.flags.0 & (1 << 7) != 0) != (has_command(0x000c) && has_command(0x000d)))
        || ((value.flags.0 & (1 << 8) != 0) != has_command(0x0007))
        || ((value.flags.0 & (1 << 9) != 0) != has_event(0x8020))
        || ((value.flags.0 & (1 << 10) != 0) != (has_command(0x000a) && has_event(0x8021)))
        || ((value.flags.0 & (1 << 11) != 0) != (has_command(0x000b) && has_event(0x8030)))
        || ((value.flags.0 & CapabilityFlags::SESSION_EVENT_STREAM.0 != 0) != session_family)
        || ((value.flags.0 & (1 << 13) != 0) != (has_command(0x0008) && has_event(0x8010)))
        // Bit 14 means `RESPONSE_GET` and its `RESPONSE_READY` event together (#1578 D1).
        || ((value.flags.0 & (1 << 14) != 0) != has_command(0x000e))
        || (has_command(0x000e) != has_event(0x8040))
        // `OBSERVATION_READ` and bit 15 together, and both ride the session family, since the
        // `Observe` edit that arms a tap is a transaction's (#1586 D6).
        || ((value.flags.0 & CapabilityFlags::OBSERVATION.0 != 0) != has_command(0x000f))
        || (has_command(0x000f) && !session_family)
        // The three spectrum commands are listed together or not at all, and bit 16 says so
        // (#1594 D10, #1595 D8a).
        || ((value.flags.0 & CapabilityFlags::SPECTRUM.0 != 0) != has_command(0x0010))
        || (has_command(0x0010) != has_command(0x0011))
        || (has_command(0x0010) != has_command(0x0012))
        // `LOUDNESS` and bit 17 together (#2115).
        || ((value.flags.0 & CapabilityFlags::LOUDNESS.0 != 0) != has_command(0x0014))
        // `SESSION_REPLACE` rides the session family and has no flag bit of its own (#1386).
        || (has_command(0x0013) && !session_family)
        || (has_command(0x0003) != has_event(0x8001))
        // `OBSERVATION_ENDED` rides the session family (#1584): it follows a commit.
        || (has_event(0x8011) && !session_family)
        || (has_command(0x000c) != has_command(0x000d))
        || (has_command(0x0008) != has_event(0x8010))
        || (has_command(0x000a) != has_event(0x8021))
        || (has_command(0x000b) != has_event(0x8030))
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(())
}
fn allocated_id(id: u16, event: bool) -> bool {
    if event {
        // `0x8002` is retired: never allocated again, never advertised (#1628). `0x8040` is
        // `RESPONSE_READY` (#1578).
        matches!(
            id,
            0x8001 | 0x8010 | 0x8011 | 0x8020 | 0x8021 | 0x8030 | 0x8040
        ) && !crate::wire::is_retired_message_id(id)
    } else {
        // `0x0004`, `0x0005` and `0x0006` are retired: never allocated again, never advertised.
        // `0x000f` is `OBSERVATION_READ` (#1586), `0x0010` is `SPECTRUM_CONFIGURE` (#1594),
        // `0x0011` and `0x0012` are `SPECTRUM_CAPTURE` and `SPECTRUM_STREAM` (#1595) and `0x0013`
        // is `SESSION_REPLACE` (#1386). `0x0014` is `LOUDNESS` (#2115).
        matches!(id, 1..=0x0014) && !crate::wire::is_retired_message_id(id)
    }
}

fn write_diagnostic_field(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| {
        write_diagnostic_message(codec, sink, value)
    })
}

fn write_diagnostic_message(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &Diagnostic,
) -> Result<(), EncodeError> {
    check_diagnostic(codec, value)?;
    let count = schema::diagnostic::SPEC.field_count(&[
        (schema::diagnostic::PATH, value.path.len()),
        (
            schema::diagnostic::DETAIL,
            usize::from(value.detail.is_some()),
        ),
        (
            schema::diagnostic::OPERATION_INDEX,
            usize::from(value.operation_index.is_some()),
        ),
        (
            schema::diagnostic::SAMPLE_TIME,
            usize::from(value.sample_time.is_some()),
        ),
        (
            schema::diagnostic::PROVIDER_SEQUENCE,
            usize::from(value.provider_sequence.is_some()),
        ),
    ])?;
    sink.message_header(count)?;
    write_spec!(sink, schema::diagnostic::CODE, value.code.as_bytes())?;
    write_spec!(sink, schema::diagnostic::SEVERITY, &[value.severity as u8])?;
    for segment in &value.path {
        write_path_segment_field(codec, sink, schema::diagnostic::PATH, segment)?;
    }
    if let Some(detail) = &value.detail {
        write_spec!(sink, schema::diagnostic::DETAIL, detail.as_bytes())?;
    }
    if let Some(index) = value.operation_index {
        write_spec!(
            sink,
            schema::diagnostic::OPERATION_INDEX,
            &index.to_le_bytes()
        )?;
    }
    if let Some(sample) = value.sample_time {
        write_spec!(sink, schema::diagnostic::SAMPLE_TIME, &sample.to_le_bytes())?;
    }
    if let Some(sequence) = value.provider_sequence {
        write_spec!(
            sink,
            schema::diagnostic::PROVIDER_SEQUENCE,
            &sequence.to_le_bytes()
        )?;
    }
    Ok(())
}

fn write_path_segment_field(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: &PathSegment,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| {
        write_path_segment_message(codec, sink, value)
    })
}

fn write_path_segment_message(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &PathSegment,
) -> Result<(), EncodeError> {
    check_path_segment(codec, value)?;
    let variant = match value {
        PathSegment::Field(_) => schema::path_segment::FIELD,
        PathSegment::Index(_) => schema::path_segment::INDEX,
        PathSegment::StableId(_) => schema::path_segment::STABLE_ID,
    };
    sink.message_header(schema::path_segment::SPEC.field_count(&[(variant, 1)])?)?;
    write_spec!(sink, schema::path_segment::TAG, &[path_segment_tag(value)])?;
    match value {
        PathSegment::Field(field) => {
            write_spec!(sink, schema::path_segment::FIELD, field.as_bytes())?
        }
        PathSegment::Index(index) => {
            write_spec!(sink, schema::path_segment::INDEX, &index.to_le_bytes())?
        }
        PathSegment::StableId(id) => {
            write_spec!(sink, schema::path_segment::STABLE_ID, id.as_bytes())?
        }
    }
    Ok(())
}

fn write_backpressure_field(
    sink: &mut dyn Sink,
    spec: schema::FieldSpec,
    value: Backpressure,
) -> Result<(), EncodeError> {
    sink.nested_spec(spec, &mut |sink| write_backpressure_message(sink, value))
}

fn write_backpressure_message(sink: &mut dyn Sink, value: Backpressure) -> Result<(), EncodeError> {
    check_backpressure(value)?;
    let count = schema::backpressure::SPEC.field_count(&[
        (
            schema::backpressure::GENERATION,
            usize::from(value.generation.is_some()),
        ),
        (
            schema::backpressure::RETRY_BOUNDARY,
            usize::from(value.retry_boundary.is_some()),
        ),
        (
            schema::backpressure::REQUESTED_BYTES,
            usize::from(value.requested_bytes.is_some()),
        ),
        (
            schema::backpressure::AVAILABLE_BYTES,
            usize::from(value.available_bytes.is_some()),
        ),
    ])?;
    sink.message_header(count)?;
    write_spec!(
        sink,
        schema::backpressure::QUEUE_KIND,
        &[value.queue_kind as u8]
    )?;
    write_spec!(
        sink,
        schema::backpressure::CAPACITY,
        &value.capacity.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::backpressure::OCCUPANCY,
        &value.occupancy.to_le_bytes()
    )?;
    write_spec!(
        sink,
        schema::backpressure::REQUESTED_ITEMS,
        &value.requested_items.to_le_bytes()
    )?;
    if let Some(generation) = value.generation {
        write_spec!(
            sink,
            schema::backpressure::GENERATION,
            &generation.to_le_bytes()
        )?;
    }
    if let Some(boundary) = value.retry_boundary {
        write_spec!(
            sink,
            schema::backpressure::RETRY_BOUNDARY,
            &boundary.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.requested_bytes {
        write_spec!(
            sink,
            schema::backpressure::REQUESTED_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    if let Some(bytes) = value.available_bytes {
        write_spec!(
            sink,
            schema::backpressure::AVAILABLE_BYTES,
            &bytes.to_le_bytes()
        )?;
    }
    Ok(())
}

pub(crate) fn write_diagnostic_event(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    diagnostic: &Diagnostic,
) -> Result<(), EncodeError> {
    if diagnostic.provider_sequence.is_none() {
        return Err(EncodeError::LimitExceeded);
    }
    let required_nesting = if diagnostic.path.is_empty() { 1 } else { 2 };
    if codec.limits().max_nesting < required_nesting {
        return Err(EncodeError::LimitExceeded);
    }
    sink.check_field_count(schema::diagnostic_event::SPEC.field_count(&[])?)?;
    write_diagnostic_field(
        codec,
        sink,
        schema::diagnostic_event::DIAGNOSTIC,
        diagnostic,
    )
}

pub(crate) fn write_diagnostics_page(
    codec: &ProtocolCodec,
    sink: &mut dyn Sink,
    value: &DiagnosticsPage,
) -> Result<(), EncodeError> {
    validate_diagnostics_page(value)?;
    let required_nesting = if value
        .diagnostics
        .iter()
        .any(|diagnostic| !diagnostic.path.is_empty())
    {
        2
    } else {
        1
    };
    if codec.limits().max_nesting < required_nesting {
        return Err(EncodeError::LimitExceeded);
    }
    let count = schema::diagnostics_page::SPEC.field_count(&[(
        schema::diagnostics_page::DIAGNOSTIC,
        value.diagnostics.len(),
    )])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        schema::diagnostics_page::LAST_SEQUENCE,
        &value.last_sequence.to_le_bytes()
    )?;
    write_spec!(sink, schema::diagnostics_page::EOF, &[u8::from(value.eof)])?;
    for diagnostic in &value.diagnostics {
        write_diagnostic_field(
            codec,
            sink,
            schema::diagnostics_page::DIAGNOSTIC,
            diagnostic,
        )?;
    }
    Ok(())
}

fn check_path_segment(codec: &ProtocolCodec, value: &PathSegment) -> Result<(), EncodeError> {
    match value {
        PathSegment::Field(field) => check_string(codec, field),
        PathSegment::Index(_) => Ok(()),
        PathSegment::StableId(id) => {
            check_string(codec, id)?;
            if !valid_stable_id(id) {
                return Err(EncodeError::LimitExceeded);
            }
            Ok(())
        }
    }
}

fn check_diagnostic(codec: &ProtocolCodec, value: &Diagnostic) -> Result<(), EncodeError> {
    check_string(codec, &value.code)?;
    if !valid_dotted_code(&value.code) {
        return Err(EncodeError::LimitExceeded);
    }
    if let Some(detail) = &value.detail {
        check_string(codec, detail)?;
    }
    Ok(())
}

fn check_non_ok_nesting(codec: &ProtocolCodec, value: &NonOkResponse) -> Result<(), EncodeError> {
    let nested_diagnostic = !value.diagnostics.is_empty();
    let nested_backpressure = value.backpressure.is_some();
    let nested_path = value
        .diagnostics
        .iter()
        .any(|diagnostic| !diagnostic.path.is_empty());
    let required_depth = if nested_path {
        2
    } else if nested_diagnostic || nested_backpressure {
        1
    } else {
        0
    };
    if codec.limits().max_nesting < required_depth {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn check_backpressure(value: Backpressure) -> Result<(), EncodeError> {
    if value.capacity == 0
        || value.occupancy > value.capacity
        || value.requested_items != 1
        || value.queue_kind.has_generation() != value.generation.is_some()
        || value.requested_bytes.is_some() != value.available_bytes.is_some()
    {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn check_string(codec: &ProtocolCodec, value: &str) -> Result<(), EncodeError> {
    if value.len() > codec.limits().max_string_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(())
}

fn path_segment_tag(value: &PathSegment) -> u8 {
    match value {
        PathSegment::Field(_) => 1,
        PathSegment::Index(_) => 2,
        PathSegment::StableId(_) => 3,
    }
}

fn decode_non_ok(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<NonOkResponse, DecodeError> {
    let message = message.schema_spec(&schema::non_ok::SPEC)?;
    let diagnostics = values_spec!(message, schema::non_ok::DIAGNOSTIC)?
        .map(|value| decode_diagnostic(codec, Message::nested_at_depth(value, codec.limits(), 1)?))
        .collect::<Result<Vec<_>, _>>()?;
    let omitted_diagnostics = read_u32(one_spec!(message, schema::non_ok::OMITTED_DIAGNOSTICS)?)?;
    let backpressure = optional_spec!(message, schema::non_ok::BACKPRESSURE)?
        .map(|value| {
            decode_backpressure(codec, Message::nested_at_depth(value, codec.limits(), 1)?)
        })
        .transpose()?;
    Ok(NonOkResponse {
        diagnostics,
        omitted_diagnostics,
        backpressure,
    })
}

fn decode_diagnostic(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<Diagnostic, DecodeError> {
    let message = message.schema_spec(&schema::diagnostic::SPEC)?;
    let code = read_string(codec, one_spec!(message, schema::diagnostic::CODE)?)?.to_owned();
    if !valid_dotted_code(&code) {
        return Err(DecodeError::InvalidTlv);
    }
    let severity =
        DiagnosticSeverity::decode(read_u8(one_spec!(message, schema::diagnostic::SEVERITY)?)?)?;
    let path = values_spec!(message, schema::diagnostic::PATH)?
        .map(|value| {
            decode_path_segment(codec, Message::nested_at_depth(value, codec.limits(), 2)?)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let detail = optional_spec!(message, schema::diagnostic::DETAIL)?
        .map(|value| read_string(codec, value).map(str::to_owned))
        .transpose()?;
    let operation_index = optional_spec!(message, schema::diagnostic::OPERATION_INDEX)?
        .map(read_u32)
        .transpose()?;
    let sample_time = optional_spec!(message, schema::diagnostic::SAMPLE_TIME)?
        .map(read_u64)
        .transpose()?;
    let provider_sequence = optional_spec!(message, schema::diagnostic::PROVIDER_SEQUENCE)?
        .map(read_u64)
        .transpose()?;
    Ok(Diagnostic {
        code,
        severity,
        path,
        detail,
        operation_index,
        sample_time,
        provider_sequence,
    })
}

fn decode_path_segment(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<PathSegment, DecodeError> {
    let message = message.schema_spec(&schema::path_segment::SPEC)?;
    let tag = read_u8(one_spec!(message, schema::path_segment::TAG)?)?;
    let field = optional_spec!(message, schema::path_segment::FIELD)?;
    let index = optional_spec!(message, schema::path_segment::INDEX)?;
    let stable_id = optional_spec!(message, schema::path_segment::STABLE_ID)?;
    match tag {
        1 if field.is_some() && index.is_none() && stable_id.is_none() => Ok(PathSegment::Field(
            read_string(codec, field.expect("present"))?.to_owned(),
        )),
        2 if field.is_none() && index.is_some() && stable_id.is_none() => {
            Ok(PathSegment::Index(read_u64(index.expect("present"))?))
        }
        3 if field.is_none() && index.is_none() && stable_id.is_some() => {
            let stable_id = read_string(codec, stable_id.expect("present"))?;
            if !valid_stable_id(stable_id) {
                return Err(DecodeError::InvalidTlv);
            }
            Ok(PathSegment::StableId(stable_id.to_owned()))
        }
        _ => Err(DecodeError::InvalidTlv),
    }
}

fn decode_backpressure(
    _codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<Backpressure, DecodeError> {
    let message = message.schema_spec(&schema::backpressure::SPEC)?;
    let queue_kind = BackpressureQueueKind::decode(read_u8(one_spec!(
        message,
        schema::backpressure::QUEUE_KIND
    )?)?)?;
    let value = Backpressure {
        queue_kind,
        capacity: read_u64(one_spec!(message, schema::backpressure::CAPACITY)?)?,
        occupancy: read_u64(one_spec!(message, schema::backpressure::OCCUPANCY)?)?,
        requested_items: read_u16(one_spec!(message, schema::backpressure::REQUESTED_ITEMS)?)?,
        generation: optional_spec!(message, schema::backpressure::GENERATION)?
            .map(read_u64)
            .transpose()?,
        retry_boundary: optional_spec!(message, schema::backpressure::RETRY_BOUNDARY)?
            .map(read_u64)
            .transpose()?,
        requested_bytes: optional_spec!(message, schema::backpressure::REQUESTED_BYTES)?
            .map(read_u64)
            .transpose()?,
        available_bytes: optional_spec!(message, schema::backpressure::AVAILABLE_BYTES)?
            .map(read_u64)
            .transpose()?,
    };
    if value.capacity == 0
        || value.occupancy > value.capacity
        || value.requested_items != 1
        || value.queue_kind.has_generation() != value.generation.is_some()
        || value.requested_bytes.is_some() != value.available_bytes.is_some()
    {
        return Err(DecodeError::InvalidTlv);
    }
    Ok(value)
}

fn read_string<'a>(codec: &ProtocolCodec, value: &'a [u8]) -> Result<&'a str, DecodeError> {
    read_string_within(codec.limits(), value)
}

fn read_string_within(limits: crate::ProtocolLimits, value: &[u8]) -> Result<&str, DecodeError> {
    if value.len() > limits.max_string_bytes {
        return Err(DecodeError::LimitExceeded);
    }
    core::str::from_utf8(value).map_err(|_| DecodeError::InvalidUtf8)
}

fn read_bool(value: &[u8]) -> Result<bool, DecodeError> {
    match read_u8(value)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(DecodeError::InvalidValueLength),
    }
}

fn valid_dotted_code(value: &str) -> bool {
    !value.is_empty()
        && value.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
}

pub(crate) fn valid_stable_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=127).contains(&bytes.len())
        && bytes[0].is_ascii_lowercase()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn checked_sink_len(codec: &ProtocolCodec, sink: &mut dyn Sink) -> Result<usize, EncodeError> {
    sink.finish_message()?;
    let written = sink.written();
    if written > codec.limits().max_frame_bytes {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(written)
}

fn checked_writer_len(sink: &dyn Sink, required: usize) -> Result<usize, EncodeError> {
    sink.finish_message()?;
    if sink.written() != required {
        return Err(EncodeError::LimitExceeded);
    }
    Ok(required)
}

// --- OBSERVATION_READ (`0x000f`, #1586 D1) ---

/// The most selections one `OBSERVATION_READ` names (#1586 D1). The codec refuses an empty
/// request and one with more.
pub const OBSERVATION_READ_MAXIMUM_SELECTIONS: usize = 256;

/// One selection of an `OBSERVATION_READ`: a tap by its stable address and the lanes to return.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationReadSelection {
    /// The track or submix, by its ID.
    pub strip_id: session::StableId,
    /// The console slot or the insert, by its ID.
    pub section: controls::TapSection,
    /// The effect-local tap ID (`ObservationDescriptor::id`).
    pub tap_id: u32,
    /// The lanes to return.
    pub channels: ParameterChannel,
}

/// One selection of an `OBSERVATION_READ`, borrowed from the request bytes (#1586 gate 6): what
/// [`ProtocolCodec::decode_observation_read_frame`] writes into a caller's table, so a host reads a
/// request without allocating.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationSelectionRef<'a> {
    /// The track or submix, by its ID.
    pub strip_id: &'a str,
    /// The console slot or the insert section.
    pub section: ObservationSection,
    /// The console slot's or the insert's ID.
    pub slot_id: &'a str,
    /// The effect-local tap ID.
    pub tap_id: u32,
    /// The lanes to return.
    pub channels: ParameterChannel,
}

impl ObservationSelectionRef<'_> {
    /// A placeholder entry of a caller's table, overwritten by the decode.
    pub const EMPTY: ObservationSelectionRef<'static> = ObservationSelectionRef {
        strip_id: "",
        section: ObservationSection::Inserts,
        slot_id: "",
        tap_id: 0,
        channels: ParameterChannel::Both,
    };
}

/// An `OBSERVATION_READ` request: 1 to [`OBSERVATION_READ_MAXIMUM_SELECTIONS`] selections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationReadRequest {
    /// The taps to read, in the order the reply's rows follow.
    pub selections: Vec<ObservationReadSelection>,
}

control_code_table! {
    /// The status of one `OBSERVATION_READ` row (#1586 D2, under #1585's status contract).
    ObservationRowStatus {
        /// The tap's last subscription is an arm render has not applied yet, or it is armed and
        /// no complete window has closed since its arm, or the read tore on every attempt.
        Pending = 1,
        /// The tap is not armed, or its last subscription is a disarm.
        Unarmed = 2,
        /// A window that opened at or after the arm has closed: the row carries it.
        Ready = 3,
    }
}

/// The window of a ready row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRowWindow {
    /// The first absolute sample of the window; at or after the row's `arm_sample`.
    pub first_sample: u64,
    /// The exclusive end sample of the window.
    pub end_sample: u64,
    /// The window's sequence number in its slot.
    pub sequence: u64,
    /// The render blocks the window spans.
    pub blocks: u32,
}

/// One row of an `OBSERVATION_READ` reply, for the selection at the same position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservationReadRow {
    /// The row's status.
    pub status: ObservationRowStatus,
    /// Whether render has applied an arm the slot still holds: the slot's published state, or
    /// `false` when render has not applied the tap's last record or the read tore.
    pub armed: bool,
    /// The sample of the arm render applied; `0` when none.
    pub arm_sample: u64,
    /// The tag of the record render applied last; `0` when none.
    pub arm_tag: u64,
    /// The window: present exactly when the status is ready.
    pub window: Option<ObservationRowWindow>,
    /// The left lane: present exactly when ready and asked for.
    pub left: Option<f32>,
    /// The right lane: present exactly when ready and asked for.
    pub right: Option<f32>,
    /// The tap's native unit.
    pub unit: ParameterUnit,
}

/// The success reply of an `OBSERVATION_READ`: one row per selection, in selection order. The
/// rows borrow the session's row table when it encodes and own them when decoded.
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationReadPage<'a> {
    /// The render sample at the read: the end of the last block render completed.
    pub observed_sample: u64,
    /// The rows: 1 to [`OBSERVATION_READ_MAXIMUM_SELECTIONS`].
    pub rows: std::borrow::Cow<'a, [ObservationReadRow]>,
}

const fn observation_count_is_valid(count: usize) -> bool {
    count != 0 && count <= OBSERVATION_READ_MAXIMUM_SELECTIONS
}

/// A row's presence rules, both directions: a window and lanes exactly when ready; a ready row is
/// armed, tagged, and its window opened at or after the arm and is not empty.
fn observation_row_is_valid(row: &ObservationReadRow) -> bool {
    match (row.status, row.window) {
        (ObservationRowStatus::Ready, Some(window)) => {
            row.armed
                && row.arm_tag != 0
                && window.first_sample >= row.arm_sample
                && window.end_sample > window.first_sample
                && window.blocks != 0
                && (row.left.is_some() || row.right.is_some())
        }
        (ObservationRowStatus::Ready, None) | (_, Some(_)) => false,
        (_, None) => row.left.is_none() && row.right.is_none(),
    }
}

pub(crate) fn write_observation_read_request(
    sink: &mut dyn Sink,
    value: &ObservationReadRequest,
) -> Result<(), EncodeError> {
    use schema::{observation_read_request as r, observation_selection as s};
    if !observation_count_is_valid(value.selections.len()) {
        return Err(EncodeError::InvalidField);
    }
    let count = r::SPEC.field_count(&[(r::SELECTION, value.selections.len())])?;
    sink.check_field_count(count)?;
    for selection in &value.selections {
        sink.nested_spec(r::SELECTION, &mut |sink| {
            sink.message_header(s::SPEC.field_count(&[])?)?;
            sink.nested_spec(s::ADDRESS, &mut |sink| {
                crate::session_wire::tx_observe_address(
                    sink,
                    &selection.strip_id,
                    &selection.section,
                )
            })?;
            write_spec!(sink, s::TAP_ID, &selection.tap_id.to_le_bytes())?;
            write_spec!(
                sink,
                s::CHANNELS,
                &[schema::parameter_channel_wire(selection.channels)]
            )
        })?;
    }
    Ok(())
}

/// The one decode of an `OBSERVATION_READ` request's selections: into `out`, borrowed from the
/// request, without allocating. Returns how many it wrote.
fn decode_observation_selections<'a>(
    codec: &ProtocolCodec,
    message: Message<'a>,
    out: &mut [ObservationSelectionRef<'a>],
) -> Result<usize, DecodeError> {
    use schema::{observation_read_request as r, observation_selection as s};
    let limits = codec.limits();
    let message = message.schema_spec(&r::SPEC)?;
    let mut count = 0;
    for selection in values_spec!(message, r::SELECTION)? {
        if count == OBSERVATION_READ_MAXIMUM_SELECTIONS {
            return Err(DecodeError::InvalidField);
        }
        let slot = out.get_mut(count).ok_or(DecodeError::LimitExceeded)?;
        let selection = Message::nested_at_depth(selection, limits, 1)?.schema_spec(&s::SPEC)?;
        let (strip_id, section, slot_id) = crate::session_wire::parse_observe_address_ref(
            selection.nested_value(one_spec!(selection, s::ADDRESS)?)?,
        )?;
        *slot = ObservationSelectionRef {
            strip_id,
            section,
            slot_id,
            tap_id: read_u32(one_spec!(selection, s::TAP_ID)?)?,
            channels: schema::parameter_channel_from_wire(read_u8(one_spec!(
                selection,
                s::CHANNELS
            )?)?)
            .map_err(|_| DecodeError::InvalidField)?,
        };
        count += 1;
    }
    if count == 0 {
        return Err(DecodeError::InvalidField);
    }
    Ok(count)
}

fn decode_observation_read_request(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ObservationReadRequest, DecodeError> {
    let mut table = [ObservationSelectionRef::EMPTY; OBSERVATION_READ_MAXIMUM_SELECTIONS];
    let count = decode_observation_selections(codec, message, &mut table)?;
    let id = |text: &str| session::StableId::parse(text).ok_or(DecodeError::InvalidField);
    let selections = table[..count]
        .iter()
        .map(|selection| {
            let slot_id = id(selection.slot_id)?;
            Ok(ObservationReadSelection {
                strip_id: id(selection.strip_id)?,
                section: match selection.section {
                    ObservationSection::Console => controls::TapSection::Console(slot_id),
                    ObservationSection::Inserts => controls::TapSection::Inserts(slot_id),
                },
                tap_id: selection.tap_id,
                channels: selection.channels,
            })
        })
        .collect::<Result<Vec<_>, DecodeError>>()?;
    Ok(ObservationReadRequest { selections })
}

pub(crate) fn write_observation_read_page(
    sink: &mut dyn Sink,
    value: &ObservationReadPage<'_>,
) -> Result<(), EncodeError> {
    use schema::{observation_read_page as p, observation_row as r};
    if !observation_count_is_valid(value.rows.len())
        || !value.rows.iter().all(observation_row_is_valid)
    {
        return Err(EncodeError::InvalidField);
    }
    let count = p::SPEC.field_count(&[(p::ROW, value.rows.len())])?;
    sink.check_field_count(count)?;
    write_spec!(
        sink,
        p::OBSERVED_SAMPLE,
        &value.observed_sample.to_le_bytes()
    )?;
    for row in value.rows.iter() {
        sink.nested_spec(p::ROW, &mut |sink| {
            let window = usize::from(row.window.is_some());
            sink.message_header(r::SPEC.field_count(&[
                (r::FIRST_SAMPLE, window),
                (r::END_SAMPLE, window),
                (r::SEQUENCE, window),
                (r::BLOCKS, window),
                (r::LEFT, usize::from(row.left.is_some())),
                (r::RIGHT, usize::from(row.right.is_some())),
            ])?)?;
            write_spec!(sink, r::STATUS, &[row.status.code()])?;
            write_spec!(sink, r::ARMED, &[u8::from(row.armed)])?;
            write_spec!(sink, r::ARM_SAMPLE, &row.arm_sample.to_le_bytes())?;
            write_spec!(sink, r::ARM_TAG, &row.arm_tag.to_le_bytes())?;
            if let Some(window) = row.window {
                write_spec!(sink, r::FIRST_SAMPLE, &window.first_sample.to_le_bytes())?;
                write_spec!(sink, r::END_SAMPLE, &window.end_sample.to_le_bytes())?;
                write_spec!(sink, r::SEQUENCE, &window.sequence.to_le_bytes())?;
                write_spec!(sink, r::BLOCKS, &window.blocks.to_le_bytes())?;
            }
            if let Some(left) = row.left {
                write_spec!(sink, r::LEFT, &left.to_le_bytes())?;
            }
            if let Some(right) = row.right {
                write_spec!(sink, r::RIGHT, &right.to_le_bytes())?;
            }
            write_spec!(sink, r::UNIT, &[schema::parameter_unit_wire(row.unit)])
        })?;
    }
    Ok(())
}

fn decode_observation_read_page(
    codec: &ProtocolCodec,
    message: Message<'_>,
) -> Result<ObservationReadPage<'static>, DecodeError> {
    use schema::{observation_read_page as p, observation_row as r};
    let limits = codec.limits();
    let message = message.schema_spec(&p::SPEC)?;
    let observed_sample = read_u64(one_spec!(message, p::OBSERVED_SAMPLE)?)?;
    let mut rows = Vec::new();
    for row in values_spec!(message, p::ROW)? {
        if rows.len() == OBSERVATION_READ_MAXIMUM_SELECTIONS {
            return Err(DecodeError::InvalidField);
        }
        let row = Message::nested_at_depth(row, limits, 1)?.schema_spec(&r::SPEC)?;
        let word = |spec: schema::FieldSpec| optional_spec!(row, spec)?.map(read_u64).transpose();
        let lane = |spec: schema::FieldSpec| optional_spec!(row, spec)?.map(read_f32).transpose();
        let blocks = optional_spec!(row, r::BLOCKS)?.map(read_u32).transpose()?;
        // The four window fields are present together or not at all.
        let window = match (
            word(r::FIRST_SAMPLE)?,
            word(r::END_SAMPLE)?,
            word(r::SEQUENCE)?,
            blocks,
        ) {
            (Some(first_sample), Some(end_sample), Some(sequence), Some(blocks)) => {
                Some(ObservationRowWindow {
                    first_sample,
                    end_sample,
                    sequence,
                    blocks,
                })
            }
            (None, None, None, None) => None,
            _ => return Err(DecodeError::InvalidField),
        };
        let value = ObservationReadRow {
            status: ObservationRowStatus::from_code(read_u8(one_spec!(row, r::STATUS)?)?)?,
            armed: read_bool(one_spec!(row, r::ARMED)?)?,
            arm_sample: read_u64(one_spec!(row, r::ARM_SAMPLE)?)?,
            arm_tag: read_u64(one_spec!(row, r::ARM_TAG)?)?,
            window,
            left: lane(r::LEFT)?,
            right: lane(r::RIGHT)?,
            unit: schema::parameter_unit_from_wire(read_u8(one_spec!(row, r::UNIT)?)?)
                .map_err(|_| DecodeError::InvalidField)?,
        };
        if !observation_row_is_valid(&value) {
            return Err(DecodeError::InvalidField);
        }
        rows.push(value);
    }
    if rows.is_empty() {
        return Err(DecodeError::InvalidField);
    }
    Ok(ObservationReadPage {
        observed_sample,
        rows: std::borrow::Cow::Owned(rows),
    })
}

impl ProtocolCodec {
    /// Return the exact caller-buffer size of an `OBSERVATION_READ` request.
    pub fn encoded_observation_read_request_len(
        &self,
        value: &ObservationReadRequest,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_observation_read_request(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode an `OBSERVATION_READ` request without allocation.
    pub fn encode_observation_read_request(
        &self,
        value: &ObservationReadRequest,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_observation_read_request_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_observation_read_request(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode an `OBSERVATION_READ` request.
    pub fn decode_observation_read_request(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ObservationReadRequest, DecodeError> {
        decode_observation_read_request(self, Message::top_level(payload, count, self.limits())?)
    }

    /// Strictly decode a complete `OBSERVATION_READ` command frame's selections into `out`, borrowed
    /// from `input`, without allocating (#1586 gate 6). `out` holds
    /// [`OBSERVATION_READ_MAXIMUM_SELECTIONS`] entries; returns how many the request names.
    ///
    /// # Errors
    ///
    /// Any decode error of the frame or its payload; a frame of another message is
    /// [`DecodeError::MessageKindMismatch`].
    pub fn decode_observation_read_frame<'a>(
        &self,
        input: &'a [u8],
        out: &mut [ObservationSelectionRef<'a>],
    ) -> Result<usize, DecodeError> {
        let decoded = self.decode_header(input)?;
        let header = decoded
            .header
            .command()
            .ok_or(DecodeError::MessageKindMismatch)?;
        if header.message_id != crate::MessageId::ObservationRead {
            return Err(DecodeError::MessageKindMismatch);
        }
        decode_observation_selections(
            self,
            Message::top_level(decoded.payload, header.tlv_count, self.limits())?,
            out,
        )
    }

    /// Return the exact caller-buffer size of an `OBSERVATION_READ` reply.
    pub fn encoded_observation_read_page_len(
        &self,
        value: &ObservationReadPage<'_>,
    ) -> Result<usize, EncodeError> {
        let mut sink = CountSink::new(self.limits());
        write_observation_read_page(&mut sink, value)?;
        checked_sink_len(self, &mut sink)
    }

    /// Encode an `OBSERVATION_READ` reply without allocation.
    pub fn encode_observation_read_page(
        &self,
        value: &ObservationReadPage<'_>,
        output: &mut [u8],
    ) -> Result<usize, EncodeError> {
        let required = self.encoded_observation_read_page_len(value)?;
        if output.len() < required {
            return Err(EncodeError::OutputTooSmall { required });
        }
        let mut writer = SliceSink::new(output, self.limits());
        write_observation_read_page(&mut writer, value)?;
        checked_writer_len(&writer, required)
    }

    /// Strictly decode an `OBSERVATION_READ` reply.
    pub fn decode_observation_read_page(
        &self,
        payload: &[u8],
        count: u32,
    ) -> Result<ObservationReadPage<'static>, DecodeError> {
        decode_observation_read_page(self, Message::top_level(payload, count, self.limits())?)
    }

    /// The largest complete response frame, in bytes, of an `OBSERVATION_READ` of `selections`
    /// taps: every row ready with both lanes, the widest a row encodes. Measured by the encoder
    /// itself, so it cannot drift from it, and exact for that reply, since every row field has a
    /// fixed width. Allocates nothing.
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidField`] for 0 or more than [`OBSERVATION_READ_MAXIMUM_SELECTIONS`]
    /// selections; [`EncodeError::LimitExceeded`] when the payload exceeds this codec's frame
    /// limit.
    pub fn observation_read_largest_frame_len(
        &self,
        selections: usize,
    ) -> Result<usize, EncodeError> {
        const WIDEST: ObservationReadRow = ObservationReadRow {
            status: ObservationRowStatus::Ready,
            armed: true,
            arm_sample: 0,
            arm_tag: 1,
            window: Some(ObservationRowWindow {
                first_sample: 0,
                end_sample: 1,
                sequence: 0,
                blocks: 1,
            }),
            left: Some(0.0),
            right: Some(0.0),
            unit: ParameterUnit::Db,
        };
        static ROWS: [ObservationReadRow; OBSERVATION_READ_MAXIMUM_SELECTIONS] =
            [WIDEST; OBSERVATION_READ_MAXIMUM_SELECTIONS];
        let rows = ROWS.get(..selections).ok_or(EncodeError::InvalidField)?;
        self.encoded_observation_read_page_len(&ObservationReadPage {
            observed_sample: 0,
            rows: std::borrow::Cow::Borrowed(rows),
        })?
        .checked_add(crate::OUTER_HEADER_BYTES)
        .ok_or(EncodeError::LimitExceeded)
    }
}

#[cfg(test)]
mod tests;
