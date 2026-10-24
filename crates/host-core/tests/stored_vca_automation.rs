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

/// A plan of `model` with live controls on `backend`, rendered block by block on a DC source of `1.0`.
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
    fn new(model: &SessionModel, backend: graph_compiler::Backend) -> Self {
        let compiled = compiled(model);
        let caps = caps(model);
        let request = host_core::HostLiveControlRequest::default();
        let (mut prepared, handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_on(
                &compiled, &caps, &request, backend,
            )
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
        let rig = Rig::new(&model, backend);
        let what = format!("{backend:?}");
        assert_eq!(rig.has_offsets(t0.as_str()), [true, false], "{what}");
        assert_eq!(rig.has_offsets(t1.as_str()), [true, true], "{what}");
        assert_eq!(rig.has_offsets(t2.as_str()), [false, false], "{what}");
    }
}

// ---------------------------------------------------------------------------------------------
// #1655 V5: the objective gates on every bank width and the scalar oracle
// ---------------------------------------------------------------------------------------------

/// The session mute length here (`mute_ms` 10 at 48 kHz): the `VcaMute` row's length.
const M: u32 = 480;
/// The session fader length here (`fader_ms` 5 at 48 kHz): the `VcaOffset` row's length.
const J: u32 = 240;

/// One segment of `shape` over `[start, end)` from `from` to `to` in `unit`.
fn piece(
    shape: AutomationShape,
    (start, end): (u64, u64),
    (from, to): (f32, f32),
    unit: ParameterUnit,
) -> AutomationSegment {
    AutomationSegment {
        shape,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit,
    }
}

/// Steps from `from` at sample 0 to each `(sample, value)` of `steps` in `unit`, held after.
fn steps_of(from: f32, steps: &[(u64, f32)], unit: ParameterUnit) -> Vec<AutomationSegment> {
    let mut at = (0_u64, from);
    steps
        .iter()
        .map(|&(sample, value)| {
            let step = piece(AutomationShape::Step, (at.0, sample), (at.1, value), unit);
            at = (sample, value);
            step
        })
        .collect()
}

/// An entry `name` on `owner`'s row `control` in `section` on both lanes, with `segments`.
fn entry_of(
    name: &str,
    owner: Owner,
    section: Section,
    control: u32,
    segments: Vec<AutomationSegment>,
) -> Automation {
    Automation {
        id: id(name),
        target: ControlAddress {
            owner,
            section,
            control,
            lane: ParameterChannel::Both,
        },
        segments,
        offset: session::AutomationOffset::default(),
    }
}

/// VCA `name`'s `mute` entry stepping from `from` at each `(sample, value)` of `steps`.
fn vca_mute_steps(name: &str, from: f32, steps: &[(u64, f32)]) -> Automation {
    entry_of(
        &format!("{name}-mute"),
        Owner::Vca(id(name)),
        Section::Own,
        VCA_MUTE,
        steps_of(from, steps, ParameterUnit::Linear),
    )
}

/// `model` with a true-peak limiter (5 ms lookahead) on track `track`: a latent node before its
/// fader.
fn with_limiter(mut model: SessionModel, track: usize) -> SessionModel {
    use session::{
        Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, SidechainDeclaration,
    };
    let param = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    model.tracks[track].inserts.effects.push(Effect {
        id: id(&format!("limiter-{track}")),
        identity: EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: vec![
            param(1, ParameterUnit::Db, -8.0),
            param(2, ParameterUnit::Milliseconds, 60.0),
            param(3, ParameterUnit::Milliseconds, 5.0),
        ],
        sidechain: SidechainDeclaration::None,
    });
    normalized(&model)
}

/// The floored input arrival `a(n)` of track `track`'s fader node in `prepared`'s plan.
fn fader_arrival(prepared: &PreparedHost, model: &SessionModel, track: usize) -> u64 {
    let track_id = model.tracks[track].id.as_str();
    prepared
        .inventory
        .input_floors()
        .iter()
        .find(|(node, _)| {
            matches!(
                node,
                graph::GraphNodeId::TrackStage {
                    track_id: id,
                    stage: graph::TrackStage::PostFader,
                } if id.as_str() == track_id
            )
        })
        .map_or(0, |(_, arrival)| *arrival)
}

/// The probe's fader and mute events of `blocks` more blocks of `rig`, taken block by block.
fn probed(rig: &mut Rig, blocks: usize) -> Vec<builtins_compiler::TestOnlyFaderEvent> {
    let _ = builtins_compiler::test_only_take_fader_events();
    let mut all = Vec::new();
    for _ in 0..blocks {
        rig.block();
        let (events, overflow) = builtins_compiler::test_only_take_fader_events();
        assert!(!overflow, "the probe holds a block's events");
        all.extend(events);
    }
    all
}

