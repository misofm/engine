//! Value-only track fader, mute and pan transactions on the running C ABI plan (#1257 gates 1-4).
//!
//! Every edit goes through `miso_engine_v1_submit_command`. The source is fed one quantum per block
//! for the whole run, with no zero sample and distinct lanes, and never seeks except where a test
//! rebuilds the plan on purpose.

use super::tests::{
    SESSION, automation_status, automation_target, boxed_c_children_with_limits, command_bytes,
    command_bytes_at_revision, command_c, control_list_all_c, control_list_c, control_state_all_c,
    control_state_c, event_c, generated_parity_session, limits, submit_c,
};
use super::*;
use protocol::{ExpectedRevision, SessionEdit, SessionRevision};
use session::{
    ControlSmoothing, DualMonoFader, MatrixOrPan, RouteDestination, RouteSource, SessionModel,
    StableId, Submix, Vca, parse_session_json,
};

const SOURCE: &[u8] = b"fixture-source";
/// Long enough for every run below.
const SOURCE_FRAMES: u64 = 96 * 128;
/// The depth of the queue the effect lanes replaced (#1345): a transaction larger than it proves
/// the cells have no queue to overflow.
const OLD_QUEUE_DEPTH: usize = 16;

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

/// A source with stretches of `-0.0` samples (#1556: the input filters and the sign of a zero),
/// and, where it is not quiet, either the usual wobbling lanes or one constant.
struct Feed {
    quiet: Vec<core::ops::Range<u64>>,
    constant: Option<f32>,
}

impl Feed {
    /// `-0.0` before frame `end`, the usual source after it.
    fn quiet_until(end: u64) -> Self {
        Self {
            quiet: core::iter::once(0..end).collect(),
            constant: None,
        }
    }

    fn chunk(&self, start: u64, frames: usize) -> [Vec<f32>; 2] {
        [0, 1].map(|lane| {
            (0..frames as u64)
                .map(|offset| {
                    let frame = start + offset;
                    if self.quiet.iter().any(|range| range.contains(&frame)) {
                        -0.0
                    } else {
                        self.constant.unwrap_or_else(|| source_sample(lane, frame))
                    }
                })
                .collect()
        })
    }
}

/// The parity session with a source long enough for a whole run, under the default
/// `control_smoothing` (the table of #1055).
fn long_session(track_count: usize, sample_rate_hz: u32) -> String {
    long_session_with(track_count, sample_rate_hz, None)
}

/// [`long_session`] with the session's own `control_smoothing`.
fn long_session_with(
    track_count: usize,
    sample_rate_hz: u32,
    control_smoothing: Option<ControlSmoothing>,
) -> String {
    let mut model = parse_session_json(&generated_parity_session(track_count, sample_rate_hz))
        .expect("parity session");
    model.sources[0].frames = SOURCE_FRAMES;
    model.control_smoothing = control_smoothing;
    session::canonical_session_json(&model).expect("canonical long session")
}

/// Every length at 0: the hard switch, the session a bit-comparison against a rebuild authors
/// (#1054 D6).
const HARD_SWITCH: Option<ControlSmoothing> = Some(ControlSmoothing {
    mute_ms: 0.0,
    fader_ms: 0.0,
    pan_ms: 0.0,
});

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
        self.render_counted().0
    }

    /// One block, and the calling thread's allocator events inside the `render` call alone (the
    /// output buffer is allocated before the count starts).
    fn render_counted(&mut self) -> (Vec<f32>, bench_support::alloc::Counters) {
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
        let mark = bench_support::alloc::current_thread_counters();
        let result = crate::ffi::test_render(self.plan, self.block * self.quantum as u64, &output);
        let used = bench_support::alloc::current_thread_delta_since(mark);
        assert_eq!(result, crate::RESULT_OK);
        self.block += 1;
        (pcm, used)
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

    /// How many writes a track's cells have taken in the current or the pending epoch: the fader
    /// lane's four cells together (fader left and right, mute left and right), then the matrix
    /// cell (#1312 D3).
    fn writes(&self, epoch: Epoch, track_id: &str) -> (u64, u64) {
        let snapshot = crate::ffi::test_transaction_snapshot(self.session);
        let epoch = match epoch {
            Epoch::Current => snapshot.provider_epoch,
            Epoch::Pending => *snapshot
                .pending_provider_epochs
                .last()
                .expect("a pending candidate"),
        };
        snapshot
            .strip_cells
            .iter()
            .find(|(row_epoch, id, ..)| *row_epoch == epoch && &**id == track_id)
            .map(|&(_, _, fader, matrix)| (fader.iter().sum(), matrix))
            .expect("track producer")
    }

    /// Every send's cell write count in the current epoch, in route-ID order (#1557).
    fn current_route_cells(&self) -> Vec<u64> {
        let snapshot = crate::ffi::test_transaction_snapshot(self.session);
        snapshot
            .route_cells
            .iter()
            .filter(|(epoch, ..)| *epoch == snapshot.provider_epoch)
            .map(|&(_, _, writes)| writes)
            .collect()
    }

    /// Every strip's cell write counts in the current epoch, per cell: `([fader left, fader
    /// right, mute left, mute right], matrix)`.
    fn current_cells(&self) -> Vec<([u64; 4], u64)> {
        let snapshot = crate::ffi::test_transaction_snapshot(self.session);
        snapshot
            .strip_cells
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
        self.step_feed(&Feed {
            quiet: Vec::new(),
            constant: None,
        })
    }

    /// One block fed from `feed`.
    fn step_feed(&mut self, feed: &Feed) -> Vec<f32> {
        let start = self.block * self.quantum as u64;
        let [left, right] = feed.chunk(start, self.quantum);
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

    /// [`Reference::run`] fed from `feed`.
    fn run_feed(document: &str, blocks: u64, feed: &Feed) -> Vec<Vec<f32>> {
        let mut reference = Self::new(document);
        (0..blocks).map(|_| reference.step_feed(feed)).collect()
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
        ramp_samples: None,
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
/// Each edit ramps over the session's `control_smoothing` and, from the block its ramp and the
/// plan's latency leave behind, renders bit-identically to a rebuild of the committed session.
fn live_pcm_shape(
    track_count: usize,
    sample_rate_hz: u32,
    control_smoothing: Option<ControlSmoothing>,
) {
    let label = format!(
        "{track_count} tracks at {sample_rate_hz} Hz, control_smoothing {control_smoothing:?}"
    );
    let document = long_session_with(track_count, sample_rate_hz, control_smoothing);
    let mut rig = Rig::new(&document);
    let mut original = Reference::new(&document);
    let quantum = rig.quantum;
    let latency = rig.latency();
    let model = parse_session_json(&document).expect("model");
    let ramps = model.control_smoothing_samples();

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
    let k = window(latency, ramps.mute, quantum);
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
    let k = window(latency, ramps.mute, quantum);
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
            ramp_samples: None,
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
    // The matrix carries its own window; the fader ramps over the session's `fader_ms`.
    let settled = settle(
        &mut rig,
        window(latency, smoothing.max(ramps.fader), quantum),
        3,
    );
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
            // The default table ramps; the hard switch steps.
            live_pcm_shape(track_count, sample_rate_hz, None);
            live_pcm_shape(track_count, sample_rate_hz, HARD_SWITCH);
        }
    }
}

/// The stateless session (no console slot, no insert, no input filter, so no latency) with a source
/// long enough for a run, under `control_smoothing`; the same session with every track baked muted
/// is the control a live mute must land on (#1054 gates 4 and 5).
fn mute_ramp_documents(control_smoothing: Option<ControlSmoothing>) -> (String, String) {
    let mut model = super::tests::stateless_session(48_000);
    model.sources[0].frames = SOURCE_FRAMES;
    model.control_smoothing = control_smoothing;
    let live = session::canonical_session_json(&model).expect("canonical");
    for track in &mut model.tracks {
        track.fader.left_mute = true;
        track.fader.right_mute = true;
    }
    let muted = session::canonical_session_json(&model).expect("canonical muted");
    (live, muted)
}

/// The sum of absolute samples of one block: a measure of how far a mute's ramp has closed.
fn magnitude(pcm: &[f32]) -> f64 {
    pcm.iter().map(|sample| f64::from(sample.abs())).sum()
}

/// Gate 4 (#1054). A playing session whose `control_smoothing` is `default` takes a live mute. The
/// first block after the commit equals neither the unmuted nor the muted control, the mute ramp
/// closes block by block, and from the first block after the ramp the output is bit-identical to a
/// control booted muted. The ramp blocks render without one allocation. (The commit itself is the
/// control plane and allocates, as `classify_live_delta` documents; only render is counted.)
///
/// Test value: red if `LiveRamps::for_session` still returns 0 (the mute steps, so the first block
/// is already the muted control), if the ramp is cut short or never lands on its target, or if a
/// ramp block allocates. No other test hears the ramp: the bit-exact ones wait it out.
#[test]
fn a_live_mute_ramps_over_the_session_mute_ramp_and_lands_on_the_muted_control() {
    bench_support::alloc::assert_installed();
    let (document, muted_document) = mute_ramp_documents(None);
    let model = parse_session_json(&document).expect("model");
    assert_eq!(model.control_smoothing, None, "the session says `default`");
    let ramp = model.control_smoothing_samples().mute;
    assert_eq!(ramp, 480, "10 ms at 48 kHz");
    let mut rig = Rig::new(&document);
    let quantum = rig.quantum;
    assert_eq!(rig.latency(), 0, "no latency: the mute lands at the fader");
    assert!(ramp as usize > 2 * quantum, "the ramp spans several blocks");
    let mut unmuted = Reference::new(&document);
    let mut muted = Reference::new(&muted_document);
    for _ in 0..4 {
        let live = rig.step();
        assert_eq!(bits(&live), bits(&unmuted.step()), "warm-up");
        muted.step();
    }

    let (revision, _, epoch, pending) = rig.summary();
    assert_eq!(rig.apply(&mute_all(&model, true)), crate::RESULT_OK);
    assert_eq!(
        rig.summary(),
        (revision + 1, rig.summary().1, epoch, pending),
        "the mute commits one revision and prepares no plan"
    );
    assert_eq!(pending, 0);

    let ramp_blocks = u64::from(ramp).div_ceil(quantum as u64);
    let mut energies = Vec::new();
    for offset in 0..ramp_blocks {
        let [left, right] = source_chunk(rig.fed, quantum);
        submit_c(
            rig.session,
            rig.generation,
            rig.fed,
            rig.rate,
            &left,
            &right,
            false,
        );
        rig.fed += quantum as u64;
        let (live, used) = rig.render_counted();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "ramp block {offset} allocated or freed"
        );
        let loud = unmuted.step();
        let quiet = muted.step();
        if offset == 0 {
            assert_ne!(
                bits(&live),
                bits(&loud),
                "the first block is not the unmuted control"
            );
            assert_ne!(
                bits(&live),
                bits(&quiet),
                "the first block is not the muted control"
            );
        }
        assert!(
            live.iter().any(|sample| *sample != 0.0),
            "ramp block {offset} still sounds"
        );
        energies.push(magnitude(&live));
    }
    assert!(
        energies.windows(2).all(|pair| pair[1] < pair[0]),
        "the mute closes block by block: {energies:?}"
    );

    // From the first block after the ramp: bit-identical to the control booted muted.
    for offset in 0..3 {
        let live = rig.step();
        assert_eq!(
            bits(&live),
            bits(&muted.step()),
            "block {offset} after the ramp is the muted control"
        );
        assert!(
            live.iter().all(|sample| sample.to_bits() == 0),
            "exactly +0.0"
        );
        unmuted.step();
    }
}

/// Gate 5 (#1054). The same edit under `control_smoothing` with every length at 0 is the muted
/// control from the first block.
///
/// Test value: red if a session 0 is read as "use the default" (the first block would still be
/// ramping).
#[test]
fn a_live_mute_under_an_explicit_zero_is_the_muted_control_from_the_first_block() {
    let (document, muted_document) = mute_ramp_documents(HARD_SWITCH);
    let model = parse_session_json(&document).expect("model");
    assert_eq!(model.control_smoothing_samples().mute, 0);
    let mut rig = Rig::new(&document);
    assert_eq!(rig.latency(), 0, "no latency: the mute lands at the fader");
    let mut unmuted = Reference::new(&document);
    let mut muted = Reference::new(&muted_document);
    for _ in 0..4 {
        assert_eq!(bits(&rig.step()), bits(&unmuted.step()), "warm-up");
        muted.step();
    }
    assert_eq!(rig.apply(&mute_all(&model, true)), crate::RESULT_OK);
    for offset in 0..3 {
        let live = rig.step();
        let loud = unmuted.step();
        assert!(
            loud.iter().any(|sample| *sample != 0.0),
            "the unmuted control sounds"
        );
        assert_eq!(
            bits(&live),
            bits(&muted.step()),
            "block {offset} after the commit is the muted control"
        );
    }
}

/// `edits` with every fader edit's `ramp_samples` set to `ramp` (#1394).
fn with_ramp(edits: Vec<SessionEdit>, ramp: Option<u32>) -> Vec<SessionEdit> {
    edits
        .into_iter()
        .map(|mut edit| {
            let SessionEdit::SetTrackFader { ramp_samples, .. } = &mut edit else {
                panic!("a fader edit");
            };
            *ramp_samples = ramp;
            edit
        })
        .collect()
}

impl Rig {
    /// One block fed from `feed`, with the calling thread's allocator events inside the `render`
    /// call.
    fn step_feed(&mut self, feed: &Feed) -> (Vec<f32>, bench_support::alloc::Counters) {
        let [left, right] = feed.chunk(self.fed, self.quantum);
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
        self.render_counted()
    }

    /// [`Rig::step`] with the calling thread's allocator events inside the `render` call.
    fn step_counted(&mut self) -> (Vec<f32>, bench_support::alloc::Counters) {
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
        self.render_counted()
    }
}

/// Gate 6 (#1394). On a playing session with the default table, a mute transaction whose edits
/// carry `ramp_samples = 0` is bit-identical to a control booted muted from the first block. The
/// same edit with `ramp_samples = 96` equals neither control on the first block and equals the
/// muted control from the first block that begins 96 frames or more after the commit. A following
/// transaction with no length ramps over the session's 480 again: no block that begins before 480
/// frames equals the unmuted control, the first that begins at 480 does. Every ramp block renders
/// with no allocation.
///
/// Test value: red if `commit_live` passes no edit ramps (the 0 would ramp over 480 and the 96 over
/// 480), if an explicit 0 is read as "default", or if a length leaks into the next transaction (the
/// unmute would settle after 96 frames).
#[test]
fn a_live_edits_own_ramp_length_overrides_the_session_and_does_not_outlive_its_transaction() {
    bench_support::alloc::assert_installed();
    let (document, muted_document) = mute_ramp_documents(None);
    let model = parse_session_json(&document).expect("model");
    assert_eq!(
        model.control_smoothing_samples().mute,
        480,
        "the default table"
    );
    let warmed = |document: &str| {
        let mut rig = Rig::new(document);
        assert_eq!(rig.latency(), 0, "no latency: a mute lands at the fader");
        let mut unmuted = Reference::new(document);
        let mut muted = Reference::new(&muted_document);
        for _ in 0..4 {
            assert_eq!(bits(&rig.step()), bits(&unmuted.step()), "warm-up");
            muted.step();
        }
        (rig, unmuted, muted)
    };

    // An explicit 0 on the edit steps it, though the session says 480.
    let (mut rig, mut unmuted, mut muted) = warmed(&document);
    assert_eq!(
        rig.apply(&with_ramp(mute_all(&model, true), Some(0))),
        crate::RESULT_OK
    );
    for offset in 0..3 {
        let live = rig.step();
        assert!(unmuted.step().iter().any(|sample| *sample != 0.0));
        assert_eq!(
            bits(&live),
            bits(&muted.step()),
            "block {offset} after an explicit 0 is the muted control"
        );
    }

    // 96 frames: neither control on the first block, the muted control once 96 frames are past.
    let (mut rig, mut unmuted, mut muted) = warmed(&document);
    let quantum = rig.quantum as u64;
    assert_eq!(
        rig.apply(&with_ramp(mute_all(&model, true), Some(96))),
        crate::RESULT_OK
    );
    for offset in 0..4_u64 {
        let (live, used) = rig.step_counted();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "ramp block {offset} allocated or freed"
        );
        let (loud, quiet) = (unmuted.step(), muted.step());
        if offset * quantum < 96 {
            assert_ne!(
                bits(&live),
                bits(&loud),
                "block {offset} has left the unmuted control"
            );
            assert_ne!(
                bits(&live),
                bits(&quiet),
                "block {offset} has not reached the muted control"
            );
        } else {
            assert_eq!(
                bits(&live),
                bits(&quiet),
                "block {offset} begins 96 frames or more after"
            );
        }
    }

    // The next transaction carries no length: the session's 480 again, not the previous 96.
    assert_eq!(
        rig.apply(&with_ramp(mute_all(&model, false), None)),
        crate::RESULT_OK
    );
    let settled = 480_u64.div_ceil(quantum);
    assert!(
        settled > 96_u64.div_ceil(quantum) + 1,
        "480 and 96 frames settle in different blocks"
    );
    for offset in 0..settled + 2 {
        let (live, used) = rig.step_counted();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "unmute block {offset} allocated or freed"
        );
        let (loud, _) = (unmuted.step(), muted.step());
        if offset * quantum < 480 {
            assert_ne!(
                bits(&live),
                bits(&loud),
                "block {offset} begins {} frames after the commit and is still ramping",
                offset * quantum
            );
        } else {
            assert_eq!(
                bits(&live),
                bits(&loud),
                "block {offset} begins 480 frames or more after"
            );
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
    // A hard switch (#1054 D6): the live mute steps one block before the rebuild and the baked
    // mute is the rebuild's own retarget, so with a ramp the two would start it a block apart.
    rebuild_run(&long_session_with(10, 48_000, HARD_SWITCH), mute, None)
}

