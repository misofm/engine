//! Issue #1332 S6: the render half crosses from the instance that booted the host (the Worker) to
//! another instance (the worklet) through the exports, renders there and comes back.
//!
//! Every call goes through the module's exports, each thread standing for one instance: the
//! thread-local `LIVE_HOST`, the render-locked flag and (since S6) the render-locked count are per
//! thread natively, as they are per instance on one shared `WebAssembly.Memory`, and the handoff
//! registry is a process static, as it is one copy in the shared memory. The binary registers the
//! module's own counting allocator, as `plan_exchange_threads.rs` does, so each thread reads its
//! own render-locked count: every allocator call (allocation or free) its render-locked exports
//! made.

#![cfg(feature = "test-support")]

use std::alloc::System;
use std::sync::{Barrier, Mutex};

use host_web::{
    RESULT_INTERNAL, RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_WRONG_STATE, RenderLockedAllocator,
    Reservation, STATE_FAILED, STATE_READY, WebBootOptions, native_staging,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedEventPayload, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, ExpectedRevision, ParameterChannel, ProtocolCodec, RequestId,
    ResponseGetRequest, ResponseGrid, ResponseReadyOutcome, ResponseTarget, TypedCommandFrame,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// A thread that panics while its partner waits on a barrier would hang the test: the guard
/// aborts the binary instead, so a failure is a failure.
struct AbortOnPanic;

impl Drop for AbortOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!("a handoff thread panicked; aborting so its partner does not wait forever");
            std::process::abort();
        }
    }
}

/// The tests share the handoff registry's four entries, so they take turns.
static TURN: Mutex<()> = Mutex::new(());

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM_FRAMES: u32 = 128;
/// Blocks fed before the first render: the ring the options ask for holds them all.
const FED_BLOCKS: usize = 40;
/// A TLS block of the size and alignment a module reports (a native test has no `__tls_size`).
const TLS_SIZE: u32 = 256;
const TLS_ALIGN: u32 = 16;

fn options() -> WebBootOptions {
    WebBootOptions {
        live_control_meter_blocks: 2,
        source_ring_frames: 64 * QUANTUM_FRAMES,
        ..WebBootOptions::explicit_defaults()
    }
}

/// Boot the fixture on this thread's instance and feed `FED_BLOCKS` quanta.
fn boot() -> u32 {
    let length = native_staging::boot(options(), FIXTURE.as_bytes()).expect("the fixture stages");
    let handle = host_web::miso_engine_web_v1_boot(length);
    assert_ne!(handle, 0, "the fixture boots");
    for block in 0..FED_BLOCKS {
        assert!(native_staging::source_id(handle, SOURCE_ID));
        assert!(native_staging::source_pcm(
            handle,
            0.05 + 0.01 * block as f32
        ));
        let result = host_web::miso_engine_web_v1_source_submit(
            handle,
            SOURCE_ID.len() as u32,
            1,
            block as u64 * u64::from(QUANTUM_FRAMES),
            2,
            QUANTUM_FRAMES,
            0,
        );
        assert_eq!(result, RESULT_OK, "block {block}: the feed");
    }
    handle
}

fn frame(request: u64, payload: CommandPayload<'_>) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload,
            },
            &mut bytes,
        )
        .expect("a command frame");
    bytes.truncate(length);
    bytes
}

/// Send one command through the control export and return its decoded success payload's kind
/// check, panicking on a refusal.
fn command(handle: u32, request: u64, payload: CommandPayload<'_>) -> Vec<u8> {
    let bytes = frame(request, payload);
    let (result, response) =
        native_staging::control_command(handle, &bytes).expect("a live control half");
    assert_eq!(result, RESULT_OK, "{}", String::from_utf8_lossy(&response));
    response
}

