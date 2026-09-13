//! Native graph-to-window spectrum capture gates for issue #781.

use host_core::{
    HostConsoleRequest, HostPrepareCaps, HostShapePolicy, SPECTRUM_BIN_COUNT,
    SPECTRUM_WINDOW_FRAMES, SourceSubmission, SpectrumAnalysisError, SpectrumAnalyzer,
    SpectrumCaptureReadError, SpectrumCaptureRequest, SpectrumChannels, SpectrumTarget,
    compile_host_session, prepare_host_runtime, prepare_host_runtime_with_console_and_spectrum,
    prepare_host_runtime_with_spectrum, spectrum_capture_resources, spectrum_capture_resources_for,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;

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

fn render_window(
    host: host_core::PreparedHost,
    mut capture: host_core::SpectrumCapture,
) -> host_core::SpectrumWindow {
    capture.arm().expect("first spectrum arm");
    assert_eq!(
        capture.try_read().expect_err("window is initially pending"),
        SpectrumCaptureReadError::Pending
    );
    assert!(capture.arm().is_err(), "one outstanding capture");
    let (mut session, mut sources, _) = host.start_render_session().expect("start render");
    for block in 0..(SPECTRUM_WINDOW_FRAMES / QUANTUM) {
        let left = [0.25_f32; QUANTUM];
        let right = [-0.5_f32; QUANTUM];
        sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        session
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
            .expect("render block");
    }
    let window = capture.try_read().expect("complete spectrum window");
    assert_eq!(window.first_sample, 0);
    assert_eq!(window.end_sample(), Some(SPECTRUM_WINDOW_FRAMES as u64));
    window
}

#[test]
fn prepared_graph_captures_all_three_real_boundaries_and_analyzes() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let targets = [
        SpectrumTarget::TrackPostInputBuiltins("eq0".into()),
        SpectrumTarget::TrackPostMatrix("eq0".into()),
        SpectrumTarget::Output("main-out".into()),
    ];
    for target in targets {
        let capture_bytes = spectrum_capture_resources_for(&target).retained_bytes;
        let (host, capture) = prepare_host_runtime_with_spectrum(
            &compiled,
            &caps(),
            &SpectrumCaptureRequest {
                target,
                channels: SpectrumChannels::Stereo,
                maximum_capture_bytes: capture_bytes,
            },
        )
        .expect("selected target prepares");
        let window = render_window(host, capture);
        assert!(window.left.iter().any(|value| *value != 0.0));
        assert!(window.right.iter().any(|value| *value != 0.0));

        let analyzer = SpectrumAnalyzer::new();
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left_dbfs = [0.0; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [0.0; SPECTRUM_BIN_COUNT];
        let mut output = host_core::SpectrumOutput {
            frequencies_hz: &mut frequencies,
            left_dbfs: Some(&mut left_dbfs),
            right_dbfs: Some(&mut right_dbfs),
        };
        analyzer
            .analyze(&window, 48_000, &mut output)
            .expect("captured window analyzes");
        assert_eq!(frequencies[0], 0.0);
        assert!(left_dbfs.iter().all(|value| value.is_finite()));
        assert!(right_dbfs.iter().all(|value| value.is_finite()));
    }
}

#[test]
fn spectrum_capture_refuses_unknown_target_and_small_budget_without_partial_state() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let resources = spectrum_capture_resources();
    let unknown = match prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target: SpectrumTarget::TrackPostMatrix("missing".into()),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: resources.retained_bytes,
        },
    ) {
        Ok(_) => panic!("unknown target accepted"),
        Err(error) => error,
    };
    assert!(String::from_utf8_lossy(unknown.as_bytes()).contains("host.spectrum.target"));

    let small = match prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: resources.retained_bytes - 1,
        },
    ) {
        Ok(_) => panic!("small capture budget accepted"),
        Err(error) => error,
    };
    assert!(String::from_utf8_lossy(small.as_bytes()).contains("host.spectrum.capture_budget"));

    let output_target = SpectrumTarget::Output("main-out".into());
    let output_resources = spectrum_capture_resources_for(&output_target);
    let (host, mut capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target: output_target,
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: output_resources.retained_bytes,
        },
    )
    .expect("valid capture prepares");
    drop(host);
    capture.arm().expect("arm after preparation");
    capture.cancel();
    assert_eq!(
        capture.try_read().expect_err("cancel leaves no result"),
        SpectrumCaptureReadError::NotArmed
    );
}

