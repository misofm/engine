//! Issue #1200: a submix renders its strip on the D9 sum of the routes that target it.
//!
//! * **Gate 1, a bus equals a track fed its sum.** Three transparent tracks route, with random
//!   gains and matrices, into a submix carrying a random strip `S`; the same strip on one track
//!   whose source is the test's own `f32` sum of the three routed contributions renders the same
//!   bits. The oracle never runs the code under test: its arm has no submix at all.
//! * **Gate 2, PDC through a bus.** A bus limiter's latency reaches the arrival times.
//! * **Gate 3, a bus is never collapsed.** A mono track panned hard left into a processed bus
//!   leaves the bus's right output exactly `+0.0`.
//! * **Gate 5, the K1 live-control interim.** No bus effect gets a live channel or an
//!   observation handle until #1207.
//! * **Gate 8, render allocates nothing** for gate 1's bus session.

use core::num::{NonZeroU32, NonZeroUsize};

use bench_support::alloc as bench_alloc;
use builtins::MeterTap;
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_compiler::{EffectCompileCaps, launch_native_effect_registry};
use effect_contract::{
    LatencySamples, NativeEffectRegistry, ParameterChannelPolicy, ParameterDomain,
};
use engine::realtime::audit;
use graph::{GraphCompileCaps, GraphEdgeId};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use host_core::{
    HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission,
    compile_host_session, prepare_host_runtime, prepare_host_session_with_live_controls,
};
use session::{
    ChannelBuiltins, ChannelMatrix, DualMonoBuiltins, DualMonoFader, Effect, EffectIdentity,
    EffectParam, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel, ParameterUnit, Rack,
    Route, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source,
    StableId, Submix, canonical_session_json, compile_session, estimate_session_resources,
    parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const TEST: &str = "a_bus_renders_the_bits_of_a_track_fed_its_sum";
const REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test submix_strip \
                      -- --exact a_bus_renders_the_bits_of_a_track_fed_its_sum";

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
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
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn unity_matrix() -> ChannelMatrix {
    ChannelMatrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    }
}

fn route(id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: unity_matrix(),
        gain_db: 0.0,
    }
}

fn to_output(id: &str, source: RouteSource) -> Route {
    route(
        id,
        source,
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
    )
}

fn post_pan(track: &str) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::PostPan,
    }
}

fn bus_output(bus: &str) -> RouteSource {
    RouteSource::SubmixOutput {
        submix_id: sid(bus),
    }
}

fn into_bus(bus: &str) -> RouteDestination {
    RouteDestination::SubmixInput {
        submix_id: sid(bus),
    }
}

/// One owned strip: everything a track and a submix both carry.
#[derive(Clone)]
struct Strip {
    builtins: DualMonoBuiltins,
    inserts: Rack,
    fader: DualMonoFader,
    matrix_or_pan: MatrixOrPan,
}

impl Strip {
    fn transparent() -> Self {
        let Submix {
            builtins,
            inserts,
            fader,
            matrix_or_pan,
            ..
        } = Submix::unity(sid("unused"));
        Self {
            builtins,
            inserts,
            fader,
            matrix_or_pan,
        }
    }

    fn submix(self, id: &str) -> Submix {
        Submix {
            id: sid(id),
            builtins: self.builtins,
            inserts: self.inserts,
            fader: self.fader,
            matrix_or_pan: self.matrix_or_pan,
        }
    }
}

/// The fixture with its tracks, routes and sources removed, at `rate`.
fn empty_session(rate: u32) -> (SessionModel, Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = (QUANTUM * (BLOCKS + 8)) as u64;
    let mut track = model.tracks.swap_remove(0);
    track.console.clear();
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    let strip = Strip::transparent();
    track.builtins = strip.builtins;
    track.inserts = strip.inserts;
    track.fader = strip.fader;
    track.matrix_or_pan = strip.matrix_or_pan;
    (model, source, track)
}

/// Adds a two-channel source and a transparent track reading it, both named `id`.
fn add_track(model: &mut SessionModel, source: &Source, track: &session::Track, id: &str) {
    let mut source = source.clone();
    source.id = sid(id);
    model.sources.push(source);
    let mut track = track.clone();
    track.id = sid(id);
    track.source_id = sid(id);
    model.tracks.push(track);
}

fn document(model: &SessionModel) -> String {
    canonical_session_json(model).expect("generated session canonicalizes")
}

// ---- Gate 1 -------------------------------------------------------------------------------