fn spectrum_configure(handle: u32, request: u64) {
    let response = command(
        handle,
        request,
        CommandPayload::SpectrumConfigure(&protocol::SpectrumConfigureRequest {
            form: protocol::SpectrumForm::Single,
            entries: vec![protocol::SpectrumEntry {
                target_kind: protocol::SpectrumTargetKind::TrackPostPan,
                target_id: "eq0".to_owned(),
                channels: ParameterChannel::Both,
            }],
            maximum_capture_bytes: 1 << 20,
            hop_frames: None,
        }),
    );
    let mut fields = [0_u16; 64];
    assert!(
        matches!(
            ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("a response"),
            DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::SpectrumConfigured(_),
                ..
            }
        ),
        "the spectrum is configured"
    );
}

fn spectrum_arm(handle: u32, request: u64) {
    command(
        handle,
        request,
        CommandPayload::SpectrumCapture(&protocol::SpectrumCaptureRequest::Arm),
    );
}

/// The capture's status after its read.
fn spectrum_read(handle: u32, request: u64) -> protocol::SpectrumCaptureStatus {
    let response = command(
        handle,
        request,
        CommandPayload::SpectrumCapture(&protocol::SpectrumCaptureRequest::Read {
            channels: ParameterChannel::Both,
            include_window: false,
        }),
    );
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::SpectrumCaptured(reply),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("a response")
    else {
        panic!("a capture reply")
    };
    reply.status
}

fn response_get(handle: u32, request: u64) {
    command(
        handle,
        request,
        CommandPayload::ResponseGet(&ResponseGetRequest {
            target: ResponseTarget::Strip("eq0".to_owned()),
            grid: ResponseGrid::Logarithmic,
            points: 64,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
            channels: ParameterChannel::Both,
            sections: false,
            overrides: Vec::new(),
            configuration_id: None,
            sample_rate_hz: None,
            quantum_frames: None,
            maximum_prepared_bytes: None,
            maximum_state_bytes: None,
            maximum_scratch_bytes: None,
            maximum_automation_spans: None,
            maximum_reply_bytes: None,
        }),
    );
}

/// Drain the reliable lane; whether a `RESPONSE_READY` with a ready curve came through.
fn drain_response_ready(handle: u32) -> bool {
    let mut ready = false;
    for _ in 0..16 {
        let (result, event) = native_staging::control_event(handle, 0);
        assert_eq!(result, RESULT_OK);
        if event.is_empty() {
            return ready;
        }
        let mut fields = [0_u16; 1024];
        let decoded = ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("a reliable event");
        if let DecodedEventPayload::ResponseReady(value) = decoded.payload {
            ready |= matches!(value.outcome, ResponseReadyOutcome::Ready(_));
        }
    }
    panic!("the reliable lane does not empty")
}

/// One block's output planes as bits, and the render result.
fn render(handle: u32) -> (u32, Vec<u32>) {
    let result = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
    let bits = native_staging::render_output(handle)
        .map(|(pcm, _, _)| pcm.iter().map(|sample| sample.to_bits()).collect())
        .unwrap_or_default();
    (result, bits)
}

/// Render `blocks` blocks, polling the meters after each as the worklet does; the bits.
fn render_blocks(handle: u32, blocks: usize, bits: &mut Vec<Vec<u32>>) {
    for _ in 0..blocks {
        let (result, block) = render(handle);
        assert_eq!(result, RESULT_OK, "block {}: the render", bits.len());
        let _ = host_web::miso_engine_web_v1_meter_poll(handle);
        bits.push(block);
    }
}

/// The script on one thread: the bits a single instance renders for the same commands, the
/// spectrum configured before the first block (where it adopts in place).
fn single_instance_bits() -> Vec<Vec<u32>> {
    std::thread::spawn(|| {
        let handle = boot();
        spectrum_configure(handle, 1);
        assert_eq!(
            host_web::miso_engine_web_v1_meter_lease(handle, 1),
            RESULT_OK
        );
        let mut bits = Vec::new();
        render_blocks(handle, 4, &mut bits);
        assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
        spectrum_arm(handle, 2);
        render_blocks(handle, 20, &mut bits);
        assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
        assert_eq!(
            spectrum_read(handle, 3),
            protocol::SpectrumCaptureStatus::Ready
        );
        response_get(handle, 4);
        render_blocks(handle, 4, &mut bits);
        assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
        assert!(drain_response_ready(handle));
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        bits
    })
    .join()
    .expect("the single-instance run completes")
}

