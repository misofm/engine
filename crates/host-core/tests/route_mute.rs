//! Issue #1216: a route's `mute` is a session switch, and it is not structural. Issue #1217: an
//! undelayed muted route is inactive -- neither mixed nor read by its destination's sum. Issue
//! #1218: a send into a submix may follow its source strip's lane mutes.
//!
//! * **#1217 gate 1, an inactive route is neither mixed nor read.** A bus, and the session output,
//!   sum two contributors fed distinct noise, one route muted, and render the scalar D3 oracle
//!   bit for bit (the first input in route-ID order owns the store; a later one is added only when
//!   active), while the muted route's op never mixes. A muted route that is its bus's sole, in-place
//!   contributor leaves the bus exact `+0.0`, and so does a bus, or the session output, whose every
//!   contributor is muted. The host's output planes are pre-filled with [`HOST_SENTINEL`], so the
//!   host-master form must store its `+0.0` itself.
//! * **#1217 gate 2, the first input owns the store**, with `+0.0` when it is inactive.
//! * **#1216 gate 3, mute is not structural.** Muting the route PDC delays leaves every inserted
//!   delay, route timing and the output latency unchanged.
//! * **#1217 gate 3, a muted delayed route stays active**, mixing `[+0.0; 4]` every block, and its
//!   compensation line carries that zero mix's sign to the bus 486 frames later.
//! * **#1216 gate 4, no folded muted lane.** A bus with a muted contributor declines the route fold
//!   and renders the bits of the same plan bound with the fold declined.
//! * **#1217 gate 5**: those sessions render without one allocator call after warm-up.
//!
//! * **#1218 gates 1 and 4, a follow send zeroes its muted source lane's column**, from a track
//!   and from a submix, bit-identically to the same send with that column at `+0.0`.
//! * **#1218 gate 2, a fully follow-muted send** is inactive when undelayed and stays active, mixing
//!   `[+0.0; 4]`, when delayed.
//!
//! #1216's gate 1 (a muted route mixes zero coefficients) was superseded by #1217's: an undelayed
//! muted route is no longer mixed at all.

use bench_support::alloc as bench_alloc;
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_compiler::{EffectCompileCaps, launch_native_effect_registry};
use effect_contract::LatencySamples;
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
    ChannelMatrix, Console, Effect, EffectIdentity, EffectQuality, LinkMode, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration, Source, StableId,
    Submix, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const FRAMES: usize = QUANTUM * BLOCKS;
/// What every host output buffer holds before a render. The engine never pre-clears the host's
/// planes (a block writes each plane once), so a sum that leaves a sample unwritten shows it.
const HOST_SENTINEL: f32 = 7.0;

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
        let mut samples = [HOST_SENTINEL; QUANTUM * 2];
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

// ---- #1217: the scalar D3 oracle ----------------------------------------------------------

/// A stereo block's two planes.
type Planes = [Vec<f32>; 2];
/// Each track's id and its source planes.
type Feeds = Vec<(String, Planes)>;

/// Renders [`BLOCKS`] quanta of `document`; returns its output planes and the route-op mixes each
/// prepared route ran, in canonical route-ID order (`graph::test_only_route_mix_counts`, which the
/// bind resets and which counts only in a plan that built a route-activity table).
fn render_counted(document: &str, feeds: &[(String, [Vec<f32>; 2])]) -> ([Vec<f32>; 2], Vec<u64>) {
    let (out, _) = render(document, feeds);
    (out, graph::test_only_route_mix_counts())
}

/// The position of route `id` among `model`'s routes in route-ID order: its index in the
/// executor's route-activity table and in [`render_counted`]'s mix counts.
fn route_index(model: &SessionModel, id: &str) -> usize {
    let mut ids: Vec<&str> = model.routes.iter().map(|route| route.id.as_str()).collect();
    ids.sort_unstable();
    ids.iter()
        .position(|candidate| *candidate == id)
        .expect("a declared route")
}

/// A route's coefficients `(ll, lr, rl, rr)`: `gain * matrix` when open, `[+0.0; 4]` when muted.
fn coefficients(route: &Route) -> [f32; 4] {
    if route.mute {
        return [0.0; 4];
    }
    let gain = math::db_to_gain_f32(route.gain_db);
    let m = &route.channel_matrix;
    [gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr]
}

/// One route's mix of `planes`, per frame `left = (lr * r) + (ll * l)` and
/// `right = (rr * r) + (rl * l)` (#1215 D3).
fn mix(route: &Route, planes: &[Vec<f32>; 2]) -> [Vec<f32>; 2] {
    let [ll, lr, rl, rr] = coefficients(route);
    let frames = planes[0].len();
    let mut out = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for frame in 0..frames {
        let (l, r) = (planes[0][frame], planes[1][frame]);
        out[0][frame] = (lr * r) + (ll * l);
        out[1][frame] = (rr * r) + (rl * l);
    }
    out
}

