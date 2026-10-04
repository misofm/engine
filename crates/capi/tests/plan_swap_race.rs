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
        let left = [0.25_f32; QUANTUM as usize];
        let right = [-0.5_f32; QUANTUM as usize];
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: self.next_frame,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: QUANTUM,
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
        let session = self.session_address as *mut Session;
        // SAFETY: The producer thread is the session's only control caller while the race runs;
        // the chunk's planes and the report are owned storage that outlives the synchronous call.
        let code = unsafe {
            miso_engine_v1_source_submit_planar_f32(
                session,
                b"fixture-source".as_ptr(),
                14,
                &chunk,
                &mut report,
            )
        };
        match code {
            RESULT_OK => {
                self.next_frame += report.accepted_frames;
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
