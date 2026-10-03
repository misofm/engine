//! Issue #1216: a route's `mute` is a session switch, and it is not structural.
//!
//! * **Gate 1, a muted route mixes zero coefficients.** Bus `b` sums two contributors fed
//!   distinct noise, one route muted; the bus renders the bits of a track fed the scalar D9 sum
//!   that mixes the muted route with `[+0.0; 4]` (D3).
//! * **Gate 3, mute is not structural.** Muting the route PDC delays leaves every inserted delay,
//!   route timing and the output latency unchanged.
//! * **Gate 4, no folded muted lane.** A bus with a muted contributor declines the route fold and
//!   renders the bits of the same plan bound with the fold declined.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_compiler::{EffectCompileCaps, launch_native_effect_registry};
use effect_contract::LatencySamples;
use graph::{GraphCompileCaps, GraphEdgeId};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use host_core::{
    HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission, compile_host_session,
    prepare_host_runtime,
};
use session::{
    ChannelMatrix, Console, Effect, EffectIdentity, EffectQuality, LinkMode, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source, StableId,
    Submix, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const FRAMES: usize = QUANTUM * BLOCKS;
const TEST: &str = "a_muted_route_mixes_zero_coefficients";
const REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test route_mute \
                      -- --exact a_muted_route_mixes_zero_coefficients";

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
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

fn route(id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
        mute: false,
    }
}

fn post_pan(track: &str) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::PostPan,
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

fn into_bus(bus: &str) -> RouteDestination {
    RouteDestination::SubmixInput {
        submix_id: sid(bus),
    }
}

fn bus_output(bus: &str) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(bus),
        tap: SendTap::PostPan,
    }
}

