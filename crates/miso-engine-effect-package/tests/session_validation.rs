//! Transactional session preparation regression coverage.
use miso_engine_effect_contract::NativeEffectRegistry;
use miso_engine_effect_package::*;
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};
#[test]
fn empty_session_effects_prepare_transactionally() {
    let model =
        parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml")).unwrap();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .unwrap();
    assert!(
        prepare_session_effects(
            session,
            &NativeEffectRegistry::default(),
            None,
            EffectCompileCaps {
                maximum_state_bytes: 0,
                maximum_scratch_bytes: 0,
                maximum_automation_spans: 0
            }
        )
        .is_err()
    );
}
