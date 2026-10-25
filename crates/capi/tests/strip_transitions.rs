//! Removing a strip in two phases through the exported C entries (issue #1325, decision 15
//! D15-9): the removed strip ramps out on the running plan, then the successor is adopted at a
//! scheduled sample `S`. A strip whose successor restarts it is duck-swapped the same way and
//! fades back in after `S` (issue #1324). A route with a lane that a transaction removes or
//! re-points between two kept strips ramps out the same way, and a re-pointed one fades in at its
//! new destination from `S` (issue #1953a).
//!
//! Every run is single-threaded at 48 kHz in 128-frame blocks with a session `mute_ms` that gives
//! `N = 2000` samples and 1024-frame source rings. Tracks `a` and `b` play distinct, never-zero
//! sources; `a` carries a true-peak limiter insert, so `b`'s route into the output carries a
//! compensation delay `C > 0`; `b` also has a `pre_fader` send into submix `r`. Each command is
//! submitted between two render calls, so phase 1's ramps start at the render clock `p`.
//!
//! The reference run is a value-only transaction that mutes `b` and its send at the same point:
//! what phase 1 must sound like, block for block, until the swap and after it.
//!
//! This file has its own test binary so that `bench_support`'s audited allocator serves it, as
//! `plan_swap_race.rs` does.

#![allow(unsafe_code)]

use core::ptr;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use capi::*;
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision, StatusCode,
    TypedCommandFrame,
};
use session::{SessionModel, StableId};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: u32 = 128;
const Q: u64 = QUANTUM as u64;
const FRAMES: usize = QUANTUM as usize;
const FRAME_BYTES: usize = 65_536;
/// `N`, the session's mute ramp: 2000 samples at 48 kHz.
const N: u64 = 2_000;
/// The block after which every command is submitted.
const K: u64 = 6;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// A track of the fixture with no console entry, no insert and its input filters off, reading
/// `source`.
fn plain_track(fixture: &SessionModel, track: &str, source: &str) -> session::Track {
    let mut out = fixture.tracks[0].clone();
    out.id = id(track);
    out.source_id = id(source);
    out.console.clear();
    out.inserts.effects.clear();
    for builtins in [&mut out.builtins.left, &mut out.builtins.right] {
        builtins.hpf_hz = 0.0;
        builtins.lpf_hz = 0.0;
    }
    out
}

/// A route of the fixture from `source` at `tap` to `destination`.
fn route(
    fixture: &SessionModel,
    route_id: &str,
    source: session::RouteSource,
    destination: session::RouteDestination,
) -> session::Route {
    let mut out = fixture.routes[0].clone();
    out.id = id(route_id);
    out.source = source;
    out.destination = destination;
    out.follows_mute = false;
    out
}

fn output(fixture: &SessionModel) -> session::RouteDestination {
    fixture.routes[0].destination.clone()
}

fn track_tap(track: &str, tap: session::SendTap) -> session::RouteSource {
    session::RouteSource::Track {
        track_id: id(track),
        tap,
    }
}

/// The gate session: `a` (limiter insert) and `b` into the output, `b`'s `pre_fader` send into
/// `r`, and `r` into the output.
fn gate_model() -> SessionModel {
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let mut model = fixture.clone();
    model.sample_rate_hz = 48_000;
    model.quantum_frames = QUANTUM;
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.control_smoothing = Some(session::ControlSmoothing {
        mute_ms: 2_000.0 / 48.0,
        fader_ms: 20.0,
        pan_ms: 20.0,
    });
    let mut source = fixture.sources[0].clone();
    source.frames = 1 << 40;
    let mut source_a = source.clone();
    source_a.id = id("src-a");
    let mut source_b = source;
    source_b.id = id("src-b");
    model.sources = vec![source_a, source_b];
    let mut track_a = plain_track(&fixture, "a", "src-a");
    let limiter = session::Effect {
        id: id("lim"),
        identity: session::EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: session::EffectQuality::Normal,
        bypass: false,
        link_mode: session::LinkMode::Maximum,
        params: Vec::new(),
        sidechain: session::SidechainDeclaration::None,
    };
    track_a.inserts.effects.push(limiter);
    model.tracks = vec![track_a, plain_track(&fixture, "b", "src-b")];
    model.submixes = vec![session::Submix::unity(id("r"), &model.console)];
    model.vcas.clear();
    model.automation.clear();
    let out = output(&fixture);
    model.routes = vec![
        route(
            &fixture,
            "a-main",
            track_tap("a", session::SendTap::PostPan),
            out.clone(),
        ),
        route(
            &fixture,
            "b-main",
            track_tap("b", session::SendTap::PostPan),
            out.clone(),
        ),
        route(
            &fixture,
            "b-send",
            track_tap("b", session::SendTap::PreFader),
            session::RouteDestination::SubmixInput { submix_id: id("r") },
        ),
        route(
            &fixture,
            "r-main",
            session::RouteSource::Submix {
                submix_id: id("r"),
                tap: session::SendTap::PostPan,
            },
            out,
        ),
    ];
    model
}

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
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
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 30,
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: FRAME_BYTES as u64,
        maximum_replay_bytes: 1 << 20,
        maximum_replay_entries: 16,
        maximum_submixes: 8,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// Source `src-a`'s frame: a constant pair.
fn frame_a(_frame: u64) -> [f32; 2] {
    [0.25, -0.375]
}

/// Source `src-b`'s frame: a never-zero ramp that differs from frame to frame.
fn frame_b(frame: u64) -> [f32; 2] {
    let step = (frame % 97) as f32 / 97.0;
    [0.125 + 0.25 * step, -0.5 + 0.25 * step]
}

/// One compiled session and its plan, rendered on this thread.
struct Rig {
    session: *mut Session,
    plan: *mut Plan,
    request: u64,
    revision: u64,
}

