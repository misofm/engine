//! Removing a strip in two phases through the exported C entries (issue #1325, decision 15
//! D15-9): the removed strip ramps out on the running plan, then the successor is adopted at a
//! scheduled sample `S`.
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
        assert_eq!(
            code,
            RESULT_OK,
            "the transaction is accepted: {}",
            self.last_error()
        );
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
        applied.edit_path
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

/// The value-only reference: `b`'s fader and its `pre_fader` send muted, at the session ramp.
fn reference_mutes(model: &SessionModel) -> Vec<SessionEdit> {
    let mut edits = protocol::control_core::fader_edits(
        &id("b"),
        &session::DualMonoFader {
            left_mute: true,
            right_mute: true,
            ..model.tracks[1].fader.clone()
        },
    );
    let mut send = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "b-send")
        .expect("b's send")
        .clone();
    send.mute = true;
    edits.push(SessionEdit::UpsertRoute { route: send });
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
