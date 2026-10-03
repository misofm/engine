//! Verifier scratch for #1200 attempt 1. Never committed.
#![allow(clippy::all, clippy::pedantic, unused)]

use dsp_reference::randomized::{Draw, first_difference};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{NativeEffectRegistry, ParameterChannelPolicy, ParameterDomain};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use graph_compiler::Backend;
use session::{
    ChannelBuiltins, ChannelMatrix, DualMonoBuiltins, DualMonoFader, Effect, EffectIdentity,
    EffectParam, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel, ParameterUnit, Rack,
    Route, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source,
    StableId, Submix, canonical_session_json, parse_session_json,
};

use crate::prepare::{compile_host_session, prepare_host_runtime_with_live_controls_backend};
use crate::{HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

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
    ChannelMatrix { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 }
}

fn route(id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route { id: sid(id), source, destination, channel_matrix: unity_matrix(), gain_db: 0.0 }
}

fn to_output(id: &str, source: RouteSource) -> Route {
    route(id, source, RouteDestination::OutputInput { output_id: sid("main-out") })
}

fn post_pan(track: &str) -> RouteSource {
    RouteSource::Track { track_id: sid(track), tap: SendTap::PostPan }
}

fn bus_output(bus: &str) -> RouteSource {
    RouteSource::SubmixOutput { submix_id: sid(bus) }
}

fn into_bus(bus: &str) -> RouteDestination {
    RouteDestination::SubmixInput { submix_id: sid(bus) }
}

#[derive(Clone)]
struct Strip {
    builtins: DualMonoBuiltins,
    inserts: Rack,
    fader: DualMonoFader,
    matrix_or_pan: MatrixOrPan,
}

impl Strip {
    fn transparent() -> Self {
        let Submix { builtins, inserts, fader, matrix_or_pan, .. } = Submix::unity(sid("unused"));
        Self { builtins, inserts, fader, matrix_or_pan }
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
    fn onto(self, track: &mut session::Track) {
        track.builtins = self.builtins;
        track.inserts = self.inserts;
        track.fader = self.fader;
        track.matrix_or_pan = self.matrix_or_pan;
    }
}

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
    Strip::transparent().onto(&mut track);
    (model, source, track)
}

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
    if effect_contract::parameter_value_valid(parameter, value) { value } else { parameter.default_value }
}

fn drawn_effect(draw: &mut Draw, registry: &NativeEffectRegistry, kind: &str, id: &str, link_mode: LinkMode) -> Effect {
    let descriptor = registry.get_ascii(kind).expect("launch effect").descriptor();
    let mut params = Vec::new();
    for parameter in descriptor.parameters {
        if parameter.channel_policy == ParameterChannelPolicy::PerLane && draw.chance(1, 2) {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                params.push(EffectParam { parameter_id: parameter.id.0, channel, unit: unit(parameter.unit), value: value(draw, parameter) });
            }
        } else {
            params.push(EffectParam { parameter_id: parameter.id.0, channel: ParameterChannel::Both, unit: unit(parameter.unit), value: value(draw, parameter) });
        }
    }
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native { effect_id: sid(kind) },
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
        hpf_hz: if draw.chance(1, 2) { 0.0 } else { draw.in_domain(20.0, 400.0) },
        lpf_hz: if draw.chance(1, 2) { 0.0 } else { draw.in_domain(2_000.0, 16_000.0) },
        delay_samples: 0,
    }
}

