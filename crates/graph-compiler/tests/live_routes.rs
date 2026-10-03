//! Issue #1220: a send's level, on/off and 2x2 change on the running plan by the indexed ramp,
//! with no rebuild, and a settled live route renders the bits of a plan freshly prepared with its
//! new values.
//!
//! Every gate compiles a session, attaches route controls with
//! `PreparedGraphBuiltinsArtifact::attach_route_live_controls`, binds an input processor per track
//! that writes that track's own non-constant planes (the `bank_levels.rs` compile-bind-render
//! pattern) and renders. Every strip is transparent and every bus is stateless: no console slot,
//! no insert, an identity input section. A bus is observed through a unity route from its `input`
//! tap to the session output. The "fresh plan" of a comparison is the edited session compiled
//! without route controls and fed the same sources from sample 0.
//!
//! The host's output planes are pre-filled with [`HOST_SENTINEL`] before every block, so a sample
//! the plan leaves unwritten shows.

use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime,
    bounded_spsc_retained_payload,
};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphEdgeId, GraphNodeBinding, GraphNodeId,
    GraphRouteControlBinding, GraphRouteControlError, GraphRouteControlProducer,
    GraphRuntimeBindings, GraphRuntimeProcessor, LIVE_ROUTE_OWNER_BYTES, ROUTE_RAMP_LENGTH_MAXIMUM,
    RouteControlLane, RouteControlRecord, TrackStage, route_control_resources,
};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
    route_coefficients,
};
use session::{
    ChannelMatrix, CompileCaps, Console, Effect, EffectIdentity, EffectQuality, LinkMode, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source, StableId,
    Submix, compile_session, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
/// What every host output plane holds before a block.
const HOST_SENTINEL: f32 = 7.0;
/// The true-peak limiter's prepared latency at 48 kHz.
const LIMITER_LATENCY_48K: usize = 486;
/// The queue depth every gate attaches with, unless it says otherwise.
const DEPTH: usize = 8;

// ---- Sessions ---------------------------------------------------------------------------------

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
        follows_mute: false,
    }
}

fn tap(track: &str, tap: SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap,
    }
}

fn into_bus(bus: &str) -> RouteDestination {
    RouteDestination::SubmixInput {
        submix_id: sid(bus),
    }
}

/// The unity route `<bus>-main` from `bus`'s `input` tap to the session output.
fn bus_to_output(bus: &str) -> Route {
    route(
        &format!("{bus}-main"),
        RouteSource::Submix {
            submix_id: sid(bus),
            tap: SendTap::Input,
        },
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
    )
}

/// A send from `track`'s `tap` into `bus`, with drawn values.
fn send(id: &str, track: &str, at: SendTap, bus: &str, values: Values) -> Route {
    let mut send = route(id, tap(track, at), into_bus(bus));
    send.gain_db = values.gain_db;
    send.channel_matrix = ChannelMatrix {
        ll: values.matrix[0],
        lr: values.matrix[1],
        rl: values.matrix[2],
        rr: values.matrix[3],
    };
    send.mute = values.mute;
    send
}

