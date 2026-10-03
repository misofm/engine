//! Issue #1242: a VCA applies at preparation, on the path the browser and the C ABI share.
//!
//! * **Gate 2, the effective fader equals a plain fader.** Random VCA forests render bit-identically
//!   to the same session with no VCA and each strip's fader written as the test's own `f64`
//!   reference value.
//! * **Gate 3, a post-fader send follows the VCA, and a VCA mute silences a member's follow send in
//!   a fresh plan** (VERIFY-2 M12). The sealed graph text's `route-follow-zeroed` rows are
//!   `graph-compiler`'s `vca_follow` test.
//! * **Gate 4, a VCA mutes a submix member.**
//!
//! Every comparison feeds distinct noise per track and lane through strips with an empty console,
//! no inserts and an identity input section, and reads the session output through unity routes.

use std::collections::BTreeSet;

use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use host_core::{
    HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission, compile_host_session,
    prepare_host_runtime,
};
use session::{
    ChannelMatrix, Console, DualMonoFader, Route, RouteDestination, RouteSource, SendTap,
    SessionModel, Source, StableId, Submix, Vca, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 4;
const FRAMES: usize = QUANTUM * BLOCKS;
/// What every host output buffer holds before a render, so an unwritten sample shows.
const HOST_SENTINEL: f32 = 7.0;

/// A stereo signal's two planes.
type Planes = [Vec<f32>; 2];
/// Each track's id and its source planes.
type Feeds = Vec<(String, Planes)>;

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

/// A unity, open, non-following route.
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
    )
}

/// The fixture emptied of tracks, routes, sources, automation and console slots, at 48 kHz, with a
/// transparent (unity) track template and its source.
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

/// Distinct nonzero noise on each lane.
fn planes(draw: &mut Draw) -> Planes {
    let mut plane = || {
        (0..FRAMES)
            .map(|_| {
                let sample = draw.noise(0.9);
                if sample == 0.0 { 0.5 } else { sample }
            })
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

fn submit(prepared: &mut PreparedHost, feeds: &Feeds, block: usize) {
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

/// Compiles and prepares `model` through the host path the browser and the C ABI share, renders
/// [`BLOCKS`] quanta, and returns the session output's planes.
fn render(model: &SessionModel, feeds: &Feeds) -> Planes {
    let document = canonical_session_json(model).expect("generated session canonicalizes");
    let compiled = compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
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
    out
}

/// Both planes the same, NaNs folded (decision 10).
fn assert_planes_equal(actual: &Planes, expected: &Planes, what: &str) {
    for plane in 0..2 {
        if let Some(index) = first_difference(&actual[plane], &expected[plane]) {
            panic!(
                "{what}: plane {plane} sample {index}: {:?} != {:?}",
                actual[plane][index], expected[plane][index]
            );
        }
    }
}

fn faders_mut(model: &mut SessionModel) -> impl Iterator<Item = &mut DualMonoFader> {
    model
        .tracks
        .iter_mut()
        .map(|track| &mut track.fader)
        .chain(model.submixes.iter_mut().map(|submix| &mut submix.fader))
}

// ---- Gate 2 --------------------------------------------------------------------------------

/// The test's own reach: `members` closed downward from each VCA (the forest is acyclic by
/// construction), then, per strip, the IDs of the VCAs whose closure holds it, in ascending ID
/// order. It shares nothing with `SessionModel::vca_reach`, which walks upward.
fn reference_reach(vcas: &[Vca], strip: &str) -> Vec<usize> {
    fn below<'a>(vcas: &'a [Vca], index: usize, into: &mut BTreeSet<&'a str>) {
        for member in &vcas[index].members {
            if into.insert(member.as_str())
                && let Some(nested) = vcas.iter().position(|vca| vca.id == *member)
            {
                below(vcas, nested, into);
            }
        }
    }
    let mut reach: Vec<usize> = (0..vcas.len())
        .filter(|index| {
            let mut closure = BTreeSet::new();
            below(vcas, *index, &mut closure);
            closure.contains(strip)
        })
        .collect();
    reach.sort_by(|left, right| vcas[*left].id.cmp(&vcas[*right].id));
    reach
}

/// Decision 13 (a) as the test states it: `own + offsets` in `f64`, the offsets in ascending VCA
/// ID order, clamped to `[-144, 24]` in `f64`, rounded once; the own value exactly when unreached.
fn reference_db(own: f32, offsets: &[f32]) -> f32 {
    if offsets.is_empty() {
        return own;
    }
    let mut sum = f64::from(own);
    for offset in offsets {
        sum += f64::from(*offset);
    }
    sum.clamp(-144.0, 24.0) as f32
}

/// A random forest of up to eight VCAs over `strips`, at most four levels deep: a VCA's members
/// are strips and VCAs of strictly deeper levels, up to 16, so VCAs overlap and diamonds occur.
/// Offsets are drawn across the whole domain or near unity, so sums both clamp and do not; each
/// lane's mute is drawn. Declared in an order unrelated to ID order.
fn forest(draw: &mut Draw, strips: &[String]) -> Vec<Vca> {
    let count = 1 + draw.below(8);
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
            let offset = |draw: &mut Draw| {
                if draw.chance(1, 3) {
                    draw.in_domain(-144.0, 24.0)
                } else {
                    draw.in_domain(-12.0, 12.0)
                }
            };
            Vca {
                id: sid(&names[index]),
                fader: DualMonoFader {
                    left_db: offset(draw),
                    right_db: offset(draw),
                    left_mute: draw.chance(1, 4),
                    right_mute: draw.chance(1, 4),
                },
                members: candidates[..take]
                    .iter()
                    .map(|member| sid(member))
                    .collect(),
            }
        })
        .collect()
}

