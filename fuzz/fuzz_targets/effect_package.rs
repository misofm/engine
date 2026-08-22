#![no_main]
use libfuzzer_sys::fuzz_target;
use miso_engine_effect_package::{EffectPackageLimitsV1, verify_effect_package_v1};

fuzz_target!(|data: &[u8]| {
    let _ = verify_effect_package_v1(data, EffectPackageLimitsV1::default());
});
