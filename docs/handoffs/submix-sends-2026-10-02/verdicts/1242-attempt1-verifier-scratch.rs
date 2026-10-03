// Sol #1242 attempt-1 verifier scratch (never committed).
// Part 1: append to hosts/host-web/src/tests.rs (P1 reproducer, P2 randomized browser differential).

// ---- Sol #1242 attempt 1 verifier probes ----------------------------------------------------

/// Sol probe P1: a kind 4 un-mute with `channel` 2 (both lanes) on a member whose VCA mutes one
/// lane only must leave the VCA-muted lane muted and the other open: bit-identical to a host that
/// received nothing (the member's own mute was already `false`).
#[test]
fn sol_probe_both_lane_unmute_on_a_one_lane_vca_mute() {
    for vca_mutes in [[true, false], [false, true]] {
        let mut live = vca_follow_host(&[], vca_mutes, 0.0);
        let mut untouched = vca_follow_host(&[], vca_mutes, 0.0);
        follow_lockstep(
            &mut [&mut live, &mut untouched],
            &SOLO_FOLLOW_TRACKS,
            0,
            1,
            false,
            "warm-up",
        );
        stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, false, 0);
        assert_eq!(live.submit_commands(1), RESULT_OK, "un-mute drums, both lanes");
        let solo = &live.ready.as_ref().expect("ready").solo;
        let emitted = [solo.emitted_mute(1, 0), solo.emitted_mute(1, 1)];
        let effective = [solo.effective_mute(1, 0), solo.effective_mute(1, 1)];
        eprintln!("vca {vca_mutes:?}: emitted {emitted:?} effective {effective:?}");
        let _ = (emitted, effective);
        assert!(
            follow_lockstep(
                &mut [&mut live, &mut untouched],
                &SOLO_FOLLOW_TRACKS,
                1,
                3,
                true,
                &format!("vca {vca_mutes:?}: both-lane un-mute vs untouched"),
            ),
            "audible"
        );
    }
}

struct SolRng(u64);
impl SolRng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }
    fn range(&mut self, low: f32, high: f32) -> f32 {
        let unit = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        (low + (high - low) * unit).clamp(low, high)
    }
}

fn sol_reach(vcas: &[session::Vca], strip: &str) -> Vec<usize> {
    fn below<'a>(vcas: &'a [session::Vca], index: usize, into: &mut std::collections::BTreeSet<&'a str>) {
        for member in &vcas[index].members {
            if into.insert(member.as_str()) {
                if let Some(nested) = vcas.iter().position(|vca| vca.id == *member) {
                    below(vcas, nested, into);
                }
            }
        }
    }
    let mut reach: Vec<usize> = (0..vcas.len())
        .filter(|index| {
            let mut closure = std::collections::BTreeSet::new();
            below(vcas, *index, &mut closure);
            closure.contains(strip)
        })
        .collect();
    reach.sort_by(|l, r| vcas[*l].id.cmp(&vcas[*r].id));
    reach
}

