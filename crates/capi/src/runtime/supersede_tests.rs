//! #1310: a structural transaction submitted while an earlier one's candidate plan waits
//! unadopted commits and supersedes it.

use super::*;
use crate::runtime::tests::{
    command_bytes_at_revision, limits, render_direct, stateless_session, submit_direct,
};
use protocol::{ExpectedRevision, SessionEdit, SessionRevision};
use session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, RackName, RouteSource, SessionModel,
    SidechainDeclaration, StableId,
};

const FIXTURE: &[u8] = b"fixture-source";
const ADDED: &[u8] = b"s-source";

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// A native insert `instance` of `effect` with its defaults.
fn insert(instance: &str, effect: &str) -> Effect {
    Effect {
        id: id(instance),
        identity: EffectIdentity::Native {
            effect_id: id(effect),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

fn compressor() -> Effect {
    insert("comp", "miso.compressor")
}

/// The compressor's replacement under the same insert ID: a prepared change that restarts the
/// insert. No launch effect declares a second quality, so a quality edit does not prepare (#1344).
fn gate() -> Effect {
    insert("comp", "miso.gate-expander")
}

fn put(model: &SessionModel, track: usize, effect: Effect) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: model.tracks[track].id.clone(),
        rack_name: RackName::Inserts,
        final_position: 0,
        effect,
    }
}

fn remove(model: &SessionModel, track: usize) -> SessionEdit {
    SessionEdit::RemoveTrackEffect {
        track_id: model.tracks[track].id.clone(),
        rack_name: RackName::Inserts,
        effect_id: id("comp"),
    }
}

/// `UpsertSource` of `s-source`, `UpsertTrack` of track `ts` on it (a copy of the first track
/// with no insert) and `UpsertRoute` of its route.
fn add_source_track(model: &SessionModel) -> [SessionEdit; 3] {
    let mut source = model.sources[0].clone();
    source.id = id("s-source");
    let mut track = model.tracks[0].clone();
    track.id = id("ts");
    track.source_id = source.id.clone();
    track.inserts.effects.clear();
    let mut route = model.routes[0].clone();
    route.id = id("ts-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        SessionEdit::UpsertSource { source },
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn transaction(request_id: u64, revision: u64, edits: &[SessionEdit]) -> Vec<u8> {
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(revision)),
        protocol::CommandPayload::SessionTransactionApply(edits),
    )
}

/// Commits `edits` at `base` and drains the reliable events, so a later transaction never meets a
/// full event lane. Panics unless the transaction commits as revision `base + 1`.
fn commit(children: &mut CompiledChildren, request_id: u64, base: u64, edits: &[SessionEdit]) {
    let length = children
        .session
        .command(&transaction(request_id, base, edits), 4_096)
        .unwrap_or_else(|error| panic!("request {request_id}: {error:?}"));
    let response = children.session.command_response(length).to_vec();
    let mut fields = [0_u16; 64];
    let header = match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("a response frame")
    {
        protocol::DecodedTypedResponseFrame::Success { header, .. } => header,
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => {
            panic!(
                "request {request_id}: {:?} {:?}",
                header.status, payload.diagnostics
            )
        }
    };
    assert_eq!(
        header.revision,
        SessionRevision(base + 1),
        "request {request_id}"
    );
    while children
        .session
        .dequeue_event(EventLane::Reliable, 4_096)
        .expect("a reliable event")
        .is_some()
    {}
}

fn compile(model: &SessionModel, limits: CompileLimits) -> CompiledChildren {
    let document = session::canonical_session_json(model).expect("canonical");
    compile_children(&document, limits).unwrap_or_else(|_| panic!("the session compiles"))
}

fn bits(block: &[f32]) -> Vec<u32> {
    block.iter().map(|sample| sample.to_bits()).collect()
}

fn revision(children: &CompiledChildren) -> u64 {
    children.session.test_controller().session().revision().0
}

/// Gate 1: three structural transactions submitted without a render between them each commit,
/// and the plan the last one publishes renders exactly what one transaction of all three edits
/// renders; the watermark completes the first two as superseded.
///
/// P0 has tracks `t1`-`t4` on the fixture source, `t2` and `t3` with a compressor insert. T1 adds
/// source `s` with its track and a compressor on `t1`, and moves `t4`'s fader; T2 replaces `t2`'s
/// compressor; T3 removes `t3`'s. `s` is fed between T1 and T2, so its ring is the withdrawn
/// candidate's and moves by the donation; `t4` carries from P0 with its fader retargeted at T1's
/// own ramp length.
///
/// Test value: red if any of T1-T3 is refused, if the replacement carries from the withdrawn
/// candidate's identity while P0 runs (the carry finds no predecessor and the fixture source and
/// `t2` restart), if it is prepared against the current model instead of P0's (`t4`'s fader is
/// then no change, and `t4` keeps P0's), if the retarget forgets T1's ramp length, if `s`'s acked
/// PCM is lost with the withdrawn candidate, or if the superseded count is off.
#[test]
fn a_superseding_replacement_renders_as_one_combined_transaction() {
    let mut model = stateless_session(48_000);
    model.tracks[1].inserts.effects.push(compressor());
    model.tracks[2].inserts.effects.push(compressor());
    // `t4` is untouched but for a fader change T1 carries: it carries from P0 in every plan.
    let mut t4 = model.tracks[0].clone();
    t4.id = id("t4");
    let mut route = model.routes[0].clone();
    route.id = id("t4-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = t4.id.clone();
    model.tracks.push(t4);
    model.routes.push(route);
    let fader = session::DualMonoFader {
        left_db: -6.0,
        right_db: -9.0,
        left_mute: false,
        right_mute: false,
    };
    // The fader edits carry their own ramp length (#1394), which the replacement's base keeps
    // across the withdrawn candidate (root ruling B15-Q3).
    let mut fader_edits = protocol::control_core::fader_edits(&id("t4"), &fader);
    for edit in &mut fader_edits[..2] {
        let SessionEdit::SetControl { ramp_samples, .. } = edit else {
            panic!("a control edit")
        };
        *ramp_samples = Some(1);
    }
    let first: Vec<SessionEdit> = add_source_track(&model)
        .into_iter()
        .chain([put(&model, 0, compressor())])
        .chain(fader_edits)
        .collect();
    let second = [put(&model, 1, gate())];
    let third = [remove(&model, 2)];
    let combined: Vec<SessionEdit> = first.iter().chain(&second).chain(&third).cloned().collect();

    let run = |superseding: bool| {
        let mut children = compile(&model, limits());
        submit_direct(&mut children, &[FIXTURE], 0);
        let mut blocks = vec![render_direct(&mut children, 0)];
        let before = children.plan.queries().watermark().expect("watermark");
        if superseding {
            for (request, (base, edits)) in [(42, &first[..]), (43, &second[..]), (44, &third[..])]
                .into_iter()
                .enumerate()
            {
                commit(&mut children, request as u64 + 1, base, edits);
                if request == 0 {
                    // `s` is fed while T1's candidate waits, into the candidate's own ring.
                    submit_direct(&mut children, &[ADDED], 0);
                    submit_direct(&mut children, &[ADDED], 1);
                }
            }
            assert_eq!(revision(&children), 45);
        } else {
            commit(&mut children, 1, 42, &combined);
            submit_direct(&mut children, &[ADDED], 0);
            submit_direct(&mut children, &[ADDED], 1);
        }
        submit_direct(&mut children, &[FIXTURE], 1);
        blocks.push(render_direct(&mut children, 1));
        let adopted = children.plan.queries().watermark().expect("watermark");
        for block in 2..9 {
            submit_direct(&mut children, &[FIXTURE, ADDED], block);
            blocks.push(render_direct(&mut children, block));
        }
        (blocks, before, adopted)
    };
    let (superseded, before, adopted) = run(true);
    let (twin, _, _) = run(false);
    for (block, (actual, expected)) in superseded.iter().zip(&twin).enumerate() {
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "block {block}: the twin plays"
        );
        assert_eq!(bits(actual), bits(expected), "block {block}");
    }
    assert_eq!(adopted.revision, 45);
    assert_eq!(
        adopted.outcome_flags,
        engine::realtime::OUTCOME_EXACT | engine::realtime::OUTCOME_SUPERSEDED
    );
    assert_eq!(adopted.superseded, before.superseded + 2);
    assert_eq!(adopted.exact, before.exact + 1);
}

/// The lifecycle counters every constructed owner and every reservation are balanced in.
fn assert_balanced(label: &str, counters: TestOwnerCounters) {
    assert_eq!(
        counters.current_provider_constructed + counters.candidate_provider_constructed,
        counters.current_provider_disposed + counters.candidate_provider_disposed,
        "{label}: providers"
    );
    assert_eq!(
        counters.current_plan_constructed + counters.candidate_plan_constructed,
        counters.current_plan_disposed + counters.candidate_plan_disposed,
        "{label}: plans"
    );
    assert_eq!(
        counters.token_constructed, counters.token_disposed,
        "{label}: tokens"
    );
    assert_eq!(
        counters.reservation_constructed,
        counters.reservation_canceled + counters.reservation_committed,
        "{label}: reservations"
    );
}

/// Gate 3: a replacement refused after the candidate was withdrawn restores it. T2 is refused
/// for its effect state (a cap one byte below its own), with the compile rejection; the revision
/// stays T1's, and the next block renders exactly what the T1-only session renders. Then each
/// structural fault phase is armed during T2, the one between the donation check and the commit
/// (`BeforeProtocolCommit`) and the commit's own included: the refusal changes no committed or
/// published state, the candidate is republished each time it was withdrawn, a retried T2
/// supersedes it and renders as a supersession nothing refused, and every owner and reservation
/// balances once both handles are gone. Last, the withdrawn candidate is itself a replacement
/// with a nonzero `superseded` word (batch 15a review fold m4): the refusal republishes it with
/// that word, and a retry's word counts every revision both candidates folded in.
///
/// Test value: red if a refusal after the withdrawal drops or leaks the candidate (the next block
/// is P0's, or an owner stays undisposed), if a donation runs before the commit (the fault
/// between the check and the commit would leave the candidate without its rings), or if a
/// republished replacement loses its `superseded` word (T1 then never completes).
#[test]
fn a_refused_replacement_restores_the_withdrawn_candidate() {
    use TestStructuralFaultPhase::{
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterRuntimePrepare,
        BeforeProtocolCommit, BeforeRuntimePrepare, ProtocolCommit,
    };
    let model = stateless_session(48_000);
    let first = add_source_track(&model);
    let second = [put(&model, 1, compressor()), put(&model, 2, compressor())];
    // T1 adds no effect, so the replacement's effect state is T2's two compressors'.
    let mut after_second = model.clone();
    for track in &mut after_second.tracks[1..] {
        track.inserts.effects.push(compressor());
    }
    let need = compile(&after_second, limits())
        .plan
        .resources()
        .effect_scalar_state_bytes;
    // T1 is pending, with block 1 of both sources fed.
    let pending = |limits: CompileLimits| {
        let mut children = compile(&model, limits);
        submit_direct(&mut children, &[FIXTURE], 0);
        render_direct(&mut children, 0);
        commit(&mut children, 1, 42, &first);
        submit_direct(&mut children, &[ADDED], 0);
        submit_direct(&mut children, &[FIXTURE, ADDED], 1);
        children
    };
    let twin = render_direct(&mut pending(limits()), 1);
    assert!(twin.iter().any(|sample| *sample != 0.0), "the twin plays");
    // T2 superseding T1 with nothing refused, the reference of the retries below.
    let superseding = {
        let mut children = pending(limits());
        commit(&mut children, 2, 43, &second);
        render_direct(&mut children, 1)
    };

    let mut tight = limits();
    tight.maximum_effect_state_bytes = need - 1;
    let mut children = pending(tight);
    let epoch = children.session.test_pending_providers()[0].test_epoch();
    let counters = children.session.test_owner_counters();
    let refused = children
        .session
        .command(&transaction(2, 43, &second), 4_096);
    let Err(CommandError::CompileRejected(failure)) = &refused else {
        panic!("a compile rejection, not {refused:?}")
    };
    let diagnostic = String::from_utf8(failure.diagnostic().into_owned()).expect("UTF-8");
    assert!(
        diagnostic.contains("effect") && diagnostic.contains("resource.limit"),
        "{diagnostic}"
    );
    assert_eq!(revision(&children), 43, "the revision is T1's");
    let after = children.session.test_owner_counters();
    assert_eq!(after.candidate_withdrawn, counters.candidate_withdrawn + 1);
    assert_eq!(
        after.candidate_republished,
        counters.candidate_republished + 1
    );
    assert_eq!(
        children.session.test_pending_providers()[0].test_epoch(),
        epoch,
        "the candidate is republished with its own epoch"
    );
    let block = render_direct(&mut children, 1);
    assert_eq!(bits(&block), bits(&twin), "T1's candidate renders");

    for phase in [
        AfterProtocolPrepare,
        BeforeRuntimePrepare,
        AfterRuntimePrepare,
        AfterAdmission,
        AfterPlanReservation,
        BeforeProtocolCommit,
        ProtocolCommit,
    ] {
        control_plane::test_reset_lifecycle_observer();
        let mut children = pending(limits());
        let snapshot = children.session.test_transaction_snapshot();
        children
            .session
            .test_set_structural_faults([Some(phase), None]);
        let refused = children
            .session
            .command(&transaction(2, 43, &second), 4_096);
        assert!(
            matches!(
                refused,
                Err(CommandError::Backpressure | CommandError::Internal)
            ),
            "{phase:?}: {refused:?}"
        );
        assert_eq!(
            children.session.test_transaction_snapshot(),
            snapshot,
            "{phase:?}: the refusal changed nothing"
        );
        let counters = children.session.test_owner_counters();
        let withdrawn = !matches!(phase, AfterProtocolPrepare | BeforeRuntimePrepare);
        assert_eq!(
            (counters.candidate_withdrawn, counters.candidate_republished),
            (u64::from(withdrawn), u64::from(withdrawn)),
            "{phase:?}"
        );
        // The retry supersedes the republished candidate.
        commit(&mut children, 3, 43, &second);
        assert_eq!(
            children.session.test_owner_counters().candidate_superseded,
            1,
            "{phase:?}"
        );
        let block = render_direct(&mut children, 1);
        assert_eq!(
            bits(&block),
            bits(&superseding),
            "{phase:?}: the retry renders as a supersession nothing refused"
        );
        children.session.service().expect("service");
        assert_eq!(revision(&children), 44, "{phase:?}");
        assert!(children.session.test_pending_providers().is_empty());
        drop(children);
        assert_balanced(
            &format!("{phase:?}"),
            control_plane::test_lifecycle_counters(),
        );
    }

    // Batch 15a review fold m4: the withdrawn candidate is itself a replacement, B, whose
    // `superseded` word counts T1. T3 is refused after B was withdrawn; B is republished with its
    // word, so the block that adopts it completes T1 as superseded and T2 as exact. Then a retried
    // T3 supersedes B, and its word counts T1 and T2.
    let third = [put(&model, 0, compressor())];
    for retry in [false, true] {
        control_plane::test_reset_lifecycle_observer();
        let mut children = pending(limits());
        let before = children.plan.queries().watermark().expect("watermark");
        commit(&mut children, 2, 43, &second);
        children
            .session
            .test_set_structural_faults([Some(BeforeProtocolCommit), None]);
        let refused = children.session.command(&transaction(3, 44, &third), 4_096);
        assert!(
            matches!(refused, Err(CommandError::Backpressure)),
            "retry {retry}: {refused:?}"
        );
        let counters = children.session.test_owner_counters();
        assert_eq!(
            (counters.candidate_withdrawn, counters.candidate_republished),
            (2, 1),
            "retry {retry}: B was withdrawn and republished"
        );
        if retry {
            commit(&mut children, 4, 44, &third);
        }
        render_direct(&mut children, 1);
        let adopted = children.plan.queries().watermark().expect("watermark");
        let folded = if retry { 2 } else { 1 };
        assert_eq!(adopted.revision, 44 + u64::from(retry), "retry {retry}");
        assert_eq!(
            adopted.outcome_flags,
            engine::realtime::OUTCOME_EXACT | engine::realtime::OUTCOME_SUPERSEDED,
            "retry {retry}"
        );
        assert_eq!(
            (adopted.superseded, adopted.exact),
            (before.superseded + folded, before.exact + 1),
            "retry {retry}: the republished word completes every revision it folded in"
        );
        children.session.service().expect("service");
        drop(children);
        assert_balanced(
            &format!("m4 retry {retry}"),
            control_plane::test_lifecycle_counters(),
        );
    }
}

/// A plan's graph bytes and its compiled model's bytes.
type GraphRow = (u64, u64);

/// The graph rows of P0, of A (P0 with `first`) and of B (A with `second`), each read while its
/// plan renders (gate 4).
fn graph_rows(
    model: &SessionModel,
    first: &[SessionEdit],
    second: &[SessionEdit],
) -> (GraphRow, GraphRow, GraphRow) {
    let graph = |children: &CompiledChildren| {
        (
            children.plan.resources().graph_session_plus_plan_bytes,
            children
                .session
                .committed_session()
                .resource_estimate()
                .compiled_model_bytes,
        )
    };
    let mut children = compile(model, limits());
    let p0 = graph(&children);
    submit_direct(&mut children, &[FIXTURE], 0);
    render_direct(&mut children, 0);
    commit(&mut children, 1, 42, first);
    render_direct(&mut children, 1);
    children.session.service().expect("service");
    let a = graph(&children);
    commit(&mut children, 2, 43, second);
    render_direct(&mut children, 2);
    children.session.service().expect("service");
    (p0, a, graph(&children))
}

/// Gate 4: the replacement admission holds the running plan, the withdrawn candidate and the
/// replacement at once (#1310 D3, #1398 D4). With a graph cap above every two-plan peak and below
/// that three-plan peak, T2 behind a pending T1 is refused, the revision stays T1's and T1's
/// candidate renders; once it is adopted the same T2 commits. T2 fits beside T1's plan alone, so
/// the refusal is retryable `BACKPRESSURE` (batch 15a review fold, item 2). With a cap that admits
/// T1 beside the running plan but not T2 beside T1's plan, the refusal is the permanent
/// `graph.resource.limit`, before and after T1's plan is adopted.
///
/// Test value: red if a three-plan peak is admitted beyond the cap, if the refusal loses the
/// withdrawn candidate, if a refusal a render clears is permanent, or if one no render clears is
/// retryable.
#[test]
fn a_replacement_over_the_three_plan_ceiling_is_refused_and_the_candidate_kept() {
    let model = stateless_session(48_000);
    let first = add_source_track(&model);
    let second = [put(&model, 1, compressor())];
    let (p0, a, b) = graph_rows(&model, &first, &second);
    let two_plans = (p0.0 + a.0 + p0.1 + a.1).max(a.0 + b.0 + a.1 + b.1);
    let three_plans = p0.0 + a.0 + b.0 + p0.1 + a.1 + b.1;
    let cap = two_plans + (p0.0 + p0.1) / 2;
    assert!(cap < three_plans, "{cap} against {three_plans}");
    let mut limits = limits();
    limits.maximum_graph_session_plus_plan_bytes = cap;

    let twin = {
        let mut children = compile(&model, crate::runtime::tests::limits());
        submit_direct(&mut children, &[FIXTURE], 0);
        render_direct(&mut children, 0);
        commit(&mut children, 1, 42, &first);
        submit_direct(&mut children, &[ADDED], 0);
        submit_direct(&mut children, &[FIXTURE, ADDED], 1);
        render_direct(&mut children, 1)
    };
    assert!(twin.iter().any(|sample| *sample != 0.0), "the twin plays");
    let mut children = compile(&model, limits);
    submit_direct(&mut children, &[FIXTURE], 0);
    render_direct(&mut children, 0);
    commit(&mut children, 1, 42, &first);
    let refused = children
        .session
        .command(&transaction(2, 43, &second), 4_096);
    assert!(
        matches!(refused, Err(CommandError::Backpressure)),
        "a refusal a render clears is retryable, not {refused:?}"
    );
    assert_eq!(revision(&children), 43);
    submit_direct(&mut children, &[ADDED], 0);
    submit_direct(&mut children, &[FIXTURE, ADDED], 1);
    let rendered = render_direct(&mut children, 1);
    assert_eq!(bits(&rendered), bits(&twin), "A was republished");
    commit(&mut children, 3, 43, &second);

    // A cap that admits A beside P0 but not B beside A: no render clears it.
    let a_beside_p0 = p0.0 + a.0 + p0.1 + a.1;
    let b_beside_a = a.0 + b.0 + a.1 + b.1;
    assert!(
        a_beside_p0 < b_beside_a,
        "{a_beside_p0} against {b_beside_a}"
    );
    let mut limits = crate::runtime::tests::limits();
    limits.maximum_graph_session_plus_plan_bytes = a_beside_p0;
    let mut children = compile(&model, limits);
    submit_direct(&mut children, &[FIXTURE], 0);
    render_direct(&mut children, 0);
    commit(&mut children, 1, 42, &first);
    for (request, label) in [(2, "while A waits"), (3, "once A runs")] {
        let refused = children
            .session
            .command(&transaction(request, 43, &second), 4_096);
        let Err(CommandError::CompileRejected(failure)) = &refused else {
            panic!("{label}: a compile rejection, not {refused:?}")
        };
        assert_eq!(
            &*failure.diagnostic(),
            b"graph.resource.limit\t$\n",
            "{label}"
        );
        assert_eq!(revision(&children), 43, "{label}");
        render_direct(&mut children, request - 1);
    }
}

/// Gate 6: P0 has sources `s` and `u`. T1 changes `u`'s track's insert, so its candidate A
/// persists `s` and holds `s`'s producer; T2 removes `s`'s track and `s`. The superseding
/// replacement B holds no `s`, and `s`'s producer returns to the running epoch, which still renders
/// `s` until B is adopted; it retires with that epoch.
///
/// Test value: red if the supersession drops A with the producers B did not take (D5 step 5
/// missing, or after A's drop): the running epoch's entry for `s` is then vacant. Gates 1-5 do not
/// see it, because none removes a source A persisted.
#[test]
fn a_removed_sources_producer_returns_to_the_running_epoch() {
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    model.tracks[0].inserts.effects.push(compressor());
    let mut source = model.sources[0].clone();
    source.id = id("s-source");
    model.tracks[1].source_id = source.id.clone();
    model.sources.push(source);
    model.sources.sort_by(|left, right| left.id.cmp(&right.id));
    let first = [put(&model, 0, gate())];
    let second = [
        SessionEdit::RemoveRoute {
            route_id: model.routes[1].id.clone(),
        },
        SessionEdit::RemoveTrack {
            track_id: model.tracks[1].id.clone(),
        },
        SessionEdit::RemoveSource {
            source_id: id("s-source"),
        },
    ];
    let mut children = compile(&model, limits());
    submit_direct(&mut children, &[FIXTURE, ADDED], 0);
    render_direct(&mut children, 0);
    submit_direct(&mut children, &[FIXTURE, ADDED], 1);
    commit(&mut children, 1, 42, &first);
    let running = children.session.test_providers().test_epoch();
    let vacant = |children: &mut CompiledChildren, epoch: u64, source: &[u8]| {
        children
            .session
            .test_epoch_mut(epoch)
            .expect("the epoch")
            .test_source_vacant(source)
    };
    assert_eq!(
        vacant(&mut children, running, ADDED),
        Some(true),
        "A holds `s`'s producer"
    );
    let disposed = children.session.test_owner_counters();
    commit(&mut children, 2, 43, &second);
    let counters = children.session.test_owner_counters();
    assert_eq!(
        counters.candidate_superseded,
        disposed.candidate_superseded + 1
    );
    assert_eq!(
        counters.candidate_provider_disposed,
        disposed.candidate_provider_disposed + 1,
        "A's epoch is dropped"
    );
    let replacement = children.session.test_pending_providers()[0].test_epoch();
    assert_eq!(
        vacant(&mut children, running, ADDED),
        Some(false),
        "`s`'s producer is back in the running epoch"
    );
    assert_eq!(
        vacant(&mut children, running, FIXTURE),
        Some(true),
        "`u`'s producer moved on to the replacement"
    );
    assert_eq!(vacant(&mut children, replacement, ADDED), None);
    assert_eq!(vacant(&mut children, replacement, FIXTURE), Some(false));
    // B's adoption retires the running epoch, `s`'s producer with it.
    submit_direct(&mut children, &[FIXTURE], 2);
    render_direct(&mut children, 1);
    children.session.service().expect("service");
    assert_eq!(children.session.test_providers().test_epoch(), replacement);
    assert_eq!(
        children
            .session
            .test_owner_counters()
            .current_provider_disposed,
        counters.current_provider_disposed + 1
    );
}

/// `model` with a fourth track `t4` on the fixture source, a copy of the first track with its
/// route: no transaction below changes its chain, so every plan carries it from P0.
fn with_t4(mut model: SessionModel) -> SessionModel {
    let mut t4 = model.tracks[0].clone();
    t4.id = id("t4");
    t4.inserts.effects.clear();
    let mut route = model.routes[0].clone();
    route.id = id("t4-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = t4.id.clone();
    model.tracks.push(t4);
    model.routes.push(route);
    model
}

/// The `SET_CONTROL`s of `t4`'s fader at `left_db`/`right_db`, the two gain rows with `ramp`.
fn t4_fader(left_db: f32, right_db: f32, ramp: Option<u32>) -> Vec<SessionEdit> {
    let fader = session::DualMonoFader {
        left_db,
        right_db,
        left_mute: false,
        right_mute: false,
    };
    let mut edits = protocol::control_core::fader_edits(&id("t4"), &fader);
    for edit in &mut edits[..2] {
        let SessionEdit::SetControl { ramp_samples, .. } = edit else {
            panic!("a control edit")
        };
        *ramp_samples = ramp;
    }
    edits
}

/// Renders blocks 0-8 of `model` fed on the fixture source, with `steps` committed one after
/// another after block 0 (each `(edits, advances)`: whether it commits a model revision) and
/// then with `combined` committed as one transaction instead. Returns both block lists.
fn superseding_and_combined(
    model: &SessionModel,
    steps: &[(Vec<SessionEdit>, bool)],
    combined: &[SessionEdit],
) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let run = |superseding: bool| {
        let mut children = compile(model, limits());
        submit_direct(&mut children, &[FIXTURE], 0);
        let mut blocks = vec![render_direct(&mut children, 0)];
        if superseding {
            let mut base = 42;
            for (request, (edits, advances)) in steps.iter().enumerate() {
                if *advances {
                    commit(&mut children, request as u64 + 1, base, edits);
                    base += 1;
                } else {
                    // A transaction of solo writes commits no model (#1559 D4).
                    children
                        .session
                        .command(&transaction(request as u64 + 1, base, edits), 4_096)
                        .unwrap_or_else(|error| panic!("solo: {error:?}"));
                    assert_eq!(revision(&children), base);
                }
            }
            assert_eq!(
                children.session.test_owner_counters().candidate_superseded,
                steps.iter().filter(|(_, advances)| *advances).count() as u64 - 1,
                "every structural step but the last was superseded"
            );
        } else {
            commit(&mut children, 1, 42, combined);
        }
        for block in 1..9 {
            submit_direct(&mut children, &[FIXTURE], block);
            blocks.push(render_direct(&mut children, block));
        }
        blocks
    };
    (run(true), run(false))
}

fn assert_same_blocks(label: &str, actual: &[Vec<f32>], expected: &[Vec<f32>]) {
    assert_eq!(actual.len(), expected.len());
    for (block, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            expected.iter().any(|sample| *sample != 0.0),
            "{label} block {block}: the twin plays"
        );
        assert_eq!(bits(actual), bits(expected), "{label} block {block}");
    }
}

