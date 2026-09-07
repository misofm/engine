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

// SAFETY: Allocation and deallocation delegate unchanged pointers and layouts to
// System; the thread-local counters do not access or alter allocation storage.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The caller supplies the valid layout required by GlobalAlloc.
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
        // SAFETY: The caller supplies a live allocation from this allocator and its
        // original layout; allocation was delegated to System without modification.
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

#[cfg(feature = "test-support")]
fn fixture_descriptor() -> ParameterDescriptor {
    ParameterDescriptor {
        handle: 7,
        track_id: "fixture".to_owned(),
        rack: ParameterRack::Dynamic,
        effect_id: "effect".to_owned(),
        parameter_id: 7,
        channel: ParameterChannel::Left,
        value_kind: ParameterValueKind::F32,
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(-1.0),
        maximum: Some(1.0),
        default: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: ParameterAutomationRate::Sample,
        smoothing_samples: 0,
        flags: 3,
        display_name: None,
        display_unit: None,
        enum_choices: Vec::new(),
    }
}

#[cfg(feature = "test-support")]
fn fixture_session() -> SessionStore {
    let source = include_str!("../../../fixtures/session/v1/canonical.json");
    SessionStore::new(
        session::parse_session_json(source).unwrap(),
        session::CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .unwrap()
}

#[cfg(feature = "test-support")]
fn fixture_provider() -> MockProvider {
    MockProvider::try_with_retained_capacity_and_automation(
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
        fixture_descriptor(),
    )
    .unwrap()
}

#[cfg(feature = "test-support")]
fn replay_config() -> ReplayCacheConfig {
    ReplayCacheConfig {
        entries: NonZeroUsize::new(32).unwrap(),
        bytes: NonZeroUsize::new(16 * 1024).unwrap(),
        max_response_bytes: 2048,
    }
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

#[test]
fn generic_boundary_cancel_reports_zero_partial_and_full_without_releasing_credits_early() {
    for (logical_count, applied_prefix) in [(1_u16, 0_u16), (3, 1), (2, 2)] {
        let (mut control, mut render) =
            PreparedDelivery::<u32>::prepare(NonZeroUsize::new(2).unwrap()).unwrap();
        let ticket = control.try_publish(17, logical_count).unwrap();
        if applied_prefix != 0 {
            assert_eq!(render.begin().unwrap(), (ticket, 17));
            render.mark_progress(ticket, applied_prefix).unwrap();
            if applied_prefix == logical_count {
                render.finish(ticket, applied_prefix).unwrap();
            }
        }
        let token = control.begin_cancel().unwrap();
        assert_eq!(control.poll_cancel_boundary(token).unwrap(), None);
        let (_, render_counts) = measured(|| render.cancel_boundary(SampleTime(91)).unwrap());
        assert_eq!((render_counts.0, render_counts.1), (0, 0));
        let complete = control.poll_cancel_boundary(token).unwrap().unwrap();
        assert_eq!(complete.acknowledged_sample, SampleTime(91));
        let result = control.collect(ticket).unwrap();
        assert_eq!(result.applied_prefix, applied_prefix);
        assert_eq!(result.remaining_count, logical_count - applied_prefix);
        assert_eq!(
            result.disposition,
            if applied_prefix == logical_count {
                CoreTerminalDisposition::Applied
            } else {
                CoreTerminalDisposition::Canceled
            }
        );
        assert_eq!(
            result.acknowledged_sample,
            if applied_prefix == logical_count {
                None
            } else {
                Some(SampleTime(91))
            }
        );
        let next = control.try_publish(18, 1).unwrap();
        assert_ne!(next.generation, ticket.generation);
    }
}

#[test]
fn generic_boundary_cancel_has_independent_request_capacity_and_exact_frontier() {
    let (mut control, mut render) =
        PreparedDelivery::<u32>::prepare(NonZeroUsize::new(2).unwrap()).unwrap();
    let first = control.try_publish(1, 1).unwrap();
    let second = control.try_publish(2, 1).unwrap();
    assert_eq!(control.try_publish(3, 1), Err(DeliveryError::Full));
    let token = control.begin_cancel().unwrap();
    assert_eq!(
        control.try_publish(4, 1),
        Err(DeliveryError::CancellationPending)
    );
    render.cancel_boundary(SampleTime(12)).unwrap();
    assert_eq!(
        control
            .poll_cancel_boundary(token)
            .unwrap()
            .unwrap()
            .frontier,
        Some(second.serial)
    );
    assert_eq!(control.collect(first).unwrap().remaining_count, 1);
    assert_eq!(control.collect(second).unwrap().remaining_count, 1);
    assert_eq!(
        control.poll_cancel_boundary(token),
        Err(DeliveryError::StaleTicket)
    );
    let replacement = control.try_publish(5, 1).unwrap();
    assert!(replacement.serial > second.serial);
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

#[cfg(feature = "test-support")]
#[test]
fn controller_facade_preparation_has_one_queue_allocation_authority() {
    let cfg = config();
    let capabilities =
        PreparedDeliveryCapabilities::new_exact(&[(ParameterHandle(7), AutomationKind::Point)])
            .unwrap();
    let session_a = fixture_session();
    let provider_a = fixture_provider();
    let (a, a_counts) = measured(|| {
        ControllerAutomationDelivery::prepare(
            session_a,
            cfg,
            provider_a,
            replay_config(),
            ProtocolCodec::default(),
            ProtocolControllerConfig::default(),
            ControllerRetainedCapacity {
                meter_handles: 0,
                counter_ids: 0,
            },
            capabilities,
        )
        .unwrap()
    });
    let session_b = fixture_session();
    let provider_b = fixture_provider();
    let (b, b_counts) = measured(|| {
        let queues = ProtocolQueues::prepare(cfg).unwrap();
        let replay = ReplayCache::try_new(replay_config()).unwrap();
        ProtocolController::try_with_config_and_retained_capacity(
            session_b,
            queues,
            provider_b,
            replay,
            ProtocolCodec::default(),
            ProtocolControllerConfig::default(),
            ControllerRetainedCapacity {
                meter_handles: 0,
                counter_ids: 0,
            },
        )
        .unwrap()
    });
    let (c, c_counts) = measured(|| PreparedAutomationDelivery::prepare(cfg, 1).unwrap());
    let (d, d_counts) = measured(|| ProtocolQueues::prepare(cfg).unwrap());
    assert_eq!(a_counts.1, 0);
    assert_eq!(b_counts.1, 0);
    assert_eq!(c_counts.1, 0);
    assert_eq!(d_counts.1, 0);
    assert_eq!(a_counts.0, b_counts.0 + c_counts.0 - d_counts.0);
    assert_eq!(a_counts.2, b_counts.2 + c_counts.2 - d_counts.2);
    assert_eq!(
        PreparedAutomationDelivery::resource_report_for_config(cfg).unwrap(),
        a.2.queue_and_delivery
    );
    let _ = (b, c, d);
}
