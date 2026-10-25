//! #1577: a strip's response owners, captured on the render thread after a block.
//!
//! Each test drives control-plane's own session (`SessionState::command` and `service`, the code
//! both hosts run) and its render half (`PlanState::render`) on one thread. The session is the
//! nine-track EQ fixture cut to two tracks, at 48 kHz: input filters on every strip, the EQ
//! console slot `eq`, and an EQ insert on `eq0`.
#![cfg(feature = "test-support")]

#[path = "support/adoption.rs"]
mod adoption;

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    ResponseCaptureFailure, ResponseCaptureRefusal, ResponseChannels, ResponseMemberSection,
    ResponsePreviewGrid, ResponseReady, ResponseSnapshotCollector, SuccessorBase, TrackResponse,
    compose_track_response, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    ControlAddress, Effect, EffectIdentity, EffectQuality, LinkMode, Owner, ParameterChannel,
    RackName, Section, SidechainDeclaration, StableId,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const GRID: ResponsePreviewGrid = ResponsePreviewGrid::Logarithmic {
    points: 64,
    minimum_hz: 20.0,
    maximum_hz: 20_000.0,
};

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

/// A native EQ named `name`, with `eq0`'s console values.
fn eq(model: &session::SessionModel, name: &str) -> Effect {
    Effect {
        id: id(name),
        identity: EffectIdentity::Native {
            effect_id: id("miso.parametric-eq"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: model.tracks[0].console[0].params.clone(),
        sidechain: SidechainDeclaration::None,
    }
}

/// The two-track session; `left_gain_db` sets `eq0`'s console EQ band gain on the left lane.
fn document(left_gain_db: Option<f32>) -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.tracks.truncate(2);
    let kept: Vec<_> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| {
        matches!(&route.source, session::RouteSource::Track { track_id, .. } if kept.contains(track_id))
    });
    let insert = eq(&model, "eq-insert");
    model.tracks[0].inserts.effects.push(insert);
    if let Some(gain) = left_gain_db {
        for param in &mut model.tracks[0].console[0].params {
            if param.parameter_id == 4 && param.channel == ParameterChannel::Left {
                param.value = gain;
            }
        }
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
    at: u64,
}

impl Rig {
    fn new(left_gain_db: Option<f32>) -> Self {
        let (document, revision) = document(left_gain_db);
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document, limits(), Box::new(Preparer), None)
                .expect("boot compile");
        Self {
            session,
            plan,
            revision,
            request: 1,
            at: 0,
        }
    }

    /// Render one block (the sources underrun: silence is enough); the render call's allocator
    /// events.
    fn block(&mut self) -> bench_support::alloc::Counters {
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.plan.render(self.at, output).expect("a rendered block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        self.at += QUANTUM as u64;
        used
    }

    /// The applied revision, from the plan's watermark.
    fn applied(&self) -> engine::realtime::PlanWatermark {
        self.plan.queries().watermark().expect("a watermark")
    }

    /// Renders and services block by block until the plan applies the committed revision: a
    /// rebuild that restarts a strip is a duck-swap, adopted at its `S` (#1324a).
    fn render_until_applied(&mut self) {
        let revision = self.revision;
        adoption::render_until_applied(
            self,
            revision,
            |rig| rig.applied().revision,
            |rig| {
                rig.block();
                rig.session.service().expect("service");
            },
        );
    }

    /// One `service` step; the delivered capture result, taken and released.
    fn service(&mut self) -> Option<ResponseReady> {
        self.session.service().expect("service");
        self.session.test_take_track_response()
    }

    fn arm(&mut self, strip: &str) -> Result<u64, ResponseCaptureRefusal> {
        self.session
            .test_arm_track_response(strip, GRID, ResponseChannels::Both)
    }

    /// Render and service until the armed capture is delivered, at most `blocks` blocks.
    fn deliver(&mut self, blocks: usize) -> ResponseReady {
        for _ in 0..blocks {
            self.block();
            if let Some(ready) = self.service() {
                return ready;
            }
        }
        panic!("no capture result within {blocks} blocks");
    }

    /// The test's own capture of `strip` from the plan, at the current boundary, composed.
    fn direct(&mut self, strip: &str) -> TrackResponse {
        let mut collector = ResponseSnapshotCollector::new(48_000, 16, 6, 127);
        let capture = self
            .plan
            .copy_response_snapshot(engine::realtime::ResponseSnapshotRequest {
                track_id: strip,
                sink: &mut collector,
            })
            .expect("a direct capture");
        compose_track_response(
            strip,
            &collector.finish(capture),
            GRID,
            ResponseChannels::Both,
        )
        .expect("a composed response")
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
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
        path
    }
}

/// An edit that adds an EQ insert named `name` to `eq0`, last.
fn add_insert(name: &str, position: u32) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: id("eq0"),
        rack_name: RackName::Inserts,
        final_position: position,
        effect: eq(
            &session::parse_session_json(SESSION).expect("fixture"),
            name,
        ),
    }
}