/// The fixture with its tracks, routes, sources and console removed, at `rate`. The returned
/// track is transparent: a unity strip.
fn empty_session(rate: u32) -> (SessionModel, Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = 1 << 20;
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

fn limiter() -> Effect {
    Effect {
        id: sid("ceiling"),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// A send's values: gain, matrix `[ll, lr, rl, rr]` and mute.
#[derive(Clone, Copy, Debug)]
struct Values {
    gain_db: f32,
    matrix: [f32; 4],
    mute: bool,
}

impl Values {
    const UNITY: Self = Self {
        gain_db: 0.0,
        matrix: [1.0, 0.0, 0.0, 1.0],
        mute: false,
    };
    /// The coefficients a plan binds for these values, and a live record carries.
    fn coefficients(self) -> [f32; 4] {
        route_coefficients(self.gain_db, self.matrix, self.mute, [false; 2]).expect("in domain")
    }
    /// The record that moves a live route to these values over `length` samples.
    fn record(self, length: u32) -> RouteControlRecord {
        RouteControlRecord::new(self.coefficients(), self.mute, length).expect("a valid record")
    }
}

/// The position of route `id` among `model`'s routes in route-ID order: its index in the
/// executor's route-activity table and in the test-only route counters.
fn route_index(model: &SessionModel, id: &str) -> usize {
    let mut ids: Vec<&str> = model.routes.iter().map(|route| route.id.as_str()).collect();
    ids.sort_unstable();
    ids.iter()
        .position(|candidate| *candidate == id)
        .expect("a declared route")
}

// ---- Sources ----------------------------------------------------------------------------------

/// xorshift64*: deterministic and dependency-free.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        let mut rng = Self(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        for _ in 0..4 {
            rng.next();
        }
        rng
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    /// Uniform in `[0, 1)`.
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    fn chance(&mut self, per_mille: u64) -> bool {
        self.below(1000) < per_mille
    }
    /// A magnitude in `[0.1, 1]` with a drawn sign.
    fn coefficient(&mut self) -> f32 {
        let magnitude = 0.1 + 0.9 * self.unit();
        if self.chance(500) {
            -magnitude
        } else {
            magnitude
        }
    }
    fn values(&mut self, mute: bool) -> Values {
        Values {
            gain_db: -30.0 + 36.0 * self.unit(),
            matrix: [
                self.coefficient(),
                self.coefficient(),
                self.coefficient(),
                self.coefficient(),
            ],
            mute,
        }
    }
}

type Planes = [Vec<f32>; 2];

/// `frames` of nonzero noise in `(-0.9, 0.9)` per lane.
fn noise(seed: u64, frames: usize) -> Planes {
    let mut rng = Rng::new(seed);
    let mut plane = || {
        (0..frames)
            .map(|_| {
                let sample = 0.9 * (2.0 * rng.unit() - 1.0);
                if sample == 0.0 { 0.5 } else { sample }
            })
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

/// `frames` of strictly positive noise in `[0.05, 0.9)` per lane.
fn positive_noise(seed: u64, frames: usize) -> Planes {
    let mut rng = Rng::new(seed);
    let mut plane = || {
        (0..frames)
            .map(|_| 0.05 + 0.85 * rng.unit())
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

/// A track's input: its own planes, from the block's first sample.
struct Feed(Planes);
impl GraphRuntimeProcessor for Feed {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let start = usize::try_from(block.first_sample).expect("first sample");
        let frames = block.left.len();
        block
            .left
            .copy_from_slice(&self.0[0][start..start + frames]);
        block
            .right
            .copy_from_slice(&self.0[1][start..start + frames]);
        Ok(())
    }
}

struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

type Feeds = BTreeMap<&'static str, Planes>;

// ---- Compile, attach, bind, render ------------------------------------------------------------

fn compile(model: &SessionModel) -> PreparedGraphBuiltinsArtifact {
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
    .expect("the session compiles");
    let registry = launch_native_effect_registry().expect("launch registry");
    let effects = prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 26,
            maximum_scratch_bytes: 1 << 26,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the native effects prepare");
    let builtins = prepare_session_builtins(
        &session,
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
    .expect("the builtins prepare");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1_220,
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

/// The external bindings of `artifact`: each track's input reads its feed.
fn bindings(artifact: &PreparedGraphBuiltinsArtifact, feeds: &Feeds) -> GraphRuntimeBindings {
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::Input,
                } => Box::new(Feed(feeds[track_id.as_str()].clone())),
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    GraphRuntimeBindings {
        envelope: artifact.envelope(),
        nodes,
        observers: Vec::new(),
    }
}

/// One bound plan, its live-route producers (empty when none were attached) and its block count.
struct Bound {
    plan: PreparedRenderPlan,
    producers: Vec<GraphRouteControlProducer>,
    block: u64,
    pcm: Vec<f32>,
}

impl Bound {
    /// `model` compiled, with route controls attached when `live`, and bound to `feeds`.
    fn new(model: &SessionModel, feeds: &Feeds, live: bool) -> Self {
        let mut artifact = compile(model);
        let producers = if live {
            artifact
                .attach_route_live_controls(NonZeroUsize::new(DEPTH).expect("depth"))
                .expect("route controls attach")
        } else {
            Vec::new()
        };
        let bindings = bindings(&artifact, feeds);
        let plan = artifact
            .into_bound(bindings)
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
            .plan;
        Self {
            plan,
            producers,
            block: 0,
            pcm: vec![0.0; QUANTUM * 2],
        }
    }

    fn producer(&mut self, route: &str) -> &mut GraphRouteControlProducer {
        self.producers
            .iter_mut()
            .find(|producer| &*producer.route_id == route)
            .unwrap_or_else(|| panic!("route {route} is live"))
    }

    fn push(&mut self, route: &str, record: RouteControlRecord) {
        self.producer(route)
            .try_push(record)
            .unwrap_or_else(|full| panic!("route {route}'s queue is full: {full:?}"));
    }

    /// Renders one block into `self.pcm`, allocating nothing itself.
    fn render_in_place(&mut self) {
        self.pcm.fill(HOST_SENTINEL);
        self.plan
            .render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut self.pcm, 2, QUANTUM, QUANTUM)
                        .expect("output planes"),
                },
                RenderTime {
                    absolute_sample: self.block * QUANTUM as u64,
                },
            )
            .expect("render");
        self.block += 1;
    }

    /// Renders `blocks` blocks and returns their output planes.
    fn render(&mut self, blocks: usize) -> Planes {
        let mut out = [Vec::new(), Vec::new()];
        for _ in 0..blocks {
            self.render_in_place();
            out[0].extend_from_slice(&self.pcm[..QUANTUM]);
            out[1].extend_from_slice(&self.pcm[QUANTUM..]);
        }
        out
    }
}

/// Bits equal, NaNs folded (decision 10).
fn same(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

fn assert_bits(actual: &[f32], expected: &[f32], what: &str) {
    assert_eq!(actual.len(), expected.len(), "{what}: length");
    if let Some(index) = (0..actual.len()).find(|&index| !same(actual[index], expected[index])) {
        panic!(
            "{what}: sample {index} (block {}, frame {}): {:?} != {:?}",
            index / QUANTUM,
            index % QUANTUM,
            actual[index],
            expected[index]
        );
    }
}

fn assert_planes(actual: &Planes, expected: &Planes, range: std::ops::Range<usize>, what: &str) {
    for plane in 0..2 {
        assert_bits(
            &actual[plane][range.clone()],
            &expected[plane][range.clone()],
            &format!("{what}, plane {plane} from sample {}", range.start),
        );
    }
}

// ---- The scalar oracle -----------------------------------------------------------------------

/// The indexed ramp's `c(k)` from `start` to `target` over `length`, in plain `f32` arithmetic:
/// `c(0) = start`, `c(k) = k * step + start` with `step = (target - start) / length` below the
/// length, and `target` from it on. Rust never contracts `a * b + c`, so each operation rounds.
fn ramp_at(start: [f32; 4], target: [f32; 4], length: u32, k: u32) -> [f32; 4] {
    if k >= length {
        return target;
    }
    if k == 0 {
        return start;
    }
    let mut c = [0.0_f32; 4];
    for lane in 0..4 {
        let step = (target[lane] - start[lane]) / length as f32;
        c[lane] = k as f32 * step + start[lane];
    }
    c
}

/// One frame's 2x2 mix in the route's frozen order: `l' = (lr * r) + (ll * l)`,
/// `r' = (rr * r) + (rl * l)`.
fn mix_frame(c: [f32; 4], left: f32, right: f32) -> (f32, f32) {
    (c[1] * right + c[0] * left, c[3] * right + c[2] * left)
}

/// The unity route `<bus>-main` applied to a bus input, as the output sees it.
fn unity(left: f32, right: f32) -> (f32, f32) {
    mix_frame(Values::UNITY.coefficients(), left, right)
}

// ---- Gate 1 -----------------------------------------------------------------------------------

/// Track `a` carries a true-peak limiter and sends `a-b` into bus `b`; `c` sends `c-b` (open) and
/// `d` sends `d-b` (muted) into `b` from their `post_pan` taps, both carrying the limiter's
/// compensation. `e` sends `e-x` (muted) and `f` sends `f-x` (open) into bus `x` from their `input`
/// taps, undelayed.
fn mixed_session(rng: &mut Rng) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["a", "c", "d", "e", "f"] {
        add_track(&mut model, &source, &track, id);
    }
    model.tracks[0].inserts.effects.push(limiter());
    model.routes = vec![
        send("a-b", "a", SendTap::PostPan, "b", rng.values(false)),
        send("c-b", "c", SendTap::PostPan, "b", rng.values(false)),
        send("d-b", "d", SendTap::PostPan, "b", rng.values(true)),
        send("e-x", "e", SendTap::Input, "x", rng.values(true)),
        send("f-x", "f", SendTap::Input, "x", rng.values(false)),
        bus_to_output("b"),
        bus_to_output("x"),
    ];
    model.submixes = vec![
        Submix::unity(sid("b"), &model.console),
        Submix::unity(sid("x"), &model.console),
    ];
    model
}

fn mixed_feeds(frames: usize) -> Feeds {
    ["a", "c", "d", "e", "f"]
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, noise(1_220 + index as u64, frames)))
        .collect()
}

