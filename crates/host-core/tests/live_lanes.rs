//! Issue #1254 gate 1(a)-(c): `prepare_host_runtime_with_live_lanes` with
//! `HostLiveLanes::FADER_AND_MATRIX` attaches every strip's fader/mute and matrix/pan lanes and
//! nothing else.
//!
//! Two sessions. The nine-track EQ fixture carries an EQ console slot (whose tail is infinite) and
//! enabled input filters, so its tail is infinite whatever lanes are attached; it shows the effect
//! lanes staying off. The second is the same fixture with an empty console, no inserts and both
//! input filters off, so its lanes-free tail is finite; only it can show the tail rule.

use core::num::NonZeroUsize;

use builtins::{BuiltinLaneSelector, Matrix2x2};
use builtins_compiler::{
    TrackControlRecord, TrackFaderRecord, test_only_fader_matrix_witness,
    test_only_reset_fader_matrix_witness,
};
use dsp_reference::randomized::Draw;
use effect_contract::TailSamples;
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps,
    HostShapePolicy, PreparedHost, SourceSubmission, compile_host_session, prepare_host_runtime,
    prepare_host_runtime_between_render_calls, prepare_host_runtime_with_live_lanes,
};
use session::{
    Console, MatrixOrPan, RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix,
    canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const BLOCKS: usize = 8;
/// The strip whose fader lane gets the mute record.
const MUTED: usize = 2;
/// The strip whose matrix lane gets the matrix record.
const PANNED: usize = 5;
const MATRIX: Matrix2x2 = Matrix2x2 {
    ll: 0.5,
    lr: 0.25,
    rl: -0.5,
    rr: 0.75,
};

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn eq_fixture() -> SessionModel {
    parse_session_json(FIXTURE).expect("fixture parses")
}

/// The fixture with an empty console, no inserts and both input filters off on every track.
fn plain_fixture() -> SessionModel {
    let mut model = eq_fixture();
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
    }
    // The last track reaches the output through a unity submix, so the session has a route into
    // a submix: the one kind of route that gets a live lane under `HostLiveLanes::ALL`.
    let bus = StableId::parse("bus").expect("bus id");
    let last = model.routes.last_mut().expect("fixture route");
    let output = core::mem::replace(
        &mut last.destination,
        RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        },
    );
    let mut bus_main = last.clone();
    bus_main.id = StableId::parse("bus-main").expect("route id");
    bus_main.source = RouteSource::Submix {
        submix_id: bus.clone(),
        tap: SendTap::PostPan,
    };
    bus_main.destination = output;
    model.routes.push(bus_main);
    model.submixes.push(Submix::unity(bus, &model.console));
    model
}

/// `model` with strip `MUTED` muted and strip `PANNED` on `MATRIX`: what the live records set.
fn baked(mut model: SessionModel) -> SessionModel {
    let fader = &mut model.tracks[MUTED].fader;
    fader.left_mute = true;
    fader.right_mute = true;
    model.tracks[PANNED].matrix_or_pan = MatrixOrPan::Matrix {
        ll: MATRIX.ll,
        lr: MATRIX.lr,
        rl: MATRIX.rl,
        rr: MATRIX.rr,
        smoothing_samples: 0,
    };
    model
}

fn compile(model: &SessionModel) -> session::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

fn lanes_free(model: &SessionModel) -> PreparedHost {
    prepare_host_runtime(&compile(model), &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

fn request() -> HostLiveControlRequest {
    HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
        ..HostLiveControlRequest::default()
    }
}

fn with_lanes(
    model: &SessionModel,
    lanes: HostLiveLanes,
) -> (PreparedHost, HostLiveControlHandles) {
    prepare_host_runtime_with_live_lanes(&compile(model), &caps(), &request(), lanes)
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
}

fn feed(frames: usize) -> [Vec<f32>; 2] {
    let mut draw = Draw::new(0x1254);
    let mut plane = || -> Vec<f32> { (0..frames).map(|_| draw.noise(0.5)).collect() };
    [plane(), plane()]
}

/// Renders `BLOCKS` blocks from sample 0, feeding `planes`, and returns the output bits.
fn render(host: &mut PreparedHost, planes: &[Vec<f32>; 2]) -> Vec<u32> {
    let quantum = host.report.quantum_frames as usize;
    let rate = host.report.sample_rate_hz;
    let mut bits = Vec::with_capacity(quantum * 2 * BLOCKS);
    for block in 0..BLOCKS {
        let range = block * quantum..(block + 1) * quantum;
        host.sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = vec![0.0_f32; quantum * 2];
        let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
        host.plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: range.start as u64,
                },
            )
            .expect("render");
        bits.extend(samples.iter().map(|sample| sample.to_bits()));
    }
    bits
}

