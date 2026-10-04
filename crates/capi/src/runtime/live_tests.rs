//! Value-only track fader, mute and pan transactions on the running C ABI plan (#1257 gates 1-4).
//!
//! Every edit goes through `miso_engine_v1_submit_command`. The source is fed one quantum per block
//! for the whole run, with no zero sample and distinct lanes, and never seeks except where a test
//! rebuilds the plan on purpose.

use super::tests::{
    SESSION, boxed_c_children, command_bytes, command_bytes_at_revision, command_c, event_c,
    generated_parity_session, limits, submit_c,
};
use super::*;
use protocol::{ExpectedRevision, SessionEdit, SessionRevision};
use session::{
    DualMonoFader, MatrixOrPan, RouteDestination, RouteSource, SessionModel, StableId, Submix, Vca,
    parse_session_json,
};

const SOURCE: &[u8] = b"fixture-source";
/// Long enough for every run below.
const SOURCE_FRAMES: u64 = 96 * 128;

/// One lane of the source: never zero, and different on the left and the right.
fn source_sample(lane: usize, frame: u64) -> f32 {
    let wobble = ((frame * if lane == 0 { 7_919 } else { 104_729 }) % 101) as f32 / 101.0;
    if lane == 0 {
        0.2 + 0.1 * wobble
    } else {
        -0.15 - 0.1 * wobble
    }
}

fn source_chunk(start: u64, frames: usize) -> [Vec<f32>; 2] {
    [0, 1].map(|lane| {
        (0..frames as u64)
            .map(|offset| source_sample(lane, start + offset))
            .collect()
    })
}

/// The parity session with a source long enough for a whole run.
fn long_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&generated_parity_session(track_count, sample_rate_hz))
        .expect("parity session");
    model.sources[0].frames = SOURCE_FRAMES;
    session::canonical_session_json(&model).expect("canonical long session")
}

fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
}

/// A C ABI session and plan, fed and rendered one quantum per block.
struct Rig {
    session: *mut crate::Session,
    plan: *mut crate::Plan,
    rate: u32,
    quantum: usize,
    /// The next block to render.
    block: u64,
    generation: u64,
    /// The next source frame to submit.
    fed: u64,
    request: u64,
}

impl Rig {
    fn new(document: &str) -> Self {
        let (session, plan) = boxed_c_children(document);
        let model = parse_session_json(document).expect("rig session");
        Self {
            session,
            plan,
            rate: model.sample_rate_hz,
            quantum: model.quantum_frames as usize,
            block: 0,
            generation: 1,
            fed: 0,
            request: 1_000,
        }
    }

    fn summary(&self) -> (u64, usize, u64, usize) {
        crate::ffi::test_session_state_summary(self.session)
    }

    fn latency(&self) -> u32 {
        u32::try_from(crate::ffi::test_plan_snapshot(self.plan).1.latency_samples)
            .expect("latency fits")
    }

    /// Submits the next quantum of the source, then renders one block.
    fn step(&mut self) -> Vec<f32> {
        let [left, right] = source_chunk(self.fed, self.quantum);
        submit_c(
            self.session,
            self.generation,
            self.fed,
            self.rate,
            &left,
            &right,
            false,
        );
        self.fed += self.quantum as u64;
        self.render()
    }

