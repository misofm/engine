//! Value-only track fader, mute and pan transactions on the running C ABI plan (#1257 gates 1-4).
//!
//! Every edit goes through `miso_engine_v1_submit_command`. The source is fed one quantum per block
//! for the whole run, with no zero sample and distinct lanes, and never seeks except where a test
//! rebuilds the plan on purpose.

use super::tests::{
    SESSION, boxed_c_children_with_limits, command_bytes, command_bytes_at_revision, command_c,
    event_c, generated_parity_session, limits, submit_c,
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
        Self::with_limits(document, limits())
    }

    fn with_limits(document: &str, limits: CompileLimits) -> Self {
        let (session, plan) = boxed_c_children_with_limits(document, limits);
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

/// How gate 2's run mutes every other track.
#[derive(Clone, Copy, PartialEq)]
enum Mute {
    /// Live, one block before the rebuild.
    Live,
    /// In the rebuild's own transaction.
    Baked,
    /// Never.
    Unmuted,
}

/// Gate 2 run: three blocks, every other track muted per `mute`, one more block, then a content
/// edit rebuilds; the swap, a seek, and six blocks, which it returns. Every run feeds the same
/// source over the same blocks and swaps at the same block, so whatever the swap carries (the
/// strip input sections, #1276) is the same in every run, and the rebuilt blocks differ only by
/// what the successor was prepared from.
fn persistence_run(mute: Mute) -> Vec<Vec<f32>> {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let model = parse_session_json(&document).expect("model");
    let muted: Vec<SessionEdit> = mute_all(&model, true).into_iter().step_by(2).collect();
    for _ in 0..3 {
        rig.step();
    }
    let mut structural = vec![content_edit(&model)];
    match mute {
        Mute::Live => {
            assert_eq!(rig.apply(&muted), crate::RESULT_OK);
            assert_eq!(rig.summary().3, 0, "the mute is live");
            let committed = rig.model();
            assert!(
                committed.tracks.iter().enumerate().all(|(index, track)| {
                    track.fader.left_mute == (index % 2 == 0)
                        && track.fader.right_mute == (index % 2 == 0)
                }),
                "SessionSnapshotGet returns the live mute"
            );
        }
        Mute::Baked => structural.extend(muted),
        Mute::Unmuted => {}
    }
    rig.step();
    assert_eq!(rig.apply(&structural), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "a content edit prepares a candidate");
    // The render call swaps the candidate in; its fresh ring then waits for a seek.
    rig.render();
    rig.seek_to_start();
    let rebuilt = (0..6).map(|_| rig.step()).collect();
    assert_eq!(
        crate::ffi::test_plan_carry_counts(rig.plan),
        (1, 0),
        "the swap carried the input sections, so a fresh compile is no oracle"
    );
    rebuilt
}

/// Gate 2: a live mute is in the committed model, so a later rebuild keeps it. Red if the plan a
/// later rebuild prepares does not follow the committed live mute (it renders like the unmuted
/// run, or unlike a rebuild that bakes the mute into its own transaction from the same history).
#[test]
fn a_live_mute_survives_a_later_rebuild() {
    let blocks = |run: Vec<Vec<f32>>| run.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>();
    let live = blocks(persistence_run(Mute::Live));
    let baked = blocks(persistence_run(Mute::Baked));
    let unmuted = blocks(persistence_run(Mute::Unmuted));
    assert_eq!(
        live, baked,
        "the rebuild after a live mute renders like a rebuild that bakes it"
    );
    assert!(
        live.iter()
            .flatten()
            .any(|&sample| f32::from_bits(sample) != 0.0),
        "the unmuted half plays"
    );
    assert_ne!(live, unmuted, "the mute is audible after the rebuild");
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

    rig.render();
    rig.seek_to_start();
    // The candidate ramps its pan from its prepared value; the control starts at the target.
    let k = window(rig.latency(), 8, rig.quantum);
    let blocks = k + 3;
    let rendered: Vec<_> = (0..blocks).map(|_| rig.step()).collect();
    // The controls take the same rebuild from the same history, so they carry what the candidate
    // carried (#1276); one bakes the edits into the rebuild's transaction, one leaves them out.
    let control = |baked: &[SessionEdit]| {
        let mut control = Rig::new(&document);
        control.step();
        let mut structural = vec![content_edit(&model)];
        structural.extend_from_slice(baked);
        assert_eq!(control.apply(&structural), crate::RESULT_OK);
        assert_eq!(control.summary().3, 1, "the control rebuilds");
        control.render();
        control.seek_to_start();
        (0..blocks).map(|_| control.step()).collect::<Vec<_>>()
    };
    let reference = control(&edits);
    let unedited = control(&[]);
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

/// A live fader edit beside a C ABI structural transaction whose successor carries the source
/// ring (#1053 merged into #1269): one track of a stateless session at 0 dB goes to -6 dB live,
/// and a muted track whose ID sorts first is added (`add_muted_track`), with no render between.
/// `live_first` sends the fader edit before the structural commit, so its record waits undrained
/// in the running plan's queue at the swap; otherwise after it, while the successor is pending, so
/// it reaches the successor's producer. From the swap block on, the plan renders -6 dB of the
/// input, bit-identical to the committed session compiled fresh (a stateless strip carries
/// nothing that a fresh plan does not start with) and fed the same source without a seek.
fn live_fader_across_a_carrying_swap(live_first: bool) {
    let label = if live_first {
        "live first"
    } else {
        "live second"
    };
    let mut model = super::tests::stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.sources[0].frames = SOURCE_FRAMES;
    let fader = model.tracks[0].fader.clone();
    assert_eq!((fader.left_db, fader.right_db), (0.0, 0.0));
    let document = session::canonical_session_json(&model).expect("canonical");
    let structural = super::tests::add_muted_track(&model, "a-muted");
    let live = [fader_edit(
        "eq0",
        DualMonoFader {
            left_db: -6.0,
            right_db: -6.0,
            ..fader
        },
    )];
    let depth = LIVE_QUEUE_DEPTH.get();
    let mut rig = Rig::new(&document);
    for _ in 0..4 {
        rig.step();
    }
    let swap = rig.block;
    if live_first {
        assert_eq!(rig.apply(&live), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 0, "{label}: the fader edit is live");
        assert!(rig.room(Epoch::Current, "eq0").0 < depth, "{label}");
        assert_eq!(rig.apply(&structural), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 1, "{label}: the transaction rebuilds");
        assert_eq!(
            rig.room(Epoch::Pending, "eq0"),
            (depth, depth),
            "{label}: the successor bakes -6 dB and inherits no record"
        );
    } else {
        assert_eq!(rig.apply(&structural), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 1, "{label}: the transaction rebuilds");
        assert_eq!(
            rig.apply(&live),
            crate::RESULT_OK,
            "{label}: the live edit reaches the pending successor"
        );
        assert_eq!(rig.summary().3, 1, "{label}: still the one candidate");
        assert!(rig.room(Epoch::Pending, "eq0").0 < depth, "{label}");
        assert_eq!(rig.room(Epoch::Current, "eq0"), (depth, depth), "{label}");
    }
    let committed = rig.model();
    let track = committed
        .tracks
        .iter()
        .find(|track| track.id.as_str() == "eq0")
        .expect("eq0");
    assert_eq!(
        (track.fader.left_db, track.fader.right_db),
        (-6.0, -6.0),
        "{label}: the readback is the live value"
    );
    let snapshot = rig.snapshot();
    let rendered: Vec<_> = (0..4).map(|_| rig.step()).collect();
    assert_eq!(
        crate::ffi::test_plan_carry_counts(rig.plan),
        (1, 0),
        "{label}: one swap, and it carried the source ring"
    );
    let reference = Reference::run(&snapshot, rig.block);
    let unity = Reference::run(&document, rig.block);
    // 10^(-6 / 20).
    let gain = 0.501_187_2_f32;
    for (offset, pcm) in rendered.iter().enumerate() {
        let block = (swap as usize) + offset;
        assert_eq!(
            bits(pcm),
            bits(&reference[block]),
            "{label}: block {block} renders the committed session"
        );
        // The fixture pans both lanes to one side, where they sum (and can nearly cancel); the
        // other side holds a residue near 1e-20. Neither near-zero ratio says anything.
        let mut audible = 0;
        for (sample, unity) in pcm.iter().zip(&unity[block]) {
            if unity.abs() < 1.0e-3 {
                continue;
            }
            audible += 1;
            assert!(
                (sample / unity - gain).abs() < 1.0e-5,
                "{label}: block {block} is -6 dB of the input: {sample} against {unity}"
            );
        }
        assert!(
            audible >= pcm.len() / 4,
            "{label}: block {block} carries signal"
        );
    }
}

/// Red if a live fader edit committed while a carrying successor is pending is not applied from
/// the successor's very first (swap) block: the pending-candidate test compares only later blocks,
/// so a successor that skips its lane drain on the swap block is caught here alone.
#[test]
fn a_live_fader_after_a_carrying_structural_commit_reaches_the_successor() {
    live_fader_across_a_carrying_swap(false);
}

/// Red if a live fader record still undrained in the retiring plan's queue at a carrying
/// structural commit reaches the successor's queue (it must arrive only baked into the committed
/// model the successor is prepared from), the "drain, then carry" rule #1277 D3 must keep.
#[test]
fn a_live_fader_left_undrained_before_a_carrying_structural_commit_is_baked() {
    live_fader_across_a_carrying_swap(true);
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
    let revision = children.session.test_controller().session().revision().0;
    let edit = content_edit(&parse_session_json(SESSION).expect("session"));
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
    let old_epoch = children.session.test_providers().test_epoch();
    let new_epoch = children.session.test_pending_providers()[0].test_epoch();

    // First half of `PlanState::render`: the owner swaps in the candidate.
    let report = children
        .plan
        .test_owner_mut()
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
        children.plan.test_active_epoch().load(Ordering::Acquire),
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

/// The four model-only edits of #1260 D1, each a change no prepared plan reads.
fn model_only_edits(model: &SessionModel) -> [(&'static str, SessionEdit); 4] {
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    [
        (
            "session ID",
            SessionEdit::SetSessionId {
                session_id: id("model-only-session"),
            },
        ),
        (
            "render profile ID",
            SessionEdit::SetRenderProfile {
                render_profile: session::RenderProfile {
                    id: id("model-only-render-profile"),
                    mode: model.render_profile.mode,
                },
            },
        ),
        (
            "output profile ID",
            SessionEdit::SetOutputProfile {
                output_profile: session::OutputProfile {
                    id: id("model-only-output-profile"),
                    ..model.output_profile.clone()
                },
            },
        ),
        (
            "automation upsert",
            SessionEdit::UpsertAutomation {
                automation: session::Automation {
                    id: id("fader-ride"),
                    target: session::AutomationTarget {
                        entity_id: model.tracks[0].id.clone(),
                        rack: session::RackName::Builtins,
                        effect_id: id(session::BUILTIN_AUTOMATION_EFFECT_ID),
                        parameter_id: 5,
                        channel: session::ParameterChannel::Both,
                    },
                    segments: vec![session::AutomationSegment {
                        shape: session::AutomationShape::Linear,
                        start_sample: 0,
                        end_sample: 960,
                        start_value: 0.0,
                        end_value: -3.0,
                        unit: session::ParameterUnit::Db,
                    }],
                },
            },
        ),
    ]
}

/// #1260 gate 2: a session ID, render-profile ID, output-profile ID or stored automation edit
/// commits one revision with one `SESSION_COMMITTED`, shows in the snapshot, and leaves the running
/// plan, its source ring and its output untouched: the host keeps feeding with no seek, and every
/// block is bit-identical to an uninterrupted render of the original session.
///
/// Test value: red if a model-only edit still rebuilds the plan (a new provider epoch or a pending
/// candidate, a source-ring reset that needs a seek, or a silent block), or if the classifier's
/// mask drops the edit from the committed model.
#[test]
fn model_only_edits_commit_without_a_plan_rebuild() {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let mut original = Reference::new(&document);
    // Warm up past the plan's latency, so every compared block below carries signal.
    for _ in 0..window(rig.latency(), 0, rig.quantum) {
        assert_eq!(bits(&rig.step()), bits(&original.step()), "warm-up");
    }
    let base = parse_session_json(&document).expect("model");
    for (name, edit) in model_only_edits(&base) {
        let mut expected = rig.model();
        protocol::apply_session_edit(&mut expected, &edit).expect("edit applies");
        let (revision, _, epoch, pending) = rig.summary();
        assert_eq!(pending, 0, "{name}");
        let bytes = rig.transaction(core::slice::from_ref(&edit));
        assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK, "{name}");
        let (after, _, after_epoch, after_pending) = rig.summary();
        assert_eq!(
            (after, after_epoch, after_pending),
            (revision + 1, epoch, 0),
            "{name}: one revision, the same provider epoch, no candidate"
        );
        assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"], "{name}");
        expected.revision = after;
        assert_eq!(
            session::canonical_session_json(&rig.model()).expect("canonical committed"),
            session::canonical_session_json(&expected).expect("canonical expected"),
            "{name}: the snapshot holds the edit"
        );
        for _ in 0..3 {
            let block = rig.block;
            let reference = original.step();
            assert!(
                reference.iter().any(|sample| *sample != 0.0),
                "{name}: block {block}: the reference carries signal"
            );
            assert_eq!(bits(&rig.step()), bits(&reference), "{name}: block {block}");
        }
    }
    assert_eq!(rig.generation, 1, "the host never seeked");
    assert_eq!(rig.summary().2, 0, "the first epoch still renders");
}

// Issue #1264: value-only effect parameter edits.

/// `long_session` with a compressor insert (`comp`) on every track and a `post_insert` console
/// soft-clip slot (`clip`), both at their defaults.
fn effect_session(track_count: usize, sample_rate_hz: u32) -> String {
    with_model(&long_session(track_count, sample_rate_hz), |model| {
        let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        let clip = StableId::parse("clip").expect("clip slot");
        for track in &mut model.tracks {
            let mut compressor = template.clone();
            compressor.id = StableId::parse("comp").expect("compressor");
            compressor.identity = session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor ID"),
            };
            compressor.params.clear();
            compressor.bypass = false;
            track.inserts.effects.push(compressor);
            track.console.push(session::ConsoleEntry {
                slot: clip.clone(),
                bypass: false,
                params: Vec::new(),
            });
        }
        model.console.post_insert.push(session::ConsoleSlot {
            slot: clip,
            identity: session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.soft-clip").expect("soft-clip ID"),
            },
            quality: template.quality,
            link_mode: template.link_mode,
        });
    })
}

fn upsert(
    track_id: &str,
    rack_name: session::RackName,
    effect_id: &str,
    parameter_id: u32,
    channel: session::ParameterChannel,
    unit: session::ParameterUnit,
    value: f32,
) -> SessionEdit {
    SessionEdit::UpsertEffectParam {
        track_id: StableId::parse(track_id).expect("track ID"),
        rack_name,
        effect_id: StableId::parse(effect_id).expect("effect ID"),
        param: session::EffectParam {
            parameter_id,
            channel,
            unit,
            value,
        },
    }
}

/// A `host_core` plan of one document prepared with every live lane, as the browser prepares,
/// fed the same source, with its effect producers.
struct LaneReference {
    reference: Reference,
    effects: Vec<host_core::EffectControlProducer>,
}

impl LaneReference {
    fn new(document: &str) -> Self {
        let caps = prepare_caps(limits());
        let compiled = host_core::compile_host_session(document, &caps).expect("lane compile");
        let (host, handles) = prepare_host_runtime_with_live_lanes(
            &compiled,
            &caps,
            &HostLiveControlRequest {
                control_queue_depth: Some(LIVE_QUEUE_DEPTH),
                ..HostLiveControlRequest::default()
            },
            HostLiveLanes::ALL,
        )
        .expect("lane prepare");
        Self {
            reference: Reference {
                plan: host.plan,
                sources: host.sources,
                rate: compiled.sample_rate().0,
                quantum: compiled.quantum().0 as usize,
                block: 0,
            },
            effects: handles.effect_controls,
        }
    }

    /// Pushes one parameter record, built from the descriptor, to the instance `effect_id` of
    /// the strip `track_id`.
    fn push(
        &mut self,
        track_id: &str,
        effect_id: &str,
        parameter_id: u32,
        channel: effect_contract::ParameterChannel,
        value: f32,
    ) {
        let producer = self
            .effects
            .iter_mut()
            .find(|producer| &*producer.track_id == track_id && &*producer.effect_id == effect_id)
            .expect("effect producer");
        let parameter_index = producer
            .descriptor
            .parameters
            .iter()
            .position(|parameter| parameter.id.0 == parameter_id)
            .expect("declared parameter") as u32;
        producer
            .try_push(effect_contract::EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value,
            })
            .expect("reference effect queue room");
    }
}

