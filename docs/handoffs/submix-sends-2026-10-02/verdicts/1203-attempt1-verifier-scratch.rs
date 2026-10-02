// #1203 attempt 1 -- Sol verifier scratch. Not committed anywhere; for the root to adopt or drop.
//
// Append to `crates/host-core/tests/submix_strip.rs` at 6a4d729d9 (it reuses that file's helpers:
// tap_bus_session, bus_as_track, bus_tap, track_tap, TAPS, render, graph_artifact, ...).
// All five `v_*` tests are green on 6a4d729d9 at native W8 (x86-64-v3).
//
// * v_muted_bus_pre_fader_tap_is_ungated (MINOR-2, D4): the only test that pins "a pre-fader tap
//   is not gated by the fader mute" -- for a bus or a track. Red if a muted strip's upstream is
//   elided or zeroed (a plausible skip-on-silence optimisation).
// * v_delayed_bus_input_taps_match_a_delayed_track (NIT-1): the bus delay sits at the `input` tap
//   as a track's does. Cheaper alternative: give `tap_strip()` delay_samples [37, 5].
// * v_bus_to_bus_taps_match_track_to_bus, v_sidechain_from_a_latent_bus_tap_is_compensated,
//   v_tap_cycles_through_sidechains: coverage of bus->bus taps, sidechain PDC from a latent bus
//   tap, and sidechain cycles through bus taps (own insert_return..post_pan refuse; own
//   input/post_input/insert_send prepare).
// * v1203_migration_digest: the cross-commit "no bit moved" probe (PR evidence, not a committed
//   test). Run with V1203_SESSIONS=<a.json>:<b.json> at b0c7e29fb (submix_output spelling) and at
//   6a4d729d9 (submix + post_pan); it prints the graph SHA-256 and an FNV-1a-64 of the PCM bits.
// ---- verifier scratch (#1203 adversarial) ----

fn v_seeds() -> u64 { 6 }

fn v_assert_same(label: &str, a: &[Vec<f32>; 2], b: &[Vec<f32>; 2]) {
    for plane in 0..2 {
        assert_bits_equal(&a[plane], &b[plane], &format!("{label} plane {plane}"));
    }
}

/// S1: a bus delay sits at the strip's Input stage for a bus as for a track: the `input` and
/// `post_input` taps of a delayed bus equal the same taps of a delayed track fed the sum.
#[test]
fn v_delayed_bus_input_taps_match_a_delayed_track() {
    for seed in 0..v_seeds() {
        let mut draw = Draw::new(100 + seed);
        let (mut bus, feeds, sum) = tap_bus_session(&mut draw);
        bus.submixes[0].builtins.left.delay_samples = 37;
        bus.submixes[0].builtins.right.delay_samples = 5;
        let mut oracle = bus_as_track(&bus);
        oracle.tracks[0].builtins.left.delay_samples = 37;
        oracle.tracks[0].builtins.right.delay_samples = 5;
        for tap in [SendTap::Input, SendTap::PostInput, SendTap::PostPan] {
            let mut a = bus.clone();
            a.routes.push(to_output("bus-tap", bus_tap(tap)));
            let mut b = oracle.clone();
            b.routes.push(to_output("bus-tap", track_tap("bus", tap)));
            let actual = render(&document(&a), &borrowed(&feeds), BLOCKS);
            let expected = render(&document(&b), &[("bus", &sum)], BLOCKS);
            v_assert_same(&format!("seed {seed} tap {tap:?}"), &actual, &expected);
            if tap != SendTap::PostPan {
                assert!(actual[0][..37].iter().all(|s| *s == 0.0), "left delayed by 37");
                assert!(actual[0][37..].iter().any(|s| *s != 0.0));
            } else {
                assert!(actual[1][..37].iter().all(|s| *s == 0.0), "swapped: right delayed by 37");
            }
        }
    }
}

/// S2: a muted bus still feeds its pre-fader tap (D4), exactly as a muted track does, and its
/// post-fader tap is silent.
#[test]
fn v_muted_bus_pre_fader_tap_is_ungated() {
    for seed in 0..v_seeds() {
        let mut draw = Draw::new(200 + seed);
        let (mut bus, feeds, sum) = tap_bus_session(&mut draw);
        bus.submixes[0].fader.left_mute = true;
        bus.submixes[0].fader.right_mute = true;
        let mut oracle = bus_as_track(&bus);
        oracle.tracks[0].fader.left_mute = true;
        oracle.tracks[0].fader.right_mute = true;
        for tap in [SendTap::PreFader, SendTap::PostFader, SendTap::PostPan] {
            let mut a = bus.clone();
            a.routes.push(to_output("bus-tap", bus_tap(tap)));
            let mut b = oracle.clone();
            b.routes.push(to_output("bus-tap", track_tap("bus", tap)));
            let actual = render(&document(&a), &borrowed(&feeds), BLOCKS);
            let expected = render(&document(&b), &[("bus", &sum)], BLOCKS);
            v_assert_same(&format!("seed {seed} tap {tap:?}"), &actual, &expected);
            let loud = actual[0].iter().chain(&actual[1]).any(|s| *s != 0.0);
            assert_eq!(loud, tap == SendTap::PreFader, "seed {seed} tap {tap:?}");
        }
    }
}

