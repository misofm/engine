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
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
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
        input_floors: Default::default(),
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

// ---- #1652: a following send of a stored mute ---------------------------------------------

/// #1652 (batch 18, #1652c): a send into a submix whose source strip automates its mute holds the
/// seven-word cell on every plan and follows the curve in render at its tap's arrival.
#[cfg(feature = "test-support")]
#[path = "support/successor.rs"]
mod successor;

#[cfg(feature = "test-support")]
mod follow {
    use super::successor;
    use super::{
        ChannelMatrix, Effect, EffectIdentity, EffectQuality, LinkMode, RouteSource, SendTap,
        SessionModel, SidechainDeclaration, Submix, add_track, bench_alloc, empty_session,
        into_bus, route, sid, to_output,
    };
    use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
    use graph_compiler::Backend;
    use host_core::{HostLiveControlRequest, PreparedHost, SourceSubmission};
    use session::{
        Automation, AutomationSegment, AutomationShape, ControlAddress, ControlSmoothing, Owner,
        ParameterChannel, ParameterUnit, Section,
    };

    /// The session mute length: `mute_ms` 10 at 48 kHz.
    pub(super) const M: u32 = 480;
    /// The mute row (builtin parameter 6).
    const MUTE: u32 = 6;
    /// The true-peak limiter's prepared latency at 48 kHz: no native effect declares 64.
    pub(super) const L: u64 = 486;

    /// Every bank width of the target and the scalar oracle (R17-4).
    pub(super) fn backends() -> Vec<Backend> {
        vec![
            #[cfg(target_feature = "avx2")]
            Backend::Simd8,
            Backend::Simd4,
            Backend::Scalar,
        ]
    }

    /// Track `t` (two-channel source `t`) and bus `b`. `t-main` feeds the output's left plane from
    /// `t`'s left lane (`ll` only); the send `t-b` feeds bus `b`'s right plane from `t`'s right
    /// lane at `tap` (`rr` only), and `follows_mute` when `follows`; `b-main` takes the bus to
    /// the output. So the output's left plane is the strip and its right plane is the send. With
    /// `latent`, `t` carries a true-peak limiter insert ([`L`] samples, before the fader).
    pub(super) fn model(quantum: u32, tap: SendTap, latent: bool, follows: bool) -> SessionModel {
        let (mut model, source, track) = empty_session();
        model.quantum_frames = quantum;
        model.control_smoothing = Some(ControlSmoothing {
            mute_ms: 10.0,
            fader_ms: 5.0,
            pan_ms: 5.0,
        });
        let mut source = source;
        source.frames = 1 << 16;
        add_track(&mut model, &source, &track, "t");
        if latent {
            model.tracks[0].inserts.effects.push(Effect {
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
        }
        let mut direct = to_output(
            "t-main",
            RouteSource::Track {
                track_id: sid("t"),
                tap: SendTap::PostPan,
            },
        );
        direct.channel_matrix = ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 0.0,
        };
        let mut send = route(
            "t-b",
            RouteSource::Track {
                track_id: sid("t"),
                tap,
            },
            into_bus("b"),
        );
        send.channel_matrix = ChannelMatrix {
            ll: 0.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        };
        send.follows_mute = follows;
        model.routes.push(direct);
        model.routes.push(send);
        model.routes.push(to_output(
            "b-main",
            RouteSource::Submix {
                submix_id: sid("b"),
                tap: SendTap::PostPan,
            },
        ));
        model.submixes.push(Submix::unity(sid("b"), &model.console));
        assert_eq!(controls::LiveRamps::for_session(&model).mute_samples, M);
        model
    }

    /// A mute entry on `both` lanes of track `t`, from `from`, with a step to each `(sample,
    /// value)` of `steps`.
    pub(super) fn mute_entry(from: f32, steps: &[(u64, f32)]) -> Automation {
        mute_entry_on("t", from, steps)
    }

    /// [`mute_entry`] on strip `strip`.
    pub(super) fn mute_entry_on(strip: &str, from: f32, steps: &[(u64, f32)]) -> Automation {
        let mut at = (0_u64, from);
        let mut segments: Vec<AutomationSegment> = steps
            .iter()
            .map(|&(sample, value)| {
                let segment = AutomationSegment {
                    shape: AutomationShape::Step,
                    start_sample: at.0,
                    end_sample: sample,
                    start_value: at.1,
                    end_value: value,
                    unit: ParameterUnit::Linear,
                };
                at = (sample, value);
                segment
            })
            .collect();
        if segments.is_empty() {
            segments.push(AutomationSegment {
                shape: AutomationShape::Step,
                start_sample: 0,
                end_sample: 1,
                start_value: from,
                end_value: from,
                unit: ParameterUnit::Linear,
            });
        }
        Automation {
            id: sid("mute-entry"),
            target: ControlAddress {
                owner: Owner::Strip(sid(strip)),
                section: Section::Builtins,
                control: MUTE,
                lane: ParameterChannel::Both,
            },
            segments,
            offset: session::AutomationOffset::default(),
        }
    }

    /// `model` with `t`'s two lanes statically muted.
    pub(super) fn statically_muted(mut model: SessionModel) -> SessionModel {
        let fader = &mut model.tracks[0].fader;
        (fader.left_mute, fader.right_mute) = (true, true);
        model
    }

    /// A two-plane feed of `frames` samples: DC `value`, or with `value = None` a signed
    /// pseudo-random signal that is never zero, half its samples negative (signed zeros show).
    pub(super) fn feed(frames: usize, value: Option<f32>) -> [Vec<f32>; 2] {
        let plane = |channel: u32| {
            (0..frames as u32)
                .map(|frame| {
                    value.unwrap_or_else(|| {
                        let step = frame
                            .wrapping_mul(7_919)
                            .wrapping_add(channel * 31)
                            .wrapping_add(17)
                            % 1_999;
                        (step as f32 - 999.5) / 2_000.0
                    })
                })
                .collect::<Vec<f32>>()
        };
        [plane(0), plane(1)]
    }

    /// How a plan is prepared: no live control or live controls.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(super) enum Live {
        None,
        Controls,
    }

