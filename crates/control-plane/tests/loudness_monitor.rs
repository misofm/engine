//! #2115: the session's output loudness meter through the real session, as both hosts drive it:
//! submit, render, `observe_rendered_block`, then the service step, with `LOUDNESS` (`0x0014`)
//! commands between blocks.
#![cfg(feature = "test-support")]

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, PlanResources, PlanState, RuntimePreparer,
    SessionState, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    PreparedSpectrumCaptures, SourceSubmission, SpectrumPreparation, SuccessorBase,
    prepare_host_runtime_with_live_lanes_and_spectrum,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    ExpectedRevision, LoudnessReply, LoudnessRequest, ProtocolCodec, RequestId, StatusCode,
    TypedCommandFrame,
};

const SOURCE: &[u8] = b"tone";
const QUANTUM: usize = 128;
const SAMPLE_RATE_HZ: u32 = 48_000;
/// Samples per 10 ms hop at 48 kHz.
const HOP: u64 = 480;
/// Blocks per 1,920 samples, the least common multiple of the hop and the quantum.
const ALIGNED_BLOCKS: u64 = 15;

/// One track playing the stereo source at unity (trim 0, fader 0, the left channel panned hard
/// left and the right hard right, an identity route) into the one output, with no effect.
const SESSION: &str = r#"{
  "schema_version": 1,
  "session_id": "loudness.session",
  "revision": "0",
  "sample_rate_hz": 48000,
  "quantum_frames": 128,
  "render_profile": { "id": "single", "mode": "single_thread" },
  "output_profile": { "id": "main", "channels": 2, "sample_format": "f32_planar" },
  "sources": [
    {
      "id": "tone",
      "content": "blake3:7e945c107a97cd24135e85dc2f407c5ecd39663a8737bf5b92114ccce38f1ab8",
      "channels": 2,
      "bit_depth": "32f",
      "frames": "48000000"
    }
  ],
  "console": { "pre_insert": [], "post_insert": [] },
  "tracks": [
    {
      "id": "t0",
      "source_id": "tone",
      "left_source_channel": 0,
      "right_source_channel": 1,
      "builtins": {
        "left": { "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 20.0, "lpf_hz": 20000.0, "delay_samples": 0 },
        "right": { "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 20.0, "lpf_hz": 20000.0, "delay_samples": 0 }
      },
      "console": [],
      "inserts": { "effects": [] },
      "fader": { "left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false },
      "pan": { "left": -1.0, "right": 1.0, "smoothing_samples": 16 }
    }
  ],
  "submixes": [],
  "vcas": [],
  "control_smoothing": { "kind": "default" },
  "outputs": [ { "id": "main-out" } ],
  "routes": [
    {
      "id": "t0-main",
      "source": { "kind": "track", "track_id": "t0", "tap": "post_pan" },
      "destination": { "kind": "output_input", "output_id": "main-out" },
      "channel_matrix": { "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 },
      "gain_db": 0.0,
      "mute": false,
      "follows_mute": false
    }
  ],
  "automation": []
}"#;

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

/// A 1 kHz sine at -23 dBFS on both channels: Tech 3341 case 1, which reads -23 LUFS.
fn tone(frame: u64) -> [f32; QUANTUM] {
    let amplitude = math::db_to_gain(-23.0);
    core::array::from_fn(|index| {
        let t = (frame + index as u64) as f64 / f64::from(SAMPLE_RATE_HZ);
        (amplitude * math::sin(2.0 * core::f64::consts::PI * 1_000.0 * t)) as f32
    })
}

struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    frame: u64,
    request: u64,
}

impl Rig {
    fn new() -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(SESSION, limits(), Box::new(Preparer))
                .expect("boot compile");
        Self {
            session,
            plan,
            frame: 0,
            request: 0,
        }
    }

    /// Feed, render and observe one block as both hosts do, then run the service step when
    /// `serviced` (a submit without it, as the browser's render-side feed submits).
    fn block(&mut self, serviced: bool) {
        let samples = tone(self.frame);
        let planes: [&[f32]; 2] = [&samples, &samples];
        let submission = SourceSubmission {
            generation: 1,
            start_frame: self.frame,
            sample_rate_hz: SAMPLE_RATE_HZ,
            planes: &planes,
            frames: QUANTUM as u32,
            end_of_region: false,
        };
        self.session
            .submit_unserviced(SOURCE, submission)
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        let buffer = engine::realtime::PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        self.plan.render(self.frame, buffer).expect("render block");
        let next = self.plan.owner().next_absolute_sample();
        let (left, right) = output.split_at(QUANTUM);
        self.plan
            .observe_rendered_block([left, right], self.frame, next);
        self.frame = next;
        if serviced {
            self.session.service().expect("service step");
        }
    }

    fn blocks(&mut self, count: u64, serviced: bool) {
        for _ in 0..count {
            self.block(serviced);
        }
    }

    /// Send one `LOUDNESS`: its reply, or the refusal's status and code.
    fn loudness(
        &mut self,
        request: LoudnessRequest,
    ) -> Result<LoudnessReply, (StatusCode, String)> {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(self.request).expect("nonzero request"),
            expected_revision: ExpectedRevision::Any,
            payload: CommandPayload::Loudness(request),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("loudness command");
        let length = self
            .session
            .command(&bytes[..length], 65_536)
            .expect("a loudness command answers a frame");
        let mut fields = [0_u16; 64];
        match codec
            .decode_typed_response(
                self.session.command_response(length),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a loudness response")
        {
            DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::Loudness(reply),
                ..
            } => Ok(reply),
            DecodedTypedResponseFrame::NonOk { header, payload } => {
                Err((header.status, payload.diagnostics[0].code.clone()))
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        }
    }
}

fn within(level: Option<f32>, label: &str) {
    let level = level.unwrap_or_else(|| panic!("{label} is defined"));
    assert!((level + 23.0).abs() <= 0.1, "{label} {level} LUFS");
}

/// Start, read, reset and stop on a -23 dBFS 1 kHz stereo sine through the real session.
///
/// Test value: red if the meter is not wired to the rendered master (no hops, or the wrong
/// planes), if start arms from the wrong sample or a second start does not refuse, if a read
/// does not drain first, if reset does not restart integration on a hop boundary at the render
/// sample, if stop leaves the meter armed, or if a start after an immediate stop continues the
/// last arming's windows instead of restarting the analysis.
#[test]
fn start_read_reset_and_stop_measure_the_master() {
    let mut rig = Rig::new();
    rig.blocks(3, true);
    let started = rig.loudness(LoudnessRequest::Start).expect("start");
    let origin = rig.frame;
    assert!(started.armed);
    assert_eq!(
        started.observed_sample, origin,
        "armed from the render position"
    );
    assert_eq!(started.integrating_since, None);
    assert_eq!(
        rig.loudness(LoudnessRequest::Start),
        Err((StatusCode::ValidationFailed, "loudness.active".to_owned())),
        "a start while armed is refused"
    );

    // 5 s: the 3 s short-term window is full and integration has many gating blocks.
    rig.blocks(1_875, true);
    let read = rig.loudness(LoudnessRequest::Read).expect("read");
    within(read.momentary_lufs, "M");
    within(read.short_term_lufs, "S");
    within(read.integrated_lufs, "I");
    assert_eq!(
        read.integrating_since,
        Some(origin),
        "the first hop starts at the arming"
    );
    assert_eq!(
        read.observed_sample,
        origin + (rig.frame - origin) / HOP * HOP
    );
    assert_eq!(read.hops_dropped, 0);
    let [left, right] = read.true_peak_dbtp.expect("a true peak");
    assert!(
        (left + 23.0).abs() < 0.1 && (right + 23.0).abs() < 0.1,
        "{left} {right} dBTP"
    );

    let reset_at = rig.frame;
    let reset = rig.loudness(LoudnessRequest::Reset).expect("reset");
    assert!(
        reset.momentary_lufs.is_some(),
        "M keeps running across a reset"
    );
    assert_eq!(reset.integrated_lufs, None, "a reset clears integration");
    rig.blocks(ALIGNED_BLOCKS, true);
    let since = rig
        .loudness(LoudnessRequest::Read)
        .expect("read")
        .integrating_since
        .expect("a kept hop after the reset");
    assert!(
        since >= reset_at && since > origin,
        "{since} after {reset_at}"
    );
    assert_eq!((since - origin) % HOP, 0, "on the hop grid");

    // Stop at a sample where a hop ends, then start at once: the analysis restarts, so no
    // momentary window joins the two armings.
    rig.blocks(
        ALIGNED_BLOCKS - (rig.frame - origin) / QUANTUM as u64 % ALIGNED_BLOCKS,
        true,
    );
    assert_eq!((rig.frame - origin) % HOP, 0);
    let stopped = rig.loudness(LoudnessRequest::Stop).expect("stop");
    assert!(!stopped.armed);
    assert_eq!(
        stopped.observed_sample, rig.frame,
        "the stop drained every hop"
    );
    assert!(
        stopped.momentary_lufs.is_some(),
        "the readings stay readable"
    );
    let restarted = rig.loudness(LoudnessRequest::Start).expect("start again");
    assert_eq!(
        restarted.momentary_lufs, None,
        "a start restarts the analysis"
    );
    rig.blocks(ALIGNED_BLOCKS, true);
    let early = rig.loudness(LoudnessRequest::Read).expect("read");
    assert!(
        early.observed_sample > restarted.observed_sample,
        "the new arming delivers"
    );
    assert_eq!(
        early.momentary_lufs, None,
        "4 hops of the new arming are no 400 ms window"
    );
}

/// More than 512 hops render with no service step and no read: the queue keeps 512, render
/// drops and counts the rest, and the next read still answers, with the exact count.
///
/// Test value: red if a full record queue blocks, panics or overwrites, if a drop is not counted
/// or counted twice, or if a read after the overflow fails or reports a hop it did not deliver.
#[test]
fn a_full_record_queue_drops_and_counts_exactly() {
    let mut rig = Rig::new();
    let started = rig.loudness(LoudnessRequest::Start).expect("start");
    let origin = started.observed_sample;
    // 2,400 blocks of 128 are 640 hops of 480.
    rig.blocks(2_400, false);
    let read = rig
        .loudness(LoudnessRequest::Read)
        .expect("read after the overflow");
    assert_eq!(read.hops_dropped, 640 - 512);
    assert_eq!(
        read.observed_sample,
        origin + 512 * HOP,
        "the 512 queued hops delivered"
    );
    assert!(read.armed);
}