/// Gate 3. Thread 1 (the Worker) boots and releases; thread 2 (the worklet) binds its reserved
/// stack, adopts, renders with the lease and the poll, releases; thread 1 meanwhile configures
/// and reads the spectrum and asks a strip's response, and at the end adopts and disposes.
///
/// Test value: red if a release or an adoption allocates or frees on either instance (the old
/// `release_render` boxed the shell: thread 1's count moves), if a render export on the released
/// host renders or takes the lease (it must refuse), if the adopted half renders other bits than
/// the single instance (the handoff loses or reorders state), if the spectrum or the
/// strip response cannot be served while the render half is away, or if the dispose after the
/// rejoin frees nothing (the half was not adopted back, so it leaks).
#[test]
fn a_render_half_renders_on_another_instance_without_allocating_and_comes_back() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let expected = single_instance_bits();

    let step = Barrier::new(2);
    let token = Mutex::new(0_u64);
    let reservation: Mutex<Option<Reservation>> = Mutex::new(None);
    let handle_slot = Mutex::new(0_u32);
    let bits = std::thread::scope(|scope| {
        let worklet = scope.spawn(|| {
            let _guard = AbortOnPanic;
            step.wait(); // A: the token is out.
            let handle = *handle_slot.lock().expect("handle");
            let record = reservation
                .lock()
                .expect("reservation")
                .expect("reserved")
                .record;
            assert_eq!(native_staging::instance_bind(record), RESULT_OK);
            assert_eq!(
                native_staging::instance_bind(record),
                RESULT_INVALID_ARGUMENT,
                "an instance binds once"
            );
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            let taken = *token.lock().expect("token");
            assert_eq!(host_web::miso_engine_web_v1_host_adopt(taken), RESULT_OK);
            assert_eq!(
                host_web::miso_engine_web_v1_host_adopt(taken),
                RESULT_WRONG_STATE,
                "the slot is taken"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_meter_lease(handle, 1),
                RESULT_OK
            );
            let mut bits = Vec::new();
            render_blocks(handle, 4, &mut bits);
            step.wait(); // B: four blocks rendered.
            step.wait(); // C: the spectrum is armed.
            render_blocks(handle, 20, &mut bits);
            step.wait(); // D: the capture window is complete.
            step.wait(); // E: the response is asked.
            render_blocks(handle, 4, &mut bits);
            let status = native_staging::render_status(handle).expect("the half's status");
            assert_eq!(status.rendered_quanta, 28);
            assert_eq!(status.state, STATE_READY);
            let returned = host_web::miso_engine_web_v1_host_release(handle);
            assert_ne!(returned, 0, "the release hands back a token");
            assert_eq!(
                host_web::miso_engine_web_v1_render_allocation_count() - before,
                0,
                "the adoption, every render, poll and lease, and the release allocated or freed"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
                RESULT_INVALID_ARGUMENT,
                "the instance holds no session after the release"
            );
            *token.lock().expect("token") = returned;
            step.wait(); // F: the half is back in the registry.
            bits
        });

        // Thread 1, the Worker.
        let worker = scope.spawn(|| {
            let _guard = AbortOnPanic;
            let handle = boot();
            *handle_slot.lock().expect("handle") = handle;
            *reservation.lock().expect("reservation") =
                Some(native_staging::instance_reserve(TLS_SIZE, TLS_ALIGN).expect("a reservation"));
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            let released = host_web::miso_engine_web_v1_host_release(handle);
            assert_ne!(released, 0, "the release hands out a token");
            assert_eq!(
                host_web::miso_engine_web_v1_render_allocation_count() - before,
                0,
                "the release allocated or freed"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_host_release(handle),
                0,
                "a second release has nothing to release"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
                RESULT_WRONG_STATE,
                "the released host's render refuses"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_meter_lease(handle, 1),
                RESULT_WRONG_STATE,
                "the released host's lease refuses"
            );
            assert_eq!(host_web::miso_engine_web_v1_meter_poll(handle), 0);
            // With the half away, the configure publishes a successor, which the first block
            // adopts.
            spectrum_configure(handle, 1);
            assert_eq!(
                native_staging::render_status(handle).map(|status| status.blocking_rebuilds),
                Some(0),
                "a rebuild published while render is elsewhere blocks nothing"
            );
            assert_ne!(
                native_staging::successor_header(handle),
                0,
                "the Worker can name the successor's header"
            );
            *token.lock().expect("token") = released;
            step.wait(); // A
            step.wait(); // B
            let record = reservation
                .lock()
                .expect("reservation")
                .expect("reserved")
                .record;
            assert_eq!(
                native_staging::instance_bind(record),
                RESULT_INVALID_ARGUMENT,
                "a second instance cannot bind a stack another instance runs on"
            );
            assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
            spectrum_arm(handle, 2);
            step.wait(); // C
            step.wait(); // D
            assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
            assert_eq!(
                spectrum_read(handle, 3),
                protocol::SpectrumCaptureStatus::Ready,
                "the capture is read while the half renders elsewhere"
            );
            response_get(handle, 4);
            // While the worklet renders, polls, copies its output and releases, this instance's
            // control count does not move: each instance counts its own (#1332 S6).
            let quiet = host_web::miso_engine_web_v1_control_allocation_count();
            step.wait(); // E
            step.wait(); // F
            assert_eq!(
                host_web::miso_engine_web_v1_control_allocation_count(),
                quiet,
                "the worklet's allocations moved the Worker's control count"
            );
            assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
            assert!(
                drain_response_ready(handle),
                "the strip's response arrives while the half renders elsewhere"
            );
            let back = *token.lock().expect("token");
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            assert_eq!(host_web::miso_engine_web_v1_host_adopt(back), RESULT_OK);
            assert_eq!(
                host_web::miso_engine_web_v1_render_allocation_count() - before,
                0,
                "the adoption back allocated or freed"
            );
            assert_eq!(
                host_web::miso_engine_web_v1_host_adopt(back),
                RESULT_WRONG_STATE,
                "the render half is here"
            );
            let status = native_staging::render_status(handle).expect("status");
            assert_eq!(
                status.rendered_quanta, 28,
                "the host's status is the half's again"
            );
            let control = host_web::miso_engine_web_v1_control_allocation_count();
            assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
            assert!(
                host_web::miso_engine_web_v1_control_allocation_count() > control,
                "the dispose after the rejoin frees the session"
            );
        });
        worker.join().expect("the Worker thread");
        worklet.join().expect("the worklet thread")
    });
    assert_eq!(bits.len(), expected.len());
    for (block, (got, want)) in bits.iter().zip(&expected).enumerate() {
        assert_eq!(got, want, "block {block}: the bits of the single instance");
    }
}