/// #1310 (D3 with root ruling B15-Q3, "B wins on a shared row"): T1 moves `t4`'s fader with its own
/// ramp length; T2, which supersedes T1, moves it again with no length. The replacement ramps
/// `t4` with the session default, as a transaction holding T1's structural edit and T2 does: B
/// wrote the row last, so T1's length no longer applies to it.
///
/// Test value: red if the merge keeps T1's length on a row B wrote without one (it lays only B's
/// lengths over T1's), so `t4` steps where the combined transaction ramps.
#[test]
fn a_superseding_edit_without_a_ramp_length_takes_the_default_on_its_row() {
    let mut model = with_t4(stateless_session(48_000));
    model.tracks[1].inserts.effects.push(compressor());
    let first: Vec<SessionEdit> = [put(&model, 0, compressor())]
        .into_iter()
        .chain(t4_fader(-6.0, -9.0, Some(1)))
        .collect();
    let second: Vec<SessionEdit> = [put(&model, 1, gate())]
        .into_iter()
        .chain(t4_fader(-12.0, -3.0, None))
        .collect();
    // One transaction may not give a row two lengths (`control.ramp.conflict`), so the reference
    // holds the last write of `t4`'s fader only, which is what both edits together leave.
    let combined: Vec<SessionEdit> = [put(&model, 0, compressor())]
        .into_iter()
        .chain(second.iter().cloned())
        .collect();
    let (superseded, twin) =
        superseding_and_combined(&model, &[(first, true), (second, true)], &combined);
    assert_same_blocks("ramp", &superseded, &twin);
}

