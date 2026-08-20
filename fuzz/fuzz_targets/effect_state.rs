#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data:&[u8]| { let _=miso_engine_effect_package::verify_effect_state_v1(data); });
