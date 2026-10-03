//! Issue #1221: host-core produces live send records.
//!
//! With a control queue depth, preparation attaches one live lane per route into a submix and
//! hands back one [`RouteControlProducer`] per lane, in canonical route-ID order. A producer
//! builds its record's target with `graph_compiler::route_coefficients`, refuses before it pushes,
//! and its lanes are charged against the host's caps.
//!
//! Every rendered comparison has a stateless downstream: the buses are unity submixes (no console
//! slot, no insert, HPF and LPF off), and every track reads its own non-constant two-lane noise.
//! The settled comparisons run from the push block plus the ramp length plus the plan's output
//! latency, which bounds every route's compensation delay.

use core::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use dsp_reference::randomized::{Draw, first_difference};
use effect_compiler::{EffectCompileCaps, launch_native_effect_registry};
use graph::GraphCompileCaps;
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler, route_coefficients};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy,
    PrepareRejection, PreparedHost, RouteControlError, RouteControlProducer, RouteControlResources,
    SourceSubmission, compile_host_session, compiled_session_shape,
    prepare_host_session_with_live_controls,
};
use session::{
    ChannelMatrix, Console, Effect, EffectIdentity, EffectQuality, LinkMode, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source, StableId,
    Submix, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const CONSOLE_FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");
const QUANTUM: usize = 128;
/// What every host output plane holds before a block.
const HOST_SENTINEL: f32 = 7.0;
/// The depth every gate attaches with, unless it says otherwise.
const DEPTH: usize = 8;
/// One past the indexed ramp's longest length.
const TOO_LONG: u32 = (1 << 22) + 1;

type Planes = [Vec<f32>; 2];
/// Each source's id and its planes.
type Feeds = Vec<(String, Planes)>;

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
        maximum_effects: 10_000,
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

fn request(depth: Option<usize>) -> HostLiveControlRequest {
    HostLiveControlRequest {
        control_queue_depth: depth.map(|depth| NonZeroUsize::new(depth).expect("depth")),
        ..HostLiveControlRequest::default()
    }
}

// ---- Sessions -----------------------------------------------------------------------------------

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
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
}

fn route(id: &str, source: RouteSource, destination: RouteDestination, values: Values) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: ChannelMatrix {
            ll: values.matrix[0],
            lr: values.matrix[1],
            rl: values.matrix[2],
            rr: values.matrix[3],
        },
        gain_db: values.gain_db,
        mute: values.mute,
        follows_mute: false,
    }
}

fn tap(track: &str, tap: SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap,
    }
}

/// A send from `track`'s `at` tap into bus `bus`.
fn send(id: &str, track: &str, at: SendTap, bus: &str, values: Values) -> Route {
    route(
        id,
        tap(track, at),
        RouteDestination::SubmixInput {
            submix_id: sid(bus),
        },
        values,
    )
}

/// The unity route `<from>-main` to the session output.
fn to_output(id: &str, source: RouteSource) -> Route {
    route(
        id,
        source,
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
        Values::UNITY,
    )
}