fn drawn_strip(draw: &mut Draw, registry: &NativeEffectRegistry) -> Strip {
    Strip {
        builtins: DualMonoBuiltins { left: drawn_lane(draw), right: drawn_lane(draw) },
        inserts: Rack {
            effects: vec![
                drawn_effect(draw, registry, "miso.compressor", "glue", LinkMode::Maximum),
                drawn_effect(draw, registry, "miso.parametric-eq", "tone", LinkMode::DualMono),
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

fn coefficient(draw: &mut Draw) -> f32 {
    let magnitude = 0.1 + 0.9 * draw.unit();
    if draw.chance(1, 2) { -magnitude } else { magnitude }
}

fn drawn_route(draw: &mut Draw, id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    let mut r = route(id, source, destination);
    r.gain_db = draw.in_domain(-12.0, 6.0);
    r.channel_matrix = ChannelMatrix { ll: coefficient(draw), lr: coefficient(draw), rl: coefficient(draw), rr: coefficient(draw) };
    r
}

fn noise_planes(draw: &mut Draw, frames: usize) -> [Vec<f32>; 2] {
    let mut plane = || (0..frames).map(|_| { let s = draw.noise(0.9); if s == 0.0 { 0.5 } else { s } }).collect::<Vec<f32>>();
    [plane(), plane()]
}

/// The D3 contribution of one route, added into `sum` (first contribution assigns).
fn accumulate(sum: &mut [Vec<f32>; 2], route: &Route, planes: &[Vec<f32>; 2], first: bool) {
    let gain = math::db_to_gain_f32(route.gain_db);
    let m = &route.channel_matrix;
    let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
    let [sl, sr] = sum;
    for i in 0..sl.len() {
        let (l, r) = (planes[0][i], planes[1][i]);
        let left = (lr * r) + (ll * l);
        let right = (rr * r) + (rl * l);
        if first { sl[i] = left; sr[i] = right; } else { sl[i] += left; sr[i] += right; }
    }
}

fn render_at(document: &str, feeds: &[(&str, &[Vec<f32>; 2])], blocks: usize, backend: Backend) -> [Vec<f32>; 2] {
    let compiled = compile_host_session(document, &caps())
        .unwrap_or_else(|f| panic!("compile: {}", String::from_utf8_lossy(f.as_bytes())));
    let (mut prepared, _) = prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &HostLiveControlRequest::default(), backend)
        .unwrap_or_else(|f| panic!("prepare: {}", String::from_utf8_lossy(f.as_bytes())));
    let rate = prepared.report.sample_rate_hz;
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..blocks {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        for (id, planes) in feeds {
            prepared.sources.submit(id.as_bytes(), SourceSubmission {
                generation: 1,
                start_frame: range.start as u64,
                sample_rate_hz: rate,
                planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                frames: QUANTUM as u32,
                end_of_region: false,
            }).expect("source block");
        }
        let mut samples = [0.0_f32; QUANTUM * 2];
        prepared.plan.render(
            RenderIo { output: PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("planes") },
            RenderTime { absolute_sample: (block * QUANTUM) as u64 },
        ).expect("render");
        out[0].extend_from_slice(&samples[..QUANTUM]);
        out[1].extend_from_slice(&samples[QUANTUM..]);
    }
    out
}

fn backends() -> Vec<Backend> {
    vec![Backend::current(), Backend::Simd4, Backend::Scalar]
}

fn assert_same(label: &str, a: &[Vec<f32>; 2], b: &[Vec<f32>; 2]) {
    for plane in 0..2 {
        if let Some(i) = first_difference(&a[plane], &b[plane]) {
            panic!("{label}: plane {plane} sample {i}: {:?} != {:?}", a[plane][i], b[plane][i]);
        }
    }
}

/// Gate-1 shape at every width this build can render: W8 (current), W4, Scalar.
#[test]
fn v_gate_one_at_every_width() {
    let registry = launch_native_effect_registry().expect("registry");
    let mut rates = std::collections::BTreeMap::new();
    for seed in 0..32_u64 {
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&LAUNCH_RATES);
        *rates.entry(rate).or_insert(0) += 1;
        let strip = drawn_strip(&mut draw, &registry);
        let frames = QUANTUM * BLOCKS;
        let mut names = vec!["route-a", "route-b", "route-c"];
        let mut contributors: Vec<(String, Route, [Vec<f32>; 2])> = (0..3)
            .map(|index| {
                let route_id = names.remove(draw.below(names.len()));
                let track = format!("t{index}");
                let r = drawn_route(&mut draw, route_id, post_pan(&track), into_bus("bus"));
                let planes = noise_planes(&mut draw, frames);
                (track, r, planes)
            })
            .collect();
        let (mut a, source, track) = empty_session(rate);
        for (t, r, _) in &contributors {
            add_track(&mut a, &source, &track, t);
            a.routes.push(r.clone());
        }
        a.submixes = vec![strip.clone().submix("bus")];
        a.routes.push(to_output("bus-main", bus_output("bus")));
        contributors.sort_by(|x, y| x.1.id.cmp(&y.1.id));
        let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
        for (rank, (_, r, planes)) in contributors.iter().enumerate() {
            accumulate(&mut sum, r, planes, rank == 0);
        }
        let (mut b, source, mut track) = empty_session(rate);
        strip.onto(&mut track);
        add_track(&mut b, &source, &track, "summed");
        b.routes.push(to_output("summed-main", post_pan("summed")));
        let feeds: Vec<(&str, &[Vec<f32>; 2])> = contributors.iter().map(|(t, _, p)| (t.as_str(), p)).collect();
        let expected = render_at(&document(&b), &[("summed", &sum)], BLOCKS, Backend::current());
        for backend in backends() {
            let actual = render_at(&document(&a), &feeds, BLOCKS, backend);
            assert_same(&format!("seed {seed} {backend:?}"), &actual, &expected);
        }
    }
    eprintln!("v_gate_one_at_every_width rates: {rates:?}");
}

/// Nested: t0..t2 -> bus1 (S1) -> bus2 (S2) -> output, t3 -> bus2 directly. The oracle renders
/// S1 on a track fed its sum, then S2 on a track fed the D9 sum of the (bus1 -> bus2) route over
/// that output and t3's route; no submix appears in any oracle session.
#[test]
fn v_nested_bus_into_bus_equals_two_tracks_fed_their_sums() {
    let registry = launch_native_effect_registry().expect("registry");
    for seed in 100..124_u64 {
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&LAUNCH_RATES);
        let s1 = drawn_strip(&mut draw, &registry);
        let s2 = drawn_strip(&mut draw, &registry);
        let frames = QUANTUM * BLOCKS;
        let mut names = vec!["r-a", "r-b", "r-c"];
        let mut level1: Vec<(String, Route, [Vec<f32>; 2])> = (0..3)
            .map(|index| {
                let route_id = names.remove(draw.below(names.len()));
                let track = format!("t{index}");
                let r = drawn_route(&mut draw, route_id, post_pan(&track), into_bus("bus1"));
                (track, r, noise_planes(&mut draw, frames))
            })
            .collect();
        // Two routes into bus2; IDs permuted so route-ID order is random.
        let (bus_route_id, t3_route_id) = if draw.chance(1, 2) { ("x-bus", "y-t3") } else { ("y-bus", "x-t3") };
        let bb = drawn_route(&mut draw, bus_route_id, bus_output("bus1"), into_bus("bus2"));
        let t3r = drawn_route(&mut draw, t3_route_id, post_pan("t3"), into_bus("bus2"));
        let t3_planes = noise_planes(&mut draw, frames);

        let (mut a, source, track) = empty_session(rate);
        for (t, r, _) in &level1 {
            add_track(&mut a, &source, &track, t);
            a.routes.push(r.clone());
        }
        add_track(&mut a, &source, &track, "t3");
        a.routes.push(bb.clone());
        a.routes.push(t3r.clone());
        a.submixes = vec![s1.clone().submix("bus1"), s2.clone().submix("bus2")];
        a.routes.push(to_output("bus-main", bus_output("bus2")));

        level1.sort_by(|x, y| x.1.id.cmp(&y.1.id));
        let mut sum1 = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
        for (rank, (_, r, planes)) in level1.iter().enumerate() {
            accumulate(&mut sum1, r, planes, rank == 0);
        }
        let (mut b1, source, mut track1) = empty_session(rate);
        s1.onto(&mut track1);
        add_track(&mut b1, &source, &track1, "summed");
        b1.routes.push(to_output("summed-main", post_pan("summed")));
        let out1 = render_at(&document(&b1), &[("summed", &sum1)], BLOCKS, Backend::current());

        let mut sum2 = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
        let mut second: Vec<(&Route, &[Vec<f32>; 2])> = vec![(&bb, &out1), (&t3r, &t3_planes)];
        second.sort_by(|x, y| x.0.id.cmp(&y.0.id));
        for (rank, (r, planes)) in second.iter().enumerate() {
            accumulate(&mut sum2, r, planes, rank == 0);
        }
        let (mut b2, source, mut track2) = empty_session(rate);
        s2.onto(&mut track2);
        add_track(&mut b2, &source, &track2, "summed");
        b2.routes.push(to_output("summed-main", post_pan("summed")));
        let expected = render_at(&document(&b2), &[("summed", &sum2)], BLOCKS, Backend::current());

        let mut feeds: Vec<(&str, &[Vec<f32>; 2])> = level1.iter().map(|(t, _, p)| (t.as_str(), p)).collect();
        feeds.push(("t3", &t3_planes));
        for backend in backends() {
            let actual = render_at(&document(&a), &feeds, BLOCKS, backend);
            assert_same(&format!("nested seed {seed} {backend:?}"), &actual, &expected);
        }
    }
}

fn limiter(id: &str) -> Effect {
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native { effect_id: sid("miso.true-peak-limiter") },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// Nested PDC: limiter on bus1 (486 at 48k), t1 -> bus2 directly, bus1 -> bus2, bus2 -> out, and
/// t0 also -> out directly. Output latency 486; an impulse on t0 and t1 arrives everywhere at 486.
#[test]
fn v_nested_pdc() {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
    }
    model.routes.push(route("t0-b1", post_pan("t0"), into_bus("bus1")));
    model.routes.push(route("t1-b2", post_pan("t1"), into_bus("bus2")));
    model.routes.push(route("b1-b2", bus_output("bus1"), into_bus("bus2")));
    model.routes.push(to_output("b2-out", bus_output("bus2")));
    model.routes.push(to_output("t0-out", post_pan("t0")));
    let mut bus1 = Strip::transparent();
    bus1.inserts.effects.push(limiter("ceiling"));
    model.submixes = vec![bus1.submix("bus1"), Strip::transparent().submix("bus2")];
    let doc = document(&model);
    let compiled = compile_host_session(&doc, &caps()).expect("compile");
    let (prepared, _) = prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &HostLiveControlRequest::default(), Backend::current()).expect("prepare");
    assert_eq!(prepared.report.latency_samples, 486);
    let mut impulse = [vec![0.0_f32; QUANTUM * BLOCKS], vec![0.0_f32; QUANTUM * BLOCKS]];
    impulse[0][0] = 0.25;
    impulse[1][0] = 0.25;
    let silence = [vec![0.0_f32; QUANTUM * BLOCKS], vec![0.0_f32; QUANTUM * BLOCKS]];
    for (label, feeds) in [("t0", [("t0", &impulse), ("t1", &silence)]), ("t1", [("t0", &silence), ("t1", &impulse)])] {
        let out = render_at(&doc, &feeds, BLOCKS, Backend::current());
        for (plane, samples) in out.iter().enumerate() {
            let nonzero: Vec<usize> = samples.iter().enumerate().filter(|(_, s)| s.abs() > 1e-3).map(|(i, _)| i).collect();
            eprintln!("v_nested_pdc impulse on {label} plane {plane}: first loud {:?} peak {:?}", nonzero.first(), samples.iter().enumerate().max_by(|x, y| x.1.abs().total_cmp(&y.1.abs())).map(|(i, v)| (i, *v)));
            let peak = samples.iter().enumerate().max_by(|x, y| x.1.abs().total_cmp(&y.1.abs())).map(|(i, _)| i).unwrap();
            assert_eq!(peak, 486, "{label} plane {plane}");
        }
    }
}

