//! Deterministic complete schema corpus shared by native and Wasm conformance runners.

use protocol::*;

use session::{
    Console, ConsoleEntry, ConsoleSlot, EffectIdentity, EffectQuality, LinkMode, Output, RackName,
    RouteSource, SendTap, SessionModel, StableId, Submix, Vca,
};

/// Build the checked-in canonical fixture transaction that contains every V1 edit opcode.
///
/// This is conformance data, not a session-edit convenience API.  It deliberately derives its
/// nested values from the checked-in strict V1 JSON fixture, so the transaction follows the
/// accepted typed model rather than maintaining a second shadow session representation.
#[must_use]
pub fn complete_all_opcode_fixture() -> Vec<SessionEdit> {
    let session =
        session::parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
            .expect("checked-in canonical session fixture is valid");
    let source = session.sources[0].clone();
    let mut track = session.tracks[0].clone();
    let effect = track.inserts.effects[0].clone();
    // Decision 12: carry a console entry so the corpus encodes the track's field 11. The fixture
    // declares no console slot; this is codec data, not a session a store would accept.
    track.console = vec![session::ConsoleEntry {
        slot: StableId::parse("desk-eq").expect("literal stable ID"),
        bypass: true,
        link_mode: session::ConsoleLinkMode::Slot,
        params: effect.params.clone(),
    }];
    let route = session.routes[0].clone();
    let automation = session.automation[0].clone();
    let track_id = track.id.clone();
    let effect_id = effect.id.clone();
    let id = |value| StableId::parse(value).expect("literal stable ID");
    // Decision 12 (#1094): one slot in each console section, so `SetConsole` encodes both repeated
    // section fields, and a track entry for each, so `SetTrackConsole` repeats its entry field.
    let console = Console {
        pre_insert: vec![ConsoleSlot {
            slot: id("desk-eq"),
            identity: effect.identity.clone(),
            quality: effect.quality,
            link_mode: effect.link_mode,
        }],
        post_insert: vec![ConsoleSlot {
            slot: id("desk-limit"),
            identity: EffectIdentity::Native {
                effect_id: id("miso.true-peak-limiter"),
            },
            quality: EffectQuality::High,
            link_mode: LinkMode::Maximum,
        }],
    };
    // #1199: a non-trivial submix strip -- non-default trim, an insert, a muted lane and a
    // matrix -- so the corpus encodes every submix field (1, 2, 4, 5 and tagged 6). #1202: it
    // carries an entry for both console slots, one live with the effect's parameters and one
    // bypassed, so the corpus repeats submix field 3.
    let mut submix = Submix::unity(id("drums"), &console);
    submix.builtins.left.trim_db = -3.0;
    submix.console[0].bypass = false;
    submix.console[0].params = effect.params.clone();
    submix.inserts.effects.push(effect.clone());
    submix.fader.right_mute = true;
    submix.matrix_or_pan = session::MatrixOrPan::Matrix {
        ll: 0.75,
        lr: 0.25,
        rl: -0.25,
        rr: 0.5,
        smoothing_samples: 16,
    };
    let mut track_console = track.console.clone();
    track_console.push(ConsoleEntry {
        slot: id("desk-limit"),
        bypass: false,
        link_mode: session::ConsoleLinkMode::Slot,
        params: Vec::new(),
    });
    vec![
        SessionEdit::SetSessionId {
            session_id: id("demo.session"),
        },
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 48_000,
        },
        SessionEdit::SetQuantumFrames {
            quantum_frames: 128,
        },
        SessionEdit::SetRenderProfile {
            render_profile: session.render_profile.clone(),
        },
        SessionEdit::SetOutputProfile {
            output_profile: session.output_profile.clone(),
        },
        SessionEdit::SetConsole {
            console,
            ramp_samples: None,
        },
        SessionEdit::UpsertSource {
            source: source.clone(),
        },
        SessionEdit::RemoveSource {
            source_id: source.id.clone(),
        },
        SessionEdit::SetSourceContent {
            source_id: source.id.clone(),
            content: source.content.clone(),
            channels: source.channels,
            bit_depth: source.bit_depth,
            frames: source.frames,
        },
        SessionEdit::UpsertTrack {
            track: track.clone(),
        },
        SessionEdit::RemoveTrack {
            track_id: track_id.clone(),
        },
        SessionEdit::SetTrackSourceAssignment {
            track_id: track_id.clone(),
            source_id: source.id.clone(),
            left_source_channel: 0,
            right_source_channel: 1,
        },
        SessionEdit::SetTrackRack {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            rack: track.inserts.clone(),
        },
        SessionEdit::PutTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            final_position: 0,
            effect: effect.clone(),
        },
        SessionEdit::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
        },
        SessionEdit::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_ids: vec![effect_id.clone()],
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            identity: effect.identity.clone(),
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            quality: effect.quality,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            sidechain: effect.sidechain.clone(),
        },
        SessionEdit::SetTrackConsole {
            track_id: track_id.clone(),
            console: track_console,
            ramp_samples: None,
        },
        SessionEdit::UpsertSubmix { submix },
        SessionEdit::RemoveSubmix {
            submix_id: id("drums"),
        },
        SessionEdit::UpsertOutput {
            output: Output { id: id("alt-out") },
        },
        SessionEdit::RemoveOutput {
            output_id: id("alt-out"),
        },
        SessionEdit::UpsertRoute {
            route: route.clone(),
        },
        SessionEdit::RemoveRoute {
            route_id: route.id.clone(),
        },
        // #1203 D5: a tapped submix source, so the hash covers tag 2's required tap. A codec
        // value; the corpus is not a session a store applies.
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: RouteSource::Submix {
                submix_id: id("drums"),
                tap: SendTap::PreFader,
            },
        },
        SessionEdit::SetRouteDestination {
            route_id: route.id.clone(),
            destination: route.destination.clone(),
        },
        SessionEdit::UpsertAutomation {
            automation: automation.clone(),
        },
        SessionEdit::RemoveAutomation {
            automation_id: automation.id.clone(),
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: automation.target.clone(),
        },
        SessionEdit::SetAutomationSegments {
            automation_id: automation.id.clone(),
            segments: automation.segments.clone(),
        },
        // #1241: a VCA over the track and a submix (members repeat field 3), with distinct lane
        // offsets and one muted lane, then its removal and a fader set. Codec values: the corpus
        // is never applied to a store.
        SessionEdit::UpsertVca {
            vca: Vca {
                id: id("drums-vca"),
                fader: session::DualMonoFader {
                    left_db: -6.0,
                    right_db: -4.5,
                    left_mute: false,
                    right_mute: true,
                },
                members: vec![track_id.clone(), id("drums")],
            },
        },
        SessionEdit::RemoveVca {
            vca_id: id("drums-vca"),
        },
        // #1553: the first fader of the track by its stable address, with a ramp of 240 samples.
        SessionEdit::SetControl {
            expected_effect: None,
            address: control_core::ControlAddress {
                owner: control_core::Owner::Strip(track_id.clone()),
                section: control_core::Section::Builtins,
                control: 5,
                lane: control_core::Lane::Left,
            },
            value: -3.0,
            ramp_samples: Some(240),
        },
        // #1678: an insert's parameter, with the native effect the edit was written for.
        SessionEdit::SetControl {
            address: control_core::ControlAddress {
                owner: control_core::Owner::Strip(track_id.clone()),
                section: control_core::Section::Inserts(id("eq1")),
                control: 2,
                lane: control_core::Lane::Both,
            },
            value: 1.5,
            ramp_samples: None,
            expected_effect: Some(id("miso.parametric-eq")),
        },
        // #1584 D1: `edit.observe`, an arm of an insert's tap with a window of four blocks.
        SessionEdit::Observe {
            strip_id: track_id.clone(),
            section: control_core::TapSection::Inserts(id("eq1")),
            tap_id: 1,
            armed: true,
            window_blocks: 4,
            arm_tag: 0,
        },
    ]
}