/// [`persistence_run`] on `document`, with `ramp` as the `ramp_samples` of the mute edits.
fn rebuild_run(document: &str, mute: Mute, ramp: Option<u32>) -> Vec<Vec<f32>> {
    let mut rig = Rig::new(document);
    let model = parse_session_json(document).expect("model");
    let muted: Vec<SessionEdit> = with_ramp(
        mute_all(&model, true).into_iter().step_by(2).collect(),
        ramp,
    );
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

/// #1394 D6 on a rebuild: the plan a content edit rebuilds carries each strip and retargets it to
/// the transaction's values, over the length the transaction's own edit gave (an explicit 0 steps),
/// else the session's. On the default session (mute 480 samples), a rebuild whose mute edits say
/// `ramp_samples` 0 sounds exactly like the same rebuild on a session that steps every ramp; the
/// same edits with no length ramp, and sound otherwise.
///
/// Test value: red if the control plane prepares the successor without the transaction's edit
/// ramps (`SuccessorBase::edit_ramps`): the explicit 0 would ramp over the session's 480 and differ
/// from the step. The host-core join test cannot see that, and the live-path test never rebuilds.
#[test]
fn a_rebuilds_carried_strip_retargets_over_its_edits_own_ramp() {
    let blocks = |run: Vec<Vec<f32>>| run.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>();
    let default_session = long_session(10, 48_000);
    let step_session = long_session_with(10, 48_000, HARD_SWITCH);
    let stepped = blocks(rebuild_run(&step_session, Mute::Baked, None));
    let explicit_zero = blocks(rebuild_run(&default_session, Mute::Baked, Some(0)));
    let session_ramp = blocks(rebuild_run(&default_session, Mute::Baked, None));
    assert_eq!(
        explicit_zero, stepped,
        "an explicit 0 on the edit steps the carried strip's retarget, whatever the session says"
    );
    assert_ne!(
        session_ramp, stepped,
        "with no length the same retarget ramps over the session's"
    );
}

/// Gate 4 (#1365), the rebuild leg ("batch verdict 2, m1"): one transaction changes
/// `control_smoothing`, rebuilds the plan with a content edit and retargets the carried strips: a
/// mute on the even tracks, a fader level on the odd ones, a matrix on the last. Three blocks and
/// one more run first; the swap, a seek, and six blocks follow, with no allocation or free in any
/// of them. `smoothing` is either put into that transaction (the session booted on the default
/// table) or already in the document (the same transaction without the edit).
fn control_smoothing_rebuild_run(
    smoothing: ControlSmoothing,
    in_transaction: bool,
) -> Vec<Vec<f32>> {
    let base = long_session(10, 48_000);
    let document = if in_transaction {
        base
    } else {
        long_session_with(10, 48_000, Some(smoothing))
    };
    let mut rig = Rig::new(&document);
    let model = parse_session_json(&document).expect("model");
    for _ in 0..4 {
        rig.step();
    }
    let mut edits = Vec::new();
    if in_transaction {
        edits.push(SessionEdit::SetControlSmoothing {
            control_smoothing: Some(smoothing),
        });
    }
    edits.push(content_edit(&model));
    let last = model.tracks.len() - 1;
    for (index, track) in model.tracks.iter().enumerate() {
        let id = track.id.as_str();
        if index % 2 == 0 {
            edits.push(fader_edit(
                id,
                DualMonoFader {
                    left_mute: true,
                    right_mute: true,
                    ..track.fader.clone()
                },
            ));
        } else {
            edits.push(fader_edit(
                id,
                DualMonoFader {
                    left_db: -6.0,
                    right_db: 3.0,
                    ..track.fader.clone()
                },
            ));
        }
    }
    edits.push(SessionEdit::SetTrackMatrixOrPan {
        track_id: model.tracks[last].id.clone(),
        matrix_or_pan: MatrixOrPan::Matrix {
            ll: 0.75,
            lr: 0.125,
            rl: -0.25,
            rr: 0.5,
            smoothing_samples: 0,
        },
        ramp_samples: None,
    });
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "a content edit prepares a candidate");
    let (_, swap_used) = rig.render_counted();
    assert_eq!(
        (swap_used.allocations, swap_used.deallocations),
        (0, 0),
        "the swap block allocated or freed"
    );
    rig.seek_to_start();
    let rebuilt = (0..6)
        .map(|block| {
            let (pcm, used) = rig.step_counted();
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "rebuilt block {block} allocated or freed"
            );
            pcm
        })
        .collect();
    assert_eq!(
        crate::ffi::test_plan_carry_counts(rig.plan),
        (1, 0),
        "the swap carried the input sections, so a fresh compile is no oracle"
    );
    rebuilt
}

/// Gate 4 (#1365), the rebuild leg ("batch verdict 2, m1"): a transaction that changes
/// `control_smoothing` and rebuilds the plan retargets its carried strips (mute, fader level and
/// matrix) over that transaction's new lengths. Putting the hard switch into the rebuilding
/// transaction sounds exactly like the same transaction on a session that already says hard switch,
/// and not like the same rebuild under the default table.
///
/// Test value: red if the successor's carry retargets are built from the pre-commit model's
/// lengths (`LiveRamps::for_session(committed)` in `join_strips`), so a rebuild transaction's own
/// `control_smoothing` takes effect one transaction late. Gate 4's other tests commit live and
/// never rebuild, and the join tests in host-core pass the lengths in.
#[test]
fn a_control_smoothing_edit_governs_the_retargets_of_its_own_rebuild() {
    bench_support::alloc::assert_installed();
    let blocks = |run: Vec<Vec<f32>>| run.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>();
    let hard = HARD_SWITCH.expect("the hard switch");
    let in_transaction = blocks(control_smoothing_rebuild_run(hard, true));
    let in_document = blocks(control_smoothing_rebuild_run(hard, false));
    assert_eq!(
        in_transaction, in_document,
        "the transaction's own lengths govern its rebuild's retargets"
    );
    let default_table = blocks(control_smoothing_rebuild_run(
        ControlSmoothing {
            mute_ms: 10.0,
            fader_ms: 20.0,
            pan_ms: 20.0,
        },
        true,
    ));
    assert_ne!(
        in_transaction, default_table,
        "the default table ramps those retargets, so the comparison is not vacuous"
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

impl Rig {
    /// One counter of `COUNTERS_GET`, read through the C command path.
    fn counter(&mut self, id: protocol::CounterId) -> u64 {
        let request = self.next_request();
        let bytes = command_bytes(
            request,
            protocol::CommandPayload::CountersGet(&protocol::CountersRequest {
                all: false,
                ids: vec![id as u32],
            }),
        );
        let (result, response) = command_c(self.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("counters response")
        else {
            panic!("expected a counter snapshot")
        };
        snapshot
            .values
            .iter()
            .find(|value| value.id == id)
            .expect("the counter is in the snapshot")
            .value
    }
}

/// Gate 1: a paused host sends forty fader edits, each a `Both` record, and none is refused for
/// room. One render applies the last, so the output equals a twin that made only that edit, and
/// `COUNTERS_GET` reports `LIVE_VALUES_SUPERSEDED` as 78: the 39 writes each of the two channel
/// cells that a later write replaced before a block read them (D2).
///
/// Test value: red if a fader lane is still a bounded queue (the seventeenth edit refuses), or if
/// the counter's unit is not the per-cell replaced-write count.
#[test]
fn forty_fader_edits_before_one_block_are_all_accepted_and_the_last_is_heard() {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    let mut quiet = Rig::new(&document);
    rig.step();
    twin.step();
    quiet.step();
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    let both = |db: f32| {
        fader_edit(
            "eq0",
            DualMonoFader {
                left_db: db,
                right_db: db,
                left_mute: false,
                right_mute: false,
            },
        )
    };
    let last = -1.0 - 39.0 * 0.25;
    for edit in 0..40 {
        assert_eq!(
            rig.apply(&[both(-1.0 - edit as f32 * 0.25)]),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (80, 0));
    assert_eq!(twin.apply(&[both(last)]), crate::RESULT_OK);
    // The counter is the render side's: nothing has been read yet.
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    // The plan's latency delays the first audible block; render past it.
    let blocks = u64::from(rig.latency()) / rig.quantum as u64 + 4;
    let mut audible = false;
    for block in 0..blocks {
        let heard = rig.step();
        assert_eq!(
            bits(&heard),
            bits(&twin.step()),
            "block {block}: the last edit is heard"
        );
        audible |= bits(&heard) != bits(&quiet.step());
        if block == 0 {
            assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 78);
        }
    }
    assert!(
        audible,
        "the edit changes the output against an unedited run"
    );
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 78);
    assert_eq!(twin.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    // Nothing is pending now: another block replaces nothing.
    rig.step();
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 78);
}

/// Gate 3 (c): an exact replay of a live edit returns the cached response and writes nothing.
#[test]
fn a_replayed_live_edit_writes_nothing() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let edits = [left_db_edit("eq2", -4.5)];
    let bytes = rig.transaction(&edits);
    let (result, first) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    let writes = rig.writes(Epoch::Current, "eq2");
    assert_eq!(writes, (1, 0), "the edit wrote its one cell");
    let cells = rig.current_cells();
    let revision = rig.summary().0;
    let (result, replay) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(replay, first, "the replay is the cached response");
    assert_eq!(rig.writes(Epoch::Current, "eq2"), writes);
    assert_eq!(rig.current_cells(), cells, "no cell took a write");
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
    assert_eq!(rig.writes(Epoch::Pending, "eq3"), (0, 0));

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
            ramp_samples: None,
        },
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 1, "still the one candidate");
    // Two fader cells (left and right dB differ), one mute cell (right only), one matrix cell.
    assert_eq!(rig.writes(Epoch::Pending, "eq3"), (3, 1));
    assert_eq!(rig.writes(Epoch::Current, "eq3"), (0, 0));

    rig.render();
    rig.seek_to_start();
    // The candidate ramps its fader and mute from their prepared values over the session's ramps
    // and its pan over its own 8-sample window; the control starts at the target.
    let ramps = model.control_smoothing_samples();
    let k = window(
        rig.latency(),
        ramps.fader.max(ramps.mute).max(8),
        rig.quantum,
    );
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

/// The plan's watermark as a host reads it: `(revision, first sample, flags, exact count)`.
fn watermark(rig: &Rig) -> (u64, u64, u64, u64) {
    let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        (
            watermark.transition_fallback_count,
            watermark.superseded_count
        ),
        (0, 0),
        "this slice completes every revision exactly"
    );
    (
        watermark.revision,
        watermark.first_sample,
        watermark.outcome_flags,
        watermark.exact_count,
    )
}

/// #1314 gate 2: a rebuild (r1) and then a live fader edit (r2), committed before any render,
/// both complete at the swap block and not one block earlier: the live edit went to the pending
/// candidate's revision gate, so the running plan never reports either revision.
#[test]
fn a_live_edit_on_a_pending_candidate_completes_at_the_swap() {
    let document = long_session(1, 48_000);
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    let initial = rig.summary().0;
    for _ in 0..3 {
        rig.step();
        assert_eq!(watermark(&rig), (initial, 0, crate::OUTCOME_EXACT, 0));
    }
    assert_eq!(rig.apply(&[content_edit(&model)]), crate::RESULT_OK);
    let r1 = rig.summary().0;
    assert_eq!(rig.summary().3, 1, "a candidate is pending");
    let edit = fader_edit(
        "eq0",
        DualMonoFader {
            left_db: -6.0,
            right_db: -6.0,
            left_mute: false,
            right_mute: false,
        },
    );
    assert_eq!(rig.apply(&[edit]), crate::RESULT_OK);
    let r2 = rig.summary().0;
    assert_eq!((r1, r2), (initial + 1, initial + 2));
    assert_eq!(
        rig.summary().3,
        1,
        "the live edit went to the one candidate"
    );
    assert_eq!(watermark(&rig), (initial, 0, crate::OUTCOME_EXACT, 0));

    let k = rig.block;
    rig.render();
    assert_eq!(
        watermark(&rig),
        (r2, k * rig.quantum as u64, crate::OUTCOME_EXACT, 2)
    );

    // A rebuild alone carries its own revision to its swap.
    rig.render();
    let source = &model.sources[0];
    let edit = SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content: format!("blake3:{}", "cd".repeat(32)),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    };
    assert_eq!(rig.apply(&[edit]), crate::RESULT_OK);
    let r3 = rig.summary().0;
    assert_eq!(rig.summary().3, 1, "a second candidate is pending");
    assert_eq!(
        watermark(&rig),
        (r2, k * rig.quantum as u64, crate::OUTCOME_EXACT, 2)
    );
    let swap = rig.block;
    rig.render();
    assert_eq!(
        watermark(&rig),
        (r3, swap * rig.quantum as u64, crate::OUTCOME_EXACT, 3)
    );
}