fn unit(unit: effect_contract::ParameterUnit) -> ParameterUnit {
    use effect_contract::ParameterUnit as Contract;
    match unit {
        Contract::Db => ParameterUnit::Db,
        Contract::Hz => ParameterUnit::Hz,
        Contract::Milliseconds => ParameterUnit::Milliseconds,
        Contract::Samples => ParameterUnit::Samples,
        Contract::Linear => ParameterUnit::Linear,
        Contract::Ratio => ParameterUnit::Ratio,
    }
}

fn value(draw: &mut Draw, parameter: &effect_contract::ParameterDescriptor) -> f32 {
    let value = match parameter.domain {
        ParameterDomain::Boolean => draw.pick(&[0.0_f32, 1.0]),
        ParameterDomain::Enumeration => draw.pick(parameter.enum_choices).value,
        ParameterDomain::Continuous => draw.in_domain(
            parameter.minimum.unwrap_or(parameter.default_value),
            parameter.maximum.unwrap_or(parameter.default_value),
        ),
    };
    if effect_contract::parameter_value_valid(parameter, value) {
        value
    } else {
        parameter.default_value
    }
}

/// A native insert with every parameter drawn, per lane apart where the effect allows it.
fn drawn_effect(
    draw: &mut Draw,
    registry: &NativeEffectRegistry,
    kind: &str,
    id: &str,
    link_mode: LinkMode,
) -> Effect {
    let descriptor = registry
        .get_ascii(kind)
        .expect("launch effect")
        .descriptor();
    let mut params = Vec::new();
    for parameter in descriptor.parameters {
        if parameter.channel_policy == ParameterChannelPolicy::PerLane && draw.chance(1, 2) {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                params.push(EffectParam {
                    parameter_id: parameter.id.0,
                    channel,
                    unit: unit(parameter.unit),
                    value: value(draw, parameter),
                });
            }
        } else {
            params.push(EffectParam {
                parameter_id: parameter.id.0,
                channel: ParameterChannel::Both,
                unit: unit(parameter.unit),
                value: value(draw, parameter),
            });
        }
    }
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(kind),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        params,
        sidechain: SidechainDeclaration::None,
    }
}

fn drawn_lane(draw: &mut Draw) -> ChannelBuiltins {
    ChannelBuiltins {
        polarity_invert: draw.chance(1, 2),
        trim_db: draw.in_domain(-12.0, 12.0),
        hpf_hz: if draw.chance(1, 2) {
            0.0
        } else {
            draw.in_domain(20.0, 400.0)
        },
        lpf_hz: if draw.chance(1, 2) {
            0.0
        } else {
            draw.in_domain(2_000.0, 16_000.0)
        },
        delay_samples: 0,
    }
}

/// Strip `S`: a drawn input section, a `link_mode: maximum` compressor and an EQ, a fader and a
/// pan. No delay (#1201) and no console entries (#1202).
fn drawn_strip(draw: &mut Draw, registry: &NativeEffectRegistry) -> Strip {
    Strip {
        builtins: DualMonoBuiltins {
            left: drawn_lane(draw),
            right: drawn_lane(draw),
        },
        inserts: Rack {
            effects: vec![
                drawn_effect(draw, registry, "miso.compressor", "glue", LinkMode::Maximum),
                drawn_effect(
                    draw,
                    registry,
                    "miso.parametric-eq",
                    "tone",
                    LinkMode::DualMono,
                ),
            ],
        },
        fader: DualMonoFader {
            left_db: draw.in_domain(-24.0, 6.0),
            right_db: draw.in_domain(-24.0, 6.0),
            left_mute: false,
            right_mute: false,
        },
        matrix_or_pan: MatrixOrPan::Pan {
            left: draw.in_domain(-1.0, 1.0),
            right: draw.in_domain(-1.0, 1.0),
            smoothing_samples: draw.pick(&[0, 0, 64, 300]),
        },
    }
}

/// A nonzero coefficient in `[-1, -0.1] U [0.1, 1]`, so no routed product is a signed zero.
fn coefficient(draw: &mut Draw) -> f32 {
    let magnitude = 0.1 + 0.9 * draw.unit();
    if draw.chance(1, 2) {
        -magnitude
    } else {
        magnitude
    }
}

/// One routed contributor: its track ID, its route ID, the route, and its two source planes.
struct Contributor {
    track: String,
    route: Route,
    planes: [Vec<f32>; 2],
}

