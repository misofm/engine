//! Issue #1570b gates 1 to 3: the browser serves every protocol command and event through the core,
//! and gives the same bytes as the C ABI.
//!
//! Both legs run natively in this process. The C ABI leg is the `capi` FFI, built with the limits
//! the browser's control half was built with (D6). The browser leg is `AudioWorkletEngineHost`
//! through the Rust functions the exports call: `control_command`, `control_event`, `service` and
//! `render_next`. Both boot the browser fixture `browser-v1/session.json` with its source extended
//! to 8,192 frames and are fed the same PCM; the one-script gate boots the observation fixture
//! (`browser-v1/observation-session.json`, the same session with a compressor insert) instead,
//! with its source extended to 16,384 frames, so its `Observe` step has a tap (#1589 D6).

#![cfg(feature = "test-support")]
#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use core::ffi::c_void;
use core::ptr;
use std::alloc::{GlobalAlloc, System};

use capi::{
    ABI_VERSION as C_ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits,
    ENGINE_CONFIG_SIZE, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput,
    RESULT_OK as C_RESULT_OK, SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SubmitReport,
};
use control_plane::ControlLimits;
use controls::{ControlAddress, Owner, Section};
use host_web::{
    AudioWorkletEngineHost, EVENT_LANE_LOSSY, EVENT_LANE_RELIABLE, RESULT_OK, WebBootOptions,
};
use protocol::{
    CommandPayload, ControlListRequest, CounterId, CountersRequest, DiagnosticSeverity,
    DiagnosticsRequest, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    SessionSnapshotRequest, TelemetryConfiguration, TransportSetRequest, TransportState,
    TypedCommandFrame,
};
use session::{ParameterChannel, StableId, canonical_session_json, parse_session_json};

mod common;
use common::{QUANTUM, Verdict, bits, decode_verdict, planes};

const SESSION: &str = include_str!("browser-v1/session.json");
const SOURCE_ID: &str = "fixture-source";
const TRACK_ID: &str = "track";
const RATE: u32 = 48_000;
const FADER_DB: u32 = 5;
/// The most `control_event` calls the control half's drain makes on a lane (D4).
const DRAIN_CALLS: usize = 16;

const MESSAGE_SESSION_COMMITTED: u16 = 0x8001;
const MESSAGE_TRANSPORT_STATE: u16 = 0x8010;
const MESSAGE_METER_BATCH: u16 = 0x8020;
const MESSAGE_COUNTER_SNAPSHOT: u16 = 0x8021;
const MESSAGE_DIAGNOSTIC: u16 = 0x8030;

/// The fixture at `frames` source frames.
fn document(frames: u64) -> String {
    let mut model = parse_session_json(SESSION).expect("the fixture parses");
    model.sources[0].frames = frames;
    canonical_session_json(&model).expect("canonical session")
}

/// The browser boots with the default meter window, the C ABI's at 48 kHz and 128 frames, so both
/// hosts serve one meter table on one window grid (#1588 D5).
fn options() -> WebBootOptions {
    WebBootOptions {
        require_sample_rate_hz: RATE,
        require_quantum_frames: QUANTUM,
        source_ring_frames: QUANTUM * 2,
        ..WebBootOptions::live_control_defaults()
    }
}

/// #1588 D5's meter table: the output's peak and the track's peak and gain reduction (#1587b).
fn meter_table() -> Vec<protocol::MeterEntry> {
    let strip = || protocol::MeterTarget::Strip(TRACK_ID.into());
    vec![
        protocol::MeterEntry {
            target: protocol::MeterTarget::Output,
            metric: protocol::MeterMetric::Peak,
        },
        protocol::MeterEntry {
            target: strip(),
            metric: protocol::MeterMetric::Peak,
        },
        protocol::MeterEntry {
            target: strip(),
            metric: protocol::MeterMetric::GainReduction,
        },
    ]
}

/// The meter period of the script's table: two windows of the default meter window.
const METER_PERIOD_BLOCKS: u32 = 2 * host_web::DEFAULT_METER_BLOCKS;

fn boot_browser(frames: u64) -> AudioWorkletEngineHost {
    AudioWorkletEngineHost::boot(document(frames).as_bytes(), options())
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())))
}

/// One command frame.
fn frame(request: u64, expected: ExpectedRevision, payload: CommandPayload<'_>) -> Vec<u8> {
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

fn fader_edit(db: f32) -> [SessionEdit; 1] {
    [SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner: Owner::Strip(StableId::parse(TRACK_ID).expect("track ID")),
            section: Section::Builtins,
            control: FADER_DB,
            lane: ParameterChannel::Both,
        },
        value: db,
        ramp_samples: None,
    }]
}

/// A structural transaction: a muted copy of the fixture's track, whose ID sorts before it, and
/// its route to the output.
fn added_muted_track() -> [SessionEdit; 2] {
    const ADDED: &str = "added";
    let model = parse_session_json(SESSION).expect("the fixture parses");
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse(ADDED).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = StableId::parse(&format!("{ADDED}-main")).expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

/// The message ID of an event frame, after checking it is an event frame.
fn event_message(event: &[u8]) -> u16 {
    assert_eq!(event[14], 3, "an event frame");
    u16::from_le_bytes([event[16], event[17]])
}

/// D4's drain over one host's event entry: the reliable lane until a call returns 0 bytes, then the
/// lossy lane the same way, at most [`DRAIN_CALLS`] calls per lane.
fn drain(mut next: impl FnMut(u32) -> (u32, Vec<u8>)) -> Vec<(u32, Vec<u8>)> {
    let mut events = Vec::new();
    for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
        for _ in 0..DRAIN_CALLS {
            let (result, event) = next(lane);
            assert_eq!(result, RESULT_OK, "lane {lane}");
            if event.is_empty() {
                break;
            }
            events.push((lane, event));
        }
    }
    events
}