/// A muted copy of `eq1` named `name` on its source, with no route: a structural edit that
/// restarts no strip.
fn muted_track(name: &str) -> SessionEdit {
    let model = session::parse_session_json(SESSION).expect("fixture");
    let mut track = model.tracks[1].clone();
    track.id = id(name);
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    SessionEdit::UpsertTrack { track }
}

fn response(ready: ResponseReady) -> TrackResponse {
    ready.result.expect("a track response")
}

/// Gates 1, 3 and 4. Refusals first, while idle (an unknown strip, 4,097 points) and while armed
/// or delivered (`Pending`): none leaves a tag or a pending state, so the first capture is tag 1
/// and the next tag 2. The first capture is delivered after one block and equals the composition
/// of the test's own capture after that block. A capture armed before an edit that adds an insert
/// is answered by the running plan the duck-swap keeps until `S` (#1324a), without the insert; one
/// armed before an edit that restarts no strip is answered from the plan that holds the edit; one
/// armed on `eq1` before an edit that removes it is `StripRemoved`. A request keeps its tag.
///
/// Test value: red if the render step captures before the block (from the plan the block
/// displaced) or from other owners than the plan holds, if a replacement's epoch is not recorded
/// (the stale capture never settles), if a reply is dropped or delivered twice, or if a refused
/// request leaves a tag or a pending state behind.
#[test]
fn capture_follows_newest_plan() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(None);
    assert_eq!(rig.arm("nope"), Err(ResponseCaptureRefusal::UnknownStrip));
    assert_eq!(
        rig.session.test_arm_track_response(
            "eq0",
            ResponsePreviewGrid::Logarithmic {
                points: 4_097,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
            },
            ResponseChannels::Both,
        ),
        Err(ResponseCaptureRefusal::Grid)
    );
    assert_eq!(rig.arm("eq0"), Ok(1));
    assert_eq!(rig.arm("eq1"), Err(ResponseCaptureRefusal::Pending));
    assert_eq!(
        rig.arm("nope"),
        Err(ResponseCaptureRefusal::UnknownStrip),
        "an unknown strip is refused before a pending capture"
    );
    assert_eq!(rig.service(), None, "nothing is captured before a block");

    // Gate 1: one block, one capture, equal to the test's own after the same block.
    rig.block();
    rig.session.service().expect("service");
    assert_eq!(
        rig.arm("eq1"),
        Err(ResponseCaptureRefusal::Pending),
        "a delivered capture is pending until released"
    );
    let ready = rig
        .session
        .test_take_track_response()
        .expect("delivered after one block");
    assert_eq!((ready.tag, &*ready.strip_id), (1, "eq0"));
    let expected = rig.direct("eq0");
    assert_eq!(response(ready), expected);
    assert_eq!(rig.service(), None, "a result is delivered once");

    // Gate 3, the keep branch: a rebuild that adds an insert, a duck-swap adopted blocks later at
    // its `S` (#1324a); a capture armed after its commit waits for the adoption, and no block
    // allocates or frees, the swap block included.
    assert_eq!(
        rig.transact(&[add_insert("eq-added", 1)]),
        EditPath::Rebuild
    );
    assert_eq!(rig.arm("eq0"), Ok(2));
    let mut waited = 0;
    let ready = loop {
        let start = rig.at;
        let used = rig.block();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "the block at {start} allocated or freed"
        );
        let watermark = rig.applied();
        let adopted = watermark.revision >= rig.revision;
        if let Some(ready) = rig.service() {
            assert!(
                adopted && watermark.first_sample == start,
                "delivered from the adoption block, not before"
            );
            assert!(waited > 0, "the capture waits for the duck-swap's adoption");
            break ready;
        }
        assert!(!adopted, "no capture after the adoption block");
        waited += 1;
    };
    assert_eq!(ready.tag, 2);
    let members = response(ready).members;
    assert!(
        members
            .iter()
            .any(|member| member.section == ResponseMemberSection::Inserts("eq-added".into())),
        "the capture holds the added insert: {members:?}"
    );
    assert_eq!(
        members.len(),
        4,
        "input filters, the console slot, two inserts"
    );

    // Gate 3, a duck-swap: armed before an edit that adds an insert, which restarts `eq0`, so the
    // running plan renders on until `S` (#1324a) and answers it, without the insert, as it answers
    // a capture armed before a removal (#1325).
    assert_eq!(rig.arm("eq0"), Ok(3));
    assert_eq!(
        rig.transact(&[add_insert("eq-later", 2)]),
        EditPath::Rebuild
    );
    let ready = rig.deliver(1);
    assert_eq!(ready.tag, 3);
    assert_eq!(
        response(ready).members.len(),
        4,
        "two inserts: the running plan"
    );
    rig.render_until_applied();

    // Gate 3, the stale branch: armed before a structural edit that restarts no strip (a muted
    // track added), so the next block adopts its plan: answered from the plan that holds the edit
    // and the insert before it.
    assert_eq!(rig.arm("eq0"), Ok(4));
    assert_eq!(rig.transact(&[muted_track("eq-muted")]), EditPath::Rebuild);
    let ready = rig.deliver(4);
    assert_eq!(ready.tag, 4);
    assert_eq!(response(ready).members.len(), 5, "three inserts");

    // Gate 3: armed on a strip an edit then removes.
    // A removal ducks its strip out before its plan is adopted (#1325), so a capture armed on the
    // running plan is answered from it, with the strip. Armed instead on a pending rebuild (due 2
    // blocks later) that the removal then supersedes, it waits, finds the removal's plan newer,
    // and its strip gone.
    rig.session
        .test_set_next_adoption(engine::realtime::PlanAdoption::NoEarlierThan(
            rig.at + 2 * QUANTUM as u64,
        ));
    assert_eq!(rig.transact(&[add_insert("eq-last", 3)]), EditPath::Rebuild);
    assert_eq!(rig.arm("eq1"), Ok(5));
    let path = rig.transact(&[
        SessionEdit::RemoveRoute {
            route_id: id("eq1-main"),
        },
        SessionEdit::RemoveTrack {
            track_id: id("eq1"),
        },
    ]);
    assert_eq!(path, EditPath::Rebuild);
    let ready = rig.deliver(256);
    assert_eq!(
        (ready.tag, &*ready.strip_id, ready.result),
        (5, "eq1", Err(ResponseCaptureFailure::StripRemoved))
    );
}

