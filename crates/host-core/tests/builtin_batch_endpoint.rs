//! Focused product gates for the prepared typed builtin batch endpoint.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;

use builtins::{BuiltinLaneSelector, Matrix2x2};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord};
use host_core::{
    BuiltinBatch, BuiltinBatchAdmissionError, BuiltinBatchRecord, BuiltinBatchRenderReport,
    HostConsoleRequest, HostPrepareCaps, HostShapePolicy, SourceSubmission,
    prepare_builtin_batch_endpoint, prepare_host_runtime_with_console,
};
use protocol::{CoreTerminalDisposition, SampleTime, SessionRevision};
use std::sync::mpsc::sync_channel;

const REVISION: SessionRevision = SessionRevision(42);
const QUANTUM: usize = 128;
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn assert_send<T: Send>() {}

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 256,
        maximum_tracks: 64,
        maximum_sources: 64,
        maximum_routes: 128,
        maximum_effects: 128,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
    }
}

fn endpoint() -> (
    host_core::BuiltinBatchControl,
    host_core::StartedBuiltinBatchRender,
    host_core::BuiltinBatchResources,
) {
    endpoint_with_capacity(4)
}

fn endpoint_with_capacity(
    capacity: usize,
) -> (
    host_core::BuiltinBatchControl,
    host_core::StartedBuiltinBatchRender,
    host_core::BuiltinBatchResources,
) {
    let compiled = host_core::compile_host_session(SESSION, &caps()).expect("compile fixture");
    prepare_builtin_batch_endpoint(
        &compiled,
        &caps(),
        REVISION,
        NonZeroUsize::new(capacity).unwrap(),
    )
    .expect("prepared endpoint")
    .start()
    .unwrap_or_else(|_| panic!("started endpoint"))
}

fn render_block(
    render: &mut host_core::StartedBuiltinBatchRender,
    first: u64,
) -> BuiltinBatchRenderReport {
    let mut samples = [0.0_f32; QUANTUM * 2];
    render
        .render(&mut samples, 2, QUANTUM, QUANTUM, SampleTime(first))
        .expect("render block")
}

