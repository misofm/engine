//! Gates for the shared host preparation facade (audit #103 F1/F6).
//!
//! These are the tests the C ABI host and the browser host each used to carry a private copy of,
//! plus the two properties that only exist because the code is now shared: every resource cap is
//! checked against the row the facade itself reports, and every source-control rejection is typed.

use host_core::{
    HostPrepareCaps, HostPrepareReport, HostShapePolicy, LAUNCH_SAMPLE_RATES_HZ, PrepareRejection,
    SOURCE_STALL_TOLERANCE_MS, SourceControlError, SourceSubmission, compile_host_session,
    control_table_bytes, default_source_ring_frames, diagnostic_lines, prepare_host_runtime,
    prepare_host_session, source_id_arena_bytes,
};
use source::{HostChunkError, SourceSeekError};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
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
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
    }
}

fn report() -> HostPrepareReport {
    prepare_host_session(SESSION, &caps())
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
        .1
        .report
}

/// The ported C ABI ownership test: one preparation yields the session's shape, a live plan, and a
/// source set that accepts a chunk, a seek and the zero-frame end-of-region marker.
#[test]
fn prepare_reports_the_session_shape_and_feeds_sources_independently() {
    let (_compiled, mut prepared) =
        prepare_host_session(SESSION, &caps()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!(prepared.report.sample_rate_hz, 48_000);
    assert_eq!(prepared.report.quantum_frames, 128);
    assert_eq!(prepared.report.source_count, 1);
    assert_eq!(prepared.report.track_count, 9);
    assert!(prepared.report.graph_session_plus_plan_bytes > 0);
    assert!(prepared.report.source_total_bytes > 0);
    assert!(prepared.report.effect_scalar_state_bytes > 0);
    assert!(prepared.report.builtin_retained_payload_bytes > 0);
    assert!(prepared.report.largest_engine_allocation_bytes > 0);
    assert_eq!(prepared.sources.len(), 1);

    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    let submitted = prepared
        .sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("first source block");
    assert_eq!(submitted.accepted_frames, 128);
    prepared
        .sources
        .seek(b"fixture-source", 2, 48_000)
        .expect("inclusive end seek");
    prepared
        .sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 48_000,
                sample_rate_hz: 48_000,
                planes: &[&[], &[]],
                frames: 0,
                end_of_region: true,
            },
        )
        .expect("zero-frame final marker");
}

