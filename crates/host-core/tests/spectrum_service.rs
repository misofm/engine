//! The one-shot spectrum service (issue #1591): request rules, arm/read/cancel/select sequences,
//! the arm-ownership ruling (D7), the analysis half and the allocation-free control calls.

use bench_support::alloc as bench_alloc;
use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy, LiveValuesSuperseded,
    PreparedHost, PreparedSpectrumCaptures, SPECTRUM_BIN_COUNT, SPECTRUM_MAXIMUM_CAPTURE_BYTES,
    SPECTRUM_MAXIMUM_ID_BYTES, SPECTRUM_TARGET_OUTPUT, SPECTRUM_WINDOW_FRAMES, SourceControlSet,
    SourceSubmission, SpectrumAnalysis, SpectrumAnalysisResult, SpectrumAnalyzer,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumConfiguration, SpectrumHop, SpectrumOutput, SpectrumPreparation,
    SpectrumRequestError, SpectrumService, SpectrumServiceError, SpectrumSnapshot,
    SpectrumStreamMetadata, SpectrumStreamStatus, SpectrumTarget, SpectrumTargetRef,
    StartedRenderSession, compile_host_session, prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_spectrum, prepare_host_runtime_with_spectrum_collection,
    spectrum_capture_collection_resources, spectrum_capture_resources_for,
    spectrum_channels_from_raw, spectrum_smoothing, spectrum_target_kind, spectrum_target_ref,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const BLOCKS_PER_WINDOW: usize = SPECTRUM_WINDOW_FRAMES / QUANTUM;
const SAMPLE_RATE_HZ: u32 = 48_000;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
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

/// A started session that is fed a 997 Hz sine, one quantum at a time.
struct Rig {
    render: StartedRenderSession,
    sources: SourceControlSet,
    frame: u64,
}

impl Rig {
    fn new(host: PreparedHost) -> Self {
        let (render, sources, _) = host.start_render_session().expect("start render");
        Self {
            render,
            sources,
            frame: 0,
        }
    }

    fn block(&mut self) {
        let mut left = [0.0_f32; QUANTUM];
        let mut right = [0.0_f32; QUANTUM];
        for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let phase = 2.0 * core::f64::consts::PI * 997.0 * (self.frame + index as u64) as f64
                / f64::from(SAMPLE_RATE_HZ);
            let sample = math::sin(phase);
            *l = (0.25 * sample) as f32;
            *r = (0.125 * sample) as f32;
        }
        self.sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: self.frame,
                    sample_rate_hz: SAMPLE_RATE_HZ,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        self.render
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, self.frame)
            .expect("render block");
        self.frame += QUANTUM as u64;
    }

    fn window(&mut self) {
        for _ in 0..BLOCKS_PER_WINDOW {
            self.block();
        }
    }
}

fn single_service(channels: SpectrumChannels) -> (Rig, SpectrumService) {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let target = SpectrumTarget::Output("main-out".into());
    let (host, capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
            target,
            channels,
        },
    )
    .expect("single prepares");
    (
        Rig::new(host),
        SpectrumService::new(
            PreparedSpectrumCaptures::from(capture),
            SAMPLE_RATE_HZ,
            QUANTUM as u32,
            None,
        ),
    )
}

const TRACK: SpectrumTargetRef<'static> = SpectrumTargetRef::TrackPostInputBuiltins("eq0");
const OUTPUT: SpectrumTargetRef<'static> = SpectrumTargetRef::Output("main-out");

fn collection_service() -> (Rig, SpectrumService) {
    collection_service_with_hop(None)
}

fn collection_service_with_hop(hop: Option<SpectrumHop>) -> (Rig, SpectrumService) {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let entries = vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostInputBuiltins("eq0".into()),
            channels: SpectrumChannels::Left,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
        },
    ];
    let resources = spectrum_capture_collection_resources(&entries).expect("resources");
    let (host, capture) = prepare_host_runtime_with_spectrum_collection(
        &compiled,
        &caps(),
        &SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("collection prepares");
    (
        Rig::new(host),
        SpectrumService::new(
            PreparedSpectrumCaptures::from(capture),
            SAMPLE_RATE_HZ,
            QUANTUM as u32,
            hop,
        ),
    )
}

fn read(service: &mut SpectrumService, channels: SpectrumChannels) -> SpectrumSnapshot {
    service
        .read(channels)
        .expect("read")
        .expect("window complete")
}

/// Gate 1. Turns red if a rule moved to the core accepts or refuses differently from the browser:
/// the checks run identity, kind, mask, budget, and the first failure is the answer.
#[test]
fn request_rules_refuse_in_the_browsers_order() {
    let ids = [
        ("", Some(SpectrumRequestError::TargetId)),
        ("a", None),
        (&"i".repeat(127), None),
        (&"i".repeat(128), Some(SpectrumRequestError::TargetId)),
    ];
    let kinds = [
        (0, Some(SpectrumRequestError::TargetKind)),
        (1, None),
        (2, None),
        (3, None),
        (4, Some(SpectrumRequestError::TargetKind)),
    ];
    let masks = [
        (0, Some(SpectrumRequestError::Channels)),
        (1, None),
        (2, None),
        (3, None),
        (4, Some(SpectrumRequestError::Channels)),
    ];
    let budgets = [
        (0, Some(SpectrumRequestError::Budget)),
        (1, None),
        (SPECTRUM_MAXIMUM_CAPTURE_BYTES, None),
        (
            SPECTRUM_MAXIMUM_CAPTURE_BYTES + 1,
            Some(SpectrumRequestError::Budget),
        ),
    ];
    for (id, id_error) in &ids {
        for (kind, kind_error) in kinds {
            for (mask, mask_error) in masks {
                for (budget, budget_error) in budgets {
                    let expected = id_error.or(kind_error).or(mask_error).or(budget_error);
                    let got = SpectrumPreparation::single(kind, id, mask, budget);
                    match (expected, got) {
                        (None, Ok(SpectrumPreparation::Single(request))) => {
                            assert_eq!(request.maximum_capture_bytes, budget);
                            assert_eq!(request.channels as u32, mask);
                            assert_eq!(spectrum_target_kind(&request.target), kind);
                        }
                        (Some(error), Err(got)) => assert_eq!(got, error, "{kind} {id:?} {mask}"),
                        (expected, got) => panic!(
                            "kind {kind} id {id:?} mask {mask} budget {budget}: \
                             expected {expected:?}, got {got:?}"
                        ),
                    }
                }
            }
        }
    }
    assert_eq!(SPECTRUM_MAXIMUM_ID_BYTES, 127);
    assert_eq!(
        spectrum_target_ref(SPECTRUM_TARGET_OUTPUT, "x"),
        Ok(SpectrumTargetRef::Output("x"))
    );
    assert_eq!(spectrum_channels_from_raw(3), Ok(SpectrumChannels::Stereo));
    for (error, code) in [
        (SpectrumRequestError::TargetKind, "spectrum.target.kind"),
        (SpectrumRequestError::TargetId, "spectrum.target.id"),
        (SpectrumRequestError::Channels, "spectrum.channels"),
        (SpectrumRequestError::Budget, "spectrum.budget"),
    ] {
        assert_eq!(error.code(), code);
    }
}