/// A bus-to-bus cycle is refused; print its diagnostic.
#[test]
fn v_bus_cycle_is_refused() {
    let (mut model, source, track) = empty_session(48_000);
    add_track(&mut model, &source, &track, "t0");
    model.routes.push(route("t0-a", post_pan("t0"), into_bus("bus-a")));
    model.routes.push(route("ab", bus_output("bus-a"), into_bus("bus-b")));
    model.routes.push(route("ba", bus_output("bus-b"), into_bus("bus-a")));
    model.routes.push(to_output("a-out", bus_output("bus-a")));
    model.submixes = vec![Strip::transparent().submix("bus-a"), Strip::transparent().submix("bus-b")];
    let doc = document(&model);
    let result = compile_host_session(&doc, &caps());
    match result {
        Ok(compiled) => {
            let prepared = prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &HostLiveControlRequest::default(), Backend::current());
            match prepared {
                Ok(_) => panic!("a bus cycle prepared"),
                Err(f) => eprintln!("v_bus_cycle prepare refused: {}", String::from_utf8_lossy(f.as_bytes())),
            }
        }
        Err(f) => eprintln!("v_bus_cycle compile refused: {}", String::from_utf8_lossy(f.as_bytes())),
    }
}

fn keyed(mut effect: Effect, source: RouteSource) -> Effect {
    effect.sidechain = SidechainDeclaration::Routed(session::Sidechain { source, port_id: sid("sidechain-in") });
    effect
}