/// Gate 1. A session with muted, open, delayed and undelayed sends renders the same bits for 16
/// blocks with route controls attached and idle as without them, and its route ops mix the same
/// blocks (the muted undelayed send none, the muted delayed one every block).
///
/// Test value: red if an idle live route's bind-time state is not its prepared, gated
/// coefficients (for example a ramp settled on `[0.0; 4]`), or if attaching changes which routes
/// are active.
#[test]
fn idle_attached_lanes_change_nothing() {
    let mut rng = Rng::new(1);
    let model = mixed_session(&mut rng);
    let frames = 16 * QUANTUM;
    let feeds = mixed_feeds(frames);
    let mut prepared = Bound::new(&model, &feeds, false);
    let reference = prepared.render(16);
    let reference_mixes = graph::test_only_route_mix_counts();
    let mut live = Bound::new(&model, &feeds, true);
    let live_ids: Vec<&str> = live.producers.iter().map(|p| &*p.route_id).collect();
    assert_eq!(
        live_ids,
        ["a-b", "c-b", "d-b", "e-x", "f-x"],
        "every route into a submix, and only those, is live, in route-ID order"
    );
    let actual = live.render(16);
    let mixes = graph::test_only_route_mix_counts();
    assert_planes(
        &actual,
        &reference,
        0..frames,
        "idle live plan against the prepared one",
    );
    assert_eq!(
        mixes, reference_mixes,
        "attaching changes no route's activity"
    );
    assert_eq!(
        mixes[route_index(&model, "e-x")],
        0,
        "the muted undelayed send is inactive"
    );
    assert_eq!(
        mixes[route_index(&model, "d-b")],
        16,
        "the muted delayed send mixes"
    );
    assert!(
        actual[0][frames / 2..].iter().any(|sample| *sample != 0.0),
        "audio"
    );
}

// ---- Gate 2 -----------------------------------------------------------------------------------

/// Tracks `t0` and `t1` send `r0` and `r1` from their `input` taps into bus `b`.
fn two_into_bus(r0: Values, r1: Values) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["t0", "t1"] {
        add_track(&mut model, &source, &track, id);
    }
    model.routes = vec![
        send("r0", "t0", SendTap::Input, "b", r0),
        send("r1", "t1", SendTap::Input, "b", r1),
        bus_to_output("b"),
    ];
    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    model
}

fn two_feeds(frames: usize) -> Feeds {
    [("t0", noise(20, frames)), ("t1", noise(21, frames))]
        .into_iter()
        .collect()
}

const R0: Values = Values {
    gain_db: -3.0,
    matrix: [0.8, -0.3, 0.25, 0.9],
    mute: false,
};
const R1: Values = Values {
    gain_db: 2.5,
    matrix: [-0.6, 0.45, 0.7, -0.35],
    mute: false,
};