/// #1310 (D3 with root ruling B15-Q3, the kept monitor overlay): T1 is pending, a solo of `t1` with its
/// own ramp length commits on it, and T2 supersedes T1. The replacement renders the solo from the
/// overlay the running plan's cells hold, with the solo's ramp: exactly what one transaction
/// holding T1, the solo and T2 renders.
///
/// Test value: red if the replacement takes the session's current overlay as the one the running
/// plan holds (the solo is then no change, and the other tracks stay unmuted), or if it forgets the
/// solo's ramp length (the mutes ramp with `mute_ms` instead).
#[test]
fn a_solo_committed_while_a_candidate_waits_holds_on_its_replacement() {
    let mut model = with_t4(stateless_session(48_000));
    model.tracks[1].inserts.effects.push(compressor());
    let first = vec![put(&model, 0, compressor())];
    let solo = vec![SessionEdit::set_control(
        protocol::control_core::row_address(
            protocol::control_core::Owner::Strip(model.tracks[0].id.clone()),
            protocol::control_core::Section::Own,
            "solo",
            protocol::control_core::Lane::Both,
        )
        .expect("the solo row"),
        1.0,
        Some(1),
    )];
    let second = vec![put(&model, 1, gate())];
    let combined: Vec<SessionEdit> = first.iter().chain(&solo).chain(&second).cloned().collect();
    let (superseded, twin) = superseding_and_combined(
        &model,
        &[(first, true), (solo, false), (second, true)],
        &combined,
    );
    assert_same_blocks("solo", &superseded, &twin);
}

