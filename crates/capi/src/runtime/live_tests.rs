//! Value-only track fader, mute and pan transactions on the running C ABI plan (#1257 gates 1-4).
//!
//! Every edit goes through `miso_engine_v1_submit_command`. The source is fed one quantum per block
//! for the whole run, with no zero sample and distinct lanes, and never seeks except where a test
//! rebuilds the plan on purpose.

use super::tests::{
    SESSION, add_muted_track, boxed_c_children_with_limits, command_bytes,
    command_bytes_at_revision, command_c, control_list_all_c, control_list_c, control_state_all_c,
    counter_telemetry_for, event_c, generated_parity_session, limits, lossy_counter_snapshots,
    stateless_session, submit_c,
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
            let decoded = ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("snapshot response");
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(page),
                ..
            } = decoded
            else {
                let protocol::DecodedTypedResponseFrame::NonOk { header, payload } = decoded else {
                    panic!("expected a snapshot page")
                };
                panic!(
                    "expected a snapshot page, got {:?} {:?}",
                    header.status, payload.diagnostics
                )
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
        self.step_planes(feed.chunk(start, self.quantum))
    }

    /// One block fed from the given planes, which start at this block's first frame.
    fn step_planes(&mut self, [left, right]: [Vec<f32>; 2]) -> Vec<f32> {
        let start = self.block * self.quantum as u64;
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

/// The `SET_CONTROL`s that give strip `track_id` the whole fader `fader` (decision 17, #1561).
fn fader_edit(track_id: &str, fader: DualMonoFader) -> Vec<SessionEdit> {
    protocol::control_core::fader_edits(&StableId::parse(track_id).expect("track ID"), &fader)
}

/// The two `SET_CONTROL`s that set the fader dB of strip `track_id` (left, right).
fn db_edits(track_id: &str, left_db: f32, right_db: f32) -> Vec<SessionEdit> {
    [
        (protocol::control_core::Lane::Left, left_db),
        (protocol::control_core::Lane::Right, right_db),
    ]
    .into_iter()
    .map(|(lane, db)| {
        SessionEdit::set_control(
            protocol::control_core::row_address(
                protocol::control_core::Owner::Strip(StableId::parse(track_id).expect("track ID")),
                protocol::control_core::Section::Builtins,
                "fader_db",
                lane,
            )
            .expect("the fader row"),
            db,
            None,
        )
    })
    .collect()
}

/// One `SET_CONTROL` that sets the mute of both lanes of strip `track_id`. A `SET_CONTROL` is about
/// 160 bytes, so a transaction over ten strips stays inside the test limit's 4 KiB frame only when
/// it writes the rows that change.
fn mute_edit(track_id: &str, muted: bool) -> SessionEdit {
    SessionEdit::set_control(
        protocol::control_core::row_address(
            protocol::control_core::Owner::Strip(StableId::parse(track_id).expect("track ID")),
            protocol::control_core::Section::Builtins,
            "mute",
            protocol::control_core::Lane::Both,
        )
        .expect("the mute row"),
        f32::from(u8::from(muted)),
        None,
    )
}

/// The `SET_CONTROL`s that give strip `track_id` the whole pan or matrix `shape`.
fn matrix_edit(track_id: &StableId, shape: MatrixOrPan) -> Vec<SessionEdit> {
    protocol::control_core::matrix_or_pan_edits(track_id, &shape)
}

/// The `SET_CONTROL`s of the three session rows that give `control_smoothing` the explicit
/// `lengths`. Since #1561 no transaction writes the `default` arm back.
fn smoothing_edits(lengths: ControlSmoothing) -> Vec<SessionEdit> {
    [
        ("fader_ms", lengths.fader_ms),
        ("mute_ms", lengths.mute_ms),
        ("pan_ms", lengths.pan_ms),
    ]
    .into_iter()
    .map(|(row, value)| {
        SessionEdit::set_control(
            protocol::control_core::row_address(
                protocol::control_core::Owner::Session,
                protocol::control_core::Section::Own,
                row,
                protocol::control_core::Lane::Both,
            )
            .expect("a session row"),
            value,
            None,
        )
    })
    .collect()
}

fn mute_all(model: &SessionModel, muted: bool) -> Vec<SessionEdit> {
    model
        .tracks
        .iter()
        .map(|track| mute_edit(track.id.as_str(), muted))
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
    let mut edits = fader_edit(
        fader_track,
        DualMonoFader {
            left_db: -6.0,
            right_db: 3.0,
            left_mute: false,
            right_mute: false,
        },
    );
    edits.extend(matrix_edit(
        &StableId::parse(matrix_track).expect("matrix track"),
        MatrixOrPan::Matrix {
            ll: 0.75,
            lr: 0.125,
            rl: -0.25,
            rr: 0.5,
            smoothing_samples: smoothing,
        },
    ));
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

/// `edits` with every fader edit's `ramp_samples` set to `ramp` (#1394).
fn with_ramp(edits: Vec<SessionEdit>, ramp: Option<u32>) -> Vec<SessionEdit> {
    edits
        .into_iter()
        .map(|mut edit| {
            let SessionEdit::SetControl { ramp_samples, .. } = &mut edit else {
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

    /// One block fed the two planes `[left, right]`, with the calling thread's allocator events
    /// inside the `render` call.
    fn step_planes_counted(
        &mut self,
        [left, right]: [Vec<f32>; 2],
    ) -> (Vec<f32>, bench_support::alloc::Counters) {
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
/// edit rebuilds; the swap, a seek, and the blocks until the carried lines flush plus six, which it
/// returns. Every run feeds the same source over the same blocks and swaps at the same block, so
/// whatever the swap carries (the strip input sections, #1276) is the same in every run, and the
/// rebuilt blocks differ only by what the successor was prepared from.
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
        model
            .tracks
            .iter()
            .step_by(2)
            .map(|track| mute_edit(track.id.as_str(), true))
            .collect(),
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
    // The lines flush `flushed` blocks after the swap; six more are compared.
    let flushed =
        usize::try_from(u64::from(rig.latency()).div_ceil(rig.quantum as u64)).expect("blocks");
    let rebuilt = (0..flushed + 6).map(|_| rig.step()).collect();
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
    // The rebuild carries the compensation lines (#1283), so the audio a line held at the swap
    // plays out after it: the live mute silenced the last block of the muted strips' lines, and
    // the baked mute takes effect at the swap, so the lines of those strips still hold that block
    // unmuted, as a live mute at the swap would. The two runs agree once the lines are flushed,
    // which is the session's latency after the swap block.
    let (latency, quantum) = {
        let rig = Rig::new(&long_session_with(10, 48_000, HARD_SWITCH));
        (rig.latency(), rig.quantum)
    };
    let flushed = usize::try_from(u64::from(latency).div_ceil(quantum as u64)).expect("blocks");
    let blocks = |run: Vec<Vec<f32>>| {
        run[flushed..]
            .iter()
            .map(|pcm| bits(pcm))
            .collect::<Vec<_>>()
    };
    let live = blocks(persistence_run(Mute::Live));
    let baked = blocks(persistence_run(Mute::Baked));
    let unmuted = blocks(persistence_run(Mute::Unmuted));
    assert!(!live.is_empty(), "blocks remain after the lines flush");
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
        edits.extend(smoothing_edits(smoothing));
    }
    edits.push(content_edit(&model));
    let last = model.tracks.len() - 1;
    for (index, track) in model.tracks.iter().enumerate() {
        let id = track.id.as_str();
        if index % 2 == 0 {
            edits.push(mute_edit(id, true));
        } else {
            edits.extend(db_edits(id, -6.0, 3.0));
        }
    }
    edits.extend(matrix_edit(
        &model.tracks[last].id,
        MatrixOrPan::Matrix {
            ll: 0.75,
            lr: 0.125,
            rl: -0.25,
            rr: 0.5,
            smoothing_samples: 0,
        },
    ));
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

/// A `SET_CONTROL` of the left fader dB of strip `track_id`.
fn left_db_edit(track_id: &str, left_db: f32) -> SessionEdit {
    SessionEdit::set_control(
        protocol::control_core::row_address(
            protocol::control_core::Owner::Strip(StableId::parse(track_id).expect("track ID")),
            protocol::control_core::Section::Builtins,
            "fader_db",
            protocol::control_core::Lane::Left,
        )
        .expect("the fader row"),
        left_db,
        None,
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
            rig.apply(&both(-1.0 - edit as f32 * 0.25)),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(rig.writes(Epoch::Current, "eq0"), (80, 0));
    assert_eq!(twin.apply(&both(last)), crate::RESULT_OK);
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

/// Issue #1351 gate 7 (#1348 verdict N-3): the service step refreshes the provider's counters
/// before it stages the snapshot, so a counter that render changed since the last call reaches the
/// snapshot that the next call stages. Three fader edits on one track, then one block, replace two
/// writes of each of the two fader cells: render adds `2 x 2 = 4` to `LIVE_VALUES_SUPERSEDED`
/// (#1312 D2), and the next call, a `source_submit`, stages exactly that.
///
/// Test value: red if the service step stages the snapshot before refreshing the provider (the
/// snapshot then reads the stale 0), which no other counter shows because the telemetry counters
/// change only inside the step itself.
#[test]
fn the_first_refresh_puts_a_render_changed_counter_in_the_next_snapshot() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let request = rig.next_request();
    let configure = counter_telemetry_for(
        request,
        SessionRevision(rig.summary().0),
        vec![protocol::CounterId::LiveValuesSuperseded],
        1,
    );
    assert_eq!(rig.send(&configure).0, crate::RESULT_OK);
    for edit in 0..3 {
        let fader = DualMonoFader {
            left_db: -1.0 - edit as f32,
            right_db: -1.0 - edit as f32,
            left_mute: false,
            right_mute: false,
        };
        assert_eq!(
            rig.apply(&fader_edit("eq0", fader)),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert!(
        lossy_counter_snapshots(rig.session).is_empty(),
        "no block was rendered since the configuration, so nothing is staged"
    );
    rig.render();
    // The next call is a source submit, not a command.
    let [left, right] = source_chunk(rig.fed, rig.quantum);
    submit_c(
        rig.session,
        rig.generation,
        rig.fed,
        rig.rate,
        &left,
        &right,
        false,
    );
    let snapshots = lossy_counter_snapshots(rig.session);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(
        snapshots[0].values,
        vec![protocol::CounterValue {
            id: protocol::CounterId::LiveValuesSuperseded,
            value: 4,
        }]
    );
    assert_eq!(rig.counter(protocol::CounterId::LiveValuesSuperseded), 4);
}

/// Issue #1521 gate 1: the session's `live_block_errors` mirror reaches `COUNTERS_GET` and a
/// configured `COUNTER_SNAPSHOT`. A fresh session serves `LIVE_BLOCK_ERRORS` at 0; after five
/// blocks, a store of 3 through the plan's own `queries` (D6's seam) is what the session reads, so
/// the plan and the session share one mirror and the service step copies it. No valid plan reaches a counted error block, so the seam stands in for render's store.
///
/// Test value: red if the service step never copies the mirror into the provider, copies another
/// source (the render sequence 5 or the carry count 0 instead of 3), or the counter is missing
/// from the served set, so the configuration is refused; no other test reads the mirror outside
/// `control-plane`'s `plan.rs`.
#[test]
fn the_live_block_error_mirror_reaches_counters_get_and_the_snapshot() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    assert_eq!(rig.counter(protocol::CounterId::LiveBlockErrors), 0);
    for _ in 0..5 {
        rig.step();
    }
    crate::ffi::test_store_live_block_errors(rig.plan, 3);
    assert_eq!(rig.counter(protocol::CounterId::LiveBlockErrors), 3);
    let request = rig.next_request();
    let configure = counter_telemetry_for(
        request,
        SessionRevision(rig.summary().0),
        vec![protocol::CounterId::LiveBlockErrors],
        1,
    );
    let (result, response) = rig.send(&configure);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    assert!(
        matches!(
            ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("configure response"),
            protocol::DecodedTypedResponseFrame::Success { .. }
        ),
        "the configuration naming LIVE_BLOCK_ERRORS is accepted, not NOT_FOUND"
    );
    rig.step();
    let [left, right] = source_chunk(rig.fed, rig.quantum);
    submit_c(
        rig.session,
        rig.generation,
        rig.fed,
        rig.rate,
        &left,
        &right,
        false,
    );
    let snapshots = lossy_counter_snapshots(rig.session);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(
        snapshots[0].values,
        vec![protocol::CounterValue {
            id: protocol::CounterId::LiveBlockErrors,
            value: 3,
        }]
    );
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

/// Every word of the plan's watermark: `(revision, first sample, flags, exact, fallback,
/// superseded)`.
fn watermark_words(rig: &Rig) -> (u64, u64, u64, u64, u64, u64) {
    let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
    assert_eq!(result, crate::RESULT_OK);
    (
        watermark.revision,
        watermark.first_sample,
        watermark.outcome_flags,
        watermark.exact_count,
        watermark.transition_fallback_count,
        watermark.superseded_count,
    )
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
    assert_eq!(rig.apply(&edit), crate::RESULT_OK);
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
    let live = fader_edit(
        "eq0",
        DualMonoFader {
            left_db: -6.0,
            right_db: -6.0,
            ..fader
        },
    );
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
    assert_rebuilds_ducking(document, edits, &[], label);
}

/// [`assert_rebuilds`] for a rebuild that restarts the strips `ducked`: a duck-swap, whose phase 1
/// writes exactly one ramped mute to each channel of each of them on the rendering plan (#1324a,
/// #1325 D2) and nothing else to any cell.
fn assert_rebuilds_ducking(document: &str, edits: &[SessionEdit], ducked: &[&str], label: &str) {
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
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let cells: Vec<(&str, [u64; 4], u64)> = snapshot
        .strip_cells
        .iter()
        .filter(|(epoch, ..)| *epoch == snapshot.provider_epoch)
        .map(|(_, id, fader, matrix)| (&**id, *fader, *matrix))
        .collect();
    assert!(!cells.is_empty(), "{label}");
    for (id, fader, matrix) in cells {
        let expected = if ducked.contains(&id) {
            [0, 0, 1, 1]
        } else {
            [0; 4]
        };
        assert_eq!(
            (fader, matrix),
            (expected, 0),
            "{label}: the rendering plan got no write but the duck's, on {id}"
        );
    }
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
        assert_eq!(rig.apply(&edit), crate::RESULT_OK, "{label}");
        assert_eq!(rig.summary().3, 0, "{label} is live");
    }
    let unfollowed = submix_session(false);

    // A submix strip's fader is live since #1554; `set_control::submix_strips_take_live_edits`.
    assert_rebuilds(
        &unfollowed,
        &[
            fader_edit("eq0", quiet.clone()),
            vec![content_edit(
                &parse_session_json(&unfollowed).expect("model"),
            )],
        ]
        .concat(),
        "a live fader beside a source content edit",
    );

    let model = parse_session_json(&base).expect("model");
    assert_rebuilds(&base, &[content_edit(&model)], "a source content edit");

    // A fader outside the control's domain is refused at its own edit by the control validator
    // (`control.domain`, a protocol refusal under `RESULT_OK`), and writes nothing.
    let mut rig = Rig::new(&base);
    rig.step();
    let before = refusal_state(&rig);
    assert_eq!(
        set_control::refusal(&mut rig, &[left_db_edit("eq0", 30.0)]),
        (
            protocol::StatusCode::ValidationFailed,
            vec![("control.domain".to_owned(), 0)]
        )
    );
    // The revision, the snapshot and the event lane are as they were; the refusal itself is the one
    // answer the replay cache keeps.
    let after = refusal_state(&rig);
    assert_eq!(
        (after.0, &after.1, after.3),
        (before.0, &before.1, before.3)
    );
    assert_eq!(after.2, before.2 + 1);
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
    let mut refused = vec![left_db_edit("eq0", -9.0)];
    refused.extend(matrix_edit(
        &StableId::parse("eq3").expect("eq3"),
        MatrixOrPan::Pan {
            left: -0.75,
            right: 0.25,
            smoothing_samples: 0,
        },
    ));
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
    let mut retargeted = vec![structural[0].clone(), structural[1].clone()];
    retargeted.extend(fader_edit(
        "eq0",
        DualMonoFader {
            left_db: -6.0,
            right_db: -6.0,
            ..fader
        },
    ));
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

    let mut edits = vec![left_db_edit("eq1", -3.0)];
    edits.extend(matrix_edit(
        &StableId::parse("eq2").expect("eq2"),
        MatrixOrPan::Pan {
            left: 0.5,
            right: 0.25,
            smoothing_samples: 16,
        },
    ));
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
            _ => "other",
        });
    }
}

/// #1053 D10: a live edit emits exactly the events a rebuild's commit does -- one
/// `SESSION_COMMITTED` and nothing else: `AUTOMATION_CANCELED` is retired with `AUTOMATION_ENQUEUE`
/// (#1627, #1628), so no commit cancels anything.
///
/// Test value: red if a live edit commits through a path that skips the protocol commit's events,
/// for example a live-only commit that emits no `SESSION_COMMITTED`.
#[test]
fn a_live_edit_emits_the_commit_events_of_a_rebuild() {
    let mut rig = Rig::new(&long_session(10, 48_000));
    rig.step();
    let bytes = rig.transaction(&[left_db_edit("eq0", -1.0)]);
    assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"]);

    let bytes = rig.transaction(&[left_db_edit("eq0", -2.0)]);
    assert_eq!(command_c(rig.session, &bytes).0, crate::RESULT_OK);
    assert_eq!(rig.summary().3, 0, "the edit is live");
    assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"]);
}

/// The model-only edits: #1260 D1's four and #1365's `control_smoothing`, each a change no
/// prepared plan reads. The `control_smoothing` edit is model-only only on a session without stored
/// automation that reads its lengths (#1639 D3), so it comes before the automation upsert.
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
            "control smoothing",
            // On a session with a fader entry the edit writes the plan's jump-length cell and is
            // `live` (#1639 D3); the EQ entry below reads no length, so the session has none.
            // The first write turns the `default` arm into the explicit lengths (#1561 D4a).
            smoothing_edits(ControlSmoothing {
                mute_ms: 1.0,
                fader_ms: 2.0,
                pan_ms: 3.0,
            })
            .remove(0),
        ),
        (
            // An entry on the console EQ's band-1 gain: a valid block-rate target whose row does
            // not render until #1667, the last row to leave the mask. A fader entry is a rebuild
            // (#1648 D1).
            "automation upsert",
            SessionEdit::UpsertAutomation {
                automation: session::Automation {
                    id: id("eq-ride"),
                    target: session::ControlAddress {
                        owner: session::Owner::Strip(model.tracks[0].id.clone()),
                        section: session::Section::Console(id("eq")),
                        control: 4,
                        lane: session::ParameterChannel::Left,
                    },
                    segments: vec![session::AutomationSegment {
                        shape: session::AutomationShape::Linear,
                        start_sample: 0,
                        end_sample: 960,
                        start_value: 0.0,
                        end_value: -3.0,
                        unit: session::ParameterUnit::Db,
                    }],
                    offset: session::AutomationOffset::default(),
                },
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
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK, "{name}");
        assert_eq!(
            edit_path_of(&response),
            protocol::EditPath::ModelOnly,
            "{name}: a delta with no record is reported as model-only"
        );
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

/// The edit path a `SESSION_TRANSACTION_APPLY` success response carries (#1313).
fn edit_path_of(response: &[u8]) -> protocol::EditPath {
    applied_of(response).edit_path
}

/// The `TransactionApplied` fields of a transaction's success response.
fn applied_of(response: &[u8]) -> protocol::TransactionApplied {
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("transaction response")
    else {
        panic!("expected a transaction success")
    };
    applied
}

/// #1313 gates 1 and 2: the response names how the engine applied the transaction. A fader change
/// is `live`, a session ID change is `model_only`, an added track is `rebuild`, and a fader change
/// beside a source content edit is the `rebuild` it takes. An exact replay of the live
/// request returns the same bytes, path included.
///
/// Test value: red if the path is derived from anything but the classifier's result (for example
/// "a record was pushed", which calls the mixed transaction live; or `live` for an empty delta),
/// or if the commit forgets to overwrite the replay entry, so a retry reports the placeholder
/// the prepared frame carried.
#[test]
fn each_transactions_response_names_its_edit_path() {
    let document = effect_session(10, 48_000);

    let mut rig = Rig::new(&document);
    rig.step();
    let live = rig.transaction(&[left_db_edit("eq2", -4.5)]);
    let (result, first) = rig.send(&live);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&first), protocol::EditPath::Live);
    assert_eq!(rig.summary().3, 0, "live prepares no candidate");
    let (result, replay) = rig.send(&live);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        replay, first,
        "the replay is the first response, byte for byte"
    );
    assert_eq!(edit_path_of(&replay), protocol::EditPath::Live);

    let session_id = rig.transaction(&[SessionEdit::SetSessionId {
        session_id: StableId::parse("path-session").expect("session ID"),
    }]);
    let (result, response) = rig.send(&session_id);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&response), protocol::EditPath::ModelOnly);
    assert_eq!(rig.summary().3, 0, "model-only prepares no candidate");

    let mut rig = Rig::new(&document);
    rig.step();
    let model = parse_session_json(&document).expect("model");
    let added = rig.transaction(&add_muted_track(&model, "added"));
    let (result, response) = rig.send(&added);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
    assert_eq!(rig.summary().3, 1, "rebuild prepares a candidate");

    let mut rig = Rig::new(&document);
    rig.step();
    // No native effect prepares at a quality other than `normal` today, so a quality edit would be
    // refused at preparation; a source content edit is the structure change that beside a fader
    // value is still the rebuild it takes (`LiveRebuild::Structure`).
    let mixed = rig.transaction(&[left_db_edit("eq2", -4.5), content_edit(&model)]);
    let (result, response) = rig.send(&mixed);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        edit_path_of(&response),
        protocol::EditPath::Rebuild,
        "a transaction that also has live records is the rebuild it takes"
    );
    assert_eq!(rig.summary().3, 1);
}