/// Gate 2 (VERIFY-1 MAJOR-1). At 48 kHz and a 128-frame quantum, `r0` receives a mute with
/// `length = 480` before block 0, so `k` reaches 480 at frame 95 of block 3. Blocks 0 to 3 equal
/// the scalar oracle with `r0` mixed by `c(k)` (block 3 whole: its ramp frames, then exact zero
/// coefficients); from block 4 `r0` mixes no more and the bus is the P4 sum without it: `r0`,
/// first in route-ID order, owns the store with `+0.0`, and `r1` is added.
///
/// Test value: red if activity is decided after the mix or mid-block, which drops frames 0-94 of
/// the fade's last block, or if a settled mute keeps mixing.
#[test]
fn the_block_in_which_a_mute_ramp_ends_is_mixed_whole() {
    let blocks = 8;
    let frames = blocks * QUANTUM;
    let feeds = two_feeds(frames);
    let model = two_into_bus(R0, R1);
    let mut live = Bound::new(&model, &feeds, true);
    let muted = Values { mute: true, ..R0 };
    live.push("r0", muted.record(480));
    let actual = live.render(blocks);
    let mixes = graph::test_only_route_mix_counts();
    assert_eq!(
        mixes[route_index(&model, "r0")],
        4,
        "r0 mixes blocks 0 to 3, then stops"
    );
    assert_eq!(mixes[route_index(&model, "r1")], blocks as u64);
    let (t0, t1) = (&feeds["t0"], &feeds["t1"]);
    let (start, open) = (R0.coefficients(), R1.coefficients());
    let mut expected = [vec![0.0; frames], vec![0.0; frames]];
    for n in 0..frames {
        let (one_l, one_r) = mix_frame(open, t1[0][n], t1[1][n]);
        let (sum_l, sum_r) = if n < 4 * QUANTUM {
            let c = ramp_at(start, [0.0; 4], 480, n as u32 + 1);
            let (zero_l, zero_r) = mix_frame(c, t0[0][n], t0[1][n]);
            (zero_l + one_l, zero_r + one_r)
        } else {
            (0.0 + one_l, 0.0 + one_r)
        };
        (expected[0][n], expected[1][n]) = unity(sum_l, sum_r);
    }
    assert_planes(
        &actual,
        &expected,
        3 * QUANTUM..4 * QUANTUM,
        "block 3, mixed whole",
    );
    assert_planes(&actual, &expected, 0..frames, "every block");
}

/// #1217 verdict-2 NIT-1, the arena form. Both sends into bus `b` are live and mixing; before
/// block 2 both are muted by a step. Bus `b`'s buffer held block 1's sum, and from block 2 every
/// input is inactive, so the sum must store `+0.0` itself: the output is exact `+0.0`.
///
/// Test value: red if the arena-form sum skips a destination whose every route input is inactive
/// (verdict mutation V3c), which leaves the stale sum of an earlier block on the bus.
#[test]
fn a_bus_whose_every_send_goes_inactive_is_refilled_with_positive_zero() {
    let blocks = 6;
    let feeds = two_feeds(blocks * QUANTUM);
    let model = two_into_bus(R0, R1);
    let mut live = Bound::new(&model, &feeds, true);
    let before = live.render(2);
    assert!(
        before[0].iter().all(|sample| *sample != 0.0),
        "the bus carries audio first"
    );
    live.push("r0", Values { mute: true, ..R0 }.record(0));
    live.push("r1", Values { mute: true, ..R1 }.record(0));
    let after = live.render(blocks - 2);
    for (plane, samples) in after.iter().enumerate() {
        if let Some(frame) = samples.iter().position(|sample| sample.to_bits() != 0) {
            panic!(
                "plane {plane} frame {frame}: {:?}, expected +0.0",
                samples[frame]
            );
        }
    }
    let mixes = graph::test_only_route_mix_counts();
    assert_eq!(mixes[route_index(&model, "r0")], 2);
    assert_eq!(mixes[route_index(&model, "r1")], 2);
}

// ---- Gate 3 -----------------------------------------------------------------------------------

/// Track `a` carries a true-peak limiter and sends `a-b` from `post_pan` into bus `b`; when
/// `with_c`, track `c` sends `c-b` from `post_pan` into `b`, so `c-b`'s edge carries the limiter's
/// compensation.
fn delayed_send(with_c: bool) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    add_track(&mut model, &source, &track, "a");
    model.tracks[0].inserts.effects.push(limiter());
    model.routes = vec![
        send("a-b", "a", SendTap::PostPan, "b", Values::UNITY),
        bus_to_output("b"),
    ];
    if with_c {
        add_track(&mut model, &source, &track, "c");
        model
            .routes
            .push(send("c-b", "c", SendTap::PostPan, "b", R0));
    }
    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    model
}

/// The compensation `artifact` inserts, asserted to be one delay on `route`'s destination edge.
fn single_route_delay(artifact: &PreparedGraphBuiltinsArtifact, route: &str) -> usize {
    let id = graph::StableGraphId::parse(route).expect("graph id");
    match artifact.graph().inserted_delays.as_slice() {
        [delay]
            if matches!(&delay.edge_id, GraphEdgeId::RouteDestination { route_id }
            if *route_id == id) =>
        {
            usize::try_from(delay.samples.0).expect("delay")
        }
        other => panic!("one delay on {route}'s edge into its bus: {other:?}"),
    }
}

/// The compensation `artifact` inserts on `route`'s destination edge.
fn route_delay(artifact: &PreparedGraphBuiltinsArtifact, route: &str) -> usize {
    let id = graph::StableGraphId::parse(route).expect("graph id");
    artifact
        .graph()
        .inserted_delays
        .iter()
        .find(|delay| {
            matches!(&delay.edge_id, GraphEdgeId::RouteDestination { route_id } if *route_id == id)
        })
        .map(|delay| usize::try_from(delay.samples.0).expect("delay"))
        .unwrap_or_else(|| panic!("{route}'s edge carries compensation"))
}

