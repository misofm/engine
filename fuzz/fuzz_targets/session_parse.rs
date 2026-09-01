#![no_main]

use libfuzzer_sys::fuzz_target;
use session::{canonical_session_toml, parse_session_toml};

fuzz_target!(|bytes: &[u8]| {
    let Ok(source) = core::str::from_utf8(bytes) else {
        return;
    };
    let Ok(model) = parse_session_toml(source) else {
        return;
    };
    let Ok(canonical) = canonical_session_toml(&model) else {
        return;
    };
    let reparsed = parse_session_toml(&canonical).expect("canonical models reparse");
    assert_eq!(
        canonical_session_toml(&reparsed).expect("reparsed models canonicalize"),
        canonical
    );
});