    fn render(&mut self) -> Vec<f32> {
        let mut pcm = vec![f32::NAN; self.quantum * 2];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: self.quantum as u32,
            plane_stride_samples: self.quantum as u32,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(self.plan, self.block * self.quantum as u64, &output),
            crate::RESULT_OK
        );
        self.block += 1;
        pcm
    }

    /// Restarts the source at frame 0 under a new generation, as a host does after a rebuild.
    fn seek_to_start(&mut self) {
        self.generation += 1;
        assert_eq!(
            crate::ffi::test_source_seek(self.session, SOURCE, self.generation, 0),
            crate::RESULT_OK
        );
        self.fed = 0;
    }

    fn next_request(&mut self) -> u64 {
        self.request += 1;
        self.request
    }

    /// Applies one transaction; drains the reliable lane after a success.
    fn apply(&mut self, edits: &[SessionEdit]) -> u32 {
        let bytes = self.transaction(edits);
        self.send(&bytes).0
    }

    /// The bytes of one transaction at the committed revision, under a new request ID.
    fn transaction(&mut self, edits: &[SessionEdit]) -> Vec<u8> {
        let request = self.next_request();
        command_bytes_at_revision(
            request,
            ExpectedRevision::Exact(SessionRevision(self.summary().0)),
            protocol::CommandPayload::SessionTransactionApply(edits),
        )
    }

    /// Sends raw command bytes; drains the reliable lane after a success.
    fn send(&mut self, bytes: &[u8]) -> (u32, Vec<u8>) {
        let (result, response) = command_c(self.session, bytes);
        if result == crate::RESULT_OK {
            self.drain_reliable();
        }
        (result, response)
    }

    fn drain_reliable(&mut self) {
        loop {
            let (result, frame) = event_c(self.session, crate::EVENT_LANE_RELIABLE);
            assert_eq!(result, crate::RESULT_OK);
            if frame.is_empty() {
                break;
            }
        }
    }

    fn last_error(&self) -> Vec<u8> {
        crate::ffi::test_last_error(self.session)
    }

    /// The committed session, paged through `SessionSnapshotGet`.
    fn snapshot(&mut self) -> String {
        let mut json = Vec::new();
        loop {
            let request = self.next_request();
            let bytes = command_bytes(
                request,
                protocol::CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                    offset: json.len() as u64,
                    maximum_bytes: 2_048,
                }),
            );
            let (result, response) = command_c(self.session, &bytes);
            assert_eq!(result, crate::RESULT_OK);
            let mut fields = [0_u16; 64];
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(page),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("snapshot response")
            else {
                panic!("expected a snapshot page")
            };
            json.extend_from_slice(page.canonical_json_chunk);
            if page.eof {
                break;
            }
        }
        String::from_utf8(json).expect("UTF-8 snapshot")
    }

    fn model(&mut self) -> SessionModel {
        parse_session_json(&self.snapshot()).expect("committed snapshot")
    }

    /// The free fader and matrix room of a track's producer in the current or the pending epoch.
    fn room(&self, epoch: Epoch, track_id: &str) -> (usize, usize) {
        let snapshot = crate::ffi::test_transaction_snapshot(self.session);
        let epoch = match epoch {
            Epoch::Current => snapshot.provider_epoch,
            Epoch::Pending => *snapshot
                .pending_provider_epochs
                .last()
                .expect("a pending candidate"),
        };
        snapshot
            .live_rooms
            .iter()
            .find(|(row_epoch, id, ..)| *row_epoch == epoch && &**id == track_id)
            .map(|&(_, _, fader, matrix)| (fader, matrix))
            .expect("track producer")
    }

    /// Every producer's room in the current epoch.
    fn current_rooms(&self) -> Vec<(usize, usize)> {
        let snapshot = crate::ffi::test_transaction_snapshot(self.session);
        snapshot
            .live_rooms
            .iter()
            .filter(|(epoch, ..)| *epoch == snapshot.provider_epoch)
            .map(|&(_, _, fader, matrix)| (fader, matrix))
            .collect()
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        crate::ffi::test_plan_destroy(self.plan);
        crate::ffi::test_session_destroy(self.session);
    }
}

#[derive(Clone, Copy)]
enum Epoch {
    Current,
    Pending,
}

/// A lanes-free `host_core::prepare_host_runtime` plan of one document, fed the same source.
struct Reference {
    plan: PreparedRenderPlan,
    sources: SourceControlSet,
    rate: u32,
    quantum: usize,
    block: u64,
}

impl Reference {
    fn new(document: &str) -> Self {
        let caps = prepare_caps(limits());
        let compiled = host_core::compile_host_session(document, &caps).expect("reference compile");
        let host_core::PreparedHost { plan, sources, .. } =
            host_core::prepare_host_runtime(&compiled, &caps).expect("reference prepare");
        Self {
            plan,
            sources,
            rate: compiled.sample_rate().0,
            quantum: compiled.quantum().0 as usize,
            block: 0,
        }
    }

    fn step(&mut self) -> Vec<f32> {
        let start = self.block * self.quantum as u64;
        let [left, right] = source_chunk(start, self.quantum);
        self.sources
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: start,
                    sample_rate_hz: self.rate,
                    planes: &[&left, &right],
                    frames: self.quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("reference submit");
        let mut pcm = vec![f32::NAN; self.quantum * 2];
        self.plan
            .render_contiguous(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut pcm, 2, self.quantum, self.quantum)
                        .expect("reference output"),
                },
                start,
            )
            .expect("reference render");
        self.block += 1;
        pcm
    }

    /// Renders blocks `0..blocks` and returns them all.
    fn run(document: &str, blocks: u64) -> Vec<Vec<f32>> {
        let mut reference = Self::new(document);
        (0..blocks).map(|_| reference.step()).collect()
    }
}

/// K: the blocks after the edit's first block within which the edit may still be settling.
fn window(latency_samples: u32, smoothing_samples: u32, quantum: usize) -> u64 {
    u64::from(latency_samples + smoothing_samples).div_ceil(quantum as u64) + 1
}

fn fader_edit(track_id: &str, fader: DualMonoFader) -> SessionEdit {
    SessionEdit::SetTrackFader {
        track_id: StableId::parse(track_id).expect("track ID"),
        fader,
    }
}

fn mute_all(model: &SessionModel, muted: bool) -> Vec<SessionEdit> {
    model
        .tracks
        .iter()
        .map(|track| {
            fader_edit(
                track.id.as_str(),
                DualMonoFader {
                    left_mute: muted,
                    right_mute: muted,
                    ..track.fader.clone()
                },
            )
        })
        .collect()
}