/// A second token into an occupied slot, a stale token and a foreign token.
///
/// Test value: red if a render-only instance takes a second half over the first (the first is
/// lost and the second token consumed), if a stale token adopts anything, or if a host takes back
/// a half another host released (and so frees the wrong session's plan).
#[test]
fn an_occupied_slot_refuses_a_second_token_and_leaves_it_unconsumed() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let first = Barrier::new(3);
    let second = Barrier::new(3);
    let third = Barrier::new(2);
    let tokens = Mutex::new([0_u64; 2]);
    let handles = Mutex::new([0_u32; 2]);
    std::thread::scope(|scope| {
        for host in 0..2 {
            let (first, second, third, tokens, handles) =
                (&first, &second, &third, &tokens, &handles);
            scope.spawn(move || {
                let _guard = AbortOnPanic;
                let handle = boot();
                let token = host_web::miso_engine_web_v1_host_release(handle);
                assert_ne!(token, 0);
                tokens.lock().expect("tokens")[host] = token;
                handles.lock().expect("handles")[host] = handle;
                first.wait();
                second.wait();
                let back = tokens.lock().expect("tokens")[host];
                if host == 0 {
                    // Host 0's own token went to the worklet and came back as a new one; host 1's
                    // token is foreign here.
                    let foreign = tokens.lock().expect("tokens")[1];
                    assert_eq!(
                        host_web::miso_engine_web_v1_host_adopt(foreign),
                        RESULT_INVALID_ARGUMENT,
                        "a host does not take another host's half"
                    );
                }
                // Host 1 adopts after host 0's foreign attempt: while one instance decides on a
                // token, another's attempt with it is refused.
                third.wait();
                assert_eq!(host_web::miso_engine_web_v1_host_adopt(back), RESULT_OK);
                assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
            });
        }
        // The worklet.
        let _guard = AbortOnPanic;
        first.wait();
        let [token_a, token_b] = *tokens.lock().expect("tokens");
        let handle_a = handles.lock().expect("handles")[0];
        assert_eq!(host_web::miso_engine_web_v1_host_adopt(token_a), RESULT_OK);
        assert_eq!(
            host_web::miso_engine_web_v1_host_adopt(token_b),
            RESULT_WRONG_STATE,
            "the second token is refused"
        );
        assert_eq!(
            host_web::miso_engine_web_v1_host_adopt(token_a),
            RESULT_WRONG_STATE,
            "the slot is taken"
        );
        let mut bits = Vec::new();
        render_blocks(handle_a, 2, &mut bits);
        assert_eq!(bits.len(), 2, "the first host still renders");
        let back = host_web::miso_engine_web_v1_host_release(handle_a);
        assert_ne!(back, 0);
        assert_eq!(
            host_web::miso_engine_web_v1_host_adopt(token_a),
            RESULT_INVALID_ARGUMENT,
            "a consumed token adopts nothing"
        );
        tokens.lock().expect("tokens")[0] = back;
        second.wait();
    });
}

