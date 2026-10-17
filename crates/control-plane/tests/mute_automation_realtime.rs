//! #1651 gate 5: stored mute automation (builtin row 6) renders allocation-free.
//!
//! The script of gate 1 runs through control-plane's shared commit (`SessionState::command`, the
//! code the browser Worker and the C ABI both run) and host-core's render session
//! (`PlanState::render`): `eq0`'s mute is driven by a step curve on `both` lanes (unmuted, muted
//! from sample 1,000 to 2,500, unmuted; neither a grid sample), and the run crosses both steps,
//! takes the mute terms the curve does not drive (a hand mute of `eq0`, OQ1 = C, and a VCA mute of
//! it, each set and released live), and ends with a plain session seek into the muted span and
//! the curve's opening after it. Every render call after the warm-up allocates and frees nothing,
//! measured with the audited allocator's thread-scoped counters: the blocks of each curve step
//! and its ramp, of each terms write, and of the seek's swap and landing.

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, SourceSubmission,
    SuccessorBase, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ControlAddress,
    DualMonoFader, Owner, ParameterChannel, ParameterUnit, Section, StableId, Vca,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const SOURCE: &[u8] = b"fixture-source";

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

struct Preparer;

impl RuntimePreparer for Preparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
        let live = HostLiveControlRequest::default();
        match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }
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
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 65_536,
        maximum_replay_bytes: 262_144,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 4,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The fixture with VCA `v` holding `eq0` and, with `curve`, `eq0`'s mute driven by a step curve
/// on both lanes, closed over `1_000..2_500`.
fn document(curve: bool) -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    let frames = model.sources[0].frames;
    let step = |start_sample, end_sample, value| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample,
        end_sample,
        start_value: value,
        end_value: value,
        unit: ParameterUnit::Linear,
    };
    model.automation.extend(curve.then(|| Automation {
        id: id("eq0-mute-curve"),
        target: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Builtins,
            control: 6,
            lane: ParameterChannel::Both,
        },
        segments: vec![
            step(0, 1_000, 0.0),
            step(1_000, 2_500, 1.0),
            step(2_500, frames, 0.0),
        ],
        offset: AutomationOffset::default(),
    }));
    model.vcas.push(Vca {
        id: id("v"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id("eq0")],
    });
    (
        session::canonical_session_json(&model).expect("canonical"),
        model.revision,
    )
}

struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    revision: u64,
    request: u64,
    /// The next block's render sample, and the source generation and frame it is fed from.
    at: u64,
    generation: u64,
    start: u64,
    /// The last block's samples, left plane then right.
    last: Vec<u32>,
}

impl Rig {
    /// A rig on [`document`], booted.
    fn new(curve: bool) -> Self {
        let (document, revision) = document(curve);
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document, limits(), Box::new(Preparer))
                .expect("boot compile");
        Self {
            session,
            plan,
            revision,
            request: 1,
            at: 0,
            generation: 1,
            start: 0,
            last: Vec::new(),
        }
    }

    /// Feed and render one block; the allocator events of the render call alone.
    fn block(&mut self) -> bench_support::alloc::Counters {
        let start = self.start;
        // Negative pairs in every fourth stretch of 300 frames.
        let plane: Vec<f32> = (0..QUANTUM as u64)
            .map(|frame| {
                let value = ((start + frame) % 97) as f32 / 97.0 - 0.5 + 1.0e-3;
                if ((start + frame) / 300) % 4 == 1 {
                    -value.abs()
                } else {
                    value
                }
            })
            .collect();
        self.session
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: self.generation,
                    start_frame: start,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.plan.render(self.at, output).expect("a rendered block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        self.last = samples.iter().map(|sample| sample.to_bits()).collect();
        self.at += QUANTUM as u64;
        self.start += QUANTUM as u64;
        used
    }

    /// Commit `value` at `address` live (the session's ramp).
    fn set(&mut self, address: ControlAddress, value: f32) {
        self.request += 1;
        let edits = [SessionEdit::SetControl {
            expected_effect: None,
            address,
            value,
            ramp_samples: None,
        }];
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        bytes.truncate(length);
        let length = self
            .session
            .command(&bytes, 65_536)
            .expect("the edit commits");
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
            } => applied.edit_path,
            DecodedTypedResponseFrame::NonOk { payload, .. } => {
                panic!("the edit is refused: {payload:?}")
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        };
        assert_eq!(path, EditPath::Live, "a mute term is live");
        self.revision += 1;
        self.drain();
    }

    /// The commit's reliable events, read as a host reads them.
    fn drain(&mut self) {
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
    }

    /// A plain session seek to `timeline` (declared: a successor at the next block).
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.session
            .session_seek(self.generation, timeline, None)
            .expect("a session seek");
        self.drain();
        self.start = timeline;
    }
}

/// Gate 5. `eq0`'s curve closes at 1,000 and opens at 2,500 (blocks 7 and 19 at quantum 128);
/// a hand mute is set at block 4 (while the curve is open) and released at block 10 (closed); a
/// VCA mute is set at block 12 and released at block 22 (after the opening); a plain session seek
/// to timeline 1,600 lands at block 30, inside the muted span, and the curve opens again 900
/// samples later. Every render call after the four warm-up blocks allocates and frees nothing. A
/// twin with no curve and no edit, fed and seeked alike, differs inside each mute and equals the
/// rig at the end (so the script is heard).
///
/// Test value: red if the mute cursor's events, the composition of `curve || terms`, the drain of
/// a terms write or the seek's landing on a mute cell allocates or frees on render.
#[test]
fn stored_mute_automation_renders_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(true);
    // The same session with no curve and no edit, fed and seeked alike: the script is heard.
    let mut plain = Rig::new(false);
    for _ in 0..4 {
        rig.block();
        plain.block();
    }
    let hand = ControlAddress {
        owner: Owner::Strip(id("eq0")),
        section: Section::Builtins,
        control: 6,
        lane: ParameterChannel::Both,
    };
    let vca = ControlAddress {
        owner: Owner::Vca(id("v")),
        section: Section::Own,
        control: 2,
        lane: ParameterChannel::Both,
    };
    for block in 4..48_u64 {
        match block {
            4 => rig.set(hand.clone(), 1.0),
            10 => rig.set(hand.clone(), 0.0),
            12 => rig.set(vca.clone(), 1.0),
            22 => rig.set(vca.clone(), 0.0),
            30 => {
                rig.seek(1_600);
                plain.seek(1_600);
            }
            _ => {}
        }
        let used = rig.block();
        plain.block();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "block {block} allocated or freed"
        );
        match block {
            // The curve's muted span, the hand mute and the VCA mute are heard.
            8 | 16 | 21 | 31 => assert_ne!(rig.last, plain.last, "block {block} mutes eq0"),
            // Open again, with no term: the session without the curve.
            47 => assert_eq!(rig.last, plain.last, "block {block} is open"),
            _ => {}
        }
    }
}