/// Independent layout oracle for the endpoint's retained queue rows. These private-shape mirrors
/// intentionally live in the qualification test: the production report remains the source of
/// caps, while this calculation checks the concrete queue payloads and ledger storage separately.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct OracleMessage {
    ticket: protocol::CoreTicket,
    payload: BuiltinBatch,
    logical_count: u16,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct OracleTerminal {
    ticket: protocol::CoreTicket,
    applied_prefix: u16,
    record_count: u16,
    disposition: CoreTerminalDisposition,
    acknowledged_sample: Option<SampleTime>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum OracleBoundary {
    Cancel {
        token: protocol::CoreCancelToken,
        frontier: Option<u64>,
    },
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct OracleCancelAck {
    token: protocol::CoreCancelToken,
    acknowledged_sample: SampleTime,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct OracleEntry {
    ticket: protocol::CoreTicket,
    payload: BuiltinBatch,
    published: bool,
    logical_count: u16,
    terminal: Option<OracleTerminal>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct OracleOutcome {
    ticket: protocol::CoreTicket,
    requested_sample: SampleTime,
    actual_sample: SampleTime,
}

fn oracle_queue_bytes<T: Copy + Send + 'static>(capacity: usize) -> (u64, u64) {
    let layout =
        engine::realtime::bounded_spsc_retained_payload::<T>(NonZeroUsize::new(capacity).unwrap())
            .expect("oracle queue layout");
    (
        u64::try_from(layout.total_bytes().unwrap()).unwrap(),
        u64::try_from(layout.largest_allocation_bytes()).unwrap(),
    )
}

fn oracle_endpoint_delivery(capacity: usize) -> (u64, u64) {
    let mut total = 0_u64;
    let mut largest = 0_u64;
    for (bytes, largest_row) in [
        oracle_queue_bytes::<OracleMessage>(capacity),
        oracle_queue_bytes::<OracleTerminal>(capacity),
        oracle_queue_bytes::<OracleBoundary>(1),
        oracle_queue_bytes::<OracleCancelAck>(1),
    ] {
        total += bytes;
        largest = largest.max(largest_row);
    }
    let entries = core::alloc::Layout::array::<Option<OracleEntry>>(capacity)
        .expect("oracle entries layout")
        .size() as u64;
    (total + entries, largest.max(entries))
}

fn oracle_outcome(capacity: usize) -> (u64, u64) {
    oracle_queue_bytes::<OracleOutcome>(capacity)
}

#[test]
fn prepared_and_control_halves_have_transferable_ownership() {
    assert_send::<host_core::PreparedBuiltinBatchEndpoint>();
    assert_send::<host_core::PreparedBuiltinBatchRender>();
    assert_send::<host_core::BuiltinBatchControl>();
}

#[test]
fn retained_endpoint_report_matches_independent_concrete_layout_oracle() {
    let capacity = 4;
    let (_, _, resources) = endpoint_with_capacity(capacity);
    let (delivery_bytes, delivery_largest) = oracle_endpoint_delivery(capacity);
    let (outcome_bytes, outcome_largest) = oracle_outcome(capacity);
    assert_eq!(resources.delivery.retained_payload_bytes, delivery_bytes);
    assert_eq!(
        resources.delivery.largest_allocation_bytes,
        delivery_largest
    );
    assert_eq!(resources.outcome.retained_payload_bytes, outcome_bytes);
    assert_eq!(resources.outcome.largest_allocation_bytes, outcome_largest);
    assert_eq!(
        resources.endpoint_retained_heap_bytes,
        delivery_bytes + outcome_bytes
    );
    assert_eq!(
        resources.largest_endpoint_heap_allocation_bytes,
        delivery_largest.max(outcome_largest)
    );
}

#[test]
fn repeated_render_and_cancellation_boundaries_are_allocation_free() {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;

    bench_alloc::assert_installed();
    let liveness_mark = bench_alloc::counters();
    let allocation = Box::new([0_u8; 128]);
    std::hint::black_box(&allocation);
    drop(allocation);
    let liveness = bench_alloc::delta_since(liveness_mark);
    assert!(liveness.allocations > 0);
    assert!(liveness.deallocations > 0);

    let (mut control, mut render, _) = endpoint_with_capacity(2);
    let batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let ticket = control.try_publish(batch).expect("ticket");
    audit::warm_up();
    audit::reset();
    let report = audit::in_render_scope(|| render_block(&mut render, 0));
    assert!(report.applied.is_some());
    let first = audit::snapshot();
    assert_eq!(first.allocations, 0);
    assert_eq!(first.deallocations, 0);
    control.collect(ticket).expect("collect");
    audit::reset();
    let token = control.begin_cancel().expect("cancel");
    let cancel = audit::in_render_scope(|| render_block(&mut render, QUANTUM as u64));
    assert!(cancel.cancellation_only);
    let second = audit::snapshot();
    assert_eq!(second.allocations, 0);
    assert_eq!(second.deallocations, 0);
    control
        .poll_cancel_boundary(token)
        .expect("cancel poll")
        .expect("cancel complete");
}

#[test]
fn actual_endpoint_allocations_and_nonempty_cancellation_reuse_are_live() {
    use bench_support::alloc as bench_alloc;

    bench_alloc::assert_installed();
    for _ in 0..2 {
        let mark = bench_alloc::counters();
        let (control, render, _) = endpoint_with_capacity(2);
        let prepared = bench_alloc::delta_since(mark);
        assert!(prepared.allocations > 0, "endpoint preparation allocated");
        assert!(
            prepared.requested_bytes > 0,
            "endpoint retained allocation bytes"
        );
        drop(render);
        drop(control);
        let released = bench_alloc::delta_since(mark);
        assert!(
            released.deallocations > 0,
            "endpoint teardown released retained allocations"
        );
    }

    for _ in 0..2 {
        let (mut control, mut render, _) = endpoint_with_capacity(3);
        let healthy = BuiltinBatch::new(
            REVISION,
            SampleTime(0),
            &[BuiltinBatchRecord::Fader {
                track_index: 0,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Both,
                    muted: true,
                    smoothing_samples: 0,
                },
            }],
        )
        .unwrap();
        let healthy_ticket = control.try_publish(healthy).expect("healthy ticket");
        assert!(render_block(&mut render, 0).applied.is_some());
        control.collect(healthy_ticket).expect("healthy collect");

        let canceled_a = BuiltinBatch::new(
            REVISION,
            SampleTime(QUANTUM as u64),
            &[BuiltinBatchRecord::Fader {
                track_index: 1,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Left,
                    muted: false,
                    smoothing_samples: 0,
                },
            }],
        )
        .unwrap();
        let canceled_b = BuiltinBatch::new(
            REVISION,
            SampleTime((QUANTUM * 2) as u64),
            &[BuiltinBatchRecord::Matrix {
                track_index: 2,
                record: TrackControlRecord {
                    matrix: Matrix2x2 {
                        ll: 0.5,
                        lr: 0.25,
                        rl: -0.25,
                        rr: 0.75,
                    },
                    smoothing_samples: 8,
                },
            }],
        )
        .unwrap();
        let canceled_a_ticket = control.try_publish(canceled_a).expect("cancel ticket a");
        let canceled_b_ticket = control.try_publish(canceled_b).expect("cancel ticket b");
        let token = control.begin_cancel().expect("nonempty cancel");
        assert!(render_block(&mut render, QUANTUM as u64).cancellation_only);
        assert!(
            control
                .poll_cancel_boundary(token)
                .expect("cancel ack")
                .is_none()
        );
        assert_eq!(
            control
                .collect(canceled_a_ticket)
                .expect("canceled a collect")
                .disposition,
            CoreTerminalDisposition::Canceled
        );
        assert_eq!(
            control
                .collect(canceled_b_ticket)
                .expect("canceled b collect")
                .disposition,
            CoreTerminalDisposition::Canceled
        );
        assert!(
            control
                .poll_cancel_boundary(token)
                .expect("final cancel")
                .is_some()
        );

        let reused = BuiltinBatch::new(
            REVISION,
            SampleTime(QUANTUM as u64),
            &[BuiltinBatchRecord::Fader {
                track_index: 0,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Both,
                    muted: false,
                    smoothing_samples: 0,
                },
            }],
        )
        .unwrap();
        let reused_ticket = control.try_publish(reused).expect("reused generation");
        assert!(render_block(&mut render, QUANTUM as u64).applied.is_some());
        control.collect(reused_ticket).expect("reused collect");
    }
}

