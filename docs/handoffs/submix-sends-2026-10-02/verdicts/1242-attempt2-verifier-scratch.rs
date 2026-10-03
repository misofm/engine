// Sol verifier scratch for #1242 attempt 2 (d5dce7b05). Not committed anywhere.
//
// Usage: append the "VCA variants" and "exhaustive" sections to hosts/host-web/src/tests.rs of a
// d5dce7b05 export and run, for example:
//   SOL_P2X_SEEDS=1000 cargo test --locked -p host-web --features host-web/test-support --lib -- sol_probe --nocapture
// The committed P2 was run at 2,000 seeds by replacing its loop bound with
//   1..=std::env::var("SOL_P2_SEEDS").ok().and_then(|t| t.parse().ok()).unwrap_or(VCA_BROWSER_SEEDS)
//
// The "no-VCA comparison" section needs the identical STAGE_LOG instrumentation in BOTH the
// 4ce7767fc and d5dce7b05 exports (it allocates, so the allocation tests go red while it is in):
// insert, just before "// Decode and validate every companion record before any target
// publication." in admit_commands_staged:
//
//    #[cfg(test)]
//    STAGE_LOG.with(|log| {
//        let mut line = String::new();
//        for entry in &ready.command_decoded[..lowered] {
//            match entry.kind {
//                StagedCommandKind::Command(AdmittedCommand::Fader(record)) => line.push_str(&format!("F{}w{}:{:?};", entry.queue_slot, entry.original_wire_index, record)),
//                StagedCommandKind::Command(AdmittedCommand::Route(record)) => line.push_str(&format!("R{}w{}:{:?};", entry.queue_slot, entry.original_wire_index, record)),
//                _ => line.push_str(&format!("O{}w{};", entry.queue_slot, entry.original_wire_index)),
//            }
//        }
//        line.push_str(&format!("|wanted{:?}", &ready.command_wanted));
//        log.borrow_mut().push(line);
//    });
//
// and at the end of lib.rs:
//
//    #[cfg(test)]
//    thread_local! {
//        pub(crate) static STAGE_LOG: core::cell::RefCell<Vec<String>> = const { core::cell::RefCell::new(Vec::new()) };
//    }
//
// then run SOL_NOVCA_OUT=<file> SOL_NOVCA_SEEDS=400 ... -- sol_probe_no_vca in each and `cmp` the files.
//
// ===================== Verified test-gap patches (MINOR-1), as a diff of tests.rs =====================
// --- tests-probe-backup.rs	2026-10-03 13:27:26.243078068 +0000
// +++ probe/hosts/host-web/src/tests.rs	2026-10-03 13:28:47.962387437 +0000
// @@ -13179,12 +13179,17 @@
//              false,
//              "warm-up",
//          );
// +        let room = follow_fader_room(&live);
//          stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, false, 0);
// +        stage_lane_mute(&mut live, 1, 0, 2, false, 0);
//          assert_eq!(
// -            live.submit_commands(1),
// +            live.submit_commands(2),
//              RESULT_OK,
//              "un-mute drums, both lanes"
//          );
// +        let after = follow_fader_room(&live);
// +        assert_eq!(room[1] - after[1], 2, "split: two records on drums");
// +        assert_eq!(room[0] - after[0], 1, "agree: one record on bass");
//          let solo = &live.ready.as_ref().expect("ready").solo;
//          let drums = SOLO_FOLLOW_DRUMS as usize;
//          for (lane, &vca_muted) in vca_mutes.iter().enumerate() {
// @@ -13374,11 +13379,12 @@
//          for block in 1..=8 {
//              let (mut a, mut b) = (0_usize, 0_usize);
//              for _ in 0..=draw.below(3) {
// +                let smoothing = [0_u32, 64][draw.below(2) as usize];
//                  if chance(&mut draw, 1, 2) {
//                      let track = draw.below(3) as u32;
//                      let on = chance(&mut draw, 1, 2);
// -                    stage_solo(&mut live, a, track, on, 0);
// -                    stage_solo(&mut reference, b, track, on, 0);
// +                    stage_solo(&mut live, a, track, on, smoothing);
// +                    stage_solo(&mut reference, b, track, on, smoothing);
//                      (a, b) = (a + 1, b + 1);
//                      log.push(format!("solo {track} {on}"));
//                      continue;
// @@ -13386,7 +13392,7 @@
//                  let strip = draw.below(5) as usize;
//                  let channel = draw.below(3) as u8;
//                  let on = chance(&mut draw, 1, 2);
// -                stage_lane_mute(&mut live, a, strip as u32, channel, on, 0);
// +                stage_lane_mute(&mut live, a, strip as u32, channel, on, smoothing);
//                  a += 1;
//                  let want = |lane: usize| on || vca_mute[strip][lane];
//                  if !on && vca_mute[strip] != [false; 2] {
// @@ -13400,7 +13406,7 @@
//                      _ => &[(0, want(0)), (1, want(1))],
//                  };
//                  for (lane, value) in lanes {
// -                    stage_lane_mute(&mut reference, b, strip as u32, *lane, *value, 0);
// +                    stage_lane_mute(&mut reference, b, strip as u32, *lane, *value, smoothing);
//                      b += 1;
//                  }
//                  log.push(format!(

