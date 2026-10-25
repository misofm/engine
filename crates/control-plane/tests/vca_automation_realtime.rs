//! #1649 gate 6: a VCA edit over a member with stored fader automation is read and rendered
//! allocation-free.
//!
//! The script of gate 1 runs through control-plane's shared commit (`SessionState::command`, the
//! code the browser Worker and the C ABI both run) and host-core's render session
//! (`PlanState::render`): `eq0` rides its fader, VCA `v` holds `eq0` and `eq1` (no automation),
//! and `v` moves live, with the session's length, with an edit length of 256 samples and twice
//! inside one block. Every render call after the warm-up allocates and frees nothing, measured
//! with the audited allocator's thread-scoped counters: the block that reads the offsets cell and
//! jumps, and every block of the jump and the grid after it.
//!
//! #1650b gate 6 runs the same rig with hand moves of `eq0`'s automated fader (`SET_CONTROL
//! fader_db`, which writes the ride's offset and the lane's offsets cell) in place of the VCA moves.
//!
//! #1655 gate 5 runs it with `v` automated itself (an `offset_db` ride and `mute` steps): the
//! composed curves' events, a hand move of the automated VCA offset, a hand move of the member
//! with no own automation (an `own_db` write) and hand VCA mutes, each block allocation-free.

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

/// The fixture with `eq0` riding its fader on both lanes (linear, -0 to -12 dB over the source)
/// and VCA `v` holding `eq0` and `eq1`; with `vca_automation`, `v` also rides its `offset_db` (linear, 0 to -9 dB over
/// the source) and steps its `mute` (on at 1,000, off at 2,600, on at 5,000, off at 6,000).
fn document_with(vca_automation: bool) -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    let frames = model.sources[0].frames;
    model.automation.push(Automation {
        id: id("eq0-fader-ride"),
        target: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Builtins,
            control: 5,
            lane: ParameterChannel::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: frames,
            start_value: 0.0,
            end_value: -12.0,
            unit: ParameterUnit::Db,
        }],
        offset: AutomationOffset::default(),
    });
    model.vcas.push(Vca {
        id: id("v"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id("eq0"), id("eq1")],
    });
    if vca_automation {
        let vca_row = |control| ControlAddress {
            owner: Owner::Vca(id("v")),
            section: Section::Own,
            control,
            lane: ParameterChannel::Both,
        };
        let step = |start, end, from, to| AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: start,
            end_sample: end,
            start_value: from,
            end_value: to,
            unit: ParameterUnit::Linear,
        };
        model.automation.extend([
            Automation {
                id: id("v-ride"),
                target: vca_row(1),
                segments: vec![AutomationSegment {
                    shape: AutomationShape::Linear,
                    start_sample: 0,
                    end_sample: frames,
                    start_value: 0.0,
                    end_value: -9.0,
                    unit: ParameterUnit::Db,
                }],
                offset: AutomationOffset::default(),
            },
            Automation {
                id: id("v-mute"),
                target: vca_row(2),
                segments: vec![
                    step(0, 1_000, 0.0, 1.0),
                    step(1_000, 2_600, 1.0, 0.0),
                    step(2_600, 5_000, 0.0, 1.0),
                    step(5_000, 6_000, 1.0, 0.0),
                ],
                offset: AutomationOffset::default(),
            },
        ]);
    }
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
    block: u64,
    /// The last block's samples, left plane then right.
    last: Vec<f32>,
}

impl Rig {
    /// A rig on [`document_with`]`(false)`, booted.
    fn new() -> Self {
        Self::with(false)
    }