impl Rig {
    fn boot(model: &SessionModel) -> Self {
        let document = session::canonical_session_json(model).expect("canonical session");
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: Every descriptor and output location lives for the complete call, and the engine
        // handle is destroyed once after compilation.
        unsafe {
            assert_eq!(
                miso_engine_v1_engine_create(&config, &mut engine),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                RESULT_OK,
                "{}",
                String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize])
            );
            miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            request: 0,
            revision: model.revision,
        }
    }

    /// Applies `edits` and returns the edit path of the committed transaction.
    fn apply(&mut self, edits: &[SessionEdit]) -> EditPath {
        let (code, path) = self.try_apply(edits);
        assert_eq!(
            code,
            RESULT_OK,
            "the transaction is accepted: {}",
            self.last_error()
        );
        path.expect("a committed transaction's path")
    }

    /// Submits `edits`; the result code, and the edit path when it committed.
    fn try_apply(&mut self, edits: &[SessionEdit]) -> (u32, Option<EditPath>) {
        self.request += 1;
        let mut request = vec![0_u8; FRAME_BYTES];
        let len = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut request,
            )
            .expect("command frame");
        let mut response = vec![0_u8; FRAME_BYTES];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; every buffer is owned.
        let code = unsafe {
            miso_engine_v1_submit_command(self.session, request.as_ptr(), len as u64, &mut out)
        };
        if code != RESULT_OK {
            return (code, None);
        }
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
        } = ProtocolCodec::default()
            .decode_typed_response(
                &response[..out.required_bytes as usize],
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("transaction reply")
        else {
            panic!("the transaction is refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        self.revision += 1;
        assert_eq!(header.revision, SessionRevision(self.revision));
        let mut event = vec![0_u8; FRAME_BYTES];
        let mut event_out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: event.as_mut_ptr(),
            capacity_bytes: event.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: As above.
        let code = unsafe {
            miso_engine_v1_dequeue_event(self.session, EVENT_LANE_RELIABLE, &mut event_out)
        };
        assert_eq!(code, RESULT_OK, "the commit's reliable event");
        (code, Some(applied.edit_path))
    }

    fn last_error(&self) -> String {
        let mut storage = [0_u8; 4_096];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live; the descriptor names complete owned storage.
        let code = unsafe { miso_engine_v1_last_error(self.session.cast(), &mut out) };
        assert_eq!(code, RESULT_OK);
        String::from_utf8_lossy(&storage[..out.required_bytes as usize]).into_owned()
    }

    /// Submits block `block` of source `name`; the result code.
    fn feed(&mut self, name: &str, block: u64) -> u32 {
        let frame = if name == "src-a" { frame_a } else { frame_b };
        self.feed_frames(name, block, &frame)
    }

    /// Submits block `block` of source `name`, whose frame `f` is `frame(f)`; the result code.
    fn feed_frames(&mut self, name: &str, block: u64, frame: &dyn Fn(u64) -> [f32; 2]) -> u32 {
        let mut left = [0.0_f32; FRAMES];
        let mut right = [0.0_f32; FRAMES];
        for index in 0..FRAMES {
            let [l, r] = frame(block * Q + index as u64);
            left[index] = l;
            right[index] = r;
        }
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: block * Q,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: QUANTUM,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut report = SubmitReport {
            struct_size: SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        // SAFETY: The session is live and used by this thread alone; the planes and the report
        // are owned storage that outlives the synchronous call.
        let code = unsafe {
            miso_engine_v1_source_submit_planar_f32(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                &chunk,
                &mut report,
            )
        };
        if code == RESULT_OK {
            assert_eq!(
                report.accepted_frames, Q,
                "{name}: block {block} is accepted whole"
            );
        }
        code
    }

    /// Renders block `block` with the render thread's allocator counters around the call alone;
    /// the output's bits and the allocations and frees.
    fn render(&mut self, block: u64) -> (Vec<u32>, u64) {
        let mut pcm = [f32::NAN; 2 * FRAMES];
        let out = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM,
            plane_stride_samples: QUANTUM,
            reserved: [0; 2],
        };
        let mark = current_thread_counters();
        // SAFETY: This thread is the plan's only renderer; `out` is owned storage.
        let code = unsafe { miso_engine_v1_render_f32_planar(self.plan, block * Q, &out) };
        let delta = current_thread_delta_since(mark);
        assert_eq!(code, RESULT_OK, "render block {block}");
        (
            pcm.iter().map(|sample| sample.to_bits()).collect(),
            delta.allocations + delta.reallocations + delta.deallocations,
        )
    }

    fn watermark(&self) -> Watermark {
        // SAFETY: The plan is live, and a zeroed watermark is a valid value whose size field is
        // then set for the complete fixed-size write.
        unsafe {
            let mut watermark: Watermark = core::mem::zeroed();
            watermark.struct_size = WATERMARK_SIZE;
            assert_eq!(
                miso_engine_v1_plan_watermark(self.plan, &mut watermark),
                RESULT_OK
            );
            watermark
        }
    }

    fn latency(&self) -> u64 {
        // SAFETY: As in `watermark`.
        unsafe {
            let mut report: PlanResourceReport = core::mem::zeroed();
            report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
            assert_eq!(
                miso_engine_v1_plan_resources(self.plan, &mut report),
                RESULT_OK
            );
            report.latency_samples
        }
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        // SAFETY: Both handles are owned by this rig and destroyed once.
        unsafe {
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

/// The edits that remove `b`, its routes and its source.
fn removal() -> Vec<SessionEdit> {
    vec![
        SessionEdit::RemoveRoute {
            route_id: id("b-main"),
        },
        SessionEdit::RemoveRoute {
            route_id: id("b-send"),
        },
        SessionEdit::RemoveTrack { track_id: id("b") },
        SessionEdit::RemoveSource {
            source_id: id("src-b"),
        },
    ]
}

/// The value-only edits that give strip `strip`'s fader and each of its routes from a tap before
/// the fader the mutes `model` declares, or, when `duck`, mute them all, at the session ramp.
fn strip_mutes(model: &SessionModel, strip: &str, duck: bool) -> Vec<SessionEdit> {
    let fader = model
        .strips()
        .find(|held| held.id.as_str() == strip)
        .expect("the strip")
        .fader
        .clone();
    let fader = if duck {
        session::DualMonoFader {
            left_mute: true,
            right_mute: true,
            ..fader
        }
    } else {
        fader
    };
    let mut edits = protocol::control_core::fader_edits(&id(strip), &fader);
    for route in &model.routes {
        let (source, tap) = match &route.source {
            session::RouteSource::Track { track_id, tap } => (track_id, tap),
            session::RouteSource::Submix { submix_id, tap } => (submix_id, tap),
        };
        if source.as_str() == strip && tap.precedes_fader() {
            let mut route = route.clone();
            route.mute |= duck;
            edits.push(SessionEdit::UpsertRoute { route });
        }
    }
    edits
}

/// The value-only reference: `b`'s fader and its `pre_fader` send muted, at the session ramp.
fn reference_mutes(model: &SessionModel) -> Vec<SessionEdit> {
    strip_mutes(model, "b", true)
}

/// The value-only unmutes that undo [`reference_mutes`]: `b`'s fader and its send as `model`
/// declares them, at the session ramp.
fn reference_unmutes(model: &SessionModel) -> Vec<SessionEdit> {
    strip_mutes(model, "b", false)
}

/// The edits that restore `b`, its routes and its source exactly as `model` declares them.
fn restore(model: &SessionModel) -> Vec<SessionEdit> {
    let mut edits = vec![
        SessionEdit::UpsertSource {
            source: model.sources[1].clone(),
        },
        SessionEdit::UpsertTrack {
            track: model.tracks[1].clone(),
        },
    ];
    edits.extend(
        model
            .routes
            .iter()
            .filter(|route| route.id.as_str().starts_with("b-"))
            .map(|route| SessionEdit::UpsertRoute {
                route: route.clone(),
            }),
    );
    edits
}

/// A muted track `name` reading `src-a`, and its route into the output.
fn muted_track(name: &str) -> Vec<SessionEdit> {
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let mut track = plain_track(&fixture, name, "src-a");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let main = route(
        &fixture,
        &format!("{name}-main"),
        track_tap(name, session::SendTap::PostPan),
        output(&fixture),
    );
    vec![
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route: main },
    ]
}

/// Asserts that block `block` equals the reference's, bit for bit, naming the first sample that
/// differs.
fn same(bits: &[u32], reference: &[u32], block: u64) {
    if let Some(index) = bits
        .iter()
        .zip(reference)
        .position(|(got, want)| got != want)
    {
        panic!(
            "block {block} differs from the reference at sample {index}: {} != {}",
            f32::from_bits(bits[index]),
            f32::from_bits(reference[index])
        );
    }
}

/// `S = ceil_q(p + q + N + C)`.
fn scheduled(p: u64, compensation: u64) -> u64 {
    (p + Q + N + compensation).div_ceil(Q) * Q
}

/// The reference run: boot, play blocks `0..=K` with both sources fed one block ahead, apply the
/// value-only mutes, then play through block `last`. The output bits of every block.
fn reference_run(last: u64) -> Vec<Vec<u32>> {
    reference_run_unmuting(last, None)
}

/// [`reference_run`], with the value-only unmutes applied before block `unmute` when one is
/// given.
fn reference_run_unmuting(last: u64, unmute: Option<u64>) -> Vec<Vec<u32>> {
    let model = gate_model();
    let mut rig = Rig::boot(&model);
    let mut blocks = Vec::new();
    for source in ["src-a", "src-b"] {
        assert_eq!(rig.feed(source, 0), RESULT_OK);
    }
    for block in 0..=last {
        if block == K + 1 {
            assert_eq!(rig.apply(&reference_mutes(&model)), EditPath::Live);
        }
        if Some(block) == unmute {
            assert_eq!(rig.apply(&reference_unmutes(&model)), EditPath::Live);
        }
        for source in ["src-a", "src-b"] {
            assert_eq!(rig.feed(source, block + 1), RESULT_OK);
        }
        blocks.push(rig.render(block).0);
    }
    blocks
}

/// Gates 1, 2 and 6 (#1325): a removal fades the strip and its pre-fader send out over `N` on the
/// running plan, then swaps at `S`. Every block equals the value-only reference's, the watermark
/// reports the removal first at `S`, exactly, the removed source keeps feeding until the
/// adoption and is refused after it, and no block from the commit through the adoption allocates.
#[test]
fn a_removed_strip_ramps_out_then_leaves_at_s() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    let model = gate_model();
    let mut rig = Rig::boot(&model);
    let compensation = rig.latency();
    let p = (K + 1) * Q;
    let s = scheduled(p, compensation);
    // The limiter's delay must reach past the rounding of `S` without it, or an `S` without `C`
    // would cut nothing this gate can hear.
    assert!(
        compensation > scheduled(p, 0) - (p + Q + N),
        "C = {compensation} is longer than the rounding slack"
    );
    let last = s / Q + 8;
    let reference = reference_run(last);

    for source in ["src-a", "src-b"] {
        assert_eq!(rig.feed(source, 0), RESULT_OK);
    }
    let mut feeding = true;
    let mut adopted = None;
    let mut fallbacks = None;
    for block in 0..=last {
        if block == K + 1 {
            let before = rig.watermark();
            fallbacks = Some(before.transition_fallback_count);
            assert_eq!(rig.apply(&removal()), EditPath::Rebuild);
        }
        assert_eq!(rig.feed("src-a", block + 1), RESULT_OK);
        if feeding {
            assert_eq!(
                rig.feed("src-b", block + 1),
                RESULT_OK,
                "block {block}: a submit before the adoption feeds the ramp"
            );
        } else if block == adopted.expect("adopted") / Q + 1 {
            assert_eq!(
                rig.feed("src-b", block + 1),
                RESULT_INVALID_ARGUMENT,
                "the first submit after the adoption is refused"
            );
            assert!(rig.last_error().contains("source.id.unknown"));
        }
        let (bits, churn) = rig.render(block);
        if block > K && adopted.is_none_or(|sample| block * Q <= sample) {
            assert_eq!(
                churn, 0,
                "block {block} allocates nothing on the render thread"
            );
        }
        same(&bits, &reference[block as usize], block);
        let watermark = rig.watermark();
        if block > K && adopted.is_none() {
            if watermark.revision >= rig.revision {
                assert_eq!(watermark.revision, rig.revision);
                assert_eq!(watermark.first_sample, s, "the removal is adopted at S");
                assert_eq!(block * Q, s, "and never earlier");
                assert_eq!(watermark.outcome_flags, OUTCOME_EXACT);
                assert_eq!(Some(watermark.transition_fallback_count), fallbacks);
                adopted = Some(s);
                feeding = false;
            } else {
                assert!(block * Q < s, "block {block}: the removal is reported by S");
            }
        }
    }
    assert!(adopted.is_some(), "the removal is adopted");
}

/// Gates 3 and 5 (#1325): supersession during phase 1. Without a render in between, `t1` adds a
/// muted track `c` (its candidate keeps `src-b`, so `b`'s producer moves into `t1`'s set) and
/// `t2` removes `b`, superseding `t1`. Mid-ramp, `t3` adds a muted track `d`, superseding `t2`:
/// `b` is in the running plan's duck overlay, so `t3` writes no second mute and inherits `S`.
/// Every block up to `S` equals gate 1's reference, the adoption is at exactly gate 1's `S`, and
/// `src-b` keeps feeding the ramp until the adoption and is refused after it.
#[test]
fn a_supersession_in_phase_one_keeps_s_and_the_feed() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    let model = gate_model();
    let mut rig = Rig::boot(&model);
    let s = scheduled((K + 1) * Q, rig.latency());
    let reference = reference_run(s / Q);

    for source in ["src-a", "src-b"] {
        assert_eq!(rig.feed(source, 0), RESULT_OK);
    }
    let mut adopted = None;
    for block in 0..=s / Q + 2 {
        if block == K + 1 {
            assert_eq!(rig.apply(&muted_track("c")), EditPath::Rebuild);
            assert_eq!(rig.apply(&removal()), EditPath::Rebuild);
        }
        if block == K + 4 {
            assert_eq!(rig.apply(&muted_track("d")), EditPath::Rebuild);
        }
        assert_eq!(rig.feed("src-a", block + 1), RESULT_OK);
        match adopted {
            None => assert_eq!(
                rig.feed("src-b", block + 1),
                RESULT_OK,
                "block {block}: a submit before the adoption feeds the ramp"
            ),
            Some(sample) if block == sample / Q + 1 => {
                assert_eq!(rig.feed("src-b", block + 1), RESULT_INVALID_ARGUMENT);
                assert!(rig.last_error().contains("source.id.unknown"));
            }
            Some(_) => {}
        }
        let (bits, _) = rig.render(block);
        if block * Q < s {
            same(&bits, &reference[block as usize], block);
        }
        let watermark = rig.watermark();
        if block > K && adopted.is_none() && watermark.revision >= rig.revision {
            assert_eq!(watermark.revision, rig.revision);
            assert_eq!(watermark.first_sample, s, "the newest candidate inherits S");
            assert_eq!(block * Q, s);
            assert_eq!(watermark.outcome_flags & OUTCOME_TRANSITION_FALLBACK, 0);
            adopted = Some(s);
        }
    }
    assert!(adopted.is_some(), "the last candidate is adopted");
}

/// Gate 4 (#1325 D6): a restore of `b` in phase 1 commits and fades `b` back in from `S`. Mid-ramp
/// (after block `K + 4`) a transaction restores `b`, its routes and its source exactly as they
/// were; it supersedes the removal's candidate and inherits its `S`. The reference is gate 1's
/// reference continued: right after the block that ends at `S`, a value-only transaction unmutes
/// `b`'s fader and its `pre_fader` send at the session ramp, so both ramps start at `S`. Every
/// block of the restore run equals it bit for bit, before `S` and after it, through the end of
/// the fade-in. `src-b` is fed one block ahead only (its ring cannot hold the ramp), and every
/// submit before and after the restore is accepted: the restored source keeps its ring and its
/// producer, so `b` plays from `S` the frames submitted for those blocks.
#[test]
fn a_restore_in_phase_one_fades_back_in_from_s() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    let model = gate_model();
    let mut rig = Rig::boot(&model);
    let s = scheduled((K + 1) * Q, rig.latency());
    let last = (s + N) / Q + 8;
    let reference = reference_run_unmuting(last, Some(s / Q));

    for source in ["src-a", "src-b"] {
        assert_eq!(rig.feed(source, 0), RESULT_OK);
    }
    let mut adopted = None;
    for block in 0..=last {
        if block == K + 1 {
            assert_eq!(rig.apply(&removal()), EditPath::Rebuild);
        }
        if block == K + 5 {
            assert_eq!(rig.apply(&restore(&model)), EditPath::Rebuild);
        }
        for source in ["src-a", "src-b"] {
            assert_eq!(
                rig.feed(source, block + 1),
                RESULT_OK,
                "block {block}: {source} is fed before and after the restore"
            );
        }
        let (bits, _) = rig.render(block);
        same(&bits, &reference[block as usize], block);
        let watermark = rig.watermark();
        if block > K + 4 && adopted.is_none() && watermark.revision >= rig.revision {
            assert_eq!(watermark.revision, rig.revision);
            assert_eq!(watermark.first_sample, s, "the restore inherits S");
            assert_eq!(block * Q, s, "and is never adopted earlier");
            adopted = Some(s);
        }
    }
    assert!(adopted.is_some(), "the restore is adopted");
}