// ---------------------------------------------------------------------------------------------
// The thread-scoped allocation counter
// ---------------------------------------------------------------------------------------------

thread_local! {
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

/// The process's allocator: `System`, counting each thread's own allocations and frees. A host
/// adapter may not depend on `bench-support` (`scripts/check-bench-policy.sh` bans it under
/// `hosts/`), so this test counts as `boot_transient_budget.rs` does.
struct CountingAllocator;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn counted(counter: &'static std::thread::LocalKey<Cell<u64>>) {
    let _ = counter.try_with(|count| count.set(count.get() + 1));
}

// SAFETY: every request is forwarded unchanged to `System`; the thread-local counters observe the
// operations but never alter a pointer, size, alignment or lifetime.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        counted(&ALLOCATIONS);
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        counted(&ALLOCATIONS);
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        counted(&DEALLOCATIONS);
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        counted(&ALLOCATIONS);
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

/// This thread's allocations and frees so far.
fn thread_counts() -> (u64, u64) {
    (ALLOCATIONS.with(Cell::get), DEALLOCATIONS.with(Cell::get))
}

/// Fails loudly if the counting allocator is not the process's: a heap allocation must move it.
fn assert_installed() {
    let before = thread_counts();
    let probe = Vec::<u8>::with_capacity(64);
    let after = thread_counts();
    drop(probe);
    assert!(
        after.0 > before.0,
        "the counting allocator is not installed in this process"
    );
}

// ---------------------------------------------------------------------------------------------
// The browser leg
// ---------------------------------------------------------------------------------------------

struct Browser {
    host: AudioWorkletEngineHost,
}

impl Browser {
    fn new(frames: u64) -> Self {
        Self {
            host: boot_browser(frames),
        }
    }

    /// Stage and run one frame. Returns the export's result and the response bytes (the response
    /// frame after `RESULT_OK`, else the refusal's diagnostic).
    fn send(&mut self, frame: &[u8]) -> (u32, Vec<u8>) {
        let length = u32::try_from(frame.len()).expect("frame length");
        self.host
            .control_request_mut(length)
            .expect("the request staging")[..frame.len()]
            .copy_from_slice(frame);
        let result = self.host.control_command(length);
        (result, self.host.control_response().to_vec())
    }

    fn event(&mut self, lane: u32) -> (u32, Vec<u8>) {
        let result = self.host.control_event(lane);
        (result, self.host.control_response().to_vec())
    }

    fn drain(&mut self) -> Vec<(u32, Vec<u8>)> {
        drain(|lane| self.event(lane))
    }

    fn feed(&mut self, block: u64) {
        let [left, right] = planes(0, block);
        assert_eq!(
            self.host.submit_source(
                SOURCE_ID.as_bytes(),
                1,
                block * u64::from(QUANTUM),
                RATE,
                &[&left, &right],
                QUANTUM,
                false,
            ),
            RESULT_OK,
            "browser source"
        );
    }

    fn step(&mut self, block: u64) -> Vec<u32> {
        self.feed(block);
        assert_eq!(self.host.render_next(), RESULT_OK, "browser render");
        bits(self.host.output_pcm().expect("browser output"))
    }

    fn service(&mut self) {
        assert_eq!(self.host.service(), RESULT_OK, "browser service");
    }
}

// ---------------------------------------------------------------------------------------------
// The C ABI leg
// ---------------------------------------------------------------------------------------------

/// The C ABI's compile limits from the browser's own, field for field (D6).
fn compile_limits(limits: ControlLimits) -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: limits.source_ring_frames,
        reserved1: 0,
        reserved0: 0,
        maximum_document_bytes: limits.maximum_document_bytes,
        maximum_diagnostic_bytes: limits.maximum_diagnostic_bytes,
        maximum_tracks: limits.maximum_tracks,
        maximum_sources: limits.maximum_sources,
        maximum_routes: limits.maximum_routes,
        maximum_effects: limits.maximum_effects,
        maximum_graph_session_plus_plan_bytes: limits.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: limits.maximum_source_total_bytes,
        maximum_source_overhead_bytes: limits.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: limits.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: limits.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: limits.maximum_builtin_retained_bytes,
        maximum_capi_retained_bytes: limits.maximum_control_retained_bytes,
        maximum_named_allocation_bytes: limits.maximum_named_allocation_bytes,
        maximum_meter_streams: limits.maximum_meter_streams,
        maximum_meter_items: limits.maximum_meter_items,
        maximum_meter_bytes: limits.maximum_meter_bytes,
        maximum_control_frame_bytes: limits.maximum_control_frame_bytes,
        maximum_replay_bytes: limits.maximum_replay_bytes,
        maximum_replay_entries: limits.maximum_replay_entries,
        maximum_submixes: limits.maximum_submixes,
        maximum_vcas: limits.maximum_vcas,
        reserved: [0; 2],
    }
}

struct CAbi {
    session: *mut Session,
    plan: *mut Plan,
    frame_capacity: usize,
}

impl CAbi {
    fn new(frames: u64, limits: ControlLimits) -> Self {
        Self::from_document(&document(frames), limits)
    }

