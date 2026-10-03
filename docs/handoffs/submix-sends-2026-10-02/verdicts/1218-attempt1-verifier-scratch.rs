// #1218 attempt 1 verifier probes. Appended to crates/host-core/tests/route_mute.rs of an export of
// 0da034dba (and, for verifier_gate3_dump, of its parent 071c6c14a; run with G3_OUT=<file> and cmp).
// Not for commit.
// ---- verifier scratch (#1218 attempt 1) ----
#[test]
fn verifier_post_fader_follow_bits() {
    // Track `t`, both lanes muted, sends post_fader into `b` with negative coefficients; compare
    // follow vs no-follow output bits (sign of zero).
    for tap in [SendTap::PostFader, SendTap::PostPan] {
        for both in [true, false] {
            let build = |follows: bool| {
                let (mut model, source_pcm, track) = empty_session();
                add_track(&mut model, &source_pcm, &track, "t");
                model.tracks[0].fader.left_mute = true;
                model.tracks[0].fader.right_mute = both;
                let mut send = route(
                    "send",
                    RouteSource::Track {
                        track_id: sid("t"),
                        tap,
                    },
                    into_bus("b"),
                );
                send.channel_matrix = ChannelMatrix {
                    ll: -0.5,
                    lr: -0.25,
                    rl: -0.75,
                    rr: -1.0,
                };
                send.follows_mute = follows;
                model.routes.push(send);
                model.routes.push(to_output("b-main", bus_input("b")));
                model.submixes.push(Submix::unity(sid("b"), &model.console));
                model
            };
            let mut draw = Draw::new(77);
            let feeds = vec![("t".to_owned(), planes(&mut draw, None))];
            let (a, _) = render(&document(&build(true)), &feeds);
            let (b, _) = render(&document(&build(false)), &feeds);
            let mut diffs = 0usize;
            let mut sign_only = 0usize;
            for plane in 0..2 {
                for (x, y) in a[plane].iter().zip(b[plane].iter()) {
                    if x.to_bits() != y.to_bits() {
                        diffs += 1;
                        if *x == 0.0 && *y == 0.0 {
                            sign_only += 1;
                        }
                    }
                }
            }
            eprintln!(
                "VERIFIER tap {tap:?} both_muted {both}: differing samples {diffs}, of which signed-zero only {sign_only}, total {}",
                a[0].len() * 2
            );
        }
    }
}

// ---- verifier scratch: #1218 gate 3 (no bit moved without a follow) ----
#[test]
fn verifier_gate3_dump() {
    let (mut model, source_pcm, track) = empty_session();
    for id in ["t0", "t1", "t2"] {
        add_track(&mut model, &source_pcm, &track, id);
    }
    model.tracks[0].fader.left_mute = true;
    model.tracks[0].fader.right_mute = true;
    model.tracks[1].fader.left_mute = true;
    model.tracks[2].inserts.effects.push(Effect {
        id: sid("ceiling"),
        identity: EffectIdentity::Native {
            effect_id: sid("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    let mut routes = Vec::new();
    for (i, id) in ["t0", "t1", "t2"].iter().enumerate() {
        routes.push(to_output(&format!("{id}-main"), post_pan(id)));
        let mut send = route(
            &format!("{id}-verb"),
            RouteSource::Track {
                track_id: sid(id),
                tap: SendTap::PreFader,
            },
            into_bus("verb"),
        );
        send.channel_matrix = ChannelMatrix {
            ll: -0.5 + i as f32 * 0.1,
            lr: 0.25,
            rl: -0.75,
            rr: 0.9,
        };
        send.gain_db = -3.0;
        routes.push(send);
    }
    routes.push(route(
        "verb-echo",
        RouteSource::Submix {
            submix_id: sid("verb"),
            tap: SendTap::Input,
        },
        into_bus("echo"),
    ));
    routes.push(to_output("verb-main", bus_output("verb")));
    routes.push(to_output("echo-main", bus_output("echo")));
    model.routes = routes;
    let mut verb = Submix::unity(sid("verb"), &model.console);
    verb.fader.right_mute = true;
    model.submixes = vec![Submix::unity(sid("echo"), &model.console), verb];
    let mut draw = Draw::new(4242);
    let feeds: Vec<(String, [Vec<f32>; 2])> = ["t0", "t1", "t2"]
        .iter()
        .map(|id| ((*id).to_owned(), planes(&mut draw, None)))
        .collect();
    let (out, _) = render(&document(&model), &feeds);
    let mut bytes = Vec::new();
    for plane in &out {
        for s in plane {
            bytes.extend_from_slice(&s.to_bits().to_le_bytes());
        }
    }
    let nonzero = out.iter().flatten().filter(|s| **s != 0.0).count();
    eprintln!("VERIFIER g3 samples {} nonzero {nonzero}", out[0].len() * 2);
    std::fs::write(std::env::var("G3_OUT").expect("G3_OUT"), bytes).expect("write");
}