/// Gate 3 (VERIFY-1 BLOCKER-1). `c-b` carries 486 samples of compensation. Live, it is muted
/// (`length` 480) before block 0, idles muted for 10 blocks once the fade has ended, and is
/// unmuted (`length` 480) before block 14. Every output sample equals the scalar oracle: `c(k)`
/// applied to `c`'s tap, delayed 486 samples (the line starts at `+0.0`), added to `a`'s exact
/// `+0.0` in D9 order. From the unmute ramp's end plus 486 samples, the output is a fresh plan's.
///
/// `a`'s source is digital silence, and its contribution's `+0.0` is asserted from the plan
/// without `c`; `c`'s source is strictly positive, so every zero `c-b` mixes is `+0.0` and the
/// oracle's zeros have one sign.
///
/// Test value: red if a delayed route is skipped while muted (the fade still in its line is cut,
/// and on unmute stale or foreign arena samples reach the bus), or if its line is not fed its
/// zero-coefficient mix.
#[test]
fn a_delayed_send_round_trips_through_mute_without_a_cut_or_stale_audio() {
    let blocks = 26;
    let frames = blocks * QUANTUM;
    let unmute_block = 14;
    let mut feeds: Feeds = BTreeMap::new();
    feeds.insert("a", [vec![0.0; frames], vec![0.0; frames]]);
    feeds.insert("c", positive_noise(30, frames));

    let alone = delayed_send(false);
    let silent = Bound::new(&alone, &feeds, false).render(blocks);
    assert!(
        silent.iter().flatten().all(|sample| sample.to_bits() == 0),
        "a's contribution through its limiter is exact +0.0"
    );

    let model = delayed_send(true);
    let delay = single_route_delay(&compile(&model), "c-b");
    assert_eq!(
        delay, LIMITER_LATENCY_48K,
        "c-b carries the limiter's compensation"
    );

    let mut live = Bound::new(&model, &feeds, true);
    let open = R0.coefficients();
    live.push("c-b", Values { mute: true, ..R0 }.record(480));
    let mut actual = live.render(unmute_block);
    live.push("c-b", R0.record(480));
    let rest = live.render(blocks - unmute_block);
    for plane in 0..2 {
        actual[plane].extend_from_slice(&rest[plane]);
    }
    let mixes = graph::test_only_route_mix_counts();
    assert_eq!(
        mixes[route_index(&model, "c-b")],
        blocks as u64,
        "the delayed send mixes every block, muted or not"
    );

    let c = &feeds["c"];
    let unmute = unmute_block * QUANTUM;
    let mut mixed = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for n in 0..frames {
        let coefficients = if n < unmute {
            ramp_at(open, [0.0; 4], 480, n as u32 + 1)
        } else {
            // The unmute ramps from where the settled mute stands: exact zeros.
            ramp_at([0.0; 4], open, 480, (n - unmute) as u32 + 1)
        };
        (mixed[0][n], mixed[1][n]) = mix_frame(coefficients, c[0][n], c[1][n]);
    }
    let mut expected = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for n in 0..frames {
        let (delayed_l, delayed_r) = if n < delay {
            (0.0, 0.0)
        } else {
            (mixed[0][n - delay], mixed[1][n - delay])
        };
        (expected[0][n], expected[1][n]) = unity(0.0 + delayed_l, 0.0 + delayed_r);
    }
    assert_planes(
        &actual,
        &expected,
        0..frames,
        "the round trip against the scalar oracle",
    );
    // The fade's tail reaches the bus after the route settled muted: the line was not cut.
    assert!(
        actual[0][470 + delay..479 + delay]
            .iter()
            .all(|sample| *sample != 0.0),
        "the fade's last frames arrive 486 samples later"
    );

    let fresh = Bound::new(&model, &feeds, false).render(blocks);
    let settled = unmute + 480 + delay;
    assert_planes(
        &actual,
        &fresh,
        settled..frames,
        "settled, against a fresh plan",
    );
}

// ---- Gate 4 -----------------------------------------------------------------------------------

/// Track `a` carries a true-peak limiter and sends `a-b` into bus `b`; `c` sends `c-b` into `b`
/// (delayed); `e` and `f` send `e-x` and `f-x` into bus `x` (undelayed). All from `post_pan`.
fn two_buses(rate: u32, values: &BTreeMap<&str, Values>) -> SessionModel {
    let (mut model, source, track) = empty_session(rate);
    for id in ["a", "c", "e", "f"] {
        add_track(&mut model, &source, &track, id);
    }
    model.tracks[0].inserts.effects.push(limiter());
    model.routes = vec![
        send("a-b", "a", SendTap::PostPan, "b", values["a-b"]),
        send("c-b", "c", SendTap::PostPan, "b", values["c-b"]),
        send("e-x", "e", SendTap::PostPan, "x", values["e-x"]),
        send("f-x", "f", SendTap::PostPan, "x", values["f-x"]),
        bus_to_output("b"),
        bus_to_output("x"),
    ];
    model.submixes = vec![
        Submix::unity(sid("b"), &model.console),
        Submix::unity(sid("x"), &model.console),
    ];
    model
}

const LENGTHS: [u32; 5] = [0, 1, 37, 480, 4800];

/// Gate 4. Per trial, random gain, matrix and mute changes to the delayed `c-b` and the undelayed
/// `e-x` (and, every other trial, the undelayed `a-b`), with targets from
/// `graph_compiler::route_coefficients` and every length in {0, 1, 37, 480, 4800}, are pushed
/// before block 2. Every block after the longest ramp, plus the compensation delay, is
/// bit-identical to a fresh plan of the edited session. At 48 kHz and 44.1 kHz.
///
/// Test value: red if the settled coefficients are computed rather than assigned, if a record
/// reaches the wrong route, or if a muted route leaves a contribution the prepared plan does not
/// have.
#[test]
fn a_settled_live_edit_equals_a_fresh_plan() {
    let mut rng = Rng::new(4);
    for rate in [48_000, 44_100] {
        for trial in 0..10 {
            let mut values: BTreeMap<&str, Values> = ["a-b", "c-b", "e-x", "f-x"]
                .into_iter()
                .map(|id| (id, rng.values(false)))
                .collect();
            let model = two_buses(rate, &values);
            let edited: Vec<&str> = if trial % 2 == 0 {
                vec!["c-b", "e-x", "a-b"]
            } else {
                vec!["c-b", "e-x"]
            };
            let mut lengths = BTreeMap::new();
            for (offset, id) in edited.iter().enumerate() {
                let mute = rng.chance(300);
                values.insert(id, rng.values(mute));
                lengths.insert(*id, LENGTHS[(trial + offset) % LENGTHS.len()]);
            }
            let edit = two_buses(rate, &values);
            let delay = route_delay(&compile(&model), "c-b");
            let longest = lengths.values().copied().max().unwrap_or(0) as usize;
            let push_block = 2;
            let settled = push_block * QUANTUM + longest + delay;
            let blocks = settled.div_ceil(QUANTUM) + 3;
            let frames = blocks * QUANTUM;
            let feeds: Feeds = ["a", "c", "e", "f"]
                .iter()
                .enumerate()
                .map(|(index, id)| (*id, noise(400 + index as u64, frames)))
                .collect();
            let mut live = Bound::new(&model, &feeds, true);
            let mut actual = live.render(push_block);
            for id in &edited {
                live.push(id, values[id].record(lengths[id]));
            }
            let rest = live.render(blocks - push_block);
            for plane in 0..2 {
                actual[plane].extend_from_slice(&rest[plane]);
            }
            let fresh = Bound::new(&edit, &feeds, false).render(blocks);
            assert_planes(
                &actual,
                &fresh,
                settled..frames,
                &format!("{rate} Hz trial {trial}, lengths {lengths:?}, values {values:?}"),
            );
            assert!(
                actual[0][settled..].iter().any(|sample| *sample != 0.0),
                "audio"
            );
        }
    }
}

