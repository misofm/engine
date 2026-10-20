//! #1651 T5 on the browser host: stored mute automation (builtin row 6) through
//! `AudioWorkletEngineHost`, every edit a command frame through the entry the Worker's control
//! plane serves (#1382, #1562a D1). The bank widths (Simd8, Simd4 and the scalar oracle) are
//! `host-core`'s `stored_mute_automation.rs`.
//!
//! The session: tracks `t` and `u`, each on its own source and routed to the output, with no
//! console, no insert and no send; VCA `v` holds `t`. `t`'s mute is driven by a step curve:
//! unmuted, muted over a span whose ends are not grid samples, unmuted again.
//!
//! Gates 1 and 3 (exact samples, a seek, solo, VCA and hand mutes composed with the curve) are
//! `host-core`'s `stored_mute_automation.rs`; the browser's parity row is `control_parity.rs`'s
//! `a_composed_stored_mute_renders_alike_on_both_hosts`.
//!
//! - Root ruling R18-2: the control list marks a `mute` row an entry drives `automated`.

use core::ops::Range;

use host_web::{AudioWorkletEngineHost, RESULT_OK, WebBootOptions};
use protocol::{
    CommandPayload, ControlListRequest, ControlSection, DecodeScratch,
    DecodedSuccessResponsePayload, DecodedTypedResponseFrame, EditPath, ExpectedRevision,
    ProtocolCodec, RequestId, SessionEdit, SessionRevision, TypedCommandFrame,
};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ControlAddress,
    ControlSmoothing, DualMonoFader, MatrixOrPan, Owner, ParameterChannel, ParameterUnit, Section,
    SessionModel, StableId, Vca, canonical_session_json, parse_session_json,
};

mod common;

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// The host the script runs on.
type Host = Browser;

/// The browser leg at any quantum: the host booted on the document.
struct Browser {
    host: AudioWorkletEngineHost,
    revision: u64,
    request: u64,
}

impl Leg for Browser {
    fn boot(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical session");
        let options = WebBootOptions {
            require_sample_rate_hz: RATE,
            require_quantum_frames: model.quantum_frames,
            source_ring_frames: model.quantum_frames * 4,
            ..WebBootOptions::explicit_defaults()
        };
        let host = AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())));
        Self {
            revision: host.committed_revision(),
            host,
            request: 0,
        }
    }

    fn command(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
        self.request += 1;
        let bytes = frame(self.request, payload, expected);
        let length = u32::try_from(bytes.len()).expect("frame length");
        self.host
            .control_request_mut(length)
            .expect("the control request staging")[..bytes.len()]
            .copy_from_slice(&bytes);
        let result = self.host.control_command(length, None);
        let response = self.host.control_response().to_vec();
        assert_eq!(result, RESULT_OK, "{}", String::from_utf8_lossy(&response));
        // A commit queues its reliable events; the control half's drain reads them (#1570b D4).
        common::drain_events(&mut self.host);
        response
    }

    fn revision(&self) -> u64 {
        self.revision
    }

    fn set_revision(&mut self, revision: u64) {
        self.revision = revision;
    }
}

// ---------------------------------------------------------------------------------------------
// The session and the control-list reads.
// ---------------------------------------------------------------------------------------------

const RATE: u32 = 48_000;
/// The curve's length in samples; the sources are longer.
const END: u64 = 5_000;
const FADER_DB: u32 = 5;
const MUTE: u32 = 6;

/// One host's session: booted on a document and driven through its command entry.
trait Leg: Sized {
    fn boot(model: &SessionModel) -> Self;
    /// One command frame through the host's command entry; the response frame.
    fn command(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8>;
    /// The committed revision the last reply named.
    fn revision(&self) -> u64;
    fn set_revision(&mut self, revision: u64);
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// `t`'s mute curve: open, closed over `closed`, open again.
fn curve(lane: ParameterChannel, closed: Range<u64>) -> Automation {
    let step = |span: Range<u64>, value: f32| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample: span.start,
        end_sample: span.end,
        start_value: value,
        end_value: value,
        unit: ParameterUnit::Linear,
    };
    Automation {
        id: id("t-mute-curve"),
        target: ControlAddress {
            owner: Owner::Strip(id("t")),
            section: Section::Builtins,
            control: MUTE,
            lane,
        },
        segments: vec![
            step(0..closed.start, 0.0),
            step(closed.clone(), 1.0),
            step(closed.end..END + 1_000, 0.0),
        ],
        offset: AutomationOffset::default(),
    }
}

/// The session at quantum `quantum`, from the nine-track fixture: tracks `t` and `u` over
/// `src-t` and `src-u`, each to the output, with no console, no insert, no input filter and no
/// send; VCA `v` holds `t`; `t` statically muted when `muted`, and `entry` in the automation
/// table.
fn model(quantum: u32, entry: Option<Automation>, muted: bool) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("the nine-track fixture");
    model.sample_rate_hz = RATE;
    model.quantum_frames = quantum;
    model.control_smoothing = Some(ControlSmoothing {
        mute_ms: 10.0,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.automation.clear();
    let source = model.sources[0].clone();
    let track = model.tracks[0].clone();
    let route = model
        .routes
        .iter()
        .find(|route| {
            matches!(&route.source, session::RouteSource::Track { track_id, .. }
                if *track_id == track.id)
        })
        .expect("the first track's route")
        .clone();
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    for (name, db, pan) in [("t", -2.0, -0.3), ("u", -4.0, 0.4)] {
        let mut source = source.clone();
        source.id = id(&format!("src-{name}"));
        source.frames = 2 * END;
        let mut track = track.clone();
        track.id = id(name);
        track.source_id = source.id.clone();
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
        track.fader = DualMonoFader {
            left_db: db,
            right_db: db,
            left_mute: muted && name == "t",
            right_mute: muted && name == "t",
        };
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: pan,
            right: -pan,
            smoothing_samples: 0,
        };
        let mut route = route.clone();
        route.id = id(&format!("{name}-main"));
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
    }
    model.vcas.push(Vca {
        id: id("v"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id("t")],
    });
    model.automation.extend(entry);
    model
}

/// Encode one command frame.
fn frame(request: u64, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
    let mut bytes = vec![0_u8; 16 << 10];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: expected,
                payload,
            },
            &mut bytes,
        )
        .expect("command frame");
    bytes.truncate(length);
    bytes
}

