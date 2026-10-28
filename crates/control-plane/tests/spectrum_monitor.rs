//! #1593: the session's spectrum taps are monitor state that rides every plan it prepares.
//!
//! The preparer here is the C ABI's: host-core's one session entry, with the session's spectrum
//! preparation as the session passes it. Gate 1 (the first plan reads the reference window),
//! gate 2 (a structural edit carries the stream), gate 3 (the drop rule), gate 4 (all or nothing),
//! gate 5 (the configuration is in no revision and no snapshot) and gate 6 (render allocates
//! nothing, swap blocks included) run on it.
#![cfg(feature = "test-support")]

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, TestStructuralFaultPhase, compile_children, prepare_caps,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    PreparedSpectrumCaptures, SPECTRUM_WINDOW_FRAMES, SourceControlSet, SourceSubmission,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumConfiguration, SpectrumContinuousWindow, SpectrumPreparation,
    SpectrumService, SpectrumServiceError, SpectrumSmoothingConfig, SpectrumSnapshot,
    SpectrumStreamStatus, SpectrumTarget, StartedRenderSession, SuccessorBase,
    compile_host_session, prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_spectrum, spectrum_capture_collection_resources,
    spectrum_capture_resources_for,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ParameterChannel, ProtocolCodec, RequestId, SessionEdit,
    SessionSnapshotRequest, SpectrumCaptureReply, SpectrumCaptureRequest as CaptureRequest,
    SpectrumCaptureStatus, SpectrumConfigureRequest, SpectrumConfigured, SpectrumEntry,
    SpectrumForm, SpectrumStreamReply, SpectrumStreamRequest as StreamRequest,
    SpectrumStreamStatus as StreamStatus, SpectrumTargetKind, StatusCode, TypedCommandFrame,
};
use session::{
    RouteDestination, RouteSource, SessionModel, StableId, Submix, canonical_session_json,
    parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &[u8] = b"fixture-source";
const QUANTUM: usize = 128;
const SAMPLE_RATE_HZ: u32 = 48_000;

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

/// The C ABI's preparation: host-core's one session entry.
struct Preparer;

impl RuntimePreparer for Preparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        spectrum: Option<&SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        prepare_host_runtime_with_live_lanes_and_spectrum(
            compiled,
            caps,
            &HostLiveControlRequest::default(),
            lanes,
            spectrum,
            successor,
            superseded,
        )
    }
}

fn limits() -> ControlLimits {
    ControlLimits {
        source_ring_frames: 1_024,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_control_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: 65_536,
        maximum_replay_bytes: 262_144,
        maximum_replay_entries: 16,
        maximum_submixes: 4,
        maximum_vcas: 0,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

/// `eq0`'s post-pan tap, both channels, at its exact budget.
fn request() -> SpectrumCaptureRequest {
    let target = SpectrumTarget::TrackPostMatrix("eq0".into());
    SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    }
}

fn configuration() -> SpectrumConfiguration {
    SpectrumConfiguration {
        preparation: SpectrumPreparation::Single(request()),
        hop: None,
    }
}

/// `configuration` as the `SPECTRUM_CONFIGURE` that sets it.
fn configure_request_of(configuration: &SpectrumConfiguration) -> SpectrumConfigureRequest {
    let entry = |target: &SpectrumTarget, channels: SpectrumChannels| {
        let (target_kind, id) = match target {
            SpectrumTarget::TrackPostInputBuiltins(id) => (SpectrumTargetKind::TrackPostInput, id),
            SpectrumTarget::TrackPostMatrix(id) => (SpectrumTargetKind::TrackPostPan, id),
            SpectrumTarget::Output(id) => (SpectrumTargetKind::Output, id),
        };
        SpectrumEntry {
            target_kind,
            target_id: id.to_string(),
            channels: match channels {
                SpectrumChannels::Left => ParameterChannel::Left,
                SpectrumChannels::Right => ParameterChannel::Right,
                SpectrumChannels::Stereo => ParameterChannel::Both,
            },
        }
    };
    let (form, entries, maximum_capture_bytes) = match &configuration.preparation {
        SpectrumPreparation::Single(request) => (
            SpectrumForm::Single,
            vec![entry(&request.target, request.channels)],
            request.maximum_capture_bytes,
        ),
        SpectrumPreparation::Collection(request) => (
            SpectrumForm::Collection,
            request
                .entries
                .iter()
                .map(|item| entry(&item.target, item.channels))
                .collect(),
            request.maximum_capture_bytes,
        ),
    };
    SpectrumConfigureRequest {
        form,
        entries,
        maximum_capture_bytes,
        hop_frames: configuration.hop.map(host_core::SpectrumHop::get),
    }
}

/// One quantum of a period-97 sawtooth from `frame` (a rich, exactly computed tone), left at
/// twice the right's level.
fn tone(frame: u64) -> ([f32; QUANTUM], [f32; QUANTUM]) {
    let mut left = [0.0_f32; QUANTUM];
    let mut right = [0.0_f32; QUANTUM];
    for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
        let sample = ((frame + index as u64) % 97) as f32 / 97.0 - 0.5;
        *l = 0.5 * sample;
        *r = 0.25 * sample;
    }
    (left, right)
}

fn submission<'a>(frame: u64, planes: &'a [&'a [f32]]) -> SourceSubmission<'a> {
    SourceSubmission {
        generation: 1,
        start_frame: frame,
        sample_rate_hz: SAMPLE_RATE_HZ,
        planes,
        frames: QUANTUM as u32,
        end_of_region: false,
    }
}

/// A session booted with `spectrum`, fed the tone one quantum at a time.
struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    frame: u64,
    request: u64,
    /// The bits of the last rendered block.
    output: Vec<u32>,
}

impl Rig {
    fn new(spectrum: Option<SpectrumConfiguration>) -> Self {
        Self::of(SESSION, spectrum)
    }

    fn of(document: &str, spectrum: Option<SpectrumConfiguration>) -> Self {
        Self::limited(document, spectrum, limits())
    }