    /// [`Self::new`] for `document`.
    fn from_document(document: &str, limits: ControlLimits) -> Self {
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: C_ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        let limits = compile_limits(limits);
        // SAFETY: Every descriptor and output location lives for the complete call, and the engine
        // handle is destroyed once after compilation, as the C ABI documents.
        unsafe {
            assert_eq!(
                capi::miso_engine_v1_engine_create(&config, &mut engine),
                C_RESULT_OK
            );
            assert_eq!(
                capi::miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &limits,
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                C_RESULT_OK,
                "{}",
                String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize])
            );
            capi::miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            frame_capacity: usize::try_from(limits.maximum_control_frame_bytes)
                .expect("frame ceiling"),
        }
    }

    fn last_error(&self) -> Vec<u8> {
        let mut text = vec![0_u8; 1_024];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: text.as_mut_ptr(),
            capacity_bytes: text.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and `out` is owned storage for the call.
        let code = unsafe {
            capi::miso_engine_v1_last_error(self.session.cast::<c_void>().cast_const(), &mut out)
        };
        assert_eq!(code, C_RESULT_OK);
        text.truncate(out.required_bytes as usize);
        text
    }

    /// One frame. Returns the result and the response bytes: the response frame after
    /// `RESULT_OK`, else the session's diagnostic text, as the browser leg reports a failure.
    fn send(&mut self, request: &[u8]) -> (u32, Vec<u8>) {
        let mut response = vec![0_u8; self.frame_capacity];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; the frame and `out` are owned
        // storage that outlives the synchronous call.
        let code = unsafe {
            capi::miso_engine_v1_submit_command(
                self.session,
                request.as_ptr(),
                request.len() as u64,
                &mut out,
            )
        };
        if code == C_RESULT_OK {
            response.truncate(out.required_bytes as usize);
            (code, response)
        } else {
            (code, self.last_error())
        }
    }

    fn event(&mut self, lane: u32) -> (u32, Vec<u8>) {
        let mut event = vec![0_u8; self.frame_capacity];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: event.as_mut_ptr(),
            capacity_bytes: event.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; `out` is owned storage.
        let code = unsafe { capi::miso_engine_v1_dequeue_event(self.session, lane, &mut out) };
        event.truncate(out.required_bytes as usize);
        (code, event)
    }

    fn drain(&mut self) -> Vec<(u32, Vec<u8>)> {
        drain(|lane| self.event(lane))
    }

    fn service(&mut self) {
        // SAFETY: The session is live and used by this thread alone.
        let code = unsafe { capi::miso_engine_v1_service(self.session) };
        assert_eq!(code, C_RESULT_OK, "C ABI service");
    }

    fn feed(&mut self, block: u64) {
        let [left, right] = planes(0, block);
        let pointers = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: RATE,
            generation: 1,
            start_frame: block * u64::from(QUANTUM),
            planes: pointers.as_ptr(),
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
        // SAFETY: The session is live and used by this thread alone; the chunk's planes and the
        // report are owned storage that outlives the synchronous call.
        let code = unsafe {
            capi::miso_engine_v1_source_submit_planar_f32(
                self.session,
                SOURCE_ID.as_ptr(),
                SOURCE_ID.len() as u64,
                &chunk,
                &mut report,
            )
        };
        assert_eq!(code, C_RESULT_OK, "C ABI source");
        assert_eq!(report.accepted_frames, u64::from(QUANTUM));
    }

    /// Render block `block` into `pcm` and return the call's result.
    fn render(&mut self, block: u64, pcm: &mut [f32]) -> u32 {
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM,
            plane_stride_samples: QUANTUM,
            reserved: [0; 2],
        };
        // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
        unsafe {
            capi::miso_engine_v1_render_f32_planar(self.plan, block * u64::from(QUANTUM), &output)
        }
    }

    fn step(&mut self, block: u64) -> Vec<u32> {
        self.feed(block);
        let mut pcm = vec![f32::NAN; QUANTUM as usize * 2];
        assert_eq!(self.render(block, &mut pcm), C_RESULT_OK, "C ABI render");
        bits(&pcm)
    }
}

