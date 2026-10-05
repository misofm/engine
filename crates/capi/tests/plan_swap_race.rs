//! Control calls racing plan-swapping render calls through the exported C entries (issues #1042
//! and #1273).
//!
//! This file has its own test binary so that `bench_support`'s audited allocator serves it: the
//! render thread's allocation count comes from `bench_support::alloc`'s thread-scoped counters,
//! and the producer side runs through `bench_support::producer::render_while_producing`.
//! `resource_lifecycle.rs` installs a different global allocator for its retained-byte oracles,
//! and a process can have only one.

#![allow(unsafe_code)]

use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use bench_support::producer::render_while_producing;
use capi::*;
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use session::StableId;

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// Frames per render block and per submitted source chunk.
const QUANTUM: u32 = 128;

/// Render blocks after an admission beyond which `RESULT_BACKPRESSURE` is a wedge. A healthy
/// replacement is admitted again once its swapping block has rendered.
const WEDGE_BLOCKS: u64 = 256;

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// Creates an engine, compiles `session_document` and destroys the engine.
///
/// # Safety
///
/// The returned handles are uniquely owned by the caller, who destroys each once.
unsafe fn compile_c(
    session_document: &str,
    compile_limits: &CompileLimits,
) -> (*mut Session, *mut Plan) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location remains live for the complete call.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v1_compile_session(
                engine,
                session_document.as_ptr(),
                session_document.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_OK,
            "{}",
            String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
        );
        miso_engine_v1_engine_destroy(engine);
    }
    (session, plan)
}

/// A structural transaction that adds (`add`) or removes the muted track `a-race` and its route.
/// The source is unchanged either way, so every swap carries its ring (#1273 gate 4).
fn track_command(
    request_id: u64,
    revision: u64,
    model: &session::SessionModel,
    add: bool,
) -> Vec<u8> {
    let id = StableId::parse("a-race").expect("track ID");
    let route_id = StableId::parse("a-race-main").expect("route ID");
    let edits = if add {
        let mut track = model.tracks[0].clone();
        track.id = id.clone();
        track.fader.left_mute = true;
        track.fader.right_mute = true;
        let mut route = model.routes[0].clone();
        route.id = route_id;
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("track route")
        };
        *track_id = id;
        vec![
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ]
    } else {
        vec![
            SessionEdit::RemoveRoute { route_id },
            SessionEdit::RemoveTrack { track_id: id },
        ]
    };
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("structural command");
    bytes.truncate(len);
    bytes
}

/// The session's last error text, read through the exported entry.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn last_error_c(session: *const Session) -> String {
    let mut storage = [0_u8; 4_096];
    let mut out = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: The caller guarantees a live session; the descriptor names complete owned storage.
    let code = unsafe { miso_engine_v1_last_error(session.cast(), &mut out) };
    assert_eq!(code, RESULT_OK);
    String::from_utf8_lossy(&storage[..out.required_bytes as usize]).into_owned()
}

/// Dequeues one frame from `lane` into `storage`, returning the result code.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn dequeue_c(session: *mut Session, lane: u32, storage: &mut [u8; 4_096]) -> u32 {
    let mut event = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: The caller guarantees a live session; the descriptor names complete owned storage.
    unsafe { miso_engine_v1_dequeue_event(session, lane, &mut event) }
}

/// Offers the fixture source `FRAMES` frames of constant planar PCM at `start_frame`; returns the
/// result code and the accepted frames.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn submit_dc_block<const FRAMES: usize>(
    session: *mut Session,
    start_frame: u64,
) -> (u32, u64) {
    let left = [0.25_f32; FRAMES];
    let right = [-0.5_f32; FRAMES];
    let planes = [left.as_ptr(), right.as_ptr()];
    let chunk = SourceChunk {
        struct_size: SOURCE_CHUNK_SIZE,
        sample_rate_hz: 48_000,
        generation: 1,
        start_frame,
        planes: planes.as_ptr(),
        plane_count: 2,
        frames: FRAMES as u32,
        end_of_region: 0,
        reserved0: 0,
    };
    let mut report = SubmitReport {
        struct_size: SUBMIT_REPORT_SIZE,
        reserved0: 0,
        accepted_frames: 0,
        cumulative_written_frames: 0,
        active_generation: 0,
    };
    // SAFETY: The caller guarantees a live session; the chunk's planes and the report are owned
    // storage that outlives the synchronous call.
    let code = unsafe {
        miso_engine_v1_source_submit_planar_f32(
            session,
            b"fixture-source".as_ptr(),
            14,
            &chunk,
            &mut report,
        )
    };
    (code, report.accepted_frames)
}