/// A stack canary overwritten while the worklet renders is caught at its release.
///
/// Test value: red if `host_release` hands the half back without checking the bound canary, so
/// a stack overflow of the worklet instance after its last block goes unreported and the session
/// keeps running.
#[test]
fn an_overwritten_stack_canary_fails_the_session_at_the_release() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let step = Barrier::new(2);
    let shared = Mutex::new((0_u32, 0_u64, None::<Reservation>));
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let _guard = AbortOnPanic;
            step.wait();
            let (handle, token, reservation) = *shared.lock().expect("shared");
            let reservation = reservation.expect("reserved");
            assert_eq!(native_staging::instance_bind(reservation.record), RESULT_OK);
            assert_eq!(host_web::miso_engine_web_v1_host_adopt(token), RESULT_OK);
            let mut bits = Vec::new();
            render_blocks(handle, 1, &mut bits);
            native_staging::overwrite_canary(&reservation);
            let back = host_web::miso_engine_web_v1_host_release(handle);
            assert_ne!(back, 0, "the half still goes back, failed");
            shared.lock().expect("shared").1 = back;
            step.wait();
        });
        let _guard = AbortOnPanic;
        let handle = boot();
        let reservation =
            native_staging::instance_reserve(TLS_SIZE, TLS_ALIGN).expect("a reservation");
        let token = host_web::miso_engine_web_v1_host_release(handle);
        *shared.lock().expect("shared") = (handle, token, Some(reservation));
        step.wait();
        step.wait();
        let back = shared.lock().expect("shared").1;
        assert_eq!(host_web::miso_engine_web_v1_host_adopt(back), RESULT_OK);
        let status = native_staging::render_status(handle).expect("status");
        assert_eq!(
            status.state, STATE_FAILED,
            "the overflow failed the session"
        );
        assert_eq!(
            host_web::miso_engine_web_v1_service(handle),
            RESULT_WRONG_STATE,
            "the control half refuses after the failure"
        );
        assert!(
            String::from_utf8_lossy(&native_staging::diagnostic(handle))
                .starts_with("web.instance.stack_canary"),
            "the diagnostic names the canary"
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    });
}