/// S3: two taps of a bus into a second bus equal the same taps of a track fed the sum into it.
#[test]
fn v_bus_to_bus_taps_match_track_to_bus() {
    for seed in 0..v_seeds() {
        let mut draw = Draw::new(300 + seed);
        let (mut bus, feeds, sum) = tap_bus_session(&mut draw);
        let mut oracle = bus_as_track(&bus);
        let mut second = Strip::transparent(&bus.console);
        second.fader.left_db = -3.0;
        second.fader.right_db = 1.5;
        second.builtins.left.trim_db = 2.0;
        for model in [&mut bus, &mut oracle] {
            model.submixes.push(second.clone().submix("bus2"));
            model.routes.push(to_output("bus2-main", RouteSource::Submix { submix_id: sid("bus2"), tap: SendTap::PostPan }));
        }
        for (id, tap) in [("x-a", SendTap::PreFader), ("x-b", SendTap::InsertReturn), ("x-c", SendTap::PostInput)] {
            bus.routes.push(route(id, bus_tap(tap), into_bus("bus2")));
            oracle.routes.push(route(id, track_tap("bus", tap), into_bus("bus2")));
        }
        let actual = render(&document(&bus), &borrowed(&feeds), BLOCKS);
        let expected = render(&document(&oracle), &[("bus", &sum)], BLOCKS);
        v_assert_same(&format!("seed {seed}"), &actual, &expected);
    }
}