/// #1264 gate 2 for one session.
fn live_effect_pcm_shape(track_count: usize, sample_rate_hz: u32) {
    use effect_contract::ParameterChannel::{Left, Right};
    use session::{ParameterChannel, ParameterUnit, RackName};
    let label = format!("{track_count} tracks at {sample_rate_hz} Hz");
    let document = effect_session(track_count, sample_rate_hz);
    let model = parse_session_json(&document).expect("model");
    let first = model.tracks[0].id.as_str();
    let last = model.tracks[model.tracks.len() - 1].id.as_str();
    let mut rig = Rig::new(&document);
    let mut lanes = LaneReference::new(&document);
    let mut unedited = Reference::new(&document);
    // Past the plan's latency, so every compared block carries signal.
    let warm_up = window(rig.latency(), 0, rig.quantum);
    for block in 0..warm_up {
        let live = rig.step();
        assert_eq!(
            bits(&live),
            bits(&lanes.reference.step()),
            "{label}: warm-up {block}"
        );
        assert_eq!(
            bits(&live),
            bits(&unedited.step()),
            "{label}: warm-up {block}"
        );
    }

    // Compressor threshold, left lane, and makeup on both lanes, on the first track; soft-clip
    // drive on the last track's console slot.
    let edits = [
        upsert(
            first,
            RackName::Inserts,
            "comp",
            1,
            ParameterChannel::Left,
            ParameterUnit::Db,
            -30.0,
        ),
        upsert(
            first,
            RackName::Inserts,
            "comp",
            6,
            ParameterChannel::Both,
            ParameterUnit::Db,
            6.0,
        ),
        upsert(
            last,
            RackName::Console,
            "clip",
            1,
            ParameterChannel::Both,
            ParameterUnit::Db,
            12.0,
        ),
    ];
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
    let (after, _, after_epoch, pending) = rig.summary();
    assert_eq!(
        (after, after_epoch, pending),
        (revision + 1, epoch, 0),
        "{label}: the edit commits one revision and prepares no plan"
    );
    lanes.push(first, "comp", 1, Left, -30.0);
    lanes.push(first, "comp", 6, Left, 6.0);
    lanes.push(first, "comp", 6, Right, 6.0);
    lanes.push(last, "clip", 1, Left, 12.0);
    lanes.push(last, "clip", 1, Right, 12.0);

    let mut audible = false;
    for block in 0..12 {
        let live = rig.step();
        let expected = lanes.reference.step();
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "{label}: block {block} after the edit carries signal"
        );
        assert_eq!(
            bits(&live),
            bits(&expected),
            "{label}: block {block} after the edit"
        );
        audible |= bits(&live) != bits(&unedited.step());
    }
    assert!(audible, "{label}: the edit changes the output");
    assert_eq!(rig.summary().2, 0, "{label}: the first epoch still renders");
}

