//! Stable structured diagnostics for strict schema parsing and compilation.

use core::fmt;

/// A stable machine-readable reason for a rejected session.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DiagnosticCode {
    /// Input was not TOML 1.0 syntax.
    TomlSyntax,
    /// The schema version key was absent.
    VersionMissing,
    /// The schema version is unsupported.
    VersionUnsupported,
    /// A required key other than the version was absent.
    MissingField,
    /// A table contains a key outside the schema.
    UnknownField,
    /// A value has the wrong TOML type.
    WrongType,
    /// A string is not a stable identifier.
    InvalidId,
    /// An identifier was repeated in its uniqueness scope.
    DuplicateId,
    /// A schema-owned entity reference has no declaration.
    MissingEntityReference,
    /// A string enum token is outside its closed schema set.
    InvalidEnum,
    /// A numeric token is NaN or infinity.
    NumericNonFinite,
    /// An engine session rate is outside the launch-supported tier.
    SampleRateUnsupportedAtLaunch,
    /// A finite numeric token cannot be represented as `f32`.
    NumericNotF32Representable,
    /// A numeric token is outside its schema-local domain.
    NumericOutOfSchemaRange,
    /// A unit token is outside the V1 registry.
    UnitInvalid,
    /// A declared source-channel index is outside its source mapping.
    SourceChannelIndexOutOfRange,
    /// A source region endpoint overflows its sample-time carrier.
    SourceRegionOverflow,
    /// Automation sample times are not monotonic.
    AutomationOutOfOrder,
    /// Automation segments overlap.
    AutomationSegmentOverlap,
    /// An automation segment has an invalid value/range relationship.
    AutomationInvalidRange,
    /// A required bounded capacity was zero.
    CapacityZero,
    /// Checked capacity arithmetic or platform-size conversion failed.
    CapacityArithmeticOverflow,
    /// A configured resource limit was exceeded.
    ResourceLimitExceeded,
}

impl DiagnosticCode {
    /// Stable dotted registry value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TomlSyntax => "toml.syntax",
            Self::VersionMissing => "schema.version_missing",
            Self::VersionUnsupported => "schema.version_unsupported",
            Self::MissingField => "schema.missing_field",
            Self::UnknownField => "schema.unknown_field",
            Self::WrongType => "schema.wrong_type",
            Self::InvalidId => "id.invalid",
            Self::DuplicateId => "id.duplicate",
            Self::MissingEntityReference => "reference.missing_entity",
            Self::InvalidEnum => "schema.invalid_enum",
            Self::NumericNonFinite => "numeric.non_finite",
            Self::SampleRateUnsupportedAtLaunch => "sample_rate.unsupported_at_launch",
            Self::NumericNotF32Representable => "numeric.not_f32_representable",
            Self::NumericOutOfSchemaRange => "numeric.out_of_schema_range",
            Self::UnitInvalid => "unit.invalid",
            Self::SourceChannelIndexOutOfRange => "source.channel_index_out_of_range",
            Self::SourceRegionOverflow => "source.region_overflow",
            Self::AutomationOutOfOrder => "automation.out_of_order",
            Self::AutomationSegmentOverlap => "automation.segment_overlap",
            Self::AutomationInvalidRange => "automation.invalid_range",
            Self::CapacityZero => "capacity.zero",
            Self::CapacityArithmeticOverflow => "capacity.arithmetic_overflow",
            Self::ResourceLimitExceeded => "resource.limit_exceeded",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One structured component of a diagnostic path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PathSegment {
    /// A TOML table key or typed field.
    Field(String),
    /// A zero-based array position.
    Index(usize),
    /// A stable-ID selector used when an index is unavailable.
    Id(String),
}

/// A deterministic structured path into a session document.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticPath(Vec<PathSegment>);

impl DiagnosticPath {
    /// The document root.
    #[must_use]
    pub const fn root() -> Self {
        Self(Vec::new())
    }

    /// Append a table key.
    #[must_use]
    pub fn key(&self, key: &str) -> Self {
        let mut segments = self.0.clone();
        segments.push(PathSegment::Field(key.to_owned()));
        Self(segments)
    }