fn v_limiter(id: &str) -> Effect {
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

fn v_keyed_track(model: &mut SessionModel, x_planes: &[Vec<f32>; 2], key: RouteSource) {
    let _ = x_planes;
    let source = model.sources[0].clone();
    let mut track = model.tracks[0].clone();
    Strip::transparent(&model.console).onto(&mut track);
    track.inserts.effects = vec![Effect {
        id: sid("duck"),
        identity: EffectIdentity::Native { effect_id: sid("miso.compressor") },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::Routed(session::Sidechain { source: key, port_id: sid("sidechain-in") }),
    }];
    add_track(model, &source, &track, "x");
    model.routes.push(to_output("x-main", post_pan("x")));
}

/// S4: a sidechain from a bus tap after a latent bus insert is compensated as for a track: bits
/// equal the oracle, and the graph delays are the same.
#[test]
fn v_sidechain_from_a_latent_bus_tap_is_compensated() {
    for seed in 0..v_seeds() {
        let mut draw = Draw::new(400 + seed);
        let (mut bus, mut feeds, sum) = tap_bus_session(&mut draw);
        bus.submixes[0].inserts.effects.push(v_limiter("lim"));
        let x_planes = [
            (0..QUANTUM * BLOCKS).map(|_| draw.noise(0.05)).collect::<Vec<f32>>(),
            (0..QUANTUM * BLOCKS).map(|_| draw.noise(0.05)).collect::<Vec<f32>>(),
        ];
        let mut oracle = bus_as_track(&bus);
        oracle.tracks[0].inserts.effects.push(v_limiter("lim"));
        v_keyed_track(&mut bus, &x_planes, bus_tap(SendTap::InsertReturn));
        bus.routes.push(to_output("bus-main", bus_tap(SendTap::PostPan)));
        v_keyed_track(&mut oracle, &x_planes, track_tap("bus", SendTap::InsertReturn));
        oracle.routes.push(to_output("bus-main", track_tap("bus", SendTap::PostPan)));
        feeds.push(("x".to_owned(), x_planes.clone()));
        let da = document(&bus);
        let db = document(&oracle);
        let ga = graph_artifact(&da, 1);
        let gb = graph_artifact(&db, 1);
        assert_eq!(ga.report().output_latency, gb.report().output_latency);
        assert!(ga.report().output_latency.0 > 0);
        let delays = |g: &PreparedGraphBuiltinsArtifact| {
            let mut v: Vec<(String, u64)> = g.graph().inserted_delays.iter().map(|d| (format!("{:?}", d.edge_id), d.samples.0 as u64)).collect();
            v.sort();
            v
        };
        let (dela, delb) = (delays(&ga), delays(&gb));
        eprintln!("seed {seed}: latency {:?} delays {dela:?}", ga.report().output_latency);
        assert_eq!(dela.iter().map(|d| d.1).collect::<Vec<_>>(), delb.iter().map(|d| d.1).collect::<Vec<_>>());
        let actual = render(&da, &borrowed(&feeds), BLOCKS);
        let expected = render(&db, &[("bus", &sum), ("x", &x_planes)], BLOCKS);
        v_assert_same(&format!("seed {seed}"), &actual, &expected);
    }
}

fn v_refusal(model: &SessionModel) -> Option<String> {
    let doc = document(model);
    let compiled = match compile_host_session(&doc, &caps()) {
        Ok(c) => c,
        Err(f) => return Some(String::from_utf8_lossy(f.as_bytes()).into_owned()),
    };
    match prepare_host_runtime(&compiled, &caps()) {
        Ok(_) => None,
        Err(f) => Some(String::from_utf8_lossy(f.as_bytes()).into_owned()),
    }
}

/// S5: cycles through bus taps by sidechain are refused; a bus keyed from its own input is not a
/// cycle and prepares.
#[test]
fn v_tap_cycles_through_sidechains() {
    let mut draw = Draw::new(500);
    let (base, _, _) = tap_bus_session(&mut draw);
    let keyed_bus = |tap: SendTap| {
        let mut m = base.clone();
        m.routes.push(to_output("bus-main", bus_tap(SendTap::PostPan)));
        m.submixes[0].inserts.effects[0].sidechain = SidechainDeclaration::Routed(session::Sidechain { source: bus_tap(tap), port_id: sid("sidechain-in") });
        m
    };
    for tap in TAPS {
        let refusal = v_refusal(&keyed_bus(tap));
        eprintln!("bus keyed from its own {tap:?}: {refusal:?}");
        let after = matches!(tap, SendTap::InsertReturn | SendTap::PreFader | SendTap::PostFader | SendTap::PostPan);
        assert_eq!(refusal.as_deref().is_some_and(|r| r.contains("graph.cycle")), after, "{tap:?}: {refusal:?}");
        if !after { assert!(refusal.is_none(), "{tap:?}: {refusal:?}"); }
    }
    // A contributor keyed from the bus it feeds, at the bus's input tap.
    let mut m = base.clone();
    m.routes.push(to_output("bus-main", bus_tap(SendTap::PostPan)));
    m.tracks[0].inserts.effects = vec![Effect {
        id: sid("duck"),
        identity: EffectIdentity::Native { effect_id: sid("miso.compressor") },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::Routed(session::Sidechain { source: bus_tap(SendTap::Input), port_id: sid("sidechain-in") }),
    }];
    let refusal = v_refusal(&m);
    eprintln!("contributor keyed from bus input: {refusal:?}");
    assert!(refusal.is_some_and(|r| r.contains("graph.cycle")));
    // Bus A's input tap into B, B into A: a cycle.
    let mut m = base.clone();
    m.submixes.push(Strip::transparent(&m.console).submix("b"));
    m.routes.push(route("ab", bus_tap(SendTap::Input), into_bus("b")));
    m.routes.push(route("ba", RouteSource::Submix { submix_id: sid("b"), tap: SendTap::Input }, into_bus("bus")));
    let refusal = v_refusal(&m);
    eprintln!("input-tap loop: {refusal:?}");
    assert!(refusal.is_some_and(|r| r.contains("graph.cycle")));
    // A self-route from a bus tap into its own input.
    let mut m = base.clone();
    m.routes.push(route("self", bus_tap(SendTap::PreFader), into_bus("bus")));
    let refusal = v_refusal(&m);
    eprintln!("self loop: {refusal:?}");
    assert!(refusal.is_some_and(|r| r.contains("graph.cycle")));
}

// ---- verifier scratch (#1203 migration bit-identity) ----
#[test]
fn v1203_migration_digest() {
    let Ok(paths) = std::env::var("V1203_SESSIONS") else {
        return;
    };
    for path in paths.split(':') {
        let text = std::fs::read_to_string(path).expect("session file");
        let artifact = graph_artifact(&text, 1);
        let sha = GraphCompiler::sha256(artifact.graph(), artifact.report());
        let compiled = compile_host_session(&text, &caps()).unwrap_or_else(|failure| {
            panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
        let mut prepared = prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
        let model = parse_session_json(&text).expect("parse");
        let blocks = 16;
        let mut draw = Draw::new(1203);
        let feeds: Vec<(String, Vec<Vec<f32>>)> = model
            .sources
            .iter()
            .map(|source| {
                (
                    source.id.as_str().to_owned(),
                    (0..source.channels)
                        .map(|_| (0..QUANTUM * blocks).map(|_| draw.noise(0.5)).collect())
                        .collect(),
                )
            })
            .collect();
        let rate = prepared.report.sample_rate_hz;
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        let mut energy = 0.0_f64;
        for block in 0..blocks {
            let range = block * QUANTUM..(block + 1) * QUANTUM;
            for (id, planes) in &feeds {
                let slices: Vec<&[f32]> = planes.iter().map(|p| &p[range.clone()]).collect();
                prepared
                    .sources
                    .submit(
                        id.as_bytes(),
                        SourceSubmission {
                            generation: 1,
                            start_frame: range.start as u64,
                            sample_rate_hz: rate,
                            planes: &slices,
                            frames: QUANTUM as u32,
                            end_of_region: false,
                        },
                    )
                    .expect("source block");
            }
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
            for sample in samples {
                energy += f64::from(sample) * f64::from(sample);
                for byte in sample.to_bits().to_le_bytes() {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
        }
        println!("V1203 {path} graph_sha={sha} pcm_fnv={hash:016x} energy={energy:.6}");
    }
}
