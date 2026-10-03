//! Issue #1208: a host meters any boundary of a submix strip and designates any strip the master.
//!
//! * **Gate 1, a bus meter equals a track meter.** Three tracks route into a submix `bus` with
//!   strip `S`; a selected meter at each of `bus`'s seven boundaries reports the snapshot words of
//!   the same meter on one track `ref` with strip `S`, fed the test's own sum of the routed
//!   contributions. The oracle has no submix at all.
//! * **Gate 2, the order rules on strips.** A selection mixing a bus and a track comes back in the
//!   caller's order; an ID that is neither refuses with `builtin.meter.unknown_track`.
//! * **Gate 3, the master designation on strips.** The last submix can be designated; one past it
//!   refuses with `host.observation.master_track`.
//! * **Gate 4, render allocates nothing** with seven bus meters attached.

use core::num::{NonZeroU32, NonZeroUsize};

use bench_support::alloc as bench_alloc;
use builtins::{MeterLaneSnapshot, MeterMetricSet, MeterSnapshot, MeterTap};
use dsp_reference::randomized::{Draw, run_seeds};
use engine::realtime::audit;
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostMeterRequest, HostPrepareCaps,
    HostShapePolicy, PrepareRejection, PreparedHost, SourceSubmission, compile_host_session,
    prepare_host_runtime_with_selected_meters_between_render_calls,
    prepare_host_session_with_live_controls,
};
use session::{
    ChannelMatrix, Console, ConsoleEntry, ConsoleSlot, Effect, EffectIdentity, EffectParam,
    EffectQuality, LinkMode, MatrixOrPan, ParameterChannel, ParameterUnit, Route, RouteDestination,
    RouteSource, SendTap, SessionModel, SidechainDeclaration, StableId, Submix,
    canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const SEEDS: u64 = 16;
const TEST: &str = "a_bus_meter_reports_the_words_of_the_same_meter_on_a_track_fed_its_sum";
const REPLAY: &str = "cargo test -p host-core --test strip_meters -- --exact \
                      a_bus_meter_reports_the_words_of_the_same_meter_on_a_track_fed_its_sum";

/// The seven strip boundaries in chain order: `input`, `post_input`, `insert_send`,
/// `insert_return`, `pre_fader`, `post_fader` and `post_pan`.
const BOUNDARIES: [MeterTap; 7] = [
    MeterTap::Input,
    MeterTap::PostInputBuiltins,
    MeterTap::PostSimd1,
    MeterTap::PostDynamic,
    MeterTap::PostSimd2PreFader,
    MeterTap::PostFader,
    MeterTap::PostMatrix,
];

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

/// Meters every block, with room for every block's window.
fn meter_controls() -> HostLiveControlRequest {
    HostLiveControlRequest {
        meter_period_frames: NonZeroU32::new(QUANTUM as u32),
        meter_queue_depth: NonZeroUsize::new(BLOCKS + 1).expect("meter depth"),
        ..HostLiveControlRequest::default()
    }
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn meter(strip: &str, tap: MeterTap) -> HostMeterRequest {
    HostMeterRequest {
        strip_id: strip.into(),
        tap,
        metrics: MeterMetricSet::ALL,
    }
}

fn to_output(id: &str, source: RouteSource) -> Route {
    Route {
        id: sid(id),
        source,
        destination: RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
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

fn post_pan_of_track(track: &str) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap: SendTap::PostPan,
    }
}

fn post_pan_of_bus(bus: &str) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(bus),
        tap: SendTap::PostPan,
    }
}

fn param(parameter_id: u32, unit: ParameterUnit, value: f32) -> EffectParam {
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
        param(1, ParameterUnit::Linear, 1.0),
        param(3, ParameterUnit::Hz, hz),
        param(4, ParameterUnit::Db, gain_db),
    ]
}

/// No console slots: the spec's strip, on which `post_input` = `insert_send` and
/// `insert_return` = `pre_fader` hold by construction.
fn no_console() -> Console {
    Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    }
}

