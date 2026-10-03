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
        SessionEdit::SetConsole { console },
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
        SessionEdit::SetTrackBuiltins {
            track_id: track_id.clone(),
            builtins: track.builtins.clone(),
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
        SessionEdit::SetEffectBypass {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            bypass: effect.bypass,
        },
        SessionEdit::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            link_mode: effect.link_mode,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            sidechain: effect.sidechain.clone(),
        },
        SessionEdit::UpsertEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            param: effect.params[0].clone(),
        },
        SessionEdit::RemoveEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            parameter_id: effect.params[0].parameter_id,
            channel: effect.params[0].channel,
        },
        SessionEdit::SetTrackFader {
            track_id: track_id.clone(),
            fader: track.fader.clone(),
        },
        SessionEdit::SetTrackMatrixOrPan {
            track_id: track_id.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        },
        SessionEdit::SetTrackConsole {
            track_id: track_id.clone(),
            console: track_console,
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
        SessionEdit::SetRouteChannelMatrix {
            route_id: route.id.clone(),
            channel_matrix: route.channel_matrix.clone(),
        },
        SessionEdit::SetRouteGainDb {
            route_id: route.id.clone(),
            gain_db: route.gain_db,
        },
        // #1216: the route's on/off switch, encoded on (the upserted route carries field 6 off).
        SessionEdit::SetRouteMute {
            route_id: route.id.clone(),
            mute: true,
        },
        // #1218: the follow switch, encoded on (the upserted route carries field 7 off). A codec
        // value: a store would refuse it on this output route, and the corpus is never applied.
        SessionEdit::SetRouteFollowsMute {
            route_id: route.id.clone(),
            follows_mute: true,
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
        SessionEdit::SetVcaFader {
            vca_id: id("drums-vca"),
            fader: session::DualMonoFader {
                left_db: 3.0,
                right_db: -1.5,
                left_mute: true,
                right_mute: false,
            },
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
        params: insert.params,
    }];
    session
}