#[test]
fn batch_applies_one_fifo_ticket_at_a_late_boundary_and_reconciles() {
    let (mut control, mut render, resources) = endpoint();
    assert!(resources.composed_retained_heap_bytes > resources.delivery.retained_payload_bytes);
    assert_eq!(
        resources.endpoint_retained_heap_bytes,
        resources
            .delivery
            .retained_payload_bytes
            .checked_add(resources.outcome.retained_payload_bytes)
            .expect("endpoint resource sum")
    );
    assert_eq!(
        resources.inline_owner_bytes,
        u64::try_from(resources.control_inline_bytes)
            .expect("control inline bytes")
            .checked_add(
                u64::try_from(resources.started_render_inline_bytes)
                    .expect("started render inline bytes"),
            )
            .expect("inline owner sum")
    );
    assert!(resources.started_render_inline_bytes >= resources.prepared_render_inline_bytes);
    assert_eq!(
        resources.composed_retained_heap_bytes,
        resources
            .host_builtin_retained_payload_bytes
            .checked_add(resources.endpoint_retained_heap_bytes)
            .expect("composed resource sum")
    );
    assert!(
        resources.host.effect_bank_scratch_bytes > 0,
        "fixture retains an actual bank lane"
    );
    let batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: -3.0,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(render_block(&mut render, 0).applied.is_none());
    let ticket = control.try_publish(batch).expect("admitted batch");
    let applied = render_block(&mut render, QUANTUM as u64);
    assert!(applied.applied.is_some());
    let completion = control.collect(ticket).expect("collected batch");
    assert_eq!(completion.ticket, ticket);
    assert_eq!(completion.disposition, CoreTerminalDisposition::Applied);
    assert_eq!(completion.applied_prefix, 1);
    assert_eq!(completion.actual_sample, Some(SampleTime(QUANTUM as u64)));
    assert!(completion.late);
    assert_eq!(control.outstanding(), 0);
    assert_eq!(
        control.collect(ticket),
        Err(protocol::DeliveryError::StaleTicket)
    );
}