/// One latency-free EQ slot per console section, so a live entry separates `post_input` from
/// `insert_send` and `insert_return` from `pre_fader`: all seven boundaries then differ.
fn desk_console() -> Console {
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

/// Strip `S` of gate 1 as a unity submix to copy onto a bus or a track: a trim, an EQ insert, a
/// compressor insert with `link_mode: maximum`, a -6 dB fader and a non-identity pan, plus a live
/// EQ in each console section when `console` declares them.
fn strip_s(console: &Console) -> Submix {
    let mut strip = Submix::unity(sid("bus"), console);
    strip.builtins.left.trim_db = 3.0;
    strip.builtins.right.trim_db = -2.0;
    if !console.pre_insert.is_empty() {
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
    }
    let insert = |id: &str, effect: &str, link_mode: LinkMode, params: Vec<EffectParam>| Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(effect),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        params,
        sidechain: SidechainDeclaration::None,
    };
    strip.inserts.effects = vec![
        insert(
            "tone",
            "miso.parametric-eq",
            LinkMode::DualMono,
            bell(500.0, 5.0),
        ),
        insert("glue", "miso.compressor", LinkMode::Maximum, Vec::new()),
    ];
    strip.fader.left_db = -6.0;
    strip.fader.right_db = -6.0;
    strip.matrix_or_pan = MatrixOrPan::Pan {
        left: 0.8,
        right: 0.35,
        smoothing_samples: 0,
    };
    strip
}

/// The fixture emptied of tracks, routes and sources, at `rate`, declaring `console`; returns it
/// with a two-channel source template and a transparent track template (every slot bypassed).
fn empty_session(rate: u32, console: Console) -> (SessionModel, session::Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = (QUANTUM * (BLOCKS + 8)) as u64;
    let mut track = model.tracks.swap_remove(0);
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    model.console = console;
    let unity = Submix::unity(sid("unused"), &model.console);
    track.builtins = unity.builtins;
    track.console = unity.console;
    track.inserts = unity.inserts;
    track.fader = unity.fader;
    track.matrix_or_pan = unity.matrix_or_pan;
    (model, source, track)
}

/// Adds a source and a track reading it, both named `id`, with `track`'s strip.
fn add_track(model: &mut SessionModel, source: &session::Source, track: &session::Track, id: &str) {
    let mut source = source.clone();
    source.id = sid(id);
    model.sources.push(source);
    let mut track = track.clone();
    track.id = sid(id);
    track.source_id = sid(id);
    model.tracks.push(track);
}

/// A nonzero coefficient in `[-1, -0.1] U [0.1, 1]`.
fn coefficient(draw: &mut Draw) -> f32 {
    let magnitude = 0.1 + 0.9 * draw.unit();
    if draw.chance(1, 2) {
        -magnitude
    } else {
        magnitude
    }
}

/// Distinct noise per lane, never a zero word.
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

/// The two gate-1 sessions and their feeds.
struct Pair {
    /// Session A: `t0`..`t2` (transparent) routed with drawn gains and matrices into `bus` (strip
    /// `S`), and `bus` at unity to the output.
    bus: String,
    bus_feeds: Vec<(String, [Vec<f32>; 2])>,
    /// Session B: one track `ref` with strip `S`, reading `sum`, at unity to the output.
    reference: String,
    sum: [Vec<f32>; 2],
}