fn try_prepare(doc: &str) -> Result<(), String> {
    let compiled = compile_host_session(doc, &caps()).map_err(|f| format!("compile: {}", String::from_utf8_lossy(f.as_bytes())))?;
    prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &HostLiveControlRequest::default(), Backend::current())
        .map(|_| ())
        .map_err(|f| format!("prepare: {}", String::from_utf8_lossy(f.as_bytes())))
}

/// A bus compressor keyed from a track renders the bits of a track compressor keyed from the same
/// track and fed the bus's sum. Then: a track keyed from a bus output (acyclic) prepares, and a
/// track keyed from the bus it feeds is refused as a cycle.
#[test]
fn v_sidechains_through_buses() {
    let registry = launch_native_effect_registry().expect("registry");
    for seed in 200..216_u64 {
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&LAUNCH_RATES);
        let mut strip = drawn_strip(&mut draw, &registry);
        strip.inserts.effects[0] = keyed(strip.inserts.effects[0].clone(), post_pan("key"));
        let frames = QUANTUM * BLOCKS;
        let r0 = drawn_route(&mut draw, "r0", post_pan("t0"), into_bus("bus"));
        let r2 = drawn_route(&mut draw, "r2", post_pan("t2"), into_bus("bus"));
        let p0 = noise_planes(&mut draw, frames);
        let p2 = noise_planes(&mut draw, frames);
        let key = noise_planes(&mut draw, frames);
        let (mut a, source, track) = empty_session(rate);
        add_track(&mut a, &source, &track, "t0");
        add_track(&mut a, &source, &track, "t2");
        add_track(&mut a, &source, &track, "key");
        a.routes.push(r0.clone());
        a.routes.push(r2.clone());
        a.routes.push(to_output("bus-main", bus_output("bus")));
        a.submixes = vec![strip.clone().submix("bus")];
        let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
        accumulate(&mut sum, &r0, &p0, true);
        accumulate(&mut sum, &r2, &p2, false);
        let (mut b, source, mut summed) = empty_session(rate);
        strip.onto(&mut summed);
        add_track(&mut b, &source, &summed, "summed");
        add_track(&mut b, &source, &track, "key");
        b.routes.push(to_output("summed-main", post_pan("summed")));
        let expected = render_at(&document(&b), &[("summed", &sum), ("key", &key)], BLOCKS, Backend::current());
        for backend in backends() {
            let actual = render_at(&document(&a), &[("t0", &p0), ("t2", &p2), ("key", &key)], BLOCKS, backend);
            assert_same(&format!("keyed bus seed {seed} {backend:?}"), &actual, &expected);
        }
    }

    let fixture = parse_session_json(FIXTURE).expect("fixture");
    let comp = fixture.tracks[0].inserts.effects[0].clone();
    // Acyclic: t1's compressor keyed from the bus that t0 feeds.
    let (mut m, source, track) = empty_session(48_000);
    add_track(&mut m, &source, &track, "t0");
    let mut t1 = track.clone();
    t1.inserts.effects.push(keyed(comp.clone(), bus_output("bus")));
    add_track(&mut m, &source, &t1, "t1");
    m.routes.push(route("t0-bus", post_pan("t0"), into_bus("bus")));
    m.routes.push(to_output("bus-out", bus_output("bus")));
    m.routes.push(to_output("t1-out", post_pan("t1")));
    m.submixes = vec![Strip::transparent().submix("bus")];
    try_prepare(&document(&m)).expect("a track keyed from a bus it does not feed prepares");
    // Cyclic: t0 keyed from the bus it feeds.
    let mut c = m.clone();
    c.tracks.iter_mut().find(|t| t.id.as_str() == "t0").unwrap().inserts.effects.push(keyed(comp.clone(), bus_output("bus")));
    let refused = try_prepare(&document(&c)).expect_err("a key cycle through a bus is refused");
    eprintln!("v_sidechains cycle (track keyed from its own bus): {refused}");
    // Cyclic: the bus keyed from its own output.
    let mut s = m.clone();
    s.submixes[0].inserts.effects.push(keyed(comp, bus_output("bus")));
    let refused = try_prepare(&document(&s)).expect_err("a bus keyed from itself is refused");
    eprintln!("v_sidechains cycle (bus keyed from itself): {refused}");
}

