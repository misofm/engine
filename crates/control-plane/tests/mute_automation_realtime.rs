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
        _spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            HostLiveControlHandles,
            Option<host_core::PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let live = HostLiveControlRequest::default();
        let (host, handles) = match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }?;
        Ok((host, handles, None))
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
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
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

/// `eq0`'s mute driven by a step curve on both lanes, closed over `close..2_500` of a source of
/// `frames` frames.
fn curve(close: u64, frames: u64) -> Automation {
    let step = |start_sample, end_sample, value| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample,
        end_sample,
        start_value: value,
        end_value: value,
        unit: ParameterUnit::Linear,
    };
    Automation {
        id: id("eq0-mute-curve"),
        target: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Builtins,
            control: 6,
            lane: ParameterChannel::Both,
        },
        segments: vec![
            step(0, close, 0.0),
            step(close, 2_500, 1.0),
            step(2_500, frames, 0.0),
        ],
        offset: AutomationOffset::default(),
    }
}

/// The fixture with VCA `v` holding `eq0` and, with `curve`, `eq0`'s mute driven by a step curve
/// on both lanes, closed over `1_000..2_500`.
fn document(with_curve: bool) -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    let frames = model.sources[0].frames;
    model
        .automation
        .extend(with_curve.then(|| curve(1_000, frames)));
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
        self.feed();
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.plan.render(self.at, output).expect("a rendered block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        self.last = samples.iter().map(|sample| sample.to_bits()).collect();
        self.at += QUANTUM as u64;
        used
    }

    /// Submit the source's next block.
    fn feed(&mut self) {
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
        self.start += QUANTUM as u64;
    }

    /// Commit `value` at `address` live (the session's ramp).
    fn set(&mut self, address: ControlAddress, value: f32) {
        let edits = [SessionEdit::SetControl {
            expected_effect: None,
            address,
            value,
            ramp_samples: None,
        }];
        let path = self.transact(&edits);
        assert_eq!(path, EditPath::Live, "a mute term is live");
    }

    /// Commit `edits` as one transaction; its edit path.
    fn transact(&mut self, edits: &[SessionEdit]) -> EditPath {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
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
        self.revision += 1;
        self.drain();
        path
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

    /// A plain session seek to `timeline` with its runway (declared: a successor at the next
    /// block).
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.session
            .session_seek(self.generation, timeline, None)
            .expect("a session seek");
        self.drain();
        self.start = timeline;
        // #2205: the seek's runway, so it lands at the next block; `block` keeps it ahead.
        for _ in 0..host_core::source_seek_runway_blocks(48_000, QUANTUM as u32, 1_024) {
            self.feed();
        }
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

/// #1651 U1 fold 1 (MAJOR-2): a hand mute committed in the same transaction as a mute curve edit
/// lands as committed. At block 4 one transaction moves `eq0`'s closing step from 1,000 to 1,100
/// (a carried rebuild: the curve's table changes, so the adopted cell repositions) and sets `eq0`'s
/// hand mute with `ramp_samples` absent (the session's mute length), `0` and `2,000`. A twin with
/// the old curve takes the hand mute alone, live, with the same ramp: the curve's change lies
/// inside the muted span the hand mute holds, so every block of the rebuilt run equals the twin's,
/// bit for bit, through the ramp and after the curve's opening (still held by the hand mute).
///
/// Test value: red if the swap block's adoption drops the record's ramp word and lands the hand
/// mute over the mute length instead (a `0` step becomes a 480-sample ramp, a 2,000-sample fade
/// becomes 480), or lands it twice.
#[test]
fn a_hand_mute_committed_with_a_mute_curve_edit_lands_as_committed() {
    let frames = session::parse_session_json(SESSION)
        .expect("fixture")
        .sources[0]
        .frames;
    let hand = ControlAddress {
        owner: Owner::Strip(id("eq0")),
        section: Section::Builtins,
        control: 6,
        lane: ParameterChannel::Both,
    };
    for ramp_samples in [None, Some(0), Some(2_000)] {
        let mut rig = Rig::new(true);
        let mut twin = Rig::new(true);
        let mute = SessionEdit::SetControl {
            expected_effect: None,
            address: hand.clone(),
            value: 1.0,
            ramp_samples,
        };
        for block in 0..40_u64 {
            if block == 4 {
                let path = rig.transact(&[
                    SessionEdit::UpsertAutomation {
                        automation: curve(1_100, frames),
                    },
                    mute.clone(),
                ]);
                assert_eq!(path, EditPath::Rebuild, "{ramp_samples:?}: the curve edit");
                let path = twin.transact(std::slice::from_ref(&mute));
                assert_eq!(path, EditPath::Live, "{ramp_samples:?}: the hand mute");
            }
            rig.block();
            twin.block();
            assert_eq!(
                rig.last, twin.last,
                "{ramp_samples:?}: block {block} equals the hand mute alone"
            );
        }
    }
}