/// Every byte cap is checked against the exact row the facade reports: the reported value is
/// admitted and one byte below it is rejected as a resource failure. The oracle is the facade's own
/// report, so the test cannot drift when a compiler's byte totals change.
#[test]
fn every_byte_cap_admits_its_reported_row_and_rejects_one_byte_below() {
    let report = report();
    type Row = (&'static str, u64, fn(&mut HostPrepareCaps, u64));
    let rows: [Row; 7] = [
        (
            "graph_session_plus_plan",
            report
                .graph_session_plus_plan_bytes
                .checked_add(report.session_model_bytes)
                .expect("graph admission total"),
            |caps, value| caps.maximum_graph_session_plus_plan_bytes = value,
        ),
        ("source_total", report.source_total_bytes, |caps, value| {
            caps.maximum_source_total_bytes = value
        }),
        (
            "source_overhead",
            report.source_overhead_bytes,
            |caps, value| caps.maximum_source_overhead_bytes = value,
        ),
        (
            "effect_state",
            report.effect_scalar_state_bytes,
            |caps, value| caps.maximum_effect_state_bytes = value,
        ),
        (
            "effect_scratch",
            report.effect_scalar_scratch_bytes,
            |caps, value| caps.maximum_effect_scratch_bytes = value,
        ),
        (
            "builtin_retained",
            report.builtin_retained_payload_bytes,
            |caps, value| caps.maximum_builtin_retained_bytes = value,
        ),
        (
            "largest_named",
            report
                .largest_engine_allocation_bytes
                .max(report.session_largest_allocation_bytes),
            |caps, value| caps.maximum_named_allocation_bytes = value,
        ),
    ];
    let mut probed = 0_usize;
    for (name, reported, apply) in rows {
        if reported == 0 {
            // A row this fixture does not exercise cannot be probed one byte below zero.
            continue;
        }
        probed += 1;
        let mut equal = caps();
        apply(&mut equal, reported);
        assert!(
            prepare_host_session(SESSION, &equal).is_ok(),
            "{name}: the reported value must be admitted"
        );
        let mut below = caps();
        apply(&mut below, reported - 1);
        match prepare_host_session(SESSION, &below) {
            Ok(_) => panic!("{name}: one byte below the reported row was admitted"),
            Err(failure) => assert!(
                matches!(
                    failure.kind(),
                    PrepareRejection::Resource
                        | PrepareRejection::Session
                        | PrepareRejection::Builtin
                        | PrepareRejection::Effect
                        | PrepareRejection::Graph
                ),
                "{name}: unexpected rejection {:?} ({})",
                failure.kind(),
                String::from_utf8_lossy(failure.as_bytes())
            ),
        }
    }
    assert!(probed >= 6, "only {probed} of the 7 cap rows were probed");
}

/// Source rejections carry the source crate's own typed error, not a collapsed "invalid argument".
#[test]
fn source_control_errors_are_typed() {
    let (_compiled, mut prepared) =
        prepare_host_session(SESSION, &caps()).expect("prepared session");
    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    fn chunk<'a>(
        generation: u64,
        start_frame: u64,
        planes: &'a [&'a [f32]],
        frames: u32,
    ) -> SourceSubmission<'a> {
        SourceSubmission {
            generation,
            start_frame,
            sample_rate_hz: 48_000,
            planes,
            frames,
            end_of_region: false,
        }
    }

    assert_eq!(
        prepared
            .sources
            .submit(b"absent", chunk(1, 0, &[&left, &right], 128))
            .expect_err("unknown ID"),
        SourceControlError::UnknownSource
    );
    assert!(matches!(
        prepared
            .sources
            .submit(b"fixture-source", chunk(1, 0, &[&left, &right, &left], 128))
            .expect_err("three planes"),
        SourceControlError::Chunk(HostChunkError::ChannelCount { expected: 2, .. })
    ));
    assert_eq!(
        prepared
            .sources
            .submit(b"fixture-source", chunk(1, 47_999, &[&left, &right], 128))
            .expect_err("past the region end"),
        SourceControlError::OutsideRegion
    );
    assert_eq!(
        prepared
            .sources
            .submit(b"fixture-source", chunk(0, 0, &[&left, &right], 128))
            .expect_err("generation zero"),
        SourceControlError::GenerationZero
    );
    let planes: [&[f32]; 2] = [&left, &right];
    let ending = chunk(1, 47_872, &planes, 128);
    assert_eq!(
        prepared
            .sources
            .submit(b"fixture-source", ending)
            .expect_err("unflagged final chunk"),
        SourceControlError::EndOfRegionMismatch
    );

    assert_eq!(
        prepared
            .sources
            .seek(b"fixture-source", 0, 0)
            .expect_err("seek generation zero"),
        SourceControlError::GenerationZero
    );
    assert_eq!(
        prepared
            .sources
            .seek(b"fixture-source", 4, 48_001)
            .expect_err("seek past the region"),
        SourceControlError::OutsideRegion
    );
    // The ring opens at generation 1, so re-seeking to it is not strictly increasing.
    assert!(matches!(
        prepared
            .sources
            .seek(b"fixture-source", 1, 256)
            .expect_err("non-increasing seek generation"),
        SourceControlError::Seek(SourceSeekError::GenerationNotStrictlyIncreasing { .. })
    ));
    // The single accepted seek fills the depth-1 command queue, so it goes last.
    prepared
        .sources
        .seek(b"fixture-source", 2, 0)
        .expect("advance the active generation");
    assert!(matches!(
        prepared
            .sources
            .submit(b"fixture-source", chunk(1, 0, &[&left, &right], 128))
            .expect_err("stale generation"),
        SourceControlError::Chunk(HostChunkError::StaleGeneration { .. })
    ));
    assert!(matches!(
        prepared
            .sources
            .seek(b"fixture-source", 3, 256)
            .expect_err("the depth-1 seek queue is full"),
        SourceControlError::Seek(SourceSeekError::Backpressure { .. })
    ));

    // Every variant names a distinct rule, and only backpressure and internal failures are
    // classified as anything other than a caller mistake.
    assert_eq!(
        SourceControlError::OutsideRegion.diagnostic(),
        "source.region.outside"
    );
    assert!(SourceControlError::Chunk(HostChunkError::Full { full_count: 1 }).is_backpressure());
    assert!(SourceControlError::Chunk(HostChunkError::InternalInvariant).is_internal());
    assert!(!SourceControlError::OutsideRegion.is_backpressure());
    assert!(!SourceControlError::OutsideRegion.is_internal());
}