#[test]
fn outcome_is_staged_before_credit_release_and_fifo_late_pair_is_exact() {
    let (mut control, mut render, _) = endpoint_with_capacity(2);
    let first_batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: 8,
            },
        }],
    )
    .unwrap();
    let second_batch = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64),
        &[BuiltinBatchRecord::Matrix {
            track_index: 1,
            record: TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.7,
                    lr: 0.1,
                    rl: -0.2,
                    rr: 0.8,
                },
                smoothing_samples: 4,
            },
        }],
    )
    .unwrap();
    let first = control.try_publish(first_batch).expect("first ticket");
    let second = control.try_publish(second_batch).expect("second ticket");
    assert_eq!(control.outstanding(), 2);
    assert_eq!(control.collect(first), Err(protocol::DeliveryError::Empty));
    assert_eq!(control.outstanding(), 2);
    assert!(render_block(&mut render, 0).applied.is_some());
    assert!(render_block(&mut render, QUANTUM as u64).applied.is_some());
    assert_eq!(
        control.collect(second),
        Err(protocol::DeliveryError::StaleTicket)
    );
    assert_eq!(control.outstanding(), 2);
    let replacement = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64 * 2),
        &[BuiltinBatchRecord::Fader {
            track_index: 2,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Left,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(replacement),
        Err(BuiltinBatchAdmissionError::Delivery {
            error: protocol::DeliveryError::Full,
            ..
        })
    ));
    let first_completion = control.collect(first).expect("first terminal");
    assert_eq!(first_completion.actual_sample, Some(SampleTime(0)));
    let second_completion = control.collect(second).expect("second terminal");
    assert_eq!(
        second_completion.actual_sample,
        Some(SampleTime(QUANTUM as u64))
    );
    assert!(!second_completion.late);
}

#[test]
fn invalid_and_saturated_publication_are_atomic_and_resources_have_exact_caps() {
    let (mut control, mut render, resources) = endpoint_with_capacity(1);
    let valid = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: -6.0,
                smoothing_samples: 4,
            },
        }],
    )
    .unwrap();
    let invalid = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Matrix {
            track_index: 0,
            record: TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: f32::NAN,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(invalid),
        Err(BuiltinBatchAdmissionError::Invalid {
            reason: host_core::BuiltinBatchAdmissionReason::InvalidValue,
            ..
        })
    ));
    let wrong_revision = BuiltinBatch::new(
        SessionRevision(7),
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(wrong_revision),
        Err(BuiltinBatchAdmissionError::Invalid {
            reason: host_core::BuiltinBatchAdmissionReason::WrongRevision,
            ..
        })
    ));
    let misaligned = BuiltinBatch::new(
        REVISION,
        SampleTime(1),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(misaligned),
        Err(BuiltinBatchAdmissionError::Invalid {
            reason: host_core::BuiltinBatchAdmissionReason::MisalignedSample,
            ..
        })
    ));
    let overflow = BuiltinBatch::new(
        REVISION,
        SampleTime(u64::MAX - (QUANTUM as u64 - 1)),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(overflow),
        Err(BuiltinBatchAdmissionError::Invalid {
            reason: host_core::BuiltinBatchAdmissionReason::SampleOverflow,
            ..
        })
    ));
    assert_eq!(control.outstanding(), 0);
    let ticket = control.try_publish(valid).expect("valid ticket");
    let saturated = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Right,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(saturated),
        Err(BuiltinBatchAdmissionError::Delivery { batch, error: protocol::DeliveryError::Full })
            if batch == saturated
    ));
    assert_eq!(control.outstanding(), 1);
    assert!(render_block(&mut render, 0).applied.is_some());
    control.collect(ticket).expect("valid collection");

    let compiled = host_core::compile_host_session(SESSION, &caps()).expect("compile fixture");
    let mut exact = caps();
    exact.maximum_builtin_retained_bytes = resources.composed_retained_heap_bytes;
    exact.maximum_named_allocation_bytes = resources.largest_composed_heap_allocation_bytes;
    prepare_builtin_batch_endpoint(&compiled, &exact, REVISION, NonZeroUsize::new(1).unwrap())
        .expect("exact resource caps");
    exact.maximum_builtin_retained_bytes -= 1;
    assert!(matches!(
        prepare_builtin_batch_endpoint(&compiled, &exact, REVISION, NonZeroUsize::new(1).unwrap()),
        Err(host_core::BuiltinBatchPrepareError::ResourceLimit)
    ));
    let mut largest_below = caps();
    largest_below.maximum_builtin_retained_bytes = u64::MAX;
    largest_below.maximum_named_allocation_bytes = resources
        .largest_endpoint_heap_allocation_bytes
        .checked_sub(1)
        .expect("endpoint largest allocation is nonzero");
    assert!(matches!(
        prepare_builtin_batch_endpoint(
            &compiled,
            &largest_below,
            REVISION,
            NonZeroUsize::new(1).unwrap()
        ),
        Err(host_core::BuiltinBatchPrepareError::ResourceLimit)
    ));
}

