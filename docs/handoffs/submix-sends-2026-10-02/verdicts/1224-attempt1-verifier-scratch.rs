// ---- Sol adversarial tests (verdict scratch, never committed) ----

/// Compare two outputs bit for bit, returning the first differing sample.
fn sol_diff(a: &[f32], b: &[f32]) -> Option<(usize, f32, f32)> {
    a.iter()
        .zip(b)
        .enumerate()
        .find(|(_, (x, y))| x.to_bits() != y.to_bits())
        .map(|(i, (x, y))| (i, *x, *y))
}

/// A: a send muted by its own switch whose source mute moves gets a follow record whose
/// coefficients do not change. Does it move bits against (1) a twin without follow and (2) a
/// host booted with the source muted?
#[test]
fn sol_a_muted_send_follow_record_is_bit_neutral() {
    let make = |follows: bool, drums_muted: bool| {
        let document = follow_document(
            &["drums"],
            &["verb"],
            &[
                (
                    "drums-verb",
                    "drums",
                    false,
                    session::SendTap::PreFader,
                    Some("verb"),
                    [0.8, -0.3, 0.25, 0.6],
                    follows,
                ),
                follow_main("verb-main", "verb", true),
            ],
            if drums_muted { &[("drums", [true; 2])] } else { &[] },
            None,
        );
        let mut model = parse_session_json(&document).expect("model");
        for route in &mut model.routes {
            if route.id.as_str() == "drums-verb" {
                route.mute = true;
            }
        }
        let document = canonical_session_json(&model).expect("canon");
        strip_boot(&document, strip_options(16, 0, 0))
    };
    let mut follow = make(true, false);
    let mut twin = make(false, false);
    let mut booted = make(true, true);
    for block in 0..2 {
        follow_render(&mut follow, &["drums"], block, None);
        follow_render(&mut twin, &["drums"], block, None);
        follow_render(&mut booted, &["drums"], block, None);
    }
    stage_lane_mute(&mut follow, 0, 0, 2, true, 480);
    stage_lane_mute(&mut twin, 0, 0, 2, true, 480);
    assert_eq!(follow.submit_commands(1), RESULT_OK);
    assert_eq!(twin.submit_commands(1), RESULT_OK);
    let pushed = 16 - send_queue_room(&follow)[0];
    let mut diffs = Vec::new();
    let mut negzero = 0;
    for block in 2..10 {
        let a = follow_render(&mut follow, &["drums"], block, None);
        let b = follow_render(&mut twin, &["drums"], block, None);
        let c = follow_render(&mut booted, &["drums"], block, None);
        negzero += a.iter().filter(|x| x.to_bits() == (-0.0_f32).to_bits()).count();
        if let Some(d) = sol_diff(&a, &b) {
            diffs.push(format!("follow vs twin block {block}: {d:?}"));
        }
        if let Some(d) = sol_diff(&a, &c) {
            diffs.push(format!("follow vs booted block {block}: {d:?}"));
        }
    }
    eprintln!("SOL-A pushed follow records on a muted send: {pushed}; -0.0 samples in follow output: {negzero}");
    assert!(diffs.is_empty(), "SOL-A: {diffs:#?}");
}