/// One wire conformance row for decision 12's retired and refused codes (#1094).
///
/// `bytes` is one complete `SESSION_TRANSACTION_APPLY` command frame, request ID 1 at exact
/// revision 7. An endpoint whose session is [`console_session_fixture`] answers it with `status`
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
/// - the retired rack codes `1` (`simd1`) and `3` (`simd2`) in every rack-addressed edit and in an
///   automation target, refused at decode (`MALFORMED_FRAME`), never read as another rack;
/// - the retired track fields 6 (`simd1`) and 8 (`simd2`), mandatory (`UNKNOWN_REQUIRED_FIELD`)
///   and optional (`MALFORMED_FRAME`, never skipped as an unknown optional field);
/// - each structural or declaration edit addressed at `console` (`VALIDATION_FAILED`,
///   `session.edit.console_slot_fixed`).
///
/// The last row, `control.unknown_optional_track_field_12`, is the splice's control: the same
/// frame surgery with a never-allocated optional field ID decodes and commits (`OK`), so a
/// retired-field row cannot pass on a frame the splice broke.
#[must_use]
pub fn retired_code_rows() -> Vec<RetiredCodeRow> {
    const MALFORMED_FRAME: u16 = StatusCode::MalformedFrame as u16;
    const UNKNOWN_REQUIRED_FIELD: u16 = StatusCode::UnknownRequiredField as u16;
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
        patch_rack_code(&mut bytes, &[1, 2, 2, 2], code);
        rows.push(RetiredCodeRow {
            name: format!("retired.rack_code_{code}.set_automation_target"),
            bytes,
            status: MALFORMED_FRAME,
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
        if matches!(
            edit,
            SessionEdit::SetEffectBypass { .. }
                | SessionEdit::UpsertEffectParam { .. }
                | SessionEdit::RemoveEffectParam { .. }
        ) {
            // A track's console knobs: edited, not refused.
            continue;
        }
        rows.push(RetiredCodeRow {
            name: format!("refused.console.{name}"),
            bytes: transaction_frame(edit),
            status: VALIDATION_FAILED,
            diagnostic: Some("session.edit.console_slot_fixed"),
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

/// The eleven rack-addressed session edits, `0x0204`-`0x020e`, each naming `rack_name` and
/// `effect` on `track_id`.
fn rack_addressed_edits(
    track_id: &StableId,
    rack_name: RackName,
    effect: &session::Effect,
) -> Vec<(&'static str, SessionEdit)> {
    let track_id = track_id.clone();
    let effect_id = effect.id.clone();
    let param = session::EffectParam {
        parameter_id: 1,
        channel: session::ParameterChannel::Both,
        unit: session::ParameterUnit::Db,
        value: -1.0,
    };
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
            "set_effect_bypass",
            SessionEdit::SetEffectBypass {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                bypass: true,
            },
        ),
        (
            "set_effect_link_mode",
            SessionEdit::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                link_mode: LinkMode::Maximum,
            },
        ),
        (
            "set_effect_sidechain",
            SessionEdit::SetEffectSidechain {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                sidechain: session::SidechainDeclaration::None,
            },
        ),
        (
            "upsert_effect_param",
            SessionEdit::UpsertEffectParam {
                track_id: track_id.clone(),
                rack_name,
                effect_id: effect_id.clone(),
                param: param.clone(),
            },
        ),
        (
            "remove_effect_param",
            SessionEdit::RemoveEffectParam {
                track_id,
                rack_name,
                effect_id,
                parameter_id: param.parameter_id,
                channel: param.channel,
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
pub const COMPLETE_SCHEMA_HASH: u64 = 0xab35_7b6c_432f_9755;

/// Build every command, successful response, registered non-OK status, event, and all-opcode
/// session transaction using only public typed encoder entry points.
#[must_use]
pub fn complete_schema_corpus() -> Vec<ConformanceFrame> {
    let codec = ProtocolCodec::default();
    let request = RequestId::new(1).expect("literal request ID");
    let revision = SessionRevision(7);
    let mut frames = Vec::new();
    let transaction_edits = complete_all_opcode_fixture();
    let state = ParameterStateRequest { handles: vec![1] };
    let automation = [AutomationRecord {
        kind: AutomationKind::Point,
        handle: ParameterHandle(1),
        start: SampleTime(1),
        end: SampleTime(1),
        start_value: 0.0,
        end_value: 0.0,
    }];
    let telemetry = TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
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
                            | CommandPayload::ParameterMetadataGet(_)
                            | CommandPayload::ParameterStateGet(_)
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
        "command.parameter_metadata_get",
        CommandPayload::ParameterMetadataGet(ParameterMetadataRequest {
            after_handle: 0,
            limit: 1
        })
    );
    command!(
        "command.parameter_state_get",
        CommandPayload::ParameterStateGet(&state)
    );
    command!(
        "command.automation_enqueue",
        CommandPayload::AutomationEnqueue(AutomationEnqueue {
            records: &automation
        })
    );
    command!("command.transport_get", CommandPayload::TransportGet);
    command!(
        "command.transport_set",
        CommandPayload::TransportSet(TransportSetRequest {
            state: TransportState::Playing,
            position: Some(SampleTime(9))
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

    let capabilities = Capabilities {
        minimum_version: ProtocolVersion::V1,
        maximum_version: ProtocolVersion::V1,
        maximum_frame_bytes: 4096,
        maximum_tlvs: 1024,
        maximum_string_bytes: 1024,
        maximum_nesting: 4,
        maximum_automation_records: 256,
        control_command_slots: 1,
        control_command_bytes: 64,
        automation_batch_slots: 1,
        reliable_response_slots: 1,
        reliable_event_slots: 1,
        telemetry_slots: 1,
        replay_entries: 1,
        replay_bytes: 64,
        maximum_cached_response_bytes: 64,
        per_block_automation_density: 1,
        admission_quantum_frames: 1,
        maximum_parameter_page_items: 256,
        maximum_diagnostic_page_items: 256,
        maximum_telemetry_handles: 256,
        maximum_transaction_edits: 64,
        supported_commands: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        supported_events: &[0x8001, 0x8002, 0x8010, 0x8020, 0x8021, 0x8030],
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
        SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
            applied_operations: 42
        })
    );
    success!(
        "response.parameter_metadata",
        SuccessResponsePayload::ParameterMetadata(ParameterMetadataPage {
            last_handle: 0,
            eof: true,
            descriptors: Vec::new()
        })
    );
    success!(
        "response.parameter_state",
        SuccessResponsePayload::ParameterState(ParameterStatePage {
            observed_sample: 0,
            records: Vec::new()
        })
    );
    success!(
        "response.automation",
        SuccessResponsePayload::AutomationEnqueued(AutomationEnqueued {
            accepted_records: 1,
            occupancy: 0,
            capacity: 1,
            generation: 1
        })
    );
    success!(
        "response.transport_get",
        SuccessResponsePayload::TransportGetSnapshot(TransportSnapshot {
            state: TransportState::Stopped,
            position: SampleTime(0),
            effective_sample: SampleTime(0)
        })
    );
    success!(
        "response.transport_set",
        SuccessResponsePayload::TransportSetSnapshot(TransportSnapshot {
            state: TransportState::Playing,
            position: SampleTime(9),
            effective_sample: SampleTime(9)
        })
    );
    success!(
        "response.telemetry",
        SuccessResponsePayload::TelemetryConfiguration(telemetry.clone())
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
        StatusCode::TimeInPast,
        StatusCode::AutomationOrder,
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
        "event.automation_canceled",
        EventPayload::AutomationCanceled(AutomationCanceled {
            event_sequence: 1,
            origin_request_id: request,
            canceled_records: 1,
            reason: AutomationCancellationReason::RevisionChanged,
            queue_generation: 1,
            effective_sample: None
        })
    );
    event!(
        "event.transport_state",
        EventPayload::TransportState(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Stopped,
            position: SampleTime(0),
            effective_sample: SampleTime(0),
            origin_request_id: None
        })
    );
    event!(
        "event.meter_batch",
        EventPayload::MeterBatch(MeterBatch {
            observed_sample: SampleTime(0),
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
    frames
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
        StatusCode::TimeInPast => "response.time_in_past",
        StatusCode::AutomationOrder => "response.automation_order",
        StatusCode::PcmForbidden => "response.pcm_forbidden",
        StatusCode::Internal => "response.internal",
    }
}