/// #1310 (D3 with root ruling B15-Q3, the live path's merge): T1 is pending, a live edit moves `t4`'s
/// fader with its own ramp length on T1's lanes, and T2 supersedes T1. The replacement carries
/// `t4` from the running plan and retargets its fader at the live edit's ramp length: exactly
/// what one transaction holding T1, the fader edit and T2 renders.
///
/// Test value: red if a live edit committed while a candidate waits does not join the kept base's
/// ramp lengths (the live path's merge deleted): the replacement then ramps `t4`'s fader with the
/// session default where the combined transaction ramps it over one sample.
#[test]
fn a_live_ramp_committed_while_a_candidate_waits_holds_on_its_replacement() {
    let mut model = with_t4(stateless_session(48_000));
    model.tracks[1].inserts.effects.push(compressor());
    let first = vec![put(&model, 0, compressor())];
    let live = t4_fader(-6.0, -9.0, Some(1));
    let second = vec![put(&model, 1, gate())];
    let combined: Vec<SessionEdit> = first.iter().chain(&live).chain(&second).cloned().collect();
    let run = |superseding: bool| {
        let mut children = compile(&model, limits());
        submit_direct(&mut children, &[FIXTURE], 0);
        let mut blocks = vec![render_direct(&mut children, 0)];
        if superseding {
            commit(&mut children, 1, 42, &first);
            let pending = children.session.test_pending_providers()[0].test_epoch();
            commit(&mut children, 2, 43, &live);
            assert_eq!(
                children.session.test_pending_providers()[0].test_epoch(),
                pending,
                "the fader edit rode the candidate's live lanes"
            );
            commit(&mut children, 3, 44, &second);
            assert_eq!(
                children.session.test_owner_counters().candidate_superseded,
                1,
                "T2 superseded T1"
            );
        } else {
            commit(&mut children, 1, 42, &combined);
        }
        for block in 1..9 {
            submit_direct(&mut children, &[FIXTURE], block);
            blocks.push(render_direct(&mut children, block));
        }
        blocks
    };
    assert_same_blocks("live ramp", &run(true), &run(false));
}