// ===================== no-VCA comparison =====================

// ---- Sol #1242 attempt 2 probe: no-VCA record and render comparison (identical text in base and head) ----
fn sol_fnv(hash: &mut u64, bits: u32) {
    for byte in bits.to_le_bytes() {
        *hash ^= u64::from(byte);
        *hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
}

#[test]
fn sol_probe_no_vca_records_and_render_compare() {
    let Ok(out) = std::env::var("SOL_NOVCA_OUT") else {
        return;
    };
    let seeds: u64 = std::env::var("SOL_NOVCA_SEEDS")
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or(200);
    let smoothings = [0_u32, 0, 1, 3, 64, 128, 300];
    let mut text = String::new();
    let mut staged_records = 0_usize;
    for seed in 1..=seeds {
        let mut draw = SendDraw(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x5151);
        let document = follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            &[],
            None,
        );
        let mut model = parse_session_json(&document).expect("parses");
        assert!(model.vcas.is_empty());
        let faders = model
            .tracks
            .iter_mut()
            .map(|track| &mut track.fader)
            .chain(model.submixes.iter_mut().map(|submix| &mut submix.fader));
        for fader in faders {
            fader.left_db = draw.uniform(-12.0, 6.0);
            fader.right_db = draw.uniform(-12.0, 6.0);
            fader.left_mute = draw.below(4) == 0;
            fader.right_mute = draw.below(4) == 0;
        }
        let mut host = strip_boot(
            &canonical_session_json(&model).expect("canonical"),
            strip_options(64, 0, 0),
        );
        STAGE_LOG.with(|log| log.borrow_mut().clear());
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut results = Vec::new();
        for bits in follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None) {
            sol_fnv(&mut hash, bits.to_bits());
        }
        for block in 1..=14 {
            let commands = 1 + draw.below(6) as usize;
            for index in 0..commands {
                let smoothing = smoothings[draw.below(smoothings.len() as u64) as usize];
                if draw.below(3) == 0 {
                    stage_solo(
                        &mut host,
                        index,
                        draw.below(3) as u32,
                        draw.below(2) == 0,
                        smoothing,
                    );
                } else {
                    stage_lane_mute(
                        &mut host,
                        index,
                        draw.below(5) as u32,
                        draw.below(3) as u8,
                        draw.below(2) == 0,
                        smoothing,
                    );
                }
            }
            results.push(host.submit_commands(commands as u32));
            for bits in follow_render(&mut host, &SOLO_FOLLOW_TRACKS, block, None) {
                sol_fnv(&mut hash, bits.to_bits());
            }
        }
        let solo = &host.ready.as_ref().expect("ready").solo;
        let mirror: Vec<[bool; 4]> = (0..5)
            .map(|s| {
                [
                    solo.emitted_mute(s, 0),
                    solo.emitted_mute(s, 1),
                    solo.effective_mute(s, 0),
                    solo.effective_mute(s, 1),
                ]
            })
            .collect();
        let log = STAGE_LOG.with(|log| log.borrow().clone());
        staged_records += log.iter().map(|line| line.matches(';').count()).sum::<usize>();
        text.push_str(&format!(
            "seed {seed}: results {results:?} render {hash:016x} mirror {mirror:?} follow {:?}\n",
            follow_lanes(&host)
        ));
        for line in log {
            text.push_str(&format!("  {line}\n"));
        }
    }
    text.push_str(&format!("staged records {staged_records}\n"));
    std::fs::write(out, text).expect("write");
}