/// #1264 gate 2. Red if the C ABI path addresses a different instance, lane or parameter index
/// than the browser's lane: hand-built records pushed into a host-core plan with every live lane
/// must render the same bits as the C ABI edit.
#[test]
fn live_effect_parameter_edits_render_like_the_browsers_lane() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        for track_count in [1, 10] {
            live_effect_pcm_shape(track_count, sample_rate_hz);
        }
    }
}

/// Every parameter row's descriptor (as text) and state record (handle, flags, value bits),
/// paged through `ParameterMetadataGet` and `ParameterStateGet`.
fn parameter_readback(rig: &mut Rig) -> (Vec<String>, Vec<(u32, u32, u32)>) {
    let mut rows = Vec::new();
    let mut handles = Vec::new();
    let mut after_handle = 0;
    loop {
        let request = rig.next_request();
        let bytes = command_bytes(
            request,
            protocol::CommandPayload::ParameterMetadataGet(protocol::ParameterMetadataRequest {
                after_handle,
                limit: 8,
            }),
        );
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 512];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::ParameterMetadata(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("metadata response")
        else {
            panic!("expected a metadata page")
        };
        for descriptor in &page.descriptors {
            rows.push(format!("{descriptor:?}"));
            handles.push(descriptor.handle);
        }
        after_handle = page.last_handle;
        if page.eof {
            break;
        }
    }
    let mut state = Vec::new();
    for chunk in handles.chunks(32) {
        let request = rig.next_request();
        let bytes = command_bytes(
            request,
            protocol::CommandPayload::ParameterStateGet(&protocol::ParameterStateRequest {
                handles: chunk.to_vec(),
            }),
        );
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 512];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::ParameterState(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("state response")
        else {
            panic!("expected a state page")
        };
        state.extend(
            page.records
                .iter()
                .map(|record| (record.handle, record.flags, record.value.to_bits())),
        );
    }
    (rows, state)
}