// Issue #1324a: a strip whose successor restarts it (an insert added, removed or reordered,
// `delay_samples`, a prepared parameter) ducks with its `pre_fader` send over `N` on the running
// plan, the successor is adopted at `S`, and the successor's armed fader and send fade it back in.

/// A native effect `insert` of kind `effect`, at its default parameters.
fn native(insert: &str, effect: &str) -> session::Effect {
    session::Effect {
        id: id(insert),
        identity: session::EffectIdentity::Native {
            effect_id: id(effect),
        },
        quality: session::EffectQuality::Normal,
        bypass: false,
        link_mode: session::LinkMode::Maximum,
        params: Vec::new(),
        sidechain: session::SidechainDeclaration::None,
    }
}

/// The compressor every duck-swap below adds to `b`: no latency, so no node's arrival grows.
fn compressor(insert: &str) -> session::Effect {
    session::Effect {
        link_mode: session::LinkMode::DualMono,
        ..native(insert, "miso.compressor")
    }
}

/// The gate session with `a` muted in the model when `a_muted` (it then adds exact `+0.0`, so the
/// output is `b`'s and `r`'s alone) and `b` holding `inserts`.
fn duck_model(a_muted: bool, inserts: Vec<session::Effect>) -> SessionModel {
    let mut model = gate_model();
    model.tracks[0].fader.left_mute = a_muted;
    model.tracks[0].fader.right_mute = a_muted;
    model.tracks[1].inserts.effects = inserts;
    model
}

