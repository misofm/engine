//! Issue #210 phase 2: the delay domain is one number, not two that agree today.
//!
//! `ChannelBuiltins::delay_samples` is bounded in two places by design -- the session schema
//! refuses an out-of-range document at stage 2, and the builtin parameter descriptor publishes the
//! same range to hosts through the parameter-metadata artifact. This crate is the narrowest one
//! that can see both, so this is where they are tied together. Neither crate depends on the other:
//! `miso-engine-builtins` has no session dependency and `miso-engine-session` has no builtins
//! dependency, which is why the constant cannot simply be shared.
//!
//! Red mutation: change either literal alone -> this fails.

use miso_engine_builtins::{BUILTIN_PARAMETER_DESCRIPTORS_V1, BuiltinParameterDomain};
use miso_engine_session::CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM;

#[test]
fn the_descriptor_and_the_schema_publish_the_same_delay_domain() {
    let delay = BUILTIN_PARAMETER_DESCRIPTORS_V1
        .iter()
        .find(|descriptor| descriptor.name == "delay_samples")
        .expect("the builtin table declares an input delay row");
    assert_eq!(delay.id, 11);
    assert_eq!(
        delay.domain,
        BuiltinParameterDomain::FiniteInclusive {
            minimum: 0.0,
            maximum: CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM as f32,
        },
        "the descriptor's published range must be the schema's validated range"
    );
    // The bound is exactly representable in `f32`, so the two forms compare without rounding.
    assert_eq!(
        CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM as f32 as u32,
        CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM
    );
}

/// A track that declares a delay is still a validly prepared session: the delay is a graph-owned
/// object, so nothing in the builtins preparation may notice it.
///
/// Red mutation: charge the ring to the builtin resource estimate as well as the graph's -> the
/// retained-byte equality fails, which is the double-count this row exists to forbid.
#[test]
fn a_declared_delay_changes_no_prepared_builtin_byte() {
    use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
    use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

    let caps = || CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    let builtin_caps = || BuiltinCompileCaps {
        maximum_total_state_bytes: 1 << 24,
        maximum_total_retained_payload_bytes: 1 << 24,
        maximum_total_meter_items: 1 << 16,
        maximum_total_meter_bytes: 1 << 24,
        maximum_single_allocation_bytes: 1 << 24,
        maximum_meter_streams: 64,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    };
    let source = include_str!("../../../fixtures/session/v1/canonical.toml");
    let prepared_bytes = |delay: u32| {
        let mut model = parse_session_toml(source).expect("fixture parses");
        model.tracks[0].builtins.left.delay_samples = delay;
        model.tracks[0].builtins.right.delay_samples = delay;
        let session = compile_session(&model, caps()).expect("session compiles");
        prepare_session_builtins(&session, &[], builtin_caps())
            .expect("builtins prepare")
            .resource_report()
    };
    assert_eq!(prepared_bytes(0), prepared_bytes(48_000));
}