/// One any-thread resource query through the exported entry.
fn query(plan_address: usize) -> u32 {
    // SAFETY: The plan outlives every caller, and a zeroed report is a valid value whose size
    // field is then set for the complete fixed-size write.
    unsafe {
        let mut report: PlanResourceReport = core::mem::zeroed();
        report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        miso_engine_v1_plan_resources(plan_address as *const Plan, &mut report)
    }
}

/// Stops the reader threads however the render loop leaves their scope, so a failed assertion
/// cannot hang the join. (`render_while_producing` stops its own producer thread.)
struct StopOnDrop<'a>(&'a AtomicBool);

impl Drop for StopOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

/// The control thread's state: the session it calls, its transaction sequence and its source
/// feed. `render_while_producing` hands it to the producer thread one iteration at a time.
struct Control<'a> {
    session_address: usize,
    plan_address: usize,
    model: session::SessionModel,
    swaps: u64,
    rendered: &'a AtomicU64,
    in_render: &'a AtomicBool,
    done: &'a AtomicBool,
    response: [u8; 4_096],
    event: [u8; 4_096],
    fields: [u16; 64],
    revision: u64,
    request_id: u64,
    request: Vec<u8>,
    admitted: u64,
    overlapped: u64,
    failure: Option<String>,
    admission_block: u64,
    next_frame: u64,
    fed_blocks: u64,
}

