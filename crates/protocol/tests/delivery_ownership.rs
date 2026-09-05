//! Allocation and separate-owner proof for the opt-in delivery service.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell, num::NonZeroUsize};
use std::alloc::{GlobalAlloc, System};
use std::sync::Barrier;

use protocol::*;

struct CountingAllocator;
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
    static FREES: Cell<u64> = const { Cell::new(0) };
    static BYTES: Cell<u64> = const { Cell::new(0) };
    static LARGEST: Cell<u64> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ACTIVE.with(|active| {
                if active.get() {
                    ALLOCS.set(ALLOCS.get() + 1);
                    BYTES.set(BYTES.get() + layout.size() as u64);
                    LARGEST.set(LARGEST.get().max(layout.size() as u64));
                }
            });
        }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        ACTIVE.with(|active| {
            if active.get() {
                FREES.set(FREES.get() + 1)
            }
        });
        unsafe { System.dealloc(pointer, layout) }
    }
}

fn measured<T>(operation: impl FnOnce() -> T) -> (T, (u64, u64, u64, u64)) {
    ALLOCS.set(0);
    FREES.set(0);
    BYTES.set(0);
    LARGEST.set(0);
    ACTIVE.set(true);
    let value = operation();
    ACTIVE.set(false);
    (
        value,
        (ALLOCS.get(), FREES.get(), BYTES.get(), LARGEST.get()),
    )
}

fn config() -> ProtocolQueueConfig {
    ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(2).unwrap(),
        control_command_bytes: NonZeroUsize::new(256).unwrap(),
        automation_batch_slots: NonZeroUsize::new(2).unwrap(),
        reliable_response_slots: NonZeroUsize::new(2).unwrap(),
        reliable_event_slots: NonZeroUsize::new(2).unwrap(),
        telemetry_slots: NonZeroUsize::new(2).unwrap(),
        per_block_automation_density: NonZeroUsize::new(256).unwrap(),
        quantum_frames: NonZeroUsize::new(64).unwrap(),
    }
}

fn batch(id: u64) -> AutomationBatchSlot {
    AutomationBatchSlot::new(
        SessionRevision(1),
        RequestId::new(id).unwrap(),
        &[AutomationRecord {
            kind: AutomationKind::Point,
            handle: ParameterHandle(7),
            start: SampleTime(id),
            end: SampleTime(id),
            start_value: 0.5,
            end_value: 0.5,
        }],
    )
    .unwrap()
}

fn two_record_batch(id: u64) -> AutomationBatchSlot {
    let first = batch(id).records[0];
    AutomationBatchSlot::new(
        SessionRevision(1),
        RequestId::new(id).unwrap(),
        &[
            first,
            AutomationRecord {
                start: SampleTime(id + 1),
                end: SampleTime(id + 1),
                ..first
            },
        ],
    )
    .unwrap()
}

#[test]
fn prepared_heaps_free_off_thread_and_realtime_owner_operations_are_zero_zero() {
    let (pair, preparation) =
        measured(|| PreparedAutomationDelivery::prepare(config(), 1).unwrap());
    assert!(preparation.0 > 0);
    let report = PreparedAutomationDelivery::resource_report_for_config(config()).unwrap();
    assert_eq!(
        (preparation.2, preparation.3),
        (
            report.retained_payload_bytes,
            report.largest_allocation_bytes
        )
    );
    let (mut control, mut render) = pair;
    let capabilities =
        PreparedDeliveryCapabilities::new_exact(&[(ParameterHandle(7), AutomationKind::Point)])
            .unwrap();
    control.try_admit(SampleTime(0), batch(1)).unwrap();
    let ticket = match control.try_handoff_next(&capabilities).unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };

    let (_, render_counts) = measured(|| {
        let pending = render.begin_boundary(SampleTime(0)).unwrap();
        assert_eq!(
            (pending.ticket, pending.records[0].handle),
            (ticket, ParameterHandle(7))
        );
        render.mark_applied(ticket, 1).unwrap();
        render.finish_applied(ticket, 1).unwrap();
    });
    assert_eq!((render_counts.0, render_counts.1), (0, 0));

    let (_, control_counts) = measured(|| {
        control.collect_terminal(ticket).unwrap();
    });
    assert_eq!((control_counts.0, control_counts.1), (0, 0));
    let teardown = std::thread::scope(|scope| {
        scope
            .spawn(move || measured(|| drop((control, render))).1)
            .join()
            .unwrap()
    });
    assert!(teardown.1 > 0);
}

#[derive(Clone, Copy)]
enum Position {
    Queued,
    HandedOff,
    Partial,
    RacingComplete,
}

#[test]
fn distinct_render_owner_reconciles_all_four_cancellation_positions_without_allocation() {
    for (case, position) in [
        Position::Queued,
        Position::HandedOff,
        Position::Partial,
        Position::RacingComplete,
    ]
    .into_iter()
    .enumerate()
    {
        let (mut control, mut render) =
            PreparedAutomationDelivery::prepare(config(), 100 + case as u64 * 2).unwrap();
        let payload = if matches!(position, Position::Partial) {
            two_record_batch(1)
        } else {
            batch(1)
        };
        control.try_admit(SampleTime(0), payload).unwrap();
        let capabilities =
            PreparedDeliveryCapabilities::new_exact(&[(ParameterHandle(7), AutomationKind::Point)])
                .unwrap();
        let ticket = if matches!(position, Position::Queued) {
            None
        } else {
            Some(match control.try_handoff_next(&capabilities).unwrap() {
                HandoffResult::HandedOff(ticket) => ticket,
                other => panic!("{other:?}"),
            })
        };
        let ready = Barrier::new(2);
        let go = Barrier::new(2);
        let sample = SampleTime(500 + case as u64);
        std::thread::scope(|scope| {
            let worker = scope.spawn(|| {
                if matches!(position, Position::Partial | Position::RacingComplete) {
                    let ticket = ticket.unwrap();
                    render.begin_boundary(SampleTime(1)).unwrap();
                    render
                        .mark_applied(
                            ticket,
                            if matches!(position, Position::Partial) {
                                1
                            } else {
                                payload.len
                            },
                        )
                        .unwrap();
                }
                ready.wait();
                go.wait();
                let (_, counts) = measured(|| {
                    if matches!(position, Position::RacingComplete) {
                        render.finish_applied(ticket.unwrap(), payload.len).unwrap();
                    }
                    assert!(render.begin_boundary(sample).is_none());
                });
                assert_eq!((counts.0, counts.1), (0, 0));
                if let Some(ticket) = ticket {
                    assert_eq!(render.pending(ticket), Err(DeliveryError::Empty));
                }
            });
            ready.wait();
            let token = control
                .begin_cancel(
                    AutomationCancellationReason::EndpointShutdown,
                    SessionRevision(9),
                )
                .unwrap();
            assert!(control.poll_cancel_boundary(token).unwrap().is_none());
            go.wait();
            worker.join().unwrap();
            let complete = control.poll_cancel_boundary(token).unwrap().unwrap();
            assert_eq!(complete.effective_sample, sample);
            if matches!(position, Position::RacingComplete) {
                assert_eq!(
                    (complete.canceled_events, complete.applied_records),
                    (0, u64::from(payload.len))
                );
            } else {
                assert_eq!(complete.canceled_events, 1);
            }
        });
    }
}
