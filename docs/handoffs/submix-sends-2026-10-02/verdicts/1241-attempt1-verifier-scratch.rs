//! #1241 attempt 1 verifier probes (Sol). Not committed anywhere; reproduce by appending to the
//! named file of a `git archive` export of 210bd252f.
//!
//! Probe 1 -- append to `crates/capi/src/runtime/tests.rs`, run
//! `cargo test --locked -p capi --lib sol_c_abi -- --nocapture`. Result: ok, "SOL peaks open
//! 0.42117363 quiet 0.21108682" (0702 -6 dB -> x0.5012 through the C ABI's own store and
//! structural re-prepare; 0702 mute -> exact silence; 020f at the VCA ID -> not_found, revision
//! and snapshot unchanged).
//!
//! Probe 2 -- append to `crates/protocol/tests/vca_edits.rs`: out-of-range 0702/0700 are refused
//! at final validation (30 dB and -200 dB -> numeric.out_of_schema_range at
//! $.vcas[0].fader.{left,right}_db; NaN -> numeric.non_finite), revision/snapshot unchanged.

// ---- Sol scratch (#1241 verifier): VCA transactions through the C ABI's own store.
fn sol_tx(children: &mut CompiledChildren, request_id: u64, revision: u64, edits: &[protocol::SessionEdit]) -> (StatusCode, SessionRevision, Vec<String>) {
    let request = command_bytes_at_revision(request_id, ExpectedRevision::Exact(SessionRevision(revision)), protocol::CommandPayload::SessionTransactionApply(edits));
    let len = children.session.command(&request, 4_096).expect("command");
    let bytes = children.session.command_response(len).to_vec();
    let mut fields = [0_u16; 512];
    match ProtocolCodec::default().decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields)).expect("typed response") {
        protocol::DecodedTypedResponseFrame::Success { header, .. } => (header.status, header.revision, Vec::new()),
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => (header.status, header.revision, payload.diagnostics.iter().map(|d| d.code.clone()).collect()),
    }
}

fn sol_render(children: &mut CompiledChildren, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    children.plan.render(block * 128, PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("out")).expect("render");
    children.session.synchronize_plan_epochs().expect("sync");
    pcm
}

fn sol_after_set_fader(muted: bool, db: f32) -> Vec<f32> {
    let mut model = parse_session_json(SESSION).expect("base");
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let session = session::canonical_session_json(&model).expect("canonical");
    let mut children = compile_children(&session, limits()).expect("children");
    let id = |v: &str| session::StableId::parse(v).expect("id");
    let fader = |mute: bool, db: f32| session::DualMonoFader { left_db: db, right_db: db, left_mute: mute, right_mute: mute };
    let upsert = [protocol::SessionEdit::UpsertVca {
        vca: session::Vca { id: id("grp"), fader: fader(false, 0.0), members: vec![id("eq0")] },
    }];
    assert_eq!(sol_tx(&mut children, 1, 42, &upsert), (StatusCode::Ok, SessionRevision(43), vec![]));
    assert_eq!(children.session.controller.session().revision(), SessionRevision(43));
    let canonical = children.session.controller.session().canonical_snapshot().to_owned();
    assert_eq!(parse_session_json(&canonical).expect("snapshot").vcas.len(), 1);
    let _ = sol_render(&mut children, 0);

    let strip = [protocol::SessionEdit::SetTrackFader { track_id: id("grp"), fader: fader(true, 0.0) }];
    let (status, _, codes) = sol_tx(&mut children, 2, 43, &strip);
    assert_ne!(status, StatusCode::Ok);
    assert_eq!(codes, vec!["session.edit.not_found".to_owned()]);
    assert_eq!(children.session.controller.session().revision(), SessionRevision(43));
    assert_eq!(children.session.controller.session().canonical_snapshot(), canonical);

    let set = [protocol::SessionEdit::SetVcaFader { vca_id: id("grp"), fader: fader(muted, db) }];
    assert_eq!(sol_tx(&mut children, 3, 43, &set), (StatusCode::Ok, SessionRevision(44), vec![]));
    let _ = sol_render(&mut children, 1);
    let left = [0.5_f32; 128];
    let right = [-0.25_f32; 128];
    children.session.seek(b"fixture-source", 2, 256).expect("seek");
    for start in [256, 384] {
        children.session.submit(b"fixture-source", SourceSubmission { generation: 2, start_frame: start, sample_rate_hz: 48_000, planes: &[&left, &right], frames: 128, end_of_region: false }).expect("submit");
    }
    let mut out = sol_render(&mut children, 2);
    out.extend(sol_render(&mut children, 3));
    out
}

#[test]
fn sol_c_abi_vca_transactions_reach_the_store() {
    let open = sol_after_set_fader(false, 0.0);
    let peak = |v: &[f32]| v.iter().fold(0.0_f32, |a, s| a.max(s.abs()));
    assert!(peak(&open) > 0.0, "unmuted VCA passes audio");
    let quiet = sol_after_set_fader(false, -6.0);
    assert!(peak(&quiet) < peak(&open) * 0.6 && peak(&quiet) > 0.0, "{} vs {}", peak(&quiet), peak(&open));
    let muted = sol_after_set_fader(true, 0.0);
    assert!(muted.iter().all(|s| *s == 0.0), "VCA-muted plan is silent");
    eprintln!("SOL peaks open {} quiet {}", peak(&open), peak(&quiet));
}


// ---- Probe 2 (crates/protocol/tests/vca_edits.rs)
#[test]
fn sol_scratch_vca_fader_range_is_validated_with_the_transaction() {
    for (left, right) in [(30.0_f32, 0.0_f32), (0.0, -200.0), (f32::NAN, 0.0)] {
        let mut store = store();
        let revision = store.revision();
        let snapshot = store.canonical_snapshot().to_owned();
        let edits = [SessionEdit::SetVcaFader { vca_id: id("grp"), fader: fader(left, right, false, false) }];
        let error = store.apply_transaction(ExpectedRevision::Exact(revision), &edits).expect_err("out of range");
        let SessionStoreError::Validation { diagnostics, .. } = &error else { panic!("{error:?}") };
        let codes: Vec<_> = diagnostics.diagnostics().iter().map(|d| (d.code.as_str().to_owned(), d.path.to_string())).collect();
        eprintln!("SOL {left} {right}: {codes:?}");
        assert_eq!(store.revision(), revision);
        assert_eq!(store.canonical_snapshot(), snapshot);
    }
}
