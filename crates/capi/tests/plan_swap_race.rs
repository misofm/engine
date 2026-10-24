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
use std::sync::{Mutex, MutexGuard, PoisonError};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use bench_support::producer::{StopOnDrop, render_while_producing};
use capi::*;
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use session::StableId;

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// Frames per render block and per submitted source chunk.
const QUANTUM: u32 = 128;

/// The fewest thin-sweep revision stores (of 320) that must return within 1/16 of the commit's own
/// duration of a render call's start for the two-strip race to count. That stretch is about where a
/// commit's cell writes come before a strip's drain and its publish after the block's snapshot, the
/// window in which a drain that reads outside the snapshot is caught, and it follows the commit's
/// cost, not the call's length. Measured counts, `taskset -c 0-3`, 12 runs each: quiet release 118
/// to 140, debug 23 to 35; two busy-loop processes on the CPUs, release 120 to 142, debug 28 to 39;
/// four of them (every CPU taken), release 8 to 31 and debug 1 to 6. So 10 passes a quiet or
/// half-loaded 4-CPU runner with a margin of 2 in debug and 12 in release, and refuses the debug
/// runs of a runner whose CPUs are all taken.
const THIN_FLOOR: u64 = 10;

/// Held for the whole of every timing-sensitive race test in this file. Each race spins at least
/// two threads and guards itself against vacuity with an overlap count, so two races at once
/// starve each other's overlap on a machine with few cores (a 4-CPU runner), and the test harness
/// runs a binary's tests in parallel. A failed race poisons the lock; the next race takes it anyway
/// so one failure stays one failure.
static RACES: Mutex<()> = Mutex::new(());

fn one_race_at_a_time() -> MutexGuard<'static, ()> {
    RACES.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Render blocks an admission may wait for before the race counts it as wedged. Since #1310 a
/// structural edit is never refused as backpressure for a pending candidate, so this only bounds
/// the blocks the race renders.
const WEDGE_BLOCKS: u64 = 256;

/// Render blocks the plan-swap race keeps running after its last admission while no control call
/// has yet overlapped a render call (#1405): on one CPU the two threads alternate, so an overlap
/// needs a preemption inside a render call, and the swaps alone render too few blocks to be sure
/// of one.
const OVERLAP_BLOCKS: u64 = 4_096;

/// The control thread's sleep between steps once every swap is admitted (#1405). Its timer wakeup
/// can preempt the render thread inside a render call; a `yield_now` cannot, because under EEVDF
/// each yield moves the yielding thread's deadline later.
const OVERLAP_SLEEP: std::time::Duration = std::time::Duration::from_micros(50);

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
    /// Set with [`Control::is_done`]: every swap is admitted, or the race failed.
    swapped: &'a AtomicBool,
    response: [u8; 4_096],
    event: [u8; 4_096],
    fields: [u16; 64],
    revision: u64,
    request_id: u64,
    request: Vec<u8>,
    admitted: u64,
    overlapped: u64,
    /// The rendered-block count when a control call first overlapped a render call.
    first_overlap: Option<u64>,
    failure: Option<String>,
    admission_block: u64,
    next_frame: u64,
    fed_blocks: u64,
    /// The frames fed so far, for a render thread that does not lock the control state.
    fed_frames: &'a AtomicU64,
    /// Offer the next structural transaction at once, not two blocks after the last admission
    /// (#1310 gate 5): an edit then lands while the last candidate may still be unadopted.
    back_to_back: bool,
    /// The control plane's withdrawal outcomes on this, the control, thread (#1310 D1):
    /// `(withdrawn, taken, superseded)`.
    outcomes: (u64, u64, u64),
}