// ===================== VCA variants =====================

// ---- Sol #1242 attempt 2 probes: VCA variants ----
fn sol_vca_case(
    draw: &mut SendDraw,
    vca_count_max: u64,
) -> (session::SessionModel, session::SessionModel, Vec<[bool; 2]>) {
    fn chance(draw: &mut SendDraw, numerator: u64, denominator: u64) -> bool {
        draw.below(denominator) < numerator
    }
    let names = ["bass", "drums", "vocal", "room", "verb"];
    let document = follow_document(
        &SOLO_FOLLOW_TRACKS,
        &["verb", "room"],
        &SOLO_FOLLOW_ROUTES,
        &[],
        None,
    );
    let mut model = parse_session_json(&document).expect("parses");
    let faders = model
        .tracks
        .iter_mut()
        .map(|track| &mut track.fader)
        .chain(model.submixes.iter_mut().map(|submix| &mut submix.fader));
    for fader in faders {
        fader.left_db = draw.uniform(-12.0, 6.0);
        fader.right_db = draw.uniform(-12.0, 6.0);
        fader.left_mute = chance(draw, 1, 5);
        fader.right_mute = chance(draw, 1, 5);
    }
    let count = 1 + draw.below(vca_count_max) as usize;
    let levels: Vec<u64> = (0..count).map(|_| draw.below(4)).collect();
    let vca_names: Vec<String> = (0..count).map(|i| format!("w{}", (i * 5) % 13)).collect();
    model.vcas = (0..count)
        .map(|i| {
            let candidates: Vec<String> = names
                .iter()
                .map(|name| (*name).to_owned())
                .chain(
                    (0..count)
                        .filter(|other| levels[*other] > levels[i])
                        .map(|other| vca_names[other].clone()),
                )
                .collect();
            let members: Vec<session::StableId> = candidates
                .into_iter()
                .filter(|_| chance(draw, 1, 2))
                .map(|member| strip_id(&member))
                .collect();
            let mut offset = || {
                if chance(draw, 1, 4) {
                    draw.uniform(-144.0, 24.0)
                } else {
                    draw.uniform(-12.0, 12.0)
                }
            };
            let (left_db, right_db) = (offset(), offset());
            // Bias toward one-lane mutes.
            let (left_mute, right_mute) = match draw.below(6) {
                0 => (true, false),
                1 => (false, true),
                2 => (true, true),
                _ => (false, false),
            };
            session::Vca {
                id: strip_id(&vca_names[i]),
                fader: session::DualMonoFader {
                    left_db,
                    right_db,
                    left_mute,
                    right_mute,
                },
                members,
            }
        })
        .collect();
    let canonical = parse_session_json(&canonical_session_json(&model).expect("canonical"))
        .expect("canonical session parses");
    let strip_ids: Vec<String> = canonical
        .tracks
        .iter()
        .map(|track| track.id.as_str().to_owned())
        .chain(canonical.submixes.iter().map(|s| s.id.as_str().to_owned()))
        .collect();
    assert_eq!(strip_ids, ["bass", "drums", "vocal", "room", "verb"]);
    let mut plain = canonical.clone();
    plain.vcas.clear();
    let mut vca_mute = Vec::new();
    let faders = plain
        .tracks
        .iter_mut()
        .map(|track| &mut track.fader)
        .chain(plain.submixes.iter_mut().map(|submix| &mut submix.fader));
    for (fader, strip) in faders.zip(&strip_ids) {
        let reach = vca_reference_reach(&canonical.vcas, strip);
        let lane = |l: usize| -> (f32, bool) {
            let own = [fader.left_db, fader.right_db][l];
            if reach.is_empty() {
                return (own, false);
            }
            let mut sum = f64::from(own);
            let mut muted = false;
            for index in &reach {
                let vca = &canonical.vcas[*index].fader;
                sum += f64::from([vca.left_db, vca.right_db][l]);
                muted |= [vca.left_mute, vca.right_mute][l];
            }
            (sum.clamp(-144.0, 24.0) as f32, muted)
        };
        let (left, right) = (lane(0), lane(1));
        fader.left_db = left.0;
        fader.right_db = right.0;
        fader.left_mute |= left.1;
        fader.right_mute |= right.1;
        vca_mute.push([left.1, right.1]);
    }
    (canonical, plain, vca_mute)
}