// ---- Gate 5 -----------------------------------------------------------------------------------

/// Gate 5. The queue is bounded and the drain lossless:
///
/// * `depth + 1` pushes without a render: the last is refused with its record handed back, and
///   the queue holds exactly `depth`;
/// * one render applies exactly those `depth` records (the drained-record counter), in order: the
///   output equals a plan sent only the last of them, and differs from one sent them reversed;
/// * the refused record, pushed again, is applied at the next block;
/// * a record pushed while the route is inactive is applied at the next block: an unmute after a
///   settled mute is audible there, with a fresh unmuted plan's bits;
/// * `RouteControlRecord::new` refuses `length = 2^22 + 1` and a mute with a nonzero target.
///
/// Test value: red if the drain drops or reorders a record, or is skipped while the route is
/// inactive. It cannot see an unbounded drain (single-threaded, that applies exactly what a bounded
/// one does); the bound is structural: the drain loops over `available_at_entry()`, at most the
/// queue's capacity (#1220 verdict MINOR-2).
#[test]
fn the_drain_is_bounded_and_lossless() {
    let blocks = 8;
    let frames = blocks * QUANTUM;
    let feeds = two_feeds(frames);
    let model = two_into_bus(R0, R1);
    let r0 = route_index(&model, "r0");
    let mut rng = Rng::new(5);
    let records: Vec<RouteControlRecord> = (0..=DEPTH)
        .map(|index| rng.values(false).record(37 + 50 * index as u32))
        .collect();

    let mut live = Bound::new(&model, &feeds, true);
    for record in &records[..DEPTH] {
        live.push("r0", *record);
    }
    assert_eq!(
        live.producer("r0").free(),
        0,
        "the queue holds exactly depth records"
    );
    let refused = live.producer("r0").try_push(records[DEPTH]);
    assert_eq!(
        refused.map_err(|full| full.record),
        Err(records[DEPTH]),
        "the record past depth is refused and handed back"
    );
    let first = live.render(1);
    assert_eq!(graph::test_only_route_drained_counts()[r0], DEPTH as u64);
    assert_eq!(live.producer("r0").free(), DEPTH);

    let mut last_only = Bound::new(&model, &feeds, true);
    last_only.push("r0", records[DEPTH - 1]);
    let expected = last_only.render(1);
    assert_planes(
        &first,
        &expected,
        0..QUANTUM,
        "depth records in one drain against the last",
    );
    let mut reversed = Bound::new(&model, &feeds, true);
    for record in records[..DEPTH].iter().rev() {
        reversed.push("r0", *record);
    }
    let reversed = reversed.render(1);
    assert!(
        (0..QUANTUM).any(|frame| !same(reversed[0][frame], first[0][frame])),
        "the order of a drain is observable, or the in-order check is vacuous"
    );

    live.push("r0", records[DEPTH]);
    live.render(1);
    assert_eq!(
        graph::test_only_route_drained_counts()[r0],
        DEPTH as u64 + 1,
        "a record that arrives later is applied in a later block"
    );

    // An unmute pushed while the route is inactive.
    let mut live = Bound::new(&model, &feeds, true);
    live.push("r0", Values { mute: true, ..R0 }.record(0));
    let mut actual = live.render(3);
    assert_eq!(
        graph::test_only_route_mix_counts()[r0],
        0,
        "the muted send is inactive"
    );
    live.push("r0", R0.record(0));
    let unmuted = live.render(blocks - 3);
    for plane in 0..2 {
        actual[plane].extend_from_slice(&unmuted[plane]);
    }
    assert_eq!(graph::test_only_route_mix_counts()[r0], (blocks - 3) as u64);
    let fresh = Bound::new(&model, &feeds, false).render(blocks);
    assert_planes(
        &actual,
        &fresh,
        3 * QUANTUM..frames,
        "audible from the next block",
    );

    assert_eq!(
        RouteControlRecord::new([0.5; 4], false, ROUTE_RAMP_LENGTH_MAXIMUM + 1),
        None
    );
    assert!(RouteControlRecord::new([0.5; 4], false, ROUTE_RAMP_LENGTH_MAXIMUM).is_some());
    assert_eq!(
        RouteControlRecord::new([0.0, 0.0, -0.0, 0.0], true, 0),
        None
    );
    assert_eq!(
        RouteControlRecord::new([0.0, 0.25, 0.0, 0.0], true, 10),
        None
    );
    assert!(RouteControlRecord::new([0.0; 4], true, 10).is_some());
}