#[test]
fn separate_control_and_render_threads_preserve_single_claim() {
    let compiled = host_core::compile_host_session(SESSION, &caps()).expect("compile fixture");
    let prepared =
        prepare_builtin_batch_endpoint(&compiled, &caps(), REVISION, NonZeroUsize::new(2).unwrap())
            .expect("prepared endpoint");
    std::thread::scope(|scope| {
        let (control_tx, control_rx) = sync_channel(1);
        let (step_tx, step_rx) = sync_channel(0);
        let (report_tx, report_rx) = sync_channel(1);
        scope.spawn(move || {
            let (control, mut render, _) = prepared
                .start()
                .unwrap_or_else(|_| panic!("started endpoint"));
            control_tx.send(control).expect("control handoff");
            for step in 0..2 {
                step_rx.recv().expect("render step");
                report_tx
                    .send(render_block(&mut render, step * QUANTUM as u64))
                    .expect("render report");
            }
        });
        let mut control = control_rx.recv().expect("control owner");
        let first = BuiltinBatch::new(
            REVISION,
            SampleTime(0),
            &[BuiltinBatchRecord::Fader {
                track_index: 0,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Both,
                    muted: true,
                    smoothing_samples: 0,
                },
            }],
        )
        .unwrap();
        let first_ticket = control.try_publish(first).expect("first publish");
        step_tx.send(()).expect("first step");
        let first_report = report_rx.recv().expect("first report");
        assert!(first_report.applied.is_some());
        assert_eq!(control.outstanding(), 1);
        control.collect(first_ticket).expect("first collect");

        // The second publication crosses the first singleton claim and cannot be applied in the
        // same render block. It is explicitly owned for the next block.
        let second = BuiltinBatch::new(
            REVISION,
            SampleTime(0),
            &[BuiltinBatchRecord::Fader {
                track_index: 1,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Both,
                    muted: false,
                    smoothing_samples: 0,
                },
            }],
        )
        .unwrap();
        let second_ticket = control
            .try_publish(second)
            .expect("second publish after claim");
        step_tx.send(()).expect("second step");
        let second_report = report_rx.recv().expect("second report");
        assert!(second_report.applied.is_some());
        control.collect(second_ticket).expect("second collect");
    });
}

#[test]
fn scoped_render_receiver_exits_when_control_sender_drops_on_panic() {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        std::thread::scope(|scope| {
            let (step_tx, step_rx) = sync_channel::<()>(0);
            scope.spawn(move || assert!(step_rx.recv().is_err(), "sender remained live"));
            drop(step_tx);
            panic!("synthetic control failure");
        });
    }));
    assert!(result.is_err());
}