/// #1264 gate 3. Red if the parameter readback keeps the old value after a live edit, or reports
/// a value a rebuild's catalog would not: a set, a lane split and a removal back to the default.
#[test]
fn the_parameter_readback_after_a_live_edit_equals_a_rebuilds() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    // The first track's compressor starts with a ratio (ID 2) of 8 on both lanes.
    let document = with_model(&effect_session(10, 48_000), |model| {
        let compressor = model.tracks[0]
            .inserts
            .effects
            .iter_mut()
            .find(|effect| effect.id.as_str() == "comp")
            .expect("compressor");
        compressor.params.push(session::EffectParam {
            parameter_id: 2,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Ratio,
            value: 8.0,
        });
    });
    let model = parse_session_json(&document).expect("model");
    let first = model.tracks[0].id.as_str();
    let last = model.tracks[model.tracks.len() - 1].id.as_str();
    let edits = vec![
        upsert(
            first,
            RackName::Inserts,
            "comp",
            1,
            ParameterChannel::Left,
            ParameterUnit::Db,
            -30.0,
        ),
        SessionEdit::RemoveEffectParam {
            track_id: StableId::parse(first).expect("track"),
            rack_name: RackName::Inserts,
            effect_id: StableId::parse("comp").expect("comp"),
            parameter_id: 2,
            channel: ParameterChannel::Both,
        },
        upsert(
            last,
            RackName::Console,
            "clip",
            1,
            ParameterChannel::Both,
            ParameterUnit::Db,
            12.0,
        ),
    ];

    let mut live = Rig::new(&document);
    live.step();
    let before = parameter_readback(&mut live);
    assert_eq!(live.apply(&edits), crate::RESULT_OK);
    assert_eq!(live.summary().3, 0, "the live rig did not rebuild");
    let after = parameter_readback(&mut live);
    assert_ne!(after.1, before.1, "the edit changes the readback");

    let mut rebuilt = Rig::new(&document);
    rebuilt.step();
    let mut structural = edits;
    structural.push(content_edit(&model));
    assert_eq!(rebuilt.apply(&structural), crate::RESULT_OK);
    assert_eq!(rebuilt.summary().3, 1, "the second rig rebuilt");
    assert_eq!(parameter_readback(&mut rebuilt), after);
}

/// The value each eligible parameter is set to: a live (`Block`), continuous parameter's
/// midpoint, or its minimum when the midpoint is its default.
fn eligible_values(
    descriptor: &effect_contract::EffectDescriptor,
) -> Vec<(u32, session::ParameterUnit, f32)> {
    use effect_contract::{AutomationRate, ParameterDomain, ParameterUnit as Unit};
    descriptor
        .parameters
        .iter()
        .filter(|parameter| {
            parameter.automatable
                && parameter.automation_rate == AutomationRate::Block
                && parameter.domain == ParameterDomain::Continuous
        })
        .map(|parameter| {
            let (minimum, maximum) = (
                parameter.minimum.expect("continuous minimum"),
                parameter.maximum.expect("continuous maximum"),
            );
            let middle = 0.5 * (minimum + maximum);
            let value = if middle == parameter.default_value {
                minimum
            } else {
                middle
            };
            let unit = match parameter.unit {
                Unit::Db => session::ParameterUnit::Db,
                Unit::Hz => session::ParameterUnit::Hz,
                Unit::Milliseconds => session::ParameterUnit::Milliseconds,
                Unit::Samples => session::ParameterUnit::Samples,
                Unit::Linear => session::ParameterUnit::Linear,
                Unit::Ratio => session::ParameterUnit::Ratio,
            };
            (parameter.id.0, unit, value)
        })
        .collect()
}

fn effect_room(rig: &Rig, epoch: Epoch, track_id: &str, effect_id: &str) -> usize {
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let epoch = match epoch {
        Epoch::Current => snapshot.provider_epoch,
        Epoch::Pending => *snapshot
            .pending_provider_epochs
            .last()
            .expect("a pending candidate"),
    };
    snapshot
        .effect_rooms
        .iter()
        .find(|(row_epoch, track, effect, _)| {
            *row_epoch == epoch && &**track == track_id && &**effect == effect_id
        })
        .map(|&(.., room)| room)
        .expect("effect producer")
}

/// #1264 gate 4. Red if a transaction whose records for one instance outnumber its queue's whole
/// capacity returns `BACKPRESSURE` (forever: no render ever makes room for it) instead of taking
/// the structural path; the boundary case, records equal to the capacity, stays live.
#[test]
fn an_effect_edit_larger_than_its_queue_rebuilds() {
    let document = with_model(&long_session(10, 48_000), |model| {
        let mut multiband = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        multiband.id = StableId::parse("mb").expect("multiband");
        multiband.identity = session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.multiband-compressor").expect("multiband ID"),
        };
        multiband.params.clear();
        multiband.bypass = false;
        model.tracks[0].inserts.effects.push(multiband);
    });
    let descriptor = compile_children(&document, limits())
        .expect("multiband session")
        .session
        .test_providers()
        .test_effects()
        .iter()
        .find(|producer| &*producer.effect_id == "mb")
        .expect("multiband producer")
        .descriptor;
    let values = eligible_values(descriptor);
    let depth = LIVE_QUEUE_DEPTH.get();
    assert!(2 * values.len() > depth, "enough live per-lane parameters");
    let edits = |count: usize| {
        values[..count]
            .iter()
            .map(|&(parameter_id, unit, value)| {
                upsert(
                    "eq0",
                    session::RackName::Inserts,
                    "mb",
                    parameter_id,
                    session::ParameterChannel::Both,
                    unit,
                    value,
                )
            })
            .collect::<Vec<_>>()
    };

    // `depth / 2` per-lane parameters on both lanes: exactly the queue's capacity, live.
    let mut rig = Rig::new(&document);
    rig.step();
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "mb"), depth);
    assert_eq!(rig.apply(&edits(depth / 2)), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "a full-capacity edit is live");
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "mb"), 0);
    drop(rig);

    // One more parameter: it could never fit, so it rebuilds, and pushes nothing.
    let mut rig = Rig::new(&document);
    rig.step();
    let (revision, ..) = rig.summary();
    let larger = edits(depth / 2 + 1);
    assert_eq!(rig.apply(&larger), crate::RESULT_OK);
    let (after, _, _, pending) = rig.summary();
    assert_eq!(
        (after, pending),
        (revision + 1, 1),
        "a rebuild prepares a candidate"
    );
    assert_eq!(
        effect_room(&rig, Epoch::Current, "eq0", "mb"),
        depth,
        "the rendering plan got no record"
    );
    assert_eq!(effect_room(&rig, Epoch::Pending, "eq0", "mb"), depth);
}

/// #1264 D3. Red if the room check skips the effect queues: a 17th single-record edit with no
/// render between would be acked and then fail to push. A full effect lane is typed backpressure
/// before anything changes, and a retry after one render succeeds.
#[test]
fn a_full_effect_lane_refuses_before_anything_changes() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let mut rig = Rig::new(&effect_session(10, 48_000));
    rig.step();
    let depth = LIVE_QUEUE_DEPTH.get();
    let threshold = |value: f32| {
        [upsert(
            "eq0",
            RackName::Inserts,
            "comp",
            1,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )]
    };
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&threshold(-21.0 - edit as f32)),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "comp"), 0);
    let before = refusal_state(&rig);
    assert_eq!(rig.apply(&threshold(-40.0)), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure".to_vec());
    assert_eq!(refusal_state(&rig), before);
    assert_eq!(rig.summary().3, 0, "no rebuild");
    rig.step();
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "comp"), depth);
    assert_eq!(rig.apply(&threshold(-40.0)), crate::RESULT_OK);
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "comp"), depth - 1);
}

// Issue #1265: parametric EQ parameter edits through prepared targets.

/// The nine-track EQ fixture at `sample_rate_hz`, with a source long enough for a whole run, an
/// EQ insert (`eq-insert`: band 1 enabled at 500 Hz, +4 dB) on its last track beside the console
/// EQ slot (`eq`) every track carries, and band 1 of eq4's console EQ enabled at +6 dB, so an
/// edit of its Q is audible.
fn eq_session(sample_rate_hz: u32) -> String {
    use session::{EffectParam, ParameterChannel, ParameterUnit};
    let mut model = parse_session_json(SESSION).expect("nine-track EQ fixture");
    model.sample_rate_hz = sample_rate_hz;
    model.sources[0].frames = SOURCE_FRAMES;
    let mut insert = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    insert.id = StableId::parse("eq-insert").expect("EQ insert");
    insert.bypass = false;
    let both = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    insert.params = vec![
        both(1, ParameterUnit::Linear, 1.0),
        both(3, ParameterUnit::Hz, 500.0),
        both(4, ParameterUnit::Db, 4.0),
    ];
    let last = model.tracks.len() - 1;
    model.tracks[last].inserts.effects.push(insert);
    let eq4 = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq4")
        .expect("eq4");
    eq4.console[0].params = vec![
        both(1, ParameterUnit::Linear, 1.0),
        both(4, ParameterUnit::Db, 6.0),
    ];
    session::canonical_session_json(&model).expect("canonical EQ session")
}

