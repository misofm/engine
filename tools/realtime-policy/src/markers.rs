//! Region markers: which lines are valid marker lines, and whether a file's markers pair.
//!
//! A valid marker line is a whole-line `//` comment with the exact text of [`marker_kind`]. A
//! line that mentions a marker word in any other way is malformed, so a marker the tool cannot
//! read as a marker is a finding and never a silently dropped region.

use proc_macro2::{LineColumn, TokenStream};

use crate::leaf;
use crate::report::{Class, Finding};
use crate::source::Source;

/// The two marker words. They are built here so that this file holds neither word in one piece.
const BEGIN_WORD: &str = concat!("REALTIME_POLICY", "_BEGIN");
const END_WORD: &str = concat!("REALTIME_POLICY", "_END");

/// Which end of a region a marker line is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Begin,
    End,
}

/// A region: the 1-based lines of its begin and end markers. The region's text is the lines
/// strictly between them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Region {
    pub(crate) begin: usize,
    pub(crate) end: usize,
}

/// What the marker pass learned about one file.
#[derive(Debug, Default)]
pub(crate) struct Markers {
    /// True when any line mentions a marker word, valid or not.
    pub(crate) mentions_marker: bool,
    /// The file's regions. Empty when its markers do not pair.
    pub(crate) regions: Vec<Region>,
}

/// Whether `line` is exactly `^\s*// <MARKER>(: .*)?$` for one of the two marker words.
fn marker_kind(line: &str) -> Option<Kind> {
    let rest = line.trim_start().strip_prefix("// ")?;
    let (kind, rest) = if let Some(rest) = rest.strip_prefix(BEGIN_WORD) {
        (Kind::Begin, rest)
    } else {
        (Kind::End, rest.strip_prefix(END_WORD)?)
    };
    (rest.is_empty() || rest.starts_with(": ")).then_some(kind)
}

/// A leaf token's extent. `end` is one past its last character.
#[derive(Clone, Copy)]
struct Extent {
    start: LineColumn,
    end: LineColumn,
}

/// The positions of the lexer's leaf tokens: identifiers, punctuation, literals and group
/// delimiters, sorted by start. Everything between two of them is whitespace and comments.
struct Leaves {
    sorted: Vec<Extent>,
    /// `reach[i]` is the greatest end among `sorted[..=i]`.
    reach: Vec<LineColumn>,
}

impl Leaves {
    fn collect(tokens: &TokenStream) -> Self {
        let mut sorted: Vec<Extent> = leaf::flatten(tokens)
            .iter()
            .map(|leaf| Extent {
                start: leaf.start,
                end: leaf.end,
            })
            .collect();
        sorted.sort_by_key(|extent| (extent.start.line, extent.start.column));
        let mut reach: Vec<LineColumn> = Vec::with_capacity(sorted.len());
        for extent in &sorted {
            let greatest = match reach.last() {
                Some(previous)
                    if (previous.line, previous.column) > (extent.end.line, extent.end.column) =>
                {
                    *previous
                }
                _ => extent.end,
            };
            reach.push(greatest);
        }
        Self { sorted, reach }
    }
}

/// Whether the start of `line` lies inside a block comment, given that no leaf token spans it.
fn starts_inside_block_comment(source: &Source, leaves: &Leaves, line: usize) -> bool {
    // The leaf tokens before the line; none of them reaches the line.
    let before = leaves.sorted.partition_point(|leaf| leaf.start.line < line);
    let gap_start = before.checked_sub(1).map_or(0, |last| {
        let reach = leaves.reach[last];
        source.offset(reach.line, reach.column)
    });
    let gap_end = source.offset(line, 0);
    let mut depth = 0_usize;
    let mut in_line_comment = false;
    let mut bytes = source.text.as_bytes()[gap_start..gap_end]
        .iter()
        .copied()
        .peekable();
    while let Some(byte) = bytes.next() {
        if in_line_comment {
            in_line_comment = byte != b'\n';
        } else if byte == b'/' && bytes.peek() == Some(&b'*') {
            bytes.next();
            depth += 1;
        } else if depth > 0 && byte == b'*' && bytes.peek() == Some(&b'/') {
            bytes.next();
            depth -= 1;
        } else if depth == 0 && byte == b'/' && bytes.peek() == Some(&b'/') {
            bytes.next();
            in_line_comment = true;
        }
    }
    depth > 0
}

/// Read the markers of one file: report malformed ones and pairing errors.
pub(crate) fn scan(source: &Source, findings: &mut Vec<Finding>) -> Markers {
    let mentions: Vec<usize> = (1..=source.line_count())
        .filter(|&line| {
            let text = source.line_text(line);
            text.contains(BEGIN_WORD) || text.contains(END_WORD)
        })
        .collect();
    if mentions.is_empty() {
        return Markers::default();
    }
    let leaves = Leaves::collect(&source.tokens);
    let mut valid: Vec<(usize, Kind)> = Vec::new();
    for &line in &mentions {
        let text = source.line_text(line);
        let reach_covers_line = {
            let after = leaves
                .sorted
                .partition_point(|leaf| leaf.start.line <= line);
            after
                .checked_sub(1)
                .is_some_and(|last| leaves.reach[last].line >= line)
        };
        let kind = marker_kind(text)
            .filter(|_| !reach_covers_line && !starts_inside_block_comment(source, &leaves, line));
        match kind {
            Some(kind) => valid.push((line, kind)),
            None => findings.push(Finding::at_line(
                Class::MalformedMarker,
                &source.path,
                line,
                text,
            )),
        }
    }
    let begins = valid
        .iter()
        .filter(|(_, kind)| *kind == Kind::Begin)
        .count();
    let ends = valid.len() - begins;
    let mut regions = Vec::new();
    if begins != ends {
        findings.push(Finding::at_path(Class::UnmatchedMarkers, &source.path));
    } else if let Some(line) = misordered_at(&valid) {
        findings.push(Finding::at_line(
            Class::MisorderedMarkers,
            &source.path,
            line,
            source.line_text(line),
        ));
    } else {
        regions.extend(valid.as_chunks::<2>().0.iter().map(|pair| Region {
            begin: pair[0].0,
            end: pair[1].0,
        }));
    }
    Markers {
        mentions_marker: true,
        regions,
    }
}

/// The line of the first marker that breaks the begin, end, begin, end alternation.
fn misordered_at(valid: &[(usize, Kind)]) -> Option<usize> {
    valid.iter().enumerate().find_map(|(index, (line, kind))| {
        let expected = if index % 2 == 0 {
            Kind::Begin
        } else {
            Kind::End
        };
        (*kind != expected).then_some(*line)
    })
}
