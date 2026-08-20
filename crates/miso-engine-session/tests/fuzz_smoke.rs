//! Deterministic mutation smoke coverage that runs without a fuzzing runtime.

use miso_engine_session::{
    CompileCaps, canonical_session_toml, compile_session, parse_session_toml,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

#[test]
fn deterministic_parser_compiler_mutation_smoke() {
    let mut state = 0x4d49_534f_454e_4732_u64;
    for iteration in 0..4_096_usize {
        state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        let mut bytes = EXAMPLE.as_bytes().to_vec();
        let index = (z as usize) % bytes.len();
        bytes[index] ^= 1_u8 << (iteration % 7);
        let Ok(source) = core::str::from_utf8(&bytes) else {
            continue;
        };
        let Ok(model) = parse_session_toml(source) else {
            continue;
        };
        if let Ok(canonical) = canonical_session_toml(&model) {
            assert!(parse_session_toml(&canonical).is_ok());
        }
        let caps = CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        };
        let _ = compile_session(&model, caps);
    }
}