fn same_bits(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a.to_bits() == b.to_bits())
}

/// Gate 1 (#1655, amended): a flat VCA `offset_db` ride at -6 dB (its static fallback +3 dB
/// unused) and a flat VCA `mute` at 0 on `va` render, bit for bit, the session whose `va` holds
/// -6 dB statically: `t0` is in `va` and in the static `vb` (-2 dB; two VCAs on one member, one
/// automated) and has no ride of its own, `t1` rides its own fader (flat -3 dB), `t2` is in no
/// VCA. On every backend, live, at quanta 128, 100 and 37, render
/// allocation-free. Not vacuous: a ride at -5 dB is heard.
///
/// Test value: red if the composed lane adds the VCA curve to `S` with other arithmetic or in
/// another order than the static composition (D2's class B order: static `vb` first, then the
/// curve), if the fallback leaks into `S`, or if a flat mute curve moves a bit.
#[test]
fn a_flat_vca_ride_and_mute_render_the_static_vca_bits() {
    for quantum in [128, 100, 37] {
        let session = |ride: Option<f32>| {
            let mut model = plain(3, quantum);
            let t: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
            let mut va = vca("va", vec![t[0].clone(), t[1].clone()]);
            let mut vb = vca("vb", vec![t[0].clone()]);
            (vb.fader.left_db, vb.fader.right_db) = (-2.0, -2.0);
            if let Some(db) = ride {
                (va.fader.left_db, va.fader.right_db) = (3.0, 3.0);
                model.automation.extend([
                    vca_ride("va", ParameterChannel::Both, db),
                    vca_mute_entry("va"),
                ]);
            } else {
                (va.fader.left_db, va.fader.right_db) = (-6.0, -6.0);
            }
            model.vcas = vec![va, vb];
            let own = own_ride(&model, 1, -3.0);
            model.automation.push(own);
            normalized(&model)
        };
        let (flat, fixed, other) = (session(Some(-6.0)), session(None), session(Some(-5.0)));
        for backend in backends() {
            let what = format!("{backend:?}, quantum {quantum}");
            let run = |model: &SessionModel| {
                let mut rig = Rig::new(model, backend);
                rig.blocks(40);
                assert_eq!(rig.allocations, (0, 0), "{what}: the render thread");
                rig.out
            };
            let (flat, fixed, other) = (run(&flat), run(&fixed), run(&other));
            for lane in 0..2 {
                assert!(
                    same_bits(&flat[lane], &fixed[lane]),
                    "{what}: lane {lane} renders the static VCA's bits"
                );
                assert!(
                    !same_bits(&other[lane], &fixed[lane]),
                    "{what}: lane {lane}, a ride at -5 dB is heard"
                );
            }
        }
    }
}

