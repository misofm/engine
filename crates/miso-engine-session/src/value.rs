//! Small checked numeric helpers shared by parsing and compilation.

use crate::diagnostic::{Diagnostic, DiagnosticCode, DiagnosticPath, SourceSpan};

pub(crate) fn f32_value(
    value: f64,
    path: DiagnosticPath,
    span: Option<SourceSpan>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<f32> {
    if !value.is_finite() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::NumericNonFinite,
            path,
            span,
            "value must be finite",
        ));
        None
    } else if value.abs() > f64::from(f32::MAX) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::NumericNotF32Representable,
            path,
            span,
            "value must be representable as f32",
        ));
        None
    } else {
        Some(value as f32)
    }
}

pub(crate) fn bounded_f32(
    value: f32,
    minimum: f32,
    maximum: f32,
    path: DiagnosticPath,
    span: Option<SourceSpan>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<f32> {
    if value < minimum || value > maximum {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            span,
            format!("value must be in [{minimum}, {maximum}]"),
        ));
        None
    } else {
        Some(if value == 0.0 { 0.0 } else { value })
    }
}