/// The transaction that gives `b` the declaration `model` holds for it.
fn upsert_b(model: &SessionModel) -> Vec<SessionEdit> {
    vec![SessionEdit::UpsertTrack {
        track: model.tracks[1].clone(),
    }]
}

/// What [`duck_run`] rendered.
struct DuckRun {
    blocks: Vec<Vec<u32>>,
    /// The watermark's first sample and flags for the last transaction's revision, and the
    /// fallback count before the first transaction and after the adoption.
    adopted: Option<(u64, u64, [u64; 2])>,
    /// The render thread's allocations and frees from the block after the first transaction on.
    churn: u64,
}

/// Boot `model`, play blocks `0..=last` with both sources fed one block ahead (`src-b`'s frame
/// `f` is `frame(f)`), and before each block `b` of `edits` apply its transaction, which must
/// commit on `path`.
fn duck_run(
    model: &SessionModel,
    edits: &[(u64, Vec<SessionEdit>, EditPath)],
    last: u64,
    frame: &dyn Fn(u64) -> [f32; 2],
) -> DuckRun {
    let mut rig = Rig::boot(model);
    let feed = |rig: &mut Rig, block: u64| {
        assert_eq!(rig.feed("src-a", block), RESULT_OK);
        assert_eq!(rig.feed_frames("src-b", block, frame), RESULT_OK);
    };
    feed(&mut rig, 0);
    let mut run = DuckRun {
        blocks: Vec::new(),
        adopted: None,
        churn: 0,
    };
    let mut fallbacks = None;
    let final_block = edits.iter().map(|(block, ..)| *block).max();
    for block in 0..=last {
        for (at, transaction, path) in edits {
            if *at == block {
                fallbacks.get_or_insert(rig.watermark().transition_fallback_count);
                assert_eq!(
                    rig.apply(transaction),
                    *path,
                    "the transaction at block {at}"
                );
            }
        }
        feed(&mut rig, block + 1);
        let (bits, churn) = rig.render(block);
        if edits.first().is_some_and(|(at, ..)| block >= *at) {
            run.churn += churn;
        }
        run.blocks.push(bits);
        let watermark = rig.watermark();
        if final_block.is_some_and(|at| block >= at)
            && run.adopted.is_none()
            && watermark.revision >= rig.revision
        {
            assert_eq!(watermark.revision, rig.revision);
            run.adopted = Some((
                watermark.first_sample,
                watermark.outcome_flags,
                [
                    fallbacks.expect("a transaction"),
                    watermark.transition_fallback_count,
                ],
            ));
        }
    }
    run
}