fn sol_check_mirrors(live: &AudioWorkletEngineHost, reference: &AudioWorkletEngineHost, what: &str) {
    let a = &live.ready.as_ref().expect("ready").solo;
    let b = &reference.ready.as_ref().expect("ready").solo;
    for strip in 0..5 {
        for lane in 0..2 {
            assert_eq!(
                a.emitted_mute(strip, lane),
                a.effective_mute(strip, lane),
                "{what}: live emitted != effective strip {strip} lane {lane}"
            );
            assert_eq!(
                a.effective_mute(strip, lane),
                b.effective_mute(strip, lane),
                "{what}: live effective != reference strip {strip} lane {lane}"
            );
            assert_eq!(
                a.emitted_mute(strip, lane),
                b.emitted_mute(strip, lane),
                "{what}: live emitted != reference strip {strip} lane {lane}"
            );
        }
    }
    let routes = &live.ready.as_ref().expect("ready").routes;
    for route in 0..routes.len() {
        let entry = routes.get(route).expect("route");
        let source = entry.source_strip;
        assert_eq!(
            entry.source_lane_muted,
            [a.effective_mute(source, 0), a.effective_mute(source, 1)],
            "{what}: follow mirror of route {route}"
        );
    }
    assert_eq!(follow_lanes(live), follow_lanes(reference), "{what}: follow lanes");
}