/// Encode the complete fixture for test-only protocol consumers without exporting this crate's
/// private `protocol` dependency type across the protocol test dependency cycle.
#[must_use]
pub fn complete_all_opcode_fixture_bytes() -> Vec<u8> {
    let edits = complete_all_opcode_fixture();
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("literal request ID"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let required = codec
        .encoded_session_transaction_len(&transaction)
        .expect("conformance fixture encodes");
    let mut bytes = vec![0; required];
    codec
        .encode_session_transaction(&transaction, &mut bytes)
        .expect("conformance fixture encodes");
    bytes
}

/// The session every [`retired_code_rows`] row is judged against: the canonical fixture with one
/// `pre_insert` console slot, `desk-eq`, and the track's entry for it (decision 12).
///
/// Each console-refused row names `desk-eq` on the fixture's track, a slot this session declares
/// and the track carries, so every such row would find something to change if `console` were not
/// refused. The session compiles.
#[must_use]
pub fn console_session_fixture() -> SessionModel {
    let mut session =
        session::parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
            .expect("checked-in canonical session fixture is valid");
    let insert = session.tracks[0].inserts.effects[0].clone();
    let slot = StableId::parse("desk-eq").expect("literal stable ID");
    session.console.pre_insert = vec![ConsoleSlot {
        slot: slot.clone(),
        identity: EffectIdentity::Native {
            effect_id: StableId::parse("miso.parametric-eq").expect("literal stable ID"),
        },
        quality: insert.quality,
        link_mode: insert.link_mode,
    }];
    session.tracks[0].console = vec![ConsoleEntry {
        slot,
        bypass: false,
        link_mode: session::ConsoleLinkMode::Slot,
        params: insert.params,
    }];
    session
}

/// One wire conformance row for decision 12's retired and refused codes (#1094).
///
/// `bytes` is one complete command frame, request ID 1: a `SESSION_TRANSACTION_APPLY` at exact
/// revision 7, or, for the four retired message IDs, a frame under the retired ID (empty, or for `0x0006` carrying the payload the command had), which
/// the endpoint refuses with the decoder's `UNSUPPORTED_MESSAGE` and no response frame. An endpoint whose session is [`console_session_fixture`] answers it with `status`
/// and exactly the diagnostic codes in `diagnostic`: the endpoint's generic `protocol.failure` for
/// a frame refused at decode, the edit's own code for an edit refusal, none for a success. A
/// refusing row commits nothing. The rows carry numbers and strings rather than protocol types so
/// a protocol unit test can judge them across the test dependency cycle.
pub struct RetiredCodeRow {
    /// Stable review label.
    pub name: String,
    /// The complete command frame.
    pub bytes: Vec<u8>,
    /// The expected response status, as its registry number.
    pub status: u16,
    /// The expected diagnostic code of a refusal; `None` for a success.
    pub diagnostic: Option<&'static str>,
}

/// Every retired or refused code decision 12 leaves on the session-edit wire, one row each:
///
/// - the retired rack codes `1` (`simd1`) and `3` (`simd2`) in every rack-addressed edit
///   (`MALFORMED_FRAME`) and in an automation target's address `section` (`INVALID_FIELD`, #1566),
///   refused at decode, never read as another rack;
/// - the retired track fields 6 (`simd1`) and 8 (`simd2`), mandatory (`UNKNOWN_REQUIRED_FIELD`)
///   and optional (`MALFORMED_FRAME`, never skipped as an unknown optional field);
/// - each structural or declaration edit addressed at `console` (`VALIDATION_FAILED`,
///   `session.edit.console_slot_fixed`);
/// - the thirteen value ops that decision 17 (C5, #1561) retired, `0x0008`, `0x0203`, `0x020a`,
///   `0x020b`, `0x020d`-`0x0210`, `0x0504`-`0x0507` and `0x0702`, spliced into a frame that
///   otherwise carries a `SET_CONTROL` payload: an unallocated opcode, refused at decode
///   (`MALFORMED_FRAME`) whatever the payload holds;
/// - the stored transport position (#1635), `TRANSPORT_SET` field 2, mandatory
///   (`UNKNOWN_REQUIRED_FIELD`) and optional (`MALFORMED_FRAME`): refused at decode, so the
///   transport state, the revision and the reliable lane stay as they were. The snapshot's field 2
///   and the `TRANSPORT_STATE` event's field 3 are responses and events no endpoint decodes;
///   the codec's decoder tests refuse them in the same two flag forms;
/// - a `SetTrackConsole` in a transaction with no `SetConsole` before it that changes the console
///   (`VALIDATION_FAILED`, `session.edit.value_op_retired`);
/// - the retired command message IDs `0x0004` (`PARAMETER_METADATA_GET`) and `0x0005`
///   (`PARAMETER_STATE_GET`), replaced by `CONTROL_LIST_GET` and `CONTROL_STATE_GET` (decision 17,
///   C4), `0x0006` (`AUTOMATION_ENQUEUE`, #1627, a frame with the payload the command had) and
///   `0x8002` (the event `AUTOMATION_CANCELED`, #1628, under the command header): `UNSUPPORTED_MESSAGE`,
///   never served and never reallocated. A message ID that decodes to
///   no message leaves no safe request identity, so these four rows carry the status the decoder
///   maps the refusal to, no diagnostic, and the endpoint writes no response frame for them.
///
/// The last row, `control.unknown_optional_track_field_12`, is the splice's control: the same
/// frame surgery with a never-allocated optional field ID decodes and commits (`OK`), so a
/// retired-field row cannot pass on a frame the splice broke.
#[must_use]
pub fn retired_code_rows() -> Vec<RetiredCodeRow> {
    const MALFORMED_FRAME: u16 = StatusCode::MalformedFrame as u16;
    const INVALID_FIELD: u16 = StatusCode::InvalidField as u16;
    const UNKNOWN_REQUIRED_FIELD: u16 = StatusCode::UnknownRequiredField as u16;
    const UNSUPPORTED_MESSAGE: u16 = StatusCode::UnsupportedMessage as u16;
    const VALIDATION_FAILED: u16 = StatusCode::ValidationFailed as u16;
    const DECODE_REFUSAL: Option<&str> = Some("protocol.failure");
    let session = console_session_fixture();
    let track = session.tracks[0].clone();
    let insert = track.inserts.effects[0].clone();
    let slot_effect = session::Effect {
        id: session.console.pre_insert[0].slot.clone(),
        identity: session.console.pre_insert[0].identity.clone(),
        params: Vec::new(),
        ..insert.clone()
    };
    let mut rows = Vec::new();
    for code in [1_u8, 3] {
        for (name, edit) in rack_addressed_edits(&track.id, RackName::Inserts, &insert) {
            let mut bytes = transaction_frame(edit);
            patch_rack_code(&mut bytes, &[1, 2, 2], code);
            rows.push(RetiredCodeRow {
                name: format!("retired.rack_code_{code}.{name}"),
                bytes,
                status: MALFORMED_FRAME,
                diagnostic: DECODE_REFUSAL,
            });
        }
        let automation = &session.automation[0];
        let mut bytes = transaction_frame(SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: automation.target.clone(),
        });
        patch_rack_code(&mut bytes, &[1, 2, 2, 3], code);
        rows.push(RetiredCodeRow {
            name: format!("retired.rack_code_{code}.set_automation_target"),
            bytes,
            // The target is the `SET_CONTROL` address now (#1566 D4): its `section` field, whose
            // unallocated codes (the retired racks among them) are `INVALID_FIELD` there too.
            status: INVALID_FIELD,
            diagnostic: DECODE_REFUSAL,
        });
    }
    for field in [6_u16, 8] {
        for (flag, status) in [
            ("mandatory", UNKNOWN_REQUIRED_FIELD),
            ("optional", MALFORMED_FRAME),
        ] {
            rows.push(RetiredCodeRow {
                name: format!("retired.track_field_{field}.{flag}"),
                bytes: upsert_track_with_field(&track, field, flag == "mandatory"),
                status,
                diagnostic: DECODE_REFUSAL,
            });
        }
    }
    for (name, edit) in rack_addressed_edits(&track.id, RackName::Console, &slot_effect) {
        rows.push(RetiredCodeRow {
            name: format!("refused.console.{name}"),
            bytes: transaction_frame(edit),
            status: VALIDATION_FAILED,
            diagnostic: Some("session.edit.console_slot_fixed"),
        });
    }
    for opcode in RETIRED_VALUE_OPCODES {
        let mut bytes = transaction_frame(retired_value_probe(&track.id));
        patch_opcode(&mut bytes, opcode);
        rows.push(RetiredCodeRow {
            name: format!("retired.value_op_{opcode:#06x}"),
            bytes,
            status: MALFORMED_FRAME,
            diagnostic: DECODE_REFUSAL,
        });
    }
    rows.push(RetiredCodeRow {
        name: "refused.set_track_console_without_set_console".to_owned(),
        bytes: transaction_frame(SessionEdit::SetTrackConsole {
            track_id: track.id.clone(),
            console: session.tracks[0].console.clone(),
            ramp_samples: None,
        }),
        status: VALIDATION_FAILED,
        diagnostic: Some("session.edit.value_op_retired"),
    });
    for retired in [0x0004_u16, 0x0005, 0x0006, 0x8002] {
        let mut bytes = vec![0; OUTER_HEADER_BYTES];
        ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(1).expect("literal request ID"),
                    expected_revision: ExpectedRevision::Any,
                    payload: CommandPayload::CapabilitiesGet,
                },
                &mut bytes,
            )
            .expect("an empty command frame encodes");
        bytes[16..18].copy_from_slice(&retired.to_le_bytes());
        if retired == 0x0006 {
            // `AUTOMATION_ENQUEUE` is refused by its ID alone, whatever the payload: carry the
            // payload the command had, so the row is the frame a peer of the old build sent.
            add_former_automation_enqueue_payload(&mut bytes);
        }
        rows.push(RetiredCodeRow {
            name: format!("retired.message_id_{retired:#06x}"),
            bytes,
            status: UNSUPPORTED_MESSAGE,
            diagnostic: None,
        });
    }
    // Counters 2 and 11 to 14 retired with the automation queue (#1629). A `TELEMETRY_CONFIGURE`
    // naming one is refused at decode; a `COUNTERS_GET` naming one finds no such counter, as for
    // any ID the provider does not report. `TRANSITION_REPREPARE_REFUSALS` (17) is the sentinel
    // the probe frame carries before it is patched.
    for retired in RETIRED_COUNTER_IDS {
        let mut configure = counter_command_frame(CommandPayload::TelemetryConfigure(
            &TelemetryConfiguration {
                meters: Vec::new(),
                meter_period_blocks: 0,
                meter_window_blocks: None,
                counter_ids: vec![CounterId::TransitionReprepareRefusals],
                counter_period_blocks: 3,
                diagnostics_enabled: false,
                minimum_diagnostic_severity: DiagnosticSeverity::Info,
            },
        ));
        let sentinel = (CounterId::TransitionReprepareRefusals as u32).to_le_bytes();
        let at = configure[OUTER_HEADER_BYTES..]
            .windows(4)
            .position(|window| window == sentinel)
            .expect("the probe frame carries its counter ID")
            + OUTER_HEADER_BYTES;
        configure[at..at + 4].copy_from_slice(&retired.to_le_bytes());
        rows.push(RetiredCodeRow {
            name: format!("retired.counter_{retired}.telemetry_configure"),
            bytes: configure,
            status: INVALID_FIELD,
            diagnostic: DECODE_REFUSAL,
        });
        rows.push(RetiredCodeRow {
            name: format!("retired.counter_{retired}.counters_get"),
            bytes: counter_command_frame(CommandPayload::CountersGet(&CountersRequest {
                all: false,
                ids: vec![retired],
            })),
            status: StatusCode::NotFound as u16,
            diagnostic: DECODE_REFUSAL,
        });
    }
    // The stored transport position (#1635): `TRANSPORT_SET` field 2 is refused at decode in both
    // flag forms, before the transport state moves or a `TRANSPORT_STATE` event is queued.
    for (flag, status) in [
        ("mandatory", UNKNOWN_REQUIRED_FIELD),
        ("optional", MALFORMED_FRAME),
    ] {
        rows.push(RetiredCodeRow {
            name: format!("retired.transport_set_field_2.{flag}"),
            bytes: transport_set_with_position(flag == "mandatory"),
            status,
            diagnostic: DECODE_REFUSAL,
        });
    }
    rows.push(RetiredCodeRow {
        name: "control.unknown_optional_track_field_12".to_owned(),
        bytes: upsert_track_with_field(&track, 12, false),
        status: StatusCode::Ok as u16,
        diagnostic: None,
    });
    rows
}