/// The source content edit that every test uses to force a rebuild.
fn content_edit(model: &SessionModel) -> SessionEdit {
    let source = &model.sources[0];
    SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content: format!("blake3:{}", "ab".repeat(32)),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    }
}

/// Renders past the edit's window and returns the blocks from E + K on, with their block indices.
fn settle(rig: &mut Rig, k: u64, compared: u64) -> Vec<(u64, Vec<f32>)> {
    let edit_block = rig.block;
    while rig.block < edit_block + k {
        rig.step();
    }
    (0..compared).map(|_| (rig.block, rig.step())).collect()
}

/// Gate 1: mute, unmute, and fader plus pan/matrix edits change the running plan with no rebuild.
fn live_pcm_shape(track_count: usize, sample_rate_hz: u32) {
    let label = format!("{track_count} tracks at {sample_rate_hz} Hz");
    let document = long_session(track_count, sample_rate_hz);
    let mut rig = Rig::new(&document);
    let mut original = Reference::new(&document);
    let quantum = rig.quantum;
    let latency = rig.latency();
    let model = parse_session_json(&document).expect("model");

    for _ in 0..4 {
        assert_eq!(
            bits(&rig.step()),
            bits(&original.step()),
            "{label}: warm-up"
        );
    }

    // (a) Mute every track.
    let (revision, _, epoch, pending) = rig.summary();
    assert_eq!(
        rig.apply(&mute_all(&model, true)),
        crate::RESULT_OK,
        "{label}"
    );
    assert_eq!(
        rig.summary(),
        (revision + 1, rig.summary().1, epoch, pending),
        "{label}: the mute commits one revision and prepares no plan"
    );
    assert_eq!(pending, 0, "{label}");
    let k = window(latency, 0, quantum);
    let muted = settle(&mut rig, k, 3);
    for (block, pcm) in &muted {
        assert!(
            pcm.iter().all(|sample| sample.to_bits() == 0),
            "{label}: block {block} after the mute is exactly +0.0"
        );
    }
    // The compared window is not vacuous: the unmuted reference carries signal in every block of
    // it.
    while original.block < rig.block {
        let block = original.block;
        let reference = original.step();
        if muted.iter().any(|(muted_block, _)| *muted_block == block) {
            assert!(
                reference.iter().any(|sample| *sample != 0.0),
                "{label}: block {block}: the reference carries signal in the mute window"
            );
        }
    }

    // (b) Unmute: bit-identical to the original session's lanes-free plan.
    assert_eq!(rig.apply(&mute_all(&model, false)), crate::RESULT_OK);
    let edit_block = rig.block;
    let k = window(latency, 0, quantum);
    while rig.block < edit_block + k + 3 {
        let block = rig.block;
        let live = rig.step();
        let reference = original.step();
        if block >= edit_block + k {
            assert!(
                reference.iter().any(|sample| *sample != 0.0),
                "{label}: block {block}: the reference carries signal"
            );
            assert_eq!(
                bits(&live),
                bits(&reference),
                "{label}: unmuted block {block}"
            );
        }
    }

    // (c) Fader -6 dB left and +3 dB right on one track; an asymmetric matrix on another.
    let fader_track = model.tracks[0].id.as_str();
    let matrix_track = model.tracks[model.tracks.len() - 1].id.as_str();
    let smoothing = 16;
    assert!(smoothing as usize <= quantum);
    let edits = [
        fader_edit(
            fader_track,
            DualMonoFader {
                left_db: -6.0,
                right_db: 3.0,
                left_mute: false,
                right_mute: false,
            },
        ),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse(matrix_track).expect("matrix track"),
            matrix_or_pan: MatrixOrPan::Matrix {
                ll: 0.75,
                lr: 0.125,
                rl: -0.25,
                rr: 0.5,
                smoothing_samples: smoothing,
            },
        },
    ];
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
    let (after, _, after_epoch, after_pending) = rig.summary();
    assert_eq!(
        (after, after_epoch, after_pending),
        (revision + 1, epoch, 0)
    );
    let committed = rig.snapshot();
    let settled = settle(&mut rig, window(latency, smoothing, quantum), 3);
    let reference = Reference::run(&committed, rig.block);
    for (block, live) in settled {
        let expected = &reference[block as usize];
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "{label}: block {block}: the reference carries signal"
        );
        assert_eq!(bits(&live), bits(expected), "{label}: edited block {block}");
    }
    // No edit above rebuilt: the plan the rig compiled is still the one rendering.
    assert_eq!(rig.summary().2, 0, "{label}: the first epoch still renders");
}

#[test]
fn live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        for track_count in [1, 10] {
            live_pcm_shape(track_count, sample_rate_hz);
        }
    }
}