/// P2 extended: nonzero smoothing, bigger batches, deeper forests, three blocks per batch,
/// mirror invariants every batch.
#[test]
fn sol_probe_p2_extended_with_ramps() {
    let seeds: u64 = std::env::var("SOL_P2X_SEEDS")
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or(32);
    let smoothings = [0_u32, 0, 1, 5, 64, 129, 256];
    let (mut split, mut mixed, mut single_on_vca_lane, mut submix_unmute, mut ramped_split) =
        (0_u32, 0_u32, 0_u32, 0_u32, 0_u32);
    for seed in 1..=seeds {
        let mut draw = SendDraw(seed.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 0x1242);
        let (canonical, plain, vca_mute) = sol_vca_case(&mut draw, 6);
        let mut live = strip_boot(
            &canonical_session_json(&canonical).expect("canonical"),
            strip_options(64, 0, 0),
        );
        let mut reference = strip_boot(
            &canonical_session_json(&plain).expect("canonical"),
            strip_options(64, 0, 0),
        );
        follow_lockstep(
            &mut [&mut live, &mut reference],
            &SOLO_FOLLOW_TRACKS,
            0,
            1,
            true,
            &format!("seed {seed}: boot"),
        );
        sol_check_mirrors(&live, &reference, &format!("seed {seed}: boot"));
        let mut log = Vec::new();
        let mut block = 1_u64;
        for _batch in 0..10 {
            let (mut a, mut b) = (0_usize, 0_usize);
            let (mut has_solo, mut has_split) = (false, false);
            for _ in 0..=draw.below(6) {
                let smoothing = smoothings[draw.below(smoothings.len() as u64) as usize];
                if draw.below(3) == 0 {
                    let track = draw.below(3) as u32;
                    let on = draw.below(2) == 0;
                    stage_solo(&mut live, a, track, on, smoothing);
                    stage_solo(&mut reference, b, track, on, smoothing);
                    (a, b) = (a + 1, b + 1);
                    has_solo = true;
                    log.push(format!("solo {track} {on} s{smoothing}"));
                    continue;
                }
                let strip = draw.below(5) as usize;
                let channel = draw.below(3) as u8;
                let on = draw.below(2) == 0;
                stage_lane_mute(&mut live, a, strip as u32, channel, on, smoothing);
                a += 1;
                let want = |lane: usize| on || vca_mute[strip][lane];
                if channel == 2 && want(0) != want(1) {
                    split += 1;
                    has_split = true;
                    ramped_split += u32::from(smoothing > 0);
                }
                if channel < 2 && !on && vca_mute[strip][channel as usize] {
                    single_on_vca_lane += 1;
                }
                if strip >= 3 && !on && vca_mute[strip] != [false; 2] {
                    submix_unmute += 1;
                }
                let lanes: &[(u8, bool)] = match channel {
                    0 => &[(0, want(0))],
                    1 => &[(1, want(1))],
                    _ if want(0) == want(1) => &[(2, want(0))],
                    _ => &[(0, want(0)), (1, want(1))],
                };
                for (lane, value) in lanes {
                    stage_lane_mute(&mut reference, b, strip as u32, *lane, *value, smoothing);
                    b += 1;
                }
                log.push(format!(
                    "mute strip {strip} ch {channel} {on} s{smoothing} (vca {:?})",
                    vca_mute[strip]
                ));
            }
            mixed += u32::from(has_solo && has_split);
            let ra = live.submit_commands(a as u32);
            let rb = reference.submit_commands(b as u32);
            assert_eq!(ra, rb, "seed {seed}: results differ after {log:?}");
            assert_eq!(ra, RESULT_OK, "seed {seed}: live after {log:?}");
            sol_check_mirrors(&live, &reference, &format!("seed {seed}: after {log:?}"));
            follow_lockstep(
                &mut [&mut live, &mut reference],
                &SOLO_FOLLOW_TRACKS,
                block,
                3,
                true,
                &format!("seed {seed}: after {log:?}"),
            );
            block += 3;
        }
    }
    eprintln!(
        "split {split} ramped_split {ramped_split} mixed {mixed} single_on_vca_lane {single_on_vca_lane} submix_unmute {submix_unmute}"
    );
    assert!(split > 0 && mixed > 0 && single_on_vca_lane > 0 && submix_unmute > 0 && ramped_split > 0);
}