    pub(super) const LIVE_FORMS: [Live; 2] = [Live::None, Live::Controls];

    /// `model` prepared on `backend` in form `live`.
    pub(super) fn prepare(
        model: &SessionModel,
        backend: Backend,
        live: Live,
    ) -> (PreparedHost, Option<host_core::HostLiveControlHandles>) {
        let document = super::document(model);
        let caps = host_core::HostPrepareCaps {
            source_ring_frames: 32 * model.quantum_frames,
            ..super::caps()
        };
        let compiled = host_core::compile_host_session(&document, &caps)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let request = HostLiveControlRequest::default();
        let fail = |failure: host_core::PrepareDiagnostics| -> ! {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        };
        match live {
            Live::None => (
                host_core::test_only_prepare_host_runtime_on(&compiled, &caps, backend)
                    .unwrap_or_else(|failure| fail(failure)),
                None,
            ),
            Live::Controls => {
                let (prepared, handles) =
                    host_core::test_only_prepare_host_runtime_with_live_controls_on(
                        &compiled, &caps, &request, backend,
                    )
                    .unwrap_or_else(|failure| fail(failure));
                (prepared, Some(handles))
            }
        }
    }

    /// What one run rendered: the output planes and the render calls' allocator counts after the
    /// first block, `(allocations, reallocations, deallocations)`.
    pub(super) struct Run {
        pub(super) out: [Vec<f32>; 2],
        pub(super) allocations: (u64, u64, u64),
        pub(super) follow_routes: usize,
    }