    fn limited(
        document: &str,
        spectrum: Option<SpectrumConfiguration>,
        limits: ControlLimits,
    ) -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(document, limits, Box::new(Preparer))
                .expect("boot compile");
        let mut rig = Self {
            session,
            plan,
            frame: 0,
            request: 0,
            output: Vec::new(),
        };
        // #1597: a boot's taps are one configure before the first block, adopted in place.
        if let Some(configuration) = spectrum {
            let reply = rig
                .configure_lent(&configure_request_of(&configuration))
                .expect("the boot configure");
            assert_eq!(
                (reply.configured_epoch, reply.adopted_epoch),
                (1, 1),
                "the boot's taps are in the plan of the first block"
            );
            assert_eq!(
                rig.session.spectrum_monitor().configuration(),
                Some(&configuration)
            );
        }
        rig
    }

    /// Feed and render one block, then run the session's service step (which promotes an
    /// adopted candidate). Gate 6: from the third block on, the render call allocates and frees
    /// nothing, swap blocks included.
    fn block(&mut self) {
        let (left, right) = tone(self.frame);
        self.session
            .submit(SOURCE, submission(self.frame, &[&left, &right]))
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.plan.render(self.frame, output).expect("render block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        if self.frame >= 2 * QUANTUM as u64 {
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "the render call at frame {} allocates or frees",
                self.frame
            );
        }
        self.output = samples.iter().map(|sample| sample.to_bits()).collect();
        self.frame += QUANTUM as u64;
        self.session.service().expect("service step");
    }

    /// Commit `edits` as one transaction: the path it took, or the refusal's debug text.
    fn apply(&mut self, edits: &[SessionEdit]) -> Result<EditPath, String> {
        self.request += 1;
        let revision = self.session.test_controller().session().revision();
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(self.request).expect("nonzero request"),
            expected_revision: ExpectedRevision::Exact(revision),
            payload: CommandPayload::SessionTransactionApply(edits),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("transaction command");
        let length = self
            .session
            .command(&bytes[..length], 65_536)
            .map_err(|error| format!("{error:?}"))?;
        let mut fields = [0_u16; 64];
        let path = match codec
            .decode_typed_response(
                self.session.command_response(length),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a transaction response")
        {
            DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
                ..
            } => Ok(applied.edit_path),
            DecodedTypedResponseFrame::NonOk { payload, .. } => Err(format!("{payload:?}")),
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        };
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
        path
    }

    /// Send one `SPECTRUM_CONFIGURE` (#1594): its reply, or the refusal's status and code.
    fn configure(
        &mut self,
        request: &SpectrumConfigureRequest,
    ) -> Result<SpectrumConfigured, (StatusCode, String)> {
        self.configure_with(request, false)
    }

    /// [`Self::configure`] with the plan lent to the command (#1597), as the browser sends it.
    fn configure_lent(
        &mut self,
        request: &SpectrumConfigureRequest,
    ) -> Result<SpectrumConfigured, (StatusCode, String)> {
        self.configure_with(request, true)
    }

    fn configure_with(
        &mut self,
        request: &SpectrumConfigureRequest,
        lend: bool,
    ) -> Result<SpectrumConfigured, (StatusCode, String)> {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(self.request).expect("nonzero request"),
            expected_revision: ExpectedRevision::Any,
            payload: CommandPayload::SpectrumConfigure(request),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("configure command");
        let length = if lend {
            self.session
                .command_with_plan(&bytes[..length], &[], 65_536, &mut self.plan)
        } else {
            self.session.command(&bytes[..length], 65_536)
        }
        .expect("a configure answers a frame");
        let mut fields = [0_u16; 64];
        match codec
            .decode_typed_response(
                self.session.command_response(length),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a configure response")
        {
            DecodedTypedResponseFrame::Success {
                header,
                payload: DecodedSuccessResponsePayload::SpectrumConfigured(reply),
            } => {
                assert_eq!(header.revision.0, reply.revision);
                Ok(reply)
            }
            DecodedTypedResponseFrame::NonOk { header, payload } => {
                Err((header.status, payload.diagnostics[0].code.clone()))
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        }
    }

    /// Send one `SPECTRUM_CAPTURE` (#1595) as request `request_id`: the response frame's bytes.
    fn capture_frame(&mut self, request_id: u64, request: &CaptureRequest) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(request_id).expect("nonzero request"),
            expected_revision: ExpectedRevision::Any,
            payload: CommandPayload::SpectrumCapture(request),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("capture command");
        let length = self
            .session
            .command(&bytes[..length], 65_536)
            .expect("a capture answers a frame");
        self.session.command_response(length).to_vec()
    }

    /// Send one `SPECTRUM_STREAM` (#1595) as a new request: the response frame's bytes.
    fn stream_frame(&mut self, request: &StreamRequest) -> Vec<u8> {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(self.request).expect("nonzero request"),
            expected_revision: ExpectedRevision::Any,
            payload: CommandPayload::SpectrumStream(request),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("stream command");
        let length = self
            .session
            .command(&bytes[..length], 65_536)
            .expect("a stream command answers a frame");
        self.session.command_response(length).to_vec()
    }

    /// Send one `SPECTRUM_STREAM` (#1595) as a new request: its reply, or the refusal's status
    /// and code.
    fn stream(
        &mut self,
        request: &StreamRequest,
    ) -> Result<SpectrumStreamReply<'static>, (StatusCode, String)> {
        let frame = self.stream_frame(request);
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(&frame, &mut DecodeScratch::new(&mut fields))
            .expect("a stream response")
        {
            DecodedTypedResponseFrame::Success {
                header,
                payload: DecodedSuccessResponsePayload::SpectrumStreamed(reply),
            } => {
                assert_eq!(header.message_id, protocol::MessageId::SpectrumStream);
                Ok(reply)
            }
            DecodedTypedResponseFrame::NonOk { header, payload } => {
                Err((header.status, payload.diagnostics[0].code.clone()))
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        }
    }

    /// Send one `SPECTRUM_CAPTURE` as a new request: its reply, or the refusal's status and code.
    fn capture(
        &mut self,
        request: &CaptureRequest,
    ) -> Result<SpectrumCaptureReply<'static>, (StatusCode, String)> {
        self.request += 1;
        let frame = self.capture_frame(self.request, request);
        decode_capture(&frame)
    }

    /// Commit `value` on `eq0`'s left fader as one live transaction.
    fn live_fader(&mut self, value: f32) {
        let edit = SessionEdit::SetControl {
            expected_effect: None,
            address: session::ControlAddress {
                owner: session::Owner::Strip(id("eq0")),
                section: session::Section::Builtins,
                control: 5,
                lane: session::ParameterChannel::Left,
            },
            value,
            ramp_samples: None,
        };
        assert_eq!(self.apply(&[edit]), Ok(EditPath::Live));
    }

    /// The plan's applied-revision watermark.
    fn watermark(&self) -> engine::realtime::PlanWatermark {
        self.plan
            .queries()
            .watermark()
            .expect("an untorn watermark")
    }

    /// The session's spectrum state: configuration, both epochs, and the pending plans' epochs.
    fn spectrum_state(&self) -> (Option<SpectrumConfiguration>, u64, u64, Vec<u64>) {
        let monitor = self.session.spectrum_monitor();
        (
            monitor.configuration().cloned(),
            monitor.configured_epoch(),
            monitor.adopted_epoch(),
            self.session
                .test_pending_providers()
                .iter()
                .map(control_plane::ProviderEpoch::test_epoch)
                .collect(),
        )
    }

    /// Commit `edits` as one rebuild transaction; gate 5: it advances the revision by one.
    fn rebuild(&mut self, edits: &[SessionEdit]) {
        let before = self.session.test_controller().session().revision().0;
        assert_eq!(self.apply(edits), Ok(EditPath::Rebuild));
        assert_eq!(
            self.session.test_controller().session().revision().0,
            before + 1,
            "a transaction advances the revision by one, and nothing else does"
        );
    }

    /// Render until render adopted the newest candidate and the service step promoted it.
    fn until_promoted(&mut self) {
        let mut blocks = 0;
        while !self.session.test_pending_providers().is_empty() {
            self.block();
            blocks += 1;
            // A removal is adopted at `S`, after the removed strip's ramp (#1325 D3).
            assert!(blocks <= 64, "the candidate is promoted");
        }
    }

    /// Read the stream after a block: its status and window.
    fn read_stream(&mut self) -> Read {
        let (service, analysis) = self
            .session
            .spectrum_monitor_mut()
            .service_and_analysis_mut()
            .expect("a configured session has a service");
        service
            .read_stream(Some(analysis))
            .expect("read the stream");
        let metadata = service.stream_metadata();
        Read {
            status: metadata.status,
            capture_epoch: metadata.capture_epoch,
            analysis_epoch: metadata.analysis_epoch,
            window: service.stream_window().copied(),
        }
    }

    fn start_stream(&mut self) {
        let (service, analysis) = self
            .session
            .spectrum_monitor_mut()
            .service_and_analysis_mut()
            .expect("a configured session has a service");
        service
            .start_stream(SpectrumSmoothingConfig::default(), Some(analysis))
            .expect("start the stream");
    }

    /// Select collection entry `target` for the next arm or stream.
    fn select(&mut self, target: &SpectrumTarget, channels: SpectrumChannels) {
        self.service()
            .select(target.as_ref(), channels)
            .expect("select the entry");
    }

    fn service(&mut self) -> &mut SpectrumService {
        self.session
            .spectrum_monitor_mut()
            .service_and_analysis_mut()
            .expect("a configured session has a service")
            .0
    }

    /// The committed session, paged through `SESSION_SNAPSHOT_GET`: every response frame's bytes.
    fn snapshot(&mut self) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut frames = Vec::new();
        let mut offset = 0_u64;
        loop {
            self.request += 1;
            let frame = TypedCommandFrame {
                request_id: RequestId::new(self.request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                    offset,
                    maximum_bytes: 2_048,
                }),
            };
            let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
            let length = codec
                .encode_command_frame_into(&frame, &mut bytes)
                .expect("snapshot command");
            let length = self
                .session
                .command(&bytes[..length], 65_536)
                .expect("snapshot page");
            let response = self.session.command_response(length);
            frames.extend_from_slice(response);
            let mut fields = [0_u16; 64];
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(page),
                ..
            } = codec
                .decode_typed_response(response, &mut protocol::DecodeScratch::new(&mut fields))
                .expect("snapshot response")
            else {
                panic!("expected a snapshot page")
            };
            offset += page.canonical_json_chunk.len() as u64;
            if page.eof {
                return frames;
            }
        }
    }
}

/// Both rigs' `SESSION_SNAPSHOT_GET` frames under the same request IDs, so they compare byte for
/// byte whatever each rig sent before (a boot's configure is one request more, #1597).
fn snapshots(a: &mut Rig, b: &mut Rig) -> (Vec<u8>, Vec<u8>) {
    let next = a.request.max(b.request);
    (a.request, b.request) = (next, next);
    (a.snapshot(), b.snapshot())
}

/// A `SPECTRUM_CAPTURE` response frame: its reply, or the refusal's status and code.
/// A non-OK response frame's status, first diagnostic code and backpressure record.
fn refusal_of(frame: &[u8]) -> (StatusCode, String, Option<protocol::Backpressure>) {
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::NonOk { header, payload } = ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut fields))
        .expect("a response")
    else {
        panic!("a non-OK reply");
    };
    (
        header.status,
        payload.diagnostics[0].code.clone(),
        payload.backpressure,
    )
}

fn decode_capture(frame: &[u8]) -> Result<SpectrumCaptureReply<'static>, (StatusCode, String)> {
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut fields))
        .expect("a capture response")
    {
        DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SpectrumCaptured(reply),
        } => {
            assert_eq!(header.message_id, protocol::MessageId::SpectrumCapture);
            Ok(reply)
        }
        DecodedTypedResponseFrame::NonOk { header, payload } => {
            Err((header.status, payload.diagnostics[0].code.clone()))
        }
        DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
    }
}

/// One stream read: its status, epochs and window.
#[derive(Clone, Copy)]
struct Read {
    status: SpectrumStreamStatus,
    capture_epoch: u64,
    analysis_epoch: u64,
    window: Option<SpectrumContinuousWindow>,
}

/// The reference: `prepare_host_runtime_with_spectrum` on the same document, fed alike.
struct Reference {
    render: StartedRenderSession,
    sources: SourceControlSet,
    service: SpectrumService,
    frame: u64,
}

impl Reference {
    fn new() -> Self {
        Self::of(&request())
    }

    fn of(request: &SpectrumCaptureRequest) -> Self {
        let caps = prepare_caps(limits());
        let compiled = compile_host_session(SESSION, &caps).expect("compiled fixture");
        let (host, capture) =
            prepare_host_runtime_with_spectrum(&compiled, &caps, request).expect("reference");
        let (render, sources, _) = host.start_render_session().expect("start render");
        Self {
            render,
            sources,
            service: SpectrumService::new(
                PreparedSpectrumCaptures::from(capture),
                SAMPLE_RATE_HZ,
                QUANTUM as u32,
                None,
            ),
            frame: 0,
        }
    }

    fn block(&mut self) {
        let (left, right) = tone(self.frame);
        self.sources
            .submit(SOURCE, submission(self.frame, &[&left, &right]))
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        self.render
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, self.frame)
            .expect("render block");
        self.frame += QUANTUM as u64;
    }
}

/// Arm at frame 0, render one window, read: on the session and on the reference alike.
macro_rules! one_window {
    ($service:expr, $block:expr) => {{
        $service.arm().expect("arm");
        for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
            $block;
        }
        $service
            .read(SpectrumChannels::Stereo)
            .expect("read")
            .expect("a complete window")
    }};
}

/// Gate 1 (#1597: the boot's taps are its first configure). Turns red if a configure before the
/// first block does not reach the plan that renders sample 0: on a session booted with no taps, a
/// configure through the lent plan answers `rebuild` with the adopted epoch already the configured
/// one and no candidate pending; a window armed then, before any block, equals bit for bit the
/// reference's window armed at frame 0; and every block renders the bits of a session booted with
/// no taps and never configured.
#[test]
fn a_configure_before_the_first_block_reaches_sample_zero() {
    let mut rig = Rig::new(None);
    let reply = rig.configure_lent(&single_request()).expect("configure");
    assert!(reply.rebuild);
    assert_eq!((reply.configured_epoch, reply.adopted_epoch), (1, 1));
    assert!(rig.session.test_pending_providers().is_empty());
    let mut plain = Rig::new(None);
    let mut reference = Reference::new();
    rig.service().arm().expect("arm before the first block");
    reference.service.arm().expect("arm the reference");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
        plain.block();
        reference.block();
        assert_eq!(rig.output, plain.output, "a block of other bits");
    }
    let got = rig
        .service()
        .read(SpectrumChannels::Stereo)
        .expect("read")
        .expect("a complete window");
    let want = reference
        .service
        .read(SpectrumChannels::Stereo)
        .expect("read")
        .expect("a complete window");
    assert_eq!(got.window.first_sample, 0);
    assert_eq!(got.window.first_sample, want.window.first_sample);
    assert!(got.window.left.iter().any(|sample| *sample != 0.0));
    assert!(same_words(&got.window.left, &want.window.left));
    assert!(same_words(&got.window.right, &want.window.right));
}

/// Gate 5. Turns red if spectrum state leaks into the model, the revision or the snapshot: a
/// session booted with a configuration has the revision and the `SESSION_SNAPSHOT_GET` bytes of
/// one booted without, before and after it reads a window; only its monitor differs.
#[test]
fn the_configuration_is_in_no_revision_and_no_snapshot() {
    let mut configured = Rig::new(Some(configuration()));
    let mut plain = Rig::new(None);
    let monitor = plain.session.spectrum_monitor();
    assert!(monitor.configuration().is_none() && monitor.service().is_none());
    assert!(monitor.analysis().is_none());
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (0, 0)
    );
    assert_eq!(
        configured.session.spectrum_monitor().configuration(),
        Some(&configuration())
    );
    let revision = |rig: &Rig| rig.session.test_controller().session().revision();
    assert_eq!(revision(&configured), revision(&plain));
    let (got, want) = snapshots(&mut configured, &mut plain);
    assert_eq!(got, want);
    let _: SpectrumSnapshot = one_window!(configured.service(), configured.block());
    assert_eq!(revision(&configured), revision(&plain));
    let (got, want) = snapshots(&mut configured, &mut plain);
    assert_eq!(got, want);
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

