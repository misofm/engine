//! A latency-growing edit during playback through the exported C entries (issue #1360): the
//! production session's warm successor (D1) is adopted by render, and the transition fallback runs
//! only from a session call's service step past the deadline (D2, D3).
//!
//! The session is the nine-track fixture at 48 kHz in 128-frame blocks over one never-ending
//! source, with the default source ring (`source_ring_frames = 0`). The growth adds a muted copy of
//! the first track with a true-peak limiter insert and its route to the output: `P` = 512, the
//! limiter's `Fs/100 + 6` = 486 rounded up to whole quanta. A muted track adds nothing to the
//! output, so an exact adoption leaves the predecessor's output continued bit for bit (D15-8).

#![allow(unsafe_code)]

use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use capi::*;
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision, StatusCode,
    TypedCommandFrame,
};
use session::{SessionModel, StableId};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const QUANTUM: u32 = 128;
const Q: u64 = QUANTUM as u64;
const FRAMES: usize = QUANTUM as usize;
const FRAME_BYTES: usize = 65_536;
/// `P` of the growth.
const LEAD: u64 = 512;
/// `prime_deadline_samples(48_000, 128)` (#1358 D1).
const DEADLINE: u64 = 7_040;
/// Blocks rendered before the growth.
const BEFORE: u64 = 6;
/// How far past the render clock a feeding host keeps the source queued: well inside the default
/// ring (9,472 frames at 48 kHz and quantum 128), and more than `2P + 2q` past it, so every carried
/// source holds `P + q` past its read position before and after the adoption moves it by `P`.
const FEED_AHEAD: u64 = 4_096;
/// What render waits for before each block of gate 1: `P + q` past the read position, which the
/// adoption moves `P` further ahead.
const READY_AHEAD: u64 = 2 * LEAD + 2 * Q;

fn model() -> SessionModel {
    let mut model = session::parse_session_json(FIXTURE).expect("fixture");
    assert_eq!(
        (model.sample_rate_hz, model.quantum_frames),
        (48_000, QUANTUM)
    );
    model.sources[0].frames = 1 << 40;
    model
}