/// One EQ edit, by descriptor parameter ID.
type EqEdit = (u32, effect_contract::ParameterChannel, f32);

/// The targets `EqTargetPreparer` designs for `edits` from `seeds`, called directly.
fn designed_eq_targets(
    sample_rate_hz: u32,
    seeds: &[f32],
    edits: &[EqEdit],
) -> Vec<effect_contract::PreparedEffectTarget> {
    let preparer = host_core::EqTargetPreparer::new(
        host_core::parametric_eq_target_preparation_factory().expect("EQ capability"),
    )
    .expect("EQ target preparer");
    let edits: Vec<host_core::EqTargetEdit> = edits
        .iter()
        .map(|&(parameter_id, channel, value)| host_core::EqTargetEdit {
            parameter_id,
            channel,
            value,
        })
        .collect();
    let mut out = [effect_contract::PreparedEffectTarget {
        slot: 0,
        channel: effect_contract::ParameterChannel::Both,
        words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
    }; host_core::EQ_TARGET_CAPACITY];
    let (_, count) = preparer
        .prepare(sample_rate_hz, seeds, &edits, &mut out)
        .expect("designed EQ targets");
    out[..count].to_vec()
}

impl LaneReference {
    fn eq_producer(
        &mut self,
        track_id: &str,
        effect_id: &str,
    ) -> &mut host_core::EffectControlProducer {
        self.effects
            .iter_mut()
            .find(|producer| &*producer.track_id == track_id && &*producer.effect_id == effect_id)
            .expect("EQ producer")
    }

    /// The EQ instance's committed rows, the seeds a designer starts from.
    fn eq_seeds(&mut self, track_id: &str, effect_id: &str) -> Vec<f32> {
        self.eq_producer(track_id, effect_id)
            .owner()
            .expect("EQ owner")
            .committed()
            .iter()
            .map(|row| row.value)
            .collect()
    }

    /// Runs one EQ owner transaction as the browser's lane does: the edits, targets designed in
    /// the test by `EqTargetPreparer` from the owner's committed rows, publication and commit.
    fn publish_eq(&mut self, track_id: &str, effect_id: &str, edits: &[EqEdit]) {
        let rate = self.reference.rate;
        let seeds = self.eq_seeds(track_id, effect_id);
        let targets = designed_eq_targets(rate, &seeds, edits);
        let producer = self.eq_producer(track_id, effect_id);
        let base = producer.owner().expect("EQ owner").committed_revision();
        producer.begin_owner(base).expect("begin");
        for &(parameter_id, channel, value) in edits {
            let index = producer
                .descriptor
                .parameters
                .iter()
                .position(|parameter| parameter.id.0 == parameter_id)
                .expect("declared parameter") as u32;
            producer.edit_owner(index, channel, value).expect("edit");
        }
        producer
            .publish_candidate_targets(base, &targets)
            .expect("reference publication");
        producer.commit_owner().expect("reference commit");
    }
}

/// #1265 gate 2 at one rate.
fn live_eq_pcm_shape(sample_rate_hz: u32) {
    use effect_contract::ParameterChannel::{Both, Left};
    use session::{ParameterChannel, ParameterUnit, RackName};
    let label = format!("nine tracks at {sample_rate_hz} Hz");
    let document = eq_session(sample_rate_hz);
    let model = parse_session_json(&document).expect("model");
    let last = model.tracks[model.tracks.len() - 1].id.as_str();
    let mut rig = Rig::new(&document);
    let mut lanes = LaneReference::new(&document);
    let mut unedited = Reference::new(&document);
    let warm_up = window(rig.latency(), 0, rig.quantum);
    for block in 0..warm_up {
        let live = rig.step();
        assert_eq!(
            bits(&live),
            bits(&lanes.reference.step()),
            "{label}: warm-up {block}"
        );
        assert_eq!(
            bits(&live),
            bits(&unedited.step()),
            "{label}: warm-up {block}"
        );
    }

    // Band 1 gain on eq0's left lane and band 1 Q on eq4's both lanes (an enabled +6 dB band),
    // on the console slot; the HPF enabled at 300 Hz on the last track's EQ insert.
    let edits = [
        upsert(
            "eq0",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            3.0,
        ),
        upsert(
            "eq4",
            RackName::Console,
            "eq",
            5,
            ParameterChannel::Both,
            ParameterUnit::Ratio,
            2.0,
        ),
        upsert(
            last,
            RackName::Inserts,
            "eq-insert",
            65,
            ParameterChannel::Both,
            ParameterUnit::Linear,
            1.0,
        ),
        upsert(
            last,
            RackName::Inserts,
            "eq-insert",
            66,
            ParameterChannel::Both,
            ParameterUnit::Hz,
            300.0,
        ),
    ];
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
    let (after, _, after_epoch, pending) = rig.summary();
    assert_eq!(
        (after, after_epoch, pending),
        (revision + 1, epoch, 0),
        "{label}: the edit commits one revision and prepares no plan"
    );
    lanes.publish_eq("eq0", "eq", &[(4, Left, 3.0)]);
    lanes.publish_eq("eq4", "eq", &[(5, Both, 2.0)]);
    lanes.publish_eq(last, "eq-insert", &[(65, Both, 1.0), (66, Both, 300.0)]);

    let mut audible = false;
    for block in 0..12 {
        let live = rig.step();
        let expected = lanes.reference.step();
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "{label}: block {block} after the edit carries signal"
        );
        assert_eq!(
            bits(&live),
            bits(&expected),
            "{label}: block {block} after the edit"
        );
        audible |= bits(&live) != bits(&unedited.step());
    }
    assert!(audible, "{label}: the edit changes the output");
    assert_eq!(rig.summary().2, 0, "{label}: the first epoch still renders");
}

/// #1265 gate 2. Red if the C ABI designs, addresses or publishes an EQ target differently from
/// the browser's lane: targets designed in the test by `EqTargetPreparer` from the owner's own
/// rows, published through the owner of a host-core plan with every live lane at the same block,
/// must render the same bits as the C ABI edit.
#[test]
fn live_eq_parameter_edits_render_like_the_browsers_lane() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        live_eq_pcm_shape(sample_rate_hz);
    }
}

fn effect_owners(rig: &Rig) -> Vec<TestOwnerState> {
    crate::ffi::test_transaction_snapshot(rig.session).effect_owners
}

/// The committed revision and phase of one EQ owner in the current epoch.
fn owner_phase(rig: &Rig, track_id: &str, effect_id: &str) -> (u64, String) {
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    snapshot
        .effect_owners
        .iter()
        .find(|owner| {
            owner.0 == snapshot.provider_epoch && &*owner.1 == track_id && &*owner.2 == effect_id
        })
        .map(|owner| (owner.3, owner.4.clone()))
        .expect("EQ owner")
}

