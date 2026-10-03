//! Issue #1200: a submix renders its strip on the D9 sum of the routes that target it.
//!
//! * **Gate 1, a bus equals a track fed its sum.** Three transparent tracks route, with random
//!   gains and matrices, into a submix carrying a random strip `S`; the same strip on one track
//!   whose source is the test's own `f32` sum of the three routed contributions renders the same
//!   bits. The oracle never runs the code under test: its arm has no submix at all.
//! * **Gate 2, PDC through a bus.** A bus limiter's latency reaches the arrival times.
//! * **Gate 3, a bus is never collapsed.** A mono track panned hard left into a processed bus
//!   leaves the bus's right output exactly `+0.0`.
//! * **Gate 8, render allocates nothing** for gate 1's bus session.
//! * **A bus into a bus** (verdict MINOR-3): a nested bus renders the bits of two tracks, each fed
//!   its sum.
//!
//! Issue #1201: a submix's `delay_samples` delays its summed input, uncompensated.
//!
//! * **Gate 1, the bus delay runs after the sum**, and PDC never sees it.
//! * **Gate 3, a folded bus still delays its sum.**
//! * **Gate 4, render allocates nothing** for gate 1's delayed bus.
//! * **Two delayed buses** (verdict MINOR-1): a delayed track into a delayed bus, beside a second
//!   delayed bus; every line is its own and delays add.
//!
//! Issue #1202: every submix carries every session console slot.
//!
//! * **Gate 1, the bus equals a track, with console slots**: gate 1's sessions declare
//!   `pre_insert: [eq, compressor]` and `post_insert: [true-peak limiter]`, the bus and the
//!   reference track carry drawn entries, and the contributors bypass every slot.
//! * **Gate 3, a bypassed bus slot keeps its latency.**
//! * **Gate 8, render allocates nothing** for gate 1's console session.
//!
//! Issue #1203: a route or a sidechain leaves a submix strip at any of the seven taps.
//!
//! * **Gate 1, a bus tap equals the same tap on a track fed the bus's sum**, for every tap, on a
//!   strip where every tap differs.
//! * **Gate 2, a sidechain keyed by a bus's `pre_fader`** reads that tap.
//! * **Gate 3, PDC by tap**: a tap before a bus's `post_insert` limiter is compensated by its
//!   latency.
//! * **Gate 6, render allocates nothing** for gate 1's tapped buses.
//! * **D4, a muted bus still feeds its `pre_fader` tap** (verdict MINOR-2).

use bench_support::alloc as bench_alloc;
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_compiler::{EffectCompileCaps, launch_native_effect_registry};
use effect_contract::{
    LatencySamples, NativeEffectRegistry, ParameterChannelPolicy, ParameterDomain,
};
use engine::realtime::audit;
use graph::{GraphCompileCaps, GraphEdgeId};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use host_core::{
    HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission, compile_host_session,
    prepare_host_runtime,
};
use session::{
    ChannelBuiltins, ChannelMatrix, Console, ConsoleEntry, ConsoleSlot, DualMonoBuiltins,
    DualMonoFader, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, MatrixOrPan,
    ParameterChannel, ParameterUnit, Rack, Route, RouteDestination, RouteSource, SendTap,
    SessionModel, SidechainDeclaration, Source, StableId, Submix, canonical_session_json,
    compile_session, estimate_session_resources, parse_session_json,
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
        mute: false,
        follows_mute: false,
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
    RouteSource::Submix {
        submix_id: sid(bus),
        tap: SendTap::PostPan,
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
    console: Vec<ConsoleEntry>,
    inserts: Rack,
    fader: DualMonoFader,
    matrix_or_pan: MatrixOrPan,
}

impl Strip {
    /// The transparent strip under `console`: every slot bypassed (#1202 D4).
    fn transparent(console: &Console) -> Self {
        let Submix {
            builtins,
            console,
            inserts,
            fader,
            matrix_or_pan,
            ..
        } = Submix::unity(sid("unused"), console);
        Self {
            builtins,
            console,
            inserts,
            fader,
            matrix_or_pan,
        }
    }

    /// Puts this strip on `track`.
    fn onto(self, track: &mut session::Track) {
        track.builtins = self.builtins;
        track.console = self.console;
        track.inserts = self.inserts;
        track.fader = self.fader;
        track.matrix_or_pan = self.matrix_or_pan;
    }

    fn submix(self, id: &str) -> Submix {
        Submix {
            id: sid(id),
            builtins: self.builtins,
            console: self.console,
            inserts: self.inserts,
            fader: self.fader,
            matrix_or_pan: self.matrix_or_pan,
        }
    }
}

/// The fixture with its tracks, routes and sources removed, at `rate`, with no console.
fn empty_session(rate: u32) -> (SessionModel, Source, session::Track) {
    empty_session_with(rate, no_console(), BLOCKS)
}

fn no_console() -> Console {
    Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    }
}

/// [`empty_session`] declaring `console`, with a source long enough for `blocks` quanta. The
/// returned track is transparent: it carries every slot bypassed.
fn empty_session_with(
    rate: u32,
    console: Console,
    blocks: usize,
) -> (SessionModel, Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = (QUANTUM * (blocks + 8)) as u64;
    let mut track = model.tracks.swap_remove(0);
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    model.console = console;
    Strip::transparent(&model.console).onto(&mut track);
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

/// Gate 1's console (#1202): `pre_insert: [eq, compressor]`, `post_insert: [true-peak limiter]`.
fn gate_one_console() -> Console {
    let slot = |id: &str, kind: &str, link_mode| ConsoleSlot {
        slot: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(kind),
        },
        quality: EffectQuality::Normal,
        link_mode,
    };
    Console {
        pre_insert: vec![
            slot("desk-eq", "miso.parametric-eq", LinkMode::DualMono),
            slot("desk-comp", "miso.compressor", LinkMode::Maximum),
        ],
        post_insert: vec![slot(
            "desk-limit",
            "miso.true-peak-limiter",
            LinkMode::Maximum,
        )],
    }
}

