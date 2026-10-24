//! #1593: the session's spectrum taps are monitor state that rides every plan it prepares.
//!
//! The preparer here is the C ABI's: host-core's one session entry, with the session's spectrum
//! preparation as the session passes it. Gate 1 (the first plan reads the reference window) and
//! gate 5 (the configuration is in no revision and no snapshot) run on it.
#![cfg(feature = "test-support")]

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, PlanResources, PlanState, RuntimePreparer,
    SessionState, compile_children, prepare_caps,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    PreparedSpectrumCaptures, SPECTRUM_WINDOW_FRAMES, SourceControlSet, SourceSubmission,
    SpectrumCaptureRequest, SpectrumChannels, SpectrumConfiguration, SpectrumPreparation,
    SpectrumService, SpectrumSnapshot, SpectrumTarget, StartedRenderSession, SuccessorBase,
    compile_host_session, prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_spectrum, spectrum_capture_resources_for,
};
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionSnapshotRequest,
    TypedCommandFrame,
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
        maximum_submixes: 0,
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
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(SESSION, limits(), Box::new(Preparer), spectrum)
                .expect("boot compile");
        Self {
            session,
            plan,
            frame: 0,
            request: 0,
        }
    }

    fn block(&mut self) {
        let (left, right) = tone(self.frame);
        self.session
            .submit(SOURCE, submission(self.frame, &[&left, &right]))
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        self.plan.render(self.frame, output).expect("render block");
        self.frame += QUANTUM as u64;
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