/// Gate 2 (#1655): a linear VCA `offset_db` ride (-12 to 0 dB over 8,192 samples) reaches two
/// members, `t0` behind a true-peak limiter (latency `L`, its own fader static at -1 dB) and `t1`
/// (its own fader riding -6.5 to -2 dB over 4,096). Every grid event the driver gives either
/// member's stage carries the fader dB `vca_compose_db(own, S + c(τ))` computed here, `τ` the
/// event's completion sample in that member's node time (render sample less `a(n)`, plus 63), so
/// its gain is `checked_fader_gain` of it; the latent member's events fall on the render samples
/// `L` after the other's. On every backend, live.
///
/// Test value: red if VCA events run on the render clock or another member's arrival (the latent
/// member's grid would fall on the plain member's samples and carry another time's value), or if
/// the composition differs from `vca_compose_db(own, S + c)`.
#[test]
fn a_vca_ride_reaches_each_member_in_its_own_node_time() {
    let mut model = plain(2, 128);
    let t: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    (
        model.tracks[0].fader.left_db,
        model.tracks[0].fader.right_db,
    ) = (-1.0, -1.0);
    model.vcas = vec![vca("va", t.clone())];
    let vca_curve = vec![piece(
        AutomationShape::Linear,
        (0, 8_192),
        (-12.0, 0.0),
        ParameterUnit::Db,
    )];
    let own_curve = vec![piece(
        AutomationShape::Linear,
        (0, 4_096),
        (-6.5, -2.0),
        ParameterUnit::Db,
    )];
    model.automation = vec![
        entry_of(
            "va-ride",
            Owner::Vca(id("va")),
            Section::Own,
            VCA_OFFSET,
            vca_curve,
        ),
        entry_of(
            "t1-ride",
            Owner::Strip(t[1].clone()),
            Section::Builtins,
            FADER_DB,
            own_curve,
        ),
    ];
    let model = with_limiter(model, 0);
    let entries = model.automation.clone();
    let curve = |name: &str, at: i64| {
        let entry = entries
            .iter()
            .find(|entry| entry.id.as_str() == name)
            .expect("an entry");
        automation::value_at(&controls::stored_segments(entry), at)
    };
    for backend in backends() {
        let what = format!("{backend:?}");
        let mut rig = Rig::new(&model, backend);
        let latent = fader_arrival(&rig.prepared, &model, 0);
        assert_eq!(fader_arrival(&rig.prepared, &model, 1), 0, "{what}");
        assert!(
            !latent.is_multiple_of(64),
            "{what}: L = {latent} is off the grid"
        );
        let events = probed(&mut rig, 30);
        let mut samples = [Vec::new(), Vec::new()];
        for event in events
            .iter()
            .filter(|event| !event.mute && !event.held && event.kind == automation::EventKind::Grid)
        {
            // The member by its grid: `t1`'s node time is render time, `t0`'s is `L` behind.
            let member = usize::from(!event.sample.is_multiple_of(64));
            let arrival = [0, latent][member];
            assert_eq!(
                (event.sample - arrival) % 64,
                0,
                "{what}: {event:?} on a member's grid"
            );
            samples[member].push(event.sample);
            let at = (event.sample - arrival) as i64 + 63;
            if at >= 4_096 {
                continue;
            }
            let own = if member == 0 {
                curve("t1-ride", at)
            } else {
                -1.0
            };
            let expected = session::vca_compose_db(own, 0.0 + f64::from(curve("va-ride", at)));
            assert_eq!(
                builtins::checked_fader_gain(event.value)
                    .expect("fader domain")
                    .to_bits(),
                builtins::checked_fader_gain(expected)
                    .expect("fader domain")
                    .to_bits(),
                "{what}: {event:?} at node time {at}"
            );
        }
        for list in &mut samples {
            list.sort_unstable();
            list.dedup();
        }
        assert!(
            samples.iter().all(|list| list.len() >= 40),
            "{what}: both members move on their grids: {samples:?}"
        );
        let shifted: Vec<u64> = samples[0]
            .iter()
            .map(|sample| sample + latent)
            .filter(|sample| (latent + 64..3_840).contains(sample))
            .collect();
        let latent_grid: Vec<u64> = samples[1]
            .iter()
            .copied()
            .filter(|sample| (latent + 64..3_840).contains(sample))
            .collect();
        assert_eq!(
            shifted, latent_grid,
            "{what}: the latent member's events fall L later"
        );
        assert_eq!(rig.allocations, (0, 0), "{what}: the render thread");
    }
}