/// Gate 2: a live mute is in the committed model, so a later rebuild keeps it.
fn persistence_run(live_mute: bool) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let model = parse_session_json(&document).expect("model");
    for _ in 0..3 {
        rig.step();
    }
    if live_mute {
        assert_eq!(rig.apply(&mute_all(&model, true)), crate::RESULT_OK);
        assert_eq!(rig.summary().3, 0, "the mute is live");
        let committed = rig.model();
        assert!(
            committed
                .tracks
                .iter()
                .all(|track| track.fader.left_mute && track.fader.right_mute),
            "SessionSnapshotGet returns the live mute"
        );
        rig.step();
    }
    assert_eq!(rig.apply(&[content_edit(&model)]), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "a content edit prepares a candidate");
    let snapshot = rig.snapshot();
    // The render call swaps the candidate in; its fresh rings then wait for a seek.
    rig.render();
    rig.seek_to_start();
    let rebuilt: Vec<_> = (0..6).map(|_| rig.step()).collect();
    (rebuilt, Reference::run(&snapshot, 6))
}

#[test]
fn a_live_mute_survives_a_later_rebuild() {
    let (rebuilt, reference) = persistence_run(true);
    assert_eq!(
        rebuilt.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>(),
        reference.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>(),
        "the rebuilt plan renders the final snapshot"
    );
    assert!(
        rebuilt.iter().flatten().all(|sample| *sample == 0.0),
        "the rebuilt plan is muted"
    );

    let (control, control_reference) = persistence_run(false);
    assert_eq!(
        control.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>(),
        control_reference
            .iter()
            .map(|pcm| bits(pcm))
            .collect::<Vec<_>>(),
        "the control rebuild renders its snapshot"
    );
    assert!(
        control.iter().flatten().any(|sample| *sample != 0.0),
        "without the live mute the rebuild carries signal"
    );
}

fn left_db_edit(track_id: &str, left_db: f32) -> SessionEdit {
    fader_edit(
        track_id,
        DualMonoFader {
            left_db,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
    )
}

/// The parts of the session state a refused live edit must leave alone.
fn refusal_state(rig: &Rig) -> (u64, Vec<u8>, usize, protocol::QueueReport) {
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    (
        snapshot.revision,
        snapshot.canonical,
        snapshot.replay_entries,
        snapshot.reliable_event,
    )
}

/// Gate 3 (a): a full lane is typed backpressure before anything changes, and a retry after one
/// render succeeds. The room check's matrix term and its fader record count are each exercised
/// too: a full matrix lane, and a two-record fader edit against room for one.
#[test]
fn a_full_live_lane_refuses_before_anything_changes() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let depth = LIVE_QUEUE_DEPTH.get();
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth));
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&[left_db_edit("eq0", -1.0 - edit as f32)]),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(rig.room(Epoch::Current, "eq0"), (0, depth));
    let before = refusal_state(&rig);
    assert_eq!(
        rig.apply(&[left_db_edit("eq0", -20.0)]),
        crate::RESULT_BACKPRESSURE
    );
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!(refusal_state(&rig), before, "the refusal changed nothing");
    assert_eq!(rig.room(Epoch::Current, "eq0"), (0, depth));
    assert_eq!(rig.summary().3, 0, "the refusal prepared no plan");

    rig.step();
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth));
    assert_eq!(rig.apply(&[left_db_edit("eq0", -20.0)]), crate::RESULT_OK);
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth - 1, depth));

    // The matrix term: sixteen pan edits fill eq3's matrix lane, and the seventeenth refuses.
    let pan = |edit: usize| SessionEdit::SetTrackMatrixOrPan {
        track_id: StableId::parse("eq3").expect("eq3"),
        matrix_or_pan: MatrixOrPan::Pan {
            left: -0.5 + edit as f32 / 64.0,
            right: 0.5,
            smoothing_samples: 0,
        },
    };
    for edit in 0..depth {
        assert_eq!(rig.apply(&[pan(edit)]), crate::RESULT_OK, "pan {edit}");
    }
    assert_eq!(rig.room(Epoch::Current, "eq3"), (depth, 0));
    let before = (refusal_state(&rig), rig.current_rooms());
    assert_eq!(rig.apply(&[pan(depth)]), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!(
        (refusal_state(&rig), rig.current_rooms()),
        before,
        "a full matrix lane refuses before anything changes"
    );
    assert_eq!(rig.summary().3, 0, "the refusal prepared no plan");

    // The fader term counts records, not edits: with room for one record, an edit whose two lanes
    // move to different dB values (two records) refuses.
    for edit in 0..depth - 1 {
        assert_eq!(
            rig.apply(&[left_db_edit("eq4", -1.0 - edit as f32)]),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(rig.room(Epoch::Current, "eq4"), (1, depth));
    let before = (refusal_state(&rig), rig.current_rooms());
    let two_records = fader_edit(
        "eq4",
        DualMonoFader {
            left_db: -40.0,
            right_db: -30.0,
            left_mute: false,
            right_mute: false,
        },
    );
    assert_eq!(rig.apply(&[two_records]), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!(
        (refusal_state(&rig), rig.current_rooms()),
        before,
        "two records against room for one refuse before anything changes"
    );
    assert_eq!(rig.room(Epoch::Current, "eq4"), (1, depth));
}

/// Gate 3 (b): a transaction is all or nothing across tracks.
#[test]
fn a_live_transaction_with_one_full_lane_pushes_to_no_lane() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let depth = LIVE_QUEUE_DEPTH.get();
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&[left_db_edit("eq1", -1.0 - edit as f32)]),
            crate::RESULT_OK
        );
    }
    let before = refusal_state(&rig);
    // The delta is in canonical track order, eq0 before the full eq1: an arm that pushed each
    // strip as soon as its own room checked would leave eq0's records behind.
    let edits = [
        left_db_edit("eq1", -30.0),
        left_db_edit("eq0", -3.0),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse("eq0").expect("eq0"),
            matrix_or_pan: MatrixOrPan::Pan {
                left: 0.5,
                right: 0.25,
                smoothing_samples: 16,
            },
        },
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!(refusal_state(&rig), before);
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth));
    assert_eq!(rig.room(Epoch::Current, "eq1"), (0, depth));
}