#[test]
fn analyzer_preserves_outputs_on_invalid_rate_and_nonfinite_window() {
    let analyzer = SpectrumAnalyzer::new();
    let window = host_core::SpectrumWindow {
        left: [0.0; SPECTRUM_WINDOW_FRAMES],
        right: [0.0; SPECTRUM_WINDOW_FRAMES],
        first_sample: 0,
        channels: SpectrumChannels::Stereo,
        source_underrun: false,
    };
    let mut frequencies = [1.0; SPECTRUM_BIN_COUNT];
    let mut left_dbfs = [2.0; SPECTRUM_BIN_COUNT];
    let mut right_dbfs = [3.0; SPECTRUM_BIN_COUNT];
    let mut output = host_core::SpectrumOutput {
        frequencies_hz: &mut frequencies,
        left_dbfs: Some(&mut left_dbfs),
        right_dbfs: Some(&mut right_dbfs),
    };
    assert_eq!(
        analyzer
            .analyze(&window, 192_000, &mut output)
            .expect_err("extended rate refuses"),
        SpectrumAnalysisError::UnsupportedRate
    );
    let mut invalid = window;
    invalid.right[7] = f32::INFINITY;
    assert_eq!(
        analyzer
            .analyze(&invalid, 48_000, &mut output)
            .expect_err("nonfinite window refuses"),
        SpectrumAnalysisError::InvalidWindow
    );
    assert_eq!(frequencies[0], 1.0);
    assert_eq!(left_dbfs[0], 2.0);
    assert_eq!(right_dbfs[0], 3.0);
}

#[test]
fn graph_wide_source_underrun_is_retained_in_the_completed_window() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let target = SpectrumTarget::Output("main-out".into());
    let resources = spectrum_capture_resources_for(&target);
    let (host, mut capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target,
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("capture prepares");
    capture.arm().expect("arm capture");
    let (mut session, mut sources, _) = host.start_render_session().expect("start render");
    for block in 0..(SPECTRUM_WINDOW_FRAMES / QUANTUM) {
        let left = [0.25_f32; QUANTUM];
        let right = [-0.5_f32; QUANTUM];
        if block != 0 {
            sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: (block * QUANTUM) as u64,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
        }
        let mut output = [0.0_f32; QUANTUM * 2];
        session
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
            .expect("render block");
        if block == 0 {
            sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: 0,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("late first source block");
        }
    }
    let window = capture.try_read().expect("underrun window completes");
    assert!(
        window.source_underrun,
        "graph-wide underrun must be retained"
    );
}

#[test]
fn seek_generation_invalidates_a_partial_spectrum_window() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let target = SpectrumTarget::Output("main-out".into());
    let resources = spectrum_capture_resources_for(&target);
    let (host, mut capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target,
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("capture prepares");
    capture.arm().expect("arm capture");
    let (mut session, mut sources, _) = host.start_render_session().expect("start render");
    for block in 0..8 {
        let left = [0.25_f32; QUANTUM];
        let right = [-0.5_f32; QUANTUM];
        sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        session
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
            .expect("render block");
    }
    sources
        .seek(b"fixture-source", 2, 2_048)
        .expect("queue generation change");
    assert!(
        session.prepare_source_seek(0, 2, 2_048),
        "exclusive plan owner applies seek"
    );
    let fresh_left = [0.75_f32; QUANTUM];
    let fresh_right = [-0.25_f32; QUANTUM];
    sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 2_048,
                sample_rate_hz: 48_000,
                planes: &[&fresh_left, &fresh_right],
                frames: QUANTUM as u32,
                end_of_region: false,
            },
        )
        .expect("fresh generation block");
    let mut output = [0.0_f32; QUANTUM * 2];
    session
        .render_planar(&mut output, 2, QUANTUM, QUANTUM, 1_024)
        .expect("render after seek");
    assert_eq!(
        capture
            .try_read()
            .expect_err("seek invalidates partial window"),
        SpectrumCaptureReadError::Invalid
    );
}