/// The unity route `<bus>-main` from `bus`'s `input` tap to the session output.
fn bus_to_output(bus: &str) -> Route {
    to_output(
        &format!("{bus}-main"),
        RouteSource::Submix {
            submix_id: sid(bus),
            tap: SendTap::Input,
        },
    )
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

fn add_buses(model: &mut SessionModel, buses: &[&str]) {
    for bus in buses {
        model.submixes.push(Submix::unity(sid(bus), &model.console));
    }
}

fn document(model: &SessionModel) -> String {
    canonical_session_json(model).expect("generated session canonicalizes")
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

/// The position of route `id` among `model`'s routes in route-ID order: its index in the
/// executor's route-activity table and in the test-only route counters.
fn route_index(model: &SessionModel, id: &str) -> usize {
    let mut ids: Vec<&str> = model.routes.iter().map(|route| route.id.as_str()).collect();
    ids.sort_unstable();
    ids.iter()
        .position(|candidate| *candidate == id)
        .expect("a declared route")
}

// ---- Draws --------------------------------------------------------------------------------------

/// A matrix coefficient: a magnitude in `[0.1, 1]` with a drawn sign, or now and then a signed
/// zero.
fn coefficient(draw: &mut Draw) -> f32 {
    if draw.chance(1, 10) {
        return if draw.chance(1, 2) { -0.0 } else { 0.0 };
    }
    let magnitude = 0.1 + 0.9 * draw.unit();
    if draw.chance(1, 2) {
        -magnitude
    } else {
        magnitude
    }
}

/// In-domain values: a gain in `[-60, +12)` dB and a drawn matrix.
fn values(draw: &mut Draw, mute: bool) -> Values {
    Values {
        gain_db: -60.0 + 72.0 * draw.unit(),
        matrix: [
            coefficient(draw),
            coefficient(draw),
            coefficient(draw),
            coefficient(draw),
        ],
        mute,
    }
}

/// `frames` of nonzero noise in `(-0.9, 0.9)` per lane, the two lanes distinct.
fn noise(draw: &mut Draw, frames: usize) -> Planes {
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

fn feeds(draw: &mut Draw, ids: &[&str], frames: usize) -> Feeds {
    ids.iter()
        .map(|id| ((*id).to_owned(), noise(draw, frames)))
        .collect()
}

fn assert_planes(actual: &Planes, expected: &Planes, from: usize, what: &str) {
    for plane in 0..2 {
        if let Some(index) = first_difference(&actual[plane][from..], &expected[plane][from..]) {
            let frame = from + index;
            panic!(
                "{what}: plane {plane} frame {frame}: {:?} != {:?}",
                actual[plane][frame], expected[plane][frame]
            );
        }
    }
}

// ---- One prepared host ------------------------------------------------------------------------

/// One host prepared through host-core, its live handles and the feeds it renders.
struct Live {
    host: PreparedHost,
    handles: HostLiveControlHandles,
    feeds: Feeds,
    block: usize,
    pcm: Vec<f32>,
}

impl Live {
    fn new(model: &SessionModel, feeds: &Feeds, depth: Option<usize>) -> Self {
        Self::from_document(&document(model), feeds, depth)
    }

    fn from_document(document: &str, feeds: &Feeds, depth: Option<usize>) -> Self {
        let (_, host, handles) =
            prepare_host_session_with_live_controls(document, &caps(), &request(depth))
                .unwrap_or_else(|failure| {
                    panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
                });
        Self {
            host,
            handles,
            feeds: feeds.clone(),
            block: 0,
            pcm: vec![0.0; QUANTUM * 2],
        }
    }

    fn producer(&mut self, route: &str) -> &mut RouteControlProducer {
        self.handles
            .route_controls
            .iter_mut()
            .find(|producer| producer.route_id() == route)
            .unwrap_or_else(|| panic!("route {route} is live"))
    }

    /// Submits the next block of every feed.
    fn submit(&mut self) {
        let range = self.block * QUANTUM..(self.block + 1) * QUANTUM;
        for (id, planes) in &self.feeds {
            self.host
                .sources
                .submit(
                    id.as_bytes(),
                    SourceSubmission {
                        generation: 1,
                        start_frame: range.start as u64,
                        sample_rate_hz: self.host.report.sample_rate_hz,
                        planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
        }
    }

    /// Renders one block into `self.pcm`; the render itself allocates nothing here.
    fn render_block(&mut self) {
        self.pcm.fill(HOST_SENTINEL);
        self.host
            .plan
            .render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(
                        &mut self.pcm,
                        2,
                        QUANTUM,
                        QUANTUM,
                    )
                    .expect("output planes"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: (self.block * QUANTUM) as u64,
                },
            )
            .expect("render");
        self.block += 1;
    }

    /// Renders `blocks` blocks and appends their output planes to `out`.
    fn render_into(&mut self, blocks: usize, out: &mut Planes) {
        for _ in 0..blocks {
            self.submit();
            self.render_block();
            out[0].extend_from_slice(&self.pcm[..QUANTUM]);
            out[1].extend_from_slice(&self.pcm[QUANTUM..]);
        }
    }

    fn render(&mut self, blocks: usize) -> Planes {
        let mut out = [Vec::new(), Vec::new()];
        self.render_into(blocks, &mut out);
        out
    }
}

// ---- Gate 1 -------------------------------------------------------------------------------------

/// Track `t` sends `t-b` from its `pre_fader` tap into bus `b` with `values`, following its mute
/// when `follow`, its fader lanes muted as `lane_muted`; track `u` sends `u-b` (open) into `b`;
/// `b` feeds the output.
fn follow_session(values: Values, follow: bool, lane_muted: [bool; 2], u: Values) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    add_track(&mut model, &source, &track, "t");
    add_track(&mut model, &source, &track, "u");
    model.tracks[0].fader.left_mute = lane_muted[0];
    model.tracks[0].fader.right_mute = lane_muted[1];
    let mut t_b = send("t-b", "t", SendTap::PreFader, "b", values);
    t_b.follows_mute = follow;
    model.routes = vec![
        t_b,
        send("u-b", "u", SendTap::PreFader, "b", u),
        bus_to_output("b"),
    ];
    add_buses(&mut model, &["b"]);
    model
}

/// Gate 1. For random in-domain gains and matrices, mutes and source-lane mutes, the record a
/// producer builds carries exactly the bits of `graph_compiler::route_coefficients` for the same
/// arguments, and its mute is the gate's silence; and pushed live, it settles on the same bits a
/// plan prepared from a session holding those values (with `follows_mute` and the source strip's
/// lane mutes) renders.
///
/// Test value: red if the producer computes its target, or its follow-zeroed columns, by any path
/// other than the compiler's.
#[test]
fn the_live_target_is_the_prepared_constant() {
    let mut draw = Draw::new(1_221_001);
    let initial = values(&mut draw, false);
    let u = values(&mut draw, false);
    let base = follow_session(initial, false, [false; 2], u);
    let live = Live::new(&base, &Vec::new(), Some(DEPTH));
    let producer = &live.handles.route_controls[0];
    assert_eq!(producer.route_id(), "t-b");
    for _ in 0..2_000 {
        let mute = draw.chance(1, 5);
        let drawn = values(&mut draw, mute);
        let lanes = [draw.chance(2, 5), draw.chance(2, 5)];
        let length = draw.pick(&[0, 1, 37, 480]);
        let record = producer
            .record(drawn.gain_db, drawn.matrix, drawn.mute, lanes, length)
            .expect("in domain");
        let expected =
            route_coefficients(drawn.gain_db, drawn.matrix, drawn.mute, lanes).expect("in domain");
        assert_eq!(
            record.target().map(f32::to_bits),
            expected.map(f32::to_bits),
            "{drawn:?} {lanes:?}"
        );
        assert_eq!(record.mute(), drawn.mute || lanes == [true, true]);
        assert_eq!(record.length(), length);
    }
    drop(live);

    for trial in 0..12 {
        let drawn = values(&mut draw, trial % 4 == 3);
        let lanes = match trial % 4 {
            0 => [true, false],
            1 => [false, true],
            2 => [true, true],
            _ => [draw.chance(1, 2), draw.chance(1, 2)],
        };
        let length: u32 = draw.pick(&[0, 37, 480]);
        let push_block = 2;
        let settled = push_block * QUANTUM + length as usize;
        let blocks = settled.div_ceil(QUANTUM) + 3;
        let feeds = feeds(&mut draw, &["t", "u"], blocks * QUANTUM);
        let mut live = Live::new(&base, &feeds, Some(DEPTH));
        let mut actual = live.render(push_block);
        live.producer("t-b")
            .set(drawn.gain_db, drawn.matrix, drawn.mute, lanes, length)
            .expect("queued");
        live.render_into(blocks - push_block, &mut actual);
        let fresh = Live::new(&follow_session(drawn, true, lanes, u), &feeds, None).render(blocks);
        assert_planes(
            &actual,
            &fresh,
            settled,
            &format!("trial {trial}: {drawn:?} lanes {lanes:?} length {length}"),
        );
        assert!(actual[0][settled..].iter().any(|sample| *sample != 0.0));
    }
}

// ---- Gate 2 -------------------------------------------------------------------------------------

/// Track `a` carries a true-peak limiter and sends `a-b` into bus `b`; `c` sends `c-b` into `b`
/// (so `c-b` carries `a`'s compensation); `e` and `f` send `e-x` and `f-x` into bus `x`. All from
/// `post_pan`. Both buses feed the output.
fn two_buses(rate: u32, sends: &[(&str, Values); 4]) -> SessionModel {
    let (mut model, source, track) = empty_session(rate);
    for id in ["a", "c", "e", "f"] {
        add_track(&mut model, &source, &track, id);
    }
    model.tracks[0].inserts.effects.push(limiter());
    for (id, values) in sends {
        let (from, bus) = id.split_once('-').expect("send id");
        model
            .routes
            .push(send(id, from, SendTap::PostPan, bus, *values));
    }
    model.routes.push(bus_to_output("b"));
    model.routes.push(bus_to_output("x"));
    add_buses(&mut model, &["b", "x"]);
    model
}

/// Gate 2. Through `prepare_host_session_with_live_controls`, random gain, matrix and mute edits
/// on `c-b` (delayed, into `b`) and `e-x` (undelayed, into `x`), each with `length` 0 or 480, are
/// pushed before block 2; from the longest ramp's end plus the plan's latency, the output is
/// bit-identical to a host prepared from the edited session and fed the same sources from sample
/// 0. At 48 kHz and 44.1 kHz.
///
/// Each send is addressed as a host addresses it: by its index in `route_controls`.
///
/// Test value: red if host-core attaches lanes to the wrong routes, in the wrong order, or pushes
/// stale values.
#[test]
fn a_settled_live_edit_through_host_core_equals_a_fresh_plan() {
    let mut draw = Draw::new(1_221_002);
    for rate in [48_000, 44_100] {
        for trial in 0..6 {
            let mut sends = [
                ("a-b", values(&mut draw, false)),
                ("c-b", values(&mut draw, false)),
                ("e-x", values(&mut draw, false)),
                ("f-x", values(&mut draw, false)),
            ];
            let model = two_buses(rate, &sends);
            let lengths: [u32; 2] = [[0, 480], [480, 0], [480, 480], [0, 0]][trial % 4];
            for index in [1, 2] {
                let mute = draw.chance(3, 10);
                sends[index].1 = values(&mut draw, mute);
            }
            let edit = two_buses(rate, &sends);
            let push_block = 2;
            let feed_ids = ["a", "c", "e", "f"];
            let probe = Live::new(&model, &Vec::new(), Some(DEPTH));
            let latency = usize::try_from(probe.host.report.latency_samples).expect("latency");
            assert!(latency > 0, "c-b carries compensation");
            drop(probe);
            let settled = push_block * QUANTUM + lengths[0].max(lengths[1]) as usize + latency;
            let blocks = settled.div_ceil(QUANTUM) + 3;
            let feeds = feeds(&mut draw, &feed_ids, blocks * QUANTUM);
            let mut live = Live::new(&model, &feeds, Some(DEPTH));
            let mut actual = live.render(push_block);
            // A host addresses a live send by its index in `route_controls`, which is canonical
            // route-ID order: `a-b`, `c-b`, `e-x`, `f-x`, the same order `sends` is in.
            for (index, length) in [(1, lengths[0]), (2, lengths[1])] {
                let values = sends[index].1;
                live.handles.route_controls[index]
                    .set(
                        values.gain_db,
                        values.matrix,
                        values.mute,
                        [false; 2],
                        length,
                    )
                    .expect("queued");
            }
            live.render_into(blocks - push_block, &mut actual);
            let fresh = Live::new(&edit, &feeds, None).render(blocks);
            assert_planes(
                &actual,
                &fresh,
                settled,
                &format!("{rate} Hz trial {trial}: lengths {lengths:?}, sends {sends:?}"),
            );
            assert!(actual[0][settled..].iter().any(|sample| *sample != 0.0));
        }
    }
}

// ---- Gate 3 -------------------------------------------------------------------------------------

/// Tracks `t0` and `t1` send `r0` and `r1` from their `input` taps into bus `b`, which feeds the
/// output.
fn two_into_bus(r0: Values, r1: Values) -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    add_track(&mut model, &source, &track, "t0");
    add_track(&mut model, &source, &track, "t1");
    model.routes = vec![
        send("r0", "t0", SendTap::Input, "b", r0),
        send("r1", "t1", SendTap::Input, "b", r1),
        bus_to_output("b"),
    ];
    add_buses(&mut model, &["b"]);
    model
}

/// The records applied per route on this thread since the last bind, in route-ID order.
fn drained() -> Vec<u64> {
    graph::test_only_route_drained_counts()
}

/// Gate 3. Nothing partial:
///
/// * `depth + 1` `set` calls without a render: the last returns `Full`, and the queue held exactly
///   `depth` records (the next block drains exactly `depth`);
/// * a `Domain` refusal (a non-finite gain, also with a too-long ramp: `Domain` is decided first)
///   and a `Length` refusal push nothing: `free()` is unchanged, the next block drains nothing,
///   and the output is an unedited host's;
/// * `record` alone pushes nothing, and `push` of its result renders, bit for bit, as `set` with
///   the same arguments.
///
/// Test value: red if a refused call pushes, or if `Full` is detected only after a push.
#[test]
fn a_refused_send_record_pushes_nothing() {
    const SMALL: usize = 4;
    let mut draw = Draw::new(1_221_003);
    let r0 = values(&mut draw, false);
    let r1 = values(&mut draw, false);
    let model = two_into_bus(r0, r1);
    let index = route_index(&model, "r0");
    let blocks = 4;
    let feeds = feeds(&mut draw, &["t0", "t1"], blocks * QUANTUM);
    let edits: Vec<Values> = (0..=SMALL).map(|_| values(&mut draw, false)).collect();

    // A full queue.
    let mut live = Live::new(&model, &feeds, Some(SMALL));
    let producer = live.producer("r0");
    assert_eq!(producer.free(), SMALL);
    for (count, edit) in edits.iter().enumerate() {
        let result = producer.set(edit.gain_db, edit.matrix, edit.mute, [false; 2], 0);
        if count < SMALL {
            assert_eq!(result, Ok(()), "set {count}");
        } else {
            assert_eq!(result, Err(RouteControlError::Full), "set {count}");
        }
    }
    assert_eq!(producer.free(), 0);
    live.render(1);
    assert_eq!(
        drained()[index],
        SMALL as u64,
        "the queue held exactly depth records"
    );

    // Domain and Length refusals.
    let unedited = Live::new(&model, &feeds, None).render(blocks);
    let mut live = Live::new(&model, &feeds, Some(SMALL));
    let producer = live.producer("r0");
    let edit = edits[0];
    for (gain_db, length, error) in [
        (f32::NAN, 0, RouteControlError::Domain),
        (f32::INFINITY, 480, RouteControlError::Domain),
        (f32::NAN, TOO_LONG, RouteControlError::Domain),
        (edit.gain_db, TOO_LONG, RouteControlError::Length),
    ] {
        assert_eq!(
            producer.record(gain_db, edit.matrix, false, [false; 2], length),
            Err(error)
        );
        assert_eq!(
            producer.set(gain_db, edit.matrix, false, [false; 2], length),
            Err(error)
        );
        assert_eq!(producer.free(), SMALL, "a refusal pushed");
    }
    let actual = live.render(blocks);
    assert!(
        drained().iter().all(|count| *count == 0),
        "no record was applied"
    );
    assert_planes(&actual, &unedited, 0, "after refusals");

    // `record` then `push` is `set`.
    let mut split = Live::new(&model, &feeds, Some(SMALL));
    let producer = split.producer("r0");
    let record = producer
        .record(edit.gain_db, edit.matrix, edit.mute, [true, false], 37)
        .expect("in domain");
    assert_eq!(producer.free(), SMALL, "record pushed");
    producer.push(record).expect("room");
    let mut whole = Live::new(&model, &feeds, Some(SMALL));
    whole
        .producer("r0")
        .set(edit.gain_db, edit.matrix, edit.mute, [true, false], 37)
        .expect("room");
    let split = split.render(blocks);
    let whole = whole.render(blocks);
    assert_planes(&split, &whole, 0, "record then push against set");
    assert!(
        first_difference(&split[0], &unedited[0]).is_some(),
        "the edit is audible"
    );
}

// ---- Gate 4 -------------------------------------------------------------------------------------

/// Gate 4. `console-sixty-four-track-intended.json`, whose routes all go to the output, prepared
/// through host-core with no control channel and with a depth of 64, reports the same nonzero
/// `bank_route_folds()`, renders 8 blocks of the same source content bit-identically, and has no
/// live send.
///
/// Test value: red if attaching live controls binds or declines routes into the output and so
/// loses the master fold (VERIFY-1 MAJOR-10); the console benchmark's fold test never prepares
/// through host-core.
#[test]
fn live_controls_cost_the_standing_sessions_no_fold() {
    let blocks = 8;
    let mut draw = Draw::new(1_221_004);
    let feeds = feeds(&mut draw, &["fixture-source"], blocks * QUANTUM);
    let mut without = Live::from_document(CONSOLE_FIXTURE, &feeds, None);
    let mut with = Live::from_document(CONSOLE_FIXTURE, &feeds, Some(64));
    let folds = without.host.plan.bank_route_folds();
    assert!(folds > 0, "the fixture folds its master");
    assert_eq!(
        with.host.plan.bank_route_folds(),
        folds,
        "Some(64) against None"
    );
    assert!(with.handles.route_controls.is_empty());
    assert_eq!(
        with.host.report.route_control_resources,
        RouteControlResources::default()
    );
    let expected = without.render(blocks);
    let actual = with.render(blocks);
    assert_planes(&actual, &expected, 0, "Some(64) against None");
    assert!(actual[0].iter().any(|sample| *sample != 0.0));
}

// ---- Gate 5 -------------------------------------------------------------------------------------

/// Three tracks and two buses; the sends into the buses are declared `zz-send`, `mm-send`,
/// `aa-send`, the routes into the output between and after them.
fn out_of_order() -> SessionModel {
    let (mut model, source, track) = empty_session(48_000);
    for id in ["t0", "t1", "t2"] {
        add_track(&mut model, &source, &track, id);
    }
    model.routes = vec![
        send("zz-send", "t0", SendTap::PostPan, "bus-a", Values::UNITY),
        to_output("t0-main", tap("t0", SendTap::PostPan)),
        send("mm-send", "t1", SendTap::PostPan, "bus-b", Values::UNITY),
        send("aa-send", "t2", SendTap::PostPan, "bus-a", Values::UNITY),
        bus_to_output("bus-b"),
        bus_to_output("bus-a"),
    ];
    add_buses(&mut model, &["bus-b", "bus-a"]);
    model
}

/// Gate 5. `route_controls` lists exactly the routes into submixes, in canonical route-ID order,
/// each with its `route_id`, for a session that declares them out of that order; each producer
/// drives the lane at its own route's index; and the list is empty without a depth.
///
/// Test value: red if host-core orders producers by declaration rather than by the order the
/// render plane's lanes were attached in.
#[test]
fn live_send_handles_are_in_canonical_route_order() {
    let model = out_of_order();
    let mut live = Live::new(&model, &Vec::new(), Some(DEPTH));
    let ids: Vec<&str> = live
        .handles
        .route_controls
        .iter()
        .map(RouteControlProducer::route_id)
        .collect();
    assert_eq!(ids, ["aa-send", "mm-send", "zz-send"]);
    for position in 0..3 {
        let id = live.handles.route_controls[position].route_id().to_owned();
        live.handles.route_controls[position]
            .set(-6.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2], 0)
            .expect("queued");
        // No feeds: every source underruns to zero, which the drain does not care about.
        live.submit();
        live.render_block();
        let counts = drained();
        let index = route_index(&model, &id);
        for (route, count) in counts.iter().enumerate() {
            assert_eq!(*count, u64::from(route == index), "{id}: route {route}");
        }
        graph::test_only_route_mix_reset();
    }
    let idle = Live::new(&model, &Vec::new(), None);
    assert!(idle.handles.route_controls.is_empty());
}

/// Issue #1223 gate 4 (D2): `longest_route_id_bytes` is the longest route ID over every route of
/// the session, which here is a route into the output, longer than every send, source, track and
/// submix ID.
///
/// Test value: red if the shape measures only the live sends, or another ID family, so a host
/// sizing its ID staging from it would undersize it for a route it may name.
#[test]
fn the_session_shape_measures_every_route_id() {
    let mut model = out_of_order();
    let long = "t1-main-to-the-room-and-past-every-send";
    model
        .routes
        .push(to_output(long, tap("t1", SendTap::PostPan)));
    let compiled = compile_host_session(&document(&model), &caps()).expect("session compiles");
    let shape = compiled_session_shape(&compiled).expect("shape");
    assert_eq!(shape.longest_route_id_bytes, long.len() as u64);
    assert_eq!(shape.longest_submix_id_bytes, "bus-a".len() as u64);
    assert_eq!(shape.longest_track_id_bytes, "t0".len() as u64);
}

// ---- Gate 6 -------------------------------------------------------------------------------------

/// The same session attached directly through the graph compiler: the oracle for host-core's
/// charge.
fn graph_route_resources(document: &str, depth: usize) -> RouteControlResources {
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
    let mut artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1_221,
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
    let producers = artifact
        .attach_route_live_controls(NonZeroUsize::new(depth).expect("depth"))
        .expect("attach");
    graph::route_control_resources(&producers)
}

/// What host-core admits against `maximum_graph_session_plus_plan_bytes` for one preparation.
fn admitted(host: &PreparedHost) -> u64 {
    let report = &host.report;
    report.graph_session_plus_plan_bytes
        + report.session_model_bytes
        + report
            .effect_control_resources
            .total_bytes()
            .expect("effect control bytes")
        + report.route_control_resources.total_bytes
}

/// Gate 6. With live controls, `report.route_control_resources` is the graph's own charge for the
/// same attach, and its total is admitted: at the admitted total the session prepares, and one
/// byte below it is refused with `host.graph.resource.limit`. Without a depth the field is zero.
///
/// Test value: red if the route queues or owner boxes are uncharged, or charged on a
/// live-control-free plan.
#[test]
fn live_send_lanes_are_charged_against_the_graph_cap() {
    let mut draw = Draw::new(1_221_006);
    let document = document(&two_into_bus(
        values(&mut draw, false),
        values(&mut draw, false),
    ));
    let live = Live::from_document(&document, &Vec::new(), Some(DEPTH));
    let charged = live.host.report.route_control_resources;
    assert_eq!(charged, graph_route_resources(&document, DEPTH));
    assert_eq!(charged.routes, 2);
    assert!(charged.queue_bytes > 0 && charged.owner_bytes > 0 && charged.activity_bytes > 0);
    let total = admitted(&live.host);
    drop(live);

    let mut limits = caps();
    limits.maximum_graph_session_plus_plan_bytes = total;
    prepare_host_session_with_live_controls(&document, &limits, &request(Some(DEPTH)))
        .unwrap_or_else(|failure| {
            panic!(
                "at the admitted total: {}",
                String::from_utf8_lossy(failure.as_bytes())
            )
        });
    limits.maximum_graph_session_plus_plan_bytes = total - 1;
    let Err(failure) =
        prepare_host_session_with_live_controls(&document, &limits, &request(Some(DEPTH)))
    else {
        panic!("one byte below the admitted total prepares");
    };
    assert_eq!(failure.kind(), PrepareRejection::Resource);
    assert_eq!(failure.as_bytes(), b"host.graph.resource.limit\t$\n");

    let idle = Live::from_document(&document, &Vec::new(), None);
    assert_eq!(
        idle.host.report.route_control_resources,
        RouteControlResources::default()
    );
    assert_eq!(
        admitted(&idle.host) + charged.total_bytes,
        total,
        "only the route lanes separate the two charges"
    );
}

// ---- Gate 7 -------------------------------------------------------------------------------------

/// Gate 7. A producer thread calls `set` on both live sends of gate 2's session while this thread
/// renders, and every block applies at least one record; after the first block, every render
/// allocates and frees nothing on the render thread (`bench_support::alloc`'s thread-scoped
/// counters).
///
/// Test value: red if host-core's attachment leaves any lane state to be built on the render
/// thread.
#[test]
fn live_sends_render_without_allocating() {
    let mut draw = Draw::new(1_221_007);
    let sends = [
        ("a-b", values(&mut draw, false)),
        ("c-b", values(&mut draw, false)),
        ("e-x", values(&mut draw, false)),
        ("f-x", values(&mut draw, false)),
    ];
    let blocks = 48;
    let feeds = feeds(&mut draw, &["a", "c", "e", "f"], blocks * QUANTUM);
    let mut live = Live::new(&two_buses(48_000, &sends), &feeds, Some(DEPTH));
    let mut producers = core::mem::take(&mut live.handles.route_controls);
    assert_eq!(producers.len(), 4);
    let edits: Vec<(Values, [bool; 2], u32)> = (0..32)
        .map(|index| {
            (
                values(&mut draw, index % 5 == 0),
                [index % 3 == 0, index % 7 == 0],
                [0, 1, 37, 480][index % 4],
            )
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
                    let (values, lanes, length) = edits[index % edits.len()];
                    if producer
                        .set(values.gain_db, values.matrix, values.mute, lanes, length)
                        .is_ok()
                    {
                        pushed.fetch_add(1, Ordering::Release);
                    }
                    index += 1;
                }
                std::thread::yield_now();
            }
        });
        let mut seen = 0;
        for block in 0..blocks {
            while pushed.load(Ordering::Acquire) == seen {
                std::thread::yield_now();
            }
            seen = pushed.load(Ordering::Acquire);
            live.submit();
            let before: u64 = drained().iter().sum();
            let mark = current_thread_counters();
            live.render_block();
            let delta = current_thread_delta_since(mark);
            let after: u64 = drained().iter().sum();
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
