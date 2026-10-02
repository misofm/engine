// #1204 attempt 1 -- Sol verifier scratch tests (never committed; run in a git-archive export of 7f767ae0c).
// Part 1 was appended to crates/protocol/tests/submix_strip_edits.rs; part 2 to crates/capi/src/runtime/tests.rs.

// ===== Part 1: crates/protocol/tests/submix_strip_edits.rs =====
// ---- Sol scratch (#1204 verification), never committed ----
#[test]
fn sol_mixed_batch_failure_commits_nothing() {
    for failing in [
        SessionEdit::SetTrackFader { track_id: id("nope"), fader: fader(-1.0) },
        SessionEdit::SetTrackConsole { track_id: id("bus"), console: Vec::new() }, // final validation
        SessionEdit::RemoveEffectParam { track_id: id("bus"), rack_name: RackName::Console, effect_id: id("eq"), parameter_id: 99, channel: ParameterChannel::Both },
    ] {
        let mut store = store();
        let snapshot = store.canonical_snapshot().to_owned();
        let edits = through_the_wire(&[
            SessionEdit::SetTrackFader { track_id: id("bus"), fader: fader(-6.0) },
            SessionEdit::SetEffectBypass { track_id: id("bus"), rack_name: RackName::Console, effect_id: id("eq"), bypass: true },
            SessionEdit::PutTrackEffect { track_id: id("a"), rack_name: RackName::Inserts, final_position: 0, effect: limiter() },
            failing.clone(),
        ]);
        let error = store.apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &edits).expect_err("refused");
        println!("SOL mixed failure {:?} -> {error:?}", failing.opcode());
        assert_eq!(store.revision(), SessionRevision(7));
        assert_eq!(store.canonical_snapshot(), snapshot);
    }
}

#[test]
fn sol_snapshot_round_trips_after_every_submix_edit() {
    let all: Vec<SessionEdit> = cases().into_iter().map(|case| case.1).collect();
    // one transaction with every case except the rack replacement, which would remove eq/comp
    let mut store = store();
    for (name, edit, _) in cases() {
        let mut one = store_at(&store);
        let _ = name;
        one.apply_transaction(ExpectedRevision::Exact(one.revision()), &[edit]).expect("commit");
        let snapshot = one.canonical_snapshot().to_owned();
        let reparsed = parse_session_json(&snapshot).expect("snapshot parses");
        assert_eq!(&reparsed, one.compiled().normalized_model());
        assert_eq!(canonical_session_json(&reparsed).expect("canonical"), snapshot);
    }
    let _ = all;
    store.apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &[SessionEdit::SetTrackFader { track_id: id("bus"), fader: fader(-6.0) }]).expect("ok");
}

fn store_at(_store: &SessionStore) -> SessionStore { store() }

#[test]
fn sol_output_id_refuses_every_strip_opcode() {
    for (name, mut edit, _) in cases() {
        set_strip(&mut edit, id("main-out"));
        let mut store = store();
        let error = store.apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &[edit]).expect_err(name);
        assert_eq!(error, SessionStoreError::Edit { operation_index: 0, error: SessionEditError::NotFound }, "{name}");
    }
}

fn set_strip(edit: &mut SessionEdit, strip: StableId) {
    match edit {
        SessionEdit::SetTrackBuiltins { track_id, .. }
        | SessionEdit::SetTrackRack { track_id, .. }
        | SessionEdit::PutTrackEffect { track_id, .. }
        | SessionEdit::RemoveTrackEffect { track_id, .. }
        | SessionEdit::SetTrackEffectOrder { track_id, .. }
        | SessionEdit::SetEffectIdentity { track_id, .. }
        | SessionEdit::SetEffectQuality { track_id, .. }
        | SessionEdit::SetEffectBypass { track_id, .. }
        | SessionEdit::SetEffectLinkMode { track_id, .. }
        | SessionEdit::SetEffectSidechain { track_id, .. }
        | SessionEdit::UpsertEffectParam { track_id, .. }
        | SessionEdit::RemoveEffectParam { track_id, .. }
        | SessionEdit::SetTrackFader { track_id, .. }
        | SessionEdit::SetTrackMatrixOrPan { track_id, .. }
        | SessionEdit::SetTrackConsole { track_id, .. } => *track_id = strip,
        _ => unreachable!(),
    }
}

