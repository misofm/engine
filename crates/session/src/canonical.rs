//! Canonical JSON writer over the shared emit-side model walk.

use core::{convert::Infallible, fmt::Write as _};

use crate::{
    FieldKey, ModelVisitor, SessionModel, Token, VisitModel, WalkOrder, validate::validate_session,
    value::write_f32,
};

/// Produce canonical Session V1 JSON as UTF-8 text with one final newline.
pub fn canonical_session_json(session: &SessionModel) -> Result<String, crate::DiagnosticSet> {
    validate_session(session)?;
    Ok(write_canonical(session))
}

pub(crate) fn write_canonical(session: &SessionModel) -> String {
    let mut writer = JsonWriter {
        output: String::new(),
        containers: Vec::new(),
    };
    match session.visit(WalkOrder::Canonical, &mut writer) {
        Ok(()) => {
            writer.output.push('\n');
            writer.output
        }
        Err(error) => match error {},
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Object,
    Array,
}
#[derive(Clone, Copy)]
struct Container {
    kind: Kind,
    first: bool,
    empty: bool,
}
struct JsonWriter {
    output: String,
    containers: Vec<Container>,
}

impl JsonWriter {
    fn indent(&mut self, depth: usize) {
        self.output.extend(core::iter::repeat_n(' ', depth * 2));
    }
    fn entry_prefix(&mut self) {
        let depth = self.containers.len();
        let container = self.containers.last_mut().expect("value has a parent");
        if !container.first {
            self.output.push(',');
        }
        self.output.push('\n');
        container.first = false;
        self.indent(depth);
    }
    fn field(&mut self, key: FieldKey) {
        debug_assert_eq!(self.containers.last().map(|c| c.kind), Some(Kind::Object));
        self.entry_prefix();
        write_quoted(&mut self.output, key.name);
        self.output.push_str(": ");
    }
    fn array_item(&mut self) {
        debug_assert_eq!(self.containers.last().map(|c| c.kind), Some(Kind::Array));
        self.entry_prefix();
    }
    fn close(&mut self, expected: Kind, delimiter: char) {
        let container = self.containers.pop().expect("balanced model walk");
        debug_assert_eq!(container.kind, expected);
        if !container.empty {
            self.output.push('\n');
            self.indent(self.containers.len());
        }
        self.output.push(delimiter);
    }
}

impl ModelVisitor for JsonWriter {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, _fields: u32) -> Result<(), Self::Error> {
        match key {
            Some(key) => self.field(key),
            None if !self.containers.is_empty() => self.array_item(),
            None => {}
        }
        self.output.push('{');
        self.containers.push(Container {
            kind: Kind::Object,
            first: true,
            empty: false,
        });
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Self::Error> {
        self.close(Kind::Object, '}');
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, length: usize) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push('[');
        self.containers.push(Container {
            kind: Kind::Array,
            first: true,
            empty: length == 0,
        });
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Self::Error> {
        self.close(Kind::Array, ']');
        Ok(())
    }
    fn wire_tag(&mut self, _: Token) -> Result<(), Self::Error> {
        Ok(())
    }
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push_str(if value { "true" } else { "false" });
        Ok(())
    }
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        Ok(())
    }
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        Ok(())
    }
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error> {
        self.field(key);
        write_quoted(&mut self.output, &value.to_string());
        Ok(())
    }
    fn source_bit_depth(
        &mut self,
        key: FieldKey,
        value: crate::SourceBitDepth,
    ) -> Result<(), Self::Error> {
        self.field(key);
        match value {
            crate::SourceBitDepth::Pcm16 => self.output.push_str("16"),
            crate::SourceBitDepth::Pcm24 => self.output.push_str("24"),
            crate::SourceBitDepth::Float32 => write_quoted(&mut self.output, "32f"),
        }
        Ok(())
    }
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write_f32(&mut self.output, value);
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &crate::StableId) -> Result<(), Self::Error> {
        self.text(key, value.as_str())
    }
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Self::Error> {
        self.field(key);
        write_quoted(&mut self.output, value);
        Ok(())
    }
    fn token(&mut self, key: FieldKey, value: Token) -> Result<(), Self::Error> {
        self.text(key, value.text)
    }
}

fn write_quoted(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\u{0C}' => output.push_str("\\f"),
            '\r' => output.push_str("\\r"),
            character if character.is_control() => {
                let _ = write!(output, "\\u{:04X}", u32::from(character));
            }
            character => output.push(character),
        }
    }
    output.push('"');
}

#[cfg(test)]
mod tests {
    use super::write_quoted;
    #[test]
    fn canonical_string_escaping_is_frozen() {
        let mut output = String::new();
        write_quoted(
            &mut output,
            "\"\\\u{8}\t\n\u{c}\r\u{0}\u{7f}\u{80}/\u{2028}\u{2029}\u{1f642}",
        );
        assert_eq!(
            output,
            "\"\\\"\\\\\\b\\t\\n\\f\\r\\u0000\\u007F\\u0080/\u{2028}\u{2029}\u{1f642}\""
        );
    }
}