impl Drop for CAbi {
    fn drop(&mut self) {
        // SAFETY: Each handle is live, unique and destroyed once; the plan goes first.
        unsafe {
            capi::miso_engine_v1_plan_destroy(self.plan);
            capi::miso_engine_v1_session_destroy(self.session);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 1
// ---------------------------------------------------------------------------------------------

/// The two legs and the script's bookkeeping.
struct Legs {
    browser: Browser,
    c: CAbi,
    revision: u64,
    request: u64,
    events: [Vec<(u32, Vec<u8>)>; 2],
}

impl Legs {
    /// Both legs on the observation fixture (#1589 D6), its source extended to 16,384 frames, the
    /// browser booted with observation taps for every declared tap (#1589 R1), so both serve the
    /// `Observe` edit and `OBSERVATION_READ`.
    fn new() -> Self {
        let mut model = parse_session_json(OBSERVATION_SESSION).expect("the fixture parses");
        model.sources[0].frames = 16_384;
        let document = canonical_session_json(&model).expect("canonical session");
        let browser = Browser {
            host: AudioWorkletEngineHost::boot(document.as_bytes(), options()).unwrap_or_else(
                |failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())),
            ),
        };
        let limits = browser
            .host
            .control_limits_for_test()
            .expect("the control half's limits");
        let c = CAbi::from_document(&document, limits);
        let revision = browser.host.committed_revision();
        Self {
            browser,
            c,
            revision,
            request: 0,
            events: [Vec::new(), Vec::new()],
        }
    }

    fn next(&mut self) -> u64 {
        self.request += 1;
        self.request
    }

    /// Send `frame` to both legs and require the same result and the same bytes.
    fn both(&mut self, step: &str, frame: &[u8]) -> Vec<u8> {
        let (browser_result, browser_response) = self.browser.send(frame);
        let (c_result, c_response) = self.c.send(frame);
        assert_eq!(browser_result, c_result, "{step}: the result codes");
        assert_eq!(
            browser_response, c_response,
            "{step}: the response bytes (or the refusal text)"
        );
        browser_response
    }

    /// A command at the committed revision when it requires one, else with no precondition.
    fn command(&mut self, step: &str, payload: CommandPayload<'_>) -> Vec<u8> {
        let request = self.next();
        let expected = if matches!(
            payload,
            CommandPayload::SessionTransactionApply(_)
                | CommandPayload::TransportSet(_)
                | CommandPayload::TelemetryConfigure(_)
        ) {
            ExpectedRevision::Exact(SessionRevision(self.revision))
        } else {
            ExpectedRevision::Any
        };
        let bytes = frame(request, expected, payload);
        self.both(step, &bytes)
    }

    /// Drain both legs in D4's order and require equal event sequences.
    fn drain(&mut self, step: &str) {
        let browser = self.browser.drain();
        let c = self.c.drain();
        assert_eq!(browser, c, "{step}: the drained events, lane and bytes");
        self.events[0].extend(browser);
    }

    fn block(&mut self, block: u64) {
        let browser = self.browser.step(block);
        let c = self.c.step(block);
        assert_eq!(browser, c, "block {block}: the output is bit-identical");
        self.browser.service();
        self.c.service();
    }
}

/// The `CAPABILITIES_GET` reply `reply` with `added` merged into its ascending command list and
/// `flags` set, re-encoded by the codec so every length field matches. The browser's list must not
/// hold an added ID already.
fn capabilities_plus(reply: &[u8], added: &[u16], flags: u64) -> Vec<u8> {
    let codec = protocol::ProtocolCodec::default();
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        header,
        payload: protocol::DecodedSuccessResponsePayload::Capabilities(value),
    } = codec
        .decode_typed_response(reply, &mut protocol::DecodeScratch::new(&mut fields))
        .expect("a capabilities reply")
    else {
        panic!("a capabilities reply")
    };
    let ids = |bytes: &[u8]| {
        bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>()
    };
    let mut commands = ids(value.supported_commands);
    assert!(added.iter().all(|id| !commands.contains(id)));
    commands.extend_from_slice(added);
    commands.sort_unstable();
    let events = ids(value.supported_events);
    let capabilities = protocol::Capabilities {
        minimum_version: value.minimum_version,
        maximum_version: value.maximum_version,
        maximum_frame_bytes: value.maximum_frame_bytes,
        maximum_tlvs: value.maximum_tlvs,
        maximum_string_bytes: value.maximum_string_bytes,
        maximum_nesting: value.maximum_nesting,
        control_command_slots: value.control_command_slots,
        control_command_bytes: value.control_command_bytes,
        reliable_response_slots: value.reliable_response_slots,
        reliable_event_slots: value.reliable_event_slots,
        telemetry_slots: value.telemetry_slots,
        replay_entries: value.replay_entries,
        replay_bytes: value.replay_bytes,
        maximum_cached_response_bytes: value.maximum_cached_response_bytes,
        maximum_control_page_items: value.maximum_control_page_items,
        maximum_diagnostic_page_items: value.maximum_diagnostic_page_items,
        maximum_telemetry_handles: value.maximum_telemetry_handles,
        maximum_transaction_edits: value.maximum_transaction_edits,
        supported_commands: &commands,
        supported_events: &events,
        flags: protocol::CapabilityFlags(value.flags.0 | flags),
    };
    let mut out = vec![0_u8; reply.len() + 64];
    let length = codec
        .encode_success_response_frame_into(
            &protocol::TypedSuccessResponseFrame {
                request_id: header.request_id,
                revision: header.revision,
                payload: protocol::SuccessResponsePayload::Capabilities(capabilities),
            },
            &mut out,
        )
        .expect("a capabilities frame");
    out.truncate(length);
    out
}