/// A `SET_CONTROL` of the session's `mute_ms` row: the given `mute_ms` and the default table's
/// other two lengths (the first write makes the lengths explicit, #1561 D4a).
fn mute_smoothing_edit(mute_ms: f32) -> SessionEdit {
    smoothing_edits(ControlSmoothing {
        mute_ms,
        fader_ms: 20.0,
        pan_ms: 20.0,
    })
    .remove(1)
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
                link_mode: session::ConsoleLinkMode::Slot,
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
    _unit: session::ParameterUnit,
    value: f32,
) -> SessionEdit {
    let effect = StableId::parse(effect_id).expect("effect ID");
    let section = match rack_name {
        session::RackName::Console => protocol::control_core::Section::Console(effect),
        _ => protocol::control_core::Section::Inserts(effect),
    };
    // The unit is the effect descriptor's (decision 17): a `SET_CONTROL` names only the value.
    SessionEdit::set_control(
        protocol::control_core::ControlAddress {
            owner: protocol::control_core::Owner::Strip(
                StableId::parse(track_id).expect("track ID"),
            ),
            section,
            control: parameter_id,
            lane: channel,
        },
        value,
        None,
    )
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
/// than the direct host-core lane (the records the browser wrote before #1565A deleted its own
/// command path; the test names keep the old word): hand-built records pushed into a host-core plan with every live lane
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
    let children = compile_children(&document, limits()).expect("EQ session");
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

    /// Runs one EQ owner transaction as the direct host-core lane does: the edits, targets designed in
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
/// the direct host-core lane: targets designed in the test by `EqTargetPreparer` from the owner's own
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
        // Band 1's gain goes back to its default (0 dB), the value a removed entry reads as.
        upsert(
            last,
            RackName::Inserts,
            "eq-insert",
            4,
            ParameterChannel::Both,
            ParameterUnit::Db,
            0.0,
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

// Issue #1266: value-only effect bypass edits through the latency-preserving shunt.

fn bypass_edit(
    track_id: &str,
    rack_name: session::RackName,
    effect_id: &str,
    bypass: bool,
) -> SessionEdit {
    let effect = StableId::parse(effect_id).expect("effect ID");
    let section = match rack_name {
        session::RackName::Console => protocol::control_core::Section::Console(effect),
        _ => protocol::control_core::Section::Inserts(effect),
    };
    SessionEdit::set_control(
        protocol::control_core::ControlAddress {
            owner: protocol::control_core::Owner::Strip(
                StableId::parse(track_id).expect("track ID"),
            ),
            section,
            control: protocol::control_core::BYPASS_CONTROL,
            lane: protocol::control_core::Lane::Both,
        },
        f32::from(u8::from(bypass)),
        None,
    )
}

/// A loud noise source, different on the two lanes, that the strip's input high-pass leaves alone
/// (`source_sample` is nearly DC, which it removes): a hash of the frame, so every target and every
/// call gives the same words.
fn noise_chunk(start: u64, frames: usize) -> [Vec<f32>; 2] {
    [(0_u64, 0.9_f32), (1, -0.8)].map(|(lane, gain)| {
        (0..frames as u64)
            .map(|offset| {
                let mut x = (start + offset).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (lane << 59);
                x ^= x >> 29;
                x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
                x ^= x >> 32;
                gain * ((x >> 40) as f32 / (1_u64 << 23) as f32 - 1.0)
            })
            .collect()
    })
}

/// The one-track session with one native effect insert and nothing after it that is not unity, so
/// the effect's output reaches the master untouched (a lone insert is a remainder, which renders
/// per node), and the same session with the insert baked bypassed: the control a live bypass must
/// land on (#1341 gate 5).
fn bypass_crossfade_documents(
    effect_id: &str,
    control_smoothing: Option<ControlSmoothing>,
    console: bool,
) -> (String, String) {
    let donor = parse_session_json(&long_session(1, 48_000)).expect("session");
    let mut effect = donor.lower_track(&donor.tracks[0]).pre_insert[0].clone();
    effect.id = StableId::parse("fx").expect("effect ID");
    effect.identity = session::EffectIdentity::Native {
        effect_id: StableId::parse(effect_id).expect("effect"),
    };
    effect.params.clear();
    effect.bypass = false;
    let mut model = super::tests::stateless_session(48_000);
    model.sources[0].frames = SOURCE_FRAMES;
    model.control_smoothing = control_smoothing;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let builtins = &mut model.tracks[0].builtins;
    builtins.left.lpf_hz = 20_000.0;
    builtins.right.lpf_hz = 20_000.0;
    if console {
        // A console slot always banks (a lone lane in a padded bank), where the insert is a
        // remainder that renders per node: the same crossfade must come out of both (#1341a2).
        model.tracks[0].console.push(session::ConsoleEntry {
            slot: effect.id.clone(),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Slot,
            params: Vec::new(),
        });
        model.console.post_insert.push(session::ConsoleSlot {
            slot: effect.id.clone(),
            identity: effect.identity.clone(),
            quality: effect.quality,
            link_mode: effect.link_mode,
        });
    } else {
        model.tracks[0].inserts.effects.push(effect);
    }
    let live = session::canonical_session_json(&model).expect("canonical");
    if console {
        model.tracks[0].console[0].bypass = true;
    } else {
        model.tracks[0].inserts.effects[0].bypass = true;
    }
    let bypassed = session::canonical_session_json(&model).expect("canonical bypassed");
    (live, bypassed)
}

/// Gate 5 of #1341. A bypass transaction on a playing session crossfades the per-node effect over
/// the session's `mute_ms` ramp (480 frames at 48 kHz, so the ramp ends inside a block) and lands
/// on a control booted bypassed. The effect sits once as an insert (a lone track's remainder,
/// rendered per node) and once as a console slot (always banked): both render the one law. Frame
/// `k` of the ramp (counted from the first frame of the block that drains the commit, from 1) is
/// `D + (W - D) * m(k)` with `m` the indexed ramp law from `1` to `0`, `W` the output of a control
/// that is never bypassed and `D` that of a control booted bypassed; frames before the commit equal
/// the booted state and frames from the ramp's end equal `D`, bit for bit.
///
/// A lift resumes the effect from rest (#2129, owner decision "un-bypass from rest"): its effect is
/// reset at the commit and holds `D` for the effect's warm-up (60 frames for the soft clip, none
/// for the others) before its crossfade, so no never-bypassed control is its target. The lift rows
/// hold `D` through the warm-up, and the insert and the console slot render the same lifted words,
/// bit for bit (the crossfade law itself is pinned at the effect's output by `graph-compiler`'s
/// `bypass_skip_warmup`).
/// The ramp blocks render without one allocation (statics warmed). A zero-latency effect (the
/// compressor) and one with latency (the soft clip, 31 samples) run, and the delay and the
/// multiband compressor, whose bypasses are live since #1339 and #1340 (inserts only, as neither
/// is console-eligible); a session
/// whose `mute_ms` is 0 steps at the commit's first block.
///
/// Test value: red if the C ABI producer forgets the session ramp (the toggle steps, so the first
/// block is already D), if the record's ramp is another row's (the fader's or pan's length), if a
/// ramp block allocates, if a lift leaves `D` before its warm-up ends, if a lift is acked and its
/// output stays `D` past its warm-up and ramp (every effect row; the delay's and the multiband's
/// red if their bypass is still prepared, #1339 gate 4, #1340 gate 5), if the lift is refused or
/// rebuilds a plan, or if the per-node and the banked lift disagree.
#[test]
fn a_live_bypass_crossfades_over_the_session_mute_ramp_and_lands_on_the_bypassed_control() {
    bench_support::alloc::assert_installed();
    for effect_id in [
        "miso.compressor",
        "miso.soft-clip",
        "miso.delay",
        "miso.multiband-compressor",
    ] {
        for hard in [false, true] {
            for lift in [false, true] {
                // The delay and the multiband are not console-eligible: inserts only.
                let consoles: &[bool] =
                    if matches!(effect_id, "miso.delay" | "miso.multiband-compressor") {
                        &[false]
                    } else {
                        &[false, true]
                    };
                // #2129: the lifted words of each placement, compared after the loop.
                let mut lifted: Vec<Vec<Vec<u32>>> = Vec::new();
                for &console in consoles {
                    let mut lifted_rows = Vec::new();
                    let (document, bypassed_document) = bypass_crossfade_documents(
                        effect_id,
                        hard.then_some(HARD_SWITCH).flatten(),
                        console,
                    );
                    let model = parse_session_json(&document).expect("model");
                    let ramp = model.control_smoothing_samples().mute;
                    assert_eq!(
                        ramp,
                        if hard { 0 } else { 480 },
                        "10 ms at 48 kHz, or the hard switch"
                    );
                    let first = model.tracks[0].id.as_str();
                    let rack = if console {
                        session::RackName::Console
                    } else {
                        session::RackName::Inserts
                    };
                    let mut rig = Rig::new(if lift { &bypassed_document } else { &document });
                    let quantum = rig.quantum;
                    let record_block = 6_u64;
                    let blocks = record_block + 14;
                    let run = |document: &str| {
                        let mut reference = Reference::new(document);
                        (0..blocks)
                            .map(|block| {
                                reference.step_planes(noise_chunk(block * quantum as u64, quantum))
                            })
                            .collect::<Vec<_>>()
                    };
                    let wet = run(&document);
                    let dry = run(&bypassed_document);
                    // What the session renders before the commit, and after the ramp.
                    let (booted, landed) = if lift { (&dry, &wet) } else { (&wet, &dry) };
                    let (from, to) = if lift { (0.0, 1.0) } else { (1.0, 0.0) };
                    assert!(
                        wet.iter()
                            .zip(&dry)
                            .skip(record_block as usize)
                            .any(|(w, d)| bits(w) != bits(d)),
                        "{effect_id}: the bypassed control differs from the wet one"
                    );
                    for block in 0..record_block {
                        let planes = noise_chunk(rig.fed, quantum);
                        assert_eq!(
                            bits(&rig.step_planes_counted(planes).0),
                            bits(&booted[block as usize]),
                            "warm-up {block}"
                        );
                    }
                    let (revision, _, epoch, _) = rig.summary();
                    assert_eq!(
                        rig.apply(&[bypass_edit(first, rack, "fx", !lift)]),
                        crate::RESULT_OK
                    );
                    assert_eq!(
                        (rig.summary().0, rig.summary().2, rig.summary().3),
                        (revision + 1, epoch, 0),
                        "one live revision and no plan prepared"
                    );
                    let mut ramping = 0_usize;
                    let mut between = 0_usize;
                    // A lift's words past its warm-up and ramp that are not the dry control's.
                    let mut lifted_off_dry = 0_usize;
                    for block in record_block..blocks {
                        let planes = noise_chunk(rig.fed, quantum);
                        let (live, used) = rig.step_planes_counted(planes);
                        assert_eq!(
                            (used.allocations, used.deallocations),
                            (0, 0),
                            "{effect_id}: block {block} allocated or freed"
                        );
                        if lift {
                            // The effect's warm-up: the soft clip's latency plus its tail.
                            let warmup = if effect_id == "miso.soft-clip" { 60 } else { 0 };
                            for (index, sample) in live.iter().enumerate().skip(quantum) {
                                let frame = (block - record_block) * quantum as u64
                                    + (index % quantum) as u64;
                                if frame < warmup {
                                    assert_eq!(
                                        sample.to_bits(),
                                        dry[block as usize][index].to_bits(),
                                        "{effect_id}: block {block} sample {index} holds dry"
                                    );
                                } else if frame >= warmup + u64::from(ramp) {
                                    lifted_off_dry += usize::from(
                                        sample.to_bits() != dry[block as usize][index].to_bits(),
                                    );
                                }
                            }
                            lifted_rows.push(bits(&live));
                            continue;
                        }
                        let peak = [&wet, &dry]
                            .iter()
                            .flat_map(|control| control[block as usize][quantum..].iter())
                            .fold(0.0_f32, |peak, sample| peak.max(sample.abs()));
                        for (index, sample) in live.iter().enumerate().skip(quantum) {
                            // The fixture's pan sends both lanes to the right output plane; the left plane
                            // holds only the pan's leakage, so the law is read on the right one.
                            let frame = index % quantum;
                            let (w, d) = (wet[block as usize][index], dry[block as usize][index]);
                            let k = (block - record_block) * quantum as u64 + frame as u64 + 1;
                            let what = format!(
                                "{effect_id}, mute_ms {}, {}, {}: block {block} sample {index}",
                                if hard { 0 } else { 10 },
                                if console { "console slot" } else { "insert" },
                                if lift { "lift" } else { "bypass" }
                            );
                            if k >= u64::from(ramp) {
                                assert_eq!(
                                    sample.to_bits(),
                                    landed[block as usize][index].to_bits(),
                                    "{what}: settled on the control"
                                );
                                continue;
                            }
                            ramping += 1;
                            let m = lane::kernels::IndexedRamp::new(
                                [from, 0.0, 0.0, 0.0],
                                [to, 0.0, 0.0, 0.0],
                                ramp,
                            )
                            .coefficients_at(k as u32)[0];
                            // The strip's own gain stages follow the effect and round (the pan sums the
                            // two lanes), so the law holds to a few ulps of the block's peak here
                            // (`bypass_shunt_identity` pins it exactly at the effect's output). An
                            // off-by-one frame moves a sample by `(W - D) / 480`, over twenty times this
                            // bound wherever the two controls differ audibly.
                            let tolerance = 64.0 * f32::EPSILON * peak;
                            assert!(
                                (sample - (d + (w - d) * m)).abs() <= tolerance,
                                "{what}: {sample:e} is not D + (W - D) * m with D {d:e}, W {w:e}, m {m}"
                            );
                            between += usize::from(
                                (sample - w).abs() > tolerance && (sample - d).abs() > tolerance,
                            );
                        }
                    }
                    if lift {
                        assert!(
                            lifted_off_dry > 0,
                            "{effect_id}: the lift was acked but its output stayed dry"
                        );
                        lifted.push(lifted_rows);
                        continue;
                    }
                    assert_eq!(
                        ramping,
                        (ramp as usize).saturating_sub(1),
                        "{effect_id}: ramp frames"
                    );
                    assert!(
                        between >= ramping / 2,
                        "{effect_id}: the ramp is audibly between the wet and the dry control"
                    );
                }
                if let [insert, console] = lifted.as_slice() {
                    assert_eq!(
                        insert, console,
                        "{effect_id}: the per-node and banked lifts"
                    );
                }
            }
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
    let descriptor = compile_children(&document, limits())
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
        // The claim is the lane's drain order, and the reference lane is pushed bare `Bypass`
        // records, which step, so the session authors the hard switch (#1341 hazards).
        let document = with_model(&effect_session(10, sample_rate_hz), |model| {
            model.control_smoothing = HARD_SWITCH;
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
            reference.write(
                first,
                "comp",
                Bypass(effect_contract::BypassSwitch::step(false)),
            );
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
    // The claim is the drain order beside the owner's transaction, and the reference lane is
    // pushed a bare `Bypass` record, which steps, so the session authors the hard switch (#1341
    // hazards). The console EQ banks, and a bank crossfades as a node does (#1341a2).
    let document = with_model(&eq_session(sample_rate_hz), |model| {
        model.control_smoothing = HARD_SWITCH;
    });
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
        effect_contract::EffectControlRecord::Bypass(effect_contract::BypassSwitch::step(true)),
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

// Issue #1335: stored automation on an effect parameter the plan keeps prepared.

/// An `UpsertAutomation` of one step on `eq_session`'s `eq0` console EQ, on the row `parameter_id`
/// at `channel`.
fn eq_automation_edit(
    parameter_id: u32,
    channel: session::ParameterChannel,
    unit: session::ParameterUnit,
) -> SessionEdit {
    strip_automation_edit(
        session::Section::Console(StableId::parse("eq").expect("stable ID")),
        parameter_id,
        channel,
        unit,
    )
}

/// `eq_session` with a true-peak limiter insert (`limiter`) on track `eq0`.
fn limiter_session(sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&eq_session(sample_rate_hz)).expect("EQ session");
    let mut limiter = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    limiter.id = StableId::parse("limiter").expect("limiter insert");
    limiter.identity = session::EffectIdentity::Native {
        effect_id: StableId::parse("miso.true-peak-limiter").expect("limiter ID"),
    };
    limiter.bypass = false;
    limiter.params = Vec::new();
    model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq0")
        .expect("eq0")
        .inserts
        .effects
        .push(limiter);
    session::canonical_session_json(&model).expect("canonical limiter session")
}

/// An `UpsertAutomation` of one step on track `eq0`'s `section`, on the row `parameter_id` at
/// `channel`.
fn strip_automation_edit(
    section: session::Section,
    parameter_id: u32,
    channel: session::ParameterChannel,
    unit: session::ParameterUnit,
) -> SessionEdit {
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("eq-ride"),
            target: session::ControlAddress {
                owner: session::Owner::Strip(id("eq0")),
                section,
                control: parameter_id,
                lane: channel,
            },
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Step,
                start_sample: 0,
                end_sample: 960,
                start_value: 1.0,
                end_value: 1.0,
                unit,
            }],
            offset: session::AutomationOffset::default(),
        },
    }
}

/// #1335 gate 3. On a playing engine, a transaction that only adds an automation on a true-peak
/// limiter insert's `lookahead` (`automation_rate` `None`; the console EQ's `band-1-enabled` was
/// the subject until #1337 made it live) is refused with the compile rejection a
/// rebuild's preparation gives (`control.automation.target`): the revision, the committed model and
/// the reliable lane do not move, no candidate is prepared, and the following blocks are
/// bit-identical to an engine that never received it. The same transaction on the block-rate
/// `band-1-gain` commits live, as #1260 D2 made it.
///
/// Red mutation: drop the classifier's automation check -> the edit commits live with no records:
/// acked, and never heard (decision 14 F1).
#[test]
fn an_automation_on_a_prepared_effect_parameter_is_refused_before_any_ack() {
    use session::{ParameterChannel, ParameterUnit};
    let document = limiter_session(48_000);
    let mut rig = Rig::new(&document);
    let mut untouched = Rig::new(&document);
    for _ in 0..window(rig.latency(), 0, rig.quantum) {
        assert_eq!(bits(&rig.step()), bits(&untouched.step()), "warm-up");
    }

    let refused = strip_automation_edit(
        session::Section::Inserts(StableId::parse("limiter").expect("limiter")),
        3,
        ParameterChannel::Both,
        ParameterUnit::Milliseconds,
    );
    let before = (refusal_state(&rig), rig.summary());
    assert_eq!(
        rig.apply(core::slice::from_ref(&refused)),
        crate::RESULT_COMPILE_REJECTED
    );
    let mut rejected = parse_session_json(&document).expect("model");
    protocol::apply_session_edit(&mut rejected, &refused).expect("the edit applies to the model");
    let rejected = session::canonical_session_json(&rejected).expect("canonical rejected");
    let Err(failure) = compile_children(&rejected, limits()) else {
        panic!("an automation on the lookahead does not prepare")
    };
    assert_eq!(
        rig.last_error(),
        failure.diagnostics,
        "the refusal is preparation's diagnostic"
    );
    assert!(
        String::from_utf8_lossy(&failure.diagnostics).contains("control.automation.target"),
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

// Issue #1645: the builtin rows' shape and filter-order rules on the C ABI's live path.

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

    // A bypass crossfades over the session's `mute_ms` and takes no length of its own (#1341): the
    // row lists that ramp as its default and names no ramp row.
    let bypass = whole
        .iter()
        .filter(|entry| entry.address.section == protocol::ControlSection::Inserts)
        .find_map(|entry| control_of(entry).filter(|row| row.name == "bypass"))
        .expect("an insert's bypass row");
    assert_eq!(
        (bypass.ramp_row, bypass.default_ramp_samples),
        (None, model.control_smoothing_samples().mute)
    );

    // The state poll reports the same values for the same handles.
    let state = control_state_all_c(rig.session, &mut rig.request, 256);
    let expected = whole
        .iter()
        .filter_map(|entry| control_of(entry).map(|row| (entry.handle, row.value.to_bits())))
        .collect::<Vec<_>>();
    assert_eq!(state, expected);
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
        rig.apply(&matrix_edit(
            &model.tracks[0].id,
            MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples,
            },
        )),
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
        headers_only: false,
    };
    let request = rig.next_request();
    assert_eq!(
        control_list_c(rig.session, request, &stale)
            .err()
            .map(|(status, _)| status),
        Some(protocol::StatusCode::RevisionConflict)
    );
}

// Issue #1600: a live commit refreshes only the changed owners' catalog rows.

/// The `SET_CONTROL` edit that writes `value` to the control that `entry` lists.
fn set_listed(entry: &protocol::ControlEntry, value: f32) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: protocol::control_core::ControlAddress::try_from(&entry.address)
            .expect("a listed address"),
        value,
        ramp_samples: None,
    }
}

/// The first control row of `list` that `keep` accepts.
fn listed(
    list: &[protocol::ControlEntry],
    keep: impl Fn(&protocol::ControlEntry, &protocol::ControlRow) -> bool,
) -> &protocol::ControlEntry {
    list.iter()
        .find(|entry| control_of(entry).is_some_and(|row| keep(entry, row)))
        .expect("a listed control")
}

/// #1600 gate 5. A live commit whose catalog stage fails is refused before its first write: the
/// session, its replay log and its list are what they were, and the same edit commits once the
/// fault is gone.
///
/// Red if the stage moves after the first cell write, or an acknowledged commit can drop it.
#[test]
fn a_failed_catalog_stage_commits_nothing() {
    let document = control_list_session();
    let mut rig = Rig::new(&document);
    rig.step();
    let (list, ..) = control_list_all_c(rig.session, &mut rig.request, 256);
    let edit = set_listed(
        listed(&list, |entry, row| {
            entry.address.owner_id.as_deref() == Some("eq0")
                && entry.address.section == protocol::ControlSection::Builtins
                && row.name == "fader_db"
                && entry.address.lane == Some(protocol::ParameterChannel::Left)
        }),
        -6.0,
    );
    let state = refusal_state(&rig);
    let writes = rig.writes(Epoch::Current, "eq0");
    host_core::fail_next_stages(1);
    assert_ne!(rig.apply(std::slice::from_ref(&edit)), crate::RESULT_OK);
    assert_eq!(refusal_state(&rig), state);
    assert_eq!(
        rig.writes(Epoch::Current, "eq0"),
        writes,
        "no cell was written"
    );
    let (same, ..) = control_list_all_c(rig.session, &mut rig.request, 256);
    assert_eq!(same, list);

    assert_eq!(rig.apply(&[edit]), crate::RESULT_OK);
    assert_ne!(rig.writes(Epoch::Current, "eq0"), writes);
}

// Issue #1553: `SET_CONTROL`, any control by its stable address.

mod set_control {
    use super::*;
    use protocol::control_core::{ControlAddress, Lane, Owner, Section};

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
            expected_effect: None,
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

