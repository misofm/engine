//! #1379 Amendment 1, the graph's tail composition through gain, recomputed (gates C3, C4, C5, C7,
//! C11 (a) and C13).
//!
//! Every compiled value here -- `output_tail`, `output_tail_every_peak` and `P*_graph` -- is
//! recomputed by [`enumerate`], which evaluates H4's rule from the plan's own per-node values by
//! explicitly enumerating every path to the output, with no code shared with the compiler's
//! `pdc.rs`. The real-kernel gates (C1, C2, C6, C11 (b)) are host-core's
//! `tail_composition.rs`.

use std::collections::BTreeMap;

use builtins_compiler::{
    BuiltinCompileCaps, LiveValuesSuperseded, TrackControlRequest, prepare_session_builtins,
    prepare_session_builtins_with_live_controls,
};
use conformance::DualAccumulatorDelayFactory;
use effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
use effect_contract::{
    CompositionBound, FlushStall, NativeEffectFactory, NativeEffectRegistry, NodeTailBound,
    PeakGain, RestBound, TailDecay, TailSamples,
};
use graph::{GraphCompileCaps, GraphPortKind};
use graph_compiler::{
    Backend, FlushFloor, GainLiveness, GraphBuiltinsCompileRequest, GraphCompileReport,
    GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ControlAddress, Lane, Owner,
    ParameterUnit, Section,
};
use session::{
    ChannelMatrix, CompileCaps, Effect, EffectIdentity, EffectQuality, LinkMode, MatrixOrPan,
    Route, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, StableId,
    Submix, compile_session, parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn sid(value: &str) -> StableId {
    StableId::parse(value).expect("stable id")
}

fn graph_caps(maximum_finite_tail_samples: u64) -> GraphCompileCaps {
    GraphCompileCaps {
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
        maximum_finite_tail_samples,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
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
    }
}

const ONES: ChannelMatrix = ChannelMatrix {
    ll: 1.0,
    lr: 1.0,
    rl: 1.0,
    rr: 1.0,
};

const MATRIX_ONES: MatrixOrPan = MatrixOrPan::Matrix {
    ll: 1.0,
    lr: 1.0,
    rl: 1.0,
    rr: 1.0,
    smoothing_samples: 0,
};

fn worst_pair() -> (f32, f32) {
    builtins::input_section_worst_case_pair(48_000).expect("a launch rate")
}

fn route(id: &str, source: RouteSource, destination: RouteDestination) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: ONES,
        gain_db: 24.0,
        mute: false,
        follows_mute: false,
    }
}

fn from_track(track: &str) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::PostPan,
    }
}

fn from_submix(submix: &str) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(submix),
        tap: SendTap::PostPan,
    }
}

fn into_submix(submix: &str) -> RouteDestination {
    RouteDestination::SubmixInput {
        submix_id: sid(submix),
    }
}

fn into_output() -> RouteDestination {
    RouteDestination::OutputInput {
        output_id: sid("main-out"),
    }
}

/// The fixture's one 48 kHz track with no inserts and no automation, its input section at
/// `(hpf, lpf)` with trim `trim_db`, every gain after it at its domain maximum (fader +24 dB, all
/// four matrix coefficients 1), and no routes.
fn base(hpf: f32, lpf: f32, trim_db: f32) -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture parses");
    model.automation.clear();
    model.routes.clear();
    let track = &mut model.tracks[0];
    track.inserts.effects.clear();
    for lane in [&mut track.builtins.left, &mut track.builtins.right] {
        lane.hpf_hz = hpf;
        lane.lpf_hz = lpf;
        lane.trim_db = trim_db;
    }
    track.fader.left_db = 24.0;
    track.fader.right_db = 24.0;
    track.matrix_or_pan = MATRIX_ONES;
    model
}

/// A submix strip at every gain maximum (fader +24 dB, matrix ones) with input section
/// `(hpf, lpf)` and trim `trim_db`.
fn submix(model: &mut SessionModel, id: &str, hpf: f32, lpf: f32, trim_db: f32) {
    let mut strip = Submix::unity(sid(id), &model.console);
    for lane in [&mut strip.builtins.left, &mut strip.builtins.right] {
        lane.hpf_hz = hpf;
        lane.lpf_hz = lpf;
        lane.trim_db = trim_db;
    }
    strip.fader.left_db = 24.0;
    strip.fader.right_db = 24.0;
    strip.matrix_or_pan = MATRIX_ONES;
    model.submixes.push(strip);
}