/// #1314 gate 3: on one thread, a live edit committed between blocks 3 and 4 completes in block 4,
/// from its first sample, and block 5 publishes nothing new.
#[test]
fn a_live_edit_completes_in_the_next_block() {
    let document = long_session(1, 48_000);
    let mut rig = Rig::new(&document);
    let initial = rig.summary().0;
    for _ in 0..4 {
        rig.step();
    }
    assert_eq!(watermark(&rig), (initial, 0, crate::OUTCOME_EXACT, 0));
    let edit = fader_edit(
        "eq0",
        DualMonoFader {
            left_db: -3.0,
            right_db: -3.0,
            left_mute: false,
            right_mute: false,
        },
    );
    assert_eq!(rig.apply(&[edit]), crate::RESULT_OK);
    let r = rig.summary().0;
    assert_eq!(rig.summary().3, 0, "the edit was live");
    assert_eq!(watermark(&rig), (initial, 0, crate::OUTCOME_EXACT, 0));
    assert_eq!(rig.block, 4);
    rig.step();
    let expected = (r, 4 * rig.quantum as u64, crate::OUTCOME_EXACT, 1);
    assert_eq!(expected.1, 512);
    assert_eq!(watermark(&rig), expected);
    rig.step();
    assert_eq!(watermark(&rig), expected);
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
    // The claim is bit-identity to a fresh compile from the swap block on, so the live fader is a
    // step (#1054 D6); a ramp would still be on its way.
    model.control_smoothing = HARD_SWITCH;
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
    let mut rig = Rig::new(&document);
    for _ in 0..4 {
        rig.step();
    }
    let swap = rig.block;
    if live_first {
        assert_eq!(rig.apply(&live), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 0, "{label}: the fader edit is live");
        assert_eq!(
            rig.writes(Epoch::Current, "eq0"),
            (2, 0),
            "{label}: both fader cells took the write"
        );
        assert_eq!(rig.apply(&structural), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 1, "{label}: the transaction rebuilds");
        assert_eq!(
            rig.writes(Epoch::Pending, "eq0"),
            (0, 0),
            "{label}: the successor bakes -6 dB and inherits no write"
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
        assert_eq!(rig.writes(Epoch::Pending, "eq0"), (2, 0), "{label}");
        assert_eq!(rig.writes(Epoch::Current, "eq0"), (0, 0), "{label}");
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
    assert!(
        rig.current_cells()
            .iter()
            .all(|&cells| cells == ([0; 4], 0)),
        "{label}: the rendering plan got no write"
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

/// Gate 4: every delta outside the live set rebuilds, and a domain failure writes no cell. (A VCA
/// and a followed mute left the set in #1558.)
#[test]
fn deltas_outside_the_live_set_rebuild_and_a_domain_failure_writes_nothing() {
    let base = long_session(10, 48_000);
    let quiet = DualMonoFader {
        left_db: -2.0,
        right_db: -2.0,
        left_mute: false,
        right_mute: false,
    };

    // A VCA in the session no longer forces a rebuild (#1558), nor does a mute that a send follows.
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
    let followed = submix_session(true);
    let muted = DualMonoFader {
        left_mute: true,
        ..parse_session_json(&followed).expect("model").tracks[0]
            .fader
            .clone()
    };
    for (document, edit, label) in [
        (&vca, fader_edit("eq0", quiet.clone()), "a VCA"),
        (
            &followed,
            fader_edit("eq0", muted.clone()),
            "a followed mute",
        ),
        (
            &submix_session(false),
            fader_edit("eq0", muted.clone()),
            "an unfollowed mute",
        ),
    ] {
        let mut rig = Rig::new(document);
        rig.step();
        assert_eq!(rig.apply(&[edit]), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 0, "{label} is live");
    }
    let unfollowed = submix_session(false);

    // A submix strip's fader is live since #1554; `set_control::submix_strips_take_live_edits`.
    assert_rebuilds(
        &unfollowed,
        &[
            fader_edit("eq0", quiet.clone()),
            content_edit(&parse_session_json(&unfollowed).expect("model")),
        ],
        "a live fader beside a source content edit",
    );

    let model = parse_session_json(&base).expect("model");
    assert_rebuilds(&base, &[content_edit(&model)], "a source content edit");

    // A fader outside the setter's domain is today's compile rejection, and writes nothing.
    let mut rig = Rig::new(&base);
    rig.step();
    let before = refusal_state(&rig);
    assert_eq!(
        rig.apply(&[left_db_edit("eq0", 30.0)]),
        crate::RESULT_COMPILE_REJECTED
    );
    let rejected = with_model(&base, |model| model.tracks[0].fader.left_db = 30.0);
    let Err(failure) = compile_children(&rejected, limits(), None) else {
        panic!("a 30 dB fader does not prepare")
    };
    assert_eq!(
        rig.last_error(),
        failure.diagnostics,
        "the live arm reports today's preparation diagnostic"
    );
    assert_eq!(refusal_state(&rig), before);
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (0, 0));
    assert_eq!(rig.summary().3, 0);
}

/// Gate 2: a refusal after the commit predicate and before the write leaves every cell, the
/// output and the model alone. A cell write is acked state once its revision is published: a write
/// made before the last fallible step would be revealed by the next transaction's publication
/// (the stamp's revision is not consumed by a refusal), so the next transaction here is a
/// different edit, and the output must show that edit alone.
///
/// Red mutation: the strip cell writes moved above the commit predicate (#1312 D7).
#[test]
fn a_fault_before_the_live_write_leaves_every_cell_and_the_output_alone() {
    let document = long_session(10, 48_000);
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    rig.step();
    twin.step();
    let before = refusal_state(&rig);
    let cells = rig.current_cells();
    // A fader edit and a pan edit: two lanes, three cells.
    let refused = [
        left_db_edit("eq0", -9.0),
        SessionEdit::SetTrackMatrixOrPan {
            track_id: StableId::parse("eq3").expect("eq3"),
            matrix_or_pan: MatrixOrPan::Pan {
                left: -0.75,
                right: 0.25,
                smoothing_samples: 0,
            },
            ramp_samples: None,
        },
    ];
    crate::ffi::test_set_structural_faults(
        rig.session,
        [Some(TestStructuralFaultPhase::BeforeLivePush), None],
    );
    assert_eq!(rig.apply(&refused), crate::RESULT_BACKPRESSURE);
    assert_eq!(refusal_state(&rig), before);
    assert_eq!(rig.current_cells(), cells, "no cell took a write");
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (0, 0));
    assert_eq!(rig.writes(Epoch::Current, "eq3"), (0, 0));

    // A different edit then commits, and only it is heard: the twin takes it alone.
    let next = [left_db_edit("eq1", -3.0)];
    assert_eq!(rig.apply(&next), crate::RESULT_OK);
    assert_eq!(twin.apply(&next), crate::RESULT_OK);
    assert_eq!(rig.writes(Epoch::Current, "eq1"), (1, 0));
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (0, 0));
    assert_eq!(rig.writes(Epoch::Current, "eq3"), (0, 0));
    for block in 0..6 {
        assert_eq!(
            bits(&rig.step()),
            bits(&twin.step()),
            "block {block}: the output is the twin's, which took the second edit alone"
        );
    }
}

/// #1277 gate 8 (Amendment 2, strengthened by Amendment 3). A rebuild whose successor retargets a
/// carried strip (track `eq0` gains -6 dB while a track is added) writes the retarget into the
/// candidate's cells, stamped with the structural revision, inside preparation. It is then refused
/// at `BeforeProtocolCommit`, which drops the candidate, its provider epoch, its gate handle and
/// its cells. A refused attempt uses no revision, so the next one stamps the same revision again.
/// That next attempt is the same structural change *without* `eq0`'s fader change, committed on the
/// twin as well: if the first candidate, a cell of it or its stamp were kept and reused, its -6 dB
/// would be heard or a debug assertion would fire. The output is the twin's, which never saw the
/// fader change, in every block. (Were the accepted rebuild the same edit, a kept retarget would
/// be inaudible.) A third rig accepts the first transaction, so the retarget is audible.
///
/// Red mutation: the refused candidate's plan, cells or stamp kept and reused by the next
/// rebuild; the control plane takes the revision again for the same plan.
#[test]
fn a_refused_rebuild_with_a_carry_retarget_is_never_reused() {
    let mut model = super::tests::stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.sources[0].frames = SOURCE_FRAMES;
    let fader = model.tracks[0].fader.clone();
    let document = session::canonical_session_json(&model).expect("canonical");
    let structural = super::tests::add_muted_track(&model, "a-muted");
    let retargeted = [
        structural[0].clone(),
        structural[1].clone(),
        fader_edit(
            "eq0",
            DualMonoFader {
                left_db: -6.0,
                right_db: -6.0,
                ..fader
            },
        ),
    ];
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    let mut accepted = Rig::new(&document);
    for _ in 0..4 {
        rig.step();
        twin.step();
        accepted.step();
    }
    let before = refusal_state(&rig);
    crate::ffi::test_set_structural_faults(
        rig.session,
        [Some(TestStructuralFaultPhase::BeforeProtocolCommit), None],
    );
    assert_eq!(rig.apply(&retargeted), crate::RESULT_BACKPRESSURE);
    assert_eq!(refusal_state(&rig), before, "the refusal changed nothing");
    assert_eq!(rig.summary().3, 0, "no candidate is pending");

    assert_eq!(rig.apply(&structural), crate::RESULT_OK);
    assert_eq!(twin.apply(&structural), crate::RESULT_OK);
    assert_eq!(accepted.apply(&retargeted), crate::RESULT_OK);
    let mut audible = false;
    for block in 0..8 {
        let rendered = rig.step();
        assert_eq!(
            bits(&rendered),
            bits(&twin.step()),
            "block {block}: the output is the twin's, which never saw the fader change"
        );
        audible |= bits(&rendered) != bits(&accepted.step());
    }
    assert!(
        audible,
        "vacuous: the retarget the refused attempt wrote is audible when accepted"
    );
}

/// Gate 1 on a session with a submix strip: the producer table holds the tracks, then the
/// submixes, and a live track edit there still reaches its track by ID and renders like a rebuild.
#[test]
fn a_live_track_edit_beside_a_submix_strip_reaches_its_track() {
    let document = submix_session(false);
    let mut rig = Rig::new(&document);
    let quantum = rig.quantum;
    let latency = rig.latency();
    for _ in 0..3 {
        rig.step();
    }
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let strips: Vec<_> = snapshot
        .strip_cells
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
            ramp_samples: None,
        },
    ];
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(rig.writes(Epoch::Current, "eq9"), (2, 0));
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (0, 1));
    assert_eq!(rig.writes(Epoch::Current, "bus"), (0, 0));

    let committed = rig.snapshot();
    let fader_ramp = rig.model().control_smoothing_samples().fader;
    let settled = settle(
        &mut rig,
        window(latency, smoothing.max(fader_ramp), quantum),
        3,
    );
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
/// `Backpressure` response under `RESULT_OK`, decided before a token exists, so it writes no cell.
///
/// Test value: red if the live arm writes its cells before the reliable-event capacity check (a
/// refused transaction's value would be heard with no commit), or if that check stops refusing a
/// live transaction.
#[test]
fn a_live_edit_without_reliable_event_room_is_protocol_backpressure_and_writes_nothing() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    // Two live edits, never dequeued: each commits one `SESSION_COMMITTED` event.
    for db in [-1.0, -2.0] {
        let bytes = rig.transaction(&[left_db_edit("eq0", db)]);
        assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    }
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (2, 0));
    let cells = crate::ffi::test_transaction_snapshot(rig.session).strip_cells;
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
            ramp_samples: None,
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
        crate::ffi::test_transaction_snapshot(rig.session).strip_cells,
        cells,
        "no strip cell took a write"
    );

    // Once the events are dequeued, the same edit under a new request ID commits live.
    rig.drain_reliable();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(rig.summary().0, revision + 1);
    assert_eq!(rig.writes(Epoch::Current, "eq1"), (1, 0));
    assert_eq!(rig.writes(Epoch::Current, "eq2"), (0, 1));
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
    let mut children = compile_children(SESSION, limits(), None).expect("children");
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
    let writes: Vec<_> = snapshot
        .strip_cells
        .iter()
        .filter(|(_, id, ..)| &**id == "eq0")
        .map(|&(epoch, _, fader, matrix)| (epoch, fader.iter().sum::<u64>(), matrix))
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
            writes,
        ),
        (
            Ok(()),
            revision + 2,
            old_epoch,
            new_epoch,
            Vec::new(),
            vec![(new_epoch, 1, 0)],
        ),
        "the live edit commits inside the window, prepares no candidate, and takes one write \
         in the promoted provider's fader cells"
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
    let (entries, ..) = control_list_all_c(rig.session, &mut rig.request, 64);
    let record = protocol::AutomationRecord {
        kind: protocol::AutomationKind::Point,
        handle: protocol::ParameterHandle(automation_target(&entries, 100.0)),
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

/// The model-only edits: #1260 D1's four and #1365's `control_smoothing`, each a change no
/// prepared plan reads.
fn model_only_edits(model: &SessionModel) -> [(&'static str, SessionEdit); 5] {
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
        (
            "control smoothing",
            SessionEdit::SetControlSmoothing {
                control_smoothing: Some(ControlSmoothing {
                    mute_ms: 1.0,
                    fader_ms: 2.0,
                    pan_ms: 3.0,
                }),
            },
        ),
    ]
}

/// #1260 gate 2, #1365 gate 3: a session ID, render-profile ID, output-profile ID, stored
/// automation or `control_smoothing` edit commits one revision with one `SESSION_COMMITTED`, shows
/// in the snapshot, and leaves the running plan, its source ring and its output untouched: the
/// host keeps feeding with no seek, and every block is bit-identical to an uninterrupted render of
/// the original session.
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

/// `SetControlSmoothing` with the given `mute_ms` and the default table's other two lengths.
fn mute_smoothing_edit(mute_ms: f32) -> SessionEdit {
    SessionEdit::SetControlSmoothing {
        control_smoothing: Some(ControlSmoothing {
            mute_ms,
            fader_ms: 20.0,
            pan_ms: 20.0,
        }),
    }
}

/// Gate 4 (#1365), the step. Under the default table (a 10 ms mute ramp), `mute_ms` set to 0 by a
/// transaction makes the next mute a step, whether the lengths arrive one transaction ahead of the
/// mute or in the same one: the first block after the commit is the control booted muted, and the
/// length edit alone moves no bit.
///
/// Test value: red if the commit builds its `LiveRamps` from the pre-commit model, so the mute of
/// the same transaction still ramps over the old 10 ms (the "same transaction" row), or if the
/// edit does not reach the model the next commit reads (the "one transaction ahead" row).
#[test]
fn a_control_smoothing_edit_makes_the_next_mute_a_step() {
    for same_transaction in [false, true] {
        let label = if same_transaction {
            "in the mute's transaction"
        } else {
            "one transaction ahead"
        };
        let (document, muted_document) = mute_ramp_documents(None);
        let model = parse_session_json(&document).expect("model");
        assert_eq!(model.control_smoothing_samples().mute, 480, "{label}");
        let mut rig = Rig::new(&document);
        assert_eq!(rig.latency(), 0, "{label}: the mute lands at the fader");
        let mut unmuted = Reference::new(&document);
        let mut muted = Reference::new(&muted_document);
        for _ in 0..4 {
            assert_eq!(bits(&rig.step()), bits(&unmuted.step()), "{label}: warm-up");
            muted.step();
        }
        let mut edits = vec![mute_smoothing_edit(0.0)];
        if same_transaction {
            edits.extend(mute_all(&model, true));
        } else {
            let (revision, _, epoch, _) = rig.summary();
            assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
            assert_eq!(
                (rig.summary().0, rig.summary().2, rig.summary().3),
                (revision + 1, epoch, 0),
                "{label}: the length edit commits and prepares no plan"
            );
            assert_eq!(
                rig.model().control_smoothing_samples().mute,
                0,
                "{label}: the committed model holds the step"
            );
            // The length edit alone moves no bit.
            assert_eq!(bits(&rig.step()), bits(&unmuted.step()), "{label}");
            muted.step();
            edits = mute_all(&model, true);
        }
        assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 0, "{label}: the mute is live");
        for offset in 0..3 {
            let live = rig.step();
            assert!(
                unmuted.step().iter().any(|sample| *sample != 0.0),
                "{label}: the unmuted control sounds"
            );
            assert_eq!(
                bits(&live),
                bits(&muted.step()),
                "{label}: block {offset} after the commit is the muted control"
            );
        }
    }
}

/// Gate 4 (#1365), the ramp. A session booted muted under the hard switch takes `mute_ms` 20 and an
/// unmute in one transaction. The first block after the commit is ramping, and every block that
/// starts inside the 960-sample ramp is neither the muted nor the unmuted control, which the
/// default 10 ms would have already reached; from the first block after the ramp it is the
/// unmuted control, bit for bit.
///
/// Test value: red if the new length is read one transaction late (the unmute steps, since the
/// session before the commit says 0), if the key is read as the default table, or if the ramp
/// does not land on the control.
#[test]
fn a_control_smoothing_edit_gives_the_next_unmute_its_ramp() {
    bench_support::alloc::assert_installed();
    let (document, muted_document) = mute_ramp_documents(HARD_SWITCH);
    let model = parse_session_json(&muted_document).expect("muted model");
    assert_eq!(model.control_smoothing_samples().mute, 0);
    let mut rig = Rig::new(&muted_document);
    let quantum = rig.quantum;
    assert_eq!(
        rig.latency(),
        0,
        "no latency: the unmute lands at the fader"
    );
    let mut unmuted = Reference::new(&document);
    let mut muted = Reference::new(&muted_document);
    for _ in 0..4 {
        assert_eq!(bits(&rig.step()), bits(&muted.step()), "warm-up");
        unmuted.step();
    }
    let mut edits = vec![mute_smoothing_edit(20.0)];
    edits.extend(mute_all(&model, false));
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    assert_eq!(
        (rig.summary().0, rig.summary().2, rig.summary().3),
        (revision + 1, epoch, 0),
        "one revision, no plan rebuilt"
    );
    let ramp = 960_usize;
    assert!(ramp > 4 * quantum, "the ramp spans several blocks");
    let mut start = 0;
    while start < ramp {
        let [left, right] = source_chunk(rig.fed, quantum);
        submit_c(
            rig.session,
            rig.generation,
            rig.fed,
            rig.rate,
            &left,
            &right,
            false,
        );
        rig.fed += quantum as u64;
        let (live, used) = rig.render_counted();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "ramp block at {start} allocated or freed"
        );
        let (loud, quiet) = (unmuted.step(), muted.step());
        assert_ne!(
            bits(&live),
            bits(&loud),
            "block at {start} is still ramping"
        );
        assert_ne!(
            bits(&live),
            bits(&quiet),
            "block at {start} has left the mute"
        );
        start += quantum;
    }
    for offset in 0..3 {
        let live = rig.step();
        assert_eq!(
            bits(&live),
            bits(&unmuted.step()),
            "block {offset} after the ramp is the unmuted control"
        );
        assert!(live.iter().any(|sample| *sample != 0.0), "it sounds");
    }
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
    /// The plan's gate handle: the effects' values are cells, written under a stamp of it.
    gate: engine::realtime::PlanRevisionGate,
    effects: Vec<host_core::EffectControlProducer>,
}

impl LaneReference {
    fn new(document: &str) -> Self {
        let caps = prepare_caps(limits());
        let compiled = host_core::compile_host_session(document, &caps).expect("lane compile");
        let (mut host, handles) = prepare_host_runtime_with_live_lanes(
            &compiled,
            &caps,
            &HostLiveControlRequest::default(),
            HostLiveLanes::ALL,
            None,
            &host_core::LiveValuesSuperseded::default(),
        )
        .expect("lane prepare");
        let gate = host
            .plan
            .take_revision_gate()
            .expect("the plan's gate handle");
        Self {
            gate,
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
        let Self { effects, gate, .. } = self;
        let producer = effects
            .iter_mut()
            .find(|producer| &*producer.track_id == track_id && &*producer.effect_id == effect_id)
            .expect("effect producer");
        let parameter_index = producer
            .descriptor
            .parameters
            .iter()
            .position(|parameter| parameter.id.0 == parameter_id)
            .expect("declared parameter") as u32;
        Self::write_cell(
            producer,
            gate,
            effect_contract::EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value,
            },
        );
    }

    /// One transaction on `producer`: stamp the next revision on the plan's gate, write `record`
    /// to its cell, publish the revision.
    fn write_cell(
        producer: &mut host_core::EffectControlProducer,
        gate: &mut engine::realtime::PlanRevisionGate,
        record: effect_contract::EffectControlRecord,
    ) {
        let revision = gate.last_published() + 1;
        let stamp = gate.stamp(revision).expect("a legal revision");
        producer
            .write(stamp, record)
            .expect("the reference effect has a cell for the record");
        gate.publish(revision);
    }