/// The counter IDs retired with the automation queue (#1629). Their codes are refused or unserved
/// and never reallocated.
const RETIRED_COUNTER_IDS: [u32; 5] = [2, 11, 12, 13, 14];

/// One complete command frame, request ID 1: exact revision 7 for the state-changing
/// `TELEMETRY_CONFIGURE` and `TRANSPORT_SET`, any revision for the read.
fn counter_command_frame(payload: CommandPayload<'_>) -> Vec<u8> {
    let frame = TypedCommandFrame {
        request_id: RequestId::new(1).expect("literal request ID"),
        expected_revision: match payload {
            CommandPayload::TelemetryConfigure(_) | CommandPayload::TransportSet(_) => {
                ExpectedRevision::Exact(SessionRevision(7))
            }
            _ => ExpectedRevision::Any,
        },
        payload,
    };
    let mut bytes = vec![0; 512];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("a counter command frame encodes");
    bytes.truncate(len);
    bytes
}

/// A `TRANSPORT_SET` to playing that also carries the retired field 2, the absolute position
/// (#1635), as a `U64` (wire code 4) of 9 in the given flag form: the frame a peer of the old build sent.
fn transport_set_with_position(mandatory: bool) -> Vec<u8> {
    let mut bytes = counter_command_frame(CommandPayload::TransportSet(TransportSetRequest {
        state: TransportState::Playing,
    }));
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&[4, u8::from(mandatory)]);
    bytes.extend_from_slice(&8_u32.to_le_bytes());
    bytes.extend_from_slice(&9_u64.to_le_bytes());
    let payload_len = u32::try_from(bytes.len() - OUTER_HEADER_BYTES).expect("a short frame");
    bytes[20..24].copy_from_slice(&payload_len.to_le_bytes());
    bytes[40..44].copy_from_slice(&2_u32.to_le_bytes());
    bytes
}

/// Append the payload `AUTOMATION_ENQUEUE` carried (#1627) to an empty command frame: `count`
/// (field 1, `U16`), `record_bytes` (field 2, `U16`) and `records` (field 3, `Bytes`), one valid
/// 32-byte point record. The result is the complete frame the retired command had.
fn add_former_automation_enqueue_payload(bytes: &mut Vec<u8>) {
    // The 32-byte point record: kind 1, three reserved zero bytes, handle 1, start 1, end 1 (two
    // `u64`), then both values 0.0 (two `f32`).
    let mut record_bytes = [0_u8; 32];
    record_bytes[0] = 1;
    record_bytes[4] = 1;
    record_bytes[8] = 1;
    record_bytes[16] = 1;
    let mut field = |id: u16, wire: u8, value: &[u8]| {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&[wire, 1]);
        bytes.extend_from_slice(
            &u32::try_from(value.len())
                .expect("a short value")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(value);
        bytes.resize(bytes.len().next_multiple_of(8), 0);
    };
    field(1, 2, &1_u16.to_le_bytes());
    field(2, 2, &32_u16.to_le_bytes());
    field(3, 10, &record_bytes);
    let payload_len = u32::try_from(bytes.len() - OUTER_HEADER_BYTES).expect("a short frame");
    bytes[20..24].copy_from_slice(&payload_len.to_le_bytes());
    bytes[40..44].copy_from_slice(&3_u32.to_le_bytes());
}

/// The value ops decision 17 retired (C5, #1561). Their codes are refused at decode and never
/// reallocated.
const RETIRED_VALUE_OPCODES: [u16; 13] = [
    0x0008, 0x0203, 0x020a, 0x020b, 0x020d, 0x020e, 0x020f, 0x0210, 0x0504, 0x0505, 0x0506, 0x0507,
    0x0702,
];

/// A `SET_CONTROL` of the track's first fader: the payload a retired-opcode row carries, which the
/// decoder must refuse for its opcode alone.
fn retired_value_probe(track_id: &StableId) -> SessionEdit {
    SessionEdit::set_control(
        control_core::ControlAddress {
            owner: control_core::Owner::Strip(track_id.clone()),
            section: control_core::Section::Builtins,
            control: 5,
            lane: control_core::Lane::Left,
        },
        -3.0,
        None,
    )
}

/// Overwrite the opcode of the one edit of a transaction frame with `opcode`.
fn patch_opcode(bytes: &mut [u8], opcode: u16) {
    let prefixes = tlv_path(bytes, &[1, 1]);
    let field = *prefixes.last().expect("a nonempty path");
    assert_eq!(
        (bytes[field + 2], bytes[field + 3], u32_at(bytes, field + 4)),
        (2, 1, 2),
        "the patched field must be the mandatory U16 opcode"
    );
    bytes[field + 8..field + 10].copy_from_slice(&opcode.to_le_bytes());
}

/// The seven rack-addressed structural and declaration session edits, each naming `rack_name` and
/// `effect` on `track_id`.
fn rack_addressed_edits(
    track_id: &StableId,
    rack_name: RackName,
    effect: &session::Effect,
) -> Vec<(&'static str, SessionEdit)> {
    let track_id = track_id.clone();
    let effect_id = effect.id.clone();
    vec![
        (
            "set_track_rack",
            SessionEdit::SetTrackRack {
                track_id: track_id.clone(),
                rack_name,
                rack: session::Rack {
                    effects: vec![effect.clone()],
                },
            },
        ),
        (
            "put_track_effect",
            SessionEdit::PutTrackEffect {
                track_id: track_id.clone(),
                rack_name,
                final_position: 0,
                effect: effect.clone(),
            },
        ),
        (
            "remove_track_effect",
            SessionEdit::RemoveTrackEffect {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
            },
        ),
        (
            "set_track_effect_order",
            SessionEdit::SetTrackEffectOrder {
                track_id: track_id.clone(),
                rack_name,
                effect_ids: vec![effect_id.clone()],
            },
        ),
        (
            "set_effect_identity",
            SessionEdit::SetEffectIdentity {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                identity: effect.identity.clone(),
            },
        ),
        (
            "set_effect_quality",
            SessionEdit::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                quality: EffectQuality::High,
            },
        ),
        (
            "set_effect_sidechain",
            SessionEdit::SetEffectSidechain {
                track_id,
                rack_name,
                effect_id,
                sidechain: session::SidechainDeclaration::None,
            },
        ),
    ]
}

/// Encode one edit as a complete transaction frame: request ID 1, exact revision 7.
fn transaction_frame(edit: SessionEdit) -> Vec<u8> {
    let edits = [edit];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("literal request ID"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&transaction)
            .expect("conformance row encodes")
    ];
    codec
        .encode_session_transaction(&transaction, &mut bytes)
        .expect("conformance row encodes");
    bytes
}

/// Byte offset of the nested-field TLVs of the message whose count word is at `message`.
const fn fields_of(message: usize) -> usize {
    message + 8
}

fn u32_at(bytes: &[u8], at: usize) -> usize {
    let word = u32::from_le_bytes(bytes[at..at + 4].try_into().expect("a four-byte word"));
    usize::try_from(word).expect("a u32 fits usize")
}

