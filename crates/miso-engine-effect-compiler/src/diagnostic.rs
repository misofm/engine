use core::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EffectDiagnostic {
    pub code: &'static str,
    pub path: String,
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
        write!(formatter, "{}: {}", self.path, self.code)
    }
}
