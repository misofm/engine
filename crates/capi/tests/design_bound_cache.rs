//! Issue #1471: the engine's and each session's design-bound caches, through the exported C ABI.
//!
//! The counters are `builtins::test_support`'s, read on this test's thread across one call: a
//! compile and a structural rebuild both run synchronously on the calling thread.

#![allow(unsafe_code)]

use core::ptr;

use builtins::test_support::{fixed_input_bounds_computed, fixed_input_frames_walked};
use capi::*;
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use session::{SessionModel, StableId};

const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");

/// Ceilings far above every document here: the gate is the design bounds, not the limits.
fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1 << 30,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 4_096,
        maximum_sources: 4_096,
        maximum_routes: 8_192,
        maximum_effects: 8_192,
        maximum_graph_session_plus_plan_bytes: 1 << 32,
        maximum_source_total_bytes: 1 << 30,
        maximum_source_overhead_bytes: 1 << 30,
        maximum_effect_state_bytes: 1 << 32,
        maximum_effect_scratch_bytes: 1 << 32,
        maximum_builtin_retained_bytes: 1 << 32,
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 32,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// What a call computed on this thread: designs bounded and frames walked.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Work {
    designs: u64,
    frames: u64,
}

fn measure<T>(call: impl FnOnce() -> T) -> (T, Work) {
    let (designs, frames) = (fixed_input_bounds_computed(), fixed_input_frames_walked());
    let value = call();
    let work = Work {
        designs: fixed_input_bounds_computed() - designs,
        frames: fixed_input_frames_walked() - frames,
    };
    (value, work)
}

struct Engine(*mut capi::Engine);

impl Engine {
    fn new() -> Self {
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        // SAFETY: Valid config and output pointers.
        let code = unsafe { miso_engine_v1_engine_create(&config, &mut engine) };
        assert_eq!(code, RESULT_OK);
        Self(engine)
    }

    /// One `miso_engine_v1_compile_session`, and the work it did.
    fn compile(&self, document: &str) -> (Compiled, Work) {
        let (code, compiled, work, diagnostics) = self.compile_at(document, &limits());
        assert_eq!(code, RESULT_OK, "compile: {diagnostics}");
        (compiled.expect("a compiled session"), work)
    }

    /// One `miso_engine_v1_compile_session` at `compile_limits`: its result code, the children
    /// when it compiled, the work it did and its diagnostics.
    fn compile_at(
        &self,
        document: &str,
        compile_limits: &CompileLimits,
    ) -> (u32, Option<Compiled>, Work, String) {
        let mut storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let (code, work) = measure(|| {
            // SAFETY: Every pointer is live and uniquely owned for the call.
            unsafe {
                miso_engine_v1_compile_session(
                    self.0,
                    document.as_ptr(),
                    document.len() as u64,
                    compile_limits,
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                )
            }
        });
        let text =
            String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize]).into_owned();
        let compiled = (code == RESULT_OK).then_some(Compiled { session, plan });
        (code, compiled, work, text)
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        // SAFETY: The unique live handle, destroyed once; sessions and plans are independent.
        unsafe { miso_engine_v1_engine_destroy(self.0) }
    }
}

struct Compiled {
    session: *mut Session,
    plan: *mut Plan,
}

impl Compiled {
    /// Renders one block at `at` and returns the left plane's bits.
    fn render(&self, at: u64) -> Vec<u32> {
        let mut pcm = [f32::NAN; 256];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        // SAFETY: `plan` is live and `output` describes `pcm`.
        let code = unsafe { miso_engine_v1_render_f32_planar(self.plan, at, &output) };
        assert_eq!(code, RESULT_OK);
        pcm.iter().map(|sample| sample.to_bits()).collect()
    }