    /// One record to the instance `effect_id` of `track_id`.
    fn write(
        &mut self,
        track_id: &str,
        effect_id: &str,
        record: effect_contract::EffectControlRecord,
    ) {
        let Self { effects, gate, .. } = self;
        let producer = effects
            .iter_mut()
            .find(|producer| &*producer.track_id == track_id && &*producer.effect_id == effect_id)
            .expect("effect producer");
        Self::write_cell(producer, gate, record);
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

/// Every control row (its `Debug` text: address, descriptor and value) and every state record
/// (handle, value bits), paged through `CONTROL_LIST_GET` and `CONTROL_STATE_GET`. The two
/// messages must agree on every value.
fn control_readback(rig: &mut Rig) -> (Vec<String>, Vec<(u32, u32)>) {
    let (entries, ..) = control_list_all_c(rig.session, &mut rig.request, 256);
    let state = control_state_all_c(rig.session, &mut rig.request, 256);
    let listed = entries
        .iter()
        .filter_map(|entry| match &entry.body {
            protocol::ControlBody::Control(row) => Some((entry.handle, row.value.to_bits())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        state, listed,
        "the list and the state report one value per control"
    );
    (
        entries.iter().map(|entry| format!("{entry:?}")).collect(),
        state,
    )
}

/// #1264 gate 3, over the control list (#1552). Red if the readback keeps the old value after a
/// live edit, or reports a value a rebuild's catalog would not: a set, a lane split and a removal
/// back to the default.
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
    let before = control_readback(&mut live);
    assert_eq!(live.apply(&edits), crate::RESULT_OK);
    assert_eq!(live.summary().3, 0, "the live rig did not rebuild");
    let after = control_readback(&mut live);
    assert_ne!(after.1, before.1, "the edit changes the readback");

    let mut rebuilt = Rig::new(&document);
    rebuilt.step();
    let mut structural = edits;
    structural.push(content_edit(&model));
    assert_eq!(rebuilt.apply(&structural), crate::RESULT_OK);
    assert_eq!(rebuilt.summary().3, 1, "the second rig rebuilt");
    assert_eq!(control_readback(&mut rebuilt), after);
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

/// #1555 gate 2: the lanes the core chooses for a session with a console slot on every track and
/// one insert are the lanes the C ABI carries: an effect producer per console slot and insert, and
/// a fader/matrix and an input producer per strip (#1556: the input rows are live).
///
/// Test value: red if choosing the lanes from the rows leaves a strip without its input cells,
/// which would hand a live trim, polarity or filter edit no cell.
#[test]
fn a_session_with_a_console_slot_and_an_insert_carries_the_effect_and_input_lanes() {
    let document = eq_session(48_000);
    let children = compile_children(&document, limits(), None).expect("EQ session");
    let providers = children.session.test_providers();
    let model = parse_session_json(&document).expect("EQ session parses");
    let strips = model.tracks.len() + model.submixes.len();
    assert_eq!(providers.test_strip_controls().len(), strips);
    assert!(
        providers
            .test_strip_controls()
            .iter()
            .all(|strip| strip.input.is_some()),
        "every strip carries an input lane"
    );
    let slots = model.console.pre_insert.len() + model.console.post_insert.len();
    let inserts: usize = model
        .tracks
        .iter()
        .map(|track| track.inserts.effects.len())
        .chain(
            model
                .submixes
                .iter()
                .map(|submix| submix.inserts.effects.len()),
        )
        .sum();
    assert!(slots > 0 && inserts > 0, "the session has both kinds");
    assert_eq!(
        providers.test_effects().len(),
        strips * slots + inserts,
        "one effect producer per console slot of every strip and per insert"
    );
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
        let revision = self.gate.last_published() + 1;
        let stamp = self.gate.stamp(revision).expect("a legal revision");
        let producer = self
            .effects
            .iter_mut()
            .find(|producer| &*producer.track_id == track_id && &*producer.effect_id == effect_id)
            .expect("EQ producer");
        producer
            .publish_candidate_targets(stamp, base, &targets)
            .expect("reference publication");
        self.gate.publish(revision);
        self.eq_producer(track_id, effect_id)
            .commit_owner()
            .expect("reference commit");
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

/// How many writes an effect instance's bypass cell, parameter cells, then target cells, have
/// taken in the current or the pending epoch (#1345 D1).
fn effect_writes(rig: &Rig, epoch: Epoch, track_id: &str, effect_id: &str) -> Vec<u64> {
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let epoch = match epoch {
        Epoch::Current => snapshot.provider_epoch,
        Epoch::Pending => *snapshot
            .pending_provider_epochs
            .last()
            .expect("a pending candidate"),
    };
    snapshot
        .effect_cells
        .iter()
        .find(|(row_epoch, track, effect, _)| {
            *row_epoch == epoch && &**track == track_id && &**effect == effect_id
        })
        .map(|(.., writes)| writes.clone())
        .expect("effect producer")
}

/// #1265 gate 3, on cells (#1345 D5, D7). Red if a transaction refused after its EQ owners were
/// begun leaves an owner begun or published, a target cell written, or the model changed: two
/// owners are begun and preflighted (EQ edits on eq0 and on eq1), the commit is refused at the
/// last fallible step, and every owner, every effect cell and the model are as they were; the
/// retry commits both.
#[test]
fn a_refused_eq_transaction_leaves_its_owners_idle_and_writes_no_cell() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let mut rig = Rig::new(&eq_session(48_000));
    rig.step();
    let gain = |track: &str, value: f32| {
        upsert(
            track,
            RackName::Console,
            "eq",
            4,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )
    };
    let transaction = [
        gain("eq0", 3.0),
        gain("eq1", -20.0),
        bypass_edit("eq1", RackName::Console, "eq", true),
    ];
    let before = (refusal_state(&rig), effect_owners(&rig));
    let cells = (
        effect_writes(&rig, Epoch::Current, "eq0", "eq"),
        effect_writes(&rig, Epoch::Current, "eq1", "eq"),
    );
    assert!(cells.0.iter().chain(&cells.1).all(|writes| *writes == 0));
    crate::ffi::test_set_structural_faults(
        rig.session,
        [Some(TestStructuralFaultPhase::BeforeLivePush), None],
    );
    assert_eq!(rig.apply(&transaction), crate::RESULT_BACKPRESSURE);
    assert_eq!((refusal_state(&rig), effect_owners(&rig)), before);
    assert_eq!(
        (
            effect_writes(&rig, Epoch::Current, "eq0", "eq"),
            effect_writes(&rig, Epoch::Current, "eq1", "eq"),
        ),
        cells,
        "no target cell took a write"
    );
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (0, "Idle".to_owned()));
    assert_eq!(owner_phase(&rig, "eq1", "eq"), (0, "Idle".to_owned()));
    assert_eq!(rig.summary().3, 0, "no rebuild");

    assert_eq!(rig.apply(&transaction), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the retry is live");
    assert_eq!(owner_phase(&rig, "eq0", "eq"), (1, "Idle".to_owned()));
    assert_eq!(owner_phase(&rig, "eq1", "eq"), (1, "Idle".to_owned()));
    let written = (
        effect_writes(&rig, Epoch::Current, "eq0", "eq"),
        effect_writes(&rig, Epoch::Current, "eq1", "eq"),
    );
    assert_eq!(
        written.0.iter().sum::<u64>(),
        1,
        "eq0's left gain wrote one target cell"
    );
    assert_eq!(
        written.1.iter().sum::<u64>(),
        2,
        "eq1's left gain wrote one target cell and its bypass one bypass cell"
    );
    assert_eq!(written.1[0], 1, "the bypass cell is the first");
}

/// #1265 gate 5, over the control list (#1552). Red if the readback keeps an EQ value's old state
/// after a live edit, or reports one a rebuild's catalog would not.
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
    let before = control_readback(&mut live);
    assert_eq!(live.apply(&edits), crate::RESULT_OK);
    assert_eq!(live.summary().3, 0, "the live rig did not rebuild");
    let after = control_readback(&mut live);
    assert_ne!(after.1, before.1, "the edit changes the readback");

    let mut rebuilt = Rig::new(&document);
    rebuilt.step();
    let mut structural = edits;
    structural.push(content_edit(&model));
    assert_eq!(rebuilt.apply(&structural), crate::RESULT_OK);
    assert_eq!(rebuilt.summary().3, 1, "the second rig rebuilt");
    assert_eq!(control_readback(&mut rebuilt), after);
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

/// Gate 1 of #1345: a paused host sends forty edits that change only the left channel of a
/// compressor's threshold (a `PerLane` parameter, so one `Left` cell), then forty edits of one EQ
/// band's gain on both channels (a `Both` target, so two cells), and none is refused for room or
/// turned into a rebuild. One render applies the last of each, so the output equals a twin that
/// made only the last of each, and `COUNTERS_GET` reports `LIVE_VALUES_SUPERSEDED` as 117: the 39
/// writes the threshold cell lost, and the 39 each of the two target cells that a later write
/// replaced before a block read them (#1312 D2's per-cell unit).
///
/// Test value: red if an effect lane is still a bounded queue (the seventeenth edit refuses or
/// rebuilds), or if the counter misses a parameter cell or counts a `Both` target once.
#[test]
fn forty_effect_edits_of_each_kind_before_one_block_are_all_accepted_and_the_last_is_heard() {
    use session::{ParameterChannel, ParameterUnit, RackName};
    let document = with_model(&eq_session(48_000), |model| {
        let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        for track in &mut model.tracks {
            let mut compressor = template.clone();
            compressor.id = StableId::parse("comp").expect("compressor");
            compressor.identity = session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor ID"),
            };
            compressor.params.clear();
            compressor.bypass = false;
            track.inserts.effects.push(compressor);
        }
    });
    let threshold = |value: f32| {
        upsert(
            "eq0",
            RackName::Inserts,
            "comp",
            1,
            ParameterChannel::Left,
            ParameterUnit::Db,
            value,
        )
    };
    // A both-channel gain edit is one upsert per lane, which designs one `Both` target: two cells.
    let gain = |value: f32| {
        [ParameterChannel::Left, ParameterChannel::Right].map(|channel| {
            upsert(
                "eq0",
                RackName::Console,
                "eq",
                4,
                channel,
                ParameterUnit::Db,
                value,
            )
        })
    };
    let threshold_at = |edit: usize| -10.0 - edit as f32 * 0.5;
    let gain_at = |edit: usize| 1.0 + edit as f32 * 0.25;
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    let mut quiet = Rig::new(&document);
    rig.step();
    twin.step();
    quiet.step();
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    let (revision, _, epoch, _) = rig.summary();
    for edit in 0..40 {
        assert_eq!(
            rig.apply(&[threshold(threshold_at(edit))]),
            crate::RESULT_OK,
            "threshold edit {edit}"
        );
    }
    for edit in 0..40 {
        assert_eq!(
            rig.apply(&gain(gain_at(edit))),
            crate::RESULT_OK,
            "gain edit {edit}"
        );
    }
    let (after, after_epoch, pending) = {
        let summary = rig.summary();
        (summary.0, summary.2, summary.3)
    };
    assert_eq!(
        (after, after_epoch, pending),
        (revision + 80, epoch, 0),
        "eighty live revisions and no plan prepared"
    );
    // The threshold lane's `Left` cell took forty writes and its `Right` cell none; the gain's two
    // target cells took forty each. (The first entry of each row is the bypass cell: no write.)
    let comp = effect_writes(&rig, Epoch::Current, "eq0", "comp");
    assert_eq!(comp.iter().sum::<u64>(), 40, "{comp:?}");
    assert_eq!(comp.iter().filter(|writes| **writes != 0).count(), 1);
    let eq = effect_writes(&rig, Epoch::Current, "eq0", "eq");
    assert_eq!(eq.iter().sum::<u64>(), 80, "{eq:?}");
    assert_eq!(eq.iter().filter(|writes| **writes == 40).count(), 2);

    assert_eq!(twin.apply(&[threshold(threshold_at(39))]), crate::RESULT_OK);
    assert_eq!(twin.apply(&gain(gain_at(39))), crate::RESULT_OK);
    // The counter is the render side's: nothing has been read yet.
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    let blocks = u64::from(rig.latency()) / rig.quantum as u64 + 4;
    let mut audible = false;
    for block in 0..blocks {
        let heard = rig.step();
        assert_eq!(
            bits(&heard),
            bits(&twin.step()),
            "block {block}: the last edit of each kind is heard"
        );
        audible |= bits(&heard) != bits(&quiet.step());
        if block == 0 {
            assert_eq!(
                rig.counter(protocol::CounterId::LiveValuesSuperseded),
                39 + 2 * 39
            );
        }
    }
    assert!(
        audible,
        "the edits change the output against an unedited run"
    );
    assert_eq!(
        rig.counter(protocol::CounterId::LiveValuesSuperseded),
        117,
        "39 + 2 x 39, in the per-cell unit"
    );
    assert_eq!(twin.counter(protocol::CounterId::LiveValuesSuperseded), 0);
    // Nothing is pending now: another block replaces nothing.
    rig.step();
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 117);
}

/// The value each live, continuous parameter of `descriptor` is set to: its midpoint, or its
/// minimum when the midpoint is its default.
fn live_midpoints(
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

/// #1345 D5, the product outcome: a single transaction that writes more cells to one effect
/// instance than the old 16-record queue held is live. It is not refused, and it is not turned
/// into a rebuild because it outgrew a queue (the old `commit_live` rebuilt it).
///
/// Every live parameter of a multiband compressor is changed on both channels in one transaction:
/// more cells than the old depth. The transaction commits one revision, no candidate plan is
/// prepared, every cell of the instance took exactly one write, and the render equals a twin that
/// received the same values one transaction at a time.
///
/// Test value: red if a transaction is rebuilt for being larger than some queue-like bound. A
/// rebuild renders the same samples, so only the pending-candidate count and the per-cell write
/// counts can tell; nothing else defends "none is turned into a rebuild".
#[test]
fn one_transaction_with_more_cells_than_the_old_queue_held_stays_live() {
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
    let descriptor = compile_children(&document, limits(), None)
        .expect("multiband session")
        .session
        .test_providers()
        .test_effects()
        .iter()
        .find(|producer| &*producer.effect_id == "mb")
        .expect("multiband producer")
        .descriptor;
    let values = live_midpoints(descriptor);
    let cells: usize = descriptor
        .parameters
        .iter()
        .filter(|parameter| {
            parameter.automatable
                && parameter.automation_rate == effect_contract::AutomationRate::Block
                && parameter.domain == effect_contract::ParameterDomain::Continuous
        })
        .map(|parameter| match parameter.channel_policy {
            effect_contract::ParameterChannelPolicy::Shared => 1,
            effect_contract::ParameterChannelPolicy::PerLane => 2,
        })
        .sum();
    assert!(
        cells > OLD_QUEUE_DEPTH,
        "the transaction writes more cells than the old queue held ({cells} cells)"
    );
    let edit = |&(parameter_id, unit, value): &(u32, session::ParameterUnit, f32)| {
        upsert(
            "eq0",
            session::RackName::Inserts,
            "mb",
            parameter_id,
            session::ParameterChannel::Both,
            unit,
            value,
        )
    };
    let all: Vec<SessionEdit> = values.iter().map(edit).collect();

    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    rig.step();
    twin.step();
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&all), crate::RESULT_OK);
    assert_eq!(
        rig.summary(),
        (revision + 1, rig.summary().1, epoch, 0),
        "one revision, the same plan epoch, no candidate prepared"
    );
    let writes = effect_writes(&rig, Epoch::Current, "eq0", "mb");
    assert_eq!(
        writes.iter().sum::<u64>(),
        cells as u64,
        "every cell of the instance took one write: {writes:?}"
    );
    for parameter in &all {
        assert_eq!(
            twin.apply(core::slice::from_ref(parameter)),
            crate::RESULT_OK
        );
    }
    let blocks = u64::from(rig.latency()) / rig.quantum as u64 + 4;
    for block in 0..blocks {
        assert_eq!(
            bits(&rig.step()),
            bits(&twin.step()),
            "block {block}: one transaction renders what one-at-a-time renders"
        );
    }
}

/// Gate 2 of #1345: one transaction lifts a compressor's bypass and changes its threshold. Both
/// are cells of one revision, so they land in one block: the C ABI plan renders, bit for bit, what
/// a host-core plan with every live lane renders when it is given the bypass cell and the
/// threshold cells before the same block.
///
/// Two controls make the bits mean something: a plan given only the lifted bypass, and one given
/// only the new threshold, each render differently from the plan that got both.
///
/// The reference is host-core's lane given the same cells, the convention of this file's #1264 and
/// #1265 tests, not a rebuild of the committed model: a live edit keeps the effect's state, so a
/// rebuilt plan matches it only after the settling window the #1266 section states, and a twin
/// built that way would not catch more. The spec's "twin's rebuild" is not implemented; the
/// reference shares the lane code, so the order and the block in which a revision's cells land are
/// held at the lane, by effect-contract's `live_control` and `live_window` tests.
///
/// Test value: the C ABI dropping a `Bypass` that rides beside parameter records, or losing a
/// parameter record beside a `Bypass` (the output would equal a control's). It does not catch a
/// lane that defers the parameters a block after the bypass (the reference shares the lane).
/// Mutation (PR evidence): skip the bypass cell's read in `EffectControlLane::stage`.
#[test]
fn a_bypass_lifted_beside_a_parameter_renders_like_the_browsers_lane() {
    use effect_contract::{
        EffectControlRecord::Bypass, ParameterChannel::Left, ParameterChannel::Right,
    };
    use session::{ParameterChannel, ParameterUnit, RackName};
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        let label = format!("{sample_rate_hz} Hz");
        let document = with_model(&effect_session(10, sample_rate_hz), |model| {
            let compressor = model.tracks[0]
                .inserts
                .effects
                .iter_mut()
                .find(|effect| effect.id.as_str() == "comp")
                .expect("compressor");
            compressor.bypass = true;
        });
        let model = parse_session_json(&document).expect("model");
        let first = model.tracks[0].id.as_str();
        let mut rig = Rig::new(&document);
        let mut both = LaneReference::new(&document);
        let mut lift_only = LaneReference::new(&document);
        let mut threshold_only = LaneReference::new(&document);
        for block in 0..window(rig.latency(), 0, rig.quantum) {
            let live = bits(&rig.step());
            for (name, reference) in [
                ("both", &mut both),
                ("lift only", &mut lift_only),
                ("threshold only", &mut threshold_only),
            ] {
                assert_eq!(
                    live,
                    bits(&reference.reference.step()),
                    "{label}: {name}, warm-up {block}"
                );
            }
        }
        let edits = [
            bypass_edit(first, RackName::Inserts, "comp", false),
            upsert(
                first,
                RackName::Inserts,
                "comp",
                1,
                ParameterChannel::Both,
                ParameterUnit::Db,
                -40.0,
            ),
        ];
        let (revision, _, epoch, _) = rig.summary();
        assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
        let (after, _, after_epoch, pending) = rig.summary();
        assert_eq!(
            (after, after_epoch, pending),
            (revision + 1, epoch, 0),
            "{label}: one live revision and no plan prepared"
        );
        for reference in [&mut both, &mut threshold_only] {
            reference.push(first, "comp", 1, Left, -40.0);
            reference.push(first, "comp", 1, Right, -40.0);
        }
        for reference in [&mut both, &mut lift_only] {
            reference.write(first, "comp", Bypass(false));
        }

        let (mut differs_from_lift, mut differs_from_threshold) = (false, false);
        for block in 0..12 {
            let live = bits(&rig.step());
            let expected = both.reference.step();
            assert!(
                expected.iter().any(|sample| *sample != 0.0),
                "{label}: block {block} carries signal"
            );
            assert_eq!(
                live,
                bits(&expected),
                "{label}: block {block} after the edit"
            );
            differs_from_lift |= live != bits(&lift_only.reference.step());
            differs_from_threshold |= live != bits(&threshold_only.reference.step());
        }
        assert!(
            differs_from_lift && differs_from_threshold,
            "{label}: the lifted bypass and the new threshold are each audible"
        );
        assert_eq!(rig.summary().2, 0, "{label}: the first epoch still renders");
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
    lanes.write(
        "eq0",
        "eq",
        effect_contract::EffectControlRecord::Bypass(true),
    );
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
    let Err(failure) = compile_children(&rejected, limits(), None) else {
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

// Issue #1552: the control list on the C ABI.

/// Two tracks (each with an insert and the session's console slots), a submix that a send feeds, a
/// `pre_fader` and a `post_fader` route into the output, and a VCA over the second track.
fn control_list_session() -> String {
    with_model(&effect_session(1, 48_000), |model| {
        let track = model.tracks[0].clone();
        let mut second = track.clone();
        second.id = StableId::parse("eq1").expect("second track");
        model.tracks.push(second.clone());
        let mut second_route = model.routes[0].clone();
        second_route.id = StableId::parse("eq1-main").expect("second route");
        if let RouteSource::Track { track_id, .. } = &mut second_route.source {
            *track_id = second.id;
        }
        model.routes.push(second_route);
        let bus = StableId::parse("bus").expect("bus");
        model.submixes.push(Submix {
            id: bus.clone(),
            builtins: track.builtins.clone(),
            console: track.console.clone(),
            inserts: session::Rack {
                effects: Vec::new(),
            },
            fader: track.fader.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        });
        for (route, tap) in model
            .routes
            .iter_mut()
            .zip([session::SendTap::PreFader, session::SendTap::PostFader])
        {
            if let RouteSource::Track { tap: source, .. } = &mut route.source {
                *source = tap;
            }
        }
        let mut send = model.routes[0].clone();
        send.id = StableId::parse("eq0-bus").expect("send");
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        let mut out = model.routes[0].clone();
        out.id = StableId::parse("bus-main").expect("bus output");
        out.source = RouteSource::Submix {
            submix_id: bus,
            tap: session::SendTap::PostPan,
        };
        model.routes.push(send);
        model.routes.push(out);
        model.vcas.push(Vca {
            id: StableId::parse("vca").expect("vca"),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: vec![model.tracks[1].id.clone()],
        });
    })
}

fn control_of(entry: &protocol::ControlEntry) -> Option<&protocol::ControlRow> {
    match &entry.body {
        protocol::ControlBody::Control(row) => Some(row),
        _ => None,
    }
}

/// The handle of the control `name` of strip `owner` in its builtin section, on the left lane.
fn builtin_handle(entries: &[protocol::ControlEntry], owner: &str, name: &str) -> u32 {
    entries
        .iter()
        .find(|entry| {
            entry.address.owner_id.as_deref() == Some(owner)
                && entry.address.section == protocol::ControlSection::Builtins
                && entry.address.lane == Some(protocol::ParameterChannel::Left)
                && control_of(entry).is_some_and(|row| row.name == name)
        })
        .expect("the builtin row")
        .handle
}

/// #1552 gate 1. Red if the list misses builtins, routes, VCAs or session rows, or paging drops or
/// repeats an entry.
#[test]
fn the_c_abi_lists_every_control_of_the_session_in_pages() {
    let document = control_list_session();
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    rig.step();
    let (whole, epoch, revision) = control_list_all_c(rig.session, &mut rig.request, 256);
    let (single, ..) = control_list_all_c(rig.session, &mut rig.request, 1);
    assert_eq!(single, whole, "limit 1 and limit 256 list the same entries");
    assert_eq!((epoch, revision), (1, rig.summary().0));

    // Handles run from 1 without a gap, and no address is listed twice.
    for (index, entry) in whole.iter().enumerate() {
        assert_eq!(entry.handle as usize, index + 1);
    }
    let mut addresses = std::collections::HashSet::new();
    // (An owner's header and its `own` section's header share one address.)
    for entry in whole.iter().filter(|entry| control_of(entry).is_some()) {
        assert!(
            addresses.insert(format!("{:?}", entry.address)),
            "{entry:?}"
        );
    }

    // The owners come in C4's order: strips (tracks, then submixes), routes, VCAs, the session.
    let owner_headers = whole
        .iter()
        .filter(|entry| matches!(entry.body, protocol::ControlBody::OwnerHeader(_)))
        .map(|entry| (entry.address.owner_kind, entry.address.owner_id.clone()))
        .collect::<Vec<_>>();
    let expected_owners = model
        .strips()
        .map(|strip| {
            (
                protocol::ControlOwnerKind::Strip,
                Some(strip.id.as_str().to_owned()),
            )
        })
        .chain(model.routes.iter().map(|route| {
            (
                protocol::ControlOwnerKind::Route,
                Some(route.id.as_str().to_owned()),
            )
        }))
        .chain(model.vcas.iter().map(|vca| {
            (
                protocol::ControlOwnerKind::Vca,
                Some(vca.id.as_str().to_owned()),
            )
        }))
        .chain([(protocol::ControlOwnerKind::Session, None)])
        .collect::<Vec<_>>();
    assert_eq!(owner_headers, expected_owners);

    // Every control row follows its owner's header and its section's header. (That the rows are
    // the committed model's controls, with its values, is host-core's catalog test.)
    let mut owner = None;
    let mut section = None;
    for entry in &whole {
        match &entry.body {
            protocol::ControlBody::OwnerHeader(_) => {
                owner = Some((entry.address.owner_kind, entry.address.owner_id.clone()));
                section = None;
            }
            protocol::ControlBody::SectionHeader(_) => {
                assert_eq!(
                    owner,
                    Some((entry.address.owner_kind, entry.address.owner_id.clone()))
                );
                section = Some((entry.address.section, entry.address.slot_id.clone()));
            }
            protocol::ControlBody::Control(_) => {
                assert_eq!(
                    owner,
                    Some((entry.address.owner_kind, entry.address.owner_id.clone()))
                );
                assert_eq!(
                    section,
                    Some((entry.address.section, entry.address.slot_id.clone()))
                );
            }
        }
    }
    for kind in [
        protocol::ControlSection::Own,
        protocol::ControlSection::Builtins,
        protocol::ControlSection::Console,
        protocol::ControlSection::Inserts,
    ] {
        assert!(
            whole.iter().any(|entry| entry.address.section == kind
                && entry.address.owner_kind == protocol::ControlOwnerKind::Strip),
            "{kind:?}"
        );
    }

    // The state poll reports the same values for the same handles.
    let state = control_state_all_c(rig.session, &mut rig.request, 256);
    let expected = whole
        .iter()
        .filter_map(|entry| control_of(entry).map(|row| (entry.handle, row.value.to_bits())))
        .collect::<Vec<_>>();
    assert_eq!(state, expected);
}

/// #1552 gate 2. Red if a live commit leaves a stale value or moves handles.
#[test]
fn a_live_fader_commit_updates_the_list_and_keeps_every_handle() {
    let mut rig = Rig::new(&effect_session(10, 48_000));
    rig.step();
    let (before, epoch, revision) = control_list_all_c(rig.session, &mut rig.request, 256);
    let fader = builtin_handle(&before, "eq0", "fader_db");

    assert_eq!(rig.apply(&[left_db_edit("eq0", -3.0)]), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    let (after, after_epoch, after_revision) =
        control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!((after_epoch, after_revision), (epoch, revision + 1));
    assert_eq!(
        control_of(&after[fader as usize - 1]).map(|row| row.value),
        Some(-3.0)
    );
    // Only the fader and the effective fader derived from it moved; every handle is where it was.
    for (old, new) in before.iter().zip(&after) {
        assert_eq!((old.handle, &old.address), (new.handle, &new.address));
        if old != new {
            let name = control_of(new).map(|row| row.name.as_str());
            assert!(
                matches!(name, Some("fader_db" | "effective_fader_db"))
                    && new.address.owner_id.as_deref() == Some("eq0"),
                "{new:?}"
            );
        }
    }
    let state = control_state_all_c(rig.session, &mut rig.request, 256);
    assert!(state.contains(&(fader, (-3.0_f32).to_bits())));
}

/// #1552 gate 3. Red if a structural change keeps old handles valid, or a value-only rebuild
/// invalidates a client's list.
#[test]
fn a_structural_commit_advances_the_epoch_and_a_value_only_rebuild_keeps_it() {
    let document = effect_session(10, 48_000);
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    rig.step();
    let (_, epoch, revision) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!(epoch, 1);

    // An insert changes the row set: the epoch advances once, and the old one is told so.
    let mut insert = model.tracks[0].inserts.effects[0].clone();
    insert.id = StableId::parse("comp2").expect("second insert");
    assert_eq!(
        rig.apply(&[SessionEdit::PutTrackEffect {
            track_id: model.tracks[0].id.clone(),
            rack_name: session::RackName::Inserts,
            final_position: 1,
            effect: insert,
        }]),
        crate::RESULT_OK
    );
    assert_eq!(rig.summary().3, 1, "an insert rebuilds");
    let (after, epoch, after_revision) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!((epoch, after_revision), (2, revision + 1));
    assert!(after.iter().any(|entry| {
        entry.address.owner_id.as_deref() == Some("eq0")
            && entry.address.slot_id.as_deref() == Some("comp2")
    }));
    let stale = protocol::ControlListRequest {
        structure_epoch: 1,
        after_handle: 0,
        limit: 1,
        owner: None,
    };
    let expected = Some((
        protocol::StatusCode::RevisionConflict,
        vec!["control.list.stale".to_owned()],
    ));
    let request = rig.next_request();
    assert_eq!(control_list_c(rig.session, request, &stale).err(), expected);
    let request = rig.next_request();
    assert_eq!(
        control_state_c(rig.session, request, &stale).err(),
        expected
    );

    // A rebuild-only value (the input delay) changes no address: the epoch and handles hold.
    // (Render adopts the first rebuild's plan before a second one can be prepared.)
    rig.step();
    rig.step();
    let mut builtins = model.tracks[0].builtins.clone();
    builtins.left.delay_samples = 4;
    builtins.right.delay_samples = 4;
    assert_eq!(
        rig.apply(&[SessionEdit::SetTrackBuiltins {
            track_id: model.tracks[0].id.clone(),
            builtins,
            ramp_samples: None,
        }]),
        crate::RESULT_OK
    );
    assert_eq!(rig.summary().3, 1, "a delay rebuilds");
    let (again, same_epoch, _) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!(same_epoch, 2, "the rebuild kept the structure");
    let delay = builtin_handle(&again, "eq0", "delay_samples");
    assert_eq!(
        control_of(&again[delay as usize - 1]).map(|row| row.value),
        Some(4.0)
    );
    for (old, new) in after.iter().zip(&again) {
        assert_eq!((old.handle, &old.address), (new.handle, &new.address));
    }
    let current = protocol::ControlListRequest {
        structure_epoch: 2,
        ..stale
    };
    let request = rig.next_request();
    assert!(control_list_c(rig.session, request, &current).is_ok());
}

/// #1552 gate 4. Red if automation is acked for a row nothing renders.
#[test]
fn automation_is_admitted_for_an_effect_parameter_and_refused_for_any_other_row() {
    let mut rig = Rig::new(&effect_session(10, 48_000));
    rig.step();
    let (entries, ..) = control_list_all_c(rig.session, &mut rig.request, 256);
    let revision = rig.summary().0;
    let parameter = automation_target(&entries, 0.0);
    let request = rig.next_request();
    assert_eq!(
        automation_status(rig.session, request, revision, parameter, 0.0),
        (protocol::StatusCode::Ok, Vec::new()),
        "an effect parameter is admitted as before"
    );
    let fader = builtin_handle(&entries, "eq0", "fader_db");
    let bypass = entries
        .iter()
        .find(|entry| {
            entry.address.control == Some(0) && entry.address.owner_id.as_deref() == Some("eq0")
        })
        .expect("a bypass row")
        .handle;
    for handle in [fader, bypass] {
        let request = rig.next_request();
        assert_eq!(
            automation_status(rig.session, request, revision, handle, 0.0),
            (
                protocol::StatusCode::InvalidField,
                vec!["automation.target.unsupported".to_owned()]
            ),
            "handle {handle}"
        );
    }
}

/// #1552: a live commit that changes a descriptor advances the structure epoch, and one that changes
/// values only does not. A `matrix_mode` switch is live and flips `active` on the strip's pan and
/// matrix rows without moving an address.
///
/// Red if a client keeps a current epoch while holding a descriptor that the commit changed (the
/// list still says a matrix row is inactive), or if a fader move invalidates the client's list.
#[test]
fn a_live_matrix_mode_switch_advances_the_epoch_and_a_fader_move_does_not() {
    let document = effect_session(10, 48_000);
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    rig.step();
    let (before, epoch, _) = control_list_all_c(rig.session, &mut rig.request, 256);

    assert_eq!(rig.apply(&[left_db_edit("eq0", -3.0)]), crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "a fader move is live");
    let (_, same, _) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!(same, epoch, "values moved, no descriptor did");

    let MatrixOrPan::Pan {
        smoothing_samples, ..
    } = model.tracks[0].matrix_or_pan
    else {
        panic!("the fixture's tracks pan");
    };
    assert_eq!(
        rig.apply(&[SessionEdit::SetTrackMatrixOrPan {
            track_id: model.tracks[0].id.clone(),
            matrix_or_pan: MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples,
            },
            ramp_samples: None,
        }]),
        crate::RESULT_OK
    );
    assert_eq!(rig.summary().3, 0, "a mode switch is live");
    let (after, switched, _) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!(switched, epoch + 1);
    let active = |list: &[protocol::ControlEntry]| {
        list.iter()
            .filter(|entry| entry.address.owner_id.as_deref() == Some("eq0"))
            .filter_map(|entry| control_of(entry).map(|row| (row.name.clone(), row.active)))
            .collect::<Vec<_>>()
    };
    let (old, new) = (active(&before), active(&after));
    assert_ne!(old, new, "the list shows the new `active` flags");
    assert!(
        old.iter().any(|(name, active)| name == "pan" && *active)
            && new.iter().any(|(name, active)| name == "pan" && !*active)
            && new
                .iter()
                .any(|(name, active)| name == "matrix_ll" && *active),
        "pan went inactive and the matrix active"
    );
    assert_eq!(
        before
            .iter()
            .map(|entry| &entry.address)
            .collect::<Vec<_>>(),
        after.iter().map(|entry| &entry.address).collect::<Vec<_>>(),
        "no address moved"
    );
    let stale = protocol::ControlListRequest {
        structure_epoch: epoch,
        after_handle: 0,
        limit: 1,
        owner: None,
    };
    let request = rig.next_request();
    assert_eq!(
        control_list_c(rig.session, request, &stale)
            .err()
            .map(|(status, _)| status),
        Some(protocol::StatusCode::RevisionConflict)
    );
}

// Issue #1553: `SET_CONTROL`, any control by its stable address.

mod set_control {
    use super::*;
    use protocol::control_core::{ControlAddress, Lane, Owner, Section};
    use session::{ParameterChannel, ParameterUnit, RackName};

    /// The builtin IDs the addresses name (`BUILTIN_PARAMETER_DESCRIPTORS`).
    const POLARITY_INVERT: u32 = 1;
    const TRIM_DB: u32 = 2;
    const HPF_HZ: u32 = 3;
    const LPF_HZ: u32 = 4;
    const DELAY_SAMPLES: u32 = 11;
    const FADER_DB: u32 = 5;
    const MUTE: u32 = 6;
    const MATRIX_LL: u32 = 7;
    const PAN: u32 = 12;
    /// The own-row IDs.
    const MATRIX_MODE: u32 = 1;
    const SOLO: u32 = 2;
    const PAN_WINDOW_SAMPLES: u32 = 3;
    const EFFECTIVE_FADER_DB: u32 = 64;

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("stable ID")
    }

    fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
        ControlAddress {
            owner,
            section,
            control,
            lane,
        }
    }

    fn builtin(track: &str, control: u32, lane: Lane) -> ControlAddress {
        address(Owner::Strip(id(track)), Section::Builtins, control, lane)
    }

    fn own(track: &str, control: u32, lane: Lane) -> ControlAddress {
        address(Owner::Strip(id(track)), Section::Own, control, lane)
    }

    fn insert(track: &str, effect: &str, control: u32, lane: Lane) -> ControlAddress {
        address(
            Owner::Strip(id(track)),
            Section::Inserts(id(effect)),
            control,
            lane,
        )
    }

    fn console(track: &str, slot: &str, control: u32, lane: Lane) -> ControlAddress {
        address(
            Owner::Strip(id(track)),
            Section::Console(id(slot)),
            control,
            lane,
        )
    }

    fn set(address: ControlAddress, value: f32, ramp_samples: Option<u32>) -> SessionEdit {
        SessionEdit::SetControl {
            address,
            value,
            ramp_samples,
        }
    }

    /// The model after `edits`, applied the way a store applies them.
    fn applied(document: &str, edits: &[SessionEdit]) -> SessionModel {
        let mut model = parse_session_json(document).expect("model");
        for edit in edits {
            protocol::apply_session_edit(&mut model, edit).expect("the edit applies");
        }
        model
    }

    /// Two rigs of one document: one takes `by_address`, the other `per_kind`. Both commit one
    /// revision with the plan epoch unchanged and no candidate, and their next 8 blocks are
    /// bit-identical, and differ from an untouched third rig (the edit is heard). Returns the 8
    /// blocks.
    fn live_twin(
        document: &str,
        by_address: &[SessionEdit],
        per_kind: &[SessionEdit],
        label: &str,
    ) -> Vec<Vec<f32>> {
        let mut address = Rig::new(document);
        let mut kind = Rig::new(document);
        let mut untouched = Rig::new(document);
        for block in 0..window(address.latency(), 0, address.quantum) {
            let expected = bits(&untouched.step());
            assert_eq!(bits(&address.step()), expected, "{label}: warm-up {block}");
            assert_eq!(bits(&kind.step()), expected, "{label}: warm-up {block}");
        }
        let (revision, _, epoch, _) = address.summary();
        assert_eq!(address.apply(by_address), crate::RESULT_OK, "{label}");
        assert_eq!(kind.apply(per_kind), crate::RESULT_OK, "{label}");
        for rig in [&address, &kind] {
            let (after, _, after_epoch, pending) = rig.summary();
            assert_eq!(
                (after, after_epoch, pending),
                (revision + 1, epoch, 0),
                "{label}: one revision, the same plan, no candidate"
            );
        }
        let mut heard = false;
        let mut blocks = Vec::new();
        for block in 0..8 {
            let live = address.step();
            let expected = kind.step();
            assert!(
                expected.iter().any(|sample| *sample != 0.0),
                "{label}: block {block} carries signal"
            );
            assert_eq!(bits(&live), bits(&expected), "{label}: block {block}");
            heard |= bits(&live) != bits(&untouched.step());
            blocks.push(live);
        }
        assert!(heard, "{label}: the edit changes the output");
        blocks
    }

    /// Gate 1: each value the old classifier makes live is as live through `SET_CONTROL`, and
    /// renders the bits of the per-kind edit on a second rig.
    ///
    /// Red if `SET_CONTROL` writes a different field, lane or value than the per-kind op (the
    /// left lane for a both-lane write, the mute in place of the fader, a matrix coefficient
    /// written without its mode switch), or reaches the plan through a path the per-kind op does
    /// not.
    #[test]
    fn a_set_control_on_a_live_value_is_live_and_renders_like_the_per_kind_edit() {
        let document = long_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let track = model.tracks[0].clone();
        let name = track.id.as_str();
        let fader = |left_db, right_db, mute: bool| DualMonoFader {
            left_db,
            right_db,
            left_mute: mute,
            right_mute: mute,
        };

        live_twin(
            &document,
            &[set(builtin(name, FADER_DB, Lane::Left), -6.0, None)],
            &[fader_edit(
                name,
                DualMonoFader {
                    left_db: -6.0,
                    ..track.fader.clone()
                },
            )],
            "fader, left lane",
        );
        live_twin(
            &document,
            &[set(builtin(name, FADER_DB, Lane::Both), -3.0, None)],
            &[fader_edit(name, fader(-3.0, -3.0, false))],
            "fader, both lanes",
        );
        live_twin(
            &document,
            &[set(builtin(name, MUTE, Lane::Both), 1.0, None)],
            &[fader_edit(
                name,
                DualMonoFader {
                    left_mute: true,
                    right_mute: true,
                    ..track.fader.clone()
                },
            )],
            "mute",
        );

        let pan_edit = set(builtin(name, PAN, Lane::Both), 0.5, None);
        let panned = applied(&document, core::slice::from_ref(&pan_edit)).tracks[0]
            .matrix_or_pan
            .clone();
        live_twin(
            &document,
            &[pan_edit],
            &[SessionEdit::SetTrackMatrixOrPan {
                track_id: id(name),
                matrix_or_pan: panned,
                ramp_samples: None,
            }],
            "pan",
        );

        // A matrix coefficient follows its mode switch in the same transaction.
        let matrix = [
            set(own(name, MATRIX_MODE, Lane::Both), 1.0, None),
            set(builtin(name, MATRIX_LL, Lane::Both), 0.5, None),
        ];
        let switched = applied(&document, &matrix).tracks[0].matrix_or_pan.clone();
        assert!(matches!(switched, MatrixOrPan::Matrix { ll, .. } if ll == 0.5));
        live_twin(
            &document,
            &matrix,
            &[SessionEdit::SetTrackMatrixOrPan {
                track_id: id(name),
                matrix_or_pan: switched,
                ramp_samples: None,
            }],
            "matrix after a mode switch",
        );

        // An effect parameter and an effect bypass: an insert's and a console slot's.
        let document = effect_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let first = model.tracks[0].id.as_str();
        let last = model.tracks[model.tracks.len() - 1].id.as_str();
        live_twin(
            &document,
            &[
                set(insert(first, "comp", 1, Lane::Left), -30.0, None),
                set(insert(first, "comp", 6, Lane::Both), 6.0, None),
                set(console(last, "clip", 1, Lane::Both), 12.0, None),
            ],
            &[
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
            ],
            "effect parameters",
        );
        live_twin(
            &document,
            &[
                set(insert(first, "comp", 0, Lane::Both), 1.0, None),
                set(console(last, "clip", 0, Lane::Both), 1.0, None),
            ],
            &[
                bypass_edit(first, RackName::Inserts, "comp", true),
                bypass_edit(last, RackName::Console, "clip", true),
            ],
            "effect bypass",
        );
    }

    /// Gate 2: a value the old classifier leaves to a rebuild is a rebuild through `SET_CONTROL`,
    /// exactly as the per-kind op is.
    ///
    /// Red if a rebuild-only value is acked as live (a candidate not prepared, a cell written).
    #[test]
    fn a_set_control_on_a_rebuild_value_is_a_rebuild() {
        let document = long_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let track = model.tracks[0].clone();
        let name = track.id.as_str();
        let builtins = |change: &dyn Fn(&mut session::ChannelBuiltins)| {
            let mut builtins = track.builtins.clone();
            change(&mut builtins.left);
            change(&mut builtins.right);
            SessionEdit::SetTrackBuiltins {
                track_id: id(name),
                builtins,
                ramp_samples: None,
            }
        };
        let by_address = set(builtin(name, DELAY_SAMPLES, Lane::Both), 64.0, None);
        let per_kind = builtins(&|lane| lane.delay_samples = 64);
        assert_rebuilds(
            &document,
            core::slice::from_ref(&by_address),
            "delay by address",
        );
        assert_rebuilds(
            &document,
            core::slice::from_ref(&per_kind),
            "delay per kind",
        );
        let route_id = model.routes[0].id.clone();
        assert_rebuilds(
            &document,
            &[set(
                address(Owner::Route(route_id.clone()), Section::Own, 1, Lane::Both),
                -6.0,
                None,
            )],
            "send gain by address",
        );
        assert_rebuilds(
            &document,
            &[SessionEdit::SetRouteGainDb {
                route_id,
                gain_db: -6.0,
                ramp_samples: None,
            }],
            "send gain per kind",
        );
    }

    /// Gate 3: the ramp of an address edit is the per-kind edit's. Absent, 0 and 480 are the
    /// session default, a step and 480 samples, and are three different renders.
    ///
    /// Red if an absent ramp is read as 0 or 480 is ignored, or the length is applied to another
    /// row.
    #[test]
    fn a_set_control_ramp_is_the_per_kind_ramp() {
        let document = long_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let track = model.tracks[0].clone();
        let name = track.id.as_str();
        let mut renders = Vec::new();
        for ramp in [None, Some(0), Some(480)] {
            let per_kind = SessionEdit::SetTrackFader {
                track_id: id(name),
                fader: DualMonoFader {
                    left_db: -12.0,
                    right_db: -12.0,
                    ..track.fader.clone()
                },
                ramp_samples: ramp,
            };
            renders.push(live_twin(
                &document,
                &[set(builtin(name, FADER_DB, Lane::Both), -12.0, ramp)],
                &[per_kind],
                &format!("fader ramp {ramp:?}"),
            ));
        }
        assert!(
            renders[0] != renders[1] && renders[1] != renders[2] && renders[0] != renders[2],
            "the session default, a step and 480 samples render differently"
        );
    }

    /// The stateless session of three tracks, centred so both lanes carry signal, with a bus fed
    /// by `eq0` and the other tracks into the output. Nothing in it keeps state while its input
    /// filters are off, so a live edit renders like a plan prepared with it once its ramp has
    /// passed and the filter memories started from the same silence.
    fn input_document(sample_rate_hz: u32, with_bus: bool) -> String {
        let mut model = crate::runtime::tests::stateless_session(sample_rate_hz);
        model.sources[0].frames = SOURCE_FRAMES;
        for track in &mut model.tracks {
            track.matrix_or_pan = MatrixOrPan::Pan {
                left: 0.0,
                right: 0.0,
                smoothing_samples: 0,
            };
        }
        let document = session::canonical_session_json(&model).expect("canonical input session");
        if !with_bus {
            return document;
        }
        with_bus_strip(&document, |model, submix| {
            let track = model.tracks[0].clone();
            submix.builtins = track.builtins.clone();
            submix.fader = track.fader.clone();
            submix.matrix_or_pan = track.matrix_or_pan.clone();
        })
    }

    /// The longest ramp an input edit leaves behind: a trim (fader), a flip (twice the mute) or a
    /// filter (64 coefficient updates).
    fn input_ramp(document: &str) -> u32 {
        let ramps = parse_session_json(document)
            .expect("model")
            .control_smoothing_samples();
        ramps.fader.max(ramps.mute * 2).max(64)
    }

    /// Gate 1 (#1556): a trim, a polarity flip, an HPF and an LPF, on a track and on a submix
    /// strip, per lane and on both, commit one revision with the plan epoch unchanged, and from the
    /// block their ramp leaves behind render the bits of a plan prepared from the committed
    /// session, at 44.1 and 96 kHz. The source is `-0.0` until the ramp has passed, so the
    /// filters, which keep memory, start from one silence in both plans. No render block after the
    /// commit allocates or frees.
    ///
    /// Red if a record lands on the wrong lane, strip or rate, a submix strip stays structural, a
    /// filter target is designed from the wrong seeds, or the input lane adds render work that
    /// allocates.
    #[test]
    fn input_values_are_live_and_render_the_prepared_plan_bits() {
        bench_support::alloc::assert_installed();
        for rate in [44_100, 96_000] {
            let document = input_document(rate, true);
            let ramp = input_ramp(&document);
            for strip in ["bus", "eq0"] {
                let cases: [(&str, Vec<SessionEdit>); 9] = [
                    (
                        "trim, left",
                        vec![set(builtin(strip, TRIM_DB, Lane::Left), -6.0, None)],
                    ),
                    (
                        "trim, both",
                        vec![set(builtin(strip, TRIM_DB, Lane::Both), -3.0, None)],
                    ),
                    (
                        "trim, split",
                        vec![
                            set(builtin(strip, TRIM_DB, Lane::Left), -2.0, None),
                            set(builtin(strip, TRIM_DB, Lane::Right), -4.0, None),
                        ],
                    ),
                    (
                        "polarity, both",
                        vec![set(builtin(strip, POLARITY_INVERT, Lane::Both), 1.0, None)],
                    ),
                    (
                        "polarity, right",
                        vec![set(builtin(strip, POLARITY_INVERT, Lane::Right), 1.0, None)],
                    ),
                    (
                        "hpf, both",
                        vec![set(builtin(strip, HPF_HZ, Lane::Both), 80.0, None)],
                    ),
                    (
                        "lpf, both",
                        vec![set(builtin(strip, LPF_HZ, Lane::Both), 8_000.0, None)],
                    ),
                    (
                        "hpf left and lpf right",
                        vec![
                            set(builtin(strip, HPF_HZ, Lane::Left), 80.0, None),
                            set(builtin(strip, LPF_HZ, Lane::Right), 6_000.0, None),
                        ],
                    ),
                    (
                        "hpf and lpf together",
                        vec![
                            set(builtin(strip, HPF_HZ, Lane::Both), 80.0, None),
                            set(builtin(strip, LPF_HZ, Lane::Both), 8_000.0, None),
                        ],
                    ),
                ];
                for (case, edits) in cases {
                    let label = format!("{rate} Hz, {strip}, {case}");
                    let mut rig = Rig::new(&document);
                    let edit_block = 4;
                    let settled = edit_block + window(rig.latency(), ramp, rig.quantum) + 1;
                    let feed = Feed::quiet_until(settled * rig.quantum as u64);
                    let total = settled + 4;
                    let mut live = Vec::new();
                    for _ in 0..edit_block {
                        live.push(rig.step_feed(&feed).0);
                    }
                    let (revision, _, epoch, pending) = rig.summary();
                    assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
                    assert_eq!(
                        (rig.summary().0, rig.summary().2, rig.summary().3),
                        (revision + 1, epoch, pending),
                        "{label}: one revision, the same plan, no candidate"
                    );
                    let committed = rig.snapshot();
                    for block in edit_block..total {
                        let (pcm, used) = rig.step_feed(&feed);
                        assert_eq!(
                            (used.allocations, used.deallocations),
                            (0, 0),
                            "{label}: block {block} allocated or freed"
                        );
                        live.push(pcm);
                    }
                    let reference = Reference::run_feed(&committed, total, &feed);
                    let untouched = Reference::run_feed(&document, total, &feed);
                    let mut heard = false;
                    for block in settled as usize..total as usize {
                        assert!(
                            reference[block].iter().any(|sample| *sample != 0.0),
                            "{label}: block {block} carries signal"
                        );
                        assert_eq!(
                            bits(&live[block]),
                            bits(&reference[block]),
                            "{label}: block {block}"
                        );
                        heard |= bits(&live[block]) != bits(&untouched[block]);
                    }
                    assert!(heard, "{label}: the edit changes the output");
                    assert_eq!(rig.summary().2, epoch, "{label}: the first epoch renders");
                }
            }
        }
    }

    /// Gate 2 (#1556): an HPF enabled from off, and disabled again, with `-0.0` samples in the
    /// source, ends on the bits of the plan prepared with the filter off; while it is on the
    /// output has the bits of the plan prepared with it on.
    ///
    /// Red if a disabled filter section changes a sample (a `-0.0` turned into `+0.0`, a stale
    /// coefficient left behind), or the enable and disable targets are designed from the wrong
    /// seeds.
    #[test]
    fn an_enabled_and_disabled_hpf_ends_on_the_prepared_plan() {
        for strip in ["bus", "eq0"] {
            let document = input_document(48_000, true);
            let mut rig = Rig::new(&document);
            let q = rig.quantum as u64;
            let ramp = input_ramp(&document);
            let k = window(rig.latency(), ramp, rig.quantum);
            let enable_block = 4;
            let loud = enable_block + k + 1;
            let disable_block = loud + 8;
            let quiet_again = disable_block + k + 1;
            let total = quiet_again + 4;
            let feed = Feed {
                quiet: Vec::from([0..loud * q, quiet_again * q..total * q]),
                constant: None,
            };
            let mut live = Vec::new();
            for _ in 0..enable_block {
                live.push(rig.step_feed(&feed).0);
            }
            assert_eq!(
                rig.apply(&[set(builtin(strip, HPF_HZ, Lane::Both), 80.0, None)]),
                crate::RESULT_OK
            );
            let enabled = rig.snapshot();
            for _ in enable_block..disable_block {
                live.push(rig.step_feed(&feed).0);
            }
            assert_eq!(
                rig.apply(&[set(builtin(strip, HPF_HZ, Lane::Both), 0.0, None)]),
                crate::RESULT_OK
            );
            let disabled = rig.snapshot();
            assert_eq!(disabled, rig.snapshot());
            for _ in disable_block..total {
                live.push(rig.step_feed(&feed).0);
            }
            assert_eq!(rig.summary().3, 0, "{strip}: no rebuild");
            assert_eq!(rig.summary().2, 0, "{strip}: the first epoch renders");

            let with_filter = Reference::run_feed(&enabled, total, &feed);
            for block in loud..disable_block {
                assert_eq!(
                    bits(&live[block as usize]),
                    bits(&with_filter[block as usize]),
                    "{strip}: filter on, block {block}"
                );
            }
            let without = Reference::run_feed(&disabled, total, &feed);
            assert!(
                (loud..disable_block)
                    .any(|block| bits(&live[block as usize]) != bits(&without[block as usize])),
                "{strip}: the filter is heard while it is on"
            );
            let mut quiet_blocks = 0;
            for block in quiet_again - 2..total {
                let expected = &without[block as usize];
                assert_eq!(
                    bits(&live[block as usize]),
                    bits(expected),
                    "{strip}: filter off again, block {block}"
                );
                quiet_blocks += usize::from(block >= quiet_again);
            }
            assert_eq!(quiet_blocks, 4);
            assert!(
                without[quiet_again as usize]
                    .iter()
                    .all(|sample| *sample == 0.0),
                "the compared window includes silent blocks"
            );
        }
    }

    /// Gate 2 (#1556): a pair change on one lane (the HPF passes the old LPF while the LPF rises)
    /// commits live, in one transaction, where two one-sided edits applied in turn would cross.
    ///
    /// Red if a filter pair is split into one-sided edits, which the preparer refuses, so the
    /// transaction rebuilds.
    #[test]
    fn a_filter_pair_that_crosses_the_old_cutoff_is_live() {
        let mut model = parse_session_json(&input_document(48_000, false)).expect("model");
        let builtins = &mut model.tracks[0].builtins;
        for lane in [&mut builtins.left, &mut builtins.right] {
            lane.hpf_hz = 1_000.0;
            lane.lpf_hz = 2_000.0;
        }
        let document = session::canonical_session_json(&model).expect("canonical");
        let mut rig = Rig::new(&document);
        rig.step();
        let (revision, _, epoch, pending) = rig.summary();
        assert_eq!(
            rig.apply(&[
                set(builtin("eq0", HPF_HZ, Lane::Left), 3_000.0, None),
                set(builtin("eq0", LPF_HZ, Lane::Left), 6_000.0, None),
            ]),
            crate::RESULT_OK
        );
        assert_eq!(
            (rig.summary().0, rig.summary().2, rig.summary().3),
            (revision + 1, epoch, pending),
            "the pair change is one live revision"
        );
        let committed = rig.model().tracks[0].builtins.left.clone();
        assert_eq!((committed.hpf_hz, committed.lpf_hz), (3_000.0, 6_000.0));
    }

    /// Gate 3 (#1556): a polarity flip with `ramp_samples` 480 on a constant source is a linear
    /// ramp from `+g` to `-g` over 480 samples, with no step, and the plan reports a finite tail.
    ///
    /// Red if a flip steps, ramps over another length, or does not pass through zero.
    #[test]
    fn a_polarity_flip_ramps_linearly_through_zero() {
        let document = input_document(48_000, false);
        let model = parse_session_json(&document).expect("model");
        let mut rig = Rig::new(&document);
        assert_eq!(
            crate::ffi::test_plan_snapshot(rig.plan).1.tail_kind,
            crate::TAIL_FINITE,
            "the input lane's bound is finite"
        );
        let feed = Feed {
            quiet: Vec::new(),
            constant: Some(0.25),
        };
        let q = rig.quantum;
        for _ in 0..4 {
            rig.step_feed(&feed);
        }
        let before = rig.step_feed(&feed).0;
        let level = before[0];
        assert!(level > 0.0);
        assert!(
            before[..q]
                .iter()
                .all(|sample| sample.to_bits() == level.to_bits()),
            "the constant source gives a constant output"
        );
        let flips: Vec<_> = model
            .tracks
            .iter()
            .map(|track| {
                set(
                    builtin(track.id.as_str(), POLARITY_INVERT, Lane::Both),
                    1.0,
                    Some(480),
                )
            })
            .collect();
        assert_eq!(rig.apply(&flips), crate::RESULT_OK);
        assert_eq!(rig.summary().3, 0, "the flip is live");
        // The sample before the flip is the ramp's start.
        let mut ratio = vec![1.0_f64];
        for _ in 0..8 {
            let (pcm, used) = rig.step_feed(&feed);
            assert_eq!((used.allocations, used.deallocations), (0, 0));
            ratio.extend(pcm[..q].iter().map(|sample| f64::from(*sample / level)));
        }
        // The ramp runs for 480 samples: no sample jumps by more than one step of 2/480, the last
        // sample is -1, and the middle passes through zero.
        let step = 2.0 / 480.0;
        let first_moved = ratio
            .iter()
            .position(|r| (r - 1.0).abs() > 1e-6)
            .expect("moves");
        let landed = ratio
            .iter()
            .position(|r| (r + 1.0).abs() < 1e-6)
            .expect("lands on -1");
        assert_eq!(landed - first_moved, 479, "{first_moved} {landed}");
        for pair in ratio[first_moved - 1..=landed].windows(2) {
            assert!(
                (pair[0] - pair[1] - step).abs() < 1e-5,
                "a linear step of {step}: {pair:?}"
            );
        }
        assert!(
            ratio[first_moved..landed].iter().any(|r| r.abs() < step),
            "the ramp passes through zero"
        );
        assert!(ratio[landed..].iter().all(|r| (r + 1.0).abs() < 1e-6));
    }

    /// Gate 3, measured: an address edit's explicit 480-sample ramp takes 480 samples. Every block
    /// that begins before sample 480 has left the old fader and not reached the new one; every
    /// block from `480 / quantum` on is the new fader, bit for bit, as a rebuilt plan renders it.
    ///
    /// Red if the length is rounded to a block (the ramp then ends at the first block boundary at
    /// or after 480), is shortened or stretched, or never lands on the target.
    #[test]
    fn a_set_control_ramp_of_480_samples_reaches_its_target_at_block_480_over_the_quantum() {
        // The stateless session has no latency, so the ramp is heard from the commit.
        let mut stateless = super::super::tests::stateless_session(48_000);
        stateless.sources[0].frames = SOURCE_FRAMES;
        let document = session::canonical_session_json(&stateless).expect("canonical");
        let model = parse_session_json(&document).expect("model");
        let name = model.tracks[0].id.as_str().to_owned();
        let target = set(builtin(&name, FADER_DB, Lane::Both), -12.0, Some(480));
        let target_document = session::canonical_session_json(&applied(
            &document,
            &[set(builtin(&name, FADER_DB, Lane::Both), -12.0, None)],
        ))
        .expect("canonical");
        let mut rig = Rig::new(&document);
        let quantum = rig.quantum as u64;
        assert_eq!(
            rig.latency(),
            0,
            "no latency: the fader lands at the output"
        );
        assert!(
            480 > 2 * quantum && 480 % quantum != 0,
            "the ramp spans several blocks"
        );
        let mut before = Reference::new(&document);
        let mut after = Reference::new(&target_document);
        for _ in 0..4 {
            let live = rig.step();
            assert_eq!(bits(&live), bits(&before.step()), "warm-up");
            after.step();
        }
        assert_eq!(rig.apply(&[target]), crate::RESULT_OK);
        for offset in 0..480_u64.div_ceil(quantum) + 2 {
            let live = rig.step();
            let (old, new) = (before.step(), after.step());
            if offset * quantum < 480 {
                assert_ne!(
                    bits(&live),
                    bits(&old),
                    "block {offset} has left the old fader"
                );
                assert_ne!(
                    bits(&live),
                    bits(&new),
                    "block {offset} has not reached the new one"
                );
            } else {
                assert_eq!(
                    bits(&live),
                    bits(&new),
                    "block {offset} begins 480 samples or more after the commit"
                );
            }
        }
    }

    /// The status and the `(code, operation index)` of each diagnostic of a refused transaction.
    fn refusal(rig: &mut Rig, edits: &[SessionEdit]) -> (protocol::StatusCode, Vec<(String, u32)>) {
        let bytes = rig.transaction(edits);
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK, "a protocol refusal is RESULT_OK");
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("refusal response")
        {
            protocol::DecodedTypedResponseFrame::NonOk { header, payload } => (
                header.status,
                payload
                    .diagnostics
                    .into_iter()
                    .map(|diagnostic| {
                        (
                            diagnostic.code,
                            diagnostic.operation_index.expect("an edit index"),
                        )
                    })
                    .collect(),
            ),
            protocol::DecodedTypedResponseFrame::Success { .. } => {
                panic!("expected a refusal")
            }
        }
    }