// ---- Gate 6 -----------------------------------------------------------------------------------

/// Gate 6. A producer thread pushes records to every live route of gate 1's session while this
/// thread renders, and every block applies at least one of them; after the first block, every render allocates and frees nothing on the render
/// thread (`bench_support::alloc`'s thread-scoped counters).
///
/// Test value: red if the drain, a record's application or the activity write allocates or frees
/// on the render thread.
#[test]
fn live_routes_render_without_allocating() {
    let mut rng = Rng::new(6);
    let model = mixed_session(&mut rng);
    let blocks = 64;
    let feeds = mixed_feeds(blocks * QUANTUM);
    let mut live = Bound::new(&model, &feeds, true);
    let mut producers = core::mem::take(&mut live.producers);
    let records: Vec<RouteControlRecord> = (0..32)
        .map(|index| {
            rng.values(index % 3 == 0)
                .record(LENGTHS[index % LENGTHS.len()])
        })
        .collect();
    let done = AtomicBool::new(false);
    let pushed = AtomicUsize::new(0);
    assert_installed();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let mut index = 0;
            while !done.load(Ordering::Acquire) {
                for producer in &mut producers {
                    if producer.try_push(records[index % records.len()]).is_ok() {
                        pushed.fetch_add(1, Ordering::Release);
                    }
                    index += 1;
                }
                std::thread::yield_now();
            }
        });
        let mut seen = 0;
        for block in 0..blocks {
            // Every block drains at least one record the other thread pushed since the last one.
            while pushed.load(Ordering::Acquire) == seen {
                std::thread::yield_now();
            }
            seen = pushed.load(Ordering::Acquire);
            let before: u64 = graph::test_only_route_drained_counts().iter().sum();
            let mark = current_thread_counters();
            live.render_in_place();
            let delta = current_thread_delta_since(mark);
            let after: u64 = graph::test_only_route_drained_counts().iter().sum();
            assert!(after > before, "block {block} applied a record");
            if block > 0 {
                assert_eq!(
                    (delta.allocations, delta.reallocations, delta.deallocations),
                    (0, 0, 0),
                    "block {block}: the render thread allocated or freed"
                );
            }
        }
        done.store(true, Ordering::Release);
    });
}

// ---- Gate 7 -----------------------------------------------------------------------------------

/// Eight tracks feed bus `bus` from `post_pan`, and the bus feeds the output from its `pre_fader`
/// tap, so the bus input is the plan's one fold candidate (#1216 gate 4's shape).
fn eight_into_one(sends: &[Values; 8]) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    for (index, values) in sends.iter().enumerate() {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        model.routes.push(send(
            &format!("{id}-bus"),
            &id,
            SendTap::PostPan,
            "bus",
            *values,
        ));
    }
    model.routes.push(route(
        "bus-main",
        RouteSource::Submix {
            submix_id: sid("bus"),
            tap: SendTap::PreFader,
        },
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
    ));
    model.submixes = vec![Submix::unity(sid("bus"), &model.console)];
    model
}

fn eight_feeds(frames: usize) -> Feeds {
    const IDS: [&str; 8] = ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7"];
    IDS.iter()
        .enumerate()
        .map(|(index, id)| (*id, noise(70 + index as u64, frames)))
        .collect()
}

/// Gate 7 (VERIFY-2 M3). With route controls attached the eight-into-one bus folds nothing
/// (`bank_route_folds() == 0`; without them the same plan folds), and a gain record on `t3-bus`
/// moves the output to a fresh plan's bits once its ramp ends.
///
/// Test value: red if the fold planner folds a live route, which would never drain its queue.
#[test]
fn no_live_route_folds() {
    let blocks = 12;
    let frames = blocks * QUANTUM;
    let feeds = eight_feeds(frames);
    let mut rng = Rng::new(7);
    let mut sends = [Values::UNITY; 8];
    for values in &mut sends {
        *values = rng.values(false);
    }
    let model = eight_into_one(&sends);
    let prepared = Bound::new(&model, &feeds, false);
    let prepared_folds = prepared.plan.bank_route_folds();
    assert!(
        prepared_folds > 0,
        "without route controls the bus folds, or this is vacuous"
    );
    let mut live = Bound::new(&model, &feeds, true);
    assert_eq!(live.producers.len(), 8);
    assert_eq!(live.plan.bank_route_folds(), 0, "no live route folds");
    let mut actual = live.render(1);
    sends[3] = rng.values(false);
    live.push("t3-bus", sends[3].record(480));
    let rest = live.render(blocks - 1);
    for plane in 0..2 {
        actual[plane].extend_from_slice(&rest[plane]);
    }
    let fresh = Bound::new(&eight_into_one(&sends), &feeds, false).render(blocks);
    assert_planes(
        &actual,
        &fresh,
        QUANTUM + 480..frames,
        "after t3-bus's ramp",
    );
    eprintln!("live_routes gate 7: the prepared bus folds {prepared_folds} lanes, the live one 0");
}

// ---- Gate 8 -----------------------------------------------------------------------------------

/// What one compile, optional attach and bind of `model` left live on this thread, and the
/// attach's producers.
fn retained(
    model: &SessionModel,
    feeds: &Feeds,
    live: bool,
) -> (u64, Vec<GraphRouteControlProducer>) {
    assert_installed();
    let mark = current_thread_counters();
    let mut artifact = compile(model);
    let producers = if live {
        artifact
            .attach_route_live_controls(NonZeroUsize::new(DEPTH).expect("depth"))
            .expect("attach")
    } else {
        Vec::new()
    };
    let bindings = bindings(&artifact, feeds);
    let bound = artifact
        .into_bound(bindings)
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
    let window = current_thread_delta_since(mark);
    drop(bound);
    let live_bytes = window
        .requested_bytes
        .checked_sub(window.released_bytes)
        .expect("the window frees nothing it did not allocate");
    (live_bytes, producers)
}