#[test]
fn sol_track_and_submix_cannot_share_an_id_at_commit() {
    let mut store = store();
    let unity = Submix::unity(id("a"), &base().console);
    let error = store.apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &[SessionEdit::UpsertSubmix { submix: unity.clone() }]).expect_err("shared id");
    println!("SOL shared id -> {error:?}");
    // output-ID shadowing: upsert a submix named like the output
    let unity_out = Submix::unity(id("main-out"), &base().console);
    let error = store.apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &[SessionEdit::UpsertSubmix { submix: unity_out }, SessionEdit::SetTrackFader { track_id: id("main-out"), fader: fader(-6.0) }]).expect_err("shared id with output");
    println!("SOL shared output id -> {error:?}");
    assert_eq!(store.revision(), SessionRevision(7));
}

// ===== Part 2: crates/capi/src/runtime/tests.rs =====
// ---- Sol scratch (#1204 verification), never committed ----
fn sol_bus_session(bus_fader_db: f32) -> String {
    let mut model = parse_session_json(SESSION).expect("fixture");
    model.tracks.truncate(1);
    model.routes.clear();
    let bus = session::StableId::parse("bus").expect("bus");
    let mut submix = session::Submix::unity(bus.clone(), &model.console);
    submix.fader.left_db = bus_fader_db;
    submix.fader.right_db = bus_fader_db;
    model.submixes = vec![submix];
    let unit = session::ChannelMatrix { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 };
    model.routes = vec![
        session::Route {
            id: session::StableId::parse("eq0-bus").expect("id"),
            source: session::RouteSource::Track { track_id: model.tracks[0].id.clone(), tap: session::SendTap::PostPan },
            destination: session::RouteDestination::SubmixInput { submix_id: bus.clone() },
            channel_matrix: unit.clone(),
            gain_db: 0.0,
        },
        session::Route {
            id: session::StableId::parse("bus-out").expect("id"),
            source: session::RouteSource::Submix { submix_id: bus, tap: session::SendTap::PostPan },
            destination: session::RouteDestination::OutputInput { output_id: session::StableId::parse("main-out").expect("id") },
            channel_matrix: unit,
            gain_db: 0.0,
        },
    ];
    session::canonical_session_json(&model).expect("canonical")
}