/// Renders block `block` as the first half of `PlanState::render` does: the owner swaps to a
/// published candidate and retires the plan it displaced, but the atomic the control plane reads
/// is not published (a render call preempted before it returns).
fn render_unpublished(children: &mut CompiledChildren, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    children
        .plan
        .test_owner_mut()
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
            },
            block * 128,
        )
        .expect("block");
    pcm
}

/// Batch 15a review fold M1: render claims candidate A and is preempted before A's predecessor
/// reaches the control side, so a structural edit C finds A taken but cannot promote it; C
/// publishes B on A. Render then adopts B as well, and edit D finds B taken with A still pending.
/// Each edit that finds the newest candidate taken promotes every pending candidate up to it, so
/// D commits, the blocks equal a run whose renders each published their epoch, and once the
/// retirements reach the control side every plan and owner is accounted for.
///
/// Test value: red if a taken candidate stays pending until its predecessor's retirement is
/// reclaimed: D then meets two pending candidates and a full pending list, the accounting fault
/// of #1398 D3 (`debug_assert!`, `INTERNAL`).
#[test]
fn edits_after_render_took_two_candidates_unretired_each_commit() {
    let model = stateless_session(48_000);
    let edits = [
        add_source_track(&model).to_vec(),
        vec![put(&model, 1, compressor())],
        vec![put(&model, 2, compressor())],
    ];
    let run = |race: bool| {
        control_plane::test_reset_lifecycle_observer();
        let mut children = compile(&model, limits());
        let mut blocks = Vec::new();
        submit_direct(&mut children, &[FIXTURE], 0);
        blocks.push(render_direct(&mut children, 0));
        control_plane::test_hold_reclamation(race);
        for (index, edits) in edits.iter().enumerate() {
            let block = index as u64 + 1;
            commit(&mut children, block, 41 + block, edits);
            if index == 0 {
                submit_direct(&mut children, &[ADDED], 0);
            } else {
                submit_direct(&mut children, &[ADDED], block - 1);
            }
            submit_direct(&mut children, &[FIXTURE], block);
            blocks.push(if race {
                render_unpublished(&mut children, block)
            } else {
                render_direct(&mut children, block)
            });
        }
        if race {
            let counters = children.session.test_owner_counters();
            assert_eq!(
                (counters.candidate_taken, counters.candidate_withdrawn),
                (2, 0),
                "both later edits found the newest candidate taken"
            );
        }
        control_plane::test_hold_reclamation(false);
        submit_direct(&mut children, &[FIXTURE], 4);
        submit_direct(&mut children, &[ADDED], 3);
        blocks.push(render_direct(&mut children, 4));
        children.session.service().expect("service");
        assert_eq!(revision(&children), 45);
        assert!(children.session.test_pending_providers().is_empty());
        assert!(children.session.test_retired_providers().is_empty());
        drop(children);
        assert_balanced(
            if race { "race" } else { "paced" },
            control_plane::test_lifecycle_counters(),
        );
        blocks
    };
    let paced = run(false);
    let raced = run(true);
    assert!(
        paced[3].iter().any(|sample| *sample != 0.0),
        "the compared blocks play"
    );
    for (block, (actual, expected)) in raced.iter().zip(&paced).enumerate() {
        assert_eq!(bits(actual), bits(expected), "block {block}");
    }
}

