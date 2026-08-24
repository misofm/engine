//! The one JSON string escaper.
//!
//! Audit #104 F4 found nine copies. Two of them were wrong: `rack-bench` escaped only `"` and `\`
//! and passed control characters through raw, and `effect-contract-bench` "escaped" by rewriting
//! `"` to `'`, so a backslash or a control character in any environment value produced a record
//! that no validator could parse. This is RFC 8259 section 7 in full.

use std::fmt::Write as _;

/// Escape `value` for use inside a JSON string literal.
///
/// `"` `\` `\b` `\f` `\n` `\r` `\t` get their two-character escapes, every other code point below
/// U+0020 becomes `\u00XX`, and everything else -- non-ASCII included, since the output is UTF-8 --
/// is copied through unchanged.
#[must_use]
pub fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{00}'..='\u{1f}' => {
                write!(&mut escaped, "\\u{:04x}", character as u32)
                    .expect("writing to a String cannot fail");
            }
            _ => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::escape;

    #[test]
    fn escapes_every_rfc_8259_two_character_form() {
        assert_eq!(
            escape("a\"b\\c\u{8}\u{c}\n\r\td"),
            "a\\\"b\\\\c\\b\\f\\n\\r\\td"
        );
    }

    #[test]
    fn escapes_remaining_control_characters_as_four_hex_digits() {
        assert_eq!(escape("\u{1}\u{1f}"), "\\u0001\\u001f");
    }

    #[test]
    fn passes_non_ascii_through_unchanged() {
        assert_eq!(escape("é 音 \u{1f600}"), "é 音 \u{1f600}");
    }

    #[test]
    fn leaves_ordinary_text_alone() {
        assert_eq!(
            escape("x86_64-unknown-linux-gnu"),
            "x86_64-unknown-linux-gnu"
        );
    }
}
