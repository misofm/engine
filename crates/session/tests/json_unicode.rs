//! Canonical JSON Unicode and escape behavior.

use session::{DiagnosticCode, canonical_session_json, parse_session_json};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");

#[test]
fn escaped_and_direct_unicode_decode_equally_and_emit_directly() {
    let mut model = parse_session_json(CANONICAL).expect("fixture");
    model.tracks[0].dynamic.effects[0].identity = session::EffectIdentity::ThirdPartyCid {
        cid: "bafy.\u{e9}\u{2028}\u{1f642}".to_owned(),
    };
    let direct = canonical_session_json(&model).expect("direct Unicode");
    let escaped = direct.replace("bafy.\u{e9}", r"bafy.\u00E9");
    let direct_model = parse_session_json(&direct).expect("direct Unicode");
    let escaped_model = parse_session_json(&escaped).expect("escaped Unicode");
    assert_eq!(direct_model, escaped_model);
    assert!(
        canonical_session_json(&direct_model)
            .expect("canonical")
            .contains("bafy.\u{e9}\u{2028}\u{1f642}")
    );
}

#[test]
fn invalid_escape_and_unpaired_surrogate_refuse() {
    for source in [r#"{"x":"\x41"}"#, r#"{"x":"\uD800"}"#] {
        let error = parse_session_json(source).expect_err("invalid JSON string");
        assert_eq!(error.diagnostics()[0].code, DiagnosticCode::JsonSyntax);
    }
}