fn fixture() -> SessionModel {
    parse_session_json(SESSION).expect("fixture")
}

/// `eq0`'s post-pan tap (both channels) and `eq1`'s post-input tap (left), as one collection.
fn collection() -> SpectrumConfiguration {
    let entries = vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq0".into()),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostInputBuiltins("eq1".into()),
            channels: SpectrumChannels::Left,
        },
    ];
    let maximum_capture_bytes = spectrum_capture_collection_resources(&entries)
        .expect("collection resources")
        .retained_bytes;
    SpectrumConfiguration {
        preparation: SpectrumPreparation::Collection(SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes,
        }),
        hop: None,
    }
}

/// `UpsertTrack` of a muted copy of `eq0` as `a-added`, and `UpsertRoute` of its route.
fn add_muted_track() -> [SessionEdit; 2] {
    let model = fixture();
    let mut track = model.tracks[0].clone();
    track.id = id("a-added");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = id("a-added-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

/// `RemoveRoute` of `track`'s route and `RemoveTrack` of `track`.
fn remove_track(track: &str) -> [SessionEdit; 2] {
    [
        SessionEdit::RemoveRoute {
            route_id: id(&format!("{track}-main")),
        },
        SessionEdit::RemoveTrack {
            track_id: id(track),
        },
    ]
}

/// The fixture with `edits` applied, as a canonical document.
fn document_after(edits: &[SessionEdit]) -> String {
    let mut model = fixture();
    for edit in edits {
        match edit {
            SessionEdit::UpsertTrack { track } => model.tracks.push(track.clone()),
            SessionEdit::UpsertRoute { route } => model.routes.push(route.clone()),
            other => panic!("unexpected edit {other:?}"),
        }
    }
    canonical_session_json(&model).expect("canonical document")
}

/// The block before which the streams start: 100 ms at 48 kHz, rounded up to a block.
const STREAM_BLOCK: u64 = 4_800_u64.div_ceil(QUANTUM as u64);
/// The block after which the structural edit commits.
const EDIT_BLOCK: u64 = STREAM_BLOCK + 40;
/// Blocks rendered in all.
const TOTAL_BLOCKS: u64 = EDIT_BLOCK + 60;

/// Render `rig` to [`TOTAL_BLOCKS`], reading after every block from [`STREAM_BLOCK`]; `edit` runs
/// after block [`EDIT_BLOCK`].
fn stream_run(rig: &mut Rig, mut edit: impl FnMut(&mut Rig)) -> Vec<Read> {
    let mut reads = Vec::new();
    for block in 0..TOTAL_BLOCKS {
        if block == STREAM_BLOCK {
            rig.start_stream();
        }
        rig.block();
        if block >= STREAM_BLOCK {
            reads.push(rig.read_stream());
        }
        if block == EDIT_BLOCK {
            edit(rig);
        }
    }
    reads
}

/// The continuity of a carried stream (gate 2's reads): after the first window, no read is
/// `Failed`, `Gap` or `Warming`; the capture and analysis epochs never change; the windows'
/// sequence has no gap.
fn assert_continuous(reads: &[Read]) {
    let first = reads
        .iter()
        .position(|read| read.status == SpectrumStreamStatus::Ready)
        .expect("a first window");
    let epochs = (reads[first].capture_epoch, reads[first].analysis_epoch);
    let mut sequence = None;
    for (index, read) in reads.iter().enumerate().skip(first) {
        assert!(
            !matches!(
                read.status,
                SpectrumStreamStatus::Failed
                    | SpectrumStreamStatus::Gap
                    | SpectrumStreamStatus::Warming
            ),
            "read {index} is {:?}",
            read.status
        );
        assert_eq!(
            (read.capture_epoch, read.analysis_epoch),
            epochs,
            "read {index}"
        );
        if let Some(window) = read.window {
            if let Some(previous) = sequence {
                assert_eq!(window.sequence, previous + 1, "read {index}: a gap");
            }
            sequence = Some(window.sequence);
        }
    }
}

fn same_window(got: &SpectrumContinuousWindow, want: &SpectrumContinuousWindow) -> bool {
    got.first_sample == want.first_sample
        && got.sequence == want.sequence
        && got.stream_epoch == want.stream_epoch
        && got.channels == want.channels
        && got.source_underrun == want.source_underrun
        && got.dropped_captures == want.dropped_captures
        && got
            .left
            .iter()
            .chain(&got.right)
            .zip(want.left.iter().chain(&want.right))
            .all(|(got, want)| got.to_bits() == want.to_bits())
}

/// Gate 2, with gates 5 and 6 over its transaction and renders. A stream on `eq0` (selected in a
/// collection with `eq1`) starts at 100 ms; a structural edit adds a muted track. Two claims:
///
/// - an edit that keeps every target leaves the configuration unchanged, so `configured_epoch`
///   (and `adopted_epoch`) stay at 1;
/// - the carried stream reads on with no `Failed`, `Gap` or `Warming` read, unchanged epochs and
///   no sequence gap, and every window is bit-equal to that of a session that had the track from
///   the start.
///
/// The transaction advances the revision by one and the snapshot equals a plain session's.
#[test]
fn a_structural_edit_carries_the_stream() {
    bench_support::alloc::assert_installed();
    let eq0 = SpectrumTarget::TrackPostMatrix("eq0".into());
    let edits = add_muted_track();
    let mut rig = Rig::new(Some(collection()));
    rig.select(&eq0, SpectrumChannels::Stereo);
    let mut plain = Rig::new(None);
    let reads = stream_run(&mut rig, |rig| {
        rig.rebuild(&edits);
        plain.rebuild(&edits);
    });
    let monitor = rig.session.spectrum_monitor();
    assert_eq!(monitor.configuration(), Some(&collection()));
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (1, 1)
    );
    assert!(
        rig.session.test_pending_providers().is_empty(),
        "the successor was promoted"
    );
    assert_continuous(&reads);
    let (got, want) = snapshots(&mut rig, &mut plain);
    assert_eq!(got, want, "no spectrum in the snapshot");

    let mut reference = Rig::of(&document_after(&edits), Some(collection()));
    reference.select(&eq0, SpectrumChannels::Stereo);
    let want = stream_run(&mut reference, |_| {});
    assert_eq!(reads.len(), want.len());
    for (index, (got, want)) in reads.iter().zip(&want).enumerate() {
        assert_eq!(got.status, want.status, "read {index}");
        match (&got.window, &want.window) {
            (Some(got), Some(want)) => {
                assert!(same_window(got, want), "read {index}: the window differs")
            }
            (None, None) => {}
            _ => panic!("read {index}: one side has a window"),
        }
    }
    assert!(
        reads
            .iter()
            .skip(EDIT_BLOCK as usize - STREAM_BLOCK as usize + 1)
            .any(|read| read.window.is_some()),
        "windows after the edit"
    );
}

/// Gate 3, with gates 5 and 6 over its transactions and renders. Turns red if a removed target
/// refuses the edit (it does on the merge base), survives in the configuration, or drops a
/// caller's arm silently: removing `eq1` is accepted, leaves one entry and advances the
/// configuration epoch by one, and `eq0`'s stream reads on as in gate 2; removing `eq0` then
/// leaves no configuration and no service. On a second session, an arm on `eq1` taken before
/// `eq1`'s removal reads `Dropped` once after the promotion, and the read after that does not.
#[test]
fn a_removed_target_drops_its_entry_and_its_arm() {
    bench_support::alloc::assert_installed();
    let eq0 = SpectrumTarget::TrackPostMatrix("eq0".into());
    let eq1 = SpectrumTarget::TrackPostInputBuiltins("eq1".into());
    let mut rig = Rig::new(Some(collection()));
    rig.select(&eq0, SpectrumChannels::Stereo);
    let mut plain = Rig::new(None);
    let reads = stream_run(&mut rig, |rig| {
        rig.rebuild(&remove_track("eq1"));
        plain.rebuild(&remove_track("eq1"));
    });
    assert_continuous(&reads);
    let monitor = rig.session.spectrum_monitor();
    let SpectrumPreparation::Collection(kept) = &monitor
        .configuration()
        .expect("one entry stays")
        .preparation
    else {
        panic!("a collection")
    };
    assert_eq!(
        kept.entries,
        [SpectrumCaptureCollectionEntry {
            target: eq0.clone(),
            channels: SpectrumChannels::Stereo,
        }]
    );
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (2, 2)
    );
    let (got, want) = snapshots(&mut rig, &mut plain);
    assert_eq!(got, want, "no spectrum in the snapshot");

    rig.rebuild(&remove_track("eq0"));
    plain.rebuild(&remove_track("eq0"));
    let monitor = rig.session.spectrum_monitor();
    assert!(monitor.configuration().is_none());
    assert_eq!(monitor.configured_epoch(), 3);
    rig.until_promoted();
    let monitor = rig.session.spectrum_monitor();
    assert!(monitor.service().is_none() && monitor.analysis().is_none());
    assert_eq!(monitor.adopted_epoch(), 3);
    let (got, want) = snapshots(&mut rig, &mut plain);
    assert_eq!(got, want, "no spectrum in the snapshot");

    let mut armed = Rig::new(Some(collection()));
    armed.block();
    armed.select(&eq1, SpectrumChannels::Left);
    // The select armed the entry for nobody: the caller's own arm replaces it.
    let _ = armed.service().cancel();
    armed.service().arm().expect("arm eq1");
    armed.block();
    armed.rebuild(&remove_track("eq1"));
    // Until the promotion the service reads the running plan's captures (D7): the arm waits.
    assert!(matches!(
        armed.service().read(SpectrumChannels::Left),
        Ok(None)
    ));
    assert_eq!(armed.session.spectrum_monitor().adopted_epoch(), 1);
    armed.until_promoted();
    assert_eq!(armed.session.spectrum_monitor().adopted_epoch(), 2);
    assert!(matches!(
        armed.service().read(SpectrumChannels::Left),
        Err(SpectrumServiceError::Dropped)
    ));
    assert!(
        !matches!(
            armed.service().read(SpectrumChannels::Left),
            Err(SpectrumServiceError::Dropped)
        ),
        "the drop answers once"
    );
}