impl Control<'_> {
    fn is_done(&self) -> bool {
        self.admitted == self.swaps || self.failure.is_some()
    }

    fn fail(&mut self, failure: String) {
        self.failure = Some(failure);
        self.done.store(true, Ordering::Release);
    }

    /// Offers the source its next contiguous block. `Err` names a refusal other than a full ring.
    fn feed(&mut self) -> Result<(), String> {
        let session = self.session_address as *mut Session;
        // SAFETY: The producer thread is the session's only control caller while the race runs.
        let (code, accepted) =
            unsafe { submit_dc_block::<{ QUANTUM as usize }>(session, self.next_frame) };
        match code {
            RESULT_OK => {
                self.next_frame += accepted;
                self.fed_blocks += 1;
                Ok(())
            }
            RESULT_BACKPRESSURE => Ok(()),
            other => {
                // SAFETY: The producer thread is the session's only control caller.
                let diagnostic = unsafe { last_error_c(session) };
                Err(format!(
                    "submit at frame {}: {other} {diagnostic}",
                    self.next_frame
                ))
            }
        }
    }

    /// One control iteration: feed the source, race the two synchronizing dequeue entries and a
    /// resource query against the render thread, and, once two blocks have rendered since the
    /// last admission, offer the next structural transaction.
    fn step(&mut self) {
        if self.is_done() {
            return;
        }
        let admitted = self.admitted;
        if let Err(refusal) = self.feed() {
            return self.fail(format!("after {admitted} swaps: {refusal}"));
        }
        let session = self.session_address as *mut Session;
        // Drain the reliable lane so `SESSION_COMMITTED` never backpressures a later transaction.
        for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
            let overlapping = self.in_render.load(Ordering::SeqCst);
            // SAFETY: The producer thread is the session's only control caller.
            let code = unsafe { dequeue_c(session, lane, &mut self.event) };
            if code != RESULT_OK {
                return self.fail(format!(
                    "dequeue lane {lane} after {admitted} swaps: {code}"
                ));
            }
            self.overlapped += u64::from(overlapping && self.in_render.load(Ordering::SeqCst));
        }
        let code = query(self.plan_address);
        if code != RESULT_OK {
            return self.fail(format!("plan resources after {admitted} swaps: {code}"));
        }
        // The block in flight at admission may predate the publication; the next one swaps.
        let blocks = self.rendered.load(Ordering::Acquire);
        if blocks < self.admission_block + 2 {
            return;
        }
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: self.response.as_mut_ptr(),
            capacity_bytes: self.response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The producer thread is the session's only control caller; buffers are owned.
        let code = unsafe {
            miso_engine_v1_submit_command(
                session,
                self.request.as_ptr(),
                self.request.len() as u64,
                &mut output,
            )
        };
        match code {
            RESULT_OK => {
                let (header, refusal) = match ProtocolCodec::default()
                    .decode_typed_response(
                        &self.response[..output.required_bytes as usize],
                        &mut protocol::DecodeScratch::new(&mut self.fields),
                    )
                    .expect("structural response")
                {
                    protocol::DecodedTypedResponseFrame::Success { header, .. } => (header, None),
                    protocol::DecodedTypedResponseFrame::NonOk { header, payload } => {
                        (header, Some(format!("{payload:?}")))
                    }
                };
                if header.status != StatusCode::Ok
                    || header.revision != SessionRevision(self.revision + 1)
                {
                    return self.fail(format!(
                        "swap {admitted}: {:?} at revision {}: {refusal:?}",
                        header.status, header.revision.0
                    ));
                }
                self.admitted += 1;
                self.revision += 1;
                self.request_id += 1;
                self.request = track_command(
                    self.request_id,
                    self.revision,
                    &self.model,
                    self.admitted.is_multiple_of(2),
                );
                self.admission_block = self.rendered.load(Ordering::Acquire);
                if self.is_done() {
                    self.done.store(true, Ordering::Release);
                }
            }
            RESULT_BACKPRESSURE if blocks - self.admission_block <= WEDGE_BLOCKS => {}
            other => self.fail(format!(
                "submit after {admitted} swaps and {} blocks: {other}",
                blocks - self.admission_block
            )),
        }
    }

    /// Whether the next block has its source PCM queued, or the race is over. Only a render
    /// drains the ring, so PCM queued here is still there when the render starts.
    fn ready(&self) -> bool {
        self.done.load(Ordering::Acquire)
            || self.next_frame >= (self.rendered.load(Ordering::Acquire) + 1) * u64::from(QUANTUM)
    }
}

