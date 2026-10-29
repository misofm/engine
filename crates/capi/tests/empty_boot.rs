//! #1682: a session with no source and no track boots through the C ABI (owner answer of
//! 2026-10-15), and a structural transaction then adds a stereo stem with its track.
//!
//! * **Boot.** The empty session (only the output and one declared console slot) compiles. Its
//!   plan reports no source and no track, renders silence at the output with zero render
//!   allocations, and the resource row, the watermark, the control list, the counters and the
//!   snapshot all describe the same empty mixer.
//! * **Growth.** One `SESSION_TRANSACTION_APPLY` that adds a stereo source, a track and its route
//!   takes the rebuild path (the successor is prepared off the render thread). The swap block
//!   allocates nothing, the control list grows by the strip's rows, and a fed stem is heard.
//!
//! This file has its own test binary so that `bench_support`'s audited allocator serves it. The
//! browser leg mirrors it as a parity row (`hosts/host-web/tests/control_parity.rs`).

#![allow(unsafe_code)]

use core::ptr;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use capi::*;
use protocol::{
    CommandPayload, ControlListRequest, CounterId, CountersRequest, DecodeScratch,
    DecodedSuccessResponsePayload, DecodedTypedResponseFrame, EditPath, ExpectedRevision,
    ProtocolCodec, RequestId, SessionEdit, SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{SessionModel, StableId};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: u32 = 128;
const FRAME_BYTES: usize = 65_536;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The fixture cut to an empty mixer: no source, no track, no route. The console slot stays
/// declared, so the track added later carries it.
fn empty_model(rate: u32) -> SessionModel {
    let mut model = session::parse_session_json(FIXTURE).expect("fixture");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM;
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    assert_eq!(model.console.pre_insert.len(), 1, "a declared console slot");
    model
}

/// The stereo source, the track that reads it and the route of that track into the output.
fn stem_edits(rate: u32) -> Vec<SessionEdit> {
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let mut source = fixture.sources[0].clone();
    source.id = id("stem");
    source.frames = u64::from(QUANTUM) * 64;
    let mut track = fixture.tracks[0].clone();
    track.id = id("t1");
    track.source_id = id("stem");
    let mut route = fixture.routes[0].clone();
    route.id = id("t1-main");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("the fixture routes a track")
    };
    *track_id = id("t1");
    let _ = rate;
    vec![
        SessionEdit::UpsertSource { source },
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        reserved1: 0,
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
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 30,
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: FRAME_BYTES as u64,
        maximum_replay_bytes: 1 << 20,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// One compiled session and its plan.
struct Endpoint {
    session: *mut Session,
    plan: *mut Plan,
    revision: u64,
    request: u64,
}

impl Endpoint {
    fn boot(model: &SessionModel) -> Self {
        let document = session::canonical_session_json(model).expect("canonical session");
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
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
        // SAFETY: Every descriptor and output location lives for the complete call, and the engine
        // handle is destroyed once after compilation.
        unsafe {
            assert_eq!(
                miso_engine_v1_engine_create(&config, &mut engine),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                RESULT_OK,
                "{}",
                String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize])
            );
            miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            revision: model.revision,
            request: 0,
        }
    }

    /// One command through the exported entry; the response frame.
    fn send(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
        self.request += 1;
        let mut request = vec![0_u8; FRAME_BYTES];
        let len = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: expected,
                    payload,
                },
                &mut request,
            )
            .expect("command frame");
        let mut response = vec![0_u8; FRAME_BYTES];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; every buffer is owned.
        let code = unsafe {
            miso_engine_v1_submit_command(self.session, request.as_ptr(), len as u64, &mut output)
        };
        assert_eq!(code, RESULT_OK, "submit");
        response.truncate(output.required_bytes as usize);
        response
    }

    /// The control list as `(structure epoch, rows)`, read page by page.
    fn control_list(&mut self) -> (u64, Vec<String>) {
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
            let bytes = self.send(
                CommandPayload::ControlListGet(&request),
                ExpectedRevision::Any,
            );
            let mut fields = [0_u16; 4096];
            let DecodedTypedResponseFrame::Success { payload, .. } = ProtocolCodec::default()
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
                .expect("a control page")
            else {
                panic!("expected a control page")
            };
            let DecodedSuccessResponsePayload::ControlListPage(page) = payload else {
                panic!("expected a control list page")
            };
            rows.extend(page.entries.iter().map(|entry| format!("{entry:?}")));
            if page.eof {
                return (page.structure_epoch, rows);
            }
            (epoch, after) = (page.structure_epoch, page.last_handle);
        }
    }

    /// One counter's value.
    fn counter(&mut self, counter: CounterId) -> u64 {
        let bytes = self.send(
            CommandPayload::CountersGet(&CountersRequest {
                all: false,
                ids: vec![counter as u32],
            }),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("counters response")
        else {
            panic!("expected a counter snapshot")
        };
        assert_eq!(snapshot.values.len(), 1);
        snapshot.values[0].value
    }

    /// The canonical snapshot document the session reports.
    fn snapshot(&mut self) -> (u64, String) {
        let bytes = self.send(
            CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                offset: 0,
                maximum_bytes: 60_000,
            }),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionSnapshot(snapshot),
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("snapshot response")
        else {
            panic!("expected a session snapshot")
        };
        assert!(snapshot.eof, "the snapshot fits one chunk");
        (
            header.revision.0,
            String::from_utf8(snapshot.canonical_json_chunk.to_vec()).expect("utf-8"),
        )
    }

    /// The peaks of every meter record on the lossy lane.
    fn meter_peaks(&mut self) -> Vec<f32> {
        let mut peaks = Vec::new();
        loop {
            let mut event = vec![0_u8; FRAME_BYTES];
            let mut out = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: event.as_mut_ptr(),
                capacity_bytes: event.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: The session is live and used by this thread alone; the buffer is owned.
            let code =
                unsafe { miso_engine_v1_dequeue_event(self.session, EVENT_LANE_LOSSY, &mut out) };
            assert_eq!(code, RESULT_OK, "dequeue");
            if out.required_bytes == 0 {
                return peaks;
            }
            let mut fields = [0_u16; 64];
            let decoded = ProtocolCodec::default()
                .decode_typed_event(
                    &event[..out.required_bytes as usize],
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("a lossy event");
            if let protocol::DecodedEventPayload::MeterBatch(batch) = decoded.payload {
                for index in 0..usize::from(batch.count) {
                    peaks.push(batch.record(index).expect("a meter record").value);
                }
            }
        }
    }

    fn resources(&self) -> PlanResourceReport {
        // SAFETY: A zeroed report is a valid value whose size field is then set for the write.
        unsafe {
            let mut report: PlanResourceReport = core::mem::zeroed();
            report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
            assert_eq!(
                miso_engine_v1_plan_resources(self.plan, &mut report),
                RESULT_OK
            );
            report
        }
    }

    fn watermark(&self) -> Watermark {
        // SAFETY: As in `resources`.
        unsafe {
            let mut watermark: Watermark = core::mem::zeroed();
            watermark.struct_size = WATERMARK_SIZE;
            assert_eq!(
                miso_engine_v1_plan_watermark(self.plan, &mut watermark),
                RESULT_OK
            );
            watermark
        }
    }

    /// Renders block `block`; the output's bits.
    fn render(&mut self, block: u64) -> Vec<u32> {
        let mut pcm = vec![f32::NAN; QUANTUM as usize * 2];
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
        let code = unsafe {
            miso_engine_v1_render_f32_planar(self.plan, block * u64::from(QUANTUM), &output)
        };
        assert_eq!(code, RESULT_OK, "render");
        pcm.iter().map(|sample| sample.to_bits()).collect()
    }

    /// Renders one block with the render thread's allocator counters around the call alone.
    fn render_counted(&mut self, block: u64) -> (Vec<u32>, u64, u64) {
        let mut pcm = [f32::NAN; QUANTUM as usize * 2];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM,
            plane_stride_samples: QUANTUM,
            reserved: [0; 2],
        };
        let mark = current_thread_counters();
        // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
        let code = unsafe {
            miso_engine_v1_render_f32_planar(self.plan, block * u64::from(QUANTUM), &output)
        };
        let moved = current_thread_delta_since(mark);
        assert_eq!(code, RESULT_OK, "render");
        (
            pcm.iter().map(|sample| sample.to_bits()).collect(),
            moved.allocations,
            moved.deallocations,
        )
    }

    /// Offers `stem` one block of constant stereo PCM, `block` blocks into its generation.
    fn feed(&mut self, generation: u64, block: u64, rate: u32) {
        let left = [0.25_f32; QUANTUM as usize];
        let right = [-0.5_f32; QUANTUM as usize];
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: rate,
            generation,
            start_frame: block * u64::from(QUANTUM),
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
        // SAFETY: The session is live and used by this thread alone; the planes and the report
        // are owned storage that outlives the synchronous call.
        let code = unsafe {
            miso_engine_v1_source_submit_planar_f32(
                self.session,
                b"stem".as_ptr(),
                4,
                &chunk,
                &mut report,
            )
        };
        assert_eq!(code, RESULT_OK, "feed");
        assert_eq!(report.accepted_frames, u64::from(QUANTUM));
    }
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        // SAFETY: These are the exact live handles `boot` returned, destroyed once.
        unsafe {
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

/// The empty mixer boots at every launch rate and renders silence without allocating; its status,
/// control list, counters and snapshot agree.
///
/// Test value: red if any boot-time size that depends on the session's strip, source or meter
/// count fails at zero (a refused compile, a zero-length allocation the budget row cannot report,
/// a render that reads a first strip that is not there), if the silence is anything but `+0.0`, or
/// if one view of the mixer (resource row, watermark, control list, counters, snapshot) describes
/// a strip the others do not.
#[test]
fn an_empty_session_boots_and_renders_silence_without_allocating() {
    assert_installed();
    for rate in RATES {
        let model = empty_model(rate);
        let mut host = Endpoint::boot(&model);

        let resources = host.resources();
        assert_eq!(
            (
                resources.sample_rate_hz,
                resources.quantum_frames,
                resources.source_count,
                resources.track_count,
            ),
            (rate, QUANTUM, 0, 0),
            "{rate} Hz: the plan is empty"
        );
        let watermark = host.watermark();
        assert_eq!(
            watermark.revision, model.revision,
            "{rate} Hz: boot watermark"
        );

        for block in 0..4 {
            let (bits, allocations, deallocations) = host.render_counted(block);
            assert!(
                bits.iter().all(|word| *word == 0),
                "{rate} Hz: block {block} is +0.0 silence"
            );
            assert_eq!(
                (allocations, deallocations),
                (0, 0),
                "{rate} Hz: block {block} allocates nothing"
            );
        }

        // The control list names the console slot's session-level rows and no strip.
        let (epoch, rows) = host.control_list();
        assert!(
            rows.first()
                .is_some_and(|row| row.contains("owner_kind: Session"))
                && rows.iter().all(|row| !row.contains("owner_kind: Strip")),
            "{rate} Hz: only the session owns rows: {rows:?}"
        );
        // The output meter of an empty mixer reads silence (#1583): configure the output's peak at
        // the window the echo names, render one window past the activation, and read the batch
        // the service step stages.
        let probe = protocol::TelemetryConfiguration {
            meters: Vec::new(),
            meter_period_blocks: 0,
            meter_window_blocks: None,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
        };
        let window = |reply: &[u8]| {
            let mut fields = [0_u16; 64];
            match ProtocolCodec::default()
                .decode_typed_response(reply, &mut DecodeScratch::new(&mut fields))
                .expect("telemetry reply")
            {
                DecodedTypedResponseFrame::Success {
                    payload: protocol::DecodedSuccessResponsePayload::TelemetryConfiguration(echo),
                    ..
                } => echo
                    .meter_window_blocks
                    .expect("the C ABI has a meter window"),
                _ => panic!("{rate} Hz: the telemetry configuration is refused"),
            }
        };
        let blocks = window(&host.send(
            CommandPayload::TelemetryConfigure(&probe),
            ExpectedRevision::Exact(SessionRevision(model.revision)),
        ))
        .get();
        let meter = protocol::TelemetryConfiguration {
            meters: vec![protocol::MeterEntry {
                target: protocol::MeterTarget::Output,
                metric: protocol::MeterMetric::Peak,
            }],
            meter_period_blocks: blocks,
            ..probe
        };
        assert_eq!(
            window(&host.send(
                CommandPayload::TelemetryConfigure(&meter),
                ExpectedRevision::Exact(SessionRevision(model.revision)),
            ))
            .get(),
            blocks
        );
        // Four blocks rendered: the meter opens at the window grid's first boundary after them.
        let first = 4_u64.div_ceil(u64::from(blocks)) * u64::from(blocks);
        for block in 4..first + u64::from(blocks) {
            let _ = host.render(block);
        }
        // SAFETY: The session is live and used by this thread alone.
        assert_eq!(unsafe { miso_engine_v1_service(host.session) }, RESULT_OK);
        let peaks = host.meter_peaks();
        assert!(
            peaks.len() == 2 && peaks.iter().all(|peak| *peak == 0.0),
            "{rate} Hz: the meter reads silence, left and right: {peaks:?}"
        );
        let (again_epoch, again_rows) = host.control_list();
        assert_eq!((epoch, &rows), (again_epoch, &again_rows));

        assert_eq!(
            host.counter(CounterId::LiveValuesSuperseded),
            0,
            "{rate} Hz: no live value was superseded"
        );
        let (revision, json) = host.snapshot();
        assert_eq!(revision, model.revision);
        let snapshot = session::parse_session_json(&json).expect("the snapshot parses");
        assert!(snapshot.sources.is_empty() && snapshot.tracks.is_empty());
        assert_eq!(snapshot.console, model.console);
    }
}

/// One structural transaction adds a stereo stem, its track and its route to the booted empty
/// mixer: it takes the rebuild path, the swap block allocates nothing, the control list grows,
/// and the fed stem is heard. The browser mirrors this as a parity row.
///
/// Test value: red if the first strip cannot arrive through the successor paths (a ring, a bank,
/// a meter or a staging size fixed at zero by the empty boot), if the swap block allocates or
/// frees, or if the stem is silent after the swap.
#[test]
fn a_structural_transaction_adds_the_first_stem_to_an_empty_session() {
    assert_installed();
    for rate in RATES {
        let model = empty_model(rate);
        let mut host = Endpoint::boot(&model);
        let (empty_epoch, empty_rows) = host.control_list();
        for block in 0..2 {
            let _ = host.render(block);
        }

        let edits = stem_edits(rate);
        let reply = host.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(host.revision)),
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
        } = ProtocolCodec::default()
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("transaction reply")
        else {
            panic!("{rate} Hz: the transaction is refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        assert_eq!(applied.edit_path, EditPath::Rebuild, "{rate} Hz");
        assert_eq!(header.revision, SessionRevision(model.revision + 1));

        // The swap block: the successor adopts at the next block boundary, without allocating.
        let (_, allocations, deallocations) = host.render_counted(2);
        assert_eq!(
            (allocations, deallocations),
            (0, 0),
            "{rate} Hz: swap block"
        );
        let resources = host.resources();
        assert_eq!(
            (resources.source_count, resources.track_count),
            (1, 1),
            "{rate} Hz: the plan holds the stem"
        );
        assert_eq!(host.watermark().revision, model.revision + 1);

        let (epoch, rows) = host.control_list();
        assert!(epoch > empty_epoch, "{rate} Hz: the structure epoch moved");
        assert!(
            rows.len() > empty_rows.len() && rows.iter().any(|row| row.contains("t1")),
            "{rate} Hz: the strip's rows are listed"
        );

        // The source enters at the render clock the successor adopted at: a seek to a new
        // generation, then that generation's chunks.
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        let code = unsafe { miso_engine_v1_source_seek(host.session, b"stem".as_ptr(), 4, 2, 0) };
        assert_eq!(code, RESULT_OK, "{rate} Hz: seek");
        let mut heard = false;
        for block in 3..11 {
            host.feed(2, block - 3, rate);
            let (bits, allocations, deallocations) = host.render_counted(block);
            assert_eq!(
                (allocations, deallocations),
                (0, 0),
                "{rate} Hz: block {block} allocates nothing"
            );
            heard |= bits.iter().any(|word| f32::from_bits(*word) != 0.0);
        }
        assert!(heard, "{rate} Hz: the stem is heard");

        // The last strip leaves by the same path: the mixer is empty again and silent.
        let removal = [
            SessionEdit::RemoveRoute {
                route_id: id("t1-main"),
            },
            SessionEdit::RemoveTrack { track_id: id("t1") },
            SessionEdit::RemoveSource {
                source_id: id("stem"),
            },
        ];
        let reply = host.send(
            CommandPayload::SessionTransactionApply(&removal),
            ExpectedRevision::Exact(SessionRevision(model.revision + 1)),
        );
        let DecodedTypedResponseFrame::Success { header, .. } = ProtocolCodec::default()
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("removal reply")
        else {
            panic!("{rate} Hz: removing the last strip is refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        // The strip ramps out, then its removal is adopted at `S` (#1325 D3): from the block after
        // that, the mixer is silent.
        let mut block = 11;
        let mut adopted = None;
        while adopted.is_none_or(|first: u64| block < first + 3) {
            let (bits, allocations, deallocations) = host.render_counted(block);
            assert_eq!(
                (allocations, deallocations),
                (0, 0),
                "{rate} Hz: block {block}"
            );
            if adopted.is_some() {
                assert!(
                    bits.iter().all(|word| *word == 0),
                    "{rate} Hz: block {block} is silent again"
                );
            } else if host.watermark().revision == model.revision + 2 {
                adopted = Some(block);
            }
            block += 1;
            assert!(block < 64, "{rate} Hz: the removal is adopted");
        }
        let resources = host.resources();
        assert_eq!((resources.source_count, resources.track_count), (0, 0));
    }
}