/// Gate 3 (c): an exact replay of a live edit returns the cached response and pushes nothing.
#[test]
fn a_replayed_live_edit_pushes_nothing() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let edits = [left_db_edit("eq2", -4.5)];
    let bytes = rig.transaction(&edits);
    let (result, first) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    let room = rig.room(Epoch::Current, "eq2");
    assert_eq!(room.0, LIVE_QUEUE_DEPTH.get() - 1);
    let revision = rig.summary().0;
    let (result, replay) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(replay, first, "the replay is the cached response");
    assert_eq!(rig.room(Epoch::Current, "eq2"), room);
    assert_eq!(rig.summary().0, revision);
}

/// Gate 3 (d): a live edit while a candidate is pending goes to the candidate, which renders it.
#[test]
fn a_live_edit_while_a_candidate_is_pending_reaches_the_candidate() {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let model = parse_session_json(&document).expect("model");
    rig.step();
    assert_eq!(rig.apply(&[content_edit(&model)]), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "a candidate is pending");
    let depth = LIVE_QUEUE_DEPTH.get();
    assert_eq!(rig.room(Epoch::Pending, "eq3"), (depth, depth));

    let edits = [
        fader_edit(
            "eq3",
            DualMonoFader {
                left_db: -6.0,
                right_db: 3.0,
                left_mute: false,
                right_mute: true,
            },
        ),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse("eq3").expect("eq3"),
            matrix_or_pan: MatrixOrPan::Pan {
                left: 0.5,
                right: 0.75,
                smoothing_samples: 8,
            },
        },
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "still the one candidate");
    // Two fader records (left and right dB differ), one mute record (right only), one matrix.
    assert_eq!(rig.room(Epoch::Pending, "eq3"), (depth - 3, depth - 1));
    assert_eq!(rig.room(Epoch::Current, "eq3"), (depth, depth));

    let snapshot = rig.snapshot();
    rig.render();
    rig.seek_to_start();
    // The candidate ramps its pan from its prepared value; the reference starts at the target.
    let k = window(rig.latency(), 8, rig.quantum);
    let blocks = k + 3;
    let rendered: Vec<_> = (0..blocks).map(|_| rig.step()).collect();
    let reference = Reference::run(&snapshot, blocks);
    let unedited = Reference::run(
        &{
            let mut model = model.clone();
            model.sources[0] = parse_session_json(&snapshot).expect("final").sources[0].clone();
            session::canonical_session_json(&model).expect("pre-edit candidate")
        },
        blocks,
    );
    let mut differs = false;
    for block in k as usize..blocks as usize {
        let expected = &reference[block];
        assert!(expected.iter().any(|sample| *sample != 0.0));
        assert_eq!(
            bits(&rendered[block]),
            bits(expected),
            "candidate block {block}"
        );
        differs |= bits(&unedited[block]) != bits(expected);
    }
    assert!(
        differs,
        "the edit is audible against the candidate's own values"
    );
}

fn assert_rebuilds(document: &str, edits: &[SessionEdit], label: &str) {
    let mut rig = Rig::new(document);
    rig.step();
    let (revision, _, epoch, pending) = rig.summary();
    assert_eq!((epoch, pending), (0, 0), "{label}");
    assert_eq!(rig.apply(edits), crate::RESULT_OK, "{label}");
    let (after, _, _, pending) = rig.summary();
    assert_eq!(
        (after, pending),
        (revision + 1, 1),
        "{label}: a rebuild prepares a candidate"
    );
    let depth = LIVE_QUEUE_DEPTH.get();
    assert!(
        rig.current_rooms()
            .iter()
            .all(|&room| room == (depth, depth)),
        "{label}: the rendering plan got no record"
    );
}