/// #1265 gate 3. Red if a refused transaction leaves an EQ owner begun or published: a fader
/// lane's backpressure after the EQ owner's preflight, a full EQ lane, and a full EQ lane behind
/// another begun owner each leave every owner, queue and the model as they were, and the retry
/// after a render commits.
#[test]
fn a_refused_eq_transaction_leaves_its_owner_idle() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let mut rig = Rig::new(&eq_session(48_000));
    rig.step();
    let depth = LIVE_QUEUE_DEPTH.get();
    let gain = |value: f32| {
        upsert(
            "eq0",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )
    };

    // eq1's fader lane is full; the EQ edit on eq0 rides beside a fader edit on eq1.
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&[left_db_edit("eq1", -1.0 - edit as f32)]),
            crate::RESULT_OK
        );
    }
    let before = (refusal_state(&rig), effect_owners(&rig));
    let rooms = effect_room(&rig, Epoch::Current, "eq0", "eq");
    let transaction = [gain(3.0), left_db_edit("eq1", -30.0)];
    assert_eq!(rig.apply(&transaction), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!((refusal_state(&rig), effect_owners(&rig)), before);
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), rooms);
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (0, "Idle".to_owned()));
    rig.step();
    assert_eq!(rig.apply(&transaction), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the retry is live");
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (1, "Idle".to_owned()));
    rig.step();

    // Each band-1 left gain edit designs one target: sixteen fill eq0's EQ lane, and the
    // seventeenth is typed backpressure that changes nothing.
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&[gain(-1.0 - edit as f32)]),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 0);
    let before = (refusal_state(&rig), effect_owners(&rig));
    assert_eq!(rig.apply(&[gain(-20.0)]), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!((refusal_state(&rig), effect_owners(&rig)), before);
    assert_eq!(rig.summary().3, 0, "no rebuild");
    rig.step();
    assert_eq!(rig.apply(&[gain(-20.0)]), crate::RESULT_OK);
    assert_eq!(
        owner_phase(&rig, "eq0", "eq"),
        (2 + depth as u64, "Idle".to_owned())
    );
    drop(rig);

    // Two begun owners (#1265 verdict MINOR 1): EQ edits on eq0 and on eq1 while eq1's EQ lane
    // is full. eq0's owner is begun and preflighted before eq1's refuses, and both are discarded.
    let mut rig = Rig::new(&eq_session(48_000));
    rig.step();
    let eq1_gain = |value: f32| {
        upsert(
            "eq1",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )
    };
    for edit in 0..depth {
        assert_eq!(
            rig.apply(&[eq1_gain(-1.0 - edit as f32)]),
            crate::RESULT_OK,
            "eq1 edit {edit}"
        );
    }
    assert_eq!(effect_room(&rig, Epoch::Current, "eq1", "eq"), 0);
    let before = (refusal_state(&rig), effect_owners(&rig));
    let rooms = (
        effect_room(&rig, Epoch::Current, "eq0", "eq"),
        effect_room(&rig, Epoch::Current, "eq1", "eq"),
    );
    let transaction = [gain(3.0), eq1_gain(-20.0)];
    assert_eq!(rig.apply(&transaction), crate::RESULT_BACKPRESSURE);
    assert_eq!(rig.last_error(), b"control.live.backpressure");
    assert_eq!((refusal_state(&rig), effect_owners(&rig)), before);
    assert_eq!(
        (
            effect_room(&rig, Epoch::Current, "eq0", "eq"),
            effect_room(&rig, Epoch::Current, "eq1", "eq"),
        ),
        rooms
    );
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (0, "Idle".to_owned()));
    assert_eq!(
        owner_phase(&rig, "eq1", "eq"),
        (depth as u64, "Idle".to_owned())
    );
    rig.step();
    assert_eq!(rig.apply(&transaction), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the retry is live");
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (1, "Idle".to_owned()));
    assert_eq!(
        owner_phase(&rig, "eq1", "eq"),
        (1 + depth as u64, "Idle".to_owned())
    );
}

/// #1265 gate 4. Red if an EQ edit whose designed targets outnumber its queue's whole capacity
/// returns `BACKPRESSURE` (forever) instead of taking the structural path; one whose targets fit
/// stays live. The queue is capped at four by `maximum_automation_spans_per_block`.
#[test]
fn an_eq_edit_designing_more_targets_than_its_queue_rebuilds() {
    use effect_contract::ParameterChannel::Both;
    use session::{ParameterChannel, ParameterUnit, RackName};
    let document = eq_session(48_000);
    let small = CompileLimits {
        maximum_automation_spans_per_block: 4,
        ..limits()
    };
    // Band gains on both lanes: band 1's lanes differ (two targets), bands 2-4 share one each.
    let gains = [4, 20, 36, 52];
    let seeds = LaneReference::new(&document).eq_seeds("eq0", "eq");
    let edits = |bands: usize| {
        // Per lane: the fixture sets band 1's gain per lane, so a `Both` entry would duplicate it.
        gains[..bands]
            .iter()
            .flat_map(|&parameter_id| {
                [ParameterChannel::Left, ParameterChannel::Right].map(|channel| {
                    upsert(
                        "eq0",
                        RackName::Console,
                        "eq",
                        parameter_id,
                        channel,
                        ParameterUnit::Db,
                        -5.0,
                    )
                })
            })
            .collect::<Vec<_>>()
    };
    let designed = |bands: usize| {
        let edits: Vec<EqEdit> = gains[..bands]
            .iter()
            .map(|&parameter_id| (parameter_id, Both, -5.0))
            .collect();
        designed_eq_targets(48_000, &seeds, &edits).len()
    };
    assert_eq!((designed(3), designed(4)), (4, 5));

    let mut rig = Rig::with_limits(&document, small);
    rig.step();
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 4);
    assert_eq!(rig.apply(&edits(3)), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "four targets are live");
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 0);
    drop(rig);

    let mut rig = Rig::with_limits(&document, small);
    rig.step();
    let (revision, ..) = rig.summary();
    let owners = effect_owners(&rig);
    assert_eq!(rig.apply(&edits(4)), crate::RESULT_OK);
    let (after, _, _, pending) = rig.summary();
    assert_eq!((after, pending), (revision + 1, 1), "five targets rebuild");
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 4);
    assert_eq!(effect_room(&rig, Epoch::Pending, "eq0", "eq"), 4);
    let current: Vec<_> = effect_owners(&rig)
        .into_iter()
        .filter(|owner| owner.0 == owners[0].0)
        .collect();
    assert_eq!(current, owners, "the rendering plan's owners are untouched");
}

/// #1265 gate 5. Red if the parameter readback keeps an EQ value's old state after a live edit,
/// or reports one a rebuild's catalog would not.
#[test]
fn the_eq_parameter_readback_after_a_live_edit_equals_a_rebuilds() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let document = eq_session(48_000);
    let model = parse_session_json(&document).expect("model");
    let last = model.tracks[model.tracks.len() - 1].id.as_str();
    let edits = vec![
        upsert(
            "eq0",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            3.0,
        ),
        upsert(
            last,
            RackName::Inserts,
            "eq-insert",
            66,
            ParameterChannel::Both,
            ParameterUnit::Hz,
            300.0,
        ),
        SessionEdit::RemoveEffectParam {
            track_id: StableId::parse(last).expect("track"),
            rack_name: RackName::Inserts,
            effect_id: StableId::parse("eq-insert").expect("EQ insert"),
            parameter_id: 4,
            channel: ParameterChannel::Both,
        },
    ];

    let mut live = Rig::new(&document);
    live.step();
    let before = parameter_readback(&mut live);
    assert_eq!(live.apply(&edits), crate::RESULT_OK);
    assert_eq!(live.summary().3, 0, "the live rig did not rebuild");
    let after = parameter_readback(&mut live);
    assert_ne!(after.1, before.1, "the edit changes the readback");

    let mut rebuilt = Rig::new(&document);
    rebuilt.step();
    let mut structural = edits;
    structural.push(content_edit(&model));
    assert_eq!(rebuilt.apply(&structural), crate::RESULT_OK);
    assert_eq!(rebuilt.summary().3, 1, "the second rig rebuilt");
    assert_eq!(parameter_readback(&mut rebuilt), after);
}