/// Issue #1042 (`docs/handoffs/live-control-2026-09-28/VERIFY.md`, F1): control calls, and
/// optionally `readers` any-thread resource queries, racing plan-swapping render calls.
///
/// This thread renders blocks through the exported entry while `render_while_producing`'s
/// producer thread runs the control side ([`Control::step`]): it commits `swaps` structural
/// transactions, offering each once two blocks have rendered since the last admission, and between
/// them spins on the two dequeue entries, which synchronize plan epochs first, and on
/// `miso_engine_v1_plan_resources`. The swap therefore lands inside a render call that the
/// control thread is racing. Each reader thread polls `miso_engine_v1_plan_resources` until the
/// race ends.
///
/// Any result other than OK fails, and so does `RESULT_BACKPRESSURE` more than 256 blocks after an
/// admission. Every render call allocates and frees nothing on the render thread: swapping plans
/// adds none.
///
/// Issue #1273 gate 4: each transaction adds or removes a muted track (`UpsertTrack`), so every
/// swap carries the source's ring, and the control thread feeds the source contiguously in every
/// iteration, so PCM is submitted while a candidate is pending and inside the swap window. Each
/// block waits until its PCM is queued. A submit may only be accepted or backpressured by a full
/// ring: `source.ring.vacated` (a submit routed to an epoch whose producer has moved) or
/// `source.frame.noncontiguous` (a ring that lost its write position) fails.
fn race_plan_swaps(swaps: u64, readers: usize) {
    assert_installed();
    // Warm the JSON frontend's lazily boxed process statics before anything is observed.
    let _ = session::parse_session_json(r#"{"warm":0}"#);

    let mut model = session::parse_session_json(SESSION).expect("fixture");
    // A region the race never reaches the end of.
    model.sources[0].frames = 1 << 40;
    let document = session::canonical_session_json(&model).expect("canonical");
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(&document, &limits()) };
    let plan_address = plan as usize;
    let stop = AtomicBool::new(false);
    let in_render = AtomicBool::new(false);
    let done = AtomicBool::new(false);
    let rendered = AtomicU64::new(0);
    let request = track_command(1, 42, &model, true);
    let mut control = Control {
        session_address: session as usize,
        plan_address,
        model,
        swaps,
        rendered: &rendered,
        in_render: &in_render,
        done: &done,
        response: [0; 4_096],
        event: [0; 4_096],
        fields: [0; 64],
        revision: 42,
        request_id: 1,
        request,
        admitted: 0,
        overlapped: 0,
        failure: None,
        admission_block: 0,
        next_frame: 0,
        fed_blocks: 0,
    };

    let mut pcm = [f32::NAN; 256];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: QUANTUM,
        plane_stride_samples: QUANTUM,
        reserved: [0; 2],
    };
    // Every swap needs two blocks, and a healthy one is never backpressured for longer than
    // `WEDGE_BLOCKS`; blocks after the race ends render nothing.
    let block_bound = usize::try_from(swaps * (WEDGE_BLOCKS + 2)).expect("block bound");
    let polled = std::thread::scope(|scope| {
        let pollers = (0..readers)
            .map(|_| {
                scope.spawn(|| {
                    let mut queries = 0_u64;
                    while !stop.load(Ordering::Acquire) {
                        let code = query(plan_address);
                        queries += 1;
                        if code != RESULT_OK {
                            return (queries, Some(code));
                        }
                    }
                    (queries, None)
                })
            })
            .collect::<Vec<_>>();
        let stop_readers = StopOnDrop(&stop);
        render_while_producing(
            block_bound,
            &mut control,
            |control| control.step(),
            |control| control.ready(),
            |_| {
                if done.load(Ordering::Acquire) {
                    return;
                }
                let plan = plan_address as *mut Plan;
                let block = rendered.load(Ordering::Acquire);
                let mark = current_thread_counters();
                in_render.store(true, Ordering::SeqCst);
                // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
                let code = unsafe {
                    miso_engine_v1_render_f32_planar(plan, block * u64::from(QUANTUM), &output)
                };
                in_render.store(false, Ordering::SeqCst);
                let delta = current_thread_delta_since(mark);
                assert_eq!(code, RESULT_OK, "render refused block {block}");
                assert_eq!(
                    (delta.allocations, delta.reallocations, delta.deallocations),
                    (0, 0, 0),
                    "block {block}: the render thread allocated or freed across a plan swap"
                );
                rendered.store(block + 1, Ordering::Release);
            },
        );
        drop(stop_readers);
        pollers
            .into_iter()
            .map(|poller| poller.join().expect("reader thread"))
            .collect::<Vec<_>>()
    });

    // One more block applies the last admitted swap, which the race may have stopped before.
    let blocks = rendered.load(Ordering::Acquire);
    // SAFETY: The race has joined, so this thread is the plan's only caller.
    let (last_render, carry_counts) = unsafe {
        (
            miso_engine_v1_render_f32_planar(plan, blocks * u64::from(QUANTUM), &output),
            plan_carry_counts(plan),
        )
    };
    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    assert_eq!(last_render, RESULT_OK, "render refused the last block");
    for (reader, (queries, failure)) in polled.iter().enumerate() {
        assert_eq!(
            *failure, None,
            "reader {reader} failed after {queries} resource queries"
        );
        assert!(*queries > 0, "vacuous: reader {reader} never queried");
    }
    assert_eq!(
        control.failure, None,
        "a control call lost the race with a plan swap"
    );
    assert_eq!(
        control.admitted, swaps,
        "every swap admitted within the block bound"
    );
    assert_eq!(
        carry_counts,
        (control.admitted, 0),
        "every applied swap carried the source's ring"
    );
    assert!(
        control.fed_blocks > swaps,
        "vacuous: {} source blocks fed across {swaps} swaps",
        control.fed_blocks
    );
    assert!(
        control.overlapped > 0,
        "vacuous: no control call overlapped a render call"
    );
    assert!(blocks > swaps, "each swap rendered its own block");
}