/// Deterministic mixed batches: kind 4 split + solo + VCA-member submix in one submission, then
/// single-lane commands, ramps on, compared with the written-directly reference at every block.
#[test]
fn sol_probe_mixed_batch_split_and_single_lanes() {
    for vca_mutes in [[true, false], [false, true]] {
        let mut live = vca_follow_host(&[], vca_mutes, 0.0);
        // Reference: no VCA, drums and room written at -3 dB with the VCA lane mute baked.
        let document = follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            &[("drums", vca_mutes), ("room", vca_mutes)],
            None,
        );
        let mut model = parse_session_json(&document).expect("parses");
        for track in &mut model.tracks {
            if track.id.as_str() == "drums" {
                track.fader.left_db = -3.0;
                track.fader.right_db = -3.0;
            }
        }
        for submix in &mut model.submixes {
            if submix.id.as_str() == "room" {
                submix.fader.left_db = -3.0;
                submix.fader.right_db = -3.0;
            }
        }
        let mut reference = strip_boot(
            &canonical_session_json(&model).expect("canonical"),
            strip_options(16, 0, 0),
        );
        follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 0, 1, true, "boot");
        let want = |lane: usize, on: bool| on || vca_mutes[lane];
        // Batch 1: solo drums, kind 4 Both false on drums (split), kind 4 Both false on room
        // (submix, split), solo vocal. Smoothing 64.
        stage_solo(&mut live, 0, SOLO_FOLLOW_DRUMS, true, 64);
        stage_lane_mute(&mut live, 1, SOLO_FOLLOW_DRUMS, 2, false, 64);
        stage_lane_mute(&mut live, 2, 3, 2, false, 64);
        stage_solo(&mut live, 3, SOLO_FOLLOW_VOCAL, true, 64);
        assert_eq!(live.submit_commands(4), RESULT_OK);
        stage_solo(&mut reference, 0, SOLO_FOLLOW_DRUMS, true, 64);
        stage_lane_mute(&mut reference, 1, SOLO_FOLLOW_DRUMS, 0, want(0, false), 64);
        stage_lane_mute(&mut reference, 2, SOLO_FOLLOW_DRUMS, 1, want(1, false), 64);
        stage_lane_mute(&mut reference, 3, 3, 0, want(0, false), 64);
        stage_lane_mute(&mut reference, 4, 3, 1, want(1, false), 64);
        stage_solo(&mut reference, 5, SOLO_FOLLOW_VOCAL, true, 64);
        assert_eq!(reference.submit_commands(6), RESULT_OK);
        sol_check_mirrors(&live, &reference, "batch 1");
        follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 1, 3, true, "batch 1");
        // Batch 2: kind 4 single lanes on drums: the VCA-muted lane un-muted (stays muted), the
        // open lane muted; then unsolo both. Smoothing 7.
        let vca_lane = usize::from(vca_mutes[1]);
        let open_lane = 1 - vca_lane;
        stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, vca_lane as u8, false, 7);
        stage_lane_mute(&mut live, 1, SOLO_FOLLOW_DRUMS, open_lane as u8, true, 7);
        stage_solo(&mut live, 2, SOLO_FOLLOW_DRUMS, false, 7);
        stage_solo(&mut live, 3, SOLO_FOLLOW_VOCAL, false, 7);
        assert_eq!(live.submit_commands(4), RESULT_OK);
        stage_lane_mute(&mut reference, 0, SOLO_FOLLOW_DRUMS, vca_lane as u8, true, 7);
        stage_lane_mute(&mut reference, 1, SOLO_FOLLOW_DRUMS, open_lane as u8, true, 7);
        stage_solo(&mut reference, 2, SOLO_FOLLOW_DRUMS, false, 7);
        stage_solo(&mut reference, 3, SOLO_FOLLOW_VOCAL, false, 7);
        assert_eq!(reference.submit_commands(4), RESULT_OK);
        sol_check_mirrors(&live, &reference, "batch 2");
        follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 4, 3, true, "batch 2");
        // Batch 3: kind 4 Both false on drums again (split back), kind 4 Both true on room
        // (agree), kind 4 Both false on room (split), solo bass. Smoothing 200.
        stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, false, 200);
        stage_lane_mute(&mut live, 1, 3, 2, true, 200);
        stage_lane_mute(&mut live, 2, 3, 2, false, 200);
        stage_solo(&mut live, 3, 0, true, 200);
        assert_eq!(live.submit_commands(4), RESULT_OK);
        stage_lane_mute(&mut reference, 0, SOLO_FOLLOW_DRUMS, 0, want(0, false), 200);
        stage_lane_mute(&mut reference, 1, SOLO_FOLLOW_DRUMS, 1, want(1, false), 200);
        stage_lane_mute(&mut reference, 2, 3, 2, true, 200);
        stage_lane_mute(&mut reference, 3, 3, 0, want(0, false), 200);
        stage_lane_mute(&mut reference, 4, 3, 1, want(1, false), 200);
        stage_solo(&mut reference, 5, 0, true, 200);
        assert_eq!(reference.submit_commands(6), RESULT_OK);
        sol_check_mirrors(&live, &reference, "batch 3");
        assert!(
            follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 7, 4, true, "batch 3"),
            "audible"
        );
    }
}