/// Gate 2. A live edit of `eq0`'s console EQ band gain, committed and then armed, is captured
/// from the block that applies it: the curve equals the one of a session booted with that gain,
/// differs from the one before the edit, and `captured_sample` is the block's end.
///
/// Test value: red if the capture reads the values the plan held before the block (the step runs
/// before the block's live snapshot) rather than the applied targets after it.
#[test]
fn capture_reads_applied_values() {
    let mut rig = Rig::new(None);
    rig.block();
    assert_eq!(rig.arm("eq0"), Ok(1));
    let before = response(rig.deliver(1));
    let path = rig.transact(&[SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Console(id("eq")),
            control: 4,
            lane: ParameterChannel::Left,
        },
        value: -3.0,
        ramp_samples: None,
    }]);
    assert_eq!(path, EditPath::Live);
    assert_eq!(rig.arm("eq0"), Ok(2));
    let after = response(rig.deliver(1));
    assert_eq!(after.captured_sample, rig.at, "captured at the block's end");

    let mut booted = Rig::new(Some(-3.0));
    booted.block();
    assert_eq!(booted.arm("eq0"), Ok(1));
    let reference = response(booted.deliver(1));
    assert_eq!(
        after.left_db, reference.left_db,
        "the edited gain is captured"
    );
    assert_eq!(
        after.right_db, before.right_db,
        "the right lane is untouched"
    );
    assert_ne!(
        after.left_db, before.left_db,
        "the edit is heard in the curve"
    );
}

/// Gate 5 (and the root comment on #1577): the render step on the real sink. After two warm-up
/// captures, 200 render calls each allocate and free nothing, measured with the audited
/// allocator's thread-scoped counters, while the control side arms a capture every 4 blocks and
/// polls between blocks; every capture is delivered, so the step's copy ran on each.
///
/// Test value: red if the render step, the collector's copy or the job's move allocates or frees
/// on render (a dropped job frees its collector), or if a job is lost.
#[test]
fn render_step_allocates_nothing() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(None);
    for _ in 0..2 {
        rig.arm("eq0").expect("arm");
        rig.deliver(1).result.expect("a warm-up capture");
    }
    let mut delivered = 0;
    for block in 0..200_u32 {
        if block % 4 == 0 {
            rig.arm(if block % 8 == 0 { "eq0" } else { "eq1" })
                .expect("idle at every fourth block");
        }
        if block == 100 {
            // A structural push between arm and render: the job turns stale and is armed again.
            assert_eq!(rig.transact(&[add_insert("eq-late", 1)]), EditPath::Rebuild);
        }
        let used = rig.block();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "block {block} allocated or freed"
        );
        if let Some(ready) = rig.service() {
            ready.result.expect("a track response");
            delivered += 1;
        }
    }
    assert_eq!(delivered, 50, "every armed capture is delivered");
}