    /// Gate 2: a value the classifier leaves to a rebuild is a rebuild through `SET_CONTROL`.
    ///
    /// Red if a rebuild-only value is acked as live (a candidate not prepared, a cell written).
    #[test]
    fn a_set_control_on_a_rebuild_value_is_a_rebuild() {
        let document = long_session(10, 48_000);
        let model = parse_session_json(&document).expect("model");
        let track = model.tracks[0].clone();
        let name = track.id.as_str();
        // A changed `delay_samples` restarts the strip: a duck-swap (#1324a).
        assert_rebuilds_ducking(
            &document,
            &[set(builtin(name, DELAY_SAMPLES, Lane::Both), 64.0, None)],
            &[name],
            "delay by address",
        );
        // The whole builtin section with the same delay is the same rebuild.
        let mut builtins = track.builtins.clone();
        builtins.left.delay_samples = 64;
        builtins.right.delay_samples = 64;
        assert_rebuilds_ducking(
            &document,
            &protocol::control_core::builtins_edits(&track.id, &builtins),
            &[name],
            "delay in the whole builtin section",
        );
        let route_id = model.routes[0].id.clone();
        assert_rebuilds(
            &document,
            &[set(
                address(Owner::Route(route_id), Section::Own, 1, Lane::Both),
                -6.0,
                None,
            )],
            "send gain by address",
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
    pub(super) fn refusal(
        rig: &mut Rig,
        edits: &[SessionEdit],
    ) -> (protocol::StatusCode, Vec<(String, u32)>) {
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
    /// changes, and a good edit beside it does not commit. Every `ControlError` code but
    /// `control.monitor` (a solo is a monitor write now, #1559) is produced once through the frame
    /// path, and a ramp the row cannot honour or above the rate is refused.
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

    /// Gate 1, effect family on a submix strip: a console slot's parameter, an insert's
    /// parameter and an insert's bypass commit live and render the bits of the direct host-core lane
    /// (host-core records pushed to the bus's own producers at the same block).
    ///
    /// Red if a submix effect is left a rebuild, or its record reaches another strip's instance
    /// of the same name (every track carries `comp` and `clip` too).
    #[test]
    fn submix_effects_take_live_edits() {
        use effect_contract::ParameterChannel::{Left, Right};
        let document = with_bus_strip(&effect_session(10, 48_000), |model, submix| {
            // The reference lane is pushed bare `Bypass` records, which step, so the session
            // authors the hard switch (#1341 hazards).
            model.control_smoothing = HARD_SWITCH;
            let template = model.tracks[0].inserts.effects[0].clone();
            let mut compressor = template;
            compressor.id = id("bcomp");
            compressor.bypass = false;
            submix.inserts.effects.push(compressor);
            for entry in &mut submix.console {
                entry.bypass = false;
            }
        });
        // A case: its label, its edits, the records the direct host-core lane gets for them, and whether it
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
                    effect_contract::EffectControlRecord::Bypass(
                        effect_contract::BypassSwitch::step(true),
                    ),
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
            let bytes = rig.transaction(&edits);
            let (result, response) = rig.send(&bytes);
            assert_eq!(result, crate::RESULT_OK, "{label}");
            // #1313 D2: a delta with a record is `live`; one that moves no effective value has
            // none, and is `model_only`.
            assert_eq!(
                edit_path_of(&response),
                if audible {
                    protocol::EditPath::Live
                } else {
                    protocol::EditPath::ModelOnly
                },
                "{label}: the response names the path the classifier took"
            );
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
        let Err(failure) = compile_children(&over, capped) else {
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

    /// Gate 7 (#1558 D6, batch 3 fold): VCAs added live replace the control catalog by a larger one,
    /// and the live admission charges the growth. With the capi ceiling set to the smallest one
    /// that admits a live edit that adds no catalog bytes, plus the growth less one byte, the add
    /// is refused as a rebuild would, and the model, the revision and every cell are as they were;
    /// at the growth it commits, and the plan reports the committed catalog's bytes, which is what
    /// a rebuild of the same session reports.
    ///
    /// Red if the live admission charges the catalog the plans were prepared with and not the one
    /// the commit publishes.
    #[test]
    fn a_vca_added_live_is_charged_for_the_catalog_it_adds() {
        let document = vca_document();
        let long = |tag: &str| format!("{tag}-{}", "v".repeat(48));
        let names = [long("one"), long("two"), long("three")];
        let grown = with_model(&document, |model| {
            for name in &names {
                model.vcas.push(vca_of(name, &["eq1"]));
            }
        });
        let adds: Vec<SessionEdit> = names
            .iter()
            .map(|name| SessionEdit::UpsertVca {
                vca: vca_of(name, &["eq1"]),
            })
            .collect();
        let same_size = [set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -3.0, None)];
        let catalog_bytes = |document: &str| {
            let model = parse_session_json(document).expect("model");
            host_core::ControlCatalog::build(
                &model,
                &protocol::control_core::MonitorOverlay::default(),
                model.sample_rate_hz,
            )
            .expect("catalog")
            .resources()
            .retained_bytes
        };
        // #1696: the added VCAs also raise the kept bases' edit-ramp row, which follows the
        // committed model as the catalog does.
        let ramp_bytes = |document: &str| {
            let model = parse_session_json(document).expect("model");
            control_plane::monitor_and_ramp_bytes(&model)
                .expect("the ramp row")
                .1
        };
        assert!(
            ramp_bytes(&grown) > ramp_bytes(&document),
            "the VCAs add ramp rows"
        );
        let growth = catalog_bytes(&grown) - catalog_bytes(&document) + ramp_bytes(&grown)
            - ramp_bytes(&document);
        assert!(growth > 0, "the added VCAs add catalog bytes");

        // Whether the edits commit under `ceiling`; a refusal changes nothing.
        let commits = |ceiling: u64, edits: &[SessionEdit]| {
            let tight = CompileLimits {
                maximum_capi_retained_bytes: ceiling,
                ..limits()
            };
            let mut rig = Rig::with_limits(&document, tight);
            rig.step();
            let before = refusal_state(&rig);
            let cells = rig.current_cells();
            let result = rig.apply(edits);
            if result == crate::RESULT_OK {
                return true;
            }
            assert_eq!(result, crate::RESULT_COMPILE_REJECTED);
            assert_eq!(refusal_state(&rig), before, "a refusal writes nothing");
            assert_eq!(rig.current_cells(), cells);
            assert_eq!(rig.summary().3, 0, "no candidate");
            false
        };
        // The smallest ceiling that admits an edit that leaves the catalog's size as it is.
        let active = crate::ffi::test_plan_snapshot(Rig::new(&document).plan)
            .1
            .capi_retained_bytes;
        let (mut low, mut high) = (active, limits().maximum_capi_retained_bytes);
        assert!(commits(high, &same_size));
        while low < high {
            let middle = low + (high - low) / 2;
            if commits(middle, &same_size) {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        let floor = low;
        assert!(floor > active, "the live peak holds the catalog twice");

        assert!(!commits(floor + growth - 1, &adds), "one byte short");
        assert!(commits(floor + growth, &adds), "at the growth");

        let tight = CompileLimits {
            maximum_capi_retained_bytes: floor + growth,
            ..limits()
        };
        let mut rig = Rig::with_limits(&document, tight);
        rig.step();
        let retained = |rig: &Rig| {
            crate::ffi::test_plan_snapshot(rig.plan)
                .1
                .capi_retained_bytes
        };
        let before = retained(&rig);
        assert_eq!(rig.apply(&adds), crate::RESULT_OK);
        assert_eq!(rig.summary().3, 0, "the adds are live");
        assert_eq!(retained(&rig), before + growth);
        let rebuilt = compile_children(&grown, tight)
            .unwrap_or_else(|_| panic!("the grown session compiles"))
            .plan
            .resources()
            .capi_retained_bytes;
        assert_eq!(retained(&rig), rebuilt, "what a rebuild reports");
        // The next live edit meets the committed catalog's bytes: the peak holds it twice, so at
        // this ceiling it is refused, and with the second copy's room it commits.
        assert_eq!(rig.apply(&same_size), crate::RESULT_COMPILE_REJECTED);
        let roomy = CompileLimits {
            maximum_capi_retained_bytes: floor + 2 * growth,
            ..limits()
        };
        let mut rig = Rig::with_limits(&document, roomy);
        rig.step();
        assert_eq!(rig.apply(&adds), crate::RESULT_OK);
        assert_eq!(rig.apply(&same_size), crate::RESULT_OK);
    }

    // ---- Solo (#1559): a monitor write through the core's overlay -----------------------------

    fn solo_edit(strip: &str, on: bool, ramp: Option<u32>) -> SessionEdit {
        set(own(strip, SOLO, Lane::Both), f32::from(u8::from(on)), ramp)
    }

    /// [`stateless_bus_document`] with `eq0`'s send into the bus pre-fader and following its mute,
    /// and a second send from `eq1` into the bus, post-fader and following. A solo on `eq1`
    /// mutes `eq0` and closes the send it feeds, while `eq1` and the bus stay audible.
    fn solo_document() -> String {
        with_model(&stateless_bus_document(), |model| {
            let mut other = model
                .routes
                .iter()
                .find(|route| route.id == id("eq0-bus"))
                .expect("the send")
                .clone();
            for route in model.routes.iter_mut().filter(|r| r.id == id("eq0-bus")) {
                route.follows_mute = true;
                if let RouteSource::Track { tap, .. } = &mut route.source {
                    *tap = session::SendTap::PreFader;
                }
            }
            other.id = id("eq1-bus");
            other.follows_mute = true;
            other.source = RouteSource::Track {
                track_id: id("eq1"),
                tap: session::SendTap::PostFader,
            };
            model.routes.push(other);
        })
    }

    /// `document` with every track but `soloed` muted: the committed session that a solo of
    /// `soloed` is equivalent to, once the mutes have ramped.
    fn muted_but(document: &str, soloed: &[&str]) -> String {
        with_model(document, |model| {
            for track in &mut model.tracks {
                if !soloed.contains(&track.id.as_str()) {
                    track.fader.left_mute = true;
                    track.fader.right_mute = true;
                }
            }
        })
    }

    /// The revision a success response frame carries.
    fn response_revision(response: &[u8]) -> u64 {
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::Success { header, .. } = ProtocolCodec::default()
            .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
            .expect("transaction response")
        else {
            panic!("expected a success")
        };
        header.revision.0
    }

    /// The value `CONTROL_STATE_GET` reports for strip `strip`'s solo row, and the structure epoch.
    fn solo_state(rig: &mut Rig, strip: &str) -> (f32, u64) {
        let (entries, epoch, _) = control_list_all_c(rig.session, &mut rig.request, 256);
        let handle = entries
            .iter()
            .find(|entry| {
                entry.address.owner_id.as_deref() == Some(strip)
                    && entry.address.section == protocol::ControlSection::Own
                    && control_of(entry).is_some_and(|row| row.name == "solo")
            })
            .expect("the solo row")
            .handle;
        let value = control_state_all_c(rig.session, &mut rig.request, 256)
            .into_iter()
            .find(|(candidate, _)| *candidate == handle)
            .map(|(_, bits)| f32::from_bits(bits))
            .expect("the solo row has a value");
        (value, epoch)
    }

    /// Gate 1, 6 and 7 (#1559): a solo is a monitor write. The reply is `monitor` at the committed
    /// revision, the plan and the snapshot stay, no `SESSION_COMMITTED` follows, an exact replay
    /// returns the same bytes and writes no cell, the list shows the solo, and from the block the
    /// mute ramp leaves behind the output is that of the plan prepared with every other track
    /// muted: the other tracks mute, the send they feed closes, and the bus (solo-safe) stays.
    /// An unsolo returns the unsoloed plan's bits. No render block allocates or frees.
    ///
    /// Red if the live solo composes differently from preparation, mutes a solo-safe submix,
    /// leaves a following send open, advances the revision or the epoch, or re-applies a replay.
    #[test]
    fn a_solo_is_a_monitor_write_that_renders_the_prepared_plan_bits() {
        bench_support::alloc::assert_installed();
        let document = solo_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let (revision, _, epoch, pending) = rig.summary();
        let snapshot = rig.snapshot();
        let (listed, structure_epoch) = solo_state(&mut rig, "eq1");
        assert_eq!(listed, 0.0);
        let cells = rig.current_cells();
        let routes = rig.current_route_cells();

        let bytes = rig.transaction(&[solo_edit("eq1", true, None)]);
        let (result, first) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&first), protocol::EditPath::Monitor);
        assert_eq!(response_revision(&first), revision, "no new revision");
        assert_eq!(
            (rig.summary().0, rig.summary().2, rig.summary().3),
            (revision, epoch, pending),
            "the model, the plan epoch and the candidate are as they were"
        );
        assert_eq!(
            reliable_event_kinds(&rig),
            Vec::<&str>::new(),
            "a monitor write is not a session change"
        );
        // An exact replay (before any newer request, which would expire it): the same bytes, and
        // no cell is written.
        let written = (rig.current_cells(), rig.current_route_cells());
        let (result, replay) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(replay, first, "the replay is the first response");
        assert_eq!((rig.current_cells(), rig.current_route_cells()), written);
        assert_eq!(reliable_event_kinds(&rig), Vec::<&str>::new());
        assert_eq!(rig.snapshot(), snapshot, "a solo is never saved");
        assert_eq!(solo_state(&mut rig, "eq1"), (1.0, structure_epoch));
        assert_eq!(solo_state(&mut rig, "eq0").0, 0.0);
        assert_ne!(
            rig.current_cells(),
            cells,
            "the other tracks' mutes were written"
        );
        assert_ne!(
            rig.current_route_cells(),
            routes,
            "the send's gate was written"
        );
        let soloed = muted_but(&document, &["eq1"]);
        let k = window(rig.latency(), ramp, rig.quantum);
        let edit_block = rig.block;
        let mut live = Vec::new();
        for _ in 0..k + 4 {
            let (pcm, used) = rig.step_counted();
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "a render block allocated or freed"
            );
            live.push(pcm);
        }
        let prepared = Reference::run(&soloed, rig.block);
        let unsoloed = Reference::run(&document, rig.block);
        let mut heard = false;
        for (offset, pcm) in live.iter().enumerate().skip(k as usize) {
            let block = edit_block as usize + offset;
            assert!(pcm.iter().any(|sample| *sample != 0.0), "signal");
            assert_eq!(bits(pcm), bits(&prepared[block]), "block {block}");
            heard |= bits(pcm) != bits(&unsoloed[block]);
        }
        assert!(heard, "the solo is heard");

        // Unsolo: back to the unsoloed plan's bits.
        let unsolo = rig.transaction(&[solo_edit("eq1", false, None)]);
        let (result, response) = rig.send(&unsolo);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), protocol::EditPath::Monitor);
        assert_eq!(solo_state(&mut rig, "eq1").0, 0.0);
        let edit_block = rig.block;
        let mut back = Vec::new();
        for _ in 0..k + 4 {
            back.push(rig.step());
        }
        let unsoloed = Reference::run(&document, rig.block);
        for (offset, pcm) in back.iter().enumerate().skip(k as usize) {
            let block = edit_block as usize + offset;
            assert_eq!(bits(pcm), bits(&unsoloed[block]), "unsolo, block {block}");
        }
        assert_eq!((rig.summary().0, rig.summary().2), (revision, epoch));
    }

    /// The edits that add an unmuted track `name`, a copy of `eq0`, with a route to the output.
    fn added_track(model: &SessionModel, name: &str) -> [SessionEdit; 2] {
        let mut track = model.tracks[0].clone();
        track.id = id(name);
        let mut route = model
            .routes
            .iter()
            .find(|route| route.id == id("eq0-main"))
            .expect("eq0's route to the output")
            .clone();
        route.id = id(&format!("{name}-main"));
        let RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        [
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ]
    }

    /// Renders until the pending candidate (if any) is adopted and its ramps and latency have
    /// passed, then `compared` more blocks; returns those and the index of the first.
    fn run_to_settled(rig: &mut Rig, ramp: u32, compared: u64) -> (Vec<Vec<f32>>, u64) {
        let mut guard = 0;
        while rig.summary().3 != 0 {
            rig.step();
            guard += 1;
            assert!(guard < 16, "the candidate is adopted");
        }
        for _ in 0..window(rig.latency(), ramp, rig.quantum) {
            rig.step();
        }
        let first = rig.block;
        ((0..compared).map(|_| rig.step()).collect(), first)
    }

    /// One stage of [`structural_script`]: its settled blocks, the first block's index and the
    /// committed session.
    type Stage = (Vec<Vec<f32>>, u64, String);

    /// The stages of the structural script of [`a_structural_commit_keeps_the_solo_and_a_removed_strip_drops_it`]
    /// on one rig: `eq1` soloed first when `solo`, then a track added, `eq1` removed, and a track
    /// named `eq1` added. Each stage's settled blocks and first block index come back with the
    /// rig's committed session.
    fn structural_script(document: &str, ramp: u32, solo: bool) -> (Vec<Stage>, Rig) {
        let model = parse_session_json(document).expect("model");
        let mut rig = Rig::new(document);
        for _ in 0..4 {
            rig.step();
        }
        if solo {
            assert_eq!(rig.apply(&[solo_edit("eq1", true, None)]), crate::RESULT_OK);
        }
        for _ in 0..window(rig.latency(), ramp, rig.quantum) {
            rig.step();
        }
        let mut stages = Vec::new();
        let mut stage = |rig: &mut Rig, edits: &[SessionEdit]| {
            let bytes = rig.transaction(edits);
            let (result, response) = rig.send(&bytes);
            assert_eq!(result, crate::RESULT_OK);
            assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
            let (blocks, first) = run_to_settled(rig, ramp, 4);
            stages.push((blocks, first, rig.snapshot()));
        };
        stage(&mut rig, &added_track(&model, "added"));
        stage(
            &mut rig,
            &[
                SessionEdit::RemoveRoute {
                    route_id: id("eq1-bus"),
                },
                SessionEdit::RemoveRoute {
                    route_id: id("eq1-main"),
                },
                SessionEdit::RemoveTrack {
                    track_id: id("eq1"),
                },
            ],
        );
        let reborn = added_track(&rig.model(), "eq1");
        stage(&mut rig, &reborn);
        (stages, rig)
    }

    /// Gate 2 (#1559): a structural commit keeps the solo. With `eq1` soloed, a track added by a
    /// rebuild is solo-muted from the successor's first block and `eq1` stays audible, as in a
    /// fresh plan prepared with the same overlay; removing `eq1` drops its entry, so every track is
    /// audible again (the blocks are those of a rig that never soloed), and a new strip named
    /// `eq1` is not soloed. A solo spends no revision number (#1603): the three rebuilds after it
    /// are the next three revisions.
    ///
    /// Red if a rebuild drops the overlay, keeps an entry for a removed strip, or leaves a new
    /// strip audible beside a soloed one.
    #[test]
    fn a_structural_commit_keeps_the_solo_and_a_removed_strip_drops_it() {
        let document = solo_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let (soloed, mut rig) = structural_script(&document, ramp, true);
        let (plain, _) = structural_script(&document, ramp, false);

        // The first rebuild: the solo survives, the added track is solo-muted, and the rebuilt
        // plan renders the bits of a fresh plan with the same overlay.
        let (blocks, first, committed) = &soloed[0];
        let prepared = Reference::run(&muted_but(committed, &["eq1"]), first + 4);
        for (offset, pcm) in blocks.iter().enumerate() {
            let block = *first as usize + offset;
            assert!(pcm.iter().any(|sample| *sample != 0.0), "signal");
            assert_eq!(
                bits(pcm),
                bits(&prepared[block]),
                "added track, block {block}"
            );
        }
        // Gone with the strip; and the strip that takes its name later is not soloed.
        for (stage, (a, _, _)) in soloed.iter().enumerate().skip(1) {
            assert_eq!(
                a.iter().map(|pcm| bits(pcm)).collect::<Vec<_>>(),
                plain[stage]
                    .0
                    .iter()
                    .map(|pcm| bits(pcm))
                    .collect::<Vec<_>>(),
                "stage {stage}: every track is audible, as on a rig that never soloed"
            );
        }
        assert_eq!(solo_state(&mut rig, "eq1").0, 0.0);
        assert_eq!(solo_state(&mut rig, "eq0").0, 0.0);
        // The solo took no revision number; the three rebuilds are consecutive after the boot
        // revision.
        assert_eq!(
            rig.summary().0,
            parse_session_json(&document).expect("model").revision + 3
        );
    }

    /// #1603 (#1559 D4): a solo spends no revision number. Solos between two model commits leave
    /// the committed revisions dense (committed `r`, solos, the next commit `r + 1`, a rebuild
    /// `r + 2`), the applied watermark never reads a revision above the committed one, its exact
    /// count is one per model revision, and a rebuild after the solos keeps the solo: the
    /// successor renders the bits of a plan prepared with every other track muted.
    ///
    /// Red if a solo's records consume a gate revision (the next commit skips a number or the
    /// watermark passes the committed revision), if a solo moves the exact count, or if a
    /// successor is swapped in without the solo it replaces.
    #[test]
    fn a_solo_spends_no_revision_and_a_rebuild_after_it_keeps_it() {
        let document = solo_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let initial = rig.summary().0;
        // Committed `revision`, the watermark's applied revision and its exact count, after two
        // blocks that give the plan its chance to apply anything pending.
        let settled = |rig: &mut Rig, revision: u64| {
            rig.step();
            rig.step();
            assert_eq!(rig.summary().0, revision, "the committed revision");
            let (applied, _, flags, exact) = watermark(rig);
            assert_eq!(applied, revision, "the watermark is the committed revision");
            assert_eq!(flags, crate::OUTCOME_EXACT);
            assert_eq!(
                exact,
                revision - initial,
                "one exact revision per model commit"
            );
        };
        let revision_of = |rig: &mut Rig, edits: &[SessionEdit]| {
            let bytes = rig.transaction(edits);
            let (result, response) = command_c(rig.session, &bytes);
            assert_eq!(result, crate::RESULT_OK);
            response_revision(&response)
        };

        // Solos, then a model commit: the commit is `r + 1`.
        assert_eq!(
            revision_of(&mut rig, &[solo_edit("eq1", true, None)]),
            initial
        );
        settled(&mut rig, initial);
        assert_eq!(
            revision_of(&mut rig, &[solo_edit("eq1", false, None)]),
            initial
        );
        settled(&mut rig, initial);
        let live = revision_of(
            &mut rig,
            &[set(builtin("eq0", FADER_DB, Lane::Both), -4.0, None)],
        );
        assert_eq!(live, initial + 1, "no number was spent by the solos");
        settled(&mut rig, initial + 1);

        // Many solos, each on a block of its own, never move the watermark past the commit.
        for round in 0..9 {
            let on = round % 2 == 0;
            assert_eq!(
                revision_of(&mut rig, &[solo_edit("eq1", on, None)]),
                initial + 1
            );
            settled(&mut rig, initial + 1);
        }
        assert_eq!(
            solo_state(&mut rig, "eq1").0,
            1.0,
            "the run ends soloing eq1"
        );

        // A rebuild is the next revision again, and the successor keeps the solo.
        let added = added_track(&rig.model(), "added");
        let bytes = rig.transaction(&added);
        let (result, response) = rig.send(&bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
        assert_eq!(response_revision(&response), initial + 2);
        let (blocks, first) = run_to_settled(&mut rig, ramp, 4);
        assert_eq!(rig.summary().0, initial + 2);
        let (applied, _, _, exact) = watermark(&rig);
        assert_eq!((applied, exact), (initial + 2, 2));
        assert_eq!(solo_state(&mut rig, "eq1").0, 1.0);
        let committed = rig.snapshot();
        let prepared = Reference::run(&muted_but(&committed, &["eq1"]), first + 4);
        for (offset, pcm) in blocks.iter().enumerate() {
            let block = first as usize + offset;
            assert!(pcm.iter().any(|sample| *sample != 0.0), "signal");
            assert_eq!(bits(pcm), bits(&prepared[block]), "rebuilt, block {block}");
        }
    }

    /// Gates 3 and 5 (#1559): a transaction of a VCA move, a solo and a pan out of domain is
    /// refused with the pan's `control.domain` at its edit, and the model, the revision, the
    /// overlay, the cells and the next 8 blocks are as without the call. A solo on a submix is
    /// `control.address.unknown`, on one lane `control.lane`, with `0.5` `control.domain`, and
    /// `controls::set` on the row is `control.monitor`; each changes nothing.
    ///
    /// Red if the overlay commits while the model refuses, if a submix becomes soloable, or if a
    /// malformed solo is accepted.
    #[test]
    fn a_refused_transaction_leaves_the_overlay_with_the_model() {
        let document = with_model(&solo_document(), |model| {
            model.vcas.push(vca_of("inner", &["eq0"]));
        });
        let mut rig = Rig::new(&document);
        let untouched = Reference::run(&document, 64);
        for (block, expected) in untouched.iter().enumerate().take(4) {
            assert_eq!(bits(&rig.step()), bits(expected), "warm-up {block}");
        }
        let validation = protocol::StatusCode::ValidationFailed;
        let before = refusal_state(&rig);
        let cells = rig.current_cells();
        let routes = rig.current_route_cells();
        let cases: [(&str, Vec<SessionEdit>, &str, u32); 5] = [
            (
                "a VCA move, a solo and a pan out of domain",
                vec![
                    set(vca_row("inner", VCA_OFFSET_DB, Lane::Both), -6.0, None),
                    solo_edit("eq1", true, None),
                    set(builtin("eq2", PAN, Lane::Both), 7.0, None),
                ],
                "control.domain",
                2,
            ),
            (
                "a submix has no solo",
                vec![solo_edit("bus", true, None)],
                "control.address.unknown",
                0,
            ),
            (
                "one lane",
                vec![set(own("eq1", SOLO, Lane::Left), 1.0, None)],
                "control.lane",
                0,
            ),
            (
                "a value between",
                vec![set(own("eq1", SOLO, Lane::Both), 0.5, None)],
                "control.domain",
                0,
            ),
            (
                "a ramp above the rate",
                vec![solo_edit("eq1", true, Some(48_001))],
                "session.edit.ramp_out_of_range",
                0,
            ),
        ];
        for (label, edits, code, index) in cases {
            assert_eq!(
                refusal(&mut rig, &edits),
                (validation, vec![(code.to_owned(), index)]),
                "{label}"
            );
            assert_eq!(refusal_state(&rig).0, before.0, "{label}: the revision");
            assert_eq!(refusal_state(&rig).1, before.1, "{label}: the model");
            assert_eq!(solo_state(&mut rig, "eq1").0, 0.0, "{label}: the overlay");
            assert_eq!(rig.current_cells(), cells, "{label}: the strip cells");
            assert_eq!(
                rig.current_route_cells(),
                routes,
                "{label}: the route cells"
            );
            assert_eq!(rig.summary().3, 0, "{label}: no candidate");
        }
        // The model write refuses the row, whatever transport reaches it (`controls::set` has the
        // same test in its own crate).
        let mut model = parse_session_json(&document).expect("model");
        assert_eq!(
            protocol::apply_session_edit(&mut model, &solo_edit("eq1", true, None)),
            Err(protocol::SessionEditError::Control(
                protocol::control_core::ControlError::Monitor
            ))
        );
        for (block, expected) in untouched.iter().enumerate().take(12).skip(4) {
            assert_eq!(bits(&rig.step()), bits(expected), "block {block}");
        }
        // The good solo alone commits, as a monitor write.
        let bytes = rig.transaction(&[solo_edit("eq1", true, None)]);
        let (result, response) = rig.send(&bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), protocol::EditPath::Monitor);
        assert_eq!(solo_state(&mut rig, "eq1").0, 1.0);
    }

    /// D5 (#1559): a solo beside a model edit is one model transaction. With a
    /// value edit it is `live`, with a structural one `rebuild`; either way the revision advances
    /// by one, exactly one `SESSION_COMMITTED` follows, the model holds the edit and not the solo,
    /// and the solo is in effect: the live one renders the bits of the plan prepared with the
    /// committed model and the other tracks muted.
    ///
    /// Red if a mixed transaction takes the monitor path or spends a second revision, loses the
    /// solo or the model edit, or reports the solo's records as nothing.
    #[test]
    fn a_solo_beside_a_model_edit_is_one_model_transaction() {
        let document = solo_document();
        let model = parse_session_json(&document).expect("model");
        let ramps = model.control_smoothing_samples();
        let ramp = ramps.fader.max(ramps.mute).max(ramps.pan).max(64);

        // A value edit: live.
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let (revision, _, epoch, _) = rig.summary();
        let bytes = rig.transaction(&[
            set(builtin("eq1", FADER_DB, Lane::Both), -4.0, None),
            solo_edit("eq1", true, None),
        ]);
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), protocol::EditPath::Live);
        assert_eq!(response_revision(&response), revision + 1);
        assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"]);
        assert_eq!((rig.summary().0, rig.summary().2), (revision + 1, epoch));
        assert_eq!(solo_state(&mut rig, "eq1").0, 1.0);
        let committed = rig.snapshot();
        assert!(
            !committed.contains("solo"),
            "the solo is not in the session"
        );
        let edit_block = rig.block;
        let k = window(rig.latency(), ramp, rig.quantum);
        let live: Vec<_> = (0..k + 4).map(|_| rig.step()).collect();
        let prepared = Reference::run(&muted_but(&committed, &["eq1"]), rig.block);
        for (offset, pcm) in live.iter().enumerate().skip(k as usize) {
            let block = edit_block as usize + offset;
            assert_eq!(bits(pcm), bits(&prepared[block]), "live, block {block}");
        }

        // A structural edit: a rebuild that carries the solo.
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let (revision, ..) = rig.summary();
        let [track, route] = added_track(&model, "added");
        let bytes = rig.transaction(&[track, route, solo_edit("eq1", true, Some(96))]);
        let (result, response) = command_c(rig.session, &bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
        assert_eq!(response_revision(&response), revision + 1);
        assert_eq!(reliable_event_kinds(&rig), ["SESSION_COMMITTED"]);
        assert_eq!(solo_state(&mut rig, "eq1").0, 1.0);
        let (blocks, first) = run_to_settled(&mut rig, ramp, 4);
        let committed = rig.snapshot();
        let prepared = Reference::run(&muted_but(&committed, &["eq1"]), first + 4);
        for (offset, pcm) in blocks.iter().enumerate() {
            let block = first as usize + offset;
            assert_eq!(bits(pcm), bits(&prepared[block]), "rebuild, block {block}");
        }
    }

    // #1709: live level values are composed in the canonical floating-point environment.