/// Gate 1's sessions. `sum` is the D9 sum of the three routed contributions: each route folds its
/// gain into its matrix once, then `l' = (lr * r) + (ll * l)` with two roundings, added left to
/// right in route-ID order. Route IDs are a drawn permutation, so route-ID order is not track
/// order.
fn pair(draw: &mut Draw, console: &Console) -> Pair {
    let rate = draw.pick(&LAUNCH_RATES);
    let frames = QUANTUM * BLOCKS;
    let (mut a, source, track) = empty_session(rate, console.clone());
    let mut names = vec!["route-a", "route-b", "route-c"];
    let mut contributions: Vec<(Route, [Vec<f32>; 2])> = Vec::new();
    let mut bus_feeds = Vec::new();
    for index in 0..3 {
        let name = format!("t{index}");
        add_track(&mut a, &source, &track, &name);
        let route = Route {
            id: sid(names.remove(draw.below(names.len()))),
            source: post_pan_of_track(&name),
            destination: RouteDestination::SubmixInput {
                submix_id: sid("bus"),
            },
            channel_matrix: ChannelMatrix {
                ll: coefficient(draw),
                lr: coefficient(draw),
                rl: coefficient(draw),
                rr: coefficient(draw),
            },
            gain_db: draw.in_domain(-12.0, 6.0),
            mute: false,
        };
        a.routes.push(route.clone());
        let planes = noise_planes(draw, frames);
        contributions.push((route, planes.clone()));
        bus_feeds.push((name, planes));
    }
    a.submixes = vec![strip_s(console)];
    a.routes.push(to_output("bus-main", post_pan_of_bus("bus")));

    contributions.sort_by(|x, y| x.0.id.cmp(&y.0.id));
    let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for (rank, (route, planes)) in contributions.iter().enumerate() {
        let gain = math::db_to_gain_f32(route.gain_db);
        let m = &route.channel_matrix;
        let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
        for frame in 0..frames {
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

    let (mut b, source, mut track) = empty_session(rate, console.clone());
    let s = strip_s(console);
    track.builtins = s.builtins;
    track.console = s.console;
    track.inserts = s.inserts;
    track.fader = s.fader;
    track.matrix_or_pan = s.matrix_or_pan;
    add_track(&mut b, &source, &track, "ref");
    b.routes
        .push(to_output("ref-main", post_pan_of_track("ref")));

    Pair {
        bus: canonical_session_json(&a).expect("bus session canonicalizes"),
        bus_feeds,
        reference: canonical_session_json(&b).expect("reference session canonicalizes"),
        sum,
    }
}

fn prepare_selected(
    document: &str,
    meters: &[HostMeterRequest],
) -> (PreparedHost, HostLiveControlHandles) {
    let compiled = compile_host_session(document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    prepare_host_runtime_with_selected_meters_between_render_calls(
        &compiled,
        &caps(),
        &meter_controls(),
        meters,
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())))
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

/// A float word with every NaN folded to one value (decision 10).
fn word32(value: f32) -> u64 {
    if value.is_nan() {
        u64::from(f32::NAN.to_bits())
    } else {
        u64::from(value.to_bits())
    }
}

fn word64(value: f64) -> u64 {
    if value.is_nan() {
        f64::NAN.to_bits()
    } else {
        value.to_bits()
    }
}

fn lane_words(lane: &MeterLaneSnapshot) -> [u64; 6] {
    [
        word32(lane.sample_peak),
        word64(lane.rms),
        word64(lane.energy),
        word32(lane.held_peak),
        lane.clipped_samples,
        lane.sanitized_samples,
    ]
}

/// Every word of a snapshot: handle, presence, window, both lanes and the cumulative counters.
fn snapshot_words(snapshot: &MeterSnapshot) -> Vec<u64> {
    let mut words = vec![
        snapshot.handle.0.get(),
        u64::from(snapshot.present_metrics.bits()),
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        u64::from(snapshot.frames),
    ];
    words.extend(lane_words(&snapshot.left));
    words.extend(lane_words(&snapshot.right));
    words.extend([
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_discontinuities,
        snapshot.cumulative_dropped_snapshots,
    ]);
    words
}

/// Prepares `document` with `meters` selected, renders `BLOCKS` quanta, and returns each meter's
/// snapshot words, `[meter][window]`, after checking the handles came back as requested.
fn metered(
    document: &str,
    feeds: &[(&str, &[Vec<f32>; 2])],
    meters: &[HostMeterRequest],
) -> Vec<Vec<Vec<u64>>> {
    let (mut prepared, mut handles) = prepare_selected(document, meters);
    let bound: Vec<(&str, MeterTap)> = handles
        .meters
        .iter()
        .map(|meter| (meter.track_id.as_ref(), meter.tap))
        .collect();
    let requested: Vec<(&str, MeterTap)> = meters
        .iter()
        .map(|meter| (meter.strip_id.as_ref(), meter.tap))
        .collect();
    assert_eq!(bound, requested, "meters come back in the caller's order");
    let mut words = vec![Vec::new(); meters.len()];
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
        for (index, meter) in handles.meters.iter_mut().enumerate() {
            while let Ok(snapshot) = meter.consumer.try_pop() {
                words[index].push(snapshot_words(&snapshot));
            }
        }
    }
    for (index, windows) in words.iter().enumerate() {
        assert_eq!(windows.len(), BLOCKS, "meter {index}: one window per block");
    }
    words
}

/// Whether two boundaries carry the same signal by construction on strip `S`: with no console
/// slot, `post_input` = `insert_send` and `insert_return` = `pre_fader`.
fn coincide(with_console: bool, x: MeterTap, y: MeterTap) -> bool {
    let pair = |a: MeterTap, b: MeterTap| (x, y) == (a, b) || (x, y) == (b, a);
    !with_console
        && (pair(MeterTap::PostInputBuiltins, MeterTap::PostSimd1)
            || pair(MeterTap::PostDynamic, MeterTap::PostSimd2PreFader))
}

/// Gate 1: on 16 seeds (even: no console slot, as the spec's strip; odd: a live EQ in each
/// console section, so every boundary differs), a meter at each of `bus`'s seven boundaries,
/// selected in a drawn order, reports every snapshot word of the same meter on `ref`. The
/// reference's seven meters are pairwise different except where the strip makes them coincide, so
/// a bus meter mapped to any other stage, or either lane swapped, changes the words.
///
/// Red if a bus meter observes the wrong stage or lane, is refused, or comes back at a different
/// position. No meter had ever observed a bus.
#[test]
fn a_bus_meter_reports_the_words_of_the_same_meter_on_a_track_fed_its_sum() {
    let ran = run_seeds(TEST, REPLAY, SEEDS, |seed| {
        let mut draw = Draw::new(seed);
        let with_console = seed % 2 == 1;
        let console = if with_console {
            desk_console()
        } else {
            no_console()
        };
        let pair = pair(&mut draw, &console);
        let mut order: Vec<MeterTap> = BOUNDARIES.to_vec();
        let mut taps = Vec::new();
        while !order.is_empty() {
            taps.push(order.remove(draw.below(order.len())));
        }
        let bus_meters: Vec<HostMeterRequest> = taps.iter().map(|&tap| meter("bus", tap)).collect();
        let ref_meters: Vec<HostMeterRequest> = taps.iter().map(|&tap| meter("ref", tap)).collect();
        let bus_feeds: Vec<(&str, &[Vec<f32>; 2])> = pair
            .bus_feeds
            .iter()
            .map(|(id, planes)| (id.as_str(), planes))
            .collect();
        let actual = metered(&pair.bus, &bus_feeds, &bus_meters);
        let expected = metered(&pair.reference, &[("ref", &pair.sum)], &ref_meters);
        for (index, tap) in taps.iter().enumerate() {
            for window in 0..BLOCKS {
                assert_eq!(
                    actual[index][window], expected[index][window],
                    "seed {seed}: bus {tap:?} window {window} differs from the reference"
                );
            }
        }
        for x in 0..taps.len() {
            for y in 0..taps.len() {
                if x == y || coincide(with_console, taps[x], taps[y]) {
                    continue;
                }
                assert_ne!(
                    expected[x], expected[y],
                    "seed {seed}: boundaries {:?} and {:?} report the same words",
                    taps[x], taps[y]
                );
            }
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, SEEDS);
    }
}

/// Gate 2: `[bus post_pan, t0 pre_fader]` binds and comes back in that order, handles `1, 2`; a
/// selection naming `nowhere` (neither a track nor a submix) refuses with
/// `builtin.meter.unknown_track` alone and binds nothing.
///
/// Red if the canonical order behind rule 1 is track-only (every bus meter refused with
/// `host.meter.order`), or if selected meters are re-sorted into canonical order.
#[test]
fn selected_strip_meters_keep_the_callers_order_and_an_unknown_strip_is_refused() {
    let pair = pair(&mut Draw::new(1), &no_console());
    let (_, handles) = prepare_selected(
        &pair.bus,
        &[
            meter("bus", MeterTap::PostMatrix),
            meter("t0", MeterTap::PostSimd2PreFader),
        ],
    );
    let bound: Vec<(u64, &str, MeterTap)> = handles
        .meters
        .iter()
        .map(|meter| (meter.handle.0.get(), meter.track_id.as_ref(), meter.tap))
        .collect();
    assert_eq!(
        bound,
        [
            (1, "bus", MeterTap::PostMatrix),
            (2, "t0", MeterTap::PostSimd2PreFader),
        ]
    );

    let compiled = compile_host_session(&pair.bus, &caps()).expect("compile");
    let Err(failure) = prepare_host_runtime_with_selected_meters_between_render_calls(
        &compiled,
        &caps(),
        &meter_controls(),
        &[
            meter("bus", MeterTap::PostMatrix),
            meter("nowhere", MeterTap::PostSimd2PreFader),
        ],
    ) else {
        panic!("a meter on an unknown strip must be refused");
    };
    assert_eq!(failure.kind(), PrepareRejection::Builtin);
    let text = String::from_utf8_lossy(failure.as_bytes()).into_owned();
    let codes: Vec<&str> = text
        .lines()
        .map(|line| line.split('\t').next().unwrap_or_default())
        .collect();
    assert_eq!(codes, ["builtin.meter.unknown_track"], "{text}");
}

/// Gate 3's session: gate 1's bus session plus a second, unrouted unity submix `aux`, so `T = 3`
/// and `S = 2`, and a one-past-the-tracks bound is distinguishable from the strip bound.
fn two_bus_session() -> String {
    let pair = pair(&mut Draw::new(2), &no_console());
    let mut model = parse_session_json(&pair.bus).expect("bus session parses");
    model
        .submixes
        .push(Submix::unity(sid("aux"), &model.console));
    canonical_session_json(&model).expect("two-bus session canonicalizes")
}

/// Gate 3: with live controls and observation taps, `master_track = T + S - 1` (the last submix)
/// prepares and is echoed; `T + S` refuses with `host.observation.master_track`.
///
/// Red if the designation is still validated against the tracks only (refusing every bus), or
/// against anything larger than the strip list.
#[test]
fn the_last_submix_can_be_designated_master_and_one_past_it_is_refused() {
    let document = two_bus_session();
    let request = |master: u32| HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
        meter_period_frames: NonZeroU32::new(QUANTUM as u32),
        meter_queue_depth: NonZeroUsize::new(4).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 1,
        master_track: Some(master),
    };
    let (_, _, handles) = prepare_host_session_with_live_controls(&document, &caps(), &request(4))
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!((handles.track_count, handles.strips.len()), (3, 5));
    assert_eq!(&*handles.strips[4], "bus");
    assert_eq!(handles.master_track, Some(4));

    let Err(failure) = prepare_host_session_with_live_controls(&document, &caps(), &request(5))
    else {
        panic!("a master one past the strips must be refused");
    };
    let text = String::from_utf8_lossy(failure.as_bytes()).into_owned();
    let codes: Vec<&str> = text
        .lines()
        .map(|line| line.split('\t').next().unwrap_or_default())
        .collect();
    assert_eq!(codes, ["host.observation.master_track"], "{text}");
}

/// Gate 4: with a meter at each of `bus`'s seven boundaries, every render after block 0 allocates
/// and frees nothing: the render audit and `bench_support::alloc`'s thread-scoped counters both
/// read exact zero around each call.
///
/// Red if a bus meter's observer allocates per block, which no track meter test exercises on a
/// reduction-fed `Input` stage.
#[test]
fn seven_bus_meters_render_without_allocating() {
    let pair = pair(&mut Draw::new(3), &desk_console());
    let meters: Vec<HostMeterRequest> = BOUNDARIES.iter().map(|&tap| meter("bus", tap)).collect();
    let (prepared, handles) = prepare_selected(&pair.bus, &meters);
    assert_eq!(handles.meters.len(), BOUNDARIES.len());
    let rate = prepared.report.sample_rate_hz;
    let (mut session, mut sources, _) = prepared
        .start_render_session()
        .unwrap_or_else(|_| panic!("render session starts"));
    audit::warm_up();
    bench_alloc::assert_installed();
    for block in 0..BLOCKS {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        for (id, planes) in &pair.bus_feeds {
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
    drop(handles);
}