/// Test value: red if any command or event takes a browser-only path, if the browser publishes no
/// render activity (no telemetry events, a wrong `effective_sample`), if the two hosts are built
/// with different protocol limits, or if the browser answers an `Observe` arm or an
/// `OBSERVATION_READ` otherwise than the C ABI (#1589 D6: a reader render does not publish to, or
/// an index that does not reach the browser's readers), or if a `LOUDNESS` reply differs between the
/// hosts or delivers no hop (#2115).
#[test]
fn the_browser_and_the_c_abi_give_the_same_bytes_for_one_script() {
    let mut legs = Legs::new();

    // 1. CAPABILITIES_GET. Both hosts serve `LOUDNESS` (`0x0014`) with flag bit 17 (#2115),
    // `SESSION_REPLACE` (`0x0013`, #1697), the three
    // spectrum commands (`0x0010`-`0x0012`) with flag bit 16 (#1596) and, since #1589a,
    // `OBSERVATION_READ` (`0x000f`) with flag bit 15 (#1586 R2), and size their lossy meter lane
    // to one whole meter batch (#1587a, #1588). The whole reply is byte-equal.
    let request = legs.next();
    let bytes = frame(
        request,
        ExpectedRevision::Any,
        CommandPayload::CapabilitiesGet,
    );
    let (browser_result, browser_reply) = legs.browser.send(&bytes);
    let (c_result, c_reply) = legs.c.send(&bytes);
    assert_eq!(browser_result, c_result, "1 capabilities: the result codes");
    assert_eq!(
        capabilities_plus(&browser_reply, &[], 0),
        browser_reply,
        "1 capabilities: the browser's reply re-encodes to itself"
    );
    assert_eq!(
        browser_reply, c_reply,
        "1 capabilities: the C ABI's reply is the browser's"
    );

    // 2. SESSION_SNAPSHOT_GET at offset 0 with 64 bytes, then continuation pages until eof.
    let mut offset = 0_u64;
    loop {
        let response = legs.command(
            "2 snapshot",
            CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                offset,
                maximum_bytes: 64,
            }),
        );
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(chunk),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut protocol::DecodeScratch::new(&mut fields))
            .expect("a snapshot chunk")
        else {
            panic!("expected a snapshot chunk")
        };
        offset += chunk.canonical_json_chunk.len() as u64;
        if chunk.eof {
            break;
        }
    }
    assert!(offset > 64, "the snapshot took more than one page");

    // 3. TRANSPORT_GET. Kept for the replay in step 14.
    let transport_request = legs.next();
    let transport_get = frame(
        transport_request,
        ExpectedRevision::Any,
        CommandPayload::TransportGet,
    );
    legs.both("3 transport get", &transport_get);

    // 4. TRANSPORT_SET to playing.
    legs.command(
        "4 transport set",
        CommandPayload::TransportSet(TransportSetRequest {
            state: TransportState::Playing,
        }),
    );
    legs.drain("after 4");

    // 5. TELEMETRY_CONFIGURE: #1588 D5's meter table at two windows a batch,
    // LIVE_VALUES_SUPERSEDED at period 1, diagnostics on at Info. Both echoes carry the meter
    // window (field 8).
    legs.command(
        "5 telemetry",
        CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
            meters: meter_table(),
            meter_period_blocks: METER_PERIOD_BLOCKS,
            meter_window_blocks: None,
            counter_ids: vec![CounterId::LiveValuesSuperseded],
            counter_period_blocks: 1,
            diagnostics_enabled: true,
            minimum_diagnostic_severity: DiagnosticSeverity::Info,
        }),
    );
    legs.drain("after 5");

    // 6. CONTROL_LIST_GET, first page.
    legs.command(
        "6 control list",
        CommandPayload::ControlListGet(&ControlListRequest {
            structure_epoch: 0,
            after_handle: 0,
            limit: 256,
            owner: None,
            headers_only: false,
        }),
    );

    // 7. One SESSION_TRANSACTION_APPLY with one SET_CONTROL fader edit.
    let request = legs.next();
    let revision = legs.revision;
    let edit = fader_edit(-6.0);
    let response = legs.both(
        "7 transaction",
        &frame(
            request,
            ExpectedRevision::Exact(SessionRevision(revision)),
            CommandPayload::SessionTransactionApply(&edit),
        ),
    );
    let Verdict::Applied { revision, .. } = decode_verdict(&response) else {
        panic!("the fader edit is refused")
    };
    legs.revision = revision;
    legs.drain("after 7");

    // 8. Three meter batches' render blocks, with a service step and a drain after each, so every
    // `METER_BATCH` is compared where it is staged. After the first batch a structural
    // transaction adds a muted track whose ID sorts first, so the batches after its adoption come
    // from the successor plan's ring with the metered track at another strip index (#1588 D3).
    for block in 0..3 * u64::from(METER_PERIOD_BLOCKS) {
        if block == u64::from(METER_PERIOD_BLOCKS) {
            let request = legs.next();
            let edits = added_muted_track();
            let response = legs.both(
                "8 structural transaction",
                &frame(
                    request,
                    ExpectedRevision::Exact(SessionRevision(legs.revision)),
                    CommandPayload::SessionTransactionApply(&edits),
                ),
            );
            let Verdict::Applied { revision, .. } = decode_verdict(&response) else {
                panic!("the structural transaction is refused")
            };
            legs.revision = revision;
            legs.drain("after 8's structural transaction");
        }
        legs.block(block);
        legs.drain(&format!("after block {block}"));
    }

    // 9. COUNTERS_GET for all counters.
    legs.command(
        "9 counters",
        CommandPayload::CountersGet(&CountersRequest {
            all: true,
            ids: Vec::new(),
        }),
    );

    // 10. DIAGNOSTICS_GET after sequence 0, limit 16, at Info.
    legs.command(
        "10 diagnostics",
        CommandPayload::DiagnosticsGet(DiagnosticsRequest {
            after_sequence: 0,
            limit: 16,
            minimum_severity: DiagnosticSeverity::Info,
        }),
    );

    // 11. A frame with message ID 0x0006, the retired `AUTOMATION_ENQUEUE` (#1627): refused as an
    // unsupported message at decode on both hosts, before any state moves.
    let mut retired_automation = frame(
        legs.next(),
        ExpectedRevision::Any,
        CommandPayload::TransportGet,
    );
    retired_automation[16..18].copy_from_slice(&0x0006_u16.to_le_bytes());
    legs.both("11 message 0x0006", &retired_automation);

    // 12. A frame with message ID 0x0004.
    let mut unknown_message = frame(
        legs.next(),
        ExpectedRevision::Any,
        CommandPayload::TransportGet,
    );
    unknown_message[16..18].copy_from_slice(&0x0004_u16.to_le_bytes());
    legs.both("12 message 0x0004", &unknown_message);

    // 13. A frame with a wrong magic.
    let mut wrong_magic = frame(
        legs.next(),
        ExpectedRevision::Any,
        CommandPayload::TransportGet,
    );
    wrong_magic[0] = 0;
    legs.both("13 wrong magic", &wrong_magic);

    // 14. Step 3's exact bytes again: a replay.
    legs.both("14 replay", &transport_get);

    // 15. Step 3's request ID with different bytes.
    let reused = frame(
        transport_request,
        ExpectedRevision::Exact(SessionRevision(legs.revision)),
        CommandPayload::TransportGet,
    );
    legs.both("15 request ID reuse", &reused);
    legs.drain("after 15");

    // 16. One `Observe` arm of the compressor insert's gain-reduction tap with a one-block window
    // (#1589 D6): the same reply, its arm tag included, on both hosts.
    let strip = StableId::parse(TRACK_ID).expect("track ID");
    let comp = StableId::parse("comp").expect("insert ID");
    let arm = [SessionEdit::Observe {
        strip_id: strip.clone(),
        section: controls::TapSection::Inserts(comp.clone()),
        tap_id: 1,
        armed: true,
        window_blocks: 1,
        arm_tag: 0,
    }];
    let response = legs.command("16 observe", CommandPayload::SessionTransactionApply(&arm));
    let Verdict::Applied { revision, .. } = decode_verdict(&response) else {
        panic!("the observe edit is refused")
    };
    legs.revision = revision;
    legs.drain("after 16");

    // 17. Two render blocks, after step 8's last: the first applies the arm, and its window
    // closes with it.
    let next = 3 * u64::from(METER_PERIOD_BLOCKS);
    for block in next..next + 2 {
        legs.block(block);
    }

    // 18. One OBSERVATION_READ of the tap: the same page on both hosts, and a ready row, so an
    // armed reader that render publishes to answers it on both.
    let read = protocol::ObservationReadRequest {
        selections: vec![protocol::ObservationReadSelection {
            strip_id: strip,
            section: controls::TapSection::Inserts(comp),
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        }],
    };
    let response = legs.command(
        "18 observation read",
        CommandPayload::ObservationRead(&read),
    );
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::ObservationRead(page),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut protocol::DecodeScratch::new(&mut fields))
        .expect("a response frame")
    else {
        panic!("18 observation read: an observation read page");
    };
    assert_eq!(
        (page.rows[0].status, page.rows[0].armed),
        (protocol::ObservationRowStatus::Ready, true),
        "18 observation read: the armed tap's window is ready"
    );

    // 19. Spectrum through the frames (#1596 gate 6): one `SPECTRUM_CONFIGURE` for the track's
    // post-pan tap, two render blocks for the adoption and the promotion, a `SPECTRUM_CAPTURE`
    // arm, one 2,048-frame window of render blocks, then the read of both channels: the same
    // bytes on both legs, so the analyzed bins are equal word for word.
    legs.command(
        "19 spectrum configure",
        CommandPayload::SpectrumConfigure(&protocol::SpectrumConfigureRequest {
            form: protocol::SpectrumForm::Single,
            entries: vec![protocol::SpectrumEntry {
                target_kind: protocol::SpectrumTargetKind::TrackPostPan,
                target_id: TRACK_ID.to_owned(),
                channels: protocol::ParameterChannel::Both,
            }],
            maximum_capture_bytes: 1 << 20,
            hop_frames: None,
        }),
    );
    // The blocks continue from step 17's last.
    for block in next + 2..next + 4 {
        legs.block(block);
    }
    legs.command(
        "19 spectrum arm",
        CommandPayload::SpectrumCapture(&protocol::SpectrumCaptureRequest::Arm),
    );
    for block in next + 4..next + 20 {
        legs.block(block);
    }
    let read = legs.command(
        "19 spectrum read",
        CommandPayload::SpectrumCapture(&protocol::SpectrumCaptureRequest::Read {
            channels: protocol::ParameterChannel::Both,
            include_window: false,
        }),
    );
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::SpectrumCaptured(reply),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&read, &mut protocol::DecodeScratch::new(&mut fields))
        .expect("19 spectrum read: a response frame")
    else {
        panic!("19 spectrum read: a capture reply")
    };
    assert_eq!(
        reply.status,
        protocol::SpectrumCaptureStatus::Ready,
        "19 spectrum read: vacuous without a ready window"
    );

    legs.drain("at the end");

    // The browser leg published every kind of protocol event it serves, and the C ABI leg's equal
    // sequence says the same: step 8's three meter batches among them (#1588 gate 1).
    let kinds: Vec<u16> = legs.events[0]
        .iter()
        .map(|(_, event)| event_message(event))
        .collect();
    for (name, message) in [
        ("SESSION_COMMITTED", MESSAGE_SESSION_COMMITTED),
        ("TRANSPORT_STATE", MESSAGE_TRANSPORT_STATE),
        ("COUNTER_SNAPSHOT", MESSAGE_COUNTER_SNAPSHOT),
        ("DIAGNOSTIC", MESSAGE_DIAGNOSTIC),
    ] {
        assert!(kinds.contains(&message), "no {name} event in {kinds:04x?}");
    }
    assert_eq!(
        kinds
            .iter()
            .filter(|&&message| message == MESSAGE_METER_BATCH)
            .count(),
        3,
        "step 8's three METER_BATCH events in {kinds:04x?}"
    );

    // 20. The output loudness meter (#2115): a `LOUDNESS` start, sixteen render blocks (four
    // 10 ms hops), then a read: the same bytes on both legs, so both hosts serve one core meter.
    let start = legs.command(
        "20 loudness start",
        CommandPayload::Loudness(protocol::LoudnessRequest::Start),
    );
    for block in next + 20..next + 36 {
        legs.block(block);
    }
    let read = legs.command(
        "20 loudness read",
        CommandPayload::Loudness(protocol::LoudnessRequest::Read),
    );
    let loudness = |step: &str, frame: &[u8]| {
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(frame, &mut protocol::DecodeScratch::new(&mut fields))
        {
            Ok(protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::Loudness(reply),
                ..
            }) => reply,
            _ => panic!("{step}: a loudness reply"),
        }
    };
    let (start, read) = (
        loudness("20 loudness start", &start),
        loudness("20 loudness read", &read),
    );
    assert!(
        read.armed && read.observed_sample >= start.observed_sample + 4 * 480,
        "20 loudness read: vacuous without delivered hops"
    );

    legs.drain("after 20");
}

