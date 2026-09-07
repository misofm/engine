//! Focused product gates for the prepared typed builtin batch endpoint.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;

use builtins::{BuiltinLaneSelector, Matrix2x2};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord};
use host_core::{
    BuiltinBatch, BuiltinBatchAdmissionError, BuiltinBatchRecord, BuiltinBatchRenderReport,
    HostPrepareCaps, HostShapePolicy, prepare_builtin_batch_endpoint,
};
use protocol::{CoreTerminalDisposition, SampleTime, SessionRevision};

const REVISION: SessionRevision = SessionRevision(42);
const QUANTUM: usize = 128;
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

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
    let compiled = host_core::compile_host_session(SESSION, &caps()).expect("compile fixture");
    prepare_builtin_batch_endpoint(&compiled, &caps(), REVISION, NonZeroUsize::new(4).unwrap())
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

#[test]
fn batch_applies_one_fifo_ticket_at_a_late_boundary_and_reconciles() {
    let (mut control, mut render, resources) = endpoint();
    assert!(resources.retained_bytes > resources.delivery.retained_payload_bytes);
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
    let ticket = control.try_publish(batch).expect("admitted batch");
    assert!(render_block(&mut render, 0).applied.is_some());
    let completion = control.collect(ticket).expect("collected batch");
    assert_eq!(completion.ticket, ticket);
    assert_eq!(completion.disposition, CoreTerminalDisposition::Applied);
    assert_eq!(completion.applied_prefix, 1);
    assert_eq!(completion.actual_sample, Some(SampleTime(0)));
    assert!(!completion.late);
    assert_eq!(control.outstanding(), 0);
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
    let complete = control
        .poll_cancel_boundary(token)
        .expect("cancel poll")
        .expect("cancel complete");
    assert_eq!(complete.acknowledged_sample, SampleTime(0));
    let completion = control.collect(ticket).expect("canceled collection");
    assert_eq!(completion.disposition, CoreTerminalDisposition::Canceled);
    assert_eq!(completion.applied_prefix, 0);
    assert_eq!(completion.actual_sample, None);
    assert_eq!(completion.acknowledged_sample, Some(SampleTime(0)));
    assert_eq!(control.outstanding(), 0);
}