/// A fresh plan of `model` with track `strip`'s fader and its routes from a tap before the fader
/// muted, fed `src-b` from frame `offset` on (its frame `f` plays `frame_b(offset + f)`), given the
/// value-only unmutes of all of them before block `fire`: the oracle of a duck-swap's fade-in from
/// `S = offset`. Blocks `0..blocks`.
fn fresh_fade_run(
    model: &SessionModel,
    strip: &str,
    offset: u64,
    fire: u64,
    blocks: u64,
) -> Vec<Vec<u32>> {
    let mut muted = model.clone();
    for track in &mut muted.tracks {
        if track.id.as_str() == strip {
            track.fader.left_mute = true;
            track.fader.right_mute = true;
        }
    }
    for route in &mut muted.routes {
        if matches!(&route.source, session::RouteSource::Track { track_id, tap }
            if track_id.as_str() == strip && tap.precedes_fader())
        {
            route.mute = true;
        }
    }
    let mut rig = Rig::boot(&muted);
    let shifted = |frame: u64| frame_b(offset + frame);
    let feed = |rig: &mut Rig, block: u64| {
        assert_eq!(rig.feed("src-a", block), RESULT_OK);
        assert_eq!(rig.feed_frames("src-b", block, &shifted), RESULT_OK);
    };
    feed(&mut rig, 0);
    (0..blocks)
        .map(|block| {
            if block == fire {
                assert_eq!(rig.apply(&strip_mutes(model, strip, false)), EditPath::Live);
            }
            feed(&mut rig, block + 1);
            rig.render(block).0
        })
        .collect()
}

/// #1324a gates 1 and 8. `a` is muted, `b` audible; after block `K` one transaction adds a
/// compressor insert to `b`, and `b` holds nothing (case 1) or a true-peak limiter (case 2, so
/// `b`'s `D` is the limiter's latency) before it. (a) Every block before `S` equals the reference
/// that live-mutes `b` and its `pre_fader` send at the same point. (b) From `S` every block equals
/// a fresh plan of the successor session fed the same source frames, with `b` and its send muted
/// and live-unmuted at its block `ceil_q(D)/q`. (d) The watermark reports the revision first at
/// `S = ceil_q(p + q + N + C)`, `C` the longest line of a route out of `b` in the running plan
/// (case 1: `b-main`'s, the output latency; case 2: none), as `EXACT` only, with no fallback.
/// Gate 8: no block from the transaction through the fade allocates or frees on the render thread.
///
/// Test value: red if the successor carries `b`'s ducked fader or its send (they stay muted), does
/// not arm them (they pop in at `S`), fires without waiting `D`, schedules `S` without `C` or
/// before the duck ends, or reports the planned duck-swap as a fallback.
#[test]
fn a_restarted_strip_ducks_then_fades_in_after_s() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    for limited in [false, true] {
        let before = duck_model(
            true,
            if limited {
                vec![native("blim", "miso.true-peak-limiter")]
            } else {
                Vec::new()
            },
        );
        let mut after = before.clone();
        after.tracks[1].inserts.effects.push(compressor("comp"));
        let latency = Rig::boot(&before).latency();
        assert!(latency > 0, "a's limiter delays the output");
        let p = (K + 1) * Q;
        let s = scheduled(p, if limited { 0 } else { latency });
        let fire = if limited { latency.div_ceil(Q) } else { 0 };
        let last = s / Q + fire + N.div_ceil(Q) + 4;
        let what = if limited { "limited b" } else { "plain b" };
        let reference = duck_run(
            &before,
            &[(K + 1, reference_mutes(&before), EditPath::Live)],
            last,
            &frame_b,
        );
        let run = duck_run(
            &before,
            &[(K + 1, upsert_b(&after), EditPath::Rebuild)],
            last,
            &frame_b,
        );
        let (first, flags, [before_count, after_count]) = run.adopted.expect("adopted");
        assert_eq!(first, s, "{what}: the duck-swap is adopted at S");
        assert_eq!(flags, OUTCOME_EXACT, "{what}");
        assert_eq!(before_count, after_count, "{what}: no fallback");
        assert_eq!(run.churn, 0, "{what}: the render thread allocates nothing");
        for block in 0..s / Q {
            same(
                &run.blocks[block as usize],
                &reference.blocks[block as usize],
                block,
            );
        }
        let fresh = fresh_fade_run(&after, "b", s, fire, last + 1 - s / Q);
        for block in s / Q..=last {
            same(
                &run.blocks[block as usize],
                &fresh[(block - s / Q) as usize],
                block,
            );
        }
        assert!(
            fresh
                .last()
                .expect("a block")
                .iter()
                .any(|bits| f32::from_bits(*bits) != 0.0),
            "{what}: the fade is heard"
        );
    }
}

/// #1324a gate 2. `a` and `b` audible, `b`'s source exact zeros: gate 1's transaction leaves every
/// block equal to a run with no transaction, the duck-swap's adoption included.
///
/// Test value: red if the duck-swap ducks or restarts a strip the edit did not touch.
#[test]
fn an_untouched_strip_plays_through_a_duck_swap() {
    assert_installed();
    let before = duck_model(false, Vec::new());
    let mut after = before.clone();
    after.tracks[1].inserts.effects.push(compressor("comp"));
    let s = scheduled((K + 1) * Q, Rig::boot(&before).latency());
    let last = s / Q + N.div_ceil(Q) + 4;
    let silent = |_: u64| [0.0_f32; 2];
    let reference = duck_run(&before, &[], last, &silent);
    let run = duck_run(
        &before,
        &[(K + 1, upsert_b(&after), EditPath::Rebuild)],
        last,
        &silent,
    );
    assert_eq!(run.adopted.map(|(first, ..)| first), Some(s));
    for block in 0..=last {
        same(
            &run.blocks[block as usize],
            &reference.blocks[block as usize],
            block,
        );
    }
    assert!(
        reference.blocks[last as usize]
            .iter()
            .any(|bits| f32::from_bits(*bits) != 0.0),
        "a is heard"
    );
}

/// The longest run of frames from `from` on in which both output channels are exact `+0.0`, and
/// where it starts.
fn longest_silence(blocks: &[Vec<u32>], from: u64) -> (u64, u64) {
    let frames = blocks.len() as u64 * Q;
    let silent = |frame: u64| {
        let block = &blocks[(frame / Q) as usize];
        let index = (frame % Q) as usize;
        f32::from_bits(block[index]) == 0.0 && f32::from_bits(block[FRAMES + index]) == 0.0
    };
    let (mut best, mut start, mut run) = ((0, 0), 0, 0);
    for frame in from..frames {
        if silent(frame) {
            if run == 0 {
                start = frame;
            }
            run += 1;
            if run > best.0 {
                best = (run, start);
            }
        } else {
            run = 0;
        }
    }
    best
}

