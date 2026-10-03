// #1222 attempt 1 verifier probes (Sol). Append to hosts/host-web/src/tests.rs at 466f0ab63; they reuse the #1213 strip_* and #1222 send_* helpers.
// Run: cargo test --locked -p host-web --features host-web/test-support --lib verifier_
// Mutation harness used (V1 V2 V3 V4 V6 V8, I1 I2) is described in 1222-attempt1.md.

// ===== Verifier probes (#1222 attempt 1, Sol). Not for commit. =====

/// Sessions for the follow-mute seeding probe: four tracks with fader lane mutes
/// [F,F], [T,F], [F,T], [T,T]; bus `bx` with its right lane muted; bus `by`.
/// Sends (all pre-fader, so a muted strip still carries signal at the tap):
/// following sends from every track into `bx` and `by`, a non-following one from t3 into `by`,
/// and a following `bx -> by`.
const VP_SENDS: [(&str, &str, bool, &str, bool); 10] = [
    ("f-a", "t0", false, "bx", true),
    ("f-b", "t1", false, "bx", true),
    ("f-c", "t2", false, "bx", true),
    ("f-d", "t3", false, "bx", true),
    ("f-e", "t0", false, "by", true),
    ("f-f", "t1", false, "by", true),
    ("f-g", "t2", false, "by", true),
    ("f-h", "t3", false, "by", false),
    ("f-i", "bx", true, "by", true),
    ("f-j", "t1", false, "by", false),
];

fn vp_document(values: &[SendValues], with_inserts: bool) -> String {
    let (mut model, source, mut track, compressor, route) = strip_base();
    if with_inserts {
        track.inserts.effects = vec![compressor];
    }
    let output = route.destination.clone();
    let mutes = [[false, false], [true, false], [false, true], [true, true]];
    for index in 0..4 {
        strip_add_track(&mut model, &source, &track, &format!("t{index}"));
        model.tracks[index].fader.left_mute = mutes[index][0];
        model.tracks[index].fader.right_mute = mutes[index][1];
    }
    let mut bx = session::Submix::unity(strip_id("bx"), &model.console);
    bx.fader.right_mute = true;
    model.submixes = vec![bx, session::Submix::unity(strip_id("by"), &model.console)];
    for (live, (id, source, submix, bus, follows)) in VP_SENDS.iter().enumerate().rev() {
        let tap = session::SendTap::PreFader;
        let source = if *submix {
            session::RouteSource::Submix { submix_id: strip_id(source), tap }
        } else {
            session::RouteSource::Track { track_id: strip_id(source), tap }
        };
        let mut send = strip_route(&route, id, source, strip_into(bus), values[live].matrix);
        send.gain_db = values[live].gain_db;
        send.mute = values[live].mute;
        send.follows_mute = *follows;
        model.routes.push(send);
    }
    for strip in ["t0", "t1", "t2", "t3", "bx", "by"] {
        model.routes.push(strip_route(
            &route,
            &format!("{strip}-main"),
            strip_post_pan(strip, strip.starts_with('b')),
            output.clone(),
            [1.0, 0.0, 0.0, 1.0],
        ));
    }
    canonical_session_json(&model).expect("probe session canonicalizes")
}

fn vp_seeds() -> Vec<SendValues> {
    let mut draw = SendDraw(0xFEED);
    (0..VP_SENDS.len())
        .map(|_| SendValues {
            gain_db: draw.uniform(-9.0, 3.0),
            matrix: [draw.coefficient(), draw.coefficient(), draw.coefficient(), draw.coefficient()],
            mute: false,
        })
        .collect()
}