#[test]
fn endpoint_drives_nonzero_pcm_through_the_prepared_bank_and_scalar_plan() {
    let compiled = host_core::compile_host_session(SESSION, &caps()).expect("compile fixture");
    let console = HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(host_core::BUILTIN_BATCH_MAX_RECORDS).unwrap()),
        ..HostConsoleRequest::default()
    };
    let (baseline_host, mut baseline_handles) =
        prepare_host_runtime_with_console(&compiled, &caps(), &console).expect("baseline host");
    let (mut baseline_render, mut baseline_sources, _) = baseline_host
        .start_render_session()
        .unwrap_or_else(|_| panic!("baseline start"));
    let (mut control, mut render, resources) =
        prepare_builtin_batch_endpoint(&compiled, &caps(), REVISION, NonZeroUsize::new(4).unwrap())
            .expect("endpoint")
            .start()
            .unwrap_or_else(|_| panic!("endpoint start"));
    assert!(resources.host.effect_bank_scratch_bytes > 0);
    let left = [0.25_f32; QUANTUM];
    let right = [-0.5_f32; QUANTUM];
    let submission = SourceSubmission {
        generation: 1,
        start_frame: 0,
        sample_rate_hz: 48_000,
        planes: &[&left, &right],
        frames: QUANTUM as u32,
        end_of_region: false,
    };
    control
        .sources()
        .submit(b"fixture-source", submission)
        .expect("source block");
    baseline_sources
        .submit(b"fixture-source", submission)
        .expect("baseline source block");
    let matrix = TrackControlRecord {
        matrix: Matrix2x2 {
            ll: 0.7,
            lr: 0.2,
            rl: -0.1,
            rr: 0.8,
        },
        smoothing_samples: 16,
    };
    baseline_handles.track_controls[0]
        .fader
        .try_push(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Left,
            db: -6.0,
            smoothing_samples: 16,
        })
        .expect("baseline fader");
    baseline_handles.track_controls[1]
        .producer
        .try_push(matrix)
        .expect("baseline matrix");
    let scalar_fader = TrackFaderRecord::FaderDb {
        lanes: BuiltinLaneSelector::Right,
        db: -3.0,
        smoothing_samples: 13,
    };
    let scalar_matrix = TrackControlRecord {
        matrix: Matrix2x2 {
            ll: 0.5,
            lr: 0.25,
            rl: -0.25,
            rr: 0.75,
        },
        smoothing_samples: 11,
    };
    baseline_handles.track_controls[8]
        .fader
        .try_push(scalar_fader)
        .expect("baseline scalar fader");
    baseline_handles.track_controls[8]
        .producer
        .try_push(scalar_matrix)
        .expect("baseline scalar matrix");
    let batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[
            BuiltinBatchRecord::Fader {
                track_index: 0,
                record: TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Left,
                    db: -6.0,
                    smoothing_samples: 16,
                },
            },
            BuiltinBatchRecord::Matrix {
                track_index: 1,
                record: matrix,
            },
            BuiltinBatchRecord::Fader {
                track_index: 8,
                record: scalar_fader,
            },
            BuiltinBatchRecord::Matrix {
                track_index: 8,
                record: scalar_matrix,
            },
        ],
    )
    .expect("batch");
    let ticket = control.try_publish(batch).expect("publish");
    let mut endpoint_samples = [0.0_f32; QUANTUM * 2];
    let mut baseline_samples = [0.0_f32; QUANTUM * 2];
    let baseline_report = baseline_render
        .render_planar(&mut baseline_samples, 2, QUANTUM, QUANTUM, 0)
        .expect("baseline render");
    let report = render
        .render(&mut endpoint_samples, 2, QUANTUM, QUANTUM, SampleTime(0))
        .expect("render");
    assert!(report.applied.is_some());
    assert_eq!(
        report.graph.map(|value| value.frames),
        Some(baseline_report.frames)
    );
    assert_eq!(endpoint_samples, baseline_samples);
    assert!(endpoint_samples.iter().any(|sample| sample.to_bits() != 0));
    assert_eq!(
        control.collect(ticket).expect("collect").disposition,
        CoreTerminalDisposition::Applied
    );
}

