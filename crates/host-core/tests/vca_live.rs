//! Issue #1244: host-core composes live VCA moves exactly as preparation does.
//!
//! * **Gate 1** `a_live_recompute_equals_preparation`: random forests and edit sequences against
//!   `SessionModel::effective_strip_faders` of a model the test edits the same way.
//! * **Gate 2** `a_composition_never_owes_a_redundant_record` and
//!   `an_unchanged_effective_value_owes_no_record_and_the_delta_shape_follows_the_lanes`.
//! * **Gate 3** `a_clamped_member_returns_to_its_own_value`.
//! * **Gate 4** `rollback_restores_every_mirror_and_commit_keeps_them`.
//! * **Gate 5** `a_vca_mute_wins_over_solo_and_reaches_the_following_sends`.
//! * **Gate 6** `the_command_path_allocates_nothing_and_the_retained_bytes_are_measured`.
//! * **The render differential** `live_vca_moves_render_as_a_fresh_plan`: live moves staged through
//!   the fader queues, the strip-mute owner and the route mirror render, block for block, the bits
//!   of a plan freshly prepared from the session edited the same way.
//!
//! Every session is built from the observation fixture emptied to transparent strips (no console
//! slot, no insert, an identity input section), so the output is the faders, mutes and sends alone.

use core::num::NonZeroUsize;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins::BuiltinLaneSelector;
use builtins_compiler::TrackFaderRecord;
use dsp_reference::randomized::{Draw, first_difference, replaying, run_seeds};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy,
    LiveControlSoloState, LiveRouteMuteFollow, LiveRouteState, LiveVcaState, PreparedHost,
    SourceSubmission, StripMuteSeed, compile_host_session, prepare_host_runtime_with_live_controls,
};
use session::{
    ChannelMatrix, Console, DualMonoFader, Route, RouteDestination, RouteSource, SendTap,
    SessionModel, Source, StableId, Submix, Vca, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const DEPTH: usize = 8;
/// What every host output buffer holds before a render, so an unwritten sample shows.
const HOST_SENTINEL: f32 = 7.0;
const LANES: [BuiltinLaneSelector; 3] = [
    BuiltinLaneSelector::Left,
    BuiltinLaneSelector::Right,
    BuiltinLaneSelector::Both,
];

type Planes = [Vec<f32>; 2];

/// 0 dB on both lanes, open.
const UNITY: DualMonoFader = DualMonoFader {
    left_db: 0.0,
    right_db: 0.0,
    left_mute: false,
    right_mute: false,
};

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

// ---- Sessions -----------------------------------------------------------------------------------

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// A unity, open route.
fn route(id: &str, source: RouteSource, destination: RouteDestination, follows: bool) -> Route {
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
        follows_mute: follows,
    }
}

fn track_tap(track: &str, tap: SendTap) -> RouteSource {
    RouteSource::Track {
        track_id: sid(track),
        tap,
    }
}

fn bus_tap(bus: &str, tap: SendTap) -> RouteSource {
    RouteSource::Submix {
        submix_id: sid(bus),
        tap,
    }
}

fn into_bus(bus: &str) -> RouteDestination {
    RouteDestination::SubmixInput {
        submix_id: sid(bus),
    }
}

fn to_output(id: &str, source: RouteSource) -> Route {
    route(
        id,
        source,
        RouteDestination::OutputInput {
            output_id: sid("main-out"),
        },
        false,
    )
}