// Issue #1266: value-only effect bypass edits through the latency-preserving shunt.

fn bypass_edit(
    track_id: &str,
    rack_name: session::RackName,
    effect_id: &str,
    bypass: bool,
) -> SessionEdit {
    SessionEdit::SetEffectBypass {
        track_id: StableId::parse(track_id).expect("track ID"),
        rack_name,
        effect_id: StableId::parse(effect_id).expect("effect ID"),
        bypass,
    }
}

/// #1266 gate 2 for one session: bypass the first track's compressor insert and the last track's
/// console soft-clip live, then lift both.
fn live_bypass_pcm_shape(track_count: usize, sample_rate_hz: u32) {
    use session::RackName;
    let label = format!("{track_count} tracks at {sample_rate_hz} Hz");
    let document = effect_session(track_count, sample_rate_hz);
    let model = parse_session_json(&document).expect("model");
    let first = model.tracks[0].id.as_str();
    let last = model.tracks[model.tracks.len() - 1].id.as_str();
    let mut rig = Rig::new(&document);
    let latency = rig.latency();
    let quantum = rig.quantum;
    for _ in 0..window(latency, 0, quantum) {
        rig.step();
    }

    let mut was_audible = false;
    for bypass in [true, false] {
        let edits = [
            bypass_edit(first, RackName::Inserts, "comp", bypass),
            bypass_edit(last, RackName::Console, "clip", bypass),
        ];
        let (revision, _, epoch, _) = rig.summary();
        assert_eq!(
            rig.apply(&edits),
            crate::RESULT_OK,
            "{label}: bypass {bypass}"
        );
        let (after, _, after_epoch, pending) = rig.summary();
        assert_eq!(
            (after, after_epoch, pending),
            (revision + 1, epoch, 0),
            "{label}: bypass {bypass} commits one revision and prepares no plan"
        );
        let committed = rig.snapshot();
        let settled = settle(&mut rig, window(latency, 0, quantum), 3);
        let rebuilt = Reference::run(&committed, rig.block);
        let unedited = Reference::run(&document, rig.block);
        for (block, live) in settled {
            let expected = &rebuilt[block as usize];
            assert!(
                expected.iter().any(|sample| *sample != 0.0),
                "{label}: block {block} carries signal"
            );
            assert_eq!(
                bits(&live),
                bits(expected),
                "{label}: bypass {bypass}, block {block}"
            );
            if bypass {
                was_audible |= bits(&live) != bits(&unedited[block as usize]);
            }
        }
    }
    assert!(was_audible, "{label}: the bypass changes the output");
    assert_eq!(rig.summary().2, 0, "{label}: the first epoch still renders");
}

/// #1266 gate 2. Red if a live bypass record differs from what preparation bakes for the
/// committed bypass, or lands on another instance: from E + K on, the live plan must render the
/// bits of a plan compiled from the committed snapshot and fed the same source from sample 0,
/// after the bypass and again after the lift.
#[test]
fn live_effect_bypass_edits_render_like_a_rebuild() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        for track_count in [1, 10] {
            live_bypass_pcm_shape(track_count, sample_rate_hz);
        }
    }
}

/// #1266 D2 on the EQ, at one rate.
fn live_eq_bypass_pcm_shape(sample_rate_hz: u32) {
    use effect_contract::ParameterChannel::Left;
    use session::{ParameterChannel, ParameterUnit, RackName};
    let label = format!("nine tracks at {sample_rate_hz} Hz");
    let document = eq_session(sample_rate_hz);
    let mut rig = Rig::new(&document);
    let mut lanes = LaneReference::new(&document);
    for block in 0..window(rig.latency(), 0, rig.quantum) {
        assert_eq!(
            bits(&rig.step()),
            bits(&lanes.reference.step()),
            "{label}: warm-up {block}"
        );
    }
    let edits = [
        bypass_edit("eq0", RackName::Console, "eq", true),
        upsert(
            "eq0",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            3.0,
        ),
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
    assert_eq!(rig.summary().3, 0, "{label}: live");
    lanes
        .eq_producer("eq0", "eq")
        .try_push(effect_contract::EffectControlRecord::Bypass(true))
        .expect("reference bypass");
    lanes.publish_eq("eq0", "eq", &[(4, Left, 3.0)]);
    for block in 0..12 {
        assert_eq!(
            bits(&rig.step()),
            bits(&lanes.reference.step()),
            "{label}: block {block} after the edit"
        );
    }
}

/// #1266 D2. Red if an EQ's bypass record is dropped beside its owner's transaction (or taken
/// for an owner edit, which refuses the commit): the C ABI must render the bits of a host-core plan
/// with every live lane given the `Bypass` record and then the owner's designed targets.
#[test]
fn a_live_eq_bypass_beside_a_gain_change_renders_like_the_browsers_lane() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        live_eq_bypass_pcm_shape(sample_rate_hz);
    }
}

/// #1266 D2, the acked-batch question. Red if the room check counts an EQ's targets without its
/// `Bypass` record: with room for the targets alone, the bypass would be pushed and the
/// publication would then fail after the transaction was admitted. It is typed backpressure
/// instead, before anything changes, and a retry after one render succeeds.
#[test]
fn an_eq_bypass_and_targets_without_room_for_both_refuse_before_anything_changes() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let mut rig = Rig::new(&eq_session(48_000));
    rig.step();
    let depth = LIVE_QUEUE_DEPTH.get();
    let gain = |value| {
        upsert(
            "eq0",
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )
    };
    // One left-lane band gain designs one target: fill the queue to one free slot.
    for step in 1..depth {
        assert_eq!(rig.apply(&[gain(step as f32 / 8.0)]), crate::RESULT_OK);
    }
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 1);
    let before = refusal_state(&rig);
    let owners = effect_owners(&rig);
    let edits = [
        bypass_edit("eq0", RackName::Console, "eq", true),
        gain(-3.0),
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_BACKPRESSURE);
    assert_eq!(refusal_state(&rig), before, "the refusal changes nothing");
    assert_eq!(effect_owners(&rig), owners, "every owner is left as it was");
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 1);
    rig.step();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the retry is live");
}