/// Gate 4. Turns red if the drop is stored before the commit: a transaction that removes `eq1`
/// and is refused (by an out-of-domain edit, and by a protocol commit fault after the successor
/// was prepared) leaves the configuration, both epochs and the service as they were, and the next
/// window reads on the running captures.
#[test]
fn a_refused_transaction_changes_no_spectrum_state() {
    let eq1 = SpectrumTarget::TrackPostInputBuiltins("eq1".into());
    let mut rig = Rig::new(Some(collection()));
    rig.block();
    let mut out_of_domain = fixture().tracks[2].clone();
    out_of_domain.fader.left_db = 1_000.0;
    let [route, track] = remove_track("eq1");
    let refused_edit = [
        route.clone(),
        track.clone(),
        SessionEdit::UpsertTrack {
            track: out_of_domain,
        },
    ];
    assert!(rig.apply(&refused_edit).is_err(), "an out-of-domain edit");
    rig.session
        .test_set_structural_faults([Some(TestStructuralFaultPhase::ProtocolCommit), None]);
    assert!(rig.apply(&[route, track]).is_err(), "a commit fault");
    let monitor = rig.session.spectrum_monitor();
    assert_eq!(monitor.configuration(), Some(&collection()));
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (1, 1)
    );
    assert!(rig.session.test_pending_providers().is_empty());
    rig.select(&eq1, SpectrumChannels::Left);
    let _ = rig.service().cancel();
    rig.service().arm().expect("arm eq1");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert!(
        rig.service()
            .read(SpectrumChannels::Left)
            .expect("read")
            .is_some(),
        "the window reads"
    );
}

/// #1593 T1's carry note. Turns red if an old observer is carried onto a new strip that reuses
/// its ID with another kind: one transaction removes track `eq1` and adds a submix `eq1`, whose
/// post-pan tap is the same `SpectrumTarget`; the stream on the track's tap does not read on into
/// the submix's (it ends, as for an entry that did not carry), and a new arm completes on the
/// submix.
#[test]
fn an_observer_is_never_carried_onto_a_strip_of_another_kind() {
    let eq1 = SpectrumTarget::TrackPostMatrix("eq1".into());
    let configuration = SpectrumConfiguration {
        preparation: SpectrumPreparation::Single(SpectrumCaptureRequest {
            maximum_capture_bytes: spectrum_capture_resources_for(&eq1).retained_bytes,
            target: eq1.clone(),
            channels: SpectrumChannels::Stereo,
        }),
        hop: None,
    };
    let mut rig = Rig::new(Some(configuration.clone()));
    rig.block();
    rig.start_stream();
    let mut ready = false;
    for _ in 0..2 * SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
        ready |= rig.read_stream().status == SpectrumStreamStatus::Ready;
    }
    assert!(ready, "the track's stream reads windows");
    let model = fixture();
    let mut feed = model.routes[2].clone();
    feed.id = id("eq2-eq1");
    feed.destination = RouteDestination::SubmixInput {
        submix_id: id("eq1"),
    };
    let mut out = model.routes[1].clone();
    out.source = RouteSource::Submix {
        submix_id: id("eq1"),
        tap: session::SendTap::PostPan,
    };
    let [route, track] = remove_track("eq1");
    rig.rebuild(&[
        route,
        track,
        SessionEdit::UpsertSubmix {
            submix: Submix::unity(id("eq1"), &model.console),
        },
        SessionEdit::UpsertRoute { route: feed },
        SessionEdit::UpsertRoute { route: out },
    ]);
    assert_eq!(
        rig.session.spectrum_monitor().configuration(),
        Some(&configuration),
        "the submix keeps the entry"
    );
    rig.until_promoted();
    let read = rig.read_stream();
    assert_eq!(
        read.status,
        SpectrumStreamStatus::Inactive,
        "the track's stream does not read on into the submix"
    );
    rig.service().arm().expect("arm the submix's tap");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert!(
        rig.service()
            .read(SpectrumChannels::Stereo)
            .expect("read")
            .is_some(),
        "the submix's tap completes a window"
    );
}

/// The fixture document at committed revision `revision`.
fn at_revision(revision: u64) -> String {
    let document = SESSION.replacen(
        "\"revision\": \"42\"",
        &format!("\"revision\": \"{revision}\""),
        1,
    );
    assert_ne!(document, SESSION);
    document
}

/// A `SPECTRUM_CONFIGURE` request entry.
fn entry(kind: SpectrumTargetKind, target: &str, channels: ParameterChannel) -> SpectrumEntry {
    SpectrumEntry {
        target_kind: kind,
        target_id: target.to_owned(),
        channels,
    }
}

/// A collection request of `entries` at exactly the budget its captures retain.
fn collection_request(
    entries: Vec<SpectrumEntry>,
    hop_frames: Option<u32>,
) -> SpectrumConfigureRequest {
    let prepared: Vec<SpectrumCaptureCollectionEntry> = entries
        .iter()
        .map(|entry| SpectrumCaptureCollectionEntry {
            target: match entry.target_kind {
                SpectrumTargetKind::TrackPostInput => {
                    SpectrumTarget::TrackPostInputBuiltins(entry.target_id.as_str().into())
                }
                SpectrumTargetKind::TrackPostPan => {
                    SpectrumTarget::TrackPostMatrix(entry.target_id.as_str().into())
                }
                SpectrumTargetKind::Output => {
                    SpectrumTarget::Output(entry.target_id.as_str().into())
                }
            },
            channels: match entry.channels {
                ParameterChannel::Left => SpectrumChannels::Left,
                ParameterChannel::Right => SpectrumChannels::Right,
                ParameterChannel::Both => SpectrumChannels::Stereo,
            },
        })
        .collect();
    let maximum_capture_bytes = if prepared.is_empty() {
        0
    } else {
        spectrum_capture_collection_resources(&prepared)
            .expect("collection resources")
            .retained_bytes
    };
    SpectrumConfigureRequest {
        form: SpectrumForm::Collection,
        entries,
        maximum_capture_bytes,
        hop_frames,
    }
}

/// `request()` as a `SPECTRUM_CONFIGURE`: `eq0`'s post-pan tap, both channels, single form.
fn single_request() -> SpectrumConfigureRequest {
    SpectrumConfigureRequest {
        form: SpectrumForm::Single,
        entries: vec![entry(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Both,
        )],
        maximum_capture_bytes: request().maximum_capture_bytes,
        hop_frames: None,
    }
}

/// `collection()` as a `SPECTRUM_CONFIGURE`.
fn collection_as_request(hop_frames: Option<u32>) -> SpectrumConfigureRequest {
    collection_request(
        vec![
            entry(
                SpectrumTargetKind::TrackPostPan,
                "eq0",
                ParameterChannel::Both,
            ),
            entry(
                SpectrumTargetKind::TrackPostInput,
                "eq1",
                ParameterChannel::Left,
            ),
        ],
        hop_frames,
    )
}

/// #1594 gate 2 (scope gate 3). Turns red if any gate still refuses the unchanged revision 0: on a
/// session booted with no taps, at revision 0, a single `eq0` configure answers `revision` 0,
/// `rebuild` and a plan epoch; after the promotion the adopted epoch is the configured one, and a
/// window armed then equals, bit for bit, the reference's window armed at the same frame.
#[test]
fn a_configure_at_revision_zero_reads_the_reference_window() {
    let mut rig = Rig::of(&at_revision(0), None);
    assert_eq!(rig.session.test_controller().session().revision().0, 0);
    rig.block();
    let reply = rig.configure(&single_request()).expect("configure");
    assert_eq!(reply.revision, 0);
    assert!(reply.rebuild);
    assert_eq!(
        reply.plan_epoch,
        rig.session
            .test_pending_providers()
            .last()
            .map(control_plane::ProviderEpoch::test_epoch)
    );
    assert_eq!((reply.configured_epoch, reply.adopted_epoch), (1, 0));
    rig.until_promoted();
    let monitor = rig.session.spectrum_monitor();
    assert_eq!(monitor.configuration(), Some(&configuration()));
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (1, 1)
    );
    assert_eq!(rig.session.test_controller().session().revision().0, 0);
    let mut reference = Reference::new();
    while reference.frame < rig.frame {
        reference.block();
    }
    let got: SpectrumSnapshot = one_window!(rig.service(), rig.block());
    let want: SpectrumSnapshot = one_window!(reference.service, reference.block());
    assert_eq!(got.window.first_sample, want.window.first_sample);
    assert!(
        got.window.left.iter().any(|sample| *sample != 0.0),
        "signal"
    );
    for (got, want) in [
        (&got.window.left, &want.window.left),
        (&got.window.right, &want.window.right),
    ] {
        assert!(
            got.iter()
                .zip(want)
                .all(|(got, want)| got.to_bits() == want.to_bits())
        );
    }
}

/// #1594 gate 3 (scope gate 4). Turns red if a monitor rebuild moves the watermark or leaks its
/// claim: at revision 5, a configure's plan is adopted with the watermark unchanged in every field,
/// and the next live fader edit advances it to 6 with `OUTCOME_EXACT`, `exact` one more and no
/// fallback or supersession.
#[test]
fn a_monitor_rebuild_keeps_the_watermark_exact() {
    let mut rig = Rig::of(&at_revision(2), None);
    for value in 1..=3 {
        rig.live_fader(-f32::from(u8::try_from(value).expect("small")));
        rig.block();
    }
    rig.block();
    let before = rig.watermark();
    assert_eq!(before.revision, 5);
    let reply = rig.configure(&single_request()).expect("configure");
    assert_eq!(reply.revision, 5);
    assert!(reply.rebuild);
    rig.until_promoted();
    rig.block();
    assert_eq!(
        rig.watermark(),
        before,
        "a monitor rebuild moves no watermark field"
    );
    rig.live_fader(-6.0);
    rig.block();
    rig.block();
    let after = rig.watermark();
    assert_eq!(after.revision, 6);
    assert_eq!(after.outcome_flags, engine::realtime::OUTCOME_EXACT);
    assert_eq!(after.exact, before.exact + 1);
    assert_eq!(after.transition_fallback, 0);
    assert_eq!(after.superseded, 0);
}

/// The windows of a stream's reads, one per sequence number.
fn windows(reads: &[Read]) -> Vec<SpectrumContinuousWindow> {
    let mut out: Vec<SpectrumContinuousWindow> = Vec::new();
    for window in reads.iter().filter_map(|read| read.window) {
        if out
            .last()
            .is_none_or(|last| last.sequence != window.sequence)
        {
            out.push(window);
        }
    }
    out
}

fn spacings(windows: &[SpectrumContinuousWindow]) -> Vec<u64> {
    windows
        .windows(2)
        .map(|pair| pair[1].first_sample - pair[0].first_sample)
        .collect()
}