    /// A rig on [`document_with`], booted.
    fn with(vca_automation: bool) -> Self {
        let (document, revision) = document_with(vca_automation);
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document, limits(), Box::new(Preparer))
                .expect("boot compile");
        Self {
            session,
            plan,
            revision,
            request: 1,
            block: 0,
            last: vec![0.0; QUANTUM * 2],
        }
    }

    /// Feed and render one block; the allocator events of the render call alone.
    fn block(&mut self) -> bench_support::alloc::Counters {
        let start = self.block * QUANTUM as u64;
        let plane: Vec<f32> = (0..QUANTUM as u64)
            .map(|frame| ((start + frame) % 97) as f32 / 97.0 - 0.5)
            .collect();
        self.session
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: 1,
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
        self.plan.render(start, output).expect("a rendered block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        self.last.copy_from_slice(&samples);
        self.block += 1;
        used
    }

    /// Commit `v`'s offset at `db` on both lanes, over `ramp` samples or the session's length.
    fn move_vca(&mut self, db: f32, ramp: Option<u32>) {
        self.move_row(
            ControlAddress {
                owner: Owner::Vca(id("v")),
                section: Section::Own,
                control: 1,
                lane: ParameterChannel::Both,
            },
            db,
            ramp,
        );
    }

    /// Commit `value` at `address` live, over `ramp` samples or the session's length.
    fn move_row(&mut self, address: ControlAddress, db: f32, ramp: Option<u32>) {
        self.request += 1;
        let edits = [SessionEdit::SetControl {
            expected_effect: None,
            address,
            value: db,
            ramp_samples: ramp,
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
            .expect("the move commits");
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
                panic!("the move is refused: {payload:?}")
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        };
        assert_eq!(path, EditPath::Live, "the move is live");
        self.revision += 1;
        // The commit's reliable events, read as a host reads them.
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
    }
}

/// A rig on [`document_with`]`(false)`, warmed: the process statics and the first blocks' one-time work.
fn warm_rig() -> Rig {
    warm(Rig::new())
}

/// `rig` warmed: four blocks.
fn warm(mut rig: Rig) -> Rig {
    for _ in 0..4 {
        rig.block();
    }
    rig
}

/// Gate 6. Every render block after the warm-up allocates and frees nothing. The script ends
/// with `v` back at 0 dB, so once the last jump and the first grid ramp after it have passed
/// (twelve blocks of 128 samples against a jump of 960), the output equals an unedited rig's.
///
/// Test value: red if the offsets cell's read, the jump or the composition with the new `S`
/// allocates or frees on render, or if render hears only some of the writes of a cell (one that
/// ignores every write after the first is left at -6 dB).
#[test]
fn a_vca_move_over_an_automated_member_renders_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = warm_rig();
    let mut plain = warm_rig();
    let script: [&[(f32, Option<u32>)]; 4] = [
        &[(-6.0, None)],
        &[(-3.0, Some(256))],
        // Two moves before one block: render reads the newest slot only.
        &[(-9.0, None), (-1.5, Some(256))],
        &[(0.0, None)],
    ];
    for moves in script {
        for &(db, ramp) in moves {
            rig.move_vca(db, ramp);
        }
        for _ in 0..12 {
            let block = rig.block;
            let used = rig.block();
            plain.block();
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "block {block} allocated or freed"
            );
        }
    }
    let bits = |samples: &[f32]| {
        samples
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        bits(&rig.last),
        bits(&plain.last),
        "v back at 0 dB renders the unedited session"
    );
}

/// #1650b gate 6. Hand moves of `eq0`'s automated fader commit live (each writes the ride's offset
/// and the lane's offsets cell): on the left lane with the session's length, on both lanes with an
/// edit length of 256 samples, twice before one block, and back to `0`. Every render call after
/// the warm-up allocates and frees nothing.
///
/// Test value: red if the offsets-cell read, the jump or the composition with a hand offset's `S`
/// allocates or frees on render.
#[test]
fn a_hand_move_on_an_automated_fader_renders_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = warm_rig();
    let fader = |lane| ControlAddress {
        owner: Owner::Strip(id("eq0")),
        section: Section::Builtins,
        control: 5,
        lane,
    };
    let script: [&[(ParameterChannel, f32, Option<u32>)]; 4] = [
        &[(ParameterChannel::Left, 3.0, None)],
        &[(ParameterChannel::Both, -4.5, Some(256))],
        // Two moves before one block: render reads the newest slot only.
        &[
            (ParameterChannel::Right, 6.0, None),
            (ParameterChannel::Left, -1.5, Some(256)),
        ],
        &[(ParameterChannel::Both, 0.0, None)],
    ];
    for moves in script {
        for &(lane, db, ramp) in moves {
            rig.move_row(fader(lane), db, ramp);
        }
        for _ in 0..12 {
            let block = rig.block;
            let used = rig.block();
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "block {block} allocated or freed"
            );
        }
    }
}

/// #1655 gate 5. With `v` automated (its ride and mute steps compose into both members' lanes),
/// every render block after the warm-up allocates and frees nothing: blocks that only play the
/// composed curves (grid events of the ride, the mute steps), a hand move of the automated VCA
/// offset (it writes the ride's offset and `S` of both members), a hand move of `eq1`'s fader
/// (no own automation: an `own_db` write through its offsets cell), and a hand mute and unmute of
/// `v` (the members' `terms`, ORed with the curve). Each edit commits live. Not vacuous: the
/// composed curves are heard (the output differs from the rig without VCA automation).
///
/// Test value: red if the composed walk, the VCA mute cursors, the `own_db` read or a terms write
/// on a composed lane allocates or frees on render.
#[test]
fn a_vca_curve_and_hand_edits_over_it_render_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = warm(Rig::with(true));
    let mut plain = warm_rig();
    let row = |owner: Owner, section: Section, control: u32| ControlAddress {
        owner,
        section,
        control,
        lane: ParameterChannel::Both,
    };
    let offset = row(Owner::Vca(id("v")), Section::Own, 1);
    let mute = row(Owner::Vca(id("v")), Section::Own, 2);
    let member = row(Owner::Strip(id("eq1")), Section::Builtins, 5);
    let script: [&[(&ControlAddress, f32, Option<u32>)]; 6] = [
        &[],
        &[(&offset, 3.0, None)],
        &[(&member, -6.0, Some(256))],
        &[(&mute, 1.0, None)],
        &[(&mute, 0.0, Some(256))],
        &[(&offset, -1.5, Some(256)), (&member, 0.0, None)],
    ];
    let mut heard = false;
    for moves in script {
        for &(address, value, ramp) in moves {
            rig.move_row(address.clone(), value, ramp);
        }
        for _ in 0..12 {
            let block = rig.block;
            let used = rig.block();
            plain.block();
            heard |= rig.last != plain.last;
            assert_eq!(
                (used.allocations, used.deallocations),
                (0, 0),
                "block {block} allocated or freed"
            );
        }
    }
    assert!(heard, "the composed curves are heard");
}