fn with_model(document: &str, change: impl FnOnce(&mut SessionModel)) -> String {
    let mut model = parse_session_json(document).expect("model");
    change(&mut model);
    session::canonical_session_json(&model).expect("canonical variant")
}

fn submix_session(follows_mute: bool) -> String {
    with_model(&long_session(10, 48_000), |model| {
        let track = model.tracks[0].clone();
        let bus = StableId::parse("bus").expect("bus");
        // Nothing on the bus keeps state, so once the window has passed, a live edit upstream of
        // it renders bit-exactly like a plan prepared with the edit from sample 0: its filters
        // are off and its console EQ is bypassed. (With a stateful bus the two runs' bus filter
        // memories differ forever, which says nothing about the edit.)
        let mut builtins = track.builtins.clone();
        for lane in [&mut builtins.left, &mut builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
        let mut console = track.console.clone();
        for entry in &mut console {
            entry.bypass = true;
        }
        model.submixes.push(Submix {
            id: bus.clone(),
            builtins,
            console,
            inserts: session::Rack {
                effects: Vec::new(),
            },
            fader: track.fader.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        });
        let mut send = model.routes[0].clone();
        send.id = StableId::parse("eq0-bus").expect("send");
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        send.follows_mute = follows_mute;
        let mut out = model.routes[0].clone();
        out.id = StableId::parse("bus-main").expect("bus out");
        out.source = RouteSource::Submix {
            submix_id: bus,
            tap: session::SendTap::PostPan,
        };
        model.routes.push(send);
        model.routes.push(out);
    })
}

/// Gate 4: every delta outside the live set rebuilds, and a domain failure reaches no queue.
#[test]
fn deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing() {
    let base = long_session(10, 48_000);
    let quiet = DualMonoFader {
        left_db: -2.0,
        right_db: -2.0,
        left_mute: false,
        right_mute: false,
    };

    let vca = with_model(&base, |model| {
        model.vcas.push(Vca {
            id: StableId::parse("vca").expect("vca"),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: vec![StableId::parse("eq5").expect("member")],
        });
    });
    assert_rebuilds(&vca, &[fader_edit("eq0", quiet.clone())], "a VCA (G3)");

    let followed = submix_session(true);
    let muted = DualMonoFader {
        left_mute: true,
        ..parse_session_json(&followed).expect("model").tracks[0]
            .fader
            .clone()
    };
    assert_rebuilds(
        &followed,
        &[fader_edit("eq0", muted.clone())],
        "a followed mute (G2)",
    );
    // The same mute without `follows_mute` is live, so the guard above is what rebuilt.
    let unfollowed = submix_session(false);
    let mut rig = Rig::new(&unfollowed);
    rig.step();
    assert_eq!(rig.apply(&[fader_edit("eq0", muted)]), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "an unfollowed mute is live");
    drop(rig);

    assert_rebuilds(
        &unfollowed,
        &[fader_edit("bus", quiet)],
        "a submix fader (G1)",
    );

    let model = parse_session_json(&base).expect("model");
    assert_rebuilds(&base, &[content_edit(&model)], "a source content edit");

    // A fader outside the setter's domain is today's compile rejection, and pushes nothing.
    let mut rig = Rig::new(&base);
    rig.step();
    let before = refusal_state(&rig);
    assert_eq!(
        rig.apply(&[left_db_edit("eq0", 30.0)]),
        crate::RESULT_COMPILE_REJECTED
    );
    let rejected = with_model(&base, |model| model.tracks[0].fader.left_db = 30.0);
    let Err(failure) = compile_children(&rejected, limits()) else {
        panic!("a 30 dB fader does not prepare")
    };
    assert_eq!(
        rig.last_error(),
        failure.diagnostics,
        "the live arm reports today's preparation diagnostic"
    );
    assert_eq!(refusal_state(&rig), before);
    let depth = LIVE_QUEUE_DEPTH.get();
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth));
    assert_eq!(rig.summary().3, 0);
}

/// Every refusal before the push leaves the queues alone, including the commit predicate's slot.
#[test]
fn a_fault_before_the_live_push_leaves_every_queue_and_the_model_alone() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let before = refusal_state(&rig);
    crate::ffi::test_set_structural_faults(
        rig.session,
        [Some(TestStructuralFaultPhase::BeforeLivePush), None],
    );
    assert_eq!(
        rig.apply(&[left_db_edit("eq0", -9.0)]),
        crate::RESULT_BACKPRESSURE
    );
    assert_eq!(refusal_state(&rig), before);
    let depth = LIVE_QUEUE_DEPTH.get();
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth));
    // The same edit then commits.
    assert_eq!(rig.apply(&[left_db_edit("eq0", -9.0)]), crate::RESULT_OK);
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth - 1, depth));
}

