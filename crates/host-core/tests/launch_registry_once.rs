//! Issue #1469 gate 1: the launch native-effect registry is built once per process and shared by
//! session preparation, live classification and the EQ response preview.
//!
//! The evaluation counter is process-global, so this binary holds one `#[test]` and nothing else
//! builds a registry in it. Every registry build evaluates each launch descriptor's
//! `tail_and_rest` once per declared quality row; a second build anywhere shows as a count above
//! one build's rows.
//!
//! The counter comes from `test-support`, and live classification from `control-provider`. Only
//! capi may enable `control-provider` (scripts/check-host-core-policy.sh), so this binary is a
//! no-op unless capi's dependency edge unifies it in, as in test-debug-a; `tests/live_delta.rs`
//! is gated the same way.
#![cfg(all(feature = "test-support", feature = "control-provider"))]

use std::sync::Barrier;

use effect_compiler::launch_native_effect_registry;
use effect_contract::{EffectQuality, LinkMode, tail_bound_evaluations};
use host_core::{
    HostPrepareCaps, HostShapePolicy, LAUNCH_SAMPLE_RATES, LiveRamps, ResponsePreviewLimits,
    ResponsePreviewRequest, ResponsePreviewTarget, classify_live_delta, prepare_host_session,
    prepare_response_preview,
};
use session::{
    Automation, AutomationSegment, AutomationShape, AutomationTarget, ParameterChannel,
    ParameterUnit, RackName, SessionModel, StableId, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const REPEATS: usize = 8;
const THREADS: usize = 4;
const STEP: LiveRamps = LiveRamps {
    fader_samples: 0,
    mute_samples: 0,
};

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn fixture() -> SessionModel {
    parse_session_json(FIXTURE).expect("fixture parses")
}

/// Session preparation through host-core's public entry, at `rate`.
fn prepare_at(rate: u32) {
    let mut model = fixture();
    model.sample_rate_hz = rate;
    let document = canonical_session_json(&model).expect("canonical");
    if let Err(failure) = prepare_host_session(&document, &caps()) {
        panic!(
            "prepare at {rate} Hz: {}",
            String::from_utf8_lossy(failure.as_bytes())
        );
    }
}

/// A live EQ parameter change on the first track's console EQ: reaches `parameter_records`.
fn classify_parameter_change(current: &SessionModel) {
    let mut next = current.clone();
    let gain = next.tracks[0].console[0]
        .params
        .iter_mut()
        .find(|param| param.parameter_id == 4 && param.channel == ParameterChannel::Left)
        .expect("the fixture's band-1 gain, left");
    gain.value = 3.0;
    let delta = classify_live_delta(current, &next, STEP).expect("a live EQ gain change");
    assert_eq!(delta.effects.len(), 1, "one instance's records");
}

/// A stored automation added on the first track's console EQ: reaches
/// `effect_automation_diagnostics`.
fn classify_automation_change(current: &SessionModel) {
    let mut next = current.clone();
    next.automation.push(Automation {
        id: StableId::parse("eq-ride").expect("stable id"),
        target: AutomationTarget {
            entity_id: next.tracks[0].id.clone(),
            rack: RackName::Console,
            effect_id: StableId::parse("eq").expect("stable id"),
            parameter_id: 4,
            channel: ParameterChannel::Left,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 960,
            start_value: 1.0,
            end_value: 1.0,
            unit: ParameterUnit::Db,
        }],
    });
    let delta = classify_live_delta(current, &next, STEP).expect("a block-rate EQ ride is live");
    assert!(delta.effects.is_empty() && delta.strips.is_empty());
}

/// An EQ response preview at `rate`.
fn preview_at(rate: u32) {
    prepare_response_preview(ResponsePreviewRequest {
        configuration_id: 1,
        sample_rate_hz: rate,
        quantum_frames: 128,
        target: ResponsePreviewTarget::Effect {
            effect_id: "miso.parametric-eq",
            overrides: &[],
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
        },
        limits: ResponsePreviewLimits::default(),
    })
    .expect("EQ response preview");
}

/// Red if any production path (preparation, live classification or preview) builds a launch
/// registry again, or if concurrent first use builds the shared one more than once.
#[test]
fn the_launch_registry_is_built_once_per_process() {
    assert_eq!(
        tail_bound_evaluations(),
        0,
        "nothing has built a registry yet"
    );
    let current = fixture();

    // Concurrent first use (D4): every thread's first act is to reach the registry.
    let barrier = Barrier::new(THREADS);
    std::thread::scope(|scope| {
        for thread in 0..THREADS {
            let (barrier, current) = (&barrier, &current);
            scope.spawn(move || {
                barrier.wait();
                let registry = launch_native_effect_registry().expect("launch registry");
                let rate = LAUNCH_SAMPLE_RATES[thread % LAUNCH_SAMPLE_RATES.len()].0;
                prepare_at(rate);
                classify_parameter_change(current);
                classify_automation_change(current);
                preview_at(rate);
                assert!(std::ptr::eq(
                    registry,
                    launch_native_effect_registry().expect("launch registry")
                ));
            });
        }
    });

    // Repeated use on one thread, at every launch rate.
    for _ in 0..REPEATS {
        for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
            prepare_at(rate);
            preview_at(rate);
        }
        classify_parameter_change(&current);
        classify_automation_change(&current);
    }

    let registry = launch_native_effect_registry().expect("launch registry");
    assert!(std::ptr::eq(
        registry,
        launch_native_effect_registry().expect("launch registry")
    ));
    let rows: usize = registry
        .descriptors()
        .map(|descriptor| descriptor.qualities.len())
        .sum();
    assert_eq!(rows, 32, "8 launch effects x 4 launch rates x Normal");
    assert_eq!(tail_bound_evaluations(), rows as u64, "one registry build");
}