/// Sol probe P2: random VCA forests over #1224's follow session (3 tracks, 2 buses, 2 following
/// pre-fader sends), then random batches of solo toggles and kind 4 edits (smoothing 0). The VCA
/// host must render bit-identically, every block, to a host booted from the same session with no
/// VCA and every strip's fader written as its independently computed effective value, which
/// receives the same solos and, for each kind 4, the per-lane equivalent `value || vca_mute[l]`.
#[test]
fn sol_probe_browser_vca_equals_written_directly_under_solo_and_mute() {
    let names = ["bass", "drums", "vocal", "room", "verb"];
    let only_solo = std::env::var("SOL_ONLY_SOLO").is_ok();
    let mut failures = Vec::new();
    let seeds: u64 = std::env::var("SOL_SEEDS").ok().and_then(|v| v.parse().ok()).unwrap_or(48);
    for seed in 1..=seeds {
        let mut rng = SolRng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        let document = follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            &[],
            None,
        );
        let mut model = parse_session_json(&document).expect("parses");
        {
            let faders = model
                .tracks
                .iter_mut()
                .map(|t| &mut t.fader)
                .chain(model.submixes.iter_mut().map(|s| &mut s.fader));
            for fader in faders {
                fader.left_db = rng.range(-12.0, 6.0);
                fader.right_db = rng.range(-12.0, 6.0);
                fader.left_mute = rng.chance(1, 6);
                fader.right_mute = rng.chance(1, 6);
            }
        }
        let count = 1 + rng.below(4) as usize;
        let levels: Vec<u64> = (0..count).map(|_| rng.below(3)).collect();
        let vca_names: Vec<String> = (0..count).map(|i| format!("v{}", (i * 7) % 11)).collect();
        model.vcas = (0..count)
            .map(|i| {
                let mut candidates: Vec<String> = names.iter().map(|s| (*s).to_owned()).collect();
                candidates.extend((0..count).filter(|o| levels[*o] > levels[i]).map(|o| vca_names[o].clone()));
                let members: Vec<session::StableId> = candidates
                    .iter()
                    .filter(|_| rng.chance(1, 2))
                    .map(|m| strip_id(m))
                    .collect();
                let mut offset = || if rng.chance(1, 4) { rng.range(-144.0, 24.0) } else { rng.range(-12.0, 12.0) };
                let (l, r) = (offset(), offset());
                session::Vca {
                    id: strip_id(&vca_names[i]),
                    fader: session::DualMonoFader {
                        left_db: l,
                        right_db: r,
                        left_mute: rng.chance(1, 3),
                        right_mute: rng.chance(1, 3),
                    },
                    members,
                }
            })
            .collect();
        // Strip order: tracks then submixes, canonical (sorted) order.
        let canonical = parse_session_json(&canonical_session_json(&model).expect("canon")).expect("reparse");
        let strip_ids: Vec<String> = canonical
            .tracks
            .iter()
            .map(|t| t.id.as_str().to_owned())
            .chain(canonical.submixes.iter().map(|s| s.id.as_str().to_owned()))
            .collect();
        let mut plain = canonical.clone();
        plain.vcas.clear();
        let mut vca_mute = Vec::new();
        {
            let faders = plain
                .tracks
                .iter_mut()
                .map(|t| &mut t.fader)
                .chain(plain.submixes.iter_mut().map(|s| &mut s.fader));
            for (fader, strip) in faders.zip(&strip_ids) {
                let reach = sol_reach(&canonical.vcas, strip);
                let lane = |l: usize| -> (f32, bool) {
                    let own = [fader.left_db, fader.right_db][l];
                    if reach.is_empty() {
                        return (own, false);
                    }
                    let mut sum = f64::from(own);
                    let mut muted = false;
                    for i in &reach {
                        let f = &canonical.vcas[*i].fader;
                        sum += f64::from([f.left_db, f.right_db][l]);
                        muted |= [f.left_mute, f.right_mute][l];
                    }
                    (sum.clamp(-144.0, 24.0) as f32, muted)
                };
                let (l, r) = (lane(0), lane(1));
                fader.left_db = l.0;
                fader.right_db = r.0;
                fader.left_mute |= l.1;
                fader.right_mute |= r.1;
                vca_mute.push([l.1, r.1]);
            }
        }
        let mut live = strip_boot(&canonical_session_json(&canonical).expect("canon"), strip_options(64, 0, 0));
        let mut reference = strip_boot(&canonical_session_json(&plain).expect("canon"), strip_options(64, 0, 0));
        follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 0, 1, true, &format!("seed {seed} boot"));
        let mut block = 1;
        let mut log = Vec::new();
        for _round in 0..8 {
            let commands = 1 + rng.below(3) as usize;
            let (mut a, mut b) = (0_usize, 0_usize);
            for _ in 0..commands {
                if only_solo || rng.chance(1, 2) {
                    let track = rng.below(3) as u32;
                    let on = rng.chance(1, 2);
                    stage_solo(&mut live, a, track, on, 0);
                    stage_solo(&mut reference, b, track, on, 0);
                    a += 1;
                    b += 1;
                    log.push(format!("solo {track} {on}"));
                } else {
                    let strip = rng.below(5) as usize;
                    let channel = rng.below(3) as u8;
                    let on = rng.chance(1, 2);
                    stage_lane_mute(&mut live, a, strip as u32, channel, on, 0);
                    a += 1;
                    let want = |l: usize| on || vca_mute[strip][l];
                    match channel {
                        0 => { stage_lane_mute(&mut reference, b, strip as u32, 0, want(0), 0); b += 1; }
                        1 => { stage_lane_mute(&mut reference, b, strip as u32, 1, want(1), 0); b += 1; }
                        _ => {
                            if want(0) == want(1) {
                                stage_lane_mute(&mut reference, b, strip as u32, 2, want(0), 0); b += 1;
                            } else {
                                stage_lane_mute(&mut reference, b, strip as u32, 0, want(0), 0); b += 1;
                                stage_lane_mute(&mut reference, b, strip as u32, 1, want(1), 0); b += 1;
                            }
                        }
                    }
                    log.push(format!("mute strip {strip} ch {channel} {on} (vca {:?})", vca_mute[strip]));
                }
            }
            assert_eq!(live.submit_commands(a as u32), RESULT_OK, "seed {seed} live submit");
            assert_eq!(reference.submit_commands(b as u32), RESULT_OK, "seed {seed} reference submit");
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, block, 1, true, &format!("seed {seed}"));
            }));
            block += 1;
            if result.is_err() {
                failures.push(format!("seed {seed}: after {log:?}"));
                break;
            }
        }
    }
    eprintln!("failures: {}", failures.len());
    for f in failures.iter().take(4) {
        eprintln!("{f}");
    }
    assert!(failures.is_empty(), "{} failing seeds", failures.len());
}