/// C1's session: the worst-case pair at 0 dB trim, fader +24 dB, matrix ones, one route at +24 dB
/// with ones to the output.
fn c1() -> SessionModel {
    let (hpf, lpf) = worst_pair();
    let mut model = base(hpf, lpf, 0.0);
    model
        .routes
        .push(route("to-main", from_track("vocal"), into_output()));
    model
}

/// C2's session: a track with both filters off at trim +24 dB into a submix whose input section is
/// the worst-case pair at 0 dB trim, every gain at its maximum, the submix routed to the output.
fn c2() -> SessionModel {
    let (hpf, lpf) = worst_pair();
    let mut model = base(0.0, 0.0, 24.0);
    submix(&mut model, "bus", hpf, lpf, 0.0);
    model
        .routes
        .push(route("to-bus", from_track("vocal"), into_submix("bus")));
    model
        .routes
        .push(route("bus-main", from_submix("bus"), into_output()));
    model
}

/// C4's session: `tracks` tracks, each with the worst-case pair and every gain at its maximum,
/// into one submix with both filters off at trim +15 dB, routed to the output.
fn c4(tracks: usize) -> SessionModel {
    let (hpf, lpf) = worst_pair();
    let mut model = base(hpf, lpf, 0.0);
    let template = model.tracks[0].clone();
    model.tracks.clear();
    for index in 0..tracks {
        let mut track = template.clone();
        track.id = sid(&format!("t{index}"));
        model.tracks.push(track);
        model.routes.push(route(
            &format!("t{index}-bus"),
            from_track(&format!("t{index}")),
            into_submix("bus"),
        ));
    }
    submix(&mut model, "bus", 0.0, 0.0, 15.0);
    model
        .routes
        .push(route("bus-main", from_submix("bus"), into_output()));
    model
}

/// C6's first design at `rate` (Amendment 1 H5): the 1 kHz input LPF, HPF off, trim 0 dB, every
/// downstream gain at 0 dB (fader, identity matrix, one 0 dB identity route).
fn c6(rate: u32) -> SessionModel {
    let mut model = base(0.0, 1_000.0, 0.0);
    model.sample_rate_hz = rate;
    let track = &mut model.tracks[0];
    track.fader.left_db = 0.0;
    track.fader.right_db = 0.0;
    track.matrix_or_pan = MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        smoothing_samples: 0,
    };
    let mut to_main = route("to-main", from_track("vocal"), into_output());
    to_main.gain_db = 0.0;
    to_main.channel_matrix = ChannelMatrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };
    model.routes.push(to_main);
    model
}

/// C13 (b): a track with the worst-case pair into `depth` nested submixes (filters off, every gain
/// at its maximum), the last routed to the output.
fn nested(depth: usize) -> SessionModel {
    let (hpf, lpf) = worst_pair();
    let mut model = base(hpf, lpf, 0.0);
    let mut source = from_track("vocal");
    for level in 0..depth {
        let id = format!("bus{level:03}");
        submix(&mut model, &id, 0.0, 0.0, 24.0);
        model
            .routes
            .push(route(&format!("to-{id}"), source, into_submix(&id)));
        source = from_submix(&id);
    }
    model.routes.push(route("to-main", source, into_output()));
    model
}

/// Which strips get a live input lane (`TrackControlRequest::input_lane`).
#[derive(Clone, Copy)]
enum InputLanes {
    None,
    All,
}