#[test]
fn sol_submix_fader_edit_through_the_c_abi() {
    let base = sol_bus_session(0.0);
    let edited = sol_bus_session(-6.0);
    let mut unedited_direct = compile_children(&base, limits()).expect("unedited");
    let mut edited_direct = compile_children(&edited, limits()).expect("edited");
    let (c_session, c_plan) = boxed_c_children(&base);
    let rev = |session: &str| parse_session_json(session).expect("p").revision;
    let base_rev = rev(&base);
    let edits = [protocol::SessionEdit::SetTrackFader {
        track_id: session::StableId::parse("bus").expect("bus"),
        fader: session::DualMonoFader { left_db: -6.0, right_db: -6.0, left_mute: false, right_mute: false },
    }];
    let request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(base_rev)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    let (result, response) = command_c(c_session, &request);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let decoded = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("response");
    println!("SOL response: success={}", matches!(decoded, protocol::DecodedTypedResponseFrame::Success { .. }));
    assert!(matches!(decoded, protocol::DecodedTypedResponseFrame::Success { header, .. } if header.revision == SessionRevision(base_rev + 1)));
    let (event_result, event) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
    assert_eq!(event_result, crate::RESULT_OK);
    let mut efields = [0_u16; 64];
    let decoded_event = ProtocolCodec::default().decode_typed_event(&event, &mut DecodeScratch::new(&mut efields)).expect("event");
    println!("SOL event: committed={} rev={:?}", matches!(decoded_event.payload, protocol::DecodedEventPayload::SessionCommitted(_)), decoded_event.header.revision);
    let quantum = 128_usize;
    let sr = 48_000_u32;
    for block in 0..8_u64 {
        // Feed a 1 kHz sine one block ahead, same submission to all three.
        if block == 1 {
            // the replacement boundary resets the C source state; re-seek every side to generation 2
            for direct in [&mut unedited_direct, &mut edited_direct] {
                direct.session.seek(b"fixture-source", 2, block * quantum as u64).expect("seek");
            }
            assert_eq!(crate::ffi::test_source_seek(c_session, b"fixture-source", 2, block * quantum as u64), crate::RESULT_OK);
        }
        let generation = if block == 0 { 1 } else { 2 };
        let start = block * quantum as u64;
        let left: Vec<f32> = (0..quantum).map(|i| 0.5 * (2.0 * core::f32::consts::PI * 1000.0 * (start + i as u64) as f32 / sr as f32).sin()).collect();
        let right: Vec<f32> = left.iter().map(|v| -0.5 * v).collect();
        for (index, direct) in [&mut unedited_direct, &mut edited_direct].into_iter().enumerate() {
            if index == 1 && block == 0 { continue; } // mirror the C side's reset at the boundary
            direct.session.submit(b"fixture-source", SourceSubmission { generation, start_frame: start, sample_rate_hz: sr, planes: &[&left, &right], frames: quantum as u32, end_of_region: false }).expect("direct submit");
        }
        submit_c(c_session, generation, start, sr, &left, &right, false);
        let mut un = vec![f32::NAN; quantum * 2];
        unedited_direct.plan.render(start, PlanarBufferMut::try_new(&mut un, 2, quantum, quantum).expect("o")).expect("r");
        let mut ed = vec![f32::NAN; quantum * 2];
        edited_direct.plan.render(start, PlanarBufferMut::try_new(&mut ed, 2, quantum, quantum).expect("o")).expect("r");
        let mut c_pcm = vec![f32::NAN; quantum * 2];
        let output = crate::PlanarOutput { struct_size: crate::PLANAR_OUTPUT_SIZE, channels: 2, samples: c_pcm.as_mut_ptr(), sample_capacity: c_pcm.len() as u64, frames: quantum as u32, plane_stride_samples: quantum as u32, reserved: [0; 2] };
        assert_eq!(crate::ffi::test_render(c_plan, start, &output), crate::RESULT_OK);
        let rms = |v: &[f32]| (v.iter().map(|x| x * x).sum::<f32>() / v.len() as f32).sqrt();
        let bitwise = c_pcm.iter().zip(&ed).all(|(a, b)| a.to_bits() == b.to_bits());
        println!("SOL block {block}: C rms {:.6} edited {:.6} unedited {:.6} C==edited bitwise {bitwise} C/unedited dB {:.3}", rms(&c_pcm), rms(&ed), rms(&un), 20.0 * (rms(&c_pcm) / rms(&un)).log10());
    }
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn sol_mixed_failing_submix_batch_through_the_c_abi_acks_nothing() {
    let base = sol_bus_session(0.0);
    let (c_session, c_plan) = boxed_c_children(&base);
    let base_rev = parse_session_json(&base).expect("p").revision;
    let fader = session::DualMonoFader { left_db: -6.0, right_db: -6.0, left_mute: false, right_mute: false };
    let edits = [
        protocol::SessionEdit::SetTrackFader { track_id: session::StableId::parse("bus").expect("bus"), fader: fader.clone() },
        protocol::SessionEdit::SetTrackFader { track_id: session::StableId::parse("eq0").expect("t"), fader: fader.clone() },
        protocol::SessionEdit::SetTrackSourceAssignment { track_id: session::StableId::parse("bus").expect("bus"), source_id: session::StableId::parse("fixture-source").expect("s"), left_source_channel: 0, right_source_channel: 1 },
    ];
    let request = command_bytes_at_revision(1, ExpectedRevision::Exact(SessionRevision(base_rev)), protocol::CommandPayload::SessionTransactionApply(&edits));
    let (result, response) = command_c(c_session, &request);
    println!("SOL failing batch result {result}");
    let mut fields = [0_u16; 64];
    let decoded = ProtocolCodec::default().decode_typed_response(&response, &mut DecodeScratch::new(&mut fields)).expect("response");
    match decoded {
        protocol::DecodedTypedResponseFrame::Success { .. } => panic!("acked a failing batch"),
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => {
            println!("SOL non-ok header rev {:?} payload {:?}", header.revision, payload);
        }
    }
    let (event_result, event) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
    println!("SOL reliable lane after refusal: result {event_result} bytes {}", event.len());
    // The same revision still accepts a valid transaction: nothing advanced.
    let ok = command_bytes_at_revision(2, ExpectedRevision::Exact(SessionRevision(base_rev)), protocol::CommandPayload::SessionTransactionApply(&edits[..2]));
    let (result, response) = command_c(c_session, &ok);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    assert!(matches!(ProtocolCodec::default().decode_typed_response(&response, &mut DecodeScratch::new(&mut fields)).expect("r"), protocol::DecodedTypedResponseFrame::Success { header, .. } if header.revision == SessionRevision(base_rev + 1)));
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}