    /// Gates 3 and 4: a bad write is refused with its control code at its edit, before anything
    /// changes, and a good edit beside it does not commit. Every `ControlError` code is produced
    /// once through the frame path, and a ramp the row cannot honour or above the rate is refused.
    ///
    /// Red if one bad edit lets a good edit of the same transaction commit (the revision, the
    /// model, a cell or the next block moves), if a fixed-ramp row silently ignores a length, or
    /// if a code is reported at the wrong edit.
    #[test]
    fn a_refused_set_control_writes_nothing_and_names_its_code_and_edit() {
        let document = effect_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let name = model.tracks[0].id.as_str();
        let mut rig = Rig::new(&document);
        let mut untouched = Rig::new(&document);
        for _ in 0..window(rig.latency(), 0, rig.quantum) {
            assert_eq!(bits(&rig.step()), bits(&untouched.step()), "warm-up");
        }
        let good = set(builtin(name, FADER_DB, Lane::Both), -9.0, None);
        let validation = protocol::StatusCode::ValidationFailed;
        // The replay table records each answered request, a refusal included, so it is not state
        // that a refusal must leave alone.
        let state = |rig: &Rig| {
            let (revision, canonical, _, events) = refusal_state(rig);
            let (_, _, epoch, pending) = rig.summary();
            (
                revision,
                canonical,
                events,
                (epoch, pending),
                rig.current_cells(),
            )
        };
        let before = state(&rig);
        let cases = [
            (
                "domain",
                vec![good.clone(), set(builtin(name, PAN, Lane::Both), 2.0, None)],
                "control.domain",
                1,
            ),
            (
                "not finite",
                vec![
                    good.clone(),
                    set(builtin(name, FADER_DB, Lane::Both), f32::NAN, None),
                ],
                "control.domain",
                1,
            ),
            (
                "unknown",
                vec![good.clone(), set(builtin(name, 99, Lane::Both), 0.0, None)],
                "control.address.unknown",
                1,
            ),
            (
                "unknown owner",
                vec![set(builtin("nobody", FADER_DB, Lane::Both), 0.0, None)],
                "control.address.unknown",
                0,
            ),
            (
                "read only",
                vec![set(own(name, EFFECTIVE_FADER_DB, Lane::Left), 0.0, None)],
                "control.read_only",
                0,
            ),
            (
                "inactive",
                vec![
                    good.clone(),
                    set(builtin(name, MATRIX_LL, Lane::Both), 0.5, None),
                ],
                "control.inactive",
                1,
            ),
            (
                "lane",
                vec![set(own(name, PAN_WINDOW_SAMPLES, Lane::Left), 16.0, None)],
                "control.lane",
                0,
            ),
            (
                "fixed ramp",
                vec![
                    good.clone(),
                    set(insert(name, "comp", 1, Lane::Left), -30.0, Some(480)),
                ],
                "control.ramp.fixed",
                1,
            ),
            (
                "monitor",
                vec![set(own(name, SOLO, Lane::Both), 1.0, None)],
                "control.monitor",
                0,
            ),
            (
                "ramp above the rate",
                vec![
                    good.clone(),
                    set(builtin(name, PAN, Lane::Both), 0.25, Some(48_001)),
                ],
                "session.edit.ramp_out_of_range",
                1,
            ),
            (
                "ramp conflict",
                vec![
                    good.clone(),
                    set(builtin(name, FADER_DB, Lane::Both), -3.0, Some(480)),
                ],
                "control.ramp.conflict",
                1,
            ),
        ];
        for (label, edits, code, index) in cases {
            assert_eq!(
                refusal(&mut rig, &edits),
                (validation, vec![(code.to_owned(), index)]),
                "{label}"
            );
            let after = state(&rig);
            assert_eq!(after, before, "{label}");
        }
        for block in 0..8 {
            assert_eq!(bits(&rig.step()), bits(&untouched.step()), "block {block}");
        }
        // The good edit alone commits.
        assert_eq!(rig.apply(&[good]), crate::RESULT_OK);
        assert_eq!(rig.summary().0, before.0 + 1);
    }