fn compile(
    model: &SessionModel,
    liveness: GainLiveness,
    inputs: InputLanes,
    cap: u64,
) -> Result<PreparedGraphBuiltinsArtifact, Vec<&'static str>> {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session compiles");
    let builtins = match inputs {
        InputLanes::None => prepare_session_builtins(&session, &[], builtin_caps()),
        InputLanes::All => {
            let requests: Vec<_> = session
                .normalized_model()
                .strips()
                .map(|strip| TrackControlRequest {
                    track_id: strip.id.as_str().to_owned(),
                    superseded: LiveValuesSuperseded::default(),
                    input_lane: true,
                })
                .collect();
            let effective = controls::effective(
                session.normalized_model(),
                &controls::MonitorOverlay::default(),
            );
            prepare_session_builtins_with_live_controls(
                &session,
                &effective,
                &[],
                &requests,
                builtin_caps(),
            )
        }
    }
    .expect("builtins prepare");
    // The conformance mock (C7) has its own registry; every other session prepares through the
    // launch registry.
    let mock = model.tracks.iter().any(|track| {
        track.inserts.effects.iter().any(|effect| {
            matches!(&effect.identity, EffectIdentity::Native { effect_id }
                if effect_id.as_str() == "conformance.delay")
        })
    });
    let owned;
    let registry: &NativeEffectRegistry = if mock {
        owned = NativeEffectRegistry::new([
            Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>
        ])
        .expect("registry");
        &owned
    } else {
        effect_compiler::launch_native_effect_registry().expect("launch registry")
    };
    let effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("effects prepare");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 1,
        effects,
        builtins,
        caps: graph_caps(cap),
        dispatch: Backend::current(),
        input_floors: BTreeMap::new(),
        gain_liveness: liveness,
    })
    .map_err(|failure| {
        failure
            .diagnostics
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect()
    })
}

const FIXED: GainLiveness = GainLiveness::FIXED;
const LIVE: GainLiveness = GainLiveness {
    strip_fader_matrix: true,
    routes: true,
};

/// What [`enumerate`] computes: H4's three values.
#[derive(Debug, PartialEq)]
struct Rule {
    output_tail: TailSamples,
    output_tail_every_peak: TailSamples,
    output_flush_floor: FlushFloor,
}

impl Rule {
    fn of(report: &GraphCompileReport) -> Self {
        Self {
            output_tail: report.output_tail,
            output_tail_every_peak: report.output_tail_every_peak,
            output_flush_floor: report.output_flush_floor,
        }
    }
}

/// An upper bound of the exact value of one rounded non-negative `f64` operation.
fn up(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        x * (1.0 + 1.0 / 1_073_741_824.0) + f64::from_bits(1)
    }
}

/// `10^(mb / 2000)` rounded up, with values below `10^-300` stated as `10^-300`.
fn ten_to(mb: i64) -> f64 {
    let mb = mb.max(-600_000) as f64;
    up(math::exp(mb * (core::f64::consts::LN_10 / 2000.0)))
}

fn gain(value: PeakGain) -> Option<i64> {
    match value {
        PeakGain::Zero => None,
        PeakGain::Millibels(value) => Some(i64::from(value)),
    }
}

fn stall(value: FlushStall) -> f64 {
    match value {
        FlushStall::Zero => 0.0,
        FlushStall::Level(level) => ten_to(i64::from(level)),
    }
}

/// One node's composition values, `(D, G_p, G_t, sigma_p, sigma_t)`.
fn stated(bound: &NodeTailBound) -> Option<(u64, PeakGain, PeakGain, FlushStall, FlushStall)> {
    match bound.composition {
        CompositionBound::Stated {
            decay: TailDecay(decay),
            peak_gain,
            tail_gain,
            peak_stall,
            tail_stall,
        } => Some((decay, peak_gain, tail_gain, peak_stall, tail_stall)),
        CompositionBound::Unstated => None,
    }
}

/// A sum of millibel values, `None` (`-infinity`) absorbing.
fn sum(values: impl IntoIterator<Item = Option<i64>>) -> Option<i64> {
    values
        .into_iter()
        .try_fold(0_i64, |total, value| Some(total + value?))
}