/// Gate 1, collection half. Turns red if a collection budget is capped like a single one, if an
/// empty collection stops being the one way to prepare nothing, or if an entry is checked before
/// the budget.
#[test]
fn collection_rules_take_an_aggregate_budget() {
    let entries = [(1, "eq0", 1), (3, "main-out", 3)];
    assert!(matches!(
        SpectrumPreparation::collection(&entries, 2 << 20),
        Ok(Some(SpectrumPreparation::Collection(request)))
            if request.entries.len() == 2 && request.maximum_capture_bytes == 2 << 20
    ));
    assert!(matches!(
        SpectrumPreparation::collection(&entries, 0),
        Err(SpectrumRequestError::Budget)
    ));
    assert!(matches!(SpectrumPreparation::collection(&[], 0), Ok(None)));
    assert!(matches!(
        SpectrumPreparation::collection(&[], 1),
        Err(SpectrumRequestError::Budget)
    ));
    assert!(matches!(
        SpectrumPreparation::collection(&[(1, "eq0", 1), (9, "x", 1)], 1),
        Err(SpectrumRequestError::TargetKind)
    ));
    assert!(matches!(
        SpectrumPreparation::collection(&[(9, "x", 1)], 0),
        Err(SpectrumRequestError::Budget)
    ));
    assert!(matches!(
        SpectrumPreparation::collection(&[(1, "", 1)], 1),
        Err(SpectrumRequestError::TargetId)
    ));
    // A duplicate is a preparation refusal, not a request refusal.
    assert!(matches!(
        SpectrumPreparation::collection(&[(1, "eq0", 1), (1, "eq0", 1)], 1),
        Ok(Some(SpectrumPreparation::Collection(_)))
    ));
}

/// Gate 2, single form. Turns red if a read consumes a window it refuses, if the token skips or
/// repeats, or if a single preparation accepts a select.
#[test]
fn single_one_shot_sequence() {
    let (mut rig, mut service) = single_service(SpectrumChannels::Right);
    assert_eq!(service.target_kind(), SPECTRUM_TARGET_OUTPUT);
    assert_eq!(service.channels(), Some(SpectrumChannels::Right));
    assert_eq!(
        service.read(SpectrumChannels::Right).unwrap_err(),
        SpectrumServiceError::NotArmed
    );
    assert_eq!(
        service.select(OUTPUT, SpectrumChannels::Right).unwrap_err(),
        SpectrumServiceError::SelectSingle
    );
    assert_eq!(
        service
            .selection_would_change(OUTPUT, SpectrumChannels::Right)
            .unwrap_err(),
        SpectrumServiceError::SelectSingle
    );
    assert_eq!(service.selection_epoch(), 0);

    service.arm().expect("arm");
    assert_eq!(service.arm().unwrap_err(), SpectrumServiceError::Busy);
    assert!(service.read(SpectrumChannels::Right).unwrap().is_none());
    rig.block();
    assert!(service.read(SpectrumChannels::Right).unwrap().is_none());
    for _ in 1..BLOCKS_PER_WINDOW {
        rig.block();
    }
    assert_eq!(
        service.read(SpectrumChannels::Left).unwrap_err(),
        SpectrumServiceError::Channels
    );
    let first = read(&mut service, SpectrumChannels::Right);
    assert_eq!(first.token, 1);
    assert_eq!(first.window.first_sample, 0);
    assert_eq!(first.window.channels, SpectrumChannels::Right);
    assert_eq!(first.target_kind, SPECTRUM_TARGET_OUTPUT);
    assert_eq!(first.sample_rate_hz, SAMPLE_RATE_HZ);
    assert!(first.window.right.iter().any(|value| *value != 0.0));
    first.validate().expect("a read snapshot validates");

    for round in 2..=3_u64 {
        service.arm().expect("arm");
        rig.window();
        let snapshot = read(&mut service, SpectrumChannels::Right);
        assert_eq!(snapshot.token, round);
        assert_eq!(
            snapshot.window.first_sample,
            (round - 1) * SPECTRUM_WINDOW_FRAMES as u64
        );
    }

    service.arm().expect("arm");
    service.cancel().unwrap();
    assert_eq!(
        service.read(SpectrumChannels::Right).unwrap_err(),
        SpectrumServiceError::NotArmed
    );
    service.arm().expect("a cancelled arm can be made again");
}

/// Gate 2, collection form. Turns red if an unknown entry is accepted or a read before any
/// selection pops.
#[test]
fn collection_one_shot_sequence() {
    let (mut rig, mut service) = collection_service();
    assert_eq!(service.target(), None);
    assert_eq!(service.target_kind(), 0);
    assert_eq!(service.channels(), None);
    assert_eq!(
        service.read(SpectrumChannels::Left).unwrap_err(),
        SpectrumServiceError::NotArmed
    );
    assert_eq!(service.arm().unwrap_err(), SpectrumServiceError::Busy);
    assert_eq!(
        service
            .select(
                SpectrumTargetRef::Output("missing"),
                SpectrumChannels::Stereo
            )
            .unwrap_err(),
        SpectrumServiceError::UnknownEntry
    );
    assert_eq!(
        service.select(OUTPUT, SpectrumChannels::Left).unwrap_err(),
        SpectrumServiceError::UnknownEntry,
        "the mask is part of the entry"
    );
    assert_eq!(service.selection_epoch(), 0);

    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(false));
    assert_eq!(service.selection_epoch(), 1);
    assert_eq!(service.channels(), Some(SpectrumChannels::Left));
    assert_eq!(
        service.selection_would_change(OUTPUT, SpectrumChannels::Stereo),
        Ok(true)
    );
    rig.window();
    let snapshot = read(&mut service, SpectrumChannels::Left);
    assert_eq!(snapshot.token, 1);
    assert_eq!(
        snapshot.target_kind,
        host_core::SPECTRUM_TARGET_TRACK_POST_INPUT
    );
}