/// Queue room: a split stages two records into one fader queue; at depth 1 a Both kind 4 on a
/// VCA-split strip is refused whole with backpressure and leaves no state behind.
#[test]
fn sol_probe_split_at_queue_depth_one() {
    let document = follow_document(
        &SOLO_FOLLOW_TRACKS,
        &["verb", "room"],
        &SOLO_FOLLOW_ROUTES,
        &[],
        None,
    );
    let mut model = parse_session_json(&document).expect("parses");
    model.vcas = vec![session::Vca {
        id: strip_id("fx"),
        fader: session::DualMonoFader {
            left_db: -3.0,
            right_db: -3.0,
            left_mute: false,
            right_mute: true,
        },
        members: vec![strip_id("drums")],
    }];
    let mut host = strip_boot(
        &canonical_session_json(&model).expect("canonical"),
        strip_options(1, 0, 0),
    );
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    stage_lane_mute(&mut host, 0, SOLO_FOLLOW_DRUMS, 2, true, 0);
    let result = host.submit_commands(1);
    eprintln!("depth-1 Both mute on split strip (mute true -> agree): {result}");
    stage_lane_mute(&mut host, 0, SOLO_FOLLOW_DRUMS, 2, false, 0);
    let result = host.submit_commands(1);
    eprintln!("depth-1 Both un-mute on split strip: {result} (backpressure = {RESULT_BACKPRESSURE})");
    let solo = &host.ready.as_ref().expect("ready").solo;
    eprintln!(
        "user {:?} emitted {:?}",
        [solo.user_mute(1, 0), solo.user_mute(1, 1)],
        [solo.emitted_mute(1, 0), solo.emitted_mute(1, 1)]
    );
}

// ===================== exhaustive =====================

