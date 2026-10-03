// #1220 attempt 1 verifier probes (Sol). Append to a copy of crates/graph-compiler/tests/live_routes.rs
// (they reuse its helpers) and run with --features graph/test-support,builtins-compiler/test-support.

// =================================================================================================
// Verifier (Sol) probes for #1220 attempt 1. Not part of the deliverable.
// =================================================================================================

const ALL_TAPS: [SendTap; 7] = [
    SendTap::Input,
    SendTap::PostInput,
    SendTap::InsertSend,
    SendTap::InsertReturn,
    SendTap::PreFader,
    SendTap::PostFader,
    SendTap::PostPan,
];

/// An independent scalar model of one live route's D3/D4 state.
#[derive(Clone, Copy, Debug)]
struct Model {
    start: [f32; 4],
    target: [f32; 4],
    length: u32,
    position: u32,
    mute: bool,
    delayed: bool,
}

impl Model {
    fn settled(coefficients: [f32; 4], mute: bool, delayed: bool) -> Self {
        Self {
            start: coefficients,
            target: coefficients,
            length: 0,
            position: 0,
            mute,
            delayed,
        }
    }
    fn at(&self, k: u32) -> [f32; 4] {
        ramp_at(self.start, self.target, self.length, k)
    }
    fn apply(&mut self, target: [f32; 4], mute: bool, length: u32) {
        let current = self.at(self.position);
        if length == 0 {
            self.start = target;
        } else {
            self.start = current;
        }
        self.target = target;
        self.length = length;
        self.mute = mute;
        self.position = 0;
    }
    fn active(&self) -> bool {
        !(self.mute && self.position >= self.length && !self.delayed)
    }
    /// Mixes one block of `tap` from `first`; `None` when inactive.
    fn block(&mut self, tap: &Planes, first: usize) -> Option<Planes> {
        if !self.active() {
            return None;
        }
        let mut out = [vec![0.0; QUANTUM], vec![0.0; QUANTUM]];
        for f in 0..QUANTUM {
            let c = self.at(self.position + f as u32 + 1);
            let (l, r) = mix_frame(c, tap[0][first + f], tap[1][first + f]);
            out[0][f] = l;
            out[1][f] = r;
        }
        self.position = (self.position + QUANTUM as u32).min(self.length);
        Some(out)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SourceKind {
    Track,
    Bus,
}

/// Track `c` (positive noise) reaches bus `b` through the live route under test, from `tap`:
/// directly (`c-b`) or through bus `y` (`c-y` from `c`'s input, then `y-b` from `y`'s `tap`).
/// When `delayed`, a silent track `a` with a limiter also sends into `b`, so the route under test
/// carries the limiter's compensation.
fn sweep_session(tap: SendTap, source: SourceKind, delayed: bool) -> (SessionModel, &'static str) {
    let (mut model, src, track) = empty_session(48_000);
    if delayed {
        add_track(&mut model, &src, &track, "a");
        model.tracks[0].inserts.effects.push(limiter());
        model
            .routes
            .push(send("a-b", "a", SendTap::PostPan, "b", Values::UNITY));
    }
    add_track(&mut model, &src, &track, "c");
    let id = match source {
        SourceKind::Track => {
            model.routes.push(send("c-b", "c", tap, "b", R0));
            "c-b"
        }
        SourceKind::Bus => {
            model
                .routes
                .push(send("c-y", "c", SendTap::Input, "y", Values::UNITY));
            let mut y_b = route(
                "y-b",
                RouteSource::Submix {
                    submix_id: sid("y"),
                    tap,
                },
                into_bus("b"),
            );
            y_b.gain_db = R0.gain_db;
            y_b.channel_matrix = ChannelMatrix {
                ll: R0.matrix[0],
                lr: R0.matrix[1],
                rl: R0.matrix[2],
                rr: R0.matrix[3],
            };
            model.routes.push(y_b);
            "y-b"
        }
    };
    model.routes.push(bus_to_output("b"));
    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    if source == SourceKind::Bus {
        model
            .submixes
            .push(Submix::unity(sid("y"), &model.console));
    }
    (model, id)
}

/// Every tap, from a track and from a bus, delayed and undelayed: mute (480) before block 0,
/// unmute (480) before block 14, a retarget to `R1` (300) mid-ramp before block 16, and a step
/// mute plus step unmute in one drain before block 20. Every output sample equals an independent
/// scalar model; the settled tail equals a fresh plan.
#[test]
fn verifier_tap_sweep_round_trip() {
    let blocks = 30;
    let frames = blocks * QUANTUM;
    let mut feeds: Feeds = BTreeMap::new();
    feeds.insert("a", [vec![0.0; frames], vec![0.0; frames]]);
    feeds.insert("c", positive_noise(77, frames));
    let tap_signal = feeds["c"].clone();
    for delayed in [true, false] {
        for source in [SourceKind::Track, SourceKind::Bus] {
            for tap in ALL_TAPS {
                let (model, id) = sweep_session(tap, source, delayed);
                let artifact = compile(&model);
                let delay = if delayed {
                    route_delay(&artifact, id)
                } else {
                    assert!(
                        artifact.graph().inserted_delays.is_empty(),
                        "{tap:?} {source:?}: undelayed"
                    );
                    0
                };
                drop(artifact);
                let what = format!("{tap:?} {source:?} delayed={delayed} (d={delay})");
                let mut live = Bound::new(&model, &feeds, true);
                let [chains, slots] = live.plan.bank_shape();
                assert!(chains > 0, "{what}: banked strips ({chains} chains, {slots} slots)");
                let redirects = live.plan.bank_scatter_redirects();
                let mut oracle = Model::settled(R0.coefficients(), false, delay > 0);
                let mut events: BTreeMap<usize, Vec<(Values, u32)>> = BTreeMap::new();
                events.insert(0, vec![(Values { mute: true, ..R0 }, 480)]);
                events.insert(14, vec![(R0, 480)]);
                events.insert(16, vec![(R1, 300)]);
                events.insert(
                    20,
                    vec![(Values { mute: true, ..R1 }, 0), (R1, 0)],
                );
                let mut actual = [Vec::new(), Vec::new()];
                let mut mixed = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
                let mut active_blocks = 0;
                let mut inactive = vec![false; blocks];
                for block in 0..blocks {
                    if let Some(records) = events.get(&block) {
                        for (values, length) in records {
                            live.push(id, values.record(*length));
                            oracle.apply(values.coefficients(), values.mute, *length);
                        }
                    }
                    let out = live.render(1);
                    actual[0].extend_from_slice(&out[0]);
                    actual[1].extend_from_slice(&out[1]);
                    match oracle.block(&tap_signal, block * QUANTUM) {
                        Some(planes) => {
                            active_blocks += 1;
                            mixed[0][block * QUANTUM..(block + 1) * QUANTUM]
                                .copy_from_slice(&planes[0]);
                            mixed[1][block * QUANTUM..(block + 1) * QUANTUM]
                                .copy_from_slice(&planes[1]);
                        }
                        None => inactive[block] = true,
                    }
                }
                let mixes = graph::test_only_route_mix_counts();
                assert_eq!(
                    mixes[route_index(&model, id)],
                    active_blocks,
                    "{what}: route mixes"
                );
                if delayed {
                    assert_eq!(active_blocks, blocks as u64, "{what}: delayed is always active");
                }
                let mut expected = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
                for n in 0..frames {
                    let block = n / QUANTUM;
                    let (l, r) = if delayed {
                        let (dl, dr) = if n < delay {
                            (0.0, 0.0)
                        } else {
                            (mixed[0][n - delay], mixed[1][n - delay])
                        };
                        (0.0 + dl, 0.0 + dr)
                    } else if inactive[block] {
                        (0.0, 0.0)
                    } else {
                        (mixed[0][n], mixed[1][n])
                    };
                    (expected[0][n], expected[1][n]) = unity(l, r);
                }
                assert_planes(&actual, &expected, 0..frames, &what);
                let mut fresh_model = model.clone();
                for route in &mut fresh_model.routes {
                    if route.id.as_str() == id {
                        route.gain_db = R1.gain_db;
                        route.channel_matrix = ChannelMatrix {
                            ll: R1.matrix[0],
                            lr: R1.matrix[1],
                            rl: R1.matrix[2],
                            rr: R1.matrix[3],
                        };
                    }
                }
                let fresh = Bound::new(&fresh_model, &feeds, false).render(blocks);
                assert_planes(&actual, &fresh, 20 * QUANTUM + delay..frames, &format!("{what} fresh"));
                eprintln!("verifier sweep ok: {what}, banks {chains}/{slots}, redirects {redirects}, active {active_blocks}");
            }
        }
    }
}

/// Eight banked tracks into one bus, all live and undelayed: every block a few routes receive a
/// step (mute toggle or new values). Each block equals a fresh plan prepared with that block's
/// values, so activity flips at the store owner and every later position are exact.
#[test]
fn verifier_per_block_steps_match_fresh_plans() {
    let blocks = 18;
    let frames = blocks * QUANTUM;
    let feeds = eight_feeds(frames);
    let mut rng = Rng::new(1_220_999);
    let mut sends = [Values::UNITY; 8];
    for values in &mut sends {
        *values = rng.values(false);
    }
    let model = eight_into_one(&sends);
    let mut live = Bound::new(&model, &feeds, true);
    for block in 0..blocks {
        let changes = 1 + rng.below(4) as usize;
        for _ in 0..changes {
            let index = if block % 3 == 0 { 0 } else { rng.below(8) as usize };
            let mut values = sends[index];
            if rng.chance(500) {
                values.mute = !values.mute;
            } else {
                values = rng.values(values.mute);
            }
            if block == 7 {
                values.mute = true;
            }
            sends[index] = values;
            live.push(&format!("t{index}-bus"), values.record(0));
        }
        if block == 7 {
            for index in 0..8 {
                sends[index].mute = true;
                live.push(&format!("t{index}-bus"), sends[index].record(0));
            }
        }
        let out = live.render(1);
        let fresh = Bound::new(&eight_into_one(&sends), &feeds, false).render(block + 1);
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        for plane in 0..2 {
            assert_bits(
                &out[plane],
                &fresh[plane][range.clone()],
                &format!("block {block}, plane {plane}, sends {sends:?}"),
            );
        }
    }
}

/// Mid-ramp retargets and several records in one drain against the independent model.
#[test]
fn verifier_retarget_and_multi_record_drains() {
    let blocks = 24;
    let frames = blocks * QUANTUM;
    let feeds = two_feeds(frames);
    let model = two_into_bus(R0, R1);
    let mut live = Bound::new(&model, &feeds, true);
    let mut rng = Rng::new(31337);
    let mut oracle = Model::settled(R0.coefficients(), false, false);
    let mut events: BTreeMap<usize, Vec<(Values, u32)>> = BTreeMap::new();
    events.insert(0, vec![(rng.values(false), 4800)]);
    events.insert(3, vec![(rng.values(false), 0), (rng.values(false), 37)]);
    events.insert(4, vec![(rng.values(false), 1000), (rng.values(true), 200)]);
    events.insert(5, vec![(rng.values(false), 1)]);
    events.insert(6, vec![(rng.values(true), 129)]);
    events.insert(9, vec![(rng.values(false), 0), (rng.values(true), 0), (rng.values(false), 77)]);
    events.insert(10, vec![(rng.values(false), 333)]);
    events.insert(11, vec![(rng.values(false), 4000)]);
    events.insert(14, vec![(rng.values(true), 4000)]);
    events.insert(16, vec![(rng.values(false), 127)]);
    events.insert(17, vec![(rng.values(true), 128)]);
    events.insert(19, vec![(rng.values(true), 0)]);
    let open = R1.coefficients();
    let mut actual = [Vec::new(), Vec::new()];
    let mut expected = [Vec::new(), Vec::new()];
    for block in 0..blocks {
        if let Some(records) = events.get(&block) {
            for (values, length) in records {
                live.push("r0", values.record(*length));
                oracle.apply(values.coefficients(), values.mute, *length);
            }
        }
        let out = live.render(1);
        actual[0].extend_from_slice(&out[0]);
        actual[1].extend_from_slice(&out[1]);
        let r0 = oracle.block(&feeds["t0"], block * QUANTUM);
        for f in 0..QUANTUM {
            let n = block * QUANTUM + f;
            let (one_l, one_r) = mix_frame(open, feeds["t1"][0][n], feeds["t1"][1][n]);
            let (zl, zr) = r0.as_ref().map_or((0.0, 0.0), |p| (p[0][f], p[1][f]));
            let (l, r) = unity(zl + one_l, zr + one_r);
            expected[0].push(l);
            expected[1].push(r);
        }
    }
    assert_planes(&actual, &expected, 0..frames, "retargets");
}

/// follows_mute: a fully follow-muted send idles inactive exactly as prepared, takes a gain edit
/// while staying silent, and a half follow-muted send settles on a fresh plan's bits.
#[test]
fn verifier_follows_mute_interplay() {
    let blocks = 12;
    let frames = blocks * QUANTUM;
    let (mut model, src, track) = empty_session(48_000);
    for id in ["m", "h", "n"] {
        add_track(&mut model, &src, &track, id);
    }
    model.tracks[0].fader.left_mute = true;
    model.tracks[0].fader.right_mute = true;
    model.tracks[1].fader.left_mute = true;
    let mut m_b = send("m-b", "m", SendTap::PreFader, "b", R0);
    m_b.follows_mute = true;
    let mut h_b = send("h-b", "h", SendTap::PreFader, "b", R1);
    h_b.follows_mute = true;
    model.routes = vec![m_b, h_b, send("n-b", "n", SendTap::Input, "b", R0), bus_to_output("b")];
    model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    let feeds: Feeds = [("m", noise(1, frames)), ("h", noise(2, frames)), ("n", noise(3, frames))]
        .into_iter()
        .collect();
    let reference = Bound::new(&model, &feeds, false).render(blocks);
    let ref_mixes = graph::test_only_route_mix_counts();
    let mut live = Bound::new(&model, &feeds, true);
    let idle = live.render(2);
    assert_planes(&idle, &[reference[0][..2 * QUANTUM].to_vec(), reference[1][..2 * QUANTUM].to_vec()], 0..2 * QUANTUM, "idle follow");
    assert_eq!(graph::test_only_route_mix_counts()[route_index(&model, "m-b")], 0);
    let _ = ref_mixes;
    let new_m = Values { gain_db: -6.0, matrix: [0.3, 0.2, -0.4, 0.9], mute: false };
    let new_h = Values { gain_db: 3.0, matrix: [0.5, -0.2, 0.1, 0.7], mute: false };
    let m_target = route_coefficients(new_m.gain_db, new_m.matrix, false, [true, true]).unwrap();
    let h_target = route_coefficients(new_h.gain_db, new_h.matrix, false, [true, false]).unwrap();
    live.push("m-b", RouteControlRecord::new(m_target, true, 480).expect("follow-silenced record"));
    live.push("h-b", RouteControlRecord::new(h_target, false, 480).expect("half follow record"));
    let rest = live.render(blocks - 2);
    let mut actual = idle;
    for plane in 0..2 {
        actual[plane].extend_from_slice(&rest[plane]);
    }
    let mut edited = model.clone();
    for route in &mut edited.routes {
        let values = match route.id.as_str() {
            "m-b" => new_m,
            "h-b" => new_h,
            _ => continue,
        };
        route.gain_db = values.gain_db;
        route.channel_matrix = ChannelMatrix { ll: values.matrix[0], lr: values.matrix[1], rl: values.matrix[2], rr: values.matrix[3] };
    }
    let fresh = Bound::new(&edited, &feeds, false).render(blocks);
    assert_planes(&actual, &fresh, 2 * QUANTUM + 480..frames, "follow edits settled");
    let mixes_fresh = graph::test_only_route_mix_counts();
    assert_eq!(mixes_fresh[route_index(&model, "m-b")], 0);
}

/// D8 deviation 1 claims the charge bounds the attached (pre-bind) state too. Long route IDs, a
/// muted route (no activity charge): measure what the attach alone retains.
#[test]
fn verifier_attached_state_charge() {
    let feeds = two_feeds(QUANTUM);
    for (label, long) in [("short", false), ("long", true)] {
        let mut model = two_into_bus(R0, Values { mute: true, ..R1 });
        if long {
            let a = format!("r{}", "a".repeat(126));
            let b = format!("r{}", "b".repeat(126));
            model.routes[0].id = sid(&a);
            model.routes[1].id = sid(&b);
        }
        assert_installed();
        let mut artifact = compile(&model);
        let mark = current_thread_counters();
        let producers = artifact
            .attach_route_live_controls(NonZeroUsize::new(DEPTH).expect("depth"))
            .expect("attach");
        let window = current_thread_delta_since(mark);
        let attached = window.requested_bytes - window.released_bytes;
        let resources = route_control_resources(&producers);
        let mark = current_thread_counters();
        let bindings = bindings(&artifact, &feeds);
        let bound = artifact.into_bound(bindings).unwrap_or_else(|f| panic!("{}", f.code));
        let window = current_thread_delta_since(mark);
        let bind_net = window.requested_bytes as i64 - window.released_bytes as i64;
        drop(bound);
        eprintln!(
            "verifier D8 {label}: LIVE_ROUTE_OWNER_BYTES={LIVE_ROUTE_OWNER_BYTES} lane={} binding={} producer={} attach retained={attached} charged={} bind net={bind_net}",
            core::mem::size_of::<RouteControlLane>(),
            core::mem::size_of::<GraphRouteControlBinding>(),
            core::mem::size_of::<GraphRouteControlProducer>(),
            resources.total_bytes
        );
    }
}

#[test]
fn verifier_gate3_audio_only() {
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