/// `edits` as one transaction at the committed revision: the path and the revision the reply
/// names. A refusal panics with its diagnostic codes.
fn transaction<L: Leg>(leg: &mut L, edits: &[SessionEdit]) -> (EditPath, u64) {
    let revision = leg.revision();
    let reply = leg.command(
        CommandPayload::SessionTransactionApply(edits),
        ExpectedRevision::Exact(SessionRevision(revision)),
    );
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
        .expect("a transaction response")
    {
        DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
        } => {
            leg.set_revision(header.revision.0);
            (applied.edit_path, header.revision.0)
        }
        DecodedTypedResponseFrame::NonOk { payload, .. } => panic!(
            "the transaction is refused: {:?}",
            payload
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect::<Vec<_>>()
        ),
        DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
    }
}

/// `t`'s own (hand) mute on `lane`, through `SET_CONTROL` (OQ1 = C).
fn hand_mute(lane: ParameterChannel) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner: Owner::Strip(id("t")),
            section: Section::Builtins,
            control: MUTE,
            lane,
        },
        value: 1.0,
        ramp_samples: None,
    }
}

/// One `mute` or `fader_db` row of strip `owner` as the host lists it: its lane, the bits of its
/// value and its `automated` flag.
type Row = (Option<protocol::ParameterChannel>, u32, bool);

/// The builtin rows `control` of strip `owner`, paged through `CONTROL_LIST_GET`.
fn listed<L: Leg>(leg: &mut L, owner: &str, control: u32) -> Vec<Row> {
    let mut rows = Vec::new();
    let (mut epoch, mut after) = (0, 0);
    loop {
        let request = ControlListRequest {
            structure_epoch: epoch,
            after_handle: after,
            limit: 256,
            owner: None,
        };
        let bytes = leg.command(
            CommandPayload::ControlListGet(&request),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 4096];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ControlListPage(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("a control page")
        else {
            panic!("expected a control list page")
        };
        rows.extend(page.entries.iter().filter_map(|entry| match &entry.body {
            protocol::ControlBody::Control(row)
                if entry.address.owner_id.as_deref() == Some(owner)
                    && entry.address.section == ControlSection::Builtins
                    && entry.address.control == Some(control) =>
            {
                Some((entry.address.lane, row.value.to_bits(), row.automated))
            }
            _ => None,
        }));
        if page.eof {
            return rows;
        }
        epoch = page.structure_epoch;
        after = page.last_handle;
    }
}

/// Root ruling R18-2 (#1650 D4 for the mute row). A mute entry on `t`'s left lane marks `t`'s
/// left `mute` row `automated` and not its right one; an entry on `both` marks both. The row's
/// value stays the static (hand) mute, which a hand mute on the automated lane moves while the
/// row stays `automated`. `t`'s `fader_db` rows and `u`'s `mute` rows are not marked.
///
/// Test value: red if the flag stays fader-only (a client would offer a plain toggle on a lane the
/// curve drives), if it marks a lane no entry drives, or if the row lists the curve's value instead
/// of the static mute.
#[test]
fn the_list_marks_a_mute_row_an_entry_drives_automated() {
    use protocol::ParameterChannel::{Left, Right};
    let closed = 1_000..2_500;
    for (lane, expected) in [
        (ParameterChannel::Left, [true, false]),
        (ParameterChannel::Both, [true, true]),
    ] {
        let label = format!("an entry on {lane:?}");
        let mut leg = Host::boot(&model(128, Some(curve(lane, closed.clone())), false));
        let unmuted = 0.0_f32.to_bits();
        assert_eq!(
            listed(&mut leg, "t", MUTE),
            vec![
                (Some(Left), unmuted, expected[0]),
                (Some(Right), unmuted, expected[1]),
            ],
            "{label}: t's mute rows"
        );
        assert!(
            listed(&mut leg, "t", FADER_DB).iter().all(|row| !row.2),
            "{label}: t's fader rows are not automated"
        );
        assert!(
            listed(&mut leg, "u", MUTE).iter().all(|row| !row.2),
            "{label}: u's mute rows are not automated"
        );
        assert_eq!(
            transaction(&mut leg, &[hand_mute(ParameterChannel::Left)]).0,
            EditPath::Live,
            "{label}: a hand mute is live"
        );
        assert_eq!(
            listed(&mut leg, "t", MUTE),
            vec![
                (Some(Left), 1.0_f32.to_bits(), expected[0]),
                (Some(Right), unmuted, expected[1]),
            ],
            "{label}: the hand mute is the left row's value"
        );
    }
}