// ---------------------------------------------------------------------------------------------
// Gate 2
// ---------------------------------------------------------------------------------------------

/// One fader transaction at the committed revision. A refusal is a non-ok response frame in an ok
/// call, as on the C ABI.
fn transaction(browser: &mut Browser, request: u64, revision: &mut u64, db: f32) -> Verdict {
    let edit = fader_edit(db);
    let (result, response) = browser.send(&frame(
        request,
        ExpectedRevision::Exact(SessionRevision(*revision)),
        CommandPayload::SessionTransactionApply(&edit),
    ));
    assert_eq!(result, RESULT_OK, "transaction {request}");
    let verdict = decode_verdict(&response);
    if let Verdict::Applied { revision: next, .. } = &verdict {
        *revision = *next;
    }
    verdict
}

/// Test value: red if the control half stops draining its event lanes, so the browser refuses its
/// third commit. The undrained boot shows what the drain is for.
#[test]
fn draining_the_event_lanes_lets_the_browser_commit_without_end() {
    let mut browser = Browser::new(8_192);
    let mut revision = browser.host.committed_revision();
    for request in 1..=20 {
        let db = if request % 2 == 0 { -6.0 } else { -3.0 };
        assert!(
            matches!(
                transaction(&mut browser, request, &mut revision, db),
                Verdict::Applied { .. }
            ),
            "transaction {request} commits"
        );
        let drained = browser.drain();
        assert_eq!(drained.len(), 1, "transaction {request}: its commit event");
        assert_eq!(event_message(&drained[0].1), MESSAGE_SESSION_COMMITTED);
    }

    let mut undrained = Browser::new(8_192);
    let mut revision = undrained.host.committed_revision();
    for request in 1..=2 {
        assert!(
            matches!(
                transaction(
                    &mut undrained,
                    request,
                    &mut revision,
                    -6.0 + request as f32
                ),
                Verdict::Applied { .. }
            ),
            "transaction {request} commits into the empty lane"
        );
    }
    let Verdict::Refused {
        codes,
        revision: kept,
    } = transaction(&mut undrained, 3, &mut revision, -6.0)
    else {
        panic!("an undrained reliable lane refuses the third commit")
    };
    assert_eq!(codes, ["protocol.backpressure"]);
    assert_eq!(kept, revision, "the refused commit changed nothing");
}