    /// `model` on `backend` in form `live`, fed `pcm` on source `t`, for `frames` samples.
    pub(super) fn run(
        model: &SessionModel,
        backend: Backend,
        live: Live,
        pcm: &[Vec<f32>; 2],
        frames: usize,
    ) -> Run {
        let quantum = model.quantum_frames as usize;
        let (mut prepared, handles) = prepare(model, backend, live);
        let follow_routes = handles
            .as_ref()
            .map_or(usize::MAX, |handles| handles.follow_route_controls.len());
        let rate = prepared.report.sample_rate_hz;
        let mut out = [Vec::new(), Vec::new()];
        let mut allocations = (0, 0, 0);
        bench_alloc::assert_installed();
        let mut at = 0;
        while at < frames {
            prepared
                .sources
                .submit(
                    b"t",
                    SourceSubmission {
                        generation: 1,
                        start_frame: at as u64,
                        sample_rate_hz: rate,
                        planes: &[&pcm[0][at..at + quantum], &pcm[1][at..at + quantum]],
                        frames: quantum as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
            let mut samples = vec![0.0_f32; quantum * 2];
            let output =
                PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
            let mark = bench_alloc::current_thread_counters();
            prepared
                .plan
                .render(
                    RenderIo { output },
                    RenderTime {
                        absolute_sample: at as u64,
                    },
                )
                .expect("render");
            let delta = bench_alloc::current_thread_delta_since(mark);
            if at > 0 {
                allocations.0 += delta.allocations;
                allocations.1 += delta.reallocations;
                allocations.2 += delta.deallocations;
            }
            out[0].extend_from_slice(&samples[..quantum]);
            out[1].extend_from_slice(&samples[quantum..]);
            at += quantum;
        }
        drop(handles);
        Run {
            out: out.map(|mut plane| {
                plane.truncate(frames);
                plane
            }),
            allocations,
            follow_routes,
        }
    }

    /// The first index at or after `from` where `a` and `b` differ by bits.
    pub(super) fn first_difference_from(a: &[f32], b: &[f32], from: usize) -> Option<usize> {
        (from..a.len().min(b.len())).find(|&index| a[index].to_bits() != b[index].to_bits())
    }

    /// The first index at or after `from` where `plane` is exact `+0.0`.
    pub(super) fn first_zero_from(plane: &[f32], from: usize) -> Option<usize> {
        (from..plane.len()).find(|&index| plane[index].to_bits() == 0)
    }

    /// A predecessor of `a` with live controls rendered for [`successor::SWAP_BLOCK`] blocks (with
    /// `edit` run before each block's render, writing through its handles and gate), then a
    /// successor of `b` prepared from the predecessor's inventory and committed model
    /// `committed`, swapped in and rendered to [`successor::BLOCKS`]; or, with `b = None`, the
    /// predecessor alone for every block (the never-swapped twin). Fed `pcm` on source `t`.
    pub(super) fn live_swap(
        a: &SessionModel,
        b: Option<(&SessionModel, &SessionModel)>,
        backend: Backend,
        pcm: &successor::Feed,
        edit: impl FnMut(
            usize,
            &mut host_core::HostLiveControlHandles,
            &mut engine::realtime::PlanRevisionGate,
        ),
    ) -> (
        Vec<successor::Block>,
        Option<host_core::HostLiveControlHandles>,
    ) {
        live_swap_ramped(a, b, backend, pcm, &session::EditRamps::new(), edit)
    }

    /// [`live_swap`] with the transaction's edit ramps `edit_ramps` given to the successor's
    /// base.
    pub(super) fn live_swap_ramped(
        a: &SessionModel,
        b: Option<(&SessionModel, &SessionModel)>,
        backend: Backend,
        pcm: &successor::Feed,
        edit_ramps: &session::EditRamps,
        mut edit: impl FnMut(
            usize,
            &mut host_core::HostLiveControlHandles,
            &mut engine::realtime::PlanRevisionGate,
        ),
    ) -> (
        Vec<successor::Block>,
        Option<host_core::HostLiveControlHandles>,
    ) {
        use successor::{BLOCKS, SWAP_BLOCK};
        let request = HostLiveControlRequest::default();
        let caps = successor::caps();
        let session = successor::Session::compile(a.clone());
        let (prepared, mut handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_on(
                &session.compiled,
                &caps,
                &request,
                backend,
            )
            .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
        let rate = prepared.report.sample_rate_hz;
        let PreparedHost {
            mut plan,
            sources: mut a_sources,
            inventory,
            ..
        } = prepared;
        let mut gate = plan.take_revision_gate().expect("the plan's gate");
        let (mut publisher, mut owner, _retirer) = successor::exchange(plan);
        pcm.submit(&mut a_sources, rate, 0);
        let mut blocks = Vec::with_capacity(BLOCKS);
        let swap_at = if b.is_some() { SWAP_BLOCK } else { BLOCKS };
        for block in 0..swap_at {
            if block + 1 < BLOCKS {
                pcm.submit(&mut a_sources, rate, block + 1);
            }
            edit(block, &mut handles, &mut gate);
            blocks.push(successor::render(&mut owner));
        }
        let Some((b, committed)) = b else {
            return (blocks, Some(handles));
        };
        let next = successor::Session::compile(b.clone());
        let committed = successor::Session::compile(committed.clone()).model;
        let revision = gate.last_published() + 1;
        let (mut prepared, successor_handles) =
            host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                &next.compiled,
                &caps,
                &request,
                host_core::SuccessorBase {
                    edit_ramps,
                    ..host_core::SuccessorBase::new(&inventory, &committed, revision)
                },
                backend,
            )
            .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
        prepared.sources.adopt_persisting(&mut a_sources);
        let PreparedHost {
            mut plan,
            sources: mut b_sources,
            ..
        } = prepared;
        plan.take_revision_gate()
            .expect("the successor's gate")
            .publish(revision);
        publisher
            .reserve_replacement(plan, engine::realtime::PlanAdoption::Next)
            .unwrap_or_else(|_| panic!("reserve the successor"))
            .commit(revision);
        for block in SWAP_BLOCK..BLOCKS {
            if block + 1 < BLOCKS {
                pcm.submit(&mut b_sources, rate, block + 1);
            }
            blocks.push(successor::render(&mut owner));
        }
        drop(handles);
        (blocks, Some(successor_handles))
    }

    /// Plane `plane` of a run's blocks as samples.
    pub(super) fn plane(blocks: &[successor::Block], plane: usize) -> Vec<f32> {
        let quantum = successor::QUANTUM;
        blocks
            .iter()
            .flat_map(|block| {
                block.bits[plane * quantum..(plane + 1) * quantum]
                    .iter()
                    .map(|bits| f32::from_bits(*bits))
            })
            .collect()
    }

    /// Every render call after block 0 allocated and freed nothing, by the thread counters and
    /// the render audit.
    pub(super) fn assert_quiet(blocks: &[successor::Block], what: &str) {
        for (index, block) in blocks.iter().enumerate().skip(1) {
            assert_eq!(
                (block.allocator, block.audit),
                ((0, 0, 0), (0, 0)),
                "{what}: block {index} allocated or freed"
            );
        }
    }

    /// `sample` rounded up to a block boundary of `quantum`.
    pub(super) const fn next_boundary(sample: usize, quantum: usize) -> usize {
        sample.div_ceil(quantum) * quantum
    }
}

/// #1652 gate 1 (host-core form, both lanes, every bank width and the scalar oracle, every live
/// form). Track `t` sends `pre_fader` with `follows_mute` into bus `b`; `t` has a mute entry on
/// `both` with steps 0, 1, 0 at samples 1,000 and 2,500 (not grid samples), fed a signed signal.
/// From the first block boundary after the end of the mute ramp to the unmute, the output is bit
/// for bit a plan with `t` statically muted, and from the first block boundary after the end of
/// the unmute ramp it is bit for bit a plan with no mute, at quanta 128 and 100 (the send has no
/// compensation delay). Each plan has exactly one following route, and render allocates nothing.
///
/// A muted route that follows its source (`mute = true`, the strip open until a late step) renders
/// exact `+0.0` on the send plane at every width, while the strip is heard (U2b MINOR-1).
///
/// Test value: red if a following send ignores the curve (its pre-fader audio leaks while the
/// strip is muted), if a plan without live controls attaches no lane for it, if the send does
/// not reopen after the unmute, or if the seeded flags drop the route's own `mute` (a muted
/// following send leaks).
#[cfg(feature = "test-support")]
#[test]
fn a_following_send_leaks_nothing_of_a_stored_mute() {
    use follow::*;
    const FRAMES: usize = 4_000;
    const DOWN: usize = 1_000;
    const UP: usize = 2_500;
    let pcm = feed(FRAMES + 256, None);
    for quantum in [128_usize, 100] {
        let base = model(quantum as u32, SendTap::PreFader, false, true);
        let mut automated = base.clone();
        automated.automation = vec![mute_entry(0.0, &[(DOWN as u64, 1.0), (UP as u64, 0.0)])];
        let muted = statically_muted(base.clone());
        let mut leaking = automated.clone();
        leaking.routes[1].follows_mute = false;
        assert_eq!(leaking.routes[1].id.as_str(), "t-b");
        for backend in follow::backends() {
            for live in LIVE_FORMS {
                let what = format!("q={quantum} {backend:?} {live:?}");
                let curve = run(&automated, backend, live, &pcm, FRAMES);
                if live != Live::None {
                    assert_eq!(curve.follow_routes, 1, "{what}: one following route");
                }
                assert_eq!(curve.allocations, (0, 0, 0), "{what}: render allocates");
                let fixed = run(&muted, backend, live, &pcm, FRAMES);
                let open = run(&base, backend, live, &pcm, FRAMES);
                let muted_window = next_boundary(DOWN + M as usize, quantum)..UP;
                let open_window = next_boundary(UP + M as usize, quantum)..FRAMES;
                for plane in 0..2 {
                    assert_bits_equal(
                        &curve.out[plane][muted_window.clone()],
                        &fixed.out[plane][muted_window.clone()],
                        &format!("{what}: plane {plane} while muted"),
                    );
                    assert_bits_equal(
                        &curve.out[plane][open_window.clone()],
                        &open.out[plane][open_window.clone()],
                        &format!("{what}: plane {plane} after the unmute"),
                    );
                }
                // Not vacuous: without the follow the pre-fader send leaks while muted.
                let leak = run(&leaking, backend, live, &pcm, FRAMES);
                assert!(
                    first_difference_from(&leak.out[1], &fixed.out[1], muted_window.start)
                        .is_some_and(|index| index < UP),
                    "{what}: without the follow the send leaks"
                );
            }
        }
    }
    // The seeded flags carry the route's own `mute` (U2b MINOR-1): a muted route that follows
    // its source, whose strip is open until a late step, renders exact `+0.0` on the send plane.
    let mut stored = model(128, SendTap::PreFader, false, true);
    stored.routes[1].mute = true;
    stored.automation = vec![mute_entry(0.0, &[(3_000, 1.0)])];
    assert_eq!(stored.routes[1].id.as_str(), "t-b");
    for backend in follow::backends() {
        let what = format!("muted route {backend:?}");
        let run = run(&stored, backend, Live::None, &pcm, FRAMES);
        assert!(
            run.out[1].iter().all(|sample| sample.to_bits() == 0),
            "{what}: a muted following send leaks"
        );
        assert!(
            run.out[0].iter().any(|sample| *sample != 0.0),
            "{what}: the strip itself is heard"
        );
    }
}

/// #1652 gate 2 (every bank width and the scalar oracle, quanta 128 and 100). Track `t` carries a
/// latent insert of `L` samples before its fader (a true-peak limiter, `L = 486` at 48 kHz: no
/// native effect declares 64) and a mute entry on `both` with steps at 1,000 and 2,500, fed DC.
/// The output's left plane is the strip, its right plane the send into bus `b`. With an `input`
/// tap the send reads `t` `L` render samples before the strip's fader does, and PDC delays the
/// bus path by `L` at the output; with a `post_fader` tap both read at the fader's arrival. In both
/// cases the send's gate change and the strip's mute ramp start on the same output sample, the
/// one where the strip's audio of timeline sample 1,000 arrives (and of 2,500 for the unmute), and
/// with an `input` tap they reach exact `+0.0` after the same length.
///
/// Test value: red if the route op times its events at the fader's arrival (an `input` tap would
/// gate `L` samples late at the output and leak) or at the route's own arrival, or if the send
/// ramp and the strip ramp differ in length.
#[cfg(feature = "test-support")]
#[test]
fn the_send_gates_on_the_timeline_sample_the_strip_mutes() {
    use follow::*;
    const FRAMES: usize = 4_000;
    let pcm = feed(FRAMES + 256, Some(0.25));
    for quantum in [128_usize, 100] {
        for tap in [SendTap::Input, SendTap::PostFader] {
            let with_curve = |follows: bool| {
                let mut model = model(quantum as u32, tap, true, follows);
                model.automation = vec![mute_entry(0.0, &[(1_000, 1.0), (2_500, 0.0)])];
                model
            };
            let following = with_curve(true);
            let still = with_curve(false);
            let plain = model(quantum as u32, tap, true, true);
            let muted = statically_muted(plain.clone());
            for backend in follow::backends() {
                let what = format!("q={quantum} {tap:?} {backend:?}");
                let a = run(&following, backend, Live::None, &pcm, FRAMES);
                let b = run(&still, backend, Live::None, &pcm, FRAMES);
                let c = run(&plain, backend, Live::None, &pcm, FRAMES);
                let d = run(&muted, backend, Live::None, &pcm, FRAMES);
                // The mute against the open twins (the strip with no curve, the send with no
                // follow); the unmute against the statically muted twin, strip and send.
                for (from, event, strip_twin, send_twin) in
                    [(0, "mute", &c, &b), (2_000, "unmute", &d, &d)]
                {
                    let strip = first_difference_from(&a.out[0], &strip_twin.out[0], from)
                        .unwrap_or_else(|| panic!("{what}: the strip {event}s"));
                    let send = first_difference_from(&a.out[1], &send_twin.out[1], from)
                        .unwrap_or_else(|| panic!("{what}: the send {event}s"));
                    assert_eq!(
                        send, strip,
                        "{what}: the send's {event} starts on the strip's output sample"
                    );
                    let step = if event == "mute" { 1_000 } else { 2_500 };
                    assert!(
                        (step + L as usize..step + L as usize + 2).contains(&strip),
                        "{what}: the {event} lands where timeline {step} arrives, at {strip}"
                    );
                }
                if tap == SendTap::Input {
                    let strip = first_difference_from(&a.out[0], &c.out[0], 0).expect("mute");
                    let send = first_difference_from(&a.out[1], &b.out[1], 0).expect("mute");
                    let strip_end = first_zero_from(&a.out[0], strip).expect("the strip mutes");
                    let send_end = first_zero_from(&a.out[1], send).expect("the send mutes");
                    assert_eq!(
                        send_end - send,
                        strip_end - strip,
                        "{what}: the two mute ramps have one length"
                    );
                    assert!(
                        (M as usize - 1..=M as usize).contains(&(send_end - send)),
                        "{what}: the length is the session's mute length, {}",
                        send_end - send
                    );
                }
            }
        }
    }
}

/// The send gain record a host's commit writes for `t-b` going to `gain_db` over `length`, its
/// right-to-right matrix kept and the gate open.
#[cfg(feature = "test-support")]
fn send_gain(gain_db: f32, length: u32) -> host_core::RouteControlRecord {
    host_core::route_record(gain_db, [0.0, 0.0, 0.0, 1.0], false, [false; 2], length)
        .expect("an in-domain send record")
}

/// #1652 gate 4(a) (every bank width and the scalar oracle). A predecessor with live controls
/// and no stored automation ramps its send `t-b` toward `-12` dB over 2,000 samples from block 4.
/// A rebuild at block 6 adds a mute entry on `t` (held unmuted, then a step to muted at 1,000):
/// the successor gives `t-b` the seven-word cell, so the route changes kind. It keeps the send's
/// ramp by route ID: until the step, the swapped run is bit for bit the never-swapped
/// predecessor. The join writes no seven-word record (no word the models hold changed), the swap
/// block and every later block allocate and free nothing, and from the step the send follows.
///
/// Test value: red if gaining automation on the source re-points the send and drops its ramp
/// (the successor starts at its prepared `-12` dB), if the join writes a record only because the
/// kind changed, or if the swap allocates.
#[cfg(feature = "test-support")]
#[test]
fn a_rebuild_that_adds_a_mute_entry_keeps_the_sends_ramp() {
    use follow::*;
    use successor::{QUANTUM, SWAP_BLOCK};
    let pcm = successor::Feed::new("t", 11);
    let a = model(QUANTUM as u32, SendTap::PreFader, false, true);
    let mut committed = a.clone();
    committed.routes[1].gain_db = -12.0;
    assert_eq!(committed.routes[1].id.as_str(), "t-b");
    let mut b = committed.clone();
    b.automation = vec![mute_entry(0.0, &[(1_000, 1.0)])];
    let ramp = |block: usize,
                handles: &mut host_core::HostLiveControlHandles,
                gate: &mut engine::realtime::PlanRevisionGate| {
        use builtins_compiler::TestOnlyWriteGate;
        if block == 4 {
            let position = host_core::route_producer_position(&handles.route_controls, "t-b")
                .expect("t-b is a six-word route of the predecessor");
            gate.write_published(|stamp| {
                handles.route_controls[position].write(stamp, send_gain(-12.0, 2_000));
            });
        }
    };
    for backend in follow::backends() {
        let what = format!("{backend:?}");
        let (twin, _) = live_swap(&a, None, backend, &pcm, ramp);
        let (swapped, handles) = live_swap(&a, Some((&b, &committed)), backend, &pcm, ramp);
        let handles = handles.expect("the successor's handles");
        assert_eq!(handles.follow_route_controls.len(), 1, "{what}");
        assert!(
            host_core::route_producer_position(&handles.route_controls, "t-b").is_none(),
            "{what}: a following route has no six-word lane"
        );
        assert_eq!(
            handles.follow_route_controls[0].test_only_write_count(),
            0,
            "{what}: the join writes no record for a change of kind"
        );
        assert_quiet(&swapped, &what);
        let step = 1_000;
        for index in 0..2 {
            let (swapped, twin) = (plane(&swapped, index), plane(&twin, index));
            assert_bits_equal(
                &swapped[..step],
                &twin[..step],
                &format!("{what}: plane {index} before the step"),
            );
            // The ramp is still moving at the swap, so the comparison is not vacuous.
            let at = SWAP_BLOCK * QUANTUM;
            if index == 1 {
                assert_ne!(
                    swapped[at] / pcm.planes[1][at],
                    swapped[step - 1] / pcm.planes[1][step - 1],
                    "{what}: the send ramps through the swap"
                );
                assert!(
                    swapped[step + M as usize..]
                        .iter()
                        .all(|sample| *sample == 0.0),
                    "{what}: the send follows the successor's curve"
                );
            }
        }
    }
}

/// #1652 gate 4(b) (every bank width and the scalar oracle). A predecessor whose mute entry holds
/// `t` muted (so its seven-word send `t-b` is follow-muted) is rebuilt at block 6 without the
/// entry: the successor's `t-b` is a six-word route again, and nothing in the two models differs
/// for it, so the join writes nothing. The send reopens over the session mute length from the
/// swap block: silent before it, ramping inside `[768, 768 + M)`, and from the next block
/// boundary the successor prepared fresh, bit for bit. The swap allocates and frees nothing.
///
/// Test value: red if the six-word successor keeps the adopted mute (the send stays silent), or
/// steps open instead of ramping, or if the swap allocates.
#[cfg(feature = "test-support")]
#[test]
fn a_rebuild_that_removes_the_mute_entry_reopens_the_send() {
    use follow::*;
    use successor::{BLOCKS, QUANTUM, SWAP_BLOCK};
    let pcm = successor::Feed::new("t", 13);
    let b = model(QUANTUM as u32, SendTap::PreFader, false, true);
    let mut a = b.clone();
    a.automation = vec![mute_entry(1.0, &[])];
    let quiet = |_: usize,
                 _: &mut host_core::HostLiveControlHandles,
                 _: &mut engine::realtime::PlanRevisionGate| {};
    for backend in follow::backends() {
        let what = format!("{backend:?}");
        let (swapped, handles) = live_swap(&a, Some((&b, &a)), backend, &pcm, quiet);
        let handles = handles.expect("the successor's handles");
        assert!(handles.follow_route_controls.is_empty(), "{what}");
        assert_quiet(&swapped, &what);
        let send = plane(&swapped, 1);
        let at = SWAP_BLOCK * QUANTUM;
        assert!(
            send[..at].iter().all(|sample| *sample == 0.0),
            "{what}: the send is follow-muted before the swap"
        );
        let ramp = &send[at..at + M as usize];
        assert!(
            ramp.iter().any(|sample| *sample != 0.0),
            "{what}: the send reopens from the swap block"
        );
        let open = next_boundary(at + M as usize, QUANTUM);
        assert!(
            ramp.iter()
                .zip(&pcm.planes[1][at..])
                .any(|(sample, source)| sample.to_bits() != source.to_bits()),
            "{what}: the send ramps open, it does not step"
        );
        let fresh =
            successor::reference_run(&successor::Session::compile(b.clone()), &[&pcm], backend);
        let fresh = plane(&fresh, 1);
        assert_bits_equal(
            &send[open..BLOCKS * QUANTUM],
            &fresh[open..BLOCKS * QUANTUM],
            &format!("{what}: open after the ramp"),
        );
    }
}

/// #1652 U2a fold 2, NIT-E (fold 1's MINOR-2, two swaps with a pending comparison; every bank
/// width and the scalar oracle). As gate 4(b), predecessor `A`'s mute entry holds `t` muted and
/// `B` removes it, so `B`'s six-word `t-b` leaves a comparison for its first block. Render adopts
/// `B` at block 6 in a call that fails before it renders (a time discontinuity: the host asked for
/// another sample), so `B` is displaced before its first block by `C`, the same session prepared
/// from `B`'s inventory, which carries `t-b` six -> six and is adopted by the next call. `C` takes
/// the state `B`'s first block would have rendered from: the send reopens over the session mute
/// length from that block, and from the next boundary it is `C` prepared fresh, bit for bit.
/// Neither swap allocates or frees on render: the refused call that adopts `B` is measured with
/// the thread counters and the render audit, as is every block after the first.
///
/// Test value: red if adopting a predecessor that never rendered drops the comparisons its own
/// carry left (`C` then takes the adopted mute, and the send stays silent for good), or if the
/// adoption of a displaced plan allocates or frees on the refused call.
#[cfg(feature = "test-support")]
#[test]
fn a_successor_of_a_plan_that_never_rendered_takes_its_pending_comparison() {
    use engine::realtime::{PlanAdoption, PlanarBufferMut, RenderError, RenderIo};
    use follow::*;
    use successor::{BLOCKS, QUANTUM, SWAP_BLOCK};
    let pcm = successor::Feed::new("t", 13);
    let b = model(QUANTUM as u32, SendTap::PreFader, false, true);
    let mut a = b.clone();
    a.automation = vec![mute_entry(1.0, &[])];
    let request = host_core::HostLiveControlRequest::default();
    let caps = successor::caps();
    for backend in follow::backends() {
        let what = format!("{backend:?}");
        let prepare =
            |model: &SessionModel,
             base: Option<(&host_core::PlanStateInventory, &SessionModel, u64)>| {
                let session = successor::Session::compile(model.clone());
                match base {
                    None => host_core::test_only_prepare_host_runtime_with_live_controls_on(
                        &session.compiled,
                        &caps,
                        &request,
                        backend,
                    ),
                    Some((inventory, committed, revision)) => {
                        let committed = successor::Session::compile(committed.clone()).model;
                        host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                            &session.compiled,
                            &caps,
                            &request,
                            host_core::SuccessorBase::new(inventory, &committed, revision),
                            backend,
                        )
                    }
                }
                .unwrap_or_else(|failure| panic!("{what}: prepare: {failure:?}"))
                .0
            };
        let first = prepare(&a, None);
        let rate = first.report.sample_rate_hz;
        let PreparedHost {
            mut plan,
            sources: mut a_sources,
            inventory: a_inventory,
            ..
        } = first;
        let gate = plan.take_revision_gate().expect("the plan's gate");
        let (mut publisher, mut owner, mut retirer) = successor::exchange(plan);
        pcm.submit(&mut a_sources, rate, 0);
        let mut blocks = Vec::with_capacity(BLOCKS);
        for block in 0..SWAP_BLOCK {
            pcm.submit(&mut a_sources, rate, block + 1);
            blocks.push(successor::render(&mut owner));
        }
        let mut publish = |prepared: PreparedHost, revision: u64| {
            let PreparedHost {
                mut plan,
                sources,
                inventory,
                ..
            } = prepared;
            plan.take_revision_gate()
                .expect("the successor's gate")
                .publish(revision);
            publisher
                .reserve_replacement(plan, PlanAdoption::Next)
                .unwrap_or_else(|_| panic!("{what}: reserve the successor"))
                .commit(revision);
            (sources, inventory)
        };
        let revision = gate.last_published() + 1;
        let mut b_prepared = prepare(&b, Some((&a_inventory, &a, revision)));
        b_prepared.sources.adopt_persisting(&mut a_sources);
        let (mut b_sources, b_inventory) = publish(b_prepared, revision);
        // Render adopts B at the block boundary, then refuses the call: B renders nothing.
        let mut output = [f32::NAN; QUANTUM * 2];
        let wrong = owner.next_absolute_sample() + 1;
        audit::reset();
        let mark = bench_alloc::current_thread_counters();
        let refused = owner.render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
            },
            wrong,
        );
        let delta = bench_alloc::current_thread_delta_since(mark);
        let snapshot = audit::snapshot();
        assert_eq!(
            (
                delta.allocations,
                delta.reallocations,
                delta.deallocations,
                snapshot.allocations,
                snapshot.deallocations
            ),
            (0, 0, 0, 0, 0),
            "{what}: the call that adopts B and refuses allocates or frees nothing"
        );
        assert!(
            matches!(refused, Err(RenderError::TimeDiscontinuity { .. })),
            "{what}: the call is refused after the boundary"
        );
        drop(retirer.try_reclaim().expect("A retired at B's adoption"));
        let mut c_prepared = prepare(&b, Some((&b_inventory, &b, revision + 1)));
        c_prepared.sources.adopt_persisting(&mut b_sources);
        let (mut c_sources, _) = publish(c_prepared, revision + 1);
        for block in SWAP_BLOCK..BLOCKS {
            if block + 1 < BLOCKS {
                pcm.submit(&mut c_sources, rate, block + 1);
            }
            blocks.push(successor::render(&mut owner));
        }
        assert_eq!(
            blocks[SWAP_BLOCK].swap,
            engine::realtime::SwapOutcome::Applied,
            "{what}: C is adopted at the swap block"
        );
        assert_quiet(&blocks, &what);
        let send = plane(&blocks, 1);
        let at = SWAP_BLOCK * QUANTUM;
        assert!(
            send[..at].iter().all(|sample| *sample == 0.0),
            "{what}: the send is follow-muted before the swaps"
        );
        let ramp = &send[at..at + M as usize];
        assert!(
            ramp.iter().any(|sample| *sample != 0.0),
            "{what}: the send reopens from the block C first renders"
        );
        assert!(
            ramp.iter()
                .zip(&pcm.planes[1][at..])
                .any(|(sample, source)| sample.to_bits() != source.to_bits()),
            "{what}: the send ramps open, it does not step"
        );
        let open = next_boundary(at + M as usize, QUANTUM);
        let fresh =
            successor::reference_run(&successor::Session::compile(b.clone()), &[&pcm], backend);
        let fresh = plane(&fresh, 1);
        assert_bits_equal(
            &send[open..BLOCKS * QUANTUM],
            &fresh[open..BLOCKS * QUANTUM],
            &format!("{what}: open after the ramp"),
        );
    }
}