/// #1266 gate 3, decision 14 F4. Red if the C ABI acks a multiband compressor bypass change, in
/// either direction, on the running plan, which holds that bypass prepared and would never render
/// it: the change prepares a new plan, and once it renders, its output follows the committed model.
/// The oracle is a control that holds the committed bypass from the start and rebuilds through
/// content edits on the same blocks, so it carries what the bypass rebuild carried (#1276).
#[test]
fn a_prepared_bypass_change_rebuilds_and_renders_the_committed_model() {
    let with_bypass = |bypass: bool| {
        with_model(&long_session(10, 48_000), |model| {
            let mut multiband = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
            multiband.id = StableId::parse("mb").expect("multiband");
            multiband.identity = session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.multiband-compressor").expect("multiband ID"),
            };
            multiband.params.clear();
            multiband.bypass = bypass;
            model.tracks[0].inserts.effects.push(multiband);
        })
    };
    let document = with_bypass(false);
    // The blocks after the swap of `rebuilds` successive rebuilds, each one block, the swap, a
    // seek and six blocks, by content edits from a document holding the multiband at `bypass`.
    let control = |bypass: bool, rebuilds: u8| {
        let document = with_bypass(bypass);
        let model = parse_session_json(&document).expect("control model");
        let mut control = Rig::new(&document);
        control.step();
        let mut rebuilt = Vec::new();
        for tag in 0..rebuilds {
            let mut edit = content_edit(&model);
            if let SessionEdit::SetSourceContent { content, .. } = &mut edit {
                *content = format!("blake3:{}", format!("{:02x}", 0xc0 + tag).repeat(32));
            }
            assert_eq!(control.apply(&[edit]), crate::RESULT_OK);
            assert_eq!(control.summary().3, 1, "the control rebuilds");
            control.render();
            control.seek_to_start();
            rebuilt = (0..6).map(|_| bits(&control.step())).collect::<Vec<_>>();
        }
        rebuilt
    };
    let mut rig = Rig::new(&document);
    rig.step();
    for (rebuilds, bypass) in [(1, true), (2, false)] {
        let (revision, _, epoch, pending) = rig.summary();
        assert_eq!(pending, 0);
        assert_eq!(
            rig.apply(&[bypass_edit("eq0", session::RackName::Inserts, "mb", bypass)]),
            crate::RESULT_OK
        );
        assert_eq!(
            rig.summary().3,
            1,
            "bypass {bypass}: the change prepares a candidate"
        );
        assert_eq!(rig.summary().0, revision + 1);
        let snapshot = rig.snapshot();
        assert_eq!(
            parse_session_json(&snapshot).expect("snapshot").tracks[0]
                .inserts
                .effects
                .last()
                .expect("mb")
                .bypass,
            bypass
        );
        // The render call swaps the candidate in; the host then restarts the source.
        rig.render();
        rig.seek_to_start();
        let rebuilt: Vec<_> = (0..6).map(|_| bits(&rig.step())).collect();
        let (_, _, swapped, pending) = rig.summary();
        assert_eq!(pending, 0, "bypass {bypass}: the candidate was taken");
        assert_ne!(swapped, epoch, "bypass {bypass}: a new epoch renders");
        let reference = control(bypass, rebuilds);
        assert_eq!(
            rebuilt, reference,
            "bypass {bypass}: the rebuilt plan renders the committed model"
        );
        assert_ne!(
            reference,
            control(!bypass, rebuilds),
            "bypass {bypass}: the change is audible"
        );
    }
}

/// #1266 verdict MINOR 1. Red if the over-capacity check leaves an EQ's `Bypass` record out of the
/// count: a bypass beside edits whose designed targets exactly fill the queue could never fit, so
/// it must rebuild, never return an endless `BACKPRESSURE`.
#[test]
fn an_eq_bypass_beside_targets_filling_its_queue_rebuilds() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let document = eq_session(48_000);
    let small = CompileLimits {
        maximum_automation_spans_per_block: 4,
        ..limits()
    };
    // Bands 1-3 gains on both lanes design four targets (gate 4), filling the queue of four.
    let mut edits: Vec<SessionEdit> = [4, 20, 36]
        .iter()
        .flat_map(|&parameter_id| {
            [ParameterChannel::Left, ParameterChannel::Right].map(|channel| {
                upsert(
                    "eq0",
                    RackName::Console,
                    "eq",
                    parameter_id,
                    channel,
                    ParameterUnit::Db,
                    -5.0,
                )
            })
        })
        .collect();
    edits.insert(0, bypass_edit("eq0", RackName::Console, "eq", true));
    let mut rig = Rig::with_limits(&document, small);
    rig.step();
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 4);
    let (revision, ..) = rig.summary();
    assert_eq!(
        rig.apply(&edits),
        crate::RESULT_OK,
        "admitted, not backpressure"
    );
    let (after, _, _, pending) = rig.summary();
    assert_eq!(
        (after, pending),
        (revision + 1, 1),
        "a bypass and four targets rebuild"
    );
    assert_eq!(effect_room(&rig, Epoch::Current, "eq0", "eq"), 4);
}

// Issue #1335: stored automation on an effect parameter the plan keeps prepared.

/// An `UpsertAutomation` of one step on `eq_session`'s `eq0` console EQ, on a `(parameter_id,
/// channel)` that `eq0` declares.
fn eq_automation_edit(
    parameter_id: u32,
    channel: session::ParameterChannel,
    unit: session::ParameterUnit,
) -> SessionEdit {
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("eq-ride"),
            target: session::AutomationTarget {
                entity_id: id("eq0"),
                rack: session::RackName::Console,
                effect_id: id("eq"),
                parameter_id,
                channel,
            },
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Step,
                start_sample: 0,
                end_sample: 960,
                start_value: 1.0,
                end_value: 1.0,
                unit,
            }],
        },
    }
}

/// #1335 gate 3. On a playing engine, a transaction that only adds an automation on the console
/// EQ's `band-1-enabled` (`automation_rate` `None`) is refused with the compile rejection a
/// rebuild's preparation gives (`effect.automation.rate`): the revision, the committed model and
/// the reliable lane do not move, no candidate is prepared, and the following blocks are
/// bit-identical to an engine that never received it. The same transaction on the block-rate
/// `band-1-gain` commits live, as #1260 D2 made it.
///
/// Red mutation: drop the classifier's automation check -> the edit commits live with no records:
/// acked, and never heard (decision 14 F1).
#[test]
fn an_automation_on_a_prepared_effect_parameter_is_refused_before_any_ack() {
    use session::{ParameterChannel, ParameterUnit};
    let document = eq_session(48_000);
    let mut rig = Rig::new(&document);
    let mut untouched = Rig::new(&document);
    for _ in 0..window(rig.latency(), 0, rig.quantum) {
        assert_eq!(bits(&rig.step()), bits(&untouched.step()), "warm-up");
    }

    let refused = eq_automation_edit(1, ParameterChannel::Both, ParameterUnit::Linear);
    let before = (refusal_state(&rig), rig.summary());
    assert_eq!(
        rig.apply(core::slice::from_ref(&refused)),
        crate::RESULT_COMPILE_REJECTED
    );
    let mut rejected = parse_session_json(&document).expect("model");
    protocol::apply_session_edit(&mut rejected, &refused).expect("the edit applies to the model");
    let rejected = session::canonical_session_json(&rejected).expect("canonical rejected");
    let Err(failure) = compile_children(&rejected, limits()) else {
        panic!("an automation on band-1-enabled does not prepare")
    };
    assert_eq!(
        rig.last_error(),
        failure.diagnostics,
        "the refusal is preparation's diagnostic"
    );
    assert!(
        String::from_utf8_lossy(&failure.diagnostics).contains("effect.automation.rate"),
        "{}",
        String::from_utf8_lossy(&failure.diagnostics)
    );
    assert_eq!(
        (refusal_state(&rig), rig.summary()),
        before,
        "no revision, no candidate, no event"
    );
    for _ in 0..3 {
        let block = rig.block;
        let expected = untouched.step();
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "block {block}"
        );
        assert_eq!(bits(&rig.step()), bits(&expected), "block {block}");
    }

    let accepted = eq_automation_edit(4, ParameterChannel::Left, ParameterUnit::Db);
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(
        rig.apply(core::slice::from_ref(&accepted)),
        crate::RESULT_OK
    );
    let (after, _, after_epoch, pending) = rig.summary();
    assert_eq!(
        (after, after_epoch, pending),
        (revision + 1, epoch, 0),
        "a block-rate target commits live: one revision, no candidate"
    );
    for _ in 0..3 {
        let block = rig.block;
        assert_eq!(bits(&rig.step()), bits(&untouched.step()), "block {block}");
    }
}
