//! The forbidden-body predicate: spellings that a marked realtime region must not hold.
//!
//! A region's tokens are rendered as text, and the text is searched for the patterns. The
//! rendering puts one space only between two adjacent words (identifiers, keywords and
//! literals), so layout and comments cannot hide or fake a spelling, and it renders every literal
//! empty, so text inside a literal cannot match. Macro token trees are tokens like any other.

use std::collections::BTreeSet;

use crate::leaf::{self, Kind, Leaf};
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;

/// The forbidden spellings, matched as substrings of the rendered text. `.collect(` and
/// `.collect::<` are the two spellings of a collect call.
const PATTERNS: [&str; 37] = [
    "Vec::",
    "vec!",
    "Box::",
    "String::",
    ".to_vec(",
    ".collect(",
    ".collect::<",
    "Arc::clone",
    "Rc::clone",
    "drop(",
    "Mutex",
    "RwLock",
    "Condvar",
    "mpsc",
    "sync_channel",
    "thread::",
    "sleep(",
    "yield_now",
    "spin_loop",
    "std::fs",
    "std::net",
    "std::process",
    "println!",
    "eprintln!",
    "format!",
    "log::",
    "tracing::",
    ".await",
    "File::",
    "Tcp",
    "Udp",
    ".expect(",
    ".unwrap(",
    "panic!(",
    "unreachable!(",
    "todo!(",
    "unimplemented!(",
];

/// The one forbidden keyword. A spelling that starts with it (`async {`, `async move`) renders
/// without the space that a text pattern would need, so the keyword token is the rule.
const ASYNC: &str = "async";

/// Check the regions of `source`: report each line that holds a forbidden spelling.
pub(crate) fn scan(source: &Source, regions: &[Region], findings: &mut Vec<Finding>) {
    let leaves = leaf::flatten(&source.tokens);
    for region in regions {
        // The leaves are in source order, so those that start strictly between the markers are
        // one run.
        let from = leaves.partition_point(|leaf| leaf.start.line <= region.begin);
        let to = leaves.partition_point(|leaf| leaf.start.line < region.end);
        let inside = &leaves[from..to.max(from)];
        let mut lines = BTreeSet::new();
        lines.extend(
            inside
                .iter()
                .filter(|leaf| matches!(&leaf.kind, Kind::Word(word) if word == ASYNC))
                .map(|leaf| leaf.start.line),
        );
        let rendered = Rendered::of(inside);
        for pattern in PATTERNS {
            lines.extend(
                rendered
                    .text
                    .match_indices(pattern)
                    .map(|(offset, _)| rendered.line_at(offset)),
            );
        }
        findings.extend(lines.into_iter().map(|line| {
            Finding::at_line(
                Class::ForbiddenBody,
                &source.path,
                line,
                source.line_text(line),
            )
        }));
    }
}

/// A run of leaves as text, with the line that each byte of the text came from.
struct Rendered {
    text: String,
    /// The byte offset in `text` of each leaf's first byte, and that leaf's start line.
    starts: Vec<(usize, usize)>,
}

impl Rendered {
    fn of(leaves: &[Leaf]) -> Self {
        let mut text = String::new();
        let mut starts = Vec::with_capacity(leaves.len());
        let mut previous_was_word = false;
        for leaf in leaves {
            let is_word = matches!(leaf.kind, Kind::Word(_) | Kind::Literal(_));
            if previous_was_word && is_word {
                text.push(' ');
            }
            previous_was_word = is_word;
            starts.push((text.len(), leaf.start.line));
            match &leaf.kind {
                Kind::Word(word) => text.push_str(word),
                Kind::Literal(literal) => text.push_str(empty_literal(literal)),
                Kind::Punct(character) | Kind::Open(character) | Kind::Close(character) => {
                    text.push(*character);
                }
            }
        }
        Self { text, starts }
    }

    /// The start line of the leaf that holds byte `offset` of the text.
    fn line_at(&self, offset: usize) -> usize {
        let index = self.starts.partition_point(|(start, _)| *start <= offset);
        self.starts[index - 1].1
    }
}

/// The empty literal of the kind of `literal`.
fn empty_literal(literal: &proc_macro2::Literal) -> &'static str {
    match syn::Lit::new(literal.clone()) {
        syn::Lit::Str(_) => "\"\"",
        syn::Lit::ByteStr(_) => "b\"\"",
        syn::Lit::CStr(_) => "c\"\"",
        syn::Lit::Byte(_) => "b''",
        syn::Lit::Char(_) => "''",
        syn::Lit::Float(_) => "0.0",
        _ => "0",
    }
}
