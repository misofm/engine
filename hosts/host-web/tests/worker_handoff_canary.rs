//! Issue #1332 S6: a stack canary of the worklet instance's reserved stack, overwritten as a stack
//! overflow would, fails the session. Split from `worker_handoff.rs` so that each process stays
//! below its four instance reservations (each test here reserves one; they are never freed).

#![cfg(feature = "test-support")]

mod handoff;

use std::alloc::System;
use std::sync::{Barrier, Mutex};

use handoff::{AbortOnPanic, QUANTUM_FRAMES, TLS_ALIGN, TLS_SIZE, boot, render_blocks};
use host_web::{
    RESULT_INTERNAL, RESULT_OK, RESULT_WRONG_STATE, RenderLockedAllocator, Reservation,
    STATE_FAILED, native_staging,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The tests share the handoff registry's four entries, so they take turns.
static TURN: Mutex<()> = Mutex::new(());
// A process holds at most four instance reservations (never freed): this binary's tests reserve
// two.

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