/// Gate 3 (#1655, amended): a VCA `mute` curve on `va` closes at timeline 1,000 (not a grid
/// sample) and opens at 2,500. `va` holds `t0` (behind a true-peak limiter, no mute entry of its
/// own), `t1` (a flat open mute entry of its own) and the nested VCA `vn`, which holds `t2`; `t3`
/// is in no VCA. Each member mutes at its own node time over the `VcaMute` length: the driver's
/// `SetMute` events fall on render samples `1,000` (`t1`, `t2`) and `1,000 + L` (`t0`) and back at
/// `2,500` and `2,500 + L`, each over `M`. From the first block boundary after the mute ramp's end
/// in the output until the unmute, the output equals the plan with `va` muted statically; from
/// the first boundary after the unmute ramp, the plan with no VCA mute. On every backend, live.
///
/// Test value: red if a member's mute follows the render clock (the latent member closes `L`
/// early), if a nested member or one with no own mute entry misses the VCA curve, or if the
/// composed mute ramps over another length.
#[test]
fn a_vca_mute_step_mutes_each_member_at_its_node_time() {
    let session = |curve: bool, muted: bool| {
        let mut model = plain(4, 128);
        let t: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
        let mut va = vca("va", vec![t[0].clone(), t[1].clone(), id("vn")]);
        (va.fader.left_mute, va.fader.right_mute) = (muted, muted);
        model.vcas = vec![va, vca("vn", vec![t[2].clone()])];
        model.automation.push(entry_of(
            "t1-mute",
            Owner::Strip(t[1].clone()),
            Section::Builtins,
            6,
            steps_of(0.0, &[(4_096, 0.0)], ParameterUnit::Linear),
        ));
        if curve {
            model
                .automation
                .push(vca_mute_steps("va", 0.0, &[(1_000, 1.0), (2_500, 0.0)]));
        }
        with_limiter(model, 0)
    };
    let (curve, muted, open) = (
        session(true, false),
        session(false, true),
        session(false, false),
    );
    const BLOCKS: usize = 40;
    for backend in backends() {
        let what = format!("{backend:?}");
        let mut rig = Rig::new(&curve, backend);
        let latent = fader_arrival(&rig.prepared, &curve, 0);
        assert!(latent > 0, "{what}: the limiter delays t0's fader");
        let events = probed(&mut rig, BLOCKS);
        let mut mutes: Vec<(u64, bool, u32)> = events
            .iter()
            .filter(|event| event.mute)
            .map(|event| (event.sample, event.value != 0.0, event.ramp))
            .collect();
        mutes.sort_unstable();
        mutes.dedup();
        assert_eq!(
            mutes,
            [
                (1_000, true, M),
                (1_000 + latent, true, M),
                (2_500, false, M),
                (2_500 + latent, false, M),
            ],
            "{what}: each member's mute at its node time over VcaMute"
        );
        let mut fixed = Rig::new(&muted, backend);
        fixed.blocks(BLOCKS);
        let mut plain = Rig::new(&open, backend);
        plain.blocks(BLOCKS);
        let q = rig.quantum as u64;
        let closed = ((1_000 + latent + u64::from(M)).div_ceil(q) * q) as usize;
        let opened = ((2_500 + latent + u64::from(M)).div_ceil(q) * q) as usize;
        let end = BLOCKS * rig.quantum;
        for lane in 0..2 {
            assert!(
                same_bits(
                    &rig.out[lane][closed..2_500],
                    &fixed.out[lane][closed..2_500]
                ),
                "{what}: lane {lane} equals the static VCA mute while the curve mutes"
            );
            assert!(
                same_bits(&rig.out[lane][opened..end], &plain.out[lane][opened..end]),
                "{what}: lane {lane} equals the open plan after the unmute"
            );
            assert!(
                !same_bits(&fixed.out[lane][..900], &rig.out[lane][..900]),
                "{what}: lane {lane} plays before the mute"
            );
        }
        assert_eq!(rig.allocations, (0, 0), "{what}: the render thread");
    }
}

/// R19-4 at host level (#1655): `t0`'s own fader steps from -3 to -9 dB at timeline 1,000 and
/// VCA `va`'s ride steps from 0 to -6 dB at the same sample; `t0`'s own mute and `va`'s mute
/// curve each step on at 3,000. The driver gives each channel one fader event at 1,000 over the
/// longest of the two jumps (the fader and `VcaOffset` rows' lengths, both `J` here) and one
/// `SetMute` at 3,000 over `M`, and the output equals, bit for bit, a session whose own curves
/// alone make the composed moves (-3 to -15 dB, one mute step) under a static `va` at 0 dB. On
/// every backend, live.
///
/// Test value: red if coincident curve events land as two jumps (the second retargets inside the
/// first), as a shorter jump, or compose another value than one curve making the sum would.
#[test]
fn coincident_member_and_vca_events_take_one_longest_jump() {
    let session = |vca_curves: bool| {
        let mut model = plain(1, 128);
        let t0 = model.tracks[0].id.clone();
        model.vcas = vec![vca("va", vec![t0.clone()])];
        let to = if vca_curves { -9.0 } else { -15.0 };
        model.automation = vec![
            entry_of(
                "t0-ride",
                Owner::Strip(t0.clone()),
                Section::Builtins,
                FADER_DB,
                steps_of(-3.0, &[(1_000, to)], ParameterUnit::Db),
            ),
            entry_of(
                "t0-mute",
                Owner::Strip(t0.clone()),
                Section::Builtins,
                6,
                steps_of(0.0, &[(3_000, 1.0)], ParameterUnit::Linear),
            ),
        ];
        if vca_curves {
            model.automation.extend([
                entry_of(
                    "va-ride",
                    Owner::Vca(id("va")),
                    Section::Own,
                    VCA_OFFSET,
                    steps_of(0.0, &[(1_000, -6.0)], ParameterUnit::Db),
                ),
                vca_mute_steps("va", 0.0, &[(3_000, 1.0)]),
            ]);
        }
        normalized(&model)
    };
    let (composed, single) = (session(true), session(false));
    for backend in backends() {
        let what = format!("{backend:?}");
        let mut rig = Rig::new(&composed, backend);
        let events = probed(&mut rig, 32);
        let at = |sample: u64, mute: bool| {
            let mut found: Vec<(usize, u32)> = events
                .iter()
                .filter(|event| event.sample == sample && event.mute == mute && !event.held)
                .map(|event| (event.channel, event.ramp))
                .collect();
            found.sort_unstable();
            found
        };
        assert_eq!(at(1_000, false), [(0, J), (1, J)], "{what}: one fader jump");
        assert_eq!(at(3_000, true), [(0, M), (1, M)], "{what}: one mute step");
        let mut twin = Rig::new(&single, backend);
        twin.blocks(32);
        for lane in 0..2 {
            assert!(
                same_bits(&rig.out[lane], &twin.out[lane]),
                "{what}: lane {lane} equals one curve making the sum"
            );
        }
    }
}

