//! Native graph-to-window spectrum capture gates for issue #781.

use host_core::{
    HostPrepareCaps, HostShapePolicy, SPECTRUM_BIN_COUNT, SPECTRUM_WINDOW_FRAMES, SourceSubmission,
    SpectrumAnalysisError, SpectrumAnalyzer, SpectrumCaptureReadError, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumTarget, compile_host_session, prepare_host_runtime_with_spectrum,
    spectrum_capture_resources, spectrum_capture_resources_for,
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