impl Control<'_> {
    /// Whether every swap is admitted (or the race failed). The race goes on after this.
    fn is_done(&self) -> bool {
        self.admitted == self.swaps || self.failure.is_some()
    }

    /// Whether the race is over (#1405): every swap is admitted and either a control call has
    /// overlapped a render call or [`OVERLAP_BLOCKS`] blocks have rendered since the last
    /// admission, or the race failed.
    fn race_over(&self) -> bool {
        self.failure.is_some()
            || (self.is_done()
                && (self.overlapped > 0
                    || self.rendered.load(Ordering::Acquire)
                        >= self.admission_block + OVERLAP_BLOCKS))
    }

    fn fail(&mut self, failure: String) {
        self.failure = Some(failure);
        self.swapped.store(true, Ordering::Release);
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
                self.fed_frames.store(self.next_frame, Ordering::Release);
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
        // #1310 gate 5: back to back, every other step feeds only after its submit, so no block
        // can start while the command runs and the candidate before it stays unadopted
        // (`Withdrawn`); the others feed first, so render may adopt it during the command
        // (`Taken`).
        let starve = self.back_to_back && self.admitted % 2 == 1;
        self.step_feeding(!starve);
        if starve && !self.is_done() {
            let admitted = self.admitted;
            if let Err(refusal) = self.feed() {
                self.fail(format!("after {admitted} swaps: {refusal}"));
            }
        }
    }

    fn step_feeding(&mut self, feed: bool) {
        if self.done.load(Ordering::Acquire) {
            return;
        }
        let admitted = self.admitted;
        if feed && let Err(refusal) = self.feed() {
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
            if overlapping && self.in_render.load(Ordering::SeqCst) {
                self.overlapped += 1;
                self.first_overlap
                    .get_or_insert_with(|| self.rendered.load(Ordering::Acquire));
            }
        }
        let code = query(self.plan_address);
        if code != RESULT_OK {
            return self.fail(format!("plan resources after {admitted} swaps: {code}"));
        }
        // Every swap is admitted: keep feeding, dequeuing and querying until the race is over.
        if self.is_done() {
            if self.race_over() {
                self.done.store(true, Ordering::Release);
            }
            return;
        }
        // The block in flight at admission may predate the publication; the next one swaps.
        let blocks = self.rendered.load(Ordering::Acquire);
        if !self.back_to_back && blocks < self.admission_block + 2 {
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
                let counters = control_plane::test_lifecycle_counters();
                self.outcomes = (
                    counters.candidate_withdrawn,
                    counters.candidate_taken,
                    counters.candidate_superseded,
                );
                if self.is_done() {
                    self.swapped.store(true, Ordering::Release);
                }
                if self.race_over() {
                    self.done.store(true, Ordering::Release);
                }
            }
            // #1310 gate 5: a structural edit behind a pending candidate supersedes it, so any
            // `RESULT_BACKPRESSURE` here fails.
            other => self.fail(format!(
                "submit after {admitted} swaps and {} blocks: {other}",
                blocks - self.admission_block
            )),
        }
    }

    /// Whether the next block has its source PCM queued, or the race is over. Only a render
    /// drains the ring, so PCM queued here is still there when the render starts.
    fn ready(&self) -> bool {
        self.is_done()
            || self.done.load(Ordering::Acquire)
            || self.next_frame >= (self.rendered.load(Ordering::Acquire) + 1) * u64::from(QUANTUM)
    }
}