/// H4's rule from the plan's per-node values, by explicit enumeration of every path that ends at
/// the output (`output_tail`, the rest extent), every signal path from a source to each node
/// (`up`, `nu`, `S_out`) and every signal path from each node to the output (`down`, `nd`).
fn enumerate(artifact: &PreparedGraphBuiltinsArtifact) -> Rule {
    let spec = &artifact.graph().spec;
    let index: BTreeMap<_, _> = spec
        .nodes
        .iter()
        .enumerate()
        .map(|(position, node)| (&node.id, position))
        .collect();
    let delays: BTreeMap<_, _> = artifact
        .graph()
        .inserted_delays
        .iter()
        .map(|delay| (&delay.edge_id, delay.samples.0))
        .collect();
    // `(source, destination, signal, comp)`.
    let edges: Vec<(usize, usize, bool, u64)> = spec
        .edges
        .iter()
        .map(|edge| {
            (
                index[&edge.source.node],
                index[&edge.destination.node],
                edge.destination.kind != GraphPortKind::SidechainInput,
                delays.get(&edge.id).copied().unwrap_or(0),
            )
        })
        .collect();
    let output = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, graph::GraphNodeId::Output { .. }))
        .expect("an output");
    let count = spec.nodes.len();
    // Every path (any edge kind) ending at the output, as node lists with the comps between them.
    let mut to_output: Vec<(Vec<usize>, u64)> = Vec::new();
    let mut stack = vec![(vec![output], 0_u64)];
    while let Some((path, comp)) = stack.pop() {
        let head = path[0];
        let mut extended = false;
        for &(source, destination, _, delay) in &edges {
            if destination == head {
                let mut longer = vec![source];
                longer.extend(&path);
                stack.push((longer, comp + delay));
                extended = true;
            }
        }
        if !extended {
            to_output.push((path, comp));
        }
    }
    let mut reaches = vec![false; count];
    for (path, _) in &to_output {
        for &node in path {
            reaches[node] = true;
        }
    }
    // Step 8.
    if (0..count).any(|node| {
        reaches[node]
            && (spec.nodes[node].bound.tail == TailSamples::Infinite
                || stated(&spec.nodes[node].bound).is_none())
    }) {
        return Rule {
            output_tail: TailSamples::Infinite,
            output_tail_every_peak: TailSamples::Infinite,
            output_flush_floor: FlushFloor::Unstated,
        };
    }
    let values: Vec<_> = (0..count)
        .map(|node| stated(&spec.nodes[node].bound))
        .collect();
    let value = |node: usize| values[node].expect("stated");
    let decays = |node: usize| u64::from(value(node).0 > 0);
    let signal_into = |node: usize| {
        edges
            .iter()
            .filter(move |edge| edge.1 == node && edge.2)
            .map(|edge| edge.0)
    };
    // Every signal path from a source ending at each node.
    let mut from_source: Vec<Vec<Vec<usize>>> = vec![Vec::new(); count];
    for node in 0..count {
        if !reaches[node] {
            continue;
        }
        let mut stack = vec![vec![node]];
        while let Some(path) = stack.pop() {
            let head = path[0];
            let predecessors: Vec<_> = signal_into(head).collect();
            if predecessors.is_empty() {
                from_source[node].push(path);
            } else {
                for predecessor in predecessors {
                    let mut longer = vec![predecessor];
                    longer.extend(&path);
                    stack.push(longer);
                }
            }
        }
    }
    // Every signal path from each node to the output.
    let mut to_sink: Vec<Vec<Vec<usize>>> = vec![Vec::new(); count];
    for (node, paths) in to_sink.iter_mut().enumerate() {
        let mut stack = vec![vec![node]];
        while let Some(path) = stack.pop() {
            let tail = *path.last().expect("nonempty");
            if tail == output {
                paths.push(path.clone());
            }
            for edge in edges.iter().filter(|edge| edge.0 == tail && edge.2) {
                let mut longer = path.clone();
                longer.push(edge.1);
                stack.push(longer);
            }
        }
    }
    let up_of = |node: usize| {
        from_source[node]
            .iter()
            .map(|path| sum(path[..path.len() - 1].iter().map(|&q| gain(value(q).1))))
            .max()
            .flatten()
    };
    let nu_of = |node: usize| {
        from_source[node]
            .iter()
            .map(|path| {
                path[..path.len() - 1]
                    .iter()
                    .map(|&q| decays(q))
                    .sum::<u64>()
            })
            .max()
            .unwrap_or(0)
    };
    let down_of = |node: usize| {
        to_sink[node]
            .iter()
            .map(|path| sum(path[1..].iter().map(|&q| gain(value(q).2))))
            .max()
            .flatten()
    };
    let nd_of = |node: usize| {
        to_sink[node]
            .iter()
            .map(|path| path[1..].iter().map(|&q| decays(q)).sum::<u64>())
            .max()
            .unwrap_or(0)
    };
    let k: Vec<u64> = (0..count)
        .map(|node| {
            if !reaches[node] || value(node).0 == 0 {
                return 0;
            }
            let (Some(upstream), Some(below)) = (up_of(node), down_of(node)) else {
                return 0;
            };
            let paths = nu_of(node) + nd_of(node) + 1;
            let a = upstream
                + below
                + i64::from(math::tail::ceil_millibels(
                    paths as f64,
                    math::tail::Rounding::Exact,
                ));
            if a <= 0 { 0 } else { (a as u64).div_ceil(2000) }
        })
        .collect();
    // Step 5: the longest path ending at the output.
    let output_tail = to_output
        .iter()
        .map(|(path, comp)| {
            comp + path
                .iter()
                .map(|&node| {
                    let TailSamples::Finite(tail) = spec.nodes[node].bound.tail else {
                        unreachable!()
                    };
                    spec.nodes[node].latency.0 + tail + k[node] * value(node).0
                })
                .sum::<u64>()
        })
        .max()
        .expect("a path");
    // Step 6: `S_out(v)`, the largest stall a signal path from a source carries to `v`'s output.
    let s_out = |node: usize| {
        from_source[node]
            .iter()
            .map(|path| {
                path.iter().fold(0.0_f64, |carried, &q| {
                    let (_, peak_gain, _, peak_stall, _) = value(q);
                    let g = gain(peak_gain).map_or(0.0, ten_to);
                    up(up(g * carried) + stall(peak_stall))
                })
            })
            .fold(0.0_f64, f64::max)
    };
    let s_in = |node: usize| signal_into(node).map(s_out).fold(0.0_f64, f64::max);
    let mut sigma = 0.0_f64;
    for node in (0..count).filter(|&node| reaches[node]) {
        let (decay, _, _, _, tail_stall) = value(node);
        let mut effective = stall(tail_stall);
        if decay > 0 {
            let relative = up(up(0.75 * math::tail::TAIL_FLOOR) * ten_to(-2000 * k[node] as i64));
            effective = up(effective + up(relative * s_in(node)));
        }
        if let Some(below) = down_of(node)
            && effective > 0.0
        {
            sigma = up(sigma + up(ten_to(below) * effective));
        }
    }
    let linear = up(up(4.0 * sigma) / math::tail::TAIL_FLOOR);
    let overflow = !linear.is_finite()
        || (0..count)
            .filter(|&node| reaches[node])
            .any(|node| !s_out(node).is_finite());
    let floor = if overflow {
        FlushFloor::AboveRange
    } else if sigma == 0.0 {
        FlushFloor::Zero
    } else {
        FlushFloor::Millibels(math::tail::ceil_millibels(
            linear,
            math::tail::Rounding::Computed,
        ))
    };
    // Step 7: the rest branch.
    let every_peak = match floor {
        FlushFloor::Zero => output_tail,
        _ => {
            let plus_24 = f64::from(builtins::checked_fader_gain(24.0).expect("domain"))
                .min(math::pow(10.0, 1.2));
            let rest_of = |node: usize| {
                let Some(rest) = spec.nodes[node].bound.rest.samples() else {
                    unreachable!("C3's sessions state every rest")
                };
                let quiet = match floor {
                    FlushFloor::Millibels(mb) => {
                        let peak = up_of(node)
                            .map_or(0.0, |upstream| up(ten_to(i64::from(mb)) * ten_to(upstream)));
                        up(peak + s_in(node)) <= plus_24
                    }
                    _ => false,
                };
                if quiet {
                    rest.peak_plus_24_dbfs
                } else {
                    rest.any_sanitized_input
                }
            };
            let rest_extent = to_output
                .iter()
                .map(|(path, comp)| {
                    comp + path
                        .iter()
                        .map(|&node| spec.nodes[node].latency.0 + rest_of(node))
                        .sum::<u64>()
                })
                .max()
                .expect("a path");
            output_tail.max(rest_extent)
        }
    };
    Rule {
        output_tail: TailSamples::Finite(output_tail),
        output_tail_every_peak: TailSamples::Finite(every_peak),
        output_flush_floor: floor,
    }
}

