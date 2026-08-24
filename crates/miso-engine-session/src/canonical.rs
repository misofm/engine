//! Canonical TOML writer over the shared emit-side model walk.

use core::{convert::Infallible, fmt::Write as _};

use crate::{
    FieldKey, ModelVisitor, SessionTomlV1, Token, VisitModel, WalkOrder,
    validate::validate_session, value::write_f32,
};

/// Produce canonical V1 TOML bytes as UTF-8 text with LF line endings and one final newline.
pub fn canonical_session_toml(session: &SessionTomlV1) -> Result<String, crate::DiagnosticSet> {
    validate_session(session)?;
    Ok(write_canonical(session))
}

pub(crate) fn write_canonical(session: &SessionTomlV1) -> String {
    let mut writer = TomlWriter {
        output: String::new(),
        depth: 0,
        first_field: true,
    };
    match session.visit(WalkOrder::Canonical, &mut writer) {
        Ok(()) => writer.output,
        Err(error) => match error {},
    }
}

struct TomlWriter {
    output: String,
    depth: usize,
    first_field: bool,
}
impl TomlWriter {
    fn field(&mut self, key: FieldKey) {
        if self.depth != 0 && !self.first_field {
            self.output.push_str(", ");
        }
        self.output.push_str(key.name);
        self.output.push_str(" = ");
        self.first_field = false;
    }
    fn scalar_end(&mut self) {
        if self.depth == 0 {
            self.output.push('\n');
        }
    }
}

impl ModelVisitor for TomlWriter {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, _: u32) -> Result<(), Self::Error> {
        if let Some(key) = key {
            self.field(key);
            self.output.push_str("{ ");
        } else if self.depth != 0 {
            if self.depth == 1 {
                self.output.push_str("  ");
            } else if !self.first_field {
                self.output.push_str(", ");
            }
            self.output.push_str("{ ");
        } else {
            return Ok(());
        }
        self.depth += 1;
        self.first_field = true;
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Self::Error> {
        if self.depth == 0 {
            return Ok(());
        }
        self.depth -= 1;
        self.output.push_str(" }");
        if self.depth == 0 {
            self.output.push('\n');
        } else if self.depth == 1 {
            self.output.push_str(",\n");
        }
        self.first_field = false;
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, _: usize) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push('[');
        if self.depth == 0 {
            self.output.push('\n');
        }
        self.depth += 1;
        self.first_field = true;
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Self::Error> {
        self.depth -= 1;
        self.output.push(']');
        if self.depth == 0 {
            self.output.push('\n');
        }
        self.first_field = false;
        Ok(())
    }
    fn wire_tag(&mut self, _: Token) -> Result<(), Self::Error> {
        Ok(())
    }
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push_str(if value { "true" } else { "false" });
        self.scalar_end();
        Ok(())
    }
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error> {
        self.u64(key, u64::from(value))
    }
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error> {
        self.u64(key, u64::from(value))
    }
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        self.scalar_end();
        Ok(())
    }
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write_f32(&mut self.output, value);
        self.scalar_end();
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &crate::StableId) -> Result<(), Self::Error> {
        self.text(key, value.as_str())
    }
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Self::Error> {
        self.field(key);
        write_quoted(&mut self.output, value);
        self.scalar_end();
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
                let _ = if u32::from(character) <= 0xffff {
                    write!(output, "\\u{:04X}", u32::from(character))
                } else {
                    write!(output, "\\U{:08X}", u32::from(character))
                };
            }
            character => output.push(character),
        }
    }
    output.push('"');
}
