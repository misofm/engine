//! Stored VCA automation in host-core preparation and the classifier (#1655 V3): every fader event
//! lane (an own fader curve, or an automated VCA `offset_db` lane that reaches it) has an offsets
//! cell; a hand fader move on an event lane with no own fader automation writes that lane's
//! `own_db` through the cell (D3), never a `FaderDb`; a hand move of an automated VCA `offset_db`
//! writes the entry's offset and rewrites `S` of every lane the VCA reaches (D5); a hand mute of an
//! automated VCA writes the reached lanes' `terms` (a `Mute` record, D4); a reach change of an
//! automated VCA is a rebuild, of a VCA no entry drives a live write (D5, root's ruling).
//!
//! Every render here prepares on a chosen backend (Simd8 on an AVX2 target, Simd4, the scalar
//! oracle), a `test-support` entry of host-core; the classifier is `control-provider`.
#![cfg(feature = "test-support")]

use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission};
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, ControlSmoothing,
    DualMonoFader, Owner, ParameterChannel, ParameterUnit, Section, SessionModel, StableId, Vca,
    canonical_session_json, parse_session_json,
};

const NINE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const NINE_SOURCE: &[u8] = b"fixture-source";
const RATE: u32 = 48_000;
/// The strip fader row (builtin parameter 5).
const FADER_DB: u32 = 5;
/// The VCA rows: `offset_db` (1) and `mute` (2).
const VCA_OFFSET: u32 = 1;
const VCA_MUTE: u32 = 2;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn caps(model: &SessionModel) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 32 * model.quantum_frames,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn compiled(model: &SessionModel) -> host_core::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    host_core::compile_host_session(&document, &caps(model))
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())))
}

fn normalized(model: &SessionModel) -> SessionModel {
    compiled(model).normalized_model().clone()
}