    /// #1709 D2 on the C ABI: a live move made while the host's control thread runs FTZ, DAZ and
    /// round toward zero.
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    mod canonical_live {
        use super::*;
        use lane::fpenv::under_hostile_fp_env;

        /// The member dB and the VCA offset of #1646's environment gate: their composition rounds
        /// differently under round toward zero, and the two compositions convert to different
        /// gains (both asserted below).
        const MEMBER_DB: f32 = -0.01384;
        const OFFSET_DB: f32 = 1.0 / 3.0;

        /// The flat curve of the automated rows (#1649, #1650b): `3 * 2^-20` dB, exactly.
        const CURVE_DB: f32 = 3.0 / 1_048_576.0;
        /// The VCA offset of the automated rows.
        const VCA_DB: f32 = 20.0;
        /// The hand offset of the automated rows: below half an `f64` ulp of [`VCA_DB`], so the
        /// lane's `S = (HAND_DB + VCA_DB) + 0` is `20` under round to nearest and the `f64` below
        /// `20` under round toward zero. `CURVE_DB + 20` is a midpoint between two `f32` values
        /// that rounds up (ties to even) and the composition with the lower `S` rounds down, so
        /// the two `S` reach different fader dB and different gains (asserted below).
        const HAND_DB: f32 = -1e-30;
        /// The hand offset the hand-move row starts from, so that its move changes `S`.
        const PRIOR_HAND_DB: f32 = 1.0;

        /// [`vca_document`] with `eq0`'s left fader at `member` and `inner`'s left offset at
        /// `offset` (`outer` stays at `0` dB, so the offsets' sum is `offset` exactly).
        fn document(member: f32, offset: f32) -> String {
            with_model(&vca_document(), |model| {
                model.tracks[0].fader.left_db = member;
                model.vcas[0].fader.left_db = offset;
            })
        }

        /// [`document`] with `eq0`'s left fader at [`CURVE_DB`] and also automated, flat at
        /// [`CURVE_DB`] with the stored hand offset `hand` (#1649, #1650): its gain is composed at
        /// render from the curve and the offsets cell's `S = (hand + vca) + 0`.
        fn automated_document(hand: f32, vca: f32) -> String {
            with_model(&document(CURVE_DB, vca), |model| {
                model.automation.push(session::Automation {
                    id: id("eq0-flat"),
                    target: session::ControlAddress {
                        owner: session::Owner::Strip(id("eq0")),
                        section: session::Section::Builtins,
                        control: FADER_DB,
                        lane: session::ParameterChannel::Left,
                    },
                    segments: vec![session::AutomationSegment {
                        shape: session::AutomationShape::Step,
                        start_sample: 0,
                        end_sample: SOURCE_FRAMES,
                        start_value: CURVE_DB,
                        end_value: CURVE_DB,
                        unit: session::ParameterUnit::Db,
                    }],
                    offset: session::AutomationOffset {
                        left_db: hand,
                        right_db: 0.0,
                    },
                });
            })
        }

        /// The gain bits of `db`.
        fn gain(db: f32) -> Option<u32> {
            builtins::checked_fader_gain(db).map(f32::to_bits).ok()
        }

        /// The fixture is not vacuous: the hostile word composes the pair to another `f32`, and
        /// that `f32` converts to another gain.
        fn assert_the_hostile_word_rounds_the_composition() {
            // `black_box` on the inputs and outputs: under `--release` LLVM otherwise folds the
            // composition at compile time, whatever the thread's word.
            let compose = || {
                core::hint::black_box(session::vca_effective_db(
                    core::hint::black_box(MEMBER_DB),
                    [core::hint::black_box(OFFSET_DB), 0.0],
                ))
            };
            let canonical = compose();
            let rounded = under_hostile_fp_env(compose);
            assert_ne!(
                canonical.to_bits(),
                rounded.to_bits(),
                "the gate is vacuous: the composition rounds alike under the hostile word"
            );
            assert_ne!(
                gain(canonical),
                gain(rounded),
                "the gate is vacuous: the composition's rounding does not reach the gain"
            );
        }

        /// The automated rows are not vacuous: the hostile word sums the lane's `S` to another
        /// `f64`, and render's composition of the curve with it (canonical, on the render
        /// thread) reaches another fader dB and another gain.
        fn assert_the_hostile_word_rounds_the_offsets_sum() {
            // `black_box` on the inputs and outputs: under `--release` LLVM otherwise folds the
            // constant sum at compile time, whatever the thread's word (#1775).
            let sum = || {
                core::hint::black_box(session::vca_offsets_sum([
                    core::hint::black_box(HAND_DB),
                    core::hint::black_box(VCA_DB),
                    core::hint::black_box(0.0),
                ]))
                .expect("three offsets")
            };
            let canonical = sum();
            let rounded = under_hostile_fp_env(sum);
            assert_eq!(canonical.to_bits(), f64::from(VCA_DB).to_bits());
            assert_ne!(
                canonical.to_bits(),
                rounded.to_bits(),
                "the gate is vacuous: S sums alike under the hostile word"
            );
            let (canonical, rounded) = (
                session::vca_compose_db(CURVE_DB, canonical),
                session::vca_compose_db(CURVE_DB, rounded),
            );
            assert_ne!(
                canonical.to_bits(),
                rounded.to_bits(),
                "the gate is vacuous: the two S compose to one fader dB"
            );
            assert_ne!(
                gain(canonical),
                gain(rounded),
                "the gate is vacuous: the two fader dB give one gain"
            );
        }

        /// One case: a rig on `before` takes `edit` under the hostile word, live, and from the
        /// block its ramp leaves behind renders the bits of a canonical preparation of the
        /// committed session. `committed` checks the committed model and the rig's catalog.
        fn live_move_under_hostile_word(
            label: &str,
            before: &str,
            edit: SessionEdit,
            committed: impl FnOnce(&mut Rig, &SessionModel),
        ) {
            let mut rig = Rig::new(before);
            for _ in 0..4 {
                rig.step();
            }
            let (revision, _, epoch, pending) = rig.summary();
            let bytes = rig.transaction(&[edit]);
            let (result, response) = under_hostile_fp_env(|| rig.send(&bytes));
            assert_eq!(result, crate::RESULT_OK, "{label}");
            assert_eq!(
                edit_path_of(&response),
                protocol::EditPath::Live,
                "{label}: the move is live"
            );
            assert_eq!(
                (rig.summary().0, rig.summary().2, rig.summary().3),
                (revision + 1, epoch, pending),
                "{label}: one revision, the same plan, no candidate"
            );
            let snapshot = rig.snapshot();
            let model = parse_session_json(&snapshot).expect("model");
            committed(&mut rig, &model);
            let ramps = model.control_smoothing_samples();
            let k = window(rig.latency(), ramps.fader.max(64), rig.quantum);
            let edit_block = rig.block;
            let live: Vec<Vec<f32>> = (0..k + 4).map(|_| rig.step()).collect();
            let reference = Reference::run(&snapshot, rig.block);
            for (offset, pcm) in live.iter().enumerate().skip(k as usize) {
                let block = edit_block as usize + offset;
                assert!(pcm.iter().any(|sample| *sample != 0.0), "{label}: signal");
                assert_eq!(
                    bits(pcm),
                    bits(&reference[block]),
                    "{label}: block {block} renders the canonical preparation's bits"
                );
            }
        }

        /// The static rows' check: the committed pair is the fixture's, and the catalog's derived
        /// `effective_fader_db` row holds the canonical composition.
        fn static_pair(label: &'static str) -> impl FnOnce(&mut Rig, &SessionModel) {
            move |rig, model| {
                assert_eq!(
                    (
                        model.tracks[0].fader.left_db.to_bits(),
                        model.vcas[0].fader.left_db.to_bits()
                    ),
                    (MEMBER_DB.to_bits(), OFFSET_DB.to_bits()),
                    "{label}: the committed pair is the fixture's"
                );
                let (list, ..) = control_list_all_c(rig.session, &mut rig.request, 256);
                let derived = listed(&list, |entry, row| {
                    entry.address.owner_id.as_deref() == Some("eq0")
                        && row.name == "effective_fader_db"
                        && entry.address.lane == Some(protocol::ParameterChannel::Left)
                });
                assert_eq!(
                    control_of(derived).map(|row| row.value.to_bits()),
                    Some(session::vca_effective_db(MEMBER_DB, [OFFSET_DB, 0.0]).to_bits()),
                    "{label}: the catalog's derived row holds the canonical composition"
                );
            }
        }

        /// The automated rows' check: the committed hand offset and VCA offset are the fixture's.
        fn automated_pair(label: &'static str) -> impl FnOnce(&mut Rig, &SessionModel) {
            move |_, model| {
                let ride = model
                    .automation
                    .iter()
                    .find(|entry| entry.id == id("eq0-flat"))
                    .expect("the ride");
                assert_eq!(
                    (
                        ride.offset.left_db.to_bits(),
                        model.vcas[0].fader.left_db.to_bits()
                    ),
                    (HAND_DB.to_bits(), VCA_DB.to_bits()),
                    "{label}: the committed pair is the fixture's"
                );
            }
        }

        /// #1709 D2 (C ABI): a live fader move on a VCA member, a live VCA offset move over it,
        /// (#1649) a VCA move over a member whose fader is automated, which writes its offsets
        /// cell, and (#1650b) a hand move on that automated fader, which writes it too, each made
        /// while the caller's thread runs FTZ, DAZ and round toward zero, render from the end of
        /// their ramp the bits of the same committed session prepared on a canonical thread, and
        /// the catalog's derived fader row holds the canonical composition. The static pair's
        /// composition rounds to another `f32`, and another gain, under that word, and the
        /// automated rows' `S` sums to another `f64`, which render composes to another gain
        /// (both asserted), so no row is vacuous.
        ///
        /// Test value: red if a live transaction composes a member's effective fader (the VCA
        /// sum) or an automated lane's offsets-cell `S` in the caller's environment: remove the
        /// guard in `SessionState::command`.
        #[test]
        fn a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits() {
            assert_the_hostile_word_rounds_the_composition();
            assert_the_hostile_word_rounds_the_offsets_sum();
            live_move_under_hostile_word(
                "a live fader move",
                &document(0.0, OFFSET_DB),
                set(builtin("eq0", FADER_DB, Lane::Left), MEMBER_DB, None),
                static_pair("a live fader move"),
            );
            live_move_under_hostile_word(
                "a live VCA move",
                &document(MEMBER_DB, 0.0),
                set(vca_row("inner", VCA_OFFSET_DB, Lane::Left), OFFSET_DB, None),
                static_pair("a live VCA move"),
            );
            // #1649: a VCA move over an automated member writes its offsets cell.
            live_move_under_hostile_word(
                "a live VCA move on an automated lane",
                &automated_document(HAND_DB, 0.0),
                set(vca_row("inner", VCA_OFFSET_DB, Lane::Left), VCA_DB, None),
                automated_pair("a live VCA move on an automated lane"),
            );
            // #1650b: a hand move on the automated lane writes its offsets cell with `S` = the
            // stored offset and the VCA's.
            live_move_under_hostile_word(
                "a hand move on an automated lane",
                &automated_document(PRIOR_HAND_DB, VCA_DB),
                set(builtin("eq0", FADER_DB, Lane::Left), HAND_DB, None),
                automated_pair("a hand move on an automated lane"),
            );
        }

        /// #1709 D1 (C ABI): a session compiled and booted from its JSON while the caller's
        /// thread runs FTZ, DAZ and round toward zero is the session a canonical boot gives: the
        /// same committed snapshot and the same rendered bits. The session's JSON parse reads
        /// `0.1` as another `f32` under that word (asserted), so the gate is not vacuous.
        ///
        /// Test value: red if `compile_children` parses or prepares the session in the caller's
        /// environment: remove its `CanonicalFpEnv` guard.
        #[test]
        fn a_session_booted_under_a_hostile_caller_word_is_the_canonical_session() {
            // Opaque, so no profile folds the parse at compile time (#1775).
            let parse = || {
                core::hint::black_box(
                    core::hint::black_box("0.1")
                        .parse::<f32>()
                        .expect("a number"),
                )
            };
            assert_ne!(
                parse().to_bits(),
                under_hostile_fp_env(parse).to_bits(),
                "the gate is vacuous: the parse rounds alike under the hostile word"
            );
            let document = with_model(&vca_document(), |model| {
                model.tracks[0].fader.left_db = 0.1;
                model.vcas[0].fader.left_db = 0.1;
            });
            let mut canonical = Rig::new(&document);
            let mut hostile = under_hostile_fp_env(|| Rig::new(&document));
            assert_eq!(
                hostile.snapshot(),
                canonical.snapshot(),
                "the committed session is the canonical parse"
            );
            let mut signal = false;
            for block in 0..16 {
                let (pcm, reference) = (hostile.step(), canonical.step());
                signal |= pcm.iter().any(|sample| *sample != 0.0);
                assert_eq!(bits(&pcm), bits(&reference), "block {block}");
            }
            assert!(signal, "signal");
        }
    }

    /// #1649: a VCA edit over a member with stored fader automation, on the C ABI.
    mod automated_vca {
        use super::*;

        /// The fader ride of `eq0` (both lanes): linear from 0 dB at sample 0 to -12 dB at the
        /// source's end, so every grid point moves the lane.
        pub(super) fn ride() -> session::Automation {
            session::Automation {
                id: id("eq0-fader-ride"),
                target: session::ControlAddress {
                    owner: session::Owner::Strip(id("eq0")),
                    section: session::Section::Builtins,
                    control: FADER_DB,
                    lane: session::ParameterChannel::Both,
                },
                segments: vec![session::AutomationSegment {
                    shape: session::AutomationShape::Linear,
                    start_sample: 0,
                    end_sample: SOURCE_FRAMES,
                    start_value: 0.0,
                    end_value: -12.0,
                    unit: session::ParameterUnit::Db,
                }],
                offset: session::AutomationOffset::default(),
            }
        }
    }

    /// #1648 D2 and #1649 across a plan swap (batch 17 U3 fold): a level value that reaches an
    /// automated fader lane around a rebuild is heard as the uninterrupted engine hears it. Each
    /// test renders rigs of one document in lockstep: `a` takes a rebuild (a muted track added,
    /// whose strip renders exact zeros), `u` the same live edits with no rebuild, and `plain`
    /// none of the edits, to show that they are heard.
    mod automated_carry {
        use super::*;

        /// A fader entry of track `track` (both lanes) with `segments`.
        fn entry(track: &str, segments: Vec<session::AutomationSegment>) -> session::Automation {
            session::Automation {
                id: id("ride"),
                target: session::ControlAddress {
                    owner: session::Owner::Strip(id(track)),
                    section: session::Section::Builtins,
                    control: FADER_DB,
                    lane: session::ParameterChannel::Both,
                },
                segments,
                offset: session::AutomationOffset::default(),
            }
        }

        /// One segment of `shape` over `[start, end)`, from `from` to `to` dB.
        fn segment(
            shape: session::AutomationShape,
            (start, end): (u64, u64),
            (from, to): (f32, f32),
        ) -> session::AutomationSegment {
            session::AutomationSegment {
                shape,
                start_sample: start,
                end_sample: end,
                start_value: from,
                end_value: to,
                unit: session::ParameterUnit::Db,
            }
        }

        /// The stateless session (no effect, no input filter) with a long source and `change`
        /// applied, given the first two track IDs.
        fn session_with(change: impl FnOnce(&mut SessionModel, &str, &str)) -> String {
            let mut model = stateless_session(48_000);
            model.sources[0].frames = SOURCE_FRAMES;
            let first = model.tracks[0].id.as_str().to_owned();
            let second = model.tracks[1].id.as_str().to_owned();
            change(&mut model, &first, &second);
            session::canonical_session_json(&model).expect("canonical session")
        }

        /// The first track rides its fader linearly from 0 dB to -12 dB over the source, so every
        /// grid point moves the lane, and VCA `v` holds the first two tracks (the second has no
        /// automation).
        fn riding() -> String {
            session_with(|model, first, second| {
                model.automation.push(entry(
                    first,
                    vec![segment(
                        session::AutomationShape::Linear,
                        (0, SOURCE_FRAMES),
                        (0.0, -12.0),
                    )],
                ));
                model.vcas.push(vca_of("v", &[first, second]));
            })
        }

        /// [`riding`] with a flat ride at -3 dB: an offsets jump then moves the lane only because
        /// `S` moved, so the `S` it jumps from decides whether it jumps at all.
        fn flat() -> String {
            session_with(|model, first, second| {
                model.automation.push(entry(
                    first,
                    vec![segment(
                        session::AutomationShape::Step,
                        (0, SOURCE_FRAMES),
                        (-3.0, -3.0),
                    )],
                ));
                model.vcas.push(vca_of("v", &[first, second]));
            })
        }

        /// The first track of `document`.
        fn first_track(document: &str) -> String {
            parse_session_json(document).expect("model").tracks[0]
                .id
                .as_str()
                .to_owned()
        }

        /// A move of `v` to `db` over `ramp`.
        fn vca_move(db: f32, ramp: Option<u32>) -> SessionEdit {
            set(vca_row("v", VCA_OFFSET_DB, Lane::Both), db, ramp)
        }

        /// Send `edits` on `rig`; the reply's path must be `path`.
        fn send(rig: &mut Rig, edits: &[SessionEdit], path: protocol::EditPath, label: &str) {
            let bytes = rig.transaction(edits);
            let (result, response) = rig.send(&bytes);
            assert_eq!(result, crate::RESULT_OK, "{label}");
            assert_eq!(edit_path_of(&response), path, "{label}: path");
        }

        /// Three rigs of `document`, four blocks rendered on each.
        fn warm(document: &str) -> [Rig; 3] {
            let mut rigs = [0, 1, 2].map(|_| Rig::new(document));
            for _ in 0..4 {
                let [a, u, plain] = &mut rigs;
                let x = bits(&a.step());
                assert_eq!(x, bits(&u.step()), "warm-up");
                assert_eq!(x, bits(&plain.step()), "warm-up");
            }
            rigs
        }

        /// Render `blocks` blocks on the three rigs. Returns the first block of `a` whose bits
        /// differ from `u`'s, with its first differing sample, and whether `u` differs from
        /// `plain` in any block. Every block of `a` is allocation-free.
        fn lockstep(rigs: &mut [Rig; 3], blocks: usize) -> (Option<(u64, usize)>, bool) {
            let [a, u, plain] = rigs;
            let (mut first, mut heard) = (None, false);
            for _ in 0..blocks {
                let block = a.block;
                let (x, used) = a.step_counted();
                assert_eq!(
                    (used.allocations, used.deallocations),
                    (0, 0),
                    "block {block} allocated or freed"
                );
                let (x, y) = (bits(&x), bits(&u.step()));
                heard |= y != bits(&plain.step());
                if x != y && first.is_none() {
                    first = Some((block, x.iter().zip(&y).position(|(x, y)| x != y).unwrap()));
                }
            }
            (first, heard)
        }

        /// The superseded-value counter of `rig`.
        fn superseded(rig: &mut Rig) -> u64 {
            rig.counter(protocol::CounterId::LiveValuesSuperseded)
        }

        /// U3 fold, MAJOR 1 (R1c, R9, R10). Live VCA moves whose offsets writes are still unread
        /// when a rebuild commits (no block between them) are carried into the successor: the
        /// carry drain reads them, and the successor's first block jumps from the `S` the stage
        /// heard over the edit's own length, as the uninterrupted engine does. Cases: one move
        /// with its own 4,000 samples, one with the session's length, two moves (the first is
        /// superseded unheard), and a rebuild committed first with the move written to the
        /// pending successor. Every block equals the uninterrupted engine's, the moves are
        /// heard, the successor is adopted, and `LIVE_VALUES_SUPERSEDED` holds the same count.
        ///
        /// Test value: red if the carry drain does not read the offsets cells (the successor
        /// starts from the committed `S` and jumps over the fader length, and a superseded write
        /// goes uncounted), or if the carried record or the heard `S` does not reach the
        /// successor's cell.
        #[test]
        fn vca_moves_pending_at_a_swap_keep_their_length_and_their_count() {
            bench_support::alloc::assert_installed();
            for (ride, document) in [("moving ride", riding()), ("flat ride", flat())] {
                pending_case(ride, &document);
            }
        }

        /// One document of [`vca_moves_pending_at_a_swap_keep_their_length_and_their_count`].
        fn pending_case(ride: &str, document: &str) {
            let model = parse_session_json(document).expect("model");
            let rebuild = add_muted_track(&model, "added");
            let cases: [(&str, Vec<SessionEdit>, bool); 4] = [
                (
                    "its own 4,000 samples",
                    vec![vca_move(-6.0, Some(4_000))],
                    false,
                ),
                ("the session's length", vec![vca_move(-6.0, None)], false),
                (
                    "two moves",
                    vec![vca_move(-6.0, None), vca_move(-3.0, Some(4_000))],
                    false,
                ),
                (
                    "the rebuild committed first",
                    vec![vca_move(-6.0, Some(4_000))],
                    true,
                ),
            ];
            for (label, moves, rebuild_first) in cases {
                let label = &format!("{ride}, {label}");
                let mut rigs = warm(document);
                if rebuild_first {
                    send(&mut rigs[0], &rebuild, protocol::EditPath::Rebuild, label);
                }
                for edit in &moves {
                    for rig in &mut rigs[..2] {
                        send(
                            rig,
                            core::slice::from_ref(edit),
                            protocol::EditPath::Live,
                            label,
                        );
                    }
                }
                if !rebuild_first {
                    send(&mut rigs[0], &rebuild, protocol::EditPath::Rebuild, label);
                }
                let (first, heard) = lockstep(&mut rigs, 44);
                assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the moves are heard");
                let counts = [superseded(&mut rigs[0]), superseded(&mut rigs[1])];
                assert_eq!(
                    counts[0], counts[1],
                    "{label}: LIVE_VALUES_SUPERSEDED [a, u]"
                );
            }
        }

        /// A hand move (#1650b) of `document`'s first track's fader to `db` on both lanes over
        /// `ramp`: on an automated fader it is stored as the ride's offset and moves `S`.
        fn hand_move(document: &str, db: f32, ramp: Option<u32>) -> SessionEdit {
            set(
                builtin(&first_track(document), FADER_DB, Lane::Both),
                db,
                ramp,
            )
        }

        /// U3 fold residue R2b (#1648, #1650b). A live hand move on the riding fader with its own
        /// 4,000-sample ramp, still unread when a rebuild commits (no block between them), is
        /// carried into the successor as the VCA move is: every block equals the uninterrupted
        /// engine's, the move is heard, the successor is adopted, and `LIVE_VALUES_SUPERSEDED`
        /// holds the same count. On the moving and the flat ride.
        ///
        /// Test value: red if a hand move's offsets write misses the carry drain (checked once)
        /// or its record loses its ramp word across the swap: the successor then jumps over the
        /// fader length from its prepared `S`. A hand move reaches the offsets cell through the
        /// fader row's stored offset, not the VCA row, which the VCA forms do not exercise. The
        /// rigs are C ABI plans (the shipped preparer, live cells on every strip): with the carry
        /// drain's read removed, a debug build stops first at the B17-R12 `debug_assert` (the
        /// adoption backstop becomes reachable; a panic in render aborts the test process, red),
        /// and a release build reaches this test's bit assert (red at the moving ride's first
        /// block after the swap).
        #[test]
        fn a_hand_move_pending_at_a_swap_keeps_its_length() {
            bench_support::alloc::assert_installed();
            for (ride, document) in [("moving ride", riding()), ("flat ride", flat())] {
                let label = &format!("{ride}, a hand move of 4,000 samples");
                let model = parse_session_json(&document).expect("model");
                let mut rigs = warm(&document);
                let edit = hand_move(&document, -6.0, Some(4_000));
                for rig in &mut rigs[..2] {
                    send(
                        rig,
                        core::slice::from_ref(&edit),
                        protocol::EditPath::Live,
                        label,
                    );
                }
                send(
                    &mut rigs[0],
                    &add_muted_track(&model, "added"),
                    protocol::EditPath::Rebuild,
                    label,
                );
                let (first, heard) = lockstep(&mut rigs, 44);
                assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the move is heard");
                let counts = [superseded(&mut rigs[0]), superseded(&mut rigs[1])];
                assert_eq!(
                    counts[0], counts[1],
                    "{label}: LIVE_VALUES_SUPERSEDED [a, u]"
                );
            }
        }

        /// U3 fold residue, reviewer C's case as a hand move (#1648, #1650b). A hand move with an
        /// explicit 4,096-sample ramp in the same transaction as a structural edit (the
        /// transaction rebuilds) reaches the carried automated lane with its own length: every
        /// block equals the same hand move sent alone (`live`). On the moving and the flat ride.
        ///
        /// Test value: red if the join writes no offsets record for a hand move's stored offset
        /// (the successor's `S` then reaches the lane by the adoption rule, over the fader length
        /// `J`, not over 4,096 samples). The rigs are C ABI plans (the shipped preparer): with the
        /// join's offsets write removed, a debug build stops first at the B17-R12 `debug_assert`
        /// (a panic in render aborts the test process, red), and a release build reaches this
        /// test's bit assert.
        #[test]
        fn a_hand_move_inside_a_rebuild_keeps_its_own_length() {
            bench_support::alloc::assert_installed();
            for (ride, document) in [("moving ride", riding()), ("flat ride", flat())] {
                let label = &format!("{ride}, a hand move of 4,096 samples");
                let model = parse_session_json(&document).expect("model");
                let mut rigs = warm(&document);
                let edit = hand_move(&document, -6.0, Some(4_096));
                let mut edits = vec![edit.clone()];
                edits.extend(add_muted_track(&model, "added"));
                send(&mut rigs[0], &edits, protocol::EditPath::Rebuild, label);
                send(&mut rigs[1], &[edit], protocol::EditPath::Live, label);
                let (first, heard) = lockstep(&mut rigs, 44);
                assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the move is heard");
            }
        }

        /// When a live level move reaches a swap whose rebuild changes the ride's table: still
        /// unread, in the rebuild's own transaction, or heard some blocks before it.
        #[derive(Clone, Copy, Debug)]
        enum MoveTiming {
            /// The move is committed live and the rebuild follows with no block between them.
            Pending,
            /// The move is in the rebuild's transaction.
            SameTransaction,
            /// The move is heard for this many blocks before the rebuild.
            Heard(usize),
        }

        /// The ride of [`riding`] split into two linear segments at `at` that join where the
        /// single segment passes (equal values up to `f32` rounding), the second ending at
        /// `end_db` (-12 dB keeps the ride; another value changes only the second segment), with
        /// the stored `offset` (a hand move's, which the upsert must keep).
        fn split_ride(
            track: &str,
            (at, end_db): (u64, f32),
            offset: session::AutomationOffset,
        ) -> SessionEdit {
            let lin = session::AutomationShape::Linear;
            let k = -12.0 * at as f32 / SOURCE_FRAMES as f32;
            let mut automation = entry(
                track,
                vec![
                    segment(lin, (0, at), (0.0, k)),
                    segment(lin, (at, SOURCE_FRAMES), (k, end_db)),
                ],
            );
            automation.offset = offset;
            SessionEdit::UpsertAutomation { automation }
        }

        /// U3 fold 2 (#1648 D2, D15-7) and #1711 G1. A live level move with its own 4,000 samples
        /// on the riding fader (a VCA move and a hand move), and a rebuild whose only change is
        /// the ride's table: the same ride split in two at sample 10,000, or a change only in its
        /// last 2,000 samples. The move is pending at the swap, in the rebuild's transaction, or
        /// heard 1, 12, 24, 26 and 30 blocks before it (3,872, 2,464, 928, 672 and 160 samples
        /// left, the last three shorter than the fader length `J` = 960). Every block of the
        /// rebuilt rig is bit-equal to the uninterrupted engine's over 44 blocks, allocation-free,
        /// the successor is adopted, and the rebuild's watermark advance reports `OUTCOME_EXACT`
        /// with no transition fallback (#1711 D2: the B17-R14 flag is deleted). The same rebuild
        /// with no live move is `a_table_only_rebuild_with_no_move_is_bit_equal`.
        ///
        /// Test value: red if the adoption rule compares the curve with the stage's fader gain
        /// rather than its ramp's target (#1711 F0: the held grid events only remember a gain, so
        /// the fader gain follows the ride and every heard move lands at the swap; 4.6e-6 heard
        /// 12, up to 3.1e-3 heard 24 to 30), if the repositioned cell drops the hold of the ramp
        /// the free stage still runs, or if it compares the curve at the block's first sample
        /// rather than at that ramp's end.
        #[test]
        fn a_level_move_across_a_table_change_keeps_its_length() {
            bench_support::alloc::assert_installed();
            let document = riding();
            let track = first_track(&document);
            let moves = [
                ("a VCA move", vca_move(-6.0, Some(4_000))),
                ("a hand move", hand_move(&document, -6.0, Some(4_000))),
            ];
            let changes = [
                ("the ride split at 10,000", (10_000, -12.0)),
                (
                    "a change in the last 2,000 samples",
                    (SOURCE_FRAMES - 2_000, -6.0),
                ),
            ];
            let timings = [
                MoveTiming::Pending,
                MoveTiming::SameTransaction,
                MoveTiming::Heard(1),
                MoveTiming::Heard(12),
                MoveTiming::Heard(24),
                MoveTiming::Heard(26),
                MoveTiming::Heard(30),
            ];
            for (what, edit) in &moves {
                for &(change, table) in &changes {
                    for timing in timings {
                        let label = &format!("{what}, {change}, {timing:?}");
                        let mut rigs = warm(&document);
                        send(
                            &mut rigs[1],
                            core::slice::from_ref(edit),
                            protocol::EditPath::Live,
                            label,
                        );
                        // The offset the move stored (a hand move's; zero for a VCA move).
                        let offset = rigs[1].model().automation[0].offset;
                        let rebuild = &split_ride(&track, table, offset);
                        if let MoveTiming::SameTransaction = timing {
                            send(
                                &mut rigs[0],
                                &[edit.clone(), rebuild.clone()],
                                protocol::EditPath::Rebuild,
                                label,
                            );
                        } else {
                            send(
                                &mut rigs[0],
                                core::slice::from_ref(edit),
                                protocol::EditPath::Live,
                                label,
                            );
                            if let MoveTiming::Heard(blocks) = timing {
                                let (first, _) = lockstep(&mut rigs, blocks);
                                assert_eq!(first, None, "{label}: before the rebuild");
                            }
                            send(
                                &mut rigs[0],
                                core::slice::from_ref(rebuild),
                                protocol::EditPath::Rebuild,
                                label,
                            );
                        }
                        let (first, _) = lockstep(&mut rigs, 44);
                        assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                        assert_eq!(first, None, "{label}: first difference (block, sample)");
                        assert_exact(&rigs[0], label);
                    }
                }
            }
        }