fn add_u32(bytes: &mut [u8], at: usize, delta: usize) {
    let value = u32::try_from(u32_at(bytes, at) + delta).expect("a grown length fits u32");
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

/// Prefix offset of the first TLV with `id` among `count` TLVs starting at `fields`, or of the
/// first TLV with a greater ID (or the end) when `insertion` is set.
fn tlv(bytes: &[u8], fields: usize, count: usize, id: u16, insertion: bool) -> usize {
    let mut cursor = fields;
    for _ in 0..count {
        let found = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        if found == id || (insertion && found > id) {
            return cursor;
        }
        cursor += 8 + u32_at(bytes, cursor + 4).next_multiple_of(8);
    }
    assert!(insertion, "field {id} is absent from the conformance frame");
    cursor
}

/// Prefix offsets along `path`: the top-level field `path[0]` (the transaction's edit, `1`), then
/// each nested message's field (the edit's payload, `2`, then the payload's own fields).
fn tlv_path(bytes: &[u8], path: &[u16]) -> Vec<usize> {
    let mut prefixes = Vec::with_capacity(path.len());
    let mut fields = OUTER_HEADER_BYTES;
    let mut count = u32_at(bytes, 40);
    for &id in path {
        let prefix = tlv(bytes, fields, count, id, false);
        prefixes.push(prefix);
        count = u32_at(bytes, prefix + 8);
        fields = fields_of(prefix + 8);
    }
    prefixes
}

/// Overwrite the `inserts` rack code (`2`) that `path` reaches with a retired `code`. The field
/// must be the one-byte mandatory `U8` holding `2`, so the row can only patch the rack byte.
fn patch_rack_code(bytes: &mut [u8], path: &[u16], code: u8) {
    let prefixes = tlv_path(bytes, path);
    let rack = *prefixes.last().expect("a nonempty path");
    assert_eq!(
        (
            bytes[rack + 2],
            bytes[rack + 3],
            u32_at(bytes, rack + 4),
            bytes[rack + 8]
        ),
        (1, 1, 1, 2),
        "the patched field must be the mandatory U8 inserts rack code"
    );
    bytes[rack + 8] = code;
}

/// Encode `UpsertTrack { track }` and splice one extra track field `id`: a `MESSAGE` holding an
/// empty message, in ascending field order, with every enclosing length and the track's field
/// count grown to match.
fn upsert_track_with_field(track: &session::Track, id: u16, mandatory: bool) -> Vec<u8> {
    let mut bytes = transaction_frame(SessionEdit::UpsertTrack {
        track: track.clone(),
    });
    let prefixes = tlv_path(&bytes, &[1, 2, 1]);
    let track_message = prefixes[2] + 8;
    let at = tlv(
        &bytes,
        fields_of(track_message),
        u32_at(&bytes, track_message),
        id,
        true,
    );
    let mut field = Vec::with_capacity(16);
    field.extend_from_slice(&id.to_le_bytes());
    field.extend_from_slice(&[11, u8::from(mandatory)]);
    field.extend_from_slice(&8_u32.to_le_bytes());
    field.extend_from_slice(&[0; 8]);
    let grown = field.len();
    bytes.splice(at..at, field);
    add_u32(&mut bytes, track_message, 1);
    for prefix in &prefixes {
        add_u32(&mut bytes, prefix + 4, grown);
    }
    add_u32(&mut bytes, 20, grown);
    bytes
}

/// A canonical complete BTLV frame and the exact typed decoder that owns it.
pub struct ConformanceFrame {
    /// Stable review label.
    pub name: &'static str,
    /// Canonical BTLV bytes.
    pub bytes: Vec<u8>,
    /// Decoder family required for this frame.
    pub decoder: ConformanceDecoder,
}

/// The schema-closed decoder selected by one corpus frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceDecoder {
    /// Full typed command decoder.
    Command,
    /// Full typed response decoder.
    Response,
    /// Full typed event decoder.
    Event,
    /// Full typed session-transaction decoder.
    Transaction,
}

