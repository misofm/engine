#![no_main]
#[path = "protocol_support.rs"] mod protocol_support;
use libfuzzer_sys::fuzz_target;
fuzz_target!(|bytes: &[u8]| { let mut decoded = [0_u8; protocol_support::MAX_FRAME_BYTES]; let input = protocol_support::corpus_or_raw(bytes, &mut decoded); protocol_support::assert_stable(protocol_support::command_class, input); });
