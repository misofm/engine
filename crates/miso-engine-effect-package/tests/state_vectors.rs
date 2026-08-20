//! State envelope regression coverage.
use miso_engine_effect_contract::{EffectId, LatencySamples, TailSamples};
use miso_engine_effect_package::*;
#[test]
fn state_envelope_rejects_tail_and_payload_changes() {
    let mut payload = Vec::new();
    encode_lane_payload_v1(b"c", b"l", b"r", &mut payload).unwrap();
    let state = EffectStateV1 {
        contract_minor: 0,
        state_schema_version: 1,
        sample_rate: 48_000,
        quantum: 128,
        quality: 2,
        link_mode: 1,
        bypass: false,
        latency: LatencySamples(3),
        tail: TailSamples::Finite(4),
        effect_id: EffectId::parse("test.effect").unwrap(),
        payload,
    };
    let mut bytes = Vec::new();
    encode_effect_state_v1(&state, &mut bytes).unwrap();
    assert!(verify_effect_state_v1(&bytes).is_ok());
    bytes[143] ^= 1;
    assert!(verify_effect_state_v1(&bytes).is_err());
}