    /// Renders one block at `at` so a pending plan swaps in, then drains both event lanes: a
    /// committed rebuild queues reliable events, and the queue holds two.
    fn settle(&self, at: u64) {
        self.render(at);
        let mut storage = [0_u8; 4_096];
        for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
            loop {
                let mut event = BytesOut {
                    struct_size: BYTES_OUT_SIZE,
                    reserved0: 0,
                    data: storage.as_mut_ptr(),
                    capacity_bytes: storage.len() as u64,
                    required_bytes: 0,
                };
                // SAFETY: `session` is live and used by this thread alone.
                let code = unsafe { miso_engine_v1_dequeue_event(self.session, lane, &mut event) };
                assert_eq!(code, RESULT_OK);
                if event.required_bytes == 0 {
                    break;
                }
            }
        }
    }

    /// One structural transaction that renders identically: the source's content identity becomes
    /// `tag`'s. Returns the work its rebuild did. `revision` is the session's current revision.
    fn rebuild(&self, document: &str, revision: u64, tag: u8) -> Work {
        let model = session::parse_session_json(document).expect("the document");
        let source = &model.sources[0];
        let edit = SessionEdit::SetSourceContent {
            source_id: source.id.clone(),
            content: format!("blake3:{}", format!("{tag:02x}").repeat(32)),
            channels: source.channels,
            bit_depth: source.bit_depth,
            frames: source.frames,
        };
        let (code, status, work) = self.submit(&[edit], revision, tag);
        assert_eq!(
            (code, status),
            (RESULT_OK, Some(StatusCode::Ok)),
            "the rebuild commits"
        );
        work
    }

    /// Submits `edits` as the transaction `request_id` at `revision`, and returns the call's
    /// result code, the status of its response when it has one, and the work it did.
    fn submit(
        &self,
        edits: &[SessionEdit],
        revision: u64,
        request_id: u8,
    ) -> (u32, Option<StatusCode>, Work) {
        let mut request = vec![0_u8; 8_192];
        let len = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(u64::from(request_id)).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut request,
            )
            .expect("structural command");
        request.truncate(len);
        let mut response = [0_u8; 4_096];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        let (code, work) = measure(|| {
            // SAFETY: `session` is live; the request and response are complete buffers.
            unsafe {
                miso_engine_v1_submit_command(
                    self.session,
                    request.as_ptr(),
                    request.len() as u64,
                    &mut output,
                )
            }
        });
        if code != RESULT_OK {
            return (code, None, work);
        }
        let mut fields = [0_u16; 64];
        let (protocol::DecodedTypedResponseFrame::Success { header, .. }
        | protocol::DecodedTypedResponseFrame::NonOk { header, .. }) = ProtocolCodec::default()
            .decode_typed_response(
                &response[..output.required_bytes as usize],
                &mut protocol::DecodeScratch::new(&mut fields),
            )
            .expect("a decodable response");
        (code, Some(header.status), work)
    }
}

/// The edit that sets track `index`'s builtins to `document`'s own, with both HPF cutoffs at
/// `hpf_hz` when given.
fn builtins_edit(document: &str, index: usize, hpf_hz: Option<f32>) -> SessionEdit {
    let model = session::parse_session_json(document).expect("the document");
    let track = &model.tracks[index];
    let mut builtins = track.builtins.clone();
    if let Some(hpf_hz) = hpf_hz {
        builtins.left.hpf_hz = hpf_hz;
        builtins.right.hpf_hz = hpf_hz;
    }
    SessionEdit::SetTrackBuiltins {
        track_id: track.id.clone(),
        builtins,
        ramp_samples: None,
    }
}

impl Drop for Compiled {
    fn drop(&mut self) {
        // SAFETY: Each handle is live and destroyed once.
        unsafe {
            miso_engine_v1_plan_destroy(self.plan);
            miso_engine_v1_session_destroy(self.session);
        }
    }
}

/// Cutoffs whose designs are quick to bound: 800 of them fit one preparation's budget.
const CHEAP: (f32, f32) = (1_500.0, 4_000.0);
/// Cutoffs whose designs are slow to bound: 1,200 of them pass one preparation's budget.
const SLOW: (f32, f32) = (200.0, 9_000.0);

/// A session of `count` tracks whose designs are all different from each other and from every
/// design of `offset`'s other documents (each track's cutoffs come from its global index), built
/// from the nine-track fixture's track, source, route and console.
fn filler_document(offset: usize, count: usize, base: (f32, f32)) -> String {
    let mut model: SessionModel = session::parse_session_json(NINE_TRACK).expect("fixture");
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in offset..offset + count {
        let mut track = track.clone();
        let id = format!("f{index}");
        track.id = StableId::parse(&id).expect("track ID");
        // Cutoffs that differ in the low bits of the word too: the key is the designed words.
        let hpf = base.0 + 0.173 * index as f32;
        let lpf = base.1 + 0.311 * index as f32;
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = hpf;
            lane.lpf_hz = lpf;
        }
        let mut route = route.clone();
        route.id = StableId::parse(&format!("{id}-main")).expect("route ID");
        if let session::RouteSource::Track { track_id, .. } = &mut route.source {
            *track_id = track.id.clone();
        }
        model.tracks.push(track);
        model.routes.push(route);
    }
    session::canonical_session_json(&model).expect("canonical document")
}

fn revision(document: &str) -> u64 {
    session::parse_session_json(document)
        .expect("document")
        .revision
}