/// The fixture emptied of tracks, routes, sources, automation and console slots, at 48 kHz, with a
/// transparent track template and its source.
fn empty_session(frames: usize) -> (SessionModel, Source, session::Track) {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = 48_000;
    model.quantum_frames = QUANTUM as u32;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = frames as u64;
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

/// A strip's fader, by its `strips()` index.
fn strip_fader(model: &mut SessionModel, strip: usize) -> &mut DualMonoFader {
    let tracks = model.tracks.len();
    if strip < tracks {
        &mut model.tracks[strip].fader
    } else {
        &mut model.submixes[strip - tracks].fader
    }
}

/// A dB value from the whole fader domain, near unity, or one of the edges and the words whose sum
/// order matters (`24 + 1e-30 - 24` is not `24 - 24 + 1e-30` in `f64`).
fn db(draw: &mut Draw) -> f32 {
    match draw.below(3) {
        0 => draw.in_domain(-144.0, 24.0),
        1 => draw.in_domain(-12.0, 12.0),
        _ => draw.pick(&[
            -144.0, 24.0, -24.0, 0.0, -0.0, 1.0e-30, -1.0e-30, 12.0, -100.0,
        ]),
    }
}

/// A gentler value for a rendered session: mostly near unity, sometimes at an edge, so the output
/// stays audible while sums still clamp.
fn audible_db(draw: &mut Draw) -> f32 {
    match draw.below(4) {
        0 => draw.pick(&[-144.0, 24.0, 20.0, -0.0]),
        _ => draw.in_domain(-18.0, 9.0),
    }
}

/// Sets the covered lanes of a fader's dB.
fn set_db(fader: &mut DualMonoFader, lanes: BuiltinLaneSelector, db: f32) {
    if lanes != BuiltinLaneSelector::Right {
        fader.left_db = db;
    }
    if lanes != BuiltinLaneSelector::Left {
        fader.right_db = db;
    }
}

/// Sets the covered lanes of a fader's mute.
fn set_mute(fader: &mut DualMonoFader, lanes: BuiltinLaneSelector, muted: bool) {
    if lanes != BuiltinLaneSelector::Right {
        fader.left_mute = muted;
    }
    if lanes != BuiltinLaneSelector::Left {
        fader.right_mute = muted;
    }
}

/// A random forest of up to `most` VCAs over `strips`, at most four levels deep: a VCA's members
/// are strips and VCAs of strictly deeper levels, up to 16, so VCAs overlap and diamonds occur.
/// Declared in an order unrelated to ID order. `value` draws each lane's offset.
fn forest(
    draw: &mut Draw,
    strips: &[String],
    most: usize,
    value: fn(&mut Draw) -> f32,
) -> Vec<Vca> {
    let count = 1 + draw.below(most);
    let levels: Vec<usize> = (0..count).map(|_| draw.below(4)).collect();
    let names: Vec<String> = (0..count)
        .map(|index| format!("vca-{:02}", (index * 37) % 101))
        .collect();
    (0..count)
        .map(|index| {
            let mut candidates: Vec<&str> = strips.iter().map(String::as_str).collect();
            candidates.extend(
                (0..count)
                    .filter(|other| levels[*other] > levels[index])
                    .map(|other| names[other].as_str()),
            );
            for slot in (1..candidates.len()).rev() {
                candidates.swap(slot, draw.below(slot + 1));
            }
            let take = draw.below(candidates.len().min(16) + 1);
            Vca {
                id: sid(&names[index]),
                fader: DualMonoFader {
                    left_db: value(draw),
                    right_db: value(draw),
                    left_mute: draw.chance(1, 5),
                    right_mute: draw.chance(1, 5),
                },
                members: candidates[..take]
                    .iter()
                    .map(|member| sid(member))
                    .collect(),
            }
        })
        .collect()
}

/// Tracks `t0..` and submixes `s0..`, each reaching the output from its `post_pan` tap. Each track
/// may send into a submix from a random tap, following its mute or not, and a submix may send into
/// a later one. Every strip's own fader and lane mutes are drawn, then a VCA forest over them.
fn generate(
    draw: &mut Draw,
    tracks: usize,
    submixes: usize,
    vcas: usize,
    value: fn(&mut Draw) -> f32,
    frames: usize,
) -> SessionModel {
    let (mut model, source, track) = empty_session(frames);
    let mut strips = Vec::new();
    for index in 0..tracks {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        model.routes.push(to_output(
            &format!("{id}-main"),
            track_tap(&id, SendTap::PostPan),
        ));
        strips.push(id);
    }
    for index in 0..submixes {
        let id = format!("s{index}");
        model.submixes.push(Submix::unity(sid(&id), &model.console));
        model.routes.push(to_output(
            &format!("{id}-main"),
            bus_tap(&id, SendTap::PostPan),
        ));
        strips.push(id);
    }
    for index in 0..tracks {
        for send in 0..draw.below(3) {
            let bus = format!("s{}", draw.below(submixes));
            let tap = draw.pick(&[SendTap::PreFader, SendTap::PostFader, SendTap::PostPan]);
            let id = format!("t{index}-{bus}-{send}");
            if model.routes.iter().all(|route| route.id.as_str() != id) {
                model.routes.push(route(
                    &id,
                    track_tap(&format!("t{index}"), tap),
                    into_bus(&bus),
                    draw.chance(2, 3),
                ));
            }
        }
    }
    for from in 0..submixes {
        for to in from + 1..submixes {
            if draw.chance(1, 2) {
                model.routes.push(route(
                    &format!("s{from}-s{to}"),
                    bus_tap(&format!("s{from}"), SendTap::PostFader),
                    into_bus(&format!("s{to}")),
                    draw.chance(2, 3),
                ));
            }
        }
    }
    for strip in 0..tracks + submixes {
        let fader = strip_fader(&mut model, strip);
        fader.left_db = value(draw);
        fader.right_db = value(draw);
        fader.left_mute = draw.chance(1, 6);
        fader.right_mute = draw.chance(1, 6);
    }
    model.vcas = forest(draw, &strips, vcas, value);
    model
}

fn document(model: &SessionModel) -> String {
    canonical_session_json(model).expect("generated session canonicalizes")
}

/// The normalized model a host prepares from: strips, routes and VCAs in canonical ID order.
fn normalized(model: &SessionModel) -> SessionModel {
    compile_host_session(&document(model), &caps())
        .unwrap_or_else(|failure| {
            panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
        .normalized_model()
        .clone()
}

/// The strip-mute owner seeded from a normalized model, as a host seeds it.
fn solo_state(model: &SessionModel) -> LiveControlSoloState {
    let tracks = model.tracks.len();
    let seeds: Vec<StripMuteSeed> = model
        .strips()
        .zip(model.effective_strip_faders())
        .enumerate()
        .map(|(index, (strip, effective))| StripMuteSeed {
            mutes: [strip.fader.left_mute, strip.fader.right_mute],
            solo_safe: index >= tracks,
            vca_mute: effective.vca_mute,
        })
        .collect();
    LiveControlSoloState::try_new(&seeds).expect("solo state")
}

/// Every strip's every lane: `(effective dB bits, VCA mute)`.
fn composition(state: &LiveVcaState, strips: usize) -> Vec<([u32; 2], [bool; 2])> {
    (0..strips)
        .map(|strip| {
            (
                [0, 1].map(|lane| state.effective_db(strip, lane).to_bits()),
                state.vca_mute(strip),
            )
        })
        .collect()
}

/// `effective_strip_faders()` as [`composition`] reads the state.
fn prepared(model: &SessionModel) -> Vec<([u32; 2], [bool; 2])> {
    model
        .effective_strip_faders()
        .into_iter()
        .map(|fader| (fader.db.map(f32::to_bits), fader.vca_mute))
        .collect()
}

/// One random edit, applied to the state and to `model` alike. Returns `(kind, accepted, index)`:
/// kind 0 is a VCA dB move, 1 a VCA mute move (`index` is the VCA), 2 a member's own fader move
/// (`index` is the strip).
fn random_edit(
    draw: &mut Draw,
    state: &mut LiveVcaState,
    model: &mut SessionModel,
    value: fn(&mut Draw) -> f32,
) -> (usize, bool, usize) {
    let strips = model.tracks.len() + model.submixes.len();
    let lanes = draw.pick(&LANES);
    match draw.below(3) {
        0 => {
            let vca = draw.below(model.vcas.len());
            let db = value(draw);
            assert!(state.set_vca_db(vca, lanes, db));
            set_db(&mut model.vcas[vca].fader, lanes, db);
            (0, true, vca)
        }
        1 => {
            let vca = draw.below(model.vcas.len());
            let muted = draw.chance(1, 2);
            assert!(state.set_vca_mute(vca, lanes, muted));
            set_mute(&mut model.vcas[vca].fader, lanes, muted);
            (1, true, vca)
        }
        _ => {
            let strip = draw.below(strips);
            let db = value(draw);
            let accepted = state.set_member_db(strip, lanes, db);
            assert_eq!(accepted, state.reaches(strip), "strip {strip}");
            if accepted {
                set_db(strip_fader(model, strip), lanes, db);
            }
            (2, accepted, strip)
        }
    }
}

// ---- Gate 1 -------------------------------------------------------------------------------------

/// How many membership paths lead from VCA `vca` down to the member `id` (a diamond has two).
fn paths(vcas: &[Vca], vca: usize, id: &str) -> usize {
    vcas[vca]
        .members
        .iter()
        .map(|member| {
            if member.as_str() == id {
                1
            } else {
                vcas.iter()
                    .position(|nested| nested.id == *member)
                    .map_or(0, |nested| paths(vcas, nested, id))
            }
        })
        .sum()
}

const GATE_1: &str = "a_live_recompute_equals_preparation";
const GATE_1_REPLAY: &str =
    "cargo test -p host-core --test vca_live -- --exact a_live_recompute_equals_preparation";

/// Gate 1. Random forests (up to four deep, up to 16 members, overlapping, diamonds, tracks and
/// submixes as members) and up to 64 random edits each, values across the whole domain: after
/// every edit every strip's `effective_db` (bits) and `vca_mute` equal `effective_strip_faders()`
/// of a model edited the same way, and `reached_by` is the inverse of `vca_reach()`.
///
/// Red if live composition differs from preparation in order, reach, clamp or lane, if a member's
/// own value is overwritten by an effective one, or if the inverse table misses a nested member.
#[test]
fn a_live_recompute_equals_preparation() {
    // Multi-VCA strips, clamped lanes, reached submixes, refused unreached member edits, diamonds.
    let mut reached = [0_usize; 5];
    let ran = run_seeds(GATE_1, GATE_1_REPLAY, 32, |seed| {
        let mut draw = Draw::new(seed);
        let tracks = 2 + draw.below(7);
        let submixes = 1 + draw.below(4);
        let mut model = normalized(&generate(&mut draw, tracks, submixes, 10, db, 1_024));
        let mut state = LiveVcaState::try_new(&model).expect("state");
        let strips = tracks + submixes;
        assert_eq!(state.vca_count(), model.vcas.len());

        let reach = model.vca_reach();
        for vca in 0..model.vcas.len() {
            let expected: Vec<usize> = (0..strips)
                .filter(|strip| reach[*strip].contains(&vca))
                .collect();
            assert_eq!(state.reached_by(vca), expected, "seed {seed}, vca {vca}");
        }
        assert_eq!(
            state.reached_strip_count(),
            reach.iter().filter(|list| !list.is_empty()).count()
        );
        let ids: Vec<String> = model
            .strips()
            .map(|strip| strip.id.as_str().to_owned())
            .collect();
        for (strip, list) in reach.iter().enumerate() {
            assert_eq!(state.reaches(strip), !list.is_empty());
            reached[4] += list
                .iter()
                .filter(|vca| paths(&model.vcas, **vca, &ids[strip]) > 1)
                .count();
            reached[0] += usize::from(list.len() > 1);
            reached[2] += usize::from(strip >= tracks && !list.is_empty());
        }
        assert_eq!(composition(&state, strips), prepared(&model), "seed {seed}");

        for edit in 0..1 + draw.below(64) {
            let (kind, accepted, _) = random_edit(&mut draw, &mut state, &mut model, db);
            reached[3] += usize::from(kind == 2 && !accepted);
            let expected = prepared(&model);
            assert_eq!(
                composition(&state, strips),
                expected,
                "seed {seed}, edit {edit} (kind {kind})"
            );
            reached[1] += expected
                .iter()
                .zip(&reach)
                .filter(|((bits, _), list)| {
                    !list.is_empty()
                        && bits
                            .iter()
                            .any(|bits| [-144.0_f32, 24.0].contains(&f32::from_bits(*bits)))
                })
                .count();
            if draw.chance(1, 2) {
                state.commit();
            }
        }
    });
    if !replaying() {
        assert!(ran >= 32, "{ran} seeds");
        assert!(reached.iter().all(|count| *count > 0), "{reached:?}");
    }
}

// ---- Gate 2 -------------------------------------------------------------------------------------

/// The records `told` owes against `now`, in [`LiveVcaState::fader_delta`]'s shape.
fn expected_delta(told: [f32; 2], now: [f32; 2]) -> host_core::LiveVcaFaderDelta {
    match (now[0] != told[0], now[1] != told[1]) {
        (false, false) => [None, None],
        (true, false) => [Some((BuiltinLaneSelector::Left, now[0])), None],
        (false, true) => [Some((BuiltinLaneSelector::Right, now[1])), None],
        (true, true) if now[0] == now[1] => [Some((BuiltinLaneSelector::Both, now[0])), None],
        (true, true) => [
            Some((BuiltinLaneSelector::Left, now[0])),
            Some((BuiltinLaneSelector::Right, now[1])),
        ],
    }
}

const GATE_2: &str = "a_composition_never_owes_a_redundant_record";
const GATE_2_REPLAY: &str = "cargo test -p host-core --test vca_live -- --exact a_composition_never_owes_a_redundant_record";

/// Gate 2, randomized. The state starts owing nothing (the emitted mirror is what preparation
/// baked). After every edit each strip's `fader_delta` is exactly the lanes whose prepared
/// effective value moved since the test last staged it -- one `Both` when both moved to one value
/// -- and after `record_emitted_db` of every entry nothing is owed anywhere.
///
/// Red if composition re-emits an unchanged target (a settled lane's bits move), misses a moved
/// one, or seeds the mirror from anything but the prepared values.
#[test]
fn a_composition_never_owes_a_redundant_record() {
    let mut shapes = [0_usize; 4];
    let ran = run_seeds(GATE_2, GATE_2_REPLAY, 16, |seed| {
        let mut draw = Draw::new(seed);
        let tracks = 2 + draw.below(5);
        let submixes = 1 + draw.below(3);
        let mut model = normalized(&generate(&mut draw, tracks, submixes, 8, db, 1_024));
        let mut state = LiveVcaState::try_new(&model).expect("state");
        let strips = tracks + submixes;
        let mut told: Vec<[f32; 2]> = model
            .effective_strip_faders()
            .iter()
            .map(|fader| fader.db)
            .collect();
        for strip in 0..strips {
            assert_eq!(
                state.fader_delta(strip),
                [None, None],
                "seed {seed}: seeded"
            );
        }
        for edit in 0..48 {
            random_edit(&mut draw, &mut state, &mut model, db);
            let now = model.effective_strip_faders();
            for strip in 0..strips {
                let delta = state.fader_delta(strip);
                assert_eq!(
                    delta,
                    expected_delta(told[strip], now[strip].db),
                    "seed {seed}, edit {edit}, strip {strip}"
                );
                if !state.reaches(strip) {
                    assert_eq!(delta, [None, None]);
                }
                match delta {
                    [None, None] => shapes[0] += 1,
                    [Some((BuiltinLaneSelector::Both, _)), None] => shapes[1] += 1,
                    [Some(_), None] => shapes[2] += 1,
                    _ => shapes[3] += 1,
                }
                for (lanes, db) in delta.into_iter().flatten() {
                    state.record_emitted_db(strip, lanes, db);
                }
                told[strip] = now[strip].db;
            }
            for strip in 0..strips {
                assert_eq!(state.fader_delta(strip), [None, None], "seed {seed}");
            }
            state.commit();
        }
    });
    if !replaying() {
        assert!(ran >= 16, "{ran} seeds");
        assert!(shapes.iter().all(|count| *count > 0), "{shapes:?}");
    }
}

/// Tracks `t0`..`t3` and submix `s0`; VCAs `clamp` (members `t0`, `t1`, own faders -100 dB, the
/// VCA at -60 dB, so both clamp at -144), `pair` (`t2`, own `[0, 0]`), `split` (`t3`, own
/// `[-3, -9]`) and `void` (no member).
fn shape_session() -> SessionModel {
    let (mut model, source, track) = empty_session(1_024);
    for (index, own) in [[-100.0, -100.0], [-100.0, -100.0], [0.0, 0.0], [-3.0, -9.0]]
        .into_iter()
        .enumerate()
    {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        model.tracks[index].fader.left_db = own[0];
        model.tracks[index].fader.right_db = own[1];
        model.routes.push(to_output(
            &format!("{id}-main"),
            track_tap(&id, SendTap::PostPan),
        ));
    }
    model
        .submixes
        .push(Submix::unity(sid("s0"), &model.console));
    model
        .routes
        .push(to_output("s0-main", bus_tap("s0", SendTap::PostPan)));
    let vca = |id: &str, db: f32, members: &[&str]| Vca {
        id: sid(id),
        fader: DualMonoFader {
            left_db: db,
            right_db: db,
            left_mute: false,
            right_mute: false,
        },
        members: members.iter().map(|member| sid(member)).collect(),
    };
    model.vcas = vec![
        vca("clamp", -60.0, &["t0", "t1"]),
        vca("pair", 0.0, &["t2"]),
        vca("split", 0.0, &["t3"]),
        vca("void", 0.0, &[]),
    ];
    normalized(&model)
}

/// Every strip's delta, in strip order.
fn deltas(state: &LiveVcaState) -> Vec<host_core::LiveVcaFaderDelta> {
    (0..5).map(|strip| state.fader_delta(strip)).collect()
}

/// Gate 2, the named cases: a move that keeps every member clamped at -144 dB, and a move of a VCA
/// with no member, owe nothing; both lanes to one value is one `Both`, one lane is one `Left` or
/// `Right`, and both lanes to two values is a `Left` and a `Right`.
///
/// Red if composition re-emits an unchanged (clamped) target, or a delta's lane selector or count
/// disagrees with the lanes that moved.
#[test]
fn an_unchanged_effective_value_owes_no_record_and_the_delta_shape_follows_the_lanes() {
    let model = shape_session();
    let index = |id: &str| {
        model
            .vcas
            .iter()
            .position(|vca| vca.id.as_str() == id)
            .expect("vca")
    };
    let mut state = LiveVcaState::try_new(&model).expect("state");
    let quiet = vec![[None, None]; 5];
    assert_eq!(deltas(&state), quiet);
    assert_eq!(state.effective_db(0, 0), -144.0);

    // Clamped before (-160) and after (-190): nothing to tell.
    assert!(state.set_vca_db(index("clamp"), BuiltinLaneSelector::Both, -90.0));
    assert_eq!(deltas(&state), quiet);
    // A VCA with no member reaches nothing.
    assert!(state.set_vca_db(index("void"), BuiltinLaneSelector::Both, 24.0));
    assert!(state.set_vca_mute(index("void"), BuiltinLaneSelector::Both, true));
    assert!(state.reached_by(index("void")).is_empty());
    assert_eq!(deltas(&state), quiet);

    // `pair` to -6 on both lanes: `t2`'s lanes both move to -6, one `Both`.
    assert!(state.set_vca_db(index("pair"), BuiltinLaneSelector::Both, -6.0));
    assert_eq!(
        state.fader_delta(2),
        [Some((BuiltinLaneSelector::Both, -6.0)), None]
    );
    state.record_emitted_db(2, BuiltinLaneSelector::Both, -6.0);
    // One lane: `Right` alone.
    assert!(state.set_vca_db(index("pair"), BuiltinLaneSelector::Right, -7.0));
    assert_eq!(
        state.fader_delta(2),
        [Some((BuiltinLaneSelector::Right, -7.0)), None]
    );
    state.record_emitted_db(2, BuiltinLaneSelector::Right, -7.0);
    // `split` on both lanes moves `t3`'s lanes to two values: a `Left` and a `Right`.
    assert!(state.set_vca_db(index("split"), BuiltinLaneSelector::Both, 1.0));
    assert_eq!(
        state.fader_delta(3),
        [
            Some((BuiltinLaneSelector::Left, -2.0)),
            Some((BuiltinLaneSelector::Right, -8.0)),
        ]
    );
    state.record_emitted_db(3, BuiltinLaneSelector::Left, -2.0);
    state.record_emitted_db(3, BuiltinLaneSelector::Right, -8.0);
    // A member's own move on one lane: `Left` alone.
    assert!(state.set_member_db(3, BuiltinLaneSelector::Right, -9.0));
    assert!(state.set_member_db(3, BuiltinLaneSelector::Left, -4.0));
    assert_eq!(
        state.fader_delta(3),
        [Some((BuiltinLaneSelector::Left, -3.0)), None]
    );
    // The submix no VCA reaches: its own move is refused and owes nothing here.
    assert!(!state.set_member_db(4, BuiltinLaneSelector::Both, -6.0));
    assert_eq!(state.fader_delta(4), [None, None]);
}

// ---- Gate 3 -------------------------------------------------------------------------------------

/// Gate 3. A member at +20 dB in a VCA moved to +24 dB is at +24 dB effective; moving the VCA back
/// to 0 dB returns exactly +20 dB, with each staged record recorded as emitted on the way.
///
/// Red if the state stores the clamped effective value as the member's own.
#[test]
fn a_clamped_member_returns_to_its_own_value() {
    let (mut model, source, track) = empty_session(1_024);
    add_track(&mut model, &source, &track, "t0");
    model.tracks[0].fader.left_db = 20.0;
    model.tracks[0].fader.right_db = 20.0;
    model
        .routes
        .push(to_output("t0-main", track_tap("t0", SendTap::PostPan)));
    model.vcas = vec![Vca {
        id: sid("lift"),
        fader: UNITY,
        members: vec![sid("t0")],
    }];
    let model = normalized(&model);
    let mut state = LiveVcaState::try_new(&model).expect("state");
    for (offset, effective) in [(24.0, 24.0_f32), (0.0, 20.0)] {
        assert!(state.set_vca_db(0, BuiltinLaneSelector::Both, offset));
        for lane in 0..2 {
            assert_eq!(state.effective_db(0, lane).to_bits(), effective.to_bits());
        }
        assert_eq!(
            state.fader_delta(0),
            [Some((BuiltinLaneSelector::Both, effective)), None]
        );
        state.record_emitted_db(0, BuiltinLaneSelector::Both, effective);
        state.commit();
    }
}

// ---- Gate 4 -------------------------------------------------------------------------------------

/// Everything a host can read from the two states: per strip, the effective dB bits, the VCA
/// mute, the owed fader records, and the strip-mute owner's VCA mute and owed mute records.
type Snapshot = Vec<(
    [u32; 2],
    [bool; 2],
    host_core::LiveVcaFaderDelta,
    [bool; 2],
    host_core::LiveControlMuteDelta,
)>;

fn snapshot(state: &LiveVcaState, solo: &LiveControlSoloState, strips: usize) -> Snapshot {
    (0..strips)
        .map(|strip| {
            (
                [0, 1].map(|lane| state.effective_db(strip, lane).to_bits()),
                state.vca_mute(strip),
                state.fader_delta(strip),
                [0, 1].map(|lane| solo.vca_mute(strip, lane)),
                solo.strip_delta(strip),
            )
        })
        .collect()
}

/// Edits every mirror of both states: a VCA value and mute, a member value, an emitted value and
/// the strip-mute owner's VCA term, on lanes no clamp hides.
fn edit_everything(state: &mut LiveVcaState, solo: &mut LiveControlSoloState, offset: f32) {
    assert!(state.set_vca_db(1, BuiltinLaneSelector::Both, offset));
    assert!(state.set_vca_mute(1, BuiltinLaneSelector::Left, true));
    assert!(state.set_member_db(3, BuiltinLaneSelector::Right, offset - 2.0));
    state.record_emitted_db(2, BuiltinLaneSelector::Both, offset + 1.0);
    for &strip in state.reached_by(1) {
        assert!(solo.set_vca_mute(strip, state.vca_mute(strip)));
    }
}

/// Gate 4. After a mix of edits, `rollback` restores every VCA value, member value, emitted value
/// and the strip-mute owner's VCA mute; `commit` keeps them; and a second transaction shadows
/// afresh, so its rollback returns to the committed state, not the first one.
///
/// Red if any mirror is left changed by a refused submission.
#[test]
fn rollback_restores_every_mirror_and_commit_keeps_them() {
    let model = shape_session();
    let mut state = LiveVcaState::try_new(&model).expect("state");
    let mut solo = solo_state(&model);
    let original = snapshot(&state, &solo, 5);

    edit_everything(&mut state, &mut solo, -4.0);
    assert!(state.transaction_open() && solo.transaction_open());
    let edited = snapshot(&state, &solo, 5);
    assert_ne!(edited, original);
    state.rollback();
    solo.rollback();
    assert!(!state.transaction_open() && !solo.transaction_open());
    assert_eq!(snapshot(&state, &solo, 5), original);

    edit_everything(&mut state, &mut solo, -4.0);
    state.commit();
    solo.commit();
    assert_eq!(snapshot(&state, &solo, 5), edited);

    edit_everything(&mut state, &mut solo, 3.0);
    assert!(state.set_vca_mute(1, BuiltinLaneSelector::Left, false));
    for &strip in state.reached_by(1) {
        assert!(solo.set_vca_mute(strip, state.vca_mute(strip)));
    }
    assert_ne!(snapshot(&state, &solo, 5), edited);
    state.rollback();
    solo.rollback();
    assert_eq!(snapshot(&state, &solo, 5), edited);
}

// ---- Gate 5 -------------------------------------------------------------------------------------

/// Tracks `a`, `b` and `c` and submixes `bx` and `by`. `a` and `b` send `pre_fader` into `bx`
/// following their mutes, `c` sends into `bx` without following, and `bx` sends into `by`
/// following its mute. VCA `va` holds `a`, `vb` holds `b` and `vx` holds `bx`.
fn follow_session() -> SessionModel {
    let (mut model, source, track) = empty_session(1_024);
    for id in ["a", "b", "c"] {
        add_track(&mut model, &source, &track, id);
        model.routes.push(to_output(
            &format!("{id}-main"),
            track_tap(id, SendTap::PostPan),
        ));
    }
    for id in ["bx", "by"] {
        model.submixes.push(Submix::unity(sid(id), &model.console));
        model.routes.push(to_output(
            &format!("{id}-main"),
            bus_tap(id, SendTap::PostPan),
        ));
    }
    for (id, follows) in [("a", true), ("b", true), ("c", false)] {
        model.routes.push(route(
            &format!("{id}-bx"),
            track_tap(id, SendTap::PreFader),
            into_bus("bx"),
            follows,
        ));
    }
    model.routes.push(route(
        "bx-by",
        bus_tap("bx", SendTap::PostFader),
        into_bus("by"),
        true,
    ));
    for (id, member) in [("va", "a"), ("vb", "b"), ("vx", "bx")] {
        model.vcas.push(Vca {
            id: sid(id),
            fader: UNITY,
            members: vec![sid(member)],
        });
    }
    normalized(&model)
}

/// The follow delta, by route ID.
fn follow_delta(
    model: &SessionModel,
    routes: &LiveRouteState,
    solo: &LiveControlSoloState,
) -> Vec<(String, [bool; 2])> {
    let ids: Vec<&Route> = LiveRouteState::live_routes(model).collect();
    LiveRouteMuteFollow::delta(routes, &|strip, lane| solo.effective_mute(strip, lane))
        .map(|(route, lanes)| (ids[route].id.as_str().to_owned(), lanes))
        .collect()
}

/// Stages every owed mute record and follow lane, as a host does at the end of a submission.
fn settle(solo: &mut LiveControlSoloState, routes: &mut LiveRouteState, strips: usize) {
    for strip in 0..strips {
        for (lanes, muted) in solo.strip_delta(strip).into_iter().flatten() {
            solo.record_emitted(strip, lanes, muted);
        }
    }
    let moved: Vec<usize> =
        LiveRouteMuteFollow::delta(routes, &|strip, lane| solo.effective_mute(strip, lane))
            .map(|(route, _)| route)
            .collect();
    for route in moved {
        assert!(routes.follow(route, &|strip, lane| solo.effective_mute(strip, lane)));
    }
    solo.commit();
    routes.commit();
}

/// Sets VCA `vca`'s mute and hands each reached strip's new VCA term to the strip-mute owner (D3).
fn mute_vca(
    state: &mut LiveVcaState,
    solo: &mut LiveControlSoloState,
    vca: usize,
    lanes: BuiltinLaneSelector,
    muted: bool,
) {
    assert!(state.set_vca_mute(vca, lanes, muted));
    for &strip in state.reached_by(vca) {
        assert!(solo.set_vca_mute(strip, state.vca_mute(strip)));
    }
}

/// Gate 5. A VCA mute on a soloed track mutes it (the owner owes the record) and outlives the
/// un-solo; a VCA mute on a solo-safe submix mutes it; a one-lane VCA mute owes one lane; and the
/// follow delta read through the owner's `effective_mute` is exactly the following sends of the
/// VCA-muted strips -- never the send that does not follow.
///
/// Red if solo clears a VCA mute, if solo-safe exempts a strip from it, or if the VCA mute does
/// not reach the follow composition.
#[test]
fn a_vca_mute_wins_over_solo_and_reaches_the_following_sends() {
    let model = follow_session();
    // Strips: a, b, c, bx, by. VCAs: va, vb, vx.
    let (a, b, bx) = (0, 1, 3);
    let (va, vb, vx) = (0, 1, 2);
    let mut state = LiveVcaState::try_new(&model).expect("state");
    let mut solo = solo_state(&model);
    let mut routes =
        LiveRouteState::try_new(&model, &|strip, lane| solo.effective_mute(strip, lane))
            .expect("routes");
    assert!(follow_delta(&model, &routes, &solo).is_empty());

    // Solo `a`, and stage what that owes: `b` and its follow send go quiet, `a` stays open.
    assert!(solo.set_solo(a, true));
    settle(&mut solo, &mut routes, 5);
    assert!(!solo.effective_mute(a, 0));

    // The VCA mute on the soloed track: muted, owed, and only its own follow send moves.
    mute_vca(&mut state, &mut solo, va, BuiltinLaneSelector::Both, true);
    assert_eq!(
        solo.strip_delta(a),
        [Some((BuiltinLaneSelector::Both, true)), None]
    );
    assert!(solo.effective_mute(a, 0) && solo.effective_mute(a, 1));
    assert_eq!(
        follow_delta(&model, &routes, &solo),
        [("a-bx".to_owned(), [true, true])]
    );
    settle(&mut solo, &mut routes, 5);

    // Un-soloing keeps it; `b` reopens and its follow send with it.
    assert!(solo.set_solo(a, false));
    assert!(solo.effective_mute(a, 0) && solo.effective_mute(a, 1));
    assert_eq!(solo.strip_delta(a), [None, None]);
    assert_eq!(
        follow_delta(&model, &routes, &solo),
        [("b-bx".to_owned(), [false, false])]
    );
    settle(&mut solo, &mut routes, 5);

    // The solo-safe submix is not VCA-safe, and its following send into `by` follows.
    mute_vca(&mut state, &mut solo, vx, BuiltinLaneSelector::Both, true);
    assert!(solo.solo_safe(bx));
    assert_eq!(
        solo.strip_delta(bx),
        [Some((BuiltinLaneSelector::Both, true)), None]
    );
    assert_eq!(
        follow_delta(&model, &routes, &solo),
        [("bx-by".to_owned(), [true, true])]
    );
    settle(&mut solo, &mut routes, 5);

    // A one-lane VCA mute: one lane owed, one follow lane.
    mute_vca(&mut state, &mut solo, vb, BuiltinLaneSelector::Right, true);
    assert_eq!(
        solo.strip_delta(b),
        [Some((BuiltinLaneSelector::Right, true)), None]
    );
    assert_eq!(
        follow_delta(&model, &routes, &solo),
        [("b-bx".to_owned(), [false, true])]
    );
    settle(&mut solo, &mut routes, 5);

    // A solo engaged now neither clears nor re-owes a VCA-muted lane.
    assert!(solo.set_solo(b, true));
    assert_eq!(solo.strip_delta(b), [None, None]);
    assert!(solo.effective_mute(b, 1) && !solo.effective_mute(b, 0));
    assert!(!solo.set_vca_mute(5, [true; 2]));
}

// ---- Gate 6 -------------------------------------------------------------------------------------

/// One submission of the D3 flow with every setter and delta, on fixed-size staging only.
fn command_pass(
    state: &mut LiveVcaState,
    solo: &mut LiveControlSoloState,
    routes: &mut LiveRouteState,
    strips: usize,
    step: usize,
) -> usize {
    let mut staged = 0;
    let vca = step % state.vca_count();
    let db = [-6.0, 3.0, -144.0, 24.0][step % 4];
    let lanes = LANES[step % 3];
    assert!(state.set_vca_db(vca, lanes, db));
    state.set_vca_mute(vca, lanes, step.is_multiple_of(2));
    for &strip in state.reached_by(vca) {
        solo.set_vca_mute(strip, state.vca_mute(strip));
    }
    if let Some(strip) = (0..strips).find(|strip| state.reaches(*strip)) {
        state.set_member_db(strip, lanes, db - 1.0);
    }
    for strip in 0..strips {
        for (lanes, db) in state.fader_delta(strip).into_iter().flatten() {
            state.record_emitted_db(strip, lanes, db);
            staged += 1;
        }
        staged += usize::from(state.effective_db(strip, 0) > 0.0);
        for (lanes, muted) in solo.strip_delta(strip).into_iter().flatten() {
            solo.record_emitted(strip, lanes, muted);
            staged += 1;
        }
    }
    let mut moved = [0_usize; 16];
    let mut count = 0;
    for (route, _) in
        LiveRouteMuteFollow::delta(routes, &|strip, lane| solo.effective_mute(strip, lane))
    {
        moved[count] = route;
        count += 1;
    }
    for &route in &moved[..count] {
        routes.follow(route, &|strip, lane| solo.effective_mute(strip, lane));
    }
    if step.is_multiple_of(3) {
        state.rollback();
        solo.rollback();
        routes.rollback();
    } else {
        state.commit();
        solo.commit();
        routes.commit();
    }
    staged + count
}

/// Gate 6. After `try_new`, a sequence of setters, deltas, `record_emitted_db`, `commit` and
/// `rollback`, with the strip-mute owner's `set_vca_mute` and the follow pass, allocates and frees
/// nothing on this thread; `try_new` on a model with no VCA allocates nothing; and
/// `retained_bytes()` is the net bytes `try_new` keeps, with the largest allocation positive and
/// at most that (both 0 for `empty()`).
///
/// Red if a setter or delta allocates on the audio thread, if the VCA-free path allocates arrays
/// that would move a host's retained bytes, or if the reported bytes omit the shadow or a table.
#[test]
fn the_command_path_allocates_nothing_and_the_retained_bytes_are_measured() {
    assert_installed();
    let mut draw = Draw::new(1_244_006);
    let model = normalized(&generate(&mut draw, 6, 3, 8, db, 1_024));
    assert!(!model.vcas.is_empty());
    let strips = model.tracks.len() + model.submixes.len();

    let mark = current_thread_counters();
    let mut state = LiveVcaState::try_new(&model).expect("state");
    let built = current_thread_delta_since(mark);
    assert!(built.allocations > 0);
    assert_eq!(
        state.retained_bytes(),
        built.requested_bytes - built.released_bytes
    );
    assert!(state.largest_allocation_bytes() > 0);
    assert!(state.largest_allocation_bytes() <= state.retained_bytes());

    let mut solo = solo_state(&model);
    let mut routes =
        LiveRouteState::try_new(&model, &|strip, lane| solo.effective_mute(strip, lane))
            .expect("routes");
    assert!(routes.len() <= 16);
    // Warm-up: one pass of every path, so nothing lazily initialized is counted below.
    let mut staged = command_pass(&mut state, &mut solo, &mut routes, strips, 0);
    let mark = current_thread_counters();
    for step in 1..64 {
        staged += command_pass(&mut state, &mut solo, &mut routes, strips, step);
    }
    let used = current_thread_delta_since(mark);
    assert_eq!(
        (used.allocations, used.deallocations),
        (0, 0),
        "the command path allocated or freed"
    );
    assert!(staged > 0);

    let mut plain = model;
    plain.vcas.clear();
    let mark = current_thread_counters();
    let empty = LiveVcaState::try_new(&plain).expect("state");
    let used = current_thread_delta_since(mark);
    assert_eq!((used.allocations, used.deallocations), (0, 0));
    assert_eq!(empty.vca_count(), 0);
    for state in [empty, LiveVcaState::empty()] {
        assert_eq!(state.retained_bytes(), 0);
        assert_eq!(state.largest_allocation_bytes(), 0);
        assert_eq!(state.reached_strip_count(), 0);
        assert!(!state.reaches(0));
    }
}

// ---- The render differential --------------------------------------------------------------------

/// One host prepared with live controls, its handles and the feeds it renders.
struct Live {
    host: PreparedHost,
    handles: HostLiveControlHandles,
    block: usize,
}

impl Live {
    fn new(model: &SessionModel) -> Self {
        let compiled = compile_host_session(&document(model), &caps()).unwrap_or_else(|failure| {
            panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
        let request = HostLiveControlRequest {
            control_queue_depth: Some(NonZeroUsize::new(DEPTH).expect("depth")),
            ..HostLiveControlRequest::default()
        };
        let (host, handles) = prepare_host_runtime_with_live_controls(&compiled, &caps(), &request)
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
        Self {
            host,
            handles,
            block: 0,
        }
    }

    /// Submits and renders the next block; returns its output planes.
    fn render_block(&mut self, feeds: &[(String, Planes)]) -> Planes {
        let range = self.block * QUANTUM..(self.block + 1) * QUANTUM;
        for (id, planes) in feeds {
            self.host
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
        let mut samples = [HOST_SENTINEL; QUANTUM * 2];
        self.host
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
                    absolute_sample: range.start as u64,
                },
            )
            .expect("render");
        self.block += 1;
        [samples[..QUANTUM].to_vec(), samples[QUANTUM..].to_vec()]
    }
}

/// The three live states a host owns, seeded from one normalized model.
struct Mirrors {
    vca: LiveVcaState,
    solo: LiveControlSoloState,
    routes: LiveRouteState,
}

impl Mirrors {
    fn rollback(&mut self) {
        self.vca.rollback();
        self.solo.rollback();
        self.routes.rollback();
    }

    fn commit(&mut self) {
        self.vca.commit();
        self.solo.commit();
        self.routes.commit();
    }
}

/// The intent the test keeps itself, independent of the states under test: the edited model (VCA
/// and member values), each strip's user mute and each track's solo.
#[derive(Clone)]
struct Intent {
    model: SessionModel,
    user: Vec<[bool; 2]>,
    soloed: Vec<bool>,
}

impl Intent {
    /// The session a fresh plan is prepared from: the edited model with each strip's own mute set
    /// to its user mute or its solo-derived mute (the plan has no solo). VCAs stay, so preparation
    /// composes their offsets and mutes itself.
    fn session(&self) -> SessionModel {
        let mut model = self.model.clone();
        let tracks = model.tracks.len();
        let any_solo = self.soloed.iter().any(|soloed| *soloed);
        for (strip, user) in self.user.iter().enumerate() {
            let solo_muted = any_solo && strip < tracks && !self.soloed[strip];
            let fader = strip_fader(&mut model, strip);
            fader.left_mute = user[0] || solo_muted;
            fader.right_mute = user[1] || solo_muted;
        }
        model
    }
}

/// Reach counters for the render differential.
#[derive(Debug, Default)]
struct Reach {
    fader_records: usize,
    mute_records: usize,
    route_records: usize,
    one_lane_vca_mutes: usize,
    vca_muted_submixes: usize,
    vca_muted_followed: usize,
    solos: usize,
    rollbacks: usize,
}

/// One random batch of edits through the states (and the intent alike); then, unless refused,
/// the owed fader, mute and follow records are pushed and the states committed.
fn batch(
    draw: &mut Draw,
    live: &mut Live,
    mirrors: &mut Mirrors,
    intent: &mut Intent,
    reach: &mut Reach,
) {
    let before = intent.clone();
    let tracks = intent.model.tracks.len();
    let strips = intent.user.len();
    for _ in 0..1 + draw.below(4) {
        let lanes = draw.pick(&LANES);
        match draw.below(5) {
            0 | 1 => {
                let (kind, _, vca) =
                    random_edit(draw, &mut mirrors.vca, &mut intent.model, audible_db);
                if kind == 1 {
                    for &strip in mirrors.vca.reached_by(vca) {
                        let lanes = mirrors.vca.vca_mute(strip);
                        assert!(mirrors.solo.set_vca_mute(strip, lanes));
                    }
                }
            }
            2 => {
                let vca = draw.below(intent.model.vcas.len());
                let muted = draw.chance(1, 2);
                mute_vca(&mut mirrors.vca, &mut mirrors.solo, vca, lanes, muted);
                set_mute(&mut intent.model.vcas[vca].fader, lanes, muted);
                reach.one_lane_vca_mutes +=
                    usize::from(muted && lanes != BuiltinLaneSelector::Both);
            }
            3 => {
                let track = draw.below(tracks);
                let engaged = draw.chance(1, 2);
                assert!(mirrors.solo.set_solo(track, engaged));
                intent.soloed[track] = engaged;
                reach.solos += usize::from(engaged);
            }
            _ => {
                let strip = draw.below(strips);
                let muted = draw.chance(1, 3);
                assert!(mirrors.solo.set_user_mute(strip, lanes, muted));
                let mut fader = DualMonoFader {
                    left_mute: intent.user[strip][0],
                    right_mute: intent.user[strip][1],
                    ..UNITY
                };
                set_mute(&mut fader, lanes, muted);
                intent.user[strip] = [fader.left_mute, fader.right_mute];
            }
        }
    }
    if draw.chance(1, 5) {
        mirrors.rollback();
        *intent = before;
        reach.rollbacks += 1;
        return;
    }
    for strip in 0..strips {
        for (lanes, db) in mirrors.vca.fader_delta(strip).into_iter().flatten() {
            live.handles.strip_controls[strip]
                .fader
                .try_push(TrackFaderRecord::FaderDb {
                    lanes,
                    db,
                    smoothing_samples: 0,
                })
                .expect("fader queue room");
            mirrors.vca.record_emitted_db(strip, lanes, db);
            reach.fader_records += 1;
        }
        for (lanes, muted) in mirrors.solo.strip_delta(strip).into_iter().flatten() {
            live.handles.strip_controls[strip]
                .fader
                .try_push(TrackFaderRecord::Mute {
                    lanes,
                    muted,
                    smoothing_samples: 0,
                })
                .expect("fader queue room");
            mirrors.solo.record_emitted(strip, lanes, muted);
            reach.mute_records += 1;
        }
        let vca_muted = mirrors.vca.vca_mute(strip).contains(&true);
        reach.vca_muted_submixes += usize::from(vca_muted && strip >= tracks);
    }
    let solo = &mirrors.solo;
    let moved: Vec<usize> = LiveRouteMuteFollow::delta(&mirrors.routes, &|strip, lane| {
        solo.effective_mute(strip, lane)
    })
    .map(|(route, _)| route)
    .collect();
    for route in moved {
        assert!(
            mirrors
                .routes
                .follow(route, &|strip, lane| solo.effective_mute(strip, lane))
        );
        let entry = *mirrors.routes.get(route).expect("live route");
        live.handles.route_controls[route]
            .set(
                entry.gain_db,
                entry.matrix,
                entry.mute,
                entry.source_lane_muted,
                0,
            )
            .expect("route queue room");
        reach.route_records += 1;
        reach.vca_muted_followed +=
            usize::from(mirrors.vca.vca_mute(entry.source_strip).contains(&true));
    }
    mirrors.commit();
}

const RENDER: &str = "live_vca_moves_render_as_a_fresh_plan";
const RENDER_REPLAY: &str =
    "cargo test -p host-core --test vca_live -- --exact live_vca_moves_render_as_a_fresh_plan";
const BATCHES: usize = 8;

/// The render differential. A session with a random VCA forest, following and non-following
/// sends from every tap and bus-to-bus sends is prepared with live controls. Before each block a
/// random batch -- VCA dB and mute moves (one lane or both), member fader moves, solo toggles and
/// user mutes -- goes through `LiveVcaState`, the strip-mute owner and the route mirror, and their
/// owed records are pushed at zero smoothing, or the batch is refused and rolled back. Every block
/// renders the bits of a plan freshly prepared from the session the test edited the same way.
///
/// A randomized differential: judged by what its generator reaches (asserted below), the staging
/// of the D3 flow end to end, through the render plane.
#[test]
fn live_vca_moves_render_as_a_fresh_plan() {
    let mut reach = Reach::default();
    let ran = run_seeds(RENDER, RENDER_REPLAY, 12, |seed| {
        let mut draw = Draw::new(seed);
        let tracks = 2 + draw.below(3);
        let submixes = 1 + draw.below(3);
        let frames = QUANTUM * (BATCHES + 2);
        let model = normalized(&generate(
            &mut draw, tracks, submixes, 6, audible_db, frames,
        ));
        let feeds: Vec<(String, Planes)> = (0..tracks)
            .map(|index| {
                let mut plane = || -> Vec<f32> {
                    (0..frames)
                        .map(|_| {
                            let sample = draw.noise(0.5);
                            if sample == 0.0 { 0.25 } else { sample }
                        })
                        .collect()
                };
                (format!("t{index}"), [plane(), plane()])
            })
            .collect();
        let mut live = Live::new(&model);
        let names: Vec<&str> = model.strips().map(|strip| strip.id.as_str()).collect();
        assert_eq!(
            live.handles
                .strips
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
            names
        );
        let solo = solo_state(&model);
        let routes =
            LiveRouteState::try_new(&model, &|strip, lane| solo.effective_mute(strip, lane))
                .expect("routes");
        assert_eq!(live.handles.route_controls.len(), routes.len());
        let mut mirrors = Mirrors {
            vca: LiveVcaState::try_new(&model).expect("state"),
            solo,
            routes,
        };
        let mut intent = Intent {
            user: model
                .strips()
                .map(|strip| [strip.fader.left_mute, strip.fader.right_mute])
                .collect(),
            soloed: vec![false; tracks],
            model,
        };
        for block in 0..BATCHES {
            if block > 0 {
                batch(&mut draw, &mut live, &mut mirrors, &mut intent, &mut reach);
            }
            let actual = live.render_block(&feeds);
            let mut fresh = Live::new(&intent.session());
            let mut expected = fresh.render_block(&feeds);
            for _ in 0..block {
                expected = fresh.render_block(&feeds);
            }
            for plane in 0..2 {
                if let Some(index) = first_difference(&actual[plane], &expected[plane]) {
                    panic!(
                        "seed {seed}, block {block}, plane {plane}, frame {index}: {:?} != {:?}",
                        actual[plane][index], expected[plane][index]
                    );
                }
            }
        }
    });
    if !replaying() {
        assert!(ran >= 12, "{ran} seeds");
        let counts = [
            reach.fader_records,
            reach.mute_records,
            reach.route_records,
            reach.one_lane_vca_mutes,
            reach.vca_muted_submixes,
            reach.vca_muted_followed,
            reach.solos,
            reach.rollbacks,
        ];
        assert!(counts.iter().all(|count| *count > 0), "{reach:?}");
    }
}
