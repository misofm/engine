#![no_main]
use libfuzzer_sys::fuzz_target;
use effect_package::{EffectPackageLimits, verify_effect_package};

fuzz_target!(|data: &[u8]| {
    let _ = verify_effect_package(data, EffectPackageLimits::default());
});