/// Issue #1042, F1: on the unmodified base, a control call that fell between the swap at the start
/// of a render call and the `active_epoch` publication at its end returned `RESULT_INTERNAL`, and
/// every structural command after it returned `RESULT_BACKPRESSURE` for good.
#[test]
fn control_calls_racing_plan_swapping_renders_never_wedge_replacement() {
    race_plan_swaps(24, 0);
}

/// Issue #1042, attempt 2: `miso_engine_v1_plan_resources` may run on any thread, concurrently with
/// render and control. Reading the epoch before taking the report lock let a render publish the
/// next epoch and a control call remove the old row in between; the lookup then panicked while
/// holding the lock, which poisoned it, so every later control call returned `RESULT_INTERNAL`
/// (and a release build, with `panic = "abort"`, aborts).
#[test]
fn resource_queries_racing_plan_swaps_always_find_the_published_row() {
    race_plan_swaps(200, 2);
}

/// Live fader edits that alternate track `eq0` between 0 dB and -6 dB: edit `n` (counting from 1)
/// sets -6 dB when `n` is odd.
fn fader_command(request_id: u64, revision: u64, edit: u64) -> Vec<u8> {
    let db = if edit % 2 == 1 { -6.0 } else { 0.0 };
    let edits = [SessionEdit::SetTrackFader {
        track_id: StableId::parse("eq0").expect("track ID"),
        fader: session::DualMonoFader {
            left_db: db,
            right_db: db,
            left_mute: false,
            right_mute: false,
        },
    }];
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("fader command");
    bytes.truncate(len);
    bytes
}

/// One applied-revision watermark read through the any-thread entry, or `None` if it was busy.
fn watermark_c(plan_address: usize) -> Option<Watermark> {
    // SAFETY: The plan outlives every caller, and a zeroed watermark is a valid value whose size
    // field is then set for the complete fixed-size write.
    unsafe {
        let mut watermark: Watermark = core::mem::zeroed();
        watermark.struct_size = WATERMARK_SIZE;
        match miso_engine_v1_plan_watermark(plan_address as *const Plan, &mut watermark) {
            RESULT_OK => Some(watermark),
            RESULT_BACKPRESSURE => None,
            other => panic!("plan watermark: {other}"),
        }
    }
}

/// Submits one command. If it is accepted, asserts that the session committed it at `revision + 1`
/// and dequeues its reliable event.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn commit_c(session: *mut Session, request: &[u8], revision: u64) -> u32 {
    let mut response = [0_u8; 4_096];
    let mut fields = [0_u16; 64];
    let mut output = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: response.as_mut_ptr(),
        capacity_bytes: response.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: The caller guarantees a live session; every buffer is owned storage.
    let code = unsafe {
        miso_engine_v1_submit_command(session, request.as_ptr(), request.len() as u64, &mut output)
    };
    if code == RESULT_OK {
        let header = match ProtocolCodec::default()
            .decode_typed_response(
                &response[..output.required_bytes as usize],
                &mut protocol::DecodeScratch::new(&mut fields),
            )
            .expect("fader response")
        {
            protocol::DecodedTypedResponseFrame::Success { header, .. }
            | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
        };
        assert_eq!(
            (header.status, header.revision),
            (StatusCode::Ok, SessionRevision(revision + 1)),
            "the fader edit commits"
        );
        let mut event = [0_u8; 4_096];
        // SAFETY: The caller guarantees a live session used by this thread alone.
        let code = unsafe { dequeue_c(session, EVENT_LANE_RELIABLE, &mut event) };
        assert_eq!(code, RESULT_OK);
    }
    code
}