/// The frozen FNV-1a-64 roll of `complete_schema_corpus()` over each frame's `(name, bytes)`,
/// seeded with the 64-bit offset basis `0xcbf2_9ce4_8422_2325` and multiplied by the prime
/// `0x0000_0100_0000_01b3`.
///
/// This is the ONE pin. Before #274 the value was written out twice -- in
/// `tests/conformance_corpus.rs` and again in `src/main.rs` --
/// and the Wasm copy silently fell two re-pins behind (`b454b230`, then #241's `04d291dd`)
/// because the gate that should have caught it could not fail. Both runners now read this
/// constant, so a re-pin is one edit and the two arms cannot disagree by omission; if the Wasm
/// arm ever computes something else, that is a real target divergence and the parity gate says so.
/// Issue #787 changed the two source-identity spellings in the transaction frame from `sha256:`
/// to same-length `blake3:` strings. The frame count and total encoded byte count remain fixed.
/// Issue #1093 (decision 12) repinned it: the track message drops the retired fields 6 and 8,
/// carries `inserts` in field 7 and a console entry in field 11, rack-addressed edits spell the
/// `inserts` code, and the route taps keep their codes under new names.
/// Issue #1094 repinned it from `af1b9b71a0a31727`: the transaction appends `SetConsole`
/// (`0x0007`) and `SetTrackConsole` (`0x0211`), 41 edits, one per allocated opcode. The frame count
/// stays 46.
/// Issue #1199 repinned it from `ebf282621550d44a`: the submix message carries the strip in submix
/// fields 2, 4, 5, 6 (builtins, inserts, fader, tagged pan or matrix), and `UpsertSubmix` encodes a
/// non-trivial one.
/// Issue #1202 repinned it from `ca48855fd3a756b7`: the submix message carries its console entries
/// in submix field 3, and `UpsertSubmix` encodes two (one live, one bypassed).
/// Issue #1203 repinned it from `c0f6ecedbf50920a`: a tag-2 route source carries a required tap,
/// and the corpus's `SetRouteSource` value is a tapped submix source.
/// Issue #1216 repinned it from `a1dcc56f2e4a48f9`: route field `mute` (field 6), and opcode
/// `0x0506` (`SetRouteMute`) appended, 42 edits, one per allocated opcode. The frame count stays 46.
/// Issue #1218 repinned it from `39e5a2c1d317a9fe`: route field `follows_mute` (field 7), and
/// opcode `0x0507` (`SetRouteFollowsMute`) appended, 43 edits. The frame count stays 46.
/// Issue #1241 repinned it from `95c1ceb68e44f6e2`: the VCA message, and opcodes `0x0700`-`0x0702`
/// (`UpsertVca`, `RemoveVca`, `SetVcaFader`) appended, 46 edits. The frame count stays 46.
/// Issue #1394 repinned it from `ab357b6c432f9755`: #1394 appends optional `ramp_samples` to the
/// live-value edits, and the corpus's `SetTrackFader` sets it (480). Absent bytes are unchanged, so
/// that one edit is the whole move; the frame count and the edit count stay 46.
/// Issue #1365 repinned it from `5429f052592101ef`: #1365 appends `SetControlSmoothing` (`0x0008`),
/// 47 edits, one per allocated opcode. The frame count stays 46.
/// Issue #1551 (decision 17, C4) repinned it from `8eb4cc8d075a4355`: `CONTROL_LIST_GET` and
/// `CONTROL_STATE_GET` (`0x000c`, `0x000d`) replace the retired `PARAMETER_METADATA_GET` and
/// `PARAMETER_STATE_GET` (`0x0004`, `0x0005`) as command and response frames, the capability
/// registry lists the new pair, and the capabilities field 21 is the control page bound. The frame
/// count stays 46.
/// Issue #1553 (decision 17, C5) repinned it from `0902357f75bdddcd`: `SetControl` (`0x0800`)
/// appended, 48 edits, one per allocated opcode. The frame count stays 46.
/// Issue #1313 repinned it from `9f13a06993efa99e`: `TransactionApplied` gains the required field
/// 2, `edit_path` (`live` 1, `model_only` 2, `rebuild` 3), so the `SESSION_TRANSACTION_APPLY`
/// response frame carries it. The frame count stays 46.
/// Issue #1561 (decision 17, C5) repinned it from `a19a779439660743`: the thirteen per-kind value
/// ops (`0x0008`, `0x0203`, `0x020a`, `0x020b`, `0x020d`-`0x0210`, `0x0504`-`0x0507`, `0x0702`) are
/// retired and never reallocated, so the transaction carries 35 edits, one per allocated opcode.
/// The frame count stays 46.
/// Batch 4 review fold (decision 17, #1559) repinned it from `0433af905dc618bc`: the corpus gains
/// `response.session_transaction_monitor`, a `TransactionApplied` with `edit_path` `monitor` (4),
/// so the response frame for the fourth path is covered by the typed decoders and the Wasm
/// golden. The frame count is 47.
/// Issue #1573 (decision 17, C13) repinned it from `7de408612b837afd`: `RESPONSE_GET` (`0x000e`)
/// appends one command frame (every optional field of an effect configuration, three overrides)
/// and two response frames (a curve reply with sections on both sides, and a lone tag), the
/// capability registry lists `0x000e` and sets bit 14. The frame count is 50.
/// Issue #1566 (decision 17, C2 and C10) repinned it from `cdfdb0540f093424`: an automation target
/// is the `SET_CONTROL` address message (`owner_kind`, `owner_id`, `section`, `slot_id`,
/// `control`, `lane`) and no longer the five-field strip target, so the `UpsertAutomation` and
/// `SetAutomationTarget` edits of the transaction carry it. The frame count stays 50.
/// Issue #1627 (draft 21a) repinned it from `4f6a80fb52c12aed`: `AUTOMATION_ENQUEUE` (`0x0006`),
/// statuses 15 (`TIME_IN_PAST`) and 16 (`AUTOMATION_ORDER`) and capability fields 9, 12, 19 and 20
/// are retired and never reallocated, so the corpus loses the `command.automation_enqueue` and
/// `response.automation` frames and the two status frames, and the capabilities response carries
/// 23 fields and lists no command `0x0006` (flag bit 5 clear). The frame count is 46.
/// Issue #1628 (draft 21b) repinned it from `cfa58ab7b7bbb7c4`: the event `AUTOMATION_CANCELED`
/// (`0x8002`) is retired and never reallocated, so the corpus loses `event.automation_canceled`
/// and the capabilities response lists no event `0x8002`. The frame count is 45.
/// Issue #1584 (D6a) repinned it from `4b21d9e6f520afd9`: the reliable event `OBSERVATION_ENDED`
/// (`0x8011`) is added, so the corpus gains `event.observation_ended` (the frame count is 46 and the
/// events are six) and the capabilities response lists event `0x8011`.
/// #1584 D6a fold: one OBSERVATION_ENDED per commit. Root ruling A repinned it from
/// `33e4b1cebcbfd902`: the event carries a repeated `key` message (the wire shape is the sequence
/// and one nested key per ended tap), so `event.observation_ended` carries two keys. The frame
/// count stays 46.
/// #1678: an optional seventh field of the `SET_CONTROL` address message names the native effect an
/// effect-row edit was written for. The transaction's last
/// `SetControl` is an insert edit that carries the expected effect `miso.parametric-eq` (36 edits). The
/// frame count stays 46.
/// #1635 retires the stored transport position.
/// `TRANSPORT_SET` field 2, `TransportSnapshot` field 2 and `TransportStateEvent` field 3 are
/// refused and never reallocated, so `command.transport_set` carries the state alone and the
/// snapshot and event frames carry the state and the effective sample. The frame count stays 46.
/// Batch 13b carries #1678 and #1635 together and re-pins it once from `cc9a7bcc502c4936`.
/// #1650 adds the automation offset: the `Automation` message gains a required field 4, the
/// nested `AutomationOffset` (`left_db` 1, `right_db` 2), so the transaction's `UpsertAutomation`
/// carries it. Re-pinned from `0c7196f86abe6d57`; the frame count stays 46.
/// #1650b adds the control row's `automated` flag (#1650 D4): `ControlEntry` field 35, written on
/// every control row, and the list page's live `ratio` row is automated. Re-pinned from
/// `5eb9ad2ed137f3be`; the frame count stays 46.
/// #1386 adds `SESSION_REPLACE` (`0x0013`): the corpus gains `command.session_replace` (the length
/// and the 32-byte digest) and `response.session_replace` (`edit_path` `unchanged`, 5), and the
/// capabilities response lists command `0x0013`. Re-pinned from `5cab2cf3b4e4767f`; the frame
/// count is 48.
/// #1594 adds `SPECTRUM_CONFIGURE` (`0x0010`): the corpus gains `command.spectrum_configure` (a
/// collection of two taps with the hop) and `response.spectrum_configured` (a monitor rebuild with
/// its plan epoch). The capability registry does not list `0x0010` until #1595. The 48 earlier
/// frames are byte-equal: their hash is the old pin. Re-pinned from `c85b2a0dff5d2a92`; the frame
/// count is 50.
/// #1584 adds the `Observe` edit (`0x0801`): the transaction appends `edit.observe`, an arm of an
/// insert's tap with a window of four blocks (37 edits). Only `command.session_transaction_apply`
/// moves; the 49 other frames are byte-equal. Re-pinned from `26579fcd7f959310`; the frame count
/// stays 50.
/// #1595 adds `SPECTRUM_CAPTURE` (`0x0011`): the corpus gains `command.spectrum_capture` (a read of
/// both planes with the raw window) and `response.spectrum_captured` (a ready one-plane read). The
/// capability registry does not list `0x0011` until #1595's second slice. The 50 earlier frames
/// are byte-equal: their hash is the old pin. Re-pinned from `a3d6a9dd72195701`; the frame count
/// is 52.
/// #1595's second slice adds `SPECTRUM_STREAM` (`0x0012`): the corpus gains
/// `command.spectrum_stream` (a select with a smoothing) and `response.spectrum_streamed` (a ready
/// one-plane read). The static capabilities frame lists none of the three spectrum commands, which
/// the bit-16 invariant allows. The 52 earlier frames are byte-equal: their hash is the old pin.
/// Re-pinned from `c7e010362827dab3`; the frame count is 54.
/// #1583 replaces the positional meter handles with the meter table: `command.telemetry_configure`
/// carries a table with both targets and both metrics (field 1 retired, field 7 repeated),
/// `response.telemetry` the same table and the meter window (field 8), and `event.meter_batch` the
/// window's span and sequence (fields 5 and 6). Only those three frames move; the 51 others are
/// byte-equal. Re-pinned from `c146e8fad0b26299`; the frame count stays 54.
/// #1369 appends the console entry's required `LINK_MODE` (field 4, `slot` 1 to `average` 4), so
/// every console entry of the transaction carries it. Only `command.session_transaction_apply`
/// moves; the 53 other frames are byte-equal. Re-pinned from `480ffcf546bea7fb`; the frame count
/// stays 54.
/// #1585 gives `TransactionApplied` its optional arm-tag lists (`observe_arm_tags` 3 and
/// `removed_arm_tags` 4, packed `u64`), and `response.session_transaction` carries both. Only that
/// frame moves; the 53 others are byte-equal. Re-pinned from `06bb3b6f65f39eec`; the frame count
/// stays 54.
/// #1586 adds `OBSERVATION_READ` (`0x000f`): the corpus gains `command.observation_read` (a console
/// slot's tap with the left lane and an insert's with the right) and `response.observation_read` (a
/// ready row with both lanes, a pending armed row and an unarmed row). The static capabilities
/// frame does not list `0x000f`, with bit 15 clear, which the bit-15 invariant allows. The 54
/// earlier frames are byte-equal: their hash is the old pin. Re-pinned from `5b6cbc157a1f6154`;
/// the frame count is 56.
/// #1578 adds `RESPONSE_READY` (`0x8040`): the corpus gains `command.response_get_strip` (a strip
/// target, the tag reply's request), `event.response_ready` (a ready event with one side and a
/// member of each section) and `event.response_ready_refused` (`NOT_FOUND` with its diagnostic),
/// and the static capabilities frame lists `0x8040` with `0x000e`, as bit 14 now requires. Only
/// `response.capabilities` moves (its event list's one padded field gains `0x8040`); with it patched
/// back and the three new frames filtered out, the 56 earlier frames hash to the old pin.
/// Re-pinned from `f4c2fe333739f44d`; the frame count is 59.
/// #1690 gives the meter table the master target (`3`, no strip ID, gain reduction only):
/// `command.telemetry_configure` and `response.telemetry` carry a third entry, the master's gain
/// reduction. Only those two frames move; the 57 others are byte-equal. Re-pinned from
/// `cb8db815ed0103a0`; the frame count stays 59.
/// #2115 adds `LOUDNESS` (`0x0014`): the corpus gains `command.loudness` (a start) and
/// `response.loudness` (an armed read with every optional level present). The static capabilities
/// frame does not list `0x0014`, with bit 17 clear, which the bit-17 invariant allows. The 59
/// earlier frames are byte-equal: their hash is the old pin. Re-pinned from `dea650f7e506833e`;
/// the frame count is 61.
pub const COMPLETE_SCHEMA_HASH: u64 = 0xde7d_7cf5_04ca_191a;