        /// #1716 (fix D). Two table-only rebuilds of the riding fader with no live move, 5 blocks
        /// apart: the ride split in two at sample 10,000 exactly on its line (every split-table
        /// value equals the ride's in `f32` bits), then the single ride again. Every block of the
        /// rebuilt rig is bit-equal to the uninterrupted engine's over 44 blocks, as a plan
        /// prepared with either table renders, the successor is adopted, and each rebuild's
        /// watermark advance reports `OUTCOME_EXACT` with no transition fallback.
        ///
        /// Test value: red when a table-only rebuild of an unchanged curve opens with an adoption
        /// jump that a plan prepared with the new table never makes. When the block starts on the
        /// grid (here every 128-sample block does), the last grid ramp has just completed, so the
        /// stage's target is the old curve at `span.start - 1`; an adoption that compares the new
        /// curve at `span.start` with it differs on every ride and opens the block with a jump
        /// over `J` = 960 (measured on that compare: first difference at block 4, sample 0; at most
        /// 1.7e-4, block 7; equal again from block 12). The claim is owned by builtins-compiler's
        /// `a_carried_composed_lane_renders_the_uninterrupted_bits` (its 128-frame case); this is
        /// the C ABI's parity row: it adds the C ABI rebuild's `Reposition` classification and
        /// the watermark's `OUTCOME_EXACT`.
        #[test]
        fn a_table_only_rebuild_with_no_move_is_bit_equal() {
            bench_support::alloc::assert_installed();
            let document = riding();
            let track = first_track(&document);
            let mut rigs = warm(&document);
            let offset = rigs[0].model().automation[0].offset;
            send(
                &mut rigs[0],
                &[split_ride(&track, (10_000, -12.0), offset)],
                protocol::EditPath::Rebuild,
                "split",
            );
            let (first, _) = lockstep(&mut rigs, 5);
            assert_eq!(rigs[0].summary().3, 0, "split: the successor is adopted");
            assert_eq!(first, None, "split: first difference (block, sample)");
            assert_exact(&rigs[0], "split");
            let mut whole = entry(
                &track,
                vec![segment(
                    session::AutomationShape::Linear,
                    (0, SOURCE_FRAMES),
                    (0.0, -12.0),
                )],
            );
            whole.offset = offset;
            send(
                &mut rigs[0],
                &[SessionEdit::UpsertAutomation { automation: whole }],
                protocol::EditPath::Rebuild,
                "whole",
            );
            let (first, _) = lockstep(&mut rigs, 39);
            assert_eq!(rigs[0].summary().3, 0, "whole: the successor is adopted");
            assert_eq!(first, None, "whole: first difference (block, sample)");
            assert_exact(&rigs[0], "whole");
        }

        /// [`riding`]'s ride kept up to `at`, then a step to `db` from `at` to the source's end,
        /// with the stored `offset`.
        fn stepped_ride(
            track: &str,
            at: u64,
            db: f32,
            offset: session::AutomationOffset,
        ) -> session::Automation {
            let k = -12.0 * at as f32 / SOURCE_FRAMES as f32;
            let mut automation = entry(
                track,
                vec![
                    segment(session::AutomationShape::Linear, (0, at), (0.0, k)),
                    segment(
                        session::AutomationShape::Step,
                        (at, SOURCE_FRAMES),
                        (db, db),
                    ),
                ],
            );
            automation.offset = offset;
            automation
        }

        /// B17-R13 (#1648 D2). A live level move with its own 4,000 samples (a VCA move and a
        /// hand move) heard 1 and 12 blocks, then a rebuild whose ride steps to -20 dB exactly
        /// at the swap block's first sample, or one sample after it (inside the landing block).
        /// The rebuilt rig is compared with a fresh plan prepared with the stepped ride from the
        /// start and given the same move at the same block: at the step the lane jumps over the
        /// fader length `J` in both, so from the step plus `J` on every sample stays within 1e-3
        /// of the fresh plan's (measured: 7.8e-5 heard 1, 5.2e-4 heard 12) over 44 blocks, every
        /// block is allocation-free, and the successor is adopted.
        ///
        /// Test value: red if the adoption skips a step at the block's first sample (it compares
        /// the curve where the move's ramp ends and lands over the ramp's remaining length, not
        /// over `J` at the step; 8.6e-2, checked once), or if the step one sample after the swap
        /// does not jump at its own sample (B17-R13 measured 8.6e-2 when a landing ran over the
        /// whole block, step included; #1711 D1 removed that landing).
        #[test]
        fn a_step_at_a_reposition_swap_jumps_over_the_fader_length() {
            /// The session's fader jump length `J`.
            const FADER_JUMP: u64 = 960;
            bench_support::alloc::assert_installed();
            let document = riding();
            let track = first_track(&document);
            for (what, hand) in [("a VCA move", false), ("a hand move", true)] {
                for heard in [1_usize, 12] {
                    for delta in [0_u64, 1] {
                        let label = &format!("{what}, heard {heard}, a step at the swap + {delta}");
                        let edit_of = |document: &str| {
                            if hand {
                                hand_move(document, -6.0, Some(4_000))
                            } else {
                                vca_move(-6.0, Some(4_000))
                            }
                        };
                        let mut a = Rig::new(&document);
                        for _ in 0..4 {
                            a.step();
                        }
                        send(
                            &mut a,
                            core::slice::from_ref(&edit_of(&document)),
                            protocol::EditPath::Live,
                            label,
                        );
                        let swap_block = a.block + heard as u64;
                        let at = swap_block * a.quantum as u64 + delta;
                        let offset = a.model().automation[0].offset;
                        let stepped = stepped_ride(&track, at, -20.0, offset);
                        // The fresh plan: the stepped ride from the start, the same move.
                        let mut model = parse_session_json(&document).expect("model");
                        model.automation[0] =
                            stepped_ride(&track, at, -20.0, session::AutomationOffset::default());
                        let fresh = session::canonical_session_json(&model).expect("canonical");
                        let mut r = Rig::new(&fresh);
                        for _ in 0..4 {
                            r.step();
                        }
                        send(
                            &mut r,
                            core::slice::from_ref(&edit_of(&fresh)),
                            protocol::EditPath::Live,
                            label,
                        );
                        // Before the swap the fresh plan differs already (its move aims short of
                        // the step, the rule of the cell's own target), so only the samples from
                        // the step's jump completion on are compared.
                        for _ in 0..heard {
                            a.step();
                            r.step();
                        }
                        send(
                            &mut a,
                            &[SessionEdit::UpsertAutomation {
                                automation: stepped,
                            }],
                            protocol::EditPath::Rebuild,
                            label,
                        );
                        let mut worst = (0.0_f32, 0_u64);
                        for _ in 0..44 {
                            let block = a.block;
                            let (x, used) = a.step_counted();
                            assert_eq!(
                                (used.allocations, used.deallocations),
                                (0, 0),
                                "{label}: block {block} allocated or freed"
                            );
                            let start = block * a.quantum as u64;
                            for (i, (p, q)) in x.iter().zip(&r.step()).enumerate() {
                                let d = (p - q).abs();
                                if start + i as u64 >= at + FADER_JUMP && d > worst.0 {
                                    worst = (d, block);
                                }
                            }
                        }
                        assert_eq!(a.summary().3, 0, "{label}: the successor is adopted");
                        assert!(
                            worst.0 < 1e-3,
                            "{label}: max |a - r| from the step plus J {} at block {}",
                            worst.0,
                            worst.1
                        );
                    }
                }
            }
        }

        /// One moved-target case of #1711 G2 and R19-1 on the C ABI: on [`flat`]'s ride (-3 dB),
        /// a live level move to -6 dB over its own 4,000 samples from block 4 (a VCA move or a
        /// hand move), heard `heard` blocks, then a table-only rebuild to -5 dB up to 6,000, then
        /// -3 dB (the move's target moves), and the `later` live moves `(block, dB, ramp)`. The
        /// rebuilt rig is compared with a fresh plan prepared with the reference table (-3 dB to
        /// the move's first free sample, -5 dB to 6,000, then -3 dB; with later moves, the
        /// rebuild's own table) and given the same moves at the same blocks: from sample `from`
        /// on every sample is bit-equal over 44 blocks after the swap, every block is
        /// allocation-free, the successor is adopted, and every revision completed exactly (the
        /// rebuild's and the later moves': `OUTCOME_EXACT`, no transition fallback).
        fn moved_target_case(hand: bool, heard: usize, later: &[(u64, f32, u32)], from: u64) {
            let free = 4 * 128 + 4_000;
            let pieces = |pieces: &[(u64, u64, f32)]| {
                let step = session::AutomationShape::Step;
                pieces
                    .iter()
                    .map(|&(start, end, db)| segment(step, (start, end), (db, db)))
                    .collect::<Vec<_>>()
            };
            let table = pieces(&[(0, 6_000, -5.0), (6_000, SOURCE_FRAMES, -3.0)]);
            let fresh_table = pieces(&[
                (0, free, -3.0),
                (free, 6_000, -5.0),
                (6_000, SOURCE_FRAMES, -3.0),
            ]);
            let reference = if later.is_empty() {
                fresh_table
            } else {
                table.clone()
            };
            rebuilt_table_case(hand, heard, (&table, &reference), later, from);
        }

        /// [`moved_target_case`] with the rebuild's `table` and the fresh plan's `reference`
        /// table (both track 0's ride segments) on [`flat`]'s ride.
        fn rebuilt_table_case(
            hand: bool,
            heard: usize,
            (table, reference): (&[session::AutomationSegment], &[session::AutomationSegment]),
            later: &[(u64, f32, u32)],
            from: u64,
        ) {
            const MOVE: u32 = 4_000;
            bench_support::alloc::assert_installed();
            let document = flat();
            let track = first_track(&document);
            let label = &format!("hand {hand}, heard {heard}, {} later moves", later.len());
            let ride = |segments: &[session::AutomationSegment], offset| {
                let mut automation = entry(&track, segments.to_vec());
                automation.offset = offset;
                automation
            };
            let move_of = |document: &str, db: f32, ramp: u32| {
                if hand {
                    hand_move(document, db, Some(ramp))
                } else {
                    vca_move(db, Some(ramp))
                }
            };
            let mut a = Rig::new(&document);
            let mut model = parse_session_json(&document).expect("model");
            model.automation[0] = ride(reference, session::AutomationOffset::default());
            let fresh = session::canonical_session_json(&model).expect("canonical");
            let mut r = Rig::new(&fresh);
            for rig in [&mut a, &mut r] {
                for _ in 0..4 {
                    rig.step();
                }
            }
            send(
                &mut a,
                &[move_of(&document, -6.0, MOVE)],
                protocol::EditPath::Live,
                label,
            );
            send(
                &mut r,
                &[move_of(&fresh, -6.0, MOVE)],
                protocol::EditPath::Live,
                label,
            );
            let (mut x, mut y) = (Vec::new(), Vec::new());
            for _ in 0..heard {
                x.push(a.step());
                y.push(r.step());
            }
            let offset = a.model().automation[0].offset;
            send(
                &mut a,
                &[SessionEdit::UpsertAutomation {
                    automation: ride(table, offset),
                }],
                protocol::EditPath::Rebuild,
                label,
            );
            for _ in 0..44 {
                for &(block, db, ramp) in later {
                    if a.block == block {
                        send(
                            &mut a,
                            &[move_of(&document, db, ramp)],
                            protocol::EditPath::Live,
                            label,
                        );
                        send(
                            &mut r,
                            &[move_of(&fresh, db, ramp)],
                            protocol::EditPath::Live,
                            label,
                        );
                    }
                }
                let block = a.block;
                let (out, used) = a.step_counted();
                assert_eq!(
                    (used.allocations, used.deallocations),
                    (0, 0),
                    "{label}: block {block} allocated or freed"
                );
                x.push(out);
                y.push(r.step());
            }
            assert_eq!(a.summary().3, 0, "{label}: the successor is adopted");
            assert_exact(&a, label);
            let quantum = a.quantum as u64;
            let first = x.iter().zip(&y).enumerate().find_map(|(index, (x, y))| {
                let start = (4 + index as u64) * quantum;
                x.iter().zip(y).enumerate().find_map(|(i, (p, q))| {
                    let sample = start + i as u64 % quantum;
                    (sample >= from && p.to_bits() != q.to_bits()).then_some((sample, i))
                })
            });
            assert_eq!(first, None, "{label}: first (sample, index) that differs");
        }

        /// #1711 G2 on the C ABI (A3). A VCA or hand move heard 12 and 30 blocks (2,464 and 160
        /// samples left; `J` = 960) whose target a table-only rebuild moves: the cell jumps over
        /// `J` at the move's first free sample (4,512), so every sample is bit-equal to a fresh
        /// plan whose table has that jump as a real discontinuity there. Case (c): a second live
        /// move to -9 dB over 1,200 samples from block 35 (before 4,512, ending after
        /// `4,512 + J`) clears the deferral; from its last sample on every sample is bit-equal to
        /// a fresh plan with the rebuild's table given both moves.
        ///
        /// Test value: red if the adoption lands a moved target at the swap instead of deferring
        /// its jump (the previous landing over `max(J, remaining)`), or if the grid is not held
        /// until the deferral fires.
        #[test]
        fn a_moved_target_jumps_at_a_live_moves_first_free_sample() {
            for hand in [false, true] {
                for heard in [12, 30] {
                    moved_target_case(hand, heard, &[], 0);
                    moved_target_case(hand, heard, &[(35, -9.0, 1_200)], 35 * 128 + 1_199);
                }
            }
        }

        /// #1711 U1 fold 1 (MAJOR-1) on the C ABI. A VCA or hand move heard 1 and 4 blocks on
        /// the ride flat at -3 dB, then a table-only rebuild to -5 dB up to 2,000, a linear -8 to
        /// -14 dB over 2,000 to 8,000, then -14 dB: the move's target moves, so its jump is
        /// deferred to the move's first free sample (4,512), and the table's step at 2,000, inside
        /// the move's remaining ramp, replaces the ramp the deferral waits for. Every sample is
        /// bit-equal to a fresh plan whose table is -3 dB before 2,000 (the same after it), given
        /// the same move; every revision completes as `OUTCOME_EXACT`.
        ///
        /// Test value: red if a discontinuity jump on a restartable cell leaves the pending
        /// deferral in place (its grid hold freezes the lane on the step's target until 4,512).
        #[test]
        fn a_table_step_inside_a_deferred_moves_ramp_drops_the_deferral() {
            let stepped = |first: f32| {
                let (step, lin) = (
                    session::AutomationShape::Step,
                    session::AutomationShape::Linear,
                );
                vec![
                    segment(step, (0, 2_000), (first, first)),
                    segment(lin, (2_000, 8_000), (-8.0, -14.0)),
                    segment(step, (8_000, SOURCE_FRAMES), (-14.0, -14.0)),
                ]
            };
            for hand in [false, true] {
                for heard in [1, 4] {
                    rebuilt_table_case(hand, heard, (&stepped(-5.0), &stepped(-3.0)), &[], 0);
                }
            }
        }

        /// The outcome flags and the transition-fallback count of `rig`'s watermark.
        fn outcome(rig: &Rig) -> (u64, u64) {
            let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
            assert_eq!(result, crate::RESULT_OK);
            (watermark.outcome_flags, watermark.transition_fallback_count)
        }

        /// The watermark covers `rig`'s newest committed revision, its last advance reports
        /// `OUTCOME_EXACT` alone, and no revision of the rig completed as the transition fallback.
        fn assert_exact(rig: &Rig, label: &str) {
            let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
            assert_eq!(result, crate::RESULT_OK);
            assert_eq!(
                watermark.revision,
                rig.summary().0,
                "{label}: the watermark covers the newest revision"
            );
            assert_eq!(
                outcome(rig),
                (crate::abi::OUTCOME_EXACT, 0),
                "{label}: the outcome flags and the transition-fallback count"
            );
        }

        /// U3 fold 2, MINOR (#1648 D2, D15-2). Two live VCA moves around a rebuild, with no block
        /// between any of them: a move pending at a rebuild whose own transaction moves `v`
        /// again, and a move, a rebuild, then a second live move. The first move is replaced
        /// unheard in both, as two writes before one uninterrupted block are, so
        /// `LIVE_VALUES_SUPERSEDED` equals the uninterrupted engine's, and every block does too.
        ///
        /// Test value: red if the successor's first block, applying a record of its own over the
        /// carried one, does not count the carried record it replaces unheard.
        #[test]
        fn a_carried_move_replaced_unheard_counts_as_superseded() {
            bench_support::alloc::assert_installed();
            for document in [riding(), flat()] {
                let model = parse_session_json(&document).expect("model");
                let rebuild = add_muted_track(&model, "added");
                for (label, inside) in [
                    ("a rebuild that moves again", true),
                    ("a move, a rebuild, a move", false),
                ] {
                    let mut rigs = warm(&document);
                    let (first_move, second) = (vca_move(-6.0, None), vca_move(-3.0, Some(4_000)));
                    for rig in &mut rigs[..2] {
                        send(
                            rig,
                            core::slice::from_ref(&first_move),
                            protocol::EditPath::Live,
                            label,
                        );
                    }
                    if inside {
                        let mut edits = vec![second.clone()];
                        edits.extend(rebuild.iter().cloned());
                        send(&mut rigs[0], &edits, protocol::EditPath::Rebuild, label);
                    } else {
                        send(&mut rigs[0], &rebuild, protocol::EditPath::Rebuild, label);
                        send(
                            &mut rigs[0],
                            core::slice::from_ref(&second),
                            protocol::EditPath::Live,
                            label,
                        );
                    }
                    send(
                        &mut rigs[1],
                        core::slice::from_ref(&second),
                        protocol::EditPath::Live,
                        label,
                    );
                    let (first, heard) = lockstep(&mut rigs, 44);
                    assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                    assert_eq!(first, None, "{label}: first difference (block, sample)");
                    assert!(heard, "{label}: the moves are heard");
                    let counts = [superseded(&mut rigs[0]), superseded(&mut rigs[1])];
                    assert_eq!(
                        counts[0], counts[1],
                        "{label}: LIVE_VALUES_SUPERSEDED [a, u]"
                    );
                }
            }
        }

        /// U3 fold, MAJOR 2 (R1). A live VCA move heard (mid-jump after three blocks, settled
        /// after twelve), then a rebuild that keeps the ride: the carried lane continues with no
        /// adoption jump, and every block equals the uninterrupted engine's.
        ///
        /// Test value: red if adoption compares the successor's `S` with anything but the `S`
        /// the stage heard (a stale `S` makes a jump the uninterrupted engine never makes), or
        /// the carried state does not continue.
        #[test]
        fn a_heard_vca_move_then_a_rebuild_continues_the_ride() {
            bench_support::alloc::assert_installed();
            let document = riding();
            let model = parse_session_json(&document).expect("model");
            for heard_blocks in [3, 12] {
                let label = format!("rebuilt after {heard_blocks} blocks");
                let mut rigs = warm(&document);
                for rig in &mut rigs[..2] {
                    send(
                        rig,
                        &[vca_move(-6.0, None)],
                        protocol::EditPath::Live,
                        &label,
                    );
                }
                let (first, _) = lockstep(&mut rigs, heard_blocks);
                assert_eq!(first, None, "{label}: before the rebuild");
                send(
                    &mut rigs[0],
                    &add_muted_track(&model, "added"),
                    protocol::EditPath::Rebuild,
                    &label,
                );
                let (first, heard) = lockstep(&mut rigs, 30);
                assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the move is heard");
            }
        }

        /// U3 fold, MAJOR 2 (R6, R7). The batch 13b held-ramp obligation owed at a swap: the
        /// muted first track steps from 0 dB to -9 dB at sample 3,000 inside an unmute ramp, so
        /// the step is only remembered and the lane owes a move where the ramp ends. A rebuild
        /// lands while the obligation is owed: inside an 8,000-sample ramp, and when a
        /// 3,072-sample ramp ends on the last sample before the swap (the move is due at the
        /// swap block's first sample). Every block equals the uninterrupted engine's.
        ///
        /// Test value: red if the obligation (owed, due at entry) is not carried across the swap:
        /// the successor's lane then stays at the gain the unmute aimed for.
        #[test]
        fn a_held_ramp_obligation_owed_at_a_swap_is_carried() {
            bench_support::alloc::assert_installed();
            let document = session_with(|model, first, _| {
                model.tracks[0].fader.left_mute = true;
                model.tracks[0].fader.right_mute = true;
                let step = |(start, end), db| {
                    segment(session::AutomationShape::Step, (start, end), (db, db))
                };
                model.automation.push(entry(
                    first,
                    vec![step((0, 3_000), 0.0), step((3_000, SOURCE_FRAMES), -9.0)],
                ));
            });
            let model = parse_session_json(&document).expect("model");
            let track = first_track(&document);
            let quantum = Rig::new(&document).quantum;
            // The unmute starts at block 4's first sample.
            for (label, ramp, swap) in [
                ("owed inside the ramp", 8_000, 26_u64),
                (
                    "due at the swap's first sample",
                    (28 - 4) * quantum as u32,
                    28,
                ),
            ] {
                let mut rigs = warm(&document);
                let unmute = [set(builtin(&track, MUTE, Lane::Both), 0.0, Some(ramp))];
                for rig in &mut rigs[..2] {
                    send(rig, &unmute, protocol::EditPath::Live, label);
                }
                let (first, _) = lockstep(&mut rigs, (swap - 4) as usize);
                assert_eq!(first, None, "{label}: before the swap");
                send(
                    &mut rigs[0],
                    &add_muted_track(&model, "added"),
                    protocol::EditPath::Rebuild,
                    label,
                );
                let (first, heard) = lockstep(&mut rigs, 60);
                assert_eq!(rigs[0].summary().3, 0, "{label}: the successor is adopted");
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the unmute is heard");
            }
        }

        /// U3 fold, MINOR 8 (#1648 D2, the reposition's seeding). A live fader move to -6 dB
        /// (the session's 960 samples) and a rebuild that adds a ride on the same track commit in
        /// one gap between blocks. The ride holds 0 dB until sample 100 and -6 dB after, so the
        /// gained cell is seeded with 0 dB while its curve at the swap equals the gain the stage
        /// is ramping to: the cell takes that value as its target and the stage finishes its own
        /// ramp, as the plain fader does with no ride. Every block equals the rig that only moved
        /// the fader.
        ///
        /// Test value: red if a repositioned cell whose curve equals the stage's target keeps its
        /// seeded target (its first grid event would retarget the stage over 64 samples and cut
        /// the fader's ramp short).
        #[test]
        fn a_gained_ride_on_a_moving_fader_lets_the_fader_finish_its_ramp() {
            bench_support::alloc::assert_installed();
            let document = session_with(|_, _, _| {});
            let mut model = parse_session_json(&document).expect("model");
            let track = first_track(&document);
            let step =
                |(start, end), db| segment(session::AutomationShape::Step, (start, end), (db, db));
            model.automation.push(entry(
                &track,
                vec![step((0, 100), 0.0), step((100, SOURCE_FRAMES), -6.0)],
            ));
            let ride = SessionEdit::UpsertAutomation {
                automation: model.automation[0].clone(),
            };
            let mut rigs = warm(&document);
            let fader = [
                set(builtin(&track, FADER_DB, Lane::Left), -6.0, None),
                set(builtin(&track, FADER_DB, Lane::Right), -6.0, None),
            ];
            for rig in &mut rigs[..2] {
                send(rig, &fader, protocol::EditPath::Live, "the fader move");
            }
            send(
                &mut rigs[0],
                &[ride],
                protocol::EditPath::Rebuild,
                "the ride",
            );
            let (first, heard) = lockstep(&mut rigs, 20);
            assert_eq!(rigs[0].summary().3, 0, "the successor is adopted");
            assert_eq!(first, None, "first difference (block, sample)");
            assert!(heard, "the fader move is heard");
        }

        /// U3 fold, MAJOR 3 (#1649 D3's ramp word). Twin A holds a flat step ride at -7.25 dB
        /// under VCA `v`; twin B holds a static -7.25 dB fader under `v`. They render the same
        /// bits, and a move of `v` on both (with the session's length and with its own 256
        /// samples) ramps the automated lane exactly as the plain one: equal bits on every block
        /// from the move on, with no settle window.
        ///
        /// Test value: red if the offsets cell's ramp word is lost or replaced (a `0` word sets
        /// the gain at once, a click; another length ramps differently), which a comparison
        /// after the jump cannot see.
        #[test]
        fn a_flat_ride_under_a_vca_move_ramps_as_a_static_fader() {
            bench_support::alloc::assert_installed();
            let ride = session_with(|model, first, _| {
                model.automation.push(entry(
                    first,
                    vec![segment(
                        session::AutomationShape::Step,
                        (0, SOURCE_FRAMES),
                        (-7.25, -7.25),
                    )],
                ));
                model.vcas.push(vca_of("v", &[first]));
            });
            let fixed = session_with(|model, first, _| {
                model.tracks[0].fader.left_db = -7.25;
                model.tracks[0].fader.right_db = -7.25;
                model.vcas.push(vca_of("v", &[first]));
            });
            for (label, ramp) in [("the session's length", None), ("its own 256", Some(256))] {
                let mut rigs = [Rig::new(&ride), Rig::new(&fixed), Rig::new(&fixed)];
                let (first, _) = lockstep(&mut rigs, 4);
                assert_eq!(first, None, "{label}: before the move");
                for rig in &mut rigs[..2] {
                    send(
                        rig,
                        &[vca_move(-6.0, ramp)],
                        protocol::EditPath::Live,
                        label,
                    );
                }
                let (first, heard) = lockstep(&mut rigs, 20);
                assert_eq!(first, None, "{label}: first difference (block, sample)");
                assert!(heard, "{label}: the move is heard");
            }
        }
    }

    /// #1650b: a hand move on a fader that stored automation drives, on the C ABI.
    mod automated_hand_move {
        use super::automated_vca::ride;
        use super::*;