/// B: unmute while soloed elsewhere, a solo switch in one batch, and a send muted by the user
/// and by follow; every settled state against a freshly booted host.
#[test]
fn sol_b_solo_switch_unmute_under_solo_and_double_muted_send() {
    // strips: bass 0, drums 1, vocal 2, room 3, verb 4; sends: bass-room 0, drums-verb 1.
    let tracks = &SOLO_FOLLOW_TRACKS;
    let mut live = solo_follow_host(&[], 16);
    let mut block = 0_u64;
    let settle_and_compare =
        |live: &mut AudioWorkletEngineHost, block: &mut u64, muted: &[(&str, [bool; 2])], what: &str| {
            for _ in 0..5 {
                follow_render(live, tracks, *block, None);
                *block += 1;
            }
            let mut booted = solo_follow_host(muted, 16);
            for b in 0..*block {
                follow_render(&mut booted, tracks, b, None);
            }
            for _ in 0..2 {
                let a = follow_render(live, tracks, *block, None);
                let c = follow_render(&mut booted, tracks, *block, None);
                assert!(sol_diff(&a, &c).is_none(), "{what}: block {block} {:?}", sol_diff(&a, &c));
                assert!(a.iter().any(|x| *x != 0.0), "{what}: audible");
                *block += 1;
            }
        };
    // 1. mute drums.
    stage_mute(&mut live, 0, 1, true, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[false; 2], [true; 2]]);
    settle_and_compare(&mut live, &mut block, &[("drums", [true; 2])], "drums muted");
    // 2. solo vocal: bass-room follows, drums-verb no record.
    let room = send_queue_room(&live);
    stage_solo(&mut live, 0, 2, true, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[true; 2], [true; 2]]);
    assert_eq!(send_queue_room(&live), [room[0] - 1, room[1]]);
    settle_and_compare(&mut live, &mut block, &[("drums", [true; 2]), ("bass", [true; 2])], "solo vocal");
    // 3. unmute drums while vocal is soloed: still solo-muted, no record.
    let room = send_queue_room(&live);
    stage_mute(&mut live, 0, 1, false, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[true; 2], [true; 2]]);
    assert_eq!(send_queue_room(&live), room, "no record on unmute under solo");
    settle_and_compare(&mut live, &mut block, &[("drums", [true; 2]), ("bass", [true; 2])], "unmute under solo");
    // 4. solo switch in one batch: vocal off, drums on.
    stage_solo(&mut live, 0, 2, false, 480);
    stage_solo(&mut live, 1, 1, true, 480);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[true; 2], [false; 2]]);
    settle_and_compare(&mut live, &mut block, &[("bass", [true; 2]), ("vocal", [true; 2])], "solo switch");
    // 5. user mutes drums-verb, then switch solo back to vocal: follow on a muted send.
    stage_send(&mut live, 0, COMMAND_ROUTE_MUTE, 1, 480, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_solo(&mut live, 0, 1, false, 480);
    stage_solo(&mut live, 1, 2, true, 480);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[true; 2], [true; 2]]);
    // 6. user unmutes drums-verb: still follow-silenced.
    stage_send(&mut live, 0, COMMAND_ROUTE_MUTE, 1, 480, [0.0; 4]);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    settle_and_compare(&mut live, &mut block, &[("drums", [true; 2]), ("bass", [true; 2])], "double muted then route unmuted");
    // 7. unsolo: everything reopens.
    stage_solo(&mut live, 0, 2, false, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[false; 2], [false; 2]]);
    settle_and_compare(&mut live, &mut block, &[], "all open");
}