/// Gate 3. Turns red if a select discards a caller's armed window without a refusal, or if the
/// select-made arm is no longer replaceable.
#[test]
fn a_callers_arm_is_never_discarded_by_select() {
    let (mut rig, mut service) = collection_service();
    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
    assert_eq!(
        service.select(OUTPUT, SpectrumChannels::Stereo),
        Ok(true),
        "a select-made arm is replaced"
    );
    assert_eq!(service.selection_epoch(), 2);
    assert_eq!(
        service.arm().unwrap_err(),
        SpectrumServiceError::Busy,
        "already armed by select"
    );
    rig.window();
    let first = read(&mut service, SpectrumChannels::Stereo);
    assert_eq!(first.token, 1);
    assert_eq!(first.target_kind, SPECTRUM_TARGET_OUTPUT);
    assert_eq!(first.window.first_sample, 0);

    service.arm().expect("the caller arms after the read");
    assert_eq!(
        service.select(TRACK, SpectrumChannels::Left).unwrap_err(),
        SpectrumServiceError::Busy
    );
    assert_eq!(
        service
            .selection_would_change(TRACK, SpectrumChannels::Left)
            .unwrap_err(),
        SpectrumServiceError::Busy
    );
    assert_eq!(
        service.selection_epoch(),
        2,
        "a refused select changes nothing"
    );
    assert_eq!(
        service.select(OUTPUT, SpectrumChannels::Stereo),
        Ok(false),
        "the selected entry discards nothing"
    );
    rig.window();
    let second = read(&mut service, SpectrumChannels::Stereo);
    assert_eq!(second.token, 2);
    assert_eq!(second.target_kind, SPECTRUM_TARGET_OUTPUT);
    assert_eq!(second.window.first_sample, SPECTRUM_WINDOW_FRAMES as u64);

    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
    assert_eq!(service.selection_epoch(), 3);

    // A cancel ends the caller's claim.
    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    service.cancel().unwrap();
    service.arm().unwrap();
    service.cancel().unwrap();
    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
}

fn window_snapshot(channels: SpectrumChannels) -> (SpectrumSnapshot, SpectrumService, Rig) {
    let (mut rig, mut service) = single_service(channels);
    service.arm().unwrap();
    rig.window();
    let snapshot = read(&mut service, channels);
    (snapshot, service, rig)
}

/// Gate 4. Turns red if the analysis half changes a bin or writes output for a refused snapshot.
#[test]
fn analysis_matches_the_analyzer_and_refuses_without_writing() {
    let (snapshot, _service, _rig) = window_snapshot(SpectrumChannels::Stereo);
    let analysis = SpectrumAnalysis::new();
    let mut out = SpectrumAnalysisResult::new();
    analysis
        .analyze_into(&snapshot, &mut out)
        .expect("analyzes");

    let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
    let mut left = [0.0; SPECTRUM_BIN_COUNT];
    let mut right = [0.0; SPECTRUM_BIN_COUNT];
    SpectrumAnalyzer::new()
        .analyze(
            &snapshot.window,
            snapshot.sample_rate_hz,
            &mut SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            },
        )
        .unwrap();
    let bits = |values: &[f32]| {
        values
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    };
    assert_eq!(bits(&out.frequencies_hz), bits(&frequencies));
    assert_eq!(bits(&out.left_dbfs), bits(&left));
    assert_eq!(bits(&out.right_dbfs), bits(&right));
    assert_eq!(out.token, snapshot.token);
    assert_eq!(out.channels, SpectrumChannels::Stereo);
    assert_eq!(out.target_kind, snapshot.target_kind);
    assert_eq!(out.sample_rate_hz, SAMPLE_RATE_HZ);
    assert_eq!(out.captured_sample, 0);
    assert_eq!(out.end_sample, SPECTRUM_WINDOW_FRAMES as u64);
    assert!(!out.source_underrun);
    // The fixture's output carries the sine in one plane; that plane peaks at 997 Hz.
    let peak = |plane: &[f32]| {
        plane
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map(|(bin, level)| (bin, *level))
            .unwrap()
    };
    let (left_bin, left_level) = peak(&out.left_dbfs);
    let (right_bin, right_level) = peak(&out.right_dbfs);
    let (bin, level) = if left_level > right_level {
        (left_bin, left_level)
    } else {
        (right_bin, right_level)
    };
    assert!(level > -60.0, "the sine reaches the output: {level} dBFS");
    assert!(
        (42..=43).contains(&bin),
        "997 Hz at 48 kHz sits in bin 42 or 43, got {bin}"
    );

    // A one-plane window analyzes as that plane (the other plane is unspecified, #1595).
    let (left_only, _service, _rig) = window_snapshot(SpectrumChannels::Left);
    let mut left_out = SpectrumAnalysisResult::new();
    analysis.analyze_into(&left_only, &mut left_out).unwrap();
    assert_eq!(left_out.channels, SpectrumChannels::Left);

    let refusals: [(&str, SpectrumSnapshot); 5] = [
        (
            "token 0",
            SpectrumSnapshot {
                token: 0,
                ..snapshot
            },
        ),
        (
            "kind 4",
            SpectrumSnapshot {
                target_kind: 4,
                ..snapshot
            },
        ),
        (
            "kind 0",
            SpectrumSnapshot {
                target_kind: 0,
                ..snapshot
            },
        ),
        (
            "32 kHz",
            SpectrumSnapshot {
                sample_rate_hz: 32_000,
                ..snapshot
            },
        ),
        ("end sample", {
            let mut late = snapshot;
            late.window.first_sample = u64::MAX - 10;
            late
        }),
    ];
    for (name, refused) in refusals {
        let before = out.clone();
        assert_eq!(
            analysis.analyze_into(&refused, &mut out),
            Err(SpectrumServiceError::WindowInvalid),
            "{name}"
        );
        assert_eq!(out, before, "{name}: a refused snapshot wrote output");
    }
    let mut nonfinite = snapshot;
    nonfinite.window.left[5] = f32::NAN;
    let before = out.clone();
    assert_eq!(
        analysis.analyze_into(&nonfinite, &mut out),
        Err(SpectrumServiceError::WindowInvalid)
    );
    assert_eq!(out, before);
}