/// Gate 1 on a session with a submix strip: the producer table holds the tracks, then the
/// submixes, and a live track edit there still reaches its track by ID and renders like a rebuild.
#[test]
fn a_live_track_edit_beside_a_submix_strip_reaches_its_track() {
    let document = submix_session(false);
    let mut rig = Rig::new(&document);
    let quantum = rig.quantum;
    let latency = rig.latency();
    let depth = LIVE_QUEUE_DEPTH.get();
    for _ in 0..3 {
        rig.step();
    }
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let strips: Vec<_> = snapshot
        .live_rooms
        .iter()
        .map(|(_, id, ..)| &**id)
        .collect();
    assert_eq!(
        strips.last(),
        Some(&"bus"),
        "the submix strip follows the tracks"
    );

    // The last track sits right before the submix in the strip table; eq0 feeds the submix.
    let smoothing = 12;
    let edits = [
        fader_edit(
            "eq9",
            DualMonoFader {
                left_db: -6.0,
                right_db: -6.0,
                left_mute: false,
                right_mute: false,
            },
        ),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse("eq0").expect("eq0"),
            matrix_or_pan: MatrixOrPan::Pan {
                left: 0.25,
                right: 0.875,
                smoothing_samples: smoothing,
            },
        },
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(rig.room(Epoch::Current, "eq9"), (depth - 1, depth));
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth - 1));
    assert_eq!(rig.room(Epoch::Current, "bus"), (depth, depth));

    let committed = rig.snapshot();
    let settled = settle(&mut rig, window(latency, smoothing, quantum), 3);
    let reference = Reference::run(&committed, rig.block);
    let original = Reference::run(&document, rig.block);
    for (block, live) in settled {
        let expected = &reference[block as usize];
        assert!(expected.iter().any(|sample| *sample != 0.0));
        assert_ne!(
            bits(expected),
            bits(&original[block as usize]),
            "block {block}: the edit is audible"
        );
        assert_eq!(bits(&live), bits(expected), "submix session block {block}");
    }
}

/// #1258 gate 4: with the two-slot reliable-event lane full, a live edit is the protocol's own
/// `Backpressure` response under `RESULT_OK`, decided before a token exists, so it reaches no lane.
///
/// Test value: red if the live arm pushes its records before the reliable-event capacity check (a
/// strip's room would drop with no commit), or if that check stops refusing a live transaction.
#[test]
fn a_live_edit_without_reliable_event_room_is_protocol_backpressure_and_pushes_nothing() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    // Two live edits, never dequeued: each commits one `SESSION_COMMITTED` event.
    for db in [-1.0, -2.0] {
        let bytes = rig.transaction(&[left_db_edit("eq0", db)]);
        assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    }
    let depth = LIVE_QUEUE_DEPTH.get();
    assert_eq!(rig.room(Epoch::Current, "eq0"), (depth - 2, depth));
    let rooms = crate::ffi::test_transaction_snapshot(rig.session).live_rooms;
    let (revision, ..) = rig.summary();

    let edits = [
        left_db_edit("eq1", -3.0),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse("eq2").expect("eq2"),
            matrix_or_pan: MatrixOrPan::Pan {
                left: 0.5,
                right: 0.25,
                smoothing_samples: 16,
            },
        },
    ];
    let bytes = rig.transaction(&edits);
    let (result, response) = command_c(rig.session, &bytes);
    assert_eq!(result, crate::RESULT_OK, "a protocol refusal is RESULT_OK");
    let mut fields = [0_u16; 64];
    let header = match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("refusal response")
    {
        protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
        protocol::DecodedTypedResponseFrame::Success { header, .. } => {
            panic!(
                "expected Backpressure, got success at revision {}",
                header.revision.0
            )
        }
    };
    assert_eq!(header.status, protocol::StatusCode::Backpressure);
    assert_eq!(rig.summary().0, revision, "the revision is unchanged");
    assert_eq!(rig.summary().3, 0, "no plan was prepared");
    assert_eq!(
        crate::ffi::test_transaction_snapshot(rig.session).live_rooms,
        rooms,
        "no strip queue's room changed"
    );

    // Once the events are dequeued, the same edit under a new request ID commits live.
    rig.drain_reliable();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().0, revision + 1);
    assert_eq!(rig.room(Epoch::Current, "eq1"), (depth - 1, depth));
    assert_eq!(rig.room(Epoch::Current, "eq2"), (depth, depth - 1));
}