/// Gate 2's session: tracks `t0..` and submixes `s0..`, every strip's own fader drawn across the
/// domain with drawn mutes. Each strip reaches the output by `<strip>-main` from its `post_pan`
/// tap, and submix `s<j>` is fed from the `input` tap of track `t<j mod n>`, which no fader
/// touches. Only `probe`'s `-main` route is open, so the output is that one strip's lanes.
fn forest_session(draw: &mut Draw) -> (SessionModel, Vec<String>, Feeds) {
    let (mut model, source, track) = empty_session();
    let tracks = 2 + draw.below(4);
    let submixes = 1 + draw.below(3);
    let mut strips = Vec::new();
    let mut feeds = Vec::new();
    for index in 0..tracks {
        let id = format!("t{index}");
        add_track(&mut model, &source, &track, &id);
        model.routes.push(to_output(
            &format!("{id}-main"),
            track_tap(&id, SendTap::PostPan),
        ));
        feeds.push((id.clone(), planes(draw)));
        strips.push(id);
    }
    for index in 0..submixes {
        let id = format!("s{index}");
        model.submixes.push(Submix::unity(sid(&id), &model.console));
        let feeder = format!("t{}", index % tracks);
        model.routes.push(route(
            &format!("{feeder}-{id}"),
            track_tap(&feeder, SendTap::Input),
            into_bus(&id),
        ));
        model.routes.push(to_output(
            &format!("{id}-main"),
            bus_tap(&id, SendTap::PostPan),
        ));
        strips.push(id);
    }
    for fader in faders_mut(&mut model) {
        fader.left_db = draw.in_domain(-144.0, 24.0);
        fader.right_db = draw.in_domain(-144.0, 24.0);
        fader.left_mute = draw.chance(1, 5);
        fader.right_mute = draw.chance(1, 5);
    }
    model.vcas = forest(draw, &strips);
    (model, strips, feeds)
}

const FOREST: &str = "a_vca_forest_renders_the_bits_of_its_effective_faders";
const FOREST_REPLAY: &str = "cargo test -p host-core --test vca -- --exact \
                             a_vca_forest_renders_the_bits_of_its_effective_faders";