/// The fixture with its tracks, routes, sources and console removed, at 48 kHz. The returned
/// track is transparent: a unity strip.
fn empty_session() -> (SessionModel, Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = 48_000;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = (QUANTUM * (BLOCKS + 8)) as u64;
    let mut track = model.tracks.swap_remove(0);
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    let unity = Submix::unity(sid("unused"), &model.console);
    track.builtins = unity.builtins;
    track.console = unity.console;
    track.inserts = unity.inserts;
    track.fader = unity.fader;
    track.matrix_or_pan = unity.matrix_or_pan;
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

fn submit(prepared: &mut PreparedHost, feeds: &[(String, [Vec<f32>; 2])], block: usize) {
    let range = block * QUANTUM..(block + 1) * QUANTUM;
    for (id, planes) in feeds {
        prepared
            .sources
            .submit(
                id.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
    }
}

/// Renders [`BLOCKS`] quanta of `document`; returns its output planes and the plan's admitted
/// route-fold lane count.
fn render(document: &str, feeds: &[(String, [Vec<f32>; 2])]) -> ([Vec<f32>; 2], u64) {
    let compiled = compile_host_session(document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut prepared = prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
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
    (out, prepared.plan.bank_route_folds())
}

fn assert_bits_equal(actual: &[f32], expected: &[f32], what: &str) {
    if let Some(index) = first_difference(actual, expected) {
        panic!(
            "{what}: sample {index}: {:?} != {:?}",
            actual[index], expected[index]
        );
    }
}

// ---- Gate 1 -------------------------------------------------------------------------------

/// A nonzero magnitude in `[0.1, 1]` with the given sign.
fn signed(draw: &mut Draw, negative: bool) -> f32 {
    let magnitude = 0.1 + 0.9 * draw.unit();
    if negative { -magnitude } else { magnitude }
}

/// One contributor's planes: nonzero noise, except, given `zero`, every fourth frame, which carries
/// those signed zeros (left, right).
fn planes(draw: &mut Draw, zero: Option<[f32; 2]>) -> [Vec<f32>; 2] {
    let mut plane = |lane: usize| {
        (0..FRAMES)
            .map(|frame| {
                if let Some(zero) = zero.filter(|_| frame.is_multiple_of(4)) {
                    return zero[lane];
                }
                let sample = draw.noise(0.9);
                if sample == 0.0 { 0.5 } else { sample }
            })
            .collect::<Vec<f32>>()
    };
    [plane(0), plane(1)]
}

/// The D9 sum of `routes` in route-ID order, each by the D3 expression `(lr * r) + (ll * l)`
/// over its coefficients: `gain * matrix` for an open route and `[+0.0; 4]` for a muted one.
fn routed_sum(routes: &mut [(&Route, &[Vec<f32>; 2])]) -> [Vec<f32>; 2] {
    routes.sort_by(|x, y| x.0.id.cmp(&y.0.id));
    let mut sum = [vec![0.0_f32; FRAMES], vec![0.0_f32; FRAMES]];
    for (rank, (route, planes)) in routes.iter().enumerate() {
        let (ll, lr, rl, rr) = if route.mute {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            let gain = math::db_to_gain_f32(route.gain_db);
            let m = &route.channel_matrix;
            (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr)
        };
        for frame in 0..FRAMES {
            let (l, r) = (planes[0][frame], planes[1][frame]);
            let left = (lr * r) + (ll * l);
            let right = (rr * r) + (rl * l);
            if rank == 0 {
                sum[0][frame] = left;
                sum[1][frame] = right;
            } else {
                sum[0][frame] += left;
                sum[1][frame] += right;
            }
        }
    }
    sum
}

/// Gate 1. Tracks `t0` and `t1` route from their `input` taps into bus `b` by `r0` and `r1`, with
/// drawn gains and nonzero matrices, and one of the two (drawn) is muted; the bus's `input` tap (its
/// D9 sum) feeds the output. The oracle renders no submix and no mute: a track reads the scalar
/// [`routed_sum`] and feeds the output from its own `input` tap, by the same unity route.
///
/// The open contributor's every fourth frame is the signed zero that makes its contribution `-0.0`
/// on both lanes (its matrix columns share a sign per source lane, and the zero takes the other
/// sign), so on those frames the bus sum *is* the muted contribution, `(+0.0 * r) + (+0.0 * l)`,
/// whose sign follows the muted track's samples alone. The `input` taps keep every signed zero:
/// a strip's pan would turn some of them positive and hide the muted contribution's sign.
///
/// Red if the compiler drops `mute` (the muted route mixes its open coefficients), or applies it
/// as anything other than four `+0.0` coefficients: through the gain, a muted coefficient keeps
/// its matrix entry's sign, and the zero frames show it.
#[test]
fn a_muted_route_mixes_zero_coefficients() {
    let input = |track: &str| RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::Input,
    };
    let mut negative_zeros = 0;
    let ran = run_seeds(TEST, REPLAY, 16, |seed| {
        let mut draw = Draw::new(seed);
        let (mut model, source, track) = empty_session();
        let muted = draw.below(2);
        let mut feeds = Vec::new();
        let mut routes = Vec::new();
        for index in 0..2 {
            let id = format!("t{index}");
            add_track(&mut model, &source, &track, &id);
            let mut drawn = route(&format!("r{index}"), input(&id), into_bus("b"));
            drawn.gain_db = draw.in_domain(-12.0, 6.0);
            // Column signs: `ll`/`rl` scale the left source lane, `lr`/`rr` the right.
            let (left_negative, right_negative) = (draw.chance(1, 2), draw.chance(1, 2));
            drawn.channel_matrix = ChannelMatrix {
                ll: signed(&mut draw, left_negative),
                lr: signed(&mut draw, right_negative),
                rl: signed(&mut draw, left_negative),
                rr: signed(&mut draw, right_negative),
            };
            drawn.mute = index == muted;
            // A zero of the column's opposite sign makes each open product `-0.0`.
            let zero = |negative: bool| if negative { 0.0 } else { -0.0 };
            let zeros = (!drawn.mute).then(|| [zero(left_negative), zero(right_negative)]);
            feeds.push((id, planes(&mut draw, zeros)));
            routes.push(drawn);
        }
        model.routes = routes.clone();
        model.routes.push(to_output(
            "b-main",
            RouteSource::Submix {
                submix_id: sid("b"),
                tap: SendTap::Input,
            },
        ));
        model.submixes = vec![Submix::unity(sid("b"), &model.console)];
        let (actual, _) = render(&document(&model), &feeds);

        let mut contributions: Vec<(&Route, &[Vec<f32>; 2])> = routes
            .iter()
            .zip(feeds.iter().map(|(_, planes)| planes))
            .collect();
        let sum = routed_sum(&mut contributions);
        let (mut oracle, _, track) = empty_session();
        add_track(&mut oracle, &source, &track, "summed");
        oracle
            .routes
            .push(to_output("summed-main", input("summed")));
        let (expected, _) = render(&document(&oracle), &[("summed".to_owned(), sum)]);
        for plane in 0..2 {
            assert_bits_equal(
                &actual[plane],
                &expected[plane],
                &format!("seed {seed}, r{muted} muted, plane {plane}"),
            );
        }
        assert!(
            actual.iter().flatten().any(|sample| *sample != 0.0),
            "seed {seed}: the open route rendered audio"
        );
        negative_zeros += actual
            .iter()
            .flatten()
            .filter(|sample| sample.to_bits() == (-0.0_f32).to_bits())
            .count();
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 16);
        // The muted contribution's sign reached the output, or the zero frames prove nothing.
        assert!(negative_zeros > 0, "no muted contribution rendered as -0.0");
    }
}

// ---- Gate 3 -------------------------------------------------------------------------------

/// `t0` and `t1` feed a bus carrying a true-peak limiter (486 samples at 48 kHz); `t0` also feeds
/// the output directly by `t0-direct`, the edge PDC delays.
fn latent_session(direct_muted: bool) -> SessionModel {
    let (mut model, source, track) = empty_session();
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
        model
            .routes
            .push(route(&format!("{id}-bus"), post_pan(id), into_bus("bus")));
    }
    let mut direct = to_output("t0-direct", post_pan("t0"));
    direct.mute = direct_muted;
    model.routes.push(direct);
    model.routes.push(to_output("bus-main", bus_output("bus")));
    let mut bus = Submix::unity(sid("bus"), &model.console);
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
    model.submixes = vec![bus];
    model
}

/// The graph artifact of `document`, through the pipeline host-core uses: `PreparedHost` exposes
/// no inserted delays.
fn graph_artifact(document: &str) -> PreparedGraphBuiltinsArtifact {
    let compiled = compile_host_session(document, &caps()).expect("compile");
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
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1_216,
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
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics))
}