// Part 2: crates/host-core/tests/sol_vca_probe.rs (prepared-path randomized differential, VCA floor).
// //! Sol #1242 attempt-1 verifier probe (scratch, never committed).
// //!
// //! Random nested VCA forests over tracks AND submixes, with random sends (every non-insert tap,
// //! `follows_mute` or not, track -> submix and submix -> submix, random gain and matrix), render
// //! the whole session output bit-identically to the same session with `vcas: []` and every strip's
// //! fader written as its effective value computed independently here (top-down closure, f64 sum
// //! in ascending VCA-ID order, clamp, one rounding; mute = own || any reaching VCA's mute).

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

type Planes = [Vec<f32>; 2];
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
        maximum_routes: 200,
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

fn render(model: &SessionModel, feeds: &Feeds) -> Planes {
    let document = canonical_session_json(model).expect("canonicalizes");
    let compiled = compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut prepared = prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
        submit(&mut prepared, feeds, block);
        let mut samples = [7.0_f32; QUANTUM * 2];
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

fn reference_reach(vcas: &[Vca], strip: &str) -> Vec<usize> {
    fn below<'a>(vcas: &'a [Vca], index: usize, into: &mut BTreeSet<&'a str>) {
        for member in &vcas[index].members {
            if into.insert(member.as_str()) {
                if let Some(nested) = vcas.iter().position(|vca| vca.id == *member) {
                    below(vcas, nested, into);
                }
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
    reach.sort_by(|l, r| vcas[*l].id.cmp(&vcas[*r].id));
    reach
}

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

const TAPS: [SendTap; 5] = [
    SendTap::Input,
    SendTap::PostInput,
    SendTap::PreFader,
    SendTap::PostFader,
    SendTap::PostPan,
];

fn send(id: String, source: RouteSource, destination: RouteDestination, draw: &mut Draw) -> Route {
    let coefficient = |draw: &mut Draw| {
        if draw.chance(1, 2) {
            draw.unit() * 2.0 - 1.0
        } else {
            0.0
        }
    };
    let ll = 0.5 + 0.5 * draw.unit();
    let lr = coefficient(draw);
    let rl = coefficient(draw);
    let rr = 0.5 + 0.5 * draw.unit();
    let matrix = ChannelMatrix { ll, lr, rl, rr };
    Route {
        id: sid(&id),
        source,
        destination,
        channel_matrix: matrix,
        gain_db: draw.in_domain(-12.0, 6.0),
        mute: draw.chance(1, 10),
        follows_mute: false,
    }
}

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
                members: candidates[..take].iter().map(|member| sid(member)).collect(),
            }
        })
        .collect()
}

