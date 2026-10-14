//! The one-shot spectrum service (issue #1591): request rules, arm/read/cancel/select sequences,
//! the arm-ownership ruling (D7), the analysis half and the allocation-free control calls.

use bench_support::alloc as bench_alloc;
use host_core::{
    HostPrepareCaps, HostShapePolicy, PreparedHost, PreparedSpectrumCaptures, SPECTRUM_BIN_COUNT,
    SPECTRUM_FLOOR_DB, SPECTRUM_MAXIMUM_CAPTURE_BYTES, SPECTRUM_MAXIMUM_ID_BYTES,
    SPECTRUM_TARGET_OUTPUT, SPECTRUM_WINDOW_FRAMES, SourceControlSet, SourceSubmission,
    SpectrumAnalysis, SpectrumAnalysisResult, SpectrumAnalyzer, SpectrumCaptureCollectionEntry,
    SpectrumCaptureCollectionRequest, SpectrumCaptureRequest, SpectrumChannels, SpectrumOutput,
    SpectrumPreparation, SpectrumRequestError, SpectrumService, SpectrumServiceError,
    SpectrumSnapshot, SpectrumTarget, SpectrumTargetRef, StartedRenderSession,
    compile_host_session, prepare_host_runtime_with_spectrum,
    prepare_host_runtime_with_spectrum_collection, spectrum_capture_collection_resources,
    spectrum_capture_resources_for, spectrum_channels_from_raw, spectrum_target_kind,
    spectrum_target_ref,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const BLOCKS_PER_WINDOW: usize = SPECTRUM_WINDOW_FRAMES / QUANTUM;
const SAMPLE_RATE_HZ: u32 = 48_000;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
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
        ),
    )
}

const TRACK: SpectrumTargetRef<'static> = SpectrumTargetRef::TrackPostInputBuiltins("eq0");
const OUTPUT: SpectrumTargetRef<'static> = SpectrumTargetRef::Output("main-out");

fn collection_service() -> (Rig, SpectrumService) {
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
    service.cancel();
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
    service.cancel();
    service.arm().unwrap();
    service.cancel();
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

    // A plane outside the mask holds the floor.
    let (left_only, _service, _rig) = window_snapshot(SpectrumChannels::Left);
    let mut left_out = SpectrumAnalysisResult::new();
    left_out.right_dbfs = [1.0; SPECTRUM_BIN_COUNT];
    analysis.analyze_into(&left_only, &mut left_out).unwrap();
    assert!(left_out.right_dbfs.iter().all(|v| *v == SPECTRUM_FLOOR_DB));
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
    service.cancel();
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
    service.cancel();
    assert_eq!(service.select(TRACK, SpectrumChannels::Left), Ok(true));
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert!(seen > 1);
    assert_eq!(
        (delta.allocations, delta.reallocations, delta.deallocations),
        (0, 0, 0),
        "arm, read, cancel and select allocated or freed"
    );
}