/// The live race's render quantum: long enough that a fader ramp ends within one block.
const LIVE_QUANTUM: u32 = 1_024;
const LIVE_FRAMES: usize = LIVE_QUANTUM as usize;

/// The fixture with every strip stateless (no console, no inserts, no filters), rendered in
/// [`LIVE_QUANTUM`] blocks: a constant source renders a constant block once every fader ramp has
/// ended, so a block's last sample is the level its fader targets.
fn live_document() -> String {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.sources[0].frames = 1 << 40;
    model.quantum_frames = LIVE_QUANTUM;
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for builtins in [&mut track.builtins.left, &mut track.builtins.right] {
            builtins.hpf_hz = 0.0;
            builtins.lpf_hz = 0.0;
        }
    }
    session::canonical_session_json(&model).expect("canonical")
}

fn live_limits() -> CompileLimits {
    CompileLimits {
        source_ring_frames: 4 * LIVE_QUANTUM,
        ..limits()
    }
}

/// Renders the block at `start` into `pcm` through the exported entry.
///
/// # Safety
///
/// `plan` must be live and rendered by this thread alone.
unsafe fn render_live(plan: *mut Plan, start: u64, pcm: &mut [f32; 2 * LIVE_FRAMES]) -> u32 {
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: LIVE_QUANTUM,
        plane_stride_samples: LIVE_QUANTUM,
        reserved: [0; 2],
    };
    // SAFETY: The caller guarantees a live plan; `output` names complete owned storage.
    unsafe { miso_engine_v1_render_f32_planar(plan, start, &output) }
}

/// The last sample of each plane of a block.
fn last_samples(pcm: &[f32; 2 * LIVE_FRAMES]) -> [u32; 2] {
    [
        pcm[LIVE_FRAMES - 1].to_bits(),
        pcm[2 * LIVE_FRAMES - 1].to_bits(),
    ]
}

/// The steady levels at 0 dB and at -6 dB on `eq0`, measured on one thread: the last samples of a
/// block before any edit, and of the first block after the -6 dB edit, whose ramp must end inside
/// it (the next block repeats it bit for bit).
fn live_levels(document: &str) -> [[u32; 2]; 2] {
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(document, &live_limits()) };
    let first = watermark_c(plan as usize)
        .expect("no render in flight")
        .revision;
    let mut pcm = [f32::NAN; 2 * LIVE_FRAMES];
    let block = |index: u64, pcm: &mut [f32; 2 * LIVE_FRAMES]| {
        let start = index * u64::from(LIVE_QUANTUM);
        // SAFETY: This thread is the session's and the plan's only caller.
        unsafe {
            assert_eq!(
                submit_dc_block::<LIVE_FRAMES>(session, start),
                (RESULT_OK, u64::from(LIVE_QUANTUM))
            );
            assert_eq!(render_live(plan, start, pcm), RESULT_OK);
        }
        last_samples(pcm)
    };
    let _ = block(0, &mut pcm);
    let unity = block(1, &mut pcm);
    // SAFETY: as above.
    let code = unsafe { commit_c(session, &fader_command(1, first, 1), first) };
    assert_eq!(code, RESULT_OK);
    let attenuated = block(2, &mut pcm);
    assert_eq!(
        block(3, &mut pcm),
        attenuated,
        "the ramp ends within a block"
    );
    assert_ne!(attenuated, unity, "the edit changes the level");
    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    [unity, attenuated]
}

/// The handshake that times a render call against a commit, in nanoseconds since `base`.
///
/// When control has an edit ready it sets `want`. The render thread then parks before its next
/// render call (`parked`) until `start`, which control sets just before it commits. Control
/// chooses `start` from the last commit's and the last render call's durations so that its
/// revision stores sweep the whole render call, before its drains and after them, whatever the
/// build profile or the machine: the producer lock alone would order every commit between two
/// blocks.
struct RenderGate {
    base: std::time::Instant,
    want: AtomicBool,
    parked: AtomicBool,
    start: AtomicU64,
    in_render: AtomicBool,
    render_nanos: AtomicU64,
}