    /// `document` with a submix `bus` fed by the first track and sending to the output, as
    /// `submix_session` builds it, and `configure` applied to the bus and the model.
    fn with_bus_strip(
        document: &str,
        configure: impl FnOnce(&mut SessionModel, &mut session::Submix),
    ) -> String {
        with_model(document, |model| {
            let bus = id("bus");
            let mut submix = session::Submix::unity(bus.clone(), &model.console);
            configure(model, &mut submix);
            model.submixes.push(submix);
            let mut send = model.routes[0].clone();
            send.id = id("eq0-bus");
            send.destination = RouteDestination::SubmixInput {
                submix_id: bus.clone(),
            };
            send.follows_mute = false;
            let mut out = model.routes[0].clone();
            out.id = id("bus-main");
            out.source = RouteSource::Submix {
                submix_id: bus,
                tap: session::SendTap::PostPan,
            };
            model.routes.push(send);
            model.routes.push(out);
        })
    }

    /// The 10-track session with a bus fed by the first track. The bus keeps no state of its own
    /// (its filters are off and its console is bypassed), so once a ramp has passed, a live edit
    /// renders like a plan prepared with it from sample 0. The fixture pans both lanes of every
    /// track fully right (`pan` 1.0 and 1.0), so a send from it carries nothing on the bus's left
    /// lane; eq0 is centred to feed both lanes.
    fn stateless_bus_document() -> String {
        stateless_bus_document_with(10, 48_000)
    }

