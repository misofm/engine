// #1245 attempt 1 verifier scratch (Sol). Probes appended to hosts/host-web/src/tests.rs of a
// git-archive export of 86c050075; never committed. The mutation driver follows as a comment.

// ---- Sol #1245 verifier probes (scratch, never committed) ----

fn sol_seeds(default: u64) -> u64 {
    std::env::var("SOL_SEEDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

#[derive(Debug, Default)]
struct SolReach {
    batches: u32,
    refusals_after_17: u32,
    k17_then_k4_same: u32,
    k17_then_k9: u32,
    k17_then_k4_split: u32,
    k3_split: u32,
    k16_and_k3_same_batch: u32,
    k16_and_k17_same_batch: u32,
    diamonds: u32,
    follow_moves: u32,
    nonzero_ramp_vca: u32,
}

/// The strip's own fader in the model, by strip index in `VCA_STRIPS`.
fn sol_own(model: &session::SessionModel, strip: usize) -> session::DualMonoFader {
    let id = VCA_STRIPS[strip];
    model
        .tracks
        .iter()
        .map(|t| (t.id.as_str(), t.fader.clone()))
        .chain(model.submixes.iter().map(|s| (s.id.as_str(), s.fader.clone())))
        .find(|(candidate, _)| *candidate == id)
        .expect("strip")
        .1
}

/// The reference effective mute of strip/lane: own || any reaching VCA || solo term.
fn sol_ref_mute(model: &session::SessionModel, solos: &[bool; 3], strip: usize, lane: usize) -> bool {
    let own = sol_own(model, strip);
    let own_mute = [own.left_mute, own.right_mute][lane];
    let vca = vca_reference_reach(&model.vcas, VCA_STRIPS[strip])
        .iter()
        .any(|index| [model.vcas[*index].fader.left_mute, model.vcas[*index].fader.right_mute][lane]);
    let any_solo = solos.iter().any(|solo| *solo);
    let solo_term = any_solo && strip < 3 && !solos[strip];
    own_mute || vca || solo_term
}

fn sol_random_model(draw: &mut SendDraw, reach: &mut SolReach) -> session::SessionModel {
    let mut model = vca_ride_model(&[], &[]);
    for strip in VCA_STRIPS {
        let left_db = if draw.below(6) == 0 { draw.uniform(-144.0, 24.0) } else { draw.uniform(-12.0, 6.0) };
        let right_db = if draw.below(6) == 0 { draw.uniform(-144.0, 24.0) } else { draw.uniform(-12.0, 6.0) };
        let (left_mute, right_mute) = (draw.below(8) == 0, draw.below(8) == 0);
        let fader = vca_strip_fader(&mut model, strip);
        (fader.left_db, fader.right_db) = (left_db, right_db);
        (fader.left_mute, fader.right_mute) = (left_mute, right_mute);
    }
    let ids = ["va", "vb", "vc", "vd", "ve", "vf"];
    let count = 2 + draw.below(5) as usize;
    let levels: Vec<u64> = (0..count).map(|_| draw.below(4)).collect();
    let mut vcas: Vec<session::Vca> = (0..count)
        .map(|i| {
            let mut members: Vec<session::StableId> = Vec::new();
            for strip in VCA_STRIPS {
                if draw.below(5) < 2 {
                    members.push(strip_id(strip));
                }
            }
            for j in 0..count {
                if levels[j] > levels[i] && draw.below(2) == 0 {
                    members.push(strip_id(ids[j]));
                }
            }
            let mut offset = || match draw.below(8) {
                0 => 24.0,
                1 => -144.0,
                2 => draw.uniform(-144.0, 24.0),
                _ => draw.uniform(-12.0, 12.0),
            };
            let (left_db, right_db) = (offset(), offset());
            session::Vca {
                id: strip_id(ids[i]),
                fader: session::DualMonoFader {
                    left_db,
                    right_db,
                    left_mute: draw.below(5) == 0,
                    right_mute: draw.below(5) == 0,
                },
                members,
            }
        })
        .collect();
    vcas.sort_by(|a, b| a.id.cmp(&b.id));
    model.vcas = vcas;
    let model = parse_session_json(&canonical_session_json(&model).expect("canonical"))
        .expect("canonical session parses");
    // A diamond: some VCA held by two VCAs.
    let diamond = model.vcas.iter().any(|vca| {
        model.vcas.iter().filter(|other| other.members.contains(&vca.id)).count() >= 2
    });
    reach.diamonds += u32::from(diamond);
    model
}

/// Every mirror the admission must leave consistent after a committed batch.
fn sol_check_mirrors(
    live: &AudioWorkletEngineHost,
    model: &session::SessionModel,
    solos: &[bool; 3],
    what: &str,
) {
    let ready = live.ready.as_ref().expect("ready");
    for strip in 0..VCA_STRIPS.len() {
        for lane in 0..2 {
            let reference_db = vca_reference_effective(model, VCA_STRIPS[strip], lane).0;
            let reached = !vca_reference_reach(&model.vcas, VCA_STRIPS[strip]).is_empty();
            if reached {
                assert_eq!(
                    ready.vcas.effective_db(strip, lane).to_bits(),
                    reference_db.to_bits(),
                    "{what}: strip {strip} lane {lane} effective dB"
                );
            }
            assert_eq!(
                ready.solo.effective_mute(strip, lane),
                sol_ref_mute(model, solos, strip, lane),
                "{what}: strip {strip} lane {lane} effective mute"
            );
            assert_eq!(
                ready.solo.emitted_mute(strip, lane),
                ready.solo.effective_mute(strip, lane),
                "{what}: strip {strip} lane {lane} emitted mute"
            );
            if reached {
                assert_eq!(
                    ready.solo.vca_mute(strip, lane),
                    ready.vcas.vca_mute(strip)[lane],
                    "{what}: strip {strip} lane {lane} owner's VCA term"
                );
            }
        }
        assert_eq!(ready.vcas.fader_delta(strip), [None, None], "{what}: strip {strip} owes a fader record");
    }
    assert!(!ready.vcas.transaction_open(), "{what}: VCA transaction open");
}

/// Heavier randomized settled differential: diamonds, refusals mid-batch after VCA records,
/// biased batch shapes (kind 17 then kind 4 on a member, kind 17 then solo), nonzero ramps on the
/// VCA kinds, mirrors checked after every batch, and bit identity to a fresh plan after settle.
#[test]
fn sol_probe_randomized_settled_differential() {
    let nonzero_user = std::env::var("SOL_NONZERO_USER").is_ok();
    let mut reach = SolReach::default();
    for seed in 1..=sol_seeds(60) {
        let mut draw = SendDraw(seed.wrapping_mul(0x5DEE_CE66_D1CE_4E5B) ^ 0x1245);
        let mut model = sol_random_model(&mut draw, &mut reach);
        let mut live = vca_ride_host(&model, 256);
        follow_render(&mut live, &SOLO_FOLLOW_TRACKS, 0, None);
        let mut block = 1_u64;
        let mut solos = [false; 3];
        let user_ramps: &[u32] = if nonzero_user { &[0, 64, 300] } else { &[0] };
        for batch in 0..8 {
            let lanes_before = follow_lanes(&live);
            let mut scratch = model.clone();
            let mut scratch_solos = solos;
            let records = 1 + draw.below(10) as usize;
            let mut log = Vec::new();
            let mut seen17 = false;
            let mut seen16 = false;
            let mut seen3_reached = false;
            let mut seen17_16 = false;
            let mut mutes_after_17: Vec<usize> = Vec::new();
            let vca_count = scratch.vcas.len();
            for index in 0..records {
                let channel = draw.below(3) as u8;
                let covers = |lane: usize| channel == 2 || usize::from(channel) == lane;
                // Bias: after a kind 17, a kind 4 on a member of that VCA half the time.
                let choice = if seen17 && draw.below(2) == 0 { 100 } else { draw.below(12) };
                match choice {
                    0..=2 => {
                        let vca = draw.below(vca_count as u64) as usize;
                        let db = match draw.below(6) {
                            0 => 24.0,
                            1 => -144.0,
                            2 | 3 => draw.uniform(-144.0, 24.0),
                            _ => draw.uniform(-12.0, 12.0),
                        };
                        let smoothing = [0, 1, 64, 200, 300][draw.below(5) as usize];
                        reach.nonzero_ramp_vca += u32::from(smoothing != 0);
                        stage_vca(&mut live, index, COMMAND_VCA_FADER_DB, vca as u32, channel, db, smoothing);
                        let fader = &mut scratch.vcas[vca].fader;
                        if covers(0) { fader.left_db = db; }
                        if covers(1) { fader.right_db = db; }
                        seen16 = true;
                        seen17_16 |= seen17;
                        log.push(format!("k16 v{vca} ch{channel} {db} @{smoothing}"));
                    }
                    3..=5 => {
                        let vca = draw.below(vca_count as u64) as usize;
                        let on = draw.below(2) == 0;
                        let smoothing = [0, 1, 64, 200, 300][draw.below(5) as usize];
                        stage_vca(&mut live, index, COMMAND_VCA_MUTE, vca as u32, channel, if on { 1.0 } else { 0.0 }, smoothing);
                        let fader = &mut scratch.vcas[vca].fader;
                        if covers(0) { fader.left_mute = on; }
                        if covers(1) { fader.right_mute = on; }
                        seen17 = true;
                        seen17_16 |= seen16;
                        log.push(format!("k17 v{vca} ch{channel} {on} @{smoothing}"));
                    }
                    6 | 7 => {
                        let strip = draw.below(5) as usize;
                        let db = if draw.below(4) == 0 { draw.uniform(-144.0, 24.0) } else { draw.uniform(-18.0, 12.0) };
                        let smoothing = user_ramps[draw.below(user_ramps.len() as u64) as usize];
                        stage_fader_db(&mut live, index, strip as u32, channel, db, smoothing);
                        let fader = vca_strip_fader(&mut scratch, VCA_STRIPS[strip]);
                        if covers(0) { fader.left_db = db; }
                        if covers(1) { fader.right_db = db; }
                        if !vca_reference_reach(&scratch.vcas, VCA_STRIPS[strip]).is_empty() {
                            seen3_reached = true;
                            if channel == 2
                                && vca_reference_effective(&scratch, VCA_STRIPS[strip], 0).0.to_bits()
                                    != vca_reference_effective(&scratch, VCA_STRIPS[strip], 1).0.to_bits()
                            {
                                reach.k3_split += 1;
                            }
                        }
                        log.push(format!("k3 s{strip} ch{channel} {db} @{smoothing}"));
                    }
                    8 | 9 | 100 => {
                        let strip = if choice == 100 {
                            // a strip some VCA reaches, if any
                            let reached: Vec<usize> = (0..5)
                                .filter(|s| !vca_reference_reach(&scratch.vcas, VCA_STRIPS[*s]).is_empty())
                                .collect();
                            if reached.is_empty() { draw.below(5) as usize } else { reached[draw.below(reached.len() as u64) as usize] }
                        } else {
                            draw.below(5) as usize
                        };
                        let on = draw.below(2) == 0;
                        let smoothing = user_ramps[draw.below(user_ramps.len() as u64) as usize];
                        stage_lane_mute(&mut live, index, strip as u32, channel, on, smoothing);
                        let fader = vca_strip_fader(&mut scratch, VCA_STRIPS[strip]);
                        if covers(0) { fader.left_mute = on; }
                        if covers(1) { fader.right_mute = on; }
                        if seen17 {
                            mutes_after_17.push(strip);
                            if channel == 2
                                && sol_ref_mute(&scratch, &scratch_solos, strip, 0)
                                    != sol_ref_mute(&scratch, &scratch_solos, strip, 1)
                            {
                                reach.k17_then_k4_split += 1;
                            }
                        }
                        log.push(format!("k4 s{strip} ch{channel} {on} @{smoothing}"));
                    }
                    _ => {
                        let track = draw.below(3) as usize;
                        let on = draw.below(2) == 0;
                        let smoothing = user_ramps[draw.below(user_ramps.len() as u64) as usize];
                        stage_solo(&mut live, index, track as u32, on, smoothing);
                        scratch_solos[track] = on;
                        reach.k17_then_k9 += u32::from(seen17);
                        log.push(format!("k9 t{track} {on} @{smoothing}"));
                    }
                }
            }
            reach.k17_then_k4_same += u32::from(!mutes_after_17.is_empty());
            reach.k16_and_k3_same_batch += u32::from(seen16 && seen3_reached);
            reach.k16_and_k17_same_batch += u32::from(seen17_16);
            // One batch in five ends with a refused record after everything above.
            let refuse = draw.below(5) == 0 && records < MAXIMUM_COMMAND_RECORDS as usize;
            if refuse {
                let (expected_reason, what) = match draw.below(3) {
                    0 => {
                        stage_vca(&mut live, records, COMMAND_VCA_FADER_DB, vca_count as u32, 2, 0.0, 0);
                        (COMMAND_REASON_UNKNOWN_VCA, "unknown VCA")
                    }
                    1 => {
                        stage_vca(&mut live, records, COMMAND_VCA_MUTE, 0, 2, 0.5, 0);
                        (COMMAND_REASON_DOMAIN, "mute 0.5")
                    }
                    _ => {
                        stage_fader_db(&mut live, records, 0, 2, 30.0, 0);
                        (COMMAND_REASON_DOMAIN, "fader 30 dB")
                    }
                };
                reach.refusals_after_17 += u32::from(seen17);
                assert_vca_refusal(
                    &mut live,
                    (records + 1) as u32,
                    RESULT_INVALID_ARGUMENT,
                    expected_reason,
                    records as u32,
                    &format!("seed {seed} batch {batch}: {what} after {log:?}"),
                );
                let ready = live.ready.as_ref().expect("ready");
                assert!(!ready.solo.transaction_open());
                // The refused batch moved nothing: the mirrors still match the old model.
                sol_check_mirrors(&live, &model, &solos, &format!("seed {seed} batch {batch} after refusal"));
                continue;
            }
            assert_eq!(
                live.submit_commands(records as u32),
                RESULT_OK,
                "seed {seed}: {log:?} (reason {})",
                live.command_report().reason
            );
            model = scratch;
            solos = scratch_solos;
            reach.batches += 1;
            sol_check_mirrors(&live, &model, &solos, &format!("seed {seed} batch {batch}: {log:?}"));
            for ramp in block..block + 3 {
                follow_render(&mut live, &SOLO_FOLLOW_TRACKS, ramp, None);
            }
            let mut fresh = vca_ride_host(&model, 64);
            follow_render(&mut fresh, &SOLO_FOLLOW_TRACKS, 0, None);
            let soloed: Vec<usize> = (0..3).filter(|track| solos[*track]).collect();
            for (index, track) in soloed.iter().enumerate() {
                stage_solo(&mut fresh, index, *track as u32, true, 0);
            }
            if !soloed.is_empty() {
                assert_eq!(fresh.submit_commands(soloed.len() as u32), RESULT_OK);
            }
            for history in 1..block + 3 {
                follow_render(&mut fresh, &SOLO_FOLLOW_TRACKS, history, None);
            }
            assert_eq!(follow_lanes(&live), follow_lanes(&fresh), "seed {seed}: follow mirrors after {log:?}");
            vca_lockstep(
                &mut [&mut live, &mut fresh],
                Some(1),
                block + 3,
                2,
                &format!("seed {seed}: live vs fresh after {log:?}"),
            );
            reach.follow_moves += u32::from(follow_lanes(&live) != lanes_before);
            block += 5;
        }
    }
    eprintln!("sol reach: {reach:?}");
    assert!(reach.refusals_after_17 > 0 && reach.k17_then_k4_same > 0 && reach.k17_then_k9 > 0);
}

/// Ramp-phase differential: a VCA ride or mute renders, block by block *through its ramp*, exactly
/// as a VCA-free host told the same per-member moves directly (kind 3 and kind 4 with the same
/// ramp), so every derived record carries the gesture's window (no click, no hard switch).
#[test]
fn sol_probe_ramps_match_direct_member_moves() {
    let mut checked = 0;
    for seed in 1..=sol_seeds(40) {
        let mut draw = SendDraw(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xBEEF);
        let mut reach = SolReach::default();
        let model = sol_random_model(&mut draw, &mut reach);
        let mut live = vca_ride_host(&model, 256);
        // The reference: no VCAs, every strip's own fader is the live effective one.
        let mut flat = model.clone();
        for strip in 0..5 {
            let left = vca_reference_effective(&model, VCA_STRIPS[strip], 0).0;
            let right = vca_reference_effective(&model, VCA_STRIPS[strip], 1).0;
            let lm = sol_ref_mute(&model, &[false; 3], strip, 0);
            let rm = sol_ref_mute(&model, &[false; 3], strip, 1);
            let fader = vca_strip_fader(&mut flat, VCA_STRIPS[strip]);
            (fader.left_db, fader.right_db) = (left, right);
            (fader.left_mute, fader.right_mute) = (lm, rm);
        }
        flat.vcas = Vec::new();
        let mut reference = vca_ride_host(&flat, 256);
        let mut block = 0_u64;
        vca_lockstep(&mut [&mut live, &mut reference], Some(1), block, 1, &format!("seed {seed}: boot"));
        block += 1;
        let mut current = model.clone();
        for step in 0..6 {
            // One batch: a ride (ramp A) and/or a mute (ramp B) of random VCAs.
            let before: Vec<([f32; 2], [bool; 2])> = (0..5)
                .map(|s| {
                    (
                        [vca_reference_effective(&current, VCA_STRIPS[s], 0).0, vca_reference_effective(&current, VCA_STRIPS[s], 1).0],
                        [sol_ref_mute(&current, &[false; 3], s, 0), sol_ref_mute(&current, &[false; 3], s, 1)],
                    )
                })
                .collect();
            let ride_ramp = [64_u32, 200, 300, 500][draw.below(4) as usize];
            let mute_ramp = [64_u32, 200, 300, 500][draw.below(4) as usize];
            let mut records = 0;
            let kinds = draw.below(3); // 0 ride, 1 mute, 2 both
            if kinds != 1 {
                let vca = draw.below(current.vcas.len() as u64) as usize;
                let channel = draw.below(3) as u8;
                let db = draw.uniform(-24.0, 12.0);
                stage_vca(&mut live, records, COMMAND_VCA_FADER_DB, vca as u32, channel, db, ride_ramp);
                records += 1;
                let fader = &mut current.vcas[vca].fader;
                if channel != 1 { fader.left_db = db; }
                if channel != 0 { fader.right_db = db; }
            }
            if kinds != 0 {
                let vca = draw.below(current.vcas.len() as u64) as usize;
                let channel = draw.below(3) as u8;
                let on = draw.below(2) == 0;
                stage_vca(&mut live, records, COMMAND_VCA_MUTE, vca as u32, channel, if on { 1.0 } else { 0.0 }, mute_ramp);
                records += 1;
                let fader = &mut current.vcas[vca].fader;
                if channel != 1 { fader.left_mute = on; }
                if channel != 0 { fader.right_mute = on; }
            }
            assert_eq!(live.submit_commands(records as u32), RESULT_OK, "seed {seed} step {step}");
            // The same changes, told directly: per changed lane, kind 3 with the ride's ramp, then
            // kind 4 with the mute's ramp.
            let mut direct = 0;
            for s in 0..5 {
                let db = [vca_reference_effective(&current, VCA_STRIPS[s], 0).0, vca_reference_effective(&current, VCA_STRIPS[s], 1).0];
                let changed = [db[0] != before[s].0[0], db[1] != before[s].0[1]];
                match changed {
                    [true, true] if db[0] == db[1] => {
                        stage_fader_db(&mut reference, direct, s as u32, 2, db[0], ride_ramp);
                        direct += 1;
                    }
                    _ => {
                        for lane in 0..2 {
                            if changed[lane] {
                                stage_fader_db(&mut reference, direct, s as u32, lane as u8, db[lane], ride_ramp);
                                direct += 1;
                            }
                        }
                    }
                }
            }
            for s in 0..5 {
                let mute = [sol_ref_mute(&current, &[false; 3], s, 0), sol_ref_mute(&current, &[false; 3], s, 1)];
                let changed = [mute[0] != before[s].1[0], mute[1] != before[s].1[1]];
                match changed {
                    [true, true] if mute[0] == mute[1] => {
                        stage_lane_mute(&mut reference, direct, s as u32, 2, mute[0], mute_ramp);
                        direct += 1;
                    }
                    _ => {
                        for lane in 0..2 {
                            if changed[lane] {
                                stage_lane_mute(&mut reference, direct, s as u32, lane as u8, mute[lane], mute_ramp);
                                direct += 1;
                            }
                        }
                    }
                }
            }
            if direct > 0 {
                assert_eq!(reference.submit_commands(direct as u32), RESULT_OK, "seed {seed} step {step} direct");
                checked += 1;
            }
            assert_eq!(follow_lanes(&live), follow_lanes(&reference), "seed {seed} step {step}: follow");
            // Every block from the batch on, through the ramp.
            vca_lockstep(&mut [&mut live, &mut reference], Some(1), block, 6, &format!("seed {seed} step {step} ramp (ride @{ride_ramp}, mute @{mute_ramp})"));
            block += 6;
        }
    }
    eprintln!("sol ramp probe: {checked} batches with records");
    assert!(checked > 20);
}

/// A clamped member's own move (INFO-1 of the #1244 verdict): own +20 dB in VCA +24 dB, then the
/// member moves to +21 dB with a ramp. The effective value is +24 dB before and after.
#[test]
fn sol_probe_clamped_member_own_move() {
    let host = |drums: f32| {
        vca_ride_host(
            &vca_ride_model(&[("drums", [drums; 2], [false; 2])], &[("band", [24.0; 2], [false; 2], &["drums"])]),
            16,
        )
    };
    let mut live = host(20.0);
    let mut fresh = host(21.0);
    vca_lockstep(&mut [&mut live, &mut fresh], Some(1), 0, 1, "boot");
    let room = follow_fader_room(&live);
    stage_fader_db(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, 21.0, 300);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    eprintln!("sol clamped own move: fader room {:?} -> {:?}", room, follow_fader_room(&live));
    vca_lockstep(&mut [&mut live, &mut fresh], Some(1), 1, 5, "clamped own move vs fresh");
}

/// The bound test's batch, timed in a loop (release), for the A1b claim.
#[test]
#[ignore]
fn sol_probe_time_worst_batch() {
    let document = vca_bound_document(64, 256, 64, 0);
    let mut host = strip_boot(&document, strip_options(256, 0, 0));
    let tracks: Vec<String> = (0..64).map(|track| format!("t{track:03}")).collect();
    let ids: Vec<&str> = tracks.iter().map(String::as_str).collect();
    follow_render(&mut host, &ids, 0, None);
    let mut times = Vec::new();
    for round in 0..50_u64 {
        let top = 255;
        let on = round % 2 == 0;
        stage_vca(&mut host, 0, COMMAND_VCA_FADER_DB, top, 0, if on { -7.0 } else { -1.0 }, 128);
        stage_vca(&mut host, 1, COMMAND_VCA_MUTE, top, 2, if on { 1.0 } else { 0.0 }, 128);
        for index in 2..MAXIMUM_COMMAND_RECORDS as usize {
            stage_fader_db(&mut host, index, (index % 2) as u32, (index % 3) as u8, -(index as f32) / 32.0 - round as f32 * 0.01, 0);
        }
        let started = std::time::Instant::now();
        let result = host.submit_commands(MAXIMUM_COMMAND_RECORDS);
        let elapsed = started.elapsed();
        assert_eq!(result, RESULT_OK, "reason {}", host.command_report().reason);
        // drain
        follow_render(&mut host, &ids, 1 + round, None);
        if round >= 10 {
            times.push(elapsed.as_secs_f64() * 1e6);
        }
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    eprintln!(
        "sol worst batch (us): min {:.1} median {:.1} p90 {:.1} max {:.1}",
        times[0],
        times[times.len() / 2],
        times[times.len() * 9 / 10],
        times[times.len() - 1]
    );
}

/// A kind 3 move on a VCA member whose lanes compose differently (a split) renders, through its
/// ramp, as a VCA-free host told the two lane moves directly with the same ramp; and two rides in
/// one batch take the *last* ride's ramp.
#[test]
fn sol_probe_split_member_move_and_last_ride_ramp() {
    // band holds drums with offsets [-6, +3]; drums own [0, 0] -> effective [-6, +3].
    let live_model = vca_ride_model(&[("drums", [0.0; 2], [false; 2])], &[("band", [-6.0, 3.0], [false; 2], &["drums"])]);
    let flat = vca_ride_model(&[("drums", [-6.0, 3.0], [false; 2])], &[]);
    let mut live = vca_ride_host(&live_model, 16);
    let mut reference = vca_ride_host(&flat, 16);
    vca_lockstep(&mut [&mut live, &mut reference], Some(1), 0, 1, "boot");
    // A both-lane member move to -2 dB with a 300-sample ramp: effective [-8, +1].
    stage_fader_db(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, -2.0, 300);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_fader_db(&mut reference, 0, SOLO_FOLLOW_DRUMS, 0, -8.0, 300);
    stage_fader_db(&mut reference, 1, SOLO_FOLLOW_DRUMS, 1, 1.0, 300);
    assert_eq!(reference.submit_commands(2), RESULT_OK);
    vca_lockstep(&mut [&mut live, &mut reference], Some(1), 1, 4, "split member move ramp");
    // Two rides in one batch: ramps 64 then 400; the members follow with 400.
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, -3.0, 64);
    stage_vca(&mut live, 1, COMMAND_VCA_FADER_DB, 0, 2, 2.0, 400);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    // effective: own -2 + 2 = 0 on both lanes.
    stage_fader_db(&mut reference, 0, SOLO_FOLLOW_DRUMS, 2, 0.0, 400);
    assert_eq!(reference.submit_commands(1), RESULT_OK);
    vca_lockstep(&mut [&mut live, &mut reference], Some(1), 5, 5, "two rides: last ramp");
}

/// D3: a kind 17 whose ramp exceeds the route ramp maximum and moves a following send refuses the
/// whole submission `domain` at the coalesced record's wire index (the batch's first kind 9 or 17).
#[test]
fn sol_probe_long_vca_mute_ramp_refuses_at_first_coalesced_index() {
    let model = vca_ride_model(&[], &[("band", [0.0; 2], [false; 2], &["drums"])]);
    let mut live = vca_ride_host(&model, 16);
    follow_render(&mut live, &SOLO_FOLLOW_TRACKS, 0, None);
    let too_long = lane::kernels::INDEXED_RAMP_LENGTH_MAXIMUM + 1;
    stage_fader_db(&mut live, 0, VCA_BASS, 2, -1.0, 0);
    stage_vca(&mut live, 1, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    stage_fader_db(&mut live, 2, VCA_BASS, 2, -2.0, 0);
    assert_vca_refusal(&mut live, 3, RESULT_INVALID_ARGUMENT, COMMAND_REASON_DOMAIN, 1, "long VCA mute ramp");
    // With a solo first, the refusal names the solo's index.
    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, false, 0);
    stage_fader_db(&mut live, 1, VCA_BASS, 2, -1.0, 0);
    stage_vca(&mut live, 2, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    assert_vca_refusal(&mut live, 3, RESULT_INVALID_ARGUMENT, COMMAND_REASON_DOMAIN, 0, "solo then long VCA mute ramp");
    // A VCA mute alone at index 2 behind two fader moves names index 2.
    stage_fader_db(&mut live, 0, VCA_BASS, 2, -1.0, 0);
    stage_fader_db(&mut live, 1, VCA_BASS, 2, -2.0, 0);
    stage_vca(&mut live, 2, COMMAND_VCA_MUTE, 0, 2, 1.0, 64);
    stage_vca(&mut live, 3, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    assert_vca_refusal(&mut live, 4, RESULT_INVALID_ARGUMENT, COMMAND_REASON_DOMAIN, 2, "first kind 17 index");
}

/// A1c: is `host.budget.vca_projection` reachable? Bisect the smallest admitted budget for the
/// bound session and print what one byte less refuses with.
#[test]
fn sol_probe_vca_projection_reachable() {
    for document in [vca_bound_document(64, 256, 64, 0), vca_bound_document(16, 4, 16, 0)] {
        let boot = |budget: u64| {
            AudioWorkletEngineHost::boot(
                document.as_bytes(),
                WebBootOptions { maximum_memory_bytes: budget, ..strip_options(16, 0, 0) },
            )
        };
        let (mut low, mut high) = (1_u64, 1_u64 << 33);
        while low < high {
            let mid = low + (high - low) / 2;
            if boot(mid).is_ok() { high = mid; } else { low = mid + 1; }
        }
        let host = boot(low).ok().expect("boots");
        let exact = exact_retained_report_total(host.resources());
        let refused = boot(low - 1).err().expect("refuses");
        eprintln!(
            "sol A1c: smallest budget {low}, exact retained {exact}, one less: {}",
            String::from_utf8_lossy(refused.diagnostic())
        );
    }
}

fn sol_lattice_document(tracks: usize, vcas: usize) -> String {
    let names: Vec<String> = (0..tracks).map(|track| format!("t{track:03}")).collect();
    let ids: Vec<&str> = names.iter().map(String::as_str).collect();
    let mains: Vec<String> = names.iter().map(|track| format!("{track}-main")).collect();
    let routes: Vec<FollowRoute<'_>> = mains.iter().zip(&ids).map(|(id, track)| follow_main(id, track, false)).collect();
    let mut model = parse_session_json(&follow_document(&ids, &[], &routes, &[], None)).expect("parses");
    let fader = session::DualMonoFader { left_db: -0.5, right_db: -0.5, left_mute: false, right_mute: false };
    model.vcas = (0..vcas)
        .map(|index| session::Vca {
            id: strip_id(&format!("v{index:03}")),
            fader: fader.clone(),
            members: if index == 0 {
                ids.iter().map(|t| strip_id(t)).collect()
            } else {
                (0..index).map(|j| strip_id(&format!("v{j:03}"))).collect()
            },
        })
        .collect();
    canonical_session_json(&model).expect("canonicalizes")
}

#[test]
fn sol_probe_vca_projection_lattice() {
    let document = sol_lattice_document(64, 256);
    eprintln!("sol lattice doc bytes {}", document.len());
    let boot = |budget: u64| {
        AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions { maximum_memory_bytes: budget, ..strip_options(16, 0, 0) },
        )
    };
    let (mut low, mut high) = (1_u64, 1_u64 << 33);
    while low < high {
        let mid = low + (high - low) / 2;
        if boot(mid).is_ok() { high = mid; } else { low = mid + 1; }
    }
    let host = boot(low).ok().expect("boots");
    let exact = exact_retained_report_total(host.resources());
    let refused = boot(low - 1).err().expect("refuses");
    eprintln!(
        "sol A1c lattice: smallest budget {low}, exact retained {exact}, one less: {}",
        String::from_utf8_lossy(refused.diagnostic())
    );
    let started = std::time::Instant::now();
    let _ = boot(1 << 30).ok().expect("boots");
    eprintln!("sol lattice boot time {:?}", started.elapsed());
}

/* ---- mutation driver (python3) ----
import subprocess, sys, re, os
LIB = '/tmp/claude-1002/v1245/mut/hosts/host-web/src/lib.rs'
orig = open(LIB).read()
# (name, old, new, occurrence (0-based) or None for unique)
M = [
 ("S1 VCA fader pass drops its ramp (smoothing 0)",
  "                            smoothing_samples: vca_fader_smoothing,", "                            smoothing_samples: 0,", None),
 ("S2 kind 17 does not set the coalesced ramp",
  "                        vca_mute_seen = true;\n                        coalesce_smoothing = command.smoothing_samples;",
  "                        vca_mute_seen = true;", None),
 ("S3 kind 3 split records drop the window",
  "                            staged[position] = AdmittedCommand::Fader(TrackFaderRecord::FaderDb {\n                                lanes: lane,\n                                db,\n                                smoothing_samples,",
  "                            staged[position] = AdmittedCommand::Fader(TrackFaderRecord::FaderDb {\n                                lanes: lane,\n                                db,\n                                smoothing_samples: 0,", None),
 ("S4 coalescing refresh never releases (only refreshes muted strips)",
  "                if ready.vcas.reaches(strip)\n                    && !ready.solo.set_vca_mute(strip, ready.vcas.vca_mute(strip))",
  "                if ready.vcas.reaches(strip)\n                    && ready.vcas.vca_mute(strip) != [false; 2]\n                    && !ready.solo.set_vca_mute(strip, ready.vcas.vca_mute(strip))", None),
 ("S5 kind 3 non-split stages the left lane's value for a Right command",
  "                        let effective = if matches!(lanes, BuiltinLaneSelector::Right) {\n                            right\n                        } else {\n                            left\n                        };\n                        ready.vcas.record_emitted_db",
  "                        let effective = left;\n                        ready.vcas.record_emitted_db", None),
 ("S6 kind 3 non-split omits record_emitted_db",
  "                        ready.vcas.record_emitted_db(track, lanes, effective);\n", "", None),
 ("S8 pair bound off by one (exactly at the bound refuses)",
  "    if pairs > MAXIMUM_BROWSER_VCA_REACH_PAIRS {", "    if pairs >= MAXIMUM_BROWSER_VCA_REACH_PAIRS {", None),
 ("S9 the A1c boot projection check is removed",
  "        if vca_peak > memory_budget {", "        if vca_peak > u64::MAX - 1 {", None),
 ("S12 kind 17 does not set the coalesced wire index",
  "                        if !solo_seen && !vca_mute_seen {\n                            coalesce_first_wire_index = index as u32;\n                        }\n                        vca_mute_seen = true;",
  "                        vca_mute_seen = true;", None),
 ("S15 the VCA state is not rolled back on refusal",
  "            ready.routes.rollback();\n            ready.vcas.rollback();", "            ready.routes.rollback();", None),
 ("S19 the VCA fader pass skips the last strip",
  "    if vca_fader_seen {\n        for strip in 0..strip_count {", "    if vca_fader_seen {\n        for strip in 0..strip_count.saturating_sub(1) {", None),
 ("S20 the VCA fader pass is skipped in a batch that also mutes a VCA",
  "    if vca_fader_seen {\n", "    if vca_fader_seen && !vca_mute_seen {\n", None),
 ("S22 a one-lane VCA mute is applied to both lanes",
  "                        if !ready.vcas.set_vca_mute(track, lanes, muted) {", "                        if !ready.vcas.set_vca_mute(track, BuiltinLaneSelector::Both, muted) {", None),
 ("S23 the VCA fader pass ramp is the first kind 16's, not the last",
  "                        vca_fader_seen = true;\n                        vca_fader_smoothing = command.smoothing_samples;",
  "                        if !vca_fader_seen { vca_fader_smoothing = command.smoothing_samples; }\n                        vca_fader_seen = true;", None),
 ("R15 a later kind 4 does not refresh its strip's VCA mute term",
  "                if vca_mute_seen\n                    && ready.vcas.reaches(track)\n                    && !ready.solo.set_vca_mute(track, ready.vcas.vca_mute(track))\n                {\n                    return Err(refuse(COMMAND_REASON_UNKNOWN_TRACK, index));\n                }\n", "", None),
 ("R10 the staging does not grow by the reached strips",
  "        .and_then(|count| count.checked_add(vca_reached_strips.checked_mul(2)?))\n", "", None),
 ("S24 a VCA-free kind 3 path: set_member_db result ignored for split condition (always one record with left)",
  "let produced = if matches!(lanes, BuiltinLaneSelector::Both) && left != right {\n                        for (position, (lane, db)) in [",
  "let produced = if false {\n                        for (position, (lane, db)) in [", None),
]
only = sys.argv[1:] 
results = []
for name, old, new, occ in M:
    if only and not any(name.startswith(o + ' ') for o in only):
        continue
    n = orig.count(old)
    if n != 1:
        results.append((name, f'ANCHOR COUNT {n}'))
        print(name, 'ANCHOR COUNT', n, flush=True)
        continue
    open(LIB, 'w').write(orig.replace(old, new))
    try:
        p = subprocess.run(['cargo','test','--locked','-p','host-web','--features','host-web/test-support','--lib','--no-fail-fast'],
            cwd='/tmp/claude-1002/v1245/mut', env=dict(os.environ, CARGO_TARGET_DIR='/tmp/claude-1002/v1245/target-mut', SOL_SEEDS='60'),
            capture_output=True, text=True, timeout=3000)
        out = p.stdout + p.stderr
        tag = re.sub(r'[^A-Za-z0-9]+','_',name.split(' ')[0])
        open(f'/tmp/claude-1002/v1245/mutlog/{tag}.log','w').write(out)
        if 'error[' in out or 'could not compile' in out:
            res = 'COMPILE ERROR'
        else:
            failed = re.findall(r'^    (tests::\S+)$', out, re.M)
            failed = sorted(set(failed))
            res = 'RED: ' + ', '.join(f.replace('tests::','') for f in failed) if failed else 'GREEN (survived)'
    finally:
        open(LIB, 'w').write(orig)
    results.append((name, res))
    print(name, '=>', res, flush=True)
*/
