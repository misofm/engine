#![no_main]

use libfuzzer_sys::fuzz_target;
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

fuzz_target!(|bytes: &[u8]| {
    let Ok(source) = core::str::from_utf8(bytes) else {
        return;
    };
    let Ok(model) = parse_session_toml(source) else {
        return;
    };
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    if let Ok(compiled) = compile_session(&model, caps) {
        assert_eq!(compiled.normalized_model().schema_version, 1);
        assert!(parse_session_toml(compiled.canonical_toml()).is_ok());
    }
});