/// Gate 1: a rebuild of an unchanged session computes no design bound and walks no frame, and a
/// second compile of the same document on the same engine likewise.
#[test]
fn a_rebuild_and_a_recompile_compute_no_design_bound() {
    let engine = Engine::new();
    let (first, cold) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(
        cold.designs, 64,
        "the cold compile bounds every distinct design"
    );
    assert!(cold.frames > 0);
    let revision = revision(SIXTY_FOUR_TRACK);
    // Two rebuilds in a row: the second needs the first's plan swapped in, which a render does.
    let rebuilt = first.rebuild(SIXTY_FOUR_TRACK, revision, 0x11);
    assert_eq!(
        rebuilt,
        Work {
            designs: 0,
            frames: 0
        },
        "the first rebuild"
    );
    first.render(0);
    let rebuilt = first.rebuild(SIXTY_FOUR_TRACK, revision + 1, 0x12);
    assert_eq!(
        rebuilt,
        Work {
            designs: 0,
            frames: 0
        },
        "the second rebuild"
    );

    let (_second, warm) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(
        warm,
        Work {
            designs: 0,
            frames: 0
        },
        "the recompile"
    );
}

/// Gate 1 near the cap (#1457 Amendment 3, m3): the engine's cache holds 8,160 other designs, so
/// the 64-track document's insertions pass `INPUT_BOUND_CACHE_ENTRIES` (8,192) and clear it
/// midway. The session's first rebuild still computes no design bound, because its first epoch
/// owns the designs the compile used (journaled at the time of use) and not the engine's cache
/// after it.
#[test]
fn a_compile_that_clears_the_engine_cache_still_seeds_its_session() {
    const CAP: u64 = 8_192;
    const FILLED: u64 = CAP - 32;
    let engine = Engine::new();
    // Documents of at most 800 distinct designs: each is inside one preparation's budget, so each
    // design is computed and cached (asserted below).
    let mut offset = 0;
    while (offset as u64) < FILLED {
        let count = (FILLED - offset as u64).min(800) as usize;
        let (_compiled, work) = engine.compile(&filler_document(offset, count, CHEAP));
        assert_eq!(
            work.designs, count as u64,
            "filler at {offset} is computed whole"
        );
        offset += count;
    }
    let (session, compile) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(
        compile.designs, 64,
        "no filler design is one of the document's"
    );
    // The clear happened: the 33rd insertion emptied the cache, so a second compile finds only
    // the 32 designs inserted after it and computes the first 32 again.
    let (_again, after_clear) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(
        after_clear.designs, 32,
        "vacuous: the engine's cache did not clear"
    );

    let rebuilt = session.rebuild(SIXTY_FOUR_TRACK, revision(SIXTY_FOUR_TRACK), 0x21);
    assert_eq!(
        rebuilt,
        Work {
            designs: 0,
            frames: 0
        },
        "the session's seed lacks designs the compile used"
    );
}

/// Gate 1 and 2 past the budget: a document of 1,200 slow designs passes
/// `INPUT_BOUND_BUDGET_FRAMES` midway, so its tail reports the live bound from there on. The
/// design that stopped is cached as a horizon, not a bound, and is journaled as one: a warm
/// compile and a rebuild stop it again from the stored horizon and charge the budget what a cold
/// compile does, so they compute no design and walk no frame. (A hit charging nothing would
/// bound designs a cold compile cannot afford, and a seed without the horizon would walk the
/// stopped design again.)
#[test]
fn a_session_past_the_budget_rebuilds_without_walking() {
    let document = filler_document(0, 1_200, SLOW);
    let engine = Engine::new();
    let (first, cold) = engine.compile(&document);
    assert!(
        cold.designs > 0 && cold.designs < 1_200,
        "vacuous: the document does not pass the budget ({} designs)",
        cold.designs
    );
    let (_second, warm) = engine.compile(&document);
    assert_eq!(
        warm,
        Work {
            designs: 0,
            frames: 0
        },
        "the warm compile"
    );
    let rebuilt = first.rebuild(&document, revision(&document), 0x31);
    assert_eq!(
        rebuilt,
        Work {
            designs: 0,
            frames: 0
        },
        "the rebuild"
    );
}

/// Amendment 2 D1: a session keeps the designs its newest plan used and no others. On the 64-track
/// document, a rebuild that changes one strip's HPF computes exactly that strip's design; an
/// unchanged rebuild then computes none; and reverting the strip computes exactly one again,
/// because the seed followed the newest plan and no longer holds the original design.
#[test]
fn a_rebuild_keeps_only_the_designs_its_plan_uses() {
    let engine = Engine::new();
    let (session, cold) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(cold.designs, 64);
    let revision = revision(SIXTY_FOUR_TRACK);
    let commits = (RESULT_OK, Some(StatusCode::Ok));

    // A strip's HPF changes: its new design is the only one computed.
    let (code, status, changed) = session.submit(
        &[builtins_edit(SIXTY_FOUR_TRACK, 7, Some(55.0))],
        revision,
        0x51,
    );
    assert_eq!((code, status), commits, "the HPF edit");
    assert_eq!(changed.designs, 1, "the changed strip's design only");
    assert!(changed.frames > 0);
    session.settle(0);

    // Nothing changes: the seed is the newest plan's 64 designs, and all are held.
    let unchanged = session.rebuild(SIXTY_FOUR_TRACK, revision + 1, 0x52);
    assert_eq!(unchanged, Work::default(), "the unchanged rebuild");
    session.settle(128);

    // The strip goes back: the original design left the seed with the plan that used it.
    let (code, status, reverted) = session.submit(
        &[builtins_edit(SIXTY_FOUR_TRACK, 7, None)],
        revision + 2,
        0x53,
    );
    assert_eq!((code, status), commits, "the revert");
    assert_eq!(reverted.designs, 1, "the original design is computed again");
}