/// #1652 gate 4, the combined transaction (coordinator obligation 3). One rebuild at block 6
/// changes the send's gain and adds a mute entry that holds `t` muted from the start: the strip
/// and the send both adopt the curve's mute at the swap, over the same session mute length, so the
/// strip's lane and the send reach exact `+0.0` on the same output sample, `M` samples after the
/// swap block starts.
///
/// The bank widths only: the scalar oracle binds the strip's fader as a graph node, which no
/// plan swap carries, so its strip restarts at the swap (muted exactly, as a fresh plan) while the
/// send carries.
///
/// Test value: red if the join's seven-word record (the gain edit's `SendGain` length) governs the
/// send's mute instead of the curve's own jump over the mute length, or if the strip and the send
/// adopt over different lengths (#1651 T4: red when a carried strip lane does not take on a
/// gained mute curve).
#[cfg(feature = "test-support")]
#[test]
fn a_send_edit_and_a_mute_entry_in_one_rebuild_adopt_over_one_length() {
    use follow::*;
    use successor::{QUANTUM, SWAP_BLOCK};
    let pcm = successor::Feed::new("t", 17);
    let a = model(QUANTUM as u32, SendTap::PreFader, false, true);
    let mut b = a.clone();
    b.routes[1].gain_db = -6.0;
    b.automation = vec![mute_entry(1.0, &[])];
    let quiet = |_: usize,
                 _: &mut host_core::HostLiveControlHandles,
                 _: &mut engine::realtime::PlanRevisionGate| {};
    for backend in follow::backends()
        .into_iter()
        .filter(|backend| *backend != Backend::Scalar)
    {
        let what = format!("{backend:?}");
        let (swapped, _) = live_swap(&a, Some((&b, &a)), backend, &pcm, quiet);
        assert_quiet(&swapped, &what);
        let at = SWAP_BLOCK * QUANTUM;
        let strip = first_zero_from(&plane(&swapped, 0), at);
        let send = first_zero_from(&plane(&swapped, 1), at);
        eprintln!("{what}: strip zero at {strip:?}, send zero at {send:?}");
        let (strip, send) = (
            strip.expect("the strip mutes"),
            send.expect("the send mutes"),
        );
        assert_eq!(send, strip, "{what}: one length for the strip and the send");
        assert!(
            (at + M as usize - 1..=at + M as usize).contains(&send),
            "{what}: over the mute length, at {send}"
        );
    }
}