impl RenderGate {
    fn now(&self) -> u64 {
        u64::try_from(self.base.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }

    /// Parks the render thread until control's start time if control asked for it.
    fn wait_for_start(&self) {
        if !self.want.load(Ordering::Acquire) {
            return;
        }
        self.parked.store(true, Ordering::Release);
        let deadline = self.now() + 10_000_000_000;
        loop {
            let start = self.start.load(Ordering::Acquire);
            let now = self.now();
            if start != 0 && now >= start {
                break;
            }
            assert!(now < deadline, "control never released the parked render");
            core::hint::spin_loop();
        }
        self.start.store(0, Ordering::Relaxed);
        self.parked.store(false, Ordering::Release);
    }
}

/// Offsets per sweep: edit `n` lands its revision store about `n % PHASES` steps along a span
/// from a quarter of a render call before the call to the call's end.
const PHASES: u64 = 16;

/// The control side of [`a_watermark_advance_names_a_block_that_applied_the_edit`]: it feeds the
/// source and, once the watermark covers the last edit, commits the next live fader edit against
/// a parked render call, so at most one edit is in flight and each advance covers exactly one
/// revision.
struct LiveControl<'a> {
    session_address: usize,
    plan_address: usize,
    edits: u64,
    committed: u64,
    revision: u64,
    gate: &'a RenderGate,
    commit_nanos: u64,
    overlapped: u64,
    next_frame: u64,
}

impl LiveControl<'_> {
    fn step(&mut self) {
        let session = self.session_address as *mut Session;
        // SAFETY: The producer thread is the session's only control caller while the race runs.
        let (code, accepted) = unsafe { submit_dc_block::<LIVE_FRAMES>(session, self.next_frame) };
        match code {
            RESULT_OK => self.next_frame += accepted,
            RESULT_BACKPRESSURE => {}
            other => panic!("submit at frame {}: {other}", self.next_frame),
        }
        if self.committed == self.edits {
            return;
        }
        match watermark_c(self.plan_address) {
            Some(watermark) if watermark.revision == self.revision => {}
            _ => return,
        }
        if !self.gate.parked.load(Ordering::Acquire) {
            self.gate.want.store(true, Ordering::Release);
            return;
        }
        // The store lands about `commit_nanos` after the commit starts. Aim it `target` nanoseconds
        // after the render call starts, `target` sweeping [-render / 4, render): start the call
        // `commit_nanos - target` before the store when that is positive, else start it now and
        // commit `target - commit_nanos` later.
        let render = self.gate.render_nanos.load(Ordering::Acquire);
        let sweep = i128::from((self.committed % PHASES) * render * 5 / (4 * PHASES));
        let target = sweep - i128::from(render / 4);
        let lead = i128::from(self.commit_nanos) - target;
        let request = fader_command(self.committed + 1, self.revision, self.committed + 1);
        self.gate.want.store(false, Ordering::Relaxed);
        let now = self.gate.now();
        let start = now + u64::try_from(lead.max(1)).expect("positive");
        self.gate.start.store(start, Ordering::Release);
        let delay = u64::try_from((-lead).max(0)).expect("non-negative");
        while self.gate.now() < now + delay {
            core::hint::spin_loop();
        }
        let began = self.gate.now();
        // SAFETY: as above.
        let code = unsafe { commit_c(session, &request, self.revision) };
        // The commit's last write, the revision store, fell inside a render call.
        let inside = self.gate.in_render.load(Ordering::SeqCst);
        self.commit_nanos = self.gate.now() - began;
        match code {
            RESULT_OK => {
                self.overlapped += u64::from(inside);
                self.committed += 1;
                self.revision += 1;
            }
            RESULT_BACKPRESSURE => {}
            other => panic!("fader edit {}: {other}", self.committed + 1),
        }
    }
}