/// Gate 5. Turns red if the moved code allocates on the browser's audio thread: the control calls
/// run in render-locked exports there.
#[test]
fn control_calls_allocate_and_free_nothing() {
    let (mut rig, mut service) = collection_service();
    bench_alloc::assert_installed();
    // Warm up one full cycle so every process static is initialized.
    service.select(TRACK, SpectrumChannels::Left).unwrap();
    rig.window();
    let _ = read(&mut service, SpectrumChannels::Left);
    service.arm().unwrap();
    service.cancel().unwrap();
    rig.window();

    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    rig.window();
    let mark = bench_alloc::current_thread_counters();
    let mut seen = 0_u64;
    seen += u64::from(service.arm().is_err());
    let snapshot = service.read(SpectrumChannels::Stereo).unwrap().unwrap();
    seen += snapshot.token;
    service.arm().unwrap();
    assert!(service.read(SpectrumChannels::Stereo).unwrap().is_none());
    assert_eq!(
        service.select(TRACK, SpectrumChannels::Left),
        Err(SpectrumServiceError::Busy)
    );
    assert_eq!(
        service.select(TRACK, SpectrumChannels::Right),
        Err(SpectrumServiceError::UnknownEntry)
    );
    assert_eq!(
        service.read(SpectrumChannels::Left).map(|s| s.is_some()),
        Ok(false)
    );
    service.cancel().unwrap();
    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert!(seen > 1);
    assert_eq!(
        (delta.allocations, delta.reallocations, delta.deallocations),
        (0, 0, 0),
        "arm, read, cancel and select allocated or freed"
    );
}

// ---- The continuous stream (issue #1592) ----

const OUTPUT_KIND: u32 = SPECTRUM_TARGET_OUTPUT;
const INPUT_KIND: u32 = 1;
const HOP: u64 = SPECTRUM_WINDOW_FRAMES as u64;

fn smoothing(ms: f64) -> host_core::SpectrumSmoothingConfig {
    spectrum_smoothing(ms).expect("smoothing in range")
}

fn metadata(service: &SpectrumService) -> SpectrumStreamMetadata {
    *service.stream_metadata()
}

/// Read the stream once and return its metadata.
fn read_stream(
    service: &mut SpectrumService,
    analysis: &mut Option<SpectrumAnalysis>,
) -> SpectrumStreamMetadata {
    service.read_stream(analysis.as_mut()).expect("read");
    metadata(service)
}