/// The P4/D3 sum of a destination's contributions in route-ID order, each `(active, planes)`: the
/// first owns the store (its planes when active, `+0.0` when inactive) and each later one is added
/// only when active. Never "the first active stores".
fn d3_sum(contributions: &[(bool, [Vec<f32>; 2])]) -> [Vec<f32>; 2] {
    let (first_active, first) = &contributions[0];
    let mut sum = if *first_active {
        first.clone()
    } else {
        [vec![0.0_f32; FRAMES], vec![0.0_f32; FRAMES]]
    };
    for (active, planes) in &contributions[1..] {
        if *active {
            for plane in 0..2 {
                for (total, sample) in sum[plane].iter_mut().zip(&planes[plane]) {
                    *total += *sample;
                }
            }
        }
    }
    sum
}

fn input_tap(track: &str) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::Input,
    }
}

fn bus_input(bus: &str) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(bus),
        tap: SendTap::Input,
    }
}

fn assert_all_bits(planes: &[Vec<f32>; 2], expected: f32, what: &str) {
    for (plane, samples) in planes.iter().enumerate() {
        if let Some(frame) = samples
            .iter()
            .position(|sample| sample.to_bits() != expected.to_bits())
        {
            panic!(
                "{what}: plane {plane} frame {frame}: {:?}, expected {expected:?}",
                samples[frame]
            );
        }
    }
}

// ---- #1217 gate 1 ---------------------------------------------------------------------------

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

/// Where gate 1's two contributors are summed.
#[derive(Clone, Copy, Debug)]
enum Sum {
    /// Bus `b`, whose `input` tap feeds the output by the unity route `b-main`.
    Bus,
    /// The session output itself: the host-master reduction.
    Output,
}

/// Tracks `t0` and `t1` route from their `input` taps by `r0` and `r1`, with drawn gains and
/// nonzero matrices, into `sum`; route `r<muted>` is muted. Returns the session, the two routes and
/// the feeds. The open contributor's every fourth frame is the signed zero that makes its
/// contribution `-0.0` on both lanes, so a muted route that is still mixed shows its sign there.
fn two_contributors(draw: &mut Draw, sum: Sum, muted: usize) -> (SessionModel, Vec<Route>, Feeds) {
    let (mut model, source, track) = empty_session();
    let mut feeds = Vec::new();
    let mut routes = Vec::new();
    for index in 0..2 {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        let destination = match sum {
            Sum::Bus => into_bus("b"),
            Sum::Output => RouteDestination::OutputInput {
                output_id: sid("main-out"),
            },
        };
        let mut drawn = route(&format!("r{index}"), input_tap(&id), destination);
        drawn.gain_db = draw.in_domain(-12.0, 6.0);
        // Column signs: `ll`/`rl` scale the left source lane, `lr`/`rr` the right.
        let (left_negative, right_negative) = (draw.chance(1, 2), draw.chance(1, 2));
        drawn.channel_matrix = ChannelMatrix {
            ll: signed(draw, left_negative),
            lr: signed(draw, right_negative),
            rl: signed(draw, left_negative),
            rr: signed(draw, right_negative),
        };
        drawn.mute = index == muted;
        // A zero of the column's opposite sign makes each open product `-0.0`.
        let zero = |negative: bool| if negative { 0.0 } else { -0.0 };
        let zeros = (!drawn.mute).then(|| [zero(left_negative), zero(right_negative)]);
        feeds.push((id, planes(draw, zeros)));
        routes.push(drawn);
    }
    model.routes = routes.clone();
    if let Sum::Bus = sum {
        model.routes.push(to_output("b-main", bus_input("b")));
        model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    }
    (model, routes, feeds)
}

const GATE_1: &str = "an_inactive_route_is_neither_mixed_nor_read";
const GATE_1_REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test \
                             route_mute -- --exact an_inactive_route_is_neither_mixed_nor_read";