/// A host's pre-flight resource projection reads the facade's layout mirror, never a copy of the
/// facade's private struct. The mirror and the live set must agree exactly.
#[test]
fn retained_bytes_projection_matches_the_live_set() {
    let (_compiled, prepared) = prepare_host_session(SESSION, &caps()).expect("prepared session");
    let projected = control_table_bytes(prepared.sources.len())
        .and_then(|table| table.checked_add(source_id_arena_bytes(prepared.sources.id_bytes())?))
        .expect("projected control bytes");
    assert_eq!(
        prepared.sources.retained_bytes(),
        Some(projected),
        "the live set and the layout mirror must agree"
    );
    assert_eq!(prepared.report.control_retained_bytes, projected);
    assert_eq!(prepared.sources.longest_id_bytes(), "fixture-source".len());
    assert_eq!(
        prepared.report.source_id_bytes,
        "fixture-source".len() as u64
    );
}

/// `Exact` pins both the rate and the quantum; `AnyLaunchRate` accepts the launch set and the ring
/// rule applies to both policies.
#[test]
fn shape_policy_pins_rate_and_quantum() {
    let exact = HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz: 48_000,
            quantum_frames: 128,
        },
        ..caps()
    };
    prepare_host_session(SESSION, &exact).expect("the exact shape is accepted");

    let wrong_rate = HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz: 44_100,
            quantum_frames: 128,
        },
        ..caps()
    };
    let failure = prepare_host_session(SESSION, &wrong_rate)
        .map(|_| ())
        .expect_err("wrong rate");
    assert_eq!(failure.kind(), PrepareRejection::Shape);
    assert_eq!(failure.as_bytes(), b"host.session.shape\t$\n");

    let wrong_quantum = HostPrepareCaps {
        shape: HostShapePolicy::Exact {
            sample_rate_hz: 48_000,
            quantum_frames: 64,
        },
        ..caps()
    };
    assert_eq!(
        prepare_host_session(SESSION, &wrong_quantum)
            .map(|_| ())
            .expect_err("wrong quantum")
            .as_bytes(),
        b"host.session.shape\t$\n"
    );

    let bad_ring = HostPrepareCaps {
        source_ring_frames: 1_100,
        ..caps()
    };
    assert_eq!(
        prepare_host_session(SESSION, &bad_ring)
            .map(|_| ())
            .expect_err("ring is not a quantum multiple")
            .as_bytes(),
        b"host.source.ring_frames\t$\n"
    );

    let compiled = compile_host_session(SESSION, &caps()).expect("compiled");
    assert!(caps().validate_shape(&compiled).is_ok());
    assert!(wrong_rate.validate_shape(&compiled).is_err());
    assert!(prepare_host_runtime(&compiled, &caps()).is_ok());
}