/// Gate 1's two sessions: A, three transparent tracks into a submix with strip `S`; B, one track
/// with strip `S` whose source is the test's own sum of A's routed contributions. Returns the two
/// documents and each one's `(source id, planes)` feed.
#[allow(clippy::type_complexity)]
fn gate_one_sessions(
    draw: &mut Draw,
    registry: &NativeEffectRegistry,
) -> (String, Vec<(String, [Vec<f32>; 2])>, String, [Vec<f32>; 2]) {
    let rate = draw.pick(&LAUNCH_RATES);
    let strip = drawn_strip(draw, registry);
    let frames = QUANTUM * BLOCKS;
    // Route IDs are a random permutation of the tracks, so route-ID order is not track order.
    let mut names = vec!["route-a", "route-b", "route-c"];
    let mut contributors: Vec<Contributor> = (0..3)
        .map(|index| {
            let route_id = names.remove(draw.below(names.len()));
            let track = format!("t{index}");
            let mut route = route(route_id, post_pan(&track), into_bus("bus"));
            route.gain_db = draw.in_domain(-12.0, 6.0);
            route.channel_matrix = ChannelMatrix {
                ll: coefficient(draw),
                lr: coefficient(draw),
                rl: coefficient(draw),
                rr: coefficient(draw),
            };
            let mut plane = || {
                (0..frames)
                    .map(|_| {
                        let sample = draw.noise(0.9);
                        if sample == 0.0 { 0.5 } else { sample }
                    })
                    .collect::<Vec<f32>>()
            };
            let planes = [plane(), plane()];
            Contributor {
                track,
                route,
                planes,
            }
        })
        .collect();

    let (mut a, source, track) = empty_session(rate);
    for contributor in &contributors {
        add_track(&mut a, &source, &track, &contributor.track);
        a.routes.push(contributor.route.clone());
    }
    a.submixes = vec![strip.clone().submix("bus")];
    a.routes.push(to_output("bus-main", bus_output("bus")));

    // The D9 sum, by the D3 expression: each route folds its gain into its matrix once, then
    // `l' = (lr * r) + (ll * l)` with two roundings, and the contributions are added left to right
    // in route-ID order.
    contributors.sort_by(|x, y| x.route.id.cmp(&y.route.id));
    let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for (rank, contributor) in contributors.iter().enumerate() {
        let gain = math::db_to_gain_f32(contributor.route.gain_db);
        let m = &contributor.route.channel_matrix;
        let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
        let [sum_left, sum_right] = &mut sum;
        let frames = sum_left
            .iter_mut()
            .zip(sum_right.iter_mut())
            .zip(contributor.planes[0].iter().zip(&contributor.planes[1]));
        for ((out_left, out_right), (&l, &r)) in frames {
            let left = (lr * r) + (ll * l);
            let right = (rr * r) + (rl * l);
            if rank == 0 {
                *out_left = left;
                *out_right = right;
            } else {
                *out_left += left;
                *out_right += right;
            }
        }
    }

    let (mut b, source, mut track) = empty_session(rate);
    track.builtins = strip.builtins;
    track.inserts = strip.inserts;
    track.fader = strip.fader;
    track.matrix_or_pan = strip.matrix_or_pan;
    add_track(&mut b, &source, &track, "summed");
    b.routes.push(to_output("summed-main", post_pan("summed")));

    let feeds = contributors
        .into_iter()
        .map(|contributor| (contributor.track, contributor.planes))
        .collect();
    (document(&a), feeds, document(&b), sum)
}