/// #1324a gate 3. `a` is muted and `b` holds a true-peak limiter then a compressor, with 64
/// samples of `delay_samples`. Each edit that restarts `b` (an insert added, removed, the inserts
/// reordered, `delay_samples`, the limiter's prepared `lookahead_ms`) is a rebuild that ducks `b`:
/// the output holds at least `q` exact-zero frames on both channels, a run that reaches `S`. A
/// value-only fader edit and a re-pointed `pre_fader` send (D15-9: ramped at route level, never a
/// duck of its source) produce no such run. An insert's quality is not listed: every launch
/// effect declares only `normal`, so no session can change one.
///
/// Test value: red if an edit kind restarts `b` without the duck (no silence before `S`), or if a
/// re-pointed send or a fader edit ducks its strip.
#[test]
fn every_restarting_edit_ducks_and_a_send_or_value_edit_does_not() {
    assert_installed();
    let mut base = duck_model(
        true,
        vec![native("blim", "miso.true-peak-limiter"), compressor("comp")],
    );
    let builtins = &mut base.tracks[1].builtins;
    for channel in [&mut builtins.left, &mut builtins.right] {
        channel.delay_samples = 64;
    }
    let edit = |change: &dyn Fn(&mut SessionModel)| {
        let mut after = base.clone();
        change(&mut after);
        after
    };
    let ducking: [(&str, SessionModel); 5] = [
        (
            "an insert added",
            edit(&|model| model.tracks[1].inserts.effects.push(compressor("comp2"))),
        ),
        (
            "an insert removed",
            edit(&|model| model.tracks[1].inserts.effects.truncate(1)),
        ),
        (
            "the inserts reordered",
            edit(&|model| model.tracks[1].inserts.effects.reverse()),
        ),
        (
            "delay_samples",
            edit(&|model| {
                model.tracks[1].builtins.left.delay_samples = 32;
                model.tracks[1].builtins.right.delay_samples = 32;
            }),
        ),
        (
            "lookahead_ms",
            edit(&|model| {
                model.tracks[1].inserts.effects[0].params = vec![session::EffectParam {
                    parameter_id: 3,
                    channel: session::ParameterChannel::Both,
                    unit: session::ParameterUnit::Milliseconds,
                    value: 2.0,
                }];
            }),
        ),
    ];
    let p = (K + 1) * Q;
    let last = p / Q + (2 * N).div_ceil(Q) + 16;
    for (what, after) in &ducking {
        let run = duck_run(
            &base,
            &[(K + 1, upsert_b(after), EditPath::Rebuild)],
            last,
            &frame_b,
        );
        let (s, ..) = run.adopted.expect("adopted");
        let (length, start) = longest_silence(&run.blocks, p);
        assert!(
            length >= Q && start < s && start + length >= s,
            "{what}: {length} silent frames from {start}, S = {s}"
        );
    }
    let mut quieter = base.tracks[1].fader.clone();
    quieter.left_db = -6.0;
    quieter.right_db = -6.0;
    let repointed = edit(&|model| {
        for route in &mut model.routes {
            if route.id.as_str() == "b-send" {
                route.source = track_tap("b", session::SendTap::PostFader);
            }
        }
    });
    let send = repointed
        .routes
        .iter()
        .find(|route| route.id.as_str() == "b-send")
        .expect("b's send")
        .clone();
    let quiet: [(&str, Vec<SessionEdit>, EditPath); 2] = [
        (
            "a fader edit",
            protocol::control_core::fader_edits(&id("b"), &quieter),
            EditPath::Live,
        ),
        (
            "a re-pointed send",
            vec![SessionEdit::UpsertRoute { route: send }],
            EditPath::Rebuild,
        ),
    ];
    for (what, transaction, path) in quiet {
        let run = duck_run(&base, &[(K + 1, transaction, path)], last, &frame_b);
        assert!(run.adopted.is_some(), "{what}");
        let (length, start) = longest_silence(&run.blocks, p);
        assert!(length < Q, "{what}: {length} silent frames from {start}");
    }
}

/// #1324a gate 6 (#1324 D5). Gate 1's duck-swap (case 1), and after block `K + 3` a second
/// transaction removes the compressor again. It commits, writes `b` no second mute (`b` and its
/// send are in the running plan's duck overlay) and is adopted at exactly gate 1's `S`, which
/// counts `C`, the line of `b`'s route into the output. Every block equals the reference that
/// live-mutes `b` and its send after block `K` and live-unmutes both right after the block that
/// ends at `S`: the revert restores the strip with the fade-in at `S`, from the ducked level.
///
/// Test value: red if a revert during the duck leaves `b` muted (a base without the overlay), steps
/// it in at full gain, writes a second ramped mute (the blocks before `S` differ), or moves `S`.
#[test]
fn a_revert_during_the_duck_fades_back_in_at_s() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    let before = duck_model(true, Vec::new());
    let mut after = before.clone();
    after.tracks[1].inserts.effects.push(compressor("comp"));
    let s = scheduled((K + 1) * Q, Rig::boot(&before).latency());
    let last = (s + N) / Q + 8;
    let reference = duck_run(
        &before,
        &[
            (K + 1, reference_mutes(&before), EditPath::Live),
            (s / Q, reference_unmutes(&before), EditPath::Live),
        ],
        last,
        &frame_b,
    );
    let run = duck_run(
        &before,
        &[
            (K + 1, upsert_b(&after), EditPath::Rebuild),
            (K + 4, upsert_b(&before), EditPath::Rebuild),
        ],
        last,
        &frame_b,
    );
    assert_eq!(
        run.adopted.map(|(first, ..)| first),
        Some(s),
        "the revert inherits S"
    );
    for block in 0..=last {
        same(
            &run.blocks[block as usize],
            &reference.blocks[block as usize],
            block,
        );
    }
}

// Issue #1324b: a carried effect keyed from a restarted strip ducks with it (D1's sidechain
// closure), and `C` counts the sidechain lines a successor can shorten (D4).

/// A compressor `insert` whose `sidechain-in` port reads `source`, at threshold `threshold_db`.
fn keyed_compressor(
    insert: &str,
    source: session::RouteSource,
    threshold_db: f32,
) -> session::Effect {
    session::Effect {
        params: vec![session::EffectParam {
            parameter_id: 1,
            channel: session::ParameterChannel::Both,
            unit: session::ParameterUnit::Db,
            value: threshold_db,
        }],
        sidechain: session::SidechainDeclaration::Routed(session::Sidechain {
            source,
            port_id: id("sidechain-in"),
        }),
        ..compressor(insert)
    }
}

