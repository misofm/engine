//! Findings and the report that `check` returns.

use std::collections::BTreeSet;
use std::fmt;

use crate::whole_plan;

/// What a finding says. The declaration order is the order in which a report lists findings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// `crates/engine/src/realtime` is not a directory.
    MissingRealtimeModule,
    /// A scan root is missing or is not a directory.
    ScanRootUnusable,
    /// A directory under a scan root cannot be listed.
    DirectoryUnlistable,
    /// A symbolic link under a scan root points at a directory, or has a `.rs` name.
    RefusedSymlink,
    /// A file with a `.rs` name is not a regular file (a directory named `x.rs`, a pipe).
    NonRegularRustFile,
    /// A `.rs` file cannot be read.
    UnreadableFile,
    /// A `.rs` file is not UTF-8.
    NotUtf8,
    /// A `.rs` file does not lex.
    LexError,
    /// A marked file, or a file the policy always parses, does not parse.
    ParseError,
    /// A parsed file holds a node that `syn` keeps as raw tokens, so no rule can judge it.
    Verbatim,
    /// Unsafe code outside the allowlisted ownership files.
    UnsafeOutsideOwnership,
    /// A line mentions a region marker but is not a valid marker line.
    MalformedMarker,
    /// A file has a different number of begin and end markers.
    UnmatchedMarkers,
    /// A file's markers do not alternate begin, end, begin, end.
    MisorderedMarkers,
    /// A region marker sits inside a syntax node instead of between two; the text names the node.
    RegionSplits(&'static str),
    /// A region's markers sit in different containers and one of them is a macro token tree.
    RegionSplitsMacroTree,
    /// A marked region holds a forbidden spelling.
    ForbiddenBody,
    /// `crates/capi/src/ffi.rs` does not exist.
    MissingCapiFfi,
    /// `crates/capi/src/ffi.rs` has no production render entry point.
    NoRenderEntry,
    /// Production code of `crates/capi/src/ffi.rs` borrows the whole plan.
    WholePlanReference,
    /// Fewer marked files than the policy's floor; the number is the floor.
    MarkedFileFloor(usize),
    /// Fewer marked regions than the policy's floor; the number is the floor.
    MarkedRegionFloor(usize),
}

impl fmt::Display for Class {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRealtimeModule => out.write_str("missing realtime module"),
            Self::ScanRootUnusable => out.write_str("scan root is missing or is not a directory"),
            Self::DirectoryUnlistable => out.write_str("directory cannot be listed"),
            Self::RefusedSymlink => out.write_str("symbolic link under a scan root"),
            Self::NonRegularRustFile => out.write_str("non-regular file with a .rs name"),
            Self::UnreadableFile => out.write_str("file cannot be read"),
            Self::NotUtf8 => out.write_str("file is not UTF-8"),
            Self::LexError => out.write_str("file does not lex"),
            Self::ParseError => out.write_str("file does not parse"),
            Self::Verbatim => out.write_str("syntax node kept verbatim by the parser"),
            Self::UnsafeOutsideOwnership => {
                out.write_str("unsafe code exists outside the issue-approved ownership/audit files")
            }
            Self::MalformedMarker => out.write_str("malformed realtime policy marker"),
            Self::UnmatchedMarkers => out.write_str("unmatched realtime policy markers"),
            Self::MisorderedMarkers => out.write_str("misordered realtime policy markers"),
            Self::RegionSplits(node) => write!(out, "region boundary splits {node}"),
            Self::RegionSplitsMacroTree => out.write_str("region boundary splits a macro token tree"),
            Self::ForbiddenBody => out.write_str("marked realtime forbidden-body predicate"),
            Self::MissingCapiFfi => write!(out, "missing {}", whole_plan::CAPI_FFI),
            Self::NoRenderEntry => out.write_str("has no production render entry point to scan"),
            Self::WholePlanReference => out.write_str(
                "the C ABI forms a reference to a whole Plan; project the field with &raw const or &raw mut",
            ),
            Self::MarkedFileFloor(floor) => {
                write!(out, "expected at least {floor} marked realtime files")
            }
            Self::MarkedRegionFloor(floor) => {
                write!(out, "expected at least {floor} marked realtime regions")
            }
        }
    }
}

/// One finding: a class, and where it was found when it has a place.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    /// What the finding says.
    pub class: Class,
    /// The file or directory, relative to the workspace root, with `/` separators.
    pub path: Option<String>,
    /// The 1-based line in `path`. Set only with a path.
    pub line: Option<usize>,
    /// The text of that line, trimmed. Set only with a line.
    pub source: Option<String>,
}

impl Finding {
    /// A finding with no place.
    pub(crate) fn bare(class: Class) -> Self {
        Self {
            class,
            path: None,
            line: None,
            source: None,
        }
    }

    /// A finding about a whole path.
    pub(crate) fn at_path(class: Class, path: &str) -> Self {
        Self {
            class,
            path: Some(path.to_owned()),
            line: None,
            source: None,
        }
    }

    /// A finding about a line of a file; `source` is that line's text.
    pub(crate) fn at_line(class: Class, path: &str, line: usize, source: &str) -> Self {
        Self {
            class,
            path: Some(path.to_owned()),
            line: Some(line),
            source: Some(source.trim().to_owned()),
        }
    }

    /// A finding about a line whose text is not at hand (a lex or parse error).
    pub(crate) fn at_line_only(class: Class, path: &str, line: usize) -> Self {
        Self {
            class,
            path: Some(path.to_owned()),
            line: Some(line),
            source: None,
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "realtime policy failure: {}", self.class)?;
        if let Some(path) = &self.path {
            write!(out, ": {path}")?;
            if let Some(line) = self.line {
                write!(out, ":{line}")?;
                if let Some(source) = &self.source {
                    write!(out, ": {source}")?;
                }
            }
        }
        Ok(())
    }
}

/// The result of one `check`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    /// Marked regions in files whose markers pair.
    pub regions: usize,
    /// Files that hold at least one marked region and whose markers pair.
    pub files: usize,
    /// Every finding, sorted by class, then path, then line.
    pub findings: Vec<Finding>,
}

impl Report {
    /// True when there is no finding.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.findings.is_empty()
    }

    /// The distinct classes of the findings.
    #[must_use]
    pub fn classes(&self) -> BTreeSet<Class> {
        self.findings.iter().map(|finding| finding.class).collect()
    }

    /// The success line, `realtime policy: ok (<regions> marked regions in <files> files)`.
    #[must_use]
    pub fn success_line(&self) -> String {
        format!(
            "realtime policy: ok ({} marked regions in {} files)",
            self.regions, self.files
        )
    }
}