/// The edit that sets track `index`'s builtins to track `from`'s, with both HPF cutoffs at
/// `hpf_hz`: the two strips then share one input design.
fn builtins_like(document: &str, index: usize, from: usize, hpf_hz: f32) -> SessionEdit {
    let model = session::parse_session_json(document).expect("the document");
    let mut builtins = model.tracks[from].builtins.clone();
    builtins.left.hpf_hz = hpf_hz;
    builtins.right.hpf_hz = hpf_hz;
    SessionEdit::SetTrackBuiltins {
        track_id: model.tracks[index].id.clone(),
        builtins,
        ramp_samples: None,
    }
}

/// Amendment 2 D1 (gate 5): a rebuild that is prepared and then refused leaves every seed as it
/// was. The limit is `maximum_capi_retained_bytes`, set one byte below the replacement peak of the
/// edit under test (found as the smallest cap at which a fresh engine commits it): the refusal is
/// the existing resource limit, after the successor was prepared and its peak computed, before
/// any commit. Edit A changes strip 7's HPF (a new design, the seed stays 64 designs). Refused, it
/// has computed that design. Edit B then gives strips 7 and 8 that same design (a 63-design seed,
/// so a peak 240 bytes lower, which the same limit admits): it commits and computes the design
/// again. A rebuild that wrote its designs into the newest seed before its commit would find the
/// design held and compute none.
#[test]
fn a_refused_rebuild_leaves_the_seed_as_it_was() {
    let revision = revision(SIXTY_FOUR_TRACK);
    let edit_a = [builtins_edit(SIXTY_FOUR_TRACK, 7, Some(55.0))];
    let edit_b = [
        builtins_like(SIXTY_FOUR_TRACK, 7, 7, 55.0),
        builtins_like(SIXTY_FOUR_TRACK, 8, 7, 55.0),
    ];
    let commits = (RESULT_OK, Some(StatusCode::Ok));
    let capped = |cap: u64| CompileLimits {
        maximum_capi_retained_bytes: cap,
        ..limits()
    };

    // Whether the document compiles at `cap` and edit A then commits, on a fresh engine.
    let a_commits_at = |cap: u64| {
        let engine = Engine::new();
        let (code, compiled, _, _) = engine.compile_at(SIXTY_FOUR_TRACK, &capped(cap));
        let Some(compiled) = compiled else {
            assert_ne!(code, RESULT_OK);
            return false;
        };
        let (code, status, _) = compiled.submit(&edit_a, revision, 0x61);
        (code, status) == commits
    };
    // The replacement peak: the smallest cap that admits compile and edit A (admission is
    // monotone in the cap).
    let (mut refuses, mut admits) = (0_u64, 1_u64 << 30);
    assert!(a_commits_at(admits), "vacuous: the roomy cap refuses");
    assert!(!a_commits_at(refuses), "vacuous: a zero cap admits");
    while admits - refuses > 1 {
        let middle = refuses + (admits - refuses) / 2;
        if a_commits_at(middle) {
            admits = middle;
        } else {
            refuses = middle;
        }
    }
    let peak = admits;

    let engine = Engine::new();
    let (code, session, _, diagnostics) = engine.compile_at(SIXTY_FOUR_TRACK, &capped(peak - 1));
    assert_eq!(
        code, RESULT_OK,
        "vacuous: the initial compile is itself refused one byte below the peak: {diagnostics}"
    );
    let session = session.expect("a compiled session");

    let (code, status, refused) = session.submit(&edit_a, revision, 0x62);
    assert_ne!(
        (code, status),
        commits,
        "edit A is refused one byte below its peak"
    );
    assert_eq!(
        refused.designs, 1,
        "vacuous: the refused rebuild did not prepare its design"
    );

    let (code, status, committed) = session.submit(&edit_b, revision, 0x63);
    assert_eq!((code, status), commits, "edit B fits the same cap");
    assert_eq!(
        committed.designs, 1,
        "the refused rebuild left its design in the newest seed"
    );
}
