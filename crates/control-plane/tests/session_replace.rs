//! #1386 (a1): `SESSION_REPLACE` through the one core path, `SessionState::command_with_document`.
//!
//! Every case boots a session from one document and replaces it with another. The fixture
//! `builtins-automation.json` holds one or more of every keyed kind (a source, a track, a submix,
//! a VCA, an output, routes and automation), so each kind is added, removed and changed from it.
#![cfg(feature = "test-support")]

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, RuntimePreparer,
    SessionState, compile_children, diff_document,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, SuccessorBase,
    prepare_host_runtime_with_live_lanes, prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionReplaceRequest, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use session::{
    ControlSmoothing, Output, RouteDestination, RouteSource, SessionModel, StableId,
    canonical_session_json, compile_session, parse_session_json,
};
use sha2::{Digest, Sha256};

const BASE: &str = include_str!("../../../fixtures/session/v1/builtins-automation.json");
const CONSOLE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
    const DOCUMENT_TRANSPORT: bool = true;
}

/// The C ABI's own preparation: host-core's live-lanes entries.
struct Preparer;

impl RuntimePreparer for Preparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        _spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            HostLiveControlHandles,
            Option<host_core::PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let live = HostLiveControlRequest::default();
        let (host, handles) = match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }?;
        Ok((host, handles, None))
    }
}