    /// Append an array index.
    #[must_use]
    pub fn index(&self, index: usize) -> Self {
        let mut segments = self.0.clone();
        segments.push(PathSegment::Index(index));
        Self(segments)
    }

    /// Append a stable-ID selector.
    #[must_use]
    pub fn id(&self, id: &str) -> Self {
        let mut segments = self.0.clone();
        segments.push(PathSegment::Id(id.to_owned()));
        Self(segments)
    }

    /// Borrow structured path components.
    #[must_use]
    pub fn segments(&self) -> &[PathSegment] {
        &self.0
    }

    /// Convert internal dotted/index notation into structured components.
    pub(crate) fn from_dotted(value: &str) -> Self {
        let bytes = value.as_bytes();
        let mut result = Self::root();
        let mut cursor = usize::from(bytes.first() == Some(&b'$'));
        while cursor < bytes.len() {
            match bytes[cursor] {
                b'.' => {
                    cursor += 1;
                    let start = cursor;
                    while cursor < bytes.len() && !matches!(bytes[cursor], b'.' | b'[') {
                        cursor += 1;
                    }
                    if start != cursor {
                        result
                            .0
                            .push(PathSegment::Field(value[start..cursor].to_owned()));
                    }
                }
                b'[' => {
                    cursor += 1;
                    let start = cursor;
                    while cursor < bytes.len() && bytes[cursor] != b']' {
                        cursor += 1;
                    }
                    if let Ok(index) = value[start..cursor].parse::<usize>() {
                        result.0.push(PathSegment::Index(index));
                    }
                    cursor = cursor.saturating_add(1);
                }
                _ => cursor += 1,
            }
        }
        result
    }
}

impl fmt::Display for DiagnosticPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("$")?;
        for segment in &self.0 {
            match segment {
                PathSegment::Field(field) => write!(formatter, ".{field}")?,
                PathSegment::Index(index) => write!(formatter, "[{index}]")?,
                PathSegment::Id(id) => write!(formatter, "[id={id}]")?,
            }
        }
        Ok(())
    }
}

/// Byte and line/column span in the TOML source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceSpan {
    /// Inclusive byte offset.
    pub byte_start: usize,
    /// Exclusive byte offset.
    pub byte_end: usize,
    /// One-based start line.
    pub line: usize,
    /// One-based start column in Unicode scalar values.
    pub column: usize,
}

impl SourceSpan {
    pub(crate) fn whole(source: &str) -> Self {
        Self {
            byte_start: 0,
            byte_end: source.len(),
            line: 1,
            column: 1,
        }
    }
}

/// One deterministic session diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Stable code for machine handling.
    pub code: DiagnosticCode,
    /// Location in the schema tree.
    pub path: DiagnosticPath,
    /// Source extent, if the source parser exposed one.
    pub span: Option<SourceSpan>,
    /// Concise human explanation. It is not a compatibility contract.
    pub message: String,
}

impl Diagnostic {
    pub(crate) fn new(
        code: DiagnosticCode,
        path: DiagnosticPath,
        span: Option<SourceSpan>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            path,
            span,
            message: message.into(),
        }
    }
}

/// A nonempty, deterministically sorted set of errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticSet(Vec<Diagnostic>);

impl DiagnosticSet {
    pub(crate) fn from_vec(mut diagnostics: Vec<Diagnostic>) -> Self {
        assert!(!diagnostics.is_empty(), "DiagnosticSet must be nonempty");
        diagnostics.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then(left.code.cmp(&right.code))
                .then(left.message.cmp(&right.message))
        });
        diagnostics.dedup_by(|left, right| {
            left.code == right.code && left.path == right.path && left.message == right.message
        });
        Self(diagnostics)
    }

    /// Borrow errors in stable path/code order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.0
    }
}

impl fmt::Display for DiagnosticSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, diagnostic) in self.0.iter().enumerate() {
            if index != 0 {
                formatter.write_str("; ")?;
            }
            write!(
                formatter,
                "{} at {}: {}",
                diagnostic.code, diagnostic.path, diagnostic.message
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for DiagnosticSet {}
