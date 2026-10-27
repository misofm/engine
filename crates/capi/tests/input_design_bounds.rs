//! Issues #1556 and #1602: a C ABI plan bounds no input design, through the exported C ABI.
//!
//! Since #1556 every C ABI strip carries the live input lane, whose tail is the rate's live bound
//! and needs no design, and since #1602 nothing caches a design either, so a compile or rebuild
//! that designed one would pay for it in full. The counters are `builtins::test_support`'s, read on this test's thread across one call:
//! a compile and a structural rebuild both run synchronously on the calling thread.

#![allow(unsafe_code)]

use core::ptr;

use builtins::test_support::{fixed_input_bounds_computed, fixed_input_frames_walked};
use capi::*;
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    StatusCode, TypedCommandFrame,
};

const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");

/// Ceilings far above every document here: the gate is the design bounds, not the limits.
fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        reserved1: 0,
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
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
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

/// The edits that set track `index`'s builtins to `document`'s own, with both HPF cutoffs at
/// `hpf_hz` when given.
fn builtins_edit(document: &str, index: usize, hpf_hz: Option<f32>) -> Vec<SessionEdit> {
    let model = session::parse_session_json(document).expect("the document");
    let track = &model.tracks[index];
    let mut builtins = track.builtins.clone();
    if let Some(hpf_hz) = hpf_hz {
        builtins.left.hpf_hz = hpf_hz;
        builtins.right.hpf_hz = hpf_hz;
    }
    protocol::control_core::builtins_edits(&track.id, &builtins)
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

fn revision(document: &str) -> u64 {
    session::parse_session_json(document)
        .expect("document")
        .revision
}

/// Gate 5 (#1556): the 64-track console session compiles and rebuilds without bounding one input
/// design or walking one frame, and an HPF change commits as a live revision (no plan is
/// prepared) that bounds none either. Before the input rows were live the same compile bounded 64
/// distinct designs.
///
/// Red if a strip of a C ABI plan lacks the input lane and falls back to its prepared design's
/// bound, which costs a walk of the filter's response at every compile and rebuild and can make
/// the tail infinite.
#[test]
fn a_c_abi_plan_bounds_no_input_design() {
    let engine = Engine::new();
    let (session, cold) = engine.compile(SIXTY_FOUR_TRACK);
    assert_eq!(cold, Work::default(), "the compile");
    let revision = revision(SIXTY_FOUR_TRACK);
    assert_eq!(
        session.rebuild(SIXTY_FOUR_TRACK, revision, 0x11),
        Work::default(),
        "a rebuild"
    );
    session.settle(0);
    let (code, status, live) = session.submit(
        &builtins_edit(SIXTY_FOUR_TRACK, 7, Some(55.0)),
        revision + 1,
        0x12,
    );
    assert_eq!(
        (code, status),
        (RESULT_OK, Some(StatusCode::Ok)),
        "the HPF edit"
    );
    assert_eq!(live, Work::default(), "a live HPF edit");
    // The edit was live: the plan still renders without a swap.
    session.settle(128);
}
