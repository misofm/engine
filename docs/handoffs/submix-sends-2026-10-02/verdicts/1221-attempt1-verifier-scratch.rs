// #1221 attempt 1 verifier probes (scratch; not for commit). Append to
// crates/host-core/tests/live_routes.rs at 1c5ce5d02; they reuse its helpers. Run with
// `cargo test -p host-core --features host-core/test-support --test live_routes verifier -- --nocapture`.

/// Every per-lane source mute x route mute x initial state (open, or prepared follow-silenced)
/// x u-b silent or not x length {0, 300}: the settled live render equals a fresh plan with
/// `follows_mute` and the source strip's fader lane mutes, from the snap block's end, and the live
/// route mixes in exactly the blocks a fresh plan's would (0 when silenced). Green at head; red under
/// V1 (record mute ignores follow) on the activity check.
#[test]
fn verifier_every_lane_mute_combination_settles_on_a_fresh_plan() {
    let mut draw = Draw::new(77_1221);
    let mut checked = 0;
    for u_muted in [false, true] {
        for initial_follow_silenced in [false, true] {
            for mute in [false, true] {
                for lanes in [[false, false], [true, false], [false, true], [true, true]] {
                    for length in [0_u32, 300] {
                        let initial = values(&mut draw, false);
                        let u = values(&mut draw, u_muted);
                        let drawn = values(&mut draw, mute);
                        let base = if initial_follow_silenced {
                            follow_session(initial, true, [true, true], u)
                        } else {
                            follow_session(initial, false, [false; 2], u)
                        };
                        let push_block = 2;
                        let settled = push_block * QUANTUM + length as usize;
                        let settle_block = settled.div_ceil(QUANTUM);
                        let tail = 4;
                        let blocks = settle_block + tail;
                        let feeds = feeds(&mut draw, &["t", "u"], blocks * QUANTUM);
                        let index = route_index(&base, "t-b");
                        let mut live = Live::new(&base, &feeds, Some(DEPTH));
                        let mut actual = live.render(push_block);
                        live.producer("t-b")
                            .set(drawn.gain_db, drawn.matrix, drawn.mute, lanes, length)
                            .expect("queued");
                        live.render_into(settle_block - push_block, &mut actual);
                        graph::test_only_route_mix_reset();
                        live.render_into(tail, &mut actual);
                        let live_mixes = graph::test_only_route_mix_counts()[index];
                        let edited = follow_session(drawn, true, lanes, u);
                        let mut fresh_host = Live::new(&edited, &feeds, None);
                        let mut fresh = fresh_host.render(settle_block);
                        graph::test_only_route_mix_reset();
                        fresh_host.render_into(tail, &mut fresh);
                        let fresh_counts = graph::test_only_route_mix_counts();
                        let silenced = drawn.mute || lanes == [true, true];
                        let expected_mixes = if silenced { 0 } else { tail as u64 };
                        if !fresh_counts.is_empty() {
                            assert_eq!(fresh_counts[index], expected_mixes, "fresh activity");
                        }
                        let what = format!(
                            "u_muted {u_muted} initial_silenced {initial_follow_silenced} mute {mute} lanes {lanes:?} length {length}"
                        );
                        assert_eq!(live_mixes, expected_mixes, "live activity: {what}");
                        // From `settled` instead, this is red for u_muted && lanes [T,T] && length
                        // 300 (-0.0 != +0.0 at frame 558): #1220's by-design snap-block INFO.
                        assert_planes(&actual, &fresh, settle_block * QUANTUM, &what);
                        checked += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checked, 64);
}

/// Whether D4's named-allocation join is reachable: the builtin strip queues at the same depth are
/// larger than the route queue, so the builtin compile (or the compile caps) refuses first.
#[test]
fn verifier_named_allocation_join_reachability() {
    let mut draw = Draw::new(1_221_006);
    let document = document(&two_into_bus(values(&mut draw, false), values(&mut draw, false)));
    for depth in [8_usize, 1 << 12, 1 << 16] {
        let live = Live::from_document(&document, &Vec::new(), Some(depth));
        let r = live.host.report.route_control_resources;
        let engine_largest = live.host.report.largest_engine_allocation_bytes;
        drop(live);
        let mut limits = caps();
        limits.maximum_named_allocation_bytes = r.largest_allocation_bytes;
        let at = prepare_host_session_with_live_controls(&document, &limits, &request(Some(depth)))
            .map(|_| ())
            .map_err(|f| String::from_utf8_lossy(f.as_bytes()).into_owned());
        println!(
            "depth {depth}: route largest {} engine largest {engine_largest}; at route largest: {at:?}",
            r.largest_allocation_bytes
        );
    }
}