/// #1594 gate 4 (scope gate 5). Turns red if a no-op or a hop change publishes a plan, or a hop
/// change restarts a stream: the same configuration again answers `rebuild` false with the epochs
/// unchanged; a hop of 1024 answers `rebuild` false with both epochs one more and nothing pending;
/// the running stream keeps the default 2048-frame spacing and its epochs, and a stream started
/// after it publishes windows 1024 frames apart.
#[test]
fn an_equal_or_hop_only_configure_publishes_no_plan() {
    let eq0 = SpectrumTarget::TrackPostMatrix("eq0".into());
    let mut rig = Rig::new(Some(collection()));
    rig.select(&eq0, SpectrumChannels::Stereo);
    rig.start_stream();
    let mut reads = Vec::new();
    for _ in 0..40 {
        rig.block();
        reads.push(rig.read_stream());
    }
    let same = rig.configure(&collection_as_request(None)).expect("same");
    assert_eq!(
        (
            same.rebuild,
            same.plan_epoch,
            same.configured_epoch,
            same.adopted_epoch
        ),
        (false, None, 1, 1)
    );
    let hop = rig
        .configure(&collection_as_request(Some(1024)))
        .expect("hop only");
    assert_eq!(
        (
            hop.rebuild,
            hop.plan_epoch,
            hop.configured_epoch,
            hop.adopted_epoch
        ),
        (false, None, 2, 2)
    );
    assert!(rig.session.test_pending_providers().is_empty(), "no plan");
    for _ in 0..40 {
        rig.block();
        reads.push(rig.read_stream());
    }
    assert_continuous(&reads);
    let running = spacings(&windows(&reads));
    assert!(running.len() >= 4);
    assert!(
        running.iter().all(|spacing| *spacing == 2_048),
        "{running:?}"
    );

    let (service, analysis) = rig
        .session
        .spectrum_monitor_mut()
        .service_and_analysis_mut()
        .expect("a service");
    service.stop_stream(Some(analysis));
    rig.start_stream();
    let mut next = Vec::new();
    for _ in 0..80 {
        rig.block();
        next.push(rig.read_stream());
    }
    let restarted = spacings(&windows(&next));
    assert!(restarted.len() >= 4);
    assert!(
        restarted.iter().all(|spacing| *spacing == 1_024),
        "{restarted:?}"
    );

    // A hop-only configure while a structural candidate is pending: the candidate carries the
    // new configuration, so its promotion adopts the configured epoch and the next stream starts
    // with the new hop.
    rig.rebuild(&add_muted_track());
    let pending = rig
        .configure(&collection_as_request(Some(512)))
        .expect("hop only while pending");
    assert!(!pending.rebuild);
    rig.until_promoted();
    let monitor = rig.session.spectrum_monitor();
    assert_eq!(monitor.adopted_epoch(), monitor.configured_epoch());
    assert_eq!(monitor.configured_epoch(), pending.configured_epoch);
    let (service, analysis) = rig
        .session
        .spectrum_monitor_mut()
        .service_and_analysis_mut()
        .expect("a service");
    service.stop_stream(Some(analysis));
    rig.start_stream();
    let mut last = Vec::new();
    for _ in 0..40 {
        rig.block();
        last.push(rig.read_stream());
    }
    let tuned = spacings(&windows(&last));
    assert!(tuned.len() >= 4);
    assert!(tuned.iter().all(|spacing| *spacing == 512), "{tuned:?}");
}

/// #1594 gate 5 (scope gate 6), with the render allocation check of every block. Turns red if a
/// kept tap restarts or a removed tap stays: from a one-entry collection on `eq0` with a running
/// stream, adding `eq1` keeps `eq0`'s stream continuous; keeping only `eq1` ends it (`Inactive`);
/// an empty collection with budget 0 removes the service and leaves the next plan no observer;
/// and a configure after that removal makes a new service whose arm completes a window.
#[test]
fn a_configure_keeps_adds_and_removes_taps() {
    let eq0 = SpectrumTarget::TrackPostMatrix("eq0".into());
    let only_eq0 = collection_request(
        vec![entry(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Both,
        )],
        None,
    );
    let mut rig = Rig::new(None);
    rig.configure(&only_eq0).expect("boot taps");
    rig.until_promoted();
    rig.select(&eq0, SpectrumChannels::Stereo);
    rig.start_stream();
    let mut reads = Vec::new();
    for _ in 0..30 {
        rig.block();
        reads.push(rig.read_stream());
    }
    let added = rig
        .configure(&collection_as_request(None))
        .expect("add eq1");
    assert!(added.rebuild);
    for _ in 0..30 {
        rig.block();
        reads.push(rig.read_stream());
    }
    assert!(rig.session.test_pending_providers().is_empty());
    assert_continuous(&reads);
    assert_eq!(
        rig.session.spectrum_monitor().adopted_epoch(),
        added.configured_epoch
    );

    let only_eq1 = collection_request(
        vec![entry(
            SpectrumTargetKind::TrackPostInput,
            "eq1",
            ParameterChannel::Left,
        )],
        None,
    );
    assert!(rig.configure(&only_eq1).expect("keep eq1").rebuild);
    rig.until_promoted();
    assert_eq!(rig.read_stream().status, SpectrumStreamStatus::Inactive);

    let none = collection_request(Vec::new(), None);
    let removed = rig.configure(&none).expect("remove every tap");
    assert!(removed.rebuild);
    rig.until_promoted();
    let monitor = rig.session.spectrum_monitor();
    assert!(monitor.configuration().is_none());
    assert!(monitor.service().is_none() && monitor.analysis().is_none());
    assert_eq!(monitor.adopted_epoch(), removed.configured_epoch);
    assert!(
        rig.session
            .test_providers()
            .test_inventory()
            .spectrum_pairing()
            .is_empty(),
        "the running plan has no spectrum observer"
    );

    let again = rig.configure(&single_request()).expect("configure again");
    assert!(again.rebuild);
    rig.until_promoted();
    let window: SpectrumSnapshot = one_window!(rig.service(), rig.block());
    assert!(window.window.left.iter().any(|sample| *sample != 0.0));
}

/// #1594 gate 6 (scope gate 7). Turns red if a refused configure writes anything, or a supersession
/// loses an acknowledged edit: an unknown target, a duplicate entry and a budget below what the
/// captures retain are refused with their status and code, and the configuration, both epochs,
/// the pending plans and the watermark are unchanged; a configure over a pending structural
/// candidate supersedes it, and the edit's revision completes `EXACT` at the adoption.
#[test]
fn a_refused_configure_changes_nothing_and_a_pending_edit_is_superseded() {
    let mut rig = Rig::new(Some(collection()));
    rig.block();
    rig.block();
    let state = rig.spectrum_state();
    let watermark = rig.watermark();
    let mut unknown = collection_as_request(None);
    unknown.entries[1].target_id = "nobody".to_owned();
    let mut duplicate = collection_as_request(None);
    duplicate.entries[1] = duplicate.entries[0].clone();
    let mut short = collection_as_request(Some(512));
    short.maximum_capture_bytes -= 1;
    for (request, status, code) in [
        (&unknown, StatusCode::NotFound, "spectrum.target.unknown"),
        (
            &duplicate,
            StatusCode::ValidationFailed,
            "spectrum.duplicate",
        ),
        (
            &short,
            StatusCode::LimitExceeded,
            "spectrum.budget.retained",
        ),
    ] {
        assert_eq!(rig.configure(request), Err((status, code.to_owned())));
        assert!(
            rig.spectrum_state() == state,
            "{code} changed the spectrum state"
        );
        assert_eq!(rig.watermark(), watermark, "{code}");
    }

    // The pending edit adds a track (a rebuild) and moves `eq1`'s fader, which the successor
    // carries from the running plan as a retarget: the superseding configure is prepared against
    // the running plan's base, whose model lacks the edit, and stamps that retarget.
    let mut moved = fixture().tracks[1].clone();
    moved.fader.left_db = -6.0;
    let mut pending = add_muted_track().to_vec();
    pending.push(SessionEdit::UpsertTrack { track: moved });
    rig.rebuild(&pending);
    let revision = rig.session.test_controller().session().revision().0;
    assert_eq!(
        rig.session.test_pending_providers().len(),
        1,
        "a pending candidate"
    );
    let reply = rig.configure(&single_request()).expect("supersede");
    assert!(reply.rebuild && reply.revision == revision);
    assert_eq!(
        rig.session.test_pending_providers().len(),
        1,
        "the candidate was superseded"
    );
    rig.until_promoted();
    rig.block();
    let after = rig.watermark();
    assert_eq!(after.revision, revision);
    assert_eq!(after.outcome_flags, engine::realtime::OUTCOME_EXACT);
    assert_eq!(after.exact, watermark.exact + 1);
    assert_eq!((after.transition_fallback, after.superseded), (0, 0));
    assert_eq!(
        rig.session.spectrum_monitor().configuration(),
        Some(&configuration())
    );
}

/// #1594 gate 8 (scope gate 8) and #1595 gate 8. Turns red if a monitor rebuild or a spectrum
/// read allocates on the render thread or races into an internal error: while another thread
/// renders, the control thread alternates two configurations 40 times (each a rebuild) and sends
/// 25 `SPECTRUM_CAPTURE` operations after each (1,000 in all: arms, reads with and without the
/// window, cancels); every render call allocates and frees nothing, every configure answers a
/// reply and no capture answers `INTERNAL`.
#[test]
fn alternating_configures_against_a_rendering_thread_allocate_nothing_on_render() {
    use std::sync::atomic::{AtomicBool, Ordering};
    bench_support::alloc::assert_installed();
    let Rig {
        mut session,
        mut plan,
        ..
    } = Rig::new(None);
    let done = AtomicBool::new(false);
    let mut request = 0_u64;
    let mut ready = 0_u32;
    std::thread::scope(|scope| {
        let done = &done;
        let render = scope.spawn(move || {
            let mut frame = 0_u64;
            let mut samples = [0.0_f32; QUANTUM * 2];
            while !done.load(Ordering::Acquire) {
                let output =
                    engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
                        .expect("output planes");
                let mark = bench_support::alloc::current_thread_counters();
                plan.render(frame, output).expect("render block");
                let used = bench_support::alloc::current_thread_delta_since(mark);
                if frame >= 2 * QUANTUM as u64 {
                    assert_eq!(
                        (used.allocations, used.deallocations),
                        (0, 0),
                        "frame {frame}"
                    );
                }
                frame += QUANTUM as u64;
                std::thread::yield_now();
            }
            frame
        });
        let codec = ProtocolCodec::default();
        let requests = [single_request(), collection_as_request(None)];
        for index in 0..40 {
            request += 1;
            let frame = TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::SpectrumConfigure(&requests[index % 2]),
            };
            let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
            let length = codec
                .encode_command_frame_into(&frame, &mut bytes)
                .expect("configure command");
            let length = loop {
                match session.command(&bytes[..length], 65_536) {
                    Ok(length) => break length,
                    Err(control_plane::CommandError::Backpressure) => {
                        session.service().expect("service step");
                        std::thread::yield_now();
                    }
                    Err(error) => panic!("configure {index}: {error:?}"),
                }
            };
            let mut fields = [0_u16; 64];
            assert!(matches!(
                codec
                    .decode_typed_response(
                        session.command_response(length),
                        &mut DecodeScratch::new(&mut fields),
                    )
                    .expect("a configure response"),
                DecodedTypedResponseFrame::Success {
                    payload: DecodedSuccessResponsePayload::SpectrumConfigured(_),
                    ..
                }
            ));
            session.service().expect("service step");
            // The collection has no selection: the first operation selects `eq0`, whose select
            // arms it for nobody, so the caller cancels before it arms its own.
            let select = CaptureRequest::Select {
                target_kind: SpectrumTargetKind::TrackPostPan,
                target_id: "eq0".to_owned(),
                channels: ParameterChannel::Both,
            };
            let mut next = if index % 2 == 1 {
                select
            } else {
                CaptureRequest::Arm
            };
            for capture in 0..25 {
                request += 1;
                let frame = TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Any,
                    payload: CommandPayload::SpectrumCapture(&next),
                };
                let length = codec
                    .encode_command_frame_into(&frame, &mut bytes)
                    .expect("capture command");
                let length = session
                    .command(&bytes[..length], 65_536)
                    .unwrap_or_else(|error| panic!("capture {index}.{capture}: {error:?}"));
                let reply = decode_capture(session.command_response(length));
                next = match (&next, reply) {
                    (_, Err((status, code))) => {
                        assert_ne!(status, StatusCode::Internal, "{code}");
                        CaptureRequest::Cancel
                    }
                    (CaptureRequest::Select { .. }, Ok(_)) => CaptureRequest::Cancel,
                    (CaptureRequest::Cancel, Ok(_)) => CaptureRequest::Arm,
                    (_, Ok(reply)) if reply.status == SpectrumCaptureStatus::Ready => {
                        ready += 1;
                        CaptureRequest::Arm
                    }
                    (_, Ok(_)) => CaptureRequest::Read {
                        channels: ParameterChannel::Both,
                        include_window: capture % 2 == 0,
                    },
                };
                session.service().expect("service step");
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        }
        assert!(ready > 0, "some reads analyzed a window");
        done.store(true, Ordering::Release);
        assert!(render.join().expect("render thread") > 0);
    });
}