/// A bus with no inputs (fan-in 0) and a bus with no outgoing route both prepare and render.
#[test]
fn v_degenerate_buses_render() {
    let (mut model, source, track) = empty_session(48_000);
    add_track(&mut model, &source, &track, "t0");
    model.routes.push(to_output("t0-out", post_pan("t0")));
    model.routes.push(route("t0-sink", post_pan("t0"), into_bus("sink")));
    model.routes.push(to_output("empty-out", bus_output("empty")));
    let registry = launch_native_effect_registry().expect("registry");
    let mut draw = Draw::new(77);
    let mut empty = drawn_strip(&mut draw, &registry);
    let sink = drawn_strip(&mut draw, &registry);
    model.submixes = vec![empty.submix("empty"), sink.submix("sink")];
    let doc = document(&model);
    let mut d = Draw::new(5);
    let planes = noise_planes(&mut d, QUANTUM * BLOCKS);
    // Expected: t0 alone through a transparent strip (plus +0.0 from the empty bus).
    let (mut solo, source, track) = empty_session(48_000);
    add_track(&mut solo, &source, &track, "t0");
    solo.routes.push(to_output("t0-out", post_pan("t0")));
    let expected = render_at(&document(&solo), &[("t0", &planes)], BLOCKS, Backend::current());
    for backend in backends() {
        let out = render_at(&doc, &[("t0", &planes)], BLOCKS, backend);
        assert_same(&format!("degenerate {backend:?}"), &out, &expected);
    }
}