/// Batch 15a review fold 2, B-1: render claims A and is preempted before it retires P0; edit C
/// finds A taken and parks P0; render adopts B and retires A; edit D finds B taken and parks A;
/// render then adopts C and publishes its epoch. The control side now holds two parked epochs, the
/// running B and the pending C, with the atomic naming C. The next service step reclaims P0's, A's
/// and B's plans and promotes C, and the session takes edits again.
///
/// Test value: red if the parked list holds fewer epochs than the retirement queue has credits
/// (the step that reclaims nothing cannot park a third), and red if, with that list short, the
/// service step promotes from the atomic before it reclaims: every later call then answers
/// `INTERNAL`, permanently.
#[test]
fn two_taken_candidates_and_a_published_adoption_never_wedge_the_session() {
    let model = stateless_session(48_000);
    let edits = [
        add_source_track(&model).to_vec(),
        vec![put(&model, 1, compressor())],
        vec![put(&model, 2, compressor())],
    ];
    control_plane::test_reset_lifecycle_observer();
    let mut children = compile(&model, limits());
    submit_direct(&mut children, &[FIXTURE], 0);
    render_direct(&mut children, 0);
    control_plane::test_hold_reclamation(true);
    for (index, edits) in edits.iter().enumerate() {
        let block = index as u64 + 1;
        commit(&mut children, block, 41 + block, edits);
        submit_direct(&mut children, &[ADDED], block - 1);
        submit_direct(&mut children, &[FIXTURE], block);
        if index < 2 {
            render_unpublished(&mut children, block);
        } else {
            render_direct(&mut children, block);
        }
    }
    let snapshot = children.session.test_transaction_snapshot();
    assert_eq!(
        (
            snapshot.retired_provider_epochs,
            snapshot.provider_epoch,
            snapshot.pending_provider_epochs,
            snapshot.active_plan_epoch,
        ),
        (vec![0, 1], 2, vec![3], 3),
        "two parked epochs, B running on the control side, C adopted by render"
    );
    // A step that reclaims nothing (the seam holds every plan) still promotes C from the atomic:
    // the parked list holds one epoch per retirement credit, so the third fits.
    children
        .session
        .service()
        .expect("a step that reclaims nothing promotes C");
    assert_eq!(
        children
            .session
            .test_transaction_snapshot()
            .retired_provider_epochs,
        vec![0, 1, 2]
    );
    control_plane::test_hold_reclamation(false);
    for block in 4..8 {
        submit_direct(&mut children, &[ADDED], block - 1);
        submit_direct(&mut children, &[FIXTURE], block);
        render_direct(&mut children, block);
        children
            .session
            .service()
            .unwrap_or_else(|error| panic!("block {block}: {error:?}"));
    }
    assert_eq!(children.session.test_providers().test_epoch(), 3);
    assert!(children.session.test_pending_providers().is_empty());
    assert!(children.session.test_retired_providers().is_empty());
    commit(&mut children, 4, 45, &[remove(&model, 1)]);
    render_direct(&mut children, 8);
    children.session.service().expect("service");
    drop(children);
    assert_balanced("wedge", control_plane::test_lifecycle_counters());
}

