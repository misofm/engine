// #1202 attempt 1 -- Sol verifier scratch. Not committed anywhere; for the root to adopt or drop.
//
// Part A: the proposed guard (MINOR-1). In `crates/graph-compiler/src/banks.rs`,
// `bind_rack_banks_indexed`, directly after the existing `unbanked_console_slot` call. The console
// population comes from preparation (`effects.entries` + `ids`), not from the strip walk that
// builds `chains`, so a console entry the walk never collected fails the compile instead of
// rendering per node. Evidence: with mutation E1 (the walk skips submix console racks) the commit
// compiles and every host-core gate stays green; with this guard the compile is refused with
// `console.slot.unbanked` and gate 1, gate 3, gate 7 (host-core half) and gate 8 go red. Without
// E1, `cargo test -p graph-compiler -p host-core --all-targets` (test-support features) is green:
// 25 binaries, 288 passed. Prefer `level_by_node.get(node)` -> `graph.internal.invariant` over
// the `u64::MAX` placeholder below if you adopt it.
//
//    let collected: BTreeSet<&EffectNodeId> = chains.values().flatten().collect();
//    for (index, entry) in effects.entries.iter().enumerate() {
//        if !is_console_rack(rack_location(crate::ids::rack_id(entry.rack))) {
//            continue;
//        }
//        let Some(node) = ids.get(index).and_then(Option::as_ref) else {
//            return Err(diag("graph.internal.invariant", "$.effects"));
//        };
//        if !collected.contains(node) {
//            let section = if node.rack == RackId::Simd1 { "pre_insert" } else { "post_insert" };
//            let pool = match classes.class_of(node.track_id.as_str()) {
//                CohortPoolClass::MonoSymmetricAtPrepare => "mono",
//                CohortPoolClass::Stereo => "stereo",
//            };
//            let level = level_by_node.get(node).copied().unwrap_or(u64::MAX);
//            return Err(diag(
//                "console.slot.unbanked",
//                &format!(
//                    "$.console.{section}[slot={}].bank[pool={pool},level={level}]",
//                    node.effect_id.as_str()
//                ),
//            ));
//        }
//    }
//
// Part B (MINOR-2, crates/graph-compiler/tests/bank_levels.rs): D3's mixed group. A fifth track
// `chu`, not routed to the bus, whose 11 inserts put its post_insert limiter at the bus limiter's
// level 17 (Stereo). Green on b0c7e29fb at W8 and at a simulated native W4.

/// VERIFIER E4: a console group that holds a track lane and a bus lane (D3) binds and renders the
/// scalar bits at every width that compiles.
#[test]
fn verifier_mixed_track_and_bus_console_group() {
    let mut found = 0;
    for inserts in 0..16 {
        let mut model = bus_console_session(1, 1, 99);
        let templates = Templates::of(&model);
        let mut late = model.tracks[0].clone();
        late.id = sid("chu");
        late.inserts.effects = (0..inserts)
            .map(|index| templates.effect(Kind::SoftClip, &format!("sc{index}")))
            .collect();
        model.tracks.push(late);
        model.routes.push(route(
            "chu-main",
            RouteSource::Track {
                track_id: sid("chu"),
                tap: SendTap::PostPan,
            },
            RouteDestination::OutputInput {
                output_id: sid("main-out"),
            },
            0.0,
        ));
        let native = compile_bind_render(&model, Backend::current(), BLOCKS, Collapse::Unarmed);
        let mixed: Vec<_> = native
            .console_groups
            .iter()
            .filter(|((slot, _, level), [lanes, _])| {
                native.bus_console_levels.contains(&(slot.clone(), *level)) && *lanes >= 2
            })
            .collect();
        eprintln!(
            "inserts {inserts}: compile {:?} bind {:?} mixed {mixed:?} bus levels {:?}",
            native.compile, native.bind, native.bus_console_levels
        );
        if !mixed.is_empty() {
            found += 1;
            let (outcomes, _) =
                assert_binds_and_renders_the_scalar_bits(&format!("mixed {inserts}"), &model);
            let native = &outcomes[native_index() + 1];
            for ((slot, class, level), [lanes, banks]) in &native.console_groups {
                assert_eq!(
                    *banks,
                    lanes.div_ceil(Backend::current().width()),
                    "({slot}, {class}, {level})"
                );
            }
        }
    }
    assert!(found > 0, "no mixed group formed");
}