        /// #1650b gate 3. A flat ride at -7.25 dB with offset `0` renders the bits of a static
        /// -7.25 dB fader (#1646 gate 1's twin), with no VCA and under a reaching VCA at -2.5 dB;
        /// so does an offset of `-0.0`.
        ///
        /// Test value: red if the offset term changes the gain bits of a zero offset (a `+0.0`
        /// seed that moves `S` off the VCA sum, or a `-0.0` that does).
        #[test]
        fn a_zero_offset_renders_the_static_fader() {
            let flat = |offset: f32, vca: Option<f32>| {
                with_model(&long_session(10, 48_000), |model| {
                    let mut entry = ride();
                    entry.segments[0].shape = session::AutomationShape::Step;
                    entry.segments[0].start_value = -7.25;
                    entry.segments[0].end_value = -7.25;
                    entry.offset = session::AutomationOffset {
                        left_db: offset,
                        right_db: offset,
                    };
                    model.automation.push(entry);
                    if let Some(db) = vca {
                        let mut v = vca_of("v", &["eq0"]);
                        v.fader.left_db = db;
                        v.fader.right_db = db;
                        model.vcas.push(v);
                    }
                })
            };
            let fixed = |vca: Option<f32>| {
                with_model(&long_session(10, 48_000), |model| {
                    model.tracks[0].fader.left_db = -7.25;
                    model.tracks[0].fader.right_db = -7.25;
                    if let Some(db) = vca {
                        let mut v = vca_of("v", &["eq0"]);
                        v.fader.left_db = db;
                        v.fader.right_db = db;
                        model.vcas.push(v);
                    }
                })
            };
            for vca in [None, Some(-2.5)] {
                let reference = Reference::run(&fixed(vca), 24);
                for offset in [0.0, -0.0] {
                    let rendered = Reference::run(&flat(offset, vca), 24);
                    assert!(
                        rendered.iter().flatten().any(|sample| *sample != 0.0),
                        "signal"
                    );
                    for (block, (left, right)) in rendered.iter().zip(&reference).enumerate() {
                        assert_eq!(
                            bits(left),
                            bits(right),
                            "VCA {vca:?}, offset {offset:?}: block {block}"
                        );
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// #1613: a live-only transaction committed in place
// ---------------------------------------------------------------------------------------------

/// A small deterministic generator (xorshift64*).
struct Draw(u64);

impl Draw {
    fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as usize % bound
    }

    fn unit(&mut self) -> f32 {
        self.next(1 << 20) as f32 / (1 << 20) as f32
    }
}

/// A value for `row` that its domain may accept: a toggle, a choice, the disabled value, or a point
/// of the range.
fn drawn_value(draw: &mut Draw, row: &protocol::ControlRow) -> f32 {
    match row.kind {
        protocol::ControlKind::Toggle => draw.next(2) as f32,
        protocol::ControlKind::Enumeration => row.choices[draw.next(row.choices.len())].value,
        protocol::ControlKind::Continuous => {
            if let Some(disabled) = row.disabled_value
                && draw.next(6) == 0
            {
                return disabled;
            }
            let value = row.minimum + (row.maximum - row.minimum) * draw.unit();
            if row.unit == protocol::ParameterUnit::Samples {
                value.floor()
            } else {
                value
            }
        }
    }
}

/// What a command leaves behind: its result and response, then every event of both lanes.
fn command_and_events(rig: &Rig, bytes: &[u8]) -> (u32, Vec<u8>, Vec<Vec<u8>>) {
    let (result, response) = command_c(rig.session, bytes);
    let mut events = Vec::new();
    for lane in [crate::EVENT_LANE_RELIABLE, crate::EVENT_LANE_LOSSY] {
        loop {
            let (code, frame) = event_c(rig.session, lane);
            assert_eq!(code, crate::RESULT_OK);
            if frame.is_empty() {
                break;
            }
            events.push(frame);
        }
    }
    (result, response, events)
}

/// The snapshot, the control list with its epoch and revision, and the control values.
type Observed = (
    String,
    (Vec<protocol::ControlEntry>, u64, u64),
    Vec<(u32, u32)>,
);

/// Everything a host can read of a session's model: the committed snapshot and the whole control
/// list and its values.
fn observed(rig: &mut Rig) -> Observed {
    let snapshot = rig.snapshot();
    let list = control_list_all_c(rig.session, &mut rig.request, 256);
    let state = control_state_all_c(rig.session, &mut rig.request, 256);
    (snapshot, list, state)
}

/// #1613 gate 1, through the C ABI. Two sessions of one document are given the same stream of
/// transactions of one to three edits drawn from the control list (every row of every class, the
/// solo rows, with and without a ramp): the first prepares each by the full path (a clone of the
/// model, compiled), the second in place. Every command gives the same result, the same response
/// bytes and the same events; the plans render the same samples; and the sessions read the same:
/// snapshot, control list with its epoch, control values, summary and watermark.
///
/// Red if the in-place commit differs anywhere a host can see, or if the stream did not take the
/// in-place form for the rows it exists for and the full one for the rest.
#[test]
fn in_place_commands_equal_full_commands_through_the_c_abi() {
    let document = control_list_session();
    let mut full = Rig::new(&document);
    let mut fast = Rig::new(&document);
    full.step();
    fast.step();
    let (list, ..) = control_list_all_c(fast.session, &mut fast.request, 256);
    let rows: Vec<(&protocol::ControlEntry, &protocol::ControlRow)> = list
        .iter()
        .filter_map(|entry| control_of(entry).map(|row| (entry, row)))
        .filter(|(_, row)| row.class != protocol::ControlClass::Derived && row.active)
        .collect();
    assert!(
        rows.len() > 300,
        "the session lists its controls: {}",
        rows.len()
    );
    let mut draw = Draw(0xC0FF_EE15_BADC_0DED);
    control_plane::test_take_in_place_counts();
    let (mut in_place, mut materialized, mut refused) = (0_usize, 0_usize, 0_usize);
    for step in 0..500_u64 {
        let mut edits = Vec::new();
        for _ in 0..1 + draw.next(3) {
            let (entry, row) = rows[draw.next(rows.len())];
            let value = drawn_value(&mut draw, row);
            let ramp =
                (row.ramp_row.is_some() && draw.next(3) == 0).then(|| [0, 64, 480][draw.next(3)]);
            edits.push(SessionEdit::SetControl {
                expected_effect: None,
                address: protocol::control_core::ControlAddress::try_from(&entry.address)
                    .expect("a listed address"),
                value,
                ramp_samples: ramp,
            });
        }
        // One request ID for both sessions: their replay caches then see the same stream.
        let request = full.request.max(fast.request);
        (full.request, fast.request) = (request, request);
        let bytes = full.transaction(&edits);
        fast.request = full.request;
        control_plane::test_force_full_preparation(true);
        let expected = command_and_events(&full, &bytes);
        control_plane::test_force_full_preparation(false);
        control_plane::test_take_in_place_counts();
        let actual = command_and_events(&fast, &bytes);
        let (commits, falls) = control_plane::test_take_in_place_counts();
        in_place += commits;
        materialized += falls;
        refused += usize::from(expected.0 != crate::RESULT_OK);
        assert_eq!(actual, expected, "step {step}: {edits:?}");
        if step % 40 == 0 && expected.0 == crate::RESULT_OK {
            // An exact retry of a committed command returns its recorded response and commits
            // nothing, on either path (the replay cache the commit completed, D15-17).
            control_plane::test_force_full_preparation(true);
            let replayed_full = command_and_events(&full, &bytes);
            control_plane::test_force_full_preparation(false);
            let replayed_fast = command_and_events(&fast, &bytes);
            assert_eq!(replayed_fast, replayed_full, "step {step}: the replay");
            assert_eq!(
                replayed_fast.1, expected.1,
                "step {step}: the recorded response"
            );
            assert!(
                replayed_fast.2.is_empty(),
                "step {step}: a replay emits no event"
            );
        }
        // The plans render alike while the stream goes on (the source starts over when it ends).
        if fast.fed + 2 * fast.quantum as u64 > SOURCE_FRAMES {
            fast.seek_to_start();
            full.seek_to_start();
        }
        assert_eq!(
            bits(&fast.step()),
            bits(&full.step()),
            "step {step}: the rendered block"
        );
        if step % 25 == 0 {
            assert_eq!(observed(&mut fast), observed(&mut full), "step {step}");
            // The whole watermark, its counters included: a drawn edit that restarts a strip is a
            // duck-swap adopted at its `S` (#1324a), so the next step's edit may supersede it,
            // alike on both paths.
            assert_eq!(
                watermark_words(&fast),
                watermark_words(&full),
                "step {step}"
            );
            assert_eq!(
                watermark_words(&fast).4,
                0,
                "step {step}: no transition fallback"
            );
            assert_eq!(fast.summary(), full.summary(), "step {step}");
        }
    }
    assert_eq!(observed(&mut fast), observed(&mut full), "the end");
    println!("in place {in_place}, materialized {materialized}, refused {refused}");
    assert!(in_place > 120, "commits made in place: {in_place}");
    assert!(
        materialized > 5,
        "rows the fast classification hands over: {materialized}"
    );
    assert!(refused > 3, "refusals alike: {refused}");
}

/// A session of `tracks` tracks: the control-list session's submix, sends, VCA, console slot and
/// inserts, with plain copies of its second track. Its VCA and sends do not grow with `tracks`.
fn grown_session(tracks: usize) -> String {
    with_model(&control_list_session(), |model| {
        let template = model
            .tracks
            .iter()
            .find(|track| track.id.as_str() == "eq1")
            .expect("eq1")
            .clone();
        let route = model
            .routes
            .iter()
            .find(|route| route.id.as_str() == "eq1-main")
            .expect("eq1-main")
            .clone();
        for index in 0..tracks.saturating_sub(model.tracks.len()) {
            let name = format!("g{index:04}");
            let mut track = template.clone();
            track.id = StableId::parse(&name).expect("track ID");
            let mut main = route.clone();
            main.id = StableId::parse(&format!("{name}-main")).expect("route ID");
            if let RouteSource::Track { track_id, .. } = &mut main.source {
                *track_id = track.id.clone();
            }
            model.tracks.push(track);
            model.routes.push(main);
        }
    })
}

/// The limits of a session of a few hundred tracks.
fn grown_limits() -> CompileLimits {
    let mut raised = limits();
    raised.maximum_document_bytes = 50_000_000;
    raised.maximum_tracks = 1_000;
    raised.maximum_routes = 2_000;
    raised.maximum_effects = 20_000;
    raised.maximum_submixes = 8;
    raised.maximum_vcas = 8;
    raised.maximum_capi_retained_bytes = 400_000_000;
    raised.maximum_graph_session_plus_plan_bytes = 4_000_000_000;
    raised.maximum_named_allocation_bytes = 400_000_000;
    raised
}

fn strip_row(
    strip: &str,
    section: protocol::control_core::Section,
    control: u32,
    lane: protocol::control_core::Lane,
) -> protocol::control_core::ControlAddress {
    protocol::control_core::ControlAddress {
        owner: protocol::control_core::Owner::Strip(StableId::parse(strip).expect("strip")),
        section,
        control,
        lane,
    }
}

/// #1613 gate 2. The allocations and frees of one whole `SESSION_TRANSACTION_APPLY` that edits live
/// values, through the C ABI's control entry, are the same at 64 and 256 tracks, for each kind of
/// live row: and the commit was made in place. With the full preparation forced, the count at 256
/// tracks is larger than at 64, so the gate sees what it guards.
///
/// Red if a live-only commit clones, compiles, lists or walks the session: one allocation per
/// strip, route or VCA of the session moves the count with the track count.
#[test]
fn the_cost_of_a_live_only_command_does_not_grow_with_the_session() {
    use protocol::control_core::{ControlAddress, Lane, Owner, Section};
    bench_support::alloc::assert_installed();
    // The debug-build check that a values-only stage equals a full build builds the whole catalog.
    control_plane::test_verify_catalog_stages(false);
    let sizes = [64_usize, 256];
    let mut rigs: Vec<Rig> = sizes
        .iter()
        .map(|&tracks| Rig::with_limits(&grown_session(tracks), grown_limits()))
        .collect();
    for rig in &mut rigs {
        rig.step();
    }
    let template_is_matrix = {
        let model = rigs[0].model();
        matches!(
            model
                .tracks
                .iter()
                .find(|track| track.id.as_str() == "g0004")
                .expect("g0004")
                .matrix_or_pan,
            MatrixOrPan::Matrix { .. }
        )
    };
    fn both(strip: &str, control: u32) -> ControlAddress {
        strip_row(strip, Section::Builtins, control, Lane::Both)
    }
    let route = |id: &str, control: u32| ControlAddress {
        owner: Owner::Route(StableId::parse(id).expect("route")),
        section: Section::Own,
        control,
        lane: Lane::Both,
    };
    let vca = |id: &str, control: u32, lane: Lane| ControlAddress {
        owner: Owner::Vca(StableId::parse(id).expect("vca")),
        section: Section::Own,
        control,
        lane,
    };
    let shaped = move |value: f32| -> Vec<SessionEdit> {
        if template_is_matrix {
            (7..11)
                .map(|control| SessionEdit::set_control(both("g0004", control), value * 0.1, None))
                .collect()
        } else {
            [Lane::Left, Lane::Right]
                .into_iter()
                .map(|lane| {
                    SessionEdit::set_control(
                        strip_row("g0004", Section::Builtins, 12, lane),
                        value * 0.1,
                        None,
                    )
                })
                .collect()
        }
    };
    type Kind = (&'static str, Box<dyn Fn(f32) -> Vec<SessionEdit>>);
    let kinds: Vec<Kind> = vec![
        (
            "fader",
            Box::new(move |value| vec![SessionEdit::set_control(both("g0007", 5), -value, None)]),
        ),
        (
            "mute",
            Box::new(move |value| {
                vec![SessionEdit::set_control(
                    both("g0006", 6),
                    (value > 0.5) as u8 as f32,
                    None,
                )]
            }),
        ),
        ("pan or matrix", Box::new(shaped)),
        (
            "trim",
            Box::new(move |value| vec![SessionEdit::set_control(both("g0005", 2), -value, None)]),
        ),
        (
            "effect parameter",
            Box::new(move |value| {
                vec![SessionEdit::set_control(
                    strip_row(
                        "g0003",
                        Section::Inserts(StableId::parse("comp").expect("comp")),
                        1,
                        Lane::Both,
                    ),
                    -10.0 - value,
                    None,
                )]
            }),
        ),
        (
            "send gain",
            Box::new(move |value| {
                vec![SessionEdit::set_control(route("eq0-bus", 1), -value, None)]
            }),
        ),
        (
            "send mute",
            Box::new(move |value| {
                vec![SessionEdit::set_control(
                    route("eq0-bus", 2),
                    (value > 0.5) as u8 as f32,
                    None,
                )]
            }),
        ),
        (
            "VCA offset",
            Box::new(move |value| {
                vec![SessionEdit::set_control(
                    vca("vca", 1, Lane::Both),
                    -value,
                    None,
                )]
            }),
        ),
        (
            "VCA mute",
            Box::new(move |value| {
                vec![SessionEdit::set_control(
                    vca("vca", 2, Lane::Left),
                    (value > 0.5) as u8 as f32,
                    None,
                )]
            }),
        ),
    ];
    // Each call is made twice before it is counted: a first write of a parameter adds its entry,
    // which is the full path's, and the first call of the process warms its statics.
    let measure = |rig: &mut Rig, edits: Vec<SessionEdit>| -> (u64, u64, (usize, usize)) {
        let bytes = rig.transaction(&edits);
        control_plane::test_take_in_place_counts();
        let mark = bench_support::alloc::current_thread_counters();
        let (result, _) = command_c(rig.session, &bytes);
        let used = bench_support::alloc::current_thread_delta_since(mark);
        assert_eq!(result, crate::RESULT_OK);
        rig.drain_reliable();
        (
            used.allocations,
            used.deallocations,
            control_plane::test_take_in_place_counts(),
        )
    };
    for (name, edits) in &kinds {
        let mut counts = Vec::new();
        for rig in &mut rigs {
            measure(rig, edits(2.0));
            measure(rig, edits(3.0));
            counts.push(measure(rig, edits(4.0)));
        }
        println!("{name}: {counts:?}");
        assert!(
            counts.iter().all(|count| count.2 == (1, 0)),
            "{name}: in place {counts:?}"
        );
        assert!(counts[0].0 > 0, "{name}: the call is counted");
        assert!(
            counts
                .iter()
                .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
            "{name}: the allocations depend on the track count {sizes:?}: {counts:?}"
        );
    }
    // The solo of a strip while another is soloed moves one strip.
    let solo = |strip: &str, on: bool| {
        vec![SessionEdit::set_control(
            strip_row(strip, Section::Own, 2, Lane::Both),
            f32::from(u8::from(on)),
            None,
        )]
    };
    let mut counts = Vec::new();
    for rig in &mut rigs {
        measure(rig, solo("g0001", true));
        measure(rig, solo("g0002", true));
        measure(rig, solo("g0002", false));
        counts.push(measure(rig, solo("g0002", true)));
    }
    println!("solo: {counts:?}");
    assert!(
        counts.iter().all(|count| count.2 == (1, 0)),
        "solo in place {counts:?}"
    );
    assert!(
        counts
            .iter()
            .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
        "solo: the allocations depend on the track count: {counts:?}"
    );
    // The same fader edit by the full preparation grows with the session.
    control_plane::test_force_full_preparation(true);
    let forced: Vec<u64> = rigs
        .iter_mut()
        .map(|rig| {
            measure(
                rig,
                vec![SessionEdit::set_control(both("g0007", 5), -6.0, None)],
            );
            measure(
                rig,
                vec![SessionEdit::set_control(both("g0007", 5), -7.0, None)],
            )
            .0
        })
        .collect();
    control_plane::test_force_full_preparation(false);
    println!("full preparation, fader: {forced:?} allocations at {sizes:?} tracks");
    assert!(
        forced[1] > forced[0],
        "the full preparation grows with the session: {forced:?}"
    );
}

/// `document` with one stored fader ride on its first track (builtin 5, both lanes).
fn with_fader_automation(document: &str) -> String {
    with_model(document, |model| {
        let target = session::ControlAddress {
            owner: session::Owner::Strip(model.tracks[0].id.clone()),
            section: session::Section::Builtins,
            control: 5,
            lane: session::ParameterChannel::Both,
        };
        model.automation.push(session::Automation {
            id: StableId::parse("fader-ride").expect("automation ID"),
            target,
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Linear,
                start_sample: 0,
                end_sample: 960,
                start_value: 0.0,
                end_value: -3.0,
                unit: session::ParameterUnit::Db,
            }],
            offset: session::AutomationOffset::default(),
        });
    })
}

/// #1639 gate 3. A playing session with a fader entry takes `SET_CONTROL`s of its three
/// `control_smoothing` rows: the response path is `live`, the provider epoch is unchanged with no
/// candidate, and the plan holds the new lengths from the next block, which allocates and frees
/// nothing. The same edit on a session without stored automation reports `model_only` and leaves
/// the prepared lengths held.
///
/// Test value: red if the commit does not write the cell (the next block still holds the prepared
/// lengths), or reports the path from anything but the classifier's jump-length value.
#[test]
fn a_control_smoothing_edit_on_an_automated_session_writes_the_jump_lengths_live() {
    bench_support::alloc::assert_installed();
    let fast = ControlSmoothing {
        mute_ms: 1.0,
        fader_ms: 2.0,
        pan_ms: 3.0,
    };
    // The default table at 48 kHz: fader 20 ms, mute 10 ms, pan 20 ms.
    let prepared = graph::GraphJumpLengths {
        fader_samples: 960,
        mute_samples: 480,
        pan_samples: 960,
    };
    let written = graph::GraphJumpLengths {
        fader_samples: 96,
        mute_samples: 48,
        pan_samples: 144,
    };
    let plain = long_session(10, 48_000);
    for (document, path, held) in [
        (
            with_fader_automation(&plain),
            protocol::EditPath::Live,
            written,
        ),
        (plain, protocol::EditPath::ModelOnly, prepared),
    ] {
        let mut rig = Rig::new(&document);
        for _ in 0..2 {
            rig.step();
        }
        assert_eq!(
            graph::test_only_block_jump_lengths(),
            Some(prepared),
            "a fresh plan holds the prepared lengths"
        );
        let (revision, _, epoch, _) = rig.summary();
        let bytes = rig.transaction(&smoothing_edits(fast));
        let (result, response) = rig.send(&bytes);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(edit_path_of(&response), path);
        let (after, _, after_epoch, pending) = rig.summary();
        assert_eq!(
            (after, after_epoch, pending),
            (revision + 1, epoch, 0),
            "{path:?}: one revision, the same provider epoch, no candidate"
        );
        let (_, used) = rig.step_counted();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "{path:?}: the block that reads the cell allocates and frees nothing"
        );
        assert_eq!(
            graph::test_only_block_jump_lengths(),
            Some(held),
            "{path:?}: the next block's lengths"
        );
        rig.step();
        assert_eq!(graph::test_only_block_jump_lengths(), Some(held));
    }
}

/// #1639 D4. One transaction adds a track (a structural edit, so a successor plan) and changes the
/// three `control_smoothing` rows: the path is `rebuild`, and once render adopts the successor
/// (the watermark covers the revision, the provider epoch moves) the plan holds the new lengths.
/// The session has no stored automation, so no live write reaches any plan's cell: only the
/// successor's seed can carry the new lengths.
///
/// Test value: red if the successor's cell is seeded from anything but the committed model that
/// holds the edit (the model before the transaction, or a constant), so a rebuild silently drops
/// a `control_smoothing` change that came with it.
#[test]
fn a_successor_prepared_with_a_control_smoothing_edit_holds_the_new_lengths() {
    let fast = ControlSmoothing {
        mute_ms: 1.0,
        fader_ms: 2.0,
        pan_ms: 3.0,
    };
    // The default table at 48 kHz: fader 20 ms, mute 10 ms, pan 20 ms.
    let prepared = graph::GraphJumpLengths {
        fader_samples: 960,
        mute_samples: 480,
        pan_samples: 960,
    };
    let written = graph::GraphJumpLengths {
        fader_samples: 96,
        mute_samples: 48,
        pan_samples: 144,
    };
    let document = long_session(10, 48_000);
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    for _ in 0..2 {
        rig.step();
    }
    assert_eq!(
        graph::test_only_block_jump_lengths(),
        Some(prepared),
        "a fresh plan holds the prepared lengths"
    );
    let (revision, _, epoch, _) = rig.summary();
    let mut edits = add_muted_track(&model, "added").to_vec();
    edits.extend(smoothing_edits(fast));
    let bytes = rig.transaction(&edits);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
    assert_eq!(
        rig.summary().3,
        1,
        "the structural edit prepares a candidate"
    );
    let mut guard = 0;
    while rig.summary().3 != 0 {
        rig.step();
        guard += 1;
        assert!(guard < 16, "the candidate is adopted");
    }
    let (after, _, after_epoch, _) = rig.summary();
    assert_eq!(after, revision + 1, "one revision");
    assert_ne!(after_epoch, epoch, "the successor renders");
    rig.step();
    assert_eq!(
        graph::test_only_block_jump_lengths(),
        Some(written),
        "the successor's cell holds the committed lengths"
    );
}

/// #1648b: the fader ride of gates 2-4, on the first track's both lanes: linear from `0` dB at
/// timeline 0 to `end_db` at 6,000, so it moves through every adoption block below.
fn ride_entry(track: &StableId, end_db: f32) -> session::Automation {
    session::Automation {
        id: StableId::parse("fader-ride").expect("automation ID"),
        target: session::ControlAddress {
            owner: session::Owner::Strip(track.clone()),
            section: session::Section::Builtins,
            control: 5,
            lane: session::ParameterChannel::Both,
        },
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: 6_000,
            start_value: 0.0,
            end_value: end_db,
            unit: session::ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// #1648b: a stateless session (no effect, no input filter, so a fresh plan's strips render a
/// playing plan's bits once their faders agree) with a long source and, when `end_db` is given,
/// [`ride_entry`] on its first track.
fn ride_document(end_db: Option<f32>) -> String {
    let mut model = stateless_session(48_000);
    model.sources[0].frames = SOURCE_FRAMES;
    if let Some(end_db) = end_db {
        let track = model.tracks[0].id.clone();
        model.automation.push(ride_entry(&track, end_db));
    }
    session::canonical_session_json(&model).expect("canonical ride session")
}

/// #1648b: send `edits` on `rig`, which must answer `rebuild`, and render until render adopts the
/// successor. The watermark then publishes the revision `EXACT` at the adoption block (#1314),
/// and the source was never reseeked. Returns the adoption block's first sample `T` and the
/// blocks rendered from it on, the adoption block first, `blocks` long.
fn rebuild_and_adopt(rig: &mut Rig, edits: &[SessionEdit], blocks: usize) -> (u64, Vec<Vec<f32>>) {
    rebuild_and_adopt_as(rig, edits, blocks, None)
}

/// [`rebuild_and_adopt`] whose revision completes with the outcome flags `outcome` (B19-R7);
/// `None` is `EXACT` with no transition fallback or supersession on the plan.
fn rebuild_and_adopt_as(
    rig: &mut Rig,
    edits: &[SessionEdit],
    blocks: usize,
    outcome: Option<u64>,
) -> (u64, Vec<Vec<f32>>) {
    let bytes = rig.transaction(edits);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&response), protocol::EditPath::Rebuild);
    let revision = rig.summary().0;
    let mut rendered = Vec::new();
    while rig.summary().3 != 0 {
        rendered.push((rig.block, rig.step()));
        assert!(rendered.len() < 16, "the candidate is adopted");
    }
    let (applied, first, flags) = if outcome.is_none() {
        let (applied, first, flags, _) = watermark(rig);
        (applied, first, flags)
    } else {
        let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
        assert_eq!(result, crate::RESULT_OK);
        (
            watermark.revision,
            watermark.first_sample,
            watermark.outcome_flags,
        )
    };
    assert_eq!(
        (applied, flags),
        (revision, outcome.unwrap_or(crate::OUTCOME_EXACT)),
        "the revision's outcome at adoption"
    );
    let adoption = first / rig.quantum as u64;
    let mut from: Vec<Vec<f32>> = rendered
        .into_iter()
        .filter(|(block, _)| *block >= adoption)
        .map(|(_, pcm)| pcm)
        .collect();
    assert!(!from.is_empty(), "the adoption block was rendered");
    while from.len() < blocks {
        from.push(rig.step());
    }
    let (result, report) = crate::ffi::test_source_seek_report(rig.session, SOURCE);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(report.generation, 1, "no source is reseeked");
    (first, from)
}

/// #1648b: a fresh plan of `document`, session-seeked to timeline sample `timeline` (#1642) and
/// fed the PCM from there: `blocks` blocks.
fn seeked_reference(document: &str, timeline: u64, blocks: usize) -> Vec<Vec<f32>> {
    let mut rig = Rig::new(document);
    rig.generation = 2;
    assert_eq!(
        crate::ffi::test_session_seek(rig.session, 2, timeline, None),
        crate::RESULT_OK
    );
    rig.fed = timeline;
    (0..blocks).map(|_| rig.step()).collect()
}

/// One plane of `blocks`, sample by sample, as bits.
fn plane_bits(blocks: &[Vec<f32>], plane: usize, quantum: usize) -> Vec<u32> {
    blocks
        .iter()
        .flat_map(|block| block[plane * quantum..(plane + 1) * quantum].iter())
        .map(|sample| sample.to_bits())
        .collect()
}

/// #1648b blocks: blocks rendered from the adoption block in gates 2 and 4.
const ADOPTED_BLOCKS: usize = 16;

/// #1648 gate 2. A playing session with a linear fader ride on track 0 takes an
/// `UpsertAutomation` that changes the ride: the path is `rebuild`, the watermark publishes the
/// revision `EXACT` at the adoption block `T`, and no source is reseeked. The carried lane jumps
/// to the new curve over the fader jump length `J` (960 samples, the default table at 48 kHz):
/// it still differs from a fresh plan of the new session, session-seeked to `T` and fed the same
/// PCM, in the jump's last block; and from the completion sample of the first grid ramp that
/// starts after the jump ends (`T + J + 63`), every sample equals that reference.
///
/// Test value: red if the cell does not carry (the lane restarts with a `Set`), its cursor is not
/// repositioned in the new table, or the adoption jump is missing or not `J` long.
#[test]
fn a_changed_fader_ride_rebuilds_and_carries_into_its_jump() {
    let (old, new) = (ride_document(Some(-12.0)), ride_document(Some(-24.0)));
    let mut rig = Rig::new(&old);
    for _ in 0..4 {
        rig.step();
    }
    let track = parse_session_json(&old).expect("model").tracks[0]
        .id
        .clone();
    let edit = SessionEdit::UpsertAutomation {
        automation: ride_entry(&track, -24.0),
    };
    let (adoption, live) = rebuild_and_adopt(&mut rig, &[edit], ADOPTED_BLOCKS);
    assert_eq!(
        rig.model().automation,
        parse_session_json(&new).expect("new model").automation
    );
    let reference = seeked_reference(&new, adoption, ADOPTED_BLOCKS);
    let jump = 960_usize;
    let settled = jump + 63;
    let quantum = rig.quantum;
    for plane in 0..2 {
        let (live, reference) = (
            plane_bits(&live, plane, quantum),
            plane_bits(&reference, plane, quantum),
        );
        assert_ne!(
            live[jump - quantum..jump],
            reference[jump - quantum..jump],
            "plane {plane}: the lane is still in its adoption jump"
        );
        assert_eq!(
            live[settled..],
            reference[settled..],
            "plane {plane}: the new curve's bits from the first grid ramp after the jump"
        );
    }
}

/// #1648 gate 3. A transaction that adds a muted track and keeps the ride rebuilds, and every
/// block from the swap on is bit-identical to the uninterrupted render of the old session.
///
/// Test value: red if adoption emits a jump to an unchanged target, or the carried cell restarts
/// or repositions (a moving curve's in-flight state would be dropped).
#[test]
fn an_unchanged_fader_ride_carries_with_no_jump() {
    let document = ride_document(Some(-12.0));
    let model = parse_session_json(&document).expect("model");
    let mut rig = Rig::new(&document);
    let mut uninterrupted = Rig::new(&document);
    for _ in 0..4 {
        assert_eq!(bits(&rig.step()), bits(&uninterrupted.step()), "warm-up");
    }
    let (adoption, live) = rebuild_and_adopt(&mut rig, &add_muted_track(&model, "added"), 12);
    let mut reference = Vec::new();
    while (uninterrupted.block as usize) * uninterrupted.quantum < adoption as usize {
        uninterrupted.step();
    }
    for _ in 0..live.len() {
        reference.push(uninterrupted.step());
    }
    for (index, (live, reference)) in live.iter().zip(&reference).enumerate() {
        assert_eq!(bits(live), bits(reference), "block {index} from the swap");
    }
}

/// #1648 gate 4. Removing the ride rebuilds; the released lane ramps from the curve's last target
/// to its static value over the fader jump length, and from the end of that release every sample
/// equals a fresh plan of the new session (a static fader), session-seeked to the adoption block.
///
/// Test value: red if a removed entry leaves the lane on its last automated value, or steps it.
#[test]
fn a_removed_fader_ride_releases_to_the_static_fader() {
    let (old, new) = (ride_document(Some(-12.0)), ride_document(None));
    let mut rig = Rig::new(&old);
    for _ in 0..8 {
        rig.step();
    }
    let edit = SessionEdit::RemoveAutomation {
        automation_id: StableId::parse("fader-ride").expect("automation ID"),
    };
    let (adoption, live) = rebuild_and_adopt(&mut rig, &[edit], ADOPTED_BLOCKS);
    let reference = seeked_reference(&new, adoption, ADOPTED_BLOCKS);
    let jump = 960_usize;
    let quantum = rig.quantum;
    for plane in 0..2 {
        let (live, reference) = (
            plane_bits(&live, plane, quantum),
            plane_bits(&reference, plane, quantum),
        );
        assert_ne!(
            live[jump - quantum..jump - 1],
            reference[jump - quantum..jump - 1],
            "plane {plane}: the lane is still in its release"
        );
        assert_eq!(
            live[jump - 1..],
            reference[jump - 1..],
            "plane {plane}: the static fader's bits from the release's last sample"
        );
    }
}

/// #1652 gate 1: track `eq0` sends `pre_fader` (its right lane, `rr` only) with `follows_mute`
/// into bus `b`, and its direct route takes only its left lane to the output, so the output's
/// left plane is the strip and its right plane the send. With `entry`, `eq0` has a mute entry on
/// `both` with steps 0, 1, 0 at 1,000 and 2,500; with `muted`, `eq0` is muted statically.
fn follow_document(entry: bool, muted: bool) -> String {
    use session::{
        Automation, AutomationOffset, AutomationSegment, AutomationShape, ChannelMatrix,
        ControlAddress, Owner, ParameterChannel, ParameterUnit, Section, SendTap,
    };
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    let mut model = stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.sources[0].frames = SOURCE_FRAMES;
    model.control_smoothing = Some(ControlSmoothing {
        mute_ms: 10.0,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    let track = model.tracks[0].id.clone();
    (
        model.tracks[0].fader.left_mute,
        model.tracks[0].fader.right_mute,
    ) = (muted, muted);
    let matrix = |ll: f32, rr: f32| ChannelMatrix {
        ll,
        lr: 0.0,
        rl: 0.0,
        rr,
    };
    model.routes[0].channel_matrix = matrix(1.0, 0.0);
    let mut send = model.routes[0].clone();
    send.id = id("eq0-b");
    send.source = RouteSource::Track {
        track_id: track.clone(),
        tap: SendTap::PreFader,
    };
    send.destination = RouteDestination::SubmixInput { submix_id: id("b") };
    send.channel_matrix = matrix(0.0, 1.0);
    send.follows_mute = true;
    let mut out = model.routes[0].clone();
    out.id = id("b-main");
    out.source = RouteSource::Submix {
        submix_id: id("b"),
        tap: SendTap::PostPan,
    };
    out.channel_matrix = matrix(1.0, 1.0);
    model.routes.push(send);
    model.routes.push(out);
    let console = model.console.clone();
    model.submixes.push(Submix::unity(id("b"), &console));
    if entry {
        let step = |start: u64, end: u64, from: f32, to: f32| AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: start,
            end_sample: end,
            start_value: from,
            end_value: to,
            unit: ParameterUnit::Linear,
        };
        model.automation.push(Automation {
            id: id("eq0-mute"),
            target: ControlAddress {
                owner: Owner::Strip(track),
                section: Section::Builtins,
                control: 6,
                lane: ParameterChannel::Both,
            },
            segments: vec![step(0, 1_000, 0.0, 1.0), step(1_000, 2_500, 1.0, 0.0)],
            offset: AutomationOffset::default(),
        });
    }
    session::canonical_session_json(&model).expect("canonical follow session")
}

/// #1653: sends `edits` on `rig`, which must answer `live`, and renders `blocks` blocks from the
/// next block on. Returns that block's first sample `T` (the first block that begins after the
/// submit returns, which applies the transaction's records) and the blocks, `T`'s first.
fn live_and_render(rig: &mut Rig, edits: &[SessionEdit], blocks: usize) -> (u64, Vec<Vec<f32>>) {
    let bytes = rig.transaction(edits);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(edit_path_of(&response), protocol::EditPath::Live);
    let first = rig.block * rig.quantum as u64;
    (first, (0..blocks).map(|_| rig.step()).collect())
}

/// #1653 (replaces #1652c's rebuild gate): a send gain edit on the following route `eq0-b`, and a
/// hand mute of its source `eq0` (which changes its terms and its gate), are each `live`, and from
/// the first block boundary after their ramps the C ABI renders what the committed document
/// renders fresh, session-seeked to the edit's block.
///
/// Test value: red if either edit still rebuilds, if the commit refuses the seven-word record
/// (an internal fault; the edit is lost), or if the record does not carry the committed values.
#[test]
fn an_edit_of_a_following_route_is_live() {
    use protocol::control_core::{Lane, Owner, Section, row_address};
    let gain = SessionEdit::set_control(
        row_address(
            Owner::Route(StableId::parse("eq0-b").expect("route ID")),
            Section::Own,
            "gain_db",
            Lane::Both,
        )
        .expect("the send gain row"),
        -9.0,
        None,
    );
    for edit in [gain, mute_edit("eq0", true)] {
        let mut rig = Rig::new(&follow_document(true, false));
        for _ in 0..4 {
            rig.step();
        }
        let (at, live) = live_and_render(&mut rig, std::slice::from_ref(&edit), 12);
        let reference = seeked_reference(&rig.snapshot(), at, 12);
        let settle = 4;
        assert_eq!(
            live[settle..]
                .iter()
                .map(|pcm| bits(pcm))
                .collect::<Vec<_>>(),
            reference[settle..]
                .iter()
                .map(|pcm| bits(pcm))
                .collect::<Vec<_>>(),
            "the plan renders the committed document once its ramps settle"
        );
    }
}

/// A flat curve at `value` over the source (step-shaped for a mute row).
fn send_flat(
    value: f32,
    unit: session::ParameterUnit,
    step: bool,
) -> Vec<session::AutomationSegment> {
    let shape = if step {
        session::AutomationShape::Step
    } else {
        session::AutomationShape::Linear
    };
    vec![session::AutomationSegment {
        shape,
        start_sample: 0,
        end_sample: SOURCE_FRAMES,
        start_value: value,
        end_value: value,
        unit,
    }]
}

/// The send `eq0-b`'s own row `name` (`gain_db`, `mute`, `matrix_lr`, ...) set to `value`, over
/// `ramp` samples when given.
fn send_row_edit(name: &str, value: f32, ramp: Option<u32>) -> SessionEdit {
    use protocol::control_core::{Lane, Owner, Section, row_address};
    SessionEdit::set_control(
        row_address(
            Owner::Route(StableId::parse("eq0-b").expect("route ID")),
            Section::Own,
            name,
            Lane::Both,
        )
        .expect("a send row"),
        value,
        ramp,
    )
}

/// A mute entry `name` on `eq0`'s lane `lane`, flat at `value` (held open at 0, muted at 1).
fn lane_mute_entry(name: &str, lane: session::ParameterChannel, value: f32) -> session::Automation {
    session::Automation {
        id: StableId::parse(name).expect("automation ID"),
        target: session::ControlAddress {
            owner: session::Owner::Strip(StableId::parse("eq0").expect("track ID")),
            section: session::Section::Builtins,
            control: 6,
            lane,
        },
        segments: send_flat(value, session::ParameterUnit::Linear, true),
        offset: session::AutomationOffset::default(),
    }
}

/// [`follow_document`]`(false, false)` whose send `eq0-b` follows a stored mute curve held open
/// (`eq0`'s mute entry flat at 0 on both lanes): a seven-word following send whose curves never
/// move, so a live send move on it is the only ramp in flight.
fn open_follow_document() -> String {
    with_model(&follow_document(false, false), |model| {
        model.automation.push(lane_mute_entry(
            "eq0-mute",
            session::ParameterChannel::Both,
            0.0,
        ));
    })
}

/// [`follow_document`]`(false, false)`'s direct route `eq0-main` at `db`: a rebuild that edits
/// only the direct route.
fn direct_route_edit(rig: &mut Rig, db: f32) -> SessionEdit {
    let mut route = rig
        .model()
        .routes
        .into_iter()
        .find(|route| route.id.as_str() == "eq0-main")
        .expect("the direct route");
    route.gain_db = db;
    SessionEdit::UpsertRoute { route }
}

/// The transition-fallback count of `rig`'s watermark.
fn fallback_count(rig: &Rig) -> u64 {
    let (result, watermark) = crate::ffi::test_plan_watermark(rig.plan);
    assert_eq!(result, crate::RESULT_OK);
    watermark.transition_fallback_count
}

/// #1654 root ruling B19-R8 (the split verifier's MAJOR-1) and R19-5 on the C ABI. On
/// [`open_follow_document`] (`eq0-b` follows a held-open mute curve), a live send move over
/// 4,000 samples (a D6 `gain_db` edit to -3 dB, or a `matrix_rr` edit to 0.5) is in flight when
/// a rebuild edits only the direct route `eq0-main`. The move keeps its own length: the send's
/// plane is the never-rebuilt twin's, bit for bit, through the move and after it. Control cannot
/// tell that, so the rebuild completes `TRANSITION_FALLBACK` (count 1, the conservative term).
/// With the move long finished the same rebuild completes `EXACT` (count 0).
///
/// Test value: red if an unrelated rebuild that carries a following send completes `EXACT` while
/// a live send move may still ramp, if it is flagged with no live send move in the window, or if
/// the carry cuts the move (R19-5).
#[test]
fn an_unrelated_rebuild_across_a_live_send_move_on_a_following_send_is_flagged_on_the_c_abi() {
    const RAMP: u32 = 4_000;
    let document = open_follow_document();
    let quantum = 128_usize;
    // (label, the live send move, its lead before the rebuild in blocks, flagged)
    for (label, edit, lead, flagged) in [
        (
            "gain_db in flight",
            send_row_edit("gain_db", -3.0, Some(RAMP)),
            2,
            true,
        ),
        (
            "matrix_rr in flight",
            send_row_edit("matrix_rr", 0.5, Some(RAMP)),
            2,
            true,
        ),
        (
            "the move long finished",
            send_row_edit("gain_db", -3.0, Some(RAMP)),
            40,
            false,
        ),
    ] {
        let mut rig = Rig::new(&document);
        let mut twin = Rig::new(&document);
        for _ in 0..4 {
            assert_eq!(bits(&rig.step()), bits(&twin.step()), "{label}");
        }
        let (start, _) = live_and_render(&mut rig, std::slice::from_ref(&edit), lead);
        let _ = live_and_render(&mut twin, std::slice::from_ref(&edit), lead);
        let now = rig.block * rig.quantum as u64;
        assert_eq!(
            now < start + u64::from(RAMP),
            flagged,
            "{label}: the move is in flight at the rebuild"
        );
        let direct = direct_route_edit(&mut rig, -3.0);
        let outcome = if flagged {
            crate::OUTCOME_TRANSITION_FALLBACK
        } else {
            crate::OUTCOME_EXACT
        };
        let (adoption, live) = rebuild_and_adopt_as(&mut rig, &[direct], 48, Some(outcome));
        assert_eq!(
            fallback_count(&rig),
            u64::from(flagged),
            "{label}: the fallback count"
        );
        while (twin.block as usize) * quantum < adoption as usize {
            twin.step();
        }
        let reference: Vec<_> = (0..live.len()).map(|_| twin.step()).collect();
        assert_eq!(
            plane_bits(&live, 1, quantum),
            plane_bits(&reference, 1, quantum),
            "{label}: the send keeps its move, the twin's bits"
        );
    }
}

/// #1654 root ruling B19-R8 (the split verifier's MAJOR-2) on the C ABI. On
/// [`open_follow_document`], one transaction moves the send's `gain_db` to -3 dB over 4,000
/// samples and edits the direct route `eq0-main`: it rebuilds, and its own send move is a join
/// record with a ramp. The rebuild completes `TRANSITION_FALLBACK` (count 1), though no earlier
/// send move is in the window. The direct-route edit alone completes `EXACT`.
///
/// Test value: red if the flag reads only the send moves of earlier commits (the window folds the
/// transaction's own records after the commit), or if a rebuild with no live send move is flagged.
#[test]
fn a_send_move_inside_a_rebuild_is_flagged_on_the_c_abi() {
    let document = open_follow_document();
    for (label, moved) in [
        ("the move with the rebuild", true),
        ("the rebuild alone", false),
    ] {
        let mut rig = Rig::new(&document);
        for _ in 0..4 {
            rig.step();
        }
        let mut edits = Vec::new();
        if moved {
            edits.push(send_row_edit("gain_db", -3.0, Some(4_000)));
        }
        edits.push(direct_route_edit(&mut rig, -3.0));
        let outcome = if moved {
            crate::OUTCOME_TRANSITION_FALLBACK
        } else {
            crate::OUTCOME_EXACT
        };
        let _ = rebuild_and_adopt_as(&mut rig, &edits, 1, Some(outcome));
        assert_eq!(
            fallback_count(&rig),
            u64::from(moved),
            "{label}: the fallback count"
        );
    }
}

/// #1654 root ruling B19-R9 (the B19-R8 check's repro) on the C ABI. On [`open_follow_document`]
/// (quantum 128, 480-sample landings), a live send gain move to -3 dB from 512 over 200 samples
/// is in flight when R1 (the direct route `eq0-main` to -3 dB) rebuilds: R1 adopts at 640 flagged
/// (count 1), and render may land the send at once there over the landing length. R2 (`eq0-main`
/// to -6 dB), submitted at 896 after the move ended but inside R1's landing, completes
/// `TRANSITION_FALLBACK` too (count 2). R3 (`eq0-main` to -9 dB), long after every landing ended,
/// with no live send move, completes `EXACT` (count stays 2).
///
/// Test value: red if a flagged rebuild does not fold its own at-once landing into the send move
/// window (a second rebuild inside it reads `EXACT`), or if the window outlives that landing.
#[test]
fn a_second_rebuild_inside_a_flagged_rebuilds_landing_is_flagged_on_the_c_abi() {
    let mut rig = Rig::new(&open_follow_document());
    assert_eq!(rig.quantum, 128);
    for _ in 0..4 {
        rig.step();
    }
    let (start, _) = live_and_render(&mut rig, &[send_row_edit("gain_db", -3.0, Some(200))], 1);
    assert_eq!(start, 512, "the move starts at 512");
    // (label, submit block, gain, outcome, fallback count)
    for (label, block, db, outcome, fallbacks) in [
        ("R1", 5, -3.0, crate::OUTCOME_TRANSITION_FALLBACK, 1),
        ("R2", 7, -6.0, crate::OUTCOME_TRANSITION_FALLBACK, 2),
        ("R3", 16, -9.0, crate::OUTCOME_EXACT, 2),
    ] {
        while rig.block < block {
            rig.step();
        }
        assert_eq!(rig.block, block, "{label}: submitted at its block");
        let edits = [direct_route_edit(&mut rig, db)];
        let (first, _) = rebuild_and_adopt_as(&mut rig, &edits, 1, Some(outcome));
        assert_eq!(first, block * 128, "{label}: adopted at its submit block");
        assert_eq!(
            fallback_count(&rig),
            fallbacks,
            "{label}: the fallback count"
        );
    }
}

/// #1859 gate 1: one transaction that enables band 2 of eq4's console EQ (at -6 dB, so it is
/// heard) and switches band 1 of the last track's EQ insert from a bell to a notch commits one
/// revision with no plan, and, with the source `-0.0` until the 64-sample ramp has passed, renders
/// from then on the bits of a plan prepared from the committed session.
///
/// Red if a host path classifies a band's enable or family row as prepared (a rebuild), refuses
/// its target, or designs it from the wrong seeds, so the live ramp settles on words or state a
/// fresh plan of the committed model does not have.
#[test]
fn a_live_eq_band_enable_and_family_change_renders_the_prepared_plan_bits() {
    use session::{EffectParam, ParameterChannel, ParameterUnit, RackName};
    /// The notch family's choice value (`EqBandKind::Notch`).
    const NOTCH: f32 = 6.0;
    let document = eq_session(48_000);
    let mut model = parse_session_json(&document).expect("model");
    let last = model.tracks.len() - 1;
    let last_id = model.tracks[last].id.as_str().to_owned();
    let edits = [
        upsert(
            "eq4",
            RackName::Console,
            "eq",
            17,
            ParameterChannel::Both,
            ParameterUnit::Linear,
            1.0,
        ),
        upsert(
            "eq4",
            RackName::Console,
            "eq",
            20,
            ParameterChannel::Both,
            ParameterUnit::Db,
            -6.0,
        ),
        upsert(
            &last_id,
            RackName::Inserts,
            "eq-insert",
            2,
            ParameterChannel::Both,
            ParameterUnit::Linear,
            NOTCH,
        ),
    ];
    let both = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    let eq4 = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "eq4")
        .expect("eq4");
    eq4.console[0].params.extend([
        both(17, ParameterUnit::Linear, 1.0),
        both(20, ParameterUnit::Db, -6.0),
    ]);
    let insert = model.tracks[last]
        .inserts
        .effects
        .iter_mut()
        .find(|effect| effect.id.as_str() == "eq-insert")
        .expect("eq-insert");
    insert.params.push(both(2, ParameterUnit::Linear, NOTCH));
    let committed = session::canonical_session_json(&model).expect("committed session");

    let mut rig = Rig::new(&document);
    let edit_block = 4;
    let settled = edit_block + window(rig.latency(), 64, rig.quantum) + 1;
    let feed = Feed::quiet_until(settled * rig.quantum as u64);
    let total = settled + 4;
    let mut live = Vec::new();
    for _ in 0..edit_block {
        live.push(rig.step_feed(&feed).0);
    }
    let (revision, _, epoch, _) = rig.summary();
    assert_eq!(rig.apply(&edits), crate::RESULT_OK);
    let (after, _, after_epoch, pending) = rig.summary();
    assert_eq!(
        (after, after_epoch, pending),
        (revision + 1, epoch, 0),
        "the edit commits one revision and prepares no plan"
    );
    for _ in edit_block..total {
        live.push(rig.step_feed(&feed).0);
    }
    let reference = Reference::run_feed(&committed, total, &feed);
    let unedited = Reference::run_feed(&document, total, &feed);
    let mut heard = false;
    for block in settled as usize..total as usize {
        assert!(
            reference[block].iter().any(|sample| *sample != 0.0),
            "block {block} carries signal"
        );
        assert_eq!(bits(&live[block]), bits(&reference[block]), "block {block}");
        heard |= bits(&live[block]) != bits(&unedited[block]);
    }
    assert!(heard, "the edit changes the output");
    assert_eq!(rig.summary().2, epoch, "the first epoch still renders");
}

/// #1584 gate 7: the C ABI's parity row. One `Observe` arm of a compressor insert's gain-reduction
/// tap through `miso_engine_v1_submit_command` replies `monitor` at the committed revision, and the
/// reader the session keeps for the plan publishes four-block windows from the block after the
/// commit: three windows after twelve blocks, the last opening at the ninth.
///
/// Red if the C ABI plan has no observation capacity or keeps no reader (the edit would be refused
/// or acknowledged with nothing to read), or arms the tap at another window or block.
#[test]
fn an_observe_edit_arms_a_tap_through_the_c_abi() {
    let document = effect_session(10, 48_000);
    let mut rig = Rig::new(&document);
    rig.step();
    let revision = rig.summary().0;
    let track = parse_session_json(&document).expect("model").tracks[0]
        .id
        .clone();
    let bytes = rig.transaction(&[SessionEdit::Observe {
        strip_id: track.clone(),
        section: protocol::control_core::TapSection::Inserts(
            StableId::parse("comp").expect("compressor"),
        ),
        tap_id: 1,
        armed: true,
        window_blocks: 4,
        arm_tag: 0,
    }]);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
    assert_eq!(edit_path_of(&response), protocol::EditPath::Monitor);
    assert_eq!(rig.summary().0, revision, "a subscription is no revision");
    let armed_at = rig.block;
    for _ in 0..12 {
        rig.step();
    }
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let window = snapshot
        .observation_windows
        .iter()
        .find(|(epoch, strip, effect, _)| {
            *epoch == snapshot.provider_epoch && **strip == *track.as_str() && &**effect == "comp"
        })
        .expect("the compressor's readers")
        .3[0]
        .expect("a published window");
    let quantum = rig.quantum as u64;
    assert_eq!(
        (window.0, window.1, window.2),
        (3, (armed_at + 8) * quantum, 4)
    );
}

/// #1585 gate 2: each `Observe` reply names the arm tag of the record behind the subscription,
/// and a commit that ends one names its tag. On the compressor insert's tap: an arm (tag 1), a
/// second subscriber's arm (still 1), one disarm (still 1), the last disarm (its record, 2); then
/// an arm (3); one transaction of a join, a disarm, the last disarm and an arm with another
/// window (3, 3, then the re-arm record's 4 twice); a release and a re-arm with the same window
/// (no record: 4, 4, and the binding keeps 4); and a `RemoveTrackEffect` of the compressor,
/// whose reply carries `removed_arm_tags` `[4]` and no `observe_arm_tags`. No other reply carries
/// `removed_arm_tags`.
///
/// Red if a reply cannot be matched to the arm it caused (a join or a kept disarm that reports a
/// fresh tag, a last disarm that reports the binding's old one, a tag the record was not pushed
/// with), or if a removed subscription leaves no trace in the reply.
#[test]
fn each_observe_reply_names_the_arm_tag_of_its_record() {
    let document = effect_session(10, 48_000);
    let mut rig = Rig::new(&document);
    rig.step();
    let track = parse_session_json(&document).expect("model").tracks[0]
        .id
        .clone();
    let comp = StableId::parse("comp").expect("compressor");
    // A disarm names the tag of the arm it ends (#2087), here the arm bound when it is sent.
    let observe_with = |armed, window_blocks, arm_tag| SessionEdit::Observe {
        strip_id: track.clone(),
        section: protocol::control_core::TapSection::Inserts(comp.clone()),
        tap_id: 1,
        armed,
        window_blocks,
        arm_tag: if armed { 0 } else { arm_tag },
    };
    let observe = |armed, arm_tag| observe_with(armed, 4, arm_tag);
    for (armed, tag) in [(true, 1), (true, 1), (false, 1), (false, 2), (true, 3)] {
        let bytes = rig.transaction(&[observe(armed, 1)]);
        let (result, response) = rig.send(&bytes);
        assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
        let applied = applied_of(&response);
        assert_eq!(
            (applied.observe_arm_tags, applied.removed_arm_tags),
            (vec![tag], Vec::new()),
            "an arm {armed} replies tag {tag}"
        );
        rig.step();
    }
    // One transaction: a join and a disarm keep the binding tagged 3; the last disarm and an arm
    // with another window are one re-arm record, tag 4, which both report.
    let bytes = rig.transaction(&[
        observe(true, 0),
        observe(false, 3),
        observe(false, 3),
        observe_with(true, 8, 0),
    ]);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
    assert_eq!(applied_of(&response).observe_arm_tags, vec![3, 3, 4, 4]);
    rig.step();
    // The last disarm and an arm with the same window queue nothing: the tap stays as it was, and
    // its binding keeps tag 4.
    let bytes = rig.transaction(&[observe_with(false, 8, 4), observe_with(true, 8, 0)]);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
    assert_eq!(applied_of(&response).observe_arm_tags, vec![4, 4]);
    rig.step();
    let bytes = rig.transaction(&[SessionEdit::RemoveTrackEffect {
        track_id: track.clone(),
        rack_name: session::RackName::Inserts,
        effect_id: comp.clone(),
    }]);
    let (result, response) = rig.send(&bytes);
    assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
    let applied = applied_of(&response);
    assert_eq!(applied.edit_path, protocol::EditPath::Rebuild);
    assert_eq!(
        (applied.observe_arm_tags, applied.removed_arm_tags),
        (Vec::new(), vec![4]),
        "the removal names the ended subscription's tag"
    );
}

/// `tracks` tracks of the parity session with a compressor console slot `glue` after its EQ, at the
/// slot's `dual_mono`, every entry on `slot` with a -40 dB threshold so the detector law is heard,
/// and on the first track a compressor insert `duck` whose sidechain is the track's own input, so
/// it renders per node (a connected sidechain never banks), at the same threshold (#1371).
fn link_session(tracks: usize) -> String {
    let base = if tracks == 1 {
        long_session(1, 48_000)
    } else {
        long_session(10, 48_000)
    };
    with_model(&base, |model| {
        model.tracks.truncate(tracks);
        let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
        model.routes.retain(
            |route| matches!(&route.source, RouteSource::Track { track_id, .. } if kept.contains(track_id)),
        );
        let glue = StableId::parse("glue").expect("slot ID");
        let threshold = session::EffectParam {
            parameter_id: 1,
            channel: session::ParameterChannel::Both,
            unit: session::ParameterUnit::Db,
            value: -40.0,
        };
        model.console.pre_insert.push(session::ConsoleSlot {
            slot: glue.clone(),
            identity: session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor ID"),
            },
            quality: session::EffectQuality::Normal,
            link_mode: session::LinkMode::DualMono,
        });
        let position = model.console.pre_insert.len() - 1;
        for track in &mut model.tracks {
            track.console.insert(
                position,
                session::ConsoleEntry {
                    slot: glue.clone(),
                    bypass: false,
                    link_mode: session::ConsoleLinkMode::Slot,
                    params: vec![threshold.clone()],
                },
            );
        }
        let first = model.tracks[0].id.clone();
        model.tracks[0].inserts.effects.push(session::Effect {
            id: StableId::parse("duck").expect("insert ID"),
            identity: session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor ID"),
            },
            quality: session::EffectQuality::Normal,
            bypass: false,
            link_mode: session::LinkMode::DualMono,
            params: vec![threshold.clone()],
            sidechain: session::SidechainDeclaration::Routed(session::Sidechain {
                source: RouteSource::Track {
                    track_id: first,
                    tap: session::SendTap::Input,
                },
                port_id: StableId::parse("sidechain-in").expect("port ID"),
            }),
        });
    })
}

/// A `SET_CONTROL` of strip `track_id`'s `glue` entry's link-mode row to `code` (`0` slot, then
/// the `effect_contract::LinkMode` codes), with the edit's own glide `ramp`.
fn link_edit(track_id: &str, code: f32, ramp: Option<u32>) -> SessionEdit {
    link_edit_at(
        track_id,
        protocol::control_core::Section::Console(StableId::parse("glue").expect("slot ID")),
        code,
        ramp,
    )
}

/// [`link_edit`] of the instance at `section`.
fn link_edit_at(
    track_id: &str,
    section: protocol::control_core::Section,
    code: f32,
    ramp: Option<u32>,
) -> SessionEdit {
    SessionEdit::set_control(
        protocol::control_core::ControlAddress {
            owner: protocol::control_core::Owner::Strip(
                StableId::parse(track_id).expect("track ID"),
            ),
            section,
            control: protocol::control_core::LINK_MODE_CONTROL,
            lane: protocol::control_core::Lane::Both,
        },
        code,
        ramp,
    )
}

fn glide(
    mode: effect_contract::LinkMode,
    ramp_samples: u32,
) -> effect_contract::EffectControlRecord {
    effect_contract::EffectControlRecord::Link(effect_contract::LinkSwitch { mode, ramp_samples })
}

/// #1371 gates 2 and 5. A committed link change commits one revision and prepares no plan, on a
/// console entry's override (a banked lane) and on an insert that renders per node; from the next
/// block the C ABI plan renders what a host-core plan with every live lane renders when its lane's
/// link cell takes the same switch at the same block, the output differs from an unedited run,
/// and the block that applies the switch allocates nothing.
///
/// Test value: red if the link row still rebuilds, if a drained switch never reaches
/// `retarget_link` on a bank stage or on a per-node instance (acked and never heard: the output
/// equals the unedited run), if the C ABI writes another mode, glide or instance than the cell the
/// lane reference takes, or if applying a switch allocates on the render thread.
#[test]
fn a_link_change_glides_live_and_is_heard() {
    let document = link_session(2);
    let id = |text| StableId::parse(text).expect("an ID");
    let console = || protocol::control_core::Section::Console(id("glue"));
    let insert = || protocol::control_core::Section::Inserts(id("duck"));
    for (label, section, instance, code, mode) in [
        (
            "console override",
            console(),
            "glue",
            3.0,
            effect_contract::LinkMode::Average,
        ),
        (
            "per-node insert",
            insert(),
            "duck",
            2.0,
            effect_contract::LinkMode::Maximum,
        ),
    ] {
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
            unedited.step();
        }
        let (revision, _, epoch, _) = rig.summary();
        assert_eq!(
            rig.apply(&[link_edit_at("eq0", section, code, Some(256))]),
            crate::RESULT_OK,
            "{label}"
        );
        assert_eq!(
            {
                let (after, _, after_epoch, pending) = rig.summary();
                (after, after_epoch, pending)
            },
            (revision + 1, epoch, 0),
            "{label}: the change commits one revision and prepares no plan"
        );
        lanes.write("eq0", instance, glide(mode, 256));
        let mut audible = false;
        for block in 0..12 {
            let (live, used) = rig.step_counted();
            if block == 0 {
                assert_eq!(
                    (used.allocations, used.deallocations, used.reallocations),
                    (0, 0, 0),
                    "{label}: the block that applies the switch"
                );
            }
            assert_eq!(
                bits(&live),
                bits(&lanes.reference.step()),
                "{label}: block {block}"
            );
            audible |= bits(&live) != bits(&unedited.step());
        }
        assert!(audible, "{label}: the change changes the output");
    }
}

