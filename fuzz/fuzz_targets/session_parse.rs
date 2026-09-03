#![no_main]

use libfuzzer_sys::fuzz_target;
use session::{canonical_session_json, parse_session_json};

fuzz_target!(|bytes: &[u8]| {
    let Ok(source) = core::str::from_utf8(bytes) else {
        return;
    };
    let Ok(model) = parse_session_json(source) else {
        return;
    };
    let Ok(canonical) = canonical_session_json(&model) else {
        return;
    };
    let reparsed = parse_session_json(&canonical).expect("canonical models reparse");
    assert_eq!(
        canonical_session_json(&reparsed).expect("reparsed models canonicalize"),
        canonical
    );
});