/// #1324b gate 7 through the C entries. `b` and its send are muted in the model; `a` is audible
/// and holds only a compressor keyed from `b`'s `tap`. Gate 1's transaction adds a compressor to
/// `b`. (a) From `b`'s `insert_return` tap, `a` reads `b`'s restarted chain, so it is duck-swapped
/// with `b`: every block before `S` equals the reference that live-mutes `a` at the same point, and
/// from `S` every block equals a fresh plan of the successor with `a` muted and live-unmuted at its
/// first block. (b) From `b`'s `input` tap, the raw source, `a` carries: every block equals a run
/// with no transaction. Bit-identical.
///
/// Test value: red if the duck set ignores sidechain edges (`a`'s compressor reads `b`'s old chain
/// until `S`, then the restarted chain's at-rest output: no duck before `S`), or if it also adds
/// the consumers of an `input` tap (`a` ducks needlessly).
#[test]
fn a_sidechain_consumer_ducks_with_its_restarted_source() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    for (tap, ducks) in [
        (session::SendTap::InsertReturn, true),
        (session::SendTap::Input, false),
    ] {
        let mut before = duck_model(false, Vec::new());
        before.tracks[0].inserts.effects =
            vec![keyed_compressor("key", track_tap("b", tap), -18.0)];
        before.tracks[1].fader.left_mute = true;
        before.tracks[1].fader.right_mute = true;
        for route in &mut before.routes {
            if route.id.as_str() == "b-send" {
                route.mute = true;
            }
        }
        let mut after = before.clone();
        after.tracks[1].inserts.effects.push(compressor("comp"));
        let s = scheduled((K + 1) * Q, 0);
        let last = s / Q + N.div_ceil(Q) + 4;
        let what = format!("{tap:?}");
        let run = duck_run(
            &before,
            &[(K + 1, upsert_b(&after), EditPath::Rebuild)],
            last,
            &frame_b,
        );
        assert_eq!(run.adopted.map(|(first, ..)| first), Some(s), "{what}");
        let reference = if ducks {
            duck_run(
                &before,
                &[(K + 1, strip_mutes(&before, "a", true), EditPath::Live)],
                last,
                &frame_b,
            )
        } else {
            duck_run(&before, &[], last, &frame_b)
        };
        let carried_through = if ducks { s / Q } else { last + 1 };
        for block in 0..carried_through {
            same(
                &run.blocks[block as usize],
                &reference.blocks[block as usize],
                block,
            );
        }
        if ducks {
            let fresh = fresh_fade_run(&after, "a", s, 0, last + 1 - s / Q);
            for block in s / Q..=last {
                same(
                    &run.blocks[block as usize],
                    &fresh[(block - s / Q) as usize],
                    block,
                );
            }
        }
        assert!(
            run.blocks[last as usize]
                .iter()
                .any(|bits| f32::from_bits(*bits) != 0.0),
            "{what}: a is heard"
        );
    }
}

/// #1324b gate 9's session: `t2` (a true-peak limiter, latency `L`) and the plain `t4` route from
/// their `post_fader` taps into submix `bu`, which holds a true-peak limiter and routes into the
/// output, so `t4`'s line into `bu` is `L`; `t3` holds two true-peak limiters, then a compressor
/// keyed from `bu`'s `input` tap at a threshold every nonzero key exceeds, and routes into the
/// output. So `t3`'s sidechain line is `L` and the output arrives at `2L`.
fn gate_nine_model() -> SessionModel {
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let mut model = gate_model();
    let mut t2 = plain_track(&fixture, "t2", "src-b");
    t2.inserts.effects = vec![native("lim2", "miso.true-peak-limiter")];
    let t4 = plain_track(&fixture, "t4", "src-a");
    let mut t3 = plain_track(&fixture, "t3", "src-a");
    t3.inserts.effects = vec![
        native("lim3", "miso.true-peak-limiter"),
        native("lim3b", "miso.true-peak-limiter"),
        keyed_compressor(
            "key",
            session::RouteSource::Submix {
                submix_id: id("bu"),
                tap: session::SendTap::Input,
            },
            -80.0,
        ),
    ];
    model.tracks = vec![t2, t3, t4];
    let mut bu = session::Submix::unity(id("bu"), &model.console);
    bu.inserts
        .effects
        .push(native("bulim", "miso.true-peak-limiter"));
    model.submixes = vec![bu];
    let out = output(&fixture);
    model.routes = vec![
        route(
            &fixture,
            "bu-main",
            session::RouteSource::Submix {
                submix_id: id("bu"),
                tap: session::SendTap::PostPan,
            },
            out.clone(),
        ),
        route(
            &fixture,
            "t2-bu",
            track_tap("t2", session::SendTap::PostFader),
            session::RouteDestination::SubmixInput {
                submix_id: id("bu"),
            },
        ),
        route(
            &fixture,
            "t4-bu",
            track_tap("t4", session::SendTap::PostFader),
            session::RouteDestination::SubmixInput {
                submix_id: id("bu"),
            },
        ),
        route(
            &fixture,
            "t3-main",
            track_tap("t3", session::SendTap::PostPan),
            out,
        ),
    ];
    model
}

