// #1207 attempt 1 -- Sol verifier probes. NOT committed anywhere; each was appended temporarily to
// an exported copy of 7bdb45187 (/tmp/claude-1002/v1207/src), run, and removed (files restored and
// compared with `cmp` against `git show 7bdb45187:<path>`).
//
// ---------------------------------------------------------------------------------------------
// (1) appended to hosts/host-web/src/tests.rs. Result: PASS.
//     in_flight=18 wanted=18 effect_controls=12 effect_base=[0,3,6,9] rack_effects=[[1,1,1]; 4]
//     obs_tracks=[MAX,0,0,MAX,MAX,1,MAX,2,2,MAX,3,3] present=4
//     bindings report track_index 2 and 3 for the buses (accepted hazard)
//     copy_eq_target_config(2, 3, 0) -> 0 (RESULT_OK, bus EQ; accepted hazard); (4, 3, 0) -> 1
//     poll_meters: 0 allocations / 0 frees on each of 8 polls; frame master word stays 0.0 with a
//     force-armed bus compressor publishing GR 8.69 dB. With D4 removed (mutation M2) the master
//     word becomes 8.689951 -- only this probe sees it.
// ---------------------------------------------------------------------------------------------
#[test]
fn verifier_probe_1207_bus_session() {
    const QUANTUM: u32 = 128;
    let mut host = bus_effect_host(QUANTUM);
    {
        let ready = host.ready.as_ref().expect("ready");
        assert_eq!(ready.in_flight.len(), 3 * 2 + 12);
        assert_eq!(ready.command_wanted.len(), 3 * 2 + 12);
    }
    for bus in [2_u32, 3] {
        stage_command(&mut host, 0, COMMAND_EFFECT_BYPASS, 1, 255, bus, 0, 0, 0, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT, "bus {bus}");
        assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TRACK, "bus {bus}");
        assert_eq!(observe(&mut host, bus, 1, 0, 1, 2, true), RESULT_INVALID_ARGUMENT);
    }
    for index in 0..host.observation_binding_count() as u32 {
        let binding = host.observation_binding(index).expect("binding");
        eprintln!("PROBE binding {index}: track_index={}", binding.address.track_index);
    }
    eprintln!("PROBE eq config bus 2 console 0 -> {}", host.copy_eq_target_config(2, 3, 0));
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    {
        // Force-arm aaa-bus's compressor past the per-track guard, to exercise D4.
        let ready = host.ready.as_mut().expect("ready");
        let slot = dense_effect_slot(
            ready.effect_base[2],
            ready.rack_effects[2],
            LiveEffectAddress { rack: LiveEffectRack::Inserts, index: 0 },
        )
        .expect("bus slot");
        assert_eq!(ready.observation_tracks[slot], 2);
        ready.effect_controls[slot]
            .as_mut()
            .expect("bus producer")
            .try_push(EffectControlRecord::Observe { tap_index: 0, armed: true, window_blocks: 2 })
            .expect("push");
        ready.observation_armed[slot] = 1;
    }
    let mut polled = 0;
    for block in 0..8 {
        feed_and_render_tracks(&mut host, block, 0.9);
        let (count, allocations, deallocations) =
            crate::ffi::live_response_ffi_tests::measured(|| host.poll_meters());
        assert_eq!(allocations, 0, "poll allocated");
        assert_eq!(deallocations, 0, "poll freed");
        polled += count;
    }
    assert!(polled > 0, "a window closed");
    let frame = host.meter_frame();
    assert_eq!(frame[2 * 2 + 2 + 2], 0.0, "master GR word untouched by a bus");
}