fn submit(prepared: &mut PreparedHost, feeds: &[(&str, &[Vec<f32>; 2])], block: usize) {
    let rate = prepared.report.sample_rate_hz;
    let range = block * QUANTUM..(block + 1) * QUANTUM;
    for (id, planes) in feeds {
        prepared
            .sources
            .submit(
                id.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
    }
}

/// Renders `blocks` quanta of `document` and returns its two output planes.
fn render(document: &str, feeds: &[(&str, &[Vec<f32>; 2])], blocks: usize) -> [Vec<f32>; 2] {
    let compiled = compile_host_session(document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut prepared = prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..blocks {
        submit(&mut prepared, feeds, block);
        let mut samples = [0.0_f32; QUANTUM * 2];
        prepared
            .plan
            .render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(
                        &mut samples,
                        2,
                        QUANTUM,
                        QUANTUM,
                    )
                    .expect("output planes"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        out[0].extend_from_slice(&samples[..QUANTUM]);
        out[1].extend_from_slice(&samples[QUANTUM..]);
    }
    out
}

#[test]
fn a_bus_renders_the_bits_of_a_track_fed_its_sum() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut audible = 0;
    let ran = run_seeds(TEST, REPLAY, 32, |seed| {
        let mut draw = Draw::new(seed);
        let (bus, feeds, oracle, sum) = gate_one_sessions(&mut draw, &registry);
        let feeds: Vec<(&str, &[Vec<f32>; 2])> = feeds
            .iter()
            .map(|(id, planes)| (id.as_str(), planes))
            .collect();
        let actual = render(&bus, &feeds, BLOCKS);
        let expected = render(&oracle, &[("summed", &sum)], BLOCKS);
        for plane in 0..2 {
            if let Some(index) = first_difference(&actual[plane], &expected[plane]) {
                panic!(
                    "seed {seed}: plane {plane} sample {index}: bus {:?} != track-fed-its-sum {:?}",
                    actual[plane][index], expected[plane][index]
                );
            }
        }
        if actual.iter().flatten().any(|sample| *sample != 0.0) {
            audible += 1;
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 32);
        assert!(
            audible > ran / 2,
            "too few seeds rendered audio: {audible}/{ran}"
        );
    }
}

/// Gate 1, the estimate half: a bus insert is charged once, as a track insert is.
#[test]
fn a_bus_insert_is_charged_once_in_the_session_estimate() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut draw = Draw::new(1_200);
    let (mut bare, source, track) = empty_session(48_000);
    add_track(&mut bare, &source, &track, "t0");
    bare.routes
        .push(route("t0-bus", post_pan("t0"), into_bus("bus")));
    bare.routes.push(to_output("bus-main", bus_output("bus")));
    bare.submixes = vec![Submix::unity(sid("bus"))];
    let mut processed = bare.clone();
    let insert = drawn_effect(
        &mut draw,
        &registry,
        "miso.compressor",
        "glue",
        LinkMode::Maximum,
    );
    processed.submixes[0].inserts.effects.push(insert.clone());

    let bare = estimate_session_resources(&bare).expect("bare estimate");
    let processed = estimate_session_resources(&processed).expect("processed estimate");
    let params = insert.params.len() as u64;
    assert_eq!(processed.effect_count - bare.effect_count, 1);
    assert_eq!(processed.parameter_count - bare.parameter_count, params);
    // By hand: the insert's vector element and its parameter vector, its two retained strings
    // (instance ID and native ID) once in the model and ten times in the canonical bound, and one
    // 1 KiB canonical item for the instance and for each parameter.
    let strings = (insert.id.as_str().len() + "miso.compressor".len()) as u64;
    let vectors = (size_of::<Effect>() as u64) + params * size_of::<EffectParam>() as u64;
    assert_eq!(
        processed.retained_string_bytes - bare.retained_string_bytes,
        strings
    );
    assert_eq!(
        processed.compiled_model_bytes - bare.compiled_model_bytes,
        vectors + strings + 10 * strings + 1_024 * (1 + params)
    );
}

/// Heavy inserts: eight effects of eight parameters each, and one effect of 100 parameters.
fn heavy_inserts() -> Vec<Effect> {
    let effect = |id: String, params: u32| Effect {
        id: sid(&id),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: (1..=params)
            .map(|parameter_id| EffectParam {
                parameter_id,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Linear,
                value: 0.5,
            })
            .collect(),
        sidechain: SidechainDeclaration::None,
    };
    let mut effects: Vec<Effect> = (0..8).map(|index| effect(format!("e{index}"), 8)).collect();
    effects.push(effect("wide".to_owned(), 100));
    effects
}

/// The #1199 verdict's MINOR-1: a submix's inserts enter the session estimate exactly as a
/// track's do, so the canonical writer's preflight bound covers them and a session whose bus
/// carries heavy inserts compiles. The same inserts on the track instead give the identical
/// estimate.
#[test]
fn heavy_bus_inserts_compile_and_are_estimated_as_track_inserts_are() {
    let (mut on_bus, source, track) = empty_session(48_000);
    add_track(&mut on_bus, &source, &track, "t0");
    on_bus
        .routes
        .push(route("t0-bus", post_pan("t0"), into_bus("bus")));
    on_bus.routes.push(to_output("bus-main", bus_output("bus")));
    on_bus.submixes = vec![Submix::unity(sid("bus"))];
    let mut on_track = on_bus.clone();
    on_bus.submixes[0].inserts.effects = heavy_inserts();
    on_track.tracks[0].inserts.effects = heavy_inserts();

    assert_eq!(
        estimate_session_resources(&on_bus).expect("bus estimate"),
        estimate_session_resources(&on_track).expect("track estimate"),
        "the same inserts are charged the same on a bus as on a track"
    );
    let caps = session::CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    for (name, model) in [("bus", &on_bus), ("track", &on_track)] {
        if let Err(diagnostics) = compile_session(model, caps) {
            panic!("heavy inserts on the {name} are refused: {diagnostics:?}");
        }
    }
}

// ---- Gate 2 -------------------------------------------------------------------------------

/// A true-peak limiter on the bus (486 samples at 48 kHz) and one contributor also routed straight
/// to the output.
fn pdc_session() -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
        model
            .routes
            .push(route(&format!("{id}-bus"), post_pan(id), into_bus("bus")));
    }
    model.routes.push(to_output("t0-direct", post_pan("t0")));
    model.routes.push(to_output("bus-main", bus_output("bus")));
    let mut bus = Strip::transparent();
    bus.inserts.effects.push(Effect {
        id: sid("ceiling"),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    model.submixes = vec![bus.submix("bus")];
    model
}

#[test]
fn a_bus_limiters_latency_is_compensated_on_the_direct_edge() {
    let model = pdc_session();
    let document = document(&model);
    let compiled = compile_host_session(&document, &caps()).expect("compile");
    let registry = launch_native_effect_registry().expect("launch registry");
    let effects = effect_compiler::prepare_native_session_effects(
        &compiled,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("effects");
    let builtins = prepare_session_builtins(
        &effects.session,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1_200,
        effects,
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: u64::MAX,
            maximum_edges: u64::MAX,
            maximum_schedule_items: u64::MAX,
            maximum_dependency_levels: u64::MAX,
            maximum_audio_buffer_samples: u64::MAX,
            maximum_delay_samples_per_edge: u64::MAX,
            maximum_total_delay_samples: u64::MAX,
            maximum_graph_bytes: u64::MAX,
            maximum_plan_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_finite_tail_samples: u64::MAX,
        },
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    assert_eq!(artifact.report().output_latency, LatencySamples(486));
    let delays = &artifact.graph().inserted_delays;
    assert_eq!(
        delays.len(),
        1,
        "exactly the direct edge is delayed: {delays:?}"
    );
    let direct = graph::StableGraphId::parse("t0-direct").expect("graph id");
    assert!(
        matches!(
            &delays[0].edge_id,
            GraphEdgeId::RouteSource { route_id } | GraphEdgeId::RouteDestination { route_id }
                if *route_id == direct
        ),
        "the delay sits on the direct route: {delays:?}"
    );
    assert_eq!(delays[0].samples, LatencySamples(486));

    // An impulse on t0 arrives at the output once through the bus and once directly; both arrive
    // at sample 486.
    let mut impulse = [
        vec![0.0_f32; QUANTUM * BLOCKS],
        vec![0.0_f32; QUANTUM * BLOCKS],
    ];
    impulse[0][0] = 0.25;
    impulse[1][0] = 0.25;
    let silence = [
        vec![0.0_f32; QUANTUM * BLOCKS],
        vec![0.0_f32; QUANTUM * BLOCKS],
    ];
    let out = render(&document, &[("t0", &impulse), ("t1", &silence)], BLOCKS);
    for (plane, samples) in out.iter().enumerate() {
        let peak = samples
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.abs().total_cmp(&y.1.abs()))
            .map(|(index, _)| index)
            .expect("rendered samples");
        assert_eq!(peak, 486, "plane {plane} peaks at sample {peak}");
        assert!(
            samples[486].abs() > 0.25,
            "plane {plane}: both paths arrive together"
        );
    }
}

// ---- Gate 3 -------------------------------------------------------------------------------

#[test]
fn a_mono_track_panned_left_into_a_processed_bus_leaves_its_right_silent() {
    let (mut model, source, mut track) = empty_session(48_000);
    track.left_source_channel = 0;
    track.right_source_channel = 0;
    track.matrix_or_pan = MatrixOrPan::Pan {
        left: -1.0,
        right: -1.0,
        smoothing_samples: 0,
    };
    add_track(&mut model, &source, &track, "mono");
    model
        .routes
        .push(route("mono-bus", post_pan("mono"), into_bus("bus")));
    model.routes.push(to_output("bus-main", bus_output("bus")));
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    let mut compressor = fixture.tracks[0].inserts.effects[0].clone();
    assert_eq!(compressor.link_mode, LinkMode::DualMono);
    compressor.id = sid("glue");
    let mut bus = Strip::transparent();
    bus.inserts.effects.push(compressor);
    model.submixes = vec![bus.submix("bus")];
    let document = document(&model);

    let mut draw = Draw::new(3);
    let planes = [
        (0..QUANTUM * BLOCKS)
            .map(|_| draw.noise(0.9))
            .collect::<Vec<_>>(),
        (0..QUANTUM * BLOCKS)
            .map(|_| draw.noise(0.9))
            .collect::<Vec<_>>(),
    ];
    let out = render(&document, &[("mono", &planes)], BLOCKS);
    assert!(
        out[0].iter().any(|sample| *sample != 0.0),
        "the left is audible"
    );
    for (index, sample) in out[1].iter().enumerate() {
        assert_eq!(
            sample.to_bits(),
            0.0_f32.to_bits(),
            "right sample {index} is {sample:?}: the bus was collapsed"
        );
    }
}

// ---- Gate 5 -------------------------------------------------------------------------------

#[test]
fn no_bus_effect_gets_a_live_channel_or_an_observation_handle() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut draw = Draw::new(5);
    let (bus, _, _, _) = gate_one_sessions(&mut draw, &registry);
    // Session A, with one track-owned compressor so the track-owned half is not vacuous.
    let mut model = parse_session_json(&bus).expect("session A parses");
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    model.tracks[0]
        .inserts
        .effects
        .push(fixture.tracks[0].inserts.effects[0].clone());
    let document = document(&model);
    let (_, _, handles) = prepare_host_session_with_live_controls(
        &document,
        &caps(),
        &HostLiveControlRequest {
            control_queue_depth: Some(NonZeroUsize::new(64).expect("depth")),
            meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
            meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
            meter_tap: MeterTap::PostMatrix,
            observation_taps: 4,
            master_track: None,
        },
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));
    let tracks: Vec<&str> = model.tracks.iter().map(|track| track.id.as_str()).collect();
    let controls: Vec<(&str, &str)> = handles
        .effect_controls
        .iter()
        .map(|control| (&*control.track_id, &*control.effect_id))
        .collect();
    let observations: Vec<(&str, &str)> = handles
        .effect_observations
        .iter()
        .map(|handle| (&*handle.track_id, &*handle.effect_id))
        .collect();
    assert!(
        controls
            .iter()
            .chain(&observations)
            .all(|(owner, _)| tracks.contains(owner)),
        "a bus effect got a live channel: {controls:?} {observations:?}"
    );
    assert_eq!(controls, [("t0", "comp")]);
    assert_eq!(observations, [("t0", "comp")]);
}