/// The largest accepted grid, both sides.
const LARGEST: ResponsePreviewGrid = ResponsePreviewGrid::Logarithmic {
    points: host_core::RESPONSE_MAXIMUM_POINTS,
    minimum_hz: 20.0,
    maximum_hz: 20_000.0,
};

fn live(used: bench_support::alloc::Counters) -> u64 {
    used.requested_bytes - used.released_bytes
}

/// The charge (root rulings, attempts 2 and 3): a capture at the session's bounds stays within
/// it, the job within `job_bytes` and the result within `result_bytes`. `eq0` has the most owners
/// (its input filters, the console slot and the one insert), and the grid is the largest accepted
/// one, both sides. The job is the bytes `arm` leaves live; the result the bytes left live after
/// the `service` that delivers it, the job freed.
///
/// Test value: red if the job's charge leaves out a part of the job (the collector's table or its
/// three identities per owner), or the result's leaves out a part of the result that persists
/// across calls (the curves, the member table, the member identities or a strip ID).
#[test]
fn a_capture_at_the_bound_stays_within_its_charge() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(None);
    rig.block();
    rig.session.service().expect("service");
    let model = session::parse_session_json(&document(None).0).expect("model");
    let charge = host_core::response_capture_owners(&model)
        .and_then(host_core::response_capture_ceiling)
        .expect("the charge");
    let mark = bench_support::alloc::current_thread_counters();
    rig.session
        .test_arm_track_response("eq0", LARGEST, ResponseChannels::Both)
        .expect("arm");
    let armed = live(bench_support::alloc::current_thread_delta_since(mark));
    rig.block();
    rig.session.service().expect("service");
    let delivered = live(bench_support::alloc::current_thread_delta_since(mark));
    assert!(
        armed <= charge.job_bytes && delivered <= charge.result_bytes,
        "armed {armed} of {} and delivered {delivered} of {} bytes",
        charge.job_bytes,
        charge.result_bytes
    );
    assert_eq!(
        rig.session
            .test_take_track_response()
            .expect("delivered")
            .result
            .expect("a track response")
            .frequencies_hz
            .len(),
        host_core::RESPONSE_MAXIMUM_POINTS
    );
}

/// The charge's high-water (root ruling, attempt 3): `eq0` grows to three inserts (five owners),
/// a capture of it is armed, and a rebuild then removes every insert, adopted four blocks later.
/// The job and then the result, both sized under the larger model and held after the smaller one
/// commits, stay within the charge the session gives its successor.
///
/// Test value: red if the charge follows the newest committed model alone, not the most owners
/// any committed model of the session has had.
#[test]
fn a_capture_held_across_a_shrinking_rebuild_stays_within_its_charge() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(None);
    for (name, position) in [("eq-second", 1), ("eq-third", 2)] {
        assert_eq!(
            rig.transact(&[add_insert(name, position)]),
            EditPath::Rebuild
        );
        rig.render_until_applied();
    }
    let mark = bench_support::alloc::current_thread_counters();
    rig.session
        .test_arm_track_response("eq0", LARGEST, ResponseChannels::Both)
        .expect("arm");
    let armed = live(bench_support::alloc::current_thread_delta_since(mark));
    rig.session
        .test_set_next_adoption(engine::realtime::PlanAdoption::NoEarlierThan(
            rig.at + 4 * QUANTUM as u64,
        ));
    let path = rig.transact(&[SessionEdit::SetTrackRack {
        track_id: id("eq0"),
        rack_name: RackName::Inserts,
        rack: session::Rack {
            effects: Vec::new(),
        },
    }]);
    assert_eq!(path, EditPath::Rebuild);
    // The row the pending successor was charged: at five owners, not the two of its own model.
    let charge = host_core::response_capture_ceiling(5).expect("the charge");
    assert_eq!(
        rig.session.test_response_capture_charge(),
        charge.total_bytes
    );
    assert!(
        armed <= charge.job_bytes,
        "the held job: {armed} of {} bytes",
        charge.job_bytes
    );
    let mark = bench_support::alloc::current_thread_counters();
    rig.block();
    rig.session.service().expect("service");
    let delivered = armed + live(bench_support::alloc::current_thread_delta_since(mark));
    assert!(
        delivered <= charge.result_bytes,
        "the held result: {delivered} of {} bytes",
        charge.result_bytes
    );
    let ready = rig.session.test_take_track_response().expect("delivered");
    assert_eq!(
        ready.result.expect("a track response").members.len(),
        5,
        "captured from the plan with three inserts"
    );
}