/// The first `tracks` tracks of the nine-track fixture at quantum `quantum`, as a normalized model:
/// no console, no insert, no input filter, an identity matrix, each routed to the output, faders
/// at 0 dB unmuted, `control_smoothing` fader 5 ms (240 samples), mute 10 ms (480). On a DC source
/// of `1.0` a track's output is its fader gain.
fn plain(tracks: usize, quantum: u32) -> SessionModel {
    let mut model = parse_session_json(NINE).expect("fixture parses");
    model.automation.clear();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.truncate(tracks);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| match &route.source {
        session::RouteSource::Track { track_id, .. } => kept.contains(track_id),
        _ => false,
    });
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
        track.fader = DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        };
        track.matrix_or_pan = session::MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples: 0,
        };
    }
    model.sources[0].frames = 1 << 20;
    model.quantum_frames = quantum;
    model.control_smoothing = Some(ControlSmoothing {
        mute_ms: 10.0,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    normalized(&model)
}

/// VCA `name` at 0 dB, unmuted, holding `members`.
fn vca(name: &str, members: Vec<StableId>) -> Vca {
    Vca {
        id: id(name),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members,
    }
}

/// A flat entry `entry` on `owner`'s row `control` (section `section`), on `lane`, at `value` in
/// `unit` (a step segment over the first 4096 samples, then held).
fn flat(
    entry: &str,
    owner: Owner,
    section: Section,
    control: u32,
    lane: ParameterChannel,
    value: f32,
    unit: ParameterUnit,
) -> Automation {
    Automation {
        id: id(entry),
        target: ControlAddress {
            owner,
            section,
            control,
            lane,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 4096,
            start_value: value,
            end_value: value,
            unit,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// A flat VCA `offset_db` ride of VCA `name` on `lane` at `db`.
fn vca_ride(name: &str, lane: ParameterChannel, db: f32) -> Automation {
    flat(
        &format!("{name}-ride"),
        Owner::Vca(id(name)),
        Section::Own,
        VCA_OFFSET,
        lane,
        db,
        ParameterUnit::Db,
    )
}

/// A flat VCA `mute` entry of VCA `name` on both lanes, unmuted.
fn vca_mute_entry(name: &str) -> Automation {
    flat(
        &format!("{name}-mute"),
        Owner::Vca(id(name)),
        Section::Own,
        VCA_MUTE,
        ParameterChannel::Both,
        0.0,
        ParameterUnit::Linear,
    )
}

/// A flat own fader ride of track `track` on both lanes at `db`.
fn own_ride(model: &SessionModel, track: usize, db: f32) -> Automation {
    flat(
        &format!("fader-{track}"),
        Owner::Strip(model.tracks[track].id.clone()),
        Section::Builtins,
        FADER_DB,
        ParameterChannel::Both,
        db,
        ParameterUnit::Db,
    )
}

/// Every backend of the target.
fn backends() -> Vec<graph_compiler::Backend> {
    vec![
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
        graph_compiler::Backend::Scalar,
    ]
}

/// A plan of `model` with live controls on `backend` (`between`: delivery between render calls),
/// rendered block by block on a DC source of `1.0`.
struct Rig {
    prepared: PreparedHost,
    handles: host_core::HostLiveControlHandles,
    gate: engine::realtime::PlanRevisionGate,
    quantum: usize,
    rendered: u64,
    out: [Vec<f32>; 2],
    /// The render calls' allocations and frees.
    allocations: (u64, u64),
}

impl Rig {
    fn new(model: &SessionModel, backend: graph_compiler::Backend, between: bool) -> Self {
        let compiled = compiled(model);
        let caps = caps(model);
        let request = host_core::HostLiveControlRequest::default();
        let (mut prepared, handles) = if between {
            host_core::prepare::test_only_prepare_host_runtime_between_render_calls_on(
                &compiled, &caps, &request, backend,
            )
        } else {
            host_core::test_only_prepare_host_runtime_with_live_controls_on(
                &compiled, &caps, &request, backend,
            )
        }
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let gate = prepared.plan.take_revision_gate().expect("the plan's gate");
        Self {
            prepared,
            handles,
            gate,
            quantum: model.quantum_frames as usize,
            rendered: 0,
            out: [Vec::new(), Vec::new()],
            allocations: (0, 0),
        }
    }

    /// The position of strip `strip` among the handles' producers.
    fn strip(&self, strip: &str) -> usize {
        self.handles
            .strips
            .iter()
            .position(|held| held.as_ref() == strip)
            .expect("a strip of the plan")
    }

    /// Whether strip `strip`'s fader channels have an offsets cell, `[left, right]`.
    fn has_offsets(&self, strip: &str) -> [bool; 2] {
        let fader = &self.handles.strip_controls[self.strip(strip)].fader;
        [fader.has_offsets(0), fader.has_offsets(1)]
    }

    /// Render one block.
    fn block(&mut self) {
        let quantum = self.quantum;
        let plane = vec![1.0_f32; quantum];
        self.prepared
            .sources
            .submit(
                NINE_SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: self.rendered,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: self.rendered,
                },
            )
            .expect("render");
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        self.allocations.0 += delta.allocations + delta.reallocations;
        self.allocations.1 += delta.deallocations;
        self.out[0].extend_from_slice(&samples[..quantum]);
        self.out[1].extend_from_slice(&samples[quantum..]);
        self.rendered += quantum as u64;
    }

    fn blocks(&mut self, count: usize) {
        for _ in 0..count {
            self.block();
        }
    }
}

/// #1655 D3 and the batch 19 amendment's Slot: every fader event lane has an offsets cell, also
/// when its own fader is not automated, and only event lanes do. Track 0 and track 1 are members
/// of `vca`, whose left `offset_db` lane rides; track 1 also rides its own fader on both lanes;
/// track 2 is in no VCA.
///
/// Test value: red if an event lane that only a VCA ride reaches has no offsets cell (its `own_db`
/// and `S` writes would be refused at the commit's pre-write check), or if a lane outside the
/// ride (the right lane of track 0, track 2) gets one.
#[test]
fn every_fader_event_lane_has_an_offsets_cell() {
    let mut model = plain(3, 128);
    let (t0, t1, t2) = (
        model.tracks[0].id.clone(),
        model.tracks[1].id.clone(),
        model.tracks[2].id.clone(),
    );
    model.vcas = vec![vca("vca", vec![t0.clone(), t1.clone()])];
    model
        .automation
        .push(vca_ride("vca", ParameterChannel::Left, -6.0));
    let ride = own_ride(&model, 1, -3.0);
    model.automation.push(ride);
    let model = normalized(&model);
    for backend in backends() {
        for between in [false, true] {
            let rig = Rig::new(&model, backend, between);
            let what = format!("{backend:?} between {between}");
            assert_eq!(rig.has_offsets(t0.as_str()), [true, false], "{what}");
            assert_eq!(rig.has_offsets(t1.as_str()), [true, true], "{what}");
            assert_eq!(rig.has_offsets(t2.as_str()), [false, false], "{what}");
        }
    }
}

#[cfg(feature = "control-provider")]
mod classifier {
    use super::*;
    use host_core::{
        BuiltinLaneSelector, FaderOffsetsRecord, LiveDelta, LiveRamps, LiveRebuild,
        TrackFaderRecord, classify_live_delta,
    };
    use session::{EditRampRow, EditRampTarget, EditRamps};

    /// An edit of a VCA's reach, given the tracks' IDs.
    type ReachEdit = fn(&mut SessionModel, &[StableId]);

    fn edited(current: &SessionModel, edit: impl FnOnce(&mut SessionModel)) -> SessionModel {
        let mut next = current.clone();
        next.revision += 1;
        edit(&mut next);
        next
    }

    /// One strip's records, owned: its ID, its fader/mute records and its offsets writes.
    type Level = (
        String,
        Vec<TrackFaderRecord>,
        [Option<FaderOffsetsRecord>; 2],
    );

    fn level(delta: &LiveDelta<'_>) -> Vec<Level> {
        delta
            .strips
            .iter()
            .map(|strip| {
                (
                    strip.strip_id.to_owned(),
                    strip.fader_records().collect(),
                    strip.offsets,
                )
            })
            .collect()
    }

    fn classify(
        current: &SessionModel,
        next: &SessionModel,
        edit_ramps: &EditRamps,
    ) -> Result<Vec<Level>, LiveRebuild> {
        classify_live_delta(current, next, LiveRamps::for_session(next), edit_ramps)
            .map(|delta| level(&delta))
    }

    /// [`classify`] of a transaction of one `SetControl` on `row`, which the fast path
    /// (`classify_live_edit` with the row) judges with the same records.
    fn classify_set(
        current: &SessionModel,
        next: &SessionModel,
        edit_ramps: &EditRamps,
        row: &ControlAddress,
    ) -> Result<Vec<Level>, LiveRebuild> {
        let full = classify(current, next, edit_ramps);
        let _ = host_core::take_fast_classification_count();
        let fast = host_core::classify_live_edit(
            current,
            next,
            LiveRamps::for_session(next),
            edit_ramps,
            host_core::MonitorBase::NONE,
            Some(core::slice::from_ref(row)),
        )
        .map(|delta| level(&delta));
        assert_eq!(
            host_core::take_fast_classification_count(),
            1,
            "the fast path"
        );
        assert_eq!(fast, full, "the fast path's records");
        full
    }

    fn offsets(
        offsets_sum: f64,
        smoothing_samples: u32,
        own_db: Option<f32>,
    ) -> FaderOffsetsRecord {
        FaderOffsetsRecord {
            offsets_sum,
            smoothing_samples,
            own_db,
        }
    }

    /// `S` of strip `strip` (strips order) on `lane` as `controls::effective` composes it.
    fn s_of(model: &SessionModel, strip: usize, lane: usize) -> f64 {
        controls::effective(model, &controls::MonitorOverlay::default()).strips[strip].offsets_sum
            [lane]
    }

    fn address(owner: Owner, section: Section, control: u32) -> ControlAddress {
        ControlAddress {
            owner,
            section,
            control,
            lane: ParameterChannel::Both,
        }
    }

    /// Two tracks in `vca`, whose `offset_db` rides at -6 dB on both lanes and whose `mute` has
    /// an entry; track 1 also rides its own fader at -3 dB.
    fn session() -> SessionModel {
        let mut model = plain(2, 128);
        let members = model.tracks.iter().map(|track| track.id.clone()).collect();
        model.vcas = vec![vca("vca", members)];
        model
            .automation
            .push(vca_ride("vca", ParameterChannel::Both, -6.0));
        model.automation.push(vca_mute_entry("vca"));
        let ride = own_ride(&model, 1, -3.0);
        model.automation.push(ride);
        normalized(&model)
    }

    /// Write `delta`'s strip records through `rig`'s live cells, as the shared commit does.
    fn write(rig: &mut Rig, delta: &[Level]) {
        use builtins_compiler::TestOnlyWriteGate;
        let positions: Vec<usize> = delta.iter().map(|(strip, ..)| rig.strip(strip)).collect();
        let handles = &mut rig.handles;
        rig.gate.write_published(|stamp| {
            for ((_, fader, cells), &position) in delta.iter().zip(&positions) {
                let producer = &mut handles.strip_controls[position];
                for &record in fader {
                    producer.fader.write(stamp, record);
                }
                for (channel, record) in cells.iter().enumerate() {
                    if let Some(record) = *record {
                        assert!(producer.fader.has_offsets(channel), "a cell to write");
                        producer.fader.write_offsets(stamp, channel, record);
                    }
                }
            }
        });
    }

    /// Render `current` on `backend` for 8 blocks, write the classifier's records of
    /// `current -> next` and render 24 more; a fresh plan of `next` renders 32 blocks. The last 8
    /// blocks of each (every ramp long done) are equal, bit for bit, they differ from the block
    /// before the write (the edit is heard), and the render thread allocated and freed nothing.
    fn assert_heard_as_fresh(current: &SessionModel, next: &SessionModel, records: &[Level]) {
        for backend in backends() {
            for between in [false, true] {
                let what = format!("{backend:?} between {between}");
                let mut live = Rig::new(current, backend, between);
                live.blocks(8);
                write(&mut live, records);
                live.blocks(24);
                let mut fresh = Rig::new(next, backend, between);
                fresh.blocks(32);
                let tail = 24 * live.quantum..32 * live.quantum;
                for lane in 0..2 {
                    let (heard, expected) = (
                        &live.out[lane][tail.clone()],
                        &fresh.out[lane][tail.clone()],
                    );
                    assert!(
                        heard
                            .iter()
                            .zip(expected)
                            .all(|(a, b)| a.to_bits() == b.to_bits()),
                        "{what}: lane {lane} heard {:?}, a fresh plan {:?}",
                        heard[0],
                        expected[0]
                    );
                }
                assert_ne!(
                    live.out[0][8 * live.quantum - 1].to_bits(),
                    live.out[0][tail.start].to_bits(),
                    "{what}: the edit is heard"
                );
                assert_eq!(live.allocations, (0, 0), "{what}: the render thread");
            }
        }
    }

    /// Gate 4 (a hand member move, D3): a hand fader move on track 0, which only the VCA ride
    /// reaches, is live with one offsets write per lane carrying `own_db` and the unchanged `S`,
    /// with the `Fader` row's length (the edit's own 256 samples), and no `FaderDb`; track 1
    /// (whose own fader rides) is untouched. After the jump the plan sounds as a fresh plan of the
    /// edited session, on every backend.
    ///
    /// Test value: red if the move lands as a `FaderDb` (which the composed lane's stage would
    /// apply over the VCA curve), if the record leaves `own_db` out (the move is acked and never
    /// heard), or if it takes the VCA row's length instead of the fader's.
    #[test]
    fn a_hand_move_on_a_member_only_a_vca_ride_reaches_writes_its_own_db() {
        let current = session();
        let t0 = current.tracks[0].id.clone();
        let mut ramp = EditRamps::new();
        ramp.set(
            &EditRampTarget::Strip(t0.clone()),
            EditRampRow::Fader,
            Some(256),
        );
        let row = address(Owner::Strip(t0.clone()), Section::Builtins, FADER_DB);
        let next = edited(&current, |next| {
            controls::set(next, &row, -10.0, Some(256), RATE).expect("a hand move");
        });
        let s = s_of(&next, 0, 0);
        assert_eq!(
            s.to_bits(),
            s_of(&current, 0, 0).to_bits(),
            "S does not move"
        );
        let records = classify_set(&current, &next, &ramp, &row).expect("live");
        assert_eq!(
            records,
            vec![(
                t0.as_str().to_owned(),
                Vec::new(),
                [Some(offsets(s, 256, Some(-10.0))); 2]
            )]
        );
        // Without an edit length, the session's fader length.
        let session_length = classify(&current, &next, &EditRamps::new()).expect("live");
        assert_eq!(session_length[0].2, [Some(offsets(s, 240, Some(-10.0))); 2]);
        assert_heard_as_fresh(&current, &next, &records);
    }

    /// Gate 4 (a hand VCA move, D5): `SET_CONTROL offset_db = +3` on the automated VCA writes the
    /// ride's offset and is live: each member lane gets one offsets write of the new `S` (the
    /// VCA's hand offset) with the VCA row's edit length; track 0's carries its static `own_db`,
    /// track 1's (own ride) none; no `FaderDb`. After the jump it sounds as a fresh plan.
    ///
    /// Test value: red if the hand move does not reach a member's `S` (acked, never heard), if a
    /// member takes a `FaderDb`, or if the records take the session length instead of the VCA
    /// edit's (the hand offset is the VCA's own edit).
    #[test]
    fn a_hand_move_on_an_automated_vca_offset_rewrites_every_reached_lanes_s() {
        let current = session();
        let (t0, t1) = (current.tracks[0].id.clone(), current.tracks[1].id.clone());
        let mut ramp = EditRamps::new();
        ramp.set(
            &EditRampTarget::Vca(id("vca")),
            EditRampRow::VcaFader,
            Some(128),
        );
        let row = address(Owner::Vca(id("vca")), Section::Own, VCA_OFFSET);
        let next = edited(&current, |next| {
            controls::set(next, &row, 3.0, Some(128), RATE).expect("a hand move");
        });
        assert_eq!(next.vcas, current.vcas, "the static offset is the fallback");
        assert_eq!(
            next.automation
                .iter()
                .find(|entry| entry.id == id("vca-ride"))
                .expect("the ride")
                .offset
                .left_db
                .to_bits(),
            3.0_f32.to_bits()
        );
        let s = s_of(&next, 0, 0);
        assert_eq!(s.to_bits(), f64::from(3.0_f32).to_bits());
        let records = classify_set(&current, &next, &ramp, &row).expect("live");
        assert_eq!(
            records,
            vec![
                (
                    t0.as_str().to_owned(),
                    Vec::new(),
                    [Some(offsets(s, 128, Some(0.0))); 2]
                ),
                (
                    t1.as_str().to_owned(),
                    Vec::new(),
                    [Some(offsets(s_of(&next, 1, 0), 128, None)); 2]
                ),
            ]
        );
        assert_heard_as_fresh(&current, &next, &records);
    }

    /// Gate 4 (a hand VCA mute, D5): a hand mute of the automated VCA is live and writes each
    /// reached lane's `terms` (one `Mute` record per member, the session mute length); the curve
    /// is open, so the members close, as a fresh plan with the static mute does.
    ///
    /// Test value: red if a hand mute of an automated VCA is refused or reaches no member, or if
    /// the composed mute does not OR it with the open curve.
    #[test]
    fn a_hand_mute_of_an_automated_vca_writes_each_reached_lanes_terms() {
        let current = session();
        let row = address(Owner::Vca(id("vca")), Section::Own, VCA_MUTE);
        let next = edited(&current, |next| {
            controls::set(next, &row, 1.0, None, RATE).expect("a hand mute");
        });
        let records = classify_set(&current, &next, &EditRamps::new(), &row).expect("live");
        let muted = TrackFaderRecord::Mute {
            lanes: BuiltinLaneSelector::Both,
            muted: true,
            smoothing_samples: 480,
        };
        assert_eq!(
            records,
            current
                .tracks
                .iter()
                .map(|track| (track.id.as_str().to_owned(), vec![muted], [None; 2]))
                .collect::<Vec<_>>()
        );
        assert_heard_as_fresh(&current, &next, &records);
    }

    /// D5's last bullet (root's ruling): a membership edit of the automated VCA (a member removed,
    /// a member added, a nesting edit) is a rebuild (`Automation`); the same edits of a VCA no
    /// entry drives stay live, as #1649 made them.
    ///
    /// Test value: red if an automated VCA's reach change commits live, with no event state for a
    /// new pair (or a stale one for a lost pair).
    #[test]
    fn a_reach_change_of_an_automated_vca_rebuilds_and_of_a_plain_vca_is_live() {
        let mut base = plain(3, 128);
        let t: Vec<StableId> = base.tracks.iter().map(|track| track.id.clone()).collect();
        base.vcas = vec![vca("vca", vec![t[0].clone()]), vca("vcb", Vec::new())];
        let plain_vca = normalized(&base);
        base.automation
            .push(vca_ride("vca", ParameterChannel::Both, -6.0));
        let automated = normalized(&base);
        let edits: [(&str, ReachEdit); 3] = [
            ("a member added", |next, t| {
                next.vcas[0].members.push(t[1].clone())
            }),
            ("a member removed", |next, _| next.vcas[0].members.clear()),
            ("a nesting edit", |next, t| {
                next.vcas[1].members.push(t[2].clone());
                next.vcas[0].members.push(id("vcb"));
            }),
        ];
        for (what, edit) in edits {
            let next = edited(&automated, |next| edit(next, &t));
            assert_eq!(
                classify(&automated, &next, &EditRamps::new()),
                Err(LiveRebuild::Automation),
                "automated VCA: {what}"
            );
            let next = edited(&plain_vca, |next| edit(next, &t));
            assert!(
                classify(&plain_vca, &next, &EditRamps::new()).is_ok(),
                "plain VCA: {what}"
            );
        }
        // An edit of a VCA no entry drives, beside the automated one, stays live: a static move
        // of `vcb` reaching track 2 alone is a `FaderDb` there.
        let mut beside = automated.clone();
        beside.vcas[1].members.push(t[2].clone());
        let beside = normalized(&beside);
        let next = edited(&beside, |next| next.vcas[1].fader.left_db = -6.0);
        assert_eq!(
            classify(&beside, &next, &EditRamps::new()),
            Ok(vec![(
                t[2].as_str().to_owned(),
                vec![TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Left,
                    db: -6.0,
                    smoothing_samples: 240,
                }],
                [None; 2]
            )])
        );
    }

    /// D3 at the clamp: track 0 sits at +20 dB under a static VCA at +10 dB (the effective fader
    /// clamps to +24) and an automated VCA ride at -6 dB. A hand move to +16 dB leaves the static
    /// effective fader (`16 + 10` clamps to +24 too) and `S` as they were, but the composed lane
    /// hears it (`16 + 10 - 6 = 20` dB, not 24), so the move still writes `own_db`.
    ///
    /// Test value: red if the classifier decides "level moved" from the clamped effective fader
    /// alone, so an acked hand move on a composed lane is never written.
    #[test]
    fn a_hand_move_hidden_by_the_static_clamp_still_writes_own_db() {
        let mut model = plain(1, 128);
        let t0 = model.tracks[0].id.clone();
        model.tracks[0].fader.left_db = 20.0;
        model.tracks[0].fader.right_db = 20.0;
        let mut boost = vca("vca-a", vec![t0.clone()]);
        boost.fader.left_db = 10.0;
        boost.fader.right_db = 10.0;
        model.vcas = vec![boost, vca("vca-b", vec![t0.clone()])];
        model
            .automation
            .push(vca_ride("vca-b", ParameterChannel::Both, -6.0));
        let current = normalized(&model);
        let row = address(Owner::Strip(t0.clone()), Section::Builtins, FADER_DB);
        let next = edited(&current, |next| {
            controls::set(next, &row, 16.0, None, RATE).expect("a hand move");
        });
        let effective = |model: &SessionModel| {
            controls::effective(model, &controls::MonitorOverlay::default()).strips[0]
        };
        assert_eq!(
            effective(&current).fader_db,
            effective(&next).fader_db,
            "clamped"
        );
        let s = s_of(&next, 0, 0);
        let records = classify_set(&current, &next, &EditRamps::new(), &row).expect("live");
        assert_eq!(
            records,
            vec![(
                t0.as_str().to_owned(),
                Vec::new(),
                [Some(offsets(s, 240, Some(16.0))); 2]
            )]
        );
        assert_heard_as_fresh(&current, &next, &records);
    }
}