/// The fixture with a submix `b` fed by `eq0`'s pre-fader send `eq0-b`, which follows `eq0`'s mute,
/// and `eq0`'s mute lane driven by a stored entry `eq0-mute` held open: a following route whose
/// source automates a mute lane.
fn follow_document() -> String {
    use session::{
        Automation, AutomationOffset, AutomationSegment, AutomationShape, ControlAddress, Owner,
        ParameterUnit, Section, SendTap,
    };
    let mut model = fixture();
    let mut send = model.routes[0].clone();
    send.id = id("eq0-b");
    send.source = RouteSource::Track {
        track_id: id("eq0"),
        tap: SendTap::PreFader,
    };
    send.destination = RouteDestination::SubmixInput { submix_id: id("b") };
    send.follows_mute = true;
    let mut out = model.routes[0].clone();
    out.id = id("b-main");
    out.source = RouteSource::Submix {
        submix_id: id("b"),
        tap: SendTap::PostPan,
    };
    model.routes.push(send);
    model.routes.push(out);
    let console = model.console.clone();
    model.submixes.push(Submix::unity(id("b"), &console));
    model.automation.push(Automation {
        id: id("eq0-mute"),
        target: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Builtins,
            control: 6,
            lane: session::ParameterChannel::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 1 << 30,
            start_value: 0.0,
            end_value: 0.0,
            unit: ParameterUnit::Linear,
        }],
        offset: AutomationOffset::default(),
    });
    canonical_session_json(&model).expect("canonical follow document")
}

/// #1594 attempt 2 (the verifier's first MAJOR). Turns red if a monitor rebuild that supersedes a
/// pending transaction drops the transaction's fallback: removing `eq0`'s last mute entry carries
/// the following send `eq0-b` into a six-word route, so that transaction completes as the
/// transition fallback (shown on a twin); superseded by a configure before render adopts it, the
/// successor carries the same edit against the same base and completes the revision as the
/// fallback too.
#[test]
fn a_configure_that_supersedes_a_fallback_edit_reports_the_fallback() {
    let document = follow_document();
    let remove = [SessionEdit::RemoveAutomation {
        automation_id: id("eq0-mute"),
    }];
    let mut twin = Rig::of(&document, None);
    let mut rig = Rig::of(&document, None);
    for _ in 0..3 {
        twin.block();
        rig.block();
    }
    twin.rebuild(&remove);
    twin.until_promoted();
    twin.block();
    assert_eq!(
        twin.watermark().transition_fallback,
        1,
        "the edit alone is the fallback"
    );

    rig.rebuild(&remove);
    let reply = rig.configure(&single_request()).expect("supersede");
    assert!(reply.rebuild);
    rig.until_promoted();
    rig.block();
    let after = rig.watermark();
    assert_eq!(after.revision, twin.watermark().revision);
    assert_eq!(
        after.transition_fallback, 1,
        "the superseding configure keeps the fallback"
    );
    assert_eq!(after.outcome_flags, twin.watermark().outcome_flags);
}

/// #1594 R6, #1595 R7. Turns red if a plan with spectrum captures does not charge the session's
/// spectrum service, analysis and one-shot slot: the running plan's control-retained bytes grow
/// by `SpectrumService::RETAINED_BYTES`, one `SpectrumAnalysisResult` and one `SpectrumSnapshot`
/// plus the plan inventory's own spectrum row when a configure adds a tap, and fall back when a
/// configure removes every tap, to the boot plan's bytes plus the pairing rows of the meters the
/// plan carries (#1674 R1). The row is 51 bytes here (one `SpectrumInventoryRow`, 48 bytes on a
/// 64-bit target, plus the 3 bytes of `eq0`, and no pairing row, since the predecessor had no
/// capture); it is host-core's private layout, so the check is a 1 KiB ceiling over the service's
/// bytes, which no inventory row of one tap approaches and which stays far below the service's own
/// size, so a missing charge cannot pass.
#[test]
fn a_plan_with_spectrum_charges_the_service_once() {
    let mut rig = Rig::new(None);
    rig.block();
    let bytes = |rig: &Rig| rig.plan.resources().control_retained_bytes;
    let before = bytes(&rig);
    rig.configure(&single_request()).expect("add a tap");
    rig.until_promoted();
    let grown = bytes(&rig) - before;
    let monitor = SpectrumService::RETAINED_BYTES
        + core::mem::size_of::<host_core::SpectrumAnalysisResult>() as u64
        + core::mem::size_of::<SpectrumSnapshot>() as u64;
    assert!(
        (monitor..monitor + 1_024).contains(&grown),
        "the service is charged once: {grown}"
    );
    rig.configure(&collection_request(Vec::new(), None))
        .expect("remove every tap");
    rig.until_promoted();
    // #1674 R1: the third plan carries its predecessor's meters, and its inventory charges one
    // pairing row per carried meter, which the boot plan has none of; the service is not charged.
    let pairing = rig
        .session
        .test_providers()
        .test_inventory()
        .meter_pairing();
    assert!(!pairing.is_empty(), "the third plan carries meters");
    assert_eq!(bytes(&rig), before + core::mem::size_of_val(pairing) as u64);
}

/// #1594 attempt 2 (the verifier's fifth point). Turns red if a monitor rebuild on the running
/// plan cuts a live send move in flight on a following route: on [`follow_document`], the
/// following send `eq0-b` moves to -6 dB over 4,000 samples, and two blocks into the move a
/// configure adds a tap (a monitor rebuild with no supersession). Every rendered block, through
/// the swap and the rest of the move, is bit-equal to a twin that never configures.
#[test]
fn a_monitor_rebuild_keeps_a_live_send_move_on_a_following_route() {
    let document = follow_document();
    let mut rig = Rig::of(&document, None);
    let mut twin = Rig::of(&document, None);
    // A session with no send move: the move is audible against it.
    let mut still = Rig::of(&document, None);
    for _ in 0..3 {
        rig.block();
        twin.block();
        still.block();
    }
    let edit = [SessionEdit::SetControl {
        expected_effect: None,
        address: session::ControlAddress {
            owner: session::Owner::Route(id("eq0-b")),
            section: session::Section::Own,
            control: 1,
            lane: session::ParameterChannel::Both,
        },
        value: -6.0,
        ramp_samples: Some(4_000),
    }];
    assert_eq!(rig.apply(&edit), Ok(EditPath::Live));
    assert_eq!(twin.apply(&edit), Ok(EditPath::Live));
    for _ in 0..2 {
        rig.block();
        twin.block();
        still.block();
        assert_eq!(rig.output, twin.output);
    }
    assert!(rig.configure(&single_request()).expect("configure").rebuild);
    let mut moved = false;
    for block in 0..40 {
        rig.block();
        twin.block();
        still.block();
        moved |= still.output != twin.output;
        assert_eq!(rig.output, twin.output, "block {block} after the configure");
    }
    assert!(
        rig.session.test_pending_providers().is_empty(),
        "the plan was adopted"
    );
    assert!(moved, "the send move is audible");
}

/// #1594 attempt-2 fold. Turns red if a configure that supersedes another monitor candidate
/// inherits a transaction's outcome terms: during a ramping send move on a following route (live,
/// at revision r, which the watermark has reached), a configure is superseded by a second one
/// before render adopts it. The second covers no new revision, so its claim must be `EXACT`; the
/// next live edit's revision r + 1 then reports no transition fallback.
#[test]
fn a_configure_that_supersedes_a_monitor_candidate_reports_no_fallback() {
    let mut rig = Rig::of(&follow_document(), None);
    for _ in 0..3 {
        rig.block();
    }
    let send_move = [SessionEdit::SetControl {
        expected_effect: None,
        address: session::ControlAddress {
            owner: session::Owner::Route(id("eq0-b")),
            section: session::Section::Own,
            control: 1,
            lane: session::ParameterChannel::Both,
        },
        value: -6.0,
        ramp_samples: Some(4_000),
    }];
    assert_eq!(rig.apply(&send_move), Ok(EditPath::Live));
    rig.block();
    let revision = rig.session.test_controller().session().revision().0;
    assert_eq!(rig.watermark().revision, revision);
    assert!(rig.configure(&single_request()).expect("first").rebuild);
    assert!(
        rig.configure(&collection_as_request(None))
            .expect("second")
            .rebuild
    );
    assert_eq!(
        rig.session.test_pending_providers().len(),
        1,
        "the first was superseded"
    );
    rig.until_promoted();
    rig.live_fader(-3.0);
    rig.block();
    rig.block();
    let after = rig.watermark();
    assert_eq!(after.revision, revision + 1);
    assert_eq!(after.transition_fallback, 0, "no false fallback");
    assert_eq!(after.outcome_flags, engine::realtime::OUTCOME_EXACT);
}

const ARM: CaptureRequest = CaptureRequest::Arm;

const fn read(channels: ParameterChannel, include_window: bool) -> CaptureRequest {
    CaptureRequest::Read {
        channels,
        include_window,
    }
}

fn select(kind: SpectrumTargetKind, target: &str, channels: ParameterChannel) -> CaptureRequest {
    CaptureRequest::Select {
        target_kind: kind,
        target_id: target.to_owned(),
        channels,
    }
}

fn same_words(got: &[f32], want: &[f32]) -> bool {
    got.len() == want.len()
        && got
            .iter()
            .zip(want)
            .all(|(got, want)| got.to_bits() == want.to_bits())
}

/// The reply status and, when ready, the window's token and first sample.
fn status_of(reply: &SpectrumCaptureReply<'_>) -> (SpectrumCaptureStatus, Option<(u64, u64)>) {
    (
        reply.status,
        reply
            .window
            .as_ref()
            .map(|window| (window.snapshot_token, window.captured_sample)),
    )
}