/// Build every command, successful response, registered non-OK status, event, and all-opcode
/// session transaction using only public typed encoder entry points.
#[must_use]
pub fn complete_schema_corpus() -> Vec<ConformanceFrame> {
    let codec = ProtocolCodec::default();
    let request = RequestId::new(1).expect("literal request ID");
    let revision = SessionRevision(7);
    let mut frames = Vec::new();
    let transaction_edits = complete_all_opcode_fixture();
    let control_request = ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 8,
        owner: Some(ControlOwnerFilter {
            kind: ControlOwnerKind::Strip,
            id: Some("vocal".to_owned()),
        }),
        headers_only: false,
    };
    let response_request = ResponseGetRequest {
        target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
            effect: "miso.parametric-eq".to_owned(),
            quality: session::EffectQuality::High,
            link_mode: session::LinkMode::DualMono,
        }),
        grid: ResponseGrid::Logarithmic,
        points: 3,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: ParameterChannel::Both,
        sections: true,
        overrides: vec![
            ResponseOverride {
                control: RESPONSE_BYPASS_CONTROL,
                lane: ParameterChannel::Both,
                value: 0.0,
            },
            ResponseOverride {
                control: 5,
                lane: ParameterChannel::Left,
                value: 3.5,
            },
            ResponseOverride {
                control: 5,
                lane: ParameterChannel::Right,
                value: -2.0,
            },
        ],
        configuration_id: Some(42),
        sample_rate_hz: Some(48_000),
        quantum_frames: Some(128),
        maximum_prepared_bytes: Some(1 << 20),
        maximum_state_bytes: Some(1 << 24),
        maximum_scratch_bytes: Some(1 << 24),
        maximum_automation_spans: Some(4096),
        maximum_reply_bytes: Some(65_536),
    };
    // #1594: a collection of two taps with every optional field (the hop).
    let spectrum_request = SpectrumConfigureRequest {
        form: SpectrumForm::Collection,
        entries: vec![
            SpectrumEntry {
                target_kind: SpectrumTargetKind::TrackPostPan,
                target_id: "vocal".to_owned(),
                channels: ParameterChannel::Both,
            },
            SpectrumEntry {
                target_kind: SpectrumTargetKind::Output,
                target_id: "main".to_owned(),
                channels: ParameterChannel::Left,
            },
        ],
        maximum_capture_bytes: 1 << 18,
        hop_frames: Some(512),
    };
    // #1583 D1/D2: a meter table with every target and both metrics; the echo adds the window.
    // #1690 D3: the master's gain reduction, target 3 with no strip ID.
    let telemetry = TelemetryConfiguration {
        meters: vec![
            MeterEntry {
                target: MeterTarget::Output,
                metric: MeterMetric::Peak,
            },
            MeterEntry {
                target: MeterTarget::Strip("vocal".into()),
                metric: MeterMetric::GainReduction,
            },
            MeterEntry {
                target: MeterTarget::Master,
                metric: MeterMetric::GainReduction,
            },
        ],
        meter_period_blocks: 24,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    };
    let counters = CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    macro_rules! command {
        ($name:literal, $payload:expr) => {
            push_command(
                &mut frames,
                &codec,
                $name,
                TypedCommandFrame {
                    request_id: request,
                    expected_revision: if matches!(
                        &$payload,
                        CommandPayload::CapabilitiesGet
                            | CommandPayload::SessionSnapshotGet(_)
                            | CommandPayload::ControlListGet(_)
                            | CommandPayload::ControlStateGet(_)
                            | CommandPayload::ResponseGet(_)
                            | CommandPayload::SpectrumConfigure(_)
                            | CommandPayload::SpectrumCapture(_)
                            | CommandPayload::SpectrumStream(_)
                            | CommandPayload::ObservationRead(_)
                            | CommandPayload::TransportGet
                            | CommandPayload::CountersGet(_)
                            | CommandPayload::DiagnosticsGet(_)
                    ) {
                        ExpectedRevision::Any
                    } else {
                        ExpectedRevision::Exact(revision)
                    },
                    payload: $payload,
                },
            );
        };
    }
    command!("command.capabilities_get", CommandPayload::CapabilitiesGet);
    command!(
        "command.session_snapshot_get",
        CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
            offset: 0,
            maximum_bytes: 1
        })
    );
    command!(
        "command.session_transaction_apply",
        CommandPayload::SessionTransactionApply(&transaction_edits)
    );
    command!(
        "command.control_list_get",
        CommandPayload::ControlListGet(&control_request)
    );
    command!(
        "command.control_state_get",
        CommandPayload::ControlStateGet(&control_request)
    );
    command!(
        "command.response_get",
        CommandPayload::ResponseGet(&response_request)
    );
    // #1578: a strip target (3), which answers a tag and later a `RESPONSE_READY`. A strip request
    // carries no sections and no overrides.
    let strip_request = ResponseGetRequest {
        target: ResponseTarget::Strip("vocal".to_owned()),
        channels: ParameterChannel::Left,
        sections: false,
        overrides: Vec::new(),
        configuration_id: None,
        maximum_reply_bytes: Some(1 << 16),
        ..response_request.clone()
    };
    command!(
        "command.response_get_strip",
        CommandPayload::ResponseGet(&strip_request)
    );
    command!("command.transport_get", CommandPayload::TransportGet);
    command!(
        "command.transport_set",
        CommandPayload::TransportSet(TransportSetRequest {
            state: TransportState::Playing
        })
    );
    command!(
        "command.telemetry_configure",
        CommandPayload::TelemetryConfigure(&telemetry)
    );
    command!(
        "command.counters_get",
        CommandPayload::CountersGet(&counters)
    );
    command!(
        "command.diagnostics_get",
        CommandPayload::DiagnosticsGet(DiagnosticsRequest {
            after_sequence: 0,
            limit: 1,
            minimum_severity: DiagnosticSeverity::Info
        })
    );
    // #1386: the frame names the document by length and SHA-256 (32 bytes); it carries none.
    command!(
        "command.session_replace",
        CommandPayload::SessionReplace(SessionReplaceRequest {
            document_bytes: 70_000,
            document_sha256: core::array::from_fn(|index| u8::try_from(index).unwrap_or(0)),
        })
    );
    command!(
        "command.spectrum_configure",
        CommandPayload::SpectrumConfigure(&spectrum_request)
    );
    // #1595: a read of both planes with the raw window, so the optional field is present.
    command!(
        "command.spectrum_capture",
        CommandPayload::SpectrumCapture(&SpectrumCaptureRequest::Read {
            channels: ParameterChannel::Both,
            include_window: true,
        })
    );
    // #1595: a select with a smoothing, so every optional field is present.
    command!(
        "command.spectrum_stream",
        CommandPayload::SpectrumStream(&SpectrumStreamRequest::Select {
            target_kind: SpectrumTargetKind::TrackPostInput,
            target_id: "vocal".to_owned(),
            channels: ParameterChannel::Left,
            smoothing_ms: Some(150.0),
        })
    );
    // #2115: a start (the request's one field is the op).
    command!(
        "command.loudness",
        CommandPayload::Loudness(LoudnessRequest::Start)
    );
    // #1586: a console slot's tap with the left lane and an insert's with the right, so both
    // sections and two lane sets are present.
    let observation_read = ObservationReadRequest {
        selections: vec![
            ObservationReadSelection {
                strip_id: StableId::parse("vocal").expect("literal strip ID"),
                section: control_core::TapSection::Console(
                    StableId::parse("bus_comp").expect("literal slot ID"),
                ),
                tap_id: 1,
                channels: ParameterChannel::Left,
            },
            ObservationReadSelection {
                strip_id: StableId::parse("drums").expect("literal strip ID"),
                section: control_core::TapSection::Inserts(
                    StableId::parse("gate").expect("literal insert ID"),
                ),
                tap_id: 2,
                channels: ParameterChannel::Right,
            },
        ],
    };
    command!(
        "command.observation_read",
        CommandPayload::ObservationRead(&observation_read)
    );

    let capabilities = Capabilities {
        minimum_version: ProtocolVersion::V1,
        maximum_version: ProtocolVersion::V1,
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
        maximum_transaction_edits: 64,
        supported_commands: &[1, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14, 0x0013],
        // `RESPONSE_READY` with `RESPONSE_GET`, under bit 14 (#1578 D1).
        supported_events: &[0x8001, 0x8010, 0x8011, 0x8020, 0x8021, 0x8030, 0x8040],
        flags: CapabilityFlags::B4_BASE,
    };
    macro_rules! success {
        ($name:literal, $payload:expr) => {
            push_success(
                &mut frames,
                &codec,
                $name,
                TypedSuccessResponseFrame {
                    request_id: request,
                    revision,
                    payload: $payload,
                },
            );
        };
    }
    success!(
        "response.capabilities",
        SuccessResponsePayload::Capabilities(capabilities)
    );
    success!(
        "response.session_snapshot",
        SuccessResponsePayload::SessionSnapshot(SessionSnapshot {
            total_bytes: 0,
            offset: 0,
            canonical_json_chunk: &[],
            eof: true
        })
    );
    success!(
        "response.session_transaction",
        // #1585 D5: both optional arm-tag lists, so the response carries every field.
        SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
            applied_operations: 42,
            edit_path: EditPath::Rebuild,
            observe_arm_tags: vec![3, 3, 1 << 40],
            removed_arm_tags: vec![2],
        })
    );
    success!(
        "response.session_transaction_monitor",
        SuccessResponsePayload::SessionTransactionApplied(TransactionApplied::new(
            1,
            EditPath::Monitor
        ))
    );
    // #1386: an identical document, so the fifth path (`unchanged`, 5) is covered too.
    success!(
        "response.session_replace",
        SuccessResponsePayload::SessionReplaced(TransactionApplied::new(0, EditPath::Unchanged))
    );
    let (control_page, state_page) = control_corpus_pages();
    success!(
        "response.control_list",
        SuccessResponsePayload::ControlListPage(control_page)
    );
    success!(
        "response.control_state",
        SuccessResponsePayload::ControlStatePage(state_page)
    );
    success!(
        "response.response_get",
        SuccessResponsePayload::ResponseResult(ResponseResult::Curves(ResponseCurves {
            sample_rate_hz: 48_000,
            configuration_id: 42,
            floor_db: -120.0,
            bypass: false,
            section_count: 2,
            enabled_left: 0b11,
            enabled_right: 0b01,
            retained_bytes: 512,
            frequencies_hz: vec![20.0, 632.455_5, 20_000.0],
            left_db: Some(vec![0.0, 1.5, -3.0]),
            right_db: Some(vec![0.0, 0.5, -1.0]),
            sections_left_db: Some(vec![0.0, 1.0, -2.0, 0.0, 0.5, -1.0]),
            sections_right_db: Some(vec![0.0, 0.25, -0.5, 0.0, 0.25, -0.5]),
        }))
    );
    success!(
        "response.response_get_tag",
        SuccessResponsePayload::ResponseResult(ResponseResult::Tag(77))
    );
    // #1594: a monitor rebuild, so the optional plan epoch is present.
    success!(
        "response.spectrum_configured",
        SuccessResponsePayload::SpectrumConfigured(SpectrumConfigured {
            revision: 7,
            configured_epoch: 3,
            adopted_epoch: 2,
            rebuild: true,
            plan_epoch: Some(3),
        })
    );
    // #1595: a ready one-plane read without the window: the smallest ready reply.
    let frequencies: Vec<f32> = (0..SPECTRUM_CAPTURE_BIN_COUNT)
        .map(|bin| f32::from(u16::try_from(bin).unwrap_or(0)) * 23.4375)
        .collect();
    let left_db: Vec<f32> = (0..SPECTRUM_CAPTURE_BIN_COUNT)
        .map(|bin| -f32::from(u16::try_from(bin % 120).unwrap_or(0)))
        .collect();
    success!(
        "response.spectrum_captured",
        SuccessResponsePayload::SpectrumCaptured(SpectrumCaptureReply {
            adopted_epoch: 3,
            selection_epoch: 2,
            status: SpectrumCaptureStatus::Ready,
            window: Some(SpectrumCaptureWindow {
                target_kind: SpectrumTargetKind::TrackPostPan,
                target_id: "vocal".into(),
                channels: ParameterChannel::Left,
                sample_rate_hz: 48_000,
                window_frames: SPECTRUM_CAPTURE_WINDOW_FRAMES,
                bin_count: SPECTRUM_CAPTURE_BIN_COUNT,
                floor_db: -120.0,
                captured_sample: 96_000,
                end_sample: 98_048,
                snapshot_token: 2,
                source_underrun: false,
                frequencies_hz: frequencies.as_slice().into(),
                left_db: Some(left_db.as_slice().into()),
                right_db: None,
                left_window: None,
                right_window: None,
            }),
        })
    );
    // #1595: a ready one-plane stream read, after one dropped window.
    success!(
        "response.spectrum_streamed",
        SuccessResponsePayload::SpectrumStreamed(SpectrumStreamReply {
            adopted_epoch: 3,
            selection_epoch: 2,
            status: SpectrumStreamStatus::Ready,
            target: Some(SpectrumStreamTarget {
                target_kind: SpectrumTargetKind::TrackPostInput,
                target_id: "vocal".into(),
                channels: ParameterChannel::Left,
            }),
            sample_rate_hz: 48_000,
            quantum_frames: 128,
            hop_frames: 1_024,
            capture_epoch: 2,
            sequence: 5,
            dropped_captures: 1,
            windows: 6,
            captured_sample: 96_000,
            end_sample: 98_048,
            analysis_epoch: 4,
            history_start_sample: 90_880,
            smoothing_ms: 150.0,
            source_underrun: false,
            bins: Some(SpectrumStreamBins {
                frequencies_hz: frequencies.as_slice().into(),
                left_db: Some(left_db.as_slice().into()),
                right_db: None,
            }),
        })
    );
    // #2115: an armed read with every optional level present, after two dropped hops.
    success!(
        "response.loudness",
        SuccessResponsePayload::Loudness(LoudnessReply {
            observed_sample: 192_000,
            armed: true,
            integrating_since: Some(48_480),
            hops_dropped: 2,
            sanitized_samples: 3,
            gated_block_count: 290,
            momentary_lufs: Some(-23.25),
            short_term_lufs: Some(-22.5),
            integrated_lufs: Some(-23.0),
            loudness_range_lu: Some(4.5),
            momentary_max_lufs: Some(-18.0),
            short_term_max_lufs: Some(-20.75),
            true_peak_dbtp: Some([-1.5, -2.25]),
            hop_true_peak_dbtp: Some([-6.0, -120.0]),
        })
    );
    // #1586: a ready row with both lanes, a pending armed row and an unarmed one, so every row
    // field is present.
    let ready_row = ObservationReadRow {
        status: ObservationRowStatus::Ready,
        armed: true,
        arm_sample: 4_096,
        arm_tag: 3,
        window: Some(ObservationRowWindow {
            first_sample: 4_096,
            end_sample: 4_608,
            sequence: 2,
            blocks: 4,
        }),
        left: Some(-3.5),
        right: Some(-1.25),
        unit: ParameterUnit::Db,
    };
    let observation_rows = [
        ready_row,
        ObservationReadRow {
            status: ObservationRowStatus::Pending,
            window: None,
            left: None,
            right: None,
            ..ready_row
        },
        ObservationReadRow {
            status: ObservationRowStatus::Unarmed,
            armed: false,
            arm_sample: 0,
            arm_tag: 0,
            window: None,
            left: None,
            right: None,
            unit: ParameterUnit::Linear,
        },
    ];
    success!(
        "response.observation_read",
        SuccessResponsePayload::ObservationRead(ObservationReadPage {
            observed_sample: 4_736,
            rows: observation_rows.as_slice().into(),
        })
    );
    success!(
        "response.transport_get",
        SuccessResponsePayload::TransportGetSnapshot(TransportSnapshot {
            state: TransportState::Stopped,
            effective_sample: SampleTime(0)
        })
    );
    success!(
        "response.transport_set",
        SuccessResponsePayload::TransportSetSnapshot(TransportSnapshot {
            state: TransportState::Playing,
            effective_sample: SampleTime(9)
        })
    );
    success!(
        "response.telemetry",
        SuccessResponsePayload::TelemetryConfiguration(TelemetryConfiguration {
            meter_window_blocks: core::num::NonZeroU32::new(12),
            ..telemetry.clone()
        })
    );
    success!(
        "response.counters",
        SuccessResponsePayload::CounterSnapshot(CounterSnapshot {
            observed_sample: SampleTime(0),
            values: Vec::new()
        })
    );
    success!(
        "response.diagnostics",
        SuccessResponsePayload::DiagnosticsPage(DiagnosticsPage {
            last_sequence: 0,
            eof: true,
            diagnostics: Vec::new()
        })
    );
    for status in [
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
    ] {
        let payload = NonOkResponse {
            diagnostics: Vec::new(),
            omitted_diagnostics: 0,
            backpressure: (status == StatusCode::Backpressure).then_some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: 1,
                occupancy: 0,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        };
        push_non_ok(
            &mut frames,
            &codec,
            status,
            TypedNonOkResponseFrame {
                request_id: request,
                revision,
                message_id: MessageId::CapabilitiesGet,
                status,
                payload: &payload,
            },
        );
    }
    let meter = [MeterRecord {
        handle: 1,
        component: MeterComponent::Left,
        flags: 1,
        value: 0.0,
    }];
    let diagnostic = Diagnostic {
        code: "protocol.conformance".to_owned(),
        severity: DiagnosticSeverity::Error,
        path: Vec::new(),
        detail: None,
        operation_index: None,
        sample_time: None,
        provider_sequence: Some(1),
    };
    macro_rules! event {
        ($name:literal, $payload:expr) => {
            push_event(
                &mut frames,
                &codec,
                $name,
                TypedEventFrame {
                    revision,
                    payload: $payload,
                },
            );
        };
    }
    event!(
        "event.session_committed",
        EventPayload::SessionCommitted(SessionCommitted {
            event_sequence: 1,
            origin_request_id: request,
            previous_revision: SessionRevision(6),
            applied_operations: 1
        })
    );
    event!(
        "event.transport_state",
        EventPayload::TransportState(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Stopped,
            effective_sample: SampleTime(0),
            origin_request_id: None
        })
    );
    event!(
        "event.observation_ended",
        EventPayload::ObservationEnded(&ObservationEnded {
            event_sequence: 2,
            // Two keys of one commit: a console slot, then an insert (console sorts first).
            keys: vec![
                ObservationTapKey {
                    strip_id: "vocal".to_owned(),
                    section: ObservationSection::Console,
                    effect_id: "glue".to_owned(),
                    tap_id: 3,
                },
                ObservationTapKey {
                    strip_id: "vocal".to_owned(),
                    section: ObservationSection::Inserts,
                    effect_id: "comp".to_owned(),
                    tap_id: 1,
                },
            ],
        })
    );
    event!(
        "event.meter_batch",
        EventPayload::MeterBatch(MeterBatch {
            observed_sample: SampleTime(3072),
            first_sample: SampleTime(1536),
            window_sequence: 1,
            records: &meter
        })
    );
    event!(
        "event.counter_snapshot",
        EventPayload::CounterSnapshot(CounterSnapshotRef {
            observed_sample: SampleTime(0),
            values: &[]
        })
    );
    event!("event.diagnostic", EventPayload::Diagnostic(&diagnostic));
    // #1578: a ready `RESPONSE_READY` with one side and a member of each section (the insert
    // without a linear response), and a refused one with its diagnostic.
    let member = |section, effect: &str, kind, enabled_left| ResponseMember {
        section,
        effect: effect.to_owned(),
        kind,
        bypassed: false,
        enabled_left,
        enabled_right: 0b01,
        section_count: 2,
    };
    let ready = ResponseReady {
        sequence: 3,
        request_tag: 77,
        strip_id: "vocal".to_owned(),
        outcome: ResponseReadyOutcome::Ready(TrackResponseCapture {
            captured_sample: 4_096,
            sample_rate_hz: 48_000,
            floor_db: -120.0,
            frequencies_hz: vec![20.0, 632.455_5, 20_000.0],
            left_db: Some(vec![0.0, 1.5, -3.0]),
            right_db: None,
            excluded_count: 1,
            members: vec![
                member(
                    ResponseMemberSection::Builtins,
                    "miso.input-filters",
                    ResponseMemberKind::InputFilters,
                    0b11,
                ),
                member(
                    ResponseMemberSection::Console("bus_eq".to_owned()),
                    "miso.parametric-eq",
                    ResponseMemberKind::ParametricEq,
                    0b10,
                ),
                ResponseMember {
                    bypassed: true,
                    section_count: 0,
                    enabled_left: 0,
                    enabled_right: 0,
                    ..member(
                        ResponseMemberSection::Inserts("comp".to_owned()),
                        "miso.compressor",
                        ResponseMemberKind::Unavailable,
                        0,
                    )
                },
            ],
        }),
    };
    event!("event.response_ready", EventPayload::ResponseReady(&ready));
    event!(
        "event.response_ready_refused",
        EventPayload::ResponseReady(&ResponseReady {
            sequence: 4,
            request_tag: 78,
            strip_id: "drums".to_owned(),
            outcome: ResponseReadyOutcome::Refused {
                status: StatusCode::NotFound,
                diagnostic: "response.strip.removed".to_owned(),
            },
        })
    );
    frames
}

