//! Allocation and separate-owner proof for the opt-in delivery service.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell, num::NonZeroUsize};
use std::alloc::{GlobalAlloc, System};

use protocol::*;

struct CountingAllocator;
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
    static FREES: Cell<u64> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ACTIVE.with(|active| {
                if active.get() {
                    ALLOCS.set(ALLOCS.get() + 1)
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

fn measured<T>(operation: impl FnOnce() -> T) -> (T, (u64, u64)) {
    ALLOCS.set(0);
    FREES.set(0);
    ACTIVE.set(true);
    let value = operation();
    ACTIVE.set(false);
    (value, (ALLOCS.get(), FREES.get()))
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

#[test]
fn prepared_heaps_free_off_thread_and_realtime_owner_operations_are_zero_zero() {
    let (pair, preparation) =
        measured(|| PreparedAutomationDelivery::prepare(config(), 1).unwrap());
    assert!(preparation.0 > 0);
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
    assert_eq!(render_counts, (0, 0));

    let (_, control_counts) = measured(|| {
        control.collect_terminal(ticket).unwrap();
    });
    assert_eq!(control_counts, (0, 0));
    let (_, teardown) = measured(|| drop((control, render)));
    assert!(teardown.1 > 0);
}