/// Mixed banks: a deep track (8 EQ inserts) puts its fader on the same level as a shallow bus's
/// fader, so a bank may hold a live-controlled track lane beside an uncontrolled bus lane. With
/// live controls (no commands) the render matches the live-control-free render; prints banks.
#[test]
fn v_mixed_control_bank_with_a_bus_lane() {
    let registry = launch_native_effect_registry().expect("registry");
    let mut draw = Draw::new(9);
    let (mut model, source, track) = empty_session(48_000);
    let mut deep = track.clone();
    for index in 0..8 {
        deep.inserts.effects.push(drawn_effect(&mut draw, &registry, "miso.parametric-eq", &format!("eq{index}"), LinkMode::DualMono));
    }
    add_track(&mut model, &source, &deep, "t0");
    add_track(&mut model, &source, &track, "t1");
    let mut bus = Strip::transparent();
    bus.fader.left_db = -3.0;
    bus.fader.right_db = -9.0;
    model.submixes = vec![bus.submix("bus")];
    model.routes.push(route("t1-bus", post_pan("t1"), into_bus("bus")));
    model.routes.push(to_output("bus-out", bus_output("bus")));
    model.routes.push(to_output("t0-out", post_pan("t0")));
    let doc = document(&model);
    let p0 = noise_planes(&mut draw, QUANTUM * BLOCKS);
    let p1 = noise_planes(&mut draw, QUANTUM * BLOCKS);
    let feeds: [(&str, &[Vec<f32>; 2]); 2] = [("t0", &p0), ("t1", &p1)];
    for backend in backends() {
        let plain = render_at(&doc, &feeds, BLOCKS, backend);
        let compiled = compile_host_session(&doc, &caps()).expect("compile");
        let request = HostLiveControlRequest {
            control_queue_depth: Some(core::num::NonZeroUsize::new(64).unwrap()),
            ..HostLiveControlRequest::default()
        };
        let (mut prepared, handles) = prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &request, backend)
            .unwrap_or_else(|f| panic!("prepare: {}", String::from_utf8_lossy(f.as_bytes())));
        let banks: Vec<Vec<String>> = prepared.plan.unit_eligibility().iter().filter(|u| u.banked).map(|u| u.lane_tracks.iter().map(|t| t.to_string()).collect()).collect();
        eprintln!("{backend:?} banks: {banks:?}; tracks {:?}; track_controls {}", handles.tracks, handles.track_controls.len());
        let rate = prepared.report.sample_rate_hz;
        let mut out = [Vec::new(), Vec::new()];
        for block in 0..BLOCKS {
            let range = block * QUANTUM..(block + 1) * QUANTUM;
            for (id, planes) in &feeds {
                prepared.sources.submit(id.as_bytes(), SourceSubmission { generation: 1, start_frame: range.start as u64, sample_rate_hz: rate, planes: &[&planes[0][range.clone()], &planes[1][range.clone()]], frames: QUANTUM as u32, end_of_region: false }).expect("source");
            }
            let mut samples = [0.0_f32; QUANTUM * 2];
            prepared.plan.render(RenderIo { output: PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).unwrap() }, RenderTime { absolute_sample: (block * QUANTUM) as u64 }).expect("render");
            out[0].extend_from_slice(&samples[..QUANTUM]);
            out[1].extend_from_slice(&samples[QUANTUM..]);
        }
        assert_same(&format!("mixed {backend:?}"), &out, &plain);
    }
}