fn vp_run(with_inserts: bool, seed: u64) {
    let seeds = vp_seeds();
    let mut draw = SendDraw(seed);
    for trial in 0..6 {
        for smoothing in [0_u32, 480] {
            let mut target = seeds.clone();
            let mut edits: Vec<(u32, usize, [f32; 4])> = Vec::new();
            let n = 3 + draw.below(6) as usize;
            for _ in 0..n {
                let kind = COMMAND_ROUTE_GAIN_DB + draw.below(3) as u32;
                let route = draw.below(VP_SENDS.len() as u64) as usize;
                let values = match kind {
                    COMMAND_ROUTE_GAIN_DB => {
                        let g = draw.uniform(-24.0, 6.0);
                        target[route].gain_db = g;
                        [g, 0.0, 0.0, 0.0]
                    }
                    COMMAND_ROUTE_MUTE => {
                        let m = draw.below(2) == 1;
                        target[route].mute = m;
                        [f32::from(u8::from(m)), 0.0, 0.0, 0.0]
                    }
                    _ => {
                        let m = [draw.coefficient(), draw.coefficient(), draw.coefficient(), draw.coefficient()];
                        target[route].matrix = m;
                        m
                    }
                };
                edits.push((kind, route, values));
            }
            let what = format!("inserts {with_inserts} trial {trial} smoothing {smoothing} edits {edits:?}");
            let opts = strip_options(16, 0, 0);
            let mut live = strip_boot(&vp_document(&seeds, with_inserts), opts);
            let mut fresh = strip_boot(&vp_document(&target, with_inserts), opts);
            assert_eq!(live.ready.as_ref().unwrap().route_controls.len(), VP_SENDS.len());
            let feed = |host: &mut AudioWorkletEngineHost, block: u64| -> Vec<f32> {
                for f in 0..4_u64 {
                    submit_strip_source(host, &format!("t{f}"), block, &strip_planes(f, block));
                }
                assert_eq!(host.render_next(), RESULT_OK);
                host.output_pcm().expect("output").to_vec()
            };
            for block in 0..2 {
                feed(&mut live, block);
                feed(&mut fresh, block);
            }
            // One batch; check every record landed on its own send's queue.
            let before = send_queue_room(&live);
            for (index, (kind, route, values)) in edits.iter().enumerate() {
                stage_send(&mut live, index, *kind, *route as u32, smoothing, *values);
            }
            assert_eq!(live.submit_commands(edits.len() as u32), RESULT_OK, "{what}");
            let after = send_queue_room(&live);
            let mut expected = before.clone();
            for (_, route, _) in &edits {
                expected[*route] -= 1;
            }
            assert_eq!(after, expected, "{what}: per-send queue occupancy");
            for block in 2..7 {
                feed(&mut live, block);
                feed(&mut fresh, block);
            }
            let mut audible = false;
            for block in 7..11 {
                let x = feed(&mut live, block);
                let y = feed(&mut fresh, block);
                for (s, (a, b)) in x.iter().zip(&y).enumerate() {
                    assert_eq!(a.to_bits(), b.to_bits(), "{what}: block {block} sample {s}: {a} vs {b}");
                }
                audible |= x.iter().any(|v| *v != 0.0);
            }
            assert!(audible, "{what}");
        }
    }
}

/// Probe 1: following sends whose source lanes are muted in every combination, edited live,
/// settle on a fresh plan's bits (the mirror's seeded `source_lane_muted` equals the prepared
/// `follow_zeroed`).
#[test]
fn verifier_follow_mute_seed_lane_combos() {
    vp_run(false, 0xA11CE);
}

/// Probe 2: the same with an insert on every track, so `E > 0` and the send band starts after the
/// effect band; checks each record lands on its own send's queue.
#[test]
fn verifier_send_band_after_effects() {
    vp_run(true, 0xB0B);
}

/// Probe 3: a send record and an effect-bypass record, with the effect's queue full, push neither.
#[test]
fn verifier_effect_queue_full_blocks_send() {
    let seeds = vp_seeds();
    let mut host = strip_boot(&vp_document(&seeds, true), strip_options(4, 0, 0));
    for f in 0..4_u64 {
        submit_strip_source(&mut host, &format!("t{f}"), 0, &strip_planes(f, 0));
    }
    assert_eq!(host.render_next(), RESULT_OK);
    // Fill t0's insert 0 queue.
    for index in 0..4 {
        stage_command(&mut host, index, COMMAND_EFFECT_BYPASS, RACK_INSERTS, 255, 0, 0, 0, 0, [(index % 2) as f32, 0.0, 0.0, 0.0]);
    }
    assert_eq!(host.submit_commands(4), RESULT_OK);
    let room = send_queue_room(&host);
    let mirror = send_mirror(&host);
    stage_send(&mut host, 0, COMMAND_ROUTE_GAIN_DB, 0, 0, [-12.0, 0.0, 0.0, 0.0]);
    stage_command(&mut host, 1, COMMAND_EFFECT_BYPASS, RACK_INSERTS, 255, 0, 0, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(2), RESULT_BACKPRESSURE);
    assert_eq!(host.command_report().rejected_index, 1);
    assert_eq!(send_queue_room(&host), room);
    assert_eq!(send_mirror(&host), mirror);
}

/// Probe 4: an admitted batch's mirror edits survive a later refused batch (the mirror is
/// committed on success, so the refused batch's rollback returns to the admitted values), and a
/// further edit then settles on a fresh plan holding every admitted value.
#[test]
fn verifier_a_refusal_after_an_admission_keeps_the_admitted_mirror() {
    let mut host = send_host(&SEND_SEEDS, 16);
    send_render(&mut host, 0);
    stage_send(&mut host, 0, COMMAND_ROUTE_GAIN_DB, 0, 0, [-9.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    send_render(&mut host, 1);
    stage_send(&mut host, 0, COMMAND_ROUTE_MATRIX, 0, 0, [0.1, 0.2, 0.3, 0.4]);
    stage_send(&mut host, 1, COMMAND_ROUTE_MATRIX, 1, 0, [3.0e38, 0.1, 0.2, 0.3]);
    assert_eq!(host.submit_commands(2), RESULT_INVALID_ARGUMENT);
    let mut want = SEND_SEEDS;
    want[0].gain_db = -9.0;
    assert_eq!(send_mirror(&host), want, "the admitted gain survives the refusal");
    stage_send(&mut host, 0, COMMAND_ROUTE_MUTE, 0, 0, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let mut fresh = send_host(&want, 16);
    send_render(&mut fresh, 0);
    send_render(&mut fresh, 1);
    for block in 2..6 {
        let x = send_render(&mut host, block);
        let y = send_render(&mut fresh, block);
        for (s, (a, b)) in x.iter().zip(&y).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "block {block} sample {s}");
        }
    }
}