/// #1653 U2b MINOR-2 (every bank width and the scalar oracle). A predecessor whose mute entry
/// holds `t` open is rebuilt at block 6 with `t` hand-muted (both lanes) and a transaction that
/// gave the strip's mute row an explicit 200-sample ramp, not the session's `M`. The send follows,
/// so the join's record is built from the source's effective mute in both models: its `before`
/// term is the open strip, its `after` term the muted one, so the record exists and ramps over the
/// gate length, the strip's 200. The send reaches exact `+0.0` 200 samples after the swap block
/// starts (the strip does too, on the bank widths: the scalar oracle restarts its strip at the
/// swap).
///
/// Test value: red if the join reads the source's mute of the successor for both terms (the
/// records of the two models then agree, no record is written, and the carried send reopens its
/// ramp over `M` instead of the edit's length).
#[cfg(feature = "test-support")]
#[test]
fn a_rebuild_that_hand_mutes_the_source_ramps_the_send_over_the_edits_length() {
    use follow::*;
    use successor::{QUANTUM, SWAP_BLOCK};
    const RAMP: u32 = 200;
    assert_ne!(RAMP, M);
    let pcm = successor::Feed::new("t", 19);
    let mut a = model(QUANTUM as u32, SendTap::PreFader, false, true);
    a.automation = vec![mute_entry(0.0, &[])];
    let b = statically_muted(a.clone());
    let mut ramps = session::EditRamps::new();
    ramps.set(
        &session::EditRampTarget::Strip(sid("t")),
        session::EditRampRow::Mute,
        Some(RAMP),
    );
    let quiet = |_: usize,
                 _: &mut host_core::HostLiveControlHandles,
                 _: &mut engine::realtime::PlanRevisionGate| {};
    for backend in follow::backends() {
        let what = format!("{backend:?}");
        let (swapped, handles) = live_swap_ramped(&a, Some((&b, &a)), backend, &pcm, &ramps, quiet);
        assert_eq!(
            handles
                .expect("the successor's handles")
                .follow_route_controls
                .len(),
            1,
            "{what}"
        );
        assert_quiet(&swapped, &what);
        let at = SWAP_BLOCK * QUANTUM;
        let send = plane(&swapped, 1);
        assert!(
            send[at..at + 8].iter().all(|sample| *sample != 0.0),
            "{what}: the send is open when the swap block starts"
        );
        let end = first_zero_from(&send, at).expect("the send mutes");
        assert!(
            (at + RAMP as usize - 1..=at + RAMP as usize).contains(&end),
            "{what}: the send reaches +0.0 over the edit's {RAMP} samples, at {}",
            end - at
        );
        if backend != Backend::Scalar {
            let strip = first_zero_from(&plane(&swapped, 0), at).expect("the strip mutes");
            assert_eq!(strip, end, "{what}: the strip and the send ramp together");
        }
    }
}