/// #1595 gate 2. Turns red if the C ABI's read path analyzes, labels or counts a window
/// differently from the core: on the single `(eq0 post-input, Stereo)` session (a boundary the
/// tone reaches at full level), `arm` is done, a read is pending until 2,048 frames have rendered,
/// then ready with token 1. The second round is token 2, and its bins equal word for word
/// `SpectrumAnalysis::analyze_into` of the reference window (`prepare_host_runtime_with_spectrum`
/// fed the same PCM from frame 0, read the same way), its raw planes equal that window, and its
/// labels are the window's.
#[test]
fn a_capture_read_returns_the_reference_analysis() {
    let target = SpectrumTarget::TrackPostInputBuiltins("eq0".into());
    let tap = SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    };
    let mut rig = Rig::new(Some(SpectrumConfiguration {
        preparation: SpectrumPreparation::Single(tap.clone()),
        hop: None,
    }));
    let done = rig.capture(&ARM).expect("arm");
    assert_eq!(
        (done.status, done.adopted_epoch, done.selection_epoch),
        (SpectrumCaptureStatus::Done, 1, 0)
    );
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        assert_eq!(
            status_of(
                &rig.capture(&read(ParameterChannel::Left, false))
                    .expect("read")
            ),
            (SpectrumCaptureStatus::Pending, None)
        );
        rig.block();
    }
    let first = rig
        .capture(&read(ParameterChannel::Left, false))
        .expect("read");
    assert_eq!(
        status_of(&first),
        (SpectrumCaptureStatus::Ready, Some((1, 0)))
    );
    let first = first.window.expect("a ready window");
    assert!(first.right_db.is_none() && first.left_window.is_none());

    rig.capture(&ARM).expect("arm again");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    let ready = rig
        .capture(&read(ParameterChannel::Both, true))
        .expect("read");
    let mut reference = Reference::of(&tap);
    let _: SpectrumSnapshot = one_window!(reference.service, reference.block());
    let snapshot: SpectrumSnapshot = one_window!(reference.service, reference.block());
    let mut want = host_core::SpectrumAnalysisResult::new();
    host_core::SpectrumAnalysis::new()
        .analyze_into(&snapshot, &mut want)
        .expect("reference analysis");
    assert!(
        want.left_dbfs.iter().any(|bin| *bin > -60.0),
        "a spectrum of signal"
    );
    let window = ready.window.as_ref().expect("a ready window");
    assert_eq!(ready.status, SpectrumCaptureStatus::Ready);
    assert_eq!(
        (
            window.target_kind,
            window.target_id.as_ref(),
            window.channels,
            window.sample_rate_hz,
            window.captured_sample,
            window.end_sample,
            window.snapshot_token,
            window.source_underrun,
        ),
        (
            SpectrumTargetKind::TrackPostInput,
            "eq0",
            ParameterChannel::Both,
            want.sample_rate_hz,
            want.captured_sample,
            want.end_sample,
            2,
            want.source_underrun,
        )
    );
    assert_eq!(want.token, 2);
    assert_eq!((window.window_frames, window.bin_count), (2_048, 1_025));
    assert_eq!(window.floor_db, -120.0);
    assert!(same_words(&window.frequencies_hz, &want.frequencies_hz));
    assert!(same_words(
        window.left_db.as_deref().expect("left"),
        &want.left_dbfs
    ));
    assert!(same_words(
        window.right_db.as_deref().expect("right"),
        &want.right_dbfs
    ));
    assert!(same_words(
        window.left_window.as_deref().expect("left"),
        &snapshot.window.left
    ));
    assert!(same_words(
        window.right_window.as_deref().expect("right"),
        &snapshot.window.right
    ));
}

/// #1595 gate 3 (one-shot). Turns red if a read pops a window it then fails to deliver: on a
/// session whose frame limit is 16,384 bytes, a ready stereo read with the window (29,008 bytes)
/// and a `Left` read with its window (16,696 bytes) each answer `LIMIT_EXCEEDED`
/// `spectrum.frame`, and a stereo read without it (12,608 bytes) then returns that same window
/// (token 1, frame 0). A configure of the stereo tap is accepted there and refused
/// `LIMIT_EXCEEDED` `spectrum.frame` at 8,192 bytes, setting nothing.
#[test]
fn a_read_the_frame_cannot_carry_consumes_nothing() {
    let mut rig = Rig::limited(
        SESSION,
        Some(configuration()),
        ControlLimits {
            maximum_control_frame_bytes: 16_384,
            ..limits()
        },
    );
    rig.capture(&ARM).expect("arm");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert_eq!(
        rig.capture(&read(ParameterChannel::Both, true)),
        Err((StatusCode::LimitExceeded, "spectrum.frame".to_owned()))
    );
    assert_eq!(
        rig.capture(&read(ParameterChannel::Left, true)),
        Err((StatusCode::LimitExceeded, "spectrum.frame".to_owned()))
    );
    assert_eq!(
        status_of(
            &rig.capture(&read(ParameterChannel::Both, false))
                .expect("read")
        ),
        (SpectrumCaptureStatus::Ready, Some((1, 0)))
    );
    assert!(
        rig.configure(&single_request()).is_ok(),
        "a tap whose reads fit is accepted"
    );

    let mut small = Rig::limited(
        SESSION,
        None,
        ControlLimits {
            maximum_control_frame_bytes: 8_192,
            ..limits()
        },
    );
    small.block();
    assert_eq!(
        small.configure(&single_request()),
        Err((StatusCode::LimitExceeded, "spectrum.frame".to_owned()))
    );
    assert_eq!(small.spectrum_state(), (None, 0, 0, Vec::new()));
}

/// #1595 gate 6 (one-shot). Turns red if a refusal writes service state or maps to a status
/// outside D6's table: each refusal answers its status and code, and the next valid call behaves
/// as if the refused one never ran. #1944: red if a `Busy` capture or stream-start refusal names
/// the replay cache, or no record, instead of the arm slot (capacity 1, occupancy 1).
#[test]
fn each_capture_refusal_answers_its_status_and_changes_nothing() {
    let refusal = |status, code: &str| Err((status, code.to_owned()));
    // `Unprepared`: no taps.
    let mut bare = Rig::new(None);
    assert_eq!(
        bare.capture(&ARM),
        refusal(StatusCode::NotFound, "spectrum.unprepared")
    );

    // On a single tap: `NotArmed`, `SelectSingle`, then `Busy`.
    let mut rig = Rig::new(Some(configuration()));
    assert_eq!(
        rig.capture(&read(ParameterChannel::Both, false)),
        refusal(StatusCode::ValidationFailed, "spectrum.not_armed")
    );
    assert_eq!(
        rig.capture(&select(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Both
        )),
        refusal(StatusCode::ValidationFailed, "spectrum.select.single")
    );
    rig.capture(&ARM).expect("arm");
    // #1944: a `Busy` capture, and a stream start while the caller holds the arm, each name the
    // one-shot arm slot, not the replay cache.
    let busy = (
        StatusCode::Backpressure,
        "spectrum.busy".to_owned(),
        Some(protocol::Backpressure {
            queue_kind: protocol::BackpressureQueueKind::SpectrumArm,
            capacity: 1,
            occupancy: 1,
            requested_items: 1,
            generation: None,
            retry_boundary: None,
            requested_bytes: None,
            available_bytes: None,
        }),
    );
    rig.request += 1;
    let frame = rig.capture_frame(rig.request, &ARM);
    assert_eq!(refusal_of(&frame), busy);
    let frame = rig.stream_frame(&StreamRequest::Start { smoothing_ms: 0.0 });
    assert_eq!(refusal_of(&frame), busy, "a stream start");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert_eq!(
        status_of(
            &rig.capture(&read(ParameterChannel::Both, false))
                .expect("read")
        ),
        (SpectrumCaptureStatus::Ready, Some((1, 0))),
        "the refused arm left the caller's"
    );

    // On a collection: `UnknownEntry`, `Channels`, `Busy` on a select, `Dropped`.
    let mut rig = Rig::new(Some(collection()));
    rig.block();
    assert_eq!(
        rig.capture(&select(
            SpectrumTargetKind::TrackPostPan,
            "eq1",
            ParameterChannel::Both
        )),
        refusal(StatusCode::NotFound, "spectrum.entry.unknown")
    );
    assert_eq!(
        rig.capture(&select(
            SpectrumTargetKind::TrackPostInput,
            "eq1",
            ParameterChannel::Left
        ))
        .map(|reply| (reply.status, reply.selection_epoch)),
        Ok((SpectrumCaptureStatus::Done, 1)),
        "the refused select changed no selection epoch"
    );
    // The select armed the entry for nobody; the caller cancels it and arms its own.
    rig.capture(&CaptureRequest::Cancel).expect("cancel");
    rig.capture(&ARM).expect("arm");
    assert_eq!(
        rig.capture(&select(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Both
        )),
        refusal(StatusCode::Backpressure, "spectrum.busy")
    );
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert_eq!(
        rig.capture(&read(ParameterChannel::Both, false)),
        refusal(StatusCode::ValidationFailed, "spectrum.channels")
    );
    let ready = rig
        .capture(&read(ParameterChannel::Left, false))
        .expect("read");
    let window = ready.window.expect("the window the refusals left");
    assert_eq!(
        (window.target_id.as_ref(), window.snapshot_token),
        ("eq1", 1)
    );
    rig.capture(&ARM).expect("arm");
    rig.block();
    rig.rebuild(&remove_track("eq1"));
    rig.until_promoted();
    assert_eq!(
        rig.capture(&read(ParameterChannel::Left, false)),
        refusal(StatusCode::Unavailable, "spectrum.capture.dropped")
    );
    assert_eq!(
        rig.capture(&select(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Both
        ))
        .map(|reply| reply.status),
        Ok(SpectrumCaptureStatus::Done)
    );
}

/// #1595 gate 7. Turns red if a replayed read pops again: after a second window is complete, the
/// exact replay of the first ready read returns the same bytes, and the next new read returns the
/// second window (token 2, frame 2,048): the replay consumed nothing.
#[test]
fn a_replayed_read_returns_its_bytes_and_pops_nothing() {
    let mut rig = Rig::new(Some(configuration()));
    rig.capture(&ARM).expect("arm");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    let request = read(ParameterChannel::Both, false);
    rig.request += 1;
    let id = rig.request;
    let first = rig.capture_frame(id, &request);
    assert_eq!(
        decode_capture(&first).expect("ready").status,
        SpectrumCaptureStatus::Ready
    );
    rig.capture(&ARM).expect("arm again");
    for _ in 0..SPECTRUM_WINDOW_FRAMES / QUANTUM {
        rig.block();
    }
    assert_eq!(
        rig.capture_frame(id, &request),
        first,
        "the replay is the recorded reply"
    );
    assert_eq!(
        status_of(&rig.capture(&request).expect("read")),
        (SpectrumCaptureStatus::Ready, Some((2, 2_048)))
    );
}

/// One `SPECTRUM_STREAM` reply's machine facts, comparable to the core's metadata.
fn stream_facts(reply: &SpectrumStreamReply<'_>) -> (u8, u64, u64, u64, u64, u64, u32) {
    (
        reply.status.code(),
        reply.sequence,
        reply.windows,
        reply.dropped_captures,
        reply.captured_sample,
        reply.end_sample,
        reply.hop_frames,
    )
}