/// C: randomized differential. Random batches of kind 4 (any strip, any channel) and kind 9
/// (any track); after each, the mirror equals the effective mutes and, once settled, the output
/// equals a freshly booted host whose session mutes are those effective mutes.
#[test]
fn sol_c_randomized_follow_matches_a_fresh_plan() {
    // strips: bass 0, drums 1, vocal 2, room 3, verb 4. Add bus sends: room->verb follows.
    let routes: [FollowRoute<'static>; 9] = [
        ("drums-verb", "drums", false, session::SendTap::PreFader, Some("verb"), [0.8, -0.3, 0.25, 0.6], true),
        ("bass-room", "bass", false, session::SendTap::PostFader, Some("room"), [-0.5, 0.7, 0.9, -0.2], true),
        ("vocal-room", "vocal", false, session::SendTap::PreFader, Some("room"), [0.3, 0.2, -0.4, 0.7], true),
        ("room-verb", "room", true, session::SendTap::PreFader, Some("verb"), [0.6, -0.1, 0.2, 0.5], true),
        ("drums-room", "drums", false, session::SendTap::Input, Some("room"), [0.2, 0.3, 0.1, -0.6], false),
        follow_main("drums-main", "drums", false),
        follow_main("vocal-main", "vocal", false),
        follow_main("verb-main", "verb", true),
        follow_main("room-main", "room", true),
    ];
    let tracks = &SOLO_FOLLOW_TRACKS;
    let names = ["bass", "drums", "vocal", "room", "verb"];
    let boot = |muted: &[(&str, [bool; 2])]| {
        strip_boot(
            &follow_document(tracks, &["verb", "room"], &routes, muted, None),
            strip_options(64, 0, 0),
        )
    };
    for seed in 0..12_u64 {
    let mut live = boot(&[]);
    let mut draw = SendDraw(0x1224_5017 ^ seed.wrapping_mul(0x9E37));
    let mut block = 0_u64;
    let mut nontrivial = 0;
    for step in 0..25 {
        let records = 1 + (draw.next() % 3) as usize;
        for index in 0..records {
            if draw.next() % 3 == 0 {
                let track = (draw.next() % 3) as u32;
                stage_solo(&mut live, index, track, draw.next() % 2 == 0, [0, 64, 480][(draw.next() % 3) as usize]);
            } else {
                let strip = (draw.next() % 5) as u32;
                let channel = (draw.next() % 3) as u8;
                stage_lane_mute(&mut live, index, strip, channel, draw.next() % 2 == 0, [0, 64, 480][(draw.next() % 3) as usize]);
            }
        }
        assert_eq!(live.submit_commands(records as u32), RESULT_OK, "step {step}");
        let ready = live.ready.as_ref().expect("ready");
        let mut effective = Vec::new();
        for strip in 0..5 {
            effective.push([ready.solo.effective_mute(strip, 0), ready.solo.effective_mute(strip, 1)]);
        }
        for route in 0..ready.routes.len() {
            let entry = ready.routes.get(route).expect("route");
            let want = if entry.follows_mute { effective[entry.source_strip] } else { [false; 2] };
            assert_eq!(entry.source_lane_muted, want, "step {step} route {route}");
        }
        for _ in 0..4 {
            follow_render(&mut live, tracks, block, None);
            block += 1;
        }
        let muted: Vec<(&str, [bool; 2])> = names.iter().zip(&effective).map(|(n, m)| (*n, *m)).collect();
        let mut booted = boot(&muted);
        for b in 0..block {
            follow_render(&mut booted, tracks, b, None);
        }
        let a = follow_render(&mut live, tracks, block, None);
        let c = follow_render(&mut booted, tracks, block, None);
        block += 1;
        assert!(sol_diff(&a, &c).is_none(), "seed {seed} step {step} effective {effective:?}: {:?}", sol_diff(&a, &c));
        if effective.iter().any(|m| m[0] != m[1]) && effective.iter().any(|m| m[0] || m[1]) { nontrivial += 1; }
    }
    eprintln!("SOL-C seed {seed}: {nontrivial} steps with asymmetric mutes");
    }
}

/// D: an overlong solo smoothing on the second solo record: which wire index is refused?
#[test]
fn sol_d_overlong_solo_refusal_index() {
    let mut host = solo_follow_host(&[], 16);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    stage_solo(&mut host, 0, 0, true, 100);
    stage_solo(&mut host, 1, 2, true, (1 << 22) + 1);
    let result = host.submit_commands(2);
    let report = *host.command_report();
    eprintln!("SOL-D result {result} reason {} index {}", report.reason, report.rejected_index);
}