/// Exhaustive lane-combination probe: VCA lane mute (L, R, both, none) x member (track drums,
/// submix room, which has its own following send room-verb) x the member's own mute x solo
/// (none, the member's track, another track) x kind 4 channel x value x smoothing x same-batch.
#[test]
fn sol_probe_exhaustive_lane_combinations() {
    let mut routes: Vec<FollowRoute<'static>> = SOLO_FOLLOW_ROUTES.to_vec();
    routes.push((
        "room-verb",
        "room",
        true,
        session::SendTap::PreFader,
        Some("verb"),
        [0.6, 0.2, -0.4, 0.7],
        true,
    ));
    let mut cases = 0_u32;
    for vca_lanes in [[true, false], [false, true], [true, true], [false, false]] {
        for (member, member_strip) in [("drums", 1_u32), ("room", 3_u32)] {
            for own in [[false, false], [true, false], [false, true], [true, true]] {
                for solo in [None, Some(SOLO_FOLLOW_DRUMS), Some(SOLO_FOLLOW_VOCAL)] {
                    for channel in 0_u8..3 {
                        for on in [false, true] {
                            for smoothing in [0_u32, 64] {
                                for same_batch in [false, true] {
                                    cases += 1;
                                    let what = format!(
                                        "vca {vca_lanes:?} member {member} own {own:?} solo {solo:?} ch {channel} on {on} s{smoothing} same {same_batch}"
                                    );
                                    let document = follow_document(
                                        &SOLO_FOLLOW_TRACKS,
                                        &["verb", "room"],
                                        &routes,
                                        &[(member, own)],
                                        None,
                                    );
                                    let mut model = parse_session_json(&document).expect("parses");
                                    let mut plain = model.clone();
                                    model.vcas = vec![session::Vca {
                                        id: strip_id("fx"),
                                        fader: session::DualMonoFader {
                                            left_db: -3.0,
                                            right_db: -3.0,
                                            left_mute: vca_lanes[0],
                                            right_mute: vca_lanes[1],
                                        },
                                        members: vec![strip_id(member)],
                                    }];
                                    let fader = plain
                                        .tracks
                                        .iter_mut()
                                        .map(|t| (t.id.as_str().to_owned(), &mut t.fader))
                                        .chain(plain.submixes.iter_mut().map(|s| (s.id.as_str().to_owned(), &mut s.fader)))
                                        .find(|(id, _)| id == member)
                                        .expect("member")
                                        .1;
                                    fader.left_db = -3.0;
                                    fader.right_db = -3.0;
                                    fader.left_mute |= vca_lanes[0];
                                    fader.right_mute |= vca_lanes[1];
                                    let mut live = strip_boot(
                                        &canonical_session_json(&model).expect("canonical"),
                                        strip_options(16, 0, 0),
                                    );
                                    let mut reference = strip_boot(
                                        &canonical_session_json(&plain).expect("canonical"),
                                        strip_options(16, 0, 0),
                                    );
                                    follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, 0, 1, true, &what);
                                    sol_check_mirrors(&live, &reference, &what);
                                    let want = |lane: usize, value: bool| value || vca_lanes[lane];
                                    let stage = |host: &mut AudioWorkletEngineHost, at: &mut usize, reference: bool, channel: u8, value: bool| {
                                        if !reference {
                                            stage_lane_mute(host, *at, member_strip, channel, value, smoothing);
                                            *at += 1;
                                            return;
                                        }
                                        let lanes: &[(u8, bool)] = match channel {
                                            0 => &[(0, want(0, value))],
                                            1 => &[(1, want(1, value))],
                                            _ if want(0, value) == want(1, value) => &[(2, want(0, value))],
                                            _ => &[(0, want(0, value)), (1, want(1, value))],
                                        };
                                        for (lane, v) in lanes {
                                            stage_lane_mute(host, *at, member_strip, *lane, *v, smoothing);
                                            *at += 1;
                                        }
                                    };
                                    let mut block = 1_u64;
                                    // Batch A: solo on (alone unless same_batch).
                                    let (mut a, mut b) = (0_usize, 0_usize);
                                    if let Some(track) = solo {
                                        stage_solo(&mut live, a, track, true, smoothing);
                                        stage_solo(&mut reference, b, track, true, smoothing);
                                        a += 1;
                                        b += 1;
                                        if !same_batch {
                                            assert_eq!(live.submit_commands(a as u32), RESULT_OK, "{what}");
                                            assert_eq!(reference.submit_commands(b as u32), RESULT_OK, "{what}");
                                            sol_check_mirrors(&live, &reference, &what);
                                            follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, block, 2, true, &what);
                                            block += 2;
                                            (a, b) = (0, 0);
                                        }
                                    }
                                    // Batch B: the kind 4 under test.
                                    stage(&mut live, &mut a, false, channel, on);
                                    stage(&mut reference, &mut b, true, channel, on);
                                    assert_eq!(live.submit_commands(a as u32), RESULT_OK, "{what}");
                                    assert_eq!(reference.submit_commands(b as u32), RESULT_OK, "{what}");
                                    sol_check_mirrors(&live, &reference, &what);
                                    follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, block, 2, true, &what);
                                    block += 2;
                                    // Batch C: solo off and a Both kind 4 of the opposite value.
                                    let (mut a, mut b) = (0_usize, 0_usize);
                                    if let Some(track) = solo {
                                        stage_solo(&mut live, a, track, false, smoothing);
                                        stage_solo(&mut reference, b, track, false, smoothing);
                                        a += 1;
                                        b += 1;
                                    }
                                    stage(&mut live, &mut a, false, 2, !on);
                                    stage(&mut reference, &mut b, true, 2, !on);
                                    assert_eq!(live.submit_commands(a as u32), RESULT_OK, "{what}");
                                    assert_eq!(reference.submit_commands(b as u32), RESULT_OK, "{what}");
                                    sol_check_mirrors(&live, &reference, &what);
                                    follow_lockstep(&mut [&mut live, &mut reference], &SOLO_FOLLOW_TRACKS, block, 2, true, &what);
                                    // Mute wins: every VCA-muted lane of the member is muted.
                                    let state = &live.ready.as_ref().expect("ready").solo;
                                    for lane in 0..2 {
                                        if vca_lanes[lane] {
                                            assert!(state.effective_mute(member_strip as usize, lane), "{what}: lane {lane}");
                                            assert!(state.emitted_mute(member_strip as usize, lane), "{what}: lane {lane}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("exhaustive cases {cases}");
}
