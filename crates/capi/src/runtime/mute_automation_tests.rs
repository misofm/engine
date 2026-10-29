//! #1651 T5 on the C ABI: stored mute automation (builtin row 6) through the exported entries
//! (`miso_engine_v1_submit_command`, the source submit, the session seek and the render, through
//! `ffi`'s test wrappers).
//!
//! The session: tracks `t` and `u`, each on its own source and routed to the output, with no
//! console, no insert and no send; VCA `v` holds `t`. `t`'s mute is driven by a step curve:
//! unmuted, muted over a span whose ends are not grid samples, unmuted again. The session mute
//! length is `mute_ms` 10, `M = 480` at 48 kHz. Every check compares the output with plans
//! prepared with `t` statically unmuted and muted, fed the same PCM.
//!
//! This file tests two things on the C ABI:
//! - Gate 3 (composition): solo, a VCA mute and a hand mute (OQ1 = C) compose with the curve.
//!   `solo_vca_and_hand_mutes_compose_with_the_curve` is the only test that checks the solo and
//!   VCA mute terms composing with a stored curve against a static-plan oracle; host-core covers
//!   them only in pieces (the hand mute in
//!   `a_hand_mute_on_an_automated_lane_composes_with_the_curve`). The host-web twin was cut, so
//!   this test is also the C ABI's parity row for the composed stored mute.
//! - Root ruling R18-2: the control list marks a `mute` row an entry drives `automated`.
//!
//! The owner tests of gate 1 are `crates/host-core/tests/stored_mute_automation.rs`'s
//! `a_mute_step_mutes_the_lane_at_its_exact_sample_over_the_mute_length` (exact samples) and
//! `a_seek_that_leaves_the_mute_unchanged_lets_the_unmute_ramp_finish` (a seek into a muted span),
//! on every bank width and the scalar oracle. `hosts/host-web/tests/control_parity.rs`'s
//! `a_composed_stored_mute_renders_alike_on_both_hosts` compares the two hosts.

use core::ops::Range;

use super::tests::{SESSION as FIXTURE, boxed_c_children_with_limits, command_c_capacity, limits};
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

/// The host the script runs on.
type Host = CAbi;

/// The C ABI leg at any quantum: a session and plan compiled from the document.
struct CAbi {
    session: *mut crate::Session,
    plan: *mut crate::Plan,
    quantum: u64,
    revision: u64,
    request: u64,
    /// The next block's render sample, and the source generation and frame it is fed from.
    at: u64,
    generation: u64,
    start: u64,
}

impl Drop for CAbi {
    fn drop(&mut self) {
        crate::ffi::test_plan_destroy(self.plan);
        crate::ffi::test_session_destroy(self.session);
    }
}

impl Leg for CAbi {
    fn boot(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical session");
        let (session, plan) = boxed_c_children_with_limits(
            &document,
            crate::CompileLimits {
                source_ring_frames: model.quantum_frames * 4,
                maximum_vcas: 4,
                ..limits()
            },
        );
        Self {
            session,
            plan,
            quantum: u64::from(model.quantum_frames),
            revision: model.revision,
            request: 0,
            at: 0,
            generation: 1,
            start: 0,
        }
    }

    fn quantum(&self) -> u64 {
        self.quantum
    }

    fn at(&self) -> u64 {
        self.at
    }