// ---------------------------------------------------------------------------------------------
// Gate 3
// ---------------------------------------------------------------------------------------------

const WARM_BLOCKS: u64 = 8;
const MEASURED_BLOCKS: u64 = 64;
const REALTIME_FRAMES: u64 = (WARM_BLOCKS + MEASURED_BLOCKS + 8) * QUANTUM as u64;

fn telemetry_frame(request: u64, revision: u64) -> Vec<u8> {
    frame(
        request,
        ExpectedRevision::Exact(SessionRevision(revision)),
        CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
            // #1588 gate 5: the meter table is on, so the browser's render half pushes a frame
            // into the plan's ring at every closed window.
            meters: meter_table(),
            meter_period_blocks: METER_PERIOD_BLOCKS,
            meter_window_blocks: None,
            counter_ids: vec![CounterId::LiveValuesSuperseded],
            counter_period_blocks: 1,
            diagnostics_enabled: true,
            minimum_diagnostic_severity: DiagnosticSeverity::Info,
        }),
    )
}

/// Run `render` after `blocks` warm-up blocks and require zero allocations and frees around each of
/// `MEASURED_BLOCKS` calls.
fn assert_render_never_allocates(
    leg: &str,
    mut prepare: impl FnMut(u64),
    mut render: impl FnMut(u64),
    mut after: impl FnMut(),
) {
    for block in 0..WARM_BLOCKS + MEASURED_BLOCKS {
        prepare(block);
        let mark = thread_counts();
        render(block);
        let counts = thread_counts();
        if block >= WARM_BLOCKS {
            assert_eq!(
                (counts.0 - mark.0, counts.1 - mark.1),
                (0, 0),
                "{leg}: block {block} allocated or freed in a render call"
            );
        }
        after();
    }
}

/// Test value: red if the moved peak scan, or the browser's call to it, allocates on the render
/// thread. Telemetry is configured, so the scan runs on every block. #2115: the output loudness
/// meter is armed on both legs, so every measured block runs its render half and its hops close
/// and are pushed; red if that path allocates or frees.
#[test]
fn the_render_call_never_allocates_with_telemetry_configured() {
    assert_installed();

    let mut browser = Browser::new(REALTIME_FRAMES);
    let (result, _) = browser.send(&telemetry_frame(1, browser.host.committed_revision()));
    assert_eq!(result, RESULT_OK);
    let (result, _) = browser.send(&loudness_frame(2, protocol::LoudnessRequest::Start));
    assert_eq!(result, RESULT_OK);
    browser.drain();
    browser.service();
    let host = std::cell::RefCell::new(browser);
    let served = Cell::new(0_usize);
    assert_render_never_allocates(
        "browser",
        |block| host.borrow_mut().feed(block),
        |_| assert_eq!(host.borrow_mut().host.render_next(), RESULT_OK),
        || {
            let mut browser = host.borrow_mut();
            browser.service();
            let batches = browser
                .drain()
                .iter()
                .filter(|(_, event)| event_message(event) == MESSAGE_METER_BATCH)
                .count();
            served.set(served.get() + batches);
        },
    );
    // The first batch closes at block `METER_PERIOD_BLOCKS - 1`, after the warm-up, so every
    // batch served came from the measured blocks' ring pushes.
    const _: () = assert!(METER_PERIOD_BLOCKS as u64 > WARM_BLOCKS);
    assert!(
        served.get() >= 1,
        "vacuous: the measured blocks served no METER_BATCH"
    );
    assert_loudness_delivered(
        "browser",
        &host
            .borrow_mut()
            .send(&loudness_frame(3, protocol::LoudnessRequest::Read))
            .1,
    );

    let limits = host
        .borrow()
        .host
        .control_limits_for_test()
        .expect("the control half's limits");
    let mut c = CAbi::new(REALTIME_FRAMES, limits);
    let (code, _) = c.send(&telemetry_frame(1, host.borrow().host.committed_revision()));
    assert_eq!(code, C_RESULT_OK);
    let (code, _) = c.send(&loudness_frame(2, protocol::LoudnessRequest::Start));
    assert_eq!(code, C_RESULT_OK);
    c.drain();
    c.service();
    let c = std::cell::RefCell::new(c);
    let mut pcm = vec![f32::NAN; QUANTUM as usize * 2];
    assert_render_never_allocates(
        "miso_engine_v1_render_f32_planar",
        |block| c.borrow_mut().feed(block),
        |block| assert_eq!(c.borrow_mut().render(block, &mut pcm), C_RESULT_OK),
        || {
            let mut c = c.borrow_mut();
            c.service();
            c.drain();
        },
    );
    assert_loudness_delivered(
        "miso_engine_v1_render_f32_planar",
        &c.borrow_mut()
            .send(&loudness_frame(3, protocol::LoudnessRequest::Read))
            .1,
    );
}