/// Batch 15a review fold 2, M-1: render claims A and retires P0, and the service step has not yet
/// reclaimed P0's plan. Edit T2 finds A taken and promotes it, but P0's plan is still alive, so
/// the admission charges P0, A and B together. With gate 4's cap (above every two-plan peak,
/// below the three-plan peak) T2 is refused as `BACKPRESSURE`; one service step reclaims P0 and
/// the same T2 commits.
///
/// With a source-total cap instead, the parked P0 is charged with each carried ring once: T2 is
/// admitted at the counted-once peak and refused one byte below.
///
/// Test value: red if a parked epoch's plan is left out of the admission after a `Taken`
/// promotion (T2 is then admitted at the three-plan peak while P0's plan still lives, and one
/// byte below the source peak), and red if the parked epoch's place in the held list double counts
/// a ring A carried from it (refused at the counted-once peak).
#[test]
fn a_displaced_plan_not_yet_reclaimed_is_charged_after_a_taken_promotion() {
    let model = stateless_session(48_000);
    let first = add_source_track(&model);
    let second = [put(&model, 1, compressor())];
    let (p0, a, b) = graph_rows(&model, &first, &second);
    let two_plans = (p0.0 + a.0 + p0.1 + a.1).max(a.0 + b.0 + a.1 + b.1);
    let three_plans = p0.0 + a.0 + b.0 + p0.1 + a.1 + b.1;
    let cap = two_plans + (p0.0 + p0.1) / 2;
    assert!(cap < three_plans, "{cap} against {three_plans}");
    let mut limits = limits();
    limits.maximum_graph_session_plus_plan_bytes = cap;

    control_plane::test_reset_lifecycle_observer();
    let mut children = compile(&model, limits);
    submit_direct(&mut children, &[FIXTURE], 0);
    render_direct(&mut children, 0);
    commit(&mut children, 1, 42, &first);
    control_plane::test_hold_reclamation(true);
    submit_direct(&mut children, &[ADDED], 0);
    submit_direct(&mut children, &[FIXTURE, ADDED], 1);
    render_unpublished(&mut children, 1);
    let refused = children
        .session
        .command(&transaction(2, 43, &second), 4_096);
    assert!(
        matches!(refused, Err(CommandError::Backpressure)),
        "P0's plan is alive and charged: {refused:?} (cap {cap}, three plans {three_plans})"
    );
    assert_eq!(children.session.test_owner_counters().candidate_taken, 1);
    assert_eq!(revision(&children), 43);
    assert_eq!(
        children
            .session
            .test_transaction_snapshot()
            .retired_provider_epochs,
        vec![0],
        "P0 is parked until its plan is reclaimed"
    );
    control_plane::test_hold_reclamation(false);
    children.session.service().expect("service");
    assert!(children.session.test_retired_providers().is_empty());
    commit(&mut children, 3, 43, &second);
    submit_direct(&mut children, &[FIXTURE, ADDED], 2);
    render_direct(&mut children, 2);
    children.session.service().expect("service");
    drop(children);
    assert_balanced("parked charge", control_plane::test_lifecycle_counters());

    // #1398 M2 with the parked epoch: each carried ring is counted once, in the oldest held plan
    // that holds it. The source peak is P0's rows, A's less what A carried from P0, and B's less
    // what B carried from A: admitted at that cap, `BACKPRESSURE` one byte below.
    let source_rows = {
        let mut children = compile(&model, crate::runtime::tests::limits());
        let row = |children: &CompiledChildren| {
            (
                children.plan.resources().source_total_bytes,
                children.session.test_providers().test_carried().0,
            )
        };
        let p0 = row(&children);
        submit_direct(&mut children, &[FIXTURE], 0);
        render_direct(&mut children, 0);
        commit(&mut children, 1, 42, &first);
        render_direct(&mut children, 1);
        children.session.service().expect("service");
        let a = row(&children);
        commit(&mut children, 2, 43, &second);
        render_direct(&mut children, 2);
        children.session.service().expect("service");
        [p0, a, row(&children)]
    };
    assert!(
        source_rows[1].1 > 0 && source_rows[2].1 > 0,
        "A and B carry rings: {source_rows:?}"
    );
    let counted_once = source_rows[0].0 + source_rows[1].0 - source_rows[1].1 + source_rows[2].0
        - source_rows[2].1;
    for (cap, admitted) in [(counted_once, true), (counted_once - 1, false)] {
        let mut limits = crate::runtime::tests::limits();
        limits.maximum_source_total_bytes = cap;
        let mut children = compile(&model, limits);
        submit_direct(&mut children, &[FIXTURE], 0);
        render_direct(&mut children, 0);
        commit(&mut children, 1, 42, &first);
        control_plane::test_hold_reclamation(true);
        submit_direct(&mut children, &[ADDED], 0);
        submit_direct(&mut children, &[FIXTURE, ADDED], 1);
        render_unpublished(&mut children, 1);
        let result = children
            .session
            .command(&transaction(2, 43, &second), 4_096);
        assert_eq!(
            children
                .session
                .test_transaction_snapshot()
                .retired_provider_epochs,
            vec![0],
            "cap {cap}: P0 parked"
        );
        control_plane::test_hold_reclamation(false);
        if admitted {
            assert!(result.is_ok(), "cap {cap} = {source_rows:?}: {result:?}");
        } else {
            assert!(
                matches!(result, Err(CommandError::Backpressure)),
                "cap {cap}: {result:?}"
            );
        }
    }
}