/// The growth: a muted copy of the first track named `grown`, with a true-peak limiter insert,
/// and its route to the output.
fn growth(model: &SessionModel) -> Vec<SessionEdit> {
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse("grown").expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    track.inserts.effects.push(session::Effect {
        id: StableId::parse("grown-limiter").expect("effect ID"),
        identity: session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.true-peak-limiter").expect("effect ID"),
        },
        quality: session::EffectQuality::Normal,
        bypass: false,
        link_mode: session::LinkMode::Maximum,
        params: Vec::new(),
        sidechain: session::SidechainDeclaration::None,
    });
    let mut route = model.routes[0].clone();
    route.id = StableId::parse("grown-main").expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    vec![
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 0,
        reserved1: 0,
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
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// The source's frame `frame`: never zero, different from frame to frame.
fn pcm(frame: u64) -> [f32; 2] {
    let step = (frame.wrapping_mul(7_919) % 1_999) as f32;
    [(step - 999.5) / 2_000.0, (999.5 - step) / 4_000.0]
}

/// The session half of a compiled session, driven from one control thread.
struct Control {
    session: *mut Session,
    request: u64,
    revision: u64,
    /// The next source frame to submit.
    fed: u64,
}

// SAFETY: The session handle is used by one thread at a time; the ABI's session calls are
// serialized by the thread that owns this value.
unsafe impl Send for Control {}

/// The plan half, rendered on one render thread.
struct Render {
    plan: *mut Plan,
    /// The next block's render sample.
    at: u64,
    /// Every rendered block's output bits.
    blocks: Vec<Vec<u32>>,
}

// SAFETY: The plan handle is rendered by one thread at a time; the watermark query is any-thread.
unsafe impl Send for Render {}

fn boot(model: &SessionModel) -> (Control, Render) {
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
    (
        Control {
            session,
            request: 0,
            revision: model.revision,
            fed: 0,
        },
        Render {
            plan,
            at: 0,
            blocks: Vec::new(),
        },
    )
}

impl Control {
    /// Applies `edits`: the transaction must commit with path `rebuild`. Returns its revision.
    fn apply(&mut self, edits: &[SessionEdit]) -> u64 {
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
        assert_eq!(code, RESULT_OK, "the growth is accepted");
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
            panic!("the growth is refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        assert_eq!(applied.edit_path, EditPath::Rebuild);
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
        self.revision
    }

    /// Submits whole blocks of the source until `until` frames are queued.
    fn feed_to(&mut self, until: u64) {
        while self.fed < until {
            let mut left = [0.0_f32; FRAMES];
            let mut right = [0.0_f32; FRAMES];
            for index in 0..FRAMES {
                [left[index], right[index]] = pcm(self.fed + index as u64);
            }
            let planes = [left.as_ptr(), right.as_ptr()];
            let chunk = SourceChunk {
                struct_size: SOURCE_CHUNK_SIZE,
                sample_rate_hz: 48_000,
                generation: 1,
                start_frame: self.fed,
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
            // SAFETY: The session is live and used by this thread alone; the planes and the
            // report are owned storage that outlives the synchronous call.
            let code = unsafe {
                miso_engine_v1_source_submit_planar_f32(
                    self.session,
                    SOURCE.as_ptr(),
                    SOURCE.len() as u64,
                    &chunk,
                    &mut report,
                )
            };
            assert_eq!(code, RESULT_OK, "frame {}", self.fed);
            assert_eq!(report.accepted_frames, Q);
            self.fed += Q;
        }
    }

    fn service(&mut self) {
        // SAFETY: The session is live and used by this thread alone.
        assert_eq!(unsafe { miso_engine_v1_service(self.session) }, RESULT_OK);
    }
}

impl Render {
    /// Renders the next block.
    fn render(&mut self) {
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
        // SAFETY: This thread is the plan's only renderer; `out` is owned storage.
        let code = unsafe { miso_engine_v1_render_f32_planar(self.plan, self.at, &out) };
        assert_eq!(code, RESULT_OK, "render at {}", self.at);
        self.blocks
            .push(pcm.iter().map(|sample| sample.to_bits()).collect());
        self.at += Q;
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
}

/// Destroys both halves.
fn destroy(control: Control, render: Render) {
    // SAFETY: Both handles are owned here and destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(control.session);
        miso_engine_v1_plan_destroy(render.plan);
    }
}

/// The predecessor rendered on with no edit: `blocks` blocks, the source fed by `feed(block)`
/// frames before each block.
fn predecessor(blocks: u64, feed: impl Fn(u64) -> u64) -> Vec<Vec<u32>> {
    let (mut control, mut render) = boot(&model());
    for block in 0..blocks {
        control.feed_to(feed(block).max(control.fed));
        render.render();
    }
    let bits = core::mem::take(&mut render.blocks);
    destroy(control, render);
    bits
}

/// The first block whose bits differ, if any.
fn first_divergence(left: &[Vec<u32>], right: &[Vec<u32>]) -> Option<usize> {
    assert_eq!(left.len(), right.len());
    left.iter().zip(right).position(|(a, b)| a != b)
}

/// Gate 1, exact through the ABI. A render thread renders 128-frame blocks, each once the source
/// is queued `2P + 2q` past its start, while this control thread keeps the source queued
/// [`FEED_AHEAD`] past the render clock, applies the growth after [`BEFORE`] blocks and then calls
/// `miso_engine_v1_service` in a loop. The watermark reaches the growth's revision with `EXACT`,
/// and the stream equals the predecessor rendered on with no edit, bit for bit.
///
/// Test value: a production session that publishes the growth as an ordinary successor (no
/// `WarmConfig`) floors the output by `P` (a gap), and a host path that does not raise the
/// source-read offset shifts the stream; both differ from the predecessor.
#[test]
fn a_growth_with_sources_queued_ahead_is_adopted_exact() {
    const TOTAL: u64 = 64;
    let (mut control, mut render) = boot(&model());
    let growth = growth(&model());
    let (rendered, fed, done) = (
        &AtomicU64::new(0),
        &AtomicU64::new(0),
        &AtomicBool::new(false),
    );
    let (revision, render) = std::thread::scope(|scope| {
        let worker = scope.spawn(move || {
            for block in 0..TOTAL {
                while fed.load(Ordering::Acquire) < block * Q + READY_AHEAD {
                    assert!(!done.load(Ordering::Acquire), "the feed stopped");
                    std::hint::spin_loop();
                }
                render.render();
                rendered.store(block + 1, Ordering::Release);
            }
            render
        });
        let mut revision = None;
        loop {
            let blocks = rendered.load(Ordering::Acquire);
            control.feed_to(blocks * Q + FEED_AHEAD);
            fed.store(control.fed, Ordering::Release);
            if blocks >= TOTAL {
                break;
            }
            if revision.is_none() && blocks >= BEFORE {
                revision = Some(control.apply(&growth));
            }
            control.service();
            std::thread::yield_now();
        }
        done.store(true, Ordering::Release);
        (revision.expect("applied"), worker.join().expect("render"))
    });
    let watermark = render.watermark();
    assert_eq!(watermark.revision, revision);
    assert_eq!(
        (
            watermark.outcome_flags,
            watermark.exact_count,
            watermark.transition_fallback_count
        ),
        (OUTCOME_EXACT, 1, 0)
    );
    let continued = predecessor(TOTAL, |block| block * Q + FEED_AHEAD);
    assert_eq!(first_divergence(&render.blocks, &continued), None);
    destroy(control, render);
}

/// Gate 2, submit does no adoption work and render adopts without a service call. After
/// [`BEFORE`] blocks the source is queued [`FEED_AHEAD`] past the render clock and, with no render
/// running, the growth's `SESSION_TRANSACTION_APPLY` returns `OK` with path `rebuild` and its
/// revision pending. Render then runs 16 blocks with no further session call: the watermark reads
/// the revision with `EXACT`, and the output equals the predecessor rendered on.
///
/// Test value: a submit that waits for render never returns here, and a design that needs a
/// control call to adopt leaves the revision pending; an ordinary rebuild floors the output.
#[test]
fn render_adopts_the_growth_with_no_session_call() {
    const AFTER: u64 = 16;
    let (mut control, mut render) = boot(&model());
    for _ in 0..BEFORE {
        control.feed_to(render.at + FEED_AHEAD);
        render.render();
    }
    control.feed_to(render.at + FEED_AHEAD);
    let revision = control.apply(&growth(&model()));
    assert!(render.watermark().revision < revision, "pending at submit");
    for _ in 0..AFTER {
        render.render();
    }
    let watermark = render.watermark();
    assert_eq!(
        (watermark.revision, watermark.outcome_flags),
        (revision, OUTCOME_EXACT)
    );
    let continued = predecessor(BEFORE + AFTER, |block| {
        (block * Q + FEED_AHEAD).min(BEFORE * Q + FEED_AHEAD)
    });
    assert_eq!(first_divergence(&render.blocks, &continued), None);
    destroy(control, render);
}

/// Gate 3, no service, no fallback. The source is fed only the block render reads next, so the
/// growth is never ready. After the growth, render runs for twice the deadline with no session
/// call (the source runs dry after one block): the revision stays pending, and the output equals
/// the predecessor rendered on with the same feed. The first `miso_engine_v1_service` call after
/// that publishes the transition, which render adopts at its `S`, past the session's 480-sample
/// mute ramp and one quantum, and the watermark reports `TRANSITION_FALLBACK`.
///
/// Test value: a deadline counted in wall time or in control calls, or a fallback started without
/// a service call (on the render thread), changes when and how the revision completes; an
/// ordinary rebuild completes it `EXACT` with no service call.
#[test]
fn a_growth_never_ready_falls_back_only_at_a_service_call() {
    const WITHHELD: u64 = 2 * DEADLINE / Q;
    let (mut control, mut render) = boot(&model());
    for _ in 0..BEFORE {
        control.feed_to(render.at + Q);
        render.render();
    }
    control.feed_to(render.at + Q);
    let revision = control.apply(&growth(&model()));
    for _ in 0..WITHHELD {
        render.render();
        assert!(
            render.watermark().revision < revision,
            "pending at {}",
            render.at
        );
    }
    let continued = predecessor(BEFORE + WITHHELD, |block| (block + 1).min(BEFORE + 1) * Q);
    assert_eq!(first_divergence(&render.blocks, &continued), None);

    control.service();
    let published = render.at;
    // The transition ducks the new strip and is adopted at its `S`: ceil_q(clock + q + 480).
    let s = (published + Q + 480).div_ceil(Q) * Q;
    while render.at <= s {
        render.render();
    }
    let watermark = render.watermark();
    assert_eq!((watermark.revision, watermark.first_sample), (revision, s));
    assert_eq!(
        (watermark.outcome_flags, watermark.transition_fallback_count),
        (OUTCOME_TRANSITION_FALLBACK, 1)
    );
    destroy(control, render);
}
