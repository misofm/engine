// Sol verifier probes for #1217 attempt 1 (scratch; appended to crates/host-core/tests/route_mute.rs at 071c6c14a).
// Also: render() and the gate-5 pcm buffer were pre-filled with 7.0 instead of 0.0 (sentinel host planes).

// ==== Sol adversarial probes (verifier scratch; not part of the PR) ===========================

fn adv_session(console: Console) -> (SessionModel, Source, session::Track) {
    let (mut model, source, mut track) = empty_session();
    model.console = console;
    let unity = Submix::unity(sid("unused"), &model.console);
    track.console = unity.console;
    (model, source, track)
}

fn desk() -> Console {
    let slot = |id: &str| session::ConsoleSlot {
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

fn negative_planes(draw: &mut Draw) -> Planes {
    let mut plane = || {
        (0..FRAMES)
            .map(|_| -(0.1 + 0.8 * draw.unit()))
            .collect::<Vec<f32>>()
    };
    [plane(), plane()]
}

fn tap(track: &str, tap: SendTap) -> RouteSource {
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

/// Every contributor muted (>= 2, undelayed), into a bus and into the output: exact +0.0, and no
/// muted route mixes. Negative feeds make a zero-coefficient mix -0.0, so a mixed muted route shows.
#[test]
fn adv_every_contributor_muted_is_positive_zero() {
    for console in [Console { pre_insert: Vec::new(), post_insert: Vec::new() }, desk()] {
        for into_output in [false, true] {
            for n in [1usize, 2, 3, 9] {
                let (mut model, source, track) = adv_session(console.clone());
                let mut draw = Draw::new(77 + n as u64);
                let mut feeds = Vec::new();
                for index in 0..n {
                    let id = format!("t{index}");
                    add_track(&mut model, &source, &track, &id);
                    let destination = if into_output {
                        RouteDestination::OutputInput { output_id: sid("main-out") }
                    } else {
                        into_bus("b")
                    };
                    // input tap: never in place, so the route owns a coloured buffer.
                    let mut r = route(&format!("r{index}"), input_tap(&id), destination);
                    r.mute = true;
                    model.routes.push(r);
                    feeds.push((id, negative_planes(&mut draw)));
                }
                if !into_output {
                    model.routes.push(to_output("z-main", bus_input("b")));
                    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
                }
                let (actual, mixes) = render_counted(&document(&model), &feeds);
                let what = format!("console {} output {into_output} n {n}", !console.pre_insert.is_empty());
                assert_all_bits(&actual, 0.0, &what);
                for index in 0..n {
                    assert_eq!(mixes[route_index(&model, &format!("r{index}"))], 0, "{what}: r{index} mixed");
                }
            }
        }
    }
}

/// Muted routes from mid-chain taps, bus->bus, banked strips, first and later in route-ID order:
/// the render equals the same session with the muted routes deleted (noise feeds, so D3's +0.0
/// store and "first active stores" agree except at exact-zero sums, which noise does not reach).
#[test]
fn adv_route_shapes_match_the_session_without_the_muted_routes() {
    for console in [Console { pre_insert: Vec::new(), post_insert: Vec::new() }, desk()] {
        for seed in 0..6u64 {
            let mut draw = Draw::new(9_000 + seed);
            let (mut model, source, track) = adv_session(console.clone());
            let mut feeds = Vec::new();
            for index in 0..5 {
                let id = format!("t{index}");
                add_track(&mut model, &source, &track, &id);
                feeds.push((id, planes(&mut draw, None)));
            }
            let taps = [SendTap::PreFader, SendTap::InsertSend, SendTap::PostPan, SendTap::PostFader, SendTap::PostInput];
            let mut all = vec![
                route("a0", tap("t0", taps[(seed as usize) % 5]), into_bus("b")),
                route("a1", tap("t1", taps[(seed as usize + 1) % 5]), into_bus("b")),
                route("a2", tap("t2", SendTap::PostPan), into_bus("b")),
                route("c0", bus_tap("b", SendTap::PreFader), into_bus("c")),
                route("c1", tap("t3", SendTap::PostFader), into_bus("c")),
                route("c2", bus_tap("b", SendTap::InsertReturn), into_bus("c")),
                to_output("m0", bus_tap("b", SendTap::PostPan)),
                to_output("m1", bus_tap("c", SendTap::PostPan)),
                to_output("m2", tap("t4", SendTap::PostPan)),
                to_output("m3", tap("t0", SendTap::PostPan)),
            ];
            for r in &mut all {
                r.gain_db = draw.in_domain(-12.0, 6.0);
            }
            // Mute a drawn subset, never every route into one destination.
            let muted: Vec<bool> = (0..all.len()).map(|_| draw.chance(1, 3)).collect();
            let groups = [[0usize, 1, 2].as_slice(), &[3, 4, 5], &[6, 7, 8, 9]];
            let mut muted = muted;
            for g in groups {
                if g.iter().all(|i| muted[*i]) {
                    muted[g[g.len() - 1]] = false;
                }
            }
            for (r, m) in all.iter_mut().zip(&muted) {
                r.mute = *m;
            }
            model.routes = all.clone();
            model.submixes = vec![
                Submix::unity(sid("b"), &model.console),
                Submix::unity(sid("c"), &model.console),
            ];
            let (actual, mixes) = render_counted(&document(&model), &feeds);
            let mut reference = model.clone();
            reference.routes.retain(|r| !r.mute);
            let (expected, _) = render(&document(&reference), &feeds);
            let what = format!("console {} seed {seed} muted {muted:?}", !console.pre_insert.is_empty());
            for plane in 0..2 {
                assert_bits_equal(&actual[plane], &expected[plane], &format!("{what}, plane {plane}"));
            }
            assert!(actual.iter().flatten().any(|s| *s != 0.0), "{what}: audio");
            for (r, m) in all.iter().zip(&muted) {
                let count = mixes[route_index(&model, r.id.as_str())];
                assert_eq!(count, if *m { 0 } else { BLOCKS as u64 }, "{what}: {} mixes", r.id.as_str());
            }
        }
    }
}

/// A muted delayed route fed +inf at known frames carries NaN through its zero coefficients and its
/// 486-sample line: NaN must appear at exactly frame + 486 at the bus input tap (and so the output),
/// and nowhere else. This observes the line content directly, not a counter.
#[test]
fn adv_muted_delayed_route_carries_its_zero_mix_through_the_line() {
    let model = delayed_session_checked();
    let mut draw = Draw::new(4_242);
    let mut d = planes(&mut draw, None);
    let spikes = [3usize, 130, 500, 700];
    for f in spikes {
        d[0][f] = f32::INFINITY;
        d[1][f] = f32::INFINITY;
    }
    let s = planes(&mut draw, None);
    let feeds = vec![("d".to_owned(), d), ("s".to_owned(), s)];
    let (actual, mixes) = render_counted(&document(&model), &feeds);
    for plane in 0..2 {
        let nan: Vec<usize> = actual[plane]
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_nan())
            .map(|(i, _)| i)
            .collect();
        let want: Vec<usize> = spikes.iter().map(|f| f + LIMITER_LATENCY).filter(|f| *f < FRAMES).collect();
        assert_eq!(nan, want, "plane {plane}: NaN frames");
    }
    assert_eq!(mixes[route_index(&model, "d-e")], BLOCKS as u64);
}

/// Signed-zero view of the line: `d` all negative, so its zero-coefficient mix is -0.0; `d-e` is
/// first in route-ID order and `s-e` (undelayed, late) is muted too, so it is inactive. The bus
/// input is then exactly d's delayed zero mix: +0.0 for the line's first 486 frames, -0.0 after.
#[test]
fn adv_muted_delayed_route_zero_sign_follows_the_line() {
    let mut model = delayed_session_checked();
    for r in &mut model.routes {
        if r.id.as_str() == "s-e" {
            r.mute = true;
        }
    }
    let mut draw = Draw::new(4_343);
    let d = negative_planes(&mut draw);
    let s = planes(&mut draw, None);
    let feeds = vec![("d".to_owned(), d), ("s".to_owned(), s)];
    let (actual, mixes) = render_counted(&document(&model), &feeds);
    for plane in 0..2 {
        for f in 0..FRAMES {
            let want = if f < LIMITER_LATENCY { 0.0f32 } else { -0.0f32 };
            assert_eq!(actual[plane][f].to_bits(), want.to_bits(), "plane {plane} frame {f}");
        }
    }
    assert_eq!(mixes[route_index(&model, "d-e")], BLOCKS as u64);
    assert_eq!(mixes[route_index(&model, "s-e")], 0);
}

/// Delayed muted routes in other shapes stay active (counter) and render the session with the
/// muted route at zero gain... i.e. the reference without the route, where contributions are noise.
#[test]
fn adv_delayed_muted_route_shapes_stay_active() {
    for shape in 0..3 {
        let (mut model, source, track) = empty_session();
        for id in ["t0", "t1", "t2"] {
            add_track(&mut model, &source, &track, id);
        }
        let limiter = Effect {
            id: sid("ceiling"),
            identity: EffectIdentity::Native { effect_id: sid("miso.true-peak-limiter") },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::Maximum,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        };
        let mut x = Submix::unity(sid("x"), &model.console);
        x.inserts.effects.push(limiter);
        let y = Submix::unity(sid("y"), &model.console);
        model.submixes = vec![x, y];
        let muted_id;
        match shape {
            // into the output (host form), delayed, first in route-ID order
            0 => {
                model.routes.push(route("t0-x", post_pan("t0"), into_bus("x")));
                model.routes.push(to_output("a-direct", tap("t1", SendTap::PreFader)));
                model.routes.push(to_output("x-main", bus_output("x")));
                model.routes.push(to_output("y-main", bus_output("y")));
                model.routes.push(route("t2-y", post_pan("t2"), into_bus("y")));
                muted_id = "a-direct";
            }
            // bus -> bus delayed: y sums x (latent) and b-y from bus... t2's mid-chain tap
            1 => {
                model.routes.push(route("t0-x", post_pan("t0"), into_bus("x")));
                model.routes.push(route("x-y", bus_output("x"), into_bus("y")));
                model.routes.push(route("a-t2-y", tap("t2", SendTap::InsertReturn), into_bus("y")));
                model.routes.push(to_output("y-main", bus_output("y")));
                model.routes.push(to_output("t1-main", post_pan("t1")));
                muted_id = "a-t2-y";
            }
            // a bus's own tap delayed into another bus
            _ => {
                model.routes.push(route("t0-x", post_pan("t0"), into_bus("x")));
                model.routes.push(route("t1-x", post_pan("t1"), into_bus("x")));
                model.routes.push(route("z-x-y", bus_output("x"), into_bus("y")));
                model.routes.push(route("a-t2-y", post_pan("t2"), into_bus("y")));
                model.routes.push(to_output("y-main", bus_output("y")));
                muted_id = "a-t2-y";
            }
        }
        for r in &mut model.routes {
            r.mute = r.id.as_str() == muted_id;
        }
        let artifact = graph_artifact(&document(&model));
        let delayed = artifact.graph().inserted_delays.iter().any(|d| matches!(&d.edge_id,
            GraphEdgeId::RouteDestination { route_id } if route_id.as_str() == muted_id));
        assert!(delayed, "shape {shape}: {muted_id} is delayed: {:?}", artifact.graph().inserted_delays);
        let mut draw = Draw::new(55 + shape as u64);
        let feeds: Feeds = ["t0", "t1", "t2"].iter().map(|id| ((*id).to_owned(), planes(&mut draw, None))).collect();
        let (actual, mixes) = render_counted(&document(&model), &feeds);
        assert_eq!(mixes[route_index(&model, muted_id)], BLOCKS as u64, "shape {shape}: delayed muted route is active");
        let mut reference = model.clone();
        for r in &mut reference.routes {
            if r.id.as_str() == muted_id {
                r.gain_db = -120.0; // not exactly zero; only checked for latency sanity below
            }
        }
        // compare with the open session: identical latency and every nonzero contribution path
        let mut open = model.clone();
        open.routes.retain(|r| r.id.as_str() != muted_id);
        let (expected, _) = render(&document(&open), &feeds);
        // removing the delayed route does not change PDC of the others (it is the early arrival)
        for plane in 0..2 {
            for f in 0..FRAMES {
                if expected[plane][f] != 0.0 {
                    assert_eq!(actual[plane][f].to_bits(), expected[plane][f].to_bits(), "shape {shape} plane {plane} frame {f}");
                }
            }
        }
        let _ = reference;
    }
}

/// A bus with `delay_samples` (a `SumDelay` destination): first route muted and every route muted.
#[test]
fn adv_sum_delay_bus_destination() {
    for all in [false, true] {
        let (mut model, source, track) = empty_session();
        let mut draw = Draw::new(31);
        let mut feeds = Vec::new();
        for index in 0..3 {
            let id = format!("t{index}");
            add_track(&mut model, &source, &track, &id);
            let mut r = route(&format!("r{index}"), input_tap(&id), into_bus("b"));
            r.mute = index == 0 || all;
            model.routes.push(r);
            feeds.push((id, negative_planes(&mut draw)));
        }
        model.routes.push(to_output("z-main", bus_tap("b", SendTap::PostPan)));
        let mut bus = Submix::unity(sid("b"), &model.console);
        bus.builtins.left.delay_samples = 37;
        bus.builtins.right.delay_samples = 37;
        model.submixes = vec![bus];
        let (actual, mixes) = render_counted(&document(&model), &feeds);
        let mut reference = model.clone();
        reference.routes.retain(|r| !r.mute);
        if all {
            assert_all_bits(&actual, 0.0, "every route muted into a delayed bus");
        } else {
            let (expected, _) = render(&document(&reference), &feeds);
            for plane in 0..2 {
                assert_bits_equal(&actual[plane], &expected[plane], &format!("plane {plane}"));
            }
            assert!(actual.iter().flatten().any(|s| *s != 0.0));
        }
        assert_eq!(mixes[route_index(&model, "r0")], 0);
    }
}
