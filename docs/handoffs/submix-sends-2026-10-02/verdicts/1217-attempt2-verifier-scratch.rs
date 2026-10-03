// Sol verifier probes for #1217 attempt 2 (1b5034c35). Scratch, never committed: append to
// crates/host-core/tests/route_mute.rs (they use its helpers) and run with
//   cargo test -p host-core --features host-core/test-support,graph/test-support --test route_mute probe_
//
// probe_middle_and_scattered_inactive_runs_match_d3: green on 1b5034c35; RED under mutation MC
// (route_segments loses `stored = true` after an opening Store), which every committed test misses:
//   Bus, 3 inputs, muted [1], plane 0: sample 0: 0.29300022 != 0.36220396

/// `muted.len()` tracks with drawn gains/matrices route into `sum`; `r<k>` muted when `muted[k]`.
fn probe_many(draw: &mut Draw, sum: Sum, muted: &[bool]) -> (SessionModel, Vec<Route>, Feeds) {
    let (mut model, source, track) = empty_session();
    let mut feeds = Vec::new();
    let mut routes = Vec::new();
    for (index, is_muted) in muted.iter().enumerate() {
        let id = format!("t{index:02}");
        add_track(&mut model, &source, &track, &id);
        let destination = match sum {
            Sum::Bus => into_bus("b"),
            Sum::Output => RouteDestination::OutputInput {
                output_id: sid("main-out"),
            },
        };
        let mut drawn = route(&format!("r{index:02}"), input_tap(&id), destination);
        drawn.gain_db = draw.in_domain(-12.0, 6.0);
        let signs = [draw.chance(1, 2), draw.chance(1, 2), draw.chance(1, 2), draw.chance(1, 2)];
        drawn.channel_matrix = ChannelMatrix {
            ll: signed(draw, signs[0]),
            lr: signed(draw, signs[1]),
            rl: signed(draw, signs[2]),
            rr: signed(draw, signs[3]),
        };
        drawn.mute = *is_muted;
        feeds.push((id, planes(draw, None)));
        routes.push(drawn);
    }
    model.routes = routes.clone();
    if let Sum::Bus = sum {
        model.routes.push(to_output("b-main", bus_input("b")));
        model.submixes = vec![Submix::unity(sid("b"), &model.console)];
    }
    (model, routes, feeds)
}

#[test]
fn probe_middle_and_scattered_inactive_runs_match_d3() {
    let patterns: [&[usize]; 6] = [
        &[1],          // open, muted, open
        &[1, 3],       // o m o m o
        &[3, 9],       // 12 inputs, a later run crossing a group boundary
        &[0, 5, 10],   // first inactive, scattered
        &[8],          // the 9th input muted: group boundary
        &[2, 3, 4, 5, 6, 7, 8, 9, 10],
    ];
    let counts = [3, 5, 12, 12, 11, 12];
    let mut draw = Draw::new(99_217);
    for (pattern, count) in patterns.iter().zip(counts) {
        let muted: Vec<bool> = (0..count).map(|index| pattern.contains(&index)).collect();
        for sum in [Sum::Bus, Sum::Output] {
            let (model, routes, feeds) = probe_many(&mut draw, sum, &muted);
            let (actual, mixes) = render_counted(&document(&model), &feeds);
            let contributions: Vec<(bool, [Vec<f32>; 2])> = routes
                .iter()
                .zip(&feeds)
                .map(|(route, (_, planes))| (!route.mute, mix(route, planes)))
                .collect();
            let summed = d3_sum(&contributions);
            let expected = match sum {
                Sum::Bus => mix(&to_output("b-main", bus_input("b")), &summed),
                Sum::Output => summed,
            };
            let what = format!("{sum:?}, {count} inputs, muted {pattern:?}");
            for plane in 0..2 {
                assert_bits_equal(&actual[plane], &expected[plane], &format!("{what}, plane {plane}"));
            }
            for (index, is_muted) in muted.iter().enumerate() {
                assert_eq!(
                    mixes[route_index(&model, &format!("r{index:02}"))],
                    if *is_muted { 0 } else { BLOCKS as u64 },
                    "{what}: r{index:02} mixes"
                );
            }
        }
    }
}

// Render-bit digest probe (the inline(always) no-bit-change evidence). Run in f48fe7c74, 0da034dba
// and 1b5034c35 with DIGEST_PROBE_OUT=<file> [DIGEST_PROBE_MUTES=1] [DIGEST_PROBE_DECLINE=1], debug
// and --release; every comparable digest file was identical.
// ---- Sol attempt-2 verifier: render-bit digest probe (scratch) ------------------------------

fn digest_probe_session(
    draw: &mut Draw,
    output: bool,
    muted: &[bool],
) -> (SessionModel, Vec<(String, [Vec<f32>; 2])>) {
    let (mut model, source, track) = empty_session();
    let mut feeds = Vec::new();
    let mut routes = Vec::new();
    for (index, is_muted) in muted.iter().enumerate() {
        let id = format!("t{index:02}");
        add_track(&mut model, &source, &track, &id);
        let destination = if output {
            session::RouteDestination::OutputInput { output_id: sid("main-out") }
        } else {
            into_bus("b")
        };
        let mut drawn = route(&format!("r{index:02}"), post_pan(&id), destination);
        drawn.gain_db = draw.in_domain(-12.0, 6.0);
        let signs = [draw.chance(1, 2), draw.chance(1, 2), draw.chance(1, 2), draw.chance(1, 2)];
        drawn.channel_matrix = session::ChannelMatrix {
            ll: signed(draw, signs[0]),
            lr: signed(draw, signs[1]),
            rl: signed(draw, signs[2]),
            rr: signed(draw, signs[3]),
        };
        drawn.mute = *is_muted;
        feeds.push((id, planes(draw, Some([-0.0, 0.0]))));
        routes.push(drawn);
    }
    model.routes = routes;
    if !output {
        model.routes.push(to_output("b-main", RouteSource::Submix { submix_id: sid("b"), tap: SendTap::Input }));
        model.submixes = vec![session::Submix::unity(sid("b"), &model.console)];
    }
    (model, feeds)
}

fn digest_probe_fnv(planes: &[Vec<f32>; 2]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for sample in planes.iter().flatten() {
        for byte in sample.to_bits().to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    hash
}

#[test]
fn digest_probe_render_bits() {
    let with_mutes = std::env::var("DIGEST_PROBE_MUTES").is_ok();
    if std::env::var("DIGEST_PROBE_DECLINE").is_ok() {
        graph::test_only_set_route_fold_declined(true);
    }
    let mut lines = Vec::new();
    let mut draw = Draw::new(4_242);
    for count in 1..=12_usize {
        let mut patterns: Vec<Vec<bool>> = vec![vec![false; count]];
        if with_mutes && count >= 2 {
            patterns.push((0..count).map(|i| i % 3 == 1).collect());
            patterns.push((0..count).map(|i| i == 0 || i == count - 1).collect());
        }
        for pattern in patterns {
            for output in [false, true] {
                let (model, feeds) = digest_probe_session(&mut draw, output, &pattern);
                let (out, folds) = render(&document(&model), &feeds);
                lines.push(format!(
                    "count={count} output={output} muted={:?} folds={folds} digest={:016x}",
                    pattern.iter().map(|m| u8::from(*m)).collect::<Vec<_>>(),
                    digest_probe_fnv(&out)
                ));
            }
        }
    }
    let path = std::env::var("DIGEST_PROBE_OUT").expect("DIGEST_PROBE_OUT");
    std::fs::write(path, lines.join("\n") + "\n").expect("write digests");
}