#[test]
fn preexecutor_render_refusal_invalidates_a_partial_spectrum_window() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let target = SpectrumTarget::Output("main-out".into());
    let resources = spectrum_capture_resources_for(&target);
    let (host, mut capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &caps(),
        &SpectrumCaptureRequest {
            target,
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("capture prepares");
    capture.arm().expect("arm capture");
    let (mut session, mut sources, _) = host.start_render_session().expect("start render");
    for block in 0..8 {
        let left = [0.25_f32; QUANTUM];
        let right = [-0.5_f32; QUANTUM];
        sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut output = [0.0_f32; QUANTUM * 2];
        session
            .render_planar(&mut output, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
            .expect("render block");
    }

    let mut output = [0.0_f32; QUANTUM * 2];
    let error = session
        .render_planar(&mut output, 2, QUANTUM, QUANTUM, 7)
        .expect_err("discontinuous preexecutor call refuses");
    assert!(
        matches!(
            error,
            engine::realtime::RenderError::TimeDiscontinuity { .. }
        ),
        "unexpected refusal: {error:?}"
    );
    assert_eq!(
        capture
            .try_read()
            .expect_err("discontinuous render invalidates partial capture"),
        SpectrumCaptureReadError::Invalid
    );
    capture.arm().expect("invalid result can be re-armed");
}

#[test]
fn console_and_meter_preparation_keeps_spectrum_in_one_transaction() {
    let compiled = compile_host_session(SESSION, &caps()).expect("compiled fixture");
    let mut limits = caps();
    limits.maximum_meter_streams = 9;
    limits.maximum_meter_items = u64::MAX;
    limits.maximum_meter_bytes = u64::MAX;
    let console = HostConsoleRequest {
        control_queue_depth: core::num::NonZeroUsize::new(1),
        meter_period_frames: core::num::NonZeroU32::new(128),
        ..HostConsoleRequest::default()
    };
    let target = SpectrumTarget::Output("main-out".into());
    let resources = spectrum_capture_resources_for(&target);
    let (host, handles, capture) = prepare_host_runtime_with_console_and_spectrum(
        &compiled,
        &limits,
        &console,
        &SpectrumCaptureRequest {
            target,
            channels: SpectrumChannels::Left,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("console, meters and spectrum prepare together");
    assert_eq!(handles.track_controls.len(), 9);
    assert_eq!(handles.meters.len(), 9);
    assert_eq!(
        host.report.spectrum_capture_retained_bytes,
        resources.retained_bytes
    );
    assert_eq!(capture.target(), &SpectrumTarget::Output("main-out".into()));
}

#[test]
fn selected_capture_is_pcm_bit_exact_and_allocation_free_for_idle_and_active_renders() {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;

    const NON_DIVIDING_QUANTUM: usize = 192;
    const BLOCKS: usize = SPECTRUM_WINDOW_FRAMES.div_ceil(NON_DIVIDING_QUANTUM);
    let document = SESSION.replace("\"quantum_frames\": 128", "\"quantum_frames\": 192");
    let mut test_caps = caps();
    test_caps.source_ring_frames = 1_152;
    let compiled =
        compile_host_session(&document, &test_caps).expect("nondividing session compiles");
    let target = SpectrumTarget::Output("main-out".into());
    let resources = spectrum_capture_resources_for(&target);
    let baseline_host = prepare_host_runtime(&compiled, &test_caps).expect("baseline prepares");
    let (idle_host, mut idle_capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &test_caps,
        &SpectrumCaptureRequest {
            target: target.clone(),
            channels: SpectrumChannels::Left,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("idle capture prepares");
    let (active_host, mut active_capture) = prepare_host_runtime_with_spectrum(
        &compiled,
        &test_caps,
        &SpectrumCaptureRequest {
            target,
            channels: SpectrumChannels::Left,
            maximum_capture_bytes: resources.retained_bytes,
        },
    )
    .expect("active capture prepares");
    active_capture.arm().expect("active capture arms");

    let (mut baseline, mut baseline_sources, _) = baseline_host
        .start_render_session()
        .expect("baseline starts");
    let (mut idle, mut idle_sources, _) = idle_host.start_render_session().expect("idle starts");
    let (mut active, mut active_sources, _) =
        active_host.start_render_session().expect("active starts");
    audit::warm_up();
    bench_alloc::assert_installed();

    for block in 0..BLOCKS {
        let left = [0.25_f32; NON_DIVIDING_QUANTUM];
        let right = [-0.5_f32; NON_DIVIDING_QUANTUM];
        let start = (block * NON_DIVIDING_QUANTUM) as u64;
        for sources in [
            &mut baseline_sources,
            &mut idle_sources,
            &mut active_sources,
        ] {
            sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: start,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: NON_DIVIDING_QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("source block");
        }

        let mut baseline_pcm = [f32::from_bits(0x7fc0_3990); NON_DIVIDING_QUANTUM * 2];
        let mut idle_pcm = [f32::from_bits(0x7fc0_3990); NON_DIVIDING_QUANTUM * 2];
        let mut active_pcm = [f32::from_bits(0x7fc0_3990); NON_DIVIDING_QUANTUM * 2];
        audit::reset();
        let thread_mark = bench_alloc::current_thread_counters();
        let reports = audit::in_render_scope(|| {
            let baseline_report = baseline.render_planar(
                &mut baseline_pcm,
                2,
                NON_DIVIDING_QUANTUM,
                NON_DIVIDING_QUANTUM,
                start,
            );
            let idle_report = idle.render_planar(
                &mut idle_pcm,
                2,
                NON_DIVIDING_QUANTUM,
                NON_DIVIDING_QUANTUM,
                start,
            );
            let active_report = active.render_planar(
                &mut active_pcm,
                2,
                NON_DIVIDING_QUANTUM,
                NON_DIVIDING_QUANTUM,
                start,
            );
            (
                baseline_report,
                idle_report,
                active_report,
                audit::snapshot(),
            )
        });
        let thread_delta = bench_alloc::current_thread_delta_since(thread_mark);
        assert!(reports.0.is_ok(), "baseline render: {:?}", reports.0);
        assert!(reports.1.is_ok(), "idle render: {:?}", reports.1);
        assert!(reports.2.is_ok(), "active render: {:?}", reports.2);
        assert_eq!(
            (reports.3.allocations, reports.3.deallocations),
            (0, 0),
            "spectrum render touched the realtime allocator audit"
        );
        assert_eq!(
            (
                thread_delta.allocations,
                thread_delta.reallocations,
                thread_delta.deallocations
            ),
            (0, 0, 0),
            "spectrum render allocated or freed on the render owner"
        );
        for (baseline, idle) in baseline_pcm.iter().zip(idle_pcm.iter()) {
            assert_eq!(
                baseline.to_bits(),
                idle.to_bits(),
                "idle capture changed PCM"
            );
        }
        for (baseline, active) in baseline_pcm.iter().zip(active_pcm.iter()) {
            assert_eq!(
                baseline.to_bits(),
                active.to_bits(),
                "active capture changed PCM"
            );
        }
    }

    assert_eq!(
        idle_capture
            .try_read()
            .expect_err("idle observer never arms"),
        SpectrumCaptureReadError::NotArmed
    );
    let window = active_capture
        .try_read()
        .expect("nondividing quantum completes exact window");
    assert_eq!(window.channels, SpectrumChannels::Left);
    assert_eq!(window.first_sample, 0);
    assert_eq!(window.end_sample(), Some(SPECTRUM_WINDOW_FRAMES as u64));
    assert!(window.left.iter().any(|value| *value != 0.0));
    assert!(window.right.iter().all(|value| *value == 0.0));
}
