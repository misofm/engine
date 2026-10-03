// K3 follow-ups verifier (Sol) scratch, 2026-10-03. Not for commit as is.
//
// Part 1: append to hosts/host-web/src/tests.rs. sol_k3fv_v1..v4 are the #1224 MINOR-1 patch probes
// (V3 and V4 red on eff44271d, green on 843e27558; V4 also red under `Both => changed[0] && changed[1]`).
// k3fv_render_digests prints render digests to compare across commits (--nocapture).

// ---- K3 follow-ups verifier (Sol) probes: not for commit ----

/// The solo-follow session, with `follows_mute` kept or cleared on both sends, and session lane
/// mutes `muted`. Strips: bass 0, drums 1, vocal 2, room 3, verb 4; sends: bass-room 0,
/// drums-verb 1.
fn k3fv_host(follows: bool, muted: &[(&str, [bool; 2])]) -> AudioWorkletEngineHost {
    let routes: Vec<FollowRoute<'static>> = SOLO_FOLLOW_ROUTES
        .iter()
        .map(|r| (r.0, r.1, r.2, r.3, r.4, r.5, r.6 && follows))
        .collect();
    strip_boot(
        &follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &routes, muted, None),
        strip_options(16, 0, 0),
    )
}

fn k3fv_diff(a: &[f32], b: &[f32]) -> Option<(usize, f32, f32)> {
    a.iter()
        .zip(b)
        .enumerate()
        .find(|(_, (x, y))| x.to_bits() != y.to_bits())
        .map(|(i, (x, y))| (i, *x, *y))
}

/// Render `a` and `b` in lockstep over `blocks`, returning the first bit difference and whether
/// any block carried signal.
fn k3fv_lockstep(
    a: &mut AudioWorkletEngineHost,
    b: &mut AudioWorkletEngineHost,
    blocks: core::ops::Range<u64>,
) -> (Option<(u64, (usize, f32, f32))>, bool) {
    let mut first = None;
    let mut audible = false;
    for block in blocks {
        let x = follow_render(a, &SOLO_FOLLOW_TRACKS, block, None);
        let y = follow_render(b, &SOLO_FOLLOW_TRACKS, block, None);
        audible |= x.iter().any(|s| *s != 0.0);
        if first.is_none() {
            first = k3fv_diff(&x, &y).map(|d| (block, d));
        }
    }
    (first, audible)
}