/// P1: a follow record carries the mirror's live gain, matrix and mute (D1), not the prepared
/// route's. Live edits by kinds 13-15, then one-lane source mutes, against a booted host whose
/// session holds the edited values and the mutes.
#[test]
fn sol_p1_follow_records_carry_the_mirrors_live_values() {
    // strips: bass 0, drums 1, vocal 2, room 3, verb 4; sends: bass-room 0, drums-verb 1.
    let edited = [0.45_f32, -0.65, 0.15, 0.9];
    let boot = |muted: &[(&str, [bool; 2])], values: bool| {
        let document = follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &SOLO_FOLLOW_ROUTES, muted, None);
        let mut model = parse_session_json(&document).expect("model");
        if values {
            for route in &mut model.routes {
                match route.id.as_str() {
                    "drums-verb" => {
                        route.gain_db = -6.0;
                        route.channel_matrix = session::ChannelMatrix { ll: edited[0], lr: edited[1], rl: edited[2], rr: edited[3] };
                    }
                    "bass-room" => route.mute = true,
                    _ => {}
                }
            }
        }
        strip_boot(&canonical_session_json(&model).expect("canon"), strip_options(16, 0, 0))
    };
    let mut live = boot(&[], false);
    stage_send(&mut live, 0, COMMAND_ROUTE_GAIN_DB, 1, 0, [-6.0, 0.0, 0.0, 0.0]);
    stage_send(&mut live, 1, COMMAND_ROUTE_MATRIX, 1, 0, edited);
    stage_send(&mut live, 2, COMMAND_ROUTE_MUTE, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(live.submit_commands(3), RESULT_OK);
    // drums left only; bass right only (bass-room is user-muted: the right lane must not reopen it).
    stage_lane_mute(&mut live, 0, 1, 0, true, 480);
    stage_lane_mute(&mut live, 1, 0, 1, true, 480);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[false, true], [true, false]]);
    let mut booted = boot(&[("drums", [true, false]), ("bass", [false, true])], true);
    for block in 0..5 {
        follow_render(&mut live, &SOLO_FOLLOW_TRACKS, block, None);
        follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
    }
    for block in 5..8 {
        let a = follow_render(&mut live, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
        assert!(sol_diff(&a, &c).is_none(), "block {block}: {:?}", sol_diff(&a, &c));
    }
}

/// P2: two strips muted in one batch with different windows: each send ramps with its own
/// strip's window, as an explicit `routeMute` at that window does.
#[test]
fn sol_p2_each_follow_takes_its_own_strips_window() {
    let boot = |follows: bool| {
        let routes: Vec<FollowRoute<'static>> = SOLO_FOLLOW_ROUTES
            .iter()
            .map(|r| (r.0, r.1, r.2, r.3, r.4, r.5, r.6 && follows))
            .collect();
        strip_boot(
            &follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &routes, &[], None),
            strip_options(16, 0, 0),
        )
    };
    let mut follow = boot(true);
    let mut explicit = boot(false);
    for host in [&mut follow, &mut explicit] {
        stage_mute(host, 0, 1, true, 480); // drums
        stage_mute(host, 1, 0, true, 37); // bass
    }
    stage_send(&mut explicit, 2, COMMAND_ROUTE_MUTE, 1, 480, [1.0, 0.0, 0.0, 0.0]);
    stage_send(&mut explicit, 3, COMMAND_ROUTE_MUTE, 0, 37, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(follow.submit_commands(2), RESULT_OK);
    assert_eq!(explicit.submit_commands(4), RESULT_OK);
    for block in 0..6 {
        let a = follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut explicit, &SOLO_FOLLOW_TRACKS, block, None);
        assert!(sol_diff(&a, &c).is_none(), "block {block}: {:?}", sol_diff(&a, &c));
    }
}

/// Cost probe: one solo moving every following send of a wide session.
#[test]
#[ignore = "timing probe"]
fn sol_cost_probe() {
    for (tracks, buses) in [(16_usize, 8_usize), (32, 16), (64, 16)] {
        let track_ids: Vec<String> = (0..tracks).map(|t| format!("t{t:03}")).collect();
        let bus_ids: Vec<String> = (0..buses).map(|b| format!("b{b:03}")).collect();
        let ids: Vec<(String, usize, usize)> = (0..tracks)
            .flat_map(|t| (0..buses).map(move |b| (format!("t{t:03}-b{b:03}"), t, b)))
            .collect();
        let routes: Vec<FollowRoute<'_>> = ids
            .iter()
            .map(|(id, t, b)| {
                (id.as_str(), track_ids[*t].as_str(), false, session::SendTap::PreFader,
                 Some(bus_ids[*b].as_str()), [0.8, -0.3, 0.25, 0.6], true)
            })
            .collect();
        let tr: Vec<&str> = track_ids.iter().map(String::as_str).collect();
        let bu: Vec<&str> = bus_ids.iter().map(String::as_str).collect();
        let document = follow_document(&tr, &bu, &routes, &[], None);
        let mut host = strip_boot(&document, strip_options(64, 0, 0));
        let mut best = u128::MAX;
        for round in 0..20 {
            stage_solo(&mut host, 0, 0, round % 2 == 0, 480);
            let start = std::time::Instant::now();
            assert_eq!(host.submit_commands(1), RESULT_OK);
            let elapsed = start.elapsed().as_nanos();
            best = best.min(elapsed);
            // drain the queues
            for b in 0..1 { let _ = b; }
            follow_render(&mut host, &tr, round as u64, None);
        }
        eprintln!("SOL-COST {tracks}x{buses} ({} sends): best solo submit {} us", routes.len(), best / 1000);
    }
}