    fn step(&mut self) -> Vec<u32> {
        let frames = u32::try_from(self.quantum).expect("quantum");
        for (feed, name) in ["src-t", "src-u"].into_iter().enumerate() {
            let [left, right] = chunk(feed as u64, self.start, self.quantum);
            let planes = [left.as_ptr(), right.as_ptr()];
            let chunk = crate::SourceChunk {
                struct_size: crate::SOURCE_CHUNK_SIZE,
                sample_rate_hz: RATE,
                generation: self.generation,
                start_frame: self.start,
                planes: planes.as_ptr(),
                plane_count: 2,
                frames,
                end_of_region: 0,
                reserved0: 0,
            };
            let mut report = crate::SubmitReport {
                struct_size: crate::SUBMIT_REPORT_SIZE,
                reserved0: 0,
                accepted_frames: 0,
                cumulative_written_frames: 0,
                active_generation: 0,
            };
            assert_eq!(
                crate::ffi::test_source_submit(self.session, name.as_bytes(), &chunk, &mut report),
                crate::RESULT_OK,
                "C ABI source {name}"
            );
            assert_eq!(report.accepted_frames, self.quantum);
        }
        let mut pcm = vec![f32::NAN; 2 * self.quantum as usize];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames,
            plane_stride_samples: frames,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(self.plan, self.at, &output),
            crate::RESULT_OK,
            "C ABI render"
        );
        self.at += self.quantum;
        self.start += self.quantum;
        bits(&pcm)
    }

    fn seek(&mut self, timeline_frame: u64) {
        self.generation += 1;
        assert_eq!(
            crate::ffi::test_session_seek(self.session, self.generation, timeline_frame, None),
            crate::RESULT_OK,
            "C ABI session seek"
        );
        self.start = timeline_frame;
    }

    fn command(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
        self.request += 1;
        let bytes = frame(self.request, payload, expected);
        let (result, length, mut response) = command_c_capacity(self.session, &bytes, 16 << 10);
        assert_eq!(result, crate::RESULT_OK, "C ABI command");
        response.truncate(usize::try_from(length).expect("response length"));
        // Drain the reliable lane, so a later commit is never backpressured by an unread event.
        loop {
            let mut event = [0_u8; 4_096];
            let mut out = crate::BytesOut {
                struct_size: crate::BYTES_OUT_SIZE,
                reserved0: 0,
                data: event.as_mut_ptr(),
                capacity_bytes: event.len() as u64,
                required_bytes: 0,
            };
            assert_eq!(
                crate::ffi::test_dequeue_event(self.session, crate::EVENT_LANE_RELIABLE, &mut out),
                crate::RESULT_OK
            );
            if out.required_bytes == 0 {
                break;
            }
        }
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
// The script: the session, the curve and the oracle checks that the tests below run on the C ABI
// host (`Host`).
// ---------------------------------------------------------------------------------------------

const RATE: u32 = 48_000;
/// The session mute length: `mute_ms` 10 at 48 kHz.
const M: u64 = 480;
/// Every run's length in samples; the sources are longer.
const END: u64 = 5_000;
const FADER_DB: u32 = 5;
const MUTE: u32 = 6;
/// The own-row ID of a strip's `solo`, a monitor row.
const SOLO: u32 = 2;
/// The own-row ID of a VCA's `mute`.
const VCA_MUTE: u32 = 2;

/// One host's session as the script drives it: booted on a document, fed one quantum of both
/// sources per block (`src-t` and `src-u`, from [`chunk`]) and rendered.
trait Leg: Sized {
    fn boot(model: &SessionModel) -> Self;
    fn quantum(&self) -> u64;
    /// The render sample of the next block.
    fn at(&self) -> u64;
    /// Feed and render the next block: the output's bits, left plane then right.
    fn step(&mut self) -> Vec<u32>;
    /// A plain session seek to `timeline_frame`, applied at the next block; later blocks are fed
    /// from there under a new generation.
    fn seek(&mut self, timeline_frame: u64);
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

/// Source `feed` (`0` for `t`, `1` for `u`), lane `lane`, source frame `frame`: deterministic and
/// never zero, but for `u`'s stretches of `-0.0`; both of `t`'s lanes are negative in every
/// fourth stretch of 300 frames, so a sign of zero of `t`'s muted lane reaches the output.
fn sample(feed: u64, lane: u64, frame: u64) -> f32 {
    if feed == 1 && (frame / 250) % 5 == 3 {
        return -0.0;
    }
    let mut z = (feed + 1)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(lane.wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add(frame.wrapping_mul(0x94D0_49BB_1331_11EB));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let value = ((z >> 40) as f32 / (1_u64 << 24) as f32 - 0.5).clamp(-0.5, 0.5);
    let value = if value == 0.0 { 0.25 } else { value };
    if feed == 0 && (frame / 300) % 4 == 1 {
        -value.abs()
    } else {
        value
    }
}

/// Both lanes of source `feed` from source frame `start`.
fn chunk(feed: u64, start: u64, frames: u64) -> [Vec<f32>; 2] {
    [0, 1].map(|lane| {
        (0..frames)
            .map(|offset| sample(feed, lane, start + offset))
            .collect()
    })
}

fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
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

/// Which static plan a span of render samples must equal.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Static {
    Open,
    Closed,
}

/// What the output must be: spans equal to a static plan, and samples that differ from one (where
/// a ramp starts, the first sample that differs from the static plan the lane leaves; or late in a
/// ramp, a sample that has not yet reached the static plan the lane moves to).
#[derive(Default)]
struct Expect {
    equal: Vec<(Range<u64>, Static)>,
    differ: Vec<(u64, Static)>,
}

/// The first block boundary at or after `sample`.
fn boundary(sample: u64, quantum: u64) -> u64 {
    sample.div_ceil(quantum) * quantum
}

/// Check one block's bits (`at` its first render sample) against the static plans' blocks.
fn check(
    label: &str,
    expect: &Expect,
    at: u64,
    quantum: u64,
    out: &[u32],
    open: &[u32],
    closed: &[u32],
) {
    let pick = |which: Static| match which {
        Static::Open => open,
        Static::Closed => closed,
    };
    for offset in 0..quantum {
        let sample = at + offset;
        let index = offset as usize;
        let planes = [index, index + quantum as usize];
        for (span, which) in &expect.equal {
            if span.contains(&sample) {
                let reference = pick(*which);
                assert_eq!(
                    planes.map(|i| out[i]),
                    planes.map(|i| reference[i]),
                    "{label}: sample {sample} equals the {which:?} static plan"
                );
            }
        }
        for (start, which) in &expect.differ {
            if *start == sample {
                let reference = pick(*which);
                assert_ne!(
                    planes.map(|i| out[i]),
                    planes.map(|i| reference[i]),
                    "{label}: sample {sample} differs from the {which:?} static plan (a ramp)"
                );
            }
        }
    }
}

/// Run `leg` and two static plans on the same host (`open`: `t` unmuted, `closed`: `t` muted)
/// to [`END`], fed the same PCM: before the block at render sample `at`, `before(at, leg)` may
/// edit, and may return a session seek's timeline frame (mirrored on the static plans); every
/// block is checked against `expect`.
fn run<L: Leg>(
    label: &str,
    leg: &mut L,
    expect: &Expect,
    mut before: impl FnMut(u64, &mut L) -> Option<u64>,
) {
    let quantum = u32::try_from(leg.quantum()).expect("quantum");
    let mut open = L::boot(&model(quantum, None, false));
    let mut closed = L::boot(&model(quantum, None, true));
    while leg.at() < END {
        let at = leg.at();
        if let Some(frame) = before(at, leg) {
            leg.seek(frame);
            open.seek(frame);
            closed.seek(frame);
        }
        let out = leg.step();
        check(
            label,
            expect,
            at,
            leg.quantum(),
            &out,
            &open.step(),
            &closed.step(),
        );
    }
}

/// A mute term of `t` the curve does not drive, set on or off as one transaction.
#[derive(Clone, Copy, Debug)]
enum Term {
    /// Solo on `u` (a monitor row: `t` is muted by solo).
    Solo,
    /// VCA `v`'s mute.
    Vca,
    /// `t`'s own (hand) mute, through `SET_CONTROL` (OQ1 = C).
    Hand,
}

impl Term {
    fn edit(self, on: bool) -> SessionEdit {
        let (owner, section, control) = match self {
            Self::Solo => (Owner::Strip(id("u")), Section::Own, SOLO),
            Self::Vca => (Owner::Vca(id("v")), Section::Own, VCA_MUTE),
            Self::Hand => (Owner::Strip(id("t")), Section::Builtins, MUTE),
        };
        SessionEdit::SetControl {
            expected_effect: None,
            address: ControlAddress {
                owner,
                section,
                control,
                lane: ParameterChannel::Both,
            },
            value: f32::from(u8::from(on)),
            ramp_samples: None,
        }
    }

    fn path(self) -> EditPath {
        match self {
            Self::Solo => EditPath::Monitor,
            Self::Vca | Self::Hand => EditPath::Live,
        }
    }
}

/// #1651 gate 3 and gate 4's live case. `t`'s curve closes at 2,000 and opens at 3,500; quantum
/// 128. Each of solo on `u`, a VCA mute of `t` and a hand mute of `t` (OQ1 = C):
///
/// - set at 256 while the curve is open, it mutes `t` (the muted static plan from the ramp's end);
/// - released at 2,560, while the curve is closed, `t` stays muted, and the curve's opening at
///   3,500 ramps the lane open;
/// - released at 1,024, while the curve is open, the lane ramps open at 1,024, then the curve
///   closes it at 2,000 and opens it at 3,500;
/// - never released, `t` stays muted through the curve's opening.
///
/// Every edit commits with its own path: solo `monitor`, the VCA and the hand mute `live` (on the
/// browser, through the Worker's apply).
///
/// Test value: red if solo, a VCA mute or a hand mute overwrites the curve instead of composing
/// with it (a release while closed would open the lane), if a curve step overwrites the term (the
/// opening at 3,500 would unmute a lane the term holds), or if a term on an automated lane is
/// dropped (`model_only`).
#[test]
fn solo_vca_and_hand_mutes_compose_with_the_curve() {
    const QUANTUM: u32 = 128;
    let q = u64::from(QUANTUM);
    let closed = 2_000..3_500;
    let on_at = 2 * q;
    for term in [Term::Solo, Term::Vca, Term::Hand] {
        for release in [Some(20 * q), Some(8 * q), None] {
            let label = format!("{term:?}, released at {release:?}");
            let mut leg = Host::boot(&model(
                QUANTUM,
                Some(curve(ParameterChannel::Both, closed.clone())),
                false,
            ));
            let mut expect = Expect {
                equal: vec![(0..on_at, Static::Open)],
                differ: vec![(on_at, Static::Open)],
            };
            let settled = boundary(on_at + M, q);
            match release {
                // Held through the curve's opening: closed to the end.
                None => expect.equal.push((settled..END, Static::Closed)),
                // Released while the curve is closed: closed until the curve opens.
                Some(at) if closed.contains(&at) => {
                    expect.equal.push((settled..closed.end, Static::Closed));
                    expect.differ.push((closed.end, Static::Closed));
                    expect
                        .equal
                        .push((boundary(closed.end + M, q)..END, Static::Open));
                }
                // Released while the curve is open: open again, then the curve's own steps.
                Some(at) => {
                    expect.equal.push((settled..at, Static::Closed));
                    expect.differ.push((at, Static::Closed));
                    expect
                        .equal
                        .push((boundary(at + M, q)..closed.start, Static::Open));
                    expect.differ.push((closed.start, Static::Open));
                    expect
                        .equal
                        .push((boundary(closed.start + M, q)..closed.end, Static::Closed));
                    expect.differ.push((closed.end, Static::Closed));
                    expect
                        .equal
                        .push((boundary(closed.end + M, q)..END, Static::Open));
                }
            }
            let mut revision = leg.revision();
            run(&label, &mut leg, &expect, |at, leg| {
                let on = if at == on_at {
                    true
                } else if Some(at) == release {
                    false
                } else {
                    return None;
                };
                // Solo is a monitor write: the revision does not advance (C8).
                if term.path() == EditPath::Live {
                    revision += 1;
                }
                assert_eq!(
                    transaction(leg, &[term.edit(on)]),
                    (term.path(), revision),
                    "{label}: the term at {at} commits with its path"
                );
                None
            });
        }
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
            headers_only: false,
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
        let mut hand = Term::Hand.edit(true);
        if let SessionEdit::SetControl { address, .. } = &mut hand {
            address.lane = ParameterChannel::Left;
        }
        assert_eq!(
            transaction(&mut leg, &[hand]).0,
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