/// #1217 gate 1, two undelayed contributors. One of `r0` and `r1` (drawn) is muted, so it is
/// inactive. Summed by bus `b` and, separately, by the session output (the host-master form), the
/// rendered output equals the scalar D3 oracle bit for bit -- through `b-main`'s unity mix for the
/// bus -- and the muted route's op never mixes while the open one mixes every block.
///
/// When `r0` is muted the zero frames render `+0.0 + (-0.0) = +0.0`; when `r1` is, `-0.0` alone.
/// Mixing the muted route instead adds its `(+0.0 * r) + (+0.0 * l)`, whose sign follows the muted
/// track's samples, so a muted route that is still mixed moves those zeros.
///
/// Red if a muted undelayed route is still mixed or added (the counter, and the signed zeros), or
/// if either reduction form moves the store owner.
#[test]
fn an_inactive_route_is_neither_mixed_nor_read() {
    let mut negative_zeros = 0;
    let ran = run_seeds(GATE_1, GATE_1_REPLAY, 16, |seed| {
        for sum in [Sum::Bus, Sum::Output] {
            let mut draw = Draw::new(seed);
            let muted = draw.below(2);
            let (model, routes, feeds) = two_contributors(&mut draw, sum, muted);
            let (actual, mixes) = render_counted(&document(&model), &feeds);
            let contributions: Vec<(bool, [Vec<f32>; 2])> = routes
                .iter()
                .zip(&feeds)
                .map(|(route, (_, planes))| (!route.mute, mix(route, planes)))
                .collect();
            let summed = d3_sum(&contributions);
            let expected = match sum {
                Sum::Bus => mix(&to_output("b-main", bus_input("b")), &summed),
                Sum::Output => summed,
            };
            let what = format!("seed {seed}, {sum:?}, r{muted} muted");
            for plane in 0..2 {
                assert_bits_equal(
                    &actual[plane],
                    &expected[plane],
                    &format!("{what}, plane {plane}"),
                );
            }
            assert!(
                actual.iter().flatten().any(|sample| *sample != 0.0),
                "{what}: the open route rendered audio"
            );
            let open = 1 - muted;
            assert_eq!(
                mixes[route_index(&model, &format!("r{muted}"))],
                0,
                "{what}: the muted route's op mixed"
            );
            assert_eq!(
                mixes[route_index(&model, &format!("r{open}"))],
                BLOCKS as u64,
                "{what}: the open route's op mixes every block"
            );
            if let Sum::Output = sum {
                negative_zeros += actual
                    .iter()
                    .flatten()
                    .filter(|sample| sample.to_bits() == (-0.0_f32).to_bits())
                    .count();
            }
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 16);
        // The open contribution's `-0.0` reached the output, or the zero frames prove nothing.
        assert!(negative_zeros > 0, "no zero frame rendered as -0.0");
    }
}

/// Track `t` feeds bus `c` alone, by `t-c` from its `post_pan` tap, and `c`'s `input` tap feeds
/// the output by the unity route `c-main`; `t-c` is muted when `muted`. `t-c` is its tap's sole
/// reader, so it mixes in place over the tap, and `c`'s input reads `t-c` in place in turn.
fn in_place_chain(muted: bool) -> SessionModel {
    let (mut model, source, track) = empty_session();
    add_track(&mut model, &source, &track, "t");
    let mut into = route("t-c", post_pan("t"), into_bus("c"));
    into.mute = muted;
    model.routes.push(into);
    model.routes.push(to_output("c-main", bus_input("c")));
    model.submixes = vec![Submix::unity(sid("c"), &model.console)];
    model
}

/// #1217 gate 1, the in-place sole contributor. With `t-c` muted, `c`'s input is exact `+0.0` on
/// both planes, so the output, `c-main`'s unity mix of it, is too; `t-c` never mixes, while `c-main`
/// mixes every block. The same chain open renders audio, so the muted render is not silent by
/// construction.
///
/// Red if an inactive in-place route leaves the raw tap in its destination: the skipped route's
/// buffer still holds `t`'s `post_pan` audio, and a destination that aliased it would pass it on.
#[test]
fn an_inactive_in_place_sole_route_fills_its_bus_with_positive_zero() {
    let mut draw = Draw::new(1_217);
    let feeds = vec![("t".to_owned(), planes(&mut draw, None))];
    let (open, _) = render_counted(&document(&in_place_chain(false)), &feeds);
    assert!(
        open.iter().flatten().any(|sample| *sample != 0.0),
        "the open chain renders audio, or this is vacuous"
    );
    let model = in_place_chain(true);
    let (actual, mixes) = render_counted(&document(&model), &feeds);
    assert_all_bits(&actual, 0.0, "the muted sole route's bus");
    assert_eq!(mixes[route_index(&model, "t-c")], 0, "t-c mixed");
    assert_eq!(
        mixes[route_index(&model, "c-main")],
        BLOCKS as u64,
        "c-main mixes every block"
    );
}

/// `count` tracks route from their `input` taps by muted unity routes `r0..` into `sum`: bus `b`,
/// whose `input` tap feeds the output by the unity route `b-main`, or the session output itself.
fn every_contributor_muted(sum: Sum, count: usize) -> SessionModel {
    let (mut model, source, track) = empty_session();
    for index in 0..count {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        let destination = match sum {
            Sum::Bus => into_bus("b"),
            Sum::Output => RouteDestination::OutputInput {
                output_id: sid("main-out"),
            },
        };
        let mut into = route(&format!("r{index}"), input_tap(&id), destination);
        into.mute = true;
        model.routes.push(into);
    }
    if let Sum::Bus = sum {
        model.routes.push(to_output("b-main", bus_input("b")));
        model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    }
    model
}

/// #1217 gate 1, every contributor muted. Two and nine (more than one reduction group) muted
/// routes into bus `b`, and into the session output (the host-master form, whose planes hold
/// [`HOST_SENTINEL`] before each block): the result is exact `+0.0` on both planes, and no muted
/// route mixes. Every track is fed strictly negative samples, so a muted route that was mixed
/// anyway would show `-0.0`.
///
/// Red if a destination whose every route input is inactive skips its sum instead of storing
/// `+0.0`: the session output then keeps the host's sentinel.
#[test]
fn a_destination_whose_every_route_is_muted_renders_positive_zero() {
    let mut draw = Draw::new(12_176);
    for sum in [Sum::Bus, Sum::Output] {
        for count in [2, 9] {
            let model = every_contributor_muted(sum, count);
            let feeds: Vec<(String, [Vec<f32>; 2])> = (0..count)
                .map(|index| (format!("t{index}"), negative_planes(&mut draw)))
                .collect();
            let what = format!("{sum:?}, {count} muted routes");
            let (actual, mixes) = render_counted(&document(&model), &feeds);
            assert_all_bits(&actual, 0.0, &what);
            for index in 0..count {
                assert_eq!(
                    mixes[route_index(&model, &format!("r{index}"))],
                    0,
                    "{what}: r{index} mixed"
                );
            }
        }
    }
}

/// Strictly negative samples in `[-1, -0.1]` on both planes, so a zero-coefficient mix of them is
/// `-0.0`.
fn negative_planes(draw: &mut Draw) -> Planes {
    let mut plane = || {
        (0..FRAMES)
            .map(|_| signed(draw, true))
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

/// `muted.len()` tracks route from their `input` taps by `r00..`, with drawn gains and nonzero
/// matrices, into `sum`; `r<k>` is muted when `muted[k]`. Returns the session, the routes and the
/// feeds (#1217 verdict 2, MINOR-1).
fn many_contributors(
    draw: &mut Draw,
    sum: Sum,
    muted: &[bool],
) -> (SessionModel, Vec<Route>, Feeds) {
    let (mut model, source, track) = empty_session();
    let mut feeds = Vec::new();
    let mut routes = Vec::new();
    for (index, is_muted) in muted.iter().enumerate() {
        let id = format!("t{index:02}");
        add_track(&mut model, &source, &track, &id);
        let destination = match sum {
            Sum::Bus => into_bus("b"),
            Sum::Output => RouteDestination::OutputInput {
                output_id: sid("main-out"),
            },
        };
        let mut drawn = route(&format!("r{index:02}"), input_tap(&id), destination);
        drawn.gain_db = draw.in_domain(-12.0, 6.0);
        let signs = [
            draw.chance(1, 2),
            draw.chance(1, 2),
            draw.chance(1, 2),
            draw.chance(1, 2),
        ];
        drawn.channel_matrix = ChannelMatrix {
            ll: signed(draw, signs[0]),
            lr: signed(draw, signs[1]),
            rl: signed(draw, signs[2]),
            rr: signed(draw, signs[3]),
        };
        drawn.mute = *is_muted;
        feeds.push((id, planes(draw, None)));
        routes.push(drawn);
    }
    model.routes = routes.clone();
    if let Sum::Bus = sum {
        model.routes.push(to_output("b-main", bus_input("b")));
        model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    }
    (model, routes, feeds)
}

/// #1217 gate 1, three or more contributors (verdict 2, MINOR-1). An inactive route between
/// active ones -- `open, muted, open` and wider patterns, including later runs that cross the
/// 8-input reduction group -- leaves every active contribution in the sum: the bus, and the
/// session output, render the scalar D3 oracle bit for bit, and only the muted routes skip their
/// mix.
///
/// Red if a later active run after an inactive route stores over the earlier active run instead of
/// adding to it (verdict mutation MC: `route_segments` forgets that the opening run stored), which
/// silently drops every send before the muted one. No two-contributor case reaches that shape.
#[test]
fn a_middle_inactive_route_keeps_every_active_contribution() {
    let patterns: [(&[usize], usize); 6] = [
        (&[1], 3),
        (&[1, 3], 5),
        (&[3, 9], 12),
        (&[0, 5, 10], 12),
        (&[8], 11),
        (&[2, 3, 4, 5, 6, 7, 8, 9, 10], 12),
    ];
    let mut draw = Draw::new(99_217);
    for (pattern, count) in patterns {
        let muted: Vec<bool> = (0..count).map(|index| pattern.contains(&index)).collect();
        for sum in [Sum::Bus, Sum::Output] {
            let (model, routes, feeds) = many_contributors(&mut draw, sum, &muted);
            let (actual, mixes) = render_counted(&document(&model), &feeds);
            let contributions: Vec<(bool, [Vec<f32>; 2])> = routes
                .iter()
                .zip(&feeds)
                .map(|(route, (_, planes))| (!route.mute, mix(route, planes)))
                .collect();
            let summed = d3_sum(&contributions);
            let expected = match sum {
                Sum::Bus => mix(&to_output("b-main", bus_input("b")), &summed),
                Sum::Output => summed,
            };
            let what = format!("{sum:?}, {count} inputs, muted {pattern:?}");
            for plane in 0..2 {
                assert_bits_equal(
                    &actual[plane],
                    &expected[plane],
                    &format!("{what}, plane {plane}"),
                );
            }
            for (index, is_muted) in muted.iter().enumerate() {
                assert_eq!(
                    mixes[route_index(&model, &format!("r{index:02}"))],
                    if *is_muted { 0 } else { BLOCKS as u64 },
                    "{what}: r{index:02} mixes"
                );
            }
        }
    }
}

// ---- #1217 gate 2 ---------------------------------------------------------------------------

/// Tracks `t0..` route from their `input` taps by unity routes `r0..` into bus `b`, whose `input`
/// tap feeds the output by the unity route `b-main`. Track `t<k>` is fed `feeds[k]`, and `r<k>` is
/// muted when `muted[k]`.
fn unity_bus(muted: &[bool]) -> SessionModel {
    let (mut model, source, track) = empty_session();
    for (index, muted) in muted.iter().enumerate() {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        let mut into = route(&format!("r{index}"), input_tap(&id), into_bus("b"));
        into.mute = *muted;
        model.routes.push(into);
    }
    model.routes.push(to_output("b-main", bus_input("b")));
    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    model
}

/// #1217 gate 2. A unity route mixes an all-`-0.0` block to `-0.0`, and `b-main`'s unity mix
/// passes the bus input's zero sign to the output.
///
/// * `r0` muted, `r1` open over an all-`-0.0` block: the inactive first input still owns the store,
///   so the bus input is `+0.0 + (-0.0) = +0.0`, and the output is `+0.0` everywhere.
/// * `r0` open over an all-`-0.0` block, `r1` and `r2` muted: the bus input is `r0`'s `-0.0` alone,
///   and so is the output.
///
/// Red if the store owner moves to the first *active* input (the first case renders `-0.0`), or if
/// an active first input's `-0.0` is lost, for instance to a `+0.0` fill it is then added to (the
/// second renders `+0.0`).
#[test]
fn the_first_route_in_id_order_owns_the_store() {
    let mut draw = Draw::new(12_172);
    let negative = [vec![-0.0_f32; FRAMES], vec![-0.0_f32; FRAMES]];
    let mut noise = || planes(&mut draw, None);
    let cases: [(&[bool], Vec<Planes>, f32, &str); 2] = [
        (
            &[true, false],
            vec![noise(), negative.clone()],
            0.0,
            "r0 muted, r1 open at -0.0",
        ),
        (
            &[false, true, true],
            vec![negative.clone(), noise(), noise()],
            -0.0,
            "r0 open at -0.0, r1 and r2 muted",
        ),
    ];
    for (muted, inputs, expected, what) in cases {
        let model = unity_bus(muted);
        let feeds: Vec<(String, [Vec<f32>; 2])> = inputs
            .into_iter()
            .enumerate()
            .map(|(index, planes)| (format!("t{index}"), planes))
            .collect();
        let (actual, mixes) = render_counted(&document(&model), &feeds);
        assert_all_bits(&actual, expected, what);
        for (index, muted) in muted.iter().enumerate() {
            assert_eq!(
                mixes[route_index(&model, &format!("r{index}"))],
                if *muted { 0 } else { BLOCKS as u64 },
                "{what}: r{index}'s mixes"
            );
        }
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
        registry,
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

// ---- #1217 gate 3 ---------------------------------------------------------------------------

/// The true-peak limiter's prepared latency at 48 kHz, and the compensation `d-e` carries.
const LIMITER_LATENCY: usize = 486;

/// Tracks `d` and `s` feed bus `e`: `d` by `d-e` from its `input` tap, when `with_d`, muted; `s`
/// by `s-e` from its `post_pan` tap, through a true-peak limiter insert. PDC delays `d-e`'s edge
/// into `e` by the limiter's latency. `e`'s `input` tap feeds the output by the unity route
/// `e-main`.
fn delayed_bus(with_d: bool) -> SessionModel {
    let (mut model, source, track) = empty_session();
    for id in ["d", "s"] {
        add_track(&mut model, &source, &track, id);
    }
    model.tracks[1].inserts.effects.push(Effect {
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
    if with_d {
        let mut muted = route("d-e", input_tap("d"), into_bus("e"));
        muted.mute = true;
        model.routes.push(muted);
    }
    model
        .routes
        .push(route("s-e", post_pan("s"), into_bus("e")));
    model.routes.push(to_output("e-main", bus_input("e")));
    model.submixes = vec![Submix::unity(sid("e"), &model.console)];
    model
}

/// #1217 gate 3. `d-e` is muted, but its edge into `e` carries the 486-sample compensation, so it
/// stays active: its op mixes `[+0.0; 4]` every block and its line stages that mix.
///
/// The oracle is D3 over `e`'s inputs in route-ID order: `d-e` first, so it owns the store, with
/// its delayed zero-coefficient mix, then `s-e`. `s-e`'s contribution `s` comes from the render of
/// the same session without `d-e`, through `e-main`'s unity mix, where `out = (+0.0 * s_r) + s_l`
/// is `s_l` exactly wherever `s_l` is nonzero. The delayed zero mix is `+0.0` (the line's initial
/// state) for the first 486 frames and a signed zero after them, so wherever `s` is nonzero the
/// oracle is `s`, and wherever `s` is zero it must lie in those first 486 frames (asserted), where
/// the oracle is `+0.0 + s = +0.0` on both lanes, and so is its `e-main` mix.
///
/// Red if a delayed muted route goes inactive: its op stops mixing, and its consumer would stage,
/// every block, an arena buffer this block never wrote.
#[test]
fn a_muted_delayed_route_stays_active() {
    assert_delayed_route_stays_active(&delayed_session_checked(delayed_bus(true)));
}

/// The body of [`a_muted_delayed_route_stays_active`] for any `model` built from
/// [`delayed_bus`]`(true)` whose `d-e` is silenced (muted, or following a fully muted `d`).
fn assert_delayed_route_stays_active(model: &SessionModel) {
    let mut draw = Draw::new(12_173);
    let feeds: Vec<(String, [Vec<f32>; 2])> = ["d", "s"]
        .iter()
        .map(|id| ((*id).to_owned(), planes(&mut draw, None)))
        .collect();
    let (reference, _) = render_counted(&document(&delayed_bus(false)), &feeds);
    let (actual, mixes) = render_counted(&document(model), &feeds);
    assert!(
        graph::test_only_route_activity_built(),
        "a plan with a silenced route builds its route-activity table"
    );
    let mut expected = reference.clone();
    for (plane, samples) in expected.iter_mut().enumerate() {
        for (frame, sample) in samples.iter_mut().enumerate() {
            if *sample == 0.0 {
                assert!(
                    frame < LIMITER_LATENCY,
                    "plane {plane} frame {frame}: `s` is zero past the delayed line's +0.0 \
                     prefix, so this oracle cannot fix the sum's zero sign"
                );
                *sample = 0.0;
            }
        }
    }
    for plane in 0..2 {
        assert_bits_equal(
            &actual[plane],
            &expected[plane],
            &format!("plane {plane}: e's input against the D3 oracle"),
        );
    }
    assert!(
        actual.iter().flatten().any(|sample| *sample != 0.0),
        "the bus renders audio"
    );
    assert_eq!(
        mixes[route_index(model, "d-e")],
        BLOCKS as u64,
        "the silenced delayed route mixes its zero coefficients every block"
    );
}

/// `model` ([`delayed_bus`] with `d-e`), after asserting from the compiled plan that `d-e` is the
/// one route PDC delays, by [`LIMITER_LATENCY`] samples.
fn delayed_session_checked(model: SessionModel) -> SessionModel {
    let artifact = graph_artifact(&document(&model));
    let route = graph::StableGraphId::parse("d-e").expect("graph id");
    assert!(
        matches!(
            artifact.graph().inserted_delays.as_slice(),
            [delay] if delay.samples == LatencySamples(LIMITER_LATENCY as u64)
                && matches!(&delay.edge_id, GraphEdgeId::RouteDestination { route_id }
                    if *route_id == route)
        ),
        "d-e's edge into e carries the limiter's compensation: {:?}",
        artifact.graph().inserted_delays
    );
    model
}

/// #1217 gate 3, the line's contents. In [`delayed_bus`] with both routes muted, `s-e` (undelayed)
/// is inactive, while `d-e` (delayed) stays active and, first in route-ID order, owns `e`'s store.
/// `d` is fed strictly negative samples, so `d-e`'s zero-coefficient mix is `-0.0` on both lanes,
/// and `e`'s input is that mix through the 486-sample line alone: the line's initial `+0.0` for
/// frames `< 486`, then `-0.0`. The output, `e-main`'s unity mix of it, carries the same bits.
///
/// Red if a delayed muted route goes inactive, on the audio itself (its line no longer carries its
/// mix, so frame 486 on renders `+0.0`), if its line holds anything but this block's zero mix, or if
/// the delay moves.
#[test]
fn a_muted_delayed_routes_line_carries_its_zero_mix() {
    let mut model = delayed_session_checked(delayed_bus(true));
    for route in &mut model.routes {
        if route.id.as_str() == "s-e" {
            route.mute = true;
        }
    }
    let mut draw = Draw::new(12_177);
    let feeds = vec![
        ("d".to_owned(), negative_planes(&mut draw)),
        ("s".to_owned(), planes(&mut draw, None)),
    ];
    let (actual, mixes) = render_counted(&document(&model), &feeds);
    let expected: Vec<f32> = (0..FRAMES)
        .map(|frame| if frame < LIMITER_LATENCY { 0.0 } else { -0.0 })
        .collect();
    for (plane, samples) in actual.iter().enumerate() {
        assert_bits_equal(
            samples,
            &expected,
            &format!("plane {plane}: e's input, d-e's delayed zero mix alone"),
        );
    }
    assert_eq!(
        mixes[route_index(&model, "d-e")],
        BLOCKS as u64,
        "the delayed muted route mixes every block"
    );
    assert_eq!(
        mixes[route_index(&model, "s-e")],
        0,
        "the undelayed muted route never mixes"
    );
}

// ---- #1217 gate 5 ---------------------------------------------------------------------------

/// After block 0, every render of `document` allocates and frees nothing: the render audit and
/// `bench_support::alloc`'s thread-scoped counters both read exact zero around each call.
fn assert_renders_without_allocating(document: &str, feeds: &[(String, [Vec<f32>; 2])]) {
    let compiled = compile_host_session(document, &caps()).expect("compile");
    let prepared = prepare_host_runtime(&compiled, &caps()).expect("prepare");
    assert!(
        graph::test_only_route_activity_built(),
        "the plan builds its route-activity table, or this is vacuous"
    );
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
        let mut pcm = [HOST_SENTINEL; QUANTUM * 2];
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

/// #1217 gate 5. Gate 1's bus and output sessions, its in-place chain and gate 3's delayed bus
/// render without one allocator call after warm-up.
///
/// Red if the route-activity table or a destination's route inputs are built or resized on the
/// render thread.
#[test]
fn route_activity_renders_without_allocating() {
    let mut draw = Draw::new(12_175);
    for sum in [Sum::Bus, Sum::Output] {
        let (model, _, feeds) = two_contributors(&mut draw, sum, 0);
        assert_renders_without_allocating(&document(&model), &feeds);
    }
    let one = vec![("t".to_owned(), planes(&mut draw, None))];
    assert_renders_without_allocating(&document(&in_place_chain(true)), &one);
    let two: Vec<(String, [Vec<f32>; 2])> = ["d", "s"]
        .iter()
        .map(|id| ((*id).to_owned(), planes(&mut draw, None)))
        .collect();
    assert_renders_without_allocating(&document(&delayed_session_checked(delayed_bus(true))), &two);
}

// ---- #1218: a send that follows its source strip's mute -----------------------------------

/// Which strip a follow send leaves.
#[derive(Clone, Copy, Debug)]
enum FollowSource {
    /// Track `t` itself.
    Track,
    /// Submix `s`, which `t` feeds by the unity route `t-s` from its `input` tap.
    Submix,
}

/// #1218 gates 1 and 4. The source strip (`t`, or `s` fed by `t`) has its `lane` muted and sends
/// `send` from its `pre_fader` tap, with `gain_db` and `matrix`, into bus `b`, whose `input` tap
/// feeds the output by the unity route `b-main`. `send` follows the source's mute when `follows`.
fn follow_send(
    source: FollowSource,
    lane: usize,
    gain_db: f32,
    matrix: &ChannelMatrix,
    follows: bool,
) -> SessionModel {
    let (mut model, source_pcm, track) = empty_session();
    add_track(&mut model, &source_pcm, &track, "t");
    let tap = SendTap::PreFader;
    let (from, fader) = match source {
        FollowSource::Track => (
            RouteSource::Track {
                track_id: sid("t"),
                tap,
            },
            &mut model.tracks[0].fader,
        ),
        FollowSource::Submix => {
            model
                .routes
                .push(route("t-s", input_tap("t"), into_bus("s")));
            model.submixes.push(Submix::unity(sid("s"), &model.console));
            (
                RouteSource::Submix {
                    submix_id: sid("s"),
                    tap,
                },
                &mut model.submixes[0].fader,
            )
        }
    };
    match lane {
        0 => fader.left_mute = true,
        _ => fader.right_mute = true,
    }
    let mut send = route("send", from, into_bus("b"));
    send.gain_db = gain_db;
    send.channel_matrix = matrix.clone();
    send.follows_mute = follows;
    model.routes.push(send);
    model.routes.push(to_output("b-main", bus_input("b")));
    model.submixes.push(Submix::unity(sid("b"), &model.console));
    model
}

/// `matrix` with source lane `lane`'s column (`ll` and `rl` for the left, `lr` and `rr` for the
/// right) set to `+0.0`: what a follow of a muted `lane` must render as.
fn column_zeroed(matrix: &ChannelMatrix, lane: usize) -> ChannelMatrix {
    let mut zeroed = matrix.clone();
    match lane {
        0 => (zeroed.ll, zeroed.rl) = (0.0, 0.0),
        _ => (zeroed.lr, zeroed.rr) = (0.0, 0.0),
    }
    zeroed
}

const FOLLOW_COLUMN: &str = "a_follow_send_zeroes_its_muted_source_lanes_column";
const FOLLOW_COLUMN_REPLAY: &str = "cargo test -p host-core --features host-core/test-support \
                                    --test route_mute -- --exact \
                                    a_follow_send_zeroes_its_muted_source_lanes_column";

/// #1218 gates 1 and 4. A `pre_fader` follow send from a strip with one muted lane renders
/// exactly the same session with that lane's matrix column at `+0.0` and no follow: the muted
/// lane's column contributes nothing and the other column is unchanged. Gate 1 is a track with
/// `left_mute`; gate 4 is a submix with `right_mute` (the construction mirrored); each also runs
/// the other lane. The send's matrix and its source's two lanes are distinct, so a zeroed column
/// is visible on both output lanes; without the follow the muted lane leaks (the pre-fader tap is
/// un-gated), so the comparison is not vacuous.
///
/// Red if the flag is ignored (the muted column leaks), zeroes the wrong column (the lanes swap),
/// is applied through the gain (both columns go silent), or reads source mutes from tracks only
/// (the submix case leaks).
#[test]
fn a_follow_send_zeroes_its_muted_source_lanes_column() {
    let ran = run_seeds(FOLLOW_COLUMN, FOLLOW_COLUMN_REPLAY, 8, |seed| {
        for (source, lane) in [
            (FollowSource::Track, 0),
            (FollowSource::Submix, 1),
            (FollowSource::Track, 1),
            (FollowSource::Submix, 0),
        ] {
            let mut draw = Draw::new(seed);
            let gain_db = draw.in_domain(-12.0, 6.0);
            let mut coefficient = || {
                let negative = draw.chance(1, 2);
                signed(&mut draw, negative)
            };
            let matrix = ChannelMatrix {
                ll: coefficient(),
                lr: coefficient(),
                rl: coefficient(),
                rr: coefficient(),
            };
            let feeds = vec![("t".to_owned(), planes(&mut draw, None))];
            let what = format!("seed {seed}, {source:?} lane {lane} muted");
            let (actual, _) = render(
                &document(&follow_send(source, lane, gain_db, &matrix, true)),
                &feeds,
            );
            let zeroed = column_zeroed(&matrix, lane);
            let (expected, _) = render(
                &document(&follow_send(source, lane, gain_db, &zeroed, false)),
                &feeds,
            );
            for plane in 0..2 {
                assert_bits_equal(
                    &actual[plane],
                    &expected[plane],
                    &format!("{what}, plane {plane}"),
                );
                assert!(
                    actual[plane].iter().any(|sample| *sample != 0.0),
                    "{what}: plane {plane} carries the open lane's column"
                );
            }
            let (leak, _) = render(
                &document(&follow_send(source, lane, gain_db, &matrix, false)),
                &feeds,
            );
            assert_ne!(
                leak, actual,
                "{what}: without the follow the muted lane leaks"
            );
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 8);
    }
}

const FOLLOW_SILENT: &str = "a_fully_follow_muted_undelayed_send_is_inactive";
const FOLLOW_SILENT_REPLAY: &str = "cargo test -p host-core --features host-core/test-support \
                                    --test route_mute -- --exact \
                                    a_fully_follow_muted_undelayed_send_is_inactive";

/// #1218 gate 2, undelayed. Gate 1 of #1217's bus, with no route muted: `r0` follows `t0`, whose
/// two lanes are muted, so `r0` is silenced and, undelayed, inactive. The bus renders the P4/D3
/// oracle (`r0` first in route-ID order owns the store with `+0.0`, `r1` is added) bit for bit, and
/// `r0`'s op never mixes while `r1`'s mixes every block.
///
/// Red if a fully follow-muted undelayed send is still mixed (its counter, and the signed zeros
/// its zero-coefficient mix would add), or if the follow is ignored (the oracle omits `r0`'s
/// pre-fader audio).
#[test]
fn a_fully_follow_muted_undelayed_send_is_inactive() {
    let ran = run_seeds(FOLLOW_SILENT, FOLLOW_SILENT_REPLAY, 16, |seed| {
        let mut draw = Draw::new(seed);
        // No route muted: both contributors carry their signed-zero frames.
        let (mut model, routes, feeds) = two_contributors(&mut draw, Sum::Bus, 2);
        let fader = &mut model.tracks[0].fader;
        (fader.left_mute, fader.right_mute) = (true, true);
        assert_eq!(model.routes[0].id.as_str(), "r0");
        model.routes[0].follows_mute = true;
        let (actual, mixes) = render_counted(&document(&model), &feeds);
        assert!(
            graph::test_only_route_activity_built(),
            "seed {seed}: a fully follow-zeroed send builds the route-activity table"
        );
        let contributions = vec![
            (false, mix(&routes[0], &feeds[0].1)),
            (true, mix(&routes[1], &feeds[1].1)),
        ];
        let expected = mix(
            &to_output("b-main", bus_input("b")),
            &d3_sum(&contributions),
        );
        for plane in 0..2 {
            assert_bits_equal(
                &actual[plane],
                &expected[plane],
                &format!("seed {seed}, plane {plane}"),
            );
        }
        assert_eq!(mixes[route_index(&model, "r0")], 0, "seed {seed}: r0 mixed");
        assert_eq!(
            mixes[route_index(&model, "r1")],
            BLOCKS as u64,
            "seed {seed}: r1 mixes every block"
        );
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 16);
    }
}

/// #1218 gate 2, delayed. #1217 gate 3's bus with `d-e` open but following `d`, whose two lanes
/// are muted: `d-e`'s edge into `e` carries the limiter's 486-sample compensation (asserted from
/// the compiled plan), so it stays active, mixes `[+0.0; 4]` every block, and the bus renders that
/// gate's oracle.
///
/// Red if a fully follow-muted delayed send is skipped (its counter stops, and its consumer would
/// stage a buffer this block never wrote) or still mixes its open coefficients.
#[test]
fn a_fully_follow_muted_delayed_send_stays_active() {
    let mut model = delayed_bus(true);
    assert_eq!(model.tracks[0].id.as_str(), "d");
    let fader = &mut model.tracks[0].fader;
    (fader.left_mute, fader.right_mute) = (true, true);
    let follow = model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "d-e")
        .expect("d-e is declared");
    (follow.mute, follow.follows_mute) = (false, true);
    assert_delayed_route_stays_active(&delayed_session_checked(model));
}