/// A stack canary overwritten between two blocks fails the session at the next block.
///
/// Test value: red if the render export checks the canary only at the release, so a worklet whose
/// stack overflowed keeps rendering over corrupted memory until its teardown.
#[test]
fn an_overwritten_stack_canary_fails_the_next_block() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let step = Barrier::new(2);
    let shared = Mutex::new((0_u32, 0_u64, None::<Reservation>));
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let _guard = AbortOnPanic;
            step.wait();
            let (handle, token, reservation) = *shared.lock().expect("shared");
            let reservation = reservation.expect("reserved");
            assert_eq!(native_staging::instance_bind(reservation.record), RESULT_OK);
            assert_eq!(host_web::miso_engine_web_v1_host_adopt(token), RESULT_OK);
            let mut bits = Vec::new();
            render_blocks(handle, 2, &mut bits);
            native_staging::overwrite_canary(&reservation);
            assert_eq!(
                host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
                RESULT_INTERNAL,
                "the first block after the overflow fails"
            );
            let status = native_staging::render_status(handle).expect("status");
            assert_eq!(status.state, STATE_FAILED);
            assert_eq!(
                status.rendered_quanta, 2,
                "the failed block rendered nothing"
            );
            assert!(
                String::from_utf8_lossy(&native_staging::render_diagnostic(handle))
                    .starts_with("web.instance.stack_canary"),
                "the render diagnostic names the canary"
            );
            let back = host_web::miso_engine_web_v1_host_release(handle);
            assert_ne!(back, 0);
            shared.lock().expect("shared").1 = back;
            step.wait();
        });
        let _guard = AbortOnPanic;
        let handle = boot();
        let reservation =
            native_staging::instance_reserve(TLS_SIZE, TLS_ALIGN).expect("a reservation");
        let token = host_web::miso_engine_web_v1_host_release(handle);
        *shared.lock().expect("shared") = (handle, token, Some(reservation));
        step.wait();
        step.wait();
        let back = shared.lock().expect("shared").1;
        assert_eq!(host_web::miso_engine_web_v1_host_adopt(back), RESULT_OK);
        assert_eq!(
            native_staging::render_status(handle).map(|status| status.state),
            Some(STATE_FAILED)
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    });
}

/// A release into a full registry hands back nothing and leaves the half where it was.
///
/// Test value: red if a release that finds every entry taken loses the half (the host can no
/// longer render, or its plan is dropped) or reports a token nothing holds.
#[test]
fn a_release_into_a_full_registry_keeps_the_half() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let full = Barrier::new(host_web::HANDOFF_SLOTS + 1);
    let done = Barrier::new(host_web::HANDOFF_SLOTS + 1);
    std::thread::scope(|scope| {
        for _ in 0..host_web::HANDOFF_SLOTS {
            scope.spawn(|| {
                let _guard = AbortOnPanic;
                let handle = boot();
                let token = host_web::miso_engine_web_v1_host_release(handle);
                assert_ne!(token, 0, "a free entry takes the half");
                full.wait();
                done.wait();
                assert_eq!(host_web::miso_engine_web_v1_host_adopt(token), RESULT_OK);
                assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
            });
        }
        let _guard = AbortOnPanic;
        let handle = boot();
        full.wait();
        assert_eq!(
            host_web::miso_engine_web_v1_host_release(handle),
            0,
            "every entry is taken"
        );
        let mut bits = Vec::new();
        render_blocks(handle, 2, &mut bits);
        assert_eq!(
            native_staging::render_status(handle).map(|status| status.rendered_quanta),
            Some(2),
            "the host still renders: its half stayed"
        );
        done.wait();
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    });
}