#[test]
fn v_print_bank_route_folds() {
    let registry = launch_native_effect_registry().expect("registry");
    let mut folds = Vec::new();
    for seed in 0..32_u64 {
        // Replays gate 1's draw sequence exactly (gate_one_sessions).
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&LAUNCH_RATES);
        let strip = drawn_strip(&mut draw, &registry);
        let frames = QUANTUM * BLOCKS;
        let mut names = vec!["route-a", "route-b", "route-c"];
        let contributors: Vec<(String, Route)> = (0..3)
            .map(|index| {
                let route_id = names.remove(draw.below(names.len()));
                let track = format!("t{index}");
                let r = drawn_route(&mut draw, route_id, post_pan(&track), into_bus("bus"));
                let _ = noise_planes(&mut draw, frames);
                (track, r)
            })
            .collect();
        let (mut a, source, track) = empty_session(rate);
        for (t, r) in &contributors {
            add_track(&mut a, &source, &track, t);
            a.routes.push(r.clone());
        }
        a.submixes = vec![strip.submix("bus")];
        a.routes.push(to_output("bus-main", bus_output("bus")));
        let compiled = compile_host_session(&document(&a), &caps()).expect("compile");
        let (prepared, _) = prepare_host_runtime_with_live_controls_backend(&compiled, &caps(), &HostLiveControlRequest::default(), Backend::current()).expect("prepare");
        folds.push(prepared.plan.bank_route_folds());
    }
    eprintln!("gate-1 bank_route_folds: {folds:?}");
}