/// Gate 1. Turns red if a status, an epoch or a history reset moves to a different call than the
/// browser's: warming, pending, ready with its sequence and hop spacing, a restart on a changed
/// entry or smoothing, a gap with its drops, the cancel refusal and the stop.
#[test]
fn stream_sequence_one_owner() {
    let (mut rig, mut service) = collection_service();
    let mut analysis = Some(SpectrumAnalysis::new());
    assert_eq!(
        service
            .select_stream(OUTPUT, SpectrumChannels::Stereo, None, analysis.as_mut())
            .unwrap_err(),
        SpectrumServiceError::StreamInactive
    );
    assert_eq!(service.select(OUTPUT, SpectrumChannels::Stereo), Ok(true));
    service
        .start_stream(smoothing(100.0), analysis.as_mut())
        .expect("start");
    assert!(service.stream_active());
    assert_eq!(
        service
            .start_stream(smoothing(100.0), analysis.as_mut())
            .unwrap_err(),
        SpectrumServiceError::Busy,
        "a stream already runs"
    );
    assert_eq!(
        service.select(TRACK, SpectrumChannels::Left),
        Err(SpectrumServiceError::StreamActive),
        "a running stream selects through select_stream"
    );
    assert_eq!(service.selection_epoch(), 1);
    let started = metadata(&service);
    assert_eq!(started.status, SpectrumStreamStatus::Warming);
    assert_eq!(started.target_kind, OUTPUT_KIND);
    assert_eq!(started.channels, 3);
    assert_eq!(
        (started.sample_rate_hz, started.quantum_frames),
        (48_000, 128)
    );
    assert_eq!(started.hop_frames, 2_048);
    assert_eq!(started.capture_epoch, 1);
    assert_eq!(started.smoothing_ms, 100.0);
    assert_eq!(started.analysis_epoch, 0);

    assert_eq!(
        read_stream(&mut service, &mut analysis).status,
        SpectrumStreamStatus::Warming
    );
    rig.block();
    assert_eq!(
        read_stream(&mut service, &mut analysis).status,
        SpectrumStreamStatus::Pending
    );
    for _ in 1..BLOCKS_PER_WINDOW {
        rig.block();
    }
    let mut out = SpectrumAnalysisResult::new();
    for sequence in 0..3_u64 {
        let ready = read_stream(&mut service, &mut analysis);
        assert_eq!(ready.status, SpectrumStreamStatus::Ready);
        assert_eq!(ready.sequence, sequence);
        assert_eq!(ready.windows, sequence + 1);
        assert_eq!(ready.captured_sample, sequence * HOP, "default hop spacing");
        assert_eq!(
            ready.end_sample,
            sequence * HOP + SPECTRUM_WINDOW_FRAMES as u64
        );
        assert_eq!(ready.capture_epoch, 1);
        let window = *service.stream_window().expect("a ready window");
        assert_eq!(window.first_sample, ready.captured_sample);
        let mut analyzed = ready;
        analysis
            .as_mut()
            .unwrap()
            .analyze_stream_into(&window, &mut out, &mut analyzed)
            .expect("analyzes");
        assert_eq!(analyzed.analysis_epoch, 0, "one history across the epoch");
        assert_eq!(analyzed.history_start_sample, 0);
        assert_eq!(out.token, sequence + 1);
        rig.window();
    }
    assert!(analysis.as_ref().unwrap().has_stream_history());

    // A new entry restarts the capture and resets the history at one commit.
    assert_eq!(service.selection_epoch(), 1);
    let _ = read_stream(&mut service, &mut analysis);
    assert_eq!(
        service.select_stream(TRACK, SpectrumChannels::Right, None, analysis.as_mut()),
        Err(SpectrumServiceError::UnknownEntry)
    );
    assert_eq!(metadata(&service).status, SpectrumStreamStatus::Ready);
    assert_eq!(
        service.select_stream(
            OUTPUT,
            SpectrumChannels::Stereo,
            Some(smoothing(100.0)),
            analysis.as_mut()
        ),
        Ok(false),
        "the same entry and smoothing change nothing"
    );
    assert_eq!(metadata(&service).status, SpectrumStreamStatus::Ready);
    let before = metadata(&service);
    assert_eq!(
        service.select_stream(
            TRACK,
            SpectrumChannels::Left,
            Some(smoothing(100.0)),
            analysis.as_mut()
        ),
        Ok(true)
    );
    assert_eq!(service.selection_epoch(), 2);
    let switched = metadata(&service);
    assert_eq!(switched.status, SpectrumStreamStatus::Warming);
    assert_eq!(switched.target_kind, INPUT_KIND);
    assert_eq!(switched.channels, 1);
    assert_eq!(
        switched.capture_epoch, 1,
        "each entry counts its own epochs"
    );
    assert_ne!(switched.capture_epoch, 0);
    let _ = before;
    assert_eq!(switched.analysis_epoch, 1, "the history reset");
    assert_eq!(switched.history_start_sample, 0);
    assert_eq!((switched.sequence, switched.windows), (0, 0));
    assert!(service.stream_window().is_none());
    rig.window();
    let after_switch = read_stream(&mut service, &mut analysis);
    assert_eq!(after_switch.status, SpectrumStreamStatus::Ready);
    assert_eq!(
        after_switch.target_kind, INPUT_KIND,
        "the next window is the new entry's"
    );
    assert_eq!(after_switch.sequence, 0);
    assert_eq!(
        service.stream_window().unwrap().channels,
        SpectrumChannels::Left
    );

    // The same entry with other smoothing restarts the capture epoch.
    assert_eq!(
        service.select_stream(
            TRACK,
            SpectrumChannels::Left,
            Some(smoothing(50.0)),
            analysis.as_mut()
        ),
        Ok(true)
    );
    let restarted = metadata(&service);
    assert_eq!(restarted.capture_epoch, after_switch.capture_epoch + 1);
    assert_eq!(restarted.analysis_epoch, 2);
    assert_eq!(restarted.smoothing_ms, 50.0);
    assert_eq!(service.selection_epoch(), 2, "the entry did not change");
    assert_eq!(
        analysis.as_ref().unwrap().stream_smoothing(),
        Some(smoothing(50.0))
    );

    // Three hops that nobody reads: the queue drops two and the read reports a gap.
    for _ in 0..3 {
        rig.window();
    }
    let gap = read_stream(&mut service, &mut analysis);
    assert_eq!(gap.status, SpectrumStreamStatus::Gap);
    assert_eq!(gap.dropped_captures, 2);
    assert_eq!(gap.analysis_epoch, 3, "a gap resets the history");
    assert!(service.stream_window().is_none());
    let recovered = read_stream(&mut service, &mut analysis);
    assert_eq!(
        recovered.status,
        SpectrumStreamStatus::Ready,
        "one window recovers the stream"
    );

    assert_eq!(service.cancel(), Err(SpectrumServiceError::StreamActive));
    service.stop_stream(None);
    assert!(!service.stream_active());
    assert_eq!(metadata(&service).status, SpectrumStreamStatus::Stopped);
    assert!(service.stream_window().is_none());
    assert_eq!(
        read_stream(&mut service, &mut analysis).status,
        SpectrumStreamStatus::Inactive
    );
    service.cancel().expect("a stopped stream may cancel");
}

/// Gate 2. Turns red if the service starts or switches a stream with the default cadence when the
/// preparation named a hop.
#[test]
fn an_explicit_hop_survives_start_and_select() {
    let hop = SpectrumHop::new(1_024).unwrap();
    let (mut rig, mut service) = collection_service_with_hop(Some(hop));
    let mut analysis = None;
    assert_eq!(service.hop(), Some(hop));
    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    service.start_stream(smoothing(0.0), None).unwrap();
    assert_eq!(metadata(&service).hop_frames, 1_024);
    for _ in 0..BLOCKS_PER_WINDOW {
        rig.block();
    }
    let first = read_stream(&mut service, &mut analysis);
    assert_eq!(first.status, SpectrumStreamStatus::Ready);
    for _ in 0..BLOCKS_PER_WINDOW / 2 {
        rig.block();
    }
    let second = read_stream(&mut service, &mut analysis);
    assert_eq!(second.captured_sample - first.captured_sample, 1_024);

    assert_eq!(
        service.select_stream(TRACK, SpectrumChannels::Left, None, None),
        Ok(true)
    );
    assert_eq!(service.stream_cadence().unwrap().hop_frames(), 1_024);
    assert_eq!(metadata(&service).hop_frames, 1_024);
    for _ in 0..BLOCKS_PER_WINDOW {
        rig.block();
    }
    let third = read_stream(&mut service, &mut analysis);
    assert_eq!(third.status, SpectrumStreamStatus::Ready, "{third:?}");
    for _ in 0..BLOCKS_PER_WINDOW / 2 {
        rig.block();
    }
    let fourth = read_stream(&mut service, &mut analysis);
    assert_eq!(fourth.status, SpectrumStreamStatus::Ready, "{fourth:?}");
    assert_eq!(
        fourth.captured_sample - third.captured_sample,
        1_024,
        "the switched entry keeps the hop"
    );
}