/// V1: two strips, interleaved lane mutes with a no-op on each, and a solo in the same batch.
/// Expected ramps: bass-room 0 (the last bass record covering a changed lane is `bass L @0`),
/// drums-verb 200 (the coalesced `drums R @200`). Compared, block for block, with a host whose
/// sends do not follow and which carries explicit `routeMute` records at exactly those windows.
/// Then an un-solo batch with an interleaved strip unmute, compared settled with a booted host.
#[test]
fn sol_k3fv_v1_two_strips_interleaved_with_solo_in_the_batch() {
    let mut follow = k3fv_host(true, &[]);
    let mut explicit = k3fv_host(false, &[]);
    let mut booted = k3fv_host(true, &[("bass", [false, true])]);
    for host in [&mut follow, &mut explicit] {
        stage_lane_mute(host, 0, 1, 0, true, 480); // drums L (changes)
        stage_lane_mute(host, 1, 0, 1, true, 37); // bass R (changes)
        stage_lane_mute(host, 2, 1, 1, false, 0); // drums R unmute (no-op)
        stage_solo(host, 3, SOLO_FOLLOW_VOCAL, true, 200);
        stage_lane_mute(host, 4, 0, 0, false, 0); // bass L unmute under solo: stays muted, @0
    }
    stage_send(&mut explicit, 5, COMMAND_ROUTE_MUTE, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    stage_send(&mut explicit, 6, COMMAND_ROUTE_MUTE, 1, 200, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(follow.submit_commands(5), RESULT_OK);
    assert_eq!(explicit.submit_commands(7), RESULT_OK);
    assert_eq!(follow_lanes(&follow), [[true, true], [true, true]]);
    let mut first = None;
    for block in 0..8 {
        let x = follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        let y = follow_render(&mut explicit, &SOLO_FOLLOW_TRACKS, block, None);
        follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
        assert!(x.iter().any(|s| *s != 0.0));
        if first.is_none() {
            first = k3fv_diff(&x, &y).map(|d| (block, d));
        }
    }
    assert_eq!(first, None, "V1 batch 1: follow vs explicit");

    // Batch 2: un-solo at 250 with drums L unmuted at 90 in between.
    stage_solo(&mut follow, 0, SOLO_FOLLOW_VOCAL, false, 250);
    stage_lane_mute(&mut follow, 1, 1, 0, false, 90);
    assert_eq!(follow.submit_commands(2), RESULT_OK);
    // bass: user L F, R T; drums: user L F, R F.
    assert_eq!(follow_lanes(&follow), [[false, true], [false, false]]);
    for block in 8..14 {
        follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
    }
    let (difference, audible) = k3fv_lockstep(&mut follow, &mut booted, 14..18);
    assert!(audible);
    assert_eq!(difference, None, "V1 batch 2: settled vs booted");
}

/// V1b: batch 2's ramp. The follow host's un-solo batch against a twin whose only difference is
/// one extra no-op record on the already-settled bass R lane at window 0, placed last. With the
/// patch the twin's no-op never sets bass-room's ramp (it covers no changed lane: bass R stays
/// muted), so both hosts render the same bits.
#[test]
fn sol_k3fv_v1b_unsolo_no_op_on_a_settled_lane() {
    let mut x = k3fv_host(true, &[]);
    let mut z = k3fv_host(true, &[]);
    for host in [&mut x, &mut z] {
        stage_lane_mute(host, 0, 0, 1, true, 37); // bass R
        stage_solo(host, 1, SOLO_FOLLOW_VOCAL, true, 0);
        assert_eq!(host.submit_commands(2), RESULT_OK);
    }
    let (difference, _) = k3fv_lockstep(&mut x, &mut z, 0..4);
    assert_eq!(difference, None);
    for host in [&mut x, &mut z] {
        stage_solo(host, 0, SOLO_FOLLOW_VOCAL, false, 300);
    }
    stage_lane_mute(&mut x, 1, 0, 1, true, 0); // bass R re-mute: a no-op, last
    assert_eq!(x.submit_commands(2), RESULT_OK);
    assert_eq!(z.submit_commands(1), RESULT_OK);
    assert_eq!(follow_lanes(&x), [[false, true], [false, false]]);
    let (difference, audible) = k3fv_lockstep(&mut x, &mut z, 4..12);
    assert!(audible);
    assert_eq!(difference, None, "a no-op on a settled lane set the follow ramp");
}

/// V3: a no-op re-mute of an already-muted lane, with a solo in the same batch. drums starts
/// with R muted; the batch mutes drums L at 480, solos bass at 100, then re-mutes drums R at 0.
/// drums-verb's follow must ramp at 480 (the only record covering the changed L lane), exactly as
/// in a twin without the re-mute. Red on the pre-patch lookup (it took R @0).
#[test]
fn sol_k3fv_v3_no_op_re_mute_with_solo_in_the_batch() {
    let muted = [("drums", [false, true])];
    let mut x = k3fv_host(true, &muted);
    let mut z = k3fv_host(true, &muted);
    assert_eq!(follow_lanes(&x), [[false, false], [false, true]]);
    for host in [&mut x, &mut z] {
        stage_lane_mute(host, 0, 1, 0, true, 480);
        stage_solo(host, 1, 0, true, 100);
    }
    stage_lane_mute(&mut x, 2, 1, 1, true, 0);
    assert_eq!(x.submit_commands(3), RESULT_OK);
    assert_eq!(z.submit_commands(2), RESULT_OK);
    assert_eq!(follow_lanes(&x), [[false, false], [true, true]]);
    let (difference, audible) = k3fv_lockstep(&mut x, &mut z, 0..8);
    assert!(audible);
    assert_eq!(difference, None, "V3: the no-op re-mute set the follow ramp");
}

/// V4: one wire record (`Both`) changes one lane and is a no-op on the other; a later
/// single-lane record on the unchanged lane must not set the ramp, the `Both` record must.
/// Compared with an explicit host whose `routeMute`... cannot express a one-column ramp, so the
/// comparison is X (later no-op at 0) vs Y (later no-op at 480): the strip renders the same, so
/// the follow ramp is the only possible difference.
#[test]
fn sol_k3fv_v4_both_record_then_no_op_lane_record() {
    let muted = [("bass", [false, true])];
    let mut x = k3fv_host(true, &muted);
    let mut y = k3fv_host(true, &muted);
    for host in [&mut x, &mut y] {
        stage_lane_mute(host, 0, 0, 2, true, 400); // bass both: L changes, R no-op
    }
    stage_lane_mute(&mut x, 1, 0, 1, true, 0);
    stage_lane_mute(&mut y, 1, 0, 1, true, 0);
    // Also a drums change in the same batch, interleaved, at another window.
    stage_lane_mute(&mut x, 2, 1, 1, true, 64);
    stage_lane_mute(&mut y, 2, 1, 1, true, 64);
    assert_eq!(x.submit_commands(3), RESULT_OK);
    assert_eq!(y.submit_commands(3), RESULT_OK);
    // The reference: the same batch without the later no-op.
    let mut z = k3fv_host(true, &muted);
    stage_lane_mute(&mut z, 0, 0, 2, true, 400);
    stage_lane_mute(&mut z, 1, 1, 1, true, 64);
    assert_eq!(z.submit_commands(2), RESULT_OK);
    let (difference, audible) = k3fv_lockstep(&mut x, &mut z, 0..8);
    assert!(audible);
    assert_eq!(difference, None, "V4: the later no-op lane record set the ramp");
    let _ = &mut y;
}

// ---- K3 follow-ups verifier (Sol) render-digest probe: not for commit ----

fn k3fv_fnv(h: &mut u64, samples: &[f32]) {
    for s in samples {
        for b in s.to_bits().to_le_bytes() {
            *h ^= u64::from(b);
            *h = h.wrapping_mul(0x100000001b3);
        }
    }
}

#[test]
fn k3fv_render_digests() {
    let mut lines = Vec::new();
    // 1. The #1222 send session, booted, then live edits of every kind on both sends.
    let mut h = 0xcbf29ce484222325_u64;
    let mut host = send_host(&SEND_SEEDS, 16);
    for block in 0..3 {
        k3fv_fnv(&mut h, &send_render(&mut host, block));
    }
    stage_send(&mut host, 0, COMMAND_ROUTE_GAIN_DB, 0, 64, [-7.5, 0.0, 0.0, 0.0]);
    stage_send(&mut host, 1, COMMAND_ROUTE_MATRIX, 1, 200, [0.3, -0.6, 0.45, 0.9]);
    stage_send(&mut host, 2, COMMAND_ROUTE_MUTE, 2, 37, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(3), RESULT_OK);
    for block in 3..9 {
        k3fv_fnv(&mut h, &send_render(&mut host, block));
    }
    stage_send(&mut host, 0, COMMAND_ROUTE_MUTE, 2, 0, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    for block in 9..12 {
        k3fv_fnv(&mut h, &send_render(&mut host, block));
    }
    lines.push(format!("send-session {h:016x}"));
    // 2. The #1224 solo-follow session under every session lane-mute combination of two strips,
    //    then plain single-lane live mutes (no no-op records) and a solo.
    let combos: [[bool; 2]; 4] = [[false, false], [true, false], [false, true], [true, true]];
    for drums in combos {
        for bass in combos {
            let mut h = 0xcbf29ce484222325_u64;
            let mut host = solo_follow_host(&[("drums", drums), ("bass", bass)], 16);
            for block in 0..3 {
                k3fv_fnv(&mut h, &follow_render(&mut host, &SOLO_FOLLOW_TRACKS, block, None));
            }
            let mut index = 0;
            if !drums[0] {
                stage_lane_mute(&mut host, index, 1, 0, true, 300);
                index += 1;
            }
            if bass[1] {
                stage_lane_mute(&mut host, index, 0, 1, false, 90);
                index += 1;
            }
            if index > 0 {
                assert_eq!(host.submit_commands(index as u32), RESULT_OK);
            }
            for block in 3..8 {
                k3fv_fnv(&mut h, &follow_render(&mut host, &SOLO_FOLLOW_TRACKS, block, None));
            }
            stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, true, 150);
            assert_eq!(host.submit_commands(1), RESULT_OK);
            for block in 8..13 {
                k3fv_fnv(&mut h, &follow_render(&mut host, &SOLO_FOLLOW_TRACKS, block, None));
            }
            stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, false, 0);
            assert_eq!(host.submit_commands(1), RESULT_OK);
            for block in 13..16 {
                k3fv_fnv(&mut h, &follow_render(&mut host, &SOLO_FOLLOW_TRACKS, block, None));
            }
            lines.push(format!("solo-follow drums {drums:?} bass {bass:?} {h:016x}"));
        }
    }
    for line in &lines {
        println!("k3fv {line}");
    }
}


// Part 2: crates/graph-compiler/tests/k3fv_bits.rs, built as route_coefficients.rs's contents
// followed by the test below (its helpers are private to that file). Run with K3FV_OUT=<path>
// `cargo test -p graph-compiler --test k3fv_bits -- k3fv --nocapture` at both commits and cmp the dumps.
/*
//! K3 follow-ups verifier probe (not for commit): the compiler's route lowering, dumped bit for
//! bit, so two commits can be diffed.
#[path = "route_coefficients.rs"]
#[allow(dead_code, unused_imports)]
mod rc;

use graph::{PreparedRoute, RouteGate, gated_route_coefficients};
use graph_compiler::GraphCompiler;
use rc::*;
use session::{RouteDestination, parse_session_json};
use std::fmt::Write as _;

#[test]
fn k3fv_dump_route_lowering() {
    let base = with_sends(&parse_session_json(SESSION).expect("fixture parses"));
    let mut draw = Draw(0x00C0_FFEE_1234_5678);
    let mut out = String::new();
    let (mut ok, mut err) = (0, 0);
    for round in 0..400 {
        let mut model = base.clone();
        for fader in model
            .tracks
            .iter_mut()
            .map(|t| &mut t.fader)
            .chain(model.submixes.iter_mut().map(|s| &mut s.fader))
        {
            fader.left_mute = draw.next() % 3 == 0;
            fader.right_mute = draw.next() % 3 == 0;
        }
        for route in &mut model.routes {
            route.gain_db = match draw.next() % 16 {
                0 => 700.0,
                1 => -144.0,
                2 => 24.0,
                3 => 0.0,
                _ => draw.uniform(-150.0, 40.0),
            };
            let m = &mut route.channel_matrix;
            [m.ll, m.lr, m.rl, m.rr] = [(); 4].map(|()| match draw.next() % 20 {
                0 => 1.0e10,
                1 => 3.0e38,
                2 => 1.2e-38,
                _ => draw.coefficient(),
            });
            route.mute = draw.next() % 3 == 0;
            let into_submix = matches!(route.destination, RouteDestination::SubmixInput { .. });
            route.follows_mute = into_submix && draw.next() % 2 == 0;
        }
        match compile(&model) {
            Ok(artifact) => {
                ok += 1;
                let text = GraphCompiler::evidence(artifact.graph(), artifact.report()).canonical_bytes;
                let _ = writeln!(out, "round {round} ok text_len {} text_fnv {:016x}", text.len(), fnv(&text));
                let routes: &[PreparedRoute] = artifact.graph().routes();
                for r in routes {
                    let RouteGate { mute, follow_zeroed } = r.gate;
                    let c = gated_route_coefficients(&r.transform, r.gate).map(f32::to_bits);
                    let _ = writeln!(out, "  {:?} gate {mute} {follow_zeroed:?} coeff {c:08x?}", r.node);
                }
            }
            Err(diags) => {
                err += 1;
                let _ = writeln!(out, "round {round} err {diags:?}");
            }
        }
    }
    let path = std::env::var("K3FV_OUT").expect("K3FV_OUT");
    std::fs::write(&path, &out).expect("write");
    println!("k3fv: {ok} compiled, {err} refused, {} bytes, fnv {:016x}", out.len(), fnv(out.as_bytes()));
}

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

*/

// Part 3: BLOCKER-1 fix, crates/lane/src/kernels.rs (route_mix_ramp_block), bit-identical:
/*
-    let advance = L::splat(L::WIDTH as f32);
-    // Exact: `position < length <= 2^22` whenever a frame ramps.
-    let mut index = L::splat(position as f32).add(L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]));
+    let offsets = L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]);
+    // Exact: `position + vectored < length <= 2^22` whenever a frame ramps.
+    let mut first = position;
     for (left, right) in left_vectors
         .chunks_exact_mut(L::WIDTH)
         .zip(right_vectors.chunks_exact_mut(L::WIDTH))
     {
+        let index = L::splat(first as f32).add(offsets);
+        first += L::WIDTH as u32;
         ...
-        index = index.add(advance);
     }
*/