/// The core's metadata facts, as [`stream_facts`] takes them from a reply.
fn core_facts(metadata: &host_core::SpectrumStreamMetadata) -> (u8, u64, u64, u64, u64, u64, u32) {
    (
        u8::try_from(metadata.status.code()).expect("a status code"),
        metadata.sequence,
        metadata.windows,
        metadata.dropped_captures,
        metadata.captured_sample,
        metadata.end_sample,
        metadata.hop_frames,
    )
}

/// #1595 gate 4. Turns red if a stream reply's status, epochs, drop count or bins differ from the
/// core machine: on the collection `(eq0 post-input, Stereo)`, `(eq1 post-input, Left)`
/// (post-input: a boundary the tone reaches at full level), `select` `eq0` answers `inactive`
/// with selection epoch 1; `start` (100 ms smoothing) answers `warming`,
/// and every read after each block equals, in status, sequence, window count, drops, samples and
/// hop, the core's `read_stream` on the reference session fed the same PCM, with the bins of the
/// first four ready windows equal word for word to `analyze_stream_into` there, and ready windows
/// one hop apart. A `select` of `eq1` at 50 ms answers `warming` with both the selection epoch
/// and the analysis epoch one more; three hops without a read give `gap` with two dropped
/// windows, then `ready`; `reset` advances the analysis epoch and answers `pending` (only a read
/// delivers a window); a one-shot cancel is
/// `VALIDATION_FAILED` `spectrum.stream.active`; `stop` answers `stopped`.
#[test]
fn a_stream_read_follows_the_core_machine() {
    let target = SpectrumTarget::TrackPostInputBuiltins("eq0".into());
    let entries = vec![
        SpectrumCaptureCollectionEntry {
            target: target.clone(),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostInputBuiltins("eq1".into()),
            channels: SpectrumChannels::Left,
        },
    ];
    let maximum_capture_bytes = spectrum_capture_collection_resources(&entries)
        .expect("collection resources")
        .retained_bytes;
    let mut rig = Rig::new(Some(SpectrumConfiguration {
        preparation: SpectrumPreparation::Collection(SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes,
        }),
        hop: None,
    }));
    let mut reference = Reference::of(&SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    });
    let mut analysis = host_core::SpectrumAnalysis::new();
    let selected = rig
        .stream(&StreamRequest::Select {
            target_kind: SpectrumTargetKind::TrackPostInput,
            target_id: "eq0".to_owned(),
            channels: ParameterChannel::Both,
            smoothing_ms: None,
        })
        .expect("select");
    assert_eq!(
        (selected.status, selected.selection_epoch),
        (StreamStatus::Inactive, 1)
    );
    let hop = host_core::SpectrumCadence::new(SAMPLE_RATE_HZ, QUANTUM as u32)
        .expect("a launch cadence")
        .hop_frames();
    let mut ready = Vec::new();
    let mut last = None;
    for block in 0..STREAM_BLOCK + 80 {
        if block == STREAM_BLOCK {
            let started = rig
                .stream(&StreamRequest::Start {
                    smoothing_ms: 100.0,
                })
                .expect("start");
            assert_eq!(started.status, StreamStatus::Warming);
            reference
                .service
                .start_stream(
                    SpectrumSmoothingConfig::new(100.0).expect("smoothing"),
                    Some(&mut analysis),
                )
                .expect("reference start");
        }
        rig.block();
        reference.block();
        if block < STREAM_BLOCK {
            continue;
        }
        let reply = rig.stream(&StreamRequest::Read).expect("read");
        reference
            .service
            .read_stream(Some(&mut analysis))
            .expect("reference read");
        let mut want = *reference.service.stream_metadata();
        let mut bins = host_core::SpectrumAnalysisResult::new();
        if let Some(window) = reference.service.stream_window() {
            analysis
                .analyze_stream_into(window, &mut bins, &mut want)
                .expect("reference analysis");
        }
        assert_eq!(stream_facts(&reply), core_facts(&want), "block {block}");
        if reply.status == StreamStatus::Ready {
            let got = reply.bins.as_ref().expect("ready bins");
            if ready.len() < 4 {
                assert!(
                    bins.left_dbfs.iter().any(|bin| *bin > -60.0),
                    "a spectrum of signal"
                );
                assert!(same_words(&got.frequencies_hz, &bins.frequencies_hz));
                assert!(same_words(
                    got.left_db.as_deref().expect("left"),
                    &bins.left_dbfs
                ));
                assert!(same_words(
                    got.right_db.as_deref().expect("right"),
                    &bins.right_dbfs
                ));
            }
            ready.push(reply.captured_sample);
        }
        last = Some(reply);
    }
    assert!(ready.len() >= 4, "{} ready windows", ready.len());
    assert!(
        ready
            .windows(2)
            .all(|pair| pair[1] - pair[0] == u64::from(hop))
    );
    let before = last.expect("a read");

    let selected = rig
        .stream(&StreamRequest::Select {
            target_kind: SpectrumTargetKind::TrackPostInput,
            target_id: "eq1".to_owned(),
            channels: ParameterChannel::Left,
            smoothing_ms: Some(50.0),
        })
        .expect("select eq1");
    assert_eq!(
        (
            selected.status,
            selected.selection_epoch,
            selected.analysis_epoch,
            selected.smoothing_ms
        ),
        (
            StreamStatus::Warming,
            before.selection_epoch + 1,
            before.analysis_epoch + 1,
            50.0
        )
    );
    let mut reads = 0;
    let window = loop {
        rig.block();
        let reply = rig.stream(&StreamRequest::Read).expect("read eq1");
        if reply.status == StreamStatus::Ready {
            break reply;
        }
        reads += 1;
        assert!(reads < 64, "eq1 reads ready");
    };
    let bins = window.bins.as_ref().expect("bins");
    assert!(bins.left_db.is_some() && bins.right_db.is_none());
    for _ in 0..3 * hop / QUANTUM as u32 {
        rig.block();
    }
    let gap = rig.stream(&StreamRequest::Read).expect("gap");
    assert_eq!(
        (gap.status, gap.dropped_captures),
        (StreamStatus::Gap, 2),
        "{gap:?}"
    );
    rig.block();
    let mut after = rig.stream(&StreamRequest::Read).expect("after the gap");
    while after.status != StreamStatus::Ready {
        rig.block();
        after = rig.stream(&StreamRequest::Read).expect("after the gap");
    }
    let reset = rig.stream(&StreamRequest::Reset).expect("reset");
    assert_eq!(
        (reset.status, reset.analysis_epoch),
        (StreamStatus::Pending, after.analysis_epoch + 1)
    );
    assert_eq!(
        rig.capture(&CaptureRequest::Cancel),
        Err((
            StatusCode::ValidationFailed,
            "spectrum.stream.active".to_owned()
        ))
    );
    let stopped = rig.stream(&StreamRequest::Stop).expect("stop");
    assert_eq!(stopped.status, StreamStatus::Stopped);
}

/// #1595 gate 3 (stream) and R6. Turns red if a stream read loses a drop count or R6 measures a
/// tap's reads wrongly. On a session whose frame limit is exactly a stereo stream read (12,688
/// bytes), a stereo `eq0` stream whose queue dropped two windows answers a read `gap` with both
/// drops, and the next read `ready`. R6 refuses a `Left` tap one byte below its stream read
/// (8,576 bytes, above the one-shot read's 8,496) and a stereo tap one byte below its two-plane
/// stream read, by a configure and by a boot, and accepts each at its size. (The read's own size
/// preflight is defence in depth: R6 leaves it no accepted entry to refuse.)
#[test]
fn a_stream_read_at_the_frame_limit_delivers_its_gap() {
    let codec = ProtocolCodec::default();
    let size = |bytes: Result<usize, protocol::EncodeError>| {
        u64::try_from(bytes.expect("a ready frame")).expect("a size")
    };
    let stereo = size(codec.spectrum_stream_ready_frame_len(ParameterChannel::Both, 3));
    let one_plane = size(codec.spectrum_stream_ready_frame_len(ParameterChannel::Left, 3));
    let one_shot = size(codec.spectrum_capture_ready_frame_len(ParameterChannel::Left, false, 3));
    assert!(one_shot < one_plane && one_plane < stereo);
    let at = |limit: u64| ControlLimits {
        maximum_control_frame_bytes: limit,
        ..limits()
    };
    let mut rig = Rig::limited(SESSION, Some(configuration()), at(stereo));
    assert_eq!(
        rig.stream(&StreamRequest::Start { smoothing_ms: 0.0 })
            .map(|reply| reply.status),
        Ok(StreamStatus::Warming)
    );
    let mut blocks = 0;
    loop {
        rig.block();
        let reply = rig.stream(&StreamRequest::Read).expect("read");
        if reply.status == StreamStatus::Ready {
            break;
        }
        blocks += 1;
        assert!(blocks < 64, "the stream reads ready");
    }
    let hop = rig.service().stream_metadata().hop_frames;
    for _ in 0..3 * hop / QUANTUM as u32 {
        rig.block();
    }
    let gap = rig.stream(&StreamRequest::Read).expect("gap");
    assert_eq!((gap.status, gap.dropped_captures), (StreamStatus::Gap, 2));
    rig.block();
    let mut after = rig.stream(&StreamRequest::Read).expect("after the gap");
    while after.status != StreamStatus::Ready {
        rig.block();
        after = rig.stream(&StreamRequest::Read).expect("after the gap");
    }

    let left = SpectrumConfigureRequest {
        entries: vec![entry(
            SpectrumTargetKind::TrackPostPan,
            "eq0",
            ParameterChannel::Left,
        )],
        ..single_request()
    };
    let refused = Err((StatusCode::LimitExceeded, "spectrum.frame".to_owned()));
    for (request, fits) in [(&left, one_plane), (&single_request(), stereo)] {
        let mut below = Rig::limited(SESSION, None, at(fits - 1));
        assert_eq!(below.configure(request), refused, "{fits} - 1");
        let mut exact = Rig::limited(SESSION, None, at(fits));
        assert!(exact.configure(request).is_ok(), "{fits}");
    }
}

/// #2166: a boot command that does not decode, or that names another command than
/// `SPECTRUM_CONFIGURE`, refuses the boot with `control.boot.command`.
///
/// Red if the boot ignores a frame it cannot apply (it would boot without the taps the host
/// staged, and the SDK's boot would fail only at its confirmation).
#[test]
fn a_boot_command_that_is_not_a_spectrum_configure_refuses_the_boot() {
    let mut read = vec![0_u8; 4_096];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::SpectrumStream(&StreamRequest::Read),
            },
            &mut read,
        )
        .expect("a stream frame");
    read.truncate(length);
    for (label, frame) in [
        ("malformed", &b"not a frame"[..]),
        ("wrong kind", &read[..]),
    ] {
        let Err(failure) = control_plane::compile_children_with_boot_command::<Adapter>(
            SESSION,
            limits(),
            Box::new(Preparer),
            frame,
        ) else {
            panic!("{label}: the boot accepted the command");
        };
        assert_eq!(
            failure.diagnostic().as_ref(),
            b"control.boot.command\t$\n",
            "{label}"
        );
    }
}