/// #1324b gate 9 through the C entries. One transaction removes `t2` and `t4`, adds `t1` (two
/// true-peak limiters, reading `t2`'s source) routed from its `post_fader` tap into `bu`, and
/// removes `bu`'s limiter. `bu` restarts and `t3` stays carried under D1's exemption (the only line
/// into `bu`'s `Input` stage comes from added `t1`), while `t3`'s sidechain line shrinks from `L`
/// to 0. (a) The watermark's first sample is `S = ceil_q(p + q + N + 2L)`: `C` is `t3`'s sidechain
/// line `L` plus the longest line into `bu`, `t4`'s `L`; every route line out of a ducked strip is
/// at most `L`. (b) Every block before `t1`'s fire block (the first at or after `S + 2L`) equals the
/// reference that makes no structural edit and live-mutes `t2`, `t4` and `bu` at the same point.
/// Bit-identical.
///
/// Test value: red if `C` counts route lines only or omits the line into `bu` (`S` lands before
/// the key's path empties, so the successor drops the last of the fade from `t3`'s detector), or
/// if the exemption is missing (`t3` ducks needlessly).
#[test]
fn a_shortened_sidechain_line_from_a_restarted_submix_drops_no_fade_sample() {
    assert_installed();
    let _ = session::parse_session_json(r#"{"warm":0}"#);
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let before = gate_nine_model();
    let latency = Rig::boot(&before).latency();
    let p = (K + 1) * Q;
    let s = scheduled(p, latency);
    assert!(
        latency > scheduled(p, 0) - (p + Q + N),
        "the sidechain line reaches past the rounding of S without it"
    );
    let mut t1 = plain_track(&fixture, "t1", "src-b");
    t1.inserts.effects = vec![
        native("lim1", "miso.true-peak-limiter"),
        native("lim1b", "miso.true-peak-limiter"),
    ];
    let mut bu = before.submixes[0].clone();
    bu.inserts.effects.clear();
    let edit = vec![
        SessionEdit::RemoveRoute {
            route_id: id("t2-bu"),
        },
        SessionEdit::RemoveTrack { track_id: id("t2") },
        SessionEdit::RemoveRoute {
            route_id: id("t4-bu"),
        },
        SessionEdit::RemoveTrack { track_id: id("t4") },
        SessionEdit::UpsertTrack { track: t1 },
        SessionEdit::UpsertRoute {
            route: route(
                &fixture,
                "t1-bu",
                track_tap("t1", session::SendTap::PostFader),
                session::RouteDestination::SubmixInput {
                    submix_id: id("bu"),
                },
            ),
        },
        SessionEdit::UpsertSubmix { submix: bu },
    ];
    let fire = (s + latency).div_ceil(Q);
    let run = duck_run(&before, &[(K + 1, edit, EditPath::Rebuild)], fire, &frame_b);
    assert_eq!(run.adopted.map(|(first, ..)| first), Some(s));
    let mut mutes = strip_mutes(&before, "t2", true);
    mutes.extend(strip_mutes(&before, "t4", true));
    mutes.extend(strip_mutes(&before, "bu", true));
    let reference = duck_run(&before, &[(K + 1, mutes, EditPath::Live)], fire, &frame_b);
    for block in 0..fire {
        same(
            &run.blocks[block as usize],
            &reference.blocks[block as usize],
            block,
        );
    }
    assert!(
        reference.blocks[(fire - 1) as usize]
            .iter()
            .any(|bits| f32::from_bits(*bits) != 0.0),
        "t3 is heard"
    );
}

/// #1953a's session: the gate session plus `a`'s `pre_fader` send `a-send` into `r`, so `r`'s
/// input arrives `L` (the limiter's latency) late and `b-send` carries a compensation line of `L`,
/// and a second submix `r2` into the output that nothing feeds.
fn route_duck_model() -> SessionModel {
    let fixture = session::parse_session_json(FIXTURE).expect("fixture");
    let mut model = gate_model();
    model
        .submixes
        .push(session::Submix::unity(id("r2"), &model.console));
    model.routes.push(route(
        &fixture,
        "a-send",
        track_tap("a", session::SendTap::PreFader),
        session::RouteDestination::SubmixInput { submix_id: id("r") },
    ));
    model.routes.push(route(
        &fixture,
        "r2-main",
        session::RouteSource::Submix {
            submix_id: id("r2"),
            tap: session::SendTap::PostPan,
        },
        output(&fixture),
    ));
    model
        .routes
        .sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
    model
}

/// The route `route_id` of `model`, changed by `change`, as one upsert.
fn upsert_route(
    model: &SessionModel,
    route_id: &str,
    change: &dyn Fn(&mut session::Route),
) -> Vec<SessionEdit> {
    let mut route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == route_id)
        .expect("the route")
        .clone();
    change(&mut route);
    vec![SessionEdit::UpsertRoute { route }]
}

/// #1953a (D15-9: a re-point is a removal plus an addition). After block `K` one transaction
/// (a) removes `b`'s `pre_fader` send `b-send` into `r`, or (b) re-points it into `r2`; `b` and
/// both buses stay. Each is a rebuild whose phase 1 ramps the send out over `N` on the running plan
/// and whose successor is adopted at `S = ceil_q(p + q + N + C)`, `C` the send's line `L`, as
/// `EXACT` with no fallback, and no block from the transaction on allocates or frees on the render
/// thread. (a) Every block equals the reference that live-mutes `b-send` at the same point. (b)
/// Every block equals a plan with a muted copy `b-send2` of the send into `r2` that live-mutes
/// `b-send` at the same point and live-unmutes `b-send2` at `S`: the old bus hears (a), the new
/// bus hears a fade from the adoption block, and `b`'s own route to the output is untouched.
///
/// Test value: (a) red on a removal that cuts the send at the adoption (no phase-1 route duck) or
/// whose `S` ignores the send's line (its last ducked frames cut at `S`); (b) red on a re-point
/// treated as a value change (a step at either bus), one that ducks `b`, or one whose new route
/// starts at its prepared value instead of fading in.
#[test]
fn a_removed_or_repointed_send_ramps_out_then_in_at_s() {
    assert_installed();
    let base = route_duck_model();
    let latency = Rig::boot(&base).latency();
    let p = (K + 1) * Q;
    let s = scheduled(p, latency);
    assert!(
        latency > scheduled(p, 0) - (p + Q + N),
        "C = {latency} is longer than the rounding slack"
    );
    let last = s / Q + (N + latency).div_ceil(Q) + 8;
    let mute_send = upsert_route(&base, "b-send", &|route| route.mute = true);
    let mut with_copy = base.clone();
    let mut copy = with_copy
        .routes
        .iter()
        .find(|route| route.id.as_str() == "b-send")
        .expect("b's send")
        .clone();
    copy.id = id("b-send2");
    copy.destination = session::RouteDestination::SubmixInput {
        submix_id: id("r2"),
    };
    copy.mute = true;
    with_copy.routes.push(copy);
    with_copy
        .routes
        .sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
    let unmute_copy = upsert_route(&with_copy, "b-send2", &|route| route.mute = false);
    let removed = vec![SessionEdit::RemoveRoute {
        route_id: id("b-send"),
    }];
    let repointed = upsert_route(&base, "b-send", &|route| {
        route.destination = session::RouteDestination::SubmixInput {
            submix_id: id("r2"),
        };
    });
    let cases: [(&str, Vec<SessionEdit>, DuckRun); 2] = [
        (
            "(a) removed",
            removed,
            duck_run(
                &base,
                &[(K + 1, mute_send.clone(), EditPath::Live)],
                last,
                &frame_b,
            ),
        ),
        (
            "(b) re-pointed",
            repointed,
            duck_run(
                &with_copy,
                &[
                    (K + 1, mute_send, EditPath::Live),
                    (s / Q, unmute_copy, EditPath::Live),
                ],
                last,
                &frame_b,
            ),
        ),
    ];
    for (what, transaction, reference) in cases {
        let run = duck_run(
            &base,
            &[(K + 1, transaction, EditPath::Rebuild)],
            last,
            &frame_b,
        );
        let (first_sample, flags, [before, after]) = run.adopted.expect("adopted");
        assert_eq!(first_sample, s, "{what}: the successor is adopted at S");
        assert_eq!(flags, OUTCOME_EXACT, "{what}");
        assert_eq!(before, after, "{what}: no transition fallback");
        assert_eq!(run.churn, 0, "{what}: render allocates nothing");
        for (block, (bits, want)) in run.blocks.iter().zip(&reference.blocks).enumerate() {
            same(bits, want, block as u64);
        }
    }
}