// Gate 8 (#1655 amendment): the strip carry of VCA mute automation across a plan swap.

/// The block a carry case's successor is swapped in at.
const SWAP_BLOCK: usize = 8;

/// A signed two-plane source that is never zero.
fn carry_pcm(frames: usize) -> [Vec<f32>; 2] {
    let plane = |channel: u32| {
        (0..frames as u32)
            .map(|frame| {
                let step = frame
                    .wrapping_mul(7_919)
                    .wrapping_add(channel * 31)
                    .wrapping_add(17)
                    % 1_999;
                (step as f32 - 999.5) / 2_000.0
            })
            .collect::<Vec<f32>>()
    };
    [plane(0), plane(1)]
}

/// What a carry run rendered: the planes, the swap block's `(allocations, frees)` and the
/// successor's restarted strips.
struct CarryRun {
    out: [Vec<f32>; 2],
    swap_allocations: (u64, u64),
    restarted: Vec<Box<str>>,
}

/// A strip record write of a carry run: before block `block` renders, through the live handles
/// and gate of the plan that renders it.
type CarryEdit<'a> = &'a dyn Fn(
    usize,
    &mut host_core::HostLiveControlHandles,
    &mut engine::realtime::PlanRevisionGate,
);

/// `a` prepared with live controls on `backend` and rendered for [`SWAP_BLOCK`] blocks, then, with
/// `b = Some((next, committed))`, `next` prepared as its successor on the committed model
/// `committed`, swapped in and rendered to `frames`; with `None`, `a` alone. `edit` runs before
/// every block on the plan that renders it. Fed `pcm` from frame 0.
fn carry_run(
    a: &SessionModel,
    b: Option<(&SessionModel, &SessionModel)>,
    backend: graph_compiler::Backend,
    pcm: &[Vec<f32>; 2],
    frames: usize,
    edit: CarryEdit<'_>,
) -> CarryRun {
    use engine::realtime::{PlanExchangeConfig, SwapOutcome, plan_exchange};
    let quantum = a.quantum_frames as usize;
    let caps = caps(a);
    let request = host_core::HostLiveControlRequest::default();
    let fail = |failure: host_core::PrepareDiagnostics| -> ! {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    };
    let (prepared, mut handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &compiled(a),
        &caps,
        &request,
        backend,
    )
    .unwrap_or_else(|failure| fail(failure));
    let PreparedHost {
        mut plan,
        mut sources,
        inventory,
        ..
    } = prepared;
    let mut gate = plan.take_revision_gate().expect("the plan's gate");
    let (mut publisher, mut owner, _retirer) = plan_exchange(
        plan,
        PlanExchangeConfig {
            retirement_capacity: core::num::NonZeroUsize::MIN,
        },
    )
    .unwrap_or_else(|_| panic!("plan exchange"));
    bench_support::alloc::assert_installed();
    let mut out = [Vec::new(), Vec::new()];
    let mut swap_allocations = (0, 0);
    let mut restarted = Vec::new();
    let mut successor: Option<(
        host_core::SourceControlSet,
        host_core::HostLiveControlHandles,
        engine::realtime::PlanRevisionGate,
    )> = None;
    let mut block = 0;
    while block * quantum < frames {
        if block == SWAP_BLOCK
            && let Some((next, committed)) = b
        {
            let committed = compiled(committed).normalized_model().clone();
            let revision = gate.last_published() + 1;
            let base = host_core::SuccessorBase::new(&inventory, &committed, revision);
            let (mut prepared, next_handles) =
                host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                    &compiled(next),
                    &caps,
                    &request,
                    base,
                    backend,
                )
                .unwrap_or_else(|failure| fail(failure));
            prepared.sources.adopt_persisting(&mut sources);
            restarted = prepared.restarted_strips().to_vec();
            let PreparedHost {
                mut plan,
                sources: next_sources,
                ..
            } = prepared;
            let mut next_gate = plan.take_revision_gate().expect("the successor's gate");
            next_gate.publish(revision);
            publisher
                .reserve_replacement(plan, engine::realtime::PlanAdoption::Next)
                .unwrap_or_else(|_| panic!("reserve the successor"))
                .commit(revision);
            successor = Some((next_sources, next_handles, next_gate));
        }
        let at = block * quantum;
        let (feed, live, live_gate) = match successor.as_mut() {
            Some((sources, handles, gate)) => (sources, handles, gate),
            None => (&mut sources, &mut handles, &mut gate),
        };
        feed.submit(
            NINE_SOURCE,
            SourceSubmission {
                generation: 1,
                start_frame: at as u64,
                sample_rate_hz: RATE,
                planes: &[&pcm[0][at..at + quantum], &pcm[1][at..at + quantum]],
                frames: quantum as u32,
                end_of_region: false,
            },
        )
        .expect("source block");
        edit(block, live, live_gate);
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        let report = owner
            .render_contiguous(RenderIo { output }, at as u64)
            .unwrap_or_else(|error| panic!("render: {error:?}"));
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        let swapped = report.swap == SwapOutcome::Applied;
        assert_eq!(
            swapped,
            b.is_some() && block == SWAP_BLOCK,
            "the successor is adopted at the swap block"
        );
        if swapped {
            swap_allocations = (delta.allocations + delta.reallocations, delta.deallocations);
        }
        out[0].extend_from_slice(&samples[..quantum]);
        out[1].extend_from_slice(&samples[quantum..]);
        block += 1;
    }
    drop(handles);
    drop(successor);
    CarryRun {
        out: out.map(|mut plane| {
            plane.truncate(frames);
            plane
        }),
        swap_allocations,
        restarted,
    }
}