/// #1314 D3 (verifier F2): render loads the running plan's revision word before `render_inner`,
/// so a watermark advance to `r` at a block's first sample means that block applied `r` from its
/// first drain on.
///
/// The control thread commits live fader edits on `eq0`, alternating -6 dB and 0 dB, one at a time,
/// each timed by a [`RenderGate`] against a render call, so the revision stores sweep the call:
/// some land before it, many inside `render_inner`, before the strip's drain and after it.
/// After each block this thread reads the watermark. When it advanced, to `r`, it must name this
/// block's first sample, and the block's last samples must be the steady level `r` targets
/// (measured single-threaded by [`live_levels`]): the ramp started at or before the block's first
/// sample and ends within the block. The reverse does not hold, and the test does not ask it: an
/// edit whose record a drain takes after render loaded the revision word is applied in a block the
/// watermark reports one block later.
///
/// Loading the revision after `render_inner` lets a commit that lands after the block's drains
/// advance the watermark at that block, which renders the previous level throughout.
#[test]
fn a_watermark_advance_names_a_block_that_applied_the_edit() {
    const EDITS: u64 = 320;
    let document = live_document();
    let levels = live_levels(&document);

    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(&document, &live_limits()) };
    let plan_address = plan as usize;
    let first = watermark_c(plan_address)
        .expect("no render in flight")
        .revision;
    let gate = RenderGate {
        base: std::time::Instant::now(),
        want: AtomicBool::new(false),
        parked: AtomicBool::new(false),
        start: AtomicU64::new(0),
        in_render: AtomicBool::new(false),
        render_nanos: AtomicU64::new(0),
    };
    let rendered = AtomicU64::new(0);
    // The newest revision a block's watermark read returned.
    let last = AtomicU64::new(first);
    let mut control = LiveControl {
        session_address: session as usize,
        plan_address,
        edits: EDITS,
        committed: 0,
        revision: first,
        gate: &gate,
        commit_nanos: 0,
        overlapped: 0,
        next_frame: 0,
    };
    let block_bound = usize::try_from(EDITS * 64).expect("block bound");
    render_while_producing(
        block_bound,
        &mut control,
        |control| control.step(),
        |control| {
            last.load(Ordering::Acquire) == first + EDITS
                || control.next_frame
                    >= (rendered.load(Ordering::Acquire) + 1) * u64::from(LIVE_QUANTUM)
        },
        |_| {
            let newest = last.load(Ordering::Acquire);
            if newest == first + EDITS {
                return;
            }
            let block = rendered.load(Ordering::Acquire);
            let start = block * u64::from(LIVE_QUANTUM);
            let mut pcm = [f32::NAN; 2 * LIVE_FRAMES];
            gate.wait_for_start();
            let started = gate.now();
            gate.in_render.store(true, Ordering::SeqCst);
            // SAFETY: This thread is the plan's only renderer.
            let code = unsafe { render_live(plan_address as *mut Plan, start, &mut pcm) };
            gate.in_render.store(false, Ordering::SeqCst);
            gate.render_nanos
                .store(gate.now() - started, Ordering::Release);
            assert_eq!(code, RESULT_OK, "render refused block {block}");
            let watermark = watermark_c(plan_address).expect("the renderer never reads busy");
            if watermark.revision != newest {
                let edit = watermark.revision - first;
                assert_eq!(watermark.revision, newest + 1, "one edit per advance");
                assert_eq!(
                    watermark.first_sample, start,
                    "an advance names the block that made it"
                );
                assert_eq!(
                    last_samples(&pcm),
                    levels[usize::from(edit % 2 == 1)],
                    "block {block}: the watermark advanced to edit {edit} in a block that did not \
                     apply it"
                );
                last.store(watermark.revision, Ordering::Release);
            }
            rendered.store(block + 1, Ordering::Release);
        },
    );
    let overlapped = control.overlapped;
    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    assert_eq!(
        last.load(Ordering::Acquire),
        first + EDITS,
        "every edit completed"
    );
    assert!(
        overlapped > EDITS / 10,
        "vacuous: {overlapped} of {EDITS} revision stores fell inside a render call"
    );
}