/// One control list page that carries every kind of row and every optional field once -- an
/// owner header, a route owner header, two section headers, and a control row of each `kind`
/// and `class` -- and the state page of its control rows.
fn control_corpus_pages() -> (ControlListPage, ControlStatePage) {
    let address = |section, slot: Option<&str>, control, lane| ControlAddress {
        owner_kind: ControlOwnerKind::Strip,
        owner_id: Some("vocal".to_owned()),
        section,
        slot_id: slot.map(str::to_owned),
        control,
        lane,
    };
    let row = |name: &str, kind, class| ControlRow {
        name: name.to_owned(),
        display_name: "Ratio".to_owned(),
        display_unit: ":1".to_owned(),
        unit: ParameterUnit::Ratio,
        kind,
        minimum: 1.0,
        maximum: 20.0,
        disabled_value: None,
        default: 4.0,
        taper: ControlTaper::Logarithmic,
        step: 0.5,
        precision: 1,
        ladder: vec![2.0, 4.0, 8.0],
        choices: Vec::new(),
        ramp_row: None,
        default_ramp_samples: 0,
        class,
        reason: None,
        reason_issue: None,
        reopen: None,
        scope: ControlScope::Model,
        automatable: class == ControlClass::Live,
        family: ControlFamily::Effect,
        active: true,
        value: 4.0,
        automated: false,
    };
    let mut live = row("ratio", ControlKind::Continuous, ControlClass::Live);
    live.disabled_value = Some(0.0);
    live.ramp_row = Some(ControlRampRow::Fader);
    live.default_ramp_samples = 480;
    // #1650 D4: the corpus carries an automated row.
    live.automated = true;
    let mut toggle = row("bypass", ControlKind::Toggle, ControlClass::Live);
    toggle.minimum = 0.0;
    toggle.maximum = 1.0;
    toggle.default = 0.0;
    toggle.value = 1.0;
    toggle.taper = ControlTaper::Stepped;
    toggle.ladder = Vec::new();
    let mut choice = row(
        "detector",
        ControlKind::Enumeration,
        ControlClass::RebuildOnly,
    );
    choice.choices = vec![
        EnumChoice {
            value: 0.0,
            label: "Peak".to_owned(),
        },
        EnumChoice {
            value: 1.0,
            label: "RMS".to_owned(),
        },
    ];
    choice.default = 0.0;
    choice.value = 1.0;
    choice.minimum = 0.0;
    choice.maximum = 1.0;
    choice.taper = ControlTaper::Stepped;
    choice.reason = Some(ControlReason::NewMemory);
    choice.reopen = Some("a detector that shares one buffer".to_owned());
    let mut ruled = row(
        "lookahead",
        ControlKind::Continuous,
        ControlClass::RebuildOnly,
    );
    ruled.reason = Some(ControlReason::RuledLiveNotBuilt);
    ruled.reason_issue = Some(1557);
    ruled.reopen = Some("#1557".to_owned());
    ruled.family = ControlFamily::Route;
    let mut derived = row(
        "effective_fader",
        ControlKind::Continuous,
        ControlClass::Derived,
    );
    derived.scope = ControlScope::Monitor;
    derived.active = false;
    derived.unit = ParameterUnit::Db;
    let controls = [live, toggle, choice, ruled, derived];
    let mut entries = vec![
        ControlEntry {
            handle: 1,
            address: address(ControlSection::Own, None, None, None),
            body: ControlBody::OwnerHeader(ControlOwnerHeader {
                kind: ControlOwnerHeaderKind::Track,
                name: "Vocal".to_owned(),
                route: None,
            }),
        },
        ControlEntry {
            handle: 2,
            address: ControlAddress {
                owner_kind: ControlOwnerKind::Route,
                owner_id: Some("vocal-main".to_owned()),
                section: ControlSection::Own,
                slot_id: None,
                control: None,
                lane: None,
            },
            body: ControlBody::OwnerHeader(ControlOwnerHeader {
                kind: ControlOwnerHeaderKind::Route,
                name: "vocal-main".to_owned(),
                route: Some(ControlRouteEnds {
                    source: "vocal".to_owned(),
                    destination: "main".to_owned(),
                    tap: SendTap::PostFader,
                }),
            }),
        },
        ControlEntry {
            handle: 3,
            address: address(ControlSection::Builtins, None, None, None),
            body: ControlBody::SectionHeader(ControlSectionHeader {
                name: "Builtins".to_owned(),
                effect: None,
            }),
        },
        ControlEntry {
            handle: 4,
            address: address(ControlSection::Inserts, Some("comp"), None, None),
            body: ControlBody::SectionHeader(ControlSectionHeader {
                name: "Compressor".to_owned(),
                effect: Some("miso.compressor".to_owned()),
            }),
        },
    ];
    let mut records = Vec::new();
    for (index, control) in controls.into_iter().enumerate() {
        let handle = 5 + u32::try_from(index).expect("five controls");
        records.push(ControlStateRecord {
            handle,
            value: control.value,
        });
        entries.push(ControlEntry {
            handle,
            address: address(
                ControlSection::Inserts,
                Some("comp"),
                Some(u32::try_from(index).expect("five controls") + 1),
                Some(ParameterChannel::Left),
            ),
            body: ControlBody::Control(control),
        });
    }
    (
        ControlListPage {
            revision: 7,
            structure_epoch: 3,
            last_handle: 9,
            eof: true,
            entries,
        },
        ControlStatePage {
            revision: 7,
            structure_epoch: 3,
            last_handle: 9,
            eof: true,
            records,
        },
    )
}