fn loudness_frame(request: u64, operation: protocol::LoudnessRequest) -> Vec<u8> {
    frame(
        request,
        ExpectedRevision::Any,
        CommandPayload::Loudness(operation),
    )
}

/// The armed meter's hops reached the read: the measured blocks ran the render half's hop pushes.
fn assert_loudness_delivered(leg: &str, response: &[u8]) {
    let mut fields = [0_u16; 64];
    let Ok(protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::Loudness(reply),
        ..
    }) = ProtocolCodec::default()
        .decode_typed_response(response, &mut protocol::DecodeScratch::new(&mut fields))
    else {
        panic!("{leg}: a loudness reply")
    };
    // The measured blocks alone are 64 x 128 frames, 17 hops of 480.
    let measured = MEASURED_BLOCKS * u64::from(QUANTUM);
    assert!(
        reply.armed && reply.hops_dropped == 0 && reply.observed_sample >= measured,
        "{leg}: vacuous without hops closed in the measured blocks: {reply:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// #1586 gate 5: one tap read alike on both hosts
// ---------------------------------------------------------------------------------------------

/// The browser observation fixture: one track over `fixture-source` with a compressor insert whose
/// tap 1 is its gain reduction.
const OBSERVATION_SESSION: &str = include_str!("browser-v1/observation-session.json");

/// #1586 gate 5 (R4's parity row): both hosts boot the observation fixture, arm the compressor
/// insert's tap with a window of four blocks by the same `Observe` transaction, render the same PCM
/// bit-identically, and read the same row by the same `OBSERVATION_READ` frame (#1590: the
/// browser's own selected read is gone). Status, window and both lanes' bits agree pending and
/// ready.
///
/// Red if the two hosts read the same tap differently: another slot, another window, a lane
/// swapped or another status.
#[test]
fn both_hosts_read_one_observation_tap_alike() {
    const WINDOW_BLOCKS: u32 = 4;
    let mut model = parse_session_json(OBSERVATION_SESSION).expect("the fixture parses");
    model.sources[0].frames = 8_192;
    let document = canonical_session_json(&model).expect("canonical session");
    let mut browser = Browser {
        host: AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                live_control_meter_blocks: u64::from(WINDOW_BLOCKS),
                ..options()
            },
        )
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic()))),
    };
    let limits = browser
        .host
        .control_limits_for_test()
        .expect("the control half's limits");
    let mut c = CAbi::from_document(&document, limits);
    let strip = StableId::parse(TRACK_ID).expect("track ID");
    let comp = StableId::parse("comp").expect("insert ID");

    // Both hosts arm by the same `Observe` transaction.
    let revision = browser.host.committed_revision();
    let arm = frame(
        1,
        ExpectedRevision::Exact(SessionRevision(revision)),
        CommandPayload::SessionTransactionApply(&[SessionEdit::Observe {
            strip_id: strip.clone(),
            section: controls::TapSection::Inserts(comp.clone()),
            tap_id: 1,
            armed: true,
            window_blocks: WINDOW_BLOCKS,
            arm_tag: 0,
        }]),
    );
    let (result, reply) = browser.send(&arm);
    assert_eq!(result, RESULT_OK, "the browser arm");
    assert!(
        matches!(decode_verdict(&reply), Verdict::Applied { .. }),
        "the browser arm is applied"
    );
    browser.drain();
    let (result, reply) = c.send(&arm);
    assert_eq!(result, C_RESULT_OK, "{}", String::from_utf8_lossy(&reply));
    c.drain();

    let read = protocol::ObservationReadRequest {
        selections: vec![protocol::ObservationReadSelection {
            strip_id: strip,
            section: controls::TapSection::Inserts(comp),
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        }],
    };
    let mut request = 1;
    let mut ready = false;
    for block in 0..2 * u64::from(WINDOW_BLOCKS) {
        let browser_pcm = browser.step(block);
        assert_eq!(browser_pcm, c.step(block), "block {block}: the same PCM");
        browser.service();
        c.service();
        request += 1;
        let read_frame = frame(
            request,
            ExpectedRevision::Any,
            CommandPayload::ObservationRead(&read),
        );
        let row = |(result, reply): (u32, Vec<u8>), expected: u32| {
            assert_eq!(result, expected, "{}", String::from_utf8_lossy(&reply));
            let mut fields = [0_u16; 64];
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::ObservationRead(page),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&reply, &mut protocol::DecodeScratch::new(&mut fields))
                .expect("a response frame")
            else {
                panic!("an observation read page");
            };
            page.rows[0]
        };
        let c_row = row(c.send(&read_frame), C_RESULT_OK);
        let browser_row = row(browser.send(&read_frame), RESULT_OK);
        let lanes = |row: protocol::ObservationReadRow| {
            (
                row.status,
                row.window,
                row.left.map(f32::to_bits),
                row.right.map(f32::to_bits),
            )
        };
        assert_eq!(
            lanes(c_row),
            lanes(browser_row),
            "block {block}: the same row"
        );
        ready |= c_row.status == protocol::ObservationRowStatus::Ready;
    }
    assert!(ready, "vacuous: no window closed");
}