fn session_with_sends(draw: &mut Draw) -> (SessionModel, Vec<String>, Feeds) {
    let (mut model, source, track) = empty_session();
    let tracks = 2 + draw.below(4);
    let submixes = 1 + draw.below(4);
    let mut strips = Vec::new();
    let mut feeds = Vec::new();
    let out = || RouteDestination::OutputInput {
        output_id: sid("main-out"),
    };
    for index in 0..tracks {
        let id = format!("t{index}");
        let mut s = source.clone();
        s.id = sid(&id);
        model.sources.push(s);
        let mut t = track.clone();
        t.id = sid(&id);
        t.source_id = sid(&id);
        model.tracks.push(t);
        feeds.push((id.clone(), planes(draw)));
        strips.push(id);
    }
    for index in 0..submixes {
        let id = format!("s{index}");
        model.submixes.push(Submix::unity(sid(&id), &model.console));
        strips.push(id);
    }
    let mut routes = Vec::new();
    for t in 0..tracks {
        if draw.chance(3, 4) {
            let tap = draw.pick(&TAPS);
            routes.push(send(
                format!("t{t}-main"),
                RouteSource::Track { track_id: sid(&format!("t{t}")), tap },
                out(),
                draw,
            ));
        }
        for s in 0..submixes {
            if draw.chance(1, 2) {
                let tap = draw.pick(&TAPS);
                let mut route = send(
                    format!("t{t}-s{s}"),
                    RouteSource::Track { track_id: sid(&format!("t{t}")), tap },
                    RouteDestination::SubmixInput { submix_id: sid(&format!("s{s}")) },
                    draw,
                );
                route.follows_mute = draw.chance(1, 2);
                routes.push(route);
            }
        }
    }
    for s in 0..submixes {
        let tap = draw.pick(&TAPS);
        routes.push(send(
            format!("s{s}-main"),
            RouteSource::Submix { submix_id: sid(&format!("s{s}")), tap },
            out(),
            draw,
        ));
        for k in s + 1..submixes {
            if draw.chance(1, 2) {
                let tap = draw.pick(&TAPS);
                let mut route = send(
                    format!("s{s}-s{k}"),
                    RouteSource::Submix { submix_id: sid(&format!("s{s}")), tap },
                    RouteDestination::SubmixInput { submix_id: sid(&format!("s{k}")) },
                    draw,
                );
                route.follows_mute = draw.chance(1, 2);
                routes.push(route);
            }
        }
    }
    model.routes = routes;
    let faders = model
        .tracks
        .iter_mut()
        .map(|t| &mut t.fader)
        .chain(model.submixes.iter_mut().map(|s| &mut s.fader));
    for fader in faders {
        let db = |draw: &mut Draw| {
            if draw.chance(1, 3) {
                draw.in_domain(-144.0, 24.0)
            } else {
                draw.in_domain(-12.0, 6.0)
            }
        };
        fader.left_db = db(draw);
        fader.right_db = db(draw);
        fader.left_mute = draw.chance(1, 5);
        fader.right_mute = draw.chance(1, 5);
    }
    model.vcas = forest(draw, &strips);
    (model, strips, feeds)
}

fn written_directly(model: &SessionModel, strips: &[String]) -> SessionModel {
    let mut plain = model.clone();
    plain.vcas.clear();
    let vcas = model.vcas.clone();
    let faders = plain
        .tracks
        .iter_mut()
        .map(|t| &mut t.fader)
        .chain(plain.submixes.iter_mut().map(|s| &mut s.fader));
    for (fader, strip) in faders.zip(strips) {
        let reach = reference_reach(&vcas, strip);
        let offsets = |lane: usize| -> Vec<f32> {
            reach
                .iter()
                .map(|i| [vcas[*i].fader.left_db, vcas[*i].fader.right_db][lane])
                .collect()
        };
        let muted = |lane: usize| {
            reach
                .iter()
                .any(|i| [vcas[*i].fader.left_mute, vcas[*i].fader.right_mute][lane])
        };
        let db = [
            reference_db(fader.left_db, &offsets(0)),
            reference_db(fader.right_db, &offsets(1)),
        ];
        (fader.left_db, fader.right_db) = (db[0], db[1]);
        fader.left_mute |= muted(0);
        fader.right_mute |= muted(1);
    }
    plain
}