fn limits(maximum_document_bytes: u64) -> ControlLimits {
    ControlLimits {
        source_ring_frames: 0,
        maximum_document_bytes,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_control_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: 16_384,
        maximum_replay_bytes: 65_536,
        maximum_replay_entries: 16,
        maximum_submixes: 16,
        maximum_vcas: 16,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

fn boot(document: &str, maximum_document_bytes: u64) -> CompiledChildren<Adapter> {
    compile_children::<Adapter>(document, limits(maximum_document_bytes), Box::new(Preparer))
        .expect("boot compile")
}

fn model(document: &str) -> SessionModel {
    parse_session_json(document).expect("fixture parses")
}

fn json(model: &SessionModel) -> String {
    canonical_session_json(model).expect("a valid document")
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// A `SESSION_REPLACE` frame that names `document` (its length and its SHA-256).
fn replace_frame(request: u64, revision: SessionRevision, document: &[u8]) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = TypedCommandFrame {
        request_id: RequestId::new(request).expect("nonzero request"),
        expected_revision: ExpectedRevision::Exact(revision),
        payload: CommandPayload::SessionReplace(SessionReplaceRequest {
            document_bytes: u32::try_from(document.len()).expect("a small document"),
            document_sha256: Sha256::digest(document).into(),
        }),
    };
    let mut bytes = vec![0_u8; 256];
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("typed command");
    bytes.truncate(length);
    bytes
}

/// What a reply says: its status, its revision, the applied fields of an OK reply and the
/// diagnostic codes of a refusal.
#[derive(Debug, PartialEq)]
struct Reply {
    status: StatusCode,
    revision: u64,
    applied: Option<(u32, EditPath)>,
    codes: Vec<String>,
}

fn reply(bytes: &[u8]) -> Reply {
    let mut fields = [0_u16; 256];
    match ProtocolCodec::default()
        .decode_typed_response(bytes, &mut DecodeScratch::new(&mut fields))
        .expect("a typed reply")
    {
        DecodedTypedResponseFrame::Success { header, payload } => {
            let DecodedSuccessResponsePayload::SessionReplaced(applied) = payload else {
                panic!("a SESSION_REPLACE reply")
            };
            Reply {
                status: header.status,
                revision: header.revision.0,
                applied: Some((applied.applied_operations, applied.edit_path)),
                codes: Vec::new(),
            }
        }
        DecodedTypedResponseFrame::NonOk { header, payload } => Reply {
            status: header.status,
            revision: header.revision.0,
            applied: None,
            codes: payload.diagnostics.into_iter().map(|d| d.code).collect(),
        },
    }
}

fn send(
    session: &mut SessionState<Adapter>,
    request: u64,
    frame_document: &[u8],
    passed: &[u8],
) -> Reply {
    let revision = revision(session);
    let frame = replace_frame(request, revision, frame_document);
    let length = session
        .command_with_document(&frame, passed, 16_384)
        .expect("a reply frame");
    reply(session.command_response(length))
}

fn revision(session: &SessionState<Adapter>) -> SessionRevision {
    session.test_controller().session().revision()
}

fn snapshot(session: &SessionState<Adapter>) -> String {
    session
        .test_controller()
        .session()
        .canonical_snapshot()
        .to_owned()
}

fn reliable_events(session: &mut SessionState<Adapter>) -> usize {
    let mut count = 0;
    while session
        .dequeue_event(EventLane::Reliable, 16_384)
        .expect("event lane")
        .is_some()
    {
        count += 1;
    }
    count
}

/// The canonical text a boot of `next` gives, at `revision`.
fn booted_snapshot(next: &SessionModel, revision: u64) -> String {
    let mut next = next.clone();
    next.revision = revision;
    json(&next)
}

/// `base` with an extra unused source, a track reading it and a route from it to the output.
fn with_dependents(base: &SessionModel) -> SessionModel {
    let mut next = base.clone();
    let mut source = next.sources[0].clone();
    source.id = id("extra");
    next.sources.push(source);
    let mut track = next.tracks[0].clone();
    track.id = id("extra-track");
    track.source_id = id("extra");
    next.tracks.push(track);
    let mut route = next
        .routes
        .iter()
        .find(|route| matches!(route.destination, RouteDestination::OutputInput { .. }))
        .expect("a route to the output")
        .clone();
    route.id = id("extra-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = id("extra-track");
    next.routes.push(route);
    next
}

/// Test value: red if the diff misses a keyed kind, emits an unchanged entity, compares with
/// `PartialEq` (the fader `0.0` to `-0.0` case diffs to nothing), cannot return to the default
/// smoothing, or commits anything other than what a boot of the document gives.
#[test]
fn a_replace_commits_what_a_boot_of_the_document_gives_for_every_kind() {
    let base = model(BASE);
    let mut cases: Vec<(&str, SessionModel, SessionModel, usize)> = Vec::new();
    let mut case = |name, committed: &SessionModel, edit: &dyn Fn(&mut SessionModel), edits| {
        let mut next = committed.clone();
        edit(&mut next);
        cases.push((name, committed.clone(), next.clone(), edits));
        cases.push((name, next, committed.clone(), edits));
    };
    // Each pair adds and removes; the change cases run both ways too.
    case(
        "source",
        &base,
        &|m| {
            let mut source = m.sources[0].clone();
            source.id = id("extra");
            m.sources.push(source);
        },
        1,
    );
    case("source frames", &base, &|m| m.sources[0].frames = 24_000, 1);
    // A session has one output: renaming it removes one, adds one and retargets its route.
    case(
        "output",
        &base,
        &|m| {
            m.outputs = vec![Output { id: id("alt-out") }];
            for route in &mut m.routes {
                if let RouteDestination::OutputInput { output_id } = &mut route.destination {
                    *output_id = id("alt-out");
                }
            }
        },
        3,
    );
    case(
        "submix",
        &base,
        &|m| {
            let mut submix = m.submixes[0].clone();
            submix.id = id("bus-b");
            m.submixes.push(submix);
        },
        1,
    );
    case(
        "submix fader",
        &base,
        &|m| m.submixes[0].fader.left_db = -3.0,
        1,
    );
    case(
        "track",
        &base,
        &|m| {
            let mut track = m.tracks[0].clone();
            track.id = id("vocal-b");
            m.tracks.push(track);
        },
        1,
    );
    case(
        "track fader",
        &base,
        &|m| m.tracks[0].fader.left_db = -6.0,
        1,
    );
    case(
        "route",
        &base,
        &|m| {
            let mut route = m.routes[1].clone();
            route.id = id("bus-main");
            route.source = RouteSource::Submix {
                submix_id: id("bus"),
                tap: session::SendTap::PostPan,
            };
            m.routes.push(route);
        },
        1,
    );
    case("route gain", &base, &|m| m.routes[0].gain_db = -2.0, 1);
    case(
        "automation",
        &base,
        &|m| m.automation.retain(|a| a.id.as_str() != "pan-ride"),
        1,
    );
    case(
        "automation segment",
        &base,
        &|m| {
            m.automation[0].segments[0].end_value = -6.0;
        },
        1,
    );
    case(
        "vca",
        &base,
        &|m| {
            m.vcas.clear();
            m.automation.retain(|a| a.id.as_str() != "vca-ride");
        },
        2,
    );
    case("vca fader", &base, &|m| m.vcas[0].fader.left_db = -1.0, 1);
    case("session id", &base, &|m| m.session_id = id("renamed"), 1);
    // `PartialEq` holds these equal; their bits, and the canonical text, differ.
    case(
        "signed zero",
        &base,
        &|m| m.tracks[0].fader.right_db = -0.0,
        1,
    );
    // Default to explicit writes the two rows that differ from the defaults; explicit back to
    // default has no edit and commits the smoothing alone.
    let explicit = |m: &mut SessionModel| {
        m.control_smoothing = Some(ControlSmoothing {
            mute_ms: 10.0,
            fader_ms: 25.0,
            pan_ms: 40.0,
        });
    };
    let mut smooth = base.clone();
    explicit(&mut smooth);
    cases.push(("smoothing on", base.clone(), smooth.clone(), 2));
    cases.push(("smoothing off", smooth, base.clone(), 0));
    let console = model(CONSOLE);
    // Removing the slot rewrites the console and every strip's entries: one `SetConsole` and
    // one upsert per track.
    let mut bare = console.clone();
    bare.console.pre_insert.clear();
    for track in &mut bare.tracks {
        track.console.clear();
    }
    let strips = 1 + bare.tracks.len();
    cases.push(("console", console.clone(), bare.clone(), strips));
    cases.push(("console", bare, console, strips));

    for (index, (name, committed, next, edits)) in cases.into_iter().enumerate() {
        let committed_text = json(&committed);
        let next_text = json(&next);
        let compiled = |m: &SessionModel| {
            compile_session(
                m,
                session::CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            )
            .expect("compiles")
            .normalized_model()
            .clone()
        };
        assert_eq!(
            diff_document(&compiled(&committed), &compiled(&next)).len(),
            edits,
            "{name} #{index}: one edit per differing entity"
        );
        let CompiledChildren {
            mut session,
            plan: _plan,
        } = boot(&committed_text, 1_000_000);
        let before = revision(&session).0;
        reliable_events(&mut session);
        let answer = send(&mut session, 1, next_text.as_bytes(), next_text.as_bytes());
        assert_eq!(answer.status, StatusCode::Ok, "{name} #{index}: {answer:?}");
        let (applied, path) = answer.applied.expect("applied fields");
        assert_eq!(applied as usize, edits, "{name} #{index}");
        assert_ne!(path, EditPath::Unchanged, "{name} #{index}");
        assert_eq!(answer.revision, before + 1, "{name} #{index}");
        assert_eq!(
            snapshot(&session),
            booted_snapshot(&next, before + 1),
            "{name} #{index}: the committed snapshot is a boot of the document"
        );
        assert_eq!(
            reliable_events(&mut session),
            1,
            "{name} #{index}: one event"
        );
    }
}

/// Test value: red if a removal order makes an intermediate check refuse a legal document: the
/// replace removes a route, the track it reads and the source the track reads.
#[test]
fn a_replace_removes_a_source_and_the_entities_that_read_it() {
    let base = model(BASE);
    let committed = with_dependents(&base);
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = boot(&json(&committed), 1_000_000);
    let before = revision(&session).0;
    let next = json(&base);
    let answer = send(&mut session, 1, next.as_bytes(), next.as_bytes());
    assert_eq!(answer.status, StatusCode::Ok, "{answer:?}");
    assert_eq!(answer.applied, Some((3, EditPath::Rebuild)));
    assert_eq!(snapshot(&session), booted_snapshot(&base, before + 1));
}

/// Test value: red if a replace commits, moves the revision or queues an event for a document
/// that is the committed model (the fixture's own text, spelled other than the snapshot).
#[test]
fn an_identical_document_is_unchanged_and_commits_nothing() {
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = boot(BASE, 1_000_000);
    let before = (revision(&session), snapshot(&session));
    reliable_events(&mut session);
    let answer = send(&mut session, 1, BASE.as_bytes(), BASE.as_bytes());
    assert_eq!(
        answer,
        Reply {
            status: StatusCode::Ok,
            revision: before.0.0,
            applied: Some((0, EditPath::Unchanged)),
            codes: Vec::new(),
        }
    );
    assert_eq!((revision(&session), snapshot(&session)), before);
    assert_eq!(reliable_events(&mut session), 0);
}

/// Test value: red if a replace skips the replay preflight (the retry with no document would be
/// refused, or commit twice), or if the frame did not bind the document (another document under
/// the same request ID would answer the cached reply instead of `REQUEST_ID_REUSE`).
#[test]
fn a_retry_replays_without_the_document_and_another_document_is_request_id_reuse() {
    let base = model(BASE);
    let mut next = base.clone();
    // Nothing automates the submix's fader, so the replace rides the live lanes.
    next.submixes[0].fader.left_db = -6.0;
    let next = json(&next);
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = boot(BASE, 1_000_000);
    let start = revision(&session);
    let frame = replace_frame(7, start, next.as_bytes());
    let length = session
        .command_with_document(&frame, next.as_bytes(), 16_384)
        .expect("reply");
    let first = session.command_response(length).to_vec();
    assert_eq!(reply(&first).applied, Some((1, EditPath::Live)));
    let committed = snapshot(&session);
    // The same frame, no document: the cached reply, nothing committed again.
    let length = session.command(&frame, 16_384).expect("replay");
    assert_eq!(session.command_response(length), first.as_slice());
    assert_eq!(revision(&session).0, start.0 + 1);
    // The same request ID naming another document: other frame bytes.
    let other = json(&base);
    let reuse = replace_frame(7, start, other.as_bytes());
    let length = session
        .command_with_document(&reuse, other.as_bytes(), 16_384)
        .expect("reply");
    assert_eq!(
        reply(session.command_response(length)).status,
        StatusCode::RequestIdReuse
    );
    assert_eq!(snapshot(&session), committed);
}

/// One refusal case: its name, the document the frame names, the document passed, the status
/// and the one diagnostic code (empty for the parser's own diagnostics).
type Refusal<'a> = (&'a str, &'a [u8], &'a [u8], StatusCode, &'a str);

/// Test value: red if a refusal of D4 moves the model, the revision or queues an event before it
/// fails, or answers another status or diagnostic than the registry names.
#[test]
fn every_document_refusal_happens_before_anything_commits() {
    let base = model(BASE);
    let mut bigger = with_dependents(&base);
    bigger.tracks[0].fader.left_db = -1.0;
    let bigger = json(&bigger);
    let mut changed = base.clone();
    changed.tracks[0].fader.left_db = -6.0;
    let changed = json(&changed);
    let mut wrong_digest = changed.clone().into_bytes();
    // Same length, other bytes: the frame's digest no longer matches. The first byte is
    // structural, so the document would not parse either: the digest refusal proves the digest
    // is checked before any parse.
    wrong_digest[0] = b'x';
    let invalid = BASE.replacen("\"sample_rate_hz\"", "\"sample_rate\"", 1);
    let not_utf8 = [0xff_u8; 8];
    let bound = u64::try_from(BASE.len() + 64).expect("small");
    let cases: [Refusal<'_>; 6] = [
        (
            "no document",
            changed.as_bytes(),
            &[],
            StatusCode::InvalidField,
            "session.replace.document_length",
        ),
        (
            "short document",
            changed.as_bytes(),
            &changed.as_bytes()[1..],
            StatusCode::InvalidField,
            "session.replace.document_length",
        ),
        (
            "too large",
            bigger.as_bytes(),
            bigger.as_bytes(),
            StatusCode::LimitExceeded,
            "session.replace.document_too_large",
        ),
        (
            "digest",
            changed.as_bytes(),
            &wrong_digest,
            StatusCode::InvalidField,
            "session.replace.document_digest",
        ),
        (
            "not UTF-8",
            &not_utf8,
            &not_utf8,
            StatusCode::ValidationFailed,
            "session.replace.document_utf8",
        ),
        (
            "invalid",
            invalid.as_bytes(),
            invalid.as_bytes(),
            StatusCode::ValidationFailed,
            "",
        ),
    ];
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = boot(BASE, bound);
    reliable_events(&mut session);
    let before = (revision(&session), snapshot(&session));
    for (request, (name, framed, passed, status, code)) in (1_u64..).zip(cases) {
        let answer = send(&mut session, request, framed, passed);
        assert_eq!(answer.status, status, "{name}: {answer:?}");
        assert_eq!(answer.revision, before.0.0, "{name}");
        if code.is_empty() {
            assert!(!answer.codes.is_empty(), "{name}: the parser's diagnostics");
        } else {
            assert_eq!(answer.codes, [code], "{name}");
        }
        assert_eq!((revision(&session), snapshot(&session)), before, "{name}");
        assert_eq!(reliable_events(&mut session), 0, "{name}");
    }
    // A rate change is refused as a transaction's is, typed and permanent.
    let mut rate = base.clone();
    rate.sample_rate_hz = 44_100;
    let rate = json(&rate);
    let frame = replace_frame(9, before.0, rate.as_bytes());
    let refused = session.command_with_document(&frame, rate.as_bytes(), 16_384);
    assert!(
        matches!(
            refused,
            Err(control_plane::CommandError::CompileRejected(_))
        ),
        "{refused:?}"
    );
    assert_eq!((revision(&session), snapshot(&session)), before);
    assert_eq!(reliable_events(&mut session), 0);
}

/// `tracks` tracks on the nine-track fixture's first source, each with `delays` default delay
/// inserts and a post-pan route to the main output (issue #2194).
fn delay_document(tracks: usize, delays: usize) -> String {
    let mut model = model(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ));
    let mut track = model.tracks[1].clone();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    track.console.clear();
    track.inserts.effects = (0..delays)
        .map(|index| session::Effect {
            id: id(&format!("delay-{index}")),
            identity: session::EffectIdentity::Native {
                effect_id: id("miso.delay"),
            },
            quality: session::EffectQuality::Normal,
            bypass: false,
            link_mode: session::LinkMode::DualMono,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        })
        .collect();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    for index in 0..tracks {
        let track_id = id(&format!("track-{index:03}"));
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        model.tracks.push(next_track);
        let mut next_route = route.clone();
        next_route.id = id(&format!("route-{index:03}"));
        next_route.source = RouteSource::Track {
            track_id,
            tap: session::SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    json(&model)
}

/// Issue #2194 gate F: a `SESSION_REPLACE` whose candidate declares about 1.5 GB of effect state
/// (512 tracks with four delays each) under a 512 MiB replacement peak is refused
/// `control.resource.replacement_peak` before the candidate is prepared: the command's heap peak
/// stays inside the document's parse transient, and the running session is untouched. The effect
/// cap is unbounded here, so only the replacement admission can refuse it.
///
/// Test value: red when the control plane prepares the candidate before it admits it (the command
/// then peaks at about 1.5 GB before the post-preparation admission refuses it), which no other
/// replace test can see: their candidates fit.
#[test]
fn a_replace_whose_effects_cannot_fit_is_refused_before_it_is_prepared() {
    use bench_support::alloc as audited_allocator;
    /// `host_web::PARSE_TRANSIENT_MULTIPLIER`, which this crate does not depend on.
    const PARSE_TRANSIENT_MULTIPLIER: usize = 20;
    audited_allocator::assert_installed();
    let booted = delay_document(1, 0);
    let candidate = delay_document(512, 4);
    let limits = ControlLimits {
        maximum_tracks: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_replacement_peak_bytes: 512 << 20,
        ..limits(u64::try_from(candidate.len()).expect("a bounded document"))
    };
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = compile_children::<Adapter>(&booted, limits, Box::new(Preparer)).expect("boot compile");
    reliable_events(&mut session);
    let before = (revision(&session), snapshot(&session));
    let frame = replace_frame(1, before.0, candidate.as_bytes());
    audited_allocator::current_thread_peak_reset();
    let refused = session.command_with_document(&frame, candidate.as_bytes(), 16_384);
    let peak = audited_allocator::current_thread_peak().peak_bytes;
    let transient = (candidate.len() * PARSE_TRANSIENT_MULTIPLIER) as u64;
    assert!(
        peak <= transient,
        "the replace peaked at {peak} bytes, over the {transient}-byte parse transient"
    );
    let Err(control_plane::CommandError::CompileRejected(failure)) = refused else {
        panic!("a permanent refusal: {refused:?}")
    };
    assert_eq!(
        &*failure.diagnostic(),
        b"control.resource.replacement_peak\t$\n"
    );
    assert_eq!((revision(&session), snapshot(&session)), before);
    assert_eq!(reliable_events(&mut session), 0);
}