/// The session validator owns the launch-rate set, so boot can rely on the field-local session
/// diagnostic before applying a host shape policy.
#[test]
fn session_validation_owns_the_launch_rate_set() {
    const RATE: &str = "\"sample_rate_hz\": 48000";
    let at = |rate: u32| SESSION.replace(RATE, &format!("\"sample_rate_hz\": {rate}"));
    for rate in LAUNCH_SAMPLE_RATES_HZ {
        assert!(compile_host_session(&at(rate), &caps()).is_ok(), "{rate}");
    }
    for rate in LAUNCH_SAMPLE_RATES_HZ
        .into_iter()
        .flat_map(|rate| [rate - 1, rate + 1])
        .chain([1, 8_000, 22_050, 32_000, 176_400, 192_000])
    {
        let failure = compile_host_session(&at(rate), &caps()).expect_err("unsupported rate");
        assert_eq!(failure.kind(), PrepareRejection::Session, "{rate}");
        assert_eq!(
            failure.as_bytes(),
            b"sample_rate.unsupported_at_launch\t$.sample_rate_hz\n",
            "{rate}"
        );
    }
}

/// The shared derivation is pinned across every launch tier and includes the adversarial 127-frame
/// quantum from the boot brief.
#[test]
fn default_ring_derivation_covers_the_stall_and_two_in_flight_quanta() {
    for rate in LAUNCH_SAMPLE_RATES_HZ {
        for quantum in [64_u32, 127, 128, 4_096] {
            let frames = default_source_ring_frames(rate, quantum);
            assert_ne!(frames, 0);
            assert_eq!(frames % quantum, 0);
            let stall = u64::from(rate) * u64::from(SOURCE_STALL_TOLERANCE_MS) / 1_000;
            assert!(u64::from(frames) >= stall + 2 * u64::from(quantum));
        }
    }
    assert_eq!(default_source_ring_frames(96_000, 127), 9_906);
}

/// A per-source channel cap is a host policy, not a session rule: the C ABI host does not set one
/// and the browser host does.
#[test]
fn source_channel_cap_is_optional() {
    let capped = HostPrepareCaps {
        maximum_source_channels: Some(1),
        ..caps()
    };
    let failure = prepare_host_session(SESSION, &capped)
        .map(|_| ())
        .expect_err("stereo source over the cap");
    assert_eq!(failure.kind(), PrepareRejection::Shape);
    assert_eq!(failure.as_bytes(), b"host.source.channels\t$\n");

    let permitted = HostPrepareCaps {
        maximum_source_channels: Some(2),
        ..caps()
    };
    prepare_host_session(SESSION, &permitted).expect("stereo source within the cap");
}

/// The `Send + !Sync` half of the crate-level `# Host callback contract (V1)`.
///
/// A host that could share these across threads could render from two threads at once, which the
/// contract forbids and which no amount of documentation prevents. `Send` is asserted here; the
/// absence of `Sync` is asserted by the `compile_fail` doctest below, because a negative bound
/// cannot be written as a runtime assertion.
///
/// Red mutation: hand-write a `Sync` implementation for `PreparedHost` -> `PreparedHost`'s
/// `compile_fail` doctest in `src/prepare.rs` starts compiling, and the doctest fails.
#[test]
fn contract_types_are_send_and_not_sync() {
    fn assert_send<T: Send>() {}
    assert_send::<host_core::PreparedHost>();
    assert_send::<host_core::SourceControlSet>();
    assert_send::<engine::realtime::PreparedRenderPlan>();
}