#[test]
fn sol_probe_vca_forest_with_sends_and_follows_equals_written_directly() {
    let mut stats = [0_usize; 4];
    let ran = run_seeds(
        "sol_probe_vca_forest_with_sends_and_follows_equals_written_directly",
        "cargo test -p host-core --test sol_vca_probe",
        96,
        |seed| {
            let mut draw = Draw::new(seed ^ 0x5017_1242);
            let (model, strips, feeds) = session_with_sends(&mut draw);
            let plain = written_directly(&model, &strips);
            // The implementation's own composition agrees with the independent one.
            let effective = model.effective_strip_faders();
            let faders: Vec<&DualMonoFader> = plain
                .tracks
                .iter()
                .map(|t| &t.fader)
                .chain(plain.submixes.iter().map(|s| &s.fader))
                .collect();
            for (index, (fader, value)) in faders.iter().zip(&effective).enumerate() {
                assert_eq!(
                    [fader.left_db.to_bits(), fader.right_db.to_bits()],
                    [value.db[0].to_bits(), value.db[1].to_bits()],
                    "seed {seed} strip {index} db"
                );
                assert_eq!([fader.left_mute, fader.right_mute], value.mute, "seed {seed} strip {index} mute");
            }
            for (strip, value) in strips.iter().zip(&effective) {
                if strip.starts_with('s') && value.vca_mute.iter().any(|m| *m) {
                    stats[0] += 1;
                }
            }
            for route in &model.routes {
                if route.follows_mute {
                    let source = match &route.source {
                        RouteSource::Track { track_id, .. } => track_id.as_str(),
                        RouteSource::Submix { submix_id, .. } => submix_id.as_str(),
                    };
                    let index = strips.iter().position(|s| s == source).unwrap();
                    if effective[index].vca_mute.iter().any(|m| *m) {
                        stats[1] += 1;
                        if source.starts_with('s') {
                            stats[2] += 1;
                        }
                    }
                }
            }
            if effective.iter().any(|v| v.db.iter().any(|d| *d == -144.0 || *d == 24.0)) {
                stats[3] += 1;
            }
            let actual = render(&model, &feeds);
            assert_planes_equal(&actual, &render(&plain, &feeds), &format!("seed {seed}"));
        },
    );
    eprintln!("ran {ran}; vca-muted submixes {}, vca-muted follow sources {}, of which submix {}, clamped seeds {}", stats[0], stats[1], stats[2], stats[3]);
    assert!(stats.iter().all(|c| *c > 0), "{stats:?}");
}

/// A VCA at -144 (or a sum far below) is a fader at -144 dB, not a mute: the output equals a plain
/// fader at -144 and is not exact zero.
#[test]
fn sol_probe_vca_floor_is_a_fader_minimum_not_a_mute() {
    let mut draw = Draw::new(77);
    let (mut model, source, track) = empty_session();
    let mut s = source.clone();
    s.id = sid("t");
    model.sources.push(s);
    let mut t = track.clone();
    t.id = sid("t");
    t.source_id = sid("t");
    t.fader.left_db = 0.0;
    t.fader.right_db = -100.0;
    model.tracks.push(t);
    let feeds = vec![("t".to_owned(), planes(&mut draw))];
    model.routes = vec![Route {
        id: sid("t-main"),
        source: RouteSource::Track { track_id: sid("t"), tap: SendTap::PostPan },
        destination: RouteDestination::OutputInput { output_id: sid("main-out") },
        channel_matrix: ChannelMatrix { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 },
        gain_db: 24.0,
        mute: false,
        follows_mute: false,
    }];
    let mut plain = model.clone();
    plain.tracks[0].fader.left_db = -144.0;
    plain.tracks[0].fader.right_db = -144.0;
    model.vcas = vec![
        Vca {
            id: sid("a"),
            fader: DualMonoFader { left_db: -144.0, right_db: -144.0, left_mute: false, right_mute: false },
            members: vec![sid("b")],
        },
        Vca {
            id: sid("b"),
            fader: DualMonoFader { left_db: -144.0, right_db: 24.0, left_mute: false, right_mute: false },
            members: vec![sid("t")],
        },
    ];
    // left: 0 - 144 - 144 = -288 -> -144; right: -100 - 144 + 24 = -220 -> -144.
    let out = render(&model, &feeds);
    assert_planes_equal(&out, &render(&plain, &feeds), "floor");
    for plane in &out {
        assert!(plane.iter().any(|s| *s != 0.0), "a VCA floor is not a mute");
    }
}