/// #1371 gate 3. A paused host sends forty transactions, each changing the first track's `glue`
/// override (cycling `maximum`, `average`, `dual_mono`), and every one is `RESULT_OK` with no plan
/// prepared. The next blocks equal a twin that made only the last edit, and
/// `LIVE_VALUES_SUPERSEDED` grows by exactly 39 (one link cell, 39 writes replaced unread). A
/// transaction with a link change refused at its last fallible step writes no cell and leaves the
/// model as it was.
///
/// Test value: red if a link value rides a bounded queue (an edit refused for room) or rebuilds,
/// if the supersession count is not one per replaced link write, or if a link cell is written
/// before a later check of its transaction can still refuse it (an ack could precede a drop).
#[test]
fn forty_link_edits_before_one_block_are_all_accepted_and_the_last_is_heard() {
    let document = link_session(2);
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    rig.step();
    twin.step();
    let code = |edit: usize| [2.0, 3.0, 1.0][edit % 3];
    for edit in 0..40 {
        assert_eq!(
            rig.apply(&[link_edit("eq0", code(edit), None)]),
            crate::RESULT_OK,
            "edit {edit}"
        );
    }
    assert_eq!(rig.summary().3, 0, "no edit prepared a plan");
    assert_eq!(effect_writes(&rig, Epoch::Current, "eq0", "glue")[1], 40);
    assert_eq!(
        twin.apply(&[link_edit("eq0", code(39), None)]),
        crate::RESULT_OK
    );
    let before = rig.counter(protocol::CounterId::LiveValuesSuperseded);
    for block in 0..12 {
        assert_eq!(bits(&rig.step()), bits(&twin.step()), "block {block}");
    }
    assert_eq!(
        rig.counter(protocol::CounterId::LiveValuesSuperseded) - before,
        39
    );

    // A link change refused after every preflight, at the commit's last fallible step.
    let state = refusal_state(&rig);
    let cells = effect_writes(&rig, Epoch::Current, "eq0", "glue");
    crate::ffi::test_set_structural_faults(
        rig.session,
        [Some(TestStructuralFaultPhase::BeforeLivePush), None],
    );
    assert_eq!(
        rig.apply(&[link_edit("eq0", 3.0, None)]),
        crate::RESULT_BACKPRESSURE
    );
    assert_eq!(refusal_state(&rig), state);
    assert_eq!(effect_writes(&rig, Epoch::Current, "eq0", "glue"), cells);
    assert_eq!(rig.apply(&[link_edit("eq0", 3.0, None)]), crate::RESULT_OK);
    assert_eq!(
        effect_writes(&rig, Epoch::Current, "eq0", "glue")[1],
        cells[1] + 1
    );
}

