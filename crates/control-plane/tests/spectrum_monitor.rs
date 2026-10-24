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
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionSnapshotRequest,
    TypedCommandFrame,
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
        maximum_automation_spans_per_block: 128,
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
}

impl Rig {
    fn new(spectrum: Option<SpectrumConfiguration>) -> Self {
        Self::of(SESSION, spectrum)
    }

    fn of(document: &str, spectrum: Option<SpectrumConfiguration>) -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(document, limits(), Box::new(Preparer), spectrum)
                .expect("boot compile");
        Self {
            session,
            plan,
            frame: 0,
            request: 0,
        }
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
            assert!(blocks <= 4, "the candidate is promoted within a few blocks");
        }
    }

    /// Read the stream after a block: its status and window.
    fn read_stream(&mut self) -> Read {
        let (service, analysis) = self
            .session
            .test_spectrum_mut()
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
            .test_spectrum_mut()
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
            .test_spectrum_mut()
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
        let caps = prepare_caps(limits());
        let compiled = compile_host_session(SESSION, &caps).expect("compiled fixture");
        let (host, capture) =
            prepare_host_runtime_with_spectrum(&compiled, &caps, &request()).expect("reference");
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

/// Gate 1. Turns red if the session's configuration does not reach the first plan's graph: the
/// session's window (armed at frame 0, read after 2,048 frames) equals, field by field, the window
/// of `prepare_host_runtime_with_spectrum` on the same document fed the same PCM.
#[test]
fn the_first_plan_reads_the_reference_window() {
    let mut rig = Rig::new(Some(configuration()));
    assert_eq!(rig.session.test_spectrum().configured_epoch(), 1);
    assert_eq!(rig.session.test_spectrum().adopted_epoch(), 1);
    let got: SpectrumSnapshot = one_window!(rig.service(), rig.block());
    let mut reference = Reference::new();
    let want: SpectrumSnapshot = one_window!(reference.service, reference.block());
    assert_eq!(got.target_kind, want.target_kind);
    assert_eq!(got.sample_rate_hz, want.sample_rate_hz);
    assert_eq!(got.token, want.token);
    assert_eq!(got.window.first_sample, want.window.first_sample);
    assert_eq!(got.window.channels, want.window.channels);
    assert_eq!(got.window.source_underrun, want.window.source_underrun);
    for (got, want) in [
        (&got.window.left, &want.window.left),
        (&got.window.right, &want.window.right),
    ] {
        assert!(
            got.iter().any(|sample| *sample != 0.0),
            "a window of signal"
        );
        assert!(
            got.iter()
                .zip(want)
                .all(|(got, want)| got.to_bits() == want.to_bits()),
            "the planes equal the reference's bit for bit"
        );
    }
}

/// Gate 5. Turns red if spectrum state leaks into the model, the revision or the snapshot: a
/// session booted with a configuration has the revision and the `SESSION_SNAPSHOT_GET` bytes of
/// one booted without, before and after it reads a window; only its monitor differs.
#[test]
fn the_configuration_is_in_no_revision_and_no_snapshot() {
    let mut configured = Rig::new(Some(configuration()));
    let mut plain = Rig::new(None);
    let monitor = plain.session.test_spectrum();
    assert!(monitor.configuration().is_none() && monitor.service().is_none());
    assert!(monitor.analysis().is_none());
    assert_eq!(
        (monitor.configured_epoch(), monitor.adopted_epoch()),
        (0, 0)
    );
    assert_eq!(
        configured.session.test_spectrum().configuration(),
        Some(&configuration())
    );
    let revision = |rig: &Rig| rig.session.test_controller().session().revision();
    assert_eq!(revision(&configured), revision(&plain));
    assert_eq!(configured.snapshot(), plain.snapshot());
    let _: SpectrumSnapshot = one_window!(configured.service(), configured.block());
    assert_eq!(revision(&configured), revision(&plain));
    assert_eq!(configured.snapshot(), plain.snapshot());
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

/// Gate 2, with gates 5 and 6 over its transaction and renders. Turns red if a successor is
/// prepared without the taps, if the readers are not carried onto the successor's observers, or
/// if service state resets at a swap: a stream on `eq0` (selected in a collection with `eq1`)
/// started at 100 ms reads on across a structural edit that adds a muted track, with no `Failed`,
/// `Gap` or `Warming` read, unchanged epochs, no sequence gap, and every read equal to that of a
/// session that had the track from the start; the promoted service reads the configuration's
/// epoch. The transaction advances the revision by one and the snapshot equals a plain session's.
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
    let monitor = rig.session.test_spectrum();
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
    assert_eq!(
        rig.snapshot(),
        plain.snapshot(),
        "no spectrum in the snapshot"
    );

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
    let monitor = rig.session.test_spectrum();
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
    assert_eq!(
        rig.snapshot(),
        plain.snapshot(),
        "no spectrum in the snapshot"
    );

    rig.rebuild(&remove_track("eq0"));
    plain.rebuild(&remove_track("eq0"));
    let monitor = rig.session.test_spectrum();
    assert!(monitor.configuration().is_none());
    assert_eq!(monitor.configured_epoch(), 3);
    rig.until_promoted();
    let monitor = rig.session.test_spectrum();
    assert!(monitor.service().is_none() && monitor.analysis().is_none());
    assert_eq!(monitor.adopted_epoch(), 3);
    assert_eq!(
        rig.snapshot(),
        plain.snapshot(),
        "no spectrum in the snapshot"
    );

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
    assert_eq!(armed.session.test_spectrum().adopted_epoch(), 1);
    armed.until_promoted();
    assert_eq!(armed.session.test_spectrum().adopted_epoch(), 2);
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
    let monitor = rig.session.test_spectrum();
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
        rig.session.test_spectrum().configuration(),
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