// Part C (evidence only, crates/host-core/tests/submix_strip.rs): a console bank holding a live
// track lane and a channel-less bus lane renders, with live controls attached and no record
// pushed, the bits of the plan prepared without live controls. Green on b0c7e29fb (6 seeds, the
// mixed bank forms at 13 track inserts).
fn verifier_render_prepared(
    prepared: &mut PreparedHost,
    feeds: &[(&str, &[Vec<f32>; 2])],
    blocks: usize,
) -> [Vec<f32>; 2] {
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..blocks {
        submit(prepared, feeds, block);
        let mut samples = [0.0_f32; QUANTUM * 2];
        prepared
            .plan
            .render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(
                        &mut samples,
                        2,
                        QUANTUM,
                        QUANTUM,
                    )
                    .expect("output planes"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        out[0].extend_from_slice(&samples[..QUANTUM]);
        out[1].extend_from_slice(&samples[QUANTUM..]);
    }
    out
}

/// VERIFIER E3: a console bank holding a live track lane and a channel-less bus lane renders, with
/// live controls attached and no record pushed, the bits of the plan prepared without them.
#[test]
fn verifier_live_controlled_mixed_bank_renders_the_unlive_bits() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut mixed_seen = 0;
    for seed in 0..6_u64 {
        for inserts in 0..24 {
            let mut draw = Draw::new(seed);
            let (bus, feeds, _, _) = gate_one_sessions(&mut draw, &registry);
            let mut model = parse_session_json(&bus).expect("A parses");
            let mut late = model.tracks[0].clone();
            late.id = sid("u");
            for entry in &mut late.console {
                entry.bypass = false;
            }
            late.inserts.effects = (0..inserts)
                .map(|index| {
                    let mut eq = drawn_effect(
                        &mut draw,
                        &registry,
                        "miso.parametric-eq",
                        &format!("ueq{index}"),
                        LinkMode::DualMono,
                    );
                    eq.params.clear();
                    eq
                })
                .collect();
            model.tracks.push(late);
            model.routes.push(to_output("u-main", post_pan("u")));
            let doc = document(&model);
            let artifact = graph_artifact(&doc, 7);
            let report = &artifact.report().rack_cohorts;
            let mixed = report.plan.groups.iter().any(|group| {
                let owners: Vec<&str> = group
                    .members
                    .iter()
                    .flatten()
                    .map(|chain| chain.track_id.as_str())
                    .collect();
                matches!(format!("{:?}", group.rack).as_str(), "Simd1" | "Simd2")
                    && owners.contains(&"u")
                    && owners.contains(&"bus")
            });
            if !mixed {
                continue;
            }
            mixed_seen += 1;
            let borrowed: Vec<(&str, &[Vec<f32>; 2])> = feeds
                .iter()
                .map(|(id, planes)| (id.as_str(), planes))
                .collect();
            let plain = render(&doc, &borrowed, GATE_ONE_BLOCKS);
            let (_, mut prepared, handles) = prepare_host_session_with_live_controls(
                &doc,
                &caps(),
                &HostLiveControlRequest {
                    control_queue_depth: Some(NonZeroUsize::new(64).expect("depth")),
                    meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
                    meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
                    meter_tap: MeterTap::PostMatrix,
                    observation_taps: 4,
                    master_track: None,
                },
            )
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
            assert!(
                handles.effect_controls.iter().any(|c| &*c.track_id == "u"),
                "u is live"
            );
            let live = verifier_render_prepared(&mut prepared, &borrowed, GATE_ONE_BLOCKS);
            let audible = plain[0].iter().any(|s| *s != 0.0);
            for plane in 0..2 {
                if let Some(index) = first_difference(&live[plane], &plain[plane]) {
                    panic!(
                        "seed {seed} inserts {inserts}: plane {plane} sample {index}: live {:?} != plain {:?}",
                        live[plane][index], plain[plane][index]
                    );
                }
            }
            eprintln!("seed {seed} inserts {inserts}: mixed bank, live == plain, audible {audible}");
        }
    }
    assert!(mixed_seen > 0, "no mixed group");
}