fn finite(value: TailSamples) -> u64 {
    match value {
        TailSamples::Finite(samples) => samples,
        TailSamples::Infinite => panic!("a finite tail"),
    }
}

/// The compiled values and their enumeration, asserted equal.
fn recomputed(model: &SessionModel, liveness: GainLiveness, inputs: InputLanes) -> Rule {
    let artifact = compile(model, liveness, inputs, u64::MAX).expect("compiles");
    let compiled = Rule::of(artifact.report());
    assert_eq!(compiled, enumerate(&artifact));
    compiled
}

/// C3: the compiled values equal the enumeration for C1, C2, C4, C5, C6 (every launch rate) and
/// C11's sessions under both selections, and for a live track into a live submix (every strip
/// with a live input lane, whose `G_t` is below its `G_p`). Red for a rule that drops a path, a
/// decay term, the `n_p` term, the clamp, the stall's cross term or its downstream gain, uses peak
/// gains downstream, rounds a millibel down, or takes the maximum of the stalls.
#[test]
fn compiled_tail_values_equal_the_enumerated_rule() {
    let mut c11 = c1();
    c11.tracks[0].fader.left_db = -24.0;
    c11.tracks[0].fader.right_db = -24.0;
    let mut c5 = c1();
    c5.tracks[0].builtins.left.delay_samples = 1_000;
    c5.tracks[0].builtins.right.delay_samples = 1_000;
    let c6s = [44_100, 48_000, 88_200, 96_000].map(c6);
    for model in [c1(), c2(), c4(1), c4(2), c5, c11].into_iter().chain(c6s) {
        for liveness in [FIXED, LIVE] {
            let rule = recomputed(&model, liveness, InputLanes::None);
            assert!(matches!(rule.output_flush_floor, FlushFloor::Millibels(_)));
        }
    }
    let live = recomputed(&c2(), LIVE, InputLanes::All);
    assert!(finite(live.output_tail_every_peak) >= finite(live.output_tail));
}