/// P4: a solo's follow records ramp at the solo's window, block for block like explicit
/// `routeMute` records at that window (gate 6's comparison, driven by solo instead of kind 4).
#[test]
fn sol_p4_a_solo_follow_ramps_at_the_solo_window() {
    let boot = |follows: bool| {
        let routes: Vec<FollowRoute<'static>> = SOLO_FOLLOW_ROUTES
            .iter()
            .map(|r| (r.0, r.1, r.2, r.3, r.4, r.5, r.6 && follows))
            .collect();
        strip_boot(
            &follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &routes, &[], None),
            strip_options(16, 0, 0),
        )
    };
    let mut follow = boot(true);
    let mut explicit = boot(false);
    stage_solo(&mut follow, 0, SOLO_FOLLOW_VOCAL, true, 480);
    stage_solo(&mut explicit, 0, SOLO_FOLLOW_VOCAL, true, 480);
    stage_send(&mut explicit, 1, COMMAND_ROUTE_MUTE, 0, 480, [1.0, 0.0, 0.0, 0.0]);
    stage_send(&mut explicit, 2, COMMAND_ROUTE_MUTE, 1, 480, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(follow.submit_commands(1), RESULT_OK);
    assert_eq!(explicit.submit_commands(3), RESULT_OK);
    for block in 0..6 {
        let a = follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut explicit, &SOLO_FOLLOW_TRACKS, block, None);
        assert!(sol_diff(&a, &c).is_none(), "block {block}: {:?}", sol_diff(&a, &c));
    }
}

/// P3: `drums` left lane muted at 480, plus a no-op kind 4 on its (already unmuted) right lane.
/// Host X sends the no-op at 0, host Y at 480, host Z sends no no-op. The strip renders the same
/// in all three (a settled unity lane retargeted to itself is exact), so any difference is the
/// follow record's ramp. RED at eff44271d: D3's "last record" takes X's no-op window (0), so the
/// send steps while its strip's left lane fades over 480 samples (a click on the return).
#[test]
fn sol_p3_noop_other_lane_record_sets_the_follow_ramp() {
    let boot = || {
        strip_boot(
            &follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &SOLO_FOLLOW_ROUTES, &[], None),
            strip_options(16, 0, 0),
        )
    };
    let (mut x, mut y, mut z) = (boot(), boot(), boot());
    stage_lane_mute(&mut x, 0, 1, 0, true, 480);
    stage_lane_mute(&mut x, 1, 1, 1, false, 0);
    stage_lane_mute(&mut y, 0, 1, 0, true, 480);
    stage_lane_mute(&mut y, 1, 1, 1, false, 480);
    stage_lane_mute(&mut z, 0, 1, 0, true, 480);
    assert_eq!(x.submit_commands(2), RESULT_OK);
    assert_eq!(y.submit_commands(2), RESULT_OK);
    assert_eq!(z.submit_commands(1), RESULT_OK);
    let (mut xy, mut yz) = (None, None);
    for block in 0..6 {
        let a = follow_render(&mut x, &SOLO_FOLLOW_TRACKS, block, None);
        let b = follow_render(&mut y, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut z, &SOLO_FOLLOW_TRACKS, block, None);
        if xy.is_none() {
            xy = sol_diff(&a, &b).map(|d| (block, d));
        }
        if yz.is_none() {
            yz = sol_diff(&b, &c).map(|d| (block, d));
        }
    }
    assert_eq!(yz, None, "the no-op at 480 is invisible");
    assert_eq!(xy, None, "a no-op record on the unchanged right lane set the left follow's ramp to 0");
}