// ---- Gate 8 -------------------------------------------------------------------------------

#[test]
fn a_processed_bus_renders_without_allocating() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut draw = Draw::new(8);
    let (bus, feeds, _, _) = gate_one_sessions(&mut draw, &registry);
    let compiled = compile_host_session(&bus, &caps()).expect("compile");
    let prepared = prepare_host_runtime(&compiled, &caps()).expect("prepare");
    let rate = prepared.report.sample_rate_hz;
    let (mut session, mut sources, _) = prepared
        .start_render_session()
        .unwrap_or_else(|_| panic!("render session starts"));
    audit::warm_up();
    bench_alloc::assert_installed();
    for block in 0..BLOCKS {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        for (id, planes) in &feeds {
            sources
                .submit(
                    id.as_bytes(),
                    SourceSubmission {
                        generation: 1,
                        start_frame: range.start as u64,
                        sample_rate_hz: rate,
                        planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
        }
        let mut pcm = [0.0_f32; QUANTUM * 2];
        audit::reset();
        let mark = bench_alloc::current_thread_counters();
        let (report, snapshot) = audit::in_render_scope(|| {
            let report = session.render_planar(&mut pcm, 2, QUANTUM, QUANTUM, range.start as u64);
            (report, audit::snapshot())
        });
        let delta = bench_alloc::current_thread_delta_since(mark);
        assert!(report.is_ok(), "render: {report:?}");
        // Block 0 is the warm-up: the first render may initialise process statics.
        if block > 0 {
            assert_eq!(
                (snapshot.allocations, snapshot.deallocations),
                (0, 0),
                "block {block}: the render audit saw the allocator"
            );
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: the render thread allocated or freed"
            );
        }
    }
}