/// C11 (a): without a live lane the fader states its prepared gain, with one the domain maximum; a
/// fader or mute lane driven by stored automation is live under every selection; a live route (into
/// a submix, or into the output from before the fader, #1391) states the route domain's maximum and
/// a fixed one its prepared row. Red if a live fader or route states its prepared value (a tail
/// computed from a pre-fader output route's constant), a fixed one its domain maximum, or an
/// automated lane is taken as fixed.
#[test]
fn a_live_fader_states_its_domain_maximum_and_a_fixed_one_its_prepared_gain() {
    let mut model = c1();
    model.tracks[0].fader.left_db = -24.0;
    model.tracks[0].fader.right_db = -24.0;
    let fader_gain = |liveness| {
        let artifact = compile(&model, liveness, InputLanes::None, u64::MAX).expect("compiles");
        let node = artifact
            .graph()
            .spec
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.id, graph::GraphNodeId::TrackStage {
                    track_id, stage: graph::TrackStage::PostFader
                } if track_id.as_str() == "vocal")
            })
            .expect("fader node")
            .bound;
        stated(&node).expect("stated").1
    };
    let word = |db: f32| f64::from(builtins::checked_fader_gain(db).expect("domain"));
    let expected = |db: f32| {
        PeakGain::Millibels(math::tail::ceil_millibels(
            word(db) * (1.0 + 1.0 / 16_777_216.0) * (1.0 + 1.0 / 1_073_741_824.0),
            math::tail::Rounding::Computed,
        ))
    };
    assert_eq!(fader_gain(FIXED), expected(-24.0));
    assert_eq!(fader_gain(LIVE), expected(24.0));

    // Stored automation makes a lane live whatever the selection: a fader entry states the +24 dB
    // word, and a mute entry on a muted strip forbids `Zero` (it can unmute).
    let entry = |control: u32, unit: ParameterUnit, value: f32| Automation {
        id: sid(&format!("ride{control}")),
        target: ControlAddress {
            owner: Owner::Strip(sid("vocal")),
            section: Section::Builtins,
            control,
            lane: Lane::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 1,
            start_value: value,
            end_value: value,
            unit,
        }],
        offset: AutomationOffset::default(),
    };
    let fader_gain_of = |model: &SessionModel| {
        let artifact = compile(model, FIXED, InputLanes::None, u64::MAX).expect("compiles");
        let node = artifact
            .graph()
            .spec
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.id, graph::GraphNodeId::TrackStage {
                    track_id, stage: graph::TrackStage::PostFader
                } if track_id.as_str() == "vocal")
            })
            .expect("fader node")
            .bound;
        stated(&node).expect("stated").1
    };
    let mut ridden = model.clone();
    ridden.automation.push(entry(5, ParameterUnit::Db, -24.0));
    assert_eq!(fader_gain_of(&ridden), expected(24.0));
    let mut muted = model.clone();
    muted.tracks[0].fader.left_mute = true;
    muted.tracks[0].fader.right_mute = true;
    assert_eq!(fader_gain_of(&muted), PeakGain::Zero);
    muted.automation.push(entry(6, ParameterUnit::Linear, 1.0));
    assert_eq!(fader_gain_of(&muted), expected(-24.0));

    // A route into a submix: live under `routes`, the fold of +24 dB and 1.0 with
    // the 2x2's rounding; fixed, its prepared row (here -6 dB with ones).
    let mut sends = c2();
    sends
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "to-bus")
        .expect("the send")
        .gain_db = -6.0;
    let send_gain = |liveness| {
        let artifact = compile(&sends, liveness, InputLanes::None, u64::MAX).expect("compiles");
        let node = artifact
            .graph()
            .spec
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.id, graph::GraphNodeId::Route { route_id }
                    if route_id.as_str() == "to-bus")
            })
            .expect("route node")
            .bound;
        stated(&node).expect("stated").1
    };
    let route_expected = |db: f32| {
        let inflate = |x: f64| x * (1.0 + 1.0 / 1_073_741_824.0);
        let u = 1.0 / 16_777_216.0;
        let word = f64::from((math::db_to_gain_f32(db) * 1.0_f32).abs());
        let row = inflate(word + word);
        PeakGain::Millibels(math::tail::ceil_millibels(
            inflate(inflate(row * (1.0 + u)) * (1.0 + u)),
            math::tail::Rounding::Computed,
        ))
    };
    assert_eq!(send_gain(LIVE), route_expected(24.0));
    assert_eq!(send_gain(FIXED), route_expected(-6.0));

    // #1391: a route into the output from before its strip's fader rides a lane too, so it states
    // the live bound; from `post_pan` it keeps its prepared row.
    let output_gain = |tap, liveness| {
        let mut model = c2();
        let route = model
            .routes
            .iter_mut()
            .find(|route| route.id.as_str() == "bus-main")
            .expect("the bus route");
        route.gain_db = -6.0;
        route.source = RouteSource::Submix {
            submix_id: sid("bus"),
            tap,
        };
        let artifact = compile(&model, liveness, InputLanes::None, u64::MAX).expect("compiles");
        let node = artifact
            .graph()
            .spec
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.id, graph::GraphNodeId::Route { route_id }
                    if route_id.as_str() == "bus-main")
            })
            .expect("route node")
            .bound;
        stated(&node).expect("stated").1
    };
    assert_eq!(output_gain(SendTap::PreFader, LIVE), route_expected(24.0));
    assert_eq!(output_gain(SendTap::PreFader, FIXED), route_expected(-6.0));
    assert_eq!(output_gain(SendTap::PostPan, LIVE), route_expected(-6.0));
}