    /// [`stateless_bus_document`] with `track_count` tracks at `sample_rate_hz`.
    fn stateless_bus_document_with(track_count: usize, sample_rate_hz: u32) -> String {
        with_bus_strip(
            &long_session(track_count, sample_rate_hz),
            |model, submix| {
                model.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                    left: 0.0,
                    right: 0.0,
                    smoothing_samples: 16,
                };
                let track = model.tracks[0].clone();
                submix.builtins = track.builtins.clone();
                for lane in [&mut submix.builtins.left, &mut submix.builtins.right] {
                    lane.hpf_hz = 0.0;
                    lane.lpf_hz = 0.0;
                }
                submix.fader = track.fader.clone();
                submix.matrix_or_pan = track.matrix_or_pan.clone();
            },
        )
    }

    /// Gate 1, builtin families: a fader, a mute, a pan and a matrix, on a track and on a submix
    /// strip, commit one revision with the plan epoch unchanged, and from the block their ramp
    /// leaves behind render the bits of a plan prepared from the committed session.
    ///
    /// Red if a submix strip's value is left a rebuild, is written to another strip's cell, or a
    /// family lowers to the wrong lane (the mute in the fader's place, the pan in the matrix's).
    #[test]
    fn submix_strips_take_live_edits() {
        let document = stateless_bus_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let untouched = Reference::run(&document, 40);
        for strip in ["bus", "eq0"] {
            let cases: [(&str, Vec<SessionEdit>); 5] = [
                (
                    "fader left",
                    vec![set(builtin(strip, FADER_DB, Lane::Left), -6.0, None)],
                ),
                (
                    "fader right",
                    vec![set(builtin(strip, FADER_DB, Lane::Right), -6.0, None)],
                ),
                (
                    "mute",
                    vec![set(builtin(strip, MUTE, Lane::Both), 1.0, None)],
                ),
                ("pan", vec![set(builtin(strip, PAN, Lane::Both), 0.5, None)]),
                (
                    "matrix",
                    vec![
                        set(own(strip, MATRIX_MODE, Lane::Both), 1.0, None),
                        set(builtin(strip, MATRIX_LL, Lane::Both), 0.5, None),
                    ],
                ),
            ];
            for (family, edits) in cases {
                let label = format!("{strip} {family}");
                let mut rig = Rig::new(&document);
                for _ in 0..4 {
                    rig.step();
                }
                let (revision, _, epoch, pending) = rig.summary();
                assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
                assert_eq!(
                    (rig.summary().0, rig.summary().2, rig.summary().3),
                    (revision + 1, epoch, pending),
                    "{label}: one revision, the same plan, no candidate"
                );
                let committed = rig.snapshot();
                let k = window(rig.latency(), ramp, rig.quantum);
                let blocks = settle(&mut rig, k, 4);
                let reference = Reference::run(&committed, rig.block);
                let mut heard = false;
                for (block, live) in blocks {
                    assert!(live.iter().any(|sample| *sample != 0.0), "{label}: signal");
                    assert_eq!(
                        bits(&live),
                        bits(&reference[block as usize]),
                        "{label}: block {block}"
                    );
                    heard |= bits(&live) != bits(&untouched[block as usize]);
                }
                assert!(heard, "{label}: the edit changes the output");
                assert_eq!(rig.summary().2, epoch, "{label}: the first epoch renders");
            }
        }
    }

    /// The own-row IDs of a route (`crates/controls/src/route.rs`).
    const ROUTE_GAIN_DB: u32 = 1;
    const ROUTE_MUTE: u32 = 2;
    const ROUTE_MATRIX_LL: u32 = 3;
    const ROUTE_MATRIX_LR: u32 = 4;
    const ROUTE_MATRIX_RL: u32 = 5;

    fn route(route: &str, control: u32) -> ControlAddress {
        address(Owner::Route(id(route)), Section::Own, control, Lane::Both)
    }

    /// Gate 1 (#1557): a send's gain, mute and matrix cells commit one revision with the plan epoch
    /// unchanged, and from the block their ramp leaves behind render the bits of a plan prepared
    /// from the committed session. No render block after the commit allocates or frees, and a
    /// gain edit on a muted send writes no cell, so the later unmute carries the new gain.
    ///
    /// Red if a send edit rebuilds, lowers other coefficients than preparation folds, re-ramps a
    /// muted send, or the route lane adds render work that allocates.
    #[test]
    fn send_values_are_live_and_render_the_prepared_plan_bits() {
        bench_support::alloc::assert_installed();
        let document = stateless_bus_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let untouched = Reference::run(&document, 48);
        type Case = (&'static str, Vec<SessionEdit>);
        let cases: [Case; 6] = [
            (
                "gain",
                vec![set(route("eq0-bus", ROUTE_GAIN_DB), -6.0, None)],
            ),
            ("mute", vec![set(route("eq0-bus", ROUTE_MUTE), 1.0, None)]),
            (
                "matrix, one coefficient",
                vec![set(route("eq0-bus", ROUTE_MATRIX_LR), 0.5, None)],
            ),
            (
                "matrix, four coefficients",
                vec![
                    set(route("eq0-bus", ROUTE_MATRIX_LL), 0.5, None),
                    set(route("eq0-bus", ROUTE_MATRIX_LR), 0.25, None),
                    set(route("eq0-bus", ROUTE_MATRIX_RL), -0.5, None),
                    set(route("eq0-bus", ROUTE_MATRIX_LL + 3), 0.75, None),
                ],
            ),
            (
                "gain, mute and matrix together",
                vec![
                    set(route("eq0-bus", ROUTE_GAIN_DB), -3.0, None),
                    set(route("eq0-bus", ROUTE_MUTE), 1.0, None),
                    set(route("eq0-bus", ROUTE_MATRIX_LR), 0.5, None),
                ],
            ),
            (
                "gain with an explicit step",
                vec![set(route("eq0-bus", ROUTE_GAIN_DB), -9.0, Some(0))],
            ),
        ];
        for (case, edits) in cases {
            let label = format!("send {case}");
            let mut rig = Rig::new(&document);
            for _ in 0..4 {
                rig.step();
            }
            let (revision, _, epoch, pending) = rig.summary();
            assert_eq!(rig.current_route_cells(), vec![0], "{label}: one send lane");
            assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
            assert_eq!(
                (rig.summary().0, rig.summary().2, rig.summary().3),
                (revision + 1, epoch, pending),
                "{label}: one revision, the same plan, no candidate"
            );
            assert_eq!(
                rig.current_route_cells(),
                vec![1],
                "{label}: one write to the send's cell"
            );
            let committed = rig.snapshot();
            let k = window(rig.latency(), ramp, rig.quantum);
            let edit_block = rig.block;
            let mut live = Vec::new();
            for _ in 0..k + 4 {
                let (pcm, used) = rig.step_counted();
                assert_eq!(
                    (used.allocations, used.deallocations),
                    (0, 0),
                    "{label}: a render block allocated or freed"
                );
                live.push(pcm);
            }
            let reference = Reference::run(&committed, rig.block);
            let mut heard = false;
            for (offset, pcm) in live.iter().enumerate().skip(k as usize) {
                let block = edit_block as usize + offset;
                assert!(pcm.iter().any(|sample| *sample != 0.0), "{label}: signal");
                assert_eq!(bits(pcm), bits(&reference[block]), "{label}: block {block}");
                heard |= bits(pcm) != bits(&untouched[block]);
            }
            assert!(heard, "{label}: the edit changes the output");
            assert_eq!(rig.summary().2, epoch, "{label}: the first epoch renders");
        }

        // A gain edit on a muted send writes no cell; the unmute carries the new gain.
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        assert_eq!(
            rig.apply(&[set(route("eq0-bus", ROUTE_MUTE), 1.0, None)]),
            crate::RESULT_OK
        );
        assert_eq!(rig.current_route_cells(), vec![1]);
        assert_eq!(
            rig.apply(&[set(route("eq0-bus", ROUTE_GAIN_DB), -6.0, None)]),
            crate::RESULT_OK
        );
        assert_eq!(
            rig.current_route_cells(),
            vec![1],
            "a gain edit on a muted send writes no cell"
        );
        assert_eq!(
            rig.apply(&[set(route("eq0-bus", ROUTE_MUTE), 0.0, None)]),
            crate::RESULT_OK
        );
        assert_eq!(rig.current_route_cells(), vec![2]);
        assert_eq!(rig.summary().3, 0, "no edit built a candidate");
        let committed = rig.snapshot();
        assert_eq!(
            parse_session_json(&committed)
                .expect("model")
                .routes
                .iter()
                .find(|route| route.id == id("eq0-bus"))
                .expect("send")
                .gain_db,
            -6.0
        );
        let k = window(rig.latency(), ramp, rig.quantum);
        let blocks = settle(&mut rig, k, 4);
        let reference = Reference::run(&committed, rig.block);
        for (block, live) in blocks {
            assert_eq!(
                bits(&live),
                bits(&reference[block as usize]),
                "unmuted send, block {block}"
            );
        }
    }

    /// Gate 2 (#1557): a route into the output keeps its fold, so its gain, mute and matrix are
    /// rebuilds on a post-fader and a pre-fader route, and a send into a submix beside it is live.
    ///
    /// Red if an output route loses its fold through a live edit (a candidate not prepared).
    #[test]
    fn output_route_values_are_rebuilds() {
        let post_fader = with_model(&long_session(10, 48_000), |model| {
            model.routes[0].source = RouteSource::Track {
                track_id: model.tracks[0].id.clone(),
                tap: session::SendTap::PostFader,
            };
        });
        let pre_fader = with_model(&long_session(10, 48_000), |model| {
            model.routes[0].source = RouteSource::Track {
                track_id: model.tracks[0].id.clone(),
                tap: session::SendTap::PreFader,
            };
        });
        let first = parse_session_json(&post_fader).expect("model").routes[0]
            .id
            .as_str()
            .to_owned();
        assert_rebuilds(
            &post_fader,
            &[set(route(&first, ROUTE_GAIN_DB), -6.0, None)],
            "post-fader output route, gain",
        );
        assert_rebuilds(
            &pre_fader,
            &[set(route(&first, ROUTE_MUTE), 1.0, None)],
            "pre-fader output route, mute",
        );
        assert_rebuilds(
            &pre_fader,
            &[set(route(&first, ROUTE_MATRIX_LR), 0.5, None)],
            "pre-fader output route, matrix",
        );
    }

    /// Gate 4 (#1557): a transaction of a valid send gain and an out-of-domain matrix value is
    /// refused whole: no send cell is written and the revision does not move.
    ///
    /// Red if the gain is written before the matrix value is refused.
    #[test]
    fn a_send_transaction_with_an_out_of_domain_value_writes_no_cell() {
        let document = stateless_bus_document();
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let (revision, _, epoch, pending) = rig.summary();
        let (status, diagnostics) = refusal(
            &mut rig,
            &[
                set(route("eq0-bus", ROUTE_GAIN_DB), -6.0, None),
                set(route("eq0-bus", ROUTE_MATRIX_LR), 100.0, None),
            ],
        );
        assert_ne!(
            status,
            protocol::StatusCode::Ok,
            "the out-of-domain value is refused"
        );
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].1, 1, "the matrix edit, the second");
        assert_eq!(
            (rig.summary().0, rig.summary().2, rig.summary().3),
            (revision, epoch, pending),
            "no revision, no candidate"
        );
        assert_eq!(
            rig.current_route_cells(),
            vec![0],
            "no send cell was written"
        );
        assert_eq!(
            rig.apply(&[set(route("eq0-bus", ROUTE_GAIN_DB), -6.0, None)]),
            crate::RESULT_OK,
            "the valid half applies alone"
        );
        assert_eq!(rig.current_route_cells(), vec![1]);
    }

    /// Gate 1, effect family on a submix strip: a console slot's parameter, an insert's
    /// parameter and an insert's bypass commit live and render the bits of the browser's lane
    /// (host-core records pushed to the bus's own producers at the same block).
    ///
    /// Red if a submix effect is left a rebuild, or its record reaches another strip's instance
    /// of the same name (every track carries `comp` and `clip` too).
    #[test]
    fn submix_effects_take_live_edits() {
        use effect_contract::ParameterChannel::{Left, Right};
        let document = with_bus_strip(&effect_session(10, 48_000), |model, submix| {
            let template = model.tracks[0].inserts.effects[0].clone();
            let mut compressor = template;
            compressor.id = id("bcomp");
            compressor.bypass = false;
            submix.inserts.effects.push(compressor);
            for entry in &mut submix.console {
                entry.bypass = false;
            }
        });
        // A case: its label, its edits, the records the browser's lane gets for them, and whether it
        // bypasses the bus's compressor.
        type Push = (&'static str, u32, effect_contract::ParameterChannel, f32);
        type Case = (&'static str, Vec<SessionEdit>, Vec<Push>, bool);
        let cases: [Case; 2] = [
            (
                "parameters",
                vec![
                    set(insert("bus", "bcomp", 1, Lane::Left), -30.0, None),
                    set(console("bus", "clip", 1, Lane::Both), 12.0, None),
                ],
                vec![
                    ("bcomp", 1, Left, -30.0),
                    ("clip", 1, Left, 12.0),
                    ("clip", 1, Right, 12.0),
                ],
                false,
            ),
            (
                "bypass",
                vec![set(insert("bus", "bcomp", 0, Lane::Both), 1.0, None)],
                Vec::new(),
                true,
            ),
        ];
        for (label, edits, pushes, bypass) in cases {
            let mut rig = Rig::new(&document);
            let mut lanes = LaneReference::new(&document);
            let mut unedited = Reference::new(&document);
            for block in 0..window(rig.latency(), 0, rig.quantum) {
                let live = rig.step();
                assert_eq!(
                    bits(&live),
                    bits(&lanes.reference.step()),
                    "{label}: warm-up {block}"
                );
                unedited.step();
            }
            let (revision, _, epoch, pending) = rig.summary();
            assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
            assert_eq!(
                (rig.summary().0, rig.summary().2, rig.summary().3),
                (revision + 1, epoch, pending),
                "{label}: one revision, the same plan, no candidate"
            );
            for (effect, parameter_id, channel, value) in pushes {
                lanes.push("bus", effect, parameter_id, channel, value);
            }
            if bypass {
                lanes.write(
                    "bus",
                    "bcomp",
                    effect_contract::EffectControlRecord::Bypass(true),
                );
            }
            let mut heard = false;
            for block in 0..12 {
                let live = rig.step();
                let expected = lanes.reference.step();
                assert!(live.iter().any(|sample| *sample != 0.0), "{label}: signal");
                assert_eq!(bits(&live), bits(&expected), "{label}: block {block}");
                heard |= bits(&live) != bits(&unedited.step());
            }
            assert!(heard, "{label}: the edit changes the output");
            assert_eq!(rig.summary().2, epoch, "{label}: the first epoch renders");
        }
    }

    /// Gate 1, an EQ parameter on a submix strip: prepared targets designed for the bus's own EQ.
    ///
    /// Red if a submix EQ is left a rebuild, or its targets are published to another strip's EQ.
    #[test]
    fn a_submix_eq_takes_live_edits() {
        use effect_contract::ParameterChannel::Both;
        let document = with_bus_strip(&eq_session(48_000), |model, submix| {
            let last = model.tracks.len() - 1;
            let mut eq = model.tracks[last].inserts.effects[0].clone();
            eq.id = id("eq-bus");
            submix.inserts.effects.push(eq);
        });
        let mut rig = Rig::new(&document);
        let mut lanes = LaneReference::new(&document);
        let mut unedited = Reference::new(&document);
        for block in 0..window(rig.latency(), 0, rig.quantum) {
            assert_eq!(
                bits(&rig.step()),
                bits(&lanes.reference.step()),
                "warm-up {block}"
            );
            unedited.step();
        }
        let edits = [
            set(insert("bus", "eq-bus", 65, Lane::Both), 1.0, None),
            set(insert("bus", "eq-bus", 66, Lane::Both), 300.0, None),
        ];
        let (revision, _, epoch, pending) = rig.summary();
        assert_eq!(rig.apply(&edits), crate::RESULT_OK);
        assert_eq!(
            (rig.summary().0, rig.summary().2, rig.summary().3),
            (revision + 1, epoch, pending),
            "one revision, the same plan, no candidate"
        );
        lanes.publish_eq("bus", "eq-bus", &[(65, Both, 1.0), (66, Both, 300.0)]);
        let mut heard = false;
        for block in 0..12 {
            let live = rig.step();
            let expected = lanes.reference.step();
            assert!(live.iter().any(|sample| *sample != 0.0), "signal");
            assert_eq!(bits(&live), bits(&expected), "block {block}");
            heard |= bits(&live) != bits(&unedited.step());
        }
        assert!(heard, "the edit changes the output");
    }

    /// Gate 2: a rebuild-only value rebuilds even when a live value rides in the same
    /// transaction, and a live edit does not hide a structural one.
    ///
    /// Red if the live part of a mixed transaction is acked as live while the rest is dropped.
    #[test]
    fn a_rebuild_only_row_in_a_mixed_transaction_rebuilds() {
        let document = long_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let name = model.tracks[0].id.as_str();
        assert_rebuilds(
            &document,
            &[
                set(builtin(name, FADER_DB, Lane::Both), -6.0, None),
                set(builtin(name, DELAY_SAMPLES, Lane::Both), 64.0, None),
            ],
            "a fader and an input delay",
        );
        assert_rebuilds(
            &document,
            &[
                set(builtin(name, FADER_DB, Lane::Both), -6.0, None),
                content_edit(&model),
            ],
            "a fader and a content edit",
        );
    }

    /// A live submix fader ramp in flight is carried across a structural rebuild, as a track's is
    /// (#1554: `join_strips`): the blocks of the rebuilt rig equal those of a twin that never
    /// rebuilt, ramp and all, and the swap lands inside the ramp.
    ///
    /// Red if a submix strip restarts at rest at the swap: its fader would jump to the ramp's
    /// target at once, where the twin is still ramping.
    #[test]
    fn a_live_submix_fader_ramp_is_carried_across_a_rebuild() {
        // One stateless track (no filter, console or insert: a rebuild carries it with no gap)
        // sent to a stateless bus, as `live_fader_across_a_carrying_swap` builds its session.
        let mut base = super::tests::stateless_session(48_000);
        base.tracks.truncate(1);
        base.routes.truncate(1);
        base.sources[0].frames = SOURCE_FRAMES;
        let structural = super::tests::add_muted_track(&base, "a-muted");
        let mut model = base;
        let bus = id("bus");
        let mut send = model.routes[0].clone();
        send.id = id("eq0-bus");
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        let mut out = model.routes[0].clone();
        out.id = id("bus-main");
        out.source = RouteSource::Submix {
            submix_id: bus.clone(),
            tap: session::SendTap::PostPan,
        };
        model.routes.push(send);
        model.routes.push(out);
        let console = model.console.clone();
        model.submixes.push(session::Submix::unity(bus, &console));
        let document = session::canonical_session_json(&model).expect("canonical");
        let ramp = model.control_smoothing_samples().fader;
        assert!(ramp >= 256, "the session's default fader ramp spans blocks");
        let live = [set(builtin("bus", FADER_DB, Lane::Both), -12.0, None)];

        // The fader edit commits first (the successor is prepared from a base that holds it), or
        // in the very transaction that rebuilds (the join sees the difference and retargets).
        for combined in [false, true] {
            let label = if combined {
                "one transaction"
            } else {
                "two transactions"
            };
            let mut rebuilt = Rig::new(&document);
            let mut twin = Rig::new(&document);
            for _ in 0..4 {
                assert_eq!(bits(&rebuilt.step()), bits(&twin.step()));
            }
            assert_eq!(twin.apply(&live), crate::RESULT_OK);
            if combined {
                let both: Vec<SessionEdit> = live
                    .iter()
                    .cloned()
                    .chain(structural.iter().cloned())
                    .collect();
                assert_eq!(rebuilt.apply(&both), crate::RESULT_OK, "{label}");
            } else {
                assert_eq!(rebuilt.apply(&live), crate::RESULT_OK, "{label}");
                assert_eq!(rebuilt.summary().3, 0, "{label}: the fader edit is live");
                assert_eq!(rebuilt.apply(&structural), crate::RESULT_OK, "{label}");
            }
            assert_eq!(rebuilt.summary().3, 1, "{label}: the transaction rebuilds");
            let settled = Reference::run(&rebuilt.snapshot(), 40);
            let mut swap = None;
            let mut mid_ramp = false;
            for _ in 0..(window(rebuilt.latency(), ramp, rebuilt.quantum) + 4) {
                let block = rebuilt.block;
                let (one, two) = (rebuilt.step(), twin.step());
                assert_eq!(
                    bits(&one),
                    bits(&two),
                    "{label}: block {block} equals the twin that never rebuilt"
                );
                if swap.is_none() && crate::ffi::test_plan_carry_counts(rebuilt.plan).0 == 1 {
                    swap = Some(block);
                    // Still ramping: a stage restarted at rest would already be at the settled
                    // value.
                    mid_ramp = bits(&one) != bits(&settled[block as usize]);
                }
            }
            assert!(swap.is_some(), "{label}: the successor swapped in");
            assert!(mid_ramp, "{label}: the swap landed inside the ramp");
        }
    }

    // Issue #1558: VCA moves, VCA membership and send follows.

    /// The own-row IDs of a VCA and the follow row of a route.
    const VCA_OFFSET_DB: u32 = 1;
    const VCA_MUTE: u32 = 2;
    const ROUTE_FOLLOWS_MUTE: u32 = 7;

    fn vca_row(vca: &str, control: u32, lane: Lane) -> ControlAddress {
        address(Owner::Vca(id(vca)), Section::Own, control, lane)
    }

    fn vca_of(name: &str, members: &[&str]) -> session::Vca {
        session::Vca {
            id: id(name),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: members.iter().map(|member| id(member)).collect(),
        }
    }

    /// [`stateless_bus_document`] with a pre-fader send `eq0-bus` that follows `eq0`'s mute, and
    /// VCA `outer` holding VCA `inner` holding `eq0`.
    fn vca_document() -> String {
        vca_document_with(10, 48_000)
    }

    /// [`vca_document`] with `track_count` tracks at `sample_rate_hz`.
    fn vca_document_with(track_count: usize, sample_rate_hz: u32) -> String {
        with_model(
            &stateless_bus_document_with(track_count, sample_rate_hz),
            |model| {
                let send = model
                    .routes
                    .iter_mut()
                    .find(|route| route.id == id("eq0-bus"))
                    .expect("the send");
                send.follows_mute = true;
                if let RouteSource::Track { tap, .. } = &mut send.source {
                    *tap = session::SendTap::PreFader;
                }
                model.vcas.push(vca_of("inner", &["eq0"]));
                model.vcas.push(vca_of("outer", &["inner"]));
            },
        )
    }

    /// Gate 1 (#1558): a VCA's offset and mute, a strip's own fader and mute under a VCA, a send's
    /// `follows_mute`, a member added, a VCA added and a VCA removed each commit one revision with
    /// the plan epoch unchanged and no candidate, and from the block their ramp leaves behind
    /// render the bits of a plan prepared from the committed session. No render block allocates.
    ///
    /// Red if a VCA, membership or follow edit rebuilds, or if the live composition differs from
    /// preparation's (sum order, clamp, mute OR, follow column).
    #[test]
    fn vca_membership_and_follow_values_are_live_and_render_the_prepared_plan_bits() {
        bench_support::alloc::assert_installed();
        for rate in [44_100, 48_000, 88_200, 96_000] {
            for tracks in [1, 10] {
                live_vca_sequence(tracks, rate);
            }
        }
    }

    /// The sequence of the test above, on a session of `tracks` tracks at `rate`.
    fn live_vca_sequence(tracks: usize, rate: u32) {
        let document = vca_document_with(tracks, rate);
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let upsert = |vca: session::Vca| SessionEdit::UpsertVca { vca };
        let mut inner_two = vca_of("inner", &["eq0", "eq1"]);
        inner_two.fader.left_db = -6.0;
        inner_two.fader.right_db = -6.0;
        // `(label, edits, audible)`: a step is audible when the output differs from the previous
        // committed session's.
        let steps: Vec<(&str, Vec<SessionEdit>, bool)> = vec![
            (
                "inner offset, both lanes",
                vec![set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -6.0, None)],
                true,
            ),
            (
                "outer mute, left lane",
                vec![set(vca_row("outer", VCA_MUTE, Lane::Left), 1.0, None)],
                true,
            ),
            (
                "own fader",
                vec![set(builtin("eq0", FADER_DB, Lane::Both), -3.0, None)],
                true,
            ),
            // The left lane is muted by the VCA already: no effective value moves.
            (
                "own mute, left lane under a VCA mute",
                vec![set(builtin("eq0", MUTE, Lane::Left), 1.0, None)],
                false,
            ),
            (
                "own mute, both lanes",
                vec![set(builtin("eq0", MUTE, Lane::Both), 1.0, None)],
                true,
            ),
            (
                "own unmute",
                vec![set(builtin("eq0", MUTE, Lane::Both), 0.0, None)],
                true,
            ),
            (
                "follows_mute off while the source is muted",
                vec![set(route("eq0-bus", ROUTE_FOLLOWS_MUTE), 0.0, None)],
                true,
            ),
            (
                "follows_mute on",
                vec![set(route("eq0-bus", ROUTE_FOLLOWS_MUTE), 1.0, None)],
                true,
            ),
            ("a member added", vec![upsert(inner_two)], true),
            (
                "a VCA added over the bus",
                vec![upsert({
                    let mut over = vca_of("over-bus", &["bus"]);
                    // The left lane of the bus is silent here (eq0 is muted on the left): the right moves.
                    over.fader.right_db = -4.0;
                    over
                })],
                true,
            ),
            (
                "a VCA removed",
                vec![SessionEdit::RemoveVca {
                    vca_id: id("outer"),
                }],
                true,
            ),
        ];
        let mut previous = document.clone();
        // The source is finite: a rig serves a few steps, then the next starts from the committed
        // session of the last.
        let mut rig = Rig::new(&document);
        // A second track exists only in the ten-track session.
        let steps = steps
            .into_iter()
            .filter(|(label, ..)| tracks == 10 || *label != "a member added");
        for (served, (label, edits, audible)) in steps.enumerate() {
            let label = format!("{label} ({tracks} tracks, {rate} Hz)");
            if served % 3 == 0 {
                rig = Rig::new(&previous);
                for _ in 0..4 {
                    rig.step();
                }
            }
            let (revision, _, epoch, pending) = rig.summary();
            assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{label}");
            assert_eq!(
                (rig.summary().0, rig.summary().2, rig.summary().3),
                (revision + 1, epoch, pending),
                "{label}: one revision, the same plan, no candidate"
            );
            let committed = rig.snapshot();
            let k = window(rig.latency(), ramp, rig.quantum);
            let edit_block = rig.block;
            let mut live = Vec::new();
            for _ in 0..k + 4 {
                let (pcm, used) = rig.step_counted();
                assert_eq!(
                    (used.allocations, used.deallocations),
                    (0, 0),
                    "{label}: a render block allocated or freed"
                );
                live.push(pcm);
            }
            let reference = Reference::run(&committed, rig.block);
            let before = Reference::run(&previous, rig.block);
            let mut heard = false;
            for (offset, pcm) in live.iter().enumerate().skip(k as usize) {
                let block = edit_block as usize + offset;
                // The one-track session is silent while its track is muted.
                assert!(
                    tracks == 1 || pcm.iter().any(|sample| *sample != 0.0),
                    "{label}: signal"
                );
                assert_eq!(bits(pcm), bits(&reference[block]), "{label}: block {block}");
                heard |= bits(pcm) != bits(&before[block]);
            }
            assert_eq!(
                heard, audible,
                "{label}: the edit is heard exactly when it moves a value"
            );
            previous = committed;
        }
    }

    /// Gate 4 (#1558): with `maximum_vcas` equal to the current count, a VCA added live is refused
    /// like a compile, with the compile's own diagnostic, and the model, the revision and every
    /// cell are as they were. A VCA that stays inside the cap is live.
    ///
    /// Red if a live edit commits a model that its own rebuild refuses.
    #[test]
    fn a_vca_added_over_the_cap_is_refused_like_a_compile() {
        let document = vca_document();
        let capped = CompileLimits {
            maximum_vcas: 2,
            ..limits()
        };
        let mut rig = Rig::with_limits(&document, capped);
        rig.step();
        let before = refusal_state(&rig);
        let cells = rig.current_cells();
        let routes = rig.current_route_cells();
        let third = SessionEdit::UpsertVca {
            vca: vca_of("third", &["eq1"]),
        };
        assert_eq!(rig.apply(&[third]), crate::RESULT_COMPILE_REJECTED);
        let over = with_model(&document, |model| {
            model.vcas.push(vca_of("third", &["eq1"]))
        });
        let Err(failure) = compile_children(&over, capped, None) else {
            panic!("a third VCA does not prepare under a cap of two")
        };
        assert_eq!(rig.last_error(), failure.diagnostics);
        assert!(
            String::from_utf8_lossy(&rig.last_error()).contains("host.resource.count"),
            "{}",
            String::from_utf8_lossy(&rig.last_error())
        );
        assert_eq!(refusal_state(&rig), before);
        assert_eq!(rig.current_cells(), cells);
        assert_eq!(rig.current_route_cells(), routes);
        assert_eq!(rig.summary().3, 0, "no candidate");
        // Replacing a VCA keeps the count: live.
        assert_eq!(
            rig.apply(&[SessionEdit::UpsertVca {
                vca: vca_of("inner", &["eq0", "eq1"]),
            }]),
            crate::RESULT_OK
        );
        assert_eq!(rig.summary().3, 0, "a member change is live");
    }

    /// Gate 5 (#1558): the ramp of a record is the ramp of the edit that moved it. Each case is a
    /// VCA edit against the per-kind edit of the same effective value on a twin rig, and the two
    /// render the same bits from the block after the commit on.
    ///
    /// Red if a record takes the wrong row's length, drops a per-edit length, or ramps a follow
    /// independently of its source's mute.
    #[test]
    fn vca_and_follow_records_take_the_ramp_of_the_edit_that_moved_them() {
        let document = vca_document();
        let model = parse_session_json(&document).expect("model");
        let own = model.tracks[0].fader.clone();
        let effective = |db: f32| session::vca_effective_db(own.left_db, [db]);
        assert_eq!(own.left_db, own.right_db, "the fixture's lanes agree");
        // The VCA's offset ramps over its edit's 480 samples.
        live_twin(
            &document,
            &[set(
                vca_row("inner", VCA_OFFSET_DB, Lane::Both),
                -6.0,
                Some(480),
            )],
            &[set(
                builtin("eq0", FADER_DB, Lane::Both),
                effective(-6.0),
                Some(480),
            )],
            "a VCA offset with its own ramp",
        );
        // Without a length the offset ramps over `fader_ms`, as the strip's own edit does.
        live_twin(
            &document,
            &[set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -6.0, None)],
            &[set(
                builtin("eq0", FADER_DB, Lane::Both),
                effective(-6.0),
                None,
            )],
            "a VCA offset with no ramp",
        );
        // The strip's own edit first: its 0 steps the lane it moved, and the VCA's 480 is the
        // length of nothing else, so the sum steps.
        live_twin(
            &document,
            &[
                set(
                    builtin("eq0", FADER_DB, Lane::Both),
                    own.left_db - 2.0,
                    Some(0),
                ),
                set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -6.0, Some(480)),
            ],
            &[set(
                builtin("eq0", FADER_DB, Lane::Both),
                session::vca_effective_db(own.left_db - 2.0, [-6.0]),
                Some(0),
            )],
            "an own edit beside a VCA edit",
        );
        // A VCA mute ramps over its edit's length, and the following send ramps with it: the same
        // mute and the send's own mute edit, both 96, give the same output.
        live_twin(
            &document,
            &[set(vca_row("inner", VCA_MUTE, Lane::Both), 1.0, Some(96))],
            &[
                set(builtin("eq0", MUTE, Lane::Both), 1.0, Some(96)),
                set(route("eq0-bus", ROUTE_MUTE), 1.0, Some(96)),
            ],
            "a VCA mute and its following send",
        );
    }

    /// Gate 6 (#1558, D5): a VCA added live adds its rows, so the old structure epoch is stale and
    /// a fresh listing has the new VCA's four rows live and automatable; an offset edit keeps the
    /// epoch, so a client that holds it still pages.
    ///
    /// Red if a live VCA add keeps stale handles or lists the VCA rows with a stale class.
    #[test]
    fn a_vca_added_live_advances_the_epoch_and_lists_its_rows_live() {
        let document = vca_document();
        let mut rig = Rig::new(&document);
        rig.step();
        let (before, epoch, _) = control_list_all_c(rig.session, &mut rig.request, 256);
        let vca_rows = |list: &[protocol::ControlEntry], vca: &str| {
            list.iter()
                .filter(|entry| entry.address.owner_id.as_deref() == Some(vca))
                .filter_map(|entry| control_of(entry).cloned())
                .collect::<Vec<_>>()
        };
        assert_eq!(vca_rows(&before, "inner").len(), 4);
        assert!(
            vca_rows(&before, "inner")
                .iter()
                .all(|row| { row.class == protocol::ControlClass::Live && row.automatable })
        );
        assert!(vca_rows(&before, "extra").is_empty());

        let page = |rig: &mut Rig, epoch: u64| {
            let request = rig.next_request();
            control_list_c(
                rig.session,
                request,
                &protocol::ControlListRequest {
                    structure_epoch: epoch,
                    after_handle: 0,
                    limit: 1,
                    owner: None,
                },
            )
        };
        // An offset edit keeps the epoch: the old one still pages.
        assert_eq!(
            rig.apply(&[set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -3.0, None)]),
            crate::RESULT_OK
        );
        assert!(page(&mut rig, epoch).is_ok());
        // A member change keeps it too.
        assert_eq!(
            rig.apply(&[SessionEdit::UpsertVca {
                vca: vca_of("inner", &["eq0", "eq1"]),
            }]),
            crate::RESULT_OK
        );
        assert!(page(&mut rig, epoch).is_ok());
        // A VCA added changes the rows.
        assert_eq!(
            rig.apply(&[SessionEdit::UpsertVca {
                vca: vca_of("extra", &["eq2"]),
            }]),
            crate::RESULT_OK
        );
        assert_eq!(rig.summary().3, 0, "the add is live");
        assert_eq!(
            page(&mut rig, epoch).err().map(|(status, _)| status),
            Some(protocol::StatusCode::RevisionConflict),
            "the old epoch is stale"
        );
        let (after, fresh, _) = control_list_all_c(rig.session, &mut rig.request, 256);
        assert_eq!(fresh, epoch + 1);
        let rows = vca_rows(&after, "extra");
        assert_eq!(rows.len(), 4, "offset_db and mute, left and right");
        assert!(
            rows.iter()
                .all(|row| { row.class == protocol::ControlClass::Live && row.automatable })
        );
        // And the removal is a second advance.
        assert_eq!(
            rig.apply(&[SessionEdit::RemoveVca {
                vca_id: id("extra")
            }]),
            crate::RESULT_OK
        );
        let (gone, last, _) = control_list_all_c(rig.session, &mut rig.request, 256);
        assert_eq!(last, fresh + 1);
        assert!(vca_rows(&gone, "extra").is_empty());
    }
}