/// Gate 3. Turns red if a stream start discards a caller's arm, or no longer replaces a
/// select-made arm.
#[test]
fn a_stream_start_never_discards_a_callers_arm() {
    let (mut rig, mut service) = collection_service();
    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    service
        .start_stream(smoothing(0.0), None)
        .expect("a select-made arm is replaced");
    service.stop_stream(None);
    service.cancel().unwrap();
    service.arm().unwrap();
    assert_eq!(
        service.start_stream(smoothing(0.0), None).unwrap_err(),
        SpectrumServiceError::Busy
    );
    assert!(!service.stream_active());
    rig.window();
    let snapshot = read(&mut service, SpectrumChannels::Stereo);
    assert_eq!(snapshot.token, 1, "the armed window still reads");
    service
        .start_stream(smoothing(0.0), None)
        .expect("the read ended the caller's claim");
    service.stop_stream(None);

    let (_rig, mut single) = single_service(SpectrumChannels::Left);
    single.arm().unwrap();
    assert_eq!(
        single.start_stream(smoothing(0.0), None).unwrap_err(),
        SpectrumServiceError::Busy
    );
    single.cancel().unwrap();
    single
        .start_stream(smoothing(0.0), None)
        .expect("an idle single starts");
    assert_eq!(single.cancel(), Err(SpectrumServiceError::StreamActive));
}

fn ready_stream() -> (Rig, SpectrumService, SpectrumAnalysis) {
    let (mut rig, mut service) = collection_service();
    let mut analysis = SpectrumAnalysis::new();
    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    service
        .start_stream(smoothing(100.0), Some(&mut analysis))
        .unwrap();
    rig.window();
    service.read_stream(Some(&mut analysis)).unwrap();
    assert_eq!(metadata(&service).status, SpectrumStreamStatus::Ready);
    (rig, service, analysis)
}

/// Gate 4. Turns red if the analysis instance accepts metadata that the one-owner path would never
/// write, leaves a partly adopted configuration behind a refusal, or analyzes an imported window
/// differently from the one-owner result.
#[test]
fn imported_metadata_is_checked_and_analyzes_like_the_owner() {
    let (mut rig, service, mut owner) = ready_stream();
    let valid = metadata(&service);
    let window = *service.stream_window().unwrap();
    let mut owner_out = SpectrumAnalysisResult::new();
    let mut owner_metadata = valid;
    owner
        .analyze_stream_into(&window, &mut owner_out, &mut owner_metadata)
        .unwrap();

    let mut worker = SpectrumAnalysis::new();
    assert!(!worker.stream_active());
    worker.configure_imported(&valid).expect("a valid import");
    assert!(worker.stream_active() && worker.has_stream_history());
    let mut worker_out = SpectrumAnalysisResult::new();
    let mut worker_metadata = valid;
    worker
        .analyze_stream_into(&window, &mut worker_out, &mut worker_metadata)
        .unwrap();
    assert_eq!(worker_out, owner_out);
    assert_eq!(worker_metadata, owner_metadata);

    // Each malformed field is refused and leaves the adopted state alone.
    type Mutation = fn(&mut SpectrumStreamMetadata);
    let cases: [(&str, Mutation); 14] = [
        ("status", |m| m.status = SpectrumStreamStatus::Pending),
        ("kind 0", |m| m.target_kind = 0),
        ("kind 4", |m| m.target_kind = 4),
        ("channels 0", |m| m.channels = 0),
        ("channels 4", |m| m.channels = 4),
        ("epoch 0", |m| m.capture_epoch = 0),
        ("windows", |m| m.windows += 1),
        ("sequence overflow", |m| m.sequence = u64::MAX),
        ("end sample", |m| m.end_sample += 1),
        ("rate", |m| m.sample_rate_hz = 32_000),
        ("quantum", |m| m.quantum_frames = 0),
        ("hop", |m| m.hop_frames = 300),
        ("smoothing nan", |m| m.smoothing_ms = f64::NAN),
        ("smoothing high", |m| m.smoothing_ms = 10_001.0),
    ];
    let state = |a: &SpectrumAnalysis| {
        (
            a.stream_active(),
            a.stream_cadence(),
            a.stream_smoothing(),
            a.analysis_epoch(),
            a.history_start_sample(),
            a.has_stream_history(),
        )
    };
    let before = state(&worker);
    let mut fresh = SpectrumAnalysis::new();
    for (name, mutate) in cases {
        let mut bad = valid;
        mutate(&mut bad);
        assert_eq!(
            worker.configure_imported(&bad),
            Err(SpectrumServiceError::MetadataInvalid),
            "{name}"
        );
        assert_eq!(state(&worker), before, "{name}: an adopted state changed");
        assert_eq!(
            fresh.configure_imported(&bad),
            Err(SpectrumServiceError::MetadataInvalid),
            "{name}"
        );
        assert!(
            !fresh.stream_active() && !fresh.has_stream_history(),
            "{name}: a refused import adopted state"
        );
    }

    // An analysis that was reset, or never fed a window, refuses.
    worker.reset_stream().unwrap();
    assert_eq!(worker.analysis_epoch(), 1);
    let mut kept = SpectrumAnalysisResult::new();
    assert_eq!(
        worker.analyze_stream_into(&window, &mut kept, &mut worker_metadata),
        Err(SpectrumServiceError::WindowInvalid)
    );
    assert_eq!(kept, SpectrumAnalysisResult::new());
    worker.end_stream();
    assert_eq!(
        worker.reset_stream(),
        Err(SpectrumServiceError::StreamInactive)
    );
    assert_eq!(
        worker.analyze_stream_into(&window, &mut kept, &mut worker_metadata),
        Err(SpectrumServiceError::StreamInactive)
    );
    rig.window();
}