/// Strip `S`: a drawn input section, an entry for every slot of `console` with a drawn bypass and
/// drawn parameters (#1202), a `link_mode: maximum` compressor and an EQ, a fader and a pan. No
/// delay (#1201).
fn drawn_strip(draw: &mut Draw, registry: &NativeEffectRegistry, console: &Console) -> Strip {
    Strip {
        builtins: DualMonoBuiltins {
            left: drawn_lane(draw),
            right: drawn_lane(draw),
        },
        console: console
            .slots()
            .map(|slot| {
                let EffectIdentity::Native { effect_id } = &slot.identity else {
                    panic!("a console slot is native");
                };
                let drawn = drawn_effect(
                    draw,
                    registry,
                    effect_id.as_str(),
                    slot.slot.as_str(),
                    slot.link_mode,
                );
                ConsoleEntry {
                    slot: slot.slot.clone(),
                    bypass: draw.chance(1, 3),
                    params: drawn.params,
                }
            })
            .collect(),
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

/// Gate 1's rendered blocks: the console's latency is paid twice on the bus path, `2 L` with
/// `L = rate / 100 + 6` (the bypassed limiter's), so at 96 kHz the output starts at frame 1932.
const GATE_ONE_BLOCKS: usize = 24;

/// The latency of a bypassed true-peak limiter at `rate` (decision 12: bypass keeps it).
const fn limiter_latency(rate: u32) -> usize {
    (rate / 100 + 6) as usize
}

/// Three contributors `t0`..`t2` routed `post_pan` into `bus` with drawn gains, matrices and
/// noise planes of `frames` samples, sorted by route ID, and the D9 sum, by the D3 expression, of
/// each contributor delayed by `latency`: each route folds its gain into its matrix once, then
/// `l' = (lr * r) + (ll * l)` with two roundings, and the contributions are added left to right in
/// route-ID order.
fn drawn_contributors(
    draw: &mut Draw,
    frames: usize,
    latency: usize,
) -> (Vec<Contributor>, [Vec<f32>; 2]) {
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
    contributors.sort_by(|x, y| x.route.id.cmp(&y.route.id));
    let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for (rank, contributor) in contributors.iter().enumerate() {
        let gain = math::db_to_gain_f32(contributor.route.gain_db);
        let m = &contributor.route.channel_matrix;
        let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
        let [sum_left, sum_right] = &mut sum;
        let delayed = [
            shifted(&contributor.planes[0], latency),
            shifted(&contributor.planes[1], latency),
        ];
        let frames = sum_left
            .iter_mut()
            .zip(sum_right.iter_mut())
            .zip(delayed[0].iter().zip(&delayed[1]));
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
    (contributors, sum)
}

/// Gate 1's two sessions, both declaring [`gate_one_console`]: A, three tracks that bypass every
/// console slot and are otherwise transparent, routed into a submix with strip `S`; B, one track
/// with strip `S` whose source is the test's own sum of A's routed contributions. A contributor
/// passes its source delayed by the bypassed limiter's `L`, so B's source is the D3 sum of the
/// delayed contributions: `L` leading zeros (signed as the sum makes them), then the sum. Returns
/// the two documents and each one's `(source id, planes)` feed.
#[allow(clippy::type_complexity)]
fn gate_one_sessions(
    draw: &mut Draw,
    registry: &NativeEffectRegistry,
) -> (String, Vec<(String, [Vec<f32>; 2])>, String, [Vec<f32>; 2]) {
    let rate = draw.pick(&LAUNCH_RATES);
    let console = gate_one_console();
    let strip = drawn_strip(draw, registry, &console);
    let latency = limiter_latency(rate);
    let (contributors, sum) = drawn_contributors(draw, QUANTUM * GATE_ONE_BLOCKS, latency);

    let (mut a, source, track) = empty_session_with(rate, console.clone(), GATE_ONE_BLOCKS);
    for contributor in &contributors {
        add_track(&mut a, &source, &track, &contributor.track);
        a.routes.push(contributor.route.clone());
    }
    a.submixes = vec![strip.clone().submix("bus")];
    a.routes.push(to_output("bus-main", bus_output("bus")));

    let (mut b, source, mut track) = empty_session_with(rate, console, GATE_ONE_BLOCKS);
    strip.onto(&mut track);
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
    render_counting_folds(document, feeds, blocks).0
}

/// [`render`], plus the plan's admitted route-fold lane count after the last block.
fn render_counting_folds(
    document: &str,
    feeds: &[(&str, &[Vec<f32>; 2])],
    blocks: usize,
) -> ([Vec<f32>; 2], u64) {
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
    (out, prepared.plan.bank_route_folds())
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
        let actual = render(&bus, &feeds, GATE_ONE_BLOCKS);
        let expected = render(&oracle, &[("summed", &sum)], GATE_ONE_BLOCKS);
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

const NESTED_TEST: &str = "a_bus_into_a_bus_renders_the_bits_of_two_tracks_fed_their_sums";
const NESTED_REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test \
                             submix_strip -- --exact \
                             a_bus_into_a_bus_renders_the_bits_of_two_tracks_fed_their_sums";

/// A route `id` from `source` into `destination` with a drawn gain and a drawn nonzero matrix.
fn drawn_route(
    draw: &mut Draw,
    id: &str,
    source: RouteSource,
    destination: RouteDestination,
) -> Route {
    let mut drawn = route(id, source, destination);
    drawn.gain_db = draw.in_domain(-12.0, 6.0);
    drawn.channel_matrix = ChannelMatrix {
        ll: coefficient(draw),
        lr: coefficient(draw),
        rl: coefficient(draw),
        rr: coefficient(draw),
    };
    drawn
}

fn noise_planes(draw: &mut Draw, frames: usize) -> [Vec<f32>; 2] {
    let mut plane = || {
        (0..frames)
            .map(|_| {
                let sample = draw.noise(0.9);
                if sample == 0.0 { 0.5 } else { sample }
            })
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

/// The D9 sum of `routes`, each over its own planes, by the D3 expression, added left to right in
/// route-ID order.
fn routed_sum(routes: &mut [(&Route, &[Vec<f32>; 2])], frames: usize) -> [Vec<f32>; 2] {
    routes.sort_by(|x, y| x.0.id.cmp(&y.0.id));
    let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for (rank, (route, planes)) in routes.iter().enumerate() {
        let gain = math::db_to_gain_f32(route.gain_db);
        let m = &route.channel_matrix;
        let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
        let [sum_left, sum_right] = &mut sum;
        let frames = sum_left
            .iter_mut()
            .zip(sum_right.iter_mut())
            .zip(planes[0].iter().zip(&planes[1]));
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
    sum
}

/// `strip` on one track `summed` that reads `sum` and feeds the output: a render with no submix.
fn track_fed(rate: u32, strip: Strip, sum: &[Vec<f32>; 2]) -> [Vec<f32>; 2] {
    let (mut model, source, mut track) = empty_session(rate);
    strip.onto(&mut track);
    add_track(&mut model, &source, &track, "summed");
    model
        .routes
        .push(to_output("summed-main", post_pan("summed")));
    render(&document(&model), &[("summed", sum)], BLOCKS)
}

/// #1200 verdict MINOR-3: a nested bus. `t0`..`t2` route into `bus1` (drawn strip `S1`), `bus1`'s
/// `post_pan` and `t3` route into `bus2` (drawn strip `S2`), and `bus2` feeds the output; the two
/// route IDs into `bus2` are drawn in either order. The oracle never renders a submix: `S1` on a
/// track fed the first sum, then `S2` on a track fed the D9 sum of that render and `t3`.
///
/// Red if a bus whose input is another bus's output reads that bus at a stage other than its end,
/// is scheduled or banked before its source bus finishes, or sums its two inputs out of route-ID
/// order. Gate 1 and the tap gates route a bus only to the output, so none of them sees a defect
/// that depends on the destination being a bus.
#[test]
fn a_bus_into_a_bus_renders_the_bits_of_two_tracks_fed_their_sums() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let frames = QUANTUM * BLOCKS;
    let ran = run_seeds(NESTED_TEST, NESTED_REPLAY, 16, |seed| {
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&LAUNCH_RATES);
        let s1 = drawn_strip(&mut draw, &registry, &no_console());
        let s2 = drawn_strip(&mut draw, &registry, &no_console());
        let mut names = vec!["route-a", "route-b", "route-c"];
        let first: Vec<(String, Route, [Vec<f32>; 2])> = (0..3)
            .map(|index| {
                let route_id = names.remove(draw.below(names.len()));
                let track = format!("t{index}");
                let drawn = drawn_route(&mut draw, route_id, post_pan(&track), into_bus("bus1"));
                (track, drawn, noise_planes(&mut draw, frames))
            })
            .collect();
        let (bus_route, t3_route) = if draw.chance(1, 2) {
            ("x-bus", "y-t3")
        } else {
            ("y-bus", "x-t3")
        };
        let bus_into_bus = drawn_route(&mut draw, bus_route, bus_output("bus1"), into_bus("bus2"));
        let t3_into_bus = drawn_route(&mut draw, t3_route, post_pan("t3"), into_bus("bus2"));
        let t3_planes = noise_planes(&mut draw, frames);

        let (mut nested, source, track) = empty_session(rate);
        for (id, route, _) in &first {
            add_track(&mut nested, &source, &track, id);
            nested.routes.push(route.clone());
        }
        add_track(&mut nested, &source, &track, "t3");
        nested.routes.push(bus_into_bus.clone());
        nested.routes.push(t3_into_bus.clone());
        nested.submixes = vec![s1.clone().submix("bus1"), s2.clone().submix("bus2")];
        nested
            .routes
            .push(to_output("bus-main", bus_output("bus2")));

        let mut into_first: Vec<(&Route, &[Vec<f32>; 2])> = first
            .iter()
            .map(|(_, route, planes)| (route, planes))
            .collect();
        let bus1 = track_fed(rate, s1, &routed_sum(&mut into_first, frames));
        let mut into_second = vec![(&bus_into_bus, &bus1), (&t3_into_bus, &t3_planes)];
        let expected = track_fed(rate, s2, &routed_sum(&mut into_second, frames));

        let mut feeds: Vec<(&str, &[Vec<f32>; 2])> = first
            .iter()
            .map(|(id, _, planes)| (id.as_str(), planes))
            .collect();
        feeds.push(("t3", &t3_planes));
        let actual = render(&document(&nested), &feeds, BLOCKS);
        for plane in 0..2 {
            assert_bits_equal(
                &actual[plane],
                &expected[plane],
                &format!("seed {seed}: plane {plane}: nested bus against two tracks"),
            );
        }
        assert!(
            actual.iter().flatten().any(|sample| *sample != 0.0),
            "seed {seed}: the nested bus rendered audio"
        );
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 16);
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
    bare.submixes = vec![Submix::unity(sid("bus"), &bare.console)];
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
    on_bus.submixes = vec![Submix::unity(sid("bus"), &on_bus.console)];
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
    let mut bus = Strip::transparent(&model.console);
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

/// The graph artifact of `document`, through the public pipeline host-core uses (effects, builtins,
/// `compile_with_builtins`): `PreparedHost` exposes no inserted delays.
fn graph_artifact(document: &str, plan_id: u64) -> PreparedGraphBuiltinsArtifact {
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
        plan_id,
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

#[test]
fn a_bus_limiters_latency_is_compensated_on_the_direct_edge() {
    let model = pdc_session();
    let document = document(&model);
    let artifact = graph_artifact(&document, 1_200);
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

// ---- #1202 gate 3 --------------------------------------------------------------------------

/// A bypassed bus console slot keeps its latency (decision 12, L4). The session declares one
/// `post_insert` limiter; every strip carries it bypassed. `t0` and `t1` feed the bus and `t0` also
/// feeds the output directly, so its own path arrives at 486 samples (48 kHz) and the bus path at
/// 972: PDC delays the direct edge by exactly 486, and an impulse on `t0` arrives once, summed,
/// at sample 972 on both planes.
#[test]
fn a_bypassed_bus_console_slot_keeps_its_latency() {
    let console = Console {
        pre_insert: Vec::new(),
        post_insert: vec![ConsoleSlot {
            slot: sid("desk-limit"),
            identity: EffectIdentity::Native {
                effect_id: sid("miso.true-peak-limiter"),
            },
            quality: EffectQuality::Normal,
            link_mode: LinkMode::Maximum,
        }],
    };
    let (mut model, source, track) = empty_session_with(48_000, console, BLOCKS);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
        model
            .routes
            .push(route(&format!("{id}-bus"), post_pan(id), into_bus("bus")));
    }
    model.routes.push(to_output("t0-direct", post_pan("t0")));
    model.routes.push(to_output("bus-main", bus_output("bus")));
    model.submixes = vec![Strip::transparent(&model.console).submix("bus")];
    assert!(
        model
            .tracks
            .iter()
            .map(|track| &track.console)
            .chain([&model.submixes[0].console])
            .all(|entries| entries.len() == 1 && entries[0].bypass),
        "every strip carries the limiter, bypassed"
    );
    let document = document(&model);

    let artifact = graph_artifact(&document, 1_202);
    assert_eq!(artifact.report().output_latency, LatencySamples(972));
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
        assert_eq!(peak(samples), 972, "plane {plane} peaks at sample 972");
        assert_eq!(
            samples[972].to_bits(),
            0.5_f32.to_bits(),
            "plane {plane}: both bypassed paths arrive together, each unchanged"
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
    let mut bus = Strip::transparent(&model.console);
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

// ---- Gate 8 -------------------------------------------------------------------------------

#[test]
fn a_processed_bus_renders_without_allocating() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut draw = Draw::new(8);
    let (bus, feeds, _, _) = gate_one_sessions(&mut draw, &registry);
    assert_renders_without_allocating(&bus, &feeds);
}

/// After block 0, every render of `document` allocates and frees nothing: the render audit and
/// `bench_support::alloc`'s thread-scoped counters both read exact zero around each call.
fn assert_renders_without_allocating(document: &str, feeds: &[(String, [Vec<f32>; 2])]) {
    let compiled = compile_host_session(document, &caps()).expect("compile");
    let prepared = prepare_host_runtime(&compiled, &caps()).expect("prepare");
    let rate = prepared.report.sample_rate_hz;
    let (mut session, mut sources, _) = prepared
        .start_render_session()
        .unwrap_or_else(|_| panic!("render session starts"));
    audit::warm_up();
    bench_alloc::assert_installed();
    for block in 0..BLOCKS {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        for (id, planes) in feeds {
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

// ---- #1201 --------------------------------------------------------------------------------

/// Two transparent tracks `t0` and `t1` routed `post_pan` into a transparent bus whose lanes
/// declare `delay` samples, at 48 kHz.
fn delayed_bus_session(delay: [u32; 2]) -> String {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
        model
            .routes
            .push(route(&format!("{id}-bus"), post_pan(id), into_bus("bus")));
    }
    model.routes.push(to_output("bus-main", bus_output("bus")));
    let mut bus = Strip::transparent(&model.console);
    bus.builtins.left.delay_samples = delay[0];
    bus.builtins.right.delay_samples = delay[1];
    model.submixes = vec![bus.submix("bus")];
    document(&model)
}

/// Gate 1's feeds: an impulse into each contributor, at different samples and levels, on both
/// planes.
fn impulse_feeds() -> Vec<(String, [Vec<f32>; 2])> {
    [("t0", 3, 0.25_f32), ("t1", 10, 0.5)]
        .into_iter()
        .map(|(id, at, level)| {
            let mut plane = vec![0.0_f32; QUANTUM * BLOCKS];
            plane[at] = level;
            (id.to_owned(), [plane.clone(), plane])
        })
        .collect()
}

fn borrowed(feeds: &[(String, [Vec<f32>; 2])]) -> Vec<(&str, &[Vec<f32>; 2])> {
    feeds
        .iter()
        .map(|(id, planes)| (id.as_str(), planes))
        .collect()
}

fn peak(samples: &[f32]) -> usize {
    samples
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.abs().total_cmp(&y.1.abs()))
        .map(|(index, _)| index)
        .expect("rendered samples")
}

/// `plane` shifted later by `delay` samples, with `+0.0` shifted in.
fn shifted(plane: &[f32], delay: usize) -> Vec<f32> {
    core::iter::repeat_n(0.0_f32, delay)
        .chain(plane.iter().copied())
        .take(plane.len())
        .collect()
}

fn assert_bits_equal(actual: &[f32], expected: &[f32], what: &str) {
    if let Some(index) = first_difference(actual, expected) {
        panic!(
            "{what}: sample {index}: {:?} != {:?}",
            actual[index], expected[index]
        );
    }
}

#[test]
fn a_bus_delay_shifts_its_summed_input_and_pdc_never_sees_it() {
    const DELAY: usize = 37;
    let delayed = delayed_bus_session([DELAY as u32, 0]);
    let undelayed = delayed_bus_session([0, 0]);
    let feeds = impulse_feeds();
    let feeds = borrowed(&feeds);
    let out = render(&delayed, &feeds, BLOCKS);
    let reference = render(&undelayed, &feeds, BLOCKS);
    assert!(
        reference[0].iter().filter(|sample| **sample != 0.0).count() == 2,
        "both contributors reach the undelayed bus: {:?}",
        reference[0]
            .iter()
            .enumerate()
            .filter(|(_, sample)| **sample != 0.0)
            .collect::<Vec<_>>()
    );
    // The whole sum moves, both contributors together, by exactly the declared samples.
    assert_bits_equal(
        &out[0],
        &shifted(&reference[0], DELAY),
        "the delayed left plane against the undelayed left, shifted",
    );
    assert_bits_equal(&out[1], &reference[1], "the undelayed right plane");
    assert_eq!(peak(&out[0]), peak(&out[1]) + DELAY);

    // Not latency: PDC inserts nothing for it and the output latency does not move.
    let delayed = graph_artifact(&delayed, 1_201);
    let undelayed = graph_artifact(&undelayed, 1_202);
    assert_eq!(
        delayed.report().output_latency,
        undelayed.report().output_latency
    );
    assert_eq!(
        delayed.graph().inserted_delays,
        undelayed.graph().inserted_delays
    );
}

#[test]
fn a_folded_bus_still_delays_its_sum() {
    const DELAY: usize = 7;
    let session = |delay: u32| {
        let (mut model, source, track) = empty_session(48_000);
        for index in 0..8 {
            let id = format!("t{index}");
            add_track(&mut model, &source, &track, &id);
            model
                .routes
                .push(route(&format!("{id}-bus"), post_pan(&id), into_bus("bus")));
        }
        model.routes.push(to_output("bus-main", bus_output("bus")));
        let mut bus = Strip::transparent(&model.console);
        bus.builtins.left.delay_samples = delay;
        bus.builtins.right.delay_samples = delay;
        model.submixes = vec![bus.submix("bus")];
        document(&model)
    };
    let mut draw = Draw::new(1_201);
    let feeds: Vec<(String, [Vec<f32>; 2])> = (0..8)
        .map(|index| {
            let mut plane = || {
                (0..QUANTUM * BLOCKS)
                    .map(|_| draw.noise(0.9))
                    .collect::<Vec<f32>>()
            };
            (format!("t{index}"), [plane(), plane()])
        })
        .collect();
    let feeds = borrowed(&feeds);
    let (delayed, folds) = render_counting_folds(&session(DELAY as u32), &feeds, BLOCKS);
    let (undelayed, undelayed_folds) = render_counting_folds(&session(0), &feeds, BLOCKS);
    assert_eq!(folds, undelayed_folds, "the delay costs the bus no fold");
    assert!(
        folds > 0,
        "the bus must be a fold master, or this is vacuous"
    );
    graph::test_only_set_route_fold_declined(true);
    let (declined, declined_folds) = render_counting_folds(&session(DELAY as u32), &feeds, BLOCKS);
    graph::test_only_set_route_fold_declined(false);
    assert_eq!(declined_folds, 0, "the declined arm must not fold");
    for plane in 0..2 {
        assert_bits_equal(
            &delayed[plane],
            &declined[plane],
            &format!("plane {plane}: the folded bus against the reduction"),
        );
        assert_bits_equal(
            &delayed[plane],
            &shifted(&undelayed[plane], DELAY),
            &format!("plane {plane}: the delayed bus against the undelayed, shifted"),
        );
    }
    assert!(
        delayed.iter().flatten().any(|sample| *sample != 0.0),
        "the folded bus rendered audio"
    );
}

#[test]
fn a_delayed_bus_renders_without_allocating() {
    assert_renders_without_allocating(&delayed_bus_session([37, 0]), &impulse_feeds());
}

/// `(sample, bits)` of every nonzero sample of `plane`.
fn nonzero(plane: &[f32]) -> Vec<(usize, u32)> {
    plane
        .iter()
        .enumerate()
        .filter(|(_, sample)| **sample != 0.0)
        .map(|(index, sample)| (index, sample.to_bits()))
        .collect()
}

/// #1201 verdict MINOR-1: a delayed track into a delayed bus, beside a second delayed bus fed by
/// an undelayed track. Every line is its own, and a track's delay and its bus's delay add.
///
/// Red if delay lines alias across the `TrackDelay` and `SumDelay` arms or across two buses (two
/// arms now allocate from one line vector), or if a track's delay and its bus's delay fail to add.
/// No other test composes two delays or delays two buses.
#[test]
fn delayed_tracks_into_two_delayed_buses_each_keep_their_own_line() {
    let build = |t0: [u32; 2], bus: [u32; 2], bus2: [u32; 2]| {
        let (mut model, source, track) = empty_session(48_000);
        add_track(&mut model, &source, &track, "t0");
        add_track(&mut model, &source, &track, "t1");
        model.tracks[0].builtins.left.delay_samples = t0[0];
        model.tracks[0].builtins.right.delay_samples = t0[1];
        model
            .routes
            .push(route("t0-bus", post_pan("t0"), into_bus("bus")));
        model
            .routes
            .push(route("t1-bus2", post_pan("t1"), into_bus("bus2")));
        model.routes.push(to_output("bus-main", bus_output("bus")));
        model
            .routes
            .push(to_output("bus2-main", bus_output("bus2")));
        let mut first = Strip::transparent(&model.console);
        first.builtins.left.delay_samples = bus[0];
        first.builtins.right.delay_samples = bus[1];
        let mut second = Strip::transparent(&model.console);
        second.builtins.left.delay_samples = bus2[0];
        second.builtins.right.delay_samples = bus2[1];
        model.submixes = vec![first.submix("bus"), second.submix("bus2")];
        document(&model)
    };
    // t0: 0.25 at sample 3; t1: 0.5 at sample 10; both planes.
    let feeds = impulse_feeds();
    let feeds = borrowed(&feeds);
    let reference = render(&build([0, 0], [0, 0], [0, 0]), &feeds, BLOCKS);
    let out = render(&build([11, 0], [37, 5], [13, 0]), &feeds, BLOCKS);
    let word = |plane: &[f32], at: usize| plane[at].to_bits();
    // Left: t1 through bus2 at 10 + 13; t0 through its own 11 and the bus's 37 at 3 + 48.
    assert_eq!(
        nonzero(&out[0]),
        vec![(23, word(&reference[0], 10)), (51, word(&reference[0], 3))]
    );
    // Right: t0 at 3 + 0 + 5; t1 at 10 + 0.
    assert_eq!(
        nonzero(&out[1]),
        vec![(8, word(&reference[1], 3)), (10, word(&reference[1], 10))]
    );
}

// ---- #1203 --------------------------------------------------------------------------------

const TAP_TEST: &str = "a_bus_tap_renders_the_bits_of_the_same_tap_on_a_track_fed_its_sum";
const TAP_REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test \
                          submix_strip -- --exact \
                          a_bus_tap_renders_the_bits_of_the_same_tap_on_a_track_fed_its_sum";
const TAP_SEEDS: u64 = 8;

/// The seven taps in chain order.
const TAPS: [SendTap; 7] = [
    SendTap::Input,
    SendTap::PostInput,
    SendTap::InsertSend,
    SendTap::InsertReturn,
    SendTap::PreFader,
    SendTap::PostFader,
    SendTap::PostPan,
];

fn eq_param(parameter_id: u32, unit: ParameterUnit, value: f32) -> EffectParam {
    EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    }
}

/// Band 1 of the parametric EQ enabled as a bell at `hz` with `gain_db`.
fn bell(hz: f32, gain_db: f32) -> Vec<EffectParam> {
    vec![
        eq_param(1, ParameterUnit::Linear, 1.0),
        eq_param(3, ParameterUnit::Hz, hz),
        eq_param(4, ParameterUnit::Db, gain_db),
    ]
}

/// The tap gates' console: one latency-free EQ slot in each section, so that a live entry changes
/// the signal between `post_input` and `insert_send`, and again between `insert_return` and
/// `pre_fader`.
fn tap_console() -> Console {
    let slot = |id: &str| ConsoleSlot {
        slot: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.parametric-eq"),
        },
        quality: EffectQuality::Normal,
        link_mode: LinkMode::DualMono,
    };
    Console {
        pre_insert: vec![slot("desk-eq")],
        post_insert: vec![slot("desk-tone")],
    }
}

/// A strip on which every tap differs from the one before it: a delay of 37 samples left and 5
/// right (#1201: a bus delays its summed input, so it shows at every tap from `input` on), a trim,
/// a live console EQ in each section, an insert compressor (defaults: threshold -18 dB), a -6 dB
/// fader and a swap matrix.
fn tap_strip() -> Strip {
    let mut strip = Strip::transparent(&tap_console());
    strip.builtins.left.delay_samples = 37;
    strip.builtins.right.delay_samples = 5;
    strip.builtins.left.trim_db = 3.0;
    strip.builtins.right.trim_db = -2.0;
    strip.console = vec![
        ConsoleEntry {
            slot: sid("desk-eq"),
            bypass: false,
            params: bell(1_000.0, 6.0),
        },
        ConsoleEntry {
            slot: sid("desk-tone"),
            bypass: false,
            params: bell(3_000.0, -4.0),
        },
    ];
    strip.inserts.effects = vec![Effect {
        id: sid("glue"),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }];
    strip.fader.left_db = -6.0;
    strip.fader.right_db = -6.0;
    strip.matrix_or_pan = MatrixOrPan::Matrix {
        ll: 0.0,
        lr: 1.0,
        rl: 1.0,
        rr: 0.0,
        smoothing_samples: 0,
    };
    strip
}

/// The bus session of the tap gates: three contributors that bypass every console slot, routed
/// into `bus` with [`tap_strip`], and nothing else. Returns it with its feeds and the sum the
/// contributors deliver to the bus (no shift: every slot is latency-free).
#[allow(clippy::type_complexity)]
fn tap_bus_session(draw: &mut Draw) -> (SessionModel, Vec<(String, [Vec<f32>; 2])>, [Vec<f32>; 2]) {
    let rate = draw.pick(&LAUNCH_RATES);
    let (contributors, sum) = drawn_contributors(draw, QUANTUM * BLOCKS, 0);
    let (mut model, source, track) = empty_session_with(rate, tap_console(), BLOCKS);
    for contributor in &contributors {
        add_track(&mut model, &source, &track, &contributor.track);
        model.routes.push(contributor.route.clone());
    }
    model.submixes = vec![tap_strip().submix("bus")];
    let feeds = contributors
        .into_iter()
        .map(|contributor| (contributor.track, contributor.planes))
        .collect();
    (model, feeds, sum)
}

/// The oracle of the tap gates: `model` with the bus and its contributors replaced by one track
/// `bus` that has the bus's strip and reads the test's own sum.
fn bus_as_track(model: &SessionModel) -> SessionModel {
    let (mut oracle, source, mut track) =
        empty_session_with(model.sample_rate_hz, tap_console(), BLOCKS);
    tap_strip().onto(&mut track);
    add_track(&mut oracle, &source, &track, "bus");
    oracle
}

fn bus_tap(tap: SendTap) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid("bus"),
        tap,
    }
}

fn track_tap(track: &str, tap: SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap,
    }
}

/// Gate 1: for each tap `T`, a bus routed to the output from `T` renders the bits of the same
/// route from `T` of a track that has the bus's strip and is fed the bus's sum. The oracle's seven
/// renders are pairwise different, so a tap mapped to any other stage changes the bits.
#[test]
fn a_bus_tap_renders_the_bits_of_the_same_tap_on_a_track_fed_its_sum() {
    let ran = run_seeds(TAP_TEST, TAP_REPLAY, TAP_SEEDS, |seed| {
        let mut draw = Draw::new(seed);
        let (bus, feeds, sum) = tap_bus_session(&mut draw);
        let oracle = bus_as_track(&bus);
        let mut renders: Vec<[Vec<f32>; 2]> = Vec::new();
        for tap in TAPS {
            let mut a = bus.clone();
            a.routes.push(to_output("bus-tap", bus_tap(tap)));
            let mut b = oracle.clone();
            b.routes.push(to_output("bus-tap", track_tap("bus", tap)));
            let actual = render(&document(&a), &borrowed(&feeds), BLOCKS);
            let expected = render(&document(&b), &[("bus", &sum)], BLOCKS);
            for plane in 0..2 {
                assert_bits_equal(
                    &actual[plane],
                    &expected[plane],
                    &format!("seed {seed}: tap {tap:?} plane {plane}"),
                );
            }
            renders.push(expected);
        }
        for (x, first) in renders.iter().enumerate() {
            for (y, second) in renders.iter().enumerate().skip(x + 1) {
                assert!(
                    first != second,
                    "seed {seed}: taps {:?} and {:?} render the same bits",
                    TAPS[x],
                    TAPS[y]
                );
            }
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, TAP_SEEDS);
    }
}

/// Gate 2: a compressor insert on track `x`, keyed from the bus's `pre_fader`, renders the bits of
/// the same session in which the bus is a track fed its sum. `x` is quiet (below the threshold), so
/// its gain reduction is the key's; keying the oracle from `post_pan` instead changes the bits, so
/// a sidechain that reads the bus's end, or any stage other than `pre_fader`, turns this red.
#[test]
fn a_sidechain_keyed_by_a_bus_tap_reads_that_tap() {
    let mut draw = Draw::new(1_203);
    let (mut bus, mut feeds, sum) = tap_bus_session(&mut draw);
    let mut quiet = || {
        (0..QUANTUM * BLOCKS)
            .map(|_| draw.noise(0.05))
            .collect::<Vec<f32>>()
    };
    let x_planes = [quiet(), quiet()];
    let ducked = |model: &mut SessionModel, key: RouteSource| {
        let source = model.sources[0].clone();
        let mut track = model.tracks[0].clone();
        Strip::transparent(&model.console).onto(&mut track);
        track.inserts.effects = vec![Effect {
            id: sid("duck"),
            identity: EffectIdentity::Native {
                effect_id: sid("miso.compressor"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::Maximum,
            params: Vec::new(),
            sidechain: SidechainDeclaration::Routed(session::Sidechain {
                source: key,
                port_id: sid("sidechain-in"),
            }),
        }];
        add_track(model, &source, &track, "x");
        model.routes.push(to_output("x-main", post_pan("x")));
    };
    let mut oracle = bus_as_track(&bus);
    ducked(&mut bus, bus_tap(SendTap::PreFader));
    bus.routes
        .push(to_output("bus-main", bus_tap(SendTap::PostPan)));
    let mut wrong_key = oracle.clone();
    ducked(&mut oracle, track_tap("bus", SendTap::PreFader));
    oracle
        .routes
        .push(to_output("bus-main", track_tap("bus", SendTap::PostPan)));
    ducked(&mut wrong_key, track_tap("bus", SendTap::PostPan));
    wrong_key
        .routes
        .push(to_output("bus-main", track_tap("bus", SendTap::PostPan)));

    feeds.push(("x".to_owned(), x_planes.clone()));
    let actual = render(&document(&bus), &borrowed(&feeds), BLOCKS);
    let oracle_feeds = [("bus", &sum), ("x", &x_planes)];
    let expected = render(&document(&oracle), &oracle_feeds, BLOCKS);
    for plane in 0..2 {
        assert_bits_equal(&actual[plane], &expected[plane], &format!("plane {plane}"));
    }
    let end_keyed = render(&document(&wrong_key), &oracle_feeds, BLOCKS);
    assert!(
        end_keyed != expected,
        "keying from the bus's end must change the bits, or this gate cannot see the tap"
    );
}

/// Gate 3: the bus's `post_insert` limiter (486 samples at 48 kHz) sits between its
/// `insert_return` and `post_pan` taps. Every strip carries the slot (the contributors bypassed,
/// keeping its latency), so the bus input arrives at 486; the `post_pan` route leaves at 972 and
/// the `insert_return` route at 486, and PDC delays exactly that route's edge by 486. An impulse
/// on `t0` arrives once, summed over both routes, at sample 972 on both planes.
#[test]
fn a_tap_before_a_bus_limiter_is_compensated_by_the_limiters_latency() {
    let console = Console {
        pre_insert: Vec::new(),
        post_insert: vec![ConsoleSlot {
            slot: sid("desk-limit"),
            identity: EffectIdentity::Native {
                effect_id: sid("miso.true-peak-limiter"),
            },
            quality: EffectQuality::Normal,
            link_mode: LinkMode::Maximum,
        }],
    };
    let (mut model, source, track) = empty_session_with(48_000, console, BLOCKS);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
        model
            .routes
            .push(route(&format!("{id}-bus"), post_pan(id), into_bus("bus")));
    }
    model
        .routes
        .push(to_output("bus-return", bus_tap(SendTap::InsertReturn)));
    model.routes.push(to_output("bus-main", bus_output("bus")));
    let mut bus = Strip::transparent(&model.console);
    bus.console[0].bypass = false;
    model.submixes = vec![bus.submix("bus")];
    let document = document(&model);

    let artifact = graph_artifact(&document, 1_203);
    assert_eq!(artifact.report().output_latency, LatencySamples(972));
    let delays = &artifact.graph().inserted_delays;
    assert_eq!(
        delays.len(),
        1,
        "exactly the insert-return edge is delayed: {delays:?}"
    );
    let early = graph::StableGraphId::parse("bus-return").expect("graph id");
    assert!(
        matches!(
            &delays[0].edge_id,
            GraphEdgeId::RouteSource { route_id } | GraphEdgeId::RouteDestination { route_id }
                if *route_id == early
        ),
        "the delay sits on the insert-return route: {delays:?}"
    );
    assert_eq!(delays[0].samples, LatencySamples(486));

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
        assert_eq!(peak(samples), 972, "plane {plane} peaks at sample 972");
        assert!(
            samples[972].abs() > 0.25,
            "plane {plane}: both routes arrive together"
        );
    }
}

/// #1203 D4 (verdict MINOR-2): a muted bus still feeds its `pre_fader` tap, as a muted track does,
/// and its `post_fader` and `post_pan` taps are silent. Each tap of the muted bus renders the bits
/// of the same tap of a muted track fed the bus's sum.
///
/// Red if a muted strip's upstream is elided or zeroed, for a bus or a track (a skip-on-silence
/// optimization that treats a muted fader as a silent strip), or if the mute stops gating the
/// post-fader taps. No other test mutes a strip that has a pre-fader send.
#[test]
fn a_muted_bus_still_feeds_its_pre_fader_tap() {
    for seed in 0..4 {
        let mut draw = Draw::new(200 + seed);
        let (mut bus, feeds, sum) = tap_bus_session(&mut draw);
        bus.submixes[0].fader.left_mute = true;
        bus.submixes[0].fader.right_mute = true;
        let mut oracle = bus_as_track(&bus);
        oracle.tracks[0].fader.left_mute = true;
        oracle.tracks[0].fader.right_mute = true;
        for tap in [SendTap::PreFader, SendTap::PostFader, SendTap::PostPan] {
            let mut a = bus.clone();
            a.routes.push(to_output("bus-tap", bus_tap(tap)));
            let mut b = oracle.clone();
            b.routes.push(to_output("bus-tap", track_tap("bus", tap)));
            let actual = render(&document(&a), &borrowed(&feeds), BLOCKS);
            let expected = render(&document(&b), &[("bus", &sum)], BLOCKS);
            for plane in 0..2 {
                assert_bits_equal(
                    &actual[plane],
                    &expected[plane],
                    &format!("seed {seed}: muted tap {tap:?} plane {plane}"),
                );
            }
            let audible = actual.iter().flatten().any(|sample| *sample != 0.0);
            assert_eq!(
                audible,
                tap == SendTap::PreFader,
                "seed {seed}: only the pre-fader tap of a muted strip is audible ({tap:?})"
            );
        }
    }
}

/// Gate 6: gate 1's bus sessions, one per tap, render without allocating.
#[test]
fn a_tapped_bus_renders_without_allocating() {
    let mut draw = Draw::new(6);
    let (bus, feeds, _) = tap_bus_session(&mut draw);
    for tap in TAPS {
        let mut model = bus.clone();
        model.routes.push(to_output("bus-tap", bus_tap(tap)));
        assert_renders_without_allocating(&document(&model), &feeds);
    }
}
