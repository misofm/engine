#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data:&[u8]| { let _=miso_engine_effect_package::verify_canonical_package_v1(data,miso_engine_effect_package::PackageLimits::default()); });