/// Coordinator obligation 4: the following route's charge covers what its attach and bind
/// retain. Two tracks, `t` (which sends into bus `b`) and `u`, prepared with no live control, so
/// no six-word lane is attached: a mute entry on `t` gives `t-b` its seven-word lane, the same
/// entry on `u` gives no route a lane. The plans differ only in that lane (and the route-activity
/// table its plan builds), and the bytes the first plan retains beyond the second are within the
/// report's `route_control_resources`, which charges one route.
///
/// Test value: red if the seven-word cell, its owner, its binding, its producer or the activity
/// table it makes the plan build is left out of the charge a host admits against its caps.
#[cfg(feature = "test-support")]
#[test]
fn the_following_route_charge_covers_its_attach_and_bind() {
    use follow::*;
    let mut base = model(128, SendTap::PreFader, false, true);
    let (_, source, track) = empty_session();
    let mut source = source;
    source.frames = 1 << 16;
    add_track(&mut base, &source, &track, "u");
    base.routes.push(to_output(
        "u-main",
        RouteSource::Track {
            track_id: sid("u"),
            tap: SendTap::PostPan,
        },
    ));
    let mut following = base.clone();
    following.automation = vec![mute_entry_on("t", 1.0, &[(1_000, 0.0)])];
    let mut elsewhere = base;
    elsewhere.automation = vec![mute_entry_on("u", 1.0, &[(1_000, 0.0)])];
    launch_native_effect_registry().expect("the launch registry builds");
    bench_alloc::assert_installed();
    let retained = |model: &SessionModel| {
        let caps = caps();
        let compiled = compile_host_session(&document(model), &caps).expect("compile");
        // Warm the process statics a first preparation initialises.
        drop(host_core::test_only_prepare_host_runtime_on(
            &compiled,
            &caps,
            Backend::Simd4,
        ));
        let mark = bench_alloc::current_thread_counters();
        let prepared =
            host_core::test_only_prepare_host_runtime_on(&compiled, &caps, Backend::Simd4)
                .unwrap_or_else(|failure| {
                    panic!("{}", String::from_utf8_lossy(failure.as_bytes()))
                });
        let window = bench_alloc::current_thread_delta_since(mark);
        let resources = prepared.report.route_control_resources;
        drop(prepared);
        let bytes = window
            .requested_bytes
            .checked_sub(window.released_bytes)
            .expect("the window frees nothing it did not allocate");
        (bytes, resources)
    };
    let (with, resources) = retained(&following);
    let (without, none) = retained(&elsewhere);
    assert_eq!(none, host_core::RouteControlResources::default());
    assert_eq!(resources.routes, 1);
    let added = with
        .checked_sub(without)
        .filter(|bytes| *bytes > 0)
        .unwrap_or_else(|| panic!("the lane retains more: {with} against {without}"));
    assert!(
        resources.total_bytes >= added,
        "the charge {} covers the {added} bytes the attach and bind retain",
        resources.total_bytes
    );
    eprintln!(
        "following route charge: charged {}, retained {added}",
        resources.total_bytes
    );
}