/// Gate 3. Muting `t0-direct`, the one route PDC delays, keeps its 486-sample delay, every route
/// timing and the output latency: the muted plan differs from the open one in the gate alone.
///
/// Red if mute drops the route from the graph or its PDC, which would make a later live unmute a
/// latency change.
#[test]
fn muting_the_delayed_route_moves_no_delay_or_latency() {
    let open = graph_artifact(&document(&latent_session(false)));
    let muted = graph_artifact(&document(&latent_session(true)));
    assert_eq!(muted.report().output_latency, LatencySamples(486));
    assert_eq!(muted.report().output_latency, open.report().output_latency);
    let (open, muted) = (open.graph(), muted.graph());
    assert_eq!(muted.inserted_delays, open.inserted_delays);
    let direct = graph::StableGraphId::parse("t0-direct").expect("graph id");
    assert!(
        matches!(
            muted.inserted_delays.as_slice(),
            [delay] if delay.samples == LatencySamples(486) && matches!(&delay.edge_id,
                GraphEdgeId::RouteSource { route_id } | GraphEdgeId::RouteDestination { route_id }
                    if *route_id == direct)
        ),
        "the muted route keeps its delay: {:?}",
        muted.inserted_delays
    );
    assert_eq!(muted.route_timings, open.route_timings);
    assert_eq!(muted.spec.nodes.len(), open.spec.nodes.len());
    assert_eq!(muted.spec.edges.len(), open.spec.edges.len());
}

// ---- Gate 4 -------------------------------------------------------------------------------

/// Eight tracks feed bus `bus`, and the bus feeds the output from its `pre_fader` tap; `muted` names
/// the route into the bus that is muted, if any.
///
/// The bus's input is the plan's one fold candidate: the fold folds one reduction, and the output's
/// single contributor reads the bus mid-strip, not a chain's last slot, so the output never folds.
/// (From the bus's `post_pan` the output would fold its one lane once the bus declines.)
fn eight_into_one(muted: Option<usize>) -> String {
    let (mut model, source, track) = empty_session();
    for index in 0..8 {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        let mut into = route(&format!("{id}-bus"), post_pan(&id), into_bus("bus"));
        into.mute = muted == Some(index);
        model.routes.push(into);
    }
    model.routes.push(to_output(
        "bus-main",
        RouteSource::Submix {
            submix_id: sid("bus"),
            tap: SendTap::PreFader,
        },
    ));
    model.submixes = vec![Submix::unity(sid("bus"), &model.console)];
    document(&model)
}

/// Gate 4. With one of eight contributors muted, the bus folds no lane, and it renders the bits of
/// the same plan bound with the fold declined; the same session unmuted does fold.
///
/// Red if the fold accumulates a muted contribution with the route's open coefficients, or folds a
/// gated route at all.
#[test]
fn a_bus_with_a_muted_contributor_folds_no_lane() {
    let mut draw = Draw::new(1_216);
    let feeds: Vec<(String, [Vec<f32>; 2])> = (0..8)
        .map(|index| {
            let mut plane = || (0..FRAMES).map(|_| draw.noise(0.9)).collect::<Vec<f32>>();
            (format!("t{index}"), [plane(), plane()])
        })
        .collect();
    let (_, open_folds) = render(&eight_into_one(None), &feeds);
    assert!(
        open_folds > 0,
        "the open bus must be a fold master, or this is vacuous"
    );
    let muted = eight_into_one(Some(3));
    let (folded, folds) = render(&muted, &feeds);
    assert_eq!(folds, 0, "a bus with a muted contributor declines the fold");
    graph::test_only_set_route_fold_declined(true);
    let (declined, declined_folds) = render(&muted, &feeds);
    graph::test_only_set_route_fold_declined(false);
    assert_eq!(declined_folds, 0, "the declined arm must not fold");
    for plane in 0..2 {
        assert_bits_equal(
            &folded[plane],
            &declined[plane],
            &format!("plane {plane}: the muted bus against the declined fold"),
        );
    }
    assert!(
        folded.iter().flatten().any(|sample| *sample != 0.0),
        "the muted bus rendered audio"
    );
    eprintln!("route_mute gate 4: the open bus folds {open_folds} lanes, the muted bus 0");
}