/// Gate 5. Turns red if the moved stream code allocates or frees on the browser's audio thread:
/// the capture-side calls run in render-locked exports with no analysis.
#[test]
fn stream_calls_allocate_and_free_nothing() {
    let (mut rig, mut service) = collection_service();
    bench_alloc::assert_installed();
    let cycle = |rig: &mut Rig, service: &mut SpectrumService| -> u64 {
        let mut seen = 0_u64;
        service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
        service.start_stream(smoothing(100.0), None).unwrap();
        seen += u64::from(service.stream_window().is_none());
        service.read_stream(None).unwrap();
        for _ in 0..2 {
            rig.window();
        }
        service.read_stream(None).unwrap();
        seen += service
            .stream_window()
            .map_or(0, |window| window.sequence + 1);
        assert_eq!(
            service.select_stream(TRACK, SpectrumChannels::Left, Some(smoothing(50.0)), None),
            Ok(true)
        );
        assert_eq!(
            service.select_stream(TRACK, SpectrumChannels::Left, Some(smoothing(10.0)), None),
            Ok(true)
        );
        assert_eq!(service.cancel(), Err(SpectrumServiceError::StreamActive));
        assert_eq!(
            service.start_stream(smoothing(0.0), None),
            Err(SpectrumServiceError::Busy)
        );
        for _ in 0..3 {
            rig.window();
        }
        service.read_stream(None).unwrap();
        service.read_stream(None).unwrap();
        service.stop_stream(None);
        service.cancel().unwrap();
        seen + u64::from(service.stream_cadence().is_none())
    };
    // Warm up one full cycle so every process static is initialized.
    let _ = cycle(&mut rig, &mut service);
    let mark = bench_alloc::current_thread_counters();
    let seen = cycle(&mut rig, &mut service);
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert!(seen > 1);
    assert_eq!(
        (delta.allocations, delta.reallocations, delta.deallocations),
        (0, 0, 0),
        "start, select, read, stop or cancel allocated or freed"
    );
}

/// Fold (batch 8 review). Turns red if the one target select stops choosing the road by the
/// stream's state (a host adapter would have to repeat that choice), if a smoothing without a
/// running stream changes the selection, or if the stop leaves the analysis running or the
/// metadata without the analysis facts.
#[test]
fn one_select_and_one_stop_serve_the_entry_and_the_stream() {
    let (mut rig, mut service) = collection_service();
    let mut analysis = Some(SpectrumAnalysis::new());
    // No stream: the plain entry selection; a smoothing has no stream to change.
    assert_eq!(
        service.select_target(
            TRACK,
            SpectrumChannels::Left,
            Some(smoothing(50.0)),
            analysis.as_mut()
        ),
        Err(SpectrumServiceError::StreamInactive)
    );
    assert_eq!(service.selection_epoch(), 0, "a refusal changed the entry");
    assert_eq!(
        service.select_target(OUTPUT, SpectrumChannels::Stereo, None, analysis.as_mut()),
        Ok(false)
    );
    assert_eq!(service.selection_epoch(), 1);
    // A stream: the restarting selection; the same entry is no restart.
    service
        .start_stream(smoothing(100.0), analysis.as_mut())
        .expect("start");
    assert_eq!(
        service.select_target(OUTPUT, SpectrumChannels::Stereo, None, analysis.as_mut()),
        Ok(false)
    );
    assert_eq!(
        service.select_target(TRACK, SpectrumChannels::Left, None, analysis.as_mut()),
        Ok(true)
    );
    assert_eq!(metadata(&service).status, SpectrumStreamStatus::Warming);
    assert_eq!(metadata(&service).target_kind, INPUT_KIND);
    assert_eq!(
        service.select_target(
            TRACK,
            SpectrumChannels::Left,
            Some(smoothing(10.0)),
            analysis.as_mut()
        ),
        Ok(true),
        "a changed smoothing restarts the stream"
    );
    assert_eq!(metadata(&service).smoothing_ms, 10.0);
    // The stop ends the analysis with the service, and the metadata carries its facts.
    let mut ready = metadata(&service);
    for _ in 0..4 {
        rig.window();
        ready = read_stream(&mut service, &mut analysis);
        if ready.status == SpectrumStreamStatus::Ready {
            break;
        }
    }
    assert_eq!(ready.status, SpectrumStreamStatus::Ready);
    let window = *service.stream_window().expect("ready window");
    let (mut out, mut facts) = (SpectrumAnalysisResult::new(), ready);
    analysis
        .as_mut()
        .unwrap()
        .analyze_stream_into(&window, &mut out, &mut facts)
        .expect("analysis");
    assert!(analysis.as_ref().unwrap().has_stream_history());
    service.stop_stream(analysis.as_mut());
    let stopped = metadata(&service);
    assert_eq!(stopped.status, SpectrumStreamStatus::Stopped);
    let owned = analysis.as_ref().unwrap();
    assert!(!owned.stream_active());
    assert_eq!(stopped.analysis_epoch, owned.analysis_epoch());
    assert_eq!(stopped.history_start_sample, owned.history_start_sample());
    assert_eq!(stopped.history_start_sample, facts.history_start_sample);
    assert_eq!(stopped.analysis_epoch, facts.analysis_epoch);
}

/// Fold (batch 8 review). Turns red if an instance that owns both halves of a stream adopts a
/// window the metadata does not describe as ready, or reports a stopped or never-started stream
/// as a missing window (the browser answers the first as invalid input, the second as a wrong
/// state).
#[test]
fn adoption_follows_the_metadata_status() {
    let with = |status| SpectrumStreamMetadata {
        status,
        ..SpectrumStreamMetadata::default()
    };
    for status in [
        SpectrumStreamStatus::Inactive,
        SpectrumStreamStatus::Stopped,
    ] {
        assert_eq!(
            SpectrumAnalysis::adoption_needed(None, &with(status)),
            Err(SpectrumServiceError::StreamInactive),
            "{status:?}"
        );
    }
    for status in [
        SpectrumStreamStatus::Warming,
        SpectrumStreamStatus::Pending,
        SpectrumStreamStatus::Gap,
        SpectrumStreamStatus::Failed,
    ] {
        assert_eq!(
            SpectrumAnalysis::adoption_needed(Some(&SpectrumAnalysis::new()), &with(status)),
            Err(SpectrumServiceError::WindowInvalid),
            "{status:?}"
        );
    }
    assert_eq!(
        SpectrumAnalysis::adoption_needed(None, &with(SpectrumStreamStatus::Ready)),
        Ok(true)
    );
    // An analysis that runs the stream never adopts, whatever the record says.
    let (mut rig, mut service) = collection_service();
    let mut analysis = Some(SpectrumAnalysis::new());
    service.select(OUTPUT, SpectrumChannels::Stereo).unwrap();
    service
        .start_stream(smoothing(100.0), analysis.as_mut())
        .unwrap();
    rig.window();
    assert_eq!(
        SpectrumAnalysis::adoption_needed(analysis.as_ref(), &with(SpectrumStreamStatus::Pending)),
        Ok(false)
    );
}