/// #1654 U2 fold 1, MAJOR-3 on a mute-following send (every bank width and the scalar oracle).
/// The send `t-b` hears `t`'s right lane, whose mute entry holds it muted; `t`'s left lane has its
/// own mute entry too. A rebuild at block 6 removes the right lane's entry: the send stays a
/// following route (the left lane's curve keeps it seven-word), no join record moves its cell, and
/// no cursor of the successor's right column hears a curve. The carry lands the send's column on
/// the successor's static value over the removed curve's own length (the mute length): from the
/// first block boundary after that landing, the send's plane is the successor prepared fresh, bit
/// for bit, and render allocates nothing.
///
/// Test value: red if a removed source mute curve of a send that stays seven-word leaves the send
/// on the removed curve's last value (muted).
#[cfg(feature = "test-support")]
#[test]
fn removing_a_source_mute_curve_of_a_following_send_lands_on_the_fresh_plan() {
    use follow::*;
    use successor::{BLOCKS, QUANTUM, SWAP_BLOCK};
    let pcm = successor::Feed::new("t", 29);
    let lane_entry = |name: &str, lane: session::ParameterChannel, from: f32| {
        let mut entry = mute_entry(from, &[]);
        entry.id = sid(name);
        entry.target.lane = lane;
        entry
    };
    let mut b = model(QUANTUM as u32, SendTap::PreFader, false, true);
    b.automation = vec![lane_entry(
        "mute-left",
        session::ParameterChannel::Left,
        0.0,
    )];
    let mut a = b.clone();
    a.automation.push(lane_entry(
        "mute-right",
        session::ParameterChannel::Right,
        1.0,
    ));
    let quiet = |_: usize,
                 _: &mut host_core::HostLiveControlHandles,
                 _: &mut engine::realtime::PlanRevisionGate| {};
    for backend in follow::backends() {
        let what = format!("{backend:?}");
        let fresh =
            successor::reference_run(&successor::Session::compile(b.clone()), &[&pcm], backend);
        let fresh = plane(&fresh, 1);
        let (swapped, handles) = live_swap(&a, Some((&b, &a)), backend, &pcm, quiet);
        assert_eq!(
            handles
                .expect("the successor's handles")
                .follow_route_controls
                .len(),
            1,
            "{what}: the send stays a following route"
        );
        assert_quiet(&swapped, &what);
        let send = plane(&swapped, 1);
        let at = SWAP_BLOCK * QUANTUM;
        assert!(
            first_difference_from(&send[..at], &fresh[..at], 0).is_some(),
            "{what}: not vacuous, the removed curve is heard before the swap"
        );
        let landed = next_boundary(at + M as usize, QUANTUM);
        assert_bits_equal(
            &send[landed..BLOCKS * QUANTUM],
            &fresh[landed..BLOCKS * QUANTUM],
            &format!("{what}: the fresh plan after the landing"),
        );
    }
}