/// Issue #137 D1/D2: the live-console halves are prepared inside the plan transaction, addressed
/// by the canonical normalized track order, and bounded exactly as requested.
///
/// Red mutation: drop the `bound.track_controls.len() != control_requests.len()` leg of the
/// console arity check in `prepare_host_runtime_with_console` and make
/// `prepare_session_builtins_with_console` skip one requested channel -> this test still finds
/// nine tracks but only eight producers, so the `zip` below panics on the missing channel.
#[test]
fn console_attaches_bounded_control_and_meter_halves_in_canonical_track_order() {
    use core::num::{NonZeroU32, NonZeroUsize};
    use host_core::{HostConsoleRequest, prepare_host_session_with_console};

    let mut console_caps = caps();
    console_caps.maximum_meter_streams = 16;
    console_caps.maximum_meter_items = 1 << 16;
    console_caps.maximum_meter_bytes = 1 << 24;
    let console = HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(4).expect("nonzero")),
        meter_period_frames: Some(NonZeroU32::new(128).expect("nonzero")),
        meter_queue_depth: NonZeroUsize::new(8).expect("nonzero"),
        ..HostConsoleRequest::default()
    };
    let (compiled, prepared, mut handles) =
        prepare_host_session_with_console(SESSION, &console_caps, &console).unwrap_or_else(
            |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
        );
    let expected: Vec<String> = compiled
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str().to_owned())
        .collect();
    assert_eq!(
        handles
            .tracks
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        expected,
        "console track order is the canonical normalized order"
    );
    assert_eq!(handles.track_controls.len(), expected.len());
    assert_eq!(handles.meters.len(), expected.len());
    for (index, track) in expected.iter().enumerate() {
        assert_eq!(&*handles.track_controls[index].track_id, track.as_str());
        assert_eq!(&*handles.meters[index].track_id, track.as_str());
    }
    assert!(prepared.report.builtin_meter_payload_bytes > 0);

    // The control queue is bounded and a full queue hands the record back rather than dropping it.
    let record = builtins_compiler::TrackControlRecord {
        matrix: builtins::Matrix2x2 {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        smoothing_samples: 0,
    };
    let producer = &mut handles.track_controls[0].producer;
    for _ in 0..4 {
        producer
            .try_push(record)
            .expect("bounded depth admits four");
    }
    let full = producer
        .try_push(record)
        .expect_err("fifth is backpressure");
    assert_eq!(full.value, record, "a full queue returns the record");
}

/// Issue #137: a host that asks for no console gets none, and pays for none.
#[test]
fn no_console_request_attaches_nothing_and_charges_nothing() {
    use host_core::{
        HostConsoleRequest, compile_host_session as compile, prepare_host_runtime_with_console,
    };

    let compiled = compile(SESSION, &caps()).expect("compiled fixture");
    let (plain, handles) =
        prepare_host_runtime_with_console(&compiled, &caps(), &HostConsoleRequest::default())
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    assert!(handles.track_controls.is_empty());
    assert!(handles.meters.is_empty());
    assert_eq!(handles.tracks.len(), 9);
    assert_eq!(plain.report.builtin_meter_payload_bytes, 0);
    let baseline = prepare_host_runtime(&compiled, &caps())
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
        .report;
    assert_eq!(
        plain.report.builtin_processor_payload_bytes, baseline.builtin_processor_payload_bytes,
        "an unattached console changes no processor byte"
    );
    assert_eq!(plain.report, baseline);
}

/// Unit-level pin for the shared encoder's independent 64-line defense.
///
/// This is deliberately not #240's full refusal-time eval: the exact-1-MiB production-boot fixture
/// in `host-web` owns parser, validation, span-materialization and wall-time coverage.
#[test]
fn dense_refusal_diagnostics_are_count_bounded_and_finish_under_one_second() {
    use std::time::{Duration, Instant};

    const INVALID_ROWS: usize = 16_384;
    let started = Instant::now();
    let bytes = diagnostic_lines(
        (0..INVALID_ROWS).map(|index| ("automation.invalid", format!("$.automation[{index}]"))),
    );
    let elapsed = started.elapsed();
    assert_eq!(
        bytes.iter().filter(|byte| **byte == b'\n').count(),
        host_core::diagnostics::MAXIMUM_PREPARE_DIAGNOSTIC_LINES
    );
    assert!(
        elapsed < Duration::from_secs(1),
        "bounded dense refusal took {elapsed:?}"
    );
}
