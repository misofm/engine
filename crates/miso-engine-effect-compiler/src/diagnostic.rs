use core::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EffectDiagnostic {
    pub code: &'static str,
    pub path: String,
    /// Canonical neighbors for `effect.parameter.off_lattice`.
    pub nearest: Option<EffectNearestValues>,
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EffectNearestValues {
    pub lower: String,
    pub upper: String,
}
impl EffectDiagnostic {
    pub fn new(code: &'static str, path: String) -> Self {
        Self {
            code,
            path,
            nearest: None,
        }
    }

    pub fn off_lattice(path: String, lower: String, upper: String) -> Self {
        Self {
            code: "effect.parameter.off_lattice",
            path,
            nearest: Some(EffectNearestValues { lower, upper }),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectDiagnosticSet(pub Vec<EffectDiagnostic>);
impl EffectDiagnosticSet {
    pub fn sorted(mut diagnostics: Vec<EffectDiagnostic>) -> Self {
        diagnostics.sort();
        Self(diagnostics)
    }
}
impl fmt::Display for EffectDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.code)?;
        if let Some(nearest) = &self.nearest {
            write!(
                formatter,
                " (nearest: {}, {})",
                nearest.lower, nearest.upper
            )?;
        }
        Ok(())
    }
}