/// C4: with every gain at its maximum the submix sum's fan-in moves each track section across a
/// decade boundary, so two tracks report exactly one `D` more than one. Red if a sum is stated at
/// 0 dB (fan-in ignored).
#[test]
fn fan_in_costs_the_track_section_one_decade() {
    let one = recomputed(&c4(1), FIXED, InputLanes::None);
    let two = recomputed(&c4(2), FIXED, InputLanes::None);
    let artifact = compile(&c4(1), FIXED, InputLanes::None, u64::MAX).expect("compiles");
    let decay = artifact
        .graph()
        .spec
        .nodes
        .iter()
        .find(|node| {
            matches!(&node.id, graph::GraphNodeId::TrackStage {
                track_id, stage: graph::TrackStage::PostInputBuiltins
            } if track_id.as_str() == "t0")
        })
        .and_then(|node| stated(&node.bound))
        .expect("stated section")
        .0;
    assert!(decay > 0);
    assert_eq!(
        finite(two.output_tail),
        finite(one.output_tail) + decay,
        "one decade more for the second track"
    );
}

/// C5: a strip delay of 1,000 samples lengthens both extents by exactly 1,000 and leaves the floor.
/// Red if the extent omits the strip's delay line.
#[test]
fn a_strip_delay_lengthens_both_extents_by_its_length() {
    let plain = recomputed(&c1(), FIXED, InputLanes::None);
    let mut model = c1();
    model.tracks[0].builtins.left.delay_samples = 1_000;
    model.tracks[0].builtins.right.delay_samples = 1_000;
    let delayed = recomputed(&model, FIXED, InputLanes::None);
    assert_eq!(
        finite(delayed.output_tail),
        finite(plain.output_tail) + 1_000
    );
    assert_eq!(
        finite(delayed.output_tail_every_peak),
        finite(plain.output_tail_every_peak) + 1_000
    );
    assert_eq!(delayed.output_flush_floor, plain.output_flush_floor);
}