// ---------------------------------------------------------------------------------------------
// (2) appended to crates/host-core/tests/strip_handles.rs. Results: all PASS.
//     a) a live-controlled bus session (2 console slots on every strip, a bypassed bus limiter,
//        a compressor insert per bus; control depth 16, meters, 1 observation tap) renders the
//        plain `prepare_host_runtime` bits for 24 blocks;
//     b) live `Bypass(true)` pushed to aaa-bus's `bus-comp` producer and zzz-bus's `desk-eq`
//        producer renders the session-bypassed oracle bit-exactly (audible vs plain);
//     c) live `Bypass(true)` pushed to aaa-bus's banked console limiter (`desk-limit`, Console 1)
//        renders the session-bypassed oracle bit-exactly; 4200 samples differ from plain.
// ---------------------------------------------------------------------------------------------
fn verifier_document() -> String {
    use session::{ConsoleEntry, ConsoleSlot, EffectIdentity, EffectQuality, LinkMode};
    let mut model = parse_session_json(FIXTURE).expect("observation fixture parses");
    let slot = |slot: &str, effect: &str| ConsoleSlot {
        slot: StableId::parse(slot).expect("slot id"),
        identity: EffectIdentity::Native { effect_id: StableId::parse(effect).expect("effect id") },
        quality: EffectQuality::Normal,
        link_mode: LinkMode::DualMono,
    };
    model.console.pre_insert = vec![slot("desk-eq", "miso.parametric-eq")];
    model.console.post_insert = vec![slot("desk-limit", "miso.true-peak-limiter")];
    let slots: Vec<_> = model.console.slots().map(|s| s.slot.clone()).collect();
    for track in &mut model.tracks {
        track.console = slots
            .iter()
            .map(|s| ConsoleEntry { slot: s.clone(), bypass: false, params: Vec::new() })
            .collect();
    }
    let compressor = model.tracks[0].inserts.effects[0].clone();
    for (bus, feeder, bypass_limit) in [("zzz-bus", 0_usize, true), ("aaa-bus", 1, false)] {
        let id = StableId::parse(bus).expect("bus id");
        let mut submix = Submix::unity(id.clone(), &model.console);
        for entry in &mut submix.console {
            entry.bypass = false;
        }
        submix.console[1].bypass = bypass_limit;
        let mut insert = compressor.clone();
        insert.id = StableId::parse("bus-comp").expect("insert id");
        submix.inserts.effects.push(insert);
        model.submixes.push(submix);
        let mut out = model.routes[feeder].clone();
        model.routes[feeder].destination = RouteDestination::SubmixInput { submix_id: id.clone() };
        out.id = StableId::parse(&format!("{bus}-main")).expect("route id");
        out.source = RouteSource::Submix { submix_id: id, tap: SendTap::PostPan };
        model.routes.push(out);
    }
    canonical_session_json(&model).expect("canonical bus session")
}

fn verifier_render(prepared: &mut host_core::PreparedHost, blocks: usize) -> Vec<f32> {
    let q = QUANTUM as usize;
    let mut out = Vec::new();
    for block in 0..blocks {
        let left: Vec<f32> = (0..q).map(|i| (((block * q + i) as f32) * 0.37).sin() * 1.9).collect();
        let right: Vec<f32> = (0..q).map(|i| (((block * q + i) as f32) * 0.11).cos() * 1.7).collect();
        prepared
            .sources
            .submit(b"fixture-source", host_core::SourceSubmission {
                generation: 1,
                start_frame: (block * q) as u64,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: QUANTUM,
                end_of_region: false,
            })
            .expect("source block");
        let mut samples = vec![0.0_f32; q * 2];
        prepared
            .plan
            .render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, q, q)
                        .expect("planes"),
                },
                engine::realtime::RenderTime { absolute_sample: (block * q) as u64 },
            )
            .expect("render");
        out.extend_from_slice(&samples);
    }
    out
}

// (c); (a) and (b) follow the same shape: prepare plain + oracle with `prepare_host_runtime`,
// prepare live with `prepare_host_session_with_live_controls`, push records to the named bus
// producers, render 24 blocks each, compare `to_bits`.
#[test]
fn verifier_probe_bus_console_producer_drives_its_own_bank_lane() {
    let mut model = parse_session_json(&verifier_document()).expect("parse");
    let aaa = model.submixes.iter().position(|s| s.id.as_str() == "aaa-bus").unwrap();
    model.submixes[aaa].inserts.effects[0].bypass = true;
    for track in &mut model.tracks {
        for effect in &mut track.inserts.effects {
            effect.bypass = true;
        }
        for entry in &mut track.console {
            entry.bypass = true;
        }
    }
    let document = canonical_session_json(&model).expect("doc");
    model.submixes[aaa].console[1].bypass = true;
    let oracle_doc = canonical_session_json(&model).expect("oracle");
    let mut oracle = host_core::prepare_host_runtime(
        &host_core::compile_host_session(&oracle_doc, &caps()).unwrap(),
        &caps(),
    )
    .unwrap();
    let mut plain = host_core::prepare_host_runtime(
        &host_core::compile_host_session(&document, &caps()).unwrap(),
        &caps(),
    )
    .unwrap();
    let (_, mut live, mut handles) = prepare_host_session_with_live_controls(
        &document,
        &caps(),
        &HostLiveControlRequest {
            control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
            meter_period_frames: None,
            meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
            meter_tap: MeterTap::PostMatrix,
            observation_taps: 0,
            master_track: None,
        },
    )
    .unwrap();
    for producer in &mut handles.effect_controls {
        if &*producer.track_id == "aaa-bus" && &*producer.effect_id == "desk-limit" {
            producer
                .try_push(effect_contract::EffectControlRecord::Bypass(true))
                .expect("push");
        }
    }
    let a = verifier_render(&mut oracle, 24);
    let b = verifier_render(&mut live, 24);
    let c = verifier_render(&mut plain, 24);
    assert!(a.iter().zip(&c).any(|(x, y)| x.to_bits() != y.to_bits()), "audible");
    assert_eq!(a.iter().zip(&b).position(|(x, y)| x.to_bits() != y.to_bits()), None);
}