/// The bank backends of the target (the scalar oracle binds the fader as a graph node, which a
/// swap restarts: it has no carry to hold, #1651 T4).
fn bank_backends() -> Vec<graph_compiler::Backend> {
    backends()
        .into_iter()
        .filter(|backend| *backend != graph_compiler::Backend::Scalar)
        .collect()
}

/// A VCA mute curve: its value at sample 0 and its steps.
type MuteCurve = Option<(f32, &'static [(u64, f32)])>;

/// Two tracks at `quantum`, `t0` in VCA `va` with `va`'s mute curve `curve`, if any, and `va`
/// muted statically when `muted`.
fn vca_mute_session(quantum: u32, curve: MuteCurve, muted: bool) -> SessionModel {
    let mut model = plain(2, quantum);
    let mut va = vca("va", vec![model.tracks[0].id.clone()]);
    (va.fader.left_mute, va.fader.right_mute) = (muted, muted);
    model.vcas = vec![va];
    if let Some((from, steps)) = curve {
        model.automation = vec![vca_mute_steps("va", from, steps)];
    }
    normalized(&model)
}

/// Gate 8 (#1655 amendment, U1 fold 1 of #1651 for the composed mute): a hand mute of `va`
/// committed in the same transaction as a change of `va`'s mute curve lands over its own ramp.
/// The curve closes `t0` at 1,500 and opens it at 2,500; at block 8 (the curve open) a rebuild
/// moves the close to 1,600 and the committed model holds `va` muted, whose `Mute` record on `t0`
/// (the shared commit's terms write, over the edit's ramp word: `M`, `0` or `2,000`) reaches the
/// successor's first block. A twin that never swaps takes the same record at the same block. The
/// curve's change lies inside the span the hand mute holds, so the swapped run equals the twin on
/// every sample, at both bank widths and quanta 128 and 100; the swap block allocates nothing and
/// the strip carries.
///
/// Test value: red if the swap block's adoption of the repositioned VCA mute cursor drops the
/// record's ramp word and lands the hand mute over the mute length (a `0` step becomes a
/// 480-sample ramp, a 2,000-sample fade becomes 480), or lands it twice.
#[test]
fn a_hand_vca_mute_in_a_vca_mute_curve_rebuild_lands_over_its_own_ramp() {
    for ramp in [M, 0, 2_000] {
        let mute = move |block: usize,
                         handles: &mut host_core::HostLiveControlHandles,
                         gate: &mut engine::realtime::PlanRevisionGate| {
            use builtins_compiler::TestOnlyWriteGate;
            if block == SWAP_BLOCK {
                gate.write_published(|stamp| {
                    handles.strip_controls[0].fader.write(
                        stamp,
                        builtins_compiler::TrackFaderRecord::Mute {
                            lanes: builtins::BuiltinLaneSelector::Both,
                            muted: true,
                            smoothing_samples: ramp,
                        },
                    );
                });
            }
        };
        for quantum in [128, 100] {
            let a = vca_mute_session(quantum, Some((0.0, &[(1_500, 1.0), (2_500, 0.0)])), false);
            let b = vca_mute_session(quantum, Some((0.0, &[(1_600, 1.0), (2_500, 0.0)])), true);
            let q = quantum as usize;
            let frames = 4_096_usize.div_ceil(q) * q;
            let pcm = carry_pcm(frames);
            for backend in bank_backends() {
                let what = format!("ramp {ramp}, {backend:?}, quantum {quantum}");
                let twin = carry_run(&a, None, backend, &pcm, frames, &mute).out;
                let swapped = carry_run(&a, Some((&b, &a)), backend, &pcm, frames, &mute);
                assert_eq!(swapped.swap_allocations, (0, 0), "{what}: the swap block");
                assert!(swapped.restarted.is_empty(), "{what}: the strip carries");
                for (plane, (out, twin)) in swapped.out.iter().zip(&twin).enumerate() {
                    assert!(
                        same_bits(out, twin),
                        "{what}: plane {plane} equals the hand mute alone"
                    );
                }
            }
        }
    }
}

/// Gate 8 (#1655 amendment): an added, a removed and a changed VCA `mute` entry each rebuild and
/// carry the member's stage. (a) Added: `va`'s new curve holds `t0` muted until 2,600; (b)
/// removed: the old curve held it muted until 3,600; (c) changed: the old curve closes at 3,000,
/// the new one is closed from 500 to 2,600. Swapped in at block 8, the strip carries, the swap
/// block allocates nothing, the output equals the predecessor's before the swap and a fresh plan
/// of the new model from the first block boundary after the `VcaMute` ramp (at both bank widths,
/// quanta 128 and 100), and it ramps there (it is not set).
///
/// Test value: red if a carried member lane does not take on a gained VCA mute curve (it stays
/// open), keeps a lost one's state (it stays muted), continues a changed table's cursor (the
/// unmute at 2,600 is missed), restarts the strip, or allocates at the swap.
#[test]
fn added_removed_and_changed_vca_mute_entries_carry_the_member_stage() {
    let cases: [(&str, MuteCurve, MuteCurve); 3] = [
        ("added", None, Some((1.0, &[(2_600, 0.0)]))),
        ("removed", Some((1.0, &[(3_600, 0.0)])), None),
        (
            "changed",
            Some((0.0, &[(3_000, 1.0)])),
            Some((0.0, &[(500, 1.0), (2_600, 0.0)])),
        ),
    ];
    for (label, before, after) in cases {
        for quantum in [128, 100] {
            let a = vca_mute_session(quantum, before, false);
            let b = vca_mute_session(quantum, after, false);
            let q = quantum as usize;
            let frames = 4_096_usize.div_ceil(q) * q;
            let pcm = carry_pcm(frames);
            let at = SWAP_BLOCK * q;
            let settled = (at + M as usize).div_ceil(q) * q;
            let none = |_: usize,
                        _: &mut host_core::HostLiveControlHandles,
                        _: &mut engine::realtime::PlanRevisionGate| {};
            for backend in bank_backends() {
                let what = format!("{label}, {backend:?}, quantum {quantum}");
                let fresh = carry_run(&b, None, backend, &pcm, frames, &none).out;
                let old = carry_run(&a, None, backend, &pcm, frames, &none).out;
                let swapped = carry_run(&a, Some((&b, &a)), backend, &pcm, frames, &none);
                assert_eq!(swapped.swap_allocations, (0, 0), "{what}: the swap block");
                assert!(swapped.restarted.is_empty(), "{what}: the strip carries");
                for plane in 0..2 {
                    assert!(
                        same_bits(&swapped.out[plane][..at], &old[plane][..at]),
                        "{what}: plane {plane} before the swap"
                    );
                    assert!(
                        same_bits(&swapped.out[plane][settled..], &fresh[plane][settled..]),
                        "{what}: plane {plane} from the first block boundary after the ramp"
                    );
                    assert!(
                        !same_bits(&swapped.out[plane][at..settled], &fresh[plane][at..settled]),
                        "{what}: plane {plane} ramps at the swap"
                    );
                }
            }
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
            let what = format!("{backend:?}");
            let mut live = Rig::new(current, backend);
            live.blocks(8);
            write(&mut live, records);
            live.blocks(24);
            let mut fresh = Rig::new(next, backend);
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

    /// #1655 U3 fold 1 (MAJOR-1), live: `t0` rides its own fader (flat 0 dB to 3,328, then
    /// linear to -24 dB over 98,304 samples: every grid target is a multiple of 2^-12 dB) and the
    /// automated offset of `va` steps from 0 to -6 dB at 3,000 (`J` = 240). At 1,024 a hand move
    /// of `va` to +3 dB over 8,000 samples writes each lane's `S` with its own ramp, which every
    /// cursor of the composed lane holds through; the step's jump at 3,000 replaces that ramp on
    /// the whole lane, as one cell's jump does, so the lane follows the own ride from 3,328 on.
    /// From the step's jump's end (3,240) every sample equals the session whose `t0` rides the
    /// summed curve as its own table under a static `va` given the same hand move (one cell), on
    /// every backend, live; the classifier gives both the same records.
    /// Before 3,000 the two differ: the record's ramp aims at each curve toward its completion,
    /// clipped at that curve's own next discontinuity (the VCA step), where one cell clips the
    /// sum at the lane's; the jump's end makes the gains equal.
    ///
    /// Test value: red if a jump of one curve of a composed lane leaves the hold the record put
    /// on every cursor in place (the own cursor's grid then freezes the lane to 9,024).
    #[test]
    fn a_vca_step_inside_a_hand_moves_ramp_lets_the_own_ride_move_as_one_cell() {
        const FROM: usize = 3_000 + J as usize;
        let ride_from = 3_328_u64;
        let ride_end = ride_from + 98_304;
        let session = |composed: bool| {
            let mut model = plain(1, 128);
            let t0 = model.tracks[0].id.clone();
            model.vcas = vec![vca("va", vec![t0.clone()])];
            let own = if composed {
                vec![
                    piece(
                        AutomationShape::Step,
                        (0, ride_from),
                        (0.0, 0.0),
                        ParameterUnit::Db,
                    ),
                    piece(
                        AutomationShape::Linear,
                        (ride_from, ride_end),
                        (0.0, -24.0),
                        ParameterUnit::Db,
                    ),
                ]
            } else {
                vec![
                    piece(
                        AutomationShape::Step,
                        (0, 3_000),
                        (0.0, 0.0),
                        ParameterUnit::Db,
                    ),
                    piece(
                        AutomationShape::Step,
                        (3_000, ride_from),
                        (-6.0, -6.0),
                        ParameterUnit::Db,
                    ),
                    piece(
                        AutomationShape::Linear,
                        (ride_from, ride_end),
                        (-6.0, -30.0),
                        ParameterUnit::Db,
                    ),
                ]
            };
            model.automation = vec![entry_of(
                "t0-ride",
                Owner::Strip(t0),
                Section::Builtins,
                FADER_DB,
                own,
            )];
            if composed {
                model.automation.push(entry_of(
                    "va-ride",
                    Owner::Vca(id("va")),
                    Section::Own,
                    VCA_OFFSET,
                    steps_of(0.0, &[(3_000, -6.0)], ParameterUnit::Db),
                ));
            }
            normalized(&model)
        };
        let mut ramp = EditRamps::new();
        ramp.set(
            &EditRampTarget::Vca(id("va")),
            EditRampRow::VcaFader,
            Some(8_000),
        );
        let row = address(Owner::Vca(id("va")), Section::Own, VCA_OFFSET);
        let pair = |composed: bool| {
            let current = session(composed);
            let next = edited(&current, |next| {
                controls::set(next, &row, 3.0, Some(8_000), RATE).expect("a hand move");
            });
            let records = classify(&current, &next, &ramp).expect("live");
            (current, records)
        };
        let ((composed, records), (single, single_records)) = (pair(true), pair(false));
        let t0 = composed.tracks[0].id.as_str().to_owned();
        let s = f64::from(3.0_f32);
        assert_eq!(
            records,
            vec![(t0, Vec::new(), [Some(offsets(s, 8_000, None)); 2])]
        );
        assert_eq!(records, single_records, "one cell's records");
        for backend in backends() {
            let what = format!("{backend:?}");
            let run = |model: &SessionModel| {
                let mut rig = Rig::new(model, backend);
                rig.blocks(8);
                write(&mut rig, &records);
                rig.blocks(92);
                assert_eq!(rig.allocations, (0, 0), "{what}: the render thread");
                rig.out
            };
            let (heard, expected) = (run(&composed), run(&single));
            for lane in 0..2 {
                let first = (FROM..heard[lane].len())
                    .find(|&at| heard[lane][at].to_bits() != expected[lane][at].to_bits());
                assert_eq!(
                    first, None,
                    "{what}: lane {lane}, first sample that differs"
                );
            }
            assert_ne!(
                heard[0][3_400].to_bits(),
                heard[0][9_000].to_bits(),
                "{what}: the own ride moves the lane inside the hand move"
            );
        }
    }
}