/// C7: one insert whose composition is `Unstated` (the conformance mock: tail `Finite(3)`) makes
/// both values `Infinite` and the floor unstated. Red if an `Unstated` node composes as a gain-free,
/// decay-free node.
#[test]
fn an_unstated_node_makes_the_graph_tail_infinite() {
    let mut model = c1();
    model.tracks[0].inserts.effects.push(Effect {
        id: sid("mock"),
        identity: EffectIdentity::Native {
            effect_id: sid("conformance.delay"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    let rule = recomputed(&model, FIXED, InputLanes::None);
    assert_eq!(
        rule,
        Rule {
            output_tail: TailSamples::Infinite,
            output_tail_every_peak: TailSamples::Infinite,
            output_flush_floor: FlushFloor::Unstated,
        }
    );
}

/// #1376's `FixedPoint` rest composes as a rest (exact `+-0.0` output from `R` on): a plan whose
/// only recursive node is the gate/expander, which states one, reports a finite every-peak tail
/// equal to the enumeration. Red if the rest branch reads only `Bounded`.
#[test]
fn a_fixed_point_rest_composes_as_a_rest() {
    let mut model = base(0.0, 0.0, 0.0);
    model
        .routes
        .push(route("to-main", from_track("vocal"), into_output()));
    model.tracks[0].inserts.effects.push(Effect {
        id: sid("gate"),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.gate-expander"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    let artifact = compile(&model, FIXED, InputLanes::None, u64::MAX).expect("compiles");
    assert!(
        artifact
            .graph()
            .spec
            .nodes
            .iter()
            .any(|node| matches!(node.bound.rest, RestBound::FixedPoint(_)))
    );
    let rule = Rule::of(artifact.report());
    assert_eq!(rule, enumerate(&artifact));
    assert!(matches!(rule.output_flush_floor, FlushFloor::Millibels(_)));
    assert!(matches!(
        rule.output_tail_every_peak,
        TailSamples::Finite(_)
    ));
}

/// C13 (a): the cap checks the reported value. Red if it checks only `output_tail`.
#[test]
fn the_tail_cap_refuses_on_the_every_peak_value() {
    let rule = recomputed(&c1(), FIXED, InputLanes::None);
    let every_peak = finite(rule.output_tail_every_peak);
    assert!(
        finite(rule.output_tail) < every_peak,
        "C1's rest branch is the longer"
    );
    assert_eq!(
        compile(&c1(), FIXED, InputLanes::None, every_peak - 1).err(),
        Some(vec!["graph.tail.limit"])
    );
    assert!(compile(&c1(), FIXED, InputLanes::None, every_peak).is_ok());
}

/// C13 (b): a nesting deep enough that the `f64` stall sum is not finite reports the floor
/// `AboveRange` and the rest branch at `any_sanitized_input`; one level fewer a finite floor. The
/// depth is the least one at which the enumeration overflows, found from a first estimate of
/// the stated per-level gain. Red if an overflowing sum states a finite floor (saturated, or a
/// `NaN` cast) or loses the rest branch.
#[test]
fn an_overflowing_stall_sum_states_the_floor_above_range() {
    let floor = |depth: usize| {
        let artifact =
            compile(&nested(depth), FIXED, InputLanes::None, u64::MAX).expect("compiles");
        let rule = enumerate(&artifact);
        assert_eq!(Rule::of(artifact.report()), rule);
        rule
    };
    // Each level states about 8,400 mB of tail gain (trim, fader, matrix, route); `f64` holds
    // about 616,000 mB.
    let mut depth = 70;
    while floor(depth).output_flush_floor != FlushFloor::AboveRange {
        depth += 1;
    }
    while depth > 1 && floor(depth - 1).output_flush_floor == FlushFloor::AboveRange {
        depth -= 1;
    }
    let below = floor(depth - 1);
    assert!(matches!(below.output_flush_floor, FlushFloor::Millibels(_)));
    eprintln!(
        "C13 (b): AboveRange from {depth} levels: {:?}; {} levels: {below:?}",
        floor(depth),
        depth - 1
    );
}