#[test]
fn future_batch_stays_pending_then_applies_once_and_invalid_batches_are_atomic() {
    let (mut control, mut render, _) = endpoint();
    let future = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64 * 2),
        &[BuiltinBatchRecord::Matrix {
            track_index: 1,
            record: TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.8,
                    lr: 0.2,
                    rl: -0.1,
                    rr: 0.9,
                },
                smoothing_samples: 16,
            },
        }],
    )
    .unwrap();
    let ticket = control.try_publish(future).expect("future batch");
    assert!(render_block(&mut render, 0).applied.is_none());
    assert!(render_block(&mut render, QUANTUM as u64).applied.is_none());
    let applied = render_block(&mut render, QUANTUM as u64 * 2);
    assert!(applied.applied.is_some());
    let completion = control.collect(ticket).expect("future completion");
    assert_eq!(
        completion.actual_sample,
        Some(SampleTime(QUANTUM as u64 * 2))
    );

    let wrong_track = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64 * 3),
        &[BuiltinBatchRecord::Fader {
            track_index: u32::MAX,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Left,
                muted: true,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(wrong_track),
        Err(BuiltinBatchAdmissionError::Invalid { batch, .. }) if batch == wrong_track
    ));
    assert_eq!(control.outstanding(), 0);
}

#[test]
fn cancellation_before_claim_is_render_only_and_releases_after_collection() {
    let (mut control, mut render, _) = endpoint();
    let batch = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64 * 4),
        &[BuiltinBatchRecord::Fader {
            track_index: 2,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Right,
                muted: true,
                smoothing_samples: 8,
            },
        }],
    )
    .unwrap();
    let ticket = control.try_publish(batch).expect("admitted batch");
    let token = control.begin_cancel().expect("cancel begun");
    let boundary = render_block(&mut render, 0);
    assert!(boundary.cancellation_only);
    assert_eq!(control.collect(ticket), Err(protocol::DeliveryError::Empty));
    assert_eq!(control.outstanding(), 1);
    assert!(
        control
            .poll_cancel_boundary(token)
            .expect("cancel poll")
            .is_none()
    );
    let completion = control.collect(ticket).expect("canceled collection");
    assert_eq!(completion.disposition, CoreTerminalDisposition::Canceled);
    assert_eq!(completion.applied_prefix, 0);
    assert_eq!(completion.actual_sample, None);
    assert_eq!(completion.acknowledged_sample, Some(SampleTime(0)));
    assert_eq!(control.outstanding(), 0);
    let blocked_batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 2,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    assert!(matches!(
        control.try_publish(blocked_batch),
        Err(BuiltinBatchAdmissionError::Delivery {
            error: protocol::DeliveryError::CancellationPending,
            ..
        })
    ));
    assert_eq!(
        control.begin_cancel(),
        Err(protocol::DeliveryError::CancellationPending)
    );
    let complete = control
        .poll_cancel_boundary(token)
        .expect("final cancel poll")
        .expect("cancel complete");
    assert_eq!(complete.acknowledged_sample, SampleTime(0));
    assert_eq!(
        control.poll_cancel_boundary(token),
        Err(protocol::DeliveryError::StaleTicket)
    );
    let next_batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 2,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let next_ticket = control
        .try_publish(next_batch)
        .expect("next generation ticket");
    assert_ne!(next_ticket.generation, ticket.generation);
    assert!(render_block(&mut render, 0).applied.is_some());
    assert_eq!(
        control
            .collect(next_ticket)
            .expect("next generation collect")
            .disposition,
        CoreTerminalDisposition::Applied
    );
    assert_eq!(
        control.poll_cancel_boundary(token),
        Err(protocol::DeliveryError::StaleTicket)
    );
}