fn entry(target: SpectrumTarget, channels: SpectrumChannels) -> SpectrumCaptureCollectionEntry {
    SpectrumCaptureCollectionEntry { target, channels }
}

/// #1593 D4. Turns red if a successor is prepared with an entry whose target the edit removed
/// (which `host.spectrum.target` refuses), if a kept entry moves, changes its mask or loses the
/// budget or the hop, or if a configuration with nothing left is not `None`.
#[test]
fn retain_present_keeps_present_entries_in_order() {
    let mut model = session::parse_session_json(include_str!(
        "../../../fixtures/session/v1/builtins-automation.json"
    ))
    .expect("fixture parses");
    let hop = Some(SpectrumHop::new(512).expect("hop"));
    let collection = |entries| SpectrumConfiguration {
        preparation: SpectrumPreparation::Collection(SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes: 1_234_567,
        }),
        hop,
    };
    let output = entry(
        SpectrumTarget::Output("main-out".into()),
        SpectrumChannels::Stereo,
    );
    let submix = entry(
        SpectrumTarget::TrackPostMatrix("bus".into()),
        SpectrumChannels::Left,
    );
    let track = entry(
        SpectrumTarget::TrackPostInputBuiltins("vocal".into()),
        SpectrumChannels::Right,
    );
    let configuration = collection(vec![
        output.clone(),
        entry(
            SpectrumTarget::TrackPostMatrix("gone".into()),
            SpectrumChannels::Left,
        ),
        submix.clone(),
        entry(
            SpectrumTarget::Output("alt-out".into()),
            SpectrumChannels::Left,
        ),
        track.clone(),
        // A track tap never names an output, and an output entry never names a strip.
        entry(
            SpectrumTarget::TrackPostMatrix("main-out".into()),
            SpectrumChannels::Left,
        ),
        entry(
            SpectrumTarget::Output("vocal".into()),
            SpectrumChannels::Left,
        ),
    ]);
    assert_eq!(
        configuration.retain_present(&model),
        Some(collection(vec![output, submix.clone(), track.clone()]))
    );

    let single = |target: &str| SpectrumConfiguration {
        preparation: SpectrumPreparation::Single(SpectrumCaptureRequest {
            target: SpectrumTarget::TrackPostMatrix(target.into()),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: 99_999,
        }),
        hop,
    };
    assert_eq!(
        single("vocal").retain_present(&model),
        Some(single("vocal"))
    );
    assert_eq!(single("gone").retain_present(&model), None);

    model.tracks.clear();
    model.outputs.clear();
    assert_eq!(
        configuration.retain_present(&model),
        Some(collection(vec![submix]))
    );
    model.submixes.clear();
    assert_eq!(configuration.retain_present(&model), None);
    assert_eq!(collection(vec![track]).retain_present(&model), None);
}

/// #1593 D3. Turns red if the one session entry prepares its spectrum observer differently from
/// the reference path (a second preparation path drifting): the same document fed the same PCM
/// gives the same window, field by field; and with `None` it returns no captures.
#[test]
fn the_one_entry_prepares_the_reference_window() {
    // Live controls attach every strip's meter.
    let caps = HostPrepareCaps {
        maximum_meter_streams: 1_000,
        maximum_meter_items: 1_000_000,
        maximum_meter_bytes: 100_000_000,
        ..caps()
    };
    let compiled = compile_host_session(SESSION, &caps).expect("compiled fixture");
    let target = SpectrumTarget::Output("main-out".into());
    let request = SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    };
    let superseded = LiveValuesSuperseded::default();
    let live = HostLiveControlRequest::default();
    let (_, _, none) = prepare_host_runtime_with_live_lanes_and_spectrum(
        &compiled,
        &caps,
        &live,
        HostLiveLanes::ALL,
        None,
        None,
        &superseded,
    )
    .expect("prepares without spectrum");
    assert!(none.is_none());

    let (host, _handles, captures) = prepare_host_runtime_with_live_lanes_and_spectrum(
        &compiled,
        &caps,
        &live,
        HostLiveLanes::ALL,
        Some(&SpectrumPreparation::Single(request.clone())),
        None,
        &superseded,
    )
    .expect("prepares with spectrum");
    let (reference_host, reference_capture) =
        prepare_host_runtime_with_spectrum(&compiled, &caps, &request).expect("reference");
    let mut services = [
        (
            Rig::new(host),
            SpectrumService::new(
                captures.expect("captures"),
                SAMPLE_RATE_HZ,
                QUANTUM as u32,
                None,
            ),
        ),
        (
            Rig::new(reference_host),
            SpectrumService::new(
                PreparedSpectrumCaptures::from(reference_capture),
                SAMPLE_RATE_HZ,
                QUANTUM as u32,
                None,
            ),
        ),
    ];
    let [got, want] = services.each_mut().map(|(rig, service)| {
        rig.window();
        service.arm().expect("arm");
        rig.window();
        read(service, SpectrumChannels::Stereo)
    });
    assert_eq!(got.target_kind, want.target_kind);
    assert_eq!(got.sample_rate_hz, want.sample_rate_hz);
    assert_eq!(got.token, want.token);
    assert_eq!(got.window.first_sample, want.window.first_sample);
    assert_eq!(got.window.channels, want.window.channels);
    assert_eq!(got.window.source_underrun, want.window.source_underrun);
    for (got, want) in [
        (&got.window.left, &want.window.left),
        (&got.window.right, &want.window.right),
    ] {
        assert!(
            got.iter().any(|sample| *sample != 0.0),
            "a window of signal"
        );
        assert!(
            got.iter()
                .zip(want)
                .all(|(got, want)| got.to_bits() == want.to_bits()),
            "the planes equal the reference's bit for bit"
        );
    }
}