/// Gate 1(a). Red if a fader-and-matrix request still attaches the input lane (the plain
/// session's tail becomes infinite), attaches effect or route lanes, changes a rendered bit, or
/// prepares with `BetweenRenderCalls` delivery (a fused fader-matrix bank forms), which a
/// concurrent C ABI producer cannot declare (#1053 D5).
#[test]
fn fader_and_matrix_lanes_attach_nothing_else_and_render_the_lanes_free_bits() {
    // The precondition the tail half rests on: the plain session's lanes-free tail is finite,
    // and the EQ fixture's is not.
    assert!(matches!(
        lanes_free(&plain_fixture()).report.output_tail,
        TailSamples::Finite(_)
    ));
    assert_eq!(
        lanes_free(&eq_fixture()).report.output_tail,
        TailSamples::Infinite
    );
    for model in [eq_fixture(), plain_fixture()] {
        let strips = model.tracks.len() + model.submixes.len();
        let planes = feed(4_096);
        let mut reference = lanes_free(&model);
        // Delivery: the same request under `BetweenRenderCalls` fuses fader and matrix banks, so
        // the witness can see a fusion; the live-lanes entry must form none.
        test_only_reset_fader_matrix_witness();
        drop(
            prepare_host_runtime_between_render_calls(&compile(&model), &caps(), &request())
                .unwrap_or_else(|failure| {
                    panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
                }),
        );
        assert!(test_only_fader_matrix_witness().factory_calls > 0);
        test_only_reset_fader_matrix_witness();
        let (mut live, handles) = with_lanes(&model, HostLiveLanes::FADER_AND_MATRIX);
        assert_eq!(
            test_only_fader_matrix_witness().factory_calls,
            0,
            "Concurrent delivery forms no fused fader-matrix bank"
        );
        assert_eq!(handles.strip_controls.len(), strips);
        assert!(
            handles
                .strip_controls
                .iter()
                .zip(&handles.strips)
                .all(|(control, strip)| control.input.is_none() && control.track_id == *strip)
        );
        assert!(handles.effect_controls.is_empty());
        assert!(handles.route_controls.is_empty());
        assert!(handles.effect_observations.is_empty());
        assert_eq!(live.report.output_tail, reference.report.output_tail);
        assert_eq!(
            live.report.latency_samples,
            reference.report.latency_samples
        );
        assert_eq!(render(&mut live, &planes), render(&mut reference, &planes));
    }
    // The tail rule is what the selection changes: the same plain session under every lane is
    // infinite, because its strips then carry the input lane.
    let (all, handles) = with_lanes(&plain_fixture(), HostLiveLanes::ALL);
    assert!(
        handles
            .strip_controls
            .iter()
            .all(|control| control.input.is_some())
    );
    assert_eq!(
        handles.route_controls.len(),
        1,
        "the route into the bus is live under ALL"
    );
    assert_eq!(all.report.output_tail, TailSamples::Infinite);
}

/// Gate 1(b). Red if the lanes are attached but no longer drained by the banks.
#[test]
fn fader_and_matrix_lanes_render_as_the_session_with_their_values_baked() {
    for model in [eq_fixture(), plain_fixture()] {
        let planes = feed(4_096);
        let mut reference = lanes_free(&baked(model.clone()));
        let mut untouched = lanes_free(&model);
        let (mut live, mut handles) = with_lanes(&model, HostLiveLanes::FADER_AND_MATRIX);
        handles.strip_controls[MUTED]
            .fader
            .try_push(TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: 0,
            })
            .expect("fader queue room");
        handles.strip_controls[PANNED]
            .producer
            .try_push(TrackControlRecord {
                matrix: MATRIX,
                smoothing_samples: 0,
            })
            .expect("matrix queue room");
        let expected = render(&mut reference, &planes);
        assert_ne!(
            expected,
            render(&mut untouched, &planes),
            "the baked edits change the output"
        );
        assert_eq!(render(&mut live, &planes), expected);
    }
}

/// Gate 1(c). Red if an observation tap is attached without the effect queue it rides.
#[test]
fn observation_without_effect_lanes_is_refused() {
    let compiled = compile(&eq_fixture());
    let request = HostLiveControlRequest {
        observation_taps: 1,
        ..request()
    };
    let lanes = HostLiveLanes {
        effects: false,
        ..HostLiveLanes::ALL
    };
    let failure = prepare_host_runtime_with_live_lanes(&compiled, &caps(), &request, lanes)
        .err()
        .expect("observation without effect lanes is refused");
    assert!(
        String::from_utf8_lossy(failure.as_bytes()).contains("host.observation.live_controls"),
        "{}",
        String::from_utf8_lossy(failure.as_bytes())
    );
    // The same request with effect lanes prepares, so the refusal is the selection's.
    prepare_host_runtime_with_live_lanes(&compiled, &caps(), &request, HostLiveLanes::ALL)
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
}