/// Runs the control side on a scoped thread, unlocked, while `render` renders on this thread, until
/// the race is over or `render` returns. Between steps the control thread yields while edits
/// remain and sleeps [`OVERLAP_SLEEP`] once every swap is admitted (#1405).
fn race_unpaced(control: &mut Control<'_>, render: impl FnOnce()) {
    let halt = AtomicBool::new(false);
    std::thread::scope(|scope| {
        let halt = &halt;
        let done = control.done;
        scope.spawn(move || {
            while !halt.load(Ordering::Acquire) && !done.load(Ordering::Acquire) {
                control.step();
                if control.is_done() {
                    std::thread::sleep(OVERLAP_SLEEP);
                } else {
                    std::thread::yield_now();
                }
            }
        });
        // Stops the control thread however `render` returns, so a failed assertion cannot hang.
        let _halt = StopOnDrop(halt);
        render();
    });
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
fn race_plan_swaps(swaps: u64, readers: usize, back_to_back: bool) {
    let _race = one_race_at_a_time();
    assert_installed();
    // Warm the JSON frontend's lazily boxed process statics before anything is observed.
    let _ = session::parse_session_json(r#"{"warm":0}"#);

    let mut model = session::parse_session_json(SESSION).expect("fixture");
    // A region the race never reaches the end of.
    model.sources[0].frames = 1 << 40;
    if back_to_back {
        // #1310 gate 5 compares the settled output with a fresh plan of the final model, so the
        // session holds no DSP state a swap could carry: no console slot, no insert, and the
        // input filters off.
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        for track in &mut model.tracks {
            track.console.clear();
            track.inserts.effects.clear();
            for lane in [&mut track.builtins.left, &mut track.builtins.right] {
                lane.hpf_hz = 0.0;
                lane.lpf_hz = 0.0;
            }
        }
    }
    let document = session::canonical_session_json(&model).expect("canonical");
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(&document, &limits()) };
    let plan_address = plan as usize;
    let stop = AtomicBool::new(false);
    let in_render = AtomicBool::new(false);
    let done = AtomicBool::new(false);
    let swapped = AtomicBool::new(false);
    let rendered = AtomicU64::new(0);
    let fed_frames = AtomicU64::new(0);
    let request = track_command(1, 42, &model, true);
    let mut control = Control {
        session_address: session as usize,
        plan_address,
        model,
        swaps,
        rendered: &rendered,
        in_render: &in_render,
        done: &done,
        swapped: &swapped,
        response: [0; 4_096],
        event: [0; 4_096],
        fields: [0; 64],
        revision: 42,
        request_id: 1,
        request,
        admitted: 0,
        overlapped: 0,
        first_overlap: None,
        failure: None,
        admission_block: 0,
        next_frame: 0,
        fed_blocks: 0,
        fed_frames: &fed_frames,
        back_to_back,
        outcomes: (0, 0, 0),
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
    // `WEDGE_BLOCKS`; blocks after the race ends render nothing. The bound also holds the
    // `OVERLAP_BLOCKS` after the last admission, which the unpaced loop adds.
    let block_bound = usize::try_from(swaps * (WEDGE_BLOCKS + 2)).expect("block bound");
    let overlap_bound = usize::try_from(OVERLAP_BLOCKS + 2).expect("overlap bound");
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
        // Stops the reader threads however the render loop leaves their scope, so a failed
        // assertion cannot hang the join. (`render_while_producing` stops its own producer thread.)
        let stop_readers = StopOnDrop(&stop);
        let render_next = || {
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
        };
        // Renders each block once the unlocked control thread has queued its PCM, until the race
        // is over or `blocks` blocks have been tried.
        let render_fed = |blocks: usize| {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
            for _ in 0..blocks {
                if done.load(Ordering::Acquire) {
                    break;
                }
                let block = rendered.load(Ordering::Acquire);
                while fed_frames.load(Ordering::Acquire) < (block + 1) * u64::from(QUANTUM)
                    && !done.load(Ordering::Acquire)
                {
                    assert!(
                        std::time::Instant::now() < deadline,
                        "block {block}: no PCM was queued"
                    );
                    std::thread::yield_now();
                }
                render_next();
            }
        };
        if back_to_back {
            // #1310 gate 5: the control thread runs unlocked, so a block renders whenever its PCM
            // is queued, concurrently with a command: render may adopt a candidate before the
            // command's service step (an ordinary rebuild), between it and the withdrawal
            // (`Taken`), or not before it (`Withdrawn`).
            race_unpaced(&mut control, || render_fed(block_bound + overlap_bound));
        } else {
            // The paced race until every swap is admitted; after that it renders nothing, and the
            // unpaced, sleeping race below runs until the race is over (#1405).
            render_while_producing(
                block_bound,
                &mut control,
                |control| {
                    if !control.is_done() {
                        control.step();
                    }
                },
                |control| control.ready(),
                |_| {
                    if !swapped.load(Ordering::Acquire) {
                        render_next();
                    }
                },
            );
            race_unpaced(&mut control, || render_fed(overlap_bound));
        }
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
    // #1310 gate 5: after quiescence the raced plan renders what a fresh plan of the final model
    // renders. The race toggles a muted track, so an even count of admissions ends on the boot
    // model; the feed is constant, so once every ramp has settled the block is the twin's.
    let settled = back_to_back.then(|| {
        let ramps = control.model.control_smoothing_samples();
        let settle = u64::from(ramps.fader.max(ramps.mute)).div_ceil(u64::from(QUANTUM)) + 2;
        let mut raced = Vec::new();
        let mut next_frame = control.next_frame;
        for block in blocks + 1..=blocks + settle {
            // SAFETY: The race has joined, so this thread is the session's and plan's only caller.
            unsafe {
                while next_frame < (block + 1) * u64::from(QUANTUM) {
                    let (code, accepted) =
                        submit_dc_block::<{ QUANTUM as usize }>(session, next_frame);
                    if code != RESULT_OK {
                        break;
                    }
                    next_frame += accepted;
                }
                assert_eq!(
                    miso_engine_v1_render_f32_planar(plan, block * u64::from(QUANTUM), &output),
                    RESULT_OK
                );
            }
            raced = pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>();
        }
        assert!(swaps.is_multiple_of(2), "the race ends on the boot model");
        // SAFETY: These handles are uniquely owned until the matching destroy calls below.
        let twin = unsafe {
            let (twin_session, twin_plan) = compile_c(&document, &limits());
            let mut twin_pcm = [f32::NAN; 256];
            let twin_output = PlanarOutput {
                samples: twin_pcm.as_mut_ptr(),
                ..output
            };
            for block in 0..=settle {
                let (code, _) = submit_dc_block::<{ QUANTUM as usize }>(
                    twin_session,
                    block * u64::from(QUANTUM),
                );
                assert_eq!(code, RESULT_OK);
                assert_eq!(
                    miso_engine_v1_render_f32_planar(
                        twin_plan,
                        block * u64::from(QUANTUM),
                        &twin_output
                    ),
                    RESULT_OK
                );
            }
            miso_engine_v1_session_destroy(twin_session);
            miso_engine_v1_plan_destroy(twin_plan);
            twin_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>()
        };
        (raced, twin)
    });
    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    println!(
        "{swaps} swaps: {} overlaps, the first at block {:?}; last admission at block {}, \
         {blocks} blocks",
        control.overlapped, control.first_overlap, control.admission_block
    );
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
    let (withdrawn, taken, superseded) = control.outcomes;
    assert_eq!(
        carry_counts,
        (control.admitted - superseded, 0),
        "every applied swap carried the source's ring; a superseded candidate never swapped"
    );
    assert_eq!(
        withdrawn, superseded,
        "every withdrawn candidate was superseded"
    );
    if let Some((raced, twin)) = settled {
        assert!(
            raced == twin,
            "the settled raced plan renders as a fresh plan of the final model: {:?} against {:?} \
             ({withdrawn} withdrawn, {taken} taken, carry {carry_counts:?}, blocks {blocks})",
            [f32::from_bits(raced[0]), f32::from_bits(raced[128])],
            [f32::from_bits(twin[0]), f32::from_bits(twin[128])],
        );
        println!("{swaps} back-to-back edits: {withdrawn} withdrawn, {taken} taken");
        assert!(
            withdrawn > 0 && taken > 0,
            "vacuous: {withdrawn} withdrawn and {taken} taken over {swaps} back-to-back edits"
        );
    } else {
        assert_eq!(
            superseded, 0,
            "two blocks between admissions adopt every candidate"
        );
    }
    // Back to back, one block is fed per admission (the last admission's step ends the race).
    let fed_floor = if back_to_back { swaps - 1 } else { swaps + 1 };
    assert!(
        control.fed_blocks >= fed_floor,
        "vacuous: {} source blocks fed across {swaps} swaps",
        control.fed_blocks
    );
    // #1405: after the last admission the race keeps rendering, feeding and dequeuing until a
    // control call overlaps a render call or `OVERLAP_BLOCKS` blocks have rendered, with the
    // control thread sleeping `OVERLAP_SLEEP` between steps. On correct code this fails only if
    // no timer wakeup of the control thread preempts the render thread inside a render call
    // across those 4096 blocks. A yielding control thread did not suffice on one CPU: a vacuous
    // run stayed vacuous for all 4096 blocks (57 of 150 pinned runs), so the chance of an overlap
    // belongs to a whole run, not to each block.
    assert!(
        control.overlapped > 0,
        "vacuous: no control call overlapped a render call"
    );
    // Back to back, a superseded candidate never renders a block of its own.
    assert!(
        blocks > swaps - superseded,
        "each adopted swap rendered its own block"
    );
}

/// Issue #1042, F1: on the unmodified base, a control call that fell between the swap at the start
/// of a render call and the `active_epoch` publication at its end returned `RESULT_INTERNAL`, and
/// every structural command after it returned `RESULT_BACKPRESSURE` for good.
#[test]
fn control_calls_racing_plan_swapping_renders_never_wedge_replacement() {
    race_plan_swaps(24, 0, false);
}

/// Issue #1042, attempt 2: `miso_engine_v1_plan_resources` may run on any thread, concurrently with
/// render and control. Reading the epoch before taking the report lock let a render publish the
/// next epoch and a control call remove the old row in between; the lookup then panicked while
/// holding the lock, which poisoned it, so every later control call returned `RESULT_INTERNAL`
/// (and a release build, with `panic = "abort"`, aborts).
#[test]
fn resource_queries_racing_plan_swaps_always_find_the_published_row() {
    race_plan_swaps(200, 2, false);
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
    // SAFETY: The caller guarantees a live session used by this thread alone.
    unsafe { commit_c_noting(session, request, revision, || {}) }
}

/// [`commit_c`] that calls `submitted` as soon as the submit call returns, before its response is
/// read: the commit's revision store has landed by then.
///
/// # Safety
///
/// As [`commit_c`].
unsafe fn commit_c_noting(
    session: *mut Session,
    request: &[u8],
    revision: u64,
    submitted: impl FnOnce(),
) -> u32 {
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
    submitted();
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

/// The stateless fixture cut to two strips whose levels the output shows apart: `eq0` reaches only
/// the output's left plane and `eq1` only its right plane (a route matrix with one diagonal
/// entry), so a block's last sample on each plane is that strip's fader level times its source
/// channel. `eq0` starts at 0 dB and `eq1` at -6 dB, the levels [`pair_command`]'s even edits set.
fn pair_document() -> String {
    let mut model = session::parse_session_json(&live_document()).expect("live fixture");
    model.tracks.truncate(2);
    model.routes.retain(|route| {
        matches!(&route.source, session::RouteSource::Track { track_id, .. }
            if model.tracks.iter().any(|track| &track.id == track_id))
    });
    assert_eq!(model.routes.len(), 2, "one route per kept strip");
    for (index, route) in model.routes.iter_mut().enumerate() {
        let on = |present: bool| f32::from(u8::from(present));
        route.channel_matrix = session::ChannelMatrix {
            ll: on(index == 0),
            lr: 0.0,
            rl: 0.0,
            rr: on(index == 1),
        };
    }
    model.tracks[1].fader.left_db = -6.0;
    model.tracks[1].fader.right_db = -6.0;
    session::canonical_session_json(&model).expect("canonical")
}

/// One live transaction that sets both strips of [`pair_document`] at once: edit `n` (counting
/// from 1) sets `eq0` to -6 dB and `eq1` to 0 dB when `n` is odd, and the reverse when it is even.
fn pair_command(request_id: u64, revision: u64, edit: u64) -> Vec<u8> {
    let (first, second) = if edit % 2 == 1 {
        (-6.0, 0.0)
    } else {
        (0.0, -6.0)
    };
    let edits: Vec<SessionEdit> = [("eq0", first), ("eq1", second)]
        .into_iter()
        .flat_map(|(id, db)| {
            protocol::control_core::fader_edits(
                &StableId::parse(id).expect("track ID"),
                &session::DualMonoFader {
                    left_db: db,
                    right_db: db,
                    left_mute: false,
                    right_mute: false,
                },
            )
        })
        .collect();
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
        .expect("pair command");
    bytes.truncate(len);
    bytes
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

/// The two steady states of [`pair_document`], measured on one thread: the last samples of a block
/// before any edit (`eq0` at 0 dB, `eq1` at -6 dB), and of the first block after edit 1, whose
/// ramps must end inside it. Edit 2 must return to the first state bit for bit.
fn pair_levels(document: &str) -> [[u32; 2]; 2] {
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
    let initial = block(0, &mut pcm);
    assert_eq!(block(1, &mut pcm), initial, "the document starts steady");
    let mut flipped = initial;
    for edit in 1..=2 {
        // SAFETY: as above.
        let code = unsafe {
            commit_c(
                session,
                &pair_command(edit, first + edit - 1, edit),
                first + edit - 1,
            )
        };
        assert_eq!(code, RESULT_OK);
        let state = block(2 * edit, &mut pcm);
        assert_eq!(
            block(2 * edit + 1, &mut pcm),
            state,
            "the ramps end within a block"
        );
        if edit == 1 {
            assert_ne!(state[0], initial[0], "edit 1 changes eq0's plane");
            assert_ne!(state[1], initial[1], "edit 1 changes eq1's plane");
            flipped = state;
        } else {
            assert_eq!(state, initial, "edit 2 restores the first state");
        }
    }
    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    [initial, flipped]
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
    /// When the latest render call began (set before `in_render`).
    render_began: AtomicU64,
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

/// Offsets per sweep: an edit lands its revision store about its phase, `0..PHASES`, steps along
/// its sweep. The wide sweep runs from a quarter of a render call before the call to the call's end.
const PHASES: u64 = 16;

/// The control side of the live race ([`a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it`]):
/// it feeds the source and, once the watermark covers the last edit, commits the next live fader
/// edit against a parked render call, so at most one edit is in flight and each advance covers
/// exactly one revision.
struct LiveControl<'a> {
    session_address: usize,
    plan_address: usize,
    edits: u64,
    /// Encodes edit `n` (the request ID, the expected revision, `n`) as one transaction.
    command: fn(u64, u64, u64) -> Vec<u8>,
    /// The sweeps of a revision store across a render call, each in 64ths of the call from its
    /// `from` to its `to`. Counting edits from 0 as `committed` does (edit `n` here is edit `n + 1`
    /// of `command`), edit `n` takes sweep `n % len` and, in it, the offset
    /// `from + (to - from) * (n / len % PHASES) / PHASES`.
    windows: &'static [(i64, i64)],
    /// The sweep whose stores are timed against the render call they landed in, if any: a
    /// store counts in `early` when the submit call returned within 1/16 of the commit's own
    /// duration (`commit_nanos`) of the call's start.
    timed_sweep: Option<usize>,
    timed: u64,
    early: u64,
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
        let sweep = self.committed as usize % self.windows.len();
        let (from, to) = self.windows[sweep];
        let phase = self.committed / self.windows.len() as u64 % PHASES;
        let span = i128::from(to - from);
        let target = (i128::from(from) * i128::from(render)
            + span * i128::from(render) * i128::from(phase) / i128::from(PHASES))
            / 64;
        let lead = i128::from(self.commit_nanos) - target;
        let request = (self.command)(self.committed + 1, self.revision, self.committed + 1);
        self.gate.want.store(false, Ordering::Relaxed);
        let now = self.gate.now();
        let start = now + u64::try_from(lead.max(1)).expect("positive");
        self.gate.start.store(start, Ordering::Release);
        let delay = u64::try_from((-lead).max(0)).expect("non-negative");
        while self.gate.now() < now + delay {
            core::hint::spin_loop();
        }
        let began = self.gate.now();
        let mut stored = 0;
        let gate = self.gate;
        // SAFETY: as above.
        let code =
            unsafe { commit_c_noting(session, &request, self.revision, || stored = gate.now()) };
        // The commit's last write, the revision store, fell inside a render call.
        let inside = self.gate.in_render.load(Ordering::SeqCst);
        self.commit_nanos = self.gate.now() - began;
        match code {
            RESULT_OK => {
                if self.timed_sweep == Some(sweep) {
                    self.timed += 1;
                    let call = self.gate.render_began.load(Ordering::SeqCst);
                    // A render call that began after the store is not the call it landed in. The
                    // window is scaled by the commit, not by the call: a defect needs the commit's
                    // first cell write before a strip's drain and its publish after the block's
                    // snapshot, which is a stretch of about the commit's cost, in debug as in
                    // release. The call's length is no measure of it (1/16 of a debug call is
                    // about ten times that stretch).
                    self.early +=
                        u64::from(stored >= call && stored - call < self.commit_nanos / 16);
                }
                self.overlapped += u64::from(inside);
                self.committed += 1;
                self.revision += 1;
            }
            RESULT_BACKPRESSURE => {}
            other => panic!("fader edit {}: {other}", self.committed + 1),
        }
    }
}

/// #1505, #1312, D15-2 and D15-17: under a racing render, one live transaction that changes two
/// strips lands in one block, and the watermark names that block, neither a block early nor a block
/// late. It is also the race that a watermark advance names a block that applied the edit it
/// reports (#1314 D3, #1502 D2): the single-strip form of that race is this test's second
/// assertion, so it is not repeated (a snapshot taken after the executor rendered turns this test
/// red at its first advance, as the single-strip test did).
///
/// Each commit is one transaction over [`pair_document`] that sets `eq0` and `eq1`, whose levels
/// the output's two planes show apart, to opposite levels, and the next commit waits until the
/// watermark covers the last, so each advance covers one revision. Per rendered block this thread
/// asserts that:
///
/// * the two planes' last samples are both the old state or both the new one, never a mix (a
///   transaction split across two blocks);
/// * a watermark advance to `r` at this block names this block's first sample, and the block shows
///   `r`'s state; and
/// * a block that shows a state different from the previous block's is the block the watermark
///   advanced at (a watermark published a block late).
///
/// Reading the cells under [`engine::realtime::LiveSnapshot::ALL`] instead of the block's snapshot,
/// or publishing the watermark a block late, turns the third assertion red. Half of the commits
/// sweep their revision store across the whole render call, from a quarter of a call before it to
/// its end, and half across the call's first few percent, where a store lands after the snapshot
/// and before the strips' drains. The run refuses to pass unless at least [`THIN_FLOOR`] of the
/// thin sweep's stores landed within 1/16 of a commit's duration of a call's start, the stretch in
/// which a commit's cell writes and its publish straddle a block's snapshot and drains, as well as
/// a tenth of all stores landing inside a call.
#[test]
fn a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it() {
    let _race = one_race_at_a_time();
    const EDITS: u64 = 640;
    let document = pair_document();
    let states = pair_levels(&document);
    assert_ne!(states[0], states[1]);

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
        render_began: AtomicU64::new(0),
        render_nanos: AtomicU64::new(0),
    };
    let rendered = AtomicU64::new(0);
    // The newest revision a block's watermark read returned.
    let last = AtomicU64::new(first);
    let mut control = LiveControl {
        session_address: session as usize,
        plan_address,
        edits: EDITS,
        command: pair_command,
        windows: &[(-16, 64), (-2, 6)],
        timed_sweep: Some(1),
        timed: 0,
        early: 0,
        committed: 0,
        revision: first,
        gate: &gate,
        commit_nanos: 0,
        overlapped: 0,
        next_frame: 0,
    };
    let block_bound = usize::try_from(EDITS * 64).expect("block bound");
    // The previous block's last samples; the document starts in the first state.
    let mut shown = states[0];
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
            gate.render_began.store(started, Ordering::SeqCst);
            gate.in_render.store(true, Ordering::SeqCst);
            // SAFETY: This thread is the plan's only renderer.
            let code = unsafe { render_live(plan_address as *mut Plan, start, &mut pcm) };
            gate.in_render.store(false, Ordering::SeqCst);
            gate.render_nanos
                .store(gate.now() - started, Ordering::Release);
            assert_eq!(code, RESULT_OK, "render refused block {block}");
            let watermark = watermark_c(plan_address).expect("the renderer never reads busy");
            let now_shown = last_samples(&pcm);
            assert!(
                states.contains(&now_shown),
                "block {block} mixes the two strips' states: {now_shown:08x?}, steady states \
                 {states:08x?}"
            );
            if watermark.revision != newest {
                let edit = watermark.revision - first;
                assert_eq!(watermark.revision, newest + 1, "one edit per advance");
                assert_eq!(
                    watermark.first_sample, start,
                    "an advance names the block that made it"
                );
                assert_eq!(
                    now_shown,
                    states[usize::from(edit % 2 == 1)],
                    "block {block}: the watermark advanced to edit {edit} in a block that does \
                     not show it"
                );
                last.store(watermark.revision, Ordering::Release);
            } else {
                assert_eq!(
                    now_shown, shown,
                    "block {block} shows a new state, but the watermark (revision {newest}) did \
                     not advance at it"
                );
            }
            shown = now_shown;
            rendered.store(block + 1, Ordering::Release);
        },
    );
    let (overlapped, timed, early) = (control.overlapped, control.timed, control.early);
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
    // The thin sweep aims about one phase in 16 of its stores into the stretch of 1/16 of a commit
    // after the call's start, and the rest reach it by jitter. A store counted here returned
    // within that stretch of the call's start, which is where a drain that reads outside the
    // snapshot is caught. The count follows the commit's cost, so it is small by nature (see
    // `THIN_FLOOR` for the measurements); a run below the floor did not sweep that stretch and
    // says nothing about the defect, so it fails loudly.
    assert!(
        early >= THIN_FLOOR,
        "vacuous: {early} of {timed} thin-sweep revision stores returned within 1/16 of a \
         commit's duration of their render call's start, fewer than {THIN_FLOOR}"
    );
}

/// #1310 gate 5: a control thread submitting 64 structural edits back to back while a render
/// thread renders. Each edit lands while the last candidate is pending or just adopted: every
/// submit commits (`RESULT_OK`, never `RESULT_BACKPRESSURE`), the run withdraws and supersedes some
/// candidates and finds others taken by render first, and after quiescence the plan renders what a
/// fresh plan of the final model renders.
///
/// Test value: red if a structural edit behind a pending candidate is refused, if the withdrawal
/// races render's claim into a lost or doubly owned plan (a failed render, a carry count that
/// misses a swap, or output other than the twin's), or if the run never reaches one of the two
/// outcomes it is judged by.
#[test]
fn structural_edits_back_to_back_supersede_or_swap_and_never_backpressure() {
    race_plan_swaps(64, 0, true);
}