fn output() -> Vec<u8> {
    vec![0; 65_536]
}
fn push_command(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedCommandFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("conformance command encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Command,
    });
}
fn push_success(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedSuccessResponseFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("conformance response encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Response,
    });
}
fn push_non_ok(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    status: StatusCode,
    frame: TypedNonOkResponseFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("conformance error encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name: status_name(status),
        bytes,
        decoder: ConformanceDecoder::Response,
    });
}
fn push_event(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedEventFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_event_frame_into(&frame, &mut bytes)
        .expect("conformance event encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Event,
    });
}
fn status_name(status: StatusCode) -> &'static str {
    match status {
        StatusCode::Ok => "response.ok",
        StatusCode::MalformedFrame => "response.malformed_frame",
        StatusCode::UnsupportedVersion => "response.unsupported_version",
        StatusCode::UnsupportedMessage => "response.unsupported_message",
        StatusCode::UnknownRequiredField => "response.unknown_required_field",
        StatusCode::InvalidField => "response.invalid_field",
        StatusCode::LimitExceeded => "response.limit_exceeded",
        StatusCode::RevisionConflict => "response.revision_conflict",
        StatusCode::RevisionExhausted => "response.revision_exhausted",
        StatusCode::RequestIdReuse => "response.request_id_reuse",
        StatusCode::ReplayExpired => "response.replay_expired",
        StatusCode::Backpressure => "response.backpressure",
        StatusCode::ValidationFailed => "response.validation_failed",
        StatusCode::NotFound => "response.not_found",
        StatusCode::Unavailable => "response.unavailable",
        StatusCode::PcmForbidden => "response.pcm_forbidden",
        StatusCode::Internal => "response.internal",
    }
}