/// #1242 gate 2. A random VCA forest (up to four deep, up to 16 members, overlapping, with
/// diamonds, tracks and submixes as members, offsets and own faders across the whole domain,
/// clamping included, both lanes) renders each strip's output bit-identically to the same session
/// with `vcas: []` and that strip's fader written as the test's own reference: `reference_db` over
/// `reference_reach`'s offsets, and the own mute ORed with every reaching VCA's. Each strip is
/// probed alone, so a wrong lane gain is never hidden under another strip's sum.
///
/// Red if an offset reaches the wrong strip or lane, misses a submix or nested member, counts a
/// diamond twice, or the builtins compiler bakes another value than the composition's.
#[test]
fn a_vca_forest_renders_the_bits_of_its_effective_faders() {
    let mut reached = [0_usize; 3];
    let ran = run_seeds(FOREST, FOREST_REPLAY, 32, |seed| {
        let mut draw = Draw::new(seed);
        let (mut model, strips, feeds) = forest_session(&mut draw);
        let mut plain = model.clone();
        plain.vcas.clear();
        let vcas = model.vcas.clone();
        for (fader, strip) in faders_mut(&mut plain).zip(&strips) {
            let reach = reference_reach(&vcas, strip);
            let offsets = |lane: usize| -> Vec<f32> {
                reach
                    .iter()
                    .map(|index| [vcas[*index].fader.left_db, vcas[*index].fader.right_db][lane])
                    .collect()
            };
            let muted = |lane: usize| {
                reach.iter().any(|index| {
                    [vcas[*index].fader.left_mute, vcas[*index].fader.right_mute][lane]
                })
            };
            if reach.len() > 1 {
                reached[0] += 1;
            }
            let db = [
                reference_db(fader.left_db, &offsets(0)),
                reference_db(fader.right_db, &offsets(1)),
            ];
            if db
                .iter()
                .any(|value| value.abs() == 144.0 || *value == 24.0)
                && !reach.is_empty()
            {
                reached[1] += 1;
            }
            if strip.starts_with('s') && !reach.is_empty() {
                reached[2] += 1;
            }
            (fader.left_db, fader.right_db) = (db[0], db[1]);
            fader.left_mute |= muted(0);
            fader.right_mute |= muted(1);
        }
        for probe in &strips {
            let open = format!("{probe}-main");
            for model in [&mut model, &mut plain] {
                for route in &mut model.routes {
                    route.mute = route.id.as_str().ends_with("-main") && route.id.as_str() != open;
                }
            }
            assert_planes_equal(
                &render(&model, &feeds),
                &render(&plain, &feeds),
                &format!("seed {seed}, strip {probe}"),
            );
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 32);
        assert!(
            reached.iter().all(|count| *count > 0),
            "the seeds reach multi-VCA strips, clamped sums and submix members: {reached:?}"
        );
    }
}

// ---- Gates 3 and 4 -------------------------------------------------------------------------

/// Tracks `kick`, `snare` and `tom`, each with distinct noise and its own fader; `kick` sends
/// `post_fader` and `snare` sends `pre_fader` with `follows_mute` into bus `verb`, whose `input`
/// tap reaches the output by the unity route `verb-main`, so the output is `verb`'s input. VCA
/// `drums` holds `kick` and `snare` at -6 dB on both lanes with `mutes`; `tom` is not a member and
/// reaches nothing.
fn drums_session(draw: &mut Draw, mutes: [bool; 2]) -> (SessionModel, Feeds) {
    let (mut model, source, track) = empty_session();
    let mut feeds = Vec::new();
    for id in ["kick", "snare", "tom"] {
        add_track(&mut model, &source, &track, id);
        feeds.push((id.to_owned(), planes(draw)));
    }
    model
        .submixes
        .push(Submix::unity(sid("verb"), &model.console));
    let mut snare = route(
        "snare-verb",
        track_tap("snare", SendTap::PreFader),
        into_bus("verb"),
    );
    snare.follows_mute = true;
    model.routes = vec![
        route(
            "kick-verb",
            track_tap("kick", SendTap::PostFader),
            into_bus("verb"),
        ),
        snare,
        to_output("verb-main", bus_tap("verb", SendTap::Input)),
    ];
    model.vcas = vec![Vca {
        id: sid("drums"),
        fader: DualMonoFader {
            left_db: -6.0,
            right_db: -6.0,
            left_mute: mutes[0],
            right_mute: mutes[1],
        },
        members: vec![sid("kick"), sid("snare")],
    }];
    (model, feeds)
}

/// The position of route `id` in route-ID order: its index in the route-mix counts.
fn route_index(model: &SessionModel, id: &str) -> usize {
    let mut ids: Vec<&str> = model.routes.iter().map(|route| route.id.as_str()).collect();
    ids.sort_unstable();
    ids.iter()
        .position(|candidate| *candidate == id)
        .expect("a declared route")
}

const DRUMS: &str = "a_post_fader_send_follows_the_vca_and_a_vca_mute_silences_a_follow_send";
const DRUMS_REPLAY: &str = "cargo test -p host-core --features graph/test-support --test vca -- \
                            --exact \
                            a_post_fader_send_follows_the_vca_and_a_vca_mute_silences_a_follow_send";

/// #1242 gate 3 (VERIFY-2 M12). With `drums` at -6 dB and unmuted, `verb`'s input equals a
/// VCA-free session with only `kick`'s fader at -6 dB: `kick`'s post-fader send drops with the
/// VCA, `snare`'s pre-fader send does not. With `drums` muted on the left lane, then on both,
/// `verb`'s input equals the VCA-free session with `kick`'s and `snare`'s own dB and mutes set to
/// their effective values, and differs from the one with the VCA dropped (the mute is not
/// vacuous); muted on both lanes, `snare-verb` is silenced and undelayed, so its op never mixes.
///
/// Red if the VCA is applied after the sends (the post-fader send keeps its level), or if route
/// lowering reads the member's own mute, so a fresh plan leaks a VCA-muted member's pre-fader
/// send.
#[test]
fn a_post_fader_send_follows_the_vca_and_a_vca_mute_silences_a_follow_send() {
    let ran = run_seeds(DRUMS, DRUMS_REPLAY, 8, |seed| {
        let mut draw = Draw::new(seed);
        let (model, feeds) = drums_session(&mut draw, [false; 2]);
        let mut plain = model.clone();
        plain.vcas.clear();
        (
            plain.tracks[0].fader.left_db,
            plain.tracks[0].fader.right_db,
        ) = (-6.0, -6.0);
        assert_planes_equal(
            &render(&model, &feeds),
            &render(&plain, &feeds),
            &format!("seed {seed}, drums at -6 dB"),
        );
        for mutes in [[true, false], [true, true]] {
            let (model, feeds) = drums_session(&mut Draw::new(seed), mutes);
            let mut dropped = model.clone();
            dropped.vcas.clear();
            let mut plain = dropped.clone();
            for fader in &mut plain.tracks[..2] {
                (fader.fader.left_db, fader.fader.right_db) = (-6.0, -6.0);
                (fader.fader.left_mute, fader.fader.right_mute) = (mutes[0], mutes[1]);
            }
            let what = format!("seed {seed}, drums muted {mutes:?}");
            let actual = render(&model, &feeds);
            let mixes = graph::test_only_route_mix_counts();
            let activity_built = graph::test_only_route_activity_built();
            assert_planes_equal(&actual, &render(&plain, &feeds), &what);
            assert_ne!(
                actual,
                render(&dropped, &feeds),
                "{what}: dropping the VCA leaks the snare send"
            );
            if mutes == [true, true] {
                assert!(
                    activity_built,
                    "{what}: a fully follow-zeroed send builds the route-activity table"
                );
                assert_eq!(
                    mixes[route_index(&model, "snare-verb")],
                    0,
                    "{what}: snare-verb mixed"
                );
            }
        }
    });
    if !dsp_reference::randomized::replaying() {
        assert_eq!(ran, 8);
    }
}

/// #1242 gate 4. Track `t` feeds bus `drums` from its `input` tap, and `drums` reaches the output
/// from its `post_pan` tap. VCA `bus-vca` muting `drums` on both lanes makes the output exactly
/// `+0.0`, bit-identically to the bus's own mute; unmuted, the output carries `t`'s noise.
///
/// Red if a VCA mute reaches tracks only.
#[test]
fn a_vca_mutes_a_submix_member() {
    let mut draw = Draw::new(1_242);
    let (mut model, source, track) = empty_session();
    add_track(&mut model, &source, &track, "t");
    let feeds = vec![("t".to_owned(), planes(&mut draw))];
    model
        .submixes
        .push(Submix::unity(sid("drums"), &model.console));
    model.routes = vec![
        route("t-drums", track_tap("t", SendTap::Input), into_bus("drums")),
        to_output("drums-main", bus_tap("drums", SendTap::PostPan)),
    ];
    let mut own = model.clone();
    (
        own.submixes[0].fader.left_mute,
        own.submixes[0].fader.right_mute,
    ) = (true, true);
    let vca = |muted: bool| Vca {
        id: sid("bus-vca"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: muted,
            right_mute: muted,
        },
        members: vec![sid("drums")],
    };
    model.vcas = vec![vca(false)];
    let open = render(&model, &feeds);
    assert!(
        open.iter()
            .all(|plane| plane.iter().any(|sample| *sample != 0.0)),
        "an unmuted VCA leaves the bus audible"
    );
    model.vcas = vec![vca(true)];
    let muted = render(&model, &feeds);
    for plane in &muted {
        assert!(
            plane.iter().all(|sample| sample.to_bits() == 0),
            "a VCA-muted bus renders exact +0.0"
        );
    }
    assert_planes_equal(&muted, &render(&own, &feeds), "the bus's own mute");
}