/// Gate 8 (VERIFY-2 MINOR 5). `route_control_resources` equals the D8 formula computed here from
/// the route count, the queue depth and the route IDs, and covers what the attach and the bind
/// retain beyond the same plan without route controls. Once for a plan with no silencing gate,
/// whose route-activity table the attach charges, and once with a muted route, whose table the
/// compile-time estimate already charged.
///
/// Test value: red if a queue, a lane or owner box, a route ID or an uncharged route-activity table
/// is left out of the charge the next slice admits against the host's caps.
#[test]
fn route_control_resources_cover_the_allocation() {
    let feeds = two_feeds(QUANTUM);
    for muted in [false, true] {
        let model = two_into_bus(R0, Values { mute: muted, ..R1 });
        let (without, _) = retained(&model, &feeds, false);
        let (with, producers) = retained(&model, &feeds, true);
        let resources = route_control_resources(&producers);

        let routes = producers.len() as u64;
        assert_eq!(routes, 2);
        let payload = bounded_spsc_retained_payload::<RouteControlRecord>(
            NonZeroUsize::new(DEPTH).expect("depth"),
        )
        .expect("layout");
        let queue = payload.total_bytes().expect("queue bytes") as u64;
        let owner = (core::mem::size_of::<RouteControlLane>()
            + core::mem::size_of::<GraphRouteControlBinding>()
            + LIVE_ROUTE_OWNER_BYTES) as u64;
        // The producer's copy and the binding's node-ID copy (#1220 verdict MINOR-1).
        let ids: u64 = producers.iter().map(|p| 2 * p.route_id.len() as u64).sum();
        let table = routes * core::mem::size_of::<GraphRouteControlProducer>() as u64;
        let artifact = compile(&model);
        let nodes = artifact.graph().spec.nodes.len() as u64;
        let prepared_routes = artifact.graph().routes().len() as u64;
        let activity = if muted {
            0
        } else {
            graph::route_activity_bound_bytes(prepared_routes, nodes).expect("bound")
        };
        assert_eq!(resources.routes, routes);
        assert_eq!(resources.queue_bytes, routes * queue);
        assert_eq!(resources.owner_bytes, routes * owner);
        assert_eq!(resources.producer_table_bytes, table);
        assert_eq!(resources.route_id_bytes, ids);
        assert_eq!(resources.activity_bytes, activity);
        assert_eq!(
            resources.total_bytes,
            routes * queue + routes * owner + table + ids + activity
        );
        assert!(resources.largest_allocation_bytes >= payload.largest_allocation_bytes() as u64);

        let added = with
            .checked_sub(without)
            .filter(|bytes| *bytes > 0)
            .unwrap_or_else(|| panic!("attaching retains more: {with} against {without}"));
        assert!(
            resources.total_bytes >= added,
            "muted {muted}: the charge {} covers the {added} bytes attach and bind retain",
            resources.total_bytes
        );
        eprintln!(
            "live_routes gate 8, muted {muted}: charged {}, retained {added}",
            resources.total_bytes
        );
    }
}

/// Gate 8, the attached state (#1220 verdict MINOR-1). Between the attach and the bind the plan
/// holds each live route's [`GraphRouteControlBinding`], whose node ID is a second heap copy of the
/// route ID; the charge must cover that state too, which only long route IDs make visible. A muted
/// route (whose activity table the compile-time estimate already charged, so the attach adds none)
/// and 127-byte route IDs: what the attach alone retains is within the charge.
///
/// Test value: red if the charge counts each route ID once, leaving the binding's copy out of the
/// number host-core admits while it holds the attached artifact (gate 8 above measures only after
/// bind, which frees the bindings).
#[test]
fn route_control_resources_cover_the_attached_state() {
    let feeds = two_feeds(QUANTUM);
    let mut model = two_into_bus(R0, Values { mute: true, ..R1 });
    let long = [
        format!("r{}", "a".repeat(126)),
        format!("r{}", "b".repeat(126)),
    ];
    for (route, id) in model.routes.iter_mut().zip(&long) {
        assert_eq!(id.len(), 127);
        route.id = sid(id);
    }
    assert_installed();
    let mut artifact = compile(&model);
    let mark = current_thread_counters();
    let producers = artifact
        .attach_route_live_controls(NonZeroUsize::new(DEPTH).expect("depth"))
        .expect("attach");
    let window = current_thread_delta_since(mark);
    let attached = window
        .requested_bytes
        .checked_sub(window.released_bytes)
        .expect("the attach frees nothing it did not allocate");
    let resources = route_control_resources(&producers);
    assert_eq!(producers.len(), 2);
    assert_eq!(
        resources.activity_bytes, 0,
        "the muted plan's table is pre-charged"
    );
    assert!(
        resources.total_bytes >= attached,
        "the charge {} covers the {attached} bytes the attach retains",
        resources.total_bytes
    );
    let bindings = bindings(&artifact, &feeds);
    let bound = artifact
        .into_bound(bindings)
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
    drop(bound);
}

/// D7: a second attach is refused, and a plan with no route into a submix attaches no lane.
#[test]
fn route_controls_attach_once() {
    let model = two_into_bus(R0, R1);
    let mut artifact = compile(&model);
    let depth = NonZeroUsize::new(DEPTH).expect("depth");
    assert_eq!(
        artifact.attach_route_live_controls(depth).map(|p| p.len()),
        Ok(2)
    );
    assert_eq!(
        artifact.attach_route_live_controls(depth).map(|p| p.len()),
        Err(GraphRouteControlError::AlreadyAttached)
    );
}