/// #1371 D2, the same-mode rule. While `eq0`'s `glue` lane glides to `average` over 768 frames, two
/// transactions before the next block write `maximum` and then `average`: the newest write is the
/// mode the lane already heads for, so the glide goes on untouched, bit for bit what a twin that
/// made only the first edit renders.
///
/// Test value: red if a block that reads a switch to the lane's own target retargets it anyway
/// (the glide restarts its timeline from where it is, a different curve).
#[test]
fn a_change_and_its_reversal_before_a_block_leave_the_glide_be() {
    let document = link_session(2);
    let mut rig = Rig::new(&document);
    let mut twin = Rig::new(&document);
    for _ in 0..window(rig.latency(), 0, rig.quantum) {
        assert_eq!(bits(&rig.step()), bits(&twin.step()), "warm-up");
    }
    let first = link_edit("eq0", 3.0, Some(768));
    assert_eq!(rig.apply(std::slice::from_ref(&first)), crate::RESULT_OK);
    assert_eq!(twin.apply(std::slice::from_ref(&first)), crate::RESULT_OK);
    assert_eq!(bits(&rig.step()), bits(&twin.step()), "the glide begins");
    assert_eq!(rig.apply(&[link_edit("eq0", 2.0, None)]), crate::RESULT_OK);
    assert_eq!(rig.apply(&[link_edit("eq0", 3.0, None)]), crate::RESULT_OK);
    for block in 0..10 {
        assert_eq!(bits(&rig.step()), bits(&twin.step()), "block {block}");
    }
}

/// #1371 gate 4 (D6, root's mid-glide ruling). A rebuild that adds a muted track, whose ID sorts
/// first so the carried lane moves, lands while `eq0`'s `glue` lane glides to `average` over 768
/// frames: (a) the glide was applied a block before the swap, (b) the link edit is still unread
/// when the rebuild commits, (c) the link edit rides the rebuilding transaction. In every case
/// the swapped run renders bit for bit what a twin that only made the link edit renders.
///
/// Test value: red if the successor, prepared at the committed mode, seats it at rest and drops
/// the glide in flight, or seats the committed mode before the predecessor's unread or the
/// rebuild's own switch can glide to it (a step), or if the carry forgets the unread switch.
#[test]
fn a_rebuild_mid_glide_carries_the_glide_to_its_target() {
    let document = link_session(1);
    let model = parse_session_json(&document).expect("model");
    let structural = super::tests::add_muted_track(&model, "a-muted");
    let edit = link_edit("eq0", 3.0, Some(768));
    for case in ["applied", "unread", "same transaction"] {
        let mut rig = Rig::new(&document);
        let mut twin = Rig::new(&document);
        for _ in 0..window(rig.latency(), 0, rig.quantum) {
            assert_eq!(bits(&rig.step()), bits(&twin.step()), "{case}: warm-up");
        }
        assert_eq!(
            twin.apply(std::slice::from_ref(&edit)),
            crate::RESULT_OK,
            "{case}"
        );
        match case {
            "applied" => {
                assert_eq!(
                    rig.apply(std::slice::from_ref(&edit)),
                    crate::RESULT_OK,
                    "{case}"
                );
                assert_eq!(
                    bits(&rig.step()),
                    bits(&twin.step()),
                    "{case}: glide begins"
                );
                assert_eq!(rig.apply(&structural), crate::RESULT_OK, "{case}");
            }
            "unread" => {
                assert_eq!(
                    rig.apply(std::slice::from_ref(&edit)),
                    crate::RESULT_OK,
                    "{case}"
                );
                assert_eq!(rig.apply(&structural), crate::RESULT_OK, "{case}");
            }
            _ => {
                let mut edits = vec![edit.clone()];
                edits.extend(structural.iter().cloned());
                assert_eq!(rig.apply(&edits), crate::RESULT_OK, "{case}");
            }
        }
        assert_eq!(rig.summary().3, 1, "{case}: the transaction rebuilds");
        for block in 0..10 {
            assert_eq!(
                bits(&rig.step()),
                bits(&twin.step()),
                "{case}: block {block}"
            );
        }
        assert_eq!(
            crate::ffi::test_plan_carry_counts(rig.plan).0,
            1,
            "{case}: one swap"
        );
    }
}

// --- OBSERVATION_READ (#1586b) ---

/// A frame limit a 256-row `OBSERVATION_READ` reply fits (the default fixture's 4,096 bytes carry
/// 21 rows at most).
const OBSERVATION_FRAME_BYTES: u64 = 65_536;

fn observation_limits() -> CompileLimits {
    CompileLimits {
        maximum_control_frame_bytes: OBSERVATION_FRAME_BYTES,
        maximum_replay_bytes: 8 * OBSERVATION_FRAME_BYTES,
        maximum_effects: 512,
        ..limits()
    }
}

/// Compressor inserts per track in [`many_compressor_session`].
const COMPRESSORS_PER_TRACK: usize = 26;

/// [`effect_session`]'s ten tracks with 25 more compressor inserts each (`comp1` to `comp25`):
/// 260 compressor gain-reduction taps.
fn many_compressor_session() -> String {
    with_model(&effect_session(10, 48_000), |model| {
        for track in &mut model.tracks {
            let template = track
                .inserts
                .effects
                .iter()
                .find(|effect| effect.id.as_str() == "comp")
                .expect("the compressor")
                .clone();
            for index in 1..COMPRESSORS_PER_TRACK {
                let mut compressor = template.clone();
                compressor.id = StableId::parse(&format!("comp{index}")).expect("compressor");
                track.inserts.effects.push(compressor);
            }
        }
    })
}

fn tap_selection(
    strip: &StableId,
    section: protocol::control_core::TapSection,
    tap_id: u32,
) -> protocol::ObservationReadSelection {
    protocol::ObservationReadSelection {
        strip_id: strip.clone(),
        section,
        tap_id,
        channels: protocol::ParameterChannel::Both,
    }
}

fn insert_tap(strip: &StableId, insert: &str) -> protocol::ObservationReadSelection {
    tap_selection(
        strip,
        protocol::control_core::TapSection::Inserts(StableId::parse(insert).expect("insert")),
        1,
    )
}

impl Rig {
    /// One `OBSERVATION_READ` through `miso_engine_v1_submit_command`, with an output of the
    /// session's frame limit: the result code and the response frame.
    fn observation_read(
        &mut self,
        selections: Vec<protocol::ObservationReadSelection>,
    ) -> (u32, Vec<u8>) {
        let request = self.next_request();
        let bytes = command_bytes_at_revision(
            request,
            ExpectedRevision::Any,
            protocol::CommandPayload::ObservationRead(&protocol::ObservationReadRequest {
                selections,
            }),
        );
        let (result, _, response) = super::tests::command_c_capacity(
            self.session,
            &bytes,
            OBSERVATION_FRAME_BYTES as usize,
        );
        (result, response)
    }

    /// One transaction, as [`Self::send`] sends it, with an output of the session's frame limit:
    /// the reply's arm tags.
    fn observe_tags(&mut self, edits: &[SessionEdit]) -> Vec<u64> {
        let bytes = self.transaction(edits);
        let (result, _, response) = super::tests::command_c_capacity(
            self.session,
            &bytes,
            OBSERVATION_FRAME_BYTES as usize,
        );
        assert_eq!(result, crate::RESULT_OK, "{:?}", self.last_error());
        self.drain_reliable();
        applied_of(&response).observe_arm_tags
    }

    /// [`Self::observation_read`] with the calling thread's allocator events inside the
    /// `miso_engine_v1_submit_command` call alone: the frame and the output are allocated first.
    fn observation_read_counted(
        &mut self,
        selections: Vec<protocol::ObservationReadSelection>,
    ) -> (u32, bench_support::alloc::Counters) {
        let request = self.next_request();
        let bytes = command_bytes_at_revision(
            request,
            ExpectedRevision::Any,
            protocol::CommandPayload::ObservationRead(&protocol::ObservationReadRequest {
                selections,
            }),
        );
        let mut storage = vec![0_u8; OBSERVATION_FRAME_BYTES as usize];
        let mut output = crate::BytesOut {
            struct_size: crate::BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: u64::MAX,
        };
        let mark = bench_support::alloc::current_thread_counters();
        let result = crate::ffi::test_submit_command(self.session, &bytes, &mut output);
        (
            result,
            bench_support::alloc::current_thread_delta_since(mark),
        )
    }

    /// The rows of one successful `OBSERVATION_READ`.
    fn observation_rows(
        &mut self,
        selections: Vec<protocol::ObservationReadSelection>,
    ) -> Vec<protocol::ObservationReadRow> {
        let (result, response) = self.observation_read(selections);
        assert_eq!(result, crate::RESULT_OK, "{:?}", self.last_error());
        let mut fields = [0_u16; 1024];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::ObservationRead(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a response frame")
        else {
            panic!("an observation read page");
        };
        page.rows.into_owned()
    }
}

/// #1586 gates 1 and 6 (the allocation half) on #1584 gate 1's tap, the first track's compressor
/// insert's gain reduction. Before any `Observe` the row is unarmed. After the arm transaction and
/// before a render it is pending and claims no arm (render has not applied it: #1585's status
/// contract, under which an arm render has not applied reads pending, not unarmed). After one
/// block it is pending, armed at that block's first sample with the reply's tag. Once the first
/// four-block window closes it is ready: the window opened at the arm sample, and its span,
/// sequence and both lanes are the reader's own window, bit for bit, in dB. A second read with no
/// render answers the same rows. After a disarm it is unarmed at once, and after one block it is
/// unarmed with the disarm's tag. A read of 1 and of 256 compressor taps through
/// `miso_engine_v1_submit_command`, after the reads above, allocates nothing: the
/// frame decode, the read, the reply's encode and its replay entry.
///
/// Red if a row reports an arm render has not applied, a window from before the arm, another
/// slot's window or a swapped lane, or a tag other than the record's, or if the read allocates.
#[test]
fn an_observation_read_follows_the_arm_through_the_c_abi() {
    let document = many_compressor_session();
    let mut rig = Rig::with_limits(&document, observation_limits());
    rig.step();
    let model = parse_session_json(&document).expect("model");
    let track = model.tracks[0].id.clone();
    let comp = || insert_tap(&track, "comp");
    let row = |rig: &mut Rig| rig.observation_rows(vec![comp()])[0];
    let unarmed = row(&mut rig);
    assert_eq!(
        unarmed,
        protocol::ObservationReadRow {
            status: protocol::ObservationRowStatus::Unarmed,
            armed: false,
            arm_sample: 0,
            arm_tag: 0,
            window: None,
            left: None,
            right: None,
            unit: protocol::ParameterUnit::Db,
        }
    );
    let observe = |armed, arm_tag| SessionEdit::Observe {
        strip_id: track.clone(),
        section: protocol::control_core::TapSection::Inserts(
            StableId::parse("comp").expect("compressor"),
        ),
        tap_id: 1,
        armed,
        window_blocks: 4,
        arm_tag,
    };
    let arm_tag = rig.observe_tags(&[observe(true, 0)])[0];
    assert_eq!(
        row(&mut rig),
        protocol::ObservationReadRow {
            status: protocol::ObservationRowStatus::Pending,
            ..unarmed
        },
        "render has not applied the arm"
    );
    let quantum = rig.quantum as u64;
    let armed_at = rig.block * quantum;
    rig.step();
    let armed = protocol::ObservationReadRow {
        status: protocol::ObservationRowStatus::Pending,
        armed: true,
        arm_sample: armed_at,
        arm_tag,
        ..unarmed
    };
    assert_eq!(row(&mut rig), armed, "armed, no window since the arm");
    for _ in 0..3 {
        rig.step();
    }
    let ready = row(&mut rig);
    let snapshot = crate::ffi::test_transaction_snapshot(rig.session);
    let (sequence, first_sample, blocks, [left, right]) = snapshot
        .observation_windows
        .iter()
        .find(|(epoch, strip, effect, _)| {
            *epoch == snapshot.provider_epoch && **strip == *track.as_str() && &**effect == "comp"
        })
        .expect("the compressor's readers")
        .3[0]
        .expect("a published window");
    assert_eq!(first_sample, armed_at, "the first window opens at the arm");
    assert_eq!(
        ready,
        protocol::ObservationReadRow {
            status: protocol::ObservationRowStatus::Ready,
            window: Some(protocol::ObservationRowWindow {
                first_sample,
                end_sample: first_sample + u64::from(blocks) * quantum,
                sequence,
                blocks: 4,
            }),
            left: Some(f32::from_bits(left)),
            right: Some(f32::from_bits(right)),
            ..armed
        }
    );
    assert_eq!(row(&mut rig), ready, "a read consumes nothing");

    // Gate 6: a 1-row and a 256-row read allocate nothing.
    let every = model
        .tracks
        .iter()
        .flat_map(|track| {
            core::iter::once("comp".to_owned())
                .chain((1..COMPRESSORS_PER_TRACK).map(|index| format!("comp{index}")))
                .map(|insert| insert_tap(&track.id, &insert))
        })
        .take(protocol::OBSERVATION_READ_MAXIMUM_SELECTIONS)
        .collect::<Vec<_>>();
    assert_eq!(every.len(), protocol::OBSERVATION_READ_MAXIMUM_SELECTIONS);
    for rows in [1, protocol::OBSERVATION_READ_MAXIMUM_SELECTIONS] {
        let selections = every[..rows].to_vec();
        let (result, used) = rig.observation_read_counted(selections);
        assert_eq!(result, crate::RESULT_OK, "{:?}", rig.last_error());
        assert_eq!(
            used.allocations, 0,
            "a {rows}-row read through miso_engine_v1_submit_command allocates nothing"
        );
    }

    let disarm_tag = rig.observe_tags(&[observe(false, arm_tag)])[0];
    assert!(disarm_tag > arm_tag, "the last disarm is its own record");
    assert_eq!(
        row(&mut rig).status,
        protocol::ObservationRowStatus::Unarmed,
        "a disarm is in effect when it is accepted"
    );
    let disarmed_at = rig.block * quantum;
    rig.step();
    assert_eq!(
        row(&mut rig),
        protocol::ObservationReadRow {
            arm_sample: disarmed_at,
            arm_tag: disarm_tag,
            ..unarmed
        },
        "unarmed with the disarm's tag"
    );
}

/// #1586 gate 3 at the C ABI: a batch is checked whole before a row is written. A good selection
/// followed by an unknown tap, by the same tap again (other lanes), an unknown strip, a console
/// slot whose effect has no observation lane, and a batch whose widest reply exceeds the frame
/// limit each answer the typed refusal (D4: status, code and `selection[i]`; the frame:
/// `LIMIT_EXCEEDED`, `observation.frame`) and no page; the good selection alone then reads. A raw
/// frame of 257 selections answers a typed `INVALID_FIELD` that replays.
///
/// Red if a partly bad batch answers rows, a refusal names the wrong selection or code, or a frame
/// of 257 selections reaches the session.
#[test]
fn an_observation_read_refuses_a_bad_batch_whole() {
    let document = effect_session(10, 48_000);
    let mut rig = Rig::new(&document);
    rig.step();
    let track = parse_session_json(&document).expect("model").tracks[0]
        .id
        .clone();
    let good = insert_tap(&track, "comp");
    let refused = |rig: &mut Rig, selections| {
        let (result, response) = rig.observation_read(selections);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::NonOk { header, payload } =
            ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("a response frame")
        else {
            panic!("a refused batch answers no page");
        };
        let diagnostic = &payload.diagnostics[0];
        (
            header.status,
            diagnostic.code.clone(),
            diagnostic.path.clone(),
        )
    };
    let at = |index| {
        vec![
            protocol::PathSegment::Field("selection".to_owned()),
            protocol::PathSegment::Index(index),
        ]
    };
    let other_lanes = protocol::ObservationReadSelection {
        channels: protocol::ParameterChannel::Left,
        ..good.clone()
    };
    let clip = tap_selection(
        &track,
        protocol::control_core::TapSection::Console(StableId::parse("clip").expect("slot")),
        1,
    );
    let nowhere = insert_tap(&StableId::parse("nowhere").expect("strip"), "comp");
    for (selections, expected) in [
        (
            vec![good.clone(), tap_selection(&track, good.section.clone(), 9)],
            (protocol::StatusCode::NotFound, "observation.tap.unknown", 1),
        ),
        (
            vec![good.clone(), other_lanes],
            (
                protocol::StatusCode::InvalidField,
                "observation.selection.duplicate",
                1,
            ),
        ),
        (
            vec![good.clone(), nowhere],
            (
                protocol::StatusCode::NotFound,
                "observation.address.unknown",
                1,
            ),
        ),
        (
            vec![good.clone(), clip],
            (protocol::StatusCode::Unavailable, "observation.unbound", 1),
        ),
    ] {
        let (status, code, index) = expected;
        assert_eq!(
            refused(&mut rig, selections),
            (status, code.to_owned(), at(index)),
            "{code}"
        );
    }
    // The widest reply of 22 rows does not fit the fixture's 4,096-byte frame.
    let wide = (0..22)
        .map(|tap| tap_selection(&track, good.section.clone(), tap + 1))
        .collect::<Vec<_>>();
    assert_eq!(
        refused(&mut rig, wide),
        (
            protocol::StatusCode::LimitExceeded,
            "observation.frame".to_owned(),
            Vec::new()
        )
    );
    assert_eq!(
        rig.observation_rows(vec![good.clone()])[0].status,
        protocol::ObservationRowStatus::Unarmed
    );
    // A raw frame of 257 selections (the typed encoder writes at most 256) is refused by the
    // controller's decode as a typed `INVALID_FIELD` reply, which an exact replay answers again,
    // at a frame limit that carries the request.
    let mut wide_rig = Rig::with_limits(&document, observation_limits());
    wide_rig.step();
    let request = wide_rig.next_request();
    let mut bytes = command_bytes_at_revision(
        request,
        ExpectedRevision::Any,
        protocol::CommandPayload::ObservationRead(&protocol::ObservationReadRequest {
            selections: (1..=256)
                .map(|tap| tap_selection(&track, good.section.clone(), tap))
                .collect(),
        }),
    );
    let header = protocol::OUTER_HEADER_BYTES;
    let first_len = u32::from_le_bytes(bytes[header + 4..header + 8].try_into().expect("length"));
    let first = bytes[header..header + 8 + (first_len as usize).next_multiple_of(8)].to_vec();
    bytes.extend_from_slice(&first);
    let payload_len = u32::try_from(bytes.len() - header).expect("a short frame");
    bytes[20..24].copy_from_slice(&payload_len.to_le_bytes());
    bytes[40..44].copy_from_slice(&257_u32.to_le_bytes());
    let replies = [0, 1].map(|_| {
        let (result, _, response) = super::tests::command_c_capacity(
            wide_rig.session,
            &bytes,
            OBSERVATION_FRAME_BYTES as usize,
        );
        assert_eq!(result, crate::RESULT_OK, "{:?}", wide_rig.last_error());
        response
    });
    assert_eq!(replies[0], replies[1], "the refusal replays");
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::NonOk { header, .. } = ProtocolCodec::default()
        .decode_typed_response(&replies[0], &mut DecodeScratch::new(&mut fields))
        .expect("a response frame")
    else {
        panic!("257 selections answer no page");
    };
    assert_eq!(header.status, protocol::StatusCode::InvalidField);
}