#[test]
fn empty_and_precollected_cancellation_finalize_once_and_reopen_publication() {
    let (mut control, mut render, _) = endpoint();
    let empty_token = control.begin_cancel().expect("empty cancel");
    assert!(render_block(&mut render, 0).cancellation_only);
    assert!(
        control
            .poll_cancel_boundary(empty_token)
            .expect("empty completion")
            .is_some()
    );
    assert_eq!(
        control.poll_cancel_boundary(empty_token),
        Err(protocol::DeliveryError::StaleTicket)
    );
    let first = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let first_ticket = control.try_publish(first).expect("reopened publication");
    assert!(render_block(&mut render, 0).applied.is_some());
    control.collect(first_ticket).expect("first collect");

    let precollected_token = control.begin_cancel().expect("precollected cancel");
    assert!(render_block(&mut render, QUANTUM as u64).cancellation_only);
    assert!(
        control
            .poll_cancel_boundary(precollected_token)
            .expect("precollected completion")
            .is_some()
    );
    assert_eq!(
        control.poll_cancel_boundary(precollected_token),
        Err(protocol::DeliveryError::StaleTicket)
    );
    let second = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let second_ticket = control
        .try_publish(second)
        .expect("second reopened publication");
    assert!(render_block(&mut render, QUANTUM as u64).applied.is_some());
    control.collect(second_ticket).expect("second collect");
}

#[test]
fn cancellation_reconciles_applied_and_future_frontier_dispositions() {
    let (mut control, mut render, _) = endpoint_with_capacity(2);
    let applied_batch = BuiltinBatch::new(
        REVISION,
        SampleTime(0),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: true,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let future_batch = BuiltinBatch::new(
        REVISION,
        SampleTime((QUANTUM * 2) as u64),
        &[BuiltinBatchRecord::Fader {
            track_index: 1,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Left,
                muted: true,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let applied_ticket = control.try_publish(applied_batch).expect("applied ticket");
    let canceled_ticket = control.try_publish(future_batch).expect("future ticket");
    assert!(render_block(&mut render, 0).applied.is_some());
    let token = control.begin_cancel().expect("cancel begun");
    let boundary = render_block(&mut render, QUANTUM as u64);
    assert!(boundary.cancellation_only);
    assert!(
        control
            .poll_cancel_boundary(token)
            .expect("cancel poll")
            .is_none()
    );
    assert_eq!(
        control
            .collect(applied_ticket)
            .expect("applied collection")
            .disposition,
        CoreTerminalDisposition::Applied
    );
    let canceled = control
        .collect(canceled_ticket)
        .expect("canceled collection");
    assert_eq!(canceled.disposition, CoreTerminalDisposition::Canceled);
    assert_eq!(canceled.actual_sample, None);
    assert_eq!(
        canceled.acknowledged_sample,
        Some(SampleTime(QUANTUM as u64))
    );
    let complete = control
        .poll_cancel_boundary(token)
        .expect("final cancel poll")
        .expect("cancel complete");
    assert_eq!(complete.frontier, Some(canceled_ticket.serial));

    let next_applied = BuiltinBatch::new(
        REVISION,
        SampleTime(QUANTUM as u64),
        &[BuiltinBatchRecord::Fader {
            track_index: 0,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let next_canceled = BuiltinBatch::new(
        REVISION,
        SampleTime((QUANTUM * 2) as u64),
        &[BuiltinBatchRecord::Fader {
            track_index: 1,
            record: TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Left,
                muted: false,
                smoothing_samples: 0,
            },
        }],
    )
    .unwrap();
    let next_applied_ticket = control.try_publish(next_applied).expect("next applied");
    let next_canceled_ticket = control.try_publish(next_canceled).expect("next canceled");
    assert!(render_block(&mut render, QUANTUM as u64).applied.is_some());
    let next_token = control.begin_cancel().expect("next cancel");
    assert!(render_block(&mut render, (QUANTUM * 2) as u64).cancellation_only);
    assert!(
        control
            .poll_cancel_boundary(next_token)
            .expect("next cancel ack")
            .is_none()
    );
    assert_eq!(
        control.poll_cancel_boundary(token),
        Err(protocol::DeliveryError::StaleTicket)
    );
    control
        .collect(next_applied_ticket)
        .expect("next applied collect");
    control
        .collect(next_canceled_ticket)
        .expect("next canceled collect");
    assert!(
        control
            .poll_cancel_boundary(next_token)
            .expect("next final poll")
            .is_some()
    );
}
