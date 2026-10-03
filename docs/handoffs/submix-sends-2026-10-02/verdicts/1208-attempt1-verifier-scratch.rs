// Sol's #1208 attempt-1 probes. Not committed anywhere. Append to a copy of
// crates/host-core/tests/strip_meters.rs (as tests/probe_1208.rs) in an export of 60d3289b3.
// Probe C is a one-place edit to strip_s(), shown at the end.

fn with_metrics(strip: &str, tap: MeterTap, metrics: MeterMetricSet) -> HostMeterRequest {
    HostMeterRequest { strip_id: strip.into(), tap, metrics }
}

/// Probe A: gate 1 at metric selections gate 1 does not reach -- SAMPLE_PEAK alone (the
/// browser's #943 block-peak path), COUNTS, ENERGY_RMS, SAMPLE_PEAK|COUNTS and HELD_PEAK.
#[test]
fn probe_a_bus_meter_other_metric_sets() {
    let sets = [
        MeterMetricSet::SAMPLE_PEAK,
        MeterMetricSet::COUNTS,
        MeterMetricSet::ENERGY_RMS,
        MeterMetricSet::from_bits_retain(0b101),
        MeterMetricSet::HELD_PEAK,
    ];
    for metrics in sets {
        for seed in 0..8_u64 {
            let mut draw = Draw::new(seed);
            let console = if seed % 2 == 1 { desk_console() } else { no_console() };
            let pair = pair(&mut draw, &console);
            let bus_meters: Vec<_> = BOUNDARIES.iter().map(|&t| with_metrics("bus", t, metrics)).collect();
            let ref_meters: Vec<_> = BOUNDARIES.iter().map(|&t| with_metrics("ref", t, metrics)).collect();
            let bus_feeds: Vec<(&str, &[Vec<f32>; 2])> =
                pair.bus_feeds.iter().map(|(id, p)| (id.as_str(), p)).collect();
            let actual = metered(&pair.bus, &bus_feeds, &bus_meters);
            let expected = metered(&pair.reference, &[("ref", &pair.sum)], &ref_meters);
            assert_eq!(actual, expected, "metrics {:?} seed {seed}", metrics);
        }
    }
}

/// Probe B: two buses with strip S each (bus2's left trim differs), fed different sums,
/// metered interleaved with a track meter; each bus must equal its own one-track reference.
#[test]
fn probe_b_two_buses_metered_side_by_side() {
    for seed in 0..8_u64 {
        let mut draw = Draw::new(100 + seed);
        let console = if seed % 2 == 1 { desk_console() } else { no_console() };
        let rate = draw.pick(&LAUNCH_RATES);
        let frames = QUANTUM * BLOCKS;
        let (mut a, source, track) = empty_session(rate, console.clone());
        let mut feeds = Vec::new();
        for index in 0..3 {
            let name = format!("t{index}");
            add_track(&mut a, &source, &track, &name);
            feeds.push((name, noise_planes(&mut draw, frames)));
        }
        let plan: [(&str, &str, &str); 5] = [
            ("r-a", "t0", "bus"), ("r-b", "t1", "bus"), ("r-c", "t2", "bus"),
            ("s-a", "t0", "bus2"), ("s-b", "t2", "bus2"),
        ];
        let mut contributions: Vec<(Route, String)> = Vec::new();
        for (rid, src, dst) in plan {
            let route = Route {
                id: sid(rid),
                source: post_pan_of_track(src),
                destination: RouteDestination::SubmixInput { submix_id: sid(dst) },
                channel_matrix: ChannelMatrix {
                    ll: coefficient(&mut draw), lr: coefficient(&mut draw),
                    rl: coefficient(&mut draw), rr: coefficient(&mut draw),
                },
                gain_db: draw.in_domain(-12.0, 6.0),
            };
            a.routes.push(route.clone());
            contributions.push((route, src.to_owned()));
        }
        let s1 = strip_s(&console);
        let mut s2 = strip_s(&console);
        s2.id = sid("bus2");
        s2.builtins.left.trim_db = -1.0;
        a.submixes = vec![s1.clone(), s2.clone()];
        a.routes.push(to_output("bus-main", post_pan_of_bus("bus")));
        a.routes.push(to_output("bus2-main", post_pan_of_bus("bus2")));
        let sum_for = |dst: &str| {
            let mut rows: Vec<&(Route, String)> = contributions.iter().filter(|(r, _)| matches!(&r.destination, RouteDestination::SubmixInput{submix_id} if submix_id.as_str()==dst)).collect();
            rows.sort_by(|x, y| x.0.id.cmp(&y.0.id));
            let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
            for (rank, (route, src)) in rows.iter().enumerate() {
                let planes = &feeds.iter().find(|(n, _)| n == src).unwrap().1;
                let gain = math::db_to_gain_f32(route.gain_db);
                let m = &route.channel_matrix;
                let (ll, lr, rl, rr) = (gain * m.ll, gain * m.lr, gain * m.rl, gain * m.rr);
                for f in 0..frames {
                    let (l, r) = (planes[0][f], planes[1][f]);
                    let left = (lr * r) + (ll * l);
                    let right = (rr * r) + (rl * l);
                    if rank == 0 { sum[0][f] = left; sum[1][f] = right; } else { sum[0][f] += left; sum[1][f] += right; }
                }
            }
            sum
        };
        let sum1 = sum_for("bus");
        let sum2 = sum_for("bus2");
        let a_doc = canonical_session_json(&a).unwrap();
        let reference = |s: &Submix| {
            let (mut b, source, mut track) = empty_session(rate, console.clone());
            track.builtins = s.builtins.clone();
            track.console = s.console.clone();
            track.inserts = s.inserts.clone();
            track.fader = s.fader.clone();
            track.matrix_or_pan = s.matrix_or_pan.clone();
            add_track(&mut b, &source, &track, "ref");
            b.routes.push(to_output("ref-main", post_pan_of_track("ref")));
            canonical_session_json(&b).unwrap()
        };
        let (r1, r2) = (reference(&s1), reference(&s2));
        let mut sel = Vec::new();
        for &tap in &BOUNDARIES {
            sel.push(meter("bus2", tap));
            sel.push(meter("t1", tap));
            sel.push(meter("bus", tap));
        }
        let fa: Vec<(&str, &[Vec<f32>; 2])> = feeds.iter().map(|(n, p)| (n.as_str(), p)).collect();
        let actual = metered(&a_doc, &fa, &sel);
        let ref_sel: Vec<_> = BOUNDARIES.iter().map(|&t| meter("ref", t)).collect();
        let e1 = metered(&r1, &[("ref", &sum1)], &ref_sel);
        let e2 = metered(&r2, &[("ref", &sum2)], &ref_sel);
        for (ti, _) in BOUNDARIES.iter().enumerate() {
            for w in 0..BLOCKS {
                // every word but the handle (word 0)
                assert_eq!(actual[ti * 3][w][1..], e2[ti][w][1..], "seed {seed} bus2 tap {ti} win {w}");
                assert_eq!(actual[ti * 3 + 2][w][1..], e1[ti][w][1..], "seed {seed} bus tap {ti} win {w}");
            }
        }
    }
}

// Probe C: in strip_s(), after the two trim_db lines, add
//     strip.builtins.left.delay_samples = 37;
//     strip.builtins.right.delay_samples = 5;
//     strip.builtins.left.hpf_hz = 120.0;
//     strip.builtins.right.lpf_hz = 9_000.0;
//     strip.builtins.right.polarity_invert = true;
// and run the four committed tests unchanged (gate 1 on 16 seeds, gate 4 with a delayed bus).