/// #1257 hazard 1 (#1053 D7): the live arm adds no report row, so it skips the epoch-lag check
/// that refuses a rebuild inside a plan-swapping render call.
///
/// Modelled on `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`: the test
/// runs the two halves of `PlanState::render` separately and makes a value-only fader edit between
/// them, while the atomic still names the retired plan.
///
/// Test value: red if the lag check runs before (or inside) the live arm, which turns every live
/// edit inside the window into retryable backpressure; #1258's race gate retries that refusal and
/// so cannot see it.
#[test]
fn a_live_edit_inside_a_plan_swapping_render_call_commits_without_a_candidate() {
    let mut children = compile_children(SESSION, limits()).expect("children");
    let mut pcm = [0.0_f32; 256];
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("first block");
    let revision = children.session.controller.session().revision().0;
    let edit = protocol::SessionEdit::SetSessionId {
        session_id: StableId::parse("swap-window").expect("stable ID"),
    };
    let structural = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(revision)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    children
        .session
        .command(&structural, 4_096)
        .expect("structural command");
    assert!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("reliable event")
            .is_some()
    );
    let old_epoch = children.session.providers.epoch;
    let new_epoch = children.session.pending_providers[0].epoch;

    // First half of `PlanState::render`: the owner swaps in the candidate.
    let report = children
        .plan
        .owner
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
            },
            128,
        )
        .expect("swapping block");
    assert_eq!(report.swap, engine::realtime::SwapOutcome::Applied);
    assert_eq!(report.active_epoch.0, new_epoch);
    assert_eq!(
        children.plan.shared.active_epoch.load(Ordering::Acquire),
        old_epoch,
        "the window is open: the atomic still names the retired plan"
    );

    // Inside the window: a value-only fader edit on eq0.
    let live = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(revision + 1)),
        protocol::CommandPayload::SessionTransactionApply(&[left_db_edit("eq0", -7.5)]),
    );
    let result = children.session.command(&live, 4_096);
    let snapshot = children.session.test_transaction_snapshot();
    let depth = LIVE_QUEUE_DEPTH.get();
    let rooms: Vec<_> = snapshot
        .live_rooms
        .iter()
        .filter(|(_, id, ..)| &**id == "eq0")
        .map(|&(epoch, _, fader, matrix)| (epoch, fader, matrix))
        .collect();
    assert_eq!(
        (
            result
                .as_ref()
                .map(|_| ())
                .map_err(|error| format!("{error:?}")),
            snapshot.revision,
            snapshot.active_plan_epoch,
            snapshot.provider_epoch,
            snapshot.pending_provider_epochs,
            rooms,
        ),
        (
            Ok(()),
            revision + 2,
            old_epoch,
            new_epoch,
            Vec::new(),
            vec![(new_epoch, depth - 1, depth)],
        ),
        "the live edit commits inside the window, prepares no candidate, and takes one record \
         from the promoted provider's fader lane"
    );
}

/// The reliable lane's events, drained, by kind.
fn reliable_event_kinds(rig: &Rig) -> Vec<&'static str> {
    let mut kinds = Vec::new();
    loop {
        let (result, frame) = event_c(rig.session, crate::EVENT_LANE_RELIABLE);
        assert_eq!(result, crate::RESULT_OK);
        if frame.is_empty() {
            return kinds;
        }
        let mut fields = [0_u16; 64];
        let event = ProtocolCodec::default()
            .decode_typed_event(&frame, &mut DecodeScratch::new(&mut fields))
            .expect("reliable event");
        kinds.push(match event.payload {
            protocol::DecodedEventPayload::SessionCommitted(_) => "SESSION_COMMITTED",
            protocol::DecodedEventPayload::AutomationCanceled(_) => "AUTOMATION_CANCELED",
            _ => "other",
        });
    }
}

/// #1053 D10: a live edit emits exactly the events a rebuild's commit does -- one
/// `SESSION_COMMITTED`, and an `AUTOMATION_CANCELED` when an automation batch is queued.
///
/// Test value: red if a live edit commits through a path that skips the protocol commit's events,
/// for example a live-only commit that emits no `SESSION_COMMITTED` or leaves the queued
/// automation of the superseded revision uncanceled.
#[test]
fn a_live_edit_emits_the_commit_events_of_a_rebuild() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let bytes = rig.transaction(&[left_db_edit("eq0", -1.0)]);
    assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"]);

    // Queue one automation batch at the committed revision, then edit live again.
    let record = protocol::AutomationRecord {
        kind: protocol::AutomationKind::Point,
        handle: protocol::ParameterHandle(5),
        start: protocol::SampleTime(1 << 20),
        end: protocol::SampleTime(1 << 20),
        start_value: 100.0,
        end_value: 100.0,
    };
    let request = rig.next_request();
    let automation = command_bytes_at_revision(
        request,
        ExpectedRevision::Exact(SessionRevision(rig.summary().0)),
        protocol::CommandPayload::AutomationEnqueue(protocol::AutomationEnqueue {
            records: core::slice::from_ref(&record),
        }),
    );
    let (result, response) = command_c(rig.session, &automation);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    assert!(
        matches!(
            ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("automation response"),
            protocol::DecodedTypedResponseFrame::Success { .. }
        ),
        "the automation batch is queued"
    );
    assert_eq!(reliable_event_kinds(&rig), Vec::<&str>::new());
    let bytes = rig.transaction(&[left_db_edit("eq0", -2.0)]);
    assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(
        reliable_event_kinds(&rig),
        ["SESSION_COMMITTED", "AUTOMATION_CANCELED"]
    );
}
